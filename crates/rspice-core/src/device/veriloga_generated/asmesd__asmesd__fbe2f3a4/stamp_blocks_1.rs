#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        var_argbv: f64,
        var_argbv_db0: f64,
        var_argbv_db1: f64,
        var_argbv_db2: f64,
        var_argbv_db3: f64,
        var_argbv_db4: f64,
        var_argbv_db5: f64,
        var_argbv_db6: f64,
        var_argbv_db7: f64,
        var_argbv_dn0: f64,
        var_argbv_dn1: f64,
        var_argbv_dn2: f64,
        var_argbv_dn3: f64,
        var_argbv_dn4: f64,
        var_argbv_dn5: f64,
        var_argbv_dn6: f64,
        var_argbv_dn7: f64,
        var_argbv_dn8: f64,
        var_argbv_dn9: f64,
        var_argbvvt: f64,
        var_argbvvt_db0: f64,
        var_argbvvt_db1: f64,
        var_argbvvt_db2: f64,
        var_argbvvt_db3: f64,
        var_argbvvt_db4: f64,
        var_argbvvt_db5: f64,
        var_argbvvt_db6: f64,
        var_argbvvt_db7: f64,
        var_argbvvt_dn0: f64,
        var_argbvvt_dn1: f64,
        var_argbvvt_dn2: f64,
        var_argbvvt_dn3: f64,
        var_argbvvt_dn4: f64,
        var_argbvvt_dn5: f64,
        var_argbvvt_dn6: f64,
        var_argbvvt_dn7: f64,
        var_argbvvt_dn8: f64,
        var_argbvvt_dn9: f64,
        var_guard3: f64,
        var_guard4: f64,
        var_ijbv_t: f64,
        var_ijbv_t_db0: f64,
        var_ijbv_t_db1: f64,
        var_ijbv_t_db2: f64,
        var_ijbv_t_db3: f64,
        var_ijbv_t_db4: f64,
        var_ijbv_t_db5: f64,
        var_ijbv_t_db6: f64,
        var_ijbv_t_db7: f64,
        var_ijbv_t_dn0: f64,
        var_ijbv_t_dn1: f64,
        var_ijbv_t_dn2: f64,
        var_ijbv_t_dn3: f64,
        var_ijbv_t_dn4: f64,
        var_ijbv_t_dn5: f64,
        var_ijbv_t_dn6: f64,
        var_ijbv_t_dn7: f64,
        var_ijbv_t_dn8: f64,
        var_ijbv_t_dn9: f64,
        var_is_t: f64,
        var_is_t_db0: f64,
        var_is_t_db1: f64,
        var_is_t_db2: f64,
        var_is_t_db3: f64,
        var_is_t_db4: f64,
        var_is_t_db5: f64,
        var_is_t_db6: f64,
        var_is_t_db7: f64,
        var_is_t_dn0: f64,
        var_is_t_dn1: f64,
        var_is_t_dn2: f64,
        var_is_t_dn3: f64,
        var_is_t_dn4: f64,
        var_is_t_dn5: f64,
        var_is_t_dn6: f64,
        var_is_t_dn7: f64,
        var_is_t_dn8: f64,
        var_is_t_dn9: f64,
        var_isr_t: f64,
        var_theexp_t: f64,
        var_theexp_t_db0: f64,
        var_theexp_t_db1: f64,
        var_theexp_t_db2: f64,
        var_theexp_t_db3: f64,
        var_theexp_t_db4: f64,
        var_theexp_t_db5: f64,
        var_theexp_t_db6: f64,
        var_theexp_t_db7: f64,
        var_theexp_t_dn0: f64,
        var_theexp_t_dn1: f64,
        var_theexp_t_dn2: f64,
        var_theexp_t_dn3: f64,
        var_theexp_t_dn4: f64,
        var_theexp_t_dn5: f64,
        var_theexp_t_dn6: f64,
        var_theexp_t_dn7: f64,
        var_theexp_t_dn8: f64,
        var_theexp_t_dn9: f64,
        var_vbiei: f64,
        var_vbiei_db0: f64,
        var_vbiei_db1: f64,
        var_vbiei_db2: f64,
        var_vbiei_db3: f64,
        var_vbiei_db4: f64,
        var_vbiei_db5: f64,
        var_vbiei_db6: f64,
        var_vbiei_db7: f64,
        var_vbiei_dn0: f64,
        var_vbiei_dn1: f64,
        var_vbiei_dn2: f64,
        var_vbiei_dn3: f64,
        var_vbiei_dn4: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vbiei_dn7: f64,
        var_vbiei_dn8: f64,
        var_vbiei_dn9: f64,
        var_arg_slot: &mut f64,
        var_arg_db0_slot: &mut f64,
        var_arg_db1_slot: &mut f64,
        var_arg_db2_slot: &mut f64,
        var_arg_db3_slot: &mut f64,
        var_arg_db4_slot: &mut f64,
        var_arg_db5_slot: &mut f64,
        var_arg_db6_slot: &mut f64,
        var_arg_db7_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn1_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_arg_rdb0_slot: &mut f64,
        var_arg_rdb1_slot: &mut f64,
        var_arg_rdb2_slot: &mut f64,
        var_arg_rdb3_slot: &mut f64,
        var_arg_rdb4_slot: &mut f64,
        var_arg_rdb5_slot: &mut f64,
        var_arg_rdb6_slot: &mut f64,
        var_arg_rdb7_slot: &mut f64,
        var_arg_rdn0_slot: &mut f64,
        var_arg_rdn1_slot: &mut f64,
        var_arg_rdn2_slot: &mut f64,
        var_arg_rdn3_slot: &mut f64,
        var_arg_rdn4_slot: &mut f64,
        var_arg_rdn5_slot: &mut f64,
        var_arg_rdn6_slot: &mut f64,
        var_arg_rdn7_slot: &mut f64,
        var_arg_rdn8_slot: &mut f64,
        var_arg_rdn9_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard5_db0_slot: &mut f64,
        var_guard5_db1_slot: &mut f64,
        var_guard5_db2_slot: &mut f64,
        var_guard5_db3_slot: &mut f64,
        var_guard5_db4_slot: &mut f64,
        var_guard5_db5_slot: &mut f64,
        var_guard5_db6_slot: &mut f64,
        var_guard5_db7_slot: &mut f64,
        var_guard5_dn0_slot: &mut f64,
        var_guard5_dn1_slot: &mut f64,
        var_guard5_dn2_slot: &mut f64,
        var_guard5_dn3_slot: &mut f64,
        var_guard5_dn4_slot: &mut f64,
        var_guard5_dn5_slot: &mut f64,
        var_guard5_dn6_slot: &mut f64,
        var_guard5_dn7_slot: &mut f64,
        var_guard5_dn8_slot: &mut f64,
        var_guard5_dn9_slot: &mut f64,
        var_guard5_rdb0_slot: &mut f64,
        var_guard5_rdb1_slot: &mut f64,
        var_guard5_rdb2_slot: &mut f64,
        var_guard5_rdb3_slot: &mut f64,
        var_guard5_rdb4_slot: &mut f64,
        var_guard5_rdb5_slot: &mut f64,
        var_guard5_rdb6_slot: &mut f64,
        var_guard5_rdb7_slot: &mut f64,
        var_guard5_rdn0_slot: &mut f64,
        var_guard5_rdn1_slot: &mut f64,
        var_guard5_rdn2_slot: &mut f64,
        var_guard5_rdn3_slot: &mut f64,
        var_guard5_rdn4_slot: &mut f64,
        var_guard5_rdn5_slot: &mut f64,
        var_guard5_rdn6_slot: &mut f64,
        var_guard5_rdn7_slot: &mut f64,
        var_guard5_rdn8_slot: &mut f64,
        var_guard5_rdn9_slot: &mut f64,
        var_guard5_rv_slot: &mut f64,
        var_ifwd_slot: &mut f64,
        var_ifwd_db0_slot: &mut f64,
        var_ifwd_db1_slot: &mut f64,
        var_ifwd_db2_slot: &mut f64,
        var_ifwd_db3_slot: &mut f64,
        var_ifwd_db4_slot: &mut f64,
        var_ifwd_db5_slot: &mut f64,
        var_ifwd_db6_slot: &mut f64,
        var_ifwd_db7_slot: &mut f64,
        var_ifwd_dn0_slot: &mut f64,
        var_ifwd_dn1_slot: &mut f64,
        var_ifwd_dn2_slot: &mut f64,
        var_ifwd_dn3_slot: &mut f64,
        var_ifwd_dn4_slot: &mut f64,
        var_ifwd_dn5_slot: &mut f64,
        var_ifwd_dn6_slot: &mut f64,
        var_ifwd_dn7_slot: &mut f64,
        var_ifwd_dn8_slot: &mut f64,
        var_ifwd_dn9_slot: &mut f64,
        var_ifwd_rdb0_slot: &mut f64,
        var_ifwd_rdb1_slot: &mut f64,
        var_ifwd_rdb2_slot: &mut f64,
        var_ifwd_rdb3_slot: &mut f64,
        var_ifwd_rdb4_slot: &mut f64,
        var_ifwd_rdb5_slot: &mut f64,
        var_ifwd_rdb6_slot: &mut f64,
        var_ifwd_rdb7_slot: &mut f64,
        var_ifwd_rdn0_slot: &mut f64,
        var_ifwd_rdn1_slot: &mut f64,
        var_ifwd_rdn2_slot: &mut f64,
        var_ifwd_rdn3_slot: &mut f64,
        var_ifwd_rdn4_slot: &mut f64,
        var_ifwd_rdn5_slot: &mut f64,
        var_ifwd_rdn6_slot: &mut f64,
        var_ifwd_rdn7_slot: &mut f64,
        var_ifwd_rdn8_slot: &mut f64,
        var_ifwd_rdn9_slot: &mut f64,
        var_ifwd_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_db0_slot: &mut f64,
        var_le_db1_slot: &mut f64,
        var_le_db2_slot: &mut f64,
        var_le_db3_slot: &mut f64,
        var_le_db4_slot: &mut f64,
        var_le_db5_slot: &mut f64,
        var_le_db6_slot: &mut f64,
        var_le_db7_slot: &mut f64,
        var_le_dn0_slot: &mut f64,
        var_le_dn1_slot: &mut f64,
        var_le_dn2_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn5_slot: &mut f64,
        var_le_dn6_slot: &mut f64,
        var_le_dn7_slot: &mut f64,
        var_le_dn8_slot: &mut f64,
        var_le_dn9_slot: &mut f64,
        var_le_rdb0_slot: &mut f64,
        var_le_rdb1_slot: &mut f64,
        var_le_rdb2_slot: &mut f64,
        var_le_rdb3_slot: &mut f64,
        var_le_rdb4_slot: &mut f64,
        var_le_rdb5_slot: &mut f64,
        var_le_rdb6_slot: &mut f64,
        var_le_rdb7_slot: &mut f64,
        var_le_rdn0_slot: &mut f64,
        var_le_rdn1_slot: &mut f64,
        var_le_rdn2_slot: &mut f64,
        var_le_rdn3_slot: &mut f64,
        var_le_rdn4_slot: &mut f64,
        var_le_rdn5_slot: &mut f64,
        var_le_rdn6_slot: &mut f64,
        var_le_rdn7_slot: &mut f64,
        var_le_rdn8_slot: &mut f64,
        var_le_rdn9_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_lebv_slot: &mut f64,
        var_lebv_db0_slot: &mut f64,
        var_lebv_db1_slot: &mut f64,
        var_lebv_db2_slot: &mut f64,
        var_lebv_db3_slot: &mut f64,
        var_lebv_db4_slot: &mut f64,
        var_lebv_db5_slot: &mut f64,
        var_lebv_db6_slot: &mut f64,
        var_lebv_db7_slot: &mut f64,
        var_lebv_dn0_slot: &mut f64,
        var_lebv_dn1_slot: &mut f64,
        var_lebv_dn2_slot: &mut f64,
        var_lebv_dn3_slot: &mut f64,
        var_lebv_dn4_slot: &mut f64,
        var_lebv_dn5_slot: &mut f64,
        var_lebv_dn6_slot: &mut f64,
        var_lebv_dn7_slot: &mut f64,
        var_lebv_dn8_slot: &mut f64,
        var_lebv_dn9_slot: &mut f64,
        var_lebv_rdb0_slot: &mut f64,
        var_lebv_rdb1_slot: &mut f64,
        var_lebv_rdb2_slot: &mut f64,
        var_lebv_rdb3_slot: &mut f64,
        var_lebv_rdb4_slot: &mut f64,
        var_lebv_rdb5_slot: &mut f64,
        var_lebv_rdb6_slot: &mut f64,
        var_lebv_rdb7_slot: &mut f64,
        var_lebv_rdn0_slot: &mut f64,
        var_lebv_rdn1_slot: &mut f64,
        var_lebv_rdn2_slot: &mut f64,
        var_lebv_rdn3_slot: &mut f64,
        var_lebv_rdn4_slot: &mut f64,
        var_lebv_rdn5_slot: &mut f64,
        var_lebv_rdn6_slot: &mut f64,
        var_lebv_rdn7_slot: &mut f64,
        var_lebv_rdn8_slot: &mut f64,
        var_lebv_rdn9_slot: &mut f64,
        var_lebv_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db2_slot: &mut f64,
        var_t0_db3_slot: &mut f64,
        var_t0_db4_slot: &mut f64,
        var_t0_db5_slot: &mut f64,
        var_t0_db6_slot: &mut f64,
        var_t0_db7_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rdb0_slot: &mut f64,
        var_t0_rdb1_slot: &mut f64,
        var_t0_rdb2_slot: &mut f64,
        var_t0_rdb3_slot: &mut f64,
        var_t0_rdb4_slot: &mut f64,
        var_t0_rdb5_slot: &mut f64,
        var_t0_rdb6_slot: &mut f64,
        var_t0_rdb7_slot: &mut f64,
        var_t0_rdn0_slot: &mut f64,
        var_t0_rdn1_slot: &mut f64,
        var_t0_rdn2_slot: &mut f64,
        var_t0_rdn3_slot: &mut f64,
        var_t0_rdn4_slot: &mut f64,
        var_t0_rdn5_slot: &mut f64,
        var_t0_rdn6_slot: &mut f64,
        var_t0_rdn7_slot: &mut f64,
        var_t0_rdn8_slot: &mut f64,
        var_t0_rdn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_db0: f64 = *var_arg_db0_slot;
        let mut var_arg_db1: f64 = *var_arg_db1_slot;
        let mut var_arg_db2: f64 = *var_arg_db2_slot;
        let mut var_arg_db3: f64 = *var_arg_db3_slot;
        let mut var_arg_db4: f64 = *var_arg_db4_slot;
        let mut var_arg_db5: f64 = *var_arg_db5_slot;
        let mut var_arg_db6: f64 = *var_arg_db6_slot;
        let mut var_arg_db7: f64 = *var_arg_db7_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn1: f64 = *var_arg_dn1_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_arg_rdb0: f64 = *var_arg_rdb0_slot;
        let mut var_arg_rdb1: f64 = *var_arg_rdb1_slot;
        let mut var_arg_rdb2: f64 = *var_arg_rdb2_slot;
        let mut var_arg_rdb3: f64 = *var_arg_rdb3_slot;
        let mut var_arg_rdb4: f64 = *var_arg_rdb4_slot;
        let mut var_arg_rdb5: f64 = *var_arg_rdb5_slot;
        let mut var_arg_rdb6: f64 = *var_arg_rdb6_slot;
        let mut var_arg_rdb7: f64 = *var_arg_rdb7_slot;
        let mut var_arg_rdn0: f64 = *var_arg_rdn0_slot;
        let mut var_arg_rdn1: f64 = *var_arg_rdn1_slot;
        let mut var_arg_rdn2: f64 = *var_arg_rdn2_slot;
        let mut var_arg_rdn3: f64 = *var_arg_rdn3_slot;
        let mut var_arg_rdn4: f64 = *var_arg_rdn4_slot;
        let mut var_arg_rdn5: f64 = *var_arg_rdn5_slot;
        let mut var_arg_rdn6: f64 = *var_arg_rdn6_slot;
        let mut var_arg_rdn7: f64 = *var_arg_rdn7_slot;
        let mut var_arg_rdn8: f64 = *var_arg_rdn8_slot;
        let mut var_arg_rdn9: f64 = *var_arg_rdn9_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard5_db0: f64 = *var_guard5_db0_slot;
        let mut var_guard5_db1: f64 = *var_guard5_db1_slot;
        let mut var_guard5_db2: f64 = *var_guard5_db2_slot;
        let mut var_guard5_db3: f64 = *var_guard5_db3_slot;
        let mut var_guard5_db4: f64 = *var_guard5_db4_slot;
        let mut var_guard5_db5: f64 = *var_guard5_db5_slot;
        let mut var_guard5_db6: f64 = *var_guard5_db6_slot;
        let mut var_guard5_db7: f64 = *var_guard5_db7_slot;
        let mut var_guard5_dn0: f64 = *var_guard5_dn0_slot;
        let mut var_guard5_dn1: f64 = *var_guard5_dn1_slot;
        let mut var_guard5_dn2: f64 = *var_guard5_dn2_slot;
        let mut var_guard5_dn3: f64 = *var_guard5_dn3_slot;
        let mut var_guard5_dn4: f64 = *var_guard5_dn4_slot;
        let mut var_guard5_dn5: f64 = *var_guard5_dn5_slot;
        let mut var_guard5_dn6: f64 = *var_guard5_dn6_slot;
        let mut var_guard5_dn7: f64 = *var_guard5_dn7_slot;
        let mut var_guard5_dn8: f64 = *var_guard5_dn8_slot;
        let mut var_guard5_dn9: f64 = *var_guard5_dn9_slot;
        let mut var_guard5_rdb0: f64 = *var_guard5_rdb0_slot;
        let mut var_guard5_rdb1: f64 = *var_guard5_rdb1_slot;
        let mut var_guard5_rdb2: f64 = *var_guard5_rdb2_slot;
        let mut var_guard5_rdb3: f64 = *var_guard5_rdb3_slot;
        let mut var_guard5_rdb4: f64 = *var_guard5_rdb4_slot;
        let mut var_guard5_rdb5: f64 = *var_guard5_rdb5_slot;
        let mut var_guard5_rdb6: f64 = *var_guard5_rdb6_slot;
        let mut var_guard5_rdb7: f64 = *var_guard5_rdb7_slot;
        let mut var_guard5_rdn0: f64 = *var_guard5_rdn0_slot;
        let mut var_guard5_rdn1: f64 = *var_guard5_rdn1_slot;
        let mut var_guard5_rdn2: f64 = *var_guard5_rdn2_slot;
        let mut var_guard5_rdn3: f64 = *var_guard5_rdn3_slot;
        let mut var_guard5_rdn4: f64 = *var_guard5_rdn4_slot;
        let mut var_guard5_rdn5: f64 = *var_guard5_rdn5_slot;
        let mut var_guard5_rdn6: f64 = *var_guard5_rdn6_slot;
        let mut var_guard5_rdn7: f64 = *var_guard5_rdn7_slot;
        let mut var_guard5_rdn8: f64 = *var_guard5_rdn8_slot;
        let mut var_guard5_rdn9: f64 = *var_guard5_rdn9_slot;
        let mut var_guard5_rv: f64 = *var_guard5_rv_slot;
        let mut var_ifwd: f64 = *var_ifwd_slot;
        let mut var_ifwd_db0: f64 = *var_ifwd_db0_slot;
        let mut var_ifwd_db1: f64 = *var_ifwd_db1_slot;
        let mut var_ifwd_db2: f64 = *var_ifwd_db2_slot;
        let mut var_ifwd_db3: f64 = *var_ifwd_db3_slot;
        let mut var_ifwd_db4: f64 = *var_ifwd_db4_slot;
        let mut var_ifwd_db5: f64 = *var_ifwd_db5_slot;
        let mut var_ifwd_db6: f64 = *var_ifwd_db6_slot;
        let mut var_ifwd_db7: f64 = *var_ifwd_db7_slot;
        let mut var_ifwd_dn0: f64 = *var_ifwd_dn0_slot;
        let mut var_ifwd_dn1: f64 = *var_ifwd_dn1_slot;
        let mut var_ifwd_dn2: f64 = *var_ifwd_dn2_slot;
        let mut var_ifwd_dn3: f64 = *var_ifwd_dn3_slot;
        let mut var_ifwd_dn4: f64 = *var_ifwd_dn4_slot;
        let mut var_ifwd_dn5: f64 = *var_ifwd_dn5_slot;
        let mut var_ifwd_dn6: f64 = *var_ifwd_dn6_slot;
        let mut var_ifwd_dn7: f64 = *var_ifwd_dn7_slot;
        let mut var_ifwd_dn8: f64 = *var_ifwd_dn8_slot;
        let mut var_ifwd_dn9: f64 = *var_ifwd_dn9_slot;
        let mut var_ifwd_rdb0: f64 = *var_ifwd_rdb0_slot;
        let mut var_ifwd_rdb1: f64 = *var_ifwd_rdb1_slot;
        let mut var_ifwd_rdb2: f64 = *var_ifwd_rdb2_slot;
        let mut var_ifwd_rdb3: f64 = *var_ifwd_rdb3_slot;
        let mut var_ifwd_rdb4: f64 = *var_ifwd_rdb4_slot;
        let mut var_ifwd_rdb5: f64 = *var_ifwd_rdb5_slot;
        let mut var_ifwd_rdb6: f64 = *var_ifwd_rdb6_slot;
        let mut var_ifwd_rdb7: f64 = *var_ifwd_rdb7_slot;
        let mut var_ifwd_rdn0: f64 = *var_ifwd_rdn0_slot;
        let mut var_ifwd_rdn1: f64 = *var_ifwd_rdn1_slot;
        let mut var_ifwd_rdn2: f64 = *var_ifwd_rdn2_slot;
        let mut var_ifwd_rdn3: f64 = *var_ifwd_rdn3_slot;
        let mut var_ifwd_rdn4: f64 = *var_ifwd_rdn4_slot;
        let mut var_ifwd_rdn5: f64 = *var_ifwd_rdn5_slot;
        let mut var_ifwd_rdn6: f64 = *var_ifwd_rdn6_slot;
        let mut var_ifwd_rdn7: f64 = *var_ifwd_rdn7_slot;
        let mut var_ifwd_rdn8: f64 = *var_ifwd_rdn8_slot;
        let mut var_ifwd_rdn9: f64 = *var_ifwd_rdn9_slot;
        let mut var_ifwd_rv: f64 = *var_ifwd_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_db0: f64 = *var_le_db0_slot;
        let mut var_le_db1: f64 = *var_le_db1_slot;
        let mut var_le_db2: f64 = *var_le_db2_slot;
        let mut var_le_db3: f64 = *var_le_db3_slot;
        let mut var_le_db4: f64 = *var_le_db4_slot;
        let mut var_le_db5: f64 = *var_le_db5_slot;
        let mut var_le_db6: f64 = *var_le_db6_slot;
        let mut var_le_db7: f64 = *var_le_db7_slot;
        let mut var_le_dn0: f64 = *var_le_dn0_slot;
        let mut var_le_dn1: f64 = *var_le_dn1_slot;
        let mut var_le_dn2: f64 = *var_le_dn2_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn5: f64 = *var_le_dn5_slot;
        let mut var_le_dn6: f64 = *var_le_dn6_slot;
        let mut var_le_dn7: f64 = *var_le_dn7_slot;
        let mut var_le_dn8: f64 = *var_le_dn8_slot;
        let mut var_le_dn9: f64 = *var_le_dn9_slot;
        let mut var_le_rdb0: f64 = *var_le_rdb0_slot;
        let mut var_le_rdb1: f64 = *var_le_rdb1_slot;
        let mut var_le_rdb2: f64 = *var_le_rdb2_slot;
        let mut var_le_rdb3: f64 = *var_le_rdb3_slot;
        let mut var_le_rdb4: f64 = *var_le_rdb4_slot;
        let mut var_le_rdb5: f64 = *var_le_rdb5_slot;
        let mut var_le_rdb6: f64 = *var_le_rdb6_slot;
        let mut var_le_rdb7: f64 = *var_le_rdb7_slot;
        let mut var_le_rdn0: f64 = *var_le_rdn0_slot;
        let mut var_le_rdn1: f64 = *var_le_rdn1_slot;
        let mut var_le_rdn2: f64 = *var_le_rdn2_slot;
        let mut var_le_rdn3: f64 = *var_le_rdn3_slot;
        let mut var_le_rdn4: f64 = *var_le_rdn4_slot;
        let mut var_le_rdn5: f64 = *var_le_rdn5_slot;
        let mut var_le_rdn6: f64 = *var_le_rdn6_slot;
        let mut var_le_rdn7: f64 = *var_le_rdn7_slot;
        let mut var_le_rdn8: f64 = *var_le_rdn8_slot;
        let mut var_le_rdn9: f64 = *var_le_rdn9_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_lebv: f64 = *var_lebv_slot;
        let mut var_lebv_db0: f64 = *var_lebv_db0_slot;
        let mut var_lebv_db1: f64 = *var_lebv_db1_slot;
        let mut var_lebv_db2: f64 = *var_lebv_db2_slot;
        let mut var_lebv_db3: f64 = *var_lebv_db3_slot;
        let mut var_lebv_db4: f64 = *var_lebv_db4_slot;
        let mut var_lebv_db5: f64 = *var_lebv_db5_slot;
        let mut var_lebv_db6: f64 = *var_lebv_db6_slot;
        let mut var_lebv_db7: f64 = *var_lebv_db7_slot;
        let mut var_lebv_dn0: f64 = *var_lebv_dn0_slot;
        let mut var_lebv_dn1: f64 = *var_lebv_dn1_slot;
        let mut var_lebv_dn2: f64 = *var_lebv_dn2_slot;
        let mut var_lebv_dn3: f64 = *var_lebv_dn3_slot;
        let mut var_lebv_dn4: f64 = *var_lebv_dn4_slot;
        let mut var_lebv_dn5: f64 = *var_lebv_dn5_slot;
        let mut var_lebv_dn6: f64 = *var_lebv_dn6_slot;
        let mut var_lebv_dn7: f64 = *var_lebv_dn7_slot;
        let mut var_lebv_dn8: f64 = *var_lebv_dn8_slot;
        let mut var_lebv_dn9: f64 = *var_lebv_dn9_slot;
        let mut var_lebv_rdb0: f64 = *var_lebv_rdb0_slot;
        let mut var_lebv_rdb1: f64 = *var_lebv_rdb1_slot;
        let mut var_lebv_rdb2: f64 = *var_lebv_rdb2_slot;
        let mut var_lebv_rdb3: f64 = *var_lebv_rdb3_slot;
        let mut var_lebv_rdb4: f64 = *var_lebv_rdb4_slot;
        let mut var_lebv_rdb5: f64 = *var_lebv_rdb5_slot;
        let mut var_lebv_rdb6: f64 = *var_lebv_rdb6_slot;
        let mut var_lebv_rdb7: f64 = *var_lebv_rdb7_slot;
        let mut var_lebv_rdn0: f64 = *var_lebv_rdn0_slot;
        let mut var_lebv_rdn1: f64 = *var_lebv_rdn1_slot;
        let mut var_lebv_rdn2: f64 = *var_lebv_rdn2_slot;
        let mut var_lebv_rdn3: f64 = *var_lebv_rdn3_slot;
        let mut var_lebv_rdn4: f64 = *var_lebv_rdn4_slot;
        let mut var_lebv_rdn5: f64 = *var_lebv_rdn5_slot;
        let mut var_lebv_rdn6: f64 = *var_lebv_rdn6_slot;
        let mut var_lebv_rdn7: f64 = *var_lebv_rdn7_slot;
        let mut var_lebv_rdn8: f64 = *var_lebv_rdn8_slot;
        let mut var_lebv_rdn9: f64 = *var_lebv_rdn9_slot;
        let mut var_lebv_rv: f64 = *var_lebv_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db2: f64 = *var_t0_db2_slot;
        let mut var_t0_db3: f64 = *var_t0_db3_slot;
        let mut var_t0_db4: f64 = *var_t0_db4_slot;
        let mut var_t0_db5: f64 = *var_t0_db5_slot;
        let mut var_t0_db6: f64 = *var_t0_db6_slot;
        let mut var_t0_db7: f64 = *var_t0_db7_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rdb0: f64 = *var_t0_rdb0_slot;
        let mut var_t0_rdb1: f64 = *var_t0_rdb1_slot;
        let mut var_t0_rdb2: f64 = *var_t0_rdb2_slot;
        let mut var_t0_rdb3: f64 = *var_t0_rdb3_slot;
        let mut var_t0_rdb4: f64 = *var_t0_rdb4_slot;
        let mut var_t0_rdb5: f64 = *var_t0_rdb5_slot;
        let mut var_t0_rdb6: f64 = *var_t0_rdb6_slot;
        let mut var_t0_rdb7: f64 = *var_t0_rdb7_slot;
        let mut var_t0_rdn0: f64 = *var_t0_rdn0_slot;
        let mut var_t0_rdn1: f64 = *var_t0_rdn1_slot;
        let mut var_t0_rdn2: f64 = *var_t0_rdn2_slot;
        let mut var_t0_rdn3: f64 = *var_t0_rdn3_slot;
        let mut var_t0_rdn4: f64 = *var_t0_rdn4_slot;
        let mut var_t0_rdn5: f64 = *var_t0_rdn5_slot;
        let mut var_t0_rdn6: f64 = *var_t0_rdn6_slot;
        let mut var_t0_rdn7: f64 = *var_t0_rdn7_slot;
        let mut var_t0_rdn8: f64 = *var_t0_rdn8_slot;
        let mut var_t0_rdn9: f64 = *var_t0_rdn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;

        let (assign760_e977, assign760_e977_d_n0, assign760_e977_d_n1, assign760_e977_d_n2, assign760_e977_d_n3, assign760_e977_d_n4, assign760_e977_d_n5, assign760_e977_d_n6, assign760_e977_d_n7, assign760_e977_d_n8, assign760_e977_d_n9, assign760_e977_d_b0, assign760_e977_d_b1, assign760_e977_d_b2, assign760_e977_d_b3, assign760_e977_d_b4, assign760_e977_d_b5, assign760_e977_d_b6, assign760_e977_d_b7,) = {
    if ((var_guard3 != 0.0) && (var_guard4 != 0.0)) {
        let assign760_e974: f64 = (var_arg - 80.0);
        let assign760_e975: f64 = (1.0 + assign760_e974);
        (assign760_e975, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign760_e977;
        var_le_dn0 = assign760_e977_d_n0;
        var_le_dn1 = assign760_e977_d_n1;
        var_le_dn2 = assign760_e977_d_n2;
        var_le_dn3 = assign760_e977_d_n3;
        var_le_dn4 = assign760_e977_d_n4;
        var_le_dn5 = assign760_e977_d_n5;
        var_le_dn6 = assign760_e977_d_n6;
        var_le_dn7 = assign760_e977_d_n7;
        var_le_dn8 = assign760_e977_d_n8;
        var_le_dn9 = assign760_e977_d_n9;
        var_le_db0 = assign760_e977_d_b0;
        var_le_db1 = assign760_e977_d_b1;
        var_le_db2 = assign760_e977_d_b2;
        var_le_db3 = assign760_e977_d_b3;
        var_le_db4 = assign760_e977_d_b4;
        var_le_db5 = assign760_e977_d_b5;
        var_le_db6 = assign760_e977_d_b6;
        var_le_db7 = assign760_e977_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign770_e983, assign770_e983_d_n0, assign770_e983_d_n1, assign770_e983_d_n2, assign770_e983_d_n3, assign770_e983_d_n4, assign770_e983_d_n5, assign770_e983_d_n6, assign770_e983_d_n7, assign770_e983_d_n8, assign770_e983_d_n9, assign770_e983_d_b0, assign770_e983_d_b1, assign770_e983_d_b2, assign770_e983_d_b3, assign770_e983_d_b4, assign770_e983_d_b5, assign770_e983_d_b6, assign770_e983_d_b7,) = {
    if ((var_guard3 != 0.0) && (var_guard4 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    }
};
        var_arg = assign770_e983;
        var_arg_dn0 = assign770_e983_d_n0;
        var_arg_dn1 = assign770_e983_d_n1;
        var_arg_dn2 = assign770_e983_d_n2;
        var_arg_dn3 = assign770_e983_d_n3;
        var_arg_dn4 = assign770_e983_d_n4;
        var_arg_dn5 = assign770_e983_d_n5;
        var_arg_dn6 = assign770_e983_d_n6;
        var_arg_dn7 = assign770_e983_d_n7;
        var_arg_dn8 = assign770_e983_d_n8;
        var_arg_dn9 = assign770_e983_d_n9;
        var_arg_db0 = assign770_e983_d_b0;
        var_arg_db1 = assign770_e983_d_b1;
        var_arg_db2 = assign770_e983_d_b2;
        var_arg_db3 = assign770_e983_d_b3;
        var_arg_db4 = assign770_e983_d_b4;
        var_arg_db5 = assign770_e983_d_b5;
        var_arg_db6 = assign770_e983_d_b6;
        var_arg_db7 = assign770_e983_d_b7;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdn7 = 0.0;
        var_arg_rdn8 = 0.0;
        var_arg_rdn9 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;
        var_arg_rdb7 = 0.0;

        let (assign780_e990, assign780_e990_d_n0, assign780_e990_d_n1, assign780_e990_d_n2, assign780_e990_d_n3, assign780_e990_d_n4, assign780_e990_d_n5, assign780_e990_d_n6, assign780_e990_d_n7, assign780_e990_d_n8, assign780_e990_d_n9, assign780_e990_d_b0, assign780_e990_d_b1, assign780_e990_d_b2, assign780_e990_d_b3, assign780_e990_d_b4, assign780_e990_d_b5, assign780_e990_d_b6, assign780_e990_d_b7,) = {
    if ((var_guard3 != 0.0) && (var_guard4 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign780_e990;
        var_le_dn0 = assign780_e990_d_n0;
        var_le_dn1 = assign780_e990_d_n1;
        var_le_dn2 = assign780_e990_d_n2;
        var_le_dn3 = assign780_e990_d_n3;
        var_le_dn4 = assign780_e990_d_n4;
        var_le_dn5 = assign780_e990_d_n5;
        var_le_dn6 = assign780_e990_d_n6;
        var_le_dn7 = assign780_e990_d_n7;
        var_le_dn8 = assign780_e990_d_n8;
        var_le_dn9 = assign780_e990_d_n9;
        var_le_db0 = assign780_e990_d_b0;
        var_le_db1 = assign780_e990_d_b1;
        var_le_db2 = assign780_e990_d_b2;
        var_le_db3 = assign780_e990_d_b3;
        var_le_db4 = assign780_e990_d_b4;
        var_le_db5 = assign780_e990_d_b5;
        var_le_db6 = assign780_e990_d_b6;
        var_le_db7 = assign780_e990_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign790_e997, assign790_e997_d_n0, assign790_e997_d_n1, assign790_e997_d_n2, assign790_e997_d_n3, assign790_e997_d_n4, assign790_e997_d_n5, assign790_e997_d_n6, assign790_e997_d_n7, assign790_e997_d_n8, assign790_e997_d_n9, assign790_e997_d_b0, assign790_e997_d_b1, assign790_e997_d_b2, assign790_e997_d_b3, assign790_e997_d_b4, assign790_e997_d_b5, assign790_e997_d_b6, assign790_e997_d_b7,) = {
    if (var_guard3 != 0.0) {
        let assign790_e994: f64 = (var_arg).exp();
        let assign790_e995: f64 = (var_le * assign790_e994);
        (assign790_e995, ((var_le_dn0 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn0))), ((var_le_dn1 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn1))), ((var_le_dn2 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn2))), ((var_le_dn3 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn3))), ((var_le_dn4 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn4))), ((var_le_dn5 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn5))), ((var_le_dn6 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn6))), ((var_le_dn7 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn7))), ((var_le_dn8 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn8))), ((var_le_dn9 * assign790_e994) + (var_le * (assign790_e994 * var_arg_dn9))), ((var_le_db0 * assign790_e994) + (var_le * (assign790_e994 * var_arg_db0))), ((var_le_db1 * assign790_e994) + (var_le * (assign790_e994 * var_arg_db1))), ((var_le_db2 * assign790_e994) + (var_le * (assign790_e994 * var_arg_db2))), ((var_le_db3 * assign790_e994) + (var_le * (assign790_e994 * var_arg_db3))), ((var_le_db4 * assign790_e994) + (var_le * (assign790_e994 * var_arg_db4))), ((var_le_db5 * assign790_e994) + (var_le * (assign790_e994 * var_arg_db5))), ((var_le_db6 * assign790_e994) + (var_le * (assign790_e994 * var_arg_db6))), ((var_le_db7 * assign790_e994) + (var_le * (assign790_e994 * var_arg_db7))),)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign790_e997;
        var_le_dn0 = assign790_e997_d_n0;
        var_le_dn1 = assign790_e997_d_n1;
        var_le_dn2 = assign790_e997_d_n2;
        var_le_dn3 = assign790_e997_d_n3;
        var_le_dn4 = assign790_e997_d_n4;
        var_le_dn5 = assign790_e997_d_n5;
        var_le_dn6 = assign790_e997_d_n6;
        var_le_dn7 = assign790_e997_d_n7;
        var_le_dn8 = assign790_e997_d_n8;
        var_le_dn9 = assign790_e997_d_n9;
        var_le_db0 = assign790_e997_d_b0;
        var_le_db1 = assign790_e997_d_b1;
        var_le_db2 = assign790_e997_d_b2;
        var_le_db3 = assign790_e997_d_b3;
        var_le_db4 = assign790_e997_d_b4;
        var_le_db5 = assign790_e997_d_b5;
        var_le_db6 = assign790_e997_d_b6;
        var_le_db7 = assign790_e997_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign800_e1069, assign800_e1069_d_n0, assign800_e1069_d_n1, assign800_e1069_d_n2, assign800_e1069_d_n3, assign800_e1069_d_n4, assign800_e1069_d_n5, assign800_e1069_d_n6, assign800_e1069_d_n7, assign800_e1069_d_n8, assign800_e1069_d_n9, assign800_e1069_d_b0, assign800_e1069_d_b1, assign800_e1069_d_b2, assign800_e1069_d_b3, assign800_e1069_d_b4, assign800_e1069_d_b5, assign800_e1069_d_b6, assign800_e1069_d_b7,) = {
    if (var_guard3 != 0.0) {
        let assign800_e1005: f64 = (-37.0);
        let (assign800_e1032, assign800_e1032_d_n0, assign800_e1032_d_n1, assign800_e1032_d_n2, assign800_e1032_d_n3, assign800_e1032_d_n4, assign800_e1032_d_n5, assign800_e1032_d_n6, assign800_e1032_d_n7, assign800_e1032_d_n8, assign800_e1032_d_n9, assign800_e1032_d_b0, assign800_e1032_d_b1, assign800_e1032_d_b2, assign800_e1032_d_b3, assign800_e1032_d_b4, assign800_e1032_d_b5, assign800_e1032_d_b6, assign800_e1032_d_b7,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign800_e1005))) {
                let assign800_e1010: f64 = (var_argbv).exp();
                let assign800_e1012: f64 = (assign800_e1010 + 1.0);
                let assign800_e1013: f64 = (assign800_e1012).ln();
                (assign800_e1013, ((assign800_e1010 * var_argbv_dn0) / assign800_e1012), ((assign800_e1010 * var_argbv_dn1) / assign800_e1012), ((assign800_e1010 * var_argbv_dn2) / assign800_e1012), ((assign800_e1010 * var_argbv_dn3) / assign800_e1012), ((assign800_e1010 * var_argbv_dn4) / assign800_e1012), ((assign800_e1010 * var_argbv_dn5) / assign800_e1012), ((assign800_e1010 * var_argbv_dn6) / assign800_e1012), ((assign800_e1010 * var_argbv_dn7) / assign800_e1012), ((assign800_e1010 * var_argbv_dn8) / assign800_e1012), ((assign800_e1010 * var_argbv_dn9) / assign800_e1012), ((assign800_e1010 * var_argbv_db0) / assign800_e1012), ((assign800_e1010 * var_argbv_db1) / assign800_e1012), ((assign800_e1010 * var_argbv_db2) / assign800_e1012), ((assign800_e1010 * var_argbv_db3) / assign800_e1012), ((assign800_e1010 * var_argbv_db4) / assign800_e1012), ((assign800_e1010 * var_argbv_db5) / assign800_e1012), ((assign800_e1010 * var_argbv_db6) / assign800_e1012), ((assign800_e1010 * var_argbv_db7) / assign800_e1012),)
            } else {
                let assign800_e1020: f64 = (-37.0);
                let (assign800_e1031, assign800_e1031_d_n0, assign800_e1031_d_n1, assign800_e1031_d_n2, assign800_e1031_d_n3, assign800_e1031_d_n4, assign800_e1031_d_n5, assign800_e1031_d_n6, assign800_e1031_d_n7, assign800_e1031_d_n8, assign800_e1031_d_n9, assign800_e1031_d_b0, assign800_e1031_d_b1, assign800_e1031_d_b2, assign800_e1031_d_b3, assign800_e1031_d_b4, assign800_e1031_d_b5, assign800_e1031_d_b6, assign800_e1031_d_b7,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign800_e1020)) {
                        let assign800_e1024: f64 = (var_argbv).exp();
                        (assign800_e1024, (assign800_e1024 * var_argbv_dn0), (assign800_e1024 * var_argbv_dn1), (assign800_e1024 * var_argbv_dn2), (assign800_e1024 * var_argbv_dn3), (assign800_e1024 * var_argbv_dn4), (assign800_e1024 * var_argbv_dn5), (assign800_e1024 * var_argbv_dn6), (assign800_e1024 * var_argbv_dn7), (assign800_e1024 * var_argbv_dn8), (assign800_e1024 * var_argbv_dn9), (assign800_e1024 * var_argbv_db0), (assign800_e1024 * var_argbv_db1), (assign800_e1024 * var_argbv_db2), (assign800_e1024 * var_argbv_db3), (assign800_e1024 * var_argbv_db4), (assign800_e1024 * var_argbv_db5), (assign800_e1024 * var_argbv_db6), (assign800_e1024 * var_argbv_db7),)
                    } else {
                        let (assign800_e1030, assign800_e1030_d_n0, assign800_e1030_d_n1, assign800_e1030_d_n2, assign800_e1030_d_n3, assign800_e1030_d_n4, assign800_e1030_d_n5, assign800_e1030_d_n6, assign800_e1030_d_n7, assign800_e1030_d_n8, assign800_e1030_d_n9, assign800_e1030_d_b0, assign800_e1030_d_b1, assign800_e1030_d_b2, assign800_e1030_d_b3, assign800_e1030_d_b4, assign800_e1030_d_b5, assign800_e1030_d_b6, assign800_e1030_d_b7,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn0, var_argbv_dn1, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6, var_argbv_dn7, var_argbv_dn8, var_argbv_dn9, var_argbv_db0, var_argbv_db1, var_argbv_db2, var_argbv_db3, var_argbv_db4, var_argbv_db5, var_argbv_db6, var_argbv_db7,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign800_e1030, assign800_e1030_d_n0, assign800_e1030_d_n1, assign800_e1030_d_n2, assign800_e1030_d_n3, assign800_e1030_d_n4, assign800_e1030_d_n5, assign800_e1030_d_n6, assign800_e1030_d_n7, assign800_e1030_d_n8, assign800_e1030_d_n9, assign800_e1030_d_b0, assign800_e1030_d_b1, assign800_e1030_d_b2, assign800_e1030_d_b3, assign800_e1030_d_b4, assign800_e1030_d_b5, assign800_e1030_d_b6, assign800_e1030_d_b7,)
                    }
                };
                (assign800_e1031, assign800_e1031_d_n0, assign800_e1031_d_n1, assign800_e1031_d_n2, assign800_e1031_d_n3, assign800_e1031_d_n4, assign800_e1031_d_n5, assign800_e1031_d_n6, assign800_e1031_d_n7, assign800_e1031_d_n8, assign800_e1031_d_n9, assign800_e1031_d_b0, assign800_e1031_d_b1, assign800_e1031_d_b2, assign800_e1031_d_b3, assign800_e1031_d_b4, assign800_e1031_d_b5, assign800_e1031_d_b6, assign800_e1031_d_b7,)
            }
        };
        let assign800_e1039: f64 = (-37.0);
        let (assign800_e1066, assign800_e1066_d_n0, assign800_e1066_d_n1, assign800_e1066_d_n2, assign800_e1066_d_n3, assign800_e1066_d_n4, assign800_e1066_d_n5, assign800_e1066_d_n6, assign800_e1066_d_n7, assign800_e1066_d_n8, assign800_e1066_d_n9, assign800_e1066_d_b0, assign800_e1066_d_b1, assign800_e1066_d_b2, assign800_e1066_d_b3, assign800_e1066_d_b4, assign800_e1066_d_b5, assign800_e1066_d_b6, assign800_e1066_d_b7,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign800_e1039))) {
                let assign800_e1044: f64 = (var_argbvvt).exp();
                let assign800_e1046: f64 = (assign800_e1044 + 1.0);
                let assign800_e1047: f64 = (assign800_e1046).ln();
                (assign800_e1047, ((assign800_e1044 * var_argbvvt_dn0) / assign800_e1046), ((assign800_e1044 * var_argbvvt_dn1) / assign800_e1046), ((assign800_e1044 * var_argbvvt_dn2) / assign800_e1046), ((assign800_e1044 * var_argbvvt_dn3) / assign800_e1046), ((assign800_e1044 * var_argbvvt_dn4) / assign800_e1046), ((assign800_e1044 * var_argbvvt_dn5) / assign800_e1046), ((assign800_e1044 * var_argbvvt_dn6) / assign800_e1046), ((assign800_e1044 * var_argbvvt_dn7) / assign800_e1046), ((assign800_e1044 * var_argbvvt_dn8) / assign800_e1046), ((assign800_e1044 * var_argbvvt_dn9) / assign800_e1046), ((assign800_e1044 * var_argbvvt_db0) / assign800_e1046), ((assign800_e1044 * var_argbvvt_db1) / assign800_e1046), ((assign800_e1044 * var_argbvvt_db2) / assign800_e1046), ((assign800_e1044 * var_argbvvt_db3) / assign800_e1046), ((assign800_e1044 * var_argbvvt_db4) / assign800_e1046), ((assign800_e1044 * var_argbvvt_db5) / assign800_e1046), ((assign800_e1044 * var_argbvvt_db6) / assign800_e1046), ((assign800_e1044 * var_argbvvt_db7) / assign800_e1046),)
            } else {
                let assign800_e1054: f64 = (-37.0);
                let (assign800_e1065, assign800_e1065_d_n0, assign800_e1065_d_n1, assign800_e1065_d_n2, assign800_e1065_d_n3, assign800_e1065_d_n4, assign800_e1065_d_n5, assign800_e1065_d_n6, assign800_e1065_d_n7, assign800_e1065_d_n8, assign800_e1065_d_n9, assign800_e1065_d_b0, assign800_e1065_d_b1, assign800_e1065_d_b2, assign800_e1065_d_b3, assign800_e1065_d_b4, assign800_e1065_d_b5, assign800_e1065_d_b6, assign800_e1065_d_b7,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign800_e1054)) {
                        let assign800_e1058: f64 = (var_argbvvt).exp();
                        (assign800_e1058, (assign800_e1058 * var_argbvvt_dn0), (assign800_e1058 * var_argbvvt_dn1), (assign800_e1058 * var_argbvvt_dn2), (assign800_e1058 * var_argbvvt_dn3), (assign800_e1058 * var_argbvvt_dn4), (assign800_e1058 * var_argbvvt_dn5), (assign800_e1058 * var_argbvvt_dn6), (assign800_e1058 * var_argbvvt_dn7), (assign800_e1058 * var_argbvvt_dn8), (assign800_e1058 * var_argbvvt_dn9), (assign800_e1058 * var_argbvvt_db0), (assign800_e1058 * var_argbvvt_db1), (assign800_e1058 * var_argbvvt_db2), (assign800_e1058 * var_argbvvt_db3), (assign800_e1058 * var_argbvvt_db4), (assign800_e1058 * var_argbvvt_db5), (assign800_e1058 * var_argbvvt_db6), (assign800_e1058 * var_argbvvt_db7),)
                    } else {
                        let (assign800_e1064, assign800_e1064_d_n0, assign800_e1064_d_n1, assign800_e1064_d_n2, assign800_e1064_d_n3, assign800_e1064_d_n4, assign800_e1064_d_n5, assign800_e1064_d_n6, assign800_e1064_d_n7, assign800_e1064_d_n8, assign800_e1064_d_n9, assign800_e1064_d_b0, assign800_e1064_d_b1, assign800_e1064_d_b2, assign800_e1064_d_b3, assign800_e1064_d_b4, assign800_e1064_d_b5, assign800_e1064_d_b6, assign800_e1064_d_b7,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn0, var_argbvvt_dn1, var_argbvvt_dn2, var_argbvvt_dn3, var_argbvvt_dn4, var_argbvvt_dn5, var_argbvvt_dn6, var_argbvvt_dn7, var_argbvvt_dn8, var_argbvvt_dn9, var_argbvvt_db0, var_argbvvt_db1, var_argbvvt_db2, var_argbvvt_db3, var_argbvvt_db4, var_argbvvt_db5, var_argbvvt_db6, var_argbvvt_db7,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign800_e1064, assign800_e1064_d_n0, assign800_e1064_d_n1, assign800_e1064_d_n2, assign800_e1064_d_n3, assign800_e1064_d_n4, assign800_e1064_d_n5, assign800_e1064_d_n6, assign800_e1064_d_n7, assign800_e1064_d_n8, assign800_e1064_d_n9, assign800_e1064_d_b0, assign800_e1064_d_b1, assign800_e1064_d_b2, assign800_e1064_d_b3, assign800_e1064_d_b4, assign800_e1064_d_b5, assign800_e1064_d_b6, assign800_e1064_d_b7,)
                    }
                };
                (assign800_e1065, assign800_e1065_d_n0, assign800_e1065_d_n1, assign800_e1065_d_n2, assign800_e1065_d_n3, assign800_e1065_d_n4, assign800_e1065_d_n5, assign800_e1065_d_n6, assign800_e1065_d_n7, assign800_e1065_d_n8, assign800_e1065_d_n9, assign800_e1065_d_b0, assign800_e1065_d_b1, assign800_e1065_d_b2, assign800_e1065_d_b3, assign800_e1065_d_b4, assign800_e1065_d_b5, assign800_e1065_d_b6, assign800_e1065_d_b7,)
            }
        };
        let assign800_e1067: f64 = (assign800_e1032 - assign800_e1066);
        (assign800_e1067, (assign800_e1032_d_n0 - assign800_e1066_d_n0), (assign800_e1032_d_n1 - assign800_e1066_d_n1), (assign800_e1032_d_n2 - assign800_e1066_d_n2), (assign800_e1032_d_n3 - assign800_e1066_d_n3), (assign800_e1032_d_n4 - assign800_e1066_d_n4), (assign800_e1032_d_n5 - assign800_e1066_d_n5), (assign800_e1032_d_n6 - assign800_e1066_d_n6), (assign800_e1032_d_n7 - assign800_e1066_d_n7), (assign800_e1032_d_n8 - assign800_e1066_d_n8), (assign800_e1032_d_n9 - assign800_e1066_d_n9), (assign800_e1032_d_b0 - assign800_e1066_d_b0), (assign800_e1032_d_b1 - assign800_e1066_d_b1), (assign800_e1032_d_b2 - assign800_e1066_d_b2), (assign800_e1032_d_b3 - assign800_e1066_d_b3), (assign800_e1032_d_b4 - assign800_e1066_d_b4), (assign800_e1032_d_b5 - assign800_e1066_d_b5), (assign800_e1032_d_b6 - assign800_e1066_d_b6), (assign800_e1032_d_b7 - assign800_e1066_d_b7),)
    } else {
        (var_lebv, var_lebv_dn0, var_lebv_dn1, var_lebv_dn2, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6, var_lebv_dn7, var_lebv_dn8, var_lebv_dn9, var_lebv_db0, var_lebv_db1, var_lebv_db2, var_lebv_db3, var_lebv_db4, var_lebv_db5, var_lebv_db6, var_lebv_db7,)
    }
};
        var_lebv = assign800_e1069;
        var_lebv_dn0 = assign800_e1069_d_n0;
        var_lebv_dn1 = assign800_e1069_d_n1;
        var_lebv_dn2 = assign800_e1069_d_n2;
        var_lebv_dn3 = assign800_e1069_d_n3;
        var_lebv_dn4 = assign800_e1069_d_n4;
        var_lebv_dn5 = assign800_e1069_d_n5;
        var_lebv_dn6 = assign800_e1069_d_n6;
        var_lebv_dn7 = assign800_e1069_d_n7;
        var_lebv_dn8 = assign800_e1069_d_n8;
        var_lebv_dn9 = assign800_e1069_d_n9;
        var_lebv_db0 = assign800_e1069_d_b0;
        var_lebv_db1 = assign800_e1069_d_b1;
        var_lebv_db2 = assign800_e1069_d_b2;
        var_lebv_db3 = assign800_e1069_d_b3;
        var_lebv_db4 = assign800_e1069_d_b4;
        var_lebv_db5 = assign800_e1069_d_b5;
        var_lebv_db6 = assign800_e1069_d_b6;
        var_lebv_db7 = assign800_e1069_d_b7;
        var_lebv_rv = 0.0;
        var_lebv_rdn0 = 0.0;
        var_lebv_rdn1 = 0.0;
        var_lebv_rdn2 = 0.0;
        var_lebv_rdn3 = 0.0;
        var_lebv_rdn4 = 0.0;
        var_lebv_rdn5 = 0.0;
        var_lebv_rdn6 = 0.0;
        var_lebv_rdn7 = 0.0;
        var_lebv_rdn8 = 0.0;
        var_lebv_rdn9 = 0.0;
        var_lebv_rdb0 = 0.0;
        var_lebv_rdb1 = 0.0;
        var_lebv_rdb2 = 0.0;
        var_lebv_rdb3 = 0.0;
        var_lebv_rdb4 = 0.0;
        var_lebv_rdb5 = 0.0;
        var_lebv_rdb6 = 0.0;
        var_lebv_rdb7 = 0.0;

        let (assign810_e1090, assign810_e1090_d_n0, assign810_e1090_d_n1, assign810_e1090_d_n2, assign810_e1090_d_n3, assign810_e1090_d_n4, assign810_e1090_d_n5, assign810_e1090_d_n6, assign810_e1090_d_n7, assign810_e1090_d_n8, assign810_e1090_d_n9, assign810_e1090_d_b0, assign810_e1090_d_b1, assign810_e1090_d_b2, assign810_e1090_d_b3, assign810_e1090_d_b4, assign810_e1090_d_b5, assign810_e1090_d_b6, assign810_e1090_d_b7,) = {
    if (var_guard3 != 0.0) {
        let assign810_e1074: f64 = (var_le - 1.0);
        let assign810_e1075: f64 = (var_is_t * assign810_e1074);
        let assign810_e1078: f64 = (var_ijbv_t * var_lebv);
        let assign810_e1082: f64 = (var_vbiei).abs();
        let assign810_e1084: f64 = (assign810_e1082).powf(var_theexp_t);
        let assign810_e1085: f64 = (p.p8 * assign810_e1084);
        let assign810_e1086: f64 = (1.0 + assign810_e1085);
        let assign810_e1087: f64 = (assign810_e1078 / assign810_e1086);
        let assign810_e1088: f64 = (assign810_e1075 - assign810_e1087);
        (assign810_e1088, (((var_is_t_dn0 * assign810_e1074) + (var_is_t * var_le_dn0)) - (((((var_ijbv_t_dn0 * var_lebv) + (var_ijbv_t * var_lebv_dn0)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_dn0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn0 } else { (-var_vbiei_dn0) })) } } else { (assign810_e1084 * ((var_theexp_t_dn0 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn0 } else { (-var_vbiei_dn0) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_dn1 * assign810_e1074) + (var_is_t * var_le_dn1)) - (((((var_ijbv_t_dn1 * var_lebv) + (var_ijbv_t * var_lebv_dn1)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_dn1 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn1 } else { (-var_vbiei_dn1) })) } } else { (assign810_e1084 * ((var_theexp_t_dn1 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn1 } else { (-var_vbiei_dn1) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_dn2 * assign810_e1074) + (var_is_t * var_le_dn2)) - (((((var_ijbv_t_dn2 * var_lebv) + (var_ijbv_t * var_lebv_dn2)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_dn2 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn2 } else { (-var_vbiei_dn2) })) } } else { (assign810_e1084 * ((var_theexp_t_dn2 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn2 } else { (-var_vbiei_dn2) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_dn3 * assign810_e1074) + (var_is_t * var_le_dn3)) - (((((var_ijbv_t_dn3 * var_lebv) + (var_ijbv_t * var_lebv_dn3)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_dn3 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn3 } else { (-var_vbiei_dn3) })) } } else { (assign810_e1084 * ((var_theexp_t_dn3 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn3 } else { (-var_vbiei_dn3) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_dn4 * assign810_e1074) + (var_is_t * var_le_dn4)) - (((((var_ijbv_t_dn4 * var_lebv) + (var_ijbv_t * var_lebv_dn4)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_dn4 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn4 } else { (-var_vbiei_dn4) })) } } else { (assign810_e1084 * ((var_theexp_t_dn4 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn4 } else { (-var_vbiei_dn4) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_dn5 * assign810_e1074) + (var_is_t * var_le_dn5)) - (((((var_ijbv_t_dn5 * var_lebv) + (var_ijbv_t * var_lebv_dn5)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_dn5 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn5 } else { (-var_vbiei_dn5) })) } } else { (assign810_e1084 * ((var_theexp_t_dn5 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn5 } else { (-var_vbiei_dn5) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_dn6 * assign810_e1074) + (var_is_t * var_le_dn6)) - (((((var_ijbv_t_dn6 * var_lebv) + (var_ijbv_t * var_lebv_dn6)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_dn6 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn6 } else { (-var_vbiei_dn6) })) } } else { (assign810_e1084 * ((var_theexp_t_dn6 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn6 } else { (-var_vbiei_dn6) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_dn7 * assign810_e1074) + (var_is_t * var_le_dn7)) - (((((var_ijbv_t_dn7 * var_lebv) + (var_ijbv_t * var_lebv_dn7)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_dn7 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn7 } else { (-var_vbiei_dn7) })) } } else { (assign810_e1084 * ((var_theexp_t_dn7 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn7 } else { (-var_vbiei_dn7) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_dn8 * assign810_e1074) + (var_is_t * var_le_dn8)) - (((((var_ijbv_t_dn8 * var_lebv) + (var_ijbv_t * var_lebv_dn8)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_dn8 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn8 } else { (-var_vbiei_dn8) })) } } else { (assign810_e1084 * ((var_theexp_t_dn8 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn8 } else { (-var_vbiei_dn8) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_dn9 * assign810_e1074) + (var_is_t * var_le_dn9)) - (((((var_ijbv_t_dn9 * var_lebv) + (var_ijbv_t * var_lebv_dn9)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_dn9 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_dn9 } else { (-var_vbiei_dn9) })) } } else { (assign810_e1084 * ((var_theexp_t_dn9 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_dn9 } else { (-var_vbiei_dn9) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_db0 * assign810_e1074) + (var_is_t * var_le_db0)) - (((((var_ijbv_t_db0 * var_lebv) + (var_ijbv_t * var_lebv_db0)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_db0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db0 } else { (-var_vbiei_db0) })) } } else { (assign810_e1084 * ((var_theexp_t_db0 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db0 } else { (-var_vbiei_db0) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_db1 * assign810_e1074) + (var_is_t * var_le_db1)) - (((((var_ijbv_t_db1 * var_lebv) + (var_ijbv_t * var_lebv_db1)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_db1 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db1 } else { (-var_vbiei_db1) })) } } else { (assign810_e1084 * ((var_theexp_t_db1 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db1 } else { (-var_vbiei_db1) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_db2 * assign810_e1074) + (var_is_t * var_le_db2)) - (((((var_ijbv_t_db2 * var_lebv) + (var_ijbv_t * var_lebv_db2)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_db2 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db2 } else { (-var_vbiei_db2) })) } } else { (assign810_e1084 * ((var_theexp_t_db2 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db2 } else { (-var_vbiei_db2) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_db3 * assign810_e1074) + (var_is_t * var_le_db3)) - (((((var_ijbv_t_db3 * var_lebv) + (var_ijbv_t * var_lebv_db3)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_db3 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db3 } else { (-var_vbiei_db3) })) } } else { (assign810_e1084 * ((var_theexp_t_db3 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db3 } else { (-var_vbiei_db3) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_db4 * assign810_e1074) + (var_is_t * var_le_db4)) - (((((var_ijbv_t_db4 * var_lebv) + (var_ijbv_t * var_lebv_db4)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_db4 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db4 } else { (-var_vbiei_db4) })) } } else { (assign810_e1084 * ((var_theexp_t_db4 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db4 } else { (-var_vbiei_db4) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_db5 * assign810_e1074) + (var_is_t * var_le_db5)) - (((((var_ijbv_t_db5 * var_lebv) + (var_ijbv_t * var_lebv_db5)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_db5 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db5 } else { (-var_vbiei_db5) })) } } else { (assign810_e1084 * ((var_theexp_t_db5 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db5 } else { (-var_vbiei_db5) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_db6 * assign810_e1074) + (var_is_t * var_le_db6)) - (((((var_ijbv_t_db6 * var_lebv) + (var_ijbv_t * var_lebv_db6)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_db6 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db6 } else { (-var_vbiei_db6) })) } } else { (assign810_e1084 * ((var_theexp_t_db6 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db6 } else { (-var_vbiei_db6) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))), (((var_is_t_db7 * assign810_e1074) + (var_is_t * var_le_db7)) - (((((var_ijbv_t_db7 * var_lebv) + (var_ijbv_t * var_lebv_db7)) * assign810_e1086) - (assign810_e1078 * (p.p8 * if var_theexp_t_db7 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign810_e1082).powf(var_theexp_t - 1.0) * if var_vbiei >= 0.0 { var_vbiei_db7 } else { (-var_vbiei_db7) })) } } else { (assign810_e1084 * ((var_theexp_t_db7 * (assign810_e1082).ln()) + (var_theexp_t * (if var_vbiei >= 0.0 { var_vbiei_db7 } else { (-var_vbiei_db7) } / assign810_e1082)))) }))) / (assign810_e1086 * assign810_e1086))),)
    } else {
        (var_ifwd, var_ifwd_dn0, var_ifwd_dn1, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_ifwd_dn7, var_ifwd_dn8, var_ifwd_dn9, var_ifwd_db0, var_ifwd_db1, var_ifwd_db2, var_ifwd_db3, var_ifwd_db4, var_ifwd_db5, var_ifwd_db6, var_ifwd_db7,)
    }
};
        var_ifwd = assign810_e1090;
        var_ifwd_dn0 = assign810_e1090_d_n0;
        var_ifwd_dn1 = assign810_e1090_d_n1;
        var_ifwd_dn2 = assign810_e1090_d_n2;
        var_ifwd_dn3 = assign810_e1090_d_n3;
        var_ifwd_dn4 = assign810_e1090_d_n4;
        var_ifwd_dn5 = assign810_e1090_d_n5;
        var_ifwd_dn6 = assign810_e1090_d_n6;
        var_ifwd_dn7 = assign810_e1090_d_n7;
        var_ifwd_dn8 = assign810_e1090_d_n8;
        var_ifwd_dn9 = assign810_e1090_d_n9;
        var_ifwd_db0 = assign810_e1090_d_b0;
        var_ifwd_db1 = assign810_e1090_d_b1;
        var_ifwd_db2 = assign810_e1090_d_b2;
        var_ifwd_db3 = assign810_e1090_d_b3;
        var_ifwd_db4 = assign810_e1090_d_b4;
        var_ifwd_db5 = assign810_e1090_d_b5;
        var_ifwd_db6 = assign810_e1090_d_b6;
        var_ifwd_db7 = assign810_e1090_d_b7;
        var_ifwd_rv = 0.0;
        var_ifwd_rdn0 = 0.0;
        var_ifwd_rdn1 = 0.0;
        var_ifwd_rdn2 = 0.0;
        var_ifwd_rdn3 = 0.0;
        var_ifwd_rdn4 = 0.0;
        var_ifwd_rdn5 = 0.0;
        var_ifwd_rdn6 = 0.0;
        var_ifwd_rdn7 = 0.0;
        var_ifwd_rdn8 = 0.0;
        var_ifwd_rdn9 = 0.0;
        var_ifwd_rdb0 = 0.0;
        var_ifwd_rdb1 = 0.0;
        var_ifwd_rdb2 = 0.0;
        var_ifwd_rdb3 = 0.0;
        var_ifwd_rdb4 = 0.0;
        var_ifwd_rdb5 = 0.0;
        var_ifwd_rdb6 = 0.0;
        var_ifwd_rdb7 = 0.0;

        let (assign820_e1095, assign820_e1095_d_n0, assign820_e1095_d_n1, assign820_e1095_d_n2, assign820_e1095_d_n3, assign820_e1095_d_n4, assign820_e1095_d_n5, assign820_e1095_d_n6, assign820_e1095_d_n7, assign820_e1095_d_n8, assign820_e1095_d_n9, assign820_e1095_d_b0, assign820_e1095_d_b1, assign820_e1095_d_b2, assign820_e1095_d_b3, assign820_e1095_d_b4, assign820_e1095_d_b5, assign820_e1095_d_b6, assign820_e1095_d_b7,) = {
    if (var_guard3 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ifwd, var_ifwd_dn0, var_ifwd_dn1, var_ifwd_dn2, var_ifwd_dn3, var_ifwd_dn4, var_ifwd_dn5, var_ifwd_dn6, var_ifwd_dn7, var_ifwd_dn8, var_ifwd_dn9, var_ifwd_db0, var_ifwd_db1, var_ifwd_db2, var_ifwd_db3, var_ifwd_db4, var_ifwd_db5, var_ifwd_db6, var_ifwd_db7,)
    }
};
        var_ifwd = assign820_e1095;
        var_ifwd_dn0 = assign820_e1095_d_n0;
        var_ifwd_dn1 = assign820_e1095_d_n1;
        var_ifwd_dn2 = assign820_e1095_d_n2;
        var_ifwd_dn3 = assign820_e1095_d_n3;
        var_ifwd_dn4 = assign820_e1095_d_n4;
        var_ifwd_dn5 = assign820_e1095_d_n5;
        var_ifwd_dn6 = assign820_e1095_d_n6;
        var_ifwd_dn7 = assign820_e1095_d_n7;
        var_ifwd_dn8 = assign820_e1095_d_n8;
        var_ifwd_dn9 = assign820_e1095_d_n9;
        var_ifwd_db0 = assign820_e1095_d_b0;
        var_ifwd_db1 = assign820_e1095_d_b1;
        var_ifwd_db2 = assign820_e1095_d_b2;
        var_ifwd_db3 = assign820_e1095_d_b3;
        var_ifwd_db4 = assign820_e1095_d_b4;
        var_ifwd_db5 = assign820_e1095_d_b5;
        var_ifwd_db6 = assign820_e1095_d_b6;
        var_ifwd_db7 = assign820_e1095_d_b7;
        var_ifwd_rv = 0.0;
        var_ifwd_rdn0 = 0.0;
        var_ifwd_rdn1 = 0.0;
        var_ifwd_rdn2 = 0.0;
        var_ifwd_rdn3 = 0.0;
        var_ifwd_rdn4 = 0.0;
        var_ifwd_rdn5 = 0.0;
        var_ifwd_rdn6 = 0.0;
        var_ifwd_rdn7 = 0.0;
        var_ifwd_rdn8 = 0.0;
        var_ifwd_rdn9 = 0.0;
        var_ifwd_rdb0 = 0.0;
        var_ifwd_rdb1 = 0.0;
        var_ifwd_rdb2 = 0.0;
        var_ifwd_rdb3 = 0.0;
        var_ifwd_rdb4 = 0.0;
        var_ifwd_rdb5 = 0.0;
        var_ifwd_rdb6 = 0.0;
        var_ifwd_rdb7 = 0.0;

        let assign830_e1098: f64 = if var_isr_t > 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign830_e1098;
        var_guard5_dn0 = 0.0;
        var_guard5_dn1 = 0.0;
        var_guard5_dn2 = 0.0;
        var_guard5_dn3 = 0.0;
        var_guard5_dn4 = 0.0;
        var_guard5_dn5 = 0.0;
        var_guard5_dn6 = 0.0;
        var_guard5_dn7 = 0.0;
        var_guard5_dn8 = 0.0;
        var_guard5_dn9 = 0.0;
        var_guard5_db0 = 0.0;
        var_guard5_db1 = 0.0;
        var_guard5_db2 = 0.0;
        var_guard5_db3 = 0.0;
        var_guard5_db4 = 0.0;
        var_guard5_db5 = 0.0;
        var_guard5_db6 = 0.0;
        var_guard5_db7 = 0.0;
        var_guard5_rv = 0.0;
        var_guard5_rdn0 = 0.0;
        var_guard5_rdn1 = 0.0;
        var_guard5_rdn2 = 0.0;
        var_guard5_rdn3 = 0.0;
        var_guard5_rdn4 = 0.0;
        var_guard5_rdn5 = 0.0;
        var_guard5_rdn6 = 0.0;
        var_guard5_rdn7 = 0.0;
        var_guard5_rdn8 = 0.0;
        var_guard5_rdn9 = 0.0;
        var_guard5_rdb0 = 0.0;
        var_guard5_rdb1 = 0.0;
        var_guard5_rdb2 = 0.0;
        var_guard5_rdb3 = 0.0;
        var_guard5_rdb4 = 0.0;
        var_guard5_rdb5 = 0.0;
        var_guard5_rdb6 = 0.0;
        var_guard5_rdb7 = 0.0;

        let (assign840_e1106, assign840_e1106_d_n0, assign840_e1106_d_n1, assign840_e1106_d_n2, assign840_e1106_d_n3, assign840_e1106_d_n4, assign840_e1106_d_n5, assign840_e1106_d_n6, assign840_e1106_d_n7, assign840_e1106_d_n8, assign840_e1106_d_n9, assign840_e1106_d_b0, assign840_e1106_d_b1, assign840_e1106_d_b2, assign840_e1106_d_b3, assign840_e1106_d_b4, assign840_e1106_d_b5, assign840_e1106_d_b6, assign840_e1106_d_b7,) = {
    if (var_guard5 != 0.0) {
        let assign840_e1102: f64 = (p.p4 - var_vbiei);
        let assign840_e1104: f64 = (assign840_e1102).max(0.001);
        (assign840_e1104, if assign840_e1102 >= 0.001 { (-var_vbiei_dn0) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_dn1) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_dn2) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_dn3) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_dn4) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_dn5) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_dn6) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_dn7) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_dn8) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_dn9) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_db0) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_db1) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_db2) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_db3) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_db4) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_db5) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_db6) } else { 0.0 }, if assign840_e1102 >= 0.001 { (-var_vbiei_db7) } else { 0.0 },)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4, var_t0_db5, var_t0_db6, var_t0_db7,)
    }
};
        var_t0 = assign840_e1106;
        var_t0_dn0 = assign840_e1106_d_n0;
        var_t0_dn1 = assign840_e1106_d_n1;
        var_t0_dn2 = assign840_e1106_d_n2;
        var_t0_dn3 = assign840_e1106_d_n3;
        var_t0_dn4 = assign840_e1106_d_n4;
        var_t0_dn5 = assign840_e1106_d_n5;
        var_t0_dn6 = assign840_e1106_d_n6;
        var_t0_dn7 = assign840_e1106_d_n7;
        var_t0_dn8 = assign840_e1106_d_n8;
        var_t0_dn9 = assign840_e1106_d_n9;
        var_t0_db0 = assign840_e1106_d_b0;
        var_t0_db1 = assign840_e1106_d_b1;
        var_t0_db2 = assign840_e1106_d_b2;
        var_t0_db3 = assign840_e1106_d_b3;
        var_t0_db4 = assign840_e1106_d_b4;
        var_t0_db5 = assign840_e1106_d_b5;
        var_t0_db6 = assign840_e1106_d_b6;
        var_t0_db7 = assign840_e1106_d_b7;
        var_t0_rv = 0.0;
        var_t0_rdn0 = 0.0;
        var_t0_rdn1 = 0.0;
        var_t0_rdn2 = 0.0;
        var_t0_rdn3 = 0.0;
        var_t0_rdn4 = 0.0;
        var_t0_rdn5 = 0.0;
        var_t0_rdn6 = 0.0;
        var_t0_rdn7 = 0.0;
        var_t0_rdn8 = 0.0;
        var_t0_rdn9 = 0.0;
        var_t0_rdb0 = 0.0;
        var_t0_rdb1 = 0.0;
        var_t0_rdb2 = 0.0;
        var_t0_rdb3 = 0.0;
        var_t0_rdb4 = 0.0;
        var_t0_rdb5 = 0.0;
        var_t0_rdb6 = 0.0;
        var_t0_rdb7 = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_db0_slot = var_arg_db0;
        *var_arg_db1_slot = var_arg_db1;
        *var_arg_db2_slot = var_arg_db2;
        *var_arg_db3_slot = var_arg_db3;
        *var_arg_db4_slot = var_arg_db4;
        *var_arg_db5_slot = var_arg_db5;
        *var_arg_db6_slot = var_arg_db6;
        *var_arg_db7_slot = var_arg_db7;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn1_slot = var_arg_dn1;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_arg_rdb0_slot = var_arg_rdb0;
        *var_arg_rdb1_slot = var_arg_rdb1;
        *var_arg_rdb2_slot = var_arg_rdb2;
        *var_arg_rdb3_slot = var_arg_rdb3;
        *var_arg_rdb4_slot = var_arg_rdb4;
        *var_arg_rdb5_slot = var_arg_rdb5;
        *var_arg_rdb6_slot = var_arg_rdb6;
        *var_arg_rdb7_slot = var_arg_rdb7;
        *var_arg_rdn0_slot = var_arg_rdn0;
        *var_arg_rdn1_slot = var_arg_rdn1;
        *var_arg_rdn2_slot = var_arg_rdn2;
        *var_arg_rdn3_slot = var_arg_rdn3;
        *var_arg_rdn4_slot = var_arg_rdn4;
        *var_arg_rdn5_slot = var_arg_rdn5;
        *var_arg_rdn6_slot = var_arg_rdn6;
        *var_arg_rdn7_slot = var_arg_rdn7;
        *var_arg_rdn8_slot = var_arg_rdn8;
        *var_arg_rdn9_slot = var_arg_rdn9;
        *var_arg_rv_slot = var_arg_rv;
        *var_guard5_slot = var_guard5;
        *var_guard5_db0_slot = var_guard5_db0;
        *var_guard5_db1_slot = var_guard5_db1;
        *var_guard5_db2_slot = var_guard5_db2;
        *var_guard5_db3_slot = var_guard5_db3;
        *var_guard5_db4_slot = var_guard5_db4;
        *var_guard5_db5_slot = var_guard5_db5;
        *var_guard5_db6_slot = var_guard5_db6;
        *var_guard5_db7_slot = var_guard5_db7;
        *var_guard5_dn0_slot = var_guard5_dn0;
        *var_guard5_dn1_slot = var_guard5_dn1;
        *var_guard5_dn2_slot = var_guard5_dn2;
        *var_guard5_dn3_slot = var_guard5_dn3;
        *var_guard5_dn4_slot = var_guard5_dn4;
        *var_guard5_dn5_slot = var_guard5_dn5;
        *var_guard5_dn6_slot = var_guard5_dn6;
        *var_guard5_dn7_slot = var_guard5_dn7;
        *var_guard5_dn8_slot = var_guard5_dn8;
        *var_guard5_dn9_slot = var_guard5_dn9;
        *var_guard5_rdb0_slot = var_guard5_rdb0;
        *var_guard5_rdb1_slot = var_guard5_rdb1;
        *var_guard5_rdb2_slot = var_guard5_rdb2;
        *var_guard5_rdb3_slot = var_guard5_rdb3;
        *var_guard5_rdb4_slot = var_guard5_rdb4;
        *var_guard5_rdb5_slot = var_guard5_rdb5;
        *var_guard5_rdb6_slot = var_guard5_rdb6;
        *var_guard5_rdb7_slot = var_guard5_rdb7;
        *var_guard5_rdn0_slot = var_guard5_rdn0;
        *var_guard5_rdn1_slot = var_guard5_rdn1;
        *var_guard5_rdn2_slot = var_guard5_rdn2;
        *var_guard5_rdn3_slot = var_guard5_rdn3;
        *var_guard5_rdn4_slot = var_guard5_rdn4;
        *var_guard5_rdn5_slot = var_guard5_rdn5;
        *var_guard5_rdn6_slot = var_guard5_rdn6;
        *var_guard5_rdn7_slot = var_guard5_rdn7;
        *var_guard5_rdn8_slot = var_guard5_rdn8;
        *var_guard5_rdn9_slot = var_guard5_rdn9;
        *var_guard5_rv_slot = var_guard5_rv;
        *var_ifwd_slot = var_ifwd;
        *var_ifwd_db0_slot = var_ifwd_db0;
        *var_ifwd_db1_slot = var_ifwd_db1;
        *var_ifwd_db2_slot = var_ifwd_db2;
        *var_ifwd_db3_slot = var_ifwd_db3;
        *var_ifwd_db4_slot = var_ifwd_db4;
        *var_ifwd_db5_slot = var_ifwd_db5;
        *var_ifwd_db6_slot = var_ifwd_db6;
        *var_ifwd_db7_slot = var_ifwd_db7;
        *var_ifwd_dn0_slot = var_ifwd_dn0;
        *var_ifwd_dn1_slot = var_ifwd_dn1;
        *var_ifwd_dn2_slot = var_ifwd_dn2;
        *var_ifwd_dn3_slot = var_ifwd_dn3;
        *var_ifwd_dn4_slot = var_ifwd_dn4;
        *var_ifwd_dn5_slot = var_ifwd_dn5;
        *var_ifwd_dn6_slot = var_ifwd_dn6;
        *var_ifwd_dn7_slot = var_ifwd_dn7;
        *var_ifwd_dn8_slot = var_ifwd_dn8;
        *var_ifwd_dn9_slot = var_ifwd_dn9;
        *var_ifwd_rdb0_slot = var_ifwd_rdb0;
        *var_ifwd_rdb1_slot = var_ifwd_rdb1;
        *var_ifwd_rdb2_slot = var_ifwd_rdb2;
        *var_ifwd_rdb3_slot = var_ifwd_rdb3;
        *var_ifwd_rdb4_slot = var_ifwd_rdb4;
        *var_ifwd_rdb5_slot = var_ifwd_rdb5;
        *var_ifwd_rdb6_slot = var_ifwd_rdb6;
        *var_ifwd_rdb7_slot = var_ifwd_rdb7;
        *var_ifwd_rdn0_slot = var_ifwd_rdn0;
        *var_ifwd_rdn1_slot = var_ifwd_rdn1;
        *var_ifwd_rdn2_slot = var_ifwd_rdn2;
        *var_ifwd_rdn3_slot = var_ifwd_rdn3;
        *var_ifwd_rdn4_slot = var_ifwd_rdn4;
        *var_ifwd_rdn5_slot = var_ifwd_rdn5;
        *var_ifwd_rdn6_slot = var_ifwd_rdn6;
        *var_ifwd_rdn7_slot = var_ifwd_rdn7;
        *var_ifwd_rdn8_slot = var_ifwd_rdn8;
        *var_ifwd_rdn9_slot = var_ifwd_rdn9;
        *var_ifwd_rv_slot = var_ifwd_rv;
        *var_le_slot = var_le;
        *var_le_db0_slot = var_le_db0;
        *var_le_db1_slot = var_le_db1;
        *var_le_db2_slot = var_le_db2;
        *var_le_db3_slot = var_le_db3;
        *var_le_db4_slot = var_le_db4;
        *var_le_db5_slot = var_le_db5;
        *var_le_db6_slot = var_le_db6;
        *var_le_db7_slot = var_le_db7;
        *var_le_dn0_slot = var_le_dn0;
        *var_le_dn1_slot = var_le_dn1;
        *var_le_dn2_slot = var_le_dn2;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn5_slot = var_le_dn5;
        *var_le_dn6_slot = var_le_dn6;
        *var_le_dn7_slot = var_le_dn7;
        *var_le_dn8_slot = var_le_dn8;
        *var_le_dn9_slot = var_le_dn9;
        *var_le_rdb0_slot = var_le_rdb0;
        *var_le_rdb1_slot = var_le_rdb1;
        *var_le_rdb2_slot = var_le_rdb2;
        *var_le_rdb3_slot = var_le_rdb3;
        *var_le_rdb4_slot = var_le_rdb4;
        *var_le_rdb5_slot = var_le_rdb5;
        *var_le_rdb6_slot = var_le_rdb6;
        *var_le_rdb7_slot = var_le_rdb7;
        *var_le_rdn0_slot = var_le_rdn0;
        *var_le_rdn1_slot = var_le_rdn1;
        *var_le_rdn2_slot = var_le_rdn2;
        *var_le_rdn3_slot = var_le_rdn3;
        *var_le_rdn4_slot = var_le_rdn4;
        *var_le_rdn5_slot = var_le_rdn5;
        *var_le_rdn6_slot = var_le_rdn6;
        *var_le_rdn7_slot = var_le_rdn7;
        *var_le_rdn8_slot = var_le_rdn8;
        *var_le_rdn9_slot = var_le_rdn9;
        *var_le_rv_slot = var_le_rv;
        *var_lebv_slot = var_lebv;
        *var_lebv_db0_slot = var_lebv_db0;
        *var_lebv_db1_slot = var_lebv_db1;
        *var_lebv_db2_slot = var_lebv_db2;
        *var_lebv_db3_slot = var_lebv_db3;
        *var_lebv_db4_slot = var_lebv_db4;
        *var_lebv_db5_slot = var_lebv_db5;
        *var_lebv_db6_slot = var_lebv_db6;
        *var_lebv_db7_slot = var_lebv_db7;
        *var_lebv_dn0_slot = var_lebv_dn0;
        *var_lebv_dn1_slot = var_lebv_dn1;
        *var_lebv_dn2_slot = var_lebv_dn2;
        *var_lebv_dn3_slot = var_lebv_dn3;
        *var_lebv_dn4_slot = var_lebv_dn4;
        *var_lebv_dn5_slot = var_lebv_dn5;
        *var_lebv_dn6_slot = var_lebv_dn6;
        *var_lebv_dn7_slot = var_lebv_dn7;
        *var_lebv_dn8_slot = var_lebv_dn8;
        *var_lebv_dn9_slot = var_lebv_dn9;
        *var_lebv_rdb0_slot = var_lebv_rdb0;
        *var_lebv_rdb1_slot = var_lebv_rdb1;
        *var_lebv_rdb2_slot = var_lebv_rdb2;
        *var_lebv_rdb3_slot = var_lebv_rdb3;
        *var_lebv_rdb4_slot = var_lebv_rdb4;
        *var_lebv_rdb5_slot = var_lebv_rdb5;
        *var_lebv_rdb6_slot = var_lebv_rdb6;
        *var_lebv_rdb7_slot = var_lebv_rdb7;
        *var_lebv_rdn0_slot = var_lebv_rdn0;
        *var_lebv_rdn1_slot = var_lebv_rdn1;
        *var_lebv_rdn2_slot = var_lebv_rdn2;
        *var_lebv_rdn3_slot = var_lebv_rdn3;
        *var_lebv_rdn4_slot = var_lebv_rdn4;
        *var_lebv_rdn5_slot = var_lebv_rdn5;
        *var_lebv_rdn6_slot = var_lebv_rdn6;
        *var_lebv_rdn7_slot = var_lebv_rdn7;
        *var_lebv_rdn8_slot = var_lebv_rdn8;
        *var_lebv_rdn9_slot = var_lebv_rdn9;
        *var_lebv_rv_slot = var_lebv_rv;
        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db2_slot = var_t0_db2;
        *var_t0_db3_slot = var_t0_db3;
        *var_t0_db4_slot = var_t0_db4;
        *var_t0_db5_slot = var_t0_db5;
        *var_t0_db6_slot = var_t0_db6;
        *var_t0_db7_slot = var_t0_db7;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rdb0_slot = var_t0_rdb0;
        *var_t0_rdb1_slot = var_t0_rdb1;
        *var_t0_rdb2_slot = var_t0_rdb2;
        *var_t0_rdb3_slot = var_t0_rdb3;
        *var_t0_rdb4_slot = var_t0_rdb4;
        *var_t0_rdb5_slot = var_t0_rdb5;
        *var_t0_rdb6_slot = var_t0_rdb6;
        *var_t0_rdb7_slot = var_t0_rdb7;
        *var_t0_rdn0_slot = var_t0_rdn0;
        *var_t0_rdn1_slot = var_t0_rdn1;
        *var_t0_rdn2_slot = var_t0_rdn2;
        *var_t0_rdn3_slot = var_t0_rdn3;
        *var_t0_rdn4_slot = var_t0_rdn4;
        *var_t0_rdn5_slot = var_t0_rdn5;
        *var_t0_rdn6_slot = var_t0_rdn6;
        *var_t0_rdn7_slot = var_t0_rdn7;
        *var_t0_rdn8_slot = var_t0_rdn8;
        *var_t0_rdn9_slot = var_t0_rdn9;
        *var_t0_rv_slot = var_t0_rv;
    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        var_bvr_t: f64,
        var_bvr_t_db0: f64,
        var_bvr_t_db1: f64,
        var_bvr_t_db2: f64,
        var_bvr_t_db3: f64,
        var_bvr_t_db4: f64,
        var_bvr_t_db5: f64,
        var_bvr_t_db6: f64,
        var_bvr_t_db7: f64,
        var_bvr_t_dn0: f64,
        var_bvr_t_dn1: f64,
        var_bvr_t_dn2: f64,
        var_bvr_t_dn3: f64,
        var_bvr_t_dn4: f64,
        var_bvr_t_dn5: f64,
        var_bvr_t_dn6: f64,
        var_bvr_t_dn7: f64,
        var_bvr_t_dn8: f64,
        var_bvr_t_dn9: f64,
        var_guard5: f64,
        var_ise_t: f64,
        var_t0: f64,
        var_t0_db0: f64,
        var_t0_db1: f64,
        var_t0_db2: f64,
        var_t0_db3: f64,
        var_t0_db4: f64,
        var_t0_db5: f64,
        var_t0_db6: f64,
        var_t0_db7: f64,
        var_t0_dn0: f64,
        var_t0_dn1: f64,
        var_t0_dn2: f64,
        var_t0_dn3: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn7: f64,
        var_t0_dn8: f64,
        var_t0_dn9: f64,
        var_vbiei: f64,
        var_vbiei_db0: f64,
        var_vbiei_db1: f64,
        var_vbiei_db2: f64,
        var_vbiei_db3: f64,
        var_vbiei_db4: f64,
        var_vbiei_db5: f64,
        var_vbiei_db6: f64,
        var_vbiei_db7: f64,
        var_vbiei_dn0: f64,
        var_vbiei_dn1: f64,
        var_vbiei_dn2: f64,
        var_vbiei_dn3: f64,
        var_vbiei_dn4: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vbiei_dn7: f64,
        var_vbiei_dn8: f64,
        var_vbiei_dn9: f64,
        var_vt: f64,
        var_vt_db0: f64,
        var_vt_db1: f64,
        var_vt_db2: f64,
        var_vt_db3: f64,
        var_vt_db4: f64,
        var_vt_db5: f64,
        var_vt_db6: f64,
        var_vt_db7: f64,
        var_vt_dn0: f64,
        var_vt_dn1: f64,
        var_vt_dn2: f64,
        var_vt_dn3: f64,
        var_vt_dn4: f64,
        var_vt_dn5: f64,
        var_vt_dn6: f64,
        var_vt_dn7: f64,
        var_vt_dn8: f64,
        var_vt_dn9: f64,
        var_arg_slot: &mut f64,
        var_arg_db0_slot: &mut f64,
        var_arg_db1_slot: &mut f64,
        var_arg_db2_slot: &mut f64,
        var_arg_db3_slot: &mut f64,
        var_arg_db4_slot: &mut f64,
        var_arg_db5_slot: &mut f64,
        var_arg_db6_slot: &mut f64,
        var_arg_db7_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn1_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_arg_rdb0_slot: &mut f64,
        var_arg_rdb1_slot: &mut f64,
        var_arg_rdb2_slot: &mut f64,
        var_arg_rdb3_slot: &mut f64,
        var_arg_rdb4_slot: &mut f64,
        var_arg_rdb5_slot: &mut f64,
        var_arg_rdb6_slot: &mut f64,
        var_arg_rdb7_slot: &mut f64,
        var_arg_rdn0_slot: &mut f64,
        var_arg_rdn1_slot: &mut f64,
        var_arg_rdn2_slot: &mut f64,
        var_arg_rdn3_slot: &mut f64,
        var_arg_rdn4_slot: &mut f64,
        var_arg_rdn5_slot: &mut f64,
        var_arg_rdn6_slot: &mut f64,
        var_arg_rdn7_slot: &mut f64,
        var_arg_rdn8_slot: &mut f64,
        var_arg_rdn9_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_argbv_slot: &mut f64,
        var_argbv_db0_slot: &mut f64,
        var_argbv_db1_slot: &mut f64,
        var_argbv_db2_slot: &mut f64,
        var_argbv_db3_slot: &mut f64,
        var_argbv_db4_slot: &mut f64,
        var_argbv_db5_slot: &mut f64,
        var_argbv_db6_slot: &mut f64,
        var_argbv_db7_slot: &mut f64,
        var_argbv_dn0_slot: &mut f64,
        var_argbv_dn1_slot: &mut f64,
        var_argbv_dn2_slot: &mut f64,
        var_argbv_dn3_slot: &mut f64,
        var_argbv_dn4_slot: &mut f64,
        var_argbv_dn5_slot: &mut f64,
        var_argbv_dn6_slot: &mut f64,
        var_argbv_dn7_slot: &mut f64,
        var_argbv_dn8_slot: &mut f64,
        var_argbv_dn9_slot: &mut f64,
        var_argbv_rdb0_slot: &mut f64,
        var_argbv_rdb1_slot: &mut f64,
        var_argbv_rdb2_slot: &mut f64,
        var_argbv_rdb3_slot: &mut f64,
        var_argbv_rdb4_slot: &mut f64,
        var_argbv_rdb5_slot: &mut f64,
        var_argbv_rdb6_slot: &mut f64,
        var_argbv_rdb7_slot: &mut f64,
        var_argbv_rdn0_slot: &mut f64,
        var_argbv_rdn1_slot: &mut f64,
        var_argbv_rdn2_slot: &mut f64,
        var_argbv_rdn3_slot: &mut f64,
        var_argbv_rdn4_slot: &mut f64,
        var_argbv_rdn5_slot: &mut f64,
        var_argbv_rdn6_slot: &mut f64,
        var_argbv_rdn7_slot: &mut f64,
        var_argbv_rdn8_slot: &mut f64,
        var_argbv_rdn9_slot: &mut f64,
        var_argbv_rv_slot: &mut f64,
        var_argbvvt_slot: &mut f64,
        var_argbvvt_db0_slot: &mut f64,
        var_argbvvt_db1_slot: &mut f64,
        var_argbvvt_db2_slot: &mut f64,
        var_argbvvt_db3_slot: &mut f64,
        var_argbvvt_db4_slot: &mut f64,
        var_argbvvt_db5_slot: &mut f64,
        var_argbvvt_db6_slot: &mut f64,
        var_argbvvt_db7_slot: &mut f64,
        var_argbvvt_dn0_slot: &mut f64,
        var_argbvvt_dn1_slot: &mut f64,
        var_argbvvt_dn2_slot: &mut f64,
        var_argbvvt_dn3_slot: &mut f64,
        var_argbvvt_dn4_slot: &mut f64,
        var_argbvvt_dn5_slot: &mut f64,
        var_argbvvt_dn6_slot: &mut f64,
        var_argbvvt_dn7_slot: &mut f64,
        var_argbvvt_dn8_slot: &mut f64,
        var_argbvvt_dn9_slot: &mut f64,
        var_argbvvt_rdb0_slot: &mut f64,
        var_argbvvt_rdb1_slot: &mut f64,
        var_argbvvt_rdb2_slot: &mut f64,
        var_argbvvt_rdb3_slot: &mut f64,
        var_argbvvt_rdb4_slot: &mut f64,
        var_argbvvt_rdb5_slot: &mut f64,
        var_argbvvt_rdb6_slot: &mut f64,
        var_argbvvt_rdb7_slot: &mut f64,
        var_argbvvt_rdn0_slot: &mut f64,
        var_argbvvt_rdn1_slot: &mut f64,
        var_argbvvt_rdn2_slot: &mut f64,
        var_argbvvt_rdn3_slot: &mut f64,
        var_argbvvt_rdn4_slot: &mut f64,
        var_argbvvt_rdn5_slot: &mut f64,
        var_argbvvt_rdn6_slot: &mut f64,
        var_argbvvt_rdn7_slot: &mut f64,
        var_argbvvt_rdn8_slot: &mut f64,
        var_argbvvt_rdn9_slot: &mut f64,
        var_argbvvt_rv_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard6_db0_slot: &mut f64,
        var_guard6_db1_slot: &mut f64,
        var_guard6_db2_slot: &mut f64,
        var_guard6_db3_slot: &mut f64,
        var_guard6_db4_slot: &mut f64,
        var_guard6_db5_slot: &mut f64,
        var_guard6_db6_slot: &mut f64,
        var_guard6_db7_slot: &mut f64,
        var_guard6_dn0_slot: &mut f64,
        var_guard6_dn1_slot: &mut f64,
        var_guard6_dn2_slot: &mut f64,
        var_guard6_dn3_slot: &mut f64,
        var_guard6_dn4_slot: &mut f64,
        var_guard6_dn5_slot: &mut f64,
        var_guard6_dn6_slot: &mut f64,
        var_guard6_dn7_slot: &mut f64,
        var_guard6_dn8_slot: &mut f64,
        var_guard6_dn9_slot: &mut f64,
        var_guard6_rdb0_slot: &mut f64,
        var_guard6_rdb1_slot: &mut f64,
        var_guard6_rdb2_slot: &mut f64,
        var_guard6_rdb3_slot: &mut f64,
        var_guard6_rdb4_slot: &mut f64,
        var_guard6_rdb5_slot: &mut f64,
        var_guard6_rdb6_slot: &mut f64,
        var_guard6_rdb7_slot: &mut f64,
        var_guard6_rdn0_slot: &mut f64,
        var_guard6_rdn1_slot: &mut f64,
        var_guard6_rdn2_slot: &mut f64,
        var_guard6_rdn3_slot: &mut f64,
        var_guard6_rdn4_slot: &mut f64,
        var_guard6_rdn5_slot: &mut f64,
        var_guard6_rdn6_slot: &mut f64,
        var_guard6_rdn7_slot: &mut f64,
        var_guard6_rdn8_slot: &mut f64,
        var_guard6_rdn9_slot: &mut f64,
        var_guard6_rv_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard7_db0_slot: &mut f64,
        var_guard7_db1_slot: &mut f64,
        var_guard7_db2_slot: &mut f64,
        var_guard7_db3_slot: &mut f64,
        var_guard7_db4_slot: &mut f64,
        var_guard7_db5_slot: &mut f64,
        var_guard7_db6_slot: &mut f64,
        var_guard7_db7_slot: &mut f64,
        var_guard7_dn0_slot: &mut f64,
        var_guard7_dn1_slot: &mut f64,
        var_guard7_dn2_slot: &mut f64,
        var_guard7_dn3_slot: &mut f64,
        var_guard7_dn4_slot: &mut f64,
        var_guard7_dn5_slot: &mut f64,
        var_guard7_dn6_slot: &mut f64,
        var_guard7_dn7_slot: &mut f64,
        var_guard7_dn8_slot: &mut f64,
        var_guard7_dn9_slot: &mut f64,
        var_guard7_rdb0_slot: &mut f64,
        var_guard7_rdb1_slot: &mut f64,
        var_guard7_rdb2_slot: &mut f64,
        var_guard7_rdb3_slot: &mut f64,
        var_guard7_rdb4_slot: &mut f64,
        var_guard7_rdb5_slot: &mut f64,
        var_guard7_rdb6_slot: &mut f64,
        var_guard7_rdb7_slot: &mut f64,
        var_guard7_rdn0_slot: &mut f64,
        var_guard7_rdn1_slot: &mut f64,
        var_guard7_rdn2_slot: &mut f64,
        var_guard7_rdn3_slot: &mut f64,
        var_guard7_rdn4_slot: &mut f64,
        var_guard7_rdn5_slot: &mut f64,
        var_guard7_rdn6_slot: &mut f64,
        var_guard7_rdn7_slot: &mut f64,
        var_guard7_rdn8_slot: &mut f64,
        var_guard7_rdn9_slot: &mut f64,
        var_guard7_rv_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard8_db0_slot: &mut f64,
        var_guard8_db1_slot: &mut f64,
        var_guard8_db2_slot: &mut f64,
        var_guard8_db3_slot: &mut f64,
        var_guard8_db4_slot: &mut f64,
        var_guard8_db5_slot: &mut f64,
        var_guard8_db6_slot: &mut f64,
        var_guard8_db7_slot: &mut f64,
        var_guard8_dn0_slot: &mut f64,
        var_guard8_dn1_slot: &mut f64,
        var_guard8_dn2_slot: &mut f64,
        var_guard8_dn3_slot: &mut f64,
        var_guard8_dn4_slot: &mut f64,
        var_guard8_dn5_slot: &mut f64,
        var_guard8_dn6_slot: &mut f64,
        var_guard8_dn7_slot: &mut f64,
        var_guard8_dn8_slot: &mut f64,
        var_guard8_dn9_slot: &mut f64,
        var_guard8_rdb0_slot: &mut f64,
        var_guard8_rdb1_slot: &mut f64,
        var_guard8_rdb2_slot: &mut f64,
        var_guard8_rdb3_slot: &mut f64,
        var_guard8_rdb4_slot: &mut f64,
        var_guard8_rdb5_slot: &mut f64,
        var_guard8_rdb6_slot: &mut f64,
        var_guard8_rdb7_slot: &mut f64,
        var_guard8_rdn0_slot: &mut f64,
        var_guard8_rdn1_slot: &mut f64,
        var_guard8_rdn2_slot: &mut f64,
        var_guard8_rdn3_slot: &mut f64,
        var_guard8_rdn4_slot: &mut f64,
        var_guard8_rdn5_slot: &mut f64,
        var_guard8_rdn6_slot: &mut f64,
        var_guard8_rdn7_slot: &mut f64,
        var_guard8_rdn8_slot: &mut f64,
        var_guard8_rdn9_slot: &mut f64,
        var_guard8_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_db0_slot: &mut f64,
        var_le_db1_slot: &mut f64,
        var_le_db2_slot: &mut f64,
        var_le_db3_slot: &mut f64,
        var_le_db4_slot: &mut f64,
        var_le_db5_slot: &mut f64,
        var_le_db6_slot: &mut f64,
        var_le_db7_slot: &mut f64,
        var_le_dn0_slot: &mut f64,
        var_le_dn1_slot: &mut f64,
        var_le_dn2_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn5_slot: &mut f64,
        var_le_dn6_slot: &mut f64,
        var_le_dn7_slot: &mut f64,
        var_le_dn8_slot: &mut f64,
        var_le_dn9_slot: &mut f64,
        var_le_rdb0_slot: &mut f64,
        var_le_rdb1_slot: &mut f64,
        var_le_rdb2_slot: &mut f64,
        var_le_rdb3_slot: &mut f64,
        var_le_rdb4_slot: &mut f64,
        var_le_rdb5_slot: &mut f64,
        var_le_rdb6_slot: &mut f64,
        var_le_rdb7_slot: &mut f64,
        var_le_rdn0_slot: &mut f64,
        var_le_rdn1_slot: &mut f64,
        var_le_rdn2_slot: &mut f64,
        var_le_rdn3_slot: &mut f64,
        var_le_rdn4_slot: &mut f64,
        var_le_rdn5_slot: &mut f64,
        var_le_rdn6_slot: &mut f64,
        var_le_rdn7_slot: &mut f64,
        var_le_rdn8_slot: &mut f64,
        var_le_rdn9_slot: &mut f64,
        var_le_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_db0: f64 = *var_arg_db0_slot;
        let mut var_arg_db1: f64 = *var_arg_db1_slot;
        let mut var_arg_db2: f64 = *var_arg_db2_slot;
        let mut var_arg_db3: f64 = *var_arg_db3_slot;
        let mut var_arg_db4: f64 = *var_arg_db4_slot;
        let mut var_arg_db5: f64 = *var_arg_db5_slot;
        let mut var_arg_db6: f64 = *var_arg_db6_slot;
        let mut var_arg_db7: f64 = *var_arg_db7_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn1: f64 = *var_arg_dn1_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_arg_rdb0: f64 = *var_arg_rdb0_slot;
        let mut var_arg_rdb1: f64 = *var_arg_rdb1_slot;
        let mut var_arg_rdb2: f64 = *var_arg_rdb2_slot;
        let mut var_arg_rdb3: f64 = *var_arg_rdb3_slot;
        let mut var_arg_rdb4: f64 = *var_arg_rdb4_slot;
        let mut var_arg_rdb5: f64 = *var_arg_rdb5_slot;
        let mut var_arg_rdb6: f64 = *var_arg_rdb6_slot;
        let mut var_arg_rdb7: f64 = *var_arg_rdb7_slot;
        let mut var_arg_rdn0: f64 = *var_arg_rdn0_slot;
        let mut var_arg_rdn1: f64 = *var_arg_rdn1_slot;
        let mut var_arg_rdn2: f64 = *var_arg_rdn2_slot;
        let mut var_arg_rdn3: f64 = *var_arg_rdn3_slot;
        let mut var_arg_rdn4: f64 = *var_arg_rdn4_slot;
        let mut var_arg_rdn5: f64 = *var_arg_rdn5_slot;
        let mut var_arg_rdn6: f64 = *var_arg_rdn6_slot;
        let mut var_arg_rdn7: f64 = *var_arg_rdn7_slot;
        let mut var_arg_rdn8: f64 = *var_arg_rdn8_slot;
        let mut var_arg_rdn9: f64 = *var_arg_rdn9_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_argbv: f64 = *var_argbv_slot;
        let mut var_argbv_db0: f64 = *var_argbv_db0_slot;
        let mut var_argbv_db1: f64 = *var_argbv_db1_slot;
        let mut var_argbv_db2: f64 = *var_argbv_db2_slot;
        let mut var_argbv_db3: f64 = *var_argbv_db3_slot;
        let mut var_argbv_db4: f64 = *var_argbv_db4_slot;
        let mut var_argbv_db5: f64 = *var_argbv_db5_slot;
        let mut var_argbv_db6: f64 = *var_argbv_db6_slot;
        let mut var_argbv_db7: f64 = *var_argbv_db7_slot;
        let mut var_argbv_dn0: f64 = *var_argbv_dn0_slot;
        let mut var_argbv_dn1: f64 = *var_argbv_dn1_slot;
        let mut var_argbv_dn2: f64 = *var_argbv_dn2_slot;
        let mut var_argbv_dn3: f64 = *var_argbv_dn3_slot;
        let mut var_argbv_dn4: f64 = *var_argbv_dn4_slot;
        let mut var_argbv_dn5: f64 = *var_argbv_dn5_slot;
        let mut var_argbv_dn6: f64 = *var_argbv_dn6_slot;
        let mut var_argbv_dn7: f64 = *var_argbv_dn7_slot;
        let mut var_argbv_dn8: f64 = *var_argbv_dn8_slot;
        let mut var_argbv_dn9: f64 = *var_argbv_dn9_slot;
        let mut var_argbv_rdb0: f64 = *var_argbv_rdb0_slot;
        let mut var_argbv_rdb1: f64 = *var_argbv_rdb1_slot;
        let mut var_argbv_rdb2: f64 = *var_argbv_rdb2_slot;
        let mut var_argbv_rdb3: f64 = *var_argbv_rdb3_slot;
        let mut var_argbv_rdb4: f64 = *var_argbv_rdb4_slot;
        let mut var_argbv_rdb5: f64 = *var_argbv_rdb5_slot;
        let mut var_argbv_rdb6: f64 = *var_argbv_rdb6_slot;
        let mut var_argbv_rdb7: f64 = *var_argbv_rdb7_slot;
        let mut var_argbv_rdn0: f64 = *var_argbv_rdn0_slot;
        let mut var_argbv_rdn1: f64 = *var_argbv_rdn1_slot;
        let mut var_argbv_rdn2: f64 = *var_argbv_rdn2_slot;
        let mut var_argbv_rdn3: f64 = *var_argbv_rdn3_slot;
        let mut var_argbv_rdn4: f64 = *var_argbv_rdn4_slot;
        let mut var_argbv_rdn5: f64 = *var_argbv_rdn5_slot;
        let mut var_argbv_rdn6: f64 = *var_argbv_rdn6_slot;
        let mut var_argbv_rdn7: f64 = *var_argbv_rdn7_slot;
        let mut var_argbv_rdn8: f64 = *var_argbv_rdn8_slot;
        let mut var_argbv_rdn9: f64 = *var_argbv_rdn9_slot;
        let mut var_argbv_rv: f64 = *var_argbv_rv_slot;
        let mut var_argbvvt: f64 = *var_argbvvt_slot;
        let mut var_argbvvt_db0: f64 = *var_argbvvt_db0_slot;
        let mut var_argbvvt_db1: f64 = *var_argbvvt_db1_slot;
        let mut var_argbvvt_db2: f64 = *var_argbvvt_db2_slot;
        let mut var_argbvvt_db3: f64 = *var_argbvvt_db3_slot;
        let mut var_argbvvt_db4: f64 = *var_argbvvt_db4_slot;
        let mut var_argbvvt_db5: f64 = *var_argbvvt_db5_slot;
        let mut var_argbvvt_db6: f64 = *var_argbvvt_db6_slot;
        let mut var_argbvvt_db7: f64 = *var_argbvvt_db7_slot;
        let mut var_argbvvt_dn0: f64 = *var_argbvvt_dn0_slot;
        let mut var_argbvvt_dn1: f64 = *var_argbvvt_dn1_slot;
        let mut var_argbvvt_dn2: f64 = *var_argbvvt_dn2_slot;
        let mut var_argbvvt_dn3: f64 = *var_argbvvt_dn3_slot;
        let mut var_argbvvt_dn4: f64 = *var_argbvvt_dn4_slot;
        let mut var_argbvvt_dn5: f64 = *var_argbvvt_dn5_slot;
        let mut var_argbvvt_dn6: f64 = *var_argbvvt_dn6_slot;
        let mut var_argbvvt_dn7: f64 = *var_argbvvt_dn7_slot;
        let mut var_argbvvt_dn8: f64 = *var_argbvvt_dn8_slot;
        let mut var_argbvvt_dn9: f64 = *var_argbvvt_dn9_slot;
        let mut var_argbvvt_rdb0: f64 = *var_argbvvt_rdb0_slot;
        let mut var_argbvvt_rdb1: f64 = *var_argbvvt_rdb1_slot;
        let mut var_argbvvt_rdb2: f64 = *var_argbvvt_rdb2_slot;
        let mut var_argbvvt_rdb3: f64 = *var_argbvvt_rdb3_slot;
        let mut var_argbvvt_rdb4: f64 = *var_argbvvt_rdb4_slot;
        let mut var_argbvvt_rdb5: f64 = *var_argbvvt_rdb5_slot;
        let mut var_argbvvt_rdb6: f64 = *var_argbvvt_rdb6_slot;
        let mut var_argbvvt_rdb7: f64 = *var_argbvvt_rdb7_slot;
        let mut var_argbvvt_rdn0: f64 = *var_argbvvt_rdn0_slot;
        let mut var_argbvvt_rdn1: f64 = *var_argbvvt_rdn1_slot;
        let mut var_argbvvt_rdn2: f64 = *var_argbvvt_rdn2_slot;
        let mut var_argbvvt_rdn3: f64 = *var_argbvvt_rdn3_slot;
        let mut var_argbvvt_rdn4: f64 = *var_argbvvt_rdn4_slot;
        let mut var_argbvvt_rdn5: f64 = *var_argbvvt_rdn5_slot;
        let mut var_argbvvt_rdn6: f64 = *var_argbvvt_rdn6_slot;
        let mut var_argbvvt_rdn7: f64 = *var_argbvvt_rdn7_slot;
        let mut var_argbvvt_rdn8: f64 = *var_argbvvt_rdn8_slot;
        let mut var_argbvvt_rdn9: f64 = *var_argbvvt_rdn9_slot;
        let mut var_argbvvt_rv: f64 = *var_argbvvt_rv_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard6_db0: f64 = *var_guard6_db0_slot;
        let mut var_guard6_db1: f64 = *var_guard6_db1_slot;
        let mut var_guard6_db2: f64 = *var_guard6_db2_slot;
        let mut var_guard6_db3: f64 = *var_guard6_db3_slot;
        let mut var_guard6_db4: f64 = *var_guard6_db4_slot;
        let mut var_guard6_db5: f64 = *var_guard6_db5_slot;
        let mut var_guard6_db6: f64 = *var_guard6_db6_slot;
        let mut var_guard6_db7: f64 = *var_guard6_db7_slot;
        let mut var_guard6_dn0: f64 = *var_guard6_dn0_slot;
        let mut var_guard6_dn1: f64 = *var_guard6_dn1_slot;
        let mut var_guard6_dn2: f64 = *var_guard6_dn2_slot;
        let mut var_guard6_dn3: f64 = *var_guard6_dn3_slot;
        let mut var_guard6_dn4: f64 = *var_guard6_dn4_slot;
        let mut var_guard6_dn5: f64 = *var_guard6_dn5_slot;
        let mut var_guard6_dn6: f64 = *var_guard6_dn6_slot;
        let mut var_guard6_dn7: f64 = *var_guard6_dn7_slot;
        let mut var_guard6_dn8: f64 = *var_guard6_dn8_slot;
        let mut var_guard6_dn9: f64 = *var_guard6_dn9_slot;
        let mut var_guard6_rdb0: f64 = *var_guard6_rdb0_slot;
        let mut var_guard6_rdb1: f64 = *var_guard6_rdb1_slot;
        let mut var_guard6_rdb2: f64 = *var_guard6_rdb2_slot;
        let mut var_guard6_rdb3: f64 = *var_guard6_rdb3_slot;
        let mut var_guard6_rdb4: f64 = *var_guard6_rdb4_slot;
        let mut var_guard6_rdb5: f64 = *var_guard6_rdb5_slot;
        let mut var_guard6_rdb6: f64 = *var_guard6_rdb6_slot;
        let mut var_guard6_rdb7: f64 = *var_guard6_rdb7_slot;
        let mut var_guard6_rdn0: f64 = *var_guard6_rdn0_slot;
        let mut var_guard6_rdn1: f64 = *var_guard6_rdn1_slot;
        let mut var_guard6_rdn2: f64 = *var_guard6_rdn2_slot;
        let mut var_guard6_rdn3: f64 = *var_guard6_rdn3_slot;
        let mut var_guard6_rdn4: f64 = *var_guard6_rdn4_slot;
        let mut var_guard6_rdn5: f64 = *var_guard6_rdn5_slot;
        let mut var_guard6_rdn6: f64 = *var_guard6_rdn6_slot;
        let mut var_guard6_rdn7: f64 = *var_guard6_rdn7_slot;
        let mut var_guard6_rdn8: f64 = *var_guard6_rdn8_slot;
        let mut var_guard6_rdn9: f64 = *var_guard6_rdn9_slot;
        let mut var_guard6_rv: f64 = *var_guard6_rv_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard7_db0: f64 = *var_guard7_db0_slot;
        let mut var_guard7_db1: f64 = *var_guard7_db1_slot;
        let mut var_guard7_db2: f64 = *var_guard7_db2_slot;
        let mut var_guard7_db3: f64 = *var_guard7_db3_slot;
        let mut var_guard7_db4: f64 = *var_guard7_db4_slot;
        let mut var_guard7_db5: f64 = *var_guard7_db5_slot;
        let mut var_guard7_db6: f64 = *var_guard7_db6_slot;
        let mut var_guard7_db7: f64 = *var_guard7_db7_slot;
        let mut var_guard7_dn0: f64 = *var_guard7_dn0_slot;
        let mut var_guard7_dn1: f64 = *var_guard7_dn1_slot;
        let mut var_guard7_dn2: f64 = *var_guard7_dn2_slot;
        let mut var_guard7_dn3: f64 = *var_guard7_dn3_slot;
        let mut var_guard7_dn4: f64 = *var_guard7_dn4_slot;
        let mut var_guard7_dn5: f64 = *var_guard7_dn5_slot;
        let mut var_guard7_dn6: f64 = *var_guard7_dn6_slot;
        let mut var_guard7_dn7: f64 = *var_guard7_dn7_slot;
        let mut var_guard7_dn8: f64 = *var_guard7_dn8_slot;
        let mut var_guard7_dn9: f64 = *var_guard7_dn9_slot;
        let mut var_guard7_rdb0: f64 = *var_guard7_rdb0_slot;
        let mut var_guard7_rdb1: f64 = *var_guard7_rdb1_slot;
        let mut var_guard7_rdb2: f64 = *var_guard7_rdb2_slot;
        let mut var_guard7_rdb3: f64 = *var_guard7_rdb3_slot;
        let mut var_guard7_rdb4: f64 = *var_guard7_rdb4_slot;
        let mut var_guard7_rdb5: f64 = *var_guard7_rdb5_slot;
        let mut var_guard7_rdb6: f64 = *var_guard7_rdb6_slot;
        let mut var_guard7_rdb7: f64 = *var_guard7_rdb7_slot;
        let mut var_guard7_rdn0: f64 = *var_guard7_rdn0_slot;
        let mut var_guard7_rdn1: f64 = *var_guard7_rdn1_slot;
        let mut var_guard7_rdn2: f64 = *var_guard7_rdn2_slot;
        let mut var_guard7_rdn3: f64 = *var_guard7_rdn3_slot;
        let mut var_guard7_rdn4: f64 = *var_guard7_rdn4_slot;
        let mut var_guard7_rdn5: f64 = *var_guard7_rdn5_slot;
        let mut var_guard7_rdn6: f64 = *var_guard7_rdn6_slot;
        let mut var_guard7_rdn7: f64 = *var_guard7_rdn7_slot;
        let mut var_guard7_rdn8: f64 = *var_guard7_rdn8_slot;
        let mut var_guard7_rdn9: f64 = *var_guard7_rdn9_slot;
        let mut var_guard7_rv: f64 = *var_guard7_rv_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard8_db0: f64 = *var_guard8_db0_slot;
        let mut var_guard8_db1: f64 = *var_guard8_db1_slot;
        let mut var_guard8_db2: f64 = *var_guard8_db2_slot;
        let mut var_guard8_db3: f64 = *var_guard8_db3_slot;
        let mut var_guard8_db4: f64 = *var_guard8_db4_slot;
        let mut var_guard8_db5: f64 = *var_guard8_db5_slot;
        let mut var_guard8_db6: f64 = *var_guard8_db6_slot;
        let mut var_guard8_db7: f64 = *var_guard8_db7_slot;
        let mut var_guard8_dn0: f64 = *var_guard8_dn0_slot;
        let mut var_guard8_dn1: f64 = *var_guard8_dn1_slot;
        let mut var_guard8_dn2: f64 = *var_guard8_dn2_slot;
        let mut var_guard8_dn3: f64 = *var_guard8_dn3_slot;
        let mut var_guard8_dn4: f64 = *var_guard8_dn4_slot;
        let mut var_guard8_dn5: f64 = *var_guard8_dn5_slot;
        let mut var_guard8_dn6: f64 = *var_guard8_dn6_slot;
        let mut var_guard8_dn7: f64 = *var_guard8_dn7_slot;
        let mut var_guard8_dn8: f64 = *var_guard8_dn8_slot;
        let mut var_guard8_dn9: f64 = *var_guard8_dn9_slot;
        let mut var_guard8_rdb0: f64 = *var_guard8_rdb0_slot;
        let mut var_guard8_rdb1: f64 = *var_guard8_rdb1_slot;
        let mut var_guard8_rdb2: f64 = *var_guard8_rdb2_slot;
        let mut var_guard8_rdb3: f64 = *var_guard8_rdb3_slot;
        let mut var_guard8_rdb4: f64 = *var_guard8_rdb4_slot;
        let mut var_guard8_rdb5: f64 = *var_guard8_rdb5_slot;
        let mut var_guard8_rdb6: f64 = *var_guard8_rdb6_slot;
        let mut var_guard8_rdb7: f64 = *var_guard8_rdb7_slot;
        let mut var_guard8_rdn0: f64 = *var_guard8_rdn0_slot;
        let mut var_guard8_rdn1: f64 = *var_guard8_rdn1_slot;
        let mut var_guard8_rdn2: f64 = *var_guard8_rdn2_slot;
        let mut var_guard8_rdn3: f64 = *var_guard8_rdn3_slot;
        let mut var_guard8_rdn4: f64 = *var_guard8_rdn4_slot;
        let mut var_guard8_rdn5: f64 = *var_guard8_rdn5_slot;
        let mut var_guard8_rdn6: f64 = *var_guard8_rdn6_slot;
        let mut var_guard8_rdn7: f64 = *var_guard8_rdn7_slot;
        let mut var_guard8_rdn8: f64 = *var_guard8_rdn8_slot;
        let mut var_guard8_rdn9: f64 = *var_guard8_rdn9_slot;
        let mut var_guard8_rv: f64 = *var_guard8_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_db0: f64 = *var_le_db0_slot;
        let mut var_le_db1: f64 = *var_le_db1_slot;
        let mut var_le_db2: f64 = *var_le_db2_slot;
        let mut var_le_db3: f64 = *var_le_db3_slot;
        let mut var_le_db4: f64 = *var_le_db4_slot;
        let mut var_le_db5: f64 = *var_le_db5_slot;
        let mut var_le_db6: f64 = *var_le_db6_slot;
        let mut var_le_db7: f64 = *var_le_db7_slot;
        let mut var_le_dn0: f64 = *var_le_dn0_slot;
        let mut var_le_dn1: f64 = *var_le_dn1_slot;
        let mut var_le_dn2: f64 = *var_le_dn2_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn5: f64 = *var_le_dn5_slot;
        let mut var_le_dn6: f64 = *var_le_dn6_slot;
        let mut var_le_dn7: f64 = *var_le_dn7_slot;
        let mut var_le_dn8: f64 = *var_le_dn8_slot;
        let mut var_le_dn9: f64 = *var_le_dn9_slot;
        let mut var_le_rdb0: f64 = *var_le_rdb0_slot;
        let mut var_le_rdb1: f64 = *var_le_rdb1_slot;
        let mut var_le_rdb2: f64 = *var_le_rdb2_slot;
        let mut var_le_rdb3: f64 = *var_le_rdb3_slot;
        let mut var_le_rdb4: f64 = *var_le_rdb4_slot;
        let mut var_le_rdb5: f64 = *var_le_rdb5_slot;
        let mut var_le_rdb6: f64 = *var_le_rdb6_slot;
        let mut var_le_rdb7: f64 = *var_le_rdb7_slot;
        let mut var_le_rdn0: f64 = *var_le_rdn0_slot;
        let mut var_le_rdn1: f64 = *var_le_rdn1_slot;
        let mut var_le_rdn2: f64 = *var_le_rdn2_slot;
        let mut var_le_rdn3: f64 = *var_le_rdn3_slot;
        let mut var_le_rdn4: f64 = *var_le_rdn4_slot;
        let mut var_le_rdn5: f64 = *var_le_rdn5_slot;
        let mut var_le_rdn6: f64 = *var_le_rdn6_slot;
        let mut var_le_rdn7: f64 = *var_le_rdn7_slot;
        let mut var_le_rdn8: f64 = *var_le_rdn8_slot;
        let mut var_le_rdn9: f64 = *var_le_rdn9_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;

        let (assign850_e1121, assign850_e1121_d_n0, assign850_e1121_d_n1, assign850_e1121_d_n2, assign850_e1121_d_n3, assign850_e1121_d_n4, assign850_e1121_d_n5, assign850_e1121_d_n6, assign850_e1121_d_n7, assign850_e1121_d_n8, assign850_e1121_d_n9, assign850_e1121_d_b0, assign850_e1121_d_b1, assign850_e1121_d_b2, assign850_e1121_d_b3, assign850_e1121_d_b4, assign850_e1121_d_b5, assign850_e1121_d_b6, assign850_e1121_d_b7,) = {
    if (var_guard5 != 0.0) {
        let assign850_e1109: f64 = (-1.0);
        let assign850_e1111: f64 = (assign850_e1109 * var_vbiei);
        let assign850_e1113: f64 = (assign850_e1111 * p.p4);
        let assign850_e1116: f64 = (p.p3 * var_vt);
        let assign850_e1118: f64 = (assign850_e1116 * var_t0);
        let assign850_e1119: f64 = (assign850_e1113 / assign850_e1118);
        (assign850_e1119, (((((assign850_e1109 * var_vbiei_dn0) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_dn0) * var_t0) + (assign850_e1116 * var_t0_dn0)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_dn1) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_dn1) * var_t0) + (assign850_e1116 * var_t0_dn1)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_dn2) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_dn2) * var_t0) + (assign850_e1116 * var_t0_dn2)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_dn3) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_dn3) * var_t0) + (assign850_e1116 * var_t0_dn3)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_dn4) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_dn4) * var_t0) + (assign850_e1116 * var_t0_dn4)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_dn5) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_dn5) * var_t0) + (assign850_e1116 * var_t0_dn5)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_dn6) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_dn6) * var_t0) + (assign850_e1116 * var_t0_dn6)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_dn7) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_dn7) * var_t0) + (assign850_e1116 * var_t0_dn7)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_dn8) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_dn8) * var_t0) + (assign850_e1116 * var_t0_dn8)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_dn9) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_dn9) * var_t0) + (assign850_e1116 * var_t0_dn9)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_db0) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_db0) * var_t0) + (assign850_e1116 * var_t0_db0)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_db1) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_db1) * var_t0) + (assign850_e1116 * var_t0_db1)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_db2) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_db2) * var_t0) + (assign850_e1116 * var_t0_db2)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_db3) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_db3) * var_t0) + (assign850_e1116 * var_t0_db3)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_db4) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_db4) * var_t0) + (assign850_e1116 * var_t0_db4)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_db5) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_db5) * var_t0) + (assign850_e1116 * var_t0_db5)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_db6) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_db6) * var_t0) + (assign850_e1116 * var_t0_db6)))) / (assign850_e1118 * assign850_e1118)), (((((assign850_e1109 * var_vbiei_db7) * p.p4) * assign850_e1118) - (assign850_e1113 * (((p.p3 * var_vt_db7) * var_t0) + (assign850_e1116 * var_t0_db7)))) / (assign850_e1118 * assign850_e1118)),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    }
};
        var_arg = assign850_e1121;
        var_arg_dn0 = assign850_e1121_d_n0;
        var_arg_dn1 = assign850_e1121_d_n1;
        var_arg_dn2 = assign850_e1121_d_n2;
        var_arg_dn3 = assign850_e1121_d_n3;
        var_arg_dn4 = assign850_e1121_d_n4;
        var_arg_dn5 = assign850_e1121_d_n5;
        var_arg_dn6 = assign850_e1121_d_n6;
        var_arg_dn7 = assign850_e1121_d_n7;
        var_arg_dn8 = assign850_e1121_d_n8;
        var_arg_dn9 = assign850_e1121_d_n9;
        var_arg_db0 = assign850_e1121_d_b0;
        var_arg_db1 = assign850_e1121_d_b1;
        var_arg_db2 = assign850_e1121_d_b2;
        var_arg_db3 = assign850_e1121_d_b3;
        var_arg_db4 = assign850_e1121_d_b4;
        var_arg_db5 = assign850_e1121_d_b5;
        var_arg_db6 = assign850_e1121_d_b6;
        var_arg_db7 = assign850_e1121_d_b7;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdn7 = 0.0;
        var_arg_rdn8 = 0.0;
        var_arg_rdn9 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;
        var_arg_rdb7 = 0.0;

        let assign860_e1124: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard6 = assign860_e1124;
        var_guard6_dn0 = 0.0;
        var_guard6_dn1 = 0.0;
        var_guard6_dn2 = 0.0;
        var_guard6_dn3 = 0.0;
        var_guard6_dn4 = 0.0;
        var_guard6_dn5 = 0.0;
        var_guard6_dn6 = 0.0;
        var_guard6_dn7 = 0.0;
        var_guard6_dn8 = 0.0;
        var_guard6_dn9 = 0.0;
        var_guard6_db0 = 0.0;
        var_guard6_db1 = 0.0;
        var_guard6_db2 = 0.0;
        var_guard6_db3 = 0.0;
        var_guard6_db4 = 0.0;
        var_guard6_db5 = 0.0;
        var_guard6_db6 = 0.0;
        var_guard6_db7 = 0.0;
        var_guard6_rv = 0.0;
        var_guard6_rdn0 = 0.0;
        var_guard6_rdn1 = 0.0;
        var_guard6_rdn2 = 0.0;
        var_guard6_rdn3 = 0.0;
        var_guard6_rdn4 = 0.0;
        var_guard6_rdn5 = 0.0;
        var_guard6_rdn6 = 0.0;
        var_guard6_rdn7 = 0.0;
        var_guard6_rdn8 = 0.0;
        var_guard6_rdn9 = 0.0;
        var_guard6_rdb0 = 0.0;
        var_guard6_rdb1 = 0.0;
        var_guard6_rdb2 = 0.0;
        var_guard6_rdb3 = 0.0;
        var_guard6_rdb4 = 0.0;
        var_guard6_rdb5 = 0.0;
        var_guard6_rdb6 = 0.0;
        var_guard6_rdb7 = 0.0;

        let (assign870_e1134, assign870_e1134_d_n0, assign870_e1134_d_n1, assign870_e1134_d_n2, assign870_e1134_d_n3, assign870_e1134_d_n4, assign870_e1134_d_n5, assign870_e1134_d_n6, assign870_e1134_d_n7, assign870_e1134_d_n8, assign870_e1134_d_n9, assign870_e1134_d_b0, assign870_e1134_d_b1, assign870_e1134_d_b2, assign870_e1134_d_b3, assign870_e1134_d_b4, assign870_e1134_d_b5, assign870_e1134_d_b6, assign870_e1134_d_b7,) = {
    if ((var_guard5 != 0.0) && (var_guard6 != 0.0)) {
        let assign870_e1131: f64 = (var_arg - 80.0);
        let assign870_e1132: f64 = (1.0 + assign870_e1131);
        (assign870_e1132, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign870_e1134;
        var_le_dn0 = assign870_e1134_d_n0;
        var_le_dn1 = assign870_e1134_d_n1;
        var_le_dn2 = assign870_e1134_d_n2;
        var_le_dn3 = assign870_e1134_d_n3;
        var_le_dn4 = assign870_e1134_d_n4;
        var_le_dn5 = assign870_e1134_d_n5;
        var_le_dn6 = assign870_e1134_d_n6;
        var_le_dn7 = assign870_e1134_d_n7;
        var_le_dn8 = assign870_e1134_d_n8;
        var_le_dn9 = assign870_e1134_d_n9;
        var_le_db0 = assign870_e1134_d_b0;
        var_le_db1 = assign870_e1134_d_b1;
        var_le_db2 = assign870_e1134_d_b2;
        var_le_db3 = assign870_e1134_d_b3;
        var_le_db4 = assign870_e1134_d_b4;
        var_le_db5 = assign870_e1134_d_b5;
        var_le_db6 = assign870_e1134_d_b6;
        var_le_db7 = assign870_e1134_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign880_e1140, assign880_e1140_d_n0, assign880_e1140_d_n1, assign880_e1140_d_n2, assign880_e1140_d_n3, assign880_e1140_d_n4, assign880_e1140_d_n5, assign880_e1140_d_n6, assign880_e1140_d_n7, assign880_e1140_d_n8, assign880_e1140_d_n9, assign880_e1140_d_b0, assign880_e1140_d_b1, assign880_e1140_d_b2, assign880_e1140_d_b3, assign880_e1140_d_b4, assign880_e1140_d_b5, assign880_e1140_d_b6, assign880_e1140_d_b7,) = {
    if ((var_guard5 != 0.0) && (var_guard6 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    }
};
        var_arg = assign880_e1140;
        var_arg_dn0 = assign880_e1140_d_n0;
        var_arg_dn1 = assign880_e1140_d_n1;
        var_arg_dn2 = assign880_e1140_d_n2;
        var_arg_dn3 = assign880_e1140_d_n3;
        var_arg_dn4 = assign880_e1140_d_n4;
        var_arg_dn5 = assign880_e1140_d_n5;
        var_arg_dn6 = assign880_e1140_d_n6;
        var_arg_dn7 = assign880_e1140_d_n7;
        var_arg_dn8 = assign880_e1140_d_n8;
        var_arg_dn9 = assign880_e1140_d_n9;
        var_arg_db0 = assign880_e1140_d_b0;
        var_arg_db1 = assign880_e1140_d_b1;
        var_arg_db2 = assign880_e1140_d_b2;
        var_arg_db3 = assign880_e1140_d_b3;
        var_arg_db4 = assign880_e1140_d_b4;
        var_arg_db5 = assign880_e1140_d_b5;
        var_arg_db6 = assign880_e1140_d_b6;
        var_arg_db7 = assign880_e1140_d_b7;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdn7 = 0.0;
        var_arg_rdn8 = 0.0;
        var_arg_rdn9 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;
        var_arg_rdb7 = 0.0;

        let (assign890_e1147, assign890_e1147_d_n0, assign890_e1147_d_n1, assign890_e1147_d_n2, assign890_e1147_d_n3, assign890_e1147_d_n4, assign890_e1147_d_n5, assign890_e1147_d_n6, assign890_e1147_d_n7, assign890_e1147_d_n8, assign890_e1147_d_n9, assign890_e1147_d_b0, assign890_e1147_d_b1, assign890_e1147_d_b2, assign890_e1147_d_b3, assign890_e1147_d_b4, assign890_e1147_d_b5, assign890_e1147_d_b6, assign890_e1147_d_b7,) = {
    if ((var_guard5 != 0.0) && (var_guard6 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign890_e1147;
        var_le_dn0 = assign890_e1147_d_n0;
        var_le_dn1 = assign890_e1147_d_n1;
        var_le_dn2 = assign890_e1147_d_n2;
        var_le_dn3 = assign890_e1147_d_n3;
        var_le_dn4 = assign890_e1147_d_n4;
        var_le_dn5 = assign890_e1147_d_n5;
        var_le_dn6 = assign890_e1147_d_n6;
        var_le_dn7 = assign890_e1147_d_n7;
        var_le_dn8 = assign890_e1147_d_n8;
        var_le_dn9 = assign890_e1147_d_n9;
        var_le_db0 = assign890_e1147_d_b0;
        var_le_db1 = assign890_e1147_d_b1;
        var_le_db2 = assign890_e1147_d_b2;
        var_le_db3 = assign890_e1147_d_b3;
        var_le_db4 = assign890_e1147_d_b4;
        var_le_db5 = assign890_e1147_d_b5;
        var_le_db6 = assign890_e1147_d_b6;
        var_le_db7 = assign890_e1147_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign900_e1154, assign900_e1154_d_n0, assign900_e1154_d_n1, assign900_e1154_d_n2, assign900_e1154_d_n3, assign900_e1154_d_n4, assign900_e1154_d_n5, assign900_e1154_d_n6, assign900_e1154_d_n7, assign900_e1154_d_n8, assign900_e1154_d_n9, assign900_e1154_d_b0, assign900_e1154_d_b1, assign900_e1154_d_b2, assign900_e1154_d_b3, assign900_e1154_d_b4, assign900_e1154_d_b5, assign900_e1154_d_b6, assign900_e1154_d_b7,) = {
    if (var_guard5 != 0.0) {
        let assign900_e1151: f64 = (var_arg).exp();
        let assign900_e1152: f64 = (var_le * assign900_e1151);
        (assign900_e1152, ((var_le_dn0 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn0))), ((var_le_dn1 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn1))), ((var_le_dn2 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn2))), ((var_le_dn3 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn3))), ((var_le_dn4 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn4))), ((var_le_dn5 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn5))), ((var_le_dn6 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn6))), ((var_le_dn7 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn7))), ((var_le_dn8 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn8))), ((var_le_dn9 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_dn9))), ((var_le_db0 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_db0))), ((var_le_db1 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_db1))), ((var_le_db2 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_db2))), ((var_le_db3 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_db3))), ((var_le_db4 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_db4))), ((var_le_db5 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_db5))), ((var_le_db6 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_db6))), ((var_le_db7 * assign900_e1151) + (var_le * (assign900_e1151 * var_arg_db7))),)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign900_e1154;
        var_le_dn0 = assign900_e1154_d_n0;
        var_le_dn1 = assign900_e1154_d_n1;
        var_le_dn2 = assign900_e1154_d_n2;
        var_le_dn3 = assign900_e1154_d_n3;
        var_le_dn4 = assign900_e1154_d_n4;
        var_le_dn5 = assign900_e1154_d_n5;
        var_le_dn6 = assign900_e1154_d_n6;
        var_le_dn7 = assign900_e1154_d_n7;
        var_le_dn8 = assign900_e1154_d_n8;
        var_le_dn9 = assign900_e1154_d_n9;
        var_le_db0 = assign900_e1154_d_b0;
        var_le_db1 = assign900_e1154_d_b1;
        var_le_db2 = assign900_e1154_d_b2;
        var_le_db3 = assign900_e1154_d_b3;
        var_le_db4 = assign900_e1154_d_b4;
        var_le_db5 = assign900_e1154_d_b5;
        var_le_db6 = assign900_e1154_d_b6;
        var_le_db7 = assign900_e1154_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let assign930_e1170: f64 = if var_ise_t > 0.0 { 1.0 } else { 0.0 };
        var_guard7 = assign930_e1170;
        var_guard7_dn0 = 0.0;
        var_guard7_dn1 = 0.0;
        var_guard7_dn2 = 0.0;
        var_guard7_dn3 = 0.0;
        var_guard7_dn4 = 0.0;
        var_guard7_dn5 = 0.0;
        var_guard7_dn6 = 0.0;
        var_guard7_dn7 = 0.0;
        var_guard7_dn8 = 0.0;
        var_guard7_dn9 = 0.0;
        var_guard7_db0 = 0.0;
        var_guard7_db1 = 0.0;
        var_guard7_db2 = 0.0;
        var_guard7_db3 = 0.0;
        var_guard7_db4 = 0.0;
        var_guard7_db5 = 0.0;
        var_guard7_db6 = 0.0;
        var_guard7_db7 = 0.0;
        var_guard7_rv = 0.0;
        var_guard7_rdn0 = 0.0;
        var_guard7_rdn1 = 0.0;
        var_guard7_rdn2 = 0.0;
        var_guard7_rdn3 = 0.0;
        var_guard7_rdn4 = 0.0;
        var_guard7_rdn5 = 0.0;
        var_guard7_rdn6 = 0.0;
        var_guard7_rdn7 = 0.0;
        var_guard7_rdn8 = 0.0;
        var_guard7_rdn9 = 0.0;
        var_guard7_rdb0 = 0.0;
        var_guard7_rdb1 = 0.0;
        var_guard7_rdb2 = 0.0;
        var_guard7_rdb3 = 0.0;
        var_guard7_rdb4 = 0.0;
        var_guard7_rdb5 = 0.0;
        var_guard7_rdb6 = 0.0;
        var_guard7_rdb7 = 0.0;

        let (assign940_e1178, assign940_e1178_d_n0, assign940_e1178_d_n1, assign940_e1178_d_n2, assign940_e1178_d_n3, assign940_e1178_d_n4, assign940_e1178_d_n5, assign940_e1178_d_n6, assign940_e1178_d_n7, assign940_e1178_d_n8, assign940_e1178_d_n9, assign940_e1178_d_b0, assign940_e1178_d_b1, assign940_e1178_d_b2, assign940_e1178_d_b3, assign940_e1178_d_b4, assign940_e1178_d_b5, assign940_e1178_d_b6, assign940_e1178_d_b7,) = {
    if (var_guard7 != 0.0) {
        let assign940_e1175: f64 = (p.p59 * var_vt);
        let assign940_e1176: f64 = (var_vbiei / assign940_e1175);
        (assign940_e1176, (((var_vbiei_dn0 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_dn0))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_dn1 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_dn1))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_dn2 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_dn2))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_dn3 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_dn3))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_dn4 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_dn4))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_dn5 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_dn5))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_dn6 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_dn6))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_dn7 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_dn7))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_dn8 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_dn8))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_dn9 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_dn9))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_db0 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_db0))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_db1 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_db1))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_db2 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_db2))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_db3 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_db3))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_db4 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_db4))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_db5 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_db5))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_db6 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_db6))) / (assign940_e1175 * assign940_e1175)), (((var_vbiei_db7 * assign940_e1175) - (var_vbiei * (p.p59 * var_vt_db7))) / (assign940_e1175 * assign940_e1175)),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    }
};
        var_arg = assign940_e1178;
        var_arg_dn0 = assign940_e1178_d_n0;
        var_arg_dn1 = assign940_e1178_d_n1;
        var_arg_dn2 = assign940_e1178_d_n2;
        var_arg_dn3 = assign940_e1178_d_n3;
        var_arg_dn4 = assign940_e1178_d_n4;
        var_arg_dn5 = assign940_e1178_d_n5;
        var_arg_dn6 = assign940_e1178_d_n6;
        var_arg_dn7 = assign940_e1178_d_n7;
        var_arg_dn8 = assign940_e1178_d_n8;
        var_arg_dn9 = assign940_e1178_d_n9;
        var_arg_db0 = assign940_e1178_d_b0;
        var_arg_db1 = assign940_e1178_d_b1;
        var_arg_db2 = assign940_e1178_d_b2;
        var_arg_db3 = assign940_e1178_d_b3;
        var_arg_db4 = assign940_e1178_d_b4;
        var_arg_db5 = assign940_e1178_d_b5;
        var_arg_db6 = assign940_e1178_d_b6;
        var_arg_db7 = assign940_e1178_d_b7;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdn7 = 0.0;
        var_arg_rdn8 = 0.0;
        var_arg_rdn9 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;
        var_arg_rdb7 = 0.0;

        let (assign950_e1189, assign950_e1189_d_n0, assign950_e1189_d_n1, assign950_e1189_d_n2, assign950_e1189_d_n3, assign950_e1189_d_n4, assign950_e1189_d_n5, assign950_e1189_d_n6, assign950_e1189_d_n7, assign950_e1189_d_n8, assign950_e1189_d_n9, assign950_e1189_d_b0, assign950_e1189_d_b1, assign950_e1189_d_b2, assign950_e1189_d_b3, assign950_e1189_d_b4, assign950_e1189_d_b5, assign950_e1189_d_b6, assign950_e1189_d_b7,) = {
    if (var_guard7 != 0.0) {
        let assign950_e1181: f64 = (-var_vbiei);
        let assign950_e1183: f64 = (assign950_e1181 - var_bvr_t);
        let assign950_e1186: f64 = (p.p57 * var_vt);
        let assign950_e1187: f64 = (assign950_e1183 / assign950_e1186);
        (assign950_e1187, (((((-var_vbiei_dn0) - var_bvr_t_dn0) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_dn0))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_dn1) - var_bvr_t_dn1) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_dn1))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_dn2) - var_bvr_t_dn2) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_dn2))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_dn3) - var_bvr_t_dn3) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_dn3))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_dn4) - var_bvr_t_dn4) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_dn4))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_dn5) - var_bvr_t_dn5) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_dn5))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_dn6) - var_bvr_t_dn6) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_dn6))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_dn7) - var_bvr_t_dn7) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_dn7))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_dn8) - var_bvr_t_dn8) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_dn8))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_dn9) - var_bvr_t_dn9) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_dn9))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_db0) - var_bvr_t_db0) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_db0))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_db1) - var_bvr_t_db1) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_db1))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_db2) - var_bvr_t_db2) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_db2))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_db3) - var_bvr_t_db3) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_db3))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_db4) - var_bvr_t_db4) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_db4))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_db5) - var_bvr_t_db5) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_db5))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_db6) - var_bvr_t_db6) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_db6))) / (assign950_e1186 * assign950_e1186)), (((((-var_vbiei_db7) - var_bvr_t_db7) * assign950_e1186) - (assign950_e1183 * (p.p57 * var_vt_db7))) / (assign950_e1186 * assign950_e1186)),)
    } else {
        (var_argbv, var_argbv_dn0, var_argbv_dn1, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6, var_argbv_dn7, var_argbv_dn8, var_argbv_dn9, var_argbv_db0, var_argbv_db1, var_argbv_db2, var_argbv_db3, var_argbv_db4, var_argbv_db5, var_argbv_db6, var_argbv_db7,)
    }
};
        var_argbv = assign950_e1189;
        var_argbv_dn0 = assign950_e1189_d_n0;
        var_argbv_dn1 = assign950_e1189_d_n1;
        var_argbv_dn2 = assign950_e1189_d_n2;
        var_argbv_dn3 = assign950_e1189_d_n3;
        var_argbv_dn4 = assign950_e1189_d_n4;
        var_argbv_dn5 = assign950_e1189_d_n5;
        var_argbv_dn6 = assign950_e1189_d_n6;
        var_argbv_dn7 = assign950_e1189_d_n7;
        var_argbv_dn8 = assign950_e1189_d_n8;
        var_argbv_dn9 = assign950_e1189_d_n9;
        var_argbv_db0 = assign950_e1189_d_b0;
        var_argbv_db1 = assign950_e1189_d_b1;
        var_argbv_db2 = assign950_e1189_d_b2;
        var_argbv_db3 = assign950_e1189_d_b3;
        var_argbv_db4 = assign950_e1189_d_b4;
        var_argbv_db5 = assign950_e1189_d_b5;
        var_argbv_db6 = assign950_e1189_d_b6;
        var_argbv_db7 = assign950_e1189_d_b7;
        var_argbv_rv = 0.0;
        var_argbv_rdn0 = 0.0;
        var_argbv_rdn1 = 0.0;
        var_argbv_rdn2 = 0.0;
        var_argbv_rdn3 = 0.0;
        var_argbv_rdn4 = 0.0;
        var_argbv_rdn5 = 0.0;
        var_argbv_rdn6 = 0.0;
        var_argbv_rdn7 = 0.0;
        var_argbv_rdn8 = 0.0;
        var_argbv_rdn9 = 0.0;
        var_argbv_rdb0 = 0.0;
        var_argbv_rdb1 = 0.0;
        var_argbv_rdb2 = 0.0;
        var_argbv_rdb3 = 0.0;
        var_argbv_rdb4 = 0.0;
        var_argbv_rdb5 = 0.0;
        var_argbv_rdb6 = 0.0;
        var_argbv_rdb7 = 0.0;

        let (assign960_e1198, assign960_e1198_d_n0, assign960_e1198_d_n1, assign960_e1198_d_n2, assign960_e1198_d_n3, assign960_e1198_d_n4, assign960_e1198_d_n5, assign960_e1198_d_n6, assign960_e1198_d_n7, assign960_e1198_d_n8, assign960_e1198_d_n9, assign960_e1198_d_b0, assign960_e1198_d_b1, assign960_e1198_d_b2, assign960_e1198_d_b3, assign960_e1198_d_b4, assign960_e1198_d_b5, assign960_e1198_d_b6, assign960_e1198_d_b7,) = {
    if (var_guard7 != 0.0) {
        let assign960_e1192: f64 = (-var_bvr_t);
        let assign960_e1195: f64 = (p.p57 * var_vt);
        let assign960_e1196: f64 = (assign960_e1192 / assign960_e1195);
        (assign960_e1196, ((((-var_bvr_t_dn0) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_dn0))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_dn1) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_dn1))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_dn2) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_dn2))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_dn3) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_dn3))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_dn4) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_dn4))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_dn5) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_dn5))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_dn6) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_dn6))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_dn7) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_dn7))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_dn8) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_dn8))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_dn9) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_dn9))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_db0) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_db0))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_db1) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_db1))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_db2) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_db2))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_db3) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_db3))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_db4) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_db4))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_db5) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_db5))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_db6) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_db6))) / (assign960_e1195 * assign960_e1195)), ((((-var_bvr_t_db7) * assign960_e1195) - (assign960_e1192 * (p.p57 * var_vt_db7))) / (assign960_e1195 * assign960_e1195)),)
    } else {
        (var_argbvvt, var_argbvvt_dn0, var_argbvvt_dn1, var_argbvvt_dn2, var_argbvvt_dn3, var_argbvvt_dn4, var_argbvvt_dn5, var_argbvvt_dn6, var_argbvvt_dn7, var_argbvvt_dn8, var_argbvvt_dn9, var_argbvvt_db0, var_argbvvt_db1, var_argbvvt_db2, var_argbvvt_db3, var_argbvvt_db4, var_argbvvt_db5, var_argbvvt_db6, var_argbvvt_db7,)
    }
};
        var_argbvvt = assign960_e1198;
        var_argbvvt_dn0 = assign960_e1198_d_n0;
        var_argbvvt_dn1 = assign960_e1198_d_n1;
        var_argbvvt_dn2 = assign960_e1198_d_n2;
        var_argbvvt_dn3 = assign960_e1198_d_n3;
        var_argbvvt_dn4 = assign960_e1198_d_n4;
        var_argbvvt_dn5 = assign960_e1198_d_n5;
        var_argbvvt_dn6 = assign960_e1198_d_n6;
        var_argbvvt_dn7 = assign960_e1198_d_n7;
        var_argbvvt_dn8 = assign960_e1198_d_n8;
        var_argbvvt_dn9 = assign960_e1198_d_n9;
        var_argbvvt_db0 = assign960_e1198_d_b0;
        var_argbvvt_db1 = assign960_e1198_d_b1;
        var_argbvvt_db2 = assign960_e1198_d_b2;
        var_argbvvt_db3 = assign960_e1198_d_b3;
        var_argbvvt_db4 = assign960_e1198_d_b4;
        var_argbvvt_db5 = assign960_e1198_d_b5;
        var_argbvvt_db6 = assign960_e1198_d_b6;
        var_argbvvt_db7 = assign960_e1198_d_b7;
        var_argbvvt_rv = 0.0;
        var_argbvvt_rdn0 = 0.0;
        var_argbvvt_rdn1 = 0.0;
        var_argbvvt_rdn2 = 0.0;
        var_argbvvt_rdn3 = 0.0;
        var_argbvvt_rdn4 = 0.0;
        var_argbvvt_rdn5 = 0.0;
        var_argbvvt_rdn6 = 0.0;
        var_argbvvt_rdn7 = 0.0;
        var_argbvvt_rdn8 = 0.0;
        var_argbvvt_rdn9 = 0.0;
        var_argbvvt_rdb0 = 0.0;
        var_argbvvt_rdb1 = 0.0;
        var_argbvvt_rdb2 = 0.0;
        var_argbvvt_rdb3 = 0.0;
        var_argbvvt_rdb4 = 0.0;
        var_argbvvt_rdb5 = 0.0;
        var_argbvvt_rdb6 = 0.0;
        var_argbvvt_rdb7 = 0.0;

        let assign970_e1201: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard8 = assign970_e1201;
        var_guard8_dn0 = 0.0;
        var_guard8_dn1 = 0.0;
        var_guard8_dn2 = 0.0;
        var_guard8_dn3 = 0.0;
        var_guard8_dn4 = 0.0;
        var_guard8_dn5 = 0.0;
        var_guard8_dn6 = 0.0;
        var_guard8_dn7 = 0.0;
        var_guard8_dn8 = 0.0;
        var_guard8_dn9 = 0.0;
        var_guard8_db0 = 0.0;
        var_guard8_db1 = 0.0;
        var_guard8_db2 = 0.0;
        var_guard8_db3 = 0.0;
        var_guard8_db4 = 0.0;
        var_guard8_db5 = 0.0;
        var_guard8_db6 = 0.0;
        var_guard8_db7 = 0.0;
        var_guard8_rv = 0.0;
        var_guard8_rdn0 = 0.0;
        var_guard8_rdn1 = 0.0;
        var_guard8_rdn2 = 0.0;
        var_guard8_rdn3 = 0.0;
        var_guard8_rdn4 = 0.0;
        var_guard8_rdn5 = 0.0;
        var_guard8_rdn6 = 0.0;
        var_guard8_rdn7 = 0.0;
        var_guard8_rdn8 = 0.0;
        var_guard8_rdn9 = 0.0;
        var_guard8_rdb0 = 0.0;
        var_guard8_rdb1 = 0.0;
        var_guard8_rdb2 = 0.0;
        var_guard8_rdb3 = 0.0;
        var_guard8_rdb4 = 0.0;
        var_guard8_rdb5 = 0.0;
        var_guard8_rdb6 = 0.0;
        var_guard8_rdb7 = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_db0_slot = var_arg_db0;
        *var_arg_db1_slot = var_arg_db1;
        *var_arg_db2_slot = var_arg_db2;
        *var_arg_db3_slot = var_arg_db3;
        *var_arg_db4_slot = var_arg_db4;
        *var_arg_db5_slot = var_arg_db5;
        *var_arg_db6_slot = var_arg_db6;
        *var_arg_db7_slot = var_arg_db7;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn1_slot = var_arg_dn1;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_arg_rdb0_slot = var_arg_rdb0;
        *var_arg_rdb1_slot = var_arg_rdb1;
        *var_arg_rdb2_slot = var_arg_rdb2;
        *var_arg_rdb3_slot = var_arg_rdb3;
        *var_arg_rdb4_slot = var_arg_rdb4;
        *var_arg_rdb5_slot = var_arg_rdb5;
        *var_arg_rdb6_slot = var_arg_rdb6;
        *var_arg_rdb7_slot = var_arg_rdb7;
        *var_arg_rdn0_slot = var_arg_rdn0;
        *var_arg_rdn1_slot = var_arg_rdn1;
        *var_arg_rdn2_slot = var_arg_rdn2;
        *var_arg_rdn3_slot = var_arg_rdn3;
        *var_arg_rdn4_slot = var_arg_rdn4;
        *var_arg_rdn5_slot = var_arg_rdn5;
        *var_arg_rdn6_slot = var_arg_rdn6;
        *var_arg_rdn7_slot = var_arg_rdn7;
        *var_arg_rdn8_slot = var_arg_rdn8;
        *var_arg_rdn9_slot = var_arg_rdn9;
        *var_arg_rv_slot = var_arg_rv;
        *var_argbv_slot = var_argbv;
        *var_argbv_db0_slot = var_argbv_db0;
        *var_argbv_db1_slot = var_argbv_db1;
        *var_argbv_db2_slot = var_argbv_db2;
        *var_argbv_db3_slot = var_argbv_db3;
        *var_argbv_db4_slot = var_argbv_db4;
        *var_argbv_db5_slot = var_argbv_db5;
        *var_argbv_db6_slot = var_argbv_db6;
        *var_argbv_db7_slot = var_argbv_db7;
        *var_argbv_dn0_slot = var_argbv_dn0;
        *var_argbv_dn1_slot = var_argbv_dn1;
        *var_argbv_dn2_slot = var_argbv_dn2;
        *var_argbv_dn3_slot = var_argbv_dn3;
        *var_argbv_dn4_slot = var_argbv_dn4;
        *var_argbv_dn5_slot = var_argbv_dn5;
        *var_argbv_dn6_slot = var_argbv_dn6;
        *var_argbv_dn7_slot = var_argbv_dn7;
        *var_argbv_dn8_slot = var_argbv_dn8;
        *var_argbv_dn9_slot = var_argbv_dn9;
        *var_argbv_rdb0_slot = var_argbv_rdb0;
        *var_argbv_rdb1_slot = var_argbv_rdb1;
        *var_argbv_rdb2_slot = var_argbv_rdb2;
        *var_argbv_rdb3_slot = var_argbv_rdb3;
        *var_argbv_rdb4_slot = var_argbv_rdb4;
        *var_argbv_rdb5_slot = var_argbv_rdb5;
        *var_argbv_rdb6_slot = var_argbv_rdb6;
        *var_argbv_rdb7_slot = var_argbv_rdb7;
        *var_argbv_rdn0_slot = var_argbv_rdn0;
        *var_argbv_rdn1_slot = var_argbv_rdn1;
        *var_argbv_rdn2_slot = var_argbv_rdn2;
        *var_argbv_rdn3_slot = var_argbv_rdn3;
        *var_argbv_rdn4_slot = var_argbv_rdn4;
        *var_argbv_rdn5_slot = var_argbv_rdn5;
        *var_argbv_rdn6_slot = var_argbv_rdn6;
        *var_argbv_rdn7_slot = var_argbv_rdn7;
        *var_argbv_rdn8_slot = var_argbv_rdn8;
        *var_argbv_rdn9_slot = var_argbv_rdn9;
        *var_argbv_rv_slot = var_argbv_rv;
        *var_argbvvt_slot = var_argbvvt;
        *var_argbvvt_db0_slot = var_argbvvt_db0;
        *var_argbvvt_db1_slot = var_argbvvt_db1;
        *var_argbvvt_db2_slot = var_argbvvt_db2;
        *var_argbvvt_db3_slot = var_argbvvt_db3;
        *var_argbvvt_db4_slot = var_argbvvt_db4;
        *var_argbvvt_db5_slot = var_argbvvt_db5;
        *var_argbvvt_db6_slot = var_argbvvt_db6;
        *var_argbvvt_db7_slot = var_argbvvt_db7;
        *var_argbvvt_dn0_slot = var_argbvvt_dn0;
        *var_argbvvt_dn1_slot = var_argbvvt_dn1;
        *var_argbvvt_dn2_slot = var_argbvvt_dn2;
        *var_argbvvt_dn3_slot = var_argbvvt_dn3;
        *var_argbvvt_dn4_slot = var_argbvvt_dn4;
        *var_argbvvt_dn5_slot = var_argbvvt_dn5;
        *var_argbvvt_dn6_slot = var_argbvvt_dn6;
        *var_argbvvt_dn7_slot = var_argbvvt_dn7;
        *var_argbvvt_dn8_slot = var_argbvvt_dn8;
        *var_argbvvt_dn9_slot = var_argbvvt_dn9;
        *var_argbvvt_rdb0_slot = var_argbvvt_rdb0;
        *var_argbvvt_rdb1_slot = var_argbvvt_rdb1;
        *var_argbvvt_rdb2_slot = var_argbvvt_rdb2;
        *var_argbvvt_rdb3_slot = var_argbvvt_rdb3;
        *var_argbvvt_rdb4_slot = var_argbvvt_rdb4;
        *var_argbvvt_rdb5_slot = var_argbvvt_rdb5;
        *var_argbvvt_rdb6_slot = var_argbvvt_rdb6;
        *var_argbvvt_rdb7_slot = var_argbvvt_rdb7;
        *var_argbvvt_rdn0_slot = var_argbvvt_rdn0;
        *var_argbvvt_rdn1_slot = var_argbvvt_rdn1;
        *var_argbvvt_rdn2_slot = var_argbvvt_rdn2;
        *var_argbvvt_rdn3_slot = var_argbvvt_rdn3;
        *var_argbvvt_rdn4_slot = var_argbvvt_rdn4;
        *var_argbvvt_rdn5_slot = var_argbvvt_rdn5;
        *var_argbvvt_rdn6_slot = var_argbvvt_rdn6;
        *var_argbvvt_rdn7_slot = var_argbvvt_rdn7;
        *var_argbvvt_rdn8_slot = var_argbvvt_rdn8;
        *var_argbvvt_rdn9_slot = var_argbvvt_rdn9;
        *var_argbvvt_rv_slot = var_argbvvt_rv;
        *var_guard6_slot = var_guard6;
        *var_guard6_db0_slot = var_guard6_db0;
        *var_guard6_db1_slot = var_guard6_db1;
        *var_guard6_db2_slot = var_guard6_db2;
        *var_guard6_db3_slot = var_guard6_db3;
        *var_guard6_db4_slot = var_guard6_db4;
        *var_guard6_db5_slot = var_guard6_db5;
        *var_guard6_db6_slot = var_guard6_db6;
        *var_guard6_db7_slot = var_guard6_db7;
        *var_guard6_dn0_slot = var_guard6_dn0;
        *var_guard6_dn1_slot = var_guard6_dn1;
        *var_guard6_dn2_slot = var_guard6_dn2;
        *var_guard6_dn3_slot = var_guard6_dn3;
        *var_guard6_dn4_slot = var_guard6_dn4;
        *var_guard6_dn5_slot = var_guard6_dn5;
        *var_guard6_dn6_slot = var_guard6_dn6;
        *var_guard6_dn7_slot = var_guard6_dn7;
        *var_guard6_dn8_slot = var_guard6_dn8;
        *var_guard6_dn9_slot = var_guard6_dn9;
        *var_guard6_rdb0_slot = var_guard6_rdb0;
        *var_guard6_rdb1_slot = var_guard6_rdb1;
        *var_guard6_rdb2_slot = var_guard6_rdb2;
        *var_guard6_rdb3_slot = var_guard6_rdb3;
        *var_guard6_rdb4_slot = var_guard6_rdb4;
        *var_guard6_rdb5_slot = var_guard6_rdb5;
        *var_guard6_rdb6_slot = var_guard6_rdb6;
        *var_guard6_rdb7_slot = var_guard6_rdb7;
        *var_guard6_rdn0_slot = var_guard6_rdn0;
        *var_guard6_rdn1_slot = var_guard6_rdn1;
        *var_guard6_rdn2_slot = var_guard6_rdn2;
        *var_guard6_rdn3_slot = var_guard6_rdn3;
        *var_guard6_rdn4_slot = var_guard6_rdn4;
        *var_guard6_rdn5_slot = var_guard6_rdn5;
        *var_guard6_rdn6_slot = var_guard6_rdn6;
        *var_guard6_rdn7_slot = var_guard6_rdn7;
        *var_guard6_rdn8_slot = var_guard6_rdn8;
        *var_guard6_rdn9_slot = var_guard6_rdn9;
        *var_guard6_rv_slot = var_guard6_rv;
        *var_guard7_slot = var_guard7;
        *var_guard7_db0_slot = var_guard7_db0;
        *var_guard7_db1_slot = var_guard7_db1;
        *var_guard7_db2_slot = var_guard7_db2;
        *var_guard7_db3_slot = var_guard7_db3;
        *var_guard7_db4_slot = var_guard7_db4;
        *var_guard7_db5_slot = var_guard7_db5;
        *var_guard7_db6_slot = var_guard7_db6;
        *var_guard7_db7_slot = var_guard7_db7;
        *var_guard7_dn0_slot = var_guard7_dn0;
        *var_guard7_dn1_slot = var_guard7_dn1;
        *var_guard7_dn2_slot = var_guard7_dn2;
        *var_guard7_dn3_slot = var_guard7_dn3;
        *var_guard7_dn4_slot = var_guard7_dn4;
        *var_guard7_dn5_slot = var_guard7_dn5;
        *var_guard7_dn6_slot = var_guard7_dn6;
        *var_guard7_dn7_slot = var_guard7_dn7;
        *var_guard7_dn8_slot = var_guard7_dn8;
        *var_guard7_dn9_slot = var_guard7_dn9;
        *var_guard7_rdb0_slot = var_guard7_rdb0;
        *var_guard7_rdb1_slot = var_guard7_rdb1;
        *var_guard7_rdb2_slot = var_guard7_rdb2;
        *var_guard7_rdb3_slot = var_guard7_rdb3;
        *var_guard7_rdb4_slot = var_guard7_rdb4;
        *var_guard7_rdb5_slot = var_guard7_rdb5;
        *var_guard7_rdb6_slot = var_guard7_rdb6;
        *var_guard7_rdb7_slot = var_guard7_rdb7;
        *var_guard7_rdn0_slot = var_guard7_rdn0;
        *var_guard7_rdn1_slot = var_guard7_rdn1;
        *var_guard7_rdn2_slot = var_guard7_rdn2;
        *var_guard7_rdn3_slot = var_guard7_rdn3;
        *var_guard7_rdn4_slot = var_guard7_rdn4;
        *var_guard7_rdn5_slot = var_guard7_rdn5;
        *var_guard7_rdn6_slot = var_guard7_rdn6;
        *var_guard7_rdn7_slot = var_guard7_rdn7;
        *var_guard7_rdn8_slot = var_guard7_rdn8;
        *var_guard7_rdn9_slot = var_guard7_rdn9;
        *var_guard7_rv_slot = var_guard7_rv;
        *var_guard8_slot = var_guard8;
        *var_guard8_db0_slot = var_guard8_db0;
        *var_guard8_db1_slot = var_guard8_db1;
        *var_guard8_db2_slot = var_guard8_db2;
        *var_guard8_db3_slot = var_guard8_db3;
        *var_guard8_db4_slot = var_guard8_db4;
        *var_guard8_db5_slot = var_guard8_db5;
        *var_guard8_db6_slot = var_guard8_db6;
        *var_guard8_db7_slot = var_guard8_db7;
        *var_guard8_dn0_slot = var_guard8_dn0;
        *var_guard8_dn1_slot = var_guard8_dn1;
        *var_guard8_dn2_slot = var_guard8_dn2;
        *var_guard8_dn3_slot = var_guard8_dn3;
        *var_guard8_dn4_slot = var_guard8_dn4;
        *var_guard8_dn5_slot = var_guard8_dn5;
        *var_guard8_dn6_slot = var_guard8_dn6;
        *var_guard8_dn7_slot = var_guard8_dn7;
        *var_guard8_dn8_slot = var_guard8_dn8;
        *var_guard8_dn9_slot = var_guard8_dn9;
        *var_guard8_rdb0_slot = var_guard8_rdb0;
        *var_guard8_rdb1_slot = var_guard8_rdb1;
        *var_guard8_rdb2_slot = var_guard8_rdb2;
        *var_guard8_rdb3_slot = var_guard8_rdb3;
        *var_guard8_rdb4_slot = var_guard8_rdb4;
        *var_guard8_rdb5_slot = var_guard8_rdb5;
        *var_guard8_rdb6_slot = var_guard8_rdb6;
        *var_guard8_rdb7_slot = var_guard8_rdb7;
        *var_guard8_rdn0_slot = var_guard8_rdn0;
        *var_guard8_rdn1_slot = var_guard8_rdn1;
        *var_guard8_rdn2_slot = var_guard8_rdn2;
        *var_guard8_rdn3_slot = var_guard8_rdn3;
        *var_guard8_rdn4_slot = var_guard8_rdn4;
        *var_guard8_rdn5_slot = var_guard8_rdn5;
        *var_guard8_rdn6_slot = var_guard8_rdn6;
        *var_guard8_rdn7_slot = var_guard8_rdn7;
        *var_guard8_rdn8_slot = var_guard8_rdn8;
        *var_guard8_rdn9_slot = var_guard8_rdn9;
        *var_guard8_rv_slot = var_guard8_rv;
        *var_le_slot = var_le;
        *var_le_db0_slot = var_le_db0;
        *var_le_db1_slot = var_le_db1;
        *var_le_db2_slot = var_le_db2;
        *var_le_db3_slot = var_le_db3;
        *var_le_db4_slot = var_le_db4;
        *var_le_db5_slot = var_le_db5;
        *var_le_db6_slot = var_le_db6;
        *var_le_db7_slot = var_le_db7;
        *var_le_dn0_slot = var_le_dn0;
        *var_le_dn1_slot = var_le_dn1;
        *var_le_dn2_slot = var_le_dn2;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn5_slot = var_le_dn5;
        *var_le_dn6_slot = var_le_dn6;
        *var_le_dn7_slot = var_le_dn7;
        *var_le_dn8_slot = var_le_dn8;
        *var_le_dn9_slot = var_le_dn9;
        *var_le_rdb0_slot = var_le_rdb0;
        *var_le_rdb1_slot = var_le_rdb1;
        *var_le_rdb2_slot = var_le_rdb2;
        *var_le_rdb3_slot = var_le_rdb3;
        *var_le_rdb4_slot = var_le_rdb4;
        *var_le_rdb5_slot = var_le_rdb5;
        *var_le_rdb6_slot = var_le_rdb6;
        *var_le_rdb7_slot = var_le_rdb7;
        *var_le_rdn0_slot = var_le_rdn0;
        *var_le_rdn1_slot = var_le_rdn1;
        *var_le_rdn2_slot = var_le_rdn2;
        *var_le_rdn3_slot = var_le_rdn3;
        *var_le_rdn4_slot = var_le_rdn4;
        *var_le_rdn5_slot = var_le_rdn5;
        *var_le_rdn6_slot = var_le_rdn6;
        *var_le_rdn7_slot = var_le_rdn7;
        *var_le_rdn8_slot = var_le_rdn8;
        *var_le_rdn9_slot = var_le_rdn9;
        *var_le_rv_slot = var_le_rv;
    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        var_bvr_t: f64,
        var_bvr_t_db0: f64,
        var_bvr_t_db1: f64,
        var_bvr_t_db2: f64,
        var_bvr_t_db3: f64,
        var_bvr_t_db4: f64,
        var_bvr_t_db5: f64,
        var_bvr_t_db6: f64,
        var_bvr_t_db7: f64,
        var_bvr_t_dn0: f64,
        var_bvr_t_dn1: f64,
        var_bvr_t_dn2: f64,
        var_bvr_t_dn3: f64,
        var_bvr_t_dn4: f64,
        var_bvr_t_dn5: f64,
        var_bvr_t_dn6: f64,
        var_bvr_t_dn7: f64,
        var_bvr_t_dn8: f64,
        var_bvr_t_dn9: f64,
        var_guard7: f64,
        var_guard8: f64,
        var_is_t: f64,
        var_vbici: f64,
        var_vbici_db0: f64,
        var_vbici_db1: f64,
        var_vbici_db2: f64,
        var_vbici_db3: f64,
        var_vbici_db4: f64,
        var_vbici_db5: f64,
        var_vbici_db6: f64,
        var_vbici_db7: f64,
        var_vbici_dn0: f64,
        var_vbici_dn1: f64,
        var_vbici_dn2: f64,
        var_vbici_dn3: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbici_dn6: f64,
        var_vbici_dn7: f64,
        var_vbici_dn8: f64,
        var_vbici_dn9: f64,
        var_vt: f64,
        var_vt_db0: f64,
        var_vt_db1: f64,
        var_vt_db2: f64,
        var_vt_db3: f64,
        var_vt_db4: f64,
        var_vt_db5: f64,
        var_vt_db6: f64,
        var_vt_db7: f64,
        var_vt_dn0: f64,
        var_vt_dn1: f64,
        var_vt_dn2: f64,
        var_vt_dn3: f64,
        var_vt_dn4: f64,
        var_vt_dn5: f64,
        var_vt_dn6: f64,
        var_vt_dn7: f64,
        var_vt_dn8: f64,
        var_vt_dn9: f64,
        var_arg_slot: &mut f64,
        var_arg_db0_slot: &mut f64,
        var_arg_db1_slot: &mut f64,
        var_arg_db2_slot: &mut f64,
        var_arg_db3_slot: &mut f64,
        var_arg_db4_slot: &mut f64,
        var_arg_db5_slot: &mut f64,
        var_arg_db6_slot: &mut f64,
        var_arg_db7_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn1_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_arg_rdb0_slot: &mut f64,
        var_arg_rdb1_slot: &mut f64,
        var_arg_rdb2_slot: &mut f64,
        var_arg_rdb3_slot: &mut f64,
        var_arg_rdb4_slot: &mut f64,
        var_arg_rdb5_slot: &mut f64,
        var_arg_rdb6_slot: &mut f64,
        var_arg_rdb7_slot: &mut f64,
        var_arg_rdn0_slot: &mut f64,
        var_arg_rdn1_slot: &mut f64,
        var_arg_rdn2_slot: &mut f64,
        var_arg_rdn3_slot: &mut f64,
        var_arg_rdn4_slot: &mut f64,
        var_arg_rdn5_slot: &mut f64,
        var_arg_rdn6_slot: &mut f64,
        var_arg_rdn7_slot: &mut f64,
        var_arg_rdn8_slot: &mut f64,
        var_arg_rdn9_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_argbv_slot: &mut f64,
        var_argbv_db0_slot: &mut f64,
        var_argbv_db1_slot: &mut f64,
        var_argbv_db2_slot: &mut f64,
        var_argbv_db3_slot: &mut f64,
        var_argbv_db4_slot: &mut f64,
        var_argbv_db5_slot: &mut f64,
        var_argbv_db6_slot: &mut f64,
        var_argbv_db7_slot: &mut f64,
        var_argbv_dn0_slot: &mut f64,
        var_argbv_dn1_slot: &mut f64,
        var_argbv_dn2_slot: &mut f64,
        var_argbv_dn3_slot: &mut f64,
        var_argbv_dn4_slot: &mut f64,
        var_argbv_dn5_slot: &mut f64,
        var_argbv_dn6_slot: &mut f64,
        var_argbv_dn7_slot: &mut f64,
        var_argbv_dn8_slot: &mut f64,
        var_argbv_dn9_slot: &mut f64,
        var_argbv_rdb0_slot: &mut f64,
        var_argbv_rdb1_slot: &mut f64,
        var_argbv_rdb2_slot: &mut f64,
        var_argbv_rdb3_slot: &mut f64,
        var_argbv_rdb4_slot: &mut f64,
        var_argbv_rdb5_slot: &mut f64,
        var_argbv_rdb6_slot: &mut f64,
        var_argbv_rdb7_slot: &mut f64,
        var_argbv_rdn0_slot: &mut f64,
        var_argbv_rdn1_slot: &mut f64,
        var_argbv_rdn2_slot: &mut f64,
        var_argbv_rdn3_slot: &mut f64,
        var_argbv_rdn4_slot: &mut f64,
        var_argbv_rdn5_slot: &mut f64,
        var_argbv_rdn6_slot: &mut f64,
        var_argbv_rdn7_slot: &mut f64,
        var_argbv_rdn8_slot: &mut f64,
        var_argbv_rdn9_slot: &mut f64,
        var_argbv_rv_slot: &mut f64,
        var_argbvvt_slot: &mut f64,
        var_argbvvt_db0_slot: &mut f64,
        var_argbvvt_db1_slot: &mut f64,
        var_argbvvt_db2_slot: &mut f64,
        var_argbvvt_db3_slot: &mut f64,
        var_argbvvt_db4_slot: &mut f64,
        var_argbvvt_db5_slot: &mut f64,
        var_argbvvt_db6_slot: &mut f64,
        var_argbvvt_db7_slot: &mut f64,
        var_argbvvt_dn0_slot: &mut f64,
        var_argbvvt_dn1_slot: &mut f64,
        var_argbvvt_dn2_slot: &mut f64,
        var_argbvvt_dn3_slot: &mut f64,
        var_argbvvt_dn4_slot: &mut f64,
        var_argbvvt_dn5_slot: &mut f64,
        var_argbvvt_dn6_slot: &mut f64,
        var_argbvvt_dn7_slot: &mut f64,
        var_argbvvt_dn8_slot: &mut f64,
        var_argbvvt_dn9_slot: &mut f64,
        var_argbvvt_rdb0_slot: &mut f64,
        var_argbvvt_rdb1_slot: &mut f64,
        var_argbvvt_rdb2_slot: &mut f64,
        var_argbvvt_rdb3_slot: &mut f64,
        var_argbvvt_rdb4_slot: &mut f64,
        var_argbvvt_rdb5_slot: &mut f64,
        var_argbvvt_rdb6_slot: &mut f64,
        var_argbvvt_rdb7_slot: &mut f64,
        var_argbvvt_rdn0_slot: &mut f64,
        var_argbvvt_rdn1_slot: &mut f64,
        var_argbvvt_rdn2_slot: &mut f64,
        var_argbvvt_rdn3_slot: &mut f64,
        var_argbvvt_rdn4_slot: &mut f64,
        var_argbvvt_rdn5_slot: &mut f64,
        var_argbvvt_rdn6_slot: &mut f64,
        var_argbvvt_rdn7_slot: &mut f64,
        var_argbvvt_rdn8_slot: &mut f64,
        var_argbvvt_rdn9_slot: &mut f64,
        var_argbvvt_rv_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_guard9_db0_slot: &mut f64,
        var_guard9_db1_slot: &mut f64,
        var_guard9_db2_slot: &mut f64,
        var_guard9_db3_slot: &mut f64,
        var_guard9_db4_slot: &mut f64,
        var_guard9_db5_slot: &mut f64,
        var_guard9_db6_slot: &mut f64,
        var_guard9_db7_slot: &mut f64,
        var_guard9_dn0_slot: &mut f64,
        var_guard9_dn1_slot: &mut f64,
        var_guard9_dn2_slot: &mut f64,
        var_guard9_dn3_slot: &mut f64,
        var_guard9_dn4_slot: &mut f64,
        var_guard9_dn5_slot: &mut f64,
        var_guard9_dn6_slot: &mut f64,
        var_guard9_dn7_slot: &mut f64,
        var_guard9_dn8_slot: &mut f64,
        var_guard9_dn9_slot: &mut f64,
        var_guard9_rdb0_slot: &mut f64,
        var_guard9_rdb1_slot: &mut f64,
        var_guard9_rdb2_slot: &mut f64,
        var_guard9_rdb3_slot: &mut f64,
        var_guard9_rdb4_slot: &mut f64,
        var_guard9_rdb5_slot: &mut f64,
        var_guard9_rdb6_slot: &mut f64,
        var_guard9_rdb7_slot: &mut f64,
        var_guard9_rdn0_slot: &mut f64,
        var_guard9_rdn1_slot: &mut f64,
        var_guard9_rdn2_slot: &mut f64,
        var_guard9_rdn3_slot: &mut f64,
        var_guard9_rdn4_slot: &mut f64,
        var_guard9_rdn5_slot: &mut f64,
        var_guard9_rdn6_slot: &mut f64,
        var_guard9_rdn7_slot: &mut f64,
        var_guard9_rdn8_slot: &mut f64,
        var_guard9_rdn9_slot: &mut f64,
        var_guard9_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_db0_slot: &mut f64,
        var_le_db1_slot: &mut f64,
        var_le_db2_slot: &mut f64,
        var_le_db3_slot: &mut f64,
        var_le_db4_slot: &mut f64,
        var_le_db5_slot: &mut f64,
        var_le_db6_slot: &mut f64,
        var_le_db7_slot: &mut f64,
        var_le_dn0_slot: &mut f64,
        var_le_dn1_slot: &mut f64,
        var_le_dn2_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn5_slot: &mut f64,
        var_le_dn6_slot: &mut f64,
        var_le_dn7_slot: &mut f64,
        var_le_dn8_slot: &mut f64,
        var_le_dn9_slot: &mut f64,
        var_le_rdb0_slot: &mut f64,
        var_le_rdb1_slot: &mut f64,
        var_le_rdb2_slot: &mut f64,
        var_le_rdb3_slot: &mut f64,
        var_le_rdb4_slot: &mut f64,
        var_le_rdb5_slot: &mut f64,
        var_le_rdb6_slot: &mut f64,
        var_le_rdb7_slot: &mut f64,
        var_le_rdn0_slot: &mut f64,
        var_le_rdn1_slot: &mut f64,
        var_le_rdn2_slot: &mut f64,
        var_le_rdn3_slot: &mut f64,
        var_le_rdn4_slot: &mut f64,
        var_le_rdn5_slot: &mut f64,
        var_le_rdn6_slot: &mut f64,
        var_le_rdn7_slot: &mut f64,
        var_le_rdn8_slot: &mut f64,
        var_le_rdn9_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_lebv_slot: &mut f64,
        var_lebv_db0_slot: &mut f64,
        var_lebv_db1_slot: &mut f64,
        var_lebv_db2_slot: &mut f64,
        var_lebv_db3_slot: &mut f64,
        var_lebv_db4_slot: &mut f64,
        var_lebv_db5_slot: &mut f64,
        var_lebv_db6_slot: &mut f64,
        var_lebv_db7_slot: &mut f64,
        var_lebv_dn0_slot: &mut f64,
        var_lebv_dn1_slot: &mut f64,
        var_lebv_dn2_slot: &mut f64,
        var_lebv_dn3_slot: &mut f64,
        var_lebv_dn4_slot: &mut f64,
        var_lebv_dn5_slot: &mut f64,
        var_lebv_dn6_slot: &mut f64,
        var_lebv_dn7_slot: &mut f64,
        var_lebv_dn8_slot: &mut f64,
        var_lebv_dn9_slot: &mut f64,
        var_lebv_rdb0_slot: &mut f64,
        var_lebv_rdb1_slot: &mut f64,
        var_lebv_rdb2_slot: &mut f64,
        var_lebv_rdb3_slot: &mut f64,
        var_lebv_rdb4_slot: &mut f64,
        var_lebv_rdb5_slot: &mut f64,
        var_lebv_rdb6_slot: &mut f64,
        var_lebv_rdb7_slot: &mut f64,
        var_lebv_rdn0_slot: &mut f64,
        var_lebv_rdn1_slot: &mut f64,
        var_lebv_rdn2_slot: &mut f64,
        var_lebv_rdn3_slot: &mut f64,
        var_lebv_rdn4_slot: &mut f64,
        var_lebv_rdn5_slot: &mut f64,
        var_lebv_rdn6_slot: &mut f64,
        var_lebv_rdn7_slot: &mut f64,
        var_lebv_rdn8_slot: &mut f64,
        var_lebv_rdn9_slot: &mut f64,
        var_lebv_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_db0: f64 = *var_arg_db0_slot;
        let mut var_arg_db1: f64 = *var_arg_db1_slot;
        let mut var_arg_db2: f64 = *var_arg_db2_slot;
        let mut var_arg_db3: f64 = *var_arg_db3_slot;
        let mut var_arg_db4: f64 = *var_arg_db4_slot;
        let mut var_arg_db5: f64 = *var_arg_db5_slot;
        let mut var_arg_db6: f64 = *var_arg_db6_slot;
        let mut var_arg_db7: f64 = *var_arg_db7_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn1: f64 = *var_arg_dn1_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_arg_rdb0: f64 = *var_arg_rdb0_slot;
        let mut var_arg_rdb1: f64 = *var_arg_rdb1_slot;
        let mut var_arg_rdb2: f64 = *var_arg_rdb2_slot;
        let mut var_arg_rdb3: f64 = *var_arg_rdb3_slot;
        let mut var_arg_rdb4: f64 = *var_arg_rdb4_slot;
        let mut var_arg_rdb5: f64 = *var_arg_rdb5_slot;
        let mut var_arg_rdb6: f64 = *var_arg_rdb6_slot;
        let mut var_arg_rdb7: f64 = *var_arg_rdb7_slot;
        let mut var_arg_rdn0: f64 = *var_arg_rdn0_slot;
        let mut var_arg_rdn1: f64 = *var_arg_rdn1_slot;
        let mut var_arg_rdn2: f64 = *var_arg_rdn2_slot;
        let mut var_arg_rdn3: f64 = *var_arg_rdn3_slot;
        let mut var_arg_rdn4: f64 = *var_arg_rdn4_slot;
        let mut var_arg_rdn5: f64 = *var_arg_rdn5_slot;
        let mut var_arg_rdn6: f64 = *var_arg_rdn6_slot;
        let mut var_arg_rdn7: f64 = *var_arg_rdn7_slot;
        let mut var_arg_rdn8: f64 = *var_arg_rdn8_slot;
        let mut var_arg_rdn9: f64 = *var_arg_rdn9_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_argbv: f64 = *var_argbv_slot;
        let mut var_argbv_db0: f64 = *var_argbv_db0_slot;
        let mut var_argbv_db1: f64 = *var_argbv_db1_slot;
        let mut var_argbv_db2: f64 = *var_argbv_db2_slot;
        let mut var_argbv_db3: f64 = *var_argbv_db3_slot;
        let mut var_argbv_db4: f64 = *var_argbv_db4_slot;
        let mut var_argbv_db5: f64 = *var_argbv_db5_slot;
        let mut var_argbv_db6: f64 = *var_argbv_db6_slot;
        let mut var_argbv_db7: f64 = *var_argbv_db7_slot;
        let mut var_argbv_dn0: f64 = *var_argbv_dn0_slot;
        let mut var_argbv_dn1: f64 = *var_argbv_dn1_slot;
        let mut var_argbv_dn2: f64 = *var_argbv_dn2_slot;
        let mut var_argbv_dn3: f64 = *var_argbv_dn3_slot;
        let mut var_argbv_dn4: f64 = *var_argbv_dn4_slot;
        let mut var_argbv_dn5: f64 = *var_argbv_dn5_slot;
        let mut var_argbv_dn6: f64 = *var_argbv_dn6_slot;
        let mut var_argbv_dn7: f64 = *var_argbv_dn7_slot;
        let mut var_argbv_dn8: f64 = *var_argbv_dn8_slot;
        let mut var_argbv_dn9: f64 = *var_argbv_dn9_slot;
        let mut var_argbv_rdb0: f64 = *var_argbv_rdb0_slot;
        let mut var_argbv_rdb1: f64 = *var_argbv_rdb1_slot;
        let mut var_argbv_rdb2: f64 = *var_argbv_rdb2_slot;
        let mut var_argbv_rdb3: f64 = *var_argbv_rdb3_slot;
        let mut var_argbv_rdb4: f64 = *var_argbv_rdb4_slot;
        let mut var_argbv_rdb5: f64 = *var_argbv_rdb5_slot;
        let mut var_argbv_rdb6: f64 = *var_argbv_rdb6_slot;
        let mut var_argbv_rdb7: f64 = *var_argbv_rdb7_slot;
        let mut var_argbv_rdn0: f64 = *var_argbv_rdn0_slot;
        let mut var_argbv_rdn1: f64 = *var_argbv_rdn1_slot;
        let mut var_argbv_rdn2: f64 = *var_argbv_rdn2_slot;
        let mut var_argbv_rdn3: f64 = *var_argbv_rdn3_slot;
        let mut var_argbv_rdn4: f64 = *var_argbv_rdn4_slot;
        let mut var_argbv_rdn5: f64 = *var_argbv_rdn5_slot;
        let mut var_argbv_rdn6: f64 = *var_argbv_rdn6_slot;
        let mut var_argbv_rdn7: f64 = *var_argbv_rdn7_slot;
        let mut var_argbv_rdn8: f64 = *var_argbv_rdn8_slot;
        let mut var_argbv_rdn9: f64 = *var_argbv_rdn9_slot;
        let mut var_argbv_rv: f64 = *var_argbv_rv_slot;
        let mut var_argbvvt: f64 = *var_argbvvt_slot;
        let mut var_argbvvt_db0: f64 = *var_argbvvt_db0_slot;
        let mut var_argbvvt_db1: f64 = *var_argbvvt_db1_slot;
        let mut var_argbvvt_db2: f64 = *var_argbvvt_db2_slot;
        let mut var_argbvvt_db3: f64 = *var_argbvvt_db3_slot;
        let mut var_argbvvt_db4: f64 = *var_argbvvt_db4_slot;
        let mut var_argbvvt_db5: f64 = *var_argbvvt_db5_slot;
        let mut var_argbvvt_db6: f64 = *var_argbvvt_db6_slot;
        let mut var_argbvvt_db7: f64 = *var_argbvvt_db7_slot;
        let mut var_argbvvt_dn0: f64 = *var_argbvvt_dn0_slot;
        let mut var_argbvvt_dn1: f64 = *var_argbvvt_dn1_slot;
        let mut var_argbvvt_dn2: f64 = *var_argbvvt_dn2_slot;
        let mut var_argbvvt_dn3: f64 = *var_argbvvt_dn3_slot;
        let mut var_argbvvt_dn4: f64 = *var_argbvvt_dn4_slot;
        let mut var_argbvvt_dn5: f64 = *var_argbvvt_dn5_slot;
        let mut var_argbvvt_dn6: f64 = *var_argbvvt_dn6_slot;
        let mut var_argbvvt_dn7: f64 = *var_argbvvt_dn7_slot;
        let mut var_argbvvt_dn8: f64 = *var_argbvvt_dn8_slot;
        let mut var_argbvvt_dn9: f64 = *var_argbvvt_dn9_slot;
        let mut var_argbvvt_rdb0: f64 = *var_argbvvt_rdb0_slot;
        let mut var_argbvvt_rdb1: f64 = *var_argbvvt_rdb1_slot;
        let mut var_argbvvt_rdb2: f64 = *var_argbvvt_rdb2_slot;
        let mut var_argbvvt_rdb3: f64 = *var_argbvvt_rdb3_slot;
        let mut var_argbvvt_rdb4: f64 = *var_argbvvt_rdb4_slot;
        let mut var_argbvvt_rdb5: f64 = *var_argbvvt_rdb5_slot;
        let mut var_argbvvt_rdb6: f64 = *var_argbvvt_rdb6_slot;
        let mut var_argbvvt_rdb7: f64 = *var_argbvvt_rdb7_slot;
        let mut var_argbvvt_rdn0: f64 = *var_argbvvt_rdn0_slot;
        let mut var_argbvvt_rdn1: f64 = *var_argbvvt_rdn1_slot;
        let mut var_argbvvt_rdn2: f64 = *var_argbvvt_rdn2_slot;
        let mut var_argbvvt_rdn3: f64 = *var_argbvvt_rdn3_slot;
        let mut var_argbvvt_rdn4: f64 = *var_argbvvt_rdn4_slot;
        let mut var_argbvvt_rdn5: f64 = *var_argbvvt_rdn5_slot;
        let mut var_argbvvt_rdn6: f64 = *var_argbvvt_rdn6_slot;
        let mut var_argbvvt_rdn7: f64 = *var_argbvvt_rdn7_slot;
        let mut var_argbvvt_rdn8: f64 = *var_argbvvt_rdn8_slot;
        let mut var_argbvvt_rdn9: f64 = *var_argbvvt_rdn9_slot;
        let mut var_argbvvt_rv: f64 = *var_argbvvt_rv_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_guard9_db0: f64 = *var_guard9_db0_slot;
        let mut var_guard9_db1: f64 = *var_guard9_db1_slot;
        let mut var_guard9_db2: f64 = *var_guard9_db2_slot;
        let mut var_guard9_db3: f64 = *var_guard9_db3_slot;
        let mut var_guard9_db4: f64 = *var_guard9_db4_slot;
        let mut var_guard9_db5: f64 = *var_guard9_db5_slot;
        let mut var_guard9_db6: f64 = *var_guard9_db6_slot;
        let mut var_guard9_db7: f64 = *var_guard9_db7_slot;
        let mut var_guard9_dn0: f64 = *var_guard9_dn0_slot;
        let mut var_guard9_dn1: f64 = *var_guard9_dn1_slot;
        let mut var_guard9_dn2: f64 = *var_guard9_dn2_slot;
        let mut var_guard9_dn3: f64 = *var_guard9_dn3_slot;
        let mut var_guard9_dn4: f64 = *var_guard9_dn4_slot;
        let mut var_guard9_dn5: f64 = *var_guard9_dn5_slot;
        let mut var_guard9_dn6: f64 = *var_guard9_dn6_slot;
        let mut var_guard9_dn7: f64 = *var_guard9_dn7_slot;
        let mut var_guard9_dn8: f64 = *var_guard9_dn8_slot;
        let mut var_guard9_dn9: f64 = *var_guard9_dn9_slot;
        let mut var_guard9_rdb0: f64 = *var_guard9_rdb0_slot;
        let mut var_guard9_rdb1: f64 = *var_guard9_rdb1_slot;
        let mut var_guard9_rdb2: f64 = *var_guard9_rdb2_slot;
        let mut var_guard9_rdb3: f64 = *var_guard9_rdb3_slot;
        let mut var_guard9_rdb4: f64 = *var_guard9_rdb4_slot;
        let mut var_guard9_rdb5: f64 = *var_guard9_rdb5_slot;
        let mut var_guard9_rdb6: f64 = *var_guard9_rdb6_slot;
        let mut var_guard9_rdb7: f64 = *var_guard9_rdb7_slot;
        let mut var_guard9_rdn0: f64 = *var_guard9_rdn0_slot;
        let mut var_guard9_rdn1: f64 = *var_guard9_rdn1_slot;
        let mut var_guard9_rdn2: f64 = *var_guard9_rdn2_slot;
        let mut var_guard9_rdn3: f64 = *var_guard9_rdn3_slot;
        let mut var_guard9_rdn4: f64 = *var_guard9_rdn4_slot;
        let mut var_guard9_rdn5: f64 = *var_guard9_rdn5_slot;
        let mut var_guard9_rdn6: f64 = *var_guard9_rdn6_slot;
        let mut var_guard9_rdn7: f64 = *var_guard9_rdn7_slot;
        let mut var_guard9_rdn8: f64 = *var_guard9_rdn8_slot;
        let mut var_guard9_rdn9: f64 = *var_guard9_rdn9_slot;
        let mut var_guard9_rv: f64 = *var_guard9_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_db0: f64 = *var_le_db0_slot;
        let mut var_le_db1: f64 = *var_le_db1_slot;
        let mut var_le_db2: f64 = *var_le_db2_slot;
        let mut var_le_db3: f64 = *var_le_db3_slot;
        let mut var_le_db4: f64 = *var_le_db4_slot;
        let mut var_le_db5: f64 = *var_le_db5_slot;
        let mut var_le_db6: f64 = *var_le_db6_slot;
        let mut var_le_db7: f64 = *var_le_db7_slot;
        let mut var_le_dn0: f64 = *var_le_dn0_slot;
        let mut var_le_dn1: f64 = *var_le_dn1_slot;
        let mut var_le_dn2: f64 = *var_le_dn2_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn5: f64 = *var_le_dn5_slot;
        let mut var_le_dn6: f64 = *var_le_dn6_slot;
        let mut var_le_dn7: f64 = *var_le_dn7_slot;
        let mut var_le_dn8: f64 = *var_le_dn8_slot;
        let mut var_le_dn9: f64 = *var_le_dn9_slot;
        let mut var_le_rdb0: f64 = *var_le_rdb0_slot;
        let mut var_le_rdb1: f64 = *var_le_rdb1_slot;
        let mut var_le_rdb2: f64 = *var_le_rdb2_slot;
        let mut var_le_rdb3: f64 = *var_le_rdb3_slot;
        let mut var_le_rdb4: f64 = *var_le_rdb4_slot;
        let mut var_le_rdb5: f64 = *var_le_rdb5_slot;
        let mut var_le_rdb6: f64 = *var_le_rdb6_slot;
        let mut var_le_rdb7: f64 = *var_le_rdb7_slot;
        let mut var_le_rdn0: f64 = *var_le_rdn0_slot;
        let mut var_le_rdn1: f64 = *var_le_rdn1_slot;
        let mut var_le_rdn2: f64 = *var_le_rdn2_slot;
        let mut var_le_rdn3: f64 = *var_le_rdn3_slot;
        let mut var_le_rdn4: f64 = *var_le_rdn4_slot;
        let mut var_le_rdn5: f64 = *var_le_rdn5_slot;
        let mut var_le_rdn6: f64 = *var_le_rdn6_slot;
        let mut var_le_rdn7: f64 = *var_le_rdn7_slot;
        let mut var_le_rdn8: f64 = *var_le_rdn8_slot;
        let mut var_le_rdn9: f64 = *var_le_rdn9_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_lebv: f64 = *var_lebv_slot;
        let mut var_lebv_db0: f64 = *var_lebv_db0_slot;
        let mut var_lebv_db1: f64 = *var_lebv_db1_slot;
        let mut var_lebv_db2: f64 = *var_lebv_db2_slot;
        let mut var_lebv_db3: f64 = *var_lebv_db3_slot;
        let mut var_lebv_db4: f64 = *var_lebv_db4_slot;
        let mut var_lebv_db5: f64 = *var_lebv_db5_slot;
        let mut var_lebv_db6: f64 = *var_lebv_db6_slot;
        let mut var_lebv_db7: f64 = *var_lebv_db7_slot;
        let mut var_lebv_dn0: f64 = *var_lebv_dn0_slot;
        let mut var_lebv_dn1: f64 = *var_lebv_dn1_slot;
        let mut var_lebv_dn2: f64 = *var_lebv_dn2_slot;
        let mut var_lebv_dn3: f64 = *var_lebv_dn3_slot;
        let mut var_lebv_dn4: f64 = *var_lebv_dn4_slot;
        let mut var_lebv_dn5: f64 = *var_lebv_dn5_slot;
        let mut var_lebv_dn6: f64 = *var_lebv_dn6_slot;
        let mut var_lebv_dn7: f64 = *var_lebv_dn7_slot;
        let mut var_lebv_dn8: f64 = *var_lebv_dn8_slot;
        let mut var_lebv_dn9: f64 = *var_lebv_dn9_slot;
        let mut var_lebv_rdb0: f64 = *var_lebv_rdb0_slot;
        let mut var_lebv_rdb1: f64 = *var_lebv_rdb1_slot;
        let mut var_lebv_rdb2: f64 = *var_lebv_rdb2_slot;
        let mut var_lebv_rdb3: f64 = *var_lebv_rdb3_slot;
        let mut var_lebv_rdb4: f64 = *var_lebv_rdb4_slot;
        let mut var_lebv_rdb5: f64 = *var_lebv_rdb5_slot;
        let mut var_lebv_rdb6: f64 = *var_lebv_rdb6_slot;
        let mut var_lebv_rdb7: f64 = *var_lebv_rdb7_slot;
        let mut var_lebv_rdn0: f64 = *var_lebv_rdn0_slot;
        let mut var_lebv_rdn1: f64 = *var_lebv_rdn1_slot;
        let mut var_lebv_rdn2: f64 = *var_lebv_rdn2_slot;
        let mut var_lebv_rdn3: f64 = *var_lebv_rdn3_slot;
        let mut var_lebv_rdn4: f64 = *var_lebv_rdn4_slot;
        let mut var_lebv_rdn5: f64 = *var_lebv_rdn5_slot;
        let mut var_lebv_rdn6: f64 = *var_lebv_rdn6_slot;
        let mut var_lebv_rdn7: f64 = *var_lebv_rdn7_slot;
        let mut var_lebv_rdn8: f64 = *var_lebv_rdn8_slot;
        let mut var_lebv_rdn9: f64 = *var_lebv_rdn9_slot;
        let mut var_lebv_rv: f64 = *var_lebv_rv_slot;

        let (assign980_e1211, assign980_e1211_d_n0, assign980_e1211_d_n1, assign980_e1211_d_n2, assign980_e1211_d_n3, assign980_e1211_d_n4, assign980_e1211_d_n5, assign980_e1211_d_n6, assign980_e1211_d_n7, assign980_e1211_d_n8, assign980_e1211_d_n9, assign980_e1211_d_b0, assign980_e1211_d_b1, assign980_e1211_d_b2, assign980_e1211_d_b3, assign980_e1211_d_b4, assign980_e1211_d_b5, assign980_e1211_d_b6, assign980_e1211_d_b7,) = {
    if ((var_guard7 != 0.0) && (var_guard8 != 0.0)) {
        let assign980_e1208: f64 = (var_arg - 80.0);
        let assign980_e1209: f64 = (1.0 + assign980_e1208);
        (assign980_e1209, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign980_e1211;
        var_le_dn0 = assign980_e1211_d_n0;
        var_le_dn1 = assign980_e1211_d_n1;
        var_le_dn2 = assign980_e1211_d_n2;
        var_le_dn3 = assign980_e1211_d_n3;
        var_le_dn4 = assign980_e1211_d_n4;
        var_le_dn5 = assign980_e1211_d_n5;
        var_le_dn6 = assign980_e1211_d_n6;
        var_le_dn7 = assign980_e1211_d_n7;
        var_le_dn8 = assign980_e1211_d_n8;
        var_le_dn9 = assign980_e1211_d_n9;
        var_le_db0 = assign980_e1211_d_b0;
        var_le_db1 = assign980_e1211_d_b1;
        var_le_db2 = assign980_e1211_d_b2;
        var_le_db3 = assign980_e1211_d_b3;
        var_le_db4 = assign980_e1211_d_b4;
        var_le_db5 = assign980_e1211_d_b5;
        var_le_db6 = assign980_e1211_d_b6;
        var_le_db7 = assign980_e1211_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign990_e1217, assign990_e1217_d_n0, assign990_e1217_d_n1, assign990_e1217_d_n2, assign990_e1217_d_n3, assign990_e1217_d_n4, assign990_e1217_d_n5, assign990_e1217_d_n6, assign990_e1217_d_n7, assign990_e1217_d_n8, assign990_e1217_d_n9, assign990_e1217_d_b0, assign990_e1217_d_b1, assign990_e1217_d_b2, assign990_e1217_d_b3, assign990_e1217_d_b4, assign990_e1217_d_b5, assign990_e1217_d_b6, assign990_e1217_d_b7,) = {
    if ((var_guard7 != 0.0) && (var_guard8 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    }
};
        var_arg = assign990_e1217;
        var_arg_dn0 = assign990_e1217_d_n0;
        var_arg_dn1 = assign990_e1217_d_n1;
        var_arg_dn2 = assign990_e1217_d_n2;
        var_arg_dn3 = assign990_e1217_d_n3;
        var_arg_dn4 = assign990_e1217_d_n4;
        var_arg_dn5 = assign990_e1217_d_n5;
        var_arg_dn6 = assign990_e1217_d_n6;
        var_arg_dn7 = assign990_e1217_d_n7;
        var_arg_dn8 = assign990_e1217_d_n8;
        var_arg_dn9 = assign990_e1217_d_n9;
        var_arg_db0 = assign990_e1217_d_b0;
        var_arg_db1 = assign990_e1217_d_b1;
        var_arg_db2 = assign990_e1217_d_b2;
        var_arg_db3 = assign990_e1217_d_b3;
        var_arg_db4 = assign990_e1217_d_b4;
        var_arg_db5 = assign990_e1217_d_b5;
        var_arg_db6 = assign990_e1217_d_b6;
        var_arg_db7 = assign990_e1217_d_b7;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdn7 = 0.0;
        var_arg_rdn8 = 0.0;
        var_arg_rdn9 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;
        var_arg_rdb7 = 0.0;

        let (assign1000_e1224, assign1000_e1224_d_n0, assign1000_e1224_d_n1, assign1000_e1224_d_n2, assign1000_e1224_d_n3, assign1000_e1224_d_n4, assign1000_e1224_d_n5, assign1000_e1224_d_n6, assign1000_e1224_d_n7, assign1000_e1224_d_n8, assign1000_e1224_d_n9, assign1000_e1224_d_b0, assign1000_e1224_d_b1, assign1000_e1224_d_b2, assign1000_e1224_d_b3, assign1000_e1224_d_b4, assign1000_e1224_d_b5, assign1000_e1224_d_b6, assign1000_e1224_d_b7,) = {
    if ((var_guard7 != 0.0) && (var_guard8 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign1000_e1224;
        var_le_dn0 = assign1000_e1224_d_n0;
        var_le_dn1 = assign1000_e1224_d_n1;
        var_le_dn2 = assign1000_e1224_d_n2;
        var_le_dn3 = assign1000_e1224_d_n3;
        var_le_dn4 = assign1000_e1224_d_n4;
        var_le_dn5 = assign1000_e1224_d_n5;
        var_le_dn6 = assign1000_e1224_d_n6;
        var_le_dn7 = assign1000_e1224_d_n7;
        var_le_dn8 = assign1000_e1224_d_n8;
        var_le_dn9 = assign1000_e1224_d_n9;
        var_le_db0 = assign1000_e1224_d_b0;
        var_le_db1 = assign1000_e1224_d_b1;
        var_le_db2 = assign1000_e1224_d_b2;
        var_le_db3 = assign1000_e1224_d_b3;
        var_le_db4 = assign1000_e1224_d_b4;
        var_le_db5 = assign1000_e1224_d_b5;
        var_le_db6 = assign1000_e1224_d_b6;
        var_le_db7 = assign1000_e1224_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign1010_e1231, assign1010_e1231_d_n0, assign1010_e1231_d_n1, assign1010_e1231_d_n2, assign1010_e1231_d_n3, assign1010_e1231_d_n4, assign1010_e1231_d_n5, assign1010_e1231_d_n6, assign1010_e1231_d_n7, assign1010_e1231_d_n8, assign1010_e1231_d_n9, assign1010_e1231_d_b0, assign1010_e1231_d_b1, assign1010_e1231_d_b2, assign1010_e1231_d_b3, assign1010_e1231_d_b4, assign1010_e1231_d_b5, assign1010_e1231_d_b6, assign1010_e1231_d_b7,) = {
    if (var_guard7 != 0.0) {
        let assign1010_e1228: f64 = (var_arg).exp();
        let assign1010_e1229: f64 = (var_le * assign1010_e1228);
        (assign1010_e1229, ((var_le_dn0 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn0))), ((var_le_dn1 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn1))), ((var_le_dn2 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn2))), ((var_le_dn3 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn3))), ((var_le_dn4 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn4))), ((var_le_dn5 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn5))), ((var_le_dn6 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn6))), ((var_le_dn7 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn7))), ((var_le_dn8 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn8))), ((var_le_dn9 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_dn9))), ((var_le_db0 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_db0))), ((var_le_db1 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_db1))), ((var_le_db2 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_db2))), ((var_le_db3 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_db3))), ((var_le_db4 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_db4))), ((var_le_db5 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_db5))), ((var_le_db6 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_db6))), ((var_le_db7 * assign1010_e1228) + (var_le * (assign1010_e1228 * var_arg_db7))),)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign1010_e1231;
        var_le_dn0 = assign1010_e1231_d_n0;
        var_le_dn1 = assign1010_e1231_d_n1;
        var_le_dn2 = assign1010_e1231_d_n2;
        var_le_dn3 = assign1010_e1231_d_n3;
        var_le_dn4 = assign1010_e1231_d_n4;
        var_le_dn5 = assign1010_e1231_d_n5;
        var_le_dn6 = assign1010_e1231_d_n6;
        var_le_dn7 = assign1010_e1231_d_n7;
        var_le_dn8 = assign1010_e1231_d_n8;
        var_le_dn9 = assign1010_e1231_d_n9;
        var_le_db0 = assign1010_e1231_d_b0;
        var_le_db1 = assign1010_e1231_d_b1;
        var_le_db2 = assign1010_e1231_d_b2;
        var_le_db3 = assign1010_e1231_d_b3;
        var_le_db4 = assign1010_e1231_d_b4;
        var_le_db5 = assign1010_e1231_d_b5;
        var_le_db6 = assign1010_e1231_d_b6;
        var_le_db7 = assign1010_e1231_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign1020_e1303, assign1020_e1303_d_n0, assign1020_e1303_d_n1, assign1020_e1303_d_n2, assign1020_e1303_d_n3, assign1020_e1303_d_n4, assign1020_e1303_d_n5, assign1020_e1303_d_n6, assign1020_e1303_d_n7, assign1020_e1303_d_n8, assign1020_e1303_d_n9, assign1020_e1303_d_b0, assign1020_e1303_d_b1, assign1020_e1303_d_b2, assign1020_e1303_d_b3, assign1020_e1303_d_b4, assign1020_e1303_d_b5, assign1020_e1303_d_b6, assign1020_e1303_d_b7,) = {
    if (var_guard7 != 0.0) {
        let assign1020_e1239: f64 = (-37.0);
        let (assign1020_e1266, assign1020_e1266_d_n0, assign1020_e1266_d_n1, assign1020_e1266_d_n2, assign1020_e1266_d_n3, assign1020_e1266_d_n4, assign1020_e1266_d_n5, assign1020_e1266_d_n6, assign1020_e1266_d_n7, assign1020_e1266_d_n8, assign1020_e1266_d_n9, assign1020_e1266_d_b0, assign1020_e1266_d_b1, assign1020_e1266_d_b2, assign1020_e1266_d_b3, assign1020_e1266_d_b4, assign1020_e1266_d_b5, assign1020_e1266_d_b6, assign1020_e1266_d_b7,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign1020_e1239))) {
                let assign1020_e1244: f64 = (var_argbv).exp();
                let assign1020_e1246: f64 = (assign1020_e1244 + 1.0);
                let assign1020_e1247: f64 = (assign1020_e1246).ln();
                (assign1020_e1247, ((assign1020_e1244 * var_argbv_dn0) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn1) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn2) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn3) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn4) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn5) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn6) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn7) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn8) / assign1020_e1246), ((assign1020_e1244 * var_argbv_dn9) / assign1020_e1246), ((assign1020_e1244 * var_argbv_db0) / assign1020_e1246), ((assign1020_e1244 * var_argbv_db1) / assign1020_e1246), ((assign1020_e1244 * var_argbv_db2) / assign1020_e1246), ((assign1020_e1244 * var_argbv_db3) / assign1020_e1246), ((assign1020_e1244 * var_argbv_db4) / assign1020_e1246), ((assign1020_e1244 * var_argbv_db5) / assign1020_e1246), ((assign1020_e1244 * var_argbv_db6) / assign1020_e1246), ((assign1020_e1244 * var_argbv_db7) / assign1020_e1246),)
            } else {
                let assign1020_e1254: f64 = (-37.0);
                let (assign1020_e1265, assign1020_e1265_d_n0, assign1020_e1265_d_n1, assign1020_e1265_d_n2, assign1020_e1265_d_n3, assign1020_e1265_d_n4, assign1020_e1265_d_n5, assign1020_e1265_d_n6, assign1020_e1265_d_n7, assign1020_e1265_d_n8, assign1020_e1265_d_n9, assign1020_e1265_d_b0, assign1020_e1265_d_b1, assign1020_e1265_d_b2, assign1020_e1265_d_b3, assign1020_e1265_d_b4, assign1020_e1265_d_b5, assign1020_e1265_d_b6, assign1020_e1265_d_b7,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign1020_e1254)) {
                        let assign1020_e1258: f64 = (var_argbv).exp();
                        (assign1020_e1258, (assign1020_e1258 * var_argbv_dn0), (assign1020_e1258 * var_argbv_dn1), (assign1020_e1258 * var_argbv_dn2), (assign1020_e1258 * var_argbv_dn3), (assign1020_e1258 * var_argbv_dn4), (assign1020_e1258 * var_argbv_dn5), (assign1020_e1258 * var_argbv_dn6), (assign1020_e1258 * var_argbv_dn7), (assign1020_e1258 * var_argbv_dn8), (assign1020_e1258 * var_argbv_dn9), (assign1020_e1258 * var_argbv_db0), (assign1020_e1258 * var_argbv_db1), (assign1020_e1258 * var_argbv_db2), (assign1020_e1258 * var_argbv_db3), (assign1020_e1258 * var_argbv_db4), (assign1020_e1258 * var_argbv_db5), (assign1020_e1258 * var_argbv_db6), (assign1020_e1258 * var_argbv_db7),)
                    } else {
                        let (assign1020_e1264, assign1020_e1264_d_n0, assign1020_e1264_d_n1, assign1020_e1264_d_n2, assign1020_e1264_d_n3, assign1020_e1264_d_n4, assign1020_e1264_d_n5, assign1020_e1264_d_n6, assign1020_e1264_d_n7, assign1020_e1264_d_n8, assign1020_e1264_d_n9, assign1020_e1264_d_b0, assign1020_e1264_d_b1, assign1020_e1264_d_b2, assign1020_e1264_d_b3, assign1020_e1264_d_b4, assign1020_e1264_d_b5, assign1020_e1264_d_b6, assign1020_e1264_d_b7,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn0, var_argbv_dn1, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6, var_argbv_dn7, var_argbv_dn8, var_argbv_dn9, var_argbv_db0, var_argbv_db1, var_argbv_db2, var_argbv_db3, var_argbv_db4, var_argbv_db5, var_argbv_db6, var_argbv_db7,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign1020_e1264, assign1020_e1264_d_n0, assign1020_e1264_d_n1, assign1020_e1264_d_n2, assign1020_e1264_d_n3, assign1020_e1264_d_n4, assign1020_e1264_d_n5, assign1020_e1264_d_n6, assign1020_e1264_d_n7, assign1020_e1264_d_n8, assign1020_e1264_d_n9, assign1020_e1264_d_b0, assign1020_e1264_d_b1, assign1020_e1264_d_b2, assign1020_e1264_d_b3, assign1020_e1264_d_b4, assign1020_e1264_d_b5, assign1020_e1264_d_b6, assign1020_e1264_d_b7,)
                    }
                };
                (assign1020_e1265, assign1020_e1265_d_n0, assign1020_e1265_d_n1, assign1020_e1265_d_n2, assign1020_e1265_d_n3, assign1020_e1265_d_n4, assign1020_e1265_d_n5, assign1020_e1265_d_n6, assign1020_e1265_d_n7, assign1020_e1265_d_n8, assign1020_e1265_d_n9, assign1020_e1265_d_b0, assign1020_e1265_d_b1, assign1020_e1265_d_b2, assign1020_e1265_d_b3, assign1020_e1265_d_b4, assign1020_e1265_d_b5, assign1020_e1265_d_b6, assign1020_e1265_d_b7,)
            }
        };
        let assign1020_e1273: f64 = (-37.0);
        let (assign1020_e1300, assign1020_e1300_d_n0, assign1020_e1300_d_n1, assign1020_e1300_d_n2, assign1020_e1300_d_n3, assign1020_e1300_d_n4, assign1020_e1300_d_n5, assign1020_e1300_d_n6, assign1020_e1300_d_n7, assign1020_e1300_d_n8, assign1020_e1300_d_n9, assign1020_e1300_d_b0, assign1020_e1300_d_b1, assign1020_e1300_d_b2, assign1020_e1300_d_b3, assign1020_e1300_d_b4, assign1020_e1300_d_b5, assign1020_e1300_d_b6, assign1020_e1300_d_b7,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign1020_e1273))) {
                let assign1020_e1278: f64 = (var_argbvvt).exp();
                let assign1020_e1280: f64 = (assign1020_e1278 + 1.0);
                let assign1020_e1281: f64 = (assign1020_e1280).ln();
                (assign1020_e1281, ((assign1020_e1278 * var_argbvvt_dn0) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_dn1) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_dn2) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_dn3) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_dn4) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_dn5) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_dn6) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_dn7) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_dn8) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_dn9) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_db0) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_db1) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_db2) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_db3) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_db4) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_db5) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_db6) / assign1020_e1280), ((assign1020_e1278 * var_argbvvt_db7) / assign1020_e1280),)
            } else {
                let assign1020_e1288: f64 = (-37.0);
                let (assign1020_e1299, assign1020_e1299_d_n0, assign1020_e1299_d_n1, assign1020_e1299_d_n2, assign1020_e1299_d_n3, assign1020_e1299_d_n4, assign1020_e1299_d_n5, assign1020_e1299_d_n6, assign1020_e1299_d_n7, assign1020_e1299_d_n8, assign1020_e1299_d_n9, assign1020_e1299_d_b0, assign1020_e1299_d_b1, assign1020_e1299_d_b2, assign1020_e1299_d_b3, assign1020_e1299_d_b4, assign1020_e1299_d_b5, assign1020_e1299_d_b6, assign1020_e1299_d_b7,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign1020_e1288)) {
                        let assign1020_e1292: f64 = (var_argbvvt).exp();
                        (assign1020_e1292, (assign1020_e1292 * var_argbvvt_dn0), (assign1020_e1292 * var_argbvvt_dn1), (assign1020_e1292 * var_argbvvt_dn2), (assign1020_e1292 * var_argbvvt_dn3), (assign1020_e1292 * var_argbvvt_dn4), (assign1020_e1292 * var_argbvvt_dn5), (assign1020_e1292 * var_argbvvt_dn6), (assign1020_e1292 * var_argbvvt_dn7), (assign1020_e1292 * var_argbvvt_dn8), (assign1020_e1292 * var_argbvvt_dn9), (assign1020_e1292 * var_argbvvt_db0), (assign1020_e1292 * var_argbvvt_db1), (assign1020_e1292 * var_argbvvt_db2), (assign1020_e1292 * var_argbvvt_db3), (assign1020_e1292 * var_argbvvt_db4), (assign1020_e1292 * var_argbvvt_db5), (assign1020_e1292 * var_argbvvt_db6), (assign1020_e1292 * var_argbvvt_db7),)
                    } else {
                        let (assign1020_e1298, assign1020_e1298_d_n0, assign1020_e1298_d_n1, assign1020_e1298_d_n2, assign1020_e1298_d_n3, assign1020_e1298_d_n4, assign1020_e1298_d_n5, assign1020_e1298_d_n6, assign1020_e1298_d_n7, assign1020_e1298_d_n8, assign1020_e1298_d_n9, assign1020_e1298_d_b0, assign1020_e1298_d_b1, assign1020_e1298_d_b2, assign1020_e1298_d_b3, assign1020_e1298_d_b4, assign1020_e1298_d_b5, assign1020_e1298_d_b6, assign1020_e1298_d_b7,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn0, var_argbvvt_dn1, var_argbvvt_dn2, var_argbvvt_dn3, var_argbvvt_dn4, var_argbvvt_dn5, var_argbvvt_dn6, var_argbvvt_dn7, var_argbvvt_dn8, var_argbvvt_dn9, var_argbvvt_db0, var_argbvvt_db1, var_argbvvt_db2, var_argbvvt_db3, var_argbvvt_db4, var_argbvvt_db5, var_argbvvt_db6, var_argbvvt_db7,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign1020_e1298, assign1020_e1298_d_n0, assign1020_e1298_d_n1, assign1020_e1298_d_n2, assign1020_e1298_d_n3, assign1020_e1298_d_n4, assign1020_e1298_d_n5, assign1020_e1298_d_n6, assign1020_e1298_d_n7, assign1020_e1298_d_n8, assign1020_e1298_d_n9, assign1020_e1298_d_b0, assign1020_e1298_d_b1, assign1020_e1298_d_b2, assign1020_e1298_d_b3, assign1020_e1298_d_b4, assign1020_e1298_d_b5, assign1020_e1298_d_b6, assign1020_e1298_d_b7,)
                    }
                };
                (assign1020_e1299, assign1020_e1299_d_n0, assign1020_e1299_d_n1, assign1020_e1299_d_n2, assign1020_e1299_d_n3, assign1020_e1299_d_n4, assign1020_e1299_d_n5, assign1020_e1299_d_n6, assign1020_e1299_d_n7, assign1020_e1299_d_n8, assign1020_e1299_d_n9, assign1020_e1299_d_b0, assign1020_e1299_d_b1, assign1020_e1299_d_b2, assign1020_e1299_d_b3, assign1020_e1299_d_b4, assign1020_e1299_d_b5, assign1020_e1299_d_b6, assign1020_e1299_d_b7,)
            }
        };
        let assign1020_e1301: f64 = (assign1020_e1266 - assign1020_e1300);
        (assign1020_e1301, (assign1020_e1266_d_n0 - assign1020_e1300_d_n0), (assign1020_e1266_d_n1 - assign1020_e1300_d_n1), (assign1020_e1266_d_n2 - assign1020_e1300_d_n2), (assign1020_e1266_d_n3 - assign1020_e1300_d_n3), (assign1020_e1266_d_n4 - assign1020_e1300_d_n4), (assign1020_e1266_d_n5 - assign1020_e1300_d_n5), (assign1020_e1266_d_n6 - assign1020_e1300_d_n6), (assign1020_e1266_d_n7 - assign1020_e1300_d_n7), (assign1020_e1266_d_n8 - assign1020_e1300_d_n8), (assign1020_e1266_d_n9 - assign1020_e1300_d_n9), (assign1020_e1266_d_b0 - assign1020_e1300_d_b0), (assign1020_e1266_d_b1 - assign1020_e1300_d_b1), (assign1020_e1266_d_b2 - assign1020_e1300_d_b2), (assign1020_e1266_d_b3 - assign1020_e1300_d_b3), (assign1020_e1266_d_b4 - assign1020_e1300_d_b4), (assign1020_e1266_d_b5 - assign1020_e1300_d_b5), (assign1020_e1266_d_b6 - assign1020_e1300_d_b6), (assign1020_e1266_d_b7 - assign1020_e1300_d_b7),)
    } else {
        (var_lebv, var_lebv_dn0, var_lebv_dn1, var_lebv_dn2, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6, var_lebv_dn7, var_lebv_dn8, var_lebv_dn9, var_lebv_db0, var_lebv_db1, var_lebv_db2, var_lebv_db3, var_lebv_db4, var_lebv_db5, var_lebv_db6, var_lebv_db7,)
    }
};
        var_lebv = assign1020_e1303;
        var_lebv_dn0 = assign1020_e1303_d_n0;
        var_lebv_dn1 = assign1020_e1303_d_n1;
        var_lebv_dn2 = assign1020_e1303_d_n2;
        var_lebv_dn3 = assign1020_e1303_d_n3;
        var_lebv_dn4 = assign1020_e1303_d_n4;
        var_lebv_dn5 = assign1020_e1303_d_n5;
        var_lebv_dn6 = assign1020_e1303_d_n6;
        var_lebv_dn7 = assign1020_e1303_d_n7;
        var_lebv_dn8 = assign1020_e1303_d_n8;
        var_lebv_dn9 = assign1020_e1303_d_n9;
        var_lebv_db0 = assign1020_e1303_d_b0;
        var_lebv_db1 = assign1020_e1303_d_b1;
        var_lebv_db2 = assign1020_e1303_d_b2;
        var_lebv_db3 = assign1020_e1303_d_b3;
        var_lebv_db4 = assign1020_e1303_d_b4;
        var_lebv_db5 = assign1020_e1303_d_b5;
        var_lebv_db6 = assign1020_e1303_d_b6;
        var_lebv_db7 = assign1020_e1303_d_b7;
        var_lebv_rv = 0.0;
        var_lebv_rdn0 = 0.0;
        var_lebv_rdn1 = 0.0;
        var_lebv_rdn2 = 0.0;
        var_lebv_rdn3 = 0.0;
        var_lebv_rdn4 = 0.0;
        var_lebv_rdn5 = 0.0;
        var_lebv_rdn6 = 0.0;
        var_lebv_rdn7 = 0.0;
        var_lebv_rdn8 = 0.0;
        var_lebv_rdn9 = 0.0;
        var_lebv_rdb0 = 0.0;
        var_lebv_rdb1 = 0.0;
        var_lebv_rdb2 = 0.0;
        var_lebv_rdb3 = 0.0;
        var_lebv_rdb4 = 0.0;
        var_lebv_rdb5 = 0.0;
        var_lebv_rdb6 = 0.0;
        var_lebv_rdb7 = 0.0;

        let assign1050_e1332: f64 = if var_is_t > 0.0 { 1.0 } else { 0.0 };
        var_guard9 = assign1050_e1332;
        var_guard9_dn0 = 0.0;
        var_guard9_dn1 = 0.0;
        var_guard9_dn2 = 0.0;
        var_guard9_dn3 = 0.0;
        var_guard9_dn4 = 0.0;
        var_guard9_dn5 = 0.0;
        var_guard9_dn6 = 0.0;
        var_guard9_dn7 = 0.0;
        var_guard9_dn8 = 0.0;
        var_guard9_dn9 = 0.0;
        var_guard9_db0 = 0.0;
        var_guard9_db1 = 0.0;
        var_guard9_db2 = 0.0;
        var_guard9_db3 = 0.0;
        var_guard9_db4 = 0.0;
        var_guard9_db5 = 0.0;
        var_guard9_db6 = 0.0;
        var_guard9_db7 = 0.0;
        var_guard9_rv = 0.0;
        var_guard9_rdn0 = 0.0;
        var_guard9_rdn1 = 0.0;
        var_guard9_rdn2 = 0.0;
        var_guard9_rdn3 = 0.0;
        var_guard9_rdn4 = 0.0;
        var_guard9_rdn5 = 0.0;
        var_guard9_rdn6 = 0.0;
        var_guard9_rdn7 = 0.0;
        var_guard9_rdn8 = 0.0;
        var_guard9_rdn9 = 0.0;
        var_guard9_rdb0 = 0.0;
        var_guard9_rdb1 = 0.0;
        var_guard9_rdb2 = 0.0;
        var_guard9_rdb3 = 0.0;
        var_guard9_rdb4 = 0.0;
        var_guard9_rdb5 = 0.0;
        var_guard9_rdb6 = 0.0;
        var_guard9_rdb7 = 0.0;

        let (assign1060_e1340, assign1060_e1340_d_n0, assign1060_e1340_d_n1, assign1060_e1340_d_n2, assign1060_e1340_d_n3, assign1060_e1340_d_n4, assign1060_e1340_d_n5, assign1060_e1340_d_n6, assign1060_e1340_d_n7, assign1060_e1340_d_n8, assign1060_e1340_d_n9, assign1060_e1340_d_b0, assign1060_e1340_d_b1, assign1060_e1340_d_b2, assign1060_e1340_d_b3, assign1060_e1340_d_b4, assign1060_e1340_d_b5, assign1060_e1340_d_b6, assign1060_e1340_d_b7,) = {
    if (var_guard9 != 0.0) {
        let assign1060_e1337: f64 = (p.p61 * var_vt);
        let assign1060_e1338: f64 = (var_vbici / assign1060_e1337);
        (assign1060_e1338, (((var_vbici_dn0 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_dn0))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_dn1 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_dn1))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_dn2 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_dn2))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_dn3 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_dn3))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_dn4 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_dn4))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_dn5 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_dn5))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_dn6 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_dn6))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_dn7 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_dn7))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_dn8 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_dn8))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_dn9 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_dn9))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_db0 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_db0))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_db1 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_db1))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_db2 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_db2))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_db3 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_db3))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_db4 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_db4))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_db5 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_db5))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_db6 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_db6))) / (assign1060_e1337 * assign1060_e1337)), (((var_vbici_db7 * assign1060_e1337) - (var_vbici * (p.p61 * var_vt_db7))) / (assign1060_e1337 * assign1060_e1337)),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    }
};
        var_arg = assign1060_e1340;
        var_arg_dn0 = assign1060_e1340_d_n0;
        var_arg_dn1 = assign1060_e1340_d_n1;
        var_arg_dn2 = assign1060_e1340_d_n2;
        var_arg_dn3 = assign1060_e1340_d_n3;
        var_arg_dn4 = assign1060_e1340_d_n4;
        var_arg_dn5 = assign1060_e1340_d_n5;
        var_arg_dn6 = assign1060_e1340_d_n6;
        var_arg_dn7 = assign1060_e1340_d_n7;
        var_arg_dn8 = assign1060_e1340_d_n8;
        var_arg_dn9 = assign1060_e1340_d_n9;
        var_arg_db0 = assign1060_e1340_d_b0;
        var_arg_db1 = assign1060_e1340_d_b1;
        var_arg_db2 = assign1060_e1340_d_b2;
        var_arg_db3 = assign1060_e1340_d_b3;
        var_arg_db4 = assign1060_e1340_d_b4;
        var_arg_db5 = assign1060_e1340_d_b5;
        var_arg_db6 = assign1060_e1340_d_b6;
        var_arg_db7 = assign1060_e1340_d_b7;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdn7 = 0.0;
        var_arg_rdn8 = 0.0;
        var_arg_rdn9 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;
        var_arg_rdb7 = 0.0;

        let (assign1070_e1351, assign1070_e1351_d_n0, assign1070_e1351_d_n1, assign1070_e1351_d_n2, assign1070_e1351_d_n3, assign1070_e1351_d_n4, assign1070_e1351_d_n5, assign1070_e1351_d_n6, assign1070_e1351_d_n7, assign1070_e1351_d_n8, assign1070_e1351_d_n9, assign1070_e1351_d_b0, assign1070_e1351_d_b1, assign1070_e1351_d_b2, assign1070_e1351_d_b3, assign1070_e1351_d_b4, assign1070_e1351_d_b5, assign1070_e1351_d_b6, assign1070_e1351_d_b7,) = {
    if (var_guard9 != 0.0) {
        let assign1070_e1343: f64 = (-var_vbici);
        let assign1070_e1345: f64 = (assign1070_e1343 - var_bvr_t);
        let assign1070_e1348: f64 = (p.p57 * var_vt);
        let assign1070_e1349: f64 = (assign1070_e1345 / assign1070_e1348);
        (assign1070_e1349, (((((-var_vbici_dn0) - var_bvr_t_dn0) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_dn0))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_dn1) - var_bvr_t_dn1) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_dn1))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_dn2) - var_bvr_t_dn2) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_dn2))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_dn3) - var_bvr_t_dn3) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_dn3))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_dn4) - var_bvr_t_dn4) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_dn4))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_dn5) - var_bvr_t_dn5) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_dn5))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_dn6) - var_bvr_t_dn6) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_dn6))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_dn7) - var_bvr_t_dn7) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_dn7))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_dn8) - var_bvr_t_dn8) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_dn8))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_dn9) - var_bvr_t_dn9) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_dn9))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_db0) - var_bvr_t_db0) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_db0))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_db1) - var_bvr_t_db1) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_db1))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_db2) - var_bvr_t_db2) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_db2))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_db3) - var_bvr_t_db3) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_db3))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_db4) - var_bvr_t_db4) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_db4))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_db5) - var_bvr_t_db5) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_db5))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_db6) - var_bvr_t_db6) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_db6))) / (assign1070_e1348 * assign1070_e1348)), (((((-var_vbici_db7) - var_bvr_t_db7) * assign1070_e1348) - (assign1070_e1345 * (p.p57 * var_vt_db7))) / (assign1070_e1348 * assign1070_e1348)),)
    } else {
        (var_argbv, var_argbv_dn0, var_argbv_dn1, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6, var_argbv_dn7, var_argbv_dn8, var_argbv_dn9, var_argbv_db0, var_argbv_db1, var_argbv_db2, var_argbv_db3, var_argbv_db4, var_argbv_db5, var_argbv_db6, var_argbv_db7,)
    }
};
        var_argbv = assign1070_e1351;
        var_argbv_dn0 = assign1070_e1351_d_n0;
        var_argbv_dn1 = assign1070_e1351_d_n1;
        var_argbv_dn2 = assign1070_e1351_d_n2;
        var_argbv_dn3 = assign1070_e1351_d_n3;
        var_argbv_dn4 = assign1070_e1351_d_n4;
        var_argbv_dn5 = assign1070_e1351_d_n5;
        var_argbv_dn6 = assign1070_e1351_d_n6;
        var_argbv_dn7 = assign1070_e1351_d_n7;
        var_argbv_dn8 = assign1070_e1351_d_n8;
        var_argbv_dn9 = assign1070_e1351_d_n9;
        var_argbv_db0 = assign1070_e1351_d_b0;
        var_argbv_db1 = assign1070_e1351_d_b1;
        var_argbv_db2 = assign1070_e1351_d_b2;
        var_argbv_db3 = assign1070_e1351_d_b3;
        var_argbv_db4 = assign1070_e1351_d_b4;
        var_argbv_db5 = assign1070_e1351_d_b5;
        var_argbv_db6 = assign1070_e1351_d_b6;
        var_argbv_db7 = assign1070_e1351_d_b7;
        var_argbv_rv = 0.0;
        var_argbv_rdn0 = 0.0;
        var_argbv_rdn1 = 0.0;
        var_argbv_rdn2 = 0.0;
        var_argbv_rdn3 = 0.0;
        var_argbv_rdn4 = 0.0;
        var_argbv_rdn5 = 0.0;
        var_argbv_rdn6 = 0.0;
        var_argbv_rdn7 = 0.0;
        var_argbv_rdn8 = 0.0;
        var_argbv_rdn9 = 0.0;
        var_argbv_rdb0 = 0.0;
        var_argbv_rdb1 = 0.0;
        var_argbv_rdb2 = 0.0;
        var_argbv_rdb3 = 0.0;
        var_argbv_rdb4 = 0.0;
        var_argbv_rdb5 = 0.0;
        var_argbv_rdb6 = 0.0;
        var_argbv_rdb7 = 0.0;

        let (assign1080_e1360, assign1080_e1360_d_n0, assign1080_e1360_d_n1, assign1080_e1360_d_n2, assign1080_e1360_d_n3, assign1080_e1360_d_n4, assign1080_e1360_d_n5, assign1080_e1360_d_n6, assign1080_e1360_d_n7, assign1080_e1360_d_n8, assign1080_e1360_d_n9, assign1080_e1360_d_b0, assign1080_e1360_d_b1, assign1080_e1360_d_b2, assign1080_e1360_d_b3, assign1080_e1360_d_b4, assign1080_e1360_d_b5, assign1080_e1360_d_b6, assign1080_e1360_d_b7,) = {
    if (var_guard9 != 0.0) {
        let assign1080_e1354: f64 = (-var_bvr_t);
        let assign1080_e1357: f64 = (p.p57 * var_vt);
        let assign1080_e1358: f64 = (assign1080_e1354 / assign1080_e1357);
        (assign1080_e1358, ((((-var_bvr_t_dn0) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_dn0))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_dn1) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_dn1))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_dn2) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_dn2))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_dn3) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_dn3))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_dn4) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_dn4))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_dn5) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_dn5))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_dn6) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_dn6))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_dn7) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_dn7))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_dn8) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_dn8))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_dn9) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_dn9))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_db0) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_db0))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_db1) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_db1))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_db2) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_db2))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_db3) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_db3))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_db4) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_db4))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_db5) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_db5))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_db6) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_db6))) / (assign1080_e1357 * assign1080_e1357)), ((((-var_bvr_t_db7) * assign1080_e1357) - (assign1080_e1354 * (p.p57 * var_vt_db7))) / (assign1080_e1357 * assign1080_e1357)),)
    } else {
        (var_argbvvt, var_argbvvt_dn0, var_argbvvt_dn1, var_argbvvt_dn2, var_argbvvt_dn3, var_argbvvt_dn4, var_argbvvt_dn5, var_argbvvt_dn6, var_argbvvt_dn7, var_argbvvt_dn8, var_argbvvt_dn9, var_argbvvt_db0, var_argbvvt_db1, var_argbvvt_db2, var_argbvvt_db3, var_argbvvt_db4, var_argbvvt_db5, var_argbvvt_db6, var_argbvvt_db7,)
    }
};
        var_argbvvt = assign1080_e1360;
        var_argbvvt_dn0 = assign1080_e1360_d_n0;
        var_argbvvt_dn1 = assign1080_e1360_d_n1;
        var_argbvvt_dn2 = assign1080_e1360_d_n2;
        var_argbvvt_dn3 = assign1080_e1360_d_n3;
        var_argbvvt_dn4 = assign1080_e1360_d_n4;
        var_argbvvt_dn5 = assign1080_e1360_d_n5;
        var_argbvvt_dn6 = assign1080_e1360_d_n6;
        var_argbvvt_dn7 = assign1080_e1360_d_n7;
        var_argbvvt_dn8 = assign1080_e1360_d_n8;
        var_argbvvt_dn9 = assign1080_e1360_d_n9;
        var_argbvvt_db0 = assign1080_e1360_d_b0;
        var_argbvvt_db1 = assign1080_e1360_d_b1;
        var_argbvvt_db2 = assign1080_e1360_d_b2;
        var_argbvvt_db3 = assign1080_e1360_d_b3;
        var_argbvvt_db4 = assign1080_e1360_d_b4;
        var_argbvvt_db5 = assign1080_e1360_d_b5;
        var_argbvvt_db6 = assign1080_e1360_d_b6;
        var_argbvvt_db7 = assign1080_e1360_d_b7;
        var_argbvvt_rv = 0.0;
        var_argbvvt_rdn0 = 0.0;
        var_argbvvt_rdn1 = 0.0;
        var_argbvvt_rdn2 = 0.0;
        var_argbvvt_rdn3 = 0.0;
        var_argbvvt_rdn4 = 0.0;
        var_argbvvt_rdn5 = 0.0;
        var_argbvvt_rdn6 = 0.0;
        var_argbvvt_rdn7 = 0.0;
        var_argbvvt_rdn8 = 0.0;
        var_argbvvt_rdn9 = 0.0;
        var_argbvvt_rdb0 = 0.0;
        var_argbvvt_rdb1 = 0.0;
        var_argbvvt_rdb2 = 0.0;
        var_argbvvt_rdb3 = 0.0;
        var_argbvvt_rdb4 = 0.0;
        var_argbvvt_rdb5 = 0.0;
        var_argbvvt_rdb6 = 0.0;
        var_argbvvt_rdb7 = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_db0_slot = var_arg_db0;
        *var_arg_db1_slot = var_arg_db1;
        *var_arg_db2_slot = var_arg_db2;
        *var_arg_db3_slot = var_arg_db3;
        *var_arg_db4_slot = var_arg_db4;
        *var_arg_db5_slot = var_arg_db5;
        *var_arg_db6_slot = var_arg_db6;
        *var_arg_db7_slot = var_arg_db7;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn1_slot = var_arg_dn1;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_arg_rdb0_slot = var_arg_rdb0;
        *var_arg_rdb1_slot = var_arg_rdb1;
        *var_arg_rdb2_slot = var_arg_rdb2;
        *var_arg_rdb3_slot = var_arg_rdb3;
        *var_arg_rdb4_slot = var_arg_rdb4;
        *var_arg_rdb5_slot = var_arg_rdb5;
        *var_arg_rdb6_slot = var_arg_rdb6;
        *var_arg_rdb7_slot = var_arg_rdb7;
        *var_arg_rdn0_slot = var_arg_rdn0;
        *var_arg_rdn1_slot = var_arg_rdn1;
        *var_arg_rdn2_slot = var_arg_rdn2;
        *var_arg_rdn3_slot = var_arg_rdn3;
        *var_arg_rdn4_slot = var_arg_rdn4;
        *var_arg_rdn5_slot = var_arg_rdn5;
        *var_arg_rdn6_slot = var_arg_rdn6;
        *var_arg_rdn7_slot = var_arg_rdn7;
        *var_arg_rdn8_slot = var_arg_rdn8;
        *var_arg_rdn9_slot = var_arg_rdn9;
        *var_arg_rv_slot = var_arg_rv;
        *var_argbv_slot = var_argbv;
        *var_argbv_db0_slot = var_argbv_db0;
        *var_argbv_db1_slot = var_argbv_db1;
        *var_argbv_db2_slot = var_argbv_db2;
        *var_argbv_db3_slot = var_argbv_db3;
        *var_argbv_db4_slot = var_argbv_db4;
        *var_argbv_db5_slot = var_argbv_db5;
        *var_argbv_db6_slot = var_argbv_db6;
        *var_argbv_db7_slot = var_argbv_db7;
        *var_argbv_dn0_slot = var_argbv_dn0;
        *var_argbv_dn1_slot = var_argbv_dn1;
        *var_argbv_dn2_slot = var_argbv_dn2;
        *var_argbv_dn3_slot = var_argbv_dn3;
        *var_argbv_dn4_slot = var_argbv_dn4;
        *var_argbv_dn5_slot = var_argbv_dn5;
        *var_argbv_dn6_slot = var_argbv_dn6;
        *var_argbv_dn7_slot = var_argbv_dn7;
        *var_argbv_dn8_slot = var_argbv_dn8;
        *var_argbv_dn9_slot = var_argbv_dn9;
        *var_argbv_rdb0_slot = var_argbv_rdb0;
        *var_argbv_rdb1_slot = var_argbv_rdb1;
        *var_argbv_rdb2_slot = var_argbv_rdb2;
        *var_argbv_rdb3_slot = var_argbv_rdb3;
        *var_argbv_rdb4_slot = var_argbv_rdb4;
        *var_argbv_rdb5_slot = var_argbv_rdb5;
        *var_argbv_rdb6_slot = var_argbv_rdb6;
        *var_argbv_rdb7_slot = var_argbv_rdb7;
        *var_argbv_rdn0_slot = var_argbv_rdn0;
        *var_argbv_rdn1_slot = var_argbv_rdn1;
        *var_argbv_rdn2_slot = var_argbv_rdn2;
        *var_argbv_rdn3_slot = var_argbv_rdn3;
        *var_argbv_rdn4_slot = var_argbv_rdn4;
        *var_argbv_rdn5_slot = var_argbv_rdn5;
        *var_argbv_rdn6_slot = var_argbv_rdn6;
        *var_argbv_rdn7_slot = var_argbv_rdn7;
        *var_argbv_rdn8_slot = var_argbv_rdn8;
        *var_argbv_rdn9_slot = var_argbv_rdn9;
        *var_argbv_rv_slot = var_argbv_rv;
        *var_argbvvt_slot = var_argbvvt;
        *var_argbvvt_db0_slot = var_argbvvt_db0;
        *var_argbvvt_db1_slot = var_argbvvt_db1;
        *var_argbvvt_db2_slot = var_argbvvt_db2;
        *var_argbvvt_db3_slot = var_argbvvt_db3;
        *var_argbvvt_db4_slot = var_argbvvt_db4;
        *var_argbvvt_db5_slot = var_argbvvt_db5;
        *var_argbvvt_db6_slot = var_argbvvt_db6;
        *var_argbvvt_db7_slot = var_argbvvt_db7;
        *var_argbvvt_dn0_slot = var_argbvvt_dn0;
        *var_argbvvt_dn1_slot = var_argbvvt_dn1;
        *var_argbvvt_dn2_slot = var_argbvvt_dn2;
        *var_argbvvt_dn3_slot = var_argbvvt_dn3;
        *var_argbvvt_dn4_slot = var_argbvvt_dn4;
        *var_argbvvt_dn5_slot = var_argbvvt_dn5;
        *var_argbvvt_dn6_slot = var_argbvvt_dn6;
        *var_argbvvt_dn7_slot = var_argbvvt_dn7;
        *var_argbvvt_dn8_slot = var_argbvvt_dn8;
        *var_argbvvt_dn9_slot = var_argbvvt_dn9;
        *var_argbvvt_rdb0_slot = var_argbvvt_rdb0;
        *var_argbvvt_rdb1_slot = var_argbvvt_rdb1;
        *var_argbvvt_rdb2_slot = var_argbvvt_rdb2;
        *var_argbvvt_rdb3_slot = var_argbvvt_rdb3;
        *var_argbvvt_rdb4_slot = var_argbvvt_rdb4;
        *var_argbvvt_rdb5_slot = var_argbvvt_rdb5;
        *var_argbvvt_rdb6_slot = var_argbvvt_rdb6;
        *var_argbvvt_rdb7_slot = var_argbvvt_rdb7;
        *var_argbvvt_rdn0_slot = var_argbvvt_rdn0;
        *var_argbvvt_rdn1_slot = var_argbvvt_rdn1;
        *var_argbvvt_rdn2_slot = var_argbvvt_rdn2;
        *var_argbvvt_rdn3_slot = var_argbvvt_rdn3;
        *var_argbvvt_rdn4_slot = var_argbvvt_rdn4;
        *var_argbvvt_rdn5_slot = var_argbvvt_rdn5;
        *var_argbvvt_rdn6_slot = var_argbvvt_rdn6;
        *var_argbvvt_rdn7_slot = var_argbvvt_rdn7;
        *var_argbvvt_rdn8_slot = var_argbvvt_rdn8;
        *var_argbvvt_rdn9_slot = var_argbvvt_rdn9;
        *var_argbvvt_rv_slot = var_argbvvt_rv;
        *var_guard9_slot = var_guard9;
        *var_guard9_db0_slot = var_guard9_db0;
        *var_guard9_db1_slot = var_guard9_db1;
        *var_guard9_db2_slot = var_guard9_db2;
        *var_guard9_db3_slot = var_guard9_db3;
        *var_guard9_db4_slot = var_guard9_db4;
        *var_guard9_db5_slot = var_guard9_db5;
        *var_guard9_db6_slot = var_guard9_db6;
        *var_guard9_db7_slot = var_guard9_db7;
        *var_guard9_dn0_slot = var_guard9_dn0;
        *var_guard9_dn1_slot = var_guard9_dn1;
        *var_guard9_dn2_slot = var_guard9_dn2;
        *var_guard9_dn3_slot = var_guard9_dn3;
        *var_guard9_dn4_slot = var_guard9_dn4;
        *var_guard9_dn5_slot = var_guard9_dn5;
        *var_guard9_dn6_slot = var_guard9_dn6;
        *var_guard9_dn7_slot = var_guard9_dn7;
        *var_guard9_dn8_slot = var_guard9_dn8;
        *var_guard9_dn9_slot = var_guard9_dn9;
        *var_guard9_rdb0_slot = var_guard9_rdb0;
        *var_guard9_rdb1_slot = var_guard9_rdb1;
        *var_guard9_rdb2_slot = var_guard9_rdb2;
        *var_guard9_rdb3_slot = var_guard9_rdb3;
        *var_guard9_rdb4_slot = var_guard9_rdb4;
        *var_guard9_rdb5_slot = var_guard9_rdb5;
        *var_guard9_rdb6_slot = var_guard9_rdb6;
        *var_guard9_rdb7_slot = var_guard9_rdb7;
        *var_guard9_rdn0_slot = var_guard9_rdn0;
        *var_guard9_rdn1_slot = var_guard9_rdn1;
        *var_guard9_rdn2_slot = var_guard9_rdn2;
        *var_guard9_rdn3_slot = var_guard9_rdn3;
        *var_guard9_rdn4_slot = var_guard9_rdn4;
        *var_guard9_rdn5_slot = var_guard9_rdn5;
        *var_guard9_rdn6_slot = var_guard9_rdn6;
        *var_guard9_rdn7_slot = var_guard9_rdn7;
        *var_guard9_rdn8_slot = var_guard9_rdn8;
        *var_guard9_rdn9_slot = var_guard9_rdn9;
        *var_guard9_rv_slot = var_guard9_rv;
        *var_le_slot = var_le;
        *var_le_db0_slot = var_le_db0;
        *var_le_db1_slot = var_le_db1;
        *var_le_db2_slot = var_le_db2;
        *var_le_db3_slot = var_le_db3;
        *var_le_db4_slot = var_le_db4;
        *var_le_db5_slot = var_le_db5;
        *var_le_db6_slot = var_le_db6;
        *var_le_db7_slot = var_le_db7;
        *var_le_dn0_slot = var_le_dn0;
        *var_le_dn1_slot = var_le_dn1;
        *var_le_dn2_slot = var_le_dn2;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn5_slot = var_le_dn5;
        *var_le_dn6_slot = var_le_dn6;
        *var_le_dn7_slot = var_le_dn7;
        *var_le_dn8_slot = var_le_dn8;
        *var_le_dn9_slot = var_le_dn9;
        *var_le_rdb0_slot = var_le_rdb0;
        *var_le_rdb1_slot = var_le_rdb1;
        *var_le_rdb2_slot = var_le_rdb2;
        *var_le_rdb3_slot = var_le_rdb3;
        *var_le_rdb4_slot = var_le_rdb4;
        *var_le_rdb5_slot = var_le_rdb5;
        *var_le_rdb6_slot = var_le_rdb6;
        *var_le_rdb7_slot = var_le_rdb7;
        *var_le_rdn0_slot = var_le_rdn0;
        *var_le_rdn1_slot = var_le_rdn1;
        *var_le_rdn2_slot = var_le_rdn2;
        *var_le_rdn3_slot = var_le_rdn3;
        *var_le_rdn4_slot = var_le_rdn4;
        *var_le_rdn5_slot = var_le_rdn5;
        *var_le_rdn6_slot = var_le_rdn6;
        *var_le_rdn7_slot = var_le_rdn7;
        *var_le_rdn8_slot = var_le_rdn8;
        *var_le_rdn9_slot = var_le_rdn9;
        *var_le_rv_slot = var_le_rv;
        *var_lebv_slot = var_lebv;
        *var_lebv_db0_slot = var_lebv_db0;
        *var_lebv_db1_slot = var_lebv_db1;
        *var_lebv_db2_slot = var_lebv_db2;
        *var_lebv_db3_slot = var_lebv_db3;
        *var_lebv_db4_slot = var_lebv_db4;
        *var_lebv_db5_slot = var_lebv_db5;
        *var_lebv_db6_slot = var_lebv_db6;
        *var_lebv_db7_slot = var_lebv_db7;
        *var_lebv_dn0_slot = var_lebv_dn0;
        *var_lebv_dn1_slot = var_lebv_dn1;
        *var_lebv_dn2_slot = var_lebv_dn2;
        *var_lebv_dn3_slot = var_lebv_dn3;
        *var_lebv_dn4_slot = var_lebv_dn4;
        *var_lebv_dn5_slot = var_lebv_dn5;
        *var_lebv_dn6_slot = var_lebv_dn6;
        *var_lebv_dn7_slot = var_lebv_dn7;
        *var_lebv_dn8_slot = var_lebv_dn8;
        *var_lebv_dn9_slot = var_lebv_dn9;
        *var_lebv_rdb0_slot = var_lebv_rdb0;
        *var_lebv_rdb1_slot = var_lebv_rdb1;
        *var_lebv_rdb2_slot = var_lebv_rdb2;
        *var_lebv_rdb3_slot = var_lebv_rdb3;
        *var_lebv_rdb4_slot = var_lebv_rdb4;
        *var_lebv_rdb5_slot = var_lebv_rdb5;
        *var_lebv_rdb6_slot = var_lebv_rdb6;
        *var_lebv_rdb7_slot = var_lebv_rdb7;
        *var_lebv_rdn0_slot = var_lebv_rdn0;
        *var_lebv_rdn1_slot = var_lebv_rdn1;
        *var_lebv_rdn2_slot = var_lebv_rdn2;
        *var_lebv_rdn3_slot = var_lebv_rdn3;
        *var_lebv_rdn4_slot = var_lebv_rdn4;
        *var_lebv_rdn5_slot = var_lebv_rdn5;
        *var_lebv_rdn6_slot = var_lebv_rdn6;
        *var_lebv_rdn7_slot = var_lebv_rdn7;
        *var_lebv_rdn8_slot = var_lebv_rdn8;
        *var_lebv_rdn9_slot = var_lebv_rdn9;
        *var_lebv_rv_slot = var_lebv_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        var_argbv: f64,
        var_argbv_db0: f64,
        var_argbv_db1: f64,
        var_argbv_db2: f64,
        var_argbv_db3: f64,
        var_argbv_db4: f64,
        var_argbv_db5: f64,
        var_argbv_db6: f64,
        var_argbv_db7: f64,
        var_argbv_dn0: f64,
        var_argbv_dn1: f64,
        var_argbv_dn2: f64,
        var_argbv_dn3: f64,
        var_argbv_dn4: f64,
        var_argbv_dn5: f64,
        var_argbv_dn6: f64,
        var_argbv_dn7: f64,
        var_argbv_dn8: f64,
        var_argbv_dn9: f64,
        var_argbvvt: f64,
        var_argbvvt_db0: f64,
        var_argbvvt_db1: f64,
        var_argbvvt_db2: f64,
        var_argbvvt_db3: f64,
        var_argbvvt_db4: f64,
        var_argbvvt_db5: f64,
        var_argbvvt_db6: f64,
        var_argbvvt_db7: f64,
        var_argbvvt_dn0: f64,
        var_argbvvt_dn1: f64,
        var_argbvvt_dn2: f64,
        var_argbvvt_dn3: f64,
        var_argbvvt_dn4: f64,
        var_argbvvt_dn5: f64,
        var_argbvvt_dn6: f64,
        var_argbvvt_dn7: f64,
        var_argbvvt_dn8: f64,
        var_argbvvt_dn9: f64,
        var_guard9: f64,
        var_ijbvc_t: f64,
        var_ijbvc_t_db0: f64,
        var_ijbvc_t_db1: f64,
        var_ijbvc_t_db2: f64,
        var_ijbvc_t_db3: f64,
        var_ijbvc_t_db4: f64,
        var_ijbvc_t_db5: f64,
        var_ijbvc_t_db6: f64,
        var_ijbvc_t_db7: f64,
        var_ijbvc_t_dn0: f64,
        var_ijbvc_t_dn1: f64,
        var_ijbvc_t_dn2: f64,
        var_ijbvc_t_dn3: f64,
        var_ijbvc_t_dn4: f64,
        var_ijbvc_t_dn5: f64,
        var_ijbvc_t_dn6: f64,
        var_ijbvc_t_dn7: f64,
        var_ijbvc_t_dn8: f64,
        var_ijbvc_t_dn9: f64,
        var_is_t: f64,
        var_is_t_db0: f64,
        var_is_t_db1: f64,
        var_is_t_db2: f64,
        var_is_t_db3: f64,
        var_is_t_db4: f64,
        var_is_t_db5: f64,
        var_is_t_db6: f64,
        var_is_t_db7: f64,
        var_is_t_dn0: f64,
        var_is_t_dn1: f64,
        var_is_t_dn2: f64,
        var_is_t_dn3: f64,
        var_is_t_dn4: f64,
        var_is_t_dn5: f64,
        var_is_t_dn6: f64,
        var_is_t_dn7: f64,
        var_is_t_dn8: f64,
        var_is_t_dn9: f64,
        var_isc_t: f64,
        var_theexp_t: f64,
        var_theexp_t_db0: f64,
        var_theexp_t_db1: f64,
        var_theexp_t_db2: f64,
        var_theexp_t_db3: f64,
        var_theexp_t_db4: f64,
        var_theexp_t_db5: f64,
        var_theexp_t_db6: f64,
        var_theexp_t_db7: f64,
        var_theexp_t_dn0: f64,
        var_theexp_t_dn1: f64,
        var_theexp_t_dn2: f64,
        var_theexp_t_dn3: f64,
        var_theexp_t_dn4: f64,
        var_theexp_t_dn5: f64,
        var_theexp_t_dn6: f64,
        var_theexp_t_dn7: f64,
        var_theexp_t_dn8: f64,
        var_theexp_t_dn9: f64,
        var_vbici: f64,
        var_vbici_db0: f64,
        var_vbici_db1: f64,
        var_vbici_db2: f64,
        var_vbici_db3: f64,
        var_vbici_db4: f64,
        var_vbici_db5: f64,
        var_vbici_db6: f64,
        var_vbici_db7: f64,
        var_vbici_dn0: f64,
        var_vbici_dn1: f64,
        var_vbici_dn2: f64,
        var_vbici_dn3: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbici_dn6: f64,
        var_vbici_dn7: f64,
        var_vbici_dn8: f64,
        var_vbici_dn9: f64,
        var_arg_slot: &mut f64,
        var_arg_db0_slot: &mut f64,
        var_arg_db1_slot: &mut f64,
        var_arg_db2_slot: &mut f64,
        var_arg_db3_slot: &mut f64,
        var_arg_db4_slot: &mut f64,
        var_arg_db5_slot: &mut f64,
        var_arg_db6_slot: &mut f64,
        var_arg_db7_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn1_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_arg_rdb0_slot: &mut f64,
        var_arg_rdb1_slot: &mut f64,
        var_arg_rdb2_slot: &mut f64,
        var_arg_rdb3_slot: &mut f64,
        var_arg_rdb4_slot: &mut f64,
        var_arg_rdb5_slot: &mut f64,
        var_arg_rdb6_slot: &mut f64,
        var_arg_rdb7_slot: &mut f64,
        var_arg_rdn0_slot: &mut f64,
        var_arg_rdn1_slot: &mut f64,
        var_arg_rdn2_slot: &mut f64,
        var_arg_rdn3_slot: &mut f64,
        var_arg_rdn4_slot: &mut f64,
        var_arg_rdn5_slot: &mut f64,
        var_arg_rdn6_slot: &mut f64,
        var_arg_rdn7_slot: &mut f64,
        var_arg_rdn8_slot: &mut f64,
        var_arg_rdn9_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_guard10_slot: &mut f64,
        var_guard10_db0_slot: &mut f64,
        var_guard10_db1_slot: &mut f64,
        var_guard10_db2_slot: &mut f64,
        var_guard10_db3_slot: &mut f64,
        var_guard10_db4_slot: &mut f64,
        var_guard10_db5_slot: &mut f64,
        var_guard10_db6_slot: &mut f64,
        var_guard10_db7_slot: &mut f64,
        var_guard10_dn0_slot: &mut f64,
        var_guard10_dn1_slot: &mut f64,
        var_guard10_dn2_slot: &mut f64,
        var_guard10_dn3_slot: &mut f64,
        var_guard10_dn4_slot: &mut f64,
        var_guard10_dn5_slot: &mut f64,
        var_guard10_dn6_slot: &mut f64,
        var_guard10_dn7_slot: &mut f64,
        var_guard10_dn8_slot: &mut f64,
        var_guard10_dn9_slot: &mut f64,
        var_guard10_rdb0_slot: &mut f64,
        var_guard10_rdb1_slot: &mut f64,
        var_guard10_rdb2_slot: &mut f64,
        var_guard10_rdb3_slot: &mut f64,
        var_guard10_rdb4_slot: &mut f64,
        var_guard10_rdb5_slot: &mut f64,
        var_guard10_rdb6_slot: &mut f64,
        var_guard10_rdb7_slot: &mut f64,
        var_guard10_rdn0_slot: &mut f64,
        var_guard10_rdn1_slot: &mut f64,
        var_guard10_rdn2_slot: &mut f64,
        var_guard10_rdn3_slot: &mut f64,
        var_guard10_rdn4_slot: &mut f64,
        var_guard10_rdn5_slot: &mut f64,
        var_guard10_rdn6_slot: &mut f64,
        var_guard10_rdn7_slot: &mut f64,
        var_guard10_rdn8_slot: &mut f64,
        var_guard10_rdn9_slot: &mut f64,
        var_guard10_rv_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard11_db0_slot: &mut f64,
        var_guard11_db1_slot: &mut f64,
        var_guard11_db2_slot: &mut f64,
        var_guard11_db3_slot: &mut f64,
        var_guard11_db4_slot: &mut f64,
        var_guard11_db5_slot: &mut f64,
        var_guard11_db6_slot: &mut f64,
        var_guard11_db7_slot: &mut f64,
        var_guard11_dn0_slot: &mut f64,
        var_guard11_dn1_slot: &mut f64,
        var_guard11_dn2_slot: &mut f64,
        var_guard11_dn3_slot: &mut f64,
        var_guard11_dn4_slot: &mut f64,
        var_guard11_dn5_slot: &mut f64,
        var_guard11_dn6_slot: &mut f64,
        var_guard11_dn7_slot: &mut f64,
        var_guard11_dn8_slot: &mut f64,
        var_guard11_dn9_slot: &mut f64,
        var_guard11_rdb0_slot: &mut f64,
        var_guard11_rdb1_slot: &mut f64,
        var_guard11_rdb2_slot: &mut f64,
        var_guard11_rdb3_slot: &mut f64,
        var_guard11_rdb4_slot: &mut f64,
        var_guard11_rdb5_slot: &mut f64,
        var_guard11_rdb6_slot: &mut f64,
        var_guard11_rdb7_slot: &mut f64,
        var_guard11_rdn0_slot: &mut f64,
        var_guard11_rdn1_slot: &mut f64,
        var_guard11_rdn2_slot: &mut f64,
        var_guard11_rdn3_slot: &mut f64,
        var_guard11_rdn4_slot: &mut f64,
        var_guard11_rdn5_slot: &mut f64,
        var_guard11_rdn6_slot: &mut f64,
        var_guard11_rdn7_slot: &mut f64,
        var_guard11_rdn8_slot: &mut f64,
        var_guard11_rdn9_slot: &mut f64,
        var_guard11_rv_slot: &mut f64,
        var_ibwd_slot: &mut f64,
        var_ibwd_db0_slot: &mut f64,
        var_ibwd_db1_slot: &mut f64,
        var_ibwd_db2_slot: &mut f64,
        var_ibwd_db3_slot: &mut f64,
        var_ibwd_db4_slot: &mut f64,
        var_ibwd_db5_slot: &mut f64,
        var_ibwd_db6_slot: &mut f64,
        var_ibwd_db7_slot: &mut f64,
        var_ibwd_dn0_slot: &mut f64,
        var_ibwd_dn1_slot: &mut f64,
        var_ibwd_dn2_slot: &mut f64,
        var_ibwd_dn3_slot: &mut f64,
        var_ibwd_dn4_slot: &mut f64,
        var_ibwd_dn5_slot: &mut f64,
        var_ibwd_dn6_slot: &mut f64,
        var_ibwd_dn7_slot: &mut f64,
        var_ibwd_dn8_slot: &mut f64,
        var_ibwd_dn9_slot: &mut f64,
        var_ibwd_rdb0_slot: &mut f64,
        var_ibwd_rdb1_slot: &mut f64,
        var_ibwd_rdb2_slot: &mut f64,
        var_ibwd_rdb3_slot: &mut f64,
        var_ibwd_rdb4_slot: &mut f64,
        var_ibwd_rdb5_slot: &mut f64,
        var_ibwd_rdb6_slot: &mut f64,
        var_ibwd_rdb7_slot: &mut f64,
        var_ibwd_rdn0_slot: &mut f64,
        var_ibwd_rdn1_slot: &mut f64,
        var_ibwd_rdn2_slot: &mut f64,
        var_ibwd_rdn3_slot: &mut f64,
        var_ibwd_rdn4_slot: &mut f64,
        var_ibwd_rdn5_slot: &mut f64,
        var_ibwd_rdn6_slot: &mut f64,
        var_ibwd_rdn7_slot: &mut f64,
        var_ibwd_rdn8_slot: &mut f64,
        var_ibwd_rdn9_slot: &mut f64,
        var_ibwd_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_db0_slot: &mut f64,
        var_le_db1_slot: &mut f64,
        var_le_db2_slot: &mut f64,
        var_le_db3_slot: &mut f64,
        var_le_db4_slot: &mut f64,
        var_le_db5_slot: &mut f64,
        var_le_db6_slot: &mut f64,
        var_le_db7_slot: &mut f64,
        var_le_dn0_slot: &mut f64,
        var_le_dn1_slot: &mut f64,
        var_le_dn2_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn5_slot: &mut f64,
        var_le_dn6_slot: &mut f64,
        var_le_dn7_slot: &mut f64,
        var_le_dn8_slot: &mut f64,
        var_le_dn9_slot: &mut f64,
        var_le_rdb0_slot: &mut f64,
        var_le_rdb1_slot: &mut f64,
        var_le_rdb2_slot: &mut f64,
        var_le_rdb3_slot: &mut f64,
        var_le_rdb4_slot: &mut f64,
        var_le_rdb5_slot: &mut f64,
        var_le_rdb6_slot: &mut f64,
        var_le_rdb7_slot: &mut f64,
        var_le_rdn0_slot: &mut f64,
        var_le_rdn1_slot: &mut f64,
        var_le_rdn2_slot: &mut f64,
        var_le_rdn3_slot: &mut f64,
        var_le_rdn4_slot: &mut f64,
        var_le_rdn5_slot: &mut f64,
        var_le_rdn6_slot: &mut f64,
        var_le_rdn7_slot: &mut f64,
        var_le_rdn8_slot: &mut f64,
        var_le_rdn9_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_lebv_slot: &mut f64,
        var_lebv_db0_slot: &mut f64,
        var_lebv_db1_slot: &mut f64,
        var_lebv_db2_slot: &mut f64,
        var_lebv_db3_slot: &mut f64,
        var_lebv_db4_slot: &mut f64,
        var_lebv_db5_slot: &mut f64,
        var_lebv_db6_slot: &mut f64,
        var_lebv_db7_slot: &mut f64,
        var_lebv_dn0_slot: &mut f64,
        var_lebv_dn1_slot: &mut f64,
        var_lebv_dn2_slot: &mut f64,
        var_lebv_dn3_slot: &mut f64,
        var_lebv_dn4_slot: &mut f64,
        var_lebv_dn5_slot: &mut f64,
        var_lebv_dn6_slot: &mut f64,
        var_lebv_dn7_slot: &mut f64,
        var_lebv_dn8_slot: &mut f64,
        var_lebv_dn9_slot: &mut f64,
        var_lebv_rdb0_slot: &mut f64,
        var_lebv_rdb1_slot: &mut f64,
        var_lebv_rdb2_slot: &mut f64,
        var_lebv_rdb3_slot: &mut f64,
        var_lebv_rdb4_slot: &mut f64,
        var_lebv_rdb5_slot: &mut f64,
        var_lebv_rdb6_slot: &mut f64,
        var_lebv_rdb7_slot: &mut f64,
        var_lebv_rdn0_slot: &mut f64,
        var_lebv_rdn1_slot: &mut f64,
        var_lebv_rdn2_slot: &mut f64,
        var_lebv_rdn3_slot: &mut f64,
        var_lebv_rdn4_slot: &mut f64,
        var_lebv_rdn5_slot: &mut f64,
        var_lebv_rdn6_slot: &mut f64,
        var_lebv_rdn7_slot: &mut f64,
        var_lebv_rdn8_slot: &mut f64,
        var_lebv_rdn9_slot: &mut f64,
        var_lebv_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_db0: f64 = *var_arg_db0_slot;
        let mut var_arg_db1: f64 = *var_arg_db1_slot;
        let mut var_arg_db2: f64 = *var_arg_db2_slot;
        let mut var_arg_db3: f64 = *var_arg_db3_slot;
        let mut var_arg_db4: f64 = *var_arg_db4_slot;
        let mut var_arg_db5: f64 = *var_arg_db5_slot;
        let mut var_arg_db6: f64 = *var_arg_db6_slot;
        let mut var_arg_db7: f64 = *var_arg_db7_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn1: f64 = *var_arg_dn1_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_arg_rdb0: f64 = *var_arg_rdb0_slot;
        let mut var_arg_rdb1: f64 = *var_arg_rdb1_slot;
        let mut var_arg_rdb2: f64 = *var_arg_rdb2_slot;
        let mut var_arg_rdb3: f64 = *var_arg_rdb3_slot;
        let mut var_arg_rdb4: f64 = *var_arg_rdb4_slot;
        let mut var_arg_rdb5: f64 = *var_arg_rdb5_slot;
        let mut var_arg_rdb6: f64 = *var_arg_rdb6_slot;
        let mut var_arg_rdb7: f64 = *var_arg_rdb7_slot;
        let mut var_arg_rdn0: f64 = *var_arg_rdn0_slot;
        let mut var_arg_rdn1: f64 = *var_arg_rdn1_slot;
        let mut var_arg_rdn2: f64 = *var_arg_rdn2_slot;
        let mut var_arg_rdn3: f64 = *var_arg_rdn3_slot;
        let mut var_arg_rdn4: f64 = *var_arg_rdn4_slot;
        let mut var_arg_rdn5: f64 = *var_arg_rdn5_slot;
        let mut var_arg_rdn6: f64 = *var_arg_rdn6_slot;
        let mut var_arg_rdn7: f64 = *var_arg_rdn7_slot;
        let mut var_arg_rdn8: f64 = *var_arg_rdn8_slot;
        let mut var_arg_rdn9: f64 = *var_arg_rdn9_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard10_db0: f64 = *var_guard10_db0_slot;
        let mut var_guard10_db1: f64 = *var_guard10_db1_slot;
        let mut var_guard10_db2: f64 = *var_guard10_db2_slot;
        let mut var_guard10_db3: f64 = *var_guard10_db3_slot;
        let mut var_guard10_db4: f64 = *var_guard10_db4_slot;
        let mut var_guard10_db5: f64 = *var_guard10_db5_slot;
        let mut var_guard10_db6: f64 = *var_guard10_db6_slot;
        let mut var_guard10_db7: f64 = *var_guard10_db7_slot;
        let mut var_guard10_dn0: f64 = *var_guard10_dn0_slot;
        let mut var_guard10_dn1: f64 = *var_guard10_dn1_slot;
        let mut var_guard10_dn2: f64 = *var_guard10_dn2_slot;
        let mut var_guard10_dn3: f64 = *var_guard10_dn3_slot;
        let mut var_guard10_dn4: f64 = *var_guard10_dn4_slot;
        let mut var_guard10_dn5: f64 = *var_guard10_dn5_slot;
        let mut var_guard10_dn6: f64 = *var_guard10_dn6_slot;
        let mut var_guard10_dn7: f64 = *var_guard10_dn7_slot;
        let mut var_guard10_dn8: f64 = *var_guard10_dn8_slot;
        let mut var_guard10_dn9: f64 = *var_guard10_dn9_slot;
        let mut var_guard10_rdb0: f64 = *var_guard10_rdb0_slot;
        let mut var_guard10_rdb1: f64 = *var_guard10_rdb1_slot;
        let mut var_guard10_rdb2: f64 = *var_guard10_rdb2_slot;
        let mut var_guard10_rdb3: f64 = *var_guard10_rdb3_slot;
        let mut var_guard10_rdb4: f64 = *var_guard10_rdb4_slot;
        let mut var_guard10_rdb5: f64 = *var_guard10_rdb5_slot;
        let mut var_guard10_rdb6: f64 = *var_guard10_rdb6_slot;
        let mut var_guard10_rdb7: f64 = *var_guard10_rdb7_slot;
        let mut var_guard10_rdn0: f64 = *var_guard10_rdn0_slot;
        let mut var_guard10_rdn1: f64 = *var_guard10_rdn1_slot;
        let mut var_guard10_rdn2: f64 = *var_guard10_rdn2_slot;
        let mut var_guard10_rdn3: f64 = *var_guard10_rdn3_slot;
        let mut var_guard10_rdn4: f64 = *var_guard10_rdn4_slot;
        let mut var_guard10_rdn5: f64 = *var_guard10_rdn5_slot;
        let mut var_guard10_rdn6: f64 = *var_guard10_rdn6_slot;
        let mut var_guard10_rdn7: f64 = *var_guard10_rdn7_slot;
        let mut var_guard10_rdn8: f64 = *var_guard10_rdn8_slot;
        let mut var_guard10_rdn9: f64 = *var_guard10_rdn9_slot;
        let mut var_guard10_rv: f64 = *var_guard10_rv_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard11_db0: f64 = *var_guard11_db0_slot;
        let mut var_guard11_db1: f64 = *var_guard11_db1_slot;
        let mut var_guard11_db2: f64 = *var_guard11_db2_slot;
        let mut var_guard11_db3: f64 = *var_guard11_db3_slot;
        let mut var_guard11_db4: f64 = *var_guard11_db4_slot;
        let mut var_guard11_db5: f64 = *var_guard11_db5_slot;
        let mut var_guard11_db6: f64 = *var_guard11_db6_slot;
        let mut var_guard11_db7: f64 = *var_guard11_db7_slot;
        let mut var_guard11_dn0: f64 = *var_guard11_dn0_slot;
        let mut var_guard11_dn1: f64 = *var_guard11_dn1_slot;
        let mut var_guard11_dn2: f64 = *var_guard11_dn2_slot;
        let mut var_guard11_dn3: f64 = *var_guard11_dn3_slot;
        let mut var_guard11_dn4: f64 = *var_guard11_dn4_slot;
        let mut var_guard11_dn5: f64 = *var_guard11_dn5_slot;
        let mut var_guard11_dn6: f64 = *var_guard11_dn6_slot;
        let mut var_guard11_dn7: f64 = *var_guard11_dn7_slot;
        let mut var_guard11_dn8: f64 = *var_guard11_dn8_slot;
        let mut var_guard11_dn9: f64 = *var_guard11_dn9_slot;
        let mut var_guard11_rdb0: f64 = *var_guard11_rdb0_slot;
        let mut var_guard11_rdb1: f64 = *var_guard11_rdb1_slot;
        let mut var_guard11_rdb2: f64 = *var_guard11_rdb2_slot;
        let mut var_guard11_rdb3: f64 = *var_guard11_rdb3_slot;
        let mut var_guard11_rdb4: f64 = *var_guard11_rdb4_slot;
        let mut var_guard11_rdb5: f64 = *var_guard11_rdb5_slot;
        let mut var_guard11_rdb6: f64 = *var_guard11_rdb6_slot;
        let mut var_guard11_rdb7: f64 = *var_guard11_rdb7_slot;
        let mut var_guard11_rdn0: f64 = *var_guard11_rdn0_slot;
        let mut var_guard11_rdn1: f64 = *var_guard11_rdn1_slot;
        let mut var_guard11_rdn2: f64 = *var_guard11_rdn2_slot;
        let mut var_guard11_rdn3: f64 = *var_guard11_rdn3_slot;
        let mut var_guard11_rdn4: f64 = *var_guard11_rdn4_slot;
        let mut var_guard11_rdn5: f64 = *var_guard11_rdn5_slot;
        let mut var_guard11_rdn6: f64 = *var_guard11_rdn6_slot;
        let mut var_guard11_rdn7: f64 = *var_guard11_rdn7_slot;
        let mut var_guard11_rdn8: f64 = *var_guard11_rdn8_slot;
        let mut var_guard11_rdn9: f64 = *var_guard11_rdn9_slot;
        let mut var_guard11_rv: f64 = *var_guard11_rv_slot;
        let mut var_ibwd: f64 = *var_ibwd_slot;
        let mut var_ibwd_db0: f64 = *var_ibwd_db0_slot;
        let mut var_ibwd_db1: f64 = *var_ibwd_db1_slot;
        let mut var_ibwd_db2: f64 = *var_ibwd_db2_slot;
        let mut var_ibwd_db3: f64 = *var_ibwd_db3_slot;
        let mut var_ibwd_db4: f64 = *var_ibwd_db4_slot;
        let mut var_ibwd_db5: f64 = *var_ibwd_db5_slot;
        let mut var_ibwd_db6: f64 = *var_ibwd_db6_slot;
        let mut var_ibwd_db7: f64 = *var_ibwd_db7_slot;
        let mut var_ibwd_dn0: f64 = *var_ibwd_dn0_slot;
        let mut var_ibwd_dn1: f64 = *var_ibwd_dn1_slot;
        let mut var_ibwd_dn2: f64 = *var_ibwd_dn2_slot;
        let mut var_ibwd_dn3: f64 = *var_ibwd_dn3_slot;
        let mut var_ibwd_dn4: f64 = *var_ibwd_dn4_slot;
        let mut var_ibwd_dn5: f64 = *var_ibwd_dn5_slot;
        let mut var_ibwd_dn6: f64 = *var_ibwd_dn6_slot;
        let mut var_ibwd_dn7: f64 = *var_ibwd_dn7_slot;
        let mut var_ibwd_dn8: f64 = *var_ibwd_dn8_slot;
        let mut var_ibwd_dn9: f64 = *var_ibwd_dn9_slot;
        let mut var_ibwd_rdb0: f64 = *var_ibwd_rdb0_slot;
        let mut var_ibwd_rdb1: f64 = *var_ibwd_rdb1_slot;
        let mut var_ibwd_rdb2: f64 = *var_ibwd_rdb2_slot;
        let mut var_ibwd_rdb3: f64 = *var_ibwd_rdb3_slot;
        let mut var_ibwd_rdb4: f64 = *var_ibwd_rdb4_slot;
        let mut var_ibwd_rdb5: f64 = *var_ibwd_rdb5_slot;
        let mut var_ibwd_rdb6: f64 = *var_ibwd_rdb6_slot;
        let mut var_ibwd_rdb7: f64 = *var_ibwd_rdb7_slot;
        let mut var_ibwd_rdn0: f64 = *var_ibwd_rdn0_slot;
        let mut var_ibwd_rdn1: f64 = *var_ibwd_rdn1_slot;
        let mut var_ibwd_rdn2: f64 = *var_ibwd_rdn2_slot;
        let mut var_ibwd_rdn3: f64 = *var_ibwd_rdn3_slot;
        let mut var_ibwd_rdn4: f64 = *var_ibwd_rdn4_slot;
        let mut var_ibwd_rdn5: f64 = *var_ibwd_rdn5_slot;
        let mut var_ibwd_rdn6: f64 = *var_ibwd_rdn6_slot;
        let mut var_ibwd_rdn7: f64 = *var_ibwd_rdn7_slot;
        let mut var_ibwd_rdn8: f64 = *var_ibwd_rdn8_slot;
        let mut var_ibwd_rdn9: f64 = *var_ibwd_rdn9_slot;
        let mut var_ibwd_rv: f64 = *var_ibwd_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_db0: f64 = *var_le_db0_slot;
        let mut var_le_db1: f64 = *var_le_db1_slot;
        let mut var_le_db2: f64 = *var_le_db2_slot;
        let mut var_le_db3: f64 = *var_le_db3_slot;
        let mut var_le_db4: f64 = *var_le_db4_slot;
        let mut var_le_db5: f64 = *var_le_db5_slot;
        let mut var_le_db6: f64 = *var_le_db6_slot;
        let mut var_le_db7: f64 = *var_le_db7_slot;
        let mut var_le_dn0: f64 = *var_le_dn0_slot;
        let mut var_le_dn1: f64 = *var_le_dn1_slot;
        let mut var_le_dn2: f64 = *var_le_dn2_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn5: f64 = *var_le_dn5_slot;
        let mut var_le_dn6: f64 = *var_le_dn6_slot;
        let mut var_le_dn7: f64 = *var_le_dn7_slot;
        let mut var_le_dn8: f64 = *var_le_dn8_slot;
        let mut var_le_dn9: f64 = *var_le_dn9_slot;
        let mut var_le_rdb0: f64 = *var_le_rdb0_slot;
        let mut var_le_rdb1: f64 = *var_le_rdb1_slot;
        let mut var_le_rdb2: f64 = *var_le_rdb2_slot;
        let mut var_le_rdb3: f64 = *var_le_rdb3_slot;
        let mut var_le_rdb4: f64 = *var_le_rdb4_slot;
        let mut var_le_rdb5: f64 = *var_le_rdb5_slot;
        let mut var_le_rdb6: f64 = *var_le_rdb6_slot;
        let mut var_le_rdb7: f64 = *var_le_rdb7_slot;
        let mut var_le_rdn0: f64 = *var_le_rdn0_slot;
        let mut var_le_rdn1: f64 = *var_le_rdn1_slot;
        let mut var_le_rdn2: f64 = *var_le_rdn2_slot;
        let mut var_le_rdn3: f64 = *var_le_rdn3_slot;
        let mut var_le_rdn4: f64 = *var_le_rdn4_slot;
        let mut var_le_rdn5: f64 = *var_le_rdn5_slot;
        let mut var_le_rdn6: f64 = *var_le_rdn6_slot;
        let mut var_le_rdn7: f64 = *var_le_rdn7_slot;
        let mut var_le_rdn8: f64 = *var_le_rdn8_slot;
        let mut var_le_rdn9: f64 = *var_le_rdn9_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_lebv: f64 = *var_lebv_slot;
        let mut var_lebv_db0: f64 = *var_lebv_db0_slot;
        let mut var_lebv_db1: f64 = *var_lebv_db1_slot;
        let mut var_lebv_db2: f64 = *var_lebv_db2_slot;
        let mut var_lebv_db3: f64 = *var_lebv_db3_slot;
        let mut var_lebv_db4: f64 = *var_lebv_db4_slot;
        let mut var_lebv_db5: f64 = *var_lebv_db5_slot;
        let mut var_lebv_db6: f64 = *var_lebv_db6_slot;
        let mut var_lebv_db7: f64 = *var_lebv_db7_slot;
        let mut var_lebv_dn0: f64 = *var_lebv_dn0_slot;
        let mut var_lebv_dn1: f64 = *var_lebv_dn1_slot;
        let mut var_lebv_dn2: f64 = *var_lebv_dn2_slot;
        let mut var_lebv_dn3: f64 = *var_lebv_dn3_slot;
        let mut var_lebv_dn4: f64 = *var_lebv_dn4_slot;
        let mut var_lebv_dn5: f64 = *var_lebv_dn5_slot;
        let mut var_lebv_dn6: f64 = *var_lebv_dn6_slot;
        let mut var_lebv_dn7: f64 = *var_lebv_dn7_slot;
        let mut var_lebv_dn8: f64 = *var_lebv_dn8_slot;
        let mut var_lebv_dn9: f64 = *var_lebv_dn9_slot;
        let mut var_lebv_rdb0: f64 = *var_lebv_rdb0_slot;
        let mut var_lebv_rdb1: f64 = *var_lebv_rdb1_slot;
        let mut var_lebv_rdb2: f64 = *var_lebv_rdb2_slot;
        let mut var_lebv_rdb3: f64 = *var_lebv_rdb3_slot;
        let mut var_lebv_rdb4: f64 = *var_lebv_rdb4_slot;
        let mut var_lebv_rdb5: f64 = *var_lebv_rdb5_slot;
        let mut var_lebv_rdb6: f64 = *var_lebv_rdb6_slot;
        let mut var_lebv_rdb7: f64 = *var_lebv_rdb7_slot;
        let mut var_lebv_rdn0: f64 = *var_lebv_rdn0_slot;
        let mut var_lebv_rdn1: f64 = *var_lebv_rdn1_slot;
        let mut var_lebv_rdn2: f64 = *var_lebv_rdn2_slot;
        let mut var_lebv_rdn3: f64 = *var_lebv_rdn3_slot;
        let mut var_lebv_rdn4: f64 = *var_lebv_rdn4_slot;
        let mut var_lebv_rdn5: f64 = *var_lebv_rdn5_slot;
        let mut var_lebv_rdn6: f64 = *var_lebv_rdn6_slot;
        let mut var_lebv_rdn7: f64 = *var_lebv_rdn7_slot;
        let mut var_lebv_rdn8: f64 = *var_lebv_rdn8_slot;
        let mut var_lebv_rdn9: f64 = *var_lebv_rdn9_slot;
        let mut var_lebv_rv: f64 = *var_lebv_rv_slot;

        let assign1090_e1363: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard10 = assign1090_e1363;
        var_guard10_dn0 = 0.0;
        var_guard10_dn1 = 0.0;
        var_guard10_dn2 = 0.0;
        var_guard10_dn3 = 0.0;
        var_guard10_dn4 = 0.0;
        var_guard10_dn5 = 0.0;
        var_guard10_dn6 = 0.0;
        var_guard10_dn7 = 0.0;
        var_guard10_dn8 = 0.0;
        var_guard10_dn9 = 0.0;
        var_guard10_db0 = 0.0;
        var_guard10_db1 = 0.0;
        var_guard10_db2 = 0.0;
        var_guard10_db3 = 0.0;
        var_guard10_db4 = 0.0;
        var_guard10_db5 = 0.0;
        var_guard10_db6 = 0.0;
        var_guard10_db7 = 0.0;
        var_guard10_rv = 0.0;
        var_guard10_rdn0 = 0.0;
        var_guard10_rdn1 = 0.0;
        var_guard10_rdn2 = 0.0;
        var_guard10_rdn3 = 0.0;
        var_guard10_rdn4 = 0.0;
        var_guard10_rdn5 = 0.0;
        var_guard10_rdn6 = 0.0;
        var_guard10_rdn7 = 0.0;
        var_guard10_rdn8 = 0.0;
        var_guard10_rdn9 = 0.0;
        var_guard10_rdb0 = 0.0;
        var_guard10_rdb1 = 0.0;
        var_guard10_rdb2 = 0.0;
        var_guard10_rdb3 = 0.0;
        var_guard10_rdb4 = 0.0;
        var_guard10_rdb5 = 0.0;
        var_guard10_rdb6 = 0.0;
        var_guard10_rdb7 = 0.0;

        let (assign1100_e1373, assign1100_e1373_d_n0, assign1100_e1373_d_n1, assign1100_e1373_d_n2, assign1100_e1373_d_n3, assign1100_e1373_d_n4, assign1100_e1373_d_n5, assign1100_e1373_d_n6, assign1100_e1373_d_n7, assign1100_e1373_d_n8, assign1100_e1373_d_n9, assign1100_e1373_d_b0, assign1100_e1373_d_b1, assign1100_e1373_d_b2, assign1100_e1373_d_b3, assign1100_e1373_d_b4, assign1100_e1373_d_b5, assign1100_e1373_d_b6, assign1100_e1373_d_b7,) = {
    if ((var_guard9 != 0.0) && (var_guard10 != 0.0)) {
        let assign1100_e1370: f64 = (var_arg - 80.0);
        let assign1100_e1371: f64 = (1.0 + assign1100_e1370);
        (assign1100_e1371, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign1100_e1373;
        var_le_dn0 = assign1100_e1373_d_n0;
        var_le_dn1 = assign1100_e1373_d_n1;
        var_le_dn2 = assign1100_e1373_d_n2;
        var_le_dn3 = assign1100_e1373_d_n3;
        var_le_dn4 = assign1100_e1373_d_n4;
        var_le_dn5 = assign1100_e1373_d_n5;
        var_le_dn6 = assign1100_e1373_d_n6;
        var_le_dn7 = assign1100_e1373_d_n7;
        var_le_dn8 = assign1100_e1373_d_n8;
        var_le_dn9 = assign1100_e1373_d_n9;
        var_le_db0 = assign1100_e1373_d_b0;
        var_le_db1 = assign1100_e1373_d_b1;
        var_le_db2 = assign1100_e1373_d_b2;
        var_le_db3 = assign1100_e1373_d_b3;
        var_le_db4 = assign1100_e1373_d_b4;
        var_le_db5 = assign1100_e1373_d_b5;
        var_le_db6 = assign1100_e1373_d_b6;
        var_le_db7 = assign1100_e1373_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign1110_e1379, assign1110_e1379_d_n0, assign1110_e1379_d_n1, assign1110_e1379_d_n2, assign1110_e1379_d_n3, assign1110_e1379_d_n4, assign1110_e1379_d_n5, assign1110_e1379_d_n6, assign1110_e1379_d_n7, assign1110_e1379_d_n8, assign1110_e1379_d_n9, assign1110_e1379_d_b0, assign1110_e1379_d_b1, assign1110_e1379_d_b2, assign1110_e1379_d_b3, assign1110_e1379_d_b4, assign1110_e1379_d_b5, assign1110_e1379_d_b6, assign1110_e1379_d_b7,) = {
    if ((var_guard9 != 0.0) && (var_guard10 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    }
};
        var_arg = assign1110_e1379;
        var_arg_dn0 = assign1110_e1379_d_n0;
        var_arg_dn1 = assign1110_e1379_d_n1;
        var_arg_dn2 = assign1110_e1379_d_n2;
        var_arg_dn3 = assign1110_e1379_d_n3;
        var_arg_dn4 = assign1110_e1379_d_n4;
        var_arg_dn5 = assign1110_e1379_d_n5;
        var_arg_dn6 = assign1110_e1379_d_n6;
        var_arg_dn7 = assign1110_e1379_d_n7;
        var_arg_dn8 = assign1110_e1379_d_n8;
        var_arg_dn9 = assign1110_e1379_d_n9;
        var_arg_db0 = assign1110_e1379_d_b0;
        var_arg_db1 = assign1110_e1379_d_b1;
        var_arg_db2 = assign1110_e1379_d_b2;
        var_arg_db3 = assign1110_e1379_d_b3;
        var_arg_db4 = assign1110_e1379_d_b4;
        var_arg_db5 = assign1110_e1379_d_b5;
        var_arg_db6 = assign1110_e1379_d_b6;
        var_arg_db7 = assign1110_e1379_d_b7;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdn7 = 0.0;
        var_arg_rdn8 = 0.0;
        var_arg_rdn9 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;
        var_arg_rdb7 = 0.0;

        let (assign1120_e1386, assign1120_e1386_d_n0, assign1120_e1386_d_n1, assign1120_e1386_d_n2, assign1120_e1386_d_n3, assign1120_e1386_d_n4, assign1120_e1386_d_n5, assign1120_e1386_d_n6, assign1120_e1386_d_n7, assign1120_e1386_d_n8, assign1120_e1386_d_n9, assign1120_e1386_d_b0, assign1120_e1386_d_b1, assign1120_e1386_d_b2, assign1120_e1386_d_b3, assign1120_e1386_d_b4, assign1120_e1386_d_b5, assign1120_e1386_d_b6, assign1120_e1386_d_b7,) = {
    if ((var_guard9 != 0.0) && (var_guard10 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign1120_e1386;
        var_le_dn0 = assign1120_e1386_d_n0;
        var_le_dn1 = assign1120_e1386_d_n1;
        var_le_dn2 = assign1120_e1386_d_n2;
        var_le_dn3 = assign1120_e1386_d_n3;
        var_le_dn4 = assign1120_e1386_d_n4;
        var_le_dn5 = assign1120_e1386_d_n5;
        var_le_dn6 = assign1120_e1386_d_n6;
        var_le_dn7 = assign1120_e1386_d_n7;
        var_le_dn8 = assign1120_e1386_d_n8;
        var_le_dn9 = assign1120_e1386_d_n9;
        var_le_db0 = assign1120_e1386_d_b0;
        var_le_db1 = assign1120_e1386_d_b1;
        var_le_db2 = assign1120_e1386_d_b2;
        var_le_db3 = assign1120_e1386_d_b3;
        var_le_db4 = assign1120_e1386_d_b4;
        var_le_db5 = assign1120_e1386_d_b5;
        var_le_db6 = assign1120_e1386_d_b6;
        var_le_db7 = assign1120_e1386_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign1130_e1393, assign1130_e1393_d_n0, assign1130_e1393_d_n1, assign1130_e1393_d_n2, assign1130_e1393_d_n3, assign1130_e1393_d_n4, assign1130_e1393_d_n5, assign1130_e1393_d_n6, assign1130_e1393_d_n7, assign1130_e1393_d_n8, assign1130_e1393_d_n9, assign1130_e1393_d_b0, assign1130_e1393_d_b1, assign1130_e1393_d_b2, assign1130_e1393_d_b3, assign1130_e1393_d_b4, assign1130_e1393_d_b5, assign1130_e1393_d_b6, assign1130_e1393_d_b7,) = {
    if (var_guard9 != 0.0) {
        let assign1130_e1390: f64 = (var_arg).exp();
        let assign1130_e1391: f64 = (var_le * assign1130_e1390);
        (assign1130_e1391, ((var_le_dn0 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn0))), ((var_le_dn1 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn1))), ((var_le_dn2 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn2))), ((var_le_dn3 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn3))), ((var_le_dn4 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn4))), ((var_le_dn5 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn5))), ((var_le_dn6 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn6))), ((var_le_dn7 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn7))), ((var_le_dn8 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn8))), ((var_le_dn9 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_dn9))), ((var_le_db0 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_db0))), ((var_le_db1 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_db1))), ((var_le_db2 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_db2))), ((var_le_db3 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_db3))), ((var_le_db4 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_db4))), ((var_le_db5 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_db5))), ((var_le_db6 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_db6))), ((var_le_db7 * assign1130_e1390) + (var_le * (assign1130_e1390 * var_arg_db7))),)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign1130_e1393;
        var_le_dn0 = assign1130_e1393_d_n0;
        var_le_dn1 = assign1130_e1393_d_n1;
        var_le_dn2 = assign1130_e1393_d_n2;
        var_le_dn3 = assign1130_e1393_d_n3;
        var_le_dn4 = assign1130_e1393_d_n4;
        var_le_dn5 = assign1130_e1393_d_n5;
        var_le_dn6 = assign1130_e1393_d_n6;
        var_le_dn7 = assign1130_e1393_d_n7;
        var_le_dn8 = assign1130_e1393_d_n8;
        var_le_dn9 = assign1130_e1393_d_n9;
        var_le_db0 = assign1130_e1393_d_b0;
        var_le_db1 = assign1130_e1393_d_b1;
        var_le_db2 = assign1130_e1393_d_b2;
        var_le_db3 = assign1130_e1393_d_b3;
        var_le_db4 = assign1130_e1393_d_b4;
        var_le_db5 = assign1130_e1393_d_b5;
        var_le_db6 = assign1130_e1393_d_b6;
        var_le_db7 = assign1130_e1393_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign1140_e1465, assign1140_e1465_d_n0, assign1140_e1465_d_n1, assign1140_e1465_d_n2, assign1140_e1465_d_n3, assign1140_e1465_d_n4, assign1140_e1465_d_n5, assign1140_e1465_d_n6, assign1140_e1465_d_n7, assign1140_e1465_d_n8, assign1140_e1465_d_n9, assign1140_e1465_d_b0, assign1140_e1465_d_b1, assign1140_e1465_d_b2, assign1140_e1465_d_b3, assign1140_e1465_d_b4, assign1140_e1465_d_b5, assign1140_e1465_d_b6, assign1140_e1465_d_b7,) = {
    if (var_guard9 != 0.0) {
        let assign1140_e1401: f64 = (-37.0);
        let (assign1140_e1428, assign1140_e1428_d_n0, assign1140_e1428_d_n1, assign1140_e1428_d_n2, assign1140_e1428_d_n3, assign1140_e1428_d_n4, assign1140_e1428_d_n5, assign1140_e1428_d_n6, assign1140_e1428_d_n7, assign1140_e1428_d_n8, assign1140_e1428_d_n9, assign1140_e1428_d_b0, assign1140_e1428_d_b1, assign1140_e1428_d_b2, assign1140_e1428_d_b3, assign1140_e1428_d_b4, assign1140_e1428_d_b5, assign1140_e1428_d_b6, assign1140_e1428_d_b7,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign1140_e1401))) {
                let assign1140_e1406: f64 = (var_argbv).exp();
                let assign1140_e1408: f64 = (assign1140_e1406 + 1.0);
                let assign1140_e1409: f64 = (assign1140_e1408).ln();
                (assign1140_e1409, ((assign1140_e1406 * var_argbv_dn0) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn1) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn2) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn3) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn4) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn5) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn6) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn7) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn8) / assign1140_e1408), ((assign1140_e1406 * var_argbv_dn9) / assign1140_e1408), ((assign1140_e1406 * var_argbv_db0) / assign1140_e1408), ((assign1140_e1406 * var_argbv_db1) / assign1140_e1408), ((assign1140_e1406 * var_argbv_db2) / assign1140_e1408), ((assign1140_e1406 * var_argbv_db3) / assign1140_e1408), ((assign1140_e1406 * var_argbv_db4) / assign1140_e1408), ((assign1140_e1406 * var_argbv_db5) / assign1140_e1408), ((assign1140_e1406 * var_argbv_db6) / assign1140_e1408), ((assign1140_e1406 * var_argbv_db7) / assign1140_e1408),)
            } else {
                let assign1140_e1416: f64 = (-37.0);
                let (assign1140_e1427, assign1140_e1427_d_n0, assign1140_e1427_d_n1, assign1140_e1427_d_n2, assign1140_e1427_d_n3, assign1140_e1427_d_n4, assign1140_e1427_d_n5, assign1140_e1427_d_n6, assign1140_e1427_d_n7, assign1140_e1427_d_n8, assign1140_e1427_d_n9, assign1140_e1427_d_b0, assign1140_e1427_d_b1, assign1140_e1427_d_b2, assign1140_e1427_d_b3, assign1140_e1427_d_b4, assign1140_e1427_d_b5, assign1140_e1427_d_b6, assign1140_e1427_d_b7,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign1140_e1416)) {
                        let assign1140_e1420: f64 = (var_argbv).exp();
                        (assign1140_e1420, (assign1140_e1420 * var_argbv_dn0), (assign1140_e1420 * var_argbv_dn1), (assign1140_e1420 * var_argbv_dn2), (assign1140_e1420 * var_argbv_dn3), (assign1140_e1420 * var_argbv_dn4), (assign1140_e1420 * var_argbv_dn5), (assign1140_e1420 * var_argbv_dn6), (assign1140_e1420 * var_argbv_dn7), (assign1140_e1420 * var_argbv_dn8), (assign1140_e1420 * var_argbv_dn9), (assign1140_e1420 * var_argbv_db0), (assign1140_e1420 * var_argbv_db1), (assign1140_e1420 * var_argbv_db2), (assign1140_e1420 * var_argbv_db3), (assign1140_e1420 * var_argbv_db4), (assign1140_e1420 * var_argbv_db5), (assign1140_e1420 * var_argbv_db6), (assign1140_e1420 * var_argbv_db7),)
                    } else {
                        let (assign1140_e1426, assign1140_e1426_d_n0, assign1140_e1426_d_n1, assign1140_e1426_d_n2, assign1140_e1426_d_n3, assign1140_e1426_d_n4, assign1140_e1426_d_n5, assign1140_e1426_d_n6, assign1140_e1426_d_n7, assign1140_e1426_d_n8, assign1140_e1426_d_n9, assign1140_e1426_d_b0, assign1140_e1426_d_b1, assign1140_e1426_d_b2, assign1140_e1426_d_b3, assign1140_e1426_d_b4, assign1140_e1426_d_b5, assign1140_e1426_d_b6, assign1140_e1426_d_b7,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn0, var_argbv_dn1, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6, var_argbv_dn7, var_argbv_dn8, var_argbv_dn9, var_argbv_db0, var_argbv_db1, var_argbv_db2, var_argbv_db3, var_argbv_db4, var_argbv_db5, var_argbv_db6, var_argbv_db7,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign1140_e1426, assign1140_e1426_d_n0, assign1140_e1426_d_n1, assign1140_e1426_d_n2, assign1140_e1426_d_n3, assign1140_e1426_d_n4, assign1140_e1426_d_n5, assign1140_e1426_d_n6, assign1140_e1426_d_n7, assign1140_e1426_d_n8, assign1140_e1426_d_n9, assign1140_e1426_d_b0, assign1140_e1426_d_b1, assign1140_e1426_d_b2, assign1140_e1426_d_b3, assign1140_e1426_d_b4, assign1140_e1426_d_b5, assign1140_e1426_d_b6, assign1140_e1426_d_b7,)
                    }
                };
                (assign1140_e1427, assign1140_e1427_d_n0, assign1140_e1427_d_n1, assign1140_e1427_d_n2, assign1140_e1427_d_n3, assign1140_e1427_d_n4, assign1140_e1427_d_n5, assign1140_e1427_d_n6, assign1140_e1427_d_n7, assign1140_e1427_d_n8, assign1140_e1427_d_n9, assign1140_e1427_d_b0, assign1140_e1427_d_b1, assign1140_e1427_d_b2, assign1140_e1427_d_b3, assign1140_e1427_d_b4, assign1140_e1427_d_b5, assign1140_e1427_d_b6, assign1140_e1427_d_b7,)
            }
        };
        let assign1140_e1435: f64 = (-37.0);
        let (assign1140_e1462, assign1140_e1462_d_n0, assign1140_e1462_d_n1, assign1140_e1462_d_n2, assign1140_e1462_d_n3, assign1140_e1462_d_n4, assign1140_e1462_d_n5, assign1140_e1462_d_n6, assign1140_e1462_d_n7, assign1140_e1462_d_n8, assign1140_e1462_d_n9, assign1140_e1462_d_b0, assign1140_e1462_d_b1, assign1140_e1462_d_b2, assign1140_e1462_d_b3, assign1140_e1462_d_b4, assign1140_e1462_d_b5, assign1140_e1462_d_b6, assign1140_e1462_d_b7,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign1140_e1435))) {
                let assign1140_e1440: f64 = (var_argbvvt).exp();
                let assign1140_e1442: f64 = (assign1140_e1440 + 1.0);
                let assign1140_e1443: f64 = (assign1140_e1442).ln();
                (assign1140_e1443, ((assign1140_e1440 * var_argbvvt_dn0) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_dn1) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_dn2) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_dn3) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_dn4) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_dn5) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_dn6) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_dn7) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_dn8) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_dn9) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_db0) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_db1) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_db2) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_db3) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_db4) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_db5) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_db6) / assign1140_e1442), ((assign1140_e1440 * var_argbvvt_db7) / assign1140_e1442),)
            } else {
                let assign1140_e1450: f64 = (-37.0);
                let (assign1140_e1461, assign1140_e1461_d_n0, assign1140_e1461_d_n1, assign1140_e1461_d_n2, assign1140_e1461_d_n3, assign1140_e1461_d_n4, assign1140_e1461_d_n5, assign1140_e1461_d_n6, assign1140_e1461_d_n7, assign1140_e1461_d_n8, assign1140_e1461_d_n9, assign1140_e1461_d_b0, assign1140_e1461_d_b1, assign1140_e1461_d_b2, assign1140_e1461_d_b3, assign1140_e1461_d_b4, assign1140_e1461_d_b5, assign1140_e1461_d_b6, assign1140_e1461_d_b7,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign1140_e1450)) {
                        let assign1140_e1454: f64 = (var_argbvvt).exp();
                        (assign1140_e1454, (assign1140_e1454 * var_argbvvt_dn0), (assign1140_e1454 * var_argbvvt_dn1), (assign1140_e1454 * var_argbvvt_dn2), (assign1140_e1454 * var_argbvvt_dn3), (assign1140_e1454 * var_argbvvt_dn4), (assign1140_e1454 * var_argbvvt_dn5), (assign1140_e1454 * var_argbvvt_dn6), (assign1140_e1454 * var_argbvvt_dn7), (assign1140_e1454 * var_argbvvt_dn8), (assign1140_e1454 * var_argbvvt_dn9), (assign1140_e1454 * var_argbvvt_db0), (assign1140_e1454 * var_argbvvt_db1), (assign1140_e1454 * var_argbvvt_db2), (assign1140_e1454 * var_argbvvt_db3), (assign1140_e1454 * var_argbvvt_db4), (assign1140_e1454 * var_argbvvt_db5), (assign1140_e1454 * var_argbvvt_db6), (assign1140_e1454 * var_argbvvt_db7),)
                    } else {
                        let (assign1140_e1460, assign1140_e1460_d_n0, assign1140_e1460_d_n1, assign1140_e1460_d_n2, assign1140_e1460_d_n3, assign1140_e1460_d_n4, assign1140_e1460_d_n5, assign1140_e1460_d_n6, assign1140_e1460_d_n7, assign1140_e1460_d_n8, assign1140_e1460_d_n9, assign1140_e1460_d_b0, assign1140_e1460_d_b1, assign1140_e1460_d_b2, assign1140_e1460_d_b3, assign1140_e1460_d_b4, assign1140_e1460_d_b5, assign1140_e1460_d_b6, assign1140_e1460_d_b7,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn0, var_argbvvt_dn1, var_argbvvt_dn2, var_argbvvt_dn3, var_argbvvt_dn4, var_argbvvt_dn5, var_argbvvt_dn6, var_argbvvt_dn7, var_argbvvt_dn8, var_argbvvt_dn9, var_argbvvt_db0, var_argbvvt_db1, var_argbvvt_db2, var_argbvvt_db3, var_argbvvt_db4, var_argbvvt_db5, var_argbvvt_db6, var_argbvvt_db7,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign1140_e1460, assign1140_e1460_d_n0, assign1140_e1460_d_n1, assign1140_e1460_d_n2, assign1140_e1460_d_n3, assign1140_e1460_d_n4, assign1140_e1460_d_n5, assign1140_e1460_d_n6, assign1140_e1460_d_n7, assign1140_e1460_d_n8, assign1140_e1460_d_n9, assign1140_e1460_d_b0, assign1140_e1460_d_b1, assign1140_e1460_d_b2, assign1140_e1460_d_b3, assign1140_e1460_d_b4, assign1140_e1460_d_b5, assign1140_e1460_d_b6, assign1140_e1460_d_b7,)
                    }
                };
                (assign1140_e1461, assign1140_e1461_d_n0, assign1140_e1461_d_n1, assign1140_e1461_d_n2, assign1140_e1461_d_n3, assign1140_e1461_d_n4, assign1140_e1461_d_n5, assign1140_e1461_d_n6, assign1140_e1461_d_n7, assign1140_e1461_d_n8, assign1140_e1461_d_n9, assign1140_e1461_d_b0, assign1140_e1461_d_b1, assign1140_e1461_d_b2, assign1140_e1461_d_b3, assign1140_e1461_d_b4, assign1140_e1461_d_b5, assign1140_e1461_d_b6, assign1140_e1461_d_b7,)
            }
        };
        let assign1140_e1463: f64 = (assign1140_e1428 - assign1140_e1462);
        (assign1140_e1463, (assign1140_e1428_d_n0 - assign1140_e1462_d_n0), (assign1140_e1428_d_n1 - assign1140_e1462_d_n1), (assign1140_e1428_d_n2 - assign1140_e1462_d_n2), (assign1140_e1428_d_n3 - assign1140_e1462_d_n3), (assign1140_e1428_d_n4 - assign1140_e1462_d_n4), (assign1140_e1428_d_n5 - assign1140_e1462_d_n5), (assign1140_e1428_d_n6 - assign1140_e1462_d_n6), (assign1140_e1428_d_n7 - assign1140_e1462_d_n7), (assign1140_e1428_d_n8 - assign1140_e1462_d_n8), (assign1140_e1428_d_n9 - assign1140_e1462_d_n9), (assign1140_e1428_d_b0 - assign1140_e1462_d_b0), (assign1140_e1428_d_b1 - assign1140_e1462_d_b1), (assign1140_e1428_d_b2 - assign1140_e1462_d_b2), (assign1140_e1428_d_b3 - assign1140_e1462_d_b3), (assign1140_e1428_d_b4 - assign1140_e1462_d_b4), (assign1140_e1428_d_b5 - assign1140_e1462_d_b5), (assign1140_e1428_d_b6 - assign1140_e1462_d_b6), (assign1140_e1428_d_b7 - assign1140_e1462_d_b7),)
    } else {
        (var_lebv, var_lebv_dn0, var_lebv_dn1, var_lebv_dn2, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6, var_lebv_dn7, var_lebv_dn8, var_lebv_dn9, var_lebv_db0, var_lebv_db1, var_lebv_db2, var_lebv_db3, var_lebv_db4, var_lebv_db5, var_lebv_db6, var_lebv_db7,)
    }
};
        var_lebv = assign1140_e1465;
        var_lebv_dn0 = assign1140_e1465_d_n0;
        var_lebv_dn1 = assign1140_e1465_d_n1;
        var_lebv_dn2 = assign1140_e1465_d_n2;
        var_lebv_dn3 = assign1140_e1465_d_n3;
        var_lebv_dn4 = assign1140_e1465_d_n4;
        var_lebv_dn5 = assign1140_e1465_d_n5;
        var_lebv_dn6 = assign1140_e1465_d_n6;
        var_lebv_dn7 = assign1140_e1465_d_n7;
        var_lebv_dn8 = assign1140_e1465_d_n8;
        var_lebv_dn9 = assign1140_e1465_d_n9;
        var_lebv_db0 = assign1140_e1465_d_b0;
        var_lebv_db1 = assign1140_e1465_d_b1;
        var_lebv_db2 = assign1140_e1465_d_b2;
        var_lebv_db3 = assign1140_e1465_d_b3;
        var_lebv_db4 = assign1140_e1465_d_b4;
        var_lebv_db5 = assign1140_e1465_d_b5;
        var_lebv_db6 = assign1140_e1465_d_b6;
        var_lebv_db7 = assign1140_e1465_d_b7;
        var_lebv_rv = 0.0;
        var_lebv_rdn0 = 0.0;
        var_lebv_rdn1 = 0.0;
        var_lebv_rdn2 = 0.0;
        var_lebv_rdn3 = 0.0;
        var_lebv_rdn4 = 0.0;
        var_lebv_rdn5 = 0.0;
        var_lebv_rdn6 = 0.0;
        var_lebv_rdn7 = 0.0;
        var_lebv_rdn8 = 0.0;
        var_lebv_rdn9 = 0.0;
        var_lebv_rdb0 = 0.0;
        var_lebv_rdb1 = 0.0;
        var_lebv_rdb2 = 0.0;
        var_lebv_rdb3 = 0.0;
        var_lebv_rdb4 = 0.0;
        var_lebv_rdb5 = 0.0;
        var_lebv_rdb6 = 0.0;
        var_lebv_rdb7 = 0.0;

        let (assign1150_e1486, assign1150_e1486_d_n0, assign1150_e1486_d_n1, assign1150_e1486_d_n2, assign1150_e1486_d_n3, assign1150_e1486_d_n4, assign1150_e1486_d_n5, assign1150_e1486_d_n6, assign1150_e1486_d_n7, assign1150_e1486_d_n8, assign1150_e1486_d_n9, assign1150_e1486_d_b0, assign1150_e1486_d_b1, assign1150_e1486_d_b2, assign1150_e1486_d_b3, assign1150_e1486_d_b4, assign1150_e1486_d_b5, assign1150_e1486_d_b6, assign1150_e1486_d_b7,) = {
    if (var_guard9 != 0.0) {
        let assign1150_e1470: f64 = (var_le - 1.0);
        let assign1150_e1471: f64 = (var_is_t * assign1150_e1470);
        let assign1150_e1474: f64 = (var_ijbvc_t * var_lebv);
        let assign1150_e1478: f64 = (var_vbici).abs();
        let assign1150_e1480: f64 = (assign1150_e1478).powf(var_theexp_t);
        let assign1150_e1481: f64 = (p.p8 * assign1150_e1480);
        let assign1150_e1482: f64 = (1.0 + assign1150_e1481);
        let assign1150_e1483: f64 = (assign1150_e1474 / assign1150_e1482);
        let assign1150_e1484: f64 = (assign1150_e1471 - assign1150_e1483);
        (assign1150_e1484, (((var_is_t_dn0 * assign1150_e1470) + (var_is_t * var_le_dn0)) - (((((var_ijbvc_t_dn0 * var_lebv) + (var_ijbvc_t * var_lebv_dn0)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_dn0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn0 } else { (-var_vbici_dn0) })) } } else { (assign1150_e1480 * ((var_theexp_t_dn0 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn0 } else { (-var_vbici_dn0) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_dn1 * assign1150_e1470) + (var_is_t * var_le_dn1)) - (((((var_ijbvc_t_dn1 * var_lebv) + (var_ijbvc_t * var_lebv_dn1)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_dn1 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn1 } else { (-var_vbici_dn1) })) } } else { (assign1150_e1480 * ((var_theexp_t_dn1 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn1 } else { (-var_vbici_dn1) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_dn2 * assign1150_e1470) + (var_is_t * var_le_dn2)) - (((((var_ijbvc_t_dn2 * var_lebv) + (var_ijbvc_t * var_lebv_dn2)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_dn2 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn2 } else { (-var_vbici_dn2) })) } } else { (assign1150_e1480 * ((var_theexp_t_dn2 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn2 } else { (-var_vbici_dn2) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_dn3 * assign1150_e1470) + (var_is_t * var_le_dn3)) - (((((var_ijbvc_t_dn3 * var_lebv) + (var_ijbvc_t * var_lebv_dn3)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_dn3 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn3 } else { (-var_vbici_dn3) })) } } else { (assign1150_e1480 * ((var_theexp_t_dn3 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn3 } else { (-var_vbici_dn3) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_dn4 * assign1150_e1470) + (var_is_t * var_le_dn4)) - (((((var_ijbvc_t_dn4 * var_lebv) + (var_ijbvc_t * var_lebv_dn4)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_dn4 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn4 } else { (-var_vbici_dn4) })) } } else { (assign1150_e1480 * ((var_theexp_t_dn4 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn4 } else { (-var_vbici_dn4) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_dn5 * assign1150_e1470) + (var_is_t * var_le_dn5)) - (((((var_ijbvc_t_dn5 * var_lebv) + (var_ijbvc_t * var_lebv_dn5)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_dn5 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn5 } else { (-var_vbici_dn5) })) } } else { (assign1150_e1480 * ((var_theexp_t_dn5 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn5 } else { (-var_vbici_dn5) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_dn6 * assign1150_e1470) + (var_is_t * var_le_dn6)) - (((((var_ijbvc_t_dn6 * var_lebv) + (var_ijbvc_t * var_lebv_dn6)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_dn6 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn6 } else { (-var_vbici_dn6) })) } } else { (assign1150_e1480 * ((var_theexp_t_dn6 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn6 } else { (-var_vbici_dn6) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_dn7 * assign1150_e1470) + (var_is_t * var_le_dn7)) - (((((var_ijbvc_t_dn7 * var_lebv) + (var_ijbvc_t * var_lebv_dn7)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_dn7 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn7 } else { (-var_vbici_dn7) })) } } else { (assign1150_e1480 * ((var_theexp_t_dn7 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn7 } else { (-var_vbici_dn7) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_dn8 * assign1150_e1470) + (var_is_t * var_le_dn8)) - (((((var_ijbvc_t_dn8 * var_lebv) + (var_ijbvc_t * var_lebv_dn8)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_dn8 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn8 } else { (-var_vbici_dn8) })) } } else { (assign1150_e1480 * ((var_theexp_t_dn8 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn8 } else { (-var_vbici_dn8) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_dn9 * assign1150_e1470) + (var_is_t * var_le_dn9)) - (((((var_ijbvc_t_dn9 * var_lebv) + (var_ijbvc_t * var_lebv_dn9)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_dn9 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_dn9 } else { (-var_vbici_dn9) })) } } else { (assign1150_e1480 * ((var_theexp_t_dn9 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_dn9 } else { (-var_vbici_dn9) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_db0 * assign1150_e1470) + (var_is_t * var_le_db0)) - (((((var_ijbvc_t_db0 * var_lebv) + (var_ijbvc_t * var_lebv_db0)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_db0 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_db0 } else { (-var_vbici_db0) })) } } else { (assign1150_e1480 * ((var_theexp_t_db0 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_db0 } else { (-var_vbici_db0) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_db1 * assign1150_e1470) + (var_is_t * var_le_db1)) - (((((var_ijbvc_t_db1 * var_lebv) + (var_ijbvc_t * var_lebv_db1)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_db1 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_db1 } else { (-var_vbici_db1) })) } } else { (assign1150_e1480 * ((var_theexp_t_db1 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_db1 } else { (-var_vbici_db1) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_db2 * assign1150_e1470) + (var_is_t * var_le_db2)) - (((((var_ijbvc_t_db2 * var_lebv) + (var_ijbvc_t * var_lebv_db2)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_db2 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_db2 } else { (-var_vbici_db2) })) } } else { (assign1150_e1480 * ((var_theexp_t_db2 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_db2 } else { (-var_vbici_db2) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_db3 * assign1150_e1470) + (var_is_t * var_le_db3)) - (((((var_ijbvc_t_db3 * var_lebv) + (var_ijbvc_t * var_lebv_db3)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_db3 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_db3 } else { (-var_vbici_db3) })) } } else { (assign1150_e1480 * ((var_theexp_t_db3 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_db3 } else { (-var_vbici_db3) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_db4 * assign1150_e1470) + (var_is_t * var_le_db4)) - (((((var_ijbvc_t_db4 * var_lebv) + (var_ijbvc_t * var_lebv_db4)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_db4 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_db4 } else { (-var_vbici_db4) })) } } else { (assign1150_e1480 * ((var_theexp_t_db4 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_db4 } else { (-var_vbici_db4) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_db5 * assign1150_e1470) + (var_is_t * var_le_db5)) - (((((var_ijbvc_t_db5 * var_lebv) + (var_ijbvc_t * var_lebv_db5)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_db5 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_db5 } else { (-var_vbici_db5) })) } } else { (assign1150_e1480 * ((var_theexp_t_db5 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_db5 } else { (-var_vbici_db5) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_db6 * assign1150_e1470) + (var_is_t * var_le_db6)) - (((((var_ijbvc_t_db6 * var_lebv) + (var_ijbvc_t * var_lebv_db6)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_db6 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_db6 } else { (-var_vbici_db6) })) } } else { (assign1150_e1480 * ((var_theexp_t_db6 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_db6 } else { (-var_vbici_db6) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))), (((var_is_t_db7 * assign1150_e1470) + (var_is_t * var_le_db7)) - (((((var_ijbvc_t_db7 * var_lebv) + (var_ijbvc_t * var_lebv_db7)) * assign1150_e1482) - (assign1150_e1474 * (p.p8 * if var_theexp_t_db7 == 0.0 && ((var_theexp_t) as f64).is_finite() && ((var_theexp_t) as f64).fract() == 0.0 { if var_theexp_t == 0.0 { 0.0 } else { (var_theexp_t * ((assign1150_e1478).powf(var_theexp_t - 1.0) * if var_vbici >= 0.0 { var_vbici_db7 } else { (-var_vbici_db7) })) } } else { (assign1150_e1480 * ((var_theexp_t_db7 * (assign1150_e1478).ln()) + (var_theexp_t * (if var_vbici >= 0.0 { var_vbici_db7 } else { (-var_vbici_db7) } / assign1150_e1478)))) }))) / (assign1150_e1482 * assign1150_e1482))),)
    } else {
        (var_ibwd, var_ibwd_dn0, var_ibwd_dn1, var_ibwd_dn2, var_ibwd_dn3, var_ibwd_dn4, var_ibwd_dn5, var_ibwd_dn6, var_ibwd_dn7, var_ibwd_dn8, var_ibwd_dn9, var_ibwd_db0, var_ibwd_db1, var_ibwd_db2, var_ibwd_db3, var_ibwd_db4, var_ibwd_db5, var_ibwd_db6, var_ibwd_db7,)
    }
};
        var_ibwd = assign1150_e1486;
        var_ibwd_dn0 = assign1150_e1486_d_n0;
        var_ibwd_dn1 = assign1150_e1486_d_n1;
        var_ibwd_dn2 = assign1150_e1486_d_n2;
        var_ibwd_dn3 = assign1150_e1486_d_n3;
        var_ibwd_dn4 = assign1150_e1486_d_n4;
        var_ibwd_dn5 = assign1150_e1486_d_n5;
        var_ibwd_dn6 = assign1150_e1486_d_n6;
        var_ibwd_dn7 = assign1150_e1486_d_n7;
        var_ibwd_dn8 = assign1150_e1486_d_n8;
        var_ibwd_dn9 = assign1150_e1486_d_n9;
        var_ibwd_db0 = assign1150_e1486_d_b0;
        var_ibwd_db1 = assign1150_e1486_d_b1;
        var_ibwd_db2 = assign1150_e1486_d_b2;
        var_ibwd_db3 = assign1150_e1486_d_b3;
        var_ibwd_db4 = assign1150_e1486_d_b4;
        var_ibwd_db5 = assign1150_e1486_d_b5;
        var_ibwd_db6 = assign1150_e1486_d_b6;
        var_ibwd_db7 = assign1150_e1486_d_b7;
        var_ibwd_rv = 0.0;
        var_ibwd_rdn0 = 0.0;
        var_ibwd_rdn1 = 0.0;
        var_ibwd_rdn2 = 0.0;
        var_ibwd_rdn3 = 0.0;
        var_ibwd_rdn4 = 0.0;
        var_ibwd_rdn5 = 0.0;
        var_ibwd_rdn6 = 0.0;
        var_ibwd_rdn7 = 0.0;
        var_ibwd_rdn8 = 0.0;
        var_ibwd_rdn9 = 0.0;
        var_ibwd_rdb0 = 0.0;
        var_ibwd_rdb1 = 0.0;
        var_ibwd_rdb2 = 0.0;
        var_ibwd_rdb3 = 0.0;
        var_ibwd_rdb4 = 0.0;
        var_ibwd_rdb5 = 0.0;
        var_ibwd_rdb6 = 0.0;
        var_ibwd_rdb7 = 0.0;

        let (assign1160_e1491, assign1160_e1491_d_n0, assign1160_e1491_d_n1, assign1160_e1491_d_n2, assign1160_e1491_d_n3, assign1160_e1491_d_n4, assign1160_e1491_d_n5, assign1160_e1491_d_n6, assign1160_e1491_d_n7, assign1160_e1491_d_n8, assign1160_e1491_d_n9, assign1160_e1491_d_b0, assign1160_e1491_d_b1, assign1160_e1491_d_b2, assign1160_e1491_d_b3, assign1160_e1491_d_b4, assign1160_e1491_d_b5, assign1160_e1491_d_b6, assign1160_e1491_d_b7,) = {
    if (var_guard9 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibwd, var_ibwd_dn0, var_ibwd_dn1, var_ibwd_dn2, var_ibwd_dn3, var_ibwd_dn4, var_ibwd_dn5, var_ibwd_dn6, var_ibwd_dn7, var_ibwd_dn8, var_ibwd_dn9, var_ibwd_db0, var_ibwd_db1, var_ibwd_db2, var_ibwd_db3, var_ibwd_db4, var_ibwd_db5, var_ibwd_db6, var_ibwd_db7,)
    }
};
        var_ibwd = assign1160_e1491;
        var_ibwd_dn0 = assign1160_e1491_d_n0;
        var_ibwd_dn1 = assign1160_e1491_d_n1;
        var_ibwd_dn2 = assign1160_e1491_d_n2;
        var_ibwd_dn3 = assign1160_e1491_d_n3;
        var_ibwd_dn4 = assign1160_e1491_d_n4;
        var_ibwd_dn5 = assign1160_e1491_d_n5;
        var_ibwd_dn6 = assign1160_e1491_d_n6;
        var_ibwd_dn7 = assign1160_e1491_d_n7;
        var_ibwd_dn8 = assign1160_e1491_d_n8;
        var_ibwd_dn9 = assign1160_e1491_d_n9;
        var_ibwd_db0 = assign1160_e1491_d_b0;
        var_ibwd_db1 = assign1160_e1491_d_b1;
        var_ibwd_db2 = assign1160_e1491_d_b2;
        var_ibwd_db3 = assign1160_e1491_d_b3;
        var_ibwd_db4 = assign1160_e1491_d_b4;
        var_ibwd_db5 = assign1160_e1491_d_b5;
        var_ibwd_db6 = assign1160_e1491_d_b6;
        var_ibwd_db7 = assign1160_e1491_d_b7;
        var_ibwd_rv = 0.0;
        var_ibwd_rdn0 = 0.0;
        var_ibwd_rdn1 = 0.0;
        var_ibwd_rdn2 = 0.0;
        var_ibwd_rdn3 = 0.0;
        var_ibwd_rdn4 = 0.0;
        var_ibwd_rdn5 = 0.0;
        var_ibwd_rdn6 = 0.0;
        var_ibwd_rdn7 = 0.0;
        var_ibwd_rdn8 = 0.0;
        var_ibwd_rdn9 = 0.0;
        var_ibwd_rdb0 = 0.0;
        var_ibwd_rdb1 = 0.0;
        var_ibwd_rdb2 = 0.0;
        var_ibwd_rdb3 = 0.0;
        var_ibwd_rdb4 = 0.0;
        var_ibwd_rdb5 = 0.0;
        var_ibwd_rdb6 = 0.0;
        var_ibwd_rdb7 = 0.0;

        let assign1170_e1494: f64 = if var_isc_t > 0.0 { 1.0 } else { 0.0 };
        var_guard11 = assign1170_e1494;
        var_guard11_dn0 = 0.0;
        var_guard11_dn1 = 0.0;
        var_guard11_dn2 = 0.0;
        var_guard11_dn3 = 0.0;
        var_guard11_dn4 = 0.0;
        var_guard11_dn5 = 0.0;
        var_guard11_dn6 = 0.0;
        var_guard11_dn7 = 0.0;
        var_guard11_dn8 = 0.0;
        var_guard11_dn9 = 0.0;
        var_guard11_db0 = 0.0;
        var_guard11_db1 = 0.0;
        var_guard11_db2 = 0.0;
        var_guard11_db3 = 0.0;
        var_guard11_db4 = 0.0;
        var_guard11_db5 = 0.0;
        var_guard11_db6 = 0.0;
        var_guard11_db7 = 0.0;
        var_guard11_rv = 0.0;
        var_guard11_rdn0 = 0.0;
        var_guard11_rdn1 = 0.0;
        var_guard11_rdn2 = 0.0;
        var_guard11_rdn3 = 0.0;
        var_guard11_rdn4 = 0.0;
        var_guard11_rdn5 = 0.0;
        var_guard11_rdn6 = 0.0;
        var_guard11_rdn7 = 0.0;
        var_guard11_rdn8 = 0.0;
        var_guard11_rdn9 = 0.0;
        var_guard11_rdb0 = 0.0;
        var_guard11_rdb1 = 0.0;
        var_guard11_rdb2 = 0.0;
        var_guard11_rdb3 = 0.0;
        var_guard11_rdb4 = 0.0;
        var_guard11_rdb5 = 0.0;
        var_guard11_rdb6 = 0.0;
        var_guard11_rdb7 = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_db0_slot = var_arg_db0;
        *var_arg_db1_slot = var_arg_db1;
        *var_arg_db2_slot = var_arg_db2;
        *var_arg_db3_slot = var_arg_db3;
        *var_arg_db4_slot = var_arg_db4;
        *var_arg_db5_slot = var_arg_db5;
        *var_arg_db6_slot = var_arg_db6;
        *var_arg_db7_slot = var_arg_db7;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn1_slot = var_arg_dn1;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_arg_rdb0_slot = var_arg_rdb0;
        *var_arg_rdb1_slot = var_arg_rdb1;
        *var_arg_rdb2_slot = var_arg_rdb2;
        *var_arg_rdb3_slot = var_arg_rdb3;
        *var_arg_rdb4_slot = var_arg_rdb4;
        *var_arg_rdb5_slot = var_arg_rdb5;
        *var_arg_rdb6_slot = var_arg_rdb6;
        *var_arg_rdb7_slot = var_arg_rdb7;
        *var_arg_rdn0_slot = var_arg_rdn0;
        *var_arg_rdn1_slot = var_arg_rdn1;
        *var_arg_rdn2_slot = var_arg_rdn2;
        *var_arg_rdn3_slot = var_arg_rdn3;
        *var_arg_rdn4_slot = var_arg_rdn4;
        *var_arg_rdn5_slot = var_arg_rdn5;
        *var_arg_rdn6_slot = var_arg_rdn6;
        *var_arg_rdn7_slot = var_arg_rdn7;
        *var_arg_rdn8_slot = var_arg_rdn8;
        *var_arg_rdn9_slot = var_arg_rdn9;
        *var_arg_rv_slot = var_arg_rv;
        *var_guard10_slot = var_guard10;
        *var_guard10_db0_slot = var_guard10_db0;
        *var_guard10_db1_slot = var_guard10_db1;
        *var_guard10_db2_slot = var_guard10_db2;
        *var_guard10_db3_slot = var_guard10_db3;
        *var_guard10_db4_slot = var_guard10_db4;
        *var_guard10_db5_slot = var_guard10_db5;
        *var_guard10_db6_slot = var_guard10_db6;
        *var_guard10_db7_slot = var_guard10_db7;
        *var_guard10_dn0_slot = var_guard10_dn0;
        *var_guard10_dn1_slot = var_guard10_dn1;
        *var_guard10_dn2_slot = var_guard10_dn2;
        *var_guard10_dn3_slot = var_guard10_dn3;
        *var_guard10_dn4_slot = var_guard10_dn4;
        *var_guard10_dn5_slot = var_guard10_dn5;
        *var_guard10_dn6_slot = var_guard10_dn6;
        *var_guard10_dn7_slot = var_guard10_dn7;
        *var_guard10_dn8_slot = var_guard10_dn8;
        *var_guard10_dn9_slot = var_guard10_dn9;
        *var_guard10_rdb0_slot = var_guard10_rdb0;
        *var_guard10_rdb1_slot = var_guard10_rdb1;
        *var_guard10_rdb2_slot = var_guard10_rdb2;
        *var_guard10_rdb3_slot = var_guard10_rdb3;
        *var_guard10_rdb4_slot = var_guard10_rdb4;
        *var_guard10_rdb5_slot = var_guard10_rdb5;
        *var_guard10_rdb6_slot = var_guard10_rdb6;
        *var_guard10_rdb7_slot = var_guard10_rdb7;
        *var_guard10_rdn0_slot = var_guard10_rdn0;
        *var_guard10_rdn1_slot = var_guard10_rdn1;
        *var_guard10_rdn2_slot = var_guard10_rdn2;
        *var_guard10_rdn3_slot = var_guard10_rdn3;
        *var_guard10_rdn4_slot = var_guard10_rdn4;
        *var_guard10_rdn5_slot = var_guard10_rdn5;
        *var_guard10_rdn6_slot = var_guard10_rdn6;
        *var_guard10_rdn7_slot = var_guard10_rdn7;
        *var_guard10_rdn8_slot = var_guard10_rdn8;
        *var_guard10_rdn9_slot = var_guard10_rdn9;
        *var_guard10_rv_slot = var_guard10_rv;
        *var_guard11_slot = var_guard11;
        *var_guard11_db0_slot = var_guard11_db0;
        *var_guard11_db1_slot = var_guard11_db1;
        *var_guard11_db2_slot = var_guard11_db2;
        *var_guard11_db3_slot = var_guard11_db3;
        *var_guard11_db4_slot = var_guard11_db4;
        *var_guard11_db5_slot = var_guard11_db5;
        *var_guard11_db6_slot = var_guard11_db6;
        *var_guard11_db7_slot = var_guard11_db7;
        *var_guard11_dn0_slot = var_guard11_dn0;
        *var_guard11_dn1_slot = var_guard11_dn1;
        *var_guard11_dn2_slot = var_guard11_dn2;
        *var_guard11_dn3_slot = var_guard11_dn3;
        *var_guard11_dn4_slot = var_guard11_dn4;
        *var_guard11_dn5_slot = var_guard11_dn5;
        *var_guard11_dn6_slot = var_guard11_dn6;
        *var_guard11_dn7_slot = var_guard11_dn7;
        *var_guard11_dn8_slot = var_guard11_dn8;
        *var_guard11_dn9_slot = var_guard11_dn9;
        *var_guard11_rdb0_slot = var_guard11_rdb0;
        *var_guard11_rdb1_slot = var_guard11_rdb1;
        *var_guard11_rdb2_slot = var_guard11_rdb2;
        *var_guard11_rdb3_slot = var_guard11_rdb3;
        *var_guard11_rdb4_slot = var_guard11_rdb4;
        *var_guard11_rdb5_slot = var_guard11_rdb5;
        *var_guard11_rdb6_slot = var_guard11_rdb6;
        *var_guard11_rdb7_slot = var_guard11_rdb7;
        *var_guard11_rdn0_slot = var_guard11_rdn0;
        *var_guard11_rdn1_slot = var_guard11_rdn1;
        *var_guard11_rdn2_slot = var_guard11_rdn2;
        *var_guard11_rdn3_slot = var_guard11_rdn3;
        *var_guard11_rdn4_slot = var_guard11_rdn4;
        *var_guard11_rdn5_slot = var_guard11_rdn5;
        *var_guard11_rdn6_slot = var_guard11_rdn6;
        *var_guard11_rdn7_slot = var_guard11_rdn7;
        *var_guard11_rdn8_slot = var_guard11_rdn8;
        *var_guard11_rdn9_slot = var_guard11_rdn9;
        *var_guard11_rv_slot = var_guard11_rv;
        *var_ibwd_slot = var_ibwd;
        *var_ibwd_db0_slot = var_ibwd_db0;
        *var_ibwd_db1_slot = var_ibwd_db1;
        *var_ibwd_db2_slot = var_ibwd_db2;
        *var_ibwd_db3_slot = var_ibwd_db3;
        *var_ibwd_db4_slot = var_ibwd_db4;
        *var_ibwd_db5_slot = var_ibwd_db5;
        *var_ibwd_db6_slot = var_ibwd_db6;
        *var_ibwd_db7_slot = var_ibwd_db7;
        *var_ibwd_dn0_slot = var_ibwd_dn0;
        *var_ibwd_dn1_slot = var_ibwd_dn1;
        *var_ibwd_dn2_slot = var_ibwd_dn2;
        *var_ibwd_dn3_slot = var_ibwd_dn3;
        *var_ibwd_dn4_slot = var_ibwd_dn4;
        *var_ibwd_dn5_slot = var_ibwd_dn5;
        *var_ibwd_dn6_slot = var_ibwd_dn6;
        *var_ibwd_dn7_slot = var_ibwd_dn7;
        *var_ibwd_dn8_slot = var_ibwd_dn8;
        *var_ibwd_dn9_slot = var_ibwd_dn9;
        *var_ibwd_rdb0_slot = var_ibwd_rdb0;
        *var_ibwd_rdb1_slot = var_ibwd_rdb1;
        *var_ibwd_rdb2_slot = var_ibwd_rdb2;
        *var_ibwd_rdb3_slot = var_ibwd_rdb3;
        *var_ibwd_rdb4_slot = var_ibwd_rdb4;
        *var_ibwd_rdb5_slot = var_ibwd_rdb5;
        *var_ibwd_rdb6_slot = var_ibwd_rdb6;
        *var_ibwd_rdb7_slot = var_ibwd_rdb7;
        *var_ibwd_rdn0_slot = var_ibwd_rdn0;
        *var_ibwd_rdn1_slot = var_ibwd_rdn1;
        *var_ibwd_rdn2_slot = var_ibwd_rdn2;
        *var_ibwd_rdn3_slot = var_ibwd_rdn3;
        *var_ibwd_rdn4_slot = var_ibwd_rdn4;
        *var_ibwd_rdn5_slot = var_ibwd_rdn5;
        *var_ibwd_rdn6_slot = var_ibwd_rdn6;
        *var_ibwd_rdn7_slot = var_ibwd_rdn7;
        *var_ibwd_rdn8_slot = var_ibwd_rdn8;
        *var_ibwd_rdn9_slot = var_ibwd_rdn9;
        *var_ibwd_rv_slot = var_ibwd_rv;
        *var_le_slot = var_le;
        *var_le_db0_slot = var_le_db0;
        *var_le_db1_slot = var_le_db1;
        *var_le_db2_slot = var_le_db2;
        *var_le_db3_slot = var_le_db3;
        *var_le_db4_slot = var_le_db4;
        *var_le_db5_slot = var_le_db5;
        *var_le_db6_slot = var_le_db6;
        *var_le_db7_slot = var_le_db7;
        *var_le_dn0_slot = var_le_dn0;
        *var_le_dn1_slot = var_le_dn1;
        *var_le_dn2_slot = var_le_dn2;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn5_slot = var_le_dn5;
        *var_le_dn6_slot = var_le_dn6;
        *var_le_dn7_slot = var_le_dn7;
        *var_le_dn8_slot = var_le_dn8;
        *var_le_dn9_slot = var_le_dn9;
        *var_le_rdb0_slot = var_le_rdb0;
        *var_le_rdb1_slot = var_le_rdb1;
        *var_le_rdb2_slot = var_le_rdb2;
        *var_le_rdb3_slot = var_le_rdb3;
        *var_le_rdb4_slot = var_le_rdb4;
        *var_le_rdb5_slot = var_le_rdb5;
        *var_le_rdb6_slot = var_le_rdb6;
        *var_le_rdb7_slot = var_le_rdb7;
        *var_le_rdn0_slot = var_le_rdn0;
        *var_le_rdn1_slot = var_le_rdn1;
        *var_le_rdn2_slot = var_le_rdn2;
        *var_le_rdn3_slot = var_le_rdn3;
        *var_le_rdn4_slot = var_le_rdn4;
        *var_le_rdn5_slot = var_le_rdn5;
        *var_le_rdn6_slot = var_le_rdn6;
        *var_le_rdn7_slot = var_le_rdn7;
        *var_le_rdn8_slot = var_le_rdn8;
        *var_le_rdn9_slot = var_le_rdn9;
        *var_le_rv_slot = var_le_rv;
        *var_lebv_slot = var_lebv;
        *var_lebv_db0_slot = var_lebv_db0;
        *var_lebv_db1_slot = var_lebv_db1;
        *var_lebv_db2_slot = var_lebv_db2;
        *var_lebv_db3_slot = var_lebv_db3;
        *var_lebv_db4_slot = var_lebv_db4;
        *var_lebv_db5_slot = var_lebv_db5;
        *var_lebv_db6_slot = var_lebv_db6;
        *var_lebv_db7_slot = var_lebv_db7;
        *var_lebv_dn0_slot = var_lebv_dn0;
        *var_lebv_dn1_slot = var_lebv_dn1;
        *var_lebv_dn2_slot = var_lebv_dn2;
        *var_lebv_dn3_slot = var_lebv_dn3;
        *var_lebv_dn4_slot = var_lebv_dn4;
        *var_lebv_dn5_slot = var_lebv_dn5;
        *var_lebv_dn6_slot = var_lebv_dn6;
        *var_lebv_dn7_slot = var_lebv_dn7;
        *var_lebv_dn8_slot = var_lebv_dn8;
        *var_lebv_dn9_slot = var_lebv_dn9;
        *var_lebv_rdb0_slot = var_lebv_rdb0;
        *var_lebv_rdb1_slot = var_lebv_rdb1;
        *var_lebv_rdb2_slot = var_lebv_rdb2;
        *var_lebv_rdb3_slot = var_lebv_rdb3;
        *var_lebv_rdb4_slot = var_lebv_rdb4;
        *var_lebv_rdb5_slot = var_lebv_rdb5;
        *var_lebv_rdb6_slot = var_lebv_rdb6;
        *var_lebv_rdb7_slot = var_lebv_rdb7;
        *var_lebv_rdn0_slot = var_lebv_rdn0;
        *var_lebv_rdn1_slot = var_lebv_rdn1;
        *var_lebv_rdn2_slot = var_lebv_rdn2;
        *var_lebv_rdn3_slot = var_lebv_rdn3;
        *var_lebv_rdn4_slot = var_lebv_rdn4;
        *var_lebv_rdn5_slot = var_lebv_rdn5;
        *var_lebv_rdn6_slot = var_lebv_rdn6;
        *var_lebv_rdn7_slot = var_lebv_rdn7;
        *var_lebv_rdn8_slot = var_lebv_rdn8;
        *var_lebv_rdn9_slot = var_lebv_rdn9;
        *var_lebv_rv_slot = var_lebv_rv;
    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        var_bvr_t: f64,
        var_bvr_t_db0: f64,
        var_bvr_t_db1: f64,
        var_bvr_t_db2: f64,
        var_bvr_t_db3: f64,
        var_bvr_t_db4: f64,
        var_bvr_t_db5: f64,
        var_bvr_t_db6: f64,
        var_bvr_t_db7: f64,
        var_bvr_t_dn0: f64,
        var_bvr_t_dn1: f64,
        var_bvr_t_dn2: f64,
        var_bvr_t_dn3: f64,
        var_bvr_t_dn4: f64,
        var_bvr_t_dn5: f64,
        var_bvr_t_dn6: f64,
        var_bvr_t_dn7: f64,
        var_bvr_t_dn8: f64,
        var_bvr_t_dn9: f64,
        var_guard11: f64,
        var_vbici: f64,
        var_vbici_db0: f64,
        var_vbici_db1: f64,
        var_vbici_db2: f64,
        var_vbici_db3: f64,
        var_vbici_db4: f64,
        var_vbici_db5: f64,
        var_vbici_db6: f64,
        var_vbici_db7: f64,
        var_vbici_dn0: f64,
        var_vbici_dn1: f64,
        var_vbici_dn2: f64,
        var_vbici_dn3: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbici_dn6: f64,
        var_vbici_dn7: f64,
        var_vbici_dn8: f64,
        var_vbici_dn9: f64,
        var_vt: f64,
        var_vt_db0: f64,
        var_vt_db1: f64,
        var_vt_db2: f64,
        var_vt_db3: f64,
        var_vt_db4: f64,
        var_vt_db5: f64,
        var_vt_db6: f64,
        var_vt_db7: f64,
        var_vt_dn0: f64,
        var_vt_dn1: f64,
        var_vt_dn2: f64,
        var_vt_dn3: f64,
        var_vt_dn4: f64,
        var_vt_dn5: f64,
        var_vt_dn6: f64,
        var_vt_dn7: f64,
        var_vt_dn8: f64,
        var_vt_dn9: f64,
        var_arg_slot: &mut f64,
        var_arg_db0_slot: &mut f64,
        var_arg_db1_slot: &mut f64,
        var_arg_db2_slot: &mut f64,
        var_arg_db3_slot: &mut f64,
        var_arg_db4_slot: &mut f64,
        var_arg_db5_slot: &mut f64,
        var_arg_db6_slot: &mut f64,
        var_arg_db7_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn1_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_arg_rdb0_slot: &mut f64,
        var_arg_rdb1_slot: &mut f64,
        var_arg_rdb2_slot: &mut f64,
        var_arg_rdb3_slot: &mut f64,
        var_arg_rdb4_slot: &mut f64,
        var_arg_rdb5_slot: &mut f64,
        var_arg_rdb6_slot: &mut f64,
        var_arg_rdb7_slot: &mut f64,
        var_arg_rdn0_slot: &mut f64,
        var_arg_rdn1_slot: &mut f64,
        var_arg_rdn2_slot: &mut f64,
        var_arg_rdn3_slot: &mut f64,
        var_arg_rdn4_slot: &mut f64,
        var_arg_rdn5_slot: &mut f64,
        var_arg_rdn6_slot: &mut f64,
        var_arg_rdn7_slot: &mut f64,
        var_arg_rdn8_slot: &mut f64,
        var_arg_rdn9_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_argbv_slot: &mut f64,
        var_argbv_db0_slot: &mut f64,
        var_argbv_db1_slot: &mut f64,
        var_argbv_db2_slot: &mut f64,
        var_argbv_db3_slot: &mut f64,
        var_argbv_db4_slot: &mut f64,
        var_argbv_db5_slot: &mut f64,
        var_argbv_db6_slot: &mut f64,
        var_argbv_db7_slot: &mut f64,
        var_argbv_dn0_slot: &mut f64,
        var_argbv_dn1_slot: &mut f64,
        var_argbv_dn2_slot: &mut f64,
        var_argbv_dn3_slot: &mut f64,
        var_argbv_dn4_slot: &mut f64,
        var_argbv_dn5_slot: &mut f64,
        var_argbv_dn6_slot: &mut f64,
        var_argbv_dn7_slot: &mut f64,
        var_argbv_dn8_slot: &mut f64,
        var_argbv_dn9_slot: &mut f64,
        var_argbv_rdb0_slot: &mut f64,
        var_argbv_rdb1_slot: &mut f64,
        var_argbv_rdb2_slot: &mut f64,
        var_argbv_rdb3_slot: &mut f64,
        var_argbv_rdb4_slot: &mut f64,
        var_argbv_rdb5_slot: &mut f64,
        var_argbv_rdb6_slot: &mut f64,
        var_argbv_rdb7_slot: &mut f64,
        var_argbv_rdn0_slot: &mut f64,
        var_argbv_rdn1_slot: &mut f64,
        var_argbv_rdn2_slot: &mut f64,
        var_argbv_rdn3_slot: &mut f64,
        var_argbv_rdn4_slot: &mut f64,
        var_argbv_rdn5_slot: &mut f64,
        var_argbv_rdn6_slot: &mut f64,
        var_argbv_rdn7_slot: &mut f64,
        var_argbv_rdn8_slot: &mut f64,
        var_argbv_rdn9_slot: &mut f64,
        var_argbv_rv_slot: &mut f64,
        var_argbvvt_slot: &mut f64,
        var_argbvvt_db0_slot: &mut f64,
        var_argbvvt_db1_slot: &mut f64,
        var_argbvvt_db2_slot: &mut f64,
        var_argbvvt_db3_slot: &mut f64,
        var_argbvvt_db4_slot: &mut f64,
        var_argbvvt_db5_slot: &mut f64,
        var_argbvvt_db6_slot: &mut f64,
        var_argbvvt_db7_slot: &mut f64,
        var_argbvvt_dn0_slot: &mut f64,
        var_argbvvt_dn1_slot: &mut f64,
        var_argbvvt_dn2_slot: &mut f64,
        var_argbvvt_dn3_slot: &mut f64,
        var_argbvvt_dn4_slot: &mut f64,
        var_argbvvt_dn5_slot: &mut f64,
        var_argbvvt_dn6_slot: &mut f64,
        var_argbvvt_dn7_slot: &mut f64,
        var_argbvvt_dn8_slot: &mut f64,
        var_argbvvt_dn9_slot: &mut f64,
        var_argbvvt_rdb0_slot: &mut f64,
        var_argbvvt_rdb1_slot: &mut f64,
        var_argbvvt_rdb2_slot: &mut f64,
        var_argbvvt_rdb3_slot: &mut f64,
        var_argbvvt_rdb4_slot: &mut f64,
        var_argbvvt_rdb5_slot: &mut f64,
        var_argbvvt_rdb6_slot: &mut f64,
        var_argbvvt_rdb7_slot: &mut f64,
        var_argbvvt_rdn0_slot: &mut f64,
        var_argbvvt_rdn1_slot: &mut f64,
        var_argbvvt_rdn2_slot: &mut f64,
        var_argbvvt_rdn3_slot: &mut f64,
        var_argbvvt_rdn4_slot: &mut f64,
        var_argbvvt_rdn5_slot: &mut f64,
        var_argbvvt_rdn6_slot: &mut f64,
        var_argbvvt_rdn7_slot: &mut f64,
        var_argbvvt_rdn8_slot: &mut f64,
        var_argbvvt_rdn9_slot: &mut f64,
        var_argbvvt_rv_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_guard12_db0_slot: &mut f64,
        var_guard12_db1_slot: &mut f64,
        var_guard12_db2_slot: &mut f64,
        var_guard12_db3_slot: &mut f64,
        var_guard12_db4_slot: &mut f64,
        var_guard12_db5_slot: &mut f64,
        var_guard12_db6_slot: &mut f64,
        var_guard12_db7_slot: &mut f64,
        var_guard12_dn0_slot: &mut f64,
        var_guard12_dn1_slot: &mut f64,
        var_guard12_dn2_slot: &mut f64,
        var_guard12_dn3_slot: &mut f64,
        var_guard12_dn4_slot: &mut f64,
        var_guard12_dn5_slot: &mut f64,
        var_guard12_dn6_slot: &mut f64,
        var_guard12_dn7_slot: &mut f64,
        var_guard12_dn8_slot: &mut f64,
        var_guard12_dn9_slot: &mut f64,
        var_guard12_rdb0_slot: &mut f64,
        var_guard12_rdb1_slot: &mut f64,
        var_guard12_rdb2_slot: &mut f64,
        var_guard12_rdb3_slot: &mut f64,
        var_guard12_rdb4_slot: &mut f64,
        var_guard12_rdb5_slot: &mut f64,
        var_guard12_rdb6_slot: &mut f64,
        var_guard12_rdb7_slot: &mut f64,
        var_guard12_rdn0_slot: &mut f64,
        var_guard12_rdn1_slot: &mut f64,
        var_guard12_rdn2_slot: &mut f64,
        var_guard12_rdn3_slot: &mut f64,
        var_guard12_rdn4_slot: &mut f64,
        var_guard12_rdn5_slot: &mut f64,
        var_guard12_rdn6_slot: &mut f64,
        var_guard12_rdn7_slot: &mut f64,
        var_guard12_rdn8_slot: &mut f64,
        var_guard12_rdn9_slot: &mut f64,
        var_guard12_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_db0_slot: &mut f64,
        var_le_db1_slot: &mut f64,
        var_le_db2_slot: &mut f64,
        var_le_db3_slot: &mut f64,
        var_le_db4_slot: &mut f64,
        var_le_db5_slot: &mut f64,
        var_le_db6_slot: &mut f64,
        var_le_db7_slot: &mut f64,
        var_le_dn0_slot: &mut f64,
        var_le_dn1_slot: &mut f64,
        var_le_dn2_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn5_slot: &mut f64,
        var_le_dn6_slot: &mut f64,
        var_le_dn7_slot: &mut f64,
        var_le_dn8_slot: &mut f64,
        var_le_dn9_slot: &mut f64,
        var_le_rdb0_slot: &mut f64,
        var_le_rdb1_slot: &mut f64,
        var_le_rdb2_slot: &mut f64,
        var_le_rdb3_slot: &mut f64,
        var_le_rdb4_slot: &mut f64,
        var_le_rdb5_slot: &mut f64,
        var_le_rdb6_slot: &mut f64,
        var_le_rdb7_slot: &mut f64,
        var_le_rdn0_slot: &mut f64,
        var_le_rdn1_slot: &mut f64,
        var_le_rdn2_slot: &mut f64,
        var_le_rdn3_slot: &mut f64,
        var_le_rdn4_slot: &mut f64,
        var_le_rdn5_slot: &mut f64,
        var_le_rdn6_slot: &mut f64,
        var_le_rdn7_slot: &mut f64,
        var_le_rdn8_slot: &mut f64,
        var_le_rdn9_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_lebv_slot: &mut f64,
        var_lebv_db0_slot: &mut f64,
        var_lebv_db1_slot: &mut f64,
        var_lebv_db2_slot: &mut f64,
        var_lebv_db3_slot: &mut f64,
        var_lebv_db4_slot: &mut f64,
        var_lebv_db5_slot: &mut f64,
        var_lebv_db6_slot: &mut f64,
        var_lebv_db7_slot: &mut f64,
        var_lebv_dn0_slot: &mut f64,
        var_lebv_dn1_slot: &mut f64,
        var_lebv_dn2_slot: &mut f64,
        var_lebv_dn3_slot: &mut f64,
        var_lebv_dn4_slot: &mut f64,
        var_lebv_dn5_slot: &mut f64,
        var_lebv_dn6_slot: &mut f64,
        var_lebv_dn7_slot: &mut f64,
        var_lebv_dn8_slot: &mut f64,
        var_lebv_dn9_slot: &mut f64,
        var_lebv_rdb0_slot: &mut f64,
        var_lebv_rdb1_slot: &mut f64,
        var_lebv_rdb2_slot: &mut f64,
        var_lebv_rdb3_slot: &mut f64,
        var_lebv_rdb4_slot: &mut f64,
        var_lebv_rdb5_slot: &mut f64,
        var_lebv_rdb6_slot: &mut f64,
        var_lebv_rdb7_slot: &mut f64,
        var_lebv_rdn0_slot: &mut f64,
        var_lebv_rdn1_slot: &mut f64,
        var_lebv_rdn2_slot: &mut f64,
        var_lebv_rdn3_slot: &mut f64,
        var_lebv_rdn4_slot: &mut f64,
        var_lebv_rdn5_slot: &mut f64,
        var_lebv_rdn6_slot: &mut f64,
        var_lebv_rdn7_slot: &mut f64,
        var_lebv_rdn8_slot: &mut f64,
        var_lebv_rdn9_slot: &mut f64,
        var_lebv_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_db0: f64 = *var_arg_db0_slot;
        let mut var_arg_db1: f64 = *var_arg_db1_slot;
        let mut var_arg_db2: f64 = *var_arg_db2_slot;
        let mut var_arg_db3: f64 = *var_arg_db3_slot;
        let mut var_arg_db4: f64 = *var_arg_db4_slot;
        let mut var_arg_db5: f64 = *var_arg_db5_slot;
        let mut var_arg_db6: f64 = *var_arg_db6_slot;
        let mut var_arg_db7: f64 = *var_arg_db7_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn1: f64 = *var_arg_dn1_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_arg_rdb0: f64 = *var_arg_rdb0_slot;
        let mut var_arg_rdb1: f64 = *var_arg_rdb1_slot;
        let mut var_arg_rdb2: f64 = *var_arg_rdb2_slot;
        let mut var_arg_rdb3: f64 = *var_arg_rdb3_slot;
        let mut var_arg_rdb4: f64 = *var_arg_rdb4_slot;
        let mut var_arg_rdb5: f64 = *var_arg_rdb5_slot;
        let mut var_arg_rdb6: f64 = *var_arg_rdb6_slot;
        let mut var_arg_rdb7: f64 = *var_arg_rdb7_slot;
        let mut var_arg_rdn0: f64 = *var_arg_rdn0_slot;
        let mut var_arg_rdn1: f64 = *var_arg_rdn1_slot;
        let mut var_arg_rdn2: f64 = *var_arg_rdn2_slot;
        let mut var_arg_rdn3: f64 = *var_arg_rdn3_slot;
        let mut var_arg_rdn4: f64 = *var_arg_rdn4_slot;
        let mut var_arg_rdn5: f64 = *var_arg_rdn5_slot;
        let mut var_arg_rdn6: f64 = *var_arg_rdn6_slot;
        let mut var_arg_rdn7: f64 = *var_arg_rdn7_slot;
        let mut var_arg_rdn8: f64 = *var_arg_rdn8_slot;
        let mut var_arg_rdn9: f64 = *var_arg_rdn9_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_argbv: f64 = *var_argbv_slot;
        let mut var_argbv_db0: f64 = *var_argbv_db0_slot;
        let mut var_argbv_db1: f64 = *var_argbv_db1_slot;
        let mut var_argbv_db2: f64 = *var_argbv_db2_slot;
        let mut var_argbv_db3: f64 = *var_argbv_db3_slot;
        let mut var_argbv_db4: f64 = *var_argbv_db4_slot;
        let mut var_argbv_db5: f64 = *var_argbv_db5_slot;
        let mut var_argbv_db6: f64 = *var_argbv_db6_slot;
        let mut var_argbv_db7: f64 = *var_argbv_db7_slot;
        let mut var_argbv_dn0: f64 = *var_argbv_dn0_slot;
        let mut var_argbv_dn1: f64 = *var_argbv_dn1_slot;
        let mut var_argbv_dn2: f64 = *var_argbv_dn2_slot;
        let mut var_argbv_dn3: f64 = *var_argbv_dn3_slot;
        let mut var_argbv_dn4: f64 = *var_argbv_dn4_slot;
        let mut var_argbv_dn5: f64 = *var_argbv_dn5_slot;
        let mut var_argbv_dn6: f64 = *var_argbv_dn6_slot;
        let mut var_argbv_dn7: f64 = *var_argbv_dn7_slot;
        let mut var_argbv_dn8: f64 = *var_argbv_dn8_slot;
        let mut var_argbv_dn9: f64 = *var_argbv_dn9_slot;
        let mut var_argbv_rdb0: f64 = *var_argbv_rdb0_slot;
        let mut var_argbv_rdb1: f64 = *var_argbv_rdb1_slot;
        let mut var_argbv_rdb2: f64 = *var_argbv_rdb2_slot;
        let mut var_argbv_rdb3: f64 = *var_argbv_rdb3_slot;
        let mut var_argbv_rdb4: f64 = *var_argbv_rdb4_slot;
        let mut var_argbv_rdb5: f64 = *var_argbv_rdb5_slot;
        let mut var_argbv_rdb6: f64 = *var_argbv_rdb6_slot;
        let mut var_argbv_rdb7: f64 = *var_argbv_rdb7_slot;
        let mut var_argbv_rdn0: f64 = *var_argbv_rdn0_slot;
        let mut var_argbv_rdn1: f64 = *var_argbv_rdn1_slot;
        let mut var_argbv_rdn2: f64 = *var_argbv_rdn2_slot;
        let mut var_argbv_rdn3: f64 = *var_argbv_rdn3_slot;
        let mut var_argbv_rdn4: f64 = *var_argbv_rdn4_slot;
        let mut var_argbv_rdn5: f64 = *var_argbv_rdn5_slot;
        let mut var_argbv_rdn6: f64 = *var_argbv_rdn6_slot;
        let mut var_argbv_rdn7: f64 = *var_argbv_rdn7_slot;
        let mut var_argbv_rdn8: f64 = *var_argbv_rdn8_slot;
        let mut var_argbv_rdn9: f64 = *var_argbv_rdn9_slot;
        let mut var_argbv_rv: f64 = *var_argbv_rv_slot;
        let mut var_argbvvt: f64 = *var_argbvvt_slot;
        let mut var_argbvvt_db0: f64 = *var_argbvvt_db0_slot;
        let mut var_argbvvt_db1: f64 = *var_argbvvt_db1_slot;
        let mut var_argbvvt_db2: f64 = *var_argbvvt_db2_slot;
        let mut var_argbvvt_db3: f64 = *var_argbvvt_db3_slot;
        let mut var_argbvvt_db4: f64 = *var_argbvvt_db4_slot;
        let mut var_argbvvt_db5: f64 = *var_argbvvt_db5_slot;
        let mut var_argbvvt_db6: f64 = *var_argbvvt_db6_slot;
        let mut var_argbvvt_db7: f64 = *var_argbvvt_db7_slot;
        let mut var_argbvvt_dn0: f64 = *var_argbvvt_dn0_slot;
        let mut var_argbvvt_dn1: f64 = *var_argbvvt_dn1_slot;
        let mut var_argbvvt_dn2: f64 = *var_argbvvt_dn2_slot;
        let mut var_argbvvt_dn3: f64 = *var_argbvvt_dn3_slot;
        let mut var_argbvvt_dn4: f64 = *var_argbvvt_dn4_slot;
        let mut var_argbvvt_dn5: f64 = *var_argbvvt_dn5_slot;
        let mut var_argbvvt_dn6: f64 = *var_argbvvt_dn6_slot;
        let mut var_argbvvt_dn7: f64 = *var_argbvvt_dn7_slot;
        let mut var_argbvvt_dn8: f64 = *var_argbvvt_dn8_slot;
        let mut var_argbvvt_dn9: f64 = *var_argbvvt_dn9_slot;
        let mut var_argbvvt_rdb0: f64 = *var_argbvvt_rdb0_slot;
        let mut var_argbvvt_rdb1: f64 = *var_argbvvt_rdb1_slot;
        let mut var_argbvvt_rdb2: f64 = *var_argbvvt_rdb2_slot;
        let mut var_argbvvt_rdb3: f64 = *var_argbvvt_rdb3_slot;
        let mut var_argbvvt_rdb4: f64 = *var_argbvvt_rdb4_slot;
        let mut var_argbvvt_rdb5: f64 = *var_argbvvt_rdb5_slot;
        let mut var_argbvvt_rdb6: f64 = *var_argbvvt_rdb6_slot;
        let mut var_argbvvt_rdb7: f64 = *var_argbvvt_rdb7_slot;
        let mut var_argbvvt_rdn0: f64 = *var_argbvvt_rdn0_slot;
        let mut var_argbvvt_rdn1: f64 = *var_argbvvt_rdn1_slot;
        let mut var_argbvvt_rdn2: f64 = *var_argbvvt_rdn2_slot;
        let mut var_argbvvt_rdn3: f64 = *var_argbvvt_rdn3_slot;
        let mut var_argbvvt_rdn4: f64 = *var_argbvvt_rdn4_slot;
        let mut var_argbvvt_rdn5: f64 = *var_argbvvt_rdn5_slot;
        let mut var_argbvvt_rdn6: f64 = *var_argbvvt_rdn6_slot;
        let mut var_argbvvt_rdn7: f64 = *var_argbvvt_rdn7_slot;
        let mut var_argbvvt_rdn8: f64 = *var_argbvvt_rdn8_slot;
        let mut var_argbvvt_rdn9: f64 = *var_argbvvt_rdn9_slot;
        let mut var_argbvvt_rv: f64 = *var_argbvvt_rv_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_guard12_db0: f64 = *var_guard12_db0_slot;
        let mut var_guard12_db1: f64 = *var_guard12_db1_slot;
        let mut var_guard12_db2: f64 = *var_guard12_db2_slot;
        let mut var_guard12_db3: f64 = *var_guard12_db3_slot;
        let mut var_guard12_db4: f64 = *var_guard12_db4_slot;
        let mut var_guard12_db5: f64 = *var_guard12_db5_slot;
        let mut var_guard12_db6: f64 = *var_guard12_db6_slot;
        let mut var_guard12_db7: f64 = *var_guard12_db7_slot;
        let mut var_guard12_dn0: f64 = *var_guard12_dn0_slot;
        let mut var_guard12_dn1: f64 = *var_guard12_dn1_slot;
        let mut var_guard12_dn2: f64 = *var_guard12_dn2_slot;
        let mut var_guard12_dn3: f64 = *var_guard12_dn3_slot;
        let mut var_guard12_dn4: f64 = *var_guard12_dn4_slot;
        let mut var_guard12_dn5: f64 = *var_guard12_dn5_slot;
        let mut var_guard12_dn6: f64 = *var_guard12_dn6_slot;
        let mut var_guard12_dn7: f64 = *var_guard12_dn7_slot;
        let mut var_guard12_dn8: f64 = *var_guard12_dn8_slot;
        let mut var_guard12_dn9: f64 = *var_guard12_dn9_slot;
        let mut var_guard12_rdb0: f64 = *var_guard12_rdb0_slot;
        let mut var_guard12_rdb1: f64 = *var_guard12_rdb1_slot;
        let mut var_guard12_rdb2: f64 = *var_guard12_rdb2_slot;
        let mut var_guard12_rdb3: f64 = *var_guard12_rdb3_slot;
        let mut var_guard12_rdb4: f64 = *var_guard12_rdb4_slot;
        let mut var_guard12_rdb5: f64 = *var_guard12_rdb5_slot;
        let mut var_guard12_rdb6: f64 = *var_guard12_rdb6_slot;
        let mut var_guard12_rdb7: f64 = *var_guard12_rdb7_slot;
        let mut var_guard12_rdn0: f64 = *var_guard12_rdn0_slot;
        let mut var_guard12_rdn1: f64 = *var_guard12_rdn1_slot;
        let mut var_guard12_rdn2: f64 = *var_guard12_rdn2_slot;
        let mut var_guard12_rdn3: f64 = *var_guard12_rdn3_slot;
        let mut var_guard12_rdn4: f64 = *var_guard12_rdn4_slot;
        let mut var_guard12_rdn5: f64 = *var_guard12_rdn5_slot;
        let mut var_guard12_rdn6: f64 = *var_guard12_rdn6_slot;
        let mut var_guard12_rdn7: f64 = *var_guard12_rdn7_slot;
        let mut var_guard12_rdn8: f64 = *var_guard12_rdn8_slot;
        let mut var_guard12_rdn9: f64 = *var_guard12_rdn9_slot;
        let mut var_guard12_rv: f64 = *var_guard12_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_db0: f64 = *var_le_db0_slot;
        let mut var_le_db1: f64 = *var_le_db1_slot;
        let mut var_le_db2: f64 = *var_le_db2_slot;
        let mut var_le_db3: f64 = *var_le_db3_slot;
        let mut var_le_db4: f64 = *var_le_db4_slot;
        let mut var_le_db5: f64 = *var_le_db5_slot;
        let mut var_le_db6: f64 = *var_le_db6_slot;
        let mut var_le_db7: f64 = *var_le_db7_slot;
        let mut var_le_dn0: f64 = *var_le_dn0_slot;
        let mut var_le_dn1: f64 = *var_le_dn1_slot;
        let mut var_le_dn2: f64 = *var_le_dn2_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn5: f64 = *var_le_dn5_slot;
        let mut var_le_dn6: f64 = *var_le_dn6_slot;
        let mut var_le_dn7: f64 = *var_le_dn7_slot;
        let mut var_le_dn8: f64 = *var_le_dn8_slot;
        let mut var_le_dn9: f64 = *var_le_dn9_slot;
        let mut var_le_rdb0: f64 = *var_le_rdb0_slot;
        let mut var_le_rdb1: f64 = *var_le_rdb1_slot;
        let mut var_le_rdb2: f64 = *var_le_rdb2_slot;
        let mut var_le_rdb3: f64 = *var_le_rdb3_slot;
        let mut var_le_rdb4: f64 = *var_le_rdb4_slot;
        let mut var_le_rdb5: f64 = *var_le_rdb5_slot;
        let mut var_le_rdb6: f64 = *var_le_rdb6_slot;
        let mut var_le_rdb7: f64 = *var_le_rdb7_slot;
        let mut var_le_rdn0: f64 = *var_le_rdn0_slot;
        let mut var_le_rdn1: f64 = *var_le_rdn1_slot;
        let mut var_le_rdn2: f64 = *var_le_rdn2_slot;
        let mut var_le_rdn3: f64 = *var_le_rdn3_slot;
        let mut var_le_rdn4: f64 = *var_le_rdn4_slot;
        let mut var_le_rdn5: f64 = *var_le_rdn5_slot;
        let mut var_le_rdn6: f64 = *var_le_rdn6_slot;
        let mut var_le_rdn7: f64 = *var_le_rdn7_slot;
        let mut var_le_rdn8: f64 = *var_le_rdn8_slot;
        let mut var_le_rdn9: f64 = *var_le_rdn9_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_lebv: f64 = *var_lebv_slot;
        let mut var_lebv_db0: f64 = *var_lebv_db0_slot;
        let mut var_lebv_db1: f64 = *var_lebv_db1_slot;
        let mut var_lebv_db2: f64 = *var_lebv_db2_slot;
        let mut var_lebv_db3: f64 = *var_lebv_db3_slot;
        let mut var_lebv_db4: f64 = *var_lebv_db4_slot;
        let mut var_lebv_db5: f64 = *var_lebv_db5_slot;
        let mut var_lebv_db6: f64 = *var_lebv_db6_slot;
        let mut var_lebv_db7: f64 = *var_lebv_db7_slot;
        let mut var_lebv_dn0: f64 = *var_lebv_dn0_slot;
        let mut var_lebv_dn1: f64 = *var_lebv_dn1_slot;
        let mut var_lebv_dn2: f64 = *var_lebv_dn2_slot;
        let mut var_lebv_dn3: f64 = *var_lebv_dn3_slot;
        let mut var_lebv_dn4: f64 = *var_lebv_dn4_slot;
        let mut var_lebv_dn5: f64 = *var_lebv_dn5_slot;
        let mut var_lebv_dn6: f64 = *var_lebv_dn6_slot;
        let mut var_lebv_dn7: f64 = *var_lebv_dn7_slot;
        let mut var_lebv_dn8: f64 = *var_lebv_dn8_slot;
        let mut var_lebv_dn9: f64 = *var_lebv_dn9_slot;
        let mut var_lebv_rdb0: f64 = *var_lebv_rdb0_slot;
        let mut var_lebv_rdb1: f64 = *var_lebv_rdb1_slot;
        let mut var_lebv_rdb2: f64 = *var_lebv_rdb2_slot;
        let mut var_lebv_rdb3: f64 = *var_lebv_rdb3_slot;
        let mut var_lebv_rdb4: f64 = *var_lebv_rdb4_slot;
        let mut var_lebv_rdb5: f64 = *var_lebv_rdb5_slot;
        let mut var_lebv_rdb6: f64 = *var_lebv_rdb6_slot;
        let mut var_lebv_rdb7: f64 = *var_lebv_rdb7_slot;
        let mut var_lebv_rdn0: f64 = *var_lebv_rdn0_slot;
        let mut var_lebv_rdn1: f64 = *var_lebv_rdn1_slot;
        let mut var_lebv_rdn2: f64 = *var_lebv_rdn2_slot;
        let mut var_lebv_rdn3: f64 = *var_lebv_rdn3_slot;
        let mut var_lebv_rdn4: f64 = *var_lebv_rdn4_slot;
        let mut var_lebv_rdn5: f64 = *var_lebv_rdn5_slot;
        let mut var_lebv_rdn6: f64 = *var_lebv_rdn6_slot;
        let mut var_lebv_rdn7: f64 = *var_lebv_rdn7_slot;
        let mut var_lebv_rdn8: f64 = *var_lebv_rdn8_slot;
        let mut var_lebv_rdn9: f64 = *var_lebv_rdn9_slot;
        let mut var_lebv_rv: f64 = *var_lebv_rv_slot;

        let (assign1180_e1502, assign1180_e1502_d_n0, assign1180_e1502_d_n1, assign1180_e1502_d_n2, assign1180_e1502_d_n3, assign1180_e1502_d_n4, assign1180_e1502_d_n5, assign1180_e1502_d_n6, assign1180_e1502_d_n7, assign1180_e1502_d_n8, assign1180_e1502_d_n9, assign1180_e1502_d_b0, assign1180_e1502_d_b1, assign1180_e1502_d_b2, assign1180_e1502_d_b3, assign1180_e1502_d_b4, assign1180_e1502_d_b5, assign1180_e1502_d_b6, assign1180_e1502_d_b7,) = {
    if (var_guard11 != 0.0) {
        let assign1180_e1499: f64 = (p.p65 * var_vt);
        let assign1180_e1500: f64 = (var_vbici / assign1180_e1499);
        (assign1180_e1500, (((var_vbici_dn0 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_dn0))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_dn1 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_dn1))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_dn2 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_dn2))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_dn3 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_dn3))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_dn4 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_dn4))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_dn5 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_dn5))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_dn6 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_dn6))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_dn7 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_dn7))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_dn8 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_dn8))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_dn9 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_dn9))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_db0 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_db0))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_db1 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_db1))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_db2 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_db2))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_db3 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_db3))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_db4 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_db4))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_db5 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_db5))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_db6 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_db6))) / (assign1180_e1499 * assign1180_e1499)), (((var_vbici_db7 * assign1180_e1499) - (var_vbici * (p.p65 * var_vt_db7))) / (assign1180_e1499 * assign1180_e1499)),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    }
};
        var_arg = assign1180_e1502;
        var_arg_dn0 = assign1180_e1502_d_n0;
        var_arg_dn1 = assign1180_e1502_d_n1;
        var_arg_dn2 = assign1180_e1502_d_n2;
        var_arg_dn3 = assign1180_e1502_d_n3;
        var_arg_dn4 = assign1180_e1502_d_n4;
        var_arg_dn5 = assign1180_e1502_d_n5;
        var_arg_dn6 = assign1180_e1502_d_n6;
        var_arg_dn7 = assign1180_e1502_d_n7;
        var_arg_dn8 = assign1180_e1502_d_n8;
        var_arg_dn9 = assign1180_e1502_d_n9;
        var_arg_db0 = assign1180_e1502_d_b0;
        var_arg_db1 = assign1180_e1502_d_b1;
        var_arg_db2 = assign1180_e1502_d_b2;
        var_arg_db3 = assign1180_e1502_d_b3;
        var_arg_db4 = assign1180_e1502_d_b4;
        var_arg_db5 = assign1180_e1502_d_b5;
        var_arg_db6 = assign1180_e1502_d_b6;
        var_arg_db7 = assign1180_e1502_d_b7;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdn7 = 0.0;
        var_arg_rdn8 = 0.0;
        var_arg_rdn9 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;
        var_arg_rdb7 = 0.0;

        let (assign1190_e1513, assign1190_e1513_d_n0, assign1190_e1513_d_n1, assign1190_e1513_d_n2, assign1190_e1513_d_n3, assign1190_e1513_d_n4, assign1190_e1513_d_n5, assign1190_e1513_d_n6, assign1190_e1513_d_n7, assign1190_e1513_d_n8, assign1190_e1513_d_n9, assign1190_e1513_d_b0, assign1190_e1513_d_b1, assign1190_e1513_d_b2, assign1190_e1513_d_b3, assign1190_e1513_d_b4, assign1190_e1513_d_b5, assign1190_e1513_d_b6, assign1190_e1513_d_b7,) = {
    if (var_guard11 != 0.0) {
        let assign1190_e1505: f64 = (-var_vbici);
        let assign1190_e1507: f64 = (assign1190_e1505 - var_bvr_t);
        let assign1190_e1510: f64 = (p.p57 * var_vt);
        let assign1190_e1511: f64 = (assign1190_e1507 / assign1190_e1510);
        (assign1190_e1511, (((((-var_vbici_dn0) - var_bvr_t_dn0) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_dn0))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_dn1) - var_bvr_t_dn1) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_dn1))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_dn2) - var_bvr_t_dn2) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_dn2))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_dn3) - var_bvr_t_dn3) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_dn3))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_dn4) - var_bvr_t_dn4) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_dn4))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_dn5) - var_bvr_t_dn5) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_dn5))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_dn6) - var_bvr_t_dn6) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_dn6))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_dn7) - var_bvr_t_dn7) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_dn7))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_dn8) - var_bvr_t_dn8) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_dn8))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_dn9) - var_bvr_t_dn9) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_dn9))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_db0) - var_bvr_t_db0) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_db0))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_db1) - var_bvr_t_db1) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_db1))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_db2) - var_bvr_t_db2) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_db2))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_db3) - var_bvr_t_db3) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_db3))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_db4) - var_bvr_t_db4) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_db4))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_db5) - var_bvr_t_db5) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_db5))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_db6) - var_bvr_t_db6) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_db6))) / (assign1190_e1510 * assign1190_e1510)), (((((-var_vbici_db7) - var_bvr_t_db7) * assign1190_e1510) - (assign1190_e1507 * (p.p57 * var_vt_db7))) / (assign1190_e1510 * assign1190_e1510)),)
    } else {
        (var_argbv, var_argbv_dn0, var_argbv_dn1, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6, var_argbv_dn7, var_argbv_dn8, var_argbv_dn9, var_argbv_db0, var_argbv_db1, var_argbv_db2, var_argbv_db3, var_argbv_db4, var_argbv_db5, var_argbv_db6, var_argbv_db7,)
    }
};
        var_argbv = assign1190_e1513;
        var_argbv_dn0 = assign1190_e1513_d_n0;
        var_argbv_dn1 = assign1190_e1513_d_n1;
        var_argbv_dn2 = assign1190_e1513_d_n2;
        var_argbv_dn3 = assign1190_e1513_d_n3;
        var_argbv_dn4 = assign1190_e1513_d_n4;
        var_argbv_dn5 = assign1190_e1513_d_n5;
        var_argbv_dn6 = assign1190_e1513_d_n6;
        var_argbv_dn7 = assign1190_e1513_d_n7;
        var_argbv_dn8 = assign1190_e1513_d_n8;
        var_argbv_dn9 = assign1190_e1513_d_n9;
        var_argbv_db0 = assign1190_e1513_d_b0;
        var_argbv_db1 = assign1190_e1513_d_b1;
        var_argbv_db2 = assign1190_e1513_d_b2;
        var_argbv_db3 = assign1190_e1513_d_b3;
        var_argbv_db4 = assign1190_e1513_d_b4;
        var_argbv_db5 = assign1190_e1513_d_b5;
        var_argbv_db6 = assign1190_e1513_d_b6;
        var_argbv_db7 = assign1190_e1513_d_b7;
        var_argbv_rv = 0.0;
        var_argbv_rdn0 = 0.0;
        var_argbv_rdn1 = 0.0;
        var_argbv_rdn2 = 0.0;
        var_argbv_rdn3 = 0.0;
        var_argbv_rdn4 = 0.0;
        var_argbv_rdn5 = 0.0;
        var_argbv_rdn6 = 0.0;
        var_argbv_rdn7 = 0.0;
        var_argbv_rdn8 = 0.0;
        var_argbv_rdn9 = 0.0;
        var_argbv_rdb0 = 0.0;
        var_argbv_rdb1 = 0.0;
        var_argbv_rdb2 = 0.0;
        var_argbv_rdb3 = 0.0;
        var_argbv_rdb4 = 0.0;
        var_argbv_rdb5 = 0.0;
        var_argbv_rdb6 = 0.0;
        var_argbv_rdb7 = 0.0;

        let (assign1200_e1522, assign1200_e1522_d_n0, assign1200_e1522_d_n1, assign1200_e1522_d_n2, assign1200_e1522_d_n3, assign1200_e1522_d_n4, assign1200_e1522_d_n5, assign1200_e1522_d_n6, assign1200_e1522_d_n7, assign1200_e1522_d_n8, assign1200_e1522_d_n9, assign1200_e1522_d_b0, assign1200_e1522_d_b1, assign1200_e1522_d_b2, assign1200_e1522_d_b3, assign1200_e1522_d_b4, assign1200_e1522_d_b5, assign1200_e1522_d_b6, assign1200_e1522_d_b7,) = {
    if (var_guard11 != 0.0) {
        let assign1200_e1516: f64 = (-var_bvr_t);
        let assign1200_e1519: f64 = (p.p57 * var_vt);
        let assign1200_e1520: f64 = (assign1200_e1516 / assign1200_e1519);
        (assign1200_e1520, ((((-var_bvr_t_dn0) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_dn0))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_dn1) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_dn1))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_dn2) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_dn2))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_dn3) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_dn3))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_dn4) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_dn4))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_dn5) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_dn5))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_dn6) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_dn6))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_dn7) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_dn7))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_dn8) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_dn8))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_dn9) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_dn9))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_db0) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_db0))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_db1) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_db1))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_db2) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_db2))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_db3) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_db3))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_db4) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_db4))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_db5) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_db5))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_db6) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_db6))) / (assign1200_e1519 * assign1200_e1519)), ((((-var_bvr_t_db7) * assign1200_e1519) - (assign1200_e1516 * (p.p57 * var_vt_db7))) / (assign1200_e1519 * assign1200_e1519)),)
    } else {
        (var_argbvvt, var_argbvvt_dn0, var_argbvvt_dn1, var_argbvvt_dn2, var_argbvvt_dn3, var_argbvvt_dn4, var_argbvvt_dn5, var_argbvvt_dn6, var_argbvvt_dn7, var_argbvvt_dn8, var_argbvvt_dn9, var_argbvvt_db0, var_argbvvt_db1, var_argbvvt_db2, var_argbvvt_db3, var_argbvvt_db4, var_argbvvt_db5, var_argbvvt_db6, var_argbvvt_db7,)
    }
};
        var_argbvvt = assign1200_e1522;
        var_argbvvt_dn0 = assign1200_e1522_d_n0;
        var_argbvvt_dn1 = assign1200_e1522_d_n1;
        var_argbvvt_dn2 = assign1200_e1522_d_n2;
        var_argbvvt_dn3 = assign1200_e1522_d_n3;
        var_argbvvt_dn4 = assign1200_e1522_d_n4;
        var_argbvvt_dn5 = assign1200_e1522_d_n5;
        var_argbvvt_dn6 = assign1200_e1522_d_n6;
        var_argbvvt_dn7 = assign1200_e1522_d_n7;
        var_argbvvt_dn8 = assign1200_e1522_d_n8;
        var_argbvvt_dn9 = assign1200_e1522_d_n9;
        var_argbvvt_db0 = assign1200_e1522_d_b0;
        var_argbvvt_db1 = assign1200_e1522_d_b1;
        var_argbvvt_db2 = assign1200_e1522_d_b2;
        var_argbvvt_db3 = assign1200_e1522_d_b3;
        var_argbvvt_db4 = assign1200_e1522_d_b4;
        var_argbvvt_db5 = assign1200_e1522_d_b5;
        var_argbvvt_db6 = assign1200_e1522_d_b6;
        var_argbvvt_db7 = assign1200_e1522_d_b7;
        var_argbvvt_rv = 0.0;
        var_argbvvt_rdn0 = 0.0;
        var_argbvvt_rdn1 = 0.0;
        var_argbvvt_rdn2 = 0.0;
        var_argbvvt_rdn3 = 0.0;
        var_argbvvt_rdn4 = 0.0;
        var_argbvvt_rdn5 = 0.0;
        var_argbvvt_rdn6 = 0.0;
        var_argbvvt_rdn7 = 0.0;
        var_argbvvt_rdn8 = 0.0;
        var_argbvvt_rdn9 = 0.0;
        var_argbvvt_rdb0 = 0.0;
        var_argbvvt_rdb1 = 0.0;
        var_argbvvt_rdb2 = 0.0;
        var_argbvvt_rdb3 = 0.0;
        var_argbvvt_rdb4 = 0.0;
        var_argbvvt_rdb5 = 0.0;
        var_argbvvt_rdb6 = 0.0;
        var_argbvvt_rdb7 = 0.0;

        let assign1210_e1525: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard12 = assign1210_e1525;
        var_guard12_dn0 = 0.0;
        var_guard12_dn1 = 0.0;
        var_guard12_dn2 = 0.0;
        var_guard12_dn3 = 0.0;
        var_guard12_dn4 = 0.0;
        var_guard12_dn5 = 0.0;
        var_guard12_dn6 = 0.0;
        var_guard12_dn7 = 0.0;
        var_guard12_dn8 = 0.0;
        var_guard12_dn9 = 0.0;
        var_guard12_db0 = 0.0;
        var_guard12_db1 = 0.0;
        var_guard12_db2 = 0.0;
        var_guard12_db3 = 0.0;
        var_guard12_db4 = 0.0;
        var_guard12_db5 = 0.0;
        var_guard12_db6 = 0.0;
        var_guard12_db7 = 0.0;
        var_guard12_rv = 0.0;
        var_guard12_rdn0 = 0.0;
        var_guard12_rdn1 = 0.0;
        var_guard12_rdn2 = 0.0;
        var_guard12_rdn3 = 0.0;
        var_guard12_rdn4 = 0.0;
        var_guard12_rdn5 = 0.0;
        var_guard12_rdn6 = 0.0;
        var_guard12_rdn7 = 0.0;
        var_guard12_rdn8 = 0.0;
        var_guard12_rdn9 = 0.0;
        var_guard12_rdb0 = 0.0;
        var_guard12_rdb1 = 0.0;
        var_guard12_rdb2 = 0.0;
        var_guard12_rdb3 = 0.0;
        var_guard12_rdb4 = 0.0;
        var_guard12_rdb5 = 0.0;
        var_guard12_rdb6 = 0.0;
        var_guard12_rdb7 = 0.0;

        let (assign1220_e1535, assign1220_e1535_d_n0, assign1220_e1535_d_n1, assign1220_e1535_d_n2, assign1220_e1535_d_n3, assign1220_e1535_d_n4, assign1220_e1535_d_n5, assign1220_e1535_d_n6, assign1220_e1535_d_n7, assign1220_e1535_d_n8, assign1220_e1535_d_n9, assign1220_e1535_d_b0, assign1220_e1535_d_b1, assign1220_e1535_d_b2, assign1220_e1535_d_b3, assign1220_e1535_d_b4, assign1220_e1535_d_b5, assign1220_e1535_d_b6, assign1220_e1535_d_b7,) = {
    if ((var_guard11 != 0.0) && (var_guard12 != 0.0)) {
        let assign1220_e1532: f64 = (var_arg - 80.0);
        let assign1220_e1533: f64 = (1.0 + assign1220_e1532);
        (assign1220_e1533, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign1220_e1535;
        var_le_dn0 = assign1220_e1535_d_n0;
        var_le_dn1 = assign1220_e1535_d_n1;
        var_le_dn2 = assign1220_e1535_d_n2;
        var_le_dn3 = assign1220_e1535_d_n3;
        var_le_dn4 = assign1220_e1535_d_n4;
        var_le_dn5 = assign1220_e1535_d_n5;
        var_le_dn6 = assign1220_e1535_d_n6;
        var_le_dn7 = assign1220_e1535_d_n7;
        var_le_dn8 = assign1220_e1535_d_n8;
        var_le_dn9 = assign1220_e1535_d_n9;
        var_le_db0 = assign1220_e1535_d_b0;
        var_le_db1 = assign1220_e1535_d_b1;
        var_le_db2 = assign1220_e1535_d_b2;
        var_le_db3 = assign1220_e1535_d_b3;
        var_le_db4 = assign1220_e1535_d_b4;
        var_le_db5 = assign1220_e1535_d_b5;
        var_le_db6 = assign1220_e1535_d_b6;
        var_le_db7 = assign1220_e1535_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign1230_e1541, assign1230_e1541_d_n0, assign1230_e1541_d_n1, assign1230_e1541_d_n2, assign1230_e1541_d_n3, assign1230_e1541_d_n4, assign1230_e1541_d_n5, assign1230_e1541_d_n6, assign1230_e1541_d_n7, assign1230_e1541_d_n8, assign1230_e1541_d_n9, assign1230_e1541_d_b0, assign1230_e1541_d_b1, assign1230_e1541_d_b2, assign1230_e1541_d_b3, assign1230_e1541_d_b4, assign1230_e1541_d_b5, assign1230_e1541_d_b6, assign1230_e1541_d_b7,) = {
    if ((var_guard11 != 0.0) && (var_guard12 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn1, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_db0, var_arg_db1, var_arg_db2, var_arg_db3, var_arg_db4, var_arg_db5, var_arg_db6, var_arg_db7,)
    }
};
        var_arg = assign1230_e1541;
        var_arg_dn0 = assign1230_e1541_d_n0;
        var_arg_dn1 = assign1230_e1541_d_n1;
        var_arg_dn2 = assign1230_e1541_d_n2;
        var_arg_dn3 = assign1230_e1541_d_n3;
        var_arg_dn4 = assign1230_e1541_d_n4;
        var_arg_dn5 = assign1230_e1541_d_n5;
        var_arg_dn6 = assign1230_e1541_d_n6;
        var_arg_dn7 = assign1230_e1541_d_n7;
        var_arg_dn8 = assign1230_e1541_d_n8;
        var_arg_dn9 = assign1230_e1541_d_n9;
        var_arg_db0 = assign1230_e1541_d_b0;
        var_arg_db1 = assign1230_e1541_d_b1;
        var_arg_db2 = assign1230_e1541_d_b2;
        var_arg_db3 = assign1230_e1541_d_b3;
        var_arg_db4 = assign1230_e1541_d_b4;
        var_arg_db5 = assign1230_e1541_d_b5;
        var_arg_db6 = assign1230_e1541_d_b6;
        var_arg_db7 = assign1230_e1541_d_b7;
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdn7 = 0.0;
        var_arg_rdn8 = 0.0;
        var_arg_rdn9 = 0.0;
        var_arg_rdb0 = 0.0;
        var_arg_rdb1 = 0.0;
        var_arg_rdb2 = 0.0;
        var_arg_rdb3 = 0.0;
        var_arg_rdb4 = 0.0;
        var_arg_rdb5 = 0.0;
        var_arg_rdb6 = 0.0;
        var_arg_rdb7 = 0.0;

        let (assign1240_e1548, assign1240_e1548_d_n0, assign1240_e1548_d_n1, assign1240_e1548_d_n2, assign1240_e1548_d_n3, assign1240_e1548_d_n4, assign1240_e1548_d_n5, assign1240_e1548_d_n6, assign1240_e1548_d_n7, assign1240_e1548_d_n8, assign1240_e1548_d_n9, assign1240_e1548_d_b0, assign1240_e1548_d_b1, assign1240_e1548_d_b2, assign1240_e1548_d_b3, assign1240_e1548_d_b4, assign1240_e1548_d_b5, assign1240_e1548_d_b6, assign1240_e1548_d_b7,) = {
    if ((var_guard11 != 0.0) && (var_guard12 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign1240_e1548;
        var_le_dn0 = assign1240_e1548_d_n0;
        var_le_dn1 = assign1240_e1548_d_n1;
        var_le_dn2 = assign1240_e1548_d_n2;
        var_le_dn3 = assign1240_e1548_d_n3;
        var_le_dn4 = assign1240_e1548_d_n4;
        var_le_dn5 = assign1240_e1548_d_n5;
        var_le_dn6 = assign1240_e1548_d_n6;
        var_le_dn7 = assign1240_e1548_d_n7;
        var_le_dn8 = assign1240_e1548_d_n8;
        var_le_dn9 = assign1240_e1548_d_n9;
        var_le_db0 = assign1240_e1548_d_b0;
        var_le_db1 = assign1240_e1548_d_b1;
        var_le_db2 = assign1240_e1548_d_b2;
        var_le_db3 = assign1240_e1548_d_b3;
        var_le_db4 = assign1240_e1548_d_b4;
        var_le_db5 = assign1240_e1548_d_b5;
        var_le_db6 = assign1240_e1548_d_b6;
        var_le_db7 = assign1240_e1548_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign1250_e1555, assign1250_e1555_d_n0, assign1250_e1555_d_n1, assign1250_e1555_d_n2, assign1250_e1555_d_n3, assign1250_e1555_d_n4, assign1250_e1555_d_n5, assign1250_e1555_d_n6, assign1250_e1555_d_n7, assign1250_e1555_d_n8, assign1250_e1555_d_n9, assign1250_e1555_d_b0, assign1250_e1555_d_b1, assign1250_e1555_d_b2, assign1250_e1555_d_b3, assign1250_e1555_d_b4, assign1250_e1555_d_b5, assign1250_e1555_d_b6, assign1250_e1555_d_b7,) = {
    if (var_guard11 != 0.0) {
        let assign1250_e1552: f64 = (var_arg).exp();
        let assign1250_e1553: f64 = (var_le * assign1250_e1552);
        (assign1250_e1553, ((var_le_dn0 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn0))), ((var_le_dn1 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn1))), ((var_le_dn2 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn2))), ((var_le_dn3 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn3))), ((var_le_dn4 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn4))), ((var_le_dn5 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn5))), ((var_le_dn6 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn6))), ((var_le_dn7 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn7))), ((var_le_dn8 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn8))), ((var_le_dn9 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_dn9))), ((var_le_db0 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_db0))), ((var_le_db1 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_db1))), ((var_le_db2 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_db2))), ((var_le_db3 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_db3))), ((var_le_db4 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_db4))), ((var_le_db5 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_db5))), ((var_le_db6 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_db6))), ((var_le_db7 * assign1250_e1552) + (var_le * (assign1250_e1552 * var_arg_db7))),)
    } else {
        (var_le, var_le_dn0, var_le_dn1, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn5, var_le_dn6, var_le_dn7, var_le_dn8, var_le_dn9, var_le_db0, var_le_db1, var_le_db2, var_le_db3, var_le_db4, var_le_db5, var_le_db6, var_le_db7,)
    }
};
        var_le = assign1250_e1555;
        var_le_dn0 = assign1250_e1555_d_n0;
        var_le_dn1 = assign1250_e1555_d_n1;
        var_le_dn2 = assign1250_e1555_d_n2;
        var_le_dn3 = assign1250_e1555_d_n3;
        var_le_dn4 = assign1250_e1555_d_n4;
        var_le_dn5 = assign1250_e1555_d_n5;
        var_le_dn6 = assign1250_e1555_d_n6;
        var_le_dn7 = assign1250_e1555_d_n7;
        var_le_dn8 = assign1250_e1555_d_n8;
        var_le_dn9 = assign1250_e1555_d_n9;
        var_le_db0 = assign1250_e1555_d_b0;
        var_le_db1 = assign1250_e1555_d_b1;
        var_le_db2 = assign1250_e1555_d_b2;
        var_le_db3 = assign1250_e1555_d_b3;
        var_le_db4 = assign1250_e1555_d_b4;
        var_le_db5 = assign1250_e1555_d_b5;
        var_le_db6 = assign1250_e1555_d_b6;
        var_le_db7 = assign1250_e1555_d_b7;
        var_le_rv = 0.0;
        var_le_rdn0 = 0.0;
        var_le_rdn1 = 0.0;
        var_le_rdn2 = 0.0;
        var_le_rdn3 = 0.0;
        var_le_rdn4 = 0.0;
        var_le_rdn5 = 0.0;
        var_le_rdn6 = 0.0;
        var_le_rdn7 = 0.0;
        var_le_rdn8 = 0.0;
        var_le_rdn9 = 0.0;
        var_le_rdb0 = 0.0;
        var_le_rdb1 = 0.0;
        var_le_rdb2 = 0.0;
        var_le_rdb3 = 0.0;
        var_le_rdb4 = 0.0;
        var_le_rdb5 = 0.0;
        var_le_rdb6 = 0.0;
        var_le_rdb7 = 0.0;

        let (assign1260_e1627, assign1260_e1627_d_n0, assign1260_e1627_d_n1, assign1260_e1627_d_n2, assign1260_e1627_d_n3, assign1260_e1627_d_n4, assign1260_e1627_d_n5, assign1260_e1627_d_n6, assign1260_e1627_d_n7, assign1260_e1627_d_n8, assign1260_e1627_d_n9, assign1260_e1627_d_b0, assign1260_e1627_d_b1, assign1260_e1627_d_b2, assign1260_e1627_d_b3, assign1260_e1627_d_b4, assign1260_e1627_d_b5, assign1260_e1627_d_b6, assign1260_e1627_d_b7,) = {
    if (var_guard11 != 0.0) {
        let assign1260_e1563: f64 = (-37.0);
        let (assign1260_e1590, assign1260_e1590_d_n0, assign1260_e1590_d_n1, assign1260_e1590_d_n2, assign1260_e1590_d_n3, assign1260_e1590_d_n4, assign1260_e1590_d_n5, assign1260_e1590_d_n6, assign1260_e1590_d_n7, assign1260_e1590_d_n8, assign1260_e1590_d_n9, assign1260_e1590_d_b0, assign1260_e1590_d_b1, assign1260_e1590_d_b2, assign1260_e1590_d_b3, assign1260_e1590_d_b4, assign1260_e1590_d_b5, assign1260_e1590_d_b6, assign1260_e1590_d_b7,) = {
            if ((!(var_argbv >= 37.0)) && (!(var_argbv <= assign1260_e1563))) {
                let assign1260_e1568: f64 = (var_argbv).exp();
                let assign1260_e1570: f64 = (assign1260_e1568 + 1.0);
                let assign1260_e1571: f64 = (assign1260_e1570).ln();
                (assign1260_e1571, ((assign1260_e1568 * var_argbv_dn0) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn1) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn2) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn3) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn4) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn5) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn6) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn7) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn8) / assign1260_e1570), ((assign1260_e1568 * var_argbv_dn9) / assign1260_e1570), ((assign1260_e1568 * var_argbv_db0) / assign1260_e1570), ((assign1260_e1568 * var_argbv_db1) / assign1260_e1570), ((assign1260_e1568 * var_argbv_db2) / assign1260_e1570), ((assign1260_e1568 * var_argbv_db3) / assign1260_e1570), ((assign1260_e1568 * var_argbv_db4) / assign1260_e1570), ((assign1260_e1568 * var_argbv_db5) / assign1260_e1570), ((assign1260_e1568 * var_argbv_db6) / assign1260_e1570), ((assign1260_e1568 * var_argbv_db7) / assign1260_e1570),)
            } else {
                let assign1260_e1578: f64 = (-37.0);
                let (assign1260_e1589, assign1260_e1589_d_n0, assign1260_e1589_d_n1, assign1260_e1589_d_n2, assign1260_e1589_d_n3, assign1260_e1589_d_n4, assign1260_e1589_d_n5, assign1260_e1589_d_n6, assign1260_e1589_d_n7, assign1260_e1589_d_n8, assign1260_e1589_d_n9, assign1260_e1589_d_b0, assign1260_e1589_d_b1, assign1260_e1589_d_b2, assign1260_e1589_d_b3, assign1260_e1589_d_b4, assign1260_e1589_d_b5, assign1260_e1589_d_b6, assign1260_e1589_d_b7,) = {
                    if ((!(var_argbv >= 37.0)) && (var_argbv <= assign1260_e1578)) {
                        let assign1260_e1582: f64 = (var_argbv).exp();
                        (assign1260_e1582, (assign1260_e1582 * var_argbv_dn0), (assign1260_e1582 * var_argbv_dn1), (assign1260_e1582 * var_argbv_dn2), (assign1260_e1582 * var_argbv_dn3), (assign1260_e1582 * var_argbv_dn4), (assign1260_e1582 * var_argbv_dn5), (assign1260_e1582 * var_argbv_dn6), (assign1260_e1582 * var_argbv_dn7), (assign1260_e1582 * var_argbv_dn8), (assign1260_e1582 * var_argbv_dn9), (assign1260_e1582 * var_argbv_db0), (assign1260_e1582 * var_argbv_db1), (assign1260_e1582 * var_argbv_db2), (assign1260_e1582 * var_argbv_db3), (assign1260_e1582 * var_argbv_db4), (assign1260_e1582 * var_argbv_db5), (assign1260_e1582 * var_argbv_db6), (assign1260_e1582 * var_argbv_db7),)
                    } else {
                        let (assign1260_e1588, assign1260_e1588_d_n0, assign1260_e1588_d_n1, assign1260_e1588_d_n2, assign1260_e1588_d_n3, assign1260_e1588_d_n4, assign1260_e1588_d_n5, assign1260_e1588_d_n6, assign1260_e1588_d_n7, assign1260_e1588_d_n8, assign1260_e1588_d_n9, assign1260_e1588_d_b0, assign1260_e1588_d_b1, assign1260_e1588_d_b2, assign1260_e1588_d_b3, assign1260_e1588_d_b4, assign1260_e1588_d_b5, assign1260_e1588_d_b6, assign1260_e1588_d_b7,) = {
                            if (var_argbv >= 37.0) {
                                (var_argbv, var_argbv_dn0, var_argbv_dn1, var_argbv_dn2, var_argbv_dn3, var_argbv_dn4, var_argbv_dn5, var_argbv_dn6, var_argbv_dn7, var_argbv_dn8, var_argbv_dn9, var_argbv_db0, var_argbv_db1, var_argbv_db2, var_argbv_db3, var_argbv_db4, var_argbv_db5, var_argbv_db6, var_argbv_db7,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign1260_e1588, assign1260_e1588_d_n0, assign1260_e1588_d_n1, assign1260_e1588_d_n2, assign1260_e1588_d_n3, assign1260_e1588_d_n4, assign1260_e1588_d_n5, assign1260_e1588_d_n6, assign1260_e1588_d_n7, assign1260_e1588_d_n8, assign1260_e1588_d_n9, assign1260_e1588_d_b0, assign1260_e1588_d_b1, assign1260_e1588_d_b2, assign1260_e1588_d_b3, assign1260_e1588_d_b4, assign1260_e1588_d_b5, assign1260_e1588_d_b6, assign1260_e1588_d_b7,)
                    }
                };
                (assign1260_e1589, assign1260_e1589_d_n0, assign1260_e1589_d_n1, assign1260_e1589_d_n2, assign1260_e1589_d_n3, assign1260_e1589_d_n4, assign1260_e1589_d_n5, assign1260_e1589_d_n6, assign1260_e1589_d_n7, assign1260_e1589_d_n8, assign1260_e1589_d_n9, assign1260_e1589_d_b0, assign1260_e1589_d_b1, assign1260_e1589_d_b2, assign1260_e1589_d_b3, assign1260_e1589_d_b4, assign1260_e1589_d_b5, assign1260_e1589_d_b6, assign1260_e1589_d_b7,)
            }
        };
        let assign1260_e1597: f64 = (-37.0);
        let (assign1260_e1624, assign1260_e1624_d_n0, assign1260_e1624_d_n1, assign1260_e1624_d_n2, assign1260_e1624_d_n3, assign1260_e1624_d_n4, assign1260_e1624_d_n5, assign1260_e1624_d_n6, assign1260_e1624_d_n7, assign1260_e1624_d_n8, assign1260_e1624_d_n9, assign1260_e1624_d_b0, assign1260_e1624_d_b1, assign1260_e1624_d_b2, assign1260_e1624_d_b3, assign1260_e1624_d_b4, assign1260_e1624_d_b5, assign1260_e1624_d_b6, assign1260_e1624_d_b7,) = {
            if ((!(var_argbvvt >= 37.0)) && (!(var_argbvvt <= assign1260_e1597))) {
                let assign1260_e1602: f64 = (var_argbvvt).exp();
                let assign1260_e1604: f64 = (assign1260_e1602 + 1.0);
                let assign1260_e1605: f64 = (assign1260_e1604).ln();
                (assign1260_e1605, ((assign1260_e1602 * var_argbvvt_dn0) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_dn1) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_dn2) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_dn3) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_dn4) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_dn5) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_dn6) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_dn7) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_dn8) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_dn9) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_db0) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_db1) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_db2) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_db3) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_db4) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_db5) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_db6) / assign1260_e1604), ((assign1260_e1602 * var_argbvvt_db7) / assign1260_e1604),)
            } else {
                let assign1260_e1612: f64 = (-37.0);
                let (assign1260_e1623, assign1260_e1623_d_n0, assign1260_e1623_d_n1, assign1260_e1623_d_n2, assign1260_e1623_d_n3, assign1260_e1623_d_n4, assign1260_e1623_d_n5, assign1260_e1623_d_n6, assign1260_e1623_d_n7, assign1260_e1623_d_n8, assign1260_e1623_d_n9, assign1260_e1623_d_b0, assign1260_e1623_d_b1, assign1260_e1623_d_b2, assign1260_e1623_d_b3, assign1260_e1623_d_b4, assign1260_e1623_d_b5, assign1260_e1623_d_b6, assign1260_e1623_d_b7,) = {
                    if ((!(var_argbvvt >= 37.0)) && (var_argbvvt <= assign1260_e1612)) {
                        let assign1260_e1616: f64 = (var_argbvvt).exp();
                        (assign1260_e1616, (assign1260_e1616 * var_argbvvt_dn0), (assign1260_e1616 * var_argbvvt_dn1), (assign1260_e1616 * var_argbvvt_dn2), (assign1260_e1616 * var_argbvvt_dn3), (assign1260_e1616 * var_argbvvt_dn4), (assign1260_e1616 * var_argbvvt_dn5), (assign1260_e1616 * var_argbvvt_dn6), (assign1260_e1616 * var_argbvvt_dn7), (assign1260_e1616 * var_argbvvt_dn8), (assign1260_e1616 * var_argbvvt_dn9), (assign1260_e1616 * var_argbvvt_db0), (assign1260_e1616 * var_argbvvt_db1), (assign1260_e1616 * var_argbvvt_db2), (assign1260_e1616 * var_argbvvt_db3), (assign1260_e1616 * var_argbvvt_db4), (assign1260_e1616 * var_argbvvt_db5), (assign1260_e1616 * var_argbvvt_db6), (assign1260_e1616 * var_argbvvt_db7),)
                    } else {
                        let (assign1260_e1622, assign1260_e1622_d_n0, assign1260_e1622_d_n1, assign1260_e1622_d_n2, assign1260_e1622_d_n3, assign1260_e1622_d_n4, assign1260_e1622_d_n5, assign1260_e1622_d_n6, assign1260_e1622_d_n7, assign1260_e1622_d_n8, assign1260_e1622_d_n9, assign1260_e1622_d_b0, assign1260_e1622_d_b1, assign1260_e1622_d_b2, assign1260_e1622_d_b3, assign1260_e1622_d_b4, assign1260_e1622_d_b5, assign1260_e1622_d_b6, assign1260_e1622_d_b7,) = {
                            if (var_argbvvt >= 37.0) {
                                (var_argbvvt, var_argbvvt_dn0, var_argbvvt_dn1, var_argbvvt_dn2, var_argbvvt_dn3, var_argbvvt_dn4, var_argbvvt_dn5, var_argbvvt_dn6, var_argbvvt_dn7, var_argbvvt_dn8, var_argbvvt_dn9, var_argbvvt_db0, var_argbvvt_db1, var_argbvvt_db2, var_argbvvt_db3, var_argbvvt_db4, var_argbvvt_db5, var_argbvvt_db6, var_argbvvt_db7,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign1260_e1622, assign1260_e1622_d_n0, assign1260_e1622_d_n1, assign1260_e1622_d_n2, assign1260_e1622_d_n3, assign1260_e1622_d_n4, assign1260_e1622_d_n5, assign1260_e1622_d_n6, assign1260_e1622_d_n7, assign1260_e1622_d_n8, assign1260_e1622_d_n9, assign1260_e1622_d_b0, assign1260_e1622_d_b1, assign1260_e1622_d_b2, assign1260_e1622_d_b3, assign1260_e1622_d_b4, assign1260_e1622_d_b5, assign1260_e1622_d_b6, assign1260_e1622_d_b7,)
                    }
                };
                (assign1260_e1623, assign1260_e1623_d_n0, assign1260_e1623_d_n1, assign1260_e1623_d_n2, assign1260_e1623_d_n3, assign1260_e1623_d_n4, assign1260_e1623_d_n5, assign1260_e1623_d_n6, assign1260_e1623_d_n7, assign1260_e1623_d_n8, assign1260_e1623_d_n9, assign1260_e1623_d_b0, assign1260_e1623_d_b1, assign1260_e1623_d_b2, assign1260_e1623_d_b3, assign1260_e1623_d_b4, assign1260_e1623_d_b5, assign1260_e1623_d_b6, assign1260_e1623_d_b7,)
            }
        };
        let assign1260_e1625: f64 = (assign1260_e1590 - assign1260_e1624);
        (assign1260_e1625, (assign1260_e1590_d_n0 - assign1260_e1624_d_n0), (assign1260_e1590_d_n1 - assign1260_e1624_d_n1), (assign1260_e1590_d_n2 - assign1260_e1624_d_n2), (assign1260_e1590_d_n3 - assign1260_e1624_d_n3), (assign1260_e1590_d_n4 - assign1260_e1624_d_n4), (assign1260_e1590_d_n5 - assign1260_e1624_d_n5), (assign1260_e1590_d_n6 - assign1260_e1624_d_n6), (assign1260_e1590_d_n7 - assign1260_e1624_d_n7), (assign1260_e1590_d_n8 - assign1260_e1624_d_n8), (assign1260_e1590_d_n9 - assign1260_e1624_d_n9), (assign1260_e1590_d_b0 - assign1260_e1624_d_b0), (assign1260_e1590_d_b1 - assign1260_e1624_d_b1), (assign1260_e1590_d_b2 - assign1260_e1624_d_b2), (assign1260_e1590_d_b3 - assign1260_e1624_d_b3), (assign1260_e1590_d_b4 - assign1260_e1624_d_b4), (assign1260_e1590_d_b5 - assign1260_e1624_d_b5), (assign1260_e1590_d_b6 - assign1260_e1624_d_b6), (assign1260_e1590_d_b7 - assign1260_e1624_d_b7),)
    } else {
        (var_lebv, var_lebv_dn0, var_lebv_dn1, var_lebv_dn2, var_lebv_dn3, var_lebv_dn4, var_lebv_dn5, var_lebv_dn6, var_lebv_dn7, var_lebv_dn8, var_lebv_dn9, var_lebv_db0, var_lebv_db1, var_lebv_db2, var_lebv_db3, var_lebv_db4, var_lebv_db5, var_lebv_db6, var_lebv_db7,)
    }
};
        var_lebv = assign1260_e1627;
        var_lebv_dn0 = assign1260_e1627_d_n0;
        var_lebv_dn1 = assign1260_e1627_d_n1;
        var_lebv_dn2 = assign1260_e1627_d_n2;
        var_lebv_dn3 = assign1260_e1627_d_n3;
        var_lebv_dn4 = assign1260_e1627_d_n4;
        var_lebv_dn5 = assign1260_e1627_d_n5;
        var_lebv_dn6 = assign1260_e1627_d_n6;
        var_lebv_dn7 = assign1260_e1627_d_n7;
        var_lebv_dn8 = assign1260_e1627_d_n8;
        var_lebv_dn9 = assign1260_e1627_d_n9;
        var_lebv_db0 = assign1260_e1627_d_b0;
        var_lebv_db1 = assign1260_e1627_d_b1;
        var_lebv_db2 = assign1260_e1627_d_b2;
        var_lebv_db3 = assign1260_e1627_d_b3;
        var_lebv_db4 = assign1260_e1627_d_b4;
        var_lebv_db5 = assign1260_e1627_d_b5;
        var_lebv_db6 = assign1260_e1627_d_b6;
        var_lebv_db7 = assign1260_e1627_d_b7;
        var_lebv_rv = 0.0;
        var_lebv_rdn0 = 0.0;
        var_lebv_rdn1 = 0.0;
        var_lebv_rdn2 = 0.0;
        var_lebv_rdn3 = 0.0;
        var_lebv_rdn4 = 0.0;
        var_lebv_rdn5 = 0.0;
        var_lebv_rdn6 = 0.0;
        var_lebv_rdn7 = 0.0;
        var_lebv_rdn8 = 0.0;
        var_lebv_rdn9 = 0.0;
        var_lebv_rdb0 = 0.0;
        var_lebv_rdb1 = 0.0;
        var_lebv_rdb2 = 0.0;
        var_lebv_rdb3 = 0.0;
        var_lebv_rdb4 = 0.0;
        var_lebv_rdb5 = 0.0;
        var_lebv_rdb6 = 0.0;
        var_lebv_rdb7 = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_db0_slot = var_arg_db0;
        *var_arg_db1_slot = var_arg_db1;
        *var_arg_db2_slot = var_arg_db2;
        *var_arg_db3_slot = var_arg_db3;
        *var_arg_db4_slot = var_arg_db4;
        *var_arg_db5_slot = var_arg_db5;
        *var_arg_db6_slot = var_arg_db6;
        *var_arg_db7_slot = var_arg_db7;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn1_slot = var_arg_dn1;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_arg_rdb0_slot = var_arg_rdb0;
        *var_arg_rdb1_slot = var_arg_rdb1;
        *var_arg_rdb2_slot = var_arg_rdb2;
        *var_arg_rdb3_slot = var_arg_rdb3;
        *var_arg_rdb4_slot = var_arg_rdb4;
        *var_arg_rdb5_slot = var_arg_rdb5;
        *var_arg_rdb6_slot = var_arg_rdb6;
        *var_arg_rdb7_slot = var_arg_rdb7;
        *var_arg_rdn0_slot = var_arg_rdn0;
        *var_arg_rdn1_slot = var_arg_rdn1;
        *var_arg_rdn2_slot = var_arg_rdn2;
        *var_arg_rdn3_slot = var_arg_rdn3;
        *var_arg_rdn4_slot = var_arg_rdn4;
        *var_arg_rdn5_slot = var_arg_rdn5;
        *var_arg_rdn6_slot = var_arg_rdn6;
        *var_arg_rdn7_slot = var_arg_rdn7;
        *var_arg_rdn8_slot = var_arg_rdn8;
        *var_arg_rdn9_slot = var_arg_rdn9;
        *var_arg_rv_slot = var_arg_rv;
        *var_argbv_slot = var_argbv;
        *var_argbv_db0_slot = var_argbv_db0;
        *var_argbv_db1_slot = var_argbv_db1;
        *var_argbv_db2_slot = var_argbv_db2;
        *var_argbv_db3_slot = var_argbv_db3;
        *var_argbv_db4_slot = var_argbv_db4;
        *var_argbv_db5_slot = var_argbv_db5;
        *var_argbv_db6_slot = var_argbv_db6;
        *var_argbv_db7_slot = var_argbv_db7;
        *var_argbv_dn0_slot = var_argbv_dn0;
        *var_argbv_dn1_slot = var_argbv_dn1;
        *var_argbv_dn2_slot = var_argbv_dn2;
        *var_argbv_dn3_slot = var_argbv_dn3;
        *var_argbv_dn4_slot = var_argbv_dn4;
        *var_argbv_dn5_slot = var_argbv_dn5;
        *var_argbv_dn6_slot = var_argbv_dn6;
        *var_argbv_dn7_slot = var_argbv_dn7;
        *var_argbv_dn8_slot = var_argbv_dn8;
        *var_argbv_dn9_slot = var_argbv_dn9;
        *var_argbv_rdb0_slot = var_argbv_rdb0;
        *var_argbv_rdb1_slot = var_argbv_rdb1;
        *var_argbv_rdb2_slot = var_argbv_rdb2;
        *var_argbv_rdb3_slot = var_argbv_rdb3;
        *var_argbv_rdb4_slot = var_argbv_rdb4;
        *var_argbv_rdb5_slot = var_argbv_rdb5;
        *var_argbv_rdb6_slot = var_argbv_rdb6;
        *var_argbv_rdb7_slot = var_argbv_rdb7;
        *var_argbv_rdn0_slot = var_argbv_rdn0;
        *var_argbv_rdn1_slot = var_argbv_rdn1;
        *var_argbv_rdn2_slot = var_argbv_rdn2;
        *var_argbv_rdn3_slot = var_argbv_rdn3;
        *var_argbv_rdn4_slot = var_argbv_rdn4;
        *var_argbv_rdn5_slot = var_argbv_rdn5;
        *var_argbv_rdn6_slot = var_argbv_rdn6;
        *var_argbv_rdn7_slot = var_argbv_rdn7;
        *var_argbv_rdn8_slot = var_argbv_rdn8;
        *var_argbv_rdn9_slot = var_argbv_rdn9;
        *var_argbv_rv_slot = var_argbv_rv;
        *var_argbvvt_slot = var_argbvvt;
        *var_argbvvt_db0_slot = var_argbvvt_db0;
        *var_argbvvt_db1_slot = var_argbvvt_db1;
        *var_argbvvt_db2_slot = var_argbvvt_db2;
        *var_argbvvt_db3_slot = var_argbvvt_db3;
        *var_argbvvt_db4_slot = var_argbvvt_db4;
        *var_argbvvt_db5_slot = var_argbvvt_db5;
        *var_argbvvt_db6_slot = var_argbvvt_db6;
        *var_argbvvt_db7_slot = var_argbvvt_db7;
        *var_argbvvt_dn0_slot = var_argbvvt_dn0;
        *var_argbvvt_dn1_slot = var_argbvvt_dn1;
        *var_argbvvt_dn2_slot = var_argbvvt_dn2;
        *var_argbvvt_dn3_slot = var_argbvvt_dn3;
        *var_argbvvt_dn4_slot = var_argbvvt_dn4;
        *var_argbvvt_dn5_slot = var_argbvvt_dn5;
        *var_argbvvt_dn6_slot = var_argbvvt_dn6;
        *var_argbvvt_dn7_slot = var_argbvvt_dn7;
        *var_argbvvt_dn8_slot = var_argbvvt_dn8;
        *var_argbvvt_dn9_slot = var_argbvvt_dn9;
        *var_argbvvt_rdb0_slot = var_argbvvt_rdb0;
        *var_argbvvt_rdb1_slot = var_argbvvt_rdb1;
        *var_argbvvt_rdb2_slot = var_argbvvt_rdb2;
        *var_argbvvt_rdb3_slot = var_argbvvt_rdb3;
        *var_argbvvt_rdb4_slot = var_argbvvt_rdb4;
        *var_argbvvt_rdb5_slot = var_argbvvt_rdb5;
        *var_argbvvt_rdb6_slot = var_argbvvt_rdb6;
        *var_argbvvt_rdb7_slot = var_argbvvt_rdb7;
        *var_argbvvt_rdn0_slot = var_argbvvt_rdn0;
        *var_argbvvt_rdn1_slot = var_argbvvt_rdn1;
        *var_argbvvt_rdn2_slot = var_argbvvt_rdn2;
        *var_argbvvt_rdn3_slot = var_argbvvt_rdn3;
        *var_argbvvt_rdn4_slot = var_argbvvt_rdn4;
        *var_argbvvt_rdn5_slot = var_argbvvt_rdn5;
        *var_argbvvt_rdn6_slot = var_argbvvt_rdn6;
        *var_argbvvt_rdn7_slot = var_argbvvt_rdn7;
        *var_argbvvt_rdn8_slot = var_argbvvt_rdn8;
        *var_argbvvt_rdn9_slot = var_argbvvt_rdn9;
        *var_argbvvt_rv_slot = var_argbvvt_rv;
        *var_guard12_slot = var_guard12;
        *var_guard12_db0_slot = var_guard12_db0;
        *var_guard12_db1_slot = var_guard12_db1;
        *var_guard12_db2_slot = var_guard12_db2;
        *var_guard12_db3_slot = var_guard12_db3;
        *var_guard12_db4_slot = var_guard12_db4;
        *var_guard12_db5_slot = var_guard12_db5;
        *var_guard12_db6_slot = var_guard12_db6;
        *var_guard12_db7_slot = var_guard12_db7;
        *var_guard12_dn0_slot = var_guard12_dn0;
        *var_guard12_dn1_slot = var_guard12_dn1;
        *var_guard12_dn2_slot = var_guard12_dn2;
        *var_guard12_dn3_slot = var_guard12_dn3;
        *var_guard12_dn4_slot = var_guard12_dn4;
        *var_guard12_dn5_slot = var_guard12_dn5;
        *var_guard12_dn6_slot = var_guard12_dn6;
        *var_guard12_dn7_slot = var_guard12_dn7;
        *var_guard12_dn8_slot = var_guard12_dn8;
        *var_guard12_dn9_slot = var_guard12_dn9;
        *var_guard12_rdb0_slot = var_guard12_rdb0;
        *var_guard12_rdb1_slot = var_guard12_rdb1;
        *var_guard12_rdb2_slot = var_guard12_rdb2;
        *var_guard12_rdb3_slot = var_guard12_rdb3;
        *var_guard12_rdb4_slot = var_guard12_rdb4;
        *var_guard12_rdb5_slot = var_guard12_rdb5;
        *var_guard12_rdb6_slot = var_guard12_rdb6;
        *var_guard12_rdb7_slot = var_guard12_rdb7;
        *var_guard12_rdn0_slot = var_guard12_rdn0;
        *var_guard12_rdn1_slot = var_guard12_rdn1;
        *var_guard12_rdn2_slot = var_guard12_rdn2;
        *var_guard12_rdn3_slot = var_guard12_rdn3;
        *var_guard12_rdn4_slot = var_guard12_rdn4;
        *var_guard12_rdn5_slot = var_guard12_rdn5;
        *var_guard12_rdn6_slot = var_guard12_rdn6;
        *var_guard12_rdn7_slot = var_guard12_rdn7;
        *var_guard12_rdn8_slot = var_guard12_rdn8;
        *var_guard12_rdn9_slot = var_guard12_rdn9;
        *var_guard12_rv_slot = var_guard12_rv;
        *var_le_slot = var_le;
        *var_le_db0_slot = var_le_db0;
        *var_le_db1_slot = var_le_db1;
        *var_le_db2_slot = var_le_db2;
        *var_le_db3_slot = var_le_db3;
        *var_le_db4_slot = var_le_db4;
        *var_le_db5_slot = var_le_db5;
        *var_le_db6_slot = var_le_db6;
        *var_le_db7_slot = var_le_db7;
        *var_le_dn0_slot = var_le_dn0;
        *var_le_dn1_slot = var_le_dn1;
        *var_le_dn2_slot = var_le_dn2;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn5_slot = var_le_dn5;
        *var_le_dn6_slot = var_le_dn6;
        *var_le_dn7_slot = var_le_dn7;
        *var_le_dn8_slot = var_le_dn8;
        *var_le_dn9_slot = var_le_dn9;
        *var_le_rdb0_slot = var_le_rdb0;
        *var_le_rdb1_slot = var_le_rdb1;
        *var_le_rdb2_slot = var_le_rdb2;
        *var_le_rdb3_slot = var_le_rdb3;
        *var_le_rdb4_slot = var_le_rdb4;
        *var_le_rdb5_slot = var_le_rdb5;
        *var_le_rdb6_slot = var_le_rdb6;
        *var_le_rdb7_slot = var_le_rdb7;
        *var_le_rdn0_slot = var_le_rdn0;
        *var_le_rdn1_slot = var_le_rdn1;
        *var_le_rdn2_slot = var_le_rdn2;
        *var_le_rdn3_slot = var_le_rdn3;
        *var_le_rdn4_slot = var_le_rdn4;
        *var_le_rdn5_slot = var_le_rdn5;
        *var_le_rdn6_slot = var_le_rdn6;
        *var_le_rdn7_slot = var_le_rdn7;
        *var_le_rdn8_slot = var_le_rdn8;
        *var_le_rdn9_slot = var_le_rdn9;
        *var_le_rv_slot = var_le_rv;
        *var_lebv_slot = var_lebv;
        *var_lebv_db0_slot = var_lebv_db0;
        *var_lebv_db1_slot = var_lebv_db1;
        *var_lebv_db2_slot = var_lebv_db2;
        *var_lebv_db3_slot = var_lebv_db3;
        *var_lebv_db4_slot = var_lebv_db4;
        *var_lebv_db5_slot = var_lebv_db5;
        *var_lebv_db6_slot = var_lebv_db6;
        *var_lebv_db7_slot = var_lebv_db7;
        *var_lebv_dn0_slot = var_lebv_dn0;
        *var_lebv_dn1_slot = var_lebv_dn1;
        *var_lebv_dn2_slot = var_lebv_dn2;
        *var_lebv_dn3_slot = var_lebv_dn3;
        *var_lebv_dn4_slot = var_lebv_dn4;
        *var_lebv_dn5_slot = var_lebv_dn5;
        *var_lebv_dn6_slot = var_lebv_dn6;
        *var_lebv_dn7_slot = var_lebv_dn7;
        *var_lebv_dn8_slot = var_lebv_dn8;
        *var_lebv_dn9_slot = var_lebv_dn9;
        *var_lebv_rdb0_slot = var_lebv_rdb0;
        *var_lebv_rdb1_slot = var_lebv_rdb1;
        *var_lebv_rdb2_slot = var_lebv_rdb2;
        *var_lebv_rdb3_slot = var_lebv_rdb3;
        *var_lebv_rdb4_slot = var_lebv_rdb4;
        *var_lebv_rdb5_slot = var_lebv_rdb5;
        *var_lebv_rdb6_slot = var_lebv_rdb6;
        *var_lebv_rdb7_slot = var_lebv_rdb7;
        *var_lebv_rdn0_slot = var_lebv_rdn0;
        *var_lebv_rdn1_slot = var_lebv_rdn1;
        *var_lebv_rdn2_slot = var_lebv_rdn2;
        *var_lebv_rdn3_slot = var_lebv_rdn3;
        *var_lebv_rdn4_slot = var_lebv_rdn4;
        *var_lebv_rdn5_slot = var_lebv_rdn5;
        *var_lebv_rdn6_slot = var_lebv_rdn6;
        *var_lebv_rdn7_slot = var_lebv_rdn7;
        *var_lebv_rdn8_slot = var_lebv_rdn8;
        *var_lebv_rdn9_slot = var_lebv_rdn9;
        *var_lebv_rv_slot = var_lebv_rv;
    }

    pub(super) fn stamp_reactive_block_11(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_ibwd: f64,
        var_ibwd_db0: f64,
        var_ibwd_db1: f64,
        var_ibwd_db2: f64,
        var_ibwd_db3: f64,
        var_ibwd_db4: f64,
        var_ibwd_db5: f64,
        var_ibwd_db6: f64,
        var_ibwd_db7: f64,
        var_ibwd_dn0: f64,
        var_ibwd_dn1: f64,
        var_ibwd_dn2: f64,
        var_ibwd_dn3: f64,
        var_ibwd_dn4: f64,
        var_ibwd_dn5: f64,
        var_ibwd_dn6: f64,
        var_ibwd_dn7: f64,
        var_ibwd_dn8: f64,
        var_ibwd_dn9: f64,
        var_ifwd: f64,
        var_ifwd_db0: f64,
        var_ifwd_db1: f64,
        var_ifwd_db2: f64,
        var_ifwd_db3: f64,
        var_ifwd_db4: f64,
        var_ifwd_db5: f64,
        var_ifwd_db6: f64,
        var_ifwd_db7: f64,
        var_ifwd_dn0: f64,
        var_ifwd_dn1: f64,
        var_ifwd_dn2: f64,
        var_ifwd_dn3: f64,
        var_ifwd_dn4: f64,
        var_ifwd_dn5: f64,
        var_ifwd_dn6: f64,
        var_ifwd_dn7: f64,
        var_ifwd_dn8: f64,
        var_ifwd_dn9: f64,
        var_oikr: f64,
        var_oikr_db0: f64,
        var_oikr_db1: f64,
        var_oikr_db2: f64,
        var_oikr_db3: f64,
        var_oikr_db4: f64,
        var_oikr_db5: f64,
        var_oikr_db6: f64,
        var_oikr_db7: f64,
        var_oikr_dn0: f64,
        var_oikr_dn1: f64,
        var_oikr_dn2: f64,
        var_oikr_dn3: f64,
        var_oikr_dn4: f64,
        var_oikr_dn5: f64,
        var_oikr_dn6: f64,
        var_oikr_dn7: f64,
        var_oikr_dn8: f64,
        var_oikr_dn9: f64,
        var_ovaf: f64,
        var_ovaf_db0: f64,
        var_ovaf_db1: f64,
        var_ovaf_db2: f64,
        var_ovaf_db3: f64,
        var_ovaf_db4: f64,
        var_ovaf_db5: f64,
        var_ovaf_db6: f64,
        var_ovaf_db7: f64,
        var_ovaf_dn0: f64,
        var_ovaf_dn1: f64,
        var_ovaf_dn2: f64,
        var_ovaf_dn3: f64,
        var_ovaf_dn4: f64,
        var_ovaf_dn5: f64,
        var_ovaf_dn6: f64,
        var_ovaf_dn7: f64,
        var_ovaf_dn8: f64,
        var_ovaf_dn9: f64,
        var_ovar: f64,
        var_ovar_db0: f64,
        var_ovar_db1: f64,
        var_ovar_db2: f64,
        var_ovar_db3: f64,
        var_ovar_db4: f64,
        var_ovar_db5: f64,
        var_ovar_db6: f64,
        var_ovar_db7: f64,
        var_ovar_dn0: f64,
        var_ovar_dn1: f64,
        var_ovar_dn2: f64,
        var_ovar_dn3: f64,
        var_ovar_dn4: f64,
        var_ovar_dn5: f64,
        var_ovar_dn6: f64,
        var_ovar_dn7: f64,
        var_ovar_dn8: f64,
        var_ovar_dn9: f64,
        var_vbici: f64,
        var_vbici_db0: f64,
        var_vbici_db1: f64,
        var_vbici_db2: f64,
        var_vbici_db3: f64,
        var_vbici_db4: f64,
        var_vbici_db5: f64,
        var_vbici_db6: f64,
        var_vbici_db7: f64,
        var_vbici_dn0: f64,
        var_vbici_dn1: f64,
        var_vbici_dn2: f64,
        var_vbici_dn3: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbici_dn6: f64,
        var_vbici_dn7: f64,
        var_vbici_dn8: f64,
        var_vbici_dn9: f64,
        var_vbiei: f64,
        var_vbiei_db0: f64,
        var_vbiei_db1: f64,
        var_vbiei_db2: f64,
        var_vbiei_db3: f64,
        var_vbiei_db4: f64,
        var_vbiei_db5: f64,
        var_vbiei_db6: f64,
        var_vbiei_db7: f64,
        var_vbiei_dn0: f64,
        var_vbiei_dn1: f64,
        var_vbiei_dn2: f64,
        var_vbiei_dn3: f64,
        var_vbiei_dn4: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vbiei_dn7: f64,
        var_vbiei_dn8: f64,
        var_vbiei_dn9: f64,
        var_dkqb_slot: &mut f64,
        var_dkqb_db0_slot: &mut f64,
        var_dkqb_db1_slot: &mut f64,
        var_dkqb_db2_slot: &mut f64,
        var_dkqb_db3_slot: &mut f64,
        var_dkqb_db4_slot: &mut f64,
        var_dkqb_db5_slot: &mut f64,
        var_dkqb_db6_slot: &mut f64,
        var_dkqb_db7_slot: &mut f64,
        var_dkqb_dn0_slot: &mut f64,
        var_dkqb_dn1_slot: &mut f64,
        var_dkqb_dn2_slot: &mut f64,
        var_dkqb_dn3_slot: &mut f64,
        var_dkqb_dn4_slot: &mut f64,
        var_dkqb_dn5_slot: &mut f64,
        var_dkqb_dn6_slot: &mut f64,
        var_dkqb_dn7_slot: &mut f64,
        var_dkqb_dn8_slot: &mut f64,
        var_dkqb_dn9_slot: &mut f64,
        var_dkqb_rdb0_slot: &mut f64,
        var_dkqb_rdb1_slot: &mut f64,
        var_dkqb_rdb2_slot: &mut f64,
        var_dkqb_rdb3_slot: &mut f64,
        var_dkqb_rdb4_slot: &mut f64,
        var_dkqb_rdb5_slot: &mut f64,
        var_dkqb_rdb6_slot: &mut f64,
        var_dkqb_rdb7_slot: &mut f64,
        var_dkqb_rdn0_slot: &mut f64,
        var_dkqb_rdn1_slot: &mut f64,
        var_dkqb_rdn2_slot: &mut f64,
        var_dkqb_rdn3_slot: &mut f64,
        var_dkqb_rdn4_slot: &mut f64,
        var_dkqb_rdn5_slot: &mut f64,
        var_dkqb_rdn6_slot: &mut f64,
        var_dkqb_rdn7_slot: &mut f64,
        var_dkqb_rdn8_slot: &mut f64,
        var_dkqb_rdn9_slot: &mut f64,
        var_dkqb_rv_slot: &mut f64,
        var_ikq1_slot: &mut f64,
        var_ikq1_db0_slot: &mut f64,
        var_ikq1_db1_slot: &mut f64,
        var_ikq1_db2_slot: &mut f64,
        var_ikq1_db3_slot: &mut f64,
        var_ikq1_db4_slot: &mut f64,
        var_ikq1_db5_slot: &mut f64,
        var_ikq1_db6_slot: &mut f64,
        var_ikq1_db7_slot: &mut f64,
        var_ikq1_dn0_slot: &mut f64,
        var_ikq1_dn1_slot: &mut f64,
        var_ikq1_dn2_slot: &mut f64,
        var_ikq1_dn3_slot: &mut f64,
        var_ikq1_dn4_slot: &mut f64,
        var_ikq1_dn5_slot: &mut f64,
        var_ikq1_dn6_slot: &mut f64,
        var_ikq1_dn7_slot: &mut f64,
        var_ikq1_dn8_slot: &mut f64,
        var_ikq1_dn9_slot: &mut f64,
        var_ikq1_rdb0_slot: &mut f64,
        var_ikq1_rdb1_slot: &mut f64,
        var_ikq1_rdb2_slot: &mut f64,
        var_ikq1_rdb3_slot: &mut f64,
        var_ikq1_rdb4_slot: &mut f64,
        var_ikq1_rdb5_slot: &mut f64,
        var_ikq1_rdb6_slot: &mut f64,
        var_ikq1_rdb7_slot: &mut f64,
        var_ikq1_rdn0_slot: &mut f64,
        var_ikq1_rdn1_slot: &mut f64,
        var_ikq1_rdn2_slot: &mut f64,
        var_ikq1_rdn3_slot: &mut f64,
        var_ikq1_rdn4_slot: &mut f64,
        var_ikq1_rdn5_slot: &mut f64,
        var_ikq1_rdn6_slot: &mut f64,
        var_ikq1_rdn7_slot: &mut f64,
        var_ikq1_rdn8_slot: &mut f64,
        var_ikq1_rdn9_slot: &mut f64,
        var_ikq1_rv_slot: &mut f64,
        var_ikqb_slot: &mut f64,
        var_ikqb_db0_slot: &mut f64,
        var_ikqb_db1_slot: &mut f64,
        var_ikqb_db2_slot: &mut f64,
        var_ikqb_db3_slot: &mut f64,
        var_ikqb_db4_slot: &mut f64,
        var_ikqb_db5_slot: &mut f64,
        var_ikqb_db6_slot: &mut f64,
        var_ikqb_db7_slot: &mut f64,
        var_ikqb_dn0_slot: &mut f64,
        var_ikqb_dn1_slot: &mut f64,
        var_ikqb_dn2_slot: &mut f64,
        var_ikqb_dn3_slot: &mut f64,
        var_ikqb_dn4_slot: &mut f64,
        var_ikqb_dn5_slot: &mut f64,
        var_ikqb_dn6_slot: &mut f64,
        var_ikqb_dn7_slot: &mut f64,
        var_ikqb_dn8_slot: &mut f64,
        var_ikqb_dn9_slot: &mut f64,
        var_ikqb_rdb0_slot: &mut f64,
        var_ikqb_rdb1_slot: &mut f64,
        var_ikqb_rdb2_slot: &mut f64,
        var_ikqb_rdb3_slot: &mut f64,
        var_ikqb_rdb4_slot: &mut f64,
        var_ikqb_rdb5_slot: &mut f64,
        var_ikqb_rdb6_slot: &mut f64,
        var_ikqb_rdb7_slot: &mut f64,
        var_ikqb_rdn0_slot: &mut f64,
        var_ikqb_rdn1_slot: &mut f64,
        var_ikqb_rdn2_slot: &mut f64,
        var_ikqb_rdn3_slot: &mut f64,
        var_ikqb_rdn4_slot: &mut f64,
        var_ikqb_rdn5_slot: &mut f64,
        var_ikqb_rdn6_slot: &mut f64,
        var_ikqb_rdn7_slot: &mut f64,
        var_ikqb_rdn8_slot: &mut f64,
        var_ikqb_rdn9_slot: &mut f64,
        var_ikqb_rv_slot: &mut f64,
        var_itr_slot: &mut f64,
        var_itr_db0_slot: &mut f64,
        var_itr_db1_slot: &mut f64,
        var_itr_db2_slot: &mut f64,
        var_itr_db3_slot: &mut f64,
        var_itr_db4_slot: &mut f64,
        var_itr_db5_slot: &mut f64,
        var_itr_db6_slot: &mut f64,
        var_itr_db7_slot: &mut f64,
        var_itr_dn0_slot: &mut f64,
        var_itr_dn1_slot: &mut f64,
        var_itr_dn2_slot: &mut f64,
        var_itr_dn3_slot: &mut f64,
        var_itr_dn4_slot: &mut f64,
        var_itr_dn5_slot: &mut f64,
        var_itr_dn6_slot: &mut f64,
        var_itr_dn7_slot: &mut f64,
        var_itr_dn8_slot: &mut f64,
        var_itr_dn9_slot: &mut f64,
        var_itr_rdb0_slot: &mut f64,
        var_itr_rdb1_slot: &mut f64,
        var_itr_rdb2_slot: &mut f64,
        var_itr_rdb3_slot: &mut f64,
        var_itr_rdb4_slot: &mut f64,
        var_itr_rdb5_slot: &mut f64,
        var_itr_rdb6_slot: &mut f64,
        var_itr_rdb7_slot: &mut f64,
        var_itr_rdn0_slot: &mut f64,
        var_itr_rdn1_slot: &mut f64,
        var_itr_rdn2_slot: &mut f64,
        var_itr_rdn3_slot: &mut f64,
        var_itr_rdn4_slot: &mut f64,
        var_itr_rdn5_slot: &mut f64,
        var_itr_rdn6_slot: &mut f64,
        var_itr_rdn7_slot: &mut f64,
        var_itr_rdn8_slot: &mut f64,
        var_itr_rdn9_slot: &mut f64,
        var_itr_rv_slot: &mut f64,
        var_itzf_slot: &mut f64,
        var_itzf_db0_slot: &mut f64,
        var_itzf_db1_slot: &mut f64,
        var_itzf_db2_slot: &mut f64,
        var_itzf_db3_slot: &mut f64,
        var_itzf_db4_slot: &mut f64,
        var_itzf_db5_slot: &mut f64,
        var_itzf_db6_slot: &mut f64,
        var_itzf_db7_slot: &mut f64,
        var_itzf_dn0_slot: &mut f64,
        var_itzf_dn1_slot: &mut f64,
        var_itzf_dn2_slot: &mut f64,
        var_itzf_dn3_slot: &mut f64,
        var_itzf_dn4_slot: &mut f64,
        var_itzf_dn5_slot: &mut f64,
        var_itzf_dn6_slot: &mut f64,
        var_itzf_dn7_slot: &mut f64,
        var_itzf_dn8_slot: &mut f64,
        var_itzf_dn9_slot: &mut f64,
        var_itzf_rdb0_slot: &mut f64,
        var_itzf_rdb1_slot: &mut f64,
        var_itzf_rdb2_slot: &mut f64,
        var_itzf_rdb3_slot: &mut f64,
        var_itzf_rdb4_slot: &mut f64,
        var_itzf_rdb5_slot: &mut f64,
        var_itzf_rdb6_slot: &mut f64,
        var_itzf_rdb7_slot: &mut f64,
        var_itzf_rdn0_slot: &mut f64,
        var_itzf_rdn1_slot: &mut f64,
        var_itzf_rdn2_slot: &mut f64,
        var_itzf_rdn3_slot: &mut f64,
        var_itzf_rdn4_slot: &mut f64,
        var_itzf_rdn5_slot: &mut f64,
        var_itzf_rdn6_slot: &mut f64,
        var_itzf_rdn7_slot: &mut f64,
        var_itzf_rdn8_slot: &mut f64,
        var_itzf_rdn9_slot: &mut f64,
        var_itzf_rv_slot: &mut f64,
        var_kq2_slot: &mut f64,
        var_kq2_db0_slot: &mut f64,
        var_kq2_db1_slot: &mut f64,
        var_kq2_db2_slot: &mut f64,
        var_kq2_db3_slot: &mut f64,
        var_kq2_db4_slot: &mut f64,
        var_kq2_db5_slot: &mut f64,
        var_kq2_db6_slot: &mut f64,
        var_kq2_db7_slot: &mut f64,
        var_kq2_dn0_slot: &mut f64,
        var_kq2_dn1_slot: &mut f64,
        var_kq2_dn2_slot: &mut f64,
        var_kq2_dn3_slot: &mut f64,
        var_kq2_dn4_slot: &mut f64,
        var_kq2_dn5_slot: &mut f64,
        var_kq2_dn6_slot: &mut f64,
        var_kq2_dn7_slot: &mut f64,
        var_kq2_dn8_slot: &mut f64,
        var_kq2_dn9_slot: &mut f64,
        var_kq2_rdb0_slot: &mut f64,
        var_kq2_rdb1_slot: &mut f64,
        var_kq2_rdb2_slot: &mut f64,
        var_kq2_rdb3_slot: &mut f64,
        var_kq2_rdb4_slot: &mut f64,
        var_kq2_rdb5_slot: &mut f64,
        var_kq2_rdb6_slot: &mut f64,
        var_kq2_rdb7_slot: &mut f64,
        var_kq2_rdn0_slot: &mut f64,
        var_kq2_rdn1_slot: &mut f64,
        var_kq2_rdn2_slot: &mut f64,
        var_kq2_rdn3_slot: &mut f64,
        var_kq2_rdn4_slot: &mut f64,
        var_kq2_rdn5_slot: &mut f64,
        var_kq2_rdn6_slot: &mut f64,
        var_kq2_rdn7_slot: &mut f64,
        var_kq2_rdn8_slot: &mut f64,
        var_kq2_rdn9_slot: &mut f64,
        var_kq2_rv_slot: &mut f64,
        var_oikf_slot: &mut f64,
        var_oikf_db0_slot: &mut f64,
        var_oikf_db1_slot: &mut f64,
        var_oikf_db2_slot: &mut f64,
        var_oikf_db3_slot: &mut f64,
        var_oikf_db4_slot: &mut f64,
        var_oikf_db5_slot: &mut f64,
        var_oikf_db6_slot: &mut f64,
        var_oikf_db7_slot: &mut f64,
        var_oikf_dn0_slot: &mut f64,
        var_oikf_dn1_slot: &mut f64,
        var_oikf_dn2_slot: &mut f64,
        var_oikf_dn3_slot: &mut f64,
        var_oikf_dn4_slot: &mut f64,
        var_oikf_dn5_slot: &mut f64,
        var_oikf_dn6_slot: &mut f64,
        var_oikf_dn7_slot: &mut f64,
        var_oikf_dn8_slot: &mut f64,
        var_oikf_dn9_slot: &mut f64,
        var_oikf_rdb0_slot: &mut f64,
        var_oikf_rdb1_slot: &mut f64,
        var_oikf_rdb2_slot: &mut f64,
        var_oikf_rdb3_slot: &mut f64,
        var_oikf_rdb4_slot: &mut f64,
        var_oikf_rdb5_slot: &mut f64,
        var_oikf_rdb6_slot: &mut f64,
        var_oikf_rdb7_slot: &mut f64,
        var_oikf_rdn0_slot: &mut f64,
        var_oikf_rdn1_slot: &mut f64,
        var_oikf_rdn2_slot: &mut f64,
        var_oikf_rdn3_slot: &mut f64,
        var_oikf_rdn4_slot: &mut f64,
        var_oikf_rdn5_slot: &mut f64,
        var_oikf_rdn6_slot: &mut f64,
        var_oikf_rdn7_slot: &mut f64,
        var_oikf_rdn8_slot: &mut f64,
        var_oikf_rdn9_slot: &mut f64,
        var_oikf_rv_slot: &mut f64,
        var_qdc_slot: &mut f64,
        var_qdc_db0_slot: &mut f64,
        var_qdc_db1_slot: &mut f64,
        var_qdc_db2_slot: &mut f64,
        var_qdc_db3_slot: &mut f64,
        var_qdc_db4_slot: &mut f64,
        var_qdc_db5_slot: &mut f64,
        var_qdc_db6_slot: &mut f64,
        var_qdc_db7_slot: &mut f64,
        var_qdc_dn0_slot: &mut f64,
        var_qdc_dn1_slot: &mut f64,
        var_qdc_dn2_slot: &mut f64,
        var_qdc_dn3_slot: &mut f64,
        var_qdc_dn4_slot: &mut f64,
        var_qdc_dn5_slot: &mut f64,
        var_qdc_dn6_slot: &mut f64,
        var_qdc_dn7_slot: &mut f64,
        var_qdc_dn8_slot: &mut f64,
        var_qdc_dn9_slot: &mut f64,
        var_qdc_rdb0_slot: &mut f64,
        var_qdc_rdb1_slot: &mut f64,
        var_qdc_rdb2_slot: &mut f64,
        var_qdc_rdb3_slot: &mut f64,
        var_qdc_rdb4_slot: &mut f64,
        var_qdc_rdb5_slot: &mut f64,
        var_qdc_rdb6_slot: &mut f64,
        var_qdc_rdb7_slot: &mut f64,
        var_qdc_rdn0_slot: &mut f64,
        var_qdc_rdn1_slot: &mut f64,
        var_qdc_rdn2_slot: &mut f64,
        var_qdc_rdn3_slot: &mut f64,
        var_qdc_rdn4_slot: &mut f64,
        var_qdc_rdn5_slot: &mut f64,
        var_qdc_rdn6_slot: &mut f64,
        var_qdc_rdn7_slot: &mut f64,
        var_qdc_rdn8_slot: &mut f64,
        var_qdc_rdn9_slot: &mut f64,
        var_qdc_rv_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_db0_slot: &mut f64,
        var_qde_db1_slot: &mut f64,
        var_qde_db2_slot: &mut f64,
        var_qde_db3_slot: &mut f64,
        var_qde_db4_slot: &mut f64,
        var_qde_db5_slot: &mut f64,
        var_qde_db6_slot: &mut f64,
        var_qde_db7_slot: &mut f64,
        var_qde_dn0_slot: &mut f64,
        var_qde_dn1_slot: &mut f64,
        var_qde_dn2_slot: &mut f64,
        var_qde_dn3_slot: &mut f64,
        var_qde_dn4_slot: &mut f64,
        var_qde_dn5_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qde_dn7_slot: &mut f64,
        var_qde_dn8_slot: &mut f64,
        var_qde_dn9_slot: &mut f64,
        var_qde_rdb0_slot: &mut f64,
        var_qde_rdb1_slot: &mut f64,
        var_qde_rdb2_slot: &mut f64,
        var_qde_rdb3_slot: &mut f64,
        var_qde_rdb4_slot: &mut f64,
        var_qde_rdb5_slot: &mut f64,
        var_qde_rdb6_slot: &mut f64,
        var_qde_rdb7_slot: &mut f64,
        var_qde_rdn0_slot: &mut f64,
        var_qde_rdn1_slot: &mut f64,
        var_qde_rdn2_slot: &mut f64,
        var_qde_rdn3_slot: &mut f64,
        var_qde_rdn4_slot: &mut f64,
        var_qde_rdn5_slot: &mut f64,
        var_qde_rdn6_slot: &mut f64,
        var_qde_rdn7_slot: &mut f64,
        var_qde_rdn8_slot: &mut f64,
        var_qde_rdn9_slot: &mut f64,
        var_qde_rv_slot: &mut f64,
        var_tff_slot: &mut f64,
        var_tff_db0_slot: &mut f64,
        var_tff_db1_slot: &mut f64,
        var_tff_db2_slot: &mut f64,
        var_tff_db3_slot: &mut f64,
        var_tff_db4_slot: &mut f64,
        var_tff_db5_slot: &mut f64,
        var_tff_db6_slot: &mut f64,
        var_tff_db7_slot: &mut f64,
        var_tff_dn0_slot: &mut f64,
        var_tff_dn1_slot: &mut f64,
        var_tff_dn2_slot: &mut f64,
        var_tff_dn3_slot: &mut f64,
        var_tff_dn4_slot: &mut f64,
        var_tff_dn5_slot: &mut f64,
        var_tff_dn6_slot: &mut f64,
        var_tff_dn7_slot: &mut f64,
        var_tff_dn8_slot: &mut f64,
        var_tff_dn9_slot: &mut f64,
        var_tff_rdb0_slot: &mut f64,
        var_tff_rdb1_slot: &mut f64,
        var_tff_rdb2_slot: &mut f64,
        var_tff_rdb3_slot: &mut f64,
        var_tff_rdb4_slot: &mut f64,
        var_tff_rdb5_slot: &mut f64,
        var_tff_rdb6_slot: &mut f64,
        var_tff_rdb7_slot: &mut f64,
        var_tff_rdn0_slot: &mut f64,
        var_tff_rdn1_slot: &mut f64,
        var_tff_rdn2_slot: &mut f64,
        var_tff_rdn3_slot: &mut f64,
        var_tff_rdn4_slot: &mut f64,
        var_tff_rdn5_slot: &mut f64,
        var_tff_rdn6_slot: &mut f64,
        var_tff_rdn7_slot: &mut f64,
        var_tff_rdn8_slot: &mut f64,
        var_tff_rdn9_slot: &mut f64,
        var_tff_rv_slot: &mut f64,
        var_vtff_slot: &mut f64,
        var_vtff1_slot: &mut f64,
        var_vtff1_db0_slot: &mut f64,
        var_vtff1_db1_slot: &mut f64,
        var_vtff1_db2_slot: &mut f64,
        var_vtff1_db3_slot: &mut f64,
        var_vtff1_db4_slot: &mut f64,
        var_vtff1_db5_slot: &mut f64,
        var_vtff1_db6_slot: &mut f64,
        var_vtff1_db7_slot: &mut f64,
        var_vtff1_dn0_slot: &mut f64,
        var_vtff1_dn1_slot: &mut f64,
        var_vtff1_dn2_slot: &mut f64,
        var_vtff1_dn3_slot: &mut f64,
        var_vtff1_dn4_slot: &mut f64,
        var_vtff1_dn5_slot: &mut f64,
        var_vtff1_dn6_slot: &mut f64,
        var_vtff1_dn7_slot: &mut f64,
        var_vtff1_dn8_slot: &mut f64,
        var_vtff1_dn9_slot: &mut f64,
        var_vtff1_rdb0_slot: &mut f64,
        var_vtff1_rdb1_slot: &mut f64,
        var_vtff1_rdb2_slot: &mut f64,
        var_vtff1_rdb3_slot: &mut f64,
        var_vtff1_rdb4_slot: &mut f64,
        var_vtff1_rdb5_slot: &mut f64,
        var_vtff1_rdb6_slot: &mut f64,
        var_vtff1_rdb7_slot: &mut f64,
        var_vtff1_rdn0_slot: &mut f64,
        var_vtff1_rdn1_slot: &mut f64,
        var_vtff1_rdn2_slot: &mut f64,
        var_vtff1_rdn3_slot: &mut f64,
        var_vtff1_rdn4_slot: &mut f64,
        var_vtff1_rdn5_slot: &mut f64,
        var_vtff1_rdn6_slot: &mut f64,
        var_vtff1_rdn7_slot: &mut f64,
        var_vtff1_rdn8_slot: &mut f64,
        var_vtff1_rdn9_slot: &mut f64,
        var_vtff1_rv_slot: &mut f64,
        var_vtff_db0_slot: &mut f64,
        var_vtff_db1_slot: &mut f64,
        var_vtff_db2_slot: &mut f64,
        var_vtff_db3_slot: &mut f64,
        var_vtff_db4_slot: &mut f64,
        var_vtff_db5_slot: &mut f64,
        var_vtff_db6_slot: &mut f64,
        var_vtff_db7_slot: &mut f64,
        var_vtff_dn0_slot: &mut f64,
        var_vtff_dn1_slot: &mut f64,
        var_vtff_dn2_slot: &mut f64,
        var_vtff_dn3_slot: &mut f64,
        var_vtff_dn4_slot: &mut f64,
        var_vtff_dn5_slot: &mut f64,
        var_vtff_dn6_slot: &mut f64,
        var_vtff_dn7_slot: &mut f64,
        var_vtff_dn8_slot: &mut f64,
        var_vtff_dn9_slot: &mut f64,
        var_vtff_rdb0_slot: &mut f64,
        var_vtff_rdb1_slot: &mut f64,
        var_vtff_rdb2_slot: &mut f64,
        var_vtff_rdb3_slot: &mut f64,
        var_vtff_rdb4_slot: &mut f64,
        var_vtff_rdb5_slot: &mut f64,
        var_vtff_rdb6_slot: &mut f64,
        var_vtff_rdb7_slot: &mut f64,
        var_vtff_rdn0_slot: &mut f64,
        var_vtff_rdn1_slot: &mut f64,
        var_vtff_rdn2_slot: &mut f64,
        var_vtff_rdn3_slot: &mut f64,
        var_vtff_rdn4_slot: &mut f64,
        var_vtff_rdn5_slot: &mut f64,
        var_vtff_rdn6_slot: &mut f64,
        var_vtff_rdn7_slot: &mut f64,
        var_vtff_rdn8_slot: &mut f64,
        var_vtff_rdn9_slot: &mut f64,
        var_vtff_rv_slot: &mut f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let mut var_dkqb: f64 = *var_dkqb_slot;
        let mut var_dkqb_db0: f64 = *var_dkqb_db0_slot;
        let mut var_dkqb_db1: f64 = *var_dkqb_db1_slot;
        let mut var_dkqb_db2: f64 = *var_dkqb_db2_slot;
        let mut var_dkqb_db3: f64 = *var_dkqb_db3_slot;
        let mut var_dkqb_db4: f64 = *var_dkqb_db4_slot;
        let mut var_dkqb_db5: f64 = *var_dkqb_db5_slot;
        let mut var_dkqb_db6: f64 = *var_dkqb_db6_slot;
        let mut var_dkqb_db7: f64 = *var_dkqb_db7_slot;
        let mut var_dkqb_dn0: f64 = *var_dkqb_dn0_slot;
        let mut var_dkqb_dn1: f64 = *var_dkqb_dn1_slot;
        let mut var_dkqb_dn2: f64 = *var_dkqb_dn2_slot;
        let mut var_dkqb_dn3: f64 = *var_dkqb_dn3_slot;
        let mut var_dkqb_dn4: f64 = *var_dkqb_dn4_slot;
        let mut var_dkqb_dn5: f64 = *var_dkqb_dn5_slot;
        let mut var_dkqb_dn6: f64 = *var_dkqb_dn6_slot;
        let mut var_dkqb_dn7: f64 = *var_dkqb_dn7_slot;
        let mut var_dkqb_dn8: f64 = *var_dkqb_dn8_slot;
        let mut var_dkqb_dn9: f64 = *var_dkqb_dn9_slot;
        let mut var_dkqb_rdb0: f64 = *var_dkqb_rdb0_slot;
        let mut var_dkqb_rdb1: f64 = *var_dkqb_rdb1_slot;
        let mut var_dkqb_rdb2: f64 = *var_dkqb_rdb2_slot;
        let mut var_dkqb_rdb3: f64 = *var_dkqb_rdb3_slot;
        let mut var_dkqb_rdb4: f64 = *var_dkqb_rdb4_slot;
        let mut var_dkqb_rdb5: f64 = *var_dkqb_rdb5_slot;
        let mut var_dkqb_rdb6: f64 = *var_dkqb_rdb6_slot;
        let mut var_dkqb_rdb7: f64 = *var_dkqb_rdb7_slot;
        let mut var_dkqb_rdn0: f64 = *var_dkqb_rdn0_slot;
        let mut var_dkqb_rdn1: f64 = *var_dkqb_rdn1_slot;
        let mut var_dkqb_rdn2: f64 = *var_dkqb_rdn2_slot;
        let mut var_dkqb_rdn3: f64 = *var_dkqb_rdn3_slot;
        let mut var_dkqb_rdn4: f64 = *var_dkqb_rdn4_slot;
        let mut var_dkqb_rdn5: f64 = *var_dkqb_rdn5_slot;
        let mut var_dkqb_rdn6: f64 = *var_dkqb_rdn6_slot;
        let mut var_dkqb_rdn7: f64 = *var_dkqb_rdn7_slot;
        let mut var_dkqb_rdn8: f64 = *var_dkqb_rdn8_slot;
        let mut var_dkqb_rdn9: f64 = *var_dkqb_rdn9_slot;
        let mut var_dkqb_rv: f64 = *var_dkqb_rv_slot;
        let mut var_ikq1: f64 = *var_ikq1_slot;
        let mut var_ikq1_db0: f64 = *var_ikq1_db0_slot;
        let mut var_ikq1_db1: f64 = *var_ikq1_db1_slot;
        let mut var_ikq1_db2: f64 = *var_ikq1_db2_slot;
        let mut var_ikq1_db3: f64 = *var_ikq1_db3_slot;
        let mut var_ikq1_db4: f64 = *var_ikq1_db4_slot;
        let mut var_ikq1_db5: f64 = *var_ikq1_db5_slot;
        let mut var_ikq1_db6: f64 = *var_ikq1_db6_slot;
        let mut var_ikq1_db7: f64 = *var_ikq1_db7_slot;
        let mut var_ikq1_dn0: f64 = *var_ikq1_dn0_slot;
        let mut var_ikq1_dn1: f64 = *var_ikq1_dn1_slot;
        let mut var_ikq1_dn2: f64 = *var_ikq1_dn2_slot;
        let mut var_ikq1_dn3: f64 = *var_ikq1_dn3_slot;
        let mut var_ikq1_dn4: f64 = *var_ikq1_dn4_slot;
        let mut var_ikq1_dn5: f64 = *var_ikq1_dn5_slot;
        let mut var_ikq1_dn6: f64 = *var_ikq1_dn6_slot;
        let mut var_ikq1_dn7: f64 = *var_ikq1_dn7_slot;
        let mut var_ikq1_dn8: f64 = *var_ikq1_dn8_slot;
        let mut var_ikq1_dn9: f64 = *var_ikq1_dn9_slot;
        let mut var_ikq1_rdb0: f64 = *var_ikq1_rdb0_slot;
        let mut var_ikq1_rdb1: f64 = *var_ikq1_rdb1_slot;
        let mut var_ikq1_rdb2: f64 = *var_ikq1_rdb2_slot;
        let mut var_ikq1_rdb3: f64 = *var_ikq1_rdb3_slot;
        let mut var_ikq1_rdb4: f64 = *var_ikq1_rdb4_slot;
        let mut var_ikq1_rdb5: f64 = *var_ikq1_rdb5_slot;
        let mut var_ikq1_rdb6: f64 = *var_ikq1_rdb6_slot;
        let mut var_ikq1_rdb7: f64 = *var_ikq1_rdb7_slot;
        let mut var_ikq1_rdn0: f64 = *var_ikq1_rdn0_slot;
        let mut var_ikq1_rdn1: f64 = *var_ikq1_rdn1_slot;
        let mut var_ikq1_rdn2: f64 = *var_ikq1_rdn2_slot;
        let mut var_ikq1_rdn3: f64 = *var_ikq1_rdn3_slot;
        let mut var_ikq1_rdn4: f64 = *var_ikq1_rdn4_slot;
        let mut var_ikq1_rdn5: f64 = *var_ikq1_rdn5_slot;
        let mut var_ikq1_rdn6: f64 = *var_ikq1_rdn6_slot;
        let mut var_ikq1_rdn7: f64 = *var_ikq1_rdn7_slot;
        let mut var_ikq1_rdn8: f64 = *var_ikq1_rdn8_slot;
        let mut var_ikq1_rdn9: f64 = *var_ikq1_rdn9_slot;
        let mut var_ikq1_rv: f64 = *var_ikq1_rv_slot;
        let mut var_ikqb: f64 = *var_ikqb_slot;
        let mut var_ikqb_db0: f64 = *var_ikqb_db0_slot;
        let mut var_ikqb_db1: f64 = *var_ikqb_db1_slot;
        let mut var_ikqb_db2: f64 = *var_ikqb_db2_slot;
        let mut var_ikqb_db3: f64 = *var_ikqb_db3_slot;
        let mut var_ikqb_db4: f64 = *var_ikqb_db4_slot;
        let mut var_ikqb_db5: f64 = *var_ikqb_db5_slot;
        let mut var_ikqb_db6: f64 = *var_ikqb_db6_slot;
        let mut var_ikqb_db7: f64 = *var_ikqb_db7_slot;
        let mut var_ikqb_dn0: f64 = *var_ikqb_dn0_slot;
        let mut var_ikqb_dn1: f64 = *var_ikqb_dn1_slot;
        let mut var_ikqb_dn2: f64 = *var_ikqb_dn2_slot;
        let mut var_ikqb_dn3: f64 = *var_ikqb_dn3_slot;
        let mut var_ikqb_dn4: f64 = *var_ikqb_dn4_slot;
        let mut var_ikqb_dn5: f64 = *var_ikqb_dn5_slot;
        let mut var_ikqb_dn6: f64 = *var_ikqb_dn6_slot;
        let mut var_ikqb_dn7: f64 = *var_ikqb_dn7_slot;
        let mut var_ikqb_dn8: f64 = *var_ikqb_dn8_slot;
        let mut var_ikqb_dn9: f64 = *var_ikqb_dn9_slot;
        let mut var_ikqb_rdb0: f64 = *var_ikqb_rdb0_slot;
        let mut var_ikqb_rdb1: f64 = *var_ikqb_rdb1_slot;
        let mut var_ikqb_rdb2: f64 = *var_ikqb_rdb2_slot;
        let mut var_ikqb_rdb3: f64 = *var_ikqb_rdb3_slot;
        let mut var_ikqb_rdb4: f64 = *var_ikqb_rdb4_slot;
        let mut var_ikqb_rdb5: f64 = *var_ikqb_rdb5_slot;
        let mut var_ikqb_rdb6: f64 = *var_ikqb_rdb6_slot;
        let mut var_ikqb_rdb7: f64 = *var_ikqb_rdb7_slot;
        let mut var_ikqb_rdn0: f64 = *var_ikqb_rdn0_slot;
        let mut var_ikqb_rdn1: f64 = *var_ikqb_rdn1_slot;
        let mut var_ikqb_rdn2: f64 = *var_ikqb_rdn2_slot;
        let mut var_ikqb_rdn3: f64 = *var_ikqb_rdn3_slot;
        let mut var_ikqb_rdn4: f64 = *var_ikqb_rdn4_slot;
        let mut var_ikqb_rdn5: f64 = *var_ikqb_rdn5_slot;
        let mut var_ikqb_rdn6: f64 = *var_ikqb_rdn6_slot;
        let mut var_ikqb_rdn7: f64 = *var_ikqb_rdn7_slot;
        let mut var_ikqb_rdn8: f64 = *var_ikqb_rdn8_slot;
        let mut var_ikqb_rdn9: f64 = *var_ikqb_rdn9_slot;
        let mut var_ikqb_rv: f64 = *var_ikqb_rv_slot;
        let mut var_itr: f64 = *var_itr_slot;
        let mut var_itr_db0: f64 = *var_itr_db0_slot;
        let mut var_itr_db1: f64 = *var_itr_db1_slot;
        let mut var_itr_db2: f64 = *var_itr_db2_slot;
        let mut var_itr_db3: f64 = *var_itr_db3_slot;
        let mut var_itr_db4: f64 = *var_itr_db4_slot;
        let mut var_itr_db5: f64 = *var_itr_db5_slot;
        let mut var_itr_db6: f64 = *var_itr_db6_slot;
        let mut var_itr_db7: f64 = *var_itr_db7_slot;
        let mut var_itr_dn0: f64 = *var_itr_dn0_slot;
        let mut var_itr_dn1: f64 = *var_itr_dn1_slot;
        let mut var_itr_dn2: f64 = *var_itr_dn2_slot;
        let mut var_itr_dn3: f64 = *var_itr_dn3_slot;
        let mut var_itr_dn4: f64 = *var_itr_dn4_slot;
        let mut var_itr_dn5: f64 = *var_itr_dn5_slot;
        let mut var_itr_dn6: f64 = *var_itr_dn6_slot;
        let mut var_itr_dn7: f64 = *var_itr_dn7_slot;
        let mut var_itr_dn8: f64 = *var_itr_dn8_slot;
        let mut var_itr_dn9: f64 = *var_itr_dn9_slot;
        let mut var_itr_rdb0: f64 = *var_itr_rdb0_slot;
        let mut var_itr_rdb1: f64 = *var_itr_rdb1_slot;
        let mut var_itr_rdb2: f64 = *var_itr_rdb2_slot;
        let mut var_itr_rdb3: f64 = *var_itr_rdb3_slot;
        let mut var_itr_rdb4: f64 = *var_itr_rdb4_slot;
        let mut var_itr_rdb5: f64 = *var_itr_rdb5_slot;
        let mut var_itr_rdb6: f64 = *var_itr_rdb6_slot;
        let mut var_itr_rdb7: f64 = *var_itr_rdb7_slot;
        let mut var_itr_rdn0: f64 = *var_itr_rdn0_slot;
        let mut var_itr_rdn1: f64 = *var_itr_rdn1_slot;
        let mut var_itr_rdn2: f64 = *var_itr_rdn2_slot;
        let mut var_itr_rdn3: f64 = *var_itr_rdn3_slot;
        let mut var_itr_rdn4: f64 = *var_itr_rdn4_slot;
        let mut var_itr_rdn5: f64 = *var_itr_rdn5_slot;
        let mut var_itr_rdn6: f64 = *var_itr_rdn6_slot;
        let mut var_itr_rdn7: f64 = *var_itr_rdn7_slot;
        let mut var_itr_rdn8: f64 = *var_itr_rdn8_slot;
        let mut var_itr_rdn9: f64 = *var_itr_rdn9_slot;
        let mut var_itr_rv: f64 = *var_itr_rv_slot;
        let mut var_itzf: f64 = *var_itzf_slot;
        let mut var_itzf_db0: f64 = *var_itzf_db0_slot;
        let mut var_itzf_db1: f64 = *var_itzf_db1_slot;
        let mut var_itzf_db2: f64 = *var_itzf_db2_slot;
        let mut var_itzf_db3: f64 = *var_itzf_db3_slot;
        let mut var_itzf_db4: f64 = *var_itzf_db4_slot;
        let mut var_itzf_db5: f64 = *var_itzf_db5_slot;
        let mut var_itzf_db6: f64 = *var_itzf_db6_slot;
        let mut var_itzf_db7: f64 = *var_itzf_db7_slot;
        let mut var_itzf_dn0: f64 = *var_itzf_dn0_slot;
        let mut var_itzf_dn1: f64 = *var_itzf_dn1_slot;
        let mut var_itzf_dn2: f64 = *var_itzf_dn2_slot;
        let mut var_itzf_dn3: f64 = *var_itzf_dn3_slot;
        let mut var_itzf_dn4: f64 = *var_itzf_dn4_slot;
        let mut var_itzf_dn5: f64 = *var_itzf_dn5_slot;
        let mut var_itzf_dn6: f64 = *var_itzf_dn6_slot;
        let mut var_itzf_dn7: f64 = *var_itzf_dn7_slot;
        let mut var_itzf_dn8: f64 = *var_itzf_dn8_slot;
        let mut var_itzf_dn9: f64 = *var_itzf_dn9_slot;
        let mut var_itzf_rdb0: f64 = *var_itzf_rdb0_slot;
        let mut var_itzf_rdb1: f64 = *var_itzf_rdb1_slot;
        let mut var_itzf_rdb2: f64 = *var_itzf_rdb2_slot;
        let mut var_itzf_rdb3: f64 = *var_itzf_rdb3_slot;
        let mut var_itzf_rdb4: f64 = *var_itzf_rdb4_slot;
        let mut var_itzf_rdb5: f64 = *var_itzf_rdb5_slot;
        let mut var_itzf_rdb6: f64 = *var_itzf_rdb6_slot;
        let mut var_itzf_rdb7: f64 = *var_itzf_rdb7_slot;
        let mut var_itzf_rdn0: f64 = *var_itzf_rdn0_slot;
        let mut var_itzf_rdn1: f64 = *var_itzf_rdn1_slot;
        let mut var_itzf_rdn2: f64 = *var_itzf_rdn2_slot;
        let mut var_itzf_rdn3: f64 = *var_itzf_rdn3_slot;
        let mut var_itzf_rdn4: f64 = *var_itzf_rdn4_slot;
        let mut var_itzf_rdn5: f64 = *var_itzf_rdn5_slot;
        let mut var_itzf_rdn6: f64 = *var_itzf_rdn6_slot;
        let mut var_itzf_rdn7: f64 = *var_itzf_rdn7_slot;
        let mut var_itzf_rdn8: f64 = *var_itzf_rdn8_slot;
        let mut var_itzf_rdn9: f64 = *var_itzf_rdn9_slot;
        let mut var_itzf_rv: f64 = *var_itzf_rv_slot;
        let mut var_kq2: f64 = *var_kq2_slot;
        let mut var_kq2_db0: f64 = *var_kq2_db0_slot;
        let mut var_kq2_db1: f64 = *var_kq2_db1_slot;
        let mut var_kq2_db2: f64 = *var_kq2_db2_slot;
        let mut var_kq2_db3: f64 = *var_kq2_db3_slot;
        let mut var_kq2_db4: f64 = *var_kq2_db4_slot;
        let mut var_kq2_db5: f64 = *var_kq2_db5_slot;
        let mut var_kq2_db6: f64 = *var_kq2_db6_slot;
        let mut var_kq2_db7: f64 = *var_kq2_db7_slot;
        let mut var_kq2_dn0: f64 = *var_kq2_dn0_slot;
        let mut var_kq2_dn1: f64 = *var_kq2_dn1_slot;
        let mut var_kq2_dn2: f64 = *var_kq2_dn2_slot;
        let mut var_kq2_dn3: f64 = *var_kq2_dn3_slot;
        let mut var_kq2_dn4: f64 = *var_kq2_dn4_slot;
        let mut var_kq2_dn5: f64 = *var_kq2_dn5_slot;
        let mut var_kq2_dn6: f64 = *var_kq2_dn6_slot;
        let mut var_kq2_dn7: f64 = *var_kq2_dn7_slot;
        let mut var_kq2_dn8: f64 = *var_kq2_dn8_slot;
        let mut var_kq2_dn9: f64 = *var_kq2_dn9_slot;
        let mut var_kq2_rdb0: f64 = *var_kq2_rdb0_slot;
        let mut var_kq2_rdb1: f64 = *var_kq2_rdb1_slot;
        let mut var_kq2_rdb2: f64 = *var_kq2_rdb2_slot;
        let mut var_kq2_rdb3: f64 = *var_kq2_rdb3_slot;
        let mut var_kq2_rdb4: f64 = *var_kq2_rdb4_slot;
        let mut var_kq2_rdb5: f64 = *var_kq2_rdb5_slot;
        let mut var_kq2_rdb6: f64 = *var_kq2_rdb6_slot;
        let mut var_kq2_rdb7: f64 = *var_kq2_rdb7_slot;
        let mut var_kq2_rdn0: f64 = *var_kq2_rdn0_slot;
        let mut var_kq2_rdn1: f64 = *var_kq2_rdn1_slot;
        let mut var_kq2_rdn2: f64 = *var_kq2_rdn2_slot;
        let mut var_kq2_rdn3: f64 = *var_kq2_rdn3_slot;
        let mut var_kq2_rdn4: f64 = *var_kq2_rdn4_slot;
        let mut var_kq2_rdn5: f64 = *var_kq2_rdn5_slot;
        let mut var_kq2_rdn6: f64 = *var_kq2_rdn6_slot;
        let mut var_kq2_rdn7: f64 = *var_kq2_rdn7_slot;
        let mut var_kq2_rdn8: f64 = *var_kq2_rdn8_slot;
        let mut var_kq2_rdn9: f64 = *var_kq2_rdn9_slot;
        let mut var_kq2_rv: f64 = *var_kq2_rv_slot;
        let mut var_oikf: f64 = *var_oikf_slot;
        let mut var_oikf_db0: f64 = *var_oikf_db0_slot;
        let mut var_oikf_db1: f64 = *var_oikf_db1_slot;
        let mut var_oikf_db2: f64 = *var_oikf_db2_slot;
        let mut var_oikf_db3: f64 = *var_oikf_db3_slot;
        let mut var_oikf_db4: f64 = *var_oikf_db4_slot;
        let mut var_oikf_db5: f64 = *var_oikf_db5_slot;
        let mut var_oikf_db6: f64 = *var_oikf_db6_slot;
        let mut var_oikf_db7: f64 = *var_oikf_db7_slot;
        let mut var_oikf_dn0: f64 = *var_oikf_dn0_slot;
        let mut var_oikf_dn1: f64 = *var_oikf_dn1_slot;
        let mut var_oikf_dn2: f64 = *var_oikf_dn2_slot;
        let mut var_oikf_dn3: f64 = *var_oikf_dn3_slot;
        let mut var_oikf_dn4: f64 = *var_oikf_dn4_slot;
        let mut var_oikf_dn5: f64 = *var_oikf_dn5_slot;
        let mut var_oikf_dn6: f64 = *var_oikf_dn6_slot;
        let mut var_oikf_dn7: f64 = *var_oikf_dn7_slot;
        let mut var_oikf_dn8: f64 = *var_oikf_dn8_slot;
        let mut var_oikf_dn9: f64 = *var_oikf_dn9_slot;
        let mut var_oikf_rdb0: f64 = *var_oikf_rdb0_slot;
        let mut var_oikf_rdb1: f64 = *var_oikf_rdb1_slot;
        let mut var_oikf_rdb2: f64 = *var_oikf_rdb2_slot;
        let mut var_oikf_rdb3: f64 = *var_oikf_rdb3_slot;
        let mut var_oikf_rdb4: f64 = *var_oikf_rdb4_slot;
        let mut var_oikf_rdb5: f64 = *var_oikf_rdb5_slot;
        let mut var_oikf_rdb6: f64 = *var_oikf_rdb6_slot;
        let mut var_oikf_rdb7: f64 = *var_oikf_rdb7_slot;
        let mut var_oikf_rdn0: f64 = *var_oikf_rdn0_slot;
        let mut var_oikf_rdn1: f64 = *var_oikf_rdn1_slot;
        let mut var_oikf_rdn2: f64 = *var_oikf_rdn2_slot;
        let mut var_oikf_rdn3: f64 = *var_oikf_rdn3_slot;
        let mut var_oikf_rdn4: f64 = *var_oikf_rdn4_slot;
        let mut var_oikf_rdn5: f64 = *var_oikf_rdn5_slot;
        let mut var_oikf_rdn6: f64 = *var_oikf_rdn6_slot;
        let mut var_oikf_rdn7: f64 = *var_oikf_rdn7_slot;
        let mut var_oikf_rdn8: f64 = *var_oikf_rdn8_slot;
        let mut var_oikf_rdn9: f64 = *var_oikf_rdn9_slot;
        let mut var_oikf_rv: f64 = *var_oikf_rv_slot;
        let mut var_qdc: f64 = *var_qdc_slot;
        let mut var_qdc_db0: f64 = *var_qdc_db0_slot;
        let mut var_qdc_db1: f64 = *var_qdc_db1_slot;
        let mut var_qdc_db2: f64 = *var_qdc_db2_slot;
        let mut var_qdc_db3: f64 = *var_qdc_db3_slot;
        let mut var_qdc_db4: f64 = *var_qdc_db4_slot;
        let mut var_qdc_db5: f64 = *var_qdc_db5_slot;
        let mut var_qdc_db6: f64 = *var_qdc_db6_slot;
        let mut var_qdc_db7: f64 = *var_qdc_db7_slot;
        let mut var_qdc_dn0: f64 = *var_qdc_dn0_slot;
        let mut var_qdc_dn1: f64 = *var_qdc_dn1_slot;
        let mut var_qdc_dn2: f64 = *var_qdc_dn2_slot;
        let mut var_qdc_dn3: f64 = *var_qdc_dn3_slot;
        let mut var_qdc_dn4: f64 = *var_qdc_dn4_slot;
        let mut var_qdc_dn5: f64 = *var_qdc_dn5_slot;
        let mut var_qdc_dn6: f64 = *var_qdc_dn6_slot;
        let mut var_qdc_dn7: f64 = *var_qdc_dn7_slot;
        let mut var_qdc_dn8: f64 = *var_qdc_dn8_slot;
        let mut var_qdc_dn9: f64 = *var_qdc_dn9_slot;
        let mut var_qdc_rdb0: f64 = *var_qdc_rdb0_slot;
        let mut var_qdc_rdb1: f64 = *var_qdc_rdb1_slot;
        let mut var_qdc_rdb2: f64 = *var_qdc_rdb2_slot;
        let mut var_qdc_rdb3: f64 = *var_qdc_rdb3_slot;
        let mut var_qdc_rdb4: f64 = *var_qdc_rdb4_slot;
        let mut var_qdc_rdb5: f64 = *var_qdc_rdb5_slot;
        let mut var_qdc_rdb6: f64 = *var_qdc_rdb6_slot;
        let mut var_qdc_rdb7: f64 = *var_qdc_rdb7_slot;
        let mut var_qdc_rdn0: f64 = *var_qdc_rdn0_slot;
        let mut var_qdc_rdn1: f64 = *var_qdc_rdn1_slot;
        let mut var_qdc_rdn2: f64 = *var_qdc_rdn2_slot;
        let mut var_qdc_rdn3: f64 = *var_qdc_rdn3_slot;
        let mut var_qdc_rdn4: f64 = *var_qdc_rdn4_slot;
        let mut var_qdc_rdn5: f64 = *var_qdc_rdn5_slot;
        let mut var_qdc_rdn6: f64 = *var_qdc_rdn6_slot;
        let mut var_qdc_rdn7: f64 = *var_qdc_rdn7_slot;
        let mut var_qdc_rdn8: f64 = *var_qdc_rdn8_slot;
        let mut var_qdc_rdn9: f64 = *var_qdc_rdn9_slot;
        let mut var_qdc_rv: f64 = *var_qdc_rv_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_db0: f64 = *var_qde_db0_slot;
        let mut var_qde_db1: f64 = *var_qde_db1_slot;
        let mut var_qde_db2: f64 = *var_qde_db2_slot;
        let mut var_qde_db3: f64 = *var_qde_db3_slot;
        let mut var_qde_db4: f64 = *var_qde_db4_slot;
        let mut var_qde_db5: f64 = *var_qde_db5_slot;
        let mut var_qde_db6: f64 = *var_qde_db6_slot;
        let mut var_qde_db7: f64 = *var_qde_db7_slot;
        let mut var_qde_dn0: f64 = *var_qde_dn0_slot;
        let mut var_qde_dn1: f64 = *var_qde_dn1_slot;
        let mut var_qde_dn2: f64 = *var_qde_dn2_slot;
        let mut var_qde_dn3: f64 = *var_qde_dn3_slot;
        let mut var_qde_dn4: f64 = *var_qde_dn4_slot;
        let mut var_qde_dn5: f64 = *var_qde_dn5_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qde_dn7: f64 = *var_qde_dn7_slot;
        let mut var_qde_dn8: f64 = *var_qde_dn8_slot;
        let mut var_qde_dn9: f64 = *var_qde_dn9_slot;
        let mut var_qde_rdb0: f64 = *var_qde_rdb0_slot;
        let mut var_qde_rdb1: f64 = *var_qde_rdb1_slot;
        let mut var_qde_rdb2: f64 = *var_qde_rdb2_slot;
        let mut var_qde_rdb3: f64 = *var_qde_rdb3_slot;
        let mut var_qde_rdb4: f64 = *var_qde_rdb4_slot;
        let mut var_qde_rdb5: f64 = *var_qde_rdb5_slot;
        let mut var_qde_rdb6: f64 = *var_qde_rdb6_slot;
        let mut var_qde_rdb7: f64 = *var_qde_rdb7_slot;
        let mut var_qde_rdn0: f64 = *var_qde_rdn0_slot;
        let mut var_qde_rdn1: f64 = *var_qde_rdn1_slot;
        let mut var_qde_rdn2: f64 = *var_qde_rdn2_slot;
        let mut var_qde_rdn3: f64 = *var_qde_rdn3_slot;
        let mut var_qde_rdn4: f64 = *var_qde_rdn4_slot;
        let mut var_qde_rdn5: f64 = *var_qde_rdn5_slot;
        let mut var_qde_rdn6: f64 = *var_qde_rdn6_slot;
        let mut var_qde_rdn7: f64 = *var_qde_rdn7_slot;
        let mut var_qde_rdn8: f64 = *var_qde_rdn8_slot;
        let mut var_qde_rdn9: f64 = *var_qde_rdn9_slot;
        let mut var_qde_rv: f64 = *var_qde_rv_slot;
        let mut var_tff: f64 = *var_tff_slot;
        let mut var_tff_db0: f64 = *var_tff_db0_slot;
        let mut var_tff_db1: f64 = *var_tff_db1_slot;
        let mut var_tff_db2: f64 = *var_tff_db2_slot;
        let mut var_tff_db3: f64 = *var_tff_db3_slot;
        let mut var_tff_db4: f64 = *var_tff_db4_slot;
        let mut var_tff_db5: f64 = *var_tff_db5_slot;
        let mut var_tff_db6: f64 = *var_tff_db6_slot;
        let mut var_tff_db7: f64 = *var_tff_db7_slot;
        let mut var_tff_dn0: f64 = *var_tff_dn0_slot;
        let mut var_tff_dn1: f64 = *var_tff_dn1_slot;
        let mut var_tff_dn2: f64 = *var_tff_dn2_slot;
        let mut var_tff_dn3: f64 = *var_tff_dn3_slot;
        let mut var_tff_dn4: f64 = *var_tff_dn4_slot;
        let mut var_tff_dn5: f64 = *var_tff_dn5_slot;
        let mut var_tff_dn6: f64 = *var_tff_dn6_slot;
        let mut var_tff_dn7: f64 = *var_tff_dn7_slot;
        let mut var_tff_dn8: f64 = *var_tff_dn8_slot;
        let mut var_tff_dn9: f64 = *var_tff_dn9_slot;
        let mut var_tff_rdb0: f64 = *var_tff_rdb0_slot;
        let mut var_tff_rdb1: f64 = *var_tff_rdb1_slot;
        let mut var_tff_rdb2: f64 = *var_tff_rdb2_slot;
        let mut var_tff_rdb3: f64 = *var_tff_rdb3_slot;
        let mut var_tff_rdb4: f64 = *var_tff_rdb4_slot;
        let mut var_tff_rdb5: f64 = *var_tff_rdb5_slot;
        let mut var_tff_rdb6: f64 = *var_tff_rdb6_slot;
        let mut var_tff_rdb7: f64 = *var_tff_rdb7_slot;
        let mut var_tff_rdn0: f64 = *var_tff_rdn0_slot;
        let mut var_tff_rdn1: f64 = *var_tff_rdn1_slot;
        let mut var_tff_rdn2: f64 = *var_tff_rdn2_slot;
        let mut var_tff_rdn3: f64 = *var_tff_rdn3_slot;
        let mut var_tff_rdn4: f64 = *var_tff_rdn4_slot;
        let mut var_tff_rdn5: f64 = *var_tff_rdn5_slot;
        let mut var_tff_rdn6: f64 = *var_tff_rdn6_slot;
        let mut var_tff_rdn7: f64 = *var_tff_rdn7_slot;
        let mut var_tff_rdn8: f64 = *var_tff_rdn8_slot;
        let mut var_tff_rdn9: f64 = *var_tff_rdn9_slot;
        let mut var_tff_rv: f64 = *var_tff_rv_slot;
        let mut var_vtff: f64 = *var_vtff_slot;
        let mut var_vtff1: f64 = *var_vtff1_slot;
        let mut var_vtff1_db0: f64 = *var_vtff1_db0_slot;
        let mut var_vtff1_db1: f64 = *var_vtff1_db1_slot;
        let mut var_vtff1_db2: f64 = *var_vtff1_db2_slot;
        let mut var_vtff1_db3: f64 = *var_vtff1_db3_slot;
        let mut var_vtff1_db4: f64 = *var_vtff1_db4_slot;
        let mut var_vtff1_db5: f64 = *var_vtff1_db5_slot;
        let mut var_vtff1_db6: f64 = *var_vtff1_db6_slot;
        let mut var_vtff1_db7: f64 = *var_vtff1_db7_slot;
        let mut var_vtff1_dn0: f64 = *var_vtff1_dn0_slot;
        let mut var_vtff1_dn1: f64 = *var_vtff1_dn1_slot;
        let mut var_vtff1_dn2: f64 = *var_vtff1_dn2_slot;
        let mut var_vtff1_dn3: f64 = *var_vtff1_dn3_slot;
        let mut var_vtff1_dn4: f64 = *var_vtff1_dn4_slot;
        let mut var_vtff1_dn5: f64 = *var_vtff1_dn5_slot;
        let mut var_vtff1_dn6: f64 = *var_vtff1_dn6_slot;
        let mut var_vtff1_dn7: f64 = *var_vtff1_dn7_slot;
        let mut var_vtff1_dn8: f64 = *var_vtff1_dn8_slot;
        let mut var_vtff1_dn9: f64 = *var_vtff1_dn9_slot;
        let mut var_vtff1_rdb0: f64 = *var_vtff1_rdb0_slot;
        let mut var_vtff1_rdb1: f64 = *var_vtff1_rdb1_slot;
        let mut var_vtff1_rdb2: f64 = *var_vtff1_rdb2_slot;
        let mut var_vtff1_rdb3: f64 = *var_vtff1_rdb3_slot;
        let mut var_vtff1_rdb4: f64 = *var_vtff1_rdb4_slot;
        let mut var_vtff1_rdb5: f64 = *var_vtff1_rdb5_slot;
        let mut var_vtff1_rdb6: f64 = *var_vtff1_rdb6_slot;
        let mut var_vtff1_rdb7: f64 = *var_vtff1_rdb7_slot;
        let mut var_vtff1_rdn0: f64 = *var_vtff1_rdn0_slot;
        let mut var_vtff1_rdn1: f64 = *var_vtff1_rdn1_slot;
        let mut var_vtff1_rdn2: f64 = *var_vtff1_rdn2_slot;
        let mut var_vtff1_rdn3: f64 = *var_vtff1_rdn3_slot;
        let mut var_vtff1_rdn4: f64 = *var_vtff1_rdn4_slot;
        let mut var_vtff1_rdn5: f64 = *var_vtff1_rdn5_slot;
        let mut var_vtff1_rdn6: f64 = *var_vtff1_rdn6_slot;
        let mut var_vtff1_rdn7: f64 = *var_vtff1_rdn7_slot;
        let mut var_vtff1_rdn8: f64 = *var_vtff1_rdn8_slot;
        let mut var_vtff1_rdn9: f64 = *var_vtff1_rdn9_slot;
        let mut var_vtff1_rv: f64 = *var_vtff1_rv_slot;
        let mut var_vtff_db0: f64 = *var_vtff_db0_slot;
        let mut var_vtff_db1: f64 = *var_vtff_db1_slot;
        let mut var_vtff_db2: f64 = *var_vtff_db2_slot;
        let mut var_vtff_db3: f64 = *var_vtff_db3_slot;
        let mut var_vtff_db4: f64 = *var_vtff_db4_slot;
        let mut var_vtff_db5: f64 = *var_vtff_db5_slot;
        let mut var_vtff_db6: f64 = *var_vtff_db6_slot;
        let mut var_vtff_db7: f64 = *var_vtff_db7_slot;
        let mut var_vtff_dn0: f64 = *var_vtff_dn0_slot;
        let mut var_vtff_dn1: f64 = *var_vtff_dn1_slot;
        let mut var_vtff_dn2: f64 = *var_vtff_dn2_slot;
        let mut var_vtff_dn3: f64 = *var_vtff_dn3_slot;
        let mut var_vtff_dn4: f64 = *var_vtff_dn4_slot;
        let mut var_vtff_dn5: f64 = *var_vtff_dn5_slot;
        let mut var_vtff_dn6: f64 = *var_vtff_dn6_slot;
        let mut var_vtff_dn7: f64 = *var_vtff_dn7_slot;
        let mut var_vtff_dn8: f64 = *var_vtff_dn8_slot;
        let mut var_vtff_dn9: f64 = *var_vtff_dn9_slot;
        let mut var_vtff_rdb0: f64 = *var_vtff_rdb0_slot;
        let mut var_vtff_rdb1: f64 = *var_vtff_rdb1_slot;
        let mut var_vtff_rdb2: f64 = *var_vtff_rdb2_slot;
        let mut var_vtff_rdb3: f64 = *var_vtff_rdb3_slot;
        let mut var_vtff_rdb4: f64 = *var_vtff_rdb4_slot;
        let mut var_vtff_rdb5: f64 = *var_vtff_rdb5_slot;
        let mut var_vtff_rdb6: f64 = *var_vtff_rdb6_slot;
        let mut var_vtff_rdb7: f64 = *var_vtff_rdb7_slot;
        let mut var_vtff_rdn0: f64 = *var_vtff_rdn0_slot;
        let mut var_vtff_rdn1: f64 = *var_vtff_rdn1_slot;
        let mut var_vtff_rdn2: f64 = *var_vtff_rdn2_slot;
        let mut var_vtff_rdn3: f64 = *var_vtff_rdn3_slot;
        let mut var_vtff_rdn4: f64 = *var_vtff_rdn4_slot;
        let mut var_vtff_rdn5: f64 = *var_vtff_rdn5_slot;
        let mut var_vtff_rdn6: f64 = *var_vtff_rdn6_slot;
        let mut var_vtff_rdn7: f64 = *var_vtff_rdn7_slot;
        let mut var_vtff_rdn8: f64 = *var_vtff_rdn8_slot;
        let mut var_vtff_rdn9: f64 = *var_vtff_rdn9_slot;
        let mut var_vtff_rv: f64 = *var_vtff_rv_slot;

        let assign1320_e1679: f64 = (var_vbici * p.p81);
        let assign1320_e1680: f64 = (1.0 + assign1320_e1679);
        let assign1320_e1681: f64 = (var_oikf * assign1320_e1680);
        var_oikf = assign1320_e1681;
        var_oikf_dn0 = ((var_oikf_dn0 * assign1320_e1680) + (var_oikf * (var_vbici_dn0 * p.p81)));
        var_oikf_dn1 = ((var_oikf_dn1 * assign1320_e1680) + (var_oikf * (var_vbici_dn1 * p.p81)));
        var_oikf_dn2 = ((var_oikf_dn2 * assign1320_e1680) + (var_oikf * (var_vbici_dn2 * p.p81)));
        var_oikf_dn3 = ((var_oikf_dn3 * assign1320_e1680) + (var_oikf * (var_vbici_dn3 * p.p81)));
        var_oikf_dn4 = ((var_oikf_dn4 * assign1320_e1680) + (var_oikf * (var_vbici_dn4 * p.p81)));
        var_oikf_dn5 = ((var_oikf_dn5 * assign1320_e1680) + (var_oikf * (var_vbici_dn5 * p.p81)));
        var_oikf_dn6 = ((var_oikf_dn6 * assign1320_e1680) + (var_oikf * (var_vbici_dn6 * p.p81)));
        var_oikf_dn7 = ((var_oikf_dn7 * assign1320_e1680) + (var_oikf * (var_vbici_dn7 * p.p81)));
        var_oikf_dn8 = ((var_oikf_dn8 * assign1320_e1680) + (var_oikf * (var_vbici_dn8 * p.p81)));
        var_oikf_dn9 = ((var_oikf_dn9 * assign1320_e1680) + (var_oikf * (var_vbici_dn9 * p.p81)));
        var_oikf_db0 = ((var_oikf_db0 * assign1320_e1680) + (var_oikf * (var_vbici_db0 * p.p81)));
        var_oikf_db1 = ((var_oikf_db1 * assign1320_e1680) + (var_oikf * (var_vbici_db1 * p.p81)));
        var_oikf_db2 = ((var_oikf_db2 * assign1320_e1680) + (var_oikf * (var_vbici_db2 * p.p81)));
        var_oikf_db3 = ((var_oikf_db3 * assign1320_e1680) + (var_oikf * (var_vbici_db3 * p.p81)));
        var_oikf_db4 = ((var_oikf_db4 * assign1320_e1680) + (var_oikf * (var_vbici_db4 * p.p81)));
        var_oikf_db5 = ((var_oikf_db5 * assign1320_e1680) + (var_oikf * (var_vbici_db5 * p.p81)));
        var_oikf_db6 = ((var_oikf_db6 * assign1320_e1680) + (var_oikf * (var_vbici_db6 * p.p81)));
        var_oikf_db7 = ((var_oikf_db7 * assign1320_e1680) + (var_oikf * (var_vbici_db7 * p.p81)));
        var_oikf_rv = 0.0;
        var_oikf_rdn0 = 0.0;
        var_oikf_rdn1 = 0.0;
        var_oikf_rdn2 = 0.0;
        var_oikf_rdn3 = 0.0;
        var_oikf_rdn4 = 0.0;
        var_oikf_rdn5 = 0.0;
        var_oikf_rdn6 = 0.0;
        var_oikf_rdn7 = 0.0;
        var_oikf_rdn8 = 0.0;
        var_oikf_rdn9 = 0.0;
        var_oikf_rdb0 = 0.0;
        var_oikf_rdb1 = 0.0;
        var_oikf_rdb2 = 0.0;
        var_oikf_rdb3 = 0.0;
        var_oikf_rdb4 = 0.0;
        var_oikf_rdb5 = 0.0;
        var_oikf_rdb6 = 0.0;
        var_oikf_rdb7 = 0.0;

        let assign1330_e1684: f64 = (var_ifwd * var_oikf);
        let assign1330_e1687: f64 = (var_ibwd * var_oikr);
        let assign1330_e1688: f64 = (assign1330_e1684 + assign1330_e1687);
        var_kq2 = assign1330_e1688;
        var_kq2_dn0 = (((var_ifwd_dn0 * var_oikf) + (var_ifwd * var_oikf_dn0)) + ((var_ibwd_dn0 * var_oikr) + (var_ibwd * var_oikr_dn0)));
        var_kq2_dn1 = (((var_ifwd_dn1 * var_oikf) + (var_ifwd * var_oikf_dn1)) + ((var_ibwd_dn1 * var_oikr) + (var_ibwd * var_oikr_dn1)));
        var_kq2_dn2 = (((var_ifwd_dn2 * var_oikf) + (var_ifwd * var_oikf_dn2)) + ((var_ibwd_dn2 * var_oikr) + (var_ibwd * var_oikr_dn2)));
        var_kq2_dn3 = (((var_ifwd_dn3 * var_oikf) + (var_ifwd * var_oikf_dn3)) + ((var_ibwd_dn3 * var_oikr) + (var_ibwd * var_oikr_dn3)));
        var_kq2_dn4 = (((var_ifwd_dn4 * var_oikf) + (var_ifwd * var_oikf_dn4)) + ((var_ibwd_dn4 * var_oikr) + (var_ibwd * var_oikr_dn4)));
        var_kq2_dn5 = (((var_ifwd_dn5 * var_oikf) + (var_ifwd * var_oikf_dn5)) + ((var_ibwd_dn5 * var_oikr) + (var_ibwd * var_oikr_dn5)));
        var_kq2_dn6 = (((var_ifwd_dn6 * var_oikf) + (var_ifwd * var_oikf_dn6)) + ((var_ibwd_dn6 * var_oikr) + (var_ibwd * var_oikr_dn6)));
        var_kq2_dn7 = (((var_ifwd_dn7 * var_oikf) + (var_ifwd * var_oikf_dn7)) + ((var_ibwd_dn7 * var_oikr) + (var_ibwd * var_oikr_dn7)));
        var_kq2_dn8 = (((var_ifwd_dn8 * var_oikf) + (var_ifwd * var_oikf_dn8)) + ((var_ibwd_dn8 * var_oikr) + (var_ibwd * var_oikr_dn8)));
        var_kq2_dn9 = (((var_ifwd_dn9 * var_oikf) + (var_ifwd * var_oikf_dn9)) + ((var_ibwd_dn9 * var_oikr) + (var_ibwd * var_oikr_dn9)));
        var_kq2_db0 = (((var_ifwd_db0 * var_oikf) + (var_ifwd * var_oikf_db0)) + ((var_ibwd_db0 * var_oikr) + (var_ibwd * var_oikr_db0)));
        var_kq2_db1 = (((var_ifwd_db1 * var_oikf) + (var_ifwd * var_oikf_db1)) + ((var_ibwd_db1 * var_oikr) + (var_ibwd * var_oikr_db1)));
        var_kq2_db2 = (((var_ifwd_db2 * var_oikf) + (var_ifwd * var_oikf_db2)) + ((var_ibwd_db2 * var_oikr) + (var_ibwd * var_oikr_db2)));
        var_kq2_db3 = (((var_ifwd_db3 * var_oikf) + (var_ifwd * var_oikf_db3)) + ((var_ibwd_db3 * var_oikr) + (var_ibwd * var_oikr_db3)));
        var_kq2_db4 = (((var_ifwd_db4 * var_oikf) + (var_ifwd * var_oikf_db4)) + ((var_ibwd_db4 * var_oikr) + (var_ibwd * var_oikr_db4)));
        var_kq2_db5 = (((var_ifwd_db5 * var_oikf) + (var_ifwd * var_oikf_db5)) + ((var_ibwd_db5 * var_oikr) + (var_ibwd * var_oikr_db5)));
        var_kq2_db6 = (((var_ifwd_db6 * var_oikf) + (var_ifwd * var_oikf_db6)) + ((var_ibwd_db6 * var_oikr) + (var_ibwd * var_oikr_db6)));
        var_kq2_db7 = (((var_ifwd_db7 * var_oikf) + (var_ifwd * var_oikf_db7)) + ((var_ibwd_db7 * var_oikr) + (var_ibwd * var_oikr_db7)));
        var_kq2_rv = 0.0;
        var_kq2_rdn0 = 0.0;
        var_kq2_rdn1 = 0.0;
        var_kq2_rdn2 = 0.0;
        var_kq2_rdn3 = 0.0;
        var_kq2_rdn4 = 0.0;
        var_kq2_rdn5 = 0.0;
        var_kq2_rdn6 = 0.0;
        var_kq2_rdn7 = 0.0;
        var_kq2_rdn8 = 0.0;
        var_kq2_rdn9 = 0.0;
        var_kq2_rdb0 = 0.0;
        var_kq2_rdb1 = 0.0;
        var_kq2_rdb2 = 0.0;
        var_kq2_rdb3 = 0.0;
        var_kq2_rdb4 = 0.0;
        var_kq2_rdb5 = 0.0;
        var_kq2_rdb6 = 0.0;
        var_kq2_rdb7 = 0.0;

        let assign1340_e1692: f64 = (var_vbiei * var_ovar);
        let assign1340_e1693: f64 = (1.0 - assign1340_e1692);
        let assign1340_e1696: f64 = (var_vbici * var_ovaf);
        let assign1340_e1697: f64 = (assign1340_e1693 - assign1340_e1696);
        var_ikq1 = assign1340_e1697;
        var_ikq1_dn0 = ((-((var_vbiei_dn0 * var_ovar) + (var_vbiei * var_ovar_dn0))) - ((var_vbici_dn0 * var_ovaf) + (var_vbici * var_ovaf_dn0)));
        var_ikq1_dn1 = ((-((var_vbiei_dn1 * var_ovar) + (var_vbiei * var_ovar_dn1))) - ((var_vbici_dn1 * var_ovaf) + (var_vbici * var_ovaf_dn1)));
        var_ikq1_dn2 = ((-((var_vbiei_dn2 * var_ovar) + (var_vbiei * var_ovar_dn2))) - ((var_vbici_dn2 * var_ovaf) + (var_vbici * var_ovaf_dn2)));
        var_ikq1_dn3 = ((-((var_vbiei_dn3 * var_ovar) + (var_vbiei * var_ovar_dn3))) - ((var_vbici_dn3 * var_ovaf) + (var_vbici * var_ovaf_dn3)));
        var_ikq1_dn4 = ((-((var_vbiei_dn4 * var_ovar) + (var_vbiei * var_ovar_dn4))) - ((var_vbici_dn4 * var_ovaf) + (var_vbici * var_ovaf_dn4)));
        var_ikq1_dn5 = ((-((var_vbiei_dn5 * var_ovar) + (var_vbiei * var_ovar_dn5))) - ((var_vbici_dn5 * var_ovaf) + (var_vbici * var_ovaf_dn5)));
        var_ikq1_dn6 = ((-((var_vbiei_dn6 * var_ovar) + (var_vbiei * var_ovar_dn6))) - ((var_vbici_dn6 * var_ovaf) + (var_vbici * var_ovaf_dn6)));
        var_ikq1_dn7 = ((-((var_vbiei_dn7 * var_ovar) + (var_vbiei * var_ovar_dn7))) - ((var_vbici_dn7 * var_ovaf) + (var_vbici * var_ovaf_dn7)));
        var_ikq1_dn8 = ((-((var_vbiei_dn8 * var_ovar) + (var_vbiei * var_ovar_dn8))) - ((var_vbici_dn8 * var_ovaf) + (var_vbici * var_ovaf_dn8)));
        var_ikq1_dn9 = ((-((var_vbiei_dn9 * var_ovar) + (var_vbiei * var_ovar_dn9))) - ((var_vbici_dn9 * var_ovaf) + (var_vbici * var_ovaf_dn9)));
        var_ikq1_db0 = ((-((var_vbiei_db0 * var_ovar) + (var_vbiei * var_ovar_db0))) - ((var_vbici_db0 * var_ovaf) + (var_vbici * var_ovaf_db0)));
        var_ikq1_db1 = ((-((var_vbiei_db1 * var_ovar) + (var_vbiei * var_ovar_db1))) - ((var_vbici_db1 * var_ovaf) + (var_vbici * var_ovaf_db1)));
        var_ikq1_db2 = ((-((var_vbiei_db2 * var_ovar) + (var_vbiei * var_ovar_db2))) - ((var_vbici_db2 * var_ovaf) + (var_vbici * var_ovaf_db2)));
        var_ikq1_db3 = ((-((var_vbiei_db3 * var_ovar) + (var_vbiei * var_ovar_db3))) - ((var_vbici_db3 * var_ovaf) + (var_vbici * var_ovaf_db3)));
        var_ikq1_db4 = ((-((var_vbiei_db4 * var_ovar) + (var_vbiei * var_ovar_db4))) - ((var_vbici_db4 * var_ovaf) + (var_vbici * var_ovaf_db4)));
        var_ikq1_db5 = ((-((var_vbiei_db5 * var_ovar) + (var_vbiei * var_ovar_db5))) - ((var_vbici_db5 * var_ovaf) + (var_vbici * var_ovaf_db5)));
        var_ikq1_db6 = ((-((var_vbiei_db6 * var_ovar) + (var_vbiei * var_ovar_db6))) - ((var_vbici_db6 * var_ovaf) + (var_vbici * var_ovaf_db6)));
        var_ikq1_db7 = ((-((var_vbiei_db7 * var_ovar) + (var_vbiei * var_ovar_db7))) - ((var_vbici_db7 * var_ovaf) + (var_vbici * var_ovaf_db7)));
        var_ikq1_rv = 0.0;
        var_ikq1_rdn0 = 0.0;
        var_ikq1_rdn1 = 0.0;
        var_ikq1_rdn2 = 0.0;
        var_ikq1_rdn3 = 0.0;
        var_ikq1_rdn4 = 0.0;
        var_ikq1_rdn5 = 0.0;
        var_ikq1_rdn6 = 0.0;
        var_ikq1_rdn7 = 0.0;
        var_ikq1_rdn8 = 0.0;
        var_ikq1_rdn9 = 0.0;
        var_ikq1_rdb0 = 0.0;
        var_ikq1_rdb1 = 0.0;
        var_ikq1_rdb2 = 0.0;
        var_ikq1_rdb3 = 0.0;
        var_ikq1_rdb4 = 0.0;
        var_ikq1_rdb5 = 0.0;
        var_ikq1_rdb6 = 0.0;
        var_ikq1_rdb7 = 0.0;

        let assign1350_e1702: f64 = (4.0 * var_kq2);
        let assign1350_e1703: f64 = (1.0 + assign1350_e1702);
        let assign1350_e1704: f64 = (assign1350_e1703).abs();
        let assign1350_e1706: f64 = (assign1350_e1704).powf(p.p82);
        let assign1350_e1707: f64 = (1.0 + assign1350_e1706);
        var_dkqb = assign1350_e1707;
        var_dkqb_dn0 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn0) } else { (-(4.0 * var_kq2_dn0)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn0) } else { (-(4.0 * var_kq2_dn0)) } / assign1350_e1704))) };
        var_dkqb_dn1 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn1) } else { (-(4.0 * var_kq2_dn1)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn1) } else { (-(4.0 * var_kq2_dn1)) } / assign1350_e1704))) };
        var_dkqb_dn2 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn2) } else { (-(4.0 * var_kq2_dn2)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn2) } else { (-(4.0 * var_kq2_dn2)) } / assign1350_e1704))) };
        var_dkqb_dn3 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn3) } else { (-(4.0 * var_kq2_dn3)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn3) } else { (-(4.0 * var_kq2_dn3)) } / assign1350_e1704))) };
        var_dkqb_dn4 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn4) } else { (-(4.0 * var_kq2_dn4)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn4) } else { (-(4.0 * var_kq2_dn4)) } / assign1350_e1704))) };
        var_dkqb_dn5 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn5) } else { (-(4.0 * var_kq2_dn5)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn5) } else { (-(4.0 * var_kq2_dn5)) } / assign1350_e1704))) };
        var_dkqb_dn6 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn6) } else { (-(4.0 * var_kq2_dn6)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn6) } else { (-(4.0 * var_kq2_dn6)) } / assign1350_e1704))) };
        var_dkqb_dn7 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn7) } else { (-(4.0 * var_kq2_dn7)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn7) } else { (-(4.0 * var_kq2_dn7)) } / assign1350_e1704))) };
        var_dkqb_dn8 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn8) } else { (-(4.0 * var_kq2_dn8)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn8) } else { (-(4.0 * var_kq2_dn8)) } / assign1350_e1704))) };
        var_dkqb_dn9 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn9) } else { (-(4.0 * var_kq2_dn9)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn9) } else { (-(4.0 * var_kq2_dn9)) } / assign1350_e1704))) };
        var_dkqb_db0 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db0) } else { (-(4.0 * var_kq2_db0)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db0) } else { (-(4.0 * var_kq2_db0)) } / assign1350_e1704))) };
        var_dkqb_db1 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db1) } else { (-(4.0 * var_kq2_db1)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db1) } else { (-(4.0 * var_kq2_db1)) } / assign1350_e1704))) };
        var_dkqb_db2 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db2) } else { (-(4.0 * var_kq2_db2)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db2) } else { (-(4.0 * var_kq2_db2)) } / assign1350_e1704))) };
        var_dkqb_db3 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db3) } else { (-(4.0 * var_kq2_db3)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db3) } else { (-(4.0 * var_kq2_db3)) } / assign1350_e1704))) };
        var_dkqb_db4 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db4) } else { (-(4.0 * var_kq2_db4)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db4) } else { (-(4.0 * var_kq2_db4)) } / assign1350_e1704))) };
        var_dkqb_db5 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db5) } else { (-(4.0 * var_kq2_db5)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db5) } else { (-(4.0 * var_kq2_db5)) } / assign1350_e1704))) };
        var_dkqb_db6 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db6) } else { (-(4.0 * var_kq2_db6)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db6) } else { (-(4.0 * var_kq2_db6)) } / assign1350_e1704))) };
        var_dkqb_db7 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db7) } else { (-(4.0 * var_kq2_db7)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db7) } else { (-(4.0 * var_kq2_db7)) } / assign1350_e1704))) };
        var_dkqb_rv = 0.0;
        var_dkqb_rdn0 = 0.0;
        var_dkqb_rdn1 = 0.0;
        var_dkqb_rdn2 = 0.0;
        var_dkqb_rdn3 = 0.0;
        var_dkqb_rdn4 = 0.0;
        var_dkqb_rdn5 = 0.0;
        var_dkqb_rdn6 = 0.0;
        var_dkqb_rdn7 = 0.0;
        var_dkqb_rdn8 = 0.0;
        var_dkqb_rdn9 = 0.0;
        var_dkqb_rdb0 = 0.0;
        var_dkqb_rdb1 = 0.0;
        var_dkqb_rdb2 = 0.0;
        var_dkqb_rdb3 = 0.0;
        var_dkqb_rdb4 = 0.0;
        var_dkqb_rdb5 = 0.0;
        var_dkqb_rdb6 = 0.0;
        var_dkqb_rdb7 = 0.0;

        let assign1360_e1710: f64 = (2.0 * var_ikq1);
        let assign1360_e1712: f64 = (assign1360_e1710 / var_dkqb);
        var_ikqb = assign1360_e1712;
        var_ikqb_dn0 = ((((2.0 * var_ikq1_dn0) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn0)) / (var_dkqb * var_dkqb));
        var_ikqb_dn1 = ((((2.0 * var_ikq1_dn1) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn1)) / (var_dkqb * var_dkqb));
        var_ikqb_dn2 = ((((2.0 * var_ikq1_dn2) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn2)) / (var_dkqb * var_dkqb));
        var_ikqb_dn3 = ((((2.0 * var_ikq1_dn3) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn3)) / (var_dkqb * var_dkqb));
        var_ikqb_dn4 = ((((2.0 * var_ikq1_dn4) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn4)) / (var_dkqb * var_dkqb));
        var_ikqb_dn5 = ((((2.0 * var_ikq1_dn5) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn5)) / (var_dkqb * var_dkqb));
        var_ikqb_dn6 = ((((2.0 * var_ikq1_dn6) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn6)) / (var_dkqb * var_dkqb));
        var_ikqb_dn7 = ((((2.0 * var_ikq1_dn7) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn7)) / (var_dkqb * var_dkqb));
        var_ikqb_dn8 = ((((2.0 * var_ikq1_dn8) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn8)) / (var_dkqb * var_dkqb));
        var_ikqb_dn9 = ((((2.0 * var_ikq1_dn9) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn9)) / (var_dkqb * var_dkqb));
        var_ikqb_db0 = ((((2.0 * var_ikq1_db0) * var_dkqb) - (assign1360_e1710 * var_dkqb_db0)) / (var_dkqb * var_dkqb));
        var_ikqb_db1 = ((((2.0 * var_ikq1_db1) * var_dkqb) - (assign1360_e1710 * var_dkqb_db1)) / (var_dkqb * var_dkqb));
        var_ikqb_db2 = ((((2.0 * var_ikq1_db2) * var_dkqb) - (assign1360_e1710 * var_dkqb_db2)) / (var_dkqb * var_dkqb));
        var_ikqb_db3 = ((((2.0 * var_ikq1_db3) * var_dkqb) - (assign1360_e1710 * var_dkqb_db3)) / (var_dkqb * var_dkqb));
        var_ikqb_db4 = ((((2.0 * var_ikq1_db4) * var_dkqb) - (assign1360_e1710 * var_dkqb_db4)) / (var_dkqb * var_dkqb));
        var_ikqb_db5 = ((((2.0 * var_ikq1_db5) * var_dkqb) - (assign1360_e1710 * var_dkqb_db5)) / (var_dkqb * var_dkqb));
        var_ikqb_db6 = ((((2.0 * var_ikq1_db6) * var_dkqb) - (assign1360_e1710 * var_dkqb_db6)) / (var_dkqb * var_dkqb));
        var_ikqb_db7 = ((((2.0 * var_ikq1_db7) * var_dkqb) - (assign1360_e1710 * var_dkqb_db7)) / (var_dkqb * var_dkqb));
        var_ikqb_rv = 0.0;
        var_ikqb_rdn0 = 0.0;
        var_ikqb_rdn1 = 0.0;
        var_ikqb_rdn2 = 0.0;
        var_ikqb_rdn3 = 0.0;
        var_ikqb_rdn4 = 0.0;
        var_ikqb_rdn5 = 0.0;
        var_ikqb_rdn6 = 0.0;
        var_ikqb_rdn7 = 0.0;
        var_ikqb_rdn8 = 0.0;
        var_ikqb_rdn9 = 0.0;
        var_ikqb_rdb0 = 0.0;
        var_ikqb_rdb1 = 0.0;
        var_ikqb_rdb2 = 0.0;
        var_ikqb_rdb3 = 0.0;
        var_ikqb_rdb4 = 0.0;
        var_ikqb_rdb5 = 0.0;
        var_ikqb_rdb6 = 0.0;
        var_ikqb_rdb7 = 0.0;

        let assign1370_e1715: f64 = (var_ibwd * var_ikqb);
        var_itr = assign1370_e1715;
        var_itr_dn0 = ((var_ibwd_dn0 * var_ikqb) + (var_ibwd * var_ikqb_dn0));
        var_itr_dn1 = ((var_ibwd_dn1 * var_ikqb) + (var_ibwd * var_ikqb_dn1));
        var_itr_dn2 = ((var_ibwd_dn2 * var_ikqb) + (var_ibwd * var_ikqb_dn2));
        var_itr_dn3 = ((var_ibwd_dn3 * var_ikqb) + (var_ibwd * var_ikqb_dn3));
        var_itr_dn4 = ((var_ibwd_dn4 * var_ikqb) + (var_ibwd * var_ikqb_dn4));
        var_itr_dn5 = ((var_ibwd_dn5 * var_ikqb) + (var_ibwd * var_ikqb_dn5));
        var_itr_dn6 = ((var_ibwd_dn6 * var_ikqb) + (var_ibwd * var_ikqb_dn6));
        var_itr_dn7 = ((var_ibwd_dn7 * var_ikqb) + (var_ibwd * var_ikqb_dn7));
        var_itr_dn8 = ((var_ibwd_dn8 * var_ikqb) + (var_ibwd * var_ikqb_dn8));
        var_itr_dn9 = ((var_ibwd_dn9 * var_ikqb) + (var_ibwd * var_ikqb_dn9));
        var_itr_db0 = ((var_ibwd_db0 * var_ikqb) + (var_ibwd * var_ikqb_db0));
        var_itr_db1 = ((var_ibwd_db1 * var_ikqb) + (var_ibwd * var_ikqb_db1));
        var_itr_db2 = ((var_ibwd_db2 * var_ikqb) + (var_ibwd * var_ikqb_db2));
        var_itr_db3 = ((var_ibwd_db3 * var_ikqb) + (var_ibwd * var_ikqb_db3));
        var_itr_db4 = ((var_ibwd_db4 * var_ikqb) + (var_ibwd * var_ikqb_db4));
        var_itr_db5 = ((var_ibwd_db5 * var_ikqb) + (var_ibwd * var_ikqb_db5));
        var_itr_db6 = ((var_ibwd_db6 * var_ikqb) + (var_ibwd * var_ikqb_db6));
        var_itr_db7 = ((var_ibwd_db7 * var_ikqb) + (var_ibwd * var_ikqb_db7));
        var_itr_rv = 0.0;
        var_itr_rdn0 = 0.0;
        var_itr_rdn1 = 0.0;
        var_itr_rdn2 = 0.0;
        var_itr_rdn3 = 0.0;
        var_itr_rdn4 = 0.0;
        var_itr_rdn5 = 0.0;
        var_itr_rdn6 = 0.0;
        var_itr_rdn7 = 0.0;
        var_itr_rdn8 = 0.0;
        var_itr_rdn9 = 0.0;
        var_itr_rdb0 = 0.0;
        var_itr_rdb1 = 0.0;
        var_itr_rdb2 = 0.0;
        var_itr_rdb3 = 0.0;
        var_itr_rdb4 = 0.0;
        var_itr_rdb5 = 0.0;
        var_itr_rdb6 = 0.0;
        var_itr_rdb7 = 0.0;

        let assign1380_e1718: f64 = (var_ifwd * var_ikqb);
        var_itzf = assign1380_e1718;
        var_itzf_dn0 = ((var_ifwd_dn0 * var_ikqb) + (var_ifwd * var_ikqb_dn0));
        var_itzf_dn1 = ((var_ifwd_dn1 * var_ikqb) + (var_ifwd * var_ikqb_dn1));
        var_itzf_dn2 = ((var_ifwd_dn2 * var_ikqb) + (var_ifwd * var_ikqb_dn2));
        var_itzf_dn3 = ((var_ifwd_dn3 * var_ikqb) + (var_ifwd * var_ikqb_dn3));
        var_itzf_dn4 = ((var_ifwd_dn4 * var_ikqb) + (var_ifwd * var_ikqb_dn4));
        var_itzf_dn5 = ((var_ifwd_dn5 * var_ikqb) + (var_ifwd * var_ikqb_dn5));
        var_itzf_dn6 = ((var_ifwd_dn6 * var_ikqb) + (var_ifwd * var_ikqb_dn6));
        var_itzf_dn7 = ((var_ifwd_dn7 * var_ikqb) + (var_ifwd * var_ikqb_dn7));
        var_itzf_dn8 = ((var_ifwd_dn8 * var_ikqb) + (var_ifwd * var_ikqb_dn8));
        var_itzf_dn9 = ((var_ifwd_dn9 * var_ikqb) + (var_ifwd * var_ikqb_dn9));
        var_itzf_db0 = ((var_ifwd_db0 * var_ikqb) + (var_ifwd * var_ikqb_db0));
        var_itzf_db1 = ((var_ifwd_db1 * var_ikqb) + (var_ifwd * var_ikqb_db1));
        var_itzf_db2 = ((var_ifwd_db2 * var_ikqb) + (var_ifwd * var_ikqb_db2));
        var_itzf_db3 = ((var_ifwd_db3 * var_ikqb) + (var_ifwd * var_ikqb_db3));
        var_itzf_db4 = ((var_ifwd_db4 * var_ikqb) + (var_ifwd * var_ikqb_db4));
        var_itzf_db5 = ((var_ifwd_db5 * var_ikqb) + (var_ifwd * var_ikqb_db5));
        var_itzf_db6 = ((var_ifwd_db6 * var_ikqb) + (var_ifwd * var_ikqb_db6));
        var_itzf_db7 = ((var_ifwd_db7 * var_ikqb) + (var_ifwd * var_ikqb_db7));
        var_itzf_rv = 0.0;
        var_itzf_rdn0 = 0.0;
        var_itzf_rdn1 = 0.0;
        var_itzf_rdn2 = 0.0;
        var_itzf_rdn3 = 0.0;
        var_itzf_rdn4 = 0.0;
        var_itzf_rdn5 = 0.0;
        var_itzf_rdn6 = 0.0;
        var_itzf_rdn7 = 0.0;
        var_itzf_rdn8 = 0.0;
        var_itzf_rdn9 = 0.0;
        var_itzf_rdb0 = 0.0;
        var_itzf_rdb1 = 0.0;
        var_itzf_rdb2 = 0.0;
        var_itzf_rdb3 = 0.0;
        var_itzf_rdb4 = 0.0;
        var_itzf_rdb5 = 0.0;
        var_itzf_rdb6 = 0.0;
        var_itzf_rdb7 = 0.0;

        let assign1450_e1782: f64 = ((nv1 - nv2) / p.p40);
        let assign1450_e1783: f64 = (assign1450_e1782).abs();
        let assign1450_e1785: f64 = (assign1450_e1783).powf(p.p39);
        var_vtff = assign1450_e1785;
        var_vtff_dn0 = 0.0;
        var_vtff_dn1 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign1450_e1783).powf(p.p39 - 1.0) * if assign1450_e1782 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) })) } } else { (assign1450_e1785 * (p.p39 * (if assign1450_e1782 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) } / assign1450_e1783))) };
        var_vtff_dn2 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign1450_e1783).powf(p.p39 - 1.0) * if assign1450_e1782 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) })) } } else { (assign1450_e1785 * (p.p39 * (if assign1450_e1782 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) } / assign1450_e1783))) };
        var_vtff_dn3 = 0.0;
        var_vtff_dn4 = 0.0;
        var_vtff_dn5 = 0.0;
        var_vtff_dn6 = 0.0;
        var_vtff_dn7 = 0.0;
        var_vtff_dn8 = 0.0;
        var_vtff_dn9 = 0.0;
        var_vtff_db0 = 0.0;
        var_vtff_db1 = 0.0;
        var_vtff_db2 = 0.0;
        var_vtff_db3 = 0.0;
        var_vtff_db4 = 0.0;
        var_vtff_db5 = 0.0;
        var_vtff_db6 = 0.0;
        var_vtff_db7 = 0.0;
        var_vtff_rv = 0.0;
        var_vtff_rdn0 = 0.0;
        var_vtff_rdn1 = 0.0;
        var_vtff_rdn2 = 0.0;
        var_vtff_rdn3 = 0.0;
        var_vtff_rdn4 = 0.0;
        var_vtff_rdn5 = 0.0;
        var_vtff_rdn6 = 0.0;
        var_vtff_rdn7 = 0.0;
        var_vtff_rdn8 = 0.0;
        var_vtff_rdn9 = 0.0;
        var_vtff_rdb0 = 0.0;
        var_vtff_rdb1 = 0.0;
        var_vtff_rdb2 = 0.0;
        var_vtff_rdb3 = 0.0;
        var_vtff_rdb4 = 0.0;
        var_vtff_rdb5 = 0.0;
        var_vtff_rdb6 = 0.0;
        var_vtff_rdb7 = 0.0;

        let assign1460_e1788: f64 = (1.0 + var_vtff);
        let assign1460_e1791: f64 = (1.0 / p.p39);
        let assign1460_e1792: f64 = (assign1460_e1788).powf(assign1460_e1791);
        let assign1460_e1794: f64 = (assign1460_e1792 - 1.0);
        var_vtff1 = assign1460_e1794;
        var_vtff1_dn0 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn0)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn0 / assign1460_e1788))) };
        var_vtff1_dn1 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn1)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn1 / assign1460_e1788))) };
        var_vtff1_dn2 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn2)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn2 / assign1460_e1788))) };
        var_vtff1_dn3 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn3)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn3 / assign1460_e1788))) };
        var_vtff1_dn4 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn4)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn4 / assign1460_e1788))) };
        var_vtff1_dn5 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn5)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn5 / assign1460_e1788))) };
        var_vtff1_dn6 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn6)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn6 / assign1460_e1788))) };
        var_vtff1_dn7 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn7)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn7 / assign1460_e1788))) };
        var_vtff1_dn8 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn8)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn8 / assign1460_e1788))) };
        var_vtff1_dn9 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn9)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn9 / assign1460_e1788))) };
        var_vtff1_db0 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db0)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db0 / assign1460_e1788))) };
        var_vtff1_db1 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db1)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db1 / assign1460_e1788))) };
        var_vtff1_db2 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db2)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db2 / assign1460_e1788))) };
        var_vtff1_db3 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db3)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db3 / assign1460_e1788))) };
        var_vtff1_db4 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db4)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db4 / assign1460_e1788))) };
        var_vtff1_db5 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db5)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db5 / assign1460_e1788))) };
        var_vtff1_db6 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db6)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db6 / assign1460_e1788))) };
        var_vtff1_db7 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db7)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db7 / assign1460_e1788))) };
        var_vtff1_rv = 0.0;
        var_vtff1_rdn0 = 0.0;
        var_vtff1_rdn1 = 0.0;
        var_vtff1_rdn2 = 0.0;
        var_vtff1_rdn3 = 0.0;
        var_vtff1_rdn4 = 0.0;
        var_vtff1_rdn5 = 0.0;
        var_vtff1_rdn6 = 0.0;
        var_vtff1_rdn7 = 0.0;
        var_vtff1_rdn8 = 0.0;
        var_vtff1_rdn9 = 0.0;
        var_vtff1_rdb0 = 0.0;
        var_vtff1_rdb1 = 0.0;
        var_vtff1_rdb2 = 0.0;
        var_vtff1_rdb3 = 0.0;
        var_vtff1_rdb4 = 0.0;
        var_vtff1_rdb5 = 0.0;
        var_vtff1_rdb6 = 0.0;
        var_vtff1_rdb7 = 0.0;

        let assign1470_e1799: f64 = (p.p41 * var_vtff1);
        let assign1470_e1800: f64 = (1.0 + assign1470_e1799);
        let assign1470_e1801: f64 = (p.p19 * assign1470_e1800);
        var_tff = assign1470_e1801;
        var_tff_dn0 = (p.p19 * (p.p41 * var_vtff1_dn0));
        var_tff_dn1 = (p.p19 * (p.p41 * var_vtff1_dn1));
        var_tff_dn2 = (p.p19 * (p.p41 * var_vtff1_dn2));
        var_tff_dn3 = (p.p19 * (p.p41 * var_vtff1_dn3));
        var_tff_dn4 = (p.p19 * (p.p41 * var_vtff1_dn4));
        var_tff_dn5 = (p.p19 * (p.p41 * var_vtff1_dn5));
        var_tff_dn6 = (p.p19 * (p.p41 * var_vtff1_dn6));
        var_tff_dn7 = (p.p19 * (p.p41 * var_vtff1_dn7));
        var_tff_dn8 = (p.p19 * (p.p41 * var_vtff1_dn8));
        var_tff_dn9 = (p.p19 * (p.p41 * var_vtff1_dn9));
        var_tff_db0 = (p.p19 * (p.p41 * var_vtff1_db0));
        var_tff_db1 = (p.p19 * (p.p41 * var_vtff1_db1));
        var_tff_db2 = (p.p19 * (p.p41 * var_vtff1_db2));
        var_tff_db3 = (p.p19 * (p.p41 * var_vtff1_db3));
        var_tff_db4 = (p.p19 * (p.p41 * var_vtff1_db4));
        var_tff_db5 = (p.p19 * (p.p41 * var_vtff1_db5));
        var_tff_db6 = (p.p19 * (p.p41 * var_vtff1_db6));
        var_tff_db7 = (p.p19 * (p.p41 * var_vtff1_db7));
        var_tff_rv = 0.0;
        var_tff_rdn0 = 0.0;
        var_tff_rdn1 = 0.0;
        var_tff_rdn2 = 0.0;
        var_tff_rdn3 = 0.0;
        var_tff_rdn4 = 0.0;
        var_tff_rdn5 = 0.0;
        var_tff_rdn6 = 0.0;
        var_tff_rdn7 = 0.0;
        var_tff_rdn8 = 0.0;
        var_tff_rdn9 = 0.0;
        var_tff_rdb0 = 0.0;
        var_tff_rdb1 = 0.0;
        var_tff_rdb2 = 0.0;
        var_tff_rdb3 = 0.0;
        var_tff_rdb4 = 0.0;
        var_tff_rdb5 = 0.0;
        var_tff_rdb6 = 0.0;
        var_tff_rdb7 = 0.0;

        let assign1480_e1804: f64 = (var_tff * var_ifwd);
        var_qde = assign1480_e1804;
        var_qde_dn0 = ((var_tff_dn0 * var_ifwd) + (var_tff * var_ifwd_dn0));
        var_qde_dn1 = ((var_tff_dn1 * var_ifwd) + (var_tff * var_ifwd_dn1));
        var_qde_dn2 = ((var_tff_dn2 * var_ifwd) + (var_tff * var_ifwd_dn2));
        var_qde_dn3 = ((var_tff_dn3 * var_ifwd) + (var_tff * var_ifwd_dn3));
        var_qde_dn4 = ((var_tff_dn4 * var_ifwd) + (var_tff * var_ifwd_dn4));
        var_qde_dn5 = ((var_tff_dn5 * var_ifwd) + (var_tff * var_ifwd_dn5));
        var_qde_dn6 = ((var_tff_dn6 * var_ifwd) + (var_tff * var_ifwd_dn6));
        var_qde_dn7 = ((var_tff_dn7 * var_ifwd) + (var_tff * var_ifwd_dn7));
        var_qde_dn8 = ((var_tff_dn8 * var_ifwd) + (var_tff * var_ifwd_dn8));
        var_qde_dn9 = ((var_tff_dn9 * var_ifwd) + (var_tff * var_ifwd_dn9));
        var_qde_db0 = ((var_tff_db0 * var_ifwd) + (var_tff * var_ifwd_db0));
        var_qde_db1 = ((var_tff_db1 * var_ifwd) + (var_tff * var_ifwd_db1));
        var_qde_db2 = ((var_tff_db2 * var_ifwd) + (var_tff * var_ifwd_db2));
        var_qde_db3 = ((var_tff_db3 * var_ifwd) + (var_tff * var_ifwd_db3));
        var_qde_db4 = ((var_tff_db4 * var_ifwd) + (var_tff * var_ifwd_db4));
        var_qde_db5 = ((var_tff_db5 * var_ifwd) + (var_tff * var_ifwd_db5));
        var_qde_db6 = ((var_tff_db6 * var_ifwd) + (var_tff * var_ifwd_db6));
        var_qde_db7 = ((var_tff_db7 * var_ifwd) + (var_tff * var_ifwd_db7));
        var_qde_rv = 0.0;
        var_qde_rdn0 = 0.0;
        var_qde_rdn1 = 0.0;
        var_qde_rdn2 = 0.0;
        var_qde_rdn3 = 0.0;
        var_qde_rdn4 = 0.0;
        var_qde_rdn5 = 0.0;
        var_qde_rdn6 = 0.0;
        var_qde_rdn7 = 0.0;
        var_qde_rdn8 = 0.0;
        var_qde_rdn9 = 0.0;
        var_qde_rdb0 = 0.0;
        var_qde_rdb1 = 0.0;
        var_qde_rdb2 = 0.0;
        var_qde_rdb3 = 0.0;
        var_qde_rdb4 = 0.0;
        var_qde_rdb5 = 0.0;
        var_qde_rdb6 = 0.0;
        var_qde_rdb7 = 0.0;

        let assign1490_e1807: f64 = (p.p73 * var_itr);
        var_qdc = assign1490_e1807;
        var_qdc_dn0 = (p.p73 * var_itr_dn0);
        var_qdc_dn1 = (p.p73 * var_itr_dn1);
        var_qdc_dn2 = (p.p73 * var_itr_dn2);
        var_qdc_dn3 = (p.p73 * var_itr_dn3);
        var_qdc_dn4 = (p.p73 * var_itr_dn4);
        var_qdc_dn5 = (p.p73 * var_itr_dn5);
        var_qdc_dn6 = (p.p73 * var_itr_dn6);
        var_qdc_dn7 = (p.p73 * var_itr_dn7);
        var_qdc_dn8 = (p.p73 * var_itr_dn8);
        var_qdc_dn9 = (p.p73 * var_itr_dn9);
        var_qdc_db0 = (p.p73 * var_itr_db0);
        var_qdc_db1 = (p.p73 * var_itr_db1);
        var_qdc_db2 = (p.p73 * var_itr_db2);
        var_qdc_db3 = (p.p73 * var_itr_db3);
        var_qdc_db4 = (p.p73 * var_itr_db4);
        var_qdc_db5 = (p.p73 * var_itr_db5);
        var_qdc_db6 = (p.p73 * var_itr_db6);
        var_qdc_db7 = (p.p73 * var_itr_db7);
        var_qdc_rv = 0.0;
        var_qdc_rdn0 = 0.0;
        var_qdc_rdn1 = 0.0;
        var_qdc_rdn2 = 0.0;
        var_qdc_rdn3 = 0.0;
        var_qdc_rdn4 = 0.0;
        var_qdc_rdn5 = 0.0;
        var_qdc_rdn6 = 0.0;
        var_qdc_rdn7 = 0.0;
        var_qdc_rdn8 = 0.0;
        var_qdc_rdn9 = 0.0;
        var_qdc_rdb0 = 0.0;
        var_qdc_rdb1 = 0.0;
        var_qdc_rdb2 = 0.0;
        var_qdc_rdb3 = 0.0;
        var_qdc_rdb4 = 0.0;
        var_qdc_rdb5 = 0.0;
        var_qdc_rdb6 = 0.0;
        var_qdc_rdb7 = 0.0;

        *var_dkqb_slot = var_dkqb;
        *var_dkqb_db0_slot = var_dkqb_db0;
        *var_dkqb_db1_slot = var_dkqb_db1;
        *var_dkqb_db2_slot = var_dkqb_db2;
        *var_dkqb_db3_slot = var_dkqb_db3;
        *var_dkqb_db4_slot = var_dkqb_db4;
        *var_dkqb_db5_slot = var_dkqb_db5;
        *var_dkqb_db6_slot = var_dkqb_db6;
        *var_dkqb_db7_slot = var_dkqb_db7;
        *var_dkqb_dn0_slot = var_dkqb_dn0;
        *var_dkqb_dn1_slot = var_dkqb_dn1;
        *var_dkqb_dn2_slot = var_dkqb_dn2;
        *var_dkqb_dn3_slot = var_dkqb_dn3;
        *var_dkqb_dn4_slot = var_dkqb_dn4;
        *var_dkqb_dn5_slot = var_dkqb_dn5;
        *var_dkqb_dn6_slot = var_dkqb_dn6;
        *var_dkqb_dn7_slot = var_dkqb_dn7;
        *var_dkqb_dn8_slot = var_dkqb_dn8;
        *var_dkqb_dn9_slot = var_dkqb_dn9;
        *var_dkqb_rdb0_slot = var_dkqb_rdb0;
        *var_dkqb_rdb1_slot = var_dkqb_rdb1;
        *var_dkqb_rdb2_slot = var_dkqb_rdb2;
        *var_dkqb_rdb3_slot = var_dkqb_rdb3;
        *var_dkqb_rdb4_slot = var_dkqb_rdb4;
        *var_dkqb_rdb5_slot = var_dkqb_rdb5;
        *var_dkqb_rdb6_slot = var_dkqb_rdb6;
        *var_dkqb_rdb7_slot = var_dkqb_rdb7;
        *var_dkqb_rdn0_slot = var_dkqb_rdn0;
        *var_dkqb_rdn1_slot = var_dkqb_rdn1;
        *var_dkqb_rdn2_slot = var_dkqb_rdn2;
        *var_dkqb_rdn3_slot = var_dkqb_rdn3;
        *var_dkqb_rdn4_slot = var_dkqb_rdn4;
        *var_dkqb_rdn5_slot = var_dkqb_rdn5;
        *var_dkqb_rdn6_slot = var_dkqb_rdn6;
        *var_dkqb_rdn7_slot = var_dkqb_rdn7;
        *var_dkqb_rdn8_slot = var_dkqb_rdn8;
        *var_dkqb_rdn9_slot = var_dkqb_rdn9;
        *var_dkqb_rv_slot = var_dkqb_rv;
        *var_ikq1_slot = var_ikq1;
        *var_ikq1_db0_slot = var_ikq1_db0;
        *var_ikq1_db1_slot = var_ikq1_db1;
        *var_ikq1_db2_slot = var_ikq1_db2;
        *var_ikq1_db3_slot = var_ikq1_db3;
        *var_ikq1_db4_slot = var_ikq1_db4;
        *var_ikq1_db5_slot = var_ikq1_db5;
        *var_ikq1_db6_slot = var_ikq1_db6;
        *var_ikq1_db7_slot = var_ikq1_db7;
        *var_ikq1_dn0_slot = var_ikq1_dn0;
        *var_ikq1_dn1_slot = var_ikq1_dn1;
        *var_ikq1_dn2_slot = var_ikq1_dn2;
        *var_ikq1_dn3_slot = var_ikq1_dn3;
        *var_ikq1_dn4_slot = var_ikq1_dn4;
        *var_ikq1_dn5_slot = var_ikq1_dn5;
        *var_ikq1_dn6_slot = var_ikq1_dn6;
        *var_ikq1_dn7_slot = var_ikq1_dn7;
        *var_ikq1_dn8_slot = var_ikq1_dn8;
        *var_ikq1_dn9_slot = var_ikq1_dn9;
        *var_ikq1_rdb0_slot = var_ikq1_rdb0;
        *var_ikq1_rdb1_slot = var_ikq1_rdb1;
        *var_ikq1_rdb2_slot = var_ikq1_rdb2;
        *var_ikq1_rdb3_slot = var_ikq1_rdb3;
        *var_ikq1_rdb4_slot = var_ikq1_rdb4;
        *var_ikq1_rdb5_slot = var_ikq1_rdb5;
        *var_ikq1_rdb6_slot = var_ikq1_rdb6;
        *var_ikq1_rdb7_slot = var_ikq1_rdb7;
        *var_ikq1_rdn0_slot = var_ikq1_rdn0;
        *var_ikq1_rdn1_slot = var_ikq1_rdn1;
        *var_ikq1_rdn2_slot = var_ikq1_rdn2;
        *var_ikq1_rdn3_slot = var_ikq1_rdn3;
        *var_ikq1_rdn4_slot = var_ikq1_rdn4;
        *var_ikq1_rdn5_slot = var_ikq1_rdn5;
        *var_ikq1_rdn6_slot = var_ikq1_rdn6;
        *var_ikq1_rdn7_slot = var_ikq1_rdn7;
        *var_ikq1_rdn8_slot = var_ikq1_rdn8;
        *var_ikq1_rdn9_slot = var_ikq1_rdn9;
        *var_ikq1_rv_slot = var_ikq1_rv;
        *var_ikqb_slot = var_ikqb;
        *var_ikqb_db0_slot = var_ikqb_db0;
        *var_ikqb_db1_slot = var_ikqb_db1;
        *var_ikqb_db2_slot = var_ikqb_db2;
        *var_ikqb_db3_slot = var_ikqb_db3;
        *var_ikqb_db4_slot = var_ikqb_db4;
        *var_ikqb_db5_slot = var_ikqb_db5;
        *var_ikqb_db6_slot = var_ikqb_db6;
        *var_ikqb_db7_slot = var_ikqb_db7;
        *var_ikqb_dn0_slot = var_ikqb_dn0;
        *var_ikqb_dn1_slot = var_ikqb_dn1;
        *var_ikqb_dn2_slot = var_ikqb_dn2;
        *var_ikqb_dn3_slot = var_ikqb_dn3;
        *var_ikqb_dn4_slot = var_ikqb_dn4;
        *var_ikqb_dn5_slot = var_ikqb_dn5;
        *var_ikqb_dn6_slot = var_ikqb_dn6;
        *var_ikqb_dn7_slot = var_ikqb_dn7;
        *var_ikqb_dn8_slot = var_ikqb_dn8;
        *var_ikqb_dn9_slot = var_ikqb_dn9;
        *var_ikqb_rdb0_slot = var_ikqb_rdb0;
        *var_ikqb_rdb1_slot = var_ikqb_rdb1;
        *var_ikqb_rdb2_slot = var_ikqb_rdb2;
        *var_ikqb_rdb3_slot = var_ikqb_rdb3;
        *var_ikqb_rdb4_slot = var_ikqb_rdb4;
        *var_ikqb_rdb5_slot = var_ikqb_rdb5;
        *var_ikqb_rdb6_slot = var_ikqb_rdb6;
        *var_ikqb_rdb7_slot = var_ikqb_rdb7;
        *var_ikqb_rdn0_slot = var_ikqb_rdn0;
        *var_ikqb_rdn1_slot = var_ikqb_rdn1;
        *var_ikqb_rdn2_slot = var_ikqb_rdn2;
        *var_ikqb_rdn3_slot = var_ikqb_rdn3;
        *var_ikqb_rdn4_slot = var_ikqb_rdn4;
        *var_ikqb_rdn5_slot = var_ikqb_rdn5;
        *var_ikqb_rdn6_slot = var_ikqb_rdn6;
        *var_ikqb_rdn7_slot = var_ikqb_rdn7;
        *var_ikqb_rdn8_slot = var_ikqb_rdn8;
        *var_ikqb_rdn9_slot = var_ikqb_rdn9;
        *var_ikqb_rv_slot = var_ikqb_rv;
        *var_itr_slot = var_itr;
        *var_itr_db0_slot = var_itr_db0;
        *var_itr_db1_slot = var_itr_db1;
        *var_itr_db2_slot = var_itr_db2;
        *var_itr_db3_slot = var_itr_db3;
        *var_itr_db4_slot = var_itr_db4;
        *var_itr_db5_slot = var_itr_db5;
        *var_itr_db6_slot = var_itr_db6;
        *var_itr_db7_slot = var_itr_db7;
        *var_itr_dn0_slot = var_itr_dn0;
        *var_itr_dn1_slot = var_itr_dn1;
        *var_itr_dn2_slot = var_itr_dn2;
        *var_itr_dn3_slot = var_itr_dn3;
        *var_itr_dn4_slot = var_itr_dn4;
        *var_itr_dn5_slot = var_itr_dn5;
        *var_itr_dn6_slot = var_itr_dn6;
        *var_itr_dn7_slot = var_itr_dn7;
        *var_itr_dn8_slot = var_itr_dn8;
        *var_itr_dn9_slot = var_itr_dn9;
        *var_itr_rdb0_slot = var_itr_rdb0;
        *var_itr_rdb1_slot = var_itr_rdb1;
        *var_itr_rdb2_slot = var_itr_rdb2;
        *var_itr_rdb3_slot = var_itr_rdb3;
        *var_itr_rdb4_slot = var_itr_rdb4;
        *var_itr_rdb5_slot = var_itr_rdb5;
        *var_itr_rdb6_slot = var_itr_rdb6;
        *var_itr_rdb7_slot = var_itr_rdb7;
        *var_itr_rdn0_slot = var_itr_rdn0;
        *var_itr_rdn1_slot = var_itr_rdn1;
        *var_itr_rdn2_slot = var_itr_rdn2;
        *var_itr_rdn3_slot = var_itr_rdn3;
        *var_itr_rdn4_slot = var_itr_rdn4;
        *var_itr_rdn5_slot = var_itr_rdn5;
        *var_itr_rdn6_slot = var_itr_rdn6;
        *var_itr_rdn7_slot = var_itr_rdn7;
        *var_itr_rdn8_slot = var_itr_rdn8;
        *var_itr_rdn9_slot = var_itr_rdn9;
        *var_itr_rv_slot = var_itr_rv;
        *var_itzf_slot = var_itzf;
        *var_itzf_db0_slot = var_itzf_db0;
        *var_itzf_db1_slot = var_itzf_db1;
        *var_itzf_db2_slot = var_itzf_db2;
        *var_itzf_db3_slot = var_itzf_db3;
        *var_itzf_db4_slot = var_itzf_db4;
        *var_itzf_db5_slot = var_itzf_db5;
        *var_itzf_db6_slot = var_itzf_db6;
        *var_itzf_db7_slot = var_itzf_db7;
        *var_itzf_dn0_slot = var_itzf_dn0;
        *var_itzf_dn1_slot = var_itzf_dn1;
        *var_itzf_dn2_slot = var_itzf_dn2;
        *var_itzf_dn3_slot = var_itzf_dn3;
        *var_itzf_dn4_slot = var_itzf_dn4;
        *var_itzf_dn5_slot = var_itzf_dn5;
        *var_itzf_dn6_slot = var_itzf_dn6;
        *var_itzf_dn7_slot = var_itzf_dn7;
        *var_itzf_dn8_slot = var_itzf_dn8;
        *var_itzf_dn9_slot = var_itzf_dn9;
        *var_itzf_rdb0_slot = var_itzf_rdb0;
        *var_itzf_rdb1_slot = var_itzf_rdb1;
        *var_itzf_rdb2_slot = var_itzf_rdb2;
        *var_itzf_rdb3_slot = var_itzf_rdb3;
        *var_itzf_rdb4_slot = var_itzf_rdb4;
        *var_itzf_rdb5_slot = var_itzf_rdb5;
        *var_itzf_rdb6_slot = var_itzf_rdb6;
        *var_itzf_rdb7_slot = var_itzf_rdb7;
        *var_itzf_rdn0_slot = var_itzf_rdn0;
        *var_itzf_rdn1_slot = var_itzf_rdn1;
        *var_itzf_rdn2_slot = var_itzf_rdn2;
        *var_itzf_rdn3_slot = var_itzf_rdn3;
        *var_itzf_rdn4_slot = var_itzf_rdn4;
        *var_itzf_rdn5_slot = var_itzf_rdn5;
        *var_itzf_rdn6_slot = var_itzf_rdn6;
        *var_itzf_rdn7_slot = var_itzf_rdn7;
        *var_itzf_rdn8_slot = var_itzf_rdn8;
        *var_itzf_rdn9_slot = var_itzf_rdn9;
        *var_itzf_rv_slot = var_itzf_rv;
        *var_kq2_slot = var_kq2;
        *var_kq2_db0_slot = var_kq2_db0;
        *var_kq2_db1_slot = var_kq2_db1;
        *var_kq2_db2_slot = var_kq2_db2;
        *var_kq2_db3_slot = var_kq2_db3;
        *var_kq2_db4_slot = var_kq2_db4;
        *var_kq2_db5_slot = var_kq2_db5;
        *var_kq2_db6_slot = var_kq2_db6;
        *var_kq2_db7_slot = var_kq2_db7;
        *var_kq2_dn0_slot = var_kq2_dn0;
        *var_kq2_dn1_slot = var_kq2_dn1;
        *var_kq2_dn2_slot = var_kq2_dn2;
        *var_kq2_dn3_slot = var_kq2_dn3;
        *var_kq2_dn4_slot = var_kq2_dn4;
        *var_kq2_dn5_slot = var_kq2_dn5;
        *var_kq2_dn6_slot = var_kq2_dn6;
        *var_kq2_dn7_slot = var_kq2_dn7;
        *var_kq2_dn8_slot = var_kq2_dn8;
        *var_kq2_dn9_slot = var_kq2_dn9;
        *var_kq2_rdb0_slot = var_kq2_rdb0;
        *var_kq2_rdb1_slot = var_kq2_rdb1;
        *var_kq2_rdb2_slot = var_kq2_rdb2;
        *var_kq2_rdb3_slot = var_kq2_rdb3;
        *var_kq2_rdb4_slot = var_kq2_rdb4;
        *var_kq2_rdb5_slot = var_kq2_rdb5;
        *var_kq2_rdb6_slot = var_kq2_rdb6;
        *var_kq2_rdb7_slot = var_kq2_rdb7;
        *var_kq2_rdn0_slot = var_kq2_rdn0;
        *var_kq2_rdn1_slot = var_kq2_rdn1;
        *var_kq2_rdn2_slot = var_kq2_rdn2;
        *var_kq2_rdn3_slot = var_kq2_rdn3;
        *var_kq2_rdn4_slot = var_kq2_rdn4;
        *var_kq2_rdn5_slot = var_kq2_rdn5;
        *var_kq2_rdn6_slot = var_kq2_rdn6;
        *var_kq2_rdn7_slot = var_kq2_rdn7;
        *var_kq2_rdn8_slot = var_kq2_rdn8;
        *var_kq2_rdn9_slot = var_kq2_rdn9;
        *var_kq2_rv_slot = var_kq2_rv;
        *var_oikf_slot = var_oikf;
        *var_oikf_db0_slot = var_oikf_db0;
        *var_oikf_db1_slot = var_oikf_db1;
        *var_oikf_db2_slot = var_oikf_db2;
        *var_oikf_db3_slot = var_oikf_db3;
        *var_oikf_db4_slot = var_oikf_db4;
        *var_oikf_db5_slot = var_oikf_db5;
        *var_oikf_db6_slot = var_oikf_db6;
        *var_oikf_db7_slot = var_oikf_db7;
        *var_oikf_dn0_slot = var_oikf_dn0;
        *var_oikf_dn1_slot = var_oikf_dn1;
        *var_oikf_dn2_slot = var_oikf_dn2;
        *var_oikf_dn3_slot = var_oikf_dn3;
        *var_oikf_dn4_slot = var_oikf_dn4;
        *var_oikf_dn5_slot = var_oikf_dn5;
        *var_oikf_dn6_slot = var_oikf_dn6;
        *var_oikf_dn7_slot = var_oikf_dn7;
        *var_oikf_dn8_slot = var_oikf_dn8;
        *var_oikf_dn9_slot = var_oikf_dn9;
        *var_oikf_rdb0_slot = var_oikf_rdb0;
        *var_oikf_rdb1_slot = var_oikf_rdb1;
        *var_oikf_rdb2_slot = var_oikf_rdb2;
        *var_oikf_rdb3_slot = var_oikf_rdb3;
        *var_oikf_rdb4_slot = var_oikf_rdb4;
        *var_oikf_rdb5_slot = var_oikf_rdb5;
        *var_oikf_rdb6_slot = var_oikf_rdb6;
        *var_oikf_rdb7_slot = var_oikf_rdb7;
        *var_oikf_rdn0_slot = var_oikf_rdn0;
        *var_oikf_rdn1_slot = var_oikf_rdn1;
        *var_oikf_rdn2_slot = var_oikf_rdn2;
        *var_oikf_rdn3_slot = var_oikf_rdn3;
        *var_oikf_rdn4_slot = var_oikf_rdn4;
        *var_oikf_rdn5_slot = var_oikf_rdn5;
        *var_oikf_rdn6_slot = var_oikf_rdn6;
        *var_oikf_rdn7_slot = var_oikf_rdn7;
        *var_oikf_rdn8_slot = var_oikf_rdn8;
        *var_oikf_rdn9_slot = var_oikf_rdn9;
        *var_oikf_rv_slot = var_oikf_rv;
        *var_qdc_slot = var_qdc;
        *var_qdc_db0_slot = var_qdc_db0;
        *var_qdc_db1_slot = var_qdc_db1;
        *var_qdc_db2_slot = var_qdc_db2;
        *var_qdc_db3_slot = var_qdc_db3;
        *var_qdc_db4_slot = var_qdc_db4;
        *var_qdc_db5_slot = var_qdc_db5;
        *var_qdc_db6_slot = var_qdc_db6;
        *var_qdc_db7_slot = var_qdc_db7;
        *var_qdc_dn0_slot = var_qdc_dn0;
        *var_qdc_dn1_slot = var_qdc_dn1;
        *var_qdc_dn2_slot = var_qdc_dn2;
        *var_qdc_dn3_slot = var_qdc_dn3;
        *var_qdc_dn4_slot = var_qdc_dn4;
        *var_qdc_dn5_slot = var_qdc_dn5;
        *var_qdc_dn6_slot = var_qdc_dn6;
        *var_qdc_dn7_slot = var_qdc_dn7;
        *var_qdc_dn8_slot = var_qdc_dn8;
        *var_qdc_dn9_slot = var_qdc_dn9;
        *var_qdc_rdb0_slot = var_qdc_rdb0;
        *var_qdc_rdb1_slot = var_qdc_rdb1;
        *var_qdc_rdb2_slot = var_qdc_rdb2;
        *var_qdc_rdb3_slot = var_qdc_rdb3;
        *var_qdc_rdb4_slot = var_qdc_rdb4;
        *var_qdc_rdb5_slot = var_qdc_rdb5;
        *var_qdc_rdb6_slot = var_qdc_rdb6;
        *var_qdc_rdb7_slot = var_qdc_rdb7;
        *var_qdc_rdn0_slot = var_qdc_rdn0;
        *var_qdc_rdn1_slot = var_qdc_rdn1;
        *var_qdc_rdn2_slot = var_qdc_rdn2;
        *var_qdc_rdn3_slot = var_qdc_rdn3;
        *var_qdc_rdn4_slot = var_qdc_rdn4;
        *var_qdc_rdn5_slot = var_qdc_rdn5;
        *var_qdc_rdn6_slot = var_qdc_rdn6;
        *var_qdc_rdn7_slot = var_qdc_rdn7;
        *var_qdc_rdn8_slot = var_qdc_rdn8;
        *var_qdc_rdn9_slot = var_qdc_rdn9;
        *var_qdc_rv_slot = var_qdc_rv;
        *var_qde_slot = var_qde;
        *var_qde_db0_slot = var_qde_db0;
        *var_qde_db1_slot = var_qde_db1;
        *var_qde_db2_slot = var_qde_db2;
        *var_qde_db3_slot = var_qde_db3;
        *var_qde_db4_slot = var_qde_db4;
        *var_qde_db5_slot = var_qde_db5;
        *var_qde_db6_slot = var_qde_db6;
        *var_qde_db7_slot = var_qde_db7;
        *var_qde_dn0_slot = var_qde_dn0;
        *var_qde_dn1_slot = var_qde_dn1;
        *var_qde_dn2_slot = var_qde_dn2;
        *var_qde_dn3_slot = var_qde_dn3;
        *var_qde_dn4_slot = var_qde_dn4;
        *var_qde_dn5_slot = var_qde_dn5;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qde_dn7_slot = var_qde_dn7;
        *var_qde_dn8_slot = var_qde_dn8;
        *var_qde_dn9_slot = var_qde_dn9;
        *var_qde_rdb0_slot = var_qde_rdb0;
        *var_qde_rdb1_slot = var_qde_rdb1;
        *var_qde_rdb2_slot = var_qde_rdb2;
        *var_qde_rdb3_slot = var_qde_rdb3;
        *var_qde_rdb4_slot = var_qde_rdb4;
        *var_qde_rdb5_slot = var_qde_rdb5;
        *var_qde_rdb6_slot = var_qde_rdb6;
        *var_qde_rdb7_slot = var_qde_rdb7;
        *var_qde_rdn0_slot = var_qde_rdn0;
        *var_qde_rdn1_slot = var_qde_rdn1;
        *var_qde_rdn2_slot = var_qde_rdn2;
        *var_qde_rdn3_slot = var_qde_rdn3;
        *var_qde_rdn4_slot = var_qde_rdn4;
        *var_qde_rdn5_slot = var_qde_rdn5;
        *var_qde_rdn6_slot = var_qde_rdn6;
        *var_qde_rdn7_slot = var_qde_rdn7;
        *var_qde_rdn8_slot = var_qde_rdn8;
        *var_qde_rdn9_slot = var_qde_rdn9;
        *var_qde_rv_slot = var_qde_rv;
        *var_tff_slot = var_tff;
        *var_tff_db0_slot = var_tff_db0;
        *var_tff_db1_slot = var_tff_db1;
        *var_tff_db2_slot = var_tff_db2;
        *var_tff_db3_slot = var_tff_db3;
        *var_tff_db4_slot = var_tff_db4;
        *var_tff_db5_slot = var_tff_db5;
        *var_tff_db6_slot = var_tff_db6;
        *var_tff_db7_slot = var_tff_db7;
        *var_tff_dn0_slot = var_tff_dn0;
        *var_tff_dn1_slot = var_tff_dn1;
        *var_tff_dn2_slot = var_tff_dn2;
        *var_tff_dn3_slot = var_tff_dn3;
        *var_tff_dn4_slot = var_tff_dn4;
        *var_tff_dn5_slot = var_tff_dn5;
        *var_tff_dn6_slot = var_tff_dn6;
        *var_tff_dn7_slot = var_tff_dn7;
        *var_tff_dn8_slot = var_tff_dn8;
        *var_tff_dn9_slot = var_tff_dn9;
        *var_tff_rdb0_slot = var_tff_rdb0;
        *var_tff_rdb1_slot = var_tff_rdb1;
        *var_tff_rdb2_slot = var_tff_rdb2;
        *var_tff_rdb3_slot = var_tff_rdb3;
        *var_tff_rdb4_slot = var_tff_rdb4;
        *var_tff_rdb5_slot = var_tff_rdb5;
        *var_tff_rdb6_slot = var_tff_rdb6;
        *var_tff_rdb7_slot = var_tff_rdb7;
        *var_tff_rdn0_slot = var_tff_rdn0;
        *var_tff_rdn1_slot = var_tff_rdn1;
        *var_tff_rdn2_slot = var_tff_rdn2;
        *var_tff_rdn3_slot = var_tff_rdn3;
        *var_tff_rdn4_slot = var_tff_rdn4;
        *var_tff_rdn5_slot = var_tff_rdn5;
        *var_tff_rdn6_slot = var_tff_rdn6;
        *var_tff_rdn7_slot = var_tff_rdn7;
        *var_tff_rdn8_slot = var_tff_rdn8;
        *var_tff_rdn9_slot = var_tff_rdn9;
        *var_tff_rv_slot = var_tff_rv;
        *var_vtff_slot = var_vtff;
        *var_vtff1_slot = var_vtff1;
        *var_vtff1_db0_slot = var_vtff1_db0;
        *var_vtff1_db1_slot = var_vtff1_db1;
        *var_vtff1_db2_slot = var_vtff1_db2;
        *var_vtff1_db3_slot = var_vtff1_db3;
        *var_vtff1_db4_slot = var_vtff1_db4;
        *var_vtff1_db5_slot = var_vtff1_db5;
        *var_vtff1_db6_slot = var_vtff1_db6;
        *var_vtff1_db7_slot = var_vtff1_db7;
        *var_vtff1_dn0_slot = var_vtff1_dn0;
        *var_vtff1_dn1_slot = var_vtff1_dn1;
        *var_vtff1_dn2_slot = var_vtff1_dn2;
        *var_vtff1_dn3_slot = var_vtff1_dn3;
        *var_vtff1_dn4_slot = var_vtff1_dn4;
        *var_vtff1_dn5_slot = var_vtff1_dn5;
        *var_vtff1_dn6_slot = var_vtff1_dn6;
        *var_vtff1_dn7_slot = var_vtff1_dn7;
        *var_vtff1_dn8_slot = var_vtff1_dn8;
        *var_vtff1_dn9_slot = var_vtff1_dn9;
        *var_vtff1_rdb0_slot = var_vtff1_rdb0;
        *var_vtff1_rdb1_slot = var_vtff1_rdb1;
        *var_vtff1_rdb2_slot = var_vtff1_rdb2;
        *var_vtff1_rdb3_slot = var_vtff1_rdb3;
        *var_vtff1_rdb4_slot = var_vtff1_rdb4;
        *var_vtff1_rdb5_slot = var_vtff1_rdb5;
        *var_vtff1_rdb6_slot = var_vtff1_rdb6;
        *var_vtff1_rdb7_slot = var_vtff1_rdb7;
        *var_vtff1_rdn0_slot = var_vtff1_rdn0;
        *var_vtff1_rdn1_slot = var_vtff1_rdn1;
        *var_vtff1_rdn2_slot = var_vtff1_rdn2;
        *var_vtff1_rdn3_slot = var_vtff1_rdn3;
        *var_vtff1_rdn4_slot = var_vtff1_rdn4;
        *var_vtff1_rdn5_slot = var_vtff1_rdn5;
        *var_vtff1_rdn6_slot = var_vtff1_rdn6;
        *var_vtff1_rdn7_slot = var_vtff1_rdn7;
        *var_vtff1_rdn8_slot = var_vtff1_rdn8;
        *var_vtff1_rdn9_slot = var_vtff1_rdn9;
        *var_vtff1_rv_slot = var_vtff1_rv;
        *var_vtff_db0_slot = var_vtff_db0;
        *var_vtff_db1_slot = var_vtff_db1;
        *var_vtff_db2_slot = var_vtff_db2;
        *var_vtff_db3_slot = var_vtff_db3;
        *var_vtff_db4_slot = var_vtff_db4;
        *var_vtff_db5_slot = var_vtff_db5;
        *var_vtff_db6_slot = var_vtff_db6;
        *var_vtff_db7_slot = var_vtff_db7;
        *var_vtff_dn0_slot = var_vtff_dn0;
        *var_vtff_dn1_slot = var_vtff_dn1;
        *var_vtff_dn2_slot = var_vtff_dn2;
        *var_vtff_dn3_slot = var_vtff_dn3;
        *var_vtff_dn4_slot = var_vtff_dn4;
        *var_vtff_dn5_slot = var_vtff_dn5;
        *var_vtff_dn6_slot = var_vtff_dn6;
        *var_vtff_dn7_slot = var_vtff_dn7;
        *var_vtff_dn8_slot = var_vtff_dn8;
        *var_vtff_dn9_slot = var_vtff_dn9;
        *var_vtff_rdb0_slot = var_vtff_rdb0;
        *var_vtff_rdb1_slot = var_vtff_rdb1;
        *var_vtff_rdb2_slot = var_vtff_rdb2;
        *var_vtff_rdb3_slot = var_vtff_rdb3;
        *var_vtff_rdb4_slot = var_vtff_rdb4;
        *var_vtff_rdb5_slot = var_vtff_rdb5;
        *var_vtff_rdb6_slot = var_vtff_rdb6;
        *var_vtff_rdb7_slot = var_vtff_rdb7;
        *var_vtff_rdn0_slot = var_vtff_rdn0;
        *var_vtff_rdn1_slot = var_vtff_rdn1;
        *var_vtff_rdn2_slot = var_vtff_rdn2;
        *var_vtff_rdn3_slot = var_vtff_rdn3;
        *var_vtff_rdn4_slot = var_vtff_rdn4;
        *var_vtff_rdn5_slot = var_vtff_rdn5;
        *var_vtff_rdn6_slot = var_vtff_rdn6;
        *var_vtff_rdn7_slot = var_vtff_rdn7;
        *var_vtff_rdn8_slot = var_vtff_rdn8;
        *var_vtff_rdn9_slot = var_vtff_rdn9;
        *var_vtff_rv_slot = var_vtff_rv;
    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        var_cjs_t: f64,
        var_cjs_t_db0: f64,
        var_cjs_t_db1: f64,
        var_cjs_t_db2: f64,
        var_cjs_t_db3: f64,
        var_cjs_t_db4: f64,
        var_cjs_t_db5: f64,
        var_cjs_t_db6: f64,
        var_cjs_t_db7: f64,
        var_cjs_t_dn0: f64,
        var_cjs_t_dn1: f64,
        var_cjs_t_dn2: f64,
        var_cjs_t_dn3: f64,
        var_cjs_t_dn4: f64,
        var_cjs_t_dn5: f64,
        var_cjs_t_dn6: f64,
        var_cjs_t_dn7: f64,
        var_cjs_t_dn8: f64,
        var_cjs_t_dn9: f64,
        var_vbiei: f64,
        var_vbiei_db0: f64,
        var_vbiei_db1: f64,
        var_vbiei_db2: f64,
        var_vbiei_db3: f64,
        var_vbiei_db4: f64,
        var_vbiei_db5: f64,
        var_vbiei_db6: f64,
        var_vbiei_db7: f64,
        var_vbiei_dn0: f64,
        var_vbiei_dn1: f64,
        var_vbiei_dn2: f64,
        var_vbiei_dn3: f64,
        var_vbiei_dn4: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vbiei_dn7: f64,
        var_vbiei_dn8: f64,
        var_vbiei_dn9: f64,
        var_veci: f64,
        var_veci_db0: f64,
        var_veci_db1: f64,
        var_veci_db2: f64,
        var_veci_db3: f64,
        var_veci_db4: f64,
        var_veci_db5: f64,
        var_veci_db6: f64,
        var_veci_db7: f64,
        var_veci_dn0: f64,
        var_veci_dn1: f64,
        var_veci_dn2: f64,
        var_veci_dn3: f64,
        var_veci_dn4: f64,
        var_veci_dn5: f64,
        var_veci_dn6: f64,
        var_veci_dn7: f64,
        var_veci_dn8: f64,
        var_veci_dn9: f64,
        var_vje_t: f64,
        var_vje_t_db0: f64,
        var_vje_t_db1: f64,
        var_vje_t_db2: f64,
        var_vje_t_db3: f64,
        var_vje_t_db4: f64,
        var_vje_t_db5: f64,
        var_vje_t_db6: f64,
        var_vje_t_db7: f64,
        var_vje_t_dn0: f64,
        var_vje_t_dn1: f64,
        var_vje_t_dn2: f64,
        var_vje_t_dn3: f64,
        var_vje_t_dn4: f64,
        var_vje_t_dn5: f64,
        var_vje_t_dn6: f64,
        var_vje_t_dn7: f64,
        var_vje_t_dn8: f64,
        var_vje_t_dn9: f64,
        var_vjs_t: f64,
        var_vjs_t_db0: f64,
        var_vjs_t_db1: f64,
        var_vjs_t_db2: f64,
        var_vjs_t_db3: f64,
        var_vjs_t_db4: f64,
        var_vjs_t_db5: f64,
        var_vjs_t_db6: f64,
        var_vjs_t_db7: f64,
        var_vjs_t_dn0: f64,
        var_vjs_t_dn1: f64,
        var_vjs_t_dn2: f64,
        var_vjs_t_dn3: f64,
        var_vjs_t_dn4: f64,
        var_vjs_t_dn5: f64,
        var_vjs_t_dn6: f64,
        var_vjs_t_dn7: f64,
        var_vjs_t_dn8: f64,
        var_vjs_t_dn9: f64,
        var_dv0_slot: &mut f64,
        var_dv0_db0_slot: &mut f64,
        var_dv0_db1_slot: &mut f64,
        var_dv0_db2_slot: &mut f64,
        var_dv0_db3_slot: &mut f64,
        var_dv0_db4_slot: &mut f64,
        var_dv0_db5_slot: &mut f64,
        var_dv0_db6_slot: &mut f64,
        var_dv0_db7_slot: &mut f64,
        var_dv0_dn0_slot: &mut f64,
        var_dv0_dn1_slot: &mut f64,
        var_dv0_dn2_slot: &mut f64,
        var_dv0_dn3_slot: &mut f64,
        var_dv0_dn4_slot: &mut f64,
        var_dv0_dn5_slot: &mut f64,
        var_dv0_dn6_slot: &mut f64,
        var_dv0_dn7_slot: &mut f64,
        var_dv0_dn8_slot: &mut f64,
        var_dv0_dn9_slot: &mut f64,
        var_dv0_rdb0_slot: &mut f64,
        var_dv0_rdb1_slot: &mut f64,
        var_dv0_rdb2_slot: &mut f64,
        var_dv0_rdb3_slot: &mut f64,
        var_dv0_rdb4_slot: &mut f64,
        var_dv0_rdb5_slot: &mut f64,
        var_dv0_rdb6_slot: &mut f64,
        var_dv0_rdb7_slot: &mut f64,
        var_dv0_rdn0_slot: &mut f64,
        var_dv0_rdn1_slot: &mut f64,
        var_dv0_rdn2_slot: &mut f64,
        var_dv0_rdn3_slot: &mut f64,
        var_dv0_rdn4_slot: &mut f64,
        var_dv0_rdn5_slot: &mut f64,
        var_dv0_rdn6_slot: &mut f64,
        var_dv0_rdn7_slot: &mut f64,
        var_dv0_rdn8_slot: &mut f64,
        var_dv0_rdn9_slot: &mut f64,
        var_dv0_rv_slot: &mut f64,
        var_dvh_slot: &mut f64,
        var_dvh_db0_slot: &mut f64,
        var_dvh_db1_slot: &mut f64,
        var_dvh_db2_slot: &mut f64,
        var_dvh_db3_slot: &mut f64,
        var_dvh_db4_slot: &mut f64,
        var_dvh_db5_slot: &mut f64,
        var_dvh_db6_slot: &mut f64,
        var_dvh_db7_slot: &mut f64,
        var_dvh_dn0_slot: &mut f64,
        var_dvh_dn1_slot: &mut f64,
        var_dvh_dn2_slot: &mut f64,
        var_dvh_dn3_slot: &mut f64,
        var_dvh_dn4_slot: &mut f64,
        var_dvh_dn5_slot: &mut f64,
        var_dvh_dn6_slot: &mut f64,
        var_dvh_dn7_slot: &mut f64,
        var_dvh_dn8_slot: &mut f64,
        var_dvh_dn9_slot: &mut f64,
        var_dvh_rdb0_slot: &mut f64,
        var_dvh_rdb1_slot: &mut f64,
        var_dvh_rdb2_slot: &mut f64,
        var_dvh_rdb3_slot: &mut f64,
        var_dvh_rdb4_slot: &mut f64,
        var_dvh_rdb5_slot: &mut f64,
        var_dvh_rdb6_slot: &mut f64,
        var_dvh_rdb7_slot: &mut f64,
        var_dvh_rdn0_slot: &mut f64,
        var_dvh_rdn1_slot: &mut f64,
        var_dvh_rdn2_slot: &mut f64,
        var_dvh_rdn3_slot: &mut f64,
        var_dvh_rdn4_slot: &mut f64,
        var_dvh_rdn5_slot: &mut f64,
        var_dvh_rdn6_slot: &mut f64,
        var_dvh_rdn7_slot: &mut f64,
        var_dvh_rdn8_slot: &mut f64,
        var_dvh_rdn9_slot: &mut f64,
        var_dvh_rv_slot: &mut f64,
        var_guard13_slot: &mut f64,
        var_guard13_db0_slot: &mut f64,
        var_guard13_db1_slot: &mut f64,
        var_guard13_db2_slot: &mut f64,
        var_guard13_db3_slot: &mut f64,
        var_guard13_db4_slot: &mut f64,
        var_guard13_db5_slot: &mut f64,
        var_guard13_db6_slot: &mut f64,
        var_guard13_db7_slot: &mut f64,
        var_guard13_dn0_slot: &mut f64,
        var_guard13_dn1_slot: &mut f64,
        var_guard13_dn2_slot: &mut f64,
        var_guard13_dn3_slot: &mut f64,
        var_guard13_dn4_slot: &mut f64,
        var_guard13_dn5_slot: &mut f64,
        var_guard13_dn6_slot: &mut f64,
        var_guard13_dn7_slot: &mut f64,
        var_guard13_dn8_slot: &mut f64,
        var_guard13_dn9_slot: &mut f64,
        var_guard13_rdb0_slot: &mut f64,
        var_guard13_rdb1_slot: &mut f64,
        var_guard13_rdb2_slot: &mut f64,
        var_guard13_rdb3_slot: &mut f64,
        var_guard13_rdb4_slot: &mut f64,
        var_guard13_rdb5_slot: &mut f64,
        var_guard13_rdb6_slot: &mut f64,
        var_guard13_rdb7_slot: &mut f64,
        var_guard13_rdn0_slot: &mut f64,
        var_guard13_rdn1_slot: &mut f64,
        var_guard13_rdn2_slot: &mut f64,
        var_guard13_rdn3_slot: &mut f64,
        var_guard13_rdn4_slot: &mut f64,
        var_guard13_rdn5_slot: &mut f64,
        var_guard13_rdn6_slot: &mut f64,
        var_guard13_rdn7_slot: &mut f64,
        var_guard13_rdn8_slot: &mut f64,
        var_guard13_rdn9_slot: &mut f64,
        var_guard13_rv_slot: &mut f64,
        var_guard15_slot: &mut f64,
        var_guard15_db0_slot: &mut f64,
        var_guard15_db1_slot: &mut f64,
        var_guard15_db2_slot: &mut f64,
        var_guard15_db3_slot: &mut f64,
        var_guard15_db4_slot: &mut f64,
        var_guard15_db5_slot: &mut f64,
        var_guard15_db6_slot: &mut f64,
        var_guard15_db7_slot: &mut f64,
        var_guard15_dn0_slot: &mut f64,
        var_guard15_dn1_slot: &mut f64,
        var_guard15_dn2_slot: &mut f64,
        var_guard15_dn3_slot: &mut f64,
        var_guard15_dn4_slot: &mut f64,
        var_guard15_dn5_slot: &mut f64,
        var_guard15_dn6_slot: &mut f64,
        var_guard15_dn7_slot: &mut f64,
        var_guard15_dn8_slot: &mut f64,
        var_guard15_dn9_slot: &mut f64,
        var_guard15_rdb0_slot: &mut f64,
        var_guard15_rdb1_slot: &mut f64,
        var_guard15_rdb2_slot: &mut f64,
        var_guard15_rdb3_slot: &mut f64,
        var_guard15_rdb4_slot: &mut f64,
        var_guard15_rdb5_slot: &mut f64,
        var_guard15_rdb6_slot: &mut f64,
        var_guard15_rdb7_slot: &mut f64,
        var_guard15_rdn0_slot: &mut f64,
        var_guard15_rdn1_slot: &mut f64,
        var_guard15_rdn2_slot: &mut f64,
        var_guard15_rdn3_slot: &mut f64,
        var_guard15_rdn4_slot: &mut f64,
        var_guard15_rdn5_slot: &mut f64,
        var_guard15_rdn6_slot: &mut f64,
        var_guard15_rdn7_slot: &mut f64,
        var_guard15_rdn8_slot: &mut f64,
        var_guard15_rdn9_slot: &mut f64,
        var_guard15_rv_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_guard16_db0_slot: &mut f64,
        var_guard16_db1_slot: &mut f64,
        var_guard16_db2_slot: &mut f64,
        var_guard16_db3_slot: &mut f64,
        var_guard16_db4_slot: &mut f64,
        var_guard16_db5_slot: &mut f64,
        var_guard16_db6_slot: &mut f64,
        var_guard16_db7_slot: &mut f64,
        var_guard16_dn0_slot: &mut f64,
        var_guard16_dn1_slot: &mut f64,
        var_guard16_dn2_slot: &mut f64,
        var_guard16_dn3_slot: &mut f64,
        var_guard16_dn4_slot: &mut f64,
        var_guard16_dn5_slot: &mut f64,
        var_guard16_dn6_slot: &mut f64,
        var_guard16_dn7_slot: &mut f64,
        var_guard16_dn8_slot: &mut f64,
        var_guard16_dn9_slot: &mut f64,
        var_guard16_rdb0_slot: &mut f64,
        var_guard16_rdb1_slot: &mut f64,
        var_guard16_rdb2_slot: &mut f64,
        var_guard16_rdb3_slot: &mut f64,
        var_guard16_rdb4_slot: &mut f64,
        var_guard16_rdb5_slot: &mut f64,
        var_guard16_rdb6_slot: &mut f64,
        var_guard16_rdb7_slot: &mut f64,
        var_guard16_rdn0_slot: &mut f64,
        var_guard16_rdn1_slot: &mut f64,
        var_guard16_rdn2_slot: &mut f64,
        var_guard16_rdn3_slot: &mut f64,
        var_guard16_rdn4_slot: &mut f64,
        var_guard16_rdn5_slot: &mut f64,
        var_guard16_rdn6_slot: &mut f64,
        var_guard16_rdn7_slot: &mut f64,
        var_guard16_rdn8_slot: &mut f64,
        var_guard16_rdn9_slot: &mut f64,
        var_guard16_rv_slot: &mut f64,
        var_pwq_slot: &mut f64,
        var_pwq_db0_slot: &mut f64,
        var_pwq_db1_slot: &mut f64,
        var_pwq_db2_slot: &mut f64,
        var_pwq_db3_slot: &mut f64,
        var_pwq_db4_slot: &mut f64,
        var_pwq_db5_slot: &mut f64,
        var_pwq_db6_slot: &mut f64,
        var_pwq_db7_slot: &mut f64,
        var_pwq_dn0_slot: &mut f64,
        var_pwq_dn1_slot: &mut f64,
        var_pwq_dn2_slot: &mut f64,
        var_pwq_dn3_slot: &mut f64,
        var_pwq_dn4_slot: &mut f64,
        var_pwq_dn5_slot: &mut f64,
        var_pwq_dn6_slot: &mut f64,
        var_pwq_dn7_slot: &mut f64,
        var_pwq_dn8_slot: &mut f64,
        var_pwq_dn9_slot: &mut f64,
        var_pwq_rdb0_slot: &mut f64,
        var_pwq_rdb1_slot: &mut f64,
        var_pwq_rdb2_slot: &mut f64,
        var_pwq_rdb3_slot: &mut f64,
        var_pwq_rdb4_slot: &mut f64,
        var_pwq_rdb5_slot: &mut f64,
        var_pwq_rdb6_slot: &mut f64,
        var_pwq_rdb7_slot: &mut f64,
        var_pwq_rdn0_slot: &mut f64,
        var_pwq_rdn1_slot: &mut f64,
        var_pwq_rdn2_slot: &mut f64,
        var_pwq_rdn3_slot: &mut f64,
        var_pwq_rdn4_slot: &mut f64,
        var_pwq_rdn5_slot: &mut f64,
        var_pwq_rdn6_slot: &mut f64,
        var_pwq_rdn7_slot: &mut f64,
        var_pwq_rdn8_slot: &mut f64,
        var_pwq_rdn9_slot: &mut f64,
        var_pwq_rv_slot: &mut f64,
        var_qhi_slot: &mut f64,
        var_qhi_db0_slot: &mut f64,
        var_qhi_db1_slot: &mut f64,
        var_qhi_db2_slot: &mut f64,
        var_qhi_db3_slot: &mut f64,
        var_qhi_db4_slot: &mut f64,
        var_qhi_db5_slot: &mut f64,
        var_qhi_db6_slot: &mut f64,
        var_qhi_db7_slot: &mut f64,
        var_qhi_dn0_slot: &mut f64,
        var_qhi_dn1_slot: &mut f64,
        var_qhi_dn2_slot: &mut f64,
        var_qhi_dn3_slot: &mut f64,
        var_qhi_dn4_slot: &mut f64,
        var_qhi_dn5_slot: &mut f64,
        var_qhi_dn6_slot: &mut f64,
        var_qhi_dn7_slot: &mut f64,
        var_qhi_dn8_slot: &mut f64,
        var_qhi_dn9_slot: &mut f64,
        var_qhi_rdb0_slot: &mut f64,
        var_qhi_rdb1_slot: &mut f64,
        var_qhi_rdb2_slot: &mut f64,
        var_qhi_rdb3_slot: &mut f64,
        var_qhi_rdb4_slot: &mut f64,
        var_qhi_rdb5_slot: &mut f64,
        var_qhi_rdb6_slot: &mut f64,
        var_qhi_rdb7_slot: &mut f64,
        var_qhi_rdn0_slot: &mut f64,
        var_qhi_rdn1_slot: &mut f64,
        var_qhi_rdn2_slot: &mut f64,
        var_qhi_rdn3_slot: &mut f64,
        var_qhi_rdn4_slot: &mut f64,
        var_qhi_rdn5_slot: &mut f64,
        var_qhi_rdn6_slot: &mut f64,
        var_qhi_rdn7_slot: &mut f64,
        var_qhi_rdn8_slot: &mut f64,
        var_qhi_rdn9_slot: &mut f64,
        var_qhi_rv_slot: &mut f64,
        var_qjs_slot: &mut f64,
        var_qjs_db0_slot: &mut f64,
        var_qjs_db1_slot: &mut f64,
        var_qjs_db2_slot: &mut f64,
        var_qjs_db3_slot: &mut f64,
        var_qjs_db4_slot: &mut f64,
        var_qjs_db5_slot: &mut f64,
        var_qjs_db6_slot: &mut f64,
        var_qjs_db7_slot: &mut f64,
        var_qjs_dn0_slot: &mut f64,
        var_qjs_dn1_slot: &mut f64,
        var_qjs_dn2_slot: &mut f64,
        var_qjs_dn3_slot: &mut f64,
        var_qjs_dn4_slot: &mut f64,
        var_qjs_dn5_slot: &mut f64,
        var_qjs_dn6_slot: &mut f64,
        var_qjs_dn7_slot: &mut f64,
        var_qjs_dn8_slot: &mut f64,
        var_qjs_dn9_slot: &mut f64,
        var_qjs_rdb0_slot: &mut f64,
        var_qjs_rdb1_slot: &mut f64,
        var_qjs_rdb2_slot: &mut f64,
        var_qjs_rdb3_slot: &mut f64,
        var_qjs_rdb4_slot: &mut f64,
        var_qjs_rdb5_slot: &mut f64,
        var_qjs_rdb6_slot: &mut f64,
        var_qjs_rdb7_slot: &mut f64,
        var_qjs_rdn0_slot: &mut f64,
        var_qjs_rdn1_slot: &mut f64,
        var_qjs_rdn2_slot: &mut f64,
        var_qjs_rdn3_slot: &mut f64,
        var_qjs_rdn4_slot: &mut f64,
        var_qjs_rdn5_slot: &mut f64,
        var_qjs_rdn6_slot: &mut f64,
        var_qjs_rdn7_slot: &mut f64,
        var_qjs_rdn8_slot: &mut f64,
        var_qjs_rdn9_slot: &mut f64,
        var_qjs_rv_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo_db0_slot: &mut f64,
        var_qlo_db1_slot: &mut f64,
        var_qlo_db2_slot: &mut f64,
        var_qlo_db3_slot: &mut f64,
        var_qlo_db4_slot: &mut f64,
        var_qlo_db5_slot: &mut f64,
        var_qlo_db6_slot: &mut f64,
        var_qlo_db7_slot: &mut f64,
        var_qlo_dn0_slot: &mut f64,
        var_qlo_dn1_slot: &mut f64,
        var_qlo_dn2_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_qlo_dn5_slot: &mut f64,
        var_qlo_dn6_slot: &mut f64,
        var_qlo_dn7_slot: &mut f64,
        var_qlo_dn8_slot: &mut f64,
        var_qlo_dn9_slot: &mut f64,
        var_qlo_rdb0_slot: &mut f64,
        var_qlo_rdb1_slot: &mut f64,
        var_qlo_rdb2_slot: &mut f64,
        var_qlo_rdb3_slot: &mut f64,
        var_qlo_rdb4_slot: &mut f64,
        var_qlo_rdb5_slot: &mut f64,
        var_qlo_rdb6_slot: &mut f64,
        var_qlo_rdb7_slot: &mut f64,
        var_qlo_rdn0_slot: &mut f64,
        var_qlo_rdn1_slot: &mut f64,
        var_qlo_rdn2_slot: &mut f64,
        var_qlo_rdn3_slot: &mut f64,
        var_qlo_rdn4_slot: &mut f64,
        var_qlo_rdn5_slot: &mut f64,
        var_qlo_rdn6_slot: &mut f64,
        var_qlo_rdn7_slot: &mut f64,
        var_qlo_rdn8_slot: &mut f64,
        var_qlo_rdn9_slot: &mut f64,
        var_qlo_rv_slot: &mut f64,
    ) {
        let mut var_dv0: f64 = *var_dv0_slot;
        let mut var_dv0_db0: f64 = *var_dv0_db0_slot;
        let mut var_dv0_db1: f64 = *var_dv0_db1_slot;
        let mut var_dv0_db2: f64 = *var_dv0_db2_slot;
        let mut var_dv0_db3: f64 = *var_dv0_db3_slot;
        let mut var_dv0_db4: f64 = *var_dv0_db4_slot;
        let mut var_dv0_db5: f64 = *var_dv0_db5_slot;
        let mut var_dv0_db6: f64 = *var_dv0_db6_slot;
        let mut var_dv0_db7: f64 = *var_dv0_db7_slot;
        let mut var_dv0_dn0: f64 = *var_dv0_dn0_slot;
        let mut var_dv0_dn1: f64 = *var_dv0_dn1_slot;
        let mut var_dv0_dn2: f64 = *var_dv0_dn2_slot;
        let mut var_dv0_dn3: f64 = *var_dv0_dn3_slot;
        let mut var_dv0_dn4: f64 = *var_dv0_dn4_slot;
        let mut var_dv0_dn5: f64 = *var_dv0_dn5_slot;
        let mut var_dv0_dn6: f64 = *var_dv0_dn6_slot;
        let mut var_dv0_dn7: f64 = *var_dv0_dn7_slot;
        let mut var_dv0_dn8: f64 = *var_dv0_dn8_slot;
        let mut var_dv0_dn9: f64 = *var_dv0_dn9_slot;
        let mut var_dv0_rdb0: f64 = *var_dv0_rdb0_slot;
        let mut var_dv0_rdb1: f64 = *var_dv0_rdb1_slot;
        let mut var_dv0_rdb2: f64 = *var_dv0_rdb2_slot;
        let mut var_dv0_rdb3: f64 = *var_dv0_rdb3_slot;
        let mut var_dv0_rdb4: f64 = *var_dv0_rdb4_slot;
        let mut var_dv0_rdb5: f64 = *var_dv0_rdb5_slot;
        let mut var_dv0_rdb6: f64 = *var_dv0_rdb6_slot;
        let mut var_dv0_rdb7: f64 = *var_dv0_rdb7_slot;
        let mut var_dv0_rdn0: f64 = *var_dv0_rdn0_slot;
        let mut var_dv0_rdn1: f64 = *var_dv0_rdn1_slot;
        let mut var_dv0_rdn2: f64 = *var_dv0_rdn2_slot;
        let mut var_dv0_rdn3: f64 = *var_dv0_rdn3_slot;
        let mut var_dv0_rdn4: f64 = *var_dv0_rdn4_slot;
        let mut var_dv0_rdn5: f64 = *var_dv0_rdn5_slot;
        let mut var_dv0_rdn6: f64 = *var_dv0_rdn6_slot;
        let mut var_dv0_rdn7: f64 = *var_dv0_rdn7_slot;
        let mut var_dv0_rdn8: f64 = *var_dv0_rdn8_slot;
        let mut var_dv0_rdn9: f64 = *var_dv0_rdn9_slot;
        let mut var_dv0_rv: f64 = *var_dv0_rv_slot;
        let mut var_dvh: f64 = *var_dvh_slot;
        let mut var_dvh_db0: f64 = *var_dvh_db0_slot;
        let mut var_dvh_db1: f64 = *var_dvh_db1_slot;
        let mut var_dvh_db2: f64 = *var_dvh_db2_slot;
        let mut var_dvh_db3: f64 = *var_dvh_db3_slot;
        let mut var_dvh_db4: f64 = *var_dvh_db4_slot;
        let mut var_dvh_db5: f64 = *var_dvh_db5_slot;
        let mut var_dvh_db6: f64 = *var_dvh_db6_slot;
        let mut var_dvh_db7: f64 = *var_dvh_db7_slot;
        let mut var_dvh_dn0: f64 = *var_dvh_dn0_slot;
        let mut var_dvh_dn1: f64 = *var_dvh_dn1_slot;
        let mut var_dvh_dn2: f64 = *var_dvh_dn2_slot;
        let mut var_dvh_dn3: f64 = *var_dvh_dn3_slot;
        let mut var_dvh_dn4: f64 = *var_dvh_dn4_slot;
        let mut var_dvh_dn5: f64 = *var_dvh_dn5_slot;
        let mut var_dvh_dn6: f64 = *var_dvh_dn6_slot;
        let mut var_dvh_dn7: f64 = *var_dvh_dn7_slot;
        let mut var_dvh_dn8: f64 = *var_dvh_dn8_slot;
        let mut var_dvh_dn9: f64 = *var_dvh_dn9_slot;
        let mut var_dvh_rdb0: f64 = *var_dvh_rdb0_slot;
        let mut var_dvh_rdb1: f64 = *var_dvh_rdb1_slot;
        let mut var_dvh_rdb2: f64 = *var_dvh_rdb2_slot;
        let mut var_dvh_rdb3: f64 = *var_dvh_rdb3_slot;
        let mut var_dvh_rdb4: f64 = *var_dvh_rdb4_slot;
        let mut var_dvh_rdb5: f64 = *var_dvh_rdb5_slot;
        let mut var_dvh_rdb6: f64 = *var_dvh_rdb6_slot;
        let mut var_dvh_rdb7: f64 = *var_dvh_rdb7_slot;
        let mut var_dvh_rdn0: f64 = *var_dvh_rdn0_slot;
        let mut var_dvh_rdn1: f64 = *var_dvh_rdn1_slot;
        let mut var_dvh_rdn2: f64 = *var_dvh_rdn2_slot;
        let mut var_dvh_rdn3: f64 = *var_dvh_rdn3_slot;
        let mut var_dvh_rdn4: f64 = *var_dvh_rdn4_slot;
        let mut var_dvh_rdn5: f64 = *var_dvh_rdn5_slot;
        let mut var_dvh_rdn6: f64 = *var_dvh_rdn6_slot;
        let mut var_dvh_rdn7: f64 = *var_dvh_rdn7_slot;
        let mut var_dvh_rdn8: f64 = *var_dvh_rdn8_slot;
        let mut var_dvh_rdn9: f64 = *var_dvh_rdn9_slot;
        let mut var_dvh_rv: f64 = *var_dvh_rv_slot;
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard13_db0: f64 = *var_guard13_db0_slot;
        let mut var_guard13_db1: f64 = *var_guard13_db1_slot;
        let mut var_guard13_db2: f64 = *var_guard13_db2_slot;
        let mut var_guard13_db3: f64 = *var_guard13_db3_slot;
        let mut var_guard13_db4: f64 = *var_guard13_db4_slot;
        let mut var_guard13_db5: f64 = *var_guard13_db5_slot;
        let mut var_guard13_db6: f64 = *var_guard13_db6_slot;
        let mut var_guard13_db7: f64 = *var_guard13_db7_slot;
        let mut var_guard13_dn0: f64 = *var_guard13_dn0_slot;
        let mut var_guard13_dn1: f64 = *var_guard13_dn1_slot;
        let mut var_guard13_dn2: f64 = *var_guard13_dn2_slot;
        let mut var_guard13_dn3: f64 = *var_guard13_dn3_slot;
        let mut var_guard13_dn4: f64 = *var_guard13_dn4_slot;
        let mut var_guard13_dn5: f64 = *var_guard13_dn5_slot;
        let mut var_guard13_dn6: f64 = *var_guard13_dn6_slot;
        let mut var_guard13_dn7: f64 = *var_guard13_dn7_slot;
        let mut var_guard13_dn8: f64 = *var_guard13_dn8_slot;
        let mut var_guard13_dn9: f64 = *var_guard13_dn9_slot;
        let mut var_guard13_rdb0: f64 = *var_guard13_rdb0_slot;
        let mut var_guard13_rdb1: f64 = *var_guard13_rdb1_slot;
        let mut var_guard13_rdb2: f64 = *var_guard13_rdb2_slot;
        let mut var_guard13_rdb3: f64 = *var_guard13_rdb3_slot;
        let mut var_guard13_rdb4: f64 = *var_guard13_rdb4_slot;
        let mut var_guard13_rdb5: f64 = *var_guard13_rdb5_slot;
        let mut var_guard13_rdb6: f64 = *var_guard13_rdb6_slot;
        let mut var_guard13_rdb7: f64 = *var_guard13_rdb7_slot;
        let mut var_guard13_rdn0: f64 = *var_guard13_rdn0_slot;
        let mut var_guard13_rdn1: f64 = *var_guard13_rdn1_slot;
        let mut var_guard13_rdn2: f64 = *var_guard13_rdn2_slot;
        let mut var_guard13_rdn3: f64 = *var_guard13_rdn3_slot;
        let mut var_guard13_rdn4: f64 = *var_guard13_rdn4_slot;
        let mut var_guard13_rdn5: f64 = *var_guard13_rdn5_slot;
        let mut var_guard13_rdn6: f64 = *var_guard13_rdn6_slot;
        let mut var_guard13_rdn7: f64 = *var_guard13_rdn7_slot;
        let mut var_guard13_rdn8: f64 = *var_guard13_rdn8_slot;
        let mut var_guard13_rdn9: f64 = *var_guard13_rdn9_slot;
        let mut var_guard13_rv: f64 = *var_guard13_rv_slot;
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_guard15_db0: f64 = *var_guard15_db0_slot;
        let mut var_guard15_db1: f64 = *var_guard15_db1_slot;
        let mut var_guard15_db2: f64 = *var_guard15_db2_slot;
        let mut var_guard15_db3: f64 = *var_guard15_db3_slot;
        let mut var_guard15_db4: f64 = *var_guard15_db4_slot;
        let mut var_guard15_db5: f64 = *var_guard15_db5_slot;
        let mut var_guard15_db6: f64 = *var_guard15_db6_slot;
        let mut var_guard15_db7: f64 = *var_guard15_db7_slot;
        let mut var_guard15_dn0: f64 = *var_guard15_dn0_slot;
        let mut var_guard15_dn1: f64 = *var_guard15_dn1_slot;
        let mut var_guard15_dn2: f64 = *var_guard15_dn2_slot;
        let mut var_guard15_dn3: f64 = *var_guard15_dn3_slot;
        let mut var_guard15_dn4: f64 = *var_guard15_dn4_slot;
        let mut var_guard15_dn5: f64 = *var_guard15_dn5_slot;
        let mut var_guard15_dn6: f64 = *var_guard15_dn6_slot;
        let mut var_guard15_dn7: f64 = *var_guard15_dn7_slot;
        let mut var_guard15_dn8: f64 = *var_guard15_dn8_slot;
        let mut var_guard15_dn9: f64 = *var_guard15_dn9_slot;
        let mut var_guard15_rdb0: f64 = *var_guard15_rdb0_slot;
        let mut var_guard15_rdb1: f64 = *var_guard15_rdb1_slot;
        let mut var_guard15_rdb2: f64 = *var_guard15_rdb2_slot;
        let mut var_guard15_rdb3: f64 = *var_guard15_rdb3_slot;
        let mut var_guard15_rdb4: f64 = *var_guard15_rdb4_slot;
        let mut var_guard15_rdb5: f64 = *var_guard15_rdb5_slot;
        let mut var_guard15_rdb6: f64 = *var_guard15_rdb6_slot;
        let mut var_guard15_rdb7: f64 = *var_guard15_rdb7_slot;
        let mut var_guard15_rdn0: f64 = *var_guard15_rdn0_slot;
        let mut var_guard15_rdn1: f64 = *var_guard15_rdn1_slot;
        let mut var_guard15_rdn2: f64 = *var_guard15_rdn2_slot;
        let mut var_guard15_rdn3: f64 = *var_guard15_rdn3_slot;
        let mut var_guard15_rdn4: f64 = *var_guard15_rdn4_slot;
        let mut var_guard15_rdn5: f64 = *var_guard15_rdn5_slot;
        let mut var_guard15_rdn6: f64 = *var_guard15_rdn6_slot;
        let mut var_guard15_rdn7: f64 = *var_guard15_rdn7_slot;
        let mut var_guard15_rdn8: f64 = *var_guard15_rdn8_slot;
        let mut var_guard15_rdn9: f64 = *var_guard15_rdn9_slot;
        let mut var_guard15_rv: f64 = *var_guard15_rv_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard16_db0: f64 = *var_guard16_db0_slot;
        let mut var_guard16_db1: f64 = *var_guard16_db1_slot;
        let mut var_guard16_db2: f64 = *var_guard16_db2_slot;
        let mut var_guard16_db3: f64 = *var_guard16_db3_slot;
        let mut var_guard16_db4: f64 = *var_guard16_db4_slot;
        let mut var_guard16_db5: f64 = *var_guard16_db5_slot;
        let mut var_guard16_db6: f64 = *var_guard16_db6_slot;
        let mut var_guard16_db7: f64 = *var_guard16_db7_slot;
        let mut var_guard16_dn0: f64 = *var_guard16_dn0_slot;
        let mut var_guard16_dn1: f64 = *var_guard16_dn1_slot;
        let mut var_guard16_dn2: f64 = *var_guard16_dn2_slot;
        let mut var_guard16_dn3: f64 = *var_guard16_dn3_slot;
        let mut var_guard16_dn4: f64 = *var_guard16_dn4_slot;
        let mut var_guard16_dn5: f64 = *var_guard16_dn5_slot;
        let mut var_guard16_dn6: f64 = *var_guard16_dn6_slot;
        let mut var_guard16_dn7: f64 = *var_guard16_dn7_slot;
        let mut var_guard16_dn8: f64 = *var_guard16_dn8_slot;
        let mut var_guard16_dn9: f64 = *var_guard16_dn9_slot;
        let mut var_guard16_rdb0: f64 = *var_guard16_rdb0_slot;
        let mut var_guard16_rdb1: f64 = *var_guard16_rdb1_slot;
        let mut var_guard16_rdb2: f64 = *var_guard16_rdb2_slot;
        let mut var_guard16_rdb3: f64 = *var_guard16_rdb3_slot;
        let mut var_guard16_rdb4: f64 = *var_guard16_rdb4_slot;
        let mut var_guard16_rdb5: f64 = *var_guard16_rdb5_slot;
        let mut var_guard16_rdb6: f64 = *var_guard16_rdb6_slot;
        let mut var_guard16_rdb7: f64 = *var_guard16_rdb7_slot;
        let mut var_guard16_rdn0: f64 = *var_guard16_rdn0_slot;
        let mut var_guard16_rdn1: f64 = *var_guard16_rdn1_slot;
        let mut var_guard16_rdn2: f64 = *var_guard16_rdn2_slot;
        let mut var_guard16_rdn3: f64 = *var_guard16_rdn3_slot;
        let mut var_guard16_rdn4: f64 = *var_guard16_rdn4_slot;
        let mut var_guard16_rdn5: f64 = *var_guard16_rdn5_slot;
        let mut var_guard16_rdn6: f64 = *var_guard16_rdn6_slot;
        let mut var_guard16_rdn7: f64 = *var_guard16_rdn7_slot;
        let mut var_guard16_rdn8: f64 = *var_guard16_rdn8_slot;
        let mut var_guard16_rdn9: f64 = *var_guard16_rdn9_slot;
        let mut var_guard16_rv: f64 = *var_guard16_rv_slot;
        let mut var_pwq: f64 = *var_pwq_slot;
        let mut var_pwq_db0: f64 = *var_pwq_db0_slot;
        let mut var_pwq_db1: f64 = *var_pwq_db1_slot;
        let mut var_pwq_db2: f64 = *var_pwq_db2_slot;
        let mut var_pwq_db3: f64 = *var_pwq_db3_slot;
        let mut var_pwq_db4: f64 = *var_pwq_db4_slot;
        let mut var_pwq_db5: f64 = *var_pwq_db5_slot;
        let mut var_pwq_db6: f64 = *var_pwq_db6_slot;
        let mut var_pwq_db7: f64 = *var_pwq_db7_slot;
        let mut var_pwq_dn0: f64 = *var_pwq_dn0_slot;
        let mut var_pwq_dn1: f64 = *var_pwq_dn1_slot;
        let mut var_pwq_dn2: f64 = *var_pwq_dn2_slot;
        let mut var_pwq_dn3: f64 = *var_pwq_dn3_slot;
        let mut var_pwq_dn4: f64 = *var_pwq_dn4_slot;
        let mut var_pwq_dn5: f64 = *var_pwq_dn5_slot;
        let mut var_pwq_dn6: f64 = *var_pwq_dn6_slot;
        let mut var_pwq_dn7: f64 = *var_pwq_dn7_slot;
        let mut var_pwq_dn8: f64 = *var_pwq_dn8_slot;
        let mut var_pwq_dn9: f64 = *var_pwq_dn9_slot;
        let mut var_pwq_rdb0: f64 = *var_pwq_rdb0_slot;
        let mut var_pwq_rdb1: f64 = *var_pwq_rdb1_slot;
        let mut var_pwq_rdb2: f64 = *var_pwq_rdb2_slot;
        let mut var_pwq_rdb3: f64 = *var_pwq_rdb3_slot;
        let mut var_pwq_rdb4: f64 = *var_pwq_rdb4_slot;
        let mut var_pwq_rdb5: f64 = *var_pwq_rdb5_slot;
        let mut var_pwq_rdb6: f64 = *var_pwq_rdb6_slot;
        let mut var_pwq_rdb7: f64 = *var_pwq_rdb7_slot;
        let mut var_pwq_rdn0: f64 = *var_pwq_rdn0_slot;
        let mut var_pwq_rdn1: f64 = *var_pwq_rdn1_slot;
        let mut var_pwq_rdn2: f64 = *var_pwq_rdn2_slot;
        let mut var_pwq_rdn3: f64 = *var_pwq_rdn3_slot;
        let mut var_pwq_rdn4: f64 = *var_pwq_rdn4_slot;
        let mut var_pwq_rdn5: f64 = *var_pwq_rdn5_slot;
        let mut var_pwq_rdn6: f64 = *var_pwq_rdn6_slot;
        let mut var_pwq_rdn7: f64 = *var_pwq_rdn7_slot;
        let mut var_pwq_rdn8: f64 = *var_pwq_rdn8_slot;
        let mut var_pwq_rdn9: f64 = *var_pwq_rdn9_slot;
        let mut var_pwq_rv: f64 = *var_pwq_rv_slot;
        let mut var_qhi: f64 = *var_qhi_slot;
        let mut var_qhi_db0: f64 = *var_qhi_db0_slot;
        let mut var_qhi_db1: f64 = *var_qhi_db1_slot;
        let mut var_qhi_db2: f64 = *var_qhi_db2_slot;
        let mut var_qhi_db3: f64 = *var_qhi_db3_slot;
        let mut var_qhi_db4: f64 = *var_qhi_db4_slot;
        let mut var_qhi_db5: f64 = *var_qhi_db5_slot;
        let mut var_qhi_db6: f64 = *var_qhi_db6_slot;
        let mut var_qhi_db7: f64 = *var_qhi_db7_slot;
        let mut var_qhi_dn0: f64 = *var_qhi_dn0_slot;
        let mut var_qhi_dn1: f64 = *var_qhi_dn1_slot;
        let mut var_qhi_dn2: f64 = *var_qhi_dn2_slot;
        let mut var_qhi_dn3: f64 = *var_qhi_dn3_slot;
        let mut var_qhi_dn4: f64 = *var_qhi_dn4_slot;
        let mut var_qhi_dn5: f64 = *var_qhi_dn5_slot;
        let mut var_qhi_dn6: f64 = *var_qhi_dn6_slot;
        let mut var_qhi_dn7: f64 = *var_qhi_dn7_slot;
        let mut var_qhi_dn8: f64 = *var_qhi_dn8_slot;
        let mut var_qhi_dn9: f64 = *var_qhi_dn9_slot;
        let mut var_qhi_rdb0: f64 = *var_qhi_rdb0_slot;
        let mut var_qhi_rdb1: f64 = *var_qhi_rdb1_slot;
        let mut var_qhi_rdb2: f64 = *var_qhi_rdb2_slot;
        let mut var_qhi_rdb3: f64 = *var_qhi_rdb3_slot;
        let mut var_qhi_rdb4: f64 = *var_qhi_rdb4_slot;
        let mut var_qhi_rdb5: f64 = *var_qhi_rdb5_slot;
        let mut var_qhi_rdb6: f64 = *var_qhi_rdb6_slot;
        let mut var_qhi_rdb7: f64 = *var_qhi_rdb7_slot;
        let mut var_qhi_rdn0: f64 = *var_qhi_rdn0_slot;
        let mut var_qhi_rdn1: f64 = *var_qhi_rdn1_slot;
        let mut var_qhi_rdn2: f64 = *var_qhi_rdn2_slot;
        let mut var_qhi_rdn3: f64 = *var_qhi_rdn3_slot;
        let mut var_qhi_rdn4: f64 = *var_qhi_rdn4_slot;
        let mut var_qhi_rdn5: f64 = *var_qhi_rdn5_slot;
        let mut var_qhi_rdn6: f64 = *var_qhi_rdn6_slot;
        let mut var_qhi_rdn7: f64 = *var_qhi_rdn7_slot;
        let mut var_qhi_rdn8: f64 = *var_qhi_rdn8_slot;
        let mut var_qhi_rdn9: f64 = *var_qhi_rdn9_slot;
        let mut var_qhi_rv: f64 = *var_qhi_rv_slot;
        let mut var_qjs: f64 = *var_qjs_slot;
        let mut var_qjs_db0: f64 = *var_qjs_db0_slot;
        let mut var_qjs_db1: f64 = *var_qjs_db1_slot;
        let mut var_qjs_db2: f64 = *var_qjs_db2_slot;
        let mut var_qjs_db3: f64 = *var_qjs_db3_slot;
        let mut var_qjs_db4: f64 = *var_qjs_db4_slot;
        let mut var_qjs_db5: f64 = *var_qjs_db5_slot;
        let mut var_qjs_db6: f64 = *var_qjs_db6_slot;
        let mut var_qjs_db7: f64 = *var_qjs_db7_slot;
        let mut var_qjs_dn0: f64 = *var_qjs_dn0_slot;
        let mut var_qjs_dn1: f64 = *var_qjs_dn1_slot;
        let mut var_qjs_dn2: f64 = *var_qjs_dn2_slot;
        let mut var_qjs_dn3: f64 = *var_qjs_dn3_slot;
        let mut var_qjs_dn4: f64 = *var_qjs_dn4_slot;
        let mut var_qjs_dn5: f64 = *var_qjs_dn5_slot;
        let mut var_qjs_dn6: f64 = *var_qjs_dn6_slot;
        let mut var_qjs_dn7: f64 = *var_qjs_dn7_slot;
        let mut var_qjs_dn8: f64 = *var_qjs_dn8_slot;
        let mut var_qjs_dn9: f64 = *var_qjs_dn9_slot;
        let mut var_qjs_rdb0: f64 = *var_qjs_rdb0_slot;
        let mut var_qjs_rdb1: f64 = *var_qjs_rdb1_slot;
        let mut var_qjs_rdb2: f64 = *var_qjs_rdb2_slot;
        let mut var_qjs_rdb3: f64 = *var_qjs_rdb3_slot;
        let mut var_qjs_rdb4: f64 = *var_qjs_rdb4_slot;
        let mut var_qjs_rdb5: f64 = *var_qjs_rdb5_slot;
        let mut var_qjs_rdb6: f64 = *var_qjs_rdb6_slot;
        let mut var_qjs_rdb7: f64 = *var_qjs_rdb7_slot;
        let mut var_qjs_rdn0: f64 = *var_qjs_rdn0_slot;
        let mut var_qjs_rdn1: f64 = *var_qjs_rdn1_slot;
        let mut var_qjs_rdn2: f64 = *var_qjs_rdn2_slot;
        let mut var_qjs_rdn3: f64 = *var_qjs_rdn3_slot;
        let mut var_qjs_rdn4: f64 = *var_qjs_rdn4_slot;
        let mut var_qjs_rdn5: f64 = *var_qjs_rdn5_slot;
        let mut var_qjs_rdn6: f64 = *var_qjs_rdn6_slot;
        let mut var_qjs_rdn7: f64 = *var_qjs_rdn7_slot;
        let mut var_qjs_rdn8: f64 = *var_qjs_rdn8_slot;
        let mut var_qjs_rdn9: f64 = *var_qjs_rdn9_slot;
        let mut var_qjs_rv: f64 = *var_qjs_rv_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo_db0: f64 = *var_qlo_db0_slot;
        let mut var_qlo_db1: f64 = *var_qlo_db1_slot;
        let mut var_qlo_db2: f64 = *var_qlo_db2_slot;
        let mut var_qlo_db3: f64 = *var_qlo_db3_slot;
        let mut var_qlo_db4: f64 = *var_qlo_db4_slot;
        let mut var_qlo_db5: f64 = *var_qlo_db5_slot;
        let mut var_qlo_db6: f64 = *var_qlo_db6_slot;
        let mut var_qlo_db7: f64 = *var_qlo_db7_slot;
        let mut var_qlo_dn0: f64 = *var_qlo_dn0_slot;
        let mut var_qlo_dn1: f64 = *var_qlo_dn1_slot;
        let mut var_qlo_dn2: f64 = *var_qlo_dn2_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_qlo_dn5: f64 = *var_qlo_dn5_slot;
        let mut var_qlo_dn6: f64 = *var_qlo_dn6_slot;
        let mut var_qlo_dn7: f64 = *var_qlo_dn7_slot;
        let mut var_qlo_dn8: f64 = *var_qlo_dn8_slot;
        let mut var_qlo_dn9: f64 = *var_qlo_dn9_slot;
        let mut var_qlo_rdb0: f64 = *var_qlo_rdb0_slot;
        let mut var_qlo_rdb1: f64 = *var_qlo_rdb1_slot;
        let mut var_qlo_rdb2: f64 = *var_qlo_rdb2_slot;
        let mut var_qlo_rdb3: f64 = *var_qlo_rdb3_slot;
        let mut var_qlo_rdb4: f64 = *var_qlo_rdb4_slot;
        let mut var_qlo_rdb5: f64 = *var_qlo_rdb5_slot;
        let mut var_qlo_rdb6: f64 = *var_qlo_rdb6_slot;
        let mut var_qlo_rdb7: f64 = *var_qlo_rdb7_slot;
        let mut var_qlo_rdn0: f64 = *var_qlo_rdn0_slot;
        let mut var_qlo_rdn1: f64 = *var_qlo_rdn1_slot;
        let mut var_qlo_rdn2: f64 = *var_qlo_rdn2_slot;
        let mut var_qlo_rdn3: f64 = *var_qlo_rdn3_slot;
        let mut var_qlo_rdn4: f64 = *var_qlo_rdn4_slot;
        let mut var_qlo_rdn5: f64 = *var_qlo_rdn5_slot;
        let mut var_qlo_rdn6: f64 = *var_qlo_rdn6_slot;
        let mut var_qlo_rdn7: f64 = *var_qlo_rdn7_slot;
        let mut var_qlo_rdn8: f64 = *var_qlo_rdn8_slot;
        let mut var_qlo_rdn9: f64 = *var_qlo_rdn9_slot;
        let mut var_qlo_rv: f64 = *var_qlo_rv_slot;

        let assign1500_e1810: f64 = if p.p32 == 1.0 { 1.0 } else { 0.0 };
        var_guard13 = assign1500_e1810;
        var_guard13_dn0 = 0.0;
        var_guard13_dn1 = 0.0;
        var_guard13_dn2 = 0.0;
        var_guard13_dn3 = 0.0;
        var_guard13_dn4 = 0.0;
        var_guard13_dn5 = 0.0;
        var_guard13_dn6 = 0.0;
        var_guard13_dn7 = 0.0;
        var_guard13_dn8 = 0.0;
        var_guard13_dn9 = 0.0;
        var_guard13_db0 = 0.0;
        var_guard13_db1 = 0.0;
        var_guard13_db2 = 0.0;
        var_guard13_db3 = 0.0;
        var_guard13_db4 = 0.0;
        var_guard13_db5 = 0.0;
        var_guard13_db6 = 0.0;
        var_guard13_db7 = 0.0;
        var_guard13_rv = 0.0;
        var_guard13_rdn0 = 0.0;
        var_guard13_rdn1 = 0.0;
        var_guard13_rdn2 = 0.0;
        var_guard13_rdn3 = 0.0;
        var_guard13_rdn4 = 0.0;
        var_guard13_rdn5 = 0.0;
        var_guard13_rdn6 = 0.0;
        var_guard13_rdn7 = 0.0;
        var_guard13_rdn8 = 0.0;
        var_guard13_rdn9 = 0.0;
        var_guard13_rdb0 = 0.0;
        var_guard13_rdb1 = 0.0;
        var_guard13_rdb2 = 0.0;
        var_guard13_rdb3 = 0.0;
        var_guard13_rdb4 = 0.0;
        var_guard13_rdb5 = 0.0;
        var_guard13_rdb6 = 0.0;
        var_guard13_rdb7 = 0.0;

        let assign1570_e1852: f64 = if var_veci <= 0.0 { 1.0 } else { 0.0 };
        var_guard15 = assign1570_e1852;
        var_guard15_dn0 = 0.0;
        var_guard15_dn1 = 0.0;
        var_guard15_dn2 = 0.0;
        var_guard15_dn3 = 0.0;
        var_guard15_dn4 = 0.0;
        var_guard15_dn5 = 0.0;
        var_guard15_dn6 = 0.0;
        var_guard15_dn7 = 0.0;
        var_guard15_dn8 = 0.0;
        var_guard15_dn9 = 0.0;
        var_guard15_db0 = 0.0;
        var_guard15_db1 = 0.0;
        var_guard15_db2 = 0.0;
        var_guard15_db3 = 0.0;
        var_guard15_db4 = 0.0;
        var_guard15_db5 = 0.0;
        var_guard15_db6 = 0.0;
        var_guard15_db7 = 0.0;
        var_guard15_rv = 0.0;
        var_guard15_rdn0 = 0.0;
        var_guard15_rdn1 = 0.0;
        var_guard15_rdn2 = 0.0;
        var_guard15_rdn3 = 0.0;
        var_guard15_rdn4 = 0.0;
        var_guard15_rdn5 = 0.0;
        var_guard15_rdn6 = 0.0;
        var_guard15_rdn7 = 0.0;
        var_guard15_rdn8 = 0.0;
        var_guard15_rdn9 = 0.0;
        var_guard15_rdb0 = 0.0;
        var_guard15_rdb1 = 0.0;
        var_guard15_rdb2 = 0.0;
        var_guard15_rdb3 = 0.0;
        var_guard15_rdb4 = 0.0;
        var_guard15_rdb5 = 0.0;
        var_guard15_rdb6 = 0.0;
        var_guard15_rdb7 = 0.0;

        let (assign1580_e1876, assign1580_e1876_d_n0, assign1580_e1876_d_n1, assign1580_e1876_d_n2, assign1580_e1876_d_n3, assign1580_e1876_d_n4, assign1580_e1876_d_n5, assign1580_e1876_d_n6, assign1580_e1876_d_n7, assign1580_e1876_d_n8, assign1580_e1876_d_n9, assign1580_e1876_d_b0, assign1580_e1876_d_b1, assign1580_e1876_d_b2, assign1580_e1876_d_b3, assign1580_e1876_d_b4, assign1580_e1876_d_b5, assign1580_e1876_d_b6, assign1580_e1876_d_b7,) = {
    if (var_guard15 != 0.0) {
        let assign1580_e1856: f64 = (var_cjs_t * var_vjs_t);
        let assign1580_e1860: f64 = (1.0 - p.p76);
        let assign1580_e1864: f64 = (var_veci / var_vjs_t);
        let assign1580_e1865: f64 = (1.0 - assign1580_e1864);
        let assign1580_e1866: f64 = (assign1580_e1865).ln();
        let assign1580_e1867: f64 = (assign1580_e1860 * assign1580_e1866);
        let assign1580_e1868: f64 = (assign1580_e1867).exp();
        let assign1580_e1869: f64 = (1.0 - assign1580_e1868);
        let assign1580_e1870: f64 = (assign1580_e1856 * assign1580_e1869);
        let assign1580_e1873: f64 = (1.0 - p.p76);
        let assign1580_e1874: f64 = (assign1580_e1870 / assign1580_e1873);
        (assign1580_e1874, (((((var_cjs_t_dn0 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn0)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn0 * var_vjs_t) - (var_veci * var_vjs_t_dn0)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn1 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn1)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn1 * var_vjs_t) - (var_veci * var_vjs_t_dn1)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn2 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn2)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn2 * var_vjs_t) - (var_veci * var_vjs_t_dn2)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn3 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn3)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn3 * var_vjs_t) - (var_veci * var_vjs_t_dn3)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn4 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn4)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn4 * var_vjs_t) - (var_veci * var_vjs_t_dn4)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn5 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn5)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn5 * var_vjs_t) - (var_veci * var_vjs_t_dn5)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn6 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn6)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn6 * var_vjs_t) - (var_veci * var_vjs_t_dn6)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn7 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn7)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn7 * var_vjs_t) - (var_veci * var_vjs_t_dn7)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn8 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn8)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn8 * var_vjs_t) - (var_veci * var_vjs_t_dn8)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn9 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn9)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn9 * var_vjs_t) - (var_veci * var_vjs_t_dn9)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db0 * var_vjs_t) + (var_cjs_t * var_vjs_t_db0)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db0 * var_vjs_t) - (var_veci * var_vjs_t_db0)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db1 * var_vjs_t) + (var_cjs_t * var_vjs_t_db1)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db1 * var_vjs_t) - (var_veci * var_vjs_t_db1)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db2 * var_vjs_t) + (var_cjs_t * var_vjs_t_db2)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db2 * var_vjs_t) - (var_veci * var_vjs_t_db2)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db3 * var_vjs_t) + (var_cjs_t * var_vjs_t_db3)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db3 * var_vjs_t) - (var_veci * var_vjs_t_db3)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db4 * var_vjs_t) + (var_cjs_t * var_vjs_t_db4)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db4 * var_vjs_t) - (var_veci * var_vjs_t_db4)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db5 * var_vjs_t) + (var_cjs_t * var_vjs_t_db5)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db5 * var_vjs_t) - (var_veci * var_vjs_t_db5)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db6 * var_vjs_t) + (var_cjs_t * var_vjs_t_db6)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db6 * var_vjs_t) - (var_veci * var_vjs_t_db6)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db7 * var_vjs_t) + (var_cjs_t * var_vjs_t_db7)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db7 * var_vjs_t) - (var_veci * var_vjs_t_db7)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873),)
    } else {
        (var_qjs, var_qjs_dn0, var_qjs_dn1, var_qjs_dn2, var_qjs_dn3, var_qjs_dn4, var_qjs_dn5, var_qjs_dn6, var_qjs_dn7, var_qjs_dn8, var_qjs_dn9, var_qjs_db0, var_qjs_db1, var_qjs_db2, var_qjs_db3, var_qjs_db4, var_qjs_db5, var_qjs_db6, var_qjs_db7,)
    }
};
        var_qjs = assign1580_e1876;
        var_qjs_dn0 = assign1580_e1876_d_n0;
        var_qjs_dn1 = assign1580_e1876_d_n1;
        var_qjs_dn2 = assign1580_e1876_d_n2;
        var_qjs_dn3 = assign1580_e1876_d_n3;
        var_qjs_dn4 = assign1580_e1876_d_n4;
        var_qjs_dn5 = assign1580_e1876_d_n5;
        var_qjs_dn6 = assign1580_e1876_d_n6;
        var_qjs_dn7 = assign1580_e1876_d_n7;
        var_qjs_dn8 = assign1580_e1876_d_n8;
        var_qjs_dn9 = assign1580_e1876_d_n9;
        var_qjs_db0 = assign1580_e1876_d_b0;
        var_qjs_db1 = assign1580_e1876_d_b1;
        var_qjs_db2 = assign1580_e1876_d_b2;
        var_qjs_db3 = assign1580_e1876_d_b3;
        var_qjs_db4 = assign1580_e1876_d_b4;
        var_qjs_db5 = assign1580_e1876_d_b5;
        var_qjs_db6 = assign1580_e1876_d_b6;
        var_qjs_db7 = assign1580_e1876_d_b7;
        var_qjs_rv = 0.0;
        var_qjs_rdn0 = 0.0;
        var_qjs_rdn1 = 0.0;
        var_qjs_rdn2 = 0.0;
        var_qjs_rdn3 = 0.0;
        var_qjs_rdn4 = 0.0;
        var_qjs_rdn5 = 0.0;
        var_qjs_rdn6 = 0.0;
        var_qjs_rdn7 = 0.0;
        var_qjs_rdn8 = 0.0;
        var_qjs_rdn9 = 0.0;
        var_qjs_rdb0 = 0.0;
        var_qjs_rdb1 = 0.0;
        var_qjs_rdb2 = 0.0;
        var_qjs_rdb3 = 0.0;
        var_qjs_rdb4 = 0.0;
        var_qjs_rdb5 = 0.0;
        var_qjs_rdb6 = 0.0;
        var_qjs_rdb7 = 0.0;

        let (assign1590_e1893, assign1590_e1893_d_n0, assign1590_e1893_d_n1, assign1590_e1893_d_n2, assign1590_e1893_d_n3, assign1590_e1893_d_n4, assign1590_e1893_d_n5, assign1590_e1893_d_n6, assign1590_e1893_d_n7, assign1590_e1893_d_n8, assign1590_e1893_d_n9, assign1590_e1893_d_b0, assign1590_e1893_d_b1, assign1590_e1893_d_b2, assign1590_e1893_d_b3, assign1590_e1893_d_b4, assign1590_e1893_d_b5, assign1590_e1893_d_b6, assign1590_e1893_d_b7,) = {
    if (var_guard15 == 0.0) {
        let assign1590_e1881: f64 = (var_cjs_t * var_veci);
        let assign1590_e1885: f64 = (0.5 * p.p76);
        let assign1590_e1887: f64 = (assign1590_e1885 * var_veci);
        let assign1590_e1889: f64 = (assign1590_e1887 / var_vjs_t);
        let assign1590_e1890: f64 = (1.0 + assign1590_e1889);
        let assign1590_e1891: f64 = (assign1590_e1881 * assign1590_e1890);
        (assign1590_e1891, ((((var_cjs_t_dn0 * var_veci) + (var_cjs_t * var_veci_dn0)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn0) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn0)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn1 * var_veci) + (var_cjs_t * var_veci_dn1)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn1) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn1)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn2 * var_veci) + (var_cjs_t * var_veci_dn2)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn2) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn2)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn3 * var_veci) + (var_cjs_t * var_veci_dn3)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn3) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn3)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn4 * var_veci) + (var_cjs_t * var_veci_dn4)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn4) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn4)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn5 * var_veci) + (var_cjs_t * var_veci_dn5)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn5) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn5)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn6 * var_veci) + (var_cjs_t * var_veci_dn6)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn6) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn6)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn7 * var_veci) + (var_cjs_t * var_veci_dn7)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn7) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn7)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn8 * var_veci) + (var_cjs_t * var_veci_dn8)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn8) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn8)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn9 * var_veci) + (var_cjs_t * var_veci_dn9)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn9) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn9)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db0 * var_veci) + (var_cjs_t * var_veci_db0)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db0) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db0)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db1 * var_veci) + (var_cjs_t * var_veci_db1)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db1) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db1)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db2 * var_veci) + (var_cjs_t * var_veci_db2)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db2) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db2)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db3 * var_veci) + (var_cjs_t * var_veci_db3)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db3) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db3)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db4 * var_veci) + (var_cjs_t * var_veci_db4)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db4) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db4)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db5 * var_veci) + (var_cjs_t * var_veci_db5)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db5) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db5)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db6 * var_veci) + (var_cjs_t * var_veci_db6)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db6) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db6)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db7 * var_veci) + (var_cjs_t * var_veci_db7)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db7) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db7)) / (var_vjs_t * var_vjs_t)))),)
    } else {
        (var_qjs, var_qjs_dn0, var_qjs_dn1, var_qjs_dn2, var_qjs_dn3, var_qjs_dn4, var_qjs_dn5, var_qjs_dn6, var_qjs_dn7, var_qjs_dn8, var_qjs_dn9, var_qjs_db0, var_qjs_db1, var_qjs_db2, var_qjs_db3, var_qjs_db4, var_qjs_db5, var_qjs_db6, var_qjs_db7,)
    }
};
        var_qjs = assign1590_e1893;
        var_qjs_dn0 = assign1590_e1893_d_n0;
        var_qjs_dn1 = assign1590_e1893_d_n1;
        var_qjs_dn2 = assign1590_e1893_d_n2;
        var_qjs_dn3 = assign1590_e1893_d_n3;
        var_qjs_dn4 = assign1590_e1893_d_n4;
        var_qjs_dn5 = assign1590_e1893_d_n5;
        var_qjs_dn6 = assign1590_e1893_d_n6;
        var_qjs_dn7 = assign1590_e1893_d_n7;
        var_qjs_dn8 = assign1590_e1893_d_n8;
        var_qjs_dn9 = assign1590_e1893_d_n9;
        var_qjs_db0 = assign1590_e1893_d_b0;
        var_qjs_db1 = assign1590_e1893_d_b1;
        var_qjs_db2 = assign1590_e1893_d_b2;
        var_qjs_db3 = assign1590_e1893_d_b3;
        var_qjs_db4 = assign1590_e1893_d_b4;
        var_qjs_db5 = assign1590_e1893_d_b5;
        var_qjs_db6 = assign1590_e1893_d_b6;
        var_qjs_db7 = assign1590_e1893_d_b7;
        var_qjs_rv = 0.0;
        var_qjs_rdn0 = 0.0;
        var_qjs_rdn1 = 0.0;
        var_qjs_rdn2 = 0.0;
        var_qjs_rdn3 = 0.0;
        var_qjs_rdn4 = 0.0;
        var_qjs_rdn5 = 0.0;
        var_qjs_rdn6 = 0.0;
        var_qjs_rdn7 = 0.0;
        var_qjs_rdn8 = 0.0;
        var_qjs_rdn9 = 0.0;
        var_qjs_rdb0 = 0.0;
        var_qjs_rdb1 = 0.0;
        var_qjs_rdb2 = 0.0;
        var_qjs_rdb3 = 0.0;
        var_qjs_rdb4 = 0.0;
        var_qjs_rdb5 = 0.0;
        var_qjs_rdb6 = 0.0;
        var_qjs_rdb7 = 0.0;

        let assign1600_e1895: f64 = (-var_vje_t);
        let assign1600_e1897: f64 = (assign1600_e1895 * p.p24);
        var_dv0 = assign1600_e1897;
        var_dv0_dn0 = ((-var_vje_t_dn0) * p.p24);
        var_dv0_dn1 = ((-var_vje_t_dn1) * p.p24);
        var_dv0_dn2 = ((-var_vje_t_dn2) * p.p24);
        var_dv0_dn3 = ((-var_vje_t_dn3) * p.p24);
        var_dv0_dn4 = ((-var_vje_t_dn4) * p.p24);
        var_dv0_dn5 = ((-var_vje_t_dn5) * p.p24);
        var_dv0_dn6 = ((-var_vje_t_dn6) * p.p24);
        var_dv0_dn7 = ((-var_vje_t_dn7) * p.p24);
        var_dv0_dn8 = ((-var_vje_t_dn8) * p.p24);
        var_dv0_dn9 = ((-var_vje_t_dn9) * p.p24);
        var_dv0_db0 = ((-var_vje_t_db0) * p.p24);
        var_dv0_db1 = ((-var_vje_t_db1) * p.p24);
        var_dv0_db2 = ((-var_vje_t_db2) * p.p24);
        var_dv0_db3 = ((-var_vje_t_db3) * p.p24);
        var_dv0_db4 = ((-var_vje_t_db4) * p.p24);
        var_dv0_db5 = ((-var_vje_t_db5) * p.p24);
        var_dv0_db6 = ((-var_vje_t_db6) * p.p24);
        var_dv0_db7 = ((-var_vje_t_db7) * p.p24);
        var_dv0_rv = 0.0;
        var_dv0_rdn0 = 0.0;
        var_dv0_rdn1 = 0.0;
        var_dv0_rdn2 = 0.0;
        var_dv0_rdn3 = 0.0;
        var_dv0_rdn4 = 0.0;
        var_dv0_rdn5 = 0.0;
        var_dv0_rdn6 = 0.0;
        var_dv0_rdn7 = 0.0;
        var_dv0_rdn8 = 0.0;
        var_dv0_rdn9 = 0.0;
        var_dv0_rdb0 = 0.0;
        var_dv0_rdb1 = 0.0;
        var_dv0_rdb2 = 0.0;
        var_dv0_rdb3 = 0.0;
        var_dv0_rdb4 = 0.0;
        var_dv0_rdb5 = 0.0;
        var_dv0_rdb6 = 0.0;
        var_dv0_rdb7 = 0.0;

        let assign1610_e1900: f64 = (var_vbiei + var_dv0);
        var_dvh = assign1610_e1900;
        var_dvh_dn0 = (var_vbiei_dn0 + var_dv0_dn0);
        var_dvh_dn1 = (var_vbiei_dn1 + var_dv0_dn1);
        var_dvh_dn2 = (var_vbiei_dn2 + var_dv0_dn2);
        var_dvh_dn3 = (var_vbiei_dn3 + var_dv0_dn3);
        var_dvh_dn4 = (var_vbiei_dn4 + var_dv0_dn4);
        var_dvh_dn5 = (var_vbiei_dn5 + var_dv0_dn5);
        var_dvh_dn6 = (var_vbiei_dn6 + var_dv0_dn6);
        var_dvh_dn7 = (var_vbiei_dn7 + var_dv0_dn7);
        var_dvh_dn8 = (var_vbiei_dn8 + var_dv0_dn8);
        var_dvh_dn9 = (var_vbiei_dn9 + var_dv0_dn9);
        var_dvh_db0 = (var_vbiei_db0 + var_dv0_db0);
        var_dvh_db1 = (var_vbiei_db1 + var_dv0_db1);
        var_dvh_db2 = (var_vbiei_db2 + var_dv0_db2);
        var_dvh_db3 = (var_vbiei_db3 + var_dv0_db3);
        var_dvh_db4 = (var_vbiei_db4 + var_dv0_db4);
        var_dvh_db5 = (var_vbiei_db5 + var_dv0_db5);
        var_dvh_db6 = (var_vbiei_db6 + var_dv0_db6);
        var_dvh_db7 = (var_vbiei_db7 + var_dv0_db7);
        var_dvh_rv = 0.0;
        var_dvh_rdn0 = 0.0;
        var_dvh_rdn1 = 0.0;
        var_dvh_rdn2 = 0.0;
        var_dvh_rdn3 = 0.0;
        var_dvh_rdn4 = 0.0;
        var_dvh_rdn5 = 0.0;
        var_dvh_rdn6 = 0.0;
        var_dvh_rdn7 = 0.0;
        var_dvh_rdn8 = 0.0;
        var_dvh_rdn9 = 0.0;
        var_dvh_rdb0 = 0.0;
        var_dvh_rdb1 = 0.0;
        var_dvh_rdb2 = 0.0;
        var_dvh_rdb3 = 0.0;
        var_dvh_rdb4 = 0.0;
        var_dvh_rdb5 = 0.0;
        var_dvh_rdb6 = 0.0;
        var_dvh_rdb7 = 0.0;

        let assign1620_e1903: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard16 = assign1620_e1903;
        var_guard16_dn0 = 0.0;
        var_guard16_dn1 = 0.0;
        var_guard16_dn2 = 0.0;
        var_guard16_dn3 = 0.0;
        var_guard16_dn4 = 0.0;
        var_guard16_dn5 = 0.0;
        var_guard16_dn6 = 0.0;
        var_guard16_dn7 = 0.0;
        var_guard16_dn8 = 0.0;
        var_guard16_dn9 = 0.0;
        var_guard16_db0 = 0.0;
        var_guard16_db1 = 0.0;
        var_guard16_db2 = 0.0;
        var_guard16_db3 = 0.0;
        var_guard16_db4 = 0.0;
        var_guard16_db5 = 0.0;
        var_guard16_db6 = 0.0;
        var_guard16_db7 = 0.0;
        var_guard16_rv = 0.0;
        var_guard16_rdn0 = 0.0;
        var_guard16_rdn1 = 0.0;
        var_guard16_rdn2 = 0.0;
        var_guard16_rdn3 = 0.0;
        var_guard16_rdn4 = 0.0;
        var_guard16_rdn5 = 0.0;
        var_guard16_rdn6 = 0.0;
        var_guard16_rdn7 = 0.0;
        var_guard16_rdn8 = 0.0;
        var_guard16_rdn9 = 0.0;
        var_guard16_rdb0 = 0.0;
        var_guard16_rdb1 = 0.0;
        var_guard16_rdb2 = 0.0;
        var_guard16_rdb3 = 0.0;
        var_guard16_rdb4 = 0.0;
        var_guard16_rdb5 = 0.0;
        var_guard16_rdb6 = 0.0;
        var_guard16_rdb7 = 0.0;

        let (assign1630_e1916, assign1630_e1916_d_n0, assign1630_e1916_d_n1, assign1630_e1916_d_n2, assign1630_e1916_d_n3, assign1630_e1916_d_n4, assign1630_e1916_d_n5, assign1630_e1916_d_n6, assign1630_e1916_d_n7, assign1630_e1916_d_n8, assign1630_e1916_d_n9, assign1630_e1916_d_b0, assign1630_e1916_d_b1, assign1630_e1916_d_b2, assign1630_e1916_d_b3, assign1630_e1916_d_b4, assign1630_e1916_d_b5, assign1630_e1916_d_b6, assign1630_e1916_d_b7,) = {
    if (var_guard16 != 0.0) {
        let assign1630_e1906: f64 = (-1.0);
        let assign1630_e1908: f64 = (assign1630_e1906 - p.p18);
        let assign1630_e1911: f64 = (1.0 - p.p24);
        let assign1630_e1912: f64 = (assign1630_e1911).ln();
        let assign1630_e1913: f64 = (assign1630_e1908 * assign1630_e1912);
        let assign1630_e1914: f64 = (assign1630_e1913).exp();
        (assign1630_e1914, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pwq, var_pwq_dn0, var_pwq_dn1, var_pwq_dn2, var_pwq_dn3, var_pwq_dn4, var_pwq_dn5, var_pwq_dn6, var_pwq_dn7, var_pwq_dn8, var_pwq_dn9, var_pwq_db0, var_pwq_db1, var_pwq_db2, var_pwq_db3, var_pwq_db4, var_pwq_db5, var_pwq_db6, var_pwq_db7,)
    }
};
        var_pwq = assign1630_e1916;
        var_pwq_dn0 = assign1630_e1916_d_n0;
        var_pwq_dn1 = assign1630_e1916_d_n1;
        var_pwq_dn2 = assign1630_e1916_d_n2;
        var_pwq_dn3 = assign1630_e1916_d_n3;
        var_pwq_dn4 = assign1630_e1916_d_n4;
        var_pwq_dn5 = assign1630_e1916_d_n5;
        var_pwq_dn6 = assign1630_e1916_d_n6;
        var_pwq_dn7 = assign1630_e1916_d_n7;
        var_pwq_dn8 = assign1630_e1916_d_n8;
        var_pwq_dn9 = assign1630_e1916_d_n9;
        var_pwq_db0 = assign1630_e1916_d_b0;
        var_pwq_db1 = assign1630_e1916_d_b1;
        var_pwq_db2 = assign1630_e1916_d_b2;
        var_pwq_db3 = assign1630_e1916_d_b3;
        var_pwq_db4 = assign1630_e1916_d_b4;
        var_pwq_db5 = assign1630_e1916_d_b5;
        var_pwq_db6 = assign1630_e1916_d_b6;
        var_pwq_db7 = assign1630_e1916_d_b7;
        var_pwq_rv = 0.0;
        var_pwq_rdn0 = 0.0;
        var_pwq_rdn1 = 0.0;
        var_pwq_rdn2 = 0.0;
        var_pwq_rdn3 = 0.0;
        var_pwq_rdn4 = 0.0;
        var_pwq_rdn5 = 0.0;
        var_pwq_rdn6 = 0.0;
        var_pwq_rdn7 = 0.0;
        var_pwq_rdn8 = 0.0;
        var_pwq_rdn9 = 0.0;
        var_pwq_rdb0 = 0.0;
        var_pwq_rdb1 = 0.0;
        var_pwq_rdb2 = 0.0;
        var_pwq_rdb3 = 0.0;
        var_pwq_rdb4 = 0.0;
        var_pwq_rdb5 = 0.0;
        var_pwq_rdb6 = 0.0;
        var_pwq_rdb7 = 0.0;

        let (assign1640_e1936, assign1640_e1936_d_n0, assign1640_e1936_d_n1, assign1640_e1936_d_n2, assign1640_e1936_d_n3, assign1640_e1936_d_n4, assign1640_e1936_d_n5, assign1640_e1936_d_n6, assign1640_e1936_d_n7, assign1640_e1936_d_n8, assign1640_e1936_d_n9, assign1640_e1936_d_b0, assign1640_e1936_d_b1, assign1640_e1936_d_b2, assign1640_e1936_d_b3, assign1640_e1936_d_b4, assign1640_e1936_d_b5, assign1640_e1936_d_b6, assign1640_e1936_d_b7,) = {
    if (var_guard16 != 0.0) {
        let assign1640_e1923: f64 = (1.0 - p.p24);
        let assign1640_e1924: f64 = (var_pwq * assign1640_e1923);
        let assign1640_e1927: f64 = (1.0 - p.p24);
        let assign1640_e1928: f64 = (assign1640_e1924 * assign1640_e1927);
        let assign1640_e1929: f64 = (1.0 - assign1640_e1928);
        let assign1640_e1930: f64 = (var_vje_t * assign1640_e1929);
        let assign1640_e1933: f64 = (1.0 - p.p18);
        let assign1640_e1934: f64 = (assign1640_e1930 / assign1640_e1933);
        (assign1640_e1934, (((var_vje_t_dn0 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn0 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn1 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn1 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn2 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn2 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn3 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn3 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn4 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn4 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn5 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn5 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn6 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn6 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn7 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn7 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn8 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn8 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn9 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn9 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db0 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db0 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db1 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db1 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db2 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db2 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db3 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db3 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db4 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db4 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db5 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db5 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db6 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db6 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db7 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db7 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7,)
    }
};
        var_qlo = assign1640_e1936;
        var_qlo_dn0 = assign1640_e1936_d_n0;
        var_qlo_dn1 = assign1640_e1936_d_n1;
        var_qlo_dn2 = assign1640_e1936_d_n2;
        var_qlo_dn3 = assign1640_e1936_d_n3;
        var_qlo_dn4 = assign1640_e1936_d_n4;
        var_qlo_dn5 = assign1640_e1936_d_n5;
        var_qlo_dn6 = assign1640_e1936_d_n6;
        var_qlo_dn7 = assign1640_e1936_d_n7;
        var_qlo_dn8 = assign1640_e1936_d_n8;
        var_qlo_dn9 = assign1640_e1936_d_n9;
        var_qlo_db0 = assign1640_e1936_d_b0;
        var_qlo_db1 = assign1640_e1936_d_b1;
        var_qlo_db2 = assign1640_e1936_d_b2;
        var_qlo_db3 = assign1640_e1936_d_b3;
        var_qlo_db4 = assign1640_e1936_d_b4;
        var_qlo_db5 = assign1640_e1936_d_b5;
        var_qlo_db6 = assign1640_e1936_d_b6;
        var_qlo_db7 = assign1640_e1936_d_b7;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdn7 = 0.0;
        var_qlo_rdn8 = 0.0;
        var_qlo_rdn9 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;
        var_qlo_rdb7 = 0.0;

        let (assign1650_e1954, assign1650_e1954_d_n0, assign1650_e1954_d_n1, assign1650_e1954_d_n2, assign1650_e1954_d_n3, assign1650_e1954_d_n4, assign1650_e1954_d_n5, assign1650_e1954_d_n6, assign1650_e1954_d_n7, assign1650_e1954_d_n8, assign1650_e1954_d_n9, assign1650_e1954_d_b0, assign1650_e1954_d_b1, assign1650_e1954_d_b2, assign1650_e1954_d_b3, assign1650_e1954_d_b4, assign1650_e1954_d_b5, assign1650_e1954_d_b6, assign1650_e1954_d_b7,) = {
    if (var_guard16 != 0.0) {
        let assign1650_e1941: f64 = (1.0 - p.p24);
        let assign1650_e1944: f64 = (0.5 * p.p18);
        let assign1650_e1946: f64 = (assign1650_e1944 * var_dvh);
        let assign1650_e1948: f64 = (assign1650_e1946 / var_vje_t);
        let assign1650_e1949: f64 = (assign1650_e1941 + assign1650_e1948);
        let assign1650_e1950: f64 = (var_dvh * assign1650_e1949);
        let assign1650_e1952: f64 = (assign1650_e1950 * var_pwq);
        (assign1650_e1952, ((((var_dvh_dn0 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn0) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn0)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn0)), ((((var_dvh_dn1 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn1) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn1)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn1)), ((((var_dvh_dn2 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn2) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn2)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn2)), ((((var_dvh_dn3 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn3) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn3)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn3)), ((((var_dvh_dn4 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn4) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn4)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn4)), ((((var_dvh_dn5 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn5) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn5)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn5)), ((((var_dvh_dn6 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn6) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn6)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn6)), ((((var_dvh_dn7 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn7) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn7)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn7)), ((((var_dvh_dn8 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn8) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn8)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn8)), ((((var_dvh_dn9 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn9) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn9)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn9)), ((((var_dvh_db0 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db0) * var_vje_t) - (assign1650_e1946 * var_vje_t_db0)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db0)), ((((var_dvh_db1 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db1) * var_vje_t) - (assign1650_e1946 * var_vje_t_db1)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db1)), ((((var_dvh_db2 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db2) * var_vje_t) - (assign1650_e1946 * var_vje_t_db2)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db2)), ((((var_dvh_db3 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db3) * var_vje_t) - (assign1650_e1946 * var_vje_t_db3)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db3)), ((((var_dvh_db4 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db4) * var_vje_t) - (assign1650_e1946 * var_vje_t_db4)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db4)), ((((var_dvh_db5 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db5) * var_vje_t) - (assign1650_e1946 * var_vje_t_db5)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db5)), ((((var_dvh_db6 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db6) * var_vje_t) - (assign1650_e1946 * var_vje_t_db6)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db6)), ((((var_dvh_db7 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db7) * var_vje_t) - (assign1650_e1946 * var_vje_t_db7)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db7)),)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_dn7, var_qhi_dn8, var_qhi_dn9, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6, var_qhi_db7,)
    }
};
        var_qhi = assign1650_e1954;
        var_qhi_dn0 = assign1650_e1954_d_n0;
        var_qhi_dn1 = assign1650_e1954_d_n1;
        var_qhi_dn2 = assign1650_e1954_d_n2;
        var_qhi_dn3 = assign1650_e1954_d_n3;
        var_qhi_dn4 = assign1650_e1954_d_n4;
        var_qhi_dn5 = assign1650_e1954_d_n5;
        var_qhi_dn6 = assign1650_e1954_d_n6;
        var_qhi_dn7 = assign1650_e1954_d_n7;
        var_qhi_dn8 = assign1650_e1954_d_n8;
        var_qhi_dn9 = assign1650_e1954_d_n9;
        var_qhi_db0 = assign1650_e1954_d_b0;
        var_qhi_db1 = assign1650_e1954_d_b1;
        var_qhi_db2 = assign1650_e1954_d_b2;
        var_qhi_db3 = assign1650_e1954_d_b3;
        var_qhi_db4 = assign1650_e1954_d_b4;
        var_qhi_db5 = assign1650_e1954_d_b5;
        var_qhi_db6 = assign1650_e1954_d_b6;
        var_qhi_db7 = assign1650_e1954_d_b7;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdn7 = 0.0;
        var_qhi_rdn8 = 0.0;
        var_qhi_rdn9 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;
        var_qhi_rdb7 = 0.0;

        *var_dv0_slot = var_dv0;
        *var_dv0_db0_slot = var_dv0_db0;
        *var_dv0_db1_slot = var_dv0_db1;
        *var_dv0_db2_slot = var_dv0_db2;
        *var_dv0_db3_slot = var_dv0_db3;
        *var_dv0_db4_slot = var_dv0_db4;
        *var_dv0_db5_slot = var_dv0_db5;
        *var_dv0_db6_slot = var_dv0_db6;
        *var_dv0_db7_slot = var_dv0_db7;
        *var_dv0_dn0_slot = var_dv0_dn0;
        *var_dv0_dn1_slot = var_dv0_dn1;
        *var_dv0_dn2_slot = var_dv0_dn2;
        *var_dv0_dn3_slot = var_dv0_dn3;
        *var_dv0_dn4_slot = var_dv0_dn4;
        *var_dv0_dn5_slot = var_dv0_dn5;
        *var_dv0_dn6_slot = var_dv0_dn6;
        *var_dv0_dn7_slot = var_dv0_dn7;
        *var_dv0_dn8_slot = var_dv0_dn8;
        *var_dv0_dn9_slot = var_dv0_dn9;
        *var_dv0_rdb0_slot = var_dv0_rdb0;
        *var_dv0_rdb1_slot = var_dv0_rdb1;
        *var_dv0_rdb2_slot = var_dv0_rdb2;
        *var_dv0_rdb3_slot = var_dv0_rdb3;
        *var_dv0_rdb4_slot = var_dv0_rdb4;
        *var_dv0_rdb5_slot = var_dv0_rdb5;
        *var_dv0_rdb6_slot = var_dv0_rdb6;
        *var_dv0_rdb7_slot = var_dv0_rdb7;
        *var_dv0_rdn0_slot = var_dv0_rdn0;
        *var_dv0_rdn1_slot = var_dv0_rdn1;
        *var_dv0_rdn2_slot = var_dv0_rdn2;
        *var_dv0_rdn3_slot = var_dv0_rdn3;
        *var_dv0_rdn4_slot = var_dv0_rdn4;
        *var_dv0_rdn5_slot = var_dv0_rdn5;
        *var_dv0_rdn6_slot = var_dv0_rdn6;
        *var_dv0_rdn7_slot = var_dv0_rdn7;
        *var_dv0_rdn8_slot = var_dv0_rdn8;
        *var_dv0_rdn9_slot = var_dv0_rdn9;
        *var_dv0_rv_slot = var_dv0_rv;
        *var_dvh_slot = var_dvh;
        *var_dvh_db0_slot = var_dvh_db0;
        *var_dvh_db1_slot = var_dvh_db1;
        *var_dvh_db2_slot = var_dvh_db2;
        *var_dvh_db3_slot = var_dvh_db3;
        *var_dvh_db4_slot = var_dvh_db4;
        *var_dvh_db5_slot = var_dvh_db5;
        *var_dvh_db6_slot = var_dvh_db6;
        *var_dvh_db7_slot = var_dvh_db7;
        *var_dvh_dn0_slot = var_dvh_dn0;
        *var_dvh_dn1_slot = var_dvh_dn1;
        *var_dvh_dn2_slot = var_dvh_dn2;
        *var_dvh_dn3_slot = var_dvh_dn3;
        *var_dvh_dn4_slot = var_dvh_dn4;
        *var_dvh_dn5_slot = var_dvh_dn5;
        *var_dvh_dn6_slot = var_dvh_dn6;
        *var_dvh_dn7_slot = var_dvh_dn7;
        *var_dvh_dn8_slot = var_dvh_dn8;
        *var_dvh_dn9_slot = var_dvh_dn9;
        *var_dvh_rdb0_slot = var_dvh_rdb0;
        *var_dvh_rdb1_slot = var_dvh_rdb1;
        *var_dvh_rdb2_slot = var_dvh_rdb2;
        *var_dvh_rdb3_slot = var_dvh_rdb3;
        *var_dvh_rdb4_slot = var_dvh_rdb4;
        *var_dvh_rdb5_slot = var_dvh_rdb5;
        *var_dvh_rdb6_slot = var_dvh_rdb6;
        *var_dvh_rdb7_slot = var_dvh_rdb7;
        *var_dvh_rdn0_slot = var_dvh_rdn0;
        *var_dvh_rdn1_slot = var_dvh_rdn1;
        *var_dvh_rdn2_slot = var_dvh_rdn2;
        *var_dvh_rdn3_slot = var_dvh_rdn3;
        *var_dvh_rdn4_slot = var_dvh_rdn4;
        *var_dvh_rdn5_slot = var_dvh_rdn5;
        *var_dvh_rdn6_slot = var_dvh_rdn6;
        *var_dvh_rdn7_slot = var_dvh_rdn7;
        *var_dvh_rdn8_slot = var_dvh_rdn8;
        *var_dvh_rdn9_slot = var_dvh_rdn9;
        *var_dvh_rv_slot = var_dvh_rv;
        *var_guard13_slot = var_guard13;
        *var_guard13_db0_slot = var_guard13_db0;
        *var_guard13_db1_slot = var_guard13_db1;
        *var_guard13_db2_slot = var_guard13_db2;
        *var_guard13_db3_slot = var_guard13_db3;
        *var_guard13_db4_slot = var_guard13_db4;
        *var_guard13_db5_slot = var_guard13_db5;
        *var_guard13_db6_slot = var_guard13_db6;
        *var_guard13_db7_slot = var_guard13_db7;
        *var_guard13_dn0_slot = var_guard13_dn0;
        *var_guard13_dn1_slot = var_guard13_dn1;
        *var_guard13_dn2_slot = var_guard13_dn2;
        *var_guard13_dn3_slot = var_guard13_dn3;
        *var_guard13_dn4_slot = var_guard13_dn4;
        *var_guard13_dn5_slot = var_guard13_dn5;
        *var_guard13_dn6_slot = var_guard13_dn6;
        *var_guard13_dn7_slot = var_guard13_dn7;
        *var_guard13_dn8_slot = var_guard13_dn8;
        *var_guard13_dn9_slot = var_guard13_dn9;
        *var_guard13_rdb0_slot = var_guard13_rdb0;
        *var_guard13_rdb1_slot = var_guard13_rdb1;
        *var_guard13_rdb2_slot = var_guard13_rdb2;
        *var_guard13_rdb3_slot = var_guard13_rdb3;
        *var_guard13_rdb4_slot = var_guard13_rdb4;
        *var_guard13_rdb5_slot = var_guard13_rdb5;
        *var_guard13_rdb6_slot = var_guard13_rdb6;
        *var_guard13_rdb7_slot = var_guard13_rdb7;
        *var_guard13_rdn0_slot = var_guard13_rdn0;
        *var_guard13_rdn1_slot = var_guard13_rdn1;
        *var_guard13_rdn2_slot = var_guard13_rdn2;
        *var_guard13_rdn3_slot = var_guard13_rdn3;
        *var_guard13_rdn4_slot = var_guard13_rdn4;
        *var_guard13_rdn5_slot = var_guard13_rdn5;
        *var_guard13_rdn6_slot = var_guard13_rdn6;
        *var_guard13_rdn7_slot = var_guard13_rdn7;
        *var_guard13_rdn8_slot = var_guard13_rdn8;
        *var_guard13_rdn9_slot = var_guard13_rdn9;
        *var_guard13_rv_slot = var_guard13_rv;
        *var_guard15_slot = var_guard15;
        *var_guard15_db0_slot = var_guard15_db0;
        *var_guard15_db1_slot = var_guard15_db1;
        *var_guard15_db2_slot = var_guard15_db2;
        *var_guard15_db3_slot = var_guard15_db3;
        *var_guard15_db4_slot = var_guard15_db4;
        *var_guard15_db5_slot = var_guard15_db5;
        *var_guard15_db6_slot = var_guard15_db6;
        *var_guard15_db7_slot = var_guard15_db7;
        *var_guard15_dn0_slot = var_guard15_dn0;
        *var_guard15_dn1_slot = var_guard15_dn1;
        *var_guard15_dn2_slot = var_guard15_dn2;
        *var_guard15_dn3_slot = var_guard15_dn3;
        *var_guard15_dn4_slot = var_guard15_dn4;
        *var_guard15_dn5_slot = var_guard15_dn5;
        *var_guard15_dn6_slot = var_guard15_dn6;
        *var_guard15_dn7_slot = var_guard15_dn7;
        *var_guard15_dn8_slot = var_guard15_dn8;
        *var_guard15_dn9_slot = var_guard15_dn9;
        *var_guard15_rdb0_slot = var_guard15_rdb0;
        *var_guard15_rdb1_slot = var_guard15_rdb1;
        *var_guard15_rdb2_slot = var_guard15_rdb2;
        *var_guard15_rdb3_slot = var_guard15_rdb3;
        *var_guard15_rdb4_slot = var_guard15_rdb4;
        *var_guard15_rdb5_slot = var_guard15_rdb5;
        *var_guard15_rdb6_slot = var_guard15_rdb6;
        *var_guard15_rdb7_slot = var_guard15_rdb7;
        *var_guard15_rdn0_slot = var_guard15_rdn0;
        *var_guard15_rdn1_slot = var_guard15_rdn1;
        *var_guard15_rdn2_slot = var_guard15_rdn2;
        *var_guard15_rdn3_slot = var_guard15_rdn3;
        *var_guard15_rdn4_slot = var_guard15_rdn4;
        *var_guard15_rdn5_slot = var_guard15_rdn5;
        *var_guard15_rdn6_slot = var_guard15_rdn6;
        *var_guard15_rdn7_slot = var_guard15_rdn7;
        *var_guard15_rdn8_slot = var_guard15_rdn8;
        *var_guard15_rdn9_slot = var_guard15_rdn9;
        *var_guard15_rv_slot = var_guard15_rv;
        *var_guard16_slot = var_guard16;
        *var_guard16_db0_slot = var_guard16_db0;
        *var_guard16_db1_slot = var_guard16_db1;
        *var_guard16_db2_slot = var_guard16_db2;
        *var_guard16_db3_slot = var_guard16_db3;
        *var_guard16_db4_slot = var_guard16_db4;
        *var_guard16_db5_slot = var_guard16_db5;
        *var_guard16_db6_slot = var_guard16_db6;
        *var_guard16_db7_slot = var_guard16_db7;
        *var_guard16_dn0_slot = var_guard16_dn0;
        *var_guard16_dn1_slot = var_guard16_dn1;
        *var_guard16_dn2_slot = var_guard16_dn2;
        *var_guard16_dn3_slot = var_guard16_dn3;
        *var_guard16_dn4_slot = var_guard16_dn4;
        *var_guard16_dn5_slot = var_guard16_dn5;
        *var_guard16_dn6_slot = var_guard16_dn6;
        *var_guard16_dn7_slot = var_guard16_dn7;
        *var_guard16_dn8_slot = var_guard16_dn8;
        *var_guard16_dn9_slot = var_guard16_dn9;
        *var_guard16_rdb0_slot = var_guard16_rdb0;
        *var_guard16_rdb1_slot = var_guard16_rdb1;
        *var_guard16_rdb2_slot = var_guard16_rdb2;
        *var_guard16_rdb3_slot = var_guard16_rdb3;
        *var_guard16_rdb4_slot = var_guard16_rdb4;
        *var_guard16_rdb5_slot = var_guard16_rdb5;
        *var_guard16_rdb6_slot = var_guard16_rdb6;
        *var_guard16_rdb7_slot = var_guard16_rdb7;
        *var_guard16_rdn0_slot = var_guard16_rdn0;
        *var_guard16_rdn1_slot = var_guard16_rdn1;
        *var_guard16_rdn2_slot = var_guard16_rdn2;
        *var_guard16_rdn3_slot = var_guard16_rdn3;
        *var_guard16_rdn4_slot = var_guard16_rdn4;
        *var_guard16_rdn5_slot = var_guard16_rdn5;
        *var_guard16_rdn6_slot = var_guard16_rdn6;
        *var_guard16_rdn7_slot = var_guard16_rdn7;
        *var_guard16_rdn8_slot = var_guard16_rdn8;
        *var_guard16_rdn9_slot = var_guard16_rdn9;
        *var_guard16_rv_slot = var_guard16_rv;
        *var_pwq_slot = var_pwq;
        *var_pwq_db0_slot = var_pwq_db0;
        *var_pwq_db1_slot = var_pwq_db1;
        *var_pwq_db2_slot = var_pwq_db2;
        *var_pwq_db3_slot = var_pwq_db3;
        *var_pwq_db4_slot = var_pwq_db4;
        *var_pwq_db5_slot = var_pwq_db5;
        *var_pwq_db6_slot = var_pwq_db6;
        *var_pwq_db7_slot = var_pwq_db7;
        *var_pwq_dn0_slot = var_pwq_dn0;
        *var_pwq_dn1_slot = var_pwq_dn1;
        *var_pwq_dn2_slot = var_pwq_dn2;
        *var_pwq_dn3_slot = var_pwq_dn3;
        *var_pwq_dn4_slot = var_pwq_dn4;
        *var_pwq_dn5_slot = var_pwq_dn5;
        *var_pwq_dn6_slot = var_pwq_dn6;
        *var_pwq_dn7_slot = var_pwq_dn7;
        *var_pwq_dn8_slot = var_pwq_dn8;
        *var_pwq_dn9_slot = var_pwq_dn9;
        *var_pwq_rdb0_slot = var_pwq_rdb0;
        *var_pwq_rdb1_slot = var_pwq_rdb1;
        *var_pwq_rdb2_slot = var_pwq_rdb2;
        *var_pwq_rdb3_slot = var_pwq_rdb3;
        *var_pwq_rdb4_slot = var_pwq_rdb4;
        *var_pwq_rdb5_slot = var_pwq_rdb5;
        *var_pwq_rdb6_slot = var_pwq_rdb6;
        *var_pwq_rdb7_slot = var_pwq_rdb7;
        *var_pwq_rdn0_slot = var_pwq_rdn0;
        *var_pwq_rdn1_slot = var_pwq_rdn1;
        *var_pwq_rdn2_slot = var_pwq_rdn2;
        *var_pwq_rdn3_slot = var_pwq_rdn3;
        *var_pwq_rdn4_slot = var_pwq_rdn4;
        *var_pwq_rdn5_slot = var_pwq_rdn5;
        *var_pwq_rdn6_slot = var_pwq_rdn6;
        *var_pwq_rdn7_slot = var_pwq_rdn7;
        *var_pwq_rdn8_slot = var_pwq_rdn8;
        *var_pwq_rdn9_slot = var_pwq_rdn9;
        *var_pwq_rv_slot = var_pwq_rv;
        *var_qhi_slot = var_qhi;
        *var_qhi_db0_slot = var_qhi_db0;
        *var_qhi_db1_slot = var_qhi_db1;
        *var_qhi_db2_slot = var_qhi_db2;
        *var_qhi_db3_slot = var_qhi_db3;
        *var_qhi_db4_slot = var_qhi_db4;
        *var_qhi_db5_slot = var_qhi_db5;
        *var_qhi_db6_slot = var_qhi_db6;
        *var_qhi_db7_slot = var_qhi_db7;
        *var_qhi_dn0_slot = var_qhi_dn0;
        *var_qhi_dn1_slot = var_qhi_dn1;
        *var_qhi_dn2_slot = var_qhi_dn2;
        *var_qhi_dn3_slot = var_qhi_dn3;
        *var_qhi_dn4_slot = var_qhi_dn4;
        *var_qhi_dn5_slot = var_qhi_dn5;
        *var_qhi_dn6_slot = var_qhi_dn6;
        *var_qhi_dn7_slot = var_qhi_dn7;
        *var_qhi_dn8_slot = var_qhi_dn8;
        *var_qhi_dn9_slot = var_qhi_dn9;
        *var_qhi_rdb0_slot = var_qhi_rdb0;
        *var_qhi_rdb1_slot = var_qhi_rdb1;
        *var_qhi_rdb2_slot = var_qhi_rdb2;
        *var_qhi_rdb3_slot = var_qhi_rdb3;
        *var_qhi_rdb4_slot = var_qhi_rdb4;
        *var_qhi_rdb5_slot = var_qhi_rdb5;
        *var_qhi_rdb6_slot = var_qhi_rdb6;
        *var_qhi_rdb7_slot = var_qhi_rdb7;
        *var_qhi_rdn0_slot = var_qhi_rdn0;
        *var_qhi_rdn1_slot = var_qhi_rdn1;
        *var_qhi_rdn2_slot = var_qhi_rdn2;
        *var_qhi_rdn3_slot = var_qhi_rdn3;
        *var_qhi_rdn4_slot = var_qhi_rdn4;
        *var_qhi_rdn5_slot = var_qhi_rdn5;
        *var_qhi_rdn6_slot = var_qhi_rdn6;
        *var_qhi_rdn7_slot = var_qhi_rdn7;
        *var_qhi_rdn8_slot = var_qhi_rdn8;
        *var_qhi_rdn9_slot = var_qhi_rdn9;
        *var_qhi_rv_slot = var_qhi_rv;
        *var_qjs_slot = var_qjs;
        *var_qjs_db0_slot = var_qjs_db0;
        *var_qjs_db1_slot = var_qjs_db1;
        *var_qjs_db2_slot = var_qjs_db2;
        *var_qjs_db3_slot = var_qjs_db3;
        *var_qjs_db4_slot = var_qjs_db4;
        *var_qjs_db5_slot = var_qjs_db5;
        *var_qjs_db6_slot = var_qjs_db6;
        *var_qjs_db7_slot = var_qjs_db7;
        *var_qjs_dn0_slot = var_qjs_dn0;
        *var_qjs_dn1_slot = var_qjs_dn1;
        *var_qjs_dn2_slot = var_qjs_dn2;
        *var_qjs_dn3_slot = var_qjs_dn3;
        *var_qjs_dn4_slot = var_qjs_dn4;
        *var_qjs_dn5_slot = var_qjs_dn5;
        *var_qjs_dn6_slot = var_qjs_dn6;
        *var_qjs_dn7_slot = var_qjs_dn7;
        *var_qjs_dn8_slot = var_qjs_dn8;
        *var_qjs_dn9_slot = var_qjs_dn9;
        *var_qjs_rdb0_slot = var_qjs_rdb0;
        *var_qjs_rdb1_slot = var_qjs_rdb1;
        *var_qjs_rdb2_slot = var_qjs_rdb2;
        *var_qjs_rdb3_slot = var_qjs_rdb3;
        *var_qjs_rdb4_slot = var_qjs_rdb4;
        *var_qjs_rdb5_slot = var_qjs_rdb5;
        *var_qjs_rdb6_slot = var_qjs_rdb6;
        *var_qjs_rdb7_slot = var_qjs_rdb7;
        *var_qjs_rdn0_slot = var_qjs_rdn0;
        *var_qjs_rdn1_slot = var_qjs_rdn1;
        *var_qjs_rdn2_slot = var_qjs_rdn2;
        *var_qjs_rdn3_slot = var_qjs_rdn3;
        *var_qjs_rdn4_slot = var_qjs_rdn4;
        *var_qjs_rdn5_slot = var_qjs_rdn5;
        *var_qjs_rdn6_slot = var_qjs_rdn6;
        *var_qjs_rdn7_slot = var_qjs_rdn7;
        *var_qjs_rdn8_slot = var_qjs_rdn8;
        *var_qjs_rdn9_slot = var_qjs_rdn9;
        *var_qjs_rv_slot = var_qjs_rv;
        *var_qlo_slot = var_qlo;
        *var_qlo_db0_slot = var_qlo_db0;
        *var_qlo_db1_slot = var_qlo_db1;
        *var_qlo_db2_slot = var_qlo_db2;
        *var_qlo_db3_slot = var_qlo_db3;
        *var_qlo_db4_slot = var_qlo_db4;
        *var_qlo_db5_slot = var_qlo_db5;
        *var_qlo_db6_slot = var_qlo_db6;
        *var_qlo_db7_slot = var_qlo_db7;
        *var_qlo_dn0_slot = var_qlo_dn0;
        *var_qlo_dn1_slot = var_qlo_dn1;
        *var_qlo_dn2_slot = var_qlo_dn2;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_qlo_dn5_slot = var_qlo_dn5;
        *var_qlo_dn6_slot = var_qlo_dn6;
        *var_qlo_dn7_slot = var_qlo_dn7;
        *var_qlo_dn8_slot = var_qlo_dn8;
        *var_qlo_dn9_slot = var_qlo_dn9;
        *var_qlo_rdb0_slot = var_qlo_rdb0;
        *var_qlo_rdb1_slot = var_qlo_rdb1;
        *var_qlo_rdb2_slot = var_qlo_rdb2;
        *var_qlo_rdb3_slot = var_qlo_rdb3;
        *var_qlo_rdb4_slot = var_qlo_rdb4;
        *var_qlo_rdb5_slot = var_qlo_rdb5;
        *var_qlo_rdb6_slot = var_qlo_rdb6;
        *var_qlo_rdb7_slot = var_qlo_rdb7;
        *var_qlo_rdn0_slot = var_qlo_rdn0;
        *var_qlo_rdn1_slot = var_qlo_rdn1;
        *var_qlo_rdn2_slot = var_qlo_rdn2;
        *var_qlo_rdn3_slot = var_qlo_rdn3;
        *var_qlo_rdn4_slot = var_qlo_rdn4;
        *var_qlo_rdn5_slot = var_qlo_rdn5;
        *var_qlo_rdn6_slot = var_qlo_rdn6;
        *var_qlo_rdn7_slot = var_qlo_rdn7;
        *var_qlo_rdn8_slot = var_qlo_rdn8;
        *var_qlo_rdn9_slot = var_qlo_rdn9;
        *var_qlo_rv_slot = var_qlo_rv;
    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        var_cje_t: f64,
        var_cje_t_db0: f64,
        var_cje_t_db1: f64,
        var_cje_t_db2: f64,
        var_cje_t_db3: f64,
        var_cje_t_db4: f64,
        var_cje_t_db5: f64,
        var_cje_t_db6: f64,
        var_cje_t_db7: f64,
        var_cje_t_dn0: f64,
        var_cje_t_dn1: f64,
        var_cje_t_dn2: f64,
        var_cje_t_dn3: f64,
        var_cje_t_dn4: f64,
        var_cje_t_dn5: f64,
        var_cje_t_dn6: f64,
        var_cje_t_dn7: f64,
        var_cje_t_dn8: f64,
        var_cje_t_dn9: f64,
        var_guard16: f64,
        var_vbci: f64,
        var_vbci_db0: f64,
        var_vbci_db1: f64,
        var_vbci_db2: f64,
        var_vbci_db3: f64,
        var_vbci_db4: f64,
        var_vbci_db5: f64,
        var_vbci_db6: f64,
        var_vbci_db7: f64,
        var_vbci_dn0: f64,
        var_vbci_dn1: f64,
        var_vbci_dn2: f64,
        var_vbci_dn3: f64,
        var_vbci_dn4: f64,
        var_vbci_dn5: f64,
        var_vbci_dn6: f64,
        var_vbci_dn7: f64,
        var_vbci_dn8: f64,
        var_vbci_dn9: f64,
        var_vbiei: f64,
        var_vbiei_db0: f64,
        var_vbiei_db1: f64,
        var_vbiei_db2: f64,
        var_vbiei_db3: f64,
        var_vbiei_db4: f64,
        var_vbiei_db5: f64,
        var_vbiei_db6: f64,
        var_vbiei_db7: f64,
        var_vbiei_dn0: f64,
        var_vbiei_dn1: f64,
        var_vbiei_dn2: f64,
        var_vbiei_dn3: f64,
        var_vbiei_dn4: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vbiei_dn7: f64,
        var_vbiei_dn8: f64,
        var_vbiei_dn9: f64,
        var_vjc_t: f64,
        var_vjc_t_db0: f64,
        var_vjc_t_db1: f64,
        var_vjc_t_db2: f64,
        var_vjc_t_db3: f64,
        var_vjc_t_db4: f64,
        var_vjc_t_db5: f64,
        var_vjc_t_db6: f64,
        var_vjc_t_db7: f64,
        var_vjc_t_dn0: f64,
        var_vjc_t_dn1: f64,
        var_vjc_t_dn2: f64,
        var_vjc_t_dn3: f64,
        var_vjc_t_dn4: f64,
        var_vjc_t_dn5: f64,
        var_vjc_t_dn6: f64,
        var_vjc_t_dn7: f64,
        var_vjc_t_dn8: f64,
        var_vjc_t_dn9: f64,
        var_vje_t: f64,
        var_vje_t_db0: f64,
        var_vje_t_db1: f64,
        var_vje_t_db2: f64,
        var_vje_t_db3: f64,
        var_vje_t_db4: f64,
        var_vje_t_db5: f64,
        var_vje_t_db6: f64,
        var_vje_t_db7: f64,
        var_vje_t_dn0: f64,
        var_vje_t_dn1: f64,
        var_vje_t_dn2: f64,
        var_vje_t_dn3: f64,
        var_vje_t_dn4: f64,
        var_vje_t_dn5: f64,
        var_vje_t_dn6: f64,
        var_vje_t_dn7: f64,
        var_vje_t_dn8: f64,
        var_vje_t_dn9: f64,
        var_dv0_slot: &mut f64,
        var_dv0_db0_slot: &mut f64,
        var_dv0_db1_slot: &mut f64,
        var_dv0_db2_slot: &mut f64,
        var_dv0_db3_slot: &mut f64,
        var_dv0_db4_slot: &mut f64,
        var_dv0_db5_slot: &mut f64,
        var_dv0_db6_slot: &mut f64,
        var_dv0_db7_slot: &mut f64,
        var_dv0_dn0_slot: &mut f64,
        var_dv0_dn1_slot: &mut f64,
        var_dv0_dn2_slot: &mut f64,
        var_dv0_dn3_slot: &mut f64,
        var_dv0_dn4_slot: &mut f64,
        var_dv0_dn5_slot: &mut f64,
        var_dv0_dn6_slot: &mut f64,
        var_dv0_dn7_slot: &mut f64,
        var_dv0_dn8_slot: &mut f64,
        var_dv0_dn9_slot: &mut f64,
        var_dv0_rdb0_slot: &mut f64,
        var_dv0_rdb1_slot: &mut f64,
        var_dv0_rdb2_slot: &mut f64,
        var_dv0_rdb3_slot: &mut f64,
        var_dv0_rdb4_slot: &mut f64,
        var_dv0_rdb5_slot: &mut f64,
        var_dv0_rdb6_slot: &mut f64,
        var_dv0_rdb7_slot: &mut f64,
        var_dv0_rdn0_slot: &mut f64,
        var_dv0_rdn1_slot: &mut f64,
        var_dv0_rdn2_slot: &mut f64,
        var_dv0_rdn3_slot: &mut f64,
        var_dv0_rdn4_slot: &mut f64,
        var_dv0_rdn5_slot: &mut f64,
        var_dv0_rdn6_slot: &mut f64,
        var_dv0_rdn7_slot: &mut f64,
        var_dv0_rdn8_slot: &mut f64,
        var_dv0_rdn9_slot: &mut f64,
        var_dv0_rv_slot: &mut f64,
        var_dvh_slot: &mut f64,
        var_dvh_db0_slot: &mut f64,
        var_dvh_db1_slot: &mut f64,
        var_dvh_db2_slot: &mut f64,
        var_dvh_db3_slot: &mut f64,
        var_dvh_db4_slot: &mut f64,
        var_dvh_db5_slot: &mut f64,
        var_dvh_db6_slot: &mut f64,
        var_dvh_db7_slot: &mut f64,
        var_dvh_dn0_slot: &mut f64,
        var_dvh_dn1_slot: &mut f64,
        var_dvh_dn2_slot: &mut f64,
        var_dvh_dn3_slot: &mut f64,
        var_dvh_dn4_slot: &mut f64,
        var_dvh_dn5_slot: &mut f64,
        var_dvh_dn6_slot: &mut f64,
        var_dvh_dn7_slot: &mut f64,
        var_dvh_dn8_slot: &mut f64,
        var_dvh_dn9_slot: &mut f64,
        var_dvh_rdb0_slot: &mut f64,
        var_dvh_rdb1_slot: &mut f64,
        var_dvh_rdb2_slot: &mut f64,
        var_dvh_rdb3_slot: &mut f64,
        var_dvh_rdb4_slot: &mut f64,
        var_dvh_rdb5_slot: &mut f64,
        var_dvh_rdb6_slot: &mut f64,
        var_dvh_rdb7_slot: &mut f64,
        var_dvh_rdn0_slot: &mut f64,
        var_dvh_rdn1_slot: &mut f64,
        var_dvh_rdn2_slot: &mut f64,
        var_dvh_rdn3_slot: &mut f64,
        var_dvh_rdn4_slot: &mut f64,
        var_dvh_rdn5_slot: &mut f64,
        var_dvh_rdn6_slot: &mut f64,
        var_dvh_rdn7_slot: &mut f64,
        var_dvh_rdn8_slot: &mut f64,
        var_dvh_rdn9_slot: &mut f64,
        var_dvh_rv_slot: &mut f64,
        var_guard17_slot: &mut f64,
        var_guard17_db0_slot: &mut f64,
        var_guard17_db1_slot: &mut f64,
        var_guard17_db2_slot: &mut f64,
        var_guard17_db3_slot: &mut f64,
        var_guard17_db4_slot: &mut f64,
        var_guard17_db5_slot: &mut f64,
        var_guard17_db6_slot: &mut f64,
        var_guard17_db7_slot: &mut f64,
        var_guard17_dn0_slot: &mut f64,
        var_guard17_dn1_slot: &mut f64,
        var_guard17_dn2_slot: &mut f64,
        var_guard17_dn3_slot: &mut f64,
        var_guard17_dn4_slot: &mut f64,
        var_guard17_dn5_slot: &mut f64,
        var_guard17_dn6_slot: &mut f64,
        var_guard17_dn7_slot: &mut f64,
        var_guard17_dn8_slot: &mut f64,
        var_guard17_dn9_slot: &mut f64,
        var_guard17_rdb0_slot: &mut f64,
        var_guard17_rdb1_slot: &mut f64,
        var_guard17_rdb2_slot: &mut f64,
        var_guard17_rdb3_slot: &mut f64,
        var_guard17_rdb4_slot: &mut f64,
        var_guard17_rdb5_slot: &mut f64,
        var_guard17_rdb6_slot: &mut f64,
        var_guard17_rdb7_slot: &mut f64,
        var_guard17_rdn0_slot: &mut f64,
        var_guard17_rdn1_slot: &mut f64,
        var_guard17_rdn2_slot: &mut f64,
        var_guard17_rdn3_slot: &mut f64,
        var_guard17_rdn4_slot: &mut f64,
        var_guard17_rdn5_slot: &mut f64,
        var_guard17_rdn6_slot: &mut f64,
        var_guard17_rdn7_slot: &mut f64,
        var_guard17_rdn8_slot: &mut f64,
        var_guard17_rdn9_slot: &mut f64,
        var_guard17_rv_slot: &mut f64,
        var_pwq_slot: &mut f64,
        var_pwq_db0_slot: &mut f64,
        var_pwq_db1_slot: &mut f64,
        var_pwq_db2_slot: &mut f64,
        var_pwq_db3_slot: &mut f64,
        var_pwq_db4_slot: &mut f64,
        var_pwq_db5_slot: &mut f64,
        var_pwq_db6_slot: &mut f64,
        var_pwq_db7_slot: &mut f64,
        var_pwq_dn0_slot: &mut f64,
        var_pwq_dn1_slot: &mut f64,
        var_pwq_dn2_slot: &mut f64,
        var_pwq_dn3_slot: &mut f64,
        var_pwq_dn4_slot: &mut f64,
        var_pwq_dn5_slot: &mut f64,
        var_pwq_dn6_slot: &mut f64,
        var_pwq_dn7_slot: &mut f64,
        var_pwq_dn8_slot: &mut f64,
        var_pwq_dn9_slot: &mut f64,
        var_pwq_rdb0_slot: &mut f64,
        var_pwq_rdb1_slot: &mut f64,
        var_pwq_rdb2_slot: &mut f64,
        var_pwq_rdb3_slot: &mut f64,
        var_pwq_rdb4_slot: &mut f64,
        var_pwq_rdb5_slot: &mut f64,
        var_pwq_rdb6_slot: &mut f64,
        var_pwq_rdb7_slot: &mut f64,
        var_pwq_rdn0_slot: &mut f64,
        var_pwq_rdn1_slot: &mut f64,
        var_pwq_rdn2_slot: &mut f64,
        var_pwq_rdn3_slot: &mut f64,
        var_pwq_rdn4_slot: &mut f64,
        var_pwq_rdn5_slot: &mut f64,
        var_pwq_rdn6_slot: &mut f64,
        var_pwq_rdn7_slot: &mut f64,
        var_pwq_rdn8_slot: &mut f64,
        var_pwq_rdn9_slot: &mut f64,
        var_pwq_rv_slot: &mut f64,
        var_qhi_slot: &mut f64,
        var_qhi_db0_slot: &mut f64,
        var_qhi_db1_slot: &mut f64,
        var_qhi_db2_slot: &mut f64,
        var_qhi_db3_slot: &mut f64,
        var_qhi_db4_slot: &mut f64,
        var_qhi_db5_slot: &mut f64,
        var_qhi_db6_slot: &mut f64,
        var_qhi_db7_slot: &mut f64,
        var_qhi_dn0_slot: &mut f64,
        var_qhi_dn1_slot: &mut f64,
        var_qhi_dn2_slot: &mut f64,
        var_qhi_dn3_slot: &mut f64,
        var_qhi_dn4_slot: &mut f64,
        var_qhi_dn5_slot: &mut f64,
        var_qhi_dn6_slot: &mut f64,
        var_qhi_dn7_slot: &mut f64,
        var_qhi_dn8_slot: &mut f64,
        var_qhi_dn9_slot: &mut f64,
        var_qhi_rdb0_slot: &mut f64,
        var_qhi_rdb1_slot: &mut f64,
        var_qhi_rdb2_slot: &mut f64,
        var_qhi_rdb3_slot: &mut f64,
        var_qhi_rdb4_slot: &mut f64,
        var_qhi_rdb5_slot: &mut f64,
        var_qhi_rdb6_slot: &mut f64,
        var_qhi_rdb7_slot: &mut f64,
        var_qhi_rdn0_slot: &mut f64,
        var_qhi_rdn1_slot: &mut f64,
        var_qhi_rdn2_slot: &mut f64,
        var_qhi_rdn3_slot: &mut f64,
        var_qhi_rdn4_slot: &mut f64,
        var_qhi_rdn5_slot: &mut f64,
        var_qhi_rdn6_slot: &mut f64,
        var_qhi_rdn7_slot: &mut f64,
        var_qhi_rdn8_slot: &mut f64,
        var_qhi_rdn9_slot: &mut f64,
        var_qhi_rv_slot: &mut f64,
        var_qje_slot: &mut f64,
        var_qje_db0_slot: &mut f64,
        var_qje_db1_slot: &mut f64,
        var_qje_db2_slot: &mut f64,
        var_qje_db3_slot: &mut f64,
        var_qje_db4_slot: &mut f64,
        var_qje_db5_slot: &mut f64,
        var_qje_db6_slot: &mut f64,
        var_qje_db7_slot: &mut f64,
        var_qje_dn0_slot: &mut f64,
        var_qje_dn1_slot: &mut f64,
        var_qje_dn2_slot: &mut f64,
        var_qje_dn3_slot: &mut f64,
        var_qje_dn4_slot: &mut f64,
        var_qje_dn5_slot: &mut f64,
        var_qje_dn6_slot: &mut f64,
        var_qje_dn7_slot: &mut f64,
        var_qje_dn8_slot: &mut f64,
        var_qje_dn9_slot: &mut f64,
        var_qje_rdb0_slot: &mut f64,
        var_qje_rdb1_slot: &mut f64,
        var_qje_rdb2_slot: &mut f64,
        var_qje_rdb3_slot: &mut f64,
        var_qje_rdb4_slot: &mut f64,
        var_qje_rdb5_slot: &mut f64,
        var_qje_rdb6_slot: &mut f64,
        var_qje_rdb7_slot: &mut f64,
        var_qje_rdn0_slot: &mut f64,
        var_qje_rdn1_slot: &mut f64,
        var_qje_rdn2_slot: &mut f64,
        var_qje_rdn3_slot: &mut f64,
        var_qje_rdn4_slot: &mut f64,
        var_qje_rdn5_slot: &mut f64,
        var_qje_rdn6_slot: &mut f64,
        var_qje_rdn7_slot: &mut f64,
        var_qje_rdn8_slot: &mut f64,
        var_qje_rdn9_slot: &mut f64,
        var_qje_rv_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo_db0_slot: &mut f64,
        var_qlo_db1_slot: &mut f64,
        var_qlo_db2_slot: &mut f64,
        var_qlo_db3_slot: &mut f64,
        var_qlo_db4_slot: &mut f64,
        var_qlo_db5_slot: &mut f64,
        var_qlo_db6_slot: &mut f64,
        var_qlo_db7_slot: &mut f64,
        var_qlo_dn0_slot: &mut f64,
        var_qlo_dn1_slot: &mut f64,
        var_qlo_dn2_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_qlo_dn5_slot: &mut f64,
        var_qlo_dn6_slot: &mut f64,
        var_qlo_dn7_slot: &mut f64,
        var_qlo_dn8_slot: &mut f64,
        var_qlo_dn9_slot: &mut f64,
        var_qlo_rdb0_slot: &mut f64,
        var_qlo_rdb1_slot: &mut f64,
        var_qlo_rdb2_slot: &mut f64,
        var_qlo_rdb3_slot: &mut f64,
        var_qlo_rdb4_slot: &mut f64,
        var_qlo_rdb5_slot: &mut f64,
        var_qlo_rdb6_slot: &mut f64,
        var_qlo_rdb7_slot: &mut f64,
        var_qlo_rdn0_slot: &mut f64,
        var_qlo_rdn1_slot: &mut f64,
        var_qlo_rdn2_slot: &mut f64,
        var_qlo_rdn3_slot: &mut f64,
        var_qlo_rdn4_slot: &mut f64,
        var_qlo_rdn5_slot: &mut f64,
        var_qlo_rdn6_slot: &mut f64,
        var_qlo_rdn7_slot: &mut f64,
        var_qlo_rdn8_slot: &mut f64,
        var_qlo_rdn9_slot: &mut f64,
        var_qlo_rv_slot: &mut f64,
    ) {
        let mut var_dv0: f64 = *var_dv0_slot;
        let mut var_dv0_db0: f64 = *var_dv0_db0_slot;
        let mut var_dv0_db1: f64 = *var_dv0_db1_slot;
        let mut var_dv0_db2: f64 = *var_dv0_db2_slot;
        let mut var_dv0_db3: f64 = *var_dv0_db3_slot;
        let mut var_dv0_db4: f64 = *var_dv0_db4_slot;
        let mut var_dv0_db5: f64 = *var_dv0_db5_slot;
        let mut var_dv0_db6: f64 = *var_dv0_db6_slot;
        let mut var_dv0_db7: f64 = *var_dv0_db7_slot;
        let mut var_dv0_dn0: f64 = *var_dv0_dn0_slot;
        let mut var_dv0_dn1: f64 = *var_dv0_dn1_slot;
        let mut var_dv0_dn2: f64 = *var_dv0_dn2_slot;
        let mut var_dv0_dn3: f64 = *var_dv0_dn3_slot;
        let mut var_dv0_dn4: f64 = *var_dv0_dn4_slot;
        let mut var_dv0_dn5: f64 = *var_dv0_dn5_slot;
        let mut var_dv0_dn6: f64 = *var_dv0_dn6_slot;
        let mut var_dv0_dn7: f64 = *var_dv0_dn7_slot;
        let mut var_dv0_dn8: f64 = *var_dv0_dn8_slot;
        let mut var_dv0_dn9: f64 = *var_dv0_dn9_slot;
        let mut var_dv0_rdb0: f64 = *var_dv0_rdb0_slot;
        let mut var_dv0_rdb1: f64 = *var_dv0_rdb1_slot;
        let mut var_dv0_rdb2: f64 = *var_dv0_rdb2_slot;
        let mut var_dv0_rdb3: f64 = *var_dv0_rdb3_slot;
        let mut var_dv0_rdb4: f64 = *var_dv0_rdb4_slot;
        let mut var_dv0_rdb5: f64 = *var_dv0_rdb5_slot;
        let mut var_dv0_rdb6: f64 = *var_dv0_rdb6_slot;
        let mut var_dv0_rdb7: f64 = *var_dv0_rdb7_slot;
        let mut var_dv0_rdn0: f64 = *var_dv0_rdn0_slot;
        let mut var_dv0_rdn1: f64 = *var_dv0_rdn1_slot;
        let mut var_dv0_rdn2: f64 = *var_dv0_rdn2_slot;
        let mut var_dv0_rdn3: f64 = *var_dv0_rdn3_slot;
        let mut var_dv0_rdn4: f64 = *var_dv0_rdn4_slot;
        let mut var_dv0_rdn5: f64 = *var_dv0_rdn5_slot;
        let mut var_dv0_rdn6: f64 = *var_dv0_rdn6_slot;
        let mut var_dv0_rdn7: f64 = *var_dv0_rdn7_slot;
        let mut var_dv0_rdn8: f64 = *var_dv0_rdn8_slot;
        let mut var_dv0_rdn9: f64 = *var_dv0_rdn9_slot;
        let mut var_dv0_rv: f64 = *var_dv0_rv_slot;
        let mut var_dvh: f64 = *var_dvh_slot;
        let mut var_dvh_db0: f64 = *var_dvh_db0_slot;
        let mut var_dvh_db1: f64 = *var_dvh_db1_slot;
        let mut var_dvh_db2: f64 = *var_dvh_db2_slot;
        let mut var_dvh_db3: f64 = *var_dvh_db3_slot;
        let mut var_dvh_db4: f64 = *var_dvh_db4_slot;
        let mut var_dvh_db5: f64 = *var_dvh_db5_slot;
        let mut var_dvh_db6: f64 = *var_dvh_db6_slot;
        let mut var_dvh_db7: f64 = *var_dvh_db7_slot;
        let mut var_dvh_dn0: f64 = *var_dvh_dn0_slot;
        let mut var_dvh_dn1: f64 = *var_dvh_dn1_slot;
        let mut var_dvh_dn2: f64 = *var_dvh_dn2_slot;
        let mut var_dvh_dn3: f64 = *var_dvh_dn3_slot;
        let mut var_dvh_dn4: f64 = *var_dvh_dn4_slot;
        let mut var_dvh_dn5: f64 = *var_dvh_dn5_slot;
        let mut var_dvh_dn6: f64 = *var_dvh_dn6_slot;
        let mut var_dvh_dn7: f64 = *var_dvh_dn7_slot;
        let mut var_dvh_dn8: f64 = *var_dvh_dn8_slot;
        let mut var_dvh_dn9: f64 = *var_dvh_dn9_slot;
        let mut var_dvh_rdb0: f64 = *var_dvh_rdb0_slot;
        let mut var_dvh_rdb1: f64 = *var_dvh_rdb1_slot;
        let mut var_dvh_rdb2: f64 = *var_dvh_rdb2_slot;
        let mut var_dvh_rdb3: f64 = *var_dvh_rdb3_slot;
        let mut var_dvh_rdb4: f64 = *var_dvh_rdb4_slot;
        let mut var_dvh_rdb5: f64 = *var_dvh_rdb5_slot;
        let mut var_dvh_rdb6: f64 = *var_dvh_rdb6_slot;
        let mut var_dvh_rdb7: f64 = *var_dvh_rdb7_slot;
        let mut var_dvh_rdn0: f64 = *var_dvh_rdn0_slot;
        let mut var_dvh_rdn1: f64 = *var_dvh_rdn1_slot;
        let mut var_dvh_rdn2: f64 = *var_dvh_rdn2_slot;
        let mut var_dvh_rdn3: f64 = *var_dvh_rdn3_slot;
        let mut var_dvh_rdn4: f64 = *var_dvh_rdn4_slot;
        let mut var_dvh_rdn5: f64 = *var_dvh_rdn5_slot;
        let mut var_dvh_rdn6: f64 = *var_dvh_rdn6_slot;
        let mut var_dvh_rdn7: f64 = *var_dvh_rdn7_slot;
        let mut var_dvh_rdn8: f64 = *var_dvh_rdn8_slot;
        let mut var_dvh_rdn9: f64 = *var_dvh_rdn9_slot;
        let mut var_dvh_rv: f64 = *var_dvh_rv_slot;
        let mut var_guard17: f64 = *var_guard17_slot;
        let mut var_guard17_db0: f64 = *var_guard17_db0_slot;
        let mut var_guard17_db1: f64 = *var_guard17_db1_slot;
        let mut var_guard17_db2: f64 = *var_guard17_db2_slot;
        let mut var_guard17_db3: f64 = *var_guard17_db3_slot;
        let mut var_guard17_db4: f64 = *var_guard17_db4_slot;
        let mut var_guard17_db5: f64 = *var_guard17_db5_slot;
        let mut var_guard17_db6: f64 = *var_guard17_db6_slot;
        let mut var_guard17_db7: f64 = *var_guard17_db7_slot;
        let mut var_guard17_dn0: f64 = *var_guard17_dn0_slot;
        let mut var_guard17_dn1: f64 = *var_guard17_dn1_slot;
        let mut var_guard17_dn2: f64 = *var_guard17_dn2_slot;
        let mut var_guard17_dn3: f64 = *var_guard17_dn3_slot;
        let mut var_guard17_dn4: f64 = *var_guard17_dn4_slot;
        let mut var_guard17_dn5: f64 = *var_guard17_dn5_slot;
        let mut var_guard17_dn6: f64 = *var_guard17_dn6_slot;
        let mut var_guard17_dn7: f64 = *var_guard17_dn7_slot;
        let mut var_guard17_dn8: f64 = *var_guard17_dn8_slot;
        let mut var_guard17_dn9: f64 = *var_guard17_dn9_slot;
        let mut var_guard17_rdb0: f64 = *var_guard17_rdb0_slot;
        let mut var_guard17_rdb1: f64 = *var_guard17_rdb1_slot;
        let mut var_guard17_rdb2: f64 = *var_guard17_rdb2_slot;
        let mut var_guard17_rdb3: f64 = *var_guard17_rdb3_slot;
        let mut var_guard17_rdb4: f64 = *var_guard17_rdb4_slot;
        let mut var_guard17_rdb5: f64 = *var_guard17_rdb5_slot;
        let mut var_guard17_rdb6: f64 = *var_guard17_rdb6_slot;
        let mut var_guard17_rdb7: f64 = *var_guard17_rdb7_slot;
        let mut var_guard17_rdn0: f64 = *var_guard17_rdn0_slot;
        let mut var_guard17_rdn1: f64 = *var_guard17_rdn1_slot;
        let mut var_guard17_rdn2: f64 = *var_guard17_rdn2_slot;
        let mut var_guard17_rdn3: f64 = *var_guard17_rdn3_slot;
        let mut var_guard17_rdn4: f64 = *var_guard17_rdn4_slot;
        let mut var_guard17_rdn5: f64 = *var_guard17_rdn5_slot;
        let mut var_guard17_rdn6: f64 = *var_guard17_rdn6_slot;
        let mut var_guard17_rdn7: f64 = *var_guard17_rdn7_slot;
        let mut var_guard17_rdn8: f64 = *var_guard17_rdn8_slot;
        let mut var_guard17_rdn9: f64 = *var_guard17_rdn9_slot;
        let mut var_guard17_rv: f64 = *var_guard17_rv_slot;
        let mut var_pwq: f64 = *var_pwq_slot;
        let mut var_pwq_db0: f64 = *var_pwq_db0_slot;
        let mut var_pwq_db1: f64 = *var_pwq_db1_slot;
        let mut var_pwq_db2: f64 = *var_pwq_db2_slot;
        let mut var_pwq_db3: f64 = *var_pwq_db3_slot;
        let mut var_pwq_db4: f64 = *var_pwq_db4_slot;
        let mut var_pwq_db5: f64 = *var_pwq_db5_slot;
        let mut var_pwq_db6: f64 = *var_pwq_db6_slot;
        let mut var_pwq_db7: f64 = *var_pwq_db7_slot;
        let mut var_pwq_dn0: f64 = *var_pwq_dn0_slot;
        let mut var_pwq_dn1: f64 = *var_pwq_dn1_slot;
        let mut var_pwq_dn2: f64 = *var_pwq_dn2_slot;
        let mut var_pwq_dn3: f64 = *var_pwq_dn3_slot;
        let mut var_pwq_dn4: f64 = *var_pwq_dn4_slot;
        let mut var_pwq_dn5: f64 = *var_pwq_dn5_slot;
        let mut var_pwq_dn6: f64 = *var_pwq_dn6_slot;
        let mut var_pwq_dn7: f64 = *var_pwq_dn7_slot;
        let mut var_pwq_dn8: f64 = *var_pwq_dn8_slot;
        let mut var_pwq_dn9: f64 = *var_pwq_dn9_slot;
        let mut var_pwq_rdb0: f64 = *var_pwq_rdb0_slot;
        let mut var_pwq_rdb1: f64 = *var_pwq_rdb1_slot;
        let mut var_pwq_rdb2: f64 = *var_pwq_rdb2_slot;
        let mut var_pwq_rdb3: f64 = *var_pwq_rdb3_slot;
        let mut var_pwq_rdb4: f64 = *var_pwq_rdb4_slot;
        let mut var_pwq_rdb5: f64 = *var_pwq_rdb5_slot;
        let mut var_pwq_rdb6: f64 = *var_pwq_rdb6_slot;
        let mut var_pwq_rdb7: f64 = *var_pwq_rdb7_slot;
        let mut var_pwq_rdn0: f64 = *var_pwq_rdn0_slot;
        let mut var_pwq_rdn1: f64 = *var_pwq_rdn1_slot;
        let mut var_pwq_rdn2: f64 = *var_pwq_rdn2_slot;
        let mut var_pwq_rdn3: f64 = *var_pwq_rdn3_slot;
        let mut var_pwq_rdn4: f64 = *var_pwq_rdn4_slot;
        let mut var_pwq_rdn5: f64 = *var_pwq_rdn5_slot;
        let mut var_pwq_rdn6: f64 = *var_pwq_rdn6_slot;
        let mut var_pwq_rdn7: f64 = *var_pwq_rdn7_slot;
        let mut var_pwq_rdn8: f64 = *var_pwq_rdn8_slot;
        let mut var_pwq_rdn9: f64 = *var_pwq_rdn9_slot;
        let mut var_pwq_rv: f64 = *var_pwq_rv_slot;
        let mut var_qhi: f64 = *var_qhi_slot;
        let mut var_qhi_db0: f64 = *var_qhi_db0_slot;
        let mut var_qhi_db1: f64 = *var_qhi_db1_slot;
        let mut var_qhi_db2: f64 = *var_qhi_db2_slot;
        let mut var_qhi_db3: f64 = *var_qhi_db3_slot;
        let mut var_qhi_db4: f64 = *var_qhi_db4_slot;
        let mut var_qhi_db5: f64 = *var_qhi_db5_slot;
        let mut var_qhi_db6: f64 = *var_qhi_db6_slot;
        let mut var_qhi_db7: f64 = *var_qhi_db7_slot;
        let mut var_qhi_dn0: f64 = *var_qhi_dn0_slot;
        let mut var_qhi_dn1: f64 = *var_qhi_dn1_slot;
        let mut var_qhi_dn2: f64 = *var_qhi_dn2_slot;
        let mut var_qhi_dn3: f64 = *var_qhi_dn3_slot;
        let mut var_qhi_dn4: f64 = *var_qhi_dn4_slot;
        let mut var_qhi_dn5: f64 = *var_qhi_dn5_slot;
        let mut var_qhi_dn6: f64 = *var_qhi_dn6_slot;
        let mut var_qhi_dn7: f64 = *var_qhi_dn7_slot;
        let mut var_qhi_dn8: f64 = *var_qhi_dn8_slot;
        let mut var_qhi_dn9: f64 = *var_qhi_dn9_slot;
        let mut var_qhi_rdb0: f64 = *var_qhi_rdb0_slot;
        let mut var_qhi_rdb1: f64 = *var_qhi_rdb1_slot;
        let mut var_qhi_rdb2: f64 = *var_qhi_rdb2_slot;
        let mut var_qhi_rdb3: f64 = *var_qhi_rdb3_slot;
        let mut var_qhi_rdb4: f64 = *var_qhi_rdb4_slot;
        let mut var_qhi_rdb5: f64 = *var_qhi_rdb5_slot;
        let mut var_qhi_rdb6: f64 = *var_qhi_rdb6_slot;
        let mut var_qhi_rdb7: f64 = *var_qhi_rdb7_slot;
        let mut var_qhi_rdn0: f64 = *var_qhi_rdn0_slot;
        let mut var_qhi_rdn1: f64 = *var_qhi_rdn1_slot;
        let mut var_qhi_rdn2: f64 = *var_qhi_rdn2_slot;
        let mut var_qhi_rdn3: f64 = *var_qhi_rdn3_slot;
        let mut var_qhi_rdn4: f64 = *var_qhi_rdn4_slot;
        let mut var_qhi_rdn5: f64 = *var_qhi_rdn5_slot;
        let mut var_qhi_rdn6: f64 = *var_qhi_rdn6_slot;
        let mut var_qhi_rdn7: f64 = *var_qhi_rdn7_slot;
        let mut var_qhi_rdn8: f64 = *var_qhi_rdn8_slot;
        let mut var_qhi_rdn9: f64 = *var_qhi_rdn9_slot;
        let mut var_qhi_rv: f64 = *var_qhi_rv_slot;
        let mut var_qje: f64 = *var_qje_slot;
        let mut var_qje_db0: f64 = *var_qje_db0_slot;
        let mut var_qje_db1: f64 = *var_qje_db1_slot;
        let mut var_qje_db2: f64 = *var_qje_db2_slot;
        let mut var_qje_db3: f64 = *var_qje_db3_slot;
        let mut var_qje_db4: f64 = *var_qje_db4_slot;
        let mut var_qje_db5: f64 = *var_qje_db5_slot;
        let mut var_qje_db6: f64 = *var_qje_db6_slot;
        let mut var_qje_db7: f64 = *var_qje_db7_slot;
        let mut var_qje_dn0: f64 = *var_qje_dn0_slot;
        let mut var_qje_dn1: f64 = *var_qje_dn1_slot;
        let mut var_qje_dn2: f64 = *var_qje_dn2_slot;
        let mut var_qje_dn3: f64 = *var_qje_dn3_slot;
        let mut var_qje_dn4: f64 = *var_qje_dn4_slot;
        let mut var_qje_dn5: f64 = *var_qje_dn5_slot;
        let mut var_qje_dn6: f64 = *var_qje_dn6_slot;
        let mut var_qje_dn7: f64 = *var_qje_dn7_slot;
        let mut var_qje_dn8: f64 = *var_qje_dn8_slot;
        let mut var_qje_dn9: f64 = *var_qje_dn9_slot;
        let mut var_qje_rdb0: f64 = *var_qje_rdb0_slot;
        let mut var_qje_rdb1: f64 = *var_qje_rdb1_slot;
        let mut var_qje_rdb2: f64 = *var_qje_rdb2_slot;
        let mut var_qje_rdb3: f64 = *var_qje_rdb3_slot;
        let mut var_qje_rdb4: f64 = *var_qje_rdb4_slot;
        let mut var_qje_rdb5: f64 = *var_qje_rdb5_slot;
        let mut var_qje_rdb6: f64 = *var_qje_rdb6_slot;
        let mut var_qje_rdb7: f64 = *var_qje_rdb7_slot;
        let mut var_qje_rdn0: f64 = *var_qje_rdn0_slot;
        let mut var_qje_rdn1: f64 = *var_qje_rdn1_slot;
        let mut var_qje_rdn2: f64 = *var_qje_rdn2_slot;
        let mut var_qje_rdn3: f64 = *var_qje_rdn3_slot;
        let mut var_qje_rdn4: f64 = *var_qje_rdn4_slot;
        let mut var_qje_rdn5: f64 = *var_qje_rdn5_slot;
        let mut var_qje_rdn6: f64 = *var_qje_rdn6_slot;
        let mut var_qje_rdn7: f64 = *var_qje_rdn7_slot;
        let mut var_qje_rdn8: f64 = *var_qje_rdn8_slot;
        let mut var_qje_rdn9: f64 = *var_qje_rdn9_slot;
        let mut var_qje_rv: f64 = *var_qje_rv_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo_db0: f64 = *var_qlo_db0_slot;
        let mut var_qlo_db1: f64 = *var_qlo_db1_slot;
        let mut var_qlo_db2: f64 = *var_qlo_db2_slot;
        let mut var_qlo_db3: f64 = *var_qlo_db3_slot;
        let mut var_qlo_db4: f64 = *var_qlo_db4_slot;
        let mut var_qlo_db5: f64 = *var_qlo_db5_slot;
        let mut var_qlo_db6: f64 = *var_qlo_db6_slot;
        let mut var_qlo_db7: f64 = *var_qlo_db7_slot;
        let mut var_qlo_dn0: f64 = *var_qlo_dn0_slot;
        let mut var_qlo_dn1: f64 = *var_qlo_dn1_slot;
        let mut var_qlo_dn2: f64 = *var_qlo_dn2_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_qlo_dn5: f64 = *var_qlo_dn5_slot;
        let mut var_qlo_dn6: f64 = *var_qlo_dn6_slot;
        let mut var_qlo_dn7: f64 = *var_qlo_dn7_slot;
        let mut var_qlo_dn8: f64 = *var_qlo_dn8_slot;
        let mut var_qlo_dn9: f64 = *var_qlo_dn9_slot;
        let mut var_qlo_rdb0: f64 = *var_qlo_rdb0_slot;
        let mut var_qlo_rdb1: f64 = *var_qlo_rdb1_slot;
        let mut var_qlo_rdb2: f64 = *var_qlo_rdb2_slot;
        let mut var_qlo_rdb3: f64 = *var_qlo_rdb3_slot;
        let mut var_qlo_rdb4: f64 = *var_qlo_rdb4_slot;
        let mut var_qlo_rdb5: f64 = *var_qlo_rdb5_slot;
        let mut var_qlo_rdb6: f64 = *var_qlo_rdb6_slot;
        let mut var_qlo_rdb7: f64 = *var_qlo_rdb7_slot;
        let mut var_qlo_rdn0: f64 = *var_qlo_rdn0_slot;
        let mut var_qlo_rdn1: f64 = *var_qlo_rdn1_slot;
        let mut var_qlo_rdn2: f64 = *var_qlo_rdn2_slot;
        let mut var_qlo_rdn3: f64 = *var_qlo_rdn3_slot;
        let mut var_qlo_rdn4: f64 = *var_qlo_rdn4_slot;
        let mut var_qlo_rdn5: f64 = *var_qlo_rdn5_slot;
        let mut var_qlo_rdn6: f64 = *var_qlo_rdn6_slot;
        let mut var_qlo_rdn7: f64 = *var_qlo_rdn7_slot;
        let mut var_qlo_rdn8: f64 = *var_qlo_rdn8_slot;
        let mut var_qlo_rdn9: f64 = *var_qlo_rdn9_slot;
        let mut var_qlo_rv: f64 = *var_qlo_rv_slot;

        let (assign1660_e1977, assign1660_e1977_d_n0, assign1660_e1977_d_n1, assign1660_e1977_d_n2, assign1660_e1977_d_n3, assign1660_e1977_d_n4, assign1660_e1977_d_n5, assign1660_e1977_d_n6, assign1660_e1977_d_n7, assign1660_e1977_d_n8, assign1660_e1977_d_n9, assign1660_e1977_d_b0, assign1660_e1977_d_b1, assign1660_e1977_d_b2, assign1660_e1977_d_b3, assign1660_e1977_d_b4, assign1660_e1977_d_b5, assign1660_e1977_d_b6, assign1660_e1977_d_b7,) = {
    if (var_guard16 == 0.0) {
        let assign1660_e1961: f64 = (1.0 - p.p18);
        let assign1660_e1965: f64 = (var_vbiei / var_vje_t);
        let assign1660_e1966: f64 = (1.0 - assign1660_e1965);
        let assign1660_e1967: f64 = (assign1660_e1966).ln();
        let assign1660_e1968: f64 = (assign1660_e1961 * assign1660_e1967);
        let assign1660_e1969: f64 = (assign1660_e1968).exp();
        let assign1660_e1970: f64 = (1.0 - assign1660_e1969);
        let assign1660_e1971: f64 = (var_vje_t * assign1660_e1970);
        let assign1660_e1974: f64 = (1.0 - p.p18);
        let assign1660_e1975: f64 = (assign1660_e1971 / assign1660_e1974);
        (assign1660_e1975, (((var_vje_t_dn0 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn0 * var_vje_t) - (var_vbiei * var_vje_t_dn0)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn1 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn1 * var_vje_t) - (var_vbiei * var_vje_t_dn1)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn2 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn2 * var_vje_t) - (var_vbiei * var_vje_t_dn2)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn3 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn3 * var_vje_t) - (var_vbiei * var_vje_t_dn3)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn4 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn4 * var_vje_t) - (var_vbiei * var_vje_t_dn4)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn5 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn5 * var_vje_t) - (var_vbiei * var_vje_t_dn5)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn6 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn6 * var_vje_t) - (var_vbiei * var_vje_t_dn6)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn7 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn7 * var_vje_t) - (var_vbiei * var_vje_t_dn7)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn8 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn8 * var_vje_t) - (var_vbiei * var_vje_t_dn8)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn9 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn9 * var_vje_t) - (var_vbiei * var_vje_t_dn9)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db0 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db0 * var_vje_t) - (var_vbiei * var_vje_t_db0)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db1 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db1 * var_vje_t) - (var_vbiei * var_vje_t_db1)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db2 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db2 * var_vje_t) - (var_vbiei * var_vje_t_db2)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db3 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db3 * var_vje_t) - (var_vbiei * var_vje_t_db3)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db4 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db4 * var_vje_t) - (var_vbiei * var_vje_t_db4)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db5 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db5 * var_vje_t) - (var_vbiei * var_vje_t_db5)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db6 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db6 * var_vje_t) - (var_vbiei * var_vje_t_db6)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db7 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db7 * var_vje_t) - (var_vbiei * var_vje_t_db7)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7,)
    }
};
        var_qlo = assign1660_e1977;
        var_qlo_dn0 = assign1660_e1977_d_n0;
        var_qlo_dn1 = assign1660_e1977_d_n1;
        var_qlo_dn2 = assign1660_e1977_d_n2;
        var_qlo_dn3 = assign1660_e1977_d_n3;
        var_qlo_dn4 = assign1660_e1977_d_n4;
        var_qlo_dn5 = assign1660_e1977_d_n5;
        var_qlo_dn6 = assign1660_e1977_d_n6;
        var_qlo_dn7 = assign1660_e1977_d_n7;
        var_qlo_dn8 = assign1660_e1977_d_n8;
        var_qlo_dn9 = assign1660_e1977_d_n9;
        var_qlo_db0 = assign1660_e1977_d_b0;
        var_qlo_db1 = assign1660_e1977_d_b1;
        var_qlo_db2 = assign1660_e1977_d_b2;
        var_qlo_db3 = assign1660_e1977_d_b3;
        var_qlo_db4 = assign1660_e1977_d_b4;
        var_qlo_db5 = assign1660_e1977_d_b5;
        var_qlo_db6 = assign1660_e1977_d_b6;
        var_qlo_db7 = assign1660_e1977_d_b7;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdn7 = 0.0;
        var_qlo_rdn8 = 0.0;
        var_qlo_rdn9 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;
        var_qlo_rdb7 = 0.0;

        let (assign1670_e1982, assign1670_e1982_d_n0, assign1670_e1982_d_n1, assign1670_e1982_d_n2, assign1670_e1982_d_n3, assign1670_e1982_d_n4, assign1670_e1982_d_n5, assign1670_e1982_d_n6, assign1670_e1982_d_n7, assign1670_e1982_d_n8, assign1670_e1982_d_n9, assign1670_e1982_d_b0, assign1670_e1982_d_b1, assign1670_e1982_d_b2, assign1670_e1982_d_b3, assign1670_e1982_d_b4, assign1670_e1982_d_b5, assign1670_e1982_d_b6, assign1670_e1982_d_b7,) = {
    if (var_guard16 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_dn7, var_qhi_dn8, var_qhi_dn9, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6, var_qhi_db7,)
    }
};
        var_qhi = assign1670_e1982;
        var_qhi_dn0 = assign1670_e1982_d_n0;
        var_qhi_dn1 = assign1670_e1982_d_n1;
        var_qhi_dn2 = assign1670_e1982_d_n2;
        var_qhi_dn3 = assign1670_e1982_d_n3;
        var_qhi_dn4 = assign1670_e1982_d_n4;
        var_qhi_dn5 = assign1670_e1982_d_n5;
        var_qhi_dn6 = assign1670_e1982_d_n6;
        var_qhi_dn7 = assign1670_e1982_d_n7;
        var_qhi_dn8 = assign1670_e1982_d_n8;
        var_qhi_dn9 = assign1670_e1982_d_n9;
        var_qhi_db0 = assign1670_e1982_d_b0;
        var_qhi_db1 = assign1670_e1982_d_b1;
        var_qhi_db2 = assign1670_e1982_d_b2;
        var_qhi_db3 = assign1670_e1982_d_b3;
        var_qhi_db4 = assign1670_e1982_d_b4;
        var_qhi_db5 = assign1670_e1982_d_b5;
        var_qhi_db6 = assign1670_e1982_d_b6;
        var_qhi_db7 = assign1670_e1982_d_b7;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdn7 = 0.0;
        var_qhi_rdn8 = 0.0;
        var_qhi_rdn9 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;
        var_qhi_rdb7 = 0.0;

        let assign1680_e1986: f64 = (var_qlo + var_qhi);
        let assign1680_e1987: f64 = (var_cje_t * assign1680_e1986);
        var_qje = assign1680_e1987;
        var_qje_dn0 = ((var_cje_t_dn0 * assign1680_e1986) + (var_cje_t * (var_qlo_dn0 + var_qhi_dn0)));
        var_qje_dn1 = ((var_cje_t_dn1 * assign1680_e1986) + (var_cje_t * (var_qlo_dn1 + var_qhi_dn1)));
        var_qje_dn2 = ((var_cje_t_dn2 * assign1680_e1986) + (var_cje_t * (var_qlo_dn2 + var_qhi_dn2)));
        var_qje_dn3 = ((var_cje_t_dn3 * assign1680_e1986) + (var_cje_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qje_dn4 = ((var_cje_t_dn4 * assign1680_e1986) + (var_cje_t * (var_qlo_dn4 + var_qhi_dn4)));
        var_qje_dn5 = ((var_cje_t_dn5 * assign1680_e1986) + (var_cje_t * (var_qlo_dn5 + var_qhi_dn5)));
        var_qje_dn6 = ((var_cje_t_dn6 * assign1680_e1986) + (var_cje_t * (var_qlo_dn6 + var_qhi_dn6)));
        var_qje_dn7 = ((var_cje_t_dn7 * assign1680_e1986) + (var_cje_t * (var_qlo_dn7 + var_qhi_dn7)));
        var_qje_dn8 = ((var_cje_t_dn8 * assign1680_e1986) + (var_cje_t * (var_qlo_dn8 + var_qhi_dn8)));
        var_qje_dn9 = ((var_cje_t_dn9 * assign1680_e1986) + (var_cje_t * (var_qlo_dn9 + var_qhi_dn9)));
        var_qje_db0 = ((var_cje_t_db0 * assign1680_e1986) + (var_cje_t * (var_qlo_db0 + var_qhi_db0)));
        var_qje_db1 = ((var_cje_t_db1 * assign1680_e1986) + (var_cje_t * (var_qlo_db1 + var_qhi_db1)));
        var_qje_db2 = ((var_cje_t_db2 * assign1680_e1986) + (var_cje_t * (var_qlo_db2 + var_qhi_db2)));
        var_qje_db3 = ((var_cje_t_db3 * assign1680_e1986) + (var_cje_t * (var_qlo_db3 + var_qhi_db3)));
        var_qje_db4 = ((var_cje_t_db4 * assign1680_e1986) + (var_cje_t * (var_qlo_db4 + var_qhi_db4)));
        var_qje_db5 = ((var_cje_t_db5 * assign1680_e1986) + (var_cje_t * (var_qlo_db5 + var_qhi_db5)));
        var_qje_db6 = ((var_cje_t_db6 * assign1680_e1986) + (var_cje_t * (var_qlo_db6 + var_qhi_db6)));
        var_qje_db7 = ((var_cje_t_db7 * assign1680_e1986) + (var_cje_t * (var_qlo_db7 + var_qhi_db7)));
        var_qje_rv = 0.0;
        var_qje_rdn0 = 0.0;
        var_qje_rdn1 = 0.0;
        var_qje_rdn2 = 0.0;
        var_qje_rdn3 = 0.0;
        var_qje_rdn4 = 0.0;
        var_qje_rdn5 = 0.0;
        var_qje_rdn6 = 0.0;
        var_qje_rdn7 = 0.0;
        var_qje_rdn8 = 0.0;
        var_qje_rdn9 = 0.0;
        var_qje_rdb0 = 0.0;
        var_qje_rdb1 = 0.0;
        var_qje_rdb2 = 0.0;
        var_qje_rdb3 = 0.0;
        var_qje_rdb4 = 0.0;
        var_qje_rdb5 = 0.0;
        var_qje_rdb6 = 0.0;
        var_qje_rdb7 = 0.0;

        let assign1690_e1989: f64 = (-var_vjc_t);
        let assign1690_e1991: f64 = (assign1690_e1989 * p.p24);
        var_dv0 = assign1690_e1991;
        var_dv0_dn0 = ((-var_vjc_t_dn0) * p.p24);
        var_dv0_dn1 = ((-var_vjc_t_dn1) * p.p24);
        var_dv0_dn2 = ((-var_vjc_t_dn2) * p.p24);
        var_dv0_dn3 = ((-var_vjc_t_dn3) * p.p24);
        var_dv0_dn4 = ((-var_vjc_t_dn4) * p.p24);
        var_dv0_dn5 = ((-var_vjc_t_dn5) * p.p24);
        var_dv0_dn6 = ((-var_vjc_t_dn6) * p.p24);
        var_dv0_dn7 = ((-var_vjc_t_dn7) * p.p24);
        var_dv0_dn8 = ((-var_vjc_t_dn8) * p.p24);
        var_dv0_dn9 = ((-var_vjc_t_dn9) * p.p24);
        var_dv0_db0 = ((-var_vjc_t_db0) * p.p24);
        var_dv0_db1 = ((-var_vjc_t_db1) * p.p24);
        var_dv0_db2 = ((-var_vjc_t_db2) * p.p24);
        var_dv0_db3 = ((-var_vjc_t_db3) * p.p24);
        var_dv0_db4 = ((-var_vjc_t_db4) * p.p24);
        var_dv0_db5 = ((-var_vjc_t_db5) * p.p24);
        var_dv0_db6 = ((-var_vjc_t_db6) * p.p24);
        var_dv0_db7 = ((-var_vjc_t_db7) * p.p24);
        var_dv0_rv = 0.0;
        var_dv0_rdn0 = 0.0;
        var_dv0_rdn1 = 0.0;
        var_dv0_rdn2 = 0.0;
        var_dv0_rdn3 = 0.0;
        var_dv0_rdn4 = 0.0;
        var_dv0_rdn5 = 0.0;
        var_dv0_rdn6 = 0.0;
        var_dv0_rdn7 = 0.0;
        var_dv0_rdn8 = 0.0;
        var_dv0_rdn9 = 0.0;
        var_dv0_rdb0 = 0.0;
        var_dv0_rdb1 = 0.0;
        var_dv0_rdb2 = 0.0;
        var_dv0_rdb3 = 0.0;
        var_dv0_rdb4 = 0.0;
        var_dv0_rdb5 = 0.0;
        var_dv0_rdb6 = 0.0;
        var_dv0_rdb7 = 0.0;

        let assign1700_e1994: f64 = (var_vbci + var_dv0);
        var_dvh = assign1700_e1994;
        var_dvh_dn0 = (var_vbci_dn0 + var_dv0_dn0);
        var_dvh_dn1 = (var_vbci_dn1 + var_dv0_dn1);
        var_dvh_dn2 = (var_vbci_dn2 + var_dv0_dn2);
        var_dvh_dn3 = (var_vbci_dn3 + var_dv0_dn3);
        var_dvh_dn4 = (var_vbci_dn4 + var_dv0_dn4);
        var_dvh_dn5 = (var_vbci_dn5 + var_dv0_dn5);
        var_dvh_dn6 = (var_vbci_dn6 + var_dv0_dn6);
        var_dvh_dn7 = (var_vbci_dn7 + var_dv0_dn7);
        var_dvh_dn8 = (var_vbci_dn8 + var_dv0_dn8);
        var_dvh_dn9 = (var_vbci_dn9 + var_dv0_dn9);
        var_dvh_db0 = (var_vbci_db0 + var_dv0_db0);
        var_dvh_db1 = (var_vbci_db1 + var_dv0_db1);
        var_dvh_db2 = (var_vbci_db2 + var_dv0_db2);
        var_dvh_db3 = (var_vbci_db3 + var_dv0_db3);
        var_dvh_db4 = (var_vbci_db4 + var_dv0_db4);
        var_dvh_db5 = (var_vbci_db5 + var_dv0_db5);
        var_dvh_db6 = (var_vbci_db6 + var_dv0_db6);
        var_dvh_db7 = (var_vbci_db7 + var_dv0_db7);
        var_dvh_rv = 0.0;
        var_dvh_rdn0 = 0.0;
        var_dvh_rdn1 = 0.0;
        var_dvh_rdn2 = 0.0;
        var_dvh_rdn3 = 0.0;
        var_dvh_rdn4 = 0.0;
        var_dvh_rdn5 = 0.0;
        var_dvh_rdn6 = 0.0;
        var_dvh_rdn7 = 0.0;
        var_dvh_rdn8 = 0.0;
        var_dvh_rdn9 = 0.0;
        var_dvh_rdb0 = 0.0;
        var_dvh_rdb1 = 0.0;
        var_dvh_rdb2 = 0.0;
        var_dvh_rdb3 = 0.0;
        var_dvh_rdb4 = 0.0;
        var_dvh_rdb5 = 0.0;
        var_dvh_rdb6 = 0.0;
        var_dvh_rdb7 = 0.0;

        let assign1710_e1997: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard17 = assign1710_e1997;
        var_guard17_dn0 = 0.0;
        var_guard17_dn1 = 0.0;
        var_guard17_dn2 = 0.0;
        var_guard17_dn3 = 0.0;
        var_guard17_dn4 = 0.0;
        var_guard17_dn5 = 0.0;
        var_guard17_dn6 = 0.0;
        var_guard17_dn7 = 0.0;
        var_guard17_dn8 = 0.0;
        var_guard17_dn9 = 0.0;
        var_guard17_db0 = 0.0;
        var_guard17_db1 = 0.0;
        var_guard17_db2 = 0.0;
        var_guard17_db3 = 0.0;
        var_guard17_db4 = 0.0;
        var_guard17_db5 = 0.0;
        var_guard17_db6 = 0.0;
        var_guard17_db7 = 0.0;
        var_guard17_rv = 0.0;
        var_guard17_rdn0 = 0.0;
        var_guard17_rdn1 = 0.0;
        var_guard17_rdn2 = 0.0;
        var_guard17_rdn3 = 0.0;
        var_guard17_rdn4 = 0.0;
        var_guard17_rdn5 = 0.0;
        var_guard17_rdn6 = 0.0;
        var_guard17_rdn7 = 0.0;
        var_guard17_rdn8 = 0.0;
        var_guard17_rdn9 = 0.0;
        var_guard17_rdb0 = 0.0;
        var_guard17_rdb1 = 0.0;
        var_guard17_rdb2 = 0.0;
        var_guard17_rdb3 = 0.0;
        var_guard17_rdb4 = 0.0;
        var_guard17_rdb5 = 0.0;
        var_guard17_rdb6 = 0.0;
        var_guard17_rdb7 = 0.0;

        let (assign1720_e2010, assign1720_e2010_d_n0, assign1720_e2010_d_n1, assign1720_e2010_d_n2, assign1720_e2010_d_n3, assign1720_e2010_d_n4, assign1720_e2010_d_n5, assign1720_e2010_d_n6, assign1720_e2010_d_n7, assign1720_e2010_d_n8, assign1720_e2010_d_n9, assign1720_e2010_d_b0, assign1720_e2010_d_b1, assign1720_e2010_d_b2, assign1720_e2010_d_b3, assign1720_e2010_d_b4, assign1720_e2010_d_b5, assign1720_e2010_d_b6, assign1720_e2010_d_b7,) = {
    if (var_guard17 != 0.0) {
        let assign1720_e2000: f64 = (-1.0);
        let assign1720_e2002: f64 = (assign1720_e2000 - p.p71);
        let assign1720_e2005: f64 = (1.0 - p.p24);
        let assign1720_e2006: f64 = (assign1720_e2005).ln();
        let assign1720_e2007: f64 = (assign1720_e2002 * assign1720_e2006);
        let assign1720_e2008: f64 = (assign1720_e2007).exp();
        (assign1720_e2008, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pwq, var_pwq_dn0, var_pwq_dn1, var_pwq_dn2, var_pwq_dn3, var_pwq_dn4, var_pwq_dn5, var_pwq_dn6, var_pwq_dn7, var_pwq_dn8, var_pwq_dn9, var_pwq_db0, var_pwq_db1, var_pwq_db2, var_pwq_db3, var_pwq_db4, var_pwq_db5, var_pwq_db6, var_pwq_db7,)
    }
};
        var_pwq = assign1720_e2010;
        var_pwq_dn0 = assign1720_e2010_d_n0;
        var_pwq_dn1 = assign1720_e2010_d_n1;
        var_pwq_dn2 = assign1720_e2010_d_n2;
        var_pwq_dn3 = assign1720_e2010_d_n3;
        var_pwq_dn4 = assign1720_e2010_d_n4;
        var_pwq_dn5 = assign1720_e2010_d_n5;
        var_pwq_dn6 = assign1720_e2010_d_n6;
        var_pwq_dn7 = assign1720_e2010_d_n7;
        var_pwq_dn8 = assign1720_e2010_d_n8;
        var_pwq_dn9 = assign1720_e2010_d_n9;
        var_pwq_db0 = assign1720_e2010_d_b0;
        var_pwq_db1 = assign1720_e2010_d_b1;
        var_pwq_db2 = assign1720_e2010_d_b2;
        var_pwq_db3 = assign1720_e2010_d_b3;
        var_pwq_db4 = assign1720_e2010_d_b4;
        var_pwq_db5 = assign1720_e2010_d_b5;
        var_pwq_db6 = assign1720_e2010_d_b6;
        var_pwq_db7 = assign1720_e2010_d_b7;
        var_pwq_rv = 0.0;
        var_pwq_rdn0 = 0.0;
        var_pwq_rdn1 = 0.0;
        var_pwq_rdn2 = 0.0;
        var_pwq_rdn3 = 0.0;
        var_pwq_rdn4 = 0.0;
        var_pwq_rdn5 = 0.0;
        var_pwq_rdn6 = 0.0;
        var_pwq_rdn7 = 0.0;
        var_pwq_rdn8 = 0.0;
        var_pwq_rdn9 = 0.0;
        var_pwq_rdb0 = 0.0;
        var_pwq_rdb1 = 0.0;
        var_pwq_rdb2 = 0.0;
        var_pwq_rdb3 = 0.0;
        var_pwq_rdb4 = 0.0;
        var_pwq_rdb5 = 0.0;
        var_pwq_rdb6 = 0.0;
        var_pwq_rdb7 = 0.0;

        let (assign1730_e2030, assign1730_e2030_d_n0, assign1730_e2030_d_n1, assign1730_e2030_d_n2, assign1730_e2030_d_n3, assign1730_e2030_d_n4, assign1730_e2030_d_n5, assign1730_e2030_d_n6, assign1730_e2030_d_n7, assign1730_e2030_d_n8, assign1730_e2030_d_n9, assign1730_e2030_d_b0, assign1730_e2030_d_b1, assign1730_e2030_d_b2, assign1730_e2030_d_b3, assign1730_e2030_d_b4, assign1730_e2030_d_b5, assign1730_e2030_d_b6, assign1730_e2030_d_b7,) = {
    if (var_guard17 != 0.0) {
        let assign1730_e2017: f64 = (1.0 - p.p24);
        let assign1730_e2018: f64 = (var_pwq * assign1730_e2017);
        let assign1730_e2021: f64 = (1.0 - p.p24);
        let assign1730_e2022: f64 = (assign1730_e2018 * assign1730_e2021);
        let assign1730_e2023: f64 = (1.0 - assign1730_e2022);
        let assign1730_e2024: f64 = (var_vjc_t * assign1730_e2023);
        let assign1730_e2027: f64 = (1.0 - p.p71);
        let assign1730_e2028: f64 = (assign1730_e2024 / assign1730_e2027);
        (assign1730_e2028, (((var_vjc_t_dn0 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn0 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn1 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn1 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn2 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn2 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn3 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn3 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn4 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn4 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn5 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn5 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn6 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn6 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn7 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn7 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn8 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn8 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn9 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn9 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db0 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db0 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db1 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db1 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db2 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db2 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db3 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db3 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db4 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db4 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db5 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db5 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db6 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db6 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db7 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db7 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7,)
    }
};
        var_qlo = assign1730_e2030;
        var_qlo_dn0 = assign1730_e2030_d_n0;
        var_qlo_dn1 = assign1730_e2030_d_n1;
        var_qlo_dn2 = assign1730_e2030_d_n2;
        var_qlo_dn3 = assign1730_e2030_d_n3;
        var_qlo_dn4 = assign1730_e2030_d_n4;
        var_qlo_dn5 = assign1730_e2030_d_n5;
        var_qlo_dn6 = assign1730_e2030_d_n6;
        var_qlo_dn7 = assign1730_e2030_d_n7;
        var_qlo_dn8 = assign1730_e2030_d_n8;
        var_qlo_dn9 = assign1730_e2030_d_n9;
        var_qlo_db0 = assign1730_e2030_d_b0;
        var_qlo_db1 = assign1730_e2030_d_b1;
        var_qlo_db2 = assign1730_e2030_d_b2;
        var_qlo_db3 = assign1730_e2030_d_b3;
        var_qlo_db4 = assign1730_e2030_d_b4;
        var_qlo_db5 = assign1730_e2030_d_b5;
        var_qlo_db6 = assign1730_e2030_d_b6;
        var_qlo_db7 = assign1730_e2030_d_b7;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdn7 = 0.0;
        var_qlo_rdn8 = 0.0;
        var_qlo_rdn9 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;
        var_qlo_rdb7 = 0.0;

        let (assign1740_e2048, assign1740_e2048_d_n0, assign1740_e2048_d_n1, assign1740_e2048_d_n2, assign1740_e2048_d_n3, assign1740_e2048_d_n4, assign1740_e2048_d_n5, assign1740_e2048_d_n6, assign1740_e2048_d_n7, assign1740_e2048_d_n8, assign1740_e2048_d_n9, assign1740_e2048_d_b0, assign1740_e2048_d_b1, assign1740_e2048_d_b2, assign1740_e2048_d_b3, assign1740_e2048_d_b4, assign1740_e2048_d_b5, assign1740_e2048_d_b6, assign1740_e2048_d_b7,) = {
    if (var_guard17 != 0.0) {
        let assign1740_e2035: f64 = (1.0 - p.p24);
        let assign1740_e2038: f64 = (0.5 * p.p71);
        let assign1740_e2040: f64 = (assign1740_e2038 * var_dvh);
        let assign1740_e2042: f64 = (assign1740_e2040 / var_vjc_t);
        let assign1740_e2043: f64 = (assign1740_e2035 + assign1740_e2042);
        let assign1740_e2044: f64 = (var_dvh * assign1740_e2043);
        let assign1740_e2046: f64 = (assign1740_e2044 * var_pwq);
        (assign1740_e2046, ((((var_dvh_dn0 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn0) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn0)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn0)), ((((var_dvh_dn1 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn1) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn1)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn1)), ((((var_dvh_dn2 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn2) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn2)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn2)), ((((var_dvh_dn3 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn3) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn3)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn3)), ((((var_dvh_dn4 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn4) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn4)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn4)), ((((var_dvh_dn5 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn5) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn5)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn5)), ((((var_dvh_dn6 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn6) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn6)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn6)), ((((var_dvh_dn7 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn7) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn7)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn7)), ((((var_dvh_dn8 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn8) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn8)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn8)), ((((var_dvh_dn9 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn9) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn9)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn9)), ((((var_dvh_db0 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db0) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db0)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db0)), ((((var_dvh_db1 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db1) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db1)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db1)), ((((var_dvh_db2 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db2) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db2)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db2)), ((((var_dvh_db3 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db3) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db3)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db3)), ((((var_dvh_db4 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db4) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db4)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db4)), ((((var_dvh_db5 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db5) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db5)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db5)), ((((var_dvh_db6 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db6) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db6)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db6)), ((((var_dvh_db7 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db7) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db7)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db7)),)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_dn7, var_qhi_dn8, var_qhi_dn9, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6, var_qhi_db7,)
    }
};
        var_qhi = assign1740_e2048;
        var_qhi_dn0 = assign1740_e2048_d_n0;
        var_qhi_dn1 = assign1740_e2048_d_n1;
        var_qhi_dn2 = assign1740_e2048_d_n2;
        var_qhi_dn3 = assign1740_e2048_d_n3;
        var_qhi_dn4 = assign1740_e2048_d_n4;
        var_qhi_dn5 = assign1740_e2048_d_n5;
        var_qhi_dn6 = assign1740_e2048_d_n6;
        var_qhi_dn7 = assign1740_e2048_d_n7;
        var_qhi_dn8 = assign1740_e2048_d_n8;
        var_qhi_dn9 = assign1740_e2048_d_n9;
        var_qhi_db0 = assign1740_e2048_d_b0;
        var_qhi_db1 = assign1740_e2048_d_b1;
        var_qhi_db2 = assign1740_e2048_d_b2;
        var_qhi_db3 = assign1740_e2048_d_b3;
        var_qhi_db4 = assign1740_e2048_d_b4;
        var_qhi_db5 = assign1740_e2048_d_b5;
        var_qhi_db6 = assign1740_e2048_d_b6;
        var_qhi_db7 = assign1740_e2048_d_b7;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdn7 = 0.0;
        var_qhi_rdn8 = 0.0;
        var_qhi_rdn9 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;
        var_qhi_rdb7 = 0.0;

        let (assign1750_e2071, assign1750_e2071_d_n0, assign1750_e2071_d_n1, assign1750_e2071_d_n2, assign1750_e2071_d_n3, assign1750_e2071_d_n4, assign1750_e2071_d_n5, assign1750_e2071_d_n6, assign1750_e2071_d_n7, assign1750_e2071_d_n8, assign1750_e2071_d_n9, assign1750_e2071_d_b0, assign1750_e2071_d_b1, assign1750_e2071_d_b2, assign1750_e2071_d_b3, assign1750_e2071_d_b4, assign1750_e2071_d_b5, assign1750_e2071_d_b6, assign1750_e2071_d_b7,) = {
    if (var_guard17 == 0.0) {
        let assign1750_e2055: f64 = (1.0 - p.p71);
        let assign1750_e2059: f64 = (var_vbci / var_vjc_t);
        let assign1750_e2060: f64 = (1.0 - assign1750_e2059);
        let assign1750_e2061: f64 = (assign1750_e2060).ln();
        let assign1750_e2062: f64 = (assign1750_e2055 * assign1750_e2061);
        let assign1750_e2063: f64 = (assign1750_e2062).exp();
        let assign1750_e2064: f64 = (1.0 - assign1750_e2063);
        let assign1750_e2065: f64 = (var_vjc_t * assign1750_e2064);
        let assign1750_e2068: f64 = (1.0 - p.p71);
        let assign1750_e2069: f64 = (assign1750_e2065 / assign1750_e2068);
        (assign1750_e2069, (((var_vjc_t_dn0 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn0 * var_vjc_t) - (var_vbci * var_vjc_t_dn0)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn1 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn1 * var_vjc_t) - (var_vbci * var_vjc_t_dn1)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn2 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn2 * var_vjc_t) - (var_vbci * var_vjc_t_dn2)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn3 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn3 * var_vjc_t) - (var_vbci * var_vjc_t_dn3)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn4 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn4 * var_vjc_t) - (var_vbci * var_vjc_t_dn4)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn5 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn5 * var_vjc_t) - (var_vbci * var_vjc_t_dn5)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn6 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn6 * var_vjc_t) - (var_vbci * var_vjc_t_dn6)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn7 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn7 * var_vjc_t) - (var_vbci * var_vjc_t_dn7)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn8 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn8 * var_vjc_t) - (var_vbci * var_vjc_t_dn8)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn9 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn9 * var_vjc_t) - (var_vbci * var_vjc_t_dn9)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db0 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db0 * var_vjc_t) - (var_vbci * var_vjc_t_db0)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db1 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db1 * var_vjc_t) - (var_vbci * var_vjc_t_db1)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db2 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db2 * var_vjc_t) - (var_vbci * var_vjc_t_db2)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db3 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db3 * var_vjc_t) - (var_vbci * var_vjc_t_db3)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db4 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db4 * var_vjc_t) - (var_vbci * var_vjc_t_db4)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db5 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db5 * var_vjc_t) - (var_vbci * var_vjc_t_db5)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db6 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db6 * var_vjc_t) - (var_vbci * var_vjc_t_db6)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db7 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db7 * var_vjc_t) - (var_vbci * var_vjc_t_db7)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7,)
    }
};
        var_qlo = assign1750_e2071;
        var_qlo_dn0 = assign1750_e2071_d_n0;
        var_qlo_dn1 = assign1750_e2071_d_n1;
        var_qlo_dn2 = assign1750_e2071_d_n2;
        var_qlo_dn3 = assign1750_e2071_d_n3;
        var_qlo_dn4 = assign1750_e2071_d_n4;
        var_qlo_dn5 = assign1750_e2071_d_n5;
        var_qlo_dn6 = assign1750_e2071_d_n6;
        var_qlo_dn7 = assign1750_e2071_d_n7;
        var_qlo_dn8 = assign1750_e2071_d_n8;
        var_qlo_dn9 = assign1750_e2071_d_n9;
        var_qlo_db0 = assign1750_e2071_d_b0;
        var_qlo_db1 = assign1750_e2071_d_b1;
        var_qlo_db2 = assign1750_e2071_d_b2;
        var_qlo_db3 = assign1750_e2071_d_b3;
        var_qlo_db4 = assign1750_e2071_d_b4;
        var_qlo_db5 = assign1750_e2071_d_b5;
        var_qlo_db6 = assign1750_e2071_d_b6;
        var_qlo_db7 = assign1750_e2071_d_b7;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdn7 = 0.0;
        var_qlo_rdn8 = 0.0;
        var_qlo_rdn9 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;
        var_qlo_rdb7 = 0.0;

        *var_dv0_slot = var_dv0;
        *var_dv0_db0_slot = var_dv0_db0;
        *var_dv0_db1_slot = var_dv0_db1;
        *var_dv0_db2_slot = var_dv0_db2;
        *var_dv0_db3_slot = var_dv0_db3;
        *var_dv0_db4_slot = var_dv0_db4;
        *var_dv0_db5_slot = var_dv0_db5;
        *var_dv0_db6_slot = var_dv0_db6;
        *var_dv0_db7_slot = var_dv0_db7;
        *var_dv0_dn0_slot = var_dv0_dn0;
        *var_dv0_dn1_slot = var_dv0_dn1;
        *var_dv0_dn2_slot = var_dv0_dn2;
        *var_dv0_dn3_slot = var_dv0_dn3;
        *var_dv0_dn4_slot = var_dv0_dn4;
        *var_dv0_dn5_slot = var_dv0_dn5;
        *var_dv0_dn6_slot = var_dv0_dn6;
        *var_dv0_dn7_slot = var_dv0_dn7;
        *var_dv0_dn8_slot = var_dv0_dn8;
        *var_dv0_dn9_slot = var_dv0_dn9;
        *var_dv0_rdb0_slot = var_dv0_rdb0;
        *var_dv0_rdb1_slot = var_dv0_rdb1;
        *var_dv0_rdb2_slot = var_dv0_rdb2;
        *var_dv0_rdb3_slot = var_dv0_rdb3;
        *var_dv0_rdb4_slot = var_dv0_rdb4;
        *var_dv0_rdb5_slot = var_dv0_rdb5;
        *var_dv0_rdb6_slot = var_dv0_rdb6;
        *var_dv0_rdb7_slot = var_dv0_rdb7;
        *var_dv0_rdn0_slot = var_dv0_rdn0;
        *var_dv0_rdn1_slot = var_dv0_rdn1;
        *var_dv0_rdn2_slot = var_dv0_rdn2;
        *var_dv0_rdn3_slot = var_dv0_rdn3;
        *var_dv0_rdn4_slot = var_dv0_rdn4;
        *var_dv0_rdn5_slot = var_dv0_rdn5;
        *var_dv0_rdn6_slot = var_dv0_rdn6;
        *var_dv0_rdn7_slot = var_dv0_rdn7;
        *var_dv0_rdn8_slot = var_dv0_rdn8;
        *var_dv0_rdn9_slot = var_dv0_rdn9;
        *var_dv0_rv_slot = var_dv0_rv;
        *var_dvh_slot = var_dvh;
        *var_dvh_db0_slot = var_dvh_db0;
        *var_dvh_db1_slot = var_dvh_db1;
        *var_dvh_db2_slot = var_dvh_db2;
        *var_dvh_db3_slot = var_dvh_db3;
        *var_dvh_db4_slot = var_dvh_db4;
        *var_dvh_db5_slot = var_dvh_db5;
        *var_dvh_db6_slot = var_dvh_db6;
        *var_dvh_db7_slot = var_dvh_db7;
        *var_dvh_dn0_slot = var_dvh_dn0;
        *var_dvh_dn1_slot = var_dvh_dn1;
        *var_dvh_dn2_slot = var_dvh_dn2;
        *var_dvh_dn3_slot = var_dvh_dn3;
        *var_dvh_dn4_slot = var_dvh_dn4;
        *var_dvh_dn5_slot = var_dvh_dn5;
        *var_dvh_dn6_slot = var_dvh_dn6;
        *var_dvh_dn7_slot = var_dvh_dn7;
        *var_dvh_dn8_slot = var_dvh_dn8;
        *var_dvh_dn9_slot = var_dvh_dn9;
        *var_dvh_rdb0_slot = var_dvh_rdb0;
        *var_dvh_rdb1_slot = var_dvh_rdb1;
        *var_dvh_rdb2_slot = var_dvh_rdb2;
        *var_dvh_rdb3_slot = var_dvh_rdb3;
        *var_dvh_rdb4_slot = var_dvh_rdb4;
        *var_dvh_rdb5_slot = var_dvh_rdb5;
        *var_dvh_rdb6_slot = var_dvh_rdb6;
        *var_dvh_rdb7_slot = var_dvh_rdb7;
        *var_dvh_rdn0_slot = var_dvh_rdn0;
        *var_dvh_rdn1_slot = var_dvh_rdn1;
        *var_dvh_rdn2_slot = var_dvh_rdn2;
        *var_dvh_rdn3_slot = var_dvh_rdn3;
        *var_dvh_rdn4_slot = var_dvh_rdn4;
        *var_dvh_rdn5_slot = var_dvh_rdn5;
        *var_dvh_rdn6_slot = var_dvh_rdn6;
        *var_dvh_rdn7_slot = var_dvh_rdn7;
        *var_dvh_rdn8_slot = var_dvh_rdn8;
        *var_dvh_rdn9_slot = var_dvh_rdn9;
        *var_dvh_rv_slot = var_dvh_rv;
        *var_guard17_slot = var_guard17;
        *var_guard17_db0_slot = var_guard17_db0;
        *var_guard17_db1_slot = var_guard17_db1;
        *var_guard17_db2_slot = var_guard17_db2;
        *var_guard17_db3_slot = var_guard17_db3;
        *var_guard17_db4_slot = var_guard17_db4;
        *var_guard17_db5_slot = var_guard17_db5;
        *var_guard17_db6_slot = var_guard17_db6;
        *var_guard17_db7_slot = var_guard17_db7;
        *var_guard17_dn0_slot = var_guard17_dn0;
        *var_guard17_dn1_slot = var_guard17_dn1;
        *var_guard17_dn2_slot = var_guard17_dn2;
        *var_guard17_dn3_slot = var_guard17_dn3;
        *var_guard17_dn4_slot = var_guard17_dn4;
        *var_guard17_dn5_slot = var_guard17_dn5;
        *var_guard17_dn6_slot = var_guard17_dn6;
        *var_guard17_dn7_slot = var_guard17_dn7;
        *var_guard17_dn8_slot = var_guard17_dn8;
        *var_guard17_dn9_slot = var_guard17_dn9;
        *var_guard17_rdb0_slot = var_guard17_rdb0;
        *var_guard17_rdb1_slot = var_guard17_rdb1;
        *var_guard17_rdb2_slot = var_guard17_rdb2;
        *var_guard17_rdb3_slot = var_guard17_rdb3;
        *var_guard17_rdb4_slot = var_guard17_rdb4;
        *var_guard17_rdb5_slot = var_guard17_rdb5;
        *var_guard17_rdb6_slot = var_guard17_rdb6;
        *var_guard17_rdb7_slot = var_guard17_rdb7;
        *var_guard17_rdn0_slot = var_guard17_rdn0;
        *var_guard17_rdn1_slot = var_guard17_rdn1;
        *var_guard17_rdn2_slot = var_guard17_rdn2;
        *var_guard17_rdn3_slot = var_guard17_rdn3;
        *var_guard17_rdn4_slot = var_guard17_rdn4;
        *var_guard17_rdn5_slot = var_guard17_rdn5;
        *var_guard17_rdn6_slot = var_guard17_rdn6;
        *var_guard17_rdn7_slot = var_guard17_rdn7;
        *var_guard17_rdn8_slot = var_guard17_rdn8;
        *var_guard17_rdn9_slot = var_guard17_rdn9;
        *var_guard17_rv_slot = var_guard17_rv;
        *var_pwq_slot = var_pwq;
        *var_pwq_db0_slot = var_pwq_db0;
        *var_pwq_db1_slot = var_pwq_db1;
        *var_pwq_db2_slot = var_pwq_db2;
        *var_pwq_db3_slot = var_pwq_db3;
        *var_pwq_db4_slot = var_pwq_db4;
        *var_pwq_db5_slot = var_pwq_db5;
        *var_pwq_db6_slot = var_pwq_db6;
        *var_pwq_db7_slot = var_pwq_db7;
        *var_pwq_dn0_slot = var_pwq_dn0;
        *var_pwq_dn1_slot = var_pwq_dn1;
        *var_pwq_dn2_slot = var_pwq_dn2;
        *var_pwq_dn3_slot = var_pwq_dn3;
        *var_pwq_dn4_slot = var_pwq_dn4;
        *var_pwq_dn5_slot = var_pwq_dn5;
        *var_pwq_dn6_slot = var_pwq_dn6;
        *var_pwq_dn7_slot = var_pwq_dn7;
        *var_pwq_dn8_slot = var_pwq_dn8;
        *var_pwq_dn9_slot = var_pwq_dn9;
        *var_pwq_rdb0_slot = var_pwq_rdb0;
        *var_pwq_rdb1_slot = var_pwq_rdb1;
        *var_pwq_rdb2_slot = var_pwq_rdb2;
        *var_pwq_rdb3_slot = var_pwq_rdb3;
        *var_pwq_rdb4_slot = var_pwq_rdb4;
        *var_pwq_rdb5_slot = var_pwq_rdb5;
        *var_pwq_rdb6_slot = var_pwq_rdb6;
        *var_pwq_rdb7_slot = var_pwq_rdb7;
        *var_pwq_rdn0_slot = var_pwq_rdn0;
        *var_pwq_rdn1_slot = var_pwq_rdn1;
        *var_pwq_rdn2_slot = var_pwq_rdn2;
        *var_pwq_rdn3_slot = var_pwq_rdn3;
        *var_pwq_rdn4_slot = var_pwq_rdn4;
        *var_pwq_rdn5_slot = var_pwq_rdn5;
        *var_pwq_rdn6_slot = var_pwq_rdn6;
        *var_pwq_rdn7_slot = var_pwq_rdn7;
        *var_pwq_rdn8_slot = var_pwq_rdn8;
        *var_pwq_rdn9_slot = var_pwq_rdn9;
        *var_pwq_rv_slot = var_pwq_rv;
        *var_qhi_slot = var_qhi;
        *var_qhi_db0_slot = var_qhi_db0;
        *var_qhi_db1_slot = var_qhi_db1;
        *var_qhi_db2_slot = var_qhi_db2;
        *var_qhi_db3_slot = var_qhi_db3;
        *var_qhi_db4_slot = var_qhi_db4;
        *var_qhi_db5_slot = var_qhi_db5;
        *var_qhi_db6_slot = var_qhi_db6;
        *var_qhi_db7_slot = var_qhi_db7;
        *var_qhi_dn0_slot = var_qhi_dn0;
        *var_qhi_dn1_slot = var_qhi_dn1;
        *var_qhi_dn2_slot = var_qhi_dn2;
        *var_qhi_dn3_slot = var_qhi_dn3;
        *var_qhi_dn4_slot = var_qhi_dn4;
        *var_qhi_dn5_slot = var_qhi_dn5;
        *var_qhi_dn6_slot = var_qhi_dn6;
        *var_qhi_dn7_slot = var_qhi_dn7;
        *var_qhi_dn8_slot = var_qhi_dn8;
        *var_qhi_dn9_slot = var_qhi_dn9;
        *var_qhi_rdb0_slot = var_qhi_rdb0;
        *var_qhi_rdb1_slot = var_qhi_rdb1;
        *var_qhi_rdb2_slot = var_qhi_rdb2;
        *var_qhi_rdb3_slot = var_qhi_rdb3;
        *var_qhi_rdb4_slot = var_qhi_rdb4;
        *var_qhi_rdb5_slot = var_qhi_rdb5;
        *var_qhi_rdb6_slot = var_qhi_rdb6;
        *var_qhi_rdb7_slot = var_qhi_rdb7;
        *var_qhi_rdn0_slot = var_qhi_rdn0;
        *var_qhi_rdn1_slot = var_qhi_rdn1;
        *var_qhi_rdn2_slot = var_qhi_rdn2;
        *var_qhi_rdn3_slot = var_qhi_rdn3;
        *var_qhi_rdn4_slot = var_qhi_rdn4;
        *var_qhi_rdn5_slot = var_qhi_rdn5;
        *var_qhi_rdn6_slot = var_qhi_rdn6;
        *var_qhi_rdn7_slot = var_qhi_rdn7;
        *var_qhi_rdn8_slot = var_qhi_rdn8;
        *var_qhi_rdn9_slot = var_qhi_rdn9;
        *var_qhi_rv_slot = var_qhi_rv;
        *var_qje_slot = var_qje;
        *var_qje_db0_slot = var_qje_db0;
        *var_qje_db1_slot = var_qje_db1;
        *var_qje_db2_slot = var_qje_db2;
        *var_qje_db3_slot = var_qje_db3;
        *var_qje_db4_slot = var_qje_db4;
        *var_qje_db5_slot = var_qje_db5;
        *var_qje_db6_slot = var_qje_db6;
        *var_qje_db7_slot = var_qje_db7;
        *var_qje_dn0_slot = var_qje_dn0;
        *var_qje_dn1_slot = var_qje_dn1;
        *var_qje_dn2_slot = var_qje_dn2;
        *var_qje_dn3_slot = var_qje_dn3;
        *var_qje_dn4_slot = var_qje_dn4;
        *var_qje_dn5_slot = var_qje_dn5;
        *var_qje_dn6_slot = var_qje_dn6;
        *var_qje_dn7_slot = var_qje_dn7;
        *var_qje_dn8_slot = var_qje_dn8;
        *var_qje_dn9_slot = var_qje_dn9;
        *var_qje_rdb0_slot = var_qje_rdb0;
        *var_qje_rdb1_slot = var_qje_rdb1;
        *var_qje_rdb2_slot = var_qje_rdb2;
        *var_qje_rdb3_slot = var_qje_rdb3;
        *var_qje_rdb4_slot = var_qje_rdb4;
        *var_qje_rdb5_slot = var_qje_rdb5;
        *var_qje_rdb6_slot = var_qje_rdb6;
        *var_qje_rdb7_slot = var_qje_rdb7;
        *var_qje_rdn0_slot = var_qje_rdn0;
        *var_qje_rdn1_slot = var_qje_rdn1;
        *var_qje_rdn2_slot = var_qje_rdn2;
        *var_qje_rdn3_slot = var_qje_rdn3;
        *var_qje_rdn4_slot = var_qje_rdn4;
        *var_qje_rdn5_slot = var_qje_rdn5;
        *var_qje_rdn6_slot = var_qje_rdn6;
        *var_qje_rdn7_slot = var_qje_rdn7;
        *var_qje_rdn8_slot = var_qje_rdn8;
        *var_qje_rdn9_slot = var_qje_rdn9;
        *var_qje_rv_slot = var_qje_rv;
        *var_qlo_slot = var_qlo;
        *var_qlo_db0_slot = var_qlo_db0;
        *var_qlo_db1_slot = var_qlo_db1;
        *var_qlo_db2_slot = var_qlo_db2;
        *var_qlo_db3_slot = var_qlo_db3;
        *var_qlo_db4_slot = var_qlo_db4;
        *var_qlo_db5_slot = var_qlo_db5;
        *var_qlo_db6_slot = var_qlo_db6;
        *var_qlo_db7_slot = var_qlo_db7;
        *var_qlo_dn0_slot = var_qlo_dn0;
        *var_qlo_dn1_slot = var_qlo_dn1;
        *var_qlo_dn2_slot = var_qlo_dn2;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_qlo_dn5_slot = var_qlo_dn5;
        *var_qlo_dn6_slot = var_qlo_dn6;
        *var_qlo_dn7_slot = var_qlo_dn7;
        *var_qlo_dn8_slot = var_qlo_dn8;
        *var_qlo_dn9_slot = var_qlo_dn9;
        *var_qlo_rdb0_slot = var_qlo_rdb0;
        *var_qlo_rdb1_slot = var_qlo_rdb1;
        *var_qlo_rdb2_slot = var_qlo_rdb2;
        *var_qlo_rdb3_slot = var_qlo_rdb3;
        *var_qlo_rdb4_slot = var_qlo_rdb4;
        *var_qlo_rdb5_slot = var_qlo_rdb5;
        *var_qlo_rdb6_slot = var_qlo_rdb6;
        *var_qlo_rdb7_slot = var_qlo_rdb7;
        *var_qlo_rdn0_slot = var_qlo_rdn0;
        *var_qlo_rdn1_slot = var_qlo_rdn1;
        *var_qlo_rdn2_slot = var_qlo_rdn2;
        *var_qlo_rdn3_slot = var_qlo_rdn3;
        *var_qlo_rdn4_slot = var_qlo_rdn4;
        *var_qlo_rdn5_slot = var_qlo_rdn5;
        *var_qlo_rdn6_slot = var_qlo_rdn6;
        *var_qlo_rdn7_slot = var_qlo_rdn7;
        *var_qlo_rdn8_slot = var_qlo_rdn8;
        *var_qlo_rdn9_slot = var_qlo_rdn9;
        *var_qlo_rv_slot = var_qlo_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        var_cjc_t: f64,
        var_cjc_t_db0: f64,
        var_cjc_t_db1: f64,
        var_cjc_t_db2: f64,
        var_cjc_t_db3: f64,
        var_cjc_t_db4: f64,
        var_cjc_t_db5: f64,
        var_cjc_t_db6: f64,
        var_cjc_t_db7: f64,
        var_cjc_t_dn0: f64,
        var_cjc_t_dn1: f64,
        var_cjc_t_dn2: f64,
        var_cjc_t_dn3: f64,
        var_cjc_t_dn4: f64,
        var_cjc_t_dn5: f64,
        var_cjc_t_dn6: f64,
        var_cjc_t_dn7: f64,
        var_cjc_t_dn8: f64,
        var_cjc_t_dn9: f64,
        var_guard17: f64,
        var_vbici: f64,
        var_vbici_db0: f64,
        var_vbici_db1: f64,
        var_vbici_db2: f64,
        var_vbici_db3: f64,
        var_vbici_db4: f64,
        var_vbici_db5: f64,
        var_vbici_db6: f64,
        var_vbici_db7: f64,
        var_vbici_dn0: f64,
        var_vbici_dn1: f64,
        var_vbici_dn2: f64,
        var_vbici_dn3: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbici_dn6: f64,
        var_vbici_dn7: f64,
        var_vbici_dn8: f64,
        var_vbici_dn9: f64,
        var_vjc_t: f64,
        var_vjc_t_db0: f64,
        var_vjc_t_db1: f64,
        var_vjc_t_db2: f64,
        var_vjc_t_db3: f64,
        var_vjc_t_db4: f64,
        var_vjc_t_db5: f64,
        var_vjc_t_db6: f64,
        var_vjc_t_db7: f64,
        var_vjc_t_dn0: f64,
        var_vjc_t_dn1: f64,
        var_vjc_t_dn2: f64,
        var_vjc_t_dn3: f64,
        var_vjc_t_dn4: f64,
        var_vjc_t_dn5: f64,
        var_vjc_t_dn6: f64,
        var_vjc_t_dn7: f64,
        var_vjc_t_dn8: f64,
        var_vjc_t_dn9: f64,
        var_dv0_slot: &mut f64,
        var_dv0_db0_slot: &mut f64,
        var_dv0_db1_slot: &mut f64,
        var_dv0_db2_slot: &mut f64,
        var_dv0_db3_slot: &mut f64,
        var_dv0_db4_slot: &mut f64,
        var_dv0_db5_slot: &mut f64,
        var_dv0_db6_slot: &mut f64,
        var_dv0_db7_slot: &mut f64,
        var_dv0_dn0_slot: &mut f64,
        var_dv0_dn1_slot: &mut f64,
        var_dv0_dn2_slot: &mut f64,
        var_dv0_dn3_slot: &mut f64,
        var_dv0_dn4_slot: &mut f64,
        var_dv0_dn5_slot: &mut f64,
        var_dv0_dn6_slot: &mut f64,
        var_dv0_dn7_slot: &mut f64,
        var_dv0_dn8_slot: &mut f64,
        var_dv0_dn9_slot: &mut f64,
        var_dv0_rdb0_slot: &mut f64,
        var_dv0_rdb1_slot: &mut f64,
        var_dv0_rdb2_slot: &mut f64,
        var_dv0_rdb3_slot: &mut f64,
        var_dv0_rdb4_slot: &mut f64,
        var_dv0_rdb5_slot: &mut f64,
        var_dv0_rdb6_slot: &mut f64,
        var_dv0_rdb7_slot: &mut f64,
        var_dv0_rdn0_slot: &mut f64,
        var_dv0_rdn1_slot: &mut f64,
        var_dv0_rdn2_slot: &mut f64,
        var_dv0_rdn3_slot: &mut f64,
        var_dv0_rdn4_slot: &mut f64,
        var_dv0_rdn5_slot: &mut f64,
        var_dv0_rdn6_slot: &mut f64,
        var_dv0_rdn7_slot: &mut f64,
        var_dv0_rdn8_slot: &mut f64,
        var_dv0_rdn9_slot: &mut f64,
        var_dv0_rv_slot: &mut f64,
        var_dvh_slot: &mut f64,
        var_dvh_db0_slot: &mut f64,
        var_dvh_db1_slot: &mut f64,
        var_dvh_db2_slot: &mut f64,
        var_dvh_db3_slot: &mut f64,
        var_dvh_db4_slot: &mut f64,
        var_dvh_db5_slot: &mut f64,
        var_dvh_db6_slot: &mut f64,
        var_dvh_db7_slot: &mut f64,
        var_dvh_dn0_slot: &mut f64,
        var_dvh_dn1_slot: &mut f64,
        var_dvh_dn2_slot: &mut f64,
        var_dvh_dn3_slot: &mut f64,
        var_dvh_dn4_slot: &mut f64,
        var_dvh_dn5_slot: &mut f64,
        var_dvh_dn6_slot: &mut f64,
        var_dvh_dn7_slot: &mut f64,
        var_dvh_dn8_slot: &mut f64,
        var_dvh_dn9_slot: &mut f64,
        var_dvh_rdb0_slot: &mut f64,
        var_dvh_rdb1_slot: &mut f64,
        var_dvh_rdb2_slot: &mut f64,
        var_dvh_rdb3_slot: &mut f64,
        var_dvh_rdb4_slot: &mut f64,
        var_dvh_rdb5_slot: &mut f64,
        var_dvh_rdb6_slot: &mut f64,
        var_dvh_rdb7_slot: &mut f64,
        var_dvh_rdn0_slot: &mut f64,
        var_dvh_rdn1_slot: &mut f64,
        var_dvh_rdn2_slot: &mut f64,
        var_dvh_rdn3_slot: &mut f64,
        var_dvh_rdn4_slot: &mut f64,
        var_dvh_rdn5_slot: &mut f64,
        var_dvh_rdn6_slot: &mut f64,
        var_dvh_rdn7_slot: &mut f64,
        var_dvh_rdn8_slot: &mut f64,
        var_dvh_rdn9_slot: &mut f64,
        var_dvh_rv_slot: &mut f64,
        var_guard18_slot: &mut f64,
        var_guard18_db0_slot: &mut f64,
        var_guard18_db1_slot: &mut f64,
        var_guard18_db2_slot: &mut f64,
        var_guard18_db3_slot: &mut f64,
        var_guard18_db4_slot: &mut f64,
        var_guard18_db5_slot: &mut f64,
        var_guard18_db6_slot: &mut f64,
        var_guard18_db7_slot: &mut f64,
        var_guard18_dn0_slot: &mut f64,
        var_guard18_dn1_slot: &mut f64,
        var_guard18_dn2_slot: &mut f64,
        var_guard18_dn3_slot: &mut f64,
        var_guard18_dn4_slot: &mut f64,
        var_guard18_dn5_slot: &mut f64,
        var_guard18_dn6_slot: &mut f64,
        var_guard18_dn7_slot: &mut f64,
        var_guard18_dn8_slot: &mut f64,
        var_guard18_dn9_slot: &mut f64,
        var_guard18_rdb0_slot: &mut f64,
        var_guard18_rdb1_slot: &mut f64,
        var_guard18_rdb2_slot: &mut f64,
        var_guard18_rdb3_slot: &mut f64,
        var_guard18_rdb4_slot: &mut f64,
        var_guard18_rdb5_slot: &mut f64,
        var_guard18_rdb6_slot: &mut f64,
        var_guard18_rdb7_slot: &mut f64,
        var_guard18_rdn0_slot: &mut f64,
        var_guard18_rdn1_slot: &mut f64,
        var_guard18_rdn2_slot: &mut f64,
        var_guard18_rdn3_slot: &mut f64,
        var_guard18_rdn4_slot: &mut f64,
        var_guard18_rdn5_slot: &mut f64,
        var_guard18_rdn6_slot: &mut f64,
        var_guard18_rdn7_slot: &mut f64,
        var_guard18_rdn8_slot: &mut f64,
        var_guard18_rdn9_slot: &mut f64,
        var_guard18_rv_slot: &mut f64,
        var_pwq_slot: &mut f64,
        var_pwq_db0_slot: &mut f64,
        var_pwq_db1_slot: &mut f64,
        var_pwq_db2_slot: &mut f64,
        var_pwq_db3_slot: &mut f64,
        var_pwq_db4_slot: &mut f64,
        var_pwq_db5_slot: &mut f64,
        var_pwq_db6_slot: &mut f64,
        var_pwq_db7_slot: &mut f64,
        var_pwq_dn0_slot: &mut f64,
        var_pwq_dn1_slot: &mut f64,
        var_pwq_dn2_slot: &mut f64,
        var_pwq_dn3_slot: &mut f64,
        var_pwq_dn4_slot: &mut f64,
        var_pwq_dn5_slot: &mut f64,
        var_pwq_dn6_slot: &mut f64,
        var_pwq_dn7_slot: &mut f64,
        var_pwq_dn8_slot: &mut f64,
        var_pwq_dn9_slot: &mut f64,
        var_pwq_rdb0_slot: &mut f64,
        var_pwq_rdb1_slot: &mut f64,
        var_pwq_rdb2_slot: &mut f64,
        var_pwq_rdb3_slot: &mut f64,
        var_pwq_rdb4_slot: &mut f64,
        var_pwq_rdb5_slot: &mut f64,
        var_pwq_rdb6_slot: &mut f64,
        var_pwq_rdb7_slot: &mut f64,
        var_pwq_rdn0_slot: &mut f64,
        var_pwq_rdn1_slot: &mut f64,
        var_pwq_rdn2_slot: &mut f64,
        var_pwq_rdn3_slot: &mut f64,
        var_pwq_rdn4_slot: &mut f64,
        var_pwq_rdn5_slot: &mut f64,
        var_pwq_rdn6_slot: &mut f64,
        var_pwq_rdn7_slot: &mut f64,
        var_pwq_rdn8_slot: &mut f64,
        var_pwq_rdn9_slot: &mut f64,
        var_pwq_rv_slot: &mut f64,
        var_qhi_slot: &mut f64,
        var_qhi_db0_slot: &mut f64,
        var_qhi_db1_slot: &mut f64,
        var_qhi_db2_slot: &mut f64,
        var_qhi_db3_slot: &mut f64,
        var_qhi_db4_slot: &mut f64,
        var_qhi_db5_slot: &mut f64,
        var_qhi_db6_slot: &mut f64,
        var_qhi_db7_slot: &mut f64,
        var_qhi_dn0_slot: &mut f64,
        var_qhi_dn1_slot: &mut f64,
        var_qhi_dn2_slot: &mut f64,
        var_qhi_dn3_slot: &mut f64,
        var_qhi_dn4_slot: &mut f64,
        var_qhi_dn5_slot: &mut f64,
        var_qhi_dn6_slot: &mut f64,
        var_qhi_dn7_slot: &mut f64,
        var_qhi_dn8_slot: &mut f64,
        var_qhi_dn9_slot: &mut f64,
        var_qhi_rdb0_slot: &mut f64,
        var_qhi_rdb1_slot: &mut f64,
        var_qhi_rdb2_slot: &mut f64,
        var_qhi_rdb3_slot: &mut f64,
        var_qhi_rdb4_slot: &mut f64,
        var_qhi_rdb5_slot: &mut f64,
        var_qhi_rdb6_slot: &mut f64,
        var_qhi_rdb7_slot: &mut f64,
        var_qhi_rdn0_slot: &mut f64,
        var_qhi_rdn1_slot: &mut f64,
        var_qhi_rdn2_slot: &mut f64,
        var_qhi_rdn3_slot: &mut f64,
        var_qhi_rdn4_slot: &mut f64,
        var_qhi_rdn5_slot: &mut f64,
        var_qhi_rdn6_slot: &mut f64,
        var_qhi_rdn7_slot: &mut f64,
        var_qhi_rdn8_slot: &mut f64,
        var_qhi_rdn9_slot: &mut f64,
        var_qhi_rv_slot: &mut f64,
        var_qjcx_slot: &mut f64,
        var_qjcx_1_slot: &mut f64,
        var_qjcx_1_db0_slot: &mut f64,
        var_qjcx_1_db1_slot: &mut f64,
        var_qjcx_1_db2_slot: &mut f64,
        var_qjcx_1_db3_slot: &mut f64,
        var_qjcx_1_db4_slot: &mut f64,
        var_qjcx_1_db5_slot: &mut f64,
        var_qjcx_1_db6_slot: &mut f64,
        var_qjcx_1_db7_slot: &mut f64,
        var_qjcx_1_dn0_slot: &mut f64,
        var_qjcx_1_dn1_slot: &mut f64,
        var_qjcx_1_dn2_slot: &mut f64,
        var_qjcx_1_dn3_slot: &mut f64,
        var_qjcx_1_dn4_slot: &mut f64,
        var_qjcx_1_dn5_slot: &mut f64,
        var_qjcx_1_dn6_slot: &mut f64,
        var_qjcx_1_dn7_slot: &mut f64,
        var_qjcx_1_dn8_slot: &mut f64,
        var_qjcx_1_dn9_slot: &mut f64,
        var_qjcx_1_rdb0_slot: &mut f64,
        var_qjcx_1_rdb1_slot: &mut f64,
        var_qjcx_1_rdb2_slot: &mut f64,
        var_qjcx_1_rdb3_slot: &mut f64,
        var_qjcx_1_rdb4_slot: &mut f64,
        var_qjcx_1_rdb5_slot: &mut f64,
        var_qjcx_1_rdb6_slot: &mut f64,
        var_qjcx_1_rdb7_slot: &mut f64,
        var_qjcx_1_rdn0_slot: &mut f64,
        var_qjcx_1_rdn1_slot: &mut f64,
        var_qjcx_1_rdn2_slot: &mut f64,
        var_qjcx_1_rdn3_slot: &mut f64,
        var_qjcx_1_rdn4_slot: &mut f64,
        var_qjcx_1_rdn5_slot: &mut f64,
        var_qjcx_1_rdn6_slot: &mut f64,
        var_qjcx_1_rdn7_slot: &mut f64,
        var_qjcx_1_rdn8_slot: &mut f64,
        var_qjcx_1_rdn9_slot: &mut f64,
        var_qjcx_1_rv_slot: &mut f64,
        var_qjcx_db0_slot: &mut f64,
        var_qjcx_db1_slot: &mut f64,
        var_qjcx_db2_slot: &mut f64,
        var_qjcx_db3_slot: &mut f64,
        var_qjcx_db4_slot: &mut f64,
        var_qjcx_db5_slot: &mut f64,
        var_qjcx_db6_slot: &mut f64,
        var_qjcx_db7_slot: &mut f64,
        var_qjcx_dn0_slot: &mut f64,
        var_qjcx_dn1_slot: &mut f64,
        var_qjcx_dn2_slot: &mut f64,
        var_qjcx_dn3_slot: &mut f64,
        var_qjcx_dn4_slot: &mut f64,
        var_qjcx_dn5_slot: &mut f64,
        var_qjcx_dn6_slot: &mut f64,
        var_qjcx_dn7_slot: &mut f64,
        var_qjcx_dn8_slot: &mut f64,
        var_qjcx_dn9_slot: &mut f64,
        var_qjcx_rdb0_slot: &mut f64,
        var_qjcx_rdb1_slot: &mut f64,
        var_qjcx_rdb2_slot: &mut f64,
        var_qjcx_rdb3_slot: &mut f64,
        var_qjcx_rdb4_slot: &mut f64,
        var_qjcx_rdb5_slot: &mut f64,
        var_qjcx_rdb6_slot: &mut f64,
        var_qjcx_rdb7_slot: &mut f64,
        var_qjcx_rdn0_slot: &mut f64,
        var_qjcx_rdn1_slot: &mut f64,
        var_qjcx_rdn2_slot: &mut f64,
        var_qjcx_rdn3_slot: &mut f64,
        var_qjcx_rdn4_slot: &mut f64,
        var_qjcx_rdn5_slot: &mut f64,
        var_qjcx_rdn6_slot: &mut f64,
        var_qjcx_rdn7_slot: &mut f64,
        var_qjcx_rdn8_slot: &mut f64,
        var_qjcx_rdn9_slot: &mut f64,
        var_qjcx_rv_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo_db0_slot: &mut f64,
        var_qlo_db1_slot: &mut f64,
        var_qlo_db2_slot: &mut f64,
        var_qlo_db3_slot: &mut f64,
        var_qlo_db4_slot: &mut f64,
        var_qlo_db5_slot: &mut f64,
        var_qlo_db6_slot: &mut f64,
        var_qlo_db7_slot: &mut f64,
        var_qlo_dn0_slot: &mut f64,
        var_qlo_dn1_slot: &mut f64,
        var_qlo_dn2_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_qlo_dn5_slot: &mut f64,
        var_qlo_dn6_slot: &mut f64,
        var_qlo_dn7_slot: &mut f64,
        var_qlo_dn8_slot: &mut f64,
        var_qlo_dn9_slot: &mut f64,
        var_qlo_rdb0_slot: &mut f64,
        var_qlo_rdb1_slot: &mut f64,
        var_qlo_rdb2_slot: &mut f64,
        var_qlo_rdb3_slot: &mut f64,
        var_qlo_rdb4_slot: &mut f64,
        var_qlo_rdb5_slot: &mut f64,
        var_qlo_rdb6_slot: &mut f64,
        var_qlo_rdb7_slot: &mut f64,
        var_qlo_rdn0_slot: &mut f64,
        var_qlo_rdn1_slot: &mut f64,
        var_qlo_rdn2_slot: &mut f64,
        var_qlo_rdn3_slot: &mut f64,
        var_qlo_rdn4_slot: &mut f64,
        var_qlo_rdn5_slot: &mut f64,
        var_qlo_rdn6_slot: &mut f64,
        var_qlo_rdn7_slot: &mut f64,
        var_qlo_rdn8_slot: &mut f64,
        var_qlo_rdn9_slot: &mut f64,
        var_qlo_rv_slot: &mut f64,
    ) {
        let mut var_dv0: f64 = *var_dv0_slot;
        let mut var_dv0_db0: f64 = *var_dv0_db0_slot;
        let mut var_dv0_db1: f64 = *var_dv0_db1_slot;
        let mut var_dv0_db2: f64 = *var_dv0_db2_slot;
        let mut var_dv0_db3: f64 = *var_dv0_db3_slot;
        let mut var_dv0_db4: f64 = *var_dv0_db4_slot;
        let mut var_dv0_db5: f64 = *var_dv0_db5_slot;
        let mut var_dv0_db6: f64 = *var_dv0_db6_slot;
        let mut var_dv0_db7: f64 = *var_dv0_db7_slot;
        let mut var_dv0_dn0: f64 = *var_dv0_dn0_slot;
        let mut var_dv0_dn1: f64 = *var_dv0_dn1_slot;
        let mut var_dv0_dn2: f64 = *var_dv0_dn2_slot;
        let mut var_dv0_dn3: f64 = *var_dv0_dn3_slot;
        let mut var_dv0_dn4: f64 = *var_dv0_dn4_slot;
        let mut var_dv0_dn5: f64 = *var_dv0_dn5_slot;
        let mut var_dv0_dn6: f64 = *var_dv0_dn6_slot;
        let mut var_dv0_dn7: f64 = *var_dv0_dn7_slot;
        let mut var_dv0_dn8: f64 = *var_dv0_dn8_slot;
        let mut var_dv0_dn9: f64 = *var_dv0_dn9_slot;
        let mut var_dv0_rdb0: f64 = *var_dv0_rdb0_slot;
        let mut var_dv0_rdb1: f64 = *var_dv0_rdb1_slot;
        let mut var_dv0_rdb2: f64 = *var_dv0_rdb2_slot;
        let mut var_dv0_rdb3: f64 = *var_dv0_rdb3_slot;
        let mut var_dv0_rdb4: f64 = *var_dv0_rdb4_slot;
        let mut var_dv0_rdb5: f64 = *var_dv0_rdb5_slot;
        let mut var_dv0_rdb6: f64 = *var_dv0_rdb6_slot;
        let mut var_dv0_rdb7: f64 = *var_dv0_rdb7_slot;
        let mut var_dv0_rdn0: f64 = *var_dv0_rdn0_slot;
        let mut var_dv0_rdn1: f64 = *var_dv0_rdn1_slot;
        let mut var_dv0_rdn2: f64 = *var_dv0_rdn2_slot;
        let mut var_dv0_rdn3: f64 = *var_dv0_rdn3_slot;
        let mut var_dv0_rdn4: f64 = *var_dv0_rdn4_slot;
        let mut var_dv0_rdn5: f64 = *var_dv0_rdn5_slot;
        let mut var_dv0_rdn6: f64 = *var_dv0_rdn6_slot;
        let mut var_dv0_rdn7: f64 = *var_dv0_rdn7_slot;
        let mut var_dv0_rdn8: f64 = *var_dv0_rdn8_slot;
        let mut var_dv0_rdn9: f64 = *var_dv0_rdn9_slot;
        let mut var_dv0_rv: f64 = *var_dv0_rv_slot;
        let mut var_dvh: f64 = *var_dvh_slot;
        let mut var_dvh_db0: f64 = *var_dvh_db0_slot;
        let mut var_dvh_db1: f64 = *var_dvh_db1_slot;
        let mut var_dvh_db2: f64 = *var_dvh_db2_slot;
        let mut var_dvh_db3: f64 = *var_dvh_db3_slot;
        let mut var_dvh_db4: f64 = *var_dvh_db4_slot;
        let mut var_dvh_db5: f64 = *var_dvh_db5_slot;
        let mut var_dvh_db6: f64 = *var_dvh_db6_slot;
        let mut var_dvh_db7: f64 = *var_dvh_db7_slot;
        let mut var_dvh_dn0: f64 = *var_dvh_dn0_slot;
        let mut var_dvh_dn1: f64 = *var_dvh_dn1_slot;
        let mut var_dvh_dn2: f64 = *var_dvh_dn2_slot;
        let mut var_dvh_dn3: f64 = *var_dvh_dn3_slot;
        let mut var_dvh_dn4: f64 = *var_dvh_dn4_slot;
        let mut var_dvh_dn5: f64 = *var_dvh_dn5_slot;
        let mut var_dvh_dn6: f64 = *var_dvh_dn6_slot;
        let mut var_dvh_dn7: f64 = *var_dvh_dn7_slot;
        let mut var_dvh_dn8: f64 = *var_dvh_dn8_slot;
        let mut var_dvh_dn9: f64 = *var_dvh_dn9_slot;
        let mut var_dvh_rdb0: f64 = *var_dvh_rdb0_slot;
        let mut var_dvh_rdb1: f64 = *var_dvh_rdb1_slot;
        let mut var_dvh_rdb2: f64 = *var_dvh_rdb2_slot;
        let mut var_dvh_rdb3: f64 = *var_dvh_rdb3_slot;
        let mut var_dvh_rdb4: f64 = *var_dvh_rdb4_slot;
        let mut var_dvh_rdb5: f64 = *var_dvh_rdb5_slot;
        let mut var_dvh_rdb6: f64 = *var_dvh_rdb6_slot;
        let mut var_dvh_rdb7: f64 = *var_dvh_rdb7_slot;
        let mut var_dvh_rdn0: f64 = *var_dvh_rdn0_slot;
        let mut var_dvh_rdn1: f64 = *var_dvh_rdn1_slot;
        let mut var_dvh_rdn2: f64 = *var_dvh_rdn2_slot;
        let mut var_dvh_rdn3: f64 = *var_dvh_rdn3_slot;
        let mut var_dvh_rdn4: f64 = *var_dvh_rdn4_slot;
        let mut var_dvh_rdn5: f64 = *var_dvh_rdn5_slot;
        let mut var_dvh_rdn6: f64 = *var_dvh_rdn6_slot;
        let mut var_dvh_rdn7: f64 = *var_dvh_rdn7_slot;
        let mut var_dvh_rdn8: f64 = *var_dvh_rdn8_slot;
        let mut var_dvh_rdn9: f64 = *var_dvh_rdn9_slot;
        let mut var_dvh_rv: f64 = *var_dvh_rv_slot;
        let mut var_guard18: f64 = *var_guard18_slot;
        let mut var_guard18_db0: f64 = *var_guard18_db0_slot;
        let mut var_guard18_db1: f64 = *var_guard18_db1_slot;
        let mut var_guard18_db2: f64 = *var_guard18_db2_slot;
        let mut var_guard18_db3: f64 = *var_guard18_db3_slot;
        let mut var_guard18_db4: f64 = *var_guard18_db4_slot;
        let mut var_guard18_db5: f64 = *var_guard18_db5_slot;
        let mut var_guard18_db6: f64 = *var_guard18_db6_slot;
        let mut var_guard18_db7: f64 = *var_guard18_db7_slot;
        let mut var_guard18_dn0: f64 = *var_guard18_dn0_slot;
        let mut var_guard18_dn1: f64 = *var_guard18_dn1_slot;
        let mut var_guard18_dn2: f64 = *var_guard18_dn2_slot;
        let mut var_guard18_dn3: f64 = *var_guard18_dn3_slot;
        let mut var_guard18_dn4: f64 = *var_guard18_dn4_slot;
        let mut var_guard18_dn5: f64 = *var_guard18_dn5_slot;
        let mut var_guard18_dn6: f64 = *var_guard18_dn6_slot;
        let mut var_guard18_dn7: f64 = *var_guard18_dn7_slot;
        let mut var_guard18_dn8: f64 = *var_guard18_dn8_slot;
        let mut var_guard18_dn9: f64 = *var_guard18_dn9_slot;
        let mut var_guard18_rdb0: f64 = *var_guard18_rdb0_slot;
        let mut var_guard18_rdb1: f64 = *var_guard18_rdb1_slot;
        let mut var_guard18_rdb2: f64 = *var_guard18_rdb2_slot;
        let mut var_guard18_rdb3: f64 = *var_guard18_rdb3_slot;
        let mut var_guard18_rdb4: f64 = *var_guard18_rdb4_slot;
        let mut var_guard18_rdb5: f64 = *var_guard18_rdb5_slot;
        let mut var_guard18_rdb6: f64 = *var_guard18_rdb6_slot;
        let mut var_guard18_rdb7: f64 = *var_guard18_rdb7_slot;
        let mut var_guard18_rdn0: f64 = *var_guard18_rdn0_slot;
        let mut var_guard18_rdn1: f64 = *var_guard18_rdn1_slot;
        let mut var_guard18_rdn2: f64 = *var_guard18_rdn2_slot;
        let mut var_guard18_rdn3: f64 = *var_guard18_rdn3_slot;
        let mut var_guard18_rdn4: f64 = *var_guard18_rdn4_slot;
        let mut var_guard18_rdn5: f64 = *var_guard18_rdn5_slot;
        let mut var_guard18_rdn6: f64 = *var_guard18_rdn6_slot;
        let mut var_guard18_rdn7: f64 = *var_guard18_rdn7_slot;
        let mut var_guard18_rdn8: f64 = *var_guard18_rdn8_slot;
        let mut var_guard18_rdn9: f64 = *var_guard18_rdn9_slot;
        let mut var_guard18_rv: f64 = *var_guard18_rv_slot;
        let mut var_pwq: f64 = *var_pwq_slot;
        let mut var_pwq_db0: f64 = *var_pwq_db0_slot;
        let mut var_pwq_db1: f64 = *var_pwq_db1_slot;
        let mut var_pwq_db2: f64 = *var_pwq_db2_slot;
        let mut var_pwq_db3: f64 = *var_pwq_db3_slot;
        let mut var_pwq_db4: f64 = *var_pwq_db4_slot;
        let mut var_pwq_db5: f64 = *var_pwq_db5_slot;
        let mut var_pwq_db6: f64 = *var_pwq_db6_slot;
        let mut var_pwq_db7: f64 = *var_pwq_db7_slot;
        let mut var_pwq_dn0: f64 = *var_pwq_dn0_slot;
        let mut var_pwq_dn1: f64 = *var_pwq_dn1_slot;
        let mut var_pwq_dn2: f64 = *var_pwq_dn2_slot;
        let mut var_pwq_dn3: f64 = *var_pwq_dn3_slot;
        let mut var_pwq_dn4: f64 = *var_pwq_dn4_slot;
        let mut var_pwq_dn5: f64 = *var_pwq_dn5_slot;
        let mut var_pwq_dn6: f64 = *var_pwq_dn6_slot;
        let mut var_pwq_dn7: f64 = *var_pwq_dn7_slot;
        let mut var_pwq_dn8: f64 = *var_pwq_dn8_slot;
        let mut var_pwq_dn9: f64 = *var_pwq_dn9_slot;
        let mut var_pwq_rdb0: f64 = *var_pwq_rdb0_slot;
        let mut var_pwq_rdb1: f64 = *var_pwq_rdb1_slot;
        let mut var_pwq_rdb2: f64 = *var_pwq_rdb2_slot;
        let mut var_pwq_rdb3: f64 = *var_pwq_rdb3_slot;
        let mut var_pwq_rdb4: f64 = *var_pwq_rdb4_slot;
        let mut var_pwq_rdb5: f64 = *var_pwq_rdb5_slot;
        let mut var_pwq_rdb6: f64 = *var_pwq_rdb6_slot;
        let mut var_pwq_rdb7: f64 = *var_pwq_rdb7_slot;
        let mut var_pwq_rdn0: f64 = *var_pwq_rdn0_slot;
        let mut var_pwq_rdn1: f64 = *var_pwq_rdn1_slot;
        let mut var_pwq_rdn2: f64 = *var_pwq_rdn2_slot;
        let mut var_pwq_rdn3: f64 = *var_pwq_rdn3_slot;
        let mut var_pwq_rdn4: f64 = *var_pwq_rdn4_slot;
        let mut var_pwq_rdn5: f64 = *var_pwq_rdn5_slot;
        let mut var_pwq_rdn6: f64 = *var_pwq_rdn6_slot;
        let mut var_pwq_rdn7: f64 = *var_pwq_rdn7_slot;
        let mut var_pwq_rdn8: f64 = *var_pwq_rdn8_slot;
        let mut var_pwq_rdn9: f64 = *var_pwq_rdn9_slot;
        let mut var_pwq_rv: f64 = *var_pwq_rv_slot;
        let mut var_qhi: f64 = *var_qhi_slot;
        let mut var_qhi_db0: f64 = *var_qhi_db0_slot;
        let mut var_qhi_db1: f64 = *var_qhi_db1_slot;
        let mut var_qhi_db2: f64 = *var_qhi_db2_slot;
        let mut var_qhi_db3: f64 = *var_qhi_db3_slot;
        let mut var_qhi_db4: f64 = *var_qhi_db4_slot;
        let mut var_qhi_db5: f64 = *var_qhi_db5_slot;
        let mut var_qhi_db6: f64 = *var_qhi_db6_slot;
        let mut var_qhi_db7: f64 = *var_qhi_db7_slot;
        let mut var_qhi_dn0: f64 = *var_qhi_dn0_slot;
        let mut var_qhi_dn1: f64 = *var_qhi_dn1_slot;
        let mut var_qhi_dn2: f64 = *var_qhi_dn2_slot;
        let mut var_qhi_dn3: f64 = *var_qhi_dn3_slot;
        let mut var_qhi_dn4: f64 = *var_qhi_dn4_slot;
        let mut var_qhi_dn5: f64 = *var_qhi_dn5_slot;
        let mut var_qhi_dn6: f64 = *var_qhi_dn6_slot;
        let mut var_qhi_dn7: f64 = *var_qhi_dn7_slot;
        let mut var_qhi_dn8: f64 = *var_qhi_dn8_slot;
        let mut var_qhi_dn9: f64 = *var_qhi_dn9_slot;
        let mut var_qhi_rdb0: f64 = *var_qhi_rdb0_slot;
        let mut var_qhi_rdb1: f64 = *var_qhi_rdb1_slot;
        let mut var_qhi_rdb2: f64 = *var_qhi_rdb2_slot;
        let mut var_qhi_rdb3: f64 = *var_qhi_rdb3_slot;
        let mut var_qhi_rdb4: f64 = *var_qhi_rdb4_slot;
        let mut var_qhi_rdb5: f64 = *var_qhi_rdb5_slot;
        let mut var_qhi_rdb6: f64 = *var_qhi_rdb6_slot;
        let mut var_qhi_rdb7: f64 = *var_qhi_rdb7_slot;
        let mut var_qhi_rdn0: f64 = *var_qhi_rdn0_slot;
        let mut var_qhi_rdn1: f64 = *var_qhi_rdn1_slot;
        let mut var_qhi_rdn2: f64 = *var_qhi_rdn2_slot;
        let mut var_qhi_rdn3: f64 = *var_qhi_rdn3_slot;
        let mut var_qhi_rdn4: f64 = *var_qhi_rdn4_slot;
        let mut var_qhi_rdn5: f64 = *var_qhi_rdn5_slot;
        let mut var_qhi_rdn6: f64 = *var_qhi_rdn6_slot;
        let mut var_qhi_rdn7: f64 = *var_qhi_rdn7_slot;
        let mut var_qhi_rdn8: f64 = *var_qhi_rdn8_slot;
        let mut var_qhi_rdn9: f64 = *var_qhi_rdn9_slot;
        let mut var_qhi_rv: f64 = *var_qhi_rv_slot;
        let mut var_qjcx: f64 = *var_qjcx_slot;
        let mut var_qjcx_1: f64 = *var_qjcx_1_slot;
        let mut var_qjcx_1_db0: f64 = *var_qjcx_1_db0_slot;
        let mut var_qjcx_1_db1: f64 = *var_qjcx_1_db1_slot;
        let mut var_qjcx_1_db2: f64 = *var_qjcx_1_db2_slot;
        let mut var_qjcx_1_db3: f64 = *var_qjcx_1_db3_slot;
        let mut var_qjcx_1_db4: f64 = *var_qjcx_1_db4_slot;
        let mut var_qjcx_1_db5: f64 = *var_qjcx_1_db5_slot;
        let mut var_qjcx_1_db6: f64 = *var_qjcx_1_db6_slot;
        let mut var_qjcx_1_db7: f64 = *var_qjcx_1_db7_slot;
        let mut var_qjcx_1_dn0: f64 = *var_qjcx_1_dn0_slot;
        let mut var_qjcx_1_dn1: f64 = *var_qjcx_1_dn1_slot;
        let mut var_qjcx_1_dn2: f64 = *var_qjcx_1_dn2_slot;
        let mut var_qjcx_1_dn3: f64 = *var_qjcx_1_dn3_slot;
        let mut var_qjcx_1_dn4: f64 = *var_qjcx_1_dn4_slot;
        let mut var_qjcx_1_dn5: f64 = *var_qjcx_1_dn5_slot;
        let mut var_qjcx_1_dn6: f64 = *var_qjcx_1_dn6_slot;
        let mut var_qjcx_1_dn7: f64 = *var_qjcx_1_dn7_slot;
        let mut var_qjcx_1_dn8: f64 = *var_qjcx_1_dn8_slot;
        let mut var_qjcx_1_dn9: f64 = *var_qjcx_1_dn9_slot;
        let mut var_qjcx_1_rdb0: f64 = *var_qjcx_1_rdb0_slot;
        let mut var_qjcx_1_rdb1: f64 = *var_qjcx_1_rdb1_slot;
        let mut var_qjcx_1_rdb2: f64 = *var_qjcx_1_rdb2_slot;
        let mut var_qjcx_1_rdb3: f64 = *var_qjcx_1_rdb3_slot;
        let mut var_qjcx_1_rdb4: f64 = *var_qjcx_1_rdb4_slot;
        let mut var_qjcx_1_rdb5: f64 = *var_qjcx_1_rdb5_slot;
        let mut var_qjcx_1_rdb6: f64 = *var_qjcx_1_rdb6_slot;
        let mut var_qjcx_1_rdb7: f64 = *var_qjcx_1_rdb7_slot;
        let mut var_qjcx_1_rdn0: f64 = *var_qjcx_1_rdn0_slot;
        let mut var_qjcx_1_rdn1: f64 = *var_qjcx_1_rdn1_slot;
        let mut var_qjcx_1_rdn2: f64 = *var_qjcx_1_rdn2_slot;
        let mut var_qjcx_1_rdn3: f64 = *var_qjcx_1_rdn3_slot;
        let mut var_qjcx_1_rdn4: f64 = *var_qjcx_1_rdn4_slot;
        let mut var_qjcx_1_rdn5: f64 = *var_qjcx_1_rdn5_slot;
        let mut var_qjcx_1_rdn6: f64 = *var_qjcx_1_rdn6_slot;
        let mut var_qjcx_1_rdn7: f64 = *var_qjcx_1_rdn7_slot;
        let mut var_qjcx_1_rdn8: f64 = *var_qjcx_1_rdn8_slot;
        let mut var_qjcx_1_rdn9: f64 = *var_qjcx_1_rdn9_slot;
        let mut var_qjcx_1_rv: f64 = *var_qjcx_1_rv_slot;
        let mut var_qjcx_db0: f64 = *var_qjcx_db0_slot;
        let mut var_qjcx_db1: f64 = *var_qjcx_db1_slot;
        let mut var_qjcx_db2: f64 = *var_qjcx_db2_slot;
        let mut var_qjcx_db3: f64 = *var_qjcx_db3_slot;
        let mut var_qjcx_db4: f64 = *var_qjcx_db4_slot;
        let mut var_qjcx_db5: f64 = *var_qjcx_db5_slot;
        let mut var_qjcx_db6: f64 = *var_qjcx_db6_slot;
        let mut var_qjcx_db7: f64 = *var_qjcx_db7_slot;
        let mut var_qjcx_dn0: f64 = *var_qjcx_dn0_slot;
        let mut var_qjcx_dn1: f64 = *var_qjcx_dn1_slot;
        let mut var_qjcx_dn2: f64 = *var_qjcx_dn2_slot;
        let mut var_qjcx_dn3: f64 = *var_qjcx_dn3_slot;
        let mut var_qjcx_dn4: f64 = *var_qjcx_dn4_slot;
        let mut var_qjcx_dn5: f64 = *var_qjcx_dn5_slot;
        let mut var_qjcx_dn6: f64 = *var_qjcx_dn6_slot;
        let mut var_qjcx_dn7: f64 = *var_qjcx_dn7_slot;
        let mut var_qjcx_dn8: f64 = *var_qjcx_dn8_slot;
        let mut var_qjcx_dn9: f64 = *var_qjcx_dn9_slot;
        let mut var_qjcx_rdb0: f64 = *var_qjcx_rdb0_slot;
        let mut var_qjcx_rdb1: f64 = *var_qjcx_rdb1_slot;
        let mut var_qjcx_rdb2: f64 = *var_qjcx_rdb2_slot;
        let mut var_qjcx_rdb3: f64 = *var_qjcx_rdb3_slot;
        let mut var_qjcx_rdb4: f64 = *var_qjcx_rdb4_slot;
        let mut var_qjcx_rdb5: f64 = *var_qjcx_rdb5_slot;
        let mut var_qjcx_rdb6: f64 = *var_qjcx_rdb6_slot;
        let mut var_qjcx_rdb7: f64 = *var_qjcx_rdb7_slot;
        let mut var_qjcx_rdn0: f64 = *var_qjcx_rdn0_slot;
        let mut var_qjcx_rdn1: f64 = *var_qjcx_rdn1_slot;
        let mut var_qjcx_rdn2: f64 = *var_qjcx_rdn2_slot;
        let mut var_qjcx_rdn3: f64 = *var_qjcx_rdn3_slot;
        let mut var_qjcx_rdn4: f64 = *var_qjcx_rdn4_slot;
        let mut var_qjcx_rdn5: f64 = *var_qjcx_rdn5_slot;
        let mut var_qjcx_rdn6: f64 = *var_qjcx_rdn6_slot;
        let mut var_qjcx_rdn7: f64 = *var_qjcx_rdn7_slot;
        let mut var_qjcx_rdn8: f64 = *var_qjcx_rdn8_slot;
        let mut var_qjcx_rdn9: f64 = *var_qjcx_rdn9_slot;
        let mut var_qjcx_rv: f64 = *var_qjcx_rv_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo_db0: f64 = *var_qlo_db0_slot;
        let mut var_qlo_db1: f64 = *var_qlo_db1_slot;
        let mut var_qlo_db2: f64 = *var_qlo_db2_slot;
        let mut var_qlo_db3: f64 = *var_qlo_db3_slot;
        let mut var_qlo_db4: f64 = *var_qlo_db4_slot;
        let mut var_qlo_db5: f64 = *var_qlo_db5_slot;
        let mut var_qlo_db6: f64 = *var_qlo_db6_slot;
        let mut var_qlo_db7: f64 = *var_qlo_db7_slot;
        let mut var_qlo_dn0: f64 = *var_qlo_dn0_slot;
        let mut var_qlo_dn1: f64 = *var_qlo_dn1_slot;
        let mut var_qlo_dn2: f64 = *var_qlo_dn2_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_qlo_dn5: f64 = *var_qlo_dn5_slot;
        let mut var_qlo_dn6: f64 = *var_qlo_dn6_slot;
        let mut var_qlo_dn7: f64 = *var_qlo_dn7_slot;
        let mut var_qlo_dn8: f64 = *var_qlo_dn8_slot;
        let mut var_qlo_dn9: f64 = *var_qlo_dn9_slot;
        let mut var_qlo_rdb0: f64 = *var_qlo_rdb0_slot;
        let mut var_qlo_rdb1: f64 = *var_qlo_rdb1_slot;
        let mut var_qlo_rdb2: f64 = *var_qlo_rdb2_slot;
        let mut var_qlo_rdb3: f64 = *var_qlo_rdb3_slot;
        let mut var_qlo_rdb4: f64 = *var_qlo_rdb4_slot;
        let mut var_qlo_rdb5: f64 = *var_qlo_rdb5_slot;
        let mut var_qlo_rdb6: f64 = *var_qlo_rdb6_slot;
        let mut var_qlo_rdb7: f64 = *var_qlo_rdb7_slot;
        let mut var_qlo_rdn0: f64 = *var_qlo_rdn0_slot;
        let mut var_qlo_rdn1: f64 = *var_qlo_rdn1_slot;
        let mut var_qlo_rdn2: f64 = *var_qlo_rdn2_slot;
        let mut var_qlo_rdn3: f64 = *var_qlo_rdn3_slot;
        let mut var_qlo_rdn4: f64 = *var_qlo_rdn4_slot;
        let mut var_qlo_rdn5: f64 = *var_qlo_rdn5_slot;
        let mut var_qlo_rdn6: f64 = *var_qlo_rdn6_slot;
        let mut var_qlo_rdn7: f64 = *var_qlo_rdn7_slot;
        let mut var_qlo_rdn8: f64 = *var_qlo_rdn8_slot;
        let mut var_qlo_rdn9: f64 = *var_qlo_rdn9_slot;
        let mut var_qlo_rv: f64 = *var_qlo_rv_slot;

        let (assign1760_e2076, assign1760_e2076_d_n0, assign1760_e2076_d_n1, assign1760_e2076_d_n2, assign1760_e2076_d_n3, assign1760_e2076_d_n4, assign1760_e2076_d_n5, assign1760_e2076_d_n6, assign1760_e2076_d_n7, assign1760_e2076_d_n8, assign1760_e2076_d_n9, assign1760_e2076_d_b0, assign1760_e2076_d_b1, assign1760_e2076_d_b2, assign1760_e2076_d_b3, assign1760_e2076_d_b4, assign1760_e2076_d_b5, assign1760_e2076_d_b6, assign1760_e2076_d_b7,) = {
    if (var_guard17 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_dn7, var_qhi_dn8, var_qhi_dn9, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6, var_qhi_db7,)
    }
};
        var_qhi = assign1760_e2076;
        var_qhi_dn0 = assign1760_e2076_d_n0;
        var_qhi_dn1 = assign1760_e2076_d_n1;
        var_qhi_dn2 = assign1760_e2076_d_n2;
        var_qhi_dn3 = assign1760_e2076_d_n3;
        var_qhi_dn4 = assign1760_e2076_d_n4;
        var_qhi_dn5 = assign1760_e2076_d_n5;
        var_qhi_dn6 = assign1760_e2076_d_n6;
        var_qhi_dn7 = assign1760_e2076_d_n7;
        var_qhi_dn8 = assign1760_e2076_d_n8;
        var_qhi_dn9 = assign1760_e2076_d_n9;
        var_qhi_db0 = assign1760_e2076_d_b0;
        var_qhi_db1 = assign1760_e2076_d_b1;
        var_qhi_db2 = assign1760_e2076_d_b2;
        var_qhi_db3 = assign1760_e2076_d_b3;
        var_qhi_db4 = assign1760_e2076_d_b4;
        var_qhi_db5 = assign1760_e2076_d_b5;
        var_qhi_db6 = assign1760_e2076_d_b6;
        var_qhi_db7 = assign1760_e2076_d_b7;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdn7 = 0.0;
        var_qhi_rdn8 = 0.0;
        var_qhi_rdn9 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;
        var_qhi_rdb7 = 0.0;

        let assign1770_e2080: f64 = (var_qlo + var_qhi);
        let assign1770_e2081: f64 = (var_cjc_t * assign1770_e2080);
        var_qjcx = assign1770_e2081;
        var_qjcx_dn0 = ((var_cjc_t_dn0 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn0 + var_qhi_dn0)));
        var_qjcx_dn1 = ((var_cjc_t_dn1 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn1 + var_qhi_dn1)));
        var_qjcx_dn2 = ((var_cjc_t_dn2 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn2 + var_qhi_dn2)));
        var_qjcx_dn3 = ((var_cjc_t_dn3 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qjcx_dn4 = ((var_cjc_t_dn4 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn4 + var_qhi_dn4)));
        var_qjcx_dn5 = ((var_cjc_t_dn5 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn5 + var_qhi_dn5)));
        var_qjcx_dn6 = ((var_cjc_t_dn6 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn6 + var_qhi_dn6)));
        var_qjcx_dn7 = ((var_cjc_t_dn7 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn7 + var_qhi_dn7)));
        var_qjcx_dn8 = ((var_cjc_t_dn8 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn8 + var_qhi_dn8)));
        var_qjcx_dn9 = ((var_cjc_t_dn9 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn9 + var_qhi_dn9)));
        var_qjcx_db0 = ((var_cjc_t_db0 * assign1770_e2080) + (var_cjc_t * (var_qlo_db0 + var_qhi_db0)));
        var_qjcx_db1 = ((var_cjc_t_db1 * assign1770_e2080) + (var_cjc_t * (var_qlo_db1 + var_qhi_db1)));
        var_qjcx_db2 = ((var_cjc_t_db2 * assign1770_e2080) + (var_cjc_t * (var_qlo_db2 + var_qhi_db2)));
        var_qjcx_db3 = ((var_cjc_t_db3 * assign1770_e2080) + (var_cjc_t * (var_qlo_db3 + var_qhi_db3)));
        var_qjcx_db4 = ((var_cjc_t_db4 * assign1770_e2080) + (var_cjc_t * (var_qlo_db4 + var_qhi_db4)));
        var_qjcx_db5 = ((var_cjc_t_db5 * assign1770_e2080) + (var_cjc_t * (var_qlo_db5 + var_qhi_db5)));
        var_qjcx_db6 = ((var_cjc_t_db6 * assign1770_e2080) + (var_cjc_t * (var_qlo_db6 + var_qhi_db6)));
        var_qjcx_db7 = ((var_cjc_t_db7 * assign1770_e2080) + (var_cjc_t * (var_qlo_db7 + var_qhi_db7)));
        var_qjcx_rv = 0.0;
        var_qjcx_rdn0 = 0.0;
        var_qjcx_rdn1 = 0.0;
        var_qjcx_rdn2 = 0.0;
        var_qjcx_rdn3 = 0.0;
        var_qjcx_rdn4 = 0.0;
        var_qjcx_rdn5 = 0.0;
        var_qjcx_rdn6 = 0.0;
        var_qjcx_rdn7 = 0.0;
        var_qjcx_rdn8 = 0.0;
        var_qjcx_rdn9 = 0.0;
        var_qjcx_rdb0 = 0.0;
        var_qjcx_rdb1 = 0.0;
        var_qjcx_rdb2 = 0.0;
        var_qjcx_rdb3 = 0.0;
        var_qjcx_rdb4 = 0.0;
        var_qjcx_rdb5 = 0.0;
        var_qjcx_rdb6 = 0.0;
        var_qjcx_rdb7 = 0.0;

        let assign1780_e2084: f64 = (1.0 - p.p72);
        let assign1780_e2086: f64 = (assign1780_e2084 * var_qjcx);
        var_qjcx_1 = assign1780_e2086;
        var_qjcx_1_dn0 = (assign1780_e2084 * var_qjcx_dn0);
        var_qjcx_1_dn1 = (assign1780_e2084 * var_qjcx_dn1);
        var_qjcx_1_dn2 = (assign1780_e2084 * var_qjcx_dn2);
        var_qjcx_1_dn3 = (assign1780_e2084 * var_qjcx_dn3);
        var_qjcx_1_dn4 = (assign1780_e2084 * var_qjcx_dn4);
        var_qjcx_1_dn5 = (assign1780_e2084 * var_qjcx_dn5);
        var_qjcx_1_dn6 = (assign1780_e2084 * var_qjcx_dn6);
        var_qjcx_1_dn7 = (assign1780_e2084 * var_qjcx_dn7);
        var_qjcx_1_dn8 = (assign1780_e2084 * var_qjcx_dn8);
        var_qjcx_1_dn9 = (assign1780_e2084 * var_qjcx_dn9);
        var_qjcx_1_db0 = (assign1780_e2084 * var_qjcx_db0);
        var_qjcx_1_db1 = (assign1780_e2084 * var_qjcx_db1);
        var_qjcx_1_db2 = (assign1780_e2084 * var_qjcx_db2);
        var_qjcx_1_db3 = (assign1780_e2084 * var_qjcx_db3);
        var_qjcx_1_db4 = (assign1780_e2084 * var_qjcx_db4);
        var_qjcx_1_db5 = (assign1780_e2084 * var_qjcx_db5);
        var_qjcx_1_db6 = (assign1780_e2084 * var_qjcx_db6);
        var_qjcx_1_db7 = (assign1780_e2084 * var_qjcx_db7);
        var_qjcx_1_rv = 0.0;
        var_qjcx_1_rdn0 = 0.0;
        var_qjcx_1_rdn1 = 0.0;
        var_qjcx_1_rdn2 = 0.0;
        var_qjcx_1_rdn3 = 0.0;
        var_qjcx_1_rdn4 = 0.0;
        var_qjcx_1_rdn5 = 0.0;
        var_qjcx_1_rdn6 = 0.0;
        var_qjcx_1_rdn7 = 0.0;
        var_qjcx_1_rdn8 = 0.0;
        var_qjcx_1_rdn9 = 0.0;
        var_qjcx_1_rdb0 = 0.0;
        var_qjcx_1_rdb1 = 0.0;
        var_qjcx_1_rdb2 = 0.0;
        var_qjcx_1_rdb3 = 0.0;
        var_qjcx_1_rdb4 = 0.0;
        var_qjcx_1_rdb5 = 0.0;
        var_qjcx_1_rdb6 = 0.0;
        var_qjcx_1_rdb7 = 0.0;

        let assign1790_e2088: f64 = (-var_vjc_t);
        let assign1790_e2090: f64 = (assign1790_e2088 * p.p24);
        var_dv0 = assign1790_e2090;
        var_dv0_dn0 = ((-var_vjc_t_dn0) * p.p24);
        var_dv0_dn1 = ((-var_vjc_t_dn1) * p.p24);
        var_dv0_dn2 = ((-var_vjc_t_dn2) * p.p24);
        var_dv0_dn3 = ((-var_vjc_t_dn3) * p.p24);
        var_dv0_dn4 = ((-var_vjc_t_dn4) * p.p24);
        var_dv0_dn5 = ((-var_vjc_t_dn5) * p.p24);
        var_dv0_dn6 = ((-var_vjc_t_dn6) * p.p24);
        var_dv0_dn7 = ((-var_vjc_t_dn7) * p.p24);
        var_dv0_dn8 = ((-var_vjc_t_dn8) * p.p24);
        var_dv0_dn9 = ((-var_vjc_t_dn9) * p.p24);
        var_dv0_db0 = ((-var_vjc_t_db0) * p.p24);
        var_dv0_db1 = ((-var_vjc_t_db1) * p.p24);
        var_dv0_db2 = ((-var_vjc_t_db2) * p.p24);
        var_dv0_db3 = ((-var_vjc_t_db3) * p.p24);
        var_dv0_db4 = ((-var_vjc_t_db4) * p.p24);
        var_dv0_db5 = ((-var_vjc_t_db5) * p.p24);
        var_dv0_db6 = ((-var_vjc_t_db6) * p.p24);
        var_dv0_db7 = ((-var_vjc_t_db7) * p.p24);
        var_dv0_rv = 0.0;
        var_dv0_rdn0 = 0.0;
        var_dv0_rdn1 = 0.0;
        var_dv0_rdn2 = 0.0;
        var_dv0_rdn3 = 0.0;
        var_dv0_rdn4 = 0.0;
        var_dv0_rdn5 = 0.0;
        var_dv0_rdn6 = 0.0;
        var_dv0_rdn7 = 0.0;
        var_dv0_rdn8 = 0.0;
        var_dv0_rdn9 = 0.0;
        var_dv0_rdb0 = 0.0;
        var_dv0_rdb1 = 0.0;
        var_dv0_rdb2 = 0.0;
        var_dv0_rdb3 = 0.0;
        var_dv0_rdb4 = 0.0;
        var_dv0_rdb5 = 0.0;
        var_dv0_rdb6 = 0.0;
        var_dv0_rdb7 = 0.0;

        let assign1800_e2093: f64 = (var_vbici + var_dv0);
        var_dvh = assign1800_e2093;
        var_dvh_dn0 = (var_vbici_dn0 + var_dv0_dn0);
        var_dvh_dn1 = (var_vbici_dn1 + var_dv0_dn1);
        var_dvh_dn2 = (var_vbici_dn2 + var_dv0_dn2);
        var_dvh_dn3 = (var_vbici_dn3 + var_dv0_dn3);
        var_dvh_dn4 = (var_vbici_dn4 + var_dv0_dn4);
        var_dvh_dn5 = (var_vbici_dn5 + var_dv0_dn5);
        var_dvh_dn6 = (var_vbici_dn6 + var_dv0_dn6);
        var_dvh_dn7 = (var_vbici_dn7 + var_dv0_dn7);
        var_dvh_dn8 = (var_vbici_dn8 + var_dv0_dn8);
        var_dvh_dn9 = (var_vbici_dn9 + var_dv0_dn9);
        var_dvh_db0 = (var_vbici_db0 + var_dv0_db0);
        var_dvh_db1 = (var_vbici_db1 + var_dv0_db1);
        var_dvh_db2 = (var_vbici_db2 + var_dv0_db2);
        var_dvh_db3 = (var_vbici_db3 + var_dv0_db3);
        var_dvh_db4 = (var_vbici_db4 + var_dv0_db4);
        var_dvh_db5 = (var_vbici_db5 + var_dv0_db5);
        var_dvh_db6 = (var_vbici_db6 + var_dv0_db6);
        var_dvh_db7 = (var_vbici_db7 + var_dv0_db7);
        var_dvh_rv = 0.0;
        var_dvh_rdn0 = 0.0;
        var_dvh_rdn1 = 0.0;
        var_dvh_rdn2 = 0.0;
        var_dvh_rdn3 = 0.0;
        var_dvh_rdn4 = 0.0;
        var_dvh_rdn5 = 0.0;
        var_dvh_rdn6 = 0.0;
        var_dvh_rdn7 = 0.0;
        var_dvh_rdn8 = 0.0;
        var_dvh_rdn9 = 0.0;
        var_dvh_rdb0 = 0.0;
        var_dvh_rdb1 = 0.0;
        var_dvh_rdb2 = 0.0;
        var_dvh_rdb3 = 0.0;
        var_dvh_rdb4 = 0.0;
        var_dvh_rdb5 = 0.0;
        var_dvh_rdb6 = 0.0;
        var_dvh_rdb7 = 0.0;

        let assign1810_e2096: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard18 = assign1810_e2096;
        var_guard18_dn0 = 0.0;
        var_guard18_dn1 = 0.0;
        var_guard18_dn2 = 0.0;
        var_guard18_dn3 = 0.0;
        var_guard18_dn4 = 0.0;
        var_guard18_dn5 = 0.0;
        var_guard18_dn6 = 0.0;
        var_guard18_dn7 = 0.0;
        var_guard18_dn8 = 0.0;
        var_guard18_dn9 = 0.0;
        var_guard18_db0 = 0.0;
        var_guard18_db1 = 0.0;
        var_guard18_db2 = 0.0;
        var_guard18_db3 = 0.0;
        var_guard18_db4 = 0.0;
        var_guard18_db5 = 0.0;
        var_guard18_db6 = 0.0;
        var_guard18_db7 = 0.0;
        var_guard18_rv = 0.0;
        var_guard18_rdn0 = 0.0;
        var_guard18_rdn1 = 0.0;
        var_guard18_rdn2 = 0.0;
        var_guard18_rdn3 = 0.0;
        var_guard18_rdn4 = 0.0;
        var_guard18_rdn5 = 0.0;
        var_guard18_rdn6 = 0.0;
        var_guard18_rdn7 = 0.0;
        var_guard18_rdn8 = 0.0;
        var_guard18_rdn9 = 0.0;
        var_guard18_rdb0 = 0.0;
        var_guard18_rdb1 = 0.0;
        var_guard18_rdb2 = 0.0;
        var_guard18_rdb3 = 0.0;
        var_guard18_rdb4 = 0.0;
        var_guard18_rdb5 = 0.0;
        var_guard18_rdb6 = 0.0;
        var_guard18_rdb7 = 0.0;

        let (assign1820_e2109, assign1820_e2109_d_n0, assign1820_e2109_d_n1, assign1820_e2109_d_n2, assign1820_e2109_d_n3, assign1820_e2109_d_n4, assign1820_e2109_d_n5, assign1820_e2109_d_n6, assign1820_e2109_d_n7, assign1820_e2109_d_n8, assign1820_e2109_d_n9, assign1820_e2109_d_b0, assign1820_e2109_d_b1, assign1820_e2109_d_b2, assign1820_e2109_d_b3, assign1820_e2109_d_b4, assign1820_e2109_d_b5, assign1820_e2109_d_b6, assign1820_e2109_d_b7,) = {
    if (var_guard18 != 0.0) {
        let assign1820_e2099: f64 = (-1.0);
        let assign1820_e2101: f64 = (assign1820_e2099 - p.p71);
        let assign1820_e2104: f64 = (1.0 - p.p24);
        let assign1820_e2105: f64 = (assign1820_e2104).ln();
        let assign1820_e2106: f64 = (assign1820_e2101 * assign1820_e2105);
        let assign1820_e2107: f64 = (assign1820_e2106).exp();
        (assign1820_e2107, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pwq, var_pwq_dn0, var_pwq_dn1, var_pwq_dn2, var_pwq_dn3, var_pwq_dn4, var_pwq_dn5, var_pwq_dn6, var_pwq_dn7, var_pwq_dn8, var_pwq_dn9, var_pwq_db0, var_pwq_db1, var_pwq_db2, var_pwq_db3, var_pwq_db4, var_pwq_db5, var_pwq_db6, var_pwq_db7,)
    }
};
        var_pwq = assign1820_e2109;
        var_pwq_dn0 = assign1820_e2109_d_n0;
        var_pwq_dn1 = assign1820_e2109_d_n1;
        var_pwq_dn2 = assign1820_e2109_d_n2;
        var_pwq_dn3 = assign1820_e2109_d_n3;
        var_pwq_dn4 = assign1820_e2109_d_n4;
        var_pwq_dn5 = assign1820_e2109_d_n5;
        var_pwq_dn6 = assign1820_e2109_d_n6;
        var_pwq_dn7 = assign1820_e2109_d_n7;
        var_pwq_dn8 = assign1820_e2109_d_n8;
        var_pwq_dn9 = assign1820_e2109_d_n9;
        var_pwq_db0 = assign1820_e2109_d_b0;
        var_pwq_db1 = assign1820_e2109_d_b1;
        var_pwq_db2 = assign1820_e2109_d_b2;
        var_pwq_db3 = assign1820_e2109_d_b3;
        var_pwq_db4 = assign1820_e2109_d_b4;
        var_pwq_db5 = assign1820_e2109_d_b5;
        var_pwq_db6 = assign1820_e2109_d_b6;
        var_pwq_db7 = assign1820_e2109_d_b7;
        var_pwq_rv = 0.0;
        var_pwq_rdn0 = 0.0;
        var_pwq_rdn1 = 0.0;
        var_pwq_rdn2 = 0.0;
        var_pwq_rdn3 = 0.0;
        var_pwq_rdn4 = 0.0;
        var_pwq_rdn5 = 0.0;
        var_pwq_rdn6 = 0.0;
        var_pwq_rdn7 = 0.0;
        var_pwq_rdn8 = 0.0;
        var_pwq_rdn9 = 0.0;
        var_pwq_rdb0 = 0.0;
        var_pwq_rdb1 = 0.0;
        var_pwq_rdb2 = 0.0;
        var_pwq_rdb3 = 0.0;
        var_pwq_rdb4 = 0.0;
        var_pwq_rdb5 = 0.0;
        var_pwq_rdb6 = 0.0;
        var_pwq_rdb7 = 0.0;

        let (assign1830_e2129, assign1830_e2129_d_n0, assign1830_e2129_d_n1, assign1830_e2129_d_n2, assign1830_e2129_d_n3, assign1830_e2129_d_n4, assign1830_e2129_d_n5, assign1830_e2129_d_n6, assign1830_e2129_d_n7, assign1830_e2129_d_n8, assign1830_e2129_d_n9, assign1830_e2129_d_b0, assign1830_e2129_d_b1, assign1830_e2129_d_b2, assign1830_e2129_d_b3, assign1830_e2129_d_b4, assign1830_e2129_d_b5, assign1830_e2129_d_b6, assign1830_e2129_d_b7,) = {
    if (var_guard18 != 0.0) {
        let assign1830_e2116: f64 = (1.0 - p.p24);
        let assign1830_e2117: f64 = (var_pwq * assign1830_e2116);
        let assign1830_e2120: f64 = (1.0 - p.p24);
        let assign1830_e2121: f64 = (assign1830_e2117 * assign1830_e2120);
        let assign1830_e2122: f64 = (1.0 - assign1830_e2121);
        let assign1830_e2123: f64 = (var_vjc_t * assign1830_e2122);
        let assign1830_e2126: f64 = (1.0 - p.p71);
        let assign1830_e2127: f64 = (assign1830_e2123 / assign1830_e2126);
        (assign1830_e2127, (((var_vjc_t_dn0 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn0 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn1 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn1 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn2 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn2 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn3 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn3 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn4 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn4 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn5 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn5 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn6 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn6 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn7 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn7 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn8 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn8 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn9 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn9 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db0 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db0 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db1 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db1 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db2 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db2 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db3 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db3 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db4 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db4 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db5 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db5 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db6 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db6 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db7 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db7 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7,)
    }
};
        var_qlo = assign1830_e2129;
        var_qlo_dn0 = assign1830_e2129_d_n0;
        var_qlo_dn1 = assign1830_e2129_d_n1;
        var_qlo_dn2 = assign1830_e2129_d_n2;
        var_qlo_dn3 = assign1830_e2129_d_n3;
        var_qlo_dn4 = assign1830_e2129_d_n4;
        var_qlo_dn5 = assign1830_e2129_d_n5;
        var_qlo_dn6 = assign1830_e2129_d_n6;
        var_qlo_dn7 = assign1830_e2129_d_n7;
        var_qlo_dn8 = assign1830_e2129_d_n8;
        var_qlo_dn9 = assign1830_e2129_d_n9;
        var_qlo_db0 = assign1830_e2129_d_b0;
        var_qlo_db1 = assign1830_e2129_d_b1;
        var_qlo_db2 = assign1830_e2129_d_b2;
        var_qlo_db3 = assign1830_e2129_d_b3;
        var_qlo_db4 = assign1830_e2129_d_b4;
        var_qlo_db5 = assign1830_e2129_d_b5;
        var_qlo_db6 = assign1830_e2129_d_b6;
        var_qlo_db7 = assign1830_e2129_d_b7;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdn7 = 0.0;
        var_qlo_rdn8 = 0.0;
        var_qlo_rdn9 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;
        var_qlo_rdb7 = 0.0;

        let (assign1840_e2147, assign1840_e2147_d_n0, assign1840_e2147_d_n1, assign1840_e2147_d_n2, assign1840_e2147_d_n3, assign1840_e2147_d_n4, assign1840_e2147_d_n5, assign1840_e2147_d_n6, assign1840_e2147_d_n7, assign1840_e2147_d_n8, assign1840_e2147_d_n9, assign1840_e2147_d_b0, assign1840_e2147_d_b1, assign1840_e2147_d_b2, assign1840_e2147_d_b3, assign1840_e2147_d_b4, assign1840_e2147_d_b5, assign1840_e2147_d_b6, assign1840_e2147_d_b7,) = {
    if (var_guard18 != 0.0) {
        let assign1840_e2134: f64 = (1.0 - p.p24);
        let assign1840_e2137: f64 = (0.5 * p.p71);
        let assign1840_e2139: f64 = (assign1840_e2137 * var_dvh);
        let assign1840_e2141: f64 = (assign1840_e2139 / var_vjc_t);
        let assign1840_e2142: f64 = (assign1840_e2134 + assign1840_e2141);
        let assign1840_e2143: f64 = (var_dvh * assign1840_e2142);
        let assign1840_e2145: f64 = (assign1840_e2143 * var_pwq);
        (assign1840_e2145, ((((var_dvh_dn0 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn0) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn0)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn0)), ((((var_dvh_dn1 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn1) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn1)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn1)), ((((var_dvh_dn2 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn2) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn2)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn2)), ((((var_dvh_dn3 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn3) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn3)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn3)), ((((var_dvh_dn4 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn4) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn4)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn4)), ((((var_dvh_dn5 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn5) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn5)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn5)), ((((var_dvh_dn6 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn6) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn6)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn6)), ((((var_dvh_dn7 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn7) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn7)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn7)), ((((var_dvh_dn8 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn8) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn8)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn8)), ((((var_dvh_dn9 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn9) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn9)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn9)), ((((var_dvh_db0 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db0) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db0)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db0)), ((((var_dvh_db1 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db1) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db1)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db1)), ((((var_dvh_db2 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db2) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db2)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db2)), ((((var_dvh_db3 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db3) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db3)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db3)), ((((var_dvh_db4 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db4) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db4)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db4)), ((((var_dvh_db5 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db5) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db5)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db5)), ((((var_dvh_db6 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db6) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db6)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db6)), ((((var_dvh_db7 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db7) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db7)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db7)),)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_dn7, var_qhi_dn8, var_qhi_dn9, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6, var_qhi_db7,)
    }
};
        var_qhi = assign1840_e2147;
        var_qhi_dn0 = assign1840_e2147_d_n0;
        var_qhi_dn1 = assign1840_e2147_d_n1;
        var_qhi_dn2 = assign1840_e2147_d_n2;
        var_qhi_dn3 = assign1840_e2147_d_n3;
        var_qhi_dn4 = assign1840_e2147_d_n4;
        var_qhi_dn5 = assign1840_e2147_d_n5;
        var_qhi_dn6 = assign1840_e2147_d_n6;
        var_qhi_dn7 = assign1840_e2147_d_n7;
        var_qhi_dn8 = assign1840_e2147_d_n8;
        var_qhi_dn9 = assign1840_e2147_d_n9;
        var_qhi_db0 = assign1840_e2147_d_b0;
        var_qhi_db1 = assign1840_e2147_d_b1;
        var_qhi_db2 = assign1840_e2147_d_b2;
        var_qhi_db3 = assign1840_e2147_d_b3;
        var_qhi_db4 = assign1840_e2147_d_b4;
        var_qhi_db5 = assign1840_e2147_d_b5;
        var_qhi_db6 = assign1840_e2147_d_b6;
        var_qhi_db7 = assign1840_e2147_d_b7;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdn7 = 0.0;
        var_qhi_rdn8 = 0.0;
        var_qhi_rdn9 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;
        var_qhi_rdb7 = 0.0;

        let (assign1850_e2170, assign1850_e2170_d_n0, assign1850_e2170_d_n1, assign1850_e2170_d_n2, assign1850_e2170_d_n3, assign1850_e2170_d_n4, assign1850_e2170_d_n5, assign1850_e2170_d_n6, assign1850_e2170_d_n7, assign1850_e2170_d_n8, assign1850_e2170_d_n9, assign1850_e2170_d_b0, assign1850_e2170_d_b1, assign1850_e2170_d_b2, assign1850_e2170_d_b3, assign1850_e2170_d_b4, assign1850_e2170_d_b5, assign1850_e2170_d_b6, assign1850_e2170_d_b7,) = {
    if (var_guard18 == 0.0) {
        let assign1850_e2154: f64 = (1.0 - p.p71);
        let assign1850_e2158: f64 = (var_vbici / var_vjc_t);
        let assign1850_e2159: f64 = (1.0 - assign1850_e2158);
        let assign1850_e2160: f64 = (assign1850_e2159).ln();
        let assign1850_e2161: f64 = (assign1850_e2154 * assign1850_e2160);
        let assign1850_e2162: f64 = (assign1850_e2161).exp();
        let assign1850_e2163: f64 = (1.0 - assign1850_e2162);
        let assign1850_e2164: f64 = (var_vjc_t * assign1850_e2163);
        let assign1850_e2167: f64 = (1.0 - p.p71);
        let assign1850_e2168: f64 = (assign1850_e2164 / assign1850_e2167);
        (assign1850_e2168, (((var_vjc_t_dn0 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn0 * var_vjc_t) - (var_vbici * var_vjc_t_dn0)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn1 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn1 * var_vjc_t) - (var_vbici * var_vjc_t_dn1)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn2 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn2 * var_vjc_t) - (var_vbici * var_vjc_t_dn2)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn3 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn3 * var_vjc_t) - (var_vbici * var_vjc_t_dn3)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn4 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn4 * var_vjc_t) - (var_vbici * var_vjc_t_dn4)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn5 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn5 * var_vjc_t) - (var_vbici * var_vjc_t_dn5)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn6 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn6 * var_vjc_t) - (var_vbici * var_vjc_t_dn6)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn7 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn7 * var_vjc_t) - (var_vbici * var_vjc_t_dn7)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn8 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn8 * var_vjc_t) - (var_vbici * var_vjc_t_dn8)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn9 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn9 * var_vjc_t) - (var_vbici * var_vjc_t_dn9)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db0 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db0 * var_vjc_t) - (var_vbici * var_vjc_t_db0)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db1 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db1 * var_vjc_t) - (var_vbici * var_vjc_t_db1)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db2 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db2 * var_vjc_t) - (var_vbici * var_vjc_t_db2)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db3 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db3 * var_vjc_t) - (var_vbici * var_vjc_t_db3)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db4 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db4 * var_vjc_t) - (var_vbici * var_vjc_t_db4)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db5 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db5 * var_vjc_t) - (var_vbici * var_vjc_t_db5)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db6 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db6 * var_vjc_t) - (var_vbici * var_vjc_t_db6)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db7 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db7 * var_vjc_t) - (var_vbici * var_vjc_t_db7)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7,)
    }
};
        var_qlo = assign1850_e2170;
        var_qlo_dn0 = assign1850_e2170_d_n0;
        var_qlo_dn1 = assign1850_e2170_d_n1;
        var_qlo_dn2 = assign1850_e2170_d_n2;
        var_qlo_dn3 = assign1850_e2170_d_n3;
        var_qlo_dn4 = assign1850_e2170_d_n4;
        var_qlo_dn5 = assign1850_e2170_d_n5;
        var_qlo_dn6 = assign1850_e2170_d_n6;
        var_qlo_dn7 = assign1850_e2170_d_n7;
        var_qlo_dn8 = assign1850_e2170_d_n8;
        var_qlo_dn9 = assign1850_e2170_d_n9;
        var_qlo_db0 = assign1850_e2170_d_b0;
        var_qlo_db1 = assign1850_e2170_d_b1;
        var_qlo_db2 = assign1850_e2170_d_b2;
        var_qlo_db3 = assign1850_e2170_d_b3;
        var_qlo_db4 = assign1850_e2170_d_b4;
        var_qlo_db5 = assign1850_e2170_d_b5;
        var_qlo_db6 = assign1850_e2170_d_b6;
        var_qlo_db7 = assign1850_e2170_d_b7;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdn7 = 0.0;
        var_qlo_rdn8 = 0.0;
        var_qlo_rdn9 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;
        var_qlo_rdb7 = 0.0;

        let (assign1860_e2175, assign1860_e2175_d_n0, assign1860_e2175_d_n1, assign1860_e2175_d_n2, assign1860_e2175_d_n3, assign1860_e2175_d_n4, assign1860_e2175_d_n5, assign1860_e2175_d_n6, assign1860_e2175_d_n7, assign1860_e2175_d_n8, assign1860_e2175_d_n9, assign1860_e2175_d_b0, assign1860_e2175_d_b1, assign1860_e2175_d_b2, assign1860_e2175_d_b3, assign1860_e2175_d_b4, assign1860_e2175_d_b5, assign1860_e2175_d_b6, assign1860_e2175_d_b7,) = {
    if (var_guard18 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_dn7, var_qhi_dn8, var_qhi_dn9, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6, var_qhi_db7,)
    }
};
        var_qhi = assign1860_e2175;
        var_qhi_dn0 = assign1860_e2175_d_n0;
        var_qhi_dn1 = assign1860_e2175_d_n1;
        var_qhi_dn2 = assign1860_e2175_d_n2;
        var_qhi_dn3 = assign1860_e2175_d_n3;
        var_qhi_dn4 = assign1860_e2175_d_n4;
        var_qhi_dn5 = assign1860_e2175_d_n5;
        var_qhi_dn6 = assign1860_e2175_d_n6;
        var_qhi_dn7 = assign1860_e2175_d_n7;
        var_qhi_dn8 = assign1860_e2175_d_n8;
        var_qhi_dn9 = assign1860_e2175_d_n9;
        var_qhi_db0 = assign1860_e2175_d_b0;
        var_qhi_db1 = assign1860_e2175_d_b1;
        var_qhi_db2 = assign1860_e2175_d_b2;
        var_qhi_db3 = assign1860_e2175_d_b3;
        var_qhi_db4 = assign1860_e2175_d_b4;
        var_qhi_db5 = assign1860_e2175_d_b5;
        var_qhi_db6 = assign1860_e2175_d_b6;
        var_qhi_db7 = assign1860_e2175_d_b7;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdn7 = 0.0;
        var_qhi_rdn8 = 0.0;
        var_qhi_rdn9 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;
        var_qhi_rdb7 = 0.0;

        *var_dv0_slot = var_dv0;
        *var_dv0_db0_slot = var_dv0_db0;
        *var_dv0_db1_slot = var_dv0_db1;
        *var_dv0_db2_slot = var_dv0_db2;
        *var_dv0_db3_slot = var_dv0_db3;
        *var_dv0_db4_slot = var_dv0_db4;
        *var_dv0_db5_slot = var_dv0_db5;
        *var_dv0_db6_slot = var_dv0_db6;
        *var_dv0_db7_slot = var_dv0_db7;
        *var_dv0_dn0_slot = var_dv0_dn0;
        *var_dv0_dn1_slot = var_dv0_dn1;
        *var_dv0_dn2_slot = var_dv0_dn2;
        *var_dv0_dn3_slot = var_dv0_dn3;
        *var_dv0_dn4_slot = var_dv0_dn4;
        *var_dv0_dn5_slot = var_dv0_dn5;
        *var_dv0_dn6_slot = var_dv0_dn6;
        *var_dv0_dn7_slot = var_dv0_dn7;
        *var_dv0_dn8_slot = var_dv0_dn8;
        *var_dv0_dn9_slot = var_dv0_dn9;
        *var_dv0_rdb0_slot = var_dv0_rdb0;
        *var_dv0_rdb1_slot = var_dv0_rdb1;
        *var_dv0_rdb2_slot = var_dv0_rdb2;
        *var_dv0_rdb3_slot = var_dv0_rdb3;
        *var_dv0_rdb4_slot = var_dv0_rdb4;
        *var_dv0_rdb5_slot = var_dv0_rdb5;
        *var_dv0_rdb6_slot = var_dv0_rdb6;
        *var_dv0_rdb7_slot = var_dv0_rdb7;
        *var_dv0_rdn0_slot = var_dv0_rdn0;
        *var_dv0_rdn1_slot = var_dv0_rdn1;
        *var_dv0_rdn2_slot = var_dv0_rdn2;
        *var_dv0_rdn3_slot = var_dv0_rdn3;
        *var_dv0_rdn4_slot = var_dv0_rdn4;
        *var_dv0_rdn5_slot = var_dv0_rdn5;
        *var_dv0_rdn6_slot = var_dv0_rdn6;
        *var_dv0_rdn7_slot = var_dv0_rdn7;
        *var_dv0_rdn8_slot = var_dv0_rdn8;
        *var_dv0_rdn9_slot = var_dv0_rdn9;
        *var_dv0_rv_slot = var_dv0_rv;
        *var_dvh_slot = var_dvh;
        *var_dvh_db0_slot = var_dvh_db0;
        *var_dvh_db1_slot = var_dvh_db1;
        *var_dvh_db2_slot = var_dvh_db2;
        *var_dvh_db3_slot = var_dvh_db3;
        *var_dvh_db4_slot = var_dvh_db4;
        *var_dvh_db5_slot = var_dvh_db5;
        *var_dvh_db6_slot = var_dvh_db6;
        *var_dvh_db7_slot = var_dvh_db7;
        *var_dvh_dn0_slot = var_dvh_dn0;
        *var_dvh_dn1_slot = var_dvh_dn1;
        *var_dvh_dn2_slot = var_dvh_dn2;
        *var_dvh_dn3_slot = var_dvh_dn3;
        *var_dvh_dn4_slot = var_dvh_dn4;
        *var_dvh_dn5_slot = var_dvh_dn5;
        *var_dvh_dn6_slot = var_dvh_dn6;
        *var_dvh_dn7_slot = var_dvh_dn7;
        *var_dvh_dn8_slot = var_dvh_dn8;
        *var_dvh_dn9_slot = var_dvh_dn9;
        *var_dvh_rdb0_slot = var_dvh_rdb0;
        *var_dvh_rdb1_slot = var_dvh_rdb1;
        *var_dvh_rdb2_slot = var_dvh_rdb2;
        *var_dvh_rdb3_slot = var_dvh_rdb3;
        *var_dvh_rdb4_slot = var_dvh_rdb4;
        *var_dvh_rdb5_slot = var_dvh_rdb5;
        *var_dvh_rdb6_slot = var_dvh_rdb6;
        *var_dvh_rdb7_slot = var_dvh_rdb7;
        *var_dvh_rdn0_slot = var_dvh_rdn0;
        *var_dvh_rdn1_slot = var_dvh_rdn1;
        *var_dvh_rdn2_slot = var_dvh_rdn2;
        *var_dvh_rdn3_slot = var_dvh_rdn3;
        *var_dvh_rdn4_slot = var_dvh_rdn4;
        *var_dvh_rdn5_slot = var_dvh_rdn5;
        *var_dvh_rdn6_slot = var_dvh_rdn6;
        *var_dvh_rdn7_slot = var_dvh_rdn7;
        *var_dvh_rdn8_slot = var_dvh_rdn8;
        *var_dvh_rdn9_slot = var_dvh_rdn9;
        *var_dvh_rv_slot = var_dvh_rv;
        *var_guard18_slot = var_guard18;
        *var_guard18_db0_slot = var_guard18_db0;
        *var_guard18_db1_slot = var_guard18_db1;
        *var_guard18_db2_slot = var_guard18_db2;
        *var_guard18_db3_slot = var_guard18_db3;
        *var_guard18_db4_slot = var_guard18_db4;
        *var_guard18_db5_slot = var_guard18_db5;
        *var_guard18_db6_slot = var_guard18_db6;
        *var_guard18_db7_slot = var_guard18_db7;
        *var_guard18_dn0_slot = var_guard18_dn0;
        *var_guard18_dn1_slot = var_guard18_dn1;
        *var_guard18_dn2_slot = var_guard18_dn2;
        *var_guard18_dn3_slot = var_guard18_dn3;
        *var_guard18_dn4_slot = var_guard18_dn4;
        *var_guard18_dn5_slot = var_guard18_dn5;
        *var_guard18_dn6_slot = var_guard18_dn6;
        *var_guard18_dn7_slot = var_guard18_dn7;
        *var_guard18_dn8_slot = var_guard18_dn8;
        *var_guard18_dn9_slot = var_guard18_dn9;
        *var_guard18_rdb0_slot = var_guard18_rdb0;
        *var_guard18_rdb1_slot = var_guard18_rdb1;
        *var_guard18_rdb2_slot = var_guard18_rdb2;
        *var_guard18_rdb3_slot = var_guard18_rdb3;
        *var_guard18_rdb4_slot = var_guard18_rdb4;
        *var_guard18_rdb5_slot = var_guard18_rdb5;
        *var_guard18_rdb6_slot = var_guard18_rdb6;
        *var_guard18_rdb7_slot = var_guard18_rdb7;
        *var_guard18_rdn0_slot = var_guard18_rdn0;
        *var_guard18_rdn1_slot = var_guard18_rdn1;
        *var_guard18_rdn2_slot = var_guard18_rdn2;
        *var_guard18_rdn3_slot = var_guard18_rdn3;
        *var_guard18_rdn4_slot = var_guard18_rdn4;
        *var_guard18_rdn5_slot = var_guard18_rdn5;
        *var_guard18_rdn6_slot = var_guard18_rdn6;
        *var_guard18_rdn7_slot = var_guard18_rdn7;
        *var_guard18_rdn8_slot = var_guard18_rdn8;
        *var_guard18_rdn9_slot = var_guard18_rdn9;
        *var_guard18_rv_slot = var_guard18_rv;
        *var_pwq_slot = var_pwq;
        *var_pwq_db0_slot = var_pwq_db0;
        *var_pwq_db1_slot = var_pwq_db1;
        *var_pwq_db2_slot = var_pwq_db2;
        *var_pwq_db3_slot = var_pwq_db3;
        *var_pwq_db4_slot = var_pwq_db4;
        *var_pwq_db5_slot = var_pwq_db5;
        *var_pwq_db6_slot = var_pwq_db6;
        *var_pwq_db7_slot = var_pwq_db7;
        *var_pwq_dn0_slot = var_pwq_dn0;
        *var_pwq_dn1_slot = var_pwq_dn1;
        *var_pwq_dn2_slot = var_pwq_dn2;
        *var_pwq_dn3_slot = var_pwq_dn3;
        *var_pwq_dn4_slot = var_pwq_dn4;
        *var_pwq_dn5_slot = var_pwq_dn5;
        *var_pwq_dn6_slot = var_pwq_dn6;
        *var_pwq_dn7_slot = var_pwq_dn7;
        *var_pwq_dn8_slot = var_pwq_dn8;
        *var_pwq_dn9_slot = var_pwq_dn9;
        *var_pwq_rdb0_slot = var_pwq_rdb0;
        *var_pwq_rdb1_slot = var_pwq_rdb1;
        *var_pwq_rdb2_slot = var_pwq_rdb2;
        *var_pwq_rdb3_slot = var_pwq_rdb3;
        *var_pwq_rdb4_slot = var_pwq_rdb4;
        *var_pwq_rdb5_slot = var_pwq_rdb5;
        *var_pwq_rdb6_slot = var_pwq_rdb6;
        *var_pwq_rdb7_slot = var_pwq_rdb7;
        *var_pwq_rdn0_slot = var_pwq_rdn0;
        *var_pwq_rdn1_slot = var_pwq_rdn1;
        *var_pwq_rdn2_slot = var_pwq_rdn2;
        *var_pwq_rdn3_slot = var_pwq_rdn3;
        *var_pwq_rdn4_slot = var_pwq_rdn4;
        *var_pwq_rdn5_slot = var_pwq_rdn5;
        *var_pwq_rdn6_slot = var_pwq_rdn6;
        *var_pwq_rdn7_slot = var_pwq_rdn7;
        *var_pwq_rdn8_slot = var_pwq_rdn8;
        *var_pwq_rdn9_slot = var_pwq_rdn9;
        *var_pwq_rv_slot = var_pwq_rv;
        *var_qhi_slot = var_qhi;
        *var_qhi_db0_slot = var_qhi_db0;
        *var_qhi_db1_slot = var_qhi_db1;
        *var_qhi_db2_slot = var_qhi_db2;
        *var_qhi_db3_slot = var_qhi_db3;
        *var_qhi_db4_slot = var_qhi_db4;
        *var_qhi_db5_slot = var_qhi_db5;
        *var_qhi_db6_slot = var_qhi_db6;
        *var_qhi_db7_slot = var_qhi_db7;
        *var_qhi_dn0_slot = var_qhi_dn0;
        *var_qhi_dn1_slot = var_qhi_dn1;
        *var_qhi_dn2_slot = var_qhi_dn2;
        *var_qhi_dn3_slot = var_qhi_dn3;
        *var_qhi_dn4_slot = var_qhi_dn4;
        *var_qhi_dn5_slot = var_qhi_dn5;
        *var_qhi_dn6_slot = var_qhi_dn6;
        *var_qhi_dn7_slot = var_qhi_dn7;
        *var_qhi_dn8_slot = var_qhi_dn8;
        *var_qhi_dn9_slot = var_qhi_dn9;
        *var_qhi_rdb0_slot = var_qhi_rdb0;
        *var_qhi_rdb1_slot = var_qhi_rdb1;
        *var_qhi_rdb2_slot = var_qhi_rdb2;
        *var_qhi_rdb3_slot = var_qhi_rdb3;
        *var_qhi_rdb4_slot = var_qhi_rdb4;
        *var_qhi_rdb5_slot = var_qhi_rdb5;
        *var_qhi_rdb6_slot = var_qhi_rdb6;
        *var_qhi_rdb7_slot = var_qhi_rdb7;
        *var_qhi_rdn0_slot = var_qhi_rdn0;
        *var_qhi_rdn1_slot = var_qhi_rdn1;
        *var_qhi_rdn2_slot = var_qhi_rdn2;
        *var_qhi_rdn3_slot = var_qhi_rdn3;
        *var_qhi_rdn4_slot = var_qhi_rdn4;
        *var_qhi_rdn5_slot = var_qhi_rdn5;
        *var_qhi_rdn6_slot = var_qhi_rdn6;
        *var_qhi_rdn7_slot = var_qhi_rdn7;
        *var_qhi_rdn8_slot = var_qhi_rdn8;
        *var_qhi_rdn9_slot = var_qhi_rdn9;
        *var_qhi_rv_slot = var_qhi_rv;
        *var_qjcx_slot = var_qjcx;
        *var_qjcx_1_slot = var_qjcx_1;
        *var_qjcx_1_db0_slot = var_qjcx_1_db0;
        *var_qjcx_1_db1_slot = var_qjcx_1_db1;
        *var_qjcx_1_db2_slot = var_qjcx_1_db2;
        *var_qjcx_1_db3_slot = var_qjcx_1_db3;
        *var_qjcx_1_db4_slot = var_qjcx_1_db4;
        *var_qjcx_1_db5_slot = var_qjcx_1_db5;
        *var_qjcx_1_db6_slot = var_qjcx_1_db6;
        *var_qjcx_1_db7_slot = var_qjcx_1_db7;
        *var_qjcx_1_dn0_slot = var_qjcx_1_dn0;
        *var_qjcx_1_dn1_slot = var_qjcx_1_dn1;
        *var_qjcx_1_dn2_slot = var_qjcx_1_dn2;
        *var_qjcx_1_dn3_slot = var_qjcx_1_dn3;
        *var_qjcx_1_dn4_slot = var_qjcx_1_dn4;
        *var_qjcx_1_dn5_slot = var_qjcx_1_dn5;
        *var_qjcx_1_dn6_slot = var_qjcx_1_dn6;
        *var_qjcx_1_dn7_slot = var_qjcx_1_dn7;
        *var_qjcx_1_dn8_slot = var_qjcx_1_dn8;
        *var_qjcx_1_dn9_slot = var_qjcx_1_dn9;
        *var_qjcx_1_rdb0_slot = var_qjcx_1_rdb0;
        *var_qjcx_1_rdb1_slot = var_qjcx_1_rdb1;
        *var_qjcx_1_rdb2_slot = var_qjcx_1_rdb2;
        *var_qjcx_1_rdb3_slot = var_qjcx_1_rdb3;
        *var_qjcx_1_rdb4_slot = var_qjcx_1_rdb4;
        *var_qjcx_1_rdb5_slot = var_qjcx_1_rdb5;
        *var_qjcx_1_rdb6_slot = var_qjcx_1_rdb6;
        *var_qjcx_1_rdb7_slot = var_qjcx_1_rdb7;
        *var_qjcx_1_rdn0_slot = var_qjcx_1_rdn0;
        *var_qjcx_1_rdn1_slot = var_qjcx_1_rdn1;
        *var_qjcx_1_rdn2_slot = var_qjcx_1_rdn2;
        *var_qjcx_1_rdn3_slot = var_qjcx_1_rdn3;
        *var_qjcx_1_rdn4_slot = var_qjcx_1_rdn4;
        *var_qjcx_1_rdn5_slot = var_qjcx_1_rdn5;
        *var_qjcx_1_rdn6_slot = var_qjcx_1_rdn6;
        *var_qjcx_1_rdn7_slot = var_qjcx_1_rdn7;
        *var_qjcx_1_rdn8_slot = var_qjcx_1_rdn8;
        *var_qjcx_1_rdn9_slot = var_qjcx_1_rdn9;
        *var_qjcx_1_rv_slot = var_qjcx_1_rv;
        *var_qjcx_db0_slot = var_qjcx_db0;
        *var_qjcx_db1_slot = var_qjcx_db1;
        *var_qjcx_db2_slot = var_qjcx_db2;
        *var_qjcx_db3_slot = var_qjcx_db3;
        *var_qjcx_db4_slot = var_qjcx_db4;
        *var_qjcx_db5_slot = var_qjcx_db5;
        *var_qjcx_db6_slot = var_qjcx_db6;
        *var_qjcx_db7_slot = var_qjcx_db7;
        *var_qjcx_dn0_slot = var_qjcx_dn0;
        *var_qjcx_dn1_slot = var_qjcx_dn1;
        *var_qjcx_dn2_slot = var_qjcx_dn2;
        *var_qjcx_dn3_slot = var_qjcx_dn3;
        *var_qjcx_dn4_slot = var_qjcx_dn4;
        *var_qjcx_dn5_slot = var_qjcx_dn5;
        *var_qjcx_dn6_slot = var_qjcx_dn6;
        *var_qjcx_dn7_slot = var_qjcx_dn7;
        *var_qjcx_dn8_slot = var_qjcx_dn8;
        *var_qjcx_dn9_slot = var_qjcx_dn9;
        *var_qjcx_rdb0_slot = var_qjcx_rdb0;
        *var_qjcx_rdb1_slot = var_qjcx_rdb1;
        *var_qjcx_rdb2_slot = var_qjcx_rdb2;
        *var_qjcx_rdb3_slot = var_qjcx_rdb3;
        *var_qjcx_rdb4_slot = var_qjcx_rdb4;
        *var_qjcx_rdb5_slot = var_qjcx_rdb5;
        *var_qjcx_rdb6_slot = var_qjcx_rdb6;
        *var_qjcx_rdb7_slot = var_qjcx_rdb7;
        *var_qjcx_rdn0_slot = var_qjcx_rdn0;
        *var_qjcx_rdn1_slot = var_qjcx_rdn1;
        *var_qjcx_rdn2_slot = var_qjcx_rdn2;
        *var_qjcx_rdn3_slot = var_qjcx_rdn3;
        *var_qjcx_rdn4_slot = var_qjcx_rdn4;
        *var_qjcx_rdn5_slot = var_qjcx_rdn5;
        *var_qjcx_rdn6_slot = var_qjcx_rdn6;
        *var_qjcx_rdn7_slot = var_qjcx_rdn7;
        *var_qjcx_rdn8_slot = var_qjcx_rdn8;
        *var_qjcx_rdn9_slot = var_qjcx_rdn9;
        *var_qjcx_rv_slot = var_qjcx_rv;
        *var_qlo_slot = var_qlo;
        *var_qlo_db0_slot = var_qlo_db0;
        *var_qlo_db1_slot = var_qlo_db1;
        *var_qlo_db2_slot = var_qlo_db2;
        *var_qlo_db3_slot = var_qlo_db3;
        *var_qlo_db4_slot = var_qlo_db4;
        *var_qlo_db5_slot = var_qlo_db5;
        *var_qlo_db6_slot = var_qlo_db6;
        *var_qlo_db7_slot = var_qlo_db7;
        *var_qlo_dn0_slot = var_qlo_dn0;
        *var_qlo_dn1_slot = var_qlo_dn1;
        *var_qlo_dn2_slot = var_qlo_dn2;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_qlo_dn5_slot = var_qlo_dn5;
        *var_qlo_dn6_slot = var_qlo_dn6;
        *var_qlo_dn7_slot = var_qlo_dn7;
        *var_qlo_dn8_slot = var_qlo_dn8;
        *var_qlo_dn9_slot = var_qlo_dn9;
        *var_qlo_rdb0_slot = var_qlo_rdb0;
        *var_qlo_rdb1_slot = var_qlo_rdb1;
        *var_qlo_rdb2_slot = var_qlo_rdb2;
        *var_qlo_rdb3_slot = var_qlo_rdb3;
        *var_qlo_rdb4_slot = var_qlo_rdb4;
        *var_qlo_rdb5_slot = var_qlo_rdb5;
        *var_qlo_rdb6_slot = var_qlo_rdb6;
        *var_qlo_rdb7_slot = var_qlo_rdb7;
        *var_qlo_rdn0_slot = var_qlo_rdn0;
        *var_qlo_rdn1_slot = var_qlo_rdn1;
        *var_qlo_rdn2_slot = var_qlo_rdn2;
        *var_qlo_rdn3_slot = var_qlo_rdn3;
        *var_qlo_rdn4_slot = var_qlo_rdn4;
        *var_qlo_rdn5_slot = var_qlo_rdn5;
        *var_qlo_rdn6_slot = var_qlo_rdn6;
        *var_qlo_rdn7_slot = var_qlo_rdn7;
        *var_qlo_rdn8_slot = var_qlo_rdn8;
        *var_qlo_rdn9_slot = var_qlo_rdn9;
        *var_qlo_rv_slot = var_qlo_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        var_cjc_t: f64,
        var_cjc_t_db0: f64,
        var_cjc_t_db1: f64,
        var_cjc_t_db2: f64,
        var_cjc_t_db3: f64,
        var_cjc_t_db4: f64,
        var_cjc_t_db5: f64,
        var_cjc_t_db6: f64,
        var_cjc_t_db7: f64,
        var_cjc_t_dn0: f64,
        var_cjc_t_dn1: f64,
        var_cjc_t_dn2: f64,
        var_cjc_t_dn3: f64,
        var_cjc_t_dn4: f64,
        var_cjc_t_dn5: f64,
        var_cjc_t_dn6: f64,
        var_cjc_t_dn7: f64,
        var_cjc_t_dn8: f64,
        var_cjc_t_dn9: f64,
        var_itzf: f64,
        var_itzf_db0: f64,
        var_itzf_db1: f64,
        var_itzf_db2: f64,
        var_itzf_db3: f64,
        var_itzf_db4: f64,
        var_itzf_db5: f64,
        var_itzf_db6: f64,
        var_itzf_db7: f64,
        var_itzf_dn0: f64,
        var_itzf_dn1: f64,
        var_itzf_dn2: f64,
        var_itzf_dn3: f64,
        var_itzf_dn4: f64,
        var_itzf_dn5: f64,
        var_itzf_dn6: f64,
        var_itzf_dn7: f64,
        var_itzf_dn8: f64,
        var_itzf_dn9: f64,
        var_qhi: f64,
        var_qhi_db0: f64,
        var_qhi_db1: f64,
        var_qhi_db2: f64,
        var_qhi_db3: f64,
        var_qhi_db4: f64,
        var_qhi_db5: f64,
        var_qhi_db6: f64,
        var_qhi_db7: f64,
        var_qhi_dn0: f64,
        var_qhi_dn1: f64,
        var_qhi_dn2: f64,
        var_qhi_dn3: f64,
        var_qhi_dn4: f64,
        var_qhi_dn5: f64,
        var_qhi_dn6: f64,
        var_qhi_dn7: f64,
        var_qhi_dn8: f64,
        var_qhi_dn9: f64,
        var_qlo: f64,
        var_qlo_db0: f64,
        var_qlo_db1: f64,
        var_qlo_db2: f64,
        var_qlo_db3: f64,
        var_qlo_db4: f64,
        var_qlo_db5: f64,
        var_qlo_db6: f64,
        var_qlo_db7: f64,
        var_qlo_dn0: f64,
        var_qlo_dn1: f64,
        var_qlo_dn2: f64,
        var_qlo_dn3: f64,
        var_qlo_dn4: f64,
        var_qlo_dn5: f64,
        var_qlo_dn6: f64,
        var_qlo_dn7: f64,
        var_qlo_dn8: f64,
        var_qlo_dn9: f64,
        var_ttype: f64,
        var_ttype_db0: f64,
        var_ttype_db1: f64,
        var_ttype_db2: f64,
        var_ttype_db3: f64,
        var_ttype_db4: f64,
        var_ttype_db5: f64,
        var_ttype_db6: f64,
        var_ttype_db7: f64,
        var_ttype_dn0: f64,
        var_ttype_dn1: f64,
        var_ttype_dn2: f64,
        var_ttype_dn3: f64,
        var_ttype_dn4: f64,
        var_ttype_dn5: f64,
        var_ttype_dn6: f64,
        var_ttype_dn7: f64,
        var_ttype_dn8: f64,
        var_ttype_dn9: f64,
        var_guard19_slot: &mut f64,
        var_guard19_db0_slot: &mut f64,
        var_guard19_db1_slot: &mut f64,
        var_guard19_db2_slot: &mut f64,
        var_guard19_db3_slot: &mut f64,
        var_guard19_db4_slot: &mut f64,
        var_guard19_db5_slot: &mut f64,
        var_guard19_db6_slot: &mut f64,
        var_guard19_db7_slot: &mut f64,
        var_guard19_dn0_slot: &mut f64,
        var_guard19_dn1_slot: &mut f64,
        var_guard19_dn2_slot: &mut f64,
        var_guard19_dn3_slot: &mut f64,
        var_guard19_dn4_slot: &mut f64,
        var_guard19_dn5_slot: &mut f64,
        var_guard19_dn6_slot: &mut f64,
        var_guard19_dn7_slot: &mut f64,
        var_guard19_dn8_slot: &mut f64,
        var_guard19_dn9_slot: &mut f64,
        var_guard19_rdb0_slot: &mut f64,
        var_guard19_rdb1_slot: &mut f64,
        var_guard19_rdb2_slot: &mut f64,
        var_guard19_rdb3_slot: &mut f64,
        var_guard19_rdb4_slot: &mut f64,
        var_guard19_rdb5_slot: &mut f64,
        var_guard19_rdb6_slot: &mut f64,
        var_guard19_rdb7_slot: &mut f64,
        var_guard19_rdn0_slot: &mut f64,
        var_guard19_rdn1_slot: &mut f64,
        var_guard19_rdn2_slot: &mut f64,
        var_guard19_rdn3_slot: &mut f64,
        var_guard19_rdn4_slot: &mut f64,
        var_guard19_rdn5_slot: &mut f64,
        var_guard19_rdn6_slot: &mut f64,
        var_guard19_rdn7_slot: &mut f64,
        var_guard19_rdn8_slot: &mut f64,
        var_guard19_rdn9_slot: &mut f64,
        var_guard19_rv_slot: &mut f64,
        var_guard20_slot: &mut f64,
        var_guard20_db0_slot: &mut f64,
        var_guard20_db1_slot: &mut f64,
        var_guard20_db2_slot: &mut f64,
        var_guard20_db3_slot: &mut f64,
        var_guard20_db4_slot: &mut f64,
        var_guard20_db5_slot: &mut f64,
        var_guard20_db6_slot: &mut f64,
        var_guard20_db7_slot: &mut f64,
        var_guard20_dn0_slot: &mut f64,
        var_guard20_dn1_slot: &mut f64,
        var_guard20_dn2_slot: &mut f64,
        var_guard20_dn3_slot: &mut f64,
        var_guard20_dn4_slot: &mut f64,
        var_guard20_dn5_slot: &mut f64,
        var_guard20_dn6_slot: &mut f64,
        var_guard20_dn7_slot: &mut f64,
        var_guard20_dn8_slot: &mut f64,
        var_guard20_dn9_slot: &mut f64,
        var_guard20_rdb0_slot: &mut f64,
        var_guard20_rdb1_slot: &mut f64,
        var_guard20_rdb2_slot: &mut f64,
        var_guard20_rdb3_slot: &mut f64,
        var_guard20_rdb4_slot: &mut f64,
        var_guard20_rdb5_slot: &mut f64,
        var_guard20_rdb6_slot: &mut f64,
        var_guard20_rdb7_slot: &mut f64,
        var_guard20_rdn0_slot: &mut f64,
        var_guard20_rdn1_slot: &mut f64,
        var_guard20_rdn2_slot: &mut f64,
        var_guard20_rdn3_slot: &mut f64,
        var_guard20_rdn4_slot: &mut f64,
        var_guard20_rdn5_slot: &mut f64,
        var_guard20_rdn6_slot: &mut f64,
        var_guard20_rdn7_slot: &mut f64,
        var_guard20_rdn8_slot: &mut f64,
        var_guard20_rdn9_slot: &mut f64,
        var_guard20_rv_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard21_db0_slot: &mut f64,
        var_guard21_db1_slot: &mut f64,
        var_guard21_db2_slot: &mut f64,
        var_guard21_db3_slot: &mut f64,
        var_guard21_db4_slot: &mut f64,
        var_guard21_db5_slot: &mut f64,
        var_guard21_db6_slot: &mut f64,
        var_guard21_db7_slot: &mut f64,
        var_guard21_dn0_slot: &mut f64,
        var_guard21_dn1_slot: &mut f64,
        var_guard21_dn2_slot: &mut f64,
        var_guard21_dn3_slot: &mut f64,
        var_guard21_dn4_slot: &mut f64,
        var_guard21_dn5_slot: &mut f64,
        var_guard21_dn6_slot: &mut f64,
        var_guard21_dn7_slot: &mut f64,
        var_guard21_dn8_slot: &mut f64,
        var_guard21_dn9_slot: &mut f64,
        var_guard21_rdb0_slot: &mut f64,
        var_guard21_rdb1_slot: &mut f64,
        var_guard21_rdb2_slot: &mut f64,
        var_guard21_rdb3_slot: &mut f64,
        var_guard21_rdb4_slot: &mut f64,
        var_guard21_rdb5_slot: &mut f64,
        var_guard21_rdb6_slot: &mut f64,
        var_guard21_rdb7_slot: &mut f64,
        var_guard21_rdn0_slot: &mut f64,
        var_guard21_rdn1_slot: &mut f64,
        var_guard21_rdn2_slot: &mut f64,
        var_guard21_rdn3_slot: &mut f64,
        var_guard21_rdn4_slot: &mut f64,
        var_guard21_rdn5_slot: &mut f64,
        var_guard21_rdn6_slot: &mut f64,
        var_guard21_rdn7_slot: &mut f64,
        var_guard21_rdn8_slot: &mut f64,
        var_guard21_rdn9_slot: &mut f64,
        var_guard21_rv_slot: &mut f64,
        var_qjci_slot: &mut f64,
        var_qjci_1_slot: &mut f64,
        var_qjci_1_db0_slot: &mut f64,
        var_qjci_1_db1_slot: &mut f64,
        var_qjci_1_db2_slot: &mut f64,
        var_qjci_1_db3_slot: &mut f64,
        var_qjci_1_db4_slot: &mut f64,
        var_qjci_1_db5_slot: &mut f64,
        var_qjci_1_db6_slot: &mut f64,
        var_qjci_1_db7_slot: &mut f64,
        var_qjci_1_dn0_slot: &mut f64,
        var_qjci_1_dn1_slot: &mut f64,
        var_qjci_1_dn2_slot: &mut f64,
        var_qjci_1_dn3_slot: &mut f64,
        var_qjci_1_dn4_slot: &mut f64,
        var_qjci_1_dn5_slot: &mut f64,
        var_qjci_1_dn6_slot: &mut f64,
        var_qjci_1_dn7_slot: &mut f64,
        var_qjci_1_dn8_slot: &mut f64,
        var_qjci_1_dn9_slot: &mut f64,
        var_qjci_1_rdb0_slot: &mut f64,
        var_qjci_1_rdb1_slot: &mut f64,
        var_qjci_1_rdb2_slot: &mut f64,
        var_qjci_1_rdb3_slot: &mut f64,
        var_qjci_1_rdb4_slot: &mut f64,
        var_qjci_1_rdb5_slot: &mut f64,
        var_qjci_1_rdb6_slot: &mut f64,
        var_qjci_1_rdb7_slot: &mut f64,
        var_qjci_1_rdn0_slot: &mut f64,
        var_qjci_1_rdn1_slot: &mut f64,
        var_qjci_1_rdn2_slot: &mut f64,
        var_qjci_1_rdn3_slot: &mut f64,
        var_qjci_1_rdn4_slot: &mut f64,
        var_qjci_1_rdn5_slot: &mut f64,
        var_qjci_1_rdn6_slot: &mut f64,
        var_qjci_1_rdn7_slot: &mut f64,
        var_qjci_1_rdn8_slot: &mut f64,
        var_qjci_1_rdn9_slot: &mut f64,
        var_qjci_1_rv_slot: &mut f64,
        var_qjci_db0_slot: &mut f64,
        var_qjci_db1_slot: &mut f64,
        var_qjci_db2_slot: &mut f64,
        var_qjci_db3_slot: &mut f64,
        var_qjci_db4_slot: &mut f64,
        var_qjci_db5_slot: &mut f64,
        var_qjci_db6_slot: &mut f64,
        var_qjci_db7_slot: &mut f64,
        var_qjci_dn0_slot: &mut f64,
        var_qjci_dn1_slot: &mut f64,
        var_qjci_dn2_slot: &mut f64,
        var_qjci_dn3_slot: &mut f64,
        var_qjci_dn4_slot: &mut f64,
        var_qjci_dn5_slot: &mut f64,
        var_qjci_dn6_slot: &mut f64,
        var_qjci_dn7_slot: &mut f64,
        var_qjci_dn8_slot: &mut f64,
        var_qjci_dn9_slot: &mut f64,
        var_qjci_rdb0_slot: &mut f64,
        var_qjci_rdb1_slot: &mut f64,
        var_qjci_rdb2_slot: &mut f64,
        var_qjci_rdb3_slot: &mut f64,
        var_qjci_rdb4_slot: &mut f64,
        var_qjci_rdb5_slot: &mut f64,
        var_qjci_rdb6_slot: &mut f64,
        var_qjci_rdb7_slot: &mut f64,
        var_qjci_rdn0_slot: &mut f64,
        var_qjci_rdn1_slot: &mut f64,
        var_qjci_rdn2_slot: &mut f64,
        var_qjci_rdn3_slot: &mut f64,
        var_qjci_rdn4_slot: &mut f64,
        var_qjci_rdn5_slot: &mut f64,
        var_qjci_rdn6_slot: &mut f64,
        var_qjci_rdn7_slot: &mut f64,
        var_qjci_rdn8_slot: &mut f64,
        var_qjci_rdn9_slot: &mut f64,
        var_qjci_rv_slot: &mut f64,
        var_qxf1_slot: &mut f64,
        var_qxf1_db0_slot: &mut f64,
        var_qxf1_db1_slot: &mut f64,
        var_qxf1_db2_slot: &mut f64,
        var_qxf1_db3_slot: &mut f64,
        var_qxf1_db4_slot: &mut f64,
        var_qxf1_db5_slot: &mut f64,
        var_qxf1_db6_slot: &mut f64,
        var_qxf1_db7_slot: &mut f64,
        var_qxf1_dn0_slot: &mut f64,
        var_qxf1_dn1_slot: &mut f64,
        var_qxf1_dn2_slot: &mut f64,
        var_qxf1_dn3_slot: &mut f64,
        var_qxf1_dn4_slot: &mut f64,
        var_qxf1_dn5_slot: &mut f64,
        var_qxf1_dn6_slot: &mut f64,
        var_qxf1_dn7_slot: &mut f64,
        var_qxf1_dn8_slot: &mut f64,
        var_qxf1_dn9_slot: &mut f64,
        var_qxf1_rdb0_slot: &mut f64,
        var_qxf1_rdb1_slot: &mut f64,
        var_qxf1_rdb2_slot: &mut f64,
        var_qxf1_rdb3_slot: &mut f64,
        var_qxf1_rdb4_slot: &mut f64,
        var_qxf1_rdb5_slot: &mut f64,
        var_qxf1_rdb6_slot: &mut f64,
        var_qxf1_rdb7_slot: &mut f64,
        var_qxf1_rdn0_slot: &mut f64,
        var_qxf1_rdn1_slot: &mut f64,
        var_qxf1_rdn2_slot: &mut f64,
        var_qxf1_rdn3_slot: &mut f64,
        var_qxf1_rdn4_slot: &mut f64,
        var_qxf1_rdn5_slot: &mut f64,
        var_qxf1_rdn6_slot: &mut f64,
        var_qxf1_rdn7_slot: &mut f64,
        var_qxf1_rdn8_slot: &mut f64,
        var_qxf1_rdn9_slot: &mut f64,
        var_qxf1_rv_slot: &mut f64,
    ) {
        let mut var_guard19: f64 = *var_guard19_slot;
        let mut var_guard19_db0: f64 = *var_guard19_db0_slot;
        let mut var_guard19_db1: f64 = *var_guard19_db1_slot;
        let mut var_guard19_db2: f64 = *var_guard19_db2_slot;
        let mut var_guard19_db3: f64 = *var_guard19_db3_slot;
        let mut var_guard19_db4: f64 = *var_guard19_db4_slot;
        let mut var_guard19_db5: f64 = *var_guard19_db5_slot;
        let mut var_guard19_db6: f64 = *var_guard19_db6_slot;
        let mut var_guard19_db7: f64 = *var_guard19_db7_slot;
        let mut var_guard19_dn0: f64 = *var_guard19_dn0_slot;
        let mut var_guard19_dn1: f64 = *var_guard19_dn1_slot;
        let mut var_guard19_dn2: f64 = *var_guard19_dn2_slot;
        let mut var_guard19_dn3: f64 = *var_guard19_dn3_slot;
        let mut var_guard19_dn4: f64 = *var_guard19_dn4_slot;
        let mut var_guard19_dn5: f64 = *var_guard19_dn5_slot;
        let mut var_guard19_dn6: f64 = *var_guard19_dn6_slot;
        let mut var_guard19_dn7: f64 = *var_guard19_dn7_slot;
        let mut var_guard19_dn8: f64 = *var_guard19_dn8_slot;
        let mut var_guard19_dn9: f64 = *var_guard19_dn9_slot;
        let mut var_guard19_rdb0: f64 = *var_guard19_rdb0_slot;
        let mut var_guard19_rdb1: f64 = *var_guard19_rdb1_slot;
        let mut var_guard19_rdb2: f64 = *var_guard19_rdb2_slot;
        let mut var_guard19_rdb3: f64 = *var_guard19_rdb3_slot;
        let mut var_guard19_rdb4: f64 = *var_guard19_rdb4_slot;
        let mut var_guard19_rdb5: f64 = *var_guard19_rdb5_slot;
        let mut var_guard19_rdb6: f64 = *var_guard19_rdb6_slot;
        let mut var_guard19_rdb7: f64 = *var_guard19_rdb7_slot;
        let mut var_guard19_rdn0: f64 = *var_guard19_rdn0_slot;
        let mut var_guard19_rdn1: f64 = *var_guard19_rdn1_slot;
        let mut var_guard19_rdn2: f64 = *var_guard19_rdn2_slot;
        let mut var_guard19_rdn3: f64 = *var_guard19_rdn3_slot;
        let mut var_guard19_rdn4: f64 = *var_guard19_rdn4_slot;
        let mut var_guard19_rdn5: f64 = *var_guard19_rdn5_slot;
        let mut var_guard19_rdn6: f64 = *var_guard19_rdn6_slot;
        let mut var_guard19_rdn7: f64 = *var_guard19_rdn7_slot;
        let mut var_guard19_rdn8: f64 = *var_guard19_rdn8_slot;
        let mut var_guard19_rdn9: f64 = *var_guard19_rdn9_slot;
        let mut var_guard19_rv: f64 = *var_guard19_rv_slot;
        let mut var_guard20: f64 = *var_guard20_slot;
        let mut var_guard20_db0: f64 = *var_guard20_db0_slot;
        let mut var_guard20_db1: f64 = *var_guard20_db1_slot;
        let mut var_guard20_db2: f64 = *var_guard20_db2_slot;
        let mut var_guard20_db3: f64 = *var_guard20_db3_slot;
        let mut var_guard20_db4: f64 = *var_guard20_db4_slot;
        let mut var_guard20_db5: f64 = *var_guard20_db5_slot;
        let mut var_guard20_db6: f64 = *var_guard20_db6_slot;
        let mut var_guard20_db7: f64 = *var_guard20_db7_slot;
        let mut var_guard20_dn0: f64 = *var_guard20_dn0_slot;
        let mut var_guard20_dn1: f64 = *var_guard20_dn1_slot;
        let mut var_guard20_dn2: f64 = *var_guard20_dn2_slot;
        let mut var_guard20_dn3: f64 = *var_guard20_dn3_slot;
        let mut var_guard20_dn4: f64 = *var_guard20_dn4_slot;
        let mut var_guard20_dn5: f64 = *var_guard20_dn5_slot;
        let mut var_guard20_dn6: f64 = *var_guard20_dn6_slot;
        let mut var_guard20_dn7: f64 = *var_guard20_dn7_slot;
        let mut var_guard20_dn8: f64 = *var_guard20_dn8_slot;
        let mut var_guard20_dn9: f64 = *var_guard20_dn9_slot;
        let mut var_guard20_rdb0: f64 = *var_guard20_rdb0_slot;
        let mut var_guard20_rdb1: f64 = *var_guard20_rdb1_slot;
        let mut var_guard20_rdb2: f64 = *var_guard20_rdb2_slot;
        let mut var_guard20_rdb3: f64 = *var_guard20_rdb3_slot;
        let mut var_guard20_rdb4: f64 = *var_guard20_rdb4_slot;
        let mut var_guard20_rdb5: f64 = *var_guard20_rdb5_slot;
        let mut var_guard20_rdb6: f64 = *var_guard20_rdb6_slot;
        let mut var_guard20_rdb7: f64 = *var_guard20_rdb7_slot;
        let mut var_guard20_rdn0: f64 = *var_guard20_rdn0_slot;
        let mut var_guard20_rdn1: f64 = *var_guard20_rdn1_slot;
        let mut var_guard20_rdn2: f64 = *var_guard20_rdn2_slot;
        let mut var_guard20_rdn3: f64 = *var_guard20_rdn3_slot;
        let mut var_guard20_rdn4: f64 = *var_guard20_rdn4_slot;
        let mut var_guard20_rdn5: f64 = *var_guard20_rdn5_slot;
        let mut var_guard20_rdn6: f64 = *var_guard20_rdn6_slot;
        let mut var_guard20_rdn7: f64 = *var_guard20_rdn7_slot;
        let mut var_guard20_rdn8: f64 = *var_guard20_rdn8_slot;
        let mut var_guard20_rdn9: f64 = *var_guard20_rdn9_slot;
        let mut var_guard20_rv: f64 = *var_guard20_rv_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard21_db0: f64 = *var_guard21_db0_slot;
        let mut var_guard21_db1: f64 = *var_guard21_db1_slot;
        let mut var_guard21_db2: f64 = *var_guard21_db2_slot;
        let mut var_guard21_db3: f64 = *var_guard21_db3_slot;
        let mut var_guard21_db4: f64 = *var_guard21_db4_slot;
        let mut var_guard21_db5: f64 = *var_guard21_db5_slot;
        let mut var_guard21_db6: f64 = *var_guard21_db6_slot;
        let mut var_guard21_db7: f64 = *var_guard21_db7_slot;
        let mut var_guard21_dn0: f64 = *var_guard21_dn0_slot;
        let mut var_guard21_dn1: f64 = *var_guard21_dn1_slot;
        let mut var_guard21_dn2: f64 = *var_guard21_dn2_slot;
        let mut var_guard21_dn3: f64 = *var_guard21_dn3_slot;
        let mut var_guard21_dn4: f64 = *var_guard21_dn4_slot;
        let mut var_guard21_dn5: f64 = *var_guard21_dn5_slot;
        let mut var_guard21_dn6: f64 = *var_guard21_dn6_slot;
        let mut var_guard21_dn7: f64 = *var_guard21_dn7_slot;
        let mut var_guard21_dn8: f64 = *var_guard21_dn8_slot;
        let mut var_guard21_dn9: f64 = *var_guard21_dn9_slot;
        let mut var_guard21_rdb0: f64 = *var_guard21_rdb0_slot;
        let mut var_guard21_rdb1: f64 = *var_guard21_rdb1_slot;
        let mut var_guard21_rdb2: f64 = *var_guard21_rdb2_slot;
        let mut var_guard21_rdb3: f64 = *var_guard21_rdb3_slot;
        let mut var_guard21_rdb4: f64 = *var_guard21_rdb4_slot;
        let mut var_guard21_rdb5: f64 = *var_guard21_rdb5_slot;
        let mut var_guard21_rdb6: f64 = *var_guard21_rdb6_slot;
        let mut var_guard21_rdb7: f64 = *var_guard21_rdb7_slot;
        let mut var_guard21_rdn0: f64 = *var_guard21_rdn0_slot;
        let mut var_guard21_rdn1: f64 = *var_guard21_rdn1_slot;
        let mut var_guard21_rdn2: f64 = *var_guard21_rdn2_slot;
        let mut var_guard21_rdn3: f64 = *var_guard21_rdn3_slot;
        let mut var_guard21_rdn4: f64 = *var_guard21_rdn4_slot;
        let mut var_guard21_rdn5: f64 = *var_guard21_rdn5_slot;
        let mut var_guard21_rdn6: f64 = *var_guard21_rdn6_slot;
        let mut var_guard21_rdn7: f64 = *var_guard21_rdn7_slot;
        let mut var_guard21_rdn8: f64 = *var_guard21_rdn8_slot;
        let mut var_guard21_rdn9: f64 = *var_guard21_rdn9_slot;
        let mut var_guard21_rv: f64 = *var_guard21_rv_slot;
        let mut var_qjci: f64 = *var_qjci_slot;
        let mut var_qjci_1: f64 = *var_qjci_1_slot;
        let mut var_qjci_1_db0: f64 = *var_qjci_1_db0_slot;
        let mut var_qjci_1_db1: f64 = *var_qjci_1_db1_slot;
        let mut var_qjci_1_db2: f64 = *var_qjci_1_db2_slot;
        let mut var_qjci_1_db3: f64 = *var_qjci_1_db3_slot;
        let mut var_qjci_1_db4: f64 = *var_qjci_1_db4_slot;
        let mut var_qjci_1_db5: f64 = *var_qjci_1_db5_slot;
        let mut var_qjci_1_db6: f64 = *var_qjci_1_db6_slot;
        let mut var_qjci_1_db7: f64 = *var_qjci_1_db7_slot;
        let mut var_qjci_1_dn0: f64 = *var_qjci_1_dn0_slot;
        let mut var_qjci_1_dn1: f64 = *var_qjci_1_dn1_slot;
        let mut var_qjci_1_dn2: f64 = *var_qjci_1_dn2_slot;
        let mut var_qjci_1_dn3: f64 = *var_qjci_1_dn3_slot;
        let mut var_qjci_1_dn4: f64 = *var_qjci_1_dn4_slot;
        let mut var_qjci_1_dn5: f64 = *var_qjci_1_dn5_slot;
        let mut var_qjci_1_dn6: f64 = *var_qjci_1_dn6_slot;
        let mut var_qjci_1_dn7: f64 = *var_qjci_1_dn7_slot;
        let mut var_qjci_1_dn8: f64 = *var_qjci_1_dn8_slot;
        let mut var_qjci_1_dn9: f64 = *var_qjci_1_dn9_slot;
        let mut var_qjci_1_rdb0: f64 = *var_qjci_1_rdb0_slot;
        let mut var_qjci_1_rdb1: f64 = *var_qjci_1_rdb1_slot;
        let mut var_qjci_1_rdb2: f64 = *var_qjci_1_rdb2_slot;
        let mut var_qjci_1_rdb3: f64 = *var_qjci_1_rdb3_slot;
        let mut var_qjci_1_rdb4: f64 = *var_qjci_1_rdb4_slot;
        let mut var_qjci_1_rdb5: f64 = *var_qjci_1_rdb5_slot;
        let mut var_qjci_1_rdb6: f64 = *var_qjci_1_rdb6_slot;
        let mut var_qjci_1_rdb7: f64 = *var_qjci_1_rdb7_slot;
        let mut var_qjci_1_rdn0: f64 = *var_qjci_1_rdn0_slot;
        let mut var_qjci_1_rdn1: f64 = *var_qjci_1_rdn1_slot;
        let mut var_qjci_1_rdn2: f64 = *var_qjci_1_rdn2_slot;
        let mut var_qjci_1_rdn3: f64 = *var_qjci_1_rdn3_slot;
        let mut var_qjci_1_rdn4: f64 = *var_qjci_1_rdn4_slot;
        let mut var_qjci_1_rdn5: f64 = *var_qjci_1_rdn5_slot;
        let mut var_qjci_1_rdn6: f64 = *var_qjci_1_rdn6_slot;
        let mut var_qjci_1_rdn7: f64 = *var_qjci_1_rdn7_slot;
        let mut var_qjci_1_rdn8: f64 = *var_qjci_1_rdn8_slot;
        let mut var_qjci_1_rdn9: f64 = *var_qjci_1_rdn9_slot;
        let mut var_qjci_1_rv: f64 = *var_qjci_1_rv_slot;
        let mut var_qjci_db0: f64 = *var_qjci_db0_slot;
        let mut var_qjci_db1: f64 = *var_qjci_db1_slot;
        let mut var_qjci_db2: f64 = *var_qjci_db2_slot;
        let mut var_qjci_db3: f64 = *var_qjci_db3_slot;
        let mut var_qjci_db4: f64 = *var_qjci_db4_slot;
        let mut var_qjci_db5: f64 = *var_qjci_db5_slot;
        let mut var_qjci_db6: f64 = *var_qjci_db6_slot;
        let mut var_qjci_db7: f64 = *var_qjci_db7_slot;
        let mut var_qjci_dn0: f64 = *var_qjci_dn0_slot;
        let mut var_qjci_dn1: f64 = *var_qjci_dn1_slot;
        let mut var_qjci_dn2: f64 = *var_qjci_dn2_slot;
        let mut var_qjci_dn3: f64 = *var_qjci_dn3_slot;
        let mut var_qjci_dn4: f64 = *var_qjci_dn4_slot;
        let mut var_qjci_dn5: f64 = *var_qjci_dn5_slot;
        let mut var_qjci_dn6: f64 = *var_qjci_dn6_slot;
        let mut var_qjci_dn7: f64 = *var_qjci_dn7_slot;
        let mut var_qjci_dn8: f64 = *var_qjci_dn8_slot;
        let mut var_qjci_dn9: f64 = *var_qjci_dn9_slot;
        let mut var_qjci_rdb0: f64 = *var_qjci_rdb0_slot;
        let mut var_qjci_rdb1: f64 = *var_qjci_rdb1_slot;
        let mut var_qjci_rdb2: f64 = *var_qjci_rdb2_slot;
        let mut var_qjci_rdb3: f64 = *var_qjci_rdb3_slot;
        let mut var_qjci_rdb4: f64 = *var_qjci_rdb4_slot;
        let mut var_qjci_rdb5: f64 = *var_qjci_rdb5_slot;
        let mut var_qjci_rdb6: f64 = *var_qjci_rdb6_slot;
        let mut var_qjci_rdb7: f64 = *var_qjci_rdb7_slot;
        let mut var_qjci_rdn0: f64 = *var_qjci_rdn0_slot;
        let mut var_qjci_rdn1: f64 = *var_qjci_rdn1_slot;
        let mut var_qjci_rdn2: f64 = *var_qjci_rdn2_slot;
        let mut var_qjci_rdn3: f64 = *var_qjci_rdn3_slot;
        let mut var_qjci_rdn4: f64 = *var_qjci_rdn4_slot;
        let mut var_qjci_rdn5: f64 = *var_qjci_rdn5_slot;
        let mut var_qjci_rdn6: f64 = *var_qjci_rdn6_slot;
        let mut var_qjci_rdn7: f64 = *var_qjci_rdn7_slot;
        let mut var_qjci_rdn8: f64 = *var_qjci_rdn8_slot;
        let mut var_qjci_rdn9: f64 = *var_qjci_rdn9_slot;
        let mut var_qjci_rv: f64 = *var_qjci_rv_slot;
        let mut var_qxf1: f64 = *var_qxf1_slot;
        let mut var_qxf1_db0: f64 = *var_qxf1_db0_slot;
        let mut var_qxf1_db1: f64 = *var_qxf1_db1_slot;
        let mut var_qxf1_db2: f64 = *var_qxf1_db2_slot;
        let mut var_qxf1_db3: f64 = *var_qxf1_db3_slot;
        let mut var_qxf1_db4: f64 = *var_qxf1_db4_slot;
        let mut var_qxf1_db5: f64 = *var_qxf1_db5_slot;
        let mut var_qxf1_db6: f64 = *var_qxf1_db6_slot;
        let mut var_qxf1_db7: f64 = *var_qxf1_db7_slot;
        let mut var_qxf1_dn0: f64 = *var_qxf1_dn0_slot;
        let mut var_qxf1_dn1: f64 = *var_qxf1_dn1_slot;
        let mut var_qxf1_dn2: f64 = *var_qxf1_dn2_slot;
        let mut var_qxf1_dn3: f64 = *var_qxf1_dn3_slot;
        let mut var_qxf1_dn4: f64 = *var_qxf1_dn4_slot;
        let mut var_qxf1_dn5: f64 = *var_qxf1_dn5_slot;
        let mut var_qxf1_dn6: f64 = *var_qxf1_dn6_slot;
        let mut var_qxf1_dn7: f64 = *var_qxf1_dn7_slot;
        let mut var_qxf1_dn8: f64 = *var_qxf1_dn8_slot;
        let mut var_qxf1_dn9: f64 = *var_qxf1_dn9_slot;
        let mut var_qxf1_rdb0: f64 = *var_qxf1_rdb0_slot;
        let mut var_qxf1_rdb1: f64 = *var_qxf1_rdb1_slot;
        let mut var_qxf1_rdb2: f64 = *var_qxf1_rdb2_slot;
        let mut var_qxf1_rdb3: f64 = *var_qxf1_rdb3_slot;
        let mut var_qxf1_rdb4: f64 = *var_qxf1_rdb4_slot;
        let mut var_qxf1_rdb5: f64 = *var_qxf1_rdb5_slot;
        let mut var_qxf1_rdb6: f64 = *var_qxf1_rdb6_slot;
        let mut var_qxf1_rdb7: f64 = *var_qxf1_rdb7_slot;
        let mut var_qxf1_rdn0: f64 = *var_qxf1_rdn0_slot;
        let mut var_qxf1_rdn1: f64 = *var_qxf1_rdn1_slot;
        let mut var_qxf1_rdn2: f64 = *var_qxf1_rdn2_slot;
        let mut var_qxf1_rdn3: f64 = *var_qxf1_rdn3_slot;
        let mut var_qxf1_rdn4: f64 = *var_qxf1_rdn4_slot;
        let mut var_qxf1_rdn5: f64 = *var_qxf1_rdn5_slot;
        let mut var_qxf1_rdn6: f64 = *var_qxf1_rdn6_slot;
        let mut var_qxf1_rdn7: f64 = *var_qxf1_rdn7_slot;
        let mut var_qxf1_rdn8: f64 = *var_qxf1_rdn8_slot;
        let mut var_qxf1_rdn9: f64 = *var_qxf1_rdn9_slot;
        let mut var_qxf1_rv: f64 = *var_qxf1_rv_slot;

        let assign1870_e2179: f64 = (var_qlo + var_qhi);
        let assign1870_e2180: f64 = (var_cjc_t * assign1870_e2179);
        var_qjci = assign1870_e2180;
        var_qjci_dn0 = ((var_cjc_t_dn0 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn0 + var_qhi_dn0)));
        var_qjci_dn1 = ((var_cjc_t_dn1 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn1 + var_qhi_dn1)));
        var_qjci_dn2 = ((var_cjc_t_dn2 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn2 + var_qhi_dn2)));
        var_qjci_dn3 = ((var_cjc_t_dn3 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qjci_dn4 = ((var_cjc_t_dn4 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn4 + var_qhi_dn4)));
        var_qjci_dn5 = ((var_cjc_t_dn5 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn5 + var_qhi_dn5)));
        var_qjci_dn6 = ((var_cjc_t_dn6 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn6 + var_qhi_dn6)));
        var_qjci_dn7 = ((var_cjc_t_dn7 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn7 + var_qhi_dn7)));
        var_qjci_dn8 = ((var_cjc_t_dn8 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn8 + var_qhi_dn8)));
        var_qjci_dn9 = ((var_cjc_t_dn9 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn9 + var_qhi_dn9)));
        var_qjci_db0 = ((var_cjc_t_db0 * assign1870_e2179) + (var_cjc_t * (var_qlo_db0 + var_qhi_db0)));
        var_qjci_db1 = ((var_cjc_t_db1 * assign1870_e2179) + (var_cjc_t * (var_qlo_db1 + var_qhi_db1)));
        var_qjci_db2 = ((var_cjc_t_db2 * assign1870_e2179) + (var_cjc_t * (var_qlo_db2 + var_qhi_db2)));
        var_qjci_db3 = ((var_cjc_t_db3 * assign1870_e2179) + (var_cjc_t * (var_qlo_db3 + var_qhi_db3)));
        var_qjci_db4 = ((var_cjc_t_db4 * assign1870_e2179) + (var_cjc_t * (var_qlo_db4 + var_qhi_db4)));
        var_qjci_db5 = ((var_cjc_t_db5 * assign1870_e2179) + (var_cjc_t * (var_qlo_db5 + var_qhi_db5)));
        var_qjci_db6 = ((var_cjc_t_db6 * assign1870_e2179) + (var_cjc_t * (var_qlo_db6 + var_qhi_db6)));
        var_qjci_db7 = ((var_cjc_t_db7 * assign1870_e2179) + (var_cjc_t * (var_qlo_db7 + var_qhi_db7)));
        var_qjci_rv = 0.0;
        var_qjci_rdn0 = 0.0;
        var_qjci_rdn1 = 0.0;
        var_qjci_rdn2 = 0.0;
        var_qjci_rdn3 = 0.0;
        var_qjci_rdn4 = 0.0;
        var_qjci_rdn5 = 0.0;
        var_qjci_rdn6 = 0.0;
        var_qjci_rdn7 = 0.0;
        var_qjci_rdn8 = 0.0;
        var_qjci_rdn9 = 0.0;
        var_qjci_rdb0 = 0.0;
        var_qjci_rdb1 = 0.0;
        var_qjci_rdb2 = 0.0;
        var_qjci_rdb3 = 0.0;
        var_qjci_rdb4 = 0.0;
        var_qjci_rdb5 = 0.0;
        var_qjci_rdb6 = 0.0;
        var_qjci_rdb7 = 0.0;

        let assign1880_e2183: f64 = (p.p72 * var_qjci);
        var_qjci_1 = assign1880_e2183;
        var_qjci_1_dn0 = (p.p72 * var_qjci_dn0);
        var_qjci_1_dn1 = (p.p72 * var_qjci_dn1);
        var_qjci_1_dn2 = (p.p72 * var_qjci_dn2);
        var_qjci_1_dn3 = (p.p72 * var_qjci_dn3);
        var_qjci_1_dn4 = (p.p72 * var_qjci_dn4);
        var_qjci_1_dn5 = (p.p72 * var_qjci_dn5);
        var_qjci_1_dn6 = (p.p72 * var_qjci_dn6);
        var_qjci_1_dn7 = (p.p72 * var_qjci_dn7);
        var_qjci_1_dn8 = (p.p72 * var_qjci_dn8);
        var_qjci_1_dn9 = (p.p72 * var_qjci_dn9);
        var_qjci_1_db0 = (p.p72 * var_qjci_db0);
        var_qjci_1_db1 = (p.p72 * var_qjci_db1);
        var_qjci_1_db2 = (p.p72 * var_qjci_db2);
        var_qjci_1_db3 = (p.p72 * var_qjci_db3);
        var_qjci_1_db4 = (p.p72 * var_qjci_db4);
        var_qjci_1_db5 = (p.p72 * var_qjci_db5);
        var_qjci_1_db6 = (p.p72 * var_qjci_db6);
        var_qjci_1_db7 = (p.p72 * var_qjci_db7);
        var_qjci_1_rv = 0.0;
        var_qjci_1_rdn0 = 0.0;
        var_qjci_1_rdn1 = 0.0;
        var_qjci_1_rdn2 = 0.0;
        var_qjci_1_rdn3 = 0.0;
        var_qjci_1_rdn4 = 0.0;
        var_qjci_1_rdn5 = 0.0;
        var_qjci_1_rdn6 = 0.0;
        var_qjci_1_rdn7 = 0.0;
        var_qjci_1_rdn8 = 0.0;
        var_qjci_1_rdn9 = 0.0;
        var_qjci_1_rdb0 = 0.0;
        var_qjci_1_rdb1 = 0.0;
        var_qjci_1_rdb2 = 0.0;
        var_qjci_1_rdb3 = 0.0;
        var_qjci_1_rdb4 = 0.0;
        var_qjci_1_rdb5 = 0.0;
        var_qjci_1_rdb6 = 0.0;
        var_qjci_1_rdb7 = 0.0;

        let assign1890_e2190: f64 = if ((p.p68 != 0.0) && (p.p19 != 0.0)) { 1.0 } else { 0.0 };
        var_guard19 = assign1890_e2190;
        var_guard19_dn0 = 0.0;
        var_guard19_dn1 = 0.0;
        var_guard19_dn2 = 0.0;
        var_guard19_dn3 = 0.0;
        var_guard19_dn4 = 0.0;
        var_guard19_dn5 = 0.0;
        var_guard19_dn6 = 0.0;
        var_guard19_dn7 = 0.0;
        var_guard19_dn8 = 0.0;
        var_guard19_dn9 = 0.0;
        var_guard19_db0 = 0.0;
        var_guard19_db1 = 0.0;
        var_guard19_db2 = 0.0;
        var_guard19_db3 = 0.0;
        var_guard19_db4 = 0.0;
        var_guard19_db5 = 0.0;
        var_guard19_db6 = 0.0;
        var_guard19_db7 = 0.0;
        var_guard19_rv = 0.0;
        var_guard19_rdn0 = 0.0;
        var_guard19_rdn1 = 0.0;
        var_guard19_rdn2 = 0.0;
        var_guard19_rdn3 = 0.0;
        var_guard19_rdn4 = 0.0;
        var_guard19_rdn5 = 0.0;
        var_guard19_rdn6 = 0.0;
        var_guard19_rdn7 = 0.0;
        var_guard19_rdn8 = 0.0;
        var_guard19_rdn9 = 0.0;
        var_guard19_rdb0 = 0.0;
        var_guard19_rdb1 = 0.0;
        var_guard19_rdb2 = 0.0;
        var_guard19_rdb3 = 0.0;
        var_guard19_rdb4 = 0.0;
        var_guard19_rdb5 = 0.0;
        var_guard19_rdb6 = 0.0;
        var_guard19_rdb7 = 0.0;

        let (assign1900_e2204, assign1900_e2204_d_n0, assign1900_e2204_d_n1, assign1900_e2204_d_n2, assign1900_e2204_d_n3, assign1900_e2204_d_n4, assign1900_e2204_d_n5, assign1900_e2204_d_n6, assign1900_e2204_d_n7, assign1900_e2204_d_n8, assign1900_e2204_d_n9, assign1900_e2204_d_b0, assign1900_e2204_d_b1, assign1900_e2204_d_b2, assign1900_e2204_d_b3, assign1900_e2204_d_b4, assign1900_e2204_d_b5, assign1900_e2204_d_b6, assign1900_e2204_d_b7,) = {
    if (var_guard19 != 0.0) {
        let assign1900_e2194: f64 = (var_ttype * p.p68);
        let assign1900_e2196: f64 = (assign1900_e2194 * 3.141592653589793);
        let assign1900_e2198: f64 = (assign1900_e2196 / 180.0);
        let assign1900_e2200: f64 = (assign1900_e2198 * p.p19);
        let assign1900_e2202: f64 = (assign1900_e2200 * var_itzf);
        (assign1900_e2202, ((((((var_ttype_dn0 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn0)), ((((((var_ttype_dn1 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn1)), ((((((var_ttype_dn2 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn2)), ((((((var_ttype_dn3 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn3)), ((((((var_ttype_dn4 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn4)), ((((((var_ttype_dn5 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn5)), ((((((var_ttype_dn6 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn6)), ((((((var_ttype_dn7 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn7)), ((((((var_ttype_dn8 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn8)), ((((((var_ttype_dn9 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn9)), ((((((var_ttype_db0 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db0)), ((((((var_ttype_db1 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db1)), ((((((var_ttype_db2 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db2)), ((((((var_ttype_db3 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db3)), ((((((var_ttype_db4 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db4)), ((((((var_ttype_db5 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db5)), ((((((var_ttype_db6 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db6)), ((((((var_ttype_db7 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db7)),)
    } else {
        (var_qxf1, var_qxf1_dn0, var_qxf1_dn1, var_qxf1_dn2, var_qxf1_dn3, var_qxf1_dn4, var_qxf1_dn5, var_qxf1_dn6, var_qxf1_dn7, var_qxf1_dn8, var_qxf1_dn9, var_qxf1_db0, var_qxf1_db1, var_qxf1_db2, var_qxf1_db3, var_qxf1_db4, var_qxf1_db5, var_qxf1_db6, var_qxf1_db7,)
    }
};
        var_qxf1 = assign1900_e2204;
        var_qxf1_dn0 = assign1900_e2204_d_n0;
        var_qxf1_dn1 = assign1900_e2204_d_n1;
        var_qxf1_dn2 = assign1900_e2204_d_n2;
        var_qxf1_dn3 = assign1900_e2204_d_n3;
        var_qxf1_dn4 = assign1900_e2204_d_n4;
        var_qxf1_dn5 = assign1900_e2204_d_n5;
        var_qxf1_dn6 = assign1900_e2204_d_n6;
        var_qxf1_dn7 = assign1900_e2204_d_n7;
        var_qxf1_dn8 = assign1900_e2204_d_n8;
        var_qxf1_dn9 = assign1900_e2204_d_n9;
        var_qxf1_db0 = assign1900_e2204_d_b0;
        var_qxf1_db1 = assign1900_e2204_d_b1;
        var_qxf1_db2 = assign1900_e2204_d_b2;
        var_qxf1_db3 = assign1900_e2204_d_b3;
        var_qxf1_db4 = assign1900_e2204_d_b4;
        var_qxf1_db5 = assign1900_e2204_d_b5;
        var_qxf1_db6 = assign1900_e2204_d_b6;
        var_qxf1_db7 = assign1900_e2204_d_b7;
        var_qxf1_rv = 0.0;
        var_qxf1_rdn0 = 0.0;
        var_qxf1_rdn1 = 0.0;
        var_qxf1_rdn2 = 0.0;
        var_qxf1_rdn3 = 0.0;
        var_qxf1_rdn4 = 0.0;
        var_qxf1_rdn5 = 0.0;
        var_qxf1_rdn6 = 0.0;
        var_qxf1_rdn7 = 0.0;
        var_qxf1_rdn8 = 0.0;
        var_qxf1_rdn9 = 0.0;
        var_qxf1_rdb0 = 0.0;
        var_qxf1_rdb1 = 0.0;
        var_qxf1_rdb2 = 0.0;
        var_qxf1_rdb3 = 0.0;
        var_qxf1_rdb4 = 0.0;
        var_qxf1_rdb5 = 0.0;
        var_qxf1_rdb6 = 0.0;
        var_qxf1_rdb7 = 0.0;

        let (assign1910_e2209, assign1910_e2209_d_n0, assign1910_e2209_d_n1, assign1910_e2209_d_n2, assign1910_e2209_d_n3, assign1910_e2209_d_n4, assign1910_e2209_d_n5, assign1910_e2209_d_n6, assign1910_e2209_d_n7, assign1910_e2209_d_n8, assign1910_e2209_d_n9, assign1910_e2209_d_b0, assign1910_e2209_d_b1, assign1910_e2209_d_b2, assign1910_e2209_d_b3, assign1910_e2209_d_b4, assign1910_e2209_d_b5, assign1910_e2209_d_b6, assign1910_e2209_d_b7,) = {
    if (var_guard19 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qxf1, var_qxf1_dn0, var_qxf1_dn1, var_qxf1_dn2, var_qxf1_dn3, var_qxf1_dn4, var_qxf1_dn5, var_qxf1_dn6, var_qxf1_dn7, var_qxf1_dn8, var_qxf1_dn9, var_qxf1_db0, var_qxf1_db1, var_qxf1_db2, var_qxf1_db3, var_qxf1_db4, var_qxf1_db5, var_qxf1_db6, var_qxf1_db7,)
    }
};
        var_qxf1 = assign1910_e2209;
        var_qxf1_dn0 = assign1910_e2209_d_n0;
        var_qxf1_dn1 = assign1910_e2209_d_n1;
        var_qxf1_dn2 = assign1910_e2209_d_n2;
        var_qxf1_dn3 = assign1910_e2209_d_n3;
        var_qxf1_dn4 = assign1910_e2209_d_n4;
        var_qxf1_dn5 = assign1910_e2209_d_n5;
        var_qxf1_dn6 = assign1910_e2209_d_n6;
        var_qxf1_dn7 = assign1910_e2209_d_n7;
        var_qxf1_dn8 = assign1910_e2209_d_n8;
        var_qxf1_dn9 = assign1910_e2209_d_n9;
        var_qxf1_db0 = assign1910_e2209_d_b0;
        var_qxf1_db1 = assign1910_e2209_d_b1;
        var_qxf1_db2 = assign1910_e2209_d_b2;
        var_qxf1_db3 = assign1910_e2209_d_b3;
        var_qxf1_db4 = assign1910_e2209_d_b4;
        var_qxf1_db5 = assign1910_e2209_d_b5;
        var_qxf1_db6 = assign1910_e2209_d_b6;
        var_qxf1_db7 = assign1910_e2209_d_b7;
        var_qxf1_rv = 0.0;
        var_qxf1_rdn0 = 0.0;
        var_qxf1_rdn1 = 0.0;
        var_qxf1_rdn2 = 0.0;
        var_qxf1_rdn3 = 0.0;
        var_qxf1_rdn4 = 0.0;
        var_qxf1_rdn5 = 0.0;
        var_qxf1_rdn6 = 0.0;
        var_qxf1_rdn7 = 0.0;
        var_qxf1_rdn8 = 0.0;
        var_qxf1_rdn9 = 0.0;
        var_qxf1_rdb0 = 0.0;
        var_qxf1_rdb1 = 0.0;
        var_qxf1_rdb2 = 0.0;
        var_qxf1_rdb3 = 0.0;
        var_qxf1_rdb4 = 0.0;
        var_qxf1_rdb5 = 0.0;
        var_qxf1_rdb6 = 0.0;
        var_qxf1_rdb7 = 0.0;

        let assign1920_e2216: f64 = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        var_guard20 = assign1920_e2216;
        var_guard20_dn0 = 0.0;
        var_guard20_dn1 = 0.0;
        var_guard20_dn2 = 0.0;
        var_guard20_dn3 = 0.0;
        var_guard20_dn4 = 0.0;
        var_guard20_dn5 = 0.0;
        var_guard20_dn6 = 0.0;
        var_guard20_dn7 = 0.0;
        var_guard20_dn8 = 0.0;
        var_guard20_dn9 = 0.0;
        var_guard20_db0 = 0.0;
        var_guard20_db1 = 0.0;
        var_guard20_db2 = 0.0;
        var_guard20_db3 = 0.0;
        var_guard20_db4 = 0.0;
        var_guard20_db5 = 0.0;
        var_guard20_db6 = 0.0;
        var_guard20_db7 = 0.0;
        var_guard20_rv = 0.0;
        var_guard20_rdn0 = 0.0;
        var_guard20_rdn1 = 0.0;
        var_guard20_rdn2 = 0.0;
        var_guard20_rdn3 = 0.0;
        var_guard20_rdn4 = 0.0;
        var_guard20_rdn5 = 0.0;
        var_guard20_rdn6 = 0.0;
        var_guard20_rdn7 = 0.0;
        var_guard20_rdn8 = 0.0;
        var_guard20_rdn9 = 0.0;
        var_guard20_rdb0 = 0.0;
        var_guard20_rdb1 = 0.0;
        var_guard20_rdb2 = 0.0;
        var_guard20_rdb3 = 0.0;
        var_guard20_rdb4 = 0.0;
        var_guard20_rdb5 = 0.0;
        var_guard20_rdb6 = 0.0;
        var_guard20_rdb7 = 0.0;

        let assign1930_e2227: f64 = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };
        var_guard21 = assign1930_e2227;
        var_guard21_dn0 = 0.0;
        var_guard21_dn1 = 0.0;
        var_guard21_dn2 = 0.0;
        var_guard21_dn3 = 0.0;
        var_guard21_dn4 = 0.0;
        var_guard21_dn5 = 0.0;
        var_guard21_dn6 = 0.0;
        var_guard21_dn7 = 0.0;
        var_guard21_dn8 = 0.0;
        var_guard21_dn9 = 0.0;
        var_guard21_db0 = 0.0;
        var_guard21_db1 = 0.0;
        var_guard21_db2 = 0.0;
        var_guard21_db3 = 0.0;
        var_guard21_db4 = 0.0;
        var_guard21_db5 = 0.0;
        var_guard21_db6 = 0.0;
        var_guard21_db7 = 0.0;
        var_guard21_rv = 0.0;
        var_guard21_rdn0 = 0.0;
        var_guard21_rdn1 = 0.0;
        var_guard21_rdn2 = 0.0;
        var_guard21_rdn3 = 0.0;
        var_guard21_rdn4 = 0.0;
        var_guard21_rdn5 = 0.0;
        var_guard21_rdn6 = 0.0;
        var_guard21_rdn7 = 0.0;
        var_guard21_rdn8 = 0.0;
        var_guard21_rdn9 = 0.0;
        var_guard21_rdb0 = 0.0;
        var_guard21_rdb1 = 0.0;
        var_guard21_rdb2 = 0.0;
        var_guard21_rdb3 = 0.0;
        var_guard21_rdb4 = 0.0;
        var_guard21_rdb5 = 0.0;
        var_guard21_rdb6 = 0.0;
        var_guard21_rdb7 = 0.0;

        *var_guard19_slot = var_guard19;
        *var_guard19_db0_slot = var_guard19_db0;
        *var_guard19_db1_slot = var_guard19_db1;
        *var_guard19_db2_slot = var_guard19_db2;
        *var_guard19_db3_slot = var_guard19_db3;
        *var_guard19_db4_slot = var_guard19_db4;
        *var_guard19_db5_slot = var_guard19_db5;
        *var_guard19_db6_slot = var_guard19_db6;
        *var_guard19_db7_slot = var_guard19_db7;
        *var_guard19_dn0_slot = var_guard19_dn0;
        *var_guard19_dn1_slot = var_guard19_dn1;
        *var_guard19_dn2_slot = var_guard19_dn2;
        *var_guard19_dn3_slot = var_guard19_dn3;
        *var_guard19_dn4_slot = var_guard19_dn4;
        *var_guard19_dn5_slot = var_guard19_dn5;
        *var_guard19_dn6_slot = var_guard19_dn6;
        *var_guard19_dn7_slot = var_guard19_dn7;
        *var_guard19_dn8_slot = var_guard19_dn8;
        *var_guard19_dn9_slot = var_guard19_dn9;
        *var_guard19_rdb0_slot = var_guard19_rdb0;
        *var_guard19_rdb1_slot = var_guard19_rdb1;
        *var_guard19_rdb2_slot = var_guard19_rdb2;
        *var_guard19_rdb3_slot = var_guard19_rdb3;
        *var_guard19_rdb4_slot = var_guard19_rdb4;
        *var_guard19_rdb5_slot = var_guard19_rdb5;
        *var_guard19_rdb6_slot = var_guard19_rdb6;
        *var_guard19_rdb7_slot = var_guard19_rdb7;
        *var_guard19_rdn0_slot = var_guard19_rdn0;
        *var_guard19_rdn1_slot = var_guard19_rdn1;
        *var_guard19_rdn2_slot = var_guard19_rdn2;
        *var_guard19_rdn3_slot = var_guard19_rdn3;
        *var_guard19_rdn4_slot = var_guard19_rdn4;
        *var_guard19_rdn5_slot = var_guard19_rdn5;
        *var_guard19_rdn6_slot = var_guard19_rdn6;
        *var_guard19_rdn7_slot = var_guard19_rdn7;
        *var_guard19_rdn8_slot = var_guard19_rdn8;
        *var_guard19_rdn9_slot = var_guard19_rdn9;
        *var_guard19_rv_slot = var_guard19_rv;
        *var_guard20_slot = var_guard20;
        *var_guard20_db0_slot = var_guard20_db0;
        *var_guard20_db1_slot = var_guard20_db1;
        *var_guard20_db2_slot = var_guard20_db2;
        *var_guard20_db3_slot = var_guard20_db3;
        *var_guard20_db4_slot = var_guard20_db4;
        *var_guard20_db5_slot = var_guard20_db5;
        *var_guard20_db6_slot = var_guard20_db6;
        *var_guard20_db7_slot = var_guard20_db7;
        *var_guard20_dn0_slot = var_guard20_dn0;
        *var_guard20_dn1_slot = var_guard20_dn1;
        *var_guard20_dn2_slot = var_guard20_dn2;
        *var_guard20_dn3_slot = var_guard20_dn3;
        *var_guard20_dn4_slot = var_guard20_dn4;
        *var_guard20_dn5_slot = var_guard20_dn5;
        *var_guard20_dn6_slot = var_guard20_dn6;
        *var_guard20_dn7_slot = var_guard20_dn7;
        *var_guard20_dn8_slot = var_guard20_dn8;
        *var_guard20_dn9_slot = var_guard20_dn9;
        *var_guard20_rdb0_slot = var_guard20_rdb0;
        *var_guard20_rdb1_slot = var_guard20_rdb1;
        *var_guard20_rdb2_slot = var_guard20_rdb2;
        *var_guard20_rdb3_slot = var_guard20_rdb3;
        *var_guard20_rdb4_slot = var_guard20_rdb4;
        *var_guard20_rdb5_slot = var_guard20_rdb5;
        *var_guard20_rdb6_slot = var_guard20_rdb6;
        *var_guard20_rdb7_slot = var_guard20_rdb7;
        *var_guard20_rdn0_slot = var_guard20_rdn0;
        *var_guard20_rdn1_slot = var_guard20_rdn1;
        *var_guard20_rdn2_slot = var_guard20_rdn2;
        *var_guard20_rdn3_slot = var_guard20_rdn3;
        *var_guard20_rdn4_slot = var_guard20_rdn4;
        *var_guard20_rdn5_slot = var_guard20_rdn5;
        *var_guard20_rdn6_slot = var_guard20_rdn6;
        *var_guard20_rdn7_slot = var_guard20_rdn7;
        *var_guard20_rdn8_slot = var_guard20_rdn8;
        *var_guard20_rdn9_slot = var_guard20_rdn9;
        *var_guard20_rv_slot = var_guard20_rv;
        *var_guard21_slot = var_guard21;
        *var_guard21_db0_slot = var_guard21_db0;
        *var_guard21_db1_slot = var_guard21_db1;
        *var_guard21_db2_slot = var_guard21_db2;
        *var_guard21_db3_slot = var_guard21_db3;
        *var_guard21_db4_slot = var_guard21_db4;
        *var_guard21_db5_slot = var_guard21_db5;
        *var_guard21_db6_slot = var_guard21_db6;
        *var_guard21_db7_slot = var_guard21_db7;
        *var_guard21_dn0_slot = var_guard21_dn0;
        *var_guard21_dn1_slot = var_guard21_dn1;
        *var_guard21_dn2_slot = var_guard21_dn2;
        *var_guard21_dn3_slot = var_guard21_dn3;
        *var_guard21_dn4_slot = var_guard21_dn4;
        *var_guard21_dn5_slot = var_guard21_dn5;
        *var_guard21_dn6_slot = var_guard21_dn6;
        *var_guard21_dn7_slot = var_guard21_dn7;
        *var_guard21_dn8_slot = var_guard21_dn8;
        *var_guard21_dn9_slot = var_guard21_dn9;
        *var_guard21_rdb0_slot = var_guard21_rdb0;
        *var_guard21_rdb1_slot = var_guard21_rdb1;
        *var_guard21_rdb2_slot = var_guard21_rdb2;
        *var_guard21_rdb3_slot = var_guard21_rdb3;
        *var_guard21_rdb4_slot = var_guard21_rdb4;
        *var_guard21_rdb5_slot = var_guard21_rdb5;
        *var_guard21_rdb6_slot = var_guard21_rdb6;
        *var_guard21_rdb7_slot = var_guard21_rdb7;
        *var_guard21_rdn0_slot = var_guard21_rdn0;
        *var_guard21_rdn1_slot = var_guard21_rdn1;
        *var_guard21_rdn2_slot = var_guard21_rdn2;
        *var_guard21_rdn3_slot = var_guard21_rdn3;
        *var_guard21_rdn4_slot = var_guard21_rdn4;
        *var_guard21_rdn5_slot = var_guard21_rdn5;
        *var_guard21_rdn6_slot = var_guard21_rdn6;
        *var_guard21_rdn7_slot = var_guard21_rdn7;
        *var_guard21_rdn8_slot = var_guard21_rdn8;
        *var_guard21_rdn9_slot = var_guard21_rdn9;
        *var_guard21_rv_slot = var_guard21_rv;
        *var_qjci_slot = var_qjci;
        *var_qjci_1_slot = var_qjci_1;
        *var_qjci_1_db0_slot = var_qjci_1_db0;
        *var_qjci_1_db1_slot = var_qjci_1_db1;
        *var_qjci_1_db2_slot = var_qjci_1_db2;
        *var_qjci_1_db3_slot = var_qjci_1_db3;
        *var_qjci_1_db4_slot = var_qjci_1_db4;
        *var_qjci_1_db5_slot = var_qjci_1_db5;
        *var_qjci_1_db6_slot = var_qjci_1_db6;
        *var_qjci_1_db7_slot = var_qjci_1_db7;
        *var_qjci_1_dn0_slot = var_qjci_1_dn0;
        *var_qjci_1_dn1_slot = var_qjci_1_dn1;
        *var_qjci_1_dn2_slot = var_qjci_1_dn2;
        *var_qjci_1_dn3_slot = var_qjci_1_dn3;
        *var_qjci_1_dn4_slot = var_qjci_1_dn4;
        *var_qjci_1_dn5_slot = var_qjci_1_dn5;
        *var_qjci_1_dn6_slot = var_qjci_1_dn6;
        *var_qjci_1_dn7_slot = var_qjci_1_dn7;
        *var_qjci_1_dn8_slot = var_qjci_1_dn8;
        *var_qjci_1_dn9_slot = var_qjci_1_dn9;
        *var_qjci_1_rdb0_slot = var_qjci_1_rdb0;
        *var_qjci_1_rdb1_slot = var_qjci_1_rdb1;
        *var_qjci_1_rdb2_slot = var_qjci_1_rdb2;
        *var_qjci_1_rdb3_slot = var_qjci_1_rdb3;
        *var_qjci_1_rdb4_slot = var_qjci_1_rdb4;
        *var_qjci_1_rdb5_slot = var_qjci_1_rdb5;
        *var_qjci_1_rdb6_slot = var_qjci_1_rdb6;
        *var_qjci_1_rdb7_slot = var_qjci_1_rdb7;
        *var_qjci_1_rdn0_slot = var_qjci_1_rdn0;
        *var_qjci_1_rdn1_slot = var_qjci_1_rdn1;
        *var_qjci_1_rdn2_slot = var_qjci_1_rdn2;
        *var_qjci_1_rdn3_slot = var_qjci_1_rdn3;
        *var_qjci_1_rdn4_slot = var_qjci_1_rdn4;
        *var_qjci_1_rdn5_slot = var_qjci_1_rdn5;
        *var_qjci_1_rdn6_slot = var_qjci_1_rdn6;
        *var_qjci_1_rdn7_slot = var_qjci_1_rdn7;
        *var_qjci_1_rdn8_slot = var_qjci_1_rdn8;
        *var_qjci_1_rdn9_slot = var_qjci_1_rdn9;
        *var_qjci_1_rv_slot = var_qjci_1_rv;
        *var_qjci_db0_slot = var_qjci_db0;
        *var_qjci_db1_slot = var_qjci_db1;
        *var_qjci_db2_slot = var_qjci_db2;
        *var_qjci_db3_slot = var_qjci_db3;
        *var_qjci_db4_slot = var_qjci_db4;
        *var_qjci_db5_slot = var_qjci_db5;
        *var_qjci_db6_slot = var_qjci_db6;
        *var_qjci_db7_slot = var_qjci_db7;
        *var_qjci_dn0_slot = var_qjci_dn0;
        *var_qjci_dn1_slot = var_qjci_dn1;
        *var_qjci_dn2_slot = var_qjci_dn2;
        *var_qjci_dn3_slot = var_qjci_dn3;
        *var_qjci_dn4_slot = var_qjci_dn4;
        *var_qjci_dn5_slot = var_qjci_dn5;
        *var_qjci_dn6_slot = var_qjci_dn6;
        *var_qjci_dn7_slot = var_qjci_dn7;
        *var_qjci_dn8_slot = var_qjci_dn8;
        *var_qjci_dn9_slot = var_qjci_dn9;
        *var_qjci_rdb0_slot = var_qjci_rdb0;
        *var_qjci_rdb1_slot = var_qjci_rdb1;
        *var_qjci_rdb2_slot = var_qjci_rdb2;
        *var_qjci_rdb3_slot = var_qjci_rdb3;
        *var_qjci_rdb4_slot = var_qjci_rdb4;
        *var_qjci_rdb5_slot = var_qjci_rdb5;
        *var_qjci_rdb6_slot = var_qjci_rdb6;
        *var_qjci_rdb7_slot = var_qjci_rdb7;
        *var_qjci_rdn0_slot = var_qjci_rdn0;
        *var_qjci_rdn1_slot = var_qjci_rdn1;
        *var_qjci_rdn2_slot = var_qjci_rdn2;
        *var_qjci_rdn3_slot = var_qjci_rdn3;
        *var_qjci_rdn4_slot = var_qjci_rdn4;
        *var_qjci_rdn5_slot = var_qjci_rdn5;
        *var_qjci_rdn6_slot = var_qjci_rdn6;
        *var_qjci_rdn7_slot = var_qjci_rdn7;
        *var_qjci_rdn8_slot = var_qjci_rdn8;
        *var_qjci_rdn9_slot = var_qjci_rdn9;
        *var_qjci_rv_slot = var_qjci_rv;
        *var_qxf1_slot = var_qxf1;
        *var_qxf1_db0_slot = var_qxf1_db0;
        *var_qxf1_db1_slot = var_qxf1_db1;
        *var_qxf1_db2_slot = var_qxf1_db2;
        *var_qxf1_db3_slot = var_qxf1_db3;
        *var_qxf1_db4_slot = var_qxf1_db4;
        *var_qxf1_db5_slot = var_qxf1_db5;
        *var_qxf1_db6_slot = var_qxf1_db6;
        *var_qxf1_db7_slot = var_qxf1_db7;
        *var_qxf1_dn0_slot = var_qxf1_dn0;
        *var_qxf1_dn1_slot = var_qxf1_dn1;
        *var_qxf1_dn2_slot = var_qxf1_dn2;
        *var_qxf1_dn3_slot = var_qxf1_dn3;
        *var_qxf1_dn4_slot = var_qxf1_dn4;
        *var_qxf1_dn5_slot = var_qxf1_dn5;
        *var_qxf1_dn6_slot = var_qxf1_dn6;
        *var_qxf1_dn7_slot = var_qxf1_dn7;
        *var_qxf1_dn8_slot = var_qxf1_dn8;
        *var_qxf1_dn9_slot = var_qxf1_dn9;
        *var_qxf1_rdb0_slot = var_qxf1_rdb0;
        *var_qxf1_rdb1_slot = var_qxf1_rdb1;
        *var_qxf1_rdb2_slot = var_qxf1_rdb2;
        *var_qxf1_rdb3_slot = var_qxf1_rdb3;
        *var_qxf1_rdb4_slot = var_qxf1_rdb4;
        *var_qxf1_rdb5_slot = var_qxf1_rdb5;
        *var_qxf1_rdb6_slot = var_qxf1_rdb6;
        *var_qxf1_rdb7_slot = var_qxf1_rdb7;
        *var_qxf1_rdn0_slot = var_qxf1_rdn0;
        *var_qxf1_rdn1_slot = var_qxf1_rdn1;
        *var_qxf1_rdn2_slot = var_qxf1_rdn2;
        *var_qxf1_rdn3_slot = var_qxf1_rdn3;
        *var_qxf1_rdn4_slot = var_qxf1_rdn4;
        *var_qxf1_rdn5_slot = var_qxf1_rdn5;
        *var_qxf1_rdn6_slot = var_qxf1_rdn6;
        *var_qxf1_rdn7_slot = var_qxf1_rdn7;
        *var_qxf1_rdn8_slot = var_qxf1_rdn8;
        *var_qxf1_rdn9_slot = var_qxf1_rdn9;
        *var_qxf1_rv_slot = var_qxf1_rv;
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
        var_bf_t: f64,
        var_bf_t_db0: f64,
        var_bf_t_db1: f64,
        var_bf_t_db2: f64,
        var_bf_t_db3: f64,
        var_bf_t_db4: f64,
        var_bf_t_db5: f64,
        var_bf_t_db6: f64,
        var_bf_t_db7: f64,
        var_bf_t_dn0: f64,
        var_bf_t_dn1: f64,
        var_bf_t_dn2: f64,
        var_bf_t_dn3: f64,
        var_bf_t_dn4: f64,
        var_bf_t_dn5: f64,
        var_bf_t_dn6: f64,
        var_bf_t_dn7: f64,
        var_bf_t_dn8: f64,
        var_bf_t_dn9: f64,
        var_guard13: f64,
        var_guard20: f64,
        var_guard21: f64,
        var_ibc: f64,
        var_ibc_db0: f64,
        var_ibc_db1: f64,
        var_ibc_db2: f64,
        var_ibc_db3: f64,
        var_ibc_db4: f64,
        var_ibc_db5: f64,
        var_ibc_db6: f64,
        var_ibc_db7: f64,
        var_ibc_dn0: f64,
        var_ibc_dn1: f64,
        var_ibc_dn2: f64,
        var_ibc_dn3: f64,
        var_ibc_dn4: f64,
        var_ibc_dn5: f64,
        var_ibc_dn6: f64,
        var_ibc_dn7: f64,
        var_ibc_dn8: f64,
        var_ibc_dn9: f64,
        var_ibe: f64,
        var_ibe_db0: f64,
        var_ibe_db1: f64,
        var_ibe_db2: f64,
        var_ibe_db3: f64,
        var_ibe_db4: f64,
        var_ibe_db5: f64,
        var_ibe_db6: f64,
        var_ibe_db7: f64,
        var_ibe_dn0: f64,
        var_ibe_dn1: f64,
        var_ibe_dn2: f64,
        var_ibe_dn3: f64,
        var_ibe_dn4: f64,
        var_ibe_dn5: f64,
        var_ibe_dn6: f64,
        var_ibe_dn7: f64,
        var_ibe_dn8: f64,
        var_ibe_dn9: f64,
        var_ifwd: f64,
        var_ifwd_db0: f64,
        var_ifwd_db1: f64,
        var_ifwd_db2: f64,
        var_ifwd_db3: f64,
        var_ifwd_db4: f64,
        var_ifwd_db5: f64,
        var_ifwd_db6: f64,
        var_ifwd_db7: f64,
        var_ifwd_dn0: f64,
        var_ifwd_dn1: f64,
        var_ifwd_dn2: f64,
        var_ifwd_dn3: f64,
        var_ifwd_dn4: f64,
        var_ifwd_dn5: f64,
        var_ifwd_dn6: f64,
        var_ifwd_dn7: f64,
        var_ifwd_dn8: f64,
        var_ifwd_dn9: f64,
        var_tff: f64,
        var_tff_db0: f64,
        var_tff_db1: f64,
        var_tff_db2: f64,
        var_tff_db3: f64,
        var_tff_db4: f64,
        var_tff_db5: f64,
        var_tff_db6: f64,
        var_tff_db7: f64,
        var_tff_dn0: f64,
        var_tff_dn1: f64,
        var_tff_dn2: f64,
        var_tff_dn3: f64,
        var_tff_dn4: f64,
        var_tff_dn5: f64,
        var_tff_dn6: f64,
        var_tff_dn7: f64,
        var_tff_dn8: f64,
        var_tff_dn9: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq2_e98: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (nv9 - 0.0));
        let eq2_e99: f64 = (p.p83 * eq2_e98);
        let eq2_value: f64 = eq2_e99;
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * (eq2_value),
            9,
            multiplicity * ((p.p83 * ddt_scale)),
        );
        let (eq3_e108, eq3_e108_d_n0, eq3_e108_d_n1, eq3_e108_d_n2, eq3_e108_d_n3, eq3_e108_d_n4, eq3_e108_d_n5, eq3_e108_d_n6, eq3_e108_d_n7, eq3_e108_d_n8, eq3_e108_d_n9, eq3_e108_d_b0, eq3_e108_d_b1, eq3_e108_d_b2, eq3_e108_d_b3, eq3_e108_d_b4, eq3_e108_d_b5, eq3_e108_d_b6, eq3_e108_d_b7,) = {
    if (var_guard13 != 0.0) {
        let eq3_e103: f64 = (var_ifwd / var_bf_t);
        let __rspice_inv_cse_0: f64 = 1.0 / (var_bf_t * var_bf_t);
        let eq3_e103_d_n0: f64 = (((var_ifwd_dn0 * var_bf_t) - (var_ifwd * var_bf_t_dn0)) * __rspice_inv_cse_0);
        let eq3_e103_d_n1: f64 = (((var_ifwd_dn1 * var_bf_t) - (var_ifwd * var_bf_t_dn1)) * __rspice_inv_cse_0);
        let eq3_e103_d_n2: f64 = (((var_ifwd_dn2 * var_bf_t) - (var_ifwd * var_bf_t_dn2)) * __rspice_inv_cse_0);
        let eq3_e103_d_n3: f64 = (((var_ifwd_dn3 * var_bf_t) - (var_ifwd * var_bf_t_dn3)) * __rspice_inv_cse_0);
        let eq3_e103_d_n4: f64 = (((var_ifwd_dn4 * var_bf_t) - (var_ifwd * var_bf_t_dn4)) * __rspice_inv_cse_0);
        let eq3_e103_d_n5: f64 = (((var_ifwd_dn5 * var_bf_t) - (var_ifwd * var_bf_t_dn5)) * __rspice_inv_cse_0);
        let eq3_e103_d_n6: f64 = (((var_ifwd_dn6 * var_bf_t) - (var_ifwd * var_bf_t_dn6)) * __rspice_inv_cse_0);
        let eq3_e103_d_n7: f64 = (((var_ifwd_dn7 * var_bf_t) - (var_ifwd * var_bf_t_dn7)) * __rspice_inv_cse_0);
        let eq3_e103_d_n8: f64 = (((var_ifwd_dn8 * var_bf_t) - (var_ifwd * var_bf_t_dn8)) * __rspice_inv_cse_0);
        let eq3_e103_d_n9: f64 = (((var_ifwd_dn9 * var_bf_t) - (var_ifwd * var_bf_t_dn9)) * __rspice_inv_cse_0);
        let eq3_e103_d_b0: f64 = (((var_ifwd_db0 * var_bf_t) - (var_ifwd * var_bf_t_db0)) * __rspice_inv_cse_0);
        let eq3_e103_d_b1: f64 = (((var_ifwd_db1 * var_bf_t) - (var_ifwd * var_bf_t_db1)) * __rspice_inv_cse_0);
        let eq3_e103_d_b2: f64 = (((var_ifwd_db2 * var_bf_t) - (var_ifwd * var_bf_t_db2)) * __rspice_inv_cse_0);
        let eq3_e103_d_b3: f64 = (((var_ifwd_db3 * var_bf_t) - (var_ifwd * var_bf_t_db3)) * __rspice_inv_cse_0);
        let eq3_e103_d_b4: f64 = (((var_ifwd_db4 * var_bf_t) - (var_ifwd * var_bf_t_db4)) * __rspice_inv_cse_0);
        let eq3_e103_d_b5: f64 = (((var_ifwd_db5 * var_bf_t) - (var_ifwd * var_bf_t_db5)) * __rspice_inv_cse_0);
        let eq3_e103_d_b6: f64 = (((var_ifwd_db6 * var_bf_t) - (var_ifwd * var_bf_t_db6)) * __rspice_inv_cse_0);
        let eq3_e103_d_b7: f64 = (((var_ifwd_db7 * var_bf_t) - (var_ifwd * var_bf_t_db7)) * __rspice_inv_cse_0);
        let eq3_e104: f64 = (-eq3_e103);
        let eq3_e106: f64 = (eq3_e104 * var_tff);
        let eq3_e106_d_n0: f64 = (((-eq3_e103_d_n0) * var_tff) + (eq3_e104 * var_tff_dn0));
        let eq3_e106_d_n1: f64 = (((-eq3_e103_d_n1) * var_tff) + (eq3_e104 * var_tff_dn1));
        let eq3_e106_d_n2: f64 = (((-eq3_e103_d_n2) * var_tff) + (eq3_e104 * var_tff_dn2));
        let eq3_e106_d_n3: f64 = (((-eq3_e103_d_n3) * var_tff) + (eq3_e104 * var_tff_dn3));
        let eq3_e106_d_n4: f64 = (((-eq3_e103_d_n4) * var_tff) + (eq3_e104 * var_tff_dn4));
        let eq3_e106_d_n5: f64 = (((-eq3_e103_d_n5) * var_tff) + (eq3_e104 * var_tff_dn5));
        let eq3_e106_d_n6: f64 = (((-eq3_e103_d_n6) * var_tff) + (eq3_e104 * var_tff_dn6));
        let eq3_e106_d_n7: f64 = (((-eq3_e103_d_n7) * var_tff) + (eq3_e104 * var_tff_dn7));
        let eq3_e106_d_n8: f64 = (((-eq3_e103_d_n8) * var_tff) + (eq3_e104 * var_tff_dn8));
        let eq3_e106_d_n9: f64 = (((-eq3_e103_d_n9) * var_tff) + (eq3_e104 * var_tff_dn9));
        let eq3_e106_d_b0: f64 = (((-eq3_e103_d_b0) * var_tff) + (eq3_e104 * var_tff_db0));
        let eq3_e106_d_b1: f64 = (((-eq3_e103_d_b1) * var_tff) + (eq3_e104 * var_tff_db1));
        let eq3_e106_d_b2: f64 = (((-eq3_e103_d_b2) * var_tff) + (eq3_e104 * var_tff_db2));
        let eq3_e106_d_b3: f64 = (((-eq3_e103_d_b3) * var_tff) + (eq3_e104 * var_tff_db3));
        let eq3_e106_d_b4: f64 = (((-eq3_e103_d_b4) * var_tff) + (eq3_e104 * var_tff_db4));
        let eq3_e106_d_b5: f64 = (((-eq3_e103_d_b5) * var_tff) + (eq3_e104 * var_tff_db5));
        let eq3_e106_d_b6: f64 = (((-eq3_e103_d_b6) * var_tff) + (eq3_e104 * var_tff_db6));
        let eq3_e106_d_b7: f64 = (((-eq3_e103_d_b7) * var_tff) + (eq3_e104 * var_tff_db7));
        (eq3_e106, eq3_e106_d_n0, eq3_e106_d_n1, eq3_e106_d_n2, eq3_e106_d_n3, eq3_e106_d_n4, eq3_e106_d_n5, eq3_e106_d_n6, eq3_e106_d_n7, eq3_e106_d_n8, eq3_e106_d_n9, eq3_e106_d_b0, eq3_e106_d_b1, eq3_e106_d_b2, eq3_e106_d_b3, eq3_e106_d_b4, eq3_e106_d_b5, eq3_e106_d_b6, eq3_e106_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e108;
        let eq3_node_derivatives: [f64; 10] = [eq3_e108_d_n0, eq3_e108_d_n1, eq3_e108_d_n2, eq3_e108_d_n3, eq3_e108_d_n4, eq3_e108_d_n5, eq3_e108_d_n6, eq3_e108_d_n7, eq3_e108_d_n8, eq3_e108_d_n9];
        let eq3_branch_derivatives: [f64; 8] = [eq3_e108_d_b0, eq3_e108_d_b1, eq3_e108_d_b2, eq3_e108_d_b3, eq3_e108_d_b4, eq3_e108_d_b5, eq3_e108_d_b6, eq3_e108_d_b7];
        stamper.stamp_current_dense_local(
            Some(8),
            None,
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq5_e121, eq5_e121_d_n0, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n3, eq5_e121_d_n4, eq5_e121_d_n5, eq5_e121_d_n6, eq5_e121_d_n7, eq5_e121_d_n8, eq5_e121_d_n9, eq5_e121_d_b0, eq5_e121_d_b1, eq5_e121_d_b2, eq5_e121_d_b3, eq5_e121_d_b4, eq5_e121_d_b5, eq5_e121_d_b6, eq5_e121_d_b7,) = {
    if (var_guard13 != 0.0) {
        let eq5_e118: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (nv8 - 0.0));
        let eq5_e119: f64 = (var_tff * eq5_e118);
        let eq5_e119_d_n0: f64 = (var_tff_dn0 * eq5_e118);
        let eq5_e119_d_n1: f64 = (var_tff_dn1 * eq5_e118);
        let eq5_e119_d_n2: f64 = (var_tff_dn2 * eq5_e118);
        let eq5_e119_d_n3: f64 = (var_tff_dn3 * eq5_e118);
        let eq5_e119_d_n4: f64 = (var_tff_dn4 * eq5_e118);
        let eq5_e119_d_n5: f64 = (var_tff_dn5 * eq5_e118);
        let eq5_e119_d_n6: f64 = (var_tff_dn6 * eq5_e118);
        let eq5_e119_d_n7: f64 = (var_tff_dn7 * eq5_e118);
        let eq5_e119_d_n8: f64 = ((var_tff_dn8 * eq5_e118) + (var_tff * ddt_scale));
        let eq5_e119_d_n9: f64 = (var_tff_dn9 * eq5_e118);
        let eq5_e119_d_b0: f64 = (var_tff_db0 * eq5_e118);
        let eq5_e119_d_b1: f64 = (var_tff_db1 * eq5_e118);
        let eq5_e119_d_b2: f64 = (var_tff_db2 * eq5_e118);
        let eq5_e119_d_b3: f64 = (var_tff_db3 * eq5_e118);
        let eq5_e119_d_b4: f64 = (var_tff_db4 * eq5_e118);
        let eq5_e119_d_b5: f64 = (var_tff_db5 * eq5_e118);
        let eq5_e119_d_b6: f64 = (var_tff_db6 * eq5_e118);
        let eq5_e119_d_b7: f64 = (var_tff_db7 * eq5_e118);
        (eq5_e119, eq5_e119_d_n0, eq5_e119_d_n1, eq5_e119_d_n2, eq5_e119_d_n3, eq5_e119_d_n4, eq5_e119_d_n5, eq5_e119_d_n6, eq5_e119_d_n7, eq5_e119_d_n8, eq5_e119_d_n9, eq5_e119_d_b0, eq5_e119_d_b1, eq5_e119_d_b2, eq5_e119_d_b3, eq5_e119_d_b4, eq5_e119_d_b5, eq5_e119_d_b6, eq5_e119_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e121;
        let eq5_node_derivatives: [f64; 10] = [eq5_e121_d_n0, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n3, eq5_e121_d_n4, eq5_e121_d_n5, eq5_e121_d_n6, eq5_e121_d_n7, eq5_e121_d_n8, eq5_e121_d_n9];
        let eq5_branch_derivatives: [f64; 8] = [eq5_e121_d_b0, eq5_e121_d_b1, eq5_e121_d_b2, eq5_e121_d_b3, eq5_e121_d_b4, eq5_e121_d_b5, eq5_e121_d_b6, eq5_e121_d_b7];
        stamper.stamp_current_dense_local(
            Some(8),
            None,
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq7_e141, eq7_e141_d_n0, eq7_e141_d_n1, eq7_e141_d_n2, eq7_e141_d_n3, eq7_e141_d_n4, eq7_e141_d_n5, eq7_e141_d_n6, eq7_e141_d_n7, eq7_e141_d_n8, eq7_e141_d_n9, eq7_e141_d_b0, eq7_e141_d_b1, eq7_e141_d_b2, eq7_e141_d_b3, eq7_e141_d_b4, eq7_e141_d_b5, eq7_e141_d_b6, eq7_e141_d_b7,) = {
    if (var_guard20 != 0.0) {
        let eq7_e129: f64 = (-1.0);
        let eq7_e132: f64 = (var_ibe * (nv1 - nv2));
        let eq7_e132_d_n0: f64 = (var_ibe_dn0 * (nv1 - nv2));
        let eq7_e132_d_n1: f64 = ((var_ibe_dn1 * (nv1 - nv2)) + var_ibe);
        let eq7_e132_d_n2: f64 = ((var_ibe_dn2 * (nv1 - nv2)) + (-var_ibe));
        let eq7_e132_d_n3: f64 = (var_ibe_dn3 * (nv1 - nv2));
        let eq7_e132_d_n4: f64 = (var_ibe_dn4 * (nv1 - nv2));
        let eq7_e132_d_n5: f64 = (var_ibe_dn5 * (nv1 - nv2));
        let eq7_e132_d_n6: f64 = (var_ibe_dn6 * (nv1 - nv2));
        let eq7_e132_d_n7: f64 = (var_ibe_dn7 * (nv1 - nv2));
        let eq7_e132_d_n8: f64 = (var_ibe_dn8 * (nv1 - nv2));
        let eq7_e132_d_n9: f64 = (var_ibe_dn9 * (nv1 - nv2));
        let eq7_e132_d_b0: f64 = (var_ibe_db0 * (nv1 - nv2));
        let eq7_e132_d_b1: f64 = (var_ibe_db1 * (nv1 - nv2));
        let eq7_e132_d_b2: f64 = (var_ibe_db2 * (nv1 - nv2));
        let eq7_e132_d_b3: f64 = (var_ibe_db3 * (nv1 - nv2));
        let eq7_e132_d_b4: f64 = (var_ibe_db4 * (nv1 - nv2));
        let eq7_e132_d_b5: f64 = (var_ibe_db5 * (nv1 - nv2));
        let eq7_e132_d_b6: f64 = (var_ibe_db6 * (nv1 - nv2));
        let eq7_e132_d_b7: f64 = (var_ibe_db7 * (nv1 - nv2));
        let eq7_e133: f64 = (eq7_e132).abs();
        let eq7_e133_d_n0: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n0 } else { (-eq7_e132_d_n0) };
        let eq7_e133_d_n1: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n1 } else { (-eq7_e132_d_n1) };
        let eq7_e133_d_n2: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n2 } else { (-eq7_e132_d_n2) };
        let eq7_e133_d_n3: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n3 } else { (-eq7_e132_d_n3) };
        let eq7_e133_d_n4: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n4 } else { (-eq7_e132_d_n4) };
        let eq7_e133_d_n5: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n5 } else { (-eq7_e132_d_n5) };
        let eq7_e133_d_n6: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n6 } else { (-eq7_e132_d_n6) };
        let eq7_e133_d_n7: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n7 } else { (-eq7_e132_d_n7) };
        let eq7_e133_d_n8: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n8 } else { (-eq7_e132_d_n8) };
        let eq7_e133_d_n9: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n9 } else { (-eq7_e132_d_n9) };
        let eq7_e133_d_b0: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b0 } else { (-eq7_e132_d_b0) };
        let eq7_e133_d_b1: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b1 } else { (-eq7_e132_d_b1) };
        let eq7_e133_d_b2: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b2 } else { (-eq7_e132_d_b2) };
        let eq7_e133_d_b3: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b3 } else { (-eq7_e132_d_b3) };
        let eq7_e133_d_b4: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b4 } else { (-eq7_e132_d_b4) };
        let eq7_e133_d_b5: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b5 } else { (-eq7_e132_d_b5) };
        let eq7_e133_d_b6: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b6 } else { (-eq7_e132_d_b6) };
        let eq7_e133_d_b7: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b7 } else { (-eq7_e132_d_b7) };
        let eq7_e134: f64 = (eq7_e129 * eq7_e133);
        let eq7_e134_d_n0: f64 = (eq7_e129 * eq7_e133_d_n0);
        let eq7_e134_d_n1: f64 = (eq7_e129 * eq7_e133_d_n1);
        let eq7_e134_d_n2: f64 = (eq7_e129 * eq7_e133_d_n2);
        let eq7_e134_d_n3: f64 = (eq7_e129 * eq7_e133_d_n3);
        let eq7_e134_d_n4: f64 = (eq7_e129 * eq7_e133_d_n4);
        let eq7_e134_d_n5: f64 = (eq7_e129 * eq7_e133_d_n5);
        let eq7_e134_d_n6: f64 = (eq7_e129 * eq7_e133_d_n6);
        let eq7_e134_d_n7: f64 = (eq7_e129 * eq7_e133_d_n7);
        let eq7_e134_d_n8: f64 = (eq7_e129 * eq7_e133_d_n8);
        let eq7_e134_d_n9: f64 = (eq7_e129 * eq7_e133_d_n9);
        let eq7_e134_d_b0: f64 = (eq7_e129 * eq7_e133_d_b0);
        let eq7_e134_d_b1: f64 = (eq7_e129 * eq7_e133_d_b1);
        let eq7_e134_d_b2: f64 = (eq7_e129 * eq7_e133_d_b2);
        let eq7_e134_d_b3: f64 = (eq7_e129 * eq7_e133_d_b3);
        let eq7_e134_d_b4: f64 = (eq7_e129 * eq7_e133_d_b4);
        let eq7_e134_d_b5: f64 = (eq7_e129 * eq7_e133_d_b5);
        let eq7_e134_d_b6: f64 = (eq7_e129 * eq7_e133_d_b6);
        let eq7_e134_d_b7: f64 = (eq7_e129 * eq7_e133_d_b7);
        let eq7_e137: f64 = (var_ibc * (nv1 - nv0));
        let eq7_e137_d_n0: f64 = ((var_ibc_dn0 * (nv1 - nv0)) + (-var_ibc));
        let eq7_e137_d_n1: f64 = ((var_ibc_dn1 * (nv1 - nv0)) + var_ibc);
        let eq7_e137_d_n2: f64 = (var_ibc_dn2 * (nv1 - nv0));
        let eq7_e137_d_n3: f64 = (var_ibc_dn3 * (nv1 - nv0));
        let eq7_e137_d_n4: f64 = (var_ibc_dn4 * (nv1 - nv0));
        let eq7_e137_d_n5: f64 = (var_ibc_dn5 * (nv1 - nv0));
        let eq7_e137_d_n6: f64 = (var_ibc_dn6 * (nv1 - nv0));
        let eq7_e137_d_n7: f64 = (var_ibc_dn7 * (nv1 - nv0));
        let eq7_e137_d_n8: f64 = (var_ibc_dn8 * (nv1 - nv0));
        let eq7_e137_d_n9: f64 = (var_ibc_dn9 * (nv1 - nv0));
        let eq7_e137_d_b0: f64 = (var_ibc_db0 * (nv1 - nv0));
        let eq7_e137_d_b1: f64 = (var_ibc_db1 * (nv1 - nv0));
        let eq7_e137_d_b2: f64 = (var_ibc_db2 * (nv1 - nv0));
        let eq7_e137_d_b3: f64 = (var_ibc_db3 * (nv1 - nv0));
        let eq7_e137_d_b4: f64 = (var_ibc_db4 * (nv1 - nv0));
        let eq7_e137_d_b5: f64 = (var_ibc_db5 * (nv1 - nv0));
        let eq7_e137_d_b6: f64 = (var_ibc_db6 * (nv1 - nv0));
        let eq7_e137_d_b7: f64 = (var_ibc_db7 * (nv1 - nv0));
        let eq7_e138: f64 = (eq7_e137).abs();
        let eq7_e138_d_n0: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n0 } else { (-eq7_e137_d_n0) };
        let eq7_e138_d_n1: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n1 } else { (-eq7_e137_d_n1) };
        let eq7_e138_d_n2: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n2 } else { (-eq7_e137_d_n2) };
        let eq7_e138_d_n3: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n3 } else { (-eq7_e137_d_n3) };
        let eq7_e138_d_n4: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n4 } else { (-eq7_e137_d_n4) };
        let eq7_e138_d_n5: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n5 } else { (-eq7_e137_d_n5) };
        let eq7_e138_d_n6: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n6 } else { (-eq7_e137_d_n6) };
        let eq7_e138_d_n7: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n7 } else { (-eq7_e137_d_n7) };
        let eq7_e138_d_n8: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n8 } else { (-eq7_e137_d_n8) };
        let eq7_e138_d_n9: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n9 } else { (-eq7_e137_d_n9) };
        let eq7_e138_d_b0: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b0 } else { (-eq7_e137_d_b0) };
        let eq7_e138_d_b1: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b1 } else { (-eq7_e137_d_b1) };
        let eq7_e138_d_b2: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b2 } else { (-eq7_e137_d_b2) };
        let eq7_e138_d_b3: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b3 } else { (-eq7_e137_d_b3) };
        let eq7_e138_d_b4: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b4 } else { (-eq7_e137_d_b4) };
        let eq7_e138_d_b5: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b5 } else { (-eq7_e137_d_b5) };
        let eq7_e138_d_b6: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b6 } else { (-eq7_e137_d_b6) };
        let eq7_e138_d_b7: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b7 } else { (-eq7_e137_d_b7) };
        let eq7_e139: f64 = (eq7_e134 - eq7_e138);
        let eq7_e139_d_n0: f64 = (eq7_e134_d_n0 - eq7_e138_d_n0);
        let eq7_e139_d_n1: f64 = (eq7_e134_d_n1 - eq7_e138_d_n1);
        let eq7_e139_d_n2: f64 = (eq7_e134_d_n2 - eq7_e138_d_n2);
        let eq7_e139_d_n3: f64 = (eq7_e134_d_n3 - eq7_e138_d_n3);
        let eq7_e139_d_n4: f64 = (eq7_e134_d_n4 - eq7_e138_d_n4);
        let eq7_e139_d_n5: f64 = (eq7_e134_d_n5 - eq7_e138_d_n5);
        let eq7_e139_d_n6: f64 = (eq7_e134_d_n6 - eq7_e138_d_n6);
        let eq7_e139_d_n7: f64 = (eq7_e134_d_n7 - eq7_e138_d_n7);
        let eq7_e139_d_n8: f64 = (eq7_e134_d_n8 - eq7_e138_d_n8);
        let eq7_e139_d_n9: f64 = (eq7_e134_d_n9 - eq7_e138_d_n9);
        let eq7_e139_d_b0: f64 = (eq7_e134_d_b0 - eq7_e138_d_b0);
        let eq7_e139_d_b1: f64 = (eq7_e134_d_b1 - eq7_e138_d_b1);
        let eq7_e139_d_b2: f64 = (eq7_e134_d_b2 - eq7_e138_d_b2);
        let eq7_e139_d_b3: f64 = (eq7_e134_d_b3 - eq7_e138_d_b3);
        let eq7_e139_d_b4: f64 = (eq7_e134_d_b4 - eq7_e138_d_b4);
        let eq7_e139_d_b5: f64 = (eq7_e134_d_b5 - eq7_e138_d_b5);
        let eq7_e139_d_b6: f64 = (eq7_e134_d_b6 - eq7_e138_d_b6);
        let eq7_e139_d_b7: f64 = (eq7_e134_d_b7 - eq7_e138_d_b7);
        (eq7_e139, eq7_e139_d_n0, eq7_e139_d_n1, eq7_e139_d_n2, eq7_e139_d_n3, eq7_e139_d_n4, eq7_e139_d_n5, eq7_e139_d_n6, eq7_e139_d_n7, eq7_e139_d_n8, eq7_e139_d_n9, eq7_e139_d_b0, eq7_e139_d_b1, eq7_e139_d_b2, eq7_e139_d_b3, eq7_e139_d_b4, eq7_e139_d_b5, eq7_e139_d_b6, eq7_e139_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e141;
        let eq7_node_derivatives: [f64; 10] = [eq7_e141_d_n0, eq7_e141_d_n1, eq7_e141_d_n2, eq7_e141_d_n3, eq7_e141_d_n4, eq7_e141_d_n5, eq7_e141_d_n6, eq7_e141_d_n7, eq7_e141_d_n8, eq7_e141_d_n9];
        let eq7_branch_derivatives: [f64; 8] = [eq7_e141_d_b0, eq7_e141_d_b1, eq7_e141_d_b2, eq7_e141_d_b3, eq7_e141_d_b4, eq7_e141_d_b5, eq7_e141_d_b6, eq7_e141_d_b7];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e147, eq8_e147_d_n3,) = {
    if (var_guard20 != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / p.p33;
        let eq8_e145: f64 = ((nv3 - 0.0) * __rspice_inv_cse_1);
        let eq8_e145_d_n3: f64 = (1.0 * __rspice_inv_cse_1);
        (eq8_e145, eq8_e145_d_n3,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e147;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq8_value),
            3,
            multiplicity * (eq8_e147_d_n3),
        );
        let (eq9_e154, eq9_e154_d_n3,) = {
    if (var_guard20 != 0.0) {
        let eq9_e151: f64 = ((nv3 - 0.0) * p.p34);
        let eq9_e152: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq9_e151);
        (eq9_e152, (p.p34 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e154;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq9_value),
            3,
            multiplicity * (eq9_e154_d_n3),
        );
        let (eq11_e176, eq11_e176_d_n0, eq11_e176_d_n1, eq11_e176_d_n2, eq11_e176_d_n3, eq11_e176_d_n4, eq11_e176_d_n5, eq11_e176_d_n6, eq11_e176_d_n7, eq11_e176_d_n8, eq11_e176_d_n9, eq11_e176_d_b0, eq11_e176_d_b1, eq11_e176_d_b2, eq11_e176_d_b3, eq11_e176_d_b4, eq11_e176_d_b5, eq11_e176_d_b6, eq11_e176_d_b7,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq11_e164: f64 = (-1.0);
        let eq11_e167: f64 = (var_ibe * (nv1 - nv2));
        let eq11_e167_d_n0: f64 = (var_ibe_dn0 * (nv1 - nv2));
        let eq11_e167_d_n1: f64 = ((var_ibe_dn1 * (nv1 - nv2)) + var_ibe);
        let eq11_e167_d_n2: f64 = ((var_ibe_dn2 * (nv1 - nv2)) + (-var_ibe));
        let eq11_e167_d_n3: f64 = (var_ibe_dn3 * (nv1 - nv2));
        let eq11_e167_d_n4: f64 = (var_ibe_dn4 * (nv1 - nv2));
        let eq11_e167_d_n5: f64 = (var_ibe_dn5 * (nv1 - nv2));
        let eq11_e167_d_n6: f64 = (var_ibe_dn6 * (nv1 - nv2));
        let eq11_e167_d_n7: f64 = (var_ibe_dn7 * (nv1 - nv2));
        let eq11_e167_d_n8: f64 = (var_ibe_dn8 * (nv1 - nv2));
        let eq11_e167_d_n9: f64 = (var_ibe_dn9 * (nv1 - nv2));
        let eq11_e167_d_b0: f64 = (var_ibe_db0 * (nv1 - nv2));
        let eq11_e167_d_b1: f64 = (var_ibe_db1 * (nv1 - nv2));
        let eq11_e167_d_b2: f64 = (var_ibe_db2 * (nv1 - nv2));
        let eq11_e167_d_b3: f64 = (var_ibe_db3 * (nv1 - nv2));
        let eq11_e167_d_b4: f64 = (var_ibe_db4 * (nv1 - nv2));
        let eq11_e167_d_b5: f64 = (var_ibe_db5 * (nv1 - nv2));
        let eq11_e167_d_b6: f64 = (var_ibe_db6 * (nv1 - nv2));
        let eq11_e167_d_b7: f64 = (var_ibe_db7 * (nv1 - nv2));
        let eq11_e168: f64 = (eq11_e167).abs();
        let eq11_e168_d_n0: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n0 } else { (-eq11_e167_d_n0) };
        let eq11_e168_d_n1: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n1 } else { (-eq11_e167_d_n1) };
        let eq11_e168_d_n2: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n2 } else { (-eq11_e167_d_n2) };
        let eq11_e168_d_n3: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n3 } else { (-eq11_e167_d_n3) };
        let eq11_e168_d_n4: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n4 } else { (-eq11_e167_d_n4) };
        let eq11_e168_d_n5: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n5 } else { (-eq11_e167_d_n5) };
        let eq11_e168_d_n6: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n6 } else { (-eq11_e167_d_n6) };
        let eq11_e168_d_n7: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n7 } else { (-eq11_e167_d_n7) };
        let eq11_e168_d_n8: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n8 } else { (-eq11_e167_d_n8) };
        let eq11_e168_d_n9: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n9 } else { (-eq11_e167_d_n9) };
        let eq11_e168_d_b0: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b0 } else { (-eq11_e167_d_b0) };
        let eq11_e168_d_b1: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b1 } else { (-eq11_e167_d_b1) };
        let eq11_e168_d_b2: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b2 } else { (-eq11_e167_d_b2) };
        let eq11_e168_d_b3: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b3 } else { (-eq11_e167_d_b3) };
        let eq11_e168_d_b4: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b4 } else { (-eq11_e167_d_b4) };
        let eq11_e168_d_b5: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b5 } else { (-eq11_e167_d_b5) };
        let eq11_e168_d_b6: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b6 } else { (-eq11_e167_d_b6) };
        let eq11_e168_d_b7: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b7 } else { (-eq11_e167_d_b7) };
        let eq11_e169: f64 = (eq11_e164 * eq11_e168);
        let eq11_e169_d_n0: f64 = (eq11_e164 * eq11_e168_d_n0);
        let eq11_e169_d_n1: f64 = (eq11_e164 * eq11_e168_d_n1);
        let eq11_e169_d_n2: f64 = (eq11_e164 * eq11_e168_d_n2);
        let eq11_e169_d_n3: f64 = (eq11_e164 * eq11_e168_d_n3);
        let eq11_e169_d_n4: f64 = (eq11_e164 * eq11_e168_d_n4);
        let eq11_e169_d_n5: f64 = (eq11_e164 * eq11_e168_d_n5);
        let eq11_e169_d_n6: f64 = (eq11_e164 * eq11_e168_d_n6);
        let eq11_e169_d_n7: f64 = (eq11_e164 * eq11_e168_d_n7);
        let eq11_e169_d_n8: f64 = (eq11_e164 * eq11_e168_d_n8);
        let eq11_e169_d_n9: f64 = (eq11_e164 * eq11_e168_d_n9);
        let eq11_e169_d_b0: f64 = (eq11_e164 * eq11_e168_d_b0);
        let eq11_e169_d_b1: f64 = (eq11_e164 * eq11_e168_d_b1);
        let eq11_e169_d_b2: f64 = (eq11_e164 * eq11_e168_d_b2);
        let eq11_e169_d_b3: f64 = (eq11_e164 * eq11_e168_d_b3);
        let eq11_e169_d_b4: f64 = (eq11_e164 * eq11_e168_d_b4);
        let eq11_e169_d_b5: f64 = (eq11_e164 * eq11_e168_d_b5);
        let eq11_e169_d_b6: f64 = (eq11_e164 * eq11_e168_d_b6);
        let eq11_e169_d_b7: f64 = (eq11_e164 * eq11_e168_d_b7);
        let eq11_e172: f64 = (var_ibc * (nv1 - nv0));
        let eq11_e172_d_n0: f64 = ((var_ibc_dn0 * (nv1 - nv0)) + (-var_ibc));
        let eq11_e172_d_n1: f64 = ((var_ibc_dn1 * (nv1 - nv0)) + var_ibc);
        let eq11_e172_d_n2: f64 = (var_ibc_dn2 * (nv1 - nv0));
        let eq11_e172_d_n3: f64 = (var_ibc_dn3 * (nv1 - nv0));
        let eq11_e172_d_n4: f64 = (var_ibc_dn4 * (nv1 - nv0));
        let eq11_e172_d_n5: f64 = (var_ibc_dn5 * (nv1 - nv0));
        let eq11_e172_d_n6: f64 = (var_ibc_dn6 * (nv1 - nv0));
        let eq11_e172_d_n7: f64 = (var_ibc_dn7 * (nv1 - nv0));
        let eq11_e172_d_n8: f64 = (var_ibc_dn8 * (nv1 - nv0));
        let eq11_e172_d_n9: f64 = (var_ibc_dn9 * (nv1 - nv0));
        let eq11_e172_d_b0: f64 = (var_ibc_db0 * (nv1 - nv0));
        let eq11_e172_d_b1: f64 = (var_ibc_db1 * (nv1 - nv0));
        let eq11_e172_d_b2: f64 = (var_ibc_db2 * (nv1 - nv0));
        let eq11_e172_d_b3: f64 = (var_ibc_db3 * (nv1 - nv0));
        let eq11_e172_d_b4: f64 = (var_ibc_db4 * (nv1 - nv0));
        let eq11_e172_d_b5: f64 = (var_ibc_db5 * (nv1 - nv0));
        let eq11_e172_d_b6: f64 = (var_ibc_db6 * (nv1 - nv0));
        let eq11_e172_d_b7: f64 = (var_ibc_db7 * (nv1 - nv0));
        let eq11_e173: f64 = (eq11_e172).abs();
        let eq11_e173_d_n0: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n0 } else { (-eq11_e172_d_n0) };
        let eq11_e173_d_n1: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n1 } else { (-eq11_e172_d_n1) };
        let eq11_e173_d_n2: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n2 } else { (-eq11_e172_d_n2) };
        let eq11_e173_d_n3: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n3 } else { (-eq11_e172_d_n3) };
        let eq11_e173_d_n4: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n4 } else { (-eq11_e172_d_n4) };
        let eq11_e173_d_n5: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n5 } else { (-eq11_e172_d_n5) };
        let eq11_e173_d_n6: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n6 } else { (-eq11_e172_d_n6) };
        let eq11_e173_d_n7: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n7 } else { (-eq11_e172_d_n7) };
        let eq11_e173_d_n8: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n8 } else { (-eq11_e172_d_n8) };
        let eq11_e173_d_n9: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n9 } else { (-eq11_e172_d_n9) };
        let eq11_e173_d_b0: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b0 } else { (-eq11_e172_d_b0) };
        let eq11_e173_d_b1: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b1 } else { (-eq11_e172_d_b1) };
        let eq11_e173_d_b2: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b2 } else { (-eq11_e172_d_b2) };
        let eq11_e173_d_b3: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b3 } else { (-eq11_e172_d_b3) };
        let eq11_e173_d_b4: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b4 } else { (-eq11_e172_d_b4) };
        let eq11_e173_d_b5: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b5 } else { (-eq11_e172_d_b5) };
        let eq11_e173_d_b6: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b6 } else { (-eq11_e172_d_b6) };
        let eq11_e173_d_b7: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b7 } else { (-eq11_e172_d_b7) };
        let eq11_e174: f64 = (eq11_e169 - eq11_e173);
        let eq11_e174_d_n0: f64 = (eq11_e169_d_n0 - eq11_e173_d_n0);
        let eq11_e174_d_n1: f64 = (eq11_e169_d_n1 - eq11_e173_d_n1);
        let eq11_e174_d_n2: f64 = (eq11_e169_d_n2 - eq11_e173_d_n2);
        let eq11_e174_d_n3: f64 = (eq11_e169_d_n3 - eq11_e173_d_n3);
        let eq11_e174_d_n4: f64 = (eq11_e169_d_n4 - eq11_e173_d_n4);
        let eq11_e174_d_n5: f64 = (eq11_e169_d_n5 - eq11_e173_d_n5);
        let eq11_e174_d_n6: f64 = (eq11_e169_d_n6 - eq11_e173_d_n6);
        let eq11_e174_d_n7: f64 = (eq11_e169_d_n7 - eq11_e173_d_n7);
        let eq11_e174_d_n8: f64 = (eq11_e169_d_n8 - eq11_e173_d_n8);
        let eq11_e174_d_n9: f64 = (eq11_e169_d_n9 - eq11_e173_d_n9);
        let eq11_e174_d_b0: f64 = (eq11_e169_d_b0 - eq11_e173_d_b0);
        let eq11_e174_d_b1: f64 = (eq11_e169_d_b1 - eq11_e173_d_b1);
        let eq11_e174_d_b2: f64 = (eq11_e169_d_b2 - eq11_e173_d_b2);
        let eq11_e174_d_b3: f64 = (eq11_e169_d_b3 - eq11_e173_d_b3);
        let eq11_e174_d_b4: f64 = (eq11_e169_d_b4 - eq11_e173_d_b4);
        let eq11_e174_d_b5: f64 = (eq11_e169_d_b5 - eq11_e173_d_b5);
        let eq11_e174_d_b6: f64 = (eq11_e169_d_b6 - eq11_e173_d_b6);
        let eq11_e174_d_b7: f64 = (eq11_e169_d_b7 - eq11_e173_d_b7);
        (eq11_e174, eq11_e174_d_n0, eq11_e174_d_n1, eq11_e174_d_n2, eq11_e174_d_n3, eq11_e174_d_n4, eq11_e174_d_n5, eq11_e174_d_n6, eq11_e174_d_n7, eq11_e174_d_n8, eq11_e174_d_n9, eq11_e174_d_b0, eq11_e174_d_b1, eq11_e174_d_b2, eq11_e174_d_b3, eq11_e174_d_b4, eq11_e174_d_b5, eq11_e174_d_b6, eq11_e174_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e176;
        let eq11_node_derivatives: [f64; 10] = [eq11_e176_d_n0, eq11_e176_d_n1, eq11_e176_d_n2, eq11_e176_d_n3, eq11_e176_d_n4, eq11_e176_d_n5, eq11_e176_d_n6, eq11_e176_d_n7, eq11_e176_d_n8, eq11_e176_d_n9];
        let eq11_branch_derivatives: [f64; 8] = [eq11_e176_d_b0, eq11_e176_d_b1, eq11_e176_d_b2, eq11_e176_d_b3, eq11_e176_d_b4, eq11_e176_d_b5, eq11_e176_d_b6, eq11_e176_d_b7];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq12_e185, eq12_e185_d_n3, eq12_e185_d_n7,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let __rspice_inv_cse_2: f64 = 1.0 / p.p33;
        let eq12_e183: f64 = ((nv3 - nv7) * __rspice_inv_cse_2);
        let eq12_e183_d_n3: f64 = (1.0 * __rspice_inv_cse_2);
        let eq12_e183_d_n7: f64 = ((-1.0) * __rspice_inv_cse_2);
        (eq12_e183, eq12_e183_d_n3, eq12_e183_d_n7,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e185;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * (eq12_value),
            3,
            multiplicity * (eq12_e185_d_n3),
            7,
            multiplicity * (eq12_e185_d_n7),
        );
        let (eq13_e195, eq13_e195_d_n3,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq13_e192: f64 = (p.p34 * (nv3 - 0.0));
        let eq13_e193: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq13_e192);
        (eq13_e193, (p.p34 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e195;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq13_value),
            3,
            multiplicity * (eq13_e195_d_n3),
        );
        let (eq14_e204, eq14_e204_d_n7,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let __rspice_inv_cse_3: f64 = 1.0 / p.p35;
        let eq14_e202: f64 = ((nv7 - 0.0) * __rspice_inv_cse_3);
        let eq14_e202_d_n7: f64 = (1.0 * __rspice_inv_cse_3);
        (eq14_e202, eq14_e202_d_n7,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e204;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq14_value),
            7,
            multiplicity * (eq14_e204_d_n7),
        );
        let (eq15_e214, eq15_e214_d_n7,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq15_e211: f64 = (p.p36 * (nv7 - 0.0));
        let eq15_e212: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq15_e211);
        (eq15_e212, (p.p36 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e214;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq15_value),
            7,
            multiplicity * (eq15_e214_d_n7),
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        var_guard20: f64,
        var_guard21: f64,
        var_guard22: f64,
        var_guard23: f64,
        var_guard24: f64,
        var_guard25: f64,
        var_ibc: f64,
        var_ibc_db0: f64,
        var_ibc_db1: f64,
        var_ibc_db2: f64,
        var_ibc_db3: f64,
        var_ibc_db4: f64,
        var_ibc_db5: f64,
        var_ibc_db6: f64,
        var_ibc_db7: f64,
        var_ibc_dn0: f64,
        var_ibc_dn1: f64,
        var_ibc_dn2: f64,
        var_ibc_dn3: f64,
        var_ibc_dn4: f64,
        var_ibc_dn5: f64,
        var_ibc_dn6: f64,
        var_ibc_dn7: f64,
        var_ibc_dn8: f64,
        var_ibc_dn9: f64,
        var_ibe: f64,
        var_ibe_db0: f64,
        var_ibe_db1: f64,
        var_ibe_db2: f64,
        var_ibe_db3: f64,
        var_ibe_db4: f64,
        var_ibe_db5: f64,
        var_ibe_db6: f64,
        var_ibe_db7: f64,
        var_ibe_dn0: f64,
        var_ibe_dn1: f64,
        var_ibe_dn2: f64,
        var_ibe_dn3: f64,
        var_ibe_dn4: f64,
        var_ibe_dn5: f64,
        var_ibe_dn6: f64,
        var_ibe_dn7: f64,
        var_ibe_dn8: f64,
        var_ibe_dn9: f64,
        var_rb: f64,
        var_rb_db0: f64,
        var_rb_db1: f64,
        var_rb_db2: f64,
        var_rb_db3: f64,
        var_rb_db4: f64,
        var_rb_db5: f64,
        var_rb_db6: f64,
        var_rb_db7: f64,
        var_rb_dn0: f64,
        var_rb_dn1: f64,
        var_rb_dn2: f64,
        var_rb_dn3: f64,
        var_rb_dn4: f64,
        var_rb_dn5: f64,
        var_rb_dn6: f64,
        var_rb_dn7: f64,
        var_rb_dn8: f64,
        var_rb_dn9: f64,
        var_rc: f64,
        var_rc_db0: f64,
        var_rc_db1: f64,
        var_rc_db2: f64,
        var_rc_db3: f64,
        var_rc_db4: f64,
        var_rc_db5: f64,
        var_rc_db6: f64,
        var_rc_db7: f64,
        var_rc_dn0: f64,
        var_rc_dn1: f64,
        var_rc_dn2: f64,
        var_rc_dn3: f64,
        var_rc_dn4: f64,
        var_rc_dn5: f64,
        var_rc_dn6: f64,
        var_rc_dn7: f64,
        var_rc_dn8: f64,
        var_rc_dn9: f64,
        var_re: f64,
        var_re_db0: f64,
        var_re_db1: f64,
        var_re_db2: f64,
        var_re_db3: f64,
        var_re_db4: f64,
        var_re_db5: f64,
        var_re_db6: f64,
        var_re_db7: f64,
        var_re_dn0: f64,
        var_re_dn1: f64,
        var_re_dn2: f64,
        var_re_dn3: f64,
        var_re_dn4: f64,
        var_re_dn5: f64,
        var_re_dn6: f64,
        var_re_dn7: f64,
        var_re_dn8: f64,
        var_re_dn9: f64,
        var_ttype: f64,
        var_weff: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq16_e235, eq16_e235_d_n0, eq16_e235_d_n1, eq16_e235_d_n2, eq16_e235_d_n3, eq16_e235_d_n4, eq16_e235_d_n5, eq16_e235_d_n6, eq16_e235_d_n7, eq16_e235_d_n8, eq16_e235_d_n9, eq16_e235_d_b0, eq16_e235_d_b1, eq16_e235_d_b2, eq16_e235_d_b3, eq16_e235_d_b4, eq16_e235_d_b5, eq16_e235_d_b6, eq16_e235_d_b7,) = {
    if (((var_guard20 == 0.0) && (var_guard21 == 0.0)) && (var_guard22 != 0.0)) {
        let eq16_e223: f64 = (-1.0);
        let eq16_e226: f64 = (var_ibe * (nv1 - nv2));
        let eq16_e226_d_n0: f64 = (var_ibe_dn0 * (nv1 - nv2));
        let eq16_e226_d_n1: f64 = ((var_ibe_dn1 * (nv1 - nv2)) + var_ibe);
        let eq16_e226_d_n2: f64 = ((var_ibe_dn2 * (nv1 - nv2)) + (-var_ibe));
        let eq16_e226_d_n3: f64 = (var_ibe_dn3 * (nv1 - nv2));
        let eq16_e226_d_n4: f64 = (var_ibe_dn4 * (nv1 - nv2));
        let eq16_e226_d_n5: f64 = (var_ibe_dn5 * (nv1 - nv2));
        let eq16_e226_d_n6: f64 = (var_ibe_dn6 * (nv1 - nv2));
        let eq16_e226_d_n7: f64 = (var_ibe_dn7 * (nv1 - nv2));
        let eq16_e226_d_n8: f64 = (var_ibe_dn8 * (nv1 - nv2));
        let eq16_e226_d_n9: f64 = (var_ibe_dn9 * (nv1 - nv2));
        let eq16_e226_d_b0: f64 = (var_ibe_db0 * (nv1 - nv2));
        let eq16_e226_d_b1: f64 = (var_ibe_db1 * (nv1 - nv2));
        let eq16_e226_d_b2: f64 = (var_ibe_db2 * (nv1 - nv2));
        let eq16_e226_d_b3: f64 = (var_ibe_db3 * (nv1 - nv2));
        let eq16_e226_d_b4: f64 = (var_ibe_db4 * (nv1 - nv2));
        let eq16_e226_d_b5: f64 = (var_ibe_db5 * (nv1 - nv2));
        let eq16_e226_d_b6: f64 = (var_ibe_db6 * (nv1 - nv2));
        let eq16_e226_d_b7: f64 = (var_ibe_db7 * (nv1 - nv2));
        let eq16_e227: f64 = (eq16_e226).abs();
        let eq16_e227_d_n0: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n0 } else { (-eq16_e226_d_n0) };
        let eq16_e227_d_n1: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n1 } else { (-eq16_e226_d_n1) };
        let eq16_e227_d_n2: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n2 } else { (-eq16_e226_d_n2) };
        let eq16_e227_d_n3: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n3 } else { (-eq16_e226_d_n3) };
        let eq16_e227_d_n4: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n4 } else { (-eq16_e226_d_n4) };
        let eq16_e227_d_n5: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n5 } else { (-eq16_e226_d_n5) };
        let eq16_e227_d_n6: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n6 } else { (-eq16_e226_d_n6) };
        let eq16_e227_d_n7: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n7 } else { (-eq16_e226_d_n7) };
        let eq16_e227_d_n8: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n8 } else { (-eq16_e226_d_n8) };
        let eq16_e227_d_n9: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n9 } else { (-eq16_e226_d_n9) };
        let eq16_e227_d_b0: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b0 } else { (-eq16_e226_d_b0) };
        let eq16_e227_d_b1: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b1 } else { (-eq16_e226_d_b1) };
        let eq16_e227_d_b2: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b2 } else { (-eq16_e226_d_b2) };
        let eq16_e227_d_b3: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b3 } else { (-eq16_e226_d_b3) };
        let eq16_e227_d_b4: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b4 } else { (-eq16_e226_d_b4) };
        let eq16_e227_d_b5: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b5 } else { (-eq16_e226_d_b5) };
        let eq16_e227_d_b6: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b6 } else { (-eq16_e226_d_b6) };
        let eq16_e227_d_b7: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b7 } else { (-eq16_e226_d_b7) };
        let eq16_e228: f64 = (eq16_e223 * eq16_e227);
        let eq16_e228_d_n0: f64 = (eq16_e223 * eq16_e227_d_n0);
        let eq16_e228_d_n1: f64 = (eq16_e223 * eq16_e227_d_n1);
        let eq16_e228_d_n2: f64 = (eq16_e223 * eq16_e227_d_n2);
        let eq16_e228_d_n3: f64 = (eq16_e223 * eq16_e227_d_n3);
        let eq16_e228_d_n4: f64 = (eq16_e223 * eq16_e227_d_n4);
        let eq16_e228_d_n5: f64 = (eq16_e223 * eq16_e227_d_n5);
        let eq16_e228_d_n6: f64 = (eq16_e223 * eq16_e227_d_n6);
        let eq16_e228_d_n7: f64 = (eq16_e223 * eq16_e227_d_n7);
        let eq16_e228_d_n8: f64 = (eq16_e223 * eq16_e227_d_n8);
        let eq16_e228_d_n9: f64 = (eq16_e223 * eq16_e227_d_n9);
        let eq16_e228_d_b0: f64 = (eq16_e223 * eq16_e227_d_b0);
        let eq16_e228_d_b1: f64 = (eq16_e223 * eq16_e227_d_b1);
        let eq16_e228_d_b2: f64 = (eq16_e223 * eq16_e227_d_b2);
        let eq16_e228_d_b3: f64 = (eq16_e223 * eq16_e227_d_b3);
        let eq16_e228_d_b4: f64 = (eq16_e223 * eq16_e227_d_b4);
        let eq16_e228_d_b5: f64 = (eq16_e223 * eq16_e227_d_b5);
        let eq16_e228_d_b6: f64 = (eq16_e223 * eq16_e227_d_b6);
        let eq16_e228_d_b7: f64 = (eq16_e223 * eq16_e227_d_b7);
        let eq16_e231: f64 = (var_ibc * (nv1 - nv0));
        let eq16_e231_d_n0: f64 = ((var_ibc_dn0 * (nv1 - nv0)) + (-var_ibc));
        let eq16_e231_d_n1: f64 = ((var_ibc_dn1 * (nv1 - nv0)) + var_ibc);
        let eq16_e231_d_n2: f64 = (var_ibc_dn2 * (nv1 - nv0));
        let eq16_e231_d_n3: f64 = (var_ibc_dn3 * (nv1 - nv0));
        let eq16_e231_d_n4: f64 = (var_ibc_dn4 * (nv1 - nv0));
        let eq16_e231_d_n5: f64 = (var_ibc_dn5 * (nv1 - nv0));
        let eq16_e231_d_n6: f64 = (var_ibc_dn6 * (nv1 - nv0));
        let eq16_e231_d_n7: f64 = (var_ibc_dn7 * (nv1 - nv0));
        let eq16_e231_d_n8: f64 = (var_ibc_dn8 * (nv1 - nv0));
        let eq16_e231_d_n9: f64 = (var_ibc_dn9 * (nv1 - nv0));
        let eq16_e231_d_b0: f64 = (var_ibc_db0 * (nv1 - nv0));
        let eq16_e231_d_b1: f64 = (var_ibc_db1 * (nv1 - nv0));
        let eq16_e231_d_b2: f64 = (var_ibc_db2 * (nv1 - nv0));
        let eq16_e231_d_b3: f64 = (var_ibc_db3 * (nv1 - nv0));
        let eq16_e231_d_b4: f64 = (var_ibc_db4 * (nv1 - nv0));
        let eq16_e231_d_b5: f64 = (var_ibc_db5 * (nv1 - nv0));
        let eq16_e231_d_b6: f64 = (var_ibc_db6 * (nv1 - nv0));
        let eq16_e231_d_b7: f64 = (var_ibc_db7 * (nv1 - nv0));
        let eq16_e232: f64 = (eq16_e231).abs();
        let eq16_e232_d_n0: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n0 } else { (-eq16_e231_d_n0) };
        let eq16_e232_d_n1: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n1 } else { (-eq16_e231_d_n1) };
        let eq16_e232_d_n2: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n2 } else { (-eq16_e231_d_n2) };
        let eq16_e232_d_n3: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n3 } else { (-eq16_e231_d_n3) };
        let eq16_e232_d_n4: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n4 } else { (-eq16_e231_d_n4) };
        let eq16_e232_d_n5: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n5 } else { (-eq16_e231_d_n5) };
        let eq16_e232_d_n6: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n6 } else { (-eq16_e231_d_n6) };
        let eq16_e232_d_n7: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n7 } else { (-eq16_e231_d_n7) };
        let eq16_e232_d_n8: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n8 } else { (-eq16_e231_d_n8) };
        let eq16_e232_d_n9: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n9 } else { (-eq16_e231_d_n9) };
        let eq16_e232_d_b0: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b0 } else { (-eq16_e231_d_b0) };
        let eq16_e232_d_b1: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b1 } else { (-eq16_e231_d_b1) };
        let eq16_e232_d_b2: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b2 } else { (-eq16_e231_d_b2) };
        let eq16_e232_d_b3: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b3 } else { (-eq16_e231_d_b3) };
        let eq16_e232_d_b4: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b4 } else { (-eq16_e231_d_b4) };
        let eq16_e232_d_b5: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b5 } else { (-eq16_e231_d_b5) };
        let eq16_e232_d_b6: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b6 } else { (-eq16_e231_d_b6) };
        let eq16_e232_d_b7: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b7 } else { (-eq16_e231_d_b7) };
        let eq16_e233: f64 = (eq16_e228 - eq16_e232);
        let eq16_e233_d_n0: f64 = (eq16_e228_d_n0 - eq16_e232_d_n0);
        let eq16_e233_d_n1: f64 = (eq16_e228_d_n1 - eq16_e232_d_n1);
        let eq16_e233_d_n2: f64 = (eq16_e228_d_n2 - eq16_e232_d_n2);
        let eq16_e233_d_n3: f64 = (eq16_e228_d_n3 - eq16_e232_d_n3);
        let eq16_e233_d_n4: f64 = (eq16_e228_d_n4 - eq16_e232_d_n4);
        let eq16_e233_d_n5: f64 = (eq16_e228_d_n5 - eq16_e232_d_n5);
        let eq16_e233_d_n6: f64 = (eq16_e228_d_n6 - eq16_e232_d_n6);
        let eq16_e233_d_n7: f64 = (eq16_e228_d_n7 - eq16_e232_d_n7);
        let eq16_e233_d_n8: f64 = (eq16_e228_d_n8 - eq16_e232_d_n8);
        let eq16_e233_d_n9: f64 = (eq16_e228_d_n9 - eq16_e232_d_n9);
        let eq16_e233_d_b0: f64 = (eq16_e228_d_b0 - eq16_e232_d_b0);
        let eq16_e233_d_b1: f64 = (eq16_e228_d_b1 - eq16_e232_d_b1);
        let eq16_e233_d_b2: f64 = (eq16_e228_d_b2 - eq16_e232_d_b2);
        let eq16_e233_d_b3: f64 = (eq16_e228_d_b3 - eq16_e232_d_b3);
        let eq16_e233_d_b4: f64 = (eq16_e228_d_b4 - eq16_e232_d_b4);
        let eq16_e233_d_b5: f64 = (eq16_e228_d_b5 - eq16_e232_d_b5);
        let eq16_e233_d_b6: f64 = (eq16_e228_d_b6 - eq16_e232_d_b6);
        let eq16_e233_d_b7: f64 = (eq16_e228_d_b7 - eq16_e232_d_b7);
        (eq16_e233, eq16_e233_d_n0, eq16_e233_d_n1, eq16_e233_d_n2, eq16_e233_d_n3, eq16_e233_d_n4, eq16_e233_d_n5, eq16_e233_d_n6, eq16_e233_d_n7, eq16_e233_d_n8, eq16_e233_d_n9, eq16_e233_d_b0, eq16_e233_d_b1, eq16_e233_d_b2, eq16_e233_d_b3, eq16_e233_d_b4, eq16_e233_d_b5, eq16_e233_d_b6, eq16_e233_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e235;
        let eq16_node_derivatives: [f64; 10] = [eq16_e235_d_n0, eq16_e235_d_n1, eq16_e235_d_n2, eq16_e235_d_n3, eq16_e235_d_n4, eq16_e235_d_n5, eq16_e235_d_n6, eq16_e235_d_n7, eq16_e235_d_n8, eq16_e235_d_n9];
        let eq16_branch_derivatives: [f64; 8] = [eq16_e235_d_b0, eq16_e235_d_b1, eq16_e235_d_b2, eq16_e235_d_b3, eq16_e235_d_b4, eq16_e235_d_b5, eq16_e235_d_b6, eq16_e235_d_b7];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let (eq23_e297, eq23_e297_d_n0, eq23_e297_d_n1, eq23_e297_d_n2, eq23_e297_d_n3, eq23_e297_d_n4, eq23_e297_d_n5, eq23_e297_d_n6, eq23_e297_d_n7, eq23_e297_d_n8, eq23_e297_d_n9, eq23_e297_d_b0, eq23_e297_d_b1, eq23_e297_d_b2, eq23_e297_d_b3, eq23_e297_d_b4, eq23_e297_d_b5, eq23_e297_d_b6, eq23_e297_d_b7,) = {
    if (var_guard23 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / var_weff;
        let eq23_e287: f64 = (var_rb * __rspice_inv_cse_0);
        let eq23_e287_d_n0: f64 = (var_rb_dn0 * __rspice_inv_cse_0);
        let eq23_e287_d_n1: f64 = (var_rb_dn1 * __rspice_inv_cse_0);
        let eq23_e287_d_n2: f64 = (var_rb_dn2 * __rspice_inv_cse_0);
        let eq23_e287_d_n3: f64 = (var_rb_dn3 * __rspice_inv_cse_0);
        let eq23_e287_d_n4: f64 = (var_rb_dn4 * __rspice_inv_cse_0);
        let eq23_e287_d_n5: f64 = (var_rb_dn5 * __rspice_inv_cse_0);
        let eq23_e287_d_n6: f64 = (var_rb_dn6 * __rspice_inv_cse_0);
        let eq23_e287_d_n7: f64 = (var_rb_dn7 * __rspice_inv_cse_0);
        let eq23_e287_d_n8: f64 = (var_rb_dn8 * __rspice_inv_cse_0);
        let eq23_e287_d_n9: f64 = (var_rb_dn9 * __rspice_inv_cse_0);
        let eq23_e287_d_b0: f64 = (var_rb_db0 * __rspice_inv_cse_0);
        let eq23_e287_d_b1: f64 = (var_rb_db1 * __rspice_inv_cse_0);
        let eq23_e287_d_b2: f64 = (var_rb_db2 * __rspice_inv_cse_0);
        let eq23_e287_d_b3: f64 = (var_rb_db3 * __rspice_inv_cse_0);
        let eq23_e287_d_b4: f64 = (var_rb_db4 * __rspice_inv_cse_0);
        let eq23_e287_d_b5: f64 = (var_rb_db5 * __rspice_inv_cse_0);
        let eq23_e287_d_b6: f64 = (var_rb_db6 * __rspice_inv_cse_0);
        let eq23_e287_d_b7: f64 = (var_rb_db7 * __rspice_inv_cse_0);
        let (eq23_e294, eq23_e294_d_n0, eq23_e294_d_n1, eq23_e294_d_n2, eq23_e294_d_n3, eq23_e294_d_n4, eq23_e294_d_n5, eq23_e294_d_n6, eq23_e294_d_n7, eq23_e294_d_n8, eq23_e294_d_n9, eq23_e294_d_b0, eq23_e294_d_b1, eq23_e294_d_b2, eq23_e294_d_b3, eq23_e294_d_b4, eq23_e294_d_b5, eq23_e294_d_b6, eq23_e294_d_b7,) = {
            if (eq23_e287 > p.p46) {
                let __rspice_inv_cse_1: f64 = 1.0 / var_weff;
                let eq23_e292: f64 = (var_rb * __rspice_inv_cse_1);
                let eq23_e292_d_n0: f64 = (var_rb_dn0 * __rspice_inv_cse_1);
                let eq23_e292_d_n1: f64 = (var_rb_dn1 * __rspice_inv_cse_1);
                let eq23_e292_d_n2: f64 = (var_rb_dn2 * __rspice_inv_cse_1);
                let eq23_e292_d_n3: f64 = (var_rb_dn3 * __rspice_inv_cse_1);
                let eq23_e292_d_n4: f64 = (var_rb_dn4 * __rspice_inv_cse_1);
                let eq23_e292_d_n5: f64 = (var_rb_dn5 * __rspice_inv_cse_1);
                let eq23_e292_d_n6: f64 = (var_rb_dn6 * __rspice_inv_cse_1);
                let eq23_e292_d_n7: f64 = (var_rb_dn7 * __rspice_inv_cse_1);
                let eq23_e292_d_n8: f64 = (var_rb_dn8 * __rspice_inv_cse_1);
                let eq23_e292_d_n9: f64 = (var_rb_dn9 * __rspice_inv_cse_1);
                let eq23_e292_d_b0: f64 = (var_rb_db0 * __rspice_inv_cse_1);
                let eq23_e292_d_b1: f64 = (var_rb_db1 * __rspice_inv_cse_1);
                let eq23_e292_d_b2: f64 = (var_rb_db2 * __rspice_inv_cse_1);
                let eq23_e292_d_b3: f64 = (var_rb_db3 * __rspice_inv_cse_1);
                let eq23_e292_d_b4: f64 = (var_rb_db4 * __rspice_inv_cse_1);
                let eq23_e292_d_b5: f64 = (var_rb_db5 * __rspice_inv_cse_1);
                let eq23_e292_d_b6: f64 = (var_rb_db6 * __rspice_inv_cse_1);
                let eq23_e292_d_b7: f64 = (var_rb_db7 * __rspice_inv_cse_1);
                (eq23_e292, eq23_e292_d_n0, eq23_e292_d_n1, eq23_e292_d_n2, eq23_e292_d_n3, eq23_e292_d_n4, eq23_e292_d_n5, eq23_e292_d_n6, eq23_e292_d_n7, eq23_e292_d_n8, eq23_e292_d_n9, eq23_e292_d_b0, eq23_e292_d_b1, eq23_e292_d_b2, eq23_e292_d_b3, eq23_e292_d_b4, eq23_e292_d_b5, eq23_e292_d_b6, eq23_e292_d_b7,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq23_e295: f64 = ((nv1 - nv5) / eq23_e294);
        let eq23_e295_d_n0: f64 = (-(((nv1 - nv5) * eq23_e294_d_n0) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n1: f64 = ((eq23_e294 - ((nv1 - nv5) * eq23_e294_d_n1)) / (eq23_e294 * eq23_e294));
        let eq23_e295_d_n2: f64 = (-(((nv1 - nv5) * eq23_e294_d_n2) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n3: f64 = (-(((nv1 - nv5) * eq23_e294_d_n3) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n4: f64 = (-(((nv1 - nv5) * eq23_e294_d_n4) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n5: f64 = (((-eq23_e294) - ((nv1 - nv5) * eq23_e294_d_n5)) / (eq23_e294 * eq23_e294));
        let eq23_e295_d_n6: f64 = (-(((nv1 - nv5) * eq23_e294_d_n6) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n7: f64 = (-(((nv1 - nv5) * eq23_e294_d_n7) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n8: f64 = (-(((nv1 - nv5) * eq23_e294_d_n8) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n9: f64 = (-(((nv1 - nv5) * eq23_e294_d_n9) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b0: f64 = (-(((nv1 - nv5) * eq23_e294_d_b0) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b1: f64 = (-(((nv1 - nv5) * eq23_e294_d_b1) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b2: f64 = (-(((nv1 - nv5) * eq23_e294_d_b2) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b3: f64 = (-(((nv1 - nv5) * eq23_e294_d_b3) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b4: f64 = (-(((nv1 - nv5) * eq23_e294_d_b4) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b5: f64 = (-(((nv1 - nv5) * eq23_e294_d_b5) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b6: f64 = (-(((nv1 - nv5) * eq23_e294_d_b6) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b7: f64 = (-(((nv1 - nv5) * eq23_e294_d_b7) / (eq23_e294 * eq23_e294)));
        (eq23_e295, eq23_e295_d_n0, eq23_e295_d_n1, eq23_e295_d_n2, eq23_e295_d_n3, eq23_e295_d_n4, eq23_e295_d_n5, eq23_e295_d_n6, eq23_e295_d_n7, eq23_e295_d_n8, eq23_e295_d_n9, eq23_e295_d_b0, eq23_e295_d_b1, eq23_e295_d_b2, eq23_e295_d_b3, eq23_e295_d_b4, eq23_e295_d_b5, eq23_e295_d_b6, eq23_e295_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e297;
        let eq23_node_derivatives: [f64; 10] = [eq23_e297_d_n0, eq23_e297_d_n1, eq23_e297_d_n2, eq23_e297_d_n3, eq23_e297_d_n4, eq23_e297_d_n5, eq23_e297_d_n6, eq23_e297_d_n7, eq23_e297_d_n8, eq23_e297_d_n9];
        let eq23_branch_derivatives: [f64; 8] = [eq23_e297_d_b0, eq23_e297_d_b1, eq23_e297_d_b2, eq23_e297_d_b3, eq23_e297_d_b4, eq23_e297_d_b5, eq23_e297_d_b6, eq23_e297_d_b7];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq26_e323, eq26_e323_d_n0, eq26_e323_d_n1, eq26_e323_d_n2, eq26_e323_d_n3, eq26_e323_d_n4, eq26_e323_d_n5, eq26_e323_d_n6, eq26_e323_d_n7, eq26_e323_d_n8, eq26_e323_d_n9, eq26_e323_d_b0, eq26_e323_d_b1, eq26_e323_d_b2, eq26_e323_d_b3, eq26_e323_d_b4, eq26_e323_d_b5, eq26_e323_d_b6, eq26_e323_d_b7,) = {
    if (var_guard24 != 0.0) {
        let __rspice_inv_cse_2: f64 = 1.0 / var_weff;
        let eq26_e313: f64 = (var_re * __rspice_inv_cse_2);
        let eq26_e313_d_n0: f64 = (var_re_dn0 * __rspice_inv_cse_2);
        let eq26_e313_d_n1: f64 = (var_re_dn1 * __rspice_inv_cse_2);
        let eq26_e313_d_n2: f64 = (var_re_dn2 * __rspice_inv_cse_2);
        let eq26_e313_d_n3: f64 = (var_re_dn3 * __rspice_inv_cse_2);
        let eq26_e313_d_n4: f64 = (var_re_dn4 * __rspice_inv_cse_2);
        let eq26_e313_d_n5: f64 = (var_re_dn5 * __rspice_inv_cse_2);
        let eq26_e313_d_n6: f64 = (var_re_dn6 * __rspice_inv_cse_2);
        let eq26_e313_d_n7: f64 = (var_re_dn7 * __rspice_inv_cse_2);
        let eq26_e313_d_n8: f64 = (var_re_dn8 * __rspice_inv_cse_2);
        let eq26_e313_d_n9: f64 = (var_re_dn9 * __rspice_inv_cse_2);
        let eq26_e313_d_b0: f64 = (var_re_db0 * __rspice_inv_cse_2);
        let eq26_e313_d_b1: f64 = (var_re_db1 * __rspice_inv_cse_2);
        let eq26_e313_d_b2: f64 = (var_re_db2 * __rspice_inv_cse_2);
        let eq26_e313_d_b3: f64 = (var_re_db3 * __rspice_inv_cse_2);
        let eq26_e313_d_b4: f64 = (var_re_db4 * __rspice_inv_cse_2);
        let eq26_e313_d_b5: f64 = (var_re_db5 * __rspice_inv_cse_2);
        let eq26_e313_d_b6: f64 = (var_re_db6 * __rspice_inv_cse_2);
        let eq26_e313_d_b7: f64 = (var_re_db7 * __rspice_inv_cse_2);
        let (eq26_e320, eq26_e320_d_n0, eq26_e320_d_n1, eq26_e320_d_n2, eq26_e320_d_n3, eq26_e320_d_n4, eq26_e320_d_n5, eq26_e320_d_n6, eq26_e320_d_n7, eq26_e320_d_n8, eq26_e320_d_n9, eq26_e320_d_b0, eq26_e320_d_b1, eq26_e320_d_b2, eq26_e320_d_b3, eq26_e320_d_b4, eq26_e320_d_b5, eq26_e320_d_b6, eq26_e320_d_b7,) = {
            if (eq26_e313 > p.p46) {
                let __rspice_inv_cse_3: f64 = 1.0 / var_weff;
                let eq26_e318: f64 = (var_re * __rspice_inv_cse_3);
                let eq26_e318_d_n0: f64 = (var_re_dn0 * __rspice_inv_cse_3);
                let eq26_e318_d_n1: f64 = (var_re_dn1 * __rspice_inv_cse_3);
                let eq26_e318_d_n2: f64 = (var_re_dn2 * __rspice_inv_cse_3);
                let eq26_e318_d_n3: f64 = (var_re_dn3 * __rspice_inv_cse_3);
                let eq26_e318_d_n4: f64 = (var_re_dn4 * __rspice_inv_cse_3);
                let eq26_e318_d_n5: f64 = (var_re_dn5 * __rspice_inv_cse_3);
                let eq26_e318_d_n6: f64 = (var_re_dn6 * __rspice_inv_cse_3);
                let eq26_e318_d_n7: f64 = (var_re_dn7 * __rspice_inv_cse_3);
                let eq26_e318_d_n8: f64 = (var_re_dn8 * __rspice_inv_cse_3);
                let eq26_e318_d_n9: f64 = (var_re_dn9 * __rspice_inv_cse_3);
                let eq26_e318_d_b0: f64 = (var_re_db0 * __rspice_inv_cse_3);
                let eq26_e318_d_b1: f64 = (var_re_db1 * __rspice_inv_cse_3);
                let eq26_e318_d_b2: f64 = (var_re_db2 * __rspice_inv_cse_3);
                let eq26_e318_d_b3: f64 = (var_re_db3 * __rspice_inv_cse_3);
                let eq26_e318_d_b4: f64 = (var_re_db4 * __rspice_inv_cse_3);
                let eq26_e318_d_b5: f64 = (var_re_db5 * __rspice_inv_cse_3);
                let eq26_e318_d_b6: f64 = (var_re_db6 * __rspice_inv_cse_3);
                let eq26_e318_d_b7: f64 = (var_re_db7 * __rspice_inv_cse_3);
                (eq26_e318, eq26_e318_d_n0, eq26_e318_d_n1, eq26_e318_d_n2, eq26_e318_d_n3, eq26_e318_d_n4, eq26_e318_d_n5, eq26_e318_d_n6, eq26_e318_d_n7, eq26_e318_d_n8, eq26_e318_d_n9, eq26_e318_d_b0, eq26_e318_d_b1, eq26_e318_d_b2, eq26_e318_d_b3, eq26_e318_d_b4, eq26_e318_d_b5, eq26_e318_d_b6, eq26_e318_d_b7,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq26_e321: f64 = ((nv2 - nv6) / eq26_e320);
        let eq26_e321_d_n0: f64 = (-(((nv2 - nv6) * eq26_e320_d_n0) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n1: f64 = (-(((nv2 - nv6) * eq26_e320_d_n1) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n2: f64 = ((eq26_e320 - ((nv2 - nv6) * eq26_e320_d_n2)) / (eq26_e320 * eq26_e320));
        let eq26_e321_d_n3: f64 = (-(((nv2 - nv6) * eq26_e320_d_n3) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n4: f64 = (-(((nv2 - nv6) * eq26_e320_d_n4) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n5: f64 = (-(((nv2 - nv6) * eq26_e320_d_n5) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n6: f64 = (((-eq26_e320) - ((nv2 - nv6) * eq26_e320_d_n6)) / (eq26_e320 * eq26_e320));
        let eq26_e321_d_n7: f64 = (-(((nv2 - nv6) * eq26_e320_d_n7) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n8: f64 = (-(((nv2 - nv6) * eq26_e320_d_n8) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n9: f64 = (-(((nv2 - nv6) * eq26_e320_d_n9) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b0: f64 = (-(((nv2 - nv6) * eq26_e320_d_b0) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b1: f64 = (-(((nv2 - nv6) * eq26_e320_d_b1) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b2: f64 = (-(((nv2 - nv6) * eq26_e320_d_b2) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b3: f64 = (-(((nv2 - nv6) * eq26_e320_d_b3) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b4: f64 = (-(((nv2 - nv6) * eq26_e320_d_b4) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b5: f64 = (-(((nv2 - nv6) * eq26_e320_d_b5) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b6: f64 = (-(((nv2 - nv6) * eq26_e320_d_b6) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b7: f64 = (-(((nv2 - nv6) * eq26_e320_d_b7) / (eq26_e320 * eq26_e320)));
        (eq26_e321, eq26_e321_d_n0, eq26_e321_d_n1, eq26_e321_d_n2, eq26_e321_d_n3, eq26_e321_d_n4, eq26_e321_d_n5, eq26_e321_d_n6, eq26_e321_d_n7, eq26_e321_d_n8, eq26_e321_d_n9, eq26_e321_d_b0, eq26_e321_d_b1, eq26_e321_d_b2, eq26_e321_d_b3, eq26_e321_d_b4, eq26_e321_d_b5, eq26_e321_d_b6, eq26_e321_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e323;
        let eq26_node_derivatives: [f64; 10] = [eq26_e323_d_n0, eq26_e323_d_n1, eq26_e323_d_n2, eq26_e323_d_n3, eq26_e323_d_n4, eq26_e323_d_n5, eq26_e323_d_n6, eq26_e323_d_n7, eq26_e323_d_n8, eq26_e323_d_n9];
        let eq26_branch_derivatives: [f64; 8] = [eq26_e323_d_b0, eq26_e323_d_b1, eq26_e323_d_b2, eq26_e323_d_b3, eq26_e323_d_b4, eq26_e323_d_b5, eq26_e323_d_b6, eq26_e323_d_b7];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(6),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq29_e349, eq29_e349_d_n0, eq29_e349_d_n1, eq29_e349_d_n2, eq29_e349_d_n3, eq29_e349_d_n4, eq29_e349_d_n5, eq29_e349_d_n6, eq29_e349_d_n7, eq29_e349_d_n8, eq29_e349_d_n9, eq29_e349_d_b0, eq29_e349_d_b1, eq29_e349_d_b2, eq29_e349_d_b3, eq29_e349_d_b4, eq29_e349_d_b5, eq29_e349_d_b6, eq29_e349_d_b7,) = {
    if (var_guard25 != 0.0) {
        let __rspice_inv_cse_4: f64 = 1.0 / var_weff;
        let eq29_e339: f64 = (var_rc * __rspice_inv_cse_4);
        let eq29_e339_d_n0: f64 = (var_rc_dn0 * __rspice_inv_cse_4);
        let eq29_e339_d_n1: f64 = (var_rc_dn1 * __rspice_inv_cse_4);
        let eq29_e339_d_n2: f64 = (var_rc_dn2 * __rspice_inv_cse_4);
        let eq29_e339_d_n3: f64 = (var_rc_dn3 * __rspice_inv_cse_4);
        let eq29_e339_d_n4: f64 = (var_rc_dn4 * __rspice_inv_cse_4);
        let eq29_e339_d_n5: f64 = (var_rc_dn5 * __rspice_inv_cse_4);
        let eq29_e339_d_n6: f64 = (var_rc_dn6 * __rspice_inv_cse_4);
        let eq29_e339_d_n7: f64 = (var_rc_dn7 * __rspice_inv_cse_4);
        let eq29_e339_d_n8: f64 = (var_rc_dn8 * __rspice_inv_cse_4);
        let eq29_e339_d_n9: f64 = (var_rc_dn9 * __rspice_inv_cse_4);
        let eq29_e339_d_b0: f64 = (var_rc_db0 * __rspice_inv_cse_4);
        let eq29_e339_d_b1: f64 = (var_rc_db1 * __rspice_inv_cse_4);
        let eq29_e339_d_b2: f64 = (var_rc_db2 * __rspice_inv_cse_4);
        let eq29_e339_d_b3: f64 = (var_rc_db3 * __rspice_inv_cse_4);
        let eq29_e339_d_b4: f64 = (var_rc_db4 * __rspice_inv_cse_4);
        let eq29_e339_d_b5: f64 = (var_rc_db5 * __rspice_inv_cse_4);
        let eq29_e339_d_b6: f64 = (var_rc_db6 * __rspice_inv_cse_4);
        let eq29_e339_d_b7: f64 = (var_rc_db7 * __rspice_inv_cse_4);
        let (eq29_e346, eq29_e346_d_n0, eq29_e346_d_n1, eq29_e346_d_n2, eq29_e346_d_n3, eq29_e346_d_n4, eq29_e346_d_n5, eq29_e346_d_n6, eq29_e346_d_n7, eq29_e346_d_n8, eq29_e346_d_n9, eq29_e346_d_b0, eq29_e346_d_b1, eq29_e346_d_b2, eq29_e346_d_b3, eq29_e346_d_b4, eq29_e346_d_b5, eq29_e346_d_b6, eq29_e346_d_b7,) = {
            if (eq29_e339 > p.p46) {
                let __rspice_inv_cse_5: f64 = 1.0 / var_weff;
                let eq29_e344: f64 = (var_rc * __rspice_inv_cse_5);
                let eq29_e344_d_n0: f64 = (var_rc_dn0 * __rspice_inv_cse_5);
                let eq29_e344_d_n1: f64 = (var_rc_dn1 * __rspice_inv_cse_5);
                let eq29_e344_d_n2: f64 = (var_rc_dn2 * __rspice_inv_cse_5);
                let eq29_e344_d_n3: f64 = (var_rc_dn3 * __rspice_inv_cse_5);
                let eq29_e344_d_n4: f64 = (var_rc_dn4 * __rspice_inv_cse_5);
                let eq29_e344_d_n5: f64 = (var_rc_dn5 * __rspice_inv_cse_5);
                let eq29_e344_d_n6: f64 = (var_rc_dn6 * __rspice_inv_cse_5);
                let eq29_e344_d_n7: f64 = (var_rc_dn7 * __rspice_inv_cse_5);
                let eq29_e344_d_n8: f64 = (var_rc_dn8 * __rspice_inv_cse_5);
                let eq29_e344_d_n9: f64 = (var_rc_dn9 * __rspice_inv_cse_5);
                let eq29_e344_d_b0: f64 = (var_rc_db0 * __rspice_inv_cse_5);
                let eq29_e344_d_b1: f64 = (var_rc_db1 * __rspice_inv_cse_5);
                let eq29_e344_d_b2: f64 = (var_rc_db2 * __rspice_inv_cse_5);
                let eq29_e344_d_b3: f64 = (var_rc_db3 * __rspice_inv_cse_5);
                let eq29_e344_d_b4: f64 = (var_rc_db4 * __rspice_inv_cse_5);
                let eq29_e344_d_b5: f64 = (var_rc_db5 * __rspice_inv_cse_5);
                let eq29_e344_d_b6: f64 = (var_rc_db6 * __rspice_inv_cse_5);
                let eq29_e344_d_b7: f64 = (var_rc_db7 * __rspice_inv_cse_5);
                (eq29_e344, eq29_e344_d_n0, eq29_e344_d_n1, eq29_e344_d_n2, eq29_e344_d_n3, eq29_e344_d_n4, eq29_e344_d_n5, eq29_e344_d_n6, eq29_e344_d_n7, eq29_e344_d_n8, eq29_e344_d_n9, eq29_e344_d_b0, eq29_e344_d_b1, eq29_e344_d_b2, eq29_e344_d_b3, eq29_e344_d_b4, eq29_e344_d_b5, eq29_e344_d_b6, eq29_e344_d_b7,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq29_e347: f64 = ((nv0 - nv4) / eq29_e346);
        let eq29_e347_d_n0: f64 = ((eq29_e346 - ((nv0 - nv4) * eq29_e346_d_n0)) / (eq29_e346 * eq29_e346));
        let eq29_e347_d_n1: f64 = (-(((nv0 - nv4) * eq29_e346_d_n1) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n2: f64 = (-(((nv0 - nv4) * eq29_e346_d_n2) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n3: f64 = (-(((nv0 - nv4) * eq29_e346_d_n3) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n4: f64 = (((-eq29_e346) - ((nv0 - nv4) * eq29_e346_d_n4)) / (eq29_e346 * eq29_e346));
        let eq29_e347_d_n5: f64 = (-(((nv0 - nv4) * eq29_e346_d_n5) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n6: f64 = (-(((nv0 - nv4) * eq29_e346_d_n6) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n7: f64 = (-(((nv0 - nv4) * eq29_e346_d_n7) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n8: f64 = (-(((nv0 - nv4) * eq29_e346_d_n8) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n9: f64 = (-(((nv0 - nv4) * eq29_e346_d_n9) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b0: f64 = (-(((nv0 - nv4) * eq29_e346_d_b0) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b1: f64 = (-(((nv0 - nv4) * eq29_e346_d_b1) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b2: f64 = (-(((nv0 - nv4) * eq29_e346_d_b2) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b3: f64 = (-(((nv0 - nv4) * eq29_e346_d_b3) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b4: f64 = (-(((nv0 - nv4) * eq29_e346_d_b4) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b5: f64 = (-(((nv0 - nv4) * eq29_e346_d_b5) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b6: f64 = (-(((nv0 - nv4) * eq29_e346_d_b6) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b7: f64 = (-(((nv0 - nv4) * eq29_e346_d_b7) / (eq29_e346 * eq29_e346)));
        (eq29_e347, eq29_e347_d_n0, eq29_e347_d_n1, eq29_e347_d_n2, eq29_e347_d_n3, eq29_e347_d_n4, eq29_e347_d_n5, eq29_e347_d_n6, eq29_e347_d_n7, eq29_e347_d_n8, eq29_e347_d_n9, eq29_e347_d_b0, eq29_e347_d_b1, eq29_e347_d_b2, eq29_e347_d_b3, eq29_e347_d_b4, eq29_e347_d_b5, eq29_e347_d_b6, eq29_e347_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e349;
        let eq29_node_derivatives: [f64; 10] = [eq29_e349_d_n0, eq29_e349_d_n1, eq29_e349_d_n2, eq29_e349_d_n3, eq29_e349_d_n4, eq29_e349_d_n5, eq29_e349_d_n6, eq29_e349_d_n7, eq29_e349_d_n8, eq29_e349_d_n9];
        let eq29_branch_derivatives: [f64; 8] = [eq29_e349_d_b0, eq29_e349_d_b1, eq29_e349_d_b2, eq29_e349_d_b3, eq29_e349_d_b4, eq29_e349_d_b5, eq29_e349_d_b6, eq29_e349_d_b7];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(4),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let eq32_e363: f64 = (var_ttype * var_ibe);
        let eq32_e363_d_n0: f64 = (var_ttype * var_ibe_dn0);
        let eq32_e363_d_n1: f64 = (var_ttype * var_ibe_dn1);
        let eq32_e363_d_n2: f64 = (var_ttype * var_ibe_dn2);
        let eq32_e363_d_n3: f64 = (var_ttype * var_ibe_dn3);
        let eq32_e363_d_n4: f64 = (var_ttype * var_ibe_dn4);
        let eq32_e363_d_n5: f64 = (var_ttype * var_ibe_dn5);
        let eq32_e363_d_n6: f64 = (var_ttype * var_ibe_dn6);
        let eq32_e363_d_n7: f64 = (var_ttype * var_ibe_dn7);
        let eq32_e363_d_n8: f64 = (var_ttype * var_ibe_dn8);
        let eq32_e363_d_n9: f64 = (var_ttype * var_ibe_dn9);
        let eq32_e363_d_b0: f64 = (var_ttype * var_ibe_db0);
        let eq32_e363_d_b1: f64 = (var_ttype * var_ibe_db1);
        let eq32_e363_d_b2: f64 = (var_ttype * var_ibe_db2);
        let eq32_e363_d_b3: f64 = (var_ttype * var_ibe_db3);
        let eq32_e363_d_b4: f64 = (var_ttype * var_ibe_db4);
        let eq32_e363_d_b5: f64 = (var_ttype * var_ibe_db5);
        let eq32_e363_d_b6: f64 = (var_ttype * var_ibe_db6);
        let eq32_e363_d_b7: f64 = (var_ttype * var_ibe_db7);
        let eq32_e365: f64 = (eq32_e363 * var_weff);
        let eq32_e365_d_n0: f64 = (eq32_e363_d_n0 * var_weff);
        let eq32_e365_d_n1: f64 = (eq32_e363_d_n1 * var_weff);
        let eq32_e365_d_n2: f64 = (eq32_e363_d_n2 * var_weff);
        let eq32_e365_d_n3: f64 = (eq32_e363_d_n3 * var_weff);
        let eq32_e365_d_n4: f64 = (eq32_e363_d_n4 * var_weff);
        let eq32_e365_d_n5: f64 = (eq32_e363_d_n5 * var_weff);
        let eq32_e365_d_n6: f64 = (eq32_e363_d_n6 * var_weff);
        let eq32_e365_d_n7: f64 = (eq32_e363_d_n7 * var_weff);
        let eq32_e365_d_n8: f64 = (eq32_e363_d_n8 * var_weff);
        let eq32_e365_d_n9: f64 = (eq32_e363_d_n9 * var_weff);
        let eq32_e365_d_b0: f64 = (eq32_e363_d_b0 * var_weff);
        let eq32_e365_d_b1: f64 = (eq32_e363_d_b1 * var_weff);
        let eq32_e365_d_b2: f64 = (eq32_e363_d_b2 * var_weff);
        let eq32_e365_d_b3: f64 = (eq32_e363_d_b3 * var_weff);
        let eq32_e365_d_b4: f64 = (eq32_e363_d_b4 * var_weff);
        let eq32_e365_d_b5: f64 = (eq32_e363_d_b5 * var_weff);
        let eq32_e365_d_b6: f64 = (eq32_e363_d_b6 * var_weff);
        let eq32_e365_d_b7: f64 = (eq32_e363_d_b7 * var_weff);
        let eq32_value: f64 = eq32_e365;
        let eq32_node_derivatives: [f64; 10] = [eq32_e365_d_n0, eq32_e365_d_n1, eq32_e365_d_n2, eq32_e365_d_n3, eq32_e365_d_n4, eq32_e365_d_n5, eq32_e365_d_n6, eq32_e365_d_n7, eq32_e365_d_n8, eq32_e365_d_n9];
        let eq32_branch_derivatives: [f64; 8] = [eq32_e365_d_b0, eq32_e365_d_b1, eq32_e365_d_b2, eq32_e365_d_b3, eq32_e365_d_b4, eq32_e365_d_b5, eq32_e365_d_b6, eq32_e365_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e368: f64 = (var_ttype * var_ibc);
        let eq33_e368_d_n0: f64 = (var_ttype * var_ibc_dn0);
        let eq33_e368_d_n1: f64 = (var_ttype * var_ibc_dn1);
        let eq33_e368_d_n2: f64 = (var_ttype * var_ibc_dn2);
        let eq33_e368_d_n3: f64 = (var_ttype * var_ibc_dn3);
        let eq33_e368_d_n4: f64 = (var_ttype * var_ibc_dn4);
        let eq33_e368_d_n5: f64 = (var_ttype * var_ibc_dn5);
        let eq33_e368_d_n6: f64 = (var_ttype * var_ibc_dn6);
        let eq33_e368_d_n7: f64 = (var_ttype * var_ibc_dn7);
        let eq33_e368_d_n8: f64 = (var_ttype * var_ibc_dn8);
        let eq33_e368_d_n9: f64 = (var_ttype * var_ibc_dn9);
        let eq33_e368_d_b0: f64 = (var_ttype * var_ibc_db0);
        let eq33_e368_d_b1: f64 = (var_ttype * var_ibc_db1);
        let eq33_e368_d_b2: f64 = (var_ttype * var_ibc_db2);
        let eq33_e368_d_b3: f64 = (var_ttype * var_ibc_db3);
        let eq33_e368_d_b4: f64 = (var_ttype * var_ibc_db4);
        let eq33_e368_d_b5: f64 = (var_ttype * var_ibc_db5);
        let eq33_e368_d_b6: f64 = (var_ttype * var_ibc_db6);
        let eq33_e368_d_b7: f64 = (var_ttype * var_ibc_db7);
        let eq33_e370: f64 = (eq33_e368 * var_weff);
        let eq33_e370_d_n0: f64 = (eq33_e368_d_n0 * var_weff);
        let eq33_e370_d_n1: f64 = (eq33_e368_d_n1 * var_weff);
        let eq33_e370_d_n2: f64 = (eq33_e368_d_n2 * var_weff);
        let eq33_e370_d_n3: f64 = (eq33_e368_d_n3 * var_weff);
        let eq33_e370_d_n4: f64 = (eq33_e368_d_n4 * var_weff);
        let eq33_e370_d_n5: f64 = (eq33_e368_d_n5 * var_weff);
        let eq33_e370_d_n6: f64 = (eq33_e368_d_n6 * var_weff);
        let eq33_e370_d_n7: f64 = (eq33_e368_d_n7 * var_weff);
        let eq33_e370_d_n8: f64 = (eq33_e368_d_n8 * var_weff);
        let eq33_e370_d_n9: f64 = (eq33_e368_d_n9 * var_weff);
        let eq33_e370_d_b0: f64 = (eq33_e368_d_b0 * var_weff);
        let eq33_e370_d_b1: f64 = (eq33_e368_d_b1 * var_weff);
        let eq33_e370_d_b2: f64 = (eq33_e368_d_b2 * var_weff);
        let eq33_e370_d_b3: f64 = (eq33_e368_d_b3 * var_weff);
        let eq33_e370_d_b4: f64 = (eq33_e368_d_b4 * var_weff);
        let eq33_e370_d_b5: f64 = (eq33_e368_d_b5 * var_weff);
        let eq33_e370_d_b6: f64 = (eq33_e368_d_b6 * var_weff);
        let eq33_e370_d_b7: f64 = (eq33_e368_d_b7 * var_weff);
        let eq33_value: f64 = eq33_e370;
        let eq33_node_derivatives: [f64; 10] = [eq33_e370_d_n0, eq33_e370_d_n1, eq33_e370_d_n2, eq33_e370_d_n3, eq33_e370_d_n4, eq33_e370_d_n5, eq33_e370_d_n6, eq33_e370_d_n7, eq33_e370_d_n8, eq33_e370_d_n9];
        let eq33_branch_derivatives: [f64; 8] = [eq33_e370_d_b0, eq33_e370_d_b1, eq33_e370_d_b2, eq33_e370_d_b3, eq33_e370_d_b4, eq33_e370_d_b5, eq33_e370_d_b6, eq33_e370_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        var_itr: f64,
        var_itr_db0: f64,
        var_itr_db1: f64,
        var_itr_db2: f64,
        var_itr_db3: f64,
        var_itr_db4: f64,
        var_itr_db5: f64,
        var_itr_db6: f64,
        var_itr_db7: f64,
        var_itr_dn0: f64,
        var_itr_dn1: f64,
        var_itr_dn2: f64,
        var_itr_dn3: f64,
        var_itr_dn4: f64,
        var_itr_dn5: f64,
        var_itr_dn6: f64,
        var_itr_dn7: f64,
        var_itr_dn8: f64,
        var_itr_dn9: f64,
        var_itzf_f: f64,
        var_itzf_f_db0: f64,
        var_itzf_f_db1: f64,
        var_itzf_f_db2: f64,
        var_itzf_f_db3: f64,
        var_itzf_f_db4: f64,
        var_itzf_f_db5: f64,
        var_itzf_f_db6: f64,
        var_itzf_f_db7: f64,
        var_itzf_f_dn0: f64,
        var_itzf_f_dn1: f64,
        var_itzf_f_dn2: f64,
        var_itzf_f_dn3: f64,
        var_itzf_f_dn4: f64,
        var_itzf_f_dn5: f64,
        var_itzf_f_dn6: f64,
        var_itzf_f_dn7: f64,
        var_itzf_f_dn8: f64,
        var_itzf_f_dn9: f64,
        var_qdc: f64,
        var_qdc_db0: f64,
        var_qdc_db1: f64,
        var_qdc_db2: f64,
        var_qdc_db3: f64,
        var_qdc_db4: f64,
        var_qdc_db5: f64,
        var_qdc_db6: f64,
        var_qdc_db7: f64,
        var_qdc_dn0: f64,
        var_qdc_dn1: f64,
        var_qdc_dn2: f64,
        var_qdc_dn3: f64,
        var_qdc_dn4: f64,
        var_qdc_dn5: f64,
        var_qdc_dn6: f64,
        var_qdc_dn7: f64,
        var_qdc_dn8: f64,
        var_qdc_dn9: f64,
        var_qde: f64,
        var_qde_db0: f64,
        var_qde_db1: f64,
        var_qde_db2: f64,
        var_qde_db3: f64,
        var_qde_db4: f64,
        var_qde_db5: f64,
        var_qde_db6: f64,
        var_qde_db7: f64,
        var_qde_dn0: f64,
        var_qde_dn1: f64,
        var_qde_dn2: f64,
        var_qde_dn3: f64,
        var_qde_dn4: f64,
        var_qde_dn5: f64,
        var_qde_dn6: f64,
        var_qde_dn7: f64,
        var_qde_dn8: f64,
        var_qde_dn9: f64,
        var_qjci_1: f64,
        var_qjci_1_db0: f64,
        var_qjci_1_db1: f64,
        var_qjci_1_db2: f64,
        var_qjci_1_db3: f64,
        var_qjci_1_db4: f64,
        var_qjci_1_db5: f64,
        var_qjci_1_db6: f64,
        var_qjci_1_db7: f64,
        var_qjci_1_dn0: f64,
        var_qjci_1_dn1: f64,
        var_qjci_1_dn2: f64,
        var_qjci_1_dn3: f64,
        var_qjci_1_dn4: f64,
        var_qjci_1_dn5: f64,
        var_qjci_1_dn6: f64,
        var_qjci_1_dn7: f64,
        var_qjci_1_dn8: f64,
        var_qjci_1_dn9: f64,
        var_qjcx_1: f64,
        var_qjcx_1_db0: f64,
        var_qjcx_1_db1: f64,
        var_qjcx_1_db2: f64,
        var_qjcx_1_db3: f64,
        var_qjcx_1_db4: f64,
        var_qjcx_1_db5: f64,
        var_qjcx_1_db6: f64,
        var_qjcx_1_db7: f64,
        var_qjcx_1_dn0: f64,
        var_qjcx_1_dn1: f64,
        var_qjcx_1_dn2: f64,
        var_qjcx_1_dn3: f64,
        var_qjcx_1_dn4: f64,
        var_qjcx_1_dn5: f64,
        var_qjcx_1_dn6: f64,
        var_qjcx_1_dn7: f64,
        var_qjcx_1_dn8: f64,
        var_qjcx_1_dn9: f64,
        var_qje: f64,
        var_qje_db0: f64,
        var_qje_db1: f64,
        var_qje_db2: f64,
        var_qje_db3: f64,
        var_qje_db4: f64,
        var_qje_db5: f64,
        var_qje_db6: f64,
        var_qje_db7: f64,
        var_qje_dn0: f64,
        var_qje_dn1: f64,
        var_qje_dn2: f64,
        var_qje_dn3: f64,
        var_qje_dn4: f64,
        var_qje_dn5: f64,
        var_qje_dn6: f64,
        var_qje_dn7: f64,
        var_qje_dn8: f64,
        var_qje_dn9: f64,
        var_qjs: f64,
        var_qjs_db0: f64,
        var_qjs_db1: f64,
        var_qjs_db2: f64,
        var_qjs_db3: f64,
        var_qjs_db4: f64,
        var_qjs_db5: f64,
        var_qjs_db6: f64,
        var_qjs_db7: f64,
        var_qjs_dn0: f64,
        var_qjs_dn1: f64,
        var_qjs_dn2: f64,
        var_qjs_dn3: f64,
        var_qjs_dn4: f64,
        var_qjs_dn5: f64,
        var_qjs_dn6: f64,
        var_qjs_dn7: f64,
        var_qjs_dn8: f64,
        var_qjs_dn9: f64,
        var_qxf1: f64,
        var_qxf1_db0: f64,
        var_qxf1_db1: f64,
        var_qxf1_db2: f64,
        var_qxf1_db3: f64,
        var_qxf1_db4: f64,
        var_qxf1_db5: f64,
        var_qxf1_db6: f64,
        var_qxf1_db7: f64,
        var_qxf1_dn0: f64,
        var_qxf1_dn1: f64,
        var_qxf1_dn2: f64,
        var_qxf1_dn3: f64,
        var_qxf1_dn4: f64,
        var_qxf1_dn5: f64,
        var_qxf1_dn6: f64,
        var_qxf1_dn7: f64,
        var_qxf1_dn8: f64,
        var_qxf1_dn9: f64,
        var_ttype: f64,
        var_weff: f64,
    ) {
        let eq34_e373: f64 = (-var_itr);
        let eq34_e375: f64 = (eq34_e373 * var_weff);
        let eq34_e375_d_n0: f64 = ((-var_itr_dn0) * var_weff);
        let eq34_e375_d_n1: f64 = ((-var_itr_dn1) * var_weff);
        let eq34_e375_d_n2: f64 = ((-var_itr_dn2) * var_weff);
        let eq34_e375_d_n3: f64 = ((-var_itr_dn3) * var_weff);
        let eq34_e375_d_n4: f64 = ((-var_itr_dn4) * var_weff);
        let eq34_e375_d_n5: f64 = ((-var_itr_dn5) * var_weff);
        let eq34_e375_d_n6: f64 = ((-var_itr_dn6) * var_weff);
        let eq34_e375_d_n7: f64 = ((-var_itr_dn7) * var_weff);
        let eq34_e375_d_n8: f64 = ((-var_itr_dn8) * var_weff);
        let eq34_e375_d_n9: f64 = ((-var_itr_dn9) * var_weff);
        let eq34_e375_d_b0: f64 = ((-var_itr_db0) * var_weff);
        let eq34_e375_d_b1: f64 = ((-var_itr_db1) * var_weff);
        let eq34_e375_d_b2: f64 = ((-var_itr_db2) * var_weff);
        let eq34_e375_d_b3: f64 = ((-var_itr_db3) * var_weff);
        let eq34_e375_d_b4: f64 = ((-var_itr_db4) * var_weff);
        let eq34_e375_d_b5: f64 = ((-var_itr_db5) * var_weff);
        let eq34_e375_d_b6: f64 = ((-var_itr_db6) * var_weff);
        let eq34_e375_d_b7: f64 = ((-var_itr_db7) * var_weff);
        let eq34_e376: f64 = (var_ttype * eq34_e375);
        let eq34_e376_d_n0: f64 = (var_ttype * eq34_e375_d_n0);
        let eq34_e376_d_n1: f64 = (var_ttype * eq34_e375_d_n1);
        let eq34_e376_d_n2: f64 = (var_ttype * eq34_e375_d_n2);
        let eq34_e376_d_n3: f64 = (var_ttype * eq34_e375_d_n3);
        let eq34_e376_d_n4: f64 = (var_ttype * eq34_e375_d_n4);
        let eq34_e376_d_n5: f64 = (var_ttype * eq34_e375_d_n5);
        let eq34_e376_d_n6: f64 = (var_ttype * eq34_e375_d_n6);
        let eq34_e376_d_n7: f64 = (var_ttype * eq34_e375_d_n7);
        let eq34_e376_d_n8: f64 = (var_ttype * eq34_e375_d_n8);
        let eq34_e376_d_n9: f64 = (var_ttype * eq34_e375_d_n9);
        let eq34_e376_d_b0: f64 = (var_ttype * eq34_e375_d_b0);
        let eq34_e376_d_b1: f64 = (var_ttype * eq34_e375_d_b1);
        let eq34_e376_d_b2: f64 = (var_ttype * eq34_e375_d_b2);
        let eq34_e376_d_b3: f64 = (var_ttype * eq34_e375_d_b3);
        let eq34_e376_d_b4: f64 = (var_ttype * eq34_e375_d_b4);
        let eq34_e376_d_b5: f64 = (var_ttype * eq34_e375_d_b5);
        let eq34_e376_d_b6: f64 = (var_ttype * eq34_e375_d_b6);
        let eq34_e376_d_b7: f64 = (var_ttype * eq34_e375_d_b7);
        let eq34_value: f64 = eq34_e376;
        let eq34_node_derivatives: [f64; 10] = [eq34_e376_d_n0, eq34_e376_d_n1, eq34_e376_d_n2, eq34_e376_d_n3, eq34_e376_d_n4, eq34_e376_d_n5, eq34_e376_d_n6, eq34_e376_d_n7, eq34_e376_d_n8, eq34_e376_d_n9];
        let eq34_branch_derivatives: [f64; 8] = [eq34_e376_d_b0, eq34_e376_d_b1, eq34_e376_d_b2, eq34_e376_d_b3, eq34_e376_d_b4, eq34_e376_d_b5, eq34_e376_d_b6, eq34_e376_d_b7];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(6),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let eq35_e379: f64 = (var_ttype * var_itzf_f);
        let eq35_e379_d_n0: f64 = (var_ttype * var_itzf_f_dn0);
        let eq35_e379_d_n1: f64 = (var_ttype * var_itzf_f_dn1);
        let eq35_e379_d_n2: f64 = (var_ttype * var_itzf_f_dn2);
        let eq35_e379_d_n3: f64 = (var_ttype * var_itzf_f_dn3);
        let eq35_e379_d_n4: f64 = (var_ttype * var_itzf_f_dn4);
        let eq35_e379_d_n5: f64 = (var_ttype * var_itzf_f_dn5);
        let eq35_e379_d_n6: f64 = (var_ttype * var_itzf_f_dn6);
        let eq35_e379_d_n7: f64 = (var_ttype * var_itzf_f_dn7);
        let eq35_e379_d_n8: f64 = (var_ttype * var_itzf_f_dn8);
        let eq35_e379_d_n9: f64 = (var_ttype * var_itzf_f_dn9);
        let eq35_e379_d_b0: f64 = (var_ttype * var_itzf_f_db0);
        let eq35_e379_d_b1: f64 = (var_ttype * var_itzf_f_db1);
        let eq35_e379_d_b2: f64 = (var_ttype * var_itzf_f_db2);
        let eq35_e379_d_b3: f64 = (var_ttype * var_itzf_f_db3);
        let eq35_e379_d_b4: f64 = (var_ttype * var_itzf_f_db4);
        let eq35_e379_d_b5: f64 = (var_ttype * var_itzf_f_db5);
        let eq35_e379_d_b6: f64 = (var_ttype * var_itzf_f_db6);
        let eq35_e379_d_b7: f64 = (var_ttype * var_itzf_f_db7);
        let eq35_e381: f64 = (eq35_e379 * var_weff);
        let eq35_e381_d_n0: f64 = (eq35_e379_d_n0 * var_weff);
        let eq35_e381_d_n1: f64 = (eq35_e379_d_n1 * var_weff);
        let eq35_e381_d_n2: f64 = (eq35_e379_d_n2 * var_weff);
        let eq35_e381_d_n3: f64 = (eq35_e379_d_n3 * var_weff);
        let eq35_e381_d_n4: f64 = (eq35_e379_d_n4 * var_weff);
        let eq35_e381_d_n5: f64 = (eq35_e379_d_n5 * var_weff);
        let eq35_e381_d_n6: f64 = (eq35_e379_d_n6 * var_weff);
        let eq35_e381_d_n7: f64 = (eq35_e379_d_n7 * var_weff);
        let eq35_e381_d_n8: f64 = (eq35_e379_d_n8 * var_weff);
        let eq35_e381_d_n9: f64 = (eq35_e379_d_n9 * var_weff);
        let eq35_e381_d_b0: f64 = (eq35_e379_d_b0 * var_weff);
        let eq35_e381_d_b1: f64 = (eq35_e379_d_b1 * var_weff);
        let eq35_e381_d_b2: f64 = (eq35_e379_d_b2 * var_weff);
        let eq35_e381_d_b3: f64 = (eq35_e379_d_b3 * var_weff);
        let eq35_e381_d_b4: f64 = (eq35_e379_d_b4 * var_weff);
        let eq35_e381_d_b5: f64 = (eq35_e379_d_b5 * var_weff);
        let eq35_e381_d_b6: f64 = (eq35_e379_d_b6 * var_weff);
        let eq35_e381_d_b7: f64 = (eq35_e379_d_b7 * var_weff);
        let eq35_value: f64 = eq35_e381;
        let eq35_node_derivatives: [f64; 10] = [eq35_e381_d_n0, eq35_e381_d_n1, eq35_e381_d_n2, eq35_e381_d_n3, eq35_e381_d_n4, eq35_e381_d_n5, eq35_e381_d_n6, eq35_e381_d_n7, eq35_e381_d_n8, eq35_e381_d_n9];
        let eq35_branch_derivatives: [f64; 8] = [eq35_e381_d_b0, eq35_e381_d_b1, eq35_e381_d_b2, eq35_e381_d_b3, eq35_e381_d_b4, eq35_e381_d_b5, eq35_e381_d_b6, eq35_e381_d_b7];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(6),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let eq36_e384: f64 = (var_ttype * var_qje);
        let eq36_e384_d_n0: f64 = (var_ttype * var_qje_dn0);
        let eq36_e384_d_n1: f64 = (var_ttype * var_qje_dn1);
        let eq36_e384_d_n2: f64 = (var_ttype * var_qje_dn2);
        let eq36_e384_d_n3: f64 = (var_ttype * var_qje_dn3);
        let eq36_e384_d_n4: f64 = (var_ttype * var_qje_dn4);
        let eq36_e384_d_n5: f64 = (var_ttype * var_qje_dn5);
        let eq36_e384_d_n6: f64 = (var_ttype * var_qje_dn6);
        let eq36_e384_d_n7: f64 = (var_ttype * var_qje_dn7);
        let eq36_e384_d_n8: f64 = (var_ttype * var_qje_dn8);
        let eq36_e384_d_n9: f64 = (var_ttype * var_qje_dn9);
        let eq36_e384_d_b0: f64 = (var_ttype * var_qje_db0);
        let eq36_e384_d_b1: f64 = (var_ttype * var_qje_db1);
        let eq36_e384_d_b2: f64 = (var_ttype * var_qje_db2);
        let eq36_e384_d_b3: f64 = (var_ttype * var_qje_db3);
        let eq36_e384_d_b4: f64 = (var_ttype * var_qje_db4);
        let eq36_e384_d_b5: f64 = (var_ttype * var_qje_db5);
        let eq36_e384_d_b6: f64 = (var_ttype * var_qje_db6);
        let eq36_e384_d_b7: f64 = (var_ttype * var_qje_db7);
        let eq36_e386: f64 = (eq36_e384 * var_weff);
        let eq36_e386_d_n0: f64 = (eq36_e384_d_n0 * var_weff);
        let eq36_e386_d_n1: f64 = (eq36_e384_d_n1 * var_weff);
        let eq36_e386_d_n2: f64 = (eq36_e384_d_n2 * var_weff);
        let eq36_e386_d_n3: f64 = (eq36_e384_d_n3 * var_weff);
        let eq36_e386_d_n4: f64 = (eq36_e384_d_n4 * var_weff);
        let eq36_e386_d_n5: f64 = (eq36_e384_d_n5 * var_weff);
        let eq36_e386_d_n6: f64 = (eq36_e384_d_n6 * var_weff);
        let eq36_e386_d_n7: f64 = (eq36_e384_d_n7 * var_weff);
        let eq36_e386_d_n8: f64 = (eq36_e384_d_n8 * var_weff);
        let eq36_e386_d_n9: f64 = (eq36_e384_d_n9 * var_weff);
        let eq36_e386_d_b0: f64 = (eq36_e384_d_b0 * var_weff);
        let eq36_e386_d_b1: f64 = (eq36_e384_d_b1 * var_weff);
        let eq36_e386_d_b2: f64 = (eq36_e384_d_b2 * var_weff);
        let eq36_e386_d_b3: f64 = (eq36_e384_d_b3 * var_weff);
        let eq36_e386_d_b4: f64 = (eq36_e384_d_b4 * var_weff);
        let eq36_e386_d_b5: f64 = (eq36_e384_d_b5 * var_weff);
        let eq36_e386_d_b6: f64 = (eq36_e384_d_b6 * var_weff);
        let eq36_e386_d_b7: f64 = (eq36_e384_d_b7 * var_weff);
        let eq36_e387: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq36_e386);
        let eq36_value: f64 = eq36_e387;
        let eq36_node_derivatives: [f64; 10] = [(eq36_e386_d_n0 * ddt_scale), (eq36_e386_d_n1 * ddt_scale), (eq36_e386_d_n2 * ddt_scale), (eq36_e386_d_n3 * ddt_scale), (eq36_e386_d_n4 * ddt_scale), (eq36_e386_d_n5 * ddt_scale), (eq36_e386_d_n6 * ddt_scale), (eq36_e386_d_n7 * ddt_scale), (eq36_e386_d_n8 * ddt_scale), (eq36_e386_d_n9 * ddt_scale)];
        let eq36_branch_derivatives: [f64; 8] = [(eq36_e386_d_b0 * ddt_scale), (eq36_e386_d_b1 * ddt_scale), (eq36_e386_d_b2 * ddt_scale), (eq36_e386_d_b3 * ddt_scale), (eq36_e386_d_b4 * ddt_scale), (eq36_e386_d_b5 * ddt_scale), (eq36_e386_d_b6 * ddt_scale), (eq36_e386_d_b7 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let eq37_e390: f64 = (var_ttype * var_qde);
        let eq37_e390_d_n0: f64 = (var_ttype * var_qde_dn0);
        let eq37_e390_d_n1: f64 = (var_ttype * var_qde_dn1);
        let eq37_e390_d_n2: f64 = (var_ttype * var_qde_dn2);
        let eq37_e390_d_n3: f64 = (var_ttype * var_qde_dn3);
        let eq37_e390_d_n4: f64 = (var_ttype * var_qde_dn4);
        let eq37_e390_d_n5: f64 = (var_ttype * var_qde_dn5);
        let eq37_e390_d_n6: f64 = (var_ttype * var_qde_dn6);
        let eq37_e390_d_n7: f64 = (var_ttype * var_qde_dn7);
        let eq37_e390_d_n8: f64 = (var_ttype * var_qde_dn8);
        let eq37_e390_d_n9: f64 = (var_ttype * var_qde_dn9);
        let eq37_e390_d_b0: f64 = (var_ttype * var_qde_db0);
        let eq37_e390_d_b1: f64 = (var_ttype * var_qde_db1);
        let eq37_e390_d_b2: f64 = (var_ttype * var_qde_db2);
        let eq37_e390_d_b3: f64 = (var_ttype * var_qde_db3);
        let eq37_e390_d_b4: f64 = (var_ttype * var_qde_db4);
        let eq37_e390_d_b5: f64 = (var_ttype * var_qde_db5);
        let eq37_e390_d_b6: f64 = (var_ttype * var_qde_db6);
        let eq37_e390_d_b7: f64 = (var_ttype * var_qde_db7);
        let eq37_e392: f64 = (eq37_e390 * var_weff);
        let eq37_e392_d_n0: f64 = (eq37_e390_d_n0 * var_weff);
        let eq37_e392_d_n1: f64 = (eq37_e390_d_n1 * var_weff);
        let eq37_e392_d_n2: f64 = (eq37_e390_d_n2 * var_weff);
        let eq37_e392_d_n3: f64 = (eq37_e390_d_n3 * var_weff);
        let eq37_e392_d_n4: f64 = (eq37_e390_d_n4 * var_weff);
        let eq37_e392_d_n5: f64 = (eq37_e390_d_n5 * var_weff);
        let eq37_e392_d_n6: f64 = (eq37_e390_d_n6 * var_weff);
        let eq37_e392_d_n7: f64 = (eq37_e390_d_n7 * var_weff);
        let eq37_e392_d_n8: f64 = (eq37_e390_d_n8 * var_weff);
        let eq37_e392_d_n9: f64 = (eq37_e390_d_n9 * var_weff);
        let eq37_e392_d_b0: f64 = (eq37_e390_d_b0 * var_weff);
        let eq37_e392_d_b1: f64 = (eq37_e390_d_b1 * var_weff);
        let eq37_e392_d_b2: f64 = (eq37_e390_d_b2 * var_weff);
        let eq37_e392_d_b3: f64 = (eq37_e390_d_b3 * var_weff);
        let eq37_e392_d_b4: f64 = (eq37_e390_d_b4 * var_weff);
        let eq37_e392_d_b5: f64 = (eq37_e390_d_b5 * var_weff);
        let eq37_e392_d_b6: f64 = (eq37_e390_d_b6 * var_weff);
        let eq37_e392_d_b7: f64 = (eq37_e390_d_b7 * var_weff);
        let eq37_e393: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq37_e392);
        let eq37_value: f64 = eq37_e393;
        let eq37_node_derivatives: [f64; 10] = [(eq37_e392_d_n0 * ddt_scale), (eq37_e392_d_n1 * ddt_scale), (eq37_e392_d_n2 * ddt_scale), (eq37_e392_d_n3 * ddt_scale), (eq37_e392_d_n4 * ddt_scale), (eq37_e392_d_n5 * ddt_scale), (eq37_e392_d_n6 * ddt_scale), (eq37_e392_d_n7 * ddt_scale), (eq37_e392_d_n8 * ddt_scale), (eq37_e392_d_n9 * ddt_scale)];
        let eq37_branch_derivatives: [f64; 8] = [(eq37_e392_d_b0 * ddt_scale), (eq37_e392_d_b1 * ddt_scale), (eq37_e392_d_b2 * ddt_scale), (eq37_e392_d_b3 * ddt_scale), (eq37_e392_d_b4 * ddt_scale), (eq37_e392_d_b5 * ddt_scale), (eq37_e392_d_b6 * ddt_scale), (eq37_e392_d_b7 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let eq38_e396: f64 = (var_ttype * var_qjcx_1);
        let eq38_e396_d_n0: f64 = (var_ttype * var_qjcx_1_dn0);
        let eq38_e396_d_n1: f64 = (var_ttype * var_qjcx_1_dn1);
        let eq38_e396_d_n2: f64 = (var_ttype * var_qjcx_1_dn2);
        let eq38_e396_d_n3: f64 = (var_ttype * var_qjcx_1_dn3);
        let eq38_e396_d_n4: f64 = (var_ttype * var_qjcx_1_dn4);
        let eq38_e396_d_n5: f64 = (var_ttype * var_qjcx_1_dn5);
        let eq38_e396_d_n6: f64 = (var_ttype * var_qjcx_1_dn6);
        let eq38_e396_d_n7: f64 = (var_ttype * var_qjcx_1_dn7);
        let eq38_e396_d_n8: f64 = (var_ttype * var_qjcx_1_dn8);
        let eq38_e396_d_n9: f64 = (var_ttype * var_qjcx_1_dn9);
        let eq38_e396_d_b0: f64 = (var_ttype * var_qjcx_1_db0);
        let eq38_e396_d_b1: f64 = (var_ttype * var_qjcx_1_db1);
        let eq38_e396_d_b2: f64 = (var_ttype * var_qjcx_1_db2);
        let eq38_e396_d_b3: f64 = (var_ttype * var_qjcx_1_db3);
        let eq38_e396_d_b4: f64 = (var_ttype * var_qjcx_1_db4);
        let eq38_e396_d_b5: f64 = (var_ttype * var_qjcx_1_db5);
        let eq38_e396_d_b6: f64 = (var_ttype * var_qjcx_1_db6);
        let eq38_e396_d_b7: f64 = (var_ttype * var_qjcx_1_db7);
        let eq38_e398: f64 = (eq38_e396 * var_weff);
        let eq38_e398_d_n0: f64 = (eq38_e396_d_n0 * var_weff);
        let eq38_e398_d_n1: f64 = (eq38_e396_d_n1 * var_weff);
        let eq38_e398_d_n2: f64 = (eq38_e396_d_n2 * var_weff);
        let eq38_e398_d_n3: f64 = (eq38_e396_d_n3 * var_weff);
        let eq38_e398_d_n4: f64 = (eq38_e396_d_n4 * var_weff);
        let eq38_e398_d_n5: f64 = (eq38_e396_d_n5 * var_weff);
        let eq38_e398_d_n6: f64 = (eq38_e396_d_n6 * var_weff);
        let eq38_e398_d_n7: f64 = (eq38_e396_d_n7 * var_weff);
        let eq38_e398_d_n8: f64 = (eq38_e396_d_n8 * var_weff);
        let eq38_e398_d_n9: f64 = (eq38_e396_d_n9 * var_weff);
        let eq38_e398_d_b0: f64 = (eq38_e396_d_b0 * var_weff);
        let eq38_e398_d_b1: f64 = (eq38_e396_d_b1 * var_weff);
        let eq38_e398_d_b2: f64 = (eq38_e396_d_b2 * var_weff);
        let eq38_e398_d_b3: f64 = (eq38_e396_d_b3 * var_weff);
        let eq38_e398_d_b4: f64 = (eq38_e396_d_b4 * var_weff);
        let eq38_e398_d_b5: f64 = (eq38_e396_d_b5 * var_weff);
        let eq38_e398_d_b6: f64 = (eq38_e396_d_b6 * var_weff);
        let eq38_e398_d_b7: f64 = (eq38_e396_d_b7 * var_weff);
        let eq38_e399: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq38_e398);
        let eq38_value: f64 = eq38_e399;
        let eq38_node_derivatives: [f64; 10] = [(eq38_e398_d_n0 * ddt_scale), (eq38_e398_d_n1 * ddt_scale), (eq38_e398_d_n2 * ddt_scale), (eq38_e398_d_n3 * ddt_scale), (eq38_e398_d_n4 * ddt_scale), (eq38_e398_d_n5 * ddt_scale), (eq38_e398_d_n6 * ddt_scale), (eq38_e398_d_n7 * ddt_scale), (eq38_e398_d_n8 * ddt_scale), (eq38_e398_d_n9 * ddt_scale)];
        let eq38_branch_derivatives: [f64; 8] = [(eq38_e398_d_b0 * ddt_scale), (eq38_e398_d_b1 * ddt_scale), (eq38_e398_d_b2 * ddt_scale), (eq38_e398_d_b3 * ddt_scale), (eq38_e398_d_b4 * ddt_scale), (eq38_e398_d_b5 * ddt_scale), (eq38_e398_d_b6 * ddt_scale), (eq38_e398_d_b7 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(4),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let eq39_e402: f64 = (var_ttype * var_qjci_1);
        let eq39_e402_d_n0: f64 = (var_ttype * var_qjci_1_dn0);
        let eq39_e402_d_n1: f64 = (var_ttype * var_qjci_1_dn1);
        let eq39_e402_d_n2: f64 = (var_ttype * var_qjci_1_dn2);
        let eq39_e402_d_n3: f64 = (var_ttype * var_qjci_1_dn3);
        let eq39_e402_d_n4: f64 = (var_ttype * var_qjci_1_dn4);
        let eq39_e402_d_n5: f64 = (var_ttype * var_qjci_1_dn5);
        let eq39_e402_d_n6: f64 = (var_ttype * var_qjci_1_dn6);
        let eq39_e402_d_n7: f64 = (var_ttype * var_qjci_1_dn7);
        let eq39_e402_d_n8: f64 = (var_ttype * var_qjci_1_dn8);
        let eq39_e402_d_n9: f64 = (var_ttype * var_qjci_1_dn9);
        let eq39_e402_d_b0: f64 = (var_ttype * var_qjci_1_db0);
        let eq39_e402_d_b1: f64 = (var_ttype * var_qjci_1_db1);
        let eq39_e402_d_b2: f64 = (var_ttype * var_qjci_1_db2);
        let eq39_e402_d_b3: f64 = (var_ttype * var_qjci_1_db3);
        let eq39_e402_d_b4: f64 = (var_ttype * var_qjci_1_db4);
        let eq39_e402_d_b5: f64 = (var_ttype * var_qjci_1_db5);
        let eq39_e402_d_b6: f64 = (var_ttype * var_qjci_1_db6);
        let eq39_e402_d_b7: f64 = (var_ttype * var_qjci_1_db7);
        let eq39_e404: f64 = (eq39_e402 * var_weff);
        let eq39_e404_d_n0: f64 = (eq39_e402_d_n0 * var_weff);
        let eq39_e404_d_n1: f64 = (eq39_e402_d_n1 * var_weff);
        let eq39_e404_d_n2: f64 = (eq39_e402_d_n2 * var_weff);
        let eq39_e404_d_n3: f64 = (eq39_e402_d_n3 * var_weff);
        let eq39_e404_d_n4: f64 = (eq39_e402_d_n4 * var_weff);
        let eq39_e404_d_n5: f64 = (eq39_e402_d_n5 * var_weff);
        let eq39_e404_d_n6: f64 = (eq39_e402_d_n6 * var_weff);
        let eq39_e404_d_n7: f64 = (eq39_e402_d_n7 * var_weff);
        let eq39_e404_d_n8: f64 = (eq39_e402_d_n8 * var_weff);
        let eq39_e404_d_n9: f64 = (eq39_e402_d_n9 * var_weff);
        let eq39_e404_d_b0: f64 = (eq39_e402_d_b0 * var_weff);
        let eq39_e404_d_b1: f64 = (eq39_e402_d_b1 * var_weff);
        let eq39_e404_d_b2: f64 = (eq39_e402_d_b2 * var_weff);
        let eq39_e404_d_b3: f64 = (eq39_e402_d_b3 * var_weff);
        let eq39_e404_d_b4: f64 = (eq39_e402_d_b4 * var_weff);
        let eq39_e404_d_b5: f64 = (eq39_e402_d_b5 * var_weff);
        let eq39_e404_d_b6: f64 = (eq39_e402_d_b6 * var_weff);
        let eq39_e404_d_b7: f64 = (eq39_e402_d_b7 * var_weff);
        let eq39_e405: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq39_e404);
        let eq39_value: f64 = eq39_e405;
        let eq39_node_derivatives: [f64; 10] = [(eq39_e404_d_n0 * ddt_scale), (eq39_e404_d_n1 * ddt_scale), (eq39_e404_d_n2 * ddt_scale), (eq39_e404_d_n3 * ddt_scale), (eq39_e404_d_n4 * ddt_scale), (eq39_e404_d_n5 * ddt_scale), (eq39_e404_d_n6 * ddt_scale), (eq39_e404_d_n7 * ddt_scale), (eq39_e404_d_n8 * ddt_scale), (eq39_e404_d_n9 * ddt_scale)];
        let eq39_branch_derivatives: [f64; 8] = [(eq39_e404_d_b0 * ddt_scale), (eq39_e404_d_b1 * ddt_scale), (eq39_e404_d_b2 * ddt_scale), (eq39_e404_d_b3 * ddt_scale), (eq39_e404_d_b4 * ddt_scale), (eq39_e404_d_b5 * ddt_scale), (eq39_e404_d_b6 * ddt_scale), (eq39_e404_d_b7 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let eq40_e408: f64 = (var_ttype * var_qdc);
        let eq40_e408_d_n0: f64 = (var_ttype * var_qdc_dn0);
        let eq40_e408_d_n1: f64 = (var_ttype * var_qdc_dn1);
        let eq40_e408_d_n2: f64 = (var_ttype * var_qdc_dn2);
        let eq40_e408_d_n3: f64 = (var_ttype * var_qdc_dn3);
        let eq40_e408_d_n4: f64 = (var_ttype * var_qdc_dn4);
        let eq40_e408_d_n5: f64 = (var_ttype * var_qdc_dn5);
        let eq40_e408_d_n6: f64 = (var_ttype * var_qdc_dn6);
        let eq40_e408_d_n7: f64 = (var_ttype * var_qdc_dn7);
        let eq40_e408_d_n8: f64 = (var_ttype * var_qdc_dn8);
        let eq40_e408_d_n9: f64 = (var_ttype * var_qdc_dn9);
        let eq40_e408_d_b0: f64 = (var_ttype * var_qdc_db0);
        let eq40_e408_d_b1: f64 = (var_ttype * var_qdc_db1);
        let eq40_e408_d_b2: f64 = (var_ttype * var_qdc_db2);
        let eq40_e408_d_b3: f64 = (var_ttype * var_qdc_db3);
        let eq40_e408_d_b4: f64 = (var_ttype * var_qdc_db4);
        let eq40_e408_d_b5: f64 = (var_ttype * var_qdc_db5);
        let eq40_e408_d_b6: f64 = (var_ttype * var_qdc_db6);
        let eq40_e408_d_b7: f64 = (var_ttype * var_qdc_db7);
        let eq40_e410: f64 = (eq40_e408 * var_weff);
        let eq40_e410_d_n0: f64 = (eq40_e408_d_n0 * var_weff);
        let eq40_e410_d_n1: f64 = (eq40_e408_d_n1 * var_weff);
        let eq40_e410_d_n2: f64 = (eq40_e408_d_n2 * var_weff);
        let eq40_e410_d_n3: f64 = (eq40_e408_d_n3 * var_weff);
        let eq40_e410_d_n4: f64 = (eq40_e408_d_n4 * var_weff);
        let eq40_e410_d_n5: f64 = (eq40_e408_d_n5 * var_weff);
        let eq40_e410_d_n6: f64 = (eq40_e408_d_n6 * var_weff);
        let eq40_e410_d_n7: f64 = (eq40_e408_d_n7 * var_weff);
        let eq40_e410_d_n8: f64 = (eq40_e408_d_n8 * var_weff);
        let eq40_e410_d_n9: f64 = (eq40_e408_d_n9 * var_weff);
        let eq40_e410_d_b0: f64 = (eq40_e408_d_b0 * var_weff);
        let eq40_e410_d_b1: f64 = (eq40_e408_d_b1 * var_weff);
        let eq40_e410_d_b2: f64 = (eq40_e408_d_b2 * var_weff);
        let eq40_e410_d_b3: f64 = (eq40_e408_d_b3 * var_weff);
        let eq40_e410_d_b4: f64 = (eq40_e408_d_b4 * var_weff);
        let eq40_e410_d_b5: f64 = (eq40_e408_d_b5 * var_weff);
        let eq40_e410_d_b6: f64 = (eq40_e408_d_b6 * var_weff);
        let eq40_e410_d_b7: f64 = (eq40_e408_d_b7 * var_weff);
        let eq40_e411: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq40_e410);
        let eq40_value: f64 = eq40_e411;
        let eq40_node_derivatives: [f64; 10] = [(eq40_e410_d_n0 * ddt_scale), (eq40_e410_d_n1 * ddt_scale), (eq40_e410_d_n2 * ddt_scale), (eq40_e410_d_n3 * ddt_scale), (eq40_e410_d_n4 * ddt_scale), (eq40_e410_d_n5 * ddt_scale), (eq40_e410_d_n6 * ddt_scale), (eq40_e410_d_n7 * ddt_scale), (eq40_e410_d_n8 * ddt_scale), (eq40_e410_d_n9 * ddt_scale)];
        let eq40_branch_derivatives: [f64; 8] = [(eq40_e410_d_b0 * ddt_scale), (eq40_e410_d_b1 * ddt_scale), (eq40_e410_d_b2 * ddt_scale), (eq40_e410_d_b3 * ddt_scale), (eq40_e410_d_b4 * ddt_scale), (eq40_e410_d_b5 * ddt_scale), (eq40_e410_d_b6 * ddt_scale), (eq40_e410_d_b7 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e414: f64 = (var_ttype * var_qjs);
        let eq41_e414_d_n0: f64 = (var_ttype * var_qjs_dn0);
        let eq41_e414_d_n1: f64 = (var_ttype * var_qjs_dn1);
        let eq41_e414_d_n2: f64 = (var_ttype * var_qjs_dn2);
        let eq41_e414_d_n3: f64 = (var_ttype * var_qjs_dn3);
        let eq41_e414_d_n4: f64 = (var_ttype * var_qjs_dn4);
        let eq41_e414_d_n5: f64 = (var_ttype * var_qjs_dn5);
        let eq41_e414_d_n6: f64 = (var_ttype * var_qjs_dn6);
        let eq41_e414_d_n7: f64 = (var_ttype * var_qjs_dn7);
        let eq41_e414_d_n8: f64 = (var_ttype * var_qjs_dn8);
        let eq41_e414_d_n9: f64 = (var_ttype * var_qjs_dn9);
        let eq41_e414_d_b0: f64 = (var_ttype * var_qjs_db0);
        let eq41_e414_d_b1: f64 = (var_ttype * var_qjs_db1);
        let eq41_e414_d_b2: f64 = (var_ttype * var_qjs_db2);
        let eq41_e414_d_b3: f64 = (var_ttype * var_qjs_db3);
        let eq41_e414_d_b4: f64 = (var_ttype * var_qjs_db4);
        let eq41_e414_d_b5: f64 = (var_ttype * var_qjs_db5);
        let eq41_e414_d_b6: f64 = (var_ttype * var_qjs_db6);
        let eq41_e414_d_b7: f64 = (var_ttype * var_qjs_db7);
        let eq41_e416: f64 = (eq41_e414 * var_weff);
        let eq41_e416_d_n0: f64 = (eq41_e414_d_n0 * var_weff);
        let eq41_e416_d_n1: f64 = (eq41_e414_d_n1 * var_weff);
        let eq41_e416_d_n2: f64 = (eq41_e414_d_n2 * var_weff);
        let eq41_e416_d_n3: f64 = (eq41_e414_d_n3 * var_weff);
        let eq41_e416_d_n4: f64 = (eq41_e414_d_n4 * var_weff);
        let eq41_e416_d_n5: f64 = (eq41_e414_d_n5 * var_weff);
        let eq41_e416_d_n6: f64 = (eq41_e414_d_n6 * var_weff);
        let eq41_e416_d_n7: f64 = (eq41_e414_d_n7 * var_weff);
        let eq41_e416_d_n8: f64 = (eq41_e414_d_n8 * var_weff);
        let eq41_e416_d_n9: f64 = (eq41_e414_d_n9 * var_weff);
        let eq41_e416_d_b0: f64 = (eq41_e414_d_b0 * var_weff);
        let eq41_e416_d_b1: f64 = (eq41_e414_d_b1 * var_weff);
        let eq41_e416_d_b2: f64 = (eq41_e414_d_b2 * var_weff);
        let eq41_e416_d_b3: f64 = (eq41_e414_d_b3 * var_weff);
        let eq41_e416_d_b4: f64 = (eq41_e414_d_b4 * var_weff);
        let eq41_e416_d_b5: f64 = (eq41_e414_d_b5 * var_weff);
        let eq41_e416_d_b6: f64 = (eq41_e414_d_b6 * var_weff);
        let eq41_e416_d_b7: f64 = (eq41_e414_d_b7 * var_weff);
        let eq41_e417: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq41_e416);
        let eq41_value: f64 = eq41_e417;
        let eq41_node_derivatives: [f64; 10] = [(eq41_e416_d_n0 * ddt_scale), (eq41_e416_d_n1 * ddt_scale), (eq41_e416_d_n2 * ddt_scale), (eq41_e416_d_n3 * ddt_scale), (eq41_e416_d_n4 * ddt_scale), (eq41_e416_d_n5 * ddt_scale), (eq41_e416_d_n6 * ddt_scale), (eq41_e416_d_n7 * ddt_scale), (eq41_e416_d_n8 * ddt_scale), (eq41_e416_d_n9 * ddt_scale)];
        let eq41_branch_derivatives: [f64; 8] = [(eq41_e416_d_b0 * ddt_scale), (eq41_e416_d_b1 * ddt_scale), (eq41_e416_d_b2 * ddt_scale), (eq41_e416_d_b3 * ddt_scale), (eq41_e416_d_b4 * ddt_scale), (eq41_e416_d_b5 * ddt_scale), (eq41_e416_d_b6 * ddt_scale), (eq41_e416_d_b7 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(4),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e419: f64 = (-var_qxf1);
        let eq42_e421: f64 = (eq42_e419 * var_weff);
        let eq42_e421_d_n0: f64 = ((-var_qxf1_dn0) * var_weff);
        let eq42_e421_d_n1: f64 = ((-var_qxf1_dn1) * var_weff);
        let eq42_e421_d_n2: f64 = ((-var_qxf1_dn2) * var_weff);
        let eq42_e421_d_n3: f64 = ((-var_qxf1_dn3) * var_weff);
        let eq42_e421_d_n4: f64 = ((-var_qxf1_dn4) * var_weff);
        let eq42_e421_d_n5: f64 = ((-var_qxf1_dn5) * var_weff);
        let eq42_e421_d_n6: f64 = ((-var_qxf1_dn6) * var_weff);
        let eq42_e421_d_n7: f64 = ((-var_qxf1_dn7) * var_weff);
        let eq42_e421_d_n8: f64 = ((-var_qxf1_dn8) * var_weff);
        let eq42_e421_d_n9: f64 = ((-var_qxf1_dn9) * var_weff);
        let eq42_e421_d_b0: f64 = ((-var_qxf1_db0) * var_weff);
        let eq42_e421_d_b1: f64 = ((-var_qxf1_db1) * var_weff);
        let eq42_e421_d_b2: f64 = ((-var_qxf1_db2) * var_weff);
        let eq42_e421_d_b3: f64 = ((-var_qxf1_db3) * var_weff);
        let eq42_e421_d_b4: f64 = ((-var_qxf1_db4) * var_weff);
        let eq42_e421_d_b5: f64 = ((-var_qxf1_db5) * var_weff);
        let eq42_e421_d_b6: f64 = ((-var_qxf1_db6) * var_weff);
        let eq42_e421_d_b7: f64 = ((-var_qxf1_db7) * var_weff);
        let eq42_e422: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq42_e421);
        let eq42_value: f64 = eq42_e422;
        let eq42_node_derivatives: [f64; 10] = [(eq42_e421_d_n0 * ddt_scale), (eq42_e421_d_n1 * ddt_scale), (eq42_e421_d_n2 * ddt_scale), (eq42_e421_d_n3 * ddt_scale), (eq42_e421_d_n4 * ddt_scale), (eq42_e421_d_n5 * ddt_scale), (eq42_e421_d_n6 * ddt_scale), (eq42_e421_d_n7 * ddt_scale), (eq42_e421_d_n8 * ddt_scale), (eq42_e421_d_n9 * ddt_scale)];
        let eq42_branch_derivatives: [f64; 8] = [(eq42_e421_d_b0 * ddt_scale), (eq42_e421_d_b1 * ddt_scale), (eq42_e421_d_b2 * ddt_scale), (eq42_e421_d_b3 * ddt_scale), (eq42_e421_d_b4 * ddt_scale), (eq42_e421_d_b5 * ddt_scale), (eq42_e421_d_b6 * ddt_scale), (eq42_e421_d_b7 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let eq43_e425: f64 = (var_qxf1 * var_weff);
        let eq43_e425_d_n0: f64 = (var_qxf1_dn0 * var_weff);
        let eq43_e425_d_n1: f64 = (var_qxf1_dn1 * var_weff);
        let eq43_e425_d_n2: f64 = (var_qxf1_dn2 * var_weff);
        let eq43_e425_d_n3: f64 = (var_qxf1_dn3 * var_weff);
        let eq43_e425_d_n4: f64 = (var_qxf1_dn4 * var_weff);
        let eq43_e425_d_n5: f64 = (var_qxf1_dn5 * var_weff);
        let eq43_e425_d_n6: f64 = (var_qxf1_dn6 * var_weff);
        let eq43_e425_d_n7: f64 = (var_qxf1_dn7 * var_weff);
        let eq43_e425_d_n8: f64 = (var_qxf1_dn8 * var_weff);
        let eq43_e425_d_n9: f64 = (var_qxf1_dn9 * var_weff);
        let eq43_e425_d_b0: f64 = (var_qxf1_db0 * var_weff);
        let eq43_e425_d_b1: f64 = (var_qxf1_db1 * var_weff);
        let eq43_e425_d_b2: f64 = (var_qxf1_db2 * var_weff);
        let eq43_e425_d_b3: f64 = (var_qxf1_db3 * var_weff);
        let eq43_e425_d_b4: f64 = (var_qxf1_db4 * var_weff);
        let eq43_e425_d_b5: f64 = (var_qxf1_db5 * var_weff);
        let eq43_e425_d_b6: f64 = (var_qxf1_db6 * var_weff);
        let eq43_e425_d_b7: f64 = (var_qxf1_db7 * var_weff);
        let eq43_e426: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq43_e425);
        let eq43_value: f64 = eq43_e426;
        let eq43_node_derivatives: [f64; 10] = [(eq43_e425_d_n0 * ddt_scale), (eq43_e425_d_n1 * ddt_scale), (eq43_e425_d_n2 * ddt_scale), (eq43_e425_d_n3 * ddt_scale), (eq43_e425_d_n4 * ddt_scale), (eq43_e425_d_n5 * ddt_scale), (eq43_e425_d_n6 * ddt_scale), (eq43_e425_d_n7 * ddt_scale), (eq43_e425_d_n8 * ddt_scale), (eq43_e425_d_n9 * ddt_scale)];
        let eq43_branch_derivatives: [f64; 8] = [(eq43_e425_d_b0 * ddt_scale), (eq43_e425_d_b1 * ddt_scale), (eq43_e425_d_b2 * ddt_scale), (eq43_e425_d_b3 * ddt_scale), (eq43_e425_d_b4 * ddt_scale), (eq43_e425_d_b5 * ddt_scale), (eq43_e425_d_b6 * ddt_scale), (eq43_e425_d_b7 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard13: f64,
        var_guard20: f64,
        var_guard21: f64,
        var_qdc: f64,
        var_qdc_db0: f64,
        var_qdc_db1: f64,
        var_qdc_db2: f64,
        var_qdc_db3: f64,
        var_qdc_db4: f64,
        var_qdc_db5: f64,
        var_qdc_db6: f64,
        var_qdc_db7: f64,
        var_qdc_dn0: f64,
        var_qdc_dn1: f64,
        var_qdc_dn2: f64,
        var_qdc_dn3: f64,
        var_qdc_dn4: f64,
        var_qdc_dn5: f64,
        var_qdc_dn6: f64,
        var_qdc_dn7: f64,
        var_qdc_dn8: f64,
        var_qdc_dn9: f64,
        var_qde: f64,
        var_qde_db0: f64,
        var_qde_db1: f64,
        var_qde_db2: f64,
        var_qde_db3: f64,
        var_qde_db4: f64,
        var_qde_db5: f64,
        var_qde_db6: f64,
        var_qde_db7: f64,
        var_qde_dn0: f64,
        var_qde_dn1: f64,
        var_qde_dn2: f64,
        var_qde_dn3: f64,
        var_qde_dn4: f64,
        var_qde_dn5: f64,
        var_qde_dn6: f64,
        var_qde_dn7: f64,
        var_qde_dn8: f64,
        var_qde_dn9: f64,
        var_qjci_1: f64,
        var_qjci_1_db0: f64,
        var_qjci_1_db1: f64,
        var_qjci_1_db2: f64,
        var_qjci_1_db3: f64,
        var_qjci_1_db4: f64,
        var_qjci_1_db5: f64,
        var_qjci_1_db6: f64,
        var_qjci_1_db7: f64,
        var_qjci_1_dn0: f64,
        var_qjci_1_dn1: f64,
        var_qjci_1_dn2: f64,
        var_qjci_1_dn3: f64,
        var_qjci_1_dn4: f64,
        var_qjci_1_dn5: f64,
        var_qjci_1_dn6: f64,
        var_qjci_1_dn7: f64,
        var_qjci_1_dn8: f64,
        var_qjci_1_dn9: f64,
        var_qjcx_1: f64,
        var_qjcx_1_db0: f64,
        var_qjcx_1_db1: f64,
        var_qjcx_1_db2: f64,
        var_qjcx_1_db3: f64,
        var_qjcx_1_db4: f64,
        var_qjcx_1_db5: f64,
        var_qjcx_1_db6: f64,
        var_qjcx_1_db7: f64,
        var_qjcx_1_dn0: f64,
        var_qjcx_1_dn1: f64,
        var_qjcx_1_dn2: f64,
        var_qjcx_1_dn3: f64,
        var_qjcx_1_dn4: f64,
        var_qjcx_1_dn5: f64,
        var_qjcx_1_dn6: f64,
        var_qjcx_1_dn7: f64,
        var_qjcx_1_dn8: f64,
        var_qjcx_1_dn9: f64,
        var_qje: f64,
        var_qje_db0: f64,
        var_qje_db1: f64,
        var_qje_db2: f64,
        var_qje_db3: f64,
        var_qje_db4: f64,
        var_qje_db5: f64,
        var_qje_db6: f64,
        var_qje_db7: f64,
        var_qje_dn0: f64,
        var_qje_dn1: f64,
        var_qje_dn2: f64,
        var_qje_dn3: f64,
        var_qje_dn4: f64,
        var_qje_dn5: f64,
        var_qje_dn6: f64,
        var_qje_dn7: f64,
        var_qje_dn8: f64,
        var_qje_dn9: f64,
        var_qjs: f64,
        var_qjs_db0: f64,
        var_qjs_db1: f64,
        var_qjs_db2: f64,
        var_qjs_db3: f64,
        var_qjs_db4: f64,
        var_qjs_db5: f64,
        var_qjs_db6: f64,
        var_qjs_db7: f64,
        var_qjs_dn0: f64,
        var_qjs_dn1: f64,
        var_qjs_dn2: f64,
        var_qjs_dn3: f64,
        var_qjs_dn4: f64,
        var_qjs_dn5: f64,
        var_qjs_dn6: f64,
        var_qjs_dn7: f64,
        var_qjs_dn8: f64,
        var_qjs_dn9: f64,
        var_qxf1: f64,
        var_qxf1_db0: f64,
        var_qxf1_db1: f64,
        var_qxf1_db2: f64,
        var_qxf1_db3: f64,
        var_qxf1_db4: f64,
        var_qxf1_db5: f64,
        var_qxf1_db6: f64,
        var_qxf1_db7: f64,
        var_qxf1_dn0: f64,
        var_qxf1_dn1: f64,
        var_qxf1_dn2: f64,
        var_qxf1_dn3: f64,
        var_qxf1_dn4: f64,
        var_qxf1_dn5: f64,
        var_qxf1_dn6: f64,
        var_qxf1_dn7: f64,
        var_qxf1_dn8: f64,
        var_qxf1_dn9: f64,
        var_tff: f64,
        var_tff_db0: f64,
        var_tff_db1: f64,
        var_tff_db2: f64,
        var_tff_db3: f64,
        var_tff_db4: f64,
        var_tff_db5: f64,
        var_tff_db6: f64,
        var_tff_db7: f64,
        var_tff_dn0: f64,
        var_tff_dn1: f64,
        var_tff_dn2: f64,
        var_tff_dn3: f64,
        var_tff_dn4: f64,
        var_tff_dn5: f64,
        var_tff_dn6: f64,
        var_tff_dn7: f64,
        var_tff_dn8: f64,
        var_tff_dn9: f64,
        var_ttype: f64,
        var_ttype_db0: f64,
        var_ttype_db1: f64,
        var_ttype_db2: f64,
        var_ttype_db3: f64,
        var_ttype_db4: f64,
        var_ttype_db5: f64,
        var_ttype_db6: f64,
        var_ttype_db7: f64,
        var_ttype_dn0: f64,
        var_ttype_dn1: f64,
        var_ttype_dn2: f64,
        var_ttype_dn3: f64,
        var_ttype_dn4: f64,
        var_ttype_dn5: f64,
        var_ttype_dn6: f64,
        var_ttype_dn7: f64,
        var_ttype_dn8: f64,
        var_ttype_dn9: f64,
        var_weff: f64,
        var_weff_db0: f64,
        var_weff_db1: f64,
        var_weff_db2: f64,
        var_weff_db3: f64,
        var_weff_db4: f64,
        var_weff_db5: f64,
        var_weff_db6: f64,
        var_weff_db7: f64,
        var_weff_dn0: f64,
        var_weff_dn1: f64,
        var_weff_dn2: f64,
        var_weff_dn3: f64,
        var_weff_dn4: f64,
        var_weff_dn5: f64,
        var_weff_dn6: f64,
        var_weff_dn7: f64,
        var_weff_dn8: f64,
        var_weff_dn9: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq2_e98_q: f64 = (nv9 - 0.0);
        let eq2_e99: f64 = (p.p83 * (nv9 - 0.0));
        let eq2_e99_q: f64 = (p.p83 * eq2_e98_q);
        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * (p.p83),
        );
        let (eq5_e121, eq5_e121_d_n0, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n3, eq5_e121_d_n4, eq5_e121_d_n5, eq5_e121_d_n6, eq5_e121_d_n7, eq5_e121_d_n8, eq5_e121_d_n9, eq5_e121_d_b0, eq5_e121_d_b1, eq5_e121_d_b2, eq5_e121_d_b3, eq5_e121_d_b4, eq5_e121_d_b5, eq5_e121_d_b6, eq5_e121_d_b7, eq5_e121_q, eq5_e121_q_d_n0, eq5_e121_q_d_n1, eq5_e121_q_d_n2, eq5_e121_q_d_n3, eq5_e121_q_d_n4, eq5_e121_q_d_n5, eq5_e121_q_d_n6, eq5_e121_q_d_n7, eq5_e121_q_d_n8, eq5_e121_q_d_n9, eq5_e121_q_d_b0, eq5_e121_q_d_b1, eq5_e121_q_d_b2, eq5_e121_q_d_b3, eq5_e121_q_d_b4, eq5_e121_q_d_b5, eq5_e121_q_d_b6, eq5_e121_q_d_b7,) = {
    if (var_guard13 != 0.0) {
        let eq5_e118_q: f64 = (nv8 - 0.0);
        let eq5_e119: f64 = (var_tff * (nv8 - 0.0));
        let eq5_e119_d_n0: f64 = (var_tff_dn0 * (nv8 - 0.0));
        let eq5_e119_d_n1: f64 = (var_tff_dn1 * (nv8 - 0.0));
        let eq5_e119_d_n2: f64 = (var_tff_dn2 * (nv8 - 0.0));
        let eq5_e119_d_n3: f64 = (var_tff_dn3 * (nv8 - 0.0));
        let eq5_e119_d_n4: f64 = (var_tff_dn4 * (nv8 - 0.0));
        let eq5_e119_d_n5: f64 = (var_tff_dn5 * (nv8 - 0.0));
        let eq5_e119_d_n6: f64 = (var_tff_dn6 * (nv8 - 0.0));
        let eq5_e119_d_n7: f64 = (var_tff_dn7 * (nv8 - 0.0));
        let eq5_e119_d_n8: f64 = ((var_tff_dn8 * (nv8 - 0.0)) + var_tff);
        let eq5_e119_d_n9: f64 = (var_tff_dn9 * (nv8 - 0.0));
        let eq5_e119_d_b0: f64 = (var_tff_db0 * (nv8 - 0.0));
        let eq5_e119_d_b1: f64 = (var_tff_db1 * (nv8 - 0.0));
        let eq5_e119_d_b2: f64 = (var_tff_db2 * (nv8 - 0.0));
        let eq5_e119_d_b3: f64 = (var_tff_db3 * (nv8 - 0.0));
        let eq5_e119_d_b4: f64 = (var_tff_db4 * (nv8 - 0.0));
        let eq5_e119_d_b5: f64 = (var_tff_db5 * (nv8 - 0.0));
        let eq5_e119_d_b6: f64 = (var_tff_db6 * (nv8 - 0.0));
        let eq5_e119_d_b7: f64 = (var_tff_db7 * (nv8 - 0.0));
        let eq5_e119_q: f64 = (var_tff * eq5_e118_q);
        let eq5_e119_q_d_n0: f64 = (var_tff_dn0 * eq5_e118_q);
        let eq5_e119_q_d_n1: f64 = (var_tff_dn1 * eq5_e118_q);
        let eq5_e119_q_d_n2: f64 = (var_tff_dn2 * eq5_e118_q);
        let eq5_e119_q_d_n3: f64 = (var_tff_dn3 * eq5_e118_q);
        let eq5_e119_q_d_n4: f64 = (var_tff_dn4 * eq5_e118_q);
        let eq5_e119_q_d_n5: f64 = (var_tff_dn5 * eq5_e118_q);
        let eq5_e119_q_d_n6: f64 = (var_tff_dn6 * eq5_e118_q);
        let eq5_e119_q_d_n7: f64 = (var_tff_dn7 * eq5_e118_q);
        let eq5_e119_q_d_n8: f64 = ((var_tff_dn8 * eq5_e118_q) + var_tff);
        let eq5_e119_q_d_n9: f64 = (var_tff_dn9 * eq5_e118_q);
        let eq5_e119_q_d_b0: f64 = (var_tff_db0 * eq5_e118_q);
        let eq5_e119_q_d_b1: f64 = (var_tff_db1 * eq5_e118_q);
        let eq5_e119_q_d_b2: f64 = (var_tff_db2 * eq5_e118_q);
        let eq5_e119_q_d_b3: f64 = (var_tff_db3 * eq5_e118_q);
        let eq5_e119_q_d_b4: f64 = (var_tff_db4 * eq5_e118_q);
        let eq5_e119_q_d_b5: f64 = (var_tff_db5 * eq5_e118_q);
        let eq5_e119_q_d_b6: f64 = (var_tff_db6 * eq5_e118_q);
        let eq5_e119_q_d_b7: f64 = (var_tff_db7 * eq5_e118_q);
        (eq5_e119, eq5_e119_d_n0, eq5_e119_d_n1, eq5_e119_d_n2, eq5_e119_d_n3, eq5_e119_d_n4, eq5_e119_d_n5, eq5_e119_d_n6, eq5_e119_d_n7, eq5_e119_d_n8, eq5_e119_d_n9, eq5_e119_d_b0, eq5_e119_d_b1, eq5_e119_d_b2, eq5_e119_d_b3, eq5_e119_d_b4, eq5_e119_d_b5, eq5_e119_d_b6, eq5_e119_d_b7, eq5_e119_q, eq5_e119_q_d_n0, eq5_e119_q_d_n1, eq5_e119_q_d_n2, eq5_e119_q_d_n3, eq5_e119_q_d_n4, eq5_e119_q_d_n5, eq5_e119_q_d_n6, eq5_e119_q_d_n7, eq5_e119_q_d_n8, eq5_e119_q_d_n9, eq5_e119_q_d_b0, eq5_e119_q_d_b1, eq5_e119_q_d_b2, eq5_e119_q_d_b3, eq5_e119_q_d_b4, eq5_e119_q_d_b5, eq5_e119_q_d_b6, eq5_e119_q_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_reactive_node_derivatives: [f64; 10] = [eq5_e121_q_d_n0, eq5_e121_q_d_n1, eq5_e121_q_d_n2, eq5_e121_q_d_n3, eq5_e121_q_d_n4, eq5_e121_q_d_n5, eq5_e121_q_d_n6, eq5_e121_q_d_n7, eq5_e121_q_d_n8, eq5_e121_q_d_n9];
        let eq5_reactive_branch_derivatives: [f64; 8] = [eq5_e121_q_d_b0, eq5_e121_q_d_b1, eq5_e121_q_d_b2, eq5_e121_q_d_b3, eq5_e121_q_d_b4, eq5_e121_q_d_b5, eq5_e121_q_d_b6, eq5_e121_q_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            None,
            nodes,
            &eq5_reactive_node_derivatives,
            branches,
            &eq5_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq9_e154, eq9_e154_d_n3, eq9_e154_q,) = {
    if (var_guard20 != 0.0) {
        let eq9_e151: f64 = ((nv3 - 0.0) * p.p34);
        let eq9_e152_q: f64 = eq9_e151;
        (eq9_e151, p.p34, eq9_e152_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq9_e154_d_n3),
        );
        let (eq13_e195, eq13_e195_d_n3, eq13_e195_q,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq13_e192: f64 = (p.p34 * (nv3 - 0.0));
        let eq13_e193_q: f64 = eq13_e192;
        (eq13_e192, p.p34, eq13_e193_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq13_e195_d_n3),
        );
        let (eq15_e214, eq15_e214_d_n7, eq15_e214_q,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq15_e211: f64 = (p.p36 * (nv7 - 0.0));
        let eq15_e212_q: f64 = eq15_e211;
        (eq15_e211, p.p36, eq15_e212_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            None,
            nodes[7],
            multiplicity * (eq15_e214_d_n7),
        );
        let eq36_e384: f64 = (var_ttype * var_qje);
        let eq36_e384_d_n0: f64 = ((var_ttype_dn0 * var_qje) + (var_ttype * var_qje_dn0));
        let eq36_e384_d_n1: f64 = ((var_ttype_dn1 * var_qje) + (var_ttype * var_qje_dn1));
        let eq36_e384_d_n2: f64 = ((var_ttype_dn2 * var_qje) + (var_ttype * var_qje_dn2));
        let eq36_e384_d_n3: f64 = ((var_ttype_dn3 * var_qje) + (var_ttype * var_qje_dn3));
        let eq36_e384_d_n4: f64 = ((var_ttype_dn4 * var_qje) + (var_ttype * var_qje_dn4));
        let eq36_e384_d_n5: f64 = ((var_ttype_dn5 * var_qje) + (var_ttype * var_qje_dn5));
        let eq36_e384_d_n6: f64 = ((var_ttype_dn6 * var_qje) + (var_ttype * var_qje_dn6));
        let eq36_e384_d_n7: f64 = ((var_ttype_dn7 * var_qje) + (var_ttype * var_qje_dn7));
        let eq36_e384_d_n8: f64 = ((var_ttype_dn8 * var_qje) + (var_ttype * var_qje_dn8));
        let eq36_e384_d_n9: f64 = ((var_ttype_dn9 * var_qje) + (var_ttype * var_qje_dn9));
        let eq36_e384_d_b0: f64 = ((var_ttype_db0 * var_qje) + (var_ttype * var_qje_db0));
        let eq36_e384_d_b1: f64 = ((var_ttype_db1 * var_qje) + (var_ttype * var_qje_db1));
        let eq36_e384_d_b2: f64 = ((var_ttype_db2 * var_qje) + (var_ttype * var_qje_db2));
        let eq36_e384_d_b3: f64 = ((var_ttype_db3 * var_qje) + (var_ttype * var_qje_db3));
        let eq36_e384_d_b4: f64 = ((var_ttype_db4 * var_qje) + (var_ttype * var_qje_db4));
        let eq36_e384_d_b5: f64 = ((var_ttype_db5 * var_qje) + (var_ttype * var_qje_db5));
        let eq36_e384_d_b6: f64 = ((var_ttype_db6 * var_qje) + (var_ttype * var_qje_db6));
        let eq36_e384_d_b7: f64 = ((var_ttype_db7 * var_qje) + (var_ttype * var_qje_db7));
        let eq36_e386: f64 = (eq36_e384 * var_weff);
        let eq36_e386_d_n0: f64 = ((eq36_e384_d_n0 * var_weff) + (eq36_e384 * var_weff_dn0));
        let eq36_e386_d_n1: f64 = ((eq36_e384_d_n1 * var_weff) + (eq36_e384 * var_weff_dn1));
        let eq36_e386_d_n2: f64 = ((eq36_e384_d_n2 * var_weff) + (eq36_e384 * var_weff_dn2));
        let eq36_e386_d_n3: f64 = ((eq36_e384_d_n3 * var_weff) + (eq36_e384 * var_weff_dn3));
        let eq36_e386_d_n4: f64 = ((eq36_e384_d_n4 * var_weff) + (eq36_e384 * var_weff_dn4));
        let eq36_e386_d_n5: f64 = ((eq36_e384_d_n5 * var_weff) + (eq36_e384 * var_weff_dn5));
        let eq36_e386_d_n6: f64 = ((eq36_e384_d_n6 * var_weff) + (eq36_e384 * var_weff_dn6));
        let eq36_e386_d_n7: f64 = ((eq36_e384_d_n7 * var_weff) + (eq36_e384 * var_weff_dn7));
        let eq36_e386_d_n8: f64 = ((eq36_e384_d_n8 * var_weff) + (eq36_e384 * var_weff_dn8));
        let eq36_e386_d_n9: f64 = ((eq36_e384_d_n9 * var_weff) + (eq36_e384 * var_weff_dn9));
        let eq36_e386_d_b0: f64 = ((eq36_e384_d_b0 * var_weff) + (eq36_e384 * var_weff_db0));
        let eq36_e386_d_b1: f64 = ((eq36_e384_d_b1 * var_weff) + (eq36_e384 * var_weff_db1));
        let eq36_e386_d_b2: f64 = ((eq36_e384_d_b2 * var_weff) + (eq36_e384 * var_weff_db2));
        let eq36_e386_d_b3: f64 = ((eq36_e384_d_b3 * var_weff) + (eq36_e384 * var_weff_db3));
        let eq36_e386_d_b4: f64 = ((eq36_e384_d_b4 * var_weff) + (eq36_e384 * var_weff_db4));
        let eq36_e386_d_b5: f64 = ((eq36_e384_d_b5 * var_weff) + (eq36_e384 * var_weff_db5));
        let eq36_e386_d_b6: f64 = ((eq36_e384_d_b6 * var_weff) + (eq36_e384 * var_weff_db6));
        let eq36_e386_d_b7: f64 = ((eq36_e384_d_b7 * var_weff) + (eq36_e384 * var_weff_db7));
        let eq36_e387_q: f64 = eq36_e386;
        let eq36_reactive_node_derivatives: [f64; 10] = [eq36_e386_d_n0, eq36_e386_d_n1, eq36_e386_d_n2, eq36_e386_d_n3, eq36_e386_d_n4, eq36_e386_d_n5, eq36_e386_d_n6, eq36_e386_d_n7, eq36_e386_d_n8, eq36_e386_d_n9];
        let eq36_reactive_branch_derivatives: [f64; 8] = [eq36_e386_d_b0, eq36_e386_d_b1, eq36_e386_d_b2, eq36_e386_d_b3, eq36_e386_d_b4, eq36_e386_d_b5, eq36_e386_d_b6, eq36_e386_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e390: f64 = (var_ttype * var_qde);
        let eq37_e390_d_n0: f64 = ((var_ttype_dn0 * var_qde) + (var_ttype * var_qde_dn0));
        let eq37_e390_d_n1: f64 = ((var_ttype_dn1 * var_qde) + (var_ttype * var_qde_dn1));
        let eq37_e390_d_n2: f64 = ((var_ttype_dn2 * var_qde) + (var_ttype * var_qde_dn2));
        let eq37_e390_d_n3: f64 = ((var_ttype_dn3 * var_qde) + (var_ttype * var_qde_dn3));
        let eq37_e390_d_n4: f64 = ((var_ttype_dn4 * var_qde) + (var_ttype * var_qde_dn4));
        let eq37_e390_d_n5: f64 = ((var_ttype_dn5 * var_qde) + (var_ttype * var_qde_dn5));
        let eq37_e390_d_n6: f64 = ((var_ttype_dn6 * var_qde) + (var_ttype * var_qde_dn6));
        let eq37_e390_d_n7: f64 = ((var_ttype_dn7 * var_qde) + (var_ttype * var_qde_dn7));
        let eq37_e390_d_n8: f64 = ((var_ttype_dn8 * var_qde) + (var_ttype * var_qde_dn8));
        let eq37_e390_d_n9: f64 = ((var_ttype_dn9 * var_qde) + (var_ttype * var_qde_dn9));
        let eq37_e390_d_b0: f64 = ((var_ttype_db0 * var_qde) + (var_ttype * var_qde_db0));
        let eq37_e390_d_b1: f64 = ((var_ttype_db1 * var_qde) + (var_ttype * var_qde_db1));
        let eq37_e390_d_b2: f64 = ((var_ttype_db2 * var_qde) + (var_ttype * var_qde_db2));
        let eq37_e390_d_b3: f64 = ((var_ttype_db3 * var_qde) + (var_ttype * var_qde_db3));
        let eq37_e390_d_b4: f64 = ((var_ttype_db4 * var_qde) + (var_ttype * var_qde_db4));
        let eq37_e390_d_b5: f64 = ((var_ttype_db5 * var_qde) + (var_ttype * var_qde_db5));
        let eq37_e390_d_b6: f64 = ((var_ttype_db6 * var_qde) + (var_ttype * var_qde_db6));
        let eq37_e390_d_b7: f64 = ((var_ttype_db7 * var_qde) + (var_ttype * var_qde_db7));
        let eq37_e392: f64 = (eq37_e390 * var_weff);
        let eq37_e392_d_n0: f64 = ((eq37_e390_d_n0 * var_weff) + (eq37_e390 * var_weff_dn0));
        let eq37_e392_d_n1: f64 = ((eq37_e390_d_n1 * var_weff) + (eq37_e390 * var_weff_dn1));
        let eq37_e392_d_n2: f64 = ((eq37_e390_d_n2 * var_weff) + (eq37_e390 * var_weff_dn2));
        let eq37_e392_d_n3: f64 = ((eq37_e390_d_n3 * var_weff) + (eq37_e390 * var_weff_dn3));
        let eq37_e392_d_n4: f64 = ((eq37_e390_d_n4 * var_weff) + (eq37_e390 * var_weff_dn4));
        let eq37_e392_d_n5: f64 = ((eq37_e390_d_n5 * var_weff) + (eq37_e390 * var_weff_dn5));
        let eq37_e392_d_n6: f64 = ((eq37_e390_d_n6 * var_weff) + (eq37_e390 * var_weff_dn6));
        let eq37_e392_d_n7: f64 = ((eq37_e390_d_n7 * var_weff) + (eq37_e390 * var_weff_dn7));
        let eq37_e392_d_n8: f64 = ((eq37_e390_d_n8 * var_weff) + (eq37_e390 * var_weff_dn8));
        let eq37_e392_d_n9: f64 = ((eq37_e390_d_n9 * var_weff) + (eq37_e390 * var_weff_dn9));
        let eq37_e392_d_b0: f64 = ((eq37_e390_d_b0 * var_weff) + (eq37_e390 * var_weff_db0));
        let eq37_e392_d_b1: f64 = ((eq37_e390_d_b1 * var_weff) + (eq37_e390 * var_weff_db1));
        let eq37_e392_d_b2: f64 = ((eq37_e390_d_b2 * var_weff) + (eq37_e390 * var_weff_db2));
        let eq37_e392_d_b3: f64 = ((eq37_e390_d_b3 * var_weff) + (eq37_e390 * var_weff_db3));
        let eq37_e392_d_b4: f64 = ((eq37_e390_d_b4 * var_weff) + (eq37_e390 * var_weff_db4));
        let eq37_e392_d_b5: f64 = ((eq37_e390_d_b5 * var_weff) + (eq37_e390 * var_weff_db5));
        let eq37_e392_d_b6: f64 = ((eq37_e390_d_b6 * var_weff) + (eq37_e390 * var_weff_db6));
        let eq37_e392_d_b7: f64 = ((eq37_e390_d_b7 * var_weff) + (eq37_e390 * var_weff_db7));
        let eq37_e393_q: f64 = eq37_e392;
        let eq37_reactive_node_derivatives: [f64; 10] = [eq37_e392_d_n0, eq37_e392_d_n1, eq37_e392_d_n2, eq37_e392_d_n3, eq37_e392_d_n4, eq37_e392_d_n5, eq37_e392_d_n6, eq37_e392_d_n7, eq37_e392_d_n8, eq37_e392_d_n9];
        let eq37_reactive_branch_derivatives: [f64; 8] = [eq37_e392_d_b0, eq37_e392_d_b1, eq37_e392_d_b2, eq37_e392_d_b3, eq37_e392_d_b4, eq37_e392_d_b5, eq37_e392_d_b6, eq37_e392_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let eq38_e396: f64 = (var_ttype * var_qjcx_1);
        let eq38_e396_d_n0: f64 = ((var_ttype_dn0 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn0));
        let eq38_e396_d_n1: f64 = ((var_ttype_dn1 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn1));
        let eq38_e396_d_n2: f64 = ((var_ttype_dn2 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn2));
        let eq38_e396_d_n3: f64 = ((var_ttype_dn3 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn3));
        let eq38_e396_d_n4: f64 = ((var_ttype_dn4 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn4));
        let eq38_e396_d_n5: f64 = ((var_ttype_dn5 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn5));
        let eq38_e396_d_n6: f64 = ((var_ttype_dn6 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn6));
        let eq38_e396_d_n7: f64 = ((var_ttype_dn7 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn7));
        let eq38_e396_d_n8: f64 = ((var_ttype_dn8 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn8));
        let eq38_e396_d_n9: f64 = ((var_ttype_dn9 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn9));
        let eq38_e396_d_b0: f64 = ((var_ttype_db0 * var_qjcx_1) + (var_ttype * var_qjcx_1_db0));
        let eq38_e396_d_b1: f64 = ((var_ttype_db1 * var_qjcx_1) + (var_ttype * var_qjcx_1_db1));
        let eq38_e396_d_b2: f64 = ((var_ttype_db2 * var_qjcx_1) + (var_ttype * var_qjcx_1_db2));
        let eq38_e396_d_b3: f64 = ((var_ttype_db3 * var_qjcx_1) + (var_ttype * var_qjcx_1_db3));
        let eq38_e396_d_b4: f64 = ((var_ttype_db4 * var_qjcx_1) + (var_ttype * var_qjcx_1_db4));
        let eq38_e396_d_b5: f64 = ((var_ttype_db5 * var_qjcx_1) + (var_ttype * var_qjcx_1_db5));
        let eq38_e396_d_b6: f64 = ((var_ttype_db6 * var_qjcx_1) + (var_ttype * var_qjcx_1_db6));
        let eq38_e396_d_b7: f64 = ((var_ttype_db7 * var_qjcx_1) + (var_ttype * var_qjcx_1_db7));
        let eq38_e398: f64 = (eq38_e396 * var_weff);
        let eq38_e398_d_n0: f64 = ((eq38_e396_d_n0 * var_weff) + (eq38_e396 * var_weff_dn0));
        let eq38_e398_d_n1: f64 = ((eq38_e396_d_n1 * var_weff) + (eq38_e396 * var_weff_dn1));
        let eq38_e398_d_n2: f64 = ((eq38_e396_d_n2 * var_weff) + (eq38_e396 * var_weff_dn2));
        let eq38_e398_d_n3: f64 = ((eq38_e396_d_n3 * var_weff) + (eq38_e396 * var_weff_dn3));
        let eq38_e398_d_n4: f64 = ((eq38_e396_d_n4 * var_weff) + (eq38_e396 * var_weff_dn4));
        let eq38_e398_d_n5: f64 = ((eq38_e396_d_n5 * var_weff) + (eq38_e396 * var_weff_dn5));
        let eq38_e398_d_n6: f64 = ((eq38_e396_d_n6 * var_weff) + (eq38_e396 * var_weff_dn6));
        let eq38_e398_d_n7: f64 = ((eq38_e396_d_n7 * var_weff) + (eq38_e396 * var_weff_dn7));
        let eq38_e398_d_n8: f64 = ((eq38_e396_d_n8 * var_weff) + (eq38_e396 * var_weff_dn8));
        let eq38_e398_d_n9: f64 = ((eq38_e396_d_n9 * var_weff) + (eq38_e396 * var_weff_dn9));
        let eq38_e398_d_b0: f64 = ((eq38_e396_d_b0 * var_weff) + (eq38_e396 * var_weff_db0));
        let eq38_e398_d_b1: f64 = ((eq38_e396_d_b1 * var_weff) + (eq38_e396 * var_weff_db1));
        let eq38_e398_d_b2: f64 = ((eq38_e396_d_b2 * var_weff) + (eq38_e396 * var_weff_db2));
        let eq38_e398_d_b3: f64 = ((eq38_e396_d_b3 * var_weff) + (eq38_e396 * var_weff_db3));
        let eq38_e398_d_b4: f64 = ((eq38_e396_d_b4 * var_weff) + (eq38_e396 * var_weff_db4));
        let eq38_e398_d_b5: f64 = ((eq38_e396_d_b5 * var_weff) + (eq38_e396 * var_weff_db5));
        let eq38_e398_d_b6: f64 = ((eq38_e396_d_b6 * var_weff) + (eq38_e396 * var_weff_db6));
        let eq38_e398_d_b7: f64 = ((eq38_e396_d_b7 * var_weff) + (eq38_e396 * var_weff_db7));
        let eq38_e399_q: f64 = eq38_e398;
        let eq38_reactive_node_derivatives: [f64; 10] = [eq38_e398_d_n0, eq38_e398_d_n1, eq38_e398_d_n2, eq38_e398_d_n3, eq38_e398_d_n4, eq38_e398_d_n5, eq38_e398_d_n6, eq38_e398_d_n7, eq38_e398_d_n8, eq38_e398_d_n9];
        let eq38_reactive_branch_derivatives: [f64; 8] = [eq38_e398_d_b0, eq38_e398_d_b1, eq38_e398_d_b2, eq38_e398_d_b3, eq38_e398_d_b4, eq38_e398_d_b5, eq38_e398_d_b6, eq38_e398_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let eq39_e402: f64 = (var_ttype * var_qjci_1);
        let eq39_e402_d_n0: f64 = ((var_ttype_dn0 * var_qjci_1) + (var_ttype * var_qjci_1_dn0));
        let eq39_e402_d_n1: f64 = ((var_ttype_dn1 * var_qjci_1) + (var_ttype * var_qjci_1_dn1));
        let eq39_e402_d_n2: f64 = ((var_ttype_dn2 * var_qjci_1) + (var_ttype * var_qjci_1_dn2));
        let eq39_e402_d_n3: f64 = ((var_ttype_dn3 * var_qjci_1) + (var_ttype * var_qjci_1_dn3));
        let eq39_e402_d_n4: f64 = ((var_ttype_dn4 * var_qjci_1) + (var_ttype * var_qjci_1_dn4));
        let eq39_e402_d_n5: f64 = ((var_ttype_dn5 * var_qjci_1) + (var_ttype * var_qjci_1_dn5));
        let eq39_e402_d_n6: f64 = ((var_ttype_dn6 * var_qjci_1) + (var_ttype * var_qjci_1_dn6));
        let eq39_e402_d_n7: f64 = ((var_ttype_dn7 * var_qjci_1) + (var_ttype * var_qjci_1_dn7));
        let eq39_e402_d_n8: f64 = ((var_ttype_dn8 * var_qjci_1) + (var_ttype * var_qjci_1_dn8));
        let eq39_e402_d_n9: f64 = ((var_ttype_dn9 * var_qjci_1) + (var_ttype * var_qjci_1_dn9));
        let eq39_e402_d_b0: f64 = ((var_ttype_db0 * var_qjci_1) + (var_ttype * var_qjci_1_db0));
        let eq39_e402_d_b1: f64 = ((var_ttype_db1 * var_qjci_1) + (var_ttype * var_qjci_1_db1));
        let eq39_e402_d_b2: f64 = ((var_ttype_db2 * var_qjci_1) + (var_ttype * var_qjci_1_db2));
        let eq39_e402_d_b3: f64 = ((var_ttype_db3 * var_qjci_1) + (var_ttype * var_qjci_1_db3));
        let eq39_e402_d_b4: f64 = ((var_ttype_db4 * var_qjci_1) + (var_ttype * var_qjci_1_db4));
        let eq39_e402_d_b5: f64 = ((var_ttype_db5 * var_qjci_1) + (var_ttype * var_qjci_1_db5));
        let eq39_e402_d_b6: f64 = ((var_ttype_db6 * var_qjci_1) + (var_ttype * var_qjci_1_db6));
        let eq39_e402_d_b7: f64 = ((var_ttype_db7 * var_qjci_1) + (var_ttype * var_qjci_1_db7));
        let eq39_e404: f64 = (eq39_e402 * var_weff);
        let eq39_e404_d_n0: f64 = ((eq39_e402_d_n0 * var_weff) + (eq39_e402 * var_weff_dn0));
        let eq39_e404_d_n1: f64 = ((eq39_e402_d_n1 * var_weff) + (eq39_e402 * var_weff_dn1));
        let eq39_e404_d_n2: f64 = ((eq39_e402_d_n2 * var_weff) + (eq39_e402 * var_weff_dn2));
        let eq39_e404_d_n3: f64 = ((eq39_e402_d_n3 * var_weff) + (eq39_e402 * var_weff_dn3));
        let eq39_e404_d_n4: f64 = ((eq39_e402_d_n4 * var_weff) + (eq39_e402 * var_weff_dn4));
        let eq39_e404_d_n5: f64 = ((eq39_e402_d_n5 * var_weff) + (eq39_e402 * var_weff_dn5));
        let eq39_e404_d_n6: f64 = ((eq39_e402_d_n6 * var_weff) + (eq39_e402 * var_weff_dn6));
        let eq39_e404_d_n7: f64 = ((eq39_e402_d_n7 * var_weff) + (eq39_e402 * var_weff_dn7));
        let eq39_e404_d_n8: f64 = ((eq39_e402_d_n8 * var_weff) + (eq39_e402 * var_weff_dn8));
        let eq39_e404_d_n9: f64 = ((eq39_e402_d_n9 * var_weff) + (eq39_e402 * var_weff_dn9));
        let eq39_e404_d_b0: f64 = ((eq39_e402_d_b0 * var_weff) + (eq39_e402 * var_weff_db0));
        let eq39_e404_d_b1: f64 = ((eq39_e402_d_b1 * var_weff) + (eq39_e402 * var_weff_db1));
        let eq39_e404_d_b2: f64 = ((eq39_e402_d_b2 * var_weff) + (eq39_e402 * var_weff_db2));
        let eq39_e404_d_b3: f64 = ((eq39_e402_d_b3 * var_weff) + (eq39_e402 * var_weff_db3));
        let eq39_e404_d_b4: f64 = ((eq39_e402_d_b4 * var_weff) + (eq39_e402 * var_weff_db4));
        let eq39_e404_d_b5: f64 = ((eq39_e402_d_b5 * var_weff) + (eq39_e402 * var_weff_db5));
        let eq39_e404_d_b6: f64 = ((eq39_e402_d_b6 * var_weff) + (eq39_e402 * var_weff_db6));
        let eq39_e404_d_b7: f64 = ((eq39_e402_d_b7 * var_weff) + (eq39_e402 * var_weff_db7));
        let eq39_e405_q: f64 = eq39_e404;
        let eq39_reactive_node_derivatives: [f64; 10] = [eq39_e404_d_n0, eq39_e404_d_n1, eq39_e404_d_n2, eq39_e404_d_n3, eq39_e404_d_n4, eq39_e404_d_n5, eq39_e404_d_n6, eq39_e404_d_n7, eq39_e404_d_n8, eq39_e404_d_n9];
        let eq39_reactive_branch_derivatives: [f64; 8] = [eq39_e404_d_b0, eq39_e404_d_b1, eq39_e404_d_b2, eq39_e404_d_b3, eq39_e404_d_b4, eq39_e404_d_b5, eq39_e404_d_b6, eq39_e404_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let eq40_e408: f64 = (var_ttype * var_qdc);
        let eq40_e408_d_n0: f64 = ((var_ttype_dn0 * var_qdc) + (var_ttype * var_qdc_dn0));
        let eq40_e408_d_n1: f64 = ((var_ttype_dn1 * var_qdc) + (var_ttype * var_qdc_dn1));
        let eq40_e408_d_n2: f64 = ((var_ttype_dn2 * var_qdc) + (var_ttype * var_qdc_dn2));
        let eq40_e408_d_n3: f64 = ((var_ttype_dn3 * var_qdc) + (var_ttype * var_qdc_dn3));
        let eq40_e408_d_n4: f64 = ((var_ttype_dn4 * var_qdc) + (var_ttype * var_qdc_dn4));
        let eq40_e408_d_n5: f64 = ((var_ttype_dn5 * var_qdc) + (var_ttype * var_qdc_dn5));
        let eq40_e408_d_n6: f64 = ((var_ttype_dn6 * var_qdc) + (var_ttype * var_qdc_dn6));
        let eq40_e408_d_n7: f64 = ((var_ttype_dn7 * var_qdc) + (var_ttype * var_qdc_dn7));
        let eq40_e408_d_n8: f64 = ((var_ttype_dn8 * var_qdc) + (var_ttype * var_qdc_dn8));
        let eq40_e408_d_n9: f64 = ((var_ttype_dn9 * var_qdc) + (var_ttype * var_qdc_dn9));
        let eq40_e408_d_b0: f64 = ((var_ttype_db0 * var_qdc) + (var_ttype * var_qdc_db0));
        let eq40_e408_d_b1: f64 = ((var_ttype_db1 * var_qdc) + (var_ttype * var_qdc_db1));
        let eq40_e408_d_b2: f64 = ((var_ttype_db2 * var_qdc) + (var_ttype * var_qdc_db2));
        let eq40_e408_d_b3: f64 = ((var_ttype_db3 * var_qdc) + (var_ttype * var_qdc_db3));
        let eq40_e408_d_b4: f64 = ((var_ttype_db4 * var_qdc) + (var_ttype * var_qdc_db4));
        let eq40_e408_d_b5: f64 = ((var_ttype_db5 * var_qdc) + (var_ttype * var_qdc_db5));
        let eq40_e408_d_b6: f64 = ((var_ttype_db6 * var_qdc) + (var_ttype * var_qdc_db6));
        let eq40_e408_d_b7: f64 = ((var_ttype_db7 * var_qdc) + (var_ttype * var_qdc_db7));
        let eq40_e410: f64 = (eq40_e408 * var_weff);
        let eq40_e410_d_n0: f64 = ((eq40_e408_d_n0 * var_weff) + (eq40_e408 * var_weff_dn0));
        let eq40_e410_d_n1: f64 = ((eq40_e408_d_n1 * var_weff) + (eq40_e408 * var_weff_dn1));
        let eq40_e410_d_n2: f64 = ((eq40_e408_d_n2 * var_weff) + (eq40_e408 * var_weff_dn2));
        let eq40_e410_d_n3: f64 = ((eq40_e408_d_n3 * var_weff) + (eq40_e408 * var_weff_dn3));
        let eq40_e410_d_n4: f64 = ((eq40_e408_d_n4 * var_weff) + (eq40_e408 * var_weff_dn4));
        let eq40_e410_d_n5: f64 = ((eq40_e408_d_n5 * var_weff) + (eq40_e408 * var_weff_dn5));
        let eq40_e410_d_n6: f64 = ((eq40_e408_d_n6 * var_weff) + (eq40_e408 * var_weff_dn6));
        let eq40_e410_d_n7: f64 = ((eq40_e408_d_n7 * var_weff) + (eq40_e408 * var_weff_dn7));
        let eq40_e410_d_n8: f64 = ((eq40_e408_d_n8 * var_weff) + (eq40_e408 * var_weff_dn8));
        let eq40_e410_d_n9: f64 = ((eq40_e408_d_n9 * var_weff) + (eq40_e408 * var_weff_dn9));
        let eq40_e410_d_b0: f64 = ((eq40_e408_d_b0 * var_weff) + (eq40_e408 * var_weff_db0));
        let eq40_e410_d_b1: f64 = ((eq40_e408_d_b1 * var_weff) + (eq40_e408 * var_weff_db1));
        let eq40_e410_d_b2: f64 = ((eq40_e408_d_b2 * var_weff) + (eq40_e408 * var_weff_db2));
        let eq40_e410_d_b3: f64 = ((eq40_e408_d_b3 * var_weff) + (eq40_e408 * var_weff_db3));
        let eq40_e410_d_b4: f64 = ((eq40_e408_d_b4 * var_weff) + (eq40_e408 * var_weff_db4));
        let eq40_e410_d_b5: f64 = ((eq40_e408_d_b5 * var_weff) + (eq40_e408 * var_weff_db5));
        let eq40_e410_d_b6: f64 = ((eq40_e408_d_b6 * var_weff) + (eq40_e408 * var_weff_db6));
        let eq40_e410_d_b7: f64 = ((eq40_e408_d_b7 * var_weff) + (eq40_e408 * var_weff_db7));
        let eq40_e411_q: f64 = eq40_e410;
        let eq40_reactive_node_derivatives: [f64; 10] = [eq40_e410_d_n0, eq40_e410_d_n1, eq40_e410_d_n2, eq40_e410_d_n3, eq40_e410_d_n4, eq40_e410_d_n5, eq40_e410_d_n6, eq40_e410_d_n7, eq40_e410_d_n8, eq40_e410_d_n9];
        let eq40_reactive_branch_derivatives: [f64; 8] = [eq40_e410_d_b0, eq40_e410_d_b1, eq40_e410_d_b2, eq40_e410_d_b3, eq40_e410_d_b4, eq40_e410_d_b5, eq40_e410_d_b6, eq40_e410_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e414: f64 = (var_ttype * var_qjs);
        let eq41_e414_d_n0: f64 = ((var_ttype_dn0 * var_qjs) + (var_ttype * var_qjs_dn0));
        let eq41_e414_d_n1: f64 = ((var_ttype_dn1 * var_qjs) + (var_ttype * var_qjs_dn1));
        let eq41_e414_d_n2: f64 = ((var_ttype_dn2 * var_qjs) + (var_ttype * var_qjs_dn2));
        let eq41_e414_d_n3: f64 = ((var_ttype_dn3 * var_qjs) + (var_ttype * var_qjs_dn3));
        let eq41_e414_d_n4: f64 = ((var_ttype_dn4 * var_qjs) + (var_ttype * var_qjs_dn4));
        let eq41_e414_d_n5: f64 = ((var_ttype_dn5 * var_qjs) + (var_ttype * var_qjs_dn5));
        let eq41_e414_d_n6: f64 = ((var_ttype_dn6 * var_qjs) + (var_ttype * var_qjs_dn6));
        let eq41_e414_d_n7: f64 = ((var_ttype_dn7 * var_qjs) + (var_ttype * var_qjs_dn7));
        let eq41_e414_d_n8: f64 = ((var_ttype_dn8 * var_qjs) + (var_ttype * var_qjs_dn8));
        let eq41_e414_d_n9: f64 = ((var_ttype_dn9 * var_qjs) + (var_ttype * var_qjs_dn9));
        let eq41_e414_d_b0: f64 = ((var_ttype_db0 * var_qjs) + (var_ttype * var_qjs_db0));
        let eq41_e414_d_b1: f64 = ((var_ttype_db1 * var_qjs) + (var_ttype * var_qjs_db1));
        let eq41_e414_d_b2: f64 = ((var_ttype_db2 * var_qjs) + (var_ttype * var_qjs_db2));
        let eq41_e414_d_b3: f64 = ((var_ttype_db3 * var_qjs) + (var_ttype * var_qjs_db3));
        let eq41_e414_d_b4: f64 = ((var_ttype_db4 * var_qjs) + (var_ttype * var_qjs_db4));
        let eq41_e414_d_b5: f64 = ((var_ttype_db5 * var_qjs) + (var_ttype * var_qjs_db5));
        let eq41_e414_d_b6: f64 = ((var_ttype_db6 * var_qjs) + (var_ttype * var_qjs_db6));
        let eq41_e414_d_b7: f64 = ((var_ttype_db7 * var_qjs) + (var_ttype * var_qjs_db7));
        let eq41_e416: f64 = (eq41_e414 * var_weff);
        let eq41_e416_d_n0: f64 = ((eq41_e414_d_n0 * var_weff) + (eq41_e414 * var_weff_dn0));
        let eq41_e416_d_n1: f64 = ((eq41_e414_d_n1 * var_weff) + (eq41_e414 * var_weff_dn1));
        let eq41_e416_d_n2: f64 = ((eq41_e414_d_n2 * var_weff) + (eq41_e414 * var_weff_dn2));
        let eq41_e416_d_n3: f64 = ((eq41_e414_d_n3 * var_weff) + (eq41_e414 * var_weff_dn3));
        let eq41_e416_d_n4: f64 = ((eq41_e414_d_n4 * var_weff) + (eq41_e414 * var_weff_dn4));
        let eq41_e416_d_n5: f64 = ((eq41_e414_d_n5 * var_weff) + (eq41_e414 * var_weff_dn5));
        let eq41_e416_d_n6: f64 = ((eq41_e414_d_n6 * var_weff) + (eq41_e414 * var_weff_dn6));
        let eq41_e416_d_n7: f64 = ((eq41_e414_d_n7 * var_weff) + (eq41_e414 * var_weff_dn7));
        let eq41_e416_d_n8: f64 = ((eq41_e414_d_n8 * var_weff) + (eq41_e414 * var_weff_dn8));
        let eq41_e416_d_n9: f64 = ((eq41_e414_d_n9 * var_weff) + (eq41_e414 * var_weff_dn9));
        let eq41_e416_d_b0: f64 = ((eq41_e414_d_b0 * var_weff) + (eq41_e414 * var_weff_db0));
        let eq41_e416_d_b1: f64 = ((eq41_e414_d_b1 * var_weff) + (eq41_e414 * var_weff_db1));
        let eq41_e416_d_b2: f64 = ((eq41_e414_d_b2 * var_weff) + (eq41_e414 * var_weff_db2));
        let eq41_e416_d_b3: f64 = ((eq41_e414_d_b3 * var_weff) + (eq41_e414 * var_weff_db3));
        let eq41_e416_d_b4: f64 = ((eq41_e414_d_b4 * var_weff) + (eq41_e414 * var_weff_db4));
        let eq41_e416_d_b5: f64 = ((eq41_e414_d_b5 * var_weff) + (eq41_e414 * var_weff_db5));
        let eq41_e416_d_b6: f64 = ((eq41_e414_d_b6 * var_weff) + (eq41_e414 * var_weff_db6));
        let eq41_e416_d_b7: f64 = ((eq41_e414_d_b7 * var_weff) + (eq41_e414 * var_weff_db7));
        let eq41_e417_q: f64 = eq41_e416;
        let eq41_reactive_node_derivatives: [f64; 10] = [eq41_e416_d_n0, eq41_e416_d_n1, eq41_e416_d_n2, eq41_e416_d_n3, eq41_e416_d_n4, eq41_e416_d_n5, eq41_e416_d_n6, eq41_e416_d_n7, eq41_e416_d_n8, eq41_e416_d_n9];
        let eq41_reactive_branch_derivatives: [f64; 8] = [eq41_e416_d_b0, eq41_e416_d_b1, eq41_e416_d_b2, eq41_e416_d_b3, eq41_e416_d_b4, eq41_e416_d_b5, eq41_e416_d_b6, eq41_e416_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[4]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e419: f64 = (-var_qxf1);
        let eq42_e421: f64 = (eq42_e419 * var_weff);
        let eq42_e421_d_n0: f64 = (((-var_qxf1_dn0) * var_weff) + (eq42_e419 * var_weff_dn0));
        let eq42_e421_d_n1: f64 = (((-var_qxf1_dn1) * var_weff) + (eq42_e419 * var_weff_dn1));
        let eq42_e421_d_n2: f64 = (((-var_qxf1_dn2) * var_weff) + (eq42_e419 * var_weff_dn2));
        let eq42_e421_d_n3: f64 = (((-var_qxf1_dn3) * var_weff) + (eq42_e419 * var_weff_dn3));
        let eq42_e421_d_n4: f64 = (((-var_qxf1_dn4) * var_weff) + (eq42_e419 * var_weff_dn4));
        let eq42_e421_d_n5: f64 = (((-var_qxf1_dn5) * var_weff) + (eq42_e419 * var_weff_dn5));
        let eq42_e421_d_n6: f64 = (((-var_qxf1_dn6) * var_weff) + (eq42_e419 * var_weff_dn6));
        let eq42_e421_d_n7: f64 = (((-var_qxf1_dn7) * var_weff) + (eq42_e419 * var_weff_dn7));
        let eq42_e421_d_n8: f64 = (((-var_qxf1_dn8) * var_weff) + (eq42_e419 * var_weff_dn8));
        let eq42_e421_d_n9: f64 = (((-var_qxf1_dn9) * var_weff) + (eq42_e419 * var_weff_dn9));
        let eq42_e421_d_b0: f64 = (((-var_qxf1_db0) * var_weff) + (eq42_e419 * var_weff_db0));
        let eq42_e421_d_b1: f64 = (((-var_qxf1_db1) * var_weff) + (eq42_e419 * var_weff_db1));
        let eq42_e421_d_b2: f64 = (((-var_qxf1_db2) * var_weff) + (eq42_e419 * var_weff_db2));
        let eq42_e421_d_b3: f64 = (((-var_qxf1_db3) * var_weff) + (eq42_e419 * var_weff_db3));
        let eq42_e421_d_b4: f64 = (((-var_qxf1_db4) * var_weff) + (eq42_e419 * var_weff_db4));
        let eq42_e421_d_b5: f64 = (((-var_qxf1_db5) * var_weff) + (eq42_e419 * var_weff_db5));
        let eq42_e421_d_b6: f64 = (((-var_qxf1_db6) * var_weff) + (eq42_e419 * var_weff_db6));
        let eq42_e421_d_b7: f64 = (((-var_qxf1_db7) * var_weff) + (eq42_e419 * var_weff_db7));
        let eq42_e422_q: f64 = eq42_e421;
        let eq42_reactive_node_derivatives: [f64; 10] = [eq42_e421_d_n0, eq42_e421_d_n1, eq42_e421_d_n2, eq42_e421_d_n3, eq42_e421_d_n4, eq42_e421_d_n5, eq42_e421_d_n6, eq42_e421_d_n7, eq42_e421_d_n8, eq42_e421_d_n9];
        let eq42_reactive_branch_derivatives: [f64; 8] = [eq42_e421_d_b0, eq42_e421_d_b1, eq42_e421_d_b2, eq42_e421_d_b3, eq42_e421_d_b4, eq42_e421_d_b5, eq42_e421_d_b6, eq42_e421_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e425: f64 = (var_qxf1 * var_weff);
        let eq43_e425_d_n0: f64 = ((var_qxf1_dn0 * var_weff) + (var_qxf1 * var_weff_dn0));
        let eq43_e425_d_n1: f64 = ((var_qxf1_dn1 * var_weff) + (var_qxf1 * var_weff_dn1));
        let eq43_e425_d_n2: f64 = ((var_qxf1_dn2 * var_weff) + (var_qxf1 * var_weff_dn2));
        let eq43_e425_d_n3: f64 = ((var_qxf1_dn3 * var_weff) + (var_qxf1 * var_weff_dn3));
        let eq43_e425_d_n4: f64 = ((var_qxf1_dn4 * var_weff) + (var_qxf1 * var_weff_dn4));
        let eq43_e425_d_n5: f64 = ((var_qxf1_dn5 * var_weff) + (var_qxf1 * var_weff_dn5));
        let eq43_e425_d_n6: f64 = ((var_qxf1_dn6 * var_weff) + (var_qxf1 * var_weff_dn6));
        let eq43_e425_d_n7: f64 = ((var_qxf1_dn7 * var_weff) + (var_qxf1 * var_weff_dn7));
        let eq43_e425_d_n8: f64 = ((var_qxf1_dn8 * var_weff) + (var_qxf1 * var_weff_dn8));
        let eq43_e425_d_n9: f64 = ((var_qxf1_dn9 * var_weff) + (var_qxf1 * var_weff_dn9));
        let eq43_e425_d_b0: f64 = ((var_qxf1_db0 * var_weff) + (var_qxf1 * var_weff_db0));
        let eq43_e425_d_b1: f64 = ((var_qxf1_db1 * var_weff) + (var_qxf1 * var_weff_db1));
        let eq43_e425_d_b2: f64 = ((var_qxf1_db2 * var_weff) + (var_qxf1 * var_weff_db2));
        let eq43_e425_d_b3: f64 = ((var_qxf1_db3 * var_weff) + (var_qxf1 * var_weff_db3));
        let eq43_e425_d_b4: f64 = ((var_qxf1_db4 * var_weff) + (var_qxf1 * var_weff_db4));
        let eq43_e425_d_b5: f64 = ((var_qxf1_db5 * var_weff) + (var_qxf1 * var_weff_db5));
        let eq43_e425_d_b6: f64 = ((var_qxf1_db6 * var_weff) + (var_qxf1 * var_weff_db6));
        let eq43_e425_d_b7: f64 = ((var_qxf1_db7 * var_weff) + (var_qxf1 * var_weff_db7));
        let eq43_e426_q: f64 = eq43_e425;
        let eq43_reactive_node_derivatives: [f64; 10] = [eq43_e425_d_n0, eq43_e425_d_n1, eq43_e425_d_n2, eq43_e425_d_n3, eq43_e425_d_n4, eq43_e425_d_n5, eq43_e425_d_n6, eq43_e425_d_n7, eq43_e425_d_n8, eq43_e425_d_n9];
        let eq43_reactive_branch_derivatives: [f64; 8] = [eq43_e425_d_b0, eq43_e425_d_b1, eq43_e425_d_b2, eq43_e425_d_b3, eq43_e425_d_b4, eq43_e425_d_b5, eq43_e425_d_b6, eq43_e425_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
