#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_76(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard489: f64,
        var_ndibot_i: f64,
        var_ndibot_i_db0: f64,
        var_ndibot_i_db1: f64,
        var_ndibot_i_db2: f64,
        var_ndibot_i_db3: f64,
        var_ndibot_i_dn0: f64,
        var_ndibot_i_dn1: f64,
        var_ndibot_i_dn2: f64,
        var_ndibot_i_dn3: f64,
        var_ndibot_i_dn4: f64,
        var_ndibot_i_dn5: f64,
        var_nfabot_i: f64,
        var_nfabot_i_db0: f64,
        var_nfabot_i_db1: f64,
        var_nfabot_i_db2: f64,
        var_nfabot_i_db3: f64,
        var_nfabot_i_dn0: f64,
        var_nfabot_i_dn1: f64,
        var_nfabot_i_dn2: f64,
        var_nfabot_i_dn3: f64,
        var_nfabot_i_dn4: f64,
        var_nfabot_i_dn5: f64,
        var_nfagat_i: f64,
        var_nfagat_i_db0: f64,
        var_nfagat_i_db1: f64,
        var_nfagat_i_db2: f64,
        var_nfagat_i_db3: f64,
        var_nfagat_i_dn0: f64,
        var_nfagat_i_dn1: f64,
        var_nfagat_i_dn2: f64,
        var_nfagat_i_dn3: f64,
        var_nfagat_i_dn4: f64,
        var_nfagat_i_dn5: f64,
        var_nin: f64,
        var_nin_db0: f64,
        var_nin_db1: f64,
        var_nin_db2: f64,
        var_nin_db3: f64,
        var_nin_dn0: f64,
        var_nin_dn1: f64,
        var_nin_dn2: f64,
        var_nin_dn3: f64,
        var_nin_dn4: f64,
        var_nin_dn5: f64,
        var_phitdinv: f64,
        var_phitdinv_db0: f64,
        var_phitdinv_db1: f64,
        var_phitdinv_db2: f64,
        var_phitdinv_db3: f64,
        var_phitdinv_dn0: f64,
        var_phitdinv_dn1: f64,
        var_phitdinv_dn2: f64,
        var_phitdinv_dn3: f64,
        var_phitdinv_dn4: f64,
        var_phitdinv_dn5: f64,
        var_vmax: f64,
        var_vmax_db0: f64,
        var_vmax_db1: f64,
        var_vmax_db2: f64,
        var_vmax_db3: f64,
        var_vmax_dn0: f64,
        var_vmax_dn1: f64,
        var_vmax_dn2: f64,
        var_vmax_dn3: f64,
        var_vmax_dn4: f64,
        var_vmax_dn5: f64,
        var_guard492_slot: &mut f64,
        var_guard492_db0_slot: &mut f64,
        var_guard492_db1_slot: &mut f64,
        var_guard492_db2_slot: &mut f64,
        var_guard492_db3_slot: &mut f64,
        var_guard492_dn0_slot: &mut f64,
        var_guard492_dn1_slot: &mut f64,
        var_guard492_dn2_slot: &mut f64,
        var_guard492_dn3_slot: &mut f64,
        var_guard492_dn4_slot: &mut f64,
        var_guard492_dn5_slot: &mut f64,
        var_guard492_rdb0_slot: &mut f64,
        var_guard492_rdb1_slot: &mut f64,
        var_guard492_rdb2_slot: &mut f64,
        var_guard492_rdb3_slot: &mut f64,
        var_guard492_rdn0_slot: &mut f64,
        var_guard492_rdn1_slot: &mut f64,
        var_guard492_rdn2_slot: &mut f64,
        var_guard492_rdn3_slot: &mut f64,
        var_guard492_rdn4_slot: &mut f64,
        var_guard492_rdn5_slot: &mut f64,
        var_guard492_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj0_rdb0_slot: &mut f64,
        var_nj0_rdb1_slot: &mut f64,
        var_nj0_rdb2_slot: &mut f64,
        var_nj0_rdb3_slot: &mut f64,
        var_nj0_rdn0_slot: &mut f64,
        var_nj0_rdn1_slot: &mut f64,
        var_nj0_rdn2_slot: &mut f64,
        var_nj0_rdn3_slot: &mut f64,
        var_nj0_rdn4_slot: &mut f64,
        var_nj0_rdn5_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nj1_rdb0_slot: &mut f64,
        var_nj1_rdb1_slot: &mut f64,
        var_nj1_rdb2_slot: &mut f64,
        var_nj1_rdb3_slot: &mut f64,
        var_nj1_rdn0_slot: &mut f64,
        var_nj1_rdn1_slot: &mut f64,
        var_nj1_rdn2_slot: &mut f64,
        var_nj1_rdn3_slot: &mut f64,
        var_nj1_rdn4_slot: &mut f64,
        var_nj1_rdn5_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_nja10_rdb0_slot: &mut f64,
        var_nja10_rdb1_slot: &mut f64,
        var_nja10_rdb2_slot: &mut f64,
        var_nja10_rdb3_slot: &mut f64,
        var_nja10_rdn0_slot: &mut f64,
        var_nja10_rdn1_slot: &mut f64,
        var_nja10_rdn2_slot: &mut f64,
        var_nja10_rdn3_slot: &mut f64,
        var_nja10_rdn4_slot: &mut f64,
        var_nja10_rdn5_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_db0_slot: &mut f64,
        var_pnn0_db1_slot: &mut f64,
        var_pnn0_db2_slot: &mut f64,
        var_pnn0_db3_slot: &mut f64,
        var_pnn0_dn0_slot: &mut f64,
        var_pnn0_dn1_slot: &mut f64,
        var_pnn0_dn2_slot: &mut f64,
        var_pnn0_dn3_slot: &mut f64,
        var_pnn0_dn4_slot: &mut f64,
        var_pnn0_dn5_slot: &mut f64,
        var_pnn0_rdb0_slot: &mut f64,
        var_pnn0_rdb1_slot: &mut f64,
        var_pnn0_rdb2_slot: &mut f64,
        var_pnn0_rdb3_slot: &mut f64,
        var_pnn0_rdn0_slot: &mut f64,
        var_pnn0_rdn1_slot: &mut f64,
        var_pnn0_rdn2_slot: &mut f64,
        var_pnn0_rdn3_slot: &mut f64,
        var_pnn0_rdn4_slot: &mut f64,
        var_pnn0_rdn5_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_rdb0_slot: &mut f64,
        var_tmf1_rdb1_slot: &mut f64,
        var_tmf1_rdb2_slot: &mut f64,
        var_tmf1_rdb3_slot: &mut f64,
        var_tmf1_rdn0_slot: &mut f64,
        var_tmf1_rdn1_slot: &mut f64,
        var_tmf1_rdn2_slot: &mut f64,
        var_tmf1_rdn3_slot: &mut f64,
        var_tmf1_rdn4_slot: &mut f64,
        var_tmf1_rdn5_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_rdb0_slot: &mut f64,
        var_tmf2_rdb1_slot: &mut f64,
        var_tmf2_rdb2_slot: &mut f64,
        var_tmf2_rdb3_slot: &mut f64,
        var_tmf2_rdn0_slot: &mut f64,
        var_tmf2_rdn1_slot: &mut f64,
        var_tmf2_rdn2_slot: &mut f64,
        var_tmf2_rdn3_slot: &mut f64,
        var_tmf2_rdn4_slot: &mut f64,
        var_tmf2_rdn5_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_db0_slot: &mut f64,
        var_vha1_db1_slot: &mut f64,
        var_vha1_db2_slot: &mut f64,
        var_vha1_db3_slot: &mut f64,
        var_vha1_dn0_slot: &mut f64,
        var_vha1_dn1_slot: &mut f64,
        var_vha1_dn2_slot: &mut f64,
        var_vha1_dn3_slot: &mut f64,
        var_vha1_dn4_slot: &mut f64,
        var_vha1_dn5_slot: &mut f64,
        var_vha1_rdb0_slot: &mut f64,
        var_vha1_rdb1_slot: &mut f64,
        var_vha1_rdb2_slot: &mut f64,
        var_vha1_rdb3_slot: &mut f64,
        var_vha1_rdn0_slot: &mut f64,
        var_vha1_rdn1_slot: &mut f64,
        var_vha1_rdn2_slot: &mut f64,
        var_vha1_rdn3_slot: &mut f64,
        var_vha1_rdn4_slot: &mut f64,
        var_vha1_rdn5_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_guard492: f64 = *var_guard492_slot;
        let mut var_guard492_db0: f64 = *var_guard492_db0_slot;
        let mut var_guard492_db1: f64 = *var_guard492_db1_slot;
        let mut var_guard492_db2: f64 = *var_guard492_db2_slot;
        let mut var_guard492_db3: f64 = *var_guard492_db3_slot;
        let mut var_guard492_dn0: f64 = *var_guard492_dn0_slot;
        let mut var_guard492_dn1: f64 = *var_guard492_dn1_slot;
        let mut var_guard492_dn2: f64 = *var_guard492_dn2_slot;
        let mut var_guard492_dn3: f64 = *var_guard492_dn3_slot;
        let mut var_guard492_dn4: f64 = *var_guard492_dn4_slot;
        let mut var_guard492_dn5: f64 = *var_guard492_dn5_slot;
        let mut var_guard492_rdb0: f64 = *var_guard492_rdb0_slot;
        let mut var_guard492_rdb1: f64 = *var_guard492_rdb1_slot;
        let mut var_guard492_rdb2: f64 = *var_guard492_rdb2_slot;
        let mut var_guard492_rdb3: f64 = *var_guard492_rdb3_slot;
        let mut var_guard492_rdn0: f64 = *var_guard492_rdn0_slot;
        let mut var_guard492_rdn1: f64 = *var_guard492_rdn1_slot;
        let mut var_guard492_rdn2: f64 = *var_guard492_rdn2_slot;
        let mut var_guard492_rdn3: f64 = *var_guard492_rdn3_slot;
        let mut var_guard492_rdn4: f64 = *var_guard492_rdn4_slot;
        let mut var_guard492_rdn5: f64 = *var_guard492_rdn5_slot;
        let mut var_guard492_rv: f64 = *var_guard492_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj0_rdb0: f64 = *var_nj0_rdb0_slot;
        let mut var_nj0_rdb1: f64 = *var_nj0_rdb1_slot;
        let mut var_nj0_rdb2: f64 = *var_nj0_rdb2_slot;
        let mut var_nj0_rdb3: f64 = *var_nj0_rdb3_slot;
        let mut var_nj0_rdn0: f64 = *var_nj0_rdn0_slot;
        let mut var_nj0_rdn1: f64 = *var_nj0_rdn1_slot;
        let mut var_nj0_rdn2: f64 = *var_nj0_rdn2_slot;
        let mut var_nj0_rdn3: f64 = *var_nj0_rdn3_slot;
        let mut var_nj0_rdn4: f64 = *var_nj0_rdn4_slot;
        let mut var_nj0_rdn5: f64 = *var_nj0_rdn5_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nj1_rdb0: f64 = *var_nj1_rdb0_slot;
        let mut var_nj1_rdb1: f64 = *var_nj1_rdb1_slot;
        let mut var_nj1_rdb2: f64 = *var_nj1_rdb2_slot;
        let mut var_nj1_rdb3: f64 = *var_nj1_rdb3_slot;
        let mut var_nj1_rdn0: f64 = *var_nj1_rdn0_slot;
        let mut var_nj1_rdn1: f64 = *var_nj1_rdn1_slot;
        let mut var_nj1_rdn2: f64 = *var_nj1_rdn2_slot;
        let mut var_nj1_rdn3: f64 = *var_nj1_rdn3_slot;
        let mut var_nj1_rdn4: f64 = *var_nj1_rdn4_slot;
        let mut var_nj1_rdn5: f64 = *var_nj1_rdn5_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_nja10_rdb0: f64 = *var_nja10_rdb0_slot;
        let mut var_nja10_rdb1: f64 = *var_nja10_rdb1_slot;
        let mut var_nja10_rdb2: f64 = *var_nja10_rdb2_slot;
        let mut var_nja10_rdb3: f64 = *var_nja10_rdb3_slot;
        let mut var_nja10_rdn0: f64 = *var_nja10_rdn0_slot;
        let mut var_nja10_rdn1: f64 = *var_nja10_rdn1_slot;
        let mut var_nja10_rdn2: f64 = *var_nja10_rdn2_slot;
        let mut var_nja10_rdn3: f64 = *var_nja10_rdn3_slot;
        let mut var_nja10_rdn4: f64 = *var_nja10_rdn4_slot;
        let mut var_nja10_rdn5: f64 = *var_nja10_rdn5_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_db0: f64 = *var_pnn0_db0_slot;
        let mut var_pnn0_db1: f64 = *var_pnn0_db1_slot;
        let mut var_pnn0_db2: f64 = *var_pnn0_db2_slot;
        let mut var_pnn0_db3: f64 = *var_pnn0_db3_slot;
        let mut var_pnn0_dn0: f64 = *var_pnn0_dn0_slot;
        let mut var_pnn0_dn1: f64 = *var_pnn0_dn1_slot;
        let mut var_pnn0_dn2: f64 = *var_pnn0_dn2_slot;
        let mut var_pnn0_dn3: f64 = *var_pnn0_dn3_slot;
        let mut var_pnn0_dn4: f64 = *var_pnn0_dn4_slot;
        let mut var_pnn0_dn5: f64 = *var_pnn0_dn5_slot;
        let mut var_pnn0_rdb0: f64 = *var_pnn0_rdb0_slot;
        let mut var_pnn0_rdb1: f64 = *var_pnn0_rdb1_slot;
        let mut var_pnn0_rdb2: f64 = *var_pnn0_rdb2_slot;
        let mut var_pnn0_rdb3: f64 = *var_pnn0_rdb3_slot;
        let mut var_pnn0_rdn0: f64 = *var_pnn0_rdn0_slot;
        let mut var_pnn0_rdn1: f64 = *var_pnn0_rdn1_slot;
        let mut var_pnn0_rdn2: f64 = *var_pnn0_rdn2_slot;
        let mut var_pnn0_rdn3: f64 = *var_pnn0_rdn3_slot;
        let mut var_pnn0_rdn4: f64 = *var_pnn0_rdn4_slot;
        let mut var_pnn0_rdn5: f64 = *var_pnn0_rdn5_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_rdb0: f64 = *var_tmf1_rdb0_slot;
        let mut var_tmf1_rdb1: f64 = *var_tmf1_rdb1_slot;
        let mut var_tmf1_rdb2: f64 = *var_tmf1_rdb2_slot;
        let mut var_tmf1_rdb3: f64 = *var_tmf1_rdb3_slot;
        let mut var_tmf1_rdn0: f64 = *var_tmf1_rdn0_slot;
        let mut var_tmf1_rdn1: f64 = *var_tmf1_rdn1_slot;
        let mut var_tmf1_rdn2: f64 = *var_tmf1_rdn2_slot;
        let mut var_tmf1_rdn3: f64 = *var_tmf1_rdn3_slot;
        let mut var_tmf1_rdn4: f64 = *var_tmf1_rdn4_slot;
        let mut var_tmf1_rdn5: f64 = *var_tmf1_rdn5_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_rdb0: f64 = *var_tmf2_rdb0_slot;
        let mut var_tmf2_rdb1: f64 = *var_tmf2_rdb1_slot;
        let mut var_tmf2_rdb2: f64 = *var_tmf2_rdb2_slot;
        let mut var_tmf2_rdb3: f64 = *var_tmf2_rdb3_slot;
        let mut var_tmf2_rdn0: f64 = *var_tmf2_rdn0_slot;
        let mut var_tmf2_rdn1: f64 = *var_tmf2_rdn1_slot;
        let mut var_tmf2_rdn2: f64 = *var_tmf2_rdn2_slot;
        let mut var_tmf2_rdn3: f64 = *var_tmf2_rdn3_slot;
        let mut var_tmf2_rdn4: f64 = *var_tmf2_rdn4_slot;
        let mut var_tmf2_rdn5: f64 = *var_tmf2_rdn5_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_db0: f64 = *var_vha1_db0_slot;
        let mut var_vha1_db1: f64 = *var_vha1_db1_slot;
        let mut var_vha1_db2: f64 = *var_vha1_db2_slot;
        let mut var_vha1_db3: f64 = *var_vha1_db3_slot;
        let mut var_vha1_dn0: f64 = *var_vha1_dn0_slot;
        let mut var_vha1_dn1: f64 = *var_vha1_dn1_slot;
        let mut var_vha1_dn2: f64 = *var_vha1_dn2_slot;
        let mut var_vha1_dn3: f64 = *var_vha1_dn3_slot;
        let mut var_vha1_dn4: f64 = *var_vha1_dn4_slot;
        let mut var_vha1_dn5: f64 = *var_vha1_dn5_slot;
        let mut var_vha1_rdb0: f64 = *var_vha1_rdb0_slot;
        let mut var_vha1_rdb1: f64 = *var_vha1_rdb1_slot;
        let mut var_vha1_rdb2: f64 = *var_vha1_rdb2_slot;
        let mut var_vha1_rdb3: f64 = *var_vha1_rdb3_slot;
        let mut var_vha1_rdn0: f64 = *var_vha1_rdn0_slot;
        let mut var_vha1_rdn1: f64 = *var_vha1_rdn1_slot;
        let mut var_vha1_rdn2: f64 = *var_vha1_rdn2_slot;
        let mut var_vha1_rdn3: f64 = *var_vha1_rdn3_slot;
        let mut var_vha1_rdn4: f64 = *var_vha1_rdn4_slot;
        let mut var_vha1_rdn5: f64 = *var_vha1_rdn5_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign30160_e44784, assign30160_e44784_d_n0, assign30160_e44784_d_n1, assign30160_e44784_d_n2, assign30160_e44784_d_n3, assign30160_e44784_d_n4, assign30160_e44784_d_n5, assign30160_e44784_d_b0, assign30160_e44784_d_b1, assign30160_e44784_d_b2, assign30160_e44784_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30160_e44780: f64 = (4.0 * p.p85);
        let assign30160_e44782: f64 = (assign30160_e44780 * 0.01);
        (assign30160_e44782, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30160_e44784;
        var_tmf2_dn0 = assign30160_e44784_d_n0;
        var_tmf2_dn1 = assign30160_e44784_d_n1;
        var_tmf2_dn2 = assign30160_e44784_d_n2;
        var_tmf2_dn3 = assign30160_e44784_d_n3;
        var_tmf2_dn4 = assign30160_e44784_d_n4;
        var_tmf2_dn5 = assign30160_e44784_d_n5;
        var_tmf2_db0 = assign30160_e44784_d_b0;
        var_tmf2_db1 = assign30160_e44784_d_b1;
        var_tmf2_db2 = assign30160_e44784_d_b2;
        var_tmf2_db3 = assign30160_e44784_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30170_e44801, assign30170_e44801_d_n0, assign30170_e44801_d_n1, assign30170_e44801_d_n2, assign30170_e44801_d_n3, assign30170_e44801_d_n4, assign30170_e44801_d_n5, assign30170_e44801_d_b0, assign30170_e44801_d_b1, assign30170_e44801_d_b2, assign30170_e44801_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let (assign30170_e44799, assign30170_e44799_d_n0, assign30170_e44799_d_n1, assign30170_e44799_d_n2, assign30170_e44799_d_n3, assign30170_e44799_d_n4, assign30170_e44799_d_n5, assign30170_e44799_d_b0, assign30170_e44799_d_b1, assign30170_e44799_d_b2, assign30170_e44799_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30170_e44798: f64 = (-var_tmf2);
                (assign30170_e44798, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30170_e44799, assign30170_e44799_d_n0, assign30170_e44799_d_n1, assign30170_e44799_d_n2, assign30170_e44799_d_n3, assign30170_e44799_d_n4, assign30170_e44799_d_n5, assign30170_e44799_d_b0, assign30170_e44799_d_b1, assign30170_e44799_d_b2, assign30170_e44799_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30170_e44801;
        var_tmf2_dn0 = assign30170_e44801_d_n0;
        var_tmf2_dn1 = assign30170_e44801_d_n1;
        var_tmf2_dn2 = assign30170_e44801_d_n2;
        var_tmf2_dn3 = assign30170_e44801_d_n3;
        var_tmf2_dn4 = assign30170_e44801_d_n4;
        var_tmf2_dn5 = assign30170_e44801_d_n5;
        var_tmf2_db0 = assign30170_e44801_d_b0;
        var_tmf2_db1 = assign30170_e44801_d_b1;
        var_tmf2_db2 = assign30170_e44801_d_b2;
        var_tmf2_db3 = assign30170_e44801_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30180_e44817, assign30180_e44817_d_n0, assign30180_e44817_d_n1, assign30180_e44817_d_n2, assign30180_e44817_d_n3, assign30180_e44817_d_n4, assign30180_e44817_d_n5, assign30180_e44817_d_b0, assign30180_e44817_d_b1, assign30180_e44817_d_b2, assign30180_e44817_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30180_e44812: f64 = (var_tmf1 * var_tmf1);
        let assign30180_e44814: f64 = (assign30180_e44812 + var_tmf2);
        let assign30180_e44815: f64 = (assign30180_e44814).sqrt();
        (assign30180_e44815, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30180_e44815)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30180_e44815)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30180_e44815)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30180_e44815)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30180_e44815)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30180_e44815)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30180_e44815)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30180_e44815)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30180_e44815)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30180_e44815)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30180_e44817;
        var_tmf2_dn0 = assign30180_e44817_d_n0;
        var_tmf2_dn1 = assign30180_e44817_d_n1;
        var_tmf2_dn2 = assign30180_e44817_d_n2;
        var_tmf2_dn3 = assign30180_e44817_d_n3;
        var_tmf2_dn4 = assign30180_e44817_d_n4;
        var_tmf2_dn5 = assign30180_e44817_d_n5;
        var_tmf2_db0 = assign30180_e44817_d_b0;
        var_tmf2_db1 = assign30180_e44817_d_b1;
        var_tmf2_db2 = assign30180_e44817_d_b2;
        var_tmf2_db3 = assign30180_e44817_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30190_e44834, assign30190_e44834_d_n0, assign30190_e44834_d_n1, assign30190_e44834_d_n2, assign30190_e44834_d_n3, assign30190_e44834_d_n4, assign30190_e44834_d_n5, assign30190_e44834_d_b0, assign30190_e44834_d_b1, assign30190_e44834_d_b2, assign30190_e44834_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30190_e44830: f64 = (var_tmf1 + var_tmf2);
        let assign30190_e44831: f64 = (0.5 * assign30190_e44830);
        let assign30190_e44832: f64 = (p.p85 - assign30190_e44831);
        (assign30190_e44832, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30190_e44834;
        var_nj0_dn0 = assign30190_e44834_d_n0;
        var_nj0_dn1 = assign30190_e44834_d_n1;
        var_nj0_dn2 = assign30190_e44834_d_n2;
        var_nj0_dn3 = assign30190_e44834_d_n3;
        var_nj0_dn4 = assign30190_e44834_d_n4;
        var_nj0_dn5 = assign30190_e44834_d_n5;
        var_nj0_db0 = assign30190_e44834_d_b0;
        var_nj0_db1 = assign30190_e44834_d_b1;
        var_nj0_db2 = assign30190_e44834_d_b2;
        var_nj0_db3 = assign30190_e44834_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign30200_e44849, assign30200_e44849_d_n0, assign30200_e44849_d_n1, assign30200_e44849_d_n2, assign30200_e44849_d_n3, assign30200_e44849_d_n4, assign30200_e44849_d_n5, assign30200_e44849_d_b0, assign30200_e44849_d_b1, assign30200_e44849_d_b2, assign30200_e44849_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30200_e44845: f64 = (var_nj0 - var_nfagat_i);
        let assign30200_e44847: f64 = (assign30200_e44845 - 0.01);
        (assign30200_e44847, (var_nj0_dn0 - var_nfagat_i_dn0), (var_nj0_dn1 - var_nfagat_i_dn1), (var_nj0_dn2 - var_nfagat_i_dn2), (var_nj0_dn3 - var_nfagat_i_dn3), (var_nj0_dn4 - var_nfagat_i_dn4), (var_nj0_dn5 - var_nfagat_i_dn5), (var_nj0_db0 - var_nfagat_i_db0), (var_nj0_db1 - var_nfagat_i_db1), (var_nj0_db2 - var_nfagat_i_db2), (var_nj0_db3 - var_nfagat_i_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30200_e44849;
        var_tmf1_dn0 = assign30200_e44849_d_n0;
        var_tmf1_dn1 = assign30200_e44849_d_n1;
        var_tmf1_dn2 = assign30200_e44849_d_n2;
        var_tmf1_dn3 = assign30200_e44849_d_n3;
        var_tmf1_dn4 = assign30200_e44849_d_n4;
        var_tmf1_dn5 = assign30200_e44849_d_n5;
        var_tmf1_db0 = assign30200_e44849_d_b0;
        var_tmf1_db1 = assign30200_e44849_d_b1;
        var_tmf1_db2 = assign30200_e44849_d_b2;
        var_tmf1_db3 = assign30200_e44849_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign30210_e44864, assign30210_e44864_d_n0, assign30210_e44864_d_n1, assign30210_e44864_d_n2, assign30210_e44864_d_n3, assign30210_e44864_d_n4, assign30210_e44864_d_n5, assign30210_e44864_d_b0, assign30210_e44864_d_b1, assign30210_e44864_d_b2, assign30210_e44864_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30210_e44860: f64 = (4.0 * var_nfagat_i);
        let assign30210_e44862: f64 = (assign30210_e44860 * 0.01);
        (assign30210_e44862, ((4.0 * var_nfagat_i_dn0) * 0.01), ((4.0 * var_nfagat_i_dn1) * 0.01), ((4.0 * var_nfagat_i_dn2) * 0.01), ((4.0 * var_nfagat_i_dn3) * 0.01), ((4.0 * var_nfagat_i_dn4) * 0.01), ((4.0 * var_nfagat_i_dn5) * 0.01), ((4.0 * var_nfagat_i_db0) * 0.01), ((4.0 * var_nfagat_i_db1) * 0.01), ((4.0 * var_nfagat_i_db2) * 0.01), ((4.0 * var_nfagat_i_db3) * 0.01),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30210_e44864;
        var_tmf2_dn0 = assign30210_e44864_d_n0;
        var_tmf2_dn1 = assign30210_e44864_d_n1;
        var_tmf2_dn2 = assign30210_e44864_d_n2;
        var_tmf2_dn3 = assign30210_e44864_d_n3;
        var_tmf2_dn4 = assign30210_e44864_d_n4;
        var_tmf2_dn5 = assign30210_e44864_d_n5;
        var_tmf2_db0 = assign30210_e44864_d_b0;
        var_tmf2_db1 = assign30210_e44864_d_b1;
        var_tmf2_db2 = assign30210_e44864_d_b2;
        var_tmf2_db3 = assign30210_e44864_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30220_e44881, assign30220_e44881_d_n0, assign30220_e44881_d_n1, assign30220_e44881_d_n2, assign30220_e44881_d_n3, assign30220_e44881_d_n4, assign30220_e44881_d_n5, assign30220_e44881_d_b0, assign30220_e44881_d_b1, assign30220_e44881_d_b2, assign30220_e44881_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let (assign30220_e44879, assign30220_e44879_d_n0, assign30220_e44879_d_n1, assign30220_e44879_d_n2, assign30220_e44879_d_n3, assign30220_e44879_d_n4, assign30220_e44879_d_n5, assign30220_e44879_d_b0, assign30220_e44879_d_b1, assign30220_e44879_d_b2, assign30220_e44879_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30220_e44878: f64 = (-var_tmf2);
                (assign30220_e44878, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30220_e44879, assign30220_e44879_d_n0, assign30220_e44879_d_n1, assign30220_e44879_d_n2, assign30220_e44879_d_n3, assign30220_e44879_d_n4, assign30220_e44879_d_n5, assign30220_e44879_d_b0, assign30220_e44879_d_b1, assign30220_e44879_d_b2, assign30220_e44879_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30220_e44881;
        var_tmf2_dn0 = assign30220_e44881_d_n0;
        var_tmf2_dn1 = assign30220_e44881_d_n1;
        var_tmf2_dn2 = assign30220_e44881_d_n2;
        var_tmf2_dn3 = assign30220_e44881_d_n3;
        var_tmf2_dn4 = assign30220_e44881_d_n4;
        var_tmf2_dn5 = assign30220_e44881_d_n5;
        var_tmf2_db0 = assign30220_e44881_d_b0;
        var_tmf2_db1 = assign30220_e44881_d_b1;
        var_tmf2_db2 = assign30220_e44881_d_b2;
        var_tmf2_db3 = assign30220_e44881_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30230_e44897, assign30230_e44897_d_n0, assign30230_e44897_d_n1, assign30230_e44897_d_n2, assign30230_e44897_d_n3, assign30230_e44897_d_n4, assign30230_e44897_d_n5, assign30230_e44897_d_b0, assign30230_e44897_d_b1, assign30230_e44897_d_b2, assign30230_e44897_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30230_e44892: f64 = (var_tmf1 * var_tmf1);
        let assign30230_e44894: f64 = (assign30230_e44892 + var_tmf2);
        let assign30230_e44895: f64 = (assign30230_e44894).sqrt();
        (assign30230_e44895, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30230_e44895)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30230_e44895)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30230_e44895)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30230_e44895)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30230_e44895)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30230_e44895)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30230_e44895)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30230_e44895)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30230_e44895)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30230_e44895)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30230_e44897;
        var_tmf2_dn0 = assign30230_e44897_d_n0;
        var_tmf2_dn1 = assign30230_e44897_d_n1;
        var_tmf2_dn2 = assign30230_e44897_d_n2;
        var_tmf2_dn3 = assign30230_e44897_d_n3;
        var_tmf2_dn4 = assign30230_e44897_d_n4;
        var_tmf2_dn5 = assign30230_e44897_d_n5;
        var_tmf2_db0 = assign30230_e44897_d_b0;
        var_tmf2_db1 = assign30230_e44897_d_b1;
        var_tmf2_db2 = assign30230_e44897_d_b2;
        var_tmf2_db3 = assign30230_e44897_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30240_e44914, assign30240_e44914_d_n0, assign30240_e44914_d_n1, assign30240_e44914_d_n2, assign30240_e44914_d_n3, assign30240_e44914_d_n4, assign30240_e44914_d_n5, assign30240_e44914_d_b0, assign30240_e44914_d_b1, assign30240_e44914_d_b2, assign30240_e44914_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30240_e44910: f64 = (var_tmf1 + var_tmf2);
        let assign30240_e44911: f64 = (0.5 * assign30240_e44910);
        let assign30240_e44912: f64 = (var_nfagat_i + assign30240_e44911);
        (assign30240_e44912, (var_nfagat_i_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_nfagat_i_dn1 + (0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (var_nfagat_i_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_nfagat_i_dn3 + (0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (var_nfagat_i_dn4 + (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_nfagat_i_dn5 + (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_nfagat_i_db0 + (0.5 * (var_tmf1_db0 + var_tmf2_db0))), (var_nfagat_i_db1 + (0.5 * (var_tmf1_db1 + var_tmf2_db1))), (var_nfagat_i_db2 + (0.5 * (var_tmf1_db2 + var_tmf2_db2))), (var_nfagat_i_db3 + (0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30240_e44914;
        var_nj0_dn0 = assign30240_e44914_d_n0;
        var_nj0_dn1 = assign30240_e44914_d_n1;
        var_nj0_dn2 = assign30240_e44914_d_n2;
        var_nj0_dn3 = assign30240_e44914_d_n3;
        var_nj0_dn4 = assign30240_e44914_d_n4;
        var_nj0_dn5 = assign30240_e44914_d_n5;
        var_nj0_db0 = assign30240_e44914_d_b0;
        var_nj0_db1 = assign30240_e44914_d_b1;
        var_nj0_db2 = assign30240_e44914_d_b2;
        var_nj0_db3 = assign30240_e44914_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign30250_e44926, assign30250_e44926_d_n0, assign30250_e44926_d_n1, assign30250_e44926_d_n2, assign30250_e44926_d_n3, assign30250_e44926_d_n4, assign30250_e44926_d_n5, assign30250_e44926_d_b0, assign30250_e44926_d_b1, assign30250_e44926_d_b2, assign30250_e44926_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 == 0.0)) {
        (var_nfagat_i, var_nfagat_i_dn0, var_nfagat_i_dn1, var_nfagat_i_dn2, var_nfagat_i_dn3, var_nfagat_i_dn4, var_nfagat_i_dn5, var_nfagat_i_db0, var_nfagat_i_db1, var_nfagat_i_db2, var_nfagat_i_db3,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30250_e44926;
        var_nj0_dn0 = assign30250_e44926_d_n0;
        var_nj0_dn1 = assign30250_e44926_d_n1;
        var_nj0_dn2 = assign30250_e44926_d_n2;
        var_nj0_dn3 = assign30250_e44926_d_n3;
        var_nj0_dn4 = assign30250_e44926_d_n4;
        var_nj0_dn5 = assign30250_e44926_d_n5;
        var_nj0_db0 = assign30250_e44926_d_b0;
        var_nj0_db1 = assign30250_e44926_d_b1;
        var_nj0_db2 = assign30250_e44926_d_b2;
        var_nj0_db3 = assign30250_e44926_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign30260_e44938, assign30260_e44938_d_n0, assign30260_e44938_d_n1, assign30260_e44938_d_n2, assign30260_e44938_d_n3, assign30260_e44938_d_n4, assign30260_e44938_d_n5, assign30260_e44938_d_b0, assign30260_e44938_d_b1, assign30260_e44938_d_b2, assign30260_e44938_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 == 0.0)) {
        (var_nfagat_i, var_nfagat_i_dn0, var_nfagat_i_dn1, var_nfagat_i_dn2, var_nfagat_i_dn3, var_nfagat_i_dn4, var_nfagat_i_dn5, var_nfagat_i_db0, var_nfagat_i_db1, var_nfagat_i_db2, var_nfagat_i_db3,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign30260_e44938;
        var_nj1_dn0 = assign30260_e44938_d_n0;
        var_nj1_dn1 = assign30260_e44938_d_n1;
        var_nj1_dn2 = assign30260_e44938_d_n2;
        var_nj1_dn3 = assign30260_e44938_d_n3;
        var_nj1_dn4 = assign30260_e44938_d_n4;
        var_nj1_dn5 = assign30260_e44938_d_n5;
        var_nj1_db0 = assign30260_e44938_d_b0;
        var_nj1_db1 = assign30260_e44938_d_b1;
        var_nj1_db2 = assign30260_e44938_d_b2;
        var_nj1_db3 = assign30260_e44938_d_b3;
        var_nj1_rv = 0.0;
        var_nj1_rdn0 = 0.0;
        var_nj1_rdn1 = 0.0;
        var_nj1_rdn2 = 0.0;
        var_nj1_rdn3 = 0.0;
        var_nj1_rdn4 = 0.0;
        var_nj1_rdn5 = 0.0;
        var_nj1_rdb0 = 0.0;
        var_nj1_rdb1 = 0.0;
        var_nj1_rdb2 = 0.0;
        var_nj1_rdb3 = 0.0;

        let (assign30330_e45193, assign30330_e45193_d_n0, assign30330_e45193_d_n1, assign30330_e45193_d_n2, assign30330_e45193_d_n3, assign30330_e45193_d_n4, assign30330_e45193_d_n5, assign30330_e45193_d_b0, assign30330_e45193_d_b1, assign30330_e45193_d_b2, assign30330_e45193_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30330_e45189: f64 = (var_nin * var_nin);
        let assign30330_e45191: f64 = (assign30330_e45189 / var_ndibot_i);
        (assign30330_e45191, (((((var_nin_dn0 * var_nin) + (var_nin * var_nin_dn0)) * var_ndibot_i) - (assign30330_e45189 * var_ndibot_i_dn0)) / (var_ndibot_i * var_ndibot_i)), (((((var_nin_dn1 * var_nin) + (var_nin * var_nin_dn1)) * var_ndibot_i) - (assign30330_e45189 * var_ndibot_i_dn1)) / (var_ndibot_i * var_ndibot_i)), (((((var_nin_dn2 * var_nin) + (var_nin * var_nin_dn2)) * var_ndibot_i) - (assign30330_e45189 * var_ndibot_i_dn2)) / (var_ndibot_i * var_ndibot_i)), (((((var_nin_dn3 * var_nin) + (var_nin * var_nin_dn3)) * var_ndibot_i) - (assign30330_e45189 * var_ndibot_i_dn3)) / (var_ndibot_i * var_ndibot_i)), (((((var_nin_dn4 * var_nin) + (var_nin * var_nin_dn4)) * var_ndibot_i) - (assign30330_e45189 * var_ndibot_i_dn4)) / (var_ndibot_i * var_ndibot_i)), (((((var_nin_dn5 * var_nin) + (var_nin * var_nin_dn5)) * var_ndibot_i) - (assign30330_e45189 * var_ndibot_i_dn5)) / (var_ndibot_i * var_ndibot_i)), (((((var_nin_db0 * var_nin) + (var_nin * var_nin_db0)) * var_ndibot_i) - (assign30330_e45189 * var_ndibot_i_db0)) / (var_ndibot_i * var_ndibot_i)), (((((var_nin_db1 * var_nin) + (var_nin * var_nin_db1)) * var_ndibot_i) - (assign30330_e45189 * var_ndibot_i_db1)) / (var_ndibot_i * var_ndibot_i)), (((((var_nin_db2 * var_nin) + (var_nin * var_nin_db2)) * var_ndibot_i) - (assign30330_e45189 * var_ndibot_i_db2)) / (var_ndibot_i * var_ndibot_i)), (((((var_nin_db3 * var_nin) + (var_nin * var_nin_db3)) * var_ndibot_i) - (assign30330_e45189 * var_ndibot_i_db3)) / (var_ndibot_i * var_ndibot_i)),)
    } else {
        (var_pnn0, var_pnn0_dn0, var_pnn0_dn1, var_pnn0_dn2, var_pnn0_dn3, var_pnn0_dn4, var_pnn0_dn5, var_pnn0_db0, var_pnn0_db1, var_pnn0_db2, var_pnn0_db3,)
    }
};
        var_pnn0 = assign30330_e45193;
        var_pnn0_dn0 = assign30330_e45193_d_n0;
        var_pnn0_dn1 = assign30330_e45193_d_n1;
        var_pnn0_dn2 = assign30330_e45193_d_n2;
        var_pnn0_dn3 = assign30330_e45193_d_n3;
        var_pnn0_dn4 = assign30330_e45193_d_n4;
        var_pnn0_dn5 = assign30330_e45193_d_n5;
        var_pnn0_db0 = assign30330_e45193_d_b0;
        var_pnn0_db1 = assign30330_e45193_d_b1;
        var_pnn0_db2 = assign30330_e45193_d_b2;
        var_pnn0_db3 = assign30330_e45193_d_b3;
        var_pnn0_rv = 0.0;
        var_pnn0_rdn0 = 0.0;
        var_pnn0_rdn1 = 0.0;
        var_pnn0_rdn2 = 0.0;
        var_pnn0_rdn3 = 0.0;
        var_pnn0_rdn4 = 0.0;
        var_pnn0_rdn5 = 0.0;
        var_pnn0_rdb0 = 0.0;
        var_pnn0_rdb1 = 0.0;
        var_pnn0_rdb2 = 0.0;
        var_pnn0_rdb3 = 0.0;

        let (assign30340_e45210, assign30340_e45210_d_n0, assign30340_e45210_d_n1, assign30340_e45210_d_n2, assign30340_e45210_d_n3, assign30340_e45210_d_n4, assign30340_e45210_d_n5, assign30340_e45210_d_b0, assign30340_e45210_d_b1, assign30340_e45210_d_b2, assign30340_e45210_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30340_e45203: f64 = (var_nfabot_i / var_phitdinv);
        let assign30340_e45206: f64 = (var_ndibot_i / var_pnn0);
        let assign30340_e45207: f64 = (assign30340_e45206).ln();
        let assign30340_e45208: f64 = (assign30340_e45203 * assign30340_e45207);
        (assign30340_e45208, (((((var_nfabot_i_dn0 * var_phitdinv) - (var_nfabot_i * var_phitdinv_dn0)) / (var_phitdinv * var_phitdinv)) * assign30340_e45207) + (assign30340_e45203 * ((((var_ndibot_i_dn0 * var_pnn0) - (var_ndibot_i * var_pnn0_dn0)) / (var_pnn0 * var_pnn0)) / assign30340_e45206))), (((((var_nfabot_i_dn1 * var_phitdinv) - (var_nfabot_i * var_phitdinv_dn1)) / (var_phitdinv * var_phitdinv)) * assign30340_e45207) + (assign30340_e45203 * ((((var_ndibot_i_dn1 * var_pnn0) - (var_ndibot_i * var_pnn0_dn1)) / (var_pnn0 * var_pnn0)) / assign30340_e45206))), (((((var_nfabot_i_dn2 * var_phitdinv) - (var_nfabot_i * var_phitdinv_dn2)) / (var_phitdinv * var_phitdinv)) * assign30340_e45207) + (assign30340_e45203 * ((((var_ndibot_i_dn2 * var_pnn0) - (var_ndibot_i * var_pnn0_dn2)) / (var_pnn0 * var_pnn0)) / assign30340_e45206))), (((((var_nfabot_i_dn3 * var_phitdinv) - (var_nfabot_i * var_phitdinv_dn3)) / (var_phitdinv * var_phitdinv)) * assign30340_e45207) + (assign30340_e45203 * ((((var_ndibot_i_dn3 * var_pnn0) - (var_ndibot_i * var_pnn0_dn3)) / (var_pnn0 * var_pnn0)) / assign30340_e45206))), (((((var_nfabot_i_dn4 * var_phitdinv) - (var_nfabot_i * var_phitdinv_dn4)) / (var_phitdinv * var_phitdinv)) * assign30340_e45207) + (assign30340_e45203 * ((((var_ndibot_i_dn4 * var_pnn0) - (var_ndibot_i * var_pnn0_dn4)) / (var_pnn0 * var_pnn0)) / assign30340_e45206))), (((((var_nfabot_i_dn5 * var_phitdinv) - (var_nfabot_i * var_phitdinv_dn5)) / (var_phitdinv * var_phitdinv)) * assign30340_e45207) + (assign30340_e45203 * ((((var_ndibot_i_dn5 * var_pnn0) - (var_ndibot_i * var_pnn0_dn5)) / (var_pnn0 * var_pnn0)) / assign30340_e45206))), (((((var_nfabot_i_db0 * var_phitdinv) - (var_nfabot_i * var_phitdinv_db0)) / (var_phitdinv * var_phitdinv)) * assign30340_e45207) + (assign30340_e45203 * ((((var_ndibot_i_db0 * var_pnn0) - (var_ndibot_i * var_pnn0_db0)) / (var_pnn0 * var_pnn0)) / assign30340_e45206))), (((((var_nfabot_i_db1 * var_phitdinv) - (var_nfabot_i * var_phitdinv_db1)) / (var_phitdinv * var_phitdinv)) * assign30340_e45207) + (assign30340_e45203 * ((((var_ndibot_i_db1 * var_pnn0) - (var_ndibot_i * var_pnn0_db1)) / (var_pnn0 * var_pnn0)) / assign30340_e45206))), (((((var_nfabot_i_db2 * var_phitdinv) - (var_nfabot_i * var_phitdinv_db2)) / (var_phitdinv * var_phitdinv)) * assign30340_e45207) + (assign30340_e45203 * ((((var_ndibot_i_db2 * var_pnn0) - (var_ndibot_i * var_pnn0_db2)) / (var_pnn0 * var_pnn0)) / assign30340_e45206))), (((((var_nfabot_i_db3 * var_phitdinv) - (var_nfabot_i * var_phitdinv_db3)) / (var_phitdinv * var_phitdinv)) * assign30340_e45207) + (assign30340_e45203 * ((((var_ndibot_i_db3 * var_pnn0) - (var_ndibot_i * var_pnn0_db3)) / (var_pnn0 * var_pnn0)) / assign30340_e45206))),)
    } else {
        (var_vha1, var_vha1_dn0, var_vha1_dn1, var_vha1_dn2, var_vha1_dn3, var_vha1_dn4, var_vha1_dn5, var_vha1_db0, var_vha1_db1, var_vha1_db2, var_vha1_db3,)
    }
};
        var_vha1 = assign30340_e45210;
        var_vha1_dn0 = assign30340_e45210_d_n0;
        var_vha1_dn1 = assign30340_e45210_d_n1;
        var_vha1_dn2 = assign30340_e45210_d_n2;
        var_vha1_dn3 = assign30340_e45210_d_n3;
        var_vha1_dn4 = assign30340_e45210_d_n4;
        var_vha1_dn5 = assign30340_e45210_d_n5;
        var_vha1_db0 = assign30340_e45210_d_b0;
        var_vha1_db1 = assign30340_e45210_d_b1;
        var_vha1_db2 = assign30340_e45210_d_b2;
        var_vha1_db3 = assign30340_e45210_d_b3;
        var_vha1_rv = 0.0;
        var_vha1_rdn0 = 0.0;
        var_vha1_rdn1 = 0.0;
        var_vha1_rdn2 = 0.0;
        var_vha1_rdn3 = 0.0;
        var_vha1_rdn4 = 0.0;
        var_vha1_rdn5 = 0.0;
        var_vha1_rdb0 = 0.0;
        var_vha1_rdb1 = 0.0;
        var_vha1_rdb2 = 0.0;
        var_vha1_rdb3 = 0.0;

        let assign30350_e45213: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard492 = assign30350_e45213;
        var_guard492_dn0 = 0.0;
        var_guard492_dn1 = 0.0;
        var_guard492_dn2 = 0.0;
        var_guard492_dn3 = 0.0;
        var_guard492_dn4 = 0.0;
        var_guard492_dn5 = 0.0;
        var_guard492_db0 = 0.0;
        var_guard492_db1 = 0.0;
        var_guard492_db2 = 0.0;
        var_guard492_db3 = 0.0;
        var_guard492_rv = 0.0;
        var_guard492_rdn0 = 0.0;
        var_guard492_rdn1 = 0.0;
        var_guard492_rdn2 = 0.0;
        var_guard492_rdn3 = 0.0;
        var_guard492_rdn4 = 0.0;
        var_guard492_rdn5 = 0.0;
        var_guard492_rdb0 = 0.0;
        var_guard492_rdb1 = 0.0;
        var_guard492_rdb2 = 0.0;
        var_guard492_rdb3 = 0.0;

        let (assign30360_e45231, assign30360_e45231_d_n0, assign30360_e45231_d_n1, assign30360_e45231_d_n2, assign30360_e45231_d_n3, assign30360_e45231_d_n4, assign30360_e45231_d_n5, assign30360_e45231_d_b0, assign30360_e45231_d_b1, assign30360_e45231_d_b2, assign30360_e45231_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30360_e45226: f64 = (var_vmax - var_vha1);
        let assign30360_e45227: f64 = (p.p86 * assign30360_e45226);
        let assign30360_e45229: f64 = (assign30360_e45227 + var_nfabot_i);
        (assign30360_e45229, ((p.p86 * (var_vmax_dn0 - var_vha1_dn0)) + var_nfabot_i_dn0), ((p.p86 * (var_vmax_dn1 - var_vha1_dn1)) + var_nfabot_i_dn1), ((p.p86 * (var_vmax_dn2 - var_vha1_dn2)) + var_nfabot_i_dn2), ((p.p86 * (var_vmax_dn3 - var_vha1_dn3)) + var_nfabot_i_dn3), ((p.p86 * (var_vmax_dn4 - var_vha1_dn4)) + var_nfabot_i_dn4), ((p.p86 * (var_vmax_dn5 - var_vha1_dn5)) + var_nfabot_i_dn5), ((p.p86 * (var_vmax_db0 - var_vha1_db0)) + var_nfabot_i_db0), ((p.p86 * (var_vmax_db1 - var_vha1_db1)) + var_nfabot_i_db1), ((p.p86 * (var_vmax_db2 - var_vha1_db2)) + var_nfabot_i_db2), ((p.p86 * (var_vmax_db3 - var_vha1_db3)) + var_nfabot_i_db3),)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign30360_e45231;
        var_nja10_dn0 = assign30360_e45231_d_n0;
        var_nja10_dn1 = assign30360_e45231_d_n1;
        var_nja10_dn2 = assign30360_e45231_d_n2;
        var_nja10_dn3 = assign30360_e45231_d_n3;
        var_nja10_dn4 = assign30360_e45231_d_n4;
        var_nja10_dn5 = assign30360_e45231_d_n5;
        var_nja10_db0 = assign30360_e45231_d_b0;
        var_nja10_db1 = assign30360_e45231_d_b1;
        var_nja10_db2 = assign30360_e45231_d_b2;
        var_nja10_db3 = assign30360_e45231_d_b3;
        var_nja10_rv = 0.0;
        var_nja10_rdn0 = 0.0;
        var_nja10_rdn1 = 0.0;
        var_nja10_rdn2 = 0.0;
        var_nja10_rdn3 = 0.0;
        var_nja10_rdn4 = 0.0;
        var_nja10_rdn5 = 0.0;
        var_nja10_rdb0 = 0.0;
        var_nja10_rdb1 = 0.0;
        var_nja10_rdb2 = 0.0;
        var_nja10_rdb3 = 0.0;

        *var_guard492_slot = var_guard492;
        *var_guard492_db0_slot = var_guard492_db0;
        *var_guard492_db1_slot = var_guard492_db1;
        *var_guard492_db2_slot = var_guard492_db2;
        *var_guard492_db3_slot = var_guard492_db3;
        *var_guard492_dn0_slot = var_guard492_dn0;
        *var_guard492_dn1_slot = var_guard492_dn1;
        *var_guard492_dn2_slot = var_guard492_dn2;
        *var_guard492_dn3_slot = var_guard492_dn3;
        *var_guard492_dn4_slot = var_guard492_dn4;
        *var_guard492_dn5_slot = var_guard492_dn5;
        *var_guard492_rdb0_slot = var_guard492_rdb0;
        *var_guard492_rdb1_slot = var_guard492_rdb1;
        *var_guard492_rdb2_slot = var_guard492_rdb2;
        *var_guard492_rdb3_slot = var_guard492_rdb3;
        *var_guard492_rdn0_slot = var_guard492_rdn0;
        *var_guard492_rdn1_slot = var_guard492_rdn1;
        *var_guard492_rdn2_slot = var_guard492_rdn2;
        *var_guard492_rdn3_slot = var_guard492_rdn3;
        *var_guard492_rdn4_slot = var_guard492_rdn4;
        *var_guard492_rdn5_slot = var_guard492_rdn5;
        *var_guard492_rv_slot = var_guard492_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj0_rdb0_slot = var_nj0_rdb0;
        *var_nj0_rdb1_slot = var_nj0_rdb1;
        *var_nj0_rdb2_slot = var_nj0_rdb2;
        *var_nj0_rdb3_slot = var_nj0_rdb3;
        *var_nj0_rdn0_slot = var_nj0_rdn0;
        *var_nj0_rdn1_slot = var_nj0_rdn1;
        *var_nj0_rdn2_slot = var_nj0_rdn2;
        *var_nj0_rdn3_slot = var_nj0_rdn3;
        *var_nj0_rdn4_slot = var_nj0_rdn4;
        *var_nj0_rdn5_slot = var_nj0_rdn5;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nj1_rdb0_slot = var_nj1_rdb0;
        *var_nj1_rdb1_slot = var_nj1_rdb1;
        *var_nj1_rdb2_slot = var_nj1_rdb2;
        *var_nj1_rdb3_slot = var_nj1_rdb3;
        *var_nj1_rdn0_slot = var_nj1_rdn0;
        *var_nj1_rdn1_slot = var_nj1_rdn1;
        *var_nj1_rdn2_slot = var_nj1_rdn2;
        *var_nj1_rdn3_slot = var_nj1_rdn3;
        *var_nj1_rdn4_slot = var_nj1_rdn4;
        *var_nj1_rdn5_slot = var_nj1_rdn5;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_nja10_rdb0_slot = var_nja10_rdb0;
        *var_nja10_rdb1_slot = var_nja10_rdb1;
        *var_nja10_rdb2_slot = var_nja10_rdb2;
        *var_nja10_rdb3_slot = var_nja10_rdb3;
        *var_nja10_rdn0_slot = var_nja10_rdn0;
        *var_nja10_rdn1_slot = var_nja10_rdn1;
        *var_nja10_rdn2_slot = var_nja10_rdn2;
        *var_nja10_rdn3_slot = var_nja10_rdn3;
        *var_nja10_rdn4_slot = var_nja10_rdn4;
        *var_nja10_rdn5_slot = var_nja10_rdn5;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_db0_slot = var_pnn0_db0;
        *var_pnn0_db1_slot = var_pnn0_db1;
        *var_pnn0_db2_slot = var_pnn0_db2;
        *var_pnn0_db3_slot = var_pnn0_db3;
        *var_pnn0_dn0_slot = var_pnn0_dn0;
        *var_pnn0_dn1_slot = var_pnn0_dn1;
        *var_pnn0_dn2_slot = var_pnn0_dn2;
        *var_pnn0_dn3_slot = var_pnn0_dn3;
        *var_pnn0_dn4_slot = var_pnn0_dn4;
        *var_pnn0_dn5_slot = var_pnn0_dn5;
        *var_pnn0_rdb0_slot = var_pnn0_rdb0;
        *var_pnn0_rdb1_slot = var_pnn0_rdb1;
        *var_pnn0_rdb2_slot = var_pnn0_rdb2;
        *var_pnn0_rdb3_slot = var_pnn0_rdb3;
        *var_pnn0_rdn0_slot = var_pnn0_rdn0;
        *var_pnn0_rdn1_slot = var_pnn0_rdn1;
        *var_pnn0_rdn2_slot = var_pnn0_rdn2;
        *var_pnn0_rdn3_slot = var_pnn0_rdn3;
        *var_pnn0_rdn4_slot = var_pnn0_rdn4;
        *var_pnn0_rdn5_slot = var_pnn0_rdn5;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_rdb0_slot = var_tmf1_rdb0;
        *var_tmf1_rdb1_slot = var_tmf1_rdb1;
        *var_tmf1_rdb2_slot = var_tmf1_rdb2;
        *var_tmf1_rdb3_slot = var_tmf1_rdb3;
        *var_tmf1_rdn0_slot = var_tmf1_rdn0;
        *var_tmf1_rdn1_slot = var_tmf1_rdn1;
        *var_tmf1_rdn2_slot = var_tmf1_rdn2;
        *var_tmf1_rdn3_slot = var_tmf1_rdn3;
        *var_tmf1_rdn4_slot = var_tmf1_rdn4;
        *var_tmf1_rdn5_slot = var_tmf1_rdn5;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_rdb0_slot = var_tmf2_rdb0;
        *var_tmf2_rdb1_slot = var_tmf2_rdb1;
        *var_tmf2_rdb2_slot = var_tmf2_rdb2;
        *var_tmf2_rdb3_slot = var_tmf2_rdb3;
        *var_tmf2_rdn0_slot = var_tmf2_rdn0;
        *var_tmf2_rdn1_slot = var_tmf2_rdn1;
        *var_tmf2_rdn2_slot = var_tmf2_rdn2;
        *var_tmf2_rdn3_slot = var_tmf2_rdn3;
        *var_tmf2_rdn4_slot = var_tmf2_rdn4;
        *var_tmf2_rdn5_slot = var_tmf2_rdn5;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_db0_slot = var_vha1_db0;
        *var_vha1_db1_slot = var_vha1_db1;
        *var_vha1_db2_slot = var_vha1_db2;
        *var_vha1_db3_slot = var_vha1_db3;
        *var_vha1_dn0_slot = var_vha1_dn0;
        *var_vha1_dn1_slot = var_vha1_dn1;
        *var_vha1_dn2_slot = var_vha1_dn2;
        *var_vha1_dn3_slot = var_vha1_dn3;
        *var_vha1_dn4_slot = var_vha1_dn4;
        *var_vha1_dn5_slot = var_vha1_dn5;
        *var_vha1_rdb0_slot = var_vha1_rdb0;
        *var_vha1_rdb1_slot = var_vha1_rdb1;
        *var_vha1_rdb2_slot = var_vha1_rdb2;
        *var_vha1_rdb3_slot = var_vha1_rdb3;
        *var_vha1_rdn0_slot = var_vha1_rdn0;
        *var_vha1_rdn1_slot = var_vha1_rdn1;
        *var_vha1_rdn2_slot = var_vha1_rdn2;
        *var_vha1_rdn3_slot = var_vha1_rdn3;
        *var_vha1_rdn4_slot = var_vha1_rdn4;
        *var_vha1_rdn5_slot = var_vha1_rdn5;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_77(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard492: f64,
        var_nfabot_i: f64,
        var_nfabot_i_db0: f64,
        var_nfabot_i_db1: f64,
        var_nfabot_i_db2: f64,
        var_nfabot_i_db3: f64,
        var_nfabot_i_dn0: f64,
        var_nfabot_i_dn1: f64,
        var_nfabot_i_dn2: f64,
        var_nfabot_i_dn3: f64,
        var_nfabot_i_dn4: f64,
        var_nfabot_i_dn5: f64,
        var_nja10: f64,
        var_nja10_db0: f64,
        var_nja10_db1: f64,
        var_nja10_db2: f64,
        var_nja10_db3: f64,
        var_nja10_dn0: f64,
        var_nja10_dn1: f64,
        var_nja10_dn2: f64,
        var_nja10_dn3: f64,
        var_nja10_dn4: f64,
        var_nja10_dn5: f64,
        var_vha1: f64,
        var_vha1_db0: f64,
        var_vha1_db1: f64,
        var_vha1_db2: f64,
        var_vha1_db3: f64,
        var_vha1_dn0: f64,
        var_vha1_dn1: f64,
        var_vha1_dn2: f64,
        var_vha1_dn3: f64,
        var_vha1_dn4: f64,
        var_vha1_dn5: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_db0_slot: &mut f64,
        var_dfn_sl_db1_slot: &mut f64,
        var_dfn_sl_db2_slot: &mut f64,
        var_dfn_sl_db3_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn1_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_dn3_slot: &mut f64,
        var_dfn_sl_dn4_slot: &mut f64,
        var_dfn_sl_dn5_slot: &mut f64,
        var_dfn_sl_rdb0_slot: &mut f64,
        var_dfn_sl_rdb1_slot: &mut f64,
        var_dfn_sl_rdb2_slot: &mut f64,
        var_dfn_sl_rdb3_slot: &mut f64,
        var_dfn_sl_rdn0_slot: &mut f64,
        var_dfn_sl_rdn1_slot: &mut f64,
        var_dfn_sl_rdn2_slot: &mut f64,
        var_dfn_sl_rdn3_slot: &mut f64,
        var_dfn_sl_rdn4_slot: &mut f64,
        var_dfn_sl_rdn5_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_db0_slot: &mut f64,
        var_dfn_su_db1_slot: &mut f64,
        var_dfn_su_db2_slot: &mut f64,
        var_dfn_su_db3_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn1_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_dn3_slot: &mut f64,
        var_dfn_su_dn4_slot: &mut f64,
        var_dfn_su_dn5_slot: &mut f64,
        var_dfn_su_rdb0_slot: &mut f64,
        var_dfn_su_rdb1_slot: &mut f64,
        var_dfn_su_rdb2_slot: &mut f64,
        var_dfn_su_rdb3_slot: &mut f64,
        var_dfn_su_rdn0_slot: &mut f64,
        var_dfn_su_rdn1_slot: &mut f64,
        var_dfn_su_rdn2_slot: &mut f64,
        var_dfn_su_rdn3_slot: &mut f64,
        var_dfn_su_rdn4_slot: &mut f64,
        var_dfn_su_rdn5_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj0_rdb0_slot: &mut f64,
        var_nj0_rdb1_slot: &mut f64,
        var_nj0_rdb2_slot: &mut f64,
        var_nj0_rdb3_slot: &mut f64,
        var_nj0_rdn0_slot: &mut f64,
        var_nj0_rdn1_slot: &mut f64,
        var_nj0_rdn2_slot: &mut f64,
        var_nj0_rdn3_slot: &mut f64,
        var_nj0_rdn4_slot: &mut f64,
        var_nj0_rdn5_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nj1_rdb0_slot: &mut f64,
        var_nj1_rdb1_slot: &mut f64,
        var_nj1_rdb2_slot: &mut f64,
        var_nj1_rdb3_slot: &mut f64,
        var_nj1_rdn0_slot: &mut f64,
        var_nj1_rdn1_slot: &mut f64,
        var_nj1_rdn2_slot: &mut f64,
        var_nj1_rdn3_slot: &mut f64,
        var_nj1_rdn4_slot: &mut f64,
        var_nj1_rdn5_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_nja11_rdb0_slot: &mut f64,
        var_nja11_rdb1_slot: &mut f64,
        var_nja11_rdb2_slot: &mut f64,
        var_nja11_rdb3_slot: &mut f64,
        var_nja11_rdn0_slot: &mut f64,
        var_nja11_rdn1_slot: &mut f64,
        var_nja11_rdn2_slot: &mut f64,
        var_nja11_rdn3_slot: &mut f64,
        var_nja11_rdn4_slot: &mut f64,
        var_nja11_rdn5_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_rdb0_slot: &mut f64,
        var_tmf1_rdb1_slot: &mut f64,
        var_tmf1_rdb2_slot: &mut f64,
        var_tmf1_rdb3_slot: &mut f64,
        var_tmf1_rdn0_slot: &mut f64,
        var_tmf1_rdn1_slot: &mut f64,
        var_tmf1_rdn2_slot: &mut f64,
        var_tmf1_rdn3_slot: &mut f64,
        var_tmf1_rdn4_slot: &mut f64,
        var_tmf1_rdn5_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_rdb0_slot: &mut f64,
        var_tmf2_rdb1_slot: &mut f64,
        var_tmf2_rdb2_slot: &mut f64,
        var_tmf2_rdb3_slot: &mut f64,
        var_tmf2_rdn0_slot: &mut f64,
        var_tmf2_rdn1_slot: &mut f64,
        var_tmf2_rdn2_slot: &mut f64,
        var_tmf2_rdn3_slot: &mut f64,
        var_tmf2_rdn4_slot: &mut f64,
        var_tmf2_rdn5_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_db0: f64 = *var_dfn_sl_db0_slot;
        let mut var_dfn_sl_db1: f64 = *var_dfn_sl_db1_slot;
        let mut var_dfn_sl_db2: f64 = *var_dfn_sl_db2_slot;
        let mut var_dfn_sl_db3: f64 = *var_dfn_sl_db3_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn1: f64 = *var_dfn_sl_dn1_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_dn3: f64 = *var_dfn_sl_dn3_slot;
        let mut var_dfn_sl_dn4: f64 = *var_dfn_sl_dn4_slot;
        let mut var_dfn_sl_dn5: f64 = *var_dfn_sl_dn5_slot;
        let mut var_dfn_sl_rdb0: f64 = *var_dfn_sl_rdb0_slot;
        let mut var_dfn_sl_rdb1: f64 = *var_dfn_sl_rdb1_slot;
        let mut var_dfn_sl_rdb2: f64 = *var_dfn_sl_rdb2_slot;
        let mut var_dfn_sl_rdb3: f64 = *var_dfn_sl_rdb3_slot;
        let mut var_dfn_sl_rdn0: f64 = *var_dfn_sl_rdn0_slot;
        let mut var_dfn_sl_rdn1: f64 = *var_dfn_sl_rdn1_slot;
        let mut var_dfn_sl_rdn2: f64 = *var_dfn_sl_rdn2_slot;
        let mut var_dfn_sl_rdn3: f64 = *var_dfn_sl_rdn3_slot;
        let mut var_dfn_sl_rdn4: f64 = *var_dfn_sl_rdn4_slot;
        let mut var_dfn_sl_rdn5: f64 = *var_dfn_sl_rdn5_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_db0: f64 = *var_dfn_su_db0_slot;
        let mut var_dfn_su_db1: f64 = *var_dfn_su_db1_slot;
        let mut var_dfn_su_db2: f64 = *var_dfn_su_db2_slot;
        let mut var_dfn_su_db3: f64 = *var_dfn_su_db3_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn1: f64 = *var_dfn_su_dn1_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_dn3: f64 = *var_dfn_su_dn3_slot;
        let mut var_dfn_su_dn4: f64 = *var_dfn_su_dn4_slot;
        let mut var_dfn_su_dn5: f64 = *var_dfn_su_dn5_slot;
        let mut var_dfn_su_rdb0: f64 = *var_dfn_su_rdb0_slot;
        let mut var_dfn_su_rdb1: f64 = *var_dfn_su_rdb1_slot;
        let mut var_dfn_su_rdb2: f64 = *var_dfn_su_rdb2_slot;
        let mut var_dfn_su_rdb3: f64 = *var_dfn_su_rdb3_slot;
        let mut var_dfn_su_rdn0: f64 = *var_dfn_su_rdn0_slot;
        let mut var_dfn_su_rdn1: f64 = *var_dfn_su_rdn1_slot;
        let mut var_dfn_su_rdn2: f64 = *var_dfn_su_rdn2_slot;
        let mut var_dfn_su_rdn3: f64 = *var_dfn_su_rdn3_slot;
        let mut var_dfn_su_rdn4: f64 = *var_dfn_su_rdn4_slot;
        let mut var_dfn_su_rdn5: f64 = *var_dfn_su_rdn5_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj0_rdb0: f64 = *var_nj0_rdb0_slot;
        let mut var_nj0_rdb1: f64 = *var_nj0_rdb1_slot;
        let mut var_nj0_rdb2: f64 = *var_nj0_rdb2_slot;
        let mut var_nj0_rdb3: f64 = *var_nj0_rdb3_slot;
        let mut var_nj0_rdn0: f64 = *var_nj0_rdn0_slot;
        let mut var_nj0_rdn1: f64 = *var_nj0_rdn1_slot;
        let mut var_nj0_rdn2: f64 = *var_nj0_rdn2_slot;
        let mut var_nj0_rdn3: f64 = *var_nj0_rdn3_slot;
        let mut var_nj0_rdn4: f64 = *var_nj0_rdn4_slot;
        let mut var_nj0_rdn5: f64 = *var_nj0_rdn5_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nj1_rdb0: f64 = *var_nj1_rdb0_slot;
        let mut var_nj1_rdb1: f64 = *var_nj1_rdb1_slot;
        let mut var_nj1_rdb2: f64 = *var_nj1_rdb2_slot;
        let mut var_nj1_rdb3: f64 = *var_nj1_rdb3_slot;
        let mut var_nj1_rdn0: f64 = *var_nj1_rdn0_slot;
        let mut var_nj1_rdn1: f64 = *var_nj1_rdn1_slot;
        let mut var_nj1_rdn2: f64 = *var_nj1_rdn2_slot;
        let mut var_nj1_rdn3: f64 = *var_nj1_rdn3_slot;
        let mut var_nj1_rdn4: f64 = *var_nj1_rdn4_slot;
        let mut var_nj1_rdn5: f64 = *var_nj1_rdn5_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_nja11_rdb0: f64 = *var_nja11_rdb0_slot;
        let mut var_nja11_rdb1: f64 = *var_nja11_rdb1_slot;
        let mut var_nja11_rdb2: f64 = *var_nja11_rdb2_slot;
        let mut var_nja11_rdb3: f64 = *var_nja11_rdb3_slot;
        let mut var_nja11_rdn0: f64 = *var_nja11_rdn0_slot;
        let mut var_nja11_rdn1: f64 = *var_nja11_rdn1_slot;
        let mut var_nja11_rdn2: f64 = *var_nja11_rdn2_slot;
        let mut var_nja11_rdn3: f64 = *var_nja11_rdn3_slot;
        let mut var_nja11_rdn4: f64 = *var_nja11_rdn4_slot;
        let mut var_nja11_rdn5: f64 = *var_nja11_rdn5_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_rdb0: f64 = *var_tmf1_rdb0_slot;
        let mut var_tmf1_rdb1: f64 = *var_tmf1_rdb1_slot;
        let mut var_tmf1_rdb2: f64 = *var_tmf1_rdb2_slot;
        let mut var_tmf1_rdb3: f64 = *var_tmf1_rdb3_slot;
        let mut var_tmf1_rdn0: f64 = *var_tmf1_rdn0_slot;
        let mut var_tmf1_rdn1: f64 = *var_tmf1_rdn1_slot;
        let mut var_tmf1_rdn2: f64 = *var_tmf1_rdn2_slot;
        let mut var_tmf1_rdn3: f64 = *var_tmf1_rdn3_slot;
        let mut var_tmf1_rdn4: f64 = *var_tmf1_rdn4_slot;
        let mut var_tmf1_rdn5: f64 = *var_tmf1_rdn5_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_rdb0: f64 = *var_tmf2_rdb0_slot;
        let mut var_tmf2_rdb1: f64 = *var_tmf2_rdb1_slot;
        let mut var_tmf2_rdb2: f64 = *var_tmf2_rdb2_slot;
        let mut var_tmf2_rdb3: f64 = *var_tmf2_rdb3_slot;
        let mut var_tmf2_rdn0: f64 = *var_tmf2_rdn0_slot;
        let mut var_tmf2_rdn1: f64 = *var_tmf2_rdn1_slot;
        let mut var_tmf2_rdn2: f64 = *var_tmf2_rdn2_slot;
        let mut var_tmf2_rdn3: f64 = *var_tmf2_rdn3_slot;
        let mut var_tmf2_rdn4: f64 = *var_tmf2_rdn4_slot;
        let mut var_tmf2_rdn5: f64 = *var_tmf2_rdn5_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign30370_e45247, assign30370_e45247_d_n0, assign30370_e45247_d_n1, assign30370_e45247_d_n2, assign30370_e45247_d_n3, assign30370_e45247_d_n4, assign30370_e45247_d_n5, assign30370_e45247_d_b0, assign30370_e45247_d_b1, assign30370_e45247_d_b2, assign30370_e45247_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30370_e45244: f64 = (p.p86 * var_vha1);
        let assign30370_e45245: f64 = (var_nfabot_i - assign30370_e45244);
        (assign30370_e45245, (var_nfabot_i_dn0 - (p.p86 * var_vha1_dn0)), (var_nfabot_i_dn1 - (p.p86 * var_vha1_dn1)), (var_nfabot_i_dn2 - (p.p86 * var_vha1_dn2)), (var_nfabot_i_dn3 - (p.p86 * var_vha1_dn3)), (var_nfabot_i_dn4 - (p.p86 * var_vha1_dn4)), (var_nfabot_i_dn5 - (p.p86 * var_vha1_dn5)), (var_nfabot_i_db0 - (p.p86 * var_vha1_db0)), (var_nfabot_i_db1 - (p.p86 * var_vha1_db1)), (var_nfabot_i_db2 - (p.p86 * var_vha1_db2)), (var_nfabot_i_db3 - (p.p86 * var_vha1_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30370_e45247;
        var_nj0_dn0 = assign30370_e45247_d_n0;
        var_nj0_dn1 = assign30370_e45247_d_n1;
        var_nj0_dn2 = assign30370_e45247_d_n2;
        var_nj0_dn3 = assign30370_e45247_d_n3;
        var_nj0_dn4 = assign30370_e45247_d_n4;
        var_nj0_dn5 = assign30370_e45247_d_n5;
        var_nj0_db0 = assign30370_e45247_d_b0;
        var_nj0_db1 = assign30370_e45247_d_b1;
        var_nj0_db2 = assign30370_e45247_d_b2;
        var_nj0_db3 = assign30370_e45247_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign30380_e45263, assign30380_e45263_d_n0, assign30380_e45263_d_n1, assign30380_e45263_d_n2, assign30380_e45263_d_n3, assign30380_e45263_d_n4, assign30380_e45263_d_n5, assign30380_e45263_d_b0, assign30380_e45263_d_b1, assign30380_e45263_d_b2, assign30380_e45263_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30380_e45259: f64 = (p.p85 - var_nja10);
        let assign30380_e45261: f64 = (assign30380_e45259 - 0.01);
        (assign30380_e45261, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30380_e45263;
        var_tmf1_dn0 = assign30380_e45263_d_n0;
        var_tmf1_dn1 = assign30380_e45263_d_n1;
        var_tmf1_dn2 = assign30380_e45263_d_n2;
        var_tmf1_dn3 = assign30380_e45263_d_n3;
        var_tmf1_dn4 = assign30380_e45263_d_n4;
        var_tmf1_dn5 = assign30380_e45263_d_n5;
        var_tmf1_db0 = assign30380_e45263_d_b0;
        var_tmf1_db1 = assign30380_e45263_d_b1;
        var_tmf1_db2 = assign30380_e45263_d_b2;
        var_tmf1_db3 = assign30380_e45263_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign30390_e45279, assign30390_e45279_d_n0, assign30390_e45279_d_n1, assign30390_e45279_d_n2, assign30390_e45279_d_n3, assign30390_e45279_d_n4, assign30390_e45279_d_n5, assign30390_e45279_d_b0, assign30390_e45279_d_b1, assign30390_e45279_d_b2, assign30390_e45279_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30390_e45275: f64 = (4.0 * p.p85);
        let assign30390_e45277: f64 = (assign30390_e45275 * 0.01);
        (assign30390_e45277, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30390_e45279;
        var_tmf2_dn0 = assign30390_e45279_d_n0;
        var_tmf2_dn1 = assign30390_e45279_d_n1;
        var_tmf2_dn2 = assign30390_e45279_d_n2;
        var_tmf2_dn3 = assign30390_e45279_d_n3;
        var_tmf2_dn4 = assign30390_e45279_d_n4;
        var_tmf2_dn5 = assign30390_e45279_d_n5;
        var_tmf2_db0 = assign30390_e45279_d_b0;
        var_tmf2_db1 = assign30390_e45279_d_b1;
        var_tmf2_db2 = assign30390_e45279_d_b2;
        var_tmf2_db3 = assign30390_e45279_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30400_e45297, assign30400_e45297_d_n0, assign30400_e45297_d_n1, assign30400_e45297_d_n2, assign30400_e45297_d_n3, assign30400_e45297_d_n4, assign30400_e45297_d_n5, assign30400_e45297_d_b0, assign30400_e45297_d_b1, assign30400_e45297_d_b2, assign30400_e45297_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let (assign30400_e45295, assign30400_e45295_d_n0, assign30400_e45295_d_n1, assign30400_e45295_d_n2, assign30400_e45295_d_n3, assign30400_e45295_d_n4, assign30400_e45295_d_n5, assign30400_e45295_d_b0, assign30400_e45295_d_b1, assign30400_e45295_d_b2, assign30400_e45295_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30400_e45294: f64 = (-var_tmf2);
                (assign30400_e45294, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30400_e45295, assign30400_e45295_d_n0, assign30400_e45295_d_n1, assign30400_e45295_d_n2, assign30400_e45295_d_n3, assign30400_e45295_d_n4, assign30400_e45295_d_n5, assign30400_e45295_d_b0, assign30400_e45295_d_b1, assign30400_e45295_d_b2, assign30400_e45295_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30400_e45297;
        var_tmf2_dn0 = assign30400_e45297_d_n0;
        var_tmf2_dn1 = assign30400_e45297_d_n1;
        var_tmf2_dn2 = assign30400_e45297_d_n2;
        var_tmf2_dn3 = assign30400_e45297_d_n3;
        var_tmf2_dn4 = assign30400_e45297_d_n4;
        var_tmf2_dn5 = assign30400_e45297_d_n5;
        var_tmf2_db0 = assign30400_e45297_d_b0;
        var_tmf2_db1 = assign30400_e45297_d_b1;
        var_tmf2_db2 = assign30400_e45297_d_b2;
        var_tmf2_db3 = assign30400_e45297_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30410_e45314, assign30410_e45314_d_n0, assign30410_e45314_d_n1, assign30410_e45314_d_n2, assign30410_e45314_d_n3, assign30410_e45314_d_n4, assign30410_e45314_d_n5, assign30410_e45314_d_b0, assign30410_e45314_d_b1, assign30410_e45314_d_b2, assign30410_e45314_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30410_e45309: f64 = (var_tmf1 * var_tmf1);
        let assign30410_e45311: f64 = (assign30410_e45309 + var_tmf2);
        let assign30410_e45312: f64 = (assign30410_e45311).sqrt();
        (assign30410_e45312, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30410_e45312)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30410_e45312)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30410_e45312)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30410_e45312)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30410_e45312)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30410_e45312)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30410_e45312)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30410_e45312)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30410_e45312)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30410_e45312)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30410_e45314;
        var_tmf2_dn0 = assign30410_e45314_d_n0;
        var_tmf2_dn1 = assign30410_e45314_d_n1;
        var_tmf2_dn2 = assign30410_e45314_d_n2;
        var_tmf2_dn3 = assign30410_e45314_d_n3;
        var_tmf2_dn4 = assign30410_e45314_d_n4;
        var_tmf2_dn5 = assign30410_e45314_d_n5;
        var_tmf2_db0 = assign30410_e45314_d_b0;
        var_tmf2_db1 = assign30410_e45314_d_b1;
        var_tmf2_db2 = assign30410_e45314_d_b2;
        var_tmf2_db3 = assign30410_e45314_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30420_e45332, assign30420_e45332_d_n0, assign30420_e45332_d_n1, assign30420_e45332_d_n2, assign30420_e45332_d_n3, assign30420_e45332_d_n4, assign30420_e45332_d_n5, assign30420_e45332_d_b0, assign30420_e45332_d_b1, assign30420_e45332_d_b2, assign30420_e45332_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30420_e45328: f64 = (var_tmf1 / var_tmf2);
        let assign30420_e45329: f64 = (1.0 + assign30420_e45328);
        let assign30420_e45330: f64 = (0.5 * assign30420_e45329);
        (assign30420_e45330, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign30420_e45332;
        var_dfn_su_dn0 = assign30420_e45332_d_n0;
        var_dfn_su_dn1 = assign30420_e45332_d_n1;
        var_dfn_su_dn2 = assign30420_e45332_d_n2;
        var_dfn_su_dn3 = assign30420_e45332_d_n3;
        var_dfn_su_dn4 = assign30420_e45332_d_n4;
        var_dfn_su_dn5 = assign30420_e45332_d_n5;
        var_dfn_su_db0 = assign30420_e45332_d_b0;
        var_dfn_su_db1 = assign30420_e45332_d_b1;
        var_dfn_su_db2 = assign30420_e45332_d_b2;
        var_dfn_su_db3 = assign30420_e45332_d_b3;
        var_dfn_su_rv = 0.0;
        var_dfn_su_rdn0 = 0.0;
        var_dfn_su_rdn1 = 0.0;
        var_dfn_su_rdn2 = 0.0;
        var_dfn_su_rdn3 = 0.0;
        var_dfn_su_rdn4 = 0.0;
        var_dfn_su_rdn5 = 0.0;
        var_dfn_su_rdb0 = 0.0;
        var_dfn_su_rdb1 = 0.0;
        var_dfn_su_rdb2 = 0.0;
        var_dfn_su_rdb3 = 0.0;

        let (assign30430_e45350, assign30430_e45350_d_n0, assign30430_e45350_d_n1, assign30430_e45350_d_n2, assign30430_e45350_d_n3, assign30430_e45350_d_n4, assign30430_e45350_d_n5, assign30430_e45350_d_b0, assign30430_e45350_d_b1, assign30430_e45350_d_b2, assign30430_e45350_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30430_e45346: f64 = (var_tmf1 + var_tmf2);
        let assign30430_e45347: f64 = (0.5 * assign30430_e45346);
        let assign30430_e45348: f64 = (p.p85 - assign30430_e45347);
        (assign30430_e45348, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign30430_e45350;
        var_nja11_dn0 = assign30430_e45350_d_n0;
        var_nja11_dn1 = assign30430_e45350_d_n1;
        var_nja11_dn2 = assign30430_e45350_d_n2;
        var_nja11_dn3 = assign30430_e45350_d_n3;
        var_nja11_dn4 = assign30430_e45350_d_n4;
        var_nja11_dn5 = assign30430_e45350_d_n5;
        var_nja11_db0 = assign30430_e45350_d_b0;
        var_nja11_db1 = assign30430_e45350_d_b1;
        var_nja11_db2 = assign30430_e45350_d_b2;
        var_nja11_db3 = assign30430_e45350_d_b3;
        var_nja11_rv = 0.0;
        var_nja11_rdn0 = 0.0;
        var_nja11_rdn1 = 0.0;
        var_nja11_rdn2 = 0.0;
        var_nja11_rdn3 = 0.0;
        var_nja11_rdn4 = 0.0;
        var_nja11_rdn5 = 0.0;
        var_nja11_rdb0 = 0.0;
        var_nja11_rdb1 = 0.0;
        var_nja11_rdb2 = 0.0;
        var_nja11_rdb3 = 0.0;

        let (assign30440_e45366, assign30440_e45366_d_n0, assign30440_e45366_d_n1, assign30440_e45366_d_n2, assign30440_e45366_d_n3, assign30440_e45366_d_n4, assign30440_e45366_d_n5, assign30440_e45366_d_b0, assign30440_e45366_d_b1, assign30440_e45366_d_b2, assign30440_e45366_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30440_e45362: f64 = (var_nja11 - var_nfabot_i);
        let assign30440_e45364: f64 = (assign30440_e45362 - 0.01);
        (assign30440_e45364, (var_nja11_dn0 - var_nfabot_i_dn0), (var_nja11_dn1 - var_nfabot_i_dn1), (var_nja11_dn2 - var_nfabot_i_dn2), (var_nja11_dn3 - var_nfabot_i_dn3), (var_nja11_dn4 - var_nfabot_i_dn4), (var_nja11_dn5 - var_nfabot_i_dn5), (var_nja11_db0 - var_nfabot_i_db0), (var_nja11_db1 - var_nfabot_i_db1), (var_nja11_db2 - var_nfabot_i_db2), (var_nja11_db3 - var_nfabot_i_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30440_e45366;
        var_tmf1_dn0 = assign30440_e45366_d_n0;
        var_tmf1_dn1 = assign30440_e45366_d_n1;
        var_tmf1_dn2 = assign30440_e45366_d_n2;
        var_tmf1_dn3 = assign30440_e45366_d_n3;
        var_tmf1_dn4 = assign30440_e45366_d_n4;
        var_tmf1_dn5 = assign30440_e45366_d_n5;
        var_tmf1_db0 = assign30440_e45366_d_b0;
        var_tmf1_db1 = assign30440_e45366_d_b1;
        var_tmf1_db2 = assign30440_e45366_d_b2;
        var_tmf1_db3 = assign30440_e45366_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign30450_e45382, assign30450_e45382_d_n0, assign30450_e45382_d_n1, assign30450_e45382_d_n2, assign30450_e45382_d_n3, assign30450_e45382_d_n4, assign30450_e45382_d_n5, assign30450_e45382_d_b0, assign30450_e45382_d_b1, assign30450_e45382_d_b2, assign30450_e45382_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30450_e45378: f64 = (4.0 * var_nfabot_i);
        let assign30450_e45380: f64 = (assign30450_e45378 * 0.01);
        (assign30450_e45380, ((4.0 * var_nfabot_i_dn0) * 0.01), ((4.0 * var_nfabot_i_dn1) * 0.01), ((4.0 * var_nfabot_i_dn2) * 0.01), ((4.0 * var_nfabot_i_dn3) * 0.01), ((4.0 * var_nfabot_i_dn4) * 0.01), ((4.0 * var_nfabot_i_dn5) * 0.01), ((4.0 * var_nfabot_i_db0) * 0.01), ((4.0 * var_nfabot_i_db1) * 0.01), ((4.0 * var_nfabot_i_db2) * 0.01), ((4.0 * var_nfabot_i_db3) * 0.01),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30450_e45382;
        var_tmf2_dn0 = assign30450_e45382_d_n0;
        var_tmf2_dn1 = assign30450_e45382_d_n1;
        var_tmf2_dn2 = assign30450_e45382_d_n2;
        var_tmf2_dn3 = assign30450_e45382_d_n3;
        var_tmf2_dn4 = assign30450_e45382_d_n4;
        var_tmf2_dn5 = assign30450_e45382_d_n5;
        var_tmf2_db0 = assign30450_e45382_d_b0;
        var_tmf2_db1 = assign30450_e45382_d_b1;
        var_tmf2_db2 = assign30450_e45382_d_b2;
        var_tmf2_db3 = assign30450_e45382_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30460_e45400, assign30460_e45400_d_n0, assign30460_e45400_d_n1, assign30460_e45400_d_n2, assign30460_e45400_d_n3, assign30460_e45400_d_n4, assign30460_e45400_d_n5, assign30460_e45400_d_b0, assign30460_e45400_d_b1, assign30460_e45400_d_b2, assign30460_e45400_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let (assign30460_e45398, assign30460_e45398_d_n0, assign30460_e45398_d_n1, assign30460_e45398_d_n2, assign30460_e45398_d_n3, assign30460_e45398_d_n4, assign30460_e45398_d_n5, assign30460_e45398_d_b0, assign30460_e45398_d_b1, assign30460_e45398_d_b2, assign30460_e45398_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30460_e45397: f64 = (-var_tmf2);
                (assign30460_e45397, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30460_e45398, assign30460_e45398_d_n0, assign30460_e45398_d_n1, assign30460_e45398_d_n2, assign30460_e45398_d_n3, assign30460_e45398_d_n4, assign30460_e45398_d_n5, assign30460_e45398_d_b0, assign30460_e45398_d_b1, assign30460_e45398_d_b2, assign30460_e45398_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30460_e45400;
        var_tmf2_dn0 = assign30460_e45400_d_n0;
        var_tmf2_dn1 = assign30460_e45400_d_n1;
        var_tmf2_dn2 = assign30460_e45400_d_n2;
        var_tmf2_dn3 = assign30460_e45400_d_n3;
        var_tmf2_dn4 = assign30460_e45400_d_n4;
        var_tmf2_dn5 = assign30460_e45400_d_n5;
        var_tmf2_db0 = assign30460_e45400_d_b0;
        var_tmf2_db1 = assign30460_e45400_d_b1;
        var_tmf2_db2 = assign30460_e45400_d_b2;
        var_tmf2_db3 = assign30460_e45400_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30470_e45417, assign30470_e45417_d_n0, assign30470_e45417_d_n1, assign30470_e45417_d_n2, assign30470_e45417_d_n3, assign30470_e45417_d_n4, assign30470_e45417_d_n5, assign30470_e45417_d_b0, assign30470_e45417_d_b1, assign30470_e45417_d_b2, assign30470_e45417_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30470_e45412: f64 = (var_tmf1 * var_tmf1);
        let assign30470_e45414: f64 = (assign30470_e45412 + var_tmf2);
        let assign30470_e45415: f64 = (assign30470_e45414).sqrt();
        (assign30470_e45415, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30470_e45415)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30470_e45415)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30470_e45415)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30470_e45415)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30470_e45415)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30470_e45415)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30470_e45415)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30470_e45415)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30470_e45415)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30470_e45415)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30470_e45417;
        var_tmf2_dn0 = assign30470_e45417_d_n0;
        var_tmf2_dn1 = assign30470_e45417_d_n1;
        var_tmf2_dn2 = assign30470_e45417_d_n2;
        var_tmf2_dn3 = assign30470_e45417_d_n3;
        var_tmf2_dn4 = assign30470_e45417_d_n4;
        var_tmf2_dn5 = assign30470_e45417_d_n5;
        var_tmf2_db0 = assign30470_e45417_d_b0;
        var_tmf2_db1 = assign30470_e45417_d_b1;
        var_tmf2_db2 = assign30470_e45417_d_b2;
        var_tmf2_db3 = assign30470_e45417_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30480_e45435, assign30480_e45435_d_n0, assign30480_e45435_d_n1, assign30480_e45435_d_n2, assign30480_e45435_d_n3, assign30480_e45435_d_n4, assign30480_e45435_d_n5, assign30480_e45435_d_b0, assign30480_e45435_d_b1, assign30480_e45435_d_b2, assign30480_e45435_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30480_e45431: f64 = (var_tmf1 / var_tmf2);
        let assign30480_e45432: f64 = (1.0 + assign30480_e45431);
        let assign30480_e45433: f64 = (0.5 * assign30480_e45432);
        (assign30480_e45433, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign30480_e45435;
        var_dfn_sl_dn0 = assign30480_e45435_d_n0;
        var_dfn_sl_dn1 = assign30480_e45435_d_n1;
        var_dfn_sl_dn2 = assign30480_e45435_d_n2;
        var_dfn_sl_dn3 = assign30480_e45435_d_n3;
        var_dfn_sl_dn4 = assign30480_e45435_d_n4;
        var_dfn_sl_dn5 = assign30480_e45435_d_n5;
        var_dfn_sl_db0 = assign30480_e45435_d_b0;
        var_dfn_sl_db1 = assign30480_e45435_d_b1;
        var_dfn_sl_db2 = assign30480_e45435_d_b2;
        var_dfn_sl_db3 = assign30480_e45435_d_b3;
        var_dfn_sl_rv = 0.0;
        var_dfn_sl_rdn0 = 0.0;
        var_dfn_sl_rdn1 = 0.0;
        var_dfn_sl_rdn2 = 0.0;
        var_dfn_sl_rdn3 = 0.0;
        var_dfn_sl_rdn4 = 0.0;
        var_dfn_sl_rdn5 = 0.0;
        var_dfn_sl_rdb0 = 0.0;
        var_dfn_sl_rdb1 = 0.0;
        var_dfn_sl_rdb2 = 0.0;
        var_dfn_sl_rdb3 = 0.0;

        let (assign30490_e45453, assign30490_e45453_d_n0, assign30490_e45453_d_n1, assign30490_e45453_d_n2, assign30490_e45453_d_n3, assign30490_e45453_d_n4, assign30490_e45453_d_n5, assign30490_e45453_d_b0, assign30490_e45453_d_b1, assign30490_e45453_d_b2, assign30490_e45453_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30490_e45449: f64 = (var_tmf1 + var_tmf2);
        let assign30490_e45450: f64 = (0.5 * assign30490_e45449);
        let assign30490_e45451: f64 = (var_nfabot_i + assign30490_e45450);
        (assign30490_e45451, (var_nfabot_i_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_nfabot_i_dn1 + (0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (var_nfabot_i_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_nfabot_i_dn3 + (0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (var_nfabot_i_dn4 + (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_nfabot_i_dn5 + (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_nfabot_i_db0 + (0.5 * (var_tmf1_db0 + var_tmf2_db0))), (var_nfabot_i_db1 + (0.5 * (var_tmf1_db1 + var_tmf2_db1))), (var_nfabot_i_db2 + (0.5 * (var_tmf1_db2 + var_tmf2_db2))), (var_nfabot_i_db3 + (0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign30490_e45453;
        var_nj1_dn0 = assign30490_e45453_d_n0;
        var_nj1_dn1 = assign30490_e45453_d_n1;
        var_nj1_dn2 = assign30490_e45453_d_n2;
        var_nj1_dn3 = assign30490_e45453_d_n3;
        var_nj1_dn4 = assign30490_e45453_d_n4;
        var_nj1_dn5 = assign30490_e45453_d_n5;
        var_nj1_db0 = assign30490_e45453_d_b0;
        var_nj1_db1 = assign30490_e45453_d_b1;
        var_nj1_db2 = assign30490_e45453_d_b2;
        var_nj1_db3 = assign30490_e45453_d_b3;
        var_nj1_rv = 0.0;
        var_nj1_rdn0 = 0.0;
        var_nj1_rdn1 = 0.0;
        var_nj1_rdn2 = 0.0;
        var_nj1_rdn3 = 0.0;
        var_nj1_rdn4 = 0.0;
        var_nj1_rdn5 = 0.0;
        var_nj1_rdb0 = 0.0;
        var_nj1_rdb1 = 0.0;
        var_nj1_rdb2 = 0.0;
        var_nj1_rdb3 = 0.0;

        let (assign30500_e45469, assign30500_e45469_d_n0, assign30500_e45469_d_n1, assign30500_e45469_d_n2, assign30500_e45469_d_n3, assign30500_e45469_d_n4, assign30500_e45469_d_n5, assign30500_e45469_d_b0, assign30500_e45469_d_b1, assign30500_e45469_d_b2, assign30500_e45469_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30500_e45465: f64 = (p.p85 - var_nj0);
        let assign30500_e45467: f64 = (assign30500_e45465 - 0.01);
        (assign30500_e45467, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30500_e45469;
        var_tmf1_dn0 = assign30500_e45469_d_n0;
        var_tmf1_dn1 = assign30500_e45469_d_n1;
        var_tmf1_dn2 = assign30500_e45469_d_n2;
        var_tmf1_dn3 = assign30500_e45469_d_n3;
        var_tmf1_dn4 = assign30500_e45469_d_n4;
        var_tmf1_dn5 = assign30500_e45469_d_n5;
        var_tmf1_db0 = assign30500_e45469_d_b0;
        var_tmf1_db1 = assign30500_e45469_d_b1;
        var_tmf1_db2 = assign30500_e45469_d_b2;
        var_tmf1_db3 = assign30500_e45469_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign30510_e45485, assign30510_e45485_d_n0, assign30510_e45485_d_n1, assign30510_e45485_d_n2, assign30510_e45485_d_n3, assign30510_e45485_d_n4, assign30510_e45485_d_n5, assign30510_e45485_d_b0, assign30510_e45485_d_b1, assign30510_e45485_d_b2, assign30510_e45485_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30510_e45481: f64 = (4.0 * p.p85);
        let assign30510_e45483: f64 = (assign30510_e45481 * 0.01);
        (assign30510_e45483, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30510_e45485;
        var_tmf2_dn0 = assign30510_e45485_d_n0;
        var_tmf2_dn1 = assign30510_e45485_d_n1;
        var_tmf2_dn2 = assign30510_e45485_d_n2;
        var_tmf2_dn3 = assign30510_e45485_d_n3;
        var_tmf2_dn4 = assign30510_e45485_d_n4;
        var_tmf2_dn5 = assign30510_e45485_d_n5;
        var_tmf2_db0 = assign30510_e45485_d_b0;
        var_tmf2_db1 = assign30510_e45485_d_b1;
        var_tmf2_db2 = assign30510_e45485_d_b2;
        var_tmf2_db3 = assign30510_e45485_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_db0_slot = var_dfn_sl_db0;
        *var_dfn_sl_db1_slot = var_dfn_sl_db1;
        *var_dfn_sl_db2_slot = var_dfn_sl_db2;
        *var_dfn_sl_db3_slot = var_dfn_sl_db3;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn1_slot = var_dfn_sl_dn1;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_dn3_slot = var_dfn_sl_dn3;
        *var_dfn_sl_dn4_slot = var_dfn_sl_dn4;
        *var_dfn_sl_dn5_slot = var_dfn_sl_dn5;
        *var_dfn_sl_rdb0_slot = var_dfn_sl_rdb0;
        *var_dfn_sl_rdb1_slot = var_dfn_sl_rdb1;
        *var_dfn_sl_rdb2_slot = var_dfn_sl_rdb2;
        *var_dfn_sl_rdb3_slot = var_dfn_sl_rdb3;
        *var_dfn_sl_rdn0_slot = var_dfn_sl_rdn0;
        *var_dfn_sl_rdn1_slot = var_dfn_sl_rdn1;
        *var_dfn_sl_rdn2_slot = var_dfn_sl_rdn2;
        *var_dfn_sl_rdn3_slot = var_dfn_sl_rdn3;
        *var_dfn_sl_rdn4_slot = var_dfn_sl_rdn4;
        *var_dfn_sl_rdn5_slot = var_dfn_sl_rdn5;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_db0_slot = var_dfn_su_db0;
        *var_dfn_su_db1_slot = var_dfn_su_db1;
        *var_dfn_su_db2_slot = var_dfn_su_db2;
        *var_dfn_su_db3_slot = var_dfn_su_db3;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn1_slot = var_dfn_su_dn1;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_dn3_slot = var_dfn_su_dn3;
        *var_dfn_su_dn4_slot = var_dfn_su_dn4;
        *var_dfn_su_dn5_slot = var_dfn_su_dn5;
        *var_dfn_su_rdb0_slot = var_dfn_su_rdb0;
        *var_dfn_su_rdb1_slot = var_dfn_su_rdb1;
        *var_dfn_su_rdb2_slot = var_dfn_su_rdb2;
        *var_dfn_su_rdb3_slot = var_dfn_su_rdb3;
        *var_dfn_su_rdn0_slot = var_dfn_su_rdn0;
        *var_dfn_su_rdn1_slot = var_dfn_su_rdn1;
        *var_dfn_su_rdn2_slot = var_dfn_su_rdn2;
        *var_dfn_su_rdn3_slot = var_dfn_su_rdn3;
        *var_dfn_su_rdn4_slot = var_dfn_su_rdn4;
        *var_dfn_su_rdn5_slot = var_dfn_su_rdn5;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj0_rdb0_slot = var_nj0_rdb0;
        *var_nj0_rdb1_slot = var_nj0_rdb1;
        *var_nj0_rdb2_slot = var_nj0_rdb2;
        *var_nj0_rdb3_slot = var_nj0_rdb3;
        *var_nj0_rdn0_slot = var_nj0_rdn0;
        *var_nj0_rdn1_slot = var_nj0_rdn1;
        *var_nj0_rdn2_slot = var_nj0_rdn2;
        *var_nj0_rdn3_slot = var_nj0_rdn3;
        *var_nj0_rdn4_slot = var_nj0_rdn4;
        *var_nj0_rdn5_slot = var_nj0_rdn5;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nj1_rdb0_slot = var_nj1_rdb0;
        *var_nj1_rdb1_slot = var_nj1_rdb1;
        *var_nj1_rdb2_slot = var_nj1_rdb2;
        *var_nj1_rdb3_slot = var_nj1_rdb3;
        *var_nj1_rdn0_slot = var_nj1_rdn0;
        *var_nj1_rdn1_slot = var_nj1_rdn1;
        *var_nj1_rdn2_slot = var_nj1_rdn2;
        *var_nj1_rdn3_slot = var_nj1_rdn3;
        *var_nj1_rdn4_slot = var_nj1_rdn4;
        *var_nj1_rdn5_slot = var_nj1_rdn5;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_nja11_rdb0_slot = var_nja11_rdb0;
        *var_nja11_rdb1_slot = var_nja11_rdb1;
        *var_nja11_rdb2_slot = var_nja11_rdb2;
        *var_nja11_rdb3_slot = var_nja11_rdb3;
        *var_nja11_rdn0_slot = var_nja11_rdn0;
        *var_nja11_rdn1_slot = var_nja11_rdn1;
        *var_nja11_rdn2_slot = var_nja11_rdn2;
        *var_nja11_rdn3_slot = var_nja11_rdn3;
        *var_nja11_rdn4_slot = var_nja11_rdn4;
        *var_nja11_rdn5_slot = var_nja11_rdn5;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_rdb0_slot = var_tmf1_rdb0;
        *var_tmf1_rdb1_slot = var_tmf1_rdb1;
        *var_tmf1_rdb2_slot = var_tmf1_rdb2;
        *var_tmf1_rdb3_slot = var_tmf1_rdb3;
        *var_tmf1_rdn0_slot = var_tmf1_rdn0;
        *var_tmf1_rdn1_slot = var_tmf1_rdn1;
        *var_tmf1_rdn2_slot = var_tmf1_rdn2;
        *var_tmf1_rdn3_slot = var_tmf1_rdn3;
        *var_tmf1_rdn4_slot = var_tmf1_rdn4;
        *var_tmf1_rdn5_slot = var_tmf1_rdn5;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_rdb0_slot = var_tmf2_rdb0;
        *var_tmf2_rdb1_slot = var_tmf2_rdb1;
        *var_tmf2_rdb2_slot = var_tmf2_rdb2;
        *var_tmf2_rdb3_slot = var_tmf2_rdb3;
        *var_tmf2_rdn0_slot = var_tmf2_rdn0;
        *var_tmf2_rdn1_slot = var_tmf2_rdn1;
        *var_tmf2_rdn2_slot = var_tmf2_rdn2;
        *var_tmf2_rdn3_slot = var_tmf2_rdn3;
        *var_tmf2_rdn4_slot = var_tmf2_rdn4;
        *var_tmf2_rdn5_slot = var_tmf2_rdn5;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_78(
        p: &Parameters,
        var_dfn_sl: f64,
        var_dfn_sl_db0: f64,
        var_dfn_sl_db1: f64,
        var_dfn_sl_db2: f64,
        var_dfn_sl_db3: f64,
        var_dfn_sl_dn0: f64,
        var_dfn_sl_dn1: f64,
        var_dfn_sl_dn2: f64,
        var_dfn_sl_dn3: f64,
        var_dfn_sl_dn4: f64,
        var_dfn_sl_dn5: f64,
        var_dfn_su: f64,
        var_dfn_su_db0: f64,
        var_dfn_su_db1: f64,
        var_dfn_su_db2: f64,
        var_dfn_su_db3: f64,
        var_dfn_su_dn0: f64,
        var_dfn_su_dn1: f64,
        var_dfn_su_dn2: f64,
        var_dfn_su_dn3: f64,
        var_dfn_su_dn4: f64,
        var_dfn_su_dn5: f64,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard492: f64,
        var_nfabot_i: f64,
        var_nfabot_i_db0: f64,
        var_nfabot_i_db1: f64,
        var_nfabot_i_db2: f64,
        var_nfabot_i_db3: f64,
        var_nfabot_i_dn0: f64,
        var_nfabot_i_dn1: f64,
        var_nfabot_i_dn2: f64,
        var_nfabot_i_dn3: f64,
        var_nfabot_i_dn4: f64,
        var_nfabot_i_dn5: f64,
        var_phitdinv: f64,
        var_phitdinv_db0: f64,
        var_phitdinv_db1: f64,
        var_phitdinv_db2: f64,
        var_phitdinv_db3: f64,
        var_phitdinv_dn0: f64,
        var_phitdinv_dn1: f64,
        var_phitdinv_dn2: f64,
        var_phitdinv_dn3: f64,
        var_phitdinv_dn4: f64,
        var_phitdinv_dn5: f64,
        var_vha1: f64,
        var_vha1_db0: f64,
        var_vha1_db1: f64,
        var_vha1_db2: f64,
        var_vha1_db3: f64,
        var_vha1_dn0: f64,
        var_vha1_dn1: f64,
        var_vha1_dn2: f64,
        var_vha1_dn3: f64,
        var_vha1_dn4: f64,
        var_vha1_dn5: f64,
        var_vmax: f64,
        var_vmax_db0: f64,
        var_vmax_db1: f64,
        var_vmax_db2: f64,
        var_vmax_db3: f64,
        var_vmax_dn0: f64,
        var_vmax_dn1: f64,
        var_vmax_dn2: f64,
        var_vmax_dn3: f64,
        var_vmax_dn4: f64,
        var_vmax_dn5: f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_db0_slot: &mut f64,
        var_dnj1_dv_db1_slot: &mut f64,
        var_dnj1_dv_db2_slot: &mut f64,
        var_dnj1_dv_db3_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn1_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_dn3_slot: &mut f64,
        var_dnj1_dv_dn4_slot: &mut f64,
        var_dnj1_dv_dn5_slot: &mut f64,
        var_dnj1_dv_rdb0_slot: &mut f64,
        var_dnj1_dv_rdb1_slot: &mut f64,
        var_dnj1_dv_rdb2_slot: &mut f64,
        var_dnj1_dv_rdb3_slot: &mut f64,
        var_dnj1_dv_rdn0_slot: &mut f64,
        var_dnj1_dv_rdn1_slot: &mut f64,
        var_dnj1_dv_rdn2_slot: &mut f64,
        var_dnj1_dv_rdn3_slot: &mut f64,
        var_dnj1_dv_rdn4_slot: &mut f64,
        var_dnj1_dv_rdn5_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn4_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn5_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdb0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdb1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdb2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdb3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdn1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdn3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdn4_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdn5_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rv_slot: &mut f64,
        var_guard493_slot: &mut f64,
        var_guard493_db0_slot: &mut f64,
        var_guard493_db1_slot: &mut f64,
        var_guard493_db2_slot: &mut f64,
        var_guard493_db3_slot: &mut f64,
        var_guard493_dn0_slot: &mut f64,
        var_guard493_dn1_slot: &mut f64,
        var_guard493_dn2_slot: &mut f64,
        var_guard493_dn3_slot: &mut f64,
        var_guard493_dn4_slot: &mut f64,
        var_guard493_dn5_slot: &mut f64,
        var_guard493_rdb0_slot: &mut f64,
        var_guard493_rdb1_slot: &mut f64,
        var_guard493_rdb2_slot: &mut f64,
        var_guard493_rdb3_slot: &mut f64,
        var_guard493_rdn0_slot: &mut f64,
        var_guard493_rdn1_slot: &mut f64,
        var_guard493_rdn2_slot: &mut f64,
        var_guard493_rdn3_slot: &mut f64,
        var_guard493_rdn4_slot: &mut f64,
        var_guard493_rdn5_slot: &mut f64,
        var_guard493_rv_slot: &mut f64,
        var_guard494_slot: &mut f64,
        var_guard494_db0_slot: &mut f64,
        var_guard494_db1_slot: &mut f64,
        var_guard494_db2_slot: &mut f64,
        var_guard494_db3_slot: &mut f64,
        var_guard494_dn0_slot: &mut f64,
        var_guard494_dn1_slot: &mut f64,
        var_guard494_dn2_slot: &mut f64,
        var_guard494_dn3_slot: &mut f64,
        var_guard494_dn4_slot: &mut f64,
        var_guard494_dn5_slot: &mut f64,
        var_guard494_rdb0_slot: &mut f64,
        var_guard494_rdb1_slot: &mut f64,
        var_guard494_rdb2_slot: &mut f64,
        var_guard494_rdb3_slot: &mut f64,
        var_guard494_rdn0_slot: &mut f64,
        var_guard494_rdn1_slot: &mut f64,
        var_guard494_rdn2_slot: &mut f64,
        var_guard494_rdn3_slot: &mut f64,
        var_guard494_rdn4_slot: &mut f64,
        var_guard494_rdn5_slot: &mut f64,
        var_guard494_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj0_rdb0_slot: &mut f64,
        var_nj0_rdb1_slot: &mut f64,
        var_nj0_rdb2_slot: &mut f64,
        var_nj0_rdb3_slot: &mut f64,
        var_nj0_rdn0_slot: &mut f64,
        var_nj0_rdn1_slot: &mut f64,
        var_nj0_rdn2_slot: &mut f64,
        var_nj0_rdn3_slot: &mut f64,
        var_nj0_rdn4_slot: &mut f64,
        var_nj0_rdn5_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nj1_rdb0_slot: &mut f64,
        var_nj1_rdb1_slot: &mut f64,
        var_nj1_rdb2_slot: &mut f64,
        var_nj1_rdb3_slot: &mut f64,
        var_nj1_rdn0_slot: &mut f64,
        var_nj1_rdn1_slot: &mut f64,
        var_nj1_rdn2_slot: &mut f64,
        var_nj1_rdn3_slot: &mut f64,
        var_nj1_rdn4_slot: &mut f64,
        var_nj1_rdn5_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_rdb0_slot: &mut f64,
        var_tmf1_rdb1_slot: &mut f64,
        var_tmf1_rdb2_slot: &mut f64,
        var_tmf1_rdb3_slot: &mut f64,
        var_tmf1_rdn0_slot: &mut f64,
        var_tmf1_rdn1_slot: &mut f64,
        var_tmf1_rdn2_slot: &mut f64,
        var_tmf1_rdn3_slot: &mut f64,
        var_tmf1_rdn4_slot: &mut f64,
        var_tmf1_rdn5_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_rdb0_slot: &mut f64,
        var_tmf2_rdb1_slot: &mut f64,
        var_tmf2_rdb2_slot: &mut f64,
        var_tmf2_rdb3_slot: &mut f64,
        var_tmf2_rdn0_slot: &mut f64,
        var_tmf2_rdn1_slot: &mut f64,
        var_tmf2_rdn2_slot: &mut f64,
        var_tmf2_rdn3_slot: &mut f64,
        var_tmf2_rdn4_slot: &mut f64,
        var_tmf2_rdn5_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_db0: f64 = *var_dnj1_dv_db0_slot;
        let mut var_dnj1_dv_db1: f64 = *var_dnj1_dv_db1_slot;
        let mut var_dnj1_dv_db2: f64 = *var_dnj1_dv_db2_slot;
        let mut var_dnj1_dv_db3: f64 = *var_dnj1_dv_db3_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn1: f64 = *var_dnj1_dv_dn1_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_dn3: f64 = *var_dnj1_dv_dn3_slot;
        let mut var_dnj1_dv_dn4: f64 = *var_dnj1_dv_dn4_slot;
        let mut var_dnj1_dv_dn5: f64 = *var_dnj1_dv_dn5_slot;
        let mut var_dnj1_dv_rdb0: f64 = *var_dnj1_dv_rdb0_slot;
        let mut var_dnj1_dv_rdb1: f64 = *var_dnj1_dv_rdb1_slot;
        let mut var_dnj1_dv_rdb2: f64 = *var_dnj1_dv_rdb2_slot;
        let mut var_dnj1_dv_rdb3: f64 = *var_dnj1_dv_rdb3_slot;
        let mut var_dnj1_dv_rdn0: f64 = *var_dnj1_dv_rdn0_slot;
        let mut var_dnj1_dv_rdn1: f64 = *var_dnj1_dv_rdn1_slot;
        let mut var_dnj1_dv_rdn2: f64 = *var_dnj1_dv_rdn2_slot;
        let mut var_dnj1_dv_rdn3: f64 = *var_dnj1_dv_rdn3_slot;
        let mut var_dnj1_dv_rdn4: f64 = *var_dnj1_dv_rdn4_slot;
        let mut var_dnj1_dv_rdn5: f64 = *var_dnj1_dv_rdn5_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_exp_vmax_over_phitd_bot: f64 = *var_exp_vmax_over_phitd_bot_slot;
        let mut var_exp_vmax_over_phitd_bot_db0: f64 = *var_exp_vmax_over_phitd_bot_db0_slot;
        let mut var_exp_vmax_over_phitd_bot_db1: f64 = *var_exp_vmax_over_phitd_bot_db1_slot;
        let mut var_exp_vmax_over_phitd_bot_db2: f64 = *var_exp_vmax_over_phitd_bot_db2_slot;
        let mut var_exp_vmax_over_phitd_bot_db3: f64 = *var_exp_vmax_over_phitd_bot_db3_slot;
        let mut var_exp_vmax_over_phitd_bot_dn0: f64 = *var_exp_vmax_over_phitd_bot_dn0_slot;
        let mut var_exp_vmax_over_phitd_bot_dn1: f64 = *var_exp_vmax_over_phitd_bot_dn1_slot;
        let mut var_exp_vmax_over_phitd_bot_dn2: f64 = *var_exp_vmax_over_phitd_bot_dn2_slot;
        let mut var_exp_vmax_over_phitd_bot_dn3: f64 = *var_exp_vmax_over_phitd_bot_dn3_slot;
        let mut var_exp_vmax_over_phitd_bot_dn4: f64 = *var_exp_vmax_over_phitd_bot_dn4_slot;
        let mut var_exp_vmax_over_phitd_bot_dn5: f64 = *var_exp_vmax_over_phitd_bot_dn5_slot;
        let mut var_exp_vmax_over_phitd_bot_rdb0: f64 = *var_exp_vmax_over_phitd_bot_rdb0_slot;
        let mut var_exp_vmax_over_phitd_bot_rdb1: f64 = *var_exp_vmax_over_phitd_bot_rdb1_slot;
        let mut var_exp_vmax_over_phitd_bot_rdb2: f64 = *var_exp_vmax_over_phitd_bot_rdb2_slot;
        let mut var_exp_vmax_over_phitd_bot_rdb3: f64 = *var_exp_vmax_over_phitd_bot_rdb3_slot;
        let mut var_exp_vmax_over_phitd_bot_rdn0: f64 = *var_exp_vmax_over_phitd_bot_rdn0_slot;
        let mut var_exp_vmax_over_phitd_bot_rdn1: f64 = *var_exp_vmax_over_phitd_bot_rdn1_slot;
        let mut var_exp_vmax_over_phitd_bot_rdn2: f64 = *var_exp_vmax_over_phitd_bot_rdn2_slot;
        let mut var_exp_vmax_over_phitd_bot_rdn3: f64 = *var_exp_vmax_over_phitd_bot_rdn3_slot;
        let mut var_exp_vmax_over_phitd_bot_rdn4: f64 = *var_exp_vmax_over_phitd_bot_rdn4_slot;
        let mut var_exp_vmax_over_phitd_bot_rdn5: f64 = *var_exp_vmax_over_phitd_bot_rdn5_slot;
        let mut var_exp_vmax_over_phitd_bot_rv: f64 = *var_exp_vmax_over_phitd_bot_rv_slot;
        let mut var_guard493: f64 = *var_guard493_slot;
        let mut var_guard493_db0: f64 = *var_guard493_db0_slot;
        let mut var_guard493_db1: f64 = *var_guard493_db1_slot;
        let mut var_guard493_db2: f64 = *var_guard493_db2_slot;
        let mut var_guard493_db3: f64 = *var_guard493_db3_slot;
        let mut var_guard493_dn0: f64 = *var_guard493_dn0_slot;
        let mut var_guard493_dn1: f64 = *var_guard493_dn1_slot;
        let mut var_guard493_dn2: f64 = *var_guard493_dn2_slot;
        let mut var_guard493_dn3: f64 = *var_guard493_dn3_slot;
        let mut var_guard493_dn4: f64 = *var_guard493_dn4_slot;
        let mut var_guard493_dn5: f64 = *var_guard493_dn5_slot;
        let mut var_guard493_rdb0: f64 = *var_guard493_rdb0_slot;
        let mut var_guard493_rdb1: f64 = *var_guard493_rdb1_slot;
        let mut var_guard493_rdb2: f64 = *var_guard493_rdb2_slot;
        let mut var_guard493_rdb3: f64 = *var_guard493_rdb3_slot;
        let mut var_guard493_rdn0: f64 = *var_guard493_rdn0_slot;
        let mut var_guard493_rdn1: f64 = *var_guard493_rdn1_slot;
        let mut var_guard493_rdn2: f64 = *var_guard493_rdn2_slot;
        let mut var_guard493_rdn3: f64 = *var_guard493_rdn3_slot;
        let mut var_guard493_rdn4: f64 = *var_guard493_rdn4_slot;
        let mut var_guard493_rdn5: f64 = *var_guard493_rdn5_slot;
        let mut var_guard493_rv: f64 = *var_guard493_rv_slot;
        let mut var_guard494: f64 = *var_guard494_slot;
        let mut var_guard494_db0: f64 = *var_guard494_db0_slot;
        let mut var_guard494_db1: f64 = *var_guard494_db1_slot;
        let mut var_guard494_db2: f64 = *var_guard494_db2_slot;
        let mut var_guard494_db3: f64 = *var_guard494_db3_slot;
        let mut var_guard494_dn0: f64 = *var_guard494_dn0_slot;
        let mut var_guard494_dn1: f64 = *var_guard494_dn1_slot;
        let mut var_guard494_dn2: f64 = *var_guard494_dn2_slot;
        let mut var_guard494_dn3: f64 = *var_guard494_dn3_slot;
        let mut var_guard494_dn4: f64 = *var_guard494_dn4_slot;
        let mut var_guard494_dn5: f64 = *var_guard494_dn5_slot;
        let mut var_guard494_rdb0: f64 = *var_guard494_rdb0_slot;
        let mut var_guard494_rdb1: f64 = *var_guard494_rdb1_slot;
        let mut var_guard494_rdb2: f64 = *var_guard494_rdb2_slot;
        let mut var_guard494_rdb3: f64 = *var_guard494_rdb3_slot;
        let mut var_guard494_rdn0: f64 = *var_guard494_rdn0_slot;
        let mut var_guard494_rdn1: f64 = *var_guard494_rdn1_slot;
        let mut var_guard494_rdn2: f64 = *var_guard494_rdn2_slot;
        let mut var_guard494_rdn3: f64 = *var_guard494_rdn3_slot;
        let mut var_guard494_rdn4: f64 = *var_guard494_rdn4_slot;
        let mut var_guard494_rdn5: f64 = *var_guard494_rdn5_slot;
        let mut var_guard494_rv: f64 = *var_guard494_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj0_rdb0: f64 = *var_nj0_rdb0_slot;
        let mut var_nj0_rdb1: f64 = *var_nj0_rdb1_slot;
        let mut var_nj0_rdb2: f64 = *var_nj0_rdb2_slot;
        let mut var_nj0_rdb3: f64 = *var_nj0_rdb3_slot;
        let mut var_nj0_rdn0: f64 = *var_nj0_rdn0_slot;
        let mut var_nj0_rdn1: f64 = *var_nj0_rdn1_slot;
        let mut var_nj0_rdn2: f64 = *var_nj0_rdn2_slot;
        let mut var_nj0_rdn3: f64 = *var_nj0_rdn3_slot;
        let mut var_nj0_rdn4: f64 = *var_nj0_rdn4_slot;
        let mut var_nj0_rdn5: f64 = *var_nj0_rdn5_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nj1_rdb0: f64 = *var_nj1_rdb0_slot;
        let mut var_nj1_rdb1: f64 = *var_nj1_rdb1_slot;
        let mut var_nj1_rdb2: f64 = *var_nj1_rdb2_slot;
        let mut var_nj1_rdb3: f64 = *var_nj1_rdb3_slot;
        let mut var_nj1_rdn0: f64 = *var_nj1_rdn0_slot;
        let mut var_nj1_rdn1: f64 = *var_nj1_rdn1_slot;
        let mut var_nj1_rdn2: f64 = *var_nj1_rdn2_slot;
        let mut var_nj1_rdn3: f64 = *var_nj1_rdn3_slot;
        let mut var_nj1_rdn4: f64 = *var_nj1_rdn4_slot;
        let mut var_nj1_rdn5: f64 = *var_nj1_rdn5_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_rdb0: f64 = *var_tmf1_rdb0_slot;
        let mut var_tmf1_rdb1: f64 = *var_tmf1_rdb1_slot;
        let mut var_tmf1_rdb2: f64 = *var_tmf1_rdb2_slot;
        let mut var_tmf1_rdb3: f64 = *var_tmf1_rdb3_slot;
        let mut var_tmf1_rdn0: f64 = *var_tmf1_rdn0_slot;
        let mut var_tmf1_rdn1: f64 = *var_tmf1_rdn1_slot;
        let mut var_tmf1_rdn2: f64 = *var_tmf1_rdn2_slot;
        let mut var_tmf1_rdn3: f64 = *var_tmf1_rdn3_slot;
        let mut var_tmf1_rdn4: f64 = *var_tmf1_rdn4_slot;
        let mut var_tmf1_rdn5: f64 = *var_tmf1_rdn5_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_rdb0: f64 = *var_tmf2_rdb0_slot;
        let mut var_tmf2_rdb1: f64 = *var_tmf2_rdb1_slot;
        let mut var_tmf2_rdb2: f64 = *var_tmf2_rdb2_slot;
        let mut var_tmf2_rdb3: f64 = *var_tmf2_rdb3_slot;
        let mut var_tmf2_rdn0: f64 = *var_tmf2_rdn0_slot;
        let mut var_tmf2_rdn1: f64 = *var_tmf2_rdn1_slot;
        let mut var_tmf2_rdn2: f64 = *var_tmf2_rdn2_slot;
        let mut var_tmf2_rdn3: f64 = *var_tmf2_rdn3_slot;
        let mut var_tmf2_rdn4: f64 = *var_tmf2_rdn4_slot;
        let mut var_tmf2_rdn5: f64 = *var_tmf2_rdn5_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign30520_e45503, assign30520_e45503_d_n0, assign30520_e45503_d_n1, assign30520_e45503_d_n2, assign30520_e45503_d_n3, assign30520_e45503_d_n4, assign30520_e45503_d_n5, assign30520_e45503_d_b0, assign30520_e45503_d_b1, assign30520_e45503_d_b2, assign30520_e45503_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let (assign30520_e45501, assign30520_e45501_d_n0, assign30520_e45501_d_n1, assign30520_e45501_d_n2, assign30520_e45501_d_n3, assign30520_e45501_d_n4, assign30520_e45501_d_n5, assign30520_e45501_d_b0, assign30520_e45501_d_b1, assign30520_e45501_d_b2, assign30520_e45501_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30520_e45500: f64 = (-var_tmf2);
                (assign30520_e45500, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30520_e45501, assign30520_e45501_d_n0, assign30520_e45501_d_n1, assign30520_e45501_d_n2, assign30520_e45501_d_n3, assign30520_e45501_d_n4, assign30520_e45501_d_n5, assign30520_e45501_d_b0, assign30520_e45501_d_b1, assign30520_e45501_d_b2, assign30520_e45501_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30520_e45503;
        var_tmf2_dn0 = assign30520_e45503_d_n0;
        var_tmf2_dn1 = assign30520_e45503_d_n1;
        var_tmf2_dn2 = assign30520_e45503_d_n2;
        var_tmf2_dn3 = assign30520_e45503_d_n3;
        var_tmf2_dn4 = assign30520_e45503_d_n4;
        var_tmf2_dn5 = assign30520_e45503_d_n5;
        var_tmf2_db0 = assign30520_e45503_d_b0;
        var_tmf2_db1 = assign30520_e45503_d_b1;
        var_tmf2_db2 = assign30520_e45503_d_b2;
        var_tmf2_db3 = assign30520_e45503_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30530_e45520, assign30530_e45520_d_n0, assign30530_e45520_d_n1, assign30530_e45520_d_n2, assign30530_e45520_d_n3, assign30530_e45520_d_n4, assign30530_e45520_d_n5, assign30530_e45520_d_b0, assign30530_e45520_d_b1, assign30530_e45520_d_b2, assign30530_e45520_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30530_e45515: f64 = (var_tmf1 * var_tmf1);
        let assign30530_e45517: f64 = (assign30530_e45515 + var_tmf2);
        let assign30530_e45518: f64 = (assign30530_e45517).sqrt();
        (assign30530_e45518, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30530_e45518)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30530_e45518)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30530_e45518)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30530_e45518)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30530_e45518)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30530_e45518)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30530_e45518)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30530_e45518)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30530_e45518)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30530_e45518)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30530_e45520;
        var_tmf2_dn0 = assign30530_e45520_d_n0;
        var_tmf2_dn1 = assign30530_e45520_d_n1;
        var_tmf2_dn2 = assign30530_e45520_d_n2;
        var_tmf2_dn3 = assign30530_e45520_d_n3;
        var_tmf2_dn4 = assign30530_e45520_d_n4;
        var_tmf2_dn5 = assign30530_e45520_d_n5;
        var_tmf2_db0 = assign30530_e45520_d_b0;
        var_tmf2_db1 = assign30530_e45520_d_b1;
        var_tmf2_db2 = assign30530_e45520_d_b2;
        var_tmf2_db3 = assign30530_e45520_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30540_e45538, assign30540_e45538_d_n0, assign30540_e45538_d_n1, assign30540_e45538_d_n2, assign30540_e45538_d_n3, assign30540_e45538_d_n4, assign30540_e45538_d_n5, assign30540_e45538_d_b0, assign30540_e45538_d_b1, assign30540_e45538_d_b2, assign30540_e45538_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30540_e45534: f64 = (var_tmf1 + var_tmf2);
        let assign30540_e45535: f64 = (0.5 * assign30540_e45534);
        let assign30540_e45536: f64 = (p.p85 - assign30540_e45535);
        (assign30540_e45536, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30540_e45538;
        var_nj0_dn0 = assign30540_e45538_d_n0;
        var_nj0_dn1 = assign30540_e45538_d_n1;
        var_nj0_dn2 = assign30540_e45538_d_n2;
        var_nj0_dn3 = assign30540_e45538_d_n3;
        var_nj0_dn4 = assign30540_e45538_d_n4;
        var_nj0_dn5 = assign30540_e45538_d_n5;
        var_nj0_db0 = assign30540_e45538_d_b0;
        var_nj0_db1 = assign30540_e45538_d_b1;
        var_nj0_db2 = assign30540_e45538_d_b2;
        var_nj0_db3 = assign30540_e45538_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign30550_e45554, assign30550_e45554_d_n0, assign30550_e45554_d_n1, assign30550_e45554_d_n2, assign30550_e45554_d_n3, assign30550_e45554_d_n4, assign30550_e45554_d_n5, assign30550_e45554_d_b0, assign30550_e45554_d_b1, assign30550_e45554_d_b2, assign30550_e45554_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30550_e45550: f64 = (var_nj0 - var_nfabot_i);
        let assign30550_e45552: f64 = (assign30550_e45550 - 0.01);
        (assign30550_e45552, (var_nj0_dn0 - var_nfabot_i_dn0), (var_nj0_dn1 - var_nfabot_i_dn1), (var_nj0_dn2 - var_nfabot_i_dn2), (var_nj0_dn3 - var_nfabot_i_dn3), (var_nj0_dn4 - var_nfabot_i_dn4), (var_nj0_dn5 - var_nfabot_i_dn5), (var_nj0_db0 - var_nfabot_i_db0), (var_nj0_db1 - var_nfabot_i_db1), (var_nj0_db2 - var_nfabot_i_db2), (var_nj0_db3 - var_nfabot_i_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30550_e45554;
        var_tmf1_dn0 = assign30550_e45554_d_n0;
        var_tmf1_dn1 = assign30550_e45554_d_n1;
        var_tmf1_dn2 = assign30550_e45554_d_n2;
        var_tmf1_dn3 = assign30550_e45554_d_n3;
        var_tmf1_dn4 = assign30550_e45554_d_n4;
        var_tmf1_dn5 = assign30550_e45554_d_n5;
        var_tmf1_db0 = assign30550_e45554_d_b0;
        var_tmf1_db1 = assign30550_e45554_d_b1;
        var_tmf1_db2 = assign30550_e45554_d_b2;
        var_tmf1_db3 = assign30550_e45554_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign30560_e45570, assign30560_e45570_d_n0, assign30560_e45570_d_n1, assign30560_e45570_d_n2, assign30560_e45570_d_n3, assign30560_e45570_d_n4, assign30560_e45570_d_n5, assign30560_e45570_d_b0, assign30560_e45570_d_b1, assign30560_e45570_d_b2, assign30560_e45570_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30560_e45566: f64 = (4.0 * var_nfabot_i);
        let assign30560_e45568: f64 = (assign30560_e45566 * 0.01);
        (assign30560_e45568, ((4.0 * var_nfabot_i_dn0) * 0.01), ((4.0 * var_nfabot_i_dn1) * 0.01), ((4.0 * var_nfabot_i_dn2) * 0.01), ((4.0 * var_nfabot_i_dn3) * 0.01), ((4.0 * var_nfabot_i_dn4) * 0.01), ((4.0 * var_nfabot_i_dn5) * 0.01), ((4.0 * var_nfabot_i_db0) * 0.01), ((4.0 * var_nfabot_i_db1) * 0.01), ((4.0 * var_nfabot_i_db2) * 0.01), ((4.0 * var_nfabot_i_db3) * 0.01),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30560_e45570;
        var_tmf2_dn0 = assign30560_e45570_d_n0;
        var_tmf2_dn1 = assign30560_e45570_d_n1;
        var_tmf2_dn2 = assign30560_e45570_d_n2;
        var_tmf2_dn3 = assign30560_e45570_d_n3;
        var_tmf2_dn4 = assign30560_e45570_d_n4;
        var_tmf2_dn5 = assign30560_e45570_d_n5;
        var_tmf2_db0 = assign30560_e45570_d_b0;
        var_tmf2_db1 = assign30560_e45570_d_b1;
        var_tmf2_db2 = assign30560_e45570_d_b2;
        var_tmf2_db3 = assign30560_e45570_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30570_e45588, assign30570_e45588_d_n0, assign30570_e45588_d_n1, assign30570_e45588_d_n2, assign30570_e45588_d_n3, assign30570_e45588_d_n4, assign30570_e45588_d_n5, assign30570_e45588_d_b0, assign30570_e45588_d_b1, assign30570_e45588_d_b2, assign30570_e45588_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let (assign30570_e45586, assign30570_e45586_d_n0, assign30570_e45586_d_n1, assign30570_e45586_d_n2, assign30570_e45586_d_n3, assign30570_e45586_d_n4, assign30570_e45586_d_n5, assign30570_e45586_d_b0, assign30570_e45586_d_b1, assign30570_e45586_d_b2, assign30570_e45586_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30570_e45585: f64 = (-var_tmf2);
                (assign30570_e45585, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30570_e45586, assign30570_e45586_d_n0, assign30570_e45586_d_n1, assign30570_e45586_d_n2, assign30570_e45586_d_n3, assign30570_e45586_d_n4, assign30570_e45586_d_n5, assign30570_e45586_d_b0, assign30570_e45586_d_b1, assign30570_e45586_d_b2, assign30570_e45586_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30570_e45588;
        var_tmf2_dn0 = assign30570_e45588_d_n0;
        var_tmf2_dn1 = assign30570_e45588_d_n1;
        var_tmf2_dn2 = assign30570_e45588_d_n2;
        var_tmf2_dn3 = assign30570_e45588_d_n3;
        var_tmf2_dn4 = assign30570_e45588_d_n4;
        var_tmf2_dn5 = assign30570_e45588_d_n5;
        var_tmf2_db0 = assign30570_e45588_d_b0;
        var_tmf2_db1 = assign30570_e45588_d_b1;
        var_tmf2_db2 = assign30570_e45588_d_b2;
        var_tmf2_db3 = assign30570_e45588_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30580_e45605, assign30580_e45605_d_n0, assign30580_e45605_d_n1, assign30580_e45605_d_n2, assign30580_e45605_d_n3, assign30580_e45605_d_n4, assign30580_e45605_d_n5, assign30580_e45605_d_b0, assign30580_e45605_d_b1, assign30580_e45605_d_b2, assign30580_e45605_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30580_e45600: f64 = (var_tmf1 * var_tmf1);
        let assign30580_e45602: f64 = (assign30580_e45600 + var_tmf2);
        let assign30580_e45603: f64 = (assign30580_e45602).sqrt();
        (assign30580_e45603, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30580_e45603)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30580_e45603)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30580_e45603)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30580_e45603)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30580_e45603)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30580_e45603)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30580_e45603)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30580_e45603)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30580_e45603)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30580_e45603)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30580_e45605;
        var_tmf2_dn0 = assign30580_e45605_d_n0;
        var_tmf2_dn1 = assign30580_e45605_d_n1;
        var_tmf2_dn2 = assign30580_e45605_d_n2;
        var_tmf2_dn3 = assign30580_e45605_d_n3;
        var_tmf2_dn4 = assign30580_e45605_d_n4;
        var_tmf2_dn5 = assign30580_e45605_d_n5;
        var_tmf2_db0 = assign30580_e45605_d_b0;
        var_tmf2_db1 = assign30580_e45605_d_b1;
        var_tmf2_db2 = assign30580_e45605_d_b2;
        var_tmf2_db3 = assign30580_e45605_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30590_e45623, assign30590_e45623_d_n0, assign30590_e45623_d_n1, assign30590_e45623_d_n2, assign30590_e45623_d_n3, assign30590_e45623_d_n4, assign30590_e45623_d_n5, assign30590_e45623_d_b0, assign30590_e45623_d_b1, assign30590_e45623_d_b2, assign30590_e45623_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30590_e45619: f64 = (var_tmf1 + var_tmf2);
        let assign30590_e45620: f64 = (0.5 * assign30590_e45619);
        let assign30590_e45621: f64 = (var_nfabot_i + assign30590_e45620);
        (assign30590_e45621, (var_nfabot_i_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_nfabot_i_dn1 + (0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (var_nfabot_i_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_nfabot_i_dn3 + (0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (var_nfabot_i_dn4 + (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_nfabot_i_dn5 + (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_nfabot_i_db0 + (0.5 * (var_tmf1_db0 + var_tmf2_db0))), (var_nfabot_i_db1 + (0.5 * (var_tmf1_db1 + var_tmf2_db1))), (var_nfabot_i_db2 + (0.5 * (var_tmf1_db2 + var_tmf2_db2))), (var_nfabot_i_db3 + (0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30590_e45623;
        var_nj0_dn0 = assign30590_e45623_d_n0;
        var_nj0_dn1 = assign30590_e45623_d_n1;
        var_nj0_dn2 = assign30590_e45623_d_n2;
        var_nj0_dn3 = assign30590_e45623_d_n3;
        var_nj0_dn4 = assign30590_e45623_d_n4;
        var_nj0_dn5 = assign30590_e45623_d_n5;
        var_nj0_db0 = assign30590_e45623_d_b0;
        var_nj0_db1 = assign30590_e45623_d_b1;
        var_nj0_db2 = assign30590_e45623_d_b2;
        var_nj0_db3 = assign30590_e45623_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign30600_e45639, assign30600_e45639_d_n0, assign30600_e45639_d_n1, assign30600_e45639_d_n2, assign30600_e45639_d_n3, assign30600_e45639_d_n4, assign30600_e45639_d_n5, assign30600_e45639_d_b0, assign30600_e45639_d_b1, assign30600_e45639_d_b2, assign30600_e45639_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30600_e45635: f64 = (p.p86 * var_dfn_su);
        let assign30600_e45637: f64 = (assign30600_e45635 * var_dfn_sl);
        (assign30600_e45637, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign30600_e45639;
        var_dnj1_dv_dn0 = assign30600_e45639_d_n0;
        var_dnj1_dv_dn1 = assign30600_e45639_d_n1;
        var_dnj1_dv_dn2 = assign30600_e45639_d_n2;
        var_dnj1_dv_dn3 = assign30600_e45639_d_n3;
        var_dnj1_dv_dn4 = assign30600_e45639_d_n4;
        var_dnj1_dv_dn5 = assign30600_e45639_d_n5;
        var_dnj1_dv_db0 = assign30600_e45639_d_b0;
        var_dnj1_dv_db1 = assign30600_e45639_d_b1;
        var_dnj1_dv_db2 = assign30600_e45639_d_b2;
        var_dnj1_dv_db3 = assign30600_e45639_d_b3;
        var_dnj1_dv_rv = 0.0;
        var_dnj1_dv_rdn0 = 0.0;
        var_dnj1_dv_rdn1 = 0.0;
        var_dnj1_dv_rdn2 = 0.0;
        var_dnj1_dv_rdn3 = 0.0;
        var_dnj1_dv_rdn4 = 0.0;
        var_dnj1_dv_rdn5 = 0.0;
        var_dnj1_dv_rdb0 = 0.0;
        var_dnj1_dv_rdb1 = 0.0;
        var_dnj1_dv_rdb2 = 0.0;
        var_dnj1_dv_rdb3 = 0.0;

        let (assign30610_e45652, assign30610_e45652_d_n0, assign30610_e45652_d_n1, assign30610_e45652_d_n2, assign30610_e45652_d_n3, assign30610_e45652_d_n4, assign30610_e45652_d_n5, assign30610_e45652_d_b0, assign30610_e45652_d_b1, assign30610_e45652_d_b2, assign30610_e45652_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 == 0.0)) {
        (var_nfabot_i, var_nfabot_i_dn0, var_nfabot_i_dn1, var_nfabot_i_dn2, var_nfabot_i_dn3, var_nfabot_i_dn4, var_nfabot_i_dn5, var_nfabot_i_db0, var_nfabot_i_db1, var_nfabot_i_db2, var_nfabot_i_db3,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30610_e45652;
        var_nj0_dn0 = assign30610_e45652_d_n0;
        var_nj0_dn1 = assign30610_e45652_d_n1;
        var_nj0_dn2 = assign30610_e45652_d_n2;
        var_nj0_dn3 = assign30610_e45652_d_n3;
        var_nj0_dn4 = assign30610_e45652_d_n4;
        var_nj0_dn5 = assign30610_e45652_d_n5;
        var_nj0_db0 = assign30610_e45652_d_b0;
        var_nj0_db1 = assign30610_e45652_d_b1;
        var_nj0_db2 = assign30610_e45652_d_b2;
        var_nj0_db3 = assign30610_e45652_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign30620_e45665, assign30620_e45665_d_n0, assign30620_e45665_d_n1, assign30620_e45665_d_n2, assign30620_e45665_d_n3, assign30620_e45665_d_n4, assign30620_e45665_d_n5, assign30620_e45665_d_b0, assign30620_e45665_d_b1, assign30620_e45665_d_b2, assign30620_e45665_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 == 0.0)) {
        (var_nfabot_i, var_nfabot_i_dn0, var_nfabot_i_dn1, var_nfabot_i_dn2, var_nfabot_i_dn3, var_nfabot_i_dn4, var_nfabot_i_dn5, var_nfabot_i_db0, var_nfabot_i_db1, var_nfabot_i_db2, var_nfabot_i_db3,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign30620_e45665;
        var_nj1_dn0 = assign30620_e45665_d_n0;
        var_nj1_dn1 = assign30620_e45665_d_n1;
        var_nj1_dn2 = assign30620_e45665_d_n2;
        var_nj1_dn3 = assign30620_e45665_d_n3;
        var_nj1_dn4 = assign30620_e45665_d_n4;
        var_nj1_dn5 = assign30620_e45665_d_n5;
        var_nj1_db0 = assign30620_e45665_d_b0;
        var_nj1_db1 = assign30620_e45665_d_b1;
        var_nj1_db2 = assign30620_e45665_d_b2;
        var_nj1_db3 = assign30620_e45665_d_b3;
        var_nj1_rv = 0.0;
        var_nj1_rdn0 = 0.0;
        var_nj1_rdn1 = 0.0;
        var_nj1_rdn2 = 0.0;
        var_nj1_rdn3 = 0.0;
        var_nj1_rdn4 = 0.0;
        var_nj1_rdn5 = 0.0;
        var_nj1_rdb0 = 0.0;
        var_nj1_rdb1 = 0.0;
        var_nj1_rdb2 = 0.0;
        var_nj1_rdb3 = 0.0;

        let (assign30630_e45678, assign30630_e45678_d_n0, assign30630_e45678_d_n1, assign30630_e45678_d_n2, assign30630_e45678_d_n3, assign30630_e45678_d_n4, assign30630_e45678_d_n5, assign30630_e45678_d_b0, assign30630_e45678_d_b1, assign30630_e45678_d_b2, assign30630_e45678_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign30630_e45678;
        var_dnj1_dv_dn0 = assign30630_e45678_d_n0;
        var_dnj1_dv_dn1 = assign30630_e45678_d_n1;
        var_dnj1_dv_dn2 = assign30630_e45678_d_n2;
        var_dnj1_dv_dn3 = assign30630_e45678_d_n3;
        var_dnj1_dv_dn4 = assign30630_e45678_d_n4;
        var_dnj1_dv_dn5 = assign30630_e45678_d_n5;
        var_dnj1_dv_db0 = assign30630_e45678_d_b0;
        var_dnj1_dv_db1 = assign30630_e45678_d_b1;
        var_dnj1_dv_db2 = assign30630_e45678_d_b2;
        var_dnj1_dv_db3 = assign30630_e45678_d_b3;
        var_dnj1_dv_rv = 0.0;
        var_dnj1_dv_rdn0 = 0.0;
        var_dnj1_dv_rdn1 = 0.0;
        var_dnj1_dv_rdn2 = 0.0;
        var_dnj1_dv_rdn3 = 0.0;
        var_dnj1_dv_rdn4 = 0.0;
        var_dnj1_dv_rdn5 = 0.0;
        var_dnj1_dv_rdb0 = 0.0;
        var_dnj1_dv_rdb1 = 0.0;
        var_dnj1_dv_rdb2 = 0.0;
        var_dnj1_dv_rdb3 = 0.0;

        let assign30640_e45682: f64 = (var_vmax / var_nj1);
        let assign30640_e45686: f64 = (var_nj1 - var_nj0);
        let assign30640_e45687: f64 = (var_vha1 * assign30640_e45686);
        let assign30640_e45690: f64 = (var_nj0 * p.p85);
        let assign30640_e45691: f64 = (assign30640_e45687 / assign30640_e45690);
        let assign30640_e45692: f64 = (assign30640_e45682 + assign30640_e45691);
        let assign30640_e45693: f64 = (var_phitdinv * assign30640_e45692);
        let assign30640_e45694: f64 = (assign30640_e45693).abs();
        let assign30640_e45696: f64 = if assign30640_e45694 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard493 = assign30640_e45696;
        var_guard493_dn0 = 0.0;
        var_guard493_dn1 = 0.0;
        var_guard493_dn2 = 0.0;
        var_guard493_dn3 = 0.0;
        var_guard493_dn4 = 0.0;
        var_guard493_dn5 = 0.0;
        var_guard493_db0 = 0.0;
        var_guard493_db1 = 0.0;
        var_guard493_db2 = 0.0;
        var_guard493_db3 = 0.0;
        var_guard493_rv = 0.0;
        var_guard493_rdn0 = 0.0;
        var_guard493_rdn1 = 0.0;
        var_guard493_rdn2 = 0.0;
        var_guard493_rdn3 = 0.0;
        var_guard493_rdn4 = 0.0;
        var_guard493_rdn5 = 0.0;
        var_guard493_rdb0 = 0.0;
        var_guard493_rdb1 = 0.0;
        var_guard493_rdb2 = 0.0;
        var_guard493_rdb3 = 0.0;

        let (assign30650_e45723, assign30650_e45723_d_n0, assign30650_e45723_d_n1, assign30650_e45723_d_n2, assign30650_e45723_d_n3, assign30650_e45723_d_n4, assign30650_e45723_d_n5, assign30650_e45723_d_b0, assign30650_e45723_d_b1, assign30650_e45723_d_b2, assign30650_e45723_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard493 != 0.0)) {
        let assign30650_e45709: f64 = (var_vmax / var_nj1);
        let assign30650_e45713: f64 = (var_nj1 - var_nj0);
        let assign30650_e45714: f64 = (var_vha1 * assign30650_e45713);
        let assign30650_e45717: f64 = (var_nj0 * p.p85);
        let assign30650_e45718: f64 = (assign30650_e45714 / assign30650_e45717);
        let assign30650_e45719: f64 = (assign30650_e45709 + assign30650_e45718);
        let assign30650_e45720: f64 = (var_phitdinv * assign30650_e45719);
        let assign30650_e45721: f64 = (assign30650_e45720).exp();
        (assign30650_e45721, (assign30650_e45721 * ((var_phitdinv_dn0 * assign30650_e45719) + (var_phitdinv * ((((var_vmax_dn0 * var_nj1) - (var_vmax * var_nj1_dn0)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn0 * assign30650_e45713) + (var_vha1 * (var_nj1_dn0 - var_nj0_dn0))) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn0 * p.p85))) / (assign30650_e45717 * assign30650_e45717)))))), (assign30650_e45721 * ((var_phitdinv_dn1 * assign30650_e45719) + (var_phitdinv * ((((var_vmax_dn1 * var_nj1) - (var_vmax * var_nj1_dn1)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn1 * assign30650_e45713) + (var_vha1 * (var_nj1_dn1 - var_nj0_dn1))) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn1 * p.p85))) / (assign30650_e45717 * assign30650_e45717)))))), (assign30650_e45721 * ((var_phitdinv_dn2 * assign30650_e45719) + (var_phitdinv * ((((var_vmax_dn2 * var_nj1) - (var_vmax * var_nj1_dn2)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn2 * assign30650_e45713) + (var_vha1 * (var_nj1_dn2 - var_nj0_dn2))) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn2 * p.p85))) / (assign30650_e45717 * assign30650_e45717)))))), (assign30650_e45721 * ((var_phitdinv_dn3 * assign30650_e45719) + (var_phitdinv * ((((var_vmax_dn3 * var_nj1) - (var_vmax * var_nj1_dn3)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn3 * assign30650_e45713) + (var_vha1 * (var_nj1_dn3 - var_nj0_dn3))) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn3 * p.p85))) / (assign30650_e45717 * assign30650_e45717)))))), (assign30650_e45721 * ((var_phitdinv_dn4 * assign30650_e45719) + (var_phitdinv * ((((var_vmax_dn4 * var_nj1) - (var_vmax * var_nj1_dn4)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn4 * assign30650_e45713) + (var_vha1 * (var_nj1_dn4 - var_nj0_dn4))) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn4 * p.p85))) / (assign30650_e45717 * assign30650_e45717)))))), (assign30650_e45721 * ((var_phitdinv_dn5 * assign30650_e45719) + (var_phitdinv * ((((var_vmax_dn5 * var_nj1) - (var_vmax * var_nj1_dn5)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn5 * assign30650_e45713) + (var_vha1 * (var_nj1_dn5 - var_nj0_dn5))) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn5 * p.p85))) / (assign30650_e45717 * assign30650_e45717)))))), (assign30650_e45721 * ((var_phitdinv_db0 * assign30650_e45719) + (var_phitdinv * ((((var_vmax_db0 * var_nj1) - (var_vmax * var_nj1_db0)) / (var_nj1 * var_nj1)) + (((((var_vha1_db0 * assign30650_e45713) + (var_vha1 * (var_nj1_db0 - var_nj0_db0))) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_db0 * p.p85))) / (assign30650_e45717 * assign30650_e45717)))))), (assign30650_e45721 * ((var_phitdinv_db1 * assign30650_e45719) + (var_phitdinv * ((((var_vmax_db1 * var_nj1) - (var_vmax * var_nj1_db1)) / (var_nj1 * var_nj1)) + (((((var_vha1_db1 * assign30650_e45713) + (var_vha1 * (var_nj1_db1 - var_nj0_db1))) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_db1 * p.p85))) / (assign30650_e45717 * assign30650_e45717)))))), (assign30650_e45721 * ((var_phitdinv_db2 * assign30650_e45719) + (var_phitdinv * ((((var_vmax_db2 * var_nj1) - (var_vmax * var_nj1_db2)) / (var_nj1 * var_nj1)) + (((((var_vha1_db2 * assign30650_e45713) + (var_vha1 * (var_nj1_db2 - var_nj0_db2))) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_db2 * p.p85))) / (assign30650_e45717 * assign30650_e45717)))))), (assign30650_e45721 * ((var_phitdinv_db3 * assign30650_e45719) + (var_phitdinv * ((((var_vmax_db3 * var_nj1) - (var_vmax * var_nj1_db3)) / (var_nj1 * var_nj1)) + (((((var_vha1_db3 * assign30650_e45713) + (var_vha1 * (var_nj1_db3 - var_nj0_db3))) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_db3 * p.p85))) / (assign30650_e45717 * assign30650_e45717)))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign30650_e45723;
        var_exp_vmax_over_phitd_bot_dn0 = assign30650_e45723_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign30650_e45723_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign30650_e45723_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign30650_e45723_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign30650_e45723_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign30650_e45723_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign30650_e45723_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign30650_e45723_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign30650_e45723_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign30650_e45723_d_b3;
        var_exp_vmax_over_phitd_bot_rv = 0.0;
        var_exp_vmax_over_phitd_bot_rdn0 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn1 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn2 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn3 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn4 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn5 = 0.0;
        var_exp_vmax_over_phitd_bot_rdb0 = 0.0;
        var_exp_vmax_over_phitd_bot_rdb1 = 0.0;
        var_exp_vmax_over_phitd_bot_rdb2 = 0.0;
        var_exp_vmax_over_phitd_bot_rdb3 = 0.0;

        let assign30660_e45727: f64 = (var_vmax / var_nj1);
        let assign30660_e45731: f64 = (var_nj1 - var_nj0);
        let assign30660_e45732: f64 = (var_vha1 * assign30660_e45731);
        let assign30660_e45735: f64 = (var_nj0 * p.p85);
        let assign30660_e45736: f64 = (assign30660_e45732 / assign30660_e45735);
        let assign30660_e45737: f64 = (assign30660_e45727 + assign30660_e45736);
        let assign30660_e45738: f64 = (var_phitdinv * assign30660_e45737);
        let assign30660_e45740: f64 = (-230.25850929940458);
        let assign30660_e45741: f64 = if assign30660_e45738 < assign30660_e45740 { 1.0 } else { 0.0 };
        var_guard494 = assign30660_e45741;
        var_guard494_dn0 = 0.0;
        var_guard494_dn1 = 0.0;
        var_guard494_dn2 = 0.0;
        var_guard494_dn3 = 0.0;
        var_guard494_dn4 = 0.0;
        var_guard494_dn5 = 0.0;
        var_guard494_db0 = 0.0;
        var_guard494_db1 = 0.0;
        var_guard494_db2 = 0.0;
        var_guard494_db3 = 0.0;
        var_guard494_rv = 0.0;
        var_guard494_rdn0 = 0.0;
        var_guard494_rdn1 = 0.0;
        var_guard494_rdn2 = 0.0;
        var_guard494_rdn3 = 0.0;
        var_guard494_rdn4 = 0.0;
        var_guard494_rdn5 = 0.0;
        var_guard494_rdb0 = 0.0;
        var_guard494_rdb1 = 0.0;
        var_guard494_rdb2 = 0.0;
        var_guard494_rdb3 = 0.0;

        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_db0_slot = var_dnj1_dv_db0;
        *var_dnj1_dv_db1_slot = var_dnj1_dv_db1;
        *var_dnj1_dv_db2_slot = var_dnj1_dv_db2;
        *var_dnj1_dv_db3_slot = var_dnj1_dv_db3;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn1_slot = var_dnj1_dv_dn1;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_dn3_slot = var_dnj1_dv_dn3;
        *var_dnj1_dv_dn4_slot = var_dnj1_dv_dn4;
        *var_dnj1_dv_dn5_slot = var_dnj1_dv_dn5;
        *var_dnj1_dv_rdb0_slot = var_dnj1_dv_rdb0;
        *var_dnj1_dv_rdb1_slot = var_dnj1_dv_rdb1;
        *var_dnj1_dv_rdb2_slot = var_dnj1_dv_rdb2;
        *var_dnj1_dv_rdb3_slot = var_dnj1_dv_rdb3;
        *var_dnj1_dv_rdn0_slot = var_dnj1_dv_rdn0;
        *var_dnj1_dv_rdn1_slot = var_dnj1_dv_rdn1;
        *var_dnj1_dv_rdn2_slot = var_dnj1_dv_rdn2;
        *var_dnj1_dv_rdn3_slot = var_dnj1_dv_rdn3;
        *var_dnj1_dv_rdn4_slot = var_dnj1_dv_rdn4;
        *var_dnj1_dv_rdn5_slot = var_dnj1_dv_rdn5;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_exp_vmax_over_phitd_bot_slot = var_exp_vmax_over_phitd_bot;
        *var_exp_vmax_over_phitd_bot_db0_slot = var_exp_vmax_over_phitd_bot_db0;
        *var_exp_vmax_over_phitd_bot_db1_slot = var_exp_vmax_over_phitd_bot_db1;
        *var_exp_vmax_over_phitd_bot_db2_slot = var_exp_vmax_over_phitd_bot_db2;
        *var_exp_vmax_over_phitd_bot_db3_slot = var_exp_vmax_over_phitd_bot_db3;
        *var_exp_vmax_over_phitd_bot_dn0_slot = var_exp_vmax_over_phitd_bot_dn0;
        *var_exp_vmax_over_phitd_bot_dn1_slot = var_exp_vmax_over_phitd_bot_dn1;
        *var_exp_vmax_over_phitd_bot_dn2_slot = var_exp_vmax_over_phitd_bot_dn2;
        *var_exp_vmax_over_phitd_bot_dn3_slot = var_exp_vmax_over_phitd_bot_dn3;
        *var_exp_vmax_over_phitd_bot_dn4_slot = var_exp_vmax_over_phitd_bot_dn4;
        *var_exp_vmax_over_phitd_bot_dn5_slot = var_exp_vmax_over_phitd_bot_dn5;
        *var_exp_vmax_over_phitd_bot_rdb0_slot = var_exp_vmax_over_phitd_bot_rdb0;
        *var_exp_vmax_over_phitd_bot_rdb1_slot = var_exp_vmax_over_phitd_bot_rdb1;
        *var_exp_vmax_over_phitd_bot_rdb2_slot = var_exp_vmax_over_phitd_bot_rdb2;
        *var_exp_vmax_over_phitd_bot_rdb3_slot = var_exp_vmax_over_phitd_bot_rdb3;
        *var_exp_vmax_over_phitd_bot_rdn0_slot = var_exp_vmax_over_phitd_bot_rdn0;
        *var_exp_vmax_over_phitd_bot_rdn1_slot = var_exp_vmax_over_phitd_bot_rdn1;
        *var_exp_vmax_over_phitd_bot_rdn2_slot = var_exp_vmax_over_phitd_bot_rdn2;
        *var_exp_vmax_over_phitd_bot_rdn3_slot = var_exp_vmax_over_phitd_bot_rdn3;
        *var_exp_vmax_over_phitd_bot_rdn4_slot = var_exp_vmax_over_phitd_bot_rdn4;
        *var_exp_vmax_over_phitd_bot_rdn5_slot = var_exp_vmax_over_phitd_bot_rdn5;
        *var_exp_vmax_over_phitd_bot_rv_slot = var_exp_vmax_over_phitd_bot_rv;
        *var_guard493_slot = var_guard493;
        *var_guard493_db0_slot = var_guard493_db0;
        *var_guard493_db1_slot = var_guard493_db1;
        *var_guard493_db2_slot = var_guard493_db2;
        *var_guard493_db3_slot = var_guard493_db3;
        *var_guard493_dn0_slot = var_guard493_dn0;
        *var_guard493_dn1_slot = var_guard493_dn1;
        *var_guard493_dn2_slot = var_guard493_dn2;
        *var_guard493_dn3_slot = var_guard493_dn3;
        *var_guard493_dn4_slot = var_guard493_dn4;
        *var_guard493_dn5_slot = var_guard493_dn5;
        *var_guard493_rdb0_slot = var_guard493_rdb0;
        *var_guard493_rdb1_slot = var_guard493_rdb1;
        *var_guard493_rdb2_slot = var_guard493_rdb2;
        *var_guard493_rdb3_slot = var_guard493_rdb3;
        *var_guard493_rdn0_slot = var_guard493_rdn0;
        *var_guard493_rdn1_slot = var_guard493_rdn1;
        *var_guard493_rdn2_slot = var_guard493_rdn2;
        *var_guard493_rdn3_slot = var_guard493_rdn3;
        *var_guard493_rdn4_slot = var_guard493_rdn4;
        *var_guard493_rdn5_slot = var_guard493_rdn5;
        *var_guard493_rv_slot = var_guard493_rv;
        *var_guard494_slot = var_guard494;
        *var_guard494_db0_slot = var_guard494_db0;
        *var_guard494_db1_slot = var_guard494_db1;
        *var_guard494_db2_slot = var_guard494_db2;
        *var_guard494_db3_slot = var_guard494_db3;
        *var_guard494_dn0_slot = var_guard494_dn0;
        *var_guard494_dn1_slot = var_guard494_dn1;
        *var_guard494_dn2_slot = var_guard494_dn2;
        *var_guard494_dn3_slot = var_guard494_dn3;
        *var_guard494_dn4_slot = var_guard494_dn4;
        *var_guard494_dn5_slot = var_guard494_dn5;
        *var_guard494_rdb0_slot = var_guard494_rdb0;
        *var_guard494_rdb1_slot = var_guard494_rdb1;
        *var_guard494_rdb2_slot = var_guard494_rdb2;
        *var_guard494_rdb3_slot = var_guard494_rdb3;
        *var_guard494_rdn0_slot = var_guard494_rdn0;
        *var_guard494_rdn1_slot = var_guard494_rdn1;
        *var_guard494_rdn2_slot = var_guard494_rdn2;
        *var_guard494_rdn3_slot = var_guard494_rdn3;
        *var_guard494_rdn4_slot = var_guard494_rdn4;
        *var_guard494_rdn5_slot = var_guard494_rdn5;
        *var_guard494_rv_slot = var_guard494_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj0_rdb0_slot = var_nj0_rdb0;
        *var_nj0_rdb1_slot = var_nj0_rdb1;
        *var_nj0_rdb2_slot = var_nj0_rdb2;
        *var_nj0_rdb3_slot = var_nj0_rdb3;
        *var_nj0_rdn0_slot = var_nj0_rdn0;
        *var_nj0_rdn1_slot = var_nj0_rdn1;
        *var_nj0_rdn2_slot = var_nj0_rdn2;
        *var_nj0_rdn3_slot = var_nj0_rdn3;
        *var_nj0_rdn4_slot = var_nj0_rdn4;
        *var_nj0_rdn5_slot = var_nj0_rdn5;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nj1_rdb0_slot = var_nj1_rdb0;
        *var_nj1_rdb1_slot = var_nj1_rdb1;
        *var_nj1_rdb2_slot = var_nj1_rdb2;
        *var_nj1_rdb3_slot = var_nj1_rdb3;
        *var_nj1_rdn0_slot = var_nj1_rdn0;
        *var_nj1_rdn1_slot = var_nj1_rdn1;
        *var_nj1_rdn2_slot = var_nj1_rdn2;
        *var_nj1_rdn3_slot = var_nj1_rdn3;
        *var_nj1_rdn4_slot = var_nj1_rdn4;
        *var_nj1_rdn5_slot = var_nj1_rdn5;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_rdb0_slot = var_tmf1_rdb0;
        *var_tmf1_rdb1_slot = var_tmf1_rdb1;
        *var_tmf1_rdb2_slot = var_tmf1_rdb2;
        *var_tmf1_rdb3_slot = var_tmf1_rdb3;
        *var_tmf1_rdn0_slot = var_tmf1_rdn0;
        *var_tmf1_rdn1_slot = var_tmf1_rdn1;
        *var_tmf1_rdn2_slot = var_tmf1_rdn2;
        *var_tmf1_rdn3_slot = var_tmf1_rdn3;
        *var_tmf1_rdn4_slot = var_tmf1_rdn4;
        *var_tmf1_rdn5_slot = var_tmf1_rdn5;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_rdb0_slot = var_tmf2_rdb0;
        *var_tmf2_rdb1_slot = var_tmf2_rdb1;
        *var_tmf2_rdb2_slot = var_tmf2_rdb2;
        *var_tmf2_rdb3_slot = var_tmf2_rdb3;
        *var_tmf2_rdn0_slot = var_tmf2_rdn0;
        *var_tmf2_rdn1_slot = var_tmf2_rdn1;
        *var_tmf2_rdn2_slot = var_tmf2_rdn2;
        *var_tmf2_rdn3_slot = var_tmf2_rdn3;
        *var_tmf2_rdn4_slot = var_tmf2_rdn4;
        *var_tmf2_rdn5_slot = var_tmf2_rdn5;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_79(
        p: &Parameters,
        var_dnj1_dv: f64,
        var_dnj1_dv_db0: f64,
        var_dnj1_dv_db1: f64,
        var_dnj1_dv_db2: f64,
        var_dnj1_dv_db3: f64,
        var_dnj1_dv_dn0: f64,
        var_dnj1_dv_dn1: f64,
        var_dnj1_dv_dn2: f64,
        var_dnj1_dv_dn3: f64,
        var_dnj1_dv_dn4: f64,
        var_dnj1_dv_dn5: f64,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard493: f64,
        var_guard494: f64,
        var_ndisti_i: f64,
        var_ndisti_i_db0: f64,
        var_ndisti_i_db1: f64,
        var_ndisti_i_db2: f64,
        var_ndisti_i_db3: f64,
        var_ndisti_i_dn0: f64,
        var_ndisti_i_dn1: f64,
        var_ndisti_i_dn2: f64,
        var_ndisti_i_dn3: f64,
        var_ndisti_i_dn4: f64,
        var_ndisti_i_dn5: f64,
        var_nfasti_i: f64,
        var_nfasti_i_db0: f64,
        var_nfasti_i_db1: f64,
        var_nfasti_i_db2: f64,
        var_nfasti_i_db3: f64,
        var_nfasti_i_dn0: f64,
        var_nfasti_i_dn1: f64,
        var_nfasti_i_dn2: f64,
        var_nfasti_i_dn3: f64,
        var_nfasti_i_dn4: f64,
        var_nfasti_i_dn5: f64,
        var_nin: f64,
        var_nin_db0: f64,
        var_nin_db1: f64,
        var_nin_db2: f64,
        var_nin_db3: f64,
        var_nin_dn0: f64,
        var_nin_dn1: f64,
        var_nin_dn2: f64,
        var_nin_dn3: f64,
        var_nin_dn4: f64,
        var_nin_dn5: f64,
        var_nj1: f64,
        var_nj1_db0: f64,
        var_nj1_db1: f64,
        var_nj1_db2: f64,
        var_nj1_db3: f64,
        var_nj1_dn0: f64,
        var_nj1_dn1: f64,
        var_nj1_dn2: f64,
        var_nj1_dn3: f64,
        var_nj1_dn4: f64,
        var_nj1_dn5: f64,
        var_phitdinv: f64,
        var_phitdinv_db0: f64,
        var_phitdinv_db1: f64,
        var_phitdinv_db2: f64,
        var_phitdinv_db3: f64,
        var_phitdinv_dn0: f64,
        var_phitdinv_dn1: f64,
        var_phitdinv_dn2: f64,
        var_phitdinv_dn3: f64,
        var_phitdinv_dn4: f64,
        var_phitdinv_dn5: f64,
        var_vak: f64,
        var_vak_db0: f64,
        var_vak_db1: f64,
        var_vak_db2: f64,
        var_vak_db3: f64,
        var_vak_dn0: f64,
        var_vak_dn1: f64,
        var_vak_dn2: f64,
        var_vak_dn3: f64,
        var_vak_dn4: f64,
        var_vak_dn5: f64,
        var_vmax: f64,
        var_vmax_db0: f64,
        var_vmax_db1: f64,
        var_vmax_db2: f64,
        var_vmax_db3: f64,
        var_vmax_dn0: f64,
        var_vmax_dn1: f64,
        var_vmax_dn2: f64,
        var_vmax_dn3: f64,
        var_vmax_dn4: f64,
        var_vmax_dn5: f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_db0_slot: &mut f64,
        var_dvmax_over_phitd_dv_db1_slot: &mut f64,
        var_dvmax_over_phitd_dv_db2_slot: &mut f64,
        var_dvmax_over_phitd_dv_db3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn1_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn4_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn5_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdb0_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdb1_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdb2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdb3_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn1_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn3_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn4_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn5_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn4_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn5_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdb0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdb1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdb2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdb3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdn1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdn3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdn4_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rdn5_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rv_slot: &mut f64,
        var_guard495_slot: &mut f64,
        var_guard495_db0_slot: &mut f64,
        var_guard495_db1_slot: &mut f64,
        var_guard495_db2_slot: &mut f64,
        var_guard495_db3_slot: &mut f64,
        var_guard495_dn0_slot: &mut f64,
        var_guard495_dn1_slot: &mut f64,
        var_guard495_dn2_slot: &mut f64,
        var_guard495_dn3_slot: &mut f64,
        var_guard495_dn4_slot: &mut f64,
        var_guard495_dn5_slot: &mut f64,
        var_guard495_rdb0_slot: &mut f64,
        var_guard495_rdb1_slot: &mut f64,
        var_guard495_rdb2_slot: &mut f64,
        var_guard495_rdb3_slot: &mut f64,
        var_guard495_rdn0_slot: &mut f64,
        var_guard495_rdn1_slot: &mut f64,
        var_guard495_rdn2_slot: &mut f64,
        var_guard495_rdn3_slot: &mut f64,
        var_guard495_rdn4_slot: &mut f64,
        var_guard495_rdn5_slot: &mut f64,
        var_guard495_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_db0_slot: &mut f64,
        var_idmultbot_db1_slot: &mut f64,
        var_idmultbot_db2_slot: &mut f64,
        var_idmultbot_db3_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn1_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_dn3_slot: &mut f64,
        var_idmultbot_dn4_slot: &mut f64,
        var_idmultbot_dn5_slot: &mut f64,
        var_idmultbot_rdb0_slot: &mut f64,
        var_idmultbot_rdb1_slot: &mut f64,
        var_idmultbot_rdb2_slot: &mut f64,
        var_idmultbot_rdb3_slot: &mut f64,
        var_idmultbot_rdn0_slot: &mut f64,
        var_idmultbot_rdn1_slot: &mut f64,
        var_idmultbot_rdn2_slot: &mut f64,
        var_idmultbot_rdn3_slot: &mut f64,
        var_idmultbot_rdn4_slot: &mut f64,
        var_idmultbot_rdn5_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj0_rdb0_slot: &mut f64,
        var_nj0_rdb1_slot: &mut f64,
        var_nj0_rdb2_slot: &mut f64,
        var_nj0_rdb3_slot: &mut f64,
        var_nj0_rdn0_slot: &mut f64,
        var_nj0_rdn1_slot: &mut f64,
        var_nj0_rdn2_slot: &mut f64,
        var_nj0_rdn3_slot: &mut f64,
        var_nj0_rdn4_slot: &mut f64,
        var_nj0_rdn5_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_nja10_rdb0_slot: &mut f64,
        var_nja10_rdb1_slot: &mut f64,
        var_nja10_rdb2_slot: &mut f64,
        var_nja10_rdb3_slot: &mut f64,
        var_nja10_rdn0_slot: &mut f64,
        var_nja10_rdn1_slot: &mut f64,
        var_nja10_rdn2_slot: &mut f64,
        var_nja10_rdn3_slot: &mut f64,
        var_nja10_rdn4_slot: &mut f64,
        var_nja10_rdn5_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_db0_slot: &mut f64,
        var_pnn0_db1_slot: &mut f64,
        var_pnn0_db2_slot: &mut f64,
        var_pnn0_db3_slot: &mut f64,
        var_pnn0_dn0_slot: &mut f64,
        var_pnn0_dn1_slot: &mut f64,
        var_pnn0_dn2_slot: &mut f64,
        var_pnn0_dn3_slot: &mut f64,
        var_pnn0_dn4_slot: &mut f64,
        var_pnn0_dn5_slot: &mut f64,
        var_pnn0_rdb0_slot: &mut f64,
        var_pnn0_rdb1_slot: &mut f64,
        var_pnn0_rdb2_slot: &mut f64,
        var_pnn0_rdb3_slot: &mut f64,
        var_pnn0_rdn0_slot: &mut f64,
        var_pnn0_rdn1_slot: &mut f64,
        var_pnn0_rdn2_slot: &mut f64,
        var_pnn0_rdn3_slot: &mut f64,
        var_pnn0_rdn4_slot: &mut f64,
        var_pnn0_rdn5_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_rdb0_slot: &mut f64,
        var_tmf1_rdb1_slot: &mut f64,
        var_tmf1_rdb2_slot: &mut f64,
        var_tmf1_rdb3_slot: &mut f64,
        var_tmf1_rdn0_slot: &mut f64,
        var_tmf1_rdn1_slot: &mut f64,
        var_tmf1_rdn2_slot: &mut f64,
        var_tmf1_rdn3_slot: &mut f64,
        var_tmf1_rdn4_slot: &mut f64,
        var_tmf1_rdn5_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_rdb0_slot: &mut f64,
        var_tmf2_rdb1_slot: &mut f64,
        var_tmf2_rdb2_slot: &mut f64,
        var_tmf2_rdb3_slot: &mut f64,
        var_tmf2_rdn0_slot: &mut f64,
        var_tmf2_rdn1_slot: &mut f64,
        var_tmf2_rdn2_slot: &mut f64,
        var_tmf2_rdn3_slot: &mut f64,
        var_tmf2_rdn4_slot: &mut f64,
        var_tmf2_rdn5_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_db0_slot: &mut f64,
        var_vha1_db1_slot: &mut f64,
        var_vha1_db2_slot: &mut f64,
        var_vha1_db3_slot: &mut f64,
        var_vha1_dn0_slot: &mut f64,
        var_vha1_dn1_slot: &mut f64,
        var_vha1_dn2_slot: &mut f64,
        var_vha1_dn3_slot: &mut f64,
        var_vha1_dn4_slot: &mut f64,
        var_vha1_dn5_slot: &mut f64,
        var_vha1_rdb0_slot: &mut f64,
        var_vha1_rdb1_slot: &mut f64,
        var_vha1_rdb2_slot: &mut f64,
        var_vha1_rdb3_slot: &mut f64,
        var_vha1_rdn0_slot: &mut f64,
        var_vha1_rdn1_slot: &mut f64,
        var_vha1_rdn2_slot: &mut f64,
        var_vha1_rdn3_slot: &mut f64,
        var_vha1_rdn4_slot: &mut f64,
        var_vha1_rdn5_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_db0: f64 = *var_dvmax_over_phitd_dv_db0_slot;
        let mut var_dvmax_over_phitd_dv_db1: f64 = *var_dvmax_over_phitd_dv_db1_slot;
        let mut var_dvmax_over_phitd_dv_db2: f64 = *var_dvmax_over_phitd_dv_db2_slot;
        let mut var_dvmax_over_phitd_dv_db3: f64 = *var_dvmax_over_phitd_dv_db3_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn1: f64 = *var_dvmax_over_phitd_dv_dn1_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_dn3: f64 = *var_dvmax_over_phitd_dv_dn3_slot;
        let mut var_dvmax_over_phitd_dv_dn4: f64 = *var_dvmax_over_phitd_dv_dn4_slot;
        let mut var_dvmax_over_phitd_dv_dn5: f64 = *var_dvmax_over_phitd_dv_dn5_slot;
        let mut var_dvmax_over_phitd_dv_rdb0: f64 = *var_dvmax_over_phitd_dv_rdb0_slot;
        let mut var_dvmax_over_phitd_dv_rdb1: f64 = *var_dvmax_over_phitd_dv_rdb1_slot;
        let mut var_dvmax_over_phitd_dv_rdb2: f64 = *var_dvmax_over_phitd_dv_rdb2_slot;
        let mut var_dvmax_over_phitd_dv_rdb3: f64 = *var_dvmax_over_phitd_dv_rdb3_slot;
        let mut var_dvmax_over_phitd_dv_rdn0: f64 = *var_dvmax_over_phitd_dv_rdn0_slot;
        let mut var_dvmax_over_phitd_dv_rdn1: f64 = *var_dvmax_over_phitd_dv_rdn1_slot;
        let mut var_dvmax_over_phitd_dv_rdn2: f64 = *var_dvmax_over_phitd_dv_rdn2_slot;
        let mut var_dvmax_over_phitd_dv_rdn3: f64 = *var_dvmax_over_phitd_dv_rdn3_slot;
        let mut var_dvmax_over_phitd_dv_rdn4: f64 = *var_dvmax_over_phitd_dv_rdn4_slot;
        let mut var_dvmax_over_phitd_dv_rdn5: f64 = *var_dvmax_over_phitd_dv_rdn5_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_exp_vmax_over_phitd_bot: f64 = *var_exp_vmax_over_phitd_bot_slot;
        let mut var_exp_vmax_over_phitd_bot_db0: f64 = *var_exp_vmax_over_phitd_bot_db0_slot;
        let mut var_exp_vmax_over_phitd_bot_db1: f64 = *var_exp_vmax_over_phitd_bot_db1_slot;
        let mut var_exp_vmax_over_phitd_bot_db2: f64 = *var_exp_vmax_over_phitd_bot_db2_slot;
        let mut var_exp_vmax_over_phitd_bot_db3: f64 = *var_exp_vmax_over_phitd_bot_db3_slot;
        let mut var_exp_vmax_over_phitd_bot_dn0: f64 = *var_exp_vmax_over_phitd_bot_dn0_slot;
        let mut var_exp_vmax_over_phitd_bot_dn1: f64 = *var_exp_vmax_over_phitd_bot_dn1_slot;
        let mut var_exp_vmax_over_phitd_bot_dn2: f64 = *var_exp_vmax_over_phitd_bot_dn2_slot;
        let mut var_exp_vmax_over_phitd_bot_dn3: f64 = *var_exp_vmax_over_phitd_bot_dn3_slot;
        let mut var_exp_vmax_over_phitd_bot_dn4: f64 = *var_exp_vmax_over_phitd_bot_dn4_slot;
        let mut var_exp_vmax_over_phitd_bot_dn5: f64 = *var_exp_vmax_over_phitd_bot_dn5_slot;
        let mut var_exp_vmax_over_phitd_bot_rdb0: f64 = *var_exp_vmax_over_phitd_bot_rdb0_slot;
        let mut var_exp_vmax_over_phitd_bot_rdb1: f64 = *var_exp_vmax_over_phitd_bot_rdb1_slot;
        let mut var_exp_vmax_over_phitd_bot_rdb2: f64 = *var_exp_vmax_over_phitd_bot_rdb2_slot;
        let mut var_exp_vmax_over_phitd_bot_rdb3: f64 = *var_exp_vmax_over_phitd_bot_rdb3_slot;
        let mut var_exp_vmax_over_phitd_bot_rdn0: f64 = *var_exp_vmax_over_phitd_bot_rdn0_slot;
        let mut var_exp_vmax_over_phitd_bot_rdn1: f64 = *var_exp_vmax_over_phitd_bot_rdn1_slot;
        let mut var_exp_vmax_over_phitd_bot_rdn2: f64 = *var_exp_vmax_over_phitd_bot_rdn2_slot;
        let mut var_exp_vmax_over_phitd_bot_rdn3: f64 = *var_exp_vmax_over_phitd_bot_rdn3_slot;
        let mut var_exp_vmax_over_phitd_bot_rdn4: f64 = *var_exp_vmax_over_phitd_bot_rdn4_slot;
        let mut var_exp_vmax_over_phitd_bot_rdn5: f64 = *var_exp_vmax_over_phitd_bot_rdn5_slot;
        let mut var_exp_vmax_over_phitd_bot_rv: f64 = *var_exp_vmax_over_phitd_bot_rv_slot;
        let mut var_guard495: f64 = *var_guard495_slot;
        let mut var_guard495_db0: f64 = *var_guard495_db0_slot;
        let mut var_guard495_db1: f64 = *var_guard495_db1_slot;
        let mut var_guard495_db2: f64 = *var_guard495_db2_slot;
        let mut var_guard495_db3: f64 = *var_guard495_db3_slot;
        let mut var_guard495_dn0: f64 = *var_guard495_dn0_slot;
        let mut var_guard495_dn1: f64 = *var_guard495_dn1_slot;
        let mut var_guard495_dn2: f64 = *var_guard495_dn2_slot;
        let mut var_guard495_dn3: f64 = *var_guard495_dn3_slot;
        let mut var_guard495_dn4: f64 = *var_guard495_dn4_slot;
        let mut var_guard495_dn5: f64 = *var_guard495_dn5_slot;
        let mut var_guard495_rdb0: f64 = *var_guard495_rdb0_slot;
        let mut var_guard495_rdb1: f64 = *var_guard495_rdb1_slot;
        let mut var_guard495_rdb2: f64 = *var_guard495_rdb2_slot;
        let mut var_guard495_rdb3: f64 = *var_guard495_rdb3_slot;
        let mut var_guard495_rdn0: f64 = *var_guard495_rdn0_slot;
        let mut var_guard495_rdn1: f64 = *var_guard495_rdn1_slot;
        let mut var_guard495_rdn2: f64 = *var_guard495_rdn2_slot;
        let mut var_guard495_rdn3: f64 = *var_guard495_rdn3_slot;
        let mut var_guard495_rdn4: f64 = *var_guard495_rdn4_slot;
        let mut var_guard495_rdn5: f64 = *var_guard495_rdn5_slot;
        let mut var_guard495_rv: f64 = *var_guard495_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_db0: f64 = *var_idmultbot_db0_slot;
        let mut var_idmultbot_db1: f64 = *var_idmultbot_db1_slot;
        let mut var_idmultbot_db2: f64 = *var_idmultbot_db2_slot;
        let mut var_idmultbot_db3: f64 = *var_idmultbot_db3_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn1: f64 = *var_idmultbot_dn1_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_dn3: f64 = *var_idmultbot_dn3_slot;
        let mut var_idmultbot_dn4: f64 = *var_idmultbot_dn4_slot;
        let mut var_idmultbot_dn5: f64 = *var_idmultbot_dn5_slot;
        let mut var_idmultbot_rdb0: f64 = *var_idmultbot_rdb0_slot;
        let mut var_idmultbot_rdb1: f64 = *var_idmultbot_rdb1_slot;
        let mut var_idmultbot_rdb2: f64 = *var_idmultbot_rdb2_slot;
        let mut var_idmultbot_rdb3: f64 = *var_idmultbot_rdb3_slot;
        let mut var_idmultbot_rdn0: f64 = *var_idmultbot_rdn0_slot;
        let mut var_idmultbot_rdn1: f64 = *var_idmultbot_rdn1_slot;
        let mut var_idmultbot_rdn2: f64 = *var_idmultbot_rdn2_slot;
        let mut var_idmultbot_rdn3: f64 = *var_idmultbot_rdn3_slot;
        let mut var_idmultbot_rdn4: f64 = *var_idmultbot_rdn4_slot;
        let mut var_idmultbot_rdn5: f64 = *var_idmultbot_rdn5_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj0_rdb0: f64 = *var_nj0_rdb0_slot;
        let mut var_nj0_rdb1: f64 = *var_nj0_rdb1_slot;
        let mut var_nj0_rdb2: f64 = *var_nj0_rdb2_slot;
        let mut var_nj0_rdb3: f64 = *var_nj0_rdb3_slot;
        let mut var_nj0_rdn0: f64 = *var_nj0_rdn0_slot;
        let mut var_nj0_rdn1: f64 = *var_nj0_rdn1_slot;
        let mut var_nj0_rdn2: f64 = *var_nj0_rdn2_slot;
        let mut var_nj0_rdn3: f64 = *var_nj0_rdn3_slot;
        let mut var_nj0_rdn4: f64 = *var_nj0_rdn4_slot;
        let mut var_nj0_rdn5: f64 = *var_nj0_rdn5_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_nja10_rdb0: f64 = *var_nja10_rdb0_slot;
        let mut var_nja10_rdb1: f64 = *var_nja10_rdb1_slot;
        let mut var_nja10_rdb2: f64 = *var_nja10_rdb2_slot;
        let mut var_nja10_rdb3: f64 = *var_nja10_rdb3_slot;
        let mut var_nja10_rdn0: f64 = *var_nja10_rdn0_slot;
        let mut var_nja10_rdn1: f64 = *var_nja10_rdn1_slot;
        let mut var_nja10_rdn2: f64 = *var_nja10_rdn2_slot;
        let mut var_nja10_rdn3: f64 = *var_nja10_rdn3_slot;
        let mut var_nja10_rdn4: f64 = *var_nja10_rdn4_slot;
        let mut var_nja10_rdn5: f64 = *var_nja10_rdn5_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_db0: f64 = *var_pnn0_db0_slot;
        let mut var_pnn0_db1: f64 = *var_pnn0_db1_slot;
        let mut var_pnn0_db2: f64 = *var_pnn0_db2_slot;
        let mut var_pnn0_db3: f64 = *var_pnn0_db3_slot;
        let mut var_pnn0_dn0: f64 = *var_pnn0_dn0_slot;
        let mut var_pnn0_dn1: f64 = *var_pnn0_dn1_slot;
        let mut var_pnn0_dn2: f64 = *var_pnn0_dn2_slot;
        let mut var_pnn0_dn3: f64 = *var_pnn0_dn3_slot;
        let mut var_pnn0_dn4: f64 = *var_pnn0_dn4_slot;
        let mut var_pnn0_dn5: f64 = *var_pnn0_dn5_slot;
        let mut var_pnn0_rdb0: f64 = *var_pnn0_rdb0_slot;
        let mut var_pnn0_rdb1: f64 = *var_pnn0_rdb1_slot;
        let mut var_pnn0_rdb2: f64 = *var_pnn0_rdb2_slot;
        let mut var_pnn0_rdb3: f64 = *var_pnn0_rdb3_slot;
        let mut var_pnn0_rdn0: f64 = *var_pnn0_rdn0_slot;
        let mut var_pnn0_rdn1: f64 = *var_pnn0_rdn1_slot;
        let mut var_pnn0_rdn2: f64 = *var_pnn0_rdn2_slot;
        let mut var_pnn0_rdn3: f64 = *var_pnn0_rdn3_slot;
        let mut var_pnn0_rdn4: f64 = *var_pnn0_rdn4_slot;
        let mut var_pnn0_rdn5: f64 = *var_pnn0_rdn5_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_rdb0: f64 = *var_tmf1_rdb0_slot;
        let mut var_tmf1_rdb1: f64 = *var_tmf1_rdb1_slot;
        let mut var_tmf1_rdb2: f64 = *var_tmf1_rdb2_slot;
        let mut var_tmf1_rdb3: f64 = *var_tmf1_rdb3_slot;
        let mut var_tmf1_rdn0: f64 = *var_tmf1_rdn0_slot;
        let mut var_tmf1_rdn1: f64 = *var_tmf1_rdn1_slot;
        let mut var_tmf1_rdn2: f64 = *var_tmf1_rdn2_slot;
        let mut var_tmf1_rdn3: f64 = *var_tmf1_rdn3_slot;
        let mut var_tmf1_rdn4: f64 = *var_tmf1_rdn4_slot;
        let mut var_tmf1_rdn5: f64 = *var_tmf1_rdn5_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_rdb0: f64 = *var_tmf2_rdb0_slot;
        let mut var_tmf2_rdb1: f64 = *var_tmf2_rdb1_slot;
        let mut var_tmf2_rdb2: f64 = *var_tmf2_rdb2_slot;
        let mut var_tmf2_rdb3: f64 = *var_tmf2_rdb3_slot;
        let mut var_tmf2_rdn0: f64 = *var_tmf2_rdn0_slot;
        let mut var_tmf2_rdn1: f64 = *var_tmf2_rdn1_slot;
        let mut var_tmf2_rdn2: f64 = *var_tmf2_rdn2_slot;
        let mut var_tmf2_rdn3: f64 = *var_tmf2_rdn3_slot;
        let mut var_tmf2_rdn4: f64 = *var_tmf2_rdn4_slot;
        let mut var_tmf2_rdn5: f64 = *var_tmf2_rdn5_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_db0: f64 = *var_vha1_db0_slot;
        let mut var_vha1_db1: f64 = *var_vha1_db1_slot;
        let mut var_vha1_db2: f64 = *var_vha1_db2_slot;
        let mut var_vha1_db3: f64 = *var_vha1_db3_slot;
        let mut var_vha1_dn0: f64 = *var_vha1_dn0_slot;
        let mut var_vha1_dn1: f64 = *var_vha1_dn1_slot;
        let mut var_vha1_dn2: f64 = *var_vha1_dn2_slot;
        let mut var_vha1_dn3: f64 = *var_vha1_dn3_slot;
        let mut var_vha1_dn4: f64 = *var_vha1_dn4_slot;
        let mut var_vha1_dn5: f64 = *var_vha1_dn5_slot;
        let mut var_vha1_rdb0: f64 = *var_vha1_rdb0_slot;
        let mut var_vha1_rdb1: f64 = *var_vha1_rdb1_slot;
        let mut var_vha1_rdb2: f64 = *var_vha1_rdb2_slot;
        let mut var_vha1_rdb3: f64 = *var_vha1_rdb3_slot;
        let mut var_vha1_rdn0: f64 = *var_vha1_rdn0_slot;
        let mut var_vha1_rdn1: f64 = *var_vha1_rdn1_slot;
        let mut var_vha1_rdn2: f64 = *var_vha1_rdn2_slot;
        let mut var_vha1_rdn3: f64 = *var_vha1_rdn3_slot;
        let mut var_vha1_rdn4: f64 = *var_vha1_rdn4_slot;
        let mut var_vha1_rdn5: f64 = *var_vha1_rdn5_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign30670_e45823, assign30670_e45823_d_n0, assign30670_e45823_d_n1, assign30670_e45823_d_n2, assign30670_e45823_d_n3, assign30670_e45823_d_n4, assign30670_e45823_d_n5, assign30670_e45823_d_b0, assign30670_e45823_d_b1, assign30670_e45823_d_b2, assign30670_e45823_d_b3,) = {
    if (((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard493 == 0.0)) && (var_guard494 != 0.0)) {
        let assign30670_e45757: f64 = (-230.25850929940458);
        let assign30670_e45761: f64 = (var_vmax / var_nj1);
        let assign30670_e45765: f64 = (var_nj1 - var_nj0);
        let assign30670_e45766: f64 = (var_vha1 * assign30670_e45765);
        let assign30670_e45769: f64 = (var_nj0 * p.p85);
        let assign30670_e45770: f64 = (assign30670_e45766 / assign30670_e45769);
        let assign30670_e45771: f64 = (assign30670_e45761 + assign30670_e45770);
        let assign30670_e45772: f64 = (var_phitdinv * assign30670_e45771);
        let assign30670_e45773: f64 = (assign30670_e45757 - assign30670_e45772);
        let assign30670_e45777: f64 = (-230.25850929940458);
        let assign30670_e45781: f64 = (var_vmax / var_nj1);
        let assign30670_e45785: f64 = (var_nj1 - var_nj0);
        let assign30670_e45786: f64 = (var_vha1 * assign30670_e45785);
        let assign30670_e45789: f64 = (var_nj0 * p.p85);
        let assign30670_e45790: f64 = (assign30670_e45786 / assign30670_e45789);
        let assign30670_e45791: f64 = (assign30670_e45781 + assign30670_e45790);
        let assign30670_e45792: f64 = (var_phitdinv * assign30670_e45791);
        let assign30670_e45793: f64 = (assign30670_e45777 - assign30670_e45792);
        let assign30670_e45796: f64 = (-230.25850929940458);
        let assign30670_e45800: f64 = (var_vmax / var_nj1);
        let assign30670_e45804: f64 = (var_nj1 - var_nj0);
        let assign30670_e45805: f64 = (var_vha1 * assign30670_e45804);
        let assign30670_e45808: f64 = (var_nj0 * p.p85);
        let assign30670_e45809: f64 = (assign30670_e45805 / assign30670_e45808);
        let assign30670_e45810: f64 = (assign30670_e45800 + assign30670_e45809);
        let assign30670_e45811: f64 = (var_phitdinv * assign30670_e45810);
        let assign30670_e45812: f64 = (assign30670_e45796 - assign30670_e45811);
        let assign30670_e45814: f64 = (assign30670_e45812 * 0.3333333333333333);
        let assign30670_e45815: f64 = (1.0 + assign30670_e45814);
        let assign30670_e45816: f64 = (assign30670_e45793 * assign30670_e45815);
        let assign30670_e45817: f64 = (0.5 * assign30670_e45816);
        let assign30670_e45818: f64 = (1.0 + assign30670_e45817);
        let assign30670_e45819: f64 = (assign30670_e45773 * assign30670_e45818);
        let assign30670_e45820: f64 = (1.0 + assign30670_e45819);
        let assign30670_e45821: f64 = (1e-100 / assign30670_e45820);
        (assign30670_e45821, (-((1e-100 * (((-((var_phitdinv_dn0 * assign30670_e45771) + (var_phitdinv * ((((var_vmax_dn0 * var_nj1) - (var_vmax * var_nj1_dn0)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn0 * assign30670_e45765) + (var_vha1 * (var_nj1_dn0 - var_nj0_dn0))) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn0 * p.p85))) / (assign30670_e45769 * assign30670_e45769)))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-((var_phitdinv_dn0 * assign30670_e45791) + (var_phitdinv * ((((var_vmax_dn0 * var_nj1) - (var_vmax * var_nj1_dn0)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn0 * assign30670_e45785) + (var_vha1 * (var_nj1_dn0 - var_nj0_dn0))) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn0 * p.p85))) / (assign30670_e45789 * assign30670_e45789)))))) * assign30670_e45815) + (assign30670_e45793 * ((-((var_phitdinv_dn0 * assign30670_e45810) + (var_phitdinv * ((((var_vmax_dn0 * var_nj1) - (var_vmax * var_nj1_dn0)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn0 * assign30670_e45804) + (var_vha1 * (var_nj1_dn0 - var_nj0_dn0))) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn0 * p.p85))) / (assign30670_e45808 * assign30670_e45808)))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-((var_phitdinv_dn1 * assign30670_e45771) + (var_phitdinv * ((((var_vmax_dn1 * var_nj1) - (var_vmax * var_nj1_dn1)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn1 * assign30670_e45765) + (var_vha1 * (var_nj1_dn1 - var_nj0_dn1))) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn1 * p.p85))) / (assign30670_e45769 * assign30670_e45769)))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-((var_phitdinv_dn1 * assign30670_e45791) + (var_phitdinv * ((((var_vmax_dn1 * var_nj1) - (var_vmax * var_nj1_dn1)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn1 * assign30670_e45785) + (var_vha1 * (var_nj1_dn1 - var_nj0_dn1))) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn1 * p.p85))) / (assign30670_e45789 * assign30670_e45789)))))) * assign30670_e45815) + (assign30670_e45793 * ((-((var_phitdinv_dn1 * assign30670_e45810) + (var_phitdinv * ((((var_vmax_dn1 * var_nj1) - (var_vmax * var_nj1_dn1)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn1 * assign30670_e45804) + (var_vha1 * (var_nj1_dn1 - var_nj0_dn1))) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn1 * p.p85))) / (assign30670_e45808 * assign30670_e45808)))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-((var_phitdinv_dn2 * assign30670_e45771) + (var_phitdinv * ((((var_vmax_dn2 * var_nj1) - (var_vmax * var_nj1_dn2)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn2 * assign30670_e45765) + (var_vha1 * (var_nj1_dn2 - var_nj0_dn2))) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn2 * p.p85))) / (assign30670_e45769 * assign30670_e45769)))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-((var_phitdinv_dn2 * assign30670_e45791) + (var_phitdinv * ((((var_vmax_dn2 * var_nj1) - (var_vmax * var_nj1_dn2)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn2 * assign30670_e45785) + (var_vha1 * (var_nj1_dn2 - var_nj0_dn2))) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn2 * p.p85))) / (assign30670_e45789 * assign30670_e45789)))))) * assign30670_e45815) + (assign30670_e45793 * ((-((var_phitdinv_dn2 * assign30670_e45810) + (var_phitdinv * ((((var_vmax_dn2 * var_nj1) - (var_vmax * var_nj1_dn2)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn2 * assign30670_e45804) + (var_vha1 * (var_nj1_dn2 - var_nj0_dn2))) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn2 * p.p85))) / (assign30670_e45808 * assign30670_e45808)))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-((var_phitdinv_dn3 * assign30670_e45771) + (var_phitdinv * ((((var_vmax_dn3 * var_nj1) - (var_vmax * var_nj1_dn3)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn3 * assign30670_e45765) + (var_vha1 * (var_nj1_dn3 - var_nj0_dn3))) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn3 * p.p85))) / (assign30670_e45769 * assign30670_e45769)))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-((var_phitdinv_dn3 * assign30670_e45791) + (var_phitdinv * ((((var_vmax_dn3 * var_nj1) - (var_vmax * var_nj1_dn3)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn3 * assign30670_e45785) + (var_vha1 * (var_nj1_dn3 - var_nj0_dn3))) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn3 * p.p85))) / (assign30670_e45789 * assign30670_e45789)))))) * assign30670_e45815) + (assign30670_e45793 * ((-((var_phitdinv_dn3 * assign30670_e45810) + (var_phitdinv * ((((var_vmax_dn3 * var_nj1) - (var_vmax * var_nj1_dn3)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn3 * assign30670_e45804) + (var_vha1 * (var_nj1_dn3 - var_nj0_dn3))) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn3 * p.p85))) / (assign30670_e45808 * assign30670_e45808)))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-((var_phitdinv_dn4 * assign30670_e45771) + (var_phitdinv * ((((var_vmax_dn4 * var_nj1) - (var_vmax * var_nj1_dn4)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn4 * assign30670_e45765) + (var_vha1 * (var_nj1_dn4 - var_nj0_dn4))) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn4 * p.p85))) / (assign30670_e45769 * assign30670_e45769)))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-((var_phitdinv_dn4 * assign30670_e45791) + (var_phitdinv * ((((var_vmax_dn4 * var_nj1) - (var_vmax * var_nj1_dn4)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn4 * assign30670_e45785) + (var_vha1 * (var_nj1_dn4 - var_nj0_dn4))) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn4 * p.p85))) / (assign30670_e45789 * assign30670_e45789)))))) * assign30670_e45815) + (assign30670_e45793 * ((-((var_phitdinv_dn4 * assign30670_e45810) + (var_phitdinv * ((((var_vmax_dn4 * var_nj1) - (var_vmax * var_nj1_dn4)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn4 * assign30670_e45804) + (var_vha1 * (var_nj1_dn4 - var_nj0_dn4))) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn4 * p.p85))) / (assign30670_e45808 * assign30670_e45808)))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-((var_phitdinv_dn5 * assign30670_e45771) + (var_phitdinv * ((((var_vmax_dn5 * var_nj1) - (var_vmax * var_nj1_dn5)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn5 * assign30670_e45765) + (var_vha1 * (var_nj1_dn5 - var_nj0_dn5))) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn5 * p.p85))) / (assign30670_e45769 * assign30670_e45769)))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-((var_phitdinv_dn5 * assign30670_e45791) + (var_phitdinv * ((((var_vmax_dn5 * var_nj1) - (var_vmax * var_nj1_dn5)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn5 * assign30670_e45785) + (var_vha1 * (var_nj1_dn5 - var_nj0_dn5))) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn5 * p.p85))) / (assign30670_e45789 * assign30670_e45789)))))) * assign30670_e45815) + (assign30670_e45793 * ((-((var_phitdinv_dn5 * assign30670_e45810) + (var_phitdinv * ((((var_vmax_dn5 * var_nj1) - (var_vmax * var_nj1_dn5)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn5 * assign30670_e45804) + (var_vha1 * (var_nj1_dn5 - var_nj0_dn5))) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn5 * p.p85))) / (assign30670_e45808 * assign30670_e45808)))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-((var_phitdinv_db0 * assign30670_e45771) + (var_phitdinv * ((((var_vmax_db0 * var_nj1) - (var_vmax * var_nj1_db0)) / (var_nj1 * var_nj1)) + (((((var_vha1_db0 * assign30670_e45765) + (var_vha1 * (var_nj1_db0 - var_nj0_db0))) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_db0 * p.p85))) / (assign30670_e45769 * assign30670_e45769)))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-((var_phitdinv_db0 * assign30670_e45791) + (var_phitdinv * ((((var_vmax_db0 * var_nj1) - (var_vmax * var_nj1_db0)) / (var_nj1 * var_nj1)) + (((((var_vha1_db0 * assign30670_e45785) + (var_vha1 * (var_nj1_db0 - var_nj0_db0))) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_db0 * p.p85))) / (assign30670_e45789 * assign30670_e45789)))))) * assign30670_e45815) + (assign30670_e45793 * ((-((var_phitdinv_db0 * assign30670_e45810) + (var_phitdinv * ((((var_vmax_db0 * var_nj1) - (var_vmax * var_nj1_db0)) / (var_nj1 * var_nj1)) + (((((var_vha1_db0 * assign30670_e45804) + (var_vha1 * (var_nj1_db0 - var_nj0_db0))) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_db0 * p.p85))) / (assign30670_e45808 * assign30670_e45808)))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-((var_phitdinv_db1 * assign30670_e45771) + (var_phitdinv * ((((var_vmax_db1 * var_nj1) - (var_vmax * var_nj1_db1)) / (var_nj1 * var_nj1)) + (((((var_vha1_db1 * assign30670_e45765) + (var_vha1 * (var_nj1_db1 - var_nj0_db1))) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_db1 * p.p85))) / (assign30670_e45769 * assign30670_e45769)))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-((var_phitdinv_db1 * assign30670_e45791) + (var_phitdinv * ((((var_vmax_db1 * var_nj1) - (var_vmax * var_nj1_db1)) / (var_nj1 * var_nj1)) + (((((var_vha1_db1 * assign30670_e45785) + (var_vha1 * (var_nj1_db1 - var_nj0_db1))) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_db1 * p.p85))) / (assign30670_e45789 * assign30670_e45789)))))) * assign30670_e45815) + (assign30670_e45793 * ((-((var_phitdinv_db1 * assign30670_e45810) + (var_phitdinv * ((((var_vmax_db1 * var_nj1) - (var_vmax * var_nj1_db1)) / (var_nj1 * var_nj1)) + (((((var_vha1_db1 * assign30670_e45804) + (var_vha1 * (var_nj1_db1 - var_nj0_db1))) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_db1 * p.p85))) / (assign30670_e45808 * assign30670_e45808)))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-((var_phitdinv_db2 * assign30670_e45771) + (var_phitdinv * ((((var_vmax_db2 * var_nj1) - (var_vmax * var_nj1_db2)) / (var_nj1 * var_nj1)) + (((((var_vha1_db2 * assign30670_e45765) + (var_vha1 * (var_nj1_db2 - var_nj0_db2))) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_db2 * p.p85))) / (assign30670_e45769 * assign30670_e45769)))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-((var_phitdinv_db2 * assign30670_e45791) + (var_phitdinv * ((((var_vmax_db2 * var_nj1) - (var_vmax * var_nj1_db2)) / (var_nj1 * var_nj1)) + (((((var_vha1_db2 * assign30670_e45785) + (var_vha1 * (var_nj1_db2 - var_nj0_db2))) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_db2 * p.p85))) / (assign30670_e45789 * assign30670_e45789)))))) * assign30670_e45815) + (assign30670_e45793 * ((-((var_phitdinv_db2 * assign30670_e45810) + (var_phitdinv * ((((var_vmax_db2 * var_nj1) - (var_vmax * var_nj1_db2)) / (var_nj1 * var_nj1)) + (((((var_vha1_db2 * assign30670_e45804) + (var_vha1 * (var_nj1_db2 - var_nj0_db2))) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_db2 * p.p85))) / (assign30670_e45808 * assign30670_e45808)))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-((var_phitdinv_db3 * assign30670_e45771) + (var_phitdinv * ((((var_vmax_db3 * var_nj1) - (var_vmax * var_nj1_db3)) / (var_nj1 * var_nj1)) + (((((var_vha1_db3 * assign30670_e45765) + (var_vha1 * (var_nj1_db3 - var_nj0_db3))) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_db3 * p.p85))) / (assign30670_e45769 * assign30670_e45769)))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-((var_phitdinv_db3 * assign30670_e45791) + (var_phitdinv * ((((var_vmax_db3 * var_nj1) - (var_vmax * var_nj1_db3)) / (var_nj1 * var_nj1)) + (((((var_vha1_db3 * assign30670_e45785) + (var_vha1 * (var_nj1_db3 - var_nj0_db3))) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_db3 * p.p85))) / (assign30670_e45789 * assign30670_e45789)))))) * assign30670_e45815) + (assign30670_e45793 * ((-((var_phitdinv_db3 * assign30670_e45810) + (var_phitdinv * ((((var_vmax_db3 * var_nj1) - (var_vmax * var_nj1_db3)) / (var_nj1 * var_nj1)) + (((((var_vha1_db3 * assign30670_e45804) + (var_vha1 * (var_nj1_db3 - var_nj0_db3))) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_db3 * p.p85))) / (assign30670_e45808 * assign30670_e45808)))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign30670_e45823;
        var_exp_vmax_over_phitd_bot_dn0 = assign30670_e45823_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign30670_e45823_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign30670_e45823_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign30670_e45823_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign30670_e45823_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign30670_e45823_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign30670_e45823_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign30670_e45823_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign30670_e45823_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign30670_e45823_d_b3;
        var_exp_vmax_over_phitd_bot_rv = 0.0;
        var_exp_vmax_over_phitd_bot_rdn0 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn1 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn2 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn3 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn4 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn5 = 0.0;
        var_exp_vmax_over_phitd_bot_rdb0 = 0.0;
        var_exp_vmax_over_phitd_bot_rdb1 = 0.0;
        var_exp_vmax_over_phitd_bot_rdb2 = 0.0;
        var_exp_vmax_over_phitd_bot_rdb3 = 0.0;

        let (assign30680_e45903, assign30680_e45903_d_n0, assign30680_e45903_d_n1, assign30680_e45903_d_n2, assign30680_e45903_d_n3, assign30680_e45903_d_n4, assign30680_e45903_d_n5, assign30680_e45903_d_b0, assign30680_e45903_d_b1, assign30680_e45903_d_b2, assign30680_e45903_d_b3,) = {
    if (((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard493 == 0.0)) && (var_guard494 == 0.0)) {
        let assign30680_e45842: f64 = (var_vmax / var_nj1);
        let assign30680_e45846: f64 = (var_nj1 - var_nj0);
        let assign30680_e45847: f64 = (var_vha1 * assign30680_e45846);
        let assign30680_e45850: f64 = (var_nj0 * p.p85);
        let assign30680_e45851: f64 = (assign30680_e45847 / assign30680_e45850);
        let assign30680_e45852: f64 = (assign30680_e45842 + assign30680_e45851);
        let assign30680_e45853: f64 = (var_phitdinv * assign30680_e45852);
        let assign30680_e45855: f64 = (assign30680_e45853 - 230.25850929940458);
        let assign30680_e45861: f64 = (var_vmax / var_nj1);
        let assign30680_e45865: f64 = (var_nj1 - var_nj0);
        let assign30680_e45866: f64 = (var_vha1 * assign30680_e45865);
        let assign30680_e45869: f64 = (var_nj0 * p.p85);
        let assign30680_e45870: f64 = (assign30680_e45866 / assign30680_e45869);
        let assign30680_e45871: f64 = (assign30680_e45861 + assign30680_e45870);
        let assign30680_e45872: f64 = (var_phitdinv * assign30680_e45871);
        let assign30680_e45874: f64 = (assign30680_e45872 - 230.25850929940458);
        let assign30680_e45879: f64 = (var_vmax / var_nj1);
        let assign30680_e45883: f64 = (var_nj1 - var_nj0);
        let assign30680_e45884: f64 = (var_vha1 * assign30680_e45883);
        let assign30680_e45887: f64 = (var_nj0 * p.p85);
        let assign30680_e45888: f64 = (assign30680_e45884 / assign30680_e45887);
        let assign30680_e45889: f64 = (assign30680_e45879 + assign30680_e45888);
        let assign30680_e45890: f64 = (var_phitdinv * assign30680_e45889);
        let assign30680_e45892: f64 = (assign30680_e45890 - 230.25850929940458);
        let assign30680_e45894: f64 = (assign30680_e45892 * 0.3333333333333333);
        let assign30680_e45895: f64 = (1.0 + assign30680_e45894);
        let assign30680_e45896: f64 = (assign30680_e45874 * assign30680_e45895);
        let assign30680_e45897: f64 = (0.5 * assign30680_e45896);
        let assign30680_e45898: f64 = (1.0 + assign30680_e45897);
        let assign30680_e45899: f64 = (assign30680_e45855 * assign30680_e45898);
        let assign30680_e45900: f64 = (1.0 + assign30680_e45899);
        let assign30680_e45901: f64 = (1e100 * assign30680_e45900);
        (assign30680_e45901, (1e100 * ((((var_phitdinv_dn0 * assign30680_e45852) + (var_phitdinv * ((((var_vmax_dn0 * var_nj1) - (var_vmax * var_nj1_dn0)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn0 * assign30680_e45846) + (var_vha1 * (var_nj1_dn0 - var_nj0_dn0))) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn0 * p.p85))) / (assign30680_e45850 * assign30680_e45850))))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * ((((var_phitdinv_dn0 * assign30680_e45871) + (var_phitdinv * ((((var_vmax_dn0 * var_nj1) - (var_vmax * var_nj1_dn0)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn0 * assign30680_e45865) + (var_vha1 * (var_nj1_dn0 - var_nj0_dn0))) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn0 * p.p85))) / (assign30680_e45869 * assign30680_e45869))))) * assign30680_e45895) + (assign30680_e45874 * (((var_phitdinv_dn0 * assign30680_e45889) + (var_phitdinv * ((((var_vmax_dn0 * var_nj1) - (var_vmax * var_nj1_dn0)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn0 * assign30680_e45883) + (var_vha1 * (var_nj1_dn0 - var_nj0_dn0))) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn0 * p.p85))) / (assign30680_e45887 * assign30680_e45887))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_dn1 * assign30680_e45852) + (var_phitdinv * ((((var_vmax_dn1 * var_nj1) - (var_vmax * var_nj1_dn1)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn1 * assign30680_e45846) + (var_vha1 * (var_nj1_dn1 - var_nj0_dn1))) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn1 * p.p85))) / (assign30680_e45850 * assign30680_e45850))))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * ((((var_phitdinv_dn1 * assign30680_e45871) + (var_phitdinv * ((((var_vmax_dn1 * var_nj1) - (var_vmax * var_nj1_dn1)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn1 * assign30680_e45865) + (var_vha1 * (var_nj1_dn1 - var_nj0_dn1))) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn1 * p.p85))) / (assign30680_e45869 * assign30680_e45869))))) * assign30680_e45895) + (assign30680_e45874 * (((var_phitdinv_dn1 * assign30680_e45889) + (var_phitdinv * ((((var_vmax_dn1 * var_nj1) - (var_vmax * var_nj1_dn1)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn1 * assign30680_e45883) + (var_vha1 * (var_nj1_dn1 - var_nj0_dn1))) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn1 * p.p85))) / (assign30680_e45887 * assign30680_e45887))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_dn2 * assign30680_e45852) + (var_phitdinv * ((((var_vmax_dn2 * var_nj1) - (var_vmax * var_nj1_dn2)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn2 * assign30680_e45846) + (var_vha1 * (var_nj1_dn2 - var_nj0_dn2))) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn2 * p.p85))) / (assign30680_e45850 * assign30680_e45850))))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * ((((var_phitdinv_dn2 * assign30680_e45871) + (var_phitdinv * ((((var_vmax_dn2 * var_nj1) - (var_vmax * var_nj1_dn2)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn2 * assign30680_e45865) + (var_vha1 * (var_nj1_dn2 - var_nj0_dn2))) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn2 * p.p85))) / (assign30680_e45869 * assign30680_e45869))))) * assign30680_e45895) + (assign30680_e45874 * (((var_phitdinv_dn2 * assign30680_e45889) + (var_phitdinv * ((((var_vmax_dn2 * var_nj1) - (var_vmax * var_nj1_dn2)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn2 * assign30680_e45883) + (var_vha1 * (var_nj1_dn2 - var_nj0_dn2))) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn2 * p.p85))) / (assign30680_e45887 * assign30680_e45887))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_dn3 * assign30680_e45852) + (var_phitdinv * ((((var_vmax_dn3 * var_nj1) - (var_vmax * var_nj1_dn3)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn3 * assign30680_e45846) + (var_vha1 * (var_nj1_dn3 - var_nj0_dn3))) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn3 * p.p85))) / (assign30680_e45850 * assign30680_e45850))))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * ((((var_phitdinv_dn3 * assign30680_e45871) + (var_phitdinv * ((((var_vmax_dn3 * var_nj1) - (var_vmax * var_nj1_dn3)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn3 * assign30680_e45865) + (var_vha1 * (var_nj1_dn3 - var_nj0_dn3))) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn3 * p.p85))) / (assign30680_e45869 * assign30680_e45869))))) * assign30680_e45895) + (assign30680_e45874 * (((var_phitdinv_dn3 * assign30680_e45889) + (var_phitdinv * ((((var_vmax_dn3 * var_nj1) - (var_vmax * var_nj1_dn3)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn3 * assign30680_e45883) + (var_vha1 * (var_nj1_dn3 - var_nj0_dn3))) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn3 * p.p85))) / (assign30680_e45887 * assign30680_e45887))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_dn4 * assign30680_e45852) + (var_phitdinv * ((((var_vmax_dn4 * var_nj1) - (var_vmax * var_nj1_dn4)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn4 * assign30680_e45846) + (var_vha1 * (var_nj1_dn4 - var_nj0_dn4))) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn4 * p.p85))) / (assign30680_e45850 * assign30680_e45850))))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * ((((var_phitdinv_dn4 * assign30680_e45871) + (var_phitdinv * ((((var_vmax_dn4 * var_nj1) - (var_vmax * var_nj1_dn4)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn4 * assign30680_e45865) + (var_vha1 * (var_nj1_dn4 - var_nj0_dn4))) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn4 * p.p85))) / (assign30680_e45869 * assign30680_e45869))))) * assign30680_e45895) + (assign30680_e45874 * (((var_phitdinv_dn4 * assign30680_e45889) + (var_phitdinv * ((((var_vmax_dn4 * var_nj1) - (var_vmax * var_nj1_dn4)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn4 * assign30680_e45883) + (var_vha1 * (var_nj1_dn4 - var_nj0_dn4))) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn4 * p.p85))) / (assign30680_e45887 * assign30680_e45887))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_dn5 * assign30680_e45852) + (var_phitdinv * ((((var_vmax_dn5 * var_nj1) - (var_vmax * var_nj1_dn5)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn5 * assign30680_e45846) + (var_vha1 * (var_nj1_dn5 - var_nj0_dn5))) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn5 * p.p85))) / (assign30680_e45850 * assign30680_e45850))))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * ((((var_phitdinv_dn5 * assign30680_e45871) + (var_phitdinv * ((((var_vmax_dn5 * var_nj1) - (var_vmax * var_nj1_dn5)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn5 * assign30680_e45865) + (var_vha1 * (var_nj1_dn5 - var_nj0_dn5))) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn5 * p.p85))) / (assign30680_e45869 * assign30680_e45869))))) * assign30680_e45895) + (assign30680_e45874 * (((var_phitdinv_dn5 * assign30680_e45889) + (var_phitdinv * ((((var_vmax_dn5 * var_nj1) - (var_vmax * var_nj1_dn5)) / (var_nj1 * var_nj1)) + (((((var_vha1_dn5 * assign30680_e45883) + (var_vha1 * (var_nj1_dn5 - var_nj0_dn5))) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn5 * p.p85))) / (assign30680_e45887 * assign30680_e45887))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_db0 * assign30680_e45852) + (var_phitdinv * ((((var_vmax_db0 * var_nj1) - (var_vmax * var_nj1_db0)) / (var_nj1 * var_nj1)) + (((((var_vha1_db0 * assign30680_e45846) + (var_vha1 * (var_nj1_db0 - var_nj0_db0))) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_db0 * p.p85))) / (assign30680_e45850 * assign30680_e45850))))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * ((((var_phitdinv_db0 * assign30680_e45871) + (var_phitdinv * ((((var_vmax_db0 * var_nj1) - (var_vmax * var_nj1_db0)) / (var_nj1 * var_nj1)) + (((((var_vha1_db0 * assign30680_e45865) + (var_vha1 * (var_nj1_db0 - var_nj0_db0))) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_db0 * p.p85))) / (assign30680_e45869 * assign30680_e45869))))) * assign30680_e45895) + (assign30680_e45874 * (((var_phitdinv_db0 * assign30680_e45889) + (var_phitdinv * ((((var_vmax_db0 * var_nj1) - (var_vmax * var_nj1_db0)) / (var_nj1 * var_nj1)) + (((((var_vha1_db0 * assign30680_e45883) + (var_vha1 * (var_nj1_db0 - var_nj0_db0))) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_db0 * p.p85))) / (assign30680_e45887 * assign30680_e45887))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_db1 * assign30680_e45852) + (var_phitdinv * ((((var_vmax_db1 * var_nj1) - (var_vmax * var_nj1_db1)) / (var_nj1 * var_nj1)) + (((((var_vha1_db1 * assign30680_e45846) + (var_vha1 * (var_nj1_db1 - var_nj0_db1))) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_db1 * p.p85))) / (assign30680_e45850 * assign30680_e45850))))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * ((((var_phitdinv_db1 * assign30680_e45871) + (var_phitdinv * ((((var_vmax_db1 * var_nj1) - (var_vmax * var_nj1_db1)) / (var_nj1 * var_nj1)) + (((((var_vha1_db1 * assign30680_e45865) + (var_vha1 * (var_nj1_db1 - var_nj0_db1))) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_db1 * p.p85))) / (assign30680_e45869 * assign30680_e45869))))) * assign30680_e45895) + (assign30680_e45874 * (((var_phitdinv_db1 * assign30680_e45889) + (var_phitdinv * ((((var_vmax_db1 * var_nj1) - (var_vmax * var_nj1_db1)) / (var_nj1 * var_nj1)) + (((((var_vha1_db1 * assign30680_e45883) + (var_vha1 * (var_nj1_db1 - var_nj0_db1))) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_db1 * p.p85))) / (assign30680_e45887 * assign30680_e45887))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_db2 * assign30680_e45852) + (var_phitdinv * ((((var_vmax_db2 * var_nj1) - (var_vmax * var_nj1_db2)) / (var_nj1 * var_nj1)) + (((((var_vha1_db2 * assign30680_e45846) + (var_vha1 * (var_nj1_db2 - var_nj0_db2))) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_db2 * p.p85))) / (assign30680_e45850 * assign30680_e45850))))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * ((((var_phitdinv_db2 * assign30680_e45871) + (var_phitdinv * ((((var_vmax_db2 * var_nj1) - (var_vmax * var_nj1_db2)) / (var_nj1 * var_nj1)) + (((((var_vha1_db2 * assign30680_e45865) + (var_vha1 * (var_nj1_db2 - var_nj0_db2))) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_db2 * p.p85))) / (assign30680_e45869 * assign30680_e45869))))) * assign30680_e45895) + (assign30680_e45874 * (((var_phitdinv_db2 * assign30680_e45889) + (var_phitdinv * ((((var_vmax_db2 * var_nj1) - (var_vmax * var_nj1_db2)) / (var_nj1 * var_nj1)) + (((((var_vha1_db2 * assign30680_e45883) + (var_vha1 * (var_nj1_db2 - var_nj0_db2))) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_db2 * p.p85))) / (assign30680_e45887 * assign30680_e45887))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_db3 * assign30680_e45852) + (var_phitdinv * ((((var_vmax_db3 * var_nj1) - (var_vmax * var_nj1_db3)) / (var_nj1 * var_nj1)) + (((((var_vha1_db3 * assign30680_e45846) + (var_vha1 * (var_nj1_db3 - var_nj0_db3))) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_db3 * p.p85))) / (assign30680_e45850 * assign30680_e45850))))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * ((((var_phitdinv_db3 * assign30680_e45871) + (var_phitdinv * ((((var_vmax_db3 * var_nj1) - (var_vmax * var_nj1_db3)) / (var_nj1 * var_nj1)) + (((((var_vha1_db3 * assign30680_e45865) + (var_vha1 * (var_nj1_db3 - var_nj0_db3))) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_db3 * p.p85))) / (assign30680_e45869 * assign30680_e45869))))) * assign30680_e45895) + (assign30680_e45874 * (((var_phitdinv_db3 * assign30680_e45889) + (var_phitdinv * ((((var_vmax_db3 * var_nj1) - (var_vmax * var_nj1_db3)) / (var_nj1 * var_nj1)) + (((((var_vha1_db3 * assign30680_e45883) + (var_vha1 * (var_nj1_db3 - var_nj0_db3))) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_db3 * p.p85))) / (assign30680_e45887 * assign30680_e45887))))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign30680_e45903;
        var_exp_vmax_over_phitd_bot_dn0 = assign30680_e45903_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign30680_e45903_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign30680_e45903_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign30680_e45903_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign30680_e45903_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign30680_e45903_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign30680_e45903_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign30680_e45903_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign30680_e45903_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign30680_e45903_d_b3;
        var_exp_vmax_over_phitd_bot_rv = 0.0;
        var_exp_vmax_over_phitd_bot_rdn0 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn1 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn2 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn3 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn4 = 0.0;
        var_exp_vmax_over_phitd_bot_rdn5 = 0.0;
        var_exp_vmax_over_phitd_bot_rdb0 = 0.0;
        var_exp_vmax_over_phitd_bot_rdb1 = 0.0;
        var_exp_vmax_over_phitd_bot_rdb2 = 0.0;
        var_exp_vmax_over_phitd_bot_rdb3 = 0.0;

        let (assign30690_e45931, assign30690_e45931_d_n0, assign30690_e45931_d_n1, assign30690_e45931_d_n2, assign30690_e45931_d_n3, assign30690_e45931_d_n4, assign30690_e45931_d_n5, assign30690_e45931_d_b0, assign30690_e45931_d_b1, assign30690_e45931_d_b2, assign30690_e45931_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30690_e45915: f64 = (var_vmax * var_dnj1_dv);
        let assign30690_e45916: f64 = (var_nj1 - assign30690_e45915);
        let assign30690_e45919: f64 = (var_nj1 * var_nj1);
        let assign30690_e45920: f64 = (assign30690_e45916 / assign30690_e45919);
        let assign30690_e45923: f64 = (var_vha1 * var_dnj1_dv);
        let assign30690_e45926: f64 = (var_nj0 * p.p85);
        let assign30690_e45927: f64 = (assign30690_e45923 / assign30690_e45926);
        let assign30690_e45928: f64 = (assign30690_e45920 + assign30690_e45927);
        let assign30690_e45929: f64 = (var_phitdinv * assign30690_e45928);
        (assign30690_e45929, ((var_phitdinv_dn0 * assign30690_e45928) + (var_phitdinv * (((((var_nj1_dn0 - ((var_vmax_dn0 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn0))) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign30690_e45919 * assign30690_e45919)) + (((((var_vha1_dn0 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn0)) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn0 * p.p85))) / (assign30690_e45926 * assign30690_e45926))))), ((var_phitdinv_dn1 * assign30690_e45928) + (var_phitdinv * (((((var_nj1_dn1 - ((var_vmax_dn1 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn1))) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign30690_e45919 * assign30690_e45919)) + (((((var_vha1_dn1 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn1)) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn1 * p.p85))) / (assign30690_e45926 * assign30690_e45926))))), ((var_phitdinv_dn2 * assign30690_e45928) + (var_phitdinv * (((((var_nj1_dn2 - ((var_vmax_dn2 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn2))) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign30690_e45919 * assign30690_e45919)) + (((((var_vha1_dn2 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn2)) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn2 * p.p85))) / (assign30690_e45926 * assign30690_e45926))))), ((var_phitdinv_dn3 * assign30690_e45928) + (var_phitdinv * (((((var_nj1_dn3 - ((var_vmax_dn3 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn3))) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign30690_e45919 * assign30690_e45919)) + (((((var_vha1_dn3 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn3)) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn3 * p.p85))) / (assign30690_e45926 * assign30690_e45926))))), ((var_phitdinv_dn4 * assign30690_e45928) + (var_phitdinv * (((((var_nj1_dn4 - ((var_vmax_dn4 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn4))) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign30690_e45919 * assign30690_e45919)) + (((((var_vha1_dn4 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn4)) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn4 * p.p85))) / (assign30690_e45926 * assign30690_e45926))))), ((var_phitdinv_dn5 * assign30690_e45928) + (var_phitdinv * (((((var_nj1_dn5 - ((var_vmax_dn5 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn5))) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign30690_e45919 * assign30690_e45919)) + (((((var_vha1_dn5 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn5)) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn5 * p.p85))) / (assign30690_e45926 * assign30690_e45926))))), ((var_phitdinv_db0 * assign30690_e45928) + (var_phitdinv * (((((var_nj1_db0 - ((var_vmax_db0 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_db0))) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign30690_e45919 * assign30690_e45919)) + (((((var_vha1_db0 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_db0)) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_db0 * p.p85))) / (assign30690_e45926 * assign30690_e45926))))), ((var_phitdinv_db1 * assign30690_e45928) + (var_phitdinv * (((((var_nj1_db1 - ((var_vmax_db1 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_db1))) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign30690_e45919 * assign30690_e45919)) + (((((var_vha1_db1 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_db1)) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_db1 * p.p85))) / (assign30690_e45926 * assign30690_e45926))))), ((var_phitdinv_db2 * assign30690_e45928) + (var_phitdinv * (((((var_nj1_db2 - ((var_vmax_db2 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_db2))) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign30690_e45919 * assign30690_e45919)) + (((((var_vha1_db2 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_db2)) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_db2 * p.p85))) / (assign30690_e45926 * assign30690_e45926))))), ((var_phitdinv_db3 * assign30690_e45928) + (var_phitdinv * (((((var_nj1_db3 - ((var_vmax_db3 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_db3))) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign30690_e45919 * assign30690_e45919)) + (((((var_vha1_db3 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_db3)) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_db3 * p.p85))) / (assign30690_e45926 * assign30690_e45926))))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign30690_e45931;
        var_dvmax_over_phitd_dv_dn0 = assign30690_e45931_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign30690_e45931_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign30690_e45931_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign30690_e45931_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign30690_e45931_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign30690_e45931_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign30690_e45931_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign30690_e45931_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign30690_e45931_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign30690_e45931_d_b3;
        var_dvmax_over_phitd_dv_rv = 0.0;
        var_dvmax_over_phitd_dv_rdn0 = 0.0;
        var_dvmax_over_phitd_dv_rdn1 = 0.0;
        var_dvmax_over_phitd_dv_rdn2 = 0.0;
        var_dvmax_over_phitd_dv_rdn3 = 0.0;
        var_dvmax_over_phitd_dv_rdn4 = 0.0;
        var_dvmax_over_phitd_dv_rdn5 = 0.0;
        var_dvmax_over_phitd_dv_rdb0 = 0.0;
        var_dvmax_over_phitd_dv_rdb1 = 0.0;
        var_dvmax_over_phitd_dv_rdb2 = 0.0;
        var_dvmax_over_phitd_dv_rdb3 = 0.0;

        let (assign30700_e45949, assign30700_e45949_d_n0, assign30700_e45949_d_n1, assign30700_e45949_d_n2, assign30700_e45949_d_n3, assign30700_e45949_d_n4, assign30700_e45949_d_n5, assign30700_e45949_d_b0, assign30700_e45949_d_b1, assign30700_e45949_d_b2, assign30700_e45949_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30700_e45942: f64 = (var_vak - var_vmax);
        let assign30700_e45944: f64 = (assign30700_e45942 * var_dvmax_over_phitd_dv);
        let assign30700_e45945: f64 = (1.0 + assign30700_e45944);
        let assign30700_e45947: f64 = (assign30700_e45945 * var_exp_vmax_over_phitd_bot);
        (assign30700_e45947, (((((var_vak_dn0 - var_vmax_dn0) * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn0)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn0)), (((((var_vak_dn1 - var_vmax_dn1) * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn1)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn1)), (((((var_vak_dn2 - var_vmax_dn2) * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn2)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn2)), (((((var_vak_dn3 - var_vmax_dn3) * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn3)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn3)), (((((var_vak_dn4 - var_vmax_dn4) * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn4)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn4)), (((((var_vak_dn5 - var_vmax_dn5) * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn5)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn5)), (((((var_vak_db0 - var_vmax_db0) * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_db0)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_db0)), (((((var_vak_db1 - var_vmax_db1) * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_db1)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_db1)), (((((var_vak_db2 - var_vmax_db2) * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_db2)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_db2)), (((((var_vak_db3 - var_vmax_db3) * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_db3)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_db3)),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign30700_e45949;
        var_idmultbot_dn0 = assign30700_e45949_d_n0;
        var_idmultbot_dn1 = assign30700_e45949_d_n1;
        var_idmultbot_dn2 = assign30700_e45949_d_n2;
        var_idmultbot_dn3 = assign30700_e45949_d_n3;
        var_idmultbot_dn4 = assign30700_e45949_d_n4;
        var_idmultbot_dn5 = assign30700_e45949_d_n5;
        var_idmultbot_db0 = assign30700_e45949_d_b0;
        var_idmultbot_db1 = assign30700_e45949_d_b1;
        var_idmultbot_db2 = assign30700_e45949_d_b2;
        var_idmultbot_db3 = assign30700_e45949_d_b3;
        var_idmultbot_rv = 0.0;
        var_idmultbot_rdn0 = 0.0;
        var_idmultbot_rdn1 = 0.0;
        var_idmultbot_rdn2 = 0.0;
        var_idmultbot_rdn3 = 0.0;
        var_idmultbot_rdn4 = 0.0;
        var_idmultbot_rdn5 = 0.0;
        var_idmultbot_rdb0 = 0.0;
        var_idmultbot_rdb1 = 0.0;
        var_idmultbot_rdb2 = 0.0;
        var_idmultbot_rdb3 = 0.0;

        let (assign30710_e45963, assign30710_e45963_d_n0, assign30710_e45963_d_n1, assign30710_e45963_d_n2, assign30710_e45963_d_n3, assign30710_e45963_d_n4, assign30710_e45963_d_n5, assign30710_e45963_d_b0, assign30710_e45963_d_b1, assign30710_e45963_d_b2, assign30710_e45963_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30710_e45959: f64 = (var_nin * var_nin);
        let assign30710_e45961: f64 = (assign30710_e45959 / var_ndisti_i);
        (assign30710_e45961, (((((var_nin_dn0 * var_nin) + (var_nin * var_nin_dn0)) * var_ndisti_i) - (assign30710_e45959 * var_ndisti_i_dn0)) / (var_ndisti_i * var_ndisti_i)), (((((var_nin_dn1 * var_nin) + (var_nin * var_nin_dn1)) * var_ndisti_i) - (assign30710_e45959 * var_ndisti_i_dn1)) / (var_ndisti_i * var_ndisti_i)), (((((var_nin_dn2 * var_nin) + (var_nin * var_nin_dn2)) * var_ndisti_i) - (assign30710_e45959 * var_ndisti_i_dn2)) / (var_ndisti_i * var_ndisti_i)), (((((var_nin_dn3 * var_nin) + (var_nin * var_nin_dn3)) * var_ndisti_i) - (assign30710_e45959 * var_ndisti_i_dn3)) / (var_ndisti_i * var_ndisti_i)), (((((var_nin_dn4 * var_nin) + (var_nin * var_nin_dn4)) * var_ndisti_i) - (assign30710_e45959 * var_ndisti_i_dn4)) / (var_ndisti_i * var_ndisti_i)), (((((var_nin_dn5 * var_nin) + (var_nin * var_nin_dn5)) * var_ndisti_i) - (assign30710_e45959 * var_ndisti_i_dn5)) / (var_ndisti_i * var_ndisti_i)), (((((var_nin_db0 * var_nin) + (var_nin * var_nin_db0)) * var_ndisti_i) - (assign30710_e45959 * var_ndisti_i_db0)) / (var_ndisti_i * var_ndisti_i)), (((((var_nin_db1 * var_nin) + (var_nin * var_nin_db1)) * var_ndisti_i) - (assign30710_e45959 * var_ndisti_i_db1)) / (var_ndisti_i * var_ndisti_i)), (((((var_nin_db2 * var_nin) + (var_nin * var_nin_db2)) * var_ndisti_i) - (assign30710_e45959 * var_ndisti_i_db2)) / (var_ndisti_i * var_ndisti_i)), (((((var_nin_db3 * var_nin) + (var_nin * var_nin_db3)) * var_ndisti_i) - (assign30710_e45959 * var_ndisti_i_db3)) / (var_ndisti_i * var_ndisti_i)),)
    } else {
        (var_pnn0, var_pnn0_dn0, var_pnn0_dn1, var_pnn0_dn2, var_pnn0_dn3, var_pnn0_dn4, var_pnn0_dn5, var_pnn0_db0, var_pnn0_db1, var_pnn0_db2, var_pnn0_db3,)
    }
};
        var_pnn0 = assign30710_e45963;
        var_pnn0_dn0 = assign30710_e45963_d_n0;
        var_pnn0_dn1 = assign30710_e45963_d_n1;
        var_pnn0_dn2 = assign30710_e45963_d_n2;
        var_pnn0_dn3 = assign30710_e45963_d_n3;
        var_pnn0_dn4 = assign30710_e45963_d_n4;
        var_pnn0_dn5 = assign30710_e45963_d_n5;
        var_pnn0_db0 = assign30710_e45963_d_b0;
        var_pnn0_db1 = assign30710_e45963_d_b1;
        var_pnn0_db2 = assign30710_e45963_d_b2;
        var_pnn0_db3 = assign30710_e45963_d_b3;
        var_pnn0_rv = 0.0;
        var_pnn0_rdn0 = 0.0;
        var_pnn0_rdn1 = 0.0;
        var_pnn0_rdn2 = 0.0;
        var_pnn0_rdn3 = 0.0;
        var_pnn0_rdn4 = 0.0;
        var_pnn0_rdn5 = 0.0;
        var_pnn0_rdb0 = 0.0;
        var_pnn0_rdb1 = 0.0;
        var_pnn0_rdb2 = 0.0;
        var_pnn0_rdb3 = 0.0;

        let (assign30720_e45980, assign30720_e45980_d_n0, assign30720_e45980_d_n1, assign30720_e45980_d_n2, assign30720_e45980_d_n3, assign30720_e45980_d_n4, assign30720_e45980_d_n5, assign30720_e45980_d_b0, assign30720_e45980_d_b1, assign30720_e45980_d_b2, assign30720_e45980_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30720_e45973: f64 = (var_nfasti_i / var_phitdinv);
        let assign30720_e45976: f64 = (var_ndisti_i / var_pnn0);
        let assign30720_e45977: f64 = (assign30720_e45976).ln();
        let assign30720_e45978: f64 = (assign30720_e45973 * assign30720_e45977);
        (assign30720_e45978, (((((var_nfasti_i_dn0 * var_phitdinv) - (var_nfasti_i * var_phitdinv_dn0)) / (var_phitdinv * var_phitdinv)) * assign30720_e45977) + (assign30720_e45973 * ((((var_ndisti_i_dn0 * var_pnn0) - (var_ndisti_i * var_pnn0_dn0)) / (var_pnn0 * var_pnn0)) / assign30720_e45976))), (((((var_nfasti_i_dn1 * var_phitdinv) - (var_nfasti_i * var_phitdinv_dn1)) / (var_phitdinv * var_phitdinv)) * assign30720_e45977) + (assign30720_e45973 * ((((var_ndisti_i_dn1 * var_pnn0) - (var_ndisti_i * var_pnn0_dn1)) / (var_pnn0 * var_pnn0)) / assign30720_e45976))), (((((var_nfasti_i_dn2 * var_phitdinv) - (var_nfasti_i * var_phitdinv_dn2)) / (var_phitdinv * var_phitdinv)) * assign30720_e45977) + (assign30720_e45973 * ((((var_ndisti_i_dn2 * var_pnn0) - (var_ndisti_i * var_pnn0_dn2)) / (var_pnn0 * var_pnn0)) / assign30720_e45976))), (((((var_nfasti_i_dn3 * var_phitdinv) - (var_nfasti_i * var_phitdinv_dn3)) / (var_phitdinv * var_phitdinv)) * assign30720_e45977) + (assign30720_e45973 * ((((var_ndisti_i_dn3 * var_pnn0) - (var_ndisti_i * var_pnn0_dn3)) / (var_pnn0 * var_pnn0)) / assign30720_e45976))), (((((var_nfasti_i_dn4 * var_phitdinv) - (var_nfasti_i * var_phitdinv_dn4)) / (var_phitdinv * var_phitdinv)) * assign30720_e45977) + (assign30720_e45973 * ((((var_ndisti_i_dn4 * var_pnn0) - (var_ndisti_i * var_pnn0_dn4)) / (var_pnn0 * var_pnn0)) / assign30720_e45976))), (((((var_nfasti_i_dn5 * var_phitdinv) - (var_nfasti_i * var_phitdinv_dn5)) / (var_phitdinv * var_phitdinv)) * assign30720_e45977) + (assign30720_e45973 * ((((var_ndisti_i_dn5 * var_pnn0) - (var_ndisti_i * var_pnn0_dn5)) / (var_pnn0 * var_pnn0)) / assign30720_e45976))), (((((var_nfasti_i_db0 * var_phitdinv) - (var_nfasti_i * var_phitdinv_db0)) / (var_phitdinv * var_phitdinv)) * assign30720_e45977) + (assign30720_e45973 * ((((var_ndisti_i_db0 * var_pnn0) - (var_ndisti_i * var_pnn0_db0)) / (var_pnn0 * var_pnn0)) / assign30720_e45976))), (((((var_nfasti_i_db1 * var_phitdinv) - (var_nfasti_i * var_phitdinv_db1)) / (var_phitdinv * var_phitdinv)) * assign30720_e45977) + (assign30720_e45973 * ((((var_ndisti_i_db1 * var_pnn0) - (var_ndisti_i * var_pnn0_db1)) / (var_pnn0 * var_pnn0)) / assign30720_e45976))), (((((var_nfasti_i_db2 * var_phitdinv) - (var_nfasti_i * var_phitdinv_db2)) / (var_phitdinv * var_phitdinv)) * assign30720_e45977) + (assign30720_e45973 * ((((var_ndisti_i_db2 * var_pnn0) - (var_ndisti_i * var_pnn0_db2)) / (var_pnn0 * var_pnn0)) / assign30720_e45976))), (((((var_nfasti_i_db3 * var_phitdinv) - (var_nfasti_i * var_phitdinv_db3)) / (var_phitdinv * var_phitdinv)) * assign30720_e45977) + (assign30720_e45973 * ((((var_ndisti_i_db3 * var_pnn0) - (var_ndisti_i * var_pnn0_db3)) / (var_pnn0 * var_pnn0)) / assign30720_e45976))),)
    } else {
        (var_vha1, var_vha1_dn0, var_vha1_dn1, var_vha1_dn2, var_vha1_dn3, var_vha1_dn4, var_vha1_dn5, var_vha1_db0, var_vha1_db1, var_vha1_db2, var_vha1_db3,)
    }
};
        var_vha1 = assign30720_e45980;
        var_vha1_dn0 = assign30720_e45980_d_n0;
        var_vha1_dn1 = assign30720_e45980_d_n1;
        var_vha1_dn2 = assign30720_e45980_d_n2;
        var_vha1_dn3 = assign30720_e45980_d_n3;
        var_vha1_dn4 = assign30720_e45980_d_n4;
        var_vha1_dn5 = assign30720_e45980_d_n5;
        var_vha1_db0 = assign30720_e45980_d_b0;
        var_vha1_db1 = assign30720_e45980_d_b1;
        var_vha1_db2 = assign30720_e45980_d_b2;
        var_vha1_db3 = assign30720_e45980_d_b3;
        var_vha1_rv = 0.0;
        var_vha1_rdn0 = 0.0;
        var_vha1_rdn1 = 0.0;
        var_vha1_rdn2 = 0.0;
        var_vha1_rdn3 = 0.0;
        var_vha1_rdn4 = 0.0;
        var_vha1_rdn5 = 0.0;
        var_vha1_rdb0 = 0.0;
        var_vha1_rdb1 = 0.0;
        var_vha1_rdb2 = 0.0;
        var_vha1_rdb3 = 0.0;

        let assign30730_e45983: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard495 = assign30730_e45983;
        var_guard495_dn0 = 0.0;
        var_guard495_dn1 = 0.0;
        var_guard495_dn2 = 0.0;
        var_guard495_dn3 = 0.0;
        var_guard495_dn4 = 0.0;
        var_guard495_dn5 = 0.0;
        var_guard495_db0 = 0.0;
        var_guard495_db1 = 0.0;
        var_guard495_db2 = 0.0;
        var_guard495_db3 = 0.0;
        var_guard495_rv = 0.0;
        var_guard495_rdn0 = 0.0;
        var_guard495_rdn1 = 0.0;
        var_guard495_rdn2 = 0.0;
        var_guard495_rdn3 = 0.0;
        var_guard495_rdn4 = 0.0;
        var_guard495_rdn5 = 0.0;
        var_guard495_rdb0 = 0.0;
        var_guard495_rdb1 = 0.0;
        var_guard495_rdb2 = 0.0;
        var_guard495_rdb3 = 0.0;

        let (assign30740_e46001, assign30740_e46001_d_n0, assign30740_e46001_d_n1, assign30740_e46001_d_n2, assign30740_e46001_d_n3, assign30740_e46001_d_n4, assign30740_e46001_d_n5, assign30740_e46001_d_b0, assign30740_e46001_d_b1, assign30740_e46001_d_b2, assign30740_e46001_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30740_e45996: f64 = (var_vmax - var_vha1);
        let assign30740_e45997: f64 = (p.p86 * assign30740_e45996);
        let assign30740_e45999: f64 = (assign30740_e45997 + var_nfasti_i);
        (assign30740_e45999, ((p.p86 * (var_vmax_dn0 - var_vha1_dn0)) + var_nfasti_i_dn0), ((p.p86 * (var_vmax_dn1 - var_vha1_dn1)) + var_nfasti_i_dn1), ((p.p86 * (var_vmax_dn2 - var_vha1_dn2)) + var_nfasti_i_dn2), ((p.p86 * (var_vmax_dn3 - var_vha1_dn3)) + var_nfasti_i_dn3), ((p.p86 * (var_vmax_dn4 - var_vha1_dn4)) + var_nfasti_i_dn4), ((p.p86 * (var_vmax_dn5 - var_vha1_dn5)) + var_nfasti_i_dn5), ((p.p86 * (var_vmax_db0 - var_vha1_db0)) + var_nfasti_i_db0), ((p.p86 * (var_vmax_db1 - var_vha1_db1)) + var_nfasti_i_db1), ((p.p86 * (var_vmax_db2 - var_vha1_db2)) + var_nfasti_i_db2), ((p.p86 * (var_vmax_db3 - var_vha1_db3)) + var_nfasti_i_db3),)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign30740_e46001;
        var_nja10_dn0 = assign30740_e46001_d_n0;
        var_nja10_dn1 = assign30740_e46001_d_n1;
        var_nja10_dn2 = assign30740_e46001_d_n2;
        var_nja10_dn3 = assign30740_e46001_d_n3;
        var_nja10_dn4 = assign30740_e46001_d_n4;
        var_nja10_dn5 = assign30740_e46001_d_n5;
        var_nja10_db0 = assign30740_e46001_d_b0;
        var_nja10_db1 = assign30740_e46001_d_b1;
        var_nja10_db2 = assign30740_e46001_d_b2;
        var_nja10_db3 = assign30740_e46001_d_b3;
        var_nja10_rv = 0.0;
        var_nja10_rdn0 = 0.0;
        var_nja10_rdn1 = 0.0;
        var_nja10_rdn2 = 0.0;
        var_nja10_rdn3 = 0.0;
        var_nja10_rdn4 = 0.0;
        var_nja10_rdn5 = 0.0;
        var_nja10_rdb0 = 0.0;
        var_nja10_rdb1 = 0.0;
        var_nja10_rdb2 = 0.0;
        var_nja10_rdb3 = 0.0;

        let (assign30750_e46017, assign30750_e46017_d_n0, assign30750_e46017_d_n1, assign30750_e46017_d_n2, assign30750_e46017_d_n3, assign30750_e46017_d_n4, assign30750_e46017_d_n5, assign30750_e46017_d_b0, assign30750_e46017_d_b1, assign30750_e46017_d_b2, assign30750_e46017_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30750_e46014: f64 = (p.p86 * var_vha1);
        let assign30750_e46015: f64 = (var_nfasti_i - assign30750_e46014);
        (assign30750_e46015, (var_nfasti_i_dn0 - (p.p86 * var_vha1_dn0)), (var_nfasti_i_dn1 - (p.p86 * var_vha1_dn1)), (var_nfasti_i_dn2 - (p.p86 * var_vha1_dn2)), (var_nfasti_i_dn3 - (p.p86 * var_vha1_dn3)), (var_nfasti_i_dn4 - (p.p86 * var_vha1_dn4)), (var_nfasti_i_dn5 - (p.p86 * var_vha1_dn5)), (var_nfasti_i_db0 - (p.p86 * var_vha1_db0)), (var_nfasti_i_db1 - (p.p86 * var_vha1_db1)), (var_nfasti_i_db2 - (p.p86 * var_vha1_db2)), (var_nfasti_i_db3 - (p.p86 * var_vha1_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30750_e46017;
        var_nj0_dn0 = assign30750_e46017_d_n0;
        var_nj0_dn1 = assign30750_e46017_d_n1;
        var_nj0_dn2 = assign30750_e46017_d_n2;
        var_nj0_dn3 = assign30750_e46017_d_n3;
        var_nj0_dn4 = assign30750_e46017_d_n4;
        var_nj0_dn5 = assign30750_e46017_d_n5;
        var_nj0_db0 = assign30750_e46017_d_b0;
        var_nj0_db1 = assign30750_e46017_d_b1;
        var_nj0_db2 = assign30750_e46017_d_b2;
        var_nj0_db3 = assign30750_e46017_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign30760_e46033, assign30760_e46033_d_n0, assign30760_e46033_d_n1, assign30760_e46033_d_n2, assign30760_e46033_d_n3, assign30760_e46033_d_n4, assign30760_e46033_d_n5, assign30760_e46033_d_b0, assign30760_e46033_d_b1, assign30760_e46033_d_b2, assign30760_e46033_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30760_e46029: f64 = (p.p85 - var_nja10);
        let assign30760_e46031: f64 = (assign30760_e46029 - 0.01);
        (assign30760_e46031, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30760_e46033;
        var_tmf1_dn0 = assign30760_e46033_d_n0;
        var_tmf1_dn1 = assign30760_e46033_d_n1;
        var_tmf1_dn2 = assign30760_e46033_d_n2;
        var_tmf1_dn3 = assign30760_e46033_d_n3;
        var_tmf1_dn4 = assign30760_e46033_d_n4;
        var_tmf1_dn5 = assign30760_e46033_d_n5;
        var_tmf1_db0 = assign30760_e46033_d_b0;
        var_tmf1_db1 = assign30760_e46033_d_b1;
        var_tmf1_db2 = assign30760_e46033_d_b2;
        var_tmf1_db3 = assign30760_e46033_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign30770_e46049, assign30770_e46049_d_n0, assign30770_e46049_d_n1, assign30770_e46049_d_n2, assign30770_e46049_d_n3, assign30770_e46049_d_n4, assign30770_e46049_d_n5, assign30770_e46049_d_b0, assign30770_e46049_d_b1, assign30770_e46049_d_b2, assign30770_e46049_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30770_e46045: f64 = (4.0 * p.p85);
        let assign30770_e46047: f64 = (assign30770_e46045 * 0.01);
        (assign30770_e46047, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30770_e46049;
        var_tmf2_dn0 = assign30770_e46049_d_n0;
        var_tmf2_dn1 = assign30770_e46049_d_n1;
        var_tmf2_dn2 = assign30770_e46049_d_n2;
        var_tmf2_dn3 = assign30770_e46049_d_n3;
        var_tmf2_dn4 = assign30770_e46049_d_n4;
        var_tmf2_dn5 = assign30770_e46049_d_n5;
        var_tmf2_db0 = assign30770_e46049_d_b0;
        var_tmf2_db1 = assign30770_e46049_d_b1;
        var_tmf2_db2 = assign30770_e46049_d_b2;
        var_tmf2_db3 = assign30770_e46049_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30780_e46067, assign30780_e46067_d_n0, assign30780_e46067_d_n1, assign30780_e46067_d_n2, assign30780_e46067_d_n3, assign30780_e46067_d_n4, assign30780_e46067_d_n5, assign30780_e46067_d_b0, assign30780_e46067_d_b1, assign30780_e46067_d_b2, assign30780_e46067_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let (assign30780_e46065, assign30780_e46065_d_n0, assign30780_e46065_d_n1, assign30780_e46065_d_n2, assign30780_e46065_d_n3, assign30780_e46065_d_n4, assign30780_e46065_d_n5, assign30780_e46065_d_b0, assign30780_e46065_d_b1, assign30780_e46065_d_b2, assign30780_e46065_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30780_e46064: f64 = (-var_tmf2);
                (assign30780_e46064, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30780_e46065, assign30780_e46065_d_n0, assign30780_e46065_d_n1, assign30780_e46065_d_n2, assign30780_e46065_d_n3, assign30780_e46065_d_n4, assign30780_e46065_d_n5, assign30780_e46065_d_b0, assign30780_e46065_d_b1, assign30780_e46065_d_b2, assign30780_e46065_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30780_e46067;
        var_tmf2_dn0 = assign30780_e46067_d_n0;
        var_tmf2_dn1 = assign30780_e46067_d_n1;
        var_tmf2_dn2 = assign30780_e46067_d_n2;
        var_tmf2_dn3 = assign30780_e46067_d_n3;
        var_tmf2_dn4 = assign30780_e46067_d_n4;
        var_tmf2_dn5 = assign30780_e46067_d_n5;
        var_tmf2_db0 = assign30780_e46067_d_b0;
        var_tmf2_db1 = assign30780_e46067_d_b1;
        var_tmf2_db2 = assign30780_e46067_d_b2;
        var_tmf2_db3 = assign30780_e46067_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30790_e46084, assign30790_e46084_d_n0, assign30790_e46084_d_n1, assign30790_e46084_d_n2, assign30790_e46084_d_n3, assign30790_e46084_d_n4, assign30790_e46084_d_n5, assign30790_e46084_d_b0, assign30790_e46084_d_b1, assign30790_e46084_d_b2, assign30790_e46084_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30790_e46079: f64 = (var_tmf1 * var_tmf1);
        let assign30790_e46081: f64 = (assign30790_e46079 + var_tmf2);
        let assign30790_e46082: f64 = (assign30790_e46081).sqrt();
        (assign30790_e46082, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30790_e46082)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30790_e46082)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30790_e46082)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30790_e46082)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30790_e46082)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30790_e46082)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30790_e46082)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30790_e46082)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30790_e46082)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30790_e46082)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30790_e46084;
        var_tmf2_dn0 = assign30790_e46084_d_n0;
        var_tmf2_dn1 = assign30790_e46084_d_n1;
        var_tmf2_dn2 = assign30790_e46084_d_n2;
        var_tmf2_dn3 = assign30790_e46084_d_n3;
        var_tmf2_dn4 = assign30790_e46084_d_n4;
        var_tmf2_dn5 = assign30790_e46084_d_n5;
        var_tmf2_db0 = assign30790_e46084_d_b0;
        var_tmf2_db1 = assign30790_e46084_d_b1;
        var_tmf2_db2 = assign30790_e46084_d_b2;
        var_tmf2_db3 = assign30790_e46084_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_db0_slot = var_dvmax_over_phitd_dv_db0;
        *var_dvmax_over_phitd_dv_db1_slot = var_dvmax_over_phitd_dv_db1;
        *var_dvmax_over_phitd_dv_db2_slot = var_dvmax_over_phitd_dv_db2;
        *var_dvmax_over_phitd_dv_db3_slot = var_dvmax_over_phitd_dv_db3;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn1_slot = var_dvmax_over_phitd_dv_dn1;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_dn3_slot = var_dvmax_over_phitd_dv_dn3;
        *var_dvmax_over_phitd_dv_dn4_slot = var_dvmax_over_phitd_dv_dn4;
        *var_dvmax_over_phitd_dv_dn5_slot = var_dvmax_over_phitd_dv_dn5;
        *var_dvmax_over_phitd_dv_rdb0_slot = var_dvmax_over_phitd_dv_rdb0;
        *var_dvmax_over_phitd_dv_rdb1_slot = var_dvmax_over_phitd_dv_rdb1;
        *var_dvmax_over_phitd_dv_rdb2_slot = var_dvmax_over_phitd_dv_rdb2;
        *var_dvmax_over_phitd_dv_rdb3_slot = var_dvmax_over_phitd_dv_rdb3;
        *var_dvmax_over_phitd_dv_rdn0_slot = var_dvmax_over_phitd_dv_rdn0;
        *var_dvmax_over_phitd_dv_rdn1_slot = var_dvmax_over_phitd_dv_rdn1;
        *var_dvmax_over_phitd_dv_rdn2_slot = var_dvmax_over_phitd_dv_rdn2;
        *var_dvmax_over_phitd_dv_rdn3_slot = var_dvmax_over_phitd_dv_rdn3;
        *var_dvmax_over_phitd_dv_rdn4_slot = var_dvmax_over_phitd_dv_rdn4;
        *var_dvmax_over_phitd_dv_rdn5_slot = var_dvmax_over_phitd_dv_rdn5;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_exp_vmax_over_phitd_bot_slot = var_exp_vmax_over_phitd_bot;
        *var_exp_vmax_over_phitd_bot_db0_slot = var_exp_vmax_over_phitd_bot_db0;
        *var_exp_vmax_over_phitd_bot_db1_slot = var_exp_vmax_over_phitd_bot_db1;
        *var_exp_vmax_over_phitd_bot_db2_slot = var_exp_vmax_over_phitd_bot_db2;
        *var_exp_vmax_over_phitd_bot_db3_slot = var_exp_vmax_over_phitd_bot_db3;
        *var_exp_vmax_over_phitd_bot_dn0_slot = var_exp_vmax_over_phitd_bot_dn0;
        *var_exp_vmax_over_phitd_bot_dn1_slot = var_exp_vmax_over_phitd_bot_dn1;
        *var_exp_vmax_over_phitd_bot_dn2_slot = var_exp_vmax_over_phitd_bot_dn2;
        *var_exp_vmax_over_phitd_bot_dn3_slot = var_exp_vmax_over_phitd_bot_dn3;
        *var_exp_vmax_over_phitd_bot_dn4_slot = var_exp_vmax_over_phitd_bot_dn4;
        *var_exp_vmax_over_phitd_bot_dn5_slot = var_exp_vmax_over_phitd_bot_dn5;
        *var_exp_vmax_over_phitd_bot_rdb0_slot = var_exp_vmax_over_phitd_bot_rdb0;
        *var_exp_vmax_over_phitd_bot_rdb1_slot = var_exp_vmax_over_phitd_bot_rdb1;
        *var_exp_vmax_over_phitd_bot_rdb2_slot = var_exp_vmax_over_phitd_bot_rdb2;
        *var_exp_vmax_over_phitd_bot_rdb3_slot = var_exp_vmax_over_phitd_bot_rdb3;
        *var_exp_vmax_over_phitd_bot_rdn0_slot = var_exp_vmax_over_phitd_bot_rdn0;
        *var_exp_vmax_over_phitd_bot_rdn1_slot = var_exp_vmax_over_phitd_bot_rdn1;
        *var_exp_vmax_over_phitd_bot_rdn2_slot = var_exp_vmax_over_phitd_bot_rdn2;
        *var_exp_vmax_over_phitd_bot_rdn3_slot = var_exp_vmax_over_phitd_bot_rdn3;
        *var_exp_vmax_over_phitd_bot_rdn4_slot = var_exp_vmax_over_phitd_bot_rdn4;
        *var_exp_vmax_over_phitd_bot_rdn5_slot = var_exp_vmax_over_phitd_bot_rdn5;
        *var_exp_vmax_over_phitd_bot_rv_slot = var_exp_vmax_over_phitd_bot_rv;
        *var_guard495_slot = var_guard495;
        *var_guard495_db0_slot = var_guard495_db0;
        *var_guard495_db1_slot = var_guard495_db1;
        *var_guard495_db2_slot = var_guard495_db2;
        *var_guard495_db3_slot = var_guard495_db3;
        *var_guard495_dn0_slot = var_guard495_dn0;
        *var_guard495_dn1_slot = var_guard495_dn1;
        *var_guard495_dn2_slot = var_guard495_dn2;
        *var_guard495_dn3_slot = var_guard495_dn3;
        *var_guard495_dn4_slot = var_guard495_dn4;
        *var_guard495_dn5_slot = var_guard495_dn5;
        *var_guard495_rdb0_slot = var_guard495_rdb0;
        *var_guard495_rdb1_slot = var_guard495_rdb1;
        *var_guard495_rdb2_slot = var_guard495_rdb2;
        *var_guard495_rdb3_slot = var_guard495_rdb3;
        *var_guard495_rdn0_slot = var_guard495_rdn0;
        *var_guard495_rdn1_slot = var_guard495_rdn1;
        *var_guard495_rdn2_slot = var_guard495_rdn2;
        *var_guard495_rdn3_slot = var_guard495_rdn3;
        *var_guard495_rdn4_slot = var_guard495_rdn4;
        *var_guard495_rdn5_slot = var_guard495_rdn5;
        *var_guard495_rv_slot = var_guard495_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_db0_slot = var_idmultbot_db0;
        *var_idmultbot_db1_slot = var_idmultbot_db1;
        *var_idmultbot_db2_slot = var_idmultbot_db2;
        *var_idmultbot_db3_slot = var_idmultbot_db3;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn1_slot = var_idmultbot_dn1;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_dn3_slot = var_idmultbot_dn3;
        *var_idmultbot_dn4_slot = var_idmultbot_dn4;
        *var_idmultbot_dn5_slot = var_idmultbot_dn5;
        *var_idmultbot_rdb0_slot = var_idmultbot_rdb0;
        *var_idmultbot_rdb1_slot = var_idmultbot_rdb1;
        *var_idmultbot_rdb2_slot = var_idmultbot_rdb2;
        *var_idmultbot_rdb3_slot = var_idmultbot_rdb3;
        *var_idmultbot_rdn0_slot = var_idmultbot_rdn0;
        *var_idmultbot_rdn1_slot = var_idmultbot_rdn1;
        *var_idmultbot_rdn2_slot = var_idmultbot_rdn2;
        *var_idmultbot_rdn3_slot = var_idmultbot_rdn3;
        *var_idmultbot_rdn4_slot = var_idmultbot_rdn4;
        *var_idmultbot_rdn5_slot = var_idmultbot_rdn5;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj0_rdb0_slot = var_nj0_rdb0;
        *var_nj0_rdb1_slot = var_nj0_rdb1;
        *var_nj0_rdb2_slot = var_nj0_rdb2;
        *var_nj0_rdb3_slot = var_nj0_rdb3;
        *var_nj0_rdn0_slot = var_nj0_rdn0;
        *var_nj0_rdn1_slot = var_nj0_rdn1;
        *var_nj0_rdn2_slot = var_nj0_rdn2;
        *var_nj0_rdn3_slot = var_nj0_rdn3;
        *var_nj0_rdn4_slot = var_nj0_rdn4;
        *var_nj0_rdn5_slot = var_nj0_rdn5;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_nja10_rdb0_slot = var_nja10_rdb0;
        *var_nja10_rdb1_slot = var_nja10_rdb1;
        *var_nja10_rdb2_slot = var_nja10_rdb2;
        *var_nja10_rdb3_slot = var_nja10_rdb3;
        *var_nja10_rdn0_slot = var_nja10_rdn0;
        *var_nja10_rdn1_slot = var_nja10_rdn1;
        *var_nja10_rdn2_slot = var_nja10_rdn2;
        *var_nja10_rdn3_slot = var_nja10_rdn3;
        *var_nja10_rdn4_slot = var_nja10_rdn4;
        *var_nja10_rdn5_slot = var_nja10_rdn5;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_db0_slot = var_pnn0_db0;
        *var_pnn0_db1_slot = var_pnn0_db1;
        *var_pnn0_db2_slot = var_pnn0_db2;
        *var_pnn0_db3_slot = var_pnn0_db3;
        *var_pnn0_dn0_slot = var_pnn0_dn0;
        *var_pnn0_dn1_slot = var_pnn0_dn1;
        *var_pnn0_dn2_slot = var_pnn0_dn2;
        *var_pnn0_dn3_slot = var_pnn0_dn3;
        *var_pnn0_dn4_slot = var_pnn0_dn4;
        *var_pnn0_dn5_slot = var_pnn0_dn5;
        *var_pnn0_rdb0_slot = var_pnn0_rdb0;
        *var_pnn0_rdb1_slot = var_pnn0_rdb1;
        *var_pnn0_rdb2_slot = var_pnn0_rdb2;
        *var_pnn0_rdb3_slot = var_pnn0_rdb3;
        *var_pnn0_rdn0_slot = var_pnn0_rdn0;
        *var_pnn0_rdn1_slot = var_pnn0_rdn1;
        *var_pnn0_rdn2_slot = var_pnn0_rdn2;
        *var_pnn0_rdn3_slot = var_pnn0_rdn3;
        *var_pnn0_rdn4_slot = var_pnn0_rdn4;
        *var_pnn0_rdn5_slot = var_pnn0_rdn5;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_rdb0_slot = var_tmf1_rdb0;
        *var_tmf1_rdb1_slot = var_tmf1_rdb1;
        *var_tmf1_rdb2_slot = var_tmf1_rdb2;
        *var_tmf1_rdb3_slot = var_tmf1_rdb3;
        *var_tmf1_rdn0_slot = var_tmf1_rdn0;
        *var_tmf1_rdn1_slot = var_tmf1_rdn1;
        *var_tmf1_rdn2_slot = var_tmf1_rdn2;
        *var_tmf1_rdn3_slot = var_tmf1_rdn3;
        *var_tmf1_rdn4_slot = var_tmf1_rdn4;
        *var_tmf1_rdn5_slot = var_tmf1_rdn5;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_rdb0_slot = var_tmf2_rdb0;
        *var_tmf2_rdb1_slot = var_tmf2_rdb1;
        *var_tmf2_rdb2_slot = var_tmf2_rdb2;
        *var_tmf2_rdb3_slot = var_tmf2_rdb3;
        *var_tmf2_rdn0_slot = var_tmf2_rdn0;
        *var_tmf2_rdn1_slot = var_tmf2_rdn1;
        *var_tmf2_rdn2_slot = var_tmf2_rdn2;
        *var_tmf2_rdn3_slot = var_tmf2_rdn3;
        *var_tmf2_rdn4_slot = var_tmf2_rdn4;
        *var_tmf2_rdn5_slot = var_tmf2_rdn5;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_db0_slot = var_vha1_db0;
        *var_vha1_db1_slot = var_vha1_db1;
        *var_vha1_db2_slot = var_vha1_db2;
        *var_vha1_db3_slot = var_vha1_db3;
        *var_vha1_dn0_slot = var_vha1_dn0;
        *var_vha1_dn1_slot = var_vha1_dn1;
        *var_vha1_dn2_slot = var_vha1_dn2;
        *var_vha1_dn3_slot = var_vha1_dn3;
        *var_vha1_dn4_slot = var_vha1_dn4;
        *var_vha1_dn5_slot = var_vha1_dn5;
        *var_vha1_rdb0_slot = var_vha1_rdb0;
        *var_vha1_rdb1_slot = var_vha1_rdb1;
        *var_vha1_rdb2_slot = var_vha1_rdb2;
        *var_vha1_rdb3_slot = var_vha1_rdb3;
        *var_vha1_rdn0_slot = var_vha1_rdn0;
        *var_vha1_rdn1_slot = var_vha1_rdn1;
        *var_vha1_rdn2_slot = var_vha1_rdn2;
        *var_vha1_rdn3_slot = var_vha1_rdn3;
        *var_vha1_rdn4_slot = var_vha1_rdn4;
        *var_vha1_rdn5_slot = var_vha1_rdn5;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_80(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard495: f64,
        var_nfasti_i: f64,
        var_nfasti_i_db0: f64,
        var_nfasti_i_db1: f64,
        var_nfasti_i_db2: f64,
        var_nfasti_i_db3: f64,
        var_nfasti_i_dn0: f64,
        var_nfasti_i_dn1: f64,
        var_nfasti_i_dn2: f64,
        var_nfasti_i_dn3: f64,
        var_nfasti_i_dn4: f64,
        var_nfasti_i_dn5: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_db0_slot: &mut f64,
        var_dfn_sl_db1_slot: &mut f64,
        var_dfn_sl_db2_slot: &mut f64,
        var_dfn_sl_db3_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn1_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_dn3_slot: &mut f64,
        var_dfn_sl_dn4_slot: &mut f64,
        var_dfn_sl_dn5_slot: &mut f64,
        var_dfn_sl_rdb0_slot: &mut f64,
        var_dfn_sl_rdb1_slot: &mut f64,
        var_dfn_sl_rdb2_slot: &mut f64,
        var_dfn_sl_rdb3_slot: &mut f64,
        var_dfn_sl_rdn0_slot: &mut f64,
        var_dfn_sl_rdn1_slot: &mut f64,
        var_dfn_sl_rdn2_slot: &mut f64,
        var_dfn_sl_rdn3_slot: &mut f64,
        var_dfn_sl_rdn4_slot: &mut f64,
        var_dfn_sl_rdn5_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_db0_slot: &mut f64,
        var_dfn_su_db1_slot: &mut f64,
        var_dfn_su_db2_slot: &mut f64,
        var_dfn_su_db3_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn1_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_dn3_slot: &mut f64,
        var_dfn_su_dn4_slot: &mut f64,
        var_dfn_su_dn5_slot: &mut f64,
        var_dfn_su_rdb0_slot: &mut f64,
        var_dfn_su_rdb1_slot: &mut f64,
        var_dfn_su_rdb2_slot: &mut f64,
        var_dfn_su_rdb3_slot: &mut f64,
        var_dfn_su_rdn0_slot: &mut f64,
        var_dfn_su_rdn1_slot: &mut f64,
        var_dfn_su_rdn2_slot: &mut f64,
        var_dfn_su_rdn3_slot: &mut f64,
        var_dfn_su_rdn4_slot: &mut f64,
        var_dfn_su_rdn5_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj0_rdb0_slot: &mut f64,
        var_nj0_rdb1_slot: &mut f64,
        var_nj0_rdb2_slot: &mut f64,
        var_nj0_rdb3_slot: &mut f64,
        var_nj0_rdn0_slot: &mut f64,
        var_nj0_rdn1_slot: &mut f64,
        var_nj0_rdn2_slot: &mut f64,
        var_nj0_rdn3_slot: &mut f64,
        var_nj0_rdn4_slot: &mut f64,
        var_nj0_rdn5_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nj1_rdb0_slot: &mut f64,
        var_nj1_rdb1_slot: &mut f64,
        var_nj1_rdb2_slot: &mut f64,
        var_nj1_rdb3_slot: &mut f64,
        var_nj1_rdn0_slot: &mut f64,
        var_nj1_rdn1_slot: &mut f64,
        var_nj1_rdn2_slot: &mut f64,
        var_nj1_rdn3_slot: &mut f64,
        var_nj1_rdn4_slot: &mut f64,
        var_nj1_rdn5_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_nja11_rdb0_slot: &mut f64,
        var_nja11_rdb1_slot: &mut f64,
        var_nja11_rdb2_slot: &mut f64,
        var_nja11_rdb3_slot: &mut f64,
        var_nja11_rdn0_slot: &mut f64,
        var_nja11_rdn1_slot: &mut f64,
        var_nja11_rdn2_slot: &mut f64,
        var_nja11_rdn3_slot: &mut f64,
        var_nja11_rdn4_slot: &mut f64,
        var_nja11_rdn5_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_rdb0_slot: &mut f64,
        var_tmf1_rdb1_slot: &mut f64,
        var_tmf1_rdb2_slot: &mut f64,
        var_tmf1_rdb3_slot: &mut f64,
        var_tmf1_rdn0_slot: &mut f64,
        var_tmf1_rdn1_slot: &mut f64,
        var_tmf1_rdn2_slot: &mut f64,
        var_tmf1_rdn3_slot: &mut f64,
        var_tmf1_rdn4_slot: &mut f64,
        var_tmf1_rdn5_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_rdb0_slot: &mut f64,
        var_tmf2_rdb1_slot: &mut f64,
        var_tmf2_rdb2_slot: &mut f64,
        var_tmf2_rdb3_slot: &mut f64,
        var_tmf2_rdn0_slot: &mut f64,
        var_tmf2_rdn1_slot: &mut f64,
        var_tmf2_rdn2_slot: &mut f64,
        var_tmf2_rdn3_slot: &mut f64,
        var_tmf2_rdn4_slot: &mut f64,
        var_tmf2_rdn5_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_db0: f64 = *var_dfn_sl_db0_slot;
        let mut var_dfn_sl_db1: f64 = *var_dfn_sl_db1_slot;
        let mut var_dfn_sl_db2: f64 = *var_dfn_sl_db2_slot;
        let mut var_dfn_sl_db3: f64 = *var_dfn_sl_db3_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn1: f64 = *var_dfn_sl_dn1_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_dn3: f64 = *var_dfn_sl_dn3_slot;
        let mut var_dfn_sl_dn4: f64 = *var_dfn_sl_dn4_slot;
        let mut var_dfn_sl_dn5: f64 = *var_dfn_sl_dn5_slot;
        let mut var_dfn_sl_rdb0: f64 = *var_dfn_sl_rdb0_slot;
        let mut var_dfn_sl_rdb1: f64 = *var_dfn_sl_rdb1_slot;
        let mut var_dfn_sl_rdb2: f64 = *var_dfn_sl_rdb2_slot;
        let mut var_dfn_sl_rdb3: f64 = *var_dfn_sl_rdb3_slot;
        let mut var_dfn_sl_rdn0: f64 = *var_dfn_sl_rdn0_slot;
        let mut var_dfn_sl_rdn1: f64 = *var_dfn_sl_rdn1_slot;
        let mut var_dfn_sl_rdn2: f64 = *var_dfn_sl_rdn2_slot;
        let mut var_dfn_sl_rdn3: f64 = *var_dfn_sl_rdn3_slot;
        let mut var_dfn_sl_rdn4: f64 = *var_dfn_sl_rdn4_slot;
        let mut var_dfn_sl_rdn5: f64 = *var_dfn_sl_rdn5_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_db0: f64 = *var_dfn_su_db0_slot;
        let mut var_dfn_su_db1: f64 = *var_dfn_su_db1_slot;
        let mut var_dfn_su_db2: f64 = *var_dfn_su_db2_slot;
        let mut var_dfn_su_db3: f64 = *var_dfn_su_db3_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn1: f64 = *var_dfn_su_dn1_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_dn3: f64 = *var_dfn_su_dn3_slot;
        let mut var_dfn_su_dn4: f64 = *var_dfn_su_dn4_slot;
        let mut var_dfn_su_dn5: f64 = *var_dfn_su_dn5_slot;
        let mut var_dfn_su_rdb0: f64 = *var_dfn_su_rdb0_slot;
        let mut var_dfn_su_rdb1: f64 = *var_dfn_su_rdb1_slot;
        let mut var_dfn_su_rdb2: f64 = *var_dfn_su_rdb2_slot;
        let mut var_dfn_su_rdb3: f64 = *var_dfn_su_rdb3_slot;
        let mut var_dfn_su_rdn0: f64 = *var_dfn_su_rdn0_slot;
        let mut var_dfn_su_rdn1: f64 = *var_dfn_su_rdn1_slot;
        let mut var_dfn_su_rdn2: f64 = *var_dfn_su_rdn2_slot;
        let mut var_dfn_su_rdn3: f64 = *var_dfn_su_rdn3_slot;
        let mut var_dfn_su_rdn4: f64 = *var_dfn_su_rdn4_slot;
        let mut var_dfn_su_rdn5: f64 = *var_dfn_su_rdn5_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj0_rdb0: f64 = *var_nj0_rdb0_slot;
        let mut var_nj0_rdb1: f64 = *var_nj0_rdb1_slot;
        let mut var_nj0_rdb2: f64 = *var_nj0_rdb2_slot;
        let mut var_nj0_rdb3: f64 = *var_nj0_rdb3_slot;
        let mut var_nj0_rdn0: f64 = *var_nj0_rdn0_slot;
        let mut var_nj0_rdn1: f64 = *var_nj0_rdn1_slot;
        let mut var_nj0_rdn2: f64 = *var_nj0_rdn2_slot;
        let mut var_nj0_rdn3: f64 = *var_nj0_rdn3_slot;
        let mut var_nj0_rdn4: f64 = *var_nj0_rdn4_slot;
        let mut var_nj0_rdn5: f64 = *var_nj0_rdn5_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nj1_rdb0: f64 = *var_nj1_rdb0_slot;
        let mut var_nj1_rdb1: f64 = *var_nj1_rdb1_slot;
        let mut var_nj1_rdb2: f64 = *var_nj1_rdb2_slot;
        let mut var_nj1_rdb3: f64 = *var_nj1_rdb3_slot;
        let mut var_nj1_rdn0: f64 = *var_nj1_rdn0_slot;
        let mut var_nj1_rdn1: f64 = *var_nj1_rdn1_slot;
        let mut var_nj1_rdn2: f64 = *var_nj1_rdn2_slot;
        let mut var_nj1_rdn3: f64 = *var_nj1_rdn3_slot;
        let mut var_nj1_rdn4: f64 = *var_nj1_rdn4_slot;
        let mut var_nj1_rdn5: f64 = *var_nj1_rdn5_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_nja11_rdb0: f64 = *var_nja11_rdb0_slot;
        let mut var_nja11_rdb1: f64 = *var_nja11_rdb1_slot;
        let mut var_nja11_rdb2: f64 = *var_nja11_rdb2_slot;
        let mut var_nja11_rdb3: f64 = *var_nja11_rdb3_slot;
        let mut var_nja11_rdn0: f64 = *var_nja11_rdn0_slot;
        let mut var_nja11_rdn1: f64 = *var_nja11_rdn1_slot;
        let mut var_nja11_rdn2: f64 = *var_nja11_rdn2_slot;
        let mut var_nja11_rdn3: f64 = *var_nja11_rdn3_slot;
        let mut var_nja11_rdn4: f64 = *var_nja11_rdn4_slot;
        let mut var_nja11_rdn5: f64 = *var_nja11_rdn5_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_rdb0: f64 = *var_tmf1_rdb0_slot;
        let mut var_tmf1_rdb1: f64 = *var_tmf1_rdb1_slot;
        let mut var_tmf1_rdb2: f64 = *var_tmf1_rdb2_slot;
        let mut var_tmf1_rdb3: f64 = *var_tmf1_rdb3_slot;
        let mut var_tmf1_rdn0: f64 = *var_tmf1_rdn0_slot;
        let mut var_tmf1_rdn1: f64 = *var_tmf1_rdn1_slot;
        let mut var_tmf1_rdn2: f64 = *var_tmf1_rdn2_slot;
        let mut var_tmf1_rdn3: f64 = *var_tmf1_rdn3_slot;
        let mut var_tmf1_rdn4: f64 = *var_tmf1_rdn4_slot;
        let mut var_tmf1_rdn5: f64 = *var_tmf1_rdn5_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_rdb0: f64 = *var_tmf2_rdb0_slot;
        let mut var_tmf2_rdb1: f64 = *var_tmf2_rdb1_slot;
        let mut var_tmf2_rdb2: f64 = *var_tmf2_rdb2_slot;
        let mut var_tmf2_rdb3: f64 = *var_tmf2_rdb3_slot;
        let mut var_tmf2_rdn0: f64 = *var_tmf2_rdn0_slot;
        let mut var_tmf2_rdn1: f64 = *var_tmf2_rdn1_slot;
        let mut var_tmf2_rdn2: f64 = *var_tmf2_rdn2_slot;
        let mut var_tmf2_rdn3: f64 = *var_tmf2_rdn3_slot;
        let mut var_tmf2_rdn4: f64 = *var_tmf2_rdn4_slot;
        let mut var_tmf2_rdn5: f64 = *var_tmf2_rdn5_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign30800_e46102, assign30800_e46102_d_n0, assign30800_e46102_d_n1, assign30800_e46102_d_n2, assign30800_e46102_d_n3, assign30800_e46102_d_n4, assign30800_e46102_d_n5, assign30800_e46102_d_b0, assign30800_e46102_d_b1, assign30800_e46102_d_b2, assign30800_e46102_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30800_e46098: f64 = (var_tmf1 / var_tmf2);
        let assign30800_e46099: f64 = (1.0 + assign30800_e46098);
        let assign30800_e46100: f64 = (0.5 * assign30800_e46099);
        (assign30800_e46100, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign30800_e46102;
        var_dfn_su_dn0 = assign30800_e46102_d_n0;
        var_dfn_su_dn1 = assign30800_e46102_d_n1;
        var_dfn_su_dn2 = assign30800_e46102_d_n2;
        var_dfn_su_dn3 = assign30800_e46102_d_n3;
        var_dfn_su_dn4 = assign30800_e46102_d_n4;
        var_dfn_su_dn5 = assign30800_e46102_d_n5;
        var_dfn_su_db0 = assign30800_e46102_d_b0;
        var_dfn_su_db1 = assign30800_e46102_d_b1;
        var_dfn_su_db2 = assign30800_e46102_d_b2;
        var_dfn_su_db3 = assign30800_e46102_d_b3;
        var_dfn_su_rv = 0.0;
        var_dfn_su_rdn0 = 0.0;
        var_dfn_su_rdn1 = 0.0;
        var_dfn_su_rdn2 = 0.0;
        var_dfn_su_rdn3 = 0.0;
        var_dfn_su_rdn4 = 0.0;
        var_dfn_su_rdn5 = 0.0;
        var_dfn_su_rdb0 = 0.0;
        var_dfn_su_rdb1 = 0.0;
        var_dfn_su_rdb2 = 0.0;
        var_dfn_su_rdb3 = 0.0;

        let (assign30810_e46120, assign30810_e46120_d_n0, assign30810_e46120_d_n1, assign30810_e46120_d_n2, assign30810_e46120_d_n3, assign30810_e46120_d_n4, assign30810_e46120_d_n5, assign30810_e46120_d_b0, assign30810_e46120_d_b1, assign30810_e46120_d_b2, assign30810_e46120_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30810_e46116: f64 = (var_tmf1 + var_tmf2);
        let assign30810_e46117: f64 = (0.5 * assign30810_e46116);
        let assign30810_e46118: f64 = (p.p85 - assign30810_e46117);
        (assign30810_e46118, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign30810_e46120;
        var_nja11_dn0 = assign30810_e46120_d_n0;
        var_nja11_dn1 = assign30810_e46120_d_n1;
        var_nja11_dn2 = assign30810_e46120_d_n2;
        var_nja11_dn3 = assign30810_e46120_d_n3;
        var_nja11_dn4 = assign30810_e46120_d_n4;
        var_nja11_dn5 = assign30810_e46120_d_n5;
        var_nja11_db0 = assign30810_e46120_d_b0;
        var_nja11_db1 = assign30810_e46120_d_b1;
        var_nja11_db2 = assign30810_e46120_d_b2;
        var_nja11_db3 = assign30810_e46120_d_b3;
        var_nja11_rv = 0.0;
        var_nja11_rdn0 = 0.0;
        var_nja11_rdn1 = 0.0;
        var_nja11_rdn2 = 0.0;
        var_nja11_rdn3 = 0.0;
        var_nja11_rdn4 = 0.0;
        var_nja11_rdn5 = 0.0;
        var_nja11_rdb0 = 0.0;
        var_nja11_rdb1 = 0.0;
        var_nja11_rdb2 = 0.0;
        var_nja11_rdb3 = 0.0;

        let (assign30820_e46136, assign30820_e46136_d_n0, assign30820_e46136_d_n1, assign30820_e46136_d_n2, assign30820_e46136_d_n3, assign30820_e46136_d_n4, assign30820_e46136_d_n5, assign30820_e46136_d_b0, assign30820_e46136_d_b1, assign30820_e46136_d_b2, assign30820_e46136_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30820_e46132: f64 = (var_nja11 - var_nfasti_i);
        let assign30820_e46134: f64 = (assign30820_e46132 - 0.01);
        (assign30820_e46134, (var_nja11_dn0 - var_nfasti_i_dn0), (var_nja11_dn1 - var_nfasti_i_dn1), (var_nja11_dn2 - var_nfasti_i_dn2), (var_nja11_dn3 - var_nfasti_i_dn3), (var_nja11_dn4 - var_nfasti_i_dn4), (var_nja11_dn5 - var_nfasti_i_dn5), (var_nja11_db0 - var_nfasti_i_db0), (var_nja11_db1 - var_nfasti_i_db1), (var_nja11_db2 - var_nfasti_i_db2), (var_nja11_db3 - var_nfasti_i_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30820_e46136;
        var_tmf1_dn0 = assign30820_e46136_d_n0;
        var_tmf1_dn1 = assign30820_e46136_d_n1;
        var_tmf1_dn2 = assign30820_e46136_d_n2;
        var_tmf1_dn3 = assign30820_e46136_d_n3;
        var_tmf1_dn4 = assign30820_e46136_d_n4;
        var_tmf1_dn5 = assign30820_e46136_d_n5;
        var_tmf1_db0 = assign30820_e46136_d_b0;
        var_tmf1_db1 = assign30820_e46136_d_b1;
        var_tmf1_db2 = assign30820_e46136_d_b2;
        var_tmf1_db3 = assign30820_e46136_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign30830_e46152, assign30830_e46152_d_n0, assign30830_e46152_d_n1, assign30830_e46152_d_n2, assign30830_e46152_d_n3, assign30830_e46152_d_n4, assign30830_e46152_d_n5, assign30830_e46152_d_b0, assign30830_e46152_d_b1, assign30830_e46152_d_b2, assign30830_e46152_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30830_e46148: f64 = (4.0 * var_nfasti_i);
        let assign30830_e46150: f64 = (assign30830_e46148 * 0.01);
        (assign30830_e46150, ((4.0 * var_nfasti_i_dn0) * 0.01), ((4.0 * var_nfasti_i_dn1) * 0.01), ((4.0 * var_nfasti_i_dn2) * 0.01), ((4.0 * var_nfasti_i_dn3) * 0.01), ((4.0 * var_nfasti_i_dn4) * 0.01), ((4.0 * var_nfasti_i_dn5) * 0.01), ((4.0 * var_nfasti_i_db0) * 0.01), ((4.0 * var_nfasti_i_db1) * 0.01), ((4.0 * var_nfasti_i_db2) * 0.01), ((4.0 * var_nfasti_i_db3) * 0.01),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30830_e46152;
        var_tmf2_dn0 = assign30830_e46152_d_n0;
        var_tmf2_dn1 = assign30830_e46152_d_n1;
        var_tmf2_dn2 = assign30830_e46152_d_n2;
        var_tmf2_dn3 = assign30830_e46152_d_n3;
        var_tmf2_dn4 = assign30830_e46152_d_n4;
        var_tmf2_dn5 = assign30830_e46152_d_n5;
        var_tmf2_db0 = assign30830_e46152_d_b0;
        var_tmf2_db1 = assign30830_e46152_d_b1;
        var_tmf2_db2 = assign30830_e46152_d_b2;
        var_tmf2_db3 = assign30830_e46152_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30840_e46170, assign30840_e46170_d_n0, assign30840_e46170_d_n1, assign30840_e46170_d_n2, assign30840_e46170_d_n3, assign30840_e46170_d_n4, assign30840_e46170_d_n5, assign30840_e46170_d_b0, assign30840_e46170_d_b1, assign30840_e46170_d_b2, assign30840_e46170_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let (assign30840_e46168, assign30840_e46168_d_n0, assign30840_e46168_d_n1, assign30840_e46168_d_n2, assign30840_e46168_d_n3, assign30840_e46168_d_n4, assign30840_e46168_d_n5, assign30840_e46168_d_b0, assign30840_e46168_d_b1, assign30840_e46168_d_b2, assign30840_e46168_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30840_e46167: f64 = (-var_tmf2);
                (assign30840_e46167, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30840_e46168, assign30840_e46168_d_n0, assign30840_e46168_d_n1, assign30840_e46168_d_n2, assign30840_e46168_d_n3, assign30840_e46168_d_n4, assign30840_e46168_d_n5, assign30840_e46168_d_b0, assign30840_e46168_d_b1, assign30840_e46168_d_b2, assign30840_e46168_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30840_e46170;
        var_tmf2_dn0 = assign30840_e46170_d_n0;
        var_tmf2_dn1 = assign30840_e46170_d_n1;
        var_tmf2_dn2 = assign30840_e46170_d_n2;
        var_tmf2_dn3 = assign30840_e46170_d_n3;
        var_tmf2_dn4 = assign30840_e46170_d_n4;
        var_tmf2_dn5 = assign30840_e46170_d_n5;
        var_tmf2_db0 = assign30840_e46170_d_b0;
        var_tmf2_db1 = assign30840_e46170_d_b1;
        var_tmf2_db2 = assign30840_e46170_d_b2;
        var_tmf2_db3 = assign30840_e46170_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30850_e46187, assign30850_e46187_d_n0, assign30850_e46187_d_n1, assign30850_e46187_d_n2, assign30850_e46187_d_n3, assign30850_e46187_d_n4, assign30850_e46187_d_n5, assign30850_e46187_d_b0, assign30850_e46187_d_b1, assign30850_e46187_d_b2, assign30850_e46187_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30850_e46182: f64 = (var_tmf1 * var_tmf1);
        let assign30850_e46184: f64 = (assign30850_e46182 + var_tmf2);
        let assign30850_e46185: f64 = (assign30850_e46184).sqrt();
        (assign30850_e46185, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30850_e46185)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30850_e46185)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30850_e46185)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30850_e46185)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30850_e46185)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30850_e46185)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30850_e46185)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30850_e46185)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30850_e46185)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30850_e46185)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30850_e46187;
        var_tmf2_dn0 = assign30850_e46187_d_n0;
        var_tmf2_dn1 = assign30850_e46187_d_n1;
        var_tmf2_dn2 = assign30850_e46187_d_n2;
        var_tmf2_dn3 = assign30850_e46187_d_n3;
        var_tmf2_dn4 = assign30850_e46187_d_n4;
        var_tmf2_dn5 = assign30850_e46187_d_n5;
        var_tmf2_db0 = assign30850_e46187_d_b0;
        var_tmf2_db1 = assign30850_e46187_d_b1;
        var_tmf2_db2 = assign30850_e46187_d_b2;
        var_tmf2_db3 = assign30850_e46187_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30860_e46205, assign30860_e46205_d_n0, assign30860_e46205_d_n1, assign30860_e46205_d_n2, assign30860_e46205_d_n3, assign30860_e46205_d_n4, assign30860_e46205_d_n5, assign30860_e46205_d_b0, assign30860_e46205_d_b1, assign30860_e46205_d_b2, assign30860_e46205_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30860_e46201: f64 = (var_tmf1 / var_tmf2);
        let assign30860_e46202: f64 = (1.0 + assign30860_e46201);
        let assign30860_e46203: f64 = (0.5 * assign30860_e46202);
        (assign30860_e46203, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign30860_e46205;
        var_dfn_sl_dn0 = assign30860_e46205_d_n0;
        var_dfn_sl_dn1 = assign30860_e46205_d_n1;
        var_dfn_sl_dn2 = assign30860_e46205_d_n2;
        var_dfn_sl_dn3 = assign30860_e46205_d_n3;
        var_dfn_sl_dn4 = assign30860_e46205_d_n4;
        var_dfn_sl_dn5 = assign30860_e46205_d_n5;
        var_dfn_sl_db0 = assign30860_e46205_d_b0;
        var_dfn_sl_db1 = assign30860_e46205_d_b1;
        var_dfn_sl_db2 = assign30860_e46205_d_b2;
        var_dfn_sl_db3 = assign30860_e46205_d_b3;
        var_dfn_sl_rv = 0.0;
        var_dfn_sl_rdn0 = 0.0;
        var_dfn_sl_rdn1 = 0.0;
        var_dfn_sl_rdn2 = 0.0;
        var_dfn_sl_rdn3 = 0.0;
        var_dfn_sl_rdn4 = 0.0;
        var_dfn_sl_rdn5 = 0.0;
        var_dfn_sl_rdb0 = 0.0;
        var_dfn_sl_rdb1 = 0.0;
        var_dfn_sl_rdb2 = 0.0;
        var_dfn_sl_rdb3 = 0.0;

        let (assign30870_e46223, assign30870_e46223_d_n0, assign30870_e46223_d_n1, assign30870_e46223_d_n2, assign30870_e46223_d_n3, assign30870_e46223_d_n4, assign30870_e46223_d_n5, assign30870_e46223_d_b0, assign30870_e46223_d_b1, assign30870_e46223_d_b2, assign30870_e46223_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30870_e46219: f64 = (var_tmf1 + var_tmf2);
        let assign30870_e46220: f64 = (0.5 * assign30870_e46219);
        let assign30870_e46221: f64 = (var_nfasti_i + assign30870_e46220);
        (assign30870_e46221, (var_nfasti_i_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_nfasti_i_dn1 + (0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (var_nfasti_i_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_nfasti_i_dn3 + (0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (var_nfasti_i_dn4 + (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_nfasti_i_dn5 + (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_nfasti_i_db0 + (0.5 * (var_tmf1_db0 + var_tmf2_db0))), (var_nfasti_i_db1 + (0.5 * (var_tmf1_db1 + var_tmf2_db1))), (var_nfasti_i_db2 + (0.5 * (var_tmf1_db2 + var_tmf2_db2))), (var_nfasti_i_db3 + (0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign30870_e46223;
        var_nj1_dn0 = assign30870_e46223_d_n0;
        var_nj1_dn1 = assign30870_e46223_d_n1;
        var_nj1_dn2 = assign30870_e46223_d_n2;
        var_nj1_dn3 = assign30870_e46223_d_n3;
        var_nj1_dn4 = assign30870_e46223_d_n4;
        var_nj1_dn5 = assign30870_e46223_d_n5;
        var_nj1_db0 = assign30870_e46223_d_b0;
        var_nj1_db1 = assign30870_e46223_d_b1;
        var_nj1_db2 = assign30870_e46223_d_b2;
        var_nj1_db3 = assign30870_e46223_d_b3;
        var_nj1_rv = 0.0;
        var_nj1_rdn0 = 0.0;
        var_nj1_rdn1 = 0.0;
        var_nj1_rdn2 = 0.0;
        var_nj1_rdn3 = 0.0;
        var_nj1_rdn4 = 0.0;
        var_nj1_rdn5 = 0.0;
        var_nj1_rdb0 = 0.0;
        var_nj1_rdb1 = 0.0;
        var_nj1_rdb2 = 0.0;
        var_nj1_rdb3 = 0.0;

        let (assign30880_e46239, assign30880_e46239_d_n0, assign30880_e46239_d_n1, assign30880_e46239_d_n2, assign30880_e46239_d_n3, assign30880_e46239_d_n4, assign30880_e46239_d_n5, assign30880_e46239_d_b0, assign30880_e46239_d_b1, assign30880_e46239_d_b2, assign30880_e46239_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30880_e46235: f64 = (p.p85 - var_nj0);
        let assign30880_e46237: f64 = (assign30880_e46235 - 0.01);
        (assign30880_e46237, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30880_e46239;
        var_tmf1_dn0 = assign30880_e46239_d_n0;
        var_tmf1_dn1 = assign30880_e46239_d_n1;
        var_tmf1_dn2 = assign30880_e46239_d_n2;
        var_tmf1_dn3 = assign30880_e46239_d_n3;
        var_tmf1_dn4 = assign30880_e46239_d_n4;
        var_tmf1_dn5 = assign30880_e46239_d_n5;
        var_tmf1_db0 = assign30880_e46239_d_b0;
        var_tmf1_db1 = assign30880_e46239_d_b1;
        var_tmf1_db2 = assign30880_e46239_d_b2;
        var_tmf1_db3 = assign30880_e46239_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign30890_e46255, assign30890_e46255_d_n0, assign30890_e46255_d_n1, assign30890_e46255_d_n2, assign30890_e46255_d_n3, assign30890_e46255_d_n4, assign30890_e46255_d_n5, assign30890_e46255_d_b0, assign30890_e46255_d_b1, assign30890_e46255_d_b2, assign30890_e46255_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30890_e46251: f64 = (4.0 * p.p85);
        let assign30890_e46253: f64 = (assign30890_e46251 * 0.01);
        (assign30890_e46253, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30890_e46255;
        var_tmf2_dn0 = assign30890_e46255_d_n0;
        var_tmf2_dn1 = assign30890_e46255_d_n1;
        var_tmf2_dn2 = assign30890_e46255_d_n2;
        var_tmf2_dn3 = assign30890_e46255_d_n3;
        var_tmf2_dn4 = assign30890_e46255_d_n4;
        var_tmf2_dn5 = assign30890_e46255_d_n5;
        var_tmf2_db0 = assign30890_e46255_d_b0;
        var_tmf2_db1 = assign30890_e46255_d_b1;
        var_tmf2_db2 = assign30890_e46255_d_b2;
        var_tmf2_db3 = assign30890_e46255_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30900_e46273, assign30900_e46273_d_n0, assign30900_e46273_d_n1, assign30900_e46273_d_n2, assign30900_e46273_d_n3, assign30900_e46273_d_n4, assign30900_e46273_d_n5, assign30900_e46273_d_b0, assign30900_e46273_d_b1, assign30900_e46273_d_b2, assign30900_e46273_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let (assign30900_e46271, assign30900_e46271_d_n0, assign30900_e46271_d_n1, assign30900_e46271_d_n2, assign30900_e46271_d_n3, assign30900_e46271_d_n4, assign30900_e46271_d_n5, assign30900_e46271_d_b0, assign30900_e46271_d_b1, assign30900_e46271_d_b2, assign30900_e46271_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30900_e46270: f64 = (-var_tmf2);
                (assign30900_e46270, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30900_e46271, assign30900_e46271_d_n0, assign30900_e46271_d_n1, assign30900_e46271_d_n2, assign30900_e46271_d_n3, assign30900_e46271_d_n4, assign30900_e46271_d_n5, assign30900_e46271_d_b0, assign30900_e46271_d_b1, assign30900_e46271_d_b2, assign30900_e46271_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30900_e46273;
        var_tmf2_dn0 = assign30900_e46273_d_n0;
        var_tmf2_dn1 = assign30900_e46273_d_n1;
        var_tmf2_dn2 = assign30900_e46273_d_n2;
        var_tmf2_dn3 = assign30900_e46273_d_n3;
        var_tmf2_dn4 = assign30900_e46273_d_n4;
        var_tmf2_dn5 = assign30900_e46273_d_n5;
        var_tmf2_db0 = assign30900_e46273_d_b0;
        var_tmf2_db1 = assign30900_e46273_d_b1;
        var_tmf2_db2 = assign30900_e46273_d_b2;
        var_tmf2_db3 = assign30900_e46273_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30910_e46290, assign30910_e46290_d_n0, assign30910_e46290_d_n1, assign30910_e46290_d_n2, assign30910_e46290_d_n3, assign30910_e46290_d_n4, assign30910_e46290_d_n5, assign30910_e46290_d_b0, assign30910_e46290_d_b1, assign30910_e46290_d_b2, assign30910_e46290_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30910_e46285: f64 = (var_tmf1 * var_tmf1);
        let assign30910_e46287: f64 = (assign30910_e46285 + var_tmf2);
        let assign30910_e46288: f64 = (assign30910_e46287).sqrt();
        (assign30910_e46288, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30910_e46288)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30910_e46288)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30910_e46288)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30910_e46288)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30910_e46288)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30910_e46288)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30910_e46288)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30910_e46288)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30910_e46288)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30910_e46288)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30910_e46290;
        var_tmf2_dn0 = assign30910_e46290_d_n0;
        var_tmf2_dn1 = assign30910_e46290_d_n1;
        var_tmf2_dn2 = assign30910_e46290_d_n2;
        var_tmf2_dn3 = assign30910_e46290_d_n3;
        var_tmf2_dn4 = assign30910_e46290_d_n4;
        var_tmf2_dn5 = assign30910_e46290_d_n5;
        var_tmf2_db0 = assign30910_e46290_d_b0;
        var_tmf2_db1 = assign30910_e46290_d_b1;
        var_tmf2_db2 = assign30910_e46290_d_b2;
        var_tmf2_db3 = assign30910_e46290_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30920_e46308, assign30920_e46308_d_n0, assign30920_e46308_d_n1, assign30920_e46308_d_n2, assign30920_e46308_d_n3, assign30920_e46308_d_n4, assign30920_e46308_d_n5, assign30920_e46308_d_b0, assign30920_e46308_d_b1, assign30920_e46308_d_b2, assign30920_e46308_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30920_e46304: f64 = (var_tmf1 + var_tmf2);
        let assign30920_e46305: f64 = (0.5 * assign30920_e46304);
        let assign30920_e46306: f64 = (p.p85 - assign30920_e46305);
        (assign30920_e46306, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30920_e46308;
        var_nj0_dn0 = assign30920_e46308_d_n0;
        var_nj0_dn1 = assign30920_e46308_d_n1;
        var_nj0_dn2 = assign30920_e46308_d_n2;
        var_nj0_dn3 = assign30920_e46308_d_n3;
        var_nj0_dn4 = assign30920_e46308_d_n4;
        var_nj0_dn5 = assign30920_e46308_d_n5;
        var_nj0_db0 = assign30920_e46308_d_b0;
        var_nj0_db1 = assign30920_e46308_d_b1;
        var_nj0_db2 = assign30920_e46308_d_b2;
        var_nj0_db3 = assign30920_e46308_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign30930_e46324, assign30930_e46324_d_n0, assign30930_e46324_d_n1, assign30930_e46324_d_n2, assign30930_e46324_d_n3, assign30930_e46324_d_n4, assign30930_e46324_d_n5, assign30930_e46324_d_b0, assign30930_e46324_d_b1, assign30930_e46324_d_b2, assign30930_e46324_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30930_e46320: f64 = (var_nj0 - var_nfasti_i);
        let assign30930_e46322: f64 = (assign30930_e46320 - 0.01);
        (assign30930_e46322, (var_nj0_dn0 - var_nfasti_i_dn0), (var_nj0_dn1 - var_nfasti_i_dn1), (var_nj0_dn2 - var_nfasti_i_dn2), (var_nj0_dn3 - var_nfasti_i_dn3), (var_nj0_dn4 - var_nfasti_i_dn4), (var_nj0_dn5 - var_nfasti_i_dn5), (var_nj0_db0 - var_nfasti_i_db0), (var_nj0_db1 - var_nfasti_i_db1), (var_nj0_db2 - var_nfasti_i_db2), (var_nj0_db3 - var_nfasti_i_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30930_e46324;
        var_tmf1_dn0 = assign30930_e46324_d_n0;
        var_tmf1_dn1 = assign30930_e46324_d_n1;
        var_tmf1_dn2 = assign30930_e46324_d_n2;
        var_tmf1_dn3 = assign30930_e46324_d_n3;
        var_tmf1_dn4 = assign30930_e46324_d_n4;
        var_tmf1_dn5 = assign30930_e46324_d_n5;
        var_tmf1_db0 = assign30930_e46324_d_b0;
        var_tmf1_db1 = assign30930_e46324_d_b1;
        var_tmf1_db2 = assign30930_e46324_d_b2;
        var_tmf1_db3 = assign30930_e46324_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign30940_e46340, assign30940_e46340_d_n0, assign30940_e46340_d_n1, assign30940_e46340_d_n2, assign30940_e46340_d_n3, assign30940_e46340_d_n4, assign30940_e46340_d_n5, assign30940_e46340_d_b0, assign30940_e46340_d_b1, assign30940_e46340_d_b2, assign30940_e46340_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30940_e46336: f64 = (4.0 * var_nfasti_i);
        let assign30940_e46338: f64 = (assign30940_e46336 * 0.01);
        (assign30940_e46338, ((4.0 * var_nfasti_i_dn0) * 0.01), ((4.0 * var_nfasti_i_dn1) * 0.01), ((4.0 * var_nfasti_i_dn2) * 0.01), ((4.0 * var_nfasti_i_dn3) * 0.01), ((4.0 * var_nfasti_i_dn4) * 0.01), ((4.0 * var_nfasti_i_dn5) * 0.01), ((4.0 * var_nfasti_i_db0) * 0.01), ((4.0 * var_nfasti_i_db1) * 0.01), ((4.0 * var_nfasti_i_db2) * 0.01), ((4.0 * var_nfasti_i_db3) * 0.01),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30940_e46340;
        var_tmf2_dn0 = assign30940_e46340_d_n0;
        var_tmf2_dn1 = assign30940_e46340_d_n1;
        var_tmf2_dn2 = assign30940_e46340_d_n2;
        var_tmf2_dn3 = assign30940_e46340_d_n3;
        var_tmf2_dn4 = assign30940_e46340_d_n4;
        var_tmf2_dn5 = assign30940_e46340_d_n5;
        var_tmf2_db0 = assign30940_e46340_d_b0;
        var_tmf2_db1 = assign30940_e46340_d_b1;
        var_tmf2_db2 = assign30940_e46340_d_b2;
        var_tmf2_db3 = assign30940_e46340_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_db0_slot = var_dfn_sl_db0;
        *var_dfn_sl_db1_slot = var_dfn_sl_db1;
        *var_dfn_sl_db2_slot = var_dfn_sl_db2;
        *var_dfn_sl_db3_slot = var_dfn_sl_db3;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn1_slot = var_dfn_sl_dn1;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_dn3_slot = var_dfn_sl_dn3;
        *var_dfn_sl_dn4_slot = var_dfn_sl_dn4;
        *var_dfn_sl_dn5_slot = var_dfn_sl_dn5;
        *var_dfn_sl_rdb0_slot = var_dfn_sl_rdb0;
        *var_dfn_sl_rdb1_slot = var_dfn_sl_rdb1;
        *var_dfn_sl_rdb2_slot = var_dfn_sl_rdb2;
        *var_dfn_sl_rdb3_slot = var_dfn_sl_rdb3;
        *var_dfn_sl_rdn0_slot = var_dfn_sl_rdn0;
        *var_dfn_sl_rdn1_slot = var_dfn_sl_rdn1;
        *var_dfn_sl_rdn2_slot = var_dfn_sl_rdn2;
        *var_dfn_sl_rdn3_slot = var_dfn_sl_rdn3;
        *var_dfn_sl_rdn4_slot = var_dfn_sl_rdn4;
        *var_dfn_sl_rdn5_slot = var_dfn_sl_rdn5;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_db0_slot = var_dfn_su_db0;
        *var_dfn_su_db1_slot = var_dfn_su_db1;
        *var_dfn_su_db2_slot = var_dfn_su_db2;
        *var_dfn_su_db3_slot = var_dfn_su_db3;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn1_slot = var_dfn_su_dn1;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_dn3_slot = var_dfn_su_dn3;
        *var_dfn_su_dn4_slot = var_dfn_su_dn4;
        *var_dfn_su_dn5_slot = var_dfn_su_dn5;
        *var_dfn_su_rdb0_slot = var_dfn_su_rdb0;
        *var_dfn_su_rdb1_slot = var_dfn_su_rdb1;
        *var_dfn_su_rdb2_slot = var_dfn_su_rdb2;
        *var_dfn_su_rdb3_slot = var_dfn_su_rdb3;
        *var_dfn_su_rdn0_slot = var_dfn_su_rdn0;
        *var_dfn_su_rdn1_slot = var_dfn_su_rdn1;
        *var_dfn_su_rdn2_slot = var_dfn_su_rdn2;
        *var_dfn_su_rdn3_slot = var_dfn_su_rdn3;
        *var_dfn_su_rdn4_slot = var_dfn_su_rdn4;
        *var_dfn_su_rdn5_slot = var_dfn_su_rdn5;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj0_rdb0_slot = var_nj0_rdb0;
        *var_nj0_rdb1_slot = var_nj0_rdb1;
        *var_nj0_rdb2_slot = var_nj0_rdb2;
        *var_nj0_rdb3_slot = var_nj0_rdb3;
        *var_nj0_rdn0_slot = var_nj0_rdn0;
        *var_nj0_rdn1_slot = var_nj0_rdn1;
        *var_nj0_rdn2_slot = var_nj0_rdn2;
        *var_nj0_rdn3_slot = var_nj0_rdn3;
        *var_nj0_rdn4_slot = var_nj0_rdn4;
        *var_nj0_rdn5_slot = var_nj0_rdn5;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nj1_rdb0_slot = var_nj1_rdb0;
        *var_nj1_rdb1_slot = var_nj1_rdb1;
        *var_nj1_rdb2_slot = var_nj1_rdb2;
        *var_nj1_rdb3_slot = var_nj1_rdb3;
        *var_nj1_rdn0_slot = var_nj1_rdn0;
        *var_nj1_rdn1_slot = var_nj1_rdn1;
        *var_nj1_rdn2_slot = var_nj1_rdn2;
        *var_nj1_rdn3_slot = var_nj1_rdn3;
        *var_nj1_rdn4_slot = var_nj1_rdn4;
        *var_nj1_rdn5_slot = var_nj1_rdn5;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_nja11_rdb0_slot = var_nja11_rdb0;
        *var_nja11_rdb1_slot = var_nja11_rdb1;
        *var_nja11_rdb2_slot = var_nja11_rdb2;
        *var_nja11_rdb3_slot = var_nja11_rdb3;
        *var_nja11_rdn0_slot = var_nja11_rdn0;
        *var_nja11_rdn1_slot = var_nja11_rdn1;
        *var_nja11_rdn2_slot = var_nja11_rdn2;
        *var_nja11_rdn3_slot = var_nja11_rdn3;
        *var_nja11_rdn4_slot = var_nja11_rdn4;
        *var_nja11_rdn5_slot = var_nja11_rdn5;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_rdb0_slot = var_tmf1_rdb0;
        *var_tmf1_rdb1_slot = var_tmf1_rdb1;
        *var_tmf1_rdb2_slot = var_tmf1_rdb2;
        *var_tmf1_rdb3_slot = var_tmf1_rdb3;
        *var_tmf1_rdn0_slot = var_tmf1_rdn0;
        *var_tmf1_rdn1_slot = var_tmf1_rdn1;
        *var_tmf1_rdn2_slot = var_tmf1_rdn2;
        *var_tmf1_rdn3_slot = var_tmf1_rdn3;
        *var_tmf1_rdn4_slot = var_tmf1_rdn4;
        *var_tmf1_rdn5_slot = var_tmf1_rdn5;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_rdb0_slot = var_tmf2_rdb0;
        *var_tmf2_rdb1_slot = var_tmf2_rdb1;
        *var_tmf2_rdb2_slot = var_tmf2_rdb2;
        *var_tmf2_rdb3_slot = var_tmf2_rdb3;
        *var_tmf2_rdn0_slot = var_tmf2_rdn0;
        *var_tmf2_rdn1_slot = var_tmf2_rdn1;
        *var_tmf2_rdn2_slot = var_tmf2_rdn2;
        *var_tmf2_rdn3_slot = var_tmf2_rdn3;
        *var_tmf2_rdn4_slot = var_tmf2_rdn4;
        *var_tmf2_rdn5_slot = var_tmf2_rdn5;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_81(
        p: &Parameters,
        var_dfn_sl: f64,
        var_dfn_sl_db0: f64,
        var_dfn_sl_db1: f64,
        var_dfn_sl_db2: f64,
        var_dfn_sl_db3: f64,
        var_dfn_sl_dn0: f64,
        var_dfn_sl_dn1: f64,
        var_dfn_sl_dn2: f64,
        var_dfn_sl_dn3: f64,
        var_dfn_sl_dn4: f64,
        var_dfn_sl_dn5: f64,
        var_dfn_su: f64,
        var_dfn_su_db0: f64,
        var_dfn_su_db1: f64,
        var_dfn_su_db2: f64,
        var_dfn_su_db3: f64,
        var_dfn_su_dn0: f64,
        var_dfn_su_dn1: f64,
        var_dfn_su_dn2: f64,
        var_dfn_su_dn3: f64,
        var_dfn_su_dn4: f64,
        var_dfn_su_dn5: f64,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard495: f64,
        var_ndigat_i: f64,
        var_ndigat_i_db0: f64,
        var_ndigat_i_db1: f64,
        var_ndigat_i_db2: f64,
        var_ndigat_i_db3: f64,
        var_ndigat_i_dn0: f64,
        var_ndigat_i_dn1: f64,
        var_ndigat_i_dn2: f64,
        var_ndigat_i_dn3: f64,
        var_ndigat_i_dn4: f64,
        var_ndigat_i_dn5: f64,
        var_nfagat_i: f64,
        var_nfagat_i_db0: f64,
        var_nfagat_i_db1: f64,
        var_nfagat_i_db2: f64,
        var_nfagat_i_db3: f64,
        var_nfagat_i_dn0: f64,
        var_nfagat_i_dn1: f64,
        var_nfagat_i_dn2: f64,
        var_nfagat_i_dn3: f64,
        var_nfagat_i_dn4: f64,
        var_nfagat_i_dn5: f64,
        var_nfasti_i: f64,
        var_nfasti_i_db0: f64,
        var_nfasti_i_db1: f64,
        var_nfasti_i_db2: f64,
        var_nfasti_i_db3: f64,
        var_nfasti_i_dn0: f64,
        var_nfasti_i_dn1: f64,
        var_nfasti_i_dn2: f64,
        var_nfasti_i_dn3: f64,
        var_nfasti_i_dn4: f64,
        var_nfasti_i_dn5: f64,
        var_nin: f64,
        var_nin_db0: f64,
        var_nin_db1: f64,
        var_nin_db2: f64,
        var_nin_db3: f64,
        var_nin_dn0: f64,
        var_nin_dn1: f64,
        var_nin_dn2: f64,
        var_nin_dn3: f64,
        var_nin_dn4: f64,
        var_nin_dn5: f64,
        var_phitdinv: f64,
        var_phitdinv_db0: f64,
        var_phitdinv_db1: f64,
        var_phitdinv_db2: f64,
        var_phitdinv_db3: f64,
        var_phitdinv_dn0: f64,
        var_phitdinv_dn1: f64,
        var_phitdinv_dn2: f64,
        var_phitdinv_dn3: f64,
        var_phitdinv_dn4: f64,
        var_phitdinv_dn5: f64,
        var_vmax: f64,
        var_vmax_db0: f64,
        var_vmax_db1: f64,
        var_vmax_db2: f64,
        var_vmax_db3: f64,
        var_vmax_dn0: f64,
        var_vmax_dn1: f64,
        var_vmax_dn2: f64,
        var_vmax_dn3: f64,
        var_vmax_dn4: f64,
        var_vmax_dn5: f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_db0_slot: &mut f64,
        var_dnj1_dv_db1_slot: &mut f64,
        var_dnj1_dv_db2_slot: &mut f64,
        var_dnj1_dv_db3_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn1_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_dn3_slot: &mut f64,
        var_dnj1_dv_dn4_slot: &mut f64,
        var_dnj1_dv_dn5_slot: &mut f64,
        var_dnj1_dv_rdb0_slot: &mut f64,
        var_dnj1_dv_rdb1_slot: &mut f64,
        var_dnj1_dv_rdb2_slot: &mut f64,
        var_dnj1_dv_rdb3_slot: &mut f64,
        var_dnj1_dv_rdn0_slot: &mut f64,
        var_dnj1_dv_rdn1_slot: &mut f64,
        var_dnj1_dv_rdn2_slot: &mut f64,
        var_dnj1_dv_rdn3_slot: &mut f64,
        var_dnj1_dv_rdn4_slot: &mut f64,
        var_dnj1_dv_rdn5_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_db0_slot: &mut f64,
        var_dvmax_over_phitd_dv_db1_slot: &mut f64,
        var_dvmax_over_phitd_dv_db2_slot: &mut f64,
        var_dvmax_over_phitd_dv_db3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn1_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn4_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn5_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdb0_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdb1_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdb2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdb3_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn1_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn3_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn4_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn5_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_guard498_slot: &mut f64,
        var_guard498_db0_slot: &mut f64,
        var_guard498_db1_slot: &mut f64,
        var_guard498_db2_slot: &mut f64,
        var_guard498_db3_slot: &mut f64,
        var_guard498_dn0_slot: &mut f64,
        var_guard498_dn1_slot: &mut f64,
        var_guard498_dn2_slot: &mut f64,
        var_guard498_dn3_slot: &mut f64,
        var_guard498_dn4_slot: &mut f64,
        var_guard498_dn5_slot: &mut f64,
        var_guard498_rdb0_slot: &mut f64,
        var_guard498_rdb1_slot: &mut f64,
        var_guard498_rdb2_slot: &mut f64,
        var_guard498_rdb3_slot: &mut f64,
        var_guard498_rdn0_slot: &mut f64,
        var_guard498_rdn1_slot: &mut f64,
        var_guard498_rdn2_slot: &mut f64,
        var_guard498_rdn3_slot: &mut f64,
        var_guard498_rdn4_slot: &mut f64,
        var_guard498_rdn5_slot: &mut f64,
        var_guard498_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj0_rdb0_slot: &mut f64,
        var_nj0_rdb1_slot: &mut f64,
        var_nj0_rdb2_slot: &mut f64,
        var_nj0_rdb3_slot: &mut f64,
        var_nj0_rdn0_slot: &mut f64,
        var_nj0_rdn1_slot: &mut f64,
        var_nj0_rdn2_slot: &mut f64,
        var_nj0_rdn3_slot: &mut f64,
        var_nj0_rdn4_slot: &mut f64,
        var_nj0_rdn5_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nj1_rdb0_slot: &mut f64,
        var_nj1_rdb1_slot: &mut f64,
        var_nj1_rdb2_slot: &mut f64,
        var_nj1_rdb3_slot: &mut f64,
        var_nj1_rdn0_slot: &mut f64,
        var_nj1_rdn1_slot: &mut f64,
        var_nj1_rdn2_slot: &mut f64,
        var_nj1_rdn3_slot: &mut f64,
        var_nj1_rdn4_slot: &mut f64,
        var_nj1_rdn5_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_nja10_rdb0_slot: &mut f64,
        var_nja10_rdb1_slot: &mut f64,
        var_nja10_rdb2_slot: &mut f64,
        var_nja10_rdb3_slot: &mut f64,
        var_nja10_rdn0_slot: &mut f64,
        var_nja10_rdn1_slot: &mut f64,
        var_nja10_rdn2_slot: &mut f64,
        var_nja10_rdn3_slot: &mut f64,
        var_nja10_rdn4_slot: &mut f64,
        var_nja10_rdn5_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_db0_slot: &mut f64,
        var_pnn0_db1_slot: &mut f64,
        var_pnn0_db2_slot: &mut f64,
        var_pnn0_db3_slot: &mut f64,
        var_pnn0_dn0_slot: &mut f64,
        var_pnn0_dn1_slot: &mut f64,
        var_pnn0_dn2_slot: &mut f64,
        var_pnn0_dn3_slot: &mut f64,
        var_pnn0_dn4_slot: &mut f64,
        var_pnn0_dn5_slot: &mut f64,
        var_pnn0_rdb0_slot: &mut f64,
        var_pnn0_rdb1_slot: &mut f64,
        var_pnn0_rdb2_slot: &mut f64,
        var_pnn0_rdb3_slot: &mut f64,
        var_pnn0_rdn0_slot: &mut f64,
        var_pnn0_rdn1_slot: &mut f64,
        var_pnn0_rdn2_slot: &mut f64,
        var_pnn0_rdn3_slot: &mut f64,
        var_pnn0_rdn4_slot: &mut f64,
        var_pnn0_rdn5_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_rdb0_slot: &mut f64,
        var_tmf1_rdb1_slot: &mut f64,
        var_tmf1_rdb2_slot: &mut f64,
        var_tmf1_rdb3_slot: &mut f64,
        var_tmf1_rdn0_slot: &mut f64,
        var_tmf1_rdn1_slot: &mut f64,
        var_tmf1_rdn2_slot: &mut f64,
        var_tmf1_rdn3_slot: &mut f64,
        var_tmf1_rdn4_slot: &mut f64,
        var_tmf1_rdn5_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_rdb0_slot: &mut f64,
        var_tmf2_rdb1_slot: &mut f64,
        var_tmf2_rdb2_slot: &mut f64,
        var_tmf2_rdb3_slot: &mut f64,
        var_tmf2_rdn0_slot: &mut f64,
        var_tmf2_rdn1_slot: &mut f64,
        var_tmf2_rdn2_slot: &mut f64,
        var_tmf2_rdn3_slot: &mut f64,
        var_tmf2_rdn4_slot: &mut f64,
        var_tmf2_rdn5_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_db0_slot: &mut f64,
        var_vha1_db1_slot: &mut f64,
        var_vha1_db2_slot: &mut f64,
        var_vha1_db3_slot: &mut f64,
        var_vha1_dn0_slot: &mut f64,
        var_vha1_dn1_slot: &mut f64,
        var_vha1_dn2_slot: &mut f64,
        var_vha1_dn3_slot: &mut f64,
        var_vha1_dn4_slot: &mut f64,
        var_vha1_dn5_slot: &mut f64,
        var_vha1_rdb0_slot: &mut f64,
        var_vha1_rdb1_slot: &mut f64,
        var_vha1_rdb2_slot: &mut f64,
        var_vha1_rdb3_slot: &mut f64,
        var_vha1_rdn0_slot: &mut f64,
        var_vha1_rdn1_slot: &mut f64,
        var_vha1_rdn2_slot: &mut f64,
        var_vha1_rdn3_slot: &mut f64,
        var_vha1_rdn4_slot: &mut f64,
        var_vha1_rdn5_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_db0: f64 = *var_dnj1_dv_db0_slot;
        let mut var_dnj1_dv_db1: f64 = *var_dnj1_dv_db1_slot;
        let mut var_dnj1_dv_db2: f64 = *var_dnj1_dv_db2_slot;
        let mut var_dnj1_dv_db3: f64 = *var_dnj1_dv_db3_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn1: f64 = *var_dnj1_dv_dn1_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_dn3: f64 = *var_dnj1_dv_dn3_slot;
        let mut var_dnj1_dv_dn4: f64 = *var_dnj1_dv_dn4_slot;
        let mut var_dnj1_dv_dn5: f64 = *var_dnj1_dv_dn5_slot;
        let mut var_dnj1_dv_rdb0: f64 = *var_dnj1_dv_rdb0_slot;
        let mut var_dnj1_dv_rdb1: f64 = *var_dnj1_dv_rdb1_slot;
        let mut var_dnj1_dv_rdb2: f64 = *var_dnj1_dv_rdb2_slot;
        let mut var_dnj1_dv_rdb3: f64 = *var_dnj1_dv_rdb3_slot;
        let mut var_dnj1_dv_rdn0: f64 = *var_dnj1_dv_rdn0_slot;
        let mut var_dnj1_dv_rdn1: f64 = *var_dnj1_dv_rdn1_slot;
        let mut var_dnj1_dv_rdn2: f64 = *var_dnj1_dv_rdn2_slot;
        let mut var_dnj1_dv_rdn3: f64 = *var_dnj1_dv_rdn3_slot;
        let mut var_dnj1_dv_rdn4: f64 = *var_dnj1_dv_rdn4_slot;
        let mut var_dnj1_dv_rdn5: f64 = *var_dnj1_dv_rdn5_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_db0: f64 = *var_dvmax_over_phitd_dv_db0_slot;
        let mut var_dvmax_over_phitd_dv_db1: f64 = *var_dvmax_over_phitd_dv_db1_slot;
        let mut var_dvmax_over_phitd_dv_db2: f64 = *var_dvmax_over_phitd_dv_db2_slot;
        let mut var_dvmax_over_phitd_dv_db3: f64 = *var_dvmax_over_phitd_dv_db3_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn1: f64 = *var_dvmax_over_phitd_dv_dn1_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_dn3: f64 = *var_dvmax_over_phitd_dv_dn3_slot;
        let mut var_dvmax_over_phitd_dv_dn4: f64 = *var_dvmax_over_phitd_dv_dn4_slot;
        let mut var_dvmax_over_phitd_dv_dn5: f64 = *var_dvmax_over_phitd_dv_dn5_slot;
        let mut var_dvmax_over_phitd_dv_rdb0: f64 = *var_dvmax_over_phitd_dv_rdb0_slot;
        let mut var_dvmax_over_phitd_dv_rdb1: f64 = *var_dvmax_over_phitd_dv_rdb1_slot;
        let mut var_dvmax_over_phitd_dv_rdb2: f64 = *var_dvmax_over_phitd_dv_rdb2_slot;
        let mut var_dvmax_over_phitd_dv_rdb3: f64 = *var_dvmax_over_phitd_dv_rdb3_slot;
        let mut var_dvmax_over_phitd_dv_rdn0: f64 = *var_dvmax_over_phitd_dv_rdn0_slot;
        let mut var_dvmax_over_phitd_dv_rdn1: f64 = *var_dvmax_over_phitd_dv_rdn1_slot;
        let mut var_dvmax_over_phitd_dv_rdn2: f64 = *var_dvmax_over_phitd_dv_rdn2_slot;
        let mut var_dvmax_over_phitd_dv_rdn3: f64 = *var_dvmax_over_phitd_dv_rdn3_slot;
        let mut var_dvmax_over_phitd_dv_rdn4: f64 = *var_dvmax_over_phitd_dv_rdn4_slot;
        let mut var_dvmax_over_phitd_dv_rdn5: f64 = *var_dvmax_over_phitd_dv_rdn5_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_guard498: f64 = *var_guard498_slot;
        let mut var_guard498_db0: f64 = *var_guard498_db0_slot;
        let mut var_guard498_db1: f64 = *var_guard498_db1_slot;
        let mut var_guard498_db2: f64 = *var_guard498_db2_slot;
        let mut var_guard498_db3: f64 = *var_guard498_db3_slot;
        let mut var_guard498_dn0: f64 = *var_guard498_dn0_slot;
        let mut var_guard498_dn1: f64 = *var_guard498_dn1_slot;
        let mut var_guard498_dn2: f64 = *var_guard498_dn2_slot;
        let mut var_guard498_dn3: f64 = *var_guard498_dn3_slot;
        let mut var_guard498_dn4: f64 = *var_guard498_dn4_slot;
        let mut var_guard498_dn5: f64 = *var_guard498_dn5_slot;
        let mut var_guard498_rdb0: f64 = *var_guard498_rdb0_slot;
        let mut var_guard498_rdb1: f64 = *var_guard498_rdb1_slot;
        let mut var_guard498_rdb2: f64 = *var_guard498_rdb2_slot;
        let mut var_guard498_rdb3: f64 = *var_guard498_rdb3_slot;
        let mut var_guard498_rdn0: f64 = *var_guard498_rdn0_slot;
        let mut var_guard498_rdn1: f64 = *var_guard498_rdn1_slot;
        let mut var_guard498_rdn2: f64 = *var_guard498_rdn2_slot;
        let mut var_guard498_rdn3: f64 = *var_guard498_rdn3_slot;
        let mut var_guard498_rdn4: f64 = *var_guard498_rdn4_slot;
        let mut var_guard498_rdn5: f64 = *var_guard498_rdn5_slot;
        let mut var_guard498_rv: f64 = *var_guard498_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj0_rdb0: f64 = *var_nj0_rdb0_slot;
        let mut var_nj0_rdb1: f64 = *var_nj0_rdb1_slot;
        let mut var_nj0_rdb2: f64 = *var_nj0_rdb2_slot;
        let mut var_nj0_rdb3: f64 = *var_nj0_rdb3_slot;
        let mut var_nj0_rdn0: f64 = *var_nj0_rdn0_slot;
        let mut var_nj0_rdn1: f64 = *var_nj0_rdn1_slot;
        let mut var_nj0_rdn2: f64 = *var_nj0_rdn2_slot;
        let mut var_nj0_rdn3: f64 = *var_nj0_rdn3_slot;
        let mut var_nj0_rdn4: f64 = *var_nj0_rdn4_slot;
        let mut var_nj0_rdn5: f64 = *var_nj0_rdn5_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nj1_rdb0: f64 = *var_nj1_rdb0_slot;
        let mut var_nj1_rdb1: f64 = *var_nj1_rdb1_slot;
        let mut var_nj1_rdb2: f64 = *var_nj1_rdb2_slot;
        let mut var_nj1_rdb3: f64 = *var_nj1_rdb3_slot;
        let mut var_nj1_rdn0: f64 = *var_nj1_rdn0_slot;
        let mut var_nj1_rdn1: f64 = *var_nj1_rdn1_slot;
        let mut var_nj1_rdn2: f64 = *var_nj1_rdn2_slot;
        let mut var_nj1_rdn3: f64 = *var_nj1_rdn3_slot;
        let mut var_nj1_rdn4: f64 = *var_nj1_rdn4_slot;
        let mut var_nj1_rdn5: f64 = *var_nj1_rdn5_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_nja10_rdb0: f64 = *var_nja10_rdb0_slot;
        let mut var_nja10_rdb1: f64 = *var_nja10_rdb1_slot;
        let mut var_nja10_rdb2: f64 = *var_nja10_rdb2_slot;
        let mut var_nja10_rdb3: f64 = *var_nja10_rdb3_slot;
        let mut var_nja10_rdn0: f64 = *var_nja10_rdn0_slot;
        let mut var_nja10_rdn1: f64 = *var_nja10_rdn1_slot;
        let mut var_nja10_rdn2: f64 = *var_nja10_rdn2_slot;
        let mut var_nja10_rdn3: f64 = *var_nja10_rdn3_slot;
        let mut var_nja10_rdn4: f64 = *var_nja10_rdn4_slot;
        let mut var_nja10_rdn5: f64 = *var_nja10_rdn5_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_db0: f64 = *var_pnn0_db0_slot;
        let mut var_pnn0_db1: f64 = *var_pnn0_db1_slot;
        let mut var_pnn0_db2: f64 = *var_pnn0_db2_slot;
        let mut var_pnn0_db3: f64 = *var_pnn0_db3_slot;
        let mut var_pnn0_dn0: f64 = *var_pnn0_dn0_slot;
        let mut var_pnn0_dn1: f64 = *var_pnn0_dn1_slot;
        let mut var_pnn0_dn2: f64 = *var_pnn0_dn2_slot;
        let mut var_pnn0_dn3: f64 = *var_pnn0_dn3_slot;
        let mut var_pnn0_dn4: f64 = *var_pnn0_dn4_slot;
        let mut var_pnn0_dn5: f64 = *var_pnn0_dn5_slot;
        let mut var_pnn0_rdb0: f64 = *var_pnn0_rdb0_slot;
        let mut var_pnn0_rdb1: f64 = *var_pnn0_rdb1_slot;
        let mut var_pnn0_rdb2: f64 = *var_pnn0_rdb2_slot;
        let mut var_pnn0_rdb3: f64 = *var_pnn0_rdb3_slot;
        let mut var_pnn0_rdn0: f64 = *var_pnn0_rdn0_slot;
        let mut var_pnn0_rdn1: f64 = *var_pnn0_rdn1_slot;
        let mut var_pnn0_rdn2: f64 = *var_pnn0_rdn2_slot;
        let mut var_pnn0_rdn3: f64 = *var_pnn0_rdn3_slot;
        let mut var_pnn0_rdn4: f64 = *var_pnn0_rdn4_slot;
        let mut var_pnn0_rdn5: f64 = *var_pnn0_rdn5_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_rdb0: f64 = *var_tmf1_rdb0_slot;
        let mut var_tmf1_rdb1: f64 = *var_tmf1_rdb1_slot;
        let mut var_tmf1_rdb2: f64 = *var_tmf1_rdb2_slot;
        let mut var_tmf1_rdb3: f64 = *var_tmf1_rdb3_slot;
        let mut var_tmf1_rdn0: f64 = *var_tmf1_rdn0_slot;
        let mut var_tmf1_rdn1: f64 = *var_tmf1_rdn1_slot;
        let mut var_tmf1_rdn2: f64 = *var_tmf1_rdn2_slot;
        let mut var_tmf1_rdn3: f64 = *var_tmf1_rdn3_slot;
        let mut var_tmf1_rdn4: f64 = *var_tmf1_rdn4_slot;
        let mut var_tmf1_rdn5: f64 = *var_tmf1_rdn5_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_rdb0: f64 = *var_tmf2_rdb0_slot;
        let mut var_tmf2_rdb1: f64 = *var_tmf2_rdb1_slot;
        let mut var_tmf2_rdb2: f64 = *var_tmf2_rdb2_slot;
        let mut var_tmf2_rdb3: f64 = *var_tmf2_rdb3_slot;
        let mut var_tmf2_rdn0: f64 = *var_tmf2_rdn0_slot;
        let mut var_tmf2_rdn1: f64 = *var_tmf2_rdn1_slot;
        let mut var_tmf2_rdn2: f64 = *var_tmf2_rdn2_slot;
        let mut var_tmf2_rdn3: f64 = *var_tmf2_rdn3_slot;
        let mut var_tmf2_rdn4: f64 = *var_tmf2_rdn4_slot;
        let mut var_tmf2_rdn5: f64 = *var_tmf2_rdn5_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_db0: f64 = *var_vha1_db0_slot;
        let mut var_vha1_db1: f64 = *var_vha1_db1_slot;
        let mut var_vha1_db2: f64 = *var_vha1_db2_slot;
        let mut var_vha1_db3: f64 = *var_vha1_db3_slot;
        let mut var_vha1_dn0: f64 = *var_vha1_dn0_slot;
        let mut var_vha1_dn1: f64 = *var_vha1_dn1_slot;
        let mut var_vha1_dn2: f64 = *var_vha1_dn2_slot;
        let mut var_vha1_dn3: f64 = *var_vha1_dn3_slot;
        let mut var_vha1_dn4: f64 = *var_vha1_dn4_slot;
        let mut var_vha1_dn5: f64 = *var_vha1_dn5_slot;
        let mut var_vha1_rdb0: f64 = *var_vha1_rdb0_slot;
        let mut var_vha1_rdb1: f64 = *var_vha1_rdb1_slot;
        let mut var_vha1_rdb2: f64 = *var_vha1_rdb2_slot;
        let mut var_vha1_rdb3: f64 = *var_vha1_rdb3_slot;
        let mut var_vha1_rdn0: f64 = *var_vha1_rdn0_slot;
        let mut var_vha1_rdn1: f64 = *var_vha1_rdn1_slot;
        let mut var_vha1_rdn2: f64 = *var_vha1_rdn2_slot;
        let mut var_vha1_rdn3: f64 = *var_vha1_rdn3_slot;
        let mut var_vha1_rdn4: f64 = *var_vha1_rdn4_slot;
        let mut var_vha1_rdn5: f64 = *var_vha1_rdn5_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign30950_e46358, assign30950_e46358_d_n0, assign30950_e46358_d_n1, assign30950_e46358_d_n2, assign30950_e46358_d_n3, assign30950_e46358_d_n4, assign30950_e46358_d_n5, assign30950_e46358_d_b0, assign30950_e46358_d_b1, assign30950_e46358_d_b2, assign30950_e46358_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let (assign30950_e46356, assign30950_e46356_d_n0, assign30950_e46356_d_n1, assign30950_e46356_d_n2, assign30950_e46356_d_n3, assign30950_e46356_d_n4, assign30950_e46356_d_n5, assign30950_e46356_d_b0, assign30950_e46356_d_b1, assign30950_e46356_d_b2, assign30950_e46356_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30950_e46355: f64 = (-var_tmf2);
                (assign30950_e46355, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30950_e46356, assign30950_e46356_d_n0, assign30950_e46356_d_n1, assign30950_e46356_d_n2, assign30950_e46356_d_n3, assign30950_e46356_d_n4, assign30950_e46356_d_n5, assign30950_e46356_d_b0, assign30950_e46356_d_b1, assign30950_e46356_d_b2, assign30950_e46356_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30950_e46358;
        var_tmf2_dn0 = assign30950_e46358_d_n0;
        var_tmf2_dn1 = assign30950_e46358_d_n1;
        var_tmf2_dn2 = assign30950_e46358_d_n2;
        var_tmf2_dn3 = assign30950_e46358_d_n3;
        var_tmf2_dn4 = assign30950_e46358_d_n4;
        var_tmf2_dn5 = assign30950_e46358_d_n5;
        var_tmf2_db0 = assign30950_e46358_d_b0;
        var_tmf2_db1 = assign30950_e46358_d_b1;
        var_tmf2_db2 = assign30950_e46358_d_b2;
        var_tmf2_db3 = assign30950_e46358_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30960_e46375, assign30960_e46375_d_n0, assign30960_e46375_d_n1, assign30960_e46375_d_n2, assign30960_e46375_d_n3, assign30960_e46375_d_n4, assign30960_e46375_d_n5, assign30960_e46375_d_b0, assign30960_e46375_d_b1, assign30960_e46375_d_b2, assign30960_e46375_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30960_e46370: f64 = (var_tmf1 * var_tmf1);
        let assign30960_e46372: f64 = (assign30960_e46370 + var_tmf2);
        let assign30960_e46373: f64 = (assign30960_e46372).sqrt();
        (assign30960_e46373, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30960_e46373)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30960_e46373)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30960_e46373)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30960_e46373)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30960_e46373)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30960_e46373)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30960_e46373)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30960_e46373)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30960_e46373)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30960_e46373)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30960_e46375;
        var_tmf2_dn0 = assign30960_e46375_d_n0;
        var_tmf2_dn1 = assign30960_e46375_d_n1;
        var_tmf2_dn2 = assign30960_e46375_d_n2;
        var_tmf2_dn3 = assign30960_e46375_d_n3;
        var_tmf2_dn4 = assign30960_e46375_d_n4;
        var_tmf2_dn5 = assign30960_e46375_d_n5;
        var_tmf2_db0 = assign30960_e46375_d_b0;
        var_tmf2_db1 = assign30960_e46375_d_b1;
        var_tmf2_db2 = assign30960_e46375_d_b2;
        var_tmf2_db3 = assign30960_e46375_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign30970_e46393, assign30970_e46393_d_n0, assign30970_e46393_d_n1, assign30970_e46393_d_n2, assign30970_e46393_d_n3, assign30970_e46393_d_n4, assign30970_e46393_d_n5, assign30970_e46393_d_b0, assign30970_e46393_d_b1, assign30970_e46393_d_b2, assign30970_e46393_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30970_e46389: f64 = (var_tmf1 + var_tmf2);
        let assign30970_e46390: f64 = (0.5 * assign30970_e46389);
        let assign30970_e46391: f64 = (var_nfasti_i + assign30970_e46390);
        (assign30970_e46391, (var_nfasti_i_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_nfasti_i_dn1 + (0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (var_nfasti_i_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_nfasti_i_dn3 + (0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (var_nfasti_i_dn4 + (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_nfasti_i_dn5 + (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_nfasti_i_db0 + (0.5 * (var_tmf1_db0 + var_tmf2_db0))), (var_nfasti_i_db1 + (0.5 * (var_tmf1_db1 + var_tmf2_db1))), (var_nfasti_i_db2 + (0.5 * (var_tmf1_db2 + var_tmf2_db2))), (var_nfasti_i_db3 + (0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30970_e46393;
        var_nj0_dn0 = assign30970_e46393_d_n0;
        var_nj0_dn1 = assign30970_e46393_d_n1;
        var_nj0_dn2 = assign30970_e46393_d_n2;
        var_nj0_dn3 = assign30970_e46393_d_n3;
        var_nj0_dn4 = assign30970_e46393_d_n4;
        var_nj0_dn5 = assign30970_e46393_d_n5;
        var_nj0_db0 = assign30970_e46393_d_b0;
        var_nj0_db1 = assign30970_e46393_d_b1;
        var_nj0_db2 = assign30970_e46393_d_b2;
        var_nj0_db3 = assign30970_e46393_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign30980_e46409, assign30980_e46409_d_n0, assign30980_e46409_d_n1, assign30980_e46409_d_n2, assign30980_e46409_d_n3, assign30980_e46409_d_n4, assign30980_e46409_d_n5, assign30980_e46409_d_b0, assign30980_e46409_d_b1, assign30980_e46409_d_b2, assign30980_e46409_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30980_e46405: f64 = (p.p86 * var_dfn_su);
        let assign30980_e46407: f64 = (assign30980_e46405 * var_dfn_sl);
        (assign30980_e46407, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign30980_e46409;
        var_dnj1_dv_dn0 = assign30980_e46409_d_n0;
        var_dnj1_dv_dn1 = assign30980_e46409_d_n1;
        var_dnj1_dv_dn2 = assign30980_e46409_d_n2;
        var_dnj1_dv_dn3 = assign30980_e46409_d_n3;
        var_dnj1_dv_dn4 = assign30980_e46409_d_n4;
        var_dnj1_dv_dn5 = assign30980_e46409_d_n5;
        var_dnj1_dv_db0 = assign30980_e46409_d_b0;
        var_dnj1_dv_db1 = assign30980_e46409_d_b1;
        var_dnj1_dv_db2 = assign30980_e46409_d_b2;
        var_dnj1_dv_db3 = assign30980_e46409_d_b3;
        var_dnj1_dv_rv = 0.0;
        var_dnj1_dv_rdn0 = 0.0;
        var_dnj1_dv_rdn1 = 0.0;
        var_dnj1_dv_rdn2 = 0.0;
        var_dnj1_dv_rdn3 = 0.0;
        var_dnj1_dv_rdn4 = 0.0;
        var_dnj1_dv_rdn5 = 0.0;
        var_dnj1_dv_rdb0 = 0.0;
        var_dnj1_dv_rdb1 = 0.0;
        var_dnj1_dv_rdb2 = 0.0;
        var_dnj1_dv_rdb3 = 0.0;

        let (assign30990_e46422, assign30990_e46422_d_n0, assign30990_e46422_d_n1, assign30990_e46422_d_n2, assign30990_e46422_d_n3, assign30990_e46422_d_n4, assign30990_e46422_d_n5, assign30990_e46422_d_b0, assign30990_e46422_d_b1, assign30990_e46422_d_b2, assign30990_e46422_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 == 0.0)) {
        (var_nfasti_i, var_nfasti_i_dn0, var_nfasti_i_dn1, var_nfasti_i_dn2, var_nfasti_i_dn3, var_nfasti_i_dn4, var_nfasti_i_dn5, var_nfasti_i_db0, var_nfasti_i_db1, var_nfasti_i_db2, var_nfasti_i_db3,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30990_e46422;
        var_nj0_dn0 = assign30990_e46422_d_n0;
        var_nj0_dn1 = assign30990_e46422_d_n1;
        var_nj0_dn2 = assign30990_e46422_d_n2;
        var_nj0_dn3 = assign30990_e46422_d_n3;
        var_nj0_dn4 = assign30990_e46422_d_n4;
        var_nj0_dn5 = assign30990_e46422_d_n5;
        var_nj0_db0 = assign30990_e46422_d_b0;
        var_nj0_db1 = assign30990_e46422_d_b1;
        var_nj0_db2 = assign30990_e46422_d_b2;
        var_nj0_db3 = assign30990_e46422_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign31000_e46435, assign31000_e46435_d_n0, assign31000_e46435_d_n1, assign31000_e46435_d_n2, assign31000_e46435_d_n3, assign31000_e46435_d_n4, assign31000_e46435_d_n5, assign31000_e46435_d_b0, assign31000_e46435_d_b1, assign31000_e46435_d_b2, assign31000_e46435_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 == 0.0)) {
        (var_nfasti_i, var_nfasti_i_dn0, var_nfasti_i_dn1, var_nfasti_i_dn2, var_nfasti_i_dn3, var_nfasti_i_dn4, var_nfasti_i_dn5, var_nfasti_i_db0, var_nfasti_i_db1, var_nfasti_i_db2, var_nfasti_i_db3,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign31000_e46435;
        var_nj1_dn0 = assign31000_e46435_d_n0;
        var_nj1_dn1 = assign31000_e46435_d_n1;
        var_nj1_dn2 = assign31000_e46435_d_n2;
        var_nj1_dn3 = assign31000_e46435_d_n3;
        var_nj1_dn4 = assign31000_e46435_d_n4;
        var_nj1_dn5 = assign31000_e46435_d_n5;
        var_nj1_db0 = assign31000_e46435_d_b0;
        var_nj1_db1 = assign31000_e46435_d_b1;
        var_nj1_db2 = assign31000_e46435_d_b2;
        var_nj1_db3 = assign31000_e46435_d_b3;
        var_nj1_rv = 0.0;
        var_nj1_rdn0 = 0.0;
        var_nj1_rdn1 = 0.0;
        var_nj1_rdn2 = 0.0;
        var_nj1_rdn3 = 0.0;
        var_nj1_rdn4 = 0.0;
        var_nj1_rdn5 = 0.0;
        var_nj1_rdb0 = 0.0;
        var_nj1_rdb1 = 0.0;
        var_nj1_rdb2 = 0.0;
        var_nj1_rdb3 = 0.0;

        let (assign31010_e46448, assign31010_e46448_d_n0, assign31010_e46448_d_n1, assign31010_e46448_d_n2, assign31010_e46448_d_n3, assign31010_e46448_d_n4, assign31010_e46448_d_n5, assign31010_e46448_d_b0, assign31010_e46448_d_b1, assign31010_e46448_d_b2, assign31010_e46448_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign31010_e46448;
        var_dnj1_dv_dn0 = assign31010_e46448_d_n0;
        var_dnj1_dv_dn1 = assign31010_e46448_d_n1;
        var_dnj1_dv_dn2 = assign31010_e46448_d_n2;
        var_dnj1_dv_dn3 = assign31010_e46448_d_n3;
        var_dnj1_dv_dn4 = assign31010_e46448_d_n4;
        var_dnj1_dv_dn5 = assign31010_e46448_d_n5;
        var_dnj1_dv_db0 = assign31010_e46448_d_b0;
        var_dnj1_dv_db1 = assign31010_e46448_d_b1;
        var_dnj1_dv_db2 = assign31010_e46448_d_b2;
        var_dnj1_dv_db3 = assign31010_e46448_d_b3;
        var_dnj1_dv_rv = 0.0;
        var_dnj1_dv_rdn0 = 0.0;
        var_dnj1_dv_rdn1 = 0.0;
        var_dnj1_dv_rdn2 = 0.0;
        var_dnj1_dv_rdn3 = 0.0;
        var_dnj1_dv_rdn4 = 0.0;
        var_dnj1_dv_rdn5 = 0.0;
        var_dnj1_dv_rdb0 = 0.0;
        var_dnj1_dv_rdb1 = 0.0;
        var_dnj1_dv_rdb2 = 0.0;
        var_dnj1_dv_rdb3 = 0.0;

        let (assign31070_e46701, assign31070_e46701_d_n0, assign31070_e46701_d_n1, assign31070_e46701_d_n2, assign31070_e46701_d_n3, assign31070_e46701_d_n4, assign31070_e46701_d_n5, assign31070_e46701_d_b0, assign31070_e46701_d_b1, assign31070_e46701_d_b2, assign31070_e46701_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign31070_e46685: f64 = (var_vmax * var_dnj1_dv);
        let assign31070_e46686: f64 = (var_nj1 - assign31070_e46685);
        let assign31070_e46689: f64 = (var_nj1 * var_nj1);
        let assign31070_e46690: f64 = (assign31070_e46686 / assign31070_e46689);
        let assign31070_e46693: f64 = (var_vha1 * var_dnj1_dv);
        let assign31070_e46696: f64 = (var_nj0 * p.p85);
        let assign31070_e46697: f64 = (assign31070_e46693 / assign31070_e46696);
        let assign31070_e46698: f64 = (assign31070_e46690 + assign31070_e46697);
        let assign31070_e46699: f64 = (var_phitdinv * assign31070_e46698);
        (assign31070_e46699, ((var_phitdinv_dn0 * assign31070_e46698) + (var_phitdinv * (((((var_nj1_dn0 - ((var_vmax_dn0 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn0))) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign31070_e46689 * assign31070_e46689)) + (((((var_vha1_dn0 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn0)) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn0 * p.p85))) / (assign31070_e46696 * assign31070_e46696))))), ((var_phitdinv_dn1 * assign31070_e46698) + (var_phitdinv * (((((var_nj1_dn1 - ((var_vmax_dn1 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn1))) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign31070_e46689 * assign31070_e46689)) + (((((var_vha1_dn1 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn1)) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn1 * p.p85))) / (assign31070_e46696 * assign31070_e46696))))), ((var_phitdinv_dn2 * assign31070_e46698) + (var_phitdinv * (((((var_nj1_dn2 - ((var_vmax_dn2 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn2))) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign31070_e46689 * assign31070_e46689)) + (((((var_vha1_dn2 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn2)) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn2 * p.p85))) / (assign31070_e46696 * assign31070_e46696))))), ((var_phitdinv_dn3 * assign31070_e46698) + (var_phitdinv * (((((var_nj1_dn3 - ((var_vmax_dn3 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn3))) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign31070_e46689 * assign31070_e46689)) + (((((var_vha1_dn3 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn3)) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn3 * p.p85))) / (assign31070_e46696 * assign31070_e46696))))), ((var_phitdinv_dn4 * assign31070_e46698) + (var_phitdinv * (((((var_nj1_dn4 - ((var_vmax_dn4 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn4))) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign31070_e46689 * assign31070_e46689)) + (((((var_vha1_dn4 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn4)) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn4 * p.p85))) / (assign31070_e46696 * assign31070_e46696))))), ((var_phitdinv_dn5 * assign31070_e46698) + (var_phitdinv * (((((var_nj1_dn5 - ((var_vmax_dn5 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn5))) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign31070_e46689 * assign31070_e46689)) + (((((var_vha1_dn5 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn5)) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn5 * p.p85))) / (assign31070_e46696 * assign31070_e46696))))), ((var_phitdinv_db0 * assign31070_e46698) + (var_phitdinv * (((((var_nj1_db0 - ((var_vmax_db0 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_db0))) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign31070_e46689 * assign31070_e46689)) + (((((var_vha1_db0 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_db0)) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_db0 * p.p85))) / (assign31070_e46696 * assign31070_e46696))))), ((var_phitdinv_db1 * assign31070_e46698) + (var_phitdinv * (((((var_nj1_db1 - ((var_vmax_db1 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_db1))) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign31070_e46689 * assign31070_e46689)) + (((((var_vha1_db1 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_db1)) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_db1 * p.p85))) / (assign31070_e46696 * assign31070_e46696))))), ((var_phitdinv_db2 * assign31070_e46698) + (var_phitdinv * (((((var_nj1_db2 - ((var_vmax_db2 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_db2))) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign31070_e46689 * assign31070_e46689)) + (((((var_vha1_db2 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_db2)) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_db2 * p.p85))) / (assign31070_e46696 * assign31070_e46696))))), ((var_phitdinv_db3 * assign31070_e46698) + (var_phitdinv * (((((var_nj1_db3 - ((var_vmax_db3 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_db3))) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign31070_e46689 * assign31070_e46689)) + (((((var_vha1_db3 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_db3)) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_db3 * p.p85))) / (assign31070_e46696 * assign31070_e46696))))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign31070_e46701;
        var_dvmax_over_phitd_dv_dn0 = assign31070_e46701_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign31070_e46701_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign31070_e46701_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign31070_e46701_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign31070_e46701_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign31070_e46701_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign31070_e46701_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign31070_e46701_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign31070_e46701_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign31070_e46701_d_b3;
        var_dvmax_over_phitd_dv_rv = 0.0;
        var_dvmax_over_phitd_dv_rdn0 = 0.0;
        var_dvmax_over_phitd_dv_rdn1 = 0.0;
        var_dvmax_over_phitd_dv_rdn2 = 0.0;
        var_dvmax_over_phitd_dv_rdn3 = 0.0;
        var_dvmax_over_phitd_dv_rdn4 = 0.0;
        var_dvmax_over_phitd_dv_rdn5 = 0.0;
        var_dvmax_over_phitd_dv_rdb0 = 0.0;
        var_dvmax_over_phitd_dv_rdb1 = 0.0;
        var_dvmax_over_phitd_dv_rdb2 = 0.0;
        var_dvmax_over_phitd_dv_rdb3 = 0.0;

        let (assign31090_e46733, assign31090_e46733_d_n0, assign31090_e46733_d_n1, assign31090_e46733_d_n2, assign31090_e46733_d_n3, assign31090_e46733_d_n4, assign31090_e46733_d_n5, assign31090_e46733_d_b0, assign31090_e46733_d_b1, assign31090_e46733_d_b2, assign31090_e46733_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign31090_e46729: f64 = (var_nin * var_nin);
        let assign31090_e46731: f64 = (assign31090_e46729 / var_ndigat_i);
        (assign31090_e46731, (((((var_nin_dn0 * var_nin) + (var_nin * var_nin_dn0)) * var_ndigat_i) - (assign31090_e46729 * var_ndigat_i_dn0)) / (var_ndigat_i * var_ndigat_i)), (((((var_nin_dn1 * var_nin) + (var_nin * var_nin_dn1)) * var_ndigat_i) - (assign31090_e46729 * var_ndigat_i_dn1)) / (var_ndigat_i * var_ndigat_i)), (((((var_nin_dn2 * var_nin) + (var_nin * var_nin_dn2)) * var_ndigat_i) - (assign31090_e46729 * var_ndigat_i_dn2)) / (var_ndigat_i * var_ndigat_i)), (((((var_nin_dn3 * var_nin) + (var_nin * var_nin_dn3)) * var_ndigat_i) - (assign31090_e46729 * var_ndigat_i_dn3)) / (var_ndigat_i * var_ndigat_i)), (((((var_nin_dn4 * var_nin) + (var_nin * var_nin_dn4)) * var_ndigat_i) - (assign31090_e46729 * var_ndigat_i_dn4)) / (var_ndigat_i * var_ndigat_i)), (((((var_nin_dn5 * var_nin) + (var_nin * var_nin_dn5)) * var_ndigat_i) - (assign31090_e46729 * var_ndigat_i_dn5)) / (var_ndigat_i * var_ndigat_i)), (((((var_nin_db0 * var_nin) + (var_nin * var_nin_db0)) * var_ndigat_i) - (assign31090_e46729 * var_ndigat_i_db0)) / (var_ndigat_i * var_ndigat_i)), (((((var_nin_db1 * var_nin) + (var_nin * var_nin_db1)) * var_ndigat_i) - (assign31090_e46729 * var_ndigat_i_db1)) / (var_ndigat_i * var_ndigat_i)), (((((var_nin_db2 * var_nin) + (var_nin * var_nin_db2)) * var_ndigat_i) - (assign31090_e46729 * var_ndigat_i_db2)) / (var_ndigat_i * var_ndigat_i)), (((((var_nin_db3 * var_nin) + (var_nin * var_nin_db3)) * var_ndigat_i) - (assign31090_e46729 * var_ndigat_i_db3)) / (var_ndigat_i * var_ndigat_i)),)
    } else {
        (var_pnn0, var_pnn0_dn0, var_pnn0_dn1, var_pnn0_dn2, var_pnn0_dn3, var_pnn0_dn4, var_pnn0_dn5, var_pnn0_db0, var_pnn0_db1, var_pnn0_db2, var_pnn0_db3,)
    }
};
        var_pnn0 = assign31090_e46733;
        var_pnn0_dn0 = assign31090_e46733_d_n0;
        var_pnn0_dn1 = assign31090_e46733_d_n1;
        var_pnn0_dn2 = assign31090_e46733_d_n2;
        var_pnn0_dn3 = assign31090_e46733_d_n3;
        var_pnn0_dn4 = assign31090_e46733_d_n4;
        var_pnn0_dn5 = assign31090_e46733_d_n5;
        var_pnn0_db0 = assign31090_e46733_d_b0;
        var_pnn0_db1 = assign31090_e46733_d_b1;
        var_pnn0_db2 = assign31090_e46733_d_b2;
        var_pnn0_db3 = assign31090_e46733_d_b3;
        var_pnn0_rv = 0.0;
        var_pnn0_rdn0 = 0.0;
        var_pnn0_rdn1 = 0.0;
        var_pnn0_rdn2 = 0.0;
        var_pnn0_rdn3 = 0.0;
        var_pnn0_rdn4 = 0.0;
        var_pnn0_rdn5 = 0.0;
        var_pnn0_rdb0 = 0.0;
        var_pnn0_rdb1 = 0.0;
        var_pnn0_rdb2 = 0.0;
        var_pnn0_rdb3 = 0.0;

        let (assign31100_e46750, assign31100_e46750_d_n0, assign31100_e46750_d_n1, assign31100_e46750_d_n2, assign31100_e46750_d_n3, assign31100_e46750_d_n4, assign31100_e46750_d_n5, assign31100_e46750_d_b0, assign31100_e46750_d_b1, assign31100_e46750_d_b2, assign31100_e46750_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign31100_e46743: f64 = (var_nfagat_i / var_phitdinv);
        let assign31100_e46746: f64 = (var_ndigat_i / var_pnn0);
        let assign31100_e46747: f64 = (assign31100_e46746).ln();
        let assign31100_e46748: f64 = (assign31100_e46743 * assign31100_e46747);
        (assign31100_e46748, (((((var_nfagat_i_dn0 * var_phitdinv) - (var_nfagat_i * var_phitdinv_dn0)) / (var_phitdinv * var_phitdinv)) * assign31100_e46747) + (assign31100_e46743 * ((((var_ndigat_i_dn0 * var_pnn0) - (var_ndigat_i * var_pnn0_dn0)) / (var_pnn0 * var_pnn0)) / assign31100_e46746))), (((((var_nfagat_i_dn1 * var_phitdinv) - (var_nfagat_i * var_phitdinv_dn1)) / (var_phitdinv * var_phitdinv)) * assign31100_e46747) + (assign31100_e46743 * ((((var_ndigat_i_dn1 * var_pnn0) - (var_ndigat_i * var_pnn0_dn1)) / (var_pnn0 * var_pnn0)) / assign31100_e46746))), (((((var_nfagat_i_dn2 * var_phitdinv) - (var_nfagat_i * var_phitdinv_dn2)) / (var_phitdinv * var_phitdinv)) * assign31100_e46747) + (assign31100_e46743 * ((((var_ndigat_i_dn2 * var_pnn0) - (var_ndigat_i * var_pnn0_dn2)) / (var_pnn0 * var_pnn0)) / assign31100_e46746))), (((((var_nfagat_i_dn3 * var_phitdinv) - (var_nfagat_i * var_phitdinv_dn3)) / (var_phitdinv * var_phitdinv)) * assign31100_e46747) + (assign31100_e46743 * ((((var_ndigat_i_dn3 * var_pnn0) - (var_ndigat_i * var_pnn0_dn3)) / (var_pnn0 * var_pnn0)) / assign31100_e46746))), (((((var_nfagat_i_dn4 * var_phitdinv) - (var_nfagat_i * var_phitdinv_dn4)) / (var_phitdinv * var_phitdinv)) * assign31100_e46747) + (assign31100_e46743 * ((((var_ndigat_i_dn4 * var_pnn0) - (var_ndigat_i * var_pnn0_dn4)) / (var_pnn0 * var_pnn0)) / assign31100_e46746))), (((((var_nfagat_i_dn5 * var_phitdinv) - (var_nfagat_i * var_phitdinv_dn5)) / (var_phitdinv * var_phitdinv)) * assign31100_e46747) + (assign31100_e46743 * ((((var_ndigat_i_dn5 * var_pnn0) - (var_ndigat_i * var_pnn0_dn5)) / (var_pnn0 * var_pnn0)) / assign31100_e46746))), (((((var_nfagat_i_db0 * var_phitdinv) - (var_nfagat_i * var_phitdinv_db0)) / (var_phitdinv * var_phitdinv)) * assign31100_e46747) + (assign31100_e46743 * ((((var_ndigat_i_db0 * var_pnn0) - (var_ndigat_i * var_pnn0_db0)) / (var_pnn0 * var_pnn0)) / assign31100_e46746))), (((((var_nfagat_i_db1 * var_phitdinv) - (var_nfagat_i * var_phitdinv_db1)) / (var_phitdinv * var_phitdinv)) * assign31100_e46747) + (assign31100_e46743 * ((((var_ndigat_i_db1 * var_pnn0) - (var_ndigat_i * var_pnn0_db1)) / (var_pnn0 * var_pnn0)) / assign31100_e46746))), (((((var_nfagat_i_db2 * var_phitdinv) - (var_nfagat_i * var_phitdinv_db2)) / (var_phitdinv * var_phitdinv)) * assign31100_e46747) + (assign31100_e46743 * ((((var_ndigat_i_db2 * var_pnn0) - (var_ndigat_i * var_pnn0_db2)) / (var_pnn0 * var_pnn0)) / assign31100_e46746))), (((((var_nfagat_i_db3 * var_phitdinv) - (var_nfagat_i * var_phitdinv_db3)) / (var_phitdinv * var_phitdinv)) * assign31100_e46747) + (assign31100_e46743 * ((((var_ndigat_i_db3 * var_pnn0) - (var_ndigat_i * var_pnn0_db3)) / (var_pnn0 * var_pnn0)) / assign31100_e46746))),)
    } else {
        (var_vha1, var_vha1_dn0, var_vha1_dn1, var_vha1_dn2, var_vha1_dn3, var_vha1_dn4, var_vha1_dn5, var_vha1_db0, var_vha1_db1, var_vha1_db2, var_vha1_db3,)
    }
};
        var_vha1 = assign31100_e46750;
        var_vha1_dn0 = assign31100_e46750_d_n0;
        var_vha1_dn1 = assign31100_e46750_d_n1;
        var_vha1_dn2 = assign31100_e46750_d_n2;
        var_vha1_dn3 = assign31100_e46750_d_n3;
        var_vha1_dn4 = assign31100_e46750_d_n4;
        var_vha1_dn5 = assign31100_e46750_d_n5;
        var_vha1_db0 = assign31100_e46750_d_b0;
        var_vha1_db1 = assign31100_e46750_d_b1;
        var_vha1_db2 = assign31100_e46750_d_b2;
        var_vha1_db3 = assign31100_e46750_d_b3;
        var_vha1_rv = 0.0;
        var_vha1_rdn0 = 0.0;
        var_vha1_rdn1 = 0.0;
        var_vha1_rdn2 = 0.0;
        var_vha1_rdn3 = 0.0;
        var_vha1_rdn4 = 0.0;
        var_vha1_rdn5 = 0.0;
        var_vha1_rdb0 = 0.0;
        var_vha1_rdb1 = 0.0;
        var_vha1_rdb2 = 0.0;
        var_vha1_rdb3 = 0.0;

        let assign31110_e46753: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard498 = assign31110_e46753;
        var_guard498_dn0 = 0.0;
        var_guard498_dn1 = 0.0;
        var_guard498_dn2 = 0.0;
        var_guard498_dn3 = 0.0;
        var_guard498_dn4 = 0.0;
        var_guard498_dn5 = 0.0;
        var_guard498_db0 = 0.0;
        var_guard498_db1 = 0.0;
        var_guard498_db2 = 0.0;
        var_guard498_db3 = 0.0;
        var_guard498_rv = 0.0;
        var_guard498_rdn0 = 0.0;
        var_guard498_rdn1 = 0.0;
        var_guard498_rdn2 = 0.0;
        var_guard498_rdn3 = 0.0;
        var_guard498_rdn4 = 0.0;
        var_guard498_rdn5 = 0.0;
        var_guard498_rdb0 = 0.0;
        var_guard498_rdb1 = 0.0;
        var_guard498_rdb2 = 0.0;
        var_guard498_rdb3 = 0.0;

        let (assign31120_e46771, assign31120_e46771_d_n0, assign31120_e46771_d_n1, assign31120_e46771_d_n2, assign31120_e46771_d_n3, assign31120_e46771_d_n4, assign31120_e46771_d_n5, assign31120_e46771_d_b0, assign31120_e46771_d_b1, assign31120_e46771_d_b2, assign31120_e46771_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31120_e46766: f64 = (var_vmax - var_vha1);
        let assign31120_e46767: f64 = (p.p86 * assign31120_e46766);
        let assign31120_e46769: f64 = (assign31120_e46767 + var_nfagat_i);
        (assign31120_e46769, ((p.p86 * (var_vmax_dn0 - var_vha1_dn0)) + var_nfagat_i_dn0), ((p.p86 * (var_vmax_dn1 - var_vha1_dn1)) + var_nfagat_i_dn1), ((p.p86 * (var_vmax_dn2 - var_vha1_dn2)) + var_nfagat_i_dn2), ((p.p86 * (var_vmax_dn3 - var_vha1_dn3)) + var_nfagat_i_dn3), ((p.p86 * (var_vmax_dn4 - var_vha1_dn4)) + var_nfagat_i_dn4), ((p.p86 * (var_vmax_dn5 - var_vha1_dn5)) + var_nfagat_i_dn5), ((p.p86 * (var_vmax_db0 - var_vha1_db0)) + var_nfagat_i_db0), ((p.p86 * (var_vmax_db1 - var_vha1_db1)) + var_nfagat_i_db1), ((p.p86 * (var_vmax_db2 - var_vha1_db2)) + var_nfagat_i_db2), ((p.p86 * (var_vmax_db3 - var_vha1_db3)) + var_nfagat_i_db3),)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign31120_e46771;
        var_nja10_dn0 = assign31120_e46771_d_n0;
        var_nja10_dn1 = assign31120_e46771_d_n1;
        var_nja10_dn2 = assign31120_e46771_d_n2;
        var_nja10_dn3 = assign31120_e46771_d_n3;
        var_nja10_dn4 = assign31120_e46771_d_n4;
        var_nja10_dn5 = assign31120_e46771_d_n5;
        var_nja10_db0 = assign31120_e46771_d_b0;
        var_nja10_db1 = assign31120_e46771_d_b1;
        var_nja10_db2 = assign31120_e46771_d_b2;
        var_nja10_db3 = assign31120_e46771_d_b3;
        var_nja10_rv = 0.0;
        var_nja10_rdn0 = 0.0;
        var_nja10_rdn1 = 0.0;
        var_nja10_rdn2 = 0.0;
        var_nja10_rdn3 = 0.0;
        var_nja10_rdn4 = 0.0;
        var_nja10_rdn5 = 0.0;
        var_nja10_rdb0 = 0.0;
        var_nja10_rdb1 = 0.0;
        var_nja10_rdb2 = 0.0;
        var_nja10_rdb3 = 0.0;

        let (assign31130_e46787, assign31130_e46787_d_n0, assign31130_e46787_d_n1, assign31130_e46787_d_n2, assign31130_e46787_d_n3, assign31130_e46787_d_n4, assign31130_e46787_d_n5, assign31130_e46787_d_b0, assign31130_e46787_d_b1, assign31130_e46787_d_b2, assign31130_e46787_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31130_e46784: f64 = (p.p86 * var_vha1);
        let assign31130_e46785: f64 = (var_nfagat_i - assign31130_e46784);
        (assign31130_e46785, (var_nfagat_i_dn0 - (p.p86 * var_vha1_dn0)), (var_nfagat_i_dn1 - (p.p86 * var_vha1_dn1)), (var_nfagat_i_dn2 - (p.p86 * var_vha1_dn2)), (var_nfagat_i_dn3 - (p.p86 * var_vha1_dn3)), (var_nfagat_i_dn4 - (p.p86 * var_vha1_dn4)), (var_nfagat_i_dn5 - (p.p86 * var_vha1_dn5)), (var_nfagat_i_db0 - (p.p86 * var_vha1_db0)), (var_nfagat_i_db1 - (p.p86 * var_vha1_db1)), (var_nfagat_i_db2 - (p.p86 * var_vha1_db2)), (var_nfagat_i_db3 - (p.p86 * var_vha1_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign31130_e46787;
        var_nj0_dn0 = assign31130_e46787_d_n0;
        var_nj0_dn1 = assign31130_e46787_d_n1;
        var_nj0_dn2 = assign31130_e46787_d_n2;
        var_nj0_dn3 = assign31130_e46787_d_n3;
        var_nj0_dn4 = assign31130_e46787_d_n4;
        var_nj0_dn5 = assign31130_e46787_d_n5;
        var_nj0_db0 = assign31130_e46787_d_b0;
        var_nj0_db1 = assign31130_e46787_d_b1;
        var_nj0_db2 = assign31130_e46787_d_b2;
        var_nj0_db3 = assign31130_e46787_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign31140_e46803, assign31140_e46803_d_n0, assign31140_e46803_d_n1, assign31140_e46803_d_n2, assign31140_e46803_d_n3, assign31140_e46803_d_n4, assign31140_e46803_d_n5, assign31140_e46803_d_b0, assign31140_e46803_d_b1, assign31140_e46803_d_b2, assign31140_e46803_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31140_e46799: f64 = (p.p85 - var_nja10);
        let assign31140_e46801: f64 = (assign31140_e46799 - 0.01);
        (assign31140_e46801, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign31140_e46803;
        var_tmf1_dn0 = assign31140_e46803_d_n0;
        var_tmf1_dn1 = assign31140_e46803_d_n1;
        var_tmf1_dn2 = assign31140_e46803_d_n2;
        var_tmf1_dn3 = assign31140_e46803_d_n3;
        var_tmf1_dn4 = assign31140_e46803_d_n4;
        var_tmf1_dn5 = assign31140_e46803_d_n5;
        var_tmf1_db0 = assign31140_e46803_d_b0;
        var_tmf1_db1 = assign31140_e46803_d_b1;
        var_tmf1_db2 = assign31140_e46803_d_b2;
        var_tmf1_db3 = assign31140_e46803_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign31150_e46819, assign31150_e46819_d_n0, assign31150_e46819_d_n1, assign31150_e46819_d_n2, assign31150_e46819_d_n3, assign31150_e46819_d_n4, assign31150_e46819_d_n5, assign31150_e46819_d_b0, assign31150_e46819_d_b1, assign31150_e46819_d_b2, assign31150_e46819_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31150_e46815: f64 = (4.0 * p.p85);
        let assign31150_e46817: f64 = (assign31150_e46815 * 0.01);
        (assign31150_e46817, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31150_e46819;
        var_tmf2_dn0 = assign31150_e46819_d_n0;
        var_tmf2_dn1 = assign31150_e46819_d_n1;
        var_tmf2_dn2 = assign31150_e46819_d_n2;
        var_tmf2_dn3 = assign31150_e46819_d_n3;
        var_tmf2_dn4 = assign31150_e46819_d_n4;
        var_tmf2_dn5 = assign31150_e46819_d_n5;
        var_tmf2_db0 = assign31150_e46819_d_b0;
        var_tmf2_db1 = assign31150_e46819_d_b1;
        var_tmf2_db2 = assign31150_e46819_d_b2;
        var_tmf2_db3 = assign31150_e46819_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_db0_slot = var_dnj1_dv_db0;
        *var_dnj1_dv_db1_slot = var_dnj1_dv_db1;
        *var_dnj1_dv_db2_slot = var_dnj1_dv_db2;
        *var_dnj1_dv_db3_slot = var_dnj1_dv_db3;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn1_slot = var_dnj1_dv_dn1;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_dn3_slot = var_dnj1_dv_dn3;
        *var_dnj1_dv_dn4_slot = var_dnj1_dv_dn4;
        *var_dnj1_dv_dn5_slot = var_dnj1_dv_dn5;
        *var_dnj1_dv_rdb0_slot = var_dnj1_dv_rdb0;
        *var_dnj1_dv_rdb1_slot = var_dnj1_dv_rdb1;
        *var_dnj1_dv_rdb2_slot = var_dnj1_dv_rdb2;
        *var_dnj1_dv_rdb3_slot = var_dnj1_dv_rdb3;
        *var_dnj1_dv_rdn0_slot = var_dnj1_dv_rdn0;
        *var_dnj1_dv_rdn1_slot = var_dnj1_dv_rdn1;
        *var_dnj1_dv_rdn2_slot = var_dnj1_dv_rdn2;
        *var_dnj1_dv_rdn3_slot = var_dnj1_dv_rdn3;
        *var_dnj1_dv_rdn4_slot = var_dnj1_dv_rdn4;
        *var_dnj1_dv_rdn5_slot = var_dnj1_dv_rdn5;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_db0_slot = var_dvmax_over_phitd_dv_db0;
        *var_dvmax_over_phitd_dv_db1_slot = var_dvmax_over_phitd_dv_db1;
        *var_dvmax_over_phitd_dv_db2_slot = var_dvmax_over_phitd_dv_db2;
        *var_dvmax_over_phitd_dv_db3_slot = var_dvmax_over_phitd_dv_db3;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn1_slot = var_dvmax_over_phitd_dv_dn1;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_dn3_slot = var_dvmax_over_phitd_dv_dn3;
        *var_dvmax_over_phitd_dv_dn4_slot = var_dvmax_over_phitd_dv_dn4;
        *var_dvmax_over_phitd_dv_dn5_slot = var_dvmax_over_phitd_dv_dn5;
        *var_dvmax_over_phitd_dv_rdb0_slot = var_dvmax_over_phitd_dv_rdb0;
        *var_dvmax_over_phitd_dv_rdb1_slot = var_dvmax_over_phitd_dv_rdb1;
        *var_dvmax_over_phitd_dv_rdb2_slot = var_dvmax_over_phitd_dv_rdb2;
        *var_dvmax_over_phitd_dv_rdb3_slot = var_dvmax_over_phitd_dv_rdb3;
        *var_dvmax_over_phitd_dv_rdn0_slot = var_dvmax_over_phitd_dv_rdn0;
        *var_dvmax_over_phitd_dv_rdn1_slot = var_dvmax_over_phitd_dv_rdn1;
        *var_dvmax_over_phitd_dv_rdn2_slot = var_dvmax_over_phitd_dv_rdn2;
        *var_dvmax_over_phitd_dv_rdn3_slot = var_dvmax_over_phitd_dv_rdn3;
        *var_dvmax_over_phitd_dv_rdn4_slot = var_dvmax_over_phitd_dv_rdn4;
        *var_dvmax_over_phitd_dv_rdn5_slot = var_dvmax_over_phitd_dv_rdn5;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_guard498_slot = var_guard498;
        *var_guard498_db0_slot = var_guard498_db0;
        *var_guard498_db1_slot = var_guard498_db1;
        *var_guard498_db2_slot = var_guard498_db2;
        *var_guard498_db3_slot = var_guard498_db3;
        *var_guard498_dn0_slot = var_guard498_dn0;
        *var_guard498_dn1_slot = var_guard498_dn1;
        *var_guard498_dn2_slot = var_guard498_dn2;
        *var_guard498_dn3_slot = var_guard498_dn3;
        *var_guard498_dn4_slot = var_guard498_dn4;
        *var_guard498_dn5_slot = var_guard498_dn5;
        *var_guard498_rdb0_slot = var_guard498_rdb0;
        *var_guard498_rdb1_slot = var_guard498_rdb1;
        *var_guard498_rdb2_slot = var_guard498_rdb2;
        *var_guard498_rdb3_slot = var_guard498_rdb3;
        *var_guard498_rdn0_slot = var_guard498_rdn0;
        *var_guard498_rdn1_slot = var_guard498_rdn1;
        *var_guard498_rdn2_slot = var_guard498_rdn2;
        *var_guard498_rdn3_slot = var_guard498_rdn3;
        *var_guard498_rdn4_slot = var_guard498_rdn4;
        *var_guard498_rdn5_slot = var_guard498_rdn5;
        *var_guard498_rv_slot = var_guard498_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj0_rdb0_slot = var_nj0_rdb0;
        *var_nj0_rdb1_slot = var_nj0_rdb1;
        *var_nj0_rdb2_slot = var_nj0_rdb2;
        *var_nj0_rdb3_slot = var_nj0_rdb3;
        *var_nj0_rdn0_slot = var_nj0_rdn0;
        *var_nj0_rdn1_slot = var_nj0_rdn1;
        *var_nj0_rdn2_slot = var_nj0_rdn2;
        *var_nj0_rdn3_slot = var_nj0_rdn3;
        *var_nj0_rdn4_slot = var_nj0_rdn4;
        *var_nj0_rdn5_slot = var_nj0_rdn5;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nj1_rdb0_slot = var_nj1_rdb0;
        *var_nj1_rdb1_slot = var_nj1_rdb1;
        *var_nj1_rdb2_slot = var_nj1_rdb2;
        *var_nj1_rdb3_slot = var_nj1_rdb3;
        *var_nj1_rdn0_slot = var_nj1_rdn0;
        *var_nj1_rdn1_slot = var_nj1_rdn1;
        *var_nj1_rdn2_slot = var_nj1_rdn2;
        *var_nj1_rdn3_slot = var_nj1_rdn3;
        *var_nj1_rdn4_slot = var_nj1_rdn4;
        *var_nj1_rdn5_slot = var_nj1_rdn5;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_nja10_rdb0_slot = var_nja10_rdb0;
        *var_nja10_rdb1_slot = var_nja10_rdb1;
        *var_nja10_rdb2_slot = var_nja10_rdb2;
        *var_nja10_rdb3_slot = var_nja10_rdb3;
        *var_nja10_rdn0_slot = var_nja10_rdn0;
        *var_nja10_rdn1_slot = var_nja10_rdn1;
        *var_nja10_rdn2_slot = var_nja10_rdn2;
        *var_nja10_rdn3_slot = var_nja10_rdn3;
        *var_nja10_rdn4_slot = var_nja10_rdn4;
        *var_nja10_rdn5_slot = var_nja10_rdn5;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_db0_slot = var_pnn0_db0;
        *var_pnn0_db1_slot = var_pnn0_db1;
        *var_pnn0_db2_slot = var_pnn0_db2;
        *var_pnn0_db3_slot = var_pnn0_db3;
        *var_pnn0_dn0_slot = var_pnn0_dn0;
        *var_pnn0_dn1_slot = var_pnn0_dn1;
        *var_pnn0_dn2_slot = var_pnn0_dn2;
        *var_pnn0_dn3_slot = var_pnn0_dn3;
        *var_pnn0_dn4_slot = var_pnn0_dn4;
        *var_pnn0_dn5_slot = var_pnn0_dn5;
        *var_pnn0_rdb0_slot = var_pnn0_rdb0;
        *var_pnn0_rdb1_slot = var_pnn0_rdb1;
        *var_pnn0_rdb2_slot = var_pnn0_rdb2;
        *var_pnn0_rdb3_slot = var_pnn0_rdb3;
        *var_pnn0_rdn0_slot = var_pnn0_rdn0;
        *var_pnn0_rdn1_slot = var_pnn0_rdn1;
        *var_pnn0_rdn2_slot = var_pnn0_rdn2;
        *var_pnn0_rdn3_slot = var_pnn0_rdn3;
        *var_pnn0_rdn4_slot = var_pnn0_rdn4;
        *var_pnn0_rdn5_slot = var_pnn0_rdn5;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_rdb0_slot = var_tmf1_rdb0;
        *var_tmf1_rdb1_slot = var_tmf1_rdb1;
        *var_tmf1_rdb2_slot = var_tmf1_rdb2;
        *var_tmf1_rdb3_slot = var_tmf1_rdb3;
        *var_tmf1_rdn0_slot = var_tmf1_rdn0;
        *var_tmf1_rdn1_slot = var_tmf1_rdn1;
        *var_tmf1_rdn2_slot = var_tmf1_rdn2;
        *var_tmf1_rdn3_slot = var_tmf1_rdn3;
        *var_tmf1_rdn4_slot = var_tmf1_rdn4;
        *var_tmf1_rdn5_slot = var_tmf1_rdn5;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_rdb0_slot = var_tmf2_rdb0;
        *var_tmf2_rdb1_slot = var_tmf2_rdb1;
        *var_tmf2_rdb2_slot = var_tmf2_rdb2;
        *var_tmf2_rdb3_slot = var_tmf2_rdb3;
        *var_tmf2_rdn0_slot = var_tmf2_rdn0;
        *var_tmf2_rdn1_slot = var_tmf2_rdn1;
        *var_tmf2_rdn2_slot = var_tmf2_rdn2;
        *var_tmf2_rdn3_slot = var_tmf2_rdn3;
        *var_tmf2_rdn4_slot = var_tmf2_rdn4;
        *var_tmf2_rdn5_slot = var_tmf2_rdn5;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_db0_slot = var_vha1_db0;
        *var_vha1_db1_slot = var_vha1_db1;
        *var_vha1_db2_slot = var_vha1_db2;
        *var_vha1_db3_slot = var_vha1_db3;
        *var_vha1_dn0_slot = var_vha1_dn0;
        *var_vha1_dn1_slot = var_vha1_dn1;
        *var_vha1_dn2_slot = var_vha1_dn2;
        *var_vha1_dn3_slot = var_vha1_dn3;
        *var_vha1_dn4_slot = var_vha1_dn4;
        *var_vha1_dn5_slot = var_vha1_dn5;
        *var_vha1_rdb0_slot = var_vha1_rdb0;
        *var_vha1_rdb1_slot = var_vha1_rdb1;
        *var_vha1_rdb2_slot = var_vha1_rdb2;
        *var_vha1_rdb3_slot = var_vha1_rdb3;
        *var_vha1_rdn0_slot = var_vha1_rdn0;
        *var_vha1_rdn1_slot = var_vha1_rdn1;
        *var_vha1_rdn2_slot = var_vha1_rdn2;
        *var_vha1_rdn3_slot = var_vha1_rdn3;
        *var_vha1_rdn4_slot = var_vha1_rdn4;
        *var_vha1_rdn5_slot = var_vha1_rdn5;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_82(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard498: f64,
        var_nfagat_i: f64,
        var_nfagat_i_db0: f64,
        var_nfagat_i_db1: f64,
        var_nfagat_i_db2: f64,
        var_nfagat_i_db3: f64,
        var_nfagat_i_dn0: f64,
        var_nfagat_i_dn1: f64,
        var_nfagat_i_dn2: f64,
        var_nfagat_i_dn3: f64,
        var_nfagat_i_dn4: f64,
        var_nfagat_i_dn5: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_db0_slot: &mut f64,
        var_dfn_sl_db1_slot: &mut f64,
        var_dfn_sl_db2_slot: &mut f64,
        var_dfn_sl_db3_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn1_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_dn3_slot: &mut f64,
        var_dfn_sl_dn4_slot: &mut f64,
        var_dfn_sl_dn5_slot: &mut f64,
        var_dfn_sl_rdb0_slot: &mut f64,
        var_dfn_sl_rdb1_slot: &mut f64,
        var_dfn_sl_rdb2_slot: &mut f64,
        var_dfn_sl_rdb3_slot: &mut f64,
        var_dfn_sl_rdn0_slot: &mut f64,
        var_dfn_sl_rdn1_slot: &mut f64,
        var_dfn_sl_rdn2_slot: &mut f64,
        var_dfn_sl_rdn3_slot: &mut f64,
        var_dfn_sl_rdn4_slot: &mut f64,
        var_dfn_sl_rdn5_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_db0_slot: &mut f64,
        var_dfn_su_db1_slot: &mut f64,
        var_dfn_su_db2_slot: &mut f64,
        var_dfn_su_db3_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn1_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_dn3_slot: &mut f64,
        var_dfn_su_dn4_slot: &mut f64,
        var_dfn_su_dn5_slot: &mut f64,
        var_dfn_su_rdb0_slot: &mut f64,
        var_dfn_su_rdb1_slot: &mut f64,
        var_dfn_su_rdb2_slot: &mut f64,
        var_dfn_su_rdb3_slot: &mut f64,
        var_dfn_su_rdn0_slot: &mut f64,
        var_dfn_su_rdn1_slot: &mut f64,
        var_dfn_su_rdn2_slot: &mut f64,
        var_dfn_su_rdn3_slot: &mut f64,
        var_dfn_su_rdn4_slot: &mut f64,
        var_dfn_su_rdn5_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj0_rdb0_slot: &mut f64,
        var_nj0_rdb1_slot: &mut f64,
        var_nj0_rdb2_slot: &mut f64,
        var_nj0_rdb3_slot: &mut f64,
        var_nj0_rdn0_slot: &mut f64,
        var_nj0_rdn1_slot: &mut f64,
        var_nj0_rdn2_slot: &mut f64,
        var_nj0_rdn3_slot: &mut f64,
        var_nj0_rdn4_slot: &mut f64,
        var_nj0_rdn5_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nj1_rdb0_slot: &mut f64,
        var_nj1_rdb1_slot: &mut f64,
        var_nj1_rdb2_slot: &mut f64,
        var_nj1_rdb3_slot: &mut f64,
        var_nj1_rdn0_slot: &mut f64,
        var_nj1_rdn1_slot: &mut f64,
        var_nj1_rdn2_slot: &mut f64,
        var_nj1_rdn3_slot: &mut f64,
        var_nj1_rdn4_slot: &mut f64,
        var_nj1_rdn5_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_nja11_rdb0_slot: &mut f64,
        var_nja11_rdb1_slot: &mut f64,
        var_nja11_rdb2_slot: &mut f64,
        var_nja11_rdb3_slot: &mut f64,
        var_nja11_rdn0_slot: &mut f64,
        var_nja11_rdn1_slot: &mut f64,
        var_nja11_rdn2_slot: &mut f64,
        var_nja11_rdn3_slot: &mut f64,
        var_nja11_rdn4_slot: &mut f64,
        var_nja11_rdn5_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_rdb0_slot: &mut f64,
        var_tmf1_rdb1_slot: &mut f64,
        var_tmf1_rdb2_slot: &mut f64,
        var_tmf1_rdb3_slot: &mut f64,
        var_tmf1_rdn0_slot: &mut f64,
        var_tmf1_rdn1_slot: &mut f64,
        var_tmf1_rdn2_slot: &mut f64,
        var_tmf1_rdn3_slot: &mut f64,
        var_tmf1_rdn4_slot: &mut f64,
        var_tmf1_rdn5_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_rdb0_slot: &mut f64,
        var_tmf2_rdb1_slot: &mut f64,
        var_tmf2_rdb2_slot: &mut f64,
        var_tmf2_rdb3_slot: &mut f64,
        var_tmf2_rdn0_slot: &mut f64,
        var_tmf2_rdn1_slot: &mut f64,
        var_tmf2_rdn2_slot: &mut f64,
        var_tmf2_rdn3_slot: &mut f64,
        var_tmf2_rdn4_slot: &mut f64,
        var_tmf2_rdn5_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_db0: f64 = *var_dfn_sl_db0_slot;
        let mut var_dfn_sl_db1: f64 = *var_dfn_sl_db1_slot;
        let mut var_dfn_sl_db2: f64 = *var_dfn_sl_db2_slot;
        let mut var_dfn_sl_db3: f64 = *var_dfn_sl_db3_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn1: f64 = *var_dfn_sl_dn1_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_dn3: f64 = *var_dfn_sl_dn3_slot;
        let mut var_dfn_sl_dn4: f64 = *var_dfn_sl_dn4_slot;
        let mut var_dfn_sl_dn5: f64 = *var_dfn_sl_dn5_slot;
        let mut var_dfn_sl_rdb0: f64 = *var_dfn_sl_rdb0_slot;
        let mut var_dfn_sl_rdb1: f64 = *var_dfn_sl_rdb1_slot;
        let mut var_dfn_sl_rdb2: f64 = *var_dfn_sl_rdb2_slot;
        let mut var_dfn_sl_rdb3: f64 = *var_dfn_sl_rdb3_slot;
        let mut var_dfn_sl_rdn0: f64 = *var_dfn_sl_rdn0_slot;
        let mut var_dfn_sl_rdn1: f64 = *var_dfn_sl_rdn1_slot;
        let mut var_dfn_sl_rdn2: f64 = *var_dfn_sl_rdn2_slot;
        let mut var_dfn_sl_rdn3: f64 = *var_dfn_sl_rdn3_slot;
        let mut var_dfn_sl_rdn4: f64 = *var_dfn_sl_rdn4_slot;
        let mut var_dfn_sl_rdn5: f64 = *var_dfn_sl_rdn5_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_db0: f64 = *var_dfn_su_db0_slot;
        let mut var_dfn_su_db1: f64 = *var_dfn_su_db1_slot;
        let mut var_dfn_su_db2: f64 = *var_dfn_su_db2_slot;
        let mut var_dfn_su_db3: f64 = *var_dfn_su_db3_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn1: f64 = *var_dfn_su_dn1_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_dn3: f64 = *var_dfn_su_dn3_slot;
        let mut var_dfn_su_dn4: f64 = *var_dfn_su_dn4_slot;
        let mut var_dfn_su_dn5: f64 = *var_dfn_su_dn5_slot;
        let mut var_dfn_su_rdb0: f64 = *var_dfn_su_rdb0_slot;
        let mut var_dfn_su_rdb1: f64 = *var_dfn_su_rdb1_slot;
        let mut var_dfn_su_rdb2: f64 = *var_dfn_su_rdb2_slot;
        let mut var_dfn_su_rdb3: f64 = *var_dfn_su_rdb3_slot;
        let mut var_dfn_su_rdn0: f64 = *var_dfn_su_rdn0_slot;
        let mut var_dfn_su_rdn1: f64 = *var_dfn_su_rdn1_slot;
        let mut var_dfn_su_rdn2: f64 = *var_dfn_su_rdn2_slot;
        let mut var_dfn_su_rdn3: f64 = *var_dfn_su_rdn3_slot;
        let mut var_dfn_su_rdn4: f64 = *var_dfn_su_rdn4_slot;
        let mut var_dfn_su_rdn5: f64 = *var_dfn_su_rdn5_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj0_rdb0: f64 = *var_nj0_rdb0_slot;
        let mut var_nj0_rdb1: f64 = *var_nj0_rdb1_slot;
        let mut var_nj0_rdb2: f64 = *var_nj0_rdb2_slot;
        let mut var_nj0_rdb3: f64 = *var_nj0_rdb3_slot;
        let mut var_nj0_rdn0: f64 = *var_nj0_rdn0_slot;
        let mut var_nj0_rdn1: f64 = *var_nj0_rdn1_slot;
        let mut var_nj0_rdn2: f64 = *var_nj0_rdn2_slot;
        let mut var_nj0_rdn3: f64 = *var_nj0_rdn3_slot;
        let mut var_nj0_rdn4: f64 = *var_nj0_rdn4_slot;
        let mut var_nj0_rdn5: f64 = *var_nj0_rdn5_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nj1_rdb0: f64 = *var_nj1_rdb0_slot;
        let mut var_nj1_rdb1: f64 = *var_nj1_rdb1_slot;
        let mut var_nj1_rdb2: f64 = *var_nj1_rdb2_slot;
        let mut var_nj1_rdb3: f64 = *var_nj1_rdb3_slot;
        let mut var_nj1_rdn0: f64 = *var_nj1_rdn0_slot;
        let mut var_nj1_rdn1: f64 = *var_nj1_rdn1_slot;
        let mut var_nj1_rdn2: f64 = *var_nj1_rdn2_slot;
        let mut var_nj1_rdn3: f64 = *var_nj1_rdn3_slot;
        let mut var_nj1_rdn4: f64 = *var_nj1_rdn4_slot;
        let mut var_nj1_rdn5: f64 = *var_nj1_rdn5_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_nja11_rdb0: f64 = *var_nja11_rdb0_slot;
        let mut var_nja11_rdb1: f64 = *var_nja11_rdb1_slot;
        let mut var_nja11_rdb2: f64 = *var_nja11_rdb2_slot;
        let mut var_nja11_rdb3: f64 = *var_nja11_rdb3_slot;
        let mut var_nja11_rdn0: f64 = *var_nja11_rdn0_slot;
        let mut var_nja11_rdn1: f64 = *var_nja11_rdn1_slot;
        let mut var_nja11_rdn2: f64 = *var_nja11_rdn2_slot;
        let mut var_nja11_rdn3: f64 = *var_nja11_rdn3_slot;
        let mut var_nja11_rdn4: f64 = *var_nja11_rdn4_slot;
        let mut var_nja11_rdn5: f64 = *var_nja11_rdn5_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_rdb0: f64 = *var_tmf1_rdb0_slot;
        let mut var_tmf1_rdb1: f64 = *var_tmf1_rdb1_slot;
        let mut var_tmf1_rdb2: f64 = *var_tmf1_rdb2_slot;
        let mut var_tmf1_rdb3: f64 = *var_tmf1_rdb3_slot;
        let mut var_tmf1_rdn0: f64 = *var_tmf1_rdn0_slot;
        let mut var_tmf1_rdn1: f64 = *var_tmf1_rdn1_slot;
        let mut var_tmf1_rdn2: f64 = *var_tmf1_rdn2_slot;
        let mut var_tmf1_rdn3: f64 = *var_tmf1_rdn3_slot;
        let mut var_tmf1_rdn4: f64 = *var_tmf1_rdn4_slot;
        let mut var_tmf1_rdn5: f64 = *var_tmf1_rdn5_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_rdb0: f64 = *var_tmf2_rdb0_slot;
        let mut var_tmf2_rdb1: f64 = *var_tmf2_rdb1_slot;
        let mut var_tmf2_rdb2: f64 = *var_tmf2_rdb2_slot;
        let mut var_tmf2_rdb3: f64 = *var_tmf2_rdb3_slot;
        let mut var_tmf2_rdn0: f64 = *var_tmf2_rdn0_slot;
        let mut var_tmf2_rdn1: f64 = *var_tmf2_rdn1_slot;
        let mut var_tmf2_rdn2: f64 = *var_tmf2_rdn2_slot;
        let mut var_tmf2_rdn3: f64 = *var_tmf2_rdn3_slot;
        let mut var_tmf2_rdn4: f64 = *var_tmf2_rdn4_slot;
        let mut var_tmf2_rdn5: f64 = *var_tmf2_rdn5_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign31160_e46837, assign31160_e46837_d_n0, assign31160_e46837_d_n1, assign31160_e46837_d_n2, assign31160_e46837_d_n3, assign31160_e46837_d_n4, assign31160_e46837_d_n5, assign31160_e46837_d_b0, assign31160_e46837_d_b1, assign31160_e46837_d_b2, assign31160_e46837_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let (assign31160_e46835, assign31160_e46835_d_n0, assign31160_e46835_d_n1, assign31160_e46835_d_n2, assign31160_e46835_d_n3, assign31160_e46835_d_n4, assign31160_e46835_d_n5, assign31160_e46835_d_b0, assign31160_e46835_d_b1, assign31160_e46835_d_b2, assign31160_e46835_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign31160_e46834: f64 = (-var_tmf2);
                (assign31160_e46834, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign31160_e46835, assign31160_e46835_d_n0, assign31160_e46835_d_n1, assign31160_e46835_d_n2, assign31160_e46835_d_n3, assign31160_e46835_d_n4, assign31160_e46835_d_n5, assign31160_e46835_d_b0, assign31160_e46835_d_b1, assign31160_e46835_d_b2, assign31160_e46835_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31160_e46837;
        var_tmf2_dn0 = assign31160_e46837_d_n0;
        var_tmf2_dn1 = assign31160_e46837_d_n1;
        var_tmf2_dn2 = assign31160_e46837_d_n2;
        var_tmf2_dn3 = assign31160_e46837_d_n3;
        var_tmf2_dn4 = assign31160_e46837_d_n4;
        var_tmf2_dn5 = assign31160_e46837_d_n5;
        var_tmf2_db0 = assign31160_e46837_d_b0;
        var_tmf2_db1 = assign31160_e46837_d_b1;
        var_tmf2_db2 = assign31160_e46837_d_b2;
        var_tmf2_db3 = assign31160_e46837_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign31170_e46854, assign31170_e46854_d_n0, assign31170_e46854_d_n1, assign31170_e46854_d_n2, assign31170_e46854_d_n3, assign31170_e46854_d_n4, assign31170_e46854_d_n5, assign31170_e46854_d_b0, assign31170_e46854_d_b1, assign31170_e46854_d_b2, assign31170_e46854_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31170_e46849: f64 = (var_tmf1 * var_tmf1);
        let assign31170_e46851: f64 = (assign31170_e46849 + var_tmf2);
        let assign31170_e46852: f64 = (assign31170_e46851).sqrt();
        (assign31170_e46852, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign31170_e46852)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign31170_e46852)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign31170_e46852)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign31170_e46852)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign31170_e46852)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign31170_e46852)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign31170_e46852)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign31170_e46852)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign31170_e46852)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign31170_e46852)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31170_e46854;
        var_tmf2_dn0 = assign31170_e46854_d_n0;
        var_tmf2_dn1 = assign31170_e46854_d_n1;
        var_tmf2_dn2 = assign31170_e46854_d_n2;
        var_tmf2_dn3 = assign31170_e46854_d_n3;
        var_tmf2_dn4 = assign31170_e46854_d_n4;
        var_tmf2_dn5 = assign31170_e46854_d_n5;
        var_tmf2_db0 = assign31170_e46854_d_b0;
        var_tmf2_db1 = assign31170_e46854_d_b1;
        var_tmf2_db2 = assign31170_e46854_d_b2;
        var_tmf2_db3 = assign31170_e46854_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign31180_e46872, assign31180_e46872_d_n0, assign31180_e46872_d_n1, assign31180_e46872_d_n2, assign31180_e46872_d_n3, assign31180_e46872_d_n4, assign31180_e46872_d_n5, assign31180_e46872_d_b0, assign31180_e46872_d_b1, assign31180_e46872_d_b2, assign31180_e46872_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31180_e46868: f64 = (var_tmf1 / var_tmf2);
        let assign31180_e46869: f64 = (1.0 + assign31180_e46868);
        let assign31180_e46870: f64 = (0.5 * assign31180_e46869);
        (assign31180_e46870, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign31180_e46872;
        var_dfn_su_dn0 = assign31180_e46872_d_n0;
        var_dfn_su_dn1 = assign31180_e46872_d_n1;
        var_dfn_su_dn2 = assign31180_e46872_d_n2;
        var_dfn_su_dn3 = assign31180_e46872_d_n3;
        var_dfn_su_dn4 = assign31180_e46872_d_n4;
        var_dfn_su_dn5 = assign31180_e46872_d_n5;
        var_dfn_su_db0 = assign31180_e46872_d_b0;
        var_dfn_su_db1 = assign31180_e46872_d_b1;
        var_dfn_su_db2 = assign31180_e46872_d_b2;
        var_dfn_su_db3 = assign31180_e46872_d_b3;
        var_dfn_su_rv = 0.0;
        var_dfn_su_rdn0 = 0.0;
        var_dfn_su_rdn1 = 0.0;
        var_dfn_su_rdn2 = 0.0;
        var_dfn_su_rdn3 = 0.0;
        var_dfn_su_rdn4 = 0.0;
        var_dfn_su_rdn5 = 0.0;
        var_dfn_su_rdb0 = 0.0;
        var_dfn_su_rdb1 = 0.0;
        var_dfn_su_rdb2 = 0.0;
        var_dfn_su_rdb3 = 0.0;

        let (assign31190_e46890, assign31190_e46890_d_n0, assign31190_e46890_d_n1, assign31190_e46890_d_n2, assign31190_e46890_d_n3, assign31190_e46890_d_n4, assign31190_e46890_d_n5, assign31190_e46890_d_b0, assign31190_e46890_d_b1, assign31190_e46890_d_b2, assign31190_e46890_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31190_e46886: f64 = (var_tmf1 + var_tmf2);
        let assign31190_e46887: f64 = (0.5 * assign31190_e46886);
        let assign31190_e46888: f64 = (p.p85 - assign31190_e46887);
        (assign31190_e46888, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign31190_e46890;
        var_nja11_dn0 = assign31190_e46890_d_n0;
        var_nja11_dn1 = assign31190_e46890_d_n1;
        var_nja11_dn2 = assign31190_e46890_d_n2;
        var_nja11_dn3 = assign31190_e46890_d_n3;
        var_nja11_dn4 = assign31190_e46890_d_n4;
        var_nja11_dn5 = assign31190_e46890_d_n5;
        var_nja11_db0 = assign31190_e46890_d_b0;
        var_nja11_db1 = assign31190_e46890_d_b1;
        var_nja11_db2 = assign31190_e46890_d_b2;
        var_nja11_db3 = assign31190_e46890_d_b3;
        var_nja11_rv = 0.0;
        var_nja11_rdn0 = 0.0;
        var_nja11_rdn1 = 0.0;
        var_nja11_rdn2 = 0.0;
        var_nja11_rdn3 = 0.0;
        var_nja11_rdn4 = 0.0;
        var_nja11_rdn5 = 0.0;
        var_nja11_rdb0 = 0.0;
        var_nja11_rdb1 = 0.0;
        var_nja11_rdb2 = 0.0;
        var_nja11_rdb3 = 0.0;

        let (assign31200_e46906, assign31200_e46906_d_n0, assign31200_e46906_d_n1, assign31200_e46906_d_n2, assign31200_e46906_d_n3, assign31200_e46906_d_n4, assign31200_e46906_d_n5, assign31200_e46906_d_b0, assign31200_e46906_d_b1, assign31200_e46906_d_b2, assign31200_e46906_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31200_e46902: f64 = (var_nja11 - var_nfagat_i);
        let assign31200_e46904: f64 = (assign31200_e46902 - 0.01);
        (assign31200_e46904, (var_nja11_dn0 - var_nfagat_i_dn0), (var_nja11_dn1 - var_nfagat_i_dn1), (var_nja11_dn2 - var_nfagat_i_dn2), (var_nja11_dn3 - var_nfagat_i_dn3), (var_nja11_dn4 - var_nfagat_i_dn4), (var_nja11_dn5 - var_nfagat_i_dn5), (var_nja11_db0 - var_nfagat_i_db0), (var_nja11_db1 - var_nfagat_i_db1), (var_nja11_db2 - var_nfagat_i_db2), (var_nja11_db3 - var_nfagat_i_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign31200_e46906;
        var_tmf1_dn0 = assign31200_e46906_d_n0;
        var_tmf1_dn1 = assign31200_e46906_d_n1;
        var_tmf1_dn2 = assign31200_e46906_d_n2;
        var_tmf1_dn3 = assign31200_e46906_d_n3;
        var_tmf1_dn4 = assign31200_e46906_d_n4;
        var_tmf1_dn5 = assign31200_e46906_d_n5;
        var_tmf1_db0 = assign31200_e46906_d_b0;
        var_tmf1_db1 = assign31200_e46906_d_b1;
        var_tmf1_db2 = assign31200_e46906_d_b2;
        var_tmf1_db3 = assign31200_e46906_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign31210_e46922, assign31210_e46922_d_n0, assign31210_e46922_d_n1, assign31210_e46922_d_n2, assign31210_e46922_d_n3, assign31210_e46922_d_n4, assign31210_e46922_d_n5, assign31210_e46922_d_b0, assign31210_e46922_d_b1, assign31210_e46922_d_b2, assign31210_e46922_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31210_e46918: f64 = (4.0 * var_nfagat_i);
        let assign31210_e46920: f64 = (assign31210_e46918 * 0.01);
        (assign31210_e46920, ((4.0 * var_nfagat_i_dn0) * 0.01), ((4.0 * var_nfagat_i_dn1) * 0.01), ((4.0 * var_nfagat_i_dn2) * 0.01), ((4.0 * var_nfagat_i_dn3) * 0.01), ((4.0 * var_nfagat_i_dn4) * 0.01), ((4.0 * var_nfagat_i_dn5) * 0.01), ((4.0 * var_nfagat_i_db0) * 0.01), ((4.0 * var_nfagat_i_db1) * 0.01), ((4.0 * var_nfagat_i_db2) * 0.01), ((4.0 * var_nfagat_i_db3) * 0.01),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31210_e46922;
        var_tmf2_dn0 = assign31210_e46922_d_n0;
        var_tmf2_dn1 = assign31210_e46922_d_n1;
        var_tmf2_dn2 = assign31210_e46922_d_n2;
        var_tmf2_dn3 = assign31210_e46922_d_n3;
        var_tmf2_dn4 = assign31210_e46922_d_n4;
        var_tmf2_dn5 = assign31210_e46922_d_n5;
        var_tmf2_db0 = assign31210_e46922_d_b0;
        var_tmf2_db1 = assign31210_e46922_d_b1;
        var_tmf2_db2 = assign31210_e46922_d_b2;
        var_tmf2_db3 = assign31210_e46922_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign31220_e46940, assign31220_e46940_d_n0, assign31220_e46940_d_n1, assign31220_e46940_d_n2, assign31220_e46940_d_n3, assign31220_e46940_d_n4, assign31220_e46940_d_n5, assign31220_e46940_d_b0, assign31220_e46940_d_b1, assign31220_e46940_d_b2, assign31220_e46940_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let (assign31220_e46938, assign31220_e46938_d_n0, assign31220_e46938_d_n1, assign31220_e46938_d_n2, assign31220_e46938_d_n3, assign31220_e46938_d_n4, assign31220_e46938_d_n5, assign31220_e46938_d_b0, assign31220_e46938_d_b1, assign31220_e46938_d_b2, assign31220_e46938_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign31220_e46937: f64 = (-var_tmf2);
                (assign31220_e46937, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign31220_e46938, assign31220_e46938_d_n0, assign31220_e46938_d_n1, assign31220_e46938_d_n2, assign31220_e46938_d_n3, assign31220_e46938_d_n4, assign31220_e46938_d_n5, assign31220_e46938_d_b0, assign31220_e46938_d_b1, assign31220_e46938_d_b2, assign31220_e46938_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31220_e46940;
        var_tmf2_dn0 = assign31220_e46940_d_n0;
        var_tmf2_dn1 = assign31220_e46940_d_n1;
        var_tmf2_dn2 = assign31220_e46940_d_n2;
        var_tmf2_dn3 = assign31220_e46940_d_n3;
        var_tmf2_dn4 = assign31220_e46940_d_n4;
        var_tmf2_dn5 = assign31220_e46940_d_n5;
        var_tmf2_db0 = assign31220_e46940_d_b0;
        var_tmf2_db1 = assign31220_e46940_d_b1;
        var_tmf2_db2 = assign31220_e46940_d_b2;
        var_tmf2_db3 = assign31220_e46940_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign31230_e46957, assign31230_e46957_d_n0, assign31230_e46957_d_n1, assign31230_e46957_d_n2, assign31230_e46957_d_n3, assign31230_e46957_d_n4, assign31230_e46957_d_n5, assign31230_e46957_d_b0, assign31230_e46957_d_b1, assign31230_e46957_d_b2, assign31230_e46957_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31230_e46952: f64 = (var_tmf1 * var_tmf1);
        let assign31230_e46954: f64 = (assign31230_e46952 + var_tmf2);
        let assign31230_e46955: f64 = (assign31230_e46954).sqrt();
        (assign31230_e46955, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign31230_e46955)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign31230_e46955)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign31230_e46955)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign31230_e46955)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign31230_e46955)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign31230_e46955)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign31230_e46955)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign31230_e46955)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign31230_e46955)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign31230_e46955)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31230_e46957;
        var_tmf2_dn0 = assign31230_e46957_d_n0;
        var_tmf2_dn1 = assign31230_e46957_d_n1;
        var_tmf2_dn2 = assign31230_e46957_d_n2;
        var_tmf2_dn3 = assign31230_e46957_d_n3;
        var_tmf2_dn4 = assign31230_e46957_d_n4;
        var_tmf2_dn5 = assign31230_e46957_d_n5;
        var_tmf2_db0 = assign31230_e46957_d_b0;
        var_tmf2_db1 = assign31230_e46957_d_b1;
        var_tmf2_db2 = assign31230_e46957_d_b2;
        var_tmf2_db3 = assign31230_e46957_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign31240_e46975, assign31240_e46975_d_n0, assign31240_e46975_d_n1, assign31240_e46975_d_n2, assign31240_e46975_d_n3, assign31240_e46975_d_n4, assign31240_e46975_d_n5, assign31240_e46975_d_b0, assign31240_e46975_d_b1, assign31240_e46975_d_b2, assign31240_e46975_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31240_e46971: f64 = (var_tmf1 / var_tmf2);
        let assign31240_e46972: f64 = (1.0 + assign31240_e46971);
        let assign31240_e46973: f64 = (0.5 * assign31240_e46972);
        (assign31240_e46973, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign31240_e46975;
        var_dfn_sl_dn0 = assign31240_e46975_d_n0;
        var_dfn_sl_dn1 = assign31240_e46975_d_n1;
        var_dfn_sl_dn2 = assign31240_e46975_d_n2;
        var_dfn_sl_dn3 = assign31240_e46975_d_n3;
        var_dfn_sl_dn4 = assign31240_e46975_d_n4;
        var_dfn_sl_dn5 = assign31240_e46975_d_n5;
        var_dfn_sl_db0 = assign31240_e46975_d_b0;
        var_dfn_sl_db1 = assign31240_e46975_d_b1;
        var_dfn_sl_db2 = assign31240_e46975_d_b2;
        var_dfn_sl_db3 = assign31240_e46975_d_b3;
        var_dfn_sl_rv = 0.0;
        var_dfn_sl_rdn0 = 0.0;
        var_dfn_sl_rdn1 = 0.0;
        var_dfn_sl_rdn2 = 0.0;
        var_dfn_sl_rdn3 = 0.0;
        var_dfn_sl_rdn4 = 0.0;
        var_dfn_sl_rdn5 = 0.0;
        var_dfn_sl_rdb0 = 0.0;
        var_dfn_sl_rdb1 = 0.0;
        var_dfn_sl_rdb2 = 0.0;
        var_dfn_sl_rdb3 = 0.0;

        let (assign31250_e46993, assign31250_e46993_d_n0, assign31250_e46993_d_n1, assign31250_e46993_d_n2, assign31250_e46993_d_n3, assign31250_e46993_d_n4, assign31250_e46993_d_n5, assign31250_e46993_d_b0, assign31250_e46993_d_b1, assign31250_e46993_d_b2, assign31250_e46993_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31250_e46989: f64 = (var_tmf1 + var_tmf2);
        let assign31250_e46990: f64 = (0.5 * assign31250_e46989);
        let assign31250_e46991: f64 = (var_nfagat_i + assign31250_e46990);
        (assign31250_e46991, (var_nfagat_i_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_nfagat_i_dn1 + (0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (var_nfagat_i_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_nfagat_i_dn3 + (0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (var_nfagat_i_dn4 + (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_nfagat_i_dn5 + (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_nfagat_i_db0 + (0.5 * (var_tmf1_db0 + var_tmf2_db0))), (var_nfagat_i_db1 + (0.5 * (var_tmf1_db1 + var_tmf2_db1))), (var_nfagat_i_db2 + (0.5 * (var_tmf1_db2 + var_tmf2_db2))), (var_nfagat_i_db3 + (0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign31250_e46993;
        var_nj1_dn0 = assign31250_e46993_d_n0;
        var_nj1_dn1 = assign31250_e46993_d_n1;
        var_nj1_dn2 = assign31250_e46993_d_n2;
        var_nj1_dn3 = assign31250_e46993_d_n3;
        var_nj1_dn4 = assign31250_e46993_d_n4;
        var_nj1_dn5 = assign31250_e46993_d_n5;
        var_nj1_db0 = assign31250_e46993_d_b0;
        var_nj1_db1 = assign31250_e46993_d_b1;
        var_nj1_db2 = assign31250_e46993_d_b2;
        var_nj1_db3 = assign31250_e46993_d_b3;
        var_nj1_rv = 0.0;
        var_nj1_rdn0 = 0.0;
        var_nj1_rdn1 = 0.0;
        var_nj1_rdn2 = 0.0;
        var_nj1_rdn3 = 0.0;
        var_nj1_rdn4 = 0.0;
        var_nj1_rdn5 = 0.0;
        var_nj1_rdb0 = 0.0;
        var_nj1_rdb1 = 0.0;
        var_nj1_rdb2 = 0.0;
        var_nj1_rdb3 = 0.0;

        let (assign31260_e47009, assign31260_e47009_d_n0, assign31260_e47009_d_n1, assign31260_e47009_d_n2, assign31260_e47009_d_n3, assign31260_e47009_d_n4, assign31260_e47009_d_n5, assign31260_e47009_d_b0, assign31260_e47009_d_b1, assign31260_e47009_d_b2, assign31260_e47009_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31260_e47005: f64 = (p.p85 - var_nj0);
        let assign31260_e47007: f64 = (assign31260_e47005 - 0.01);
        (assign31260_e47007, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign31260_e47009;
        var_tmf1_dn0 = assign31260_e47009_d_n0;
        var_tmf1_dn1 = assign31260_e47009_d_n1;
        var_tmf1_dn2 = assign31260_e47009_d_n2;
        var_tmf1_dn3 = assign31260_e47009_d_n3;
        var_tmf1_dn4 = assign31260_e47009_d_n4;
        var_tmf1_dn5 = assign31260_e47009_d_n5;
        var_tmf1_db0 = assign31260_e47009_d_b0;
        var_tmf1_db1 = assign31260_e47009_d_b1;
        var_tmf1_db2 = assign31260_e47009_d_b2;
        var_tmf1_db3 = assign31260_e47009_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign31270_e47025, assign31270_e47025_d_n0, assign31270_e47025_d_n1, assign31270_e47025_d_n2, assign31270_e47025_d_n3, assign31270_e47025_d_n4, assign31270_e47025_d_n5, assign31270_e47025_d_b0, assign31270_e47025_d_b1, assign31270_e47025_d_b2, assign31270_e47025_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31270_e47021: f64 = (4.0 * p.p85);
        let assign31270_e47023: f64 = (assign31270_e47021 * 0.01);
        (assign31270_e47023, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31270_e47025;
        var_tmf2_dn0 = assign31270_e47025_d_n0;
        var_tmf2_dn1 = assign31270_e47025_d_n1;
        var_tmf2_dn2 = assign31270_e47025_d_n2;
        var_tmf2_dn3 = assign31270_e47025_d_n3;
        var_tmf2_dn4 = assign31270_e47025_d_n4;
        var_tmf2_dn5 = assign31270_e47025_d_n5;
        var_tmf2_db0 = assign31270_e47025_d_b0;
        var_tmf2_db1 = assign31270_e47025_d_b1;
        var_tmf2_db2 = assign31270_e47025_d_b2;
        var_tmf2_db3 = assign31270_e47025_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign31280_e47043, assign31280_e47043_d_n0, assign31280_e47043_d_n1, assign31280_e47043_d_n2, assign31280_e47043_d_n3, assign31280_e47043_d_n4, assign31280_e47043_d_n5, assign31280_e47043_d_b0, assign31280_e47043_d_b1, assign31280_e47043_d_b2, assign31280_e47043_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let (assign31280_e47041, assign31280_e47041_d_n0, assign31280_e47041_d_n1, assign31280_e47041_d_n2, assign31280_e47041_d_n3, assign31280_e47041_d_n4, assign31280_e47041_d_n5, assign31280_e47041_d_b0, assign31280_e47041_d_b1, assign31280_e47041_d_b2, assign31280_e47041_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign31280_e47040: f64 = (-var_tmf2);
                (assign31280_e47040, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign31280_e47041, assign31280_e47041_d_n0, assign31280_e47041_d_n1, assign31280_e47041_d_n2, assign31280_e47041_d_n3, assign31280_e47041_d_n4, assign31280_e47041_d_n5, assign31280_e47041_d_b0, assign31280_e47041_d_b1, assign31280_e47041_d_b2, assign31280_e47041_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31280_e47043;
        var_tmf2_dn0 = assign31280_e47043_d_n0;
        var_tmf2_dn1 = assign31280_e47043_d_n1;
        var_tmf2_dn2 = assign31280_e47043_d_n2;
        var_tmf2_dn3 = assign31280_e47043_d_n3;
        var_tmf2_dn4 = assign31280_e47043_d_n4;
        var_tmf2_dn5 = assign31280_e47043_d_n5;
        var_tmf2_db0 = assign31280_e47043_d_b0;
        var_tmf2_db1 = assign31280_e47043_d_b1;
        var_tmf2_db2 = assign31280_e47043_d_b2;
        var_tmf2_db3 = assign31280_e47043_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign31290_e47060, assign31290_e47060_d_n0, assign31290_e47060_d_n1, assign31290_e47060_d_n2, assign31290_e47060_d_n3, assign31290_e47060_d_n4, assign31290_e47060_d_n5, assign31290_e47060_d_b0, assign31290_e47060_d_b1, assign31290_e47060_d_b2, assign31290_e47060_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31290_e47055: f64 = (var_tmf1 * var_tmf1);
        let assign31290_e47057: f64 = (assign31290_e47055 + var_tmf2);
        let assign31290_e47058: f64 = (assign31290_e47057).sqrt();
        (assign31290_e47058, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign31290_e47058)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign31290_e47058)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign31290_e47058)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign31290_e47058)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign31290_e47058)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign31290_e47058)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign31290_e47058)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign31290_e47058)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign31290_e47058)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign31290_e47058)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31290_e47060;
        var_tmf2_dn0 = assign31290_e47060_d_n0;
        var_tmf2_dn1 = assign31290_e47060_d_n1;
        var_tmf2_dn2 = assign31290_e47060_d_n2;
        var_tmf2_dn3 = assign31290_e47060_d_n3;
        var_tmf2_dn4 = assign31290_e47060_d_n4;
        var_tmf2_dn5 = assign31290_e47060_d_n5;
        var_tmf2_db0 = assign31290_e47060_d_b0;
        var_tmf2_db1 = assign31290_e47060_d_b1;
        var_tmf2_db2 = assign31290_e47060_d_b2;
        var_tmf2_db3 = assign31290_e47060_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign31300_e47078, assign31300_e47078_d_n0, assign31300_e47078_d_n1, assign31300_e47078_d_n2, assign31300_e47078_d_n3, assign31300_e47078_d_n4, assign31300_e47078_d_n5, assign31300_e47078_d_b0, assign31300_e47078_d_b1, assign31300_e47078_d_b2, assign31300_e47078_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31300_e47074: f64 = (var_tmf1 + var_tmf2);
        let assign31300_e47075: f64 = (0.5 * assign31300_e47074);
        let assign31300_e47076: f64 = (p.p85 - assign31300_e47075);
        (assign31300_e47076, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign31300_e47078;
        var_nj0_dn0 = assign31300_e47078_d_n0;
        var_nj0_dn1 = assign31300_e47078_d_n1;
        var_nj0_dn2 = assign31300_e47078_d_n2;
        var_nj0_dn3 = assign31300_e47078_d_n3;
        var_nj0_dn4 = assign31300_e47078_d_n4;
        var_nj0_dn5 = assign31300_e47078_d_n5;
        var_nj0_db0 = assign31300_e47078_d_b0;
        var_nj0_db1 = assign31300_e47078_d_b1;
        var_nj0_db2 = assign31300_e47078_d_b2;
        var_nj0_db3 = assign31300_e47078_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_db0_slot = var_dfn_sl_db0;
        *var_dfn_sl_db1_slot = var_dfn_sl_db1;
        *var_dfn_sl_db2_slot = var_dfn_sl_db2;
        *var_dfn_sl_db3_slot = var_dfn_sl_db3;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn1_slot = var_dfn_sl_dn1;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_dn3_slot = var_dfn_sl_dn3;
        *var_dfn_sl_dn4_slot = var_dfn_sl_dn4;
        *var_dfn_sl_dn5_slot = var_dfn_sl_dn5;
        *var_dfn_sl_rdb0_slot = var_dfn_sl_rdb0;
        *var_dfn_sl_rdb1_slot = var_dfn_sl_rdb1;
        *var_dfn_sl_rdb2_slot = var_dfn_sl_rdb2;
        *var_dfn_sl_rdb3_slot = var_dfn_sl_rdb3;
        *var_dfn_sl_rdn0_slot = var_dfn_sl_rdn0;
        *var_dfn_sl_rdn1_slot = var_dfn_sl_rdn1;
        *var_dfn_sl_rdn2_slot = var_dfn_sl_rdn2;
        *var_dfn_sl_rdn3_slot = var_dfn_sl_rdn3;
        *var_dfn_sl_rdn4_slot = var_dfn_sl_rdn4;
        *var_dfn_sl_rdn5_slot = var_dfn_sl_rdn5;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_db0_slot = var_dfn_su_db0;
        *var_dfn_su_db1_slot = var_dfn_su_db1;
        *var_dfn_su_db2_slot = var_dfn_su_db2;
        *var_dfn_su_db3_slot = var_dfn_su_db3;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn1_slot = var_dfn_su_dn1;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_dn3_slot = var_dfn_su_dn3;
        *var_dfn_su_dn4_slot = var_dfn_su_dn4;
        *var_dfn_su_dn5_slot = var_dfn_su_dn5;
        *var_dfn_su_rdb0_slot = var_dfn_su_rdb0;
        *var_dfn_su_rdb1_slot = var_dfn_su_rdb1;
        *var_dfn_su_rdb2_slot = var_dfn_su_rdb2;
        *var_dfn_su_rdb3_slot = var_dfn_su_rdb3;
        *var_dfn_su_rdn0_slot = var_dfn_su_rdn0;
        *var_dfn_su_rdn1_slot = var_dfn_su_rdn1;
        *var_dfn_su_rdn2_slot = var_dfn_su_rdn2;
        *var_dfn_su_rdn3_slot = var_dfn_su_rdn3;
        *var_dfn_su_rdn4_slot = var_dfn_su_rdn4;
        *var_dfn_su_rdn5_slot = var_dfn_su_rdn5;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj0_rdb0_slot = var_nj0_rdb0;
        *var_nj0_rdb1_slot = var_nj0_rdb1;
        *var_nj0_rdb2_slot = var_nj0_rdb2;
        *var_nj0_rdb3_slot = var_nj0_rdb3;
        *var_nj0_rdn0_slot = var_nj0_rdn0;
        *var_nj0_rdn1_slot = var_nj0_rdn1;
        *var_nj0_rdn2_slot = var_nj0_rdn2;
        *var_nj0_rdn3_slot = var_nj0_rdn3;
        *var_nj0_rdn4_slot = var_nj0_rdn4;
        *var_nj0_rdn5_slot = var_nj0_rdn5;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nj1_rdb0_slot = var_nj1_rdb0;
        *var_nj1_rdb1_slot = var_nj1_rdb1;
        *var_nj1_rdb2_slot = var_nj1_rdb2;
        *var_nj1_rdb3_slot = var_nj1_rdb3;
        *var_nj1_rdn0_slot = var_nj1_rdn0;
        *var_nj1_rdn1_slot = var_nj1_rdn1;
        *var_nj1_rdn2_slot = var_nj1_rdn2;
        *var_nj1_rdn3_slot = var_nj1_rdn3;
        *var_nj1_rdn4_slot = var_nj1_rdn4;
        *var_nj1_rdn5_slot = var_nj1_rdn5;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_nja11_rdb0_slot = var_nja11_rdb0;
        *var_nja11_rdb1_slot = var_nja11_rdb1;
        *var_nja11_rdb2_slot = var_nja11_rdb2;
        *var_nja11_rdb3_slot = var_nja11_rdb3;
        *var_nja11_rdn0_slot = var_nja11_rdn0;
        *var_nja11_rdn1_slot = var_nja11_rdn1;
        *var_nja11_rdn2_slot = var_nja11_rdn2;
        *var_nja11_rdn3_slot = var_nja11_rdn3;
        *var_nja11_rdn4_slot = var_nja11_rdn4;
        *var_nja11_rdn5_slot = var_nja11_rdn5;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_rdb0_slot = var_tmf1_rdb0;
        *var_tmf1_rdb1_slot = var_tmf1_rdb1;
        *var_tmf1_rdb2_slot = var_tmf1_rdb2;
        *var_tmf1_rdb3_slot = var_tmf1_rdb3;
        *var_tmf1_rdn0_slot = var_tmf1_rdn0;
        *var_tmf1_rdn1_slot = var_tmf1_rdn1;
        *var_tmf1_rdn2_slot = var_tmf1_rdn2;
        *var_tmf1_rdn3_slot = var_tmf1_rdn3;
        *var_tmf1_rdn4_slot = var_tmf1_rdn4;
        *var_tmf1_rdn5_slot = var_tmf1_rdn5;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_rdb0_slot = var_tmf2_rdb0;
        *var_tmf2_rdb1_slot = var_tmf2_rdb1;
        *var_tmf2_rdb2_slot = var_tmf2_rdb2;
        *var_tmf2_rdb3_slot = var_tmf2_rdb3;
        *var_tmf2_rdn0_slot = var_tmf2_rdn0;
        *var_tmf2_rdn1_slot = var_tmf2_rdn1;
        *var_tmf2_rdn2_slot = var_tmf2_rdn2;
        *var_tmf2_rdn3_slot = var_tmf2_rdn3;
        *var_tmf2_rdn4_slot = var_tmf2_rdn4;
        *var_tmf2_rdn5_slot = var_tmf2_rdn5;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_83(
        p: &Parameters,
        var_dfn_sl: f64,
        var_dfn_sl_db0: f64,
        var_dfn_sl_db1: f64,
        var_dfn_sl_db2: f64,
        var_dfn_sl_db3: f64,
        var_dfn_sl_dn0: f64,
        var_dfn_sl_dn1: f64,
        var_dfn_sl_dn2: f64,
        var_dfn_sl_dn3: f64,
        var_dfn_sl_dn4: f64,
        var_dfn_sl_dn5: f64,
        var_dfn_su: f64,
        var_dfn_su_db0: f64,
        var_dfn_su_db1: f64,
        var_dfn_su_db2: f64,
        var_dfn_su_db3: f64,
        var_dfn_su_dn0: f64,
        var_dfn_su_dn1: f64,
        var_dfn_su_dn2: f64,
        var_dfn_su_dn3: f64,
        var_dfn_su_dn4: f64,
        var_dfn_su_dn5: f64,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard498: f64,
        var_nfagat_i: f64,
        var_nfagat_i_db0: f64,
        var_nfagat_i_db1: f64,
        var_nfagat_i_db2: f64,
        var_nfagat_i_db3: f64,
        var_nfagat_i_dn0: f64,
        var_nfagat_i_dn1: f64,
        var_nfagat_i_dn2: f64,
        var_nfagat_i_dn3: f64,
        var_nfagat_i_dn4: f64,
        var_nfagat_i_dn5: f64,
        var_njl: f64,
        var_njl_db0: f64,
        var_njl_db1: f64,
        var_njl_db2: f64,
        var_njl_db3: f64,
        var_njl_dn0: f64,
        var_njl_dn1: f64,
        var_njl_dn2: f64,
        var_njl_dn3: f64,
        var_njl_dn4: f64,
        var_njl_dn5: f64,
        var_phitdinv: f64,
        var_phitdinv_db0: f64,
        var_phitdinv_db1: f64,
        var_phitdinv_db2: f64,
        var_phitdinv_db3: f64,
        var_phitdinv_dn0: f64,
        var_phitdinv_dn1: f64,
        var_phitdinv_dn2: f64,
        var_phitdinv_dn3: f64,
        var_phitdinv_dn4: f64,
        var_phitdinv_dn5: f64,
        var_v_hk: f64,
        var_v_hk_db0: f64,
        var_v_hk_db1: f64,
        var_v_hk_db2: f64,
        var_v_hk_db3: f64,
        var_v_hk_dn0: f64,
        var_v_hk_dn1: f64,
        var_v_hk_dn2: f64,
        var_v_hk_dn3: f64,
        var_v_hk_dn4: f64,
        var_v_hk_dn5: f64,
        var_vak: f64,
        var_vak_db0: f64,
        var_vak_db1: f64,
        var_vak_db2: f64,
        var_vak_db3: f64,
        var_vak_dn0: f64,
        var_vak_dn1: f64,
        var_vak_dn2: f64,
        var_vak_dn3: f64,
        var_vak_dn4: f64,
        var_vak_dn5: f64,
        var_vha1: f64,
        var_vha1_db0: f64,
        var_vha1_db1: f64,
        var_vha1_db2: f64,
        var_vha1_db3: f64,
        var_vha1_dn0: f64,
        var_vha1_dn1: f64,
        var_vha1_dn2: f64,
        var_vha1_dn3: f64,
        var_vha1_dn4: f64,
        var_vha1_dn5: f64,
        var_vmax: f64,
        var_vmax_db0: f64,
        var_vmax_db1: f64,
        var_vmax_db2: f64,
        var_vmax_db3: f64,
        var_vmax_dn0: f64,
        var_vmax_dn1: f64,
        var_vmax_dn2: f64,
        var_vmax_dn3: f64,
        var_vmax_dn4: f64,
        var_vmax_dn5: f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_db0_slot: &mut f64,
        var_dnj1_dv_db1_slot: &mut f64,
        var_dnj1_dv_db2_slot: &mut f64,
        var_dnj1_dv_db3_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn1_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_dn3_slot: &mut f64,
        var_dnj1_dv_dn4_slot: &mut f64,
        var_dnj1_dv_dn5_slot: &mut f64,
        var_dnj1_dv_rdb0_slot: &mut f64,
        var_dnj1_dv_rdb1_slot: &mut f64,
        var_dnj1_dv_rdb2_slot: &mut f64,
        var_dnj1_dv_rdb3_slot: &mut f64,
        var_dnj1_dv_rdn0_slot: &mut f64,
        var_dnj1_dv_rdn1_slot: &mut f64,
        var_dnj1_dv_rdn2_slot: &mut f64,
        var_dnj1_dv_rdn3_slot: &mut f64,
        var_dnj1_dv_rdn4_slot: &mut f64,
        var_dnj1_dv_rdn5_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_db0_slot: &mut f64,
        var_dvmax_over_phitd_dv_db1_slot: &mut f64,
        var_dvmax_over_phitd_dv_db2_slot: &mut f64,
        var_dvmax_over_phitd_dv_db3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn1_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn4_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn5_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdb0_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdb1_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdb2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdb3_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn1_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn3_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn4_slot: &mut f64,
        var_dvmax_over_phitd_dv_rdn5_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_guard558_slot: &mut f64,
        var_guard558_db0_slot: &mut f64,
        var_guard558_db1_slot: &mut f64,
        var_guard558_db2_slot: &mut f64,
        var_guard558_db3_slot: &mut f64,
        var_guard558_dn0_slot: &mut f64,
        var_guard558_dn1_slot: &mut f64,
        var_guard558_dn2_slot: &mut f64,
        var_guard558_dn3_slot: &mut f64,
        var_guard558_dn4_slot: &mut f64,
        var_guard558_dn5_slot: &mut f64,
        var_guard558_rdb0_slot: &mut f64,
        var_guard558_rdb1_slot: &mut f64,
        var_guard558_rdb2_slot: &mut f64,
        var_guard558_rdb3_slot: &mut f64,
        var_guard558_rdn0_slot: &mut f64,
        var_guard558_rdn1_slot: &mut f64,
        var_guard558_rdn2_slot: &mut f64,
        var_guard558_rdn3_slot: &mut f64,
        var_guard558_rdn4_slot: &mut f64,
        var_guard558_rdn5_slot: &mut f64,
        var_guard558_rv_slot: &mut f64,
        var_guard559_slot: &mut f64,
        var_guard559_db0_slot: &mut f64,
        var_guard559_db1_slot: &mut f64,
        var_guard559_db2_slot: &mut f64,
        var_guard559_db3_slot: &mut f64,
        var_guard559_dn0_slot: &mut f64,
        var_guard559_dn1_slot: &mut f64,
        var_guard559_dn2_slot: &mut f64,
        var_guard559_dn3_slot: &mut f64,
        var_guard559_dn4_slot: &mut f64,
        var_guard559_dn5_slot: &mut f64,
        var_guard559_rdb0_slot: &mut f64,
        var_guard559_rdb1_slot: &mut f64,
        var_guard559_rdb2_slot: &mut f64,
        var_guard559_rdb3_slot: &mut f64,
        var_guard559_rdn0_slot: &mut f64,
        var_guard559_rdn1_slot: &mut f64,
        var_guard559_rdn2_slot: &mut f64,
        var_guard559_rdn3_slot: &mut f64,
        var_guard559_rdn4_slot: &mut f64,
        var_guard559_rdn5_slot: &mut f64,
        var_guard559_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_db0_slot: &mut f64,
        var_idmultbot_db1_slot: &mut f64,
        var_idmultbot_db2_slot: &mut f64,
        var_idmultbot_db3_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn1_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_dn3_slot: &mut f64,
        var_idmultbot_dn4_slot: &mut f64,
        var_idmultbot_dn5_slot: &mut f64,
        var_idmultbot_rdb0_slot: &mut f64,
        var_idmultbot_rdb1_slot: &mut f64,
        var_idmultbot_rdb2_slot: &mut f64,
        var_idmultbot_rdb3_slot: &mut f64,
        var_idmultbot_rdn0_slot: &mut f64,
        var_idmultbot_rdn1_slot: &mut f64,
        var_idmultbot_rdn2_slot: &mut f64,
        var_idmultbot_rdn3_slot: &mut f64,
        var_idmultbot_rdn4_slot: &mut f64,
        var_idmultbot_rdn5_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj0_rdb0_slot: &mut f64,
        var_nj0_rdb1_slot: &mut f64,
        var_nj0_rdb2_slot: &mut f64,
        var_nj0_rdb3_slot: &mut f64,
        var_nj0_rdn0_slot: &mut f64,
        var_nj0_rdn1_slot: &mut f64,
        var_nj0_rdn2_slot: &mut f64,
        var_nj0_rdn3_slot: &mut f64,
        var_nj0_rdn4_slot: &mut f64,
        var_nj0_rdn5_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nj1_rdb0_slot: &mut f64,
        var_nj1_rdb1_slot: &mut f64,
        var_nj1_rdb2_slot: &mut f64,
        var_nj1_rdb3_slot: &mut f64,
        var_nj1_rdn0_slot: &mut f64,
        var_nj1_rdn1_slot: &mut f64,
        var_nj1_rdn2_slot: &mut f64,
        var_nj1_rdn3_slot: &mut f64,
        var_nj1_rdn4_slot: &mut f64,
        var_nj1_rdn5_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nj_k0_slot: &mut f64,
        var_nj_k0_db0_slot: &mut f64,
        var_nj_k0_db1_slot: &mut f64,
        var_nj_k0_db2_slot: &mut f64,
        var_nj_k0_db3_slot: &mut f64,
        var_nj_k0_dn0_slot: &mut f64,
        var_nj_k0_dn1_slot: &mut f64,
        var_nj_k0_dn2_slot: &mut f64,
        var_nj_k0_dn3_slot: &mut f64,
        var_nj_k0_dn4_slot: &mut f64,
        var_nj_k0_dn5_slot: &mut f64,
        var_nj_k0_rdb0_slot: &mut f64,
        var_nj_k0_rdb1_slot: &mut f64,
        var_nj_k0_rdb2_slot: &mut f64,
        var_nj_k0_rdb3_slot: &mut f64,
        var_nj_k0_rdn0_slot: &mut f64,
        var_nj_k0_rdn1_slot: &mut f64,
        var_nj_k0_rdn2_slot: &mut f64,
        var_nj_k0_rdn3_slot: &mut f64,
        var_nj_k0_rdn4_slot: &mut f64,
        var_nj_k0_rdn5_slot: &mut f64,
        var_nj_k0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_rdb0_slot: &mut f64,
        var_tmf1_rdb1_slot: &mut f64,
        var_tmf1_rdb2_slot: &mut f64,
        var_tmf1_rdb3_slot: &mut f64,
        var_tmf1_rdn0_slot: &mut f64,
        var_tmf1_rdn1_slot: &mut f64,
        var_tmf1_rdn2_slot: &mut f64,
        var_tmf1_rdn3_slot: &mut f64,
        var_tmf1_rdn4_slot: &mut f64,
        var_tmf1_rdn5_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_rdb0_slot: &mut f64,
        var_tmf2_rdb1_slot: &mut f64,
        var_tmf2_rdb2_slot: &mut f64,
        var_tmf2_rdb3_slot: &mut f64,
        var_tmf2_rdn0_slot: &mut f64,
        var_tmf2_rdn1_slot: &mut f64,
        var_tmf2_rdn2_slot: &mut f64,
        var_tmf2_rdn3_slot: &mut f64,
        var_tmf2_rdn4_slot: &mut f64,
        var_tmf2_rdn5_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_db0: f64 = *var_dnj1_dv_db0_slot;
        let mut var_dnj1_dv_db1: f64 = *var_dnj1_dv_db1_slot;
        let mut var_dnj1_dv_db2: f64 = *var_dnj1_dv_db2_slot;
        let mut var_dnj1_dv_db3: f64 = *var_dnj1_dv_db3_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn1: f64 = *var_dnj1_dv_dn1_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_dn3: f64 = *var_dnj1_dv_dn3_slot;
        let mut var_dnj1_dv_dn4: f64 = *var_dnj1_dv_dn4_slot;
        let mut var_dnj1_dv_dn5: f64 = *var_dnj1_dv_dn5_slot;
        let mut var_dnj1_dv_rdb0: f64 = *var_dnj1_dv_rdb0_slot;
        let mut var_dnj1_dv_rdb1: f64 = *var_dnj1_dv_rdb1_slot;
        let mut var_dnj1_dv_rdb2: f64 = *var_dnj1_dv_rdb2_slot;
        let mut var_dnj1_dv_rdb3: f64 = *var_dnj1_dv_rdb3_slot;
        let mut var_dnj1_dv_rdn0: f64 = *var_dnj1_dv_rdn0_slot;
        let mut var_dnj1_dv_rdn1: f64 = *var_dnj1_dv_rdn1_slot;
        let mut var_dnj1_dv_rdn2: f64 = *var_dnj1_dv_rdn2_slot;
        let mut var_dnj1_dv_rdn3: f64 = *var_dnj1_dv_rdn3_slot;
        let mut var_dnj1_dv_rdn4: f64 = *var_dnj1_dv_rdn4_slot;
        let mut var_dnj1_dv_rdn5: f64 = *var_dnj1_dv_rdn5_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_db0: f64 = *var_dvmax_over_phitd_dv_db0_slot;
        let mut var_dvmax_over_phitd_dv_db1: f64 = *var_dvmax_over_phitd_dv_db1_slot;
        let mut var_dvmax_over_phitd_dv_db2: f64 = *var_dvmax_over_phitd_dv_db2_slot;
        let mut var_dvmax_over_phitd_dv_db3: f64 = *var_dvmax_over_phitd_dv_db3_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn1: f64 = *var_dvmax_over_phitd_dv_dn1_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_dn3: f64 = *var_dvmax_over_phitd_dv_dn3_slot;
        let mut var_dvmax_over_phitd_dv_dn4: f64 = *var_dvmax_over_phitd_dv_dn4_slot;
        let mut var_dvmax_over_phitd_dv_dn5: f64 = *var_dvmax_over_phitd_dv_dn5_slot;
        let mut var_dvmax_over_phitd_dv_rdb0: f64 = *var_dvmax_over_phitd_dv_rdb0_slot;
        let mut var_dvmax_over_phitd_dv_rdb1: f64 = *var_dvmax_over_phitd_dv_rdb1_slot;
        let mut var_dvmax_over_phitd_dv_rdb2: f64 = *var_dvmax_over_phitd_dv_rdb2_slot;
        let mut var_dvmax_over_phitd_dv_rdb3: f64 = *var_dvmax_over_phitd_dv_rdb3_slot;
        let mut var_dvmax_over_phitd_dv_rdn0: f64 = *var_dvmax_over_phitd_dv_rdn0_slot;
        let mut var_dvmax_over_phitd_dv_rdn1: f64 = *var_dvmax_over_phitd_dv_rdn1_slot;
        let mut var_dvmax_over_phitd_dv_rdn2: f64 = *var_dvmax_over_phitd_dv_rdn2_slot;
        let mut var_dvmax_over_phitd_dv_rdn3: f64 = *var_dvmax_over_phitd_dv_rdn3_slot;
        let mut var_dvmax_over_phitd_dv_rdn4: f64 = *var_dvmax_over_phitd_dv_rdn4_slot;
        let mut var_dvmax_over_phitd_dv_rdn5: f64 = *var_dvmax_over_phitd_dv_rdn5_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_guard558: f64 = *var_guard558_slot;
        let mut var_guard558_db0: f64 = *var_guard558_db0_slot;
        let mut var_guard558_db1: f64 = *var_guard558_db1_slot;
        let mut var_guard558_db2: f64 = *var_guard558_db2_slot;
        let mut var_guard558_db3: f64 = *var_guard558_db3_slot;
        let mut var_guard558_dn0: f64 = *var_guard558_dn0_slot;
        let mut var_guard558_dn1: f64 = *var_guard558_dn1_slot;
        let mut var_guard558_dn2: f64 = *var_guard558_dn2_slot;
        let mut var_guard558_dn3: f64 = *var_guard558_dn3_slot;
        let mut var_guard558_dn4: f64 = *var_guard558_dn4_slot;
        let mut var_guard558_dn5: f64 = *var_guard558_dn5_slot;
        let mut var_guard558_rdb0: f64 = *var_guard558_rdb0_slot;
        let mut var_guard558_rdb1: f64 = *var_guard558_rdb1_slot;
        let mut var_guard558_rdb2: f64 = *var_guard558_rdb2_slot;
        let mut var_guard558_rdb3: f64 = *var_guard558_rdb3_slot;
        let mut var_guard558_rdn0: f64 = *var_guard558_rdn0_slot;
        let mut var_guard558_rdn1: f64 = *var_guard558_rdn1_slot;
        let mut var_guard558_rdn2: f64 = *var_guard558_rdn2_slot;
        let mut var_guard558_rdn3: f64 = *var_guard558_rdn3_slot;
        let mut var_guard558_rdn4: f64 = *var_guard558_rdn4_slot;
        let mut var_guard558_rdn5: f64 = *var_guard558_rdn5_slot;
        let mut var_guard558_rv: f64 = *var_guard558_rv_slot;
        let mut var_guard559: f64 = *var_guard559_slot;
        let mut var_guard559_db0: f64 = *var_guard559_db0_slot;
        let mut var_guard559_db1: f64 = *var_guard559_db1_slot;
        let mut var_guard559_db2: f64 = *var_guard559_db2_slot;
        let mut var_guard559_db3: f64 = *var_guard559_db3_slot;
        let mut var_guard559_dn0: f64 = *var_guard559_dn0_slot;
        let mut var_guard559_dn1: f64 = *var_guard559_dn1_slot;
        let mut var_guard559_dn2: f64 = *var_guard559_dn2_slot;
        let mut var_guard559_dn3: f64 = *var_guard559_dn3_slot;
        let mut var_guard559_dn4: f64 = *var_guard559_dn4_slot;
        let mut var_guard559_dn5: f64 = *var_guard559_dn5_slot;
        let mut var_guard559_rdb0: f64 = *var_guard559_rdb0_slot;
        let mut var_guard559_rdb1: f64 = *var_guard559_rdb1_slot;
        let mut var_guard559_rdb2: f64 = *var_guard559_rdb2_slot;
        let mut var_guard559_rdb3: f64 = *var_guard559_rdb3_slot;
        let mut var_guard559_rdn0: f64 = *var_guard559_rdn0_slot;
        let mut var_guard559_rdn1: f64 = *var_guard559_rdn1_slot;
        let mut var_guard559_rdn2: f64 = *var_guard559_rdn2_slot;
        let mut var_guard559_rdn3: f64 = *var_guard559_rdn3_slot;
        let mut var_guard559_rdn4: f64 = *var_guard559_rdn4_slot;
        let mut var_guard559_rdn5: f64 = *var_guard559_rdn5_slot;
        let mut var_guard559_rv: f64 = *var_guard559_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_db0: f64 = *var_idmultbot_db0_slot;
        let mut var_idmultbot_db1: f64 = *var_idmultbot_db1_slot;
        let mut var_idmultbot_db2: f64 = *var_idmultbot_db2_slot;
        let mut var_idmultbot_db3: f64 = *var_idmultbot_db3_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn1: f64 = *var_idmultbot_dn1_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_dn3: f64 = *var_idmultbot_dn3_slot;
        let mut var_idmultbot_dn4: f64 = *var_idmultbot_dn4_slot;
        let mut var_idmultbot_dn5: f64 = *var_idmultbot_dn5_slot;
        let mut var_idmultbot_rdb0: f64 = *var_idmultbot_rdb0_slot;
        let mut var_idmultbot_rdb1: f64 = *var_idmultbot_rdb1_slot;
        let mut var_idmultbot_rdb2: f64 = *var_idmultbot_rdb2_slot;
        let mut var_idmultbot_rdb3: f64 = *var_idmultbot_rdb3_slot;
        let mut var_idmultbot_rdn0: f64 = *var_idmultbot_rdn0_slot;
        let mut var_idmultbot_rdn1: f64 = *var_idmultbot_rdn1_slot;
        let mut var_idmultbot_rdn2: f64 = *var_idmultbot_rdn2_slot;
        let mut var_idmultbot_rdn3: f64 = *var_idmultbot_rdn3_slot;
        let mut var_idmultbot_rdn4: f64 = *var_idmultbot_rdn4_slot;
        let mut var_idmultbot_rdn5: f64 = *var_idmultbot_rdn5_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj0_rdb0: f64 = *var_nj0_rdb0_slot;
        let mut var_nj0_rdb1: f64 = *var_nj0_rdb1_slot;
        let mut var_nj0_rdb2: f64 = *var_nj0_rdb2_slot;
        let mut var_nj0_rdb3: f64 = *var_nj0_rdb3_slot;
        let mut var_nj0_rdn0: f64 = *var_nj0_rdn0_slot;
        let mut var_nj0_rdn1: f64 = *var_nj0_rdn1_slot;
        let mut var_nj0_rdn2: f64 = *var_nj0_rdn2_slot;
        let mut var_nj0_rdn3: f64 = *var_nj0_rdn3_slot;
        let mut var_nj0_rdn4: f64 = *var_nj0_rdn4_slot;
        let mut var_nj0_rdn5: f64 = *var_nj0_rdn5_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nj1_rdb0: f64 = *var_nj1_rdb0_slot;
        let mut var_nj1_rdb1: f64 = *var_nj1_rdb1_slot;
        let mut var_nj1_rdb2: f64 = *var_nj1_rdb2_slot;
        let mut var_nj1_rdb3: f64 = *var_nj1_rdb3_slot;
        let mut var_nj1_rdn0: f64 = *var_nj1_rdn0_slot;
        let mut var_nj1_rdn1: f64 = *var_nj1_rdn1_slot;
        let mut var_nj1_rdn2: f64 = *var_nj1_rdn2_slot;
        let mut var_nj1_rdn3: f64 = *var_nj1_rdn3_slot;
        let mut var_nj1_rdn4: f64 = *var_nj1_rdn4_slot;
        let mut var_nj1_rdn5: f64 = *var_nj1_rdn5_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nj_k0: f64 = *var_nj_k0_slot;
        let mut var_nj_k0_db0: f64 = *var_nj_k0_db0_slot;
        let mut var_nj_k0_db1: f64 = *var_nj_k0_db1_slot;
        let mut var_nj_k0_db2: f64 = *var_nj_k0_db2_slot;
        let mut var_nj_k0_db3: f64 = *var_nj_k0_db3_slot;
        let mut var_nj_k0_dn0: f64 = *var_nj_k0_dn0_slot;
        let mut var_nj_k0_dn1: f64 = *var_nj_k0_dn1_slot;
        let mut var_nj_k0_dn2: f64 = *var_nj_k0_dn2_slot;
        let mut var_nj_k0_dn3: f64 = *var_nj_k0_dn3_slot;
        let mut var_nj_k0_dn4: f64 = *var_nj_k0_dn4_slot;
        let mut var_nj_k0_dn5: f64 = *var_nj_k0_dn5_slot;
        let mut var_nj_k0_rdb0: f64 = *var_nj_k0_rdb0_slot;
        let mut var_nj_k0_rdb1: f64 = *var_nj_k0_rdb1_slot;
        let mut var_nj_k0_rdb2: f64 = *var_nj_k0_rdb2_slot;
        let mut var_nj_k0_rdb3: f64 = *var_nj_k0_rdb3_slot;
        let mut var_nj_k0_rdn0: f64 = *var_nj_k0_rdn0_slot;
        let mut var_nj_k0_rdn1: f64 = *var_nj_k0_rdn1_slot;
        let mut var_nj_k0_rdn2: f64 = *var_nj_k0_rdn2_slot;
        let mut var_nj_k0_rdn3: f64 = *var_nj_k0_rdn3_slot;
        let mut var_nj_k0_rdn4: f64 = *var_nj_k0_rdn4_slot;
        let mut var_nj_k0_rdn5: f64 = *var_nj_k0_rdn5_slot;
        let mut var_nj_k0_rv: f64 = *var_nj_k0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_rdb0: f64 = *var_tmf1_rdb0_slot;
        let mut var_tmf1_rdb1: f64 = *var_tmf1_rdb1_slot;
        let mut var_tmf1_rdb2: f64 = *var_tmf1_rdb2_slot;
        let mut var_tmf1_rdb3: f64 = *var_tmf1_rdb3_slot;
        let mut var_tmf1_rdn0: f64 = *var_tmf1_rdn0_slot;
        let mut var_tmf1_rdn1: f64 = *var_tmf1_rdn1_slot;
        let mut var_tmf1_rdn2: f64 = *var_tmf1_rdn2_slot;
        let mut var_tmf1_rdn3: f64 = *var_tmf1_rdn3_slot;
        let mut var_tmf1_rdn4: f64 = *var_tmf1_rdn4_slot;
        let mut var_tmf1_rdn5: f64 = *var_tmf1_rdn5_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_rdb0: f64 = *var_tmf2_rdb0_slot;
        let mut var_tmf2_rdb1: f64 = *var_tmf2_rdb1_slot;
        let mut var_tmf2_rdb2: f64 = *var_tmf2_rdb2_slot;
        let mut var_tmf2_rdb3: f64 = *var_tmf2_rdb3_slot;
        let mut var_tmf2_rdn0: f64 = *var_tmf2_rdn0_slot;
        let mut var_tmf2_rdn1: f64 = *var_tmf2_rdn1_slot;
        let mut var_tmf2_rdn2: f64 = *var_tmf2_rdn2_slot;
        let mut var_tmf2_rdn3: f64 = *var_tmf2_rdn3_slot;
        let mut var_tmf2_rdn4: f64 = *var_tmf2_rdn4_slot;
        let mut var_tmf2_rdn5: f64 = *var_tmf2_rdn5_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign31310_e47094, assign31310_e47094_d_n0, assign31310_e47094_d_n1, assign31310_e47094_d_n2, assign31310_e47094_d_n3, assign31310_e47094_d_n4, assign31310_e47094_d_n5, assign31310_e47094_d_b0, assign31310_e47094_d_b1, assign31310_e47094_d_b2, assign31310_e47094_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31310_e47090: f64 = (var_nj0 - var_nfagat_i);
        let assign31310_e47092: f64 = (assign31310_e47090 - 0.01);
        (assign31310_e47092, (var_nj0_dn0 - var_nfagat_i_dn0), (var_nj0_dn1 - var_nfagat_i_dn1), (var_nj0_dn2 - var_nfagat_i_dn2), (var_nj0_dn3 - var_nfagat_i_dn3), (var_nj0_dn4 - var_nfagat_i_dn4), (var_nj0_dn5 - var_nfagat_i_dn5), (var_nj0_db0 - var_nfagat_i_db0), (var_nj0_db1 - var_nfagat_i_db1), (var_nj0_db2 - var_nfagat_i_db2), (var_nj0_db3 - var_nfagat_i_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign31310_e47094;
        var_tmf1_dn0 = assign31310_e47094_d_n0;
        var_tmf1_dn1 = assign31310_e47094_d_n1;
        var_tmf1_dn2 = assign31310_e47094_d_n2;
        var_tmf1_dn3 = assign31310_e47094_d_n3;
        var_tmf1_dn4 = assign31310_e47094_d_n4;
        var_tmf1_dn5 = assign31310_e47094_d_n5;
        var_tmf1_db0 = assign31310_e47094_d_b0;
        var_tmf1_db1 = assign31310_e47094_d_b1;
        var_tmf1_db2 = assign31310_e47094_d_b2;
        var_tmf1_db3 = assign31310_e47094_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign31320_e47110, assign31320_e47110_d_n0, assign31320_e47110_d_n1, assign31320_e47110_d_n2, assign31320_e47110_d_n3, assign31320_e47110_d_n4, assign31320_e47110_d_n5, assign31320_e47110_d_b0, assign31320_e47110_d_b1, assign31320_e47110_d_b2, assign31320_e47110_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31320_e47106: f64 = (4.0 * var_nfagat_i);
        let assign31320_e47108: f64 = (assign31320_e47106 * 0.01);
        (assign31320_e47108, ((4.0 * var_nfagat_i_dn0) * 0.01), ((4.0 * var_nfagat_i_dn1) * 0.01), ((4.0 * var_nfagat_i_dn2) * 0.01), ((4.0 * var_nfagat_i_dn3) * 0.01), ((4.0 * var_nfagat_i_dn4) * 0.01), ((4.0 * var_nfagat_i_dn5) * 0.01), ((4.0 * var_nfagat_i_db0) * 0.01), ((4.0 * var_nfagat_i_db1) * 0.01), ((4.0 * var_nfagat_i_db2) * 0.01), ((4.0 * var_nfagat_i_db3) * 0.01),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31320_e47110;
        var_tmf2_dn0 = assign31320_e47110_d_n0;
        var_tmf2_dn1 = assign31320_e47110_d_n1;
        var_tmf2_dn2 = assign31320_e47110_d_n2;
        var_tmf2_dn3 = assign31320_e47110_d_n3;
        var_tmf2_dn4 = assign31320_e47110_d_n4;
        var_tmf2_dn5 = assign31320_e47110_d_n5;
        var_tmf2_db0 = assign31320_e47110_d_b0;
        var_tmf2_db1 = assign31320_e47110_d_b1;
        var_tmf2_db2 = assign31320_e47110_d_b2;
        var_tmf2_db3 = assign31320_e47110_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign31330_e47128, assign31330_e47128_d_n0, assign31330_e47128_d_n1, assign31330_e47128_d_n2, assign31330_e47128_d_n3, assign31330_e47128_d_n4, assign31330_e47128_d_n5, assign31330_e47128_d_b0, assign31330_e47128_d_b1, assign31330_e47128_d_b2, assign31330_e47128_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let (assign31330_e47126, assign31330_e47126_d_n0, assign31330_e47126_d_n1, assign31330_e47126_d_n2, assign31330_e47126_d_n3, assign31330_e47126_d_n4, assign31330_e47126_d_n5, assign31330_e47126_d_b0, assign31330_e47126_d_b1, assign31330_e47126_d_b2, assign31330_e47126_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign31330_e47125: f64 = (-var_tmf2);
                (assign31330_e47125, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign31330_e47126, assign31330_e47126_d_n0, assign31330_e47126_d_n1, assign31330_e47126_d_n2, assign31330_e47126_d_n3, assign31330_e47126_d_n4, assign31330_e47126_d_n5, assign31330_e47126_d_b0, assign31330_e47126_d_b1, assign31330_e47126_d_b2, assign31330_e47126_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31330_e47128;
        var_tmf2_dn0 = assign31330_e47128_d_n0;
        var_tmf2_dn1 = assign31330_e47128_d_n1;
        var_tmf2_dn2 = assign31330_e47128_d_n2;
        var_tmf2_dn3 = assign31330_e47128_d_n3;
        var_tmf2_dn4 = assign31330_e47128_d_n4;
        var_tmf2_dn5 = assign31330_e47128_d_n5;
        var_tmf2_db0 = assign31330_e47128_d_b0;
        var_tmf2_db1 = assign31330_e47128_d_b1;
        var_tmf2_db2 = assign31330_e47128_d_b2;
        var_tmf2_db3 = assign31330_e47128_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign31340_e47145, assign31340_e47145_d_n0, assign31340_e47145_d_n1, assign31340_e47145_d_n2, assign31340_e47145_d_n3, assign31340_e47145_d_n4, assign31340_e47145_d_n5, assign31340_e47145_d_b0, assign31340_e47145_d_b1, assign31340_e47145_d_b2, assign31340_e47145_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31340_e47140: f64 = (var_tmf1 * var_tmf1);
        let assign31340_e47142: f64 = (assign31340_e47140 + var_tmf2);
        let assign31340_e47143: f64 = (assign31340_e47142).sqrt();
        (assign31340_e47143, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign31340_e47143)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign31340_e47143)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign31340_e47143)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign31340_e47143)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign31340_e47143)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign31340_e47143)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign31340_e47143)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign31340_e47143)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign31340_e47143)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign31340_e47143)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31340_e47145;
        var_tmf2_dn0 = assign31340_e47145_d_n0;
        var_tmf2_dn1 = assign31340_e47145_d_n1;
        var_tmf2_dn2 = assign31340_e47145_d_n2;
        var_tmf2_dn3 = assign31340_e47145_d_n3;
        var_tmf2_dn4 = assign31340_e47145_d_n4;
        var_tmf2_dn5 = assign31340_e47145_d_n5;
        var_tmf2_db0 = assign31340_e47145_d_b0;
        var_tmf2_db1 = assign31340_e47145_d_b1;
        var_tmf2_db2 = assign31340_e47145_d_b2;
        var_tmf2_db3 = assign31340_e47145_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign31350_e47163, assign31350_e47163_d_n0, assign31350_e47163_d_n1, assign31350_e47163_d_n2, assign31350_e47163_d_n3, assign31350_e47163_d_n4, assign31350_e47163_d_n5, assign31350_e47163_d_b0, assign31350_e47163_d_b1, assign31350_e47163_d_b2, assign31350_e47163_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31350_e47159: f64 = (var_tmf1 + var_tmf2);
        let assign31350_e47160: f64 = (0.5 * assign31350_e47159);
        let assign31350_e47161: f64 = (var_nfagat_i + assign31350_e47160);
        (assign31350_e47161, (var_nfagat_i_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_nfagat_i_dn1 + (0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (var_nfagat_i_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_nfagat_i_dn3 + (0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (var_nfagat_i_dn4 + (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_nfagat_i_dn5 + (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_nfagat_i_db0 + (0.5 * (var_tmf1_db0 + var_tmf2_db0))), (var_nfagat_i_db1 + (0.5 * (var_tmf1_db1 + var_tmf2_db1))), (var_nfagat_i_db2 + (0.5 * (var_tmf1_db2 + var_tmf2_db2))), (var_nfagat_i_db3 + (0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign31350_e47163;
        var_nj0_dn0 = assign31350_e47163_d_n0;
        var_nj0_dn1 = assign31350_e47163_d_n1;
        var_nj0_dn2 = assign31350_e47163_d_n2;
        var_nj0_dn3 = assign31350_e47163_d_n3;
        var_nj0_dn4 = assign31350_e47163_d_n4;
        var_nj0_dn5 = assign31350_e47163_d_n5;
        var_nj0_db0 = assign31350_e47163_d_b0;
        var_nj0_db1 = assign31350_e47163_d_b1;
        var_nj0_db2 = assign31350_e47163_d_b2;
        var_nj0_db3 = assign31350_e47163_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign31360_e47179, assign31360_e47179_d_n0, assign31360_e47179_d_n1, assign31360_e47179_d_n2, assign31360_e47179_d_n3, assign31360_e47179_d_n4, assign31360_e47179_d_n5, assign31360_e47179_d_b0, assign31360_e47179_d_b1, assign31360_e47179_d_b2, assign31360_e47179_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31360_e47175: f64 = (p.p86 * var_dfn_su);
        let assign31360_e47177: f64 = (assign31360_e47175 * var_dfn_sl);
        (assign31360_e47177, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign31360_e47179;
        var_dnj1_dv_dn0 = assign31360_e47179_d_n0;
        var_dnj1_dv_dn1 = assign31360_e47179_d_n1;
        var_dnj1_dv_dn2 = assign31360_e47179_d_n2;
        var_dnj1_dv_dn3 = assign31360_e47179_d_n3;
        var_dnj1_dv_dn4 = assign31360_e47179_d_n4;
        var_dnj1_dv_dn5 = assign31360_e47179_d_n5;
        var_dnj1_dv_db0 = assign31360_e47179_d_b0;
        var_dnj1_dv_db1 = assign31360_e47179_d_b1;
        var_dnj1_dv_db2 = assign31360_e47179_d_b2;
        var_dnj1_dv_db3 = assign31360_e47179_d_b3;
        var_dnj1_dv_rv = 0.0;
        var_dnj1_dv_rdn0 = 0.0;
        var_dnj1_dv_rdn1 = 0.0;
        var_dnj1_dv_rdn2 = 0.0;
        var_dnj1_dv_rdn3 = 0.0;
        var_dnj1_dv_rdn4 = 0.0;
        var_dnj1_dv_rdn5 = 0.0;
        var_dnj1_dv_rdb0 = 0.0;
        var_dnj1_dv_rdb1 = 0.0;
        var_dnj1_dv_rdb2 = 0.0;
        var_dnj1_dv_rdb3 = 0.0;

        let (assign31370_e47192, assign31370_e47192_d_n0, assign31370_e47192_d_n1, assign31370_e47192_d_n2, assign31370_e47192_d_n3, assign31370_e47192_d_n4, assign31370_e47192_d_n5, assign31370_e47192_d_b0, assign31370_e47192_d_b1, assign31370_e47192_d_b2, assign31370_e47192_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 == 0.0)) {
        (var_nfagat_i, var_nfagat_i_dn0, var_nfagat_i_dn1, var_nfagat_i_dn2, var_nfagat_i_dn3, var_nfagat_i_dn4, var_nfagat_i_dn5, var_nfagat_i_db0, var_nfagat_i_db1, var_nfagat_i_db2, var_nfagat_i_db3,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign31370_e47192;
        var_nj0_dn0 = assign31370_e47192_d_n0;
        var_nj0_dn1 = assign31370_e47192_d_n1;
        var_nj0_dn2 = assign31370_e47192_d_n2;
        var_nj0_dn3 = assign31370_e47192_d_n3;
        var_nj0_dn4 = assign31370_e47192_d_n4;
        var_nj0_dn5 = assign31370_e47192_d_n5;
        var_nj0_db0 = assign31370_e47192_d_b0;
        var_nj0_db1 = assign31370_e47192_d_b1;
        var_nj0_db2 = assign31370_e47192_d_b2;
        var_nj0_db3 = assign31370_e47192_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign31380_e47205, assign31380_e47205_d_n0, assign31380_e47205_d_n1, assign31380_e47205_d_n2, assign31380_e47205_d_n3, assign31380_e47205_d_n4, assign31380_e47205_d_n5, assign31380_e47205_d_b0, assign31380_e47205_d_b1, assign31380_e47205_d_b2, assign31380_e47205_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 == 0.0)) {
        (var_nfagat_i, var_nfagat_i_dn0, var_nfagat_i_dn1, var_nfagat_i_dn2, var_nfagat_i_dn3, var_nfagat_i_dn4, var_nfagat_i_dn5, var_nfagat_i_db0, var_nfagat_i_db1, var_nfagat_i_db2, var_nfagat_i_db3,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign31380_e47205;
        var_nj1_dn0 = assign31380_e47205_d_n0;
        var_nj1_dn1 = assign31380_e47205_d_n1;
        var_nj1_dn2 = assign31380_e47205_d_n2;
        var_nj1_dn3 = assign31380_e47205_d_n3;
        var_nj1_dn4 = assign31380_e47205_d_n4;
        var_nj1_dn5 = assign31380_e47205_d_n5;
        var_nj1_db0 = assign31380_e47205_d_b0;
        var_nj1_db1 = assign31380_e47205_d_b1;
        var_nj1_db2 = assign31380_e47205_d_b2;
        var_nj1_db3 = assign31380_e47205_d_b3;
        var_nj1_rv = 0.0;
        var_nj1_rdn0 = 0.0;
        var_nj1_rdn1 = 0.0;
        var_nj1_rdn2 = 0.0;
        var_nj1_rdn3 = 0.0;
        var_nj1_rdn4 = 0.0;
        var_nj1_rdn5 = 0.0;
        var_nj1_rdb0 = 0.0;
        var_nj1_rdb1 = 0.0;
        var_nj1_rdb2 = 0.0;
        var_nj1_rdb3 = 0.0;

        let (assign31390_e47218, assign31390_e47218_d_n0, assign31390_e47218_d_n1, assign31390_e47218_d_n2, assign31390_e47218_d_n3, assign31390_e47218_d_n4, assign31390_e47218_d_n5, assign31390_e47218_d_b0, assign31390_e47218_d_b1, assign31390_e47218_d_b2, assign31390_e47218_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign31390_e47218;
        var_dnj1_dv_dn0 = assign31390_e47218_d_n0;
        var_dnj1_dv_dn1 = assign31390_e47218_d_n1;
        var_dnj1_dv_dn2 = assign31390_e47218_d_n2;
        var_dnj1_dv_dn3 = assign31390_e47218_d_n3;
        var_dnj1_dv_dn4 = assign31390_e47218_d_n4;
        var_dnj1_dv_dn5 = assign31390_e47218_d_n5;
        var_dnj1_dv_db0 = assign31390_e47218_d_b0;
        var_dnj1_dv_db1 = assign31390_e47218_d_b1;
        var_dnj1_dv_db2 = assign31390_e47218_d_b2;
        var_dnj1_dv_db3 = assign31390_e47218_d_b3;
        var_dnj1_dv_rv = 0.0;
        var_dnj1_dv_rdn0 = 0.0;
        var_dnj1_dv_rdn1 = 0.0;
        var_dnj1_dv_rdn2 = 0.0;
        var_dnj1_dv_rdn3 = 0.0;
        var_dnj1_dv_rdn4 = 0.0;
        var_dnj1_dv_rdn5 = 0.0;
        var_dnj1_dv_rdb0 = 0.0;
        var_dnj1_dv_rdb1 = 0.0;
        var_dnj1_dv_rdb2 = 0.0;
        var_dnj1_dv_rdb3 = 0.0;

        let (assign31450_e47471, assign31450_e47471_d_n0, assign31450_e47471_d_n1, assign31450_e47471_d_n2, assign31450_e47471_d_n3, assign31450_e47471_d_n4, assign31450_e47471_d_n5, assign31450_e47471_d_b0, assign31450_e47471_d_b1, assign31450_e47471_d_b2, assign31450_e47471_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign31450_e47455: f64 = (var_vmax * var_dnj1_dv);
        let assign31450_e47456: f64 = (var_nj1 - assign31450_e47455);
        let assign31450_e47459: f64 = (var_nj1 * var_nj1);
        let assign31450_e47460: f64 = (assign31450_e47456 / assign31450_e47459);
        let assign31450_e47463: f64 = (var_vha1 * var_dnj1_dv);
        let assign31450_e47466: f64 = (var_nj0 * p.p85);
        let assign31450_e47467: f64 = (assign31450_e47463 / assign31450_e47466);
        let assign31450_e47468: f64 = (assign31450_e47460 + assign31450_e47467);
        let assign31450_e47469: f64 = (var_phitdinv * assign31450_e47468);
        (assign31450_e47469, ((var_phitdinv_dn0 * assign31450_e47468) + (var_phitdinv * (((((var_nj1_dn0 - ((var_vmax_dn0 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn0))) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign31450_e47459 * assign31450_e47459)) + (((((var_vha1_dn0 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn0)) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn0 * p.p85))) / (assign31450_e47466 * assign31450_e47466))))), ((var_phitdinv_dn1 * assign31450_e47468) + (var_phitdinv * (((((var_nj1_dn1 - ((var_vmax_dn1 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn1))) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign31450_e47459 * assign31450_e47459)) + (((((var_vha1_dn1 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn1)) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn1 * p.p85))) / (assign31450_e47466 * assign31450_e47466))))), ((var_phitdinv_dn2 * assign31450_e47468) + (var_phitdinv * (((((var_nj1_dn2 - ((var_vmax_dn2 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn2))) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign31450_e47459 * assign31450_e47459)) + (((((var_vha1_dn2 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn2)) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn2 * p.p85))) / (assign31450_e47466 * assign31450_e47466))))), ((var_phitdinv_dn3 * assign31450_e47468) + (var_phitdinv * (((((var_nj1_dn3 - ((var_vmax_dn3 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn3))) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign31450_e47459 * assign31450_e47459)) + (((((var_vha1_dn3 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn3)) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn3 * p.p85))) / (assign31450_e47466 * assign31450_e47466))))), ((var_phitdinv_dn4 * assign31450_e47468) + (var_phitdinv * (((((var_nj1_dn4 - ((var_vmax_dn4 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn4))) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign31450_e47459 * assign31450_e47459)) + (((((var_vha1_dn4 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn4)) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn4 * p.p85))) / (assign31450_e47466 * assign31450_e47466))))), ((var_phitdinv_dn5 * assign31450_e47468) + (var_phitdinv * (((((var_nj1_dn5 - ((var_vmax_dn5 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_dn5))) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign31450_e47459 * assign31450_e47459)) + (((((var_vha1_dn5 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_dn5)) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn5 * p.p85))) / (assign31450_e47466 * assign31450_e47466))))), ((var_phitdinv_db0 * assign31450_e47468) + (var_phitdinv * (((((var_nj1_db0 - ((var_vmax_db0 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_db0))) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign31450_e47459 * assign31450_e47459)) + (((((var_vha1_db0 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_db0)) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_db0 * p.p85))) / (assign31450_e47466 * assign31450_e47466))))), ((var_phitdinv_db1 * assign31450_e47468) + (var_phitdinv * (((((var_nj1_db1 - ((var_vmax_db1 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_db1))) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign31450_e47459 * assign31450_e47459)) + (((((var_vha1_db1 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_db1)) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_db1 * p.p85))) / (assign31450_e47466 * assign31450_e47466))))), ((var_phitdinv_db2 * assign31450_e47468) + (var_phitdinv * (((((var_nj1_db2 - ((var_vmax_db2 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_db2))) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign31450_e47459 * assign31450_e47459)) + (((((var_vha1_db2 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_db2)) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_db2 * p.p85))) / (assign31450_e47466 * assign31450_e47466))))), ((var_phitdinv_db3 * assign31450_e47468) + (var_phitdinv * (((((var_nj1_db3 - ((var_vmax_db3 * var_dnj1_dv) + (var_vmax * var_dnj1_dv_db3))) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign31450_e47459 * assign31450_e47459)) + (((((var_vha1_db3 * var_dnj1_dv) + (var_vha1 * var_dnj1_dv_db3)) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_db3 * p.p85))) / (assign31450_e47466 * assign31450_e47466))))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign31450_e47471;
        var_dvmax_over_phitd_dv_dn0 = assign31450_e47471_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign31450_e47471_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign31450_e47471_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign31450_e47471_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign31450_e47471_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign31450_e47471_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign31450_e47471_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign31450_e47471_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign31450_e47471_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign31450_e47471_d_b3;
        var_dvmax_over_phitd_dv_rv = 0.0;
        var_dvmax_over_phitd_dv_rdn0 = 0.0;
        var_dvmax_over_phitd_dv_rdn1 = 0.0;
        var_dvmax_over_phitd_dv_rdn2 = 0.0;
        var_dvmax_over_phitd_dv_rdn3 = 0.0;
        var_dvmax_over_phitd_dv_rdn4 = 0.0;
        var_dvmax_over_phitd_dv_rdn5 = 0.0;
        var_dvmax_over_phitd_dv_rdb0 = 0.0;
        var_dvmax_over_phitd_dv_rdb1 = 0.0;
        var_dvmax_over_phitd_dv_rdb2 = 0.0;
        var_dvmax_over_phitd_dv_rdb3 = 0.0;

        let (assign31470_e47498, assign31470_e47498_d_n0, assign31470_e47498_d_n1, assign31470_e47498_d_n2, assign31470_e47498_d_n3, assign31470_e47498_d_n4, assign31470_e47498_d_n5, assign31470_e47498_d_b0, assign31470_e47498_d_b1, assign31470_e47498_d_b2, assign31470_e47498_d_b3,) = {
    if ((var_guard471 == 0.0) && (var_guard479 != 0.0)) {
        let assign31470_e47496: f64 = (var_idmultbot - 1.0);
        (assign31470_e47496, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign31470_e47498;
        var_idmultbot_dn0 = assign31470_e47498_d_n0;
        var_idmultbot_dn1 = assign31470_e47498_d_n1;
        var_idmultbot_dn2 = assign31470_e47498_d_n2;
        var_idmultbot_dn3 = assign31470_e47498_d_n3;
        var_idmultbot_dn4 = assign31470_e47498_d_n4;
        var_idmultbot_dn5 = assign31470_e47498_d_n5;
        var_idmultbot_db0 = assign31470_e47498_d_b0;
        var_idmultbot_db1 = assign31470_e47498_d_b1;
        var_idmultbot_db2 = assign31470_e47498_d_b2;
        var_idmultbot_db3 = assign31470_e47498_d_b3;
        var_idmultbot_rv = 0.0;
        var_idmultbot_rdn0 = 0.0;
        var_idmultbot_rdn1 = 0.0;
        var_idmultbot_rdn2 = 0.0;
        var_idmultbot_rdn3 = 0.0;
        var_idmultbot_rdn4 = 0.0;
        var_idmultbot_rdn5 = 0.0;
        var_idmultbot_rdb0 = 0.0;
        var_idmultbot_rdb1 = 0.0;
        var_idmultbot_rdb2 = 0.0;
        var_idmultbot_rdb3 = 0.0;

        let (assign31580_e47681, assign31580_e47681_d_n0, assign31580_e47681_d_n1, assign31580_e47681_d_n2, assign31580_e47681_d_n3, assign31580_e47681_d_n4, assign31580_e47681_d_n5, assign31580_e47681_d_b0, assign31580_e47681_d_b1, assign31580_e47681_d_b2, assign31580_e47681_d_b3,) = {
    if ((var_guard471 == 0.0) && (var_guard479 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign31580_e47681;
        var_idmultbot_dn0 = assign31580_e47681_d_n0;
        var_idmultbot_dn1 = assign31580_e47681_d_n1;
        var_idmultbot_dn2 = assign31580_e47681_d_n2;
        var_idmultbot_dn3 = assign31580_e47681_d_n3;
        var_idmultbot_dn4 = assign31580_e47681_d_n4;
        var_idmultbot_dn5 = assign31580_e47681_d_n5;
        var_idmultbot_db0 = assign31580_e47681_d_b0;
        var_idmultbot_db1 = assign31580_e47681_d_b1;
        var_idmultbot_db2 = assign31580_e47681_d_b2;
        var_idmultbot_db3 = assign31580_e47681_d_b3;
        var_idmultbot_rv = 0.0;
        var_idmultbot_rdn0 = 0.0;
        var_idmultbot_rdn1 = 0.0;
        var_idmultbot_rdn2 = 0.0;
        var_idmultbot_rdn3 = 0.0;
        var_idmultbot_rdn4 = 0.0;
        var_idmultbot_rdn5 = 0.0;
        var_idmultbot_rdb0 = 0.0;
        var_idmultbot_rdb1 = 0.0;
        var_idmultbot_rdb2 = 0.0;
        var_idmultbot_rdb3 = 0.0;

        let assign34170_e51465: f64 = if p.p84 > 0.0 { 1.0 } else { 0.0 };
        var_guard558 = assign34170_e51465;
        var_guard558_dn0 = 0.0;
        var_guard558_dn1 = 0.0;
        var_guard558_dn2 = 0.0;
        var_guard558_dn3 = 0.0;
        var_guard558_dn4 = 0.0;
        var_guard558_dn5 = 0.0;
        var_guard558_db0 = 0.0;
        var_guard558_db1 = 0.0;
        var_guard558_db2 = 0.0;
        var_guard558_db3 = 0.0;
        var_guard558_rv = 0.0;
        var_guard558_rdn0 = 0.0;
        var_guard558_rdn1 = 0.0;
        var_guard558_rdn2 = 0.0;
        var_guard558_rdn3 = 0.0;
        var_guard558_rdn4 = 0.0;
        var_guard558_rdn5 = 0.0;
        var_guard558_rdb0 = 0.0;
        var_guard558_rdb1 = 0.0;
        var_guard558_rdb2 = 0.0;
        var_guard558_rdb3 = 0.0;

        let assign34180_e51468: f64 = if var_njl < p.p85 { 1.0 } else { 0.0 };
        var_guard559 = assign34180_e51468;
        var_guard559_dn0 = 0.0;
        var_guard559_dn1 = 0.0;
        var_guard559_dn2 = 0.0;
        var_guard559_dn3 = 0.0;
        var_guard559_dn4 = 0.0;
        var_guard559_dn5 = 0.0;
        var_guard559_db0 = 0.0;
        var_guard559_db1 = 0.0;
        var_guard559_db2 = 0.0;
        var_guard559_db3 = 0.0;
        var_guard559_rv = 0.0;
        var_guard559_rdn0 = 0.0;
        var_guard559_rdn1 = 0.0;
        var_guard559_rdn2 = 0.0;
        var_guard559_rdn3 = 0.0;
        var_guard559_rdn4 = 0.0;
        var_guard559_rdn5 = 0.0;
        var_guard559_rdb0 = 0.0;
        var_guard559_rdb1 = 0.0;
        var_guard559_rdb2 = 0.0;
        var_guard559_rdb3 = 0.0;

        let (assign34190_e51480, assign34190_e51480_d_n0, assign34190_e51480_d_n1, assign34190_e51480_d_n2, assign34190_e51480_d_n3, assign34190_e51480_d_n4, assign34190_e51480_d_n5, assign34190_e51480_d_b0, assign34190_e51480_d_b1, assign34190_e51480_d_b2, assign34190_e51480_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34190_e51475: f64 = (var_vak - var_v_hk);
        let assign34190_e51476: f64 = (p.p86 * assign34190_e51475);
        let assign34190_e51478: f64 = (assign34190_e51476 + var_njl);
        (assign34190_e51478, ((p.p86 * (var_vak_dn0 - var_v_hk_dn0)) + var_njl_dn0), ((p.p86 * (var_vak_dn1 - var_v_hk_dn1)) + var_njl_dn1), ((p.p86 * (var_vak_dn2 - var_v_hk_dn2)) + var_njl_dn2), ((p.p86 * (var_vak_dn3 - var_v_hk_dn3)) + var_njl_dn3), ((p.p86 * (var_vak_dn4 - var_v_hk_dn4)) + var_njl_dn4), ((p.p86 * (var_vak_dn5 - var_v_hk_dn5)) + var_njl_dn5), ((p.p86 * (var_vak_db0 - var_v_hk_db0)) + var_njl_db0), ((p.p86 * (var_vak_db1 - var_v_hk_db1)) + var_njl_db1), ((p.p86 * (var_vak_db2 - var_v_hk_db2)) + var_njl_db2), ((p.p86 * (var_vak_db3 - var_v_hk_db3)) + var_njl_db3),)
    } else {
        (var_nj_k0, var_nj_k0_dn0, var_nj_k0_dn1, var_nj_k0_dn2, var_nj_k0_dn3, var_nj_k0_dn4, var_nj_k0_dn5, var_nj_k0_db0, var_nj_k0_db1, var_nj_k0_db2, var_nj_k0_db3,)
    }
};
        var_nj_k0 = assign34190_e51480;
        var_nj_k0_dn0 = assign34190_e51480_d_n0;
        var_nj_k0_dn1 = assign34190_e51480_d_n1;
        var_nj_k0_dn2 = assign34190_e51480_d_n2;
        var_nj_k0_dn3 = assign34190_e51480_d_n3;
        var_nj_k0_dn4 = assign34190_e51480_d_n4;
        var_nj_k0_dn5 = assign34190_e51480_d_n5;
        var_nj_k0_db0 = assign34190_e51480_d_b0;
        var_nj_k0_db1 = assign34190_e51480_d_b1;
        var_nj_k0_db2 = assign34190_e51480_d_b2;
        var_nj_k0_db3 = assign34190_e51480_d_b3;
        var_nj_k0_rv = 0.0;
        var_nj_k0_rdn0 = 0.0;
        var_nj_k0_rdn1 = 0.0;
        var_nj_k0_rdn2 = 0.0;
        var_nj_k0_rdn3 = 0.0;
        var_nj_k0_rdn4 = 0.0;
        var_nj_k0_rdn5 = 0.0;
        var_nj_k0_rdb0 = 0.0;
        var_nj_k0_rdb1 = 0.0;
        var_nj_k0_rdb2 = 0.0;
        var_nj_k0_rdb3 = 0.0;

        let (assign34200_e51490, assign34200_e51490_d_n0, assign34200_e51490_d_n1, assign34200_e51490_d_n2, assign34200_e51490_d_n3, assign34200_e51490_d_n4, assign34200_e51490_d_n5, assign34200_e51490_d_b0, assign34200_e51490_d_b1, assign34200_e51490_d_b2, assign34200_e51490_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34200_e51487: f64 = (p.p86 * var_v_hk);
        let assign34200_e51488: f64 = (var_njl - assign34200_e51487);
        (assign34200_e51488, (var_njl_dn0 - (p.p86 * var_v_hk_dn0)), (var_njl_dn1 - (p.p86 * var_v_hk_dn1)), (var_njl_dn2 - (p.p86 * var_v_hk_dn2)), (var_njl_dn3 - (p.p86 * var_v_hk_dn3)), (var_njl_dn4 - (p.p86 * var_v_hk_dn4)), (var_njl_dn5 - (p.p86 * var_v_hk_dn5)), (var_njl_db0 - (p.p86 * var_v_hk_db0)), (var_njl_db1 - (p.p86 * var_v_hk_db1)), (var_njl_db2 - (p.p86 * var_v_hk_db2)), (var_njl_db3 - (p.p86 * var_v_hk_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign34200_e51490;
        var_nj0_dn0 = assign34200_e51490_d_n0;
        var_nj0_dn1 = assign34200_e51490_d_n1;
        var_nj0_dn2 = assign34200_e51490_d_n2;
        var_nj0_dn3 = assign34200_e51490_d_n3;
        var_nj0_dn4 = assign34200_e51490_d_n4;
        var_nj0_dn5 = assign34200_e51490_d_n5;
        var_nj0_db0 = assign34200_e51490_d_b0;
        var_nj0_db1 = assign34200_e51490_d_b1;
        var_nj0_db2 = assign34200_e51490_d_b2;
        var_nj0_db3 = assign34200_e51490_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_db0_slot = var_dnj1_dv_db0;
        *var_dnj1_dv_db1_slot = var_dnj1_dv_db1;
        *var_dnj1_dv_db2_slot = var_dnj1_dv_db2;
        *var_dnj1_dv_db3_slot = var_dnj1_dv_db3;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn1_slot = var_dnj1_dv_dn1;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_dn3_slot = var_dnj1_dv_dn3;
        *var_dnj1_dv_dn4_slot = var_dnj1_dv_dn4;
        *var_dnj1_dv_dn5_slot = var_dnj1_dv_dn5;
        *var_dnj1_dv_rdb0_slot = var_dnj1_dv_rdb0;
        *var_dnj1_dv_rdb1_slot = var_dnj1_dv_rdb1;
        *var_dnj1_dv_rdb2_slot = var_dnj1_dv_rdb2;
        *var_dnj1_dv_rdb3_slot = var_dnj1_dv_rdb3;
        *var_dnj1_dv_rdn0_slot = var_dnj1_dv_rdn0;
        *var_dnj1_dv_rdn1_slot = var_dnj1_dv_rdn1;
        *var_dnj1_dv_rdn2_slot = var_dnj1_dv_rdn2;
        *var_dnj1_dv_rdn3_slot = var_dnj1_dv_rdn3;
        *var_dnj1_dv_rdn4_slot = var_dnj1_dv_rdn4;
        *var_dnj1_dv_rdn5_slot = var_dnj1_dv_rdn5;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_db0_slot = var_dvmax_over_phitd_dv_db0;
        *var_dvmax_over_phitd_dv_db1_slot = var_dvmax_over_phitd_dv_db1;
        *var_dvmax_over_phitd_dv_db2_slot = var_dvmax_over_phitd_dv_db2;
        *var_dvmax_over_phitd_dv_db3_slot = var_dvmax_over_phitd_dv_db3;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn1_slot = var_dvmax_over_phitd_dv_dn1;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_dn3_slot = var_dvmax_over_phitd_dv_dn3;
        *var_dvmax_over_phitd_dv_dn4_slot = var_dvmax_over_phitd_dv_dn4;
        *var_dvmax_over_phitd_dv_dn5_slot = var_dvmax_over_phitd_dv_dn5;
        *var_dvmax_over_phitd_dv_rdb0_slot = var_dvmax_over_phitd_dv_rdb0;
        *var_dvmax_over_phitd_dv_rdb1_slot = var_dvmax_over_phitd_dv_rdb1;
        *var_dvmax_over_phitd_dv_rdb2_slot = var_dvmax_over_phitd_dv_rdb2;
        *var_dvmax_over_phitd_dv_rdb3_slot = var_dvmax_over_phitd_dv_rdb3;
        *var_dvmax_over_phitd_dv_rdn0_slot = var_dvmax_over_phitd_dv_rdn0;
        *var_dvmax_over_phitd_dv_rdn1_slot = var_dvmax_over_phitd_dv_rdn1;
        *var_dvmax_over_phitd_dv_rdn2_slot = var_dvmax_over_phitd_dv_rdn2;
        *var_dvmax_over_phitd_dv_rdn3_slot = var_dvmax_over_phitd_dv_rdn3;
        *var_dvmax_over_phitd_dv_rdn4_slot = var_dvmax_over_phitd_dv_rdn4;
        *var_dvmax_over_phitd_dv_rdn5_slot = var_dvmax_over_phitd_dv_rdn5;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_guard558_slot = var_guard558;
        *var_guard558_db0_slot = var_guard558_db0;
        *var_guard558_db1_slot = var_guard558_db1;
        *var_guard558_db2_slot = var_guard558_db2;
        *var_guard558_db3_slot = var_guard558_db3;
        *var_guard558_dn0_slot = var_guard558_dn0;
        *var_guard558_dn1_slot = var_guard558_dn1;
        *var_guard558_dn2_slot = var_guard558_dn2;
        *var_guard558_dn3_slot = var_guard558_dn3;
        *var_guard558_dn4_slot = var_guard558_dn4;
        *var_guard558_dn5_slot = var_guard558_dn5;
        *var_guard558_rdb0_slot = var_guard558_rdb0;
        *var_guard558_rdb1_slot = var_guard558_rdb1;
        *var_guard558_rdb2_slot = var_guard558_rdb2;
        *var_guard558_rdb3_slot = var_guard558_rdb3;
        *var_guard558_rdn0_slot = var_guard558_rdn0;
        *var_guard558_rdn1_slot = var_guard558_rdn1;
        *var_guard558_rdn2_slot = var_guard558_rdn2;
        *var_guard558_rdn3_slot = var_guard558_rdn3;
        *var_guard558_rdn4_slot = var_guard558_rdn4;
        *var_guard558_rdn5_slot = var_guard558_rdn5;
        *var_guard558_rv_slot = var_guard558_rv;
        *var_guard559_slot = var_guard559;
        *var_guard559_db0_slot = var_guard559_db0;
        *var_guard559_db1_slot = var_guard559_db1;
        *var_guard559_db2_slot = var_guard559_db2;
        *var_guard559_db3_slot = var_guard559_db3;
        *var_guard559_dn0_slot = var_guard559_dn0;
        *var_guard559_dn1_slot = var_guard559_dn1;
        *var_guard559_dn2_slot = var_guard559_dn2;
        *var_guard559_dn3_slot = var_guard559_dn3;
        *var_guard559_dn4_slot = var_guard559_dn4;
        *var_guard559_dn5_slot = var_guard559_dn5;
        *var_guard559_rdb0_slot = var_guard559_rdb0;
        *var_guard559_rdb1_slot = var_guard559_rdb1;
        *var_guard559_rdb2_slot = var_guard559_rdb2;
        *var_guard559_rdb3_slot = var_guard559_rdb3;
        *var_guard559_rdn0_slot = var_guard559_rdn0;
        *var_guard559_rdn1_slot = var_guard559_rdn1;
        *var_guard559_rdn2_slot = var_guard559_rdn2;
        *var_guard559_rdn3_slot = var_guard559_rdn3;
        *var_guard559_rdn4_slot = var_guard559_rdn4;
        *var_guard559_rdn5_slot = var_guard559_rdn5;
        *var_guard559_rv_slot = var_guard559_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_db0_slot = var_idmultbot_db0;
        *var_idmultbot_db1_slot = var_idmultbot_db1;
        *var_idmultbot_db2_slot = var_idmultbot_db2;
        *var_idmultbot_db3_slot = var_idmultbot_db3;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn1_slot = var_idmultbot_dn1;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_dn3_slot = var_idmultbot_dn3;
        *var_idmultbot_dn4_slot = var_idmultbot_dn4;
        *var_idmultbot_dn5_slot = var_idmultbot_dn5;
        *var_idmultbot_rdb0_slot = var_idmultbot_rdb0;
        *var_idmultbot_rdb1_slot = var_idmultbot_rdb1;
        *var_idmultbot_rdb2_slot = var_idmultbot_rdb2;
        *var_idmultbot_rdb3_slot = var_idmultbot_rdb3;
        *var_idmultbot_rdn0_slot = var_idmultbot_rdn0;
        *var_idmultbot_rdn1_slot = var_idmultbot_rdn1;
        *var_idmultbot_rdn2_slot = var_idmultbot_rdn2;
        *var_idmultbot_rdn3_slot = var_idmultbot_rdn3;
        *var_idmultbot_rdn4_slot = var_idmultbot_rdn4;
        *var_idmultbot_rdn5_slot = var_idmultbot_rdn5;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj0_rdb0_slot = var_nj0_rdb0;
        *var_nj0_rdb1_slot = var_nj0_rdb1;
        *var_nj0_rdb2_slot = var_nj0_rdb2;
        *var_nj0_rdb3_slot = var_nj0_rdb3;
        *var_nj0_rdn0_slot = var_nj0_rdn0;
        *var_nj0_rdn1_slot = var_nj0_rdn1;
        *var_nj0_rdn2_slot = var_nj0_rdn2;
        *var_nj0_rdn3_slot = var_nj0_rdn3;
        *var_nj0_rdn4_slot = var_nj0_rdn4;
        *var_nj0_rdn5_slot = var_nj0_rdn5;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nj1_rdb0_slot = var_nj1_rdb0;
        *var_nj1_rdb1_slot = var_nj1_rdb1;
        *var_nj1_rdb2_slot = var_nj1_rdb2;
        *var_nj1_rdb3_slot = var_nj1_rdb3;
        *var_nj1_rdn0_slot = var_nj1_rdn0;
        *var_nj1_rdn1_slot = var_nj1_rdn1;
        *var_nj1_rdn2_slot = var_nj1_rdn2;
        *var_nj1_rdn3_slot = var_nj1_rdn3;
        *var_nj1_rdn4_slot = var_nj1_rdn4;
        *var_nj1_rdn5_slot = var_nj1_rdn5;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nj_k0_slot = var_nj_k0;
        *var_nj_k0_db0_slot = var_nj_k0_db0;
        *var_nj_k0_db1_slot = var_nj_k0_db1;
        *var_nj_k0_db2_slot = var_nj_k0_db2;
        *var_nj_k0_db3_slot = var_nj_k0_db3;
        *var_nj_k0_dn0_slot = var_nj_k0_dn0;
        *var_nj_k0_dn1_slot = var_nj_k0_dn1;
        *var_nj_k0_dn2_slot = var_nj_k0_dn2;
        *var_nj_k0_dn3_slot = var_nj_k0_dn3;
        *var_nj_k0_dn4_slot = var_nj_k0_dn4;
        *var_nj_k0_dn5_slot = var_nj_k0_dn5;
        *var_nj_k0_rdb0_slot = var_nj_k0_rdb0;
        *var_nj_k0_rdb1_slot = var_nj_k0_rdb1;
        *var_nj_k0_rdb2_slot = var_nj_k0_rdb2;
        *var_nj_k0_rdb3_slot = var_nj_k0_rdb3;
        *var_nj_k0_rdn0_slot = var_nj_k0_rdn0;
        *var_nj_k0_rdn1_slot = var_nj_k0_rdn1;
        *var_nj_k0_rdn2_slot = var_nj_k0_rdn2;
        *var_nj_k0_rdn3_slot = var_nj_k0_rdn3;
        *var_nj_k0_rdn4_slot = var_nj_k0_rdn4;
        *var_nj_k0_rdn5_slot = var_nj_k0_rdn5;
        *var_nj_k0_rv_slot = var_nj_k0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_rdb0_slot = var_tmf1_rdb0;
        *var_tmf1_rdb1_slot = var_tmf1_rdb1;
        *var_tmf1_rdb2_slot = var_tmf1_rdb2;
        *var_tmf1_rdb3_slot = var_tmf1_rdb3;
        *var_tmf1_rdn0_slot = var_tmf1_rdn0;
        *var_tmf1_rdn1_slot = var_tmf1_rdn1;
        *var_tmf1_rdn2_slot = var_tmf1_rdn2;
        *var_tmf1_rdn3_slot = var_tmf1_rdn3;
        *var_tmf1_rdn4_slot = var_tmf1_rdn4;
        *var_tmf1_rdn5_slot = var_tmf1_rdn5;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_rdb0_slot = var_tmf2_rdb0;
        *var_tmf2_rdb1_slot = var_tmf2_rdb1;
        *var_tmf2_rdb2_slot = var_tmf2_rdb2;
        *var_tmf2_rdb3_slot = var_tmf2_rdb3;
        *var_tmf2_rdn0_slot = var_tmf2_rdn0;
        *var_tmf2_rdn1_slot = var_tmf2_rdn1;
        *var_tmf2_rdn2_slot = var_tmf2_rdn2;
        *var_tmf2_rdn3_slot = var_tmf2_rdn3;
        *var_tmf2_rdn4_slot = var_tmf2_rdn4;
        *var_tmf2_rdn5_slot = var_tmf2_rdn5;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_84(
        p: &Parameters,
        var_guard558: f64,
        var_guard559: f64,
        var_nj_k0: f64,
        var_nj_k0_db0: f64,
        var_nj_k0_db1: f64,
        var_nj_k0_db2: f64,
        var_nj_k0_db3: f64,
        var_nj_k0_dn0: f64,
        var_nj_k0_dn1: f64,
        var_nj_k0_dn2: f64,
        var_nj_k0_dn3: f64,
        var_nj_k0_dn4: f64,
        var_nj_k0_dn5: f64,
        var_njl: f64,
        var_njl_db0: f64,
        var_njl_db1: f64,
        var_njl_db2: f64,
        var_njl_db3: f64,
        var_njl_dn0: f64,
        var_njl_dn1: f64,
        var_njl_dn2: f64,
        var_njl_dn3: f64,
        var_njl_dn4: f64,
        var_njl_dn5: f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj0_rdb0_slot: &mut f64,
        var_nj0_rdb1_slot: &mut f64,
        var_nj0_rdb2_slot: &mut f64,
        var_nj0_rdb3_slot: &mut f64,
        var_nj0_rdn0_slot: &mut f64,
        var_nj0_rdn1_slot: &mut f64,
        var_nj0_rdn2_slot: &mut f64,
        var_nj0_rdn3_slot: &mut f64,
        var_nj0_rdn4_slot: &mut f64,
        var_nj0_rdn5_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj_k_slot: &mut f64,
        var_nj_k1_slot: &mut f64,
        var_nj_k1_db0_slot: &mut f64,
        var_nj_k1_db1_slot: &mut f64,
        var_nj_k1_db2_slot: &mut f64,
        var_nj_k1_db3_slot: &mut f64,
        var_nj_k1_dn0_slot: &mut f64,
        var_nj_k1_dn1_slot: &mut f64,
        var_nj_k1_dn2_slot: &mut f64,
        var_nj_k1_dn3_slot: &mut f64,
        var_nj_k1_dn4_slot: &mut f64,
        var_nj_k1_dn5_slot: &mut f64,
        var_nj_k1_rdb0_slot: &mut f64,
        var_nj_k1_rdb1_slot: &mut f64,
        var_nj_k1_rdb2_slot: &mut f64,
        var_nj_k1_rdb3_slot: &mut f64,
        var_nj_k1_rdn0_slot: &mut f64,
        var_nj_k1_rdn1_slot: &mut f64,
        var_nj_k1_rdn2_slot: &mut f64,
        var_nj_k1_rdn3_slot: &mut f64,
        var_nj_k1_rdn4_slot: &mut f64,
        var_nj_k1_rdn5_slot: &mut f64,
        var_nj_k1_rv_slot: &mut f64,
        var_nj_k_db0_slot: &mut f64,
        var_nj_k_db1_slot: &mut f64,
        var_nj_k_db2_slot: &mut f64,
        var_nj_k_db3_slot: &mut f64,
        var_nj_k_dn0_slot: &mut f64,
        var_nj_k_dn1_slot: &mut f64,
        var_nj_k_dn2_slot: &mut f64,
        var_nj_k_dn3_slot: &mut f64,
        var_nj_k_dn4_slot: &mut f64,
        var_nj_k_dn5_slot: &mut f64,
        var_nj_k_rdb0_slot: &mut f64,
        var_nj_k_rdb1_slot: &mut f64,
        var_nj_k_rdb2_slot: &mut f64,
        var_nj_k_rdb3_slot: &mut f64,
        var_nj_k_rdn0_slot: &mut f64,
        var_nj_k_rdn1_slot: &mut f64,
        var_nj_k_rdn2_slot: &mut f64,
        var_nj_k_rdn3_slot: &mut f64,
        var_nj_k_rdn4_slot: &mut f64,
        var_nj_k_rdn5_slot: &mut f64,
        var_nj_k_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_rdb0_slot: &mut f64,
        var_tmf1_rdb1_slot: &mut f64,
        var_tmf1_rdb2_slot: &mut f64,
        var_tmf1_rdb3_slot: &mut f64,
        var_tmf1_rdn0_slot: &mut f64,
        var_tmf1_rdn1_slot: &mut f64,
        var_tmf1_rdn2_slot: &mut f64,
        var_tmf1_rdn3_slot: &mut f64,
        var_tmf1_rdn4_slot: &mut f64,
        var_tmf1_rdn5_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_rdb0_slot: &mut f64,
        var_tmf2_rdb1_slot: &mut f64,
        var_tmf2_rdb2_slot: &mut f64,
        var_tmf2_rdb3_slot: &mut f64,
        var_tmf2_rdn0_slot: &mut f64,
        var_tmf2_rdn1_slot: &mut f64,
        var_tmf2_rdn2_slot: &mut f64,
        var_tmf2_rdn3_slot: &mut f64,
        var_tmf2_rdn4_slot: &mut f64,
        var_tmf2_rdn5_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj0_rdb0: f64 = *var_nj0_rdb0_slot;
        let mut var_nj0_rdb1: f64 = *var_nj0_rdb1_slot;
        let mut var_nj0_rdb2: f64 = *var_nj0_rdb2_slot;
        let mut var_nj0_rdb3: f64 = *var_nj0_rdb3_slot;
        let mut var_nj0_rdn0: f64 = *var_nj0_rdn0_slot;
        let mut var_nj0_rdn1: f64 = *var_nj0_rdn1_slot;
        let mut var_nj0_rdn2: f64 = *var_nj0_rdn2_slot;
        let mut var_nj0_rdn3: f64 = *var_nj0_rdn3_slot;
        let mut var_nj0_rdn4: f64 = *var_nj0_rdn4_slot;
        let mut var_nj0_rdn5: f64 = *var_nj0_rdn5_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj_k: f64 = *var_nj_k_slot;
        let mut var_nj_k1: f64 = *var_nj_k1_slot;
        let mut var_nj_k1_db0: f64 = *var_nj_k1_db0_slot;
        let mut var_nj_k1_db1: f64 = *var_nj_k1_db1_slot;
        let mut var_nj_k1_db2: f64 = *var_nj_k1_db2_slot;
        let mut var_nj_k1_db3: f64 = *var_nj_k1_db3_slot;
        let mut var_nj_k1_dn0: f64 = *var_nj_k1_dn0_slot;
        let mut var_nj_k1_dn1: f64 = *var_nj_k1_dn1_slot;
        let mut var_nj_k1_dn2: f64 = *var_nj_k1_dn2_slot;
        let mut var_nj_k1_dn3: f64 = *var_nj_k1_dn3_slot;
        let mut var_nj_k1_dn4: f64 = *var_nj_k1_dn4_slot;
        let mut var_nj_k1_dn5: f64 = *var_nj_k1_dn5_slot;
        let mut var_nj_k1_rdb0: f64 = *var_nj_k1_rdb0_slot;
        let mut var_nj_k1_rdb1: f64 = *var_nj_k1_rdb1_slot;
        let mut var_nj_k1_rdb2: f64 = *var_nj_k1_rdb2_slot;
        let mut var_nj_k1_rdb3: f64 = *var_nj_k1_rdb3_slot;
        let mut var_nj_k1_rdn0: f64 = *var_nj_k1_rdn0_slot;
        let mut var_nj_k1_rdn1: f64 = *var_nj_k1_rdn1_slot;
        let mut var_nj_k1_rdn2: f64 = *var_nj_k1_rdn2_slot;
        let mut var_nj_k1_rdn3: f64 = *var_nj_k1_rdn3_slot;
        let mut var_nj_k1_rdn4: f64 = *var_nj_k1_rdn4_slot;
        let mut var_nj_k1_rdn5: f64 = *var_nj_k1_rdn5_slot;
        let mut var_nj_k1_rv: f64 = *var_nj_k1_rv_slot;
        let mut var_nj_k_db0: f64 = *var_nj_k_db0_slot;
        let mut var_nj_k_db1: f64 = *var_nj_k_db1_slot;
        let mut var_nj_k_db2: f64 = *var_nj_k_db2_slot;
        let mut var_nj_k_db3: f64 = *var_nj_k_db3_slot;
        let mut var_nj_k_dn0: f64 = *var_nj_k_dn0_slot;
        let mut var_nj_k_dn1: f64 = *var_nj_k_dn1_slot;
        let mut var_nj_k_dn2: f64 = *var_nj_k_dn2_slot;
        let mut var_nj_k_dn3: f64 = *var_nj_k_dn3_slot;
        let mut var_nj_k_dn4: f64 = *var_nj_k_dn4_slot;
        let mut var_nj_k_dn5: f64 = *var_nj_k_dn5_slot;
        let mut var_nj_k_rdb0: f64 = *var_nj_k_rdb0_slot;
        let mut var_nj_k_rdb1: f64 = *var_nj_k_rdb1_slot;
        let mut var_nj_k_rdb2: f64 = *var_nj_k_rdb2_slot;
        let mut var_nj_k_rdb3: f64 = *var_nj_k_rdb3_slot;
        let mut var_nj_k_rdn0: f64 = *var_nj_k_rdn0_slot;
        let mut var_nj_k_rdn1: f64 = *var_nj_k_rdn1_slot;
        let mut var_nj_k_rdn2: f64 = *var_nj_k_rdn2_slot;
        let mut var_nj_k_rdn3: f64 = *var_nj_k_rdn3_slot;
        let mut var_nj_k_rdn4: f64 = *var_nj_k_rdn4_slot;
        let mut var_nj_k_rdn5: f64 = *var_nj_k_rdn5_slot;
        let mut var_nj_k_rv: f64 = *var_nj_k_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_rdb0: f64 = *var_tmf1_rdb0_slot;
        let mut var_tmf1_rdb1: f64 = *var_tmf1_rdb1_slot;
        let mut var_tmf1_rdb2: f64 = *var_tmf1_rdb2_slot;
        let mut var_tmf1_rdb3: f64 = *var_tmf1_rdb3_slot;
        let mut var_tmf1_rdn0: f64 = *var_tmf1_rdn0_slot;
        let mut var_tmf1_rdn1: f64 = *var_tmf1_rdn1_slot;
        let mut var_tmf1_rdn2: f64 = *var_tmf1_rdn2_slot;
        let mut var_tmf1_rdn3: f64 = *var_tmf1_rdn3_slot;
        let mut var_tmf1_rdn4: f64 = *var_tmf1_rdn4_slot;
        let mut var_tmf1_rdn5: f64 = *var_tmf1_rdn5_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_rdb0: f64 = *var_tmf2_rdb0_slot;
        let mut var_tmf2_rdb1: f64 = *var_tmf2_rdb1_slot;
        let mut var_tmf2_rdb2: f64 = *var_tmf2_rdb2_slot;
        let mut var_tmf2_rdb3: f64 = *var_tmf2_rdb3_slot;
        let mut var_tmf2_rdn0: f64 = *var_tmf2_rdn0_slot;
        let mut var_tmf2_rdn1: f64 = *var_tmf2_rdn1_slot;
        let mut var_tmf2_rdn2: f64 = *var_tmf2_rdn2_slot;
        let mut var_tmf2_rdn3: f64 = *var_tmf2_rdn3_slot;
        let mut var_tmf2_rdn4: f64 = *var_tmf2_rdn4_slot;
        let mut var_tmf2_rdn5: f64 = *var_tmf2_rdn5_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign34210_e51500, assign34210_e51500_d_n0, assign34210_e51500_d_n1, assign34210_e51500_d_n2, assign34210_e51500_d_n3, assign34210_e51500_d_n4, assign34210_e51500_d_n5, assign34210_e51500_d_b0, assign34210_e51500_d_b1, assign34210_e51500_d_b2, assign34210_e51500_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34210_e51496: f64 = (p.p85 - var_nj_k0);
        let assign34210_e51498: f64 = (assign34210_e51496 - 0.01);
        (assign34210_e51498, (-var_nj_k0_dn0), (-var_nj_k0_dn1), (-var_nj_k0_dn2), (-var_nj_k0_dn3), (-var_nj_k0_dn4), (-var_nj_k0_dn5), (-var_nj_k0_db0), (-var_nj_k0_db1), (-var_nj_k0_db2), (-var_nj_k0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign34210_e51500;
        var_tmf1_dn0 = assign34210_e51500_d_n0;
        var_tmf1_dn1 = assign34210_e51500_d_n1;
        var_tmf1_dn2 = assign34210_e51500_d_n2;
        var_tmf1_dn3 = assign34210_e51500_d_n3;
        var_tmf1_dn4 = assign34210_e51500_d_n4;
        var_tmf1_dn5 = assign34210_e51500_d_n5;
        var_tmf1_db0 = assign34210_e51500_d_b0;
        var_tmf1_db1 = assign34210_e51500_d_b1;
        var_tmf1_db2 = assign34210_e51500_d_b2;
        var_tmf1_db3 = assign34210_e51500_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign34220_e51510, assign34220_e51510_d_n0, assign34220_e51510_d_n1, assign34220_e51510_d_n2, assign34220_e51510_d_n3, assign34220_e51510_d_n4, assign34220_e51510_d_n5, assign34220_e51510_d_b0, assign34220_e51510_d_b1, assign34220_e51510_d_b2, assign34220_e51510_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34220_e51506: f64 = (4.0 * p.p85);
        let assign34220_e51508: f64 = (assign34220_e51506 * 0.01);
        (assign34220_e51508, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34220_e51510;
        var_tmf2_dn0 = assign34220_e51510_d_n0;
        var_tmf2_dn1 = assign34220_e51510_d_n1;
        var_tmf2_dn2 = assign34220_e51510_d_n2;
        var_tmf2_dn3 = assign34220_e51510_d_n3;
        var_tmf2_dn4 = assign34220_e51510_d_n4;
        var_tmf2_dn5 = assign34220_e51510_d_n5;
        var_tmf2_db0 = assign34220_e51510_d_b0;
        var_tmf2_db1 = assign34220_e51510_d_b1;
        var_tmf2_db2 = assign34220_e51510_d_b2;
        var_tmf2_db3 = assign34220_e51510_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34230_e51522, assign34230_e51522_d_n0, assign34230_e51522_d_n1, assign34230_e51522_d_n2, assign34230_e51522_d_n3, assign34230_e51522_d_n4, assign34230_e51522_d_n5, assign34230_e51522_d_b0, assign34230_e51522_d_b1, assign34230_e51522_d_b2, assign34230_e51522_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let (assign34230_e51520, assign34230_e51520_d_n0, assign34230_e51520_d_n1, assign34230_e51520_d_n2, assign34230_e51520_d_n3, assign34230_e51520_d_n4, assign34230_e51520_d_n5, assign34230_e51520_d_b0, assign34230_e51520_d_b1, assign34230_e51520_d_b2, assign34230_e51520_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign34230_e51519: f64 = (-var_tmf2);
                (assign34230_e51519, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign34230_e51520, assign34230_e51520_d_n0, assign34230_e51520_d_n1, assign34230_e51520_d_n2, assign34230_e51520_d_n3, assign34230_e51520_d_n4, assign34230_e51520_d_n5, assign34230_e51520_d_b0, assign34230_e51520_d_b1, assign34230_e51520_d_b2, assign34230_e51520_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34230_e51522;
        var_tmf2_dn0 = assign34230_e51522_d_n0;
        var_tmf2_dn1 = assign34230_e51522_d_n1;
        var_tmf2_dn2 = assign34230_e51522_d_n2;
        var_tmf2_dn3 = assign34230_e51522_d_n3;
        var_tmf2_dn4 = assign34230_e51522_d_n4;
        var_tmf2_dn5 = assign34230_e51522_d_n5;
        var_tmf2_db0 = assign34230_e51522_d_b0;
        var_tmf2_db1 = assign34230_e51522_d_b1;
        var_tmf2_db2 = assign34230_e51522_d_b2;
        var_tmf2_db3 = assign34230_e51522_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34240_e51533, assign34240_e51533_d_n0, assign34240_e51533_d_n1, assign34240_e51533_d_n2, assign34240_e51533_d_n3, assign34240_e51533_d_n4, assign34240_e51533_d_n5, assign34240_e51533_d_b0, assign34240_e51533_d_b1, assign34240_e51533_d_b2, assign34240_e51533_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34240_e51528: f64 = (var_tmf1 * var_tmf1);
        let assign34240_e51530: f64 = (assign34240_e51528 + var_tmf2);
        let assign34240_e51531: f64 = (assign34240_e51530).sqrt();
        (assign34240_e51531, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34240_e51531)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign34240_e51531)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34240_e51531)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign34240_e51531)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign34240_e51531)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign34240_e51531)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign34240_e51531)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign34240_e51531)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign34240_e51531)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign34240_e51531)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34240_e51533;
        var_tmf2_dn0 = assign34240_e51533_d_n0;
        var_tmf2_dn1 = assign34240_e51533_d_n1;
        var_tmf2_dn2 = assign34240_e51533_d_n2;
        var_tmf2_dn3 = assign34240_e51533_d_n3;
        var_tmf2_dn4 = assign34240_e51533_d_n4;
        var_tmf2_dn5 = assign34240_e51533_d_n5;
        var_tmf2_db0 = assign34240_e51533_d_b0;
        var_tmf2_db1 = assign34240_e51533_d_b1;
        var_tmf2_db2 = assign34240_e51533_d_b2;
        var_tmf2_db3 = assign34240_e51533_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34250_e51545, assign34250_e51545_d_n0, assign34250_e51545_d_n1, assign34250_e51545_d_n2, assign34250_e51545_d_n3, assign34250_e51545_d_n4, assign34250_e51545_d_n5, assign34250_e51545_d_b0, assign34250_e51545_d_b1, assign34250_e51545_d_b2, assign34250_e51545_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34250_e51541: f64 = (var_tmf1 + var_tmf2);
        let assign34250_e51542: f64 = (0.5 * assign34250_e51541);
        let assign34250_e51543: f64 = (p.p85 - assign34250_e51542);
        (assign34250_e51543, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj_k1, var_nj_k1_dn0, var_nj_k1_dn1, var_nj_k1_dn2, var_nj_k1_dn3, var_nj_k1_dn4, var_nj_k1_dn5, var_nj_k1_db0, var_nj_k1_db1, var_nj_k1_db2, var_nj_k1_db3,)
    }
};
        var_nj_k1 = assign34250_e51545;
        var_nj_k1_dn0 = assign34250_e51545_d_n0;
        var_nj_k1_dn1 = assign34250_e51545_d_n1;
        var_nj_k1_dn2 = assign34250_e51545_d_n2;
        var_nj_k1_dn3 = assign34250_e51545_d_n3;
        var_nj_k1_dn4 = assign34250_e51545_d_n4;
        var_nj_k1_dn5 = assign34250_e51545_d_n5;
        var_nj_k1_db0 = assign34250_e51545_d_b0;
        var_nj_k1_db1 = assign34250_e51545_d_b1;
        var_nj_k1_db2 = assign34250_e51545_d_b2;
        var_nj_k1_db3 = assign34250_e51545_d_b3;
        var_nj_k1_rv = 0.0;
        var_nj_k1_rdn0 = 0.0;
        var_nj_k1_rdn1 = 0.0;
        var_nj_k1_rdn2 = 0.0;
        var_nj_k1_rdn3 = 0.0;
        var_nj_k1_rdn4 = 0.0;
        var_nj_k1_rdn5 = 0.0;
        var_nj_k1_rdb0 = 0.0;
        var_nj_k1_rdb1 = 0.0;
        var_nj_k1_rdb2 = 0.0;
        var_nj_k1_rdb3 = 0.0;

        let (assign34260_e51555, assign34260_e51555_d_n0, assign34260_e51555_d_n1, assign34260_e51555_d_n2, assign34260_e51555_d_n3, assign34260_e51555_d_n4, assign34260_e51555_d_n5, assign34260_e51555_d_b0, assign34260_e51555_d_b1, assign34260_e51555_d_b2, assign34260_e51555_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34260_e51551: f64 = (var_nj_k1 - var_njl);
        let assign34260_e51553: f64 = (assign34260_e51551 - 0.01);
        (assign34260_e51553, (var_nj_k1_dn0 - var_njl_dn0), (var_nj_k1_dn1 - var_njl_dn1), (var_nj_k1_dn2 - var_njl_dn2), (var_nj_k1_dn3 - var_njl_dn3), (var_nj_k1_dn4 - var_njl_dn4), (var_nj_k1_dn5 - var_njl_dn5), (var_nj_k1_db0 - var_njl_db0), (var_nj_k1_db1 - var_njl_db1), (var_nj_k1_db2 - var_njl_db2), (var_nj_k1_db3 - var_njl_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign34260_e51555;
        var_tmf1_dn0 = assign34260_e51555_d_n0;
        var_tmf1_dn1 = assign34260_e51555_d_n1;
        var_tmf1_dn2 = assign34260_e51555_d_n2;
        var_tmf1_dn3 = assign34260_e51555_d_n3;
        var_tmf1_dn4 = assign34260_e51555_d_n4;
        var_tmf1_dn5 = assign34260_e51555_d_n5;
        var_tmf1_db0 = assign34260_e51555_d_b0;
        var_tmf1_db1 = assign34260_e51555_d_b1;
        var_tmf1_db2 = assign34260_e51555_d_b2;
        var_tmf1_db3 = assign34260_e51555_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign34270_e51565, assign34270_e51565_d_n0, assign34270_e51565_d_n1, assign34270_e51565_d_n2, assign34270_e51565_d_n3, assign34270_e51565_d_n4, assign34270_e51565_d_n5, assign34270_e51565_d_b0, assign34270_e51565_d_b1, assign34270_e51565_d_b2, assign34270_e51565_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34270_e51561: f64 = (4.0 * var_njl);
        let assign34270_e51563: f64 = (assign34270_e51561 * 0.01);
        (assign34270_e51563, ((4.0 * var_njl_dn0) * 0.01), ((4.0 * var_njl_dn1) * 0.01), ((4.0 * var_njl_dn2) * 0.01), ((4.0 * var_njl_dn3) * 0.01), ((4.0 * var_njl_dn4) * 0.01), ((4.0 * var_njl_dn5) * 0.01), ((4.0 * var_njl_db0) * 0.01), ((4.0 * var_njl_db1) * 0.01), ((4.0 * var_njl_db2) * 0.01), ((4.0 * var_njl_db3) * 0.01),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34270_e51565;
        var_tmf2_dn0 = assign34270_e51565_d_n0;
        var_tmf2_dn1 = assign34270_e51565_d_n1;
        var_tmf2_dn2 = assign34270_e51565_d_n2;
        var_tmf2_dn3 = assign34270_e51565_d_n3;
        var_tmf2_dn4 = assign34270_e51565_d_n4;
        var_tmf2_dn5 = assign34270_e51565_d_n5;
        var_tmf2_db0 = assign34270_e51565_d_b0;
        var_tmf2_db1 = assign34270_e51565_d_b1;
        var_tmf2_db2 = assign34270_e51565_d_b2;
        var_tmf2_db3 = assign34270_e51565_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34280_e51577, assign34280_e51577_d_n0, assign34280_e51577_d_n1, assign34280_e51577_d_n2, assign34280_e51577_d_n3, assign34280_e51577_d_n4, assign34280_e51577_d_n5, assign34280_e51577_d_b0, assign34280_e51577_d_b1, assign34280_e51577_d_b2, assign34280_e51577_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let (assign34280_e51575, assign34280_e51575_d_n0, assign34280_e51575_d_n1, assign34280_e51575_d_n2, assign34280_e51575_d_n3, assign34280_e51575_d_n4, assign34280_e51575_d_n5, assign34280_e51575_d_b0, assign34280_e51575_d_b1, assign34280_e51575_d_b2, assign34280_e51575_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign34280_e51574: f64 = (-var_tmf2);
                (assign34280_e51574, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign34280_e51575, assign34280_e51575_d_n0, assign34280_e51575_d_n1, assign34280_e51575_d_n2, assign34280_e51575_d_n3, assign34280_e51575_d_n4, assign34280_e51575_d_n5, assign34280_e51575_d_b0, assign34280_e51575_d_b1, assign34280_e51575_d_b2, assign34280_e51575_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34280_e51577;
        var_tmf2_dn0 = assign34280_e51577_d_n0;
        var_tmf2_dn1 = assign34280_e51577_d_n1;
        var_tmf2_dn2 = assign34280_e51577_d_n2;
        var_tmf2_dn3 = assign34280_e51577_d_n3;
        var_tmf2_dn4 = assign34280_e51577_d_n4;
        var_tmf2_dn5 = assign34280_e51577_d_n5;
        var_tmf2_db0 = assign34280_e51577_d_b0;
        var_tmf2_db1 = assign34280_e51577_d_b1;
        var_tmf2_db2 = assign34280_e51577_d_b2;
        var_tmf2_db3 = assign34280_e51577_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34290_e51588, assign34290_e51588_d_n0, assign34290_e51588_d_n1, assign34290_e51588_d_n2, assign34290_e51588_d_n3, assign34290_e51588_d_n4, assign34290_e51588_d_n5, assign34290_e51588_d_b0, assign34290_e51588_d_b1, assign34290_e51588_d_b2, assign34290_e51588_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34290_e51583: f64 = (var_tmf1 * var_tmf1);
        let assign34290_e51585: f64 = (assign34290_e51583 + var_tmf2);
        let assign34290_e51586: f64 = (assign34290_e51585).sqrt();
        (assign34290_e51586, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34290_e51586)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign34290_e51586)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34290_e51586)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign34290_e51586)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign34290_e51586)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign34290_e51586)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign34290_e51586)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign34290_e51586)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign34290_e51586)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign34290_e51586)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34290_e51588;
        var_tmf2_dn0 = assign34290_e51588_d_n0;
        var_tmf2_dn1 = assign34290_e51588_d_n1;
        var_tmf2_dn2 = assign34290_e51588_d_n2;
        var_tmf2_dn3 = assign34290_e51588_d_n3;
        var_tmf2_dn4 = assign34290_e51588_d_n4;
        var_tmf2_dn5 = assign34290_e51588_d_n5;
        var_tmf2_db0 = assign34290_e51588_d_b0;
        var_tmf2_db1 = assign34290_e51588_d_b1;
        var_tmf2_db2 = assign34290_e51588_d_b2;
        var_tmf2_db3 = assign34290_e51588_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34300_e51600, assign34300_e51600_d_n0, assign34300_e51600_d_n1, assign34300_e51600_d_n2, assign34300_e51600_d_n3, assign34300_e51600_d_n4, assign34300_e51600_d_n5, assign34300_e51600_d_b0, assign34300_e51600_d_b1, assign34300_e51600_d_b2, assign34300_e51600_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34300_e51596: f64 = (var_tmf1 + var_tmf2);
        let assign34300_e51597: f64 = (0.5 * assign34300_e51596);
        let assign34300_e51598: f64 = (var_njl + assign34300_e51597);
        (assign34300_e51598, (var_njl_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_njl_dn1 + (0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (var_njl_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_njl_dn3 + (0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (var_njl_dn4 + (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_njl_dn5 + (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_njl_db0 + (0.5 * (var_tmf1_db0 + var_tmf2_db0))), (var_njl_db1 + (0.5 * (var_tmf1_db1 + var_tmf2_db1))), (var_njl_db2 + (0.5 * (var_tmf1_db2 + var_tmf2_db2))), (var_njl_db3 + (0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj_k, var_nj_k_dn0, var_nj_k_dn1, var_nj_k_dn2, var_nj_k_dn3, var_nj_k_dn4, var_nj_k_dn5, var_nj_k_db0, var_nj_k_db1, var_nj_k_db2, var_nj_k_db3,)
    }
};
        var_nj_k = assign34300_e51600;
        var_nj_k_dn0 = assign34300_e51600_d_n0;
        var_nj_k_dn1 = assign34300_e51600_d_n1;
        var_nj_k_dn2 = assign34300_e51600_d_n2;
        var_nj_k_dn3 = assign34300_e51600_d_n3;
        var_nj_k_dn4 = assign34300_e51600_d_n4;
        var_nj_k_dn5 = assign34300_e51600_d_n5;
        var_nj_k_db0 = assign34300_e51600_d_b0;
        var_nj_k_db1 = assign34300_e51600_d_b1;
        var_nj_k_db2 = assign34300_e51600_d_b2;
        var_nj_k_db3 = assign34300_e51600_d_b3;
        var_nj_k_rv = 0.0;
        var_nj_k_rdn0 = 0.0;
        var_nj_k_rdn1 = 0.0;
        var_nj_k_rdn2 = 0.0;
        var_nj_k_rdn3 = 0.0;
        var_nj_k_rdn4 = 0.0;
        var_nj_k_rdn5 = 0.0;
        var_nj_k_rdb0 = 0.0;
        var_nj_k_rdb1 = 0.0;
        var_nj_k_rdb2 = 0.0;
        var_nj_k_rdb3 = 0.0;

        let (assign34310_e51610, assign34310_e51610_d_n0, assign34310_e51610_d_n1, assign34310_e51610_d_n2, assign34310_e51610_d_n3, assign34310_e51610_d_n4, assign34310_e51610_d_n5, assign34310_e51610_d_b0, assign34310_e51610_d_b1, assign34310_e51610_d_b2, assign34310_e51610_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34310_e51606: f64 = (p.p85 - var_nj0);
        let assign34310_e51608: f64 = (assign34310_e51606 - 0.01);
        (assign34310_e51608, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign34310_e51610;
        var_tmf1_dn0 = assign34310_e51610_d_n0;
        var_tmf1_dn1 = assign34310_e51610_d_n1;
        var_tmf1_dn2 = assign34310_e51610_d_n2;
        var_tmf1_dn3 = assign34310_e51610_d_n3;
        var_tmf1_dn4 = assign34310_e51610_d_n4;
        var_tmf1_dn5 = assign34310_e51610_d_n5;
        var_tmf1_db0 = assign34310_e51610_d_b0;
        var_tmf1_db1 = assign34310_e51610_d_b1;
        var_tmf1_db2 = assign34310_e51610_d_b2;
        var_tmf1_db3 = assign34310_e51610_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign34320_e51620, assign34320_e51620_d_n0, assign34320_e51620_d_n1, assign34320_e51620_d_n2, assign34320_e51620_d_n3, assign34320_e51620_d_n4, assign34320_e51620_d_n5, assign34320_e51620_d_b0, assign34320_e51620_d_b1, assign34320_e51620_d_b2, assign34320_e51620_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34320_e51616: f64 = (4.0 * p.p85);
        let assign34320_e51618: f64 = (assign34320_e51616 * 0.01);
        (assign34320_e51618, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34320_e51620;
        var_tmf2_dn0 = assign34320_e51620_d_n0;
        var_tmf2_dn1 = assign34320_e51620_d_n1;
        var_tmf2_dn2 = assign34320_e51620_d_n2;
        var_tmf2_dn3 = assign34320_e51620_d_n3;
        var_tmf2_dn4 = assign34320_e51620_d_n4;
        var_tmf2_dn5 = assign34320_e51620_d_n5;
        var_tmf2_db0 = assign34320_e51620_d_b0;
        var_tmf2_db1 = assign34320_e51620_d_b1;
        var_tmf2_db2 = assign34320_e51620_d_b2;
        var_tmf2_db3 = assign34320_e51620_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34330_e51632, assign34330_e51632_d_n0, assign34330_e51632_d_n1, assign34330_e51632_d_n2, assign34330_e51632_d_n3, assign34330_e51632_d_n4, assign34330_e51632_d_n5, assign34330_e51632_d_b0, assign34330_e51632_d_b1, assign34330_e51632_d_b2, assign34330_e51632_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let (assign34330_e51630, assign34330_e51630_d_n0, assign34330_e51630_d_n1, assign34330_e51630_d_n2, assign34330_e51630_d_n3, assign34330_e51630_d_n4, assign34330_e51630_d_n5, assign34330_e51630_d_b0, assign34330_e51630_d_b1, assign34330_e51630_d_b2, assign34330_e51630_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign34330_e51629: f64 = (-var_tmf2);
                (assign34330_e51629, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign34330_e51630, assign34330_e51630_d_n0, assign34330_e51630_d_n1, assign34330_e51630_d_n2, assign34330_e51630_d_n3, assign34330_e51630_d_n4, assign34330_e51630_d_n5, assign34330_e51630_d_b0, assign34330_e51630_d_b1, assign34330_e51630_d_b2, assign34330_e51630_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34330_e51632;
        var_tmf2_dn0 = assign34330_e51632_d_n0;
        var_tmf2_dn1 = assign34330_e51632_d_n1;
        var_tmf2_dn2 = assign34330_e51632_d_n2;
        var_tmf2_dn3 = assign34330_e51632_d_n3;
        var_tmf2_dn4 = assign34330_e51632_d_n4;
        var_tmf2_dn5 = assign34330_e51632_d_n5;
        var_tmf2_db0 = assign34330_e51632_d_b0;
        var_tmf2_db1 = assign34330_e51632_d_b1;
        var_tmf2_db2 = assign34330_e51632_d_b2;
        var_tmf2_db3 = assign34330_e51632_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34340_e51643, assign34340_e51643_d_n0, assign34340_e51643_d_n1, assign34340_e51643_d_n2, assign34340_e51643_d_n3, assign34340_e51643_d_n4, assign34340_e51643_d_n5, assign34340_e51643_d_b0, assign34340_e51643_d_b1, assign34340_e51643_d_b2, assign34340_e51643_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34340_e51638: f64 = (var_tmf1 * var_tmf1);
        let assign34340_e51640: f64 = (assign34340_e51638 + var_tmf2);
        let assign34340_e51641: f64 = (assign34340_e51640).sqrt();
        (assign34340_e51641, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34340_e51641)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign34340_e51641)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34340_e51641)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign34340_e51641)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign34340_e51641)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign34340_e51641)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign34340_e51641)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign34340_e51641)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign34340_e51641)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign34340_e51641)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34340_e51643;
        var_tmf2_dn0 = assign34340_e51643_d_n0;
        var_tmf2_dn1 = assign34340_e51643_d_n1;
        var_tmf2_dn2 = assign34340_e51643_d_n2;
        var_tmf2_dn3 = assign34340_e51643_d_n3;
        var_tmf2_dn4 = assign34340_e51643_d_n4;
        var_tmf2_dn5 = assign34340_e51643_d_n5;
        var_tmf2_db0 = assign34340_e51643_d_b0;
        var_tmf2_db1 = assign34340_e51643_d_b1;
        var_tmf2_db2 = assign34340_e51643_d_b2;
        var_tmf2_db3 = assign34340_e51643_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34350_e51655, assign34350_e51655_d_n0, assign34350_e51655_d_n1, assign34350_e51655_d_n2, assign34350_e51655_d_n3, assign34350_e51655_d_n4, assign34350_e51655_d_n5, assign34350_e51655_d_b0, assign34350_e51655_d_b1, assign34350_e51655_d_b2, assign34350_e51655_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34350_e51651: f64 = (var_tmf1 + var_tmf2);
        let assign34350_e51652: f64 = (0.5 * assign34350_e51651);
        let assign34350_e51653: f64 = (p.p85 - assign34350_e51652);
        (assign34350_e51653, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign34350_e51655;
        var_nj0_dn0 = assign34350_e51655_d_n0;
        var_nj0_dn1 = assign34350_e51655_d_n1;
        var_nj0_dn2 = assign34350_e51655_d_n2;
        var_nj0_dn3 = assign34350_e51655_d_n3;
        var_nj0_dn4 = assign34350_e51655_d_n4;
        var_nj0_dn5 = assign34350_e51655_d_n5;
        var_nj0_db0 = assign34350_e51655_d_b0;
        var_nj0_db1 = assign34350_e51655_d_b1;
        var_nj0_db2 = assign34350_e51655_d_b2;
        var_nj0_db3 = assign34350_e51655_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj0_rdb0_slot = var_nj0_rdb0;
        *var_nj0_rdb1_slot = var_nj0_rdb1;
        *var_nj0_rdb2_slot = var_nj0_rdb2;
        *var_nj0_rdb3_slot = var_nj0_rdb3;
        *var_nj0_rdn0_slot = var_nj0_rdn0;
        *var_nj0_rdn1_slot = var_nj0_rdn1;
        *var_nj0_rdn2_slot = var_nj0_rdn2;
        *var_nj0_rdn3_slot = var_nj0_rdn3;
        *var_nj0_rdn4_slot = var_nj0_rdn4;
        *var_nj0_rdn5_slot = var_nj0_rdn5;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj_k_slot = var_nj_k;
        *var_nj_k1_slot = var_nj_k1;
        *var_nj_k1_db0_slot = var_nj_k1_db0;
        *var_nj_k1_db1_slot = var_nj_k1_db1;
        *var_nj_k1_db2_slot = var_nj_k1_db2;
        *var_nj_k1_db3_slot = var_nj_k1_db3;
        *var_nj_k1_dn0_slot = var_nj_k1_dn0;
        *var_nj_k1_dn1_slot = var_nj_k1_dn1;
        *var_nj_k1_dn2_slot = var_nj_k1_dn2;
        *var_nj_k1_dn3_slot = var_nj_k1_dn3;
        *var_nj_k1_dn4_slot = var_nj_k1_dn4;
        *var_nj_k1_dn5_slot = var_nj_k1_dn5;
        *var_nj_k1_rdb0_slot = var_nj_k1_rdb0;
        *var_nj_k1_rdb1_slot = var_nj_k1_rdb1;
        *var_nj_k1_rdb2_slot = var_nj_k1_rdb2;
        *var_nj_k1_rdb3_slot = var_nj_k1_rdb3;
        *var_nj_k1_rdn0_slot = var_nj_k1_rdn0;
        *var_nj_k1_rdn1_slot = var_nj_k1_rdn1;
        *var_nj_k1_rdn2_slot = var_nj_k1_rdn2;
        *var_nj_k1_rdn3_slot = var_nj_k1_rdn3;
        *var_nj_k1_rdn4_slot = var_nj_k1_rdn4;
        *var_nj_k1_rdn5_slot = var_nj_k1_rdn5;
        *var_nj_k1_rv_slot = var_nj_k1_rv;
        *var_nj_k_db0_slot = var_nj_k_db0;
        *var_nj_k_db1_slot = var_nj_k_db1;
        *var_nj_k_db2_slot = var_nj_k_db2;
        *var_nj_k_db3_slot = var_nj_k_db3;
        *var_nj_k_dn0_slot = var_nj_k_dn0;
        *var_nj_k_dn1_slot = var_nj_k_dn1;
        *var_nj_k_dn2_slot = var_nj_k_dn2;
        *var_nj_k_dn3_slot = var_nj_k_dn3;
        *var_nj_k_dn4_slot = var_nj_k_dn4;
        *var_nj_k_dn5_slot = var_nj_k_dn5;
        *var_nj_k_rdb0_slot = var_nj_k_rdb0;
        *var_nj_k_rdb1_slot = var_nj_k_rdb1;
        *var_nj_k_rdb2_slot = var_nj_k_rdb2;
        *var_nj_k_rdb3_slot = var_nj_k_rdb3;
        *var_nj_k_rdn0_slot = var_nj_k_rdn0;
        *var_nj_k_rdn1_slot = var_nj_k_rdn1;
        *var_nj_k_rdn2_slot = var_nj_k_rdn2;
        *var_nj_k_rdn3_slot = var_nj_k_rdn3;
        *var_nj_k_rdn4_slot = var_nj_k_rdn4;
        *var_nj_k_rdn5_slot = var_nj_k_rdn5;
        *var_nj_k_rv_slot = var_nj_k_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_rdb0_slot = var_tmf1_rdb0;
        *var_tmf1_rdb1_slot = var_tmf1_rdb1;
        *var_tmf1_rdb2_slot = var_tmf1_rdb2;
        *var_tmf1_rdb3_slot = var_tmf1_rdb3;
        *var_tmf1_rdn0_slot = var_tmf1_rdn0;
        *var_tmf1_rdn1_slot = var_tmf1_rdn1;
        *var_tmf1_rdn2_slot = var_tmf1_rdn2;
        *var_tmf1_rdn3_slot = var_tmf1_rdn3;
        *var_tmf1_rdn4_slot = var_tmf1_rdn4;
        *var_tmf1_rdn5_slot = var_tmf1_rdn5;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_rdb0_slot = var_tmf2_rdb0;
        *var_tmf2_rdb1_slot = var_tmf2_rdb1;
        *var_tmf2_rdb2_slot = var_tmf2_rdb2;
        *var_tmf2_rdb3_slot = var_tmf2_rdb3;
        *var_tmf2_rdn0_slot = var_tmf2_rdn0;
        *var_tmf2_rdn1_slot = var_tmf2_rdn1;
        *var_tmf2_rdn2_slot = var_tmf2_rdn2;
        *var_tmf2_rdn3_slot = var_tmf2_rdn3;
        *var_tmf2_rdn4_slot = var_tmf2_rdn4;
        *var_tmf2_rdn5_slot = var_tmf2_rdn5;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_85(
        p: &Parameters,
        var_guard558: f64,
        var_guard559: f64,
        var_idmultbot: f64,
        var_idmultbot_db0: f64,
        var_idmultbot_db1: f64,
        var_idmultbot_db2: f64,
        var_idmultbot_db3: f64,
        var_idmultbot_dn0: f64,
        var_idmultbot_dn1: f64,
        var_idmultbot_dn2: f64,
        var_idmultbot_dn3: f64,
        var_idmultbot_dn4: f64,
        var_idmultbot_dn5: f64,
        var_njl: f64,
        var_njl_db0: f64,
        var_njl_db1: f64,
        var_njl_db2: f64,
        var_njl_db3: f64,
        var_njl_dn0: f64,
        var_njl_dn1: f64,
        var_njl_dn2: f64,
        var_njl_dn3: f64,
        var_njl_dn4: f64,
        var_njl_dn5: f64,
        var_phitdinv: f64,
        var_phitdinv_db0: f64,
        var_phitdinv_db1: f64,
        var_phitdinv_db2: f64,
        var_phitdinv_db3: f64,
        var_phitdinv_dn0: f64,
        var_phitdinv_dn1: f64,
        var_phitdinv_dn2: f64,
        var_phitdinv_dn3: f64,
        var_phitdinv_dn4: f64,
        var_phitdinv_dn5: f64,
        var_v_ha: f64,
        var_v_ha_db0: f64,
        var_v_ha_db1: f64,
        var_v_ha_db2: f64,
        var_v_ha_db3: f64,
        var_v_ha_dn0: f64,
        var_v_ha_dn1: f64,
        var_v_ha_dn2: f64,
        var_v_ha_dn3: f64,
        var_v_ha_dn4: f64,
        var_v_ha_dn5: f64,
        var_v_hk: f64,
        var_v_hk_db0: f64,
        var_v_hk_db1: f64,
        var_v_hk_db2: f64,
        var_v_hk_db3: f64,
        var_v_hk_dn0: f64,
        var_v_hk_dn1: f64,
        var_v_hk_dn2: f64,
        var_v_hk_dn3: f64,
        var_v_hk_dn4: f64,
        var_v_hk_dn5: f64,
        var_vak: f64,
        var_vak_db0: f64,
        var_vak_db1: f64,
        var_vak_db2: f64,
        var_vak_db3: f64,
        var_vak_dn0: f64,
        var_vak_dn1: f64,
        var_vak_dn2: f64,
        var_vak_dn3: f64,
        var_vak_dn4: f64,
        var_vak_dn5: f64,
        var_exp_a_slot: &mut f64,
        var_exp_a_db0_slot: &mut f64,
        var_exp_a_db1_slot: &mut f64,
        var_exp_a_db2_slot: &mut f64,
        var_exp_a_db3_slot: &mut f64,
        var_exp_a_dn0_slot: &mut f64,
        var_exp_a_dn1_slot: &mut f64,
        var_exp_a_dn2_slot: &mut f64,
        var_exp_a_dn3_slot: &mut f64,
        var_exp_a_dn4_slot: &mut f64,
        var_exp_a_dn5_slot: &mut f64,
        var_exp_a_rdb0_slot: &mut f64,
        var_exp_a_rdb1_slot: &mut f64,
        var_exp_a_rdb2_slot: &mut f64,
        var_exp_a_rdb3_slot: &mut f64,
        var_exp_a_rdn0_slot: &mut f64,
        var_exp_a_rdn1_slot: &mut f64,
        var_exp_a_rdn2_slot: &mut f64,
        var_exp_a_rdn3_slot: &mut f64,
        var_exp_a_rdn4_slot: &mut f64,
        var_exp_a_rdn5_slot: &mut f64,
        var_exp_a_rv_slot: &mut f64,
        var_exp_k_slot: &mut f64,
        var_exp_k_db0_slot: &mut f64,
        var_exp_k_db1_slot: &mut f64,
        var_exp_k_db2_slot: &mut f64,
        var_exp_k_db3_slot: &mut f64,
        var_exp_k_dn0_slot: &mut f64,
        var_exp_k_dn1_slot: &mut f64,
        var_exp_k_dn2_slot: &mut f64,
        var_exp_k_dn3_slot: &mut f64,
        var_exp_k_dn4_slot: &mut f64,
        var_exp_k_dn5_slot: &mut f64,
        var_exp_k_rdb0_slot: &mut f64,
        var_exp_k_rdb1_slot: &mut f64,
        var_exp_k_rdb2_slot: &mut f64,
        var_exp_k_rdb3_slot: &mut f64,
        var_exp_k_rdn0_slot: &mut f64,
        var_exp_k_rdn1_slot: &mut f64,
        var_exp_k_rdn2_slot: &mut f64,
        var_exp_k_rdn3_slot: &mut f64,
        var_exp_k_rdn4_slot: &mut f64,
        var_exp_k_rdn5_slot: &mut f64,
        var_exp_k_rv_slot: &mut f64,
        var_guard560_slot: &mut f64,
        var_guard560_db0_slot: &mut f64,
        var_guard560_db1_slot: &mut f64,
        var_guard560_db2_slot: &mut f64,
        var_guard560_db3_slot: &mut f64,
        var_guard560_dn0_slot: &mut f64,
        var_guard560_dn1_slot: &mut f64,
        var_guard560_dn2_slot: &mut f64,
        var_guard560_dn3_slot: &mut f64,
        var_guard560_dn4_slot: &mut f64,
        var_guard560_dn5_slot: &mut f64,
        var_guard560_rdb0_slot: &mut f64,
        var_guard560_rdb1_slot: &mut f64,
        var_guard560_rdb2_slot: &mut f64,
        var_guard560_rdb3_slot: &mut f64,
        var_guard560_rdn0_slot: &mut f64,
        var_guard560_rdn1_slot: &mut f64,
        var_guard560_rdn2_slot: &mut f64,
        var_guard560_rdn3_slot: &mut f64,
        var_guard560_rdn4_slot: &mut f64,
        var_guard560_rdn5_slot: &mut f64,
        var_guard560_rv_slot: &mut f64,
        var_guard561_slot: &mut f64,
        var_guard561_db0_slot: &mut f64,
        var_guard561_db1_slot: &mut f64,
        var_guard561_db2_slot: &mut f64,
        var_guard561_db3_slot: &mut f64,
        var_guard561_dn0_slot: &mut f64,
        var_guard561_dn1_slot: &mut f64,
        var_guard561_dn2_slot: &mut f64,
        var_guard561_dn3_slot: &mut f64,
        var_guard561_dn4_slot: &mut f64,
        var_guard561_dn5_slot: &mut f64,
        var_guard561_rdb0_slot: &mut f64,
        var_guard561_rdb1_slot: &mut f64,
        var_guard561_rdb2_slot: &mut f64,
        var_guard561_rdb3_slot: &mut f64,
        var_guard561_rdn0_slot: &mut f64,
        var_guard561_rdn1_slot: &mut f64,
        var_guard561_rdn2_slot: &mut f64,
        var_guard561_rdn3_slot: &mut f64,
        var_guard561_rdn4_slot: &mut f64,
        var_guard561_rdn5_slot: &mut f64,
        var_guard561_rv_slot: &mut f64,
        var_guard562_slot: &mut f64,
        var_guard562_db0_slot: &mut f64,
        var_guard562_db1_slot: &mut f64,
        var_guard562_db2_slot: &mut f64,
        var_guard562_db3_slot: &mut f64,
        var_guard562_dn0_slot: &mut f64,
        var_guard562_dn1_slot: &mut f64,
        var_guard562_dn2_slot: &mut f64,
        var_guard562_dn3_slot: &mut f64,
        var_guard562_dn4_slot: &mut f64,
        var_guard562_dn5_slot: &mut f64,
        var_guard562_rdb0_slot: &mut f64,
        var_guard562_rdb1_slot: &mut f64,
        var_guard562_rdb2_slot: &mut f64,
        var_guard562_rdb3_slot: &mut f64,
        var_guard562_rdn0_slot: &mut f64,
        var_guard562_rdn1_slot: &mut f64,
        var_guard562_rdn2_slot: &mut f64,
        var_guard562_rdn3_slot: &mut f64,
        var_guard562_rdn4_slot: &mut f64,
        var_guard562_rdn5_slot: &mut f64,
        var_guard562_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj0_rdb0_slot: &mut f64,
        var_nj0_rdb1_slot: &mut f64,
        var_nj0_rdb2_slot: &mut f64,
        var_nj0_rdb3_slot: &mut f64,
        var_nj0_rdn0_slot: &mut f64,
        var_nj0_rdn1_slot: &mut f64,
        var_nj0_rdn2_slot: &mut f64,
        var_nj0_rdn3_slot: &mut f64,
        var_nj0_rdn4_slot: &mut f64,
        var_nj0_rdn5_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj_k_slot: &mut f64,
        var_nj_k_db0_slot: &mut f64,
        var_nj_k_db1_slot: &mut f64,
        var_nj_k_db2_slot: &mut f64,
        var_nj_k_db3_slot: &mut f64,
        var_nj_k_dn0_slot: &mut f64,
        var_nj_k_dn1_slot: &mut f64,
        var_nj_k_dn2_slot: &mut f64,
        var_nj_k_dn3_slot: &mut f64,
        var_nj_k_dn4_slot: &mut f64,
        var_nj_k_dn5_slot: &mut f64,
        var_nj_k_rdb0_slot: &mut f64,
        var_nj_k_rdb1_slot: &mut f64,
        var_nj_k_rdb2_slot: &mut f64,
        var_nj_k_rdb3_slot: &mut f64,
        var_nj_k_rdn0_slot: &mut f64,
        var_nj_k_rdn1_slot: &mut f64,
        var_nj_k_rdn2_slot: &mut f64,
        var_nj_k_rdn3_slot: &mut f64,
        var_nj_k_rdn4_slot: &mut f64,
        var_nj_k_rdn5_slot: &mut f64,
        var_nj_k_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_rdb0_slot: &mut f64,
        var_tmf1_rdb1_slot: &mut f64,
        var_tmf1_rdb2_slot: &mut f64,
        var_tmf1_rdb3_slot: &mut f64,
        var_tmf1_rdn0_slot: &mut f64,
        var_tmf1_rdn1_slot: &mut f64,
        var_tmf1_rdn2_slot: &mut f64,
        var_tmf1_rdn3_slot: &mut f64,
        var_tmf1_rdn4_slot: &mut f64,
        var_tmf1_rdn5_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_rdb0_slot: &mut f64,
        var_tmf2_rdb1_slot: &mut f64,
        var_tmf2_rdb2_slot: &mut f64,
        var_tmf2_rdb3_slot: &mut f64,
        var_tmf2_rdn0_slot: &mut f64,
        var_tmf2_rdn1_slot: &mut f64,
        var_tmf2_rdn2_slot: &mut f64,
        var_tmf2_rdn3_slot: &mut f64,
        var_tmf2_rdn4_slot: &mut f64,
        var_tmf2_rdn5_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_exp_a: f64 = *var_exp_a_slot;
        let mut var_exp_a_db0: f64 = *var_exp_a_db0_slot;
        let mut var_exp_a_db1: f64 = *var_exp_a_db1_slot;
        let mut var_exp_a_db2: f64 = *var_exp_a_db2_slot;
        let mut var_exp_a_db3: f64 = *var_exp_a_db3_slot;
        let mut var_exp_a_dn0: f64 = *var_exp_a_dn0_slot;
        let mut var_exp_a_dn1: f64 = *var_exp_a_dn1_slot;
        let mut var_exp_a_dn2: f64 = *var_exp_a_dn2_slot;
        let mut var_exp_a_dn3: f64 = *var_exp_a_dn3_slot;
        let mut var_exp_a_dn4: f64 = *var_exp_a_dn4_slot;
        let mut var_exp_a_dn5: f64 = *var_exp_a_dn5_slot;
        let mut var_exp_a_rdb0: f64 = *var_exp_a_rdb0_slot;
        let mut var_exp_a_rdb1: f64 = *var_exp_a_rdb1_slot;
        let mut var_exp_a_rdb2: f64 = *var_exp_a_rdb2_slot;
        let mut var_exp_a_rdb3: f64 = *var_exp_a_rdb3_slot;
        let mut var_exp_a_rdn0: f64 = *var_exp_a_rdn0_slot;
        let mut var_exp_a_rdn1: f64 = *var_exp_a_rdn1_slot;
        let mut var_exp_a_rdn2: f64 = *var_exp_a_rdn2_slot;
        let mut var_exp_a_rdn3: f64 = *var_exp_a_rdn3_slot;
        let mut var_exp_a_rdn4: f64 = *var_exp_a_rdn4_slot;
        let mut var_exp_a_rdn5: f64 = *var_exp_a_rdn5_slot;
        let mut var_exp_a_rv: f64 = *var_exp_a_rv_slot;
        let mut var_exp_k: f64 = *var_exp_k_slot;
        let mut var_exp_k_db0: f64 = *var_exp_k_db0_slot;
        let mut var_exp_k_db1: f64 = *var_exp_k_db1_slot;
        let mut var_exp_k_db2: f64 = *var_exp_k_db2_slot;
        let mut var_exp_k_db3: f64 = *var_exp_k_db3_slot;
        let mut var_exp_k_dn0: f64 = *var_exp_k_dn0_slot;
        let mut var_exp_k_dn1: f64 = *var_exp_k_dn1_slot;
        let mut var_exp_k_dn2: f64 = *var_exp_k_dn2_slot;
        let mut var_exp_k_dn3: f64 = *var_exp_k_dn3_slot;
        let mut var_exp_k_dn4: f64 = *var_exp_k_dn4_slot;
        let mut var_exp_k_dn5: f64 = *var_exp_k_dn5_slot;
        let mut var_exp_k_rdb0: f64 = *var_exp_k_rdb0_slot;
        let mut var_exp_k_rdb1: f64 = *var_exp_k_rdb1_slot;
        let mut var_exp_k_rdb2: f64 = *var_exp_k_rdb2_slot;
        let mut var_exp_k_rdb3: f64 = *var_exp_k_rdb3_slot;
        let mut var_exp_k_rdn0: f64 = *var_exp_k_rdn0_slot;
        let mut var_exp_k_rdn1: f64 = *var_exp_k_rdn1_slot;
        let mut var_exp_k_rdn2: f64 = *var_exp_k_rdn2_slot;
        let mut var_exp_k_rdn3: f64 = *var_exp_k_rdn3_slot;
        let mut var_exp_k_rdn4: f64 = *var_exp_k_rdn4_slot;
        let mut var_exp_k_rdn5: f64 = *var_exp_k_rdn5_slot;
        let mut var_exp_k_rv: f64 = *var_exp_k_rv_slot;
        let mut var_guard560: f64 = *var_guard560_slot;
        let mut var_guard560_db0: f64 = *var_guard560_db0_slot;
        let mut var_guard560_db1: f64 = *var_guard560_db1_slot;
        let mut var_guard560_db2: f64 = *var_guard560_db2_slot;
        let mut var_guard560_db3: f64 = *var_guard560_db3_slot;
        let mut var_guard560_dn0: f64 = *var_guard560_dn0_slot;
        let mut var_guard560_dn1: f64 = *var_guard560_dn1_slot;
        let mut var_guard560_dn2: f64 = *var_guard560_dn2_slot;
        let mut var_guard560_dn3: f64 = *var_guard560_dn3_slot;
        let mut var_guard560_dn4: f64 = *var_guard560_dn4_slot;
        let mut var_guard560_dn5: f64 = *var_guard560_dn5_slot;
        let mut var_guard560_rdb0: f64 = *var_guard560_rdb0_slot;
        let mut var_guard560_rdb1: f64 = *var_guard560_rdb1_slot;
        let mut var_guard560_rdb2: f64 = *var_guard560_rdb2_slot;
        let mut var_guard560_rdb3: f64 = *var_guard560_rdb3_slot;
        let mut var_guard560_rdn0: f64 = *var_guard560_rdn0_slot;
        let mut var_guard560_rdn1: f64 = *var_guard560_rdn1_slot;
        let mut var_guard560_rdn2: f64 = *var_guard560_rdn2_slot;
        let mut var_guard560_rdn3: f64 = *var_guard560_rdn3_slot;
        let mut var_guard560_rdn4: f64 = *var_guard560_rdn4_slot;
        let mut var_guard560_rdn5: f64 = *var_guard560_rdn5_slot;
        let mut var_guard560_rv: f64 = *var_guard560_rv_slot;
        let mut var_guard561: f64 = *var_guard561_slot;
        let mut var_guard561_db0: f64 = *var_guard561_db0_slot;
        let mut var_guard561_db1: f64 = *var_guard561_db1_slot;
        let mut var_guard561_db2: f64 = *var_guard561_db2_slot;
        let mut var_guard561_db3: f64 = *var_guard561_db3_slot;
        let mut var_guard561_dn0: f64 = *var_guard561_dn0_slot;
        let mut var_guard561_dn1: f64 = *var_guard561_dn1_slot;
        let mut var_guard561_dn2: f64 = *var_guard561_dn2_slot;
        let mut var_guard561_dn3: f64 = *var_guard561_dn3_slot;
        let mut var_guard561_dn4: f64 = *var_guard561_dn4_slot;
        let mut var_guard561_dn5: f64 = *var_guard561_dn5_slot;
        let mut var_guard561_rdb0: f64 = *var_guard561_rdb0_slot;
        let mut var_guard561_rdb1: f64 = *var_guard561_rdb1_slot;
        let mut var_guard561_rdb2: f64 = *var_guard561_rdb2_slot;
        let mut var_guard561_rdb3: f64 = *var_guard561_rdb3_slot;
        let mut var_guard561_rdn0: f64 = *var_guard561_rdn0_slot;
        let mut var_guard561_rdn1: f64 = *var_guard561_rdn1_slot;
        let mut var_guard561_rdn2: f64 = *var_guard561_rdn2_slot;
        let mut var_guard561_rdn3: f64 = *var_guard561_rdn3_slot;
        let mut var_guard561_rdn4: f64 = *var_guard561_rdn4_slot;
        let mut var_guard561_rdn5: f64 = *var_guard561_rdn5_slot;
        let mut var_guard561_rv: f64 = *var_guard561_rv_slot;
        let mut var_guard562: f64 = *var_guard562_slot;
        let mut var_guard562_db0: f64 = *var_guard562_db0_slot;
        let mut var_guard562_db1: f64 = *var_guard562_db1_slot;
        let mut var_guard562_db2: f64 = *var_guard562_db2_slot;
        let mut var_guard562_db3: f64 = *var_guard562_db3_slot;
        let mut var_guard562_dn0: f64 = *var_guard562_dn0_slot;
        let mut var_guard562_dn1: f64 = *var_guard562_dn1_slot;
        let mut var_guard562_dn2: f64 = *var_guard562_dn2_slot;
        let mut var_guard562_dn3: f64 = *var_guard562_dn3_slot;
        let mut var_guard562_dn4: f64 = *var_guard562_dn4_slot;
        let mut var_guard562_dn5: f64 = *var_guard562_dn5_slot;
        let mut var_guard562_rdb0: f64 = *var_guard562_rdb0_slot;
        let mut var_guard562_rdb1: f64 = *var_guard562_rdb1_slot;
        let mut var_guard562_rdb2: f64 = *var_guard562_rdb2_slot;
        let mut var_guard562_rdb3: f64 = *var_guard562_rdb3_slot;
        let mut var_guard562_rdn0: f64 = *var_guard562_rdn0_slot;
        let mut var_guard562_rdn1: f64 = *var_guard562_rdn1_slot;
        let mut var_guard562_rdn2: f64 = *var_guard562_rdn2_slot;
        let mut var_guard562_rdn3: f64 = *var_guard562_rdn3_slot;
        let mut var_guard562_rdn4: f64 = *var_guard562_rdn4_slot;
        let mut var_guard562_rdn5: f64 = *var_guard562_rdn5_slot;
        let mut var_guard562_rv: f64 = *var_guard562_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj0_rdb0: f64 = *var_nj0_rdb0_slot;
        let mut var_nj0_rdb1: f64 = *var_nj0_rdb1_slot;
        let mut var_nj0_rdb2: f64 = *var_nj0_rdb2_slot;
        let mut var_nj0_rdb3: f64 = *var_nj0_rdb3_slot;
        let mut var_nj0_rdn0: f64 = *var_nj0_rdn0_slot;
        let mut var_nj0_rdn1: f64 = *var_nj0_rdn1_slot;
        let mut var_nj0_rdn2: f64 = *var_nj0_rdn2_slot;
        let mut var_nj0_rdn3: f64 = *var_nj0_rdn3_slot;
        let mut var_nj0_rdn4: f64 = *var_nj0_rdn4_slot;
        let mut var_nj0_rdn5: f64 = *var_nj0_rdn5_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj_k: f64 = *var_nj_k_slot;
        let mut var_nj_k_db0: f64 = *var_nj_k_db0_slot;
        let mut var_nj_k_db1: f64 = *var_nj_k_db1_slot;
        let mut var_nj_k_db2: f64 = *var_nj_k_db2_slot;
        let mut var_nj_k_db3: f64 = *var_nj_k_db3_slot;
        let mut var_nj_k_dn0: f64 = *var_nj_k_dn0_slot;
        let mut var_nj_k_dn1: f64 = *var_nj_k_dn1_slot;
        let mut var_nj_k_dn2: f64 = *var_nj_k_dn2_slot;
        let mut var_nj_k_dn3: f64 = *var_nj_k_dn3_slot;
        let mut var_nj_k_dn4: f64 = *var_nj_k_dn4_slot;
        let mut var_nj_k_dn5: f64 = *var_nj_k_dn5_slot;
        let mut var_nj_k_rdb0: f64 = *var_nj_k_rdb0_slot;
        let mut var_nj_k_rdb1: f64 = *var_nj_k_rdb1_slot;
        let mut var_nj_k_rdb2: f64 = *var_nj_k_rdb2_slot;
        let mut var_nj_k_rdb3: f64 = *var_nj_k_rdb3_slot;
        let mut var_nj_k_rdn0: f64 = *var_nj_k_rdn0_slot;
        let mut var_nj_k_rdn1: f64 = *var_nj_k_rdn1_slot;
        let mut var_nj_k_rdn2: f64 = *var_nj_k_rdn2_slot;
        let mut var_nj_k_rdn3: f64 = *var_nj_k_rdn3_slot;
        let mut var_nj_k_rdn4: f64 = *var_nj_k_rdn4_slot;
        let mut var_nj_k_rdn5: f64 = *var_nj_k_rdn5_slot;
        let mut var_nj_k_rv: f64 = *var_nj_k_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_rdb0: f64 = *var_tmf1_rdb0_slot;
        let mut var_tmf1_rdb1: f64 = *var_tmf1_rdb1_slot;
        let mut var_tmf1_rdb2: f64 = *var_tmf1_rdb2_slot;
        let mut var_tmf1_rdb3: f64 = *var_tmf1_rdb3_slot;
        let mut var_tmf1_rdn0: f64 = *var_tmf1_rdn0_slot;
        let mut var_tmf1_rdn1: f64 = *var_tmf1_rdn1_slot;
        let mut var_tmf1_rdn2: f64 = *var_tmf1_rdn2_slot;
        let mut var_tmf1_rdn3: f64 = *var_tmf1_rdn3_slot;
        let mut var_tmf1_rdn4: f64 = *var_tmf1_rdn4_slot;
        let mut var_tmf1_rdn5: f64 = *var_tmf1_rdn5_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_rdb0: f64 = *var_tmf2_rdb0_slot;
        let mut var_tmf2_rdb1: f64 = *var_tmf2_rdb1_slot;
        let mut var_tmf2_rdb2: f64 = *var_tmf2_rdb2_slot;
        let mut var_tmf2_rdb3: f64 = *var_tmf2_rdb3_slot;
        let mut var_tmf2_rdn0: f64 = *var_tmf2_rdn0_slot;
        let mut var_tmf2_rdn1: f64 = *var_tmf2_rdn1_slot;
        let mut var_tmf2_rdn2: f64 = *var_tmf2_rdn2_slot;
        let mut var_tmf2_rdn3: f64 = *var_tmf2_rdn3_slot;
        let mut var_tmf2_rdn4: f64 = *var_tmf2_rdn4_slot;
        let mut var_tmf2_rdn5: f64 = *var_tmf2_rdn5_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign34360_e51665, assign34360_e51665_d_n0, assign34360_e51665_d_n1, assign34360_e51665_d_n2, assign34360_e51665_d_n3, assign34360_e51665_d_n4, assign34360_e51665_d_n5, assign34360_e51665_d_b0, assign34360_e51665_d_b1, assign34360_e51665_d_b2, assign34360_e51665_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34360_e51661: f64 = (var_nj0 - var_njl);
        let assign34360_e51663: f64 = (assign34360_e51661 - 0.01);
        (assign34360_e51663, (var_nj0_dn0 - var_njl_dn0), (var_nj0_dn1 - var_njl_dn1), (var_nj0_dn2 - var_njl_dn2), (var_nj0_dn3 - var_njl_dn3), (var_nj0_dn4 - var_njl_dn4), (var_nj0_dn5 - var_njl_dn5), (var_nj0_db0 - var_njl_db0), (var_nj0_db1 - var_njl_db1), (var_nj0_db2 - var_njl_db2), (var_nj0_db3 - var_njl_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign34360_e51665;
        var_tmf1_dn0 = assign34360_e51665_d_n0;
        var_tmf1_dn1 = assign34360_e51665_d_n1;
        var_tmf1_dn2 = assign34360_e51665_d_n2;
        var_tmf1_dn3 = assign34360_e51665_d_n3;
        var_tmf1_dn4 = assign34360_e51665_d_n4;
        var_tmf1_dn5 = assign34360_e51665_d_n5;
        var_tmf1_db0 = assign34360_e51665_d_b0;
        var_tmf1_db1 = assign34360_e51665_d_b1;
        var_tmf1_db2 = assign34360_e51665_d_b2;
        var_tmf1_db3 = assign34360_e51665_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        let (assign34370_e51675, assign34370_e51675_d_n0, assign34370_e51675_d_n1, assign34370_e51675_d_n2, assign34370_e51675_d_n3, assign34370_e51675_d_n4, assign34370_e51675_d_n5, assign34370_e51675_d_b0, assign34370_e51675_d_b1, assign34370_e51675_d_b2, assign34370_e51675_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34370_e51671: f64 = (4.0 * var_njl);
        let assign34370_e51673: f64 = (assign34370_e51671 * 0.01);
        (assign34370_e51673, ((4.0 * var_njl_dn0) * 0.01), ((4.0 * var_njl_dn1) * 0.01), ((4.0 * var_njl_dn2) * 0.01), ((4.0 * var_njl_dn3) * 0.01), ((4.0 * var_njl_dn4) * 0.01), ((4.0 * var_njl_dn5) * 0.01), ((4.0 * var_njl_db0) * 0.01), ((4.0 * var_njl_db1) * 0.01), ((4.0 * var_njl_db2) * 0.01), ((4.0 * var_njl_db3) * 0.01),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34370_e51675;
        var_tmf2_dn0 = assign34370_e51675_d_n0;
        var_tmf2_dn1 = assign34370_e51675_d_n1;
        var_tmf2_dn2 = assign34370_e51675_d_n2;
        var_tmf2_dn3 = assign34370_e51675_d_n3;
        var_tmf2_dn4 = assign34370_e51675_d_n4;
        var_tmf2_dn5 = assign34370_e51675_d_n5;
        var_tmf2_db0 = assign34370_e51675_d_b0;
        var_tmf2_db1 = assign34370_e51675_d_b1;
        var_tmf2_db2 = assign34370_e51675_d_b2;
        var_tmf2_db3 = assign34370_e51675_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34380_e51687, assign34380_e51687_d_n0, assign34380_e51687_d_n1, assign34380_e51687_d_n2, assign34380_e51687_d_n3, assign34380_e51687_d_n4, assign34380_e51687_d_n5, assign34380_e51687_d_b0, assign34380_e51687_d_b1, assign34380_e51687_d_b2, assign34380_e51687_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let (assign34380_e51685, assign34380_e51685_d_n0, assign34380_e51685_d_n1, assign34380_e51685_d_n2, assign34380_e51685_d_n3, assign34380_e51685_d_n4, assign34380_e51685_d_n5, assign34380_e51685_d_b0, assign34380_e51685_d_b1, assign34380_e51685_d_b2, assign34380_e51685_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign34380_e51684: f64 = (-var_tmf2);
                (assign34380_e51684, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign34380_e51685, assign34380_e51685_d_n0, assign34380_e51685_d_n1, assign34380_e51685_d_n2, assign34380_e51685_d_n3, assign34380_e51685_d_n4, assign34380_e51685_d_n5, assign34380_e51685_d_b0, assign34380_e51685_d_b1, assign34380_e51685_d_b2, assign34380_e51685_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34380_e51687;
        var_tmf2_dn0 = assign34380_e51687_d_n0;
        var_tmf2_dn1 = assign34380_e51687_d_n1;
        var_tmf2_dn2 = assign34380_e51687_d_n2;
        var_tmf2_dn3 = assign34380_e51687_d_n3;
        var_tmf2_dn4 = assign34380_e51687_d_n4;
        var_tmf2_dn5 = assign34380_e51687_d_n5;
        var_tmf2_db0 = assign34380_e51687_d_b0;
        var_tmf2_db1 = assign34380_e51687_d_b1;
        var_tmf2_db2 = assign34380_e51687_d_b2;
        var_tmf2_db3 = assign34380_e51687_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34390_e51698, assign34390_e51698_d_n0, assign34390_e51698_d_n1, assign34390_e51698_d_n2, assign34390_e51698_d_n3, assign34390_e51698_d_n4, assign34390_e51698_d_n5, assign34390_e51698_d_b0, assign34390_e51698_d_b1, assign34390_e51698_d_b2, assign34390_e51698_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34390_e51693: f64 = (var_tmf1 * var_tmf1);
        let assign34390_e51695: f64 = (assign34390_e51693 + var_tmf2);
        let assign34390_e51696: f64 = (assign34390_e51695).sqrt();
        (assign34390_e51696, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34390_e51696)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign34390_e51696)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34390_e51696)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign34390_e51696)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign34390_e51696)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign34390_e51696)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign34390_e51696)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign34390_e51696)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign34390_e51696)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign34390_e51696)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34390_e51698;
        var_tmf2_dn0 = assign34390_e51698_d_n0;
        var_tmf2_dn1 = assign34390_e51698_d_n1;
        var_tmf2_dn2 = assign34390_e51698_d_n2;
        var_tmf2_dn3 = assign34390_e51698_d_n3;
        var_tmf2_dn4 = assign34390_e51698_d_n4;
        var_tmf2_dn5 = assign34390_e51698_d_n5;
        var_tmf2_db0 = assign34390_e51698_d_b0;
        var_tmf2_db1 = assign34390_e51698_d_b1;
        var_tmf2_db2 = assign34390_e51698_d_b2;
        var_tmf2_db3 = assign34390_e51698_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34400_e51710, assign34400_e51710_d_n0, assign34400_e51710_d_n1, assign34400_e51710_d_n2, assign34400_e51710_d_n3, assign34400_e51710_d_n4, assign34400_e51710_d_n5, assign34400_e51710_d_b0, assign34400_e51710_d_b1, assign34400_e51710_d_b2, assign34400_e51710_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34400_e51706: f64 = (var_tmf1 + var_tmf2);
        let assign34400_e51707: f64 = (0.5 * assign34400_e51706);
        let assign34400_e51708: f64 = (var_njl + assign34400_e51707);
        (assign34400_e51708, (var_njl_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_njl_dn1 + (0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (var_njl_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_njl_dn3 + (0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (var_njl_dn4 + (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_njl_dn5 + (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_njl_db0 + (0.5 * (var_tmf1_db0 + var_tmf2_db0))), (var_njl_db1 + (0.5 * (var_tmf1_db1 + var_tmf2_db1))), (var_njl_db2 + (0.5 * (var_tmf1_db2 + var_tmf2_db2))), (var_njl_db3 + (0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign34400_e51710;
        var_nj0_dn0 = assign34400_e51710_d_n0;
        var_nj0_dn1 = assign34400_e51710_d_n1;
        var_nj0_dn2 = assign34400_e51710_d_n2;
        var_nj0_dn3 = assign34400_e51710_d_n3;
        var_nj0_dn4 = assign34400_e51710_d_n4;
        var_nj0_dn5 = assign34400_e51710_d_n5;
        var_nj0_db0 = assign34400_e51710_d_b0;
        var_nj0_db1 = assign34400_e51710_d_b1;
        var_nj0_db2 = assign34400_e51710_d_b2;
        var_nj0_db3 = assign34400_e51710_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign34410_e51717, assign34410_e51717_d_n0, assign34410_e51717_d_n1, assign34410_e51717_d_n2, assign34410_e51717_d_n3, assign34410_e51717_d_n4, assign34410_e51717_d_n5, assign34410_e51717_d_b0, assign34410_e51717_d_b1, assign34410_e51717_d_b2, assign34410_e51717_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 == 0.0)) {
        (var_njl, var_njl_dn0, var_njl_dn1, var_njl_dn2, var_njl_dn3, var_njl_dn4, var_njl_dn5, var_njl_db0, var_njl_db1, var_njl_db2, var_njl_db3,)
    } else {
        (var_nj_k, var_nj_k_dn0, var_nj_k_dn1, var_nj_k_dn2, var_nj_k_dn3, var_nj_k_dn4, var_nj_k_dn5, var_nj_k_db0, var_nj_k_db1, var_nj_k_db2, var_nj_k_db3,)
    }
};
        var_nj_k = assign34410_e51717;
        var_nj_k_dn0 = assign34410_e51717_d_n0;
        var_nj_k_dn1 = assign34410_e51717_d_n1;
        var_nj_k_dn2 = assign34410_e51717_d_n2;
        var_nj_k_dn3 = assign34410_e51717_d_n3;
        var_nj_k_dn4 = assign34410_e51717_d_n4;
        var_nj_k_dn5 = assign34410_e51717_d_n5;
        var_nj_k_db0 = assign34410_e51717_d_b0;
        var_nj_k_db1 = assign34410_e51717_d_b1;
        var_nj_k_db2 = assign34410_e51717_d_b2;
        var_nj_k_db3 = assign34410_e51717_d_b3;
        var_nj_k_rv = 0.0;
        var_nj_k_rdn0 = 0.0;
        var_nj_k_rdn1 = 0.0;
        var_nj_k_rdn2 = 0.0;
        var_nj_k_rdn3 = 0.0;
        var_nj_k_rdn4 = 0.0;
        var_nj_k_rdn5 = 0.0;
        var_nj_k_rdb0 = 0.0;
        var_nj_k_rdb1 = 0.0;
        var_nj_k_rdb2 = 0.0;
        var_nj_k_rdb3 = 0.0;

        let (assign34420_e51724, assign34420_e51724_d_n0, assign34420_e51724_d_n1, assign34420_e51724_d_n2, assign34420_e51724_d_n3, assign34420_e51724_d_n4, assign34420_e51724_d_n5, assign34420_e51724_d_b0, assign34420_e51724_d_b1, assign34420_e51724_d_b2, assign34420_e51724_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 == 0.0)) {
        (var_njl, var_njl_dn0, var_njl_dn1, var_njl_dn2, var_njl_dn3, var_njl_dn4, var_njl_dn5, var_njl_db0, var_njl_db1, var_njl_db2, var_njl_db3,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign34420_e51724;
        var_nj0_dn0 = assign34420_e51724_d_n0;
        var_nj0_dn1 = assign34420_e51724_d_n1;
        var_nj0_dn2 = assign34420_e51724_d_n2;
        var_nj0_dn3 = assign34420_e51724_d_n3;
        var_nj0_dn4 = assign34420_e51724_d_n4;
        var_nj0_dn5 = assign34420_e51724_d_n5;
        var_nj0_db0 = assign34420_e51724_d_b0;
        var_nj0_db1 = assign34420_e51724_d_b1;
        var_nj0_db2 = assign34420_e51724_d_b2;
        var_nj0_db3 = assign34420_e51724_d_b3;
        var_nj0_rv = 0.0;
        var_nj0_rdn0 = 0.0;
        var_nj0_rdn1 = 0.0;
        var_nj0_rdn2 = 0.0;
        var_nj0_rdn3 = 0.0;
        var_nj0_rdn4 = 0.0;
        var_nj0_rdn5 = 0.0;
        var_nj0_rdb0 = 0.0;
        var_nj0_rdb1 = 0.0;
        var_nj0_rdb2 = 0.0;
        var_nj0_rdb3 = 0.0;

        let (assign34430_e51728, assign34430_e51728_d_n0, assign34430_e51728_d_n1, assign34430_e51728_d_n2, assign34430_e51728_d_n3, assign34430_e51728_d_n4, assign34430_e51728_d_n5, assign34430_e51728_d_b0, assign34430_e51728_d_b1, assign34430_e51728_d_b2, assign34430_e51728_d_b3,) = {
    if (var_guard558 != 0.0) {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    } else {
        (var_exp_a, var_exp_a_dn0, var_exp_a_dn1, var_exp_a_dn2, var_exp_a_dn3, var_exp_a_dn4, var_exp_a_dn5, var_exp_a_db0, var_exp_a_db1, var_exp_a_db2, var_exp_a_db3,)
    }
};
        var_exp_a = assign34430_e51728;
        var_exp_a_dn0 = assign34430_e51728_d_n0;
        var_exp_a_dn1 = assign34430_e51728_d_n1;
        var_exp_a_dn2 = assign34430_e51728_d_n2;
        var_exp_a_dn3 = assign34430_e51728_d_n3;
        var_exp_a_dn4 = assign34430_e51728_d_n4;
        var_exp_a_dn5 = assign34430_e51728_d_n5;
        var_exp_a_db0 = assign34430_e51728_d_b0;
        var_exp_a_db1 = assign34430_e51728_d_b1;
        var_exp_a_db2 = assign34430_e51728_d_b2;
        var_exp_a_db3 = assign34430_e51728_d_b3;
        var_exp_a_rv = 0.0;
        var_exp_a_rdn0 = 0.0;
        var_exp_a_rdn1 = 0.0;
        var_exp_a_rdn2 = 0.0;
        var_exp_a_rdn3 = 0.0;
        var_exp_a_rdn4 = 0.0;
        var_exp_a_rdn5 = 0.0;
        var_exp_a_rdb0 = 0.0;
        var_exp_a_rdb1 = 0.0;
        var_exp_a_rdb2 = 0.0;
        var_exp_a_rdb3 = 0.0;

        let assign34440_e51732: f64 = (var_v_hk - var_v_ha);
        let assign34440_e51733: f64 = (var_vak - assign34440_e51732);
        let assign34440_e51735: f64 = if assign34440_e51733 > 0.0 { 1.0 } else { 0.0 };
        var_guard560 = assign34440_e51735;
        var_guard560_dn0 = 0.0;
        var_guard560_dn1 = 0.0;
        var_guard560_dn2 = 0.0;
        var_guard560_dn3 = 0.0;
        var_guard560_dn4 = 0.0;
        var_guard560_dn5 = 0.0;
        var_guard560_db0 = 0.0;
        var_guard560_db1 = 0.0;
        var_guard560_db2 = 0.0;
        var_guard560_db3 = 0.0;
        var_guard560_rv = 0.0;
        var_guard560_rdn0 = 0.0;
        var_guard560_rdn1 = 0.0;
        var_guard560_rdn2 = 0.0;
        var_guard560_rdn3 = 0.0;
        var_guard560_rdn4 = 0.0;
        var_guard560_rdn5 = 0.0;
        var_guard560_rdb0 = 0.0;
        var_guard560_rdb1 = 0.0;
        var_guard560_rdb2 = 0.0;
        var_guard560_rdb3 = 0.0;

        let assign34450_e51739: f64 = (var_vak / var_nj_k);
        let assign34450_e51742: f64 = (var_v_hk - var_v_ha);
        let assign34450_e51744: f64 = (assign34450_e51742 / var_nj_k);
        let assign34450_e51745: f64 = (assign34450_e51739 - assign34450_e51744);
        let assign34450_e51749: f64 = (var_nj_k - var_nj0);
        let assign34450_e51750: f64 = (var_v_hk * assign34450_e51749);
        let assign34450_e51753: f64 = (var_nj0 * p.p85);
        let assign34450_e51754: f64 = (assign34450_e51750 / assign34450_e51753);
        let assign34450_e51755: f64 = (assign34450_e51745 + assign34450_e51754);
        let assign34450_e51756: f64 = (var_phitdinv * assign34450_e51755);
        let assign34450_e51757: f64 = (assign34450_e51756).abs();
        let assign34450_e51759: f64 = if assign34450_e51757 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard561 = assign34450_e51759;
        var_guard561_dn0 = 0.0;
        var_guard561_dn1 = 0.0;
        var_guard561_dn2 = 0.0;
        var_guard561_dn3 = 0.0;
        var_guard561_dn4 = 0.0;
        var_guard561_dn5 = 0.0;
        var_guard561_db0 = 0.0;
        var_guard561_db1 = 0.0;
        var_guard561_db2 = 0.0;
        var_guard561_db3 = 0.0;
        var_guard561_rv = 0.0;
        var_guard561_rdn0 = 0.0;
        var_guard561_rdn1 = 0.0;
        var_guard561_rdn2 = 0.0;
        var_guard561_rdn3 = 0.0;
        var_guard561_rdn4 = 0.0;
        var_guard561_rdn5 = 0.0;
        var_guard561_rdb0 = 0.0;
        var_guard561_rdb1 = 0.0;
        var_guard561_rdb2 = 0.0;
        var_guard561_rdb3 = 0.0;

        let (assign34460_e51788, assign34460_e51788_d_n0, assign34460_e51788_d_n1, assign34460_e51788_d_n2, assign34460_e51788_d_n3, assign34460_e51788_d_n4, assign34460_e51788_d_n5, assign34460_e51788_d_b0, assign34460_e51788_d_b1, assign34460_e51788_d_b2, assign34460_e51788_d_b3,) = {
    if (((var_guard558 != 0.0) && (var_guard560 != 0.0)) && (var_guard561 != 0.0)) {
        let assign34460_e51768: f64 = (var_vak / var_nj_k);
        let assign34460_e51771: f64 = (var_v_hk - var_v_ha);
        let assign34460_e51773: f64 = (assign34460_e51771 / var_nj_k);
        let assign34460_e51774: f64 = (assign34460_e51768 - assign34460_e51773);
        let assign34460_e51778: f64 = (var_nj_k - var_nj0);
        let assign34460_e51779: f64 = (var_v_hk * assign34460_e51778);
        let assign34460_e51782: f64 = (var_nj0 * p.p85);
        let assign34460_e51783: f64 = (assign34460_e51779 / assign34460_e51782);
        let assign34460_e51784: f64 = (assign34460_e51774 + assign34460_e51783);
        let assign34460_e51785: f64 = (var_phitdinv * assign34460_e51784);
        let assign34460_e51786: f64 = (assign34460_e51785).exp();
        (assign34460_e51786, (assign34460_e51786 * ((var_phitdinv_dn0 * assign34460_e51784) + (var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn0 - var_v_ha_dn0) * var_nj_k) - (assign34460_e51771 * var_nj_k_dn0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn0 * assign34460_e51778) + (var_v_hk * (var_nj_k_dn0 - var_nj0_dn0))) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn0 * p.p85))) / (assign34460_e51782 * assign34460_e51782)))))), (assign34460_e51786 * ((var_phitdinv_dn1 * assign34460_e51784) + (var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn1 - var_v_ha_dn1) * var_nj_k) - (assign34460_e51771 * var_nj_k_dn1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn1 * assign34460_e51778) + (var_v_hk * (var_nj_k_dn1 - var_nj0_dn1))) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn1 * p.p85))) / (assign34460_e51782 * assign34460_e51782)))))), (assign34460_e51786 * ((var_phitdinv_dn2 * assign34460_e51784) + (var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn2 - var_v_ha_dn2) * var_nj_k) - (assign34460_e51771 * var_nj_k_dn2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn2 * assign34460_e51778) + (var_v_hk * (var_nj_k_dn2 - var_nj0_dn2))) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn2 * p.p85))) / (assign34460_e51782 * assign34460_e51782)))))), (assign34460_e51786 * ((var_phitdinv_dn3 * assign34460_e51784) + (var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn3 - var_v_ha_dn3) * var_nj_k) - (assign34460_e51771 * var_nj_k_dn3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn3 * assign34460_e51778) + (var_v_hk * (var_nj_k_dn3 - var_nj0_dn3))) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn3 * p.p85))) / (assign34460_e51782 * assign34460_e51782)))))), (assign34460_e51786 * ((var_phitdinv_dn4 * assign34460_e51784) + (var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn4 - var_v_ha_dn4) * var_nj_k) - (assign34460_e51771 * var_nj_k_dn4)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn4 * assign34460_e51778) + (var_v_hk * (var_nj_k_dn4 - var_nj0_dn4))) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn4 * p.p85))) / (assign34460_e51782 * assign34460_e51782)))))), (assign34460_e51786 * ((var_phitdinv_dn5 * assign34460_e51784) + (var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn5 - var_v_ha_dn5) * var_nj_k) - (assign34460_e51771 * var_nj_k_dn5)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn5 * assign34460_e51778) + (var_v_hk * (var_nj_k_dn5 - var_nj0_dn5))) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn5 * p.p85))) / (assign34460_e51782 * assign34460_e51782)))))), (assign34460_e51786 * ((var_phitdinv_db0 * assign34460_e51784) + (var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db0 - var_v_ha_db0) * var_nj_k) - (assign34460_e51771 * var_nj_k_db0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db0 * assign34460_e51778) + (var_v_hk * (var_nj_k_db0 - var_nj0_db0))) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_db0 * p.p85))) / (assign34460_e51782 * assign34460_e51782)))))), (assign34460_e51786 * ((var_phitdinv_db1 * assign34460_e51784) + (var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db1 - var_v_ha_db1) * var_nj_k) - (assign34460_e51771 * var_nj_k_db1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db1 * assign34460_e51778) + (var_v_hk * (var_nj_k_db1 - var_nj0_db1))) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_db1 * p.p85))) / (assign34460_e51782 * assign34460_e51782)))))), (assign34460_e51786 * ((var_phitdinv_db2 * assign34460_e51784) + (var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db2 - var_v_ha_db2) * var_nj_k) - (assign34460_e51771 * var_nj_k_db2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db2 * assign34460_e51778) + (var_v_hk * (var_nj_k_db2 - var_nj0_db2))) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_db2 * p.p85))) / (assign34460_e51782 * assign34460_e51782)))))), (assign34460_e51786 * ((var_phitdinv_db3 * assign34460_e51784) + (var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db3 - var_v_ha_db3) * var_nj_k) - (assign34460_e51771 * var_nj_k_db3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db3 * assign34460_e51778) + (var_v_hk * (var_nj_k_db3 - var_nj0_db3))) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_db3 * p.p85))) / (assign34460_e51782 * assign34460_e51782)))))),)
    } else {
        (var_exp_k, var_exp_k_dn0, var_exp_k_dn1, var_exp_k_dn2, var_exp_k_dn3, var_exp_k_dn4, var_exp_k_dn5, var_exp_k_db0, var_exp_k_db1, var_exp_k_db2, var_exp_k_db3,)
    }
};
        var_exp_k = assign34460_e51788;
        var_exp_k_dn0 = assign34460_e51788_d_n0;
        var_exp_k_dn1 = assign34460_e51788_d_n1;
        var_exp_k_dn2 = assign34460_e51788_d_n2;
        var_exp_k_dn3 = assign34460_e51788_d_n3;
        var_exp_k_dn4 = assign34460_e51788_d_n4;
        var_exp_k_dn5 = assign34460_e51788_d_n5;
        var_exp_k_db0 = assign34460_e51788_d_b0;
        var_exp_k_db1 = assign34460_e51788_d_b1;
        var_exp_k_db2 = assign34460_e51788_d_b2;
        var_exp_k_db3 = assign34460_e51788_d_b3;
        var_exp_k_rv = 0.0;
        var_exp_k_rdn0 = 0.0;
        var_exp_k_rdn1 = 0.0;
        var_exp_k_rdn2 = 0.0;
        var_exp_k_rdn3 = 0.0;
        var_exp_k_rdn4 = 0.0;
        var_exp_k_rdn5 = 0.0;
        var_exp_k_rdb0 = 0.0;
        var_exp_k_rdb1 = 0.0;
        var_exp_k_rdb2 = 0.0;
        var_exp_k_rdb3 = 0.0;

        let assign34470_e51792: f64 = (var_vak / var_nj_k);
        let assign34470_e51795: f64 = (var_v_hk - var_v_ha);
        let assign34470_e51797: f64 = (assign34470_e51795 / var_nj_k);
        let assign34470_e51798: f64 = (assign34470_e51792 - assign34470_e51797);
        let assign34470_e51802: f64 = (var_nj_k - var_nj0);
        let assign34470_e51803: f64 = (var_v_hk * assign34470_e51802);
        let assign34470_e51806: f64 = (var_nj0 * p.p85);
        let assign34470_e51807: f64 = (assign34470_e51803 / assign34470_e51806);
        let assign34470_e51808: f64 = (assign34470_e51798 + assign34470_e51807);
        let assign34470_e51809: f64 = (var_phitdinv * assign34470_e51808);
        let assign34470_e51811: f64 = (-230.25850929940458);
        let assign34470_e51812: f64 = if assign34470_e51809 < assign34470_e51811 { 1.0 } else { 0.0 };
        var_guard562 = assign34470_e51812;
        var_guard562_dn0 = 0.0;
        var_guard562_dn1 = 0.0;
        var_guard562_dn2 = 0.0;
        var_guard562_dn3 = 0.0;
        var_guard562_dn4 = 0.0;
        var_guard562_dn5 = 0.0;
        var_guard562_db0 = 0.0;
        var_guard562_db1 = 0.0;
        var_guard562_db2 = 0.0;
        var_guard562_db3 = 0.0;
        var_guard562_rv = 0.0;
        var_guard562_rdn0 = 0.0;
        var_guard562_rdn1 = 0.0;
        var_guard562_rdn2 = 0.0;
        var_guard562_rdn3 = 0.0;
        var_guard562_rdn4 = 0.0;
        var_guard562_rdn5 = 0.0;
        var_guard562_rdb0 = 0.0;
        var_guard562_rdb1 = 0.0;
        var_guard562_rdb2 = 0.0;
        var_guard562_rdb3 = 0.0;

        let (assign34480_e51908, assign34480_e51908_d_n0, assign34480_e51908_d_n1, assign34480_e51908_d_n2, assign34480_e51908_d_n3, assign34480_e51908_d_n4, assign34480_e51908_d_n5, assign34480_e51908_d_b0, assign34480_e51908_d_b1, assign34480_e51908_d_b2, assign34480_e51908_d_b3,) = {
    if ((((var_guard558 != 0.0) && (var_guard560 != 0.0)) && (var_guard561 == 0.0)) && (var_guard562 != 0.0)) {
        let assign34480_e51824: f64 = (-230.25850929940458);
        let assign34480_e51828: f64 = (var_vak / var_nj_k);
        let assign34480_e51831: f64 = (var_v_hk - var_v_ha);
        let assign34480_e51833: f64 = (assign34480_e51831 / var_nj_k);
        let assign34480_e51834: f64 = (assign34480_e51828 - assign34480_e51833);
        let assign34480_e51838: f64 = (var_nj_k - var_nj0);
        let assign34480_e51839: f64 = (var_v_hk * assign34480_e51838);
        let assign34480_e51842: f64 = (var_nj0 * p.p85);
        let assign34480_e51843: f64 = (assign34480_e51839 / assign34480_e51842);
        let assign34480_e51844: f64 = (assign34480_e51834 + assign34480_e51843);
        let assign34480_e51845: f64 = (var_phitdinv * assign34480_e51844);
        let assign34480_e51846: f64 = (assign34480_e51824 - assign34480_e51845);
        let assign34480_e51850: f64 = (-230.25850929940458);
        let assign34480_e51854: f64 = (var_vak / var_nj_k);
        let assign34480_e51857: f64 = (var_v_hk - var_v_ha);
        let assign34480_e51859: f64 = (assign34480_e51857 / var_nj_k);
        let assign34480_e51860: f64 = (assign34480_e51854 - assign34480_e51859);
        let assign34480_e51864: f64 = (var_nj_k - var_nj0);
        let assign34480_e51865: f64 = (var_v_hk * assign34480_e51864);
        let assign34480_e51868: f64 = (var_nj0 * p.p85);
        let assign34480_e51869: f64 = (assign34480_e51865 / assign34480_e51868);
        let assign34480_e51870: f64 = (assign34480_e51860 + assign34480_e51869);
        let assign34480_e51871: f64 = (var_phitdinv * assign34480_e51870);
        let assign34480_e51872: f64 = (assign34480_e51850 - assign34480_e51871);
        let assign34480_e51875: f64 = (-230.25850929940458);
        let assign34480_e51879: f64 = (var_vak / var_nj_k);
        let assign34480_e51882: f64 = (var_v_hk - var_v_ha);
        let assign34480_e51884: f64 = (assign34480_e51882 / var_nj_k);
        let assign34480_e51885: f64 = (assign34480_e51879 - assign34480_e51884);
        let assign34480_e51889: f64 = (var_nj_k - var_nj0);
        let assign34480_e51890: f64 = (var_v_hk * assign34480_e51889);
        let assign34480_e51893: f64 = (var_nj0 * p.p85);
        let assign34480_e51894: f64 = (assign34480_e51890 / assign34480_e51893);
        let assign34480_e51895: f64 = (assign34480_e51885 + assign34480_e51894);
        let assign34480_e51896: f64 = (var_phitdinv * assign34480_e51895);
        let assign34480_e51897: f64 = (assign34480_e51875 - assign34480_e51896);
        let assign34480_e51899: f64 = (assign34480_e51897 * 0.3333333333333333);
        let assign34480_e51900: f64 = (1.0 + assign34480_e51899);
        let assign34480_e51901: f64 = (assign34480_e51872 * assign34480_e51900);
        let assign34480_e51902: f64 = (0.5 * assign34480_e51901);
        let assign34480_e51903: f64 = (1.0 + assign34480_e51902);
        let assign34480_e51904: f64 = (assign34480_e51846 * assign34480_e51903);
        let assign34480_e51905: f64 = (1.0 + assign34480_e51904);
        let assign34480_e51906: f64 = (1e-100 / assign34480_e51905);
        (assign34480_e51906, (-((1e-100 * (((-((var_phitdinv_dn0 * assign34480_e51844) + (var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn0 - var_v_ha_dn0) * var_nj_k) - (assign34480_e51831 * var_nj_k_dn0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn0 * assign34480_e51838) + (var_v_hk * (var_nj_k_dn0 - var_nj0_dn0))) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn0 * p.p85))) / (assign34480_e51842 * assign34480_e51842)))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-((var_phitdinv_dn0 * assign34480_e51870) + (var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn0 - var_v_ha_dn0) * var_nj_k) - (assign34480_e51857 * var_nj_k_dn0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn0 * assign34480_e51864) + (var_v_hk * (var_nj_k_dn0 - var_nj0_dn0))) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn0 * p.p85))) / (assign34480_e51868 * assign34480_e51868)))))) * assign34480_e51900) + (assign34480_e51872 * ((-((var_phitdinv_dn0 * assign34480_e51895) + (var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn0 - var_v_ha_dn0) * var_nj_k) - (assign34480_e51882 * var_nj_k_dn0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn0 * assign34480_e51889) + (var_v_hk * (var_nj_k_dn0 - var_nj0_dn0))) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn0 * p.p85))) / (assign34480_e51893 * assign34480_e51893)))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-((var_phitdinv_dn1 * assign34480_e51844) + (var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn1 - var_v_ha_dn1) * var_nj_k) - (assign34480_e51831 * var_nj_k_dn1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn1 * assign34480_e51838) + (var_v_hk * (var_nj_k_dn1 - var_nj0_dn1))) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn1 * p.p85))) / (assign34480_e51842 * assign34480_e51842)))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-((var_phitdinv_dn1 * assign34480_e51870) + (var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn1 - var_v_ha_dn1) * var_nj_k) - (assign34480_e51857 * var_nj_k_dn1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn1 * assign34480_e51864) + (var_v_hk * (var_nj_k_dn1 - var_nj0_dn1))) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn1 * p.p85))) / (assign34480_e51868 * assign34480_e51868)))))) * assign34480_e51900) + (assign34480_e51872 * ((-((var_phitdinv_dn1 * assign34480_e51895) + (var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn1 - var_v_ha_dn1) * var_nj_k) - (assign34480_e51882 * var_nj_k_dn1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn1 * assign34480_e51889) + (var_v_hk * (var_nj_k_dn1 - var_nj0_dn1))) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn1 * p.p85))) / (assign34480_e51893 * assign34480_e51893)))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-((var_phitdinv_dn2 * assign34480_e51844) + (var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn2 - var_v_ha_dn2) * var_nj_k) - (assign34480_e51831 * var_nj_k_dn2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn2 * assign34480_e51838) + (var_v_hk * (var_nj_k_dn2 - var_nj0_dn2))) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn2 * p.p85))) / (assign34480_e51842 * assign34480_e51842)))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-((var_phitdinv_dn2 * assign34480_e51870) + (var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn2 - var_v_ha_dn2) * var_nj_k) - (assign34480_e51857 * var_nj_k_dn2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn2 * assign34480_e51864) + (var_v_hk * (var_nj_k_dn2 - var_nj0_dn2))) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn2 * p.p85))) / (assign34480_e51868 * assign34480_e51868)))))) * assign34480_e51900) + (assign34480_e51872 * ((-((var_phitdinv_dn2 * assign34480_e51895) + (var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn2 - var_v_ha_dn2) * var_nj_k) - (assign34480_e51882 * var_nj_k_dn2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn2 * assign34480_e51889) + (var_v_hk * (var_nj_k_dn2 - var_nj0_dn2))) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn2 * p.p85))) / (assign34480_e51893 * assign34480_e51893)))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-((var_phitdinv_dn3 * assign34480_e51844) + (var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn3 - var_v_ha_dn3) * var_nj_k) - (assign34480_e51831 * var_nj_k_dn3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn3 * assign34480_e51838) + (var_v_hk * (var_nj_k_dn3 - var_nj0_dn3))) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn3 * p.p85))) / (assign34480_e51842 * assign34480_e51842)))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-((var_phitdinv_dn3 * assign34480_e51870) + (var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn3 - var_v_ha_dn3) * var_nj_k) - (assign34480_e51857 * var_nj_k_dn3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn3 * assign34480_e51864) + (var_v_hk * (var_nj_k_dn3 - var_nj0_dn3))) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn3 * p.p85))) / (assign34480_e51868 * assign34480_e51868)))))) * assign34480_e51900) + (assign34480_e51872 * ((-((var_phitdinv_dn3 * assign34480_e51895) + (var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn3 - var_v_ha_dn3) * var_nj_k) - (assign34480_e51882 * var_nj_k_dn3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn3 * assign34480_e51889) + (var_v_hk * (var_nj_k_dn3 - var_nj0_dn3))) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn3 * p.p85))) / (assign34480_e51893 * assign34480_e51893)))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-((var_phitdinv_dn4 * assign34480_e51844) + (var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn4 - var_v_ha_dn4) * var_nj_k) - (assign34480_e51831 * var_nj_k_dn4)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn4 * assign34480_e51838) + (var_v_hk * (var_nj_k_dn4 - var_nj0_dn4))) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn4 * p.p85))) / (assign34480_e51842 * assign34480_e51842)))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-((var_phitdinv_dn4 * assign34480_e51870) + (var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn4 - var_v_ha_dn4) * var_nj_k) - (assign34480_e51857 * var_nj_k_dn4)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn4 * assign34480_e51864) + (var_v_hk * (var_nj_k_dn4 - var_nj0_dn4))) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn4 * p.p85))) / (assign34480_e51868 * assign34480_e51868)))))) * assign34480_e51900) + (assign34480_e51872 * ((-((var_phitdinv_dn4 * assign34480_e51895) + (var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn4 - var_v_ha_dn4) * var_nj_k) - (assign34480_e51882 * var_nj_k_dn4)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn4 * assign34480_e51889) + (var_v_hk * (var_nj_k_dn4 - var_nj0_dn4))) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn4 * p.p85))) / (assign34480_e51893 * assign34480_e51893)))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-((var_phitdinv_dn5 * assign34480_e51844) + (var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn5 - var_v_ha_dn5) * var_nj_k) - (assign34480_e51831 * var_nj_k_dn5)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn5 * assign34480_e51838) + (var_v_hk * (var_nj_k_dn5 - var_nj0_dn5))) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn5 * p.p85))) / (assign34480_e51842 * assign34480_e51842)))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-((var_phitdinv_dn5 * assign34480_e51870) + (var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn5 - var_v_ha_dn5) * var_nj_k) - (assign34480_e51857 * var_nj_k_dn5)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn5 * assign34480_e51864) + (var_v_hk * (var_nj_k_dn5 - var_nj0_dn5))) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn5 * p.p85))) / (assign34480_e51868 * assign34480_e51868)))))) * assign34480_e51900) + (assign34480_e51872 * ((-((var_phitdinv_dn5 * assign34480_e51895) + (var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn5 - var_v_ha_dn5) * var_nj_k) - (assign34480_e51882 * var_nj_k_dn5)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn5 * assign34480_e51889) + (var_v_hk * (var_nj_k_dn5 - var_nj0_dn5))) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn5 * p.p85))) / (assign34480_e51893 * assign34480_e51893)))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-((var_phitdinv_db0 * assign34480_e51844) + (var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db0 - var_v_ha_db0) * var_nj_k) - (assign34480_e51831 * var_nj_k_db0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db0 * assign34480_e51838) + (var_v_hk * (var_nj_k_db0 - var_nj0_db0))) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_db0 * p.p85))) / (assign34480_e51842 * assign34480_e51842)))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-((var_phitdinv_db0 * assign34480_e51870) + (var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db0 - var_v_ha_db0) * var_nj_k) - (assign34480_e51857 * var_nj_k_db0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db0 * assign34480_e51864) + (var_v_hk * (var_nj_k_db0 - var_nj0_db0))) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_db0 * p.p85))) / (assign34480_e51868 * assign34480_e51868)))))) * assign34480_e51900) + (assign34480_e51872 * ((-((var_phitdinv_db0 * assign34480_e51895) + (var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db0 - var_v_ha_db0) * var_nj_k) - (assign34480_e51882 * var_nj_k_db0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db0 * assign34480_e51889) + (var_v_hk * (var_nj_k_db0 - var_nj0_db0))) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_db0 * p.p85))) / (assign34480_e51893 * assign34480_e51893)))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-((var_phitdinv_db1 * assign34480_e51844) + (var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db1 - var_v_ha_db1) * var_nj_k) - (assign34480_e51831 * var_nj_k_db1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db1 * assign34480_e51838) + (var_v_hk * (var_nj_k_db1 - var_nj0_db1))) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_db1 * p.p85))) / (assign34480_e51842 * assign34480_e51842)))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-((var_phitdinv_db1 * assign34480_e51870) + (var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db1 - var_v_ha_db1) * var_nj_k) - (assign34480_e51857 * var_nj_k_db1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db1 * assign34480_e51864) + (var_v_hk * (var_nj_k_db1 - var_nj0_db1))) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_db1 * p.p85))) / (assign34480_e51868 * assign34480_e51868)))))) * assign34480_e51900) + (assign34480_e51872 * ((-((var_phitdinv_db1 * assign34480_e51895) + (var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db1 - var_v_ha_db1) * var_nj_k) - (assign34480_e51882 * var_nj_k_db1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db1 * assign34480_e51889) + (var_v_hk * (var_nj_k_db1 - var_nj0_db1))) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_db1 * p.p85))) / (assign34480_e51893 * assign34480_e51893)))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-((var_phitdinv_db2 * assign34480_e51844) + (var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db2 - var_v_ha_db2) * var_nj_k) - (assign34480_e51831 * var_nj_k_db2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db2 * assign34480_e51838) + (var_v_hk * (var_nj_k_db2 - var_nj0_db2))) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_db2 * p.p85))) / (assign34480_e51842 * assign34480_e51842)))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-((var_phitdinv_db2 * assign34480_e51870) + (var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db2 - var_v_ha_db2) * var_nj_k) - (assign34480_e51857 * var_nj_k_db2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db2 * assign34480_e51864) + (var_v_hk * (var_nj_k_db2 - var_nj0_db2))) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_db2 * p.p85))) / (assign34480_e51868 * assign34480_e51868)))))) * assign34480_e51900) + (assign34480_e51872 * ((-((var_phitdinv_db2 * assign34480_e51895) + (var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db2 - var_v_ha_db2) * var_nj_k) - (assign34480_e51882 * var_nj_k_db2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db2 * assign34480_e51889) + (var_v_hk * (var_nj_k_db2 - var_nj0_db2))) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_db2 * p.p85))) / (assign34480_e51893 * assign34480_e51893)))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-((var_phitdinv_db3 * assign34480_e51844) + (var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db3 - var_v_ha_db3) * var_nj_k) - (assign34480_e51831 * var_nj_k_db3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db3 * assign34480_e51838) + (var_v_hk * (var_nj_k_db3 - var_nj0_db3))) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_db3 * p.p85))) / (assign34480_e51842 * assign34480_e51842)))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-((var_phitdinv_db3 * assign34480_e51870) + (var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db3 - var_v_ha_db3) * var_nj_k) - (assign34480_e51857 * var_nj_k_db3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db3 * assign34480_e51864) + (var_v_hk * (var_nj_k_db3 - var_nj0_db3))) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_db3 * p.p85))) / (assign34480_e51868 * assign34480_e51868)))))) * assign34480_e51900) + (assign34480_e51872 * ((-((var_phitdinv_db3 * assign34480_e51895) + (var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db3 - var_v_ha_db3) * var_nj_k) - (assign34480_e51882 * var_nj_k_db3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db3 * assign34480_e51889) + (var_v_hk * (var_nj_k_db3 - var_nj0_db3))) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_db3 * p.p85))) / (assign34480_e51893 * assign34480_e51893)))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))),)
    } else {
        (var_exp_k, var_exp_k_dn0, var_exp_k_dn1, var_exp_k_dn2, var_exp_k_dn3, var_exp_k_dn4, var_exp_k_dn5, var_exp_k_db0, var_exp_k_db1, var_exp_k_db2, var_exp_k_db3,)
    }
};
        var_exp_k = assign34480_e51908;
        var_exp_k_dn0 = assign34480_e51908_d_n0;
        var_exp_k_dn1 = assign34480_e51908_d_n1;
        var_exp_k_dn2 = assign34480_e51908_d_n2;
        var_exp_k_dn3 = assign34480_e51908_d_n3;
        var_exp_k_dn4 = assign34480_e51908_d_n4;
        var_exp_k_dn5 = assign34480_e51908_d_n5;
        var_exp_k_db0 = assign34480_e51908_d_b0;
        var_exp_k_db1 = assign34480_e51908_d_b1;
        var_exp_k_db2 = assign34480_e51908_d_b2;
        var_exp_k_db3 = assign34480_e51908_d_b3;
        var_exp_k_rv = 0.0;
        var_exp_k_rdn0 = 0.0;
        var_exp_k_rdn1 = 0.0;
        var_exp_k_rdn2 = 0.0;
        var_exp_k_rdn3 = 0.0;
        var_exp_k_rdn4 = 0.0;
        var_exp_k_rdn5 = 0.0;
        var_exp_k_rdb0 = 0.0;
        var_exp_k_rdb1 = 0.0;
        var_exp_k_rdb2 = 0.0;
        var_exp_k_rdb3 = 0.0;

        *var_exp_a_slot = var_exp_a;
        *var_exp_a_db0_slot = var_exp_a_db0;
        *var_exp_a_db1_slot = var_exp_a_db1;
        *var_exp_a_db2_slot = var_exp_a_db2;
        *var_exp_a_db3_slot = var_exp_a_db3;
        *var_exp_a_dn0_slot = var_exp_a_dn0;
        *var_exp_a_dn1_slot = var_exp_a_dn1;
        *var_exp_a_dn2_slot = var_exp_a_dn2;
        *var_exp_a_dn3_slot = var_exp_a_dn3;
        *var_exp_a_dn4_slot = var_exp_a_dn4;
        *var_exp_a_dn5_slot = var_exp_a_dn5;
        *var_exp_a_rdb0_slot = var_exp_a_rdb0;
        *var_exp_a_rdb1_slot = var_exp_a_rdb1;
        *var_exp_a_rdb2_slot = var_exp_a_rdb2;
        *var_exp_a_rdb3_slot = var_exp_a_rdb3;
        *var_exp_a_rdn0_slot = var_exp_a_rdn0;
        *var_exp_a_rdn1_slot = var_exp_a_rdn1;
        *var_exp_a_rdn2_slot = var_exp_a_rdn2;
        *var_exp_a_rdn3_slot = var_exp_a_rdn3;
        *var_exp_a_rdn4_slot = var_exp_a_rdn4;
        *var_exp_a_rdn5_slot = var_exp_a_rdn5;
        *var_exp_a_rv_slot = var_exp_a_rv;
        *var_exp_k_slot = var_exp_k;
        *var_exp_k_db0_slot = var_exp_k_db0;
        *var_exp_k_db1_slot = var_exp_k_db1;
        *var_exp_k_db2_slot = var_exp_k_db2;
        *var_exp_k_db3_slot = var_exp_k_db3;
        *var_exp_k_dn0_slot = var_exp_k_dn0;
        *var_exp_k_dn1_slot = var_exp_k_dn1;
        *var_exp_k_dn2_slot = var_exp_k_dn2;
        *var_exp_k_dn3_slot = var_exp_k_dn3;
        *var_exp_k_dn4_slot = var_exp_k_dn4;
        *var_exp_k_dn5_slot = var_exp_k_dn5;
        *var_exp_k_rdb0_slot = var_exp_k_rdb0;
        *var_exp_k_rdb1_slot = var_exp_k_rdb1;
        *var_exp_k_rdb2_slot = var_exp_k_rdb2;
        *var_exp_k_rdb3_slot = var_exp_k_rdb3;
        *var_exp_k_rdn0_slot = var_exp_k_rdn0;
        *var_exp_k_rdn1_slot = var_exp_k_rdn1;
        *var_exp_k_rdn2_slot = var_exp_k_rdn2;
        *var_exp_k_rdn3_slot = var_exp_k_rdn3;
        *var_exp_k_rdn4_slot = var_exp_k_rdn4;
        *var_exp_k_rdn5_slot = var_exp_k_rdn5;
        *var_exp_k_rv_slot = var_exp_k_rv;
        *var_guard560_slot = var_guard560;
        *var_guard560_db0_slot = var_guard560_db0;
        *var_guard560_db1_slot = var_guard560_db1;
        *var_guard560_db2_slot = var_guard560_db2;
        *var_guard560_db3_slot = var_guard560_db3;
        *var_guard560_dn0_slot = var_guard560_dn0;
        *var_guard560_dn1_slot = var_guard560_dn1;
        *var_guard560_dn2_slot = var_guard560_dn2;
        *var_guard560_dn3_slot = var_guard560_dn3;
        *var_guard560_dn4_slot = var_guard560_dn4;
        *var_guard560_dn5_slot = var_guard560_dn5;
        *var_guard560_rdb0_slot = var_guard560_rdb0;
        *var_guard560_rdb1_slot = var_guard560_rdb1;
        *var_guard560_rdb2_slot = var_guard560_rdb2;
        *var_guard560_rdb3_slot = var_guard560_rdb3;
        *var_guard560_rdn0_slot = var_guard560_rdn0;
        *var_guard560_rdn1_slot = var_guard560_rdn1;
        *var_guard560_rdn2_slot = var_guard560_rdn2;
        *var_guard560_rdn3_slot = var_guard560_rdn3;
        *var_guard560_rdn4_slot = var_guard560_rdn4;
        *var_guard560_rdn5_slot = var_guard560_rdn5;
        *var_guard560_rv_slot = var_guard560_rv;
        *var_guard561_slot = var_guard561;
        *var_guard561_db0_slot = var_guard561_db0;
        *var_guard561_db1_slot = var_guard561_db1;
        *var_guard561_db2_slot = var_guard561_db2;
        *var_guard561_db3_slot = var_guard561_db3;
        *var_guard561_dn0_slot = var_guard561_dn0;
        *var_guard561_dn1_slot = var_guard561_dn1;
        *var_guard561_dn2_slot = var_guard561_dn2;
        *var_guard561_dn3_slot = var_guard561_dn3;
        *var_guard561_dn4_slot = var_guard561_dn4;
        *var_guard561_dn5_slot = var_guard561_dn5;
        *var_guard561_rdb0_slot = var_guard561_rdb0;
        *var_guard561_rdb1_slot = var_guard561_rdb1;
        *var_guard561_rdb2_slot = var_guard561_rdb2;
        *var_guard561_rdb3_slot = var_guard561_rdb3;
        *var_guard561_rdn0_slot = var_guard561_rdn0;
        *var_guard561_rdn1_slot = var_guard561_rdn1;
        *var_guard561_rdn2_slot = var_guard561_rdn2;
        *var_guard561_rdn3_slot = var_guard561_rdn3;
        *var_guard561_rdn4_slot = var_guard561_rdn4;
        *var_guard561_rdn5_slot = var_guard561_rdn5;
        *var_guard561_rv_slot = var_guard561_rv;
        *var_guard562_slot = var_guard562;
        *var_guard562_db0_slot = var_guard562_db0;
        *var_guard562_db1_slot = var_guard562_db1;
        *var_guard562_db2_slot = var_guard562_db2;
        *var_guard562_db3_slot = var_guard562_db3;
        *var_guard562_dn0_slot = var_guard562_dn0;
        *var_guard562_dn1_slot = var_guard562_dn1;
        *var_guard562_dn2_slot = var_guard562_dn2;
        *var_guard562_dn3_slot = var_guard562_dn3;
        *var_guard562_dn4_slot = var_guard562_dn4;
        *var_guard562_dn5_slot = var_guard562_dn5;
        *var_guard562_rdb0_slot = var_guard562_rdb0;
        *var_guard562_rdb1_slot = var_guard562_rdb1;
        *var_guard562_rdb2_slot = var_guard562_rdb2;
        *var_guard562_rdb3_slot = var_guard562_rdb3;
        *var_guard562_rdn0_slot = var_guard562_rdn0;
        *var_guard562_rdn1_slot = var_guard562_rdn1;
        *var_guard562_rdn2_slot = var_guard562_rdn2;
        *var_guard562_rdn3_slot = var_guard562_rdn3;
        *var_guard562_rdn4_slot = var_guard562_rdn4;
        *var_guard562_rdn5_slot = var_guard562_rdn5;
        *var_guard562_rv_slot = var_guard562_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj0_rdb0_slot = var_nj0_rdb0;
        *var_nj0_rdb1_slot = var_nj0_rdb1;
        *var_nj0_rdb2_slot = var_nj0_rdb2;
        *var_nj0_rdb3_slot = var_nj0_rdb3;
        *var_nj0_rdn0_slot = var_nj0_rdn0;
        *var_nj0_rdn1_slot = var_nj0_rdn1;
        *var_nj0_rdn2_slot = var_nj0_rdn2;
        *var_nj0_rdn3_slot = var_nj0_rdn3;
        *var_nj0_rdn4_slot = var_nj0_rdn4;
        *var_nj0_rdn5_slot = var_nj0_rdn5;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj_k_slot = var_nj_k;
        *var_nj_k_db0_slot = var_nj_k_db0;
        *var_nj_k_db1_slot = var_nj_k_db1;
        *var_nj_k_db2_slot = var_nj_k_db2;
        *var_nj_k_db3_slot = var_nj_k_db3;
        *var_nj_k_dn0_slot = var_nj_k_dn0;
        *var_nj_k_dn1_slot = var_nj_k_dn1;
        *var_nj_k_dn2_slot = var_nj_k_dn2;
        *var_nj_k_dn3_slot = var_nj_k_dn3;
        *var_nj_k_dn4_slot = var_nj_k_dn4;
        *var_nj_k_dn5_slot = var_nj_k_dn5;
        *var_nj_k_rdb0_slot = var_nj_k_rdb0;
        *var_nj_k_rdb1_slot = var_nj_k_rdb1;
        *var_nj_k_rdb2_slot = var_nj_k_rdb2;
        *var_nj_k_rdb3_slot = var_nj_k_rdb3;
        *var_nj_k_rdn0_slot = var_nj_k_rdn0;
        *var_nj_k_rdn1_slot = var_nj_k_rdn1;
        *var_nj_k_rdn2_slot = var_nj_k_rdn2;
        *var_nj_k_rdn3_slot = var_nj_k_rdn3;
        *var_nj_k_rdn4_slot = var_nj_k_rdn4;
        *var_nj_k_rdn5_slot = var_nj_k_rdn5;
        *var_nj_k_rv_slot = var_nj_k_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_rdb0_slot = var_tmf1_rdb0;
        *var_tmf1_rdb1_slot = var_tmf1_rdb1;
        *var_tmf1_rdb2_slot = var_tmf1_rdb2;
        *var_tmf1_rdb3_slot = var_tmf1_rdb3;
        *var_tmf1_rdn0_slot = var_tmf1_rdn0;
        *var_tmf1_rdn1_slot = var_tmf1_rdn1;
        *var_tmf1_rdn2_slot = var_tmf1_rdn2;
        *var_tmf1_rdn3_slot = var_tmf1_rdn3;
        *var_tmf1_rdn4_slot = var_tmf1_rdn4;
        *var_tmf1_rdn5_slot = var_tmf1_rdn5;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_rdb0_slot = var_tmf2_rdb0;
        *var_tmf2_rdb1_slot = var_tmf2_rdb1;
        *var_tmf2_rdb2_slot = var_tmf2_rdb2;
        *var_tmf2_rdb3_slot = var_tmf2_rdb3;
        *var_tmf2_rdn0_slot = var_tmf2_rdn0;
        *var_tmf2_rdn1_slot = var_tmf2_rdn1;
        *var_tmf2_rdn2_slot = var_tmf2_rdn2;
        *var_tmf2_rdn3_slot = var_tmf2_rdn3;
        *var_tmf2_rdn4_slot = var_tmf2_rdn4;
        *var_tmf2_rdn5_slot = var_tmf2_rdn5;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_86(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_ab_i: f64,
        var_ab_i_db0: f64,
        var_ab_i_db1: f64,
        var_ab_i_db2: f64,
        var_ab_i_db3: f64,
        var_ab_i_dn0: f64,
        var_ab_i_dn1: f64,
        var_ab_i_dn2: f64,
        var_ab_i_dn3: f64,
        var_ab_i_dn4: f64,
        var_ab_i_dn5: f64,
        var_exp_a: f64,
        var_exp_a_db0: f64,
        var_exp_a_db1: f64,
        var_exp_a_db2: f64,
        var_exp_a_db3: f64,
        var_exp_a_dn0: f64,
        var_exp_a_dn1: f64,
        var_exp_a_dn2: f64,
        var_exp_a_dn3: f64,
        var_exp_a_dn4: f64,
        var_exp_a_dn5: f64,
        var_guard558: f64,
        var_guard560: f64,
        var_guard561: f64,
        var_guard562: f64,
        var_nj0: f64,
        var_nj0_db0: f64,
        var_nj0_db1: f64,
        var_nj0_db2: f64,
        var_nj0_db3: f64,
        var_nj0_dn0: f64,
        var_nj0_dn1: f64,
        var_nj0_dn2: f64,
        var_nj0_dn3: f64,
        var_nj0_dn4: f64,
        var_nj0_dn5: f64,
        var_nj_k: f64,
        var_nj_k_db0: f64,
        var_nj_k_db1: f64,
        var_nj_k_db2: f64,
        var_nj_k_db3: f64,
        var_nj_k_dn0: f64,
        var_nj_k_dn1: f64,
        var_nj_k_dn2: f64,
        var_nj_k_dn3: f64,
        var_nj_k_dn4: f64,
        var_nj_k_dn5: f64,
        var_phitdinv: f64,
        var_phitdinv_db0: f64,
        var_phitdinv_db1: f64,
        var_phitdinv_db2: f64,
        var_phitdinv_db3: f64,
        var_phitdinv_dn0: f64,
        var_phitdinv_dn1: f64,
        var_phitdinv_dn2: f64,
        var_phitdinv_dn3: f64,
        var_phitdinv_dn4: f64,
        var_phitdinv_dn5: f64,
        var_pn0: f64,
        var_pn0_db0: f64,
        var_pn0_db1: f64,
        var_pn0_db2: f64,
        var_pn0_db3: f64,
        var_pn0_dn0: f64,
        var_pn0_dn1: f64,
        var_pn0_dn2: f64,
        var_pn0_dn3: f64,
        var_pn0_dn4: f64,
        var_pn0_dn5: f64,
        var_q_pex0: f64,
        var_q_pex0_db0: f64,
        var_q_pex0_db1: f64,
        var_q_pex0_db2: f64,
        var_q_pex0_db3: f64,
        var_q_pex0_dn0: f64,
        var_q_pex0_dn1: f64,
        var_q_pex0_dn2: f64,
        var_q_pex0_dn3: f64,
        var_q_pex0_dn4: f64,
        var_q_pex0_dn5: f64,
        var_tkd: f64,
        var_tkd_db0: f64,
        var_tkd_db1: f64,
        var_tkd_db2: f64,
        var_tkd_db3: f64,
        var_tkd_dn0: f64,
        var_tkd_dn1: f64,
        var_tkd_dn2: f64,
        var_tkd_dn3: f64,
        var_tkd_dn4: f64,
        var_tkd_dn5: f64,
        var_tkr: f64,
        var_tkr_db0: f64,
        var_tkr_db1: f64,
        var_tkr_db2: f64,
        var_tkr_db3: f64,
        var_tkr_dn0: f64,
        var_tkr_dn1: f64,
        var_tkr_dn2: f64,
        var_tkr_dn3: f64,
        var_tkr_dn4: f64,
        var_tkr_dn5: f64,
        var_v_ha: f64,
        var_v_ha_db0: f64,
        var_v_ha_db1: f64,
        var_v_ha_db2: f64,
        var_v_ha_db3: f64,
        var_v_ha_dn0: f64,
        var_v_ha_dn1: f64,
        var_v_ha_dn2: f64,
        var_v_ha_dn3: f64,
        var_v_ha_dn4: f64,
        var_v_ha_dn5: f64,
        var_v_hk: f64,
        var_v_hk_db0: f64,
        var_v_hk_db1: f64,
        var_v_hk_db2: f64,
        var_v_hk_db3: f64,
        var_v_hk_dn0: f64,
        var_v_hk_dn1: f64,
        var_v_hk_dn2: f64,
        var_v_hk_dn3: f64,
        var_v_hk_dn4: f64,
        var_v_hk_dn5: f64,
        var_vak: f64,
        var_vak_db0: f64,
        var_vak_db1: f64,
        var_vak_db2: f64,
        var_vak_db3: f64,
        var_vak_dn0: f64,
        var_vak_dn1: f64,
        var_vak_dn2: f64,
        var_vak_dn3: f64,
        var_vak_dn4: f64,
        var_vak_dn5: f64,
        var_exp_a2_slot: &mut f64,
        var_exp_a2_db0_slot: &mut f64,
        var_exp_a2_db1_slot: &mut f64,
        var_exp_a2_db2_slot: &mut f64,
        var_exp_a2_db3_slot: &mut f64,
        var_exp_a2_dn0_slot: &mut f64,
        var_exp_a2_dn1_slot: &mut f64,
        var_exp_a2_dn2_slot: &mut f64,
        var_exp_a2_dn3_slot: &mut f64,
        var_exp_a2_dn4_slot: &mut f64,
        var_exp_a2_dn5_slot: &mut f64,
        var_exp_a2_rdb0_slot: &mut f64,
        var_exp_a2_rdb1_slot: &mut f64,
        var_exp_a2_rdb2_slot: &mut f64,
        var_exp_a2_rdb3_slot: &mut f64,
        var_exp_a2_rdn0_slot: &mut f64,
        var_exp_a2_rdn1_slot: &mut f64,
        var_exp_a2_rdn2_slot: &mut f64,
        var_exp_a2_rdn3_slot: &mut f64,
        var_exp_a2_rdn4_slot: &mut f64,
        var_exp_a2_rdn5_slot: &mut f64,
        var_exp_a2_rv_slot: &mut f64,
        var_exp_k_slot: &mut f64,
        var_exp_k2_slot: &mut f64,
        var_exp_k2_db0_slot: &mut f64,
        var_exp_k2_db1_slot: &mut f64,
        var_exp_k2_db2_slot: &mut f64,
        var_exp_k2_db3_slot: &mut f64,
        var_exp_k2_dn0_slot: &mut f64,
        var_exp_k2_dn1_slot: &mut f64,
        var_exp_k2_dn2_slot: &mut f64,
        var_exp_k2_dn3_slot: &mut f64,
        var_exp_k2_dn4_slot: &mut f64,
        var_exp_k2_dn5_slot: &mut f64,
        var_exp_k2_rdb0_slot: &mut f64,
        var_exp_k2_rdb1_slot: &mut f64,
        var_exp_k2_rdb2_slot: &mut f64,
        var_exp_k2_rdb3_slot: &mut f64,
        var_exp_k2_rdn0_slot: &mut f64,
        var_exp_k2_rdn1_slot: &mut f64,
        var_exp_k2_rdn2_slot: &mut f64,
        var_exp_k2_rdn3_slot: &mut f64,
        var_exp_k2_rdn4_slot: &mut f64,
        var_exp_k2_rdn5_slot: &mut f64,
        var_exp_k2_rv_slot: &mut f64,
        var_exp_k_db0_slot: &mut f64,
        var_exp_k_db1_slot: &mut f64,
        var_exp_k_db2_slot: &mut f64,
        var_exp_k_db3_slot: &mut f64,
        var_exp_k_dn0_slot: &mut f64,
        var_exp_k_dn1_slot: &mut f64,
        var_exp_k_dn2_slot: &mut f64,
        var_exp_k_dn3_slot: &mut f64,
        var_exp_k_dn4_slot: &mut f64,
        var_exp_k_dn5_slot: &mut f64,
        var_exp_k_rdb0_slot: &mut f64,
        var_exp_k_rdb1_slot: &mut f64,
        var_exp_k_rdb2_slot: &mut f64,
        var_exp_k_rdb3_slot: &mut f64,
        var_exp_k_rdn0_slot: &mut f64,
        var_exp_k_rdn1_slot: &mut f64,
        var_exp_k_rdn2_slot: &mut f64,
        var_exp_k_rdn3_slot: &mut f64,
        var_exp_k_rdn4_slot: &mut f64,
        var_exp_k_rdn5_slot: &mut f64,
        var_exp_k_rv_slot: &mut f64,
        var_guard563_slot: &mut f64,
        var_guard563_db0_slot: &mut f64,
        var_guard563_db1_slot: &mut f64,
        var_guard563_db2_slot: &mut f64,
        var_guard563_db3_slot: &mut f64,
        var_guard563_dn0_slot: &mut f64,
        var_guard563_dn1_slot: &mut f64,
        var_guard563_dn2_slot: &mut f64,
        var_guard563_dn3_slot: &mut f64,
        var_guard563_dn4_slot: &mut f64,
        var_guard563_dn5_slot: &mut f64,
        var_guard563_rdb0_slot: &mut f64,
        var_guard563_rdb1_slot: &mut f64,
        var_guard563_rdb2_slot: &mut f64,
        var_guard563_rdb3_slot: &mut f64,
        var_guard563_rdn0_slot: &mut f64,
        var_guard563_rdn1_slot: &mut f64,
        var_guard563_rdn2_slot: &mut f64,
        var_guard563_rdn3_slot: &mut f64,
        var_guard563_rdn4_slot: &mut f64,
        var_guard563_rdn5_slot: &mut f64,
        var_guard563_rv_slot: &mut f64,
        var_guard564_slot: &mut f64,
        var_guard564_db0_slot: &mut f64,
        var_guard564_db1_slot: &mut f64,
        var_guard564_db2_slot: &mut f64,
        var_guard564_db3_slot: &mut f64,
        var_guard564_dn0_slot: &mut f64,
        var_guard564_dn1_slot: &mut f64,
        var_guard564_dn2_slot: &mut f64,
        var_guard564_dn3_slot: &mut f64,
        var_guard564_dn4_slot: &mut f64,
        var_guard564_dn5_slot: &mut f64,
        var_guard564_rdb0_slot: &mut f64,
        var_guard564_rdb1_slot: &mut f64,
        var_guard564_rdb2_slot: &mut f64,
        var_guard564_rdb3_slot: &mut f64,
        var_guard564_rdn0_slot: &mut f64,
        var_guard564_rdn1_slot: &mut f64,
        var_guard564_rdn2_slot: &mut f64,
        var_guard564_rdn3_slot: &mut f64,
        var_guard564_rdn4_slot: &mut f64,
        var_guard564_rdn5_slot: &mut f64,
        var_guard564_rv_slot: &mut f64,
        var_guard565_slot: &mut f64,
        var_guard565_db0_slot: &mut f64,
        var_guard565_db1_slot: &mut f64,
        var_guard565_db2_slot: &mut f64,
        var_guard565_db3_slot: &mut f64,
        var_guard565_dn0_slot: &mut f64,
        var_guard565_dn1_slot: &mut f64,
        var_guard565_dn2_slot: &mut f64,
        var_guard565_dn3_slot: &mut f64,
        var_guard565_dn4_slot: &mut f64,
        var_guard565_dn5_slot: &mut f64,
        var_guard565_rdb0_slot: &mut f64,
        var_guard565_rdb1_slot: &mut f64,
        var_guard565_rdb2_slot: &mut f64,
        var_guard565_rdb3_slot: &mut f64,
        var_guard565_rdn0_slot: &mut f64,
        var_guard565_rdn1_slot: &mut f64,
        var_guard565_rdn2_slot: &mut f64,
        var_guard565_rdn3_slot: &mut f64,
        var_guard565_rdn4_slot: &mut f64,
        var_guard565_rdn5_slot: &mut f64,
        var_guard565_rv_slot: &mut f64,
        var_inqs0_a_slot: &mut f64,
        var_inqs0_a_db0_slot: &mut f64,
        var_inqs0_a_db1_slot: &mut f64,
        var_inqs0_a_db2_slot: &mut f64,
        var_inqs0_a_db3_slot: &mut f64,
        var_inqs0_a_dn0_slot: &mut f64,
        var_inqs0_a_dn1_slot: &mut f64,
        var_inqs0_a_dn2_slot: &mut f64,
        var_inqs0_a_dn3_slot: &mut f64,
        var_inqs0_a_dn4_slot: &mut f64,
        var_inqs0_a_dn5_slot: &mut f64,
        var_inqs0_a_rdb0_slot: &mut f64,
        var_inqs0_a_rdb1_slot: &mut f64,
        var_inqs0_a_rdb2_slot: &mut f64,
        var_inqs0_a_rdb3_slot: &mut f64,
        var_inqs0_a_rdn0_slot: &mut f64,
        var_inqs0_a_rdn1_slot: &mut f64,
        var_inqs0_a_rdn2_slot: &mut f64,
        var_inqs0_a_rdn3_slot: &mut f64,
        var_inqs0_a_rdn4_slot: &mut f64,
        var_inqs0_a_rdn5_slot: &mut f64,
        var_inqs0_a_rv_slot: &mut f64,
        var_p_na_slot: &mut f64,
        var_p_na_db0_slot: &mut f64,
        var_p_na_db1_slot: &mut f64,
        var_p_na_db2_slot: &mut f64,
        var_p_na_db3_slot: &mut f64,
        var_p_na_dn0_slot: &mut f64,
        var_p_na_dn1_slot: &mut f64,
        var_p_na_dn2_slot: &mut f64,
        var_p_na_dn3_slot: &mut f64,
        var_p_na_dn4_slot: &mut f64,
        var_p_na_dn5_slot: &mut f64,
        var_p_na_rdb0_slot: &mut f64,
        var_p_na_rdb1_slot: &mut f64,
        var_p_na_rdb2_slot: &mut f64,
        var_p_na_rdb3_slot: &mut f64,
        var_p_na_rdn0_slot: &mut f64,
        var_p_na_rdn1_slot: &mut f64,
        var_p_na_rdn2_slot: &mut f64,
        var_p_na_rdn3_slot: &mut f64,
        var_p_na_rdn4_slot: &mut f64,
        var_p_na_rdn5_slot: &mut f64,
        var_p_na_rv_slot: &mut f64,
        var_q_nqs_a_slot: &mut f64,
        var_q_nqs_a_db0_slot: &mut f64,
        var_q_nqs_a_db1_slot: &mut f64,
        var_q_nqs_a_db2_slot: &mut f64,
        var_q_nqs_a_db3_slot: &mut f64,
        var_q_nqs_a_dn0_slot: &mut f64,
        var_q_nqs_a_dn1_slot: &mut f64,
        var_q_nqs_a_dn2_slot: &mut f64,
        var_q_nqs_a_dn3_slot: &mut f64,
        var_q_nqs_a_dn4_slot: &mut f64,
        var_q_nqs_a_dn5_slot: &mut f64,
        var_q_nqs_a_rdb0_slot: &mut f64,
        var_q_nqs_a_rdb1_slot: &mut f64,
        var_q_nqs_a_rdb2_slot: &mut f64,
        var_q_nqs_a_rdb3_slot: &mut f64,
        var_q_nqs_a_rdn0_slot: &mut f64,
        var_q_nqs_a_rdn1_slot: &mut f64,
        var_q_nqs_a_rdn2_slot: &mut f64,
        var_q_nqs_a_rdn3_slot: &mut f64,
        var_q_nqs_a_rdn4_slot: &mut f64,
        var_q_nqs_a_rdn5_slot: &mut f64,
        var_q_nqs_a_rv_slot: &mut f64,
        var_q_pexa_slot: &mut f64,
        var_q_pexa_db0_slot: &mut f64,
        var_q_pexa_db1_slot: &mut f64,
        var_q_pexa_db2_slot: &mut f64,
        var_q_pexa_db3_slot: &mut f64,
        var_q_pexa_dn0_slot: &mut f64,
        var_q_pexa_dn1_slot: &mut f64,
        var_q_pexa_dn2_slot: &mut f64,
        var_q_pexa_dn3_slot: &mut f64,
        var_q_pexa_dn4_slot: &mut f64,
        var_q_pexa_dn5_slot: &mut f64,
        var_q_pexa_rdb0_slot: &mut f64,
        var_q_pexa_rdb1_slot: &mut f64,
        var_q_pexa_rdb2_slot: &mut f64,
        var_q_pexa_rdb3_slot: &mut f64,
        var_q_pexa_rdn0_slot: &mut f64,
        var_q_pexa_rdn1_slot: &mut f64,
        var_q_pexa_rdn2_slot: &mut f64,
        var_q_pexa_rdn3_slot: &mut f64,
        var_q_pexa_rdn4_slot: &mut f64,
        var_q_pexa_rdn5_slot: &mut f64,
        var_q_pexa_rv_slot: &mut f64,
        var_q_qs_a_slot: &mut f64,
        var_q_qs_a_db0_slot: &mut f64,
        var_q_qs_a_db1_slot: &mut f64,
        var_q_qs_a_db2_slot: &mut f64,
        var_q_qs_a_db3_slot: &mut f64,
        var_q_qs_a_dn0_slot: &mut f64,
        var_q_qs_a_dn1_slot: &mut f64,
        var_q_qs_a_dn2_slot: &mut f64,
        var_q_qs_a_dn3_slot: &mut f64,
        var_q_qs_a_dn4_slot: &mut f64,
        var_q_qs_a_dn5_slot: &mut f64,
        var_q_qs_a_rdb0_slot: &mut f64,
        var_q_qs_a_rdb1_slot: &mut f64,
        var_q_qs_a_rdb2_slot: &mut f64,
        var_q_qs_a_rdb3_slot: &mut f64,
        var_q_qs_a_rdn0_slot: &mut f64,
        var_q_qs_a_rdn1_slot: &mut f64,
        var_q_qs_a_rdn2_slot: &mut f64,
        var_q_qs_a_rdn3_slot: &mut f64,
        var_q_qs_a_rdn4_slot: &mut f64,
        var_q_qs_a_rdn5_slot: &mut f64,
        var_q_qs_a_rv_slot: &mut f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let mut var_exp_a2: f64 = *var_exp_a2_slot;
        let mut var_exp_a2_db0: f64 = *var_exp_a2_db0_slot;
        let mut var_exp_a2_db1: f64 = *var_exp_a2_db1_slot;
        let mut var_exp_a2_db2: f64 = *var_exp_a2_db2_slot;
        let mut var_exp_a2_db3: f64 = *var_exp_a2_db3_slot;
        let mut var_exp_a2_dn0: f64 = *var_exp_a2_dn0_slot;
        let mut var_exp_a2_dn1: f64 = *var_exp_a2_dn1_slot;
        let mut var_exp_a2_dn2: f64 = *var_exp_a2_dn2_slot;
        let mut var_exp_a2_dn3: f64 = *var_exp_a2_dn3_slot;
        let mut var_exp_a2_dn4: f64 = *var_exp_a2_dn4_slot;
        let mut var_exp_a2_dn5: f64 = *var_exp_a2_dn5_slot;
        let mut var_exp_a2_rdb0: f64 = *var_exp_a2_rdb0_slot;
        let mut var_exp_a2_rdb1: f64 = *var_exp_a2_rdb1_slot;
        let mut var_exp_a2_rdb2: f64 = *var_exp_a2_rdb2_slot;
        let mut var_exp_a2_rdb3: f64 = *var_exp_a2_rdb3_slot;
        let mut var_exp_a2_rdn0: f64 = *var_exp_a2_rdn0_slot;
        let mut var_exp_a2_rdn1: f64 = *var_exp_a2_rdn1_slot;
        let mut var_exp_a2_rdn2: f64 = *var_exp_a2_rdn2_slot;
        let mut var_exp_a2_rdn3: f64 = *var_exp_a2_rdn3_slot;
        let mut var_exp_a2_rdn4: f64 = *var_exp_a2_rdn4_slot;
        let mut var_exp_a2_rdn5: f64 = *var_exp_a2_rdn5_slot;
        let mut var_exp_a2_rv: f64 = *var_exp_a2_rv_slot;
        let mut var_exp_k: f64 = *var_exp_k_slot;
        let mut var_exp_k2: f64 = *var_exp_k2_slot;
        let mut var_exp_k2_db0: f64 = *var_exp_k2_db0_slot;
        let mut var_exp_k2_db1: f64 = *var_exp_k2_db1_slot;
        let mut var_exp_k2_db2: f64 = *var_exp_k2_db2_slot;
        let mut var_exp_k2_db3: f64 = *var_exp_k2_db3_slot;
        let mut var_exp_k2_dn0: f64 = *var_exp_k2_dn0_slot;
        let mut var_exp_k2_dn1: f64 = *var_exp_k2_dn1_slot;
        let mut var_exp_k2_dn2: f64 = *var_exp_k2_dn2_slot;
        let mut var_exp_k2_dn3: f64 = *var_exp_k2_dn3_slot;
        let mut var_exp_k2_dn4: f64 = *var_exp_k2_dn4_slot;
        let mut var_exp_k2_dn5: f64 = *var_exp_k2_dn5_slot;
        let mut var_exp_k2_rdb0: f64 = *var_exp_k2_rdb0_slot;
        let mut var_exp_k2_rdb1: f64 = *var_exp_k2_rdb1_slot;
        let mut var_exp_k2_rdb2: f64 = *var_exp_k2_rdb2_slot;
        let mut var_exp_k2_rdb3: f64 = *var_exp_k2_rdb3_slot;
        let mut var_exp_k2_rdn0: f64 = *var_exp_k2_rdn0_slot;
        let mut var_exp_k2_rdn1: f64 = *var_exp_k2_rdn1_slot;
        let mut var_exp_k2_rdn2: f64 = *var_exp_k2_rdn2_slot;
        let mut var_exp_k2_rdn3: f64 = *var_exp_k2_rdn3_slot;
        let mut var_exp_k2_rdn4: f64 = *var_exp_k2_rdn4_slot;
        let mut var_exp_k2_rdn5: f64 = *var_exp_k2_rdn5_slot;
        let mut var_exp_k2_rv: f64 = *var_exp_k2_rv_slot;
        let mut var_exp_k_db0: f64 = *var_exp_k_db0_slot;
        let mut var_exp_k_db1: f64 = *var_exp_k_db1_slot;
        let mut var_exp_k_db2: f64 = *var_exp_k_db2_slot;
        let mut var_exp_k_db3: f64 = *var_exp_k_db3_slot;
        let mut var_exp_k_dn0: f64 = *var_exp_k_dn0_slot;
        let mut var_exp_k_dn1: f64 = *var_exp_k_dn1_slot;
        let mut var_exp_k_dn2: f64 = *var_exp_k_dn2_slot;
        let mut var_exp_k_dn3: f64 = *var_exp_k_dn3_slot;
        let mut var_exp_k_dn4: f64 = *var_exp_k_dn4_slot;
        let mut var_exp_k_dn5: f64 = *var_exp_k_dn5_slot;
        let mut var_exp_k_rdb0: f64 = *var_exp_k_rdb0_slot;
        let mut var_exp_k_rdb1: f64 = *var_exp_k_rdb1_slot;
        let mut var_exp_k_rdb2: f64 = *var_exp_k_rdb2_slot;
        let mut var_exp_k_rdb3: f64 = *var_exp_k_rdb3_slot;
        let mut var_exp_k_rdn0: f64 = *var_exp_k_rdn0_slot;
        let mut var_exp_k_rdn1: f64 = *var_exp_k_rdn1_slot;
        let mut var_exp_k_rdn2: f64 = *var_exp_k_rdn2_slot;
        let mut var_exp_k_rdn3: f64 = *var_exp_k_rdn3_slot;
        let mut var_exp_k_rdn4: f64 = *var_exp_k_rdn4_slot;
        let mut var_exp_k_rdn5: f64 = *var_exp_k_rdn5_slot;
        let mut var_exp_k_rv: f64 = *var_exp_k_rv_slot;
        let mut var_guard563: f64 = *var_guard563_slot;
        let mut var_guard563_db0: f64 = *var_guard563_db0_slot;
        let mut var_guard563_db1: f64 = *var_guard563_db1_slot;
        let mut var_guard563_db2: f64 = *var_guard563_db2_slot;
        let mut var_guard563_db3: f64 = *var_guard563_db3_slot;
        let mut var_guard563_dn0: f64 = *var_guard563_dn0_slot;
        let mut var_guard563_dn1: f64 = *var_guard563_dn1_slot;
        let mut var_guard563_dn2: f64 = *var_guard563_dn2_slot;
        let mut var_guard563_dn3: f64 = *var_guard563_dn3_slot;
        let mut var_guard563_dn4: f64 = *var_guard563_dn4_slot;
        let mut var_guard563_dn5: f64 = *var_guard563_dn5_slot;
        let mut var_guard563_rdb0: f64 = *var_guard563_rdb0_slot;
        let mut var_guard563_rdb1: f64 = *var_guard563_rdb1_slot;
        let mut var_guard563_rdb2: f64 = *var_guard563_rdb2_slot;
        let mut var_guard563_rdb3: f64 = *var_guard563_rdb3_slot;
        let mut var_guard563_rdn0: f64 = *var_guard563_rdn0_slot;
        let mut var_guard563_rdn1: f64 = *var_guard563_rdn1_slot;
        let mut var_guard563_rdn2: f64 = *var_guard563_rdn2_slot;
        let mut var_guard563_rdn3: f64 = *var_guard563_rdn3_slot;
        let mut var_guard563_rdn4: f64 = *var_guard563_rdn4_slot;
        let mut var_guard563_rdn5: f64 = *var_guard563_rdn5_slot;
        let mut var_guard563_rv: f64 = *var_guard563_rv_slot;
        let mut var_guard564: f64 = *var_guard564_slot;
        let mut var_guard564_db0: f64 = *var_guard564_db0_slot;
        let mut var_guard564_db1: f64 = *var_guard564_db1_slot;
        let mut var_guard564_db2: f64 = *var_guard564_db2_slot;
        let mut var_guard564_db3: f64 = *var_guard564_db3_slot;
        let mut var_guard564_dn0: f64 = *var_guard564_dn0_slot;
        let mut var_guard564_dn1: f64 = *var_guard564_dn1_slot;
        let mut var_guard564_dn2: f64 = *var_guard564_dn2_slot;
        let mut var_guard564_dn3: f64 = *var_guard564_dn3_slot;
        let mut var_guard564_dn4: f64 = *var_guard564_dn4_slot;
        let mut var_guard564_dn5: f64 = *var_guard564_dn5_slot;
        let mut var_guard564_rdb0: f64 = *var_guard564_rdb0_slot;
        let mut var_guard564_rdb1: f64 = *var_guard564_rdb1_slot;
        let mut var_guard564_rdb2: f64 = *var_guard564_rdb2_slot;
        let mut var_guard564_rdb3: f64 = *var_guard564_rdb3_slot;
        let mut var_guard564_rdn0: f64 = *var_guard564_rdn0_slot;
        let mut var_guard564_rdn1: f64 = *var_guard564_rdn1_slot;
        let mut var_guard564_rdn2: f64 = *var_guard564_rdn2_slot;
        let mut var_guard564_rdn3: f64 = *var_guard564_rdn3_slot;
        let mut var_guard564_rdn4: f64 = *var_guard564_rdn4_slot;
        let mut var_guard564_rdn5: f64 = *var_guard564_rdn5_slot;
        let mut var_guard564_rv: f64 = *var_guard564_rv_slot;
        let mut var_guard565: f64 = *var_guard565_slot;
        let mut var_guard565_db0: f64 = *var_guard565_db0_slot;
        let mut var_guard565_db1: f64 = *var_guard565_db1_slot;
        let mut var_guard565_db2: f64 = *var_guard565_db2_slot;
        let mut var_guard565_db3: f64 = *var_guard565_db3_slot;
        let mut var_guard565_dn0: f64 = *var_guard565_dn0_slot;
        let mut var_guard565_dn1: f64 = *var_guard565_dn1_slot;
        let mut var_guard565_dn2: f64 = *var_guard565_dn2_slot;
        let mut var_guard565_dn3: f64 = *var_guard565_dn3_slot;
        let mut var_guard565_dn4: f64 = *var_guard565_dn4_slot;
        let mut var_guard565_dn5: f64 = *var_guard565_dn5_slot;
        let mut var_guard565_rdb0: f64 = *var_guard565_rdb0_slot;
        let mut var_guard565_rdb1: f64 = *var_guard565_rdb1_slot;
        let mut var_guard565_rdb2: f64 = *var_guard565_rdb2_slot;
        let mut var_guard565_rdb3: f64 = *var_guard565_rdb3_slot;
        let mut var_guard565_rdn0: f64 = *var_guard565_rdn0_slot;
        let mut var_guard565_rdn1: f64 = *var_guard565_rdn1_slot;
        let mut var_guard565_rdn2: f64 = *var_guard565_rdn2_slot;
        let mut var_guard565_rdn3: f64 = *var_guard565_rdn3_slot;
        let mut var_guard565_rdn4: f64 = *var_guard565_rdn4_slot;
        let mut var_guard565_rdn5: f64 = *var_guard565_rdn5_slot;
        let mut var_guard565_rv: f64 = *var_guard565_rv_slot;
        let mut var_inqs0_a: f64 = *var_inqs0_a_slot;
        let mut var_inqs0_a_db0: f64 = *var_inqs0_a_db0_slot;
        let mut var_inqs0_a_db1: f64 = *var_inqs0_a_db1_slot;
        let mut var_inqs0_a_db2: f64 = *var_inqs0_a_db2_slot;
        let mut var_inqs0_a_db3: f64 = *var_inqs0_a_db3_slot;
        let mut var_inqs0_a_dn0: f64 = *var_inqs0_a_dn0_slot;
        let mut var_inqs0_a_dn1: f64 = *var_inqs0_a_dn1_slot;
        let mut var_inqs0_a_dn2: f64 = *var_inqs0_a_dn2_slot;
        let mut var_inqs0_a_dn3: f64 = *var_inqs0_a_dn3_slot;
        let mut var_inqs0_a_dn4: f64 = *var_inqs0_a_dn4_slot;
        let mut var_inqs0_a_dn5: f64 = *var_inqs0_a_dn5_slot;
        let mut var_inqs0_a_rdb0: f64 = *var_inqs0_a_rdb0_slot;
        let mut var_inqs0_a_rdb1: f64 = *var_inqs0_a_rdb1_slot;
        let mut var_inqs0_a_rdb2: f64 = *var_inqs0_a_rdb2_slot;
        let mut var_inqs0_a_rdb3: f64 = *var_inqs0_a_rdb3_slot;
        let mut var_inqs0_a_rdn0: f64 = *var_inqs0_a_rdn0_slot;
        let mut var_inqs0_a_rdn1: f64 = *var_inqs0_a_rdn1_slot;
        let mut var_inqs0_a_rdn2: f64 = *var_inqs0_a_rdn2_slot;
        let mut var_inqs0_a_rdn3: f64 = *var_inqs0_a_rdn3_slot;
        let mut var_inqs0_a_rdn4: f64 = *var_inqs0_a_rdn4_slot;
        let mut var_inqs0_a_rdn5: f64 = *var_inqs0_a_rdn5_slot;
        let mut var_inqs0_a_rv: f64 = *var_inqs0_a_rv_slot;
        let mut var_p_na: f64 = *var_p_na_slot;
        let mut var_p_na_db0: f64 = *var_p_na_db0_slot;
        let mut var_p_na_db1: f64 = *var_p_na_db1_slot;
        let mut var_p_na_db2: f64 = *var_p_na_db2_slot;
        let mut var_p_na_db3: f64 = *var_p_na_db3_slot;
        let mut var_p_na_dn0: f64 = *var_p_na_dn0_slot;
        let mut var_p_na_dn1: f64 = *var_p_na_dn1_slot;
        let mut var_p_na_dn2: f64 = *var_p_na_dn2_slot;
        let mut var_p_na_dn3: f64 = *var_p_na_dn3_slot;
        let mut var_p_na_dn4: f64 = *var_p_na_dn4_slot;
        let mut var_p_na_dn5: f64 = *var_p_na_dn5_slot;
        let mut var_p_na_rdb0: f64 = *var_p_na_rdb0_slot;
        let mut var_p_na_rdb1: f64 = *var_p_na_rdb1_slot;
        let mut var_p_na_rdb2: f64 = *var_p_na_rdb2_slot;
        let mut var_p_na_rdb3: f64 = *var_p_na_rdb3_slot;
        let mut var_p_na_rdn0: f64 = *var_p_na_rdn0_slot;
        let mut var_p_na_rdn1: f64 = *var_p_na_rdn1_slot;
        let mut var_p_na_rdn2: f64 = *var_p_na_rdn2_slot;
        let mut var_p_na_rdn3: f64 = *var_p_na_rdn3_slot;
        let mut var_p_na_rdn4: f64 = *var_p_na_rdn4_slot;
        let mut var_p_na_rdn5: f64 = *var_p_na_rdn5_slot;
        let mut var_p_na_rv: f64 = *var_p_na_rv_slot;
        let mut var_q_nqs_a: f64 = *var_q_nqs_a_slot;
        let mut var_q_nqs_a_db0: f64 = *var_q_nqs_a_db0_slot;
        let mut var_q_nqs_a_db1: f64 = *var_q_nqs_a_db1_slot;
        let mut var_q_nqs_a_db2: f64 = *var_q_nqs_a_db2_slot;
        let mut var_q_nqs_a_db3: f64 = *var_q_nqs_a_db3_slot;
        let mut var_q_nqs_a_dn0: f64 = *var_q_nqs_a_dn0_slot;
        let mut var_q_nqs_a_dn1: f64 = *var_q_nqs_a_dn1_slot;
        let mut var_q_nqs_a_dn2: f64 = *var_q_nqs_a_dn2_slot;
        let mut var_q_nqs_a_dn3: f64 = *var_q_nqs_a_dn3_slot;
        let mut var_q_nqs_a_dn4: f64 = *var_q_nqs_a_dn4_slot;
        let mut var_q_nqs_a_dn5: f64 = *var_q_nqs_a_dn5_slot;
        let mut var_q_nqs_a_rdb0: f64 = *var_q_nqs_a_rdb0_slot;
        let mut var_q_nqs_a_rdb1: f64 = *var_q_nqs_a_rdb1_slot;
        let mut var_q_nqs_a_rdb2: f64 = *var_q_nqs_a_rdb2_slot;
        let mut var_q_nqs_a_rdb3: f64 = *var_q_nqs_a_rdb3_slot;
        let mut var_q_nqs_a_rdn0: f64 = *var_q_nqs_a_rdn0_slot;
        let mut var_q_nqs_a_rdn1: f64 = *var_q_nqs_a_rdn1_slot;
        let mut var_q_nqs_a_rdn2: f64 = *var_q_nqs_a_rdn2_slot;
        let mut var_q_nqs_a_rdn3: f64 = *var_q_nqs_a_rdn3_slot;
        let mut var_q_nqs_a_rdn4: f64 = *var_q_nqs_a_rdn4_slot;
        let mut var_q_nqs_a_rdn5: f64 = *var_q_nqs_a_rdn5_slot;
        let mut var_q_nqs_a_rv: f64 = *var_q_nqs_a_rv_slot;
        let mut var_q_pexa: f64 = *var_q_pexa_slot;
        let mut var_q_pexa_db0: f64 = *var_q_pexa_db0_slot;
        let mut var_q_pexa_db1: f64 = *var_q_pexa_db1_slot;
        let mut var_q_pexa_db2: f64 = *var_q_pexa_db2_slot;
        let mut var_q_pexa_db3: f64 = *var_q_pexa_db3_slot;
        let mut var_q_pexa_dn0: f64 = *var_q_pexa_dn0_slot;
        let mut var_q_pexa_dn1: f64 = *var_q_pexa_dn1_slot;
        let mut var_q_pexa_dn2: f64 = *var_q_pexa_dn2_slot;
        let mut var_q_pexa_dn3: f64 = *var_q_pexa_dn3_slot;
        let mut var_q_pexa_dn4: f64 = *var_q_pexa_dn4_slot;
        let mut var_q_pexa_dn5: f64 = *var_q_pexa_dn5_slot;
        let mut var_q_pexa_rdb0: f64 = *var_q_pexa_rdb0_slot;
        let mut var_q_pexa_rdb1: f64 = *var_q_pexa_rdb1_slot;
        let mut var_q_pexa_rdb2: f64 = *var_q_pexa_rdb2_slot;
        let mut var_q_pexa_rdb3: f64 = *var_q_pexa_rdb3_slot;
        let mut var_q_pexa_rdn0: f64 = *var_q_pexa_rdn0_slot;
        let mut var_q_pexa_rdn1: f64 = *var_q_pexa_rdn1_slot;
        let mut var_q_pexa_rdn2: f64 = *var_q_pexa_rdn2_slot;
        let mut var_q_pexa_rdn3: f64 = *var_q_pexa_rdn3_slot;
        let mut var_q_pexa_rdn4: f64 = *var_q_pexa_rdn4_slot;
        let mut var_q_pexa_rdn5: f64 = *var_q_pexa_rdn5_slot;
        let mut var_q_pexa_rv: f64 = *var_q_pexa_rv_slot;
        let mut var_q_qs_a: f64 = *var_q_qs_a_slot;
        let mut var_q_qs_a_db0: f64 = *var_q_qs_a_db0_slot;
        let mut var_q_qs_a_db1: f64 = *var_q_qs_a_db1_slot;
        let mut var_q_qs_a_db2: f64 = *var_q_qs_a_db2_slot;
        let mut var_q_qs_a_db3: f64 = *var_q_qs_a_db3_slot;
        let mut var_q_qs_a_dn0: f64 = *var_q_qs_a_dn0_slot;
        let mut var_q_qs_a_dn1: f64 = *var_q_qs_a_dn1_slot;
        let mut var_q_qs_a_dn2: f64 = *var_q_qs_a_dn2_slot;
        let mut var_q_qs_a_dn3: f64 = *var_q_qs_a_dn3_slot;
        let mut var_q_qs_a_dn4: f64 = *var_q_qs_a_dn4_slot;
        let mut var_q_qs_a_dn5: f64 = *var_q_qs_a_dn5_slot;
        let mut var_q_qs_a_rdb0: f64 = *var_q_qs_a_rdb0_slot;
        let mut var_q_qs_a_rdb1: f64 = *var_q_qs_a_rdb1_slot;
        let mut var_q_qs_a_rdb2: f64 = *var_q_qs_a_rdb2_slot;
        let mut var_q_qs_a_rdb3: f64 = *var_q_qs_a_rdb3_slot;
        let mut var_q_qs_a_rdn0: f64 = *var_q_qs_a_rdn0_slot;
        let mut var_q_qs_a_rdn1: f64 = *var_q_qs_a_rdn1_slot;
        let mut var_q_qs_a_rdn2: f64 = *var_q_qs_a_rdn2_slot;
        let mut var_q_qs_a_rdn3: f64 = *var_q_qs_a_rdn3_slot;
        let mut var_q_qs_a_rdn4: f64 = *var_q_qs_a_rdn4_slot;
        let mut var_q_qs_a_rdn5: f64 = *var_q_qs_a_rdn5_slot;
        let mut var_q_qs_a_rv: f64 = *var_q_qs_a_rv_slot;

        let (assign34490_e52002, assign34490_e52002_d_n0, assign34490_e52002_d_n1, assign34490_e52002_d_n2, assign34490_e52002_d_n3, assign34490_e52002_d_n4, assign34490_e52002_d_n5, assign34490_e52002_d_b0, assign34490_e52002_d_b1, assign34490_e52002_d_b2, assign34490_e52002_d_b3,) = {
    if ((((var_guard558 != 0.0) && (var_guard560 != 0.0)) && (var_guard561 == 0.0)) && (var_guard562 == 0.0)) {
        let assign34490_e51923: f64 = (var_vak / var_nj_k);
        let assign34490_e51926: f64 = (var_v_hk - var_v_ha);
        let assign34490_e51928: f64 = (assign34490_e51926 / var_nj_k);
        let assign34490_e51929: f64 = (assign34490_e51923 - assign34490_e51928);
        let assign34490_e51933: f64 = (var_nj_k - var_nj0);
        let assign34490_e51934: f64 = (var_v_hk * assign34490_e51933);
        let assign34490_e51937: f64 = (var_nj0 * p.p85);
        let assign34490_e51938: f64 = (assign34490_e51934 / assign34490_e51937);
        let assign34490_e51939: f64 = (assign34490_e51929 + assign34490_e51938);
        let assign34490_e51940: f64 = (var_phitdinv * assign34490_e51939);
        let assign34490_e51942: f64 = (assign34490_e51940 - 230.25850929940458);
        let assign34490_e51948: f64 = (var_vak / var_nj_k);
        let assign34490_e51951: f64 = (var_v_hk - var_v_ha);
        let assign34490_e51953: f64 = (assign34490_e51951 / var_nj_k);
        let assign34490_e51954: f64 = (assign34490_e51948 - assign34490_e51953);
        let assign34490_e51958: f64 = (var_nj_k - var_nj0);
        let assign34490_e51959: f64 = (var_v_hk * assign34490_e51958);
        let assign34490_e51962: f64 = (var_nj0 * p.p85);
        let assign34490_e51963: f64 = (assign34490_e51959 / assign34490_e51962);
        let assign34490_e51964: f64 = (assign34490_e51954 + assign34490_e51963);
        let assign34490_e51965: f64 = (var_phitdinv * assign34490_e51964);
        let assign34490_e51967: f64 = (assign34490_e51965 - 230.25850929940458);
        let assign34490_e51972: f64 = (var_vak / var_nj_k);
        let assign34490_e51975: f64 = (var_v_hk - var_v_ha);
        let assign34490_e51977: f64 = (assign34490_e51975 / var_nj_k);
        let assign34490_e51978: f64 = (assign34490_e51972 - assign34490_e51977);
        let assign34490_e51982: f64 = (var_nj_k - var_nj0);
        let assign34490_e51983: f64 = (var_v_hk * assign34490_e51982);
        let assign34490_e51986: f64 = (var_nj0 * p.p85);
        let assign34490_e51987: f64 = (assign34490_e51983 / assign34490_e51986);
        let assign34490_e51988: f64 = (assign34490_e51978 + assign34490_e51987);
        let assign34490_e51989: f64 = (var_phitdinv * assign34490_e51988);
        let assign34490_e51991: f64 = (assign34490_e51989 - 230.25850929940458);
        let assign34490_e51993: f64 = (assign34490_e51991 * 0.3333333333333333);
        let assign34490_e51994: f64 = (1.0 + assign34490_e51993);
        let assign34490_e51995: f64 = (assign34490_e51967 * assign34490_e51994);
        let assign34490_e51996: f64 = (0.5 * assign34490_e51995);
        let assign34490_e51997: f64 = (1.0 + assign34490_e51996);
        let assign34490_e51998: f64 = (assign34490_e51942 * assign34490_e51997);
        let assign34490_e51999: f64 = (1.0 + assign34490_e51998);
        let assign34490_e52000: f64 = (1e100 * assign34490_e51999);
        (assign34490_e52000, (1e100 * ((((var_phitdinv_dn0 * assign34490_e51939) + (var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn0 - var_v_ha_dn0) * var_nj_k) - (assign34490_e51926 * var_nj_k_dn0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn0 * assign34490_e51933) + (var_v_hk * (var_nj_k_dn0 - var_nj0_dn0))) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn0 * p.p85))) / (assign34490_e51937 * assign34490_e51937))))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * ((((var_phitdinv_dn0 * assign34490_e51964) + (var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn0 - var_v_ha_dn0) * var_nj_k) - (assign34490_e51951 * var_nj_k_dn0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn0 * assign34490_e51958) + (var_v_hk * (var_nj_k_dn0 - var_nj0_dn0))) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn0 * p.p85))) / (assign34490_e51962 * assign34490_e51962))))) * assign34490_e51994) + (assign34490_e51967 * (((var_phitdinv_dn0 * assign34490_e51988) + (var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn0 - var_v_ha_dn0) * var_nj_k) - (assign34490_e51975 * var_nj_k_dn0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn0 * assign34490_e51982) + (var_v_hk * (var_nj_k_dn0 - var_nj0_dn0))) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn0 * p.p85))) / (assign34490_e51986 * assign34490_e51986))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_dn1 * assign34490_e51939) + (var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn1 - var_v_ha_dn1) * var_nj_k) - (assign34490_e51926 * var_nj_k_dn1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn1 * assign34490_e51933) + (var_v_hk * (var_nj_k_dn1 - var_nj0_dn1))) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn1 * p.p85))) / (assign34490_e51937 * assign34490_e51937))))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * ((((var_phitdinv_dn1 * assign34490_e51964) + (var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn1 - var_v_ha_dn1) * var_nj_k) - (assign34490_e51951 * var_nj_k_dn1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn1 * assign34490_e51958) + (var_v_hk * (var_nj_k_dn1 - var_nj0_dn1))) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn1 * p.p85))) / (assign34490_e51962 * assign34490_e51962))))) * assign34490_e51994) + (assign34490_e51967 * (((var_phitdinv_dn1 * assign34490_e51988) + (var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn1 - var_v_ha_dn1) * var_nj_k) - (assign34490_e51975 * var_nj_k_dn1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn1 * assign34490_e51982) + (var_v_hk * (var_nj_k_dn1 - var_nj0_dn1))) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn1 * p.p85))) / (assign34490_e51986 * assign34490_e51986))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_dn2 * assign34490_e51939) + (var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn2 - var_v_ha_dn2) * var_nj_k) - (assign34490_e51926 * var_nj_k_dn2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn2 * assign34490_e51933) + (var_v_hk * (var_nj_k_dn2 - var_nj0_dn2))) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn2 * p.p85))) / (assign34490_e51937 * assign34490_e51937))))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * ((((var_phitdinv_dn2 * assign34490_e51964) + (var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn2 - var_v_ha_dn2) * var_nj_k) - (assign34490_e51951 * var_nj_k_dn2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn2 * assign34490_e51958) + (var_v_hk * (var_nj_k_dn2 - var_nj0_dn2))) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn2 * p.p85))) / (assign34490_e51962 * assign34490_e51962))))) * assign34490_e51994) + (assign34490_e51967 * (((var_phitdinv_dn2 * assign34490_e51988) + (var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn2 - var_v_ha_dn2) * var_nj_k) - (assign34490_e51975 * var_nj_k_dn2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn2 * assign34490_e51982) + (var_v_hk * (var_nj_k_dn2 - var_nj0_dn2))) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn2 * p.p85))) / (assign34490_e51986 * assign34490_e51986))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_dn3 * assign34490_e51939) + (var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn3 - var_v_ha_dn3) * var_nj_k) - (assign34490_e51926 * var_nj_k_dn3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn3 * assign34490_e51933) + (var_v_hk * (var_nj_k_dn3 - var_nj0_dn3))) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn3 * p.p85))) / (assign34490_e51937 * assign34490_e51937))))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * ((((var_phitdinv_dn3 * assign34490_e51964) + (var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn3 - var_v_ha_dn3) * var_nj_k) - (assign34490_e51951 * var_nj_k_dn3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn3 * assign34490_e51958) + (var_v_hk * (var_nj_k_dn3 - var_nj0_dn3))) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn3 * p.p85))) / (assign34490_e51962 * assign34490_e51962))))) * assign34490_e51994) + (assign34490_e51967 * (((var_phitdinv_dn3 * assign34490_e51988) + (var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn3 - var_v_ha_dn3) * var_nj_k) - (assign34490_e51975 * var_nj_k_dn3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn3 * assign34490_e51982) + (var_v_hk * (var_nj_k_dn3 - var_nj0_dn3))) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn3 * p.p85))) / (assign34490_e51986 * assign34490_e51986))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_dn4 * assign34490_e51939) + (var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn4 - var_v_ha_dn4) * var_nj_k) - (assign34490_e51926 * var_nj_k_dn4)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn4 * assign34490_e51933) + (var_v_hk * (var_nj_k_dn4 - var_nj0_dn4))) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn4 * p.p85))) / (assign34490_e51937 * assign34490_e51937))))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * ((((var_phitdinv_dn4 * assign34490_e51964) + (var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn4 - var_v_ha_dn4) * var_nj_k) - (assign34490_e51951 * var_nj_k_dn4)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn4 * assign34490_e51958) + (var_v_hk * (var_nj_k_dn4 - var_nj0_dn4))) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn4 * p.p85))) / (assign34490_e51962 * assign34490_e51962))))) * assign34490_e51994) + (assign34490_e51967 * (((var_phitdinv_dn4 * assign34490_e51988) + (var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn4 - var_v_ha_dn4) * var_nj_k) - (assign34490_e51975 * var_nj_k_dn4)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn4 * assign34490_e51982) + (var_v_hk * (var_nj_k_dn4 - var_nj0_dn4))) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn4 * p.p85))) / (assign34490_e51986 * assign34490_e51986))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_dn5 * assign34490_e51939) + (var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn5 - var_v_ha_dn5) * var_nj_k) - (assign34490_e51926 * var_nj_k_dn5)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn5 * assign34490_e51933) + (var_v_hk * (var_nj_k_dn5 - var_nj0_dn5))) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn5 * p.p85))) / (assign34490_e51937 * assign34490_e51937))))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * ((((var_phitdinv_dn5 * assign34490_e51964) + (var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn5 - var_v_ha_dn5) * var_nj_k) - (assign34490_e51951 * var_nj_k_dn5)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn5 * assign34490_e51958) + (var_v_hk * (var_nj_k_dn5 - var_nj0_dn5))) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn5 * p.p85))) / (assign34490_e51962 * assign34490_e51962))))) * assign34490_e51994) + (assign34490_e51967 * (((var_phitdinv_dn5 * assign34490_e51988) + (var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_dn5 - var_v_ha_dn5) * var_nj_k) - (assign34490_e51975 * var_nj_k_dn5)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_dn5 * assign34490_e51982) + (var_v_hk * (var_nj_k_dn5 - var_nj0_dn5))) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn5 * p.p85))) / (assign34490_e51986 * assign34490_e51986))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_db0 * assign34490_e51939) + (var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db0 - var_v_ha_db0) * var_nj_k) - (assign34490_e51926 * var_nj_k_db0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db0 * assign34490_e51933) + (var_v_hk * (var_nj_k_db0 - var_nj0_db0))) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_db0 * p.p85))) / (assign34490_e51937 * assign34490_e51937))))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * ((((var_phitdinv_db0 * assign34490_e51964) + (var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db0 - var_v_ha_db0) * var_nj_k) - (assign34490_e51951 * var_nj_k_db0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db0 * assign34490_e51958) + (var_v_hk * (var_nj_k_db0 - var_nj0_db0))) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_db0 * p.p85))) / (assign34490_e51962 * assign34490_e51962))))) * assign34490_e51994) + (assign34490_e51967 * (((var_phitdinv_db0 * assign34490_e51988) + (var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db0 - var_v_ha_db0) * var_nj_k) - (assign34490_e51975 * var_nj_k_db0)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db0 * assign34490_e51982) + (var_v_hk * (var_nj_k_db0 - var_nj0_db0))) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_db0 * p.p85))) / (assign34490_e51986 * assign34490_e51986))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_db1 * assign34490_e51939) + (var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db1 - var_v_ha_db1) * var_nj_k) - (assign34490_e51926 * var_nj_k_db1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db1 * assign34490_e51933) + (var_v_hk * (var_nj_k_db1 - var_nj0_db1))) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_db1 * p.p85))) / (assign34490_e51937 * assign34490_e51937))))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * ((((var_phitdinv_db1 * assign34490_e51964) + (var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db1 - var_v_ha_db1) * var_nj_k) - (assign34490_e51951 * var_nj_k_db1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db1 * assign34490_e51958) + (var_v_hk * (var_nj_k_db1 - var_nj0_db1))) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_db1 * p.p85))) / (assign34490_e51962 * assign34490_e51962))))) * assign34490_e51994) + (assign34490_e51967 * (((var_phitdinv_db1 * assign34490_e51988) + (var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db1 - var_v_ha_db1) * var_nj_k) - (assign34490_e51975 * var_nj_k_db1)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db1 * assign34490_e51982) + (var_v_hk * (var_nj_k_db1 - var_nj0_db1))) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_db1 * p.p85))) / (assign34490_e51986 * assign34490_e51986))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_db2 * assign34490_e51939) + (var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db2 - var_v_ha_db2) * var_nj_k) - (assign34490_e51926 * var_nj_k_db2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db2 * assign34490_e51933) + (var_v_hk * (var_nj_k_db2 - var_nj0_db2))) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_db2 * p.p85))) / (assign34490_e51937 * assign34490_e51937))))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * ((((var_phitdinv_db2 * assign34490_e51964) + (var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db2 - var_v_ha_db2) * var_nj_k) - (assign34490_e51951 * var_nj_k_db2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db2 * assign34490_e51958) + (var_v_hk * (var_nj_k_db2 - var_nj0_db2))) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_db2 * p.p85))) / (assign34490_e51962 * assign34490_e51962))))) * assign34490_e51994) + (assign34490_e51967 * (((var_phitdinv_db2 * assign34490_e51988) + (var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db2 - var_v_ha_db2) * var_nj_k) - (assign34490_e51975 * var_nj_k_db2)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db2 * assign34490_e51982) + (var_v_hk * (var_nj_k_db2 - var_nj0_db2))) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_db2 * p.p85))) / (assign34490_e51986 * assign34490_e51986))))) * 0.3333333333333333))))))), (1e100 * ((((var_phitdinv_db3 * assign34490_e51939) + (var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db3 - var_v_ha_db3) * var_nj_k) - (assign34490_e51926 * var_nj_k_db3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db3 * assign34490_e51933) + (var_v_hk * (var_nj_k_db3 - var_nj0_db3))) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_db3 * p.p85))) / (assign34490_e51937 * assign34490_e51937))))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * ((((var_phitdinv_db3 * assign34490_e51964) + (var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db3 - var_v_ha_db3) * var_nj_k) - (assign34490_e51951 * var_nj_k_db3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db3 * assign34490_e51958) + (var_v_hk * (var_nj_k_db3 - var_nj0_db3))) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_db3 * p.p85))) / (assign34490_e51962 * assign34490_e51962))))) * assign34490_e51994) + (assign34490_e51967 * (((var_phitdinv_db3 * assign34490_e51988) + (var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - ((((var_v_hk_db3 - var_v_ha_db3) * var_nj_k) - (assign34490_e51975 * var_nj_k_db3)) / (var_nj_k * var_nj_k))) + (((((var_v_hk_db3 * assign34490_e51982) + (var_v_hk * (var_nj_k_db3 - var_nj0_db3))) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_db3 * p.p85))) / (assign34490_e51986 * assign34490_e51986))))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_k, var_exp_k_dn0, var_exp_k_dn1, var_exp_k_dn2, var_exp_k_dn3, var_exp_k_dn4, var_exp_k_dn5, var_exp_k_db0, var_exp_k_db1, var_exp_k_db2, var_exp_k_db3,)
    }
};
        var_exp_k = assign34490_e52002;
        var_exp_k_dn0 = assign34490_e52002_d_n0;
        var_exp_k_dn1 = assign34490_e52002_d_n1;
        var_exp_k_dn2 = assign34490_e52002_d_n2;
        var_exp_k_dn3 = assign34490_e52002_d_n3;
        var_exp_k_dn4 = assign34490_e52002_d_n4;
        var_exp_k_dn5 = assign34490_e52002_d_n5;
        var_exp_k_db0 = assign34490_e52002_d_b0;
        var_exp_k_db1 = assign34490_e52002_d_b1;
        var_exp_k_db2 = assign34490_e52002_d_b2;
        var_exp_k_db3 = assign34490_e52002_d_b3;
        var_exp_k_rv = 0.0;
        var_exp_k_rdn0 = 0.0;
        var_exp_k_rdn1 = 0.0;
        var_exp_k_rdn2 = 0.0;
        var_exp_k_rdn3 = 0.0;
        var_exp_k_rdn4 = 0.0;
        var_exp_k_rdn5 = 0.0;
        var_exp_k_rdb0 = 0.0;
        var_exp_k_rdb1 = 0.0;
        var_exp_k_rdb2 = 0.0;
        var_exp_k_rdb3 = 0.0;

        let (assign34500_e52009, assign34500_e52009_d_n0, assign34500_e52009_d_n1, assign34500_e52009_d_n2, assign34500_e52009_d_n3, assign34500_e52009_d_n4, assign34500_e52009_d_n5, assign34500_e52009_d_b0, assign34500_e52009_d_b1, assign34500_e52009_d_b2, assign34500_e52009_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard560 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_exp_k, var_exp_k_dn0, var_exp_k_dn1, var_exp_k_dn2, var_exp_k_dn3, var_exp_k_dn4, var_exp_k_dn5, var_exp_k_db0, var_exp_k_db1, var_exp_k_db2, var_exp_k_db3,)
    }
};
        var_exp_k = assign34500_e52009;
        var_exp_k_dn0 = assign34500_e52009_d_n0;
        var_exp_k_dn1 = assign34500_e52009_d_n1;
        var_exp_k_dn2 = assign34500_e52009_d_n2;
        var_exp_k_dn3 = assign34500_e52009_d_n3;
        var_exp_k_dn4 = assign34500_e52009_d_n4;
        var_exp_k_dn5 = assign34500_e52009_d_n5;
        var_exp_k_db0 = assign34500_e52009_d_b0;
        var_exp_k_db1 = assign34500_e52009_d_b1;
        var_exp_k_db2 = assign34500_e52009_d_b2;
        var_exp_k_db3 = assign34500_e52009_d_b3;
        var_exp_k_rv = 0.0;
        var_exp_k_rdn0 = 0.0;
        var_exp_k_rdn1 = 0.0;
        var_exp_k_rdn2 = 0.0;
        var_exp_k_rdn3 = 0.0;
        var_exp_k_rdn4 = 0.0;
        var_exp_k_rdn5 = 0.0;
        var_exp_k_rdb0 = 0.0;
        var_exp_k_rdb1 = 0.0;
        var_exp_k_rdb2 = 0.0;
        var_exp_k_rdb3 = 0.0;

        let assign34510_e52016: f64 = if ((p.p91 == 0.0) || (var_vak < var_v_ha)) { 1.0 } else { 0.0 };
        var_guard563 = assign34510_e52016;
        var_guard563_dn0 = 0.0;
        var_guard563_dn1 = 0.0;
        var_guard563_dn2 = 0.0;
        var_guard563_dn3 = 0.0;
        var_guard563_dn4 = 0.0;
        var_guard563_dn5 = 0.0;
        var_guard563_db0 = 0.0;
        var_guard563_db1 = 0.0;
        var_guard563_db2 = 0.0;
        var_guard563_db3 = 0.0;
        var_guard563_rv = 0.0;
        var_guard563_rdn0 = 0.0;
        var_guard563_rdn1 = 0.0;
        var_guard563_rdn2 = 0.0;
        var_guard563_rdn3 = 0.0;
        var_guard563_rdn4 = 0.0;
        var_guard563_rdn5 = 0.0;
        var_guard563_rdb0 = 0.0;
        var_guard563_rdb1 = 0.0;
        var_guard563_rdb2 = 0.0;
        var_guard563_rdb3 = 0.0;

        let (assign34520_e52024, assign34520_e52024_d_n0, assign34520_e52024_d_n1, assign34520_e52024_d_n2, assign34520_e52024_d_n3, assign34520_e52024_d_n4, assign34520_e52024_d_n5, assign34520_e52024_d_b0, assign34520_e52024_d_b1, assign34520_e52024_d_b2, assign34520_e52024_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard563 != 0.0)) {
        let assign34520_e52022: f64 = (var_exp_a * p.p90);
        (assign34520_e52022, (var_exp_a_dn0 * p.p90), (var_exp_a_dn1 * p.p90), (var_exp_a_dn2 * p.p90), (var_exp_a_dn3 * p.p90), (var_exp_a_dn4 * p.p90), (var_exp_a_dn5 * p.p90), (var_exp_a_db0 * p.p90), (var_exp_a_db1 * p.p90), (var_exp_a_db2 * p.p90), (var_exp_a_db3 * p.p90),)
    } else {
        (var_exp_a2, var_exp_a2_dn0, var_exp_a2_dn1, var_exp_a2_dn2, var_exp_a2_dn3, var_exp_a2_dn4, var_exp_a2_dn5, var_exp_a2_db0, var_exp_a2_db1, var_exp_a2_db2, var_exp_a2_db3,)
    }
};
        var_exp_a2 = assign34520_e52024;
        var_exp_a2_dn0 = assign34520_e52024_d_n0;
        var_exp_a2_dn1 = assign34520_e52024_d_n1;
        var_exp_a2_dn2 = assign34520_e52024_d_n2;
        var_exp_a2_dn3 = assign34520_e52024_d_n3;
        var_exp_a2_dn4 = assign34520_e52024_d_n4;
        var_exp_a2_dn5 = assign34520_e52024_d_n5;
        var_exp_a2_db0 = assign34520_e52024_d_b0;
        var_exp_a2_db1 = assign34520_e52024_d_b1;
        var_exp_a2_db2 = assign34520_e52024_d_b2;
        var_exp_a2_db3 = assign34520_e52024_d_b3;
        var_exp_a2_rv = 0.0;
        var_exp_a2_rdn0 = 0.0;
        var_exp_a2_rdn1 = 0.0;
        var_exp_a2_rdn2 = 0.0;
        var_exp_a2_rdn3 = 0.0;
        var_exp_a2_rdn4 = 0.0;
        var_exp_a2_rdn5 = 0.0;
        var_exp_a2_rdb0 = 0.0;
        var_exp_a2_rdb1 = 0.0;
        var_exp_a2_rdb2 = 0.0;
        var_exp_a2_rdb3 = 0.0;

        let (assign34530_e52053, assign34530_e52053_d_n0, assign34530_e52053_d_n1, assign34530_e52053_d_n2, assign34530_e52053_d_n3, assign34530_e52053_d_n4, assign34530_e52053_d_n5, assign34530_e52053_d_b0, assign34530_e52053_d_b1, assign34530_e52053_d_b2, assign34530_e52053_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard563 == 0.0)) {
        let assign34530_e52031: f64 = (var_exp_a * p.p90);
        let assign34530_e52033: f64 = (-p.p91);
        let assign34530_e52036: f64 = (var_vak - var_v_ha);
        let assign34530_e52037: f64 = (assign34530_e52033 * assign34530_e52036);
        let assign34530_e52040: f64 = (var_vak - var_v_ha);
        let assign34530_e52041: f64 = (assign34530_e52037 * assign34530_e52040);
        let assign34530_e52045: f64 = (var_tkr / var_tkd);
        let assign34530_e52046: f64 = (assign34530_e52045).ln();
        let assign34530_e52047: f64 = (p.p98 * assign34530_e52046);
        let assign34530_e52048: f64 = (assign34530_e52047).exp();
        let assign34530_e52049: f64 = (assign34530_e52041 * assign34530_e52048);
        let assign34530_e52050: f64 = (assign34530_e52049).exp();
        let assign34530_e52051: f64 = (assign34530_e52031 * assign34530_e52050);
        (assign34530_e52051, (((var_exp_a_dn0 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * (((((assign34530_e52033 * (var_vak_dn0 - var_v_ha_dn0)) * assign34530_e52040) + (assign34530_e52037 * (var_vak_dn0 - var_v_ha_dn0))) * assign34530_e52048) + (assign34530_e52041 * (assign34530_e52048 * (p.p98 * ((((var_tkr_dn0 * var_tkd) - (var_tkr * var_tkd_dn0)) / (var_tkd * var_tkd)) / assign34530_e52045)))))))), (((var_exp_a_dn1 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * (((((assign34530_e52033 * (var_vak_dn1 - var_v_ha_dn1)) * assign34530_e52040) + (assign34530_e52037 * (var_vak_dn1 - var_v_ha_dn1))) * assign34530_e52048) + (assign34530_e52041 * (assign34530_e52048 * (p.p98 * ((((var_tkr_dn1 * var_tkd) - (var_tkr * var_tkd_dn1)) / (var_tkd * var_tkd)) / assign34530_e52045)))))))), (((var_exp_a_dn2 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * (((((assign34530_e52033 * (var_vak_dn2 - var_v_ha_dn2)) * assign34530_e52040) + (assign34530_e52037 * (var_vak_dn2 - var_v_ha_dn2))) * assign34530_e52048) + (assign34530_e52041 * (assign34530_e52048 * (p.p98 * ((((var_tkr_dn2 * var_tkd) - (var_tkr * var_tkd_dn2)) / (var_tkd * var_tkd)) / assign34530_e52045)))))))), (((var_exp_a_dn3 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * (((((assign34530_e52033 * (var_vak_dn3 - var_v_ha_dn3)) * assign34530_e52040) + (assign34530_e52037 * (var_vak_dn3 - var_v_ha_dn3))) * assign34530_e52048) + (assign34530_e52041 * (assign34530_e52048 * (p.p98 * ((((var_tkr_dn3 * var_tkd) - (var_tkr * var_tkd_dn3)) / (var_tkd * var_tkd)) / assign34530_e52045)))))))), (((var_exp_a_dn4 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * (((((assign34530_e52033 * (var_vak_dn4 - var_v_ha_dn4)) * assign34530_e52040) + (assign34530_e52037 * (var_vak_dn4 - var_v_ha_dn4))) * assign34530_e52048) + (assign34530_e52041 * (assign34530_e52048 * (p.p98 * ((((var_tkr_dn4 * var_tkd) - (var_tkr * var_tkd_dn4)) / (var_tkd * var_tkd)) / assign34530_e52045)))))))), (((var_exp_a_dn5 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * (((((assign34530_e52033 * (var_vak_dn5 - var_v_ha_dn5)) * assign34530_e52040) + (assign34530_e52037 * (var_vak_dn5 - var_v_ha_dn5))) * assign34530_e52048) + (assign34530_e52041 * (assign34530_e52048 * (p.p98 * ((((var_tkr_dn5 * var_tkd) - (var_tkr * var_tkd_dn5)) / (var_tkd * var_tkd)) / assign34530_e52045)))))))), (((var_exp_a_db0 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * (((((assign34530_e52033 * (var_vak_db0 - var_v_ha_db0)) * assign34530_e52040) + (assign34530_e52037 * (var_vak_db0 - var_v_ha_db0))) * assign34530_e52048) + (assign34530_e52041 * (assign34530_e52048 * (p.p98 * ((((var_tkr_db0 * var_tkd) - (var_tkr * var_tkd_db0)) / (var_tkd * var_tkd)) / assign34530_e52045)))))))), (((var_exp_a_db1 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * (((((assign34530_e52033 * (var_vak_db1 - var_v_ha_db1)) * assign34530_e52040) + (assign34530_e52037 * (var_vak_db1 - var_v_ha_db1))) * assign34530_e52048) + (assign34530_e52041 * (assign34530_e52048 * (p.p98 * ((((var_tkr_db1 * var_tkd) - (var_tkr * var_tkd_db1)) / (var_tkd * var_tkd)) / assign34530_e52045)))))))), (((var_exp_a_db2 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * (((((assign34530_e52033 * (var_vak_db2 - var_v_ha_db2)) * assign34530_e52040) + (assign34530_e52037 * (var_vak_db2 - var_v_ha_db2))) * assign34530_e52048) + (assign34530_e52041 * (assign34530_e52048 * (p.p98 * ((((var_tkr_db2 * var_tkd) - (var_tkr * var_tkd_db2)) / (var_tkd * var_tkd)) / assign34530_e52045)))))))), (((var_exp_a_db3 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * (((((assign34530_e52033 * (var_vak_db3 - var_v_ha_db3)) * assign34530_e52040) + (assign34530_e52037 * (var_vak_db3 - var_v_ha_db3))) * assign34530_e52048) + (assign34530_e52041 * (assign34530_e52048 * (p.p98 * ((((var_tkr_db3 * var_tkd) - (var_tkr * var_tkd_db3)) / (var_tkd * var_tkd)) / assign34530_e52045)))))))),)
    } else {
        (var_exp_a2, var_exp_a2_dn0, var_exp_a2_dn1, var_exp_a2_dn2, var_exp_a2_dn3, var_exp_a2_dn4, var_exp_a2_dn5, var_exp_a2_db0, var_exp_a2_db1, var_exp_a2_db2, var_exp_a2_db3,)
    }
};
        var_exp_a2 = assign34530_e52053;
        var_exp_a2_dn0 = assign34530_e52053_d_n0;
        var_exp_a2_dn1 = assign34530_e52053_d_n1;
        var_exp_a2_dn2 = assign34530_e52053_d_n2;
        var_exp_a2_dn3 = assign34530_e52053_d_n3;
        var_exp_a2_dn4 = assign34530_e52053_d_n4;
        var_exp_a2_dn5 = assign34530_e52053_d_n5;
        var_exp_a2_db0 = assign34530_e52053_d_b0;
        var_exp_a2_db1 = assign34530_e52053_d_b1;
        var_exp_a2_db2 = assign34530_e52053_d_b2;
        var_exp_a2_db3 = assign34530_e52053_d_b3;
        var_exp_a2_rv = 0.0;
        var_exp_a2_rdn0 = 0.0;
        var_exp_a2_rdn1 = 0.0;
        var_exp_a2_rdn2 = 0.0;
        var_exp_a2_rdn3 = 0.0;
        var_exp_a2_rdn4 = 0.0;
        var_exp_a2_rdn5 = 0.0;
        var_exp_a2_rdb0 = 0.0;
        var_exp_a2_rdb1 = 0.0;
        var_exp_a2_rdb2 = 0.0;
        var_exp_a2_rdb3 = 0.0;

        let (assign34540_e52062, assign34540_e52062_d_n0, assign34540_e52062_d_n1, assign34540_e52062_d_n2, assign34540_e52062_d_n3, assign34540_e52062_d_n4, assign34540_e52062_d_n5, assign34540_e52062_d_b0, assign34540_e52062_d_b1, assign34540_e52062_d_b2, assign34540_e52062_d_b3,) = {
    if (var_guard558 != 0.0) {
        let (assign34540_e52060, assign34540_e52060_d_n0, assign34540_e52060_d_n1, assign34540_e52060_d_n2, assign34540_e52060_d_n3, assign34540_e52060_d_n4, assign34540_e52060_d_n5, assign34540_e52060_d_b0, assign34540_e52060_d_b1, assign34540_e52060_d_b2, assign34540_e52060_d_b3,) = {
            if (var_exp_a2 > p.p79) {
                (p.p79, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (var_exp_a2, var_exp_a2_dn0, var_exp_a2_dn1, var_exp_a2_dn2, var_exp_a2_dn3, var_exp_a2_dn4, var_exp_a2_dn5, var_exp_a2_db0, var_exp_a2_db1, var_exp_a2_db2, var_exp_a2_db3,)
            }
        };
        (assign34540_e52060, assign34540_e52060_d_n0, assign34540_e52060_d_n1, assign34540_e52060_d_n2, assign34540_e52060_d_n3, assign34540_e52060_d_n4, assign34540_e52060_d_n5, assign34540_e52060_d_b0, assign34540_e52060_d_b1, assign34540_e52060_d_b2, assign34540_e52060_d_b3,)
    } else {
        (var_exp_a2, var_exp_a2_dn0, var_exp_a2_dn1, var_exp_a2_dn2, var_exp_a2_dn3, var_exp_a2_dn4, var_exp_a2_dn5, var_exp_a2_db0, var_exp_a2_db1, var_exp_a2_db2, var_exp_a2_db3,)
    }
};
        var_exp_a2 = assign34540_e52062;
        var_exp_a2_dn0 = assign34540_e52062_d_n0;
        var_exp_a2_dn1 = assign34540_e52062_d_n1;
        var_exp_a2_dn2 = assign34540_e52062_d_n2;
        var_exp_a2_dn3 = assign34540_e52062_d_n3;
        var_exp_a2_dn4 = assign34540_e52062_d_n4;
        var_exp_a2_dn5 = assign34540_e52062_d_n5;
        var_exp_a2_db0 = assign34540_e52062_d_b0;
        var_exp_a2_db1 = assign34540_e52062_d_b1;
        var_exp_a2_db2 = assign34540_e52062_d_b2;
        var_exp_a2_db3 = assign34540_e52062_d_b3;
        var_exp_a2_rv = 0.0;
        var_exp_a2_rdn0 = 0.0;
        var_exp_a2_rdn1 = 0.0;
        var_exp_a2_rdn2 = 0.0;
        var_exp_a2_rdn3 = 0.0;
        var_exp_a2_rdn4 = 0.0;
        var_exp_a2_rdn5 = 0.0;
        var_exp_a2_rdb0 = 0.0;
        var_exp_a2_rdb1 = 0.0;
        var_exp_a2_rdb2 = 0.0;
        var_exp_a2_rdb3 = 0.0;

        let (assign34550_e52068, assign34550_e52068_d_n0, assign34550_e52068_d_n1, assign34550_e52068_d_n2, assign34550_e52068_d_n3, assign34550_e52068_d_n4, assign34550_e52068_d_n5, assign34550_e52068_d_b0, assign34550_e52068_d_b1, assign34550_e52068_d_b2, assign34550_e52068_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34550_e52066: f64 = (var_pn0 * var_exp_a2);
        (assign34550_e52066, ((var_pn0_dn0 * var_exp_a2) + (var_pn0 * var_exp_a2_dn0)), ((var_pn0_dn1 * var_exp_a2) + (var_pn0 * var_exp_a2_dn1)), ((var_pn0_dn2 * var_exp_a2) + (var_pn0 * var_exp_a2_dn2)), ((var_pn0_dn3 * var_exp_a2) + (var_pn0 * var_exp_a2_dn3)), ((var_pn0_dn4 * var_exp_a2) + (var_pn0 * var_exp_a2_dn4)), ((var_pn0_dn5 * var_exp_a2) + (var_pn0 * var_exp_a2_dn5)), ((var_pn0_db0 * var_exp_a2) + (var_pn0 * var_exp_a2_db0)), ((var_pn0_db1 * var_exp_a2) + (var_pn0 * var_exp_a2_db1)), ((var_pn0_db2 * var_exp_a2) + (var_pn0 * var_exp_a2_db2)), ((var_pn0_db3 * var_exp_a2) + (var_pn0 * var_exp_a2_db3)),)
    } else {
        (var_p_na, var_p_na_dn0, var_p_na_dn1, var_p_na_dn2, var_p_na_dn3, var_p_na_dn4, var_p_na_dn5, var_p_na_db0, var_p_na_db1, var_p_na_db2, var_p_na_db3,)
    }
};
        var_p_na = assign34550_e52068;
        var_p_na_dn0 = assign34550_e52068_d_n0;
        var_p_na_dn1 = assign34550_e52068_d_n1;
        var_p_na_dn2 = assign34550_e52068_d_n2;
        var_p_na_dn3 = assign34550_e52068_d_n3;
        var_p_na_dn4 = assign34550_e52068_d_n4;
        var_p_na_dn5 = assign34550_e52068_d_n5;
        var_p_na_db0 = assign34550_e52068_d_b0;
        var_p_na_db1 = assign34550_e52068_d_b1;
        var_p_na_db2 = assign34550_e52068_d_b2;
        var_p_na_db3 = assign34550_e52068_d_b3;
        var_p_na_rv = 0.0;
        var_p_na_rdn0 = 0.0;
        var_p_na_rdn1 = 0.0;
        var_p_na_rdn2 = 0.0;
        var_p_na_rdn3 = 0.0;
        var_p_na_rdn4 = 0.0;
        var_p_na_rdn5 = 0.0;
        var_p_na_rdb0 = 0.0;
        var_p_na_rdb1 = 0.0;
        var_p_na_rdb2 = 0.0;
        var_p_na_rdb3 = 0.0;

        let (assign34560_e52078, assign34560_e52078_d_n0, assign34560_e52078_d_n1, assign34560_e52078_d_n2, assign34560_e52078_d_n3, assign34560_e52078_d_n4, assign34560_e52078_d_n5, assign34560_e52078_d_b0, assign34560_e52078_d_b1, assign34560_e52078_d_b2, assign34560_e52078_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34560_e52072: f64 = (1.6021918e-19 * var_ab_i);
        let assign34560_e52075: f64 = (var_p_na - var_pn0);
        let assign34560_e52076: f64 = (assign34560_e52072 * assign34560_e52075);
        (assign34560_e52076, (((1.6021918e-19 * var_ab_i_dn0) * assign34560_e52075) + (assign34560_e52072 * (var_p_na_dn0 - var_pn0_dn0))), (((1.6021918e-19 * var_ab_i_dn1) * assign34560_e52075) + (assign34560_e52072 * (var_p_na_dn1 - var_pn0_dn1))), (((1.6021918e-19 * var_ab_i_dn2) * assign34560_e52075) + (assign34560_e52072 * (var_p_na_dn2 - var_pn0_dn2))), (((1.6021918e-19 * var_ab_i_dn3) * assign34560_e52075) + (assign34560_e52072 * (var_p_na_dn3 - var_pn0_dn3))), (((1.6021918e-19 * var_ab_i_dn4) * assign34560_e52075) + (assign34560_e52072 * (var_p_na_dn4 - var_pn0_dn4))), (((1.6021918e-19 * var_ab_i_dn5) * assign34560_e52075) + (assign34560_e52072 * (var_p_na_dn5 - var_pn0_dn5))), (((1.6021918e-19 * var_ab_i_db0) * assign34560_e52075) + (assign34560_e52072 * (var_p_na_db0 - var_pn0_db0))), (((1.6021918e-19 * var_ab_i_db1) * assign34560_e52075) + (assign34560_e52072 * (var_p_na_db1 - var_pn0_db1))), (((1.6021918e-19 * var_ab_i_db2) * assign34560_e52075) + (assign34560_e52072 * (var_p_na_db2 - var_pn0_db2))), (((1.6021918e-19 * var_ab_i_db3) * assign34560_e52075) + (assign34560_e52072 * (var_p_na_db3 - var_pn0_db3))),)
    } else {
        (var_q_pexa, var_q_pexa_dn0, var_q_pexa_dn1, var_q_pexa_dn2, var_q_pexa_dn3, var_q_pexa_dn4, var_q_pexa_dn5, var_q_pexa_db0, var_q_pexa_db1, var_q_pexa_db2, var_q_pexa_db3,)
    }
};
        var_q_pexa = assign34560_e52078;
        var_q_pexa_dn0 = assign34560_e52078_d_n0;
        var_q_pexa_dn1 = assign34560_e52078_d_n1;
        var_q_pexa_dn2 = assign34560_e52078_d_n2;
        var_q_pexa_dn3 = assign34560_e52078_d_n3;
        var_q_pexa_dn4 = assign34560_e52078_d_n4;
        var_q_pexa_dn5 = assign34560_e52078_d_n5;
        var_q_pexa_db0 = assign34560_e52078_d_b0;
        var_q_pexa_db1 = assign34560_e52078_d_b1;
        var_q_pexa_db2 = assign34560_e52078_d_b2;
        var_q_pexa_db3 = assign34560_e52078_d_b3;
        var_q_pexa_rv = 0.0;
        var_q_pexa_rdn0 = 0.0;
        var_q_pexa_rdn1 = 0.0;
        var_q_pexa_rdn2 = 0.0;
        var_q_pexa_rdn3 = 0.0;
        var_q_pexa_rdn4 = 0.0;
        var_q_pexa_rdn5 = 0.0;
        var_q_pexa_rdb0 = 0.0;
        var_q_pexa_rdb1 = 0.0;
        var_q_pexa_rdb2 = 0.0;
        var_q_pexa_rdb3 = 0.0;

        let assign34570_e52081: f64 = if p.p92 > 0.0 { 1.0 } else { 0.0 };
        var_guard564 = assign34570_e52081;
        var_guard564_dn0 = 0.0;
        var_guard564_dn1 = 0.0;
        var_guard564_dn2 = 0.0;
        var_guard564_dn3 = 0.0;
        var_guard564_dn4 = 0.0;
        var_guard564_dn5 = 0.0;
        var_guard564_db0 = 0.0;
        var_guard564_db1 = 0.0;
        var_guard564_db2 = 0.0;
        var_guard564_db3 = 0.0;
        var_guard564_rv = 0.0;
        var_guard564_rdn0 = 0.0;
        var_guard564_rdn1 = 0.0;
        var_guard564_rdn2 = 0.0;
        var_guard564_rdn3 = 0.0;
        var_guard564_rdn4 = 0.0;
        var_guard564_rdn5 = 0.0;
        var_guard564_rdb0 = 0.0;
        var_guard564_rdb1 = 0.0;
        var_guard564_rdb2 = 0.0;
        var_guard564_rdb3 = 0.0;

        let (assign34580_e52091, assign34580_e52091_d_n0, assign34580_e52091_d_n1, assign34580_e52091_d_n2, assign34580_e52091_d_n3, assign34580_e52091_d_n4, assign34580_e52091_d_n5, assign34580_e52091_d_b0, assign34580_e52091_d_b1, assign34580_e52091_d_b2, assign34580_e52091_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard564 != 0.0)) {
        let assign34580_e52088: f64 = (1e-23 / var_q_pex0);
        let assign34580_e52089: f64 = (var_q_pexa * assign34580_e52088);
        (assign34580_e52089, ((var_q_pexa_dn0 * assign34580_e52088) + (var_q_pexa * (-((1e-23 * var_q_pex0_dn0) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexa_dn1 * assign34580_e52088) + (var_q_pexa * (-((1e-23 * var_q_pex0_dn1) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexa_dn2 * assign34580_e52088) + (var_q_pexa * (-((1e-23 * var_q_pex0_dn2) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexa_dn3 * assign34580_e52088) + (var_q_pexa * (-((1e-23 * var_q_pex0_dn3) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexa_dn4 * assign34580_e52088) + (var_q_pexa * (-((1e-23 * var_q_pex0_dn4) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexa_dn5 * assign34580_e52088) + (var_q_pexa * (-((1e-23 * var_q_pex0_dn5) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexa_db0 * assign34580_e52088) + (var_q_pexa * (-((1e-23 * var_q_pex0_db0) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexa_db1 * assign34580_e52088) + (var_q_pexa * (-((1e-23 * var_q_pex0_db1) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexa_db2 * assign34580_e52088) + (var_q_pexa * (-((1e-23 * var_q_pex0_db2) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexa_db3 * assign34580_e52088) + (var_q_pexa * (-((1e-23 * var_q_pex0_db3) / (var_q_pex0 * var_q_pex0))))),)
    } else {
        (var_q_qs_a, var_q_qs_a_dn0, var_q_qs_a_dn1, var_q_qs_a_dn2, var_q_qs_a_dn3, var_q_qs_a_dn4, var_q_qs_a_dn5, var_q_qs_a_db0, var_q_qs_a_db1, var_q_qs_a_db2, var_q_qs_a_db3,)
    }
};
        var_q_qs_a = assign34580_e52091;
        var_q_qs_a_dn0 = assign34580_e52091_d_n0;
        var_q_qs_a_dn1 = assign34580_e52091_d_n1;
        var_q_qs_a_dn2 = assign34580_e52091_d_n2;
        var_q_qs_a_dn3 = assign34580_e52091_d_n3;
        var_q_qs_a_dn4 = assign34580_e52091_d_n4;
        var_q_qs_a_dn5 = assign34580_e52091_d_n5;
        var_q_qs_a_db0 = assign34580_e52091_d_b0;
        var_q_qs_a_db1 = assign34580_e52091_d_b1;
        var_q_qs_a_db2 = assign34580_e52091_d_b2;
        var_q_qs_a_db3 = assign34580_e52091_d_b3;
        var_q_qs_a_rv = 0.0;
        var_q_qs_a_rdn0 = 0.0;
        var_q_qs_a_rdn1 = 0.0;
        var_q_qs_a_rdn2 = 0.0;
        var_q_qs_a_rdn3 = 0.0;
        var_q_qs_a_rdn4 = 0.0;
        var_q_qs_a_rdn5 = 0.0;
        var_q_qs_a_rdb0 = 0.0;
        var_q_qs_a_rdb1 = 0.0;
        var_q_qs_a_rdb2 = 0.0;
        var_q_qs_a_rdb3 = 0.0;

        let (assign34590_e52099, assign34590_e52099_d_n0, assign34590_e52099_d_n1, assign34590_e52099_d_n2, assign34590_e52099_d_n3, assign34590_e52099_d_n4, assign34590_e52099_d_n5, assign34590_e52099_d_b0, assign34590_e52099_d_b1, assign34590_e52099_d_b2, assign34590_e52099_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard564 != 0.0)) {
        let assign34590_e52097: f64 = (nv3 - 0.0);
        (assign34590_e52097, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_q_nqs_a, var_q_nqs_a_dn0, var_q_nqs_a_dn1, var_q_nqs_a_dn2, var_q_nqs_a_dn3, var_q_nqs_a_dn4, var_q_nqs_a_dn5, var_q_nqs_a_db0, var_q_nqs_a_db1, var_q_nqs_a_db2, var_q_nqs_a_db3,)
    }
};
        var_q_nqs_a = assign34590_e52099;
        var_q_nqs_a_dn0 = assign34590_e52099_d_n0;
        var_q_nqs_a_dn1 = assign34590_e52099_d_n1;
        var_q_nqs_a_dn2 = assign34590_e52099_d_n2;
        var_q_nqs_a_dn3 = assign34590_e52099_d_n3;
        var_q_nqs_a_dn4 = assign34590_e52099_d_n4;
        var_q_nqs_a_dn5 = assign34590_e52099_d_n5;
        var_q_nqs_a_db0 = assign34590_e52099_d_b0;
        var_q_nqs_a_db1 = assign34590_e52099_d_b1;
        var_q_nqs_a_db2 = assign34590_e52099_d_b2;
        var_q_nqs_a_db3 = assign34590_e52099_d_b3;
        var_q_nqs_a_rv = 0.0;
        var_q_nqs_a_rdn0 = 0.0;
        var_q_nqs_a_rdn1 = 0.0;
        var_q_nqs_a_rdn2 = 0.0;
        var_q_nqs_a_rdn3 = 0.0;
        var_q_nqs_a_rdn4 = 0.0;
        var_q_nqs_a_rdn5 = 0.0;
        var_q_nqs_a_rdb0 = 0.0;
        var_q_nqs_a_rdb1 = 0.0;
        var_q_nqs_a_rdb2 = 0.0;
        var_q_nqs_a_rdb3 = 0.0;

        let (assign34600_e52109, assign34600_e52109_d_n0, assign34600_e52109_d_n1, assign34600_e52109_d_n2, assign34600_e52109_d_n3, assign34600_e52109_d_n4, assign34600_e52109_d_n5, assign34600_e52109_d_b0, assign34600_e52109_d_b1, assign34600_e52109_d_b2, assign34600_e52109_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard564 != 0.0)) {
        let assign34600_e52105: f64 = (var_q_nqs_a - var_q_qs_a);
        let assign34600_e52107: f64 = (assign34600_e52105 / p.p92);
        (assign34600_e52107, ((var_q_nqs_a_dn0 - var_q_qs_a_dn0) / p.p92), ((var_q_nqs_a_dn1 - var_q_qs_a_dn1) / p.p92), ((var_q_nqs_a_dn2 - var_q_qs_a_dn2) / p.p92), ((var_q_nqs_a_dn3 - var_q_qs_a_dn3) / p.p92), ((var_q_nqs_a_dn4 - var_q_qs_a_dn4) / p.p92), ((var_q_nqs_a_dn5 - var_q_qs_a_dn5) / p.p92), ((var_q_nqs_a_db0 - var_q_qs_a_db0) / p.p92), ((var_q_nqs_a_db1 - var_q_qs_a_db1) / p.p92), ((var_q_nqs_a_db2 - var_q_qs_a_db2) / p.p92), ((var_q_nqs_a_db3 - var_q_qs_a_db3) / p.p92),)
    } else {
        (var_inqs0_a, var_inqs0_a_dn0, var_inqs0_a_dn1, var_inqs0_a_dn2, var_inqs0_a_dn3, var_inqs0_a_dn4, var_inqs0_a_dn5, var_inqs0_a_db0, var_inqs0_a_db1, var_inqs0_a_db2, var_inqs0_a_db3,)
    }
};
        var_inqs0_a = assign34600_e52109;
        var_inqs0_a_dn0 = assign34600_e52109_d_n0;
        var_inqs0_a_dn1 = assign34600_e52109_d_n1;
        var_inqs0_a_dn2 = assign34600_e52109_d_n2;
        var_inqs0_a_dn3 = assign34600_e52109_d_n3;
        var_inqs0_a_dn4 = assign34600_e52109_d_n4;
        var_inqs0_a_dn5 = assign34600_e52109_d_n5;
        var_inqs0_a_db0 = assign34600_e52109_d_b0;
        var_inqs0_a_db1 = assign34600_e52109_d_b1;
        var_inqs0_a_db2 = assign34600_e52109_d_b2;
        var_inqs0_a_db3 = assign34600_e52109_d_b3;
        var_inqs0_a_rv = 0.0;
        var_inqs0_a_rdn0 = 0.0;
        var_inqs0_a_rdn1 = 0.0;
        var_inqs0_a_rdn2 = 0.0;
        var_inqs0_a_rdn3 = 0.0;
        var_inqs0_a_rdn4 = 0.0;
        var_inqs0_a_rdn5 = 0.0;
        var_inqs0_a_rdb0 = 0.0;
        var_inqs0_a_rdb1 = 0.0;
        var_inqs0_a_rdb2 = 0.0;
        var_inqs0_a_rdb3 = 0.0;

        let (assign34620_e52126, assign34620_e52126_d_n0, assign34620_e52126_d_n1, assign34620_e52126_d_n2, assign34620_e52126_d_n3, assign34620_e52126_d_n4, assign34620_e52126_d_n5, assign34620_e52126_d_b0, assign34620_e52126_d_b1, assign34620_e52126_d_b2, assign34620_e52126_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard564 == 0.0)) {
        (var_q_pexa, var_q_pexa_dn0, var_q_pexa_dn1, var_q_pexa_dn2, var_q_pexa_dn3, var_q_pexa_dn4, var_q_pexa_dn5, var_q_pexa_db0, var_q_pexa_db1, var_q_pexa_db2, var_q_pexa_db3,)
    } else {
        (var_q_qs_a, var_q_qs_a_dn0, var_q_qs_a_dn1, var_q_qs_a_dn2, var_q_qs_a_dn3, var_q_qs_a_dn4, var_q_qs_a_dn5, var_q_qs_a_db0, var_q_qs_a_db1, var_q_qs_a_db2, var_q_qs_a_db3,)
    }
};
        var_q_qs_a = assign34620_e52126;
        var_q_qs_a_dn0 = assign34620_e52126_d_n0;
        var_q_qs_a_dn1 = assign34620_e52126_d_n1;
        var_q_qs_a_dn2 = assign34620_e52126_d_n2;
        var_q_qs_a_dn3 = assign34620_e52126_d_n3;
        var_q_qs_a_dn4 = assign34620_e52126_d_n4;
        var_q_qs_a_dn5 = assign34620_e52126_d_n5;
        var_q_qs_a_db0 = assign34620_e52126_d_b0;
        var_q_qs_a_db1 = assign34620_e52126_d_b1;
        var_q_qs_a_db2 = assign34620_e52126_d_b2;
        var_q_qs_a_db3 = assign34620_e52126_d_b3;
        var_q_qs_a_rv = 0.0;
        var_q_qs_a_rdn0 = 0.0;
        var_q_qs_a_rdn1 = 0.0;
        var_q_qs_a_rdn2 = 0.0;
        var_q_qs_a_rdn3 = 0.0;
        var_q_qs_a_rdn4 = 0.0;
        var_q_qs_a_rdn5 = 0.0;
        var_q_qs_a_rdb0 = 0.0;
        var_q_qs_a_rdb1 = 0.0;
        var_q_qs_a_rdb2 = 0.0;
        var_q_qs_a_rdb3 = 0.0;

        let assign34640_e52140: f64 = if ((p.p91 == 0.0) || (var_vak < var_v_hk)) { 1.0 } else { 0.0 };
        var_guard565 = assign34640_e52140;
        var_guard565_dn0 = 0.0;
        var_guard565_dn1 = 0.0;
        var_guard565_dn2 = 0.0;
        var_guard565_dn3 = 0.0;
        var_guard565_dn4 = 0.0;
        var_guard565_dn5 = 0.0;
        var_guard565_db0 = 0.0;
        var_guard565_db1 = 0.0;
        var_guard565_db2 = 0.0;
        var_guard565_db3 = 0.0;
        var_guard565_rv = 0.0;
        var_guard565_rdn0 = 0.0;
        var_guard565_rdn1 = 0.0;
        var_guard565_rdn2 = 0.0;
        var_guard565_rdn3 = 0.0;
        var_guard565_rdn4 = 0.0;
        var_guard565_rdn5 = 0.0;
        var_guard565_rdb0 = 0.0;
        var_guard565_rdb1 = 0.0;
        var_guard565_rdb2 = 0.0;
        var_guard565_rdb3 = 0.0;

        let (assign34650_e52148, assign34650_e52148_d_n0, assign34650_e52148_d_n1, assign34650_e52148_d_n2, assign34650_e52148_d_n3, assign34650_e52148_d_n4, assign34650_e52148_d_n5, assign34650_e52148_d_b0, assign34650_e52148_d_b1, assign34650_e52148_d_b2, assign34650_e52148_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard565 != 0.0)) {
        let assign34650_e52146: f64 = (var_exp_k * p.p90);
        (assign34650_e52146, (var_exp_k_dn0 * p.p90), (var_exp_k_dn1 * p.p90), (var_exp_k_dn2 * p.p90), (var_exp_k_dn3 * p.p90), (var_exp_k_dn4 * p.p90), (var_exp_k_dn5 * p.p90), (var_exp_k_db0 * p.p90), (var_exp_k_db1 * p.p90), (var_exp_k_db2 * p.p90), (var_exp_k_db3 * p.p90),)
    } else {
        (var_exp_k2, var_exp_k2_dn0, var_exp_k2_dn1, var_exp_k2_dn2, var_exp_k2_dn3, var_exp_k2_dn4, var_exp_k2_dn5, var_exp_k2_db0, var_exp_k2_db1, var_exp_k2_db2, var_exp_k2_db3,)
    }
};
        var_exp_k2 = assign34650_e52148;
        var_exp_k2_dn0 = assign34650_e52148_d_n0;
        var_exp_k2_dn1 = assign34650_e52148_d_n1;
        var_exp_k2_dn2 = assign34650_e52148_d_n2;
        var_exp_k2_dn3 = assign34650_e52148_d_n3;
        var_exp_k2_dn4 = assign34650_e52148_d_n4;
        var_exp_k2_dn5 = assign34650_e52148_d_n5;
        var_exp_k2_db0 = assign34650_e52148_d_b0;
        var_exp_k2_db1 = assign34650_e52148_d_b1;
        var_exp_k2_db2 = assign34650_e52148_d_b2;
        var_exp_k2_db3 = assign34650_e52148_d_b3;
        var_exp_k2_rv = 0.0;
        var_exp_k2_rdn0 = 0.0;
        var_exp_k2_rdn1 = 0.0;
        var_exp_k2_rdn2 = 0.0;
        var_exp_k2_rdn3 = 0.0;
        var_exp_k2_rdn4 = 0.0;
        var_exp_k2_rdn5 = 0.0;
        var_exp_k2_rdb0 = 0.0;
        var_exp_k2_rdb1 = 0.0;
        var_exp_k2_rdb2 = 0.0;
        var_exp_k2_rdb3 = 0.0;

        *var_exp_a2_slot = var_exp_a2;
        *var_exp_a2_db0_slot = var_exp_a2_db0;
        *var_exp_a2_db1_slot = var_exp_a2_db1;
        *var_exp_a2_db2_slot = var_exp_a2_db2;
        *var_exp_a2_db3_slot = var_exp_a2_db3;
        *var_exp_a2_dn0_slot = var_exp_a2_dn0;
        *var_exp_a2_dn1_slot = var_exp_a2_dn1;
        *var_exp_a2_dn2_slot = var_exp_a2_dn2;
        *var_exp_a2_dn3_slot = var_exp_a2_dn3;
        *var_exp_a2_dn4_slot = var_exp_a2_dn4;
        *var_exp_a2_dn5_slot = var_exp_a2_dn5;
        *var_exp_a2_rdb0_slot = var_exp_a2_rdb0;
        *var_exp_a2_rdb1_slot = var_exp_a2_rdb1;
        *var_exp_a2_rdb2_slot = var_exp_a2_rdb2;
        *var_exp_a2_rdb3_slot = var_exp_a2_rdb3;
        *var_exp_a2_rdn0_slot = var_exp_a2_rdn0;
        *var_exp_a2_rdn1_slot = var_exp_a2_rdn1;
        *var_exp_a2_rdn2_slot = var_exp_a2_rdn2;
        *var_exp_a2_rdn3_slot = var_exp_a2_rdn3;
        *var_exp_a2_rdn4_slot = var_exp_a2_rdn4;
        *var_exp_a2_rdn5_slot = var_exp_a2_rdn5;
        *var_exp_a2_rv_slot = var_exp_a2_rv;
        *var_exp_k_slot = var_exp_k;
        *var_exp_k2_slot = var_exp_k2;
        *var_exp_k2_db0_slot = var_exp_k2_db0;
        *var_exp_k2_db1_slot = var_exp_k2_db1;
        *var_exp_k2_db2_slot = var_exp_k2_db2;
        *var_exp_k2_db3_slot = var_exp_k2_db3;
        *var_exp_k2_dn0_slot = var_exp_k2_dn0;
        *var_exp_k2_dn1_slot = var_exp_k2_dn1;
        *var_exp_k2_dn2_slot = var_exp_k2_dn2;
        *var_exp_k2_dn3_slot = var_exp_k2_dn3;
        *var_exp_k2_dn4_slot = var_exp_k2_dn4;
        *var_exp_k2_dn5_slot = var_exp_k2_dn5;
        *var_exp_k2_rdb0_slot = var_exp_k2_rdb0;
        *var_exp_k2_rdb1_slot = var_exp_k2_rdb1;
        *var_exp_k2_rdb2_slot = var_exp_k2_rdb2;
        *var_exp_k2_rdb3_slot = var_exp_k2_rdb3;
        *var_exp_k2_rdn0_slot = var_exp_k2_rdn0;
        *var_exp_k2_rdn1_slot = var_exp_k2_rdn1;
        *var_exp_k2_rdn2_slot = var_exp_k2_rdn2;
        *var_exp_k2_rdn3_slot = var_exp_k2_rdn3;
        *var_exp_k2_rdn4_slot = var_exp_k2_rdn4;
        *var_exp_k2_rdn5_slot = var_exp_k2_rdn5;
        *var_exp_k2_rv_slot = var_exp_k2_rv;
        *var_exp_k_db0_slot = var_exp_k_db0;
        *var_exp_k_db1_slot = var_exp_k_db1;
        *var_exp_k_db2_slot = var_exp_k_db2;
        *var_exp_k_db3_slot = var_exp_k_db3;
        *var_exp_k_dn0_slot = var_exp_k_dn0;
        *var_exp_k_dn1_slot = var_exp_k_dn1;
        *var_exp_k_dn2_slot = var_exp_k_dn2;
        *var_exp_k_dn3_slot = var_exp_k_dn3;
        *var_exp_k_dn4_slot = var_exp_k_dn4;
        *var_exp_k_dn5_slot = var_exp_k_dn5;
        *var_exp_k_rdb0_slot = var_exp_k_rdb0;
        *var_exp_k_rdb1_slot = var_exp_k_rdb1;
        *var_exp_k_rdb2_slot = var_exp_k_rdb2;
        *var_exp_k_rdb3_slot = var_exp_k_rdb3;
        *var_exp_k_rdn0_slot = var_exp_k_rdn0;
        *var_exp_k_rdn1_slot = var_exp_k_rdn1;
        *var_exp_k_rdn2_slot = var_exp_k_rdn2;
        *var_exp_k_rdn3_slot = var_exp_k_rdn3;
        *var_exp_k_rdn4_slot = var_exp_k_rdn4;
        *var_exp_k_rdn5_slot = var_exp_k_rdn5;
        *var_exp_k_rv_slot = var_exp_k_rv;
        *var_guard563_slot = var_guard563;
        *var_guard563_db0_slot = var_guard563_db0;
        *var_guard563_db1_slot = var_guard563_db1;
        *var_guard563_db2_slot = var_guard563_db2;
        *var_guard563_db3_slot = var_guard563_db3;
        *var_guard563_dn0_slot = var_guard563_dn0;
        *var_guard563_dn1_slot = var_guard563_dn1;
        *var_guard563_dn2_slot = var_guard563_dn2;
        *var_guard563_dn3_slot = var_guard563_dn3;
        *var_guard563_dn4_slot = var_guard563_dn4;
        *var_guard563_dn5_slot = var_guard563_dn5;
        *var_guard563_rdb0_slot = var_guard563_rdb0;
        *var_guard563_rdb1_slot = var_guard563_rdb1;
        *var_guard563_rdb2_slot = var_guard563_rdb2;
        *var_guard563_rdb3_slot = var_guard563_rdb3;
        *var_guard563_rdn0_slot = var_guard563_rdn0;
        *var_guard563_rdn1_slot = var_guard563_rdn1;
        *var_guard563_rdn2_slot = var_guard563_rdn2;
        *var_guard563_rdn3_slot = var_guard563_rdn3;
        *var_guard563_rdn4_slot = var_guard563_rdn4;
        *var_guard563_rdn5_slot = var_guard563_rdn5;
        *var_guard563_rv_slot = var_guard563_rv;
        *var_guard564_slot = var_guard564;
        *var_guard564_db0_slot = var_guard564_db0;
        *var_guard564_db1_slot = var_guard564_db1;
        *var_guard564_db2_slot = var_guard564_db2;
        *var_guard564_db3_slot = var_guard564_db3;
        *var_guard564_dn0_slot = var_guard564_dn0;
        *var_guard564_dn1_slot = var_guard564_dn1;
        *var_guard564_dn2_slot = var_guard564_dn2;
        *var_guard564_dn3_slot = var_guard564_dn3;
        *var_guard564_dn4_slot = var_guard564_dn4;
        *var_guard564_dn5_slot = var_guard564_dn5;
        *var_guard564_rdb0_slot = var_guard564_rdb0;
        *var_guard564_rdb1_slot = var_guard564_rdb1;
        *var_guard564_rdb2_slot = var_guard564_rdb2;
        *var_guard564_rdb3_slot = var_guard564_rdb3;
        *var_guard564_rdn0_slot = var_guard564_rdn0;
        *var_guard564_rdn1_slot = var_guard564_rdn1;
        *var_guard564_rdn2_slot = var_guard564_rdn2;
        *var_guard564_rdn3_slot = var_guard564_rdn3;
        *var_guard564_rdn4_slot = var_guard564_rdn4;
        *var_guard564_rdn5_slot = var_guard564_rdn5;
        *var_guard564_rv_slot = var_guard564_rv;
        *var_guard565_slot = var_guard565;
        *var_guard565_db0_slot = var_guard565_db0;
        *var_guard565_db1_slot = var_guard565_db1;
        *var_guard565_db2_slot = var_guard565_db2;
        *var_guard565_db3_slot = var_guard565_db3;
        *var_guard565_dn0_slot = var_guard565_dn0;
        *var_guard565_dn1_slot = var_guard565_dn1;
        *var_guard565_dn2_slot = var_guard565_dn2;
        *var_guard565_dn3_slot = var_guard565_dn3;
        *var_guard565_dn4_slot = var_guard565_dn4;
        *var_guard565_dn5_slot = var_guard565_dn5;
        *var_guard565_rdb0_slot = var_guard565_rdb0;
        *var_guard565_rdb1_slot = var_guard565_rdb1;
        *var_guard565_rdb2_slot = var_guard565_rdb2;
        *var_guard565_rdb3_slot = var_guard565_rdb3;
        *var_guard565_rdn0_slot = var_guard565_rdn0;
        *var_guard565_rdn1_slot = var_guard565_rdn1;
        *var_guard565_rdn2_slot = var_guard565_rdn2;
        *var_guard565_rdn3_slot = var_guard565_rdn3;
        *var_guard565_rdn4_slot = var_guard565_rdn4;
        *var_guard565_rdn5_slot = var_guard565_rdn5;
        *var_guard565_rv_slot = var_guard565_rv;
        *var_inqs0_a_slot = var_inqs0_a;
        *var_inqs0_a_db0_slot = var_inqs0_a_db0;
        *var_inqs0_a_db1_slot = var_inqs0_a_db1;
        *var_inqs0_a_db2_slot = var_inqs0_a_db2;
        *var_inqs0_a_db3_slot = var_inqs0_a_db3;
        *var_inqs0_a_dn0_slot = var_inqs0_a_dn0;
        *var_inqs0_a_dn1_slot = var_inqs0_a_dn1;
        *var_inqs0_a_dn2_slot = var_inqs0_a_dn2;
        *var_inqs0_a_dn3_slot = var_inqs0_a_dn3;
        *var_inqs0_a_dn4_slot = var_inqs0_a_dn4;
        *var_inqs0_a_dn5_slot = var_inqs0_a_dn5;
        *var_inqs0_a_rdb0_slot = var_inqs0_a_rdb0;
        *var_inqs0_a_rdb1_slot = var_inqs0_a_rdb1;
        *var_inqs0_a_rdb2_slot = var_inqs0_a_rdb2;
        *var_inqs0_a_rdb3_slot = var_inqs0_a_rdb3;
        *var_inqs0_a_rdn0_slot = var_inqs0_a_rdn0;
        *var_inqs0_a_rdn1_slot = var_inqs0_a_rdn1;
        *var_inqs0_a_rdn2_slot = var_inqs0_a_rdn2;
        *var_inqs0_a_rdn3_slot = var_inqs0_a_rdn3;
        *var_inqs0_a_rdn4_slot = var_inqs0_a_rdn4;
        *var_inqs0_a_rdn5_slot = var_inqs0_a_rdn5;
        *var_inqs0_a_rv_slot = var_inqs0_a_rv;
        *var_p_na_slot = var_p_na;
        *var_p_na_db0_slot = var_p_na_db0;
        *var_p_na_db1_slot = var_p_na_db1;
        *var_p_na_db2_slot = var_p_na_db2;
        *var_p_na_db3_slot = var_p_na_db3;
        *var_p_na_dn0_slot = var_p_na_dn0;
        *var_p_na_dn1_slot = var_p_na_dn1;
        *var_p_na_dn2_slot = var_p_na_dn2;
        *var_p_na_dn3_slot = var_p_na_dn3;
        *var_p_na_dn4_slot = var_p_na_dn4;
        *var_p_na_dn5_slot = var_p_na_dn5;
        *var_p_na_rdb0_slot = var_p_na_rdb0;
        *var_p_na_rdb1_slot = var_p_na_rdb1;
        *var_p_na_rdb2_slot = var_p_na_rdb2;
        *var_p_na_rdb3_slot = var_p_na_rdb3;
        *var_p_na_rdn0_slot = var_p_na_rdn0;
        *var_p_na_rdn1_slot = var_p_na_rdn1;
        *var_p_na_rdn2_slot = var_p_na_rdn2;
        *var_p_na_rdn3_slot = var_p_na_rdn3;
        *var_p_na_rdn4_slot = var_p_na_rdn4;
        *var_p_na_rdn5_slot = var_p_na_rdn5;
        *var_p_na_rv_slot = var_p_na_rv;
        *var_q_nqs_a_slot = var_q_nqs_a;
        *var_q_nqs_a_db0_slot = var_q_nqs_a_db0;
        *var_q_nqs_a_db1_slot = var_q_nqs_a_db1;
        *var_q_nqs_a_db2_slot = var_q_nqs_a_db2;
        *var_q_nqs_a_db3_slot = var_q_nqs_a_db3;
        *var_q_nqs_a_dn0_slot = var_q_nqs_a_dn0;
        *var_q_nqs_a_dn1_slot = var_q_nqs_a_dn1;
        *var_q_nqs_a_dn2_slot = var_q_nqs_a_dn2;
        *var_q_nqs_a_dn3_slot = var_q_nqs_a_dn3;
        *var_q_nqs_a_dn4_slot = var_q_nqs_a_dn4;
        *var_q_nqs_a_dn5_slot = var_q_nqs_a_dn5;
        *var_q_nqs_a_rdb0_slot = var_q_nqs_a_rdb0;
        *var_q_nqs_a_rdb1_slot = var_q_nqs_a_rdb1;
        *var_q_nqs_a_rdb2_slot = var_q_nqs_a_rdb2;
        *var_q_nqs_a_rdb3_slot = var_q_nqs_a_rdb3;
        *var_q_nqs_a_rdn0_slot = var_q_nqs_a_rdn0;
        *var_q_nqs_a_rdn1_slot = var_q_nqs_a_rdn1;
        *var_q_nqs_a_rdn2_slot = var_q_nqs_a_rdn2;
        *var_q_nqs_a_rdn3_slot = var_q_nqs_a_rdn3;
        *var_q_nqs_a_rdn4_slot = var_q_nqs_a_rdn4;
        *var_q_nqs_a_rdn5_slot = var_q_nqs_a_rdn5;
        *var_q_nqs_a_rv_slot = var_q_nqs_a_rv;
        *var_q_pexa_slot = var_q_pexa;
        *var_q_pexa_db0_slot = var_q_pexa_db0;
        *var_q_pexa_db1_slot = var_q_pexa_db1;
        *var_q_pexa_db2_slot = var_q_pexa_db2;
        *var_q_pexa_db3_slot = var_q_pexa_db3;
        *var_q_pexa_dn0_slot = var_q_pexa_dn0;
        *var_q_pexa_dn1_slot = var_q_pexa_dn1;
        *var_q_pexa_dn2_slot = var_q_pexa_dn2;
        *var_q_pexa_dn3_slot = var_q_pexa_dn3;
        *var_q_pexa_dn4_slot = var_q_pexa_dn4;
        *var_q_pexa_dn5_slot = var_q_pexa_dn5;
        *var_q_pexa_rdb0_slot = var_q_pexa_rdb0;
        *var_q_pexa_rdb1_slot = var_q_pexa_rdb1;
        *var_q_pexa_rdb2_slot = var_q_pexa_rdb2;
        *var_q_pexa_rdb3_slot = var_q_pexa_rdb3;
        *var_q_pexa_rdn0_slot = var_q_pexa_rdn0;
        *var_q_pexa_rdn1_slot = var_q_pexa_rdn1;
        *var_q_pexa_rdn2_slot = var_q_pexa_rdn2;
        *var_q_pexa_rdn3_slot = var_q_pexa_rdn3;
        *var_q_pexa_rdn4_slot = var_q_pexa_rdn4;
        *var_q_pexa_rdn5_slot = var_q_pexa_rdn5;
        *var_q_pexa_rv_slot = var_q_pexa_rv;
        *var_q_qs_a_slot = var_q_qs_a;
        *var_q_qs_a_db0_slot = var_q_qs_a_db0;
        *var_q_qs_a_db1_slot = var_q_qs_a_db1;
        *var_q_qs_a_db2_slot = var_q_qs_a_db2;
        *var_q_qs_a_db3_slot = var_q_qs_a_db3;
        *var_q_qs_a_dn0_slot = var_q_qs_a_dn0;
        *var_q_qs_a_dn1_slot = var_q_qs_a_dn1;
        *var_q_qs_a_dn2_slot = var_q_qs_a_dn2;
        *var_q_qs_a_dn3_slot = var_q_qs_a_dn3;
        *var_q_qs_a_dn4_slot = var_q_qs_a_dn4;
        *var_q_qs_a_dn5_slot = var_q_qs_a_dn5;
        *var_q_qs_a_rdb0_slot = var_q_qs_a_rdb0;
        *var_q_qs_a_rdb1_slot = var_q_qs_a_rdb1;
        *var_q_qs_a_rdb2_slot = var_q_qs_a_rdb2;
        *var_q_qs_a_rdb3_slot = var_q_qs_a_rdb3;
        *var_q_qs_a_rdn0_slot = var_q_qs_a_rdn0;
        *var_q_qs_a_rdn1_slot = var_q_qs_a_rdn1;
        *var_q_qs_a_rdn2_slot = var_q_qs_a_rdn2;
        *var_q_qs_a_rdn3_slot = var_q_qs_a_rdn3;
        *var_q_qs_a_rdn4_slot = var_q_qs_a_rdn4;
        *var_q_qs_a_rdn5_slot = var_q_qs_a_rdn5;
        *var_q_qs_a_rv_slot = var_q_qs_a_rv;
    }

    pub(super) fn stamp_reactive_block_87(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_ab_i: f64,
        var_ab_i_db0: f64,
        var_ab_i_db1: f64,
        var_ab_i_db2: f64,
        var_ab_i_db3: f64,
        var_ab_i_dn0: f64,
        var_ab_i_dn1: f64,
        var_ab_i_dn2: f64,
        var_ab_i_dn3: f64,
        var_ab_i_dn4: f64,
        var_ab_i_dn5: f64,
        var_epssi: f64,
        var_epssi_db0: f64,
        var_epssi_db1: f64,
        var_epssi_db2: f64,
        var_epssi_db3: f64,
        var_epssi_dn0: f64,
        var_epssi_dn1: f64,
        var_epssi_dn2: f64,
        var_epssi_dn3: f64,
        var_epssi_dn4: f64,
        var_epssi_dn5: f64,
        var_exp_k: f64,
        var_exp_k_db0: f64,
        var_exp_k_db1: f64,
        var_exp_k_db2: f64,
        var_exp_k_db3: f64,
        var_exp_k_dn0: f64,
        var_exp_k_dn1: f64,
        var_exp_k_dn2: f64,
        var_exp_k_dn3: f64,
        var_exp_k_dn4: f64,
        var_exp_k_dn5: f64,
        var_guard558: f64,
        var_guard565: f64,
        var_juncdlt: f64,
        var_juncdlt_db0: f64,
        var_juncdlt_db1: f64,
        var_juncdlt_db2: f64,
        var_juncdlt_db3: f64,
        var_juncdlt_dn0: f64,
        var_juncdlt_dn1: f64,
        var_juncdlt_dn2: f64,
        var_juncdlt_dn3: f64,
        var_juncdlt_dn4: f64,
        var_juncdlt_dn5: f64,
        var_ndi_i: f64,
        var_ndi_i_db0: f64,
        var_ndi_i_db1: f64,
        var_ndi_i_db2: f64,
        var_ndi_i_db3: f64,
        var_ndi_i_dn0: f64,
        var_ndi_i_dn1: f64,
        var_ndi_i_dn2: f64,
        var_ndi_i_dn3: f64,
        var_ndi_i_dn4: f64,
        var_ndi_i_dn5: f64,
        var_pb: f64,
        var_pb_db0: f64,
        var_pb_db1: f64,
        var_pb_db2: f64,
        var_pb_db3: f64,
        var_pb_dn0: f64,
        var_pb_dn1: f64,
        var_pb_dn2: f64,
        var_pb_dn3: f64,
        var_pb_dn4: f64,
        var_pb_dn5: f64,
        var_pn0: f64,
        var_pn0_db0: f64,
        var_pn0_db1: f64,
        var_pn0_db2: f64,
        var_pn0_db3: f64,
        var_pn0_dn0: f64,
        var_pn0_dn1: f64,
        var_pn0_dn2: f64,
        var_pn0_dn3: f64,
        var_pn0_dn4: f64,
        var_pn0_dn5: f64,
        var_q_pex0: f64,
        var_q_pex0_db0: f64,
        var_q_pex0_db1: f64,
        var_q_pex0_db2: f64,
        var_q_pex0_db3: f64,
        var_q_pex0_dn0: f64,
        var_q_pex0_dn1: f64,
        var_q_pex0_dn2: f64,
        var_q_pex0_dn3: f64,
        var_q_pex0_dn4: f64,
        var_q_pex0_dn5: f64,
        var_tkd: f64,
        var_tkd_db0: f64,
        var_tkd_db1: f64,
        var_tkd_db2: f64,
        var_tkd_db3: f64,
        var_tkd_dn0: f64,
        var_tkd_dn1: f64,
        var_tkd_dn2: f64,
        var_tkd_dn3: f64,
        var_tkd_dn4: f64,
        var_tkd_dn5: f64,
        var_tkr: f64,
        var_tkr_db0: f64,
        var_tkr_db1: f64,
        var_tkr_db2: f64,
        var_tkr_db3: f64,
        var_tkr_dn0: f64,
        var_tkr_dn1: f64,
        var_tkr_dn2: f64,
        var_tkr_dn3: f64,
        var_tkr_dn4: f64,
        var_tkr_dn5: f64,
        var_v_hk: f64,
        var_v_hk_db0: f64,
        var_v_hk_db1: f64,
        var_v_hk_db2: f64,
        var_v_hk_db3: f64,
        var_v_hk_dn0: f64,
        var_v_hk_dn1: f64,
        var_v_hk_dn2: f64,
        var_v_hk_dn3: f64,
        var_v_hk_dn4: f64,
        var_v_hk_dn5: f64,
        var_vak: f64,
        var_vak_db0: f64,
        var_vak_db1: f64,
        var_vak_db2: f64,
        var_vak_db3: f64,
        var_vak_dn0: f64,
        var_vak_dn1: f64,
        var_vak_dn2: f64,
        var_vak_dn3: f64,
        var_vak_dn4: f64,
        var_vak_dn5: f64,
        var_exp_k2_slot: &mut f64,
        var_exp_k2_db0_slot: &mut f64,
        var_exp_k2_db1_slot: &mut f64,
        var_exp_k2_db2_slot: &mut f64,
        var_exp_k2_db3_slot: &mut f64,
        var_exp_k2_dn0_slot: &mut f64,
        var_exp_k2_dn1_slot: &mut f64,
        var_exp_k2_dn2_slot: &mut f64,
        var_exp_k2_dn3_slot: &mut f64,
        var_exp_k2_dn4_slot: &mut f64,
        var_exp_k2_dn5_slot: &mut f64,
        var_exp_k2_rdb0_slot: &mut f64,
        var_exp_k2_rdb1_slot: &mut f64,
        var_exp_k2_rdb2_slot: &mut f64,
        var_exp_k2_rdb3_slot: &mut f64,
        var_exp_k2_rdn0_slot: &mut f64,
        var_exp_k2_rdn1_slot: &mut f64,
        var_exp_k2_rdn2_slot: &mut f64,
        var_exp_k2_rdn3_slot: &mut f64,
        var_exp_k2_rdn4_slot: &mut f64,
        var_exp_k2_rdn5_slot: &mut f64,
        var_exp_k2_rv_slot: &mut f64,
        var_guard566_slot: &mut f64,
        var_guard566_db0_slot: &mut f64,
        var_guard566_db1_slot: &mut f64,
        var_guard566_db2_slot: &mut f64,
        var_guard566_db3_slot: &mut f64,
        var_guard566_dn0_slot: &mut f64,
        var_guard566_dn1_slot: &mut f64,
        var_guard566_dn2_slot: &mut f64,
        var_guard566_dn3_slot: &mut f64,
        var_guard566_dn4_slot: &mut f64,
        var_guard566_dn5_slot: &mut f64,
        var_guard566_rdb0_slot: &mut f64,
        var_guard566_rdb1_slot: &mut f64,
        var_guard566_rdb2_slot: &mut f64,
        var_guard566_rdb3_slot: &mut f64,
        var_guard566_rdn0_slot: &mut f64,
        var_guard566_rdn1_slot: &mut f64,
        var_guard566_rdn2_slot: &mut f64,
        var_guard566_rdn3_slot: &mut f64,
        var_guard566_rdn4_slot: &mut f64,
        var_guard566_rdn5_slot: &mut f64,
        var_guard566_rv_slot: &mut f64,
        var_guard567_slot: &mut f64,
        var_guard567_db0_slot: &mut f64,
        var_guard567_db1_slot: &mut f64,
        var_guard567_db2_slot: &mut f64,
        var_guard567_db3_slot: &mut f64,
        var_guard567_dn0_slot: &mut f64,
        var_guard567_dn1_slot: &mut f64,
        var_guard567_dn2_slot: &mut f64,
        var_guard567_dn3_slot: &mut f64,
        var_guard567_dn4_slot: &mut f64,
        var_guard567_dn5_slot: &mut f64,
        var_guard567_rdb0_slot: &mut f64,
        var_guard567_rdb1_slot: &mut f64,
        var_guard567_rdb2_slot: &mut f64,
        var_guard567_rdb3_slot: &mut f64,
        var_guard567_rdn0_slot: &mut f64,
        var_guard567_rdn1_slot: &mut f64,
        var_guard567_rdn2_slot: &mut f64,
        var_guard567_rdn3_slot: &mut f64,
        var_guard567_rdn4_slot: &mut f64,
        var_guard567_rdn5_slot: &mut f64,
        var_guard567_rv_slot: &mut f64,
        var_inqs0_k_slot: &mut f64,
        var_inqs0_k_db0_slot: &mut f64,
        var_inqs0_k_db1_slot: &mut f64,
        var_inqs0_k_db2_slot: &mut f64,
        var_inqs0_k_db3_slot: &mut f64,
        var_inqs0_k_dn0_slot: &mut f64,
        var_inqs0_k_dn1_slot: &mut f64,
        var_inqs0_k_dn2_slot: &mut f64,
        var_inqs0_k_dn3_slot: &mut f64,
        var_inqs0_k_dn4_slot: &mut f64,
        var_inqs0_k_dn5_slot: &mut f64,
        var_inqs0_k_rdb0_slot: &mut f64,
        var_inqs0_k_rdb1_slot: &mut f64,
        var_inqs0_k_rdb2_slot: &mut f64,
        var_inqs0_k_rdb3_slot: &mut f64,
        var_inqs0_k_rdn0_slot: &mut f64,
        var_inqs0_k_rdn1_slot: &mut f64,
        var_inqs0_k_rdn2_slot: &mut f64,
        var_inqs0_k_rdn3_slot: &mut f64,
        var_inqs0_k_rdn4_slot: &mut f64,
        var_inqs0_k_rdn5_slot: &mut f64,
        var_inqs0_k_rv_slot: &mut f64,
        var_p_nk_slot: &mut f64,
        var_p_nk_db0_slot: &mut f64,
        var_p_nk_db1_slot: &mut f64,
        var_p_nk_db2_slot: &mut f64,
        var_p_nk_db3_slot: &mut f64,
        var_p_nk_dn0_slot: &mut f64,
        var_p_nk_dn1_slot: &mut f64,
        var_p_nk_dn2_slot: &mut f64,
        var_p_nk_dn3_slot: &mut f64,
        var_p_nk_dn4_slot: &mut f64,
        var_p_nk_dn5_slot: &mut f64,
        var_p_nk_rdb0_slot: &mut f64,
        var_p_nk_rdb1_slot: &mut f64,
        var_p_nk_rdb2_slot: &mut f64,
        var_p_nk_rdb3_slot: &mut f64,
        var_p_nk_rdn0_slot: &mut f64,
        var_p_nk_rdn1_slot: &mut f64,
        var_p_nk_rdn2_slot: &mut f64,
        var_p_nk_rdn3_slot: &mut f64,
        var_p_nk_rdn4_slot: &mut f64,
        var_p_nk_rdn5_slot: &mut f64,
        var_p_nk_rv_slot: &mut f64,
        var_q_nqs_k_slot: &mut f64,
        var_q_nqs_k_db0_slot: &mut f64,
        var_q_nqs_k_db1_slot: &mut f64,
        var_q_nqs_k_db2_slot: &mut f64,
        var_q_nqs_k_db3_slot: &mut f64,
        var_q_nqs_k_dn0_slot: &mut f64,
        var_q_nqs_k_dn1_slot: &mut f64,
        var_q_nqs_k_dn2_slot: &mut f64,
        var_q_nqs_k_dn3_slot: &mut f64,
        var_q_nqs_k_dn4_slot: &mut f64,
        var_q_nqs_k_dn5_slot: &mut f64,
        var_q_nqs_k_rdb0_slot: &mut f64,
        var_q_nqs_k_rdb1_slot: &mut f64,
        var_q_nqs_k_rdb2_slot: &mut f64,
        var_q_nqs_k_rdb3_slot: &mut f64,
        var_q_nqs_k_rdn0_slot: &mut f64,
        var_q_nqs_k_rdn1_slot: &mut f64,
        var_q_nqs_k_rdn2_slot: &mut f64,
        var_q_nqs_k_rdn3_slot: &mut f64,
        var_q_nqs_k_rdn4_slot: &mut f64,
        var_q_nqs_k_rdn5_slot: &mut f64,
        var_q_nqs_k_rv_slot: &mut f64,
        var_q_pexk_slot: &mut f64,
        var_q_pexk_db0_slot: &mut f64,
        var_q_pexk_db1_slot: &mut f64,
        var_q_pexk_db2_slot: &mut f64,
        var_q_pexk_db3_slot: &mut f64,
        var_q_pexk_dn0_slot: &mut f64,
        var_q_pexk_dn1_slot: &mut f64,
        var_q_pexk_dn2_slot: &mut f64,
        var_q_pexk_dn3_slot: &mut f64,
        var_q_pexk_dn4_slot: &mut f64,
        var_q_pexk_dn5_slot: &mut f64,
        var_q_pexk_rdb0_slot: &mut f64,
        var_q_pexk_rdb1_slot: &mut f64,
        var_q_pexk_rdb2_slot: &mut f64,
        var_q_pexk_rdb3_slot: &mut f64,
        var_q_pexk_rdn0_slot: &mut f64,
        var_q_pexk_rdn1_slot: &mut f64,
        var_q_pexk_rdn2_slot: &mut f64,
        var_q_pexk_rdn3_slot: &mut f64,
        var_q_pexk_rdn4_slot: &mut f64,
        var_q_pexk_rdn5_slot: &mut f64,
        var_q_pexk_rv_slot: &mut f64,
        var_q_qs_k_slot: &mut f64,
        var_q_qs_k_db0_slot: &mut f64,
        var_q_qs_k_db1_slot: &mut f64,
        var_q_qs_k_db2_slot: &mut f64,
        var_q_qs_k_db3_slot: &mut f64,
        var_q_qs_k_dn0_slot: &mut f64,
        var_q_qs_k_dn1_slot: &mut f64,
        var_q_qs_k_dn2_slot: &mut f64,
        var_q_qs_k_dn3_slot: &mut f64,
        var_q_qs_k_dn4_slot: &mut f64,
        var_q_qs_k_dn5_slot: &mut f64,
        var_q_qs_k_rdb0_slot: &mut f64,
        var_q_qs_k_rdb1_slot: &mut f64,
        var_q_qs_k_rdb2_slot: &mut f64,
        var_q_qs_k_rdb3_slot: &mut f64,
        var_q_qs_k_rdn0_slot: &mut f64,
        var_q_qs_k_rdn1_slot: &mut f64,
        var_q_qs_k_rdn2_slot: &mut f64,
        var_q_qs_k_rdn3_slot: &mut f64,
        var_q_qs_k_rdn4_slot: &mut f64,
        var_q_qs_k_rdn5_slot: &mut f64,
        var_q_qs_k_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_rdb0_slot: &mut f64,
        var_tmf1_rdb1_slot: &mut f64,
        var_tmf1_rdb2_slot: &mut f64,
        var_tmf1_rdb3_slot: &mut f64,
        var_tmf1_rdn0_slot: &mut f64,
        var_tmf1_rdn1_slot: &mut f64,
        var_tmf1_rdn2_slot: &mut f64,
        var_tmf1_rdn3_slot: &mut f64,
        var_tmf1_rdn4_slot: &mut f64,
        var_tmf1_rdn5_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_rdb0_slot: &mut f64,
        var_tmf2_rdb1_slot: &mut f64,
        var_tmf2_rdb2_slot: &mut f64,
        var_tmf2_rdb3_slot: &mut f64,
        var_tmf2_rdn0_slot: &mut f64,
        var_tmf2_rdn1_slot: &mut f64,
        var_tmf2_rdn2_slot: &mut f64,
        var_tmf2_rdn3_slot: &mut f64,
        var_tmf2_rdn4_slot: &mut f64,
        var_tmf2_rdn5_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vjunc_a_slot: &mut f64,
        var_vjunc_a_db0_slot: &mut f64,
        var_vjunc_a_db1_slot: &mut f64,
        var_vjunc_a_db2_slot: &mut f64,
        var_vjunc_a_db3_slot: &mut f64,
        var_vjunc_a_dn0_slot: &mut f64,
        var_vjunc_a_dn1_slot: &mut f64,
        var_vjunc_a_dn2_slot: &mut f64,
        var_vjunc_a_dn3_slot: &mut f64,
        var_vjunc_a_dn4_slot: &mut f64,
        var_vjunc_a_dn5_slot: &mut f64,
        var_vjunc_a_rdb0_slot: &mut f64,
        var_vjunc_a_rdb1_slot: &mut f64,
        var_vjunc_a_rdb2_slot: &mut f64,
        var_vjunc_a_rdb3_slot: &mut f64,
        var_vjunc_a_rdn0_slot: &mut f64,
        var_vjunc_a_rdn1_slot: &mut f64,
        var_vjunc_a_rdn2_slot: &mut f64,
        var_vjunc_a_rdn3_slot: &mut f64,
        var_vjunc_a_rdn4_slot: &mut f64,
        var_vjunc_a_rdn5_slot: &mut f64,
        var_vjunc_a_rv_slot: &mut f64,
        var_w_depa_slot: &mut f64,
        var_w_depa_db0_slot: &mut f64,
        var_w_depa_db1_slot: &mut f64,
        var_w_depa_db2_slot: &mut f64,
        var_w_depa_db3_slot: &mut f64,
        var_w_depa_dn0_slot: &mut f64,
        var_w_depa_dn1_slot: &mut f64,
        var_w_depa_dn2_slot: &mut f64,
        var_w_depa_dn3_slot: &mut f64,
        var_w_depa_dn4_slot: &mut f64,
        var_w_depa_dn5_slot: &mut f64,
        var_w_depa_rdb0_slot: &mut f64,
        var_w_depa_rdb1_slot: &mut f64,
        var_w_depa_rdb2_slot: &mut f64,
        var_w_depa_rdb3_slot: &mut f64,
        var_w_depa_rdn0_slot: &mut f64,
        var_w_depa_rdn1_slot: &mut f64,
        var_w_depa_rdn2_slot: &mut f64,
        var_w_depa_rdn3_slot: &mut f64,
        var_w_depa_rdn4_slot: &mut f64,
        var_w_depa_rdn5_slot: &mut f64,
        var_w_depa_rv_slot: &mut f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let mut var_exp_k2: f64 = *var_exp_k2_slot;
        let mut var_exp_k2_db0: f64 = *var_exp_k2_db0_slot;
        let mut var_exp_k2_db1: f64 = *var_exp_k2_db1_slot;
        let mut var_exp_k2_db2: f64 = *var_exp_k2_db2_slot;
        let mut var_exp_k2_db3: f64 = *var_exp_k2_db3_slot;
        let mut var_exp_k2_dn0: f64 = *var_exp_k2_dn0_slot;
        let mut var_exp_k2_dn1: f64 = *var_exp_k2_dn1_slot;
        let mut var_exp_k2_dn2: f64 = *var_exp_k2_dn2_slot;
        let mut var_exp_k2_dn3: f64 = *var_exp_k2_dn3_slot;
        let mut var_exp_k2_dn4: f64 = *var_exp_k2_dn4_slot;
        let mut var_exp_k2_dn5: f64 = *var_exp_k2_dn5_slot;
        let mut var_exp_k2_rdb0: f64 = *var_exp_k2_rdb0_slot;
        let mut var_exp_k2_rdb1: f64 = *var_exp_k2_rdb1_slot;
        let mut var_exp_k2_rdb2: f64 = *var_exp_k2_rdb2_slot;
        let mut var_exp_k2_rdb3: f64 = *var_exp_k2_rdb3_slot;
        let mut var_exp_k2_rdn0: f64 = *var_exp_k2_rdn0_slot;
        let mut var_exp_k2_rdn1: f64 = *var_exp_k2_rdn1_slot;
        let mut var_exp_k2_rdn2: f64 = *var_exp_k2_rdn2_slot;
        let mut var_exp_k2_rdn3: f64 = *var_exp_k2_rdn3_slot;
        let mut var_exp_k2_rdn4: f64 = *var_exp_k2_rdn4_slot;
        let mut var_exp_k2_rdn5: f64 = *var_exp_k2_rdn5_slot;
        let mut var_exp_k2_rv: f64 = *var_exp_k2_rv_slot;
        let mut var_guard566: f64 = *var_guard566_slot;
        let mut var_guard566_db0: f64 = *var_guard566_db0_slot;
        let mut var_guard566_db1: f64 = *var_guard566_db1_slot;
        let mut var_guard566_db2: f64 = *var_guard566_db2_slot;
        let mut var_guard566_db3: f64 = *var_guard566_db3_slot;
        let mut var_guard566_dn0: f64 = *var_guard566_dn0_slot;
        let mut var_guard566_dn1: f64 = *var_guard566_dn1_slot;
        let mut var_guard566_dn2: f64 = *var_guard566_dn2_slot;
        let mut var_guard566_dn3: f64 = *var_guard566_dn3_slot;
        let mut var_guard566_dn4: f64 = *var_guard566_dn4_slot;
        let mut var_guard566_dn5: f64 = *var_guard566_dn5_slot;
        let mut var_guard566_rdb0: f64 = *var_guard566_rdb0_slot;
        let mut var_guard566_rdb1: f64 = *var_guard566_rdb1_slot;
        let mut var_guard566_rdb2: f64 = *var_guard566_rdb2_slot;
        let mut var_guard566_rdb3: f64 = *var_guard566_rdb3_slot;
        let mut var_guard566_rdn0: f64 = *var_guard566_rdn0_slot;
        let mut var_guard566_rdn1: f64 = *var_guard566_rdn1_slot;
        let mut var_guard566_rdn2: f64 = *var_guard566_rdn2_slot;
        let mut var_guard566_rdn3: f64 = *var_guard566_rdn3_slot;
        let mut var_guard566_rdn4: f64 = *var_guard566_rdn4_slot;
        let mut var_guard566_rdn5: f64 = *var_guard566_rdn5_slot;
        let mut var_guard566_rv: f64 = *var_guard566_rv_slot;
        let mut var_guard567: f64 = *var_guard567_slot;
        let mut var_guard567_db0: f64 = *var_guard567_db0_slot;
        let mut var_guard567_db1: f64 = *var_guard567_db1_slot;
        let mut var_guard567_db2: f64 = *var_guard567_db2_slot;
        let mut var_guard567_db3: f64 = *var_guard567_db3_slot;
        let mut var_guard567_dn0: f64 = *var_guard567_dn0_slot;
        let mut var_guard567_dn1: f64 = *var_guard567_dn1_slot;
        let mut var_guard567_dn2: f64 = *var_guard567_dn2_slot;
        let mut var_guard567_dn3: f64 = *var_guard567_dn3_slot;
        let mut var_guard567_dn4: f64 = *var_guard567_dn4_slot;
        let mut var_guard567_dn5: f64 = *var_guard567_dn5_slot;
        let mut var_guard567_rdb0: f64 = *var_guard567_rdb0_slot;
        let mut var_guard567_rdb1: f64 = *var_guard567_rdb1_slot;
        let mut var_guard567_rdb2: f64 = *var_guard567_rdb2_slot;
        let mut var_guard567_rdb3: f64 = *var_guard567_rdb3_slot;
        let mut var_guard567_rdn0: f64 = *var_guard567_rdn0_slot;
        let mut var_guard567_rdn1: f64 = *var_guard567_rdn1_slot;
        let mut var_guard567_rdn2: f64 = *var_guard567_rdn2_slot;
        let mut var_guard567_rdn3: f64 = *var_guard567_rdn3_slot;
        let mut var_guard567_rdn4: f64 = *var_guard567_rdn4_slot;
        let mut var_guard567_rdn5: f64 = *var_guard567_rdn5_slot;
        let mut var_guard567_rv: f64 = *var_guard567_rv_slot;
        let mut var_inqs0_k: f64 = *var_inqs0_k_slot;
        let mut var_inqs0_k_db0: f64 = *var_inqs0_k_db0_slot;
        let mut var_inqs0_k_db1: f64 = *var_inqs0_k_db1_slot;
        let mut var_inqs0_k_db2: f64 = *var_inqs0_k_db2_slot;
        let mut var_inqs0_k_db3: f64 = *var_inqs0_k_db3_slot;
        let mut var_inqs0_k_dn0: f64 = *var_inqs0_k_dn0_slot;
        let mut var_inqs0_k_dn1: f64 = *var_inqs0_k_dn1_slot;
        let mut var_inqs0_k_dn2: f64 = *var_inqs0_k_dn2_slot;
        let mut var_inqs0_k_dn3: f64 = *var_inqs0_k_dn3_slot;
        let mut var_inqs0_k_dn4: f64 = *var_inqs0_k_dn4_slot;
        let mut var_inqs0_k_dn5: f64 = *var_inqs0_k_dn5_slot;
        let mut var_inqs0_k_rdb0: f64 = *var_inqs0_k_rdb0_slot;
        let mut var_inqs0_k_rdb1: f64 = *var_inqs0_k_rdb1_slot;
        let mut var_inqs0_k_rdb2: f64 = *var_inqs0_k_rdb2_slot;
        let mut var_inqs0_k_rdb3: f64 = *var_inqs0_k_rdb3_slot;
        let mut var_inqs0_k_rdn0: f64 = *var_inqs0_k_rdn0_slot;
        let mut var_inqs0_k_rdn1: f64 = *var_inqs0_k_rdn1_slot;
        let mut var_inqs0_k_rdn2: f64 = *var_inqs0_k_rdn2_slot;
        let mut var_inqs0_k_rdn3: f64 = *var_inqs0_k_rdn3_slot;
        let mut var_inqs0_k_rdn4: f64 = *var_inqs0_k_rdn4_slot;
        let mut var_inqs0_k_rdn5: f64 = *var_inqs0_k_rdn5_slot;
        let mut var_inqs0_k_rv: f64 = *var_inqs0_k_rv_slot;
        let mut var_p_nk: f64 = *var_p_nk_slot;
        let mut var_p_nk_db0: f64 = *var_p_nk_db0_slot;
        let mut var_p_nk_db1: f64 = *var_p_nk_db1_slot;
        let mut var_p_nk_db2: f64 = *var_p_nk_db2_slot;
        let mut var_p_nk_db3: f64 = *var_p_nk_db3_slot;
        let mut var_p_nk_dn0: f64 = *var_p_nk_dn0_slot;
        let mut var_p_nk_dn1: f64 = *var_p_nk_dn1_slot;
        let mut var_p_nk_dn2: f64 = *var_p_nk_dn2_slot;
        let mut var_p_nk_dn3: f64 = *var_p_nk_dn3_slot;
        let mut var_p_nk_dn4: f64 = *var_p_nk_dn4_slot;
        let mut var_p_nk_dn5: f64 = *var_p_nk_dn5_slot;
        let mut var_p_nk_rdb0: f64 = *var_p_nk_rdb0_slot;
        let mut var_p_nk_rdb1: f64 = *var_p_nk_rdb1_slot;
        let mut var_p_nk_rdb2: f64 = *var_p_nk_rdb2_slot;
        let mut var_p_nk_rdb3: f64 = *var_p_nk_rdb3_slot;
        let mut var_p_nk_rdn0: f64 = *var_p_nk_rdn0_slot;
        let mut var_p_nk_rdn1: f64 = *var_p_nk_rdn1_slot;
        let mut var_p_nk_rdn2: f64 = *var_p_nk_rdn2_slot;
        let mut var_p_nk_rdn3: f64 = *var_p_nk_rdn3_slot;
        let mut var_p_nk_rdn4: f64 = *var_p_nk_rdn4_slot;
        let mut var_p_nk_rdn5: f64 = *var_p_nk_rdn5_slot;
        let mut var_p_nk_rv: f64 = *var_p_nk_rv_slot;
        let mut var_q_nqs_k: f64 = *var_q_nqs_k_slot;
        let mut var_q_nqs_k_db0: f64 = *var_q_nqs_k_db0_slot;
        let mut var_q_nqs_k_db1: f64 = *var_q_nqs_k_db1_slot;
        let mut var_q_nqs_k_db2: f64 = *var_q_nqs_k_db2_slot;
        let mut var_q_nqs_k_db3: f64 = *var_q_nqs_k_db3_slot;
        let mut var_q_nqs_k_dn0: f64 = *var_q_nqs_k_dn0_slot;
        let mut var_q_nqs_k_dn1: f64 = *var_q_nqs_k_dn1_slot;
        let mut var_q_nqs_k_dn2: f64 = *var_q_nqs_k_dn2_slot;
        let mut var_q_nqs_k_dn3: f64 = *var_q_nqs_k_dn3_slot;
        let mut var_q_nqs_k_dn4: f64 = *var_q_nqs_k_dn4_slot;
        let mut var_q_nqs_k_dn5: f64 = *var_q_nqs_k_dn5_slot;
        let mut var_q_nqs_k_rdb0: f64 = *var_q_nqs_k_rdb0_slot;
        let mut var_q_nqs_k_rdb1: f64 = *var_q_nqs_k_rdb1_slot;
        let mut var_q_nqs_k_rdb2: f64 = *var_q_nqs_k_rdb2_slot;
        let mut var_q_nqs_k_rdb3: f64 = *var_q_nqs_k_rdb3_slot;
        let mut var_q_nqs_k_rdn0: f64 = *var_q_nqs_k_rdn0_slot;
        let mut var_q_nqs_k_rdn1: f64 = *var_q_nqs_k_rdn1_slot;
        let mut var_q_nqs_k_rdn2: f64 = *var_q_nqs_k_rdn2_slot;
        let mut var_q_nqs_k_rdn3: f64 = *var_q_nqs_k_rdn3_slot;
        let mut var_q_nqs_k_rdn4: f64 = *var_q_nqs_k_rdn4_slot;
        let mut var_q_nqs_k_rdn5: f64 = *var_q_nqs_k_rdn5_slot;
        let mut var_q_nqs_k_rv: f64 = *var_q_nqs_k_rv_slot;
        let mut var_q_pexk: f64 = *var_q_pexk_slot;
        let mut var_q_pexk_db0: f64 = *var_q_pexk_db0_slot;
        let mut var_q_pexk_db1: f64 = *var_q_pexk_db1_slot;
        let mut var_q_pexk_db2: f64 = *var_q_pexk_db2_slot;
        let mut var_q_pexk_db3: f64 = *var_q_pexk_db3_slot;
        let mut var_q_pexk_dn0: f64 = *var_q_pexk_dn0_slot;
        let mut var_q_pexk_dn1: f64 = *var_q_pexk_dn1_slot;
        let mut var_q_pexk_dn2: f64 = *var_q_pexk_dn2_slot;
        let mut var_q_pexk_dn3: f64 = *var_q_pexk_dn3_slot;
        let mut var_q_pexk_dn4: f64 = *var_q_pexk_dn4_slot;
        let mut var_q_pexk_dn5: f64 = *var_q_pexk_dn5_slot;
        let mut var_q_pexk_rdb0: f64 = *var_q_pexk_rdb0_slot;
        let mut var_q_pexk_rdb1: f64 = *var_q_pexk_rdb1_slot;
        let mut var_q_pexk_rdb2: f64 = *var_q_pexk_rdb2_slot;
        let mut var_q_pexk_rdb3: f64 = *var_q_pexk_rdb3_slot;
        let mut var_q_pexk_rdn0: f64 = *var_q_pexk_rdn0_slot;
        let mut var_q_pexk_rdn1: f64 = *var_q_pexk_rdn1_slot;
        let mut var_q_pexk_rdn2: f64 = *var_q_pexk_rdn2_slot;
        let mut var_q_pexk_rdn3: f64 = *var_q_pexk_rdn3_slot;
        let mut var_q_pexk_rdn4: f64 = *var_q_pexk_rdn4_slot;
        let mut var_q_pexk_rdn5: f64 = *var_q_pexk_rdn5_slot;
        let mut var_q_pexk_rv: f64 = *var_q_pexk_rv_slot;
        let mut var_q_qs_k: f64 = *var_q_qs_k_slot;
        let mut var_q_qs_k_db0: f64 = *var_q_qs_k_db0_slot;
        let mut var_q_qs_k_db1: f64 = *var_q_qs_k_db1_slot;
        let mut var_q_qs_k_db2: f64 = *var_q_qs_k_db2_slot;
        let mut var_q_qs_k_db3: f64 = *var_q_qs_k_db3_slot;
        let mut var_q_qs_k_dn0: f64 = *var_q_qs_k_dn0_slot;
        let mut var_q_qs_k_dn1: f64 = *var_q_qs_k_dn1_slot;
        let mut var_q_qs_k_dn2: f64 = *var_q_qs_k_dn2_slot;
        let mut var_q_qs_k_dn3: f64 = *var_q_qs_k_dn3_slot;
        let mut var_q_qs_k_dn4: f64 = *var_q_qs_k_dn4_slot;
        let mut var_q_qs_k_dn5: f64 = *var_q_qs_k_dn5_slot;
        let mut var_q_qs_k_rdb0: f64 = *var_q_qs_k_rdb0_slot;
        let mut var_q_qs_k_rdb1: f64 = *var_q_qs_k_rdb1_slot;
        let mut var_q_qs_k_rdb2: f64 = *var_q_qs_k_rdb2_slot;
        let mut var_q_qs_k_rdb3: f64 = *var_q_qs_k_rdb3_slot;
        let mut var_q_qs_k_rdn0: f64 = *var_q_qs_k_rdn0_slot;
        let mut var_q_qs_k_rdn1: f64 = *var_q_qs_k_rdn1_slot;
        let mut var_q_qs_k_rdn2: f64 = *var_q_qs_k_rdn2_slot;
        let mut var_q_qs_k_rdn3: f64 = *var_q_qs_k_rdn3_slot;
        let mut var_q_qs_k_rdn4: f64 = *var_q_qs_k_rdn4_slot;
        let mut var_q_qs_k_rdn5: f64 = *var_q_qs_k_rdn5_slot;
        let mut var_q_qs_k_rv: f64 = *var_q_qs_k_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_rdb0: f64 = *var_tmf1_rdb0_slot;
        let mut var_tmf1_rdb1: f64 = *var_tmf1_rdb1_slot;
        let mut var_tmf1_rdb2: f64 = *var_tmf1_rdb2_slot;
        let mut var_tmf1_rdb3: f64 = *var_tmf1_rdb3_slot;
        let mut var_tmf1_rdn0: f64 = *var_tmf1_rdn0_slot;
        let mut var_tmf1_rdn1: f64 = *var_tmf1_rdn1_slot;
        let mut var_tmf1_rdn2: f64 = *var_tmf1_rdn2_slot;
        let mut var_tmf1_rdn3: f64 = *var_tmf1_rdn3_slot;
        let mut var_tmf1_rdn4: f64 = *var_tmf1_rdn4_slot;
        let mut var_tmf1_rdn5: f64 = *var_tmf1_rdn5_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_rdb0: f64 = *var_tmf2_rdb0_slot;
        let mut var_tmf2_rdb1: f64 = *var_tmf2_rdb1_slot;
        let mut var_tmf2_rdb2: f64 = *var_tmf2_rdb2_slot;
        let mut var_tmf2_rdb3: f64 = *var_tmf2_rdb3_slot;
        let mut var_tmf2_rdn0: f64 = *var_tmf2_rdn0_slot;
        let mut var_tmf2_rdn1: f64 = *var_tmf2_rdn1_slot;
        let mut var_tmf2_rdn2: f64 = *var_tmf2_rdn2_slot;
        let mut var_tmf2_rdn3: f64 = *var_tmf2_rdn3_slot;
        let mut var_tmf2_rdn4: f64 = *var_tmf2_rdn4_slot;
        let mut var_tmf2_rdn5: f64 = *var_tmf2_rdn5_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vjunc_a: f64 = *var_vjunc_a_slot;
        let mut var_vjunc_a_db0: f64 = *var_vjunc_a_db0_slot;
        let mut var_vjunc_a_db1: f64 = *var_vjunc_a_db1_slot;
        let mut var_vjunc_a_db2: f64 = *var_vjunc_a_db2_slot;
        let mut var_vjunc_a_db3: f64 = *var_vjunc_a_db3_slot;
        let mut var_vjunc_a_dn0: f64 = *var_vjunc_a_dn0_slot;
        let mut var_vjunc_a_dn1: f64 = *var_vjunc_a_dn1_slot;
        let mut var_vjunc_a_dn2: f64 = *var_vjunc_a_dn2_slot;
        let mut var_vjunc_a_dn3: f64 = *var_vjunc_a_dn3_slot;
        let mut var_vjunc_a_dn4: f64 = *var_vjunc_a_dn4_slot;
        let mut var_vjunc_a_dn5: f64 = *var_vjunc_a_dn5_slot;
        let mut var_vjunc_a_rdb0: f64 = *var_vjunc_a_rdb0_slot;
        let mut var_vjunc_a_rdb1: f64 = *var_vjunc_a_rdb1_slot;
        let mut var_vjunc_a_rdb2: f64 = *var_vjunc_a_rdb2_slot;
        let mut var_vjunc_a_rdb3: f64 = *var_vjunc_a_rdb3_slot;
        let mut var_vjunc_a_rdn0: f64 = *var_vjunc_a_rdn0_slot;
        let mut var_vjunc_a_rdn1: f64 = *var_vjunc_a_rdn1_slot;
        let mut var_vjunc_a_rdn2: f64 = *var_vjunc_a_rdn2_slot;
        let mut var_vjunc_a_rdn3: f64 = *var_vjunc_a_rdn3_slot;
        let mut var_vjunc_a_rdn4: f64 = *var_vjunc_a_rdn4_slot;
        let mut var_vjunc_a_rdn5: f64 = *var_vjunc_a_rdn5_slot;
        let mut var_vjunc_a_rv: f64 = *var_vjunc_a_rv_slot;
        let mut var_w_depa: f64 = *var_w_depa_slot;
        let mut var_w_depa_db0: f64 = *var_w_depa_db0_slot;
        let mut var_w_depa_db1: f64 = *var_w_depa_db1_slot;
        let mut var_w_depa_db2: f64 = *var_w_depa_db2_slot;
        let mut var_w_depa_db3: f64 = *var_w_depa_db3_slot;
        let mut var_w_depa_dn0: f64 = *var_w_depa_dn0_slot;
        let mut var_w_depa_dn1: f64 = *var_w_depa_dn1_slot;
        let mut var_w_depa_dn2: f64 = *var_w_depa_dn2_slot;
        let mut var_w_depa_dn3: f64 = *var_w_depa_dn3_slot;
        let mut var_w_depa_dn4: f64 = *var_w_depa_dn4_slot;
        let mut var_w_depa_dn5: f64 = *var_w_depa_dn5_slot;
        let mut var_w_depa_rdb0: f64 = *var_w_depa_rdb0_slot;
        let mut var_w_depa_rdb1: f64 = *var_w_depa_rdb1_slot;
        let mut var_w_depa_rdb2: f64 = *var_w_depa_rdb2_slot;
        let mut var_w_depa_rdb3: f64 = *var_w_depa_rdb3_slot;
        let mut var_w_depa_rdn0: f64 = *var_w_depa_rdn0_slot;
        let mut var_w_depa_rdn1: f64 = *var_w_depa_rdn1_slot;
        let mut var_w_depa_rdn2: f64 = *var_w_depa_rdn2_slot;
        let mut var_w_depa_rdn3: f64 = *var_w_depa_rdn3_slot;
        let mut var_w_depa_rdn4: f64 = *var_w_depa_rdn4_slot;
        let mut var_w_depa_rdn5: f64 = *var_w_depa_rdn5_slot;
        let mut var_w_depa_rv: f64 = *var_w_depa_rv_slot;

        let (assign34660_e52177, assign34660_e52177_d_n0, assign34660_e52177_d_n1, assign34660_e52177_d_n2, assign34660_e52177_d_n3, assign34660_e52177_d_n4, assign34660_e52177_d_n5, assign34660_e52177_d_b0, assign34660_e52177_d_b1, assign34660_e52177_d_b2, assign34660_e52177_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard565 == 0.0)) {
        let assign34660_e52155: f64 = (var_exp_k * p.p90);
        let assign34660_e52157: f64 = (-p.p91);
        let assign34660_e52160: f64 = (var_vak - var_v_hk);
        let assign34660_e52161: f64 = (assign34660_e52157 * assign34660_e52160);
        let assign34660_e52164: f64 = (var_vak - var_v_hk);
        let assign34660_e52165: f64 = (assign34660_e52161 * assign34660_e52164);
        let assign34660_e52169: f64 = (var_tkr / var_tkd);
        let assign34660_e52170: f64 = (assign34660_e52169).ln();
        let assign34660_e52171: f64 = (p.p98 * assign34660_e52170);
        let assign34660_e52172: f64 = (assign34660_e52171).exp();
        let assign34660_e52173: f64 = (assign34660_e52165 * assign34660_e52172);
        let assign34660_e52174: f64 = (assign34660_e52173).exp();
        let assign34660_e52175: f64 = (assign34660_e52155 * assign34660_e52174);
        (assign34660_e52175, (((var_exp_k_dn0 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * (((((assign34660_e52157 * (var_vak_dn0 - var_v_hk_dn0)) * assign34660_e52164) + (assign34660_e52161 * (var_vak_dn0 - var_v_hk_dn0))) * assign34660_e52172) + (assign34660_e52165 * (assign34660_e52172 * (p.p98 * ((((var_tkr_dn0 * var_tkd) - (var_tkr * var_tkd_dn0)) / (var_tkd * var_tkd)) / assign34660_e52169)))))))), (((var_exp_k_dn1 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * (((((assign34660_e52157 * (var_vak_dn1 - var_v_hk_dn1)) * assign34660_e52164) + (assign34660_e52161 * (var_vak_dn1 - var_v_hk_dn1))) * assign34660_e52172) + (assign34660_e52165 * (assign34660_e52172 * (p.p98 * ((((var_tkr_dn1 * var_tkd) - (var_tkr * var_tkd_dn1)) / (var_tkd * var_tkd)) / assign34660_e52169)))))))), (((var_exp_k_dn2 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * (((((assign34660_e52157 * (var_vak_dn2 - var_v_hk_dn2)) * assign34660_e52164) + (assign34660_e52161 * (var_vak_dn2 - var_v_hk_dn2))) * assign34660_e52172) + (assign34660_e52165 * (assign34660_e52172 * (p.p98 * ((((var_tkr_dn2 * var_tkd) - (var_tkr * var_tkd_dn2)) / (var_tkd * var_tkd)) / assign34660_e52169)))))))), (((var_exp_k_dn3 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * (((((assign34660_e52157 * (var_vak_dn3 - var_v_hk_dn3)) * assign34660_e52164) + (assign34660_e52161 * (var_vak_dn3 - var_v_hk_dn3))) * assign34660_e52172) + (assign34660_e52165 * (assign34660_e52172 * (p.p98 * ((((var_tkr_dn3 * var_tkd) - (var_tkr * var_tkd_dn3)) / (var_tkd * var_tkd)) / assign34660_e52169)))))))), (((var_exp_k_dn4 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * (((((assign34660_e52157 * (var_vak_dn4 - var_v_hk_dn4)) * assign34660_e52164) + (assign34660_e52161 * (var_vak_dn4 - var_v_hk_dn4))) * assign34660_e52172) + (assign34660_e52165 * (assign34660_e52172 * (p.p98 * ((((var_tkr_dn4 * var_tkd) - (var_tkr * var_tkd_dn4)) / (var_tkd * var_tkd)) / assign34660_e52169)))))))), (((var_exp_k_dn5 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * (((((assign34660_e52157 * (var_vak_dn5 - var_v_hk_dn5)) * assign34660_e52164) + (assign34660_e52161 * (var_vak_dn5 - var_v_hk_dn5))) * assign34660_e52172) + (assign34660_e52165 * (assign34660_e52172 * (p.p98 * ((((var_tkr_dn5 * var_tkd) - (var_tkr * var_tkd_dn5)) / (var_tkd * var_tkd)) / assign34660_e52169)))))))), (((var_exp_k_db0 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * (((((assign34660_e52157 * (var_vak_db0 - var_v_hk_db0)) * assign34660_e52164) + (assign34660_e52161 * (var_vak_db0 - var_v_hk_db0))) * assign34660_e52172) + (assign34660_e52165 * (assign34660_e52172 * (p.p98 * ((((var_tkr_db0 * var_tkd) - (var_tkr * var_tkd_db0)) / (var_tkd * var_tkd)) / assign34660_e52169)))))))), (((var_exp_k_db1 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * (((((assign34660_e52157 * (var_vak_db1 - var_v_hk_db1)) * assign34660_e52164) + (assign34660_e52161 * (var_vak_db1 - var_v_hk_db1))) * assign34660_e52172) + (assign34660_e52165 * (assign34660_e52172 * (p.p98 * ((((var_tkr_db1 * var_tkd) - (var_tkr * var_tkd_db1)) / (var_tkd * var_tkd)) / assign34660_e52169)))))))), (((var_exp_k_db2 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * (((((assign34660_e52157 * (var_vak_db2 - var_v_hk_db2)) * assign34660_e52164) + (assign34660_e52161 * (var_vak_db2 - var_v_hk_db2))) * assign34660_e52172) + (assign34660_e52165 * (assign34660_e52172 * (p.p98 * ((((var_tkr_db2 * var_tkd) - (var_tkr * var_tkd_db2)) / (var_tkd * var_tkd)) / assign34660_e52169)))))))), (((var_exp_k_db3 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * (((((assign34660_e52157 * (var_vak_db3 - var_v_hk_db3)) * assign34660_e52164) + (assign34660_e52161 * (var_vak_db3 - var_v_hk_db3))) * assign34660_e52172) + (assign34660_e52165 * (assign34660_e52172 * (p.p98 * ((((var_tkr_db3 * var_tkd) - (var_tkr * var_tkd_db3)) / (var_tkd * var_tkd)) / assign34660_e52169)))))))),)
    } else {
        (var_exp_k2, var_exp_k2_dn0, var_exp_k2_dn1, var_exp_k2_dn2, var_exp_k2_dn3, var_exp_k2_dn4, var_exp_k2_dn5, var_exp_k2_db0, var_exp_k2_db1, var_exp_k2_db2, var_exp_k2_db3,)
    }
};
        var_exp_k2 = assign34660_e52177;
        var_exp_k2_dn0 = assign34660_e52177_d_n0;
        var_exp_k2_dn1 = assign34660_e52177_d_n1;
        var_exp_k2_dn2 = assign34660_e52177_d_n2;
        var_exp_k2_dn3 = assign34660_e52177_d_n3;
        var_exp_k2_dn4 = assign34660_e52177_d_n4;
        var_exp_k2_dn5 = assign34660_e52177_d_n5;
        var_exp_k2_db0 = assign34660_e52177_d_b0;
        var_exp_k2_db1 = assign34660_e52177_d_b1;
        var_exp_k2_db2 = assign34660_e52177_d_b2;
        var_exp_k2_db3 = assign34660_e52177_d_b3;
        var_exp_k2_rv = 0.0;
        var_exp_k2_rdn0 = 0.0;
        var_exp_k2_rdn1 = 0.0;
        var_exp_k2_rdn2 = 0.0;
        var_exp_k2_rdn3 = 0.0;
        var_exp_k2_rdn4 = 0.0;
        var_exp_k2_rdn5 = 0.0;
        var_exp_k2_rdb0 = 0.0;
        var_exp_k2_rdb1 = 0.0;
        var_exp_k2_rdb2 = 0.0;
        var_exp_k2_rdb3 = 0.0;

        let (assign34670_e52186, assign34670_e52186_d_n0, assign34670_e52186_d_n1, assign34670_e52186_d_n2, assign34670_e52186_d_n3, assign34670_e52186_d_n4, assign34670_e52186_d_n5, assign34670_e52186_d_b0, assign34670_e52186_d_b1, assign34670_e52186_d_b2, assign34670_e52186_d_b3,) = {
    if (var_guard558 != 0.0) {
        let (assign34670_e52184, assign34670_e52184_d_n0, assign34670_e52184_d_n1, assign34670_e52184_d_n2, assign34670_e52184_d_n3, assign34670_e52184_d_n4, assign34670_e52184_d_n5, assign34670_e52184_d_b0, assign34670_e52184_d_b1, assign34670_e52184_d_b2, assign34670_e52184_d_b3,) = {
            if (var_exp_k2 > p.p79) {
                (p.p79, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (var_exp_k2, var_exp_k2_dn0, var_exp_k2_dn1, var_exp_k2_dn2, var_exp_k2_dn3, var_exp_k2_dn4, var_exp_k2_dn5, var_exp_k2_db0, var_exp_k2_db1, var_exp_k2_db2, var_exp_k2_db3,)
            }
        };
        (assign34670_e52184, assign34670_e52184_d_n0, assign34670_e52184_d_n1, assign34670_e52184_d_n2, assign34670_e52184_d_n3, assign34670_e52184_d_n4, assign34670_e52184_d_n5, assign34670_e52184_d_b0, assign34670_e52184_d_b1, assign34670_e52184_d_b2, assign34670_e52184_d_b3,)
    } else {
        (var_exp_k2, var_exp_k2_dn0, var_exp_k2_dn1, var_exp_k2_dn2, var_exp_k2_dn3, var_exp_k2_dn4, var_exp_k2_dn5, var_exp_k2_db0, var_exp_k2_db1, var_exp_k2_db2, var_exp_k2_db3,)
    }
};
        var_exp_k2 = assign34670_e52186;
        var_exp_k2_dn0 = assign34670_e52186_d_n0;
        var_exp_k2_dn1 = assign34670_e52186_d_n1;
        var_exp_k2_dn2 = assign34670_e52186_d_n2;
        var_exp_k2_dn3 = assign34670_e52186_d_n3;
        var_exp_k2_dn4 = assign34670_e52186_d_n4;
        var_exp_k2_dn5 = assign34670_e52186_d_n5;
        var_exp_k2_db0 = assign34670_e52186_d_b0;
        var_exp_k2_db1 = assign34670_e52186_d_b1;
        var_exp_k2_db2 = assign34670_e52186_d_b2;
        var_exp_k2_db3 = assign34670_e52186_d_b3;
        var_exp_k2_rv = 0.0;
        var_exp_k2_rdn0 = 0.0;
        var_exp_k2_rdn1 = 0.0;
        var_exp_k2_rdn2 = 0.0;
        var_exp_k2_rdn3 = 0.0;
        var_exp_k2_rdn4 = 0.0;
        var_exp_k2_rdn5 = 0.0;
        var_exp_k2_rdb0 = 0.0;
        var_exp_k2_rdb1 = 0.0;
        var_exp_k2_rdb2 = 0.0;
        var_exp_k2_rdb3 = 0.0;

        let (assign34680_e52192, assign34680_e52192_d_n0, assign34680_e52192_d_n1, assign34680_e52192_d_n2, assign34680_e52192_d_n3, assign34680_e52192_d_n4, assign34680_e52192_d_n5, assign34680_e52192_d_b0, assign34680_e52192_d_b1, assign34680_e52192_d_b2, assign34680_e52192_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34680_e52190: f64 = (var_pn0 * var_exp_k2);
        (assign34680_e52190, ((var_pn0_dn0 * var_exp_k2) + (var_pn0 * var_exp_k2_dn0)), ((var_pn0_dn1 * var_exp_k2) + (var_pn0 * var_exp_k2_dn1)), ((var_pn0_dn2 * var_exp_k2) + (var_pn0 * var_exp_k2_dn2)), ((var_pn0_dn3 * var_exp_k2) + (var_pn0 * var_exp_k2_dn3)), ((var_pn0_dn4 * var_exp_k2) + (var_pn0 * var_exp_k2_dn4)), ((var_pn0_dn5 * var_exp_k2) + (var_pn0 * var_exp_k2_dn5)), ((var_pn0_db0 * var_exp_k2) + (var_pn0 * var_exp_k2_db0)), ((var_pn0_db1 * var_exp_k2) + (var_pn0 * var_exp_k2_db1)), ((var_pn0_db2 * var_exp_k2) + (var_pn0 * var_exp_k2_db2)), ((var_pn0_db3 * var_exp_k2) + (var_pn0 * var_exp_k2_db3)),)
    } else {
        (var_p_nk, var_p_nk_dn0, var_p_nk_dn1, var_p_nk_dn2, var_p_nk_dn3, var_p_nk_dn4, var_p_nk_dn5, var_p_nk_db0, var_p_nk_db1, var_p_nk_db2, var_p_nk_db3,)
    }
};
        var_p_nk = assign34680_e52192;
        var_p_nk_dn0 = assign34680_e52192_d_n0;
        var_p_nk_dn1 = assign34680_e52192_d_n1;
        var_p_nk_dn2 = assign34680_e52192_d_n2;
        var_p_nk_dn3 = assign34680_e52192_d_n3;
        var_p_nk_dn4 = assign34680_e52192_d_n4;
        var_p_nk_dn5 = assign34680_e52192_d_n5;
        var_p_nk_db0 = assign34680_e52192_d_b0;
        var_p_nk_db1 = assign34680_e52192_d_b1;
        var_p_nk_db2 = assign34680_e52192_d_b2;
        var_p_nk_db3 = assign34680_e52192_d_b3;
        var_p_nk_rv = 0.0;
        var_p_nk_rdn0 = 0.0;
        var_p_nk_rdn1 = 0.0;
        var_p_nk_rdn2 = 0.0;
        var_p_nk_rdn3 = 0.0;
        var_p_nk_rdn4 = 0.0;
        var_p_nk_rdn5 = 0.0;
        var_p_nk_rdb0 = 0.0;
        var_p_nk_rdb1 = 0.0;
        var_p_nk_rdb2 = 0.0;
        var_p_nk_rdb3 = 0.0;

        let (assign34690_e52202, assign34690_e52202_d_n0, assign34690_e52202_d_n1, assign34690_e52202_d_n2, assign34690_e52202_d_n3, assign34690_e52202_d_n4, assign34690_e52202_d_n5, assign34690_e52202_d_b0, assign34690_e52202_d_b1, assign34690_e52202_d_b2, assign34690_e52202_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34690_e52196: f64 = (1.6021918e-19 * var_ab_i);
        let assign34690_e52199: f64 = (var_p_nk - var_pn0);
        let assign34690_e52200: f64 = (assign34690_e52196 * assign34690_e52199);
        (assign34690_e52200, (((1.6021918e-19 * var_ab_i_dn0) * assign34690_e52199) + (assign34690_e52196 * (var_p_nk_dn0 - var_pn0_dn0))), (((1.6021918e-19 * var_ab_i_dn1) * assign34690_e52199) + (assign34690_e52196 * (var_p_nk_dn1 - var_pn0_dn1))), (((1.6021918e-19 * var_ab_i_dn2) * assign34690_e52199) + (assign34690_e52196 * (var_p_nk_dn2 - var_pn0_dn2))), (((1.6021918e-19 * var_ab_i_dn3) * assign34690_e52199) + (assign34690_e52196 * (var_p_nk_dn3 - var_pn0_dn3))), (((1.6021918e-19 * var_ab_i_dn4) * assign34690_e52199) + (assign34690_e52196 * (var_p_nk_dn4 - var_pn0_dn4))), (((1.6021918e-19 * var_ab_i_dn5) * assign34690_e52199) + (assign34690_e52196 * (var_p_nk_dn5 - var_pn0_dn5))), (((1.6021918e-19 * var_ab_i_db0) * assign34690_e52199) + (assign34690_e52196 * (var_p_nk_db0 - var_pn0_db0))), (((1.6021918e-19 * var_ab_i_db1) * assign34690_e52199) + (assign34690_e52196 * (var_p_nk_db1 - var_pn0_db1))), (((1.6021918e-19 * var_ab_i_db2) * assign34690_e52199) + (assign34690_e52196 * (var_p_nk_db2 - var_pn0_db2))), (((1.6021918e-19 * var_ab_i_db3) * assign34690_e52199) + (assign34690_e52196 * (var_p_nk_db3 - var_pn0_db3))),)
    } else {
        (var_q_pexk, var_q_pexk_dn0, var_q_pexk_dn1, var_q_pexk_dn2, var_q_pexk_dn3, var_q_pexk_dn4, var_q_pexk_dn5, var_q_pexk_db0, var_q_pexk_db1, var_q_pexk_db2, var_q_pexk_db3,)
    }
};
        var_q_pexk = assign34690_e52202;
        var_q_pexk_dn0 = assign34690_e52202_d_n0;
        var_q_pexk_dn1 = assign34690_e52202_d_n1;
        var_q_pexk_dn2 = assign34690_e52202_d_n2;
        var_q_pexk_dn3 = assign34690_e52202_d_n3;
        var_q_pexk_dn4 = assign34690_e52202_d_n4;
        var_q_pexk_dn5 = assign34690_e52202_d_n5;
        var_q_pexk_db0 = assign34690_e52202_d_b0;
        var_q_pexk_db1 = assign34690_e52202_d_b1;
        var_q_pexk_db2 = assign34690_e52202_d_b2;
        var_q_pexk_db3 = assign34690_e52202_d_b3;
        var_q_pexk_rv = 0.0;
        var_q_pexk_rdn0 = 0.0;
        var_q_pexk_rdn1 = 0.0;
        var_q_pexk_rdn2 = 0.0;
        var_q_pexk_rdn3 = 0.0;
        var_q_pexk_rdn4 = 0.0;
        var_q_pexk_rdn5 = 0.0;
        var_q_pexk_rdb0 = 0.0;
        var_q_pexk_rdb1 = 0.0;
        var_q_pexk_rdb2 = 0.0;
        var_q_pexk_rdb3 = 0.0;

        let assign34700_e52205: f64 = if p.p92 > 0.0 { 1.0 } else { 0.0 };
        var_guard566 = assign34700_e52205;
        var_guard566_dn0 = 0.0;
        var_guard566_dn1 = 0.0;
        var_guard566_dn2 = 0.0;
        var_guard566_dn3 = 0.0;
        var_guard566_dn4 = 0.0;
        var_guard566_dn5 = 0.0;
        var_guard566_db0 = 0.0;
        var_guard566_db1 = 0.0;
        var_guard566_db2 = 0.0;
        var_guard566_db3 = 0.0;
        var_guard566_rv = 0.0;
        var_guard566_rdn0 = 0.0;
        var_guard566_rdn1 = 0.0;
        var_guard566_rdn2 = 0.0;
        var_guard566_rdn3 = 0.0;
        var_guard566_rdn4 = 0.0;
        var_guard566_rdn5 = 0.0;
        var_guard566_rdb0 = 0.0;
        var_guard566_rdb1 = 0.0;
        var_guard566_rdb2 = 0.0;
        var_guard566_rdb3 = 0.0;

        let (assign34710_e52215, assign34710_e52215_d_n0, assign34710_e52215_d_n1, assign34710_e52215_d_n2, assign34710_e52215_d_n3, assign34710_e52215_d_n4, assign34710_e52215_d_n5, assign34710_e52215_d_b0, assign34710_e52215_d_b1, assign34710_e52215_d_b2, assign34710_e52215_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard566 != 0.0)) {
        let assign34710_e52212: f64 = (1e-23 / var_q_pex0);
        let assign34710_e52213: f64 = (var_q_pexk * assign34710_e52212);
        (assign34710_e52213, ((var_q_pexk_dn0 * assign34710_e52212) + (var_q_pexk * (-((1e-23 * var_q_pex0_dn0) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexk_dn1 * assign34710_e52212) + (var_q_pexk * (-((1e-23 * var_q_pex0_dn1) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexk_dn2 * assign34710_e52212) + (var_q_pexk * (-((1e-23 * var_q_pex0_dn2) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexk_dn3 * assign34710_e52212) + (var_q_pexk * (-((1e-23 * var_q_pex0_dn3) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexk_dn4 * assign34710_e52212) + (var_q_pexk * (-((1e-23 * var_q_pex0_dn4) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexk_dn5 * assign34710_e52212) + (var_q_pexk * (-((1e-23 * var_q_pex0_dn5) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexk_db0 * assign34710_e52212) + (var_q_pexk * (-((1e-23 * var_q_pex0_db0) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexk_db1 * assign34710_e52212) + (var_q_pexk * (-((1e-23 * var_q_pex0_db1) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexk_db2 * assign34710_e52212) + (var_q_pexk * (-((1e-23 * var_q_pex0_db2) / (var_q_pex0 * var_q_pex0))))), ((var_q_pexk_db3 * assign34710_e52212) + (var_q_pexk * (-((1e-23 * var_q_pex0_db3) / (var_q_pex0 * var_q_pex0))))),)
    } else {
        (var_q_qs_k, var_q_qs_k_dn0, var_q_qs_k_dn1, var_q_qs_k_dn2, var_q_qs_k_dn3, var_q_qs_k_dn4, var_q_qs_k_dn5, var_q_qs_k_db0, var_q_qs_k_db1, var_q_qs_k_db2, var_q_qs_k_db3,)
    }
};
        var_q_qs_k = assign34710_e52215;
        var_q_qs_k_dn0 = assign34710_e52215_d_n0;
        var_q_qs_k_dn1 = assign34710_e52215_d_n1;
        var_q_qs_k_dn2 = assign34710_e52215_d_n2;
        var_q_qs_k_dn3 = assign34710_e52215_d_n3;
        var_q_qs_k_dn4 = assign34710_e52215_d_n4;
        var_q_qs_k_dn5 = assign34710_e52215_d_n5;
        var_q_qs_k_db0 = assign34710_e52215_d_b0;
        var_q_qs_k_db1 = assign34710_e52215_d_b1;
        var_q_qs_k_db2 = assign34710_e52215_d_b2;
        var_q_qs_k_db3 = assign34710_e52215_d_b3;
        var_q_qs_k_rv = 0.0;
        var_q_qs_k_rdn0 = 0.0;
        var_q_qs_k_rdn1 = 0.0;
        var_q_qs_k_rdn2 = 0.0;
        var_q_qs_k_rdn3 = 0.0;
        var_q_qs_k_rdn4 = 0.0;
        var_q_qs_k_rdn5 = 0.0;
        var_q_qs_k_rdb0 = 0.0;
        var_q_qs_k_rdb1 = 0.0;
        var_q_qs_k_rdb2 = 0.0;
        var_q_qs_k_rdb3 = 0.0;

        let (assign34720_e52223, assign34720_e52223_d_n0, assign34720_e52223_d_n1, assign34720_e52223_d_n2, assign34720_e52223_d_n3, assign34720_e52223_d_n4, assign34720_e52223_d_n5, assign34720_e52223_d_b0, assign34720_e52223_d_b1, assign34720_e52223_d_b2, assign34720_e52223_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard566 != 0.0)) {
        let assign34720_e52221: f64 = (nv4 - 0.0);
        (assign34720_e52221, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_q_nqs_k, var_q_nqs_k_dn0, var_q_nqs_k_dn1, var_q_nqs_k_dn2, var_q_nqs_k_dn3, var_q_nqs_k_dn4, var_q_nqs_k_dn5, var_q_nqs_k_db0, var_q_nqs_k_db1, var_q_nqs_k_db2, var_q_nqs_k_db3,)
    }
};
        var_q_nqs_k = assign34720_e52223;
        var_q_nqs_k_dn0 = assign34720_e52223_d_n0;
        var_q_nqs_k_dn1 = assign34720_e52223_d_n1;
        var_q_nqs_k_dn2 = assign34720_e52223_d_n2;
        var_q_nqs_k_dn3 = assign34720_e52223_d_n3;
        var_q_nqs_k_dn4 = assign34720_e52223_d_n4;
        var_q_nqs_k_dn5 = assign34720_e52223_d_n5;
        var_q_nqs_k_db0 = assign34720_e52223_d_b0;
        var_q_nqs_k_db1 = assign34720_e52223_d_b1;
        var_q_nqs_k_db2 = assign34720_e52223_d_b2;
        var_q_nqs_k_db3 = assign34720_e52223_d_b3;
        var_q_nqs_k_rv = 0.0;
        var_q_nqs_k_rdn0 = 0.0;
        var_q_nqs_k_rdn1 = 0.0;
        var_q_nqs_k_rdn2 = 0.0;
        var_q_nqs_k_rdn3 = 0.0;
        var_q_nqs_k_rdn4 = 0.0;
        var_q_nqs_k_rdn5 = 0.0;
        var_q_nqs_k_rdb0 = 0.0;
        var_q_nqs_k_rdb1 = 0.0;
        var_q_nqs_k_rdb2 = 0.0;
        var_q_nqs_k_rdb3 = 0.0;

        let (assign34730_e52233, assign34730_e52233_d_n0, assign34730_e52233_d_n1, assign34730_e52233_d_n2, assign34730_e52233_d_n3, assign34730_e52233_d_n4, assign34730_e52233_d_n5, assign34730_e52233_d_b0, assign34730_e52233_d_b1, assign34730_e52233_d_b2, assign34730_e52233_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard566 != 0.0)) {
        let assign34730_e52229: f64 = (var_q_nqs_k - var_q_qs_k);
        let assign34730_e52231: f64 = (assign34730_e52229 / p.p92);
        (assign34730_e52231, ((var_q_nqs_k_dn0 - var_q_qs_k_dn0) / p.p92), ((var_q_nqs_k_dn1 - var_q_qs_k_dn1) / p.p92), ((var_q_nqs_k_dn2 - var_q_qs_k_dn2) / p.p92), ((var_q_nqs_k_dn3 - var_q_qs_k_dn3) / p.p92), ((var_q_nqs_k_dn4 - var_q_qs_k_dn4) / p.p92), ((var_q_nqs_k_dn5 - var_q_qs_k_dn5) / p.p92), ((var_q_nqs_k_db0 - var_q_qs_k_db0) / p.p92), ((var_q_nqs_k_db1 - var_q_qs_k_db1) / p.p92), ((var_q_nqs_k_db2 - var_q_qs_k_db2) / p.p92), ((var_q_nqs_k_db3 - var_q_qs_k_db3) / p.p92),)
    } else {
        (var_inqs0_k, var_inqs0_k_dn0, var_inqs0_k_dn1, var_inqs0_k_dn2, var_inqs0_k_dn3, var_inqs0_k_dn4, var_inqs0_k_dn5, var_inqs0_k_db0, var_inqs0_k_db1, var_inqs0_k_db2, var_inqs0_k_db3,)
    }
};
        var_inqs0_k = assign34730_e52233;
        var_inqs0_k_dn0 = assign34730_e52233_d_n0;
        var_inqs0_k_dn1 = assign34730_e52233_d_n1;
        var_inqs0_k_dn2 = assign34730_e52233_d_n2;
        var_inqs0_k_dn3 = assign34730_e52233_d_n3;
        var_inqs0_k_dn4 = assign34730_e52233_d_n4;
        var_inqs0_k_dn5 = assign34730_e52233_d_n5;
        var_inqs0_k_db0 = assign34730_e52233_d_b0;
        var_inqs0_k_db1 = assign34730_e52233_d_b1;
        var_inqs0_k_db2 = assign34730_e52233_d_b2;
        var_inqs0_k_db3 = assign34730_e52233_d_b3;
        var_inqs0_k_rv = 0.0;
        var_inqs0_k_rdn0 = 0.0;
        var_inqs0_k_rdn1 = 0.0;
        var_inqs0_k_rdn2 = 0.0;
        var_inqs0_k_rdn3 = 0.0;
        var_inqs0_k_rdn4 = 0.0;
        var_inqs0_k_rdn5 = 0.0;
        var_inqs0_k_rdb0 = 0.0;
        var_inqs0_k_rdb1 = 0.0;
        var_inqs0_k_rdb2 = 0.0;
        var_inqs0_k_rdb3 = 0.0;

        let (assign34750_e52250, assign34750_e52250_d_n0, assign34750_e52250_d_n1, assign34750_e52250_d_n2, assign34750_e52250_d_n3, assign34750_e52250_d_n4, assign34750_e52250_d_n5, assign34750_e52250_d_b0, assign34750_e52250_d_b1, assign34750_e52250_d_b2, assign34750_e52250_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard566 == 0.0)) {
        (var_q_pexk, var_q_pexk_dn0, var_q_pexk_dn1, var_q_pexk_dn2, var_q_pexk_dn3, var_q_pexk_dn4, var_q_pexk_dn5, var_q_pexk_db0, var_q_pexk_db1, var_q_pexk_db2, var_q_pexk_db3,)
    } else {
        (var_q_qs_k, var_q_qs_k_dn0, var_q_qs_k_dn1, var_q_qs_k_dn2, var_q_qs_k_dn3, var_q_qs_k_dn4, var_q_qs_k_dn5, var_q_qs_k_db0, var_q_qs_k_db1, var_q_qs_k_db2, var_q_qs_k_db3,)
    }
};
        var_q_qs_k = assign34750_e52250;
        var_q_qs_k_dn0 = assign34750_e52250_d_n0;
        var_q_qs_k_dn1 = assign34750_e52250_d_n1;
        var_q_qs_k_dn2 = assign34750_e52250_d_n2;
        var_q_qs_k_dn3 = assign34750_e52250_d_n3;
        var_q_qs_k_dn4 = assign34750_e52250_d_n4;
        var_q_qs_k_dn5 = assign34750_e52250_d_n5;
        var_q_qs_k_db0 = assign34750_e52250_d_b0;
        var_q_qs_k_db1 = assign34750_e52250_d_b1;
        var_q_qs_k_db2 = assign34750_e52250_d_b2;
        var_q_qs_k_db3 = assign34750_e52250_d_b3;
        var_q_qs_k_rv = 0.0;
        var_q_qs_k_rdn0 = 0.0;
        var_q_qs_k_rdn1 = 0.0;
        var_q_qs_k_rdn2 = 0.0;
        var_q_qs_k_rdn3 = 0.0;
        var_q_qs_k_rdn4 = 0.0;
        var_q_qs_k_rdn5 = 0.0;
        var_q_qs_k_rdb0 = 0.0;
        var_q_qs_k_rdb1 = 0.0;
        var_q_qs_k_rdb2 = 0.0;
        var_q_qs_k_rdb3 = 0.0;

        let (assign34770_e52263, assign34770_e52263_d_n0, assign34770_e52263_d_n1, assign34770_e52263_d_n2, assign34770_e52263_d_n3, assign34770_e52263_d_n4, assign34770_e52263_d_n5, assign34770_e52263_d_b0, assign34770_e52263_d_b1, assign34770_e52263_d_b2, assign34770_e52263_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34770_e52261: f64 = (var_pb - var_vak);
        (assign34770_e52261, (var_pb_dn0 - var_vak_dn0), (var_pb_dn1 - var_vak_dn1), (var_pb_dn2 - var_vak_dn2), (var_pb_dn3 - var_vak_dn3), (var_pb_dn4 - var_vak_dn4), (var_pb_dn5 - var_vak_dn5), (var_pb_db0 - var_vak_db0), (var_pb_db1 - var_vak_db1), (var_pb_db2 - var_vak_db2), (var_pb_db3 - var_vak_db3),)
    } else {
        (var_vjunc_a, var_vjunc_a_dn0, var_vjunc_a_dn1, var_vjunc_a_dn2, var_vjunc_a_dn3, var_vjunc_a_dn4, var_vjunc_a_dn5, var_vjunc_a_db0, var_vjunc_a_db1, var_vjunc_a_db2, var_vjunc_a_db3,)
    }
};
        var_vjunc_a = assign34770_e52263;
        var_vjunc_a_dn0 = assign34770_e52263_d_n0;
        var_vjunc_a_dn1 = assign34770_e52263_d_n1;
        var_vjunc_a_dn2 = assign34770_e52263_d_n2;
        var_vjunc_a_dn3 = assign34770_e52263_d_n3;
        var_vjunc_a_dn4 = assign34770_e52263_d_n4;
        var_vjunc_a_dn5 = assign34770_e52263_d_n5;
        var_vjunc_a_db0 = assign34770_e52263_d_b0;
        var_vjunc_a_db1 = assign34770_e52263_d_b1;
        var_vjunc_a_db2 = assign34770_e52263_d_b2;
        var_vjunc_a_db3 = assign34770_e52263_d_b3;
        var_vjunc_a_rv = 0.0;
        var_vjunc_a_rdn0 = 0.0;
        var_vjunc_a_rdn1 = 0.0;
        var_vjunc_a_rdn2 = 0.0;
        var_vjunc_a_rdn3 = 0.0;
        var_vjunc_a_rdn4 = 0.0;
        var_vjunc_a_rdn5 = 0.0;
        var_vjunc_a_rdb0 = 0.0;
        var_vjunc_a_rdb1 = 0.0;
        var_vjunc_a_rdb2 = 0.0;
        var_vjunc_a_rdb3 = 0.0;

        let (assign34780_e52276, assign34780_e52276_d_n0, assign34780_e52276_d_n1, assign34780_e52276_d_n2, assign34780_e52276_d_n3, assign34780_e52276_d_n4, assign34780_e52276_d_n5, assign34780_e52276_d_b0, assign34780_e52276_d_b1, assign34780_e52276_d_b2, assign34780_e52276_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34780_e52267: f64 = (var_vjunc_a * var_vjunc_a);
        let assign34780_e52270: f64 = (4.0 * var_juncdlt);
        let assign34780_e52272: f64 = (assign34780_e52270 * var_juncdlt);
        let assign34780_e52273: f64 = (assign34780_e52267 + assign34780_e52272);
        let assign34780_e52274: f64 = (assign34780_e52273).sqrt();
        (assign34780_e52274, ((((var_vjunc_a_dn0 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn0)) + (((4.0 * var_juncdlt_dn0) * var_juncdlt) + (assign34780_e52270 * var_juncdlt_dn0))) / (2.0 * assign34780_e52274)), ((((var_vjunc_a_dn1 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn1)) + (((4.0 * var_juncdlt_dn1) * var_juncdlt) + (assign34780_e52270 * var_juncdlt_dn1))) / (2.0 * assign34780_e52274)), ((((var_vjunc_a_dn2 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn2)) + (((4.0 * var_juncdlt_dn2) * var_juncdlt) + (assign34780_e52270 * var_juncdlt_dn2))) / (2.0 * assign34780_e52274)), ((((var_vjunc_a_dn3 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn3)) + (((4.0 * var_juncdlt_dn3) * var_juncdlt) + (assign34780_e52270 * var_juncdlt_dn3))) / (2.0 * assign34780_e52274)), ((((var_vjunc_a_dn4 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn4)) + (((4.0 * var_juncdlt_dn4) * var_juncdlt) + (assign34780_e52270 * var_juncdlt_dn4))) / (2.0 * assign34780_e52274)), ((((var_vjunc_a_dn5 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn5)) + (((4.0 * var_juncdlt_dn5) * var_juncdlt) + (assign34780_e52270 * var_juncdlt_dn5))) / (2.0 * assign34780_e52274)), ((((var_vjunc_a_db0 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_db0)) + (((4.0 * var_juncdlt_db0) * var_juncdlt) + (assign34780_e52270 * var_juncdlt_db0))) / (2.0 * assign34780_e52274)), ((((var_vjunc_a_db1 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_db1)) + (((4.0 * var_juncdlt_db1) * var_juncdlt) + (assign34780_e52270 * var_juncdlt_db1))) / (2.0 * assign34780_e52274)), ((((var_vjunc_a_db2 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_db2)) + (((4.0 * var_juncdlt_db2) * var_juncdlt) + (assign34780_e52270 * var_juncdlt_db2))) / (2.0 * assign34780_e52274)), ((((var_vjunc_a_db3 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_db3)) + (((4.0 * var_juncdlt_db3) * var_juncdlt) + (assign34780_e52270 * var_juncdlt_db3))) / (2.0 * assign34780_e52274)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34780_e52276;
        var_tmf2_dn0 = assign34780_e52276_d_n0;
        var_tmf2_dn1 = assign34780_e52276_d_n1;
        var_tmf2_dn2 = assign34780_e52276_d_n2;
        var_tmf2_dn3 = assign34780_e52276_d_n3;
        var_tmf2_dn4 = assign34780_e52276_d_n4;
        var_tmf2_dn5 = assign34780_e52276_d_n5;
        var_tmf2_db0 = assign34780_e52276_d_b0;
        var_tmf2_db1 = assign34780_e52276_d_b1;
        var_tmf2_db2 = assign34780_e52276_d_b2;
        var_tmf2_db3 = assign34780_e52276_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34790_e52284, assign34790_e52284_d_n0, assign34790_e52284_d_n1, assign34790_e52284_d_n2, assign34790_e52284_d_n3, assign34790_e52284_d_n4, assign34790_e52284_d_n5, assign34790_e52284_d_b0, assign34790_e52284_d_b1, assign34790_e52284_d_b2, assign34790_e52284_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34790_e52281: f64 = (var_vjunc_a + var_tmf2);
        let assign34790_e52282: f64 = (0.5 * assign34790_e52281);
        (assign34790_e52282, (0.5 * (var_vjunc_a_dn0 + var_tmf2_dn0)), (0.5 * (var_vjunc_a_dn1 + var_tmf2_dn1)), (0.5 * (var_vjunc_a_dn2 + var_tmf2_dn2)), (0.5 * (var_vjunc_a_dn3 + var_tmf2_dn3)), (0.5 * (var_vjunc_a_dn4 + var_tmf2_dn4)), (0.5 * (var_vjunc_a_dn5 + var_tmf2_dn5)), (0.5 * (var_vjunc_a_db0 + var_tmf2_db0)), (0.5 * (var_vjunc_a_db1 + var_tmf2_db1)), (0.5 * (var_vjunc_a_db2 + var_tmf2_db2)), (0.5 * (var_vjunc_a_db3 + var_tmf2_db3)),)
    } else {
        (var_vjunc_a, var_vjunc_a_dn0, var_vjunc_a_dn1, var_vjunc_a_dn2, var_vjunc_a_dn3, var_vjunc_a_dn4, var_vjunc_a_dn5, var_vjunc_a_db0, var_vjunc_a_db1, var_vjunc_a_db2, var_vjunc_a_db3,)
    }
};
        var_vjunc_a = assign34790_e52284;
        var_vjunc_a_dn0 = assign34790_e52284_d_n0;
        var_vjunc_a_dn1 = assign34790_e52284_d_n1;
        var_vjunc_a_dn2 = assign34790_e52284_d_n2;
        var_vjunc_a_dn3 = assign34790_e52284_d_n3;
        var_vjunc_a_dn4 = assign34790_e52284_d_n4;
        var_vjunc_a_dn5 = assign34790_e52284_d_n5;
        var_vjunc_a_db0 = assign34790_e52284_d_b0;
        var_vjunc_a_db1 = assign34790_e52284_d_b1;
        var_vjunc_a_db2 = assign34790_e52284_d_b2;
        var_vjunc_a_db3 = assign34790_e52284_d_b3;
        var_vjunc_a_rv = 0.0;
        var_vjunc_a_rdn0 = 0.0;
        var_vjunc_a_rdn1 = 0.0;
        var_vjunc_a_rdn2 = 0.0;
        var_vjunc_a_rdn3 = 0.0;
        var_vjunc_a_rdn4 = 0.0;
        var_vjunc_a_rdn5 = 0.0;
        var_vjunc_a_rdb0 = 0.0;
        var_vjunc_a_rdb1 = 0.0;
        var_vjunc_a_rdb2 = 0.0;
        var_vjunc_a_rdb3 = 0.0;

        let assign34800_e52287: f64 = if var_vjunc_a < 0.0 { 1.0 } else { 0.0 };
        var_guard567 = assign34800_e52287;
        var_guard567_dn0 = 0.0;
        var_guard567_dn1 = 0.0;
        var_guard567_dn2 = 0.0;
        var_guard567_dn3 = 0.0;
        var_guard567_dn4 = 0.0;
        var_guard567_dn5 = 0.0;
        var_guard567_db0 = 0.0;
        var_guard567_db1 = 0.0;
        var_guard567_db2 = 0.0;
        var_guard567_db3 = 0.0;
        var_guard567_rv = 0.0;
        var_guard567_rdn0 = 0.0;
        var_guard567_rdn1 = 0.0;
        var_guard567_rdn2 = 0.0;
        var_guard567_rdn3 = 0.0;
        var_guard567_rdn4 = 0.0;
        var_guard567_rdn5 = 0.0;
        var_guard567_rdb0 = 0.0;
        var_guard567_rdb1 = 0.0;
        var_guard567_rdb2 = 0.0;
        var_guard567_rdb3 = 0.0;

        let (assign34810_e52293, assign34810_e52293_d_n0, assign34810_e52293_d_n1, assign34810_e52293_d_n2, assign34810_e52293_d_n3, assign34810_e52293_d_n4, assign34810_e52293_d_n5, assign34810_e52293_d_b0, assign34810_e52293_d_b1, assign34810_e52293_d_b2, assign34810_e52293_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard567 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vjunc_a, var_vjunc_a_dn0, var_vjunc_a_dn1, var_vjunc_a_dn2, var_vjunc_a_dn3, var_vjunc_a_dn4, var_vjunc_a_dn5, var_vjunc_a_db0, var_vjunc_a_db1, var_vjunc_a_db2, var_vjunc_a_db3,)
    }
};
        var_vjunc_a = assign34810_e52293;
        var_vjunc_a_dn0 = assign34810_e52293_d_n0;
        var_vjunc_a_dn1 = assign34810_e52293_d_n1;
        var_vjunc_a_dn2 = assign34810_e52293_d_n2;
        var_vjunc_a_dn3 = assign34810_e52293_d_n3;
        var_vjunc_a_dn4 = assign34810_e52293_d_n4;
        var_vjunc_a_dn5 = assign34810_e52293_d_n5;
        var_vjunc_a_db0 = assign34810_e52293_d_b0;
        var_vjunc_a_db1 = assign34810_e52293_d_b1;
        var_vjunc_a_db2 = assign34810_e52293_d_b2;
        var_vjunc_a_db3 = assign34810_e52293_d_b3;
        var_vjunc_a_rv = 0.0;
        var_vjunc_a_rdn0 = 0.0;
        var_vjunc_a_rdn1 = 0.0;
        var_vjunc_a_rdn2 = 0.0;
        var_vjunc_a_rdn3 = 0.0;
        var_vjunc_a_rdn4 = 0.0;
        var_vjunc_a_rdn5 = 0.0;
        var_vjunc_a_rdb0 = 0.0;
        var_vjunc_a_rdb1 = 0.0;
        var_vjunc_a_rdb2 = 0.0;
        var_vjunc_a_rdb3 = 0.0;

        let (assign34820_e52306, assign34820_e52306_d_n0, assign34820_e52306_d_n1, assign34820_e52306_d_n2, assign34820_e52306_d_n3, assign34820_e52306_d_n4, assign34820_e52306_d_n5, assign34820_e52306_d_b0, assign34820_e52306_d_b1, assign34820_e52306_d_b2, assign34820_e52306_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34820_e52297: f64 = (2.0 * var_epssi);
        let assign34820_e52299: f64 = (assign34820_e52297 * var_vjunc_a);
        let assign34820_e52302: f64 = (1.6021918e-19 * var_ndi_i);
        let assign34820_e52303: f64 = (assign34820_e52299 / assign34820_e52302);
        let assign34820_e52304: f64 = (assign34820_e52303).sqrt();
        (assign34820_e52304, (((((((2.0 * var_epssi_dn0) * var_vjunc_a) + (assign34820_e52297 * var_vjunc_a_dn0)) * assign34820_e52302) - (assign34820_e52299 * (1.6021918e-19 * var_ndi_i_dn0))) / (assign34820_e52302 * assign34820_e52302)) / (2.0 * assign34820_e52304)), (((((((2.0 * var_epssi_dn1) * var_vjunc_a) + (assign34820_e52297 * var_vjunc_a_dn1)) * assign34820_e52302) - (assign34820_e52299 * (1.6021918e-19 * var_ndi_i_dn1))) / (assign34820_e52302 * assign34820_e52302)) / (2.0 * assign34820_e52304)), (((((((2.0 * var_epssi_dn2) * var_vjunc_a) + (assign34820_e52297 * var_vjunc_a_dn2)) * assign34820_e52302) - (assign34820_e52299 * (1.6021918e-19 * var_ndi_i_dn2))) / (assign34820_e52302 * assign34820_e52302)) / (2.0 * assign34820_e52304)), (((((((2.0 * var_epssi_dn3) * var_vjunc_a) + (assign34820_e52297 * var_vjunc_a_dn3)) * assign34820_e52302) - (assign34820_e52299 * (1.6021918e-19 * var_ndi_i_dn3))) / (assign34820_e52302 * assign34820_e52302)) / (2.0 * assign34820_e52304)), (((((((2.0 * var_epssi_dn4) * var_vjunc_a) + (assign34820_e52297 * var_vjunc_a_dn4)) * assign34820_e52302) - (assign34820_e52299 * (1.6021918e-19 * var_ndi_i_dn4))) / (assign34820_e52302 * assign34820_e52302)) / (2.0 * assign34820_e52304)), (((((((2.0 * var_epssi_dn5) * var_vjunc_a) + (assign34820_e52297 * var_vjunc_a_dn5)) * assign34820_e52302) - (assign34820_e52299 * (1.6021918e-19 * var_ndi_i_dn5))) / (assign34820_e52302 * assign34820_e52302)) / (2.0 * assign34820_e52304)), (((((((2.0 * var_epssi_db0) * var_vjunc_a) + (assign34820_e52297 * var_vjunc_a_db0)) * assign34820_e52302) - (assign34820_e52299 * (1.6021918e-19 * var_ndi_i_db0))) / (assign34820_e52302 * assign34820_e52302)) / (2.0 * assign34820_e52304)), (((((((2.0 * var_epssi_db1) * var_vjunc_a) + (assign34820_e52297 * var_vjunc_a_db1)) * assign34820_e52302) - (assign34820_e52299 * (1.6021918e-19 * var_ndi_i_db1))) / (assign34820_e52302 * assign34820_e52302)) / (2.0 * assign34820_e52304)), (((((((2.0 * var_epssi_db2) * var_vjunc_a) + (assign34820_e52297 * var_vjunc_a_db2)) * assign34820_e52302) - (assign34820_e52299 * (1.6021918e-19 * var_ndi_i_db2))) / (assign34820_e52302 * assign34820_e52302)) / (2.0 * assign34820_e52304)), (((((((2.0 * var_epssi_db3) * var_vjunc_a) + (assign34820_e52297 * var_vjunc_a_db3)) * assign34820_e52302) - (assign34820_e52299 * (1.6021918e-19 * var_ndi_i_db3))) / (assign34820_e52302 * assign34820_e52302)) / (2.0 * assign34820_e52304)),)
    } else {
        (var_w_depa, var_w_depa_dn0, var_w_depa_dn1, var_w_depa_dn2, var_w_depa_dn3, var_w_depa_dn4, var_w_depa_dn5, var_w_depa_db0, var_w_depa_db1, var_w_depa_db2, var_w_depa_db3,)
    }
};
        var_w_depa = assign34820_e52306;
        var_w_depa_dn0 = assign34820_e52306_d_n0;
        var_w_depa_dn1 = assign34820_e52306_d_n1;
        var_w_depa_dn2 = assign34820_e52306_d_n2;
        var_w_depa_dn3 = assign34820_e52306_d_n3;
        var_w_depa_dn4 = assign34820_e52306_d_n4;
        var_w_depa_dn5 = assign34820_e52306_d_n5;
        var_w_depa_db0 = assign34820_e52306_d_b0;
        var_w_depa_db1 = assign34820_e52306_d_b1;
        var_w_depa_db2 = assign34820_e52306_d_b2;
        var_w_depa_db3 = assign34820_e52306_d_b3;
        var_w_depa_rv = 0.0;
        var_w_depa_rdn0 = 0.0;
        var_w_depa_rdn1 = 0.0;
        var_w_depa_rdn2 = 0.0;
        var_w_depa_rdn3 = 0.0;
        var_w_depa_rdn4 = 0.0;
        var_w_depa_rdn5 = 0.0;
        var_w_depa_rdb0 = 0.0;
        var_w_depa_rdb1 = 0.0;
        var_w_depa_rdb2 = 0.0;
        var_w_depa_rdb3 = 0.0;

        let (assign34830_e52314, assign34830_e52314_d_n0, assign34830_e52314_d_n1, assign34830_e52314_d_n2, assign34830_e52314_d_n3, assign34830_e52314_d_n4, assign34830_e52314_d_n5, assign34830_e52314_d_b0, assign34830_e52314_d_b1, assign34830_e52314_d_b2, assign34830_e52314_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34830_e52310: f64 = (p.p94 - var_w_depa);
        let assign34830_e52312: f64 = (assign34830_e52310 - 1e-7);
        (assign34830_e52312, (-var_w_depa_dn0), (-var_w_depa_dn1), (-var_w_depa_dn2), (-var_w_depa_dn3), (-var_w_depa_dn4), (-var_w_depa_dn5), (-var_w_depa_db0), (-var_w_depa_db1), (-var_w_depa_db2), (-var_w_depa_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign34830_e52314;
        var_tmf1_dn0 = assign34830_e52314_d_n0;
        var_tmf1_dn1 = assign34830_e52314_d_n1;
        var_tmf1_dn2 = assign34830_e52314_d_n2;
        var_tmf1_dn3 = assign34830_e52314_d_n3;
        var_tmf1_dn4 = assign34830_e52314_d_n4;
        var_tmf1_dn5 = assign34830_e52314_d_n5;
        var_tmf1_db0 = assign34830_e52314_d_b0;
        var_tmf1_db1 = assign34830_e52314_d_b1;
        var_tmf1_db2 = assign34830_e52314_d_b2;
        var_tmf1_db3 = assign34830_e52314_d_b3;
        var_tmf1_rv = 0.0;
        var_tmf1_rdn0 = 0.0;
        var_tmf1_rdn1 = 0.0;
        var_tmf1_rdn2 = 0.0;
        var_tmf1_rdn3 = 0.0;
        var_tmf1_rdn4 = 0.0;
        var_tmf1_rdn5 = 0.0;
        var_tmf1_rdb0 = 0.0;
        var_tmf1_rdb1 = 0.0;
        var_tmf1_rdb2 = 0.0;
        var_tmf1_rdb3 = 0.0;

        *var_exp_k2_slot = var_exp_k2;
        *var_exp_k2_db0_slot = var_exp_k2_db0;
        *var_exp_k2_db1_slot = var_exp_k2_db1;
        *var_exp_k2_db2_slot = var_exp_k2_db2;
        *var_exp_k2_db3_slot = var_exp_k2_db3;
        *var_exp_k2_dn0_slot = var_exp_k2_dn0;
        *var_exp_k2_dn1_slot = var_exp_k2_dn1;
        *var_exp_k2_dn2_slot = var_exp_k2_dn2;
        *var_exp_k2_dn3_slot = var_exp_k2_dn3;
        *var_exp_k2_dn4_slot = var_exp_k2_dn4;
        *var_exp_k2_dn5_slot = var_exp_k2_dn5;
        *var_exp_k2_rdb0_slot = var_exp_k2_rdb0;
        *var_exp_k2_rdb1_slot = var_exp_k2_rdb1;
        *var_exp_k2_rdb2_slot = var_exp_k2_rdb2;
        *var_exp_k2_rdb3_slot = var_exp_k2_rdb3;
        *var_exp_k2_rdn0_slot = var_exp_k2_rdn0;
        *var_exp_k2_rdn1_slot = var_exp_k2_rdn1;
        *var_exp_k2_rdn2_slot = var_exp_k2_rdn2;
        *var_exp_k2_rdn3_slot = var_exp_k2_rdn3;
        *var_exp_k2_rdn4_slot = var_exp_k2_rdn4;
        *var_exp_k2_rdn5_slot = var_exp_k2_rdn5;
        *var_exp_k2_rv_slot = var_exp_k2_rv;
        *var_guard566_slot = var_guard566;
        *var_guard566_db0_slot = var_guard566_db0;
        *var_guard566_db1_slot = var_guard566_db1;
        *var_guard566_db2_slot = var_guard566_db2;
        *var_guard566_db3_slot = var_guard566_db3;
        *var_guard566_dn0_slot = var_guard566_dn0;
        *var_guard566_dn1_slot = var_guard566_dn1;
        *var_guard566_dn2_slot = var_guard566_dn2;
        *var_guard566_dn3_slot = var_guard566_dn3;
        *var_guard566_dn4_slot = var_guard566_dn4;
        *var_guard566_dn5_slot = var_guard566_dn5;
        *var_guard566_rdb0_slot = var_guard566_rdb0;
        *var_guard566_rdb1_slot = var_guard566_rdb1;
        *var_guard566_rdb2_slot = var_guard566_rdb2;
        *var_guard566_rdb3_slot = var_guard566_rdb3;
        *var_guard566_rdn0_slot = var_guard566_rdn0;
        *var_guard566_rdn1_slot = var_guard566_rdn1;
        *var_guard566_rdn2_slot = var_guard566_rdn2;
        *var_guard566_rdn3_slot = var_guard566_rdn3;
        *var_guard566_rdn4_slot = var_guard566_rdn4;
        *var_guard566_rdn5_slot = var_guard566_rdn5;
        *var_guard566_rv_slot = var_guard566_rv;
        *var_guard567_slot = var_guard567;
        *var_guard567_db0_slot = var_guard567_db0;
        *var_guard567_db1_slot = var_guard567_db1;
        *var_guard567_db2_slot = var_guard567_db2;
        *var_guard567_db3_slot = var_guard567_db3;
        *var_guard567_dn0_slot = var_guard567_dn0;
        *var_guard567_dn1_slot = var_guard567_dn1;
        *var_guard567_dn2_slot = var_guard567_dn2;
        *var_guard567_dn3_slot = var_guard567_dn3;
        *var_guard567_dn4_slot = var_guard567_dn4;
        *var_guard567_dn5_slot = var_guard567_dn5;
        *var_guard567_rdb0_slot = var_guard567_rdb0;
        *var_guard567_rdb1_slot = var_guard567_rdb1;
        *var_guard567_rdb2_slot = var_guard567_rdb2;
        *var_guard567_rdb3_slot = var_guard567_rdb3;
        *var_guard567_rdn0_slot = var_guard567_rdn0;
        *var_guard567_rdn1_slot = var_guard567_rdn1;
        *var_guard567_rdn2_slot = var_guard567_rdn2;
        *var_guard567_rdn3_slot = var_guard567_rdn3;
        *var_guard567_rdn4_slot = var_guard567_rdn4;
        *var_guard567_rdn5_slot = var_guard567_rdn5;
        *var_guard567_rv_slot = var_guard567_rv;
        *var_inqs0_k_slot = var_inqs0_k;
        *var_inqs0_k_db0_slot = var_inqs0_k_db0;
        *var_inqs0_k_db1_slot = var_inqs0_k_db1;
        *var_inqs0_k_db2_slot = var_inqs0_k_db2;
        *var_inqs0_k_db3_slot = var_inqs0_k_db3;
        *var_inqs0_k_dn0_slot = var_inqs0_k_dn0;
        *var_inqs0_k_dn1_slot = var_inqs0_k_dn1;
        *var_inqs0_k_dn2_slot = var_inqs0_k_dn2;
        *var_inqs0_k_dn3_slot = var_inqs0_k_dn3;
        *var_inqs0_k_dn4_slot = var_inqs0_k_dn4;
        *var_inqs0_k_dn5_slot = var_inqs0_k_dn5;
        *var_inqs0_k_rdb0_slot = var_inqs0_k_rdb0;
        *var_inqs0_k_rdb1_slot = var_inqs0_k_rdb1;
        *var_inqs0_k_rdb2_slot = var_inqs0_k_rdb2;
        *var_inqs0_k_rdb3_slot = var_inqs0_k_rdb3;
        *var_inqs0_k_rdn0_slot = var_inqs0_k_rdn0;
        *var_inqs0_k_rdn1_slot = var_inqs0_k_rdn1;
        *var_inqs0_k_rdn2_slot = var_inqs0_k_rdn2;
        *var_inqs0_k_rdn3_slot = var_inqs0_k_rdn3;
        *var_inqs0_k_rdn4_slot = var_inqs0_k_rdn4;
        *var_inqs0_k_rdn5_slot = var_inqs0_k_rdn5;
        *var_inqs0_k_rv_slot = var_inqs0_k_rv;
        *var_p_nk_slot = var_p_nk;
        *var_p_nk_db0_slot = var_p_nk_db0;
        *var_p_nk_db1_slot = var_p_nk_db1;
        *var_p_nk_db2_slot = var_p_nk_db2;
        *var_p_nk_db3_slot = var_p_nk_db3;
        *var_p_nk_dn0_slot = var_p_nk_dn0;
        *var_p_nk_dn1_slot = var_p_nk_dn1;
        *var_p_nk_dn2_slot = var_p_nk_dn2;
        *var_p_nk_dn3_slot = var_p_nk_dn3;
        *var_p_nk_dn4_slot = var_p_nk_dn4;
        *var_p_nk_dn5_slot = var_p_nk_dn5;
        *var_p_nk_rdb0_slot = var_p_nk_rdb0;
        *var_p_nk_rdb1_slot = var_p_nk_rdb1;
        *var_p_nk_rdb2_slot = var_p_nk_rdb2;
        *var_p_nk_rdb3_slot = var_p_nk_rdb3;
        *var_p_nk_rdn0_slot = var_p_nk_rdn0;
        *var_p_nk_rdn1_slot = var_p_nk_rdn1;
        *var_p_nk_rdn2_slot = var_p_nk_rdn2;
        *var_p_nk_rdn3_slot = var_p_nk_rdn3;
        *var_p_nk_rdn4_slot = var_p_nk_rdn4;
        *var_p_nk_rdn5_slot = var_p_nk_rdn5;
        *var_p_nk_rv_slot = var_p_nk_rv;
        *var_q_nqs_k_slot = var_q_nqs_k;
        *var_q_nqs_k_db0_slot = var_q_nqs_k_db0;
        *var_q_nqs_k_db1_slot = var_q_nqs_k_db1;
        *var_q_nqs_k_db2_slot = var_q_nqs_k_db2;
        *var_q_nqs_k_db3_slot = var_q_nqs_k_db3;
        *var_q_nqs_k_dn0_slot = var_q_nqs_k_dn0;
        *var_q_nqs_k_dn1_slot = var_q_nqs_k_dn1;
        *var_q_nqs_k_dn2_slot = var_q_nqs_k_dn2;
        *var_q_nqs_k_dn3_slot = var_q_nqs_k_dn3;
        *var_q_nqs_k_dn4_slot = var_q_nqs_k_dn4;
        *var_q_nqs_k_dn5_slot = var_q_nqs_k_dn5;
        *var_q_nqs_k_rdb0_slot = var_q_nqs_k_rdb0;
        *var_q_nqs_k_rdb1_slot = var_q_nqs_k_rdb1;
        *var_q_nqs_k_rdb2_slot = var_q_nqs_k_rdb2;
        *var_q_nqs_k_rdb3_slot = var_q_nqs_k_rdb3;
        *var_q_nqs_k_rdn0_slot = var_q_nqs_k_rdn0;
        *var_q_nqs_k_rdn1_slot = var_q_nqs_k_rdn1;
        *var_q_nqs_k_rdn2_slot = var_q_nqs_k_rdn2;
        *var_q_nqs_k_rdn3_slot = var_q_nqs_k_rdn3;
        *var_q_nqs_k_rdn4_slot = var_q_nqs_k_rdn4;
        *var_q_nqs_k_rdn5_slot = var_q_nqs_k_rdn5;
        *var_q_nqs_k_rv_slot = var_q_nqs_k_rv;
        *var_q_pexk_slot = var_q_pexk;
        *var_q_pexk_db0_slot = var_q_pexk_db0;
        *var_q_pexk_db1_slot = var_q_pexk_db1;
        *var_q_pexk_db2_slot = var_q_pexk_db2;
        *var_q_pexk_db3_slot = var_q_pexk_db3;
        *var_q_pexk_dn0_slot = var_q_pexk_dn0;
        *var_q_pexk_dn1_slot = var_q_pexk_dn1;
        *var_q_pexk_dn2_slot = var_q_pexk_dn2;
        *var_q_pexk_dn3_slot = var_q_pexk_dn3;
        *var_q_pexk_dn4_slot = var_q_pexk_dn4;
        *var_q_pexk_dn5_slot = var_q_pexk_dn5;
        *var_q_pexk_rdb0_slot = var_q_pexk_rdb0;
        *var_q_pexk_rdb1_slot = var_q_pexk_rdb1;
        *var_q_pexk_rdb2_slot = var_q_pexk_rdb2;
        *var_q_pexk_rdb3_slot = var_q_pexk_rdb3;
        *var_q_pexk_rdn0_slot = var_q_pexk_rdn0;
        *var_q_pexk_rdn1_slot = var_q_pexk_rdn1;
        *var_q_pexk_rdn2_slot = var_q_pexk_rdn2;
        *var_q_pexk_rdn3_slot = var_q_pexk_rdn3;
        *var_q_pexk_rdn4_slot = var_q_pexk_rdn4;
        *var_q_pexk_rdn5_slot = var_q_pexk_rdn5;
        *var_q_pexk_rv_slot = var_q_pexk_rv;
        *var_q_qs_k_slot = var_q_qs_k;
        *var_q_qs_k_db0_slot = var_q_qs_k_db0;
        *var_q_qs_k_db1_slot = var_q_qs_k_db1;
        *var_q_qs_k_db2_slot = var_q_qs_k_db2;
        *var_q_qs_k_db3_slot = var_q_qs_k_db3;
        *var_q_qs_k_dn0_slot = var_q_qs_k_dn0;
        *var_q_qs_k_dn1_slot = var_q_qs_k_dn1;
        *var_q_qs_k_dn2_slot = var_q_qs_k_dn2;
        *var_q_qs_k_dn3_slot = var_q_qs_k_dn3;
        *var_q_qs_k_dn4_slot = var_q_qs_k_dn4;
        *var_q_qs_k_dn5_slot = var_q_qs_k_dn5;
        *var_q_qs_k_rdb0_slot = var_q_qs_k_rdb0;
        *var_q_qs_k_rdb1_slot = var_q_qs_k_rdb1;
        *var_q_qs_k_rdb2_slot = var_q_qs_k_rdb2;
        *var_q_qs_k_rdb3_slot = var_q_qs_k_rdb3;
        *var_q_qs_k_rdn0_slot = var_q_qs_k_rdn0;
        *var_q_qs_k_rdn1_slot = var_q_qs_k_rdn1;
        *var_q_qs_k_rdn2_slot = var_q_qs_k_rdn2;
        *var_q_qs_k_rdn3_slot = var_q_qs_k_rdn3;
        *var_q_qs_k_rdn4_slot = var_q_qs_k_rdn4;
        *var_q_qs_k_rdn5_slot = var_q_qs_k_rdn5;
        *var_q_qs_k_rv_slot = var_q_qs_k_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_rdb0_slot = var_tmf1_rdb0;
        *var_tmf1_rdb1_slot = var_tmf1_rdb1;
        *var_tmf1_rdb2_slot = var_tmf1_rdb2;
        *var_tmf1_rdb3_slot = var_tmf1_rdb3;
        *var_tmf1_rdn0_slot = var_tmf1_rdn0;
        *var_tmf1_rdn1_slot = var_tmf1_rdn1;
        *var_tmf1_rdn2_slot = var_tmf1_rdn2;
        *var_tmf1_rdn3_slot = var_tmf1_rdn3;
        *var_tmf1_rdn4_slot = var_tmf1_rdn4;
        *var_tmf1_rdn5_slot = var_tmf1_rdn5;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_rdb0_slot = var_tmf2_rdb0;
        *var_tmf2_rdb1_slot = var_tmf2_rdb1;
        *var_tmf2_rdb2_slot = var_tmf2_rdb2;
        *var_tmf2_rdb3_slot = var_tmf2_rdb3;
        *var_tmf2_rdn0_slot = var_tmf2_rdn0;
        *var_tmf2_rdn1_slot = var_tmf2_rdn1;
        *var_tmf2_rdn2_slot = var_tmf2_rdn2;
        *var_tmf2_rdn3_slot = var_tmf2_rdn3;
        *var_tmf2_rdn4_slot = var_tmf2_rdn4;
        *var_tmf2_rdn5_slot = var_tmf2_rdn5;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vjunc_a_slot = var_vjunc_a;
        *var_vjunc_a_db0_slot = var_vjunc_a_db0;
        *var_vjunc_a_db1_slot = var_vjunc_a_db1;
        *var_vjunc_a_db2_slot = var_vjunc_a_db2;
        *var_vjunc_a_db3_slot = var_vjunc_a_db3;
        *var_vjunc_a_dn0_slot = var_vjunc_a_dn0;
        *var_vjunc_a_dn1_slot = var_vjunc_a_dn1;
        *var_vjunc_a_dn2_slot = var_vjunc_a_dn2;
        *var_vjunc_a_dn3_slot = var_vjunc_a_dn3;
        *var_vjunc_a_dn4_slot = var_vjunc_a_dn4;
        *var_vjunc_a_dn5_slot = var_vjunc_a_dn5;
        *var_vjunc_a_rdb0_slot = var_vjunc_a_rdb0;
        *var_vjunc_a_rdb1_slot = var_vjunc_a_rdb1;
        *var_vjunc_a_rdb2_slot = var_vjunc_a_rdb2;
        *var_vjunc_a_rdb3_slot = var_vjunc_a_rdb3;
        *var_vjunc_a_rdn0_slot = var_vjunc_a_rdn0;
        *var_vjunc_a_rdn1_slot = var_vjunc_a_rdn1;
        *var_vjunc_a_rdn2_slot = var_vjunc_a_rdn2;
        *var_vjunc_a_rdn3_slot = var_vjunc_a_rdn3;
        *var_vjunc_a_rdn4_slot = var_vjunc_a_rdn4;
        *var_vjunc_a_rdn5_slot = var_vjunc_a_rdn5;
        *var_vjunc_a_rv_slot = var_vjunc_a_rv;
        *var_w_depa_slot = var_w_depa;
        *var_w_depa_db0_slot = var_w_depa_db0;
        *var_w_depa_db1_slot = var_w_depa_db1;
        *var_w_depa_db2_slot = var_w_depa_db2;
        *var_w_depa_db3_slot = var_w_depa_db3;
        *var_w_depa_dn0_slot = var_w_depa_dn0;
        *var_w_depa_dn1_slot = var_w_depa_dn1;
        *var_w_depa_dn2_slot = var_w_depa_dn2;
        *var_w_depa_dn3_slot = var_w_depa_dn3;
        *var_w_depa_dn4_slot = var_w_depa_dn4;
        *var_w_depa_dn5_slot = var_w_depa_dn5;
        *var_w_depa_rdb0_slot = var_w_depa_rdb0;
        *var_w_depa_rdb1_slot = var_w_depa_rdb1;
        *var_w_depa_rdb2_slot = var_w_depa_rdb2;
        *var_w_depa_rdb3_slot = var_w_depa_rdb3;
        *var_w_depa_rdn0_slot = var_w_depa_rdn0;
        *var_w_depa_rdn1_slot = var_w_depa_rdn1;
        *var_w_depa_rdn2_slot = var_w_depa_rdn2;
        *var_w_depa_rdn3_slot = var_w_depa_rdn3;
        *var_w_depa_rdn4_slot = var_w_depa_rdn4;
        *var_w_depa_rdn5_slot = var_w_depa_rdn5;
        *var_w_depa_rv_slot = var_w_depa_rv;
    }

    pub(super) fn stamp_reactive_block_88(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard558: f64,
        var_tmf1: f64,
        var_tmf1_db0: f64,
        var_tmf1_db1: f64,
        var_tmf1_db2: f64,
        var_tmf1_db3: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn1: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn3: f64,
        var_tmf1_dn4: f64,
        var_tmf1_dn5: f64,
        var_w_depa0: f64,
        var_w_depa0_db0: f64,
        var_w_depa0_db1: f64,
        var_w_depa0_db2: f64,
        var_w_depa0_db3: f64,
        var_w_depa0_dn0: f64,
        var_w_depa0_dn1: f64,
        var_w_depa0_dn2: f64,
        var_w_depa0_dn3: f64,
        var_w_depa0_dn4: f64,
        var_w_depa0_dn5: f64,
        var_guard568_slot: &mut f64,
        var_guard568_db0_slot: &mut f64,
        var_guard568_db1_slot: &mut f64,
        var_guard568_db2_slot: &mut f64,
        var_guard568_db3_slot: &mut f64,
        var_guard568_dn0_slot: &mut f64,
        var_guard568_dn1_slot: &mut f64,
        var_guard568_dn2_slot: &mut f64,
        var_guard568_dn3_slot: &mut f64,
        var_guard568_dn4_slot: &mut f64,
        var_guard568_dn5_slot: &mut f64,
        var_guard568_rdb0_slot: &mut f64,
        var_guard568_rdb1_slot: &mut f64,
        var_guard568_rdb2_slot: &mut f64,
        var_guard568_rdb3_slot: &mut f64,
        var_guard568_rdn0_slot: &mut f64,
        var_guard568_rdn1_slot: &mut f64,
        var_guard568_rdn2_slot: &mut f64,
        var_guard568_rdn3_slot: &mut f64,
        var_guard568_rdn4_slot: &mut f64,
        var_guard568_rdn5_slot: &mut f64,
        var_guard568_rv_slot: &mut f64,
        var_guard571_slot: &mut f64,
        var_guard571_db0_slot: &mut f64,
        var_guard571_db1_slot: &mut f64,
        var_guard571_db2_slot: &mut f64,
        var_guard571_db3_slot: &mut f64,
        var_guard571_dn0_slot: &mut f64,
        var_guard571_dn1_slot: &mut f64,
        var_guard571_dn2_slot: &mut f64,
        var_guard571_dn3_slot: &mut f64,
        var_guard571_dn4_slot: &mut f64,
        var_guard571_dn5_slot: &mut f64,
        var_guard571_rdb0_slot: &mut f64,
        var_guard571_rdb1_slot: &mut f64,
        var_guard571_rdb2_slot: &mut f64,
        var_guard571_rdb3_slot: &mut f64,
        var_guard571_rdn0_slot: &mut f64,
        var_guard571_rdn1_slot: &mut f64,
        var_guard571_rdn2_slot: &mut f64,
        var_guard571_rdn3_slot: &mut f64,
        var_guard571_rdn4_slot: &mut f64,
        var_guard571_rdn5_slot: &mut f64,
        var_guard571_rv_slot: &mut f64,
        var_guard572_slot: &mut f64,
        var_guard572_db0_slot: &mut f64,
        var_guard572_db1_slot: &mut f64,
        var_guard572_db2_slot: &mut f64,
        var_guard572_db3_slot: &mut f64,
        var_guard572_dn0_slot: &mut f64,
        var_guard572_dn1_slot: &mut f64,
        var_guard572_dn2_slot: &mut f64,
        var_guard572_dn3_slot: &mut f64,
        var_guard572_dn4_slot: &mut f64,
        var_guard572_dn5_slot: &mut f64,
        var_guard572_rdb0_slot: &mut f64,
        var_guard572_rdb1_slot: &mut f64,
        var_guard572_rdb2_slot: &mut f64,
        var_guard572_rdb3_slot: &mut f64,
        var_guard572_rdn0_slot: &mut f64,
        var_guard572_rdn1_slot: &mut f64,
        var_guard572_rdn2_slot: &mut f64,
        var_guard572_rdn3_slot: &mut f64,
        var_guard572_rdn4_slot: &mut f64,
        var_guard572_rdn5_slot: &mut f64,
        var_guard572_rv_slot: &mut f64,
        var_iwnqs0_a_slot: &mut f64,
        var_iwnqs0_a_db0_slot: &mut f64,
        var_iwnqs0_a_db1_slot: &mut f64,
        var_iwnqs0_a_db2_slot: &mut f64,
        var_iwnqs0_a_db3_slot: &mut f64,
        var_iwnqs0_a_dn0_slot: &mut f64,
        var_iwnqs0_a_dn1_slot: &mut f64,
        var_iwnqs0_a_dn2_slot: &mut f64,
        var_iwnqs0_a_dn3_slot: &mut f64,
        var_iwnqs0_a_dn4_slot: &mut f64,
        var_iwnqs0_a_dn5_slot: &mut f64,
        var_iwnqs0_a_rdb0_slot: &mut f64,
        var_iwnqs0_a_rdb1_slot: &mut f64,
        var_iwnqs0_a_rdb2_slot: &mut f64,
        var_iwnqs0_a_rdb3_slot: &mut f64,
        var_iwnqs0_a_rdn0_slot: &mut f64,
        var_iwnqs0_a_rdn1_slot: &mut f64,
        var_iwnqs0_a_rdn2_slot: &mut f64,
        var_iwnqs0_a_rdn3_slot: &mut f64,
        var_iwnqs0_a_rdn4_slot: &mut f64,
        var_iwnqs0_a_rdn5_slot: &mut f64,
        var_iwnqs0_a_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_rdb0_slot: &mut f64,
        var_tmf2_rdb1_slot: &mut f64,
        var_tmf2_rdb2_slot: &mut f64,
        var_tmf2_rdb3_slot: &mut f64,
        var_tmf2_rdn0_slot: &mut f64,
        var_tmf2_rdn1_slot: &mut f64,
        var_tmf2_rdn2_slot: &mut f64,
        var_tmf2_rdn3_slot: &mut f64,
        var_tmf2_rdn4_slot: &mut f64,
        var_tmf2_rdn5_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_w_depa_slot: &mut f64,
        var_w_depa_db0_slot: &mut f64,
        var_w_depa_db1_slot: &mut f64,
        var_w_depa_db2_slot: &mut f64,
        var_w_depa_db3_slot: &mut f64,
        var_w_depa_dn0_slot: &mut f64,
        var_w_depa_dn1_slot: &mut f64,
        var_w_depa_dn2_slot: &mut f64,
        var_w_depa_dn3_slot: &mut f64,
        var_w_depa_dn4_slot: &mut f64,
        var_w_depa_dn5_slot: &mut f64,
        var_w_depa_rdb0_slot: &mut f64,
        var_w_depa_rdb1_slot: &mut f64,
        var_w_depa_rdb2_slot: &mut f64,
        var_w_depa_rdb3_slot: &mut f64,
        var_w_depa_rdn0_slot: &mut f64,
        var_w_depa_rdn1_slot: &mut f64,
        var_w_depa_rdn2_slot: &mut f64,
        var_w_depa_rdn3_slot: &mut f64,
        var_w_depa_rdn4_slot: &mut f64,
        var_w_depa_rdn5_slot: &mut f64,
        var_w_depa_rv_slot: &mut f64,
        var_w_nqs_a_slot: &mut f64,
        var_w_nqs_a_db0_slot: &mut f64,
        var_w_nqs_a_db1_slot: &mut f64,
        var_w_nqs_a_db2_slot: &mut f64,
        var_w_nqs_a_db3_slot: &mut f64,
        var_w_nqs_a_dn0_slot: &mut f64,
        var_w_nqs_a_dn1_slot: &mut f64,
        var_w_nqs_a_dn2_slot: &mut f64,
        var_w_nqs_a_dn3_slot: &mut f64,
        var_w_nqs_a_dn4_slot: &mut f64,
        var_w_nqs_a_dn5_slot: &mut f64,
        var_w_nqs_a_rdb0_slot: &mut f64,
        var_w_nqs_a_rdb1_slot: &mut f64,
        var_w_nqs_a_rdb2_slot: &mut f64,
        var_w_nqs_a_rdb3_slot: &mut f64,
        var_w_nqs_a_rdn0_slot: &mut f64,
        var_w_nqs_a_rdn1_slot: &mut f64,
        var_w_nqs_a_rdn2_slot: &mut f64,
        var_w_nqs_a_rdn3_slot: &mut f64,
        var_w_nqs_a_rdn4_slot: &mut f64,
        var_w_nqs_a_rdn5_slot: &mut f64,
        var_w_nqs_a_rv_slot: &mut f64,
        var_w_qs_a_slot: &mut f64,
        var_w_qs_a_db0_slot: &mut f64,
        var_w_qs_a_db1_slot: &mut f64,
        var_w_qs_a_db2_slot: &mut f64,
        var_w_qs_a_db3_slot: &mut f64,
        var_w_qs_a_dn0_slot: &mut f64,
        var_w_qs_a_dn1_slot: &mut f64,
        var_w_qs_a_dn2_slot: &mut f64,
        var_w_qs_a_dn3_slot: &mut f64,
        var_w_qs_a_dn4_slot: &mut f64,
        var_w_qs_a_dn5_slot: &mut f64,
        var_w_qs_a_rdb0_slot: &mut f64,
        var_w_qs_a_rdb1_slot: &mut f64,
        var_w_qs_a_rdb2_slot: &mut f64,
        var_w_qs_a_rdb3_slot: &mut f64,
        var_w_qs_a_rdn0_slot: &mut f64,
        var_w_qs_a_rdn1_slot: &mut f64,
        var_w_qs_a_rdn2_slot: &mut f64,
        var_w_qs_a_rdn3_slot: &mut f64,
        var_w_qs_a_rdn4_slot: &mut f64,
        var_w_qs_a_rdn5_slot: &mut f64,
        var_w_qs_a_rv_slot: &mut f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let mut var_guard568: f64 = *var_guard568_slot;
        let mut var_guard568_db0: f64 = *var_guard568_db0_slot;
        let mut var_guard568_db1: f64 = *var_guard568_db1_slot;
        let mut var_guard568_db2: f64 = *var_guard568_db2_slot;
        let mut var_guard568_db3: f64 = *var_guard568_db3_slot;
        let mut var_guard568_dn0: f64 = *var_guard568_dn0_slot;
        let mut var_guard568_dn1: f64 = *var_guard568_dn1_slot;
        let mut var_guard568_dn2: f64 = *var_guard568_dn2_slot;
        let mut var_guard568_dn3: f64 = *var_guard568_dn3_slot;
        let mut var_guard568_dn4: f64 = *var_guard568_dn4_slot;
        let mut var_guard568_dn5: f64 = *var_guard568_dn5_slot;
        let mut var_guard568_rdb0: f64 = *var_guard568_rdb0_slot;
        let mut var_guard568_rdb1: f64 = *var_guard568_rdb1_slot;
        let mut var_guard568_rdb2: f64 = *var_guard568_rdb2_slot;
        let mut var_guard568_rdb3: f64 = *var_guard568_rdb3_slot;
        let mut var_guard568_rdn0: f64 = *var_guard568_rdn0_slot;
        let mut var_guard568_rdn1: f64 = *var_guard568_rdn1_slot;
        let mut var_guard568_rdn2: f64 = *var_guard568_rdn2_slot;
        let mut var_guard568_rdn3: f64 = *var_guard568_rdn3_slot;
        let mut var_guard568_rdn4: f64 = *var_guard568_rdn4_slot;
        let mut var_guard568_rdn5: f64 = *var_guard568_rdn5_slot;
        let mut var_guard568_rv: f64 = *var_guard568_rv_slot;
        let mut var_guard571: f64 = *var_guard571_slot;
        let mut var_guard571_db0: f64 = *var_guard571_db0_slot;
        let mut var_guard571_db1: f64 = *var_guard571_db1_slot;
        let mut var_guard571_db2: f64 = *var_guard571_db2_slot;
        let mut var_guard571_db3: f64 = *var_guard571_db3_slot;
        let mut var_guard571_dn0: f64 = *var_guard571_dn0_slot;
        let mut var_guard571_dn1: f64 = *var_guard571_dn1_slot;
        let mut var_guard571_dn2: f64 = *var_guard571_dn2_slot;
        let mut var_guard571_dn3: f64 = *var_guard571_dn3_slot;
        let mut var_guard571_dn4: f64 = *var_guard571_dn4_slot;
        let mut var_guard571_dn5: f64 = *var_guard571_dn5_slot;
        let mut var_guard571_rdb0: f64 = *var_guard571_rdb0_slot;
        let mut var_guard571_rdb1: f64 = *var_guard571_rdb1_slot;
        let mut var_guard571_rdb2: f64 = *var_guard571_rdb2_slot;
        let mut var_guard571_rdb3: f64 = *var_guard571_rdb3_slot;
        let mut var_guard571_rdn0: f64 = *var_guard571_rdn0_slot;
        let mut var_guard571_rdn1: f64 = *var_guard571_rdn1_slot;
        let mut var_guard571_rdn2: f64 = *var_guard571_rdn2_slot;
        let mut var_guard571_rdn3: f64 = *var_guard571_rdn3_slot;
        let mut var_guard571_rdn4: f64 = *var_guard571_rdn4_slot;
        let mut var_guard571_rdn5: f64 = *var_guard571_rdn5_slot;
        let mut var_guard571_rv: f64 = *var_guard571_rv_slot;
        let mut var_guard572: f64 = *var_guard572_slot;
        let mut var_guard572_db0: f64 = *var_guard572_db0_slot;
        let mut var_guard572_db1: f64 = *var_guard572_db1_slot;
        let mut var_guard572_db2: f64 = *var_guard572_db2_slot;
        let mut var_guard572_db3: f64 = *var_guard572_db3_slot;
        let mut var_guard572_dn0: f64 = *var_guard572_dn0_slot;
        let mut var_guard572_dn1: f64 = *var_guard572_dn1_slot;
        let mut var_guard572_dn2: f64 = *var_guard572_dn2_slot;
        let mut var_guard572_dn3: f64 = *var_guard572_dn3_slot;
        let mut var_guard572_dn4: f64 = *var_guard572_dn4_slot;
        let mut var_guard572_dn5: f64 = *var_guard572_dn5_slot;
        let mut var_guard572_rdb0: f64 = *var_guard572_rdb0_slot;
        let mut var_guard572_rdb1: f64 = *var_guard572_rdb1_slot;
        let mut var_guard572_rdb2: f64 = *var_guard572_rdb2_slot;
        let mut var_guard572_rdb3: f64 = *var_guard572_rdb3_slot;
        let mut var_guard572_rdn0: f64 = *var_guard572_rdn0_slot;
        let mut var_guard572_rdn1: f64 = *var_guard572_rdn1_slot;
        let mut var_guard572_rdn2: f64 = *var_guard572_rdn2_slot;
        let mut var_guard572_rdn3: f64 = *var_guard572_rdn3_slot;
        let mut var_guard572_rdn4: f64 = *var_guard572_rdn4_slot;
        let mut var_guard572_rdn5: f64 = *var_guard572_rdn5_slot;
        let mut var_guard572_rv: f64 = *var_guard572_rv_slot;
        let mut var_iwnqs0_a: f64 = *var_iwnqs0_a_slot;
        let mut var_iwnqs0_a_db0: f64 = *var_iwnqs0_a_db0_slot;
        let mut var_iwnqs0_a_db1: f64 = *var_iwnqs0_a_db1_slot;
        let mut var_iwnqs0_a_db2: f64 = *var_iwnqs0_a_db2_slot;
        let mut var_iwnqs0_a_db3: f64 = *var_iwnqs0_a_db3_slot;
        let mut var_iwnqs0_a_dn0: f64 = *var_iwnqs0_a_dn0_slot;
        let mut var_iwnqs0_a_dn1: f64 = *var_iwnqs0_a_dn1_slot;
        let mut var_iwnqs0_a_dn2: f64 = *var_iwnqs0_a_dn2_slot;
        let mut var_iwnqs0_a_dn3: f64 = *var_iwnqs0_a_dn3_slot;
        let mut var_iwnqs0_a_dn4: f64 = *var_iwnqs0_a_dn4_slot;
        let mut var_iwnqs0_a_dn5: f64 = *var_iwnqs0_a_dn5_slot;
        let mut var_iwnqs0_a_rdb0: f64 = *var_iwnqs0_a_rdb0_slot;
        let mut var_iwnqs0_a_rdb1: f64 = *var_iwnqs0_a_rdb1_slot;
        let mut var_iwnqs0_a_rdb2: f64 = *var_iwnqs0_a_rdb2_slot;
        let mut var_iwnqs0_a_rdb3: f64 = *var_iwnqs0_a_rdb3_slot;
        let mut var_iwnqs0_a_rdn0: f64 = *var_iwnqs0_a_rdn0_slot;
        let mut var_iwnqs0_a_rdn1: f64 = *var_iwnqs0_a_rdn1_slot;
        let mut var_iwnqs0_a_rdn2: f64 = *var_iwnqs0_a_rdn2_slot;
        let mut var_iwnqs0_a_rdn3: f64 = *var_iwnqs0_a_rdn3_slot;
        let mut var_iwnqs0_a_rdn4: f64 = *var_iwnqs0_a_rdn4_slot;
        let mut var_iwnqs0_a_rdn5: f64 = *var_iwnqs0_a_rdn5_slot;
        let mut var_iwnqs0_a_rv: f64 = *var_iwnqs0_a_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_rdb0: f64 = *var_tmf2_rdb0_slot;
        let mut var_tmf2_rdb1: f64 = *var_tmf2_rdb1_slot;
        let mut var_tmf2_rdb2: f64 = *var_tmf2_rdb2_slot;
        let mut var_tmf2_rdb3: f64 = *var_tmf2_rdb3_slot;
        let mut var_tmf2_rdn0: f64 = *var_tmf2_rdn0_slot;
        let mut var_tmf2_rdn1: f64 = *var_tmf2_rdn1_slot;
        let mut var_tmf2_rdn2: f64 = *var_tmf2_rdn2_slot;
        let mut var_tmf2_rdn3: f64 = *var_tmf2_rdn3_slot;
        let mut var_tmf2_rdn4: f64 = *var_tmf2_rdn4_slot;
        let mut var_tmf2_rdn5: f64 = *var_tmf2_rdn5_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_w_depa: f64 = *var_w_depa_slot;
        let mut var_w_depa_db0: f64 = *var_w_depa_db0_slot;
        let mut var_w_depa_db1: f64 = *var_w_depa_db1_slot;
        let mut var_w_depa_db2: f64 = *var_w_depa_db2_slot;
        let mut var_w_depa_db3: f64 = *var_w_depa_db3_slot;
        let mut var_w_depa_dn0: f64 = *var_w_depa_dn0_slot;
        let mut var_w_depa_dn1: f64 = *var_w_depa_dn1_slot;
        let mut var_w_depa_dn2: f64 = *var_w_depa_dn2_slot;
        let mut var_w_depa_dn3: f64 = *var_w_depa_dn3_slot;
        let mut var_w_depa_dn4: f64 = *var_w_depa_dn4_slot;
        let mut var_w_depa_dn5: f64 = *var_w_depa_dn5_slot;
        let mut var_w_depa_rdb0: f64 = *var_w_depa_rdb0_slot;
        let mut var_w_depa_rdb1: f64 = *var_w_depa_rdb1_slot;
        let mut var_w_depa_rdb2: f64 = *var_w_depa_rdb2_slot;
        let mut var_w_depa_rdb3: f64 = *var_w_depa_rdb3_slot;
        let mut var_w_depa_rdn0: f64 = *var_w_depa_rdn0_slot;
        let mut var_w_depa_rdn1: f64 = *var_w_depa_rdn1_slot;
        let mut var_w_depa_rdn2: f64 = *var_w_depa_rdn2_slot;
        let mut var_w_depa_rdn3: f64 = *var_w_depa_rdn3_slot;
        let mut var_w_depa_rdn4: f64 = *var_w_depa_rdn4_slot;
        let mut var_w_depa_rdn5: f64 = *var_w_depa_rdn5_slot;
        let mut var_w_depa_rv: f64 = *var_w_depa_rv_slot;
        let mut var_w_nqs_a: f64 = *var_w_nqs_a_slot;
        let mut var_w_nqs_a_db0: f64 = *var_w_nqs_a_db0_slot;
        let mut var_w_nqs_a_db1: f64 = *var_w_nqs_a_db1_slot;
        let mut var_w_nqs_a_db2: f64 = *var_w_nqs_a_db2_slot;
        let mut var_w_nqs_a_db3: f64 = *var_w_nqs_a_db3_slot;
        let mut var_w_nqs_a_dn0: f64 = *var_w_nqs_a_dn0_slot;
        let mut var_w_nqs_a_dn1: f64 = *var_w_nqs_a_dn1_slot;
        let mut var_w_nqs_a_dn2: f64 = *var_w_nqs_a_dn2_slot;
        let mut var_w_nqs_a_dn3: f64 = *var_w_nqs_a_dn3_slot;
        let mut var_w_nqs_a_dn4: f64 = *var_w_nqs_a_dn4_slot;
        let mut var_w_nqs_a_dn5: f64 = *var_w_nqs_a_dn5_slot;
        let mut var_w_nqs_a_rdb0: f64 = *var_w_nqs_a_rdb0_slot;
        let mut var_w_nqs_a_rdb1: f64 = *var_w_nqs_a_rdb1_slot;
        let mut var_w_nqs_a_rdb2: f64 = *var_w_nqs_a_rdb2_slot;
        let mut var_w_nqs_a_rdb3: f64 = *var_w_nqs_a_rdb3_slot;
        let mut var_w_nqs_a_rdn0: f64 = *var_w_nqs_a_rdn0_slot;
        let mut var_w_nqs_a_rdn1: f64 = *var_w_nqs_a_rdn1_slot;
        let mut var_w_nqs_a_rdn2: f64 = *var_w_nqs_a_rdn2_slot;
        let mut var_w_nqs_a_rdn3: f64 = *var_w_nqs_a_rdn3_slot;
        let mut var_w_nqs_a_rdn4: f64 = *var_w_nqs_a_rdn4_slot;
        let mut var_w_nqs_a_rdn5: f64 = *var_w_nqs_a_rdn5_slot;
        let mut var_w_nqs_a_rv: f64 = *var_w_nqs_a_rv_slot;
        let mut var_w_qs_a: f64 = *var_w_qs_a_slot;
        let mut var_w_qs_a_db0: f64 = *var_w_qs_a_db0_slot;
        let mut var_w_qs_a_db1: f64 = *var_w_qs_a_db1_slot;
        let mut var_w_qs_a_db2: f64 = *var_w_qs_a_db2_slot;
        let mut var_w_qs_a_db3: f64 = *var_w_qs_a_db3_slot;
        let mut var_w_qs_a_dn0: f64 = *var_w_qs_a_dn0_slot;
        let mut var_w_qs_a_dn1: f64 = *var_w_qs_a_dn1_slot;
        let mut var_w_qs_a_dn2: f64 = *var_w_qs_a_dn2_slot;
        let mut var_w_qs_a_dn3: f64 = *var_w_qs_a_dn3_slot;
        let mut var_w_qs_a_dn4: f64 = *var_w_qs_a_dn4_slot;
        let mut var_w_qs_a_dn5: f64 = *var_w_qs_a_dn5_slot;
        let mut var_w_qs_a_rdb0: f64 = *var_w_qs_a_rdb0_slot;
        let mut var_w_qs_a_rdb1: f64 = *var_w_qs_a_rdb1_slot;
        let mut var_w_qs_a_rdb2: f64 = *var_w_qs_a_rdb2_slot;
        let mut var_w_qs_a_rdb3: f64 = *var_w_qs_a_rdb3_slot;
        let mut var_w_qs_a_rdn0: f64 = *var_w_qs_a_rdn0_slot;
        let mut var_w_qs_a_rdn1: f64 = *var_w_qs_a_rdn1_slot;
        let mut var_w_qs_a_rdn2: f64 = *var_w_qs_a_rdn2_slot;
        let mut var_w_qs_a_rdn3: f64 = *var_w_qs_a_rdn3_slot;
        let mut var_w_qs_a_rdn4: f64 = *var_w_qs_a_rdn4_slot;
        let mut var_w_qs_a_rdn5: f64 = *var_w_qs_a_rdn5_slot;
        let mut var_w_qs_a_rv: f64 = *var_w_qs_a_rv_slot;

        let (assign34840_e52322, assign34840_e52322_d_n0, assign34840_e52322_d_n1, assign34840_e52322_d_n2, assign34840_e52322_d_n3, assign34840_e52322_d_n4, assign34840_e52322_d_n5, assign34840_e52322_d_b0, assign34840_e52322_d_b1, assign34840_e52322_d_b2, assign34840_e52322_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34840_e52318: f64 = (4.0 * p.p94);
        let assign34840_e52320: f64 = (assign34840_e52318 * 1e-7);
        (assign34840_e52320, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34840_e52322;
        var_tmf2_dn0 = assign34840_e52322_d_n0;
        var_tmf2_dn1 = assign34840_e52322_d_n1;
        var_tmf2_dn2 = assign34840_e52322_d_n2;
        var_tmf2_dn3 = assign34840_e52322_d_n3;
        var_tmf2_dn4 = assign34840_e52322_d_n4;
        var_tmf2_dn5 = assign34840_e52322_d_n5;
        var_tmf2_db0 = assign34840_e52322_d_b0;
        var_tmf2_db1 = assign34840_e52322_d_b1;
        var_tmf2_db2 = assign34840_e52322_d_b2;
        var_tmf2_db3 = assign34840_e52322_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34850_e52332, assign34850_e52332_d_n0, assign34850_e52332_d_n1, assign34850_e52332_d_n2, assign34850_e52332_d_n3, assign34850_e52332_d_n4, assign34850_e52332_d_n5, assign34850_e52332_d_b0, assign34850_e52332_d_b1, assign34850_e52332_d_b2, assign34850_e52332_d_b3,) = {
    if (var_guard558 != 0.0) {
        let (assign34850_e52330, assign34850_e52330_d_n0, assign34850_e52330_d_n1, assign34850_e52330_d_n2, assign34850_e52330_d_n3, assign34850_e52330_d_n4, assign34850_e52330_d_n5, assign34850_e52330_d_b0, assign34850_e52330_d_b1, assign34850_e52330_d_b2, assign34850_e52330_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign34850_e52329: f64 = (-var_tmf2);
                (assign34850_e52329, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign34850_e52330, assign34850_e52330_d_n0, assign34850_e52330_d_n1, assign34850_e52330_d_n2, assign34850_e52330_d_n3, assign34850_e52330_d_n4, assign34850_e52330_d_n5, assign34850_e52330_d_b0, assign34850_e52330_d_b1, assign34850_e52330_d_b2, assign34850_e52330_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34850_e52332;
        var_tmf2_dn0 = assign34850_e52332_d_n0;
        var_tmf2_dn1 = assign34850_e52332_d_n1;
        var_tmf2_dn2 = assign34850_e52332_d_n2;
        var_tmf2_dn3 = assign34850_e52332_d_n3;
        var_tmf2_dn4 = assign34850_e52332_d_n4;
        var_tmf2_dn5 = assign34850_e52332_d_n5;
        var_tmf2_db0 = assign34850_e52332_d_b0;
        var_tmf2_db1 = assign34850_e52332_d_b1;
        var_tmf2_db2 = assign34850_e52332_d_b2;
        var_tmf2_db3 = assign34850_e52332_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34860_e52341, assign34860_e52341_d_n0, assign34860_e52341_d_n1, assign34860_e52341_d_n2, assign34860_e52341_d_n3, assign34860_e52341_d_n4, assign34860_e52341_d_n5, assign34860_e52341_d_b0, assign34860_e52341_d_b1, assign34860_e52341_d_b2, assign34860_e52341_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34860_e52336: f64 = (var_tmf1 * var_tmf1);
        let assign34860_e52338: f64 = (assign34860_e52336 + var_tmf2);
        let assign34860_e52339: f64 = (assign34860_e52338).sqrt();
        (assign34860_e52339, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34860_e52339)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign34860_e52339)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34860_e52339)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign34860_e52339)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign34860_e52339)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign34860_e52339)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign34860_e52339)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign34860_e52339)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign34860_e52339)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign34860_e52339)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34860_e52341;
        var_tmf2_dn0 = assign34860_e52341_d_n0;
        var_tmf2_dn1 = assign34860_e52341_d_n1;
        var_tmf2_dn2 = assign34860_e52341_d_n2;
        var_tmf2_dn3 = assign34860_e52341_d_n3;
        var_tmf2_dn4 = assign34860_e52341_d_n4;
        var_tmf2_dn5 = assign34860_e52341_d_n5;
        var_tmf2_db0 = assign34860_e52341_d_b0;
        var_tmf2_db1 = assign34860_e52341_d_b1;
        var_tmf2_db2 = assign34860_e52341_d_b2;
        var_tmf2_db3 = assign34860_e52341_d_b3;
        var_tmf2_rv = 0.0;
        var_tmf2_rdn0 = 0.0;
        var_tmf2_rdn1 = 0.0;
        var_tmf2_rdn2 = 0.0;
        var_tmf2_rdn3 = 0.0;
        var_tmf2_rdn4 = 0.0;
        var_tmf2_rdn5 = 0.0;
        var_tmf2_rdb0 = 0.0;
        var_tmf2_rdb1 = 0.0;
        var_tmf2_rdb2 = 0.0;
        var_tmf2_rdb3 = 0.0;

        let (assign34870_e52351, assign34870_e52351_d_n0, assign34870_e52351_d_n1, assign34870_e52351_d_n2, assign34870_e52351_d_n3, assign34870_e52351_d_n4, assign34870_e52351_d_n5, assign34870_e52351_d_b0, assign34870_e52351_d_b1, assign34870_e52351_d_b2, assign34870_e52351_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34870_e52347: f64 = (var_tmf1 + var_tmf2);
        let assign34870_e52348: f64 = (0.5 * assign34870_e52347);
        let assign34870_e52349: f64 = (p.p94 - assign34870_e52348);
        (assign34870_e52349, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_w_depa, var_w_depa_dn0, var_w_depa_dn1, var_w_depa_dn2, var_w_depa_dn3, var_w_depa_dn4, var_w_depa_dn5, var_w_depa_db0, var_w_depa_db1, var_w_depa_db2, var_w_depa_db3,)
    }
};
        var_w_depa = assign34870_e52351;
        var_w_depa_dn0 = assign34870_e52351_d_n0;
        var_w_depa_dn1 = assign34870_e52351_d_n1;
        var_w_depa_dn2 = assign34870_e52351_d_n2;
        var_w_depa_dn3 = assign34870_e52351_d_n3;
        var_w_depa_dn4 = assign34870_e52351_d_n4;
        var_w_depa_dn5 = assign34870_e52351_d_n5;
        var_w_depa_db0 = assign34870_e52351_d_b0;
        var_w_depa_db1 = assign34870_e52351_d_b1;
        var_w_depa_db2 = assign34870_e52351_d_b2;
        var_w_depa_db3 = assign34870_e52351_d_b3;
        var_w_depa_rv = 0.0;
        var_w_depa_rdn0 = 0.0;
        var_w_depa_rdn1 = 0.0;
        var_w_depa_rdn2 = 0.0;
        var_w_depa_rdn3 = 0.0;
        var_w_depa_rdn4 = 0.0;
        var_w_depa_rdn5 = 0.0;
        var_w_depa_rdb0 = 0.0;
        var_w_depa_rdb1 = 0.0;
        var_w_depa_rdb2 = 0.0;
        var_w_depa_rdb3 = 0.0;

        let assign34880_e52354: f64 = if p.p95 > 0.0 { 1.0 } else { 0.0 };
        var_guard568 = assign34880_e52354;
        var_guard568_dn0 = 0.0;
        var_guard568_dn1 = 0.0;
        var_guard568_dn2 = 0.0;
        var_guard568_dn3 = 0.0;
        var_guard568_dn4 = 0.0;
        var_guard568_dn5 = 0.0;
        var_guard568_db0 = 0.0;
        var_guard568_db1 = 0.0;
        var_guard568_db2 = 0.0;
        var_guard568_db3 = 0.0;
        var_guard568_rv = 0.0;
        var_guard568_rdn0 = 0.0;
        var_guard568_rdn1 = 0.0;
        var_guard568_rdn2 = 0.0;
        var_guard568_rdn3 = 0.0;
        var_guard568_rdn4 = 0.0;
        var_guard568_rdn5 = 0.0;
        var_guard568_rdb0 = 0.0;
        var_guard568_rdb1 = 0.0;
        var_guard568_rdb2 = 0.0;
        var_guard568_rdb3 = 0.0;

        let (assign34890_e52364, assign34890_e52364_d_n0, assign34890_e52364_d_n1, assign34890_e52364_d_n2, assign34890_e52364_d_n3, assign34890_e52364_d_n4, assign34890_e52364_d_n5, assign34890_e52364_d_b0, assign34890_e52364_d_b1, assign34890_e52364_d_b2, assign34890_e52364_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard568 != 0.0)) {
        let assign34890_e52361: f64 = (1.0 / var_w_depa0);
        let assign34890_e52362: f64 = (var_w_depa * assign34890_e52361);
        (assign34890_e52362, ((var_w_depa_dn0 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn0 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_dn1 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn1 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_dn2 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn2 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_dn3 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn3 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_dn4 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn4 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_dn5 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn5 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_db0 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_db0 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_db1 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_db1 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_db2 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_db2 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_db3 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_db3 / (var_w_depa0 * var_w_depa0))))),)
    } else {
        (var_w_qs_a, var_w_qs_a_dn0, var_w_qs_a_dn1, var_w_qs_a_dn2, var_w_qs_a_dn3, var_w_qs_a_dn4, var_w_qs_a_dn5, var_w_qs_a_db0, var_w_qs_a_db1, var_w_qs_a_db2, var_w_qs_a_db3,)
    }
};
        var_w_qs_a = assign34890_e52364;
        var_w_qs_a_dn0 = assign34890_e52364_d_n0;
        var_w_qs_a_dn1 = assign34890_e52364_d_n1;
        var_w_qs_a_dn2 = assign34890_e52364_d_n2;
        var_w_qs_a_dn3 = assign34890_e52364_d_n3;
        var_w_qs_a_dn4 = assign34890_e52364_d_n4;
        var_w_qs_a_dn5 = assign34890_e52364_d_n5;
        var_w_qs_a_db0 = assign34890_e52364_d_b0;
        var_w_qs_a_db1 = assign34890_e52364_d_b1;
        var_w_qs_a_db2 = assign34890_e52364_d_b2;
        var_w_qs_a_db3 = assign34890_e52364_d_b3;
        var_w_qs_a_rv = 0.0;
        var_w_qs_a_rdn0 = 0.0;
        var_w_qs_a_rdn1 = 0.0;
        var_w_qs_a_rdn2 = 0.0;
        var_w_qs_a_rdn3 = 0.0;
        var_w_qs_a_rdn4 = 0.0;
        var_w_qs_a_rdn5 = 0.0;
        var_w_qs_a_rdb0 = 0.0;
        var_w_qs_a_rdb1 = 0.0;
        var_w_qs_a_rdb2 = 0.0;
        var_w_qs_a_rdb3 = 0.0;

        let (assign34900_e52372, assign34900_e52372_d_n0, assign34900_e52372_d_n1, assign34900_e52372_d_n2, assign34900_e52372_d_n3, assign34900_e52372_d_n4, assign34900_e52372_d_n5, assign34900_e52372_d_b0, assign34900_e52372_d_b1, assign34900_e52372_d_b2, assign34900_e52372_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard568 != 0.0)) {
        let assign34900_e52370: f64 = (nv5 - 0.0);
        (assign34900_e52370, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_w_nqs_a, var_w_nqs_a_dn0, var_w_nqs_a_dn1, var_w_nqs_a_dn2, var_w_nqs_a_dn3, var_w_nqs_a_dn4, var_w_nqs_a_dn5, var_w_nqs_a_db0, var_w_nqs_a_db1, var_w_nqs_a_db2, var_w_nqs_a_db3,)
    }
};
        var_w_nqs_a = assign34900_e52372;
        var_w_nqs_a_dn0 = assign34900_e52372_d_n0;
        var_w_nqs_a_dn1 = assign34900_e52372_d_n1;
        var_w_nqs_a_dn2 = assign34900_e52372_d_n2;
        var_w_nqs_a_dn3 = assign34900_e52372_d_n3;
        var_w_nqs_a_dn4 = assign34900_e52372_d_n4;
        var_w_nqs_a_dn5 = assign34900_e52372_d_n5;
        var_w_nqs_a_db0 = assign34900_e52372_d_b0;
        var_w_nqs_a_db1 = assign34900_e52372_d_b1;
        var_w_nqs_a_db2 = assign34900_e52372_d_b2;
        var_w_nqs_a_db3 = assign34900_e52372_d_b3;
        var_w_nqs_a_rv = 0.0;
        var_w_nqs_a_rdn0 = 0.0;
        var_w_nqs_a_rdn1 = 0.0;
        var_w_nqs_a_rdn2 = 0.0;
        var_w_nqs_a_rdn3 = 0.0;
        var_w_nqs_a_rdn4 = 0.0;
        var_w_nqs_a_rdn5 = 0.0;
        var_w_nqs_a_rdb0 = 0.0;
        var_w_nqs_a_rdb1 = 0.0;
        var_w_nqs_a_rdb2 = 0.0;
        var_w_nqs_a_rdb3 = 0.0;

        let (assign34910_e52382, assign34910_e52382_d_n0, assign34910_e52382_d_n1, assign34910_e52382_d_n2, assign34910_e52382_d_n3, assign34910_e52382_d_n4, assign34910_e52382_d_n5, assign34910_e52382_d_b0, assign34910_e52382_d_b1, assign34910_e52382_d_b2, assign34910_e52382_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard568 != 0.0)) {
        let assign34910_e52378: f64 = (var_w_nqs_a - var_w_qs_a);
        let assign34910_e52380: f64 = (assign34910_e52378 / p.p95);
        (assign34910_e52380, ((var_w_nqs_a_dn0 - var_w_qs_a_dn0) / p.p95), ((var_w_nqs_a_dn1 - var_w_qs_a_dn1) / p.p95), ((var_w_nqs_a_dn2 - var_w_qs_a_dn2) / p.p95), ((var_w_nqs_a_dn3 - var_w_qs_a_dn3) / p.p95), ((var_w_nqs_a_dn4 - var_w_qs_a_dn4) / p.p95), ((var_w_nqs_a_dn5 - var_w_qs_a_dn5) / p.p95), ((var_w_nqs_a_db0 - var_w_qs_a_db0) / p.p95), ((var_w_nqs_a_db1 - var_w_qs_a_db1) / p.p95), ((var_w_nqs_a_db2 - var_w_qs_a_db2) / p.p95), ((var_w_nqs_a_db3 - var_w_qs_a_db3) / p.p95),)
    } else {
        (var_iwnqs0_a, var_iwnqs0_a_dn0, var_iwnqs0_a_dn1, var_iwnqs0_a_dn2, var_iwnqs0_a_dn3, var_iwnqs0_a_dn4, var_iwnqs0_a_dn5, var_iwnqs0_a_db0, var_iwnqs0_a_db1, var_iwnqs0_a_db2, var_iwnqs0_a_db3,)
    }
};
        var_iwnqs0_a = assign34910_e52382;
        var_iwnqs0_a_dn0 = assign34910_e52382_d_n0;
        var_iwnqs0_a_dn1 = assign34910_e52382_d_n1;
        var_iwnqs0_a_dn2 = assign34910_e52382_d_n2;
        var_iwnqs0_a_dn3 = assign34910_e52382_d_n3;
        var_iwnqs0_a_dn4 = assign34910_e52382_d_n4;
        var_iwnqs0_a_dn5 = assign34910_e52382_d_n5;
        var_iwnqs0_a_db0 = assign34910_e52382_d_b0;
        var_iwnqs0_a_db1 = assign34910_e52382_d_b1;
        var_iwnqs0_a_db2 = assign34910_e52382_d_b2;
        var_iwnqs0_a_db3 = assign34910_e52382_d_b3;
        var_iwnqs0_a_rv = 0.0;
        var_iwnqs0_a_rdn0 = 0.0;
        var_iwnqs0_a_rdn1 = 0.0;
        var_iwnqs0_a_rdn2 = 0.0;
        var_iwnqs0_a_rdn3 = 0.0;
        var_iwnqs0_a_rdn4 = 0.0;
        var_iwnqs0_a_rdn5 = 0.0;
        var_iwnqs0_a_rdb0 = 0.0;
        var_iwnqs0_a_rdb1 = 0.0;
        var_iwnqs0_a_rdb2 = 0.0;
        var_iwnqs0_a_rdb3 = 0.0;

        let (assign34930_e52399, assign34930_e52399_d_n0, assign34930_e52399_d_n1, assign34930_e52399_d_n2, assign34930_e52399_d_n3, assign34930_e52399_d_n4, assign34930_e52399_d_n5, assign34930_e52399_d_b0, assign34930_e52399_d_b1, assign34930_e52399_d_b2, assign34930_e52399_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard568 == 0.0)) {
        (var_w_depa, var_w_depa_dn0, var_w_depa_dn1, var_w_depa_dn2, var_w_depa_dn3, var_w_depa_dn4, var_w_depa_dn5, var_w_depa_db0, var_w_depa_db1, var_w_depa_db2, var_w_depa_db3,)
    } else {
        (var_w_qs_a, var_w_qs_a_dn0, var_w_qs_a_dn1, var_w_qs_a_dn2, var_w_qs_a_dn3, var_w_qs_a_dn4, var_w_qs_a_dn5, var_w_qs_a_db0, var_w_qs_a_db1, var_w_qs_a_db2, var_w_qs_a_db3,)
    }
};
        var_w_qs_a = assign34930_e52399;
        var_w_qs_a_dn0 = assign34930_e52399_d_n0;
        var_w_qs_a_dn1 = assign34930_e52399_d_n1;
        var_w_qs_a_dn2 = assign34930_e52399_d_n2;
        var_w_qs_a_dn3 = assign34930_e52399_d_n3;
        var_w_qs_a_dn4 = assign34930_e52399_d_n4;
        var_w_qs_a_dn5 = assign34930_e52399_d_n5;
        var_w_qs_a_db0 = assign34930_e52399_d_b0;
        var_w_qs_a_db1 = assign34930_e52399_d_b1;
        var_w_qs_a_db2 = assign34930_e52399_d_b2;
        var_w_qs_a_db3 = assign34930_e52399_d_b3;
        var_w_qs_a_rv = 0.0;
        var_w_qs_a_rdn0 = 0.0;
        var_w_qs_a_rdn1 = 0.0;
        var_w_qs_a_rdn2 = 0.0;
        var_w_qs_a_rdn3 = 0.0;
        var_w_qs_a_rdn4 = 0.0;
        var_w_qs_a_rdn5 = 0.0;
        var_w_qs_a_rdb0 = 0.0;
        var_w_qs_a_rdb1 = 0.0;
        var_w_qs_a_rdb2 = 0.0;
        var_w_qs_a_rdb3 = 0.0;

        let assign35080_e52535: f64 = if ((p.p84 > 0.0) && (p.p92 > 0.0)) { 1.0 } else { 0.0 };
        var_guard571 = assign35080_e52535;
        var_guard571_dn0 = 0.0;
        var_guard571_dn1 = 0.0;
        var_guard571_dn2 = 0.0;
        var_guard571_dn3 = 0.0;
        var_guard571_dn4 = 0.0;
        var_guard571_dn5 = 0.0;
        var_guard571_db0 = 0.0;
        var_guard571_db1 = 0.0;
        var_guard571_db2 = 0.0;
        var_guard571_db3 = 0.0;
        var_guard571_rv = 0.0;
        var_guard571_rdn0 = 0.0;
        var_guard571_rdn1 = 0.0;
        var_guard571_rdn2 = 0.0;
        var_guard571_rdn3 = 0.0;
        var_guard571_rdn4 = 0.0;
        var_guard571_rdn5 = 0.0;
        var_guard571_rdb0 = 0.0;
        var_guard571_rdb1 = 0.0;
        var_guard571_rdb2 = 0.0;
        var_guard571_rdb3 = 0.0;

        let assign35090_e52542: f64 = if ((p.p84 > 0.0) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };
        var_guard572 = assign35090_e52542;
        var_guard572_dn0 = 0.0;
        var_guard572_dn1 = 0.0;
        var_guard572_dn2 = 0.0;
        var_guard572_dn3 = 0.0;
        var_guard572_dn4 = 0.0;
        var_guard572_dn5 = 0.0;
        var_guard572_db0 = 0.0;
        var_guard572_db1 = 0.0;
        var_guard572_db2 = 0.0;
        var_guard572_db3 = 0.0;
        var_guard572_rv = 0.0;
        var_guard572_rdn0 = 0.0;
        var_guard572_rdn1 = 0.0;
        var_guard572_rdn2 = 0.0;
        var_guard572_rdn3 = 0.0;
        var_guard572_rdn4 = 0.0;
        var_guard572_rdn5 = 0.0;
        var_guard572_rdb0 = 0.0;
        var_guard572_rdb1 = 0.0;
        var_guard572_rdb2 = 0.0;
        var_guard572_rdb3 = 0.0;

        *var_guard568_slot = var_guard568;
        *var_guard568_db0_slot = var_guard568_db0;
        *var_guard568_db1_slot = var_guard568_db1;
        *var_guard568_db2_slot = var_guard568_db2;
        *var_guard568_db3_slot = var_guard568_db3;
        *var_guard568_dn0_slot = var_guard568_dn0;
        *var_guard568_dn1_slot = var_guard568_dn1;
        *var_guard568_dn2_slot = var_guard568_dn2;
        *var_guard568_dn3_slot = var_guard568_dn3;
        *var_guard568_dn4_slot = var_guard568_dn4;
        *var_guard568_dn5_slot = var_guard568_dn5;
        *var_guard568_rdb0_slot = var_guard568_rdb0;
        *var_guard568_rdb1_slot = var_guard568_rdb1;
        *var_guard568_rdb2_slot = var_guard568_rdb2;
        *var_guard568_rdb3_slot = var_guard568_rdb3;
        *var_guard568_rdn0_slot = var_guard568_rdn0;
        *var_guard568_rdn1_slot = var_guard568_rdn1;
        *var_guard568_rdn2_slot = var_guard568_rdn2;
        *var_guard568_rdn3_slot = var_guard568_rdn3;
        *var_guard568_rdn4_slot = var_guard568_rdn4;
        *var_guard568_rdn5_slot = var_guard568_rdn5;
        *var_guard568_rv_slot = var_guard568_rv;
        *var_guard571_slot = var_guard571;
        *var_guard571_db0_slot = var_guard571_db0;
        *var_guard571_db1_slot = var_guard571_db1;
        *var_guard571_db2_slot = var_guard571_db2;
        *var_guard571_db3_slot = var_guard571_db3;
        *var_guard571_dn0_slot = var_guard571_dn0;
        *var_guard571_dn1_slot = var_guard571_dn1;
        *var_guard571_dn2_slot = var_guard571_dn2;
        *var_guard571_dn3_slot = var_guard571_dn3;
        *var_guard571_dn4_slot = var_guard571_dn4;
        *var_guard571_dn5_slot = var_guard571_dn5;
        *var_guard571_rdb0_slot = var_guard571_rdb0;
        *var_guard571_rdb1_slot = var_guard571_rdb1;
        *var_guard571_rdb2_slot = var_guard571_rdb2;
        *var_guard571_rdb3_slot = var_guard571_rdb3;
        *var_guard571_rdn0_slot = var_guard571_rdn0;
        *var_guard571_rdn1_slot = var_guard571_rdn1;
        *var_guard571_rdn2_slot = var_guard571_rdn2;
        *var_guard571_rdn3_slot = var_guard571_rdn3;
        *var_guard571_rdn4_slot = var_guard571_rdn4;
        *var_guard571_rdn5_slot = var_guard571_rdn5;
        *var_guard571_rv_slot = var_guard571_rv;
        *var_guard572_slot = var_guard572;
        *var_guard572_db0_slot = var_guard572_db0;
        *var_guard572_db1_slot = var_guard572_db1;
        *var_guard572_db2_slot = var_guard572_db2;
        *var_guard572_db3_slot = var_guard572_db3;
        *var_guard572_dn0_slot = var_guard572_dn0;
        *var_guard572_dn1_slot = var_guard572_dn1;
        *var_guard572_dn2_slot = var_guard572_dn2;
        *var_guard572_dn3_slot = var_guard572_dn3;
        *var_guard572_dn4_slot = var_guard572_dn4;
        *var_guard572_dn5_slot = var_guard572_dn5;
        *var_guard572_rdb0_slot = var_guard572_rdb0;
        *var_guard572_rdb1_slot = var_guard572_rdb1;
        *var_guard572_rdb2_slot = var_guard572_rdb2;
        *var_guard572_rdb3_slot = var_guard572_rdb3;
        *var_guard572_rdn0_slot = var_guard572_rdn0;
        *var_guard572_rdn1_slot = var_guard572_rdn1;
        *var_guard572_rdn2_slot = var_guard572_rdn2;
        *var_guard572_rdn3_slot = var_guard572_rdn3;
        *var_guard572_rdn4_slot = var_guard572_rdn4;
        *var_guard572_rdn5_slot = var_guard572_rdn5;
        *var_guard572_rv_slot = var_guard572_rv;
        *var_iwnqs0_a_slot = var_iwnqs0_a;
        *var_iwnqs0_a_db0_slot = var_iwnqs0_a_db0;
        *var_iwnqs0_a_db1_slot = var_iwnqs0_a_db1;
        *var_iwnqs0_a_db2_slot = var_iwnqs0_a_db2;
        *var_iwnqs0_a_db3_slot = var_iwnqs0_a_db3;
        *var_iwnqs0_a_dn0_slot = var_iwnqs0_a_dn0;
        *var_iwnqs0_a_dn1_slot = var_iwnqs0_a_dn1;
        *var_iwnqs0_a_dn2_slot = var_iwnqs0_a_dn2;
        *var_iwnqs0_a_dn3_slot = var_iwnqs0_a_dn3;
        *var_iwnqs0_a_dn4_slot = var_iwnqs0_a_dn4;
        *var_iwnqs0_a_dn5_slot = var_iwnqs0_a_dn5;
        *var_iwnqs0_a_rdb0_slot = var_iwnqs0_a_rdb0;
        *var_iwnqs0_a_rdb1_slot = var_iwnqs0_a_rdb1;
        *var_iwnqs0_a_rdb2_slot = var_iwnqs0_a_rdb2;
        *var_iwnqs0_a_rdb3_slot = var_iwnqs0_a_rdb3;
        *var_iwnqs0_a_rdn0_slot = var_iwnqs0_a_rdn0;
        *var_iwnqs0_a_rdn1_slot = var_iwnqs0_a_rdn1;
        *var_iwnqs0_a_rdn2_slot = var_iwnqs0_a_rdn2;
        *var_iwnqs0_a_rdn3_slot = var_iwnqs0_a_rdn3;
        *var_iwnqs0_a_rdn4_slot = var_iwnqs0_a_rdn4;
        *var_iwnqs0_a_rdn5_slot = var_iwnqs0_a_rdn5;
        *var_iwnqs0_a_rv_slot = var_iwnqs0_a_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_rdb0_slot = var_tmf2_rdb0;
        *var_tmf2_rdb1_slot = var_tmf2_rdb1;
        *var_tmf2_rdb2_slot = var_tmf2_rdb2;
        *var_tmf2_rdb3_slot = var_tmf2_rdb3;
        *var_tmf2_rdn0_slot = var_tmf2_rdn0;
        *var_tmf2_rdn1_slot = var_tmf2_rdn1;
        *var_tmf2_rdn2_slot = var_tmf2_rdn2;
        *var_tmf2_rdn3_slot = var_tmf2_rdn3;
        *var_tmf2_rdn4_slot = var_tmf2_rdn4;
        *var_tmf2_rdn5_slot = var_tmf2_rdn5;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_w_depa_slot = var_w_depa;
        *var_w_depa_db0_slot = var_w_depa_db0;
        *var_w_depa_db1_slot = var_w_depa_db1;
        *var_w_depa_db2_slot = var_w_depa_db2;
        *var_w_depa_db3_slot = var_w_depa_db3;
        *var_w_depa_dn0_slot = var_w_depa_dn0;
        *var_w_depa_dn1_slot = var_w_depa_dn1;
        *var_w_depa_dn2_slot = var_w_depa_dn2;
        *var_w_depa_dn3_slot = var_w_depa_dn3;
        *var_w_depa_dn4_slot = var_w_depa_dn4;
        *var_w_depa_dn5_slot = var_w_depa_dn5;
        *var_w_depa_rdb0_slot = var_w_depa_rdb0;
        *var_w_depa_rdb1_slot = var_w_depa_rdb1;
        *var_w_depa_rdb2_slot = var_w_depa_rdb2;
        *var_w_depa_rdb3_slot = var_w_depa_rdb3;
        *var_w_depa_rdn0_slot = var_w_depa_rdn0;
        *var_w_depa_rdn1_slot = var_w_depa_rdn1;
        *var_w_depa_rdn2_slot = var_w_depa_rdn2;
        *var_w_depa_rdn3_slot = var_w_depa_rdn3;
        *var_w_depa_rdn4_slot = var_w_depa_rdn4;
        *var_w_depa_rdn5_slot = var_w_depa_rdn5;
        *var_w_depa_rv_slot = var_w_depa_rv;
        *var_w_nqs_a_slot = var_w_nqs_a;
        *var_w_nqs_a_db0_slot = var_w_nqs_a_db0;
        *var_w_nqs_a_db1_slot = var_w_nqs_a_db1;
        *var_w_nqs_a_db2_slot = var_w_nqs_a_db2;
        *var_w_nqs_a_db3_slot = var_w_nqs_a_db3;
        *var_w_nqs_a_dn0_slot = var_w_nqs_a_dn0;
        *var_w_nqs_a_dn1_slot = var_w_nqs_a_dn1;
        *var_w_nqs_a_dn2_slot = var_w_nqs_a_dn2;
        *var_w_nqs_a_dn3_slot = var_w_nqs_a_dn3;
        *var_w_nqs_a_dn4_slot = var_w_nqs_a_dn4;
        *var_w_nqs_a_dn5_slot = var_w_nqs_a_dn5;
        *var_w_nqs_a_rdb0_slot = var_w_nqs_a_rdb0;
        *var_w_nqs_a_rdb1_slot = var_w_nqs_a_rdb1;
        *var_w_nqs_a_rdb2_slot = var_w_nqs_a_rdb2;
        *var_w_nqs_a_rdb3_slot = var_w_nqs_a_rdb3;
        *var_w_nqs_a_rdn0_slot = var_w_nqs_a_rdn0;
        *var_w_nqs_a_rdn1_slot = var_w_nqs_a_rdn1;
        *var_w_nqs_a_rdn2_slot = var_w_nqs_a_rdn2;
        *var_w_nqs_a_rdn3_slot = var_w_nqs_a_rdn3;
        *var_w_nqs_a_rdn4_slot = var_w_nqs_a_rdn4;
        *var_w_nqs_a_rdn5_slot = var_w_nqs_a_rdn5;
        *var_w_nqs_a_rv_slot = var_w_nqs_a_rv;
        *var_w_qs_a_slot = var_w_qs_a;
        *var_w_qs_a_db0_slot = var_w_qs_a_db0;
        *var_w_qs_a_db1_slot = var_w_qs_a_db1;
        *var_w_qs_a_db2_slot = var_w_qs_a_db2;
        *var_w_qs_a_db3_slot = var_w_qs_a_db3;
        *var_w_qs_a_dn0_slot = var_w_qs_a_dn0;
        *var_w_qs_a_dn1_slot = var_w_qs_a_dn1;
        *var_w_qs_a_dn2_slot = var_w_qs_a_dn2;
        *var_w_qs_a_dn3_slot = var_w_qs_a_dn3;
        *var_w_qs_a_dn4_slot = var_w_qs_a_dn4;
        *var_w_qs_a_dn5_slot = var_w_qs_a_dn5;
        *var_w_qs_a_rdb0_slot = var_w_qs_a_rdb0;
        *var_w_qs_a_rdb1_slot = var_w_qs_a_rdb1;
        *var_w_qs_a_rdb2_slot = var_w_qs_a_rdb2;
        *var_w_qs_a_rdb3_slot = var_w_qs_a_rdb3;
        *var_w_qs_a_rdn0_slot = var_w_qs_a_rdn0;
        *var_w_qs_a_rdn1_slot = var_w_qs_a_rdn1;
        *var_w_qs_a_rdn2_slot = var_w_qs_a_rdn2;
        *var_w_qs_a_rdn3_slot = var_w_qs_a_rdn3;
        *var_w_qs_a_rdn4_slot = var_w_qs_a_rdn4;
        *var_w_qs_a_rdn5_slot = var_w_qs_a_rdn5;
        *var_w_qs_a_rv_slot = var_w_qs_a_rv;
    }

    pub(super) fn stamp_transient_equations_block_0(
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
        var_guard571: f64,
        var_guard572: f64,
        var_inqs0_a: f64,
        var_inqs0_a_dn0: f64,
        var_inqs0_a_dn2: f64,
        var_inqs0_a_dn3: f64,
        var_inqs0_k: f64,
        var_inqs0_k_dn0: f64,
        var_inqs0_k_dn2: f64,
        var_inqs0_k_dn4: f64,
        var_iwnqs0_a: f64,
        var_iwnqs0_a_dn0: f64,
        var_iwnqs0_a_dn2: f64,
        var_iwnqs0_a_dn5: f64,
        var_q_nqs_a: f64,
        var_q_nqs_a_dn3: f64,
        var_q_nqs_k: f64,
        var_q_nqs_k_dn4: f64,
        var_w_nqs_a: f64,
        var_w_nqs_a_dn5: f64,
    ) {
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n2, eq7_e144_d_n3,) = {
    if (var_guard571 != 0.0) {
        let eq7_e140: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, var_q_nqs_a);
        let eq7_e141: f64 = (var_inqs0_a + eq7_e140);
        let eq7_e141_d_n3: f64 = (var_inqs0_a_dn3 + (var_q_nqs_a_dn3 * ddt_scale));
        let eq7_e142: f64 = (1e-12 * eq7_e141);
        let eq7_e142_d_n0: f64 = (1e-12 * var_inqs0_a_dn0);
        let eq7_e142_d_n2: f64 = (1e-12 * var_inqs0_a_dn2);
        let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n2, eq7_e142_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e144;
        stamper.stamp_current_node3_local(
            Some(3),
            None,
            multiplicity * (eq7_value),
            0,
            multiplicity * (eq7_e144_d_n0),
            2,
            multiplicity * (eq7_e144_d_n2),
            3,
            multiplicity * (eq7_e144_d_n3),
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n2, eq8_e153_d_n4,) = {
    if (var_guard571 != 0.0) {
        let eq8_e149: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, var_q_nqs_k);
        let eq8_e150: f64 = (var_inqs0_k + eq8_e149);
        let eq8_e150_d_n4: f64 = (var_inqs0_k_dn4 + (var_q_nqs_k_dn4 * ddt_scale));
        let eq8_e151: f64 = (1e-12 * eq8_e150);
        let eq8_e151_d_n0: f64 = (1e-12 * var_inqs0_k_dn0);
        let eq8_e151_d_n2: f64 = (1e-12 * var_inqs0_k_dn2);
        let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n2, eq8_e151_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e153;
        stamper.stamp_current_node3_local(
            Some(4),
            None,
            multiplicity * (eq8_value),
            0,
            multiplicity * (eq8_e153_d_n0),
            2,
            multiplicity * (eq8_e153_d_n2),
            4,
            multiplicity * (eq8_e153_d_n4),
        );
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n2, eq11_e172_d_n5,) = {
    if (var_guard572 != 0.0) {
        let eq11_e168: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_w_nqs_a);
        let eq11_e169: f64 = (var_iwnqs0_a + eq11_e168);
        let eq11_e169_d_n5: f64 = (var_iwnqs0_a_dn5 + (var_w_nqs_a_dn5 * ddt_scale));
        let eq11_e170: f64 = (1e-13 * eq11_e169);
        let eq11_e170_d_n0: f64 = (1e-13 * var_iwnqs0_a_dn0);
        let eq11_e170_d_n2: f64 = (1e-13 * var_iwnqs0_a_dn2);
        let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n2, eq11_e170_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e172;
        stamper.stamp_current_node3_local(
            Some(5),
            None,
            multiplicity * (eq11_value),
            0,
            multiplicity * (eq11_e172_d_n0),
            2,
            multiplicity * (eq11_e172_d_n2),
            5,
            multiplicity * (eq11_e172_d_n5),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard571: f64,
        var_guard572: f64,
        var_inqs0_a: f64,
        var_inqs0_a_db0: f64,
        var_inqs0_a_db1: f64,
        var_inqs0_a_db2: f64,
        var_inqs0_a_db3: f64,
        var_inqs0_a_dn0: f64,
        var_inqs0_a_dn1: f64,
        var_inqs0_a_dn2: f64,
        var_inqs0_a_dn3: f64,
        var_inqs0_a_dn4: f64,
        var_inqs0_a_dn5: f64,
        var_inqs0_k: f64,
        var_inqs0_k_db0: f64,
        var_inqs0_k_db1: f64,
        var_inqs0_k_db2: f64,
        var_inqs0_k_db3: f64,
        var_inqs0_k_dn0: f64,
        var_inqs0_k_dn1: f64,
        var_inqs0_k_dn2: f64,
        var_inqs0_k_dn3: f64,
        var_inqs0_k_dn4: f64,
        var_inqs0_k_dn5: f64,
        var_iwnqs0_a: f64,
        var_iwnqs0_a_db0: f64,
        var_iwnqs0_a_db1: f64,
        var_iwnqs0_a_db2: f64,
        var_iwnqs0_a_db3: f64,
        var_iwnqs0_a_dn0: f64,
        var_iwnqs0_a_dn1: f64,
        var_iwnqs0_a_dn2: f64,
        var_iwnqs0_a_dn3: f64,
        var_iwnqs0_a_dn4: f64,
        var_iwnqs0_a_dn5: f64,
        var_q_nqs_a: f64,
        var_q_nqs_a_db0: f64,
        var_q_nqs_a_db1: f64,
        var_q_nqs_a_db2: f64,
        var_q_nqs_a_db3: f64,
        var_q_nqs_a_dn0: f64,
        var_q_nqs_a_dn1: f64,
        var_q_nqs_a_dn2: f64,
        var_q_nqs_a_dn3: f64,
        var_q_nqs_a_dn4: f64,
        var_q_nqs_a_dn5: f64,
        var_q_nqs_k: f64,
        var_q_nqs_k_db0: f64,
        var_q_nqs_k_db1: f64,
        var_q_nqs_k_db2: f64,
        var_q_nqs_k_db3: f64,
        var_q_nqs_k_dn0: f64,
        var_q_nqs_k_dn1: f64,
        var_q_nqs_k_dn2: f64,
        var_q_nqs_k_dn3: f64,
        var_q_nqs_k_dn4: f64,
        var_q_nqs_k_dn5: f64,
        var_w_nqs_a: f64,
        var_w_nqs_a_db0: f64,
        var_w_nqs_a_db1: f64,
        var_w_nqs_a_db2: f64,
        var_w_nqs_a_db3: f64,
        var_w_nqs_a_dn0: f64,
        var_w_nqs_a_dn1: f64,
        var_w_nqs_a_dn2: f64,
        var_w_nqs_a_dn3: f64,
        var_w_nqs_a_dn4: f64,
        var_w_nqs_a_dn5: f64,
    ) {
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5, eq7_e144_d_b0, eq7_e144_d_b1, eq7_e144_d_b2, eq7_e144_d_b3, eq7_e144_q, eq7_e144_q_d_n0, eq7_e144_q_d_n1, eq7_e144_q_d_n2, eq7_e144_q_d_n3, eq7_e144_q_d_n4, eq7_e144_q_d_n5, eq7_e144_q_d_b0, eq7_e144_q_d_b1, eq7_e144_q_d_b2, eq7_e144_q_d_b3,) = {
    if (var_guard571 != 0.0) {
        let eq7_e140_q: f64 = var_q_nqs_a;
        let eq7_e141: f64 = (var_inqs0_a + var_q_nqs_a);
        let eq7_e141_d_n0: f64 = (var_inqs0_a_dn0 + var_q_nqs_a_dn0);
        let eq7_e141_d_n1: f64 = (var_inqs0_a_dn1 + var_q_nqs_a_dn1);
        let eq7_e141_d_n2: f64 = (var_inqs0_a_dn2 + var_q_nqs_a_dn2);
        let eq7_e141_d_n3: f64 = (var_inqs0_a_dn3 + var_q_nqs_a_dn3);
        let eq7_e141_d_n4: f64 = (var_inqs0_a_dn4 + var_q_nqs_a_dn4);
        let eq7_e141_d_n5: f64 = (var_inqs0_a_dn5 + var_q_nqs_a_dn5);
        let eq7_e141_d_b0: f64 = (var_inqs0_a_db0 + var_q_nqs_a_db0);
        let eq7_e141_d_b1: f64 = (var_inqs0_a_db1 + var_q_nqs_a_db1);
        let eq7_e141_d_b2: f64 = (var_inqs0_a_db2 + var_q_nqs_a_db2);
        let eq7_e141_d_b3: f64 = (var_inqs0_a_db3 + var_q_nqs_a_db3);
        let eq7_e141_q: f64 = eq7_e140_q;
        let eq7_e142: f64 = (1e-12 * eq7_e141);
        let eq7_e142_d_n0: f64 = (1e-12 * eq7_e141_d_n0);
        let eq7_e142_d_n1: f64 = (1e-12 * eq7_e141_d_n1);
        let eq7_e142_d_n2: f64 = (1e-12 * eq7_e141_d_n2);
        let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);
        let eq7_e142_d_n4: f64 = (1e-12 * eq7_e141_d_n4);
        let eq7_e142_d_n5: f64 = (1e-12 * eq7_e141_d_n5);
        let eq7_e142_d_b0: f64 = (1e-12 * eq7_e141_d_b0);
        let eq7_e142_d_b1: f64 = (1e-12 * eq7_e141_d_b1);
        let eq7_e142_d_b2: f64 = (1e-12 * eq7_e141_d_b2);
        let eq7_e142_d_b3: f64 = (1e-12 * eq7_e141_d_b3);
        let eq7_e142_q: f64 = (1e-12 * eq7_e141_q);
        let eq7_e142_q_d_n0: f64 = (1e-12 * var_q_nqs_a_dn0);
        let eq7_e142_q_d_n1: f64 = (1e-12 * var_q_nqs_a_dn1);
        let eq7_e142_q_d_n2: f64 = (1e-12 * var_q_nqs_a_dn2);
        let eq7_e142_q_d_n3: f64 = (1e-12 * var_q_nqs_a_dn3);
        let eq7_e142_q_d_n4: f64 = (1e-12 * var_q_nqs_a_dn4);
        let eq7_e142_q_d_n5: f64 = (1e-12 * var_q_nqs_a_dn5);
        let eq7_e142_q_d_b0: f64 = (1e-12 * var_q_nqs_a_db0);
        let eq7_e142_q_d_b1: f64 = (1e-12 * var_q_nqs_a_db1);
        let eq7_e142_q_d_b2: f64 = (1e-12 * var_q_nqs_a_db2);
        let eq7_e142_q_d_b3: f64 = (1e-12 * var_q_nqs_a_db3);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n1, eq7_e142_d_n2, eq7_e142_d_n3, eq7_e142_d_n4, eq7_e142_d_n5, eq7_e142_d_b0, eq7_e142_d_b1, eq7_e142_d_b2, eq7_e142_d_b3, eq7_e142_q, eq7_e142_q_d_n0, eq7_e142_q_d_n1, eq7_e142_q_d_n2, eq7_e142_q_d_n3, eq7_e142_q_d_n4, eq7_e142_q_d_n5, eq7_e142_q_d_b0, eq7_e142_q_d_b1, eq7_e142_q_d_b2, eq7_e142_q_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 6] = [eq7_e144_q_d_n0, eq7_e144_q_d_n1, eq7_e144_q_d_n2, eq7_e144_q_d_n3, eq7_e144_q_d_n4, eq7_e144_q_d_n5];
        let eq7_reactive_branch_derivatives: [f64; 4] = [eq7_e144_q_d_b0, eq7_e144_q_d_b1, eq7_e144_q_d_b2, eq7_e144_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            None,
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5, eq8_e153_d_b0, eq8_e153_d_b1, eq8_e153_d_b2, eq8_e153_d_b3, eq8_e153_q, eq8_e153_q_d_n0, eq8_e153_q_d_n1, eq8_e153_q_d_n2, eq8_e153_q_d_n3, eq8_e153_q_d_n4, eq8_e153_q_d_n5, eq8_e153_q_d_b0, eq8_e153_q_d_b1, eq8_e153_q_d_b2, eq8_e153_q_d_b3,) = {
    if (var_guard571 != 0.0) {
        let eq8_e149_q: f64 = var_q_nqs_k;
        let eq8_e150: f64 = (var_inqs0_k + var_q_nqs_k);
        let eq8_e150_d_n0: f64 = (var_inqs0_k_dn0 + var_q_nqs_k_dn0);
        let eq8_e150_d_n1: f64 = (var_inqs0_k_dn1 + var_q_nqs_k_dn1);
        let eq8_e150_d_n2: f64 = (var_inqs0_k_dn2 + var_q_nqs_k_dn2);
        let eq8_e150_d_n3: f64 = (var_inqs0_k_dn3 + var_q_nqs_k_dn3);
        let eq8_e150_d_n4: f64 = (var_inqs0_k_dn4 + var_q_nqs_k_dn4);
        let eq8_e150_d_n5: f64 = (var_inqs0_k_dn5 + var_q_nqs_k_dn5);
        let eq8_e150_d_b0: f64 = (var_inqs0_k_db0 + var_q_nqs_k_db0);
        let eq8_e150_d_b1: f64 = (var_inqs0_k_db1 + var_q_nqs_k_db1);
        let eq8_e150_d_b2: f64 = (var_inqs0_k_db2 + var_q_nqs_k_db2);
        let eq8_e150_d_b3: f64 = (var_inqs0_k_db3 + var_q_nqs_k_db3);
        let eq8_e150_q: f64 = eq8_e149_q;
        let eq8_e151: f64 = (1e-12 * eq8_e150);
        let eq8_e151_d_n0: f64 = (1e-12 * eq8_e150_d_n0);
        let eq8_e151_d_n1: f64 = (1e-12 * eq8_e150_d_n1);
        let eq8_e151_d_n2: f64 = (1e-12 * eq8_e150_d_n2);
        let eq8_e151_d_n3: f64 = (1e-12 * eq8_e150_d_n3);
        let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);
        let eq8_e151_d_n5: f64 = (1e-12 * eq8_e150_d_n5);
        let eq8_e151_d_b0: f64 = (1e-12 * eq8_e150_d_b0);
        let eq8_e151_d_b1: f64 = (1e-12 * eq8_e150_d_b1);
        let eq8_e151_d_b2: f64 = (1e-12 * eq8_e150_d_b2);
        let eq8_e151_d_b3: f64 = (1e-12 * eq8_e150_d_b3);
        let eq8_e151_q: f64 = (1e-12 * eq8_e150_q);
        let eq8_e151_q_d_n0: f64 = (1e-12 * var_q_nqs_k_dn0);
        let eq8_e151_q_d_n1: f64 = (1e-12 * var_q_nqs_k_dn1);
        let eq8_e151_q_d_n2: f64 = (1e-12 * var_q_nqs_k_dn2);
        let eq8_e151_q_d_n3: f64 = (1e-12 * var_q_nqs_k_dn3);
        let eq8_e151_q_d_n4: f64 = (1e-12 * var_q_nqs_k_dn4);
        let eq8_e151_q_d_n5: f64 = (1e-12 * var_q_nqs_k_dn5);
        let eq8_e151_q_d_b0: f64 = (1e-12 * var_q_nqs_k_db0);
        let eq8_e151_q_d_b1: f64 = (1e-12 * var_q_nqs_k_db1);
        let eq8_e151_q_d_b2: f64 = (1e-12 * var_q_nqs_k_db2);
        let eq8_e151_q_d_b3: f64 = (1e-12 * var_q_nqs_k_db3);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n1, eq8_e151_d_n2, eq8_e151_d_n3, eq8_e151_d_n4, eq8_e151_d_n5, eq8_e151_d_b0, eq8_e151_d_b1, eq8_e151_d_b2, eq8_e151_d_b3, eq8_e151_q, eq8_e151_q_d_n0, eq8_e151_q_d_n1, eq8_e151_q_d_n2, eq8_e151_q_d_n3, eq8_e151_q_d_n4, eq8_e151_q_d_n5, eq8_e151_q_d_b0, eq8_e151_q_d_b1, eq8_e151_q_d_b2, eq8_e151_q_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 6] = [eq8_e153_q_d_n0, eq8_e153_q_d_n1, eq8_e153_q_d_n2, eq8_e153_q_d_n3, eq8_e153_q_d_n4, eq8_e153_q_d_n5];
        let eq8_reactive_branch_derivatives: [f64; 4] = [eq8_e153_q_d_b0, eq8_e153_q_d_b1, eq8_e153_q_d_b2, eq8_e153_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5, eq11_e172_d_b0, eq11_e172_d_b1, eq11_e172_d_b2, eq11_e172_d_b3, eq11_e172_q, eq11_e172_q_d_n0, eq11_e172_q_d_n1, eq11_e172_q_d_n2, eq11_e172_q_d_n3, eq11_e172_q_d_n4, eq11_e172_q_d_n5, eq11_e172_q_d_b0, eq11_e172_q_d_b1, eq11_e172_q_d_b2, eq11_e172_q_d_b3,) = {
    if (var_guard572 != 0.0) {
        let eq11_e168_q: f64 = var_w_nqs_a;
        let eq11_e169: f64 = (var_iwnqs0_a + var_w_nqs_a);
        let eq11_e169_d_n0: f64 = (var_iwnqs0_a_dn0 + var_w_nqs_a_dn0);
        let eq11_e169_d_n1: f64 = (var_iwnqs0_a_dn1 + var_w_nqs_a_dn1);
        let eq11_e169_d_n2: f64 = (var_iwnqs0_a_dn2 + var_w_nqs_a_dn2);
        let eq11_e169_d_n3: f64 = (var_iwnqs0_a_dn3 + var_w_nqs_a_dn3);
        let eq11_e169_d_n4: f64 = (var_iwnqs0_a_dn4 + var_w_nqs_a_dn4);
        let eq11_e169_d_n5: f64 = (var_iwnqs0_a_dn5 + var_w_nqs_a_dn5);
        let eq11_e169_d_b0: f64 = (var_iwnqs0_a_db0 + var_w_nqs_a_db0);
        let eq11_e169_d_b1: f64 = (var_iwnqs0_a_db1 + var_w_nqs_a_db1);
        let eq11_e169_d_b2: f64 = (var_iwnqs0_a_db2 + var_w_nqs_a_db2);
        let eq11_e169_d_b3: f64 = (var_iwnqs0_a_db3 + var_w_nqs_a_db3);
        let eq11_e169_q: f64 = eq11_e168_q;
        let eq11_e170: f64 = (1e-13 * eq11_e169);
        let eq11_e170_d_n0: f64 = (1e-13 * eq11_e169_d_n0);
        let eq11_e170_d_n1: f64 = (1e-13 * eq11_e169_d_n1);
        let eq11_e170_d_n2: f64 = (1e-13 * eq11_e169_d_n2);
        let eq11_e170_d_n3: f64 = (1e-13 * eq11_e169_d_n3);
        let eq11_e170_d_n4: f64 = (1e-13 * eq11_e169_d_n4);
        let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);
        let eq11_e170_d_b0: f64 = (1e-13 * eq11_e169_d_b0);
        let eq11_e170_d_b1: f64 = (1e-13 * eq11_e169_d_b1);
        let eq11_e170_d_b2: f64 = (1e-13 * eq11_e169_d_b2);
        let eq11_e170_d_b3: f64 = (1e-13 * eq11_e169_d_b3);
        let eq11_e170_q: f64 = (1e-13 * eq11_e169_q);
        let eq11_e170_q_d_n0: f64 = (1e-13 * var_w_nqs_a_dn0);
        let eq11_e170_q_d_n1: f64 = (1e-13 * var_w_nqs_a_dn1);
        let eq11_e170_q_d_n2: f64 = (1e-13 * var_w_nqs_a_dn2);
        let eq11_e170_q_d_n3: f64 = (1e-13 * var_w_nqs_a_dn3);
        let eq11_e170_q_d_n4: f64 = (1e-13 * var_w_nqs_a_dn4);
        let eq11_e170_q_d_n5: f64 = (1e-13 * var_w_nqs_a_dn5);
        let eq11_e170_q_d_b0: f64 = (1e-13 * var_w_nqs_a_db0);
        let eq11_e170_q_d_b1: f64 = (1e-13 * var_w_nqs_a_db1);
        let eq11_e170_q_d_b2: f64 = (1e-13 * var_w_nqs_a_db2);
        let eq11_e170_q_d_b3: f64 = (1e-13 * var_w_nqs_a_db3);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n1, eq11_e170_d_n2, eq11_e170_d_n3, eq11_e170_d_n4, eq11_e170_d_n5, eq11_e170_d_b0, eq11_e170_d_b1, eq11_e170_d_b2, eq11_e170_d_b3, eq11_e170_q, eq11_e170_q_d_n0, eq11_e170_q_d_n1, eq11_e170_q_d_n2, eq11_e170_q_d_n3, eq11_e170_q_d_n4, eq11_e170_q_d_n5, eq11_e170_q_d_b0, eq11_e170_q_d_b1, eq11_e170_q_d_b2, eq11_e170_q_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 6] = [eq11_e172_q_d_n0, eq11_e172_q_d_n1, eq11_e172_q_d_n2, eq11_e172_q_d_n3, eq11_e172_q_d_n4, eq11_e172_q_d_n5];
        let eq11_reactive_branch_derivatives: [f64; 4] = [eq11_e172_q_d_b0, eq11_e172_q_d_b1, eq11_e172_q_d_b2, eq11_e172_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
