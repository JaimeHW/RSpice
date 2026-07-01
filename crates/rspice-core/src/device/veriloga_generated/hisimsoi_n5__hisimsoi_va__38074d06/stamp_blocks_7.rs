#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_112(
        p: &Parameters,
        var_cgdo_given: f64,
        var_cgso_given: f64,
        var_chi__blk945: f64,
        var_chi__blk945_dn0: f64,
        var_chi__blk945_dn10: f64,
        var_chi__blk945_dn11: f64,
        var_chi__blk945_dn12: f64,
        var_chi__blk945_dn17: f64,
        var_chi__blk945_dn2: f64,
        var_chi__blk945_dn6: f64,
        var_chi__blk945_dn7: f64,
        var_cnst0over__blk930: f64,
        var_cnst0over__blk930_dn0: f64,
        var_cnst0over__blk930_dn10: f64,
        var_cnst0over__blk930_dn11: f64,
        var_cnst0over__blk930_dn12: f64,
        var_cnst0over__blk930_dn17: f64,
        var_cnst0over__blk930_dn2: f64,
        var_cnst0over__blk930_dn6: f64,
        var_cnst0over__blk930_dn7: f64,
        var_fb__blk969: f64,
        var_fb__blk969_dn0: f64,
        var_fb__blk969_dn10: f64,
        var_fb__blk969_dn11: f64,
        var_fb__blk969_dn12: f64,
        var_fb__blk969_dn17: f64,
        var_fb__blk969_dn2: f64,
        var_fb__blk969_dn6: f64,
        var_fb__blk969_dn7: f64,
        var_flg_overd__blk917: f64,
        var_flg_overs__blk916: f64,
        var_flg_ovloopd__blk915: f64,
        var_flg_ovloops__blk914: f64,
        var_fs01__blk967: f64,
        var_fs01__blk967_dn0: f64,
        var_fs01__blk967_dn10: f64,
        var_fs01__blk967_dn11: f64,
        var_fs01__blk967_dn12: f64,
        var_fs01__blk967_dn17: f64,
        var_fs01__blk967_dn2: f64,
        var_fs01__blk967_dn6: f64,
        var_fs01__blk967_dn7: f64,
        var_fs02__blk971: f64,
        var_fs02__blk971_dn0: f64,
        var_fs02__blk971_dn10: f64,
        var_fs02__blk971_dn11: f64,
        var_fs02__blk971_dn12: f64,
        var_fs02__blk971_dn17: f64,
        var_fs02__blk971_dn2: f64,
        var_fs02__blk971_dn6: f64,
        var_fs02__blk971_dn7: f64,
        var_guard1004: f64,
        var_guard1011: f64,
        var_guard980: f64,
        var_guard981: f64,
        var_lov: f64,
        var_mode: f64,
        var_modenml: f64,
        var_modervs: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn17: f64,
        var_vds_dn2: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vgs: f64,
        var_vgs_dn11: f64,
        var_vgs_dn6: f64,
        var_vgs_dn7: f64,
        var_w_diodcv: f64,
        var_w_dioscv: f64,
        var_weffcv_nf: f64,
        var_cgdoe_slot: &mut f64,
        var_cgdoe_dn0_slot: &mut f64,
        var_cgdoe_dn10_slot: &mut f64,
        var_cgdoe_dn11_slot: &mut f64,
        var_cgdoe_dn12_slot: &mut f64,
        var_cgdoe_dn17_slot: &mut f64,
        var_cgdoe_dn2_slot: &mut f64,
        var_cgdoe_dn6_slot: &mut f64,
        var_cgdoe_dn7_slot: &mut f64,
        var_cgsoe_slot: &mut f64,
        var_cgsoe_dn0_slot: &mut f64,
        var_cgsoe_dn10_slot: &mut f64,
        var_cgsoe_dn11_slot: &mut f64,
        var_cgsoe_dn12_slot: &mut f64,
        var_cgsoe_dn17_slot: &mut f64,
        var_cgsoe_dn2_slot: &mut f64,
        var_cgsoe_dn6_slot: &mut f64,
        var_cgsoe_dn7_slot: &mut f64,
        var_flg_overgiven_slot: &mut f64,
        var_guard1018_slot: &mut f64,
        var_guard1020_slot: &mut f64,
        var_guard1021_slot: &mut f64,
        var_guard1022_slot: &mut f64,
        var_guard1023_slot: &mut f64,
        var_guard1024_slot: &mut f64,
        var_guard1025_slot: &mut f64,
        var_qbdld_slot: &mut f64,
        var_qbdld_dn0_slot: &mut f64,
        var_qbdld_dn10_slot: &mut f64,
        var_qbdld_dn11_slot: &mut f64,
        var_qbdld_dn12_slot: &mut f64,
        var_qbdld_dn17_slot: &mut f64,
        var_qbdld_dn2_slot: &mut f64,
        var_qbdld_dn6_slot: &mut f64,
        var_qbdld_dn7_slot: &mut f64,
        var_qbsld_slot: &mut f64,
        var_qbsld_dn0_slot: &mut f64,
        var_qbsld_dn10_slot: &mut f64,
        var_qbsld_dn11_slot: &mut f64,
        var_qbsld_dn12_slot: &mut f64,
        var_qbsld_dn17_slot: &mut f64,
        var_qbsld_dn2_slot: &mut f64,
        var_qbsld_dn6_slot: &mut f64,
        var_qbsld_dn7_slot: &mut f64,
        var_qbuld_slot: &mut f64,
        var_qbuld_dn0_slot: &mut f64,
        var_qbuld_dn10_slot: &mut f64,
        var_qbuld_dn11_slot: &mut f64,
        var_qbuld_dn12_slot: &mut f64,
        var_qbuld_dn17_slot: &mut f64,
        var_qbuld_dn2_slot: &mut f64,
        var_qbuld_dn6_slot: &mut f64,
        var_qbuld_dn7_slot: &mut f64,
        var_qgod_slot: &mut f64,
        var_qgod_dn0_slot: &mut f64,
        var_qgod_dn10_slot: &mut f64,
        var_qgod_dn11_slot: &mut f64,
        var_qgod_dn12_slot: &mut f64,
        var_qgod_dn17_slot: &mut f64,
        var_qgod_dn2_slot: &mut f64,
        var_qgod_dn6_slot: &mut f64,
        var_qgod_dn7_slot: &mut f64,
        var_qgos_slot: &mut f64,
        var_qgos_dn0_slot: &mut f64,
        var_qgos_dn10_slot: &mut f64,
        var_qgos_dn11_slot: &mut f64,
        var_qgos_dn12_slot: &mut f64,
        var_qgos_dn17_slot: &mut f64,
        var_qgos_dn2_slot: &mut f64,
        var_qgos_dn6_slot: &mut f64,
        var_qgos_dn7_slot: &mut f64,
        var_qiuld_slot: &mut f64,
        var_qiuld_dn0_slot: &mut f64,
        var_qiuld_dn10_slot: &mut f64,
        var_qiuld_dn11_slot: &mut f64,
        var_qiuld_dn12_slot: &mut f64,
        var_qiuld_dn17_slot: &mut f64,
        var_qiuld_dn2_slot: &mut f64,
        var_qiuld_dn6_slot: &mut f64,
        var_qiuld_dn7_slot: &mut f64,
        var_qovd_slot: &mut f64,
        var_qovd_dn0_slot: &mut f64,
        var_qovd_dn10_slot: &mut f64,
        var_qovd_dn11_slot: &mut f64,
        var_qovd_dn12_slot: &mut f64,
        var_qovd_dn17_slot: &mut f64,
        var_qovd_dn2_slot: &mut f64,
        var_qovd_dn6_slot: &mut f64,
        var_qovd_dn7_slot: &mut f64,
        var_qovs_slot: &mut f64,
        var_qovs_dn0_slot: &mut f64,
        var_qovs_dn10_slot: &mut f64,
        var_qovs_dn11_slot: &mut f64,
        var_qovs_dn12_slot: &mut f64,
        var_qovs_dn17_slot: &mut f64,
        var_qovs_dn2_slot: &mut f64,
        var_qovs_dn6_slot: &mut f64,
        var_qovs_dn7_slot: &mut f64,
        var_qsuld_slot: &mut f64,
        var_qsuld_dn0_slot: &mut f64,
        var_qsuld_dn10_slot: &mut f64,
        var_qsuld_dn11_slot: &mut f64,
        var_qsuld_dn12_slot: &mut f64,
        var_qsuld_dn17_slot: &mut f64,
        var_qsuld_dn2_slot: &mut f64,
        var_qsuld_dn6_slot: &mut f64,
        var_qsuld_dn7_slot: &mut f64,
        var_t1__blk898_slot: &mut f64,
        var_t1__blk898_dn0_slot: &mut f64,
        var_t1__blk898_dn10_slot: &mut f64,
        var_t1__blk898_dn11_slot: &mut f64,
        var_t1__blk898_dn12_slot: &mut f64,
        var_t1__blk898_dn17_slot: &mut f64,
        var_t1__blk898_dn2_slot: &mut f64,
        var_t1__blk898_dn6_slot: &mut f64,
        var_t1__blk898_dn7_slot: &mut f64,
        var_t4__blk901_slot: &mut f64,
        var_t4__blk901_dn0_slot: &mut f64,
        var_t4__blk901_dn10_slot: &mut f64,
        var_t4__blk901_dn11_slot: &mut f64,
        var_t4__blk901_dn12_slot: &mut f64,
        var_t4__blk901_dn17_slot: &mut f64,
        var_t4__blk901_dn2_slot: &mut f64,
        var_t4__blk901_dn6_slot: &mut f64,
        var_t4__blk901_dn7_slot: &mut f64,
        var_xi0__blk978_slot: &mut f64,
        var_xi0__blk978_dn0_slot: &mut f64,
        var_xi0__blk978_dn10_slot: &mut f64,
        var_xi0__blk978_dn11_slot: &mut f64,
        var_xi0__blk978_dn12_slot: &mut f64,
        var_xi0__blk978_dn17_slot: &mut f64,
        var_xi0__blk978_dn2_slot: &mut f64,
        var_xi0__blk978_dn6_slot: &mut f64,
        var_xi0__blk978_dn7_slot: &mut f64,
        var_xi0p12__blk979_slot: &mut f64,
        var_xi0p12__blk979_dn0_slot: &mut f64,
        var_xi0p12__blk979_dn10_slot: &mut f64,
        var_xi0p12__blk979_dn11_slot: &mut f64,
        var_xi0p12__blk979_dn12_slot: &mut f64,
        var_xi0p12__blk979_dn17_slot: &mut f64,
        var_xi0p12__blk979_dn2_slot: &mut f64,
        var_xi0p12__blk979_dn6_slot: &mut f64,
        var_xi0p12__blk979_dn7_slot: &mut f64,
    ) {
        let mut var_cgdoe: f64 = *var_cgdoe_slot;
        let mut var_cgdoe_dn0: f64 = *var_cgdoe_dn0_slot;
        let mut var_cgdoe_dn10: f64 = *var_cgdoe_dn10_slot;
        let mut var_cgdoe_dn11: f64 = *var_cgdoe_dn11_slot;
        let mut var_cgdoe_dn12: f64 = *var_cgdoe_dn12_slot;
        let mut var_cgdoe_dn17: f64 = *var_cgdoe_dn17_slot;
        let mut var_cgdoe_dn2: f64 = *var_cgdoe_dn2_slot;
        let mut var_cgdoe_dn6: f64 = *var_cgdoe_dn6_slot;
        let mut var_cgdoe_dn7: f64 = *var_cgdoe_dn7_slot;
        let mut var_cgsoe: f64 = *var_cgsoe_slot;
        let mut var_cgsoe_dn0: f64 = *var_cgsoe_dn0_slot;
        let mut var_cgsoe_dn10: f64 = *var_cgsoe_dn10_slot;
        let mut var_cgsoe_dn11: f64 = *var_cgsoe_dn11_slot;
        let mut var_cgsoe_dn12: f64 = *var_cgsoe_dn12_slot;
        let mut var_cgsoe_dn17: f64 = *var_cgsoe_dn17_slot;
        let mut var_cgsoe_dn2: f64 = *var_cgsoe_dn2_slot;
        let mut var_cgsoe_dn6: f64 = *var_cgsoe_dn6_slot;
        let mut var_cgsoe_dn7: f64 = *var_cgsoe_dn7_slot;
        let mut var_flg_overgiven: f64 = *var_flg_overgiven_slot;
        let mut var_guard1018: f64 = *var_guard1018_slot;
        let mut var_guard1020: f64 = *var_guard1020_slot;
        let mut var_guard1021: f64 = *var_guard1021_slot;
        let mut var_guard1022: f64 = *var_guard1022_slot;
        let mut var_guard1023: f64 = *var_guard1023_slot;
        let mut var_guard1024: f64 = *var_guard1024_slot;
        let mut var_guard1025: f64 = *var_guard1025_slot;
        let mut var_qbdld: f64 = *var_qbdld_slot;
        let mut var_qbdld_dn0: f64 = *var_qbdld_dn0_slot;
        let mut var_qbdld_dn10: f64 = *var_qbdld_dn10_slot;
        let mut var_qbdld_dn11: f64 = *var_qbdld_dn11_slot;
        let mut var_qbdld_dn12: f64 = *var_qbdld_dn12_slot;
        let mut var_qbdld_dn17: f64 = *var_qbdld_dn17_slot;
        let mut var_qbdld_dn2: f64 = *var_qbdld_dn2_slot;
        let mut var_qbdld_dn6: f64 = *var_qbdld_dn6_slot;
        let mut var_qbdld_dn7: f64 = *var_qbdld_dn7_slot;
        let mut var_qbsld: f64 = *var_qbsld_slot;
        let mut var_qbsld_dn0: f64 = *var_qbsld_dn0_slot;
        let mut var_qbsld_dn10: f64 = *var_qbsld_dn10_slot;
        let mut var_qbsld_dn11: f64 = *var_qbsld_dn11_slot;
        let mut var_qbsld_dn12: f64 = *var_qbsld_dn12_slot;
        let mut var_qbsld_dn17: f64 = *var_qbsld_dn17_slot;
        let mut var_qbsld_dn2: f64 = *var_qbsld_dn2_slot;
        let mut var_qbsld_dn6: f64 = *var_qbsld_dn6_slot;
        let mut var_qbsld_dn7: f64 = *var_qbsld_dn7_slot;
        let mut var_qbuld: f64 = *var_qbuld_slot;
        let mut var_qbuld_dn0: f64 = *var_qbuld_dn0_slot;
        let mut var_qbuld_dn10: f64 = *var_qbuld_dn10_slot;
        let mut var_qbuld_dn11: f64 = *var_qbuld_dn11_slot;
        let mut var_qbuld_dn12: f64 = *var_qbuld_dn12_slot;
        let mut var_qbuld_dn17: f64 = *var_qbuld_dn17_slot;
        let mut var_qbuld_dn2: f64 = *var_qbuld_dn2_slot;
        let mut var_qbuld_dn6: f64 = *var_qbuld_dn6_slot;
        let mut var_qbuld_dn7: f64 = *var_qbuld_dn7_slot;
        let mut var_qgod: f64 = *var_qgod_slot;
        let mut var_qgod_dn0: f64 = *var_qgod_dn0_slot;
        let mut var_qgod_dn10: f64 = *var_qgod_dn10_slot;
        let mut var_qgod_dn11: f64 = *var_qgod_dn11_slot;
        let mut var_qgod_dn12: f64 = *var_qgod_dn12_slot;
        let mut var_qgod_dn17: f64 = *var_qgod_dn17_slot;
        let mut var_qgod_dn2: f64 = *var_qgod_dn2_slot;
        let mut var_qgod_dn6: f64 = *var_qgod_dn6_slot;
        let mut var_qgod_dn7: f64 = *var_qgod_dn7_slot;
        let mut var_qgos: f64 = *var_qgos_slot;
        let mut var_qgos_dn0: f64 = *var_qgos_dn0_slot;
        let mut var_qgos_dn10: f64 = *var_qgos_dn10_slot;
        let mut var_qgos_dn11: f64 = *var_qgos_dn11_slot;
        let mut var_qgos_dn12: f64 = *var_qgos_dn12_slot;
        let mut var_qgos_dn17: f64 = *var_qgos_dn17_slot;
        let mut var_qgos_dn2: f64 = *var_qgos_dn2_slot;
        let mut var_qgos_dn6: f64 = *var_qgos_dn6_slot;
        let mut var_qgos_dn7: f64 = *var_qgos_dn7_slot;
        let mut var_qiuld: f64 = *var_qiuld_slot;
        let mut var_qiuld_dn0: f64 = *var_qiuld_dn0_slot;
        let mut var_qiuld_dn10: f64 = *var_qiuld_dn10_slot;
        let mut var_qiuld_dn11: f64 = *var_qiuld_dn11_slot;
        let mut var_qiuld_dn12: f64 = *var_qiuld_dn12_slot;
        let mut var_qiuld_dn17: f64 = *var_qiuld_dn17_slot;
        let mut var_qiuld_dn2: f64 = *var_qiuld_dn2_slot;
        let mut var_qiuld_dn6: f64 = *var_qiuld_dn6_slot;
        let mut var_qiuld_dn7: f64 = *var_qiuld_dn7_slot;
        let mut var_qovd: f64 = *var_qovd_slot;
        let mut var_qovd_dn0: f64 = *var_qovd_dn0_slot;
        let mut var_qovd_dn10: f64 = *var_qovd_dn10_slot;
        let mut var_qovd_dn11: f64 = *var_qovd_dn11_slot;
        let mut var_qovd_dn12: f64 = *var_qovd_dn12_slot;
        let mut var_qovd_dn17: f64 = *var_qovd_dn17_slot;
        let mut var_qovd_dn2: f64 = *var_qovd_dn2_slot;
        let mut var_qovd_dn6: f64 = *var_qovd_dn6_slot;
        let mut var_qovd_dn7: f64 = *var_qovd_dn7_slot;
        let mut var_qovs: f64 = *var_qovs_slot;
        let mut var_qovs_dn0: f64 = *var_qovs_dn0_slot;
        let mut var_qovs_dn10: f64 = *var_qovs_dn10_slot;
        let mut var_qovs_dn11: f64 = *var_qovs_dn11_slot;
        let mut var_qovs_dn12: f64 = *var_qovs_dn12_slot;
        let mut var_qovs_dn17: f64 = *var_qovs_dn17_slot;
        let mut var_qovs_dn2: f64 = *var_qovs_dn2_slot;
        let mut var_qovs_dn6: f64 = *var_qovs_dn6_slot;
        let mut var_qovs_dn7: f64 = *var_qovs_dn7_slot;
        let mut var_qsuld: f64 = *var_qsuld_slot;
        let mut var_qsuld_dn0: f64 = *var_qsuld_dn0_slot;
        let mut var_qsuld_dn10: f64 = *var_qsuld_dn10_slot;
        let mut var_qsuld_dn11: f64 = *var_qsuld_dn11_slot;
        let mut var_qsuld_dn12: f64 = *var_qsuld_dn12_slot;
        let mut var_qsuld_dn17: f64 = *var_qsuld_dn17_slot;
        let mut var_qsuld_dn2: f64 = *var_qsuld_dn2_slot;
        let mut var_qsuld_dn6: f64 = *var_qsuld_dn6_slot;
        let mut var_qsuld_dn7: f64 = *var_qsuld_dn7_slot;
        let mut var_t1__blk898: f64 = *var_t1__blk898_slot;
        let mut var_t1__blk898_dn0: f64 = *var_t1__blk898_dn0_slot;
        let mut var_t1__blk898_dn10: f64 = *var_t1__blk898_dn10_slot;
        let mut var_t1__blk898_dn11: f64 = *var_t1__blk898_dn11_slot;
        let mut var_t1__blk898_dn12: f64 = *var_t1__blk898_dn12_slot;
        let mut var_t1__blk898_dn17: f64 = *var_t1__blk898_dn17_slot;
        let mut var_t1__blk898_dn2: f64 = *var_t1__blk898_dn2_slot;
        let mut var_t1__blk898_dn6: f64 = *var_t1__blk898_dn6_slot;
        let mut var_t1__blk898_dn7: f64 = *var_t1__blk898_dn7_slot;
        let mut var_t4__blk901: f64 = *var_t4__blk901_slot;
        let mut var_t4__blk901_dn0: f64 = *var_t4__blk901_dn0_slot;
        let mut var_t4__blk901_dn10: f64 = *var_t4__blk901_dn10_slot;
        let mut var_t4__blk901_dn11: f64 = *var_t4__blk901_dn11_slot;
        let mut var_t4__blk901_dn12: f64 = *var_t4__blk901_dn12_slot;
        let mut var_t4__blk901_dn17: f64 = *var_t4__blk901_dn17_slot;
        let mut var_t4__blk901_dn2: f64 = *var_t4__blk901_dn2_slot;
        let mut var_t4__blk901_dn6: f64 = *var_t4__blk901_dn6_slot;
        let mut var_t4__blk901_dn7: f64 = *var_t4__blk901_dn7_slot;
        let mut var_xi0__blk978: f64 = *var_xi0__blk978_slot;
        let mut var_xi0__blk978_dn0: f64 = *var_xi0__blk978_dn0_slot;
        let mut var_xi0__blk978_dn10: f64 = *var_xi0__blk978_dn10_slot;
        let mut var_xi0__blk978_dn11: f64 = *var_xi0__blk978_dn11_slot;
        let mut var_xi0__blk978_dn12: f64 = *var_xi0__blk978_dn12_slot;
        let mut var_xi0__blk978_dn17: f64 = *var_xi0__blk978_dn17_slot;
        let mut var_xi0__blk978_dn2: f64 = *var_xi0__blk978_dn2_slot;
        let mut var_xi0__blk978_dn6: f64 = *var_xi0__blk978_dn6_slot;
        let mut var_xi0__blk978_dn7: f64 = *var_xi0__blk978_dn7_slot;
        let mut var_xi0p12__blk979: f64 = *var_xi0p12__blk979_slot;
        let mut var_xi0p12__blk979_dn0: f64 = *var_xi0p12__blk979_dn0_slot;
        let mut var_xi0p12__blk979_dn10: f64 = *var_xi0p12__blk979_dn10_slot;
        let mut var_xi0p12__blk979_dn11: f64 = *var_xi0p12__blk979_dn11_slot;
        let mut var_xi0p12__blk979_dn12: f64 = *var_xi0p12__blk979_dn12_slot;
        let mut var_xi0p12__blk979_dn17: f64 = *var_xi0p12__blk979_dn17_slot;
        let mut var_xi0p12__blk979_dn2: f64 = *var_xi0p12__blk979_dn2_slot;
        let mut var_xi0p12__blk979_dn6: f64 = *var_xi0p12__blk979_dn6_slot;
        let mut var_xi0p12__blk979_dn7: f64 = *var_xi0p12__blk979_dn7_slot;

        let assign31390_e46183: f64 = if var_chi__blk945 < 5.0 { 1.0 } else { 0.0 };
        var_guard1018 = assign31390_e46183;

        let (assign31430_e46245, assign31430_e46245_d_n0, assign31430_e46245_d_n2, assign31430_e46245_d_n6, assign31430_e46245_d_n7, assign31430_e46245_d_n10, assign31430_e46245_d_n11, assign31430_e46245_d_n12, assign31430_e46245_d_n17,) = {
    if ((((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) && (var_guard1004 == 0.0)) && (var_guard1011 != 0.0)) && (var_guard1018 != 0.0)) {
        let assign31430_e46239: f64 = (var_fb__blk969 * var_fb__blk969);
        let assign31430_e46242: f64 = (10.0 * 2.220446049250313e-16);
        let assign31430_e46243: f64 = (assign31430_e46239 + assign31430_e46242);
        (assign31430_e46243, ((var_fb__blk969_dn0 * var_fb__blk969) + (var_fb__blk969 * var_fb__blk969_dn0)), ((var_fb__blk969_dn2 * var_fb__blk969) + (var_fb__blk969 * var_fb__blk969_dn2)), ((var_fb__blk969_dn6 * var_fb__blk969) + (var_fb__blk969 * var_fb__blk969_dn6)), ((var_fb__blk969_dn7 * var_fb__blk969) + (var_fb__blk969 * var_fb__blk969_dn7)), ((var_fb__blk969_dn10 * var_fb__blk969) + (var_fb__blk969 * var_fb__blk969_dn10)), ((var_fb__blk969_dn11 * var_fb__blk969) + (var_fb__blk969 * var_fb__blk969_dn11)), ((var_fb__blk969_dn12 * var_fb__blk969) + (var_fb__blk969 * var_fb__blk969_dn12)), ((var_fb__blk969_dn17 * var_fb__blk969) + (var_fb__blk969 * var_fb__blk969_dn17)),)
    } else {
        (var_xi0__blk978, var_xi0__blk978_dn0, var_xi0__blk978_dn2, var_xi0__blk978_dn6, var_xi0__blk978_dn7, var_xi0__blk978_dn10, var_xi0__blk978_dn11, var_xi0__blk978_dn12, var_xi0__blk978_dn17,)
    }
};
        var_xi0__blk978 = assign31430_e46245;
        var_xi0__blk978_dn0 = assign31430_e46245_d_n0;
        var_xi0__blk978_dn2 = assign31430_e46245_d_n2;
        var_xi0__blk978_dn6 = assign31430_e46245_d_n6;
        var_xi0__blk978_dn7 = assign31430_e46245_d_n7;
        var_xi0__blk978_dn10 = assign31430_e46245_d_n10;
        var_xi0__blk978_dn11 = assign31430_e46245_d_n11;
        var_xi0__blk978_dn12 = assign31430_e46245_d_n12;
        var_xi0__blk978_dn17 = assign31430_e46245_d_n17;

        let (assign31440_e46265, assign31440_e46265_d_n0, assign31440_e46265_d_n2, assign31440_e46265_d_n6, assign31440_e46265_d_n7, assign31440_e46265_d_n10, assign31440_e46265_d_n11, assign31440_e46265_d_n12, assign31440_e46265_d_n17,) = {
    if ((((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) && (var_guard1004 == 0.0)) && (var_guard1011 != 0.0)) && (var_guard1018 != 0.0)) {
        let assign31440_e46262: f64 = (10.0 * 2.220446049250313e-16);
        let assign31440_e46263: f64 = (var_fb__blk969 + assign31440_e46262);
        (assign31440_e46263, var_fb__blk969_dn0, var_fb__blk969_dn2, var_fb__blk969_dn6, var_fb__blk969_dn7, var_fb__blk969_dn10, var_fb__blk969_dn11, var_fb__blk969_dn12, var_fb__blk969_dn17,)
    } else {
        (var_xi0p12__blk979, var_xi0p12__blk979_dn0, var_xi0p12__blk979_dn2, var_xi0p12__blk979_dn6, var_xi0p12__blk979_dn7, var_xi0p12__blk979_dn10, var_xi0p12__blk979_dn11, var_xi0p12__blk979_dn12, var_xi0p12__blk979_dn17,)
    }
};
        var_xi0p12__blk979 = assign31440_e46265;
        var_xi0p12__blk979_dn0 = assign31440_e46265_d_n0;
        var_xi0p12__blk979_dn2 = assign31440_e46265_d_n2;
        var_xi0p12__blk979_dn6 = assign31440_e46265_d_n6;
        var_xi0p12__blk979_dn7 = assign31440_e46265_d_n7;
        var_xi0p12__blk979_dn10 = assign31440_e46265_d_n10;
        var_xi0p12__blk979_dn11 = assign31440_e46265_d_n11;
        var_xi0p12__blk979_dn12 = assign31440_e46265_d_n12;
        var_xi0p12__blk979_dn17 = assign31440_e46265_d_n17;

        let (assign31460_e46301, assign31460_e46301_d_n0, assign31460_e46301_d_n2, assign31460_e46301_d_n6, assign31460_e46301_d_n7, assign31460_e46301_d_n10, assign31460_e46301_d_n11, assign31460_e46301_d_n12, assign31460_e46301_d_n17,) = {
    if ((((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) && (var_guard1004 == 0.0)) && (var_guard1011 != 0.0)) && (var_guard1018 == 0.0)) {
        let assign31460_e46299: f64 = (var_chi__blk945 - 1.0);
        (assign31460_e46299, var_chi__blk945_dn0, var_chi__blk945_dn2, var_chi__blk945_dn6, var_chi__blk945_dn7, var_chi__blk945_dn10, var_chi__blk945_dn11, var_chi__blk945_dn12, var_chi__blk945_dn17,)
    } else {
        (var_xi0__blk978, var_xi0__blk978_dn0, var_xi0__blk978_dn2, var_xi0__blk978_dn6, var_xi0__blk978_dn7, var_xi0__blk978_dn10, var_xi0__blk978_dn11, var_xi0__blk978_dn12, var_xi0__blk978_dn17,)
    }
};
        var_xi0__blk978 = assign31460_e46301;
        var_xi0__blk978_dn0 = assign31460_e46301_d_n0;
        var_xi0__blk978_dn2 = assign31460_e46301_d_n2;
        var_xi0__blk978_dn6 = assign31460_e46301_d_n6;
        var_xi0__blk978_dn7 = assign31460_e46301_d_n7;
        var_xi0__blk978_dn10 = assign31460_e46301_d_n10;
        var_xi0__blk978_dn11 = assign31460_e46301_d_n11;
        var_xi0__blk978_dn12 = assign31460_e46301_d_n12;
        var_xi0__blk978_dn17 = assign31460_e46301_d_n17;

        let (assign31470_e46319, assign31470_e46319_d_n0, assign31470_e46319_d_n2, assign31470_e46319_d_n6, assign31470_e46319_d_n7, assign31470_e46319_d_n10, assign31470_e46319_d_n11, assign31470_e46319_d_n12, assign31470_e46319_d_n17,) = {
    if ((((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) && (var_guard1004 == 0.0)) && (var_guard1011 != 0.0)) && (var_guard1018 == 0.0)) {
        let assign31470_e46317: f64 = (var_xi0__blk978).sqrt();
        (assign31470_e46317, (var_xi0__blk978_dn0 / (2.0 * assign31470_e46317)), (var_xi0__blk978_dn2 / (2.0 * assign31470_e46317)), (var_xi0__blk978_dn6 / (2.0 * assign31470_e46317)), (var_xi0__blk978_dn7 / (2.0 * assign31470_e46317)), (var_xi0__blk978_dn10 / (2.0 * assign31470_e46317)), (var_xi0__blk978_dn11 / (2.0 * assign31470_e46317)), (var_xi0__blk978_dn12 / (2.0 * assign31470_e46317)), (var_xi0__blk978_dn17 / (2.0 * assign31470_e46317)),)
    } else {
        (var_xi0p12__blk979, var_xi0p12__blk979_dn0, var_xi0p12__blk979_dn2, var_xi0p12__blk979_dn6, var_xi0p12__blk979_dn7, var_xi0p12__blk979_dn10, var_xi0p12__blk979_dn11, var_xi0p12__blk979_dn12, var_xi0p12__blk979_dn17,)
    }
};
        var_xi0p12__blk979 = assign31470_e46319;
        var_xi0p12__blk979_dn0 = assign31470_e46319_d_n0;
        var_xi0p12__blk979_dn2 = assign31470_e46319_d_n2;
        var_xi0p12__blk979_dn6 = assign31470_e46319_d_n6;
        var_xi0p12__blk979_dn7 = assign31470_e46319_d_n7;
        var_xi0p12__blk979_dn10 = assign31470_e46319_d_n10;
        var_xi0p12__blk979_dn11 = assign31470_e46319_d_n11;
        var_xi0p12__blk979_dn12 = assign31470_e46319_d_n12;
        var_xi0p12__blk979_dn17 = assign31470_e46319_d_n17;

        let (assign31480_e46335, assign31480_e46335_d_n0, assign31480_e46335_d_n2, assign31480_e46335_d_n6, assign31480_e46335_d_n7, assign31480_e46335_d_n10, assign31480_e46335_d_n11, assign31480_e46335_d_n12, assign31480_e46335_d_n17,) = {
    if (((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) && (var_guard1004 == 0.0)) && (var_guard1011 != 0.0)) {
        let assign31480_e46333: f64 = (var_cnst0over__blk930 * var_xi0p12__blk979);
        (assign31480_e46333, ((var_cnst0over__blk930_dn0 * var_xi0p12__blk979) + (var_cnst0over__blk930 * var_xi0p12__blk979_dn0)), ((var_cnst0over__blk930_dn2 * var_xi0p12__blk979) + (var_cnst0over__blk930 * var_xi0p12__blk979_dn2)), ((var_cnst0over__blk930_dn6 * var_xi0p12__blk979) + (var_cnst0over__blk930 * var_xi0p12__blk979_dn6)), ((var_cnst0over__blk930_dn7 * var_xi0p12__blk979) + (var_cnst0over__blk930 * var_xi0p12__blk979_dn7)), ((var_cnst0over__blk930_dn10 * var_xi0p12__blk979) + (var_cnst0over__blk930 * var_xi0p12__blk979_dn10)), ((var_cnst0over__blk930_dn11 * var_xi0p12__blk979) + (var_cnst0over__blk930 * var_xi0p12__blk979_dn11)), ((var_cnst0over__blk930_dn12 * var_xi0p12__blk979) + (var_cnst0over__blk930 * var_xi0p12__blk979_dn12)), ((var_cnst0over__blk930_dn17 * var_xi0p12__blk979) + (var_cnst0over__blk930 * var_xi0p12__blk979_dn17)),)
    } else {
        (var_qbuld, var_qbuld_dn0, var_qbuld_dn2, var_qbuld_dn6, var_qbuld_dn7, var_qbuld_dn10, var_qbuld_dn11, var_qbuld_dn12, var_qbuld_dn17,)
    }
};
        var_qbuld = assign31480_e46335;
        var_qbuld_dn0 = assign31480_e46335_d_n0;
        var_qbuld_dn2 = assign31480_e46335_d_n2;
        var_qbuld_dn6 = assign31480_e46335_d_n6;
        var_qbuld_dn7 = assign31480_e46335_d_n7;
        var_qbuld_dn10 = assign31480_e46335_d_n10;
        var_qbuld_dn11 = assign31480_e46335_d_n11;
        var_qbuld_dn12 = assign31480_e46335_d_n12;
        var_qbuld_dn17 = assign31480_e46335_d_n17;

        let (assign31490_e46353, assign31490_e46353_d_n0, assign31490_e46353_d_n2, assign31490_e46353_d_n6, assign31490_e46353_d_n7, assign31490_e46353_d_n10, assign31490_e46353_d_n11, assign31490_e46353_d_n12, assign31490_e46353_d_n17,) = {
    if (((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) && (var_guard1004 == 0.0)) && (var_guard1011 != 0.0)) {
        let assign31490_e46350: f64 = (var_fs02__blk971 + var_xi0p12__blk979);
        let assign31490_e46351: f64 = (1.0 / assign31490_e46350);
        (assign31490_e46351, (-((var_fs02__blk971_dn0 + var_xi0p12__blk979_dn0) / (assign31490_e46350 * assign31490_e46350))), (-((var_fs02__blk971_dn2 + var_xi0p12__blk979_dn2) / (assign31490_e46350 * assign31490_e46350))), (-((var_fs02__blk971_dn6 + var_xi0p12__blk979_dn6) / (assign31490_e46350 * assign31490_e46350))), (-((var_fs02__blk971_dn7 + var_xi0p12__blk979_dn7) / (assign31490_e46350 * assign31490_e46350))), (-((var_fs02__blk971_dn10 + var_xi0p12__blk979_dn10) / (assign31490_e46350 * assign31490_e46350))), (-((var_fs02__blk971_dn11 + var_xi0p12__blk979_dn11) / (assign31490_e46350 * assign31490_e46350))), (-((var_fs02__blk971_dn12 + var_xi0p12__blk979_dn12) / (assign31490_e46350 * assign31490_e46350))), (-((var_fs02__blk971_dn17 + var_xi0p12__blk979_dn17) / (assign31490_e46350 * assign31490_e46350))),)
    } else {
        (var_t1__blk898, var_t1__blk898_dn0, var_t1__blk898_dn2, var_t1__blk898_dn6, var_t1__blk898_dn7, var_t1__blk898_dn10, var_t1__blk898_dn11, var_t1__blk898_dn12, var_t1__blk898_dn17,)
    }
};
        var_t1__blk898 = assign31490_e46353;
        var_t1__blk898_dn0 = assign31490_e46353_d_n0;
        var_t1__blk898_dn2 = assign31490_e46353_d_n2;
        var_t1__blk898_dn6 = assign31490_e46353_d_n6;
        var_t1__blk898_dn7 = assign31490_e46353_d_n7;
        var_t1__blk898_dn10 = assign31490_e46353_d_n10;
        var_t1__blk898_dn11 = assign31490_e46353_d_n11;
        var_t1__blk898_dn12 = assign31490_e46353_d_n12;
        var_t1__blk898_dn17 = assign31490_e46353_d_n17;

        let (assign31500_e46371, assign31500_e46371_d_n0, assign31500_e46371_d_n2, assign31500_e46371_d_n6, assign31500_e46371_d_n7, assign31500_e46371_d_n10, assign31500_e46371_d_n11, assign31500_e46371_d_n12, assign31500_e46371_d_n17,) = {
    if (((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) && (var_guard1004 == 0.0)) && (var_guard1011 != 0.0)) {
        let assign31500_e46367: f64 = (var_cnst0over__blk930 * var_fs01__blk967);
        let assign31500_e46369: f64 = (assign31500_e46367 * var_t1__blk898);
        (assign31500_e46369, ((((var_cnst0over__blk930_dn0 * var_fs01__blk967) + (var_cnst0over__blk930 * var_fs01__blk967_dn0)) * var_t1__blk898) + (assign31500_e46367 * var_t1__blk898_dn0)), ((((var_cnst0over__blk930_dn2 * var_fs01__blk967) + (var_cnst0over__blk930 * var_fs01__blk967_dn2)) * var_t1__blk898) + (assign31500_e46367 * var_t1__blk898_dn2)), ((((var_cnst0over__blk930_dn6 * var_fs01__blk967) + (var_cnst0over__blk930 * var_fs01__blk967_dn6)) * var_t1__blk898) + (assign31500_e46367 * var_t1__blk898_dn6)), ((((var_cnst0over__blk930_dn7 * var_fs01__blk967) + (var_cnst0over__blk930 * var_fs01__blk967_dn7)) * var_t1__blk898) + (assign31500_e46367 * var_t1__blk898_dn7)), ((((var_cnst0over__blk930_dn10 * var_fs01__blk967) + (var_cnst0over__blk930 * var_fs01__blk967_dn10)) * var_t1__blk898) + (assign31500_e46367 * var_t1__blk898_dn10)), ((((var_cnst0over__blk930_dn11 * var_fs01__blk967) + (var_cnst0over__blk930 * var_fs01__blk967_dn11)) * var_t1__blk898) + (assign31500_e46367 * var_t1__blk898_dn11)), ((((var_cnst0over__blk930_dn12 * var_fs01__blk967) + (var_cnst0over__blk930 * var_fs01__blk967_dn12)) * var_t1__blk898) + (assign31500_e46367 * var_t1__blk898_dn12)), ((((var_cnst0over__blk930_dn17 * var_fs01__blk967) + (var_cnst0over__blk930 * var_fs01__blk967_dn17)) * var_t1__blk898) + (assign31500_e46367 * var_t1__blk898_dn17)),)
    } else {
        (var_qiuld, var_qiuld_dn0, var_qiuld_dn2, var_qiuld_dn6, var_qiuld_dn7, var_qiuld_dn10, var_qiuld_dn11, var_qiuld_dn12, var_qiuld_dn17,)
    }
};
        var_qiuld = assign31500_e46371;
        var_qiuld_dn0 = assign31500_e46371_d_n0;
        var_qiuld_dn2 = assign31500_e46371_d_n2;
        var_qiuld_dn6 = assign31500_e46371_d_n6;
        var_qiuld_dn7 = assign31500_e46371_d_n7;
        var_qiuld_dn10 = assign31500_e46371_d_n10;
        var_qiuld_dn11 = assign31500_e46371_d_n11;
        var_qiuld_dn12 = assign31500_e46371_d_n12;
        var_qiuld_dn17 = assign31500_e46371_d_n17;

        let (assign31510_e46387, assign31510_e46387_d_n0, assign31510_e46387_d_n2, assign31510_e46387_d_n6, assign31510_e46387_d_n7, assign31510_e46387_d_n10, assign31510_e46387_d_n11, assign31510_e46387_d_n12, assign31510_e46387_d_n17,) = {
    if (((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) && (var_guard1004 == 0.0)) && (var_guard1011 != 0.0)) {
        let assign31510_e46385: f64 = (var_qbuld + var_qiuld);
        (assign31510_e46385, (var_qbuld_dn0 + var_qiuld_dn0), (var_qbuld_dn2 + var_qiuld_dn2), (var_qbuld_dn6 + var_qiuld_dn6), (var_qbuld_dn7 + var_qiuld_dn7), (var_qbuld_dn10 + var_qiuld_dn10), (var_qbuld_dn11 + var_qiuld_dn11), (var_qbuld_dn12 + var_qiuld_dn12), (var_qbuld_dn17 + var_qiuld_dn17),)
    } else {
        (var_qsuld, var_qsuld_dn0, var_qsuld_dn2, var_qsuld_dn6, var_qsuld_dn7, var_qsuld_dn10, var_qsuld_dn11, var_qsuld_dn12, var_qsuld_dn17,)
    }
};
        var_qsuld = assign31510_e46387;
        var_qsuld_dn0 = assign31510_e46387_d_n0;
        var_qsuld_dn2 = assign31510_e46387_d_n2;
        var_qsuld_dn6 = assign31510_e46387_d_n6;
        var_qsuld_dn7 = assign31510_e46387_d_n7;
        var_qsuld_dn10 = assign31510_e46387_d_n10;
        var_qsuld_dn11 = assign31510_e46387_d_n11;
        var_qsuld_dn12 = assign31510_e46387_d_n12;
        var_qsuld_dn17 = assign31510_e46387_d_n17;

        let (assign31520_e46398, assign31520_e46398_d_n0, assign31520_e46398_d_n2, assign31520_e46398_d_n6, assign31520_e46398_d_n7, assign31520_e46398_d_n10, assign31520_e46398_d_n11, assign31520_e46398_d_n12, assign31520_e46398_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) {
        let assign31520_e46396: f64 = (var_qsuld - var_qbuld);
        (assign31520_e46396, (var_qsuld_dn0 - var_qbuld_dn0), (var_qsuld_dn2 - var_qbuld_dn2), (var_qsuld_dn6 - var_qbuld_dn6), (var_qsuld_dn7 - var_qbuld_dn7), (var_qsuld_dn10 - var_qbuld_dn10), (var_qsuld_dn11 - var_qbuld_dn11), (var_qsuld_dn12 - var_qbuld_dn12), (var_qsuld_dn17 - var_qbuld_dn17),)
    } else {
        (var_qiuld, var_qiuld_dn0, var_qiuld_dn2, var_qiuld_dn6, var_qiuld_dn7, var_qiuld_dn10, var_qiuld_dn11, var_qiuld_dn12, var_qiuld_dn17,)
    }
};
        var_qiuld = assign31520_e46398;
        var_qiuld_dn0 = assign31520_e46398_d_n0;
        var_qiuld_dn2 = assign31520_e46398_d_n2;
        var_qiuld_dn6 = assign31520_e46398_d_n6;
        var_qiuld_dn7 = assign31520_e46398_d_n7;
        var_qiuld_dn10 = assign31520_e46398_d_n10;
        var_qiuld_dn11 = assign31520_e46398_d_n11;
        var_qiuld_dn12 = assign31520_e46398_d_n12;
        var_qiuld_dn17 = assign31520_e46398_d_n17;

        let (assign31530_e46416, assign31530_e46416_d_n0, assign31530_e46416_d_n2, assign31530_e46416_d_n6, assign31530_e46416_d_n7, assign31530_e46416_d_n10, assign31530_e46416_d_n11, assign31530_e46416_d_n12, assign31530_e46416_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) {
        let (assign31530_e46414,) = {
            if (p.p43 == 1.0) {
                let assign31530_e46410: f64 = (var_w_dioscv * var_lov);
                (assign31530_e46410,)
            } else {
                let assign31530_e46413: f64 = (var_weffcv_nf * var_lov);
                (assign31530_e46413,)
            }
        };
        (assign31530_e46414, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4__blk901, var_t4__blk901_dn0, var_t4__blk901_dn2, var_t4__blk901_dn6, var_t4__blk901_dn7, var_t4__blk901_dn10, var_t4__blk901_dn11, var_t4__blk901_dn12, var_t4__blk901_dn17,)
    }
};
        var_t4__blk901 = assign31530_e46416;
        var_t4__blk901_dn0 = assign31530_e46416_d_n0;
        var_t4__blk901_dn2 = assign31530_e46416_d_n2;
        var_t4__blk901_dn6 = assign31530_e46416_d_n6;
        var_t4__blk901_dn7 = assign31530_e46416_d_n7;
        var_t4__blk901_dn10 = assign31530_e46416_d_n10;
        var_t4__blk901_dn11 = assign31530_e46416_d_n11;
        var_t4__blk901_dn12 = assign31530_e46416_d_n12;
        var_t4__blk901_dn17 = assign31530_e46416_d_n17;

        let assign31540_e46427: f64 = if (((var_flg_overs__blk916 != 0.0) && (p.p43 == 0.0)) || ((var_flg_ovloops__blk914 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        var_guard1020 = assign31540_e46427;

        let (assign31550_e46440, assign31550_e46440_d_n0, assign31550_e46440_d_n2, assign31550_e46440_d_n6, assign31550_e46440_d_n7, assign31550_e46440_d_n10, assign31550_e46440_d_n11, assign31550_e46440_d_n12, assign31550_e46440_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) && (var_guard1020 != 0.0)) {
        let assign31550_e46438: f64 = (var_t4__blk901 * var_qsuld);
        (assign31550_e46438, ((var_t4__blk901_dn0 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn0)), ((var_t4__blk901_dn2 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn2)), ((var_t4__blk901_dn6 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn6)), ((var_t4__blk901_dn7 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn7)), ((var_t4__blk901_dn10 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn10)), ((var_t4__blk901_dn11 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn11)), ((var_t4__blk901_dn12 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn12)), ((var_t4__blk901_dn17 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn17)),)
    } else {
        (var_qovs, var_qovs_dn0, var_qovs_dn2, var_qovs_dn6, var_qovs_dn7, var_qovs_dn10, var_qovs_dn11, var_qovs_dn12, var_qovs_dn17,)
    }
};
        var_qovs = assign31550_e46440;
        var_qovs_dn0 = assign31550_e46440_d_n0;
        var_qovs_dn2 = assign31550_e46440_d_n2;
        var_qovs_dn6 = assign31550_e46440_d_n6;
        var_qovs_dn7 = assign31550_e46440_d_n7;
        var_qovs_dn10 = assign31550_e46440_d_n10;
        var_qovs_dn11 = assign31550_e46440_d_n11;
        var_qovs_dn12 = assign31550_e46440_d_n12;
        var_qovs_dn17 = assign31550_e46440_d_n17;

        let (assign31560_e46453, assign31560_e46453_d_n0, assign31560_e46453_d_n2, assign31560_e46453_d_n6, assign31560_e46453_d_n7, assign31560_e46453_d_n10, assign31560_e46453_d_n11, assign31560_e46453_d_n12, assign31560_e46453_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) && (var_guard1020 != 0.0)) {
        let assign31560_e46451: f64 = (var_t4__blk901 * var_qbuld);
        (assign31560_e46451, ((var_t4__blk901_dn0 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn0)), ((var_t4__blk901_dn2 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn2)), ((var_t4__blk901_dn6 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn6)), ((var_t4__blk901_dn7 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn7)), ((var_t4__blk901_dn10 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn10)), ((var_t4__blk901_dn11 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn11)), ((var_t4__blk901_dn12 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn12)), ((var_t4__blk901_dn17 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn17)),)
    } else {
        (var_qbsld, var_qbsld_dn0, var_qbsld_dn2, var_qbsld_dn6, var_qbsld_dn7, var_qbsld_dn10, var_qbsld_dn11, var_qbsld_dn12, var_qbsld_dn17,)
    }
};
        var_qbsld = assign31560_e46453;
        var_qbsld_dn0 = assign31560_e46453_d_n0;
        var_qbsld_dn2 = assign31560_e46453_d_n2;
        var_qbsld_dn6 = assign31560_e46453_d_n6;
        var_qbsld_dn7 = assign31560_e46453_d_n7;
        var_qbsld_dn10 = assign31560_e46453_d_n10;
        var_qbsld_dn11 = assign31560_e46453_d_n11;
        var_qbsld_dn12 = assign31560_e46453_d_n12;
        var_qbsld_dn17 = assign31560_e46453_d_n17;

        let assign31570_e46464: f64 = if (((var_flg_overd__blk917 != 0.0) && (p.p43 == 0.0)) || ((var_flg_ovloopd__blk915 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        var_guard1021 = assign31570_e46464;

        let (assign31580_e46477, assign31580_e46477_d_n0, assign31580_e46477_d_n2, assign31580_e46477_d_n6, assign31580_e46477_d_n7, assign31580_e46477_d_n10, assign31580_e46477_d_n11, assign31580_e46477_d_n12, assign31580_e46477_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) && (var_guard1021 != 0.0)) {
        let assign31580_e46475: f64 = (var_t4__blk901 * var_qsuld);
        (assign31580_e46475, ((var_t4__blk901_dn0 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn0)), ((var_t4__blk901_dn2 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn2)), ((var_t4__blk901_dn6 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn6)), ((var_t4__blk901_dn7 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn7)), ((var_t4__blk901_dn10 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn10)), ((var_t4__blk901_dn11 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn11)), ((var_t4__blk901_dn12 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn12)), ((var_t4__blk901_dn17 * var_qsuld) + (var_t4__blk901 * var_qsuld_dn17)),)
    } else {
        (var_qovd, var_qovd_dn0, var_qovd_dn2, var_qovd_dn6, var_qovd_dn7, var_qovd_dn10, var_qovd_dn11, var_qovd_dn12, var_qovd_dn17,)
    }
};
        var_qovd = assign31580_e46477;
        var_qovd_dn0 = assign31580_e46477_d_n0;
        var_qovd_dn2 = assign31580_e46477_d_n2;
        var_qovd_dn6 = assign31580_e46477_d_n6;
        var_qovd_dn7 = assign31580_e46477_d_n7;
        var_qovd_dn10 = assign31580_e46477_d_n10;
        var_qovd_dn11 = assign31580_e46477_d_n11;
        var_qovd_dn12 = assign31580_e46477_d_n12;
        var_qovd_dn17 = assign31580_e46477_d_n17;

        let (assign31590_e46490, assign31590_e46490_d_n0, assign31590_e46490_d_n2, assign31590_e46490_d_n6, assign31590_e46490_d_n7, assign31590_e46490_d_n10, assign31590_e46490_d_n11, assign31590_e46490_d_n12, assign31590_e46490_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_guard981 == 0.0)) && (var_guard1021 != 0.0)) {
        let assign31590_e46488: f64 = (var_t4__blk901 * var_qbuld);
        (assign31590_e46488, ((var_t4__blk901_dn0 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn0)), ((var_t4__blk901_dn2 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn2)), ((var_t4__blk901_dn6 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn6)), ((var_t4__blk901_dn7 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn7)), ((var_t4__blk901_dn10 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn10)), ((var_t4__blk901_dn11 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn11)), ((var_t4__blk901_dn12 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn12)), ((var_t4__blk901_dn17 * var_qbuld) + (var_t4__blk901 * var_qbuld_dn17)),)
    } else {
        (var_qbdld, var_qbdld_dn0, var_qbdld_dn2, var_qbdld_dn6, var_qbdld_dn7, var_qbdld_dn10, var_qbdld_dn11, var_qbdld_dn12, var_qbdld_dn17,)
    }
};
        var_qbdld = assign31590_e46490;
        var_qbdld_dn0 = assign31590_e46490_d_n0;
        var_qbdld_dn2 = assign31590_e46490_d_n2;
        var_qbdld_dn6 = assign31590_e46490_d_n6;
        var_qbdld_dn7 = assign31590_e46490_d_n7;
        var_qbdld_dn10 = assign31590_e46490_d_n10;
        var_qbdld_dn11 = assign31590_e46490_d_n11;
        var_qbdld_dn12 = assign31590_e46490_d_n12;
        var_qbdld_dn17 = assign31590_e46490_d_n17;

        let (assign31600_e46502,) = {
    if ((p.p24 != 0.0) && (var_guard980 != 0.0)) {
        let assign31600_e46496: f64 = (var_modervs * var_cgso_given);
        let assign31600_e46499: f64 = (var_modenml * var_cgdo_given);
        let assign31600_e46500: f64 = (assign31600_e46496 + assign31600_e46499);
        (assign31600_e46500,)
    } else {
        (var_flg_overgiven,)
    }
};
        var_flg_overgiven = assign31600_e46502;

        let (assign31610_e46516, assign31610_e46516_d_n0, assign31610_e46516_d_n2, assign31610_e46516_d_n6, assign31610_e46516_d_n7, assign31610_e46516_d_n10, assign31610_e46516_d_n11, assign31610_e46516_d_n12, assign31610_e46516_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_flg_overgiven != 0.0)) {
        let assign31610_e46510: f64 = (var_modervs * p.p170);
        let assign31610_e46513: f64 = (var_modenml * p.p169);
        let assign31610_e46514: f64 = (assign31610_e46510 + assign31610_e46513);
        (assign31610_e46514, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31610_e46516;
        var_cgdoe_dn0 = assign31610_e46516_d_n0;
        var_cgdoe_dn2 = assign31610_e46516_d_n2;
        var_cgdoe_dn6 = assign31610_e46516_d_n6;
        var_cgdoe_dn7 = assign31610_e46516_d_n7;
        var_cgdoe_dn10 = assign31610_e46516_d_n10;
        var_cgdoe_dn11 = assign31610_e46516_d_n11;
        var_cgdoe_dn12 = assign31610_e46516_d_n12;
        var_cgdoe_dn17 = assign31610_e46516_d_n17;

        let assign31620_e46519: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1022 = assign31620_e46519;

        let (assign31630_e46535, assign31630_e46535_d_n0, assign31630_e46535_d_n2, assign31630_e46535_d_n6, assign31630_e46535_d_n7, assign31630_e46535_d_n10, assign31630_e46535_d_n11, assign31630_e46535_d_n12, assign31630_e46535_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_flg_overgiven != 0.0)) && (var_guard1022 != 0.0)) {
        let assign31630_e46529: f64 = (var_modervs * var_w_dioscv);
        let assign31630_e46532: f64 = (var_modenml * var_w_diodcv);
        let assign31630_e46533: f64 = (assign31630_e46529 + assign31630_e46532);
        (assign31630_e46533, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk898, var_t1__blk898_dn0, var_t1__blk898_dn2, var_t1__blk898_dn6, var_t1__blk898_dn7, var_t1__blk898_dn10, var_t1__blk898_dn11, var_t1__blk898_dn12, var_t1__blk898_dn17,)
    }
};
        var_t1__blk898 = assign31630_e46535;
        var_t1__blk898_dn0 = assign31630_e46535_d_n0;
        var_t1__blk898_dn2 = assign31630_e46535_d_n2;
        var_t1__blk898_dn6 = assign31630_e46535_d_n6;
        var_t1__blk898_dn7 = assign31630_e46535_d_n7;
        var_t1__blk898_dn10 = assign31630_e46535_d_n10;
        var_t1__blk898_dn11 = assign31630_e46535_d_n11;
        var_t1__blk898_dn12 = assign31630_e46535_d_n12;
        var_t1__blk898_dn17 = assign31630_e46535_d_n17;

        let (assign31640_e46548, assign31640_e46548_d_n0, assign31640_e46548_d_n2, assign31640_e46548_d_n6, assign31640_e46548_d_n7, assign31640_e46548_d_n10, assign31640_e46548_d_n11, assign31640_e46548_d_n12, assign31640_e46548_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_flg_overgiven != 0.0)) && (var_guard1022 != 0.0)) {
        let assign31640_e46545: f64 = (-var_t1__blk898);
        let assign31640_e46546: f64 = (var_cgdoe * assign31640_e46545);
        (assign31640_e46546, ((var_cgdoe_dn0 * assign31640_e46545) + (var_cgdoe * (-var_t1__blk898_dn0))), ((var_cgdoe_dn2 * assign31640_e46545) + (var_cgdoe * (-var_t1__blk898_dn2))), ((var_cgdoe_dn6 * assign31640_e46545) + (var_cgdoe * (-var_t1__blk898_dn6))), ((var_cgdoe_dn7 * assign31640_e46545) + (var_cgdoe * (-var_t1__blk898_dn7))), ((var_cgdoe_dn10 * assign31640_e46545) + (var_cgdoe * (-var_t1__blk898_dn10))), ((var_cgdoe_dn11 * assign31640_e46545) + (var_cgdoe * (-var_t1__blk898_dn11))), ((var_cgdoe_dn12 * assign31640_e46545) + (var_cgdoe * (-var_t1__blk898_dn12))), ((var_cgdoe_dn17 * assign31640_e46545) + (var_cgdoe * (-var_t1__blk898_dn17))),)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31640_e46548;
        var_cgdoe_dn0 = assign31640_e46548_d_n0;
        var_cgdoe_dn2 = assign31640_e46548_d_n2;
        var_cgdoe_dn6 = assign31640_e46548_d_n6;
        var_cgdoe_dn7 = assign31640_e46548_d_n7;
        var_cgdoe_dn10 = assign31640_e46548_d_n10;
        var_cgdoe_dn11 = assign31640_e46548_d_n11;
        var_cgdoe_dn12 = assign31640_e46548_d_n12;
        var_cgdoe_dn17 = assign31640_e46548_d_n17;

        let (assign31650_e46562, assign31650_e46562_d_n0, assign31650_e46562_d_n2, assign31650_e46562_d_n6, assign31650_e46562_d_n7, assign31650_e46562_d_n10, assign31650_e46562_d_n11, assign31650_e46562_d_n12, assign31650_e46562_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_flg_overgiven != 0.0)) && (var_guard1022 == 0.0)) {
        let assign31650_e46559: f64 = (-var_weffcv_nf);
        let assign31650_e46560: f64 = (var_cgdoe * assign31650_e46559);
        (assign31650_e46560, (var_cgdoe_dn0 * assign31650_e46559), (var_cgdoe_dn2 * assign31650_e46559), (var_cgdoe_dn6 * assign31650_e46559), (var_cgdoe_dn7 * assign31650_e46559), (var_cgdoe_dn10 * assign31650_e46559), (var_cgdoe_dn11 * assign31650_e46559), (var_cgdoe_dn12 * assign31650_e46559), (var_cgdoe_dn17 * assign31650_e46559),)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31650_e46562;
        var_cgdoe_dn0 = assign31650_e46562_d_n0;
        var_cgdoe_dn2 = assign31650_e46562_d_n2;
        var_cgdoe_dn6 = assign31650_e46562_d_n6;
        var_cgdoe_dn7 = assign31650_e46562_d_n7;
        var_cgdoe_dn10 = assign31650_e46562_d_n10;
        var_cgdoe_dn11 = assign31650_e46562_d_n11;
        var_cgdoe_dn12 = assign31650_e46562_d_n12;
        var_cgdoe_dn17 = assign31650_e46562_d_n17;

        let (assign31660_e46577, assign31660_e46577_d_n0, assign31660_e46577_d_n2, assign31660_e46577_d_n6, assign31660_e46577_d_n7, assign31660_e46577_d_n10, assign31660_e46577_d_n11, assign31660_e46577_d_n12, assign31660_e46577_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_flg_overgiven != 0.0)) {
        let assign31660_e46570: f64 = (-var_cgdoe);
        let assign31660_e46573: f64 = (var_vgs - var_vds);
        let assign31660_e46574: f64 = (assign31660_e46570 * assign31660_e46573);
        let assign31660_e46575: f64 = (var_qgod + assign31660_e46574);
        (assign31660_e46575, (var_qgod_dn0 + (((-var_cgdoe_dn0) * assign31660_e46573) + (assign31660_e46570 * (-var_vds_dn0)))), (var_qgod_dn2 + (((-var_cgdoe_dn2) * assign31660_e46573) + (assign31660_e46570 * (-var_vds_dn2)))), (var_qgod_dn6 + (((-var_cgdoe_dn6) * assign31660_e46573) + (assign31660_e46570 * (var_vgs_dn6 - var_vds_dn6)))), (var_qgod_dn7 + (((-var_cgdoe_dn7) * assign31660_e46573) + (assign31660_e46570 * (var_vgs_dn7 - var_vds_dn7)))), (var_qgod_dn10 + (((-var_cgdoe_dn10) * assign31660_e46573) + (assign31660_e46570 * (-var_vds_dn10)))), (var_qgod_dn11 + (((-var_cgdoe_dn11) * assign31660_e46573) + (assign31660_e46570 * (var_vgs_dn11 - var_vds_dn11)))), (var_qgod_dn12 + (((-var_cgdoe_dn12) * assign31660_e46573) + (assign31660_e46570 * (-var_vds_dn12)))), (var_qgod_dn17 + (((-var_cgdoe_dn17) * assign31660_e46573) + (assign31660_e46570 * (-var_vds_dn17)))),)
    } else {
        (var_qgod, var_qgod_dn0, var_qgod_dn2, var_qgod_dn6, var_qgod_dn7, var_qgod_dn10, var_qgod_dn11, var_qgod_dn12, var_qgod_dn17,)
    }
};
        var_qgod = assign31660_e46577;
        var_qgod_dn0 = assign31660_e46577_d_n0;
        var_qgod_dn2 = assign31660_e46577_d_n2;
        var_qgod_dn6 = assign31660_e46577_d_n6;
        var_qgod_dn7 = assign31660_e46577_d_n7;
        var_qgod_dn10 = assign31660_e46577_d_n10;
        var_qgod_dn11 = assign31660_e46577_d_n11;
        var_qgod_dn12 = assign31660_e46577_d_n12;
        var_qgod_dn17 = assign31660_e46577_d_n17;

        let (assign31670_e46589,) = {
    if ((p.p24 != 0.0) && (var_guard980 != 0.0)) {
        let assign31670_e46583: f64 = (var_modenml * var_cgso_given);
        let assign31670_e46586: f64 = (var_modervs * var_cgdo_given);
        let assign31670_e46587: f64 = (assign31670_e46583 + assign31670_e46586);
        (assign31670_e46587,)
    } else {
        (var_flg_overgiven,)
    }
};
        var_flg_overgiven = assign31670_e46589;

        let (assign31680_e46603, assign31680_e46603_d_n0, assign31680_e46603_d_n2, assign31680_e46603_d_n6, assign31680_e46603_d_n7, assign31680_e46603_d_n10, assign31680_e46603_d_n11, assign31680_e46603_d_n12, assign31680_e46603_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_flg_overgiven != 0.0)) {
        let assign31680_e46597: f64 = (var_modenml * p.p170);
        let assign31680_e46600: f64 = (var_modervs * p.p169);
        let assign31680_e46601: f64 = (assign31680_e46597 + assign31680_e46600);
        (assign31680_e46601, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31680_e46603;
        var_cgsoe_dn0 = assign31680_e46603_d_n0;
        var_cgsoe_dn2 = assign31680_e46603_d_n2;
        var_cgsoe_dn6 = assign31680_e46603_d_n6;
        var_cgsoe_dn7 = assign31680_e46603_d_n7;
        var_cgsoe_dn10 = assign31680_e46603_d_n10;
        var_cgsoe_dn11 = assign31680_e46603_d_n11;
        var_cgsoe_dn12 = assign31680_e46603_d_n12;
        var_cgsoe_dn17 = assign31680_e46603_d_n17;

        let assign31690_e46606: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1023 = assign31690_e46606;

        let (assign31700_e46622, assign31700_e46622_d_n0, assign31700_e46622_d_n2, assign31700_e46622_d_n6, assign31700_e46622_d_n7, assign31700_e46622_d_n10, assign31700_e46622_d_n11, assign31700_e46622_d_n12, assign31700_e46622_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_flg_overgiven != 0.0)) && (var_guard1023 != 0.0)) {
        let assign31700_e46616: f64 = (var_modenml * var_w_dioscv);
        let assign31700_e46619: f64 = (var_modervs * var_w_diodcv);
        let assign31700_e46620: f64 = (assign31700_e46616 + assign31700_e46619);
        (assign31700_e46620, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk898, var_t1__blk898_dn0, var_t1__blk898_dn2, var_t1__blk898_dn6, var_t1__blk898_dn7, var_t1__blk898_dn10, var_t1__blk898_dn11, var_t1__blk898_dn12, var_t1__blk898_dn17,)
    }
};
        var_t1__blk898 = assign31700_e46622;
        var_t1__blk898_dn0 = assign31700_e46622_d_n0;
        var_t1__blk898_dn2 = assign31700_e46622_d_n2;
        var_t1__blk898_dn6 = assign31700_e46622_d_n6;
        var_t1__blk898_dn7 = assign31700_e46622_d_n7;
        var_t1__blk898_dn10 = assign31700_e46622_d_n10;
        var_t1__blk898_dn11 = assign31700_e46622_d_n11;
        var_t1__blk898_dn12 = assign31700_e46622_d_n12;
        var_t1__blk898_dn17 = assign31700_e46622_d_n17;

        let (assign31710_e46635, assign31710_e46635_d_n0, assign31710_e46635_d_n2, assign31710_e46635_d_n6, assign31710_e46635_d_n7, assign31710_e46635_d_n10, assign31710_e46635_d_n11, assign31710_e46635_d_n12, assign31710_e46635_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_flg_overgiven != 0.0)) && (var_guard1023 != 0.0)) {
        let assign31710_e46632: f64 = (-var_t1__blk898);
        let assign31710_e46633: f64 = (var_cgsoe * assign31710_e46632);
        (assign31710_e46633, ((var_cgsoe_dn0 * assign31710_e46632) + (var_cgsoe * (-var_t1__blk898_dn0))), ((var_cgsoe_dn2 * assign31710_e46632) + (var_cgsoe * (-var_t1__blk898_dn2))), ((var_cgsoe_dn6 * assign31710_e46632) + (var_cgsoe * (-var_t1__blk898_dn6))), ((var_cgsoe_dn7 * assign31710_e46632) + (var_cgsoe * (-var_t1__blk898_dn7))), ((var_cgsoe_dn10 * assign31710_e46632) + (var_cgsoe * (-var_t1__blk898_dn10))), ((var_cgsoe_dn11 * assign31710_e46632) + (var_cgsoe * (-var_t1__blk898_dn11))), ((var_cgsoe_dn12 * assign31710_e46632) + (var_cgsoe * (-var_t1__blk898_dn12))), ((var_cgsoe_dn17 * assign31710_e46632) + (var_cgsoe * (-var_t1__blk898_dn17))),)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31710_e46635;
        var_cgsoe_dn0 = assign31710_e46635_d_n0;
        var_cgsoe_dn2 = assign31710_e46635_d_n2;
        var_cgsoe_dn6 = assign31710_e46635_d_n6;
        var_cgsoe_dn7 = assign31710_e46635_d_n7;
        var_cgsoe_dn10 = assign31710_e46635_d_n10;
        var_cgsoe_dn11 = assign31710_e46635_d_n11;
        var_cgsoe_dn12 = assign31710_e46635_d_n12;
        var_cgsoe_dn17 = assign31710_e46635_d_n17;

        let (assign31720_e46649, assign31720_e46649_d_n0, assign31720_e46649_d_n2, assign31720_e46649_d_n6, assign31720_e46649_d_n7, assign31720_e46649_d_n10, assign31720_e46649_d_n11, assign31720_e46649_d_n12, assign31720_e46649_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_flg_overgiven != 0.0)) && (var_guard1023 == 0.0)) {
        let assign31720_e46646: f64 = (-var_weffcv_nf);
        let assign31720_e46647: f64 = (var_cgsoe * assign31720_e46646);
        (assign31720_e46647, (var_cgsoe_dn0 * assign31720_e46646), (var_cgsoe_dn2 * assign31720_e46646), (var_cgsoe_dn6 * assign31720_e46646), (var_cgsoe_dn7 * assign31720_e46646), (var_cgsoe_dn10 * assign31720_e46646), (var_cgsoe_dn11 * assign31720_e46646), (var_cgsoe_dn12 * assign31720_e46646), (var_cgsoe_dn17 * assign31720_e46646),)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31720_e46649;
        var_cgsoe_dn0 = assign31720_e46649_d_n0;
        var_cgsoe_dn2 = assign31720_e46649_d_n2;
        var_cgsoe_dn6 = assign31720_e46649_d_n6;
        var_cgsoe_dn7 = assign31720_e46649_d_n7;
        var_cgsoe_dn10 = assign31720_e46649_d_n10;
        var_cgsoe_dn11 = assign31720_e46649_d_n11;
        var_cgsoe_dn12 = assign31720_e46649_d_n12;
        var_cgsoe_dn17 = assign31720_e46649_d_n17;

        let (assign31730_e46662, assign31730_e46662_d_n0, assign31730_e46662_d_n2, assign31730_e46662_d_n6, assign31730_e46662_d_n7, assign31730_e46662_d_n10, assign31730_e46662_d_n11, assign31730_e46662_d_n12, assign31730_e46662_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard980 != 0.0)) && (var_flg_overgiven != 0.0)) {
        let assign31730_e46657: f64 = (-var_cgsoe);
        let assign31730_e46659: f64 = (assign31730_e46657 * var_vgs);
        let assign31730_e46660: f64 = (var_qgos + assign31730_e46659);
        (assign31730_e46660, (var_qgos_dn0 + ((-var_cgsoe_dn0) * var_vgs)), (var_qgos_dn2 + ((-var_cgsoe_dn2) * var_vgs)), (var_qgos_dn6 + (((-var_cgsoe_dn6) * var_vgs) + (assign31730_e46657 * var_vgs_dn6))), (var_qgos_dn7 + (((-var_cgsoe_dn7) * var_vgs) + (assign31730_e46657 * var_vgs_dn7))), (var_qgos_dn10 + ((-var_cgsoe_dn10) * var_vgs)), (var_qgos_dn11 + (((-var_cgsoe_dn11) * var_vgs) + (assign31730_e46657 * var_vgs_dn11))), (var_qgos_dn12 + ((-var_cgsoe_dn12) * var_vgs)), (var_qgos_dn17 + ((-var_cgsoe_dn17) * var_vgs)),)
    } else {
        (var_qgos, var_qgos_dn0, var_qgos_dn2, var_qgos_dn6, var_qgos_dn7, var_qgos_dn10, var_qgos_dn11, var_qgos_dn12, var_qgos_dn17,)
    }
};
        var_qgos = assign31730_e46662;
        var_qgos_dn0 = assign31730_e46662_d_n0;
        var_qgos_dn2 = assign31730_e46662_d_n2;
        var_qgos_dn6 = assign31730_e46662_d_n6;
        var_qgos_dn7 = assign31730_e46662_d_n7;
        var_qgos_dn10 = assign31730_e46662_d_n10;
        var_qgos_dn11 = assign31730_e46662_d_n11;
        var_qgos_dn12 = assign31730_e46662_d_n12;
        var_qgos_dn17 = assign31730_e46662_d_n17;

        let assign31740_e46675: f64 = if (((var_mode == 1.0) && (var_cgdo_given == 0.0)) || ((var_mode != 1.0) && (var_cgso_given == 0.0))) { 1.0 } else { 0.0 };
        var_guard1024 = assign31740_e46675;

        let assign31750_e46678: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1025 = assign31750_e46678;

        *var_cgdoe_slot = var_cgdoe;
        *var_cgdoe_dn0_slot = var_cgdoe_dn0;
        *var_cgdoe_dn10_slot = var_cgdoe_dn10;
        *var_cgdoe_dn11_slot = var_cgdoe_dn11;
        *var_cgdoe_dn12_slot = var_cgdoe_dn12;
        *var_cgdoe_dn17_slot = var_cgdoe_dn17;
        *var_cgdoe_dn2_slot = var_cgdoe_dn2;
        *var_cgdoe_dn6_slot = var_cgdoe_dn6;
        *var_cgdoe_dn7_slot = var_cgdoe_dn7;
        *var_cgsoe_slot = var_cgsoe;
        *var_cgsoe_dn0_slot = var_cgsoe_dn0;
        *var_cgsoe_dn10_slot = var_cgsoe_dn10;
        *var_cgsoe_dn11_slot = var_cgsoe_dn11;
        *var_cgsoe_dn12_slot = var_cgsoe_dn12;
        *var_cgsoe_dn17_slot = var_cgsoe_dn17;
        *var_cgsoe_dn2_slot = var_cgsoe_dn2;
        *var_cgsoe_dn6_slot = var_cgsoe_dn6;
        *var_cgsoe_dn7_slot = var_cgsoe_dn7;
        *var_flg_overgiven_slot = var_flg_overgiven;
        *var_guard1018_slot = var_guard1018;
        *var_guard1020_slot = var_guard1020;
        *var_guard1021_slot = var_guard1021;
        *var_guard1022_slot = var_guard1022;
        *var_guard1023_slot = var_guard1023;
        *var_guard1024_slot = var_guard1024;
        *var_guard1025_slot = var_guard1025;
        *var_qbdld_slot = var_qbdld;
        *var_qbdld_dn0_slot = var_qbdld_dn0;
        *var_qbdld_dn10_slot = var_qbdld_dn10;
        *var_qbdld_dn11_slot = var_qbdld_dn11;
        *var_qbdld_dn12_slot = var_qbdld_dn12;
        *var_qbdld_dn17_slot = var_qbdld_dn17;
        *var_qbdld_dn2_slot = var_qbdld_dn2;
        *var_qbdld_dn6_slot = var_qbdld_dn6;
        *var_qbdld_dn7_slot = var_qbdld_dn7;
        *var_qbsld_slot = var_qbsld;
        *var_qbsld_dn0_slot = var_qbsld_dn0;
        *var_qbsld_dn10_slot = var_qbsld_dn10;
        *var_qbsld_dn11_slot = var_qbsld_dn11;
        *var_qbsld_dn12_slot = var_qbsld_dn12;
        *var_qbsld_dn17_slot = var_qbsld_dn17;
        *var_qbsld_dn2_slot = var_qbsld_dn2;
        *var_qbsld_dn6_slot = var_qbsld_dn6;
        *var_qbsld_dn7_slot = var_qbsld_dn7;
        *var_qbuld_slot = var_qbuld;
        *var_qbuld_dn0_slot = var_qbuld_dn0;
        *var_qbuld_dn10_slot = var_qbuld_dn10;
        *var_qbuld_dn11_slot = var_qbuld_dn11;
        *var_qbuld_dn12_slot = var_qbuld_dn12;
        *var_qbuld_dn17_slot = var_qbuld_dn17;
        *var_qbuld_dn2_slot = var_qbuld_dn2;
        *var_qbuld_dn6_slot = var_qbuld_dn6;
        *var_qbuld_dn7_slot = var_qbuld_dn7;
        *var_qgod_slot = var_qgod;
        *var_qgod_dn0_slot = var_qgod_dn0;
        *var_qgod_dn10_slot = var_qgod_dn10;
        *var_qgod_dn11_slot = var_qgod_dn11;
        *var_qgod_dn12_slot = var_qgod_dn12;
        *var_qgod_dn17_slot = var_qgod_dn17;
        *var_qgod_dn2_slot = var_qgod_dn2;
        *var_qgod_dn6_slot = var_qgod_dn6;
        *var_qgod_dn7_slot = var_qgod_dn7;
        *var_qgos_slot = var_qgos;
        *var_qgos_dn0_slot = var_qgos_dn0;
        *var_qgos_dn10_slot = var_qgos_dn10;
        *var_qgos_dn11_slot = var_qgos_dn11;
        *var_qgos_dn12_slot = var_qgos_dn12;
        *var_qgos_dn17_slot = var_qgos_dn17;
        *var_qgos_dn2_slot = var_qgos_dn2;
        *var_qgos_dn6_slot = var_qgos_dn6;
        *var_qgos_dn7_slot = var_qgos_dn7;
        *var_qiuld_slot = var_qiuld;
        *var_qiuld_dn0_slot = var_qiuld_dn0;
        *var_qiuld_dn10_slot = var_qiuld_dn10;
        *var_qiuld_dn11_slot = var_qiuld_dn11;
        *var_qiuld_dn12_slot = var_qiuld_dn12;
        *var_qiuld_dn17_slot = var_qiuld_dn17;
        *var_qiuld_dn2_slot = var_qiuld_dn2;
        *var_qiuld_dn6_slot = var_qiuld_dn6;
        *var_qiuld_dn7_slot = var_qiuld_dn7;
        *var_qovd_slot = var_qovd;
        *var_qovd_dn0_slot = var_qovd_dn0;
        *var_qovd_dn10_slot = var_qovd_dn10;
        *var_qovd_dn11_slot = var_qovd_dn11;
        *var_qovd_dn12_slot = var_qovd_dn12;
        *var_qovd_dn17_slot = var_qovd_dn17;
        *var_qovd_dn2_slot = var_qovd_dn2;
        *var_qovd_dn6_slot = var_qovd_dn6;
        *var_qovd_dn7_slot = var_qovd_dn7;
        *var_qovs_slot = var_qovs;
        *var_qovs_dn0_slot = var_qovs_dn0;
        *var_qovs_dn10_slot = var_qovs_dn10;
        *var_qovs_dn11_slot = var_qovs_dn11;
        *var_qovs_dn12_slot = var_qovs_dn12;
        *var_qovs_dn17_slot = var_qovs_dn17;
        *var_qovs_dn2_slot = var_qovs_dn2;
        *var_qovs_dn6_slot = var_qovs_dn6;
        *var_qovs_dn7_slot = var_qovs_dn7;
        *var_qsuld_slot = var_qsuld;
        *var_qsuld_dn0_slot = var_qsuld_dn0;
        *var_qsuld_dn10_slot = var_qsuld_dn10;
        *var_qsuld_dn11_slot = var_qsuld_dn11;
        *var_qsuld_dn12_slot = var_qsuld_dn12;
        *var_qsuld_dn17_slot = var_qsuld_dn17;
        *var_qsuld_dn2_slot = var_qsuld_dn2;
        *var_qsuld_dn6_slot = var_qsuld_dn6;
        *var_qsuld_dn7_slot = var_qsuld_dn7;
        *var_t1__blk898_slot = var_t1__blk898;
        *var_t1__blk898_dn0_slot = var_t1__blk898_dn0;
        *var_t1__blk898_dn10_slot = var_t1__blk898_dn10;
        *var_t1__blk898_dn11_slot = var_t1__blk898_dn11;
        *var_t1__blk898_dn12_slot = var_t1__blk898_dn12;
        *var_t1__blk898_dn17_slot = var_t1__blk898_dn17;
        *var_t1__blk898_dn2_slot = var_t1__blk898_dn2;
        *var_t1__blk898_dn6_slot = var_t1__blk898_dn6;
        *var_t1__blk898_dn7_slot = var_t1__blk898_dn7;
        *var_t4__blk901_slot = var_t4__blk901;
        *var_t4__blk901_dn0_slot = var_t4__blk901_dn0;
        *var_t4__blk901_dn10_slot = var_t4__blk901_dn10;
        *var_t4__blk901_dn11_slot = var_t4__blk901_dn11;
        *var_t4__blk901_dn12_slot = var_t4__blk901_dn12;
        *var_t4__blk901_dn17_slot = var_t4__blk901_dn17;
        *var_t4__blk901_dn2_slot = var_t4__blk901_dn2;
        *var_t4__blk901_dn6_slot = var_t4__blk901_dn6;
        *var_t4__blk901_dn7_slot = var_t4__blk901_dn7;
        *var_xi0__blk978_slot = var_xi0__blk978;
        *var_xi0__blk978_dn0_slot = var_xi0__blk978_dn0;
        *var_xi0__blk978_dn10_slot = var_xi0__blk978_dn10;
        *var_xi0__blk978_dn11_slot = var_xi0__blk978_dn11;
        *var_xi0__blk978_dn12_slot = var_xi0__blk978_dn12;
        *var_xi0__blk978_dn17_slot = var_xi0__blk978_dn17;
        *var_xi0__blk978_dn2_slot = var_xi0__blk978_dn2;
        *var_xi0__blk978_dn6_slot = var_xi0__blk978_dn6;
        *var_xi0__blk978_dn7_slot = var_xi0__blk978_dn7;
        *var_xi0p12__blk979_slot = var_xi0p12__blk979;
        *var_xi0p12__blk979_dn0_slot = var_xi0p12__blk979_dn0;
        *var_xi0p12__blk979_dn10_slot = var_xi0p12__blk979_dn10;
        *var_xi0p12__blk979_dn11_slot = var_xi0p12__blk979_dn11;
        *var_xi0p12__blk979_dn12_slot = var_xi0p12__blk979_dn12;
        *var_xi0p12__blk979_dn17_slot = var_xi0p12__blk979_dn17;
        *var_xi0p12__blk979_dn2_slot = var_xi0p12__blk979_dn2;
        *var_xi0p12__blk979_dn6_slot = var_xi0p12__blk979_dn6;
        *var_xi0p12__blk979_dn7_slot = var_xi0p12__blk979_dn7;
    }

    pub(super) fn stamp_transient_block_113(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn10: f64,
        var_beta_inv: f64,
        var_beta_inv_dn10: f64,
        var_betatnom: f64,
        var_cgdo_given: f64,
        var_cgso_given: f64,
        var_cox0__blk908: f64,
        var_eg: f64,
        var_eg_dn0: f64,
        var_eg_dn10: f64,
        var_eg_dn11: f64,
        var_eg_dn12: f64,
        var_eg_dn17: f64,
        var_eg_dn2: f64,
        var_eg_dn6: f64,
        var_eg_dn7: f64,
        var_egtnom: f64,
        var_guard1024: f64,
        var_guard1025: f64,
        var_guard980: f64,
        var_mode: f64,
        var_modenml: f64,
        var_modervs: f64,
        var_ttemp: f64,
        var_ttemp_dn10: f64,
        var_uc_tnom: f64,
        var_vbcd: f64,
        var_vbcd_dn12: f64,
        var_vbcd_dn6: f64,
        var_vbcs: f64,
        var_vbcs_dn12: f64,
        var_vbcs_dn7: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn17: f64,
        var_vds_dn2: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vgs: f64,
        var_vgs_dn11: f64,
        var_vgs_dn6: f64,
        var_vgs_dn7: f64,
        var_w_diod: f64,
        var_w_diodcv: f64,
        var_w_dios: f64,
        var_w_dioscv: f64,
        var_weffcv_nf: f64,
        var_cgdoe_slot: &mut f64,
        var_cgdoe_dn0_slot: &mut f64,
        var_cgdoe_dn10_slot: &mut f64,
        var_cgdoe_dn11_slot: &mut f64,
        var_cgdoe_dn12_slot: &mut f64,
        var_cgdoe_dn17_slot: &mut f64,
        var_cgdoe_dn2_slot: &mut f64,
        var_cgdoe_dn6_slot: &mut f64,
        var_cgdoe_dn7_slot: &mut f64,
        var_cgsoe_slot: &mut f64,
        var_cgsoe_dn0_slot: &mut f64,
        var_cgsoe_dn10_slot: &mut f64,
        var_cgsoe_dn11_slot: &mut f64,
        var_cgsoe_dn12_slot: &mut f64,
        var_cgsoe_dn17_slot: &mut f64,
        var_cgsoe_dn2_slot: &mut f64,
        var_cgsoe_dn6_slot: &mut f64,
        var_cgsoe_dn7_slot: &mut f64,
        var_guard1026_slot: &mut f64,
        var_guard1027_slot: &mut f64,
        var_guard1028_slot: &mut f64,
        var_guard1029_slot: &mut f64,
        var_guard1030_slot: &mut f64,
        var_guard1059_slot: &mut f64,
        var_isbd_slot: &mut f64,
        var_isbd2_slot: &mut f64,
        var_isbd2_dn0_slot: &mut f64,
        var_isbd2_dn10_slot: &mut f64,
        var_isbd2_dn11_slot: &mut f64,
        var_isbd2_dn12_slot: &mut f64,
        var_isbd2_dn17_slot: &mut f64,
        var_isbd2_dn2_slot: &mut f64,
        var_isbd2_dn6_slot: &mut f64,
        var_isbd2_dn7_slot: &mut f64,
        var_isbd_dn0_slot: &mut f64,
        var_isbd_dn10_slot: &mut f64,
        var_isbd_dn11_slot: &mut f64,
        var_isbd_dn12_slot: &mut f64,
        var_isbd_dn17_slot: &mut f64,
        var_isbd_dn2_slot: &mut f64,
        var_isbd_dn6_slot: &mut f64,
        var_isbd_dn7_slot: &mut f64,
        var_isbs_slot: &mut f64,
        var_isbs2_slot: &mut f64,
        var_isbs2_dn0_slot: &mut f64,
        var_isbs2_dn10_slot: &mut f64,
        var_isbs2_dn11_slot: &mut f64,
        var_isbs2_dn12_slot: &mut f64,
        var_isbs2_dn17_slot: &mut f64,
        var_isbs2_dn2_slot: &mut f64,
        var_isbs2_dn6_slot: &mut f64,
        var_isbs2_dn7_slot: &mut f64,
        var_isbs_dn0_slot: &mut f64,
        var_isbs_dn10_slot: &mut f64,
        var_isbs_dn11_slot: &mut f64,
        var_isbs_dn12_slot: &mut f64,
        var_isbs_dn17_slot: &mut f64,
        var_isbs_dn2_slot: &mut f64,
        var_isbs_dn6_slot: &mut f64,
        var_isbs_dn7_slot: &mut f64,
        var_js_slot: &mut f64,
        var_js2_slot: &mut f64,
        var_js2_dn0_slot: &mut f64,
        var_js2_dn10_slot: &mut f64,
        var_js2_dn11_slot: &mut f64,
        var_js2_dn12_slot: &mut f64,
        var_js2_dn17_slot: &mut f64,
        var_js2_dn2_slot: &mut f64,
        var_js2_dn6_slot: &mut f64,
        var_js2_dn7_slot: &mut f64,
        var_js_dn0_slot: &mut f64,
        var_js_dn10_slot: &mut f64,
        var_js_dn11_slot: &mut f64,
        var_js_dn12_slot: &mut f64,
        var_js_dn17_slot: &mut f64,
        var_js_dn2_slot: &mut f64,
        var_js_dn6_slot: &mut f64,
        var_js_dn7_slot: &mut f64,
        var_nvtm_slot: &mut f64,
        var_nvtm_dn10_slot: &mut f64,
        var_qgod_slot: &mut f64,
        var_qgod_dn0_slot: &mut f64,
        var_qgod_dn10_slot: &mut f64,
        var_qgod_dn11_slot: &mut f64,
        var_qgod_dn12_slot: &mut f64,
        var_qgod_dn17_slot: &mut f64,
        var_qgod_dn2_slot: &mut f64,
        var_qgod_dn6_slot: &mut f64,
        var_qgod_dn7_slot: &mut f64,
        var_qgos_slot: &mut f64,
        var_qgos_dn0_slot: &mut f64,
        var_qgos_dn10_slot: &mut f64,
        var_qgos_dn11_slot: &mut f64,
        var_qgos_dn12_slot: &mut f64,
        var_qgos_dn17_slot: &mut f64,
        var_qgos_dn2_slot: &mut f64,
        var_qgos_dn6_slot: &mut f64,
        var_qgos_dn7_slot: &mut f64,
        var_t1__blk1032_slot: &mut f64,
        var_t1__blk1032_dn10_slot: &mut f64,
        var_t1__blk1032_dn12_slot: &mut f64,
        var_t1__blk1032_dn6_slot: &mut f64,
        var_t1__blk1032_dn7_slot: &mut f64,
        var_t1__blk898_slot: &mut f64,
        var_t1__blk898_dn0_slot: &mut f64,
        var_t1__blk898_dn10_slot: &mut f64,
        var_t1__blk898_dn11_slot: &mut f64,
        var_t1__blk898_dn12_slot: &mut f64,
        var_t1__blk898_dn17_slot: &mut f64,
        var_t1__blk898_dn2_slot: &mut f64,
        var_t1__blk898_dn6_slot: &mut f64,
        var_t1__blk898_dn7_slot: &mut f64,
        var_t2__blk1033_slot: &mut f64,
        var_t2__blk1033_dn0_slot: &mut f64,
        var_t2__blk1033_dn10_slot: &mut f64,
        var_t2__blk1033_dn11_slot: &mut f64,
        var_t2__blk1033_dn12_slot: &mut f64,
        var_t2__blk1033_dn17_slot: &mut f64,
        var_t2__blk1033_dn2_slot: &mut f64,
        var_t2__blk1033_dn6_slot: &mut f64,
        var_t2__blk1033_dn7_slot: &mut f64,
        var_vbdj_slot: &mut f64,
        var_vbdj_dn12_slot: &mut f64,
        var_vbdj_dn6_slot: &mut f64,
        var_vbdt_slot: &mut f64,
        var_vbdt_dn10_slot: &mut f64,
        var_vbsj_slot: &mut f64,
        var_vbsj_dn12_slot: &mut f64,
        var_vbsj_dn7_slot: &mut f64,
        var_vbst_slot: &mut f64,
        var_vbst_dn10_slot: &mut f64,
    ) {
        let mut var_cgdoe: f64 = *var_cgdoe_slot;
        let mut var_cgdoe_dn0: f64 = *var_cgdoe_dn0_slot;
        let mut var_cgdoe_dn10: f64 = *var_cgdoe_dn10_slot;
        let mut var_cgdoe_dn11: f64 = *var_cgdoe_dn11_slot;
        let mut var_cgdoe_dn12: f64 = *var_cgdoe_dn12_slot;
        let mut var_cgdoe_dn17: f64 = *var_cgdoe_dn17_slot;
        let mut var_cgdoe_dn2: f64 = *var_cgdoe_dn2_slot;
        let mut var_cgdoe_dn6: f64 = *var_cgdoe_dn6_slot;
        let mut var_cgdoe_dn7: f64 = *var_cgdoe_dn7_slot;
        let mut var_cgsoe: f64 = *var_cgsoe_slot;
        let mut var_cgsoe_dn0: f64 = *var_cgsoe_dn0_slot;
        let mut var_cgsoe_dn10: f64 = *var_cgsoe_dn10_slot;
        let mut var_cgsoe_dn11: f64 = *var_cgsoe_dn11_slot;
        let mut var_cgsoe_dn12: f64 = *var_cgsoe_dn12_slot;
        let mut var_cgsoe_dn17: f64 = *var_cgsoe_dn17_slot;
        let mut var_cgsoe_dn2: f64 = *var_cgsoe_dn2_slot;
        let mut var_cgsoe_dn6: f64 = *var_cgsoe_dn6_slot;
        let mut var_cgsoe_dn7: f64 = *var_cgsoe_dn7_slot;
        let mut var_guard1026: f64 = *var_guard1026_slot;
        let mut var_guard1027: f64 = *var_guard1027_slot;
        let mut var_guard1028: f64 = *var_guard1028_slot;
        let mut var_guard1029: f64 = *var_guard1029_slot;
        let mut var_guard1030: f64 = *var_guard1030_slot;
        let mut var_guard1059: f64 = *var_guard1059_slot;
        let mut var_isbd: f64 = *var_isbd_slot;
        let mut var_isbd2: f64 = *var_isbd2_slot;
        let mut var_isbd2_dn0: f64 = *var_isbd2_dn0_slot;
        let mut var_isbd2_dn10: f64 = *var_isbd2_dn10_slot;
        let mut var_isbd2_dn11: f64 = *var_isbd2_dn11_slot;
        let mut var_isbd2_dn12: f64 = *var_isbd2_dn12_slot;
        let mut var_isbd2_dn17: f64 = *var_isbd2_dn17_slot;
        let mut var_isbd2_dn2: f64 = *var_isbd2_dn2_slot;
        let mut var_isbd2_dn6: f64 = *var_isbd2_dn6_slot;
        let mut var_isbd2_dn7: f64 = *var_isbd2_dn7_slot;
        let mut var_isbd_dn0: f64 = *var_isbd_dn0_slot;
        let mut var_isbd_dn10: f64 = *var_isbd_dn10_slot;
        let mut var_isbd_dn11: f64 = *var_isbd_dn11_slot;
        let mut var_isbd_dn12: f64 = *var_isbd_dn12_slot;
        let mut var_isbd_dn17: f64 = *var_isbd_dn17_slot;
        let mut var_isbd_dn2: f64 = *var_isbd_dn2_slot;
        let mut var_isbd_dn6: f64 = *var_isbd_dn6_slot;
        let mut var_isbd_dn7: f64 = *var_isbd_dn7_slot;
        let mut var_isbs: f64 = *var_isbs_slot;
        let mut var_isbs2: f64 = *var_isbs2_slot;
        let mut var_isbs2_dn0: f64 = *var_isbs2_dn0_slot;
        let mut var_isbs2_dn10: f64 = *var_isbs2_dn10_slot;
        let mut var_isbs2_dn11: f64 = *var_isbs2_dn11_slot;
        let mut var_isbs2_dn12: f64 = *var_isbs2_dn12_slot;
        let mut var_isbs2_dn17: f64 = *var_isbs2_dn17_slot;
        let mut var_isbs2_dn2: f64 = *var_isbs2_dn2_slot;
        let mut var_isbs2_dn6: f64 = *var_isbs2_dn6_slot;
        let mut var_isbs2_dn7: f64 = *var_isbs2_dn7_slot;
        let mut var_isbs_dn0: f64 = *var_isbs_dn0_slot;
        let mut var_isbs_dn10: f64 = *var_isbs_dn10_slot;
        let mut var_isbs_dn11: f64 = *var_isbs_dn11_slot;
        let mut var_isbs_dn12: f64 = *var_isbs_dn12_slot;
        let mut var_isbs_dn17: f64 = *var_isbs_dn17_slot;
        let mut var_isbs_dn2: f64 = *var_isbs_dn2_slot;
        let mut var_isbs_dn6: f64 = *var_isbs_dn6_slot;
        let mut var_isbs_dn7: f64 = *var_isbs_dn7_slot;
        let mut var_js: f64 = *var_js_slot;
        let mut var_js2: f64 = *var_js2_slot;
        let mut var_js2_dn0: f64 = *var_js2_dn0_slot;
        let mut var_js2_dn10: f64 = *var_js2_dn10_slot;
        let mut var_js2_dn11: f64 = *var_js2_dn11_slot;
        let mut var_js2_dn12: f64 = *var_js2_dn12_slot;
        let mut var_js2_dn17: f64 = *var_js2_dn17_slot;
        let mut var_js2_dn2: f64 = *var_js2_dn2_slot;
        let mut var_js2_dn6: f64 = *var_js2_dn6_slot;
        let mut var_js2_dn7: f64 = *var_js2_dn7_slot;
        let mut var_js_dn0: f64 = *var_js_dn0_slot;
        let mut var_js_dn10: f64 = *var_js_dn10_slot;
        let mut var_js_dn11: f64 = *var_js_dn11_slot;
        let mut var_js_dn12: f64 = *var_js_dn12_slot;
        let mut var_js_dn17: f64 = *var_js_dn17_slot;
        let mut var_js_dn2: f64 = *var_js_dn2_slot;
        let mut var_js_dn6: f64 = *var_js_dn6_slot;
        let mut var_js_dn7: f64 = *var_js_dn7_slot;
        let mut var_nvtm: f64 = *var_nvtm_slot;
        let mut var_nvtm_dn10: f64 = *var_nvtm_dn10_slot;
        let mut var_qgod: f64 = *var_qgod_slot;
        let mut var_qgod_dn0: f64 = *var_qgod_dn0_slot;
        let mut var_qgod_dn10: f64 = *var_qgod_dn10_slot;
        let mut var_qgod_dn11: f64 = *var_qgod_dn11_slot;
        let mut var_qgod_dn12: f64 = *var_qgod_dn12_slot;
        let mut var_qgod_dn17: f64 = *var_qgod_dn17_slot;
        let mut var_qgod_dn2: f64 = *var_qgod_dn2_slot;
        let mut var_qgod_dn6: f64 = *var_qgod_dn6_slot;
        let mut var_qgod_dn7: f64 = *var_qgod_dn7_slot;
        let mut var_qgos: f64 = *var_qgos_slot;
        let mut var_qgos_dn0: f64 = *var_qgos_dn0_slot;
        let mut var_qgos_dn10: f64 = *var_qgos_dn10_slot;
        let mut var_qgos_dn11: f64 = *var_qgos_dn11_slot;
        let mut var_qgos_dn12: f64 = *var_qgos_dn12_slot;
        let mut var_qgos_dn17: f64 = *var_qgos_dn17_slot;
        let mut var_qgos_dn2: f64 = *var_qgos_dn2_slot;
        let mut var_qgos_dn6: f64 = *var_qgos_dn6_slot;
        let mut var_qgos_dn7: f64 = *var_qgos_dn7_slot;
        let mut var_t1__blk1032: f64 = *var_t1__blk1032_slot;
        let mut var_t1__blk1032_dn10: f64 = *var_t1__blk1032_dn10_slot;
        let mut var_t1__blk1032_dn12: f64 = *var_t1__blk1032_dn12_slot;
        let mut var_t1__blk1032_dn6: f64 = *var_t1__blk1032_dn6_slot;
        let mut var_t1__blk1032_dn7: f64 = *var_t1__blk1032_dn7_slot;
        let mut var_t1__blk898: f64 = *var_t1__blk898_slot;
        let mut var_t1__blk898_dn0: f64 = *var_t1__blk898_dn0_slot;
        let mut var_t1__blk898_dn10: f64 = *var_t1__blk898_dn10_slot;
        let mut var_t1__blk898_dn11: f64 = *var_t1__blk898_dn11_slot;
        let mut var_t1__blk898_dn12: f64 = *var_t1__blk898_dn12_slot;
        let mut var_t1__blk898_dn17: f64 = *var_t1__blk898_dn17_slot;
        let mut var_t1__blk898_dn2: f64 = *var_t1__blk898_dn2_slot;
        let mut var_t1__blk898_dn6: f64 = *var_t1__blk898_dn6_slot;
        let mut var_t1__blk898_dn7: f64 = *var_t1__blk898_dn7_slot;
        let mut var_t2__blk1033: f64 = *var_t2__blk1033_slot;
        let mut var_t2__blk1033_dn0: f64 = *var_t2__blk1033_dn0_slot;
        let mut var_t2__blk1033_dn10: f64 = *var_t2__blk1033_dn10_slot;
        let mut var_t2__blk1033_dn11: f64 = *var_t2__blk1033_dn11_slot;
        let mut var_t2__blk1033_dn12: f64 = *var_t2__blk1033_dn12_slot;
        let mut var_t2__blk1033_dn17: f64 = *var_t2__blk1033_dn17_slot;
        let mut var_t2__blk1033_dn2: f64 = *var_t2__blk1033_dn2_slot;
        let mut var_t2__blk1033_dn6: f64 = *var_t2__blk1033_dn6_slot;
        let mut var_t2__blk1033_dn7: f64 = *var_t2__blk1033_dn7_slot;
        let mut var_vbdj: f64 = *var_vbdj_slot;
        let mut var_vbdj_dn12: f64 = *var_vbdj_dn12_slot;
        let mut var_vbdj_dn6: f64 = *var_vbdj_dn6_slot;
        let mut var_vbdt: f64 = *var_vbdt_slot;
        let mut var_vbdt_dn10: f64 = *var_vbdt_dn10_slot;
        let mut var_vbsj: f64 = *var_vbsj_slot;
        let mut var_vbsj_dn12: f64 = *var_vbsj_dn12_slot;
        let mut var_vbsj_dn7: f64 = *var_vbsj_dn7_slot;
        let mut var_vbst: f64 = *var_vbst_slot;
        let mut var_vbst_dn10: f64 = *var_vbst_dn10_slot;

        let (assign31760_e46694, assign31760_e46694_d_n0, assign31760_e46694_d_n2, assign31760_e46694_d_n6, assign31760_e46694_d_n7, assign31760_e46694_d_n10, assign31760_e46694_d_n11, assign31760_e46694_d_n12, assign31760_e46694_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 == 0.0)) && (var_guard1024 != 0.0)) && (var_guard1025 != 0.0)) {
        let assign31760_e46688: f64 = (-var_cox0__blk908);
        let assign31760_e46690: f64 = (assign31760_e46688 * p.p188);
        let assign31760_e46692: f64 = (assign31760_e46690 * var_w_diodcv);
        (assign31760_e46692, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31760_e46694;
        var_cgdoe_dn0 = assign31760_e46694_d_n0;
        var_cgdoe_dn2 = assign31760_e46694_d_n2;
        var_cgdoe_dn6 = assign31760_e46694_d_n6;
        var_cgdoe_dn7 = assign31760_e46694_d_n7;
        var_cgdoe_dn10 = assign31760_e46694_d_n10;
        var_cgdoe_dn11 = assign31760_e46694_d_n11;
        var_cgdoe_dn12 = assign31760_e46694_d_n12;
        var_cgdoe_dn17 = assign31760_e46694_d_n17;

        let (assign31770_e46711, assign31770_e46711_d_n0, assign31770_e46711_d_n2, assign31770_e46711_d_n6, assign31770_e46711_d_n7, assign31770_e46711_d_n10, assign31770_e46711_d_n11, assign31770_e46711_d_n12, assign31770_e46711_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 == 0.0)) && (var_guard1024 != 0.0)) && (var_guard1025 == 0.0)) {
        let assign31770_e46705: f64 = (-var_cox0__blk908);
        let assign31770_e46707: f64 = (assign31770_e46705 * p.p188);
        let assign31770_e46709: f64 = (assign31770_e46707 * var_weffcv_nf);
        (assign31770_e46709, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31770_e46711;
        var_cgdoe_dn0 = assign31770_e46711_d_n0;
        var_cgdoe_dn2 = assign31770_e46711_d_n2;
        var_cgdoe_dn6 = assign31770_e46711_d_n6;
        var_cgdoe_dn7 = assign31770_e46711_d_n7;
        var_cgdoe_dn10 = assign31770_e46711_d_n10;
        var_cgdoe_dn11 = assign31770_e46711_d_n11;
        var_cgdoe_dn12 = assign31770_e46711_d_n12;
        var_cgdoe_dn17 = assign31770_e46711_d_n17;

        let (assign31780_e46727, assign31780_e46727_d_n0, assign31780_e46727_d_n2, assign31780_e46727_d_n6, assign31780_e46727_d_n7, assign31780_e46727_d_n10, assign31780_e46727_d_n11, assign31780_e46727_d_n12, assign31780_e46727_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard980 == 0.0)) && (var_guard1024 == 0.0)) {
        let assign31780_e46721: f64 = (var_modervs * p.p170);
        let assign31780_e46724: f64 = (var_modenml * p.p169);
        let assign31780_e46725: f64 = (assign31780_e46721 + assign31780_e46724);
        (assign31780_e46725, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31780_e46727;
        var_cgdoe_dn0 = assign31780_e46727_d_n0;
        var_cgdoe_dn2 = assign31780_e46727_d_n2;
        var_cgdoe_dn6 = assign31780_e46727_d_n6;
        var_cgdoe_dn7 = assign31780_e46727_d_n7;
        var_cgdoe_dn10 = assign31780_e46727_d_n10;
        var_cgdoe_dn11 = assign31780_e46727_d_n11;
        var_cgdoe_dn12 = assign31780_e46727_d_n12;
        var_cgdoe_dn17 = assign31780_e46727_d_n17;

        let assign31790_e46730: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1026 = assign31790_e46730;

        let (assign31800_e46748, assign31800_e46748_d_n0, assign31800_e46748_d_n2, assign31800_e46748_d_n6, assign31800_e46748_d_n7, assign31800_e46748_d_n10, assign31800_e46748_d_n11, assign31800_e46748_d_n12, assign31800_e46748_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 == 0.0)) && (var_guard1024 == 0.0)) && (var_guard1026 != 0.0)) {
        let assign31800_e46742: f64 = (var_modervs * var_w_dioscv);
        let assign31800_e46745: f64 = (var_modenml * var_w_diodcv);
        let assign31800_e46746: f64 = (assign31800_e46742 + assign31800_e46745);
        (assign31800_e46746, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk898, var_t1__blk898_dn0, var_t1__blk898_dn2, var_t1__blk898_dn6, var_t1__blk898_dn7, var_t1__blk898_dn10, var_t1__blk898_dn11, var_t1__blk898_dn12, var_t1__blk898_dn17,)
    }
};
        var_t1__blk898 = assign31800_e46748;
        var_t1__blk898_dn0 = assign31800_e46748_d_n0;
        var_t1__blk898_dn2 = assign31800_e46748_d_n2;
        var_t1__blk898_dn6 = assign31800_e46748_d_n6;
        var_t1__blk898_dn7 = assign31800_e46748_d_n7;
        var_t1__blk898_dn10 = assign31800_e46748_d_n10;
        var_t1__blk898_dn11 = assign31800_e46748_d_n11;
        var_t1__blk898_dn12 = assign31800_e46748_d_n12;
        var_t1__blk898_dn17 = assign31800_e46748_d_n17;

        let (assign31810_e46763, assign31810_e46763_d_n0, assign31810_e46763_d_n2, assign31810_e46763_d_n6, assign31810_e46763_d_n7, assign31810_e46763_d_n10, assign31810_e46763_d_n11, assign31810_e46763_d_n12, assign31810_e46763_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 == 0.0)) && (var_guard1024 == 0.0)) && (var_guard1026 != 0.0)) {
        let assign31810_e46760: f64 = (-var_t1__blk898);
        let assign31810_e46761: f64 = (var_cgdoe * assign31810_e46760);
        (assign31810_e46761, ((var_cgdoe_dn0 * assign31810_e46760) + (var_cgdoe * (-var_t1__blk898_dn0))), ((var_cgdoe_dn2 * assign31810_e46760) + (var_cgdoe * (-var_t1__blk898_dn2))), ((var_cgdoe_dn6 * assign31810_e46760) + (var_cgdoe * (-var_t1__blk898_dn6))), ((var_cgdoe_dn7 * assign31810_e46760) + (var_cgdoe * (-var_t1__blk898_dn7))), ((var_cgdoe_dn10 * assign31810_e46760) + (var_cgdoe * (-var_t1__blk898_dn10))), ((var_cgdoe_dn11 * assign31810_e46760) + (var_cgdoe * (-var_t1__blk898_dn11))), ((var_cgdoe_dn12 * assign31810_e46760) + (var_cgdoe * (-var_t1__blk898_dn12))), ((var_cgdoe_dn17 * assign31810_e46760) + (var_cgdoe * (-var_t1__blk898_dn17))),)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31810_e46763;
        var_cgdoe_dn0 = assign31810_e46763_d_n0;
        var_cgdoe_dn2 = assign31810_e46763_d_n2;
        var_cgdoe_dn6 = assign31810_e46763_d_n6;
        var_cgdoe_dn7 = assign31810_e46763_d_n7;
        var_cgdoe_dn10 = assign31810_e46763_d_n10;
        var_cgdoe_dn11 = assign31810_e46763_d_n11;
        var_cgdoe_dn12 = assign31810_e46763_d_n12;
        var_cgdoe_dn17 = assign31810_e46763_d_n17;

        let (assign31820_e46779, assign31820_e46779_d_n0, assign31820_e46779_d_n2, assign31820_e46779_d_n6, assign31820_e46779_d_n7, assign31820_e46779_d_n10, assign31820_e46779_d_n11, assign31820_e46779_d_n12, assign31820_e46779_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 == 0.0)) && (var_guard1024 == 0.0)) && (var_guard1026 == 0.0)) {
        let assign31820_e46776: f64 = (-var_weffcv_nf);
        let assign31820_e46777: f64 = (var_cgdoe * assign31820_e46776);
        (assign31820_e46777, (var_cgdoe_dn0 * assign31820_e46776), (var_cgdoe_dn2 * assign31820_e46776), (var_cgdoe_dn6 * assign31820_e46776), (var_cgdoe_dn7 * assign31820_e46776), (var_cgdoe_dn10 * assign31820_e46776), (var_cgdoe_dn11 * assign31820_e46776), (var_cgdoe_dn12 * assign31820_e46776), (var_cgdoe_dn17 * assign31820_e46776),)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31820_e46779;
        var_cgdoe_dn0 = assign31820_e46779_d_n0;
        var_cgdoe_dn2 = assign31820_e46779_d_n2;
        var_cgdoe_dn6 = assign31820_e46779_d_n6;
        var_cgdoe_dn7 = assign31820_e46779_d_n7;
        var_cgdoe_dn10 = assign31820_e46779_d_n10;
        var_cgdoe_dn11 = assign31820_e46779_d_n11;
        var_cgdoe_dn12 = assign31820_e46779_d_n12;
        var_cgdoe_dn17 = assign31820_e46779_d_n17;

        let (assign31830_e46791, assign31830_e46791_d_n0, assign31830_e46791_d_n2, assign31830_e46791_d_n6, assign31830_e46791_d_n7, assign31830_e46791_d_n10, assign31830_e46791_d_n11, assign31830_e46791_d_n12, assign31830_e46791_d_n17,) = {
    if ((p.p24 != 0.0) && (var_guard980 == 0.0)) {
        let assign31830_e46785: f64 = (-var_cgdoe);
        let assign31830_e46788: f64 = (var_vgs - var_vds);
        let assign31830_e46789: f64 = (assign31830_e46785 * assign31830_e46788);
        (assign31830_e46789, (((-var_cgdoe_dn0) * assign31830_e46788) + (assign31830_e46785 * (-var_vds_dn0))), (((-var_cgdoe_dn2) * assign31830_e46788) + (assign31830_e46785 * (-var_vds_dn2))), (((-var_cgdoe_dn6) * assign31830_e46788) + (assign31830_e46785 * (var_vgs_dn6 - var_vds_dn6))), (((-var_cgdoe_dn7) * assign31830_e46788) + (assign31830_e46785 * (var_vgs_dn7 - var_vds_dn7))), (((-var_cgdoe_dn10) * assign31830_e46788) + (assign31830_e46785 * (-var_vds_dn10))), (((-var_cgdoe_dn11) * assign31830_e46788) + (assign31830_e46785 * (var_vgs_dn11 - var_vds_dn11))), (((-var_cgdoe_dn12) * assign31830_e46788) + (assign31830_e46785 * (-var_vds_dn12))), (((-var_cgdoe_dn17) * assign31830_e46788) + (assign31830_e46785 * (-var_vds_dn17))),)
    } else {
        (var_qgod, var_qgod_dn0, var_qgod_dn2, var_qgod_dn6, var_qgod_dn7, var_qgod_dn10, var_qgod_dn11, var_qgod_dn12, var_qgod_dn17,)
    }
};
        var_qgod = assign31830_e46791;
        var_qgod_dn0 = assign31830_e46791_d_n0;
        var_qgod_dn2 = assign31830_e46791_d_n2;
        var_qgod_dn6 = assign31830_e46791_d_n6;
        var_qgod_dn7 = assign31830_e46791_d_n7;
        var_qgod_dn10 = assign31830_e46791_d_n10;
        var_qgod_dn11 = assign31830_e46791_d_n11;
        var_qgod_dn12 = assign31830_e46791_d_n12;
        var_qgod_dn17 = assign31830_e46791_d_n17;

        let assign31840_e46804: f64 = if (((var_mode == 1.0) && (var_cgso_given == 0.0)) || ((var_mode != 1.0) && (var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        var_guard1027 = assign31840_e46804;

        let assign31850_e46807: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1028 = assign31850_e46807;

        let (assign31860_e46823, assign31860_e46823_d_n0, assign31860_e46823_d_n2, assign31860_e46823_d_n6, assign31860_e46823_d_n7, assign31860_e46823_d_n10, assign31860_e46823_d_n11, assign31860_e46823_d_n12, assign31860_e46823_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 == 0.0)) && (var_guard1027 != 0.0)) && (var_guard1028 != 0.0)) {
        let assign31860_e46817: f64 = (-var_cox0__blk908);
        let assign31860_e46819: f64 = (assign31860_e46817 * p.p188);
        let assign31860_e46821: f64 = (assign31860_e46819 * var_w_dioscv);
        (assign31860_e46821, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31860_e46823;
        var_cgsoe_dn0 = assign31860_e46823_d_n0;
        var_cgsoe_dn2 = assign31860_e46823_d_n2;
        var_cgsoe_dn6 = assign31860_e46823_d_n6;
        var_cgsoe_dn7 = assign31860_e46823_d_n7;
        var_cgsoe_dn10 = assign31860_e46823_d_n10;
        var_cgsoe_dn11 = assign31860_e46823_d_n11;
        var_cgsoe_dn12 = assign31860_e46823_d_n12;
        var_cgsoe_dn17 = assign31860_e46823_d_n17;

        let (assign31870_e46840, assign31870_e46840_d_n0, assign31870_e46840_d_n2, assign31870_e46840_d_n6, assign31870_e46840_d_n7, assign31870_e46840_d_n10, assign31870_e46840_d_n11, assign31870_e46840_d_n12, assign31870_e46840_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 == 0.0)) && (var_guard1027 != 0.0)) && (var_guard1028 == 0.0)) {
        let assign31870_e46834: f64 = (-var_cox0__blk908);
        let assign31870_e46836: f64 = (assign31870_e46834 * p.p188);
        let assign31870_e46838: f64 = (assign31870_e46836 * var_weffcv_nf);
        (assign31870_e46838, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31870_e46840;
        var_cgsoe_dn0 = assign31870_e46840_d_n0;
        var_cgsoe_dn2 = assign31870_e46840_d_n2;
        var_cgsoe_dn6 = assign31870_e46840_d_n6;
        var_cgsoe_dn7 = assign31870_e46840_d_n7;
        var_cgsoe_dn10 = assign31870_e46840_d_n10;
        var_cgsoe_dn11 = assign31870_e46840_d_n11;
        var_cgsoe_dn12 = assign31870_e46840_d_n12;
        var_cgsoe_dn17 = assign31870_e46840_d_n17;

        let (assign31880_e46856, assign31880_e46856_d_n0, assign31880_e46856_d_n2, assign31880_e46856_d_n6, assign31880_e46856_d_n7, assign31880_e46856_d_n10, assign31880_e46856_d_n11, assign31880_e46856_d_n12, assign31880_e46856_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard980 == 0.0)) && (var_guard1027 == 0.0)) {
        let assign31880_e46850: f64 = (var_modenml * p.p170);
        let assign31880_e46853: f64 = (var_modervs * p.p169);
        let assign31880_e46854: f64 = (assign31880_e46850 + assign31880_e46853);
        (assign31880_e46854, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31880_e46856;
        var_cgsoe_dn0 = assign31880_e46856_d_n0;
        var_cgsoe_dn2 = assign31880_e46856_d_n2;
        var_cgsoe_dn6 = assign31880_e46856_d_n6;
        var_cgsoe_dn7 = assign31880_e46856_d_n7;
        var_cgsoe_dn10 = assign31880_e46856_d_n10;
        var_cgsoe_dn11 = assign31880_e46856_d_n11;
        var_cgsoe_dn12 = assign31880_e46856_d_n12;
        var_cgsoe_dn17 = assign31880_e46856_d_n17;

        let assign31890_e46859: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1029 = assign31890_e46859;

        let (assign31900_e46877, assign31900_e46877_d_n0, assign31900_e46877_d_n2, assign31900_e46877_d_n6, assign31900_e46877_d_n7, assign31900_e46877_d_n10, assign31900_e46877_d_n11, assign31900_e46877_d_n12, assign31900_e46877_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 == 0.0)) && (var_guard1027 == 0.0)) && (var_guard1029 != 0.0)) {
        let assign31900_e46871: f64 = (var_modenml * var_w_dioscv);
        let assign31900_e46874: f64 = (var_modervs * var_w_diodcv);
        let assign31900_e46875: f64 = (assign31900_e46871 + assign31900_e46874);
        (assign31900_e46875, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk898, var_t1__blk898_dn0, var_t1__blk898_dn2, var_t1__blk898_dn6, var_t1__blk898_dn7, var_t1__blk898_dn10, var_t1__blk898_dn11, var_t1__blk898_dn12, var_t1__blk898_dn17,)
    }
};
        var_t1__blk898 = assign31900_e46877;
        var_t1__blk898_dn0 = assign31900_e46877_d_n0;
        var_t1__blk898_dn2 = assign31900_e46877_d_n2;
        var_t1__blk898_dn6 = assign31900_e46877_d_n6;
        var_t1__blk898_dn7 = assign31900_e46877_d_n7;
        var_t1__blk898_dn10 = assign31900_e46877_d_n10;
        var_t1__blk898_dn11 = assign31900_e46877_d_n11;
        var_t1__blk898_dn12 = assign31900_e46877_d_n12;
        var_t1__blk898_dn17 = assign31900_e46877_d_n17;

        let (assign31910_e46892, assign31910_e46892_d_n0, assign31910_e46892_d_n2, assign31910_e46892_d_n6, assign31910_e46892_d_n7, assign31910_e46892_d_n10, assign31910_e46892_d_n11, assign31910_e46892_d_n12, assign31910_e46892_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 == 0.0)) && (var_guard1027 == 0.0)) && (var_guard1029 != 0.0)) {
        let assign31910_e46889: f64 = (-var_t1__blk898);
        let assign31910_e46890: f64 = (var_cgsoe * assign31910_e46889);
        (assign31910_e46890, ((var_cgsoe_dn0 * assign31910_e46889) + (var_cgsoe * (-var_t1__blk898_dn0))), ((var_cgsoe_dn2 * assign31910_e46889) + (var_cgsoe * (-var_t1__blk898_dn2))), ((var_cgsoe_dn6 * assign31910_e46889) + (var_cgsoe * (-var_t1__blk898_dn6))), ((var_cgsoe_dn7 * assign31910_e46889) + (var_cgsoe * (-var_t1__blk898_dn7))), ((var_cgsoe_dn10 * assign31910_e46889) + (var_cgsoe * (-var_t1__blk898_dn10))), ((var_cgsoe_dn11 * assign31910_e46889) + (var_cgsoe * (-var_t1__blk898_dn11))), ((var_cgsoe_dn12 * assign31910_e46889) + (var_cgsoe * (-var_t1__blk898_dn12))), ((var_cgsoe_dn17 * assign31910_e46889) + (var_cgsoe * (-var_t1__blk898_dn17))),)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31910_e46892;
        var_cgsoe_dn0 = assign31910_e46892_d_n0;
        var_cgsoe_dn2 = assign31910_e46892_d_n2;
        var_cgsoe_dn6 = assign31910_e46892_d_n6;
        var_cgsoe_dn7 = assign31910_e46892_d_n7;
        var_cgsoe_dn10 = assign31910_e46892_d_n10;
        var_cgsoe_dn11 = assign31910_e46892_d_n11;
        var_cgsoe_dn12 = assign31910_e46892_d_n12;
        var_cgsoe_dn17 = assign31910_e46892_d_n17;

        let (assign31920_e46908, assign31920_e46908_d_n0, assign31920_e46908_d_n2, assign31920_e46908_d_n6, assign31920_e46908_d_n7, assign31920_e46908_d_n10, assign31920_e46908_d_n11, assign31920_e46908_d_n12, assign31920_e46908_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard980 == 0.0)) && (var_guard1027 == 0.0)) && (var_guard1029 == 0.0)) {
        let assign31920_e46905: f64 = (-var_weffcv_nf);
        let assign31920_e46906: f64 = (var_cgsoe * assign31920_e46905);
        (assign31920_e46906, (var_cgsoe_dn0 * assign31920_e46905), (var_cgsoe_dn2 * assign31920_e46905), (var_cgsoe_dn6 * assign31920_e46905), (var_cgsoe_dn7 * assign31920_e46905), (var_cgsoe_dn10 * assign31920_e46905), (var_cgsoe_dn11 * assign31920_e46905), (var_cgsoe_dn12 * assign31920_e46905), (var_cgsoe_dn17 * assign31920_e46905),)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31920_e46908;
        var_cgsoe_dn0 = assign31920_e46908_d_n0;
        var_cgsoe_dn2 = assign31920_e46908_d_n2;
        var_cgsoe_dn6 = assign31920_e46908_d_n6;
        var_cgsoe_dn7 = assign31920_e46908_d_n7;
        var_cgsoe_dn10 = assign31920_e46908_d_n10;
        var_cgsoe_dn11 = assign31920_e46908_d_n11;
        var_cgsoe_dn12 = assign31920_e46908_d_n12;
        var_cgsoe_dn17 = assign31920_e46908_d_n17;

        let (assign31930_e46918, assign31930_e46918_d_n0, assign31930_e46918_d_n2, assign31930_e46918_d_n6, assign31930_e46918_d_n7, assign31930_e46918_d_n10, assign31930_e46918_d_n11, assign31930_e46918_d_n12, assign31930_e46918_d_n17,) = {
    if ((p.p24 != 0.0) && (var_guard980 == 0.0)) {
        let assign31930_e46914: f64 = (-var_cgsoe);
        let assign31930_e46916: f64 = (assign31930_e46914 * var_vgs);
        (assign31930_e46916, ((-var_cgsoe_dn0) * var_vgs), ((-var_cgsoe_dn2) * var_vgs), (((-var_cgsoe_dn6) * var_vgs) + (assign31930_e46914 * var_vgs_dn6)), (((-var_cgsoe_dn7) * var_vgs) + (assign31930_e46914 * var_vgs_dn7)), ((-var_cgsoe_dn10) * var_vgs), (((-var_cgsoe_dn11) * var_vgs) + (assign31930_e46914 * var_vgs_dn11)), ((-var_cgsoe_dn12) * var_vgs), ((-var_cgsoe_dn17) * var_vgs),)
    } else {
        (var_qgos, var_qgos_dn0, var_qgos_dn2, var_qgos_dn6, var_qgos_dn7, var_qgos_dn10, var_qgos_dn11, var_qgos_dn12, var_qgos_dn17,)
    }
};
        var_qgos = assign31930_e46918;
        var_qgos_dn0 = assign31930_e46918_d_n0;
        var_qgos_dn2 = assign31930_e46918_d_n2;
        var_qgos_dn6 = assign31930_e46918_d_n6;
        var_qgos_dn7 = assign31930_e46918_d_n7;
        var_qgos_dn10 = assign31930_e46918_d_n10;
        var_qgos_dn11 = assign31930_e46918_d_n11;
        var_qgos_dn12 = assign31930_e46918_d_n12;
        var_qgos_dn17 = assign31930_e46918_d_n17;

        let assign31940_e46921: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1030 = assign31940_e46921;

        let (assign31950_e46925, assign31950_e46925_d_n6, assign31950_e46925_d_n12,) = {
    if (var_guard1030 != 0.0) {
        (var_vbcd, var_vbcd_dn6, var_vbcd_dn12,)
    } else {
        (var_vbdj, var_vbdj_dn6, var_vbdj_dn12,)
    }
};
        var_vbdj = assign31950_e46925;
        var_vbdj_dn6 = assign31950_e46925_d_n6;
        var_vbdj_dn12 = assign31950_e46925_d_n12;

        let (assign31960_e46929, assign31960_e46929_d_n7, assign31960_e46929_d_n12,) = {
    if (var_guard1030 != 0.0) {
        (var_vbcs, var_vbcs_dn7, var_vbcs_dn12,)
    } else {
        (var_vbsj, var_vbsj_dn7, var_vbsj_dn12,)
    }
};
        var_vbsj = assign31960_e46929;
        var_vbsj_dn7 = assign31960_e46929_d_n7;
        var_vbsj_dn12 = assign31960_e46929_d_n12;

        let (assign31970_e46951, assign31970_e46951_d_n0, assign31970_e46951_d_n2, assign31970_e46951_d_n6, assign31970_e46951_d_n7, assign31970_e46951_d_n10, assign31970_e46951_d_n11, assign31970_e46951_d_n12, assign31970_e46951_d_n17,) = {
    if (var_guard1030 != 0.0) {
        let assign31970_e46934: f64 = (var_egtnom * var_betatnom);
        let assign31970_e46937: f64 = (var_eg * var_beta);
        let assign31970_e46938: f64 = (assign31970_e46934 - assign31970_e46937);
        let assign31970_e46942: f64 = (var_ttemp / var_uc_tnom);
        let assign31970_e46943: f64 = (assign31970_e46942).ln();
        let assign31970_e46944: f64 = (p.p175 * assign31970_e46943);
        let assign31970_e46945: f64 = (assign31970_e46938 + assign31970_e46944);
        let assign31970_e46947: f64 = (assign31970_e46945 / p.p174);
        let assign31970_e46948: f64 = (assign31970_e46947).exp();
        let assign31970_e46949: f64 = (p.p173 * assign31970_e46948);
        (assign31970_e46949, (p.p173 * (assign31970_e46948 * ((-(var_eg_dn0 * var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(var_eg_dn2 * var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(var_eg_dn6 * var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(var_eg_dn7 * var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * (((-((var_eg_dn10 * var_beta) + (var_eg * var_beta_dn10))) + (p.p175 * ((var_ttemp_dn10 / var_uc_tnom) / assign31970_e46942))) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(var_eg_dn11 * var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(var_eg_dn12 * var_beta)) / p.p174))), (p.p173 * (assign31970_e46948 * ((-(var_eg_dn17 * var_beta)) / p.p174))),)
    } else {
        (var_js, var_js_dn0, var_js_dn2, var_js_dn6, var_js_dn7, var_js_dn10, var_js_dn11, var_js_dn12, var_js_dn17,)
    }
};
        var_js = assign31970_e46951;
        var_js_dn0 = assign31970_e46951_d_n0;
        var_js_dn2 = assign31970_e46951_d_n2;
        var_js_dn6 = assign31970_e46951_d_n6;
        var_js_dn7 = assign31970_e46951_d_n7;
        var_js_dn10 = assign31970_e46951_d_n10;
        var_js_dn11 = assign31970_e46951_d_n11;
        var_js_dn12 = assign31970_e46951_d_n12;
        var_js_dn17 = assign31970_e46951_d_n17;

        let (assign31980_e46973, assign31980_e46973_d_n0, assign31980_e46973_d_n2, assign31980_e46973_d_n6, assign31980_e46973_d_n7, assign31980_e46973_d_n10, assign31980_e46973_d_n11, assign31980_e46973_d_n12, assign31980_e46973_d_n17,) = {
    if (var_guard1030 != 0.0) {
        let assign31980_e46956: f64 = (var_egtnom * var_betatnom);
        let assign31980_e46959: f64 = (var_eg * var_beta);
        let assign31980_e46960: f64 = (assign31980_e46956 - assign31980_e46959);
        let assign31980_e46964: f64 = (var_ttemp / var_uc_tnom);
        let assign31980_e46965: f64 = (assign31980_e46964).ln();
        let assign31980_e46966: f64 = (p.p176 * assign31980_e46965);
        let assign31980_e46967: f64 = (assign31980_e46960 + assign31980_e46966);
        let assign31980_e46969: f64 = (assign31980_e46967 / p.p174);
        let assign31980_e46970: f64 = (assign31980_e46969).exp();
        let assign31980_e46971: f64 = (p.p173 * assign31980_e46970);
        (assign31980_e46971, (p.p173 * (assign31980_e46970 * ((-(var_eg_dn0 * var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(var_eg_dn2 * var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(var_eg_dn6 * var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(var_eg_dn7 * var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * (((-((var_eg_dn10 * var_beta) + (var_eg * var_beta_dn10))) + (p.p176 * ((var_ttemp_dn10 / var_uc_tnom) / assign31980_e46964))) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(var_eg_dn11 * var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(var_eg_dn12 * var_beta)) / p.p174))), (p.p173 * (assign31980_e46970 * ((-(var_eg_dn17 * var_beta)) / p.p174))),)
    } else {
        (var_js2, var_js2_dn0, var_js2_dn2, var_js2_dn6, var_js2_dn7, var_js2_dn10, var_js2_dn11, var_js2_dn12, var_js2_dn17,)
    }
};
        var_js2 = assign31980_e46973;
        var_js2_dn0 = assign31980_e46973_d_n0;
        var_js2_dn2 = assign31980_e46973_d_n2;
        var_js2_dn6 = assign31980_e46973_d_n6;
        var_js2_dn7 = assign31980_e46973_d_n7;
        var_js2_dn10 = assign31980_e46973_d_n10;
        var_js2_dn11 = assign31980_e46973_d_n11;
        var_js2_dn12 = assign31980_e46973_d_n12;
        var_js2_dn17 = assign31980_e46973_d_n17;

        let (assign31990_e46981, assign31990_e46981_d_n0, assign31990_e46981_d_n2, assign31990_e46981_d_n6, assign31990_e46981_d_n7, assign31990_e46981_d_n10, assign31990_e46981_d_n11, assign31990_e46981_d_n12, assign31990_e46981_d_n17,) = {
    if (var_guard1030 != 0.0) {
        let assign31990_e46977: f64 = (var_w_diod * p.p237);
        let assign31990_e46979: f64 = (assign31990_e46977 * var_js);
        (assign31990_e46979, (assign31990_e46977 * var_js_dn0), (assign31990_e46977 * var_js_dn2), (assign31990_e46977 * var_js_dn6), (assign31990_e46977 * var_js_dn7), (assign31990_e46977 * var_js_dn10), (assign31990_e46977 * var_js_dn11), (assign31990_e46977 * var_js_dn12), (assign31990_e46977 * var_js_dn17),)
    } else {
        (var_isbd, var_isbd_dn0, var_isbd_dn2, var_isbd_dn6, var_isbd_dn7, var_isbd_dn10, var_isbd_dn11, var_isbd_dn12, var_isbd_dn17,)
    }
};
        var_isbd = assign31990_e46981;
        var_isbd_dn0 = assign31990_e46981_d_n0;
        var_isbd_dn2 = assign31990_e46981_d_n2;
        var_isbd_dn6 = assign31990_e46981_d_n6;
        var_isbd_dn7 = assign31990_e46981_d_n7;
        var_isbd_dn10 = assign31990_e46981_d_n10;
        var_isbd_dn11 = assign31990_e46981_d_n11;
        var_isbd_dn12 = assign31990_e46981_d_n12;
        var_isbd_dn17 = assign31990_e46981_d_n17;

        let (assign32000_e46989, assign32000_e46989_d_n0, assign32000_e46989_d_n2, assign32000_e46989_d_n6, assign32000_e46989_d_n7, assign32000_e46989_d_n10, assign32000_e46989_d_n11, assign32000_e46989_d_n12, assign32000_e46989_d_n17,) = {
    if (var_guard1030 != 0.0) {
        let assign32000_e46985: f64 = (var_w_diod * p.p237);
        let assign32000_e46987: f64 = (assign32000_e46985 * var_js2);
        (assign32000_e46987, (assign32000_e46985 * var_js2_dn0), (assign32000_e46985 * var_js2_dn2), (assign32000_e46985 * var_js2_dn6), (assign32000_e46985 * var_js2_dn7), (assign32000_e46985 * var_js2_dn10), (assign32000_e46985 * var_js2_dn11), (assign32000_e46985 * var_js2_dn12), (assign32000_e46985 * var_js2_dn17),)
    } else {
        (var_isbd2, var_isbd2_dn0, var_isbd2_dn2, var_isbd2_dn6, var_isbd2_dn7, var_isbd2_dn10, var_isbd2_dn11, var_isbd2_dn12, var_isbd2_dn17,)
    }
};
        var_isbd2 = assign32000_e46989;
        var_isbd2_dn0 = assign32000_e46989_d_n0;
        var_isbd2_dn2 = assign32000_e46989_d_n2;
        var_isbd2_dn6 = assign32000_e46989_d_n6;
        var_isbd2_dn7 = assign32000_e46989_d_n7;
        var_isbd2_dn10 = assign32000_e46989_d_n10;
        var_isbd2_dn11 = assign32000_e46989_d_n11;
        var_isbd2_dn12 = assign32000_e46989_d_n12;
        var_isbd2_dn17 = assign32000_e46989_d_n17;

        let (assign32010_e46997, assign32010_e46997_d_n0, assign32010_e46997_d_n2, assign32010_e46997_d_n6, assign32010_e46997_d_n7, assign32010_e46997_d_n10, assign32010_e46997_d_n11, assign32010_e46997_d_n12, assign32010_e46997_d_n17,) = {
    if (var_guard1030 != 0.0) {
        let assign32010_e46993: f64 = (var_w_dios * p.p237);
        let assign32010_e46995: f64 = (assign32010_e46993 * var_js);
        (assign32010_e46995, (assign32010_e46993 * var_js_dn0), (assign32010_e46993 * var_js_dn2), (assign32010_e46993 * var_js_dn6), (assign32010_e46993 * var_js_dn7), (assign32010_e46993 * var_js_dn10), (assign32010_e46993 * var_js_dn11), (assign32010_e46993 * var_js_dn12), (assign32010_e46993 * var_js_dn17),)
    } else {
        (var_isbs, var_isbs_dn0, var_isbs_dn2, var_isbs_dn6, var_isbs_dn7, var_isbs_dn10, var_isbs_dn11, var_isbs_dn12, var_isbs_dn17,)
    }
};
        var_isbs = assign32010_e46997;
        var_isbs_dn0 = assign32010_e46997_d_n0;
        var_isbs_dn2 = assign32010_e46997_d_n2;
        var_isbs_dn6 = assign32010_e46997_d_n6;
        var_isbs_dn7 = assign32010_e46997_d_n7;
        var_isbs_dn10 = assign32010_e46997_d_n10;
        var_isbs_dn11 = assign32010_e46997_d_n11;
        var_isbs_dn12 = assign32010_e46997_d_n12;
        var_isbs_dn17 = assign32010_e46997_d_n17;

        let (assign32020_e47005, assign32020_e47005_d_n0, assign32020_e47005_d_n2, assign32020_e47005_d_n6, assign32020_e47005_d_n7, assign32020_e47005_d_n10, assign32020_e47005_d_n11, assign32020_e47005_d_n12, assign32020_e47005_d_n17,) = {
    if (var_guard1030 != 0.0) {
        let assign32020_e47001: f64 = (var_w_dios * p.p237);
        let assign32020_e47003: f64 = (assign32020_e47001 * var_js2);
        (assign32020_e47003, (assign32020_e47001 * var_js2_dn0), (assign32020_e47001 * var_js2_dn2), (assign32020_e47001 * var_js2_dn6), (assign32020_e47001 * var_js2_dn7), (assign32020_e47001 * var_js2_dn10), (assign32020_e47001 * var_js2_dn11), (assign32020_e47001 * var_js2_dn12), (assign32020_e47001 * var_js2_dn17),)
    } else {
        (var_isbs2, var_isbs2_dn0, var_isbs2_dn2, var_isbs2_dn6, var_isbs2_dn7, var_isbs2_dn10, var_isbs2_dn11, var_isbs2_dn12, var_isbs2_dn17,)
    }
};
        var_isbs2 = assign32020_e47005;
        var_isbs2_dn0 = assign32020_e47005_d_n0;
        var_isbs2_dn2 = assign32020_e47005_d_n2;
        var_isbs2_dn6 = assign32020_e47005_d_n6;
        var_isbs2_dn7 = assign32020_e47005_d_n7;
        var_isbs2_dn10 = assign32020_e47005_d_n10;
        var_isbs2_dn11 = assign32020_e47005_d_n11;
        var_isbs2_dn12 = assign32020_e47005_d_n12;
        var_isbs2_dn17 = assign32020_e47005_d_n17;

        let (assign32030_e47011, assign32030_e47011_d_n6, assign32030_e47011_d_n7, assign32030_e47011_d_n10, assign32030_e47011_d_n12,) = {
    if (var_guard1030 != 0.0) {
        let assign32030_e47009: f64 = (var_ttemp / var_uc_tnom);
        (assign32030_e47009, 0.0, 0.0, (var_ttemp_dn10 / var_uc_tnom), 0.0,)
    } else {
        (var_t1__blk1032, var_t1__blk1032_dn6, var_t1__blk1032_dn7, var_t1__blk1032_dn10, var_t1__blk1032_dn12,)
    }
};
        var_t1__blk1032 = assign32030_e47011;
        var_t1__blk1032_dn6 = assign32030_e47011_d_n6;
        var_t1__blk1032_dn7 = assign32030_e47011_d_n7;
        var_t1__blk1032_dn10 = assign32030_e47011_d_n10;
        var_t1__blk1032_dn12 = assign32030_e47011_d_n12;

        let (assign32050_e47023, assign32050_e47023_d_n0, assign32050_e47023_d_n2, assign32050_e47023_d_n6, assign32050_e47023_d_n7, assign32050_e47023_d_n10, assign32050_e47023_d_n11, assign32050_e47023_d_n12, assign32050_e47023_d_n17,) = {
    if (var_guard1030 != 0.0) {
        let assign32050_e47021: f64 = (var_isbd + 1e-50);
        (assign32050_e47021, var_isbd_dn0, var_isbd_dn2, var_isbd_dn6, var_isbd_dn7, var_isbd_dn10, var_isbd_dn11, var_isbd_dn12, var_isbd_dn17,)
    } else {
        (var_t2__blk1033, var_t2__blk1033_dn0, var_t2__blk1033_dn2, var_t2__blk1033_dn6, var_t2__blk1033_dn7, var_t2__blk1033_dn10, var_t2__blk1033_dn11, var_t2__blk1033_dn12, var_t2__blk1033_dn17,)
    }
};
        var_t2__blk1033 = assign32050_e47023;
        var_t2__blk1033_dn0 = assign32050_e47023_d_n0;
        var_t2__blk1033_dn2 = assign32050_e47023_d_n2;
        var_t2__blk1033_dn6 = assign32050_e47023_d_n6;
        var_t2__blk1033_dn7 = assign32050_e47023_d_n7;
        var_t2__blk1033_dn10 = assign32050_e47023_d_n10;
        var_t2__blk1033_dn11 = assign32050_e47023_d_n11;
        var_t2__blk1033_dn12 = assign32050_e47023_d_n12;
        var_t2__blk1033_dn17 = assign32050_e47023_d_n17;

        let (assign32070_e47037, assign32070_e47037_d_n10,) = {
    if (var_guard1030 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_vbdt, var_vbdt_dn10,)
    }
};
        var_vbdt = assign32070_e47037;
        var_vbdt_dn10 = assign32070_e47037_d_n10;

        let (assign32080_e47045, assign32080_e47045_d_n10,) = {
    if (var_guard1030 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_vbst, var_vbst_dn10,)
    }
};
        var_vbst = assign32080_e47045;
        var_vbst_dn10 = assign32080_e47045_d_n10;

        let (assign32090_e47051, assign32090_e47051_d_n10,) = {
    if (var_guard1030 != 0.0) {
        let assign32090_e47049: f64 = (p.p174 * var_beta_inv);
        (assign32090_e47049, (p.p174 * var_beta_inv_dn10),)
    } else {
        (var_nvtm, var_nvtm_dn10,)
    }
};
        var_nvtm = assign32090_e47051;
        var_nvtm_dn10 = assign32090_e47051_d_n10;

        let assign32100_e47054: f64 = if var_vbdj < var_vbdt { 1.0 } else { 0.0 };
        var_guard1059 = assign32100_e47054;

        *var_cgdoe_slot = var_cgdoe;
        *var_cgdoe_dn0_slot = var_cgdoe_dn0;
        *var_cgdoe_dn10_slot = var_cgdoe_dn10;
        *var_cgdoe_dn11_slot = var_cgdoe_dn11;
        *var_cgdoe_dn12_slot = var_cgdoe_dn12;
        *var_cgdoe_dn17_slot = var_cgdoe_dn17;
        *var_cgdoe_dn2_slot = var_cgdoe_dn2;
        *var_cgdoe_dn6_slot = var_cgdoe_dn6;
        *var_cgdoe_dn7_slot = var_cgdoe_dn7;
        *var_cgsoe_slot = var_cgsoe;
        *var_cgsoe_dn0_slot = var_cgsoe_dn0;
        *var_cgsoe_dn10_slot = var_cgsoe_dn10;
        *var_cgsoe_dn11_slot = var_cgsoe_dn11;
        *var_cgsoe_dn12_slot = var_cgsoe_dn12;
        *var_cgsoe_dn17_slot = var_cgsoe_dn17;
        *var_cgsoe_dn2_slot = var_cgsoe_dn2;
        *var_cgsoe_dn6_slot = var_cgsoe_dn6;
        *var_cgsoe_dn7_slot = var_cgsoe_dn7;
        *var_guard1026_slot = var_guard1026;
        *var_guard1027_slot = var_guard1027;
        *var_guard1028_slot = var_guard1028;
        *var_guard1029_slot = var_guard1029;
        *var_guard1030_slot = var_guard1030;
        *var_guard1059_slot = var_guard1059;
        *var_isbd_slot = var_isbd;
        *var_isbd2_slot = var_isbd2;
        *var_isbd2_dn0_slot = var_isbd2_dn0;
        *var_isbd2_dn10_slot = var_isbd2_dn10;
        *var_isbd2_dn11_slot = var_isbd2_dn11;
        *var_isbd2_dn12_slot = var_isbd2_dn12;
        *var_isbd2_dn17_slot = var_isbd2_dn17;
        *var_isbd2_dn2_slot = var_isbd2_dn2;
        *var_isbd2_dn6_slot = var_isbd2_dn6;
        *var_isbd2_dn7_slot = var_isbd2_dn7;
        *var_isbd_dn0_slot = var_isbd_dn0;
        *var_isbd_dn10_slot = var_isbd_dn10;
        *var_isbd_dn11_slot = var_isbd_dn11;
        *var_isbd_dn12_slot = var_isbd_dn12;
        *var_isbd_dn17_slot = var_isbd_dn17;
        *var_isbd_dn2_slot = var_isbd_dn2;
        *var_isbd_dn6_slot = var_isbd_dn6;
        *var_isbd_dn7_slot = var_isbd_dn7;
        *var_isbs_slot = var_isbs;
        *var_isbs2_slot = var_isbs2;
        *var_isbs2_dn0_slot = var_isbs2_dn0;
        *var_isbs2_dn10_slot = var_isbs2_dn10;
        *var_isbs2_dn11_slot = var_isbs2_dn11;
        *var_isbs2_dn12_slot = var_isbs2_dn12;
        *var_isbs2_dn17_slot = var_isbs2_dn17;
        *var_isbs2_dn2_slot = var_isbs2_dn2;
        *var_isbs2_dn6_slot = var_isbs2_dn6;
        *var_isbs2_dn7_slot = var_isbs2_dn7;
        *var_isbs_dn0_slot = var_isbs_dn0;
        *var_isbs_dn10_slot = var_isbs_dn10;
        *var_isbs_dn11_slot = var_isbs_dn11;
        *var_isbs_dn12_slot = var_isbs_dn12;
        *var_isbs_dn17_slot = var_isbs_dn17;
        *var_isbs_dn2_slot = var_isbs_dn2;
        *var_isbs_dn6_slot = var_isbs_dn6;
        *var_isbs_dn7_slot = var_isbs_dn7;
        *var_js_slot = var_js;
        *var_js2_slot = var_js2;
        *var_js2_dn0_slot = var_js2_dn0;
        *var_js2_dn10_slot = var_js2_dn10;
        *var_js2_dn11_slot = var_js2_dn11;
        *var_js2_dn12_slot = var_js2_dn12;
        *var_js2_dn17_slot = var_js2_dn17;
        *var_js2_dn2_slot = var_js2_dn2;
        *var_js2_dn6_slot = var_js2_dn6;
        *var_js2_dn7_slot = var_js2_dn7;
        *var_js_dn0_slot = var_js_dn0;
        *var_js_dn10_slot = var_js_dn10;
        *var_js_dn11_slot = var_js_dn11;
        *var_js_dn12_slot = var_js_dn12;
        *var_js_dn17_slot = var_js_dn17;
        *var_js_dn2_slot = var_js_dn2;
        *var_js_dn6_slot = var_js_dn6;
        *var_js_dn7_slot = var_js_dn7;
        *var_nvtm_slot = var_nvtm;
        *var_nvtm_dn10_slot = var_nvtm_dn10;
        *var_qgod_slot = var_qgod;
        *var_qgod_dn0_slot = var_qgod_dn0;
        *var_qgod_dn10_slot = var_qgod_dn10;
        *var_qgod_dn11_slot = var_qgod_dn11;
        *var_qgod_dn12_slot = var_qgod_dn12;
        *var_qgod_dn17_slot = var_qgod_dn17;
        *var_qgod_dn2_slot = var_qgod_dn2;
        *var_qgod_dn6_slot = var_qgod_dn6;
        *var_qgod_dn7_slot = var_qgod_dn7;
        *var_qgos_slot = var_qgos;
        *var_qgos_dn0_slot = var_qgos_dn0;
        *var_qgos_dn10_slot = var_qgos_dn10;
        *var_qgos_dn11_slot = var_qgos_dn11;
        *var_qgos_dn12_slot = var_qgos_dn12;
        *var_qgos_dn17_slot = var_qgos_dn17;
        *var_qgos_dn2_slot = var_qgos_dn2;
        *var_qgos_dn6_slot = var_qgos_dn6;
        *var_qgos_dn7_slot = var_qgos_dn7;
        *var_t1__blk1032_slot = var_t1__blk1032;
        *var_t1__blk1032_dn10_slot = var_t1__blk1032_dn10;
        *var_t1__blk1032_dn12_slot = var_t1__blk1032_dn12;
        *var_t1__blk1032_dn6_slot = var_t1__blk1032_dn6;
        *var_t1__blk1032_dn7_slot = var_t1__blk1032_dn7;
        *var_t1__blk898_slot = var_t1__blk898;
        *var_t1__blk898_dn0_slot = var_t1__blk898_dn0;
        *var_t1__blk898_dn10_slot = var_t1__blk898_dn10;
        *var_t1__blk898_dn11_slot = var_t1__blk898_dn11;
        *var_t1__blk898_dn12_slot = var_t1__blk898_dn12;
        *var_t1__blk898_dn17_slot = var_t1__blk898_dn17;
        *var_t1__blk898_dn2_slot = var_t1__blk898_dn2;
        *var_t1__blk898_dn6_slot = var_t1__blk898_dn6;
        *var_t1__blk898_dn7_slot = var_t1__blk898_dn7;
        *var_t2__blk1033_slot = var_t2__blk1033;
        *var_t2__blk1033_dn0_slot = var_t2__blk1033_dn0;
        *var_t2__blk1033_dn10_slot = var_t2__blk1033_dn10;
        *var_t2__blk1033_dn11_slot = var_t2__blk1033_dn11;
        *var_t2__blk1033_dn12_slot = var_t2__blk1033_dn12;
        *var_t2__blk1033_dn17_slot = var_t2__blk1033_dn17;
        *var_t2__blk1033_dn2_slot = var_t2__blk1033_dn2;
        *var_t2__blk1033_dn6_slot = var_t2__blk1033_dn6;
        *var_t2__blk1033_dn7_slot = var_t2__blk1033_dn7;
        *var_vbdj_slot = var_vbdj;
        *var_vbdj_dn12_slot = var_vbdj_dn12;
        *var_vbdj_dn6_slot = var_vbdj_dn6;
        *var_vbdt_slot = var_vbdt;
        *var_vbdt_dn10_slot = var_vbdt_dn10;
        *var_vbsj_slot = var_vbsj;
        *var_vbsj_dn12_slot = var_vbsj_dn12;
        *var_vbsj_dn7_slot = var_vbsj_dn7;
        *var_vbst_slot = var_vbst;
        *var_vbst_dn10_slot = var_vbst_dn10;
    }

    pub(super) fn stamp_transient_block_114(
        p: &Parameters,
        var_gjmin: f64,
        var_guard1030: f64,
        var_guard1059: f64,
        var_isbd: f64,
        var_isbd2: f64,
        var_isbd2_dn0: f64,
        var_isbd2_dn10: f64,
        var_isbd2_dn11: f64,
        var_isbd2_dn12: f64,
        var_isbd2_dn17: f64,
        var_isbd2_dn2: f64,
        var_isbd2_dn6: f64,
        var_isbd2_dn7: f64,
        var_isbd_dn0: f64,
        var_isbd_dn10: f64,
        var_isbd_dn11: f64,
        var_isbd_dn12: f64,
        var_isbd_dn17: f64,
        var_isbd_dn2: f64,
        var_isbd_dn6: f64,
        var_isbd_dn7: f64,
        var_isbs: f64,
        var_isbs2: f64,
        var_isbs2_dn0: f64,
        var_isbs2_dn10: f64,
        var_isbs2_dn11: f64,
        var_isbs2_dn12: f64,
        var_isbs2_dn17: f64,
        var_isbs2_dn2: f64,
        var_isbs2_dn6: f64,
        var_isbs2_dn7: f64,
        var_isbs_dn0: f64,
        var_isbs_dn10: f64,
        var_isbs_dn11: f64,
        var_isbs_dn12: f64,
        var_isbs_dn17: f64,
        var_isbs_dn2: f64,
        var_isbs_dn6: f64,
        var_isbs_dn7: f64,
        var_nvtm: f64,
        var_nvtm_dn10: f64,
        var_vbdj: f64,
        var_vbdj_dn12: f64,
        var_vbdj_dn6: f64,
        var_vbdt: f64,
        var_vbdt_dn10: f64,
        var_vbsj: f64,
        var_vbsj_dn12: f64,
        var_vbsj_dn7: f64,
        var_vbst: f64,
        var_vbst_dn10: f64,
        var_w_dioscv: f64,
        var_arg__blk1057_slot: &mut f64,
        var_arg__blk1057_dn12_slot: &mut f64,
        var_arg__blk1057_dn6_slot: &mut f64,
        var_arg__blk1057_dn7_slot: &mut f64,
        var_czbd_slot: &mut f64,
        var_czbs_slot: &mut f64,
        var_czbssw_slot: &mut f64,
        var_czbsswg_slot: &mut f64,
        var_guard1060_slot: &mut f64,
        var_guard1061_slot: &mut f64,
        var_guard1062_slot: &mut f64,
        var_guard1063_slot: &mut f64,
        var_guard1064_slot: &mut f64,
        var_guard1065_slot: &mut f64,
        var_guard1066_slot: &mut f64,
        var_guard1067_slot: &mut f64,
        var_guard1068_slot: &mut f64,
        var_guard1069_slot: &mut f64,
        var_ibd_slot: &mut f64,
        var_ibd_dn0_slot: &mut f64,
        var_ibd_dn10_slot: &mut f64,
        var_ibd_dn11_slot: &mut f64,
        var_ibd_dn12_slot: &mut f64,
        var_ibd_dn17_slot: &mut f64,
        var_ibd_dn2_slot: &mut f64,
        var_ibd_dn6_slot: &mut f64,
        var_ibd_dn7_slot: &mut f64,
        var_ibs_slot: &mut f64,
        var_ibs_dn0_slot: &mut f64,
        var_ibs_dn10_slot: &mut f64,
        var_ibs_dn11_slot: &mut f64,
        var_ibs_dn12_slot: &mut f64,
        var_ibs_dn17_slot: &mut f64,
        var_ibs_dn2_slot: &mut f64,
        var_ibs_dn6_slot: &mut f64,
        var_ibs_dn7_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn0_slot: &mut f64,
        var_qbs_dn10_slot: &mut f64,
        var_qbs_dn11_slot: &mut f64,
        var_qbs_dn12_slot: &mut f64,
        var_qbs_dn17_slot: &mut f64,
        var_qbs_dn2_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_sarg_slot: &mut f64,
        var_sarg_dn12_slot: &mut f64,
        var_sarg_dn6_slot: &mut f64,
        var_sarg_dn7_slot: &mut f64,
        var_t1__blk1032_slot: &mut f64,
        var_t1__blk1032_dn10_slot: &mut f64,
        var_t1__blk1032_dn12_slot: &mut f64,
        var_t1__blk1032_dn6_slot: &mut f64,
        var_t1__blk1032_dn7_slot: &mut f64,
        var_xp_max_slot: &mut f64,
    ) {
        let mut var_arg__blk1057: f64 = *var_arg__blk1057_slot;
        let mut var_arg__blk1057_dn12: f64 = *var_arg__blk1057_dn12_slot;
        let mut var_arg__blk1057_dn6: f64 = *var_arg__blk1057_dn6_slot;
        let mut var_arg__blk1057_dn7: f64 = *var_arg__blk1057_dn7_slot;
        let mut var_czbd: f64 = *var_czbd_slot;
        let mut var_czbs: f64 = *var_czbs_slot;
        let mut var_czbssw: f64 = *var_czbssw_slot;
        let mut var_czbsswg: f64 = *var_czbsswg_slot;
        let mut var_guard1060: f64 = *var_guard1060_slot;
        let mut var_guard1061: f64 = *var_guard1061_slot;
        let mut var_guard1062: f64 = *var_guard1062_slot;
        let mut var_guard1063: f64 = *var_guard1063_slot;
        let mut var_guard1064: f64 = *var_guard1064_slot;
        let mut var_guard1065: f64 = *var_guard1065_slot;
        let mut var_guard1066: f64 = *var_guard1066_slot;
        let mut var_guard1067: f64 = *var_guard1067_slot;
        let mut var_guard1068: f64 = *var_guard1068_slot;
        let mut var_guard1069: f64 = *var_guard1069_slot;
        let mut var_ibd: f64 = *var_ibd_slot;
        let mut var_ibd_dn0: f64 = *var_ibd_dn0_slot;
        let mut var_ibd_dn10: f64 = *var_ibd_dn10_slot;
        let mut var_ibd_dn11: f64 = *var_ibd_dn11_slot;
        let mut var_ibd_dn12: f64 = *var_ibd_dn12_slot;
        let mut var_ibd_dn17: f64 = *var_ibd_dn17_slot;
        let mut var_ibd_dn2: f64 = *var_ibd_dn2_slot;
        let mut var_ibd_dn6: f64 = *var_ibd_dn6_slot;
        let mut var_ibd_dn7: f64 = *var_ibd_dn7_slot;
        let mut var_ibs: f64 = *var_ibs_slot;
        let mut var_ibs_dn0: f64 = *var_ibs_dn0_slot;
        let mut var_ibs_dn10: f64 = *var_ibs_dn10_slot;
        let mut var_ibs_dn11: f64 = *var_ibs_dn11_slot;
        let mut var_ibs_dn12: f64 = *var_ibs_dn12_slot;
        let mut var_ibs_dn17: f64 = *var_ibs_dn17_slot;
        let mut var_ibs_dn2: f64 = *var_ibs_dn2_slot;
        let mut var_ibs_dn6: f64 = *var_ibs_dn6_slot;
        let mut var_ibs_dn7: f64 = *var_ibs_dn7_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn0: f64 = *var_qbs_dn0_slot;
        let mut var_qbs_dn10: f64 = *var_qbs_dn10_slot;
        let mut var_qbs_dn11: f64 = *var_qbs_dn11_slot;
        let mut var_qbs_dn12: f64 = *var_qbs_dn12_slot;
        let mut var_qbs_dn17: f64 = *var_qbs_dn17_slot;
        let mut var_qbs_dn2: f64 = *var_qbs_dn2_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_sarg: f64 = *var_sarg_slot;
        let mut var_sarg_dn12: f64 = *var_sarg_dn12_slot;
        let mut var_sarg_dn6: f64 = *var_sarg_dn6_slot;
        let mut var_sarg_dn7: f64 = *var_sarg_dn7_slot;
        let mut var_t1__blk1032: f64 = *var_t1__blk1032_slot;
        let mut var_t1__blk1032_dn10: f64 = *var_t1__blk1032_dn10_slot;
        let mut var_t1__blk1032_dn12: f64 = *var_t1__blk1032_dn12_slot;
        let mut var_t1__blk1032_dn6: f64 = *var_t1__blk1032_dn6_slot;
        let mut var_t1__blk1032_dn7: f64 = *var_t1__blk1032_dn7_slot;
        let mut var_xp_max: f64 = *var_xp_max_slot;

        let (assign32110_e47063, assign32110_e47063_d_n6, assign32110_e47063_d_n7, assign32110_e47063_d_n10, assign32110_e47063_d_n12,) = {
    if ((var_guard1030 != 0.0) && (var_guard1059 != 0.0)) {
        let assign32110_e47060: f64 = (var_vbdj / var_nvtm);
        let assign32110_e47061: f64 = (assign32110_e47060).exp();
        (assign32110_e47061, (assign32110_e47061 * (var_vbdj_dn6 / var_nvtm)), 0.0, (assign32110_e47061 * (-((var_vbdj * var_nvtm_dn10) / (var_nvtm * var_nvtm)))), (assign32110_e47061 * (var_vbdj_dn12 / var_nvtm)),)
    } else {
        (var_t1__blk1032, var_t1__blk1032_dn6, var_t1__blk1032_dn7, var_t1__blk1032_dn10, var_t1__blk1032_dn12,)
    }
};
        var_t1__blk1032 = assign32110_e47063;
        var_t1__blk1032_dn6 = assign32110_e47063_d_n6;
        var_t1__blk1032_dn7 = assign32110_e47063_d_n7;
        var_t1__blk1032_dn10 = assign32110_e47063_d_n10;
        var_t1__blk1032_dn12 = assign32110_e47063_d_n12;

        let (assign32120_e47073, assign32120_e47073_d_n0, assign32120_e47073_d_n2, assign32120_e47073_d_n6, assign32120_e47073_d_n7, assign32120_e47073_d_n10, assign32120_e47073_d_n11, assign32120_e47073_d_n12, assign32120_e47073_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1059 != 0.0)) {
        let assign32120_e47070: f64 = (var_t1__blk1032 - 1.0);
        let assign32120_e47071: f64 = (var_isbd * assign32120_e47070);
        (assign32120_e47071, (var_isbd_dn0 * assign32120_e47070), (var_isbd_dn2 * assign32120_e47070), ((var_isbd_dn6 * assign32120_e47070) + (var_isbd * var_t1__blk1032_dn6)), ((var_isbd_dn7 * assign32120_e47070) + (var_isbd * var_t1__blk1032_dn7)), ((var_isbd_dn10 * assign32120_e47070) + (var_isbd * var_t1__blk1032_dn10)), (var_isbd_dn11 * assign32120_e47070), ((var_isbd_dn12 * assign32120_e47070) + (var_isbd * var_t1__blk1032_dn12)), (var_isbd_dn17 * assign32120_e47070),)
    } else {
        (var_ibd, var_ibd_dn0, var_ibd_dn2, var_ibd_dn6, var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, var_ibd_dn12, var_ibd_dn17,)
    }
};
        var_ibd = assign32120_e47073;
        var_ibd_dn0 = assign32120_e47073_d_n0;
        var_ibd_dn2 = assign32120_e47073_d_n2;
        var_ibd_dn6 = assign32120_e47073_d_n6;
        var_ibd_dn7 = assign32120_e47073_d_n7;
        var_ibd_dn10 = assign32120_e47073_d_n10;
        var_ibd_dn11 = assign32120_e47073_d_n11;
        var_ibd_dn12 = assign32120_e47073_d_n12;
        var_ibd_dn17 = assign32120_e47073_d_n17;

        let (assign32130_e47083, assign32130_e47083_d_n6, assign32130_e47083_d_n7, assign32130_e47083_d_n10, assign32130_e47083_d_n12,) = {
    if ((var_guard1030 != 0.0) && (var_guard1059 == 0.0)) {
        let assign32130_e47080: f64 = (var_vbdt / var_nvtm);
        let assign32130_e47081: f64 = (assign32130_e47080).exp();
        (assign32130_e47081, 0.0, 0.0, (assign32130_e47081 * (((var_vbdt_dn10 * var_nvtm) - (var_vbdt * var_nvtm_dn10)) / (var_nvtm * var_nvtm))), 0.0,)
    } else {
        (var_t1__blk1032, var_t1__blk1032_dn6, var_t1__blk1032_dn7, var_t1__blk1032_dn10, var_t1__blk1032_dn12,)
    }
};
        var_t1__blk1032 = assign32130_e47083;
        var_t1__blk1032_dn6 = assign32130_e47083_d_n6;
        var_t1__blk1032_dn7 = assign32130_e47083_d_n7;
        var_t1__blk1032_dn10 = assign32130_e47083_d_n10;
        var_t1__blk1032_dn12 = assign32130_e47083_d_n12;

        let (assign32140_e47104, assign32140_e47104_d_n0, assign32140_e47104_d_n2, assign32140_e47104_d_n6, assign32140_e47104_d_n7, assign32140_e47104_d_n10, assign32140_e47104_d_n11, assign32140_e47104_d_n12, assign32140_e47104_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1059 == 0.0)) {
        let assign32140_e47091: f64 = (var_t1__blk1032 - 1.0);
        let assign32140_e47092: f64 = (var_isbd * assign32140_e47091);
        let assign32140_e47095: f64 = (var_isbd / var_nvtm);
        let assign32140_e47097: f64 = (assign32140_e47095 * var_t1__blk1032);
        let assign32140_e47100: f64 = (var_vbdj - var_vbdt);
        let assign32140_e47101: f64 = (assign32140_e47097 * assign32140_e47100);
        let assign32140_e47102: f64 = (assign32140_e47092 + assign32140_e47101);
        (assign32140_e47102, ((var_isbd_dn0 * assign32140_e47091) + (((var_isbd_dn0 / var_nvtm) * var_t1__blk1032) * assign32140_e47100)), ((var_isbd_dn2 * assign32140_e47091) + (((var_isbd_dn2 / var_nvtm) * var_t1__blk1032) * assign32140_e47100)), (((var_isbd_dn6 * assign32140_e47091) + (var_isbd * var_t1__blk1032_dn6)) + (((((var_isbd_dn6 / var_nvtm) * var_t1__blk1032) + (assign32140_e47095 * var_t1__blk1032_dn6)) * assign32140_e47100) + (assign32140_e47097 * var_vbdj_dn6))), (((var_isbd_dn7 * assign32140_e47091) + (var_isbd * var_t1__blk1032_dn7)) + ((((var_isbd_dn7 / var_nvtm) * var_t1__blk1032) + (assign32140_e47095 * var_t1__blk1032_dn7)) * assign32140_e47100)), (((var_isbd_dn10 * assign32140_e47091) + (var_isbd * var_t1__blk1032_dn10)) + (((((((var_isbd_dn10 * var_nvtm) - (var_isbd * var_nvtm_dn10)) / (var_nvtm * var_nvtm)) * var_t1__blk1032) + (assign32140_e47095 * var_t1__blk1032_dn10)) * assign32140_e47100) + (assign32140_e47097 * (-var_vbdt_dn10)))), ((var_isbd_dn11 * assign32140_e47091) + (((var_isbd_dn11 / var_nvtm) * var_t1__blk1032) * assign32140_e47100)), (((var_isbd_dn12 * assign32140_e47091) + (var_isbd * var_t1__blk1032_dn12)) + (((((var_isbd_dn12 / var_nvtm) * var_t1__blk1032) + (assign32140_e47095 * var_t1__blk1032_dn12)) * assign32140_e47100) + (assign32140_e47097 * var_vbdj_dn12))), ((var_isbd_dn17 * assign32140_e47091) + (((var_isbd_dn17 / var_nvtm) * var_t1__blk1032) * assign32140_e47100)),)
    } else {
        (var_ibd, var_ibd_dn0, var_ibd_dn2, var_ibd_dn6, var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, var_ibd_dn12, var_ibd_dn17,)
    }
};
        var_ibd = assign32140_e47104;
        var_ibd_dn0 = assign32140_e47104_d_n0;
        var_ibd_dn2 = assign32140_e47104_d_n2;
        var_ibd_dn6 = assign32140_e47104_d_n6;
        var_ibd_dn7 = assign32140_e47104_d_n7;
        var_ibd_dn10 = assign32140_e47104_d_n10;
        var_ibd_dn11 = assign32140_e47104_d_n11;
        var_ibd_dn12 = assign32140_e47104_d_n12;
        var_ibd_dn17 = assign32140_e47104_d_n17;

        let (assign32150_e47114, assign32150_e47114_d_n0, assign32150_e47114_d_n2, assign32150_e47114_d_n6, assign32150_e47114_d_n7, assign32150_e47114_d_n10, assign32150_e47114_d_n11, assign32150_e47114_d_n12, assign32150_e47114_d_n17,) = {
    if (var_guard1030 != 0.0) {
        let assign32150_e47109: f64 = (p.p178 * var_vbdj);
        let assign32150_e47111: f64 = (assign32150_e47109 * var_isbd2);
        let assign32150_e47112: f64 = (var_ibd + assign32150_e47111);
        (assign32150_e47112, (var_ibd_dn0 + (assign32150_e47109 * var_isbd2_dn0)), (var_ibd_dn2 + (assign32150_e47109 * var_isbd2_dn2)), (var_ibd_dn6 + (((p.p178 * var_vbdj_dn6) * var_isbd2) + (assign32150_e47109 * var_isbd2_dn6))), (var_ibd_dn7 + (assign32150_e47109 * var_isbd2_dn7)), (var_ibd_dn10 + (assign32150_e47109 * var_isbd2_dn10)), (var_ibd_dn11 + (assign32150_e47109 * var_isbd2_dn11)), (var_ibd_dn12 + (((p.p178 * var_vbdj_dn12) * var_isbd2) + (assign32150_e47109 * var_isbd2_dn12))), (var_ibd_dn17 + (assign32150_e47109 * var_isbd2_dn17)),)
    } else {
        (var_ibd, var_ibd_dn0, var_ibd_dn2, var_ibd_dn6, var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, var_ibd_dn12, var_ibd_dn17,)
    }
};
        var_ibd = assign32150_e47114;
        var_ibd_dn0 = assign32150_e47114_d_n0;
        var_ibd_dn2 = assign32150_e47114_d_n2;
        var_ibd_dn6 = assign32150_e47114_d_n6;
        var_ibd_dn7 = assign32150_e47114_d_n7;
        var_ibd_dn10 = assign32150_e47114_d_n10;
        var_ibd_dn11 = assign32150_e47114_d_n11;
        var_ibd_dn12 = assign32150_e47114_d_n12;
        var_ibd_dn17 = assign32150_e47114_d_n17;

        let assign32160_e47117: f64 = if var_vbsj < var_vbst { 1.0 } else { 0.0 };
        var_guard1060 = assign32160_e47117;

        let (assign32170_e47126, assign32170_e47126_d_n6, assign32170_e47126_d_n7, assign32170_e47126_d_n10, assign32170_e47126_d_n12,) = {
    if ((var_guard1030 != 0.0) && (var_guard1060 != 0.0)) {
        let assign32170_e47123: f64 = (var_vbsj / var_nvtm);
        let assign32170_e47124: f64 = (assign32170_e47123).exp();
        (assign32170_e47124, 0.0, (assign32170_e47124 * (var_vbsj_dn7 / var_nvtm)), (assign32170_e47124 * (-((var_vbsj * var_nvtm_dn10) / (var_nvtm * var_nvtm)))), (assign32170_e47124 * (var_vbsj_dn12 / var_nvtm)),)
    } else {
        (var_t1__blk1032, var_t1__blk1032_dn6, var_t1__blk1032_dn7, var_t1__blk1032_dn10, var_t1__blk1032_dn12,)
    }
};
        var_t1__blk1032 = assign32170_e47126;
        var_t1__blk1032_dn6 = assign32170_e47126_d_n6;
        var_t1__blk1032_dn7 = assign32170_e47126_d_n7;
        var_t1__blk1032_dn10 = assign32170_e47126_d_n10;
        var_t1__blk1032_dn12 = assign32170_e47126_d_n12;

        let (assign32180_e47136, assign32180_e47136_d_n0, assign32180_e47136_d_n2, assign32180_e47136_d_n6, assign32180_e47136_d_n7, assign32180_e47136_d_n10, assign32180_e47136_d_n11, assign32180_e47136_d_n12, assign32180_e47136_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1060 != 0.0)) {
        let assign32180_e47133: f64 = (var_t1__blk1032 - 1.0);
        let assign32180_e47134: f64 = (var_isbs * assign32180_e47133);
        (assign32180_e47134, (var_isbs_dn0 * assign32180_e47133), (var_isbs_dn2 * assign32180_e47133), ((var_isbs_dn6 * assign32180_e47133) + (var_isbs * var_t1__blk1032_dn6)), ((var_isbs_dn7 * assign32180_e47133) + (var_isbs * var_t1__blk1032_dn7)), ((var_isbs_dn10 * assign32180_e47133) + (var_isbs * var_t1__blk1032_dn10)), (var_isbs_dn11 * assign32180_e47133), ((var_isbs_dn12 * assign32180_e47133) + (var_isbs * var_t1__blk1032_dn12)), (var_isbs_dn17 * assign32180_e47133),)
    } else {
        (var_ibs, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, var_ibs_dn7, var_ibs_dn10, var_ibs_dn11, var_ibs_dn12, var_ibs_dn17,)
    }
};
        var_ibs = assign32180_e47136;
        var_ibs_dn0 = assign32180_e47136_d_n0;
        var_ibs_dn2 = assign32180_e47136_d_n2;
        var_ibs_dn6 = assign32180_e47136_d_n6;
        var_ibs_dn7 = assign32180_e47136_d_n7;
        var_ibs_dn10 = assign32180_e47136_d_n10;
        var_ibs_dn11 = assign32180_e47136_d_n11;
        var_ibs_dn12 = assign32180_e47136_d_n12;
        var_ibs_dn17 = assign32180_e47136_d_n17;

        let (assign32190_e47146, assign32190_e47146_d_n6, assign32190_e47146_d_n7, assign32190_e47146_d_n10, assign32190_e47146_d_n12,) = {
    if ((var_guard1030 != 0.0) && (var_guard1060 == 0.0)) {
        let assign32190_e47143: f64 = (var_vbst / var_nvtm);
        let assign32190_e47144: f64 = (assign32190_e47143).exp();
        (assign32190_e47144, 0.0, 0.0, (assign32190_e47144 * (((var_vbst_dn10 * var_nvtm) - (var_vbst * var_nvtm_dn10)) / (var_nvtm * var_nvtm))), 0.0,)
    } else {
        (var_t1__blk1032, var_t1__blk1032_dn6, var_t1__blk1032_dn7, var_t1__blk1032_dn10, var_t1__blk1032_dn12,)
    }
};
        var_t1__blk1032 = assign32190_e47146;
        var_t1__blk1032_dn6 = assign32190_e47146_d_n6;
        var_t1__blk1032_dn7 = assign32190_e47146_d_n7;
        var_t1__blk1032_dn10 = assign32190_e47146_d_n10;
        var_t1__blk1032_dn12 = assign32190_e47146_d_n12;

        let (assign32200_e47167, assign32200_e47167_d_n0, assign32200_e47167_d_n2, assign32200_e47167_d_n6, assign32200_e47167_d_n7, assign32200_e47167_d_n10, assign32200_e47167_d_n11, assign32200_e47167_d_n12, assign32200_e47167_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1060 == 0.0)) {
        let assign32200_e47154: f64 = (var_t1__blk1032 - 1.0);
        let assign32200_e47155: f64 = (var_isbs * assign32200_e47154);
        let assign32200_e47158: f64 = (var_isbs / var_nvtm);
        let assign32200_e47160: f64 = (assign32200_e47158 * var_t1__blk1032);
        let assign32200_e47163: f64 = (var_vbsj - var_vbst);
        let assign32200_e47164: f64 = (assign32200_e47160 * assign32200_e47163);
        let assign32200_e47165: f64 = (assign32200_e47155 + assign32200_e47164);
        (assign32200_e47165, ((var_isbs_dn0 * assign32200_e47154) + (((var_isbs_dn0 / var_nvtm) * var_t1__blk1032) * assign32200_e47163)), ((var_isbs_dn2 * assign32200_e47154) + (((var_isbs_dn2 / var_nvtm) * var_t1__blk1032) * assign32200_e47163)), (((var_isbs_dn6 * assign32200_e47154) + (var_isbs * var_t1__blk1032_dn6)) + ((((var_isbs_dn6 / var_nvtm) * var_t1__blk1032) + (assign32200_e47158 * var_t1__blk1032_dn6)) * assign32200_e47163)), (((var_isbs_dn7 * assign32200_e47154) + (var_isbs * var_t1__blk1032_dn7)) + (((((var_isbs_dn7 / var_nvtm) * var_t1__blk1032) + (assign32200_e47158 * var_t1__blk1032_dn7)) * assign32200_e47163) + (assign32200_e47160 * var_vbsj_dn7))), (((var_isbs_dn10 * assign32200_e47154) + (var_isbs * var_t1__blk1032_dn10)) + (((((((var_isbs_dn10 * var_nvtm) - (var_isbs * var_nvtm_dn10)) / (var_nvtm * var_nvtm)) * var_t1__blk1032) + (assign32200_e47158 * var_t1__blk1032_dn10)) * assign32200_e47163) + (assign32200_e47160 * (-var_vbst_dn10)))), ((var_isbs_dn11 * assign32200_e47154) + (((var_isbs_dn11 / var_nvtm) * var_t1__blk1032) * assign32200_e47163)), (((var_isbs_dn12 * assign32200_e47154) + (var_isbs * var_t1__blk1032_dn12)) + (((((var_isbs_dn12 / var_nvtm) * var_t1__blk1032) + (assign32200_e47158 * var_t1__blk1032_dn12)) * assign32200_e47163) + (assign32200_e47160 * var_vbsj_dn12))), ((var_isbs_dn17 * assign32200_e47154) + (((var_isbs_dn17 / var_nvtm) * var_t1__blk1032) * assign32200_e47163)),)
    } else {
        (var_ibs, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, var_ibs_dn7, var_ibs_dn10, var_ibs_dn11, var_ibs_dn12, var_ibs_dn17,)
    }
};
        var_ibs = assign32200_e47167;
        var_ibs_dn0 = assign32200_e47167_d_n0;
        var_ibs_dn2 = assign32200_e47167_d_n2;
        var_ibs_dn6 = assign32200_e47167_d_n6;
        var_ibs_dn7 = assign32200_e47167_d_n7;
        var_ibs_dn10 = assign32200_e47167_d_n10;
        var_ibs_dn11 = assign32200_e47167_d_n11;
        var_ibs_dn12 = assign32200_e47167_d_n12;
        var_ibs_dn17 = assign32200_e47167_d_n17;

        let (assign32210_e47177, assign32210_e47177_d_n0, assign32210_e47177_d_n2, assign32210_e47177_d_n6, assign32210_e47177_d_n7, assign32210_e47177_d_n10, assign32210_e47177_d_n11, assign32210_e47177_d_n12, assign32210_e47177_d_n17,) = {
    if (var_guard1030 != 0.0) {
        let assign32210_e47172: f64 = (p.p178 * var_vbsj);
        let assign32210_e47174: f64 = (assign32210_e47172 * var_isbs2);
        let assign32210_e47175: f64 = (var_ibs + assign32210_e47174);
        (assign32210_e47175, (var_ibs_dn0 + (assign32210_e47172 * var_isbs2_dn0)), (var_ibs_dn2 + (assign32210_e47172 * var_isbs2_dn2)), (var_ibs_dn6 + (assign32210_e47172 * var_isbs2_dn6)), (var_ibs_dn7 + (((p.p178 * var_vbsj_dn7) * var_isbs2) + (assign32210_e47172 * var_isbs2_dn7))), (var_ibs_dn10 + (assign32210_e47172 * var_isbs2_dn10)), (var_ibs_dn11 + (assign32210_e47172 * var_isbs2_dn11)), (var_ibs_dn12 + (((p.p178 * var_vbsj_dn12) * var_isbs2) + (assign32210_e47172 * var_isbs2_dn12))), (var_ibs_dn17 + (assign32210_e47172 * var_isbs2_dn17)),)
    } else {
        (var_ibs, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, var_ibs_dn7, var_ibs_dn10, var_ibs_dn11, var_ibs_dn12, var_ibs_dn17,)
    }
};
        var_ibs = assign32210_e47177;
        var_ibs_dn0 = assign32210_e47177_d_n0;
        var_ibs_dn2 = assign32210_e47177_d_n2;
        var_ibs_dn6 = assign32210_e47177_d_n6;
        var_ibs_dn7 = assign32210_e47177_d_n7;
        var_ibs_dn10 = assign32210_e47177_d_n10;
        var_ibs_dn11 = assign32210_e47177_d_n11;
        var_ibs_dn12 = assign32210_e47177_d_n12;
        var_ibs_dn17 = assign32210_e47177_d_n17;

        let (assign32220_e47185, assign32220_e47185_d_n0, assign32220_e47185_d_n2, assign32220_e47185_d_n6, assign32220_e47185_d_n7, assign32220_e47185_d_n10, assign32220_e47185_d_n11, assign32220_e47185_d_n12, assign32220_e47185_d_n17,) = {
    if (var_guard1030 != 0.0) {
        let assign32220_e47182: f64 = (var_gjmin * var_vbdj);
        let assign32220_e47183: f64 = (var_ibd + assign32220_e47182);
        (assign32220_e47183, var_ibd_dn0, var_ibd_dn2, (var_ibd_dn6 + (var_gjmin * var_vbdj_dn6)), var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, (var_ibd_dn12 + (var_gjmin * var_vbdj_dn12)), var_ibd_dn17,)
    } else {
        (var_ibd, var_ibd_dn0, var_ibd_dn2, var_ibd_dn6, var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, var_ibd_dn12, var_ibd_dn17,)
    }
};
        var_ibd = assign32220_e47185;
        var_ibd_dn0 = assign32220_e47185_d_n0;
        var_ibd_dn2 = assign32220_e47185_d_n2;
        var_ibd_dn6 = assign32220_e47185_d_n6;
        var_ibd_dn7 = assign32220_e47185_d_n7;
        var_ibd_dn10 = assign32220_e47185_d_n10;
        var_ibd_dn11 = assign32220_e47185_d_n11;
        var_ibd_dn12 = assign32220_e47185_d_n12;
        var_ibd_dn17 = assign32220_e47185_d_n17;

        let (assign32230_e47193, assign32230_e47193_d_n0, assign32230_e47193_d_n2, assign32230_e47193_d_n6, assign32230_e47193_d_n7, assign32230_e47193_d_n10, assign32230_e47193_d_n11, assign32230_e47193_d_n12, assign32230_e47193_d_n17,) = {
    if (var_guard1030 != 0.0) {
        let assign32230_e47190: f64 = (var_gjmin * var_vbsj);
        let assign32230_e47191: f64 = (var_ibs + assign32230_e47190);
        (assign32230_e47191, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, (var_ibs_dn7 + (var_gjmin * var_vbsj_dn7)), var_ibs_dn10, var_ibs_dn11, (var_ibs_dn12 + (var_gjmin * var_vbsj_dn12)), var_ibs_dn17,)
    } else {
        (var_ibs, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, var_ibs_dn7, var_ibs_dn10, var_ibs_dn11, var_ibs_dn12, var_ibs_dn17,)
    }
};
        var_ibs = assign32230_e47193;
        var_ibs_dn0 = assign32230_e47193_d_n0;
        var_ibs_dn2 = assign32230_e47193_d_n2;
        var_ibs_dn6 = assign32230_e47193_d_n6;
        var_ibs_dn7 = assign32230_e47193_d_n7;
        var_ibs_dn10 = assign32230_e47193_d_n10;
        var_ibs_dn11 = assign32230_e47193_d_n11;
        var_ibs_dn12 = assign32230_e47193_d_n12;
        var_ibs_dn17 = assign32230_e47193_d_n17;

        let (assign32240_e47199,) = {
    if (var_guard1030 != 0.0) {
        let assign32240_e47197: f64 = (p.p179 * p.p2);
        (assign32240_e47197,)
    } else {
        (var_czbd,)
    }
};
        var_czbd = assign32240_e47199;

        let (assign32250_e47205,) = {
    if (var_guard1030 != 0.0) {
        let assign32250_e47203: f64 = (p.p179 * p.p3);
        (assign32250_e47203,)
    } else {
        (var_czbs,)
    }
};
        var_czbs = assign32250_e47205;

        let (assign32260_e47211,) = {
    if (var_guard1030 != 0.0) {
        let assign32260_e47209: f64 = (p.p237 - p.p238);
        (assign32260_e47209,)
    } else {
        (var_xp_max,)
    }
};
        var_xp_max = assign32260_e47211;

        let assign32270_e47214: f64 = if var_xp_max <= 0.0 { 1.0 } else { 0.0 };
        var_guard1061 = assign32270_e47214;

        let (assign32280_e47220,) = {
    if ((var_guard1030 != 0.0) && (var_guard1061 != 0.0)) {
        (0.0,)
    } else {
        (var_czbd,)
    }
};
        var_czbd = assign32280_e47220;

        let (assign32290_e47226,) = {
    if ((var_guard1030 != 0.0) && (var_guard1061 != 0.0)) {
        (0.0,)
    } else {
        (var_czbs,)
    }
};
        var_czbs = assign32290_e47226;

        let assign32300_e47229: f64 = if p.p5 > var_w_dioscv { 1.0 } else { 0.0 };
        var_guard1062 = assign32300_e47229;

        let (assign32310_e47239,) = {
    if ((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) {
        let assign32310_e47236: f64 = (p.p5 - var_w_dioscv);
        let assign32310_e47237: f64 = (p.p180 * assign32310_e47236);
        (assign32310_e47237,)
    } else {
        (var_czbssw,)
    }
};
        var_czbssw = assign32310_e47239;

        let (assign32320_e47247,) = {
    if ((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) {
        let assign32320_e47245: f64 = (p.p181 * var_w_dioscv);
        (assign32320_e47245,)
    } else {
        (var_czbsswg,)
    }
};
        var_czbsswg = assign32320_e47247;

        let assign32330_e47250: f64 = if var_vbsj < 0.0 { 1.0 } else { 0.0 };
        var_guard1063 = assign32330_e47250;

        let assign32340_e47253: f64 = if var_czbs > 0.0 { 1.0 } else { 0.0 };
        var_guard1064 = assign32340_e47253;

        let (assign32350_e47267, assign32350_e47267_d_n6, assign32350_e47267_d_n7, assign32350_e47267_d_n12,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1064 != 0.0)) {
        let assign32350_e47264: f64 = (var_vbsj / p.p185);
        let assign32350_e47265: f64 = (1.0 - assign32350_e47264);
        (assign32350_e47265, 0.0, (-(var_vbsj_dn7 / p.p185)), (-(var_vbsj_dn12 / p.p185)),)
    } else {
        (var_arg__blk1057, var_arg__blk1057_dn6, var_arg__blk1057_dn7, var_arg__blk1057_dn12,)
    }
};
        var_arg__blk1057 = assign32350_e47267;
        var_arg__blk1057_dn6 = assign32350_e47267_d_n6;
        var_arg__blk1057_dn7 = assign32350_e47267_d_n7;
        var_arg__blk1057_dn12 = assign32350_e47267_d_n12;

        let assign32360_e47270: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        var_guard1065 = assign32360_e47270;

        let (assign32370_e47285, assign32370_e47285_d_n6, assign32370_e47285_d_n7, assign32370_e47285_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) {
        let assign32370_e47282: f64 = (var_arg__blk1057).sqrt();
        let assign32370_e47283: f64 = (1.0 / assign32370_e47282);
        (assign32370_e47283, (-((var_arg__blk1057_dn6 / (2.0 * assign32370_e47282)) / (assign32370_e47282 * assign32370_e47282))), (-((var_arg__blk1057_dn7 / (2.0 * assign32370_e47282)) / (assign32370_e47282 * assign32370_e47282))), (-((var_arg__blk1057_dn12 / (2.0 * assign32370_e47282)) / (assign32370_e47282 * assign32370_e47282))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32370_e47285;
        var_sarg_dn6 = assign32370_e47285_d_n6;
        var_sarg_dn7 = assign32370_e47285_d_n7;
        var_sarg_dn12 = assign32370_e47285_d_n12;

        let (assign32380_e47301, assign32380_e47301_d_n6, assign32380_e47301_d_n7, assign32380_e47301_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1064 != 0.0)) && (var_guard1065 == 0.0)) {
        let assign32380_e47298: f64 = (-p.p182);
        let assign32380_e47299: f64 = (var_arg__blk1057).powf(assign32380_e47298);
        (assign32380_e47299, if 0.0 == 0.0 && ((assign32380_e47298) as f64).is_finite() && ((assign32380_e47298) as f64).fract() == 0.0 { if assign32380_e47298 == 0.0 { 0.0 } else { (assign32380_e47298 * ((var_arg__blk1057).powf(assign32380_e47298 - 1.0) * var_arg__blk1057_dn6)) } } else { (assign32380_e47299 * (assign32380_e47298 * (var_arg__blk1057_dn6 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32380_e47298) as f64).is_finite() && ((assign32380_e47298) as f64).fract() == 0.0 { if assign32380_e47298 == 0.0 { 0.0 } else { (assign32380_e47298 * ((var_arg__blk1057).powf(assign32380_e47298 - 1.0) * var_arg__blk1057_dn7)) } } else { (assign32380_e47299 * (assign32380_e47298 * (var_arg__blk1057_dn7 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32380_e47298) as f64).is_finite() && ((assign32380_e47298) as f64).fract() == 0.0 { if assign32380_e47298 == 0.0 { 0.0 } else { (assign32380_e47298 * ((var_arg__blk1057).powf(assign32380_e47298 - 1.0) * var_arg__blk1057_dn12)) } } else { (assign32380_e47299 * (assign32380_e47298 * (var_arg__blk1057_dn12 / var_arg__blk1057))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32380_e47301;
        var_sarg_dn6 = assign32380_e47301_d_n6;
        var_sarg_dn7 = assign32380_e47301_d_n7;
        var_sarg_dn12 = assign32380_e47301_d_n12;

        let (assign32390_e47323, assign32390_e47323_d_n0, assign32390_e47323_d_n2, assign32390_e47323_d_n6, assign32390_e47323_d_n7, assign32390_e47323_d_n10, assign32390_e47323_d_n11, assign32390_e47323_d_n12, assign32390_e47323_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1064 != 0.0)) {
        let assign32390_e47311: f64 = (p.p185 * var_czbs);
        let assign32390_e47315: f64 = (var_arg__blk1057 * var_sarg);
        let assign32390_e47316: f64 = (1.0 - assign32390_e47315);
        let assign32390_e47317: f64 = (assign32390_e47311 * assign32390_e47316);
        let assign32390_e47320: f64 = (1.0 - p.p182);
        let assign32390_e47321: f64 = (assign32390_e47317 / assign32390_e47320);
        (assign32390_e47321, 0.0, 0.0, ((assign32390_e47311 * (-((var_arg__blk1057_dn6 * var_sarg) + (var_arg__blk1057 * var_sarg_dn6)))) / assign32390_e47320), ((assign32390_e47311 * (-((var_arg__blk1057_dn7 * var_sarg) + (var_arg__blk1057 * var_sarg_dn7)))) / assign32390_e47320), 0.0, 0.0, ((assign32390_e47311 * (-((var_arg__blk1057_dn12 * var_sarg) + (var_arg__blk1057 * var_sarg_dn12)))) / assign32390_e47320), 0.0,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32390_e47323;
        var_qbs_dn0 = assign32390_e47323_d_n0;
        var_qbs_dn2 = assign32390_e47323_d_n2;
        var_qbs_dn6 = assign32390_e47323_d_n6;
        var_qbs_dn7 = assign32390_e47323_d_n7;
        var_qbs_dn10 = assign32390_e47323_d_n10;
        var_qbs_dn11 = assign32390_e47323_d_n11;
        var_qbs_dn12 = assign32390_e47323_d_n12;
        var_qbs_dn17 = assign32390_e47323_d_n17;

        let (assign32400_e47334, assign32400_e47334_d_n0, assign32400_e47334_d_n2, assign32400_e47334_d_n6, assign32400_e47334_d_n7, assign32400_e47334_d_n10, assign32400_e47334_d_n11, assign32400_e47334_d_n12, assign32400_e47334_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1064 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32400_e47334;
        var_qbs_dn0 = assign32400_e47334_d_n0;
        var_qbs_dn2 = assign32400_e47334_d_n2;
        var_qbs_dn6 = assign32400_e47334_d_n6;
        var_qbs_dn7 = assign32400_e47334_d_n7;
        var_qbs_dn10 = assign32400_e47334_d_n10;
        var_qbs_dn11 = assign32400_e47334_d_n11;
        var_qbs_dn12 = assign32400_e47334_d_n12;
        var_qbs_dn17 = assign32400_e47334_d_n17;

        let assign32410_e47337: f64 = if var_czbssw > 0.0 { 1.0 } else { 0.0 };
        var_guard1066 = assign32410_e47337;

        let (assign32420_e47351, assign32420_e47351_d_n6, assign32420_e47351_d_n7, assign32420_e47351_d_n12,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1066 != 0.0)) {
        let assign32420_e47348: f64 = (var_vbsj / p.p186);
        let assign32420_e47349: f64 = (1.0 - assign32420_e47348);
        (assign32420_e47349, 0.0, (-(var_vbsj_dn7 / p.p186)), (-(var_vbsj_dn12 / p.p186)),)
    } else {
        (var_arg__blk1057, var_arg__blk1057_dn6, var_arg__blk1057_dn7, var_arg__blk1057_dn12,)
    }
};
        var_arg__blk1057 = assign32420_e47351;
        var_arg__blk1057_dn6 = assign32420_e47351_d_n6;
        var_arg__blk1057_dn7 = assign32420_e47351_d_n7;
        var_arg__blk1057_dn12 = assign32420_e47351_d_n12;

        let assign32430_e47354: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        var_guard1067 = assign32430_e47354;

        let (assign32440_e47369, assign32440_e47369_d_n6, assign32440_e47369_d_n7, assign32440_e47369_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1066 != 0.0)) && (var_guard1067 != 0.0)) {
        let assign32440_e47366: f64 = (var_arg__blk1057).sqrt();
        let assign32440_e47367: f64 = (1.0 / assign32440_e47366);
        (assign32440_e47367, (-((var_arg__blk1057_dn6 / (2.0 * assign32440_e47366)) / (assign32440_e47366 * assign32440_e47366))), (-((var_arg__blk1057_dn7 / (2.0 * assign32440_e47366)) / (assign32440_e47366 * assign32440_e47366))), (-((var_arg__blk1057_dn12 / (2.0 * assign32440_e47366)) / (assign32440_e47366 * assign32440_e47366))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32440_e47369;
        var_sarg_dn6 = assign32440_e47369_d_n6;
        var_sarg_dn7 = assign32440_e47369_d_n7;
        var_sarg_dn12 = assign32440_e47369_d_n12;

        let (assign32450_e47385, assign32450_e47385_d_n6, assign32450_e47385_d_n7, assign32450_e47385_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1066 != 0.0)) && (var_guard1067 == 0.0)) {
        let assign32450_e47382: f64 = (-p.p183);
        let assign32450_e47383: f64 = (var_arg__blk1057).powf(assign32450_e47382);
        (assign32450_e47383, if 0.0 == 0.0 && ((assign32450_e47382) as f64).is_finite() && ((assign32450_e47382) as f64).fract() == 0.0 { if assign32450_e47382 == 0.0 { 0.0 } else { (assign32450_e47382 * ((var_arg__blk1057).powf(assign32450_e47382 - 1.0) * var_arg__blk1057_dn6)) } } else { (assign32450_e47383 * (assign32450_e47382 * (var_arg__blk1057_dn6 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32450_e47382) as f64).is_finite() && ((assign32450_e47382) as f64).fract() == 0.0 { if assign32450_e47382 == 0.0 { 0.0 } else { (assign32450_e47382 * ((var_arg__blk1057).powf(assign32450_e47382 - 1.0) * var_arg__blk1057_dn7)) } } else { (assign32450_e47383 * (assign32450_e47382 * (var_arg__blk1057_dn7 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32450_e47382) as f64).is_finite() && ((assign32450_e47382) as f64).fract() == 0.0 { if assign32450_e47382 == 0.0 { 0.0 } else { (assign32450_e47382 * ((var_arg__blk1057).powf(assign32450_e47382 - 1.0) * var_arg__blk1057_dn12)) } } else { (assign32450_e47383 * (assign32450_e47382 * (var_arg__blk1057_dn12 / var_arg__blk1057))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32450_e47385;
        var_sarg_dn6 = assign32450_e47385_d_n6;
        var_sarg_dn7 = assign32450_e47385_d_n7;
        var_sarg_dn12 = assign32450_e47385_d_n12;

        let (assign32460_e47409, assign32460_e47409_d_n0, assign32460_e47409_d_n2, assign32460_e47409_d_n6, assign32460_e47409_d_n7, assign32460_e47409_d_n10, assign32460_e47409_d_n11, assign32460_e47409_d_n12, assign32460_e47409_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1066 != 0.0)) {
        let assign32460_e47396: f64 = (p.p186 * var_czbssw);
        let assign32460_e47400: f64 = (var_arg__blk1057 * var_sarg);
        let assign32460_e47401: f64 = (1.0 - assign32460_e47400);
        let assign32460_e47402: f64 = (assign32460_e47396 * assign32460_e47401);
        let assign32460_e47405: f64 = (1.0 - p.p183);
        let assign32460_e47406: f64 = (assign32460_e47402 / assign32460_e47405);
        let assign32460_e47407: f64 = (var_qbs + assign32460_e47406);
        (assign32460_e47407, var_qbs_dn0, var_qbs_dn2, (var_qbs_dn6 + ((assign32460_e47396 * (-((var_arg__blk1057_dn6 * var_sarg) + (var_arg__blk1057 * var_sarg_dn6)))) / assign32460_e47405)), (var_qbs_dn7 + ((assign32460_e47396 * (-((var_arg__blk1057_dn7 * var_sarg) + (var_arg__blk1057 * var_sarg_dn7)))) / assign32460_e47405)), var_qbs_dn10, var_qbs_dn11, (var_qbs_dn12 + ((assign32460_e47396 * (-((var_arg__blk1057_dn12 * var_sarg) + (var_arg__blk1057 * var_sarg_dn12)))) / assign32460_e47405)), var_qbs_dn17,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32460_e47409;
        var_qbs_dn0 = assign32460_e47409_d_n0;
        var_qbs_dn2 = assign32460_e47409_d_n2;
        var_qbs_dn6 = assign32460_e47409_d_n6;
        var_qbs_dn7 = assign32460_e47409_d_n7;
        var_qbs_dn10 = assign32460_e47409_d_n10;
        var_qbs_dn11 = assign32460_e47409_d_n11;
        var_qbs_dn12 = assign32460_e47409_d_n12;
        var_qbs_dn17 = assign32460_e47409_d_n17;

        let assign32470_e47412: f64 = if var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        var_guard1068 = assign32470_e47412;

        let (assign32480_e47426, assign32480_e47426_d_n6, assign32480_e47426_d_n7, assign32480_e47426_d_n12,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1068 != 0.0)) {
        let assign32480_e47423: f64 = (var_vbsj / p.p187);
        let assign32480_e47424: f64 = (1.0 - assign32480_e47423);
        (assign32480_e47424, 0.0, (-(var_vbsj_dn7 / p.p187)), (-(var_vbsj_dn12 / p.p187)),)
    } else {
        (var_arg__blk1057, var_arg__blk1057_dn6, var_arg__blk1057_dn7, var_arg__blk1057_dn12,)
    }
};
        var_arg__blk1057 = assign32480_e47426;
        var_arg__blk1057_dn6 = assign32480_e47426_d_n6;
        var_arg__blk1057_dn7 = assign32480_e47426_d_n7;
        var_arg__blk1057_dn12 = assign32480_e47426_d_n12;

        let assign32490_e47429: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        var_guard1069 = assign32490_e47429;

        let (assign32500_e47444, assign32500_e47444_d_n6, assign32500_e47444_d_n7, assign32500_e47444_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1068 != 0.0)) && (var_guard1069 != 0.0)) {
        let assign32500_e47441: f64 = (var_arg__blk1057).sqrt();
        let assign32500_e47442: f64 = (1.0 / assign32500_e47441);
        (assign32500_e47442, (-((var_arg__blk1057_dn6 / (2.0 * assign32500_e47441)) / (assign32500_e47441 * assign32500_e47441))), (-((var_arg__blk1057_dn7 / (2.0 * assign32500_e47441)) / (assign32500_e47441 * assign32500_e47441))), (-((var_arg__blk1057_dn12 / (2.0 * assign32500_e47441)) / (assign32500_e47441 * assign32500_e47441))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32500_e47444;
        var_sarg_dn6 = assign32500_e47444_d_n6;
        var_sarg_dn7 = assign32500_e47444_d_n7;
        var_sarg_dn12 = assign32500_e47444_d_n12;

        *var_arg__blk1057_slot = var_arg__blk1057;
        *var_arg__blk1057_dn12_slot = var_arg__blk1057_dn12;
        *var_arg__blk1057_dn6_slot = var_arg__blk1057_dn6;
        *var_arg__blk1057_dn7_slot = var_arg__blk1057_dn7;
        *var_czbd_slot = var_czbd;
        *var_czbs_slot = var_czbs;
        *var_czbssw_slot = var_czbssw;
        *var_czbsswg_slot = var_czbsswg;
        *var_guard1060_slot = var_guard1060;
        *var_guard1061_slot = var_guard1061;
        *var_guard1062_slot = var_guard1062;
        *var_guard1063_slot = var_guard1063;
        *var_guard1064_slot = var_guard1064;
        *var_guard1065_slot = var_guard1065;
        *var_guard1066_slot = var_guard1066;
        *var_guard1067_slot = var_guard1067;
        *var_guard1068_slot = var_guard1068;
        *var_guard1069_slot = var_guard1069;
        *var_ibd_slot = var_ibd;
        *var_ibd_dn0_slot = var_ibd_dn0;
        *var_ibd_dn10_slot = var_ibd_dn10;
        *var_ibd_dn11_slot = var_ibd_dn11;
        *var_ibd_dn12_slot = var_ibd_dn12;
        *var_ibd_dn17_slot = var_ibd_dn17;
        *var_ibd_dn2_slot = var_ibd_dn2;
        *var_ibd_dn6_slot = var_ibd_dn6;
        *var_ibd_dn7_slot = var_ibd_dn7;
        *var_ibs_slot = var_ibs;
        *var_ibs_dn0_slot = var_ibs_dn0;
        *var_ibs_dn10_slot = var_ibs_dn10;
        *var_ibs_dn11_slot = var_ibs_dn11;
        *var_ibs_dn12_slot = var_ibs_dn12;
        *var_ibs_dn17_slot = var_ibs_dn17;
        *var_ibs_dn2_slot = var_ibs_dn2;
        *var_ibs_dn6_slot = var_ibs_dn6;
        *var_ibs_dn7_slot = var_ibs_dn7;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn0_slot = var_qbs_dn0;
        *var_qbs_dn10_slot = var_qbs_dn10;
        *var_qbs_dn11_slot = var_qbs_dn11;
        *var_qbs_dn12_slot = var_qbs_dn12;
        *var_qbs_dn17_slot = var_qbs_dn17;
        *var_qbs_dn2_slot = var_qbs_dn2;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_sarg_slot = var_sarg;
        *var_sarg_dn12_slot = var_sarg_dn12;
        *var_sarg_dn6_slot = var_sarg_dn6;
        *var_sarg_dn7_slot = var_sarg_dn7;
        *var_t1__blk1032_slot = var_t1__blk1032;
        *var_t1__blk1032_dn10_slot = var_t1__blk1032_dn10;
        *var_t1__blk1032_dn12_slot = var_t1__blk1032_dn12;
        *var_t1__blk1032_dn6_slot = var_t1__blk1032_dn6;
        *var_t1__blk1032_dn7_slot = var_t1__blk1032_dn7;
        *var_xp_max_slot = var_xp_max;
    }

    pub(super) fn stamp_transient_block_115(
        p: &Parameters,
        var_czbd: f64,
        var_czbs: f64,
        var_czbssw: f64,
        var_guard1030: f64,
        var_guard1062: f64,
        var_guard1063: f64,
        var_guard1068: f64,
        var_guard1069: f64,
        var_vbdj: f64,
        var_vbdj_dn12: f64,
        var_vbdj_dn6: f64,
        var_vbsj: f64,
        var_vbsj_dn12: f64,
        var_vbsj_dn7: f64,
        var_w_diodcv: f64,
        var_arg__blk1057_slot: &mut f64,
        var_arg__blk1057_dn12_slot: &mut f64,
        var_arg__blk1057_dn6_slot: &mut f64,
        var_arg__blk1057_dn7_slot: &mut f64,
        var_czbdsw_slot: &mut f64,
        var_czbdswg_slot: &mut f64,
        var_czbsswg_slot: &mut f64,
        var_guard1070_slot: &mut f64,
        var_guard1071_slot: &mut f64,
        var_guard1072_slot: &mut f64,
        var_guard1073_slot: &mut f64,
        var_guard1074_slot: &mut f64,
        var_guard1075_slot: &mut f64,
        var_guard1076_slot: &mut f64,
        var_guard1077_slot: &mut f64,
        var_guard1078_slot: &mut f64,
        var_guard1079_slot: &mut f64,
        var_guard1080_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn0_slot: &mut f64,
        var_qbd_dn10_slot: &mut f64,
        var_qbd_dn11_slot: &mut f64,
        var_qbd_dn12_slot: &mut f64,
        var_qbd_dn17_slot: &mut f64,
        var_qbd_dn2_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn0_slot: &mut f64,
        var_qbs_dn10_slot: &mut f64,
        var_qbs_dn11_slot: &mut f64,
        var_qbs_dn12_slot: &mut f64,
        var_qbs_dn17_slot: &mut f64,
        var_qbs_dn2_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_sarg_slot: &mut f64,
        var_sarg_dn12_slot: &mut f64,
        var_sarg_dn6_slot: &mut f64,
        var_sarg_dn7_slot: &mut f64,
        var_t1__blk1032_slot: &mut f64,
        var_t1__blk1032_dn10_slot: &mut f64,
        var_t1__blk1032_dn12_slot: &mut f64,
        var_t1__blk1032_dn6_slot: &mut f64,
        var_t1__blk1032_dn7_slot: &mut f64,
        var_t2__blk1033_slot: &mut f64,
        var_t2__blk1033_dn0_slot: &mut f64,
        var_t2__blk1033_dn10_slot: &mut f64,
        var_t2__blk1033_dn11_slot: &mut f64,
        var_t2__blk1033_dn12_slot: &mut f64,
        var_t2__blk1033_dn17_slot: &mut f64,
        var_t2__blk1033_dn2_slot: &mut f64,
        var_t2__blk1033_dn6_slot: &mut f64,
        var_t2__blk1033_dn7_slot: &mut f64,
    ) {
        let mut var_arg__blk1057: f64 = *var_arg__blk1057_slot;
        let mut var_arg__blk1057_dn12: f64 = *var_arg__blk1057_dn12_slot;
        let mut var_arg__blk1057_dn6: f64 = *var_arg__blk1057_dn6_slot;
        let mut var_arg__blk1057_dn7: f64 = *var_arg__blk1057_dn7_slot;
        let mut var_czbdsw: f64 = *var_czbdsw_slot;
        let mut var_czbdswg: f64 = *var_czbdswg_slot;
        let mut var_czbsswg: f64 = *var_czbsswg_slot;
        let mut var_guard1070: f64 = *var_guard1070_slot;
        let mut var_guard1071: f64 = *var_guard1071_slot;
        let mut var_guard1072: f64 = *var_guard1072_slot;
        let mut var_guard1073: f64 = *var_guard1073_slot;
        let mut var_guard1074: f64 = *var_guard1074_slot;
        let mut var_guard1075: f64 = *var_guard1075_slot;
        let mut var_guard1076: f64 = *var_guard1076_slot;
        let mut var_guard1077: f64 = *var_guard1077_slot;
        let mut var_guard1078: f64 = *var_guard1078_slot;
        let mut var_guard1079: f64 = *var_guard1079_slot;
        let mut var_guard1080: f64 = *var_guard1080_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn0: f64 = *var_qbd_dn0_slot;
        let mut var_qbd_dn10: f64 = *var_qbd_dn10_slot;
        let mut var_qbd_dn11: f64 = *var_qbd_dn11_slot;
        let mut var_qbd_dn12: f64 = *var_qbd_dn12_slot;
        let mut var_qbd_dn17: f64 = *var_qbd_dn17_slot;
        let mut var_qbd_dn2: f64 = *var_qbd_dn2_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn0: f64 = *var_qbs_dn0_slot;
        let mut var_qbs_dn10: f64 = *var_qbs_dn10_slot;
        let mut var_qbs_dn11: f64 = *var_qbs_dn11_slot;
        let mut var_qbs_dn12: f64 = *var_qbs_dn12_slot;
        let mut var_qbs_dn17: f64 = *var_qbs_dn17_slot;
        let mut var_qbs_dn2: f64 = *var_qbs_dn2_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_sarg: f64 = *var_sarg_slot;
        let mut var_sarg_dn12: f64 = *var_sarg_dn12_slot;
        let mut var_sarg_dn6: f64 = *var_sarg_dn6_slot;
        let mut var_sarg_dn7: f64 = *var_sarg_dn7_slot;
        let mut var_t1__blk1032: f64 = *var_t1__blk1032_slot;
        let mut var_t1__blk1032_dn10: f64 = *var_t1__blk1032_dn10_slot;
        let mut var_t1__blk1032_dn12: f64 = *var_t1__blk1032_dn12_slot;
        let mut var_t1__blk1032_dn6: f64 = *var_t1__blk1032_dn6_slot;
        let mut var_t1__blk1032_dn7: f64 = *var_t1__blk1032_dn7_slot;
        let mut var_t2__blk1033: f64 = *var_t2__blk1033_slot;
        let mut var_t2__blk1033_dn0: f64 = *var_t2__blk1033_dn0_slot;
        let mut var_t2__blk1033_dn10: f64 = *var_t2__blk1033_dn10_slot;
        let mut var_t2__blk1033_dn11: f64 = *var_t2__blk1033_dn11_slot;
        let mut var_t2__blk1033_dn12: f64 = *var_t2__blk1033_dn12_slot;
        let mut var_t2__blk1033_dn17: f64 = *var_t2__blk1033_dn17_slot;
        let mut var_t2__blk1033_dn2: f64 = *var_t2__blk1033_dn2_slot;
        let mut var_t2__blk1033_dn6: f64 = *var_t2__blk1033_dn6_slot;
        let mut var_t2__blk1033_dn7: f64 = *var_t2__blk1033_dn7_slot;

        let (assign32510_e47460, assign32510_e47460_d_n6, assign32510_e47460_d_n7, assign32510_e47460_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1068 != 0.0)) && (var_guard1069 == 0.0)) {
        let assign32510_e47457: f64 = (-p.p184);
        let assign32510_e47458: f64 = (var_arg__blk1057).powf(assign32510_e47457);
        (assign32510_e47458, if 0.0 == 0.0 && ((assign32510_e47457) as f64).is_finite() && ((assign32510_e47457) as f64).fract() == 0.0 { if assign32510_e47457 == 0.0 { 0.0 } else { (assign32510_e47457 * ((var_arg__blk1057).powf(assign32510_e47457 - 1.0) * var_arg__blk1057_dn6)) } } else { (assign32510_e47458 * (assign32510_e47457 * (var_arg__blk1057_dn6 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32510_e47457) as f64).is_finite() && ((assign32510_e47457) as f64).fract() == 0.0 { if assign32510_e47457 == 0.0 { 0.0 } else { (assign32510_e47457 * ((var_arg__blk1057).powf(assign32510_e47457 - 1.0) * var_arg__blk1057_dn7)) } } else { (assign32510_e47458 * (assign32510_e47457 * (var_arg__blk1057_dn7 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32510_e47457) as f64).is_finite() && ((assign32510_e47457) as f64).fract() == 0.0 { if assign32510_e47457 == 0.0 { 0.0 } else { (assign32510_e47457 * ((var_arg__blk1057).powf(assign32510_e47457 - 1.0) * var_arg__blk1057_dn12)) } } else { (assign32510_e47458 * (assign32510_e47457 * (var_arg__blk1057_dn12 / var_arg__blk1057))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32510_e47460;
        var_sarg_dn6 = assign32510_e47460_d_n6;
        var_sarg_dn7 = assign32510_e47460_d_n7;
        var_sarg_dn12 = assign32510_e47460_d_n12;

        let (assign32520_e47484, assign32520_e47484_d_n0, assign32520_e47484_d_n2, assign32520_e47484_d_n6, assign32520_e47484_d_n7, assign32520_e47484_d_n10, assign32520_e47484_d_n11, assign32520_e47484_d_n12, assign32520_e47484_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 != 0.0)) && (var_guard1068 != 0.0)) {
        let assign32520_e47471: f64 = (p.p187 * var_czbsswg);
        let assign32520_e47475: f64 = (var_arg__blk1057 * var_sarg);
        let assign32520_e47476: f64 = (1.0 - assign32520_e47475);
        let assign32520_e47477: f64 = (assign32520_e47471 * assign32520_e47476);
        let assign32520_e47480: f64 = (1.0 - p.p184);
        let assign32520_e47481: f64 = (assign32520_e47477 / assign32520_e47480);
        let assign32520_e47482: f64 = (var_qbs + assign32520_e47481);
        (assign32520_e47482, var_qbs_dn0, var_qbs_dn2, (var_qbs_dn6 + ((assign32520_e47471 * (-((var_arg__blk1057_dn6 * var_sarg) + (var_arg__blk1057 * var_sarg_dn6)))) / assign32520_e47480)), (var_qbs_dn7 + ((assign32520_e47471 * (-((var_arg__blk1057_dn7 * var_sarg) + (var_arg__blk1057 * var_sarg_dn7)))) / assign32520_e47480)), var_qbs_dn10, var_qbs_dn11, (var_qbs_dn12 + ((assign32520_e47471 * (-((var_arg__blk1057_dn12 * var_sarg) + (var_arg__blk1057 * var_sarg_dn12)))) / assign32520_e47480)), var_qbs_dn17,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32520_e47484;
        var_qbs_dn0 = assign32520_e47484_d_n0;
        var_qbs_dn2 = assign32520_e47484_d_n2;
        var_qbs_dn6 = assign32520_e47484_d_n6;
        var_qbs_dn7 = assign32520_e47484_d_n7;
        var_qbs_dn10 = assign32520_e47484_d_n10;
        var_qbs_dn11 = assign32520_e47484_d_n11;
        var_qbs_dn12 = assign32520_e47484_d_n12;
        var_qbs_dn17 = assign32520_e47484_d_n17;

        let (assign32530_e47497, assign32530_e47497_d_n6, assign32530_e47497_d_n7, assign32530_e47497_d_n10, assign32530_e47497_d_n12,) = {
    if (((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 == 0.0)) {
        let assign32530_e47493: f64 = (var_czbs + var_czbssw);
        let assign32530_e47495: f64 = (assign32530_e47493 + var_czbsswg);
        (assign32530_e47495, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk1032, var_t1__blk1032_dn6, var_t1__blk1032_dn7, var_t1__blk1032_dn10, var_t1__blk1032_dn12,)
    }
};
        var_t1__blk1032 = assign32530_e47497;
        var_t1__blk1032_dn6 = assign32530_e47497_d_n6;
        var_t1__blk1032_dn7 = assign32530_e47497_d_n7;
        var_t1__blk1032_dn10 = assign32530_e47497_d_n10;
        var_t1__blk1032_dn12 = assign32530_e47497_d_n12;

        let (assign32540_e47522, assign32540_e47522_d_n0, assign32540_e47522_d_n2, assign32540_e47522_d_n6, assign32540_e47522_d_n7, assign32540_e47522_d_n10, assign32540_e47522_d_n11, assign32540_e47522_d_n12, assign32540_e47522_d_n17,) = {
    if (((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 == 0.0)) {
        let assign32540_e47506: f64 = (var_czbs * p.p182);
        let assign32540_e47508: f64 = (assign32540_e47506 / p.p185);
        let assign32540_e47511: f64 = (var_czbssw * p.p183);
        let assign32540_e47513: f64 = (assign32540_e47511 / p.p186);
        let assign32540_e47514: f64 = (assign32540_e47508 + assign32540_e47513);
        let assign32540_e47517: f64 = (var_czbsswg * p.p184);
        let assign32540_e47519: f64 = (assign32540_e47517 / p.p187);
        let assign32540_e47520: f64 = (assign32540_e47514 + assign32540_e47519);
        (assign32540_e47520, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2__blk1033, var_t2__blk1033_dn0, var_t2__blk1033_dn2, var_t2__blk1033_dn6, var_t2__blk1033_dn7, var_t2__blk1033_dn10, var_t2__blk1033_dn11, var_t2__blk1033_dn12, var_t2__blk1033_dn17,)
    }
};
        var_t2__blk1033 = assign32540_e47522;
        var_t2__blk1033_dn0 = assign32540_e47522_d_n0;
        var_t2__blk1033_dn2 = assign32540_e47522_d_n2;
        var_t2__blk1033_dn6 = assign32540_e47522_d_n6;
        var_t2__blk1033_dn7 = assign32540_e47522_d_n7;
        var_t2__blk1033_dn10 = assign32540_e47522_d_n10;
        var_t2__blk1033_dn11 = assign32540_e47522_d_n11;
        var_t2__blk1033_dn12 = assign32540_e47522_d_n12;
        var_t2__blk1033_dn17 = assign32540_e47522_d_n17;

        let (assign32550_e47539, assign32550_e47539_d_n0, assign32550_e47539_d_n2, assign32550_e47539_d_n6, assign32550_e47539_d_n7, assign32550_e47539_d_n10, assign32550_e47539_d_n11, assign32550_e47539_d_n12, assign32550_e47539_d_n17,) = {
    if (((var_guard1030 != 0.0) && (var_guard1062 != 0.0)) && (var_guard1063 == 0.0)) {
        let assign32550_e47533: f64 = (var_vbsj * 0.5);
        let assign32550_e47535: f64 = (assign32550_e47533 * var_t2__blk1033);
        let assign32550_e47536: f64 = (var_t1__blk1032 + assign32550_e47535);
        let assign32550_e47537: f64 = (var_vbsj * assign32550_e47536);
        (assign32550_e47537, (var_vbsj * (assign32550_e47533 * var_t2__blk1033_dn0)), (var_vbsj * (assign32550_e47533 * var_t2__blk1033_dn2)), (var_vbsj * (var_t1__blk1032_dn6 + (assign32550_e47533 * var_t2__blk1033_dn6))), ((var_vbsj_dn7 * assign32550_e47536) + (var_vbsj * (var_t1__blk1032_dn7 + (((var_vbsj_dn7 * 0.5) * var_t2__blk1033) + (assign32550_e47533 * var_t2__blk1033_dn7))))), (var_vbsj * (var_t1__blk1032_dn10 + (assign32550_e47533 * var_t2__blk1033_dn10))), (var_vbsj * (assign32550_e47533 * var_t2__blk1033_dn11)), ((var_vbsj_dn12 * assign32550_e47536) + (var_vbsj * (var_t1__blk1032_dn12 + (((var_vbsj_dn12 * 0.5) * var_t2__blk1033) + (assign32550_e47533 * var_t2__blk1033_dn12))))), (var_vbsj * (assign32550_e47533 * var_t2__blk1033_dn17)),)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32550_e47539;
        var_qbs_dn0 = assign32550_e47539_d_n0;
        var_qbs_dn2 = assign32550_e47539_d_n2;
        var_qbs_dn6 = assign32550_e47539_d_n6;
        var_qbs_dn7 = assign32550_e47539_d_n7;
        var_qbs_dn10 = assign32550_e47539_d_n10;
        var_qbs_dn11 = assign32550_e47539_d_n11;
        var_qbs_dn12 = assign32550_e47539_d_n12;
        var_qbs_dn17 = assign32550_e47539_d_n17;

        let (assign32560_e47548,) = {
    if ((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) {
        let assign32560_e47546: f64 = (p.p181 * p.p5);
        (assign32560_e47546,)
    } else {
        (var_czbsswg,)
    }
};
        var_czbsswg = assign32560_e47548;

        let assign32570_e47551: f64 = if var_vbsj < 0.0 { 1.0 } else { 0.0 };
        var_guard1070 = assign32570_e47551;

        let assign32580_e47554: f64 = if var_czbs > 0.0 { 1.0 } else { 0.0 };
        var_guard1071 = assign32580_e47554;

        let (assign32590_e47569, assign32590_e47569_d_n6, assign32590_e47569_d_n7, assign32590_e47569_d_n12,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) && (var_guard1070 != 0.0)) && (var_guard1071 != 0.0)) {
        let assign32590_e47566: f64 = (var_vbsj / p.p185);
        let assign32590_e47567: f64 = (1.0 - assign32590_e47566);
        (assign32590_e47567, 0.0, (-(var_vbsj_dn7 / p.p185)), (-(var_vbsj_dn12 / p.p185)),)
    } else {
        (var_arg__blk1057, var_arg__blk1057_dn6, var_arg__blk1057_dn7, var_arg__blk1057_dn12,)
    }
};
        var_arg__blk1057 = assign32590_e47569;
        var_arg__blk1057_dn6 = assign32590_e47569_d_n6;
        var_arg__blk1057_dn7 = assign32590_e47569_d_n7;
        var_arg__blk1057_dn12 = assign32590_e47569_d_n12;

        let assign32600_e47572: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        var_guard1072 = assign32600_e47572;

        let (assign32610_e47588, assign32610_e47588_d_n6, assign32610_e47588_d_n7, assign32610_e47588_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) && (var_guard1070 != 0.0)) && (var_guard1071 != 0.0)) && (var_guard1072 != 0.0)) {
        let assign32610_e47585: f64 = (var_arg__blk1057).sqrt();
        let assign32610_e47586: f64 = (1.0 / assign32610_e47585);
        (assign32610_e47586, (-((var_arg__blk1057_dn6 / (2.0 * assign32610_e47585)) / (assign32610_e47585 * assign32610_e47585))), (-((var_arg__blk1057_dn7 / (2.0 * assign32610_e47585)) / (assign32610_e47585 * assign32610_e47585))), (-((var_arg__blk1057_dn12 / (2.0 * assign32610_e47585)) / (assign32610_e47585 * assign32610_e47585))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32610_e47588;
        var_sarg_dn6 = assign32610_e47588_d_n6;
        var_sarg_dn7 = assign32610_e47588_d_n7;
        var_sarg_dn12 = assign32610_e47588_d_n12;

        let (assign32620_e47605, assign32620_e47605_d_n6, assign32620_e47605_d_n7, assign32620_e47605_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) && (var_guard1070 != 0.0)) && (var_guard1071 != 0.0)) && (var_guard1072 == 0.0)) {
        let assign32620_e47602: f64 = (-p.p182);
        let assign32620_e47603: f64 = (var_arg__blk1057).powf(assign32620_e47602);
        (assign32620_e47603, if 0.0 == 0.0 && ((assign32620_e47602) as f64).is_finite() && ((assign32620_e47602) as f64).fract() == 0.0 { if assign32620_e47602 == 0.0 { 0.0 } else { (assign32620_e47602 * ((var_arg__blk1057).powf(assign32620_e47602 - 1.0) * var_arg__blk1057_dn6)) } } else { (assign32620_e47603 * (assign32620_e47602 * (var_arg__blk1057_dn6 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32620_e47602) as f64).is_finite() && ((assign32620_e47602) as f64).fract() == 0.0 { if assign32620_e47602 == 0.0 { 0.0 } else { (assign32620_e47602 * ((var_arg__blk1057).powf(assign32620_e47602 - 1.0) * var_arg__blk1057_dn7)) } } else { (assign32620_e47603 * (assign32620_e47602 * (var_arg__blk1057_dn7 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32620_e47602) as f64).is_finite() && ((assign32620_e47602) as f64).fract() == 0.0 { if assign32620_e47602 == 0.0 { 0.0 } else { (assign32620_e47602 * ((var_arg__blk1057).powf(assign32620_e47602 - 1.0) * var_arg__blk1057_dn12)) } } else { (assign32620_e47603 * (assign32620_e47602 * (var_arg__blk1057_dn12 / var_arg__blk1057))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32620_e47605;
        var_sarg_dn6 = assign32620_e47605_d_n6;
        var_sarg_dn7 = assign32620_e47605_d_n7;
        var_sarg_dn12 = assign32620_e47605_d_n12;

        let (assign32630_e47628, assign32630_e47628_d_n0, assign32630_e47628_d_n2, assign32630_e47628_d_n6, assign32630_e47628_d_n7, assign32630_e47628_d_n10, assign32630_e47628_d_n11, assign32630_e47628_d_n12, assign32630_e47628_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) && (var_guard1070 != 0.0)) && (var_guard1071 != 0.0)) {
        let assign32630_e47616: f64 = (p.p185 * var_czbs);
        let assign32630_e47620: f64 = (var_arg__blk1057 * var_sarg);
        let assign32630_e47621: f64 = (1.0 - assign32630_e47620);
        let assign32630_e47622: f64 = (assign32630_e47616 * assign32630_e47621);
        let assign32630_e47625: f64 = (1.0 - p.p182);
        let assign32630_e47626: f64 = (assign32630_e47622 / assign32630_e47625);
        (assign32630_e47626, 0.0, 0.0, ((assign32630_e47616 * (-((var_arg__blk1057_dn6 * var_sarg) + (var_arg__blk1057 * var_sarg_dn6)))) / assign32630_e47625), ((assign32630_e47616 * (-((var_arg__blk1057_dn7 * var_sarg) + (var_arg__blk1057 * var_sarg_dn7)))) / assign32630_e47625), 0.0, 0.0, ((assign32630_e47616 * (-((var_arg__blk1057_dn12 * var_sarg) + (var_arg__blk1057 * var_sarg_dn12)))) / assign32630_e47625), 0.0,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32630_e47628;
        var_qbs_dn0 = assign32630_e47628_d_n0;
        var_qbs_dn2 = assign32630_e47628_d_n2;
        var_qbs_dn6 = assign32630_e47628_d_n6;
        var_qbs_dn7 = assign32630_e47628_d_n7;
        var_qbs_dn10 = assign32630_e47628_d_n10;
        var_qbs_dn11 = assign32630_e47628_d_n11;
        var_qbs_dn12 = assign32630_e47628_d_n12;
        var_qbs_dn17 = assign32630_e47628_d_n17;

        let (assign32640_e47640, assign32640_e47640_d_n0, assign32640_e47640_d_n2, assign32640_e47640_d_n6, assign32640_e47640_d_n7, assign32640_e47640_d_n10, assign32640_e47640_d_n11, assign32640_e47640_d_n12, assign32640_e47640_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) && (var_guard1070 != 0.0)) && (var_guard1071 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32640_e47640;
        var_qbs_dn0 = assign32640_e47640_d_n0;
        var_qbs_dn2 = assign32640_e47640_d_n2;
        var_qbs_dn6 = assign32640_e47640_d_n6;
        var_qbs_dn7 = assign32640_e47640_d_n7;
        var_qbs_dn10 = assign32640_e47640_d_n10;
        var_qbs_dn11 = assign32640_e47640_d_n11;
        var_qbs_dn12 = assign32640_e47640_d_n12;
        var_qbs_dn17 = assign32640_e47640_d_n17;

        let assign32650_e47643: f64 = if var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        var_guard1073 = assign32650_e47643;

        let (assign32660_e47658, assign32660_e47658_d_n6, assign32660_e47658_d_n7, assign32660_e47658_d_n12,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) && (var_guard1070 != 0.0)) && (var_guard1073 != 0.0)) {
        let assign32660_e47655: f64 = (var_vbsj / p.p187);
        let assign32660_e47656: f64 = (1.0 - assign32660_e47655);
        (assign32660_e47656, 0.0, (-(var_vbsj_dn7 / p.p187)), (-(var_vbsj_dn12 / p.p187)),)
    } else {
        (var_arg__blk1057, var_arg__blk1057_dn6, var_arg__blk1057_dn7, var_arg__blk1057_dn12,)
    }
};
        var_arg__blk1057 = assign32660_e47658;
        var_arg__blk1057_dn6 = assign32660_e47658_d_n6;
        var_arg__blk1057_dn7 = assign32660_e47658_d_n7;
        var_arg__blk1057_dn12 = assign32660_e47658_d_n12;

        let assign32670_e47661: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        var_guard1074 = assign32670_e47661;

        let (assign32680_e47677, assign32680_e47677_d_n6, assign32680_e47677_d_n7, assign32680_e47677_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) && (var_guard1070 != 0.0)) && (var_guard1073 != 0.0)) && (var_guard1074 != 0.0)) {
        let assign32680_e47674: f64 = (var_arg__blk1057).sqrt();
        let assign32680_e47675: f64 = (1.0 / assign32680_e47674);
        (assign32680_e47675, (-((var_arg__blk1057_dn6 / (2.0 * assign32680_e47674)) / (assign32680_e47674 * assign32680_e47674))), (-((var_arg__blk1057_dn7 / (2.0 * assign32680_e47674)) / (assign32680_e47674 * assign32680_e47674))), (-((var_arg__blk1057_dn12 / (2.0 * assign32680_e47674)) / (assign32680_e47674 * assign32680_e47674))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32680_e47677;
        var_sarg_dn6 = assign32680_e47677_d_n6;
        var_sarg_dn7 = assign32680_e47677_d_n7;
        var_sarg_dn12 = assign32680_e47677_d_n12;

        let (assign32690_e47694, assign32690_e47694_d_n6, assign32690_e47694_d_n7, assign32690_e47694_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) && (var_guard1070 != 0.0)) && (var_guard1073 != 0.0)) && (var_guard1074 == 0.0)) {
        let assign32690_e47691: f64 = (-p.p184);
        let assign32690_e47692: f64 = (var_arg__blk1057).powf(assign32690_e47691);
        (assign32690_e47692, if 0.0 == 0.0 && ((assign32690_e47691) as f64).is_finite() && ((assign32690_e47691) as f64).fract() == 0.0 { if assign32690_e47691 == 0.0 { 0.0 } else { (assign32690_e47691 * ((var_arg__blk1057).powf(assign32690_e47691 - 1.0) * var_arg__blk1057_dn6)) } } else { (assign32690_e47692 * (assign32690_e47691 * (var_arg__blk1057_dn6 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32690_e47691) as f64).is_finite() && ((assign32690_e47691) as f64).fract() == 0.0 { if assign32690_e47691 == 0.0 { 0.0 } else { (assign32690_e47691 * ((var_arg__blk1057).powf(assign32690_e47691 - 1.0) * var_arg__blk1057_dn7)) } } else { (assign32690_e47692 * (assign32690_e47691 * (var_arg__blk1057_dn7 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32690_e47691) as f64).is_finite() && ((assign32690_e47691) as f64).fract() == 0.0 { if assign32690_e47691 == 0.0 { 0.0 } else { (assign32690_e47691 * ((var_arg__blk1057).powf(assign32690_e47691 - 1.0) * var_arg__blk1057_dn12)) } } else { (assign32690_e47692 * (assign32690_e47691 * (var_arg__blk1057_dn12 / var_arg__blk1057))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32690_e47694;
        var_sarg_dn6 = assign32690_e47694_d_n6;
        var_sarg_dn7 = assign32690_e47694_d_n7;
        var_sarg_dn12 = assign32690_e47694_d_n12;

        let (assign32700_e47719, assign32700_e47719_d_n0, assign32700_e47719_d_n2, assign32700_e47719_d_n6, assign32700_e47719_d_n7, assign32700_e47719_d_n10, assign32700_e47719_d_n11, assign32700_e47719_d_n12, assign32700_e47719_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) && (var_guard1070 != 0.0)) && (var_guard1073 != 0.0)) {
        let assign32700_e47706: f64 = (p.p187 * var_czbsswg);
        let assign32700_e47710: f64 = (var_arg__blk1057 * var_sarg);
        let assign32700_e47711: f64 = (1.0 - assign32700_e47710);
        let assign32700_e47712: f64 = (assign32700_e47706 * assign32700_e47711);
        let assign32700_e47715: f64 = (1.0 - p.p184);
        let assign32700_e47716: f64 = (assign32700_e47712 / assign32700_e47715);
        let assign32700_e47717: f64 = (var_qbs + assign32700_e47716);
        (assign32700_e47717, var_qbs_dn0, var_qbs_dn2, (var_qbs_dn6 + ((assign32700_e47706 * (-((var_arg__blk1057_dn6 * var_sarg) + (var_arg__blk1057 * var_sarg_dn6)))) / assign32700_e47715)), (var_qbs_dn7 + ((assign32700_e47706 * (-((var_arg__blk1057_dn7 * var_sarg) + (var_arg__blk1057 * var_sarg_dn7)))) / assign32700_e47715)), var_qbs_dn10, var_qbs_dn11, (var_qbs_dn12 + ((assign32700_e47706 * (-((var_arg__blk1057_dn12 * var_sarg) + (var_arg__blk1057 * var_sarg_dn12)))) / assign32700_e47715)), var_qbs_dn17,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32700_e47719;
        var_qbs_dn0 = assign32700_e47719_d_n0;
        var_qbs_dn2 = assign32700_e47719_d_n2;
        var_qbs_dn6 = assign32700_e47719_d_n6;
        var_qbs_dn7 = assign32700_e47719_d_n7;
        var_qbs_dn10 = assign32700_e47719_d_n10;
        var_qbs_dn11 = assign32700_e47719_d_n11;
        var_qbs_dn12 = assign32700_e47719_d_n12;
        var_qbs_dn17 = assign32700_e47719_d_n17;

        let (assign32710_e47731, assign32710_e47731_d_n6, assign32710_e47731_d_n7, assign32710_e47731_d_n10, assign32710_e47731_d_n12,) = {
    if (((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) && (var_guard1070 == 0.0)) {
        let assign32710_e47729: f64 = (var_czbs + var_czbsswg);
        (assign32710_e47729, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk1032, var_t1__blk1032_dn6, var_t1__blk1032_dn7, var_t1__blk1032_dn10, var_t1__blk1032_dn12,)
    }
};
        var_t1__blk1032 = assign32710_e47731;
        var_t1__blk1032_dn6 = assign32710_e47731_d_n6;
        var_t1__blk1032_dn7 = assign32710_e47731_d_n7;
        var_t1__blk1032_dn10 = assign32710_e47731_d_n10;
        var_t1__blk1032_dn12 = assign32710_e47731_d_n12;

        let (assign32720_e47751, assign32720_e47751_d_n0, assign32720_e47751_d_n2, assign32720_e47751_d_n6, assign32720_e47751_d_n7, assign32720_e47751_d_n10, assign32720_e47751_d_n11, assign32720_e47751_d_n12, assign32720_e47751_d_n17,) = {
    if (((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) && (var_guard1070 == 0.0)) {
        let assign32720_e47741: f64 = (var_czbs * p.p182);
        let assign32720_e47743: f64 = (assign32720_e47741 / p.p185);
        let assign32720_e47746: f64 = (var_czbsswg * p.p184);
        let assign32720_e47748: f64 = (assign32720_e47746 / p.p187);
        let assign32720_e47749: f64 = (assign32720_e47743 + assign32720_e47748);
        (assign32720_e47749, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2__blk1033, var_t2__blk1033_dn0, var_t2__blk1033_dn2, var_t2__blk1033_dn6, var_t2__blk1033_dn7, var_t2__blk1033_dn10, var_t2__blk1033_dn11, var_t2__blk1033_dn12, var_t2__blk1033_dn17,)
    }
};
        var_t2__blk1033 = assign32720_e47751;
        var_t2__blk1033_dn0 = assign32720_e47751_d_n0;
        var_t2__blk1033_dn2 = assign32720_e47751_d_n2;
        var_t2__blk1033_dn6 = assign32720_e47751_d_n6;
        var_t2__blk1033_dn7 = assign32720_e47751_d_n7;
        var_t2__blk1033_dn10 = assign32720_e47751_d_n10;
        var_t2__blk1033_dn11 = assign32720_e47751_d_n11;
        var_t2__blk1033_dn12 = assign32720_e47751_d_n12;
        var_t2__blk1033_dn17 = assign32720_e47751_d_n17;

        let (assign32730_e47769, assign32730_e47769_d_n0, assign32730_e47769_d_n2, assign32730_e47769_d_n6, assign32730_e47769_d_n7, assign32730_e47769_d_n10, assign32730_e47769_d_n11, assign32730_e47769_d_n12, assign32730_e47769_d_n17,) = {
    if (((var_guard1030 != 0.0) && (var_guard1062 == 0.0)) && (var_guard1070 == 0.0)) {
        let assign32730_e47763: f64 = (var_vbsj * 0.5);
        let assign32730_e47765: f64 = (assign32730_e47763 * var_t2__blk1033);
        let assign32730_e47766: f64 = (var_t1__blk1032 + assign32730_e47765);
        let assign32730_e47767: f64 = (var_vbsj * assign32730_e47766);
        (assign32730_e47767, (var_vbsj * (assign32730_e47763 * var_t2__blk1033_dn0)), (var_vbsj * (assign32730_e47763 * var_t2__blk1033_dn2)), (var_vbsj * (var_t1__blk1032_dn6 + (assign32730_e47763 * var_t2__blk1033_dn6))), ((var_vbsj_dn7 * assign32730_e47766) + (var_vbsj * (var_t1__blk1032_dn7 + (((var_vbsj_dn7 * 0.5) * var_t2__blk1033) + (assign32730_e47763 * var_t2__blk1033_dn7))))), (var_vbsj * (var_t1__blk1032_dn10 + (assign32730_e47763 * var_t2__blk1033_dn10))), (var_vbsj * (assign32730_e47763 * var_t2__blk1033_dn11)), ((var_vbsj_dn12 * assign32730_e47766) + (var_vbsj * (var_t1__blk1032_dn12 + (((var_vbsj_dn12 * 0.5) * var_t2__blk1033) + (assign32730_e47763 * var_t2__blk1033_dn12))))), (var_vbsj * (assign32730_e47763 * var_t2__blk1033_dn17)),)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32730_e47769;
        var_qbs_dn0 = assign32730_e47769_d_n0;
        var_qbs_dn2 = assign32730_e47769_d_n2;
        var_qbs_dn6 = assign32730_e47769_d_n6;
        var_qbs_dn7 = assign32730_e47769_d_n7;
        var_qbs_dn10 = assign32730_e47769_d_n10;
        var_qbs_dn11 = assign32730_e47769_d_n11;
        var_qbs_dn12 = assign32730_e47769_d_n12;
        var_qbs_dn17 = assign32730_e47769_d_n17;

        let assign32740_e47772: f64 = if p.p4 > var_w_diodcv { 1.0 } else { 0.0 };
        var_guard1075 = assign32740_e47772;

        let (assign32750_e47782,) = {
    if ((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) {
        let assign32750_e47779: f64 = (p.p4 - var_w_diodcv);
        let assign32750_e47780: f64 = (p.p180 * assign32750_e47779);
        (assign32750_e47780,)
    } else {
        (var_czbdsw,)
    }
};
        var_czbdsw = assign32750_e47782;

        let (assign32760_e47790,) = {
    if ((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) {
        let assign32760_e47788: f64 = (p.p181 * var_w_diodcv);
        (assign32760_e47788,)
    } else {
        (var_czbdswg,)
    }
};
        var_czbdswg = assign32760_e47790;

        let assign32770_e47793: f64 = if var_vbdj < 0.0 { 1.0 } else { 0.0 };
        var_guard1076 = assign32770_e47793;

        let assign32780_e47796: f64 = if var_czbd > 0.0 { 1.0 } else { 0.0 };
        var_guard1077 = assign32780_e47796;

        let (assign32790_e47810, assign32790_e47810_d_n6, assign32790_e47810_d_n7, assign32790_e47810_d_n12,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1077 != 0.0)) {
        let assign32790_e47807: f64 = (var_vbdj / p.p185);
        let assign32790_e47808: f64 = (1.0 - assign32790_e47807);
        (assign32790_e47808, (-(var_vbdj_dn6 / p.p185)), 0.0, (-(var_vbdj_dn12 / p.p185)),)
    } else {
        (var_arg__blk1057, var_arg__blk1057_dn6, var_arg__blk1057_dn7, var_arg__blk1057_dn12,)
    }
};
        var_arg__blk1057 = assign32790_e47810;
        var_arg__blk1057_dn6 = assign32790_e47810_d_n6;
        var_arg__blk1057_dn7 = assign32790_e47810_d_n7;
        var_arg__blk1057_dn12 = assign32790_e47810_d_n12;

        let assign32800_e47813: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        var_guard1078 = assign32800_e47813;

        let (assign32810_e47828, assign32810_e47828_d_n6, assign32810_e47828_d_n7, assign32810_e47828_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) {
        let assign32810_e47825: f64 = (var_arg__blk1057).sqrt();
        let assign32810_e47826: f64 = (1.0 / assign32810_e47825);
        (assign32810_e47826, (-((var_arg__blk1057_dn6 / (2.0 * assign32810_e47825)) / (assign32810_e47825 * assign32810_e47825))), (-((var_arg__blk1057_dn7 / (2.0 * assign32810_e47825)) / (assign32810_e47825 * assign32810_e47825))), (-((var_arg__blk1057_dn12 / (2.0 * assign32810_e47825)) / (assign32810_e47825 * assign32810_e47825))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32810_e47828;
        var_sarg_dn6 = assign32810_e47828_d_n6;
        var_sarg_dn7 = assign32810_e47828_d_n7;
        var_sarg_dn12 = assign32810_e47828_d_n12;

        let (assign32820_e47844, assign32820_e47844_d_n6, assign32820_e47844_d_n7, assign32820_e47844_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1077 != 0.0)) && (var_guard1078 == 0.0)) {
        let assign32820_e47841: f64 = (-p.p182);
        let assign32820_e47842: f64 = (var_arg__blk1057).powf(assign32820_e47841);
        (assign32820_e47842, if 0.0 == 0.0 && ((assign32820_e47841) as f64).is_finite() && ((assign32820_e47841) as f64).fract() == 0.0 { if assign32820_e47841 == 0.0 { 0.0 } else { (assign32820_e47841 * ((var_arg__blk1057).powf(assign32820_e47841 - 1.0) * var_arg__blk1057_dn6)) } } else { (assign32820_e47842 * (assign32820_e47841 * (var_arg__blk1057_dn6 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32820_e47841) as f64).is_finite() && ((assign32820_e47841) as f64).fract() == 0.0 { if assign32820_e47841 == 0.0 { 0.0 } else { (assign32820_e47841 * ((var_arg__blk1057).powf(assign32820_e47841 - 1.0) * var_arg__blk1057_dn7)) } } else { (assign32820_e47842 * (assign32820_e47841 * (var_arg__blk1057_dn7 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32820_e47841) as f64).is_finite() && ((assign32820_e47841) as f64).fract() == 0.0 { if assign32820_e47841 == 0.0 { 0.0 } else { (assign32820_e47841 * ((var_arg__blk1057).powf(assign32820_e47841 - 1.0) * var_arg__blk1057_dn12)) } } else { (assign32820_e47842 * (assign32820_e47841 * (var_arg__blk1057_dn12 / var_arg__blk1057))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32820_e47844;
        var_sarg_dn6 = assign32820_e47844_d_n6;
        var_sarg_dn7 = assign32820_e47844_d_n7;
        var_sarg_dn12 = assign32820_e47844_d_n12;

        let (assign32830_e47866, assign32830_e47866_d_n0, assign32830_e47866_d_n2, assign32830_e47866_d_n6, assign32830_e47866_d_n7, assign32830_e47866_d_n10, assign32830_e47866_d_n11, assign32830_e47866_d_n12, assign32830_e47866_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1077 != 0.0)) {
        let assign32830_e47854: f64 = (p.p185 * var_czbd);
        let assign32830_e47858: f64 = (var_arg__blk1057 * var_sarg);
        let assign32830_e47859: f64 = (1.0 - assign32830_e47858);
        let assign32830_e47860: f64 = (assign32830_e47854 * assign32830_e47859);
        let assign32830_e47863: f64 = (1.0 - p.p182);
        let assign32830_e47864: f64 = (assign32830_e47860 / assign32830_e47863);
        (assign32830_e47864, 0.0, 0.0, ((assign32830_e47854 * (-((var_arg__blk1057_dn6 * var_sarg) + (var_arg__blk1057 * var_sarg_dn6)))) / assign32830_e47863), ((assign32830_e47854 * (-((var_arg__blk1057_dn7 * var_sarg) + (var_arg__blk1057 * var_sarg_dn7)))) / assign32830_e47863), 0.0, 0.0, ((assign32830_e47854 * (-((var_arg__blk1057_dn12 * var_sarg) + (var_arg__blk1057 * var_sarg_dn12)))) / assign32830_e47863), 0.0,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign32830_e47866;
        var_qbd_dn0 = assign32830_e47866_d_n0;
        var_qbd_dn2 = assign32830_e47866_d_n2;
        var_qbd_dn6 = assign32830_e47866_d_n6;
        var_qbd_dn7 = assign32830_e47866_d_n7;
        var_qbd_dn10 = assign32830_e47866_d_n10;
        var_qbd_dn11 = assign32830_e47866_d_n11;
        var_qbd_dn12 = assign32830_e47866_d_n12;
        var_qbd_dn17 = assign32830_e47866_d_n17;

        let (assign32840_e47877, assign32840_e47877_d_n0, assign32840_e47877_d_n2, assign32840_e47877_d_n6, assign32840_e47877_d_n7, assign32840_e47877_d_n10, assign32840_e47877_d_n11, assign32840_e47877_d_n12, assign32840_e47877_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1077 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign32840_e47877;
        var_qbd_dn0 = assign32840_e47877_d_n0;
        var_qbd_dn2 = assign32840_e47877_d_n2;
        var_qbd_dn6 = assign32840_e47877_d_n6;
        var_qbd_dn7 = assign32840_e47877_d_n7;
        var_qbd_dn10 = assign32840_e47877_d_n10;
        var_qbd_dn11 = assign32840_e47877_d_n11;
        var_qbd_dn12 = assign32840_e47877_d_n12;
        var_qbd_dn17 = assign32840_e47877_d_n17;

        let assign32850_e47880: f64 = if var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        var_guard1079 = assign32850_e47880;

        let (assign32860_e47894, assign32860_e47894_d_n6, assign32860_e47894_d_n7, assign32860_e47894_d_n12,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1079 != 0.0)) {
        let assign32860_e47891: f64 = (var_vbdj / p.p186);
        let assign32860_e47892: f64 = (1.0 - assign32860_e47891);
        (assign32860_e47892, (-(var_vbdj_dn6 / p.p186)), 0.0, (-(var_vbdj_dn12 / p.p186)),)
    } else {
        (var_arg__blk1057, var_arg__blk1057_dn6, var_arg__blk1057_dn7, var_arg__blk1057_dn12,)
    }
};
        var_arg__blk1057 = assign32860_e47894;
        var_arg__blk1057_dn6 = assign32860_e47894_d_n6;
        var_arg__blk1057_dn7 = assign32860_e47894_d_n7;
        var_arg__blk1057_dn12 = assign32860_e47894_d_n12;

        let assign32870_e47897: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        var_guard1080 = assign32870_e47897;

        let (assign32880_e47912, assign32880_e47912_d_n6, assign32880_e47912_d_n7, assign32880_e47912_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1079 != 0.0)) && (var_guard1080 != 0.0)) {
        let assign32880_e47909: f64 = (var_arg__blk1057).sqrt();
        let assign32880_e47910: f64 = (1.0 / assign32880_e47909);
        (assign32880_e47910, (-((var_arg__blk1057_dn6 / (2.0 * assign32880_e47909)) / (assign32880_e47909 * assign32880_e47909))), (-((var_arg__blk1057_dn7 / (2.0 * assign32880_e47909)) / (assign32880_e47909 * assign32880_e47909))), (-((var_arg__blk1057_dn12 / (2.0 * assign32880_e47909)) / (assign32880_e47909 * assign32880_e47909))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32880_e47912;
        var_sarg_dn6 = assign32880_e47912_d_n6;
        var_sarg_dn7 = assign32880_e47912_d_n7;
        var_sarg_dn12 = assign32880_e47912_d_n12;

        let (assign32890_e47928, assign32890_e47928_d_n6, assign32890_e47928_d_n7, assign32890_e47928_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1079 != 0.0)) && (var_guard1080 == 0.0)) {
        let assign32890_e47925: f64 = (-p.p183);
        let assign32890_e47926: f64 = (var_arg__blk1057).powf(assign32890_e47925);
        (assign32890_e47926, if 0.0 == 0.0 && ((assign32890_e47925) as f64).is_finite() && ((assign32890_e47925) as f64).fract() == 0.0 { if assign32890_e47925 == 0.0 { 0.0 } else { (assign32890_e47925 * ((var_arg__blk1057).powf(assign32890_e47925 - 1.0) * var_arg__blk1057_dn6)) } } else { (assign32890_e47926 * (assign32890_e47925 * (var_arg__blk1057_dn6 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32890_e47925) as f64).is_finite() && ((assign32890_e47925) as f64).fract() == 0.0 { if assign32890_e47925 == 0.0 { 0.0 } else { (assign32890_e47925 * ((var_arg__blk1057).powf(assign32890_e47925 - 1.0) * var_arg__blk1057_dn7)) } } else { (assign32890_e47926 * (assign32890_e47925 * (var_arg__blk1057_dn7 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32890_e47925) as f64).is_finite() && ((assign32890_e47925) as f64).fract() == 0.0 { if assign32890_e47925 == 0.0 { 0.0 } else { (assign32890_e47925 * ((var_arg__blk1057).powf(assign32890_e47925 - 1.0) * var_arg__blk1057_dn12)) } } else { (assign32890_e47926 * (assign32890_e47925 * (var_arg__blk1057_dn12 / var_arg__blk1057))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32890_e47928;
        var_sarg_dn6 = assign32890_e47928_d_n6;
        var_sarg_dn7 = assign32890_e47928_d_n7;
        var_sarg_dn12 = assign32890_e47928_d_n12;

        *var_arg__blk1057_slot = var_arg__blk1057;
        *var_arg__blk1057_dn12_slot = var_arg__blk1057_dn12;
        *var_arg__blk1057_dn6_slot = var_arg__blk1057_dn6;
        *var_arg__blk1057_dn7_slot = var_arg__blk1057_dn7;
        *var_czbdsw_slot = var_czbdsw;
        *var_czbdswg_slot = var_czbdswg;
        *var_czbsswg_slot = var_czbsswg;
        *var_guard1070_slot = var_guard1070;
        *var_guard1071_slot = var_guard1071;
        *var_guard1072_slot = var_guard1072;
        *var_guard1073_slot = var_guard1073;
        *var_guard1074_slot = var_guard1074;
        *var_guard1075_slot = var_guard1075;
        *var_guard1076_slot = var_guard1076;
        *var_guard1077_slot = var_guard1077;
        *var_guard1078_slot = var_guard1078;
        *var_guard1079_slot = var_guard1079;
        *var_guard1080_slot = var_guard1080;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn0_slot = var_qbd_dn0;
        *var_qbd_dn10_slot = var_qbd_dn10;
        *var_qbd_dn11_slot = var_qbd_dn11;
        *var_qbd_dn12_slot = var_qbd_dn12;
        *var_qbd_dn17_slot = var_qbd_dn17;
        *var_qbd_dn2_slot = var_qbd_dn2;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn0_slot = var_qbs_dn0;
        *var_qbs_dn10_slot = var_qbs_dn10;
        *var_qbs_dn11_slot = var_qbs_dn11;
        *var_qbs_dn12_slot = var_qbs_dn12;
        *var_qbs_dn17_slot = var_qbs_dn17;
        *var_qbs_dn2_slot = var_qbs_dn2;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_sarg_slot = var_sarg;
        *var_sarg_dn12_slot = var_sarg_dn12;
        *var_sarg_dn6_slot = var_sarg_dn6;
        *var_sarg_dn7_slot = var_sarg_dn7;
        *var_t1__blk1032_slot = var_t1__blk1032;
        *var_t1__blk1032_dn10_slot = var_t1__blk1032_dn10;
        *var_t1__blk1032_dn12_slot = var_t1__blk1032_dn12;
        *var_t1__blk1032_dn6_slot = var_t1__blk1032_dn6;
        *var_t1__blk1032_dn7_slot = var_t1__blk1032_dn7;
        *var_t2__blk1033_slot = var_t2__blk1033;
        *var_t2__blk1033_dn0_slot = var_t2__blk1033_dn0;
        *var_t2__blk1033_dn10_slot = var_t2__blk1033_dn10;
        *var_t2__blk1033_dn11_slot = var_t2__blk1033_dn11;
        *var_t2__blk1033_dn12_slot = var_t2__blk1033_dn12;
        *var_t2__blk1033_dn17_slot = var_t2__blk1033_dn17;
        *var_t2__blk1033_dn2_slot = var_t2__blk1033_dn2;
        *var_t2__blk1033_dn6_slot = var_t2__blk1033_dn6;
        *var_t2__blk1033_dn7_slot = var_t2__blk1033_dn7;
    }

    pub(super) fn stamp_transient_block_116(
        p: &Parameters,
        var_czbd: f64,
        var_czbdsw: f64,
        var_czbs: f64,
        var_guard1030: f64,
        var_guard1075: f64,
        var_guard1076: f64,
        var_guard1079: f64,
        var_qbs: f64,
        var_qbs_dn0: f64,
        var_qbs_dn10: f64,
        var_qbs_dn11: f64,
        var_qbs_dn12: f64,
        var_qbs_dn17: f64,
        var_qbs_dn2: f64,
        var_qbs_dn6: f64,
        var_qbs_dn7: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn17: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn7: f64,
        var_vbdj: f64,
        var_vbdj_dn12: f64,
        var_vbdj_dn6: f64,
        var_xp_max: f64,
        var_arg__blk1057_slot: &mut f64,
        var_arg__blk1057_dn12_slot: &mut f64,
        var_arg__blk1057_dn6_slot: &mut f64,
        var_arg__blk1057_dn7_slot: &mut f64,
        var_czbdswg_slot: &mut f64,
        var_dlt_qbs_slot: &mut f64,
        var_dlt_qbs_dn0_slot: &mut f64,
        var_dlt_qbs_dn10_slot: &mut f64,
        var_dlt_qbs_dn11_slot: &mut f64,
        var_dlt_qbs_dn12_slot: &mut f64,
        var_dlt_qbs_dn17_slot: &mut f64,
        var_dlt_qbs_dn2_slot: &mut f64,
        var_dlt_qbs_dn6_slot: &mut f64,
        var_dlt_qbs_dn7_slot: &mut f64,
        var_guard1081_slot: &mut f64,
        var_guard1082_slot: &mut f64,
        var_guard1083_slot: &mut f64,
        var_guard1084_slot: &mut f64,
        var_guard1085_slot: &mut f64,
        var_guard1086_slot: &mut f64,
        var_guard1087_slot: &mut f64,
        var_guard1088_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn0_slot: &mut f64,
        var_qbd_dn10_slot: &mut f64,
        var_qbd_dn11_slot: &mut f64,
        var_qbd_dn12_slot: &mut f64,
        var_qbd_dn17_slot: &mut f64,
        var_qbd_dn2_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbs_max_slot: &mut f64,
        var_qbs_max_dn0_slot: &mut f64,
        var_qbs_max_dn10_slot: &mut f64,
        var_qbs_max_dn11_slot: &mut f64,
        var_qbs_max_dn12_slot: &mut f64,
        var_qbs_max_dn17_slot: &mut f64,
        var_qbs_max_dn2_slot: &mut f64,
        var_qbs_max_dn6_slot: &mut f64,
        var_qbs_max_dn7_slot: &mut f64,
        var_sarg_slot: &mut f64,
        var_sarg_dn12_slot: &mut f64,
        var_sarg_dn6_slot: &mut f64,
        var_sarg_dn7_slot: &mut f64,
        var_t1__blk1032_slot: &mut f64,
        var_t1__blk1032_dn10_slot: &mut f64,
        var_t1__blk1032_dn12_slot: &mut f64,
        var_t1__blk1032_dn6_slot: &mut f64,
        var_t1__blk1032_dn7_slot: &mut f64,
        var_t2__blk1033_slot: &mut f64,
        var_t2__blk1033_dn0_slot: &mut f64,
        var_t2__blk1033_dn10_slot: &mut f64,
        var_t2__blk1033_dn11_slot: &mut f64,
        var_t2__blk1033_dn12_slot: &mut f64,
        var_t2__blk1033_dn17_slot: &mut f64,
        var_t2__blk1033_dn2_slot: &mut f64,
        var_t2__blk1033_dn6_slot: &mut f64,
        var_t2__blk1033_dn7_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
    ) {
        let mut var_arg__blk1057: f64 = *var_arg__blk1057_slot;
        let mut var_arg__blk1057_dn12: f64 = *var_arg__blk1057_dn12_slot;
        let mut var_arg__blk1057_dn6: f64 = *var_arg__blk1057_dn6_slot;
        let mut var_arg__blk1057_dn7: f64 = *var_arg__blk1057_dn7_slot;
        let mut var_czbdswg: f64 = *var_czbdswg_slot;
        let mut var_dlt_qbs: f64 = *var_dlt_qbs_slot;
        let mut var_dlt_qbs_dn0: f64 = *var_dlt_qbs_dn0_slot;
        let mut var_dlt_qbs_dn10: f64 = *var_dlt_qbs_dn10_slot;
        let mut var_dlt_qbs_dn11: f64 = *var_dlt_qbs_dn11_slot;
        let mut var_dlt_qbs_dn12: f64 = *var_dlt_qbs_dn12_slot;
        let mut var_dlt_qbs_dn17: f64 = *var_dlt_qbs_dn17_slot;
        let mut var_dlt_qbs_dn2: f64 = *var_dlt_qbs_dn2_slot;
        let mut var_dlt_qbs_dn6: f64 = *var_dlt_qbs_dn6_slot;
        let mut var_dlt_qbs_dn7: f64 = *var_dlt_qbs_dn7_slot;
        let mut var_guard1081: f64 = *var_guard1081_slot;
        let mut var_guard1082: f64 = *var_guard1082_slot;
        let mut var_guard1083: f64 = *var_guard1083_slot;
        let mut var_guard1084: f64 = *var_guard1084_slot;
        let mut var_guard1085: f64 = *var_guard1085_slot;
        let mut var_guard1086: f64 = *var_guard1086_slot;
        let mut var_guard1087: f64 = *var_guard1087_slot;
        let mut var_guard1088: f64 = *var_guard1088_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn0: f64 = *var_qbd_dn0_slot;
        let mut var_qbd_dn10: f64 = *var_qbd_dn10_slot;
        let mut var_qbd_dn11: f64 = *var_qbd_dn11_slot;
        let mut var_qbd_dn12: f64 = *var_qbd_dn12_slot;
        let mut var_qbd_dn17: f64 = *var_qbd_dn17_slot;
        let mut var_qbd_dn2: f64 = *var_qbd_dn2_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbs_max: f64 = *var_qbs_max_slot;
        let mut var_qbs_max_dn0: f64 = *var_qbs_max_dn0_slot;
        let mut var_qbs_max_dn10: f64 = *var_qbs_max_dn10_slot;
        let mut var_qbs_max_dn11: f64 = *var_qbs_max_dn11_slot;
        let mut var_qbs_max_dn12: f64 = *var_qbs_max_dn12_slot;
        let mut var_qbs_max_dn17: f64 = *var_qbs_max_dn17_slot;
        let mut var_qbs_max_dn2: f64 = *var_qbs_max_dn2_slot;
        let mut var_qbs_max_dn6: f64 = *var_qbs_max_dn6_slot;
        let mut var_qbs_max_dn7: f64 = *var_qbs_max_dn7_slot;
        let mut var_sarg: f64 = *var_sarg_slot;
        let mut var_sarg_dn12: f64 = *var_sarg_dn12_slot;
        let mut var_sarg_dn6: f64 = *var_sarg_dn6_slot;
        let mut var_sarg_dn7: f64 = *var_sarg_dn7_slot;
        let mut var_t1__blk1032: f64 = *var_t1__blk1032_slot;
        let mut var_t1__blk1032_dn10: f64 = *var_t1__blk1032_dn10_slot;
        let mut var_t1__blk1032_dn12: f64 = *var_t1__blk1032_dn12_slot;
        let mut var_t1__blk1032_dn6: f64 = *var_t1__blk1032_dn6_slot;
        let mut var_t1__blk1032_dn7: f64 = *var_t1__blk1032_dn7_slot;
        let mut var_t2__blk1033: f64 = *var_t2__blk1033_slot;
        let mut var_t2__blk1033_dn0: f64 = *var_t2__blk1033_dn0_slot;
        let mut var_t2__blk1033_dn10: f64 = *var_t2__blk1033_dn10_slot;
        let mut var_t2__blk1033_dn11: f64 = *var_t2__blk1033_dn11_slot;
        let mut var_t2__blk1033_dn12: f64 = *var_t2__blk1033_dn12_slot;
        let mut var_t2__blk1033_dn17: f64 = *var_t2__blk1033_dn17_slot;
        let mut var_t2__blk1033_dn2: f64 = *var_t2__blk1033_dn2_slot;
        let mut var_t2__blk1033_dn6: f64 = *var_t2__blk1033_dn6_slot;
        let mut var_t2__blk1033_dn7: f64 = *var_t2__blk1033_dn7_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;

        let (assign32900_e47952, assign32900_e47952_d_n0, assign32900_e47952_d_n2, assign32900_e47952_d_n6, assign32900_e47952_d_n7, assign32900_e47952_d_n10, assign32900_e47952_d_n11, assign32900_e47952_d_n12, assign32900_e47952_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1079 != 0.0)) {
        let assign32900_e47939: f64 = (p.p186 * var_czbdsw);
        let assign32900_e47943: f64 = (var_arg__blk1057 * var_sarg);
        let assign32900_e47944: f64 = (1.0 - assign32900_e47943);
        let assign32900_e47945: f64 = (assign32900_e47939 * assign32900_e47944);
        let assign32900_e47948: f64 = (1.0 - p.p183);
        let assign32900_e47949: f64 = (assign32900_e47945 / assign32900_e47948);
        let assign32900_e47950: f64 = (var_qbd + assign32900_e47949);
        (assign32900_e47950, var_qbd_dn0, var_qbd_dn2, (var_qbd_dn6 + ((assign32900_e47939 * (-((var_arg__blk1057_dn6 * var_sarg) + (var_arg__blk1057 * var_sarg_dn6)))) / assign32900_e47948)), (var_qbd_dn7 + ((assign32900_e47939 * (-((var_arg__blk1057_dn7 * var_sarg) + (var_arg__blk1057 * var_sarg_dn7)))) / assign32900_e47948)), var_qbd_dn10, var_qbd_dn11, (var_qbd_dn12 + ((assign32900_e47939 * (-((var_arg__blk1057_dn12 * var_sarg) + (var_arg__blk1057 * var_sarg_dn12)))) / assign32900_e47948)), var_qbd_dn17,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign32900_e47952;
        var_qbd_dn0 = assign32900_e47952_d_n0;
        var_qbd_dn2 = assign32900_e47952_d_n2;
        var_qbd_dn6 = assign32900_e47952_d_n6;
        var_qbd_dn7 = assign32900_e47952_d_n7;
        var_qbd_dn10 = assign32900_e47952_d_n10;
        var_qbd_dn11 = assign32900_e47952_d_n11;
        var_qbd_dn12 = assign32900_e47952_d_n12;
        var_qbd_dn17 = assign32900_e47952_d_n17;

        let assign32910_e47955: f64 = if var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        var_guard1081 = assign32910_e47955;

        let (assign32920_e47969, assign32920_e47969_d_n6, assign32920_e47969_d_n7, assign32920_e47969_d_n12,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1081 != 0.0)) {
        let assign32920_e47966: f64 = (var_vbdj / p.p187);
        let assign32920_e47967: f64 = (1.0 - assign32920_e47966);
        (assign32920_e47967, (-(var_vbdj_dn6 / p.p187)), 0.0, (-(var_vbdj_dn12 / p.p187)),)
    } else {
        (var_arg__blk1057, var_arg__blk1057_dn6, var_arg__blk1057_dn7, var_arg__blk1057_dn12,)
    }
};
        var_arg__blk1057 = assign32920_e47969;
        var_arg__blk1057_dn6 = assign32920_e47969_d_n6;
        var_arg__blk1057_dn7 = assign32920_e47969_d_n7;
        var_arg__blk1057_dn12 = assign32920_e47969_d_n12;

        let assign32930_e47972: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        var_guard1082 = assign32930_e47972;

        let (assign32940_e47987, assign32940_e47987_d_n6, assign32940_e47987_d_n7, assign32940_e47987_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1081 != 0.0)) && (var_guard1082 != 0.0)) {
        let assign32940_e47984: f64 = (var_arg__blk1057).sqrt();
        let assign32940_e47985: f64 = (1.0 / assign32940_e47984);
        (assign32940_e47985, (-((var_arg__blk1057_dn6 / (2.0 * assign32940_e47984)) / (assign32940_e47984 * assign32940_e47984))), (-((var_arg__blk1057_dn7 / (2.0 * assign32940_e47984)) / (assign32940_e47984 * assign32940_e47984))), (-((var_arg__blk1057_dn12 / (2.0 * assign32940_e47984)) / (assign32940_e47984 * assign32940_e47984))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32940_e47987;
        var_sarg_dn6 = assign32940_e47987_d_n6;
        var_sarg_dn7 = assign32940_e47987_d_n7;
        var_sarg_dn12 = assign32940_e47987_d_n12;

        let (assign32950_e48003, assign32950_e48003_d_n6, assign32950_e48003_d_n7, assign32950_e48003_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1081 != 0.0)) && (var_guard1082 == 0.0)) {
        let assign32950_e48000: f64 = (-p.p184);
        let assign32950_e48001: f64 = (var_arg__blk1057).powf(assign32950_e48000);
        (assign32950_e48001, if 0.0 == 0.0 && ((assign32950_e48000) as f64).is_finite() && ((assign32950_e48000) as f64).fract() == 0.0 { if assign32950_e48000 == 0.0 { 0.0 } else { (assign32950_e48000 * ((var_arg__blk1057).powf(assign32950_e48000 - 1.0) * var_arg__blk1057_dn6)) } } else { (assign32950_e48001 * (assign32950_e48000 * (var_arg__blk1057_dn6 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32950_e48000) as f64).is_finite() && ((assign32950_e48000) as f64).fract() == 0.0 { if assign32950_e48000 == 0.0 { 0.0 } else { (assign32950_e48000 * ((var_arg__blk1057).powf(assign32950_e48000 - 1.0) * var_arg__blk1057_dn7)) } } else { (assign32950_e48001 * (assign32950_e48000 * (var_arg__blk1057_dn7 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign32950_e48000) as f64).is_finite() && ((assign32950_e48000) as f64).fract() == 0.0 { if assign32950_e48000 == 0.0 { 0.0 } else { (assign32950_e48000 * ((var_arg__blk1057).powf(assign32950_e48000 - 1.0) * var_arg__blk1057_dn12)) } } else { (assign32950_e48001 * (assign32950_e48000 * (var_arg__blk1057_dn12 / var_arg__blk1057))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32950_e48003;
        var_sarg_dn6 = assign32950_e48003_d_n6;
        var_sarg_dn7 = assign32950_e48003_d_n7;
        var_sarg_dn12 = assign32950_e48003_d_n12;

        let (assign32960_e48027, assign32960_e48027_d_n0, assign32960_e48027_d_n2, assign32960_e48027_d_n6, assign32960_e48027_d_n7, assign32960_e48027_d_n10, assign32960_e48027_d_n11, assign32960_e48027_d_n12, assign32960_e48027_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) && (var_guard1081 != 0.0)) {
        let assign32960_e48014: f64 = (p.p187 * var_czbdswg);
        let assign32960_e48018: f64 = (var_arg__blk1057 * var_sarg);
        let assign32960_e48019: f64 = (1.0 - assign32960_e48018);
        let assign32960_e48020: f64 = (assign32960_e48014 * assign32960_e48019);
        let assign32960_e48023: f64 = (1.0 - p.p184);
        let assign32960_e48024: f64 = (assign32960_e48020 / assign32960_e48023);
        let assign32960_e48025: f64 = (var_qbd + assign32960_e48024);
        (assign32960_e48025, var_qbd_dn0, var_qbd_dn2, (var_qbd_dn6 + ((assign32960_e48014 * (-((var_arg__blk1057_dn6 * var_sarg) + (var_arg__blk1057 * var_sarg_dn6)))) / assign32960_e48023)), (var_qbd_dn7 + ((assign32960_e48014 * (-((var_arg__blk1057_dn7 * var_sarg) + (var_arg__blk1057 * var_sarg_dn7)))) / assign32960_e48023)), var_qbd_dn10, var_qbd_dn11, (var_qbd_dn12 + ((assign32960_e48014 * (-((var_arg__blk1057_dn12 * var_sarg) + (var_arg__blk1057 * var_sarg_dn12)))) / assign32960_e48023)), var_qbd_dn17,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign32960_e48027;
        var_qbd_dn0 = assign32960_e48027_d_n0;
        var_qbd_dn2 = assign32960_e48027_d_n2;
        var_qbd_dn6 = assign32960_e48027_d_n6;
        var_qbd_dn7 = assign32960_e48027_d_n7;
        var_qbd_dn10 = assign32960_e48027_d_n10;
        var_qbd_dn11 = assign32960_e48027_d_n11;
        var_qbd_dn12 = assign32960_e48027_d_n12;
        var_qbd_dn17 = assign32960_e48027_d_n17;

        let (assign32970_e48040, assign32970_e48040_d_n6, assign32970_e48040_d_n7, assign32970_e48040_d_n10, assign32970_e48040_d_n12,) = {
    if (((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 == 0.0)) {
        let assign32970_e48036: f64 = (var_czbd + var_czbdsw);
        let assign32970_e48038: f64 = (assign32970_e48036 + var_czbdswg);
        (assign32970_e48038, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk1032, var_t1__blk1032_dn6, var_t1__blk1032_dn7, var_t1__blk1032_dn10, var_t1__blk1032_dn12,)
    }
};
        var_t1__blk1032 = assign32970_e48040;
        var_t1__blk1032_dn6 = assign32970_e48040_d_n6;
        var_t1__blk1032_dn7 = assign32970_e48040_d_n7;
        var_t1__blk1032_dn10 = assign32970_e48040_d_n10;
        var_t1__blk1032_dn12 = assign32970_e48040_d_n12;

        let (assign32980_e48065, assign32980_e48065_d_n0, assign32980_e48065_d_n2, assign32980_e48065_d_n6, assign32980_e48065_d_n7, assign32980_e48065_d_n10, assign32980_e48065_d_n11, assign32980_e48065_d_n12, assign32980_e48065_d_n17,) = {
    if (((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 == 0.0)) {
        let assign32980_e48049: f64 = (var_czbd * p.p182);
        let assign32980_e48051: f64 = (assign32980_e48049 / p.p185);
        let assign32980_e48054: f64 = (var_czbdsw * p.p183);
        let assign32980_e48056: f64 = (assign32980_e48054 / p.p186);
        let assign32980_e48057: f64 = (assign32980_e48051 + assign32980_e48056);
        let assign32980_e48060: f64 = (var_czbdswg * p.p184);
        let assign32980_e48062: f64 = (assign32980_e48060 / p.p187);
        let assign32980_e48063: f64 = (assign32980_e48057 + assign32980_e48062);
        (assign32980_e48063, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2__blk1033, var_t2__blk1033_dn0, var_t2__blk1033_dn2, var_t2__blk1033_dn6, var_t2__blk1033_dn7, var_t2__blk1033_dn10, var_t2__blk1033_dn11, var_t2__blk1033_dn12, var_t2__blk1033_dn17,)
    }
};
        var_t2__blk1033 = assign32980_e48065;
        var_t2__blk1033_dn0 = assign32980_e48065_d_n0;
        var_t2__blk1033_dn2 = assign32980_e48065_d_n2;
        var_t2__blk1033_dn6 = assign32980_e48065_d_n6;
        var_t2__blk1033_dn7 = assign32980_e48065_d_n7;
        var_t2__blk1033_dn10 = assign32980_e48065_d_n10;
        var_t2__blk1033_dn11 = assign32980_e48065_d_n11;
        var_t2__blk1033_dn12 = assign32980_e48065_d_n12;
        var_t2__blk1033_dn17 = assign32980_e48065_d_n17;

        let (assign32990_e48082, assign32990_e48082_d_n0, assign32990_e48082_d_n2, assign32990_e48082_d_n6, assign32990_e48082_d_n7, assign32990_e48082_d_n10, assign32990_e48082_d_n11, assign32990_e48082_d_n12, assign32990_e48082_d_n17,) = {
    if (((var_guard1030 != 0.0) && (var_guard1075 != 0.0)) && (var_guard1076 == 0.0)) {
        let assign32990_e48076: f64 = (var_vbdj * 0.5);
        let assign32990_e48078: f64 = (assign32990_e48076 * var_t2__blk1033);
        let assign32990_e48079: f64 = (var_t1__blk1032 + assign32990_e48078);
        let assign32990_e48080: f64 = (var_vbdj * assign32990_e48079);
        (assign32990_e48080, (var_vbdj * (assign32990_e48076 * var_t2__blk1033_dn0)), (var_vbdj * (assign32990_e48076 * var_t2__blk1033_dn2)), ((var_vbdj_dn6 * assign32990_e48079) + (var_vbdj * (var_t1__blk1032_dn6 + (((var_vbdj_dn6 * 0.5) * var_t2__blk1033) + (assign32990_e48076 * var_t2__blk1033_dn6))))), (var_vbdj * (var_t1__blk1032_dn7 + (assign32990_e48076 * var_t2__blk1033_dn7))), (var_vbdj * (var_t1__blk1032_dn10 + (assign32990_e48076 * var_t2__blk1033_dn10))), (var_vbdj * (assign32990_e48076 * var_t2__blk1033_dn11)), ((var_vbdj_dn12 * assign32990_e48079) + (var_vbdj * (var_t1__blk1032_dn12 + (((var_vbdj_dn12 * 0.5) * var_t2__blk1033) + (assign32990_e48076 * var_t2__blk1033_dn12))))), (var_vbdj * (assign32990_e48076 * var_t2__blk1033_dn17)),)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign32990_e48082;
        var_qbd_dn0 = assign32990_e48082_d_n0;
        var_qbd_dn2 = assign32990_e48082_d_n2;
        var_qbd_dn6 = assign32990_e48082_d_n6;
        var_qbd_dn7 = assign32990_e48082_d_n7;
        var_qbd_dn10 = assign32990_e48082_d_n10;
        var_qbd_dn11 = assign32990_e48082_d_n11;
        var_qbd_dn12 = assign32990_e48082_d_n12;
        var_qbd_dn17 = assign32990_e48082_d_n17;

        let (assign33000_e48091,) = {
    if ((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) {
        let assign33000_e48089: f64 = (p.p181 * p.p4);
        (assign33000_e48089,)
    } else {
        (var_czbdswg,)
    }
};
        var_czbdswg = assign33000_e48091;

        let assign33010_e48094: f64 = if var_vbdj < 0.0 { 1.0 } else { 0.0 };
        var_guard1083 = assign33010_e48094;

        let assign33020_e48097: f64 = if var_czbd > 0.0 { 1.0 } else { 0.0 };
        var_guard1084 = assign33020_e48097;

        let (assign33030_e48112, assign33030_e48112_d_n6, assign33030_e48112_d_n7, assign33030_e48112_d_n12,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) && (var_guard1083 != 0.0)) && (var_guard1084 != 0.0)) {
        let assign33030_e48109: f64 = (var_vbdj / p.p185);
        let assign33030_e48110: f64 = (1.0 - assign33030_e48109);
        (assign33030_e48110, (-(var_vbdj_dn6 / p.p185)), 0.0, (-(var_vbdj_dn12 / p.p185)),)
    } else {
        (var_arg__blk1057, var_arg__blk1057_dn6, var_arg__blk1057_dn7, var_arg__blk1057_dn12,)
    }
};
        var_arg__blk1057 = assign33030_e48112;
        var_arg__blk1057_dn6 = assign33030_e48112_d_n6;
        var_arg__blk1057_dn7 = assign33030_e48112_d_n7;
        var_arg__blk1057_dn12 = assign33030_e48112_d_n12;

        let assign33040_e48115: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        var_guard1085 = assign33040_e48115;

        let (assign33050_e48131, assign33050_e48131_d_n6, assign33050_e48131_d_n7, assign33050_e48131_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) && (var_guard1083 != 0.0)) && (var_guard1084 != 0.0)) && (var_guard1085 != 0.0)) {
        let assign33050_e48128: f64 = (var_arg__blk1057).sqrt();
        let assign33050_e48129: f64 = (1.0 / assign33050_e48128);
        (assign33050_e48129, (-((var_arg__blk1057_dn6 / (2.0 * assign33050_e48128)) / (assign33050_e48128 * assign33050_e48128))), (-((var_arg__blk1057_dn7 / (2.0 * assign33050_e48128)) / (assign33050_e48128 * assign33050_e48128))), (-((var_arg__blk1057_dn12 / (2.0 * assign33050_e48128)) / (assign33050_e48128 * assign33050_e48128))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign33050_e48131;
        var_sarg_dn6 = assign33050_e48131_d_n6;
        var_sarg_dn7 = assign33050_e48131_d_n7;
        var_sarg_dn12 = assign33050_e48131_d_n12;

        let (assign33060_e48148, assign33060_e48148_d_n6, assign33060_e48148_d_n7, assign33060_e48148_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) && (var_guard1083 != 0.0)) && (var_guard1084 != 0.0)) && (var_guard1085 == 0.0)) {
        let assign33060_e48145: f64 = (-p.p182);
        let assign33060_e48146: f64 = (var_arg__blk1057).powf(assign33060_e48145);
        (assign33060_e48146, if 0.0 == 0.0 && ((assign33060_e48145) as f64).is_finite() && ((assign33060_e48145) as f64).fract() == 0.0 { if assign33060_e48145 == 0.0 { 0.0 } else { (assign33060_e48145 * ((var_arg__blk1057).powf(assign33060_e48145 - 1.0) * var_arg__blk1057_dn6)) } } else { (assign33060_e48146 * (assign33060_e48145 * (var_arg__blk1057_dn6 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign33060_e48145) as f64).is_finite() && ((assign33060_e48145) as f64).fract() == 0.0 { if assign33060_e48145 == 0.0 { 0.0 } else { (assign33060_e48145 * ((var_arg__blk1057).powf(assign33060_e48145 - 1.0) * var_arg__blk1057_dn7)) } } else { (assign33060_e48146 * (assign33060_e48145 * (var_arg__blk1057_dn7 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign33060_e48145) as f64).is_finite() && ((assign33060_e48145) as f64).fract() == 0.0 { if assign33060_e48145 == 0.0 { 0.0 } else { (assign33060_e48145 * ((var_arg__blk1057).powf(assign33060_e48145 - 1.0) * var_arg__blk1057_dn12)) } } else { (assign33060_e48146 * (assign33060_e48145 * (var_arg__blk1057_dn12 / var_arg__blk1057))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign33060_e48148;
        var_sarg_dn6 = assign33060_e48148_d_n6;
        var_sarg_dn7 = assign33060_e48148_d_n7;
        var_sarg_dn12 = assign33060_e48148_d_n12;

        let (assign33070_e48171, assign33070_e48171_d_n0, assign33070_e48171_d_n2, assign33070_e48171_d_n6, assign33070_e48171_d_n7, assign33070_e48171_d_n10, assign33070_e48171_d_n11, assign33070_e48171_d_n12, assign33070_e48171_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) && (var_guard1083 != 0.0)) && (var_guard1084 != 0.0)) {
        let assign33070_e48159: f64 = (p.p185 * var_czbd);
        let assign33070_e48163: f64 = (var_arg__blk1057 * var_sarg);
        let assign33070_e48164: f64 = (1.0 - assign33070_e48163);
        let assign33070_e48165: f64 = (assign33070_e48159 * assign33070_e48164);
        let assign33070_e48168: f64 = (1.0 - p.p182);
        let assign33070_e48169: f64 = (assign33070_e48165 / assign33070_e48168);
        (assign33070_e48169, 0.0, 0.0, ((assign33070_e48159 * (-((var_arg__blk1057_dn6 * var_sarg) + (var_arg__blk1057 * var_sarg_dn6)))) / assign33070_e48168), ((assign33070_e48159 * (-((var_arg__blk1057_dn7 * var_sarg) + (var_arg__blk1057 * var_sarg_dn7)))) / assign33070_e48168), 0.0, 0.0, ((assign33070_e48159 * (-((var_arg__blk1057_dn12 * var_sarg) + (var_arg__blk1057 * var_sarg_dn12)))) / assign33070_e48168), 0.0,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33070_e48171;
        var_qbd_dn0 = assign33070_e48171_d_n0;
        var_qbd_dn2 = assign33070_e48171_d_n2;
        var_qbd_dn6 = assign33070_e48171_d_n6;
        var_qbd_dn7 = assign33070_e48171_d_n7;
        var_qbd_dn10 = assign33070_e48171_d_n10;
        var_qbd_dn11 = assign33070_e48171_d_n11;
        var_qbd_dn12 = assign33070_e48171_d_n12;
        var_qbd_dn17 = assign33070_e48171_d_n17;

        let (assign33080_e48183, assign33080_e48183_d_n0, assign33080_e48183_d_n2, assign33080_e48183_d_n6, assign33080_e48183_d_n7, assign33080_e48183_d_n10, assign33080_e48183_d_n11, assign33080_e48183_d_n12, assign33080_e48183_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) && (var_guard1083 != 0.0)) && (var_guard1084 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33080_e48183;
        var_qbd_dn0 = assign33080_e48183_d_n0;
        var_qbd_dn2 = assign33080_e48183_d_n2;
        var_qbd_dn6 = assign33080_e48183_d_n6;
        var_qbd_dn7 = assign33080_e48183_d_n7;
        var_qbd_dn10 = assign33080_e48183_d_n10;
        var_qbd_dn11 = assign33080_e48183_d_n11;
        var_qbd_dn12 = assign33080_e48183_d_n12;
        var_qbd_dn17 = assign33080_e48183_d_n17;

        let assign33090_e48186: f64 = if var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        var_guard1086 = assign33090_e48186;

        let (assign33100_e48201, assign33100_e48201_d_n6, assign33100_e48201_d_n7, assign33100_e48201_d_n12,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) && (var_guard1083 != 0.0)) && (var_guard1086 != 0.0)) {
        let assign33100_e48198: f64 = (var_vbdj / p.p187);
        let assign33100_e48199: f64 = (1.0 - assign33100_e48198);
        (assign33100_e48199, (-(var_vbdj_dn6 / p.p187)), 0.0, (-(var_vbdj_dn12 / p.p187)),)
    } else {
        (var_arg__blk1057, var_arg__blk1057_dn6, var_arg__blk1057_dn7, var_arg__blk1057_dn12,)
    }
};
        var_arg__blk1057 = assign33100_e48201;
        var_arg__blk1057_dn6 = assign33100_e48201_d_n6;
        var_arg__blk1057_dn7 = assign33100_e48201_d_n7;
        var_arg__blk1057_dn12 = assign33100_e48201_d_n12;

        let assign33110_e48204: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        var_guard1087 = assign33110_e48204;

        let (assign33120_e48220, assign33120_e48220_d_n6, assign33120_e48220_d_n7, assign33120_e48220_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) && (var_guard1083 != 0.0)) && (var_guard1086 != 0.0)) && (var_guard1087 != 0.0)) {
        let assign33120_e48217: f64 = (var_arg__blk1057).sqrt();
        let assign33120_e48218: f64 = (1.0 / assign33120_e48217);
        (assign33120_e48218, (-((var_arg__blk1057_dn6 / (2.0 * assign33120_e48217)) / (assign33120_e48217 * assign33120_e48217))), (-((var_arg__blk1057_dn7 / (2.0 * assign33120_e48217)) / (assign33120_e48217 * assign33120_e48217))), (-((var_arg__blk1057_dn12 / (2.0 * assign33120_e48217)) / (assign33120_e48217 * assign33120_e48217))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign33120_e48220;
        var_sarg_dn6 = assign33120_e48220_d_n6;
        var_sarg_dn7 = assign33120_e48220_d_n7;
        var_sarg_dn12 = assign33120_e48220_d_n12;

        let (assign33130_e48237, assign33130_e48237_d_n6, assign33130_e48237_d_n7, assign33130_e48237_d_n12,) = {
    if (((((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) && (var_guard1083 != 0.0)) && (var_guard1086 != 0.0)) && (var_guard1087 == 0.0)) {
        let assign33130_e48234: f64 = (-p.p184);
        let assign33130_e48235: f64 = (var_arg__blk1057).powf(assign33130_e48234);
        (assign33130_e48235, if 0.0 == 0.0 && ((assign33130_e48234) as f64).is_finite() && ((assign33130_e48234) as f64).fract() == 0.0 { if assign33130_e48234 == 0.0 { 0.0 } else { (assign33130_e48234 * ((var_arg__blk1057).powf(assign33130_e48234 - 1.0) * var_arg__blk1057_dn6)) } } else { (assign33130_e48235 * (assign33130_e48234 * (var_arg__blk1057_dn6 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign33130_e48234) as f64).is_finite() && ((assign33130_e48234) as f64).fract() == 0.0 { if assign33130_e48234 == 0.0 { 0.0 } else { (assign33130_e48234 * ((var_arg__blk1057).powf(assign33130_e48234 - 1.0) * var_arg__blk1057_dn7)) } } else { (assign33130_e48235 * (assign33130_e48234 * (var_arg__blk1057_dn7 / var_arg__blk1057))) }, if 0.0 == 0.0 && ((assign33130_e48234) as f64).is_finite() && ((assign33130_e48234) as f64).fract() == 0.0 { if assign33130_e48234 == 0.0 { 0.0 } else { (assign33130_e48234 * ((var_arg__blk1057).powf(assign33130_e48234 - 1.0) * var_arg__blk1057_dn12)) } } else { (assign33130_e48235 * (assign33130_e48234 * (var_arg__blk1057_dn12 / var_arg__blk1057))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign33130_e48237;
        var_sarg_dn6 = assign33130_e48237_d_n6;
        var_sarg_dn7 = assign33130_e48237_d_n7;
        var_sarg_dn12 = assign33130_e48237_d_n12;

        let (assign33140_e48262, assign33140_e48262_d_n0, assign33140_e48262_d_n2, assign33140_e48262_d_n6, assign33140_e48262_d_n7, assign33140_e48262_d_n10, assign33140_e48262_d_n11, assign33140_e48262_d_n12, assign33140_e48262_d_n17,) = {
    if ((((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) && (var_guard1083 != 0.0)) && (var_guard1086 != 0.0)) {
        let assign33140_e48249: f64 = (p.p187 * var_czbdswg);
        let assign33140_e48253: f64 = (var_arg__blk1057 * var_sarg);
        let assign33140_e48254: f64 = (1.0 - assign33140_e48253);
        let assign33140_e48255: f64 = (assign33140_e48249 * assign33140_e48254);
        let assign33140_e48258: f64 = (1.0 - p.p184);
        let assign33140_e48259: f64 = (assign33140_e48255 / assign33140_e48258);
        let assign33140_e48260: f64 = (var_qbd + assign33140_e48259);
        (assign33140_e48260, var_qbd_dn0, var_qbd_dn2, (var_qbd_dn6 + ((assign33140_e48249 * (-((var_arg__blk1057_dn6 * var_sarg) + (var_arg__blk1057 * var_sarg_dn6)))) / assign33140_e48258)), (var_qbd_dn7 + ((assign33140_e48249 * (-((var_arg__blk1057_dn7 * var_sarg) + (var_arg__blk1057 * var_sarg_dn7)))) / assign33140_e48258)), var_qbd_dn10, var_qbd_dn11, (var_qbd_dn12 + ((assign33140_e48249 * (-((var_arg__blk1057_dn12 * var_sarg) + (var_arg__blk1057 * var_sarg_dn12)))) / assign33140_e48258)), var_qbd_dn17,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33140_e48262;
        var_qbd_dn0 = assign33140_e48262_d_n0;
        var_qbd_dn2 = assign33140_e48262_d_n2;
        var_qbd_dn6 = assign33140_e48262_d_n6;
        var_qbd_dn7 = assign33140_e48262_d_n7;
        var_qbd_dn10 = assign33140_e48262_d_n10;
        var_qbd_dn11 = assign33140_e48262_d_n11;
        var_qbd_dn12 = assign33140_e48262_d_n12;
        var_qbd_dn17 = assign33140_e48262_d_n17;

        let (assign33150_e48274, assign33150_e48274_d_n6, assign33150_e48274_d_n7, assign33150_e48274_d_n10, assign33150_e48274_d_n12,) = {
    if (((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) && (var_guard1083 == 0.0)) {
        let assign33150_e48272: f64 = (var_czbd + var_czbdswg);
        (assign33150_e48272, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk1032, var_t1__blk1032_dn6, var_t1__blk1032_dn7, var_t1__blk1032_dn10, var_t1__blk1032_dn12,)
    }
};
        var_t1__blk1032 = assign33150_e48274;
        var_t1__blk1032_dn6 = assign33150_e48274_d_n6;
        var_t1__blk1032_dn7 = assign33150_e48274_d_n7;
        var_t1__blk1032_dn10 = assign33150_e48274_d_n10;
        var_t1__blk1032_dn12 = assign33150_e48274_d_n12;

        let (assign33160_e48294, assign33160_e48294_d_n0, assign33160_e48294_d_n2, assign33160_e48294_d_n6, assign33160_e48294_d_n7, assign33160_e48294_d_n10, assign33160_e48294_d_n11, assign33160_e48294_d_n12, assign33160_e48294_d_n17,) = {
    if (((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) && (var_guard1083 == 0.0)) {
        let assign33160_e48284: f64 = (var_czbd * p.p182);
        let assign33160_e48286: f64 = (assign33160_e48284 / p.p185);
        let assign33160_e48289: f64 = (var_czbdswg * p.p184);
        let assign33160_e48291: f64 = (assign33160_e48289 / p.p187);
        let assign33160_e48292: f64 = (assign33160_e48286 + assign33160_e48291);
        (assign33160_e48292, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2__blk1033, var_t2__blk1033_dn0, var_t2__blk1033_dn2, var_t2__blk1033_dn6, var_t2__blk1033_dn7, var_t2__blk1033_dn10, var_t2__blk1033_dn11, var_t2__blk1033_dn12, var_t2__blk1033_dn17,)
    }
};
        var_t2__blk1033 = assign33160_e48294;
        var_t2__blk1033_dn0 = assign33160_e48294_d_n0;
        var_t2__blk1033_dn2 = assign33160_e48294_d_n2;
        var_t2__blk1033_dn6 = assign33160_e48294_d_n6;
        var_t2__blk1033_dn7 = assign33160_e48294_d_n7;
        var_t2__blk1033_dn10 = assign33160_e48294_d_n10;
        var_t2__blk1033_dn11 = assign33160_e48294_d_n11;
        var_t2__blk1033_dn12 = assign33160_e48294_d_n12;
        var_t2__blk1033_dn17 = assign33160_e48294_d_n17;

        let (assign33170_e48312, assign33170_e48312_d_n0, assign33170_e48312_d_n2, assign33170_e48312_d_n6, assign33170_e48312_d_n7, assign33170_e48312_d_n10, assign33170_e48312_d_n11, assign33170_e48312_d_n12, assign33170_e48312_d_n17,) = {
    if (((var_guard1030 != 0.0) && (var_guard1075 == 0.0)) && (var_guard1083 == 0.0)) {
        let assign33170_e48306: f64 = (var_vbdj * 0.5);
        let assign33170_e48308: f64 = (assign33170_e48306 * var_t2__blk1033);
        let assign33170_e48309: f64 = (var_t1__blk1032 + assign33170_e48308);
        let assign33170_e48310: f64 = (var_vbdj * assign33170_e48309);
        (assign33170_e48310, (var_vbdj * (assign33170_e48306 * var_t2__blk1033_dn0)), (var_vbdj * (assign33170_e48306 * var_t2__blk1033_dn2)), ((var_vbdj_dn6 * assign33170_e48309) + (var_vbdj * (var_t1__blk1032_dn6 + (((var_vbdj_dn6 * 0.5) * var_t2__blk1033) + (assign33170_e48306 * var_t2__blk1033_dn6))))), (var_vbdj * (var_t1__blk1032_dn7 + (assign33170_e48306 * var_t2__blk1033_dn7))), (var_vbdj * (var_t1__blk1032_dn10 + (assign33170_e48306 * var_t2__blk1033_dn10))), (var_vbdj * (assign33170_e48306 * var_t2__blk1033_dn11)), ((var_vbdj_dn12 * assign33170_e48309) + (var_vbdj * (var_t1__blk1032_dn12 + (((var_vbdj_dn12 * 0.5) * var_t2__blk1033) + (assign33170_e48306 * var_t2__blk1033_dn12))))), (var_vbdj * (assign33170_e48306 * var_t2__blk1033_dn17)),)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33170_e48312;
        var_qbd_dn0 = assign33170_e48312_d_n0;
        var_qbd_dn2 = assign33170_e48312_d_n2;
        var_qbd_dn6 = assign33170_e48312_d_n6;
        var_qbd_dn7 = assign33170_e48312_d_n7;
        var_qbd_dn10 = assign33170_e48312_d_n10;
        var_qbd_dn11 = assign33170_e48312_d_n11;
        var_qbd_dn12 = assign33170_e48312_d_n12;
        var_qbd_dn17 = assign33170_e48312_d_n17;

        let assign33180_e48315: f64 = if var_czbs > 0.0 { 1.0 } else { 0.0 };
        var_guard1088 = assign33180_e48315;

        let (assign33190_e48328, assign33190_e48328_d_n0, assign33190_e48328_d_n2, assign33190_e48328_d_n6, assign33190_e48328_d_n7, assign33190_e48328_d_n10, assign33190_e48328_d_n11, assign33190_e48328_d_n12, assign33190_e48328_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1088 != 0.0)) {
        let assign33190_e48320: f64 = (-1.6021918e-19);
        let assign33190_e48322: f64 = (assign33190_e48320 * var_uc_nsubs);
        let assign33190_e48324: f64 = (assign33190_e48322 * var_xp_max);
        let assign33190_e48326: f64 = (assign33190_e48324 * p.p3);
        (assign33190_e48326, (((assign33190_e48320 * var_uc_nsubs_dn0) * var_xp_max) * p.p3), (((assign33190_e48320 * var_uc_nsubs_dn2) * var_xp_max) * p.p3), (((assign33190_e48320 * var_uc_nsubs_dn6) * var_xp_max) * p.p3), (((assign33190_e48320 * var_uc_nsubs_dn7) * var_xp_max) * p.p3), (((assign33190_e48320 * var_uc_nsubs_dn10) * var_xp_max) * p.p3), (((assign33190_e48320 * var_uc_nsubs_dn11) * var_xp_max) * p.p3), (((assign33190_e48320 * var_uc_nsubs_dn12) * var_xp_max) * p.p3), (((assign33190_e48320 * var_uc_nsubs_dn17) * var_xp_max) * p.p3),)
    } else {
        (var_qbs_max, var_qbs_max_dn0, var_qbs_max_dn2, var_qbs_max_dn6, var_qbs_max_dn7, var_qbs_max_dn10, var_qbs_max_dn11, var_qbs_max_dn12, var_qbs_max_dn17,)
    }
};
        var_qbs_max = assign33190_e48328;
        var_qbs_max_dn0 = assign33190_e48328_d_n0;
        var_qbs_max_dn2 = assign33190_e48328_d_n2;
        var_qbs_max_dn6 = assign33190_e48328_d_n6;
        var_qbs_max_dn7 = assign33190_e48328_d_n7;
        var_qbs_max_dn10 = assign33190_e48328_d_n10;
        var_qbs_max_dn11 = assign33190_e48328_d_n11;
        var_qbs_max_dn12 = assign33190_e48328_d_n12;
        var_qbs_max_dn17 = assign33190_e48328_d_n17;

        let (assign33200_e48337, assign33200_e48337_d_n0, assign33200_e48337_d_n2, assign33200_e48337_d_n6, assign33200_e48337_d_n7, assign33200_e48337_d_n10, assign33200_e48337_d_n11, assign33200_e48337_d_n12, assign33200_e48337_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1088 != 0.0)) {
        let assign33200_e48334: f64 = (-var_qbs_max);
        let assign33200_e48335: f64 = (0.001 * assign33200_e48334);
        (assign33200_e48335, (0.001 * (-var_qbs_max_dn0)), (0.001 * (-var_qbs_max_dn2)), (0.001 * (-var_qbs_max_dn6)), (0.001 * (-var_qbs_max_dn7)), (0.001 * (-var_qbs_max_dn10)), (0.001 * (-var_qbs_max_dn11)), (0.001 * (-var_qbs_max_dn12)), (0.001 * (-var_qbs_max_dn17)),)
    } else {
        (var_dlt_qbs, var_dlt_qbs_dn0, var_dlt_qbs_dn2, var_dlt_qbs_dn6, var_dlt_qbs_dn7, var_dlt_qbs_dn10, var_dlt_qbs_dn11, var_dlt_qbs_dn12, var_dlt_qbs_dn17,)
    }
};
        var_dlt_qbs = assign33200_e48337;
        var_dlt_qbs_dn0 = assign33200_e48337_d_n0;
        var_dlt_qbs_dn2 = assign33200_e48337_d_n2;
        var_dlt_qbs_dn6 = assign33200_e48337_d_n6;
        var_dlt_qbs_dn7 = assign33200_e48337_d_n7;
        var_dlt_qbs_dn10 = assign33200_e48337_d_n10;
        var_dlt_qbs_dn11 = assign33200_e48337_d_n11;
        var_dlt_qbs_dn12 = assign33200_e48337_d_n12;
        var_dlt_qbs_dn17 = assign33200_e48337_d_n17;

        let (assign33210_e48349, assign33210_e48349_d_n0, assign33210_e48349_d_n2, assign33210_e48349_d_n6, assign33210_e48349_d_n7, assign33210_e48349_d_n10, assign33210_e48349_d_n11, assign33210_e48349_d_n12, assign33210_e48349_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1088 != 0.0)) {
        let assign33210_e48342: f64 = (-var_qbs_max);
        let assign33210_e48344: f64 = (-var_qbs);
        let assign33210_e48345: f64 = (assign33210_e48342 - assign33210_e48344);
        let assign33210_e48347: f64 = (assign33210_e48345 - var_dlt_qbs);
        (assign33210_e48347, (((-var_qbs_max_dn0) - (-var_qbs_dn0)) - var_dlt_qbs_dn0), (((-var_qbs_max_dn2) - (-var_qbs_dn2)) - var_dlt_qbs_dn2), (((-var_qbs_max_dn6) - (-var_qbs_dn6)) - var_dlt_qbs_dn6), (((-var_qbs_max_dn7) - (-var_qbs_dn7)) - var_dlt_qbs_dn7), (((-var_qbs_max_dn10) - (-var_qbs_dn10)) - var_dlt_qbs_dn10), (((-var_qbs_max_dn11) - (-var_qbs_dn11)) - var_dlt_qbs_dn11), (((-var_qbs_max_dn12) - (-var_qbs_dn12)) - var_dlt_qbs_dn12), (((-var_qbs_max_dn17) - (-var_qbs_dn17)) - var_dlt_qbs_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign33210_e48349;
        var_tmf1_dn0 = assign33210_e48349_d_n0;
        var_tmf1_dn2 = assign33210_e48349_d_n2;
        var_tmf1_dn6 = assign33210_e48349_d_n6;
        var_tmf1_dn7 = assign33210_e48349_d_n7;
        var_tmf1_dn10 = assign33210_e48349_d_n10;
        var_tmf1_dn11 = assign33210_e48349_d_n11;
        var_tmf1_dn12 = assign33210_e48349_d_n12;
        var_tmf1_dn17 = assign33210_e48349_d_n17;

        let (assign33220_e48360, assign33220_e48360_d_n0, assign33220_e48360_d_n2, assign33220_e48360_d_n6, assign33220_e48360_d_n7, assign33220_e48360_d_n10, assign33220_e48360_d_n11, assign33220_e48360_d_n12, assign33220_e48360_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1088 != 0.0)) {
        let assign33220_e48355: f64 = (-var_qbs_max);
        let assign33220_e48356: f64 = (4.0 * assign33220_e48355);
        let assign33220_e48358: f64 = (assign33220_e48356 * var_dlt_qbs);
        (assign33220_e48358, (((4.0 * (-var_qbs_max_dn0)) * var_dlt_qbs) + (assign33220_e48356 * var_dlt_qbs_dn0)), (((4.0 * (-var_qbs_max_dn2)) * var_dlt_qbs) + (assign33220_e48356 * var_dlt_qbs_dn2)), (((4.0 * (-var_qbs_max_dn6)) * var_dlt_qbs) + (assign33220_e48356 * var_dlt_qbs_dn6)), (((4.0 * (-var_qbs_max_dn7)) * var_dlt_qbs) + (assign33220_e48356 * var_dlt_qbs_dn7)), (((4.0 * (-var_qbs_max_dn10)) * var_dlt_qbs) + (assign33220_e48356 * var_dlt_qbs_dn10)), (((4.0 * (-var_qbs_max_dn11)) * var_dlt_qbs) + (assign33220_e48356 * var_dlt_qbs_dn11)), (((4.0 * (-var_qbs_max_dn12)) * var_dlt_qbs) + (assign33220_e48356 * var_dlt_qbs_dn12)), (((4.0 * (-var_qbs_max_dn17)) * var_dlt_qbs) + (assign33220_e48356 * var_dlt_qbs_dn17)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign33220_e48360;
        var_tmf2_dn0 = assign33220_e48360_d_n0;
        var_tmf2_dn2 = assign33220_e48360_d_n2;
        var_tmf2_dn6 = assign33220_e48360_d_n6;
        var_tmf2_dn7 = assign33220_e48360_d_n7;
        var_tmf2_dn10 = assign33220_e48360_d_n10;
        var_tmf2_dn11 = assign33220_e48360_d_n11;
        var_tmf2_dn12 = assign33220_e48360_d_n12;
        var_tmf2_dn17 = assign33220_e48360_d_n17;

        let (assign33230_e48372, assign33230_e48372_d_n0, assign33230_e48372_d_n2, assign33230_e48372_d_n6, assign33230_e48372_d_n7, assign33230_e48372_d_n10, assign33230_e48372_d_n11, assign33230_e48372_d_n12, assign33230_e48372_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1088 != 0.0)) {
        let (assign33230_e48370, assign33230_e48370_d_n0, assign33230_e48370_d_n2, assign33230_e48370_d_n6, assign33230_e48370_d_n7, assign33230_e48370_d_n10, assign33230_e48370_d_n11, assign33230_e48370_d_n12, assign33230_e48370_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign33230_e48369: f64 = (-var_tmf2);
                (assign33230_e48369, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign33230_e48370, assign33230_e48370_d_n0, assign33230_e48370_d_n2, assign33230_e48370_d_n6, assign33230_e48370_d_n7, assign33230_e48370_d_n10, assign33230_e48370_d_n11, assign33230_e48370_d_n12, assign33230_e48370_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign33230_e48372;
        var_tmf2_dn0 = assign33230_e48372_d_n0;
        var_tmf2_dn2 = assign33230_e48372_d_n2;
        var_tmf2_dn6 = assign33230_e48372_d_n6;
        var_tmf2_dn7 = assign33230_e48372_d_n7;
        var_tmf2_dn10 = assign33230_e48372_d_n10;
        var_tmf2_dn11 = assign33230_e48372_d_n11;
        var_tmf2_dn12 = assign33230_e48372_d_n12;
        var_tmf2_dn17 = assign33230_e48372_d_n17;

        *var_arg__blk1057_slot = var_arg__blk1057;
        *var_arg__blk1057_dn12_slot = var_arg__blk1057_dn12;
        *var_arg__blk1057_dn6_slot = var_arg__blk1057_dn6;
        *var_arg__blk1057_dn7_slot = var_arg__blk1057_dn7;
        *var_czbdswg_slot = var_czbdswg;
        *var_dlt_qbs_slot = var_dlt_qbs;
        *var_dlt_qbs_dn0_slot = var_dlt_qbs_dn0;
        *var_dlt_qbs_dn10_slot = var_dlt_qbs_dn10;
        *var_dlt_qbs_dn11_slot = var_dlt_qbs_dn11;
        *var_dlt_qbs_dn12_slot = var_dlt_qbs_dn12;
        *var_dlt_qbs_dn17_slot = var_dlt_qbs_dn17;
        *var_dlt_qbs_dn2_slot = var_dlt_qbs_dn2;
        *var_dlt_qbs_dn6_slot = var_dlt_qbs_dn6;
        *var_dlt_qbs_dn7_slot = var_dlt_qbs_dn7;
        *var_guard1081_slot = var_guard1081;
        *var_guard1082_slot = var_guard1082;
        *var_guard1083_slot = var_guard1083;
        *var_guard1084_slot = var_guard1084;
        *var_guard1085_slot = var_guard1085;
        *var_guard1086_slot = var_guard1086;
        *var_guard1087_slot = var_guard1087;
        *var_guard1088_slot = var_guard1088;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn0_slot = var_qbd_dn0;
        *var_qbd_dn10_slot = var_qbd_dn10;
        *var_qbd_dn11_slot = var_qbd_dn11;
        *var_qbd_dn12_slot = var_qbd_dn12;
        *var_qbd_dn17_slot = var_qbd_dn17;
        *var_qbd_dn2_slot = var_qbd_dn2;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbs_max_slot = var_qbs_max;
        *var_qbs_max_dn0_slot = var_qbs_max_dn0;
        *var_qbs_max_dn10_slot = var_qbs_max_dn10;
        *var_qbs_max_dn11_slot = var_qbs_max_dn11;
        *var_qbs_max_dn12_slot = var_qbs_max_dn12;
        *var_qbs_max_dn17_slot = var_qbs_max_dn17;
        *var_qbs_max_dn2_slot = var_qbs_max_dn2;
        *var_qbs_max_dn6_slot = var_qbs_max_dn6;
        *var_qbs_max_dn7_slot = var_qbs_max_dn7;
        *var_sarg_slot = var_sarg;
        *var_sarg_dn12_slot = var_sarg_dn12;
        *var_sarg_dn6_slot = var_sarg_dn6;
        *var_sarg_dn7_slot = var_sarg_dn7;
        *var_t1__blk1032_slot = var_t1__blk1032;
        *var_t1__blk1032_dn10_slot = var_t1__blk1032_dn10;
        *var_t1__blk1032_dn12_slot = var_t1__blk1032_dn12;
        *var_t1__blk1032_dn6_slot = var_t1__blk1032_dn6;
        *var_t1__blk1032_dn7_slot = var_t1__blk1032_dn7;
        *var_t2__blk1033_slot = var_t2__blk1033;
        *var_t2__blk1033_dn0_slot = var_t2__blk1033_dn0;
        *var_t2__blk1033_dn10_slot = var_t2__blk1033_dn10;
        *var_t2__blk1033_dn11_slot = var_t2__blk1033_dn11;
        *var_t2__blk1033_dn12_slot = var_t2__blk1033_dn12;
        *var_t2__blk1033_dn17_slot = var_t2__blk1033_dn17;
        *var_t2__blk1033_dn2_slot = var_t2__blk1033_dn2;
        *var_t2__blk1033_dn6_slot = var_t2__blk1033_dn6;
        *var_t2__blk1033_dn7_slot = var_t2__blk1033_dn7;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
    }

    pub(super) fn stamp_transient_block_117(
        p: &Parameters,
        var_c_fox: f64,
        var_c_fox_dn0: f64,
        var_c_fox_dn10: f64,
        var_c_fox_dn11: f64,
        var_c_fox_dn12: f64,
        var_c_fox_dn17: f64,
        var_c_fox_dn2: f64,
        var_c_fox_dn6: f64,
        var_c_fox_dn7: f64,
        var_czbd: f64,
        var_flg_noqi: f64,
        var_flg_nqs: f64,
        var_guard1030: f64,
        var_guard1088: f64,
        var_lch: f64,
        var_lch_dn0: f64,
        var_lch_dn10: f64,
        var_lch_dn11: f64,
        var_lch_dn12: f64,
        var_lch_dn17: f64,
        var_lch_dn2: f64,
        var_lch_dn6: f64,
        var_lch_dn7: f64,
        var_mu: f64,
        var_mu_dn0: f64,
        var_mu_dn10: f64,
        var_mu_dn11: f64,
        var_mu_dn12: f64,
        var_mu_dn17: f64,
        var_mu_dn2: f64,
        var_mu_dn6: f64,
        var_mu_dn7: f64,
        var_muun: f64,
        var_muun_dn0: f64,
        var_muun_dn10: f64,
        var_muun_dn11: f64,
        var_muun_dn12: f64,
        var_muun_dn17: f64,
        var_muun_dn2: f64,
        var_muun_dn6: f64,
        var_muun_dn7: f64,
        var_ps0: f64,
        var_ps0_dn0: f64,
        var_ps0_dn10: f64,
        var_ps0_dn11: f64,
        var_ps0_dn12: f64,
        var_ps0_dn17: f64,
        var_ps0_dn2: f64,
        var_ps0_dn6: f64,
        var_ps0_dn7: f64,
        var_psdl: f64,
        var_psdl_dn0: f64,
        var_psdl_dn10: f64,
        var_psdl_dn11: f64,
        var_psdl_dn12: f64,
        var_psdl_dn17: f64,
        var_psdl_dn2: f64,
        var_psdl_dn6: f64,
        var_psdl_dn7: f64,
        var_qbs_max: f64,
        var_qbs_max_dn0: f64,
        var_qbs_max_dn10: f64,
        var_qbs_max_dn11: f64,
        var_qbs_max_dn12: f64,
        var_qbs_max_dn17: f64,
        var_qbs_max_dn2: f64,
        var_qbs_max_dn6: f64,
        var_qbs_max_dn7: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn17: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn7: f64,
        var_vgvt: f64,
        var_vgvt_dn0: f64,
        var_vgvt_dn10: f64,
        var_vgvt_dn11: f64,
        var_vgvt_dn12: f64,
        var_vgvt_dn17: f64,
        var_vgvt_dn2: f64,
        var_vgvt_dn6: f64,
        var_vgvt_dn7: f64,
        var_xp_max: f64,
        var_dlt_qbd_slot: &mut f64,
        var_dlt_qbd_dn0_slot: &mut f64,
        var_dlt_qbd_dn10_slot: &mut f64,
        var_dlt_qbd_dn11_slot: &mut f64,
        var_dlt_qbd_dn12_slot: &mut f64,
        var_dlt_qbd_dn17_slot: &mut f64,
        var_dlt_qbd_dn2_slot: &mut f64,
        var_dlt_qbd_dn6_slot: &mut f64,
        var_dlt_qbd_dn7_slot: &mut f64,
        var_eyd_slot: &mut f64,
        var_eyd_dn0_slot: &mut f64,
        var_eyd_dn10_slot: &mut f64,
        var_eyd_dn11_slot: &mut f64,
        var_eyd_dn12_slot: &mut f64,
        var_eyd_dn17_slot: &mut f64,
        var_eyd_dn2_slot: &mut f64,
        var_eyd_dn6_slot: &mut f64,
        var_eyd_dn7_slot: &mut f64,
        var_guard1089_slot: &mut f64,
        var_guard1095_slot: &mut f64,
        var_guard1122_slot: &mut f64,
        var_guard1123_slot: &mut f64,
        var_guard1124_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn0_slot: &mut f64,
        var_qbd_dn10_slot: &mut f64,
        var_qbd_dn11_slot: &mut f64,
        var_qbd_dn12_slot: &mut f64,
        var_qbd_dn17_slot: &mut f64,
        var_qbd_dn2_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbd_max_slot: &mut f64,
        var_qbd_max_dn0_slot: &mut f64,
        var_qbd_max_dn10_slot: &mut f64,
        var_qbd_max_dn11_slot: &mut f64,
        var_qbd_max_dn12_slot: &mut f64,
        var_qbd_max_dn17_slot: &mut f64,
        var_qbd_max_dn2_slot: &mut f64,
        var_qbd_max_dn6_slot: &mut f64,
        var_qbd_max_dn7_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn0_slot: &mut f64,
        var_qbs_dn10_slot: &mut f64,
        var_qbs_dn11_slot: &mut f64,
        var_qbs_dn12_slot: &mut f64,
        var_qbs_dn17_slot: &mut f64,
        var_qbs_dn2_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_t10__blk1090_slot: &mut f64,
        var_t11__blk1091_slot: &mut f64,
        var_t12_slot: &mut f64,
        var_t12__blk1106_slot: &mut f64,
        var_t12__blk1106_dn0_slot: &mut f64,
        var_t12__blk1106_dn10_slot: &mut f64,
        var_t12__blk1106_dn11_slot: &mut f64,
        var_t12__blk1106_dn12_slot: &mut f64,
        var_t12__blk1106_dn17_slot: &mut f64,
        var_t12__blk1106_dn2_slot: &mut f64,
        var_t12__blk1106_dn6_slot: &mut f64,
        var_t12__blk1106_dn7_slot: &mut f64,
        var_t12_dn0_slot: &mut f64,
        var_t12_dn10_slot: &mut f64,
        var_t12_dn11_slot: &mut f64,
        var_t12_dn12_slot: &mut f64,
        var_t12_dn17_slot: &mut f64,
        var_t12_dn2_slot: &mut f64,
        var_t12_dn6_slot: &mut f64,
        var_t12_dn7_slot: &mut f64,
        var_t1__blk1093_slot: &mut f64,
        var_t1__blk1093_dn0_slot: &mut f64,
        var_t1__blk1093_dn10_slot: &mut f64,
        var_t1__blk1093_dn11_slot: &mut f64,
        var_t1__blk1093_dn12_slot: &mut f64,
        var_t1__blk1093_dn17_slot: &mut f64,
        var_t1__blk1093_dn2_slot: &mut f64,
        var_t1__blk1093_dn6_slot: &mut f64,
        var_t1__blk1093_dn7_slot: &mut f64,
        var_t2__blk1094_slot: &mut f64,
        var_t2__blk1094_dn0_slot: &mut f64,
        var_t2__blk1094_dn10_slot: &mut f64,
        var_t2__blk1094_dn11_slot: &mut f64,
        var_t2__blk1094_dn12_slot: &mut f64,
        var_t2__blk1094_dn17_slot: &mut f64,
        var_t2__blk1094_dn2_slot: &mut f64,
        var_t2__blk1094_dn6_slot: &mut f64,
        var_t2__blk1094_dn7_slot: &mut f64,
        var_t7__blk1107_slot: &mut f64,
        var_t7__blk1107_dn0_slot: &mut f64,
        var_t7__blk1107_dn10_slot: &mut f64,
        var_t7__blk1107_dn11_slot: &mut f64,
        var_t7__blk1107_dn12_slot: &mut f64,
        var_t7__blk1107_dn17_slot: &mut f64,
        var_t7__blk1107_dn2_slot: &mut f64,
        var_t7__blk1107_dn6_slot: &mut f64,
        var_t7__blk1107_dn7_slot: &mut f64,
        var_t8__blk1108_slot: &mut f64,
        var_t8__blk1108_dn0_slot: &mut f64,
        var_t8__blk1108_dn10_slot: &mut f64,
        var_t8__blk1108_dn11_slot: &mut f64,
        var_t8__blk1108_dn12_slot: &mut f64,
        var_t8__blk1108_dn17_slot: &mut f64,
        var_t8__blk1108_dn2_slot: &mut f64,
        var_t8__blk1108_dn6_slot: &mut f64,
        var_t8__blk1108_dn7_slot: &mut f64,
        var_tau_slot: &mut f64,
        var_tau_dn0_slot: &mut f64,
        var_tau_dn10_slot: &mut f64,
        var_tau_dn11_slot: &mut f64,
        var_tau_dn12_slot: &mut f64,
        var_tau_dn17_slot: &mut f64,
        var_tau_dn2_slot: &mut f64,
        var_tau_dn6_slot: &mut f64,
        var_tau_dn7_slot: &mut f64,
        var_taub_slot: &mut f64,
        var_taub_dn0_slot: &mut f64,
        var_taub_dn10_slot: &mut f64,
        var_taub_dn11_slot: &mut f64,
        var_taub_dn12_slot: &mut f64,
        var_taub_dn17_slot: &mut f64,
        var_taub_dn2_slot: &mut f64,
        var_taub_dn6_slot: &mut f64,
        var_taub_dn7_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
    ) {
        let mut var_dlt_qbd: f64 = *var_dlt_qbd_slot;
        let mut var_dlt_qbd_dn0: f64 = *var_dlt_qbd_dn0_slot;
        let mut var_dlt_qbd_dn10: f64 = *var_dlt_qbd_dn10_slot;
        let mut var_dlt_qbd_dn11: f64 = *var_dlt_qbd_dn11_slot;
        let mut var_dlt_qbd_dn12: f64 = *var_dlt_qbd_dn12_slot;
        let mut var_dlt_qbd_dn17: f64 = *var_dlt_qbd_dn17_slot;
        let mut var_dlt_qbd_dn2: f64 = *var_dlt_qbd_dn2_slot;
        let mut var_dlt_qbd_dn6: f64 = *var_dlt_qbd_dn6_slot;
        let mut var_dlt_qbd_dn7: f64 = *var_dlt_qbd_dn7_slot;
        let mut var_eyd: f64 = *var_eyd_slot;
        let mut var_eyd_dn0: f64 = *var_eyd_dn0_slot;
        let mut var_eyd_dn10: f64 = *var_eyd_dn10_slot;
        let mut var_eyd_dn11: f64 = *var_eyd_dn11_slot;
        let mut var_eyd_dn12: f64 = *var_eyd_dn12_slot;
        let mut var_eyd_dn17: f64 = *var_eyd_dn17_slot;
        let mut var_eyd_dn2: f64 = *var_eyd_dn2_slot;
        let mut var_eyd_dn6: f64 = *var_eyd_dn6_slot;
        let mut var_eyd_dn7: f64 = *var_eyd_dn7_slot;
        let mut var_guard1089: f64 = *var_guard1089_slot;
        let mut var_guard1095: f64 = *var_guard1095_slot;
        let mut var_guard1122: f64 = *var_guard1122_slot;
        let mut var_guard1123: f64 = *var_guard1123_slot;
        let mut var_guard1124: f64 = *var_guard1124_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn0: f64 = *var_qbd_dn0_slot;
        let mut var_qbd_dn10: f64 = *var_qbd_dn10_slot;
        let mut var_qbd_dn11: f64 = *var_qbd_dn11_slot;
        let mut var_qbd_dn12: f64 = *var_qbd_dn12_slot;
        let mut var_qbd_dn17: f64 = *var_qbd_dn17_slot;
        let mut var_qbd_dn2: f64 = *var_qbd_dn2_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbd_max: f64 = *var_qbd_max_slot;
        let mut var_qbd_max_dn0: f64 = *var_qbd_max_dn0_slot;
        let mut var_qbd_max_dn10: f64 = *var_qbd_max_dn10_slot;
        let mut var_qbd_max_dn11: f64 = *var_qbd_max_dn11_slot;
        let mut var_qbd_max_dn12: f64 = *var_qbd_max_dn12_slot;
        let mut var_qbd_max_dn17: f64 = *var_qbd_max_dn17_slot;
        let mut var_qbd_max_dn2: f64 = *var_qbd_max_dn2_slot;
        let mut var_qbd_max_dn6: f64 = *var_qbd_max_dn6_slot;
        let mut var_qbd_max_dn7: f64 = *var_qbd_max_dn7_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn0: f64 = *var_qbs_dn0_slot;
        let mut var_qbs_dn10: f64 = *var_qbs_dn10_slot;
        let mut var_qbs_dn11: f64 = *var_qbs_dn11_slot;
        let mut var_qbs_dn12: f64 = *var_qbs_dn12_slot;
        let mut var_qbs_dn17: f64 = *var_qbs_dn17_slot;
        let mut var_qbs_dn2: f64 = *var_qbs_dn2_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_t10__blk1090: f64 = *var_t10__blk1090_slot;
        let mut var_t11__blk1091: f64 = *var_t11__blk1091_slot;
        let mut var_t12: f64 = *var_t12_slot;
        let mut var_t12__blk1106: f64 = *var_t12__blk1106_slot;
        let mut var_t12__blk1106_dn0: f64 = *var_t12__blk1106_dn0_slot;
        let mut var_t12__blk1106_dn10: f64 = *var_t12__blk1106_dn10_slot;
        let mut var_t12__blk1106_dn11: f64 = *var_t12__blk1106_dn11_slot;
        let mut var_t12__blk1106_dn12: f64 = *var_t12__blk1106_dn12_slot;
        let mut var_t12__blk1106_dn17: f64 = *var_t12__blk1106_dn17_slot;
        let mut var_t12__blk1106_dn2: f64 = *var_t12__blk1106_dn2_slot;
        let mut var_t12__blk1106_dn6: f64 = *var_t12__blk1106_dn6_slot;
        let mut var_t12__blk1106_dn7: f64 = *var_t12__blk1106_dn7_slot;
        let mut var_t12_dn0: f64 = *var_t12_dn0_slot;
        let mut var_t12_dn10: f64 = *var_t12_dn10_slot;
        let mut var_t12_dn11: f64 = *var_t12_dn11_slot;
        let mut var_t12_dn12: f64 = *var_t12_dn12_slot;
        let mut var_t12_dn17: f64 = *var_t12_dn17_slot;
        let mut var_t12_dn2: f64 = *var_t12_dn2_slot;
        let mut var_t12_dn6: f64 = *var_t12_dn6_slot;
        let mut var_t12_dn7: f64 = *var_t12_dn7_slot;
        let mut var_t1__blk1093: f64 = *var_t1__blk1093_slot;
        let mut var_t1__blk1093_dn0: f64 = *var_t1__blk1093_dn0_slot;
        let mut var_t1__blk1093_dn10: f64 = *var_t1__blk1093_dn10_slot;
        let mut var_t1__blk1093_dn11: f64 = *var_t1__blk1093_dn11_slot;
        let mut var_t1__blk1093_dn12: f64 = *var_t1__blk1093_dn12_slot;
        let mut var_t1__blk1093_dn17: f64 = *var_t1__blk1093_dn17_slot;
        let mut var_t1__blk1093_dn2: f64 = *var_t1__blk1093_dn2_slot;
        let mut var_t1__blk1093_dn6: f64 = *var_t1__blk1093_dn6_slot;
        let mut var_t1__blk1093_dn7: f64 = *var_t1__blk1093_dn7_slot;
        let mut var_t2__blk1094: f64 = *var_t2__blk1094_slot;
        let mut var_t2__blk1094_dn0: f64 = *var_t2__blk1094_dn0_slot;
        let mut var_t2__blk1094_dn10: f64 = *var_t2__blk1094_dn10_slot;
        let mut var_t2__blk1094_dn11: f64 = *var_t2__blk1094_dn11_slot;
        let mut var_t2__blk1094_dn12: f64 = *var_t2__blk1094_dn12_slot;
        let mut var_t2__blk1094_dn17: f64 = *var_t2__blk1094_dn17_slot;
        let mut var_t2__blk1094_dn2: f64 = *var_t2__blk1094_dn2_slot;
        let mut var_t2__blk1094_dn6: f64 = *var_t2__blk1094_dn6_slot;
        let mut var_t2__blk1094_dn7: f64 = *var_t2__blk1094_dn7_slot;
        let mut var_t7__blk1107: f64 = *var_t7__blk1107_slot;
        let mut var_t7__blk1107_dn0: f64 = *var_t7__blk1107_dn0_slot;
        let mut var_t7__blk1107_dn10: f64 = *var_t7__blk1107_dn10_slot;
        let mut var_t7__blk1107_dn11: f64 = *var_t7__blk1107_dn11_slot;
        let mut var_t7__blk1107_dn12: f64 = *var_t7__blk1107_dn12_slot;
        let mut var_t7__blk1107_dn17: f64 = *var_t7__blk1107_dn17_slot;
        let mut var_t7__blk1107_dn2: f64 = *var_t7__blk1107_dn2_slot;
        let mut var_t7__blk1107_dn6: f64 = *var_t7__blk1107_dn6_slot;
        let mut var_t7__blk1107_dn7: f64 = *var_t7__blk1107_dn7_slot;
        let mut var_t8__blk1108: f64 = *var_t8__blk1108_slot;
        let mut var_t8__blk1108_dn0: f64 = *var_t8__blk1108_dn0_slot;
        let mut var_t8__blk1108_dn10: f64 = *var_t8__blk1108_dn10_slot;
        let mut var_t8__blk1108_dn11: f64 = *var_t8__blk1108_dn11_slot;
        let mut var_t8__blk1108_dn12: f64 = *var_t8__blk1108_dn12_slot;
        let mut var_t8__blk1108_dn17: f64 = *var_t8__blk1108_dn17_slot;
        let mut var_t8__blk1108_dn2: f64 = *var_t8__blk1108_dn2_slot;
        let mut var_t8__blk1108_dn6: f64 = *var_t8__blk1108_dn6_slot;
        let mut var_t8__blk1108_dn7: f64 = *var_t8__blk1108_dn7_slot;
        let mut var_tau: f64 = *var_tau_slot;
        let mut var_tau_dn0: f64 = *var_tau_dn0_slot;
        let mut var_tau_dn10: f64 = *var_tau_dn10_slot;
        let mut var_tau_dn11: f64 = *var_tau_dn11_slot;
        let mut var_tau_dn12: f64 = *var_tau_dn12_slot;
        let mut var_tau_dn17: f64 = *var_tau_dn17_slot;
        let mut var_tau_dn2: f64 = *var_tau_dn2_slot;
        let mut var_tau_dn6: f64 = *var_tau_dn6_slot;
        let mut var_tau_dn7: f64 = *var_tau_dn7_slot;
        let mut var_taub: f64 = *var_taub_slot;
        let mut var_taub_dn0: f64 = *var_taub_dn0_slot;
        let mut var_taub_dn10: f64 = *var_taub_dn10_slot;
        let mut var_taub_dn11: f64 = *var_taub_dn11_slot;
        let mut var_taub_dn12: f64 = *var_taub_dn12_slot;
        let mut var_taub_dn17: f64 = *var_taub_dn17_slot;
        let mut var_taub_dn2: f64 = *var_taub_dn2_slot;
        let mut var_taub_dn6: f64 = *var_taub_dn6_slot;
        let mut var_taub_dn7: f64 = *var_taub_dn7_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;

        let (assign33240_e48383, assign33240_e48383_d_n0, assign33240_e48383_d_n2, assign33240_e48383_d_n6, assign33240_e48383_d_n7, assign33240_e48383_d_n10, assign33240_e48383_d_n11, assign33240_e48383_d_n12, assign33240_e48383_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1088 != 0.0)) {
        let assign33240_e48378: f64 = (var_tmf1 * var_tmf1);
        let assign33240_e48380: f64 = (assign33240_e48378 + var_tmf2);
        let assign33240_e48381: f64 = (assign33240_e48380).sqrt();
        (assign33240_e48381, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign33240_e48381)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign33240_e48381)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign33240_e48381)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign33240_e48381)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign33240_e48381)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign33240_e48381)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign33240_e48381)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign33240_e48381)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign33240_e48383;
        var_tmf2_dn0 = assign33240_e48383_d_n0;
        var_tmf2_dn2 = assign33240_e48383_d_n2;
        var_tmf2_dn6 = assign33240_e48383_d_n6;
        var_tmf2_dn7 = assign33240_e48383_d_n7;
        var_tmf2_dn10 = assign33240_e48383_d_n10;
        var_tmf2_dn11 = assign33240_e48383_d_n11;
        var_tmf2_dn12 = assign33240_e48383_d_n12;
        var_tmf2_dn17 = assign33240_e48383_d_n17;

        let (assign33250_e48396, assign33250_e48396_d_n0, assign33250_e48396_d_n2, assign33250_e48396_d_n6, assign33250_e48396_d_n7, assign33250_e48396_d_n10, assign33250_e48396_d_n11, assign33250_e48396_d_n12, assign33250_e48396_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1088 != 0.0)) {
        let assign33250_e48388: f64 = (-var_qbs_max);
        let assign33250_e48392: f64 = (var_tmf1 + var_tmf2);
        let assign33250_e48393: f64 = (0.5 * assign33250_e48392);
        let assign33250_e48394: f64 = (assign33250_e48388 - assign33250_e48393);
        (assign33250_e48394, ((-var_qbs_max_dn0) - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), ((-var_qbs_max_dn2) - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), ((-var_qbs_max_dn6) - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), ((-var_qbs_max_dn7) - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), ((-var_qbs_max_dn10) - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), ((-var_qbs_max_dn11) - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), ((-var_qbs_max_dn12) - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), ((-var_qbs_max_dn17) - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign33250_e48396;
        var_qbs_dn0 = assign33250_e48396_d_n0;
        var_qbs_dn2 = assign33250_e48396_d_n2;
        var_qbs_dn6 = assign33250_e48396_d_n6;
        var_qbs_dn7 = assign33250_e48396_d_n7;
        var_qbs_dn10 = assign33250_e48396_d_n10;
        var_qbs_dn11 = assign33250_e48396_d_n11;
        var_qbs_dn12 = assign33250_e48396_d_n12;
        var_qbs_dn17 = assign33250_e48396_d_n17;

        let (assign33260_e48405, assign33260_e48405_d_n0, assign33260_e48405_d_n2, assign33260_e48405_d_n6, assign33260_e48405_d_n7, assign33260_e48405_d_n10, assign33260_e48405_d_n11, assign33260_e48405_d_n12, assign33260_e48405_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1088 != 0.0)) {
        let assign33260_e48402: f64 = (-1.0);
        let assign33260_e48403: f64 = (var_qbs * assign33260_e48402);
        (assign33260_e48403, (var_qbs_dn0 * assign33260_e48402), (var_qbs_dn2 * assign33260_e48402), (var_qbs_dn6 * assign33260_e48402), (var_qbs_dn7 * assign33260_e48402), (var_qbs_dn10 * assign33260_e48402), (var_qbs_dn11 * assign33260_e48402), (var_qbs_dn12 * assign33260_e48402), (var_qbs_dn17 * assign33260_e48402),)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign33260_e48405;
        var_qbs_dn0 = assign33260_e48405_d_n0;
        var_qbs_dn2 = assign33260_e48405_d_n2;
        var_qbs_dn6 = assign33260_e48405_d_n6;
        var_qbs_dn7 = assign33260_e48405_d_n7;
        var_qbs_dn10 = assign33260_e48405_d_n10;
        var_qbs_dn11 = assign33260_e48405_d_n11;
        var_qbs_dn12 = assign33260_e48405_d_n12;
        var_qbs_dn17 = assign33260_e48405_d_n17;

        let assign33270_e48408: f64 = if var_czbd > 0.0 { 1.0 } else { 0.0 };
        var_guard1089 = assign33270_e48408;

        let (assign33280_e48421, assign33280_e48421_d_n0, assign33280_e48421_d_n2, assign33280_e48421_d_n6, assign33280_e48421_d_n7, assign33280_e48421_d_n10, assign33280_e48421_d_n11, assign33280_e48421_d_n12, assign33280_e48421_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1089 != 0.0)) {
        let assign33280_e48413: f64 = (-1.6021918e-19);
        let assign33280_e48415: f64 = (assign33280_e48413 * var_uc_nsubs);
        let assign33280_e48417: f64 = (assign33280_e48415 * var_xp_max);
        let assign33280_e48419: f64 = (assign33280_e48417 * p.p2);
        (assign33280_e48419, (((assign33280_e48413 * var_uc_nsubs_dn0) * var_xp_max) * p.p2), (((assign33280_e48413 * var_uc_nsubs_dn2) * var_xp_max) * p.p2), (((assign33280_e48413 * var_uc_nsubs_dn6) * var_xp_max) * p.p2), (((assign33280_e48413 * var_uc_nsubs_dn7) * var_xp_max) * p.p2), (((assign33280_e48413 * var_uc_nsubs_dn10) * var_xp_max) * p.p2), (((assign33280_e48413 * var_uc_nsubs_dn11) * var_xp_max) * p.p2), (((assign33280_e48413 * var_uc_nsubs_dn12) * var_xp_max) * p.p2), (((assign33280_e48413 * var_uc_nsubs_dn17) * var_xp_max) * p.p2),)
    } else {
        (var_qbd_max, var_qbd_max_dn0, var_qbd_max_dn2, var_qbd_max_dn6, var_qbd_max_dn7, var_qbd_max_dn10, var_qbd_max_dn11, var_qbd_max_dn12, var_qbd_max_dn17,)
    }
};
        var_qbd_max = assign33280_e48421;
        var_qbd_max_dn0 = assign33280_e48421_d_n0;
        var_qbd_max_dn2 = assign33280_e48421_d_n2;
        var_qbd_max_dn6 = assign33280_e48421_d_n6;
        var_qbd_max_dn7 = assign33280_e48421_d_n7;
        var_qbd_max_dn10 = assign33280_e48421_d_n10;
        var_qbd_max_dn11 = assign33280_e48421_d_n11;
        var_qbd_max_dn12 = assign33280_e48421_d_n12;
        var_qbd_max_dn17 = assign33280_e48421_d_n17;

        let (assign33290_e48430, assign33290_e48430_d_n0, assign33290_e48430_d_n2, assign33290_e48430_d_n6, assign33290_e48430_d_n7, assign33290_e48430_d_n10, assign33290_e48430_d_n11, assign33290_e48430_d_n12, assign33290_e48430_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1089 != 0.0)) {
        let assign33290_e48427: f64 = (-var_qbd_max);
        let assign33290_e48428: f64 = (0.001 * assign33290_e48427);
        (assign33290_e48428, (0.001 * (-var_qbd_max_dn0)), (0.001 * (-var_qbd_max_dn2)), (0.001 * (-var_qbd_max_dn6)), (0.001 * (-var_qbd_max_dn7)), (0.001 * (-var_qbd_max_dn10)), (0.001 * (-var_qbd_max_dn11)), (0.001 * (-var_qbd_max_dn12)), (0.001 * (-var_qbd_max_dn17)),)
    } else {
        (var_dlt_qbd, var_dlt_qbd_dn0, var_dlt_qbd_dn2, var_dlt_qbd_dn6, var_dlt_qbd_dn7, var_dlt_qbd_dn10, var_dlt_qbd_dn11, var_dlt_qbd_dn12, var_dlt_qbd_dn17,)
    }
};
        var_dlt_qbd = assign33290_e48430;
        var_dlt_qbd_dn0 = assign33290_e48430_d_n0;
        var_dlt_qbd_dn2 = assign33290_e48430_d_n2;
        var_dlt_qbd_dn6 = assign33290_e48430_d_n6;
        var_dlt_qbd_dn7 = assign33290_e48430_d_n7;
        var_dlt_qbd_dn10 = assign33290_e48430_d_n10;
        var_dlt_qbd_dn11 = assign33290_e48430_d_n11;
        var_dlt_qbd_dn12 = assign33290_e48430_d_n12;
        var_dlt_qbd_dn17 = assign33290_e48430_d_n17;

        let (assign33300_e48442, assign33300_e48442_d_n0, assign33300_e48442_d_n2, assign33300_e48442_d_n6, assign33300_e48442_d_n7, assign33300_e48442_d_n10, assign33300_e48442_d_n11, assign33300_e48442_d_n12, assign33300_e48442_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1089 != 0.0)) {
        let assign33300_e48435: f64 = (-var_qbd_max);
        let assign33300_e48437: f64 = (-var_qbd);
        let assign33300_e48438: f64 = (assign33300_e48435 - assign33300_e48437);
        let assign33300_e48440: f64 = (assign33300_e48438 - var_dlt_qbd);
        (assign33300_e48440, (((-var_qbd_max_dn0) - (-var_qbd_dn0)) - var_dlt_qbd_dn0), (((-var_qbd_max_dn2) - (-var_qbd_dn2)) - var_dlt_qbd_dn2), (((-var_qbd_max_dn6) - (-var_qbd_dn6)) - var_dlt_qbd_dn6), (((-var_qbd_max_dn7) - (-var_qbd_dn7)) - var_dlt_qbd_dn7), (((-var_qbd_max_dn10) - (-var_qbd_dn10)) - var_dlt_qbd_dn10), (((-var_qbd_max_dn11) - (-var_qbd_dn11)) - var_dlt_qbd_dn11), (((-var_qbd_max_dn12) - (-var_qbd_dn12)) - var_dlt_qbd_dn12), (((-var_qbd_max_dn17) - (-var_qbd_dn17)) - var_dlt_qbd_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign33300_e48442;
        var_tmf1_dn0 = assign33300_e48442_d_n0;
        var_tmf1_dn2 = assign33300_e48442_d_n2;
        var_tmf1_dn6 = assign33300_e48442_d_n6;
        var_tmf1_dn7 = assign33300_e48442_d_n7;
        var_tmf1_dn10 = assign33300_e48442_d_n10;
        var_tmf1_dn11 = assign33300_e48442_d_n11;
        var_tmf1_dn12 = assign33300_e48442_d_n12;
        var_tmf1_dn17 = assign33300_e48442_d_n17;

        let (assign33310_e48453, assign33310_e48453_d_n0, assign33310_e48453_d_n2, assign33310_e48453_d_n6, assign33310_e48453_d_n7, assign33310_e48453_d_n10, assign33310_e48453_d_n11, assign33310_e48453_d_n12, assign33310_e48453_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1089 != 0.0)) {
        let assign33310_e48448: f64 = (-var_qbd_max);
        let assign33310_e48449: f64 = (4.0 * assign33310_e48448);
        let assign33310_e48451: f64 = (assign33310_e48449 * var_dlt_qbd);
        (assign33310_e48451, (((4.0 * (-var_qbd_max_dn0)) * var_dlt_qbd) + (assign33310_e48449 * var_dlt_qbd_dn0)), (((4.0 * (-var_qbd_max_dn2)) * var_dlt_qbd) + (assign33310_e48449 * var_dlt_qbd_dn2)), (((4.0 * (-var_qbd_max_dn6)) * var_dlt_qbd) + (assign33310_e48449 * var_dlt_qbd_dn6)), (((4.0 * (-var_qbd_max_dn7)) * var_dlt_qbd) + (assign33310_e48449 * var_dlt_qbd_dn7)), (((4.0 * (-var_qbd_max_dn10)) * var_dlt_qbd) + (assign33310_e48449 * var_dlt_qbd_dn10)), (((4.0 * (-var_qbd_max_dn11)) * var_dlt_qbd) + (assign33310_e48449 * var_dlt_qbd_dn11)), (((4.0 * (-var_qbd_max_dn12)) * var_dlt_qbd) + (assign33310_e48449 * var_dlt_qbd_dn12)), (((4.0 * (-var_qbd_max_dn17)) * var_dlt_qbd) + (assign33310_e48449 * var_dlt_qbd_dn17)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign33310_e48453;
        var_tmf2_dn0 = assign33310_e48453_d_n0;
        var_tmf2_dn2 = assign33310_e48453_d_n2;
        var_tmf2_dn6 = assign33310_e48453_d_n6;
        var_tmf2_dn7 = assign33310_e48453_d_n7;
        var_tmf2_dn10 = assign33310_e48453_d_n10;
        var_tmf2_dn11 = assign33310_e48453_d_n11;
        var_tmf2_dn12 = assign33310_e48453_d_n12;
        var_tmf2_dn17 = assign33310_e48453_d_n17;

        let (assign33320_e48465, assign33320_e48465_d_n0, assign33320_e48465_d_n2, assign33320_e48465_d_n6, assign33320_e48465_d_n7, assign33320_e48465_d_n10, assign33320_e48465_d_n11, assign33320_e48465_d_n12, assign33320_e48465_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1089 != 0.0)) {
        let (assign33320_e48463, assign33320_e48463_d_n0, assign33320_e48463_d_n2, assign33320_e48463_d_n6, assign33320_e48463_d_n7, assign33320_e48463_d_n10, assign33320_e48463_d_n11, assign33320_e48463_d_n12, assign33320_e48463_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign33320_e48462: f64 = (-var_tmf2);
                (assign33320_e48462, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign33320_e48463, assign33320_e48463_d_n0, assign33320_e48463_d_n2, assign33320_e48463_d_n6, assign33320_e48463_d_n7, assign33320_e48463_d_n10, assign33320_e48463_d_n11, assign33320_e48463_d_n12, assign33320_e48463_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign33320_e48465;
        var_tmf2_dn0 = assign33320_e48465_d_n0;
        var_tmf2_dn2 = assign33320_e48465_d_n2;
        var_tmf2_dn6 = assign33320_e48465_d_n6;
        var_tmf2_dn7 = assign33320_e48465_d_n7;
        var_tmf2_dn10 = assign33320_e48465_d_n10;
        var_tmf2_dn11 = assign33320_e48465_d_n11;
        var_tmf2_dn12 = assign33320_e48465_d_n12;
        var_tmf2_dn17 = assign33320_e48465_d_n17;

        let (assign33330_e48476, assign33330_e48476_d_n0, assign33330_e48476_d_n2, assign33330_e48476_d_n6, assign33330_e48476_d_n7, assign33330_e48476_d_n10, assign33330_e48476_d_n11, assign33330_e48476_d_n12, assign33330_e48476_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1089 != 0.0)) {
        let assign33330_e48471: f64 = (var_tmf1 * var_tmf1);
        let assign33330_e48473: f64 = (assign33330_e48471 + var_tmf2);
        let assign33330_e48474: f64 = (assign33330_e48473).sqrt();
        (assign33330_e48474, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign33330_e48474)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign33330_e48474)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign33330_e48474)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign33330_e48474)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign33330_e48474)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign33330_e48474)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign33330_e48474)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign33330_e48474)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign33330_e48476;
        var_tmf2_dn0 = assign33330_e48476_d_n0;
        var_tmf2_dn2 = assign33330_e48476_d_n2;
        var_tmf2_dn6 = assign33330_e48476_d_n6;
        var_tmf2_dn7 = assign33330_e48476_d_n7;
        var_tmf2_dn10 = assign33330_e48476_d_n10;
        var_tmf2_dn11 = assign33330_e48476_d_n11;
        var_tmf2_dn12 = assign33330_e48476_d_n12;
        var_tmf2_dn17 = assign33330_e48476_d_n17;

        let (assign33340_e48489, assign33340_e48489_d_n0, assign33340_e48489_d_n2, assign33340_e48489_d_n6, assign33340_e48489_d_n7, assign33340_e48489_d_n10, assign33340_e48489_d_n11, assign33340_e48489_d_n12, assign33340_e48489_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1089 != 0.0)) {
        let assign33340_e48481: f64 = (-var_qbd_max);
        let assign33340_e48485: f64 = (var_tmf1 + var_tmf2);
        let assign33340_e48486: f64 = (0.5 * assign33340_e48485);
        let assign33340_e48487: f64 = (assign33340_e48481 - assign33340_e48486);
        (assign33340_e48487, ((-var_qbd_max_dn0) - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), ((-var_qbd_max_dn2) - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), ((-var_qbd_max_dn6) - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), ((-var_qbd_max_dn7) - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), ((-var_qbd_max_dn10) - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), ((-var_qbd_max_dn11) - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), ((-var_qbd_max_dn12) - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), ((-var_qbd_max_dn17) - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33340_e48489;
        var_qbd_dn0 = assign33340_e48489_d_n0;
        var_qbd_dn2 = assign33340_e48489_d_n2;
        var_qbd_dn6 = assign33340_e48489_d_n6;
        var_qbd_dn7 = assign33340_e48489_d_n7;
        var_qbd_dn10 = assign33340_e48489_d_n10;
        var_qbd_dn11 = assign33340_e48489_d_n11;
        var_qbd_dn12 = assign33340_e48489_d_n12;
        var_qbd_dn17 = assign33340_e48489_d_n17;

        let (assign33350_e48498, assign33350_e48498_d_n0, assign33350_e48498_d_n2, assign33350_e48498_d_n6, assign33350_e48498_d_n7, assign33350_e48498_d_n10, assign33350_e48498_d_n11, assign33350_e48498_d_n12, assign33350_e48498_d_n17,) = {
    if ((var_guard1030 != 0.0) && (var_guard1089 != 0.0)) {
        let assign33350_e48495: f64 = (-1.0);
        let assign33350_e48496: f64 = (var_qbd * assign33350_e48495);
        (assign33350_e48496, (var_qbd_dn0 * assign33350_e48495), (var_qbd_dn2 * assign33350_e48495), (var_qbd_dn6 * assign33350_e48495), (var_qbd_dn7 * assign33350_e48495), (var_qbd_dn10 * assign33350_e48495), (var_qbd_dn11 * assign33350_e48495), (var_qbd_dn12 * assign33350_e48495), (var_qbd_dn17 * assign33350_e48495),)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33350_e48498;
        var_qbd_dn0 = assign33350_e48498_d_n0;
        var_qbd_dn2 = assign33350_e48498_d_n2;
        var_qbd_dn6 = assign33350_e48498_d_n6;
        var_qbd_dn7 = assign33350_e48498_d_n7;
        var_qbd_dn10 = assign33350_e48498_d_n10;
        var_qbd_dn11 = assign33350_e48498_d_n11;
        var_qbd_dn12 = assign33350_e48498_d_n12;
        var_qbd_dn17 = assign33350_e48498_d_n17;

        let assign33360_e48501: f64 = if var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        var_guard1095 = assign33360_e48501;

        let (assign33370_e48507,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1095 != 0.0)) {
        (p.p233,)
    } else {
        (var_t10__blk1090,)
    }
};
        var_t10__blk1090 = assign33370_e48507;

        let (assign33380_e48513,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1095 != 0.0)) {
        (p.p234,)
    } else {
        (var_t11__blk1091,)
    }
};
        var_t11__blk1091 = assign33380_e48513;

        let (assign33390_e48519, assign33390_e48519_d_n0, assign33390_e48519_d_n2, assign33390_e48519_d_n6, assign33390_e48519_d_n7, assign33390_e48519_d_n10, assign33390_e48519_d_n11, assign33390_e48519_d_n12, assign33390_e48519_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1095 != 0.0)) {
        (var_lch, var_lch_dn0, var_lch_dn2, var_lch_dn6, var_lch_dn7, var_lch_dn10, var_lch_dn11, var_lch_dn12, var_lch_dn17,)
    } else {
        (var_t12, var_t12_dn0, var_t12_dn2, var_t12_dn6, var_t12_dn7, var_t12_dn10, var_t12_dn11, var_t12_dn12, var_t12_dn17,)
    }
};
        var_t12 = assign33390_e48519;
        var_t12_dn0 = assign33390_e48519_d_n0;
        var_t12_dn2 = assign33390_e48519_d_n2;
        var_t12_dn6 = assign33390_e48519_d_n6;
        var_t12_dn7 = assign33390_e48519_d_n7;
        var_t12_dn10 = assign33390_e48519_d_n10;
        var_t12_dn11 = assign33390_e48519_d_n11;
        var_t12_dn12 = assign33390_e48519_d_n12;
        var_t12_dn17 = assign33390_e48519_d_n17;

        let (assign33400_e48531, assign33400_e48531_d_n0, assign33400_e48531_d_n2, assign33400_e48531_d_n6, assign33400_e48531_d_n7, assign33400_e48531_d_n10, assign33400_e48531_d_n11, assign33400_e48531_d_n12, assign33400_e48531_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1095 != 0.0)) {
        let assign33400_e48525: f64 = (var_t10__blk1090 * var_t11__blk1091);
        let assign33400_e48527: f64 = (assign33400_e48525 * var_t12);
        let assign33400_e48529: f64 = (assign33400_e48527 * var_t12);
        (assign33400_e48529, (((assign33400_e48525 * var_t12_dn0) * var_t12) + (assign33400_e48527 * var_t12_dn0)), (((assign33400_e48525 * var_t12_dn2) * var_t12) + (assign33400_e48527 * var_t12_dn2)), (((assign33400_e48525 * var_t12_dn6) * var_t12) + (assign33400_e48527 * var_t12_dn6)), (((assign33400_e48525 * var_t12_dn7) * var_t12) + (assign33400_e48527 * var_t12_dn7)), (((assign33400_e48525 * var_t12_dn10) * var_t12) + (assign33400_e48527 * var_t12_dn10)), (((assign33400_e48525 * var_t12_dn11) * var_t12) + (assign33400_e48527 * var_t12_dn11)), (((assign33400_e48525 * var_t12_dn12) * var_t12) + (assign33400_e48527 * var_t12_dn12)), (((assign33400_e48525 * var_t12_dn17) * var_t12) + (assign33400_e48527 * var_t12_dn17)),)
    } else {
        (var_t1__blk1093, var_t1__blk1093_dn0, var_t1__blk1093_dn2, var_t1__blk1093_dn6, var_t1__blk1093_dn7, var_t1__blk1093_dn10, var_t1__blk1093_dn11, var_t1__blk1093_dn12, var_t1__blk1093_dn17,)
    }
};
        var_t1__blk1093 = assign33400_e48531;
        var_t1__blk1093_dn0 = assign33400_e48531_d_n0;
        var_t1__blk1093_dn2 = assign33400_e48531_d_n2;
        var_t1__blk1093_dn6 = assign33400_e48531_d_n6;
        var_t1__blk1093_dn7 = assign33400_e48531_d_n7;
        var_t1__blk1093_dn10 = assign33400_e48531_d_n10;
        var_t1__blk1093_dn11 = assign33400_e48531_d_n11;
        var_t1__blk1093_dn12 = assign33400_e48531_d_n12;
        var_t1__blk1093_dn17 = assign33400_e48531_d_n17;

        let (assign33410_e48549, assign33410_e48549_d_n0, assign33410_e48549_d_n2, assign33410_e48549_d_n6, assign33410_e48549_d_n7, assign33410_e48549_d_n10, assign33410_e48549_d_n11, assign33410_e48549_d_n12, assign33410_e48549_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1095 != 0.0)) {
        let assign33410_e48537: f64 = (var_mu * var_vgvt);
        let assign33410_e48539: f64 = (assign33410_e48537 * var_t10__blk1090);
        let assign33410_e48542: f64 = (var_t11__blk1091 * var_t12);
        let assign33410_e48544: f64 = (assign33410_e48542 * var_t12);
        let assign33410_e48545: f64 = (assign33410_e48539 + assign33410_e48544);
        let assign33410_e48547: f64 = (assign33410_e48545 + 1e-50);
        (assign33410_e48547, ((((var_mu_dn0 * var_vgvt) + (var_mu * var_vgvt_dn0)) * var_t10__blk1090) + (((var_t11__blk1091 * var_t12_dn0) * var_t12) + (assign33410_e48542 * var_t12_dn0))), ((((var_mu_dn2 * var_vgvt) + (var_mu * var_vgvt_dn2)) * var_t10__blk1090) + (((var_t11__blk1091 * var_t12_dn2) * var_t12) + (assign33410_e48542 * var_t12_dn2))), ((((var_mu_dn6 * var_vgvt) + (var_mu * var_vgvt_dn6)) * var_t10__blk1090) + (((var_t11__blk1091 * var_t12_dn6) * var_t12) + (assign33410_e48542 * var_t12_dn6))), ((((var_mu_dn7 * var_vgvt) + (var_mu * var_vgvt_dn7)) * var_t10__blk1090) + (((var_t11__blk1091 * var_t12_dn7) * var_t12) + (assign33410_e48542 * var_t12_dn7))), ((((var_mu_dn10 * var_vgvt) + (var_mu * var_vgvt_dn10)) * var_t10__blk1090) + (((var_t11__blk1091 * var_t12_dn10) * var_t12) + (assign33410_e48542 * var_t12_dn10))), ((((var_mu_dn11 * var_vgvt) + (var_mu * var_vgvt_dn11)) * var_t10__blk1090) + (((var_t11__blk1091 * var_t12_dn11) * var_t12) + (assign33410_e48542 * var_t12_dn11))), ((((var_mu_dn12 * var_vgvt) + (var_mu * var_vgvt_dn12)) * var_t10__blk1090) + (((var_t11__blk1091 * var_t12_dn12) * var_t12) + (assign33410_e48542 * var_t12_dn12))), ((((var_mu_dn17 * var_vgvt) + (var_mu * var_vgvt_dn17)) * var_t10__blk1090) + (((var_t11__blk1091 * var_t12_dn17) * var_t12) + (assign33410_e48542 * var_t12_dn17))),)
    } else {
        (var_t2__blk1094, var_t2__blk1094_dn0, var_t2__blk1094_dn2, var_t2__blk1094_dn6, var_t2__blk1094_dn7, var_t2__blk1094_dn10, var_t2__blk1094_dn11, var_t2__blk1094_dn12, var_t2__blk1094_dn17,)
    }
};
        var_t2__blk1094 = assign33410_e48549;
        var_t2__blk1094_dn0 = assign33410_e48549_d_n0;
        var_t2__blk1094_dn2 = assign33410_e48549_d_n2;
        var_t2__blk1094_dn6 = assign33410_e48549_d_n6;
        var_t2__blk1094_dn7 = assign33410_e48549_d_n7;
        var_t2__blk1094_dn10 = assign33410_e48549_d_n10;
        var_t2__blk1094_dn11 = assign33410_e48549_d_n11;
        var_t2__blk1094_dn12 = assign33410_e48549_d_n12;
        var_t2__blk1094_dn17 = assign33410_e48549_d_n17;

        let (assign33420_e48557, assign33420_e48557_d_n0, assign33420_e48557_d_n2, assign33420_e48557_d_n6, assign33420_e48557_d_n7, assign33420_e48557_d_n10, assign33420_e48557_d_n11, assign33420_e48557_d_n12, assign33420_e48557_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1095 != 0.0)) {
        let assign33420_e48555: f64 = (var_t1__blk1093 / var_t2__blk1094);
        (assign33420_e48555, (((var_t1__blk1093_dn0 * var_t2__blk1094) - (var_t1__blk1093 * var_t2__blk1094_dn0)) / (var_t2__blk1094 * var_t2__blk1094)), (((var_t1__blk1093_dn2 * var_t2__blk1094) - (var_t1__blk1093 * var_t2__blk1094_dn2)) / (var_t2__blk1094 * var_t2__blk1094)), (((var_t1__blk1093_dn6 * var_t2__blk1094) - (var_t1__blk1093 * var_t2__blk1094_dn6)) / (var_t2__blk1094 * var_t2__blk1094)), (((var_t1__blk1093_dn7 * var_t2__blk1094) - (var_t1__blk1093 * var_t2__blk1094_dn7)) / (var_t2__blk1094 * var_t2__blk1094)), (((var_t1__blk1093_dn10 * var_t2__blk1094) - (var_t1__blk1093 * var_t2__blk1094_dn10)) / (var_t2__blk1094 * var_t2__blk1094)), (((var_t1__blk1093_dn11 * var_t2__blk1094) - (var_t1__blk1093 * var_t2__blk1094_dn11)) / (var_t2__blk1094 * var_t2__blk1094)), (((var_t1__blk1093_dn12 * var_t2__blk1094) - (var_t1__blk1093 * var_t2__blk1094_dn12)) / (var_t2__blk1094 * var_t2__blk1094)), (((var_t1__blk1093_dn17 * var_t2__blk1094) - (var_t1__blk1093 * var_t2__blk1094_dn17)) / (var_t2__blk1094 * var_t2__blk1094)),)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn6, var_tau_dn7, var_tau_dn10, var_tau_dn11, var_tau_dn12, var_tau_dn17,)
    }
};
        var_tau = assign33420_e48557;
        var_tau_dn0 = assign33420_e48557_d_n0;
        var_tau_dn2 = assign33420_e48557_d_n2;
        var_tau_dn6 = assign33420_e48557_d_n6;
        var_tau_dn7 = assign33420_e48557_d_n7;
        var_tau_dn10 = assign33420_e48557_d_n10;
        var_tau_dn11 = assign33420_e48557_d_n11;
        var_tau_dn12 = assign33420_e48557_d_n12;
        var_tau_dn17 = assign33420_e48557_d_n17;

        let (assign33430_e48566, assign33430_e48566_d_n0, assign33430_e48566_d_n2, assign33430_e48566_d_n6, assign33430_e48566_d_n7, assign33430_e48566_d_n10, assign33430_e48566_d_n11, assign33430_e48566_d_n12, assign33430_e48566_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1095 == 0.0)) {
        let assign33430_e48564: f64 = (p.p233 + 1e-50);
        (assign33430_e48564, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn6, var_tau_dn7, var_tau_dn10, var_tau_dn11, var_tau_dn12, var_tau_dn17,)
    }
};
        var_tau = assign33430_e48566;
        var_tau_dn0 = assign33430_e48566_d_n0;
        var_tau_dn2 = assign33430_e48566_d_n2;
        var_tau_dn6 = assign33430_e48566_d_n6;
        var_tau_dn7 = assign33430_e48566_d_n7;
        var_tau_dn10 = assign33430_e48566_d_n10;
        var_tau_dn11 = assign33430_e48566_d_n11;
        var_tau_dn12 = assign33430_e48566_d_n12;
        var_tau_dn17 = assign33430_e48566_d_n17;

        let (assign33440_e48570, assign33440_e48570_d_n0, assign33440_e48570_d_n2, assign33440_e48570_d_n6, assign33440_e48570_d_n7, assign33440_e48570_d_n10, assign33440_e48570_d_n11, assign33440_e48570_d_n12, assign33440_e48570_d_n17,) = {
    if (var_flg_nqs != 0.0) {
        (p.p235, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk1093, var_t1__blk1093_dn0, var_t1__blk1093_dn2, var_t1__blk1093_dn6, var_t1__blk1093_dn7, var_t1__blk1093_dn10, var_t1__blk1093_dn11, var_t1__blk1093_dn12, var_t1__blk1093_dn17,)
    }
};
        var_t1__blk1093 = assign33440_e48570;
        var_t1__blk1093_dn0 = assign33440_e48570_d_n0;
        var_t1__blk1093_dn2 = assign33440_e48570_d_n2;
        var_t1__blk1093_dn6 = assign33440_e48570_d_n6;
        var_t1__blk1093_dn7 = assign33440_e48570_d_n7;
        var_t1__blk1093_dn10 = assign33440_e48570_d_n10;
        var_t1__blk1093_dn11 = assign33440_e48570_d_n11;
        var_t1__blk1093_dn12 = assign33440_e48570_d_n12;
        var_t1__blk1093_dn17 = assign33440_e48570_d_n17;

        let (assign33450_e48576, assign33450_e48576_d_n0, assign33450_e48576_d_n2, assign33450_e48576_d_n6, assign33450_e48576_d_n7, assign33450_e48576_d_n10, assign33450_e48576_d_n11, assign33450_e48576_d_n12, assign33450_e48576_d_n17,) = {
    if (var_flg_nqs != 0.0) {
        let assign33450_e48574: f64 = (var_t1__blk1093 * var_c_fox);
        (assign33450_e48574, ((var_t1__blk1093_dn0 * var_c_fox) + (var_t1__blk1093 * var_c_fox_dn0)), ((var_t1__blk1093_dn2 * var_c_fox) + (var_t1__blk1093 * var_c_fox_dn2)), ((var_t1__blk1093_dn6 * var_c_fox) + (var_t1__blk1093 * var_c_fox_dn6)), ((var_t1__blk1093_dn7 * var_c_fox) + (var_t1__blk1093 * var_c_fox_dn7)), ((var_t1__blk1093_dn10 * var_c_fox) + (var_t1__blk1093 * var_c_fox_dn10)), ((var_t1__blk1093_dn11 * var_c_fox) + (var_t1__blk1093 * var_c_fox_dn11)), ((var_t1__blk1093_dn12 * var_c_fox) + (var_t1__blk1093 * var_c_fox_dn12)), ((var_t1__blk1093_dn17 * var_c_fox) + (var_t1__blk1093 * var_c_fox_dn17)),)
    } else {
        (var_taub, var_taub_dn0, var_taub_dn2, var_taub_dn6, var_taub_dn7, var_taub_dn10, var_taub_dn11, var_taub_dn12, var_taub_dn17,)
    }
};
        var_taub = assign33450_e48576;
        var_taub_dn0 = assign33450_e48576_d_n0;
        var_taub_dn2 = assign33450_e48576_d_n2;
        var_taub_dn6 = assign33450_e48576_d_n6;
        var_taub_dn7 = assign33450_e48576_d_n7;
        var_taub_dn10 = assign33450_e48576_d_n10;
        var_taub_dn11 = assign33450_e48576_d_n11;
        var_taub_dn12 = assign33450_e48576_d_n12;
        var_taub_dn17 = assign33450_e48576_d_n17;

        let assign33580_e48752: f64 = if ((p.p32 != 0.0) && (var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        var_guard1122 = assign33580_e48752;

        let (assign33590_e48760, assign33590_e48760_d_n0, assign33590_e48760_d_n2, assign33590_e48760_d_n6, assign33590_e48760_d_n7, assign33590_e48760_d_n10, assign33590_e48760_d_n11, assign33590_e48760_d_n12, assign33590_e48760_d_n17,) = {
    if (var_guard1122 != 0.0) {
        let assign33590_e48756: f64 = (var_psdl - var_ps0);
        let assign33590_e48758: f64 = (assign33590_e48756 / var_lch);
        (assign33590_e48758, ((((var_psdl_dn0 - var_ps0_dn0) * var_lch) - (assign33590_e48756 * var_lch_dn0)) / (var_lch * var_lch)), ((((var_psdl_dn2 - var_ps0_dn2) * var_lch) - (assign33590_e48756 * var_lch_dn2)) / (var_lch * var_lch)), ((((var_psdl_dn6 - var_ps0_dn6) * var_lch) - (assign33590_e48756 * var_lch_dn6)) / (var_lch * var_lch)), ((((var_psdl_dn7 - var_ps0_dn7) * var_lch) - (assign33590_e48756 * var_lch_dn7)) / (var_lch * var_lch)), ((((var_psdl_dn10 - var_ps0_dn10) * var_lch) - (assign33590_e48756 * var_lch_dn10)) / (var_lch * var_lch)), ((((var_psdl_dn11 - var_ps0_dn11) * var_lch) - (assign33590_e48756 * var_lch_dn11)) / (var_lch * var_lch)), ((((var_psdl_dn12 - var_ps0_dn12) * var_lch) - (assign33590_e48756 * var_lch_dn12)) / (var_lch * var_lch)), ((((var_psdl_dn17 - var_ps0_dn17) * var_lch) - (assign33590_e48756 * var_lch_dn17)) / (var_lch * var_lch)),)
    } else {
        (var_eyd, var_eyd_dn0, var_eyd_dn2, var_eyd_dn6, var_eyd_dn7, var_eyd_dn10, var_eyd_dn11, var_eyd_dn12, var_eyd_dn17,)
    }
};
        var_eyd = assign33590_e48760;
        var_eyd_dn0 = assign33590_e48760_d_n0;
        var_eyd_dn2 = assign33590_e48760_d_n2;
        var_eyd_dn6 = assign33590_e48760_d_n6;
        var_eyd_dn7 = assign33590_e48760_d_n7;
        var_eyd_dn10 = assign33590_e48760_d_n10;
        var_eyd_dn11 = assign33590_e48760_d_n11;
        var_eyd_dn12 = assign33590_e48760_d_n12;
        var_eyd_dn17 = assign33590_e48760_d_n17;

        let (assign33600_e48768, assign33600_e48768_d_n0, assign33600_e48768_d_n2, assign33600_e48768_d_n6, assign33600_e48768_d_n7, assign33600_e48768_d_n10, assign33600_e48768_d_n11, assign33600_e48768_d_n12, assign33600_e48768_d_n17,) = {
    if (var_guard1122 != 0.0) {
        let assign33600_e48764: f64 = (var_muun * var_eyd);
        let assign33600_e48766: f64 = (assign33600_e48764 / 100000.0);
        (assign33600_e48766, (((var_muun_dn0 * var_eyd) + (var_muun * var_eyd_dn0)) / 100000.0), (((var_muun_dn2 * var_eyd) + (var_muun * var_eyd_dn2)) / 100000.0), (((var_muun_dn6 * var_eyd) + (var_muun * var_eyd_dn6)) / 100000.0), (((var_muun_dn7 * var_eyd) + (var_muun * var_eyd_dn7)) / 100000.0), (((var_muun_dn10 * var_eyd) + (var_muun * var_eyd_dn10)) / 100000.0), (((var_muun_dn11 * var_eyd) + (var_muun * var_eyd_dn11)) / 100000.0), (((var_muun_dn12 * var_eyd) + (var_muun * var_eyd_dn12)) / 100000.0), (((var_muun_dn17 * var_eyd) + (var_muun * var_eyd_dn17)) / 100000.0),)
    } else {
        (var_t12__blk1106, var_t12__blk1106_dn0, var_t12__blk1106_dn2, var_t12__blk1106_dn6, var_t12__blk1106_dn7, var_t12__blk1106_dn10, var_t12__blk1106_dn11, var_t12__blk1106_dn12, var_t12__blk1106_dn17,)
    }
};
        var_t12__blk1106 = assign33600_e48768;
        var_t12__blk1106_dn0 = assign33600_e48768_d_n0;
        var_t12__blk1106_dn2 = assign33600_e48768_d_n2;
        var_t12__blk1106_dn6 = assign33600_e48768_d_n6;
        var_t12__blk1106_dn7 = assign33600_e48768_d_n7;
        var_t12__blk1106_dn10 = assign33600_e48768_d_n10;
        var_t12__blk1106_dn11 = assign33600_e48768_d_n11;
        var_t12__blk1106_dn12 = assign33600_e48768_d_n12;
        var_t12__blk1106_dn17 = assign33600_e48768_d_n17;

        let assign33610_e48772: f64 = (10.0 * 2.220446049250313e-16);
        let assign33610_e48773: f64 = (1.0 - assign33610_e48772);
        let assign33610_e48780: f64 = (10.0 * 2.220446049250313e-16);
        let assign33610_e48781: f64 = (1.0 + assign33610_e48780);
        let assign33610_e48783: f64 = if ((assign33610_e48773 <= p.p113) && (p.p113 <= assign33610_e48781)) { 1.0 } else { 0.0 };
        var_guard1123 = assign33610_e48783;

        let (assign33620_e48789, assign33620_e48789_d_n0, assign33620_e48789_d_n2, assign33620_e48789_d_n6, assign33620_e48789_d_n7, assign33620_e48789_d_n10, assign33620_e48789_d_n11, assign33620_e48789_d_n12, assign33620_e48789_d_n17,) = {
    if ((var_guard1122 != 0.0) && (var_guard1123 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7__blk1107, var_t7__blk1107_dn0, var_t7__blk1107_dn2, var_t7__blk1107_dn6, var_t7__blk1107_dn7, var_t7__blk1107_dn10, var_t7__blk1107_dn11, var_t7__blk1107_dn12, var_t7__blk1107_dn17,)
    }
};
        var_t7__blk1107 = assign33620_e48789;
        var_t7__blk1107_dn0 = assign33620_e48789_d_n0;
        var_t7__blk1107_dn2 = assign33620_e48789_d_n2;
        var_t7__blk1107_dn6 = assign33620_e48789_d_n6;
        var_t7__blk1107_dn7 = assign33620_e48789_d_n7;
        var_t7__blk1107_dn10 = assign33620_e48789_d_n10;
        var_t7__blk1107_dn11 = assign33620_e48789_d_n11;
        var_t7__blk1107_dn12 = assign33620_e48789_d_n12;
        var_t7__blk1107_dn17 = assign33620_e48789_d_n17;

        let assign33630_e48793: f64 = (10.0 * 2.220446049250313e-16);
        let assign33630_e48794: f64 = (2.0 - assign33630_e48793);
        let assign33630_e48801: f64 = (10.0 * 2.220446049250313e-16);
        let assign33630_e48802: f64 = (2.0 + assign33630_e48801);
        let assign33630_e48804: f64 = if ((assign33630_e48794 <= p.p113) && (p.p113 <= assign33630_e48802)) { 1.0 } else { 0.0 };
        var_guard1124 = assign33630_e48804;

        let (assign33640_e48813, assign33640_e48813_d_n0, assign33640_e48813_d_n2, assign33640_e48813_d_n6, assign33640_e48813_d_n7, assign33640_e48813_d_n10, assign33640_e48813_d_n11, assign33640_e48813_d_n12, assign33640_e48813_d_n17,) = {
    if (((var_guard1122 != 0.0) && (var_guard1123 == 0.0)) && (var_guard1124 != 0.0)) {
        (var_t12__blk1106, var_t12__blk1106_dn0, var_t12__blk1106_dn2, var_t12__blk1106_dn6, var_t12__blk1106_dn7, var_t12__blk1106_dn10, var_t12__blk1106_dn11, var_t12__blk1106_dn12, var_t12__blk1106_dn17,)
    } else {
        (var_t7__blk1107, var_t7__blk1107_dn0, var_t7__blk1107_dn2, var_t7__blk1107_dn6, var_t7__blk1107_dn7, var_t7__blk1107_dn10, var_t7__blk1107_dn11, var_t7__blk1107_dn12, var_t7__blk1107_dn17,)
    }
};
        var_t7__blk1107 = assign33640_e48813;
        var_t7__blk1107_dn0 = assign33640_e48813_d_n0;
        var_t7__blk1107_dn2 = assign33640_e48813_d_n2;
        var_t7__blk1107_dn6 = assign33640_e48813_d_n6;
        var_t7__blk1107_dn7 = assign33640_e48813_d_n7;
        var_t7__blk1107_dn10 = assign33640_e48813_d_n10;
        var_t7__blk1107_dn11 = assign33640_e48813_d_n11;
        var_t7__blk1107_dn12 = assign33640_e48813_d_n12;
        var_t7__blk1107_dn17 = assign33640_e48813_d_n17;

        let (assign33650_e48827, assign33650_e48827_d_n0, assign33650_e48827_d_n2, assign33650_e48827_d_n6, assign33650_e48827_d_n7, assign33650_e48827_d_n10, assign33650_e48827_d_n11, assign33650_e48827_d_n12, assign33650_e48827_d_n17,) = {
    if (((var_guard1122 != 0.0) && (var_guard1123 == 0.0)) && (var_guard1124 == 0.0)) {
        let assign33650_e48824: f64 = (p.p113 - 1.0);
        let assign33650_e48825: f64 = (var_t12__blk1106).powf(assign33650_e48824);
        (assign33650_e48825, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((var_t12__blk1106).powf(assign33650_e48824 - 1.0) * var_t12__blk1106_dn0)) } } else { (assign33650_e48825 * (assign33650_e48824 * (var_t12__blk1106_dn0 / var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((var_t12__blk1106).powf(assign33650_e48824 - 1.0) * var_t12__blk1106_dn2)) } } else { (assign33650_e48825 * (assign33650_e48824 * (var_t12__blk1106_dn2 / var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((var_t12__blk1106).powf(assign33650_e48824 - 1.0) * var_t12__blk1106_dn6)) } } else { (assign33650_e48825 * (assign33650_e48824 * (var_t12__blk1106_dn6 / var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((var_t12__blk1106).powf(assign33650_e48824 - 1.0) * var_t12__blk1106_dn7)) } } else { (assign33650_e48825 * (assign33650_e48824 * (var_t12__blk1106_dn7 / var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((var_t12__blk1106).powf(assign33650_e48824 - 1.0) * var_t12__blk1106_dn10)) } } else { (assign33650_e48825 * (assign33650_e48824 * (var_t12__blk1106_dn10 / var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((var_t12__blk1106).powf(assign33650_e48824 - 1.0) * var_t12__blk1106_dn11)) } } else { (assign33650_e48825 * (assign33650_e48824 * (var_t12__blk1106_dn11 / var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((var_t12__blk1106).powf(assign33650_e48824 - 1.0) * var_t12__blk1106_dn12)) } } else { (assign33650_e48825 * (assign33650_e48824 * (var_t12__blk1106_dn12 / var_t12__blk1106))) }, if 0.0 == 0.0 && ((assign33650_e48824) as f64).is_finite() && ((assign33650_e48824) as f64).fract() == 0.0 { if assign33650_e48824 == 0.0 { 0.0 } else { (assign33650_e48824 * ((var_t12__blk1106).powf(assign33650_e48824 - 1.0) * var_t12__blk1106_dn17)) } } else { (assign33650_e48825 * (assign33650_e48824 * (var_t12__blk1106_dn17 / var_t12__blk1106))) },)
    } else {
        (var_t7__blk1107, var_t7__blk1107_dn0, var_t7__blk1107_dn2, var_t7__blk1107_dn6, var_t7__blk1107_dn7, var_t7__blk1107_dn10, var_t7__blk1107_dn11, var_t7__blk1107_dn12, var_t7__blk1107_dn17,)
    }
};
        var_t7__blk1107 = assign33650_e48827;
        var_t7__blk1107_dn0 = assign33650_e48827_d_n0;
        var_t7__blk1107_dn2 = assign33650_e48827_d_n2;
        var_t7__blk1107_dn6 = assign33650_e48827_d_n6;
        var_t7__blk1107_dn7 = assign33650_e48827_d_n7;
        var_t7__blk1107_dn10 = assign33650_e48827_d_n10;
        var_t7__blk1107_dn11 = assign33650_e48827_d_n11;
        var_t7__blk1107_dn12 = assign33650_e48827_d_n12;
        var_t7__blk1107_dn17 = assign33650_e48827_d_n17;

        let (assign33660_e48833, assign33660_e48833_d_n0, assign33660_e48833_d_n2, assign33660_e48833_d_n6, assign33660_e48833_d_n7, assign33660_e48833_d_n10, assign33660_e48833_d_n11, assign33660_e48833_d_n12, assign33660_e48833_d_n17,) = {
    if (var_guard1122 != 0.0) {
        let assign33660_e48831: f64 = (var_t12__blk1106 * var_t7__blk1107);
        (assign33660_e48831, ((var_t12__blk1106_dn0 * var_t7__blk1107) + (var_t12__blk1106 * var_t7__blk1107_dn0)), ((var_t12__blk1106_dn2 * var_t7__blk1107) + (var_t12__blk1106 * var_t7__blk1107_dn2)), ((var_t12__blk1106_dn6 * var_t7__blk1107) + (var_t12__blk1106 * var_t7__blk1107_dn6)), ((var_t12__blk1106_dn7 * var_t7__blk1107) + (var_t12__blk1106 * var_t7__blk1107_dn7)), ((var_t12__blk1106_dn10 * var_t7__blk1107) + (var_t12__blk1106 * var_t7__blk1107_dn10)), ((var_t12__blk1106_dn11 * var_t7__blk1107) + (var_t12__blk1106 * var_t7__blk1107_dn11)), ((var_t12__blk1106_dn12 * var_t7__blk1107) + (var_t12__blk1106 * var_t7__blk1107_dn12)), ((var_t12__blk1106_dn17 * var_t7__blk1107) + (var_t12__blk1106 * var_t7__blk1107_dn17)),)
    } else {
        (var_t8__blk1108, var_t8__blk1108_dn0, var_t8__blk1108_dn2, var_t8__blk1108_dn6, var_t8__blk1108_dn7, var_t8__blk1108_dn10, var_t8__blk1108_dn11, var_t8__blk1108_dn12, var_t8__blk1108_dn17,)
    }
};
        var_t8__blk1108 = assign33660_e48833;
        var_t8__blk1108_dn0 = assign33660_e48833_d_n0;
        var_t8__blk1108_dn2 = assign33660_e48833_d_n2;
        var_t8__blk1108_dn6 = assign33660_e48833_d_n6;
        var_t8__blk1108_dn7 = assign33660_e48833_d_n7;
        var_t8__blk1108_dn10 = assign33660_e48833_d_n10;
        var_t8__blk1108_dn11 = assign33660_e48833_d_n11;
        var_t8__blk1108_dn12 = assign33660_e48833_d_n12;
        var_t8__blk1108_dn17 = assign33660_e48833_d_n17;

        *var_dlt_qbd_slot = var_dlt_qbd;
        *var_dlt_qbd_dn0_slot = var_dlt_qbd_dn0;
        *var_dlt_qbd_dn10_slot = var_dlt_qbd_dn10;
        *var_dlt_qbd_dn11_slot = var_dlt_qbd_dn11;
        *var_dlt_qbd_dn12_slot = var_dlt_qbd_dn12;
        *var_dlt_qbd_dn17_slot = var_dlt_qbd_dn17;
        *var_dlt_qbd_dn2_slot = var_dlt_qbd_dn2;
        *var_dlt_qbd_dn6_slot = var_dlt_qbd_dn6;
        *var_dlt_qbd_dn7_slot = var_dlt_qbd_dn7;
        *var_eyd_slot = var_eyd;
        *var_eyd_dn0_slot = var_eyd_dn0;
        *var_eyd_dn10_slot = var_eyd_dn10;
        *var_eyd_dn11_slot = var_eyd_dn11;
        *var_eyd_dn12_slot = var_eyd_dn12;
        *var_eyd_dn17_slot = var_eyd_dn17;
        *var_eyd_dn2_slot = var_eyd_dn2;
        *var_eyd_dn6_slot = var_eyd_dn6;
        *var_eyd_dn7_slot = var_eyd_dn7;
        *var_guard1089_slot = var_guard1089;
        *var_guard1095_slot = var_guard1095;
        *var_guard1122_slot = var_guard1122;
        *var_guard1123_slot = var_guard1123;
        *var_guard1124_slot = var_guard1124;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn0_slot = var_qbd_dn0;
        *var_qbd_dn10_slot = var_qbd_dn10;
        *var_qbd_dn11_slot = var_qbd_dn11;
        *var_qbd_dn12_slot = var_qbd_dn12;
        *var_qbd_dn17_slot = var_qbd_dn17;
        *var_qbd_dn2_slot = var_qbd_dn2;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbd_max_slot = var_qbd_max;
        *var_qbd_max_dn0_slot = var_qbd_max_dn0;
        *var_qbd_max_dn10_slot = var_qbd_max_dn10;
        *var_qbd_max_dn11_slot = var_qbd_max_dn11;
        *var_qbd_max_dn12_slot = var_qbd_max_dn12;
        *var_qbd_max_dn17_slot = var_qbd_max_dn17;
        *var_qbd_max_dn2_slot = var_qbd_max_dn2;
        *var_qbd_max_dn6_slot = var_qbd_max_dn6;
        *var_qbd_max_dn7_slot = var_qbd_max_dn7;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn0_slot = var_qbs_dn0;
        *var_qbs_dn10_slot = var_qbs_dn10;
        *var_qbs_dn11_slot = var_qbs_dn11;
        *var_qbs_dn12_slot = var_qbs_dn12;
        *var_qbs_dn17_slot = var_qbs_dn17;
        *var_qbs_dn2_slot = var_qbs_dn2;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_t10__blk1090_slot = var_t10__blk1090;
        *var_t11__blk1091_slot = var_t11__blk1091;
        *var_t12_slot = var_t12;
        *var_t12__blk1106_slot = var_t12__blk1106;
        *var_t12__blk1106_dn0_slot = var_t12__blk1106_dn0;
        *var_t12__blk1106_dn10_slot = var_t12__blk1106_dn10;
        *var_t12__blk1106_dn11_slot = var_t12__blk1106_dn11;
        *var_t12__blk1106_dn12_slot = var_t12__blk1106_dn12;
        *var_t12__blk1106_dn17_slot = var_t12__blk1106_dn17;
        *var_t12__blk1106_dn2_slot = var_t12__blk1106_dn2;
        *var_t12__blk1106_dn6_slot = var_t12__blk1106_dn6;
        *var_t12__blk1106_dn7_slot = var_t12__blk1106_dn7;
        *var_t12_dn0_slot = var_t12_dn0;
        *var_t12_dn10_slot = var_t12_dn10;
        *var_t12_dn11_slot = var_t12_dn11;
        *var_t12_dn12_slot = var_t12_dn12;
        *var_t12_dn17_slot = var_t12_dn17;
        *var_t12_dn2_slot = var_t12_dn2;
        *var_t12_dn6_slot = var_t12_dn6;
        *var_t12_dn7_slot = var_t12_dn7;
        *var_t1__blk1093_slot = var_t1__blk1093;
        *var_t1__blk1093_dn0_slot = var_t1__blk1093_dn0;
        *var_t1__blk1093_dn10_slot = var_t1__blk1093_dn10;
        *var_t1__blk1093_dn11_slot = var_t1__blk1093_dn11;
        *var_t1__blk1093_dn12_slot = var_t1__blk1093_dn12;
        *var_t1__blk1093_dn17_slot = var_t1__blk1093_dn17;
        *var_t1__blk1093_dn2_slot = var_t1__blk1093_dn2;
        *var_t1__blk1093_dn6_slot = var_t1__blk1093_dn6;
        *var_t1__blk1093_dn7_slot = var_t1__blk1093_dn7;
        *var_t2__blk1094_slot = var_t2__blk1094;
        *var_t2__blk1094_dn0_slot = var_t2__blk1094_dn0;
        *var_t2__blk1094_dn10_slot = var_t2__blk1094_dn10;
        *var_t2__blk1094_dn11_slot = var_t2__blk1094_dn11;
        *var_t2__blk1094_dn12_slot = var_t2__blk1094_dn12;
        *var_t2__blk1094_dn17_slot = var_t2__blk1094_dn17;
        *var_t2__blk1094_dn2_slot = var_t2__blk1094_dn2;
        *var_t2__blk1094_dn6_slot = var_t2__blk1094_dn6;
        *var_t2__blk1094_dn7_slot = var_t2__blk1094_dn7;
        *var_t7__blk1107_slot = var_t7__blk1107;
        *var_t7__blk1107_dn0_slot = var_t7__blk1107_dn0;
        *var_t7__blk1107_dn10_slot = var_t7__blk1107_dn10;
        *var_t7__blk1107_dn11_slot = var_t7__blk1107_dn11;
        *var_t7__blk1107_dn12_slot = var_t7__blk1107_dn12;
        *var_t7__blk1107_dn17_slot = var_t7__blk1107_dn17;
        *var_t7__blk1107_dn2_slot = var_t7__blk1107_dn2;
        *var_t7__blk1107_dn6_slot = var_t7__blk1107_dn6;
        *var_t7__blk1107_dn7_slot = var_t7__blk1107_dn7;
        *var_t8__blk1108_slot = var_t8__blk1108;
        *var_t8__blk1108_dn0_slot = var_t8__blk1108_dn0;
        *var_t8__blk1108_dn10_slot = var_t8__blk1108_dn10;
        *var_t8__blk1108_dn11_slot = var_t8__blk1108_dn11;
        *var_t8__blk1108_dn12_slot = var_t8__blk1108_dn12;
        *var_t8__blk1108_dn17_slot = var_t8__blk1108_dn17;
        *var_t8__blk1108_dn2_slot = var_t8__blk1108_dn2;
        *var_t8__blk1108_dn6_slot = var_t8__blk1108_dn6;
        *var_t8__blk1108_dn7_slot = var_t8__blk1108_dn7;
        *var_tau_slot = var_tau;
        *var_tau_dn0_slot = var_tau_dn0;
        *var_tau_dn10_slot = var_tau_dn10;
        *var_tau_dn11_slot = var_tau_dn11;
        *var_tau_dn12_slot = var_tau_dn12;
        *var_tau_dn17_slot = var_tau_dn17;
        *var_tau_dn2_slot = var_tau_dn2;
        *var_tau_dn6_slot = var_tau_dn6;
        *var_tau_dn7_slot = var_tau_dn7;
        *var_taub_slot = var_taub;
        *var_taub_dn0_slot = var_taub_dn0;
        *var_taub_dn10_slot = var_taub_dn10;
        *var_taub_dn11_slot = var_taub_dn11;
        *var_taub_dn12_slot = var_taub_dn12;
        *var_taub_dn17_slot = var_taub_dn17;
        *var_taub_dn2_slot = var_taub_dn2;
        *var_taub_dn6_slot = var_taub_dn6;
        *var_taub_dn7_slot = var_taub_dn7;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
    }

    pub(super) fn stamp_transient_block_118(
        p: &Parameters,
        var_alpha: f64,
        var_alpha_dn0: f64,
        var_alpha_dn10: f64,
        var_alpha_dn11: f64,
        var_alpha_dn12: f64,
        var_alpha_dn17: f64,
        var_alpha_dn2: f64,
        var_alpha_dn6: f64,
        var_alpha_dn7: f64,
        var_c_fox: f64,
        var_c_fox_dn0: f64,
        var_c_fox_dn10: f64,
        var_c_fox_dn11: f64,
        var_c_fox_dn12: f64,
        var_c_fox_dn17: f64,
        var_c_fox_dn2: f64,
        var_c_fox_dn6: f64,
        var_c_fox_dn7: f64,
        var_cbtn: f64,
        var_cbtp: f64,
        var_cgbo_given: f64,
        var_flg_ign: f64,
        var_flg_noqi: f64,
        var_guard1122: f64,
        var_idsibpc: f64,
        var_idsibpc_dn0: f64,
        var_idsibpc_dn10: f64,
        var_idsibpc_dn11: f64,
        var_idsibpc_dn12: f64,
        var_idsibpc_dn17: f64,
        var_idsibpc_dn2: f64,
        var_idsibpc_dn6: f64,
        var_idsibpc_dn7: f64,
        var_kusai00: f64,
        var_kusai00_dn0: f64,
        var_kusai00_dn10: f64,
        var_kusai00_dn11: f64,
        var_kusai00_dn12: f64,
        var_kusai00_dn17: f64,
        var_kusai00_dn2: f64,
        var_kusai00_dn6: f64,
        var_kusai00_dn7: f64,
        var_kusai00l: f64,
        var_kusai00l_dn0: f64,
        var_kusai00l_dn10: f64,
        var_kusai00l_dn11: f64,
        var_kusai00l_dn12: f64,
        var_kusai00l_dn17: f64,
        var_kusai00l_dn2: f64,
        var_kusai00l_dn6: f64,
        var_kusai00l_dn7: f64,
        var_kusail: f64,
        var_kusail_dn0: f64,
        var_kusail_dn10: f64,
        var_kusail_dn11: f64,
        var_kusail_dn12: f64,
        var_kusail_dn17: f64,
        var_kusail_dn2: f64,
        var_kusail_dn6: f64,
        var_kusail_dn7: f64,
        var_lch: f64,
        var_lch_dn0: f64,
        var_lch_dn10: f64,
        var_lch_dn11: f64,
        var_lch_dn12: f64,
        var_lch_dn17: f64,
        var_lch_dn2: f64,
        var_lch_dn6: f64,
        var_lch_dn7: f64,
        var_lgleff: f64,
        var_mu: f64,
        var_mu_dn0: f64,
        var_mu_dn10: f64,
        var_mu_dn11: f64,
        var_mu_dn12: f64,
        var_mu_dn17: f64,
        var_mu_dn2: f64,
        var_mu_dn6: f64,
        var_mu_dn7: f64,
        var_muun: f64,
        var_muun_dn0: f64,
        var_muun_dn10: f64,
        var_muun_dn11: f64,
        var_muun_dn12: f64,
        var_muun_dn17: f64,
        var_muun_dn2: f64,
        var_muun_dn6: f64,
        var_muun_dn7: f64,
        var_t8__blk1108: f64,
        var_t8__blk1108_dn0: f64,
        var_t8__blk1108_dn10: f64,
        var_t8__blk1108_dn11: f64,
        var_t8__blk1108_dn12: f64,
        var_t8__blk1108_dn17: f64,
        var_t8__blk1108_dn2: f64,
        var_t8__blk1108_dn6: f64,
        var_t8__blk1108_dn7: f64,
        var_vgvt: f64,
        var_vgvt_dn0: f64,
        var_vgvt_dn10: f64,
        var_vgvt_dn11: f64,
        var_vgvt_dn12: f64,
        var_vgvt_dn17: f64,
        var_vgvt_dn2: f64,
        var_vgvt_dn6: f64,
        var_vgvt_dn7: f64,
        var_weff_nf: f64,
        var_cgbe_slot: &mut f64,
        var_crl_f_slot: &mut f64,
        var_crl_f_dn0_slot: &mut f64,
        var_crl_f_dn10_slot: &mut f64,
        var_crl_f_dn11_slot: &mut f64,
        var_crl_f_dn12_slot: &mut f64,
        var_crl_f_dn17_slot: &mut f64,
        var_crl_f_dn2_slot: &mut f64,
        var_crl_f_dn6_slot: &mut f64,
        var_crl_f_dn7_slot: &mut f64,
        var_gamma_slot: &mut f64,
        var_gamma_dn0_slot: &mut f64,
        var_gamma_dn10_slot: &mut f64,
        var_gamma_dn11_slot: &mut f64,
        var_gamma_dn12_slot: &mut f64,
        var_gamma_dn17_slot: &mut f64,
        var_gamma_dn2_slot: &mut f64,
        var_gamma_dn6_slot: &mut f64,
        var_gamma_dn7_slot: &mut f64,
        var_gds0_h2_slot: &mut f64,
        var_gds0_h2_dn0_slot: &mut f64,
        var_gds0_h2_dn10_slot: &mut f64,
        var_gds0_h2_dn11_slot: &mut f64,
        var_gds0_h2_dn12_slot: &mut f64,
        var_gds0_h2_dn17_slot: &mut f64,
        var_gds0_h2_dn2_slot: &mut f64,
        var_gds0_h2_dn6_slot: &mut f64,
        var_gds0_h2_dn7_slot: &mut f64,
        var_gds0_ign_slot: &mut f64,
        var_gds0_ign_dn0_slot: &mut f64,
        var_gds0_ign_dn10_slot: &mut f64,
        var_gds0_ign_dn11_slot: &mut f64,
        var_gds0_ign_dn12_slot: &mut f64,
        var_gds0_ign_dn17_slot: &mut f64,
        var_gds0_ign_dn2_slot: &mut f64,
        var_gds0_ign_dn6_slot: &mut f64,
        var_gds0_ign_dn7_slot: &mut f64,
        var_guard1125_slot: &mut f64,
        var_guard1126_slot: &mut f64,
        var_ids_slot: &mut f64,
        var_ids_dn0_slot: &mut f64,
        var_ids_dn10_slot: &mut f64,
        var_ids_dn11_slot: &mut f64,
        var_ids_dn12_slot: &mut f64,
        var_ids_dn17_slot: &mut f64,
        var_ids_dn2_slot: &mut f64,
        var_ids_dn6_slot: &mut f64,
        var_ids_dn7_slot: &mut f64,
        var_kusai_ig_slot: &mut f64,
        var_kusai_ig_dn0_slot: &mut f64,
        var_kusai_ig_dn10_slot: &mut f64,
        var_kusai_ig_dn11_slot: &mut f64,
        var_kusai_ig_dn12_slot: &mut f64,
        var_kusai_ig_dn17_slot: &mut f64,
        var_kusai_ig_dn2_slot: &mut f64,
        var_kusai_ig_dn6_slot: &mut f64,
        var_kusai_ig_dn7_slot: &mut f64,
        var_mu_ave_slot: &mut f64,
        var_mu_ave_dn0_slot: &mut f64,
        var_mu_ave_dn10_slot: &mut f64,
        var_mu_ave_dn11_slot: &mut f64,
        var_mu_ave_dn12_slot: &mut f64,
        var_mu_ave_dn17_slot: &mut f64,
        var_mu_ave_dn2_slot: &mut f64,
        var_mu_ave_dn6_slot: &mut f64,
        var_mu_ave_dn7_slot: &mut f64,
        var_mud_hoso_slot: &mut f64,
        var_mud_hoso_dn0_slot: &mut f64,
        var_mud_hoso_dn10_slot: &mut f64,
        var_mud_hoso_dn11_slot: &mut f64,
        var_mud_hoso_dn12_slot: &mut f64,
        var_mud_hoso_dn17_slot: &mut f64,
        var_mud_hoso_dn2_slot: &mut f64,
        var_mud_hoso_dn6_slot: &mut f64,
        var_mud_hoso_dn7_slot: &mut f64,
        var_nthrml_slot: &mut f64,
        var_nthrml_dn0_slot: &mut f64,
        var_nthrml_dn10_slot: &mut f64,
        var_nthrml_dn11_slot: &mut f64,
        var_nthrml_dn12_slot: &mut f64,
        var_nthrml_dn17_slot: &mut f64,
        var_nthrml_dn2_slot: &mut f64,
        var_nthrml_dn6_slot: &mut f64,
        var_nthrml_dn7_slot: &mut f64,
        var_sqrtkusail_slot: &mut f64,
        var_sqrtkusail_dn0_slot: &mut f64,
        var_sqrtkusail_dn10_slot: &mut f64,
        var_sqrtkusail_dn11_slot: &mut f64,
        var_sqrtkusail_dn12_slot: &mut f64,
        var_sqrtkusail_dn17_slot: &mut f64,
        var_sqrtkusail_dn2_slot: &mut f64,
        var_sqrtkusail_dn6_slot: &mut f64,
        var_sqrtkusail_dn7_slot: &mut f64,
        var_t0__blk1112_slot: &mut f64,
        var_t0__blk1112_dn0_slot: &mut f64,
        var_t0__blk1112_dn10_slot: &mut f64,
        var_t0__blk1112_dn11_slot: &mut f64,
        var_t0__blk1112_dn12_slot: &mut f64,
        var_t0__blk1112_dn17_slot: &mut f64,
        var_t0__blk1112_dn2_slot: &mut f64,
        var_t0__blk1112_dn6_slot: &mut f64,
        var_t0__blk1112_dn7_slot: &mut f64,
        var_t10__blk1110_slot: &mut f64,
        var_t10__blk1110_dn0_slot: &mut f64,
        var_t10__blk1110_dn10_slot: &mut f64,
        var_t10__blk1110_dn11_slot: &mut f64,
        var_t10__blk1110_dn12_slot: &mut f64,
        var_t10__blk1110_dn17_slot: &mut f64,
        var_t10__blk1110_dn2_slot: &mut f64,
        var_t10__blk1110_dn6_slot: &mut f64,
        var_t10__blk1110_dn7_slot: &mut f64,
        var_t10w_slot: &mut f64,
        var_t10w_dn0_slot: &mut f64,
        var_t10w_dn10_slot: &mut f64,
        var_t10w_dn11_slot: &mut f64,
        var_t10w_dn12_slot: &mut f64,
        var_t10w_dn17_slot: &mut f64,
        var_t10w_dn2_slot: &mut f64,
        var_t10w_dn6_slot: &mut f64,
        var_t10w_dn7_slot: &mut f64,
        var_t11__blk1111_slot: &mut f64,
        var_t11__blk1111_dn0_slot: &mut f64,
        var_t11__blk1111_dn10_slot: &mut f64,
        var_t11__blk1111_dn11_slot: &mut f64,
        var_t11__blk1111_dn12_slot: &mut f64,
        var_t11__blk1111_dn17_slot: &mut f64,
        var_t11__blk1111_dn2_slot: &mut f64,
        var_t11__blk1111_dn6_slot: &mut f64,
        var_t11__blk1111_dn7_slot: &mut f64,
        var_t2__blk1114_slot: &mut f64,
        var_t2__blk1114_dn0_slot: &mut f64,
        var_t2__blk1114_dn10_slot: &mut f64,
        var_t2__blk1114_dn11_slot: &mut f64,
        var_t2__blk1114_dn12_slot: &mut f64,
        var_t2__blk1114_dn17_slot: &mut f64,
        var_t2__blk1114_dn2_slot: &mut f64,
        var_t2__blk1114_dn6_slot: &mut f64,
        var_t2__blk1114_dn7_slot: &mut f64,
        var_t3__blk1115_slot: &mut f64,
        var_t3__blk1115_dn0_slot: &mut f64,
        var_t3__blk1115_dn10_slot: &mut f64,
        var_t3__blk1115_dn11_slot: &mut f64,
        var_t3__blk1115_dn12_slot: &mut f64,
        var_t3__blk1115_dn17_slot: &mut f64,
        var_t3__blk1115_dn2_slot: &mut f64,
        var_t3__blk1115_dn6_slot: &mut f64,
        var_t3__blk1115_dn7_slot: &mut f64,
        var_t4__blk1116_slot: &mut f64,
        var_t4__blk1116_dn0_slot: &mut f64,
        var_t4__blk1116_dn10_slot: &mut f64,
        var_t4__blk1116_dn11_slot: &mut f64,
        var_t4__blk1116_dn12_slot: &mut f64,
        var_t4__blk1116_dn17_slot: &mut f64,
        var_t4__blk1116_dn2_slot: &mut f64,
        var_t4__blk1116_dn6_slot: &mut f64,
        var_t4__blk1116_dn7_slot: &mut f64,
        var_t5__blk1117_slot: &mut f64,
        var_t5__blk1117_dn0_slot: &mut f64,
        var_t5__blk1117_dn10_slot: &mut f64,
        var_t5__blk1117_dn11_slot: &mut f64,
        var_t5__blk1117_dn12_slot: &mut f64,
        var_t5__blk1117_dn17_slot: &mut f64,
        var_t5__blk1117_dn2_slot: &mut f64,
        var_t5__blk1117_dn6_slot: &mut f64,
        var_t5__blk1117_dn7_slot: &mut f64,
        var_t7w_slot: &mut f64,
        var_t7w_dn0_slot: &mut f64,
        var_t7w_dn10_slot: &mut f64,
        var_t7w_dn11_slot: &mut f64,
        var_t7w_dn12_slot: &mut f64,
        var_t7w_dn17_slot: &mut f64,
        var_t7w_dn2_slot: &mut f64,
        var_t7w_dn6_slot: &mut f64,
        var_t7w_dn7_slot: &mut f64,
        var_t9__blk1109_slot: &mut f64,
        var_t9__blk1109_dn0_slot: &mut f64,
        var_t9__blk1109_dn10_slot: &mut f64,
        var_t9__blk1109_dn11_slot: &mut f64,
        var_t9__blk1109_dn12_slot: &mut f64,
        var_t9__blk1109_dn17_slot: &mut f64,
        var_t9__blk1109_dn2_slot: &mut f64,
        var_t9__blk1109_dn6_slot: &mut f64,
        var_t9__blk1109_dn7_slot: &mut f64,
    ) {
        let mut var_cgbe: f64 = *var_cgbe_slot;
        let mut var_crl_f: f64 = *var_crl_f_slot;
        let mut var_crl_f_dn0: f64 = *var_crl_f_dn0_slot;
        let mut var_crl_f_dn10: f64 = *var_crl_f_dn10_slot;
        let mut var_crl_f_dn11: f64 = *var_crl_f_dn11_slot;
        let mut var_crl_f_dn12: f64 = *var_crl_f_dn12_slot;
        let mut var_crl_f_dn17: f64 = *var_crl_f_dn17_slot;
        let mut var_crl_f_dn2: f64 = *var_crl_f_dn2_slot;
        let mut var_crl_f_dn6: f64 = *var_crl_f_dn6_slot;
        let mut var_crl_f_dn7: f64 = *var_crl_f_dn7_slot;
        let mut var_gamma: f64 = *var_gamma_slot;
        let mut var_gamma_dn0: f64 = *var_gamma_dn0_slot;
        let mut var_gamma_dn10: f64 = *var_gamma_dn10_slot;
        let mut var_gamma_dn11: f64 = *var_gamma_dn11_slot;
        let mut var_gamma_dn12: f64 = *var_gamma_dn12_slot;
        let mut var_gamma_dn17: f64 = *var_gamma_dn17_slot;
        let mut var_gamma_dn2: f64 = *var_gamma_dn2_slot;
        let mut var_gamma_dn6: f64 = *var_gamma_dn6_slot;
        let mut var_gamma_dn7: f64 = *var_gamma_dn7_slot;
        let mut var_gds0_h2: f64 = *var_gds0_h2_slot;
        let mut var_gds0_h2_dn0: f64 = *var_gds0_h2_dn0_slot;
        let mut var_gds0_h2_dn10: f64 = *var_gds0_h2_dn10_slot;
        let mut var_gds0_h2_dn11: f64 = *var_gds0_h2_dn11_slot;
        let mut var_gds0_h2_dn12: f64 = *var_gds0_h2_dn12_slot;
        let mut var_gds0_h2_dn17: f64 = *var_gds0_h2_dn17_slot;
        let mut var_gds0_h2_dn2: f64 = *var_gds0_h2_dn2_slot;
        let mut var_gds0_h2_dn6: f64 = *var_gds0_h2_dn6_slot;
        let mut var_gds0_h2_dn7: f64 = *var_gds0_h2_dn7_slot;
        let mut var_gds0_ign: f64 = *var_gds0_ign_slot;
        let mut var_gds0_ign_dn0: f64 = *var_gds0_ign_dn0_slot;
        let mut var_gds0_ign_dn10: f64 = *var_gds0_ign_dn10_slot;
        let mut var_gds0_ign_dn11: f64 = *var_gds0_ign_dn11_slot;
        let mut var_gds0_ign_dn12: f64 = *var_gds0_ign_dn12_slot;
        let mut var_gds0_ign_dn17: f64 = *var_gds0_ign_dn17_slot;
        let mut var_gds0_ign_dn2: f64 = *var_gds0_ign_dn2_slot;
        let mut var_gds0_ign_dn6: f64 = *var_gds0_ign_dn6_slot;
        let mut var_gds0_ign_dn7: f64 = *var_gds0_ign_dn7_slot;
        let mut var_guard1125: f64 = *var_guard1125_slot;
        let mut var_guard1126: f64 = *var_guard1126_slot;
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn0: f64 = *var_ids_dn0_slot;
        let mut var_ids_dn10: f64 = *var_ids_dn10_slot;
        let mut var_ids_dn11: f64 = *var_ids_dn11_slot;
        let mut var_ids_dn12: f64 = *var_ids_dn12_slot;
        let mut var_ids_dn17: f64 = *var_ids_dn17_slot;
        let mut var_ids_dn2: f64 = *var_ids_dn2_slot;
        let mut var_ids_dn6: f64 = *var_ids_dn6_slot;
        let mut var_ids_dn7: f64 = *var_ids_dn7_slot;
        let mut var_kusai_ig: f64 = *var_kusai_ig_slot;
        let mut var_kusai_ig_dn0: f64 = *var_kusai_ig_dn0_slot;
        let mut var_kusai_ig_dn10: f64 = *var_kusai_ig_dn10_slot;
        let mut var_kusai_ig_dn11: f64 = *var_kusai_ig_dn11_slot;
        let mut var_kusai_ig_dn12: f64 = *var_kusai_ig_dn12_slot;
        let mut var_kusai_ig_dn17: f64 = *var_kusai_ig_dn17_slot;
        let mut var_kusai_ig_dn2: f64 = *var_kusai_ig_dn2_slot;
        let mut var_kusai_ig_dn6: f64 = *var_kusai_ig_dn6_slot;
        let mut var_kusai_ig_dn7: f64 = *var_kusai_ig_dn7_slot;
        let mut var_mu_ave: f64 = *var_mu_ave_slot;
        let mut var_mu_ave_dn0: f64 = *var_mu_ave_dn0_slot;
        let mut var_mu_ave_dn10: f64 = *var_mu_ave_dn10_slot;
        let mut var_mu_ave_dn11: f64 = *var_mu_ave_dn11_slot;
        let mut var_mu_ave_dn12: f64 = *var_mu_ave_dn12_slot;
        let mut var_mu_ave_dn17: f64 = *var_mu_ave_dn17_slot;
        let mut var_mu_ave_dn2: f64 = *var_mu_ave_dn2_slot;
        let mut var_mu_ave_dn6: f64 = *var_mu_ave_dn6_slot;
        let mut var_mu_ave_dn7: f64 = *var_mu_ave_dn7_slot;
        let mut var_mud_hoso: f64 = *var_mud_hoso_slot;
        let mut var_mud_hoso_dn0: f64 = *var_mud_hoso_dn0_slot;
        let mut var_mud_hoso_dn10: f64 = *var_mud_hoso_dn10_slot;
        let mut var_mud_hoso_dn11: f64 = *var_mud_hoso_dn11_slot;
        let mut var_mud_hoso_dn12: f64 = *var_mud_hoso_dn12_slot;
        let mut var_mud_hoso_dn17: f64 = *var_mud_hoso_dn17_slot;
        let mut var_mud_hoso_dn2: f64 = *var_mud_hoso_dn2_slot;
        let mut var_mud_hoso_dn6: f64 = *var_mud_hoso_dn6_slot;
        let mut var_mud_hoso_dn7: f64 = *var_mud_hoso_dn7_slot;
        let mut var_nthrml: f64 = *var_nthrml_slot;
        let mut var_nthrml_dn0: f64 = *var_nthrml_dn0_slot;
        let mut var_nthrml_dn10: f64 = *var_nthrml_dn10_slot;
        let mut var_nthrml_dn11: f64 = *var_nthrml_dn11_slot;
        let mut var_nthrml_dn12: f64 = *var_nthrml_dn12_slot;
        let mut var_nthrml_dn17: f64 = *var_nthrml_dn17_slot;
        let mut var_nthrml_dn2: f64 = *var_nthrml_dn2_slot;
        let mut var_nthrml_dn6: f64 = *var_nthrml_dn6_slot;
        let mut var_nthrml_dn7: f64 = *var_nthrml_dn7_slot;
        let mut var_sqrtkusail: f64 = *var_sqrtkusail_slot;
        let mut var_sqrtkusail_dn0: f64 = *var_sqrtkusail_dn0_slot;
        let mut var_sqrtkusail_dn10: f64 = *var_sqrtkusail_dn10_slot;
        let mut var_sqrtkusail_dn11: f64 = *var_sqrtkusail_dn11_slot;
        let mut var_sqrtkusail_dn12: f64 = *var_sqrtkusail_dn12_slot;
        let mut var_sqrtkusail_dn17: f64 = *var_sqrtkusail_dn17_slot;
        let mut var_sqrtkusail_dn2: f64 = *var_sqrtkusail_dn2_slot;
        let mut var_sqrtkusail_dn6: f64 = *var_sqrtkusail_dn6_slot;
        let mut var_sqrtkusail_dn7: f64 = *var_sqrtkusail_dn7_slot;
        let mut var_t0__blk1112: f64 = *var_t0__blk1112_slot;
        let mut var_t0__blk1112_dn0: f64 = *var_t0__blk1112_dn0_slot;
        let mut var_t0__blk1112_dn10: f64 = *var_t0__blk1112_dn10_slot;
        let mut var_t0__blk1112_dn11: f64 = *var_t0__blk1112_dn11_slot;
        let mut var_t0__blk1112_dn12: f64 = *var_t0__blk1112_dn12_slot;
        let mut var_t0__blk1112_dn17: f64 = *var_t0__blk1112_dn17_slot;
        let mut var_t0__blk1112_dn2: f64 = *var_t0__blk1112_dn2_slot;
        let mut var_t0__blk1112_dn6: f64 = *var_t0__blk1112_dn6_slot;
        let mut var_t0__blk1112_dn7: f64 = *var_t0__blk1112_dn7_slot;
        let mut var_t10__blk1110: f64 = *var_t10__blk1110_slot;
        let mut var_t10__blk1110_dn0: f64 = *var_t10__blk1110_dn0_slot;
        let mut var_t10__blk1110_dn10: f64 = *var_t10__blk1110_dn10_slot;
        let mut var_t10__blk1110_dn11: f64 = *var_t10__blk1110_dn11_slot;
        let mut var_t10__blk1110_dn12: f64 = *var_t10__blk1110_dn12_slot;
        let mut var_t10__blk1110_dn17: f64 = *var_t10__blk1110_dn17_slot;
        let mut var_t10__blk1110_dn2: f64 = *var_t10__blk1110_dn2_slot;
        let mut var_t10__blk1110_dn6: f64 = *var_t10__blk1110_dn6_slot;
        let mut var_t10__blk1110_dn7: f64 = *var_t10__blk1110_dn7_slot;
        let mut var_t10w: f64 = *var_t10w_slot;
        let mut var_t10w_dn0: f64 = *var_t10w_dn0_slot;
        let mut var_t10w_dn10: f64 = *var_t10w_dn10_slot;
        let mut var_t10w_dn11: f64 = *var_t10w_dn11_slot;
        let mut var_t10w_dn12: f64 = *var_t10w_dn12_slot;
        let mut var_t10w_dn17: f64 = *var_t10w_dn17_slot;
        let mut var_t10w_dn2: f64 = *var_t10w_dn2_slot;
        let mut var_t10w_dn6: f64 = *var_t10w_dn6_slot;
        let mut var_t10w_dn7: f64 = *var_t10w_dn7_slot;
        let mut var_t11__blk1111: f64 = *var_t11__blk1111_slot;
        let mut var_t11__blk1111_dn0: f64 = *var_t11__blk1111_dn0_slot;
        let mut var_t11__blk1111_dn10: f64 = *var_t11__blk1111_dn10_slot;
        let mut var_t11__blk1111_dn11: f64 = *var_t11__blk1111_dn11_slot;
        let mut var_t11__blk1111_dn12: f64 = *var_t11__blk1111_dn12_slot;
        let mut var_t11__blk1111_dn17: f64 = *var_t11__blk1111_dn17_slot;
        let mut var_t11__blk1111_dn2: f64 = *var_t11__blk1111_dn2_slot;
        let mut var_t11__blk1111_dn6: f64 = *var_t11__blk1111_dn6_slot;
        let mut var_t11__blk1111_dn7: f64 = *var_t11__blk1111_dn7_slot;
        let mut var_t2__blk1114: f64 = *var_t2__blk1114_slot;
        let mut var_t2__blk1114_dn0: f64 = *var_t2__blk1114_dn0_slot;
        let mut var_t2__blk1114_dn10: f64 = *var_t2__blk1114_dn10_slot;
        let mut var_t2__blk1114_dn11: f64 = *var_t2__blk1114_dn11_slot;
        let mut var_t2__blk1114_dn12: f64 = *var_t2__blk1114_dn12_slot;
        let mut var_t2__blk1114_dn17: f64 = *var_t2__blk1114_dn17_slot;
        let mut var_t2__blk1114_dn2: f64 = *var_t2__blk1114_dn2_slot;
        let mut var_t2__blk1114_dn6: f64 = *var_t2__blk1114_dn6_slot;
        let mut var_t2__blk1114_dn7: f64 = *var_t2__blk1114_dn7_slot;
        let mut var_t3__blk1115: f64 = *var_t3__blk1115_slot;
        let mut var_t3__blk1115_dn0: f64 = *var_t3__blk1115_dn0_slot;
        let mut var_t3__blk1115_dn10: f64 = *var_t3__blk1115_dn10_slot;
        let mut var_t3__blk1115_dn11: f64 = *var_t3__blk1115_dn11_slot;
        let mut var_t3__blk1115_dn12: f64 = *var_t3__blk1115_dn12_slot;
        let mut var_t3__blk1115_dn17: f64 = *var_t3__blk1115_dn17_slot;
        let mut var_t3__blk1115_dn2: f64 = *var_t3__blk1115_dn2_slot;
        let mut var_t3__blk1115_dn6: f64 = *var_t3__blk1115_dn6_slot;
        let mut var_t3__blk1115_dn7: f64 = *var_t3__blk1115_dn7_slot;
        let mut var_t4__blk1116: f64 = *var_t4__blk1116_slot;
        let mut var_t4__blk1116_dn0: f64 = *var_t4__blk1116_dn0_slot;
        let mut var_t4__blk1116_dn10: f64 = *var_t4__blk1116_dn10_slot;
        let mut var_t4__blk1116_dn11: f64 = *var_t4__blk1116_dn11_slot;
        let mut var_t4__blk1116_dn12: f64 = *var_t4__blk1116_dn12_slot;
        let mut var_t4__blk1116_dn17: f64 = *var_t4__blk1116_dn17_slot;
        let mut var_t4__blk1116_dn2: f64 = *var_t4__blk1116_dn2_slot;
        let mut var_t4__blk1116_dn6: f64 = *var_t4__blk1116_dn6_slot;
        let mut var_t4__blk1116_dn7: f64 = *var_t4__blk1116_dn7_slot;
        let mut var_t5__blk1117: f64 = *var_t5__blk1117_slot;
        let mut var_t5__blk1117_dn0: f64 = *var_t5__blk1117_dn0_slot;
        let mut var_t5__blk1117_dn10: f64 = *var_t5__blk1117_dn10_slot;
        let mut var_t5__blk1117_dn11: f64 = *var_t5__blk1117_dn11_slot;
        let mut var_t5__blk1117_dn12: f64 = *var_t5__blk1117_dn12_slot;
        let mut var_t5__blk1117_dn17: f64 = *var_t5__blk1117_dn17_slot;
        let mut var_t5__blk1117_dn2: f64 = *var_t5__blk1117_dn2_slot;
        let mut var_t5__blk1117_dn6: f64 = *var_t5__blk1117_dn6_slot;
        let mut var_t5__blk1117_dn7: f64 = *var_t5__blk1117_dn7_slot;
        let mut var_t7w: f64 = *var_t7w_slot;
        let mut var_t7w_dn0: f64 = *var_t7w_dn0_slot;
        let mut var_t7w_dn10: f64 = *var_t7w_dn10_slot;
        let mut var_t7w_dn11: f64 = *var_t7w_dn11_slot;
        let mut var_t7w_dn12: f64 = *var_t7w_dn12_slot;
        let mut var_t7w_dn17: f64 = *var_t7w_dn17_slot;
        let mut var_t7w_dn2: f64 = *var_t7w_dn2_slot;
        let mut var_t7w_dn6: f64 = *var_t7w_dn6_slot;
        let mut var_t7w_dn7: f64 = *var_t7w_dn7_slot;
        let mut var_t9__blk1109: f64 = *var_t9__blk1109_slot;
        let mut var_t9__blk1109_dn0: f64 = *var_t9__blk1109_dn0_slot;
        let mut var_t9__blk1109_dn10: f64 = *var_t9__blk1109_dn10_slot;
        let mut var_t9__blk1109_dn11: f64 = *var_t9__blk1109_dn11_slot;
        let mut var_t9__blk1109_dn12: f64 = *var_t9__blk1109_dn12_slot;
        let mut var_t9__blk1109_dn17: f64 = *var_t9__blk1109_dn17_slot;
        let mut var_t9__blk1109_dn2: f64 = *var_t9__blk1109_dn2_slot;
        let mut var_t9__blk1109_dn6: f64 = *var_t9__blk1109_dn6_slot;
        let mut var_t9__blk1109_dn7: f64 = *var_t9__blk1109_dn7_slot;

        let (assign33670_e48839, assign33670_e48839_d_n0, assign33670_e48839_d_n2, assign33670_e48839_d_n6, assign33670_e48839_d_n7, assign33670_e48839_d_n10, assign33670_e48839_d_n11, assign33670_e48839_d_n12, assign33670_e48839_d_n17,) = {
    if (var_guard1122 != 0.0) {
        let assign33670_e48837: f64 = (1.0 + var_t8__blk1108);
        (assign33670_e48837, var_t8__blk1108_dn0, var_t8__blk1108_dn2, var_t8__blk1108_dn6, var_t8__blk1108_dn7, var_t8__blk1108_dn10, var_t8__blk1108_dn11, var_t8__blk1108_dn12, var_t8__blk1108_dn17,)
    } else {
        (var_t9__blk1109, var_t9__blk1109_dn0, var_t9__blk1109_dn2, var_t9__blk1109_dn6, var_t9__blk1109_dn7, var_t9__blk1109_dn10, var_t9__blk1109_dn11, var_t9__blk1109_dn12, var_t9__blk1109_dn17,)
    }
};
        var_t9__blk1109 = assign33670_e48839;
        var_t9__blk1109_dn0 = assign33670_e48839_d_n0;
        var_t9__blk1109_dn2 = assign33670_e48839_d_n2;
        var_t9__blk1109_dn6 = assign33670_e48839_d_n6;
        var_t9__blk1109_dn7 = assign33670_e48839_d_n7;
        var_t9__blk1109_dn10 = assign33670_e48839_d_n10;
        var_t9__blk1109_dn11 = assign33670_e48839_d_n11;
        var_t9__blk1109_dn12 = assign33670_e48839_d_n12;
        var_t9__blk1109_dn17 = assign33670_e48839_d_n17;

        let (assign33680_e48850, assign33680_e48850_d_n0, assign33680_e48850_d_n2, assign33680_e48850_d_n6, assign33680_e48850_d_n7, assign33680_e48850_d_n10, assign33680_e48850_d_n11, assign33680_e48850_d_n12, assign33680_e48850_d_n17,) = {
    if (var_guard1122 != 0.0) {
        let assign33680_e48843: f64 = (-1.0);
        let assign33680_e48845: f64 = (assign33680_e48843 / p.p113);
        let assign33680_e48847: f64 = (assign33680_e48845 - 1.0);
        let assign33680_e48848: f64 = (var_t9__blk1109).powf(assign33680_e48847);
        (assign33680_e48848, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((var_t9__blk1109).powf(assign33680_e48847 - 1.0) * var_t9__blk1109_dn0)) } } else { (assign33680_e48848 * (assign33680_e48847 * (var_t9__blk1109_dn0 / var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((var_t9__blk1109).powf(assign33680_e48847 - 1.0) * var_t9__blk1109_dn2)) } } else { (assign33680_e48848 * (assign33680_e48847 * (var_t9__blk1109_dn2 / var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((var_t9__blk1109).powf(assign33680_e48847 - 1.0) * var_t9__blk1109_dn6)) } } else { (assign33680_e48848 * (assign33680_e48847 * (var_t9__blk1109_dn6 / var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((var_t9__blk1109).powf(assign33680_e48847 - 1.0) * var_t9__blk1109_dn7)) } } else { (assign33680_e48848 * (assign33680_e48847 * (var_t9__blk1109_dn7 / var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((var_t9__blk1109).powf(assign33680_e48847 - 1.0) * var_t9__blk1109_dn10)) } } else { (assign33680_e48848 * (assign33680_e48847 * (var_t9__blk1109_dn10 / var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((var_t9__blk1109).powf(assign33680_e48847 - 1.0) * var_t9__blk1109_dn11)) } } else { (assign33680_e48848 * (assign33680_e48847 * (var_t9__blk1109_dn11 / var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((var_t9__blk1109).powf(assign33680_e48847 - 1.0) * var_t9__blk1109_dn12)) } } else { (assign33680_e48848 * (assign33680_e48847 * (var_t9__blk1109_dn12 / var_t9__blk1109))) }, if 0.0 == 0.0 && ((assign33680_e48847) as f64).is_finite() && ((assign33680_e48847) as f64).fract() == 0.0 { if assign33680_e48847 == 0.0 { 0.0 } else { (assign33680_e48847 * ((var_t9__blk1109).powf(assign33680_e48847 - 1.0) * var_t9__blk1109_dn17)) } } else { (assign33680_e48848 * (assign33680_e48847 * (var_t9__blk1109_dn17 / var_t9__blk1109))) },)
    } else {
        (var_t10__blk1110, var_t10__blk1110_dn0, var_t10__blk1110_dn2, var_t10__blk1110_dn6, var_t10__blk1110_dn7, var_t10__blk1110_dn10, var_t10__blk1110_dn11, var_t10__blk1110_dn12, var_t10__blk1110_dn17,)
    }
};
        var_t10__blk1110 = assign33680_e48850;
        var_t10__blk1110_dn0 = assign33680_e48850_d_n0;
        var_t10__blk1110_dn2 = assign33680_e48850_d_n2;
        var_t10__blk1110_dn6 = assign33680_e48850_d_n6;
        var_t10__blk1110_dn7 = assign33680_e48850_d_n7;
        var_t10__blk1110_dn10 = assign33680_e48850_d_n10;
        var_t10__blk1110_dn11 = assign33680_e48850_d_n11;
        var_t10__blk1110_dn12 = assign33680_e48850_d_n12;
        var_t10__blk1110_dn17 = assign33680_e48850_d_n17;

        let (assign33690_e48856, assign33690_e48856_d_n0, assign33690_e48856_d_n2, assign33690_e48856_d_n6, assign33690_e48856_d_n7, assign33690_e48856_d_n10, assign33690_e48856_d_n11, assign33690_e48856_d_n12, assign33690_e48856_d_n17,) = {
    if (var_guard1122 != 0.0) {
        let assign33690_e48854: f64 = (var_t9__blk1109 * var_t10__blk1110);
        (assign33690_e48854, ((var_t9__blk1109_dn0 * var_t10__blk1110) + (var_t9__blk1109 * var_t10__blk1110_dn0)), ((var_t9__blk1109_dn2 * var_t10__blk1110) + (var_t9__blk1109 * var_t10__blk1110_dn2)), ((var_t9__blk1109_dn6 * var_t10__blk1110) + (var_t9__blk1109 * var_t10__blk1110_dn6)), ((var_t9__blk1109_dn7 * var_t10__blk1110) + (var_t9__blk1109 * var_t10__blk1110_dn7)), ((var_t9__blk1109_dn10 * var_t10__blk1110) + (var_t9__blk1109 * var_t10__blk1110_dn10)), ((var_t9__blk1109_dn11 * var_t10__blk1110) + (var_t9__blk1109 * var_t10__blk1110_dn11)), ((var_t9__blk1109_dn12 * var_t10__blk1110) + (var_t9__blk1109 * var_t10__blk1110_dn12)), ((var_t9__blk1109_dn17 * var_t10__blk1110) + (var_t9__blk1109 * var_t10__blk1110_dn17)),)
    } else {
        (var_t11__blk1111, var_t11__blk1111_dn0, var_t11__blk1111_dn2, var_t11__blk1111_dn6, var_t11__blk1111_dn7, var_t11__blk1111_dn10, var_t11__blk1111_dn11, var_t11__blk1111_dn12, var_t11__blk1111_dn17,)
    }
};
        var_t11__blk1111 = assign33690_e48856;
        var_t11__blk1111_dn0 = assign33690_e48856_d_n0;
        var_t11__blk1111_dn2 = assign33690_e48856_d_n2;
        var_t11__blk1111_dn6 = assign33690_e48856_d_n6;
        var_t11__blk1111_dn7 = assign33690_e48856_d_n7;
        var_t11__blk1111_dn10 = assign33690_e48856_d_n10;
        var_t11__blk1111_dn11 = assign33690_e48856_d_n11;
        var_t11__blk1111_dn12 = assign33690_e48856_d_n12;
        var_t11__blk1111_dn17 = assign33690_e48856_d_n17;

        let (assign33700_e48862, assign33700_e48862_d_n0, assign33700_e48862_d_n2, assign33700_e48862_d_n6, assign33700_e48862_d_n7, assign33700_e48862_d_n10, assign33700_e48862_d_n11, assign33700_e48862_d_n12, assign33700_e48862_d_n17,) = {
    if (var_guard1122 != 0.0) {
        let assign33700_e48860: f64 = (var_muun * var_t11__blk1111);
        (assign33700_e48860, ((var_muun_dn0 * var_t11__blk1111) + (var_muun * var_t11__blk1111_dn0)), ((var_muun_dn2 * var_t11__blk1111) + (var_muun * var_t11__blk1111_dn2)), ((var_muun_dn6 * var_t11__blk1111) + (var_muun * var_t11__blk1111_dn6)), ((var_muun_dn7 * var_t11__blk1111) + (var_muun * var_t11__blk1111_dn7)), ((var_muun_dn10 * var_t11__blk1111) + (var_muun * var_t11__blk1111_dn10)), ((var_muun_dn11 * var_t11__blk1111) + (var_muun * var_t11__blk1111_dn11)), ((var_muun_dn12 * var_t11__blk1111) + (var_muun * var_t11__blk1111_dn12)), ((var_muun_dn17 * var_t11__blk1111) + (var_muun * var_t11__blk1111_dn17)),)
    } else {
        (var_mud_hoso, var_mud_hoso_dn0, var_mud_hoso_dn2, var_mud_hoso_dn6, var_mud_hoso_dn7, var_mud_hoso_dn10, var_mud_hoso_dn11, var_mud_hoso_dn12, var_mud_hoso_dn17,)
    }
};
        var_mud_hoso = assign33700_e48862;
        var_mud_hoso_dn0 = assign33700_e48862_d_n0;
        var_mud_hoso_dn2 = assign33700_e48862_d_n2;
        var_mud_hoso_dn6 = assign33700_e48862_d_n6;
        var_mud_hoso_dn7 = assign33700_e48862_d_n7;
        var_mud_hoso_dn10 = assign33700_e48862_d_n10;
        var_mud_hoso_dn11 = assign33700_e48862_d_n11;
        var_mud_hoso_dn12 = assign33700_e48862_d_n12;
        var_mud_hoso_dn17 = assign33700_e48862_d_n17;

        let (assign33710_e48870, assign33710_e48870_d_n0, assign33710_e48870_d_n2, assign33710_e48870_d_n6, assign33710_e48870_d_n7, assign33710_e48870_d_n10, assign33710_e48870_d_n11, assign33710_e48870_d_n12, assign33710_e48870_d_n17,) = {
    if (var_guard1122 != 0.0) {
        let assign33710_e48866: f64 = (var_mu + var_mud_hoso);
        let assign33710_e48868: f64 = (assign33710_e48866 / 2.0);
        (assign33710_e48868, ((var_mu_dn0 + var_mud_hoso_dn0) / 2.0), ((var_mu_dn2 + var_mud_hoso_dn2) / 2.0), ((var_mu_dn6 + var_mud_hoso_dn6) / 2.0), ((var_mu_dn7 + var_mud_hoso_dn7) / 2.0), ((var_mu_dn10 + var_mud_hoso_dn10) / 2.0), ((var_mu_dn11 + var_mud_hoso_dn11) / 2.0), ((var_mu_dn12 + var_mud_hoso_dn12) / 2.0), ((var_mu_dn17 + var_mud_hoso_dn17) / 2.0),)
    } else {
        (var_mu_ave, var_mu_ave_dn0, var_mu_ave_dn2, var_mu_ave_dn6, var_mu_ave_dn7, var_mu_ave_dn10, var_mu_ave_dn11, var_mu_ave_dn12, var_mu_ave_dn17,)
    }
};
        var_mu_ave = assign33710_e48870;
        var_mu_ave_dn0 = assign33710_e48870_d_n0;
        var_mu_ave_dn2 = assign33710_e48870_d_n2;
        var_mu_ave_dn6 = assign33710_e48870_d_n6;
        var_mu_ave_dn7 = assign33710_e48870_d_n7;
        var_mu_ave_dn10 = assign33710_e48870_d_n10;
        var_mu_ave_dn11 = assign33710_e48870_d_n11;
        var_mu_ave_dn12 = assign33710_e48870_d_n12;
        var_mu_ave_dn17 = assign33710_e48870_d_n17;

        let (assign33720_e48876, assign33720_e48876_d_n0, assign33720_e48876_d_n2, assign33720_e48876_d_n6, assign33720_e48876_d_n7, assign33720_e48876_d_n10, assign33720_e48876_d_n11, assign33720_e48876_d_n12, assign33720_e48876_d_n17,) = {
    if (var_guard1122 != 0.0) {
        let assign33720_e48874: f64 = (var_alpha * var_alpha);
        (assign33720_e48874, ((var_alpha_dn0 * var_alpha) + (var_alpha * var_alpha_dn0)), ((var_alpha_dn2 * var_alpha) + (var_alpha * var_alpha_dn2)), ((var_alpha_dn6 * var_alpha) + (var_alpha * var_alpha_dn6)), ((var_alpha_dn7 * var_alpha) + (var_alpha * var_alpha_dn7)), ((var_alpha_dn10 * var_alpha) + (var_alpha * var_alpha_dn10)), ((var_alpha_dn11 * var_alpha) + (var_alpha * var_alpha_dn11)), ((var_alpha_dn12 * var_alpha) + (var_alpha * var_alpha_dn12)), ((var_alpha_dn17 * var_alpha) + (var_alpha * var_alpha_dn17)),)
    } else {
        (var_t0__blk1112, var_t0__blk1112_dn0, var_t0__blk1112_dn2, var_t0__blk1112_dn6, var_t0__blk1112_dn7, var_t0__blk1112_dn10, var_t0__blk1112_dn11, var_t0__blk1112_dn12, var_t0__blk1112_dn17,)
    }
};
        var_t0__blk1112 = assign33720_e48876;
        var_t0__blk1112_dn0 = assign33720_e48876_d_n0;
        var_t0__blk1112_dn2 = assign33720_e48876_d_n2;
        var_t0__blk1112_dn6 = assign33720_e48876_d_n6;
        var_t0__blk1112_dn7 = assign33720_e48876_d_n7;
        var_t0__blk1112_dn10 = assign33720_e48876_d_n10;
        var_t0__blk1112_dn11 = assign33720_e48876_d_n11;
        var_t0__blk1112_dn12 = assign33720_e48876_d_n12;
        var_t0__blk1112_dn17 = assign33720_e48876_d_n17;

        let (assign33730_e48938, assign33730_e48938_d_n0, assign33730_e48938_d_n2, assign33730_e48938_d_n6, assign33730_e48938_d_n7, assign33730_e48938_d_n10, assign33730_e48938_d_n11, assign33730_e48938_d_n12, assign33730_e48938_d_n17,) = {
    if (var_guard1122 != 0.0) {
        let assign33730_e48880: f64 = (var_weff_nf * var_c_fox);
        let assign33730_e48882: f64 = (assign33730_e48880 * var_vgvt);
        let assign33730_e48884: f64 = (assign33730_e48882 * var_mu);
        let assign33730_e48888: f64 = (3.0 * var_alpha);
        let assign33730_e48889: f64 = (1.0 + assign33730_e48888);
        let assign33730_e48892: f64 = (6.0 * var_t0__blk1112);
        let assign33730_e48893: f64 = (assign33730_e48889 + assign33730_e48892);
        let assign33730_e48895: f64 = (assign33730_e48893 * var_mud_hoso);
        let assign33730_e48897: f64 = (assign33730_e48895 * var_mud_hoso);
        let assign33730_e48901: f64 = (4.0 * var_alpha);
        let assign33730_e48902: f64 = (3.0 + assign33730_e48901);
        let assign33730_e48905: f64 = (3.0 * var_t0__blk1112);
        let assign33730_e48906: f64 = (assign33730_e48902 + assign33730_e48905);
        let assign33730_e48908: f64 = (assign33730_e48906 * var_mud_hoso);
        let assign33730_e48910: f64 = (assign33730_e48908 * var_mu);
        let assign33730_e48911: f64 = (assign33730_e48897 + assign33730_e48910);
        let assign33730_e48915: f64 = (3.0 * var_alpha);
        let assign33730_e48916: f64 = (6.0 + assign33730_e48915);
        let assign33730_e48918: f64 = (assign33730_e48916 + var_t0__blk1112);
        let assign33730_e48920: f64 = (assign33730_e48918 * var_mu);
        let assign33730_e48922: f64 = (assign33730_e48920 * var_mu);
        let assign33730_e48923: f64 = (assign33730_e48911 + assign33730_e48922);
        let assign33730_e48924: f64 = (assign33730_e48884 * assign33730_e48923);
        let assign33730_e48927: f64 = (15.0 * var_lch);
        let assign33730_e48930: f64 = (1.0 + var_alpha);
        let assign33730_e48931: f64 = (assign33730_e48927 * assign33730_e48930);
        let assign33730_e48933: f64 = (assign33730_e48931 * var_mu_ave);
        let assign33730_e48935: f64 = (assign33730_e48933 * var_mu_ave);
        let assign33730_e48936: f64 = (assign33730_e48924 / assign33730_e48935);
        (assign33730_e48936, ((((((((((var_weff_nf * var_c_fox_dn0) * var_vgvt) + (assign33730_e48880 * var_vgvt_dn0)) * var_mu) + (assign33730_e48882 * var_mu_dn0)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * var_alpha_dn0) + (6.0 * var_t0__blk1112_dn0)) * var_mud_hoso) + (assign33730_e48893 * var_mud_hoso_dn0)) * var_mud_hoso) + (assign33730_e48895 * var_mud_hoso_dn0)) + ((((((4.0 * var_alpha_dn0) + (3.0 * var_t0__blk1112_dn0)) * var_mud_hoso) + (assign33730_e48906 * var_mud_hoso_dn0)) * var_mu) + (assign33730_e48908 * var_mu_dn0))) + ((((((3.0 * var_alpha_dn0) + var_t0__blk1112_dn0) * var_mu) + (assign33730_e48918 * var_mu_dn0)) * var_mu) + (assign33730_e48920 * var_mu_dn0))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * var_lch_dn0) * assign33730_e48930) + (assign33730_e48927 * var_alpha_dn0)) * var_mu_ave) + (assign33730_e48931 * var_mu_ave_dn0)) * var_mu_ave) + (assign33730_e48933 * var_mu_ave_dn0)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((var_weff_nf * var_c_fox_dn2) * var_vgvt) + (assign33730_e48880 * var_vgvt_dn2)) * var_mu) + (assign33730_e48882 * var_mu_dn2)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * var_alpha_dn2) + (6.0 * var_t0__blk1112_dn2)) * var_mud_hoso) + (assign33730_e48893 * var_mud_hoso_dn2)) * var_mud_hoso) + (assign33730_e48895 * var_mud_hoso_dn2)) + ((((((4.0 * var_alpha_dn2) + (3.0 * var_t0__blk1112_dn2)) * var_mud_hoso) + (assign33730_e48906 * var_mud_hoso_dn2)) * var_mu) + (assign33730_e48908 * var_mu_dn2))) + ((((((3.0 * var_alpha_dn2) + var_t0__blk1112_dn2) * var_mu) + (assign33730_e48918 * var_mu_dn2)) * var_mu) + (assign33730_e48920 * var_mu_dn2))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * var_lch_dn2) * assign33730_e48930) + (assign33730_e48927 * var_alpha_dn2)) * var_mu_ave) + (assign33730_e48931 * var_mu_ave_dn2)) * var_mu_ave) + (assign33730_e48933 * var_mu_ave_dn2)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((var_weff_nf * var_c_fox_dn6) * var_vgvt) + (assign33730_e48880 * var_vgvt_dn6)) * var_mu) + (assign33730_e48882 * var_mu_dn6)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * var_alpha_dn6) + (6.0 * var_t0__blk1112_dn6)) * var_mud_hoso) + (assign33730_e48893 * var_mud_hoso_dn6)) * var_mud_hoso) + (assign33730_e48895 * var_mud_hoso_dn6)) + ((((((4.0 * var_alpha_dn6) + (3.0 * var_t0__blk1112_dn6)) * var_mud_hoso) + (assign33730_e48906 * var_mud_hoso_dn6)) * var_mu) + (assign33730_e48908 * var_mu_dn6))) + ((((((3.0 * var_alpha_dn6) + var_t0__blk1112_dn6) * var_mu) + (assign33730_e48918 * var_mu_dn6)) * var_mu) + (assign33730_e48920 * var_mu_dn6))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * var_lch_dn6) * assign33730_e48930) + (assign33730_e48927 * var_alpha_dn6)) * var_mu_ave) + (assign33730_e48931 * var_mu_ave_dn6)) * var_mu_ave) + (assign33730_e48933 * var_mu_ave_dn6)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((var_weff_nf * var_c_fox_dn7) * var_vgvt) + (assign33730_e48880 * var_vgvt_dn7)) * var_mu) + (assign33730_e48882 * var_mu_dn7)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * var_alpha_dn7) + (6.0 * var_t0__blk1112_dn7)) * var_mud_hoso) + (assign33730_e48893 * var_mud_hoso_dn7)) * var_mud_hoso) + (assign33730_e48895 * var_mud_hoso_dn7)) + ((((((4.0 * var_alpha_dn7) + (3.0 * var_t0__blk1112_dn7)) * var_mud_hoso) + (assign33730_e48906 * var_mud_hoso_dn7)) * var_mu) + (assign33730_e48908 * var_mu_dn7))) + ((((((3.0 * var_alpha_dn7) + var_t0__blk1112_dn7) * var_mu) + (assign33730_e48918 * var_mu_dn7)) * var_mu) + (assign33730_e48920 * var_mu_dn7))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * var_lch_dn7) * assign33730_e48930) + (assign33730_e48927 * var_alpha_dn7)) * var_mu_ave) + (assign33730_e48931 * var_mu_ave_dn7)) * var_mu_ave) + (assign33730_e48933 * var_mu_ave_dn7)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((var_weff_nf * var_c_fox_dn10) * var_vgvt) + (assign33730_e48880 * var_vgvt_dn10)) * var_mu) + (assign33730_e48882 * var_mu_dn10)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * var_alpha_dn10) + (6.0 * var_t0__blk1112_dn10)) * var_mud_hoso) + (assign33730_e48893 * var_mud_hoso_dn10)) * var_mud_hoso) + (assign33730_e48895 * var_mud_hoso_dn10)) + ((((((4.0 * var_alpha_dn10) + (3.0 * var_t0__blk1112_dn10)) * var_mud_hoso) + (assign33730_e48906 * var_mud_hoso_dn10)) * var_mu) + (assign33730_e48908 * var_mu_dn10))) + ((((((3.0 * var_alpha_dn10) + var_t0__blk1112_dn10) * var_mu) + (assign33730_e48918 * var_mu_dn10)) * var_mu) + (assign33730_e48920 * var_mu_dn10))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * var_lch_dn10) * assign33730_e48930) + (assign33730_e48927 * var_alpha_dn10)) * var_mu_ave) + (assign33730_e48931 * var_mu_ave_dn10)) * var_mu_ave) + (assign33730_e48933 * var_mu_ave_dn10)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((var_weff_nf * var_c_fox_dn11) * var_vgvt) + (assign33730_e48880 * var_vgvt_dn11)) * var_mu) + (assign33730_e48882 * var_mu_dn11)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * var_alpha_dn11) + (6.0 * var_t0__blk1112_dn11)) * var_mud_hoso) + (assign33730_e48893 * var_mud_hoso_dn11)) * var_mud_hoso) + (assign33730_e48895 * var_mud_hoso_dn11)) + ((((((4.0 * var_alpha_dn11) + (3.0 * var_t0__blk1112_dn11)) * var_mud_hoso) + (assign33730_e48906 * var_mud_hoso_dn11)) * var_mu) + (assign33730_e48908 * var_mu_dn11))) + ((((((3.0 * var_alpha_dn11) + var_t0__blk1112_dn11) * var_mu) + (assign33730_e48918 * var_mu_dn11)) * var_mu) + (assign33730_e48920 * var_mu_dn11))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * var_lch_dn11) * assign33730_e48930) + (assign33730_e48927 * var_alpha_dn11)) * var_mu_ave) + (assign33730_e48931 * var_mu_ave_dn11)) * var_mu_ave) + (assign33730_e48933 * var_mu_ave_dn11)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((var_weff_nf * var_c_fox_dn12) * var_vgvt) + (assign33730_e48880 * var_vgvt_dn12)) * var_mu) + (assign33730_e48882 * var_mu_dn12)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * var_alpha_dn12) + (6.0 * var_t0__blk1112_dn12)) * var_mud_hoso) + (assign33730_e48893 * var_mud_hoso_dn12)) * var_mud_hoso) + (assign33730_e48895 * var_mud_hoso_dn12)) + ((((((4.0 * var_alpha_dn12) + (3.0 * var_t0__blk1112_dn12)) * var_mud_hoso) + (assign33730_e48906 * var_mud_hoso_dn12)) * var_mu) + (assign33730_e48908 * var_mu_dn12))) + ((((((3.0 * var_alpha_dn12) + var_t0__blk1112_dn12) * var_mu) + (assign33730_e48918 * var_mu_dn12)) * var_mu) + (assign33730_e48920 * var_mu_dn12))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * var_lch_dn12) * assign33730_e48930) + (assign33730_e48927 * var_alpha_dn12)) * var_mu_ave) + (assign33730_e48931 * var_mu_ave_dn12)) * var_mu_ave) + (assign33730_e48933 * var_mu_ave_dn12)))) / (assign33730_e48935 * assign33730_e48935)), ((((((((((var_weff_nf * var_c_fox_dn17) * var_vgvt) + (assign33730_e48880 * var_vgvt_dn17)) * var_mu) + (assign33730_e48882 * var_mu_dn17)) * assign33730_e48923) + (assign33730_e48884 * ((((((((3.0 * var_alpha_dn17) + (6.0 * var_t0__blk1112_dn17)) * var_mud_hoso) + (assign33730_e48893 * var_mud_hoso_dn17)) * var_mud_hoso) + (assign33730_e48895 * var_mud_hoso_dn17)) + ((((((4.0 * var_alpha_dn17) + (3.0 * var_t0__blk1112_dn17)) * var_mud_hoso) + (assign33730_e48906 * var_mud_hoso_dn17)) * var_mu) + (assign33730_e48908 * var_mu_dn17))) + ((((((3.0 * var_alpha_dn17) + var_t0__blk1112_dn17) * var_mu) + (assign33730_e48918 * var_mu_dn17)) * var_mu) + (assign33730_e48920 * var_mu_dn17))))) * assign33730_e48935) - (assign33730_e48924 * (((((((15.0 * var_lch_dn17) * assign33730_e48930) + (assign33730_e48927 * var_alpha_dn17)) * var_mu_ave) + (assign33730_e48931 * var_mu_ave_dn17)) * var_mu_ave) + (assign33730_e48933 * var_mu_ave_dn17)))) / (assign33730_e48935 * assign33730_e48935)),)
    } else {
        (var_nthrml, var_nthrml_dn0, var_nthrml_dn2, var_nthrml_dn6, var_nthrml_dn7, var_nthrml_dn10, var_nthrml_dn11, var_nthrml_dn12, var_nthrml_dn17,)
    }
};
        var_nthrml = assign33730_e48938;
        var_nthrml_dn0 = assign33730_e48938_d_n0;
        var_nthrml_dn2 = assign33730_e48938_d_n2;
        var_nthrml_dn6 = assign33730_e48938_d_n6;
        var_nthrml_dn7 = assign33730_e48938_d_n7;
        var_nthrml_dn10 = assign33730_e48938_d_n10;
        var_nthrml_dn11 = assign33730_e48938_d_n11;
        var_nthrml_dn12 = assign33730_e48938_d_n12;
        var_nthrml_dn17 = assign33730_e48938_d_n17;

        let (assign33740_e48943, assign33740_e48943_d_n0, assign33740_e48943_d_n2, assign33740_e48943_d_n6, assign33740_e48943_d_n7, assign33740_e48943_d_n10, assign33740_e48943_d_n11, assign33740_e48943_d_n12, assign33740_e48943_d_n17,) = {
    if (var_guard1122 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nthrml, var_nthrml_dn0, var_nthrml_dn2, var_nthrml_dn6, var_nthrml_dn7, var_nthrml_dn10, var_nthrml_dn11, var_nthrml_dn12, var_nthrml_dn17,)
    }
};
        var_nthrml = assign33740_e48943;
        var_nthrml_dn0 = assign33740_e48943_d_n0;
        var_nthrml_dn2 = assign33740_e48943_d_n2;
        var_nthrml_dn6 = assign33740_e48943_d_n6;
        var_nthrml_dn7 = assign33740_e48943_d_n7;
        var_nthrml_dn10 = assign33740_e48943_d_n10;
        var_nthrml_dn11 = assign33740_e48943_d_n11;
        var_nthrml_dn12 = assign33740_e48943_d_n12;
        var_nthrml_dn17 = assign33740_e48943_d_n17;

        let assign33750_e48957: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (var_flg_ign == 1.0)) && (var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        var_guard1125 = assign33750_e48957;

        let (assign33760_e48962, assign33760_e48962_d_n0, assign33760_e48962_d_n2, assign33760_e48962_d_n6, assign33760_e48962_d_n7, assign33760_e48962_d_n10, assign33760_e48962_d_n11, assign33760_e48962_d_n12, assign33760_e48962_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33760_e48960: f64 = (var_kusail).sqrt();
        (assign33760_e48960, (var_kusail_dn0 / (2.0 * assign33760_e48960)), (var_kusail_dn2 / (2.0 * assign33760_e48960)), (var_kusail_dn6 / (2.0 * assign33760_e48960)), (var_kusail_dn7 / (2.0 * assign33760_e48960)), (var_kusail_dn10 / (2.0 * assign33760_e48960)), (var_kusail_dn11 / (2.0 * assign33760_e48960)), (var_kusail_dn12 / (2.0 * assign33760_e48960)), (var_kusail_dn17 / (2.0 * assign33760_e48960)),)
    } else {
        (var_sqrtkusail, var_sqrtkusail_dn0, var_sqrtkusail_dn2, var_sqrtkusail_dn6, var_sqrtkusail_dn7, var_sqrtkusail_dn10, var_sqrtkusail_dn11, var_sqrtkusail_dn12, var_sqrtkusail_dn17,)
    }
};
        var_sqrtkusail = assign33760_e48962;
        var_sqrtkusail_dn0 = assign33760_e48962_d_n0;
        var_sqrtkusail_dn2 = assign33760_e48962_d_n2;
        var_sqrtkusail_dn6 = assign33760_e48962_d_n6;
        var_sqrtkusail_dn7 = assign33760_e48962_d_n7;
        var_sqrtkusail_dn10 = assign33760_e48962_d_n10;
        var_sqrtkusail_dn11 = assign33760_e48962_d_n11;
        var_sqrtkusail_dn12 = assign33760_e48962_d_n12;
        var_sqrtkusail_dn17 = assign33760_e48962_d_n17;

        let (assign33770_e48968, assign33770_e48968_d_n0, assign33770_e48968_d_n2, assign33770_e48968_d_n6, assign33770_e48968_d_n7, assign33770_e48968_d_n10, assign33770_e48968_d_n11, assign33770_e48968_d_n12, assign33770_e48968_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33770_e48966: f64 = (var_vgvt + var_sqrtkusail);
        (assign33770_e48966, (var_vgvt_dn0 + var_sqrtkusail_dn0), (var_vgvt_dn2 + var_sqrtkusail_dn2), (var_vgvt_dn6 + var_sqrtkusail_dn6), (var_vgvt_dn7 + var_sqrtkusail_dn7), (var_vgvt_dn10 + var_sqrtkusail_dn10), (var_vgvt_dn11 + var_sqrtkusail_dn11), (var_vgvt_dn12 + var_sqrtkusail_dn12), (var_vgvt_dn17 + var_sqrtkusail_dn17),)
    } else {
        (var_t2__blk1114, var_t2__blk1114_dn0, var_t2__blk1114_dn2, var_t2__blk1114_dn6, var_t2__blk1114_dn7, var_t2__blk1114_dn10, var_t2__blk1114_dn11, var_t2__blk1114_dn12, var_t2__blk1114_dn17,)
    }
};
        var_t2__blk1114 = assign33770_e48968;
        var_t2__blk1114_dn0 = assign33770_e48968_d_n0;
        var_t2__blk1114_dn2 = assign33770_e48968_d_n2;
        var_t2__blk1114_dn6 = assign33770_e48968_d_n6;
        var_t2__blk1114_dn7 = assign33770_e48968_d_n7;
        var_t2__blk1114_dn10 = assign33770_e48968_d_n10;
        var_t2__blk1114_dn11 = assign33770_e48968_d_n11;
        var_t2__blk1114_dn12 = assign33770_e48968_d_n12;
        var_t2__blk1114_dn17 = assign33770_e48968_d_n17;

        let (assign33780_e48974, assign33780_e48974_d_n0, assign33780_e48974_d_n2, assign33780_e48974_d_n6, assign33780_e48974_d_n7, assign33780_e48974_d_n10, assign33780_e48974_d_n11, assign33780_e48974_d_n12, assign33780_e48974_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33780_e48972: f64 = (var_kusai00 * var_kusai00);
        (assign33780_e48972, ((var_kusai00_dn0 * var_kusai00) + (var_kusai00 * var_kusai00_dn0)), ((var_kusai00_dn2 * var_kusai00) + (var_kusai00 * var_kusai00_dn2)), ((var_kusai00_dn6 * var_kusai00) + (var_kusai00 * var_kusai00_dn6)), ((var_kusai00_dn7 * var_kusai00) + (var_kusai00 * var_kusai00_dn7)), ((var_kusai00_dn10 * var_kusai00) + (var_kusai00 * var_kusai00_dn10)), ((var_kusai00_dn11 * var_kusai00) + (var_kusai00 * var_kusai00_dn11)), ((var_kusai00_dn12 * var_kusai00) + (var_kusai00 * var_kusai00_dn12)), ((var_kusai00_dn17 * var_kusai00) + (var_kusai00 * var_kusai00_dn17)),)
    } else {
        (var_t3__blk1115, var_t3__blk1115_dn0, var_t3__blk1115_dn2, var_t3__blk1115_dn6, var_t3__blk1115_dn7, var_t3__blk1115_dn10, var_t3__blk1115_dn11, var_t3__blk1115_dn12, var_t3__blk1115_dn17,)
    }
};
        var_t3__blk1115 = assign33780_e48974;
        var_t3__blk1115_dn0 = assign33780_e48974_d_n0;
        var_t3__blk1115_dn2 = assign33780_e48974_d_n2;
        var_t3__blk1115_dn6 = assign33780_e48974_d_n6;
        var_t3__blk1115_dn7 = assign33780_e48974_d_n7;
        var_t3__blk1115_dn10 = assign33780_e48974_d_n10;
        var_t3__blk1115_dn11 = assign33780_e48974_d_n11;
        var_t3__blk1115_dn12 = assign33780_e48974_d_n12;
        var_t3__blk1115_dn17 = assign33780_e48974_d_n17;

        let (assign33790_e48980, assign33790_e48980_d_n0, assign33790_e48980_d_n2, assign33790_e48980_d_n6, assign33790_e48980_d_n7, assign33790_e48980_d_n10, assign33790_e48980_d_n11, assign33790_e48980_d_n12, assign33790_e48980_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33790_e48978: f64 = (var_kusail * var_kusail);
        (assign33790_e48978, ((var_kusail_dn0 * var_kusail) + (var_kusail * var_kusail_dn0)), ((var_kusail_dn2 * var_kusail) + (var_kusail * var_kusail_dn2)), ((var_kusail_dn6 * var_kusail) + (var_kusail * var_kusail_dn6)), ((var_kusail_dn7 * var_kusail) + (var_kusail * var_kusail_dn7)), ((var_kusail_dn10 * var_kusail) + (var_kusail * var_kusail_dn10)), ((var_kusail_dn11 * var_kusail) + (var_kusail * var_kusail_dn11)), ((var_kusail_dn12 * var_kusail) + (var_kusail * var_kusail_dn12)), ((var_kusail_dn17 * var_kusail) + (var_kusail * var_kusail_dn17)),)
    } else {
        (var_t4__blk1116, var_t4__blk1116_dn0, var_t4__blk1116_dn2, var_t4__blk1116_dn6, var_t4__blk1116_dn7, var_t4__blk1116_dn10, var_t4__blk1116_dn11, var_t4__blk1116_dn12, var_t4__blk1116_dn17,)
    }
};
        var_t4__blk1116 = assign33790_e48980;
        var_t4__blk1116_dn0 = assign33790_e48980_d_n0;
        var_t4__blk1116_dn2 = assign33790_e48980_d_n2;
        var_t4__blk1116_dn6 = assign33790_e48980_d_n6;
        var_t4__blk1116_dn7 = assign33790_e48980_d_n7;
        var_t4__blk1116_dn10 = assign33790_e48980_d_n10;
        var_t4__blk1116_dn11 = assign33790_e48980_d_n11;
        var_t4__blk1116_dn12 = assign33790_e48980_d_n12;
        var_t4__blk1116_dn17 = assign33790_e48980_d_n17;

        let (assign33800_e48988, assign33800_e48988_d_n0, assign33800_e48988_d_n2, assign33800_e48988_d_n6, assign33800_e48988_d_n7, assign33800_e48988_d_n10, assign33800_e48988_d_n11, assign33800_e48988_d_n12, assign33800_e48988_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33800_e48984: f64 = (42.0 * var_kusai00);
        let assign33800_e48986: f64 = (assign33800_e48984 * var_kusail);
        (assign33800_e48986, (((42.0 * var_kusai00_dn0) * var_kusail) + (assign33800_e48984 * var_kusail_dn0)), (((42.0 * var_kusai00_dn2) * var_kusail) + (assign33800_e48984 * var_kusail_dn2)), (((42.0 * var_kusai00_dn6) * var_kusail) + (assign33800_e48984 * var_kusail_dn6)), (((42.0 * var_kusai00_dn7) * var_kusail) + (assign33800_e48984 * var_kusail_dn7)), (((42.0 * var_kusai00_dn10) * var_kusail) + (assign33800_e48984 * var_kusail_dn10)), (((42.0 * var_kusai00_dn11) * var_kusail) + (assign33800_e48984 * var_kusail_dn11)), (((42.0 * var_kusai00_dn12) * var_kusail) + (assign33800_e48984 * var_kusail_dn12)), (((42.0 * var_kusai00_dn17) * var_kusail) + (assign33800_e48984 * var_kusail_dn17)),)
    } else {
        (var_t5__blk1117, var_t5__blk1117_dn0, var_t5__blk1117_dn2, var_t5__blk1117_dn6, var_t5__blk1117_dn7, var_t5__blk1117_dn10, var_t5__blk1117_dn11, var_t5__blk1117_dn12, var_t5__blk1117_dn17,)
    }
};
        var_t5__blk1117 = assign33800_e48988;
        var_t5__blk1117_dn0 = assign33800_e48988_d_n0;
        var_t5__blk1117_dn2 = assign33800_e48988_d_n2;
        var_t5__blk1117_dn6 = assign33800_e48988_d_n6;
        var_t5__blk1117_dn7 = assign33800_e48988_d_n7;
        var_t5__blk1117_dn10 = assign33800_e48988_d_n10;
        var_t5__blk1117_dn11 = assign33800_e48988_d_n11;
        var_t5__blk1117_dn12 = assign33800_e48988_d_n12;
        var_t5__blk1117_dn17 = assign33800_e48988_d_n17;

        let (assign33810_e48998, assign33810_e48998_d_n0, assign33810_e48998_d_n2, assign33810_e48998_d_n6, assign33810_e48998_d_n7, assign33810_e48998_d_n10, assign33810_e48998_d_n11, assign33810_e48998_d_n12, assign33810_e48998_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33810_e48994: f64 = (var_t3__blk1115 + var_t4__blk1116);
        let assign33810_e48995: f64 = (4.0 * assign33810_e48994);
        let assign33810_e48996: f64 = (var_t5__blk1117 + assign33810_e48995);
        (assign33810_e48996, (var_t5__blk1117_dn0 + (4.0 * (var_t3__blk1115_dn0 + var_t4__blk1116_dn0))), (var_t5__blk1117_dn2 + (4.0 * (var_t3__blk1115_dn2 + var_t4__blk1116_dn2))), (var_t5__blk1117_dn6 + (4.0 * (var_t3__blk1115_dn6 + var_t4__blk1116_dn6))), (var_t5__blk1117_dn7 + (4.0 * (var_t3__blk1115_dn7 + var_t4__blk1116_dn7))), (var_t5__blk1117_dn10 + (4.0 * (var_t3__blk1115_dn10 + var_t4__blk1116_dn10))), (var_t5__blk1117_dn11 + (4.0 * (var_t3__blk1115_dn11 + var_t4__blk1116_dn11))), (var_t5__blk1117_dn12 + (4.0 * (var_t3__blk1115_dn12 + var_t4__blk1116_dn12))), (var_t5__blk1117_dn17 + (4.0 * (var_t3__blk1115_dn17 + var_t4__blk1116_dn17))),)
    } else {
        (var_t5__blk1117, var_t5__blk1117_dn0, var_t5__blk1117_dn2, var_t5__blk1117_dn6, var_t5__blk1117_dn7, var_t5__blk1117_dn10, var_t5__blk1117_dn11, var_t5__blk1117_dn12, var_t5__blk1117_dn17,)
    }
};
        var_t5__blk1117 = assign33810_e48998;
        var_t5__blk1117_dn0 = assign33810_e48998_d_n0;
        var_t5__blk1117_dn2 = assign33810_e48998_d_n2;
        var_t5__blk1117_dn6 = assign33810_e48998_d_n6;
        var_t5__blk1117_dn7 = assign33810_e48998_d_n7;
        var_t5__blk1117_dn10 = assign33810_e48998_d_n10;
        var_t5__blk1117_dn11 = assign33810_e48998_d_n11;
        var_t5__blk1117_dn12 = assign33810_e48998_d_n12;
        var_t5__blk1117_dn17 = assign33810_e48998_d_n17;

        let (assign33820_e49012, assign33820_e49012_d_n0, assign33820_e49012_d_n2, assign33820_e49012_d_n6, assign33820_e49012_d_n7, assign33820_e49012_d_n10, assign33820_e49012_d_n11, assign33820_e49012_d_n12, assign33820_e49012_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33820_e49003: f64 = (20.0 * var_sqrtkusail);
        let assign33820_e49005: f64 = (assign33820_e49003 * var_vgvt);
        let assign33820_e49008: f64 = (var_kusai00 + var_kusail);
        let assign33820_e49009: f64 = (assign33820_e49005 * assign33820_e49008);
        let assign33820_e49010: f64 = (var_t5__blk1117 + assign33820_e49009);
        (assign33820_e49010, (var_t5__blk1117_dn0 + (((((20.0 * var_sqrtkusail_dn0) * var_vgvt) + (assign33820_e49003 * var_vgvt_dn0)) * assign33820_e49008) + (assign33820_e49005 * (var_kusai00_dn0 + var_kusail_dn0)))), (var_t5__blk1117_dn2 + (((((20.0 * var_sqrtkusail_dn2) * var_vgvt) + (assign33820_e49003 * var_vgvt_dn2)) * assign33820_e49008) + (assign33820_e49005 * (var_kusai00_dn2 + var_kusail_dn2)))), (var_t5__blk1117_dn6 + (((((20.0 * var_sqrtkusail_dn6) * var_vgvt) + (assign33820_e49003 * var_vgvt_dn6)) * assign33820_e49008) + (assign33820_e49005 * (var_kusai00_dn6 + var_kusail_dn6)))), (var_t5__blk1117_dn7 + (((((20.0 * var_sqrtkusail_dn7) * var_vgvt) + (assign33820_e49003 * var_vgvt_dn7)) * assign33820_e49008) + (assign33820_e49005 * (var_kusai00_dn7 + var_kusail_dn7)))), (var_t5__blk1117_dn10 + (((((20.0 * var_sqrtkusail_dn10) * var_vgvt) + (assign33820_e49003 * var_vgvt_dn10)) * assign33820_e49008) + (assign33820_e49005 * (var_kusai00_dn10 + var_kusail_dn10)))), (var_t5__blk1117_dn11 + (((((20.0 * var_sqrtkusail_dn11) * var_vgvt) + (assign33820_e49003 * var_vgvt_dn11)) * assign33820_e49008) + (assign33820_e49005 * (var_kusai00_dn11 + var_kusail_dn11)))), (var_t5__blk1117_dn12 + (((((20.0 * var_sqrtkusail_dn12) * var_vgvt) + (assign33820_e49003 * var_vgvt_dn12)) * assign33820_e49008) + (assign33820_e49005 * (var_kusai00_dn12 + var_kusail_dn12)))), (var_t5__blk1117_dn17 + (((((20.0 * var_sqrtkusail_dn17) * var_vgvt) + (assign33820_e49003 * var_vgvt_dn17)) * assign33820_e49008) + (assign33820_e49005 * (var_kusai00_dn17 + var_kusail_dn17)))),)
    } else {
        (var_t5__blk1117, var_t5__blk1117_dn0, var_t5__blk1117_dn2, var_t5__blk1117_dn6, var_t5__blk1117_dn7, var_t5__blk1117_dn10, var_t5__blk1117_dn11, var_t5__blk1117_dn12, var_t5__blk1117_dn17,)
    }
};
        var_t5__blk1117 = assign33820_e49012;
        var_t5__blk1117_dn0 = assign33820_e49012_d_n0;
        var_t5__blk1117_dn2 = assign33820_e49012_d_n2;
        var_t5__blk1117_dn6 = assign33820_e49012_d_n6;
        var_t5__blk1117_dn7 = assign33820_e49012_d_n7;
        var_t5__blk1117_dn10 = assign33820_e49012_d_n10;
        var_t5__blk1117_dn11 = assign33820_e49012_d_n11;
        var_t5__blk1117_dn12 = assign33820_e49012_d_n12;
        var_t5__blk1117_dn17 = assign33820_e49012_d_n17;

        let (assign33830_e49018, assign33830_e49018_d_n0, assign33830_e49018_d_n2, assign33830_e49018_d_n6, assign33830_e49018_d_n7, assign33830_e49018_d_n10, assign33830_e49018_d_n11, assign33830_e49018_d_n12, assign33830_e49018_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33830_e49016: f64 = (var_t2__blk1114 * var_t2__blk1114);
        (assign33830_e49016, ((var_t2__blk1114_dn0 * var_t2__blk1114) + (var_t2__blk1114 * var_t2__blk1114_dn0)), ((var_t2__blk1114_dn2 * var_t2__blk1114) + (var_t2__blk1114 * var_t2__blk1114_dn2)), ((var_t2__blk1114_dn6 * var_t2__blk1114) + (var_t2__blk1114 * var_t2__blk1114_dn6)), ((var_t2__blk1114_dn7 * var_t2__blk1114) + (var_t2__blk1114 * var_t2__blk1114_dn7)), ((var_t2__blk1114_dn10 * var_t2__blk1114) + (var_t2__blk1114 * var_t2__blk1114_dn10)), ((var_t2__blk1114_dn11 * var_t2__blk1114) + (var_t2__blk1114 * var_t2__blk1114_dn11)), ((var_t2__blk1114_dn12 * var_t2__blk1114) + (var_t2__blk1114 * var_t2__blk1114_dn12)), ((var_t2__blk1114_dn17 * var_t2__blk1114) + (var_t2__blk1114 * var_t2__blk1114_dn17)),)
    } else {
        (var_t10w, var_t10w_dn0, var_t10w_dn2, var_t10w_dn6, var_t10w_dn7, var_t10w_dn10, var_t10w_dn11, var_t10w_dn12, var_t10w_dn17,)
    }
};
        var_t10w = assign33830_e49018;
        var_t10w_dn0 = assign33830_e49018_d_n0;
        var_t10w_dn2 = assign33830_e49018_d_n2;
        var_t10w_dn6 = assign33830_e49018_d_n6;
        var_t10w_dn7 = assign33830_e49018_d_n7;
        var_t10w_dn10 = assign33830_e49018_d_n10;
        var_t10w_dn11 = assign33830_e49018_d_n11;
        var_t10w_dn12 = assign33830_e49018_d_n12;
        var_t10w_dn17 = assign33830_e49018_d_n17;

        let (assign33840_e49024, assign33840_e49024_d_n0, assign33840_e49024_d_n2, assign33840_e49024_d_n6, assign33840_e49024_d_n7, assign33840_e49024_d_n10, assign33840_e49024_d_n11, assign33840_e49024_d_n12, assign33840_e49024_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33840_e49022: f64 = (var_t10w * var_t10w);
        (assign33840_e49022, ((var_t10w_dn0 * var_t10w) + (var_t10w * var_t10w_dn0)), ((var_t10w_dn2 * var_t10w) + (var_t10w * var_t10w_dn2)), ((var_t10w_dn6 * var_t10w) + (var_t10w * var_t10w_dn6)), ((var_t10w_dn7 * var_t10w) + (var_t10w * var_t10w_dn7)), ((var_t10w_dn10 * var_t10w) + (var_t10w * var_t10w_dn10)), ((var_t10w_dn11 * var_t10w) + (var_t10w * var_t10w_dn11)), ((var_t10w_dn12 * var_t10w) + (var_t10w * var_t10w_dn12)), ((var_t10w_dn17 * var_t10w) + (var_t10w * var_t10w_dn17)),)
    } else {
        (var_t10__blk1110, var_t10__blk1110_dn0, var_t10__blk1110_dn2, var_t10__blk1110_dn6, var_t10__blk1110_dn7, var_t10__blk1110_dn10, var_t10__blk1110_dn11, var_t10__blk1110_dn12, var_t10__blk1110_dn17,)
    }
};
        var_t10__blk1110 = assign33840_e49024;
        var_t10__blk1110_dn0 = assign33840_e49024_d_n0;
        var_t10__blk1110_dn2 = assign33840_e49024_d_n2;
        var_t10__blk1110_dn6 = assign33840_e49024_d_n6;
        var_t10__blk1110_dn7 = assign33840_e49024_d_n7;
        var_t10__blk1110_dn10 = assign33840_e49024_d_n10;
        var_t10__blk1110_dn11 = assign33840_e49024_d_n11;
        var_t10__blk1110_dn12 = assign33840_e49024_d_n12;
        var_t10__blk1110_dn17 = assign33840_e49024_d_n17;

        let (assign33850_e49032, assign33850_e49032_d_n0, assign33850_e49032_d_n2, assign33850_e49032_d_n6, assign33850_e49032_d_n7, assign33850_e49032_d_n10, assign33850_e49032_d_n11, assign33850_e49032_d_n12, assign33850_e49032_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33850_e49029: f64 = (var_t10__blk1110 * var_t2__blk1114);
        let assign33850_e49030: f64 = (var_t5__blk1117 / assign33850_e49029);
        (assign33850_e49030, (((var_t5__blk1117_dn0 * assign33850_e49029) - (var_t5__blk1117 * ((var_t10__blk1110_dn0 * var_t2__blk1114) + (var_t10__blk1110 * var_t2__blk1114_dn0)))) / (assign33850_e49029 * assign33850_e49029)), (((var_t5__blk1117_dn2 * assign33850_e49029) - (var_t5__blk1117 * ((var_t10__blk1110_dn2 * var_t2__blk1114) + (var_t10__blk1110 * var_t2__blk1114_dn2)))) / (assign33850_e49029 * assign33850_e49029)), (((var_t5__blk1117_dn6 * assign33850_e49029) - (var_t5__blk1117 * ((var_t10__blk1110_dn6 * var_t2__blk1114) + (var_t10__blk1110 * var_t2__blk1114_dn6)))) / (assign33850_e49029 * assign33850_e49029)), (((var_t5__blk1117_dn7 * assign33850_e49029) - (var_t5__blk1117 * ((var_t10__blk1110_dn7 * var_t2__blk1114) + (var_t10__blk1110 * var_t2__blk1114_dn7)))) / (assign33850_e49029 * assign33850_e49029)), (((var_t5__blk1117_dn10 * assign33850_e49029) - (var_t5__blk1117 * ((var_t10__blk1110_dn10 * var_t2__blk1114) + (var_t10__blk1110 * var_t2__blk1114_dn10)))) / (assign33850_e49029 * assign33850_e49029)), (((var_t5__blk1117_dn11 * assign33850_e49029) - (var_t5__blk1117 * ((var_t10__blk1110_dn11 * var_t2__blk1114) + (var_t10__blk1110 * var_t2__blk1114_dn11)))) / (assign33850_e49029 * assign33850_e49029)), (((var_t5__blk1117_dn12 * assign33850_e49029) - (var_t5__blk1117 * ((var_t10__blk1110_dn12 * var_t2__blk1114) + (var_t10__blk1110 * var_t2__blk1114_dn12)))) / (assign33850_e49029 * assign33850_e49029)), (((var_t5__blk1117_dn17 * assign33850_e49029) - (var_t5__blk1117 * ((var_t10__blk1110_dn17 * var_t2__blk1114) + (var_t10__blk1110 * var_t2__blk1114_dn17)))) / (assign33850_e49029 * assign33850_e49029)),)
    } else {
        (var_kusai_ig, var_kusai_ig_dn0, var_kusai_ig_dn2, var_kusai_ig_dn6, var_kusai_ig_dn7, var_kusai_ig_dn10, var_kusai_ig_dn11, var_kusai_ig_dn12, var_kusai_ig_dn17,)
    }
};
        var_kusai_ig = assign33850_e49032;
        var_kusai_ig_dn0 = assign33850_e49032_d_n0;
        var_kusai_ig_dn2 = assign33850_e49032_d_n2;
        var_kusai_ig_dn6 = assign33850_e49032_d_n6;
        var_kusai_ig_dn7 = assign33850_e49032_d_n7;
        var_kusai_ig_dn10 = assign33850_e49032_d_n10;
        var_kusai_ig_dn11 = assign33850_e49032_d_n11;
        var_kusai_ig_dn12 = assign33850_e49032_d_n12;
        var_kusai_ig_dn17 = assign33850_e49032_d_n17;

        let (assign33860_e49042, assign33860_e49042_d_n0, assign33860_e49042_d_n2, assign33860_e49042_d_n6, assign33860_e49042_d_n7, assign33860_e49042_d_n10, assign33860_e49042_d_n11, assign33860_e49042_d_n12, assign33860_e49042_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33860_e49036: f64 = (var_weff_nf / var_lch);
        let assign33860_e49038: f64 = (assign33860_e49036 * var_mu);
        let assign33860_e49040: f64 = (assign33860_e49038 * var_c_fox);
        (assign33860_e49040, (((((-((var_weff_nf * var_lch_dn0) / (var_lch * var_lch))) * var_mu) + (assign33860_e49036 * var_mu_dn0)) * var_c_fox) + (assign33860_e49038 * var_c_fox_dn0)), (((((-((var_weff_nf * var_lch_dn2) / (var_lch * var_lch))) * var_mu) + (assign33860_e49036 * var_mu_dn2)) * var_c_fox) + (assign33860_e49038 * var_c_fox_dn2)), (((((-((var_weff_nf * var_lch_dn6) / (var_lch * var_lch))) * var_mu) + (assign33860_e49036 * var_mu_dn6)) * var_c_fox) + (assign33860_e49038 * var_c_fox_dn6)), (((((-((var_weff_nf * var_lch_dn7) / (var_lch * var_lch))) * var_mu) + (assign33860_e49036 * var_mu_dn7)) * var_c_fox) + (assign33860_e49038 * var_c_fox_dn7)), (((((-((var_weff_nf * var_lch_dn10) / (var_lch * var_lch))) * var_mu) + (assign33860_e49036 * var_mu_dn10)) * var_c_fox) + (assign33860_e49038 * var_c_fox_dn10)), (((((-((var_weff_nf * var_lch_dn11) / (var_lch * var_lch))) * var_mu) + (assign33860_e49036 * var_mu_dn11)) * var_c_fox) + (assign33860_e49038 * var_c_fox_dn11)), (((((-((var_weff_nf * var_lch_dn12) / (var_lch * var_lch))) * var_mu) + (assign33860_e49036 * var_mu_dn12)) * var_c_fox) + (assign33860_e49038 * var_c_fox_dn12)), (((((-((var_weff_nf * var_lch_dn17) / (var_lch * var_lch))) * var_mu) + (assign33860_e49036 * var_mu_dn17)) * var_c_fox) + (assign33860_e49038 * var_c_fox_dn17)),)
    } else {
        (var_gds0_ign, var_gds0_ign_dn0, var_gds0_ign_dn2, var_gds0_ign_dn6, var_gds0_ign_dn7, var_gds0_ign_dn10, var_gds0_ign_dn11, var_gds0_ign_dn12, var_gds0_ign_dn17,)
    }
};
        var_gds0_ign = assign33860_e49042;
        var_gds0_ign_dn0 = assign33860_e49042_d_n0;
        var_gds0_ign_dn2 = assign33860_e49042_d_n2;
        var_gds0_ign_dn6 = assign33860_e49042_d_n6;
        var_gds0_ign_dn7 = assign33860_e49042_d_n7;
        var_gds0_ign_dn10 = assign33860_e49042_d_n10;
        var_gds0_ign_dn11 = assign33860_e49042_d_n11;
        var_gds0_ign_dn12 = assign33860_e49042_d_n12;
        var_gds0_ign_dn17 = assign33860_e49042_d_n17;

        let (assign33870_e49048, assign33870_e49048_d_n0, assign33870_e49048_d_n2, assign33870_e49048_d_n6, assign33870_e49048_d_n7, assign33870_e49048_d_n10, assign33870_e49048_d_n11, assign33870_e49048_d_n12, assign33870_e49048_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33870_e49046: f64 = (var_gds0_ign * var_vgvt);
        (assign33870_e49046, ((var_gds0_ign_dn0 * var_vgvt) + (var_gds0_ign * var_vgvt_dn0)), ((var_gds0_ign_dn2 * var_vgvt) + (var_gds0_ign * var_vgvt_dn2)), ((var_gds0_ign_dn6 * var_vgvt) + (var_gds0_ign * var_vgvt_dn6)), ((var_gds0_ign_dn7 * var_vgvt) + (var_gds0_ign * var_vgvt_dn7)), ((var_gds0_ign_dn10 * var_vgvt) + (var_gds0_ign * var_vgvt_dn10)), ((var_gds0_ign_dn11 * var_vgvt) + (var_gds0_ign * var_vgvt_dn11)), ((var_gds0_ign_dn12 * var_vgvt) + (var_gds0_ign * var_vgvt_dn12)), ((var_gds0_ign_dn17 * var_vgvt) + (var_gds0_ign * var_vgvt_dn17)),)
    } else {
        (var_gds0_h2, var_gds0_h2_dn0, var_gds0_h2_dn2, var_gds0_h2_dn6, var_gds0_h2_dn7, var_gds0_h2_dn10, var_gds0_h2_dn11, var_gds0_h2_dn12, var_gds0_h2_dn17,)
    }
};
        var_gds0_h2 = assign33870_e49048;
        var_gds0_h2_dn0 = assign33870_e49048_d_n0;
        var_gds0_h2_dn2 = assign33870_e49048_d_n2;
        var_gds0_h2_dn6 = assign33870_e49048_d_n6;
        var_gds0_h2_dn7 = assign33870_e49048_d_n7;
        var_gds0_h2_dn10 = assign33870_e49048_d_n10;
        var_gds0_h2_dn11 = assign33870_e49048_d_n11;
        var_gds0_h2_dn12 = assign33870_e49048_d_n12;
        var_gds0_h2_dn17 = assign33870_e49048_d_n17;

        let (assign33880_e49054, assign33880_e49054_d_n0, assign33880_e49054_d_n2, assign33880_e49054_d_n6, assign33880_e49054_d_n7, assign33880_e49054_d_n10, assign33880_e49054_d_n11, assign33880_e49054_d_n12, assign33880_e49054_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33880_e49052: f64 = (var_nthrml / var_gds0_h2);
        (assign33880_e49052, (((var_nthrml_dn0 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn0)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn2 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn2)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn6 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn6)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn7 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn7)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn10 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn10)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn11 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn11)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn12 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn12)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn17 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn17)) / (var_gds0_h2 * var_gds0_h2)),)
    } else {
        (var_gamma, var_gamma_dn0, var_gamma_dn2, var_gamma_dn6, var_gamma_dn7, var_gamma_dn10, var_gamma_dn11, var_gamma_dn12, var_gamma_dn17,)
    }
};
        var_gamma = assign33880_e49054;
        var_gamma_dn0 = assign33880_e49054_d_n0;
        var_gamma_dn2 = assign33880_e49054_d_n2;
        var_gamma_dn6 = assign33880_e49054_d_n6;
        var_gamma_dn7 = assign33880_e49054_d_n7;
        var_gamma_dn10 = assign33880_e49054_d_n10;
        var_gamma_dn11 = assign33880_e49054_d_n11;
        var_gamma_dn12 = assign33880_e49054_d_n12;
        var_gamma_dn17 = assign33880_e49054_d_n17;

        let (assign33890_e49066, assign33890_e49066_d_n0, assign33890_e49066_d_n2, assign33890_e49066_d_n6, assign33890_e49066_d_n7, assign33890_e49066_d_n10, assign33890_e49066_d_n11, assign33890_e49066_d_n12, assign33890_e49066_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33890_e49059: f64 = (4.0 * var_vgvt);
        let assign33890_e49061: f64 = (assign33890_e49059 * var_sqrtkusail);
        let assign33890_e49062: f64 = (var_kusai00 + assign33890_e49061);
        let assign33890_e49064: f64 = (assign33890_e49062 + var_kusail);
        (assign33890_e49064, ((var_kusai00_dn0 + (((4.0 * var_vgvt_dn0) * var_sqrtkusail) + (assign33890_e49059 * var_sqrtkusail_dn0))) + var_kusail_dn0), ((var_kusai00_dn2 + (((4.0 * var_vgvt_dn2) * var_sqrtkusail) + (assign33890_e49059 * var_sqrtkusail_dn2))) + var_kusail_dn2), ((var_kusai00_dn6 + (((4.0 * var_vgvt_dn6) * var_sqrtkusail) + (assign33890_e49059 * var_sqrtkusail_dn6))) + var_kusail_dn6), ((var_kusai00_dn7 + (((4.0 * var_vgvt_dn7) * var_sqrtkusail) + (assign33890_e49059 * var_sqrtkusail_dn7))) + var_kusail_dn7), ((var_kusai00_dn10 + (((4.0 * var_vgvt_dn10) * var_sqrtkusail) + (assign33890_e49059 * var_sqrtkusail_dn10))) + var_kusail_dn10), ((var_kusai00_dn11 + (((4.0 * var_vgvt_dn11) * var_sqrtkusail) + (assign33890_e49059 * var_sqrtkusail_dn11))) + var_kusail_dn11), ((var_kusai00_dn12 + (((4.0 * var_vgvt_dn12) * var_sqrtkusail) + (assign33890_e49059 * var_sqrtkusail_dn12))) + var_kusail_dn12), ((var_kusai00_dn17 + (((4.0 * var_vgvt_dn17) * var_sqrtkusail) + (assign33890_e49059 * var_sqrtkusail_dn17))) + var_kusail_dn17),)
    } else {
        (var_t7w, var_t7w_dn0, var_t7w_dn2, var_t7w_dn6, var_t7w_dn7, var_t7w_dn10, var_t7w_dn11, var_t7w_dn12, var_t7w_dn17,)
    }
};
        var_t7w = assign33890_e49066;
        var_t7w_dn0 = assign33890_e49066_d_n0;
        var_t7w_dn2 = assign33890_e49066_d_n2;
        var_t7w_dn6 = assign33890_e49066_d_n6;
        var_t7w_dn7 = assign33890_e49066_d_n7;
        var_t7w_dn10 = assign33890_e49066_d_n10;
        var_t7w_dn11 = assign33890_e49066_d_n11;
        var_t7w_dn12 = assign33890_e49066_d_n12;
        var_t7w_dn17 = assign33890_e49066_d_n17;

        let (assign33900_e49087, assign33900_e49087_d_n0, assign33900_e49087_d_n2, assign33900_e49087_d_n6, assign33900_e49087_d_n7, assign33900_e49087_d_n10, assign33900_e49087_d_n11, assign33900_e49087_d_n12, assign33900_e49087_d_n17,) = {
    if (var_guard1125 != 0.0) {
        let assign33900_e49070: f64 = (3.872983346207417 * var_kusai00l);
        let assign33900_e49072: f64 = (assign33900_e49070 * var_t7w);
        let assign33900_e49075: f64 = (6.0 * var_t2__blk1114);
        let assign33900_e49078: f64 = (var_gamma * var_t2__blk1114);
        let assign33900_e49080: f64 = (assign33900_e49078 * var_vgvt);
        let assign33900_e49082: f64 = (assign33900_e49080 * var_t5__blk1117);
        let assign33900_e49083: f64 = (assign33900_e49082).sqrt();
        let assign33900_e49084: f64 = (assign33900_e49075 * assign33900_e49083);
        let assign33900_e49085: f64 = (assign33900_e49072 / assign33900_e49084);
        (assign33900_e49085, ((((((3.872983346207417 * var_kusai00l_dn0) * var_t7w) + (assign33900_e49070 * var_t7w_dn0)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * var_t2__blk1114_dn0) * assign33900_e49083) + (assign33900_e49075 * (((((((var_gamma_dn0 * var_t2__blk1114) + (var_gamma * var_t2__blk1114_dn0)) * var_vgvt) + (assign33900_e49078 * var_vgvt_dn0)) * var_t5__blk1117) + (assign33900_e49080 * var_t5__blk1117_dn0)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * var_kusai00l_dn2) * var_t7w) + (assign33900_e49070 * var_t7w_dn2)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * var_t2__blk1114_dn2) * assign33900_e49083) + (assign33900_e49075 * (((((((var_gamma_dn2 * var_t2__blk1114) + (var_gamma * var_t2__blk1114_dn2)) * var_vgvt) + (assign33900_e49078 * var_vgvt_dn2)) * var_t5__blk1117) + (assign33900_e49080 * var_t5__blk1117_dn2)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * var_kusai00l_dn6) * var_t7w) + (assign33900_e49070 * var_t7w_dn6)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * var_t2__blk1114_dn6) * assign33900_e49083) + (assign33900_e49075 * (((((((var_gamma_dn6 * var_t2__blk1114) + (var_gamma * var_t2__blk1114_dn6)) * var_vgvt) + (assign33900_e49078 * var_vgvt_dn6)) * var_t5__blk1117) + (assign33900_e49080 * var_t5__blk1117_dn6)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * var_kusai00l_dn7) * var_t7w) + (assign33900_e49070 * var_t7w_dn7)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * var_t2__blk1114_dn7) * assign33900_e49083) + (assign33900_e49075 * (((((((var_gamma_dn7 * var_t2__blk1114) + (var_gamma * var_t2__blk1114_dn7)) * var_vgvt) + (assign33900_e49078 * var_vgvt_dn7)) * var_t5__blk1117) + (assign33900_e49080 * var_t5__blk1117_dn7)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * var_kusai00l_dn10) * var_t7w) + (assign33900_e49070 * var_t7w_dn10)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * var_t2__blk1114_dn10) * assign33900_e49083) + (assign33900_e49075 * (((((((var_gamma_dn10 * var_t2__blk1114) + (var_gamma * var_t2__blk1114_dn10)) * var_vgvt) + (assign33900_e49078 * var_vgvt_dn10)) * var_t5__blk1117) + (assign33900_e49080 * var_t5__blk1117_dn10)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * var_kusai00l_dn11) * var_t7w) + (assign33900_e49070 * var_t7w_dn11)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * var_t2__blk1114_dn11) * assign33900_e49083) + (assign33900_e49075 * (((((((var_gamma_dn11 * var_t2__blk1114) + (var_gamma * var_t2__blk1114_dn11)) * var_vgvt) + (assign33900_e49078 * var_vgvt_dn11)) * var_t5__blk1117) + (assign33900_e49080 * var_t5__blk1117_dn11)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * var_kusai00l_dn12) * var_t7w) + (assign33900_e49070 * var_t7w_dn12)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * var_t2__blk1114_dn12) * assign33900_e49083) + (assign33900_e49075 * (((((((var_gamma_dn12 * var_t2__blk1114) + (var_gamma * var_t2__blk1114_dn12)) * var_vgvt) + (assign33900_e49078 * var_vgvt_dn12)) * var_t5__blk1117) + (assign33900_e49080 * var_t5__blk1117_dn12)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)), ((((((3.872983346207417 * var_kusai00l_dn17) * var_t7w) + (assign33900_e49070 * var_t7w_dn17)) * assign33900_e49084) - (assign33900_e49072 * (((6.0 * var_t2__blk1114_dn17) * assign33900_e49083) + (assign33900_e49075 * (((((((var_gamma_dn17 * var_t2__blk1114) + (var_gamma * var_t2__blk1114_dn17)) * var_vgvt) + (assign33900_e49078 * var_vgvt_dn17)) * var_t5__blk1117) + (assign33900_e49080 * var_t5__blk1117_dn17)) / (2.0 * assign33900_e49083)))))) / (assign33900_e49084 * assign33900_e49084)),)
    } else {
        (var_crl_f, var_crl_f_dn0, var_crl_f_dn2, var_crl_f_dn6, var_crl_f_dn7, var_crl_f_dn10, var_crl_f_dn11, var_crl_f_dn12, var_crl_f_dn17,)
    }
};
        var_crl_f = assign33900_e49087;
        var_crl_f_dn0 = assign33900_e49087_d_n0;
        var_crl_f_dn2 = assign33900_e49087_d_n2;
        var_crl_f_dn6 = assign33900_e49087_d_n6;
        var_crl_f_dn7 = assign33900_e49087_d_n7;
        var_crl_f_dn10 = assign33900_e49087_d_n10;
        var_crl_f_dn11 = assign33900_e49087_d_n11;
        var_crl_f_dn12 = assign33900_e49087_d_n12;
        var_crl_f_dn17 = assign33900_e49087_d_n17;

        let assign33910_e49090: f64 = (var_ids + var_idsibpc);
        var_ids = assign33910_e49090;
        var_ids_dn0 = (var_ids_dn0 + var_idsibpc_dn0);
        var_ids_dn2 = (var_ids_dn2 + var_idsibpc_dn2);
        var_ids_dn6 = (var_ids_dn6 + var_idsibpc_dn6);
        var_ids_dn7 = (var_ids_dn7 + var_idsibpc_dn7);
        var_ids_dn10 = (var_ids_dn10 + var_idsibpc_dn10);
        var_ids_dn11 = (var_ids_dn11 + var_idsibpc_dn11);
        var_ids_dn12 = (var_ids_dn12 + var_idsibpc_dn12);
        var_ids_dn17 = (var_ids_dn17 + var_idsibpc_dn17);

        let assign33920_e49093: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1126 = assign33920_e49093;

        let (assign33930_e49099,) = {
    if (var_guard1126 != 0.0) {
        let assign33930_e49097: f64 = (var_cbtp + var_cbtn);
        (assign33930_e49097,)
    } else {
        (var_cgbe,)
    }
};
        var_cgbe = assign33930_e49099;

        let (assign33940_e49109,) = {
    if ((var_guard1126 != 0.0) && (var_cgbo_given != 0.0)) {
        let assign33940_e49106: f64 = (p.p168 * var_lgleff);
        let assign33940_e49107: f64 = (var_cgbe - assign33940_e49106);
        (assign33940_e49107,)
    } else {
        (var_cgbe,)
    }
};
        var_cgbe = assign33940_e49109;

        *var_cgbe_slot = var_cgbe;
        *var_crl_f_slot = var_crl_f;
        *var_crl_f_dn0_slot = var_crl_f_dn0;
        *var_crl_f_dn10_slot = var_crl_f_dn10;
        *var_crl_f_dn11_slot = var_crl_f_dn11;
        *var_crl_f_dn12_slot = var_crl_f_dn12;
        *var_crl_f_dn17_slot = var_crl_f_dn17;
        *var_crl_f_dn2_slot = var_crl_f_dn2;
        *var_crl_f_dn6_slot = var_crl_f_dn6;
        *var_crl_f_dn7_slot = var_crl_f_dn7;
        *var_gamma_slot = var_gamma;
        *var_gamma_dn0_slot = var_gamma_dn0;
        *var_gamma_dn10_slot = var_gamma_dn10;
        *var_gamma_dn11_slot = var_gamma_dn11;
        *var_gamma_dn12_slot = var_gamma_dn12;
        *var_gamma_dn17_slot = var_gamma_dn17;
        *var_gamma_dn2_slot = var_gamma_dn2;
        *var_gamma_dn6_slot = var_gamma_dn6;
        *var_gamma_dn7_slot = var_gamma_dn7;
        *var_gds0_h2_slot = var_gds0_h2;
        *var_gds0_h2_dn0_slot = var_gds0_h2_dn0;
        *var_gds0_h2_dn10_slot = var_gds0_h2_dn10;
        *var_gds0_h2_dn11_slot = var_gds0_h2_dn11;
        *var_gds0_h2_dn12_slot = var_gds0_h2_dn12;
        *var_gds0_h2_dn17_slot = var_gds0_h2_dn17;
        *var_gds0_h2_dn2_slot = var_gds0_h2_dn2;
        *var_gds0_h2_dn6_slot = var_gds0_h2_dn6;
        *var_gds0_h2_dn7_slot = var_gds0_h2_dn7;
        *var_gds0_ign_slot = var_gds0_ign;
        *var_gds0_ign_dn0_slot = var_gds0_ign_dn0;
        *var_gds0_ign_dn10_slot = var_gds0_ign_dn10;
        *var_gds0_ign_dn11_slot = var_gds0_ign_dn11;
        *var_gds0_ign_dn12_slot = var_gds0_ign_dn12;
        *var_gds0_ign_dn17_slot = var_gds0_ign_dn17;
        *var_gds0_ign_dn2_slot = var_gds0_ign_dn2;
        *var_gds0_ign_dn6_slot = var_gds0_ign_dn6;
        *var_gds0_ign_dn7_slot = var_gds0_ign_dn7;
        *var_guard1125_slot = var_guard1125;
        *var_guard1126_slot = var_guard1126;
        *var_ids_slot = var_ids;
        *var_ids_dn0_slot = var_ids_dn0;
        *var_ids_dn10_slot = var_ids_dn10;
        *var_ids_dn11_slot = var_ids_dn11;
        *var_ids_dn12_slot = var_ids_dn12;
        *var_ids_dn17_slot = var_ids_dn17;
        *var_ids_dn2_slot = var_ids_dn2;
        *var_ids_dn6_slot = var_ids_dn6;
        *var_ids_dn7_slot = var_ids_dn7;
        *var_kusai_ig_slot = var_kusai_ig;
        *var_kusai_ig_dn0_slot = var_kusai_ig_dn0;
        *var_kusai_ig_dn10_slot = var_kusai_ig_dn10;
        *var_kusai_ig_dn11_slot = var_kusai_ig_dn11;
        *var_kusai_ig_dn12_slot = var_kusai_ig_dn12;
        *var_kusai_ig_dn17_slot = var_kusai_ig_dn17;
        *var_kusai_ig_dn2_slot = var_kusai_ig_dn2;
        *var_kusai_ig_dn6_slot = var_kusai_ig_dn6;
        *var_kusai_ig_dn7_slot = var_kusai_ig_dn7;
        *var_mu_ave_slot = var_mu_ave;
        *var_mu_ave_dn0_slot = var_mu_ave_dn0;
        *var_mu_ave_dn10_slot = var_mu_ave_dn10;
        *var_mu_ave_dn11_slot = var_mu_ave_dn11;
        *var_mu_ave_dn12_slot = var_mu_ave_dn12;
        *var_mu_ave_dn17_slot = var_mu_ave_dn17;
        *var_mu_ave_dn2_slot = var_mu_ave_dn2;
        *var_mu_ave_dn6_slot = var_mu_ave_dn6;
        *var_mu_ave_dn7_slot = var_mu_ave_dn7;
        *var_mud_hoso_slot = var_mud_hoso;
        *var_mud_hoso_dn0_slot = var_mud_hoso_dn0;
        *var_mud_hoso_dn10_slot = var_mud_hoso_dn10;
        *var_mud_hoso_dn11_slot = var_mud_hoso_dn11;
        *var_mud_hoso_dn12_slot = var_mud_hoso_dn12;
        *var_mud_hoso_dn17_slot = var_mud_hoso_dn17;
        *var_mud_hoso_dn2_slot = var_mud_hoso_dn2;
        *var_mud_hoso_dn6_slot = var_mud_hoso_dn6;
        *var_mud_hoso_dn7_slot = var_mud_hoso_dn7;
        *var_nthrml_slot = var_nthrml;
        *var_nthrml_dn0_slot = var_nthrml_dn0;
        *var_nthrml_dn10_slot = var_nthrml_dn10;
        *var_nthrml_dn11_slot = var_nthrml_dn11;
        *var_nthrml_dn12_slot = var_nthrml_dn12;
        *var_nthrml_dn17_slot = var_nthrml_dn17;
        *var_nthrml_dn2_slot = var_nthrml_dn2;
        *var_nthrml_dn6_slot = var_nthrml_dn6;
        *var_nthrml_dn7_slot = var_nthrml_dn7;
        *var_sqrtkusail_slot = var_sqrtkusail;
        *var_sqrtkusail_dn0_slot = var_sqrtkusail_dn0;
        *var_sqrtkusail_dn10_slot = var_sqrtkusail_dn10;
        *var_sqrtkusail_dn11_slot = var_sqrtkusail_dn11;
        *var_sqrtkusail_dn12_slot = var_sqrtkusail_dn12;
        *var_sqrtkusail_dn17_slot = var_sqrtkusail_dn17;
        *var_sqrtkusail_dn2_slot = var_sqrtkusail_dn2;
        *var_sqrtkusail_dn6_slot = var_sqrtkusail_dn6;
        *var_sqrtkusail_dn7_slot = var_sqrtkusail_dn7;
        *var_t0__blk1112_slot = var_t0__blk1112;
        *var_t0__blk1112_dn0_slot = var_t0__blk1112_dn0;
        *var_t0__blk1112_dn10_slot = var_t0__blk1112_dn10;
        *var_t0__blk1112_dn11_slot = var_t0__blk1112_dn11;
        *var_t0__blk1112_dn12_slot = var_t0__blk1112_dn12;
        *var_t0__blk1112_dn17_slot = var_t0__blk1112_dn17;
        *var_t0__blk1112_dn2_slot = var_t0__blk1112_dn2;
        *var_t0__blk1112_dn6_slot = var_t0__blk1112_dn6;
        *var_t0__blk1112_dn7_slot = var_t0__blk1112_dn7;
        *var_t10__blk1110_slot = var_t10__blk1110;
        *var_t10__blk1110_dn0_slot = var_t10__blk1110_dn0;
        *var_t10__blk1110_dn10_slot = var_t10__blk1110_dn10;
        *var_t10__blk1110_dn11_slot = var_t10__blk1110_dn11;
        *var_t10__blk1110_dn12_slot = var_t10__blk1110_dn12;
        *var_t10__blk1110_dn17_slot = var_t10__blk1110_dn17;
        *var_t10__blk1110_dn2_slot = var_t10__blk1110_dn2;
        *var_t10__blk1110_dn6_slot = var_t10__blk1110_dn6;
        *var_t10__blk1110_dn7_slot = var_t10__blk1110_dn7;
        *var_t10w_slot = var_t10w;
        *var_t10w_dn0_slot = var_t10w_dn0;
        *var_t10w_dn10_slot = var_t10w_dn10;
        *var_t10w_dn11_slot = var_t10w_dn11;
        *var_t10w_dn12_slot = var_t10w_dn12;
        *var_t10w_dn17_slot = var_t10w_dn17;
        *var_t10w_dn2_slot = var_t10w_dn2;
        *var_t10w_dn6_slot = var_t10w_dn6;
        *var_t10w_dn7_slot = var_t10w_dn7;
        *var_t11__blk1111_slot = var_t11__blk1111;
        *var_t11__blk1111_dn0_slot = var_t11__blk1111_dn0;
        *var_t11__blk1111_dn10_slot = var_t11__blk1111_dn10;
        *var_t11__blk1111_dn11_slot = var_t11__blk1111_dn11;
        *var_t11__blk1111_dn12_slot = var_t11__blk1111_dn12;
        *var_t11__blk1111_dn17_slot = var_t11__blk1111_dn17;
        *var_t11__blk1111_dn2_slot = var_t11__blk1111_dn2;
        *var_t11__blk1111_dn6_slot = var_t11__blk1111_dn6;
        *var_t11__blk1111_dn7_slot = var_t11__blk1111_dn7;
        *var_t2__blk1114_slot = var_t2__blk1114;
        *var_t2__blk1114_dn0_slot = var_t2__blk1114_dn0;
        *var_t2__blk1114_dn10_slot = var_t2__blk1114_dn10;
        *var_t2__blk1114_dn11_slot = var_t2__blk1114_dn11;
        *var_t2__blk1114_dn12_slot = var_t2__blk1114_dn12;
        *var_t2__blk1114_dn17_slot = var_t2__blk1114_dn17;
        *var_t2__blk1114_dn2_slot = var_t2__blk1114_dn2;
        *var_t2__blk1114_dn6_slot = var_t2__blk1114_dn6;
        *var_t2__blk1114_dn7_slot = var_t2__blk1114_dn7;
        *var_t3__blk1115_slot = var_t3__blk1115;
        *var_t3__blk1115_dn0_slot = var_t3__blk1115_dn0;
        *var_t3__blk1115_dn10_slot = var_t3__blk1115_dn10;
        *var_t3__blk1115_dn11_slot = var_t3__blk1115_dn11;
        *var_t3__blk1115_dn12_slot = var_t3__blk1115_dn12;
        *var_t3__blk1115_dn17_slot = var_t3__blk1115_dn17;
        *var_t3__blk1115_dn2_slot = var_t3__blk1115_dn2;
        *var_t3__blk1115_dn6_slot = var_t3__blk1115_dn6;
        *var_t3__blk1115_dn7_slot = var_t3__blk1115_dn7;
        *var_t4__blk1116_slot = var_t4__blk1116;
        *var_t4__blk1116_dn0_slot = var_t4__blk1116_dn0;
        *var_t4__blk1116_dn10_slot = var_t4__blk1116_dn10;
        *var_t4__blk1116_dn11_slot = var_t4__blk1116_dn11;
        *var_t4__blk1116_dn12_slot = var_t4__blk1116_dn12;
        *var_t4__blk1116_dn17_slot = var_t4__blk1116_dn17;
        *var_t4__blk1116_dn2_slot = var_t4__blk1116_dn2;
        *var_t4__blk1116_dn6_slot = var_t4__blk1116_dn6;
        *var_t4__blk1116_dn7_slot = var_t4__blk1116_dn7;
        *var_t5__blk1117_slot = var_t5__blk1117;
        *var_t5__blk1117_dn0_slot = var_t5__blk1117_dn0;
        *var_t5__blk1117_dn10_slot = var_t5__blk1117_dn10;
        *var_t5__blk1117_dn11_slot = var_t5__blk1117_dn11;
        *var_t5__blk1117_dn12_slot = var_t5__blk1117_dn12;
        *var_t5__blk1117_dn17_slot = var_t5__blk1117_dn17;
        *var_t5__blk1117_dn2_slot = var_t5__blk1117_dn2;
        *var_t5__blk1117_dn6_slot = var_t5__blk1117_dn6;
        *var_t5__blk1117_dn7_slot = var_t5__blk1117_dn7;
        *var_t7w_slot = var_t7w;
        *var_t7w_dn0_slot = var_t7w_dn0;
        *var_t7w_dn10_slot = var_t7w_dn10;
        *var_t7w_dn11_slot = var_t7w_dn11;
        *var_t7w_dn12_slot = var_t7w_dn12;
        *var_t7w_dn17_slot = var_t7w_dn17;
        *var_t7w_dn2_slot = var_t7w_dn2;
        *var_t7w_dn6_slot = var_t7w_dn6;
        *var_t7w_dn7_slot = var_t7w_dn7;
        *var_t9__blk1109_slot = var_t9__blk1109;
        *var_t9__blk1109_dn0_slot = var_t9__blk1109_dn0;
        *var_t9__blk1109_dn10_slot = var_t9__blk1109_dn10;
        *var_t9__blk1109_dn11_slot = var_t9__blk1109_dn11;
        *var_t9__blk1109_dn12_slot = var_t9__blk1109_dn12;
        *var_t9__blk1109_dn17_slot = var_t9__blk1109_dn17;
        *var_t9__blk1109_dn2_slot = var_t9__blk1109_dn2;
        *var_t9__blk1109_dn6_slot = var_t9__blk1109_dn6;
        *var_t9__blk1109_dn7_slot = var_t9__blk1109_dn7;
    }

    pub(super) fn stamp_transient_block_119(
        p: &Parameters,
        var_cgbo_given: f64,
        var_flg_nqs: f64,
        var_guard1126: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn12: f64,
        var_ids_dn17: f64,
        var_ids_dn2: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_lgleff: f64,
        var_mfactor: f64,
        var_qb: f64,
        var_qb_dn0: f64,
        var_qb_dn10: f64,
        var_qb_dn11: f64,
        var_qb_dn12: f64,
        var_qb_dn13: f64,
        var_qb_dn15: f64,
        var_qb_dn16: f64,
        var_qb_dn17: f64,
        var_qb_dn18: f64,
        var_qb_dn2: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qd: f64,
        var_qd_dn0: f64,
        var_qd_dn10: f64,
        var_qd_dn11: f64,
        var_qd_dn12: f64,
        var_qd_dn13: f64,
        var_qd_dn15: f64,
        var_qd_dn16: f64,
        var_qd_dn17: f64,
        var_qd_dn18: f64,
        var_qd_dn2: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_fb: f64,
        var_qd_fb_dn0: f64,
        var_qd_fb_dn10: f64,
        var_qd_fb_dn11: f64,
        var_qd_fb_dn12: f64,
        var_qd_fb_dn13: f64,
        var_qd_fb_dn15: f64,
        var_qd_fb_dn16: f64,
        var_qd_fb_dn17: f64,
        var_qd_fb_dn18: f64,
        var_qd_fb_dn2: f64,
        var_qd_fb_dn6: f64,
        var_qd_fb_dn7: f64,
        var_qdrat: f64,
        var_qdrat_dn0: f64,
        var_qdrat_dn10: f64,
        var_qdrat_dn11: f64,
        var_qdrat_dn12: f64,
        var_qdrat_dn17: f64,
        var_qdrat_dn2: f64,
        var_qdrat_dn6: f64,
        var_qdrat_dn7: f64,
        var_qi: f64,
        var_qi_dn0: f64,
        var_qi_dn10: f64,
        var_qi_dn11: f64,
        var_qi_dn12: f64,
        var_qi_dn17: f64,
        var_qi_dn2: f64,
        var_qi_dn6: f64,
        var_qi_dn7: f64,
        var_qsub: f64,
        var_qsub_dn0: f64,
        var_qsub_dn10: f64,
        var_qsub_dn11: f64,
        var_qsub_dn12: f64,
        var_qsub_dn17: f64,
        var_qsub_dn2: f64,
        var_qsub_dn6: f64,
        var_qsub_dn7: f64,
        var_uc_pdbcp: f64,
        var_uc_psbcp: f64,
        var_vbsp: f64,
        var_vbsp_dn0: f64,
        var_vbsp_dn10: f64,
        var_vbsp_dn11: f64,
        var_vbsp_dn12: f64,
        var_vbsp_dn17: f64,
        var_vbsp_dn2: f64,
        var_vbsp_dn6: f64,
        var_vbsp_dn7: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn17: f64,
        var_vds_dn2: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vgs: f64,
        var_vgs_dn11: f64,
        var_vgs_dn6: f64,
        var_vgs_dn7: f64,
        var_wgate: f64,
        var_cf_slot: &mut f64,
        var_cfd_slot: &mut f64,
        var_cfs_slot: &mut f64,
        var_cfu_slot: &mut f64,
        var_cgbe_slot: &mut f64,
        var_guard1127_slot: &mut f64,
        var_idse_slot: &mut f64,
        var_idse_dn0_slot: &mut f64,
        var_idse_dn10_slot: &mut f64,
        var_idse_dn11_slot: &mut f64,
        var_idse_dn12_slot: &mut f64,
        var_idse_dn17_slot: &mut f64,
        var_idse_dn2_slot: &mut f64,
        var_idse_dn6_slot: &mut f64,
        var_idse_dn7_slot: &mut f64,
        var_qb_qs_slot: &mut f64,
        var_qb_qs_dn0_slot: &mut f64,
        var_qb_qs_dn10_slot: &mut f64,
        var_qb_qs_dn11_slot: &mut f64,
        var_qb_qs_dn12_slot: &mut f64,
        var_qb_qs_dn13_slot: &mut f64,
        var_qb_qs_dn15_slot: &mut f64,
        var_qb_qs_dn16_slot: &mut f64,
        var_qb_qs_dn17_slot: &mut f64,
        var_qb_qs_dn18_slot: &mut f64,
        var_qb_qs_dn2_slot: &mut f64,
        var_qb_qs_dn6_slot: &mut f64,
        var_qb_qs_dn7_slot: &mut f64,
        var_qbe_slot: &mut f64,
        var_qbe_dn0_slot: &mut f64,
        var_qbe_dn10_slot: &mut f64,
        var_qbe_dn11_slot: &mut f64,
        var_qbe_dn12_slot: &mut f64,
        var_qbe_dn13_slot: &mut f64,
        var_qbe_dn15_slot: &mut f64,
        var_qbe_dn16_slot: &mut f64,
        var_qbe_dn17_slot: &mut f64,
        var_qbe_dn18_slot: &mut f64,
        var_qbe_dn2_slot: &mut f64,
        var_qbe_dn6_slot: &mut f64,
        var_qbe_dn7_slot: &mut f64,
        var_qd_qs_slot: &mut f64,
        var_qd_qs_dn0_slot: &mut f64,
        var_qd_qs_dn10_slot: &mut f64,
        var_qd_qs_dn11_slot: &mut f64,
        var_qd_qs_dn12_slot: &mut f64,
        var_qd_qs_dn13_slot: &mut f64,
        var_qd_qs_dn15_slot: &mut f64,
        var_qd_qs_dn16_slot: &mut f64,
        var_qd_qs_dn17_slot: &mut f64,
        var_qd_qs_dn18_slot: &mut f64,
        var_qd_qs_dn2_slot: &mut f64,
        var_qd_qs_dn6_slot: &mut f64,
        var_qd_qs_dn7_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_dn0_slot: &mut f64,
        var_qde_dn10_slot: &mut f64,
        var_qde_dn11_slot: &mut f64,
        var_qde_dn12_slot: &mut f64,
        var_qde_dn13_slot: &mut f64,
        var_qde_dn15_slot: &mut f64,
        var_qde_dn16_slot: &mut f64,
        var_qde_dn17_slot: &mut f64,
        var_qde_dn18_slot: &mut f64,
        var_qde_dn2_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qde_dn7_slot: &mut f64,
        var_qfbc_slot: &mut f64,
        var_qfbc_dn0_slot: &mut f64,
        var_qfbc_dn10_slot: &mut f64,
        var_qfbc_dn11_slot: &mut f64,
        var_qfbc_dn12_slot: &mut f64,
        var_qfbc_dn17_slot: &mut f64,
        var_qfbc_dn2_slot: &mut f64,
        var_qfbc_dn6_slot: &mut f64,
        var_qfbc_dn7_slot: &mut f64,
        var_qfd_slot: &mut f64,
        var_qfd_dn0_slot: &mut f64,
        var_qfd_dn10_slot: &mut f64,
        var_qfd_dn11_slot: &mut f64,
        var_qfd_dn12_slot: &mut f64,
        var_qfd_dn17_slot: &mut f64,
        var_qfd_dn2_slot: &mut f64,
        var_qfd_dn6_slot: &mut f64,
        var_qfd_dn7_slot: &mut f64,
        var_qfs_slot: &mut f64,
        var_qfs_dn11_slot: &mut f64,
        var_qfs_dn6_slot: &mut f64,
        var_qfs_dn7_slot: &mut f64,
        var_qge_slot: &mut f64,
        var_qge_dn0_slot: &mut f64,
        var_qge_dn10_slot: &mut f64,
        var_qge_dn11_slot: &mut f64,
        var_qge_dn12_slot: &mut f64,
        var_qge_dn13_slot: &mut f64,
        var_qge_dn15_slot: &mut f64,
        var_qge_dn16_slot: &mut f64,
        var_qge_dn17_slot: &mut f64,
        var_qge_dn18_slot: &mut f64,
        var_qge_dn2_slot: &mut f64,
        var_qge_dn6_slot: &mut f64,
        var_qge_dn7_slot: &mut f64,
        var_qgob_slot: &mut f64,
        var_qgob_dn0_slot: &mut f64,
        var_qgob_dn10_slot: &mut f64,
        var_qgob_dn11_slot: &mut f64,
        var_qgob_dn12_slot: &mut f64,
        var_qgob_dn17_slot: &mut f64,
        var_qgob_dn2_slot: &mut f64,
        var_qgob_dn6_slot: &mut f64,
        var_qgob_dn7_slot: &mut f64,
        var_qgod_slot: &mut f64,
        var_qgod_dn0_slot: &mut f64,
        var_qgod_dn10_slot: &mut f64,
        var_qgod_dn11_slot: &mut f64,
        var_qgod_dn12_slot: &mut f64,
        var_qgod_dn17_slot: &mut f64,
        var_qgod_dn2_slot: &mut f64,
        var_qgod_dn6_slot: &mut f64,
        var_qgod_dn7_slot: &mut f64,
        var_qgos_slot: &mut f64,
        var_qgos_dn0_slot: &mut f64,
        var_qgos_dn10_slot: &mut f64,
        var_qgos_dn11_slot: &mut f64,
        var_qgos_dn12_slot: &mut f64,
        var_qgos_dn17_slot: &mut f64,
        var_qgos_dn2_slot: &mut f64,
        var_qgos_dn6_slot: &mut f64,
        var_qgos_dn7_slot: &mut f64,
        var_qi_qs_slot: &mut f64,
        var_qi_qs_dn0_slot: &mut f64,
        var_qi_qs_dn10_slot: &mut f64,
        var_qi_qs_dn11_slot: &mut f64,
        var_qi_qs_dn12_slot: &mut f64,
        var_qi_qs_dn17_slot: &mut f64,
        var_qi_qs_dn2_slot: &mut f64,
        var_qi_qs_dn6_slot: &mut f64,
        var_qi_qs_dn7_slot: &mut f64,
        var_qse_slot: &mut f64,
        var_qse_dn0_slot: &mut f64,
        var_qse_dn10_slot: &mut f64,
        var_qse_dn11_slot: &mut f64,
        var_qse_dn12_slot: &mut f64,
        var_qse_dn13_slot: &mut f64,
        var_qse_dn15_slot: &mut f64,
        var_qse_dn16_slot: &mut f64,
        var_qse_dn17_slot: &mut f64,
        var_qse_dn18_slot: &mut f64,
        var_qse_dn2_slot: &mut f64,
        var_qse_dn6_slot: &mut f64,
        var_qse_dn7_slot: &mut f64,
        var_xd_slot: &mut f64,
        var_xd_dn0_slot: &mut f64,
        var_xd_dn10_slot: &mut f64,
        var_xd_dn11_slot: &mut f64,
        var_xd_dn12_slot: &mut f64,
        var_xd_dn17_slot: &mut f64,
        var_xd_dn2_slot: &mut f64,
        var_xd_dn6_slot: &mut f64,
        var_xd_dn7_slot: &mut f64,
    ) {
        let mut var_cf: f64 = *var_cf_slot;
        let mut var_cfd: f64 = *var_cfd_slot;
        let mut var_cfs: f64 = *var_cfs_slot;
        let mut var_cfu: f64 = *var_cfu_slot;
        let mut var_cgbe: f64 = *var_cgbe_slot;
        let mut var_guard1127: f64 = *var_guard1127_slot;
        let mut var_idse: f64 = *var_idse_slot;
        let mut var_idse_dn0: f64 = *var_idse_dn0_slot;
        let mut var_idse_dn10: f64 = *var_idse_dn10_slot;
        let mut var_idse_dn11: f64 = *var_idse_dn11_slot;
        let mut var_idse_dn12: f64 = *var_idse_dn12_slot;
        let mut var_idse_dn17: f64 = *var_idse_dn17_slot;
        let mut var_idse_dn2: f64 = *var_idse_dn2_slot;
        let mut var_idse_dn6: f64 = *var_idse_dn6_slot;
        let mut var_idse_dn7: f64 = *var_idse_dn7_slot;
        let mut var_qb_qs: f64 = *var_qb_qs_slot;
        let mut var_qb_qs_dn0: f64 = *var_qb_qs_dn0_slot;
        let mut var_qb_qs_dn10: f64 = *var_qb_qs_dn10_slot;
        let mut var_qb_qs_dn11: f64 = *var_qb_qs_dn11_slot;
        let mut var_qb_qs_dn12: f64 = *var_qb_qs_dn12_slot;
        let mut var_qb_qs_dn13: f64 = *var_qb_qs_dn13_slot;
        let mut var_qb_qs_dn15: f64 = *var_qb_qs_dn15_slot;
        let mut var_qb_qs_dn16: f64 = *var_qb_qs_dn16_slot;
        let mut var_qb_qs_dn17: f64 = *var_qb_qs_dn17_slot;
        let mut var_qb_qs_dn18: f64 = *var_qb_qs_dn18_slot;
        let mut var_qb_qs_dn2: f64 = *var_qb_qs_dn2_slot;
        let mut var_qb_qs_dn6: f64 = *var_qb_qs_dn6_slot;
        let mut var_qb_qs_dn7: f64 = *var_qb_qs_dn7_slot;
        let mut var_qbe: f64 = *var_qbe_slot;
        let mut var_qbe_dn0: f64 = *var_qbe_dn0_slot;
        let mut var_qbe_dn10: f64 = *var_qbe_dn10_slot;
        let mut var_qbe_dn11: f64 = *var_qbe_dn11_slot;
        let mut var_qbe_dn12: f64 = *var_qbe_dn12_slot;
        let mut var_qbe_dn13: f64 = *var_qbe_dn13_slot;
        let mut var_qbe_dn15: f64 = *var_qbe_dn15_slot;
        let mut var_qbe_dn16: f64 = *var_qbe_dn16_slot;
        let mut var_qbe_dn17: f64 = *var_qbe_dn17_slot;
        let mut var_qbe_dn18: f64 = *var_qbe_dn18_slot;
        let mut var_qbe_dn2: f64 = *var_qbe_dn2_slot;
        let mut var_qbe_dn6: f64 = *var_qbe_dn6_slot;
        let mut var_qbe_dn7: f64 = *var_qbe_dn7_slot;
        let mut var_qd_qs: f64 = *var_qd_qs_slot;
        let mut var_qd_qs_dn0: f64 = *var_qd_qs_dn0_slot;
        let mut var_qd_qs_dn10: f64 = *var_qd_qs_dn10_slot;
        let mut var_qd_qs_dn11: f64 = *var_qd_qs_dn11_slot;
        let mut var_qd_qs_dn12: f64 = *var_qd_qs_dn12_slot;
        let mut var_qd_qs_dn13: f64 = *var_qd_qs_dn13_slot;
        let mut var_qd_qs_dn15: f64 = *var_qd_qs_dn15_slot;
        let mut var_qd_qs_dn16: f64 = *var_qd_qs_dn16_slot;
        let mut var_qd_qs_dn17: f64 = *var_qd_qs_dn17_slot;
        let mut var_qd_qs_dn18: f64 = *var_qd_qs_dn18_slot;
        let mut var_qd_qs_dn2: f64 = *var_qd_qs_dn2_slot;
        let mut var_qd_qs_dn6: f64 = *var_qd_qs_dn6_slot;
        let mut var_qd_qs_dn7: f64 = *var_qd_qs_dn7_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_dn0: f64 = *var_qde_dn0_slot;
        let mut var_qde_dn10: f64 = *var_qde_dn10_slot;
        let mut var_qde_dn11: f64 = *var_qde_dn11_slot;
        let mut var_qde_dn12: f64 = *var_qde_dn12_slot;
        let mut var_qde_dn13: f64 = *var_qde_dn13_slot;
        let mut var_qde_dn15: f64 = *var_qde_dn15_slot;
        let mut var_qde_dn16: f64 = *var_qde_dn16_slot;
        let mut var_qde_dn17: f64 = *var_qde_dn17_slot;
        let mut var_qde_dn18: f64 = *var_qde_dn18_slot;
        let mut var_qde_dn2: f64 = *var_qde_dn2_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qde_dn7: f64 = *var_qde_dn7_slot;
        let mut var_qfbc: f64 = *var_qfbc_slot;
        let mut var_qfbc_dn0: f64 = *var_qfbc_dn0_slot;
        let mut var_qfbc_dn10: f64 = *var_qfbc_dn10_slot;
        let mut var_qfbc_dn11: f64 = *var_qfbc_dn11_slot;
        let mut var_qfbc_dn12: f64 = *var_qfbc_dn12_slot;
        let mut var_qfbc_dn17: f64 = *var_qfbc_dn17_slot;
        let mut var_qfbc_dn2: f64 = *var_qfbc_dn2_slot;
        let mut var_qfbc_dn6: f64 = *var_qfbc_dn6_slot;
        let mut var_qfbc_dn7: f64 = *var_qfbc_dn7_slot;
        let mut var_qfd: f64 = *var_qfd_slot;
        let mut var_qfd_dn0: f64 = *var_qfd_dn0_slot;
        let mut var_qfd_dn10: f64 = *var_qfd_dn10_slot;
        let mut var_qfd_dn11: f64 = *var_qfd_dn11_slot;
        let mut var_qfd_dn12: f64 = *var_qfd_dn12_slot;
        let mut var_qfd_dn17: f64 = *var_qfd_dn17_slot;
        let mut var_qfd_dn2: f64 = *var_qfd_dn2_slot;
        let mut var_qfd_dn6: f64 = *var_qfd_dn6_slot;
        let mut var_qfd_dn7: f64 = *var_qfd_dn7_slot;
        let mut var_qfs: f64 = *var_qfs_slot;
        let mut var_qfs_dn11: f64 = *var_qfs_dn11_slot;
        let mut var_qfs_dn6: f64 = *var_qfs_dn6_slot;
        let mut var_qfs_dn7: f64 = *var_qfs_dn7_slot;
        let mut var_qge: f64 = *var_qge_slot;
        let mut var_qge_dn0: f64 = *var_qge_dn0_slot;
        let mut var_qge_dn10: f64 = *var_qge_dn10_slot;
        let mut var_qge_dn11: f64 = *var_qge_dn11_slot;
        let mut var_qge_dn12: f64 = *var_qge_dn12_slot;
        let mut var_qge_dn13: f64 = *var_qge_dn13_slot;
        let mut var_qge_dn15: f64 = *var_qge_dn15_slot;
        let mut var_qge_dn16: f64 = *var_qge_dn16_slot;
        let mut var_qge_dn17: f64 = *var_qge_dn17_slot;
        let mut var_qge_dn18: f64 = *var_qge_dn18_slot;
        let mut var_qge_dn2: f64 = *var_qge_dn2_slot;
        let mut var_qge_dn6: f64 = *var_qge_dn6_slot;
        let mut var_qge_dn7: f64 = *var_qge_dn7_slot;
        let mut var_qgob: f64 = *var_qgob_slot;
        let mut var_qgob_dn0: f64 = *var_qgob_dn0_slot;
        let mut var_qgob_dn10: f64 = *var_qgob_dn10_slot;
        let mut var_qgob_dn11: f64 = *var_qgob_dn11_slot;
        let mut var_qgob_dn12: f64 = *var_qgob_dn12_slot;
        let mut var_qgob_dn17: f64 = *var_qgob_dn17_slot;
        let mut var_qgob_dn2: f64 = *var_qgob_dn2_slot;
        let mut var_qgob_dn6: f64 = *var_qgob_dn6_slot;
        let mut var_qgob_dn7: f64 = *var_qgob_dn7_slot;
        let mut var_qgod: f64 = *var_qgod_slot;
        let mut var_qgod_dn0: f64 = *var_qgod_dn0_slot;
        let mut var_qgod_dn10: f64 = *var_qgod_dn10_slot;
        let mut var_qgod_dn11: f64 = *var_qgod_dn11_slot;
        let mut var_qgod_dn12: f64 = *var_qgod_dn12_slot;
        let mut var_qgod_dn17: f64 = *var_qgod_dn17_slot;
        let mut var_qgod_dn2: f64 = *var_qgod_dn2_slot;
        let mut var_qgod_dn6: f64 = *var_qgod_dn6_slot;
        let mut var_qgod_dn7: f64 = *var_qgod_dn7_slot;
        let mut var_qgos: f64 = *var_qgos_slot;
        let mut var_qgos_dn0: f64 = *var_qgos_dn0_slot;
        let mut var_qgos_dn10: f64 = *var_qgos_dn10_slot;
        let mut var_qgos_dn11: f64 = *var_qgos_dn11_slot;
        let mut var_qgos_dn12: f64 = *var_qgos_dn12_slot;
        let mut var_qgos_dn17: f64 = *var_qgos_dn17_slot;
        let mut var_qgos_dn2: f64 = *var_qgos_dn2_slot;
        let mut var_qgos_dn6: f64 = *var_qgos_dn6_slot;
        let mut var_qgos_dn7: f64 = *var_qgos_dn7_slot;
        let mut var_qi_qs: f64 = *var_qi_qs_slot;
        let mut var_qi_qs_dn0: f64 = *var_qi_qs_dn0_slot;
        let mut var_qi_qs_dn10: f64 = *var_qi_qs_dn10_slot;
        let mut var_qi_qs_dn11: f64 = *var_qi_qs_dn11_slot;
        let mut var_qi_qs_dn12: f64 = *var_qi_qs_dn12_slot;
        let mut var_qi_qs_dn17: f64 = *var_qi_qs_dn17_slot;
        let mut var_qi_qs_dn2: f64 = *var_qi_qs_dn2_slot;
        let mut var_qi_qs_dn6: f64 = *var_qi_qs_dn6_slot;
        let mut var_qi_qs_dn7: f64 = *var_qi_qs_dn7_slot;
        let mut var_qse: f64 = *var_qse_slot;
        let mut var_qse_dn0: f64 = *var_qse_dn0_slot;
        let mut var_qse_dn10: f64 = *var_qse_dn10_slot;
        let mut var_qse_dn11: f64 = *var_qse_dn11_slot;
        let mut var_qse_dn12: f64 = *var_qse_dn12_slot;
        let mut var_qse_dn13: f64 = *var_qse_dn13_slot;
        let mut var_qse_dn15: f64 = *var_qse_dn15_slot;
        let mut var_qse_dn16: f64 = *var_qse_dn16_slot;
        let mut var_qse_dn17: f64 = *var_qse_dn17_slot;
        let mut var_qse_dn18: f64 = *var_qse_dn18_slot;
        let mut var_qse_dn2: f64 = *var_qse_dn2_slot;
        let mut var_qse_dn6: f64 = *var_qse_dn6_slot;
        let mut var_qse_dn7: f64 = *var_qse_dn7_slot;
        let mut var_xd: f64 = *var_xd_slot;
        let mut var_xd_dn0: f64 = *var_xd_dn0_slot;
        let mut var_xd_dn10: f64 = *var_xd_dn10_slot;
        let mut var_xd_dn11: f64 = *var_xd_dn11_slot;
        let mut var_xd_dn12: f64 = *var_xd_dn12_slot;
        let mut var_xd_dn17: f64 = *var_xd_dn17_slot;
        let mut var_xd_dn2: f64 = *var_xd_dn2_slot;
        let mut var_xd_dn6: f64 = *var_xd_dn6_slot;
        let mut var_xd_dn7: f64 = *var_xd_dn7_slot;

        let (assign33950_e49118, assign33950_e49118_d_n0, assign33950_e49118_d_n2, assign33950_e49118_d_n6, assign33950_e49118_d_n7, assign33950_e49118_d_n10, assign33950_e49118_d_n11, assign33950_e49118_d_n12, assign33950_e49118_d_n17,) = {
    if (var_guard1126 != 0.0) {
        let assign33950_e49112: f64 = (-var_cgbe);
        let assign33950_e49115: f64 = (var_vgs - var_vbsp);
        let assign33950_e49116: f64 = (assign33950_e49112 * assign33950_e49115);
        (assign33950_e49116, (assign33950_e49112 * (-var_vbsp_dn0)), (assign33950_e49112 * (-var_vbsp_dn2)), (assign33950_e49112 * (var_vgs_dn6 - var_vbsp_dn6)), (assign33950_e49112 * (var_vgs_dn7 - var_vbsp_dn7)), (assign33950_e49112 * (-var_vbsp_dn10)), (assign33950_e49112 * (var_vgs_dn11 - var_vbsp_dn11)), (assign33950_e49112 * (-var_vbsp_dn12)), (assign33950_e49112 * (-var_vbsp_dn17)),)
    } else {
        (var_qgob, var_qgob_dn0, var_qgob_dn2, var_qgob_dn6, var_qgob_dn7, var_qgob_dn10, var_qgob_dn11, var_qgob_dn12, var_qgob_dn17,)
    }
};
        var_qgob = assign33950_e49118;
        var_qgob_dn0 = assign33950_e49118_d_n0;
        var_qgob_dn2 = assign33950_e49118_d_n2;
        var_qgob_dn6 = assign33950_e49118_d_n6;
        var_qgob_dn7 = assign33950_e49118_d_n7;
        var_qgob_dn10 = assign33950_e49118_d_n10;
        var_qgob_dn11 = assign33950_e49118_d_n11;
        var_qgob_dn12 = assign33950_e49118_d_n12;
        var_qgob_dn17 = assign33950_e49118_d_n17;

        let (assign33960_e49128,) = {
    if (var_guard1126 != 0.0) {
        (0.0,)
    } else {
        (var_cfu,)
    }
};
        var_cfu = assign33960_e49128;

        let (assign33970_e49138,) = {
    if (var_guard1126 != 0.0) {
        let assign33970_e49132: f64 = (var_cfu * p.p9);
        let assign33970_e49135: f64 = (var_wgate + var_uc_pdbcp);
        let assign33970_e49136: f64 = (assign33970_e49132 * assign33970_e49135);
        (assign33970_e49136,)
    } else {
        (var_cfd,)
    }
};
        var_cfd = assign33970_e49138;

        let (assign33980_e49148,) = {
    if (var_guard1126 != 0.0) {
        let assign33980_e49142: f64 = (var_cfu * p.p9);
        let assign33980_e49145: f64 = (var_wgate + var_uc_psbcp);
        let assign33980_e49146: f64 = (assign33980_e49142 * assign33980_e49145);
        (assign33980_e49146,)
    } else {
        (var_cfs,)
    }
};
        var_cfs = assign33980_e49148;

        let (assign33990_e49156, assign33990_e49156_d_n0, assign33990_e49156_d_n2, assign33990_e49156_d_n6, assign33990_e49156_d_n7, assign33990_e49156_d_n10, assign33990_e49156_d_n11, assign33990_e49156_d_n12, assign33990_e49156_d_n17,) = {
    if (var_guard1126 != 0.0) {
        let assign33990_e49153: f64 = (var_vgs - var_vds);
        let assign33990_e49154: f64 = (var_cfd * assign33990_e49153);
        (assign33990_e49154, (var_cfd * (-var_vds_dn0)), (var_cfd * (-var_vds_dn2)), (var_cfd * (var_vgs_dn6 - var_vds_dn6)), (var_cfd * (var_vgs_dn7 - var_vds_dn7)), (var_cfd * (-var_vds_dn10)), (var_cfd * (var_vgs_dn11 - var_vds_dn11)), (var_cfd * (-var_vds_dn12)), (var_cfd * (-var_vds_dn17)),)
    } else {
        (var_qfd, var_qfd_dn0, var_qfd_dn2, var_qfd_dn6, var_qfd_dn7, var_qfd_dn10, var_qfd_dn11, var_qfd_dn12, var_qfd_dn17,)
    }
};
        var_qfd = assign33990_e49156;
        var_qfd_dn0 = assign33990_e49156_d_n0;
        var_qfd_dn2 = assign33990_e49156_d_n2;
        var_qfd_dn6 = assign33990_e49156_d_n6;
        var_qfd_dn7 = assign33990_e49156_d_n7;
        var_qfd_dn10 = assign33990_e49156_d_n10;
        var_qfd_dn11 = assign33990_e49156_d_n11;
        var_qfd_dn12 = assign33990_e49156_d_n12;
        var_qfd_dn17 = assign33990_e49156_d_n17;

        let (assign34000_e49162, assign34000_e49162_d_n6, assign34000_e49162_d_n7, assign34000_e49162_d_n11,) = {
    if (var_guard1126 != 0.0) {
        let assign34000_e49160: f64 = (var_cfs * var_vgs);
        (assign34000_e49160, (var_cfs * var_vgs_dn6), (var_cfs * var_vgs_dn7), (var_cfs * var_vgs_dn11),)
    } else {
        (var_qfs, var_qfs_dn6, var_qfs_dn7, var_qfs_dn11,)
    }
};
        var_qfs = assign34000_e49162;
        var_qfs_dn6 = assign34000_e49162_d_n6;
        var_qfs_dn7 = assign34000_e49162_d_n7;
        var_qfs_dn11 = assign34000_e49162_d_n11;

        let (assign34010_e49174, assign34010_e49174_d_n0, assign34010_e49174_d_n2, assign34010_e49174_d_n6, assign34010_e49174_d_n7, assign34010_e49174_d_n10, assign34010_e49174_d_n11, assign34010_e49174_d_n12, assign34010_e49174_d_n17,) = {
    if (var_guard1126 != 0.0) {
        let assign34010_e49166: f64 = (var_cfu * p.p19);
        let assign34010_e49168: f64 = (assign34010_e49166 * p.p9);
        let assign34010_e49171: f64 = (var_vgs - var_vbsp);
        let assign34010_e49172: f64 = (assign34010_e49168 * assign34010_e49171);
        (assign34010_e49172, (assign34010_e49168 * (-var_vbsp_dn0)), (assign34010_e49168 * (-var_vbsp_dn2)), (assign34010_e49168 * (var_vgs_dn6 - var_vbsp_dn6)), (assign34010_e49168 * (var_vgs_dn7 - var_vbsp_dn7)), (assign34010_e49168 * (-var_vbsp_dn10)), (assign34010_e49168 * (var_vgs_dn11 - var_vbsp_dn11)), (assign34010_e49168 * (-var_vbsp_dn12)), (assign34010_e49168 * (-var_vbsp_dn17)),)
    } else {
        (var_qfbc, var_qfbc_dn0, var_qfbc_dn2, var_qfbc_dn6, var_qfbc_dn7, var_qfbc_dn10, var_qfbc_dn11, var_qfbc_dn12, var_qfbc_dn17,)
    }
};
        var_qfbc = assign34010_e49174;
        var_qfbc_dn0 = assign34010_e49174_d_n0;
        var_qfbc_dn2 = assign34010_e49174_d_n2;
        var_qfbc_dn6 = assign34010_e49174_d_n6;
        var_qfbc_dn7 = assign34010_e49174_d_n7;
        var_qfbc_dn10 = assign34010_e49174_d_n10;
        var_qfbc_dn11 = assign34010_e49174_d_n11;
        var_qfbc_dn12 = assign34010_e49174_d_n12;
        var_qfbc_dn17 = assign34010_e49174_d_n17;

        let (assign34020_e49180, assign34020_e49180_d_n0, assign34020_e49180_d_n2, assign34020_e49180_d_n6, assign34020_e49180_d_n7, assign34020_e49180_d_n10, assign34020_e49180_d_n11, assign34020_e49180_d_n12, assign34020_e49180_d_n17,) = {
    if (var_guard1126 != 0.0) {
        let assign34020_e49178: f64 = (var_qgod + var_qfd);
        (assign34020_e49178, (var_qgod_dn0 + var_qfd_dn0), (var_qgod_dn2 + var_qfd_dn2), (var_qgod_dn6 + var_qfd_dn6), (var_qgod_dn7 + var_qfd_dn7), (var_qgod_dn10 + var_qfd_dn10), (var_qgod_dn11 + var_qfd_dn11), (var_qgod_dn12 + var_qfd_dn12), (var_qgod_dn17 + var_qfd_dn17),)
    } else {
        (var_qgod, var_qgod_dn0, var_qgod_dn2, var_qgod_dn6, var_qgod_dn7, var_qgod_dn10, var_qgod_dn11, var_qgod_dn12, var_qgod_dn17,)
    }
};
        var_qgod = assign34020_e49180;
        var_qgod_dn0 = assign34020_e49180_d_n0;
        var_qgod_dn2 = assign34020_e49180_d_n2;
        var_qgod_dn6 = assign34020_e49180_d_n6;
        var_qgod_dn7 = assign34020_e49180_d_n7;
        var_qgod_dn10 = assign34020_e49180_d_n10;
        var_qgod_dn11 = assign34020_e49180_d_n11;
        var_qgod_dn12 = assign34020_e49180_d_n12;
        var_qgod_dn17 = assign34020_e49180_d_n17;

        let (assign34030_e49186, assign34030_e49186_d_n0, assign34030_e49186_d_n2, assign34030_e49186_d_n6, assign34030_e49186_d_n7, assign34030_e49186_d_n10, assign34030_e49186_d_n11, assign34030_e49186_d_n12, assign34030_e49186_d_n17,) = {
    if (var_guard1126 != 0.0) {
        let assign34030_e49184: f64 = (var_qgos + var_qfs);
        (assign34030_e49184, var_qgos_dn0, var_qgos_dn2, (var_qgos_dn6 + var_qfs_dn6), (var_qgos_dn7 + var_qfs_dn7), var_qgos_dn10, (var_qgos_dn11 + var_qfs_dn11), var_qgos_dn12, var_qgos_dn17,)
    } else {
        (var_qgos, var_qgos_dn0, var_qgos_dn2, var_qgos_dn6, var_qgos_dn7, var_qgos_dn10, var_qgos_dn11, var_qgos_dn12, var_qgos_dn17,)
    }
};
        var_qgos = assign34030_e49186;
        var_qgos_dn0 = assign34030_e49186_d_n0;
        var_qgos_dn2 = assign34030_e49186_d_n2;
        var_qgos_dn6 = assign34030_e49186_d_n6;
        var_qgos_dn7 = assign34030_e49186_d_n7;
        var_qgos_dn10 = assign34030_e49186_d_n10;
        var_qgos_dn11 = assign34030_e49186_d_n11;
        var_qgos_dn12 = assign34030_e49186_d_n12;
        var_qgos_dn17 = assign34030_e49186_d_n17;

        let (assign34040_e49192, assign34040_e49192_d_n0, assign34040_e49192_d_n2, assign34040_e49192_d_n6, assign34040_e49192_d_n7, assign34040_e49192_d_n10, assign34040_e49192_d_n11, assign34040_e49192_d_n12, assign34040_e49192_d_n17,) = {
    if (var_guard1126 != 0.0) {
        let assign34040_e49190: f64 = (var_qgob + var_qfbc);
        (assign34040_e49190, (var_qgob_dn0 + var_qfbc_dn0), (var_qgob_dn2 + var_qfbc_dn2), (var_qgob_dn6 + var_qfbc_dn6), (var_qgob_dn7 + var_qfbc_dn7), (var_qgob_dn10 + var_qfbc_dn10), (var_qgob_dn11 + var_qfbc_dn11), (var_qgob_dn12 + var_qfbc_dn12), (var_qgob_dn17 + var_qfbc_dn17),)
    } else {
        (var_qgob, var_qgob_dn0, var_qgob_dn2, var_qgob_dn6, var_qgob_dn7, var_qgob_dn10, var_qgob_dn11, var_qgob_dn12, var_qgob_dn17,)
    }
};
        var_qgob = assign34040_e49192;
        var_qgob_dn0 = assign34040_e49192_d_n0;
        var_qgob_dn2 = assign34040_e49192_d_n2;
        var_qgob_dn6 = assign34040_e49192_d_n6;
        var_qgob_dn7 = assign34040_e49192_d_n7;
        var_qgob_dn10 = assign34040_e49192_d_n10;
        var_qgob_dn11 = assign34040_e49192_d_n11;
        var_qgob_dn12 = assign34040_e49192_d_n12;
        var_qgob_dn17 = assign34040_e49192_d_n17;

        let (assign34050_e49202,) = {
    if ((var_guard1126 == 0.0) && (var_cgbo_given != 0.0)) {
        let assign34050_e49198: f64 = (-p.p168);
        let assign34050_e49200: f64 = (assign34050_e49198 * var_lgleff);
        (assign34050_e49200,)
    } else {
        (var_cgbe,)
    }
};
        var_cgbe = assign34050_e49202;

        let (assign34060_e49214, assign34060_e49214_d_n0, assign34060_e49214_d_n2, assign34060_e49214_d_n6, assign34060_e49214_d_n7, assign34060_e49214_d_n10, assign34060_e49214_d_n11, assign34060_e49214_d_n12, assign34060_e49214_d_n17,) = {
    if ((var_guard1126 == 0.0) && (var_cgbo_given != 0.0)) {
        let assign34060_e49208: f64 = (-var_cgbe);
        let assign34060_e49211: f64 = (var_vgs - var_vbsp);
        let assign34060_e49212: f64 = (assign34060_e49208 * assign34060_e49211);
        (assign34060_e49212, (assign34060_e49208 * (-var_vbsp_dn0)), (assign34060_e49208 * (-var_vbsp_dn2)), (assign34060_e49208 * (var_vgs_dn6 - var_vbsp_dn6)), (assign34060_e49208 * (var_vgs_dn7 - var_vbsp_dn7)), (assign34060_e49208 * (-var_vbsp_dn10)), (assign34060_e49208 * (var_vgs_dn11 - var_vbsp_dn11)), (assign34060_e49208 * (-var_vbsp_dn12)), (assign34060_e49208 * (-var_vbsp_dn17)),)
    } else {
        (var_qgob, var_qgob_dn0, var_qgob_dn2, var_qgob_dn6, var_qgob_dn7, var_qgob_dn10, var_qgob_dn11, var_qgob_dn12, var_qgob_dn17,)
    }
};
        var_qgob = assign34060_e49214;
        var_qgob_dn0 = assign34060_e49214_d_n0;
        var_qgob_dn2 = assign34060_e49214_d_n2;
        var_qgob_dn6 = assign34060_e49214_d_n6;
        var_qgob_dn7 = assign34060_e49214_d_n7;
        var_qgob_dn10 = assign34060_e49214_d_n10;
        var_qgob_dn11 = assign34060_e49214_d_n11;
        var_qgob_dn12 = assign34060_e49214_d_n12;
        var_qgob_dn17 = assign34060_e49214_d_n17;

        let (assign34070_e49222,) = {
    if ((var_guard1126 == 0.0) && (var_cgbo_given == 0.0)) {
        (0.0,)
    } else {
        (var_cgbe,)
    }
};
        var_cgbe = assign34070_e49222;

        let (assign34080_e49230, assign34080_e49230_d_n0, assign34080_e49230_d_n2, assign34080_e49230_d_n6, assign34080_e49230_d_n7, assign34080_e49230_d_n10, assign34080_e49230_d_n11, assign34080_e49230_d_n12, assign34080_e49230_d_n17,) = {
    if ((var_guard1126 == 0.0) && (var_cgbo_given == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qgob, var_qgob_dn0, var_qgob_dn2, var_qgob_dn6, var_qgob_dn7, var_qgob_dn10, var_qgob_dn11, var_qgob_dn12, var_qgob_dn17,)
    }
};
        var_qgob = assign34080_e49230;
        var_qgob_dn0 = assign34080_e49230_d_n0;
        var_qgob_dn2 = assign34080_e49230_d_n2;
        var_qgob_dn6 = assign34080_e49230_d_n6;
        var_qgob_dn7 = assign34080_e49230_d_n7;
        var_qgob_dn10 = assign34080_e49230_d_n10;
        var_qgob_dn11 = assign34080_e49230_d_n11;
        var_qgob_dn12 = assign34080_e49230_d_n12;
        var_qgob_dn17 = assign34080_e49230_d_n17;

        let (assign34090_e49245,) = {
    if (var_guard1126 == 0.0) {
        (0.0,)
    } else {
        (var_cf,)
    }
};
        var_cf = assign34090_e49245;

        let (assign34100_e49250,) = {
    if (var_guard1126 == 0.0) {
        (var_cf,)
    } else {
        (var_cfd,)
    }
};
        var_cfd = assign34100_e49250;

        let (assign34110_e49255,) = {
    if (var_guard1126 == 0.0) {
        (var_cf,)
    } else {
        (var_cfs,)
    }
};
        var_cfs = assign34110_e49255;

        let (assign34120_e49264, assign34120_e49264_d_n0, assign34120_e49264_d_n2, assign34120_e49264_d_n6, assign34120_e49264_d_n7, assign34120_e49264_d_n10, assign34120_e49264_d_n11, assign34120_e49264_d_n12, assign34120_e49264_d_n17,) = {
    if (var_guard1126 == 0.0) {
        let assign34120_e49261: f64 = (var_vgs - var_vds);
        let assign34120_e49262: f64 = (var_cfd * assign34120_e49261);
        (assign34120_e49262, (var_cfd * (-var_vds_dn0)), (var_cfd * (-var_vds_dn2)), (var_cfd * (var_vgs_dn6 - var_vds_dn6)), (var_cfd * (var_vgs_dn7 - var_vds_dn7)), (var_cfd * (-var_vds_dn10)), (var_cfd * (var_vgs_dn11 - var_vds_dn11)), (var_cfd * (-var_vds_dn12)), (var_cfd * (-var_vds_dn17)),)
    } else {
        (var_qfd, var_qfd_dn0, var_qfd_dn2, var_qfd_dn6, var_qfd_dn7, var_qfd_dn10, var_qfd_dn11, var_qfd_dn12, var_qfd_dn17,)
    }
};
        var_qfd = assign34120_e49264;
        var_qfd_dn0 = assign34120_e49264_d_n0;
        var_qfd_dn2 = assign34120_e49264_d_n2;
        var_qfd_dn6 = assign34120_e49264_d_n6;
        var_qfd_dn7 = assign34120_e49264_d_n7;
        var_qfd_dn10 = assign34120_e49264_d_n10;
        var_qfd_dn11 = assign34120_e49264_d_n11;
        var_qfd_dn12 = assign34120_e49264_d_n12;
        var_qfd_dn17 = assign34120_e49264_d_n17;

        let (assign34130_e49271, assign34130_e49271_d_n6, assign34130_e49271_d_n7, assign34130_e49271_d_n11,) = {
    if (var_guard1126 == 0.0) {
        let assign34130_e49269: f64 = (var_cfs * var_vgs);
        (assign34130_e49269, (var_cfs * var_vgs_dn6), (var_cfs * var_vgs_dn7), (var_cfs * var_vgs_dn11),)
    } else {
        (var_qfs, var_qfs_dn6, var_qfs_dn7, var_qfs_dn11,)
    }
};
        var_qfs = assign34130_e49271;
        var_qfs_dn6 = assign34130_e49271_d_n6;
        var_qfs_dn7 = assign34130_e49271_d_n7;
        var_qfs_dn11 = assign34130_e49271_d_n11;

        let (assign34140_e49278, assign34140_e49278_d_n0, assign34140_e49278_d_n2, assign34140_e49278_d_n6, assign34140_e49278_d_n7, assign34140_e49278_d_n10, assign34140_e49278_d_n11, assign34140_e49278_d_n12, assign34140_e49278_d_n17,) = {
    if (var_guard1126 == 0.0) {
        let assign34140_e49276: f64 = (var_qgod + var_qfd);
        (assign34140_e49276, (var_qgod_dn0 + var_qfd_dn0), (var_qgod_dn2 + var_qfd_dn2), (var_qgod_dn6 + var_qfd_dn6), (var_qgod_dn7 + var_qfd_dn7), (var_qgod_dn10 + var_qfd_dn10), (var_qgod_dn11 + var_qfd_dn11), (var_qgod_dn12 + var_qfd_dn12), (var_qgod_dn17 + var_qfd_dn17),)
    } else {
        (var_qgod, var_qgod_dn0, var_qgod_dn2, var_qgod_dn6, var_qgod_dn7, var_qgod_dn10, var_qgod_dn11, var_qgod_dn12, var_qgod_dn17,)
    }
};
        var_qgod = assign34140_e49278;
        var_qgod_dn0 = assign34140_e49278_d_n0;
        var_qgod_dn2 = assign34140_e49278_d_n2;
        var_qgod_dn6 = assign34140_e49278_d_n6;
        var_qgod_dn7 = assign34140_e49278_d_n7;
        var_qgod_dn10 = assign34140_e49278_d_n10;
        var_qgod_dn11 = assign34140_e49278_d_n11;
        var_qgod_dn12 = assign34140_e49278_d_n12;
        var_qgod_dn17 = assign34140_e49278_d_n17;

        let (assign34150_e49285, assign34150_e49285_d_n0, assign34150_e49285_d_n2, assign34150_e49285_d_n6, assign34150_e49285_d_n7, assign34150_e49285_d_n10, assign34150_e49285_d_n11, assign34150_e49285_d_n12, assign34150_e49285_d_n17,) = {
    if (var_guard1126 == 0.0) {
        let assign34150_e49283: f64 = (var_qgos + var_qfs);
        (assign34150_e49283, var_qgos_dn0, var_qgos_dn2, (var_qgos_dn6 + var_qfs_dn6), (var_qgos_dn7 + var_qfs_dn7), var_qgos_dn10, (var_qgos_dn11 + var_qfs_dn11), var_qgos_dn12, var_qgos_dn17,)
    } else {
        (var_qgos, var_qgos_dn0, var_qgos_dn2, var_qgos_dn6, var_qgos_dn7, var_qgos_dn10, var_qgos_dn11, var_qgos_dn12, var_qgos_dn17,)
    }
};
        var_qgos = assign34150_e49285;
        var_qgos_dn0 = assign34150_e49285_d_n0;
        var_qgos_dn2 = assign34150_e49285_d_n2;
        var_qgos_dn6 = assign34150_e49285_d_n6;
        var_qgos_dn7 = assign34150_e49285_d_n7;
        var_qgos_dn10 = assign34150_e49285_d_n10;
        var_qgos_dn11 = assign34150_e49285_d_n11;
        var_qgos_dn12 = assign34150_e49285_d_n12;
        var_qgos_dn17 = assign34150_e49285_d_n17;

        let assign34160_e49288: f64 = (var_mfactor * var_ids);
        var_idse = assign34160_e49288;
        var_idse_dn0 = (var_mfactor * var_ids_dn0);
        var_idse_dn2 = (var_mfactor * var_ids_dn2);
        var_idse_dn6 = (var_mfactor * var_ids_dn6);
        var_idse_dn7 = (var_mfactor * var_ids_dn7);
        var_idse_dn10 = (var_mfactor * var_ids_dn10);
        var_idse_dn11 = (var_mfactor * var_ids_dn11);
        var_idse_dn12 = (var_mfactor * var_ids_dn12);
        var_idse_dn17 = (var_mfactor * var_ids_dn17);

        let (assign34170_e49292, assign34170_e49292_d_n0, assign34170_e49292_d_n2, assign34170_e49292_d_n6, assign34170_e49292_d_n7, assign34170_e49292_d_n10, assign34170_e49292_d_n11, assign34170_e49292_d_n12, assign34170_e49292_d_n13, assign34170_e49292_d_n15, assign34170_e49292_d_n16, assign34170_e49292_d_n17, assign34170_e49292_d_n18,) = {
    if (var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn6, var_qde_dn7, var_qde_dn10, var_qde_dn11, var_qde_dn12, var_qde_dn13, var_qde_dn15, var_qde_dn16, var_qde_dn17, var_qde_dn18,)
    }
};
        var_qde = assign34170_e49292;
        var_qde_dn0 = assign34170_e49292_d_n0;
        var_qde_dn2 = assign34170_e49292_d_n2;
        var_qde_dn6 = assign34170_e49292_d_n6;
        var_qde_dn7 = assign34170_e49292_d_n7;
        var_qde_dn10 = assign34170_e49292_d_n10;
        var_qde_dn11 = assign34170_e49292_d_n11;
        var_qde_dn12 = assign34170_e49292_d_n12;
        var_qde_dn13 = assign34170_e49292_d_n13;
        var_qde_dn15 = assign34170_e49292_d_n15;
        var_qde_dn16 = assign34170_e49292_d_n16;
        var_qde_dn17 = assign34170_e49292_d_n17;
        var_qde_dn18 = assign34170_e49292_d_n18;

        let (assign34180_e49296, assign34180_e49296_d_n0, assign34180_e49296_d_n2, assign34180_e49296_d_n6, assign34180_e49296_d_n7, assign34180_e49296_d_n10, assign34180_e49296_d_n11, assign34180_e49296_d_n12, assign34180_e49296_d_n13, assign34180_e49296_d_n15, assign34180_e49296_d_n16, assign34180_e49296_d_n17, assign34180_e49296_d_n18,) = {
    if (var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn6, var_qge_dn7, var_qge_dn10, var_qge_dn11, var_qge_dn12, var_qge_dn13, var_qge_dn15, var_qge_dn16, var_qge_dn17, var_qge_dn18,)
    }
};
        var_qge = assign34180_e49296;
        var_qge_dn0 = assign34180_e49296_d_n0;
        var_qge_dn2 = assign34180_e49296_d_n2;
        var_qge_dn6 = assign34180_e49296_d_n6;
        var_qge_dn7 = assign34180_e49296_d_n7;
        var_qge_dn10 = assign34180_e49296_d_n10;
        var_qge_dn11 = assign34180_e49296_d_n11;
        var_qge_dn12 = assign34180_e49296_d_n12;
        var_qge_dn13 = assign34180_e49296_d_n13;
        var_qge_dn15 = assign34180_e49296_d_n15;
        var_qge_dn16 = assign34180_e49296_d_n16;
        var_qge_dn17 = assign34180_e49296_d_n17;
        var_qge_dn18 = assign34180_e49296_d_n18;

        let assign34190_e49299: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1127 = assign34190_e49299;

        let (assign34200_e49305, assign34200_e49305_d_n0, assign34200_e49305_d_n2, assign34200_e49305_d_n6, assign34200_e49305_d_n7, assign34200_e49305_d_n10, assign34200_e49305_d_n11, assign34200_e49305_d_n12, assign34200_e49305_d_n13, assign34200_e49305_d_n15, assign34200_e49305_d_n16, assign34200_e49305_d_n17, assign34200_e49305_d_n18,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1127 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn6, var_qse_dn7, var_qse_dn10, var_qse_dn11, var_qse_dn12, var_qse_dn13, var_qse_dn15, var_qse_dn16, var_qse_dn17, var_qse_dn18,)
    }
};
        var_qse = assign34200_e49305;
        var_qse_dn0 = assign34200_e49305_d_n0;
        var_qse_dn2 = assign34200_e49305_d_n2;
        var_qse_dn6 = assign34200_e49305_d_n6;
        var_qse_dn7 = assign34200_e49305_d_n7;
        var_qse_dn10 = assign34200_e49305_d_n10;
        var_qse_dn11 = assign34200_e49305_d_n11;
        var_qse_dn12 = assign34200_e49305_d_n12;
        var_qse_dn13 = assign34200_e49305_d_n13;
        var_qse_dn15 = assign34200_e49305_d_n15;
        var_qse_dn16 = assign34200_e49305_d_n16;
        var_qse_dn17 = assign34200_e49305_d_n17;
        var_qse_dn18 = assign34200_e49305_d_n18;

        let (assign34210_e49311, assign34210_e49311_d_n0, assign34210_e49311_d_n2, assign34210_e49311_d_n6, assign34210_e49311_d_n7, assign34210_e49311_d_n10, assign34210_e49311_d_n11, assign34210_e49311_d_n12, assign34210_e49311_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1127 != 0.0)) {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn10, var_qdrat_dn11, var_qdrat_dn12, var_qdrat_dn17,)
    } else {
        (var_xd, var_xd_dn0, var_xd_dn2, var_xd_dn6, var_xd_dn7, var_xd_dn10, var_xd_dn11, var_xd_dn12, var_xd_dn17,)
    }
};
        var_xd = assign34210_e49311;
        var_xd_dn0 = assign34210_e49311_d_n0;
        var_xd_dn2 = assign34210_e49311_d_n2;
        var_xd_dn6 = assign34210_e49311_d_n6;
        var_xd_dn7 = assign34210_e49311_d_n7;
        var_xd_dn10 = assign34210_e49311_d_n10;
        var_xd_dn11 = assign34210_e49311_d_n11;
        var_xd_dn12 = assign34210_e49311_d_n12;
        var_xd_dn17 = assign34210_e49311_d_n17;

        let (assign34220_e49319, assign34220_e49319_d_n0, assign34220_e49319_d_n2, assign34220_e49319_d_n6, assign34220_e49319_d_n7, assign34220_e49319_d_n10, assign34220_e49319_d_n11, assign34220_e49319_d_n12, assign34220_e49319_d_n13, assign34220_e49319_d_n15, assign34220_e49319_d_n16, assign34220_e49319_d_n17, assign34220_e49319_d_n18,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1127 != 0.0)) {
        let assign34220_e49317: f64 = (var_mfactor * var_qb);
        (assign34220_e49317, (var_mfactor * var_qb_dn0), (var_mfactor * var_qb_dn2), (var_mfactor * var_qb_dn6), (var_mfactor * var_qb_dn7), (var_mfactor * var_qb_dn10), (var_mfactor * var_qb_dn11), (var_mfactor * var_qb_dn12), (var_mfactor * var_qb_dn13), (var_mfactor * var_qb_dn15), (var_mfactor * var_qb_dn16), (var_mfactor * var_qb_dn17), (var_mfactor * var_qb_dn18),)
    } else {
        (var_qb_qs, var_qb_qs_dn0, var_qb_qs_dn2, var_qb_qs_dn6, var_qb_qs_dn7, var_qb_qs_dn10, var_qb_qs_dn11, var_qb_qs_dn12, var_qb_qs_dn13, var_qb_qs_dn15, var_qb_qs_dn16, var_qb_qs_dn17, var_qb_qs_dn18,)
    }
};
        var_qb_qs = assign34220_e49319;
        var_qb_qs_dn0 = assign34220_e49319_d_n0;
        var_qb_qs_dn2 = assign34220_e49319_d_n2;
        var_qb_qs_dn6 = assign34220_e49319_d_n6;
        var_qb_qs_dn7 = assign34220_e49319_d_n7;
        var_qb_qs_dn10 = assign34220_e49319_d_n10;
        var_qb_qs_dn11 = assign34220_e49319_d_n11;
        var_qb_qs_dn12 = assign34220_e49319_d_n12;
        var_qb_qs_dn13 = assign34220_e49319_d_n13;
        var_qb_qs_dn15 = assign34220_e49319_d_n15;
        var_qb_qs_dn16 = assign34220_e49319_d_n16;
        var_qb_qs_dn17 = assign34220_e49319_d_n17;
        var_qb_qs_dn18 = assign34220_e49319_d_n18;

        let (assign34230_e49327, assign34230_e49327_d_n0, assign34230_e49327_d_n2, assign34230_e49327_d_n6, assign34230_e49327_d_n7, assign34230_e49327_d_n10, assign34230_e49327_d_n11, assign34230_e49327_d_n12, assign34230_e49327_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1127 != 0.0)) {
        let assign34230_e49325: f64 = (var_mfactor * var_qi);
        (assign34230_e49325, (var_mfactor * var_qi_dn0), (var_mfactor * var_qi_dn2), (var_mfactor * var_qi_dn6), (var_mfactor * var_qi_dn7), (var_mfactor * var_qi_dn10), (var_mfactor * var_qi_dn11), (var_mfactor * var_qi_dn12), (var_mfactor * var_qi_dn17),)
    } else {
        (var_qi_qs, var_qi_qs_dn0, var_qi_qs_dn2, var_qi_qs_dn6, var_qi_qs_dn7, var_qi_qs_dn10, var_qi_qs_dn11, var_qi_qs_dn12, var_qi_qs_dn17,)
    }
};
        var_qi_qs = assign34230_e49327;
        var_qi_qs_dn0 = assign34230_e49327_d_n0;
        var_qi_qs_dn2 = assign34230_e49327_d_n2;
        var_qi_qs_dn6 = assign34230_e49327_d_n6;
        var_qi_qs_dn7 = assign34230_e49327_d_n7;
        var_qi_qs_dn10 = assign34230_e49327_d_n10;
        var_qi_qs_dn11 = assign34230_e49327_d_n11;
        var_qi_qs_dn12 = assign34230_e49327_d_n12;
        var_qi_qs_dn17 = assign34230_e49327_d_n17;

        let (assign34240_e49334, assign34240_e49334_d_n0, assign34240_e49334_d_n2, assign34240_e49334_d_n6, assign34240_e49334_d_n7, assign34240_e49334_d_n10, assign34240_e49334_d_n11, assign34240_e49334_d_n12, assign34240_e49334_d_n13, assign34240_e49334_d_n15, assign34240_e49334_d_n16, assign34240_e49334_d_n17, assign34240_e49334_d_n18,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1127 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_dn13, var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    }
};
        var_qbe = assign34240_e49334;
        var_qbe_dn0 = assign34240_e49334_d_n0;
        var_qbe_dn2 = assign34240_e49334_d_n2;
        var_qbe_dn6 = assign34240_e49334_d_n6;
        var_qbe_dn7 = assign34240_e49334_d_n7;
        var_qbe_dn10 = assign34240_e49334_d_n10;
        var_qbe_dn11 = assign34240_e49334_d_n11;
        var_qbe_dn12 = assign34240_e49334_d_n12;
        var_qbe_dn13 = assign34240_e49334_d_n13;
        var_qbe_dn15 = assign34240_e49334_d_n15;
        var_qbe_dn16 = assign34240_e49334_d_n16;
        var_qbe_dn17 = assign34240_e49334_d_n17;
        var_qbe_dn18 = assign34240_e49334_d_n18;

        let (assign34250_e49343, assign34250_e49343_d_n0, assign34250_e49343_d_n2, assign34250_e49343_d_n6, assign34250_e49343_d_n7, assign34250_e49343_d_n10, assign34250_e49343_d_n11, assign34250_e49343_d_n12, assign34250_e49343_d_n13, assign34250_e49343_d_n15, assign34250_e49343_d_n16, assign34250_e49343_d_n17, assign34250_e49343_d_n18,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1127 == 0.0)) {
        let assign34250_e49341: f64 = (var_mfactor * var_qsub);
        (assign34250_e49341, (var_mfactor * var_qsub_dn0), (var_mfactor * var_qsub_dn2), (var_mfactor * var_qsub_dn6), (var_mfactor * var_qsub_dn7), (var_mfactor * var_qsub_dn10), (var_mfactor * var_qsub_dn11), (var_mfactor * var_qsub_dn12), 0.0, 0.0, 0.0, (var_mfactor * var_qsub_dn17), 0.0,)
    } else {
        (var_qb_qs, var_qb_qs_dn0, var_qb_qs_dn2, var_qb_qs_dn6, var_qb_qs_dn7, var_qb_qs_dn10, var_qb_qs_dn11, var_qb_qs_dn12, var_qb_qs_dn13, var_qb_qs_dn15, var_qb_qs_dn16, var_qb_qs_dn17, var_qb_qs_dn18,)
    }
};
        var_qb_qs = assign34250_e49343;
        var_qb_qs_dn0 = assign34250_e49343_d_n0;
        var_qb_qs_dn2 = assign34250_e49343_d_n2;
        var_qb_qs_dn6 = assign34250_e49343_d_n6;
        var_qb_qs_dn7 = assign34250_e49343_d_n7;
        var_qb_qs_dn10 = assign34250_e49343_d_n10;
        var_qb_qs_dn11 = assign34250_e49343_d_n11;
        var_qb_qs_dn12 = assign34250_e49343_d_n12;
        var_qb_qs_dn13 = assign34250_e49343_d_n13;
        var_qb_qs_dn15 = assign34250_e49343_d_n15;
        var_qb_qs_dn16 = assign34250_e49343_d_n16;
        var_qb_qs_dn17 = assign34250_e49343_d_n17;
        var_qb_qs_dn18 = assign34250_e49343_d_n18;

        let (assign34260_e49354, assign34260_e49354_d_n0, assign34260_e49354_d_n2, assign34260_e49354_d_n6, assign34260_e49354_d_n7, assign34260_e49354_d_n10, assign34260_e49354_d_n11, assign34260_e49354_d_n12, assign34260_e49354_d_n13, assign34260_e49354_d_n15, assign34260_e49354_d_n16, assign34260_e49354_d_n17, assign34260_e49354_d_n18,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1127 == 0.0)) {
        let assign34260_e49351: f64 = (var_qd + var_qd_fb);
        let assign34260_e49352: f64 = (var_mfactor * assign34260_e49351);
        (assign34260_e49352, (var_mfactor * (var_qd_dn0 + var_qd_fb_dn0)), (var_mfactor * (var_qd_dn2 + var_qd_fb_dn2)), (var_mfactor * (var_qd_dn6 + var_qd_fb_dn6)), (var_mfactor * (var_qd_dn7 + var_qd_fb_dn7)), (var_mfactor * (var_qd_dn10 + var_qd_fb_dn10)), (var_mfactor * (var_qd_dn11 + var_qd_fb_dn11)), (var_mfactor * (var_qd_dn12 + var_qd_fb_dn12)), (var_mfactor * (var_qd_dn13 + var_qd_fb_dn13)), (var_mfactor * (var_qd_dn15 + var_qd_fb_dn15)), (var_mfactor * (var_qd_dn16 + var_qd_fb_dn16)), (var_mfactor * (var_qd_dn17 + var_qd_fb_dn17)), (var_mfactor * (var_qd_dn18 + var_qd_fb_dn18)),)
    } else {
        (var_qd_qs, var_qd_qs_dn0, var_qd_qs_dn2, var_qd_qs_dn6, var_qd_qs_dn7, var_qd_qs_dn10, var_qd_qs_dn11, var_qd_qs_dn12, var_qd_qs_dn13, var_qd_qs_dn15, var_qd_qs_dn16, var_qd_qs_dn17, var_qd_qs_dn18,)
    }
};
        var_qd_qs = assign34260_e49354;
        var_qd_qs_dn0 = assign34260_e49354_d_n0;
        var_qd_qs_dn2 = assign34260_e49354_d_n2;
        var_qd_qs_dn6 = assign34260_e49354_d_n6;
        var_qd_qs_dn7 = assign34260_e49354_d_n7;
        var_qd_qs_dn10 = assign34260_e49354_d_n10;
        var_qd_qs_dn11 = assign34260_e49354_d_n11;
        var_qd_qs_dn12 = assign34260_e49354_d_n12;
        var_qd_qs_dn13 = assign34260_e49354_d_n13;
        var_qd_qs_dn15 = assign34260_e49354_d_n15;
        var_qd_qs_dn16 = assign34260_e49354_d_n16;
        var_qd_qs_dn17 = assign34260_e49354_d_n17;
        var_qd_qs_dn18 = assign34260_e49354_d_n18;

        *var_cf_slot = var_cf;
        *var_cfd_slot = var_cfd;
        *var_cfs_slot = var_cfs;
        *var_cfu_slot = var_cfu;
        *var_cgbe_slot = var_cgbe;
        *var_guard1127_slot = var_guard1127;
        *var_idse_slot = var_idse;
        *var_idse_dn0_slot = var_idse_dn0;
        *var_idse_dn10_slot = var_idse_dn10;
        *var_idse_dn11_slot = var_idse_dn11;
        *var_idse_dn12_slot = var_idse_dn12;
        *var_idse_dn17_slot = var_idse_dn17;
        *var_idse_dn2_slot = var_idse_dn2;
        *var_idse_dn6_slot = var_idse_dn6;
        *var_idse_dn7_slot = var_idse_dn7;
        *var_qb_qs_slot = var_qb_qs;
        *var_qb_qs_dn0_slot = var_qb_qs_dn0;
        *var_qb_qs_dn10_slot = var_qb_qs_dn10;
        *var_qb_qs_dn11_slot = var_qb_qs_dn11;
        *var_qb_qs_dn12_slot = var_qb_qs_dn12;
        *var_qb_qs_dn13_slot = var_qb_qs_dn13;
        *var_qb_qs_dn15_slot = var_qb_qs_dn15;
        *var_qb_qs_dn16_slot = var_qb_qs_dn16;
        *var_qb_qs_dn17_slot = var_qb_qs_dn17;
        *var_qb_qs_dn18_slot = var_qb_qs_dn18;
        *var_qb_qs_dn2_slot = var_qb_qs_dn2;
        *var_qb_qs_dn6_slot = var_qb_qs_dn6;
        *var_qb_qs_dn7_slot = var_qb_qs_dn7;
        *var_qbe_slot = var_qbe;
        *var_qbe_dn0_slot = var_qbe_dn0;
        *var_qbe_dn10_slot = var_qbe_dn10;
        *var_qbe_dn11_slot = var_qbe_dn11;
        *var_qbe_dn12_slot = var_qbe_dn12;
        *var_qbe_dn13_slot = var_qbe_dn13;
        *var_qbe_dn15_slot = var_qbe_dn15;
        *var_qbe_dn16_slot = var_qbe_dn16;
        *var_qbe_dn17_slot = var_qbe_dn17;
        *var_qbe_dn18_slot = var_qbe_dn18;
        *var_qbe_dn2_slot = var_qbe_dn2;
        *var_qbe_dn6_slot = var_qbe_dn6;
        *var_qbe_dn7_slot = var_qbe_dn7;
        *var_qd_qs_slot = var_qd_qs;
        *var_qd_qs_dn0_slot = var_qd_qs_dn0;
        *var_qd_qs_dn10_slot = var_qd_qs_dn10;
        *var_qd_qs_dn11_slot = var_qd_qs_dn11;
        *var_qd_qs_dn12_slot = var_qd_qs_dn12;
        *var_qd_qs_dn13_slot = var_qd_qs_dn13;
        *var_qd_qs_dn15_slot = var_qd_qs_dn15;
        *var_qd_qs_dn16_slot = var_qd_qs_dn16;
        *var_qd_qs_dn17_slot = var_qd_qs_dn17;
        *var_qd_qs_dn18_slot = var_qd_qs_dn18;
        *var_qd_qs_dn2_slot = var_qd_qs_dn2;
        *var_qd_qs_dn6_slot = var_qd_qs_dn6;
        *var_qd_qs_dn7_slot = var_qd_qs_dn7;
        *var_qde_slot = var_qde;
        *var_qde_dn0_slot = var_qde_dn0;
        *var_qde_dn10_slot = var_qde_dn10;
        *var_qde_dn11_slot = var_qde_dn11;
        *var_qde_dn12_slot = var_qde_dn12;
        *var_qde_dn13_slot = var_qde_dn13;
        *var_qde_dn15_slot = var_qde_dn15;
        *var_qde_dn16_slot = var_qde_dn16;
        *var_qde_dn17_slot = var_qde_dn17;
        *var_qde_dn18_slot = var_qde_dn18;
        *var_qde_dn2_slot = var_qde_dn2;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qde_dn7_slot = var_qde_dn7;
        *var_qfbc_slot = var_qfbc;
        *var_qfbc_dn0_slot = var_qfbc_dn0;
        *var_qfbc_dn10_slot = var_qfbc_dn10;
        *var_qfbc_dn11_slot = var_qfbc_dn11;
        *var_qfbc_dn12_slot = var_qfbc_dn12;
        *var_qfbc_dn17_slot = var_qfbc_dn17;
        *var_qfbc_dn2_slot = var_qfbc_dn2;
        *var_qfbc_dn6_slot = var_qfbc_dn6;
        *var_qfbc_dn7_slot = var_qfbc_dn7;
        *var_qfd_slot = var_qfd;
        *var_qfd_dn0_slot = var_qfd_dn0;
        *var_qfd_dn10_slot = var_qfd_dn10;
        *var_qfd_dn11_slot = var_qfd_dn11;
        *var_qfd_dn12_slot = var_qfd_dn12;
        *var_qfd_dn17_slot = var_qfd_dn17;
        *var_qfd_dn2_slot = var_qfd_dn2;
        *var_qfd_dn6_slot = var_qfd_dn6;
        *var_qfd_dn7_slot = var_qfd_dn7;
        *var_qfs_slot = var_qfs;
        *var_qfs_dn11_slot = var_qfs_dn11;
        *var_qfs_dn6_slot = var_qfs_dn6;
        *var_qfs_dn7_slot = var_qfs_dn7;
        *var_qge_slot = var_qge;
        *var_qge_dn0_slot = var_qge_dn0;
        *var_qge_dn10_slot = var_qge_dn10;
        *var_qge_dn11_slot = var_qge_dn11;
        *var_qge_dn12_slot = var_qge_dn12;
        *var_qge_dn13_slot = var_qge_dn13;
        *var_qge_dn15_slot = var_qge_dn15;
        *var_qge_dn16_slot = var_qge_dn16;
        *var_qge_dn17_slot = var_qge_dn17;
        *var_qge_dn18_slot = var_qge_dn18;
        *var_qge_dn2_slot = var_qge_dn2;
        *var_qge_dn6_slot = var_qge_dn6;
        *var_qge_dn7_slot = var_qge_dn7;
        *var_qgob_slot = var_qgob;
        *var_qgob_dn0_slot = var_qgob_dn0;
        *var_qgob_dn10_slot = var_qgob_dn10;
        *var_qgob_dn11_slot = var_qgob_dn11;
        *var_qgob_dn12_slot = var_qgob_dn12;
        *var_qgob_dn17_slot = var_qgob_dn17;
        *var_qgob_dn2_slot = var_qgob_dn2;
        *var_qgob_dn6_slot = var_qgob_dn6;
        *var_qgob_dn7_slot = var_qgob_dn7;
        *var_qgod_slot = var_qgod;
        *var_qgod_dn0_slot = var_qgod_dn0;
        *var_qgod_dn10_slot = var_qgod_dn10;
        *var_qgod_dn11_slot = var_qgod_dn11;
        *var_qgod_dn12_slot = var_qgod_dn12;
        *var_qgod_dn17_slot = var_qgod_dn17;
        *var_qgod_dn2_slot = var_qgod_dn2;
        *var_qgod_dn6_slot = var_qgod_dn6;
        *var_qgod_dn7_slot = var_qgod_dn7;
        *var_qgos_slot = var_qgos;
        *var_qgos_dn0_slot = var_qgos_dn0;
        *var_qgos_dn10_slot = var_qgos_dn10;
        *var_qgos_dn11_slot = var_qgos_dn11;
        *var_qgos_dn12_slot = var_qgos_dn12;
        *var_qgos_dn17_slot = var_qgos_dn17;
        *var_qgos_dn2_slot = var_qgos_dn2;
        *var_qgos_dn6_slot = var_qgos_dn6;
        *var_qgos_dn7_slot = var_qgos_dn7;
        *var_qi_qs_slot = var_qi_qs;
        *var_qi_qs_dn0_slot = var_qi_qs_dn0;
        *var_qi_qs_dn10_slot = var_qi_qs_dn10;
        *var_qi_qs_dn11_slot = var_qi_qs_dn11;
        *var_qi_qs_dn12_slot = var_qi_qs_dn12;
        *var_qi_qs_dn17_slot = var_qi_qs_dn17;
        *var_qi_qs_dn2_slot = var_qi_qs_dn2;
        *var_qi_qs_dn6_slot = var_qi_qs_dn6;
        *var_qi_qs_dn7_slot = var_qi_qs_dn7;
        *var_qse_slot = var_qse;
        *var_qse_dn0_slot = var_qse_dn0;
        *var_qse_dn10_slot = var_qse_dn10;
        *var_qse_dn11_slot = var_qse_dn11;
        *var_qse_dn12_slot = var_qse_dn12;
        *var_qse_dn13_slot = var_qse_dn13;
        *var_qse_dn15_slot = var_qse_dn15;
        *var_qse_dn16_slot = var_qse_dn16;
        *var_qse_dn17_slot = var_qse_dn17;
        *var_qse_dn18_slot = var_qse_dn18;
        *var_qse_dn2_slot = var_qse_dn2;
        *var_qse_dn6_slot = var_qse_dn6;
        *var_qse_dn7_slot = var_qse_dn7;
        *var_xd_slot = var_xd;
        *var_xd_dn0_slot = var_xd_dn0;
        *var_xd_dn10_slot = var_xd_dn10;
        *var_xd_dn11_slot = var_xd_dn11;
        *var_xd_dn12_slot = var_xd_dn12;
        *var_xd_dn17_slot = var_xd_dn17;
        *var_xd_dn2_slot = var_xd_dn2;
        *var_xd_dn6_slot = var_xd_dn6;
        *var_xd_dn7_slot = var_xd_dn7;
    }

    pub(super) fn stamp_transient_block_120(
        p: &Parameters,
        var_aclm: f64,
        var_cqyb0: f64,
        var_ec: f64,
        var_ec_dn0: f64,
        var_ec_dn10: f64,
        var_ec_dn11: f64,
        var_ec_dn12: f64,
        var_ec_dn17: f64,
        var_ec_dn2: f64,
        var_ec_dn6: f64,
        var_ec_dn7: f64,
        var_flg_nqs: f64,
        var_guard1127: f64,
        var_leff: f64,
        var_mfactor: f64,
        var_ps0: f64,
        var_ps0_dn0: f64,
        var_ps0_dn10: f64,
        var_ps0_dn11: f64,
        var_ps0_dn12: f64,
        var_ps0_dn17: f64,
        var_ps0_dn2: f64,
        var_ps0_dn6: f64,
        var_ps0_dn7: f64,
        var_psdl: f64,
        var_psdl_dn0: f64,
        var_psdl_dn10: f64,
        var_psdl_dn11: f64,
        var_psdl_dn12: f64,
        var_psdl_dn17: f64,
        var_psdl_dn2: f64,
        var_psdl_dn6: f64,
        var_psdl_dn7: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn11: f64,
        var_q_nsub_dn12: f64,
        var_q_nsub_dn17: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn7: f64,
        var_qb: f64,
        var_qb_dn0: f64,
        var_qb_dn10: f64,
        var_qb_dn11: f64,
        var_qb_dn12: f64,
        var_qb_dn13: f64,
        var_qb_dn15: f64,
        var_qb_dn16: f64,
        var_qb_dn17: f64,
        var_qb_dn18: f64,
        var_qb_dn2: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qbdld: f64,
        var_qbdld_dn0: f64,
        var_qbdld_dn10: f64,
        var_qbdld_dn11: f64,
        var_qbdld_dn12: f64,
        var_qbdld_dn17: f64,
        var_qbdld_dn2: f64,
        var_qbdld_dn6: f64,
        var_qbdld_dn7: f64,
        var_qbody_bt_n_iud: f64,
        var_qbody_bt_n_iud_dn0: f64,
        var_qbody_bt_n_iud_dn10: f64,
        var_qbody_bt_n_iud_dn11: f64,
        var_qbody_bt_n_iud_dn12: f64,
        var_qbody_bt_n_iud_dn17: f64,
        var_qbody_bt_n_iud_dn2: f64,
        var_qbody_bt_n_iud_dn6: f64,
        var_qbody_bt_n_iud_dn7: f64,
        var_qbody_bt_n_ius: f64,
        var_qbody_bt_n_ius_dn0: f64,
        var_qbody_bt_n_ius_dn10: f64,
        var_qbody_bt_n_ius_dn11: f64,
        var_qbody_bt_n_ius_dn12: f64,
        var_qbody_bt_n_ius_dn17: f64,
        var_qbody_bt_n_ius_dn2: f64,
        var_qbody_bt_n_ius_dn6: f64,
        var_qbody_bt_n_ius_dn7: f64,
        var_qbody_bt_n_sud: f64,
        var_qbody_bt_n_sud_dn0: f64,
        var_qbody_bt_n_sud_dn10: f64,
        var_qbody_bt_n_sud_dn11: f64,
        var_qbody_bt_n_sud_dn12: f64,
        var_qbody_bt_n_sud_dn17: f64,
        var_qbody_bt_n_sud_dn2: f64,
        var_qbody_bt_n_sud_dn6: f64,
        var_qbody_bt_n_sud_dn7: f64,
        var_qbody_bt_n_sus: f64,
        var_qbody_bt_n_sus_dn0: f64,
        var_qbody_bt_n_sus_dn10: f64,
        var_qbody_bt_n_sus_dn11: f64,
        var_qbody_bt_n_sus_dn12: f64,
        var_qbody_bt_n_sus_dn17: f64,
        var_qbody_bt_n_sus_dn2: f64,
        var_qbody_bt_n_sus_dn6: f64,
        var_qbody_bt_n_sus_dn7: f64,
        var_qbody_bt_p_iud: f64,
        var_qbody_bt_p_iud_dn0: f64,
        var_qbody_bt_p_iud_dn10: f64,
        var_qbody_bt_p_iud_dn11: f64,
        var_qbody_bt_p_iud_dn12: f64,
        var_qbody_bt_p_iud_dn17: f64,
        var_qbody_bt_p_iud_dn2: f64,
        var_qbody_bt_p_iud_dn6: f64,
        var_qbody_bt_p_iud_dn7: f64,
        var_qbody_bt_p_ius: f64,
        var_qbody_bt_p_ius_dn0: f64,
        var_qbody_bt_p_ius_dn10: f64,
        var_qbody_bt_p_ius_dn11: f64,
        var_qbody_bt_p_ius_dn12: f64,
        var_qbody_bt_p_ius_dn17: f64,
        var_qbody_bt_p_ius_dn2: f64,
        var_qbody_bt_p_ius_dn6: f64,
        var_qbody_bt_p_ius_dn7: f64,
        var_qbody_bt_p_sud: f64,
        var_qbody_bt_p_sud_dn0: f64,
        var_qbody_bt_p_sud_dn10: f64,
        var_qbody_bt_p_sud_dn11: f64,
        var_qbody_bt_p_sud_dn12: f64,
        var_qbody_bt_p_sud_dn17: f64,
        var_qbody_bt_p_sud_dn2: f64,
        var_qbody_bt_p_sud_dn6: f64,
        var_qbody_bt_p_sud_dn7: f64,
        var_qbody_bt_p_sus: f64,
        var_qbody_bt_p_sus_dn0: f64,
        var_qbody_bt_p_sus_dn10: f64,
        var_qbody_bt_p_sus_dn11: f64,
        var_qbody_bt_p_sus_dn12: f64,
        var_qbody_bt_p_sus_dn17: f64,
        var_qbody_bt_p_sus_dn2: f64,
        var_qbody_bt_p_sus_dn6: f64,
        var_qbody_bt_p_sus_dn7: f64,
        var_qbsld: f64,
        var_qbsld_dn0: f64,
        var_qbsld_dn10: f64,
        var_qbsld_dn11: f64,
        var_qbsld_dn12: f64,
        var_qbsld_dn17: f64,
        var_qbsld_dn2: f64,
        var_qbsld_dn6: f64,
        var_qbsld_dn7: f64,
        var_qd: f64,
        var_qd_dn0: f64,
        var_qd_dn10: f64,
        var_qd_dn11: f64,
        var_qd_dn12: f64,
        var_qd_dn13: f64,
        var_qd_dn15: f64,
        var_qd_dn16: f64,
        var_qd_dn17: f64,
        var_qd_dn18: f64,
        var_qd_dn2: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_fb: f64,
        var_qd_fb_dn0: f64,
        var_qd_fb_dn10: f64,
        var_qd_fb_dn11: f64,
        var_qd_fb_dn12: f64,
        var_qd_fb_dn13: f64,
        var_qd_fb_dn15: f64,
        var_qd_fb_dn16: f64,
        var_qd_fb_dn17: f64,
        var_qd_fb_dn18: f64,
        var_qd_fb_dn2: f64,
        var_qd_fb_dn6: f64,
        var_qd_fb_dn7: f64,
        var_qgob: f64,
        var_qgob_dn0: f64,
        var_qgob_dn10: f64,
        var_qgob_dn11: f64,
        var_qgob_dn12: f64,
        var_qgob_dn17: f64,
        var_qgob_dn2: f64,
        var_qgob_dn6: f64,
        var_qgob_dn7: f64,
        var_qgod: f64,
        var_qgod_dn0: f64,
        var_qgod_dn10: f64,
        var_qgod_dn11: f64,
        var_qgod_dn12: f64,
        var_qgod_dn17: f64,
        var_qgod_dn2: f64,
        var_qgod_dn6: f64,
        var_qgod_dn7: f64,
        var_qgos: f64,
        var_qgos_dn0: f64,
        var_qgos_dn10: f64,
        var_qgos_dn11: f64,
        var_qgos_dn12: f64,
        var_qgos_dn17: f64,
        var_qgos_dn2: f64,
        var_qgos_dn6: f64,
        var_qgos_dn7: f64,
        var_qi: f64,
        var_qi_dn0: f64,
        var_qi_dn10: f64,
        var_qi_dn11: f64,
        var_qi_dn12: f64,
        var_qi_dn17: f64,
        var_qi_dn2: f64,
        var_qi_dn6: f64,
        var_qi_dn7: f64,
        var_qovd: f64,
        var_qovd_dn0: f64,
        var_qovd_dn10: f64,
        var_qovd_dn11: f64,
        var_qovd_dn12: f64,
        var_qovd_dn17: f64,
        var_qovd_dn2: f64,
        var_qovd_dn6: f64,
        var_qovd_dn7: f64,
        var_qovs: f64,
        var_qovs_dn0: f64,
        var_qovs_dn10: f64,
        var_qovs_dn11: f64,
        var_qovs_dn12: f64,
        var_qovs_dn17: f64,
        var_qovs_dn2: f64,
        var_qovs_dn6: f64,
        var_qovs_dn7: f64,
        var_qs_fb: f64,
        var_qs_fb_dn0: f64,
        var_qs_fb_dn10: f64,
        var_qs_fb_dn11: f64,
        var_qs_fb_dn12: f64,
        var_qs_fb_dn13: f64,
        var_qs_fb_dn15: f64,
        var_qs_fb_dn16: f64,
        var_qs_fb_dn17: f64,
        var_qs_fb_dn18: f64,
        var_qs_fb_dn2: f64,
        var_qs_fb_dn6: f64,
        var_qs_fb_dn7: f64,
        var_qsub: f64,
        var_qsub_dn0: f64,
        var_qsub_dn10: f64,
        var_qsub_dn11: f64,
        var_qsub_dn12: f64,
        var_qsub_dn17: f64,
        var_qsub_dn2: f64,
        var_qsub_dn6: f64,
        var_qsub_dn7: f64,
        var_vbsp: f64,
        var_vbsp_dn0: f64,
        var_vbsp_dn10: f64,
        var_vbsp_dn11: f64,
        var_vbsp_dn12: f64,
        var_vbsp_dn17: f64,
        var_vbsp_dn2: f64,
        var_vbsp_dn6: f64,
        var_vbsp_dn7: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn17: f64,
        var_vds_dn2: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_weffcv_nf: f64,
        var_guard1128_slot: &mut f64,
        var_guard1134_slot: &mut f64,
        var_guard1135_slot: &mut f64,
        var_guard1136_slot: &mut f64,
        var_guard1137_slot: &mut f64,
        var_guard1138_slot: &mut f64,
        var_pslk_slot: &mut f64,
        var_pslk_dn0_slot: &mut f64,
        var_pslk_dn10_slot: &mut f64,
        var_pslk_dn11_slot: &mut f64,
        var_pslk_dn12_slot: &mut f64,
        var_pslk_dn17_slot: &mut f64,
        var_pslk_dn2_slot: &mut f64,
        var_pslk_dn6_slot: &mut f64,
        var_pslk_dn7_slot: &mut f64,
        var_q_bt_de_slot: &mut f64,
        var_q_bt_de_dn0_slot: &mut f64,
        var_q_bt_de_dn10_slot: &mut f64,
        var_q_bt_de_dn11_slot: &mut f64,
        var_q_bt_de_dn12_slot: &mut f64,
        var_q_bt_de_dn17_slot: &mut f64,
        var_q_bt_de_dn2_slot: &mut f64,
        var_q_bt_de_dn6_slot: &mut f64,
        var_q_bt_de_dn7_slot: &mut f64,
        var_q_bt_ge_slot: &mut f64,
        var_q_bt_ge_dn0_slot: &mut f64,
        var_q_bt_ge_dn10_slot: &mut f64,
        var_q_bt_ge_dn11_slot: &mut f64,
        var_q_bt_ge_dn12_slot: &mut f64,
        var_q_bt_ge_dn17_slot: &mut f64,
        var_q_bt_ge_dn2_slot: &mut f64,
        var_q_bt_ge_dn6_slot: &mut f64,
        var_q_bt_ge_dn7_slot: &mut f64,
        var_q_bt_se_slot: &mut f64,
        var_q_bt_se_dn0_slot: &mut f64,
        var_q_bt_se_dn10_slot: &mut f64,
        var_q_bt_se_dn11_slot: &mut f64,
        var_q_bt_se_dn12_slot: &mut f64,
        var_q_bt_se_dn17_slot: &mut f64,
        var_q_bt_se_dn2_slot: &mut f64,
        var_q_bt_se_dn6_slot: &mut f64,
        var_q_bt_se_dn7_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_dn0_slot: &mut f64,
        var_qde_dn10_slot: &mut f64,
        var_qde_dn11_slot: &mut f64,
        var_qde_dn12_slot: &mut f64,
        var_qde_dn13_slot: &mut f64,
        var_qde_dn15_slot: &mut f64,
        var_qde_dn16_slot: &mut f64,
        var_qde_dn17_slot: &mut f64,
        var_qde_dn18_slot: &mut f64,
        var_qde_dn2_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qde_dn7_slot: &mut f64,
        var_qge_slot: &mut f64,
        var_qge_dn0_slot: &mut f64,
        var_qge_dn10_slot: &mut f64,
        var_qge_dn11_slot: &mut f64,
        var_qge_dn12_slot: &mut f64,
        var_qge_dn13_slot: &mut f64,
        var_qge_dn15_slot: &mut f64,
        var_qge_dn16_slot: &mut f64,
        var_qge_dn17_slot: &mut f64,
        var_qge_dn18_slot: &mut f64,
        var_qge_dn2_slot: &mut f64,
        var_qge_dn6_slot: &mut f64,
        var_qge_dn7_slot: &mut f64,
        var_qs_qs_slot: &mut f64,
        var_qs_qs_dn0_slot: &mut f64,
        var_qs_qs_dn10_slot: &mut f64,
        var_qs_qs_dn11_slot: &mut f64,
        var_qs_qs_dn12_slot: &mut f64,
        var_qs_qs_dn13_slot: &mut f64,
        var_qs_qs_dn15_slot: &mut f64,
        var_qs_qs_dn16_slot: &mut f64,
        var_qs_qs_dn17_slot: &mut f64,
        var_qs_qs_dn18_slot: &mut f64,
        var_qs_qs_dn2_slot: &mut f64,
        var_qs_qs_dn6_slot: &mut f64,
        var_qs_qs_dn7_slot: &mut f64,
        var_qse_slot: &mut f64,
        var_qse_dn0_slot: &mut f64,
        var_qse_dn10_slot: &mut f64,
        var_qse_dn11_slot: &mut f64,
        var_qse_dn12_slot: &mut f64,
        var_qse_dn13_slot: &mut f64,
        var_qse_dn15_slot: &mut f64,
        var_qse_dn16_slot: &mut f64,
        var_qse_dn17_slot: &mut f64,
        var_qse_dn18_slot: &mut f64,
        var_qse_dn2_slot: &mut f64,
        var_qse_dn6_slot: &mut f64,
        var_qse_dn7_slot: &mut f64,
        var_qy_slot: &mut f64,
        var_qy_dn0_slot: &mut f64,
        var_qy_dn10_slot: &mut f64,
        var_qy_dn11_slot: &mut f64,
        var_qy_dn12_slot: &mut f64,
        var_qy_dn17_slot: &mut f64,
        var_qy_dn2_slot: &mut f64,
        var_qy_dn6_slot: &mut f64,
        var_qy_dn7_slot: &mut f64,
        var_t10__blk1131_slot: &mut f64,
        var_t10__blk1131_dn0_slot: &mut f64,
        var_t10__blk1131_dn10_slot: &mut f64,
        var_t10__blk1131_dn11_slot: &mut f64,
        var_t10__blk1131_dn12_slot: &mut f64,
        var_t10__blk1131_dn17_slot: &mut f64,
        var_t10__blk1131_dn2_slot: &mut f64,
        var_t10__blk1131_dn6_slot: &mut f64,
        var_t10__blk1131_dn7_slot: &mut f64,
        var_t1__blk1130_slot: &mut f64,
        var_t1__blk1130_dn0_slot: &mut f64,
        var_t1__blk1130_dn10_slot: &mut f64,
        var_t1__blk1130_dn11_slot: &mut f64,
        var_t1__blk1130_dn12_slot: &mut f64,
        var_t1__blk1130_dn17_slot: &mut f64,
        var_t1__blk1130_dn2_slot: &mut f64,
        var_t1__blk1130_dn6_slot: &mut f64,
        var_t1__blk1130_dn7_slot: &mut f64,
        var_t2__blk1133_slot: &mut f64,
        var_t2__blk1133_dn0_slot: &mut f64,
        var_t2__blk1133_dn10_slot: &mut f64,
        var_t2__blk1133_dn11_slot: &mut f64,
        var_t2__blk1133_dn12_slot: &mut f64,
        var_t2__blk1133_dn17_slot: &mut f64,
        var_t2__blk1133_dn2_slot: &mut f64,
        var_t2__blk1133_dn6_slot: &mut f64,
        var_t2__blk1133_dn7_slot: &mut f64,
        var_t3__blk1132_slot: &mut f64,
        var_t3__blk1132_dn0_slot: &mut f64,
        var_t3__blk1132_dn10_slot: &mut f64,
        var_t3__blk1132_dn11_slot: &mut f64,
        var_t3__blk1132_dn12_slot: &mut f64,
        var_t3__blk1132_dn17_slot: &mut f64,
        var_t3__blk1132_dn2_slot: &mut f64,
        var_t3__blk1132_dn6_slot: &mut f64,
        var_t3__blk1132_dn7_slot: &mut f64,
    ) {
        let mut var_guard1128: f64 = *var_guard1128_slot;
        let mut var_guard1134: f64 = *var_guard1134_slot;
        let mut var_guard1135: f64 = *var_guard1135_slot;
        let mut var_guard1136: f64 = *var_guard1136_slot;
        let mut var_guard1137: f64 = *var_guard1137_slot;
        let mut var_guard1138: f64 = *var_guard1138_slot;
        let mut var_pslk: f64 = *var_pslk_slot;
        let mut var_pslk_dn0: f64 = *var_pslk_dn0_slot;
        let mut var_pslk_dn10: f64 = *var_pslk_dn10_slot;
        let mut var_pslk_dn11: f64 = *var_pslk_dn11_slot;
        let mut var_pslk_dn12: f64 = *var_pslk_dn12_slot;
        let mut var_pslk_dn17: f64 = *var_pslk_dn17_slot;
        let mut var_pslk_dn2: f64 = *var_pslk_dn2_slot;
        let mut var_pslk_dn6: f64 = *var_pslk_dn6_slot;
        let mut var_pslk_dn7: f64 = *var_pslk_dn7_slot;
        let mut var_q_bt_de: f64 = *var_q_bt_de_slot;
        let mut var_q_bt_de_dn0: f64 = *var_q_bt_de_dn0_slot;
        let mut var_q_bt_de_dn10: f64 = *var_q_bt_de_dn10_slot;
        let mut var_q_bt_de_dn11: f64 = *var_q_bt_de_dn11_slot;
        let mut var_q_bt_de_dn12: f64 = *var_q_bt_de_dn12_slot;
        let mut var_q_bt_de_dn17: f64 = *var_q_bt_de_dn17_slot;
        let mut var_q_bt_de_dn2: f64 = *var_q_bt_de_dn2_slot;
        let mut var_q_bt_de_dn6: f64 = *var_q_bt_de_dn6_slot;
        let mut var_q_bt_de_dn7: f64 = *var_q_bt_de_dn7_slot;
        let mut var_q_bt_ge: f64 = *var_q_bt_ge_slot;
        let mut var_q_bt_ge_dn0: f64 = *var_q_bt_ge_dn0_slot;
        let mut var_q_bt_ge_dn10: f64 = *var_q_bt_ge_dn10_slot;
        let mut var_q_bt_ge_dn11: f64 = *var_q_bt_ge_dn11_slot;
        let mut var_q_bt_ge_dn12: f64 = *var_q_bt_ge_dn12_slot;
        let mut var_q_bt_ge_dn17: f64 = *var_q_bt_ge_dn17_slot;
        let mut var_q_bt_ge_dn2: f64 = *var_q_bt_ge_dn2_slot;
        let mut var_q_bt_ge_dn6: f64 = *var_q_bt_ge_dn6_slot;
        let mut var_q_bt_ge_dn7: f64 = *var_q_bt_ge_dn7_slot;
        let mut var_q_bt_se: f64 = *var_q_bt_se_slot;
        let mut var_q_bt_se_dn0: f64 = *var_q_bt_se_dn0_slot;
        let mut var_q_bt_se_dn10: f64 = *var_q_bt_se_dn10_slot;
        let mut var_q_bt_se_dn11: f64 = *var_q_bt_se_dn11_slot;
        let mut var_q_bt_se_dn12: f64 = *var_q_bt_se_dn12_slot;
        let mut var_q_bt_se_dn17: f64 = *var_q_bt_se_dn17_slot;
        let mut var_q_bt_se_dn2: f64 = *var_q_bt_se_dn2_slot;
        let mut var_q_bt_se_dn6: f64 = *var_q_bt_se_dn6_slot;
        let mut var_q_bt_se_dn7: f64 = *var_q_bt_se_dn7_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_dn0: f64 = *var_qde_dn0_slot;
        let mut var_qde_dn10: f64 = *var_qde_dn10_slot;
        let mut var_qde_dn11: f64 = *var_qde_dn11_slot;
        let mut var_qde_dn12: f64 = *var_qde_dn12_slot;
        let mut var_qde_dn13: f64 = *var_qde_dn13_slot;
        let mut var_qde_dn15: f64 = *var_qde_dn15_slot;
        let mut var_qde_dn16: f64 = *var_qde_dn16_slot;
        let mut var_qde_dn17: f64 = *var_qde_dn17_slot;
        let mut var_qde_dn18: f64 = *var_qde_dn18_slot;
        let mut var_qde_dn2: f64 = *var_qde_dn2_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qde_dn7: f64 = *var_qde_dn7_slot;
        let mut var_qge: f64 = *var_qge_slot;
        let mut var_qge_dn0: f64 = *var_qge_dn0_slot;
        let mut var_qge_dn10: f64 = *var_qge_dn10_slot;
        let mut var_qge_dn11: f64 = *var_qge_dn11_slot;
        let mut var_qge_dn12: f64 = *var_qge_dn12_slot;
        let mut var_qge_dn13: f64 = *var_qge_dn13_slot;
        let mut var_qge_dn15: f64 = *var_qge_dn15_slot;
        let mut var_qge_dn16: f64 = *var_qge_dn16_slot;
        let mut var_qge_dn17: f64 = *var_qge_dn17_slot;
        let mut var_qge_dn18: f64 = *var_qge_dn18_slot;
        let mut var_qge_dn2: f64 = *var_qge_dn2_slot;
        let mut var_qge_dn6: f64 = *var_qge_dn6_slot;
        let mut var_qge_dn7: f64 = *var_qge_dn7_slot;
        let mut var_qs_qs: f64 = *var_qs_qs_slot;
        let mut var_qs_qs_dn0: f64 = *var_qs_qs_dn0_slot;
        let mut var_qs_qs_dn10: f64 = *var_qs_qs_dn10_slot;
        let mut var_qs_qs_dn11: f64 = *var_qs_qs_dn11_slot;
        let mut var_qs_qs_dn12: f64 = *var_qs_qs_dn12_slot;
        let mut var_qs_qs_dn13: f64 = *var_qs_qs_dn13_slot;
        let mut var_qs_qs_dn15: f64 = *var_qs_qs_dn15_slot;
        let mut var_qs_qs_dn16: f64 = *var_qs_qs_dn16_slot;
        let mut var_qs_qs_dn17: f64 = *var_qs_qs_dn17_slot;
        let mut var_qs_qs_dn18: f64 = *var_qs_qs_dn18_slot;
        let mut var_qs_qs_dn2: f64 = *var_qs_qs_dn2_slot;
        let mut var_qs_qs_dn6: f64 = *var_qs_qs_dn6_slot;
        let mut var_qs_qs_dn7: f64 = *var_qs_qs_dn7_slot;
        let mut var_qse: f64 = *var_qse_slot;
        let mut var_qse_dn0: f64 = *var_qse_dn0_slot;
        let mut var_qse_dn10: f64 = *var_qse_dn10_slot;
        let mut var_qse_dn11: f64 = *var_qse_dn11_slot;
        let mut var_qse_dn12: f64 = *var_qse_dn12_slot;
        let mut var_qse_dn13: f64 = *var_qse_dn13_slot;
        let mut var_qse_dn15: f64 = *var_qse_dn15_slot;
        let mut var_qse_dn16: f64 = *var_qse_dn16_slot;
        let mut var_qse_dn17: f64 = *var_qse_dn17_slot;
        let mut var_qse_dn18: f64 = *var_qse_dn18_slot;
        let mut var_qse_dn2: f64 = *var_qse_dn2_slot;
        let mut var_qse_dn6: f64 = *var_qse_dn6_slot;
        let mut var_qse_dn7: f64 = *var_qse_dn7_slot;
        let mut var_qy: f64 = *var_qy_slot;
        let mut var_qy_dn0: f64 = *var_qy_dn0_slot;
        let mut var_qy_dn10: f64 = *var_qy_dn10_slot;
        let mut var_qy_dn11: f64 = *var_qy_dn11_slot;
        let mut var_qy_dn12: f64 = *var_qy_dn12_slot;
        let mut var_qy_dn17: f64 = *var_qy_dn17_slot;
        let mut var_qy_dn2: f64 = *var_qy_dn2_slot;
        let mut var_qy_dn6: f64 = *var_qy_dn6_slot;
        let mut var_qy_dn7: f64 = *var_qy_dn7_slot;
        let mut var_t10__blk1131: f64 = *var_t10__blk1131_slot;
        let mut var_t10__blk1131_dn0: f64 = *var_t10__blk1131_dn0_slot;
        let mut var_t10__blk1131_dn10: f64 = *var_t10__blk1131_dn10_slot;
        let mut var_t10__blk1131_dn11: f64 = *var_t10__blk1131_dn11_slot;
        let mut var_t10__blk1131_dn12: f64 = *var_t10__blk1131_dn12_slot;
        let mut var_t10__blk1131_dn17: f64 = *var_t10__blk1131_dn17_slot;
        let mut var_t10__blk1131_dn2: f64 = *var_t10__blk1131_dn2_slot;
        let mut var_t10__blk1131_dn6: f64 = *var_t10__blk1131_dn6_slot;
        let mut var_t10__blk1131_dn7: f64 = *var_t10__blk1131_dn7_slot;
        let mut var_t1__blk1130: f64 = *var_t1__blk1130_slot;
        let mut var_t1__blk1130_dn0: f64 = *var_t1__blk1130_dn0_slot;
        let mut var_t1__blk1130_dn10: f64 = *var_t1__blk1130_dn10_slot;
        let mut var_t1__blk1130_dn11: f64 = *var_t1__blk1130_dn11_slot;
        let mut var_t1__blk1130_dn12: f64 = *var_t1__blk1130_dn12_slot;
        let mut var_t1__blk1130_dn17: f64 = *var_t1__blk1130_dn17_slot;
        let mut var_t1__blk1130_dn2: f64 = *var_t1__blk1130_dn2_slot;
        let mut var_t1__blk1130_dn6: f64 = *var_t1__blk1130_dn6_slot;
        let mut var_t1__blk1130_dn7: f64 = *var_t1__blk1130_dn7_slot;
        let mut var_t2__blk1133: f64 = *var_t2__blk1133_slot;
        let mut var_t2__blk1133_dn0: f64 = *var_t2__blk1133_dn0_slot;
        let mut var_t2__blk1133_dn10: f64 = *var_t2__blk1133_dn10_slot;
        let mut var_t2__blk1133_dn11: f64 = *var_t2__blk1133_dn11_slot;
        let mut var_t2__blk1133_dn12: f64 = *var_t2__blk1133_dn12_slot;
        let mut var_t2__blk1133_dn17: f64 = *var_t2__blk1133_dn17_slot;
        let mut var_t2__blk1133_dn2: f64 = *var_t2__blk1133_dn2_slot;
        let mut var_t2__blk1133_dn6: f64 = *var_t2__blk1133_dn6_slot;
        let mut var_t2__blk1133_dn7: f64 = *var_t2__blk1133_dn7_slot;
        let mut var_t3__blk1132: f64 = *var_t3__blk1132_slot;
        let mut var_t3__blk1132_dn0: f64 = *var_t3__blk1132_dn0_slot;
        let mut var_t3__blk1132_dn10: f64 = *var_t3__blk1132_dn10_slot;
        let mut var_t3__blk1132_dn11: f64 = *var_t3__blk1132_dn11_slot;
        let mut var_t3__blk1132_dn12: f64 = *var_t3__blk1132_dn12_slot;
        let mut var_t3__blk1132_dn17: f64 = *var_t3__blk1132_dn17_slot;
        let mut var_t3__blk1132_dn2: f64 = *var_t3__blk1132_dn2_slot;
        let mut var_t3__blk1132_dn6: f64 = *var_t3__blk1132_dn6_slot;
        let mut var_t3__blk1132_dn7: f64 = *var_t3__blk1132_dn7_slot;

        let (assign34270_e49367, assign34270_e49367_d_n0, assign34270_e49367_d_n2, assign34270_e49367_d_n6, assign34270_e49367_d_n7, assign34270_e49367_d_n10, assign34270_e49367_d_n11, assign34270_e49367_d_n12, assign34270_e49367_d_n13, assign34270_e49367_d_n15, assign34270_e49367_d_n16, assign34270_e49367_d_n17, assign34270_e49367_d_n18,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1127 == 0.0)) {
        let assign34270_e49362: f64 = (var_qi - var_qd);
        let assign34270_e49364: f64 = (assign34270_e49362 + var_qs_fb);
        let assign34270_e49365: f64 = (var_mfactor * assign34270_e49364);
        (assign34270_e49365, (var_mfactor * ((var_qi_dn0 - var_qd_dn0) + var_qs_fb_dn0)), (var_mfactor * ((var_qi_dn2 - var_qd_dn2) + var_qs_fb_dn2)), (var_mfactor * ((var_qi_dn6 - var_qd_dn6) + var_qs_fb_dn6)), (var_mfactor * ((var_qi_dn7 - var_qd_dn7) + var_qs_fb_dn7)), (var_mfactor * ((var_qi_dn10 - var_qd_dn10) + var_qs_fb_dn10)), (var_mfactor * ((var_qi_dn11 - var_qd_dn11) + var_qs_fb_dn11)), (var_mfactor * ((var_qi_dn12 - var_qd_dn12) + var_qs_fb_dn12)), (var_mfactor * ((-var_qd_dn13) + var_qs_fb_dn13)), (var_mfactor * ((-var_qd_dn15) + var_qs_fb_dn15)), (var_mfactor * ((-var_qd_dn16) + var_qs_fb_dn16)), (var_mfactor * ((var_qi_dn17 - var_qd_dn17) + var_qs_fb_dn17)), (var_mfactor * ((-var_qd_dn18) + var_qs_fb_dn18)),)
    } else {
        (var_qs_qs, var_qs_qs_dn0, var_qs_qs_dn2, var_qs_qs_dn6, var_qs_qs_dn7, var_qs_qs_dn10, var_qs_qs_dn11, var_qs_qs_dn12, var_qs_qs_dn13, var_qs_qs_dn15, var_qs_qs_dn16, var_qs_qs_dn17, var_qs_qs_dn18,)
    }
};
        var_qs_qs = assign34270_e49367;
        var_qs_qs_dn0 = assign34270_e49367_d_n0;
        var_qs_qs_dn2 = assign34270_e49367_d_n2;
        var_qs_qs_dn6 = assign34270_e49367_d_n6;
        var_qs_qs_dn7 = assign34270_e49367_d_n7;
        var_qs_qs_dn10 = assign34270_e49367_d_n10;
        var_qs_qs_dn11 = assign34270_e49367_d_n11;
        var_qs_qs_dn12 = assign34270_e49367_d_n12;
        var_qs_qs_dn13 = assign34270_e49367_d_n13;
        var_qs_qs_dn15 = assign34270_e49367_d_n15;
        var_qs_qs_dn16 = assign34270_e49367_d_n16;
        var_qs_qs_dn17 = assign34270_e49367_d_n17;
        var_qs_qs_dn18 = assign34270_e49367_d_n18;

        let assign34280_e49370: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1128 = assign34280_e49370;

        let (assign34290_e49382, assign34290_e49382_d_n0, assign34290_e49382_d_n2, assign34290_e49382_d_n6, assign34290_e49382_d_n7, assign34290_e49382_d_n10, assign34290_e49382_d_n11, assign34290_e49382_d_n12, assign34290_e49382_d_n13, assign34290_e49382_d_n15, assign34290_e49382_d_n16, assign34290_e49382_d_n17, assign34290_e49382_d_n18,) = {
    if ((var_flg_nqs == 0.0) && (var_guard1128 != 0.0)) {
        let assign34290_e49377: f64 = (-var_qb);
        let assign34290_e49379: f64 = (assign34290_e49377 - var_qi);
        let assign34290_e49380: f64 = (var_mfactor * assign34290_e49379);
        (assign34290_e49380, (var_mfactor * ((-var_qb_dn0) - var_qi_dn0)), (var_mfactor * ((-var_qb_dn2) - var_qi_dn2)), (var_mfactor * ((-var_qb_dn6) - var_qi_dn6)), (var_mfactor * ((-var_qb_dn7) - var_qi_dn7)), (var_mfactor * ((-var_qb_dn10) - var_qi_dn10)), (var_mfactor * ((-var_qb_dn11) - var_qi_dn11)), (var_mfactor * ((-var_qb_dn12) - var_qi_dn12)), (var_mfactor * (-var_qb_dn13)), (var_mfactor * (-var_qb_dn15)), (var_mfactor * (-var_qb_dn16)), (var_mfactor * ((-var_qb_dn17) - var_qi_dn17)), (var_mfactor * (-var_qb_dn18)),)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn6, var_qge_dn7, var_qge_dn10, var_qge_dn11, var_qge_dn12, var_qge_dn13, var_qge_dn15, var_qge_dn16, var_qge_dn17, var_qge_dn18,)
    }
};
        var_qge = assign34290_e49382;
        var_qge_dn0 = assign34290_e49382_d_n0;
        var_qge_dn2 = assign34290_e49382_d_n2;
        var_qge_dn6 = assign34290_e49382_d_n6;
        var_qge_dn7 = assign34290_e49382_d_n7;
        var_qge_dn10 = assign34290_e49382_d_n10;
        var_qge_dn11 = assign34290_e49382_d_n11;
        var_qge_dn12 = assign34290_e49382_d_n12;
        var_qge_dn13 = assign34290_e49382_d_n13;
        var_qge_dn15 = assign34290_e49382_d_n15;
        var_qge_dn16 = assign34290_e49382_d_n16;
        var_qge_dn17 = assign34290_e49382_d_n17;
        var_qge_dn18 = assign34290_e49382_d_n18;

        let (assign34300_e49391, assign34300_e49391_d_n0, assign34300_e49391_d_n2, assign34300_e49391_d_n6, assign34300_e49391_d_n7, assign34300_e49391_d_n10, assign34300_e49391_d_n11, assign34300_e49391_d_n12, assign34300_e49391_d_n13, assign34300_e49391_d_n15, assign34300_e49391_d_n16, assign34300_e49391_d_n17, assign34300_e49391_d_n18,) = {
    if ((var_flg_nqs == 0.0) && (var_guard1128 != 0.0)) {
        let assign34300_e49389: f64 = (var_mfactor * var_qd);
        (assign34300_e49389, (var_mfactor * var_qd_dn0), (var_mfactor * var_qd_dn2), (var_mfactor * var_qd_dn6), (var_mfactor * var_qd_dn7), (var_mfactor * var_qd_dn10), (var_mfactor * var_qd_dn11), (var_mfactor * var_qd_dn12), (var_mfactor * var_qd_dn13), (var_mfactor * var_qd_dn15), (var_mfactor * var_qd_dn16), (var_mfactor * var_qd_dn17), (var_mfactor * var_qd_dn18),)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn6, var_qde_dn7, var_qde_dn10, var_qde_dn11, var_qde_dn12, var_qde_dn13, var_qde_dn15, var_qde_dn16, var_qde_dn17, var_qde_dn18,)
    }
};
        var_qde = assign34300_e49391;
        var_qde_dn0 = assign34300_e49391_d_n0;
        var_qde_dn2 = assign34300_e49391_d_n2;
        var_qde_dn6 = assign34300_e49391_d_n6;
        var_qde_dn7 = assign34300_e49391_d_n7;
        var_qde_dn10 = assign34300_e49391_d_n10;
        var_qde_dn11 = assign34300_e49391_d_n11;
        var_qde_dn12 = assign34300_e49391_d_n12;
        var_qde_dn13 = assign34300_e49391_d_n13;
        var_qde_dn15 = assign34300_e49391_d_n15;
        var_qde_dn16 = assign34300_e49391_d_n16;
        var_qde_dn17 = assign34300_e49391_d_n17;
        var_qde_dn18 = assign34300_e49391_d_n18;

        let (assign34310_e49402, assign34310_e49402_d_n0, assign34310_e49402_d_n2, assign34310_e49402_d_n6, assign34310_e49402_d_n7, assign34310_e49402_d_n10, assign34310_e49402_d_n11, assign34310_e49402_d_n12, assign34310_e49402_d_n13, assign34310_e49402_d_n15, assign34310_e49402_d_n16, assign34310_e49402_d_n17, assign34310_e49402_d_n18,) = {
    if ((var_flg_nqs == 0.0) && (var_guard1128 != 0.0)) {
        let assign34310_e49399: f64 = (var_qi - var_qd);
        let assign34310_e49400: f64 = (var_mfactor * assign34310_e49399);
        (assign34310_e49400, (var_mfactor * (var_qi_dn0 - var_qd_dn0)), (var_mfactor * (var_qi_dn2 - var_qd_dn2)), (var_mfactor * (var_qi_dn6 - var_qd_dn6)), (var_mfactor * (var_qi_dn7 - var_qd_dn7)), (var_mfactor * (var_qi_dn10 - var_qd_dn10)), (var_mfactor * (var_qi_dn11 - var_qd_dn11)), (var_mfactor * (var_qi_dn12 - var_qd_dn12)), (var_mfactor * (-var_qd_dn13)), (var_mfactor * (-var_qd_dn15)), (var_mfactor * (-var_qd_dn16)), (var_mfactor * (var_qi_dn17 - var_qd_dn17)), (var_mfactor * (-var_qd_dn18)),)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn6, var_qse_dn7, var_qse_dn10, var_qse_dn11, var_qse_dn12, var_qse_dn13, var_qse_dn15, var_qse_dn16, var_qse_dn17, var_qse_dn18,)
    }
};
        var_qse = assign34310_e49402;
        var_qse_dn0 = assign34310_e49402_d_n0;
        var_qse_dn2 = assign34310_e49402_d_n2;
        var_qse_dn6 = assign34310_e49402_d_n6;
        var_qse_dn7 = assign34310_e49402_d_n7;
        var_qse_dn10 = assign34310_e49402_d_n10;
        var_qse_dn11 = assign34310_e49402_d_n11;
        var_qse_dn12 = assign34310_e49402_d_n12;
        var_qse_dn13 = assign34310_e49402_d_n13;
        var_qse_dn15 = assign34310_e49402_d_n15;
        var_qse_dn16 = assign34310_e49402_d_n16;
        var_qse_dn17 = assign34310_e49402_d_n17;
        var_qse_dn18 = assign34310_e49402_d_n18;

        let (assign34320_e49419, assign34320_e49419_d_n0, assign34320_e49419_d_n2, assign34320_e49419_d_n6, assign34320_e49419_d_n7, assign34320_e49419_d_n10, assign34320_e49419_d_n11, assign34320_e49419_d_n12, assign34320_e49419_d_n13, assign34320_e49419_d_n15, assign34320_e49419_d_n16, assign34320_e49419_d_n17, assign34320_e49419_d_n18,) = {
    if ((var_flg_nqs == 0.0) && (var_guard1128 == 0.0)) {
        let assign34320_e49410: f64 = (-var_qsub);
        let assign34320_e49412: f64 = (assign34320_e49410 - var_qi);
        let assign34320_e49414: f64 = (assign34320_e49412 - var_qs_fb);
        let assign34320_e49416: f64 = (assign34320_e49414 - var_qd_fb);
        let assign34320_e49417: f64 = (var_mfactor * assign34320_e49416);
        (assign34320_e49417, (var_mfactor * ((((-var_qsub_dn0) - var_qi_dn0) - var_qs_fb_dn0) - var_qd_fb_dn0)), (var_mfactor * ((((-var_qsub_dn2) - var_qi_dn2) - var_qs_fb_dn2) - var_qd_fb_dn2)), (var_mfactor * ((((-var_qsub_dn6) - var_qi_dn6) - var_qs_fb_dn6) - var_qd_fb_dn6)), (var_mfactor * ((((-var_qsub_dn7) - var_qi_dn7) - var_qs_fb_dn7) - var_qd_fb_dn7)), (var_mfactor * ((((-var_qsub_dn10) - var_qi_dn10) - var_qs_fb_dn10) - var_qd_fb_dn10)), (var_mfactor * ((((-var_qsub_dn11) - var_qi_dn11) - var_qs_fb_dn11) - var_qd_fb_dn11)), (var_mfactor * ((((-var_qsub_dn12) - var_qi_dn12) - var_qs_fb_dn12) - var_qd_fb_dn12)), (var_mfactor * ((-var_qs_fb_dn13) - var_qd_fb_dn13)), (var_mfactor * ((-var_qs_fb_dn15) - var_qd_fb_dn15)), (var_mfactor * ((-var_qs_fb_dn16) - var_qd_fb_dn16)), (var_mfactor * ((((-var_qsub_dn17) - var_qi_dn17) - var_qs_fb_dn17) - var_qd_fb_dn17)), (var_mfactor * ((-var_qs_fb_dn18) - var_qd_fb_dn18)),)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn6, var_qge_dn7, var_qge_dn10, var_qge_dn11, var_qge_dn12, var_qge_dn13, var_qge_dn15, var_qge_dn16, var_qge_dn17, var_qge_dn18,)
    }
};
        var_qge = assign34320_e49419;
        var_qge_dn0 = assign34320_e49419_d_n0;
        var_qge_dn2 = assign34320_e49419_d_n2;
        var_qge_dn6 = assign34320_e49419_d_n6;
        var_qge_dn7 = assign34320_e49419_d_n7;
        var_qge_dn10 = assign34320_e49419_d_n10;
        var_qge_dn11 = assign34320_e49419_d_n11;
        var_qge_dn12 = assign34320_e49419_d_n12;
        var_qge_dn13 = assign34320_e49419_d_n13;
        var_qge_dn15 = assign34320_e49419_d_n15;
        var_qge_dn16 = assign34320_e49419_d_n16;
        var_qge_dn17 = assign34320_e49419_d_n17;
        var_qge_dn18 = assign34320_e49419_d_n18;

        let (assign34330_e49431, assign34330_e49431_d_n0, assign34330_e49431_d_n2, assign34330_e49431_d_n6, assign34330_e49431_d_n7, assign34330_e49431_d_n10, assign34330_e49431_d_n11, assign34330_e49431_d_n12, assign34330_e49431_d_n13, assign34330_e49431_d_n15, assign34330_e49431_d_n16, assign34330_e49431_d_n17, assign34330_e49431_d_n18,) = {
    if ((var_flg_nqs == 0.0) && (var_guard1128 == 0.0)) {
        let assign34330_e49428: f64 = (var_qd + var_qd_fb);
        let assign34330_e49429: f64 = (var_mfactor * assign34330_e49428);
        (assign34330_e49429, (var_mfactor * (var_qd_dn0 + var_qd_fb_dn0)), (var_mfactor * (var_qd_dn2 + var_qd_fb_dn2)), (var_mfactor * (var_qd_dn6 + var_qd_fb_dn6)), (var_mfactor * (var_qd_dn7 + var_qd_fb_dn7)), (var_mfactor * (var_qd_dn10 + var_qd_fb_dn10)), (var_mfactor * (var_qd_dn11 + var_qd_fb_dn11)), (var_mfactor * (var_qd_dn12 + var_qd_fb_dn12)), (var_mfactor * (var_qd_dn13 + var_qd_fb_dn13)), (var_mfactor * (var_qd_dn15 + var_qd_fb_dn15)), (var_mfactor * (var_qd_dn16 + var_qd_fb_dn16)), (var_mfactor * (var_qd_dn17 + var_qd_fb_dn17)), (var_mfactor * (var_qd_dn18 + var_qd_fb_dn18)),)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn6, var_qde_dn7, var_qde_dn10, var_qde_dn11, var_qde_dn12, var_qde_dn13, var_qde_dn15, var_qde_dn16, var_qde_dn17, var_qde_dn18,)
    }
};
        var_qde = assign34330_e49431;
        var_qde_dn0 = assign34330_e49431_d_n0;
        var_qde_dn2 = assign34330_e49431_d_n2;
        var_qde_dn6 = assign34330_e49431_d_n6;
        var_qde_dn7 = assign34330_e49431_d_n7;
        var_qde_dn10 = assign34330_e49431_d_n10;
        var_qde_dn11 = assign34330_e49431_d_n11;
        var_qde_dn12 = assign34330_e49431_d_n12;
        var_qde_dn13 = assign34330_e49431_d_n13;
        var_qde_dn15 = assign34330_e49431_d_n15;
        var_qde_dn16 = assign34330_e49431_d_n16;
        var_qde_dn17 = assign34330_e49431_d_n17;
        var_qde_dn18 = assign34330_e49431_d_n18;

        let (assign34340_e49445, assign34340_e49445_d_n0, assign34340_e49445_d_n2, assign34340_e49445_d_n6, assign34340_e49445_d_n7, assign34340_e49445_d_n10, assign34340_e49445_d_n11, assign34340_e49445_d_n12, assign34340_e49445_d_n13, assign34340_e49445_d_n15, assign34340_e49445_d_n16, assign34340_e49445_d_n17, assign34340_e49445_d_n18,) = {
    if ((var_flg_nqs == 0.0) && (var_guard1128 == 0.0)) {
        let assign34340_e49440: f64 = (var_qi - var_qd);
        let assign34340_e49442: f64 = (assign34340_e49440 + var_qs_fb);
        let assign34340_e49443: f64 = (var_mfactor * assign34340_e49442);
        (assign34340_e49443, (var_mfactor * ((var_qi_dn0 - var_qd_dn0) + var_qs_fb_dn0)), (var_mfactor * ((var_qi_dn2 - var_qd_dn2) + var_qs_fb_dn2)), (var_mfactor * ((var_qi_dn6 - var_qd_dn6) + var_qs_fb_dn6)), (var_mfactor * ((var_qi_dn7 - var_qd_dn7) + var_qs_fb_dn7)), (var_mfactor * ((var_qi_dn10 - var_qd_dn10) + var_qs_fb_dn10)), (var_mfactor * ((var_qi_dn11 - var_qd_dn11) + var_qs_fb_dn11)), (var_mfactor * ((var_qi_dn12 - var_qd_dn12) + var_qs_fb_dn12)), (var_mfactor * ((-var_qd_dn13) + var_qs_fb_dn13)), (var_mfactor * ((-var_qd_dn15) + var_qs_fb_dn15)), (var_mfactor * ((-var_qd_dn16) + var_qs_fb_dn16)), (var_mfactor * ((var_qi_dn17 - var_qd_dn17) + var_qs_fb_dn17)), (var_mfactor * ((-var_qd_dn18) + var_qs_fb_dn18)),)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn6, var_qse_dn7, var_qse_dn10, var_qse_dn11, var_qse_dn12, var_qse_dn13, var_qse_dn15, var_qse_dn16, var_qse_dn17, var_qse_dn18,)
    }
};
        var_qse = assign34340_e49445;
        var_qse_dn0 = assign34340_e49445_d_n0;
        var_qse_dn2 = assign34340_e49445_d_n2;
        var_qse_dn6 = assign34340_e49445_d_n6;
        var_qse_dn7 = assign34340_e49445_d_n7;
        var_qse_dn10 = assign34340_e49445_d_n10;
        var_qse_dn11 = assign34340_e49445_d_n11;
        var_qse_dn12 = assign34340_e49445_d_n12;
        var_qse_dn13 = assign34340_e49445_d_n13;
        var_qse_dn15 = assign34340_e49445_d_n15;
        var_qse_dn16 = assign34340_e49445_d_n16;
        var_qse_dn17 = assign34340_e49445_d_n17;
        var_qse_dn18 = assign34340_e49445_d_n18;

        let assign34350_e49448: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        var_guard1134 = assign34350_e49448;

        let (assign34360_e49452, assign34360_e49452_d_n0, assign34360_e49452_d_n2, assign34360_e49452_d_n6, assign34360_e49452_d_n7, assign34360_e49452_d_n10, assign34360_e49452_d_n11, assign34360_e49452_d_n12, assign34360_e49452_d_n17,) = {
    if (var_guard1134 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qy, var_qy_dn0, var_qy_dn2, var_qy_dn6, var_qy_dn7, var_qy_dn10, var_qy_dn11, var_qy_dn12, var_qy_dn17,)
    }
};
        var_qy = assign34360_e49452;
        var_qy_dn0 = assign34360_e49452_d_n0;
        var_qy_dn2 = assign34360_e49452_d_n2;
        var_qy_dn6 = assign34360_e49452_d_n6;
        var_qy_dn7 = assign34360_e49452_d_n7;
        var_qy_dn10 = assign34360_e49452_d_n10;
        var_qy_dn11 = assign34360_e49452_d_n11;
        var_qy_dn12 = assign34360_e49452_d_n12;
        var_qy_dn17 = assign34360_e49452_d_n17;

        let (assign34370_e49461, assign34370_e49461_d_n0, assign34370_e49461_d_n2, assign34370_e49461_d_n6, assign34370_e49461_d_n7, assign34370_e49461_d_n10, assign34370_e49461_d_n11, assign34370_e49461_d_n12, assign34370_e49461_d_n17,) = {
    if (var_guard1134 == 0.0) {
        let assign34370_e49457: f64 = (var_ec * var_leff);
        let assign34370_e49459: f64 = (assign34370_e49457 + var_ps0);
        (assign34370_e49459, ((var_ec_dn0 * var_leff) + var_ps0_dn0), ((var_ec_dn2 * var_leff) + var_ps0_dn2), ((var_ec_dn6 * var_leff) + var_ps0_dn6), ((var_ec_dn7 * var_leff) + var_ps0_dn7), ((var_ec_dn10 * var_leff) + var_ps0_dn10), ((var_ec_dn11 * var_leff) + var_ps0_dn11), ((var_ec_dn12 * var_leff) + var_ps0_dn12), ((var_ec_dn17 * var_leff) + var_ps0_dn17),)
    } else {
        (var_pslk, var_pslk_dn0, var_pslk_dn2, var_pslk_dn6, var_pslk_dn7, var_pslk_dn10, var_pslk_dn11, var_pslk_dn12, var_pslk_dn17,)
    }
};
        var_pslk = assign34370_e49461;
        var_pslk_dn0 = assign34370_e49461_d_n0;
        var_pslk_dn2 = assign34370_e49461_d_n2;
        var_pslk_dn6 = assign34370_e49461_d_n6;
        var_pslk_dn7 = assign34370_e49461_d_n7;
        var_pslk_dn10 = assign34370_e49461_d_n10;
        var_pslk_dn11 = assign34370_e49461_d_n11;
        var_pslk_dn12 = assign34370_e49461_d_n12;
        var_pslk_dn17 = assign34370_e49461_d_n17;

        let assign34380_e49464: f64 = if var_pslk > var_psdl { 1.0 } else { 0.0 };
        var_guard1135 = assign34380_e49464;

        let (assign34390_e49471, assign34390_e49471_d_n0, assign34390_e49471_d_n2, assign34390_e49471_d_n6, assign34390_e49471_d_n7, assign34390_e49471_d_n10, assign34390_e49471_d_n11, assign34390_e49471_d_n12, assign34390_e49471_d_n17,) = {
    if ((var_guard1134 == 0.0) && (var_guard1135 != 0.0)) {
        (var_psdl, var_psdl_dn0, var_psdl_dn2, var_psdl_dn6, var_psdl_dn7, var_psdl_dn10, var_psdl_dn11, var_psdl_dn12, var_psdl_dn17,)
    } else {
        (var_pslk, var_pslk_dn0, var_pslk_dn2, var_pslk_dn6, var_pslk_dn7, var_pslk_dn10, var_pslk_dn11, var_pslk_dn12, var_pslk_dn17,)
    }
};
        var_pslk = assign34390_e49471;
        var_pslk_dn0 = assign34390_e49471_d_n0;
        var_pslk_dn2 = assign34390_e49471_d_n2;
        var_pslk_dn6 = assign34390_e49471_d_n6;
        var_pslk_dn7 = assign34390_e49471_d_n7;
        var_pslk_dn10 = assign34390_e49471_d_n10;
        var_pslk_dn11 = assign34390_e49471_d_n11;
        var_pslk_dn12 = assign34390_e49471_d_n12;
        var_pslk_dn17 = assign34390_e49471_d_n17;

        let (assign34400_e49486, assign34400_e49486_d_n0, assign34400_e49486_d_n2, assign34400_e49486_d_n6, assign34400_e49486_d_n7, assign34400_e49486_d_n10, assign34400_e49486_d_n11, assign34400_e49486_d_n12, assign34400_e49486_d_n17,) = {
    if (var_guard1134 == 0.0) {
        let assign34400_e49477: f64 = (var_vds + var_ps0);
        let assign34400_e49478: f64 = (var_aclm * assign34400_e49477);
        let assign34400_e49481: f64 = (1.0 - var_aclm);
        let assign34400_e49483: f64 = (assign34400_e49481 * var_pslk);
        let assign34400_e49484: f64 = (assign34400_e49478 + assign34400_e49483);
        (assign34400_e49484, ((var_aclm * (var_vds_dn0 + var_ps0_dn0)) + (assign34400_e49481 * var_pslk_dn0)), ((var_aclm * (var_vds_dn2 + var_ps0_dn2)) + (assign34400_e49481 * var_pslk_dn2)), ((var_aclm * (var_vds_dn6 + var_ps0_dn6)) + (assign34400_e49481 * var_pslk_dn6)), ((var_aclm * (var_vds_dn7 + var_ps0_dn7)) + (assign34400_e49481 * var_pslk_dn7)), ((var_aclm * (var_vds_dn10 + var_ps0_dn10)) + (assign34400_e49481 * var_pslk_dn10)), ((var_aclm * (var_vds_dn11 + var_ps0_dn11)) + (assign34400_e49481 * var_pslk_dn11)), ((var_aclm * (var_vds_dn12 + var_ps0_dn12)) + (assign34400_e49481 * var_pslk_dn12)), ((var_aclm * (var_vds_dn17 + var_ps0_dn17)) + (assign34400_e49481 * var_pslk_dn17)),)
    } else {
        (var_t1__blk1130, var_t1__blk1130_dn0, var_t1__blk1130_dn2, var_t1__blk1130_dn6, var_t1__blk1130_dn7, var_t1__blk1130_dn10, var_t1__blk1130_dn11, var_t1__blk1130_dn12, var_t1__blk1130_dn17,)
    }
};
        var_t1__blk1130 = assign34400_e49486;
        var_t1__blk1130_dn0 = assign34400_e49486_d_n0;
        var_t1__blk1130_dn2 = assign34400_e49486_d_n2;
        var_t1__blk1130_dn6 = assign34400_e49486_d_n6;
        var_t1__blk1130_dn7 = assign34400_e49486_d_n7;
        var_t1__blk1130_dn10 = assign34400_e49486_d_n10;
        var_t1__blk1130_dn11 = assign34400_e49486_d_n11;
        var_t1__blk1130_dn12 = assign34400_e49486_d_n12;
        var_t1__blk1130_dn17 = assign34400_e49486_d_n17;

        let (assign34410_e49496, assign34410_e49496_d_n0, assign34410_e49496_d_n2, assign34410_e49496_d_n6, assign34410_e49496_d_n7, assign34410_e49496_d_n10, assign34410_e49496_d_n11, assign34410_e49496_d_n12, assign34410_e49496_d_n17,) = {
    if (var_guard1134 == 0.0) {
        let assign34410_e49491: f64 = (2.0 * 1.034943e-10);
        let assign34410_e49493: f64 = (assign34410_e49491 / var_q_nsub);
        let assign34410_e49494: f64 = (assign34410_e49493).sqrt();
        (assign34410_e49494, ((-((assign34410_e49491 * var_q_nsub_dn0) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * var_q_nsub_dn2) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * var_q_nsub_dn6) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * var_q_nsub_dn7) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * var_q_nsub_dn10) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * var_q_nsub_dn11) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * var_q_nsub_dn12) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34410_e49494)), ((-((assign34410_e49491 * var_q_nsub_dn17) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34410_e49494)),)
    } else {
        (var_t10__blk1131, var_t10__blk1131_dn0, var_t10__blk1131_dn2, var_t10__blk1131_dn6, var_t10__blk1131_dn7, var_t10__blk1131_dn10, var_t10__blk1131_dn11, var_t10__blk1131_dn12, var_t10__blk1131_dn17,)
    }
};
        var_t10__blk1131 = assign34410_e49496;
        var_t10__blk1131_dn0 = assign34410_e49496_d_n0;
        var_t10__blk1131_dn2 = assign34410_e49496_d_n2;
        var_t10__blk1131_dn6 = assign34410_e49496_d_n6;
        var_t10__blk1131_dn7 = assign34410_e49496_d_n7;
        var_t10__blk1131_dn10 = assign34410_e49496_d_n10;
        var_t10__blk1131_dn11 = assign34410_e49496_d_n11;
        var_t10__blk1131_dn12 = assign34410_e49496_d_n12;
        var_t10__blk1131_dn17 = assign34410_e49496_d_n17;

        let (assign34420_e49503, assign34420_e49503_d_n0, assign34420_e49503_d_n2, assign34420_e49503_d_n6, assign34420_e49503_d_n7, assign34420_e49503_d_n10, assign34420_e49503_d_n11, assign34420_e49503_d_n12, assign34420_e49503_d_n17,) = {
    if (var_guard1134 == 0.0) {
        let assign34420_e49501: f64 = (var_t10__blk1131 * 1.3);
        (assign34420_e49501, (var_t10__blk1131_dn0 * 1.3), (var_t10__blk1131_dn2 * 1.3), (var_t10__blk1131_dn6 * 1.3), (var_t10__blk1131_dn7 * 1.3), (var_t10__blk1131_dn10 * 1.3), (var_t10__blk1131_dn11 * 1.3), (var_t10__blk1131_dn12 * 1.3), (var_t10__blk1131_dn17 * 1.3),)
    } else {
        (var_t3__blk1132, var_t3__blk1132_dn0, var_t3__blk1132_dn2, var_t3__blk1132_dn6, var_t3__blk1132_dn7, var_t3__blk1132_dn10, var_t3__blk1132_dn11, var_t3__blk1132_dn12, var_t3__blk1132_dn17,)
    }
};
        var_t3__blk1132 = assign34420_e49503;
        var_t3__blk1132_dn0 = assign34420_e49503_d_n0;
        var_t3__blk1132_dn2 = assign34420_e49503_d_n2;
        var_t3__blk1132_dn6 = assign34420_e49503_d_n6;
        var_t3__blk1132_dn7 = assign34420_e49503_d_n7;
        var_t3__blk1132_dn10 = assign34420_e49503_d_n10;
        var_t3__blk1132_dn11 = assign34420_e49503_d_n11;
        var_t3__blk1132_dn12 = assign34420_e49503_d_n12;
        var_t3__blk1132_dn17 = assign34420_e49503_d_n17;

        let (assign34430_e49512, assign34430_e49512_d_n0, assign34430_e49512_d_n2, assign34430_e49512_d_n6, assign34430_e49512_d_n7, assign34430_e49512_d_n10, assign34430_e49512_d_n11, assign34430_e49512_d_n12, assign34430_e49512_d_n17,) = {
    if (var_guard1134 == 0.0) {
        let assign34430_e49508: f64 = (1.034943e-10 * var_weffcv_nf);
        let assign34430_e49510: f64 = (assign34430_e49508 * var_t3__blk1132);
        (assign34430_e49510, (assign34430_e49508 * var_t3__blk1132_dn0), (assign34430_e49508 * var_t3__blk1132_dn2), (assign34430_e49508 * var_t3__blk1132_dn6), (assign34430_e49508 * var_t3__blk1132_dn7), (assign34430_e49508 * var_t3__blk1132_dn10), (assign34430_e49508 * var_t3__blk1132_dn11), (assign34430_e49508 * var_t3__blk1132_dn12), (assign34430_e49508 * var_t3__blk1132_dn17),)
    } else {
        (var_t2__blk1133, var_t2__blk1133_dn0, var_t2__blk1133_dn2, var_t2__blk1133_dn6, var_t2__blk1133_dn7, var_t2__blk1133_dn10, var_t2__blk1133_dn11, var_t2__blk1133_dn12, var_t2__blk1133_dn17,)
    }
};
        var_t2__blk1133 = assign34430_e49512;
        var_t2__blk1133_dn0 = assign34430_e49512_d_n0;
        var_t2__blk1133_dn2 = assign34430_e49512_d_n2;
        var_t2__blk1133_dn6 = assign34430_e49512_d_n6;
        var_t2__blk1133_dn7 = assign34430_e49512_d_n7;
        var_t2__blk1133_dn10 = assign34430_e49512_d_n10;
        var_t2__blk1133_dn11 = assign34430_e49512_d_n11;
        var_t2__blk1133_dn12 = assign34430_e49512_d_n12;
        var_t2__blk1133_dn17 = assign34430_e49512_d_n17;

        let (assign34440_e49527, assign34440_e49527_d_n0, assign34440_e49527_d_n2, assign34440_e49527_d_n6, assign34440_e49527_d_n7, assign34440_e49527_d_n10, assign34440_e49527_d_n11, assign34440_e49527_d_n12, assign34440_e49527_d_n17,) = {
    if (var_guard1134 == 0.0) {
        let assign34440_e49517: f64 = (var_ps0 + var_vds);
        let assign34440_e49519: f64 = (assign34440_e49517 - var_t1__blk1130);
        let assign34440_e49521: f64 = (assign34440_e49519 / p.p64);
        let assign34440_e49523: f64 = (assign34440_e49521 - var_ec);
        let assign34440_e49525: f64 = (assign34440_e49523 * var_t2__blk1133);
        (assign34440_e49525, ((((((var_ps0_dn0 + var_vds_dn0) - var_t1__blk1130_dn0) / p.p64) - var_ec_dn0) * var_t2__blk1133) + (assign34440_e49523 * var_t2__blk1133_dn0)), ((((((var_ps0_dn2 + var_vds_dn2) - var_t1__blk1130_dn2) / p.p64) - var_ec_dn2) * var_t2__blk1133) + (assign34440_e49523 * var_t2__blk1133_dn2)), ((((((var_ps0_dn6 + var_vds_dn6) - var_t1__blk1130_dn6) / p.p64) - var_ec_dn6) * var_t2__blk1133) + (assign34440_e49523 * var_t2__blk1133_dn6)), ((((((var_ps0_dn7 + var_vds_dn7) - var_t1__blk1130_dn7) / p.p64) - var_ec_dn7) * var_t2__blk1133) + (assign34440_e49523 * var_t2__blk1133_dn7)), ((((((var_ps0_dn10 + var_vds_dn10) - var_t1__blk1130_dn10) / p.p64) - var_ec_dn10) * var_t2__blk1133) + (assign34440_e49523 * var_t2__blk1133_dn10)), ((((((var_ps0_dn11 + var_vds_dn11) - var_t1__blk1130_dn11) / p.p64) - var_ec_dn11) * var_t2__blk1133) + (assign34440_e49523 * var_t2__blk1133_dn11)), ((((((var_ps0_dn12 + var_vds_dn12) - var_t1__blk1130_dn12) / p.p64) - var_ec_dn12) * var_t2__blk1133) + (assign34440_e49523 * var_t2__blk1133_dn12)), ((((((var_ps0_dn17 + var_vds_dn17) - var_t1__blk1130_dn17) / p.p64) - var_ec_dn17) * var_t2__blk1133) + (assign34440_e49523 * var_t2__blk1133_dn17)),)
    } else {
        (var_qy, var_qy_dn0, var_qy_dn2, var_qy_dn6, var_qy_dn7, var_qy_dn10, var_qy_dn11, var_qy_dn12, var_qy_dn17,)
    }
};
        var_qy = assign34440_e49527;
        var_qy_dn0 = assign34440_e49527_d_n0;
        var_qy_dn2 = assign34440_e49527_d_n2;
        var_qy_dn6 = assign34440_e49527_d_n6;
        var_qy_dn7 = assign34440_e49527_d_n7;
        var_qy_dn10 = assign34440_e49527_d_n10;
        var_qy_dn11 = assign34440_e49527_d_n11;
        var_qy_dn12 = assign34440_e49527_d_n12;
        var_qy_dn17 = assign34440_e49527_d_n17;

        let assign34450_e49530: f64 = if p.p65 != 0.0 { 1.0 } else { 0.0 };
        var_guard1136 = assign34450_e49530;

        let (assign34460_e49538, assign34460_e49538_d_n0, assign34460_e49538_d_n2, assign34460_e49538_d_n6, assign34460_e49538_d_n7, assign34460_e49538_d_n10, assign34460_e49538_d_n11, assign34460_e49538_d_n12, assign34460_e49538_d_n17,) = {
    if (var_guard1136 != 0.0) {
        let assign34460_e49535: f64 = (var_cqyb0 * var_vbsp);
        let assign34460_e49536: f64 = (var_qy + assign34460_e49535);
        (assign34460_e49536, (var_qy_dn0 + (var_cqyb0 * var_vbsp_dn0)), (var_qy_dn2 + (var_cqyb0 * var_vbsp_dn2)), (var_qy_dn6 + (var_cqyb0 * var_vbsp_dn6)), (var_qy_dn7 + (var_cqyb0 * var_vbsp_dn7)), (var_qy_dn10 + (var_cqyb0 * var_vbsp_dn10)), (var_qy_dn11 + (var_cqyb0 * var_vbsp_dn11)), (var_qy_dn12 + (var_cqyb0 * var_vbsp_dn12)), (var_qy_dn17 + (var_cqyb0 * var_vbsp_dn17)),)
    } else {
        (var_qy, var_qy_dn0, var_qy_dn2, var_qy_dn6, var_qy_dn7, var_qy_dn10, var_qy_dn11, var_qy_dn12, var_qy_dn17,)
    }
};
        var_qy = assign34460_e49538;
        var_qy_dn0 = assign34460_e49538_d_n0;
        var_qy_dn2 = assign34460_e49538_d_n2;
        var_qy_dn6 = assign34460_e49538_d_n6;
        var_qy_dn7 = assign34460_e49538_d_n7;
        var_qy_dn10 = assign34460_e49538_d_n10;
        var_qy_dn11 = assign34460_e49538_d_n11;
        var_qy_dn12 = assign34460_e49538_d_n12;
        var_qy_dn17 = assign34460_e49538_d_n17;

        let assign34470_e49541: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        var_guard1137 = assign34470_e49541;

        let assign34480_e49544: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1138 = assign34480_e49544;

        let (assign34490_e49557, assign34490_e49557_d_n0, assign34490_e49557_d_n2, assign34490_e49557_d_n6, assign34490_e49557_d_n7, assign34490_e49557_d_n10, assign34490_e49557_d_n11, assign34490_e49557_d_n12, assign34490_e49557_d_n17,) = {
    if ((var_guard1137 != 0.0) && (var_guard1138 != 0.0)) {
        let assign34490_e49549: f64 = (-var_qbody_bt_p_sus);
        let assign34490_e49551: f64 = (assign34490_e49549 - var_qbody_bt_p_sud);
        let assign34490_e49553: f64 = (assign34490_e49551 - var_qbody_bt_n_sus);
        let assign34490_e49555: f64 = (assign34490_e49553 - var_qbody_bt_n_sud);
        (assign34490_e49555, ((((-var_qbody_bt_p_sus_dn0) - var_qbody_bt_p_sud_dn0) - var_qbody_bt_n_sus_dn0) - var_qbody_bt_n_sud_dn0), ((((-var_qbody_bt_p_sus_dn2) - var_qbody_bt_p_sud_dn2) - var_qbody_bt_n_sus_dn2) - var_qbody_bt_n_sud_dn2), ((((-var_qbody_bt_p_sus_dn6) - var_qbody_bt_p_sud_dn6) - var_qbody_bt_n_sus_dn6) - var_qbody_bt_n_sud_dn6), ((((-var_qbody_bt_p_sus_dn7) - var_qbody_bt_p_sud_dn7) - var_qbody_bt_n_sus_dn7) - var_qbody_bt_n_sud_dn7), ((((-var_qbody_bt_p_sus_dn10) - var_qbody_bt_p_sud_dn10) - var_qbody_bt_n_sus_dn10) - var_qbody_bt_n_sud_dn10), ((((-var_qbody_bt_p_sus_dn11) - var_qbody_bt_p_sud_dn11) - var_qbody_bt_n_sus_dn11) - var_qbody_bt_n_sud_dn11), ((((-var_qbody_bt_p_sus_dn12) - var_qbody_bt_p_sud_dn12) - var_qbody_bt_n_sus_dn12) - var_qbody_bt_n_sud_dn12), ((((-var_qbody_bt_p_sus_dn17) - var_qbody_bt_p_sud_dn17) - var_qbody_bt_n_sus_dn17) - var_qbody_bt_n_sud_dn17),)
    } else {
        (var_q_bt_ge, var_q_bt_ge_dn0, var_q_bt_ge_dn2, var_q_bt_ge_dn6, var_q_bt_ge_dn7, var_q_bt_ge_dn10, var_q_bt_ge_dn11, var_q_bt_ge_dn12, var_q_bt_ge_dn17,)
    }
};
        var_q_bt_ge = assign34490_e49557;
        var_q_bt_ge_dn0 = assign34490_e49557_d_n0;
        var_q_bt_ge_dn2 = assign34490_e49557_d_n2;
        var_q_bt_ge_dn6 = assign34490_e49557_d_n6;
        var_q_bt_ge_dn7 = assign34490_e49557_d_n7;
        var_q_bt_ge_dn10 = assign34490_e49557_d_n10;
        var_q_bt_ge_dn11 = assign34490_e49557_d_n11;
        var_q_bt_ge_dn12 = assign34490_e49557_d_n12;
        var_q_bt_ge_dn17 = assign34490_e49557_d_n17;

        let (assign34500_e49565, assign34500_e49565_d_n0, assign34500_e49565_d_n2, assign34500_e49565_d_n6, assign34500_e49565_d_n7, assign34500_e49565_d_n10, assign34500_e49565_d_n11, assign34500_e49565_d_n12, assign34500_e49565_d_n17,) = {
    if ((var_guard1137 != 0.0) && (var_guard1138 != 0.0)) {
        let assign34500_e49563: f64 = (var_qbody_bt_p_iud + var_qbody_bt_n_iud);
        (assign34500_e49563, (var_qbody_bt_p_iud_dn0 + var_qbody_bt_n_iud_dn0), (var_qbody_bt_p_iud_dn2 + var_qbody_bt_n_iud_dn2), (var_qbody_bt_p_iud_dn6 + var_qbody_bt_n_iud_dn6), (var_qbody_bt_p_iud_dn7 + var_qbody_bt_n_iud_dn7), (var_qbody_bt_p_iud_dn10 + var_qbody_bt_n_iud_dn10), (var_qbody_bt_p_iud_dn11 + var_qbody_bt_n_iud_dn11), (var_qbody_bt_p_iud_dn12 + var_qbody_bt_n_iud_dn12), (var_qbody_bt_p_iud_dn17 + var_qbody_bt_n_iud_dn17),)
    } else {
        (var_q_bt_de, var_q_bt_de_dn0, var_q_bt_de_dn2, var_q_bt_de_dn6, var_q_bt_de_dn7, var_q_bt_de_dn10, var_q_bt_de_dn11, var_q_bt_de_dn12, var_q_bt_de_dn17,)
    }
};
        var_q_bt_de = assign34500_e49565;
        var_q_bt_de_dn0 = assign34500_e49565_d_n0;
        var_q_bt_de_dn2 = assign34500_e49565_d_n2;
        var_q_bt_de_dn6 = assign34500_e49565_d_n6;
        var_q_bt_de_dn7 = assign34500_e49565_d_n7;
        var_q_bt_de_dn10 = assign34500_e49565_d_n10;
        var_q_bt_de_dn11 = assign34500_e49565_d_n11;
        var_q_bt_de_dn12 = assign34500_e49565_d_n12;
        var_q_bt_de_dn17 = assign34500_e49565_d_n17;

        let (assign34510_e49573, assign34510_e49573_d_n0, assign34510_e49573_d_n2, assign34510_e49573_d_n6, assign34510_e49573_d_n7, assign34510_e49573_d_n10, assign34510_e49573_d_n11, assign34510_e49573_d_n12, assign34510_e49573_d_n17,) = {
    if ((var_guard1137 != 0.0) && (var_guard1138 != 0.0)) {
        let assign34510_e49571: f64 = (var_qbody_bt_p_ius + var_qbody_bt_n_ius);
        (assign34510_e49571, (var_qbody_bt_p_ius_dn0 + var_qbody_bt_n_ius_dn0), (var_qbody_bt_p_ius_dn2 + var_qbody_bt_n_ius_dn2), (var_qbody_bt_p_ius_dn6 + var_qbody_bt_n_ius_dn6), (var_qbody_bt_p_ius_dn7 + var_qbody_bt_n_ius_dn7), (var_qbody_bt_p_ius_dn10 + var_qbody_bt_n_ius_dn10), (var_qbody_bt_p_ius_dn11 + var_qbody_bt_n_ius_dn11), (var_qbody_bt_p_ius_dn12 + var_qbody_bt_n_ius_dn12), (var_qbody_bt_p_ius_dn17 + var_qbody_bt_n_ius_dn17),)
    } else {
        (var_q_bt_se, var_q_bt_se_dn0, var_q_bt_se_dn2, var_q_bt_se_dn6, var_q_bt_se_dn7, var_q_bt_se_dn10, var_q_bt_se_dn11, var_q_bt_se_dn12, var_q_bt_se_dn17,)
    }
};
        var_q_bt_se = assign34510_e49573;
        var_q_bt_se_dn0 = assign34510_e49573_d_n0;
        var_q_bt_se_dn2 = assign34510_e49573_d_n2;
        var_q_bt_se_dn6 = assign34510_e49573_d_n6;
        var_q_bt_se_dn7 = assign34510_e49573_d_n7;
        var_q_bt_se_dn10 = assign34510_e49573_d_n10;
        var_q_bt_se_dn11 = assign34510_e49573_d_n11;
        var_q_bt_se_dn12 = assign34510_e49573_d_n12;
        var_q_bt_se_dn17 = assign34510_e49573_d_n17;

        let (assign34520_e49595, assign34520_e49595_d_n0, assign34520_e49595_d_n2, assign34520_e49595_d_n6, assign34520_e49595_d_n7, assign34520_e49595_d_n10, assign34520_e49595_d_n11, assign34520_e49595_d_n12, assign34520_e49595_d_n13, assign34520_e49595_d_n15, assign34520_e49595_d_n16, assign34520_e49595_d_n17, assign34520_e49595_d_n18,) = {
    if ((var_guard1137 != 0.0) && (var_guard1138 != 0.0)) {
        let assign34520_e49581: f64 = (var_qgod + var_qgos);
        let assign34520_e49583: f64 = (assign34520_e49581 + var_qgob);
        let assign34520_e49585: f64 = (assign34520_e49583 - var_qy);
        let assign34520_e49587: f64 = (assign34520_e49585 - var_qovs);
        let assign34520_e49589: f64 = (assign34520_e49587 - var_qovd);
        let assign34520_e49591: f64 = (assign34520_e49589 + var_q_bt_ge);
        let assign34520_e49592: f64 = (var_mfactor * assign34520_e49591);
        let assign34520_e49593: f64 = (var_qge + assign34520_e49592);
        (assign34520_e49593, (var_qge_dn0 + (var_mfactor * ((((((var_qgod_dn0 + var_qgos_dn0) + var_qgob_dn0) - var_qy_dn0) - var_qovs_dn0) - var_qovd_dn0) + var_q_bt_ge_dn0))), (var_qge_dn2 + (var_mfactor * ((((((var_qgod_dn2 + var_qgos_dn2) + var_qgob_dn2) - var_qy_dn2) - var_qovs_dn2) - var_qovd_dn2) + var_q_bt_ge_dn2))), (var_qge_dn6 + (var_mfactor * ((((((var_qgod_dn6 + var_qgos_dn6) + var_qgob_dn6) - var_qy_dn6) - var_qovs_dn6) - var_qovd_dn6) + var_q_bt_ge_dn6))), (var_qge_dn7 + (var_mfactor * ((((((var_qgod_dn7 + var_qgos_dn7) + var_qgob_dn7) - var_qy_dn7) - var_qovs_dn7) - var_qovd_dn7) + var_q_bt_ge_dn7))), (var_qge_dn10 + (var_mfactor * ((((((var_qgod_dn10 + var_qgos_dn10) + var_qgob_dn10) - var_qy_dn10) - var_qovs_dn10) - var_qovd_dn10) + var_q_bt_ge_dn10))), (var_qge_dn11 + (var_mfactor * ((((((var_qgod_dn11 + var_qgos_dn11) + var_qgob_dn11) - var_qy_dn11) - var_qovs_dn11) - var_qovd_dn11) + var_q_bt_ge_dn11))), (var_qge_dn12 + (var_mfactor * ((((((var_qgod_dn12 + var_qgos_dn12) + var_qgob_dn12) - var_qy_dn12) - var_qovs_dn12) - var_qovd_dn12) + var_q_bt_ge_dn12))), var_qge_dn13, var_qge_dn15, var_qge_dn16, (var_qge_dn17 + (var_mfactor * ((((((var_qgod_dn17 + var_qgos_dn17) + var_qgob_dn17) - var_qy_dn17) - var_qovs_dn17) - var_qovd_dn17) + var_q_bt_ge_dn17))), var_qge_dn18,)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn6, var_qge_dn7, var_qge_dn10, var_qge_dn11, var_qge_dn12, var_qge_dn13, var_qge_dn15, var_qge_dn16, var_qge_dn17, var_qge_dn18,)
    }
};
        var_qge = assign34520_e49595;
        var_qge_dn0 = assign34520_e49595_d_n0;
        var_qge_dn2 = assign34520_e49595_d_n2;
        var_qge_dn6 = assign34520_e49595_d_n6;
        var_qge_dn7 = assign34520_e49595_d_n7;
        var_qge_dn10 = assign34520_e49595_d_n10;
        var_qge_dn11 = assign34520_e49595_d_n11;
        var_qge_dn12 = assign34520_e49595_d_n12;
        var_qge_dn13 = assign34520_e49595_d_n13;
        var_qge_dn15 = assign34520_e49595_d_n15;
        var_qge_dn16 = assign34520_e49595_d_n16;
        var_qge_dn17 = assign34520_e49595_d_n17;
        var_qge_dn18 = assign34520_e49595_d_n18;

        let (assign34530_e49612, assign34530_e49612_d_n0, assign34530_e49612_d_n2, assign34530_e49612_d_n6, assign34530_e49612_d_n7, assign34530_e49612_d_n10, assign34530_e49612_d_n11, assign34530_e49612_d_n12, assign34530_e49612_d_n13, assign34530_e49612_d_n15, assign34530_e49612_d_n16, assign34530_e49612_d_n17, assign34530_e49612_d_n18,) = {
    if ((var_guard1137 != 0.0) && (var_guard1138 != 0.0)) {
        let assign34530_e49602: f64 = (-var_qgod);
        let assign34530_e49604: f64 = (assign34530_e49602 + var_qy);
        let assign34530_e49606: f64 = (assign34530_e49604 + var_qbdld);
        let assign34530_e49608: f64 = (assign34530_e49606 + var_q_bt_de);
        let assign34530_e49609: f64 = (var_mfactor * assign34530_e49608);
        let assign34530_e49610: f64 = (var_qde + assign34530_e49609);
        (assign34530_e49610, (var_qde_dn0 + (var_mfactor * ((((-var_qgod_dn0) + var_qy_dn0) + var_qbdld_dn0) + var_q_bt_de_dn0))), (var_qde_dn2 + (var_mfactor * ((((-var_qgod_dn2) + var_qy_dn2) + var_qbdld_dn2) + var_q_bt_de_dn2))), (var_qde_dn6 + (var_mfactor * ((((-var_qgod_dn6) + var_qy_dn6) + var_qbdld_dn6) + var_q_bt_de_dn6))), (var_qde_dn7 + (var_mfactor * ((((-var_qgod_dn7) + var_qy_dn7) + var_qbdld_dn7) + var_q_bt_de_dn7))), (var_qde_dn10 + (var_mfactor * ((((-var_qgod_dn10) + var_qy_dn10) + var_qbdld_dn10) + var_q_bt_de_dn10))), (var_qde_dn11 + (var_mfactor * ((((-var_qgod_dn11) + var_qy_dn11) + var_qbdld_dn11) + var_q_bt_de_dn11))), (var_qde_dn12 + (var_mfactor * ((((-var_qgod_dn12) + var_qy_dn12) + var_qbdld_dn12) + var_q_bt_de_dn12))), var_qde_dn13, var_qde_dn15, var_qde_dn16, (var_qde_dn17 + (var_mfactor * ((((-var_qgod_dn17) + var_qy_dn17) + var_qbdld_dn17) + var_q_bt_de_dn17))), var_qde_dn18,)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn6, var_qde_dn7, var_qde_dn10, var_qde_dn11, var_qde_dn12, var_qde_dn13, var_qde_dn15, var_qde_dn16, var_qde_dn17, var_qde_dn18,)
    }
};
        var_qde = assign34530_e49612;
        var_qde_dn0 = assign34530_e49612_d_n0;
        var_qde_dn2 = assign34530_e49612_d_n2;
        var_qde_dn6 = assign34530_e49612_d_n6;
        var_qde_dn7 = assign34530_e49612_d_n7;
        var_qde_dn10 = assign34530_e49612_d_n10;
        var_qde_dn11 = assign34530_e49612_d_n11;
        var_qde_dn12 = assign34530_e49612_d_n12;
        var_qde_dn13 = assign34530_e49612_d_n13;
        var_qde_dn15 = assign34530_e49612_d_n15;
        var_qde_dn16 = assign34530_e49612_d_n16;
        var_qde_dn17 = assign34530_e49612_d_n17;
        var_qde_dn18 = assign34530_e49612_d_n18;

        let (assign34540_e49627, assign34540_e49627_d_n0, assign34540_e49627_d_n2, assign34540_e49627_d_n6, assign34540_e49627_d_n7, assign34540_e49627_d_n10, assign34540_e49627_d_n11, assign34540_e49627_d_n12, assign34540_e49627_d_n13, assign34540_e49627_d_n15, assign34540_e49627_d_n16, assign34540_e49627_d_n17, assign34540_e49627_d_n18,) = {
    if ((var_guard1137 != 0.0) && (var_guard1138 != 0.0)) {
        let assign34540_e49619: f64 = (-var_qgos);
        let assign34540_e49621: f64 = (assign34540_e49619 + var_qbsld);
        let assign34540_e49623: f64 = (assign34540_e49621 + var_q_bt_se);
        let assign34540_e49624: f64 = (var_mfactor * assign34540_e49623);
        let assign34540_e49625: f64 = (var_qse + assign34540_e49624);
        (assign34540_e49625, (var_qse_dn0 + (var_mfactor * (((-var_qgos_dn0) + var_qbsld_dn0) + var_q_bt_se_dn0))), (var_qse_dn2 + (var_mfactor * (((-var_qgos_dn2) + var_qbsld_dn2) + var_q_bt_se_dn2))), (var_qse_dn6 + (var_mfactor * (((-var_qgos_dn6) + var_qbsld_dn6) + var_q_bt_se_dn6))), (var_qse_dn7 + (var_mfactor * (((-var_qgos_dn7) + var_qbsld_dn7) + var_q_bt_se_dn7))), (var_qse_dn10 + (var_mfactor * (((-var_qgos_dn10) + var_qbsld_dn10) + var_q_bt_se_dn10))), (var_qse_dn11 + (var_mfactor * (((-var_qgos_dn11) + var_qbsld_dn11) + var_q_bt_se_dn11))), (var_qse_dn12 + (var_mfactor * (((-var_qgos_dn12) + var_qbsld_dn12) + var_q_bt_se_dn12))), var_qse_dn13, var_qse_dn15, var_qse_dn16, (var_qse_dn17 + (var_mfactor * (((-var_qgos_dn17) + var_qbsld_dn17) + var_q_bt_se_dn17))), var_qse_dn18,)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn6, var_qse_dn7, var_qse_dn10, var_qse_dn11, var_qse_dn12, var_qse_dn13, var_qse_dn15, var_qse_dn16, var_qse_dn17, var_qse_dn18,)
    }
};
        var_qse = assign34540_e49627;
        var_qse_dn0 = assign34540_e49627_d_n0;
        var_qse_dn2 = assign34540_e49627_d_n2;
        var_qse_dn6 = assign34540_e49627_d_n6;
        var_qse_dn7 = assign34540_e49627_d_n7;
        var_qse_dn10 = assign34540_e49627_d_n10;
        var_qse_dn11 = assign34540_e49627_d_n11;
        var_qse_dn12 = assign34540_e49627_d_n12;
        var_qse_dn13 = assign34540_e49627_d_n13;
        var_qse_dn15 = assign34540_e49627_d_n15;
        var_qse_dn16 = assign34540_e49627_d_n16;
        var_qse_dn17 = assign34540_e49627_d_n17;
        var_qse_dn18 = assign34540_e49627_d_n18;

        *var_guard1128_slot = var_guard1128;
        *var_guard1134_slot = var_guard1134;
        *var_guard1135_slot = var_guard1135;
        *var_guard1136_slot = var_guard1136;
        *var_guard1137_slot = var_guard1137;
        *var_guard1138_slot = var_guard1138;
        *var_pslk_slot = var_pslk;
        *var_pslk_dn0_slot = var_pslk_dn0;
        *var_pslk_dn10_slot = var_pslk_dn10;
        *var_pslk_dn11_slot = var_pslk_dn11;
        *var_pslk_dn12_slot = var_pslk_dn12;
        *var_pslk_dn17_slot = var_pslk_dn17;
        *var_pslk_dn2_slot = var_pslk_dn2;
        *var_pslk_dn6_slot = var_pslk_dn6;
        *var_pslk_dn7_slot = var_pslk_dn7;
        *var_q_bt_de_slot = var_q_bt_de;
        *var_q_bt_de_dn0_slot = var_q_bt_de_dn0;
        *var_q_bt_de_dn10_slot = var_q_bt_de_dn10;
        *var_q_bt_de_dn11_slot = var_q_bt_de_dn11;
        *var_q_bt_de_dn12_slot = var_q_bt_de_dn12;
        *var_q_bt_de_dn17_slot = var_q_bt_de_dn17;
        *var_q_bt_de_dn2_slot = var_q_bt_de_dn2;
        *var_q_bt_de_dn6_slot = var_q_bt_de_dn6;
        *var_q_bt_de_dn7_slot = var_q_bt_de_dn7;
        *var_q_bt_ge_slot = var_q_bt_ge;
        *var_q_bt_ge_dn0_slot = var_q_bt_ge_dn0;
        *var_q_bt_ge_dn10_slot = var_q_bt_ge_dn10;
        *var_q_bt_ge_dn11_slot = var_q_bt_ge_dn11;
        *var_q_bt_ge_dn12_slot = var_q_bt_ge_dn12;
        *var_q_bt_ge_dn17_slot = var_q_bt_ge_dn17;
        *var_q_bt_ge_dn2_slot = var_q_bt_ge_dn2;
        *var_q_bt_ge_dn6_slot = var_q_bt_ge_dn6;
        *var_q_bt_ge_dn7_slot = var_q_bt_ge_dn7;
        *var_q_bt_se_slot = var_q_bt_se;
        *var_q_bt_se_dn0_slot = var_q_bt_se_dn0;
        *var_q_bt_se_dn10_slot = var_q_bt_se_dn10;
        *var_q_bt_se_dn11_slot = var_q_bt_se_dn11;
        *var_q_bt_se_dn12_slot = var_q_bt_se_dn12;
        *var_q_bt_se_dn17_slot = var_q_bt_se_dn17;
        *var_q_bt_se_dn2_slot = var_q_bt_se_dn2;
        *var_q_bt_se_dn6_slot = var_q_bt_se_dn6;
        *var_q_bt_se_dn7_slot = var_q_bt_se_dn7;
        *var_qde_slot = var_qde;
        *var_qde_dn0_slot = var_qde_dn0;
        *var_qde_dn10_slot = var_qde_dn10;
        *var_qde_dn11_slot = var_qde_dn11;
        *var_qde_dn12_slot = var_qde_dn12;
        *var_qde_dn13_slot = var_qde_dn13;
        *var_qde_dn15_slot = var_qde_dn15;
        *var_qde_dn16_slot = var_qde_dn16;
        *var_qde_dn17_slot = var_qde_dn17;
        *var_qde_dn18_slot = var_qde_dn18;
        *var_qde_dn2_slot = var_qde_dn2;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qde_dn7_slot = var_qde_dn7;
        *var_qge_slot = var_qge;
        *var_qge_dn0_slot = var_qge_dn0;
        *var_qge_dn10_slot = var_qge_dn10;
        *var_qge_dn11_slot = var_qge_dn11;
        *var_qge_dn12_slot = var_qge_dn12;
        *var_qge_dn13_slot = var_qge_dn13;
        *var_qge_dn15_slot = var_qge_dn15;
        *var_qge_dn16_slot = var_qge_dn16;
        *var_qge_dn17_slot = var_qge_dn17;
        *var_qge_dn18_slot = var_qge_dn18;
        *var_qge_dn2_slot = var_qge_dn2;
        *var_qge_dn6_slot = var_qge_dn6;
        *var_qge_dn7_slot = var_qge_dn7;
        *var_qs_qs_slot = var_qs_qs;
        *var_qs_qs_dn0_slot = var_qs_qs_dn0;
        *var_qs_qs_dn10_slot = var_qs_qs_dn10;
        *var_qs_qs_dn11_slot = var_qs_qs_dn11;
        *var_qs_qs_dn12_slot = var_qs_qs_dn12;
        *var_qs_qs_dn13_slot = var_qs_qs_dn13;
        *var_qs_qs_dn15_slot = var_qs_qs_dn15;
        *var_qs_qs_dn16_slot = var_qs_qs_dn16;
        *var_qs_qs_dn17_slot = var_qs_qs_dn17;
        *var_qs_qs_dn18_slot = var_qs_qs_dn18;
        *var_qs_qs_dn2_slot = var_qs_qs_dn2;
        *var_qs_qs_dn6_slot = var_qs_qs_dn6;
        *var_qs_qs_dn7_slot = var_qs_qs_dn7;
        *var_qse_slot = var_qse;
        *var_qse_dn0_slot = var_qse_dn0;
        *var_qse_dn10_slot = var_qse_dn10;
        *var_qse_dn11_slot = var_qse_dn11;
        *var_qse_dn12_slot = var_qse_dn12;
        *var_qse_dn13_slot = var_qse_dn13;
        *var_qse_dn15_slot = var_qse_dn15;
        *var_qse_dn16_slot = var_qse_dn16;
        *var_qse_dn17_slot = var_qse_dn17;
        *var_qse_dn18_slot = var_qse_dn18;
        *var_qse_dn2_slot = var_qse_dn2;
        *var_qse_dn6_slot = var_qse_dn6;
        *var_qse_dn7_slot = var_qse_dn7;
        *var_qy_slot = var_qy;
        *var_qy_dn0_slot = var_qy_dn0;
        *var_qy_dn10_slot = var_qy_dn10;
        *var_qy_dn11_slot = var_qy_dn11;
        *var_qy_dn12_slot = var_qy_dn12;
        *var_qy_dn17_slot = var_qy_dn17;
        *var_qy_dn2_slot = var_qy_dn2;
        *var_qy_dn6_slot = var_qy_dn6;
        *var_qy_dn7_slot = var_qy_dn7;
        *var_t10__blk1131_slot = var_t10__blk1131;
        *var_t10__blk1131_dn0_slot = var_t10__blk1131_dn0;
        *var_t10__blk1131_dn10_slot = var_t10__blk1131_dn10;
        *var_t10__blk1131_dn11_slot = var_t10__blk1131_dn11;
        *var_t10__blk1131_dn12_slot = var_t10__blk1131_dn12;
        *var_t10__blk1131_dn17_slot = var_t10__blk1131_dn17;
        *var_t10__blk1131_dn2_slot = var_t10__blk1131_dn2;
        *var_t10__blk1131_dn6_slot = var_t10__blk1131_dn6;
        *var_t10__blk1131_dn7_slot = var_t10__blk1131_dn7;
        *var_t1__blk1130_slot = var_t1__blk1130;
        *var_t1__blk1130_dn0_slot = var_t1__blk1130_dn0;
        *var_t1__blk1130_dn10_slot = var_t1__blk1130_dn10;
        *var_t1__blk1130_dn11_slot = var_t1__blk1130_dn11;
        *var_t1__blk1130_dn12_slot = var_t1__blk1130_dn12;
        *var_t1__blk1130_dn17_slot = var_t1__blk1130_dn17;
        *var_t1__blk1130_dn2_slot = var_t1__blk1130_dn2;
        *var_t1__blk1130_dn6_slot = var_t1__blk1130_dn6;
        *var_t1__blk1130_dn7_slot = var_t1__blk1130_dn7;
        *var_t2__blk1133_slot = var_t2__blk1133;
        *var_t2__blk1133_dn0_slot = var_t2__blk1133_dn0;
        *var_t2__blk1133_dn10_slot = var_t2__blk1133_dn10;
        *var_t2__blk1133_dn11_slot = var_t2__blk1133_dn11;
        *var_t2__blk1133_dn12_slot = var_t2__blk1133_dn12;
        *var_t2__blk1133_dn17_slot = var_t2__blk1133_dn17;
        *var_t2__blk1133_dn2_slot = var_t2__blk1133_dn2;
        *var_t2__blk1133_dn6_slot = var_t2__blk1133_dn6;
        *var_t2__blk1133_dn7_slot = var_t2__blk1133_dn7;
        *var_t3__blk1132_slot = var_t3__blk1132;
        *var_t3__blk1132_dn0_slot = var_t3__blk1132_dn0;
        *var_t3__blk1132_dn10_slot = var_t3__blk1132_dn10;
        *var_t3__blk1132_dn11_slot = var_t3__blk1132_dn11;
        *var_t3__blk1132_dn12_slot = var_t3__blk1132_dn12;
        *var_t3__blk1132_dn17_slot = var_t3__blk1132_dn17;
        *var_t3__blk1132_dn2_slot = var_t3__blk1132_dn2;
        *var_t3__blk1132_dn6_slot = var_t3__blk1132_dn6;
        *var_t3__blk1132_dn7_slot = var_t3__blk1132_dn7;
    }

    pub(super) fn stamp_transient_block_121(
        p: &Parameters,
        var_c_fox: f64,
        var_flg_ign: f64,
        var_flg_noqi: f64,
        var_glpart1: f64,
        var_guard1137: f64,
        var_guard1138: f64,
        var_ibd: f64,
        var_ibd_dn0: f64,
        var_ibd_dn10: f64,
        var_ibd_dn11: f64,
        var_ibd_dn12: f64,
        var_ibd_dn17: f64,
        var_ibd_dn2: f64,
        var_ibd_dn6: f64,
        var_ibd_dn7: f64,
        var_ibs: f64,
        var_ibs_dn0: f64,
        var_ibs_dn10: f64,
        var_ibs_dn11: f64,
        var_ibs_dn12: f64,
        var_ibs_dn17: f64,
        var_ibs_dn2: f64,
        var_ibs_dn6: f64,
        var_ibs_dn7: f64,
        var_igate: f64,
        var_igate_dn0: f64,
        var_igate_dn10: f64,
        var_igate_dn11: f64,
        var_igate_dn12: f64,
        var_igate_dn17: f64,
        var_igate_dn2: f64,
        var_igate_dn6: f64,
        var_igate_dn7: f64,
        var_igb: f64,
        var_igb_dn0: f64,
        var_igb_dn10: f64,
        var_igb_dn11: f64,
        var_igb_dn12: f64,
        var_igb_dn17: f64,
        var_igb_dn2: f64,
        var_igb_dn6: f64,
        var_igb_dn7: f64,
        var_igd: f64,
        var_igd_dn0: f64,
        var_igd_dn10: f64,
        var_igd_dn11: f64,
        var_igd_dn12: f64,
        var_igd_dn17: f64,
        var_igd_dn2: f64,
        var_igd_dn6: f64,
        var_igd_dn7: f64,
        var_igidl: f64,
        var_igidl_dn0: f64,
        var_igidl_dn10: f64,
        var_igidl_dn11: f64,
        var_igidl_dn12: f64,
        var_igidl_dn17: f64,
        var_igidl_dn2: f64,
        var_igidl_dn6: f64,
        var_igidl_dn7: f64,
        var_igisl: f64,
        var_igisl_dn0: f64,
        var_igisl_dn10: f64,
        var_igisl_dn11: f64,
        var_igisl_dn12: f64,
        var_igisl_dn17: f64,
        var_igisl_dn2: f64,
        var_igisl_dn6: f64,
        var_igisl_dn7: f64,
        var_igs: f64,
        var_igs_dn0: f64,
        var_igs_dn10: f64,
        var_igs_dn11: f64,
        var_igs_dn12: f64,
        var_igs_dn17: f64,
        var_igs_dn2: f64,
        var_igs_dn6: f64,
        var_igs_dn7: f64,
        var_isub: f64,
        var_isub_dn0: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn12: f64,
        var_isub_dn17: f64,
        var_isub_dn2: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_leff_cv: f64,
        var_mfactor: f64,
        var_mode: f64,
        var_nthrml: f64,
        var_nthrml_dn0: f64,
        var_nthrml_dn10: f64,
        var_nthrml_dn11: f64,
        var_nthrml_dn12: f64,
        var_nthrml_dn17: f64,
        var_nthrml_dn2: f64,
        var_nthrml_dn6: f64,
        var_nthrml_dn7: f64,
        var_qbd: f64,
        var_qbd_dn0: f64,
        var_qbd_dn10: f64,
        var_qbd_dn11: f64,
        var_qbd_dn12: f64,
        var_qbd_dn17: f64,
        var_qbd_dn2: f64,
        var_qbd_dn6: f64,
        var_qbd_dn7: f64,
        var_qbdld: f64,
        var_qbdld_dn0: f64,
        var_qbdld_dn10: f64,
        var_qbdld_dn11: f64,
        var_qbdld_dn12: f64,
        var_qbdld_dn17: f64,
        var_qbdld_dn2: f64,
        var_qbdld_dn6: f64,
        var_qbdld_dn7: f64,
        var_qbs: f64,
        var_qbs_dn0: f64,
        var_qbs_dn10: f64,
        var_qbs_dn11: f64,
        var_qbs_dn12: f64,
        var_qbs_dn17: f64,
        var_qbs_dn2: f64,
        var_qbs_dn6: f64,
        var_qbs_dn7: f64,
        var_qbsld: f64,
        var_qbsld_dn0: f64,
        var_qbsld_dn10: f64,
        var_qbsld_dn11: f64,
        var_qbsld_dn12: f64,
        var_qbsld_dn17: f64,
        var_qbsld_dn2: f64,
        var_qbsld_dn6: f64,
        var_qbsld_dn7: f64,
        var_qgob: f64,
        var_qgob_dn0: f64,
        var_qgob_dn10: f64,
        var_qgob_dn11: f64,
        var_qgob_dn12: f64,
        var_qgob_dn17: f64,
        var_qgob_dn2: f64,
        var_qgob_dn6: f64,
        var_qgob_dn7: f64,
        var_qgod: f64,
        var_qgod_dn0: f64,
        var_qgod_dn10: f64,
        var_qgod_dn11: f64,
        var_qgod_dn12: f64,
        var_qgod_dn17: f64,
        var_qgod_dn2: f64,
        var_qgod_dn6: f64,
        var_qgod_dn7: f64,
        var_qgos: f64,
        var_qgos_dn0: f64,
        var_qgos_dn10: f64,
        var_qgos_dn11: f64,
        var_qgos_dn12: f64,
        var_qgos_dn17: f64,
        var_qgos_dn2: f64,
        var_qgos_dn6: f64,
        var_qgos_dn7: f64,
        var_qovd: f64,
        var_qovd_dn0: f64,
        var_qovd_dn10: f64,
        var_qovd_dn11: f64,
        var_qovd_dn12: f64,
        var_qovd_dn17: f64,
        var_qovd_dn2: f64,
        var_qovd_dn6: f64,
        var_qovd_dn7: f64,
        var_qovs: f64,
        var_qovs_dn0: f64,
        var_qovs_dn10: f64,
        var_qovs_dn11: f64,
        var_qovs_dn12: f64,
        var_qovs_dn17: f64,
        var_qovs_dn2: f64,
        var_qovs_dn6: f64,
        var_qovs_dn7: f64,
        var_qy: f64,
        var_qy_dn0: f64,
        var_qy_dn10: f64,
        var_qy_dn11: f64,
        var_qy_dn12: f64,
        var_qy_dn17: f64,
        var_qy_dn2: f64,
        var_qy_dn6: f64,
        var_qy_dn7: f64,
        var_weffcv_nf: f64,
        var_cgdbd_slot: &mut f64,
        var_cgdbd_dn0_slot: &mut f64,
        var_cgdbd_dn10_slot: &mut f64,
        var_cgdbd_dn11_slot: &mut f64,
        var_cgdbd_dn12_slot: &mut f64,
        var_cgdbd_dn13_slot: &mut f64,
        var_cgdbd_dn15_slot: &mut f64,
        var_cgdbd_dn16_slot: &mut f64,
        var_cgdbd_dn17_slot: &mut f64,
        var_cgdbd_dn18_slot: &mut f64,
        var_cgdbd_dn2_slot: &mut f64,
        var_cgdbd_dn6_slot: &mut f64,
        var_cgdbd_dn7_slot: &mut f64,
        var_cgsb_slot: &mut f64,
        var_cgsb_dn0_slot: &mut f64,
        var_cgsb_dn10_slot: &mut f64,
        var_cgsb_dn11_slot: &mut f64,
        var_cgsb_dn12_slot: &mut f64,
        var_cgsb_dn13_slot: &mut f64,
        var_cgsb_dn15_slot: &mut f64,
        var_cgsb_dn16_slot: &mut f64,
        var_cgsb_dn17_slot: &mut f64,
        var_cgsb_dn18_slot: &mut f64,
        var_cgsb_dn2_slot: &mut f64,
        var_cgsb_dn6_slot: &mut f64,
        var_cgsb_dn7_slot: &mut f64,
        var_cgsbd_slot: &mut f64,
        var_cgsbd_dn0_slot: &mut f64,
        var_cgsbd_dn10_slot: &mut f64,
        var_cgsbd_dn11_slot: &mut f64,
        var_cgsbd_dn12_slot: &mut f64,
        var_cgsbd_dn13_slot: &mut f64,
        var_cgsbd_dn15_slot: &mut f64,
        var_cgsbd_dn16_slot: &mut f64,
        var_cgsbd_dn17_slot: &mut f64,
        var_cgsbd_dn18_slot: &mut f64,
        var_cgsbd_dn2_slot: &mut f64,
        var_cgsbd_dn6_slot: &mut f64,
        var_cgsbd_dn7_slot: &mut f64,
        var_guard1139_slot: &mut f64,
        var_guard1140_slot: &mut f64,
        var_guard1141_slot: &mut f64,
        var_guard1142_slot: &mut f64,
        var_guard1149_slot: &mut f64,
        var_ibdb_slot: &mut f64,
        var_ibdb_dn0_slot: &mut f64,
        var_ibdb_dn10_slot: &mut f64,
        var_ibdb_dn11_slot: &mut f64,
        var_ibdb_dn12_slot: &mut f64,
        var_ibdb_dn17_slot: &mut f64,
        var_ibdb_dn2_slot: &mut f64,
        var_ibdb_dn6_slot: &mut f64,
        var_ibdb_dn7_slot: &mut f64,
        var_ibsb_slot: &mut f64,
        var_ibsb_dn0_slot: &mut f64,
        var_ibsb_dn10_slot: &mut f64,
        var_ibsb_dn11_slot: &mut f64,
        var_ibsb_dn12_slot: &mut f64,
        var_ibsb_dn17_slot: &mut f64,
        var_ibsb_dn2_slot: &mut f64,
        var_ibsb_dn6_slot: &mut f64,
        var_ibsb_dn7_slot: &mut f64,
        var_igbe_slot: &mut f64,
        var_igbe_dn0_slot: &mut f64,
        var_igbe_dn10_slot: &mut f64,
        var_igbe_dn11_slot: &mut f64,
        var_igbe_dn12_slot: &mut f64,
        var_igbe_dn17_slot: &mut f64,
        var_igbe_dn2_slot: &mut f64,
        var_igbe_dn6_slot: &mut f64,
        var_igbe_dn7_slot: &mut f64,
        var_igde_slot: &mut f64,
        var_igde_dn0_slot: &mut f64,
        var_igde_dn10_slot: &mut f64,
        var_igde_dn11_slot: &mut f64,
        var_igde_dn12_slot: &mut f64,
        var_igde_dn17_slot: &mut f64,
        var_igde_dn2_slot: &mut f64,
        var_igde_dn6_slot: &mut f64,
        var_igde_dn7_slot: &mut f64,
        var_igidle_slot: &mut f64,
        var_igidle_dn0_slot: &mut f64,
        var_igidle_dn10_slot: &mut f64,
        var_igidle_dn11_slot: &mut f64,
        var_igidle_dn12_slot: &mut f64,
        var_igidle_dn17_slot: &mut f64,
        var_igidle_dn2_slot: &mut f64,
        var_igidle_dn6_slot: &mut f64,
        var_igidle_dn7_slot: &mut f64,
        var_igisle_slot: &mut f64,
        var_igisle_dn0_slot: &mut f64,
        var_igisle_dn10_slot: &mut f64,
        var_igisle_dn11_slot: &mut f64,
        var_igisle_dn12_slot: &mut f64,
        var_igisle_dn17_slot: &mut f64,
        var_igisle_dn2_slot: &mut f64,
        var_igisle_dn6_slot: &mut f64,
        var_igisle_dn7_slot: &mut f64,
        var_igse_slot: &mut f64,
        var_igse_dn0_slot: &mut f64,
        var_igse_dn10_slot: &mut f64,
        var_igse_dn11_slot: &mut f64,
        var_igse_dn12_slot: &mut f64,
        var_igse_dn17_slot: &mut f64,
        var_igse_dn2_slot: &mut f64,
        var_igse_dn6_slot: &mut f64,
        var_igse_dn7_slot: &mut f64,
        var_isube_slot: &mut f64,
        var_isube_dn0_slot: &mut f64,
        var_isube_dn10_slot: &mut f64,
        var_isube_dn11_slot: &mut f64,
        var_isube_dn12_slot: &mut f64,
        var_isube_dn17_slot: &mut f64,
        var_isube_dn2_slot: &mut f64,
        var_isube_dn6_slot: &mut f64,
        var_isube_dn7_slot: &mut f64,
        var_noithrml_slot: &mut f64,
        var_noithrml_dn0_slot: &mut f64,
        var_noithrml_dn10_slot: &mut f64,
        var_noithrml_dn11_slot: &mut f64,
        var_noithrml_dn12_slot: &mut f64,
        var_noithrml_dn17_slot: &mut f64,
        var_noithrml_dn2_slot: &mut f64,
        var_noithrml_dn6_slot: &mut f64,
        var_noithrml_dn7_slot: &mut f64,
        var_qbd_s0_slot: &mut f64,
        var_qbd_s0_dn0_slot: &mut f64,
        var_qbd_s0_dn10_slot: &mut f64,
        var_qbd_s0_dn11_slot: &mut f64,
        var_qbd_s0_dn12_slot: &mut f64,
        var_qbd_s0_dn17_slot: &mut f64,
        var_qbd_s0_dn2_slot: &mut f64,
        var_qbd_s0_dn6_slot: &mut f64,
        var_qbd_s0_dn7_slot: &mut f64,
        var_qbs_s0_slot: &mut f64,
        var_qbs_s0_dn0_slot: &mut f64,
        var_qbs_s0_dn10_slot: &mut f64,
        var_qbs_s0_dn11_slot: &mut f64,
        var_qbs_s0_dn12_slot: &mut f64,
        var_qbs_s0_dn17_slot: &mut f64,
        var_qbs_s0_dn2_slot: &mut f64,
        var_qbs_s0_dn6_slot: &mut f64,
        var_qbs_s0_dn7_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_dn0_slot: &mut f64,
        var_qde_dn10_slot: &mut f64,
        var_qde_dn11_slot: &mut f64,
        var_qde_dn12_slot: &mut f64,
        var_qde_dn13_slot: &mut f64,
        var_qde_dn15_slot: &mut f64,
        var_qde_dn16_slot: &mut f64,
        var_qde_dn17_slot: &mut f64,
        var_qde_dn18_slot: &mut f64,
        var_qde_dn2_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qde_dn7_slot: &mut f64,
        var_qge_slot: &mut f64,
        var_qge_dn0_slot: &mut f64,
        var_qge_dn10_slot: &mut f64,
        var_qge_dn11_slot: &mut f64,
        var_qge_dn12_slot: &mut f64,
        var_qge_dn13_slot: &mut f64,
        var_qge_dn15_slot: &mut f64,
        var_qge_dn16_slot: &mut f64,
        var_qge_dn17_slot: &mut f64,
        var_qge_dn18_slot: &mut f64,
        var_qge_dn2_slot: &mut f64,
        var_qge_dn6_slot: &mut f64,
        var_qge_dn7_slot: &mut f64,
        var_qse_slot: &mut f64,
        var_qse_dn0_slot: &mut f64,
        var_qse_dn10_slot: &mut f64,
        var_qse_dn11_slot: &mut f64,
        var_qse_dn12_slot: &mut f64,
        var_qse_dn13_slot: &mut f64,
        var_qse_dn15_slot: &mut f64,
        var_qse_dn16_slot: &mut f64,
        var_qse_dn17_slot: &mut f64,
        var_qse_dn18_slot: &mut f64,
        var_qse_dn2_slot: &mut f64,
        var_qse_dn6_slot: &mut f64,
        var_qse_dn7_slot: &mut f64,
        var_t0__blk1143_slot: &mut f64,
    ) {
        let mut var_cgdbd: f64 = *var_cgdbd_slot;
        let mut var_cgdbd_dn0: f64 = *var_cgdbd_dn0_slot;
        let mut var_cgdbd_dn10: f64 = *var_cgdbd_dn10_slot;
        let mut var_cgdbd_dn11: f64 = *var_cgdbd_dn11_slot;
        let mut var_cgdbd_dn12: f64 = *var_cgdbd_dn12_slot;
        let mut var_cgdbd_dn13: f64 = *var_cgdbd_dn13_slot;
        let mut var_cgdbd_dn15: f64 = *var_cgdbd_dn15_slot;
        let mut var_cgdbd_dn16: f64 = *var_cgdbd_dn16_slot;
        let mut var_cgdbd_dn17: f64 = *var_cgdbd_dn17_slot;
        let mut var_cgdbd_dn18: f64 = *var_cgdbd_dn18_slot;
        let mut var_cgdbd_dn2: f64 = *var_cgdbd_dn2_slot;
        let mut var_cgdbd_dn6: f64 = *var_cgdbd_dn6_slot;
        let mut var_cgdbd_dn7: f64 = *var_cgdbd_dn7_slot;
        let mut var_cgsb: f64 = *var_cgsb_slot;
        let mut var_cgsb_dn0: f64 = *var_cgsb_dn0_slot;
        let mut var_cgsb_dn10: f64 = *var_cgsb_dn10_slot;
        let mut var_cgsb_dn11: f64 = *var_cgsb_dn11_slot;
        let mut var_cgsb_dn12: f64 = *var_cgsb_dn12_slot;
        let mut var_cgsb_dn13: f64 = *var_cgsb_dn13_slot;
        let mut var_cgsb_dn15: f64 = *var_cgsb_dn15_slot;
        let mut var_cgsb_dn16: f64 = *var_cgsb_dn16_slot;
        let mut var_cgsb_dn17: f64 = *var_cgsb_dn17_slot;
        let mut var_cgsb_dn18: f64 = *var_cgsb_dn18_slot;
        let mut var_cgsb_dn2: f64 = *var_cgsb_dn2_slot;
        let mut var_cgsb_dn6: f64 = *var_cgsb_dn6_slot;
        let mut var_cgsb_dn7: f64 = *var_cgsb_dn7_slot;
        let mut var_cgsbd: f64 = *var_cgsbd_slot;
        let mut var_cgsbd_dn0: f64 = *var_cgsbd_dn0_slot;
        let mut var_cgsbd_dn10: f64 = *var_cgsbd_dn10_slot;
        let mut var_cgsbd_dn11: f64 = *var_cgsbd_dn11_slot;
        let mut var_cgsbd_dn12: f64 = *var_cgsbd_dn12_slot;
        let mut var_cgsbd_dn13: f64 = *var_cgsbd_dn13_slot;
        let mut var_cgsbd_dn15: f64 = *var_cgsbd_dn15_slot;
        let mut var_cgsbd_dn16: f64 = *var_cgsbd_dn16_slot;
        let mut var_cgsbd_dn17: f64 = *var_cgsbd_dn17_slot;
        let mut var_cgsbd_dn18: f64 = *var_cgsbd_dn18_slot;
        let mut var_cgsbd_dn2: f64 = *var_cgsbd_dn2_slot;
        let mut var_cgsbd_dn6: f64 = *var_cgsbd_dn6_slot;
        let mut var_cgsbd_dn7: f64 = *var_cgsbd_dn7_slot;
        let mut var_guard1139: f64 = *var_guard1139_slot;
        let mut var_guard1140: f64 = *var_guard1140_slot;
        let mut var_guard1141: f64 = *var_guard1141_slot;
        let mut var_guard1142: f64 = *var_guard1142_slot;
        let mut var_guard1149: f64 = *var_guard1149_slot;
        let mut var_ibdb: f64 = *var_ibdb_slot;
        let mut var_ibdb_dn0: f64 = *var_ibdb_dn0_slot;
        let mut var_ibdb_dn10: f64 = *var_ibdb_dn10_slot;
        let mut var_ibdb_dn11: f64 = *var_ibdb_dn11_slot;
        let mut var_ibdb_dn12: f64 = *var_ibdb_dn12_slot;
        let mut var_ibdb_dn17: f64 = *var_ibdb_dn17_slot;
        let mut var_ibdb_dn2: f64 = *var_ibdb_dn2_slot;
        let mut var_ibdb_dn6: f64 = *var_ibdb_dn6_slot;
        let mut var_ibdb_dn7: f64 = *var_ibdb_dn7_slot;
        let mut var_ibsb: f64 = *var_ibsb_slot;
        let mut var_ibsb_dn0: f64 = *var_ibsb_dn0_slot;
        let mut var_ibsb_dn10: f64 = *var_ibsb_dn10_slot;
        let mut var_ibsb_dn11: f64 = *var_ibsb_dn11_slot;
        let mut var_ibsb_dn12: f64 = *var_ibsb_dn12_slot;
        let mut var_ibsb_dn17: f64 = *var_ibsb_dn17_slot;
        let mut var_ibsb_dn2: f64 = *var_ibsb_dn2_slot;
        let mut var_ibsb_dn6: f64 = *var_ibsb_dn6_slot;
        let mut var_ibsb_dn7: f64 = *var_ibsb_dn7_slot;
        let mut var_igbe: f64 = *var_igbe_slot;
        let mut var_igbe_dn0: f64 = *var_igbe_dn0_slot;
        let mut var_igbe_dn10: f64 = *var_igbe_dn10_slot;
        let mut var_igbe_dn11: f64 = *var_igbe_dn11_slot;
        let mut var_igbe_dn12: f64 = *var_igbe_dn12_slot;
        let mut var_igbe_dn17: f64 = *var_igbe_dn17_slot;
        let mut var_igbe_dn2: f64 = *var_igbe_dn2_slot;
        let mut var_igbe_dn6: f64 = *var_igbe_dn6_slot;
        let mut var_igbe_dn7: f64 = *var_igbe_dn7_slot;
        let mut var_igde: f64 = *var_igde_slot;
        let mut var_igde_dn0: f64 = *var_igde_dn0_slot;
        let mut var_igde_dn10: f64 = *var_igde_dn10_slot;
        let mut var_igde_dn11: f64 = *var_igde_dn11_slot;
        let mut var_igde_dn12: f64 = *var_igde_dn12_slot;
        let mut var_igde_dn17: f64 = *var_igde_dn17_slot;
        let mut var_igde_dn2: f64 = *var_igde_dn2_slot;
        let mut var_igde_dn6: f64 = *var_igde_dn6_slot;
        let mut var_igde_dn7: f64 = *var_igde_dn7_slot;
        let mut var_igidle: f64 = *var_igidle_slot;
        let mut var_igidle_dn0: f64 = *var_igidle_dn0_slot;
        let mut var_igidle_dn10: f64 = *var_igidle_dn10_slot;
        let mut var_igidle_dn11: f64 = *var_igidle_dn11_slot;
        let mut var_igidle_dn12: f64 = *var_igidle_dn12_slot;
        let mut var_igidle_dn17: f64 = *var_igidle_dn17_slot;
        let mut var_igidle_dn2: f64 = *var_igidle_dn2_slot;
        let mut var_igidle_dn6: f64 = *var_igidle_dn6_slot;
        let mut var_igidle_dn7: f64 = *var_igidle_dn7_slot;
        let mut var_igisle: f64 = *var_igisle_slot;
        let mut var_igisle_dn0: f64 = *var_igisle_dn0_slot;
        let mut var_igisle_dn10: f64 = *var_igisle_dn10_slot;
        let mut var_igisle_dn11: f64 = *var_igisle_dn11_slot;
        let mut var_igisle_dn12: f64 = *var_igisle_dn12_slot;
        let mut var_igisle_dn17: f64 = *var_igisle_dn17_slot;
        let mut var_igisle_dn2: f64 = *var_igisle_dn2_slot;
        let mut var_igisle_dn6: f64 = *var_igisle_dn6_slot;
        let mut var_igisle_dn7: f64 = *var_igisle_dn7_slot;
        let mut var_igse: f64 = *var_igse_slot;
        let mut var_igse_dn0: f64 = *var_igse_dn0_slot;
        let mut var_igse_dn10: f64 = *var_igse_dn10_slot;
        let mut var_igse_dn11: f64 = *var_igse_dn11_slot;
        let mut var_igse_dn12: f64 = *var_igse_dn12_slot;
        let mut var_igse_dn17: f64 = *var_igse_dn17_slot;
        let mut var_igse_dn2: f64 = *var_igse_dn2_slot;
        let mut var_igse_dn6: f64 = *var_igse_dn6_slot;
        let mut var_igse_dn7: f64 = *var_igse_dn7_slot;
        let mut var_isube: f64 = *var_isube_slot;
        let mut var_isube_dn0: f64 = *var_isube_dn0_slot;
        let mut var_isube_dn10: f64 = *var_isube_dn10_slot;
        let mut var_isube_dn11: f64 = *var_isube_dn11_slot;
        let mut var_isube_dn12: f64 = *var_isube_dn12_slot;
        let mut var_isube_dn17: f64 = *var_isube_dn17_slot;
        let mut var_isube_dn2: f64 = *var_isube_dn2_slot;
        let mut var_isube_dn6: f64 = *var_isube_dn6_slot;
        let mut var_isube_dn7: f64 = *var_isube_dn7_slot;
        let mut var_noithrml: f64 = *var_noithrml_slot;
        let mut var_noithrml_dn0: f64 = *var_noithrml_dn0_slot;
        let mut var_noithrml_dn10: f64 = *var_noithrml_dn10_slot;
        let mut var_noithrml_dn11: f64 = *var_noithrml_dn11_slot;
        let mut var_noithrml_dn12: f64 = *var_noithrml_dn12_slot;
        let mut var_noithrml_dn17: f64 = *var_noithrml_dn17_slot;
        let mut var_noithrml_dn2: f64 = *var_noithrml_dn2_slot;
        let mut var_noithrml_dn6: f64 = *var_noithrml_dn6_slot;
        let mut var_noithrml_dn7: f64 = *var_noithrml_dn7_slot;
        let mut var_qbd_s0: f64 = *var_qbd_s0_slot;
        let mut var_qbd_s0_dn0: f64 = *var_qbd_s0_dn0_slot;
        let mut var_qbd_s0_dn10: f64 = *var_qbd_s0_dn10_slot;
        let mut var_qbd_s0_dn11: f64 = *var_qbd_s0_dn11_slot;
        let mut var_qbd_s0_dn12: f64 = *var_qbd_s0_dn12_slot;
        let mut var_qbd_s0_dn17: f64 = *var_qbd_s0_dn17_slot;
        let mut var_qbd_s0_dn2: f64 = *var_qbd_s0_dn2_slot;
        let mut var_qbd_s0_dn6: f64 = *var_qbd_s0_dn6_slot;
        let mut var_qbd_s0_dn7: f64 = *var_qbd_s0_dn7_slot;
        let mut var_qbs_s0: f64 = *var_qbs_s0_slot;
        let mut var_qbs_s0_dn0: f64 = *var_qbs_s0_dn0_slot;
        let mut var_qbs_s0_dn10: f64 = *var_qbs_s0_dn10_slot;
        let mut var_qbs_s0_dn11: f64 = *var_qbs_s0_dn11_slot;
        let mut var_qbs_s0_dn12: f64 = *var_qbs_s0_dn12_slot;
        let mut var_qbs_s0_dn17: f64 = *var_qbs_s0_dn17_slot;
        let mut var_qbs_s0_dn2: f64 = *var_qbs_s0_dn2_slot;
        let mut var_qbs_s0_dn6: f64 = *var_qbs_s0_dn6_slot;
        let mut var_qbs_s0_dn7: f64 = *var_qbs_s0_dn7_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_dn0: f64 = *var_qde_dn0_slot;
        let mut var_qde_dn10: f64 = *var_qde_dn10_slot;
        let mut var_qde_dn11: f64 = *var_qde_dn11_slot;
        let mut var_qde_dn12: f64 = *var_qde_dn12_slot;
        let mut var_qde_dn13: f64 = *var_qde_dn13_slot;
        let mut var_qde_dn15: f64 = *var_qde_dn15_slot;
        let mut var_qde_dn16: f64 = *var_qde_dn16_slot;
        let mut var_qde_dn17: f64 = *var_qde_dn17_slot;
        let mut var_qde_dn18: f64 = *var_qde_dn18_slot;
        let mut var_qde_dn2: f64 = *var_qde_dn2_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qde_dn7: f64 = *var_qde_dn7_slot;
        let mut var_qge: f64 = *var_qge_slot;
        let mut var_qge_dn0: f64 = *var_qge_dn0_slot;
        let mut var_qge_dn10: f64 = *var_qge_dn10_slot;
        let mut var_qge_dn11: f64 = *var_qge_dn11_slot;
        let mut var_qge_dn12: f64 = *var_qge_dn12_slot;
        let mut var_qge_dn13: f64 = *var_qge_dn13_slot;
        let mut var_qge_dn15: f64 = *var_qge_dn15_slot;
        let mut var_qge_dn16: f64 = *var_qge_dn16_slot;
        let mut var_qge_dn17: f64 = *var_qge_dn17_slot;
        let mut var_qge_dn18: f64 = *var_qge_dn18_slot;
        let mut var_qge_dn2: f64 = *var_qge_dn2_slot;
        let mut var_qge_dn6: f64 = *var_qge_dn6_slot;
        let mut var_qge_dn7: f64 = *var_qge_dn7_slot;
        let mut var_qse: f64 = *var_qse_slot;
        let mut var_qse_dn0: f64 = *var_qse_dn0_slot;
        let mut var_qse_dn10: f64 = *var_qse_dn10_slot;
        let mut var_qse_dn11: f64 = *var_qse_dn11_slot;
        let mut var_qse_dn12: f64 = *var_qse_dn12_slot;
        let mut var_qse_dn13: f64 = *var_qse_dn13_slot;
        let mut var_qse_dn15: f64 = *var_qse_dn15_slot;
        let mut var_qse_dn16: f64 = *var_qse_dn16_slot;
        let mut var_qse_dn17: f64 = *var_qse_dn17_slot;
        let mut var_qse_dn18: f64 = *var_qse_dn18_slot;
        let mut var_qse_dn2: f64 = *var_qse_dn2_slot;
        let mut var_qse_dn6: f64 = *var_qse_dn6_slot;
        let mut var_qse_dn7: f64 = *var_qse_dn7_slot;
        let mut var_t0__blk1143: f64 = *var_t0__blk1143_slot;

        let (assign34550_e49648, assign34550_e49648_d_n0, assign34550_e49648_d_n2, assign34550_e49648_d_n6, assign34550_e49648_d_n7, assign34550_e49648_d_n10, assign34550_e49648_d_n11, assign34550_e49648_d_n12, assign34550_e49648_d_n13, assign34550_e49648_d_n15, assign34550_e49648_d_n16, assign34550_e49648_d_n17, assign34550_e49648_d_n18,) = {
    if ((var_guard1137 != 0.0) && (var_guard1138 == 0.0)) {
        let assign34550_e49636: f64 = (var_qgod + var_qgos);
        let assign34550_e49638: f64 = (assign34550_e49636 + var_qgob);
        let assign34550_e49640: f64 = (assign34550_e49638 - var_qy);
        let assign34550_e49642: f64 = (assign34550_e49640 - var_qovs);
        let assign34550_e49644: f64 = (assign34550_e49642 - var_qovd);
        let assign34550_e49645: f64 = (var_mfactor * assign34550_e49644);
        let assign34550_e49646: f64 = (var_qge + assign34550_e49645);
        (assign34550_e49646, (var_qge_dn0 + (var_mfactor * (((((var_qgod_dn0 + var_qgos_dn0) + var_qgob_dn0) - var_qy_dn0) - var_qovs_dn0) - var_qovd_dn0))), (var_qge_dn2 + (var_mfactor * (((((var_qgod_dn2 + var_qgos_dn2) + var_qgob_dn2) - var_qy_dn2) - var_qovs_dn2) - var_qovd_dn2))), (var_qge_dn6 + (var_mfactor * (((((var_qgod_dn6 + var_qgos_dn6) + var_qgob_dn6) - var_qy_dn6) - var_qovs_dn6) - var_qovd_dn6))), (var_qge_dn7 + (var_mfactor * (((((var_qgod_dn7 + var_qgos_dn7) + var_qgob_dn7) - var_qy_dn7) - var_qovs_dn7) - var_qovd_dn7))), (var_qge_dn10 + (var_mfactor * (((((var_qgod_dn10 + var_qgos_dn10) + var_qgob_dn10) - var_qy_dn10) - var_qovs_dn10) - var_qovd_dn10))), (var_qge_dn11 + (var_mfactor * (((((var_qgod_dn11 + var_qgos_dn11) + var_qgob_dn11) - var_qy_dn11) - var_qovs_dn11) - var_qovd_dn11))), (var_qge_dn12 + (var_mfactor * (((((var_qgod_dn12 + var_qgos_dn12) + var_qgob_dn12) - var_qy_dn12) - var_qovs_dn12) - var_qovd_dn12))), var_qge_dn13, var_qge_dn15, var_qge_dn16, (var_qge_dn17 + (var_mfactor * (((((var_qgod_dn17 + var_qgos_dn17) + var_qgob_dn17) - var_qy_dn17) - var_qovs_dn17) - var_qovd_dn17))), var_qge_dn18,)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn6, var_qge_dn7, var_qge_dn10, var_qge_dn11, var_qge_dn12, var_qge_dn13, var_qge_dn15, var_qge_dn16, var_qge_dn17, var_qge_dn18,)
    }
};
        var_qge = assign34550_e49648;
        var_qge_dn0 = assign34550_e49648_d_n0;
        var_qge_dn2 = assign34550_e49648_d_n2;
        var_qge_dn6 = assign34550_e49648_d_n6;
        var_qge_dn7 = assign34550_e49648_d_n7;
        var_qge_dn10 = assign34550_e49648_d_n10;
        var_qge_dn11 = assign34550_e49648_d_n11;
        var_qge_dn12 = assign34550_e49648_d_n12;
        var_qge_dn13 = assign34550_e49648_d_n13;
        var_qge_dn15 = assign34550_e49648_d_n15;
        var_qge_dn16 = assign34550_e49648_d_n16;
        var_qge_dn17 = assign34550_e49648_d_n17;
        var_qge_dn18 = assign34550_e49648_d_n18;

        let (assign34560_e49664, assign34560_e49664_d_n0, assign34560_e49664_d_n2, assign34560_e49664_d_n6, assign34560_e49664_d_n7, assign34560_e49664_d_n10, assign34560_e49664_d_n11, assign34560_e49664_d_n12, assign34560_e49664_d_n13, assign34560_e49664_d_n15, assign34560_e49664_d_n16, assign34560_e49664_d_n17, assign34560_e49664_d_n18,) = {
    if ((var_guard1137 != 0.0) && (var_guard1138 == 0.0)) {
        let assign34560_e49656: f64 = (-var_qgod);
        let assign34560_e49658: f64 = (assign34560_e49656 + var_qy);
        let assign34560_e49660: f64 = (assign34560_e49658 + var_qbdld);
        let assign34560_e49661: f64 = (var_mfactor * assign34560_e49660);
        let assign34560_e49662: f64 = (var_qde + assign34560_e49661);
        (assign34560_e49662, (var_qde_dn0 + (var_mfactor * (((-var_qgod_dn0) + var_qy_dn0) + var_qbdld_dn0))), (var_qde_dn2 + (var_mfactor * (((-var_qgod_dn2) + var_qy_dn2) + var_qbdld_dn2))), (var_qde_dn6 + (var_mfactor * (((-var_qgod_dn6) + var_qy_dn6) + var_qbdld_dn6))), (var_qde_dn7 + (var_mfactor * (((-var_qgod_dn7) + var_qy_dn7) + var_qbdld_dn7))), (var_qde_dn10 + (var_mfactor * (((-var_qgod_dn10) + var_qy_dn10) + var_qbdld_dn10))), (var_qde_dn11 + (var_mfactor * (((-var_qgod_dn11) + var_qy_dn11) + var_qbdld_dn11))), (var_qde_dn12 + (var_mfactor * (((-var_qgod_dn12) + var_qy_dn12) + var_qbdld_dn12))), var_qde_dn13, var_qde_dn15, var_qde_dn16, (var_qde_dn17 + (var_mfactor * (((-var_qgod_dn17) + var_qy_dn17) + var_qbdld_dn17))), var_qde_dn18,)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn6, var_qde_dn7, var_qde_dn10, var_qde_dn11, var_qde_dn12, var_qde_dn13, var_qde_dn15, var_qde_dn16, var_qde_dn17, var_qde_dn18,)
    }
};
        var_qde = assign34560_e49664;
        var_qde_dn0 = assign34560_e49664_d_n0;
        var_qde_dn2 = assign34560_e49664_d_n2;
        var_qde_dn6 = assign34560_e49664_d_n6;
        var_qde_dn7 = assign34560_e49664_d_n7;
        var_qde_dn10 = assign34560_e49664_d_n10;
        var_qde_dn11 = assign34560_e49664_d_n11;
        var_qde_dn12 = assign34560_e49664_d_n12;
        var_qde_dn13 = assign34560_e49664_d_n13;
        var_qde_dn15 = assign34560_e49664_d_n15;
        var_qde_dn16 = assign34560_e49664_d_n16;
        var_qde_dn17 = assign34560_e49664_d_n17;
        var_qde_dn18 = assign34560_e49664_d_n18;

        let (assign34570_e49678, assign34570_e49678_d_n0, assign34570_e49678_d_n2, assign34570_e49678_d_n6, assign34570_e49678_d_n7, assign34570_e49678_d_n10, assign34570_e49678_d_n11, assign34570_e49678_d_n12, assign34570_e49678_d_n13, assign34570_e49678_d_n15, assign34570_e49678_d_n16, assign34570_e49678_d_n17, assign34570_e49678_d_n18,) = {
    if ((var_guard1137 != 0.0) && (var_guard1138 == 0.0)) {
        let assign34570_e49672: f64 = (-var_qgos);
        let assign34570_e49674: f64 = (assign34570_e49672 + var_qbsld);
        let assign34570_e49675: f64 = (var_mfactor * assign34570_e49674);
        let assign34570_e49676: f64 = (var_qse + assign34570_e49675);
        (assign34570_e49676, (var_qse_dn0 + (var_mfactor * ((-var_qgos_dn0) + var_qbsld_dn0))), (var_qse_dn2 + (var_mfactor * ((-var_qgos_dn2) + var_qbsld_dn2))), (var_qse_dn6 + (var_mfactor * ((-var_qgos_dn6) + var_qbsld_dn6))), (var_qse_dn7 + (var_mfactor * ((-var_qgos_dn7) + var_qbsld_dn7))), (var_qse_dn10 + (var_mfactor * ((-var_qgos_dn10) + var_qbsld_dn10))), (var_qse_dn11 + (var_mfactor * ((-var_qgos_dn11) + var_qbsld_dn11))), (var_qse_dn12 + (var_mfactor * ((-var_qgos_dn12) + var_qbsld_dn12))), var_qse_dn13, var_qse_dn15, var_qse_dn16, (var_qse_dn17 + (var_mfactor * ((-var_qgos_dn17) + var_qbsld_dn17))), var_qse_dn18,)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn6, var_qse_dn7, var_qse_dn10, var_qse_dn11, var_qse_dn12, var_qse_dn13, var_qse_dn15, var_qse_dn16, var_qse_dn17, var_qse_dn18,)
    }
};
        var_qse = assign34570_e49678;
        var_qse_dn0 = assign34570_e49678_d_n0;
        var_qse_dn2 = assign34570_e49678_d_n2;
        var_qse_dn6 = assign34570_e49678_d_n6;
        var_qse_dn7 = assign34570_e49678_d_n7;
        var_qse_dn10 = assign34570_e49678_d_n10;
        var_qse_dn11 = assign34570_e49678_d_n11;
        var_qse_dn12 = assign34570_e49678_d_n12;
        var_qse_dn13 = assign34570_e49678_d_n13;
        var_qse_dn15 = assign34570_e49678_d_n15;
        var_qse_dn16 = assign34570_e49678_d_n16;
        var_qse_dn17 = assign34570_e49678_d_n17;
        var_qse_dn18 = assign34570_e49678_d_n18;

        let assign34600_e49683: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1139 = assign34600_e49683;

        let (assign34610_e49689, assign34610_e49689_d_n0, assign34610_e49689_d_n2, assign34610_e49689_d_n6, assign34610_e49689_d_n7, assign34610_e49689_d_n10, assign34610_e49689_d_n11, assign34610_e49689_d_n12, assign34610_e49689_d_n17,) = {
    if (var_guard1139 != 0.0) {
        let assign34610_e49687: f64 = (var_mfactor * var_ibs);
        (assign34610_e49687, (var_mfactor * var_ibs_dn0), (var_mfactor * var_ibs_dn2), (var_mfactor * var_ibs_dn6), (var_mfactor * var_ibs_dn7), (var_mfactor * var_ibs_dn10), (var_mfactor * var_ibs_dn11), (var_mfactor * var_ibs_dn12), (var_mfactor * var_ibs_dn17),)
    } else {
        (var_ibsb, var_ibsb_dn0, var_ibsb_dn2, var_ibsb_dn6, var_ibsb_dn7, var_ibsb_dn10, var_ibsb_dn11, var_ibsb_dn12, var_ibsb_dn17,)
    }
};
        var_ibsb = assign34610_e49689;
        var_ibsb_dn0 = assign34610_e49689_d_n0;
        var_ibsb_dn2 = assign34610_e49689_d_n2;
        var_ibsb_dn6 = assign34610_e49689_d_n6;
        var_ibsb_dn7 = assign34610_e49689_d_n7;
        var_ibsb_dn10 = assign34610_e49689_d_n10;
        var_ibsb_dn11 = assign34610_e49689_d_n11;
        var_ibsb_dn12 = assign34610_e49689_d_n12;
        var_ibsb_dn17 = assign34610_e49689_d_n17;

        let (assign34620_e49695, assign34620_e49695_d_n0, assign34620_e49695_d_n2, assign34620_e49695_d_n6, assign34620_e49695_d_n7, assign34620_e49695_d_n10, assign34620_e49695_d_n11, assign34620_e49695_d_n12, assign34620_e49695_d_n17,) = {
    if (var_guard1139 != 0.0) {
        let assign34620_e49693: f64 = (var_mfactor * var_ibd);
        (assign34620_e49693, (var_mfactor * var_ibd_dn0), (var_mfactor * var_ibd_dn2), (var_mfactor * var_ibd_dn6), (var_mfactor * var_ibd_dn7), (var_mfactor * var_ibd_dn10), (var_mfactor * var_ibd_dn11), (var_mfactor * var_ibd_dn12), (var_mfactor * var_ibd_dn17),)
    } else {
        (var_ibdb, var_ibdb_dn0, var_ibdb_dn2, var_ibdb_dn6, var_ibdb_dn7, var_ibdb_dn10, var_ibdb_dn11, var_ibdb_dn12, var_ibdb_dn17,)
    }
};
        var_ibdb = assign34620_e49695;
        var_ibdb_dn0 = assign34620_e49695_d_n0;
        var_ibdb_dn2 = assign34620_e49695_d_n2;
        var_ibdb_dn6 = assign34620_e49695_d_n6;
        var_ibdb_dn7 = assign34620_e49695_d_n7;
        var_ibdb_dn10 = assign34620_e49695_d_n10;
        var_ibdb_dn11 = assign34620_e49695_d_n11;
        var_ibdb_dn12 = assign34620_e49695_d_n12;
        var_ibdb_dn17 = assign34620_e49695_d_n17;

        let (assign34630_e49701, assign34630_e49701_d_n0, assign34630_e49701_d_n2, assign34630_e49701_d_n6, assign34630_e49701_d_n7, assign34630_e49701_d_n10, assign34630_e49701_d_n11, assign34630_e49701_d_n12, assign34630_e49701_d_n17,) = {
    if (var_guard1139 != 0.0) {
        let assign34630_e49699: f64 = (var_mfactor * var_qbd);
        (assign34630_e49699, (var_mfactor * var_qbd_dn0), (var_mfactor * var_qbd_dn2), (var_mfactor * var_qbd_dn6), (var_mfactor * var_qbd_dn7), (var_mfactor * var_qbd_dn10), (var_mfactor * var_qbd_dn11), (var_mfactor * var_qbd_dn12), (var_mfactor * var_qbd_dn17),)
    } else {
        (var_qbd_s0, var_qbd_s0_dn0, var_qbd_s0_dn2, var_qbd_s0_dn6, var_qbd_s0_dn7, var_qbd_s0_dn10, var_qbd_s0_dn11, var_qbd_s0_dn12, var_qbd_s0_dn17,)
    }
};
        var_qbd_s0 = assign34630_e49701;
        var_qbd_s0_dn0 = assign34630_e49701_d_n0;
        var_qbd_s0_dn2 = assign34630_e49701_d_n2;
        var_qbd_s0_dn6 = assign34630_e49701_d_n6;
        var_qbd_s0_dn7 = assign34630_e49701_d_n7;
        var_qbd_s0_dn10 = assign34630_e49701_d_n10;
        var_qbd_s0_dn11 = assign34630_e49701_d_n11;
        var_qbd_s0_dn12 = assign34630_e49701_d_n12;
        var_qbd_s0_dn17 = assign34630_e49701_d_n17;

        let (assign34640_e49707, assign34640_e49707_d_n0, assign34640_e49707_d_n2, assign34640_e49707_d_n6, assign34640_e49707_d_n7, assign34640_e49707_d_n10, assign34640_e49707_d_n11, assign34640_e49707_d_n12, assign34640_e49707_d_n17,) = {
    if (var_guard1139 != 0.0) {
        let assign34640_e49705: f64 = (var_mfactor * var_qbs);
        (assign34640_e49705, (var_mfactor * var_qbs_dn0), (var_mfactor * var_qbs_dn2), (var_mfactor * var_qbs_dn6), (var_mfactor * var_qbs_dn7), (var_mfactor * var_qbs_dn10), (var_mfactor * var_qbs_dn11), (var_mfactor * var_qbs_dn12), (var_mfactor * var_qbs_dn17),)
    } else {
        (var_qbs_s0, var_qbs_s0_dn0, var_qbs_s0_dn2, var_qbs_s0_dn6, var_qbs_s0_dn7, var_qbs_s0_dn10, var_qbs_s0_dn11, var_qbs_s0_dn12, var_qbs_s0_dn17,)
    }
};
        var_qbs_s0 = assign34640_e49707;
        var_qbs_s0_dn0 = assign34640_e49707_d_n0;
        var_qbs_s0_dn2 = assign34640_e49707_d_n2;
        var_qbs_s0_dn6 = assign34640_e49707_d_n6;
        var_qbs_s0_dn7 = assign34640_e49707_d_n7;
        var_qbs_s0_dn10 = assign34640_e49707_d_n10;
        var_qbs_s0_dn11 = assign34640_e49707_d_n11;
        var_qbs_s0_dn12 = assign34640_e49707_d_n12;
        var_qbs_s0_dn17 = assign34640_e49707_d_n17;

        let (assign34650_e49712, assign34650_e49712_d_n0, assign34650_e49712_d_n2, assign34650_e49712_d_n6, assign34650_e49712_d_n7, assign34650_e49712_d_n10, assign34650_e49712_d_n11, assign34650_e49712_d_n12, assign34650_e49712_d_n17,) = {
    if (var_guard1139 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibsb, var_ibsb_dn0, var_ibsb_dn2, var_ibsb_dn6, var_ibsb_dn7, var_ibsb_dn10, var_ibsb_dn11, var_ibsb_dn12, var_ibsb_dn17,)
    }
};
        var_ibsb = assign34650_e49712;
        var_ibsb_dn0 = assign34650_e49712_d_n0;
        var_ibsb_dn2 = assign34650_e49712_d_n2;
        var_ibsb_dn6 = assign34650_e49712_d_n6;
        var_ibsb_dn7 = assign34650_e49712_d_n7;
        var_ibsb_dn10 = assign34650_e49712_d_n10;
        var_ibsb_dn11 = assign34650_e49712_d_n11;
        var_ibsb_dn12 = assign34650_e49712_d_n12;
        var_ibsb_dn17 = assign34650_e49712_d_n17;

        let (assign34660_e49717, assign34660_e49717_d_n0, assign34660_e49717_d_n2, assign34660_e49717_d_n6, assign34660_e49717_d_n7, assign34660_e49717_d_n10, assign34660_e49717_d_n11, assign34660_e49717_d_n12, assign34660_e49717_d_n17,) = {
    if (var_guard1139 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibdb, var_ibdb_dn0, var_ibdb_dn2, var_ibdb_dn6, var_ibdb_dn7, var_ibdb_dn10, var_ibdb_dn11, var_ibdb_dn12, var_ibdb_dn17,)
    }
};
        var_ibdb = assign34660_e49717;
        var_ibdb_dn0 = assign34660_e49717_d_n0;
        var_ibdb_dn2 = assign34660_e49717_d_n2;
        var_ibdb_dn6 = assign34660_e49717_d_n6;
        var_ibdb_dn7 = assign34660_e49717_d_n7;
        var_ibdb_dn10 = assign34660_e49717_d_n10;
        var_ibdb_dn11 = assign34660_e49717_d_n11;
        var_ibdb_dn12 = assign34660_e49717_d_n12;
        var_ibdb_dn17 = assign34660_e49717_d_n17;

        let (assign34670_e49722, assign34670_e49722_d_n0, assign34670_e49722_d_n2, assign34670_e49722_d_n6, assign34670_e49722_d_n7, assign34670_e49722_d_n10, assign34670_e49722_d_n11, assign34670_e49722_d_n12, assign34670_e49722_d_n17,) = {
    if (var_guard1139 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbd_s0, var_qbd_s0_dn0, var_qbd_s0_dn2, var_qbd_s0_dn6, var_qbd_s0_dn7, var_qbd_s0_dn10, var_qbd_s0_dn11, var_qbd_s0_dn12, var_qbd_s0_dn17,)
    }
};
        var_qbd_s0 = assign34670_e49722;
        var_qbd_s0_dn0 = assign34670_e49722_d_n0;
        var_qbd_s0_dn2 = assign34670_e49722_d_n2;
        var_qbd_s0_dn6 = assign34670_e49722_d_n6;
        var_qbd_s0_dn7 = assign34670_e49722_d_n7;
        var_qbd_s0_dn10 = assign34670_e49722_d_n10;
        var_qbd_s0_dn11 = assign34670_e49722_d_n11;
        var_qbd_s0_dn12 = assign34670_e49722_d_n12;
        var_qbd_s0_dn17 = assign34670_e49722_d_n17;

        let (assign34680_e49727, assign34680_e49727_d_n0, assign34680_e49727_d_n2, assign34680_e49727_d_n6, assign34680_e49727_d_n7, assign34680_e49727_d_n10, assign34680_e49727_d_n11, assign34680_e49727_d_n12, assign34680_e49727_d_n17,) = {
    if (var_guard1139 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbs_s0, var_qbs_s0_dn0, var_qbs_s0_dn2, var_qbs_s0_dn6, var_qbs_s0_dn7, var_qbs_s0_dn10, var_qbs_s0_dn11, var_qbs_s0_dn12, var_qbs_s0_dn17,)
    }
};
        var_qbs_s0 = assign34680_e49727;
        var_qbs_s0_dn0 = assign34680_e49727_d_n0;
        var_qbs_s0_dn2 = assign34680_e49727_d_n2;
        var_qbs_s0_dn6 = assign34680_e49727_d_n6;
        var_qbs_s0_dn7 = assign34680_e49727_d_n7;
        var_qbs_s0_dn10 = assign34680_e49727_d_n10;
        var_qbs_s0_dn11 = assign34680_e49727_d_n11;
        var_qbs_s0_dn12 = assign34680_e49727_d_n12;
        var_qbs_s0_dn17 = assign34680_e49727_d_n17;

        let assign34690_e49730: f64 = if p.p25 != 1.0 { 1.0 } else { 0.0 };
        var_guard1140 = assign34690_e49730;

        let (assign34700_e49734, assign34700_e49734_d_n0, assign34700_e49734_d_n2, assign34700_e49734_d_n6, assign34700_e49734_d_n7, assign34700_e49734_d_n10, assign34700_e49734_d_n11, assign34700_e49734_d_n12, assign34700_e49734_d_n17,) = {
    if (var_guard1140 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn6, var_isube_dn7, var_isube_dn10, var_isube_dn11, var_isube_dn12, var_isube_dn17,)
    }
};
        var_isube = assign34700_e49734;
        var_isube_dn0 = assign34700_e49734_d_n0;
        var_isube_dn2 = assign34700_e49734_d_n2;
        var_isube_dn6 = assign34700_e49734_d_n6;
        var_isube_dn7 = assign34700_e49734_d_n7;
        var_isube_dn10 = assign34700_e49734_d_n10;
        var_isube_dn11 = assign34700_e49734_d_n11;
        var_isube_dn12 = assign34700_e49734_d_n12;
        var_isube_dn17 = assign34700_e49734_d_n17;

        let (assign34710_e49741, assign34710_e49741_d_n0, assign34710_e49741_d_n2, assign34710_e49741_d_n6, assign34710_e49741_d_n7, assign34710_e49741_d_n10, assign34710_e49741_d_n11, assign34710_e49741_d_n12, assign34710_e49741_d_n17,) = {
    if (var_guard1140 == 0.0) {
        let assign34710_e49739: f64 = (var_mfactor * var_isub);
        (assign34710_e49739, (var_mfactor * var_isub_dn0), (var_mfactor * var_isub_dn2), (var_mfactor * var_isub_dn6), (var_mfactor * var_isub_dn7), (var_mfactor * var_isub_dn10), (var_mfactor * var_isub_dn11), (var_mfactor * var_isub_dn12), (var_mfactor * var_isub_dn17),)
    } else {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn6, var_isube_dn7, var_isube_dn10, var_isube_dn11, var_isube_dn12, var_isube_dn17,)
    }
};
        var_isube = assign34710_e49741;
        var_isube_dn0 = assign34710_e49741_d_n0;
        var_isube_dn2 = assign34710_e49741_d_n2;
        var_isube_dn6 = assign34710_e49741_d_n6;
        var_isube_dn7 = assign34710_e49741_d_n7;
        var_isube_dn10 = assign34710_e49741_d_n10;
        var_isube_dn11 = assign34710_e49741_d_n11;
        var_isube_dn12 = assign34710_e49741_d_n12;
        var_isube_dn17 = assign34710_e49741_d_n17;

        let assign34720_e49744: f64 = (-var_igb);
        let assign34720_e49745: f64 = (var_mfactor * assign34720_e49744);
        var_igbe = assign34720_e49745;
        var_igbe_dn0 = (var_mfactor * (-var_igb_dn0));
        var_igbe_dn2 = (var_mfactor * (-var_igb_dn2));
        var_igbe_dn6 = (var_mfactor * (-var_igb_dn6));
        var_igbe_dn7 = (var_mfactor * (-var_igb_dn7));
        var_igbe_dn10 = (var_mfactor * (-var_igb_dn10));
        var_igbe_dn11 = (var_mfactor * (-var_igb_dn11));
        var_igbe_dn12 = (var_mfactor * (-var_igb_dn12));
        var_igbe_dn17 = (var_mfactor * (-var_igb_dn17));

        let assign34730_e49748: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard1141 = assign34730_e49748;

        let (assign34740_e49758, assign34740_e49758_d_n0, assign34740_e49758_d_n2, assign34740_e49758_d_n6, assign34740_e49758_d_n7, assign34740_e49758_d_n10, assign34740_e49758_d_n11, assign34740_e49758_d_n12, assign34740_e49758_d_n17,) = {
    if (var_guard1141 != 0.0) {
        let assign34740_e49753: f64 = (var_glpart1 * var_igate);
        let assign34740_e49755: f64 = (assign34740_e49753 - var_igd);
        let assign34740_e49756: f64 = (var_mfactor * assign34740_e49755);
        (assign34740_e49756, (var_mfactor * ((var_glpart1 * var_igate_dn0) - var_igd_dn0)), (var_mfactor * ((var_glpart1 * var_igate_dn2) - var_igd_dn2)), (var_mfactor * ((var_glpart1 * var_igate_dn6) - var_igd_dn6)), (var_mfactor * ((var_glpart1 * var_igate_dn7) - var_igd_dn7)), (var_mfactor * ((var_glpart1 * var_igate_dn10) - var_igd_dn10)), (var_mfactor * ((var_glpart1 * var_igate_dn11) - var_igd_dn11)), (var_mfactor * ((var_glpart1 * var_igate_dn12) - var_igd_dn12)), (var_mfactor * ((var_glpart1 * var_igate_dn17) - var_igd_dn17)),)
    } else {
        (var_igde, var_igde_dn0, var_igde_dn2, var_igde_dn6, var_igde_dn7, var_igde_dn10, var_igde_dn11, var_igde_dn12, var_igde_dn17,)
    }
};
        var_igde = assign34740_e49758;
        var_igde_dn0 = assign34740_e49758_d_n0;
        var_igde_dn2 = assign34740_e49758_d_n2;
        var_igde_dn6 = assign34740_e49758_d_n6;
        var_igde_dn7 = assign34740_e49758_d_n7;
        var_igde_dn10 = assign34740_e49758_d_n10;
        var_igde_dn11 = assign34740_e49758_d_n11;
        var_igde_dn12 = assign34740_e49758_d_n12;
        var_igde_dn17 = assign34740_e49758_d_n17;

        let (assign34750_e49771, assign34750_e49771_d_n0, assign34750_e49771_d_n2, assign34750_e49771_d_n6, assign34750_e49771_d_n7, assign34750_e49771_d_n10, assign34750_e49771_d_n11, assign34750_e49771_d_n12, assign34750_e49771_d_n17,) = {
    if (var_guard1141 == 0.0) {
        let assign34750_e49764: f64 = (1.0 - var_glpart1);
        let assign34750_e49766: f64 = (assign34750_e49764 * var_igate);
        let assign34750_e49768: f64 = (assign34750_e49766 - var_igs);
        let assign34750_e49769: f64 = (var_mfactor * assign34750_e49768);
        (assign34750_e49769, (var_mfactor * ((assign34750_e49764 * var_igate_dn0) - var_igs_dn0)), (var_mfactor * ((assign34750_e49764 * var_igate_dn2) - var_igs_dn2)), (var_mfactor * ((assign34750_e49764 * var_igate_dn6) - var_igs_dn6)), (var_mfactor * ((assign34750_e49764 * var_igate_dn7) - var_igs_dn7)), (var_mfactor * ((assign34750_e49764 * var_igate_dn10) - var_igs_dn10)), (var_mfactor * ((assign34750_e49764 * var_igate_dn11) - var_igs_dn11)), (var_mfactor * ((assign34750_e49764 * var_igate_dn12) - var_igs_dn12)), (var_mfactor * ((assign34750_e49764 * var_igate_dn17) - var_igs_dn17)),)
    } else {
        (var_igde, var_igde_dn0, var_igde_dn2, var_igde_dn6, var_igde_dn7, var_igde_dn10, var_igde_dn11, var_igde_dn12, var_igde_dn17,)
    }
};
        var_igde = assign34750_e49771;
        var_igde_dn0 = assign34750_e49771_d_n0;
        var_igde_dn2 = assign34750_e49771_d_n2;
        var_igde_dn6 = assign34750_e49771_d_n6;
        var_igde_dn7 = assign34750_e49771_d_n7;
        var_igde_dn10 = assign34750_e49771_d_n10;
        var_igde_dn11 = assign34750_e49771_d_n11;
        var_igde_dn12 = assign34750_e49771_d_n12;
        var_igde_dn17 = assign34750_e49771_d_n17;

        let assign34760_e49774: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard1142 = assign34760_e49774;

        let (assign34770_e49786, assign34770_e49786_d_n0, assign34770_e49786_d_n2, assign34770_e49786_d_n6, assign34770_e49786_d_n7, assign34770_e49786_d_n10, assign34770_e49786_d_n11, assign34770_e49786_d_n12, assign34770_e49786_d_n17,) = {
    if (var_guard1142 != 0.0) {
        let assign34770_e49779: f64 = (1.0 - var_glpart1);
        let assign34770_e49781: f64 = (assign34770_e49779 * var_igate);
        let assign34770_e49783: f64 = (assign34770_e49781 - var_igs);
        let assign34770_e49784: f64 = (var_mfactor * assign34770_e49783);
        (assign34770_e49784, (var_mfactor * ((assign34770_e49779 * var_igate_dn0) - var_igs_dn0)), (var_mfactor * ((assign34770_e49779 * var_igate_dn2) - var_igs_dn2)), (var_mfactor * ((assign34770_e49779 * var_igate_dn6) - var_igs_dn6)), (var_mfactor * ((assign34770_e49779 * var_igate_dn7) - var_igs_dn7)), (var_mfactor * ((assign34770_e49779 * var_igate_dn10) - var_igs_dn10)), (var_mfactor * ((assign34770_e49779 * var_igate_dn11) - var_igs_dn11)), (var_mfactor * ((assign34770_e49779 * var_igate_dn12) - var_igs_dn12)), (var_mfactor * ((assign34770_e49779 * var_igate_dn17) - var_igs_dn17)),)
    } else {
        (var_igse, var_igse_dn0, var_igse_dn2, var_igse_dn6, var_igse_dn7, var_igse_dn10, var_igse_dn11, var_igse_dn12, var_igse_dn17,)
    }
};
        var_igse = assign34770_e49786;
        var_igse_dn0 = assign34770_e49786_d_n0;
        var_igse_dn2 = assign34770_e49786_d_n2;
        var_igse_dn6 = assign34770_e49786_d_n6;
        var_igse_dn7 = assign34770_e49786_d_n7;
        var_igse_dn10 = assign34770_e49786_d_n10;
        var_igse_dn11 = assign34770_e49786_d_n11;
        var_igse_dn12 = assign34770_e49786_d_n12;
        var_igse_dn17 = assign34770_e49786_d_n17;

        let (assign34780_e49797, assign34780_e49797_d_n0, assign34780_e49797_d_n2, assign34780_e49797_d_n6, assign34780_e49797_d_n7, assign34780_e49797_d_n10, assign34780_e49797_d_n11, assign34780_e49797_d_n12, assign34780_e49797_d_n17,) = {
    if (var_guard1142 == 0.0) {
        let assign34780_e49792: f64 = (var_glpart1 * var_igate);
        let assign34780_e49794: f64 = (assign34780_e49792 - var_igd);
        let assign34780_e49795: f64 = (var_mfactor * assign34780_e49794);
        (assign34780_e49795, (var_mfactor * ((var_glpart1 * var_igate_dn0) - var_igd_dn0)), (var_mfactor * ((var_glpart1 * var_igate_dn2) - var_igd_dn2)), (var_mfactor * ((var_glpart1 * var_igate_dn6) - var_igd_dn6)), (var_mfactor * ((var_glpart1 * var_igate_dn7) - var_igd_dn7)), (var_mfactor * ((var_glpart1 * var_igate_dn10) - var_igd_dn10)), (var_mfactor * ((var_glpart1 * var_igate_dn11) - var_igd_dn11)), (var_mfactor * ((var_glpart1 * var_igate_dn12) - var_igd_dn12)), (var_mfactor * ((var_glpart1 * var_igate_dn17) - var_igd_dn17)),)
    } else {
        (var_igse, var_igse_dn0, var_igse_dn2, var_igse_dn6, var_igse_dn7, var_igse_dn10, var_igse_dn11, var_igse_dn12, var_igse_dn17,)
    }
};
        var_igse = assign34780_e49797;
        var_igse_dn0 = assign34780_e49797_d_n0;
        var_igse_dn2 = assign34780_e49797_d_n2;
        var_igse_dn6 = assign34780_e49797_d_n6;
        var_igse_dn7 = assign34780_e49797_d_n7;
        var_igse_dn10 = assign34780_e49797_d_n10;
        var_igse_dn11 = assign34780_e49797_d_n11;
        var_igse_dn12 = assign34780_e49797_d_n12;
        var_igse_dn17 = assign34780_e49797_d_n17;

        let (assign34790_e49807, assign34790_e49807_d_n0, assign34790_e49807_d_n2, assign34790_e49807_d_n6, assign34790_e49807_d_n7, assign34790_e49807_d_n10, assign34790_e49807_d_n11, assign34790_e49807_d_n12, assign34790_e49807_d_n17,) = {
    if (var_mode == 1.0) {
        let assign34790_e49803: f64 = (var_mfactor * var_igidl);
        (assign34790_e49803, (var_mfactor * var_igidl_dn0), (var_mfactor * var_igidl_dn2), (var_mfactor * var_igidl_dn6), (var_mfactor * var_igidl_dn7), (var_mfactor * var_igidl_dn10), (var_mfactor * var_igidl_dn11), (var_mfactor * var_igidl_dn12), (var_mfactor * var_igidl_dn17),)
    } else {
        let assign34790_e49806: f64 = (var_mfactor * var_igisl);
        (assign34790_e49806, (var_mfactor * var_igisl_dn0), (var_mfactor * var_igisl_dn2), (var_mfactor * var_igisl_dn6), (var_mfactor * var_igisl_dn7), (var_mfactor * var_igisl_dn10), (var_mfactor * var_igisl_dn11), (var_mfactor * var_igisl_dn12), (var_mfactor * var_igisl_dn17),)
    }
};
        var_igidle = assign34790_e49807;
        var_igidle_dn0 = assign34790_e49807_d_n0;
        var_igidle_dn2 = assign34790_e49807_d_n2;
        var_igidle_dn6 = assign34790_e49807_d_n6;
        var_igidle_dn7 = assign34790_e49807_d_n7;
        var_igidle_dn10 = assign34790_e49807_d_n10;
        var_igidle_dn11 = assign34790_e49807_d_n11;
        var_igidle_dn12 = assign34790_e49807_d_n12;
        var_igidle_dn17 = assign34790_e49807_d_n17;

        let (assign34800_e49817, assign34800_e49817_d_n0, assign34800_e49817_d_n2, assign34800_e49817_d_n6, assign34800_e49817_d_n7, assign34800_e49817_d_n10, assign34800_e49817_d_n11, assign34800_e49817_d_n12, assign34800_e49817_d_n17,) = {
    if (var_mode == 1.0) {
        let assign34800_e49813: f64 = (var_mfactor * var_igisl);
        (assign34800_e49813, (var_mfactor * var_igisl_dn0), (var_mfactor * var_igisl_dn2), (var_mfactor * var_igisl_dn6), (var_mfactor * var_igisl_dn7), (var_mfactor * var_igisl_dn10), (var_mfactor * var_igisl_dn11), (var_mfactor * var_igisl_dn12), (var_mfactor * var_igisl_dn17),)
    } else {
        let assign34800_e49816: f64 = (var_mfactor * var_igidl);
        (assign34800_e49816, (var_mfactor * var_igidl_dn0), (var_mfactor * var_igidl_dn2), (var_mfactor * var_igidl_dn6), (var_mfactor * var_igidl_dn7), (var_mfactor * var_igidl_dn10), (var_mfactor * var_igidl_dn11), (var_mfactor * var_igidl_dn12), (var_mfactor * var_igidl_dn17),)
    }
};
        var_igisle = assign34800_e49817;
        var_igisle_dn0 = assign34800_e49817_d_n0;
        var_igisle_dn2 = assign34800_e49817_d_n2;
        var_igisle_dn6 = assign34800_e49817_d_n6;
        var_igisle_dn7 = assign34800_e49817_d_n7;
        var_igisle_dn10 = assign34800_e49817_d_n10;
        var_igisle_dn11 = assign34800_e49817_d_n11;
        var_igisle_dn12 = assign34800_e49817_d_n12;
        var_igisle_dn17 = assign34800_e49817_d_n17;

        let assign34820_e49823: f64 = (var_mfactor * var_nthrml);
        var_noithrml = assign34820_e49823;
        var_noithrml_dn0 = (var_mfactor * var_nthrml_dn0);
        var_noithrml_dn2 = (var_mfactor * var_nthrml_dn2);
        var_noithrml_dn6 = (var_mfactor * var_nthrml_dn6);
        var_noithrml_dn7 = (var_mfactor * var_nthrml_dn7);
        var_noithrml_dn10 = (var_mfactor * var_nthrml_dn10);
        var_noithrml_dn11 = (var_mfactor * var_nthrml_dn11);
        var_noithrml_dn12 = (var_mfactor * var_nthrml_dn12);
        var_noithrml_dn17 = (var_mfactor * var_nthrml_dn17);

        let assign34830_e49826: f64 = var_qge_dn6;
        var_cgdbd = assign34830_e49826;
        var_cgdbd_dn0 = 0.0;
        var_cgdbd_dn2 = 0.0;
        var_cgdbd_dn6 = 0.0;
        var_cgdbd_dn7 = 0.0;
        var_cgdbd_dn10 = 0.0;
        var_cgdbd_dn11 = 0.0;
        var_cgdbd_dn12 = 0.0;
        var_cgdbd_dn13 = 0.0;
        var_cgdbd_dn15 = 0.0;
        var_cgdbd_dn16 = 0.0;
        var_cgdbd_dn17 = 0.0;
        var_cgdbd_dn18 = 0.0;

        let assign34840_e49829: f64 = (p.p50 * var_cgdbd);
        var_cgdbd = assign34840_e49829;
        var_cgdbd_dn0 = (p.p50 * var_cgdbd_dn0);
        var_cgdbd_dn2 = (p.p50 * var_cgdbd_dn2);
        var_cgdbd_dn6 = (p.p50 * var_cgdbd_dn6);
        var_cgdbd_dn7 = (p.p50 * var_cgdbd_dn7);
        var_cgdbd_dn10 = (p.p50 * var_cgdbd_dn10);
        var_cgdbd_dn11 = (p.p50 * var_cgdbd_dn11);
        var_cgdbd_dn12 = (p.p50 * var_cgdbd_dn12);
        var_cgdbd_dn13 = (p.p50 * var_cgdbd_dn13);
        var_cgdbd_dn15 = (p.p50 * var_cgdbd_dn15);
        var_cgdbd_dn16 = (p.p50 * var_cgdbd_dn16);
        var_cgdbd_dn17 = (p.p50 * var_cgdbd_dn17);
        var_cgdbd_dn18 = (p.p50 * var_cgdbd_dn18);

        let assign34850_e49832: f64 = var_qge_dn7;
        var_cgsbd = assign34850_e49832;
        var_cgsbd_dn0 = 0.0;
        var_cgsbd_dn2 = 0.0;
        var_cgsbd_dn6 = 0.0;
        var_cgsbd_dn7 = 0.0;
        var_cgsbd_dn10 = 0.0;
        var_cgsbd_dn11 = 0.0;
        var_cgsbd_dn12 = 0.0;
        var_cgsbd_dn13 = 0.0;
        var_cgsbd_dn15 = 0.0;
        var_cgsbd_dn16 = 0.0;
        var_cgsbd_dn17 = 0.0;
        var_cgsbd_dn18 = 0.0;

        let assign34860_e49835: f64 = (p.p50 * var_cgsbd);
        var_cgsbd = assign34860_e49835;
        var_cgsbd_dn0 = (p.p50 * var_cgsbd_dn0);
        var_cgsbd_dn2 = (p.p50 * var_cgsbd_dn2);
        var_cgsbd_dn6 = (p.p50 * var_cgsbd_dn6);
        var_cgsbd_dn7 = (p.p50 * var_cgsbd_dn7);
        var_cgsbd_dn10 = (p.p50 * var_cgsbd_dn10);
        var_cgsbd_dn11 = (p.p50 * var_cgsbd_dn11);
        var_cgsbd_dn12 = (p.p50 * var_cgsbd_dn12);
        var_cgsbd_dn13 = (p.p50 * var_cgsbd_dn13);
        var_cgsbd_dn15 = (p.p50 * var_cgsbd_dn15);
        var_cgsbd_dn16 = (p.p50 * var_cgsbd_dn16);
        var_cgsbd_dn17 = (p.p50 * var_cgsbd_dn17);
        var_cgsbd_dn18 = (p.p50 * var_cgsbd_dn18);

        let (assign34870_e49841, assign34870_e49841_d_n0, assign34870_e49841_d_n2, assign34870_e49841_d_n6, assign34870_e49841_d_n7, assign34870_e49841_d_n10, assign34870_e49841_d_n11, assign34870_e49841_d_n12, assign34870_e49841_d_n13, assign34870_e49841_d_n15, assign34870_e49841_d_n16, assign34870_e49841_d_n17, assign34870_e49841_d_n18,) = {
    if (var_mode > 0.0) {
        (var_cgsbd, var_cgsbd_dn0, var_cgsbd_dn2, var_cgsbd_dn6, var_cgsbd_dn7, var_cgsbd_dn10, var_cgsbd_dn11, var_cgsbd_dn12, var_cgsbd_dn13, var_cgsbd_dn15, var_cgsbd_dn16, var_cgsbd_dn17, var_cgsbd_dn18,)
    } else {
        (var_cgdbd, var_cgdbd_dn0, var_cgdbd_dn2, var_cgdbd_dn6, var_cgdbd_dn7, var_cgdbd_dn10, var_cgdbd_dn11, var_cgdbd_dn12, var_cgdbd_dn13, var_cgdbd_dn15, var_cgdbd_dn16, var_cgdbd_dn17, var_cgdbd_dn18,)
    }
};
        var_cgsb = assign34870_e49841;
        var_cgsb_dn0 = assign34870_e49841_d_n0;
        var_cgsb_dn2 = assign34870_e49841_d_n2;
        var_cgsb_dn6 = assign34870_e49841_d_n6;
        var_cgsb_dn7 = assign34870_e49841_d_n7;
        var_cgsb_dn10 = assign34870_e49841_d_n10;
        var_cgsb_dn11 = assign34870_e49841_d_n11;
        var_cgsb_dn12 = assign34870_e49841_d_n12;
        var_cgsb_dn13 = assign34870_e49841_d_n13;
        var_cgsb_dn15 = assign34870_e49841_d_n15;
        var_cgsb_dn16 = assign34870_e49841_d_n16;
        var_cgsb_dn17 = assign34870_e49841_d_n17;
        var_cgsb_dn18 = assign34870_e49841_d_n18;

        let assign34880_e49855: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (var_flg_ign == 1.0)) && (var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        var_guard1149 = assign34880_e49855;

        let (assign34890_e49865,) = {
    if (var_guard1149 != 0.0) {
        let assign34890_e49859: f64 = (1e-6 * var_c_fox);
        let assign34890_e49861: f64 = (assign34890_e49859 * var_weffcv_nf);
        let assign34890_e49863: f64 = (assign34890_e49861 * var_leff_cv);
        (assign34890_e49863,)
    } else {
        (var_t0__blk1143,)
    }
};
        var_t0__blk1143 = assign34890_e49865;

        *var_cgdbd_slot = var_cgdbd;
        *var_cgdbd_dn0_slot = var_cgdbd_dn0;
        *var_cgdbd_dn10_slot = var_cgdbd_dn10;
        *var_cgdbd_dn11_slot = var_cgdbd_dn11;
        *var_cgdbd_dn12_slot = var_cgdbd_dn12;
        *var_cgdbd_dn13_slot = var_cgdbd_dn13;
        *var_cgdbd_dn15_slot = var_cgdbd_dn15;
        *var_cgdbd_dn16_slot = var_cgdbd_dn16;
        *var_cgdbd_dn17_slot = var_cgdbd_dn17;
        *var_cgdbd_dn18_slot = var_cgdbd_dn18;
        *var_cgdbd_dn2_slot = var_cgdbd_dn2;
        *var_cgdbd_dn6_slot = var_cgdbd_dn6;
        *var_cgdbd_dn7_slot = var_cgdbd_dn7;
        *var_cgsb_slot = var_cgsb;
        *var_cgsb_dn0_slot = var_cgsb_dn0;
        *var_cgsb_dn10_slot = var_cgsb_dn10;
        *var_cgsb_dn11_slot = var_cgsb_dn11;
        *var_cgsb_dn12_slot = var_cgsb_dn12;
        *var_cgsb_dn13_slot = var_cgsb_dn13;
        *var_cgsb_dn15_slot = var_cgsb_dn15;
        *var_cgsb_dn16_slot = var_cgsb_dn16;
        *var_cgsb_dn17_slot = var_cgsb_dn17;
        *var_cgsb_dn18_slot = var_cgsb_dn18;
        *var_cgsb_dn2_slot = var_cgsb_dn2;
        *var_cgsb_dn6_slot = var_cgsb_dn6;
        *var_cgsb_dn7_slot = var_cgsb_dn7;
        *var_cgsbd_slot = var_cgsbd;
        *var_cgsbd_dn0_slot = var_cgsbd_dn0;
        *var_cgsbd_dn10_slot = var_cgsbd_dn10;
        *var_cgsbd_dn11_slot = var_cgsbd_dn11;
        *var_cgsbd_dn12_slot = var_cgsbd_dn12;
        *var_cgsbd_dn13_slot = var_cgsbd_dn13;
        *var_cgsbd_dn15_slot = var_cgsbd_dn15;
        *var_cgsbd_dn16_slot = var_cgsbd_dn16;
        *var_cgsbd_dn17_slot = var_cgsbd_dn17;
        *var_cgsbd_dn18_slot = var_cgsbd_dn18;
        *var_cgsbd_dn2_slot = var_cgsbd_dn2;
        *var_cgsbd_dn6_slot = var_cgsbd_dn6;
        *var_cgsbd_dn7_slot = var_cgsbd_dn7;
        *var_guard1139_slot = var_guard1139;
        *var_guard1140_slot = var_guard1140;
        *var_guard1141_slot = var_guard1141;
        *var_guard1142_slot = var_guard1142;
        *var_guard1149_slot = var_guard1149;
        *var_ibdb_slot = var_ibdb;
        *var_ibdb_dn0_slot = var_ibdb_dn0;
        *var_ibdb_dn10_slot = var_ibdb_dn10;
        *var_ibdb_dn11_slot = var_ibdb_dn11;
        *var_ibdb_dn12_slot = var_ibdb_dn12;
        *var_ibdb_dn17_slot = var_ibdb_dn17;
        *var_ibdb_dn2_slot = var_ibdb_dn2;
        *var_ibdb_dn6_slot = var_ibdb_dn6;
        *var_ibdb_dn7_slot = var_ibdb_dn7;
        *var_ibsb_slot = var_ibsb;
        *var_ibsb_dn0_slot = var_ibsb_dn0;
        *var_ibsb_dn10_slot = var_ibsb_dn10;
        *var_ibsb_dn11_slot = var_ibsb_dn11;
        *var_ibsb_dn12_slot = var_ibsb_dn12;
        *var_ibsb_dn17_slot = var_ibsb_dn17;
        *var_ibsb_dn2_slot = var_ibsb_dn2;
        *var_ibsb_dn6_slot = var_ibsb_dn6;
        *var_ibsb_dn7_slot = var_ibsb_dn7;
        *var_igbe_slot = var_igbe;
        *var_igbe_dn0_slot = var_igbe_dn0;
        *var_igbe_dn10_slot = var_igbe_dn10;
        *var_igbe_dn11_slot = var_igbe_dn11;
        *var_igbe_dn12_slot = var_igbe_dn12;
        *var_igbe_dn17_slot = var_igbe_dn17;
        *var_igbe_dn2_slot = var_igbe_dn2;
        *var_igbe_dn6_slot = var_igbe_dn6;
        *var_igbe_dn7_slot = var_igbe_dn7;
        *var_igde_slot = var_igde;
        *var_igde_dn0_slot = var_igde_dn0;
        *var_igde_dn10_slot = var_igde_dn10;
        *var_igde_dn11_slot = var_igde_dn11;
        *var_igde_dn12_slot = var_igde_dn12;
        *var_igde_dn17_slot = var_igde_dn17;
        *var_igde_dn2_slot = var_igde_dn2;
        *var_igde_dn6_slot = var_igde_dn6;
        *var_igde_dn7_slot = var_igde_dn7;
        *var_igidle_slot = var_igidle;
        *var_igidle_dn0_slot = var_igidle_dn0;
        *var_igidle_dn10_slot = var_igidle_dn10;
        *var_igidle_dn11_slot = var_igidle_dn11;
        *var_igidle_dn12_slot = var_igidle_dn12;
        *var_igidle_dn17_slot = var_igidle_dn17;
        *var_igidle_dn2_slot = var_igidle_dn2;
        *var_igidle_dn6_slot = var_igidle_dn6;
        *var_igidle_dn7_slot = var_igidle_dn7;
        *var_igisle_slot = var_igisle;
        *var_igisle_dn0_slot = var_igisle_dn0;
        *var_igisle_dn10_slot = var_igisle_dn10;
        *var_igisle_dn11_slot = var_igisle_dn11;
        *var_igisle_dn12_slot = var_igisle_dn12;
        *var_igisle_dn17_slot = var_igisle_dn17;
        *var_igisle_dn2_slot = var_igisle_dn2;
        *var_igisle_dn6_slot = var_igisle_dn6;
        *var_igisle_dn7_slot = var_igisle_dn7;
        *var_igse_slot = var_igse;
        *var_igse_dn0_slot = var_igse_dn0;
        *var_igse_dn10_slot = var_igse_dn10;
        *var_igse_dn11_slot = var_igse_dn11;
        *var_igse_dn12_slot = var_igse_dn12;
        *var_igse_dn17_slot = var_igse_dn17;
        *var_igse_dn2_slot = var_igse_dn2;
        *var_igse_dn6_slot = var_igse_dn6;
        *var_igse_dn7_slot = var_igse_dn7;
        *var_isube_slot = var_isube;
        *var_isube_dn0_slot = var_isube_dn0;
        *var_isube_dn10_slot = var_isube_dn10;
        *var_isube_dn11_slot = var_isube_dn11;
        *var_isube_dn12_slot = var_isube_dn12;
        *var_isube_dn17_slot = var_isube_dn17;
        *var_isube_dn2_slot = var_isube_dn2;
        *var_isube_dn6_slot = var_isube_dn6;
        *var_isube_dn7_slot = var_isube_dn7;
        *var_noithrml_slot = var_noithrml;
        *var_noithrml_dn0_slot = var_noithrml_dn0;
        *var_noithrml_dn10_slot = var_noithrml_dn10;
        *var_noithrml_dn11_slot = var_noithrml_dn11;
        *var_noithrml_dn12_slot = var_noithrml_dn12;
        *var_noithrml_dn17_slot = var_noithrml_dn17;
        *var_noithrml_dn2_slot = var_noithrml_dn2;
        *var_noithrml_dn6_slot = var_noithrml_dn6;
        *var_noithrml_dn7_slot = var_noithrml_dn7;
        *var_qbd_s0_slot = var_qbd_s0;
        *var_qbd_s0_dn0_slot = var_qbd_s0_dn0;
        *var_qbd_s0_dn10_slot = var_qbd_s0_dn10;
        *var_qbd_s0_dn11_slot = var_qbd_s0_dn11;
        *var_qbd_s0_dn12_slot = var_qbd_s0_dn12;
        *var_qbd_s0_dn17_slot = var_qbd_s0_dn17;
        *var_qbd_s0_dn2_slot = var_qbd_s0_dn2;
        *var_qbd_s0_dn6_slot = var_qbd_s0_dn6;
        *var_qbd_s0_dn7_slot = var_qbd_s0_dn7;
        *var_qbs_s0_slot = var_qbs_s0;
        *var_qbs_s0_dn0_slot = var_qbs_s0_dn0;
        *var_qbs_s0_dn10_slot = var_qbs_s0_dn10;
        *var_qbs_s0_dn11_slot = var_qbs_s0_dn11;
        *var_qbs_s0_dn12_slot = var_qbs_s0_dn12;
        *var_qbs_s0_dn17_slot = var_qbs_s0_dn17;
        *var_qbs_s0_dn2_slot = var_qbs_s0_dn2;
        *var_qbs_s0_dn6_slot = var_qbs_s0_dn6;
        *var_qbs_s0_dn7_slot = var_qbs_s0_dn7;
        *var_qde_slot = var_qde;
        *var_qde_dn0_slot = var_qde_dn0;
        *var_qde_dn10_slot = var_qde_dn10;
        *var_qde_dn11_slot = var_qde_dn11;
        *var_qde_dn12_slot = var_qde_dn12;
        *var_qde_dn13_slot = var_qde_dn13;
        *var_qde_dn15_slot = var_qde_dn15;
        *var_qde_dn16_slot = var_qde_dn16;
        *var_qde_dn17_slot = var_qde_dn17;
        *var_qde_dn18_slot = var_qde_dn18;
        *var_qde_dn2_slot = var_qde_dn2;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qde_dn7_slot = var_qde_dn7;
        *var_qge_slot = var_qge;
        *var_qge_dn0_slot = var_qge_dn0;
        *var_qge_dn10_slot = var_qge_dn10;
        *var_qge_dn11_slot = var_qge_dn11;
        *var_qge_dn12_slot = var_qge_dn12;
        *var_qge_dn13_slot = var_qge_dn13;
        *var_qge_dn15_slot = var_qge_dn15;
        *var_qge_dn16_slot = var_qge_dn16;
        *var_qge_dn17_slot = var_qge_dn17;
        *var_qge_dn18_slot = var_qge_dn18;
        *var_qge_dn2_slot = var_qge_dn2;
        *var_qge_dn6_slot = var_qge_dn6;
        *var_qge_dn7_slot = var_qge_dn7;
        *var_qse_slot = var_qse;
        *var_qse_dn0_slot = var_qse_dn0;
        *var_qse_dn10_slot = var_qse_dn10;
        *var_qse_dn11_slot = var_qse_dn11;
        *var_qse_dn12_slot = var_qse_dn12;
        *var_qse_dn13_slot = var_qse_dn13;
        *var_qse_dn15_slot = var_qse_dn15;
        *var_qse_dn16_slot = var_qse_dn16;
        *var_qse_dn17_slot = var_qse_dn17;
        *var_qse_dn18_slot = var_qse_dn18;
        *var_qse_dn2_slot = var_qse_dn2;
        *var_qse_dn6_slot = var_qse_dn6;
        *var_qse_dn7_slot = var_qse_dn7;
        *var_t0__blk1143_slot = var_t0__blk1143;
    }

    pub(super) fn stamp_transient_block_122(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_beta_inv: f64,
        var_beta_inv_dn10: f64,
        var_cgsb: f64,
        var_cgsb_dn0: f64,
        var_cgsb_dn10: f64,
        var_cgsb_dn11: f64,
        var_cgsb_dn12: f64,
        var_cgsb_dn13: f64,
        var_cgsb_dn15: f64,
        var_cgsb_dn16: f64,
        var_cgsb_dn17: f64,
        var_cgsb_dn18: f64,
        var_cgsb_dn2: f64,
        var_cgsb_dn6: f64,
        var_cgsb_dn7: f64,
        var_crl_f: f64,
        var_crl_f_dn0: f64,
        var_crl_f_dn10: f64,
        var_crl_f_dn11: f64,
        var_crl_f_dn12: f64,
        var_crl_f_dn17: f64,
        var_crl_f_dn2: f64,
        var_crl_f_dn6: f64,
        var_crl_f_dn7: f64,
        var_gds0_ign: f64,
        var_gds0_ign_dn0: f64,
        var_gds0_ign_dn10: f64,
        var_gds0_ign_dn11: f64,
        var_gds0_ign_dn12: f64,
        var_gds0_ign_dn17: f64,
        var_gds0_ign_dn2: f64,
        var_gds0_ign_dn6: f64,
        var_gds0_ign_dn7: f64,
        var_guard1149: f64,
        var_kusai00: f64,
        var_kusai00_dn0: f64,
        var_kusai00_dn10: f64,
        var_kusai00_dn11: f64,
        var_kusai00_dn12: f64,
        var_kusai00_dn17: f64,
        var_kusai00_dn2: f64,
        var_kusai00_dn6: f64,
        var_kusai00_dn7: f64,
        var_kusai00l: f64,
        var_kusai_ig: f64,
        var_kusai_ig_dn0: f64,
        var_kusai_ig_dn10: f64,
        var_kusai_ig_dn11: f64,
        var_kusai_ig_dn12: f64,
        var_kusai_ig_dn17: f64,
        var_kusai_ig_dn2: f64,
        var_kusai_ig_dn6: f64,
        var_kusai_ig_dn7: f64,
        var_kusail: f64,
        var_kusail_dn0: f64,
        var_kusail_dn10: f64,
        var_kusail_dn11: f64,
        var_kusail_dn12: f64,
        var_kusail_dn17: f64,
        var_kusail_dn2: f64,
        var_kusail_dn6: f64,
        var_kusail_dn7: f64,
        var_mfactor: f64,
        var_mu: f64,
        var_mu_dn0: f64,
        var_mu_dn10: f64,
        var_mu_dn11: f64,
        var_mu_dn12: f64,
        var_mu_dn17: f64,
        var_mu_dn2: f64,
        var_mu_dn6: f64,
        var_mu_dn7: f64,
        var_mud_hoso: f64,
        var_mud_hoso_dn0: f64,
        var_mud_hoso_dn10: f64,
        var_mud_hoso_dn11: f64,
        var_mud_hoso_dn12: f64,
        var_mud_hoso_dn17: f64,
        var_mud_hoso_dn2: f64,
        var_mud_hoso_dn6: f64,
        var_mud_hoso_dn7: f64,
        var_muun: f64,
        var_muun_dn0: f64,
        var_muun_dn10: f64,
        var_muun_dn11: f64,
        var_muun_dn12: f64,
        var_muun_dn17: f64,
        var_muun_dn2: f64,
        var_muun_dn6: f64,
        var_muun_dn7: f64,
        var_sqrtkusail: f64,
        var_sqrtkusail_dn0: f64,
        var_sqrtkusail_dn10: f64,
        var_sqrtkusail_dn11: f64,
        var_sqrtkusail_dn12: f64,
        var_sqrtkusail_dn17: f64,
        var_sqrtkusail_dn2: f64,
        var_sqrtkusail_dn6: f64,
        var_sqrtkusail_dn7: f64,
        var_t0__blk1143: f64,
        var_ttemp: f64,
        var_ttemp_dn10: f64,
        var_uc_tnom: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn17: f64,
        var_vds_dn2: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vgvt: f64,
        var_vgvt_dn0: f64,
        var_vgvt_dn10: f64,
        var_vgvt_dn11: f64,
        var_vgvt_dn12: f64,
        var_vgvt_dn17: f64,
        var_vgvt_dn2: f64,
        var_vgvt_dn6: f64,
        var_vgvt_dn7: f64,
        var_weff: f64,
        var_correct_w1_slot: &mut f64,
        var_correct_w1_dn0_slot: &mut f64,
        var_correct_w1_dn10_slot: &mut f64,
        var_correct_w1_dn11_slot: &mut f64,
        var_correct_w1_dn12_slot: &mut f64,
        var_correct_w1_dn17_slot: &mut f64,
        var_correct_w1_dn2_slot: &mut f64,
        var_correct_w1_dn6_slot: &mut f64,
        var_correct_w1_dn7_slot: &mut f64,
        var_guard1150_slot: &mut f64,
        var_guard1151_slot: &mut f64,
        var_guard1171_slot: &mut f64,
        var_ldrifte_slot: &mut f64,
        var_mks_rdrmue_slot: &mut f64,
        var_mks_rdrvmax_slot: &mut f64,
        var_mumoda_slot: &mut f64,
        var_mumoda_dn0_slot: &mut f64,
        var_mumoda_dn10_slot: &mut f64,
        var_mumoda_dn11_slot: &mut f64,
        var_mumoda_dn12_slot: &mut f64,
        var_mumoda_dn17_slot: &mut f64,
        var_mumoda_dn2_slot: &mut f64,
        var_mumoda_dn6_slot: &mut f64,
        var_mumoda_dn7_slot: &mut f64,
        var_mumodb_slot: &mut f64,
        var_mumodb_dn0_slot: &mut f64,
        var_mumodb_dn10_slot: &mut f64,
        var_mumodb_dn11_slot: &mut f64,
        var_mumodb_dn12_slot: &mut f64,
        var_mumodb_dn17_slot: &mut f64,
        var_mumodb_dn2_slot: &mut f64,
        var_mumodb_dn6_slot: &mut f64,
        var_mumodb_dn7_slot: &mut f64,
        var_nign0_slot: &mut f64,
        var_nign0_dn0_slot: &mut f64,
        var_nign0_dn10_slot: &mut f64,
        var_nign0_dn11_slot: &mut f64,
        var_nign0_dn12_slot: &mut f64,
        var_nign0_dn13_slot: &mut f64,
        var_nign0_dn15_slot: &mut f64,
        var_nign0_dn16_slot: &mut f64,
        var_nign0_dn17_slot: &mut f64,
        var_nign0_dn18_slot: &mut f64,
        var_nign0_dn2_slot: &mut f64,
        var_nign0_dn6_slot: &mut f64,
        var_nign0_dn7_slot: &mut f64,
        var_noicross_slot: &mut f64,
        var_noicross_dn0_slot: &mut f64,
        var_noicross_dn10_slot: &mut f64,
        var_noicross_dn11_slot: &mut f64,
        var_noicross_dn12_slot: &mut f64,
        var_noicross_dn17_slot: &mut f64,
        var_noicross_dn2_slot: &mut f64,
        var_noicross_dn6_slot: &mut f64,
        var_noicross_dn7_slot: &mut f64,
        var_noiigate_slot: &mut f64,
        var_noiigate_dn0_slot: &mut f64,
        var_noiigate_dn10_slot: &mut f64,
        var_noiigate_dn11_slot: &mut f64,
        var_noiigate_dn12_slot: &mut f64,
        var_noiigate_dn13_slot: &mut f64,
        var_noiigate_dn15_slot: &mut f64,
        var_noiigate_dn16_slot: &mut f64,
        var_noiigate_dn17_slot: &mut f64,
        var_noiigate_dn18_slot: &mut f64,
        var_noiigate_dn2_slot: &mut f64,
        var_noiigate_dn6_slot: &mut f64,
        var_noiigate_dn7_slot: &mut f64,
        var_nover_slot: &mut f64,
        var_rdde_slot: &mut f64,
        var_rdde_dn0_slot: &mut f64,
        var_rdde_dn10_slot: &mut f64,
        var_rdde_dn11_slot: &mut f64,
        var_rdde_dn12_slot: &mut f64,
        var_rdde_dn17_slot: &mut f64,
        var_rdde_dn2_slot: &mut f64,
        var_rdde_dn6_slot: &mut f64,
        var_rdde_dn7_slot: &mut f64,
        var_rdmod_slot: &mut f64,
        var_rrdrbb_slot: &mut f64,
        var_rrdrbb_dn10_slot: &mut f64,
        var_rsd0_slot: &mut f64,
        var_rsde_slot: &mut f64,
        var_rsde_dn0_slot: &mut f64,
        var_rsde_dn10_slot: &mut f64,
        var_rsde_dn11_slot: &mut f64,
        var_rsde_dn12_slot: &mut f64,
        var_rsde_dn17_slot: &mut f64,
        var_rsde_dn2_slot: &mut f64,
        var_rsde_dn6_slot: &mut f64,
        var_rsde_dn7_slot: &mut f64,
        var_t1__blk1144_slot: &mut f64,
        var_t1__blk1144_dn0_slot: &mut f64,
        var_t1__blk1144_dn10_slot: &mut f64,
        var_t1__blk1144_dn11_slot: &mut f64,
        var_t1__blk1144_dn12_slot: &mut f64,
        var_t1__blk1144_dn13_slot: &mut f64,
        var_t1__blk1144_dn15_slot: &mut f64,
        var_t1__blk1144_dn16_slot: &mut f64,
        var_t1__blk1144_dn17_slot: &mut f64,
        var_t1__blk1144_dn18_slot: &mut f64,
        var_t1__blk1144_dn2_slot: &mut f64,
        var_t1__blk1144_dn6_slot: &mut f64,
        var_t1__blk1144_dn7_slot: &mut f64,
        var_tratio_slot: &mut f64,
        var_tratio_dn10_slot: &mut f64,
        var_vrdr_slot: &mut f64,
        var_vrdr_dn0_slot: &mut f64,
        var_vrdr_dn2_slot: &mut f64,
        var_vrdr_dn6_slot: &mut f64,
        var_vrdr_dn7_slot: &mut f64,
        var_weff_nf_1_slot: &mut f64,
        var_xov_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let mut var_correct_w1: f64 = *var_correct_w1_slot;
        let mut var_correct_w1_dn0: f64 = *var_correct_w1_dn0_slot;
        let mut var_correct_w1_dn10: f64 = *var_correct_w1_dn10_slot;
        let mut var_correct_w1_dn11: f64 = *var_correct_w1_dn11_slot;
        let mut var_correct_w1_dn12: f64 = *var_correct_w1_dn12_slot;
        let mut var_correct_w1_dn17: f64 = *var_correct_w1_dn17_slot;
        let mut var_correct_w1_dn2: f64 = *var_correct_w1_dn2_slot;
        let mut var_correct_w1_dn6: f64 = *var_correct_w1_dn6_slot;
        let mut var_correct_w1_dn7: f64 = *var_correct_w1_dn7_slot;
        let mut var_guard1150: f64 = *var_guard1150_slot;
        let mut var_guard1151: f64 = *var_guard1151_slot;
        let mut var_guard1171: f64 = *var_guard1171_slot;
        let mut var_ldrifte: f64 = *var_ldrifte_slot;
        let mut var_mks_rdrmue: f64 = *var_mks_rdrmue_slot;
        let mut var_mks_rdrvmax: f64 = *var_mks_rdrvmax_slot;
        let mut var_mumoda: f64 = *var_mumoda_slot;
        let mut var_mumoda_dn0: f64 = *var_mumoda_dn0_slot;
        let mut var_mumoda_dn10: f64 = *var_mumoda_dn10_slot;
        let mut var_mumoda_dn11: f64 = *var_mumoda_dn11_slot;
        let mut var_mumoda_dn12: f64 = *var_mumoda_dn12_slot;
        let mut var_mumoda_dn17: f64 = *var_mumoda_dn17_slot;
        let mut var_mumoda_dn2: f64 = *var_mumoda_dn2_slot;
        let mut var_mumoda_dn6: f64 = *var_mumoda_dn6_slot;
        let mut var_mumoda_dn7: f64 = *var_mumoda_dn7_slot;
        let mut var_mumodb: f64 = *var_mumodb_slot;
        let mut var_mumodb_dn0: f64 = *var_mumodb_dn0_slot;
        let mut var_mumodb_dn10: f64 = *var_mumodb_dn10_slot;
        let mut var_mumodb_dn11: f64 = *var_mumodb_dn11_slot;
        let mut var_mumodb_dn12: f64 = *var_mumodb_dn12_slot;
        let mut var_mumodb_dn17: f64 = *var_mumodb_dn17_slot;
        let mut var_mumodb_dn2: f64 = *var_mumodb_dn2_slot;
        let mut var_mumodb_dn6: f64 = *var_mumodb_dn6_slot;
        let mut var_mumodb_dn7: f64 = *var_mumodb_dn7_slot;
        let mut var_nign0: f64 = *var_nign0_slot;
        let mut var_nign0_dn0: f64 = *var_nign0_dn0_slot;
        let mut var_nign0_dn10: f64 = *var_nign0_dn10_slot;
        let mut var_nign0_dn11: f64 = *var_nign0_dn11_slot;
        let mut var_nign0_dn12: f64 = *var_nign0_dn12_slot;
        let mut var_nign0_dn13: f64 = *var_nign0_dn13_slot;
        let mut var_nign0_dn15: f64 = *var_nign0_dn15_slot;
        let mut var_nign0_dn16: f64 = *var_nign0_dn16_slot;
        let mut var_nign0_dn17: f64 = *var_nign0_dn17_slot;
        let mut var_nign0_dn18: f64 = *var_nign0_dn18_slot;
        let mut var_nign0_dn2: f64 = *var_nign0_dn2_slot;
        let mut var_nign0_dn6: f64 = *var_nign0_dn6_slot;
        let mut var_nign0_dn7: f64 = *var_nign0_dn7_slot;
        let mut var_noicross: f64 = *var_noicross_slot;
        let mut var_noicross_dn0: f64 = *var_noicross_dn0_slot;
        let mut var_noicross_dn10: f64 = *var_noicross_dn10_slot;
        let mut var_noicross_dn11: f64 = *var_noicross_dn11_slot;
        let mut var_noicross_dn12: f64 = *var_noicross_dn12_slot;
        let mut var_noicross_dn17: f64 = *var_noicross_dn17_slot;
        let mut var_noicross_dn2: f64 = *var_noicross_dn2_slot;
        let mut var_noicross_dn6: f64 = *var_noicross_dn6_slot;
        let mut var_noicross_dn7: f64 = *var_noicross_dn7_slot;
        let mut var_noiigate: f64 = *var_noiigate_slot;
        let mut var_noiigate_dn0: f64 = *var_noiigate_dn0_slot;
        let mut var_noiigate_dn10: f64 = *var_noiigate_dn10_slot;
        let mut var_noiigate_dn11: f64 = *var_noiigate_dn11_slot;
        let mut var_noiigate_dn12: f64 = *var_noiigate_dn12_slot;
        let mut var_noiigate_dn13: f64 = *var_noiigate_dn13_slot;
        let mut var_noiigate_dn15: f64 = *var_noiigate_dn15_slot;
        let mut var_noiigate_dn16: f64 = *var_noiigate_dn16_slot;
        let mut var_noiigate_dn17: f64 = *var_noiigate_dn17_slot;
        let mut var_noiigate_dn18: f64 = *var_noiigate_dn18_slot;
        let mut var_noiigate_dn2: f64 = *var_noiigate_dn2_slot;
        let mut var_noiigate_dn6: f64 = *var_noiigate_dn6_slot;
        let mut var_noiigate_dn7: f64 = *var_noiigate_dn7_slot;
        let mut var_nover: f64 = *var_nover_slot;
        let mut var_rdde: f64 = *var_rdde_slot;
        let mut var_rdde_dn0: f64 = *var_rdde_dn0_slot;
        let mut var_rdde_dn10: f64 = *var_rdde_dn10_slot;
        let mut var_rdde_dn11: f64 = *var_rdde_dn11_slot;
        let mut var_rdde_dn12: f64 = *var_rdde_dn12_slot;
        let mut var_rdde_dn17: f64 = *var_rdde_dn17_slot;
        let mut var_rdde_dn2: f64 = *var_rdde_dn2_slot;
        let mut var_rdde_dn6: f64 = *var_rdde_dn6_slot;
        let mut var_rdde_dn7: f64 = *var_rdde_dn7_slot;
        let mut var_rdmod: f64 = *var_rdmod_slot;
        let mut var_rrdrbb: f64 = *var_rrdrbb_slot;
        let mut var_rrdrbb_dn10: f64 = *var_rrdrbb_dn10_slot;
        let mut var_rsd0: f64 = *var_rsd0_slot;
        let mut var_rsde: f64 = *var_rsde_slot;
        let mut var_rsde_dn0: f64 = *var_rsde_dn0_slot;
        let mut var_rsde_dn10: f64 = *var_rsde_dn10_slot;
        let mut var_rsde_dn11: f64 = *var_rsde_dn11_slot;
        let mut var_rsde_dn12: f64 = *var_rsde_dn12_slot;
        let mut var_rsde_dn17: f64 = *var_rsde_dn17_slot;
        let mut var_rsde_dn2: f64 = *var_rsde_dn2_slot;
        let mut var_rsde_dn6: f64 = *var_rsde_dn6_slot;
        let mut var_rsde_dn7: f64 = *var_rsde_dn7_slot;
        let mut var_t1__blk1144: f64 = *var_t1__blk1144_slot;
        let mut var_t1__blk1144_dn0: f64 = *var_t1__blk1144_dn0_slot;
        let mut var_t1__blk1144_dn10: f64 = *var_t1__blk1144_dn10_slot;
        let mut var_t1__blk1144_dn11: f64 = *var_t1__blk1144_dn11_slot;
        let mut var_t1__blk1144_dn12: f64 = *var_t1__blk1144_dn12_slot;
        let mut var_t1__blk1144_dn13: f64 = *var_t1__blk1144_dn13_slot;
        let mut var_t1__blk1144_dn15: f64 = *var_t1__blk1144_dn15_slot;
        let mut var_t1__blk1144_dn16: f64 = *var_t1__blk1144_dn16_slot;
        let mut var_t1__blk1144_dn17: f64 = *var_t1__blk1144_dn17_slot;
        let mut var_t1__blk1144_dn18: f64 = *var_t1__blk1144_dn18_slot;
        let mut var_t1__blk1144_dn2: f64 = *var_t1__blk1144_dn2_slot;
        let mut var_t1__blk1144_dn6: f64 = *var_t1__blk1144_dn6_slot;
        let mut var_t1__blk1144_dn7: f64 = *var_t1__blk1144_dn7_slot;
        let mut var_tratio: f64 = *var_tratio_slot;
        let mut var_tratio_dn10: f64 = *var_tratio_dn10_slot;
        let mut var_vrdr: f64 = *var_vrdr_slot;
        let mut var_vrdr_dn0: f64 = *var_vrdr_dn0_slot;
        let mut var_vrdr_dn2: f64 = *var_vrdr_dn2_slot;
        let mut var_vrdr_dn6: f64 = *var_vrdr_dn6_slot;
        let mut var_vrdr_dn7: f64 = *var_vrdr_dn7_slot;
        let mut var_weff_nf_1: f64 = *var_weff_nf_1_slot;
        let mut var_xov: f64 = *var_xov_slot;

        let (assign34900_e49871, assign34900_e49871_d_n0, assign34900_e49871_d_n2, assign34900_e49871_d_n6, assign34900_e49871_d_n7, assign34900_e49871_d_n10, assign34900_e49871_d_n11, assign34900_e49871_d_n12, assign34900_e49871_d_n13, assign34900_e49871_d_n15, assign34900_e49871_d_n16, assign34900_e49871_d_n17, assign34900_e49871_d_n18,) = {
    if (var_guard1149 != 0.0) {
        let assign34900_e49869: f64 = (var_cgsb / var_mfactor);
        (assign34900_e49869, (var_cgsb_dn0 / var_mfactor), (var_cgsb_dn2 / var_mfactor), (var_cgsb_dn6 / var_mfactor), (var_cgsb_dn7 / var_mfactor), (var_cgsb_dn10 / var_mfactor), (var_cgsb_dn11 / var_mfactor), (var_cgsb_dn12 / var_mfactor), (var_cgsb_dn13 / var_mfactor), (var_cgsb_dn15 / var_mfactor), (var_cgsb_dn16 / var_mfactor), (var_cgsb_dn17 / var_mfactor), (var_cgsb_dn18 / var_mfactor),)
    } else {
        (var_t1__blk1144, var_t1__blk1144_dn0, var_t1__blk1144_dn2, var_t1__blk1144_dn6, var_t1__blk1144_dn7, var_t1__blk1144_dn10, var_t1__blk1144_dn11, var_t1__blk1144_dn12, var_t1__blk1144_dn13, var_t1__blk1144_dn15, var_t1__blk1144_dn16, var_t1__blk1144_dn17, var_t1__blk1144_dn18,)
    }
};
        var_t1__blk1144 = assign34900_e49871;
        var_t1__blk1144_dn0 = assign34900_e49871_d_n0;
        var_t1__blk1144_dn2 = assign34900_e49871_d_n2;
        var_t1__blk1144_dn6 = assign34900_e49871_d_n6;
        var_t1__blk1144_dn7 = assign34900_e49871_d_n7;
        var_t1__blk1144_dn10 = assign34900_e49871_d_n10;
        var_t1__blk1144_dn11 = assign34900_e49871_d_n11;
        var_t1__blk1144_dn12 = assign34900_e49871_d_n12;
        var_t1__blk1144_dn13 = assign34900_e49871_d_n13;
        var_t1__blk1144_dn15 = assign34900_e49871_d_n15;
        var_t1__blk1144_dn16 = assign34900_e49871_d_n16;
        var_t1__blk1144_dn17 = assign34900_e49871_d_n17;
        var_t1__blk1144_dn18 = assign34900_e49871_d_n18;

        let (assign34910_e49885, assign34910_e49885_d_n0, assign34910_e49885_d_n2, assign34910_e49885_d_n6, assign34910_e49885_d_n7, assign34910_e49885_d_n10, assign34910_e49885_d_n11, assign34910_e49885_d_n12, assign34910_e49885_d_n13, assign34910_e49885_d_n15, assign34910_e49885_d_n16, assign34910_e49885_d_n17, assign34910_e49885_d_n18,) = {
    if (var_guard1149 != 0.0) {
        let assign34910_e49875: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign34910_e49877: f64 = (assign34910_e49875 * var_beta_inv);
        let assign34910_e49879: f64 = (assign34910_e49877 * var_t1__blk1144);
        let assign34910_e49881: f64 = (assign34910_e49879 * var_t1__blk1144);
        let assign34910_e49883: f64 = (assign34910_e49881 / var_gds0_ign);
        (assign34910_e49883, ((((((assign34910_e49877 * var_t1__blk1144_dn0) * var_t1__blk1144) + (assign34910_e49879 * var_t1__blk1144_dn0)) * var_gds0_ign) - (assign34910_e49881 * var_gds0_ign_dn0)) / (var_gds0_ign * var_gds0_ign)), ((((((assign34910_e49877 * var_t1__blk1144_dn2) * var_t1__blk1144) + (assign34910_e49879 * var_t1__blk1144_dn2)) * var_gds0_ign) - (assign34910_e49881 * var_gds0_ign_dn2)) / (var_gds0_ign * var_gds0_ign)), ((((((assign34910_e49877 * var_t1__blk1144_dn6) * var_t1__blk1144) + (assign34910_e49879 * var_t1__blk1144_dn6)) * var_gds0_ign) - (assign34910_e49881 * var_gds0_ign_dn6)) / (var_gds0_ign * var_gds0_ign)), ((((((assign34910_e49877 * var_t1__blk1144_dn7) * var_t1__blk1144) + (assign34910_e49879 * var_t1__blk1144_dn7)) * var_gds0_ign) - (assign34910_e49881 * var_gds0_ign_dn7)) / (var_gds0_ign * var_gds0_ign)), ((((((((assign34910_e49875 * var_beta_inv_dn10) * var_t1__blk1144) + (assign34910_e49877 * var_t1__blk1144_dn10)) * var_t1__blk1144) + (assign34910_e49879 * var_t1__blk1144_dn10)) * var_gds0_ign) - (assign34910_e49881 * var_gds0_ign_dn10)) / (var_gds0_ign * var_gds0_ign)), ((((((assign34910_e49877 * var_t1__blk1144_dn11) * var_t1__blk1144) + (assign34910_e49879 * var_t1__blk1144_dn11)) * var_gds0_ign) - (assign34910_e49881 * var_gds0_ign_dn11)) / (var_gds0_ign * var_gds0_ign)), ((((((assign34910_e49877 * var_t1__blk1144_dn12) * var_t1__blk1144) + (assign34910_e49879 * var_t1__blk1144_dn12)) * var_gds0_ign) - (assign34910_e49881 * var_gds0_ign_dn12)) / (var_gds0_ign * var_gds0_ign)), ((((assign34910_e49877 * var_t1__blk1144_dn13) * var_t1__blk1144) + (assign34910_e49879 * var_t1__blk1144_dn13)) / var_gds0_ign), ((((assign34910_e49877 * var_t1__blk1144_dn15) * var_t1__blk1144) + (assign34910_e49879 * var_t1__blk1144_dn15)) / var_gds0_ign), ((((assign34910_e49877 * var_t1__blk1144_dn16) * var_t1__blk1144) + (assign34910_e49879 * var_t1__blk1144_dn16)) / var_gds0_ign), ((((((assign34910_e49877 * var_t1__blk1144_dn17) * var_t1__blk1144) + (assign34910_e49879 * var_t1__blk1144_dn17)) * var_gds0_ign) - (assign34910_e49881 * var_gds0_ign_dn17)) / (var_gds0_ign * var_gds0_ign)), ((((assign34910_e49877 * var_t1__blk1144_dn18) * var_t1__blk1144) + (assign34910_e49879 * var_t1__blk1144_dn18)) / var_gds0_ign),)
    } else {
        (var_nign0, var_nign0_dn0, var_nign0_dn2, var_nign0_dn6, var_nign0_dn7, var_nign0_dn10, var_nign0_dn11, var_nign0_dn12, var_nign0_dn13, var_nign0_dn15, var_nign0_dn16, var_nign0_dn17, var_nign0_dn18,)
    }
};
        var_nign0 = assign34910_e49885;
        var_nign0_dn0 = assign34910_e49885_d_n0;
        var_nign0_dn2 = assign34910_e49885_d_n2;
        var_nign0_dn6 = assign34910_e49885_d_n6;
        var_nign0_dn7 = assign34910_e49885_d_n7;
        var_nign0_dn10 = assign34910_e49885_d_n10;
        var_nign0_dn11 = assign34910_e49885_d_n11;
        var_nign0_dn12 = assign34910_e49885_d_n12;
        var_nign0_dn13 = assign34910_e49885_d_n13;
        var_nign0_dn15 = assign34910_e49885_d_n15;
        var_nign0_dn16 = assign34910_e49885_d_n16;
        var_nign0_dn17 = assign34910_e49885_d_n17;
        var_nign0_dn18 = assign34910_e49885_d_n18;

        let assign34920_e49889: f64 = (10.0 * 2.220446049250313e-16);
        let assign34920_e49894: f64 = (10.0 * 2.220446049250313e-16);
        let assign34920_e49896: f64 = if ((var_kusai00l > assign34920_e49889) && (var_vds > assign34920_e49894)) { 1.0 } else { 0.0 };
        var_guard1150 = assign34920_e49896;

        let (assign34930_e49904, assign34930_e49904_d_n0, assign34930_e49904_d_n2, assign34930_e49904_d_n6, assign34930_e49904_d_n7, assign34930_e49904_d_n10, assign34930_e49904_d_n11, assign34930_e49904_d_n12, assign34930_e49904_d_n17,) = {
    if ((var_guard1149 != 0.0) && (var_guard1150 != 0.0)) {
        let assign34930_e49902: f64 = (var_muun / var_mu);
        (assign34930_e49902, (((var_muun_dn0 * var_mu) - (var_muun * var_mu_dn0)) / (var_mu * var_mu)), (((var_muun_dn2 * var_mu) - (var_muun * var_mu_dn2)) / (var_mu * var_mu)), (((var_muun_dn6 * var_mu) - (var_muun * var_mu_dn6)) / (var_mu * var_mu)), (((var_muun_dn7 * var_mu) - (var_muun * var_mu_dn7)) / (var_mu * var_mu)), (((var_muun_dn10 * var_mu) - (var_muun * var_mu_dn10)) / (var_mu * var_mu)), (((var_muun_dn11 * var_mu) - (var_muun * var_mu_dn11)) / (var_mu * var_mu)), (((var_muun_dn12 * var_mu) - (var_muun * var_mu_dn12)) / (var_mu * var_mu)), (((var_muun_dn17 * var_mu) - (var_muun * var_mu_dn17)) / (var_mu * var_mu)),)
    } else {
        (var_mumoda, var_mumoda_dn0, var_mumoda_dn2, var_mumoda_dn6, var_mumoda_dn7, var_mumoda_dn10, var_mumoda_dn11, var_mumoda_dn12, var_mumoda_dn17,)
    }
};
        var_mumoda = assign34930_e49904;
        var_mumoda_dn0 = assign34930_e49904_d_n0;
        var_mumoda_dn2 = assign34930_e49904_d_n2;
        var_mumoda_dn6 = assign34930_e49904_d_n6;
        var_mumoda_dn7 = assign34930_e49904_d_n7;
        var_mumoda_dn10 = assign34930_e49904_d_n10;
        var_mumoda_dn11 = assign34930_e49904_d_n11;
        var_mumoda_dn12 = assign34930_e49904_d_n12;
        var_mumoda_dn17 = assign34930_e49904_d_n17;

        let (assign34940_e49916, assign34940_e49916_d_n0, assign34940_e49916_d_n2, assign34940_e49916_d_n6, assign34940_e49916_d_n7, assign34940_e49916_d_n10, assign34940_e49916_d_n11, assign34940_e49916_d_n12, assign34940_e49916_d_n17,) = {
    if ((var_guard1149 != 0.0) && (var_guard1150 != 0.0)) {
        let assign34940_e49910: f64 = (var_muun / var_mud_hoso);
        let assign34940_e49912: f64 = (assign34940_e49910 - var_mumoda);
        let assign34940_e49914: f64 = (assign34940_e49912 / var_vds);
        (assign34940_e49914, (((((((var_muun_dn0 * var_mud_hoso) - (var_muun * var_mud_hoso_dn0)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn0) * var_vds) - (assign34940_e49912 * var_vds_dn0)) / (var_vds * var_vds)), (((((((var_muun_dn2 * var_mud_hoso) - (var_muun * var_mud_hoso_dn2)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn2) * var_vds) - (assign34940_e49912 * var_vds_dn2)) / (var_vds * var_vds)), (((((((var_muun_dn6 * var_mud_hoso) - (var_muun * var_mud_hoso_dn6)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn6) * var_vds) - (assign34940_e49912 * var_vds_dn6)) / (var_vds * var_vds)), (((((((var_muun_dn7 * var_mud_hoso) - (var_muun * var_mud_hoso_dn7)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn7) * var_vds) - (assign34940_e49912 * var_vds_dn7)) / (var_vds * var_vds)), (((((((var_muun_dn10 * var_mud_hoso) - (var_muun * var_mud_hoso_dn10)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn10) * var_vds) - (assign34940_e49912 * var_vds_dn10)) / (var_vds * var_vds)), (((((((var_muun_dn11 * var_mud_hoso) - (var_muun * var_mud_hoso_dn11)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn11) * var_vds) - (assign34940_e49912 * var_vds_dn11)) / (var_vds * var_vds)), (((((((var_muun_dn12 * var_mud_hoso) - (var_muun * var_mud_hoso_dn12)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn12) * var_vds) - (assign34940_e49912 * var_vds_dn12)) / (var_vds * var_vds)), (((((((var_muun_dn17 * var_mud_hoso) - (var_muun * var_mud_hoso_dn17)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn17) * var_vds) - (assign34940_e49912 * var_vds_dn17)) / (var_vds * var_vds)),)
    } else {
        (var_mumodb, var_mumodb_dn0, var_mumodb_dn2, var_mumodb_dn6, var_mumodb_dn7, var_mumodb_dn10, var_mumodb_dn11, var_mumodb_dn12, var_mumodb_dn17,)
    }
};
        var_mumodb = assign34940_e49916;
        var_mumodb_dn0 = assign34940_e49916_d_n0;
        var_mumodb_dn2 = assign34940_e49916_d_n2;
        var_mumodb_dn6 = assign34940_e49916_d_n6;
        var_mumodb_dn7 = assign34940_e49916_d_n7;
        var_mumodb_dn10 = assign34940_e49916_d_n10;
        var_mumodb_dn11 = assign34940_e49916_d_n11;
        var_mumodb_dn12 = assign34940_e49916_d_n12;
        var_mumodb_dn17 = assign34940_e49916_d_n17;

        let (assign34950_e49938, assign34950_e49938_d_n0, assign34950_e49938_d_n2, assign34950_e49938_d_n6, assign34950_e49938_d_n7, assign34950_e49938_d_n10, assign34950_e49938_d_n11, assign34950_e49938_d_n12, assign34950_e49938_d_n17,) = {
    if ((var_guard1149 != 0.0) && (var_guard1150 != 0.0)) {
        let assign34950_e49923: f64 = (0.6666666666666667 * var_mumodb);
        let assign34950_e49927: f64 = (var_vgvt * var_sqrtkusail);
        let assign34950_e49928: f64 = (var_kusai00 + assign34950_e49927);
        let assign34950_e49930: f64 = (assign34950_e49928 + var_kusail);
        let assign34950_e49931: f64 = (assign34950_e49923 * assign34950_e49930);
        let assign34950_e49934: f64 = (var_vgvt + var_sqrtkusail);
        let assign34950_e49935: f64 = (assign34950_e49931 / assign34950_e49934);
        let assign34950_e49936: f64 = (var_mumoda + assign34950_e49935);
        (assign34950_e49936, (var_mumoda_dn0 + ((((((0.6666666666666667 * var_mumodb_dn0) * assign34950_e49930) + (assign34950_e49923 * ((var_kusai00_dn0 + ((var_vgvt_dn0 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn0))) + var_kusail_dn0))) * assign34950_e49934) - (assign34950_e49931 * (var_vgvt_dn0 + var_sqrtkusail_dn0))) / (assign34950_e49934 * assign34950_e49934))), (var_mumoda_dn2 + ((((((0.6666666666666667 * var_mumodb_dn2) * assign34950_e49930) + (assign34950_e49923 * ((var_kusai00_dn2 + ((var_vgvt_dn2 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn2))) + var_kusail_dn2))) * assign34950_e49934) - (assign34950_e49931 * (var_vgvt_dn2 + var_sqrtkusail_dn2))) / (assign34950_e49934 * assign34950_e49934))), (var_mumoda_dn6 + ((((((0.6666666666666667 * var_mumodb_dn6) * assign34950_e49930) + (assign34950_e49923 * ((var_kusai00_dn6 + ((var_vgvt_dn6 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn6))) + var_kusail_dn6))) * assign34950_e49934) - (assign34950_e49931 * (var_vgvt_dn6 + var_sqrtkusail_dn6))) / (assign34950_e49934 * assign34950_e49934))), (var_mumoda_dn7 + ((((((0.6666666666666667 * var_mumodb_dn7) * assign34950_e49930) + (assign34950_e49923 * ((var_kusai00_dn7 + ((var_vgvt_dn7 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn7))) + var_kusail_dn7))) * assign34950_e49934) - (assign34950_e49931 * (var_vgvt_dn7 + var_sqrtkusail_dn7))) / (assign34950_e49934 * assign34950_e49934))), (var_mumoda_dn10 + ((((((0.6666666666666667 * var_mumodb_dn10) * assign34950_e49930) + (assign34950_e49923 * ((var_kusai00_dn10 + ((var_vgvt_dn10 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn10))) + var_kusail_dn10))) * assign34950_e49934) - (assign34950_e49931 * (var_vgvt_dn10 + var_sqrtkusail_dn10))) / (assign34950_e49934 * assign34950_e49934))), (var_mumoda_dn11 + ((((((0.6666666666666667 * var_mumodb_dn11) * assign34950_e49930) + (assign34950_e49923 * ((var_kusai00_dn11 + ((var_vgvt_dn11 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn11))) + var_kusail_dn11))) * assign34950_e49934) - (assign34950_e49931 * (var_vgvt_dn11 + var_sqrtkusail_dn11))) / (assign34950_e49934 * assign34950_e49934))), (var_mumoda_dn12 + ((((((0.6666666666666667 * var_mumodb_dn12) * assign34950_e49930) + (assign34950_e49923 * ((var_kusai00_dn12 + ((var_vgvt_dn12 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn12))) + var_kusail_dn12))) * assign34950_e49934) - (assign34950_e49931 * (var_vgvt_dn12 + var_sqrtkusail_dn12))) / (assign34950_e49934 * assign34950_e49934))), (var_mumoda_dn17 + ((((((0.6666666666666667 * var_mumodb_dn17) * assign34950_e49930) + (assign34950_e49923 * ((var_kusai00_dn17 + ((var_vgvt_dn17 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn17))) + var_kusail_dn17))) * assign34950_e49934) - (assign34950_e49931 * (var_vgvt_dn17 + var_sqrtkusail_dn17))) / (assign34950_e49934 * assign34950_e49934))),)
    } else {
        (var_correct_w1, var_correct_w1_dn0, var_correct_w1_dn2, var_correct_w1_dn6, var_correct_w1_dn7, var_correct_w1_dn10, var_correct_w1_dn11, var_correct_w1_dn12, var_correct_w1_dn17,)
    }
};
        var_correct_w1 = assign34950_e49938;
        var_correct_w1_dn0 = assign34950_e49938_d_n0;
        var_correct_w1_dn2 = assign34950_e49938_d_n2;
        var_correct_w1_dn6 = assign34950_e49938_d_n6;
        var_correct_w1_dn7 = assign34950_e49938_d_n7;
        var_correct_w1_dn10 = assign34950_e49938_d_n10;
        var_correct_w1_dn11 = assign34950_e49938_d_n11;
        var_correct_w1_dn12 = assign34950_e49938_d_n12;
        var_correct_w1_dn17 = assign34950_e49938_d_n17;

        let (assign34960_e49947, assign34960_e49947_d_n0, assign34960_e49947_d_n2, assign34960_e49947_d_n6, assign34960_e49947_d_n7, assign34960_e49947_d_n10, assign34960_e49947_d_n11, assign34960_e49947_d_n12, assign34960_e49947_d_n17,) = {
    if ((var_guard1149 != 0.0) && (var_guard1150 == 0.0)) {
        let assign34960_e49945: f64 = (var_muun / var_mud_hoso);
        (assign34960_e49945, (((var_muun_dn0 * var_mud_hoso) - (var_muun * var_mud_hoso_dn0)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn2 * var_mud_hoso) - (var_muun * var_mud_hoso_dn2)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn6 * var_mud_hoso) - (var_muun * var_mud_hoso_dn6)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn7 * var_mud_hoso) - (var_muun * var_mud_hoso_dn7)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn10 * var_mud_hoso) - (var_muun * var_mud_hoso_dn10)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn11 * var_mud_hoso) - (var_muun * var_mud_hoso_dn11)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn12 * var_mud_hoso) - (var_muun * var_mud_hoso_dn12)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn17 * var_mud_hoso) - (var_muun * var_mud_hoso_dn17)) / (var_mud_hoso * var_mud_hoso)),)
    } else {
        (var_correct_w1, var_correct_w1_dn0, var_correct_w1_dn2, var_correct_w1_dn6, var_correct_w1_dn7, var_correct_w1_dn10, var_correct_w1_dn11, var_correct_w1_dn12, var_correct_w1_dn17,)
    }
};
        var_correct_w1 = assign34960_e49947;
        var_correct_w1_dn0 = assign34960_e49947_d_n0;
        var_correct_w1_dn2 = assign34960_e49947_d_n2;
        var_correct_w1_dn6 = assign34960_e49947_d_n6;
        var_correct_w1_dn7 = assign34960_e49947_d_n7;
        var_correct_w1_dn10 = assign34960_e49947_d_n10;
        var_correct_w1_dn11 = assign34960_e49947_d_n11;
        var_correct_w1_dn12 = assign34960_e49947_d_n12;
        var_correct_w1_dn17 = assign34960_e49947_d_n17;

        let (assign34970_e49957, assign34970_e49957_d_n0, assign34970_e49957_d_n2, assign34970_e49957_d_n6, assign34970_e49957_d_n7, assign34970_e49957_d_n10, assign34970_e49957_d_n11, assign34970_e49957_d_n12, assign34970_e49957_d_n13, assign34970_e49957_d_n15, assign34970_e49957_d_n16, assign34970_e49957_d_n17, assign34970_e49957_d_n18,) = {
    if (var_guard1149 != 0.0) {
        let assign34970_e49951: f64 = (var_mfactor * var_nign0);
        let assign34970_e49953: f64 = (assign34970_e49951 * var_kusai_ig);
        let assign34970_e49955: f64 = (assign34970_e49953 * var_correct_w1);
        (assign34970_e49955, (((((var_mfactor * var_nign0_dn0) * var_kusai_ig) + (assign34970_e49951 * var_kusai_ig_dn0)) * var_correct_w1) + (assign34970_e49953 * var_correct_w1_dn0)), (((((var_mfactor * var_nign0_dn2) * var_kusai_ig) + (assign34970_e49951 * var_kusai_ig_dn2)) * var_correct_w1) + (assign34970_e49953 * var_correct_w1_dn2)), (((((var_mfactor * var_nign0_dn6) * var_kusai_ig) + (assign34970_e49951 * var_kusai_ig_dn6)) * var_correct_w1) + (assign34970_e49953 * var_correct_w1_dn6)), (((((var_mfactor * var_nign0_dn7) * var_kusai_ig) + (assign34970_e49951 * var_kusai_ig_dn7)) * var_correct_w1) + (assign34970_e49953 * var_correct_w1_dn7)), (((((var_mfactor * var_nign0_dn10) * var_kusai_ig) + (assign34970_e49951 * var_kusai_ig_dn10)) * var_correct_w1) + (assign34970_e49953 * var_correct_w1_dn10)), (((((var_mfactor * var_nign0_dn11) * var_kusai_ig) + (assign34970_e49951 * var_kusai_ig_dn11)) * var_correct_w1) + (assign34970_e49953 * var_correct_w1_dn11)), (((((var_mfactor * var_nign0_dn12) * var_kusai_ig) + (assign34970_e49951 * var_kusai_ig_dn12)) * var_correct_w1) + (assign34970_e49953 * var_correct_w1_dn12)), (((var_mfactor * var_nign0_dn13) * var_kusai_ig) * var_correct_w1), (((var_mfactor * var_nign0_dn15) * var_kusai_ig) * var_correct_w1), (((var_mfactor * var_nign0_dn16) * var_kusai_ig) * var_correct_w1), (((((var_mfactor * var_nign0_dn17) * var_kusai_ig) + (assign34970_e49951 * var_kusai_ig_dn17)) * var_correct_w1) + (assign34970_e49953 * var_correct_w1_dn17)), (((var_mfactor * var_nign0_dn18) * var_kusai_ig) * var_correct_w1),)
    } else {
        (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn6, var_noiigate_dn7, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12, var_noiigate_dn13, var_noiigate_dn15, var_noiigate_dn16, var_noiigate_dn17, var_noiigate_dn18,)
    }
};
        var_noiigate = assign34970_e49957;
        var_noiigate_dn0 = assign34970_e49957_d_n0;
        var_noiigate_dn2 = assign34970_e49957_d_n2;
        var_noiigate_dn6 = assign34970_e49957_d_n6;
        var_noiigate_dn7 = assign34970_e49957_d_n7;
        var_noiigate_dn10 = assign34970_e49957_d_n10;
        var_noiigate_dn11 = assign34970_e49957_d_n11;
        var_noiigate_dn12 = assign34970_e49957_d_n12;
        var_noiigate_dn13 = assign34970_e49957_d_n13;
        var_noiigate_dn15 = assign34970_e49957_d_n15;
        var_noiigate_dn16 = assign34970_e49957_d_n16;
        var_noiigate_dn17 = assign34970_e49957_d_n17;
        var_noiigate_dn18 = assign34970_e49957_d_n18;

        let (assign34980_e49961, assign34980_e49961_d_n0, assign34980_e49961_d_n2, assign34980_e49961_d_n6, assign34980_e49961_d_n7, assign34980_e49961_d_n10, assign34980_e49961_d_n11, assign34980_e49961_d_n12, assign34980_e49961_d_n17,) = {
    if (var_guard1149 != 0.0) {
        (var_crl_f, var_crl_f_dn0, var_crl_f_dn2, var_crl_f_dn6, var_crl_f_dn7, var_crl_f_dn10, var_crl_f_dn11, var_crl_f_dn12, var_crl_f_dn17,)
    } else {
        (var_noicross, var_noicross_dn0, var_noicross_dn2, var_noicross_dn6, var_noicross_dn7, var_noicross_dn10, var_noicross_dn11, var_noicross_dn12, var_noicross_dn17,)
    }
};
        var_noicross = assign34980_e49961;
        var_noicross_dn0 = assign34980_e49961_d_n0;
        var_noicross_dn2 = assign34980_e49961_d_n2;
        var_noicross_dn6 = assign34980_e49961_d_n6;
        var_noicross_dn7 = assign34980_e49961_d_n7;
        var_noicross_dn10 = assign34980_e49961_d_n10;
        var_noicross_dn11 = assign34980_e49961_d_n11;
        var_noicross_dn12 = assign34980_e49961_d_n12;
        var_noicross_dn17 = assign34980_e49961_d_n17;

        let (assign34990_e49975, assign34990_e49975_d_n0, assign34990_e49975_d_n2, assign34990_e49975_d_n6, assign34990_e49975_d_n7, assign34990_e49975_d_n10, assign34990_e49975_d_n11, assign34990_e49975_d_n12, assign34990_e49975_d_n13, assign34990_e49975_d_n15, assign34990_e49975_d_n16, assign34990_e49975_d_n17, assign34990_e49975_d_n18,) = {
    if (var_guard1149 != 0.0) {
        let assign34990_e49964: f64 = (-var_t1__blk1144);
        let (assign34990_e49973, assign34990_e49973_d_n0, assign34990_e49973_d_n2, assign34990_e49973_d_n6, assign34990_e49973_d_n7, assign34990_e49973_d_n10, assign34990_e49973_d_n11, assign34990_e49973_d_n12, assign34990_e49973_d_n13, assign34990_e49973_d_n15, assign34990_e49973_d_n16, assign34990_e49973_d_n17, assign34990_e49973_d_n18,) = {
            if ((assign34990_e49964 > var_t0__blk1143) && (var_noiigate > 0.0)) {
                (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn6, var_noiigate_dn7, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12, var_noiigate_dn13, var_noiigate_dn15, var_noiigate_dn16, var_noiigate_dn17, var_noiigate_dn18,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign34990_e49973, assign34990_e49973_d_n0, assign34990_e49973_d_n2, assign34990_e49973_d_n6, assign34990_e49973_d_n7, assign34990_e49973_d_n10, assign34990_e49973_d_n11, assign34990_e49973_d_n12, assign34990_e49973_d_n13, assign34990_e49973_d_n15, assign34990_e49973_d_n16, assign34990_e49973_d_n17, assign34990_e49973_d_n18,)
    } else {
        (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn6, var_noiigate_dn7, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12, var_noiigate_dn13, var_noiigate_dn15, var_noiigate_dn16, var_noiigate_dn17, var_noiigate_dn18,)
    }
};
        var_noiigate = assign34990_e49975;
        var_noiigate_dn0 = assign34990_e49975_d_n0;
        var_noiigate_dn2 = assign34990_e49975_d_n2;
        var_noiigate_dn6 = assign34990_e49975_d_n6;
        var_noiigate_dn7 = assign34990_e49975_d_n7;
        var_noiigate_dn10 = assign34990_e49975_d_n10;
        var_noiigate_dn11 = assign34990_e49975_d_n11;
        var_noiigate_dn12 = assign34990_e49975_d_n12;
        var_noiigate_dn13 = assign34990_e49975_d_n13;
        var_noiigate_dn15 = assign34990_e49975_d_n15;
        var_noiigate_dn16 = assign34990_e49975_d_n16;
        var_noiigate_dn17 = assign34990_e49975_d_n17;
        var_noiigate_dn18 = assign34990_e49975_d_n18;

        let (assign35000_e49985, assign35000_e49985_d_n0, assign35000_e49985_d_n2, assign35000_e49985_d_n6, assign35000_e49985_d_n7, assign35000_e49985_d_n10, assign35000_e49985_d_n11, assign35000_e49985_d_n12, assign35000_e49985_d_n17,) = {
    if (var_guard1149 != 0.0) {
        let assign35000_e49978: f64 = (-var_t1__blk1144);
        let (assign35000_e49983, assign35000_e49983_d_n0, assign35000_e49983_d_n2, assign35000_e49983_d_n6, assign35000_e49983_d_n7, assign35000_e49983_d_n10, assign35000_e49983_d_n11, assign35000_e49983_d_n12, assign35000_e49983_d_n17,) = {
            if (assign35000_e49978 > var_t0__blk1143) {
                (var_noicross, var_noicross_dn0, var_noicross_dn2, var_noicross_dn6, var_noicross_dn7, var_noicross_dn10, var_noicross_dn11, var_noicross_dn12, var_noicross_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign35000_e49983, assign35000_e49983_d_n0, assign35000_e49983_d_n2, assign35000_e49983_d_n6, assign35000_e49983_d_n7, assign35000_e49983_d_n10, assign35000_e49983_d_n11, assign35000_e49983_d_n12, assign35000_e49983_d_n17,)
    } else {
        (var_noicross, var_noicross_dn0, var_noicross_dn2, var_noicross_dn6, var_noicross_dn7, var_noicross_dn10, var_noicross_dn11, var_noicross_dn12, var_noicross_dn17,)
    }
};
        var_noicross = assign35000_e49985;
        var_noicross_dn0 = assign35000_e49985_d_n0;
        var_noicross_dn2 = assign35000_e49985_d_n2;
        var_noicross_dn6 = assign35000_e49985_d_n6;
        var_noicross_dn7 = assign35000_e49985_d_n7;
        var_noicross_dn10 = assign35000_e49985_d_n10;
        var_noicross_dn11 = assign35000_e49985_d_n11;
        var_noicross_dn12 = assign35000_e49985_d_n12;
        var_noicross_dn17 = assign35000_e49985_d_n17;

        let (assign35010_e49990, assign35010_e49990_d_n0, assign35010_e49990_d_n2, assign35010_e49990_d_n6, assign35010_e49990_d_n7, assign35010_e49990_d_n10, assign35010_e49990_d_n11, assign35010_e49990_d_n12, assign35010_e49990_d_n13, assign35010_e49990_d_n15, assign35010_e49990_d_n16, assign35010_e49990_d_n17, assign35010_e49990_d_n18,) = {
    if (var_guard1149 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn6, var_noiigate_dn7, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12, var_noiigate_dn13, var_noiigate_dn15, var_noiigate_dn16, var_noiigate_dn17, var_noiigate_dn18,)
    }
};
        var_noiigate = assign35010_e49990;
        var_noiigate_dn0 = assign35010_e49990_d_n0;
        var_noiigate_dn2 = assign35010_e49990_d_n2;
        var_noiigate_dn6 = assign35010_e49990_d_n6;
        var_noiigate_dn7 = assign35010_e49990_d_n7;
        var_noiigate_dn10 = assign35010_e49990_d_n10;
        var_noiigate_dn11 = assign35010_e49990_d_n11;
        var_noiigate_dn12 = assign35010_e49990_d_n12;
        var_noiigate_dn13 = assign35010_e49990_d_n13;
        var_noiigate_dn15 = assign35010_e49990_d_n15;
        var_noiigate_dn16 = assign35010_e49990_d_n16;
        var_noiigate_dn17 = assign35010_e49990_d_n17;
        var_noiigate_dn18 = assign35010_e49990_d_n18;

        let (assign35020_e49995, assign35020_e49995_d_n0, assign35020_e49995_d_n2, assign35020_e49995_d_n6, assign35020_e49995_d_n7, assign35020_e49995_d_n10, assign35020_e49995_d_n11, assign35020_e49995_d_n12, assign35020_e49995_d_n17,) = {
    if (var_guard1149 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_noicross, var_noicross_dn0, var_noicross_dn2, var_noicross_dn6, var_noicross_dn7, var_noicross_dn10, var_noicross_dn11, var_noicross_dn12, var_noicross_dn17,)
    }
};
        var_noicross = assign35020_e49995;
        var_noicross_dn0 = assign35020_e49995_d_n0;
        var_noicross_dn2 = assign35020_e49995_d_n2;
        var_noicross_dn6 = assign35020_e49995_d_n6;
        var_noicross_dn7 = assign35020_e49995_d_n7;
        var_noicross_dn10 = assign35020_e49995_d_n10;
        var_noicross_dn11 = assign35020_e49995_d_n11;
        var_noicross_dn12 = assign35020_e49995_d_n12;
        var_noicross_dn17 = assign35020_e49995_d_n17;

        var_rdde = 0.0;
        var_rdde_dn0 = 0.0;
        var_rdde_dn2 = 0.0;
        var_rdde_dn6 = 0.0;
        var_rdde_dn7 = 0.0;
        var_rdde_dn10 = 0.0;
        var_rdde_dn11 = 0.0;
        var_rdde_dn12 = 0.0;
        var_rdde_dn17 = 0.0;

        var_rsde = 0.0;
        var_rsde_dn0 = 0.0;
        var_rsde_dn2 = 0.0;
        var_rsde_dn6 = 0.0;
        var_rsde_dn7 = 0.0;
        var_rsde_dn10 = 0.0;
        var_rsde_dn11 = 0.0;
        var_rsde_dn12 = 0.0;
        var_rsde_dn17 = 0.0;

        let assign35070_e50002: f64 = if p.p259 == 1.0 { 1.0 } else { 0.0 };
        var_guard1151 = assign35070_e50002;

        let (assign35080_e50006,) = {
    if (var_guard1151 != 0.0) {
        (1.0,)
    } else {
        (var_rdmod,)
    }
};
        var_rdmod = assign35080_e50006;

        let assign35090_e50009: f64 = if var_rdmod == 1.0 { 1.0 } else { 0.0 };
        var_guard1171 = assign35090_e50009;

        let (assign35100_e50017,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 != 0.0)) {
        let assign35100_e50015: f64 = (p.p264 / 1e-6);
        (assign35100_e50015,)
    } else {
        (var_nover,)
    }
};
        var_nover = assign35100_e50017;

        let (assign35110_e50023,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 != 0.0)) {
        (p.p266,)
    } else {
        (var_mks_rdrmue,)
    }
};
        var_mks_rdrmue = assign35110_e50023;

        let (assign35120_e50029,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 != 0.0)) {
        (p.p268,)
    } else {
        (var_mks_rdrvmax,)
    }
};
        var_mks_rdrvmax = assign35120_e50029;

        let (assign35130_e50035, assign35130_e50035_d_n10,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (var_rrdrbb, var_rrdrbb_dn10,)
    }
};
        var_rrdrbb = assign35130_e50035;
        var_rrdrbb_dn10 = assign35130_e50035_d_n10;

        let (assign35140_e50048,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 != 0.0)) {
        let (assign35140_e50046,) = {
            if (p.p263 > 0.0) {
                let assign35140_e50044: f64 = (p.p263 * p.p255);
                (assign35140_e50044,)
            } else {
                (0.0,)
            }
        };
        (assign35140_e50046,)
    } else {
        (var_rsd0,)
    }
};
        var_rsd0 = assign35140_e50048;

        let (assign35150_e50054,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 != 0.0)) {
        (p.p258,)
    } else {
        (var_ldrifte,)
    }
};
        var_ldrifte = assign35150_e50054;

        let (assign35160_e50062, assign35160_e50062_d_n0, assign35160_e50062_d_n2, assign35160_e50062_d_n6, assign35160_e50062_d_n7,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 != 0.0)) {
        let assign35160_e50060: f64 = (p.p50 * (nv7 - nv2));
        (assign35160_e50060, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (var_vrdr, var_vrdr_dn0, var_vrdr_dn2, var_vrdr_dn6, var_vrdr_dn7,)
    }
};
        var_vrdr = assign35160_e50062;
        var_vrdr_dn0 = assign35160_e50062_d_n0;
        var_vrdr_dn2 = assign35160_e50062_d_n2;
        var_vrdr_dn6 = assign35160_e50062_d_n6;
        var_vrdr_dn7 = assign35160_e50062_d_n7;

        let (assign35170_e50071,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 == 0.0)) {
        let assign35170_e50069: f64 = (p.p59 / 1e-6);
        (assign35170_e50069,)
    } else {
        (var_nover,)
    }
};
        var_nover = assign35170_e50071;

        let (assign35180_e50078,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 == 0.0)) {
        (p.p265,)
    } else {
        (var_mks_rdrmue,)
    }
};
        var_mks_rdrmue = assign35180_e50078;

        let (assign35190_e50085,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 == 0.0)) {
        (p.p267,)
    } else {
        (var_mks_rdrvmax,)
    }
};
        var_mks_rdrvmax = assign35190_e50085;

        let (assign35200_e50092, assign35200_e50092_d_n10,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (var_rrdrbb, var_rrdrbb_dn10,)
    }
};
        var_rrdrbb = assign35200_e50092;
        var_rrdrbb_dn10 = assign35200_e50092_d_n10;

        let (assign35210_e50106,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 == 0.0)) {
        let (assign35210_e50104,) = {
            if (p.p263 > 0.0) {
                let assign35210_e50102: f64 = (p.p263 * p.p256);
                (assign35210_e50102,)
            } else {
                (0.0,)
            }
        };
        (assign35210_e50104,)
    } else {
        (var_rsd0,)
    }
};
        var_rsd0 = assign35210_e50106;

        let (assign35220_e50113,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 == 0.0)) {
        (p.p257,)
    } else {
        (var_ldrifte,)
    }
};
        var_ldrifte = assign35220_e50113;

        let (assign35230_e50122, assign35230_e50122_d_n0, assign35230_e50122_d_n2, assign35230_e50122_d_n6, assign35230_e50122_d_n7,) = {
    if ((var_guard1151 != 0.0) && (var_guard1171 == 0.0)) {
        let assign35230_e50120: f64 = (p.p50 * (nv0 - nv6));
        (assign35230_e50120, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (var_vrdr, var_vrdr_dn0, var_vrdr_dn2, var_vrdr_dn6, var_vrdr_dn7,)
    }
};
        var_vrdr = assign35230_e50122;
        var_vrdr_dn0 = assign35230_e50122_d_n0;
        var_vrdr_dn2 = assign35230_e50122_d_n2;
        var_vrdr_dn6 = assign35230_e50122_d_n6;
        var_vrdr_dn7 = assign35230_e50122_d_n7;

        let (assign35240_e50133,) = {
    if (var_guard1151 != 0.0) {
        let assign35240_e50126: f64 = (p.p271 * p.p271);
        let assign35240_e50129: f64 = (p.p56 * p.p56);
        let assign35240_e50130: f64 = (assign35240_e50126 + assign35240_e50129);
        let assign35240_e50131: f64 = (assign35240_e50130).sqrt();
        (assign35240_e50131,)
    } else {
        (var_xov,)
    }
};
        var_xov = assign35240_e50133;

        let (assign35250_e50139,) = {
    if (var_guard1151 != 0.0) {
        let assign35250_e50137: f64 = (var_weff * p.p9);
        (assign35250_e50137,)
    } else {
        (var_weff_nf_1,)
    }
};
        var_weff_nf_1 = assign35250_e50139;

        let (assign35260_e50145,) = {
    if (var_guard1151 != 0.0) {
        let assign35260_e50143: f64 = (var_mks_rdrmue / 10000.0);
        (assign35260_e50143,)
    } else {
        (var_mks_rdrmue,)
    }
};
        var_mks_rdrmue = assign35260_e50145;

        let (assign35270_e50151,) = {
    if (var_guard1151 != 0.0) {
        let assign35270_e50149: f64 = (var_mks_rdrvmax / 100.0);
        (assign35270_e50149,)
    } else {
        (var_mks_rdrvmax,)
    }
};
        var_mks_rdrvmax = assign35270_e50151;

        let (assign35280_e50157, assign35280_e50157_d_n10,) = {
    if (var_guard1151 != 0.0) {
        let assign35280_e50155: f64 = (var_ttemp / var_uc_tnom);
        (assign35280_e50155, (var_ttemp_dn10 / var_uc_tnom),)
    } else {
        (var_tratio, var_tratio_dn10,)
    }
};
        var_tratio = assign35280_e50157;
        var_tratio_dn10 = assign35280_e50157_d_n10;

        *var_correct_w1_slot = var_correct_w1;
        *var_correct_w1_dn0_slot = var_correct_w1_dn0;
        *var_correct_w1_dn10_slot = var_correct_w1_dn10;
        *var_correct_w1_dn11_slot = var_correct_w1_dn11;
        *var_correct_w1_dn12_slot = var_correct_w1_dn12;
        *var_correct_w1_dn17_slot = var_correct_w1_dn17;
        *var_correct_w1_dn2_slot = var_correct_w1_dn2;
        *var_correct_w1_dn6_slot = var_correct_w1_dn6;
        *var_correct_w1_dn7_slot = var_correct_w1_dn7;
        *var_guard1150_slot = var_guard1150;
        *var_guard1151_slot = var_guard1151;
        *var_guard1171_slot = var_guard1171;
        *var_ldrifte_slot = var_ldrifte;
        *var_mks_rdrmue_slot = var_mks_rdrmue;
        *var_mks_rdrvmax_slot = var_mks_rdrvmax;
        *var_mumoda_slot = var_mumoda;
        *var_mumoda_dn0_slot = var_mumoda_dn0;
        *var_mumoda_dn10_slot = var_mumoda_dn10;
        *var_mumoda_dn11_slot = var_mumoda_dn11;
        *var_mumoda_dn12_slot = var_mumoda_dn12;
        *var_mumoda_dn17_slot = var_mumoda_dn17;
        *var_mumoda_dn2_slot = var_mumoda_dn2;
        *var_mumoda_dn6_slot = var_mumoda_dn6;
        *var_mumoda_dn7_slot = var_mumoda_dn7;
        *var_mumodb_slot = var_mumodb;
        *var_mumodb_dn0_slot = var_mumodb_dn0;
        *var_mumodb_dn10_slot = var_mumodb_dn10;
        *var_mumodb_dn11_slot = var_mumodb_dn11;
        *var_mumodb_dn12_slot = var_mumodb_dn12;
        *var_mumodb_dn17_slot = var_mumodb_dn17;
        *var_mumodb_dn2_slot = var_mumodb_dn2;
        *var_mumodb_dn6_slot = var_mumodb_dn6;
        *var_mumodb_dn7_slot = var_mumodb_dn7;
        *var_nign0_slot = var_nign0;
        *var_nign0_dn0_slot = var_nign0_dn0;
        *var_nign0_dn10_slot = var_nign0_dn10;
        *var_nign0_dn11_slot = var_nign0_dn11;
        *var_nign0_dn12_slot = var_nign0_dn12;
        *var_nign0_dn13_slot = var_nign0_dn13;
        *var_nign0_dn15_slot = var_nign0_dn15;
        *var_nign0_dn16_slot = var_nign0_dn16;
        *var_nign0_dn17_slot = var_nign0_dn17;
        *var_nign0_dn18_slot = var_nign0_dn18;
        *var_nign0_dn2_slot = var_nign0_dn2;
        *var_nign0_dn6_slot = var_nign0_dn6;
        *var_nign0_dn7_slot = var_nign0_dn7;
        *var_noicross_slot = var_noicross;
        *var_noicross_dn0_slot = var_noicross_dn0;
        *var_noicross_dn10_slot = var_noicross_dn10;
        *var_noicross_dn11_slot = var_noicross_dn11;
        *var_noicross_dn12_slot = var_noicross_dn12;
        *var_noicross_dn17_slot = var_noicross_dn17;
        *var_noicross_dn2_slot = var_noicross_dn2;
        *var_noicross_dn6_slot = var_noicross_dn6;
        *var_noicross_dn7_slot = var_noicross_dn7;
        *var_noiigate_slot = var_noiigate;
        *var_noiigate_dn0_slot = var_noiigate_dn0;
        *var_noiigate_dn10_slot = var_noiigate_dn10;
        *var_noiigate_dn11_slot = var_noiigate_dn11;
        *var_noiigate_dn12_slot = var_noiigate_dn12;
        *var_noiigate_dn13_slot = var_noiigate_dn13;
        *var_noiigate_dn15_slot = var_noiigate_dn15;
        *var_noiigate_dn16_slot = var_noiigate_dn16;
        *var_noiigate_dn17_slot = var_noiigate_dn17;
        *var_noiigate_dn18_slot = var_noiigate_dn18;
        *var_noiigate_dn2_slot = var_noiigate_dn2;
        *var_noiigate_dn6_slot = var_noiigate_dn6;
        *var_noiigate_dn7_slot = var_noiigate_dn7;
        *var_nover_slot = var_nover;
        *var_rdde_slot = var_rdde;
        *var_rdde_dn0_slot = var_rdde_dn0;
        *var_rdde_dn10_slot = var_rdde_dn10;
        *var_rdde_dn11_slot = var_rdde_dn11;
        *var_rdde_dn12_slot = var_rdde_dn12;
        *var_rdde_dn17_slot = var_rdde_dn17;
        *var_rdde_dn2_slot = var_rdde_dn2;
        *var_rdde_dn6_slot = var_rdde_dn6;
        *var_rdde_dn7_slot = var_rdde_dn7;
        *var_rdmod_slot = var_rdmod;
        *var_rrdrbb_slot = var_rrdrbb;
        *var_rrdrbb_dn10_slot = var_rrdrbb_dn10;
        *var_rsd0_slot = var_rsd0;
        *var_rsde_slot = var_rsde;
        *var_rsde_dn0_slot = var_rsde_dn0;
        *var_rsde_dn10_slot = var_rsde_dn10;
        *var_rsde_dn11_slot = var_rsde_dn11;
        *var_rsde_dn12_slot = var_rsde_dn12;
        *var_rsde_dn17_slot = var_rsde_dn17;
        *var_rsde_dn2_slot = var_rsde_dn2;
        *var_rsde_dn6_slot = var_rsde_dn6;
        *var_rsde_dn7_slot = var_rsde_dn7;
        *var_t1__blk1144_slot = var_t1__blk1144;
        *var_t1__blk1144_dn0_slot = var_t1__blk1144_dn0;
        *var_t1__blk1144_dn10_slot = var_t1__blk1144_dn10;
        *var_t1__blk1144_dn11_slot = var_t1__blk1144_dn11;
        *var_t1__blk1144_dn12_slot = var_t1__blk1144_dn12;
        *var_t1__blk1144_dn13_slot = var_t1__blk1144_dn13;
        *var_t1__blk1144_dn15_slot = var_t1__blk1144_dn15;
        *var_t1__blk1144_dn16_slot = var_t1__blk1144_dn16;
        *var_t1__blk1144_dn17_slot = var_t1__blk1144_dn17;
        *var_t1__blk1144_dn18_slot = var_t1__blk1144_dn18;
        *var_t1__blk1144_dn2_slot = var_t1__blk1144_dn2;
        *var_t1__blk1144_dn6_slot = var_t1__blk1144_dn6;
        *var_t1__blk1144_dn7_slot = var_t1__blk1144_dn7;
        *var_tratio_slot = var_tratio;
        *var_tratio_dn10_slot = var_tratio_dn10;
        *var_vrdr_slot = var_vrdr;
        *var_vrdr_dn0_slot = var_vrdr_dn0;
        *var_vrdr_dn2_slot = var_vrdr_dn2;
        *var_vrdr_dn6_slot = var_vrdr_dn6;
        *var_vrdr_dn7_slot = var_vrdr_dn7;
        *var_weff_nf_1_slot = var_weff_nf_1;
        *var_xov_slot = var_xov;
    }

    pub(super) fn stamp_transient_block_123(
        p: &Parameters,
        var_guard1151: f64,
        var_ldrifte: f64,
        var_lgle: f64,
        var_mks_rdrmue: f64,
        var_mks_rdrvmax: f64,
        var_nover: f64,
        var_tratio: f64,
        var_tratio_dn10: f64,
        var_ttemp: f64,
        var_ttemp_dn10: f64,
        var_uc_tnom: f64,
        var_vrdr: f64,
        var_vrdr_dn0: f64,
        var_vrdr_dn2: f64,
        var_vrdr_dn6: f64,
        var_vrdr_dn7: f64,
        var_wg: f64,
        var_xov: f64,
        var_edri_slot: &mut f64,
        var_edri_dn0_slot: &mut f64,
        var_edri_dn2_slot: &mut f64,
        var_edri_dn6_slot: &mut f64,
        var_edri_dn7_slot: &mut f64,
        var_gd_slot: &mut f64,
        var_gd_dn0_slot: &mut f64,
        var_gd_dn10_slot: &mut f64,
        var_gd_dn11_slot: &mut f64,
        var_gd_dn12_slot: &mut f64,
        var_gd_dn17_slot: &mut f64,
        var_gd_dn2_slot: &mut f64,
        var_gd_dn6_slot: &mut f64,
        var_gd_dn7_slot: &mut f64,
        var_guard1172_slot: &mut f64,
        var_guard1173_slot: &mut f64,
        var_guard1174_slot: &mut f64,
        var_guard1175_slot: &mut f64,
        var_guard1176_slot: &mut f64,
        var_guard1177_slot: &mut f64,
        var_mu0_slot: &mut f64,
        var_mu0_dn0_slot: &mut f64,
        var_mu0_dn10_slot: &mut f64,
        var_mu0_dn11_slot: &mut f64,
        var_mu0_dn12_slot: &mut f64,
        var_mu0_dn17_slot: &mut f64,
        var_mu0_dn2_slot: &mut f64,
        var_mu0_dn6_slot: &mut f64,
        var_mu0_dn7_slot: &mut f64,
        var_mu__blk1167_slot: &mut f64,
        var_mu__blk1167_dn0_slot: &mut f64,
        var_mu__blk1167_dn10_slot: &mut f64,
        var_mu__blk1167_dn11_slot: &mut f64,
        var_mu__blk1167_dn12_slot: &mut f64,
        var_mu__blk1167_dn17_slot: &mut f64,
        var_mu__blk1167_dn2_slot: &mut f64,
        var_mu__blk1167_dn6_slot: &mut f64,
        var_mu__blk1167_dn7_slot: &mut f64,
        var_rdrmuele_slot: &mut f64,
        var_rdrvmaxle_slot: &mut f64,
        var_rdrvmaxwe_slot: &mut f64,
        var_rrdrbb_slot: &mut f64,
        var_rrdrbb_dn10_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn17_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn17_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn17_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_vdri_slot: &mut f64,
        var_vdri_dn0_slot: &mut f64,
        var_vdri_dn10_slot: &mut f64,
        var_vdri_dn11_slot: &mut f64,
        var_vdri_dn12_slot: &mut f64,
        var_vdri_dn17_slot: &mut f64,
        var_vdri_dn2_slot: &mut f64,
        var_vdri_dn6_slot: &mut f64,
        var_vdri_dn7_slot: &mut f64,
        var_vmaxe__blk1164_slot: &mut f64,
        var_vmaxe__blk1164_dn0_slot: &mut f64,
        var_vmaxe__blk1164_dn10_slot: &mut f64,
        var_vmaxe__blk1164_dn11_slot: &mut f64,
        var_vmaxe__blk1164_dn12_slot: &mut f64,
        var_vmaxe__blk1164_dn17_slot: &mut f64,
        var_vmaxe__blk1164_dn2_slot: &mut f64,
        var_vmaxe__blk1164_dn6_slot: &mut f64,
        var_vmaxe__blk1164_dn7_slot: &mut f64,
    ) {
        let mut var_edri: f64 = *var_edri_slot;
        let mut var_edri_dn0: f64 = *var_edri_dn0_slot;
        let mut var_edri_dn2: f64 = *var_edri_dn2_slot;
        let mut var_edri_dn6: f64 = *var_edri_dn6_slot;
        let mut var_edri_dn7: f64 = *var_edri_dn7_slot;
        let mut var_gd: f64 = *var_gd_slot;
        let mut var_gd_dn0: f64 = *var_gd_dn0_slot;
        let mut var_gd_dn10: f64 = *var_gd_dn10_slot;
        let mut var_gd_dn11: f64 = *var_gd_dn11_slot;
        let mut var_gd_dn12: f64 = *var_gd_dn12_slot;
        let mut var_gd_dn17: f64 = *var_gd_dn17_slot;
        let mut var_gd_dn2: f64 = *var_gd_dn2_slot;
        let mut var_gd_dn6: f64 = *var_gd_dn6_slot;
        let mut var_gd_dn7: f64 = *var_gd_dn7_slot;
        let mut var_guard1172: f64 = *var_guard1172_slot;
        let mut var_guard1173: f64 = *var_guard1173_slot;
        let mut var_guard1174: f64 = *var_guard1174_slot;
        let mut var_guard1175: f64 = *var_guard1175_slot;
        let mut var_guard1176: f64 = *var_guard1176_slot;
        let mut var_guard1177: f64 = *var_guard1177_slot;
        let mut var_mu0: f64 = *var_mu0_slot;
        let mut var_mu0_dn0: f64 = *var_mu0_dn0_slot;
        let mut var_mu0_dn10: f64 = *var_mu0_dn10_slot;
        let mut var_mu0_dn11: f64 = *var_mu0_dn11_slot;
        let mut var_mu0_dn12: f64 = *var_mu0_dn12_slot;
        let mut var_mu0_dn17: f64 = *var_mu0_dn17_slot;
        let mut var_mu0_dn2: f64 = *var_mu0_dn2_slot;
        let mut var_mu0_dn6: f64 = *var_mu0_dn6_slot;
        let mut var_mu0_dn7: f64 = *var_mu0_dn7_slot;
        let mut var_mu__blk1167: f64 = *var_mu__blk1167_slot;
        let mut var_mu__blk1167_dn0: f64 = *var_mu__blk1167_dn0_slot;
        let mut var_mu__blk1167_dn10: f64 = *var_mu__blk1167_dn10_slot;
        let mut var_mu__blk1167_dn11: f64 = *var_mu__blk1167_dn11_slot;
        let mut var_mu__blk1167_dn12: f64 = *var_mu__blk1167_dn12_slot;
        let mut var_mu__blk1167_dn17: f64 = *var_mu__blk1167_dn17_slot;
        let mut var_mu__blk1167_dn2: f64 = *var_mu__blk1167_dn2_slot;
        let mut var_mu__blk1167_dn6: f64 = *var_mu__blk1167_dn6_slot;
        let mut var_mu__blk1167_dn7: f64 = *var_mu__blk1167_dn7_slot;
        let mut var_rdrmuele: f64 = *var_rdrmuele_slot;
        let mut var_rdrvmaxle: f64 = *var_rdrvmaxle_slot;
        let mut var_rdrvmaxwe: f64 = *var_rdrvmaxwe_slot;
        let mut var_rrdrbb: f64 = *var_rrdrbb_slot;
        let mut var_rrdrbb_dn10: f64 = *var_rrdrbb_dn10_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn17: f64 = *var_t4_dn17_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn17: f64 = *var_t5_dn17_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn17: f64 = *var_t6_dn17_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_vdri: f64 = *var_vdri_slot;
        let mut var_vdri_dn0: f64 = *var_vdri_dn0_slot;
        let mut var_vdri_dn10: f64 = *var_vdri_dn10_slot;
        let mut var_vdri_dn11: f64 = *var_vdri_dn11_slot;
        let mut var_vdri_dn12: f64 = *var_vdri_dn12_slot;
        let mut var_vdri_dn17: f64 = *var_vdri_dn17_slot;
        let mut var_vdri_dn2: f64 = *var_vdri_dn2_slot;
        let mut var_vdri_dn6: f64 = *var_vdri_dn6_slot;
        let mut var_vdri_dn7: f64 = *var_vdri_dn7_slot;
        let mut var_vmaxe__blk1164: f64 = *var_vmaxe__blk1164_slot;
        let mut var_vmaxe__blk1164_dn0: f64 = *var_vmaxe__blk1164_dn0_slot;
        let mut var_vmaxe__blk1164_dn10: f64 = *var_vmaxe__blk1164_dn10_slot;
        let mut var_vmaxe__blk1164_dn11: f64 = *var_vmaxe__blk1164_dn11_slot;
        let mut var_vmaxe__blk1164_dn12: f64 = *var_vmaxe__blk1164_dn12_slot;
        let mut var_vmaxe__blk1164_dn17: f64 = *var_vmaxe__blk1164_dn17_slot;
        let mut var_vmaxe__blk1164_dn2: f64 = *var_vmaxe__blk1164_dn2_slot;
        let mut var_vmaxe__blk1164_dn6: f64 = *var_vmaxe__blk1164_dn6_slot;
        let mut var_vmaxe__blk1164_dn7: f64 = *var_vmaxe__blk1164_dn7_slot;

        let (assign35290_e50163, assign35290_e50163_d_n0, assign35290_e50163_d_n2, assign35290_e50163_d_n6, assign35290_e50163_d_n7, assign35290_e50163_d_n10, assign35290_e50163_d_n11, assign35290_e50163_d_n12, assign35290_e50163_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35290_e50161: f64 = (var_tratio).powf(p.p269);
        (assign35290_e50161, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((var_tratio).powf(p.p269 - 1.0) * var_tratio_dn10)) } } else { (assign35290_e50161 * (p.p269 * (var_tratio_dn10 / var_tratio))) }, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35290_e50163;
        var_t1_dn0 = assign35290_e50163_d_n0;
        var_t1_dn2 = assign35290_e50163_d_n2;
        var_t1_dn6 = assign35290_e50163_d_n6;
        var_t1_dn7 = assign35290_e50163_d_n7;
        var_t1_dn10 = assign35290_e50163_d_n10;
        var_t1_dn11 = assign35290_e50163_d_n11;
        var_t1_dn12 = assign35290_e50163_d_n12;
        var_t1_dn17 = assign35290_e50163_d_n17;

        let (assign35300_e50169, assign35300_e50169_d_n0, assign35300_e50169_d_n2, assign35300_e50169_d_n6, assign35300_e50169_d_n7, assign35300_e50169_d_n10, assign35300_e50169_d_n11, assign35300_e50169_d_n12, assign35300_e50169_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35300_e50167: f64 = (var_mks_rdrmue / var_t1);
        (assign35300_e50167, (-((var_mks_rdrmue * var_t1_dn0) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn2) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn6) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn7) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn10) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn11) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn12) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn17) / (var_t1 * var_t1))),)
    } else {
        (var_mu0, var_mu0_dn0, var_mu0_dn2, var_mu0_dn6, var_mu0_dn7, var_mu0_dn10, var_mu0_dn11, var_mu0_dn12, var_mu0_dn17,)
    }
};
        var_mu0 = assign35300_e50169;
        var_mu0_dn0 = assign35300_e50169_d_n0;
        var_mu0_dn2 = assign35300_e50169_d_n2;
        var_mu0_dn6 = assign35300_e50169_d_n6;
        var_mu0_dn7 = assign35300_e50169_d_n7;
        var_mu0_dn10 = assign35300_e50169_d_n10;
        var_mu0_dn11 = assign35300_e50169_d_n11;
        var_mu0_dn12 = assign35300_e50169_d_n12;
        var_mu0_dn17 = assign35300_e50169_d_n17;

        let (assign35310_e50189, assign35310_e50189_d_n0, assign35310_e50189_d_n2, assign35310_e50189_d_n6, assign35310_e50189_d_n7, assign35310_e50189_d_n10, assign35310_e50189_d_n11, assign35310_e50189_d_n12, assign35310_e50189_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35310_e50174: f64 = (0.4 * var_tratio);
        let assign35310_e50175: f64 = (1.8 + assign35310_e50174);
        let assign35310_e50178: f64 = (0.1 * var_tratio);
        let assign35310_e50180: f64 = (assign35310_e50178 * var_tratio);
        let assign35310_e50181: f64 = (assign35310_e50175 + assign35310_e50180);
        let assign35310_e50185: f64 = (1.0 - var_tratio);
        let assign35310_e50186: f64 = (p.p270 * assign35310_e50185);
        let assign35310_e50187: f64 = (assign35310_e50181 - assign35310_e50186);
        (assign35310_e50187, 0.0, 0.0, 0.0, 0.0, (((0.4 * var_tratio_dn10) + (((0.1 * var_tratio_dn10) * var_tratio) + (assign35310_e50178 * var_tratio_dn10))) - (p.p270 * (-var_tratio_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn6, var_t0_dn7, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn17,)
    }
};
        var_t0 = assign35310_e50189;
        var_t0_dn0 = assign35310_e50189_d_n0;
        var_t0_dn2 = assign35310_e50189_d_n2;
        var_t0_dn6 = assign35310_e50189_d_n6;
        var_t0_dn7 = assign35310_e50189_d_n7;
        var_t0_dn10 = assign35310_e50189_d_n10;
        var_t0_dn11 = assign35310_e50189_d_n11;
        var_t0_dn12 = assign35310_e50189_d_n12;
        var_t0_dn17 = assign35310_e50189_d_n17;

        let (assign35320_e50195, assign35320_e50195_d_n0, assign35320_e50195_d_n2, assign35320_e50195_d_n6, assign35320_e50195_d_n7, assign35320_e50195_d_n10, assign35320_e50195_d_n11, assign35320_e50195_d_n12, assign35320_e50195_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35320_e50193: f64 = (var_mks_rdrvmax / var_t0);
        (assign35320_e50193, (-((var_mks_rdrvmax * var_t0_dn0) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn2) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn6) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn7) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn10) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn11) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn12) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn17) / (var_t0 * var_t0))),)
    } else {
        (var_vmaxe__blk1164, var_vmaxe__blk1164_dn0, var_vmaxe__blk1164_dn2, var_vmaxe__blk1164_dn6, var_vmaxe__blk1164_dn7, var_vmaxe__blk1164_dn10, var_vmaxe__blk1164_dn11, var_vmaxe__blk1164_dn12, var_vmaxe__blk1164_dn17,)
    }
};
        var_vmaxe__blk1164 = assign35320_e50195;
        var_vmaxe__blk1164_dn0 = assign35320_e50195_d_n0;
        var_vmaxe__blk1164_dn2 = assign35320_e50195_d_n2;
        var_vmaxe__blk1164_dn6 = assign35320_e50195_d_n6;
        var_vmaxe__blk1164_dn7 = assign35320_e50195_d_n7;
        var_vmaxe__blk1164_dn10 = assign35320_e50195_d_n10;
        var_vmaxe__blk1164_dn11 = assign35320_e50195_d_n11;
        var_vmaxe__blk1164_dn12 = assign35320_e50195_d_n12;
        var_vmaxe__blk1164_dn17 = assign35320_e50195_d_n17;

        let (assign35330_e50205, assign35330_e50205_d_n10,) = {
    if (var_guard1151 != 0.0) {
        let assign35330_e50201: f64 = (var_ttemp - var_uc_tnom);
        let assign35330_e50202: f64 = (p.p274 * assign35330_e50201);
        let assign35330_e50203: f64 = (var_rrdrbb + assign35330_e50202);
        (assign35330_e50203, (var_rrdrbb_dn10 + (p.p274 * var_ttemp_dn10)),)
    } else {
        (var_rrdrbb, var_rrdrbb_dn10,)
    }
};
        var_rrdrbb = assign35330_e50205;
        var_rrdrbb_dn10 = assign35330_e50205_d_n10;

        let (assign35340_e50215,) = {
    if (var_guard1151 != 0.0) {
        let assign35340_e50211: f64 = (var_lgle).powf(p.p280);
        let assign35340_e50212: f64 = (p.p279 / assign35340_e50211);
        let assign35340_e50213: f64 = (1.0 + assign35340_e50212);
        (assign35340_e50213,)
    } else {
        (var_rdrmuele,)
    }
};
        var_rdrmuele = assign35340_e50215;

        let (assign35350_e50225,) = {
    if (var_guard1151 != 0.0) {
        let assign35350_e50221: f64 = (var_lgle).powf(p.p278);
        let assign35350_e50222: f64 = (p.p277 / assign35350_e50221);
        let assign35350_e50223: f64 = (1.0 + assign35350_e50222);
        (assign35350_e50223,)
    } else {
        (var_rdrvmaxle,)
    }
};
        var_rdrvmaxle = assign35350_e50225;

        let (assign35360_e50235,) = {
    if (var_guard1151 != 0.0) {
        let assign35360_e50231: f64 = (var_wg).powf(p.p276);
        let assign35360_e50232: f64 = (p.p275 / assign35360_e50231);
        let assign35360_e50233: f64 = (1.0 + assign35360_e50232);
        (assign35360_e50233,)
    } else {
        (var_rdrvmaxwe,)
    }
};
        var_rdrvmaxwe = assign35360_e50235;

        let (assign35370_e50241, assign35370_e50241_d_n0, assign35370_e50241_d_n2, assign35370_e50241_d_n6, assign35370_e50241_d_n7, assign35370_e50241_d_n10, assign35370_e50241_d_n11, assign35370_e50241_d_n12, assign35370_e50241_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35370_e50239: f64 = (var_mu0 * var_rdrmuele);
        (assign35370_e50239, (var_mu0_dn0 * var_rdrmuele), (var_mu0_dn2 * var_rdrmuele), (var_mu0_dn6 * var_rdrmuele), (var_mu0_dn7 * var_rdrmuele), (var_mu0_dn10 * var_rdrmuele), (var_mu0_dn11 * var_rdrmuele), (var_mu0_dn12 * var_rdrmuele), (var_mu0_dn17 * var_rdrmuele),)
    } else {
        (var_mu0, var_mu0_dn0, var_mu0_dn2, var_mu0_dn6, var_mu0_dn7, var_mu0_dn10, var_mu0_dn11, var_mu0_dn12, var_mu0_dn17,)
    }
};
        var_mu0 = assign35370_e50241;
        var_mu0_dn0 = assign35370_e50241_d_n0;
        var_mu0_dn2 = assign35370_e50241_d_n2;
        var_mu0_dn6 = assign35370_e50241_d_n6;
        var_mu0_dn7 = assign35370_e50241_d_n7;
        var_mu0_dn10 = assign35370_e50241_d_n10;
        var_mu0_dn11 = assign35370_e50241_d_n11;
        var_mu0_dn12 = assign35370_e50241_d_n12;
        var_mu0_dn17 = assign35370_e50241_d_n17;

        let (assign35380_e50251, assign35380_e50251_d_n0, assign35380_e50251_d_n2, assign35380_e50251_d_n6, assign35380_e50251_d_n7, assign35380_e50251_d_n10, assign35380_e50251_d_n11, assign35380_e50251_d_n12, assign35380_e50251_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35380_e50245: f64 = (var_vmaxe__blk1164 * var_rdrvmaxwe);
        let assign35380_e50247: f64 = (assign35380_e50245 * var_rdrvmaxle);
        let assign35380_e50249: f64 = (assign35380_e50247 + 1e-50);
        (assign35380_e50249, ((var_vmaxe__blk1164_dn0 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1164_dn2 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1164_dn6 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1164_dn7 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1164_dn10 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1164_dn11 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1164_dn12 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1164_dn17 * var_rdrvmaxwe) * var_rdrvmaxle),)
    } else {
        (var_vmaxe__blk1164, var_vmaxe__blk1164_dn0, var_vmaxe__blk1164_dn2, var_vmaxe__blk1164_dn6, var_vmaxe__blk1164_dn7, var_vmaxe__blk1164_dn10, var_vmaxe__blk1164_dn11, var_vmaxe__blk1164_dn12, var_vmaxe__blk1164_dn17,)
    }
};
        var_vmaxe__blk1164 = assign35380_e50251;
        var_vmaxe__blk1164_dn0 = assign35380_e50251_d_n0;
        var_vmaxe__blk1164_dn2 = assign35380_e50251_d_n2;
        var_vmaxe__blk1164_dn6 = assign35380_e50251_d_n6;
        var_vmaxe__blk1164_dn7 = assign35380_e50251_d_n7;
        var_vmaxe__blk1164_dn10 = assign35380_e50251_d_n10;
        var_vmaxe__blk1164_dn11 = assign35380_e50251_d_n11;
        var_vmaxe__blk1164_dn12 = assign35380_e50251_d_n12;
        var_vmaxe__blk1164_dn17 = assign35380_e50251_d_n17;

        let (assign35390_e50257, assign35390_e50257_d_n0, assign35390_e50257_d_n2, assign35390_e50257_d_n6, assign35390_e50257_d_n7,) = {
    if (var_guard1151 != 0.0) {
        let assign35390_e50255: f64 = (var_vrdr / var_ldrifte);
        (assign35390_e50255, (var_vrdr_dn0 / var_ldrifte), (var_vrdr_dn2 / var_ldrifte), (var_vrdr_dn6 / var_ldrifte), (var_vrdr_dn7 / var_ldrifte),)
    } else {
        (var_edri, var_edri_dn0, var_edri_dn2, var_edri_dn6, var_edri_dn7,)
    }
};
        var_edri = assign35390_e50257;
        var_edri_dn0 = assign35390_e50257_d_n0;
        var_edri_dn2 = assign35390_e50257_d_n2;
        var_edri_dn6 = assign35390_e50257_d_n6;
        var_edri_dn7 = assign35390_e50257_d_n7;

        let (assign35400_e50263, assign35400_e50263_d_n0, assign35400_e50263_d_n2, assign35400_e50263_d_n6, assign35400_e50263_d_n7, assign35400_e50263_d_n10, assign35400_e50263_d_n11, assign35400_e50263_d_n12, assign35400_e50263_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35400_e50261: f64 = (var_mu0 * var_edri);
        (assign35400_e50261, ((var_mu0_dn0 * var_edri) + (var_mu0 * var_edri_dn0)), ((var_mu0_dn2 * var_edri) + (var_mu0 * var_edri_dn2)), ((var_mu0_dn6 * var_edri) + (var_mu0 * var_edri_dn6)), ((var_mu0_dn7 * var_edri) + (var_mu0 * var_edri_dn7)), (var_mu0_dn10 * var_edri), (var_mu0_dn11 * var_edri), (var_mu0_dn12 * var_edri), (var_mu0_dn17 * var_edri),)
    } else {
        (var_vdri, var_vdri_dn0, var_vdri_dn2, var_vdri_dn6, var_vdri_dn7, var_vdri_dn10, var_vdri_dn11, var_vdri_dn12, var_vdri_dn17,)
    }
};
        var_vdri = assign35400_e50263;
        var_vdri_dn0 = assign35400_e50263_d_n0;
        var_vdri_dn2 = assign35400_e50263_d_n2;
        var_vdri_dn6 = assign35400_e50263_d_n6;
        var_vdri_dn7 = assign35400_e50263_d_n7;
        var_vdri_dn10 = assign35400_e50263_d_n10;
        var_vdri_dn11 = assign35400_e50263_d_n11;
        var_vdri_dn12 = assign35400_e50263_d_n12;
        var_vdri_dn17 = assign35400_e50263_d_n17;

        let assign35410_e50266: f64 = if var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        var_guard1172 = assign35410_e50266;

        let (assign35420_e50274, assign35420_e50274_d_n0, assign35420_e50274_d_n2, assign35420_e50274_d_n6, assign35420_e50274_d_n7, assign35420_e50274_d_n10, assign35420_e50274_d_n11, assign35420_e50274_d_n12, assign35420_e50274_d_n17,) = {
    if ((var_guard1151 != 0.0) && (var_guard1172 != 0.0)) {
        let assign35420_e50272: f64 = (var_vdri / var_vmaxe__blk1164);
        (assign35420_e50272, (((var_vdri_dn0 * var_vmaxe__blk1164) - (var_vdri * var_vmaxe__blk1164_dn0)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), (((var_vdri_dn2 * var_vmaxe__blk1164) - (var_vdri * var_vmaxe__blk1164_dn2)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), (((var_vdri_dn6 * var_vmaxe__blk1164) - (var_vdri * var_vmaxe__blk1164_dn6)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), (((var_vdri_dn7 * var_vmaxe__blk1164) - (var_vdri * var_vmaxe__blk1164_dn7)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), (((var_vdri_dn10 * var_vmaxe__blk1164) - (var_vdri * var_vmaxe__blk1164_dn10)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), (((var_vdri_dn11 * var_vmaxe__blk1164) - (var_vdri * var_vmaxe__blk1164_dn11)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), (((var_vdri_dn12 * var_vmaxe__blk1164) - (var_vdri * var_vmaxe__blk1164_dn12)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), (((var_vdri_dn17 * var_vmaxe__blk1164) - (var_vdri * var_vmaxe__blk1164_dn17)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35420_e50274;
        var_t1_dn0 = assign35420_e50274_d_n0;
        var_t1_dn2 = assign35420_e50274_d_n2;
        var_t1_dn6 = assign35420_e50274_d_n6;
        var_t1_dn7 = assign35420_e50274_d_n7;
        var_t1_dn10 = assign35420_e50274_d_n10;
        var_t1_dn11 = assign35420_e50274_d_n11;
        var_t1_dn12 = assign35420_e50274_d_n12;
        var_t1_dn17 = assign35420_e50274_d_n17;

        let (assign35430_e50284, assign35430_e50284_d_n0, assign35430_e50284_d_n2, assign35430_e50284_d_n6, assign35430_e50284_d_n7, assign35430_e50284_d_n10, assign35430_e50284_d_n11, assign35430_e50284_d_n12, assign35430_e50284_d_n17,) = {
    if ((var_guard1151 != 0.0) && (var_guard1172 == 0.0)) {
        let assign35430_e50280: f64 = (-var_vdri);
        let assign35430_e50282: f64 = (assign35430_e50280 / var_vmaxe__blk1164);
        (assign35430_e50282, ((((-var_vdri_dn0) * var_vmaxe__blk1164) - (assign35430_e50280 * var_vmaxe__blk1164_dn0)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), ((((-var_vdri_dn2) * var_vmaxe__blk1164) - (assign35430_e50280 * var_vmaxe__blk1164_dn2)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), ((((-var_vdri_dn6) * var_vmaxe__blk1164) - (assign35430_e50280 * var_vmaxe__blk1164_dn6)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), ((((-var_vdri_dn7) * var_vmaxe__blk1164) - (assign35430_e50280 * var_vmaxe__blk1164_dn7)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), ((((-var_vdri_dn10) * var_vmaxe__blk1164) - (assign35430_e50280 * var_vmaxe__blk1164_dn10)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), ((((-var_vdri_dn11) * var_vmaxe__blk1164) - (assign35430_e50280 * var_vmaxe__blk1164_dn11)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), ((((-var_vdri_dn12) * var_vmaxe__blk1164) - (assign35430_e50280 * var_vmaxe__blk1164_dn12)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)), ((((-var_vdri_dn17) * var_vmaxe__blk1164) - (assign35430_e50280 * var_vmaxe__blk1164_dn17)) / (var_vmaxe__blk1164 * var_vmaxe__blk1164)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35430_e50284;
        var_t1_dn0 = assign35430_e50284_d_n0;
        var_t1_dn2 = assign35430_e50284_d_n2;
        var_t1_dn6 = assign35430_e50284_d_n6;
        var_t1_dn7 = assign35430_e50284_d_n7;
        var_t1_dn10 = assign35430_e50284_d_n10;
        var_t1_dn11 = assign35430_e50284_d_n11;
        var_t1_dn12 = assign35430_e50284_d_n12;
        var_t1_dn17 = assign35430_e50284_d_n17;

        let assign35440_e50288: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50289: f64 = (1.0 - assign35440_e50288);
        let assign35440_e50296: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50297: f64 = (1.0 + assign35440_e50296);
        let assign35440_e50299: f64 = if ((assign35440_e50289 <= var_rrdrbb) && (var_rrdrbb <= assign35440_e50297)) { 1.0 } else { 0.0 };
        var_guard1173 = assign35440_e50299;

        let (assign35450_e50305, assign35450_e50305_d_n0, assign35450_e50305_d_n2, assign35450_e50305_d_n6, assign35450_e50305_d_n7, assign35450_e50305_d_n10, assign35450_e50305_d_n11, assign35450_e50305_d_n12, assign35450_e50305_d_n17,) = {
    if ((var_guard1151 != 0.0) && (var_guard1173 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign35450_e50305;
        var_t3_dn0 = assign35450_e50305_d_n0;
        var_t3_dn2 = assign35450_e50305_d_n2;
        var_t3_dn6 = assign35450_e50305_d_n6;
        var_t3_dn7 = assign35450_e50305_d_n7;
        var_t3_dn10 = assign35450_e50305_d_n10;
        var_t3_dn11 = assign35450_e50305_d_n11;
        var_t3_dn12 = assign35450_e50305_d_n12;
        var_t3_dn17 = assign35450_e50305_d_n17;

        let assign35460_e50309: f64 = (10.0 * 2.220446049250313e-16);
        let assign35460_e50310: f64 = (2.0 - assign35460_e50309);
        let assign35460_e50317: f64 = (10.0 * 2.220446049250313e-16);
        let assign35460_e50318: f64 = (2.0 + assign35460_e50317);
        let assign35460_e50320: f64 = if ((assign35460_e50310 <= var_rrdrbb) && (var_rrdrbb <= assign35460_e50318)) { 1.0 } else { 0.0 };
        var_guard1174 = assign35460_e50320;

        let (assign35470_e50329, assign35470_e50329_d_n0, assign35470_e50329_d_n2, assign35470_e50329_d_n6, assign35470_e50329_d_n7, assign35470_e50329_d_n10, assign35470_e50329_d_n11, assign35470_e50329_d_n12, assign35470_e50329_d_n17,) = {
    if (((var_guard1151 != 0.0) && (var_guard1173 == 0.0)) && (var_guard1174 != 0.0)) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign35470_e50329;
        var_t3_dn0 = assign35470_e50329_d_n0;
        var_t3_dn2 = assign35470_e50329_d_n2;
        var_t3_dn6 = assign35470_e50329_d_n6;
        var_t3_dn7 = assign35470_e50329_d_n7;
        var_t3_dn10 = assign35470_e50329_d_n10;
        var_t3_dn11 = assign35470_e50329_d_n11;
        var_t3_dn12 = assign35470_e50329_d_n12;
        var_t3_dn17 = assign35470_e50329_d_n17;

        let (assign35480_e50343, assign35480_e50343_d_n0, assign35480_e50343_d_n2, assign35480_e50343_d_n6, assign35480_e50343_d_n7, assign35480_e50343_d_n10, assign35480_e50343_d_n11, assign35480_e50343_d_n12, assign35480_e50343_d_n17,) = {
    if (((var_guard1151 != 0.0) && (var_guard1173 == 0.0)) && (var_guard1174 == 0.0)) {
        let assign35480_e50340: f64 = (var_rrdrbb - 1.0);
        let assign35480_e50341: f64 = (var_t1).powf(assign35480_e50340);
        (assign35480_e50341, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((var_t1).powf(assign35480_e50340 - 1.0) * var_t1_dn0)) } } else { (assign35480_e50341 * (assign35480_e50340 * (var_t1_dn0 / var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((var_t1).powf(assign35480_e50340 - 1.0) * var_t1_dn2)) } } else { (assign35480_e50341 * (assign35480_e50340 * (var_t1_dn2 / var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((var_t1).powf(assign35480_e50340 - 1.0) * var_t1_dn6)) } } else { (assign35480_e50341 * (assign35480_e50340 * (var_t1_dn6 / var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((var_t1).powf(assign35480_e50340 - 1.0) * var_t1_dn7)) } } else { (assign35480_e50341 * (assign35480_e50340 * (var_t1_dn7 / var_t1))) }, if var_rrdrbb_dn10 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((var_t1).powf(assign35480_e50340 - 1.0) * var_t1_dn10)) } } else { (assign35480_e50341 * ((var_rrdrbb_dn10 * (var_t1).ln()) + (assign35480_e50340 * (var_t1_dn10 / var_t1)))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((var_t1).powf(assign35480_e50340 - 1.0) * var_t1_dn11)) } } else { (assign35480_e50341 * (assign35480_e50340 * (var_t1_dn11 / var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((var_t1).powf(assign35480_e50340 - 1.0) * var_t1_dn12)) } } else { (assign35480_e50341 * (assign35480_e50340 * (var_t1_dn12 / var_t1))) }, if 0.0 == 0.0 && ((assign35480_e50340) as f64).is_finite() && ((assign35480_e50340) as f64).fract() == 0.0 { if assign35480_e50340 == 0.0 { 0.0 } else { (assign35480_e50340 * ((var_t1).powf(assign35480_e50340 - 1.0) * var_t1_dn17)) } } else { (assign35480_e50341 * (assign35480_e50340 * (var_t1_dn17 / var_t1))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign35480_e50343;
        var_t3_dn0 = assign35480_e50343_d_n0;
        var_t3_dn2 = assign35480_e50343_d_n2;
        var_t3_dn6 = assign35480_e50343_d_n6;
        var_t3_dn7 = assign35480_e50343_d_n7;
        var_t3_dn10 = assign35480_e50343_d_n10;
        var_t3_dn11 = assign35480_e50343_d_n11;
        var_t3_dn12 = assign35480_e50343_d_n12;
        var_t3_dn17 = assign35480_e50343_d_n17;

        let (assign35490_e50349, assign35490_e50349_d_n0, assign35490_e50349_d_n2, assign35490_e50349_d_n6, assign35490_e50349_d_n7, assign35490_e50349_d_n10, assign35490_e50349_d_n11, assign35490_e50349_d_n12, assign35490_e50349_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35490_e50347: f64 = (var_t1 * var_t3);
        (assign35490_e50347, ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)), ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)), ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)), ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)), ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)), ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)), ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)), ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign35490_e50349;
        var_t2_dn0 = assign35490_e50349_d_n0;
        var_t2_dn2 = assign35490_e50349_d_n2;
        var_t2_dn6 = assign35490_e50349_d_n6;
        var_t2_dn7 = assign35490_e50349_d_n7;
        var_t2_dn10 = assign35490_e50349_d_n10;
        var_t2_dn11 = assign35490_e50349_d_n11;
        var_t2_dn12 = assign35490_e50349_d_n12;
        var_t2_dn17 = assign35490_e50349_d_n17;

        let (assign35500_e50355, assign35500_e50355_d_n0, assign35500_e50355_d_n2, assign35500_e50355_d_n6, assign35500_e50355_d_n7, assign35500_e50355_d_n10, assign35500_e50355_d_n11, assign35500_e50355_d_n12, assign35500_e50355_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35500_e50353: f64 = (1.0 + var_t2);
        (assign35500_e50353, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign35500_e50355;
        var_t4_dn0 = assign35500_e50355_d_n0;
        var_t4_dn2 = assign35500_e50355_d_n2;
        var_t4_dn6 = assign35500_e50355_d_n6;
        var_t4_dn7 = assign35500_e50355_d_n7;
        var_t4_dn10 = assign35500_e50355_d_n10;
        var_t4_dn11 = assign35500_e50355_d_n11;
        var_t4_dn12 = assign35500_e50355_d_n12;
        var_t4_dn17 = assign35500_e50355_d_n17;

        let assign35510_e50359: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50360: f64 = (1.0 - assign35510_e50359);
        let assign35510_e50367: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50368: f64 = (1.0 + assign35510_e50367);
        let assign35510_e50370: f64 = if ((assign35510_e50360 <= var_rrdrbb) && (var_rrdrbb <= assign35510_e50368)) { 1.0 } else { 0.0 };
        var_guard1175 = assign35510_e50370;

        let (assign35520_e50378, assign35520_e50378_d_n0, assign35520_e50378_d_n2, assign35520_e50378_d_n6, assign35520_e50378_d_n7, assign35520_e50378_d_n10, assign35520_e50378_d_n11, assign35520_e50378_d_n12, assign35520_e50378_d_n17,) = {
    if ((var_guard1151 != 0.0) && (var_guard1175 != 0.0)) {
        let assign35520_e50376: f64 = (1.0 / var_t4);
        (assign35520_e50376, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn11 / (var_t4 * var_t4))), (-(var_t4_dn12 / (var_t4 * var_t4))), (-(var_t4_dn17 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign35520_e50378;
        var_t5_dn0 = assign35520_e50378_d_n0;
        var_t5_dn2 = assign35520_e50378_d_n2;
        var_t5_dn6 = assign35520_e50378_d_n6;
        var_t5_dn7 = assign35520_e50378_d_n7;
        var_t5_dn10 = assign35520_e50378_d_n10;
        var_t5_dn11 = assign35520_e50378_d_n11;
        var_t5_dn12 = assign35520_e50378_d_n12;
        var_t5_dn17 = assign35520_e50378_d_n17;

        let assign35530_e50382: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50383: f64 = (2.0 - assign35530_e50382);
        let assign35530_e50390: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50391: f64 = (2.0 + assign35530_e50390);
        let assign35530_e50393: f64 = if ((assign35530_e50383 <= var_rrdrbb) && (var_rrdrbb <= assign35530_e50391)) { 1.0 } else { 0.0 };
        var_guard1176 = assign35530_e50393;

        let (assign35540_e50405, assign35540_e50405_d_n0, assign35540_e50405_d_n2, assign35540_e50405_d_n6, assign35540_e50405_d_n7, assign35540_e50405_d_n10, assign35540_e50405_d_n11, assign35540_e50405_d_n12, assign35540_e50405_d_n17,) = {
    if (((var_guard1151 != 0.0) && (var_guard1175 == 0.0)) && (var_guard1176 != 0.0)) {
        let assign35540_e50402: f64 = (var_t4).sqrt();
        let assign35540_e50403: f64 = (1.0 / assign35540_e50402);
        (assign35540_e50403, (-((var_t4_dn0 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn2 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn6 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn7 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn10 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn11 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn12 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn17 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign35540_e50405;
        var_t5_dn0 = assign35540_e50405_d_n0;
        var_t5_dn2 = assign35540_e50405_d_n2;
        var_t5_dn6 = assign35540_e50405_d_n6;
        var_t5_dn7 = assign35540_e50405_d_n7;
        var_t5_dn10 = assign35540_e50405_d_n10;
        var_t5_dn11 = assign35540_e50405_d_n11;
        var_t5_dn12 = assign35540_e50405_d_n12;
        var_t5_dn17 = assign35540_e50405_d_n17;

        let (assign35550_e50422, assign35550_e50422_d_n0, assign35550_e50422_d_n2, assign35550_e50422_d_n6, assign35550_e50422_d_n7, assign35550_e50422_d_n10, assign35550_e50422_d_n11, assign35550_e50422_d_n12, assign35550_e50422_d_n17,) = {
    if (((var_guard1151 != 0.0) && (var_guard1175 == 0.0)) && (var_guard1176 == 0.0)) {
        let assign35550_e50415: f64 = (-1.0);
        let assign35550_e50417: f64 = (assign35550_e50415 / var_rrdrbb);
        let assign35550_e50419: f64 = (assign35550_e50417 - 1.0);
        let assign35550_e50420: f64 = (var_t4).powf(assign35550_e50419);
        (assign35550_e50420, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn0)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn0 / var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn2)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn2 / var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn6)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn6 / var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn7)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn7 / var_t4))) }, if (-((assign35550_e50415 * var_rrdrbb_dn10) / (var_rrdrbb * var_rrdrbb))) == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn10)) } } else { (assign35550_e50420 * (((-((assign35550_e50415 * var_rrdrbb_dn10) / (var_rrdrbb * var_rrdrbb))) * (var_t4).ln()) + (assign35550_e50419 * (var_t4_dn10 / var_t4)))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn11)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn11 / var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn12)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn12 / var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn17)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn17 / var_t4))) },)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn6, var_t6_dn7, var_t6_dn10, var_t6_dn11, var_t6_dn12, var_t6_dn17,)
    }
};
        var_t6 = assign35550_e50422;
        var_t6_dn0 = assign35550_e50422_d_n0;
        var_t6_dn2 = assign35550_e50422_d_n2;
        var_t6_dn6 = assign35550_e50422_d_n6;
        var_t6_dn7 = assign35550_e50422_d_n7;
        var_t6_dn10 = assign35550_e50422_d_n10;
        var_t6_dn11 = assign35550_e50422_d_n11;
        var_t6_dn12 = assign35550_e50422_d_n12;
        var_t6_dn17 = assign35550_e50422_d_n17;

        let (assign35560_e50434, assign35560_e50434_d_n0, assign35560_e50434_d_n2, assign35560_e50434_d_n6, assign35560_e50434_d_n7, assign35560_e50434_d_n10, assign35560_e50434_d_n11, assign35560_e50434_d_n12, assign35560_e50434_d_n17,) = {
    if (((var_guard1151 != 0.0) && (var_guard1175 == 0.0)) && (var_guard1176 == 0.0)) {
        let assign35560_e50432: f64 = (var_t4 * var_t6);
        (assign35560_e50432, ((var_t4_dn0 * var_t6) + (var_t4 * var_t6_dn0)), ((var_t4_dn2 * var_t6) + (var_t4 * var_t6_dn2)), ((var_t4_dn6 * var_t6) + (var_t4 * var_t6_dn6)), ((var_t4_dn7 * var_t6) + (var_t4 * var_t6_dn7)), ((var_t4_dn10 * var_t6) + (var_t4 * var_t6_dn10)), ((var_t4_dn11 * var_t6) + (var_t4 * var_t6_dn11)), ((var_t4_dn12 * var_t6) + (var_t4 * var_t6_dn12)), ((var_t4_dn17 * var_t6) + (var_t4 * var_t6_dn17)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign35560_e50434;
        var_t5_dn0 = assign35560_e50434_d_n0;
        var_t5_dn2 = assign35560_e50434_d_n2;
        var_t5_dn6 = assign35560_e50434_d_n6;
        var_t5_dn7 = assign35560_e50434_d_n7;
        var_t5_dn10 = assign35560_e50434_d_n10;
        var_t5_dn11 = assign35560_e50434_d_n11;
        var_t5_dn12 = assign35560_e50434_d_n12;
        var_t5_dn17 = assign35560_e50434_d_n17;

        let (assign35570_e50440, assign35570_e50440_d_n0, assign35570_e50440_d_n2, assign35570_e50440_d_n6, assign35570_e50440_d_n7, assign35570_e50440_d_n10, assign35570_e50440_d_n11, assign35570_e50440_d_n12, assign35570_e50440_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35570_e50438: f64 = (var_mu0 * var_t5);
        (assign35570_e50438, ((var_mu0_dn0 * var_t5) + (var_mu0 * var_t5_dn0)), ((var_mu0_dn2 * var_t5) + (var_mu0 * var_t5_dn2)), ((var_mu0_dn6 * var_t5) + (var_mu0 * var_t5_dn6)), ((var_mu0_dn7 * var_t5) + (var_mu0 * var_t5_dn7)), ((var_mu0_dn10 * var_t5) + (var_mu0 * var_t5_dn10)), ((var_mu0_dn11 * var_t5) + (var_mu0 * var_t5_dn11)), ((var_mu0_dn12 * var_t5) + (var_mu0 * var_t5_dn12)), ((var_mu0_dn17 * var_t5) + (var_mu0 * var_t5_dn17)),)
    } else {
        (var_mu__blk1167, var_mu__blk1167_dn0, var_mu__blk1167_dn2, var_mu__blk1167_dn6, var_mu__blk1167_dn7, var_mu__blk1167_dn10, var_mu__blk1167_dn11, var_mu__blk1167_dn12, var_mu__blk1167_dn17,)
    }
};
        var_mu__blk1167 = assign35570_e50440;
        var_mu__blk1167_dn0 = assign35570_e50440_d_n0;
        var_mu__blk1167_dn2 = assign35570_e50440_d_n2;
        var_mu__blk1167_dn6 = assign35570_e50440_d_n6;
        var_mu__blk1167_dn7 = assign35570_e50440_d_n7;
        var_mu__blk1167_dn10 = assign35570_e50440_d_n10;
        var_mu__blk1167_dn11 = assign35570_e50440_d_n11;
        var_mu__blk1167_dn12 = assign35570_e50440_d_n12;
        var_mu__blk1167_dn17 = assign35570_e50440_d_n17;

        let (assign35580_e50446, assign35580_e50446_d_n0, assign35580_e50446_d_n2, assign35580_e50446_d_n6, assign35580_e50446_d_n7, assign35580_e50446_d_n10, assign35580_e50446_d_n11, assign35580_e50446_d_n12, assign35580_e50446_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35580_e50444: f64 = (1.6021918e-19 / var_ldrifte);
        (assign35580_e50444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35580_e50446;
        var_t1_dn0 = assign35580_e50446_d_n0;
        var_t1_dn2 = assign35580_e50446_d_n2;
        var_t1_dn6 = assign35580_e50446_d_n6;
        var_t1_dn7 = assign35580_e50446_d_n7;
        var_t1_dn10 = assign35580_e50446_d_n10;
        var_t1_dn11 = assign35580_e50446_d_n11;
        var_t1_dn12 = assign35580_e50446_d_n12;
        var_t1_dn17 = assign35580_e50446_d_n17;

        let (assign35590_e50456, assign35590_e50456_d_n0, assign35590_e50456_d_n2, assign35590_e50456_d_n6, assign35590_e50456_d_n7, assign35590_e50456_d_n10, assign35590_e50456_d_n11, assign35590_e50456_d_n12, assign35590_e50456_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35590_e50450: f64 = (var_t1 * var_xov);
        let assign35590_e50452: f64 = (assign35590_e50450 * var_mu__blk1167);
        let assign35590_e50454: f64 = (assign35590_e50452 * var_nover);
        (assign35590_e50454, ((((var_t1_dn0 * var_xov) * var_mu__blk1167) + (assign35590_e50450 * var_mu__blk1167_dn0)) * var_nover), ((((var_t1_dn2 * var_xov) * var_mu__blk1167) + (assign35590_e50450 * var_mu__blk1167_dn2)) * var_nover), ((((var_t1_dn6 * var_xov) * var_mu__blk1167) + (assign35590_e50450 * var_mu__blk1167_dn6)) * var_nover), ((((var_t1_dn7 * var_xov) * var_mu__blk1167) + (assign35590_e50450 * var_mu__blk1167_dn7)) * var_nover), ((((var_t1_dn10 * var_xov) * var_mu__blk1167) + (assign35590_e50450 * var_mu__blk1167_dn10)) * var_nover), ((((var_t1_dn11 * var_xov) * var_mu__blk1167) + (assign35590_e50450 * var_mu__blk1167_dn11)) * var_nover), ((((var_t1_dn12 * var_xov) * var_mu__blk1167) + (assign35590_e50450 * var_mu__blk1167_dn12)) * var_nover), ((((var_t1_dn17 * var_xov) * var_mu__blk1167) + (assign35590_e50450 * var_mu__blk1167_dn17)) * var_nover),)
    } else {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn6, var_gd_dn7, var_gd_dn10, var_gd_dn11, var_gd_dn12, var_gd_dn17,)
    }
};
        var_gd = assign35590_e50456;
        var_gd_dn0 = assign35590_e50456_d_n0;
        var_gd_dn2 = assign35590_e50456_d_n2;
        var_gd_dn6 = assign35590_e50456_d_n6;
        var_gd_dn7 = assign35590_e50456_d_n7;
        var_gd_dn10 = assign35590_e50456_d_n10;
        var_gd_dn11 = assign35590_e50456_d_n11;
        var_gd_dn12 = assign35590_e50456_d_n12;
        var_gd_dn17 = assign35590_e50456_d_n17;

        let assign35600_e50459: f64 = if var_gd <= 0.0 { 1.0 } else { 0.0 };
        var_guard1177 = assign35600_e50459;

        let (assign35610_e50465, assign35610_e50465_d_n0, assign35610_e50465_d_n2, assign35610_e50465_d_n6, assign35610_e50465_d_n7, assign35610_e50465_d_n10, assign35610_e50465_d_n11, assign35610_e50465_d_n12, assign35610_e50465_d_n17,) = {
    if ((var_guard1151 != 0.0) && (var_guard1177 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn6, var_gd_dn7, var_gd_dn10, var_gd_dn11, var_gd_dn12, var_gd_dn17,)
    }
};
        var_gd = assign35610_e50465;
        var_gd_dn0 = assign35610_e50465_d_n0;
        var_gd_dn2 = assign35610_e50465_d_n2;
        var_gd_dn6 = assign35610_e50465_d_n6;
        var_gd_dn7 = assign35610_e50465_d_n7;
        var_gd_dn10 = assign35610_e50465_d_n10;
        var_gd_dn11 = assign35610_e50465_d_n11;
        var_gd_dn12 = assign35610_e50465_d_n12;
        var_gd_dn17 = assign35610_e50465_d_n17;

        *var_edri_slot = var_edri;
        *var_edri_dn0_slot = var_edri_dn0;
        *var_edri_dn2_slot = var_edri_dn2;
        *var_edri_dn6_slot = var_edri_dn6;
        *var_edri_dn7_slot = var_edri_dn7;
        *var_gd_slot = var_gd;
        *var_gd_dn0_slot = var_gd_dn0;
        *var_gd_dn10_slot = var_gd_dn10;
        *var_gd_dn11_slot = var_gd_dn11;
        *var_gd_dn12_slot = var_gd_dn12;
        *var_gd_dn17_slot = var_gd_dn17;
        *var_gd_dn2_slot = var_gd_dn2;
        *var_gd_dn6_slot = var_gd_dn6;
        *var_gd_dn7_slot = var_gd_dn7;
        *var_guard1172_slot = var_guard1172;
        *var_guard1173_slot = var_guard1173;
        *var_guard1174_slot = var_guard1174;
        *var_guard1175_slot = var_guard1175;
        *var_guard1176_slot = var_guard1176;
        *var_guard1177_slot = var_guard1177;
        *var_mu0_slot = var_mu0;
        *var_mu0_dn0_slot = var_mu0_dn0;
        *var_mu0_dn10_slot = var_mu0_dn10;
        *var_mu0_dn11_slot = var_mu0_dn11;
        *var_mu0_dn12_slot = var_mu0_dn12;
        *var_mu0_dn17_slot = var_mu0_dn17;
        *var_mu0_dn2_slot = var_mu0_dn2;
        *var_mu0_dn6_slot = var_mu0_dn6;
        *var_mu0_dn7_slot = var_mu0_dn7;
        *var_mu__blk1167_slot = var_mu__blk1167;
        *var_mu__blk1167_dn0_slot = var_mu__blk1167_dn0;
        *var_mu__blk1167_dn10_slot = var_mu__blk1167_dn10;
        *var_mu__blk1167_dn11_slot = var_mu__blk1167_dn11;
        *var_mu__blk1167_dn12_slot = var_mu__blk1167_dn12;
        *var_mu__blk1167_dn17_slot = var_mu__blk1167_dn17;
        *var_mu__blk1167_dn2_slot = var_mu__blk1167_dn2;
        *var_mu__blk1167_dn6_slot = var_mu__blk1167_dn6;
        *var_mu__blk1167_dn7_slot = var_mu__blk1167_dn7;
        *var_rdrmuele_slot = var_rdrmuele;
        *var_rdrvmaxle_slot = var_rdrvmaxle;
        *var_rdrvmaxwe_slot = var_rdrvmaxwe;
        *var_rrdrbb_slot = var_rrdrbb;
        *var_rrdrbb_dn10_slot = var_rrdrbb_dn10;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn17_slot = var_t4_dn17;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn17_slot = var_t5_dn17;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn17_slot = var_t6_dn17;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_vdri_slot = var_vdri;
        *var_vdri_dn0_slot = var_vdri_dn0;
        *var_vdri_dn10_slot = var_vdri_dn10;
        *var_vdri_dn11_slot = var_vdri_dn11;
        *var_vdri_dn12_slot = var_vdri_dn12;
        *var_vdri_dn17_slot = var_vdri_dn17;
        *var_vdri_dn2_slot = var_vdri_dn2;
        *var_vdri_dn6_slot = var_vdri_dn6;
        *var_vdri_dn7_slot = var_vdri_dn7;
        *var_vmaxe__blk1164_slot = var_vmaxe__blk1164;
        *var_vmaxe__blk1164_dn0_slot = var_vmaxe__blk1164_dn0;
        *var_vmaxe__blk1164_dn10_slot = var_vmaxe__blk1164_dn10;
        *var_vmaxe__blk1164_dn11_slot = var_vmaxe__blk1164_dn11;
        *var_vmaxe__blk1164_dn12_slot = var_vmaxe__blk1164_dn12;
        *var_vmaxe__blk1164_dn17_slot = var_vmaxe__blk1164_dn17;
        *var_vmaxe__blk1164_dn2_slot = var_vmaxe__blk1164_dn2;
        *var_vmaxe__blk1164_dn6_slot = var_vmaxe__blk1164_dn6;
        *var_vmaxe__blk1164_dn7_slot = var_vmaxe__blk1164_dn7;
    }

    pub(super) fn stamp_transient_block_124(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_gd: f64,
        var_gd_dn0: f64,
        var_gd_dn10: f64,
        var_gd_dn11: f64,
        var_gd_dn12: f64,
        var_gd_dn17: f64,
        var_gd_dn2: f64,
        var_gd_dn6: f64,
        var_gd_dn7: f64,
        var_guard1151: f64,
        var_lgle: f64,
        var_mfactor: f64,
        var_rsd0: f64,
        var_ttemp: f64,
        var_ttemp_dn10: f64,
        var_uc_tnom: f64,
        var_weff: f64,
        var_weff_nf_1: f64,
        var_wg: f64,
        var_edri__blk1193_slot: &mut f64,
        var_edri__blk1193_dn0_slot: &mut f64,
        var_edri__blk1193_dn2_slot: &mut f64,
        var_edri__blk1193_dn6_slot: &mut f64,
        var_edri__blk1193_dn7_slot: &mut f64,
        var_guard1178_slot: &mut f64,
        var_guard1179_slot: &mut f64,
        var_guard1199_slot: &mut f64,
        var_ldrifte__blk1189_slot: &mut f64,
        var_mks_rdrmue__blk1183_slot: &mut f64,
        var_mks_rdrvmax__blk1184_slot: &mut f64,
        var_mu0__blk1191_slot: &mut f64,
        var_mu0__blk1191_dn0_slot: &mut f64,
        var_mu0__blk1191_dn10_slot: &mut f64,
        var_mu0__blk1191_dn11_slot: &mut f64,
        var_mu0__blk1191_dn12_slot: &mut f64,
        var_mu0__blk1191_dn17_slot: &mut f64,
        var_mu0__blk1191_dn2_slot: &mut f64,
        var_mu0__blk1191_dn6_slot: &mut f64,
        var_mu0__blk1191_dn7_slot: &mut f64,
        var_nover__blk1190_slot: &mut f64,
        var_rdmod_slot: &mut f64,
        var_rdrmuele__blk1180_slot: &mut f64,
        var_rdrvmaxle__blk1182_slot: &mut f64,
        var_rdrvmaxwe__blk1181_slot: &mut f64,
        var_rrdrbb__blk1185_slot: &mut f64,
        var_rrdrbb__blk1185_dn10_slot: &mut f64,
        var_rsd_slot: &mut f64,
        var_rsd0__blk1186_slot: &mut f64,
        var_rsd_dn0_slot: &mut f64,
        var_rsd_dn10_slot: &mut f64,
        var_rsd_dn11_slot: &mut f64,
        var_rsd_dn12_slot: &mut f64,
        var_rsd_dn17_slot: &mut f64,
        var_rsd_dn2_slot: &mut f64,
        var_rsd_dn6_slot: &mut f64,
        var_rsd_dn7_slot: &mut f64,
        var_rsde_slot: &mut f64,
        var_rsde_dn0_slot: &mut f64,
        var_rsde_dn10_slot: &mut f64,
        var_rsde_dn11_slot: &mut f64,
        var_rsde_dn12_slot: &mut f64,
        var_rsde_dn17_slot: &mut f64,
        var_rsde_dn2_slot: &mut f64,
        var_rsde_dn6_slot: &mut f64,
        var_rsde_dn7_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_tratio__blk1188_slot: &mut f64,
        var_tratio__blk1188_dn10_slot: &mut f64,
        var_vmaxe__blk1192_slot: &mut f64,
        var_vmaxe__blk1192_dn0_slot: &mut f64,
        var_vmaxe__blk1192_dn10_slot: &mut f64,
        var_vmaxe__blk1192_dn11_slot: &mut f64,
        var_vmaxe__blk1192_dn12_slot: &mut f64,
        var_vmaxe__blk1192_dn17_slot: &mut f64,
        var_vmaxe__blk1192_dn2_slot: &mut f64,
        var_vmaxe__blk1192_dn6_slot: &mut f64,
        var_vmaxe__blk1192_dn7_slot: &mut f64,
        var_vrdr__blk1187_slot: &mut f64,
        var_vrdr__blk1187_dn0_slot: &mut f64,
        var_vrdr__blk1187_dn2_slot: &mut f64,
        var_vrdr__blk1187_dn6_slot: &mut f64,
        var_vrdr__blk1187_dn7_slot: &mut f64,
        var_weff_nf__blk1198_slot: &mut f64,
        var_xov__blk1196_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let mut var_edri__blk1193: f64 = *var_edri__blk1193_slot;
        let mut var_edri__blk1193_dn0: f64 = *var_edri__blk1193_dn0_slot;
        let mut var_edri__blk1193_dn2: f64 = *var_edri__blk1193_dn2_slot;
        let mut var_edri__blk1193_dn6: f64 = *var_edri__blk1193_dn6_slot;
        let mut var_edri__blk1193_dn7: f64 = *var_edri__blk1193_dn7_slot;
        let mut var_guard1178: f64 = *var_guard1178_slot;
        let mut var_guard1179: f64 = *var_guard1179_slot;
        let mut var_guard1199: f64 = *var_guard1199_slot;
        let mut var_ldrifte__blk1189: f64 = *var_ldrifte__blk1189_slot;
        let mut var_mks_rdrmue__blk1183: f64 = *var_mks_rdrmue__blk1183_slot;
        let mut var_mks_rdrvmax__blk1184: f64 = *var_mks_rdrvmax__blk1184_slot;
        let mut var_mu0__blk1191: f64 = *var_mu0__blk1191_slot;
        let mut var_mu0__blk1191_dn0: f64 = *var_mu0__blk1191_dn0_slot;
        let mut var_mu0__blk1191_dn10: f64 = *var_mu0__blk1191_dn10_slot;
        let mut var_mu0__blk1191_dn11: f64 = *var_mu0__blk1191_dn11_slot;
        let mut var_mu0__blk1191_dn12: f64 = *var_mu0__blk1191_dn12_slot;
        let mut var_mu0__blk1191_dn17: f64 = *var_mu0__blk1191_dn17_slot;
        let mut var_mu0__blk1191_dn2: f64 = *var_mu0__blk1191_dn2_slot;
        let mut var_mu0__blk1191_dn6: f64 = *var_mu0__blk1191_dn6_slot;
        let mut var_mu0__blk1191_dn7: f64 = *var_mu0__blk1191_dn7_slot;
        let mut var_nover__blk1190: f64 = *var_nover__blk1190_slot;
        let mut var_rdmod: f64 = *var_rdmod_slot;
        let mut var_rdrmuele__blk1180: f64 = *var_rdrmuele__blk1180_slot;
        let mut var_rdrvmaxle__blk1182: f64 = *var_rdrvmaxle__blk1182_slot;
        let mut var_rdrvmaxwe__blk1181: f64 = *var_rdrvmaxwe__blk1181_slot;
        let mut var_rrdrbb__blk1185: f64 = *var_rrdrbb__blk1185_slot;
        let mut var_rrdrbb__blk1185_dn10: f64 = *var_rrdrbb__blk1185_dn10_slot;
        let mut var_rsd: f64 = *var_rsd_slot;
        let mut var_rsd0__blk1186: f64 = *var_rsd0__blk1186_slot;
        let mut var_rsd_dn0: f64 = *var_rsd_dn0_slot;
        let mut var_rsd_dn10: f64 = *var_rsd_dn10_slot;
        let mut var_rsd_dn11: f64 = *var_rsd_dn11_slot;
        let mut var_rsd_dn12: f64 = *var_rsd_dn12_slot;
        let mut var_rsd_dn17: f64 = *var_rsd_dn17_slot;
        let mut var_rsd_dn2: f64 = *var_rsd_dn2_slot;
        let mut var_rsd_dn6: f64 = *var_rsd_dn6_slot;
        let mut var_rsd_dn7: f64 = *var_rsd_dn7_slot;
        let mut var_rsde: f64 = *var_rsde_slot;
        let mut var_rsde_dn0: f64 = *var_rsde_dn0_slot;
        let mut var_rsde_dn10: f64 = *var_rsde_dn10_slot;
        let mut var_rsde_dn11: f64 = *var_rsde_dn11_slot;
        let mut var_rsde_dn12: f64 = *var_rsde_dn12_slot;
        let mut var_rsde_dn17: f64 = *var_rsde_dn17_slot;
        let mut var_rsde_dn2: f64 = *var_rsde_dn2_slot;
        let mut var_rsde_dn6: f64 = *var_rsde_dn6_slot;
        let mut var_rsde_dn7: f64 = *var_rsde_dn7_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_tratio__blk1188: f64 = *var_tratio__blk1188_slot;
        let mut var_tratio__blk1188_dn10: f64 = *var_tratio__blk1188_dn10_slot;
        let mut var_vmaxe__blk1192: f64 = *var_vmaxe__blk1192_slot;
        let mut var_vmaxe__blk1192_dn0: f64 = *var_vmaxe__blk1192_dn0_slot;
        let mut var_vmaxe__blk1192_dn10: f64 = *var_vmaxe__blk1192_dn10_slot;
        let mut var_vmaxe__blk1192_dn11: f64 = *var_vmaxe__blk1192_dn11_slot;
        let mut var_vmaxe__blk1192_dn12: f64 = *var_vmaxe__blk1192_dn12_slot;
        let mut var_vmaxe__blk1192_dn17: f64 = *var_vmaxe__blk1192_dn17_slot;
        let mut var_vmaxe__blk1192_dn2: f64 = *var_vmaxe__blk1192_dn2_slot;
        let mut var_vmaxe__blk1192_dn6: f64 = *var_vmaxe__blk1192_dn6_slot;
        let mut var_vmaxe__blk1192_dn7: f64 = *var_vmaxe__blk1192_dn7_slot;
        let mut var_vrdr__blk1187: f64 = *var_vrdr__blk1187_slot;
        let mut var_vrdr__blk1187_dn0: f64 = *var_vrdr__blk1187_dn0_slot;
        let mut var_vrdr__blk1187_dn2: f64 = *var_vrdr__blk1187_dn2_slot;
        let mut var_vrdr__blk1187_dn6: f64 = *var_vrdr__blk1187_dn6_slot;
        let mut var_vrdr__blk1187_dn7: f64 = *var_vrdr__blk1187_dn7_slot;
        let mut var_weff_nf__blk1198: f64 = *var_weff_nf__blk1198_slot;
        let mut var_xov__blk1196: f64 = *var_xov__blk1196_slot;

        let (assign35620_e50471, assign35620_e50471_d_n0, assign35620_e50471_d_n2, assign35620_e50471_d_n6, assign35620_e50471_d_n7, assign35620_e50471_d_n10, assign35620_e50471_d_n11, assign35620_e50471_d_n12, assign35620_e50471_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35620_e50469: f64 = (1.0 / var_gd);
        (assign35620_e50469, (-(var_gd_dn0 / (var_gd * var_gd))), (-(var_gd_dn2 / (var_gd * var_gd))), (-(var_gd_dn6 / (var_gd * var_gd))), (-(var_gd_dn7 / (var_gd * var_gd))), (-(var_gd_dn10 / (var_gd * var_gd))), (-(var_gd_dn11 / (var_gd * var_gd))), (-(var_gd_dn12 / (var_gd * var_gd))), (-(var_gd_dn17 / (var_gd * var_gd))),)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign35620_e50471;
        var_rsd_dn0 = assign35620_e50471_d_n0;
        var_rsd_dn2 = assign35620_e50471_d_n2;
        var_rsd_dn6 = assign35620_e50471_d_n6;
        var_rsd_dn7 = assign35620_e50471_d_n7;
        var_rsd_dn10 = assign35620_e50471_d_n10;
        var_rsd_dn11 = assign35620_e50471_d_n11;
        var_rsd_dn12 = assign35620_e50471_d_n12;
        var_rsd_dn17 = assign35620_e50471_d_n17;

        let (assign35630_e50477, assign35630_e50477_d_n0, assign35630_e50477_d_n2, assign35630_e50477_d_n6, assign35630_e50477_d_n7, assign35630_e50477_d_n10, assign35630_e50477_d_n11, assign35630_e50477_d_n12, assign35630_e50477_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35630_e50475: f64 = (var_rsd / var_weff_nf_1);
        (assign35630_e50475, (var_rsd_dn0 / var_weff_nf_1), (var_rsd_dn2 / var_weff_nf_1), (var_rsd_dn6 / var_weff_nf_1), (var_rsd_dn7 / var_weff_nf_1), (var_rsd_dn10 / var_weff_nf_1), (var_rsd_dn11 / var_weff_nf_1), (var_rsd_dn12 / var_weff_nf_1), (var_rsd_dn17 / var_weff_nf_1),)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign35630_e50477;
        var_rsd_dn0 = assign35630_e50477_d_n0;
        var_rsd_dn2 = assign35630_e50477_d_n2;
        var_rsd_dn6 = assign35630_e50477_d_n6;
        var_rsd_dn7 = assign35630_e50477_d_n7;
        var_rsd_dn10 = assign35630_e50477_d_n10;
        var_rsd_dn11 = assign35630_e50477_d_n11;
        var_rsd_dn12 = assign35630_e50477_d_n12;
        var_rsd_dn17 = assign35630_e50477_d_n17;

        let (assign35640_e50483, assign35640_e50483_d_n0, assign35640_e50483_d_n2, assign35640_e50483_d_n6, assign35640_e50483_d_n7, assign35640_e50483_d_n10, assign35640_e50483_d_n11, assign35640_e50483_d_n12, assign35640_e50483_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35640_e50481: f64 = (var_rsd + var_rsd0);
        (assign35640_e50481, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign35640_e50483;
        var_rsd_dn0 = assign35640_e50483_d_n0;
        var_rsd_dn2 = assign35640_e50483_d_n2;
        var_rsd_dn6 = assign35640_e50483_d_n6;
        var_rsd_dn7 = assign35640_e50483_d_n7;
        var_rsd_dn10 = assign35640_e50483_d_n10;
        var_rsd_dn11 = assign35640_e50483_d_n11;
        var_rsd_dn12 = assign35640_e50483_d_n12;
        var_rsd_dn17 = assign35640_e50483_d_n17;

        let assign35660_e50501: f64 = if var_rsd < 0.0001 { 1.0 } else { 0.0 };
        var_guard1178 = assign35660_e50501;

        let (assign35670_e50507, assign35670_e50507_d_n0, assign35670_e50507_d_n2, assign35670_e50507_d_n6, assign35670_e50507_d_n7, assign35670_e50507_d_n10, assign35670_e50507_d_n11, assign35670_e50507_d_n12, assign35670_e50507_d_n17,) = {
    if ((var_guard1151 != 0.0) && (var_guard1178 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign35670_e50507;
        var_rsd_dn0 = assign35670_e50507_d_n0;
        var_rsd_dn2 = assign35670_e50507_d_n2;
        var_rsd_dn6 = assign35670_e50507_d_n6;
        var_rsd_dn7 = assign35670_e50507_d_n7;
        var_rsd_dn10 = assign35670_e50507_d_n10;
        var_rsd_dn11 = assign35670_e50507_d_n11;
        var_rsd_dn12 = assign35670_e50507_d_n12;
        var_rsd_dn17 = assign35670_e50507_d_n17;

        let (assign35680_e50513, assign35680_e50513_d_n0, assign35680_e50513_d_n2, assign35680_e50513_d_n6, assign35680_e50513_d_n7, assign35680_e50513_d_n10, assign35680_e50513_d_n11, assign35680_e50513_d_n12, assign35680_e50513_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35680_e50511: f64 = (var_rsd / var_mfactor);
        (assign35680_e50511, (var_rsd_dn0 / var_mfactor), (var_rsd_dn2 / var_mfactor), (var_rsd_dn6 / var_mfactor), (var_rsd_dn7 / var_mfactor), (var_rsd_dn10 / var_mfactor), (var_rsd_dn11 / var_mfactor), (var_rsd_dn12 / var_mfactor), (var_rsd_dn17 / var_mfactor),)
    } else {
        (var_rsde, var_rsde_dn0, var_rsde_dn2, var_rsde_dn6, var_rsde_dn7, var_rsde_dn10, var_rsde_dn11, var_rsde_dn12, var_rsde_dn17,)
    }
};
        var_rsde = assign35680_e50513;
        var_rsde_dn0 = assign35680_e50513_d_n0;
        var_rsde_dn2 = assign35680_e50513_d_n2;
        var_rsde_dn6 = assign35680_e50513_d_n6;
        var_rsde_dn7 = assign35680_e50513_d_n7;
        var_rsde_dn10 = assign35680_e50513_d_n10;
        var_rsde_dn11 = assign35680_e50513_d_n11;
        var_rsde_dn12 = assign35680_e50513_d_n12;
        var_rsde_dn17 = assign35680_e50513_d_n17;

        let assign35700_e50520: f64 = if p.p260 == 1.0 { 1.0 } else { 0.0 };
        var_guard1179 = assign35700_e50520;

        let (assign35710_e50524,) = {
    if (var_guard1179 != 0.0) {
        (2.0,)
    } else {
        (var_rdmod,)
    }
};
        var_rdmod = assign35710_e50524;

        let assign35720_e50527: f64 = if var_rdmod == 1.0 { 1.0 } else { 0.0 };
        var_guard1199 = assign35720_e50527;

        let (assign35730_e50535,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 != 0.0)) {
        let assign35730_e50533: f64 = (p.p264 / 1e-6);
        (assign35730_e50533,)
    } else {
        (var_nover__blk1190,)
    }
};
        var_nover__blk1190 = assign35730_e50535;

        let (assign35740_e50541,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 != 0.0)) {
        (p.p266,)
    } else {
        (var_mks_rdrmue__blk1183,)
    }
};
        var_mks_rdrmue__blk1183 = assign35740_e50541;

        let (assign35750_e50547,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 != 0.0)) {
        (p.p268,)
    } else {
        (var_mks_rdrvmax__blk1184,)
    }
};
        var_mks_rdrvmax__blk1184 = assign35750_e50547;

        let (assign35760_e50553, assign35760_e50553_d_n10,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (var_rrdrbb__blk1185, var_rrdrbb__blk1185_dn10,)
    }
};
        var_rrdrbb__blk1185 = assign35760_e50553;
        var_rrdrbb__blk1185_dn10 = assign35760_e50553_d_n10;

        let (assign35770_e50566,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 != 0.0)) {
        let (assign35770_e50564,) = {
            if (p.p263 > 0.0) {
                let assign35770_e50562: f64 = (p.p263 * p.p255);
                (assign35770_e50562,)
            } else {
                (0.0,)
            }
        };
        (assign35770_e50564,)
    } else {
        (var_rsd0__blk1186,)
    }
};
        var_rsd0__blk1186 = assign35770_e50566;

        let (assign35780_e50572,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 != 0.0)) {
        (p.p258,)
    } else {
        (var_ldrifte__blk1189,)
    }
};
        var_ldrifte__blk1189 = assign35780_e50572;

        let (assign35790_e50580, assign35790_e50580_d_n0, assign35790_e50580_d_n2, assign35790_e50580_d_n6, assign35790_e50580_d_n7,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 != 0.0)) {
        let assign35790_e50578: f64 = (p.p50 * (nv7 - nv2));
        (assign35790_e50578, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (var_vrdr__blk1187, var_vrdr__blk1187_dn0, var_vrdr__blk1187_dn2, var_vrdr__blk1187_dn6, var_vrdr__blk1187_dn7,)
    }
};
        var_vrdr__blk1187 = assign35790_e50580;
        var_vrdr__blk1187_dn0 = assign35790_e50580_d_n0;
        var_vrdr__blk1187_dn2 = assign35790_e50580_d_n2;
        var_vrdr__blk1187_dn6 = assign35790_e50580_d_n6;
        var_vrdr__blk1187_dn7 = assign35790_e50580_d_n7;

        let (assign35800_e50589,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 == 0.0)) {
        let assign35800_e50587: f64 = (p.p59 / 1e-6);
        (assign35800_e50587,)
    } else {
        (var_nover__blk1190,)
    }
};
        var_nover__blk1190 = assign35800_e50589;

        let (assign35810_e50596,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 == 0.0)) {
        (p.p265,)
    } else {
        (var_mks_rdrmue__blk1183,)
    }
};
        var_mks_rdrmue__blk1183 = assign35810_e50596;

        let (assign35820_e50603,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 == 0.0)) {
        (p.p267,)
    } else {
        (var_mks_rdrvmax__blk1184,)
    }
};
        var_mks_rdrvmax__blk1184 = assign35820_e50603;

        let (assign35830_e50610, assign35830_e50610_d_n10,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (var_rrdrbb__blk1185, var_rrdrbb__blk1185_dn10,)
    }
};
        var_rrdrbb__blk1185 = assign35830_e50610;
        var_rrdrbb__blk1185_dn10 = assign35830_e50610_d_n10;

        let (assign35840_e50624,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 == 0.0)) {
        let (assign35840_e50622,) = {
            if (p.p263 > 0.0) {
                let assign35840_e50620: f64 = (p.p263 * p.p256);
                (assign35840_e50620,)
            } else {
                (0.0,)
            }
        };
        (assign35840_e50622,)
    } else {
        (var_rsd0__blk1186,)
    }
};
        var_rsd0__blk1186 = assign35840_e50624;

        let (assign35850_e50631,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 == 0.0)) {
        (p.p257,)
    } else {
        (var_ldrifte__blk1189,)
    }
};
        var_ldrifte__blk1189 = assign35850_e50631;

        let (assign35860_e50640, assign35860_e50640_d_n0, assign35860_e50640_d_n2, assign35860_e50640_d_n6, assign35860_e50640_d_n7,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 == 0.0)) {
        let assign35860_e50638: f64 = (p.p50 * (nv0 - nv6));
        (assign35860_e50638, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (var_vrdr__blk1187, var_vrdr__blk1187_dn0, var_vrdr__blk1187_dn2, var_vrdr__blk1187_dn6, var_vrdr__blk1187_dn7,)
    }
};
        var_vrdr__blk1187 = assign35860_e50640;
        var_vrdr__blk1187_dn0 = assign35860_e50640_d_n0;
        var_vrdr__blk1187_dn2 = assign35860_e50640_d_n2;
        var_vrdr__blk1187_dn6 = assign35860_e50640_d_n6;
        var_vrdr__blk1187_dn7 = assign35860_e50640_d_n7;

        let (assign35870_e50651,) = {
    if (var_guard1179 != 0.0) {
        let assign35870_e50644: f64 = (p.p271 * p.p271);
        let assign35870_e50647: f64 = (p.p56 * p.p56);
        let assign35870_e50648: f64 = (assign35870_e50644 + assign35870_e50647);
        let assign35870_e50649: f64 = (assign35870_e50648).sqrt();
        (assign35870_e50649,)
    } else {
        (var_xov__blk1196,)
    }
};
        var_xov__blk1196 = assign35870_e50651;

        let (assign35880_e50657,) = {
    if (var_guard1179 != 0.0) {
        let assign35880_e50655: f64 = (var_weff * p.p9);
        (assign35880_e50655,)
    } else {
        (var_weff_nf__blk1198,)
    }
};
        var_weff_nf__blk1198 = assign35880_e50657;

        let (assign35890_e50663,) = {
    if (var_guard1179 != 0.0) {
        let assign35890_e50661: f64 = (var_mks_rdrmue__blk1183 / 10000.0);
        (assign35890_e50661,)
    } else {
        (var_mks_rdrmue__blk1183,)
    }
};
        var_mks_rdrmue__blk1183 = assign35890_e50663;

        let (assign35900_e50669,) = {
    if (var_guard1179 != 0.0) {
        let assign35900_e50667: f64 = (var_mks_rdrvmax__blk1184 / 100.0);
        (assign35900_e50667,)
    } else {
        (var_mks_rdrvmax__blk1184,)
    }
};
        var_mks_rdrvmax__blk1184 = assign35900_e50669;

        let (assign35910_e50675, assign35910_e50675_d_n10,) = {
    if (var_guard1179 != 0.0) {
        let assign35910_e50673: f64 = (var_ttemp / var_uc_tnom);
        (assign35910_e50673, (var_ttemp_dn10 / var_uc_tnom),)
    } else {
        (var_tratio__blk1188, var_tratio__blk1188_dn10,)
    }
};
        var_tratio__blk1188 = assign35910_e50675;
        var_tratio__blk1188_dn10 = assign35910_e50675_d_n10;

        let (assign35920_e50681, assign35920_e50681_d_n0, assign35920_e50681_d_n2, assign35920_e50681_d_n6, assign35920_e50681_d_n7, assign35920_e50681_d_n10, assign35920_e50681_d_n11, assign35920_e50681_d_n12, assign35920_e50681_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign35920_e50679: f64 = (var_tratio__blk1188).powf(p.p269);
        (assign35920_e50679, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((var_tratio__blk1188).powf(p.p269 - 1.0) * var_tratio__blk1188_dn10)) } } else { (assign35920_e50679 * (p.p269 * (var_tratio__blk1188_dn10 / var_tratio__blk1188))) }, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35920_e50681;
        var_t1_dn0 = assign35920_e50681_d_n0;
        var_t1_dn2 = assign35920_e50681_d_n2;
        var_t1_dn6 = assign35920_e50681_d_n6;
        var_t1_dn7 = assign35920_e50681_d_n7;
        var_t1_dn10 = assign35920_e50681_d_n10;
        var_t1_dn11 = assign35920_e50681_d_n11;
        var_t1_dn12 = assign35920_e50681_d_n12;
        var_t1_dn17 = assign35920_e50681_d_n17;

        let (assign35930_e50687, assign35930_e50687_d_n0, assign35930_e50687_d_n2, assign35930_e50687_d_n6, assign35930_e50687_d_n7, assign35930_e50687_d_n10, assign35930_e50687_d_n11, assign35930_e50687_d_n12, assign35930_e50687_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign35930_e50685: f64 = (var_mks_rdrmue__blk1183 / var_t1);
        (assign35930_e50685, (-((var_mks_rdrmue__blk1183 * var_t1_dn0) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn2) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn6) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn7) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn10) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn11) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn12) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn17) / (var_t1 * var_t1))),)
    } else {
        (var_mu0__blk1191, var_mu0__blk1191_dn0, var_mu0__blk1191_dn2, var_mu0__blk1191_dn6, var_mu0__blk1191_dn7, var_mu0__blk1191_dn10, var_mu0__blk1191_dn11, var_mu0__blk1191_dn12, var_mu0__blk1191_dn17,)
    }
};
        var_mu0__blk1191 = assign35930_e50687;
        var_mu0__blk1191_dn0 = assign35930_e50687_d_n0;
        var_mu0__blk1191_dn2 = assign35930_e50687_d_n2;
        var_mu0__blk1191_dn6 = assign35930_e50687_d_n6;
        var_mu0__blk1191_dn7 = assign35930_e50687_d_n7;
        var_mu0__blk1191_dn10 = assign35930_e50687_d_n10;
        var_mu0__blk1191_dn11 = assign35930_e50687_d_n11;
        var_mu0__blk1191_dn12 = assign35930_e50687_d_n12;
        var_mu0__blk1191_dn17 = assign35930_e50687_d_n17;

        let (assign35940_e50707, assign35940_e50707_d_n0, assign35940_e50707_d_n2, assign35940_e50707_d_n6, assign35940_e50707_d_n7, assign35940_e50707_d_n10, assign35940_e50707_d_n11, assign35940_e50707_d_n12, assign35940_e50707_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign35940_e50692: f64 = (0.4 * var_tratio__blk1188);
        let assign35940_e50693: f64 = (1.8 + assign35940_e50692);
        let assign35940_e50696: f64 = (0.1 * var_tratio__blk1188);
        let assign35940_e50698: f64 = (assign35940_e50696 * var_tratio__blk1188);
        let assign35940_e50699: f64 = (assign35940_e50693 + assign35940_e50698);
        let assign35940_e50703: f64 = (1.0 - var_tratio__blk1188);
        let assign35940_e50704: f64 = (p.p270 * assign35940_e50703);
        let assign35940_e50705: f64 = (assign35940_e50699 - assign35940_e50704);
        (assign35940_e50705, 0.0, 0.0, 0.0, 0.0, (((0.4 * var_tratio__blk1188_dn10) + (((0.1 * var_tratio__blk1188_dn10) * var_tratio__blk1188) + (assign35940_e50696 * var_tratio__blk1188_dn10))) - (p.p270 * (-var_tratio__blk1188_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn6, var_t0_dn7, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn17,)
    }
};
        var_t0 = assign35940_e50707;
        var_t0_dn0 = assign35940_e50707_d_n0;
        var_t0_dn2 = assign35940_e50707_d_n2;
        var_t0_dn6 = assign35940_e50707_d_n6;
        var_t0_dn7 = assign35940_e50707_d_n7;
        var_t0_dn10 = assign35940_e50707_d_n10;
        var_t0_dn11 = assign35940_e50707_d_n11;
        var_t0_dn12 = assign35940_e50707_d_n12;
        var_t0_dn17 = assign35940_e50707_d_n17;

        let (assign35950_e50713, assign35950_e50713_d_n0, assign35950_e50713_d_n2, assign35950_e50713_d_n6, assign35950_e50713_d_n7, assign35950_e50713_d_n10, assign35950_e50713_d_n11, assign35950_e50713_d_n12, assign35950_e50713_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign35950_e50711: f64 = (var_mks_rdrvmax__blk1184 / var_t0);
        (assign35950_e50711, (-((var_mks_rdrvmax__blk1184 * var_t0_dn0) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn2) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn6) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn7) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn10) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn11) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn12) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn17) / (var_t0 * var_t0))),)
    } else {
        (var_vmaxe__blk1192, var_vmaxe__blk1192_dn0, var_vmaxe__blk1192_dn2, var_vmaxe__blk1192_dn6, var_vmaxe__blk1192_dn7, var_vmaxe__blk1192_dn10, var_vmaxe__blk1192_dn11, var_vmaxe__blk1192_dn12, var_vmaxe__blk1192_dn17,)
    }
};
        var_vmaxe__blk1192 = assign35950_e50713;
        var_vmaxe__blk1192_dn0 = assign35950_e50713_d_n0;
        var_vmaxe__blk1192_dn2 = assign35950_e50713_d_n2;
        var_vmaxe__blk1192_dn6 = assign35950_e50713_d_n6;
        var_vmaxe__blk1192_dn7 = assign35950_e50713_d_n7;
        var_vmaxe__blk1192_dn10 = assign35950_e50713_d_n10;
        var_vmaxe__blk1192_dn11 = assign35950_e50713_d_n11;
        var_vmaxe__blk1192_dn12 = assign35950_e50713_d_n12;
        var_vmaxe__blk1192_dn17 = assign35950_e50713_d_n17;

        let (assign35960_e50723, assign35960_e50723_d_n10,) = {
    if (var_guard1179 != 0.0) {
        let assign35960_e50719: f64 = (var_ttemp - var_uc_tnom);
        let assign35960_e50720: f64 = (p.p274 * assign35960_e50719);
        let assign35960_e50721: f64 = (var_rrdrbb__blk1185 + assign35960_e50720);
        (assign35960_e50721, (var_rrdrbb__blk1185_dn10 + (p.p274 * var_ttemp_dn10)),)
    } else {
        (var_rrdrbb__blk1185, var_rrdrbb__blk1185_dn10,)
    }
};
        var_rrdrbb__blk1185 = assign35960_e50723;
        var_rrdrbb__blk1185_dn10 = assign35960_e50723_d_n10;

        let (assign35970_e50733,) = {
    if (var_guard1179 != 0.0) {
        let assign35970_e50729: f64 = (var_lgle).powf(p.p280);
        let assign35970_e50730: f64 = (p.p279 / assign35970_e50729);
        let assign35970_e50731: f64 = (1.0 + assign35970_e50730);
        (assign35970_e50731,)
    } else {
        (var_rdrmuele__blk1180,)
    }
};
        var_rdrmuele__blk1180 = assign35970_e50733;

        let (assign35980_e50743,) = {
    if (var_guard1179 != 0.0) {
        let assign35980_e50739: f64 = (var_lgle).powf(p.p278);
        let assign35980_e50740: f64 = (p.p277 / assign35980_e50739);
        let assign35980_e50741: f64 = (1.0 + assign35980_e50740);
        (assign35980_e50741,)
    } else {
        (var_rdrvmaxle__blk1182,)
    }
};
        var_rdrvmaxle__blk1182 = assign35980_e50743;

        let (assign35990_e50753,) = {
    if (var_guard1179 != 0.0) {
        let assign35990_e50749: f64 = (var_wg).powf(p.p276);
        let assign35990_e50750: f64 = (p.p275 / assign35990_e50749);
        let assign35990_e50751: f64 = (1.0 + assign35990_e50750);
        (assign35990_e50751,)
    } else {
        (var_rdrvmaxwe__blk1181,)
    }
};
        var_rdrvmaxwe__blk1181 = assign35990_e50753;

        let (assign36000_e50759, assign36000_e50759_d_n0, assign36000_e50759_d_n2, assign36000_e50759_d_n6, assign36000_e50759_d_n7, assign36000_e50759_d_n10, assign36000_e50759_d_n11, assign36000_e50759_d_n12, assign36000_e50759_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36000_e50757: f64 = (var_mu0__blk1191 * var_rdrmuele__blk1180);
        (assign36000_e50757, (var_mu0__blk1191_dn0 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn2 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn6 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn7 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn10 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn11 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn12 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn17 * var_rdrmuele__blk1180),)
    } else {
        (var_mu0__blk1191, var_mu0__blk1191_dn0, var_mu0__blk1191_dn2, var_mu0__blk1191_dn6, var_mu0__blk1191_dn7, var_mu0__blk1191_dn10, var_mu0__blk1191_dn11, var_mu0__blk1191_dn12, var_mu0__blk1191_dn17,)
    }
};
        var_mu0__blk1191 = assign36000_e50759;
        var_mu0__blk1191_dn0 = assign36000_e50759_d_n0;
        var_mu0__blk1191_dn2 = assign36000_e50759_d_n2;
        var_mu0__blk1191_dn6 = assign36000_e50759_d_n6;
        var_mu0__blk1191_dn7 = assign36000_e50759_d_n7;
        var_mu0__blk1191_dn10 = assign36000_e50759_d_n10;
        var_mu0__blk1191_dn11 = assign36000_e50759_d_n11;
        var_mu0__blk1191_dn12 = assign36000_e50759_d_n12;
        var_mu0__blk1191_dn17 = assign36000_e50759_d_n17;

        let (assign36010_e50769, assign36010_e50769_d_n0, assign36010_e50769_d_n2, assign36010_e50769_d_n6, assign36010_e50769_d_n7, assign36010_e50769_d_n10, assign36010_e50769_d_n11, assign36010_e50769_d_n12, assign36010_e50769_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36010_e50763: f64 = (var_vmaxe__blk1192 * var_rdrvmaxwe__blk1181);
        let assign36010_e50765: f64 = (assign36010_e50763 * var_rdrvmaxle__blk1182);
        let assign36010_e50767: f64 = (assign36010_e50765 + 1e-50);
        (assign36010_e50767, ((var_vmaxe__blk1192_dn0 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn2 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn6 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn7 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn10 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn11 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn12 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn17 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182),)
    } else {
        (var_vmaxe__blk1192, var_vmaxe__blk1192_dn0, var_vmaxe__blk1192_dn2, var_vmaxe__blk1192_dn6, var_vmaxe__blk1192_dn7, var_vmaxe__blk1192_dn10, var_vmaxe__blk1192_dn11, var_vmaxe__blk1192_dn12, var_vmaxe__blk1192_dn17,)
    }
};
        var_vmaxe__blk1192 = assign36010_e50769;
        var_vmaxe__blk1192_dn0 = assign36010_e50769_d_n0;
        var_vmaxe__blk1192_dn2 = assign36010_e50769_d_n2;
        var_vmaxe__blk1192_dn6 = assign36010_e50769_d_n6;
        var_vmaxe__blk1192_dn7 = assign36010_e50769_d_n7;
        var_vmaxe__blk1192_dn10 = assign36010_e50769_d_n10;
        var_vmaxe__blk1192_dn11 = assign36010_e50769_d_n11;
        var_vmaxe__blk1192_dn12 = assign36010_e50769_d_n12;
        var_vmaxe__blk1192_dn17 = assign36010_e50769_d_n17;

        let (assign36020_e50775, assign36020_e50775_d_n0, assign36020_e50775_d_n2, assign36020_e50775_d_n6, assign36020_e50775_d_n7,) = {
    if (var_guard1179 != 0.0) {
        let assign36020_e50773: f64 = (var_vrdr__blk1187 / var_ldrifte__blk1189);
        (assign36020_e50773, (var_vrdr__blk1187_dn0 / var_ldrifte__blk1189), (var_vrdr__blk1187_dn2 / var_ldrifte__blk1189), (var_vrdr__blk1187_dn6 / var_ldrifte__blk1189), (var_vrdr__blk1187_dn7 / var_ldrifte__blk1189),)
    } else {
        (var_edri__blk1193, var_edri__blk1193_dn0, var_edri__blk1193_dn2, var_edri__blk1193_dn6, var_edri__blk1193_dn7,)
    }
};
        var_edri__blk1193 = assign36020_e50775;
        var_edri__blk1193_dn0 = assign36020_e50775_d_n0;
        var_edri__blk1193_dn2 = assign36020_e50775_d_n2;
        var_edri__blk1193_dn6 = assign36020_e50775_d_n6;
        var_edri__blk1193_dn7 = assign36020_e50775_d_n7;

        *var_edri__blk1193_slot = var_edri__blk1193;
        *var_edri__blk1193_dn0_slot = var_edri__blk1193_dn0;
        *var_edri__blk1193_dn2_slot = var_edri__blk1193_dn2;
        *var_edri__blk1193_dn6_slot = var_edri__blk1193_dn6;
        *var_edri__blk1193_dn7_slot = var_edri__blk1193_dn7;
        *var_guard1178_slot = var_guard1178;
        *var_guard1179_slot = var_guard1179;
        *var_guard1199_slot = var_guard1199;
        *var_ldrifte__blk1189_slot = var_ldrifte__blk1189;
        *var_mks_rdrmue__blk1183_slot = var_mks_rdrmue__blk1183;
        *var_mks_rdrvmax__blk1184_slot = var_mks_rdrvmax__blk1184;
        *var_mu0__blk1191_slot = var_mu0__blk1191;
        *var_mu0__blk1191_dn0_slot = var_mu0__blk1191_dn0;
        *var_mu0__blk1191_dn10_slot = var_mu0__blk1191_dn10;
        *var_mu0__blk1191_dn11_slot = var_mu0__blk1191_dn11;
        *var_mu0__blk1191_dn12_slot = var_mu0__blk1191_dn12;
        *var_mu0__blk1191_dn17_slot = var_mu0__blk1191_dn17;
        *var_mu0__blk1191_dn2_slot = var_mu0__blk1191_dn2;
        *var_mu0__blk1191_dn6_slot = var_mu0__blk1191_dn6;
        *var_mu0__blk1191_dn7_slot = var_mu0__blk1191_dn7;
        *var_nover__blk1190_slot = var_nover__blk1190;
        *var_rdmod_slot = var_rdmod;
        *var_rdrmuele__blk1180_slot = var_rdrmuele__blk1180;
        *var_rdrvmaxle__blk1182_slot = var_rdrvmaxle__blk1182;
        *var_rdrvmaxwe__blk1181_slot = var_rdrvmaxwe__blk1181;
        *var_rrdrbb__blk1185_slot = var_rrdrbb__blk1185;
        *var_rrdrbb__blk1185_dn10_slot = var_rrdrbb__blk1185_dn10;
        *var_rsd_slot = var_rsd;
        *var_rsd0__blk1186_slot = var_rsd0__blk1186;
        *var_rsd_dn0_slot = var_rsd_dn0;
        *var_rsd_dn10_slot = var_rsd_dn10;
        *var_rsd_dn11_slot = var_rsd_dn11;
        *var_rsd_dn12_slot = var_rsd_dn12;
        *var_rsd_dn17_slot = var_rsd_dn17;
        *var_rsd_dn2_slot = var_rsd_dn2;
        *var_rsd_dn6_slot = var_rsd_dn6;
        *var_rsd_dn7_slot = var_rsd_dn7;
        *var_rsde_slot = var_rsde;
        *var_rsde_dn0_slot = var_rsde_dn0;
        *var_rsde_dn10_slot = var_rsde_dn10;
        *var_rsde_dn11_slot = var_rsde_dn11;
        *var_rsde_dn12_slot = var_rsde_dn12;
        *var_rsde_dn17_slot = var_rsde_dn17;
        *var_rsde_dn2_slot = var_rsde_dn2;
        *var_rsde_dn6_slot = var_rsde_dn6;
        *var_rsde_dn7_slot = var_rsde_dn7;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_tratio__blk1188_slot = var_tratio__blk1188;
        *var_tratio__blk1188_dn10_slot = var_tratio__blk1188_dn10;
        *var_vmaxe__blk1192_slot = var_vmaxe__blk1192;
        *var_vmaxe__blk1192_dn0_slot = var_vmaxe__blk1192_dn0;
        *var_vmaxe__blk1192_dn10_slot = var_vmaxe__blk1192_dn10;
        *var_vmaxe__blk1192_dn11_slot = var_vmaxe__blk1192_dn11;
        *var_vmaxe__blk1192_dn12_slot = var_vmaxe__blk1192_dn12;
        *var_vmaxe__blk1192_dn17_slot = var_vmaxe__blk1192_dn17;
        *var_vmaxe__blk1192_dn2_slot = var_vmaxe__blk1192_dn2;
        *var_vmaxe__blk1192_dn6_slot = var_vmaxe__blk1192_dn6;
        *var_vmaxe__blk1192_dn7_slot = var_vmaxe__blk1192_dn7;
        *var_vrdr__blk1187_slot = var_vrdr__blk1187;
        *var_vrdr__blk1187_dn0_slot = var_vrdr__blk1187_dn0;
        *var_vrdr__blk1187_dn2_slot = var_vrdr__blk1187_dn2;
        *var_vrdr__blk1187_dn6_slot = var_vrdr__blk1187_dn6;
        *var_vrdr__blk1187_dn7_slot = var_vrdr__blk1187_dn7;
        *var_weff_nf__blk1198_slot = var_weff_nf__blk1198;
        *var_xov__blk1196_slot = var_xov__blk1196;
    }

    pub(super) fn stamp_transient_block_125(
        p: &Parameters,
        var_edri__blk1193: f64,
        var_edri__blk1193_dn0: f64,
        var_edri__blk1193_dn2: f64,
        var_edri__blk1193_dn6: f64,
        var_edri__blk1193_dn7: f64,
        var_flg_nqs: f64,
        var_guard1179: f64,
        var_ldrifte__blk1189: f64,
        var_mfactor: f64,
        var_mode: f64,
        var_mu0__blk1191: f64,
        var_mu0__blk1191_dn0: f64,
        var_mu0__blk1191_dn10: f64,
        var_mu0__blk1191_dn11: f64,
        var_mu0__blk1191_dn12: f64,
        var_mu0__blk1191_dn17: f64,
        var_mu0__blk1191_dn2: f64,
        var_mu0__blk1191_dn6: f64,
        var_mu0__blk1191_dn7: f64,
        var_nover__blk1190: f64,
        var_qi_nqs: f64,
        var_qi_nqs_dn18: f64,
        var_qi_qs: f64,
        var_qi_qs_dn0: f64,
        var_qi_qs_dn10: f64,
        var_qi_qs_dn11: f64,
        var_qi_qs_dn12: f64,
        var_qi_qs_dn17: f64,
        var_qi_qs_dn2: f64,
        var_qi_qs_dn6: f64,
        var_qi_qs_dn7: f64,
        var_rrdrbb__blk1185: f64,
        var_rrdrbb__blk1185_dn10: f64,
        var_rsd0__blk1186: f64,
        var_vmaxe__blk1192: f64,
        var_vmaxe__blk1192_dn0: f64,
        var_vmaxe__blk1192_dn10: f64,
        var_vmaxe__blk1192_dn11: f64,
        var_vmaxe__blk1192_dn12: f64,
        var_vmaxe__blk1192_dn17: f64,
        var_vmaxe__blk1192_dn2: f64,
        var_vmaxe__blk1192_dn6: f64,
        var_vmaxe__blk1192_dn7: f64,
        var_vrdr__blk1187: f64,
        var_weff_nf__blk1198: f64,
        var_xd: f64,
        var_xd_dn0: f64,
        var_xd_dn10: f64,
        var_xd_dn11: f64,
        var_xd_dn12: f64,
        var_xd_dn17: f64,
        var_xd_dn2: f64,
        var_xd_dn6: f64,
        var_xd_dn7: f64,
        var_xov__blk1196: f64,
        var_gd__blk1197_slot: &mut f64,
        var_gd__blk1197_dn0_slot: &mut f64,
        var_gd__blk1197_dn10_slot: &mut f64,
        var_gd__blk1197_dn11_slot: &mut f64,
        var_gd__blk1197_dn12_slot: &mut f64,
        var_gd__blk1197_dn17_slot: &mut f64,
        var_gd__blk1197_dn2_slot: &mut f64,
        var_gd__blk1197_dn6_slot: &mut f64,
        var_gd__blk1197_dn7_slot: &mut f64,
        var_guard1200_slot: &mut f64,
        var_guard1201_slot: &mut f64,
        var_guard1202_slot: &mut f64,
        var_guard1203_slot: &mut f64,
        var_guard1204_slot: &mut f64,
        var_guard1205_slot: &mut f64,
        var_guard1206_slot: &mut f64,
        var_guard1207_slot: &mut f64,
        var_guard1208_slot: &mut f64,
        var_guard1209_slot: &mut f64,
        var_iqi_nqs_slot: &mut f64,
        var_iqi_nqs_dn0_slot: &mut f64,
        var_iqi_nqs_dn10_slot: &mut f64,
        var_iqi_nqs_dn11_slot: &mut f64,
        var_iqi_nqs_dn12_slot: &mut f64,
        var_iqi_nqs_dn17_slot: &mut f64,
        var_iqi_nqs_dn18_slot: &mut f64,
        var_iqi_nqs_dn2_slot: &mut f64,
        var_iqi_nqs_dn6_slot: &mut f64,
        var_iqi_nqs_dn7_slot: &mut f64,
        var_mu__blk1195_slot: &mut f64,
        var_mu__blk1195_dn0_slot: &mut f64,
        var_mu__blk1195_dn10_slot: &mut f64,
        var_mu__blk1195_dn11_slot: &mut f64,
        var_mu__blk1195_dn12_slot: &mut f64,
        var_mu__blk1195_dn17_slot: &mut f64,
        var_mu__blk1195_dn2_slot: &mut f64,
        var_mu__blk1195_dn6_slot: &mut f64,
        var_mu__blk1195_dn7_slot: &mut f64,
        var_qdrat_slot: &mut f64,
        var_qdrat_dn0_slot: &mut f64,
        var_qdrat_dn10_slot: &mut f64,
        var_qdrat_dn11_slot: &mut f64,
        var_qdrat_dn12_slot: &mut f64,
        var_qdrat_dn17_slot: &mut f64,
        var_qdrat_dn2_slot: &mut f64,
        var_qdrat_dn6_slot: &mut f64,
        var_qdrat_dn7_slot: &mut f64,
        var_rdde_slot: &mut f64,
        var_rdde_dn0_slot: &mut f64,
        var_rdde_dn10_slot: &mut f64,
        var_rdde_dn11_slot: &mut f64,
        var_rdde_dn12_slot: &mut f64,
        var_rdde_dn17_slot: &mut f64,
        var_rdde_dn2_slot: &mut f64,
        var_rdde_dn6_slot: &mut f64,
        var_rdde_dn7_slot: &mut f64,
        var_rsd_slot: &mut f64,
        var_rsd_dn0_slot: &mut f64,
        var_rsd_dn10_slot: &mut f64,
        var_rsd_dn11_slot: &mut f64,
        var_rsd_dn12_slot: &mut f64,
        var_rsd_dn17_slot: &mut f64,
        var_rsd_dn2_slot: &mut f64,
        var_rsd_dn6_slot: &mut f64,
        var_rsd_dn7_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn17_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn17_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn17_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_tau_slot: &mut f64,
        var_tau_dn0_slot: &mut f64,
        var_tau_dn10_slot: &mut f64,
        var_tau_dn11_slot: &mut f64,
        var_tau_dn12_slot: &mut f64,
        var_tau_dn17_slot: &mut f64,
        var_tau_dn2_slot: &mut f64,
        var_tau_dn6_slot: &mut f64,
        var_tau_dn7_slot: &mut f64,
        var_taub_slot: &mut f64,
        var_taub_dn0_slot: &mut f64,
        var_taub_dn10_slot: &mut f64,
        var_taub_dn11_slot: &mut f64,
        var_taub_dn12_slot: &mut f64,
        var_taub_dn17_slot: &mut f64,
        var_taub_dn2_slot: &mut f64,
        var_taub_dn6_slot: &mut f64,
        var_taub_dn7_slot: &mut f64,
        var_vdri__blk1194_slot: &mut f64,
        var_vdri__blk1194_dn0_slot: &mut f64,
        var_vdri__blk1194_dn10_slot: &mut f64,
        var_vdri__blk1194_dn11_slot: &mut f64,
        var_vdri__blk1194_dn12_slot: &mut f64,
        var_vdri__blk1194_dn17_slot: &mut f64,
        var_vdri__blk1194_dn2_slot: &mut f64,
        var_vdri__blk1194_dn6_slot: &mut f64,
        var_vdri__blk1194_dn7_slot: &mut f64,
    ) {
        let mut var_gd__blk1197: f64 = *var_gd__blk1197_slot;
        let mut var_gd__blk1197_dn0: f64 = *var_gd__blk1197_dn0_slot;
        let mut var_gd__blk1197_dn10: f64 = *var_gd__blk1197_dn10_slot;
        let mut var_gd__blk1197_dn11: f64 = *var_gd__blk1197_dn11_slot;
        let mut var_gd__blk1197_dn12: f64 = *var_gd__blk1197_dn12_slot;
        let mut var_gd__blk1197_dn17: f64 = *var_gd__blk1197_dn17_slot;
        let mut var_gd__blk1197_dn2: f64 = *var_gd__blk1197_dn2_slot;
        let mut var_gd__blk1197_dn6: f64 = *var_gd__blk1197_dn6_slot;
        let mut var_gd__blk1197_dn7: f64 = *var_gd__blk1197_dn7_slot;
        let mut var_guard1200: f64 = *var_guard1200_slot;
        let mut var_guard1201: f64 = *var_guard1201_slot;
        let mut var_guard1202: f64 = *var_guard1202_slot;
        let mut var_guard1203: f64 = *var_guard1203_slot;
        let mut var_guard1204: f64 = *var_guard1204_slot;
        let mut var_guard1205: f64 = *var_guard1205_slot;
        let mut var_guard1206: f64 = *var_guard1206_slot;
        let mut var_guard1207: f64 = *var_guard1207_slot;
        let mut var_guard1208: f64 = *var_guard1208_slot;
        let mut var_guard1209: f64 = *var_guard1209_slot;
        let mut var_iqi_nqs: f64 = *var_iqi_nqs_slot;
        let mut var_iqi_nqs_dn0: f64 = *var_iqi_nqs_dn0_slot;
        let mut var_iqi_nqs_dn10: f64 = *var_iqi_nqs_dn10_slot;
        let mut var_iqi_nqs_dn11: f64 = *var_iqi_nqs_dn11_slot;
        let mut var_iqi_nqs_dn12: f64 = *var_iqi_nqs_dn12_slot;
        let mut var_iqi_nqs_dn17: f64 = *var_iqi_nqs_dn17_slot;
        let mut var_iqi_nqs_dn18: f64 = *var_iqi_nqs_dn18_slot;
        let mut var_iqi_nqs_dn2: f64 = *var_iqi_nqs_dn2_slot;
        let mut var_iqi_nqs_dn6: f64 = *var_iqi_nqs_dn6_slot;
        let mut var_iqi_nqs_dn7: f64 = *var_iqi_nqs_dn7_slot;
        let mut var_mu__blk1195: f64 = *var_mu__blk1195_slot;
        let mut var_mu__blk1195_dn0: f64 = *var_mu__blk1195_dn0_slot;
        let mut var_mu__blk1195_dn10: f64 = *var_mu__blk1195_dn10_slot;
        let mut var_mu__blk1195_dn11: f64 = *var_mu__blk1195_dn11_slot;
        let mut var_mu__blk1195_dn12: f64 = *var_mu__blk1195_dn12_slot;
        let mut var_mu__blk1195_dn17: f64 = *var_mu__blk1195_dn17_slot;
        let mut var_mu__blk1195_dn2: f64 = *var_mu__blk1195_dn2_slot;
        let mut var_mu__blk1195_dn6: f64 = *var_mu__blk1195_dn6_slot;
        let mut var_mu__blk1195_dn7: f64 = *var_mu__blk1195_dn7_slot;
        let mut var_qdrat: f64 = *var_qdrat_slot;
        let mut var_qdrat_dn0: f64 = *var_qdrat_dn0_slot;
        let mut var_qdrat_dn10: f64 = *var_qdrat_dn10_slot;
        let mut var_qdrat_dn11: f64 = *var_qdrat_dn11_slot;
        let mut var_qdrat_dn12: f64 = *var_qdrat_dn12_slot;
        let mut var_qdrat_dn17: f64 = *var_qdrat_dn17_slot;
        let mut var_qdrat_dn2: f64 = *var_qdrat_dn2_slot;
        let mut var_qdrat_dn6: f64 = *var_qdrat_dn6_slot;
        let mut var_qdrat_dn7: f64 = *var_qdrat_dn7_slot;
        let mut var_rdde: f64 = *var_rdde_slot;
        let mut var_rdde_dn0: f64 = *var_rdde_dn0_slot;
        let mut var_rdde_dn10: f64 = *var_rdde_dn10_slot;
        let mut var_rdde_dn11: f64 = *var_rdde_dn11_slot;
        let mut var_rdde_dn12: f64 = *var_rdde_dn12_slot;
        let mut var_rdde_dn17: f64 = *var_rdde_dn17_slot;
        let mut var_rdde_dn2: f64 = *var_rdde_dn2_slot;
        let mut var_rdde_dn6: f64 = *var_rdde_dn6_slot;
        let mut var_rdde_dn7: f64 = *var_rdde_dn7_slot;
        let mut var_rsd: f64 = *var_rsd_slot;
        let mut var_rsd_dn0: f64 = *var_rsd_dn0_slot;
        let mut var_rsd_dn10: f64 = *var_rsd_dn10_slot;
        let mut var_rsd_dn11: f64 = *var_rsd_dn11_slot;
        let mut var_rsd_dn12: f64 = *var_rsd_dn12_slot;
        let mut var_rsd_dn17: f64 = *var_rsd_dn17_slot;
        let mut var_rsd_dn2: f64 = *var_rsd_dn2_slot;
        let mut var_rsd_dn6: f64 = *var_rsd_dn6_slot;
        let mut var_rsd_dn7: f64 = *var_rsd_dn7_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn17: f64 = *var_t4_dn17_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn17: f64 = *var_t5_dn17_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn17: f64 = *var_t6_dn17_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_tau: f64 = *var_tau_slot;
        let mut var_tau_dn0: f64 = *var_tau_dn0_slot;
        let mut var_tau_dn10: f64 = *var_tau_dn10_slot;
        let mut var_tau_dn11: f64 = *var_tau_dn11_slot;
        let mut var_tau_dn12: f64 = *var_tau_dn12_slot;
        let mut var_tau_dn17: f64 = *var_tau_dn17_slot;
        let mut var_tau_dn2: f64 = *var_tau_dn2_slot;
        let mut var_tau_dn6: f64 = *var_tau_dn6_slot;
        let mut var_tau_dn7: f64 = *var_tau_dn7_slot;
        let mut var_taub: f64 = *var_taub_slot;
        let mut var_taub_dn0: f64 = *var_taub_dn0_slot;
        let mut var_taub_dn10: f64 = *var_taub_dn10_slot;
        let mut var_taub_dn11: f64 = *var_taub_dn11_slot;
        let mut var_taub_dn12: f64 = *var_taub_dn12_slot;
        let mut var_taub_dn17: f64 = *var_taub_dn17_slot;
        let mut var_taub_dn2: f64 = *var_taub_dn2_slot;
        let mut var_taub_dn6: f64 = *var_taub_dn6_slot;
        let mut var_taub_dn7: f64 = *var_taub_dn7_slot;
        let mut var_vdri__blk1194: f64 = *var_vdri__blk1194_slot;
        let mut var_vdri__blk1194_dn0: f64 = *var_vdri__blk1194_dn0_slot;
        let mut var_vdri__blk1194_dn10: f64 = *var_vdri__blk1194_dn10_slot;
        let mut var_vdri__blk1194_dn11: f64 = *var_vdri__blk1194_dn11_slot;
        let mut var_vdri__blk1194_dn12: f64 = *var_vdri__blk1194_dn12_slot;
        let mut var_vdri__blk1194_dn17: f64 = *var_vdri__blk1194_dn17_slot;
        let mut var_vdri__blk1194_dn2: f64 = *var_vdri__blk1194_dn2_slot;
        let mut var_vdri__blk1194_dn6: f64 = *var_vdri__blk1194_dn6_slot;
        let mut var_vdri__blk1194_dn7: f64 = *var_vdri__blk1194_dn7_slot;

        let (assign36030_e50781, assign36030_e50781_d_n0, assign36030_e50781_d_n2, assign36030_e50781_d_n6, assign36030_e50781_d_n7, assign36030_e50781_d_n10, assign36030_e50781_d_n11, assign36030_e50781_d_n12, assign36030_e50781_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36030_e50779: f64 = (var_mu0__blk1191 * var_edri__blk1193);
        (assign36030_e50779, ((var_mu0__blk1191_dn0 * var_edri__blk1193) + (var_mu0__blk1191 * var_edri__blk1193_dn0)), ((var_mu0__blk1191_dn2 * var_edri__blk1193) + (var_mu0__blk1191 * var_edri__blk1193_dn2)), ((var_mu0__blk1191_dn6 * var_edri__blk1193) + (var_mu0__blk1191 * var_edri__blk1193_dn6)), ((var_mu0__blk1191_dn7 * var_edri__blk1193) + (var_mu0__blk1191 * var_edri__blk1193_dn7)), (var_mu0__blk1191_dn10 * var_edri__blk1193), (var_mu0__blk1191_dn11 * var_edri__blk1193), (var_mu0__blk1191_dn12 * var_edri__blk1193), (var_mu0__blk1191_dn17 * var_edri__blk1193),)
    } else {
        (var_vdri__blk1194, var_vdri__blk1194_dn0, var_vdri__blk1194_dn2, var_vdri__blk1194_dn6, var_vdri__blk1194_dn7, var_vdri__blk1194_dn10, var_vdri__blk1194_dn11, var_vdri__blk1194_dn12, var_vdri__blk1194_dn17,)
    }
};
        var_vdri__blk1194 = assign36030_e50781;
        var_vdri__blk1194_dn0 = assign36030_e50781_d_n0;
        var_vdri__blk1194_dn2 = assign36030_e50781_d_n2;
        var_vdri__blk1194_dn6 = assign36030_e50781_d_n6;
        var_vdri__blk1194_dn7 = assign36030_e50781_d_n7;
        var_vdri__blk1194_dn10 = assign36030_e50781_d_n10;
        var_vdri__blk1194_dn11 = assign36030_e50781_d_n11;
        var_vdri__blk1194_dn12 = assign36030_e50781_d_n12;
        var_vdri__blk1194_dn17 = assign36030_e50781_d_n17;

        let assign36040_e50784: f64 = if var_vrdr__blk1187 >= 0.0 { 1.0 } else { 0.0 };
        var_guard1200 = assign36040_e50784;

        let (assign36050_e50792, assign36050_e50792_d_n0, assign36050_e50792_d_n2, assign36050_e50792_d_n6, assign36050_e50792_d_n7, assign36050_e50792_d_n10, assign36050_e50792_d_n11, assign36050_e50792_d_n12, assign36050_e50792_d_n17,) = {
    if ((var_guard1179 != 0.0) && (var_guard1200 != 0.0)) {
        let assign36050_e50790: f64 = (var_vdri__blk1194 / var_vmaxe__blk1192);
        (assign36050_e50790, (((var_vdri__blk1194_dn0 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn0)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn2 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn2)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn6 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn6)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn7 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn7)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn10 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn10)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn11 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn11)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn12 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn12)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn17 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn17)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign36050_e50792;
        var_t1_dn0 = assign36050_e50792_d_n0;
        var_t1_dn2 = assign36050_e50792_d_n2;
        var_t1_dn6 = assign36050_e50792_d_n6;
        var_t1_dn7 = assign36050_e50792_d_n7;
        var_t1_dn10 = assign36050_e50792_d_n10;
        var_t1_dn11 = assign36050_e50792_d_n11;
        var_t1_dn12 = assign36050_e50792_d_n12;
        var_t1_dn17 = assign36050_e50792_d_n17;

        let (assign36060_e50802, assign36060_e50802_d_n0, assign36060_e50802_d_n2, assign36060_e50802_d_n6, assign36060_e50802_d_n7, assign36060_e50802_d_n10, assign36060_e50802_d_n11, assign36060_e50802_d_n12, assign36060_e50802_d_n17,) = {
    if ((var_guard1179 != 0.0) && (var_guard1200 == 0.0)) {
        let assign36060_e50798: f64 = (-var_vdri__blk1194);
        let assign36060_e50800: f64 = (assign36060_e50798 / var_vmaxe__blk1192);
        (assign36060_e50800, ((((-var_vdri__blk1194_dn0) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn0)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn2) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn2)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn6) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn6)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn7) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn7)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn10) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn10)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn11) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn11)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn12) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn12)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn17) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn17)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign36060_e50802;
        var_t1_dn0 = assign36060_e50802_d_n0;
        var_t1_dn2 = assign36060_e50802_d_n2;
        var_t1_dn6 = assign36060_e50802_d_n6;
        var_t1_dn7 = assign36060_e50802_d_n7;
        var_t1_dn10 = assign36060_e50802_d_n10;
        var_t1_dn11 = assign36060_e50802_d_n11;
        var_t1_dn12 = assign36060_e50802_d_n12;
        var_t1_dn17 = assign36060_e50802_d_n17;

        let assign36070_e50806: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50807: f64 = (1.0 - assign36070_e50806);
        let assign36070_e50814: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50815: f64 = (1.0 + assign36070_e50814);
        let assign36070_e50817: f64 = if ((assign36070_e50807 <= var_rrdrbb__blk1185) && (var_rrdrbb__blk1185 <= assign36070_e50815)) { 1.0 } else { 0.0 };
        var_guard1201 = assign36070_e50817;

        let (assign36080_e50823, assign36080_e50823_d_n0, assign36080_e50823_d_n2, assign36080_e50823_d_n6, assign36080_e50823_d_n7, assign36080_e50823_d_n10, assign36080_e50823_d_n11, assign36080_e50823_d_n12, assign36080_e50823_d_n17,) = {
    if ((var_guard1179 != 0.0) && (var_guard1201 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign36080_e50823;
        var_t3_dn0 = assign36080_e50823_d_n0;
        var_t3_dn2 = assign36080_e50823_d_n2;
        var_t3_dn6 = assign36080_e50823_d_n6;
        var_t3_dn7 = assign36080_e50823_d_n7;
        var_t3_dn10 = assign36080_e50823_d_n10;
        var_t3_dn11 = assign36080_e50823_d_n11;
        var_t3_dn12 = assign36080_e50823_d_n12;
        var_t3_dn17 = assign36080_e50823_d_n17;

        let assign36090_e50827: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50828: f64 = (2.0 - assign36090_e50827);
        let assign36090_e50835: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50836: f64 = (2.0 + assign36090_e50835);
        let assign36090_e50838: f64 = if ((assign36090_e50828 <= var_rrdrbb__blk1185) && (var_rrdrbb__blk1185 <= assign36090_e50836)) { 1.0 } else { 0.0 };
        var_guard1202 = assign36090_e50838;

        let (assign36100_e50847, assign36100_e50847_d_n0, assign36100_e50847_d_n2, assign36100_e50847_d_n6, assign36100_e50847_d_n7, assign36100_e50847_d_n10, assign36100_e50847_d_n11, assign36100_e50847_d_n12, assign36100_e50847_d_n17,) = {
    if (((var_guard1179 != 0.0) && (var_guard1201 == 0.0)) && (var_guard1202 != 0.0)) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign36100_e50847;
        var_t3_dn0 = assign36100_e50847_d_n0;
        var_t3_dn2 = assign36100_e50847_d_n2;
        var_t3_dn6 = assign36100_e50847_d_n6;
        var_t3_dn7 = assign36100_e50847_d_n7;
        var_t3_dn10 = assign36100_e50847_d_n10;
        var_t3_dn11 = assign36100_e50847_d_n11;
        var_t3_dn12 = assign36100_e50847_d_n12;
        var_t3_dn17 = assign36100_e50847_d_n17;

        let (assign36110_e50861, assign36110_e50861_d_n0, assign36110_e50861_d_n2, assign36110_e50861_d_n6, assign36110_e50861_d_n7, assign36110_e50861_d_n10, assign36110_e50861_d_n11, assign36110_e50861_d_n12, assign36110_e50861_d_n17,) = {
    if (((var_guard1179 != 0.0) && (var_guard1201 == 0.0)) && (var_guard1202 == 0.0)) {
        let assign36110_e50858: f64 = (var_rrdrbb__blk1185 - 1.0);
        let assign36110_e50859: f64 = (var_t1).powf(assign36110_e50858);
        (assign36110_e50859, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn0)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn0 / var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn2)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn2 / var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn6)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn6 / var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn7)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn7 / var_t1))) }, if var_rrdrbb__blk1185_dn10 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn10)) } } else { (assign36110_e50859 * ((var_rrdrbb__blk1185_dn10 * (var_t1).ln()) + (assign36110_e50858 * (var_t1_dn10 / var_t1)))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn11)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn11 / var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn12)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn12 / var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn17)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn17 / var_t1))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign36110_e50861;
        var_t3_dn0 = assign36110_e50861_d_n0;
        var_t3_dn2 = assign36110_e50861_d_n2;
        var_t3_dn6 = assign36110_e50861_d_n6;
        var_t3_dn7 = assign36110_e50861_d_n7;
        var_t3_dn10 = assign36110_e50861_d_n10;
        var_t3_dn11 = assign36110_e50861_d_n11;
        var_t3_dn12 = assign36110_e50861_d_n12;
        var_t3_dn17 = assign36110_e50861_d_n17;

        let (assign36120_e50867, assign36120_e50867_d_n0, assign36120_e50867_d_n2, assign36120_e50867_d_n6, assign36120_e50867_d_n7, assign36120_e50867_d_n10, assign36120_e50867_d_n11, assign36120_e50867_d_n12, assign36120_e50867_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36120_e50865: f64 = (var_t1 * var_t3);
        (assign36120_e50865, ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)), ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)), ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)), ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)), ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)), ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)), ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)), ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign36120_e50867;
        var_t2_dn0 = assign36120_e50867_d_n0;
        var_t2_dn2 = assign36120_e50867_d_n2;
        var_t2_dn6 = assign36120_e50867_d_n6;
        var_t2_dn7 = assign36120_e50867_d_n7;
        var_t2_dn10 = assign36120_e50867_d_n10;
        var_t2_dn11 = assign36120_e50867_d_n11;
        var_t2_dn12 = assign36120_e50867_d_n12;
        var_t2_dn17 = assign36120_e50867_d_n17;

        let (assign36130_e50873, assign36130_e50873_d_n0, assign36130_e50873_d_n2, assign36130_e50873_d_n6, assign36130_e50873_d_n7, assign36130_e50873_d_n10, assign36130_e50873_d_n11, assign36130_e50873_d_n12, assign36130_e50873_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36130_e50871: f64 = (1.0 + var_t2);
        (assign36130_e50871, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign36130_e50873;
        var_t4_dn0 = assign36130_e50873_d_n0;
        var_t4_dn2 = assign36130_e50873_d_n2;
        var_t4_dn6 = assign36130_e50873_d_n6;
        var_t4_dn7 = assign36130_e50873_d_n7;
        var_t4_dn10 = assign36130_e50873_d_n10;
        var_t4_dn11 = assign36130_e50873_d_n11;
        var_t4_dn12 = assign36130_e50873_d_n12;
        var_t4_dn17 = assign36130_e50873_d_n17;

        let assign36140_e50877: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50878: f64 = (1.0 - assign36140_e50877);
        let assign36140_e50885: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50886: f64 = (1.0 + assign36140_e50885);
        let assign36140_e50888: f64 = if ((assign36140_e50878 <= var_rrdrbb__blk1185) && (var_rrdrbb__blk1185 <= assign36140_e50886)) { 1.0 } else { 0.0 };
        var_guard1203 = assign36140_e50888;

        let (assign36150_e50896, assign36150_e50896_d_n0, assign36150_e50896_d_n2, assign36150_e50896_d_n6, assign36150_e50896_d_n7, assign36150_e50896_d_n10, assign36150_e50896_d_n11, assign36150_e50896_d_n12, assign36150_e50896_d_n17,) = {
    if ((var_guard1179 != 0.0) && (var_guard1203 != 0.0)) {
        let assign36150_e50894: f64 = (1.0 / var_t4);
        (assign36150_e50894, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn11 / (var_t4 * var_t4))), (-(var_t4_dn12 / (var_t4 * var_t4))), (-(var_t4_dn17 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign36150_e50896;
        var_t5_dn0 = assign36150_e50896_d_n0;
        var_t5_dn2 = assign36150_e50896_d_n2;
        var_t5_dn6 = assign36150_e50896_d_n6;
        var_t5_dn7 = assign36150_e50896_d_n7;
        var_t5_dn10 = assign36150_e50896_d_n10;
        var_t5_dn11 = assign36150_e50896_d_n11;
        var_t5_dn12 = assign36150_e50896_d_n12;
        var_t5_dn17 = assign36150_e50896_d_n17;

        let assign36160_e50900: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50901: f64 = (2.0 - assign36160_e50900);
        let assign36160_e50908: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50909: f64 = (2.0 + assign36160_e50908);
        let assign36160_e50911: f64 = if ((assign36160_e50901 <= var_rrdrbb__blk1185) && (var_rrdrbb__blk1185 <= assign36160_e50909)) { 1.0 } else { 0.0 };
        var_guard1204 = assign36160_e50911;

        let (assign36170_e50923, assign36170_e50923_d_n0, assign36170_e50923_d_n2, assign36170_e50923_d_n6, assign36170_e50923_d_n7, assign36170_e50923_d_n10, assign36170_e50923_d_n11, assign36170_e50923_d_n12, assign36170_e50923_d_n17,) = {
    if (((var_guard1179 != 0.0) && (var_guard1203 == 0.0)) && (var_guard1204 != 0.0)) {
        let assign36170_e50920: f64 = (var_t4).sqrt();
        let assign36170_e50921: f64 = (1.0 / assign36170_e50920);
        (assign36170_e50921, (-((var_t4_dn0 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn2 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn6 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn7 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn10 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn11 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn12 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn17 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign36170_e50923;
        var_t5_dn0 = assign36170_e50923_d_n0;
        var_t5_dn2 = assign36170_e50923_d_n2;
        var_t5_dn6 = assign36170_e50923_d_n6;
        var_t5_dn7 = assign36170_e50923_d_n7;
        var_t5_dn10 = assign36170_e50923_d_n10;
        var_t5_dn11 = assign36170_e50923_d_n11;
        var_t5_dn12 = assign36170_e50923_d_n12;
        var_t5_dn17 = assign36170_e50923_d_n17;

        let (assign36180_e50940, assign36180_e50940_d_n0, assign36180_e50940_d_n2, assign36180_e50940_d_n6, assign36180_e50940_d_n7, assign36180_e50940_d_n10, assign36180_e50940_d_n11, assign36180_e50940_d_n12, assign36180_e50940_d_n17,) = {
    if (((var_guard1179 != 0.0) && (var_guard1203 == 0.0)) && (var_guard1204 == 0.0)) {
        let assign36180_e50933: f64 = (-1.0);
        let assign36180_e50935: f64 = (assign36180_e50933 / var_rrdrbb__blk1185);
        let assign36180_e50937: f64 = (assign36180_e50935 - 1.0);
        let assign36180_e50938: f64 = (var_t4).powf(assign36180_e50937);
        (assign36180_e50938, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn0)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn0 / var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn2)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn2 / var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn6)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn6 / var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn7)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn7 / var_t4))) }, if (-((assign36180_e50933 * var_rrdrbb__blk1185_dn10) / (var_rrdrbb__blk1185 * var_rrdrbb__blk1185))) == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn10)) } } else { (assign36180_e50938 * (((-((assign36180_e50933 * var_rrdrbb__blk1185_dn10) / (var_rrdrbb__blk1185 * var_rrdrbb__blk1185))) * (var_t4).ln()) + (assign36180_e50937 * (var_t4_dn10 / var_t4)))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn11)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn11 / var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn12)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn12 / var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn17)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn17 / var_t4))) },)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn6, var_t6_dn7, var_t6_dn10, var_t6_dn11, var_t6_dn12, var_t6_dn17,)
    }
};
        var_t6 = assign36180_e50940;
        var_t6_dn0 = assign36180_e50940_d_n0;
        var_t6_dn2 = assign36180_e50940_d_n2;
        var_t6_dn6 = assign36180_e50940_d_n6;
        var_t6_dn7 = assign36180_e50940_d_n7;
        var_t6_dn10 = assign36180_e50940_d_n10;
        var_t6_dn11 = assign36180_e50940_d_n11;
        var_t6_dn12 = assign36180_e50940_d_n12;
        var_t6_dn17 = assign36180_e50940_d_n17;

        let (assign36190_e50952, assign36190_e50952_d_n0, assign36190_e50952_d_n2, assign36190_e50952_d_n6, assign36190_e50952_d_n7, assign36190_e50952_d_n10, assign36190_e50952_d_n11, assign36190_e50952_d_n12, assign36190_e50952_d_n17,) = {
    if (((var_guard1179 != 0.0) && (var_guard1203 == 0.0)) && (var_guard1204 == 0.0)) {
        let assign36190_e50950: f64 = (var_t4 * var_t6);
        (assign36190_e50950, ((var_t4_dn0 * var_t6) + (var_t4 * var_t6_dn0)), ((var_t4_dn2 * var_t6) + (var_t4 * var_t6_dn2)), ((var_t4_dn6 * var_t6) + (var_t4 * var_t6_dn6)), ((var_t4_dn7 * var_t6) + (var_t4 * var_t6_dn7)), ((var_t4_dn10 * var_t6) + (var_t4 * var_t6_dn10)), ((var_t4_dn11 * var_t6) + (var_t4 * var_t6_dn11)), ((var_t4_dn12 * var_t6) + (var_t4 * var_t6_dn12)), ((var_t4_dn17 * var_t6) + (var_t4 * var_t6_dn17)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign36190_e50952;
        var_t5_dn0 = assign36190_e50952_d_n0;
        var_t5_dn2 = assign36190_e50952_d_n2;
        var_t5_dn6 = assign36190_e50952_d_n6;
        var_t5_dn7 = assign36190_e50952_d_n7;
        var_t5_dn10 = assign36190_e50952_d_n10;
        var_t5_dn11 = assign36190_e50952_d_n11;
        var_t5_dn12 = assign36190_e50952_d_n12;
        var_t5_dn17 = assign36190_e50952_d_n17;

        let (assign36200_e50958, assign36200_e50958_d_n0, assign36200_e50958_d_n2, assign36200_e50958_d_n6, assign36200_e50958_d_n7, assign36200_e50958_d_n10, assign36200_e50958_d_n11, assign36200_e50958_d_n12, assign36200_e50958_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36200_e50956: f64 = (var_mu0__blk1191 * var_t5);
        (assign36200_e50956, ((var_mu0__blk1191_dn0 * var_t5) + (var_mu0__blk1191 * var_t5_dn0)), ((var_mu0__blk1191_dn2 * var_t5) + (var_mu0__blk1191 * var_t5_dn2)), ((var_mu0__blk1191_dn6 * var_t5) + (var_mu0__blk1191 * var_t5_dn6)), ((var_mu0__blk1191_dn7 * var_t5) + (var_mu0__blk1191 * var_t5_dn7)), ((var_mu0__blk1191_dn10 * var_t5) + (var_mu0__blk1191 * var_t5_dn10)), ((var_mu0__blk1191_dn11 * var_t5) + (var_mu0__blk1191 * var_t5_dn11)), ((var_mu0__blk1191_dn12 * var_t5) + (var_mu0__blk1191 * var_t5_dn12)), ((var_mu0__blk1191_dn17 * var_t5) + (var_mu0__blk1191 * var_t5_dn17)),)
    } else {
        (var_mu__blk1195, var_mu__blk1195_dn0, var_mu__blk1195_dn2, var_mu__blk1195_dn6, var_mu__blk1195_dn7, var_mu__blk1195_dn10, var_mu__blk1195_dn11, var_mu__blk1195_dn12, var_mu__blk1195_dn17,)
    }
};
        var_mu__blk1195 = assign36200_e50958;
        var_mu__blk1195_dn0 = assign36200_e50958_d_n0;
        var_mu__blk1195_dn2 = assign36200_e50958_d_n2;
        var_mu__blk1195_dn6 = assign36200_e50958_d_n6;
        var_mu__blk1195_dn7 = assign36200_e50958_d_n7;
        var_mu__blk1195_dn10 = assign36200_e50958_d_n10;
        var_mu__blk1195_dn11 = assign36200_e50958_d_n11;
        var_mu__blk1195_dn12 = assign36200_e50958_d_n12;
        var_mu__blk1195_dn17 = assign36200_e50958_d_n17;

        let (assign36210_e50964, assign36210_e50964_d_n0, assign36210_e50964_d_n2, assign36210_e50964_d_n6, assign36210_e50964_d_n7, assign36210_e50964_d_n10, assign36210_e50964_d_n11, assign36210_e50964_d_n12, assign36210_e50964_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36210_e50962: f64 = (1.6021918e-19 / var_ldrifte__blk1189);
        (assign36210_e50962, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign36210_e50964;
        var_t1_dn0 = assign36210_e50964_d_n0;
        var_t1_dn2 = assign36210_e50964_d_n2;
        var_t1_dn6 = assign36210_e50964_d_n6;
        var_t1_dn7 = assign36210_e50964_d_n7;
        var_t1_dn10 = assign36210_e50964_d_n10;
        var_t1_dn11 = assign36210_e50964_d_n11;
        var_t1_dn12 = assign36210_e50964_d_n12;
        var_t1_dn17 = assign36210_e50964_d_n17;

        let (assign36220_e50974, assign36220_e50974_d_n0, assign36220_e50974_d_n2, assign36220_e50974_d_n6, assign36220_e50974_d_n7, assign36220_e50974_d_n10, assign36220_e50974_d_n11, assign36220_e50974_d_n12, assign36220_e50974_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36220_e50968: f64 = (var_t1 * var_xov__blk1196);
        let assign36220_e50970: f64 = (assign36220_e50968 * var_mu__blk1195);
        let assign36220_e50972: f64 = (assign36220_e50970 * var_nover__blk1190);
        (assign36220_e50972, ((((var_t1_dn0 * var_xov__blk1196) * var_mu__blk1195) + (assign36220_e50968 * var_mu__blk1195_dn0)) * var_nover__blk1190), ((((var_t1_dn2 * var_xov__blk1196) * var_mu__blk1195) + (assign36220_e50968 * var_mu__blk1195_dn2)) * var_nover__blk1190), ((((var_t1_dn6 * var_xov__blk1196) * var_mu__blk1195) + (assign36220_e50968 * var_mu__blk1195_dn6)) * var_nover__blk1190), ((((var_t1_dn7 * var_xov__blk1196) * var_mu__blk1195) + (assign36220_e50968 * var_mu__blk1195_dn7)) * var_nover__blk1190), ((((var_t1_dn10 * var_xov__blk1196) * var_mu__blk1195) + (assign36220_e50968 * var_mu__blk1195_dn10)) * var_nover__blk1190), ((((var_t1_dn11 * var_xov__blk1196) * var_mu__blk1195) + (assign36220_e50968 * var_mu__blk1195_dn11)) * var_nover__blk1190), ((((var_t1_dn12 * var_xov__blk1196) * var_mu__blk1195) + (assign36220_e50968 * var_mu__blk1195_dn12)) * var_nover__blk1190), ((((var_t1_dn17 * var_xov__blk1196) * var_mu__blk1195) + (assign36220_e50968 * var_mu__blk1195_dn17)) * var_nover__blk1190),)
    } else {
        (var_gd__blk1197, var_gd__blk1197_dn0, var_gd__blk1197_dn2, var_gd__blk1197_dn6, var_gd__blk1197_dn7, var_gd__blk1197_dn10, var_gd__blk1197_dn11, var_gd__blk1197_dn12, var_gd__blk1197_dn17,)
    }
};
        var_gd__blk1197 = assign36220_e50974;
        var_gd__blk1197_dn0 = assign36220_e50974_d_n0;
        var_gd__blk1197_dn2 = assign36220_e50974_d_n2;
        var_gd__blk1197_dn6 = assign36220_e50974_d_n6;
        var_gd__blk1197_dn7 = assign36220_e50974_d_n7;
        var_gd__blk1197_dn10 = assign36220_e50974_d_n10;
        var_gd__blk1197_dn11 = assign36220_e50974_d_n11;
        var_gd__blk1197_dn12 = assign36220_e50974_d_n12;
        var_gd__blk1197_dn17 = assign36220_e50974_d_n17;

        let assign36230_e50977: f64 = if var_gd__blk1197 <= 0.0 { 1.0 } else { 0.0 };
        var_guard1205 = assign36230_e50977;

        let (assign36240_e50983, assign36240_e50983_d_n0, assign36240_e50983_d_n2, assign36240_e50983_d_n6, assign36240_e50983_d_n7, assign36240_e50983_d_n10, assign36240_e50983_d_n11, assign36240_e50983_d_n12, assign36240_e50983_d_n17,) = {
    if ((var_guard1179 != 0.0) && (var_guard1205 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gd__blk1197, var_gd__blk1197_dn0, var_gd__blk1197_dn2, var_gd__blk1197_dn6, var_gd__blk1197_dn7, var_gd__blk1197_dn10, var_gd__blk1197_dn11, var_gd__blk1197_dn12, var_gd__blk1197_dn17,)
    }
};
        var_gd__blk1197 = assign36240_e50983;
        var_gd__blk1197_dn0 = assign36240_e50983_d_n0;
        var_gd__blk1197_dn2 = assign36240_e50983_d_n2;
        var_gd__blk1197_dn6 = assign36240_e50983_d_n6;
        var_gd__blk1197_dn7 = assign36240_e50983_d_n7;
        var_gd__blk1197_dn10 = assign36240_e50983_d_n10;
        var_gd__blk1197_dn11 = assign36240_e50983_d_n11;
        var_gd__blk1197_dn12 = assign36240_e50983_d_n12;
        var_gd__blk1197_dn17 = assign36240_e50983_d_n17;

        let (assign36250_e50989, assign36250_e50989_d_n0, assign36250_e50989_d_n2, assign36250_e50989_d_n6, assign36250_e50989_d_n7, assign36250_e50989_d_n10, assign36250_e50989_d_n11, assign36250_e50989_d_n12, assign36250_e50989_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36250_e50987: f64 = (1.0 / var_gd__blk1197);
        (assign36250_e50987, (-(var_gd__blk1197_dn0 / (var_gd__blk1197 * var_gd__blk1197))), (-(var_gd__blk1197_dn2 / (var_gd__blk1197 * var_gd__blk1197))), (-(var_gd__blk1197_dn6 / (var_gd__blk1197 * var_gd__blk1197))), (-(var_gd__blk1197_dn7 / (var_gd__blk1197 * var_gd__blk1197))), (-(var_gd__blk1197_dn10 / (var_gd__blk1197 * var_gd__blk1197))), (-(var_gd__blk1197_dn11 / (var_gd__blk1197 * var_gd__blk1197))), (-(var_gd__blk1197_dn12 / (var_gd__blk1197 * var_gd__blk1197))), (-(var_gd__blk1197_dn17 / (var_gd__blk1197 * var_gd__blk1197))),)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign36250_e50989;
        var_rsd_dn0 = assign36250_e50989_d_n0;
        var_rsd_dn2 = assign36250_e50989_d_n2;
        var_rsd_dn6 = assign36250_e50989_d_n6;
        var_rsd_dn7 = assign36250_e50989_d_n7;
        var_rsd_dn10 = assign36250_e50989_d_n10;
        var_rsd_dn11 = assign36250_e50989_d_n11;
        var_rsd_dn12 = assign36250_e50989_d_n12;
        var_rsd_dn17 = assign36250_e50989_d_n17;

        let (assign36260_e50995, assign36260_e50995_d_n0, assign36260_e50995_d_n2, assign36260_e50995_d_n6, assign36260_e50995_d_n7, assign36260_e50995_d_n10, assign36260_e50995_d_n11, assign36260_e50995_d_n12, assign36260_e50995_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36260_e50993: f64 = (var_rsd / var_weff_nf__blk1198);
        (assign36260_e50993, (var_rsd_dn0 / var_weff_nf__blk1198), (var_rsd_dn2 / var_weff_nf__blk1198), (var_rsd_dn6 / var_weff_nf__blk1198), (var_rsd_dn7 / var_weff_nf__blk1198), (var_rsd_dn10 / var_weff_nf__blk1198), (var_rsd_dn11 / var_weff_nf__blk1198), (var_rsd_dn12 / var_weff_nf__blk1198), (var_rsd_dn17 / var_weff_nf__blk1198),)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign36260_e50995;
        var_rsd_dn0 = assign36260_e50995_d_n0;
        var_rsd_dn2 = assign36260_e50995_d_n2;
        var_rsd_dn6 = assign36260_e50995_d_n6;
        var_rsd_dn7 = assign36260_e50995_d_n7;
        var_rsd_dn10 = assign36260_e50995_d_n10;
        var_rsd_dn11 = assign36260_e50995_d_n11;
        var_rsd_dn12 = assign36260_e50995_d_n12;
        var_rsd_dn17 = assign36260_e50995_d_n17;

        let (assign36270_e51001, assign36270_e51001_d_n0, assign36270_e51001_d_n2, assign36270_e51001_d_n6, assign36270_e51001_d_n7, assign36270_e51001_d_n10, assign36270_e51001_d_n11, assign36270_e51001_d_n12, assign36270_e51001_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36270_e50999: f64 = (var_rsd + var_rsd0__blk1186);
        (assign36270_e50999, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign36270_e51001;
        var_rsd_dn0 = assign36270_e51001_d_n0;
        var_rsd_dn2 = assign36270_e51001_d_n2;
        var_rsd_dn6 = assign36270_e51001_d_n6;
        var_rsd_dn7 = assign36270_e51001_d_n7;
        var_rsd_dn10 = assign36270_e51001_d_n10;
        var_rsd_dn11 = assign36270_e51001_d_n11;
        var_rsd_dn12 = assign36270_e51001_d_n12;
        var_rsd_dn17 = assign36270_e51001_d_n17;

        let assign36290_e51019: f64 = if var_rsd < 0.0001 { 1.0 } else { 0.0 };
        var_guard1206 = assign36290_e51019;

        let (assign36300_e51025, assign36300_e51025_d_n0, assign36300_e51025_d_n2, assign36300_e51025_d_n6, assign36300_e51025_d_n7, assign36300_e51025_d_n10, assign36300_e51025_d_n11, assign36300_e51025_d_n12, assign36300_e51025_d_n17,) = {
    if ((var_guard1179 != 0.0) && (var_guard1206 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign36300_e51025;
        var_rsd_dn0 = assign36300_e51025_d_n0;
        var_rsd_dn2 = assign36300_e51025_d_n2;
        var_rsd_dn6 = assign36300_e51025_d_n6;
        var_rsd_dn7 = assign36300_e51025_d_n7;
        var_rsd_dn10 = assign36300_e51025_d_n10;
        var_rsd_dn11 = assign36300_e51025_d_n11;
        var_rsd_dn12 = assign36300_e51025_d_n12;
        var_rsd_dn17 = assign36300_e51025_d_n17;

        let (assign36310_e51031, assign36310_e51031_d_n0, assign36310_e51031_d_n2, assign36310_e51031_d_n6, assign36310_e51031_d_n7, assign36310_e51031_d_n10, assign36310_e51031_d_n11, assign36310_e51031_d_n12, assign36310_e51031_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36310_e51029: f64 = (var_rsd / var_mfactor);
        (assign36310_e51029, (var_rsd_dn0 / var_mfactor), (var_rsd_dn2 / var_mfactor), (var_rsd_dn6 / var_mfactor), (var_rsd_dn7 / var_mfactor), (var_rsd_dn10 / var_mfactor), (var_rsd_dn11 / var_mfactor), (var_rsd_dn12 / var_mfactor), (var_rsd_dn17 / var_mfactor),)
    } else {
        (var_rdde, var_rdde_dn0, var_rdde_dn2, var_rdde_dn6, var_rdde_dn7, var_rdde_dn10, var_rdde_dn11, var_rdde_dn12, var_rdde_dn17,)
    }
};
        var_rdde = assign36310_e51031;
        var_rdde_dn0 = assign36310_e51031_d_n0;
        var_rdde_dn2 = assign36310_e51031_d_n2;
        var_rdde_dn6 = assign36310_e51031_d_n6;
        var_rdde_dn7 = assign36310_e51031_d_n7;
        var_rdde_dn10 = assign36310_e51031_d_n10;
        var_rdde_dn11 = assign36310_e51031_d_n11;
        var_rdde_dn12 = assign36310_e51031_d_n12;
        var_rdde_dn17 = assign36310_e51031_d_n17;

        let assign36330_e51038: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1207 = assign36330_e51038;

        let assign36340_e51042: f64 = (1e-15 / 0.0001);
        let assign36340_e51043: f64 = if var_tau < assign36340_e51042 { 1.0 } else { 0.0 };
        var_guard1208 = assign36340_e51043;

        let (assign36350_e51053, assign36350_e51053_d_n0, assign36350_e51053_d_n2, assign36350_e51053_d_n6, assign36350_e51053_d_n7, assign36350_e51053_d_n10, assign36350_e51053_d_n11, assign36350_e51053_d_n12, assign36350_e51053_d_n17,) = {
    if (((var_guard1207 != 0.0) && (var_flg_nqs != 0.0)) && (var_guard1208 != 0.0)) {
        let assign36350_e51051: f64 = (1e-15 / 0.0001);
        (assign36350_e51051, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn6, var_tau_dn7, var_tau_dn10, var_tau_dn11, var_tau_dn12, var_tau_dn17,)
    }
};
        var_tau = assign36350_e51053;
        var_tau_dn0 = assign36350_e51053_d_n0;
        var_tau_dn2 = assign36350_e51053_d_n2;
        var_tau_dn6 = assign36350_e51053_d_n6;
        var_tau_dn7 = assign36350_e51053_d_n7;
        var_tau_dn10 = assign36350_e51053_d_n10;
        var_tau_dn11 = assign36350_e51053_d_n11;
        var_tau_dn12 = assign36350_e51053_d_n12;
        var_tau_dn17 = assign36350_e51053_d_n17;

        let assign36360_e51057: f64 = (1e-15 / 0.0001);
        let assign36360_e51058: f64 = if var_taub < assign36360_e51057 { 1.0 } else { 0.0 };
        var_guard1209 = assign36360_e51058;

        let (assign36370_e51068, assign36370_e51068_d_n0, assign36370_e51068_d_n2, assign36370_e51068_d_n6, assign36370_e51068_d_n7, assign36370_e51068_d_n10, assign36370_e51068_d_n11, assign36370_e51068_d_n12, assign36370_e51068_d_n17,) = {
    if (((var_guard1207 != 0.0) && (var_flg_nqs != 0.0)) && (var_guard1209 != 0.0)) {
        let assign36370_e51066: f64 = (1e-15 / 0.0001);
        (assign36370_e51066, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taub, var_taub_dn0, var_taub_dn2, var_taub_dn6, var_taub_dn7, var_taub_dn10, var_taub_dn11, var_taub_dn12, var_taub_dn17,)
    }
};
        var_taub = assign36370_e51068;
        var_taub_dn0 = assign36370_e51068_d_n0;
        var_taub_dn2 = assign36370_e51068_d_n2;
        var_taub_dn6 = assign36370_e51068_d_n6;
        var_taub_dn7 = assign36370_e51068_d_n7;
        var_taub_dn10 = assign36370_e51068_d_n10;
        var_taub_dn11 = assign36370_e51068_d_n11;
        var_taub_dn12 = assign36370_e51068_d_n12;
        var_taub_dn17 = assign36370_e51068_d_n17;

        let (assign36380_e51081, assign36380_e51081_d_n0, assign36380_e51081_d_n2, assign36380_e51081_d_n6, assign36380_e51081_d_n7, assign36380_e51081_d_n10, assign36380_e51081_d_n11, assign36380_e51081_d_n12, assign36380_e51081_d_n17,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs != 0.0)) {
        let (assign36380_e51079, assign36380_e51079_d_n0, assign36380_e51079_d_n2, assign36380_e51079_d_n6, assign36380_e51079_d_n7, assign36380_e51079_d_n10, assign36380_e51079_d_n11, assign36380_e51079_d_n12, assign36380_e51079_d_n17,) = {
            if (var_mode == 1.0) {
                (var_xd, var_xd_dn0, var_xd_dn2, var_xd_dn6, var_xd_dn7, var_xd_dn10, var_xd_dn11, var_xd_dn12, var_xd_dn17,)
            } else {
                let assign36380_e51078: f64 = (1.0 - var_xd);
                (assign36380_e51078, (-var_xd_dn0), (-var_xd_dn2), (-var_xd_dn6), (-var_xd_dn7), (-var_xd_dn10), (-var_xd_dn11), (-var_xd_dn12), (-var_xd_dn17),)
            }
        };
        (assign36380_e51079, assign36380_e51079_d_n0, assign36380_e51079_d_n2, assign36380_e51079_d_n6, assign36380_e51079_d_n7, assign36380_e51079_d_n10, assign36380_e51079_d_n11, assign36380_e51079_d_n12, assign36380_e51079_d_n17,)
    } else {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn10, var_qdrat_dn11, var_qdrat_dn12, var_qdrat_dn17,)
    }
};
        var_qdrat = assign36380_e51081;
        var_qdrat_dn0 = assign36380_e51081_d_n0;
        var_qdrat_dn2 = assign36380_e51081_d_n2;
        var_qdrat_dn6 = assign36380_e51081_d_n6;
        var_qdrat_dn7 = assign36380_e51081_d_n7;
        var_qdrat_dn10 = assign36380_e51081_d_n10;
        var_qdrat_dn11 = assign36380_e51081_d_n11;
        var_qdrat_dn12 = assign36380_e51081_d_n12;
        var_qdrat_dn17 = assign36380_e51081_d_n17;

        let (assign36390_e51091, assign36390_e51091_d_n0, assign36390_e51091_d_n2, assign36390_e51091_d_n6, assign36390_e51091_d_n7, assign36390_e51091_d_n10, assign36390_e51091_d_n11, assign36390_e51091_d_n12, assign36390_e51091_d_n17, assign36390_e51091_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36390_e51087: f64 = (var_qi_nqs - var_qi_qs);
        let assign36390_e51089: f64 = (assign36390_e51087 / var_tau);
        (assign36390_e51089, ((((-var_qi_qs_dn0) * var_tau) - (assign36390_e51087 * var_tau_dn0)) / (var_tau * var_tau)), ((((-var_qi_qs_dn2) * var_tau) - (assign36390_e51087 * var_tau_dn2)) / (var_tau * var_tau)), ((((-var_qi_qs_dn6) * var_tau) - (assign36390_e51087 * var_tau_dn6)) / (var_tau * var_tau)), ((((-var_qi_qs_dn7) * var_tau) - (assign36390_e51087 * var_tau_dn7)) / (var_tau * var_tau)), ((((-var_qi_qs_dn10) * var_tau) - (assign36390_e51087 * var_tau_dn10)) / (var_tau * var_tau)), ((((-var_qi_qs_dn11) * var_tau) - (assign36390_e51087 * var_tau_dn11)) / (var_tau * var_tau)), ((((-var_qi_qs_dn12) * var_tau) - (assign36390_e51087 * var_tau_dn12)) / (var_tau * var_tau)), ((((-var_qi_qs_dn17) * var_tau) - (assign36390_e51087 * var_tau_dn17)) / (var_tau * var_tau)), (var_qi_nqs_dn18 / var_tau),)
    } else {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn6, var_iqi_nqs_dn7, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12, var_iqi_nqs_dn17, var_iqi_nqs_dn18,)
    }
};
        var_iqi_nqs = assign36390_e51091;
        var_iqi_nqs_dn0 = assign36390_e51091_d_n0;
        var_iqi_nqs_dn2 = assign36390_e51091_d_n2;
        var_iqi_nqs_dn6 = assign36390_e51091_d_n6;
        var_iqi_nqs_dn7 = assign36390_e51091_d_n7;
        var_iqi_nqs_dn10 = assign36390_e51091_d_n10;
        var_iqi_nqs_dn11 = assign36390_e51091_d_n11;
        var_iqi_nqs_dn12 = assign36390_e51091_d_n12;
        var_iqi_nqs_dn17 = assign36390_e51091_d_n17;
        var_iqi_nqs_dn18 = assign36390_e51091_d_n18;

        *var_gd__blk1197_slot = var_gd__blk1197;
        *var_gd__blk1197_dn0_slot = var_gd__blk1197_dn0;
        *var_gd__blk1197_dn10_slot = var_gd__blk1197_dn10;
        *var_gd__blk1197_dn11_slot = var_gd__blk1197_dn11;
        *var_gd__blk1197_dn12_slot = var_gd__blk1197_dn12;
        *var_gd__blk1197_dn17_slot = var_gd__blk1197_dn17;
        *var_gd__blk1197_dn2_slot = var_gd__blk1197_dn2;
        *var_gd__blk1197_dn6_slot = var_gd__blk1197_dn6;
        *var_gd__blk1197_dn7_slot = var_gd__blk1197_dn7;
        *var_guard1200_slot = var_guard1200;
        *var_guard1201_slot = var_guard1201;
        *var_guard1202_slot = var_guard1202;
        *var_guard1203_slot = var_guard1203;
        *var_guard1204_slot = var_guard1204;
        *var_guard1205_slot = var_guard1205;
        *var_guard1206_slot = var_guard1206;
        *var_guard1207_slot = var_guard1207;
        *var_guard1208_slot = var_guard1208;
        *var_guard1209_slot = var_guard1209;
        *var_iqi_nqs_slot = var_iqi_nqs;
        *var_iqi_nqs_dn0_slot = var_iqi_nqs_dn0;
        *var_iqi_nqs_dn10_slot = var_iqi_nqs_dn10;
        *var_iqi_nqs_dn11_slot = var_iqi_nqs_dn11;
        *var_iqi_nqs_dn12_slot = var_iqi_nqs_dn12;
        *var_iqi_nqs_dn17_slot = var_iqi_nqs_dn17;
        *var_iqi_nqs_dn18_slot = var_iqi_nqs_dn18;
        *var_iqi_nqs_dn2_slot = var_iqi_nqs_dn2;
        *var_iqi_nqs_dn6_slot = var_iqi_nqs_dn6;
        *var_iqi_nqs_dn7_slot = var_iqi_nqs_dn7;
        *var_mu__blk1195_slot = var_mu__blk1195;
        *var_mu__blk1195_dn0_slot = var_mu__blk1195_dn0;
        *var_mu__blk1195_dn10_slot = var_mu__blk1195_dn10;
        *var_mu__blk1195_dn11_slot = var_mu__blk1195_dn11;
        *var_mu__blk1195_dn12_slot = var_mu__blk1195_dn12;
        *var_mu__blk1195_dn17_slot = var_mu__blk1195_dn17;
        *var_mu__blk1195_dn2_slot = var_mu__blk1195_dn2;
        *var_mu__blk1195_dn6_slot = var_mu__blk1195_dn6;
        *var_mu__blk1195_dn7_slot = var_mu__blk1195_dn7;
        *var_qdrat_slot = var_qdrat;
        *var_qdrat_dn0_slot = var_qdrat_dn0;
        *var_qdrat_dn10_slot = var_qdrat_dn10;
        *var_qdrat_dn11_slot = var_qdrat_dn11;
        *var_qdrat_dn12_slot = var_qdrat_dn12;
        *var_qdrat_dn17_slot = var_qdrat_dn17;
        *var_qdrat_dn2_slot = var_qdrat_dn2;
        *var_qdrat_dn6_slot = var_qdrat_dn6;
        *var_qdrat_dn7_slot = var_qdrat_dn7;
        *var_rdde_slot = var_rdde;
        *var_rdde_dn0_slot = var_rdde_dn0;
        *var_rdde_dn10_slot = var_rdde_dn10;
        *var_rdde_dn11_slot = var_rdde_dn11;
        *var_rdde_dn12_slot = var_rdde_dn12;
        *var_rdde_dn17_slot = var_rdde_dn17;
        *var_rdde_dn2_slot = var_rdde_dn2;
        *var_rdde_dn6_slot = var_rdde_dn6;
        *var_rdde_dn7_slot = var_rdde_dn7;
        *var_rsd_slot = var_rsd;
        *var_rsd_dn0_slot = var_rsd_dn0;
        *var_rsd_dn10_slot = var_rsd_dn10;
        *var_rsd_dn11_slot = var_rsd_dn11;
        *var_rsd_dn12_slot = var_rsd_dn12;
        *var_rsd_dn17_slot = var_rsd_dn17;
        *var_rsd_dn2_slot = var_rsd_dn2;
        *var_rsd_dn6_slot = var_rsd_dn6;
        *var_rsd_dn7_slot = var_rsd_dn7;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn17_slot = var_t4_dn17;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn17_slot = var_t5_dn17;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn17_slot = var_t6_dn17;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_tau_slot = var_tau;
        *var_tau_dn0_slot = var_tau_dn0;
        *var_tau_dn10_slot = var_tau_dn10;
        *var_tau_dn11_slot = var_tau_dn11;
        *var_tau_dn12_slot = var_tau_dn12;
        *var_tau_dn17_slot = var_tau_dn17;
        *var_tau_dn2_slot = var_tau_dn2;
        *var_tau_dn6_slot = var_tau_dn6;
        *var_tau_dn7_slot = var_tau_dn7;
        *var_taub_slot = var_taub;
        *var_taub_dn0_slot = var_taub_dn0;
        *var_taub_dn10_slot = var_taub_dn10;
        *var_taub_dn11_slot = var_taub_dn11;
        *var_taub_dn12_slot = var_taub_dn12;
        *var_taub_dn17_slot = var_taub_dn17;
        *var_taub_dn2_slot = var_taub_dn2;
        *var_taub_dn6_slot = var_taub_dn6;
        *var_taub_dn7_slot = var_taub_dn7;
        *var_vdri__blk1194_slot = var_vdri__blk1194;
        *var_vdri__blk1194_dn0_slot = var_vdri__blk1194_dn0;
        *var_vdri__blk1194_dn10_slot = var_vdri__blk1194_dn10;
        *var_vdri__blk1194_dn11_slot = var_vdri__blk1194_dn11;
        *var_vdri__blk1194_dn12_slot = var_vdri__blk1194_dn12;
        *var_vdri__blk1194_dn17_slot = var_vdri__blk1194_dn17;
        *var_vdri__blk1194_dn2_slot = var_vdri__blk1194_dn2;
        *var_vdri__blk1194_dn6_slot = var_vdri__blk1194_dn6;
        *var_vdri__blk1194_dn7_slot = var_vdri__blk1194_dn7;
    }

    pub(super) fn stamp_transient_block_126(
        var_flg_nqs: f64,
        var_guard1207: f64,
        var_mode: f64,
        var_q_bt_ge: f64,
        var_q_bt_ge_dn0: f64,
        var_q_bt_ge_dn10: f64,
        var_q_bt_ge_dn11: f64,
        var_q_bt_ge_dn12: f64,
        var_q_bt_ge_dn17: f64,
        var_q_bt_ge_dn2: f64,
        var_q_bt_ge_dn6: f64,
        var_q_bt_ge_dn7: f64,
        var_q_bt_se: f64,
        var_q_bt_se_dn0: f64,
        var_q_bt_se_dn10: f64,
        var_q_bt_se_dn11: f64,
        var_q_bt_se_dn12: f64,
        var_q_bt_se_dn17: f64,
        var_q_bt_se_dn2: f64,
        var_q_bt_se_dn6: f64,
        var_q_bt_se_dn7: f64,
        var_qb_qs: f64,
        var_qb_qs_dn0: f64,
        var_qb_qs_dn10: f64,
        var_qb_qs_dn11: f64,
        var_qb_qs_dn12: f64,
        var_qb_qs_dn13: f64,
        var_qb_qs_dn15: f64,
        var_qb_qs_dn16: f64,
        var_qb_qs_dn17: f64,
        var_qb_qs_dn18: f64,
        var_qb_qs_dn2: f64,
        var_qb_qs_dn6: f64,
        var_qb_qs_dn7: f64,
        var_qd_qs: f64,
        var_qd_qs_dn0: f64,
        var_qd_qs_dn10: f64,
        var_qd_qs_dn11: f64,
        var_qd_qs_dn12: f64,
        var_qd_qs_dn13: f64,
        var_qd_qs_dn15: f64,
        var_qd_qs_dn16: f64,
        var_qd_qs_dn17: f64,
        var_qd_qs_dn18: f64,
        var_qd_qs_dn2: f64,
        var_qd_qs_dn6: f64,
        var_qd_qs_dn7: f64,
        var_qdrat: f64,
        var_qdrat_dn0: f64,
        var_qdrat_dn10: f64,
        var_qdrat_dn11: f64,
        var_qdrat_dn12: f64,
        var_qdrat_dn17: f64,
        var_qdrat_dn2: f64,
        var_qdrat_dn6: f64,
        var_qdrat_dn7: f64,
        var_qi_nqs: f64,
        var_qi_nqs_dn18: f64,
        var_qs_qs: f64,
        var_qs_qs_dn0: f64,
        var_qs_qs_dn10: f64,
        var_qs_qs_dn11: f64,
        var_qs_qs_dn12: f64,
        var_qs_qs_dn13: f64,
        var_qs_qs_dn15: f64,
        var_qs_qs_dn16: f64,
        var_qs_qs_dn17: f64,
        var_qs_qs_dn18: f64,
        var_qs_qs_dn2: f64,
        var_qs_qs_dn6: f64,
        var_qs_qs_dn7: f64,
        var_rdde: f64,
        var_rdde_dn0: f64,
        var_rdde_dn10: f64,
        var_rdde_dn11: f64,
        var_rdde_dn12: f64,
        var_rdde_dn17: f64,
        var_rdde_dn2: f64,
        var_rdde_dn6: f64,
        var_rdde_dn7: f64,
        var_rsde: f64,
        var_rsde_dn0: f64,
        var_rsde_dn10: f64,
        var_rsde_dn11: f64,
        var_rsde_dn12: f64,
        var_rsde_dn17: f64,
        var_rsde_dn2: f64,
        var_rsde_dn6: f64,
        var_rsde_dn7: f64,
        var_guard1210_slot: &mut f64,
        var_guard1211_slot: &mut f64,
        var_guard1212_slot: &mut f64,
        var_iqb_nqs_slot: &mut f64,
        var_iqb_nqs_dn0_slot: &mut f64,
        var_iqb_nqs_dn10_slot: &mut f64,
        var_iqb_nqs_dn11_slot: &mut f64,
        var_iqb_nqs_dn12_slot: &mut f64,
        var_iqb_nqs_dn13_slot: &mut f64,
        var_iqb_nqs_dn15_slot: &mut f64,
        var_iqb_nqs_dn16_slot: &mut f64,
        var_iqb_nqs_dn17_slot: &mut f64,
        var_iqb_nqs_dn18_slot: &mut f64,
        var_iqb_nqs_dn2_slot: &mut f64,
        var_iqb_nqs_dn6_slot: &mut f64,
        var_iqb_nqs_dn7_slot: &mut f64,
        var_iqd_nqs_slot: &mut f64,
        var_iqd_nqs_dn0_slot: &mut f64,
        var_iqd_nqs_dn10_slot: &mut f64,
        var_iqd_nqs_dn11_slot: &mut f64,
        var_iqd_nqs_dn12_slot: &mut f64,
        var_iqd_nqs_dn13_slot: &mut f64,
        var_iqd_nqs_dn15_slot: &mut f64,
        var_iqd_nqs_dn16_slot: &mut f64,
        var_iqd_nqs_dn17_slot: &mut f64,
        var_iqd_nqs_dn18_slot: &mut f64,
        var_iqd_nqs_dn2_slot: &mut f64,
        var_iqd_nqs_dn6_slot: &mut f64,
        var_iqd_nqs_dn7_slot: &mut f64,
        var_iqi_nqs_slot: &mut f64,
        var_iqi_nqs_dn0_slot: &mut f64,
        var_iqi_nqs_dn10_slot: &mut f64,
        var_iqi_nqs_dn11_slot: &mut f64,
        var_iqi_nqs_dn12_slot: &mut f64,
        var_iqi_nqs_dn17_slot: &mut f64,
        var_iqi_nqs_dn18_slot: &mut f64,
        var_iqi_nqs_dn2_slot: &mut f64,
        var_iqi_nqs_dn6_slot: &mut f64,
        var_iqi_nqs_dn7_slot: &mut f64,
        var_iqs_nqs_slot: &mut f64,
        var_iqs_nqs_dn0_slot: &mut f64,
        var_iqs_nqs_dn10_slot: &mut f64,
        var_iqs_nqs_dn11_slot: &mut f64,
        var_iqs_nqs_dn12_slot: &mut f64,
        var_iqs_nqs_dn13_slot: &mut f64,
        var_iqs_nqs_dn15_slot: &mut f64,
        var_iqs_nqs_dn16_slot: &mut f64,
        var_iqs_nqs_dn17_slot: &mut f64,
        var_iqs_nqs_dn18_slot: &mut f64,
        var_iqs_nqs_dn2_slot: &mut f64,
        var_iqs_nqs_dn6_slot: &mut f64,
        var_iqs_nqs_dn7_slot: &mut f64,
        var_qb_nqs_slot: &mut f64,
        var_qb_nqs_dn13_slot: &mut f64,
        var_qd_nqs_slot: &mut f64,
        var_qd_nqs_dn0_slot: &mut f64,
        var_qd_nqs_dn10_slot: &mut f64,
        var_qd_nqs_dn11_slot: &mut f64,
        var_qd_nqs_dn12_slot: &mut f64,
        var_qd_nqs_dn15_slot: &mut f64,
        var_qd_nqs_dn17_slot: &mut f64,
        var_qd_nqs_dn18_slot: &mut f64,
        var_qd_nqs_dn2_slot: &mut f64,
        var_qd_nqs_dn6_slot: &mut f64,
        var_qd_nqs_dn7_slot: &mut f64,
        var_qg_nqs_slot: &mut f64,
        var_qg_nqs_dn0_slot: &mut f64,
        var_qg_nqs_dn10_slot: &mut f64,
        var_qg_nqs_dn11_slot: &mut f64,
        var_qg_nqs_dn12_slot: &mut f64,
        var_qg_nqs_dn13_slot: &mut f64,
        var_qg_nqs_dn15_slot: &mut f64,
        var_qg_nqs_dn16_slot: &mut f64,
        var_qg_nqs_dn17_slot: &mut f64,
        var_qg_nqs_dn18_slot: &mut f64,
        var_qg_nqs_dn2_slot: &mut f64,
        var_qg_nqs_dn6_slot: &mut f64,
        var_qg_nqs_dn7_slot: &mut f64,
        var_qs_nqs_slot: &mut f64,
        var_qs_nqs_dn0_slot: &mut f64,
        var_qs_nqs_dn10_slot: &mut f64,
        var_qs_nqs_dn11_slot: &mut f64,
        var_qs_nqs_dn12_slot: &mut f64,
        var_qs_nqs_dn16_slot: &mut f64,
        var_qs_nqs_dn17_slot: &mut f64,
        var_qs_nqs_dn18_slot: &mut f64,
        var_qs_nqs_dn2_slot: &mut f64,
        var_qs_nqs_dn6_slot: &mut f64,
        var_qs_nqs_dn7_slot: &mut f64,
        var_rdd_slot: &mut f64,
        var_rdd_dn0_slot: &mut f64,
        var_rdd_dn10_slot: &mut f64,
        var_rdd_dn11_slot: &mut f64,
        var_rdd_dn12_slot: &mut f64,
        var_rdd_dn17_slot: &mut f64,
        var_rdd_dn2_slot: &mut f64,
        var_rdd_dn6_slot: &mut f64,
        var_rdd_dn7_slot: &mut f64,
        var_rsd_slot: &mut f64,
        var_rsd_dn0_slot: &mut f64,
        var_rsd_dn10_slot: &mut f64,
        var_rsd_dn11_slot: &mut f64,
        var_rsd_dn12_slot: &mut f64,
        var_rsd_dn17_slot: &mut f64,
        var_rsd_dn2_slot: &mut f64,
        var_rsd_dn6_slot: &mut f64,
        var_rsd_dn7_slot: &mut f64,
        var_tau_slot: &mut f64,
        var_tau_dn0_slot: &mut f64,
        var_tau_dn10_slot: &mut f64,
        var_tau_dn11_slot: &mut f64,
        var_tau_dn12_slot: &mut f64,
        var_tau_dn17_slot: &mut f64,
        var_tau_dn2_slot: &mut f64,
        var_tau_dn6_slot: &mut f64,
        var_tau_dn7_slot: &mut f64,
        var_taub_slot: &mut f64,
        var_taub_dn0_slot: &mut f64,
        var_taub_dn10_slot: &mut f64,
        var_taub_dn11_slot: &mut f64,
        var_taub_dn12_slot: &mut f64,
        var_taub_dn17_slot: &mut f64,
        var_taub_dn2_slot: &mut f64,
        var_taub_dn6_slot: &mut f64,
        var_taub_dn7_slot: &mut f64,
    ) {
        let mut var_guard1210: f64 = *var_guard1210_slot;
        let mut var_guard1211: f64 = *var_guard1211_slot;
        let mut var_guard1212: f64 = *var_guard1212_slot;
        let mut var_iqb_nqs: f64 = *var_iqb_nqs_slot;
        let mut var_iqb_nqs_dn0: f64 = *var_iqb_nqs_dn0_slot;
        let mut var_iqb_nqs_dn10: f64 = *var_iqb_nqs_dn10_slot;
        let mut var_iqb_nqs_dn11: f64 = *var_iqb_nqs_dn11_slot;
        let mut var_iqb_nqs_dn12: f64 = *var_iqb_nqs_dn12_slot;
        let mut var_iqb_nqs_dn13: f64 = *var_iqb_nqs_dn13_slot;
        let mut var_iqb_nqs_dn15: f64 = *var_iqb_nqs_dn15_slot;
        let mut var_iqb_nqs_dn16: f64 = *var_iqb_nqs_dn16_slot;
        let mut var_iqb_nqs_dn17: f64 = *var_iqb_nqs_dn17_slot;
        let mut var_iqb_nqs_dn18: f64 = *var_iqb_nqs_dn18_slot;
        let mut var_iqb_nqs_dn2: f64 = *var_iqb_nqs_dn2_slot;
        let mut var_iqb_nqs_dn6: f64 = *var_iqb_nqs_dn6_slot;
        let mut var_iqb_nqs_dn7: f64 = *var_iqb_nqs_dn7_slot;
        let mut var_iqd_nqs: f64 = *var_iqd_nqs_slot;
        let mut var_iqd_nqs_dn0: f64 = *var_iqd_nqs_dn0_slot;
        let mut var_iqd_nqs_dn10: f64 = *var_iqd_nqs_dn10_slot;
        let mut var_iqd_nqs_dn11: f64 = *var_iqd_nqs_dn11_slot;
        let mut var_iqd_nqs_dn12: f64 = *var_iqd_nqs_dn12_slot;
        let mut var_iqd_nqs_dn13: f64 = *var_iqd_nqs_dn13_slot;
        let mut var_iqd_nqs_dn15: f64 = *var_iqd_nqs_dn15_slot;
        let mut var_iqd_nqs_dn16: f64 = *var_iqd_nqs_dn16_slot;
        let mut var_iqd_nqs_dn17: f64 = *var_iqd_nqs_dn17_slot;
        let mut var_iqd_nqs_dn18: f64 = *var_iqd_nqs_dn18_slot;
        let mut var_iqd_nqs_dn2: f64 = *var_iqd_nqs_dn2_slot;
        let mut var_iqd_nqs_dn6: f64 = *var_iqd_nqs_dn6_slot;
        let mut var_iqd_nqs_dn7: f64 = *var_iqd_nqs_dn7_slot;
        let mut var_iqi_nqs: f64 = *var_iqi_nqs_slot;
        let mut var_iqi_nqs_dn0: f64 = *var_iqi_nqs_dn0_slot;
        let mut var_iqi_nqs_dn10: f64 = *var_iqi_nqs_dn10_slot;
        let mut var_iqi_nqs_dn11: f64 = *var_iqi_nqs_dn11_slot;
        let mut var_iqi_nqs_dn12: f64 = *var_iqi_nqs_dn12_slot;
        let mut var_iqi_nqs_dn17: f64 = *var_iqi_nqs_dn17_slot;
        let mut var_iqi_nqs_dn18: f64 = *var_iqi_nqs_dn18_slot;
        let mut var_iqi_nqs_dn2: f64 = *var_iqi_nqs_dn2_slot;
        let mut var_iqi_nqs_dn6: f64 = *var_iqi_nqs_dn6_slot;
        let mut var_iqi_nqs_dn7: f64 = *var_iqi_nqs_dn7_slot;
        let mut var_iqs_nqs: f64 = *var_iqs_nqs_slot;
        let mut var_iqs_nqs_dn0: f64 = *var_iqs_nqs_dn0_slot;
        let mut var_iqs_nqs_dn10: f64 = *var_iqs_nqs_dn10_slot;
        let mut var_iqs_nqs_dn11: f64 = *var_iqs_nqs_dn11_slot;
        let mut var_iqs_nqs_dn12: f64 = *var_iqs_nqs_dn12_slot;
        let mut var_iqs_nqs_dn13: f64 = *var_iqs_nqs_dn13_slot;
        let mut var_iqs_nqs_dn15: f64 = *var_iqs_nqs_dn15_slot;
        let mut var_iqs_nqs_dn16: f64 = *var_iqs_nqs_dn16_slot;
        let mut var_iqs_nqs_dn17: f64 = *var_iqs_nqs_dn17_slot;
        let mut var_iqs_nqs_dn18: f64 = *var_iqs_nqs_dn18_slot;
        let mut var_iqs_nqs_dn2: f64 = *var_iqs_nqs_dn2_slot;
        let mut var_iqs_nqs_dn6: f64 = *var_iqs_nqs_dn6_slot;
        let mut var_iqs_nqs_dn7: f64 = *var_iqs_nqs_dn7_slot;
        let mut var_qb_nqs: f64 = *var_qb_nqs_slot;
        let mut var_qb_nqs_dn13: f64 = *var_qb_nqs_dn13_slot;
        let mut var_qd_nqs: f64 = *var_qd_nqs_slot;
        let mut var_qd_nqs_dn0: f64 = *var_qd_nqs_dn0_slot;
        let mut var_qd_nqs_dn10: f64 = *var_qd_nqs_dn10_slot;
        let mut var_qd_nqs_dn11: f64 = *var_qd_nqs_dn11_slot;
        let mut var_qd_nqs_dn12: f64 = *var_qd_nqs_dn12_slot;
        let mut var_qd_nqs_dn15: f64 = *var_qd_nqs_dn15_slot;
        let mut var_qd_nqs_dn17: f64 = *var_qd_nqs_dn17_slot;
        let mut var_qd_nqs_dn18: f64 = *var_qd_nqs_dn18_slot;
        let mut var_qd_nqs_dn2: f64 = *var_qd_nqs_dn2_slot;
        let mut var_qd_nqs_dn6: f64 = *var_qd_nqs_dn6_slot;
        let mut var_qd_nqs_dn7: f64 = *var_qd_nqs_dn7_slot;
        let mut var_qg_nqs: f64 = *var_qg_nqs_slot;
        let mut var_qg_nqs_dn0: f64 = *var_qg_nqs_dn0_slot;
        let mut var_qg_nqs_dn10: f64 = *var_qg_nqs_dn10_slot;
        let mut var_qg_nqs_dn11: f64 = *var_qg_nqs_dn11_slot;
        let mut var_qg_nqs_dn12: f64 = *var_qg_nqs_dn12_slot;
        let mut var_qg_nqs_dn13: f64 = *var_qg_nqs_dn13_slot;
        let mut var_qg_nqs_dn15: f64 = *var_qg_nqs_dn15_slot;
        let mut var_qg_nqs_dn16: f64 = *var_qg_nqs_dn16_slot;
        let mut var_qg_nqs_dn17: f64 = *var_qg_nqs_dn17_slot;
        let mut var_qg_nqs_dn18: f64 = *var_qg_nqs_dn18_slot;
        let mut var_qg_nqs_dn2: f64 = *var_qg_nqs_dn2_slot;
        let mut var_qg_nqs_dn6: f64 = *var_qg_nqs_dn6_slot;
        let mut var_qg_nqs_dn7: f64 = *var_qg_nqs_dn7_slot;
        let mut var_qs_nqs: f64 = *var_qs_nqs_slot;
        let mut var_qs_nqs_dn0: f64 = *var_qs_nqs_dn0_slot;
        let mut var_qs_nqs_dn10: f64 = *var_qs_nqs_dn10_slot;
        let mut var_qs_nqs_dn11: f64 = *var_qs_nqs_dn11_slot;
        let mut var_qs_nqs_dn12: f64 = *var_qs_nqs_dn12_slot;
        let mut var_qs_nqs_dn16: f64 = *var_qs_nqs_dn16_slot;
        let mut var_qs_nqs_dn17: f64 = *var_qs_nqs_dn17_slot;
        let mut var_qs_nqs_dn18: f64 = *var_qs_nqs_dn18_slot;
        let mut var_qs_nqs_dn2: f64 = *var_qs_nqs_dn2_slot;
        let mut var_qs_nqs_dn6: f64 = *var_qs_nqs_dn6_slot;
        let mut var_qs_nqs_dn7: f64 = *var_qs_nqs_dn7_slot;
        let mut var_rdd: f64 = *var_rdd_slot;
        let mut var_rdd_dn0: f64 = *var_rdd_dn0_slot;
        let mut var_rdd_dn10: f64 = *var_rdd_dn10_slot;
        let mut var_rdd_dn11: f64 = *var_rdd_dn11_slot;
        let mut var_rdd_dn12: f64 = *var_rdd_dn12_slot;
        let mut var_rdd_dn17: f64 = *var_rdd_dn17_slot;
        let mut var_rdd_dn2: f64 = *var_rdd_dn2_slot;
        let mut var_rdd_dn6: f64 = *var_rdd_dn6_slot;
        let mut var_rdd_dn7: f64 = *var_rdd_dn7_slot;
        let mut var_rsd: f64 = *var_rsd_slot;
        let mut var_rsd_dn0: f64 = *var_rsd_dn0_slot;
        let mut var_rsd_dn10: f64 = *var_rsd_dn10_slot;
        let mut var_rsd_dn11: f64 = *var_rsd_dn11_slot;
        let mut var_rsd_dn12: f64 = *var_rsd_dn12_slot;
        let mut var_rsd_dn17: f64 = *var_rsd_dn17_slot;
        let mut var_rsd_dn2: f64 = *var_rsd_dn2_slot;
        let mut var_rsd_dn6: f64 = *var_rsd_dn6_slot;
        let mut var_rsd_dn7: f64 = *var_rsd_dn7_slot;
        let mut var_tau: f64 = *var_tau_slot;
        let mut var_tau_dn0: f64 = *var_tau_dn0_slot;
        let mut var_tau_dn10: f64 = *var_tau_dn10_slot;
        let mut var_tau_dn11: f64 = *var_tau_dn11_slot;
        let mut var_tau_dn12: f64 = *var_tau_dn12_slot;
        let mut var_tau_dn17: f64 = *var_tau_dn17_slot;
        let mut var_tau_dn2: f64 = *var_tau_dn2_slot;
        let mut var_tau_dn6: f64 = *var_tau_dn6_slot;
        let mut var_tau_dn7: f64 = *var_tau_dn7_slot;
        let mut var_taub: f64 = *var_taub_slot;
        let mut var_taub_dn0: f64 = *var_taub_dn0_slot;
        let mut var_taub_dn10: f64 = *var_taub_dn10_slot;
        let mut var_taub_dn11: f64 = *var_taub_dn11_slot;
        let mut var_taub_dn12: f64 = *var_taub_dn12_slot;
        let mut var_taub_dn17: f64 = *var_taub_dn17_slot;
        let mut var_taub_dn2: f64 = *var_taub_dn2_slot;
        let mut var_taub_dn6: f64 = *var_taub_dn6_slot;
        let mut var_taub_dn7: f64 = *var_taub_dn7_slot;

        let (assign36400_e51101, assign36400_e51101_d_n0, assign36400_e51101_d_n2, assign36400_e51101_d_n6, assign36400_e51101_d_n7, assign36400_e51101_d_n10, assign36400_e51101_d_n11, assign36400_e51101_d_n12, assign36400_e51101_d_n13, assign36400_e51101_d_n15, assign36400_e51101_d_n16, assign36400_e51101_d_n17, assign36400_e51101_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36400_e51097: f64 = (var_qb_nqs - var_qb_qs);
        let assign36400_e51099: f64 = (assign36400_e51097 / var_taub);
        (assign36400_e51099, ((((-var_qb_qs_dn0) * var_taub) - (assign36400_e51097 * var_taub_dn0)) / (var_taub * var_taub)), ((((-var_qb_qs_dn2) * var_taub) - (assign36400_e51097 * var_taub_dn2)) / (var_taub * var_taub)), ((((-var_qb_qs_dn6) * var_taub) - (assign36400_e51097 * var_taub_dn6)) / (var_taub * var_taub)), ((((-var_qb_qs_dn7) * var_taub) - (assign36400_e51097 * var_taub_dn7)) / (var_taub * var_taub)), ((((-var_qb_qs_dn10) * var_taub) - (assign36400_e51097 * var_taub_dn10)) / (var_taub * var_taub)), ((((-var_qb_qs_dn11) * var_taub) - (assign36400_e51097 * var_taub_dn11)) / (var_taub * var_taub)), ((((-var_qb_qs_dn12) * var_taub) - (assign36400_e51097 * var_taub_dn12)) / (var_taub * var_taub)), ((var_qb_nqs_dn13 - var_qb_qs_dn13) / var_taub), ((-var_qb_qs_dn15) / var_taub), ((-var_qb_qs_dn16) / var_taub), ((((-var_qb_qs_dn17) * var_taub) - (assign36400_e51097 * var_taub_dn17)) / (var_taub * var_taub)), ((-var_qb_qs_dn18) / var_taub),)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    }
};
        var_iqb_nqs = assign36400_e51101;
        var_iqb_nqs_dn0 = assign36400_e51101_d_n0;
        var_iqb_nqs_dn2 = assign36400_e51101_d_n2;
        var_iqb_nqs_dn6 = assign36400_e51101_d_n6;
        var_iqb_nqs_dn7 = assign36400_e51101_d_n7;
        var_iqb_nqs_dn10 = assign36400_e51101_d_n10;
        var_iqb_nqs_dn11 = assign36400_e51101_d_n11;
        var_iqb_nqs_dn12 = assign36400_e51101_d_n12;
        var_iqb_nqs_dn13 = assign36400_e51101_d_n13;
        var_iqb_nqs_dn15 = assign36400_e51101_d_n15;
        var_iqb_nqs_dn16 = assign36400_e51101_d_n16;
        var_iqb_nqs_dn17 = assign36400_e51101_d_n17;
        var_iqb_nqs_dn18 = assign36400_e51101_d_n18;

        let (assign36410_e51111, assign36410_e51111_d_n0, assign36410_e51111_d_n2, assign36410_e51111_d_n6, assign36410_e51111_d_n7, assign36410_e51111_d_n10, assign36410_e51111_d_n11, assign36410_e51111_d_n12, assign36410_e51111_d_n15, assign36410_e51111_d_n17, assign36410_e51111_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36410_e51107: f64 = (var_qi_nqs * var_qdrat);
        let assign36410_e51109: f64 = (assign36410_e51107 + var_q_bt_se);
        (assign36410_e51109, ((var_qi_nqs * var_qdrat_dn0) + var_q_bt_se_dn0), ((var_qi_nqs * var_qdrat_dn2) + var_q_bt_se_dn2), ((var_qi_nqs * var_qdrat_dn6) + var_q_bt_se_dn6), ((var_qi_nqs * var_qdrat_dn7) + var_q_bt_se_dn7), ((var_qi_nqs * var_qdrat_dn10) + var_q_bt_se_dn10), ((var_qi_nqs * var_qdrat_dn11) + var_q_bt_se_dn11), ((var_qi_nqs * var_qdrat_dn12) + var_q_bt_se_dn12), 0.0, ((var_qi_nqs * var_qdrat_dn17) + var_q_bt_se_dn17), (var_qi_nqs_dn18 * var_qdrat),)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign36410_e51111;
        var_qd_nqs_dn0 = assign36410_e51111_d_n0;
        var_qd_nqs_dn2 = assign36410_e51111_d_n2;
        var_qd_nqs_dn6 = assign36410_e51111_d_n6;
        var_qd_nqs_dn7 = assign36410_e51111_d_n7;
        var_qd_nqs_dn10 = assign36410_e51111_d_n10;
        var_qd_nqs_dn11 = assign36410_e51111_d_n11;
        var_qd_nqs_dn12 = assign36410_e51111_d_n12;
        var_qd_nqs_dn15 = assign36410_e51111_d_n15;
        var_qd_nqs_dn17 = assign36410_e51111_d_n17;
        var_qd_nqs_dn18 = assign36410_e51111_d_n18;

        let (assign36420_e51123, assign36420_e51123_d_n0, assign36420_e51123_d_n2, assign36420_e51123_d_n6, assign36420_e51123_d_n7, assign36420_e51123_d_n10, assign36420_e51123_d_n11, assign36420_e51123_d_n12, assign36420_e51123_d_n16, assign36420_e51123_d_n17, assign36420_e51123_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36420_e51118: f64 = (1.0 - var_qdrat);
        let assign36420_e51119: f64 = (var_qi_nqs * assign36420_e51118);
        let assign36420_e51121: f64 = (assign36420_e51119 + var_q_bt_se);
        (assign36420_e51121, ((var_qi_nqs * (-var_qdrat_dn0)) + var_q_bt_se_dn0), ((var_qi_nqs * (-var_qdrat_dn2)) + var_q_bt_se_dn2), ((var_qi_nqs * (-var_qdrat_dn6)) + var_q_bt_se_dn6), ((var_qi_nqs * (-var_qdrat_dn7)) + var_q_bt_se_dn7), ((var_qi_nqs * (-var_qdrat_dn10)) + var_q_bt_se_dn10), ((var_qi_nqs * (-var_qdrat_dn11)) + var_q_bt_se_dn11), ((var_qi_nqs * (-var_qdrat_dn12)) + var_q_bt_se_dn12), 0.0, ((var_qi_nqs * (-var_qdrat_dn17)) + var_q_bt_se_dn17), (var_qi_nqs_dn18 * assign36420_e51118),)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign36420_e51123;
        var_qs_nqs_dn0 = assign36420_e51123_d_n0;
        var_qs_nqs_dn2 = assign36420_e51123_d_n2;
        var_qs_nqs_dn6 = assign36420_e51123_d_n6;
        var_qs_nqs_dn7 = assign36420_e51123_d_n7;
        var_qs_nqs_dn10 = assign36420_e51123_d_n10;
        var_qs_nqs_dn11 = assign36420_e51123_d_n11;
        var_qs_nqs_dn12 = assign36420_e51123_d_n12;
        var_qs_nqs_dn16 = assign36420_e51123_d_n16;
        var_qs_nqs_dn17 = assign36420_e51123_d_n17;
        var_qs_nqs_dn18 = assign36420_e51123_d_n18;

        let (assign36430_e51134, assign36430_e51134_d_n0, assign36430_e51134_d_n2, assign36430_e51134_d_n6, assign36430_e51134_d_n7, assign36430_e51134_d_n10, assign36430_e51134_d_n11, assign36430_e51134_d_n12, assign36430_e51134_d_n13, assign36430_e51134_d_n15, assign36430_e51134_d_n16, assign36430_e51134_d_n17, assign36430_e51134_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36430_e51128: f64 = (-var_qi_nqs);
        let assign36430_e51130: f64 = (assign36430_e51128 - var_qb_nqs);
        let assign36430_e51132: f64 = (assign36430_e51130 + var_q_bt_ge);
        (assign36430_e51132, var_q_bt_ge_dn0, var_q_bt_ge_dn2, var_q_bt_ge_dn6, var_q_bt_ge_dn7, var_q_bt_ge_dn10, var_q_bt_ge_dn11, var_q_bt_ge_dn12, (-var_qb_nqs_dn13), 0.0, 0.0, var_q_bt_ge_dn17, (-var_qi_nqs_dn18),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36430_e51134;
        var_qg_nqs_dn0 = assign36430_e51134_d_n0;
        var_qg_nqs_dn2 = assign36430_e51134_d_n2;
        var_qg_nqs_dn6 = assign36430_e51134_d_n6;
        var_qg_nqs_dn7 = assign36430_e51134_d_n7;
        var_qg_nqs_dn10 = assign36430_e51134_d_n10;
        var_qg_nqs_dn11 = assign36430_e51134_d_n11;
        var_qg_nqs_dn12 = assign36430_e51134_d_n12;
        var_qg_nqs_dn13 = assign36430_e51134_d_n13;
        var_qg_nqs_dn15 = assign36430_e51134_d_n15;
        var_qg_nqs_dn16 = assign36430_e51134_d_n16;
        var_qg_nqs_dn17 = assign36430_e51134_d_n17;
        var_qg_nqs_dn18 = assign36430_e51134_d_n18;

        let (assign36440_e51141, assign36440_e51141_d_n0, assign36440_e51141_d_n2, assign36440_e51141_d_n6, assign36440_e51141_d_n7, assign36440_e51141_d_n10, assign36440_e51141_d_n11, assign36440_e51141_d_n12, assign36440_e51141_d_n17, assign36440_e51141_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn6, var_iqi_nqs_dn7, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12, var_iqi_nqs_dn17, var_iqi_nqs_dn18,)
    }
};
        var_iqi_nqs = assign36440_e51141;
        var_iqi_nqs_dn0 = assign36440_e51141_d_n0;
        var_iqi_nqs_dn2 = assign36440_e51141_d_n2;
        var_iqi_nqs_dn6 = assign36440_e51141_d_n6;
        var_iqi_nqs_dn7 = assign36440_e51141_d_n7;
        var_iqi_nqs_dn10 = assign36440_e51141_d_n10;
        var_iqi_nqs_dn11 = assign36440_e51141_d_n11;
        var_iqi_nqs_dn12 = assign36440_e51141_d_n12;
        var_iqi_nqs_dn17 = assign36440_e51141_d_n17;
        var_iqi_nqs_dn18 = assign36440_e51141_d_n18;

        let (assign36450_e51148, assign36450_e51148_d_n0, assign36450_e51148_d_n2, assign36450_e51148_d_n6, assign36450_e51148_d_n7, assign36450_e51148_d_n10, assign36450_e51148_d_n11, assign36450_e51148_d_n12, assign36450_e51148_d_n13, assign36450_e51148_d_n15, assign36450_e51148_d_n16, assign36450_e51148_d_n17, assign36450_e51148_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    }
};
        var_iqb_nqs = assign36450_e51148;
        var_iqb_nqs_dn0 = assign36450_e51148_d_n0;
        var_iqb_nqs_dn2 = assign36450_e51148_d_n2;
        var_iqb_nqs_dn6 = assign36450_e51148_d_n6;
        var_iqb_nqs_dn7 = assign36450_e51148_d_n7;
        var_iqb_nqs_dn10 = assign36450_e51148_d_n10;
        var_iqb_nqs_dn11 = assign36450_e51148_d_n11;
        var_iqb_nqs_dn12 = assign36450_e51148_d_n12;
        var_iqb_nqs_dn13 = assign36450_e51148_d_n13;
        var_iqb_nqs_dn15 = assign36450_e51148_d_n15;
        var_iqb_nqs_dn16 = assign36450_e51148_d_n16;
        var_iqb_nqs_dn17 = assign36450_e51148_d_n17;
        var_iqb_nqs_dn18 = assign36450_e51148_d_n18;

        let (assign36460_e51155, assign36460_e51155_d_n0, assign36460_e51155_d_n2, assign36460_e51155_d_n6, assign36460_e51155_d_n7, assign36460_e51155_d_n10, assign36460_e51155_d_n11, assign36460_e51155_d_n12, assign36460_e51155_d_n15, assign36460_e51155_d_n17, assign36460_e51155_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign36460_e51155;
        var_qd_nqs_dn0 = assign36460_e51155_d_n0;
        var_qd_nqs_dn2 = assign36460_e51155_d_n2;
        var_qd_nqs_dn6 = assign36460_e51155_d_n6;
        var_qd_nqs_dn7 = assign36460_e51155_d_n7;
        var_qd_nqs_dn10 = assign36460_e51155_d_n10;
        var_qd_nqs_dn11 = assign36460_e51155_d_n11;
        var_qd_nqs_dn12 = assign36460_e51155_d_n12;
        var_qd_nqs_dn15 = assign36460_e51155_d_n15;
        var_qd_nqs_dn17 = assign36460_e51155_d_n17;
        var_qd_nqs_dn18 = assign36460_e51155_d_n18;

        let (assign36470_e51162, assign36470_e51162_d_n0, assign36470_e51162_d_n2, assign36470_e51162_d_n6, assign36470_e51162_d_n7, assign36470_e51162_d_n10, assign36470_e51162_d_n11, assign36470_e51162_d_n12, assign36470_e51162_d_n16, assign36470_e51162_d_n17, assign36470_e51162_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign36470_e51162;
        var_qs_nqs_dn0 = assign36470_e51162_d_n0;
        var_qs_nqs_dn2 = assign36470_e51162_d_n2;
        var_qs_nqs_dn6 = assign36470_e51162_d_n6;
        var_qs_nqs_dn7 = assign36470_e51162_d_n7;
        var_qs_nqs_dn10 = assign36470_e51162_d_n10;
        var_qs_nqs_dn11 = assign36470_e51162_d_n11;
        var_qs_nqs_dn12 = assign36470_e51162_d_n12;
        var_qs_nqs_dn16 = assign36470_e51162_d_n16;
        var_qs_nqs_dn17 = assign36470_e51162_d_n17;
        var_qs_nqs_dn18 = assign36470_e51162_d_n18;

        let (assign36480_e51169, assign36480_e51169_d_n0, assign36480_e51169_d_n2, assign36480_e51169_d_n6, assign36480_e51169_d_n7, assign36480_e51169_d_n10, assign36480_e51169_d_n11, assign36480_e51169_d_n12, assign36480_e51169_d_n13, assign36480_e51169_d_n15, assign36480_e51169_d_n16, assign36480_e51169_d_n17, assign36480_e51169_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36480_e51169;
        var_qg_nqs_dn0 = assign36480_e51169_d_n0;
        var_qg_nqs_dn2 = assign36480_e51169_d_n2;
        var_qg_nqs_dn6 = assign36480_e51169_d_n6;
        var_qg_nqs_dn7 = assign36480_e51169_d_n7;
        var_qg_nqs_dn10 = assign36480_e51169_d_n10;
        var_qg_nqs_dn11 = assign36480_e51169_d_n11;
        var_qg_nqs_dn12 = assign36480_e51169_d_n12;
        var_qg_nqs_dn13 = assign36480_e51169_d_n13;
        var_qg_nqs_dn15 = assign36480_e51169_d_n15;
        var_qg_nqs_dn16 = assign36480_e51169_d_n16;
        var_qg_nqs_dn17 = assign36480_e51169_d_n17;
        var_qg_nqs_dn18 = assign36480_e51169_d_n18;

        let (assign36490_e51176, assign36490_e51176_d_n13,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign36490_e51176;
        var_qb_nqs_dn13 = assign36490_e51176_d_n13;

        let assign36500_e51180: f64 = (1e-15 / 0.0001);
        let assign36500_e51181: f64 = if var_tau < assign36500_e51180 { 1.0 } else { 0.0 };
        var_guard1210 = assign36500_e51181;

        let (assign36510_e51192, assign36510_e51192_d_n0, assign36510_e51192_d_n2, assign36510_e51192_d_n6, assign36510_e51192_d_n7, assign36510_e51192_d_n10, assign36510_e51192_d_n11, assign36510_e51192_d_n12, assign36510_e51192_d_n17,) = {
    if (((var_guard1207 == 0.0) && (var_flg_nqs != 0.0)) && (var_guard1210 != 0.0)) {
        let assign36510_e51190: f64 = (1e-15 / 0.0001);
        (assign36510_e51190, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn6, var_tau_dn7, var_tau_dn10, var_tau_dn11, var_tau_dn12, var_tau_dn17,)
    }
};
        var_tau = assign36510_e51192;
        var_tau_dn0 = assign36510_e51192_d_n0;
        var_tau_dn2 = assign36510_e51192_d_n2;
        var_tau_dn6 = assign36510_e51192_d_n6;
        var_tau_dn7 = assign36510_e51192_d_n7;
        var_tau_dn10 = assign36510_e51192_d_n10;
        var_tau_dn11 = assign36510_e51192_d_n11;
        var_tau_dn12 = assign36510_e51192_d_n12;
        var_tau_dn17 = assign36510_e51192_d_n17;

        let assign36520_e51196: f64 = (1e-15 / 0.0001);
        let assign36520_e51197: f64 = if var_taub < assign36520_e51196 { 1.0 } else { 0.0 };
        var_guard1211 = assign36520_e51197;

        let (assign36530_e51208, assign36530_e51208_d_n0, assign36530_e51208_d_n2, assign36530_e51208_d_n6, assign36530_e51208_d_n7, assign36530_e51208_d_n10, assign36530_e51208_d_n11, assign36530_e51208_d_n12, assign36530_e51208_d_n17,) = {
    if (((var_guard1207 == 0.0) && (var_flg_nqs != 0.0)) && (var_guard1211 != 0.0)) {
        let assign36530_e51206: f64 = (1e-15 / 0.0001);
        (assign36530_e51206, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taub, var_taub_dn0, var_taub_dn2, var_taub_dn6, var_taub_dn7, var_taub_dn10, var_taub_dn11, var_taub_dn12, var_taub_dn17,)
    }
};
        var_taub = assign36530_e51208;
        var_taub_dn0 = assign36530_e51208_d_n0;
        var_taub_dn2 = assign36530_e51208_d_n2;
        var_taub_dn6 = assign36530_e51208_d_n6;
        var_taub_dn7 = assign36530_e51208_d_n7;
        var_taub_dn10 = assign36530_e51208_d_n10;
        var_taub_dn11 = assign36530_e51208_d_n11;
        var_taub_dn12 = assign36530_e51208_d_n12;
        var_taub_dn17 = assign36530_e51208_d_n17;

        let (assign36540_e51219, assign36540_e51219_d_n0, assign36540_e51219_d_n2, assign36540_e51219_d_n6, assign36540_e51219_d_n7, assign36540_e51219_d_n10, assign36540_e51219_d_n11, assign36540_e51219_d_n12, assign36540_e51219_d_n13, assign36540_e51219_d_n15, assign36540_e51219_d_n16, assign36540_e51219_d_n17, assign36540_e51219_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign36540_e51215: f64 = (var_qd_nqs - var_qd_qs);
        let assign36540_e51217: f64 = (assign36540_e51215 / var_tau);
        (assign36540_e51217, ((((var_qd_nqs_dn0 - var_qd_qs_dn0) * var_tau) - (assign36540_e51215 * var_tau_dn0)) / (var_tau * var_tau)), ((((var_qd_nqs_dn2 - var_qd_qs_dn2) * var_tau) - (assign36540_e51215 * var_tau_dn2)) / (var_tau * var_tau)), ((((var_qd_nqs_dn6 - var_qd_qs_dn6) * var_tau) - (assign36540_e51215 * var_tau_dn6)) / (var_tau * var_tau)), ((((var_qd_nqs_dn7 - var_qd_qs_dn7) * var_tau) - (assign36540_e51215 * var_tau_dn7)) / (var_tau * var_tau)), ((((var_qd_nqs_dn10 - var_qd_qs_dn10) * var_tau) - (assign36540_e51215 * var_tau_dn10)) / (var_tau * var_tau)), ((((var_qd_nqs_dn11 - var_qd_qs_dn11) * var_tau) - (assign36540_e51215 * var_tau_dn11)) / (var_tau * var_tau)), ((((var_qd_nqs_dn12 - var_qd_qs_dn12) * var_tau) - (assign36540_e51215 * var_tau_dn12)) / (var_tau * var_tau)), ((-var_qd_qs_dn13) / var_tau), ((var_qd_nqs_dn15 - var_qd_qs_dn15) / var_tau), ((-var_qd_qs_dn16) / var_tau), ((((var_qd_nqs_dn17 - var_qd_qs_dn17) * var_tau) - (assign36540_e51215 * var_tau_dn17)) / (var_tau * var_tau)), ((var_qd_nqs_dn18 - var_qd_qs_dn18) / var_tau),)
    } else {
        (var_iqd_nqs, var_iqd_nqs_dn0, var_iqd_nqs_dn2, var_iqd_nqs_dn6, var_iqd_nqs_dn7, var_iqd_nqs_dn10, var_iqd_nqs_dn11, var_iqd_nqs_dn12, var_iqd_nqs_dn13, var_iqd_nqs_dn15, var_iqd_nqs_dn16, var_iqd_nqs_dn17, var_iqd_nqs_dn18,)
    }
};
        var_iqd_nqs = assign36540_e51219;
        var_iqd_nqs_dn0 = assign36540_e51219_d_n0;
        var_iqd_nqs_dn2 = assign36540_e51219_d_n2;
        var_iqd_nqs_dn6 = assign36540_e51219_d_n6;
        var_iqd_nqs_dn7 = assign36540_e51219_d_n7;
        var_iqd_nqs_dn10 = assign36540_e51219_d_n10;
        var_iqd_nqs_dn11 = assign36540_e51219_d_n11;
        var_iqd_nqs_dn12 = assign36540_e51219_d_n12;
        var_iqd_nqs_dn13 = assign36540_e51219_d_n13;
        var_iqd_nqs_dn15 = assign36540_e51219_d_n15;
        var_iqd_nqs_dn16 = assign36540_e51219_d_n16;
        var_iqd_nqs_dn17 = assign36540_e51219_d_n17;
        var_iqd_nqs_dn18 = assign36540_e51219_d_n18;

        let (assign36550_e51230, assign36550_e51230_d_n0, assign36550_e51230_d_n2, assign36550_e51230_d_n6, assign36550_e51230_d_n7, assign36550_e51230_d_n10, assign36550_e51230_d_n11, assign36550_e51230_d_n12, assign36550_e51230_d_n13, assign36550_e51230_d_n15, assign36550_e51230_d_n16, assign36550_e51230_d_n17, assign36550_e51230_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign36550_e51226: f64 = (var_qs_nqs - var_qs_qs);
        let assign36550_e51228: f64 = (assign36550_e51226 / var_tau);
        (assign36550_e51228, ((((var_qs_nqs_dn0 - var_qs_qs_dn0) * var_tau) - (assign36550_e51226 * var_tau_dn0)) / (var_tau * var_tau)), ((((var_qs_nqs_dn2 - var_qs_qs_dn2) * var_tau) - (assign36550_e51226 * var_tau_dn2)) / (var_tau * var_tau)), ((((var_qs_nqs_dn6 - var_qs_qs_dn6) * var_tau) - (assign36550_e51226 * var_tau_dn6)) / (var_tau * var_tau)), ((((var_qs_nqs_dn7 - var_qs_qs_dn7) * var_tau) - (assign36550_e51226 * var_tau_dn7)) / (var_tau * var_tau)), ((((var_qs_nqs_dn10 - var_qs_qs_dn10) * var_tau) - (assign36550_e51226 * var_tau_dn10)) / (var_tau * var_tau)), ((((var_qs_nqs_dn11 - var_qs_qs_dn11) * var_tau) - (assign36550_e51226 * var_tau_dn11)) / (var_tau * var_tau)), ((((var_qs_nqs_dn12 - var_qs_qs_dn12) * var_tau) - (assign36550_e51226 * var_tau_dn12)) / (var_tau * var_tau)), ((-var_qs_qs_dn13) / var_tau), ((-var_qs_qs_dn15) / var_tau), ((var_qs_nqs_dn16 - var_qs_qs_dn16) / var_tau), ((((var_qs_nqs_dn17 - var_qs_qs_dn17) * var_tau) - (assign36550_e51226 * var_tau_dn17)) / (var_tau * var_tau)), ((var_qs_nqs_dn18 - var_qs_qs_dn18) / var_tau),)
    } else {
        (var_iqs_nqs, var_iqs_nqs_dn0, var_iqs_nqs_dn2, var_iqs_nqs_dn6, var_iqs_nqs_dn7, var_iqs_nqs_dn10, var_iqs_nqs_dn11, var_iqs_nqs_dn12, var_iqs_nqs_dn13, var_iqs_nqs_dn15, var_iqs_nqs_dn16, var_iqs_nqs_dn17, var_iqs_nqs_dn18,)
    }
};
        var_iqs_nqs = assign36550_e51230;
        var_iqs_nqs_dn0 = assign36550_e51230_d_n0;
        var_iqs_nqs_dn2 = assign36550_e51230_d_n2;
        var_iqs_nqs_dn6 = assign36550_e51230_d_n6;
        var_iqs_nqs_dn7 = assign36550_e51230_d_n7;
        var_iqs_nqs_dn10 = assign36550_e51230_d_n10;
        var_iqs_nqs_dn11 = assign36550_e51230_d_n11;
        var_iqs_nqs_dn12 = assign36550_e51230_d_n12;
        var_iqs_nqs_dn13 = assign36550_e51230_d_n13;
        var_iqs_nqs_dn15 = assign36550_e51230_d_n15;
        var_iqs_nqs_dn16 = assign36550_e51230_d_n16;
        var_iqs_nqs_dn17 = assign36550_e51230_d_n17;
        var_iqs_nqs_dn18 = assign36550_e51230_d_n18;

        let (assign36560_e51241, assign36560_e51241_d_n0, assign36560_e51241_d_n2, assign36560_e51241_d_n6, assign36560_e51241_d_n7, assign36560_e51241_d_n10, assign36560_e51241_d_n11, assign36560_e51241_d_n12, assign36560_e51241_d_n13, assign36560_e51241_d_n15, assign36560_e51241_d_n16, assign36560_e51241_d_n17, assign36560_e51241_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign36560_e51237: f64 = (var_qb_nqs - var_qb_qs);
        let assign36560_e51239: f64 = (assign36560_e51237 / var_taub);
        (assign36560_e51239, ((((-var_qb_qs_dn0) * var_taub) - (assign36560_e51237 * var_taub_dn0)) / (var_taub * var_taub)), ((((-var_qb_qs_dn2) * var_taub) - (assign36560_e51237 * var_taub_dn2)) / (var_taub * var_taub)), ((((-var_qb_qs_dn6) * var_taub) - (assign36560_e51237 * var_taub_dn6)) / (var_taub * var_taub)), ((((-var_qb_qs_dn7) * var_taub) - (assign36560_e51237 * var_taub_dn7)) / (var_taub * var_taub)), ((((-var_qb_qs_dn10) * var_taub) - (assign36560_e51237 * var_taub_dn10)) / (var_taub * var_taub)), ((((-var_qb_qs_dn11) * var_taub) - (assign36560_e51237 * var_taub_dn11)) / (var_taub * var_taub)), ((((-var_qb_qs_dn12) * var_taub) - (assign36560_e51237 * var_taub_dn12)) / (var_taub * var_taub)), ((var_qb_nqs_dn13 - var_qb_qs_dn13) / var_taub), ((-var_qb_qs_dn15) / var_taub), ((-var_qb_qs_dn16) / var_taub), ((((-var_qb_qs_dn17) * var_taub) - (assign36560_e51237 * var_taub_dn17)) / (var_taub * var_taub)), ((-var_qb_qs_dn18) / var_taub),)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    }
};
        var_iqb_nqs = assign36560_e51241;
        var_iqb_nqs_dn0 = assign36560_e51241_d_n0;
        var_iqb_nqs_dn2 = assign36560_e51241_d_n2;
        var_iqb_nqs_dn6 = assign36560_e51241_d_n6;
        var_iqb_nqs_dn7 = assign36560_e51241_d_n7;
        var_iqb_nqs_dn10 = assign36560_e51241_d_n10;
        var_iqb_nqs_dn11 = assign36560_e51241_d_n11;
        var_iqb_nqs_dn12 = assign36560_e51241_d_n12;
        var_iqb_nqs_dn13 = assign36560_e51241_d_n13;
        var_iqb_nqs_dn15 = assign36560_e51241_d_n15;
        var_iqb_nqs_dn16 = assign36560_e51241_d_n16;
        var_iqb_nqs_dn17 = assign36560_e51241_d_n17;
        var_iqb_nqs_dn18 = assign36560_e51241_d_n18;

        let (assign36570_e51248, assign36570_e51248_d_n0, assign36570_e51248_d_n2, assign36570_e51248_d_n6, assign36570_e51248_d_n7, assign36570_e51248_d_n10, assign36570_e51248_d_n11, assign36570_e51248_d_n12, assign36570_e51248_d_n13, assign36570_e51248_d_n15, assign36570_e51248_d_n16, assign36570_e51248_d_n17, assign36570_e51248_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    }
};
        var_iqb_nqs = assign36570_e51248;
        var_iqb_nqs_dn0 = assign36570_e51248_d_n0;
        var_iqb_nqs_dn2 = assign36570_e51248_d_n2;
        var_iqb_nqs_dn6 = assign36570_e51248_d_n6;
        var_iqb_nqs_dn7 = assign36570_e51248_d_n7;
        var_iqb_nqs_dn10 = assign36570_e51248_d_n10;
        var_iqb_nqs_dn11 = assign36570_e51248_d_n11;
        var_iqb_nqs_dn12 = assign36570_e51248_d_n12;
        var_iqb_nqs_dn13 = assign36570_e51248_d_n13;
        var_iqb_nqs_dn15 = assign36570_e51248_d_n15;
        var_iqb_nqs_dn16 = assign36570_e51248_d_n16;
        var_iqb_nqs_dn17 = assign36570_e51248_d_n17;
        var_iqb_nqs_dn18 = assign36570_e51248_d_n18;

        let (assign36580_e51260, assign36580_e51260_d_n0, assign36580_e51260_d_n2, assign36580_e51260_d_n6, assign36580_e51260_d_n7, assign36580_e51260_d_n10, assign36580_e51260_d_n11, assign36580_e51260_d_n12, assign36580_e51260_d_n13, assign36580_e51260_d_n15, assign36580_e51260_d_n16, assign36580_e51260_d_n17, assign36580_e51260_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign36580_e51254: f64 = (-var_qd_nqs);
        let assign36580_e51256: f64 = (assign36580_e51254 - var_qs_nqs);
        let assign36580_e51258: f64 = (assign36580_e51256 - var_qb_nqs);
        (assign36580_e51258, ((-var_qd_nqs_dn0) - var_qs_nqs_dn0), ((-var_qd_nqs_dn2) - var_qs_nqs_dn2), ((-var_qd_nqs_dn6) - var_qs_nqs_dn6), ((-var_qd_nqs_dn7) - var_qs_nqs_dn7), ((-var_qd_nqs_dn10) - var_qs_nqs_dn10), ((-var_qd_nqs_dn11) - var_qs_nqs_dn11), ((-var_qd_nqs_dn12) - var_qs_nqs_dn12), (-var_qb_nqs_dn13), (-var_qd_nqs_dn15), (-var_qs_nqs_dn16), ((-var_qd_nqs_dn17) - var_qs_nqs_dn17), ((-var_qd_nqs_dn18) - var_qs_nqs_dn18),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36580_e51260;
        var_qg_nqs_dn0 = assign36580_e51260_d_n0;
        var_qg_nqs_dn2 = assign36580_e51260_d_n2;
        var_qg_nqs_dn6 = assign36580_e51260_d_n6;
        var_qg_nqs_dn7 = assign36580_e51260_d_n7;
        var_qg_nqs_dn10 = assign36580_e51260_d_n10;
        var_qg_nqs_dn11 = assign36580_e51260_d_n11;
        var_qg_nqs_dn12 = assign36580_e51260_d_n12;
        var_qg_nqs_dn13 = assign36580_e51260_d_n13;
        var_qg_nqs_dn15 = assign36580_e51260_d_n15;
        var_qg_nqs_dn16 = assign36580_e51260_d_n16;
        var_qg_nqs_dn17 = assign36580_e51260_d_n17;
        var_qg_nqs_dn18 = assign36580_e51260_d_n18;

        let (assign36590_e51268, assign36590_e51268_d_n0, assign36590_e51268_d_n2, assign36590_e51268_d_n6, assign36590_e51268_d_n7, assign36590_e51268_d_n10, assign36590_e51268_d_n11, assign36590_e51268_d_n12, assign36590_e51268_d_n13, assign36590_e51268_d_n15, assign36590_e51268_d_n16, assign36590_e51268_d_n17, assign36590_e51268_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqd_nqs, var_iqd_nqs_dn0, var_iqd_nqs_dn2, var_iqd_nqs_dn6, var_iqd_nqs_dn7, var_iqd_nqs_dn10, var_iqd_nqs_dn11, var_iqd_nqs_dn12, var_iqd_nqs_dn13, var_iqd_nqs_dn15, var_iqd_nqs_dn16, var_iqd_nqs_dn17, var_iqd_nqs_dn18,)
    }
};
        var_iqd_nqs = assign36590_e51268;
        var_iqd_nqs_dn0 = assign36590_e51268_d_n0;
        var_iqd_nqs_dn2 = assign36590_e51268_d_n2;
        var_iqd_nqs_dn6 = assign36590_e51268_d_n6;
        var_iqd_nqs_dn7 = assign36590_e51268_d_n7;
        var_iqd_nqs_dn10 = assign36590_e51268_d_n10;
        var_iqd_nqs_dn11 = assign36590_e51268_d_n11;
        var_iqd_nqs_dn12 = assign36590_e51268_d_n12;
        var_iqd_nqs_dn13 = assign36590_e51268_d_n13;
        var_iqd_nqs_dn15 = assign36590_e51268_d_n15;
        var_iqd_nqs_dn16 = assign36590_e51268_d_n16;
        var_iqd_nqs_dn17 = assign36590_e51268_d_n17;
        var_iqd_nqs_dn18 = assign36590_e51268_d_n18;

        let (assign36600_e51276, assign36600_e51276_d_n0, assign36600_e51276_d_n2, assign36600_e51276_d_n6, assign36600_e51276_d_n7, assign36600_e51276_d_n10, assign36600_e51276_d_n11, assign36600_e51276_d_n12, assign36600_e51276_d_n13, assign36600_e51276_d_n15, assign36600_e51276_d_n16, assign36600_e51276_d_n17, assign36600_e51276_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqs_nqs, var_iqs_nqs_dn0, var_iqs_nqs_dn2, var_iqs_nqs_dn6, var_iqs_nqs_dn7, var_iqs_nqs_dn10, var_iqs_nqs_dn11, var_iqs_nqs_dn12, var_iqs_nqs_dn13, var_iqs_nqs_dn15, var_iqs_nqs_dn16, var_iqs_nqs_dn17, var_iqs_nqs_dn18,)
    }
};
        var_iqs_nqs = assign36600_e51276;
        var_iqs_nqs_dn0 = assign36600_e51276_d_n0;
        var_iqs_nqs_dn2 = assign36600_e51276_d_n2;
        var_iqs_nqs_dn6 = assign36600_e51276_d_n6;
        var_iqs_nqs_dn7 = assign36600_e51276_d_n7;
        var_iqs_nqs_dn10 = assign36600_e51276_d_n10;
        var_iqs_nqs_dn11 = assign36600_e51276_d_n11;
        var_iqs_nqs_dn12 = assign36600_e51276_d_n12;
        var_iqs_nqs_dn13 = assign36600_e51276_d_n13;
        var_iqs_nqs_dn15 = assign36600_e51276_d_n15;
        var_iqs_nqs_dn16 = assign36600_e51276_d_n16;
        var_iqs_nqs_dn17 = assign36600_e51276_d_n17;
        var_iqs_nqs_dn18 = assign36600_e51276_d_n18;

        let (assign36610_e51284, assign36610_e51284_d_n0, assign36610_e51284_d_n2, assign36610_e51284_d_n6, assign36610_e51284_d_n7, assign36610_e51284_d_n10, assign36610_e51284_d_n11, assign36610_e51284_d_n12, assign36610_e51284_d_n13, assign36610_e51284_d_n15, assign36610_e51284_d_n16, assign36610_e51284_d_n17, assign36610_e51284_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    }
};
        var_iqb_nqs = assign36610_e51284;
        var_iqb_nqs_dn0 = assign36610_e51284_d_n0;
        var_iqb_nqs_dn2 = assign36610_e51284_d_n2;
        var_iqb_nqs_dn6 = assign36610_e51284_d_n6;
        var_iqb_nqs_dn7 = assign36610_e51284_d_n7;
        var_iqb_nqs_dn10 = assign36610_e51284_d_n10;
        var_iqb_nqs_dn11 = assign36610_e51284_d_n11;
        var_iqb_nqs_dn12 = assign36610_e51284_d_n12;
        var_iqb_nqs_dn13 = assign36610_e51284_d_n13;
        var_iqb_nqs_dn15 = assign36610_e51284_d_n15;
        var_iqb_nqs_dn16 = assign36610_e51284_d_n16;
        var_iqb_nqs_dn17 = assign36610_e51284_d_n17;
        var_iqb_nqs_dn18 = assign36610_e51284_d_n18;

        let (assign36620_e51292, assign36620_e51292_d_n0, assign36620_e51292_d_n2, assign36620_e51292_d_n6, assign36620_e51292_d_n7, assign36620_e51292_d_n10, assign36620_e51292_d_n11, assign36620_e51292_d_n12, assign36620_e51292_d_n15, assign36620_e51292_d_n17, assign36620_e51292_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign36620_e51292;
        var_qd_nqs_dn0 = assign36620_e51292_d_n0;
        var_qd_nqs_dn2 = assign36620_e51292_d_n2;
        var_qd_nqs_dn6 = assign36620_e51292_d_n6;
        var_qd_nqs_dn7 = assign36620_e51292_d_n7;
        var_qd_nqs_dn10 = assign36620_e51292_d_n10;
        var_qd_nqs_dn11 = assign36620_e51292_d_n11;
        var_qd_nqs_dn12 = assign36620_e51292_d_n12;
        var_qd_nqs_dn15 = assign36620_e51292_d_n15;
        var_qd_nqs_dn17 = assign36620_e51292_d_n17;
        var_qd_nqs_dn18 = assign36620_e51292_d_n18;

        let (assign36630_e51300, assign36630_e51300_d_n0, assign36630_e51300_d_n2, assign36630_e51300_d_n6, assign36630_e51300_d_n7, assign36630_e51300_d_n10, assign36630_e51300_d_n11, assign36630_e51300_d_n12, assign36630_e51300_d_n16, assign36630_e51300_d_n17, assign36630_e51300_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign36630_e51300;
        var_qs_nqs_dn0 = assign36630_e51300_d_n0;
        var_qs_nqs_dn2 = assign36630_e51300_d_n2;
        var_qs_nqs_dn6 = assign36630_e51300_d_n6;
        var_qs_nqs_dn7 = assign36630_e51300_d_n7;
        var_qs_nqs_dn10 = assign36630_e51300_d_n10;
        var_qs_nqs_dn11 = assign36630_e51300_d_n11;
        var_qs_nqs_dn12 = assign36630_e51300_d_n12;
        var_qs_nqs_dn16 = assign36630_e51300_d_n16;
        var_qs_nqs_dn17 = assign36630_e51300_d_n17;
        var_qs_nqs_dn18 = assign36630_e51300_d_n18;

        let (assign36640_e51308, assign36640_e51308_d_n0, assign36640_e51308_d_n2, assign36640_e51308_d_n6, assign36640_e51308_d_n7, assign36640_e51308_d_n10, assign36640_e51308_d_n11, assign36640_e51308_d_n12, assign36640_e51308_d_n13, assign36640_e51308_d_n15, assign36640_e51308_d_n16, assign36640_e51308_d_n17, assign36640_e51308_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36640_e51308;
        var_qg_nqs_dn0 = assign36640_e51308_d_n0;
        var_qg_nqs_dn2 = assign36640_e51308_d_n2;
        var_qg_nqs_dn6 = assign36640_e51308_d_n6;
        var_qg_nqs_dn7 = assign36640_e51308_d_n7;
        var_qg_nqs_dn10 = assign36640_e51308_d_n10;
        var_qg_nqs_dn11 = assign36640_e51308_d_n11;
        var_qg_nqs_dn12 = assign36640_e51308_d_n12;
        var_qg_nqs_dn13 = assign36640_e51308_d_n13;
        var_qg_nqs_dn15 = assign36640_e51308_d_n15;
        var_qg_nqs_dn16 = assign36640_e51308_d_n16;
        var_qg_nqs_dn17 = assign36640_e51308_d_n17;
        var_qg_nqs_dn18 = assign36640_e51308_d_n18;

        let (assign36650_e51316, assign36650_e51316_d_n13,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign36650_e51316;
        var_qb_nqs_dn13 = assign36650_e51316_d_n13;

        var_rdd = var_rdde;
        var_rdd_dn0 = var_rdde_dn0;
        var_rdd_dn2 = var_rdde_dn2;
        var_rdd_dn6 = var_rdde_dn6;
        var_rdd_dn7 = var_rdde_dn7;
        var_rdd_dn10 = var_rdde_dn10;
        var_rdd_dn11 = var_rdde_dn11;
        var_rdd_dn12 = var_rdde_dn12;
        var_rdd_dn17 = var_rdde_dn17;

        var_rsd = var_rsde;
        var_rsd_dn0 = var_rsde_dn0;
        var_rsd_dn2 = var_rsde_dn2;
        var_rsd_dn6 = var_rsde_dn6;
        var_rsd_dn7 = var_rsde_dn7;
        var_rsd_dn10 = var_rsde_dn10;
        var_rsd_dn11 = var_rsde_dn11;
        var_rsd_dn12 = var_rsde_dn12;
        var_rsd_dn17 = var_rsde_dn17;

        let assign36680_e51321: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard1212 = assign36680_e51321;

        *var_guard1210_slot = var_guard1210;
        *var_guard1211_slot = var_guard1211;
        *var_guard1212_slot = var_guard1212;
        *var_iqb_nqs_slot = var_iqb_nqs;
        *var_iqb_nqs_dn0_slot = var_iqb_nqs_dn0;
        *var_iqb_nqs_dn10_slot = var_iqb_nqs_dn10;
        *var_iqb_nqs_dn11_slot = var_iqb_nqs_dn11;
        *var_iqb_nqs_dn12_slot = var_iqb_nqs_dn12;
        *var_iqb_nqs_dn13_slot = var_iqb_nqs_dn13;
        *var_iqb_nqs_dn15_slot = var_iqb_nqs_dn15;
        *var_iqb_nqs_dn16_slot = var_iqb_nqs_dn16;
        *var_iqb_nqs_dn17_slot = var_iqb_nqs_dn17;
        *var_iqb_nqs_dn18_slot = var_iqb_nqs_dn18;
        *var_iqb_nqs_dn2_slot = var_iqb_nqs_dn2;
        *var_iqb_nqs_dn6_slot = var_iqb_nqs_dn6;
        *var_iqb_nqs_dn7_slot = var_iqb_nqs_dn7;
        *var_iqd_nqs_slot = var_iqd_nqs;
        *var_iqd_nqs_dn0_slot = var_iqd_nqs_dn0;
        *var_iqd_nqs_dn10_slot = var_iqd_nqs_dn10;
        *var_iqd_nqs_dn11_slot = var_iqd_nqs_dn11;
        *var_iqd_nqs_dn12_slot = var_iqd_nqs_dn12;
        *var_iqd_nqs_dn13_slot = var_iqd_nqs_dn13;
        *var_iqd_nqs_dn15_slot = var_iqd_nqs_dn15;
        *var_iqd_nqs_dn16_slot = var_iqd_nqs_dn16;
        *var_iqd_nqs_dn17_slot = var_iqd_nqs_dn17;
        *var_iqd_nqs_dn18_slot = var_iqd_nqs_dn18;
        *var_iqd_nqs_dn2_slot = var_iqd_nqs_dn2;
        *var_iqd_nqs_dn6_slot = var_iqd_nqs_dn6;
        *var_iqd_nqs_dn7_slot = var_iqd_nqs_dn7;
        *var_iqi_nqs_slot = var_iqi_nqs;
        *var_iqi_nqs_dn0_slot = var_iqi_nqs_dn0;
        *var_iqi_nqs_dn10_slot = var_iqi_nqs_dn10;
        *var_iqi_nqs_dn11_slot = var_iqi_nqs_dn11;
        *var_iqi_nqs_dn12_slot = var_iqi_nqs_dn12;
        *var_iqi_nqs_dn17_slot = var_iqi_nqs_dn17;
        *var_iqi_nqs_dn18_slot = var_iqi_nqs_dn18;
        *var_iqi_nqs_dn2_slot = var_iqi_nqs_dn2;
        *var_iqi_nqs_dn6_slot = var_iqi_nqs_dn6;
        *var_iqi_nqs_dn7_slot = var_iqi_nqs_dn7;
        *var_iqs_nqs_slot = var_iqs_nqs;
        *var_iqs_nqs_dn0_slot = var_iqs_nqs_dn0;
        *var_iqs_nqs_dn10_slot = var_iqs_nqs_dn10;
        *var_iqs_nqs_dn11_slot = var_iqs_nqs_dn11;
        *var_iqs_nqs_dn12_slot = var_iqs_nqs_dn12;
        *var_iqs_nqs_dn13_slot = var_iqs_nqs_dn13;
        *var_iqs_nqs_dn15_slot = var_iqs_nqs_dn15;
        *var_iqs_nqs_dn16_slot = var_iqs_nqs_dn16;
        *var_iqs_nqs_dn17_slot = var_iqs_nqs_dn17;
        *var_iqs_nqs_dn18_slot = var_iqs_nqs_dn18;
        *var_iqs_nqs_dn2_slot = var_iqs_nqs_dn2;
        *var_iqs_nqs_dn6_slot = var_iqs_nqs_dn6;
        *var_iqs_nqs_dn7_slot = var_iqs_nqs_dn7;
        *var_qb_nqs_slot = var_qb_nqs;
        *var_qb_nqs_dn13_slot = var_qb_nqs_dn13;
        *var_qd_nqs_slot = var_qd_nqs;
        *var_qd_nqs_dn0_slot = var_qd_nqs_dn0;
        *var_qd_nqs_dn10_slot = var_qd_nqs_dn10;
        *var_qd_nqs_dn11_slot = var_qd_nqs_dn11;
        *var_qd_nqs_dn12_slot = var_qd_nqs_dn12;
        *var_qd_nqs_dn15_slot = var_qd_nqs_dn15;
        *var_qd_nqs_dn17_slot = var_qd_nqs_dn17;
        *var_qd_nqs_dn18_slot = var_qd_nqs_dn18;
        *var_qd_nqs_dn2_slot = var_qd_nqs_dn2;
        *var_qd_nqs_dn6_slot = var_qd_nqs_dn6;
        *var_qd_nqs_dn7_slot = var_qd_nqs_dn7;
        *var_qg_nqs_slot = var_qg_nqs;
        *var_qg_nqs_dn0_slot = var_qg_nqs_dn0;
        *var_qg_nqs_dn10_slot = var_qg_nqs_dn10;
        *var_qg_nqs_dn11_slot = var_qg_nqs_dn11;
        *var_qg_nqs_dn12_slot = var_qg_nqs_dn12;
        *var_qg_nqs_dn13_slot = var_qg_nqs_dn13;
        *var_qg_nqs_dn15_slot = var_qg_nqs_dn15;
        *var_qg_nqs_dn16_slot = var_qg_nqs_dn16;
        *var_qg_nqs_dn17_slot = var_qg_nqs_dn17;
        *var_qg_nqs_dn18_slot = var_qg_nqs_dn18;
        *var_qg_nqs_dn2_slot = var_qg_nqs_dn2;
        *var_qg_nqs_dn6_slot = var_qg_nqs_dn6;
        *var_qg_nqs_dn7_slot = var_qg_nqs_dn7;
        *var_qs_nqs_slot = var_qs_nqs;
        *var_qs_nqs_dn0_slot = var_qs_nqs_dn0;
        *var_qs_nqs_dn10_slot = var_qs_nqs_dn10;
        *var_qs_nqs_dn11_slot = var_qs_nqs_dn11;
        *var_qs_nqs_dn12_slot = var_qs_nqs_dn12;
        *var_qs_nqs_dn16_slot = var_qs_nqs_dn16;
        *var_qs_nqs_dn17_slot = var_qs_nqs_dn17;
        *var_qs_nqs_dn18_slot = var_qs_nqs_dn18;
        *var_qs_nqs_dn2_slot = var_qs_nqs_dn2;
        *var_qs_nqs_dn6_slot = var_qs_nqs_dn6;
        *var_qs_nqs_dn7_slot = var_qs_nqs_dn7;
        *var_rdd_slot = var_rdd;
        *var_rdd_dn0_slot = var_rdd_dn0;
        *var_rdd_dn10_slot = var_rdd_dn10;
        *var_rdd_dn11_slot = var_rdd_dn11;
        *var_rdd_dn12_slot = var_rdd_dn12;
        *var_rdd_dn17_slot = var_rdd_dn17;
        *var_rdd_dn2_slot = var_rdd_dn2;
        *var_rdd_dn6_slot = var_rdd_dn6;
        *var_rdd_dn7_slot = var_rdd_dn7;
        *var_rsd_slot = var_rsd;
        *var_rsd_dn0_slot = var_rsd_dn0;
        *var_rsd_dn10_slot = var_rsd_dn10;
        *var_rsd_dn11_slot = var_rsd_dn11;
        *var_rsd_dn12_slot = var_rsd_dn12;
        *var_rsd_dn17_slot = var_rsd_dn17;
        *var_rsd_dn2_slot = var_rsd_dn2;
        *var_rsd_dn6_slot = var_rsd_dn6;
        *var_rsd_dn7_slot = var_rsd_dn7;
        *var_tau_slot = var_tau;
        *var_tau_dn0_slot = var_tau_dn0;
        *var_tau_dn10_slot = var_tau_dn10;
        *var_tau_dn11_slot = var_tau_dn11;
        *var_tau_dn12_slot = var_tau_dn12;
        *var_tau_dn17_slot = var_tau_dn17;
        *var_tau_dn2_slot = var_tau_dn2;
        *var_tau_dn6_slot = var_tau_dn6;
        *var_tau_dn7_slot = var_tau_dn7;
        *var_taub_slot = var_taub;
        *var_taub_dn0_slot = var_taub_dn0;
        *var_taub_dn10_slot = var_taub_dn10;
        *var_taub_dn11_slot = var_taub_dn11;
        *var_taub_dn12_slot = var_taub_dn12;
        *var_taub_dn17_slot = var_taub_dn17;
        *var_taub_dn2_slot = var_taub_dn2;
        *var_taub_dn6_slot = var_taub_dn6;
        *var_taub_dn7_slot = var_taub_dn7;
    }

    pub(super) fn stamp_transient_block_127(
        p: &Parameters,
        var_cth: f64,
        var_guard1212: f64,
        var_ibdb: f64,
        var_ibdb_dn0: f64,
        var_ibdb_dn10: f64,
        var_ibdb_dn11: f64,
        var_ibdb_dn12: f64,
        var_ibdb_dn17: f64,
        var_ibdb_dn2: f64,
        var_ibdb_dn6: f64,
        var_ibdb_dn7: f64,
        var_ibsb: f64,
        var_ibsb_dn0: f64,
        var_ibsb_dn10: f64,
        var_ibsb_dn11: f64,
        var_ibsb_dn12: f64,
        var_ibsb_dn17: f64,
        var_ibsb_dn2: f64,
        var_ibsb_dn6: f64,
        var_ibsb_dn7: f64,
        var_igbe: f64,
        var_igbe_dn0: f64,
        var_igbe_dn10: f64,
        var_igbe_dn11: f64,
        var_igbe_dn12: f64,
        var_igbe_dn17: f64,
        var_igbe_dn2: f64,
        var_igbe_dn6: f64,
        var_igbe_dn7: f64,
        var_igde: f64,
        var_igde_dn0: f64,
        var_igde_dn10: f64,
        var_igde_dn11: f64,
        var_igde_dn12: f64,
        var_igde_dn17: f64,
        var_igde_dn2: f64,
        var_igde_dn6: f64,
        var_igde_dn7: f64,
        var_igidle: f64,
        var_igidle_dn0: f64,
        var_igidle_dn10: f64,
        var_igidle_dn11: f64,
        var_igidle_dn12: f64,
        var_igidle_dn17: f64,
        var_igidle_dn2: f64,
        var_igidle_dn6: f64,
        var_igidle_dn7: f64,
        var_igisle: f64,
        var_igisle_dn0: f64,
        var_igisle_dn10: f64,
        var_igisle_dn11: f64,
        var_igisle_dn12: f64,
        var_igisle_dn17: f64,
        var_igisle_dn2: f64,
        var_igisle_dn6: f64,
        var_igisle_dn7: f64,
        var_igse: f64,
        var_igse_dn0: f64,
        var_igse_dn10: f64,
        var_igse_dn11: f64,
        var_igse_dn12: f64,
        var_igse_dn17: f64,
        var_igse_dn2: f64,
        var_igse_dn6: f64,
        var_igse_dn7: f64,
        var_isube: f64,
        var_isube_dn0: f64,
        var_isube_dn10: f64,
        var_isube_dn11: f64,
        var_isube_dn12: f64,
        var_isube_dn17: f64,
        var_isube_dn2: f64,
        var_isube_dn6: f64,
        var_isube_dn7: f64,
        var_mks_rth0: f64,
        var_qb_nqs: f64,
        var_qb_nqs_dn13: f64,
        var_qbd_s0: f64,
        var_qbd_s0_dn0: f64,
        var_qbd_s0_dn10: f64,
        var_qbd_s0_dn11: f64,
        var_qbd_s0_dn12: f64,
        var_qbd_s0_dn17: f64,
        var_qbd_s0_dn2: f64,
        var_qbd_s0_dn6: f64,
        var_qbd_s0_dn7: f64,
        var_qbs_s0: f64,
        var_qbs_s0_dn0: f64,
        var_qbs_s0_dn10: f64,
        var_qbs_s0_dn11: f64,
        var_qbs_s0_dn12: f64,
        var_qbs_s0_dn17: f64,
        var_qbs_s0_dn2: f64,
        var_qbs_s0_dn6: f64,
        var_qbs_s0_dn7: f64,
        var_qd_nqs: f64,
        var_qd_nqs_dn0: f64,
        var_qd_nqs_dn10: f64,
        var_qd_nqs_dn11: f64,
        var_qd_nqs_dn12: f64,
        var_qd_nqs_dn15: f64,
        var_qd_nqs_dn17: f64,
        var_qd_nqs_dn18: f64,
        var_qd_nqs_dn2: f64,
        var_qd_nqs_dn6: f64,
        var_qd_nqs_dn7: f64,
        var_qde: f64,
        var_qde_dn0: f64,
        var_qde_dn10: f64,
        var_qde_dn11: f64,
        var_qde_dn12: f64,
        var_qde_dn13: f64,
        var_qde_dn15: f64,
        var_qde_dn16: f64,
        var_qde_dn17: f64,
        var_qde_dn18: f64,
        var_qde_dn2: f64,
        var_qde_dn6: f64,
        var_qde_dn7: f64,
        var_qg_nqs: f64,
        var_qg_nqs_dn0: f64,
        var_qg_nqs_dn10: f64,
        var_qg_nqs_dn11: f64,
        var_qg_nqs_dn12: f64,
        var_qg_nqs_dn13: f64,
        var_qg_nqs_dn15: f64,
        var_qg_nqs_dn16: f64,
        var_qg_nqs_dn17: f64,
        var_qg_nqs_dn18: f64,
        var_qg_nqs_dn2: f64,
        var_qg_nqs_dn6: f64,
        var_qg_nqs_dn7: f64,
        var_qge: f64,
        var_qge_dn0: f64,
        var_qge_dn10: f64,
        var_qge_dn11: f64,
        var_qge_dn12: f64,
        var_qge_dn13: f64,
        var_qge_dn15: f64,
        var_qge_dn16: f64,
        var_qge_dn17: f64,
        var_qge_dn18: f64,
        var_qge_dn2: f64,
        var_qge_dn6: f64,
        var_qge_dn7: f64,
        var_qs_nqs: f64,
        var_qs_nqs_dn0: f64,
        var_qs_nqs_dn10: f64,
        var_qs_nqs_dn11: f64,
        var_qs_nqs_dn12: f64,
        var_qs_nqs_dn16: f64,
        var_qs_nqs_dn17: f64,
        var_qs_nqs_dn18: f64,
        var_qs_nqs_dn2: f64,
        var_qs_nqs_dn6: f64,
        var_qs_nqs_dn7: f64,
        var_qse: f64,
        var_qse_dn0: f64,
        var_qse_dn10: f64,
        var_qse_dn11: f64,
        var_qse_dn12: f64,
        var_qse_dn13: f64,
        var_qse_dn15: f64,
        var_qse_dn16: f64,
        var_qse_dn17: f64,
        var_qse_dn18: f64,
        var_qse_dn2: f64,
        var_qse_dn6: f64,
        var_qse_dn7: f64,
        var_rth: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn17: f64,
        var_vds_dn2: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_cgdbd_slot: &mut f64,
        var_cgdbd_dn0_slot: &mut f64,
        var_cgdbd_dn10_slot: &mut f64,
        var_cgdbd_dn11_slot: &mut f64,
        var_cgdbd_dn12_slot: &mut f64,
        var_cgdbd_dn13_slot: &mut f64,
        var_cgdbd_dn15_slot: &mut f64,
        var_cgdbd_dn16_slot: &mut f64,
        var_cgdbd_dn17_slot: &mut f64,
        var_cgdbd_dn18_slot: &mut f64,
        var_cgdbd_dn2_slot: &mut f64,
        var_cgdbd_dn6_slot: &mut f64,
        var_cgdbd_dn7_slot: &mut f64,
        var_cthe_slot: &mut f64,
        var_gth_slot: &mut f64,
        var_guard1213_slot: &mut f64,
        var_guard1214_slot: &mut f64,
        var_ibd_slot: &mut f64,
        var_ibd_dn0_slot: &mut f64,
        var_ibd_dn10_slot: &mut f64,
        var_ibd_dn11_slot: &mut f64,
        var_ibd_dn12_slot: &mut f64,
        var_ibd_dn17_slot: &mut f64,
        var_ibd_dn2_slot: &mut f64,
        var_ibd_dn6_slot: &mut f64,
        var_ibd_dn7_slot: &mut f64,
        var_ibs_slot: &mut f64,
        var_ibs_dn0_slot: &mut f64,
        var_ibs_dn10_slot: &mut f64,
        var_ibs_dn11_slot: &mut f64,
        var_ibs_dn12_slot: &mut f64,
        var_ibs_dn17_slot: &mut f64,
        var_ibs_dn2_slot: &mut f64,
        var_ibs_dn6_slot: &mut f64,
        var_ibs_dn7_slot: &mut f64,
        var_ids_slot: &mut f64,
        var_ids_dn0_slot: &mut f64,
        var_ids_dn10_slot: &mut f64,
        var_ids_dn11_slot: &mut f64,
        var_ids_dn12_slot: &mut f64,
        var_ids_dn17_slot: &mut f64,
        var_ids_dn2_slot: &mut f64,
        var_ids_dn6_slot: &mut f64,
        var_ids_dn7_slot: &mut f64,
        var_idse_slot: &mut f64,
        var_idse_dn0_slot: &mut f64,
        var_idse_dn10_slot: &mut f64,
        var_idse_dn11_slot: &mut f64,
        var_idse_dn12_slot: &mut f64,
        var_idse_dn17_slot: &mut f64,
        var_idse_dn2_slot: &mut f64,
        var_idse_dn6_slot: &mut f64,
        var_idse_dn7_slot: &mut f64,
        var_igb_slot: &mut f64,
        var_igb_dn0_slot: &mut f64,
        var_igb_dn10_slot: &mut f64,
        var_igb_dn11_slot: &mut f64,
        var_igb_dn12_slot: &mut f64,
        var_igb_dn17_slot: &mut f64,
        var_igb_dn2_slot: &mut f64,
        var_igb_dn6_slot: &mut f64,
        var_igb_dn7_slot: &mut f64,
        var_igd_slot: &mut f64,
        var_igd_dn0_slot: &mut f64,
        var_igd_dn10_slot: &mut f64,
        var_igd_dn11_slot: &mut f64,
        var_igd_dn12_slot: &mut f64,
        var_igd_dn17_slot: &mut f64,
        var_igd_dn2_slot: &mut f64,
        var_igd_dn6_slot: &mut f64,
        var_igd_dn7_slot: &mut f64,
        var_igidl_slot: &mut f64,
        var_igidl_dn0_slot: &mut f64,
        var_igidl_dn10_slot: &mut f64,
        var_igidl_dn11_slot: &mut f64,
        var_igidl_dn12_slot: &mut f64,
        var_igidl_dn17_slot: &mut f64,
        var_igidl_dn2_slot: &mut f64,
        var_igidl_dn6_slot: &mut f64,
        var_igidl_dn7_slot: &mut f64,
        var_igisl_slot: &mut f64,
        var_igisl_dn0_slot: &mut f64,
        var_igisl_dn10_slot: &mut f64,
        var_igisl_dn11_slot: &mut f64,
        var_igisl_dn12_slot: &mut f64,
        var_igisl_dn17_slot: &mut f64,
        var_igisl_dn2_slot: &mut f64,
        var_igisl_dn6_slot: &mut f64,
        var_igisl_dn7_slot: &mut f64,
        var_igs_slot: &mut f64,
        var_igs_dn0_slot: &mut f64,
        var_igs_dn10_slot: &mut f64,
        var_igs_dn11_slot: &mut f64,
        var_igs_dn12_slot: &mut f64,
        var_igs_dn17_slot: &mut f64,
        var_igs_dn2_slot: &mut f64,
        var_igs_dn6_slot: &mut f64,
        var_igs_dn7_slot: &mut f64,
        var_isub_slot: &mut f64,
        var_isub_dn0_slot: &mut f64,
        var_isub_dn10_slot: &mut f64,
        var_isub_dn11_slot: &mut f64,
        var_isub_dn12_slot: &mut f64,
        var_isub_dn17_slot: &mut f64,
        var_isub_dn2_slot: &mut f64,
        var_isub_dn6_slot: &mut f64,
        var_isub_dn7_slot: &mut f64,
        var_isubs_slot: &mut f64,
        var_isubs_dn0_slot: &mut f64,
        var_isubs_dn10_slot: &mut f64,
        var_isubs_dn11_slot: &mut f64,
        var_isubs_dn12_slot: &mut f64,
        var_isubs_dn17_slot: &mut f64,
        var_isubs_dn2_slot: &mut f64,
        var_isubs_dn6_slot: &mut f64,
        var_isubs_dn7_slot: &mut f64,
        var_qb_slot: &mut f64,
        var_qb_dn0_slot: &mut f64,
        var_qb_dn10_slot: &mut f64,
        var_qb_dn11_slot: &mut f64,
        var_qb_dn12_slot: &mut f64,
        var_qb_dn13_slot: &mut f64,
        var_qb_dn15_slot: &mut f64,
        var_qb_dn16_slot: &mut f64,
        var_qb_dn17_slot: &mut f64,
        var_qb_dn18_slot: &mut f64,
        var_qb_dn2_slot: &mut f64,
        var_qb_dn6_slot: &mut f64,
        var_qb_dn7_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn0_slot: &mut f64,
        var_qbd_dn10_slot: &mut f64,
        var_qbd_dn11_slot: &mut f64,
        var_qbd_dn12_slot: &mut f64,
        var_qbd_dn17_slot: &mut f64,
        var_qbd_dn2_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbe_slot: &mut f64,
        var_qbe_dn0_slot: &mut f64,
        var_qbe_dn10_slot: &mut f64,
        var_qbe_dn11_slot: &mut f64,
        var_qbe_dn12_slot: &mut f64,
        var_qbe_dn13_slot: &mut f64,
        var_qbe_dn15_slot: &mut f64,
        var_qbe_dn16_slot: &mut f64,
        var_qbe_dn17_slot: &mut f64,
        var_qbe_dn18_slot: &mut f64,
        var_qbe_dn2_slot: &mut f64,
        var_qbe_dn6_slot: &mut f64,
        var_qbe_dn7_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn0_slot: &mut f64,
        var_qbs_dn10_slot: &mut f64,
        var_qbs_dn11_slot: &mut f64,
        var_qbs_dn12_slot: &mut f64,
        var_qbs_dn17_slot: &mut f64,
        var_qbs_dn2_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn0_slot: &mut f64,
        var_qd_dn10_slot: &mut f64,
        var_qd_dn11_slot: &mut f64,
        var_qd_dn12_slot: &mut f64,
        var_qd_dn13_slot: &mut f64,
        var_qd_dn15_slot: &mut f64,
        var_qd_dn16_slot: &mut f64,
        var_qd_dn17_slot: &mut f64,
        var_qd_dn18_slot: &mut f64,
        var_qd_dn2_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qg_slot: &mut f64,
        var_qg_dn0_slot: &mut f64,
        var_qg_dn10_slot: &mut f64,
        var_qg_dn11_slot: &mut f64,
        var_qg_dn12_slot: &mut f64,
        var_qg_dn13_slot: &mut f64,
        var_qg_dn15_slot: &mut f64,
        var_qg_dn16_slot: &mut f64,
        var_qg_dn17_slot: &mut f64,
        var_qg_dn18_slot: &mut f64,
        var_qg_dn2_slot: &mut f64,
        var_qg_dn6_slot: &mut f64,
        var_qg_dn7_slot: &mut f64,
        var_rpower_slot: &mut f64,
        var_rpower_dn0_slot: &mut f64,
        var_rpower_dn10_slot: &mut f64,
        var_rpower_dn11_slot: &mut f64,
        var_rpower_dn12_slot: &mut f64,
        var_rpower_dn17_slot: &mut f64,
        var_rpower_dn2_slot: &mut f64,
        var_rpower_dn6_slot: &mut f64,
        var_rpower_dn7_slot: &mut f64,
    ) {
        let mut var_cgdbd: f64 = *var_cgdbd_slot;
        let mut var_cgdbd_dn0: f64 = *var_cgdbd_dn0_slot;
        let mut var_cgdbd_dn10: f64 = *var_cgdbd_dn10_slot;
        let mut var_cgdbd_dn11: f64 = *var_cgdbd_dn11_slot;
        let mut var_cgdbd_dn12: f64 = *var_cgdbd_dn12_slot;
        let mut var_cgdbd_dn13: f64 = *var_cgdbd_dn13_slot;
        let mut var_cgdbd_dn15: f64 = *var_cgdbd_dn15_slot;
        let mut var_cgdbd_dn16: f64 = *var_cgdbd_dn16_slot;
        let mut var_cgdbd_dn17: f64 = *var_cgdbd_dn17_slot;
        let mut var_cgdbd_dn18: f64 = *var_cgdbd_dn18_slot;
        let mut var_cgdbd_dn2: f64 = *var_cgdbd_dn2_slot;
        let mut var_cgdbd_dn6: f64 = *var_cgdbd_dn6_slot;
        let mut var_cgdbd_dn7: f64 = *var_cgdbd_dn7_slot;
        let mut var_cthe: f64 = *var_cthe_slot;
        let mut var_gth: f64 = *var_gth_slot;
        let mut var_guard1213: f64 = *var_guard1213_slot;
        let mut var_guard1214: f64 = *var_guard1214_slot;
        let mut var_ibd: f64 = *var_ibd_slot;
        let mut var_ibd_dn0: f64 = *var_ibd_dn0_slot;
        let mut var_ibd_dn10: f64 = *var_ibd_dn10_slot;
        let mut var_ibd_dn11: f64 = *var_ibd_dn11_slot;
        let mut var_ibd_dn12: f64 = *var_ibd_dn12_slot;
        let mut var_ibd_dn17: f64 = *var_ibd_dn17_slot;
        let mut var_ibd_dn2: f64 = *var_ibd_dn2_slot;
        let mut var_ibd_dn6: f64 = *var_ibd_dn6_slot;
        let mut var_ibd_dn7: f64 = *var_ibd_dn7_slot;
        let mut var_ibs: f64 = *var_ibs_slot;
        let mut var_ibs_dn0: f64 = *var_ibs_dn0_slot;
        let mut var_ibs_dn10: f64 = *var_ibs_dn10_slot;
        let mut var_ibs_dn11: f64 = *var_ibs_dn11_slot;
        let mut var_ibs_dn12: f64 = *var_ibs_dn12_slot;
        let mut var_ibs_dn17: f64 = *var_ibs_dn17_slot;
        let mut var_ibs_dn2: f64 = *var_ibs_dn2_slot;
        let mut var_ibs_dn6: f64 = *var_ibs_dn6_slot;
        let mut var_ibs_dn7: f64 = *var_ibs_dn7_slot;
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn0: f64 = *var_ids_dn0_slot;
        let mut var_ids_dn10: f64 = *var_ids_dn10_slot;
        let mut var_ids_dn11: f64 = *var_ids_dn11_slot;
        let mut var_ids_dn12: f64 = *var_ids_dn12_slot;
        let mut var_ids_dn17: f64 = *var_ids_dn17_slot;
        let mut var_ids_dn2: f64 = *var_ids_dn2_slot;
        let mut var_ids_dn6: f64 = *var_ids_dn6_slot;
        let mut var_ids_dn7: f64 = *var_ids_dn7_slot;
        let mut var_idse: f64 = *var_idse_slot;
        let mut var_idse_dn0: f64 = *var_idse_dn0_slot;
        let mut var_idse_dn10: f64 = *var_idse_dn10_slot;
        let mut var_idse_dn11: f64 = *var_idse_dn11_slot;
        let mut var_idse_dn12: f64 = *var_idse_dn12_slot;
        let mut var_idse_dn17: f64 = *var_idse_dn17_slot;
        let mut var_idse_dn2: f64 = *var_idse_dn2_slot;
        let mut var_idse_dn6: f64 = *var_idse_dn6_slot;
        let mut var_idse_dn7: f64 = *var_idse_dn7_slot;
        let mut var_igb: f64 = *var_igb_slot;
        let mut var_igb_dn0: f64 = *var_igb_dn0_slot;
        let mut var_igb_dn10: f64 = *var_igb_dn10_slot;
        let mut var_igb_dn11: f64 = *var_igb_dn11_slot;
        let mut var_igb_dn12: f64 = *var_igb_dn12_slot;
        let mut var_igb_dn17: f64 = *var_igb_dn17_slot;
        let mut var_igb_dn2: f64 = *var_igb_dn2_slot;
        let mut var_igb_dn6: f64 = *var_igb_dn6_slot;
        let mut var_igb_dn7: f64 = *var_igb_dn7_slot;
        let mut var_igd: f64 = *var_igd_slot;
        let mut var_igd_dn0: f64 = *var_igd_dn0_slot;
        let mut var_igd_dn10: f64 = *var_igd_dn10_slot;
        let mut var_igd_dn11: f64 = *var_igd_dn11_slot;
        let mut var_igd_dn12: f64 = *var_igd_dn12_slot;
        let mut var_igd_dn17: f64 = *var_igd_dn17_slot;
        let mut var_igd_dn2: f64 = *var_igd_dn2_slot;
        let mut var_igd_dn6: f64 = *var_igd_dn6_slot;
        let mut var_igd_dn7: f64 = *var_igd_dn7_slot;
        let mut var_igidl: f64 = *var_igidl_slot;
        let mut var_igidl_dn0: f64 = *var_igidl_dn0_slot;
        let mut var_igidl_dn10: f64 = *var_igidl_dn10_slot;
        let mut var_igidl_dn11: f64 = *var_igidl_dn11_slot;
        let mut var_igidl_dn12: f64 = *var_igidl_dn12_slot;
        let mut var_igidl_dn17: f64 = *var_igidl_dn17_slot;
        let mut var_igidl_dn2: f64 = *var_igidl_dn2_slot;
        let mut var_igidl_dn6: f64 = *var_igidl_dn6_slot;
        let mut var_igidl_dn7: f64 = *var_igidl_dn7_slot;
        let mut var_igisl: f64 = *var_igisl_slot;
        let mut var_igisl_dn0: f64 = *var_igisl_dn0_slot;
        let mut var_igisl_dn10: f64 = *var_igisl_dn10_slot;
        let mut var_igisl_dn11: f64 = *var_igisl_dn11_slot;
        let mut var_igisl_dn12: f64 = *var_igisl_dn12_slot;
        let mut var_igisl_dn17: f64 = *var_igisl_dn17_slot;
        let mut var_igisl_dn2: f64 = *var_igisl_dn2_slot;
        let mut var_igisl_dn6: f64 = *var_igisl_dn6_slot;
        let mut var_igisl_dn7: f64 = *var_igisl_dn7_slot;
        let mut var_igs: f64 = *var_igs_slot;
        let mut var_igs_dn0: f64 = *var_igs_dn0_slot;
        let mut var_igs_dn10: f64 = *var_igs_dn10_slot;
        let mut var_igs_dn11: f64 = *var_igs_dn11_slot;
        let mut var_igs_dn12: f64 = *var_igs_dn12_slot;
        let mut var_igs_dn17: f64 = *var_igs_dn17_slot;
        let mut var_igs_dn2: f64 = *var_igs_dn2_slot;
        let mut var_igs_dn6: f64 = *var_igs_dn6_slot;
        let mut var_igs_dn7: f64 = *var_igs_dn7_slot;
        let mut var_isub: f64 = *var_isub_slot;
        let mut var_isub_dn0: f64 = *var_isub_dn0_slot;
        let mut var_isub_dn10: f64 = *var_isub_dn10_slot;
        let mut var_isub_dn11: f64 = *var_isub_dn11_slot;
        let mut var_isub_dn12: f64 = *var_isub_dn12_slot;
        let mut var_isub_dn17: f64 = *var_isub_dn17_slot;
        let mut var_isub_dn2: f64 = *var_isub_dn2_slot;
        let mut var_isub_dn6: f64 = *var_isub_dn6_slot;
        let mut var_isub_dn7: f64 = *var_isub_dn7_slot;
        let mut var_isubs: f64 = *var_isubs_slot;
        let mut var_isubs_dn0: f64 = *var_isubs_dn0_slot;
        let mut var_isubs_dn10: f64 = *var_isubs_dn10_slot;
        let mut var_isubs_dn11: f64 = *var_isubs_dn11_slot;
        let mut var_isubs_dn12: f64 = *var_isubs_dn12_slot;
        let mut var_isubs_dn17: f64 = *var_isubs_dn17_slot;
        let mut var_isubs_dn2: f64 = *var_isubs_dn2_slot;
        let mut var_isubs_dn6: f64 = *var_isubs_dn6_slot;
        let mut var_isubs_dn7: f64 = *var_isubs_dn7_slot;
        let mut var_qb: f64 = *var_qb_slot;
        let mut var_qb_dn0: f64 = *var_qb_dn0_slot;
        let mut var_qb_dn10: f64 = *var_qb_dn10_slot;
        let mut var_qb_dn11: f64 = *var_qb_dn11_slot;
        let mut var_qb_dn12: f64 = *var_qb_dn12_slot;
        let mut var_qb_dn13: f64 = *var_qb_dn13_slot;
        let mut var_qb_dn15: f64 = *var_qb_dn15_slot;
        let mut var_qb_dn16: f64 = *var_qb_dn16_slot;
        let mut var_qb_dn17: f64 = *var_qb_dn17_slot;
        let mut var_qb_dn18: f64 = *var_qb_dn18_slot;
        let mut var_qb_dn2: f64 = *var_qb_dn2_slot;
        let mut var_qb_dn6: f64 = *var_qb_dn6_slot;
        let mut var_qb_dn7: f64 = *var_qb_dn7_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn0: f64 = *var_qbd_dn0_slot;
        let mut var_qbd_dn10: f64 = *var_qbd_dn10_slot;
        let mut var_qbd_dn11: f64 = *var_qbd_dn11_slot;
        let mut var_qbd_dn12: f64 = *var_qbd_dn12_slot;
        let mut var_qbd_dn17: f64 = *var_qbd_dn17_slot;
        let mut var_qbd_dn2: f64 = *var_qbd_dn2_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbe: f64 = *var_qbe_slot;
        let mut var_qbe_dn0: f64 = *var_qbe_dn0_slot;
        let mut var_qbe_dn10: f64 = *var_qbe_dn10_slot;
        let mut var_qbe_dn11: f64 = *var_qbe_dn11_slot;
        let mut var_qbe_dn12: f64 = *var_qbe_dn12_slot;
        let mut var_qbe_dn13: f64 = *var_qbe_dn13_slot;
        let mut var_qbe_dn15: f64 = *var_qbe_dn15_slot;
        let mut var_qbe_dn16: f64 = *var_qbe_dn16_slot;
        let mut var_qbe_dn17: f64 = *var_qbe_dn17_slot;
        let mut var_qbe_dn18: f64 = *var_qbe_dn18_slot;
        let mut var_qbe_dn2: f64 = *var_qbe_dn2_slot;
        let mut var_qbe_dn6: f64 = *var_qbe_dn6_slot;
        let mut var_qbe_dn7: f64 = *var_qbe_dn7_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn0: f64 = *var_qbs_dn0_slot;
        let mut var_qbs_dn10: f64 = *var_qbs_dn10_slot;
        let mut var_qbs_dn11: f64 = *var_qbs_dn11_slot;
        let mut var_qbs_dn12: f64 = *var_qbs_dn12_slot;
        let mut var_qbs_dn17: f64 = *var_qbs_dn17_slot;
        let mut var_qbs_dn2: f64 = *var_qbs_dn2_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn0: f64 = *var_qd_dn0_slot;
        let mut var_qd_dn10: f64 = *var_qd_dn10_slot;
        let mut var_qd_dn11: f64 = *var_qd_dn11_slot;
        let mut var_qd_dn12: f64 = *var_qd_dn12_slot;
        let mut var_qd_dn13: f64 = *var_qd_dn13_slot;
        let mut var_qd_dn15: f64 = *var_qd_dn15_slot;
        let mut var_qd_dn16: f64 = *var_qd_dn16_slot;
        let mut var_qd_dn17: f64 = *var_qd_dn17_slot;
        let mut var_qd_dn18: f64 = *var_qd_dn18_slot;
        let mut var_qd_dn2: f64 = *var_qd_dn2_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qg: f64 = *var_qg_slot;
        let mut var_qg_dn0: f64 = *var_qg_dn0_slot;
        let mut var_qg_dn10: f64 = *var_qg_dn10_slot;
        let mut var_qg_dn11: f64 = *var_qg_dn11_slot;
        let mut var_qg_dn12: f64 = *var_qg_dn12_slot;
        let mut var_qg_dn13: f64 = *var_qg_dn13_slot;
        let mut var_qg_dn15: f64 = *var_qg_dn15_slot;
        let mut var_qg_dn16: f64 = *var_qg_dn16_slot;
        let mut var_qg_dn17: f64 = *var_qg_dn17_slot;
        let mut var_qg_dn18: f64 = *var_qg_dn18_slot;
        let mut var_qg_dn2: f64 = *var_qg_dn2_slot;
        let mut var_qg_dn6: f64 = *var_qg_dn6_slot;
        let mut var_qg_dn7: f64 = *var_qg_dn7_slot;
        let mut var_rpower: f64 = *var_rpower_slot;
        let mut var_rpower_dn0: f64 = *var_rpower_dn0_slot;
        let mut var_rpower_dn10: f64 = *var_rpower_dn10_slot;
        let mut var_rpower_dn11: f64 = *var_rpower_dn11_slot;
        let mut var_rpower_dn12: f64 = *var_rpower_dn12_slot;
        let mut var_rpower_dn17: f64 = *var_rpower_dn17_slot;
        let mut var_rpower_dn2: f64 = *var_rpower_dn2_slot;
        let mut var_rpower_dn6: f64 = *var_rpower_dn6_slot;
        let mut var_rpower_dn7: f64 = *var_rpower_dn7_slot;

        let (assign36690_e51325, assign36690_e51325_d_n0, assign36690_e51325_d_n2, assign36690_e51325_d_n6, assign36690_e51325_d_n7, assign36690_e51325_d_n10, assign36690_e51325_d_n11, assign36690_e51325_d_n12, assign36690_e51325_d_n17,) = {
    if (var_guard1212 != 0.0) {
        (var_idse, var_idse_dn0, var_idse_dn2, var_idse_dn6, var_idse_dn7, var_idse_dn10, var_idse_dn11, var_idse_dn12, var_idse_dn17,)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn6, var_ids_dn7, var_ids_dn10, var_ids_dn11, var_ids_dn12, var_ids_dn17,)
    }
};
        var_ids = assign36690_e51325;
        var_ids_dn0 = assign36690_e51325_d_n0;
        var_ids_dn2 = assign36690_e51325_d_n2;
        var_ids_dn6 = assign36690_e51325_d_n6;
        var_ids_dn7 = assign36690_e51325_d_n7;
        var_ids_dn10 = assign36690_e51325_d_n10;
        var_ids_dn11 = assign36690_e51325_d_n11;
        var_ids_dn12 = assign36690_e51325_d_n12;
        var_ids_dn17 = assign36690_e51325_d_n17;

        let (assign36700_e51329, assign36700_e51329_d_n0, assign36700_e51329_d_n2, assign36700_e51329_d_n6, assign36700_e51329_d_n7, assign36700_e51329_d_n10, assign36700_e51329_d_n11, assign36700_e51329_d_n12, assign36700_e51329_d_n17,) = {
    if (var_guard1212 != 0.0) {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn6, var_isube_dn7, var_isube_dn10, var_isube_dn11, var_isube_dn12, var_isube_dn17,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn6, var_isub_dn7, var_isub_dn10, var_isub_dn11, var_isub_dn12, var_isub_dn17,)
    }
};
        var_isub = assign36700_e51329;
        var_isub_dn0 = assign36700_e51329_d_n0;
        var_isub_dn2 = assign36700_e51329_d_n2;
        var_isub_dn6 = assign36700_e51329_d_n6;
        var_isub_dn7 = assign36700_e51329_d_n7;
        var_isub_dn10 = assign36700_e51329_d_n10;
        var_isub_dn11 = assign36700_e51329_d_n11;
        var_isub_dn12 = assign36700_e51329_d_n12;
        var_isub_dn17 = assign36700_e51329_d_n17;

        let (assign36710_e51333, assign36710_e51333_d_n0, assign36710_e51333_d_n2, assign36710_e51333_d_n6, assign36710_e51333_d_n7, assign36710_e51333_d_n10, assign36710_e51333_d_n11, assign36710_e51333_d_n12, assign36710_e51333_d_n17,) = {
    if (var_guard1212 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isubs, var_isubs_dn0, var_isubs_dn2, var_isubs_dn6, var_isubs_dn7, var_isubs_dn10, var_isubs_dn11, var_isubs_dn12, var_isubs_dn17,)
    }
};
        var_isubs = assign36710_e51333;
        var_isubs_dn0 = assign36710_e51333_d_n0;
        var_isubs_dn2 = assign36710_e51333_d_n2;
        var_isubs_dn6 = assign36710_e51333_d_n6;
        var_isubs_dn7 = assign36710_e51333_d_n7;
        var_isubs_dn10 = assign36710_e51333_d_n10;
        var_isubs_dn11 = assign36710_e51333_d_n11;
        var_isubs_dn12 = assign36710_e51333_d_n12;
        var_isubs_dn17 = assign36710_e51333_d_n17;

        let (assign36720_e51339, assign36720_e51339_d_n0, assign36720_e51339_d_n2, assign36720_e51339_d_n6, assign36720_e51339_d_n7, assign36720_e51339_d_n10, assign36720_e51339_d_n11, assign36720_e51339_d_n12, assign36720_e51339_d_n13, assign36720_e51339_d_n15, assign36720_e51339_d_n16, assign36720_e51339_d_n17, assign36720_e51339_d_n18,) = {
    if (var_guard1212 != 0.0) {
        let assign36720_e51337: f64 = (var_qge + var_qg_nqs);
        (assign36720_e51337, (var_qge_dn0 + var_qg_nqs_dn0), (var_qge_dn2 + var_qg_nqs_dn2), (var_qge_dn6 + var_qg_nqs_dn6), (var_qge_dn7 + var_qg_nqs_dn7), (var_qge_dn10 + var_qg_nqs_dn10), (var_qge_dn11 + var_qg_nqs_dn11), (var_qge_dn12 + var_qg_nqs_dn12), (var_qge_dn13 + var_qg_nqs_dn13), (var_qge_dn15 + var_qg_nqs_dn15), (var_qge_dn16 + var_qg_nqs_dn16), (var_qge_dn17 + var_qg_nqs_dn17), (var_qge_dn18 + var_qg_nqs_dn18),)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn6, var_qg_dn7, var_qg_dn10, var_qg_dn11, var_qg_dn12, var_qg_dn13, var_qg_dn15, var_qg_dn16, var_qg_dn17, var_qg_dn18,)
    }
};
        var_qg = assign36720_e51339;
        var_qg_dn0 = assign36720_e51339_d_n0;
        var_qg_dn2 = assign36720_e51339_d_n2;
        var_qg_dn6 = assign36720_e51339_d_n6;
        var_qg_dn7 = assign36720_e51339_d_n7;
        var_qg_dn10 = assign36720_e51339_d_n10;
        var_qg_dn11 = assign36720_e51339_d_n11;
        var_qg_dn12 = assign36720_e51339_d_n12;
        var_qg_dn13 = assign36720_e51339_d_n13;
        var_qg_dn15 = assign36720_e51339_d_n15;
        var_qg_dn16 = assign36720_e51339_d_n16;
        var_qg_dn17 = assign36720_e51339_d_n17;
        var_qg_dn18 = assign36720_e51339_d_n18;

        let (assign36730_e51345, assign36730_e51345_d_n0, assign36730_e51345_d_n2, assign36730_e51345_d_n6, assign36730_e51345_d_n7, assign36730_e51345_d_n10, assign36730_e51345_d_n11, assign36730_e51345_d_n12, assign36730_e51345_d_n13, assign36730_e51345_d_n15, assign36730_e51345_d_n16, assign36730_e51345_d_n17, assign36730_e51345_d_n18,) = {
    if (var_guard1212 != 0.0) {
        let assign36730_e51343: f64 = (var_qde + var_qd_nqs);
        (assign36730_e51343, (var_qde_dn0 + var_qd_nqs_dn0), (var_qde_dn2 + var_qd_nqs_dn2), (var_qde_dn6 + var_qd_nqs_dn6), (var_qde_dn7 + var_qd_nqs_dn7), (var_qde_dn10 + var_qd_nqs_dn10), (var_qde_dn11 + var_qd_nqs_dn11), (var_qde_dn12 + var_qd_nqs_dn12), var_qde_dn13, (var_qde_dn15 + var_qd_nqs_dn15), var_qde_dn16, (var_qde_dn17 + var_qd_nqs_dn17), (var_qde_dn18 + var_qd_nqs_dn18),)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn6, var_qd_dn7, var_qd_dn10, var_qd_dn11, var_qd_dn12, var_qd_dn13, var_qd_dn15, var_qd_dn16, var_qd_dn17, var_qd_dn18,)
    }
};
        var_qd = assign36730_e51345;
        var_qd_dn0 = assign36730_e51345_d_n0;
        var_qd_dn2 = assign36730_e51345_d_n2;
        var_qd_dn6 = assign36730_e51345_d_n6;
        var_qd_dn7 = assign36730_e51345_d_n7;
        var_qd_dn10 = assign36730_e51345_d_n10;
        var_qd_dn11 = assign36730_e51345_d_n11;
        var_qd_dn12 = assign36730_e51345_d_n12;
        var_qd_dn13 = assign36730_e51345_d_n13;
        var_qd_dn15 = assign36730_e51345_d_n15;
        var_qd_dn16 = assign36730_e51345_d_n16;
        var_qd_dn17 = assign36730_e51345_d_n17;
        var_qd_dn18 = assign36730_e51345_d_n18;

        let (assign36750_e51360, assign36750_e51360_d_n0, assign36750_e51360_d_n2, assign36750_e51360_d_n6, assign36750_e51360_d_n7, assign36750_e51360_d_n10, assign36750_e51360_d_n11, assign36750_e51360_d_n12, assign36750_e51360_d_n13, assign36750_e51360_d_n15, assign36750_e51360_d_n16, assign36750_e51360_d_n17, assign36750_e51360_d_n18,) = {
    if (var_guard1212 != 0.0) {
        let assign36750_e51355: f64 = (var_qge + var_qde);
        let assign36750_e51357: f64 = (assign36750_e51355 + var_qse);
        let assign36750_e51358: f64 = (-assign36750_e51357);
        (assign36750_e51358, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn7 + var_qde_dn7) + var_qse_dn7)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn11 + var_qde_dn11) + var_qse_dn11)), (-((var_qge_dn12 + var_qde_dn12) + var_qse_dn12)), (-((var_qge_dn13 + var_qde_dn13) + var_qse_dn13)), (-((var_qge_dn15 + var_qde_dn15) + var_qse_dn15)), (-((var_qge_dn16 + var_qde_dn16) + var_qse_dn16)), (-((var_qge_dn17 + var_qde_dn17) + var_qse_dn17)), (-((var_qge_dn18 + var_qde_dn18) + var_qse_dn18)),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_dn13, var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    }
};
        var_qbe = assign36750_e51360;
        var_qbe_dn0 = assign36750_e51360_d_n0;
        var_qbe_dn2 = assign36750_e51360_d_n2;
        var_qbe_dn6 = assign36750_e51360_d_n6;
        var_qbe_dn7 = assign36750_e51360_d_n7;
        var_qbe_dn10 = assign36750_e51360_d_n10;
        var_qbe_dn11 = assign36750_e51360_d_n11;
        var_qbe_dn12 = assign36750_e51360_d_n12;
        var_qbe_dn13 = assign36750_e51360_d_n13;
        var_qbe_dn15 = assign36750_e51360_d_n15;
        var_qbe_dn16 = assign36750_e51360_d_n16;
        var_qbe_dn17 = assign36750_e51360_d_n17;
        var_qbe_dn18 = assign36750_e51360_d_n18;

        let (assign36760_e51366, assign36760_e51366_d_n0, assign36760_e51366_d_n2, assign36760_e51366_d_n6, assign36760_e51366_d_n7, assign36760_e51366_d_n10, assign36760_e51366_d_n11, assign36760_e51366_d_n12, assign36760_e51366_d_n13, assign36760_e51366_d_n15, assign36760_e51366_d_n16, assign36760_e51366_d_n17, assign36760_e51366_d_n18,) = {
    if (var_guard1212 != 0.0) {
        let assign36760_e51364: f64 = (var_qbe + var_qb_nqs);
        (assign36760_e51364, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, (var_qbe_dn13 + var_qb_nqs_dn13), var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn6, var_qb_dn7, var_qb_dn10, var_qb_dn11, var_qb_dn12, var_qb_dn13, var_qb_dn15, var_qb_dn16, var_qb_dn17, var_qb_dn18,)
    }
};
        var_qb = assign36760_e51366;
        var_qb_dn0 = assign36760_e51366_d_n0;
        var_qb_dn2 = assign36760_e51366_d_n2;
        var_qb_dn6 = assign36760_e51366_d_n6;
        var_qb_dn7 = assign36760_e51366_d_n7;
        var_qb_dn10 = assign36760_e51366_d_n10;
        var_qb_dn11 = assign36760_e51366_d_n11;
        var_qb_dn12 = assign36760_e51366_d_n12;
        var_qb_dn13 = assign36760_e51366_d_n13;
        var_qb_dn15 = assign36760_e51366_d_n15;
        var_qb_dn16 = assign36760_e51366_d_n16;
        var_qb_dn17 = assign36760_e51366_d_n17;
        var_qb_dn18 = assign36760_e51366_d_n18;

        let (assign36770_e51372, assign36770_e51372_d_n0, assign36770_e51372_d_n2, assign36770_e51372_d_n6, assign36770_e51372_d_n7, assign36770_e51372_d_n10, assign36770_e51372_d_n11, assign36770_e51372_d_n12, assign36770_e51372_d_n17,) = {
    if (var_guard1212 == 0.0) {
        let assign36770_e51370: f64 = (-var_idse);
        (assign36770_e51370, (-var_idse_dn0), (-var_idse_dn2), (-var_idse_dn6), (-var_idse_dn7), (-var_idse_dn10), (-var_idse_dn11), (-var_idse_dn12), (-var_idse_dn17),)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn6, var_ids_dn7, var_ids_dn10, var_ids_dn11, var_ids_dn12, var_ids_dn17,)
    }
};
        var_ids = assign36770_e51372;
        var_ids_dn0 = assign36770_e51372_d_n0;
        var_ids_dn2 = assign36770_e51372_d_n2;
        var_ids_dn6 = assign36770_e51372_d_n6;
        var_ids_dn7 = assign36770_e51372_d_n7;
        var_ids_dn10 = assign36770_e51372_d_n10;
        var_ids_dn11 = assign36770_e51372_d_n11;
        var_ids_dn12 = assign36770_e51372_d_n12;
        var_ids_dn17 = assign36770_e51372_d_n17;

        let (assign36780_e51377, assign36780_e51377_d_n0, assign36780_e51377_d_n2, assign36780_e51377_d_n6, assign36780_e51377_d_n7, assign36780_e51377_d_n10, assign36780_e51377_d_n11, assign36780_e51377_d_n12, assign36780_e51377_d_n17,) = {
    if (var_guard1212 == 0.0) {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn6, var_isube_dn7, var_isube_dn10, var_isube_dn11, var_isube_dn12, var_isube_dn17,)
    } else {
        (var_isubs, var_isubs_dn0, var_isubs_dn2, var_isubs_dn6, var_isubs_dn7, var_isubs_dn10, var_isubs_dn11, var_isubs_dn12, var_isubs_dn17,)
    }
};
        var_isubs = assign36780_e51377;
        var_isubs_dn0 = assign36780_e51377_d_n0;
        var_isubs_dn2 = assign36780_e51377_d_n2;
        var_isubs_dn6 = assign36780_e51377_d_n6;
        var_isubs_dn7 = assign36780_e51377_d_n7;
        var_isubs_dn10 = assign36780_e51377_d_n10;
        var_isubs_dn11 = assign36780_e51377_d_n11;
        var_isubs_dn12 = assign36780_e51377_d_n12;
        var_isubs_dn17 = assign36780_e51377_d_n17;

        let (assign36790_e51382, assign36790_e51382_d_n0, assign36790_e51382_d_n2, assign36790_e51382_d_n6, assign36790_e51382_d_n7, assign36790_e51382_d_n10, assign36790_e51382_d_n11, assign36790_e51382_d_n12, assign36790_e51382_d_n17,) = {
    if (var_guard1212 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn6, var_isub_dn7, var_isub_dn10, var_isub_dn11, var_isub_dn12, var_isub_dn17,)
    }
};
        var_isub = assign36790_e51382;
        var_isub_dn0 = assign36790_e51382_d_n0;
        var_isub_dn2 = assign36790_e51382_d_n2;
        var_isub_dn6 = assign36790_e51382_d_n6;
        var_isub_dn7 = assign36790_e51382_d_n7;
        var_isub_dn10 = assign36790_e51382_d_n10;
        var_isub_dn11 = assign36790_e51382_d_n11;
        var_isub_dn12 = assign36790_e51382_d_n12;
        var_isub_dn17 = assign36790_e51382_d_n17;

        let (assign36800_e51389, assign36800_e51389_d_n0, assign36800_e51389_d_n2, assign36800_e51389_d_n6, assign36800_e51389_d_n7, assign36800_e51389_d_n10, assign36800_e51389_d_n11, assign36800_e51389_d_n12, assign36800_e51389_d_n13, assign36800_e51389_d_n15, assign36800_e51389_d_n16, assign36800_e51389_d_n17, assign36800_e51389_d_n18,) = {
    if (var_guard1212 == 0.0) {
        let assign36800_e51387: f64 = (var_qge + var_qg_nqs);
        (assign36800_e51387, (var_qge_dn0 + var_qg_nqs_dn0), (var_qge_dn2 + var_qg_nqs_dn2), (var_qge_dn6 + var_qg_nqs_dn6), (var_qge_dn7 + var_qg_nqs_dn7), (var_qge_dn10 + var_qg_nqs_dn10), (var_qge_dn11 + var_qg_nqs_dn11), (var_qge_dn12 + var_qg_nqs_dn12), (var_qge_dn13 + var_qg_nqs_dn13), (var_qge_dn15 + var_qg_nqs_dn15), (var_qge_dn16 + var_qg_nqs_dn16), (var_qge_dn17 + var_qg_nqs_dn17), (var_qge_dn18 + var_qg_nqs_dn18),)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn6, var_qg_dn7, var_qg_dn10, var_qg_dn11, var_qg_dn12, var_qg_dn13, var_qg_dn15, var_qg_dn16, var_qg_dn17, var_qg_dn18,)
    }
};
        var_qg = assign36800_e51389;
        var_qg_dn0 = assign36800_e51389_d_n0;
        var_qg_dn2 = assign36800_e51389_d_n2;
        var_qg_dn6 = assign36800_e51389_d_n6;
        var_qg_dn7 = assign36800_e51389_d_n7;
        var_qg_dn10 = assign36800_e51389_d_n10;
        var_qg_dn11 = assign36800_e51389_d_n11;
        var_qg_dn12 = assign36800_e51389_d_n12;
        var_qg_dn13 = assign36800_e51389_d_n13;
        var_qg_dn15 = assign36800_e51389_d_n15;
        var_qg_dn16 = assign36800_e51389_d_n16;
        var_qg_dn17 = assign36800_e51389_d_n17;
        var_qg_dn18 = assign36800_e51389_d_n18;

        let (assign36810_e51396, assign36810_e51396_d_n0, assign36810_e51396_d_n2, assign36810_e51396_d_n6, assign36810_e51396_d_n7, assign36810_e51396_d_n10, assign36810_e51396_d_n11, assign36810_e51396_d_n12, assign36810_e51396_d_n13, assign36810_e51396_d_n15, assign36810_e51396_d_n16, assign36810_e51396_d_n17, assign36810_e51396_d_n18,) = {
    if (var_guard1212 == 0.0) {
        let assign36810_e51394: f64 = (var_qse + var_qs_nqs);
        (assign36810_e51394, (var_qse_dn0 + var_qs_nqs_dn0), (var_qse_dn2 + var_qs_nqs_dn2), (var_qse_dn6 + var_qs_nqs_dn6), (var_qse_dn7 + var_qs_nqs_dn7), (var_qse_dn10 + var_qs_nqs_dn10), (var_qse_dn11 + var_qs_nqs_dn11), (var_qse_dn12 + var_qs_nqs_dn12), var_qse_dn13, var_qse_dn15, (var_qse_dn16 + var_qs_nqs_dn16), (var_qse_dn17 + var_qs_nqs_dn17), (var_qse_dn18 + var_qs_nqs_dn18),)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn6, var_qd_dn7, var_qd_dn10, var_qd_dn11, var_qd_dn12, var_qd_dn13, var_qd_dn15, var_qd_dn16, var_qd_dn17, var_qd_dn18,)
    }
};
        var_qd = assign36810_e51396;
        var_qd_dn0 = assign36810_e51396_d_n0;
        var_qd_dn2 = assign36810_e51396_d_n2;
        var_qd_dn6 = assign36810_e51396_d_n6;
        var_qd_dn7 = assign36810_e51396_d_n7;
        var_qd_dn10 = assign36810_e51396_d_n10;
        var_qd_dn11 = assign36810_e51396_d_n11;
        var_qd_dn12 = assign36810_e51396_d_n12;
        var_qd_dn13 = assign36810_e51396_d_n13;
        var_qd_dn15 = assign36810_e51396_d_n15;
        var_qd_dn16 = assign36810_e51396_d_n16;
        var_qd_dn17 = assign36810_e51396_d_n17;
        var_qd_dn18 = assign36810_e51396_d_n18;

        let (assign36830_e51413, assign36830_e51413_d_n0, assign36830_e51413_d_n2, assign36830_e51413_d_n6, assign36830_e51413_d_n7, assign36830_e51413_d_n10, assign36830_e51413_d_n11, assign36830_e51413_d_n12, assign36830_e51413_d_n13, assign36830_e51413_d_n15, assign36830_e51413_d_n16, assign36830_e51413_d_n17, assign36830_e51413_d_n18,) = {
    if (var_guard1212 == 0.0) {
        let assign36830_e51408: f64 = (var_qge + var_qde);
        let assign36830_e51410: f64 = (assign36830_e51408 + var_qse);
        let assign36830_e51411: f64 = (-assign36830_e51410);
        (assign36830_e51411, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn7 + var_qde_dn7) + var_qse_dn7)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn11 + var_qde_dn11) + var_qse_dn11)), (-((var_qge_dn12 + var_qde_dn12) + var_qse_dn12)), (-((var_qge_dn13 + var_qde_dn13) + var_qse_dn13)), (-((var_qge_dn15 + var_qde_dn15) + var_qse_dn15)), (-((var_qge_dn16 + var_qde_dn16) + var_qse_dn16)), (-((var_qge_dn17 + var_qde_dn17) + var_qse_dn17)), (-((var_qge_dn18 + var_qde_dn18) + var_qse_dn18)),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_dn13, var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    }
};
        var_qbe = assign36830_e51413;
        var_qbe_dn0 = assign36830_e51413_d_n0;
        var_qbe_dn2 = assign36830_e51413_d_n2;
        var_qbe_dn6 = assign36830_e51413_d_n6;
        var_qbe_dn7 = assign36830_e51413_d_n7;
        var_qbe_dn10 = assign36830_e51413_d_n10;
        var_qbe_dn11 = assign36830_e51413_d_n11;
        var_qbe_dn12 = assign36830_e51413_d_n12;
        var_qbe_dn13 = assign36830_e51413_d_n13;
        var_qbe_dn15 = assign36830_e51413_d_n15;
        var_qbe_dn16 = assign36830_e51413_d_n16;
        var_qbe_dn17 = assign36830_e51413_d_n17;
        var_qbe_dn18 = assign36830_e51413_d_n18;

        let (assign36840_e51420, assign36840_e51420_d_n0, assign36840_e51420_d_n2, assign36840_e51420_d_n6, assign36840_e51420_d_n7, assign36840_e51420_d_n10, assign36840_e51420_d_n11, assign36840_e51420_d_n12, assign36840_e51420_d_n13, assign36840_e51420_d_n15, assign36840_e51420_d_n16, assign36840_e51420_d_n17, assign36840_e51420_d_n18,) = {
    if (var_guard1212 == 0.0) {
        let assign36840_e51418: f64 = (var_qbe + var_qb_nqs);
        (assign36840_e51418, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, (var_qbe_dn13 + var_qb_nqs_dn13), var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn6, var_qb_dn7, var_qb_dn10, var_qb_dn11, var_qb_dn12, var_qb_dn13, var_qb_dn15, var_qb_dn16, var_qb_dn17, var_qb_dn18,)
    }
};
        var_qb = assign36840_e51420;
        var_qb_dn0 = assign36840_e51420_d_n0;
        var_qb_dn2 = assign36840_e51420_d_n2;
        var_qb_dn6 = assign36840_e51420_d_n6;
        var_qb_dn7 = assign36840_e51420_d_n7;
        var_qb_dn10 = assign36840_e51420_d_n10;
        var_qb_dn11 = assign36840_e51420_d_n11;
        var_qb_dn12 = assign36840_e51420_d_n12;
        var_qb_dn13 = assign36840_e51420_d_n13;
        var_qb_dn15 = assign36840_e51420_d_n15;
        var_qb_dn16 = assign36840_e51420_d_n16;
        var_qb_dn17 = assign36840_e51420_d_n17;
        var_qb_dn18 = assign36840_e51420_d_n18;

        var_igd = var_igde;
        var_igd_dn0 = var_igde_dn0;
        var_igd_dn2 = var_igde_dn2;
        var_igd_dn6 = var_igde_dn6;
        var_igd_dn7 = var_igde_dn7;
        var_igd_dn10 = var_igde_dn10;
        var_igd_dn11 = var_igde_dn11;
        var_igd_dn12 = var_igde_dn12;
        var_igd_dn17 = var_igde_dn17;

        var_igs = var_igse;
        var_igs_dn0 = var_igse_dn0;
        var_igs_dn2 = var_igse_dn2;
        var_igs_dn6 = var_igse_dn6;
        var_igs_dn7 = var_igse_dn7;
        var_igs_dn10 = var_igse_dn10;
        var_igs_dn11 = var_igse_dn11;
        var_igs_dn12 = var_igse_dn12;
        var_igs_dn17 = var_igse_dn17;

        var_igb = var_igbe;
        var_igb_dn0 = var_igbe_dn0;
        var_igb_dn2 = var_igbe_dn2;
        var_igb_dn6 = var_igbe_dn6;
        var_igb_dn7 = var_igbe_dn7;
        var_igb_dn10 = var_igbe_dn10;
        var_igb_dn11 = var_igbe_dn11;
        var_igb_dn12 = var_igbe_dn12;
        var_igb_dn17 = var_igbe_dn17;

        var_igidl = var_igidle;
        var_igidl_dn0 = var_igidle_dn0;
        var_igidl_dn2 = var_igidle_dn2;
        var_igidl_dn6 = var_igidle_dn6;
        var_igidl_dn7 = var_igidle_dn7;
        var_igidl_dn10 = var_igidle_dn10;
        var_igidl_dn11 = var_igidle_dn11;
        var_igidl_dn12 = var_igidle_dn12;
        var_igidl_dn17 = var_igidle_dn17;

        var_igisl = var_igisle;
        var_igisl_dn0 = var_igisle_dn0;
        var_igisl_dn2 = var_igisle_dn2;
        var_igisl_dn6 = var_igisle_dn6;
        var_igisl_dn7 = var_igisle_dn7;
        var_igisl_dn10 = var_igisle_dn10;
        var_igisl_dn11 = var_igisle_dn11;
        var_igisl_dn12 = var_igisle_dn12;
        var_igisl_dn17 = var_igisle_dn17;

        let assign36900_e51428: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1213 = assign36900_e51428;

        let (assign36910_e51432, assign36910_e51432_d_n0, assign36910_e51432_d_n2, assign36910_e51432_d_n6, assign36910_e51432_d_n7, assign36910_e51432_d_n10, assign36910_e51432_d_n11, assign36910_e51432_d_n12, assign36910_e51432_d_n17,) = {
    if (var_guard1213 != 0.0) {
        (var_ibdb, var_ibdb_dn0, var_ibdb_dn2, var_ibdb_dn6, var_ibdb_dn7, var_ibdb_dn10, var_ibdb_dn11, var_ibdb_dn12, var_ibdb_dn17,)
    } else {
        (var_ibd, var_ibd_dn0, var_ibd_dn2, var_ibd_dn6, var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, var_ibd_dn12, var_ibd_dn17,)
    }
};
        var_ibd = assign36910_e51432;
        var_ibd_dn0 = assign36910_e51432_d_n0;
        var_ibd_dn2 = assign36910_e51432_d_n2;
        var_ibd_dn6 = assign36910_e51432_d_n6;
        var_ibd_dn7 = assign36910_e51432_d_n7;
        var_ibd_dn10 = assign36910_e51432_d_n10;
        var_ibd_dn11 = assign36910_e51432_d_n11;
        var_ibd_dn12 = assign36910_e51432_d_n12;
        var_ibd_dn17 = assign36910_e51432_d_n17;

        let (assign36920_e51436, assign36920_e51436_d_n0, assign36920_e51436_d_n2, assign36920_e51436_d_n6, assign36920_e51436_d_n7, assign36920_e51436_d_n10, assign36920_e51436_d_n11, assign36920_e51436_d_n12, assign36920_e51436_d_n17,) = {
    if (var_guard1213 != 0.0) {
        (var_qbd_s0, var_qbd_s0_dn0, var_qbd_s0_dn2, var_qbd_s0_dn6, var_qbd_s0_dn7, var_qbd_s0_dn10, var_qbd_s0_dn11, var_qbd_s0_dn12, var_qbd_s0_dn17,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign36920_e51436;
        var_qbd_dn0 = assign36920_e51436_d_n0;
        var_qbd_dn2 = assign36920_e51436_d_n2;
        var_qbd_dn6 = assign36920_e51436_d_n6;
        var_qbd_dn7 = assign36920_e51436_d_n7;
        var_qbd_dn10 = assign36920_e51436_d_n10;
        var_qbd_dn11 = assign36920_e51436_d_n11;
        var_qbd_dn12 = assign36920_e51436_d_n12;
        var_qbd_dn17 = assign36920_e51436_d_n17;

        let (assign36930_e51440, assign36930_e51440_d_n0, assign36930_e51440_d_n2, assign36930_e51440_d_n6, assign36930_e51440_d_n7, assign36930_e51440_d_n10, assign36930_e51440_d_n11, assign36930_e51440_d_n12, assign36930_e51440_d_n17,) = {
    if (var_guard1213 != 0.0) {
        (var_ibsb, var_ibsb_dn0, var_ibsb_dn2, var_ibsb_dn6, var_ibsb_dn7, var_ibsb_dn10, var_ibsb_dn11, var_ibsb_dn12, var_ibsb_dn17,)
    } else {
        (var_ibs, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, var_ibs_dn7, var_ibs_dn10, var_ibs_dn11, var_ibs_dn12, var_ibs_dn17,)
    }
};
        var_ibs = assign36930_e51440;
        var_ibs_dn0 = assign36930_e51440_d_n0;
        var_ibs_dn2 = assign36930_e51440_d_n2;
        var_ibs_dn6 = assign36930_e51440_d_n6;
        var_ibs_dn7 = assign36930_e51440_d_n7;
        var_ibs_dn10 = assign36930_e51440_d_n10;
        var_ibs_dn11 = assign36930_e51440_d_n11;
        var_ibs_dn12 = assign36930_e51440_d_n12;
        var_ibs_dn17 = assign36930_e51440_d_n17;

        let (assign36940_e51444, assign36940_e51444_d_n0, assign36940_e51444_d_n2, assign36940_e51444_d_n6, assign36940_e51444_d_n7, assign36940_e51444_d_n10, assign36940_e51444_d_n11, assign36940_e51444_d_n12, assign36940_e51444_d_n17,) = {
    if (var_guard1213 != 0.0) {
        (var_qbs_s0, var_qbs_s0_dn0, var_qbs_s0_dn2, var_qbs_s0_dn6, var_qbs_s0_dn7, var_qbs_s0_dn10, var_qbs_s0_dn11, var_qbs_s0_dn12, var_qbs_s0_dn17,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign36940_e51444;
        var_qbs_dn0 = assign36940_e51444_d_n0;
        var_qbs_dn2 = assign36940_e51444_d_n2;
        var_qbs_dn6 = assign36940_e51444_d_n6;
        var_qbs_dn7 = assign36940_e51444_d_n7;
        var_qbs_dn10 = assign36940_e51444_d_n10;
        var_qbs_dn11 = assign36940_e51444_d_n11;
        var_qbs_dn12 = assign36940_e51444_d_n12;
        var_qbs_dn17 = assign36940_e51444_d_n17;

        let assign36950_e51451: f64 = if ((p.p38 == 1.0) && (var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1214 = assign36950_e51451;

        let (assign36960_e51457, assign36960_e51457_d_n0, assign36960_e51457_d_n2, assign36960_e51457_d_n6, assign36960_e51457_d_n7, assign36960_e51457_d_n10, assign36960_e51457_d_n11, assign36960_e51457_d_n12, assign36960_e51457_d_n17,) = {
    if (var_guard1214 != 0.0) {
        let assign36960_e51455: f64 = (var_ids * var_vds);
        (assign36960_e51455, ((var_ids_dn0 * var_vds) + (var_ids * var_vds_dn0)), ((var_ids_dn2 * var_vds) + (var_ids * var_vds_dn2)), ((var_ids_dn6 * var_vds) + (var_ids * var_vds_dn6)), ((var_ids_dn7 * var_vds) + (var_ids * var_vds_dn7)), ((var_ids_dn10 * var_vds) + (var_ids * var_vds_dn10)), ((var_ids_dn11 * var_vds) + (var_ids * var_vds_dn11)), ((var_ids_dn12 * var_vds) + (var_ids * var_vds_dn12)), ((var_ids_dn17 * var_vds) + (var_ids * var_vds_dn17)),)
    } else {
        (var_rpower, var_rpower_dn0, var_rpower_dn2, var_rpower_dn6, var_rpower_dn7, var_rpower_dn10, var_rpower_dn11, var_rpower_dn12, var_rpower_dn17,)
    }
};
        var_rpower = assign36960_e51457;
        var_rpower_dn0 = assign36960_e51457_d_n0;
        var_rpower_dn2 = assign36960_e51457_d_n2;
        var_rpower_dn6 = assign36960_e51457_d_n6;
        var_rpower_dn7 = assign36960_e51457_d_n7;
        var_rpower_dn10 = assign36960_e51457_d_n10;
        var_rpower_dn11 = assign36960_e51457_d_n11;
        var_rpower_dn12 = assign36960_e51457_d_n12;
        var_rpower_dn17 = assign36960_e51457_d_n17;

        let (assign36970_e51461,) = {
    if (var_guard1214 != 0.0) {
        (var_cth,)
    } else {
        (var_cthe,)
    }
};
        var_cthe = assign36970_e51461;

        let (assign36980_e51467,) = {
    if (var_guard1214 != 0.0) {
        let assign36980_e51465: f64 = (1.0 / var_rth);
        (assign36980_e51465,)
    } else {
        (var_gth,)
    }
};
        var_gth = assign36980_e51467;

        let (assign36990_e51472, assign36990_e51472_d_n0, assign36990_e51472_d_n2, assign36990_e51472_d_n6, assign36990_e51472_d_n7, assign36990_e51472_d_n10, assign36990_e51472_d_n11, assign36990_e51472_d_n12, assign36990_e51472_d_n17,) = {
    if (var_guard1214 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rpower, var_rpower_dn0, var_rpower_dn2, var_rpower_dn6, var_rpower_dn7, var_rpower_dn10, var_rpower_dn11, var_rpower_dn12, var_rpower_dn17,)
    }
};
        var_rpower = assign36990_e51472;
        var_rpower_dn0 = assign36990_e51472_d_n0;
        var_rpower_dn2 = assign36990_e51472_d_n2;
        var_rpower_dn6 = assign36990_e51472_d_n6;
        var_rpower_dn7 = assign36990_e51472_d_n7;
        var_rpower_dn10 = assign36990_e51472_d_n10;
        var_rpower_dn11 = assign36990_e51472_d_n11;
        var_rpower_dn12 = assign36990_e51472_d_n12;
        var_rpower_dn17 = assign36990_e51472_d_n17;

        let (assign37000_e51477,) = {
    if (var_guard1214 == 0.0) {
        (0.0,)
    } else {
        (var_cthe,)
    }
};
        var_cthe = assign37000_e51477;

        let (assign37010_e51482,) = {
    if (var_guard1214 == 0.0) {
        (0.0,)
    } else {
        (var_gth,)
    }
};
        var_gth = assign37010_e51482;

        var_idse = var_ids;
        var_idse_dn0 = var_ids_dn0;
        var_idse_dn2 = var_ids_dn2;
        var_idse_dn6 = var_ids_dn6;
        var_idse_dn7 = var_ids_dn7;
        var_idse_dn10 = var_ids_dn10;
        var_idse_dn11 = var_ids_dn11;
        var_idse_dn12 = var_ids_dn12;
        var_idse_dn17 = var_ids_dn17;

        let assign37170_e51531: f64 = var_qg_dn6;
        var_cgdbd = assign37170_e51531;
        var_cgdbd_dn0 = 0.0;
        var_cgdbd_dn2 = 0.0;
        var_cgdbd_dn6 = 0.0;
        var_cgdbd_dn7 = 0.0;
        var_cgdbd_dn10 = 0.0;
        var_cgdbd_dn11 = 0.0;
        var_cgdbd_dn12 = 0.0;
        var_cgdbd_dn13 = 0.0;
        var_cgdbd_dn15 = 0.0;
        var_cgdbd_dn16 = 0.0;
        var_cgdbd_dn17 = 0.0;
        var_cgdbd_dn18 = 0.0;

        *var_cgdbd_slot = var_cgdbd;
        *var_cgdbd_dn0_slot = var_cgdbd_dn0;
        *var_cgdbd_dn10_slot = var_cgdbd_dn10;
        *var_cgdbd_dn11_slot = var_cgdbd_dn11;
        *var_cgdbd_dn12_slot = var_cgdbd_dn12;
        *var_cgdbd_dn13_slot = var_cgdbd_dn13;
        *var_cgdbd_dn15_slot = var_cgdbd_dn15;
        *var_cgdbd_dn16_slot = var_cgdbd_dn16;
        *var_cgdbd_dn17_slot = var_cgdbd_dn17;
        *var_cgdbd_dn18_slot = var_cgdbd_dn18;
        *var_cgdbd_dn2_slot = var_cgdbd_dn2;
        *var_cgdbd_dn6_slot = var_cgdbd_dn6;
        *var_cgdbd_dn7_slot = var_cgdbd_dn7;
        *var_cthe_slot = var_cthe;
        *var_gth_slot = var_gth;
        *var_guard1213_slot = var_guard1213;
        *var_guard1214_slot = var_guard1214;
        *var_ibd_slot = var_ibd;
        *var_ibd_dn0_slot = var_ibd_dn0;
        *var_ibd_dn10_slot = var_ibd_dn10;
        *var_ibd_dn11_slot = var_ibd_dn11;
        *var_ibd_dn12_slot = var_ibd_dn12;
        *var_ibd_dn17_slot = var_ibd_dn17;
        *var_ibd_dn2_slot = var_ibd_dn2;
        *var_ibd_dn6_slot = var_ibd_dn6;
        *var_ibd_dn7_slot = var_ibd_dn7;
        *var_ibs_slot = var_ibs;
        *var_ibs_dn0_slot = var_ibs_dn0;
        *var_ibs_dn10_slot = var_ibs_dn10;
        *var_ibs_dn11_slot = var_ibs_dn11;
        *var_ibs_dn12_slot = var_ibs_dn12;
        *var_ibs_dn17_slot = var_ibs_dn17;
        *var_ibs_dn2_slot = var_ibs_dn2;
        *var_ibs_dn6_slot = var_ibs_dn6;
        *var_ibs_dn7_slot = var_ibs_dn7;
        *var_ids_slot = var_ids;
        *var_ids_dn0_slot = var_ids_dn0;
        *var_ids_dn10_slot = var_ids_dn10;
        *var_ids_dn11_slot = var_ids_dn11;
        *var_ids_dn12_slot = var_ids_dn12;
        *var_ids_dn17_slot = var_ids_dn17;
        *var_ids_dn2_slot = var_ids_dn2;
        *var_ids_dn6_slot = var_ids_dn6;
        *var_ids_dn7_slot = var_ids_dn7;
        *var_idse_slot = var_idse;
        *var_idse_dn0_slot = var_idse_dn0;
        *var_idse_dn10_slot = var_idse_dn10;
        *var_idse_dn11_slot = var_idse_dn11;
        *var_idse_dn12_slot = var_idse_dn12;
        *var_idse_dn17_slot = var_idse_dn17;
        *var_idse_dn2_slot = var_idse_dn2;
        *var_idse_dn6_slot = var_idse_dn6;
        *var_idse_dn7_slot = var_idse_dn7;
        *var_igb_slot = var_igb;
        *var_igb_dn0_slot = var_igb_dn0;
        *var_igb_dn10_slot = var_igb_dn10;
        *var_igb_dn11_slot = var_igb_dn11;
        *var_igb_dn12_slot = var_igb_dn12;
        *var_igb_dn17_slot = var_igb_dn17;
        *var_igb_dn2_slot = var_igb_dn2;
        *var_igb_dn6_slot = var_igb_dn6;
        *var_igb_dn7_slot = var_igb_dn7;
        *var_igd_slot = var_igd;
        *var_igd_dn0_slot = var_igd_dn0;
        *var_igd_dn10_slot = var_igd_dn10;
        *var_igd_dn11_slot = var_igd_dn11;
        *var_igd_dn12_slot = var_igd_dn12;
        *var_igd_dn17_slot = var_igd_dn17;
        *var_igd_dn2_slot = var_igd_dn2;
        *var_igd_dn6_slot = var_igd_dn6;
        *var_igd_dn7_slot = var_igd_dn7;
        *var_igidl_slot = var_igidl;
        *var_igidl_dn0_slot = var_igidl_dn0;
        *var_igidl_dn10_slot = var_igidl_dn10;
        *var_igidl_dn11_slot = var_igidl_dn11;
        *var_igidl_dn12_slot = var_igidl_dn12;
        *var_igidl_dn17_slot = var_igidl_dn17;
        *var_igidl_dn2_slot = var_igidl_dn2;
        *var_igidl_dn6_slot = var_igidl_dn6;
        *var_igidl_dn7_slot = var_igidl_dn7;
        *var_igisl_slot = var_igisl;
        *var_igisl_dn0_slot = var_igisl_dn0;
        *var_igisl_dn10_slot = var_igisl_dn10;
        *var_igisl_dn11_slot = var_igisl_dn11;
        *var_igisl_dn12_slot = var_igisl_dn12;
        *var_igisl_dn17_slot = var_igisl_dn17;
        *var_igisl_dn2_slot = var_igisl_dn2;
        *var_igisl_dn6_slot = var_igisl_dn6;
        *var_igisl_dn7_slot = var_igisl_dn7;
        *var_igs_slot = var_igs;
        *var_igs_dn0_slot = var_igs_dn0;
        *var_igs_dn10_slot = var_igs_dn10;
        *var_igs_dn11_slot = var_igs_dn11;
        *var_igs_dn12_slot = var_igs_dn12;
        *var_igs_dn17_slot = var_igs_dn17;
        *var_igs_dn2_slot = var_igs_dn2;
        *var_igs_dn6_slot = var_igs_dn6;
        *var_igs_dn7_slot = var_igs_dn7;
        *var_isub_slot = var_isub;
        *var_isub_dn0_slot = var_isub_dn0;
        *var_isub_dn10_slot = var_isub_dn10;
        *var_isub_dn11_slot = var_isub_dn11;
        *var_isub_dn12_slot = var_isub_dn12;
        *var_isub_dn17_slot = var_isub_dn17;
        *var_isub_dn2_slot = var_isub_dn2;
        *var_isub_dn6_slot = var_isub_dn6;
        *var_isub_dn7_slot = var_isub_dn7;
        *var_isubs_slot = var_isubs;
        *var_isubs_dn0_slot = var_isubs_dn0;
        *var_isubs_dn10_slot = var_isubs_dn10;
        *var_isubs_dn11_slot = var_isubs_dn11;
        *var_isubs_dn12_slot = var_isubs_dn12;
        *var_isubs_dn17_slot = var_isubs_dn17;
        *var_isubs_dn2_slot = var_isubs_dn2;
        *var_isubs_dn6_slot = var_isubs_dn6;
        *var_isubs_dn7_slot = var_isubs_dn7;
        *var_qb_slot = var_qb;
        *var_qb_dn0_slot = var_qb_dn0;
        *var_qb_dn10_slot = var_qb_dn10;
        *var_qb_dn11_slot = var_qb_dn11;
        *var_qb_dn12_slot = var_qb_dn12;
        *var_qb_dn13_slot = var_qb_dn13;
        *var_qb_dn15_slot = var_qb_dn15;
        *var_qb_dn16_slot = var_qb_dn16;
        *var_qb_dn17_slot = var_qb_dn17;
        *var_qb_dn18_slot = var_qb_dn18;
        *var_qb_dn2_slot = var_qb_dn2;
        *var_qb_dn6_slot = var_qb_dn6;
        *var_qb_dn7_slot = var_qb_dn7;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn0_slot = var_qbd_dn0;
        *var_qbd_dn10_slot = var_qbd_dn10;
        *var_qbd_dn11_slot = var_qbd_dn11;
        *var_qbd_dn12_slot = var_qbd_dn12;
        *var_qbd_dn17_slot = var_qbd_dn17;
        *var_qbd_dn2_slot = var_qbd_dn2;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbe_slot = var_qbe;
        *var_qbe_dn0_slot = var_qbe_dn0;
        *var_qbe_dn10_slot = var_qbe_dn10;
        *var_qbe_dn11_slot = var_qbe_dn11;
        *var_qbe_dn12_slot = var_qbe_dn12;
        *var_qbe_dn13_slot = var_qbe_dn13;
        *var_qbe_dn15_slot = var_qbe_dn15;
        *var_qbe_dn16_slot = var_qbe_dn16;
        *var_qbe_dn17_slot = var_qbe_dn17;
        *var_qbe_dn18_slot = var_qbe_dn18;
        *var_qbe_dn2_slot = var_qbe_dn2;
        *var_qbe_dn6_slot = var_qbe_dn6;
        *var_qbe_dn7_slot = var_qbe_dn7;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn0_slot = var_qbs_dn0;
        *var_qbs_dn10_slot = var_qbs_dn10;
        *var_qbs_dn11_slot = var_qbs_dn11;
        *var_qbs_dn12_slot = var_qbs_dn12;
        *var_qbs_dn17_slot = var_qbs_dn17;
        *var_qbs_dn2_slot = var_qbs_dn2;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_qd_slot = var_qd;
        *var_qd_dn0_slot = var_qd_dn0;
        *var_qd_dn10_slot = var_qd_dn10;
        *var_qd_dn11_slot = var_qd_dn11;
        *var_qd_dn12_slot = var_qd_dn12;
        *var_qd_dn13_slot = var_qd_dn13;
        *var_qd_dn15_slot = var_qd_dn15;
        *var_qd_dn16_slot = var_qd_dn16;
        *var_qd_dn17_slot = var_qd_dn17;
        *var_qd_dn18_slot = var_qd_dn18;
        *var_qd_dn2_slot = var_qd_dn2;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qg_slot = var_qg;
        *var_qg_dn0_slot = var_qg_dn0;
        *var_qg_dn10_slot = var_qg_dn10;
        *var_qg_dn11_slot = var_qg_dn11;
        *var_qg_dn12_slot = var_qg_dn12;
        *var_qg_dn13_slot = var_qg_dn13;
        *var_qg_dn15_slot = var_qg_dn15;
        *var_qg_dn16_slot = var_qg_dn16;
        *var_qg_dn17_slot = var_qg_dn17;
        *var_qg_dn18_slot = var_qg_dn18;
        *var_qg_dn2_slot = var_qg_dn2;
        *var_qg_dn6_slot = var_qg_dn6;
        *var_qg_dn7_slot = var_qg_dn7;
        *var_rpower_slot = var_rpower;
        *var_rpower_dn0_slot = var_rpower_dn0;
        *var_rpower_dn10_slot = var_rpower_dn10;
        *var_rpower_dn11_slot = var_rpower_dn11;
        *var_rpower_dn12_slot = var_rpower_dn12;
        *var_rpower_dn17_slot = var_rpower_dn17;
        *var_rpower_dn2_slot = var_rpower_dn2;
        *var_rpower_dn6_slot = var_rpower_dn6;
        *var_rpower_dn7_slot = var_rpower_dn7;
    }
}
