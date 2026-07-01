#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_112(
        p: &Parameters,
        var_cgdo_given: f64,
        var_cgso_given: f64,
        var_chi__blk947: f64,
        var_chi__blk947_dn0: f64,
        var_chi__blk947_dn10: f64,
        var_chi__blk947_dn11: f64,
        var_chi__blk947_dn12: f64,
        var_chi__blk947_dn17: f64,
        var_chi__blk947_dn2: f64,
        var_chi__blk947_dn6: f64,
        var_chi__blk947_dn7: f64,
        var_cnst0over__blk932: f64,
        var_cnst0over__blk932_dn0: f64,
        var_cnst0over__blk932_dn10: f64,
        var_cnst0over__blk932_dn11: f64,
        var_cnst0over__blk932_dn12: f64,
        var_cnst0over__blk932_dn17: f64,
        var_cnst0over__blk932_dn2: f64,
        var_cnst0over__blk932_dn6: f64,
        var_cnst0over__blk932_dn7: f64,
        var_fb__blk971: f64,
        var_fb__blk971_dn0: f64,
        var_fb__blk971_dn10: f64,
        var_fb__blk971_dn11: f64,
        var_fb__blk971_dn12: f64,
        var_fb__blk971_dn17: f64,
        var_fb__blk971_dn2: f64,
        var_fb__blk971_dn6: f64,
        var_fb__blk971_dn7: f64,
        var_flg_overd__blk919: f64,
        var_flg_overs__blk918: f64,
        var_flg_ovloopd__blk917: f64,
        var_flg_ovloops__blk916: f64,
        var_fs01__blk969: f64,
        var_fs01__blk969_dn0: f64,
        var_fs01__blk969_dn10: f64,
        var_fs01__blk969_dn11: f64,
        var_fs01__blk969_dn12: f64,
        var_fs01__blk969_dn17: f64,
        var_fs01__blk969_dn2: f64,
        var_fs01__blk969_dn6: f64,
        var_fs01__blk969_dn7: f64,
        var_fs02__blk973: f64,
        var_fs02__blk973_dn0: f64,
        var_fs02__blk973_dn10: f64,
        var_fs02__blk973_dn11: f64,
        var_fs02__blk973_dn12: f64,
        var_fs02__blk973_dn17: f64,
        var_fs02__blk973_dn2: f64,
        var_fs02__blk973_dn6: f64,
        var_fs02__blk973_dn7: f64,
        var_guard1006: f64,
        var_guard1013: f64,
        var_guard982: f64,
        var_guard983: f64,
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
        var_guard1020_slot: &mut f64,
        var_guard1022_slot: &mut f64,
        var_guard1023_slot: &mut f64,
        var_guard1024_slot: &mut f64,
        var_guard1025_slot: &mut f64,
        var_guard1026_slot: &mut f64,
        var_guard1027_slot: &mut f64,
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
        var_t1__blk900_slot: &mut f64,
        var_t1__blk900_dn0_slot: &mut f64,
        var_t1__blk900_dn10_slot: &mut f64,
        var_t1__blk900_dn11_slot: &mut f64,
        var_t1__blk900_dn12_slot: &mut f64,
        var_t1__blk900_dn17_slot: &mut f64,
        var_t1__blk900_dn2_slot: &mut f64,
        var_t1__blk900_dn6_slot: &mut f64,
        var_t1__blk900_dn7_slot: &mut f64,
        var_t4__blk903_slot: &mut f64,
        var_t4__blk903_dn0_slot: &mut f64,
        var_t4__blk903_dn10_slot: &mut f64,
        var_t4__blk903_dn11_slot: &mut f64,
        var_t4__blk903_dn12_slot: &mut f64,
        var_t4__blk903_dn17_slot: &mut f64,
        var_t4__blk903_dn2_slot: &mut f64,
        var_t4__blk903_dn6_slot: &mut f64,
        var_t4__blk903_dn7_slot: &mut f64,
        var_xi0__blk980_slot: &mut f64,
        var_xi0__blk980_dn0_slot: &mut f64,
        var_xi0__blk980_dn10_slot: &mut f64,
        var_xi0__blk980_dn11_slot: &mut f64,
        var_xi0__blk980_dn12_slot: &mut f64,
        var_xi0__blk980_dn17_slot: &mut f64,
        var_xi0__blk980_dn2_slot: &mut f64,
        var_xi0__blk980_dn6_slot: &mut f64,
        var_xi0__blk980_dn7_slot: &mut f64,
        var_xi0p12__blk981_slot: &mut f64,
        var_xi0p12__blk981_dn0_slot: &mut f64,
        var_xi0p12__blk981_dn10_slot: &mut f64,
        var_xi0p12__blk981_dn11_slot: &mut f64,
        var_xi0p12__blk981_dn12_slot: &mut f64,
        var_xi0p12__blk981_dn17_slot: &mut f64,
        var_xi0p12__blk981_dn2_slot: &mut f64,
        var_xi0p12__blk981_dn6_slot: &mut f64,
        var_xi0p12__blk981_dn7_slot: &mut f64,
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
        let mut var_guard1020: f64 = *var_guard1020_slot;
        let mut var_guard1022: f64 = *var_guard1022_slot;
        let mut var_guard1023: f64 = *var_guard1023_slot;
        let mut var_guard1024: f64 = *var_guard1024_slot;
        let mut var_guard1025: f64 = *var_guard1025_slot;
        let mut var_guard1026: f64 = *var_guard1026_slot;
        let mut var_guard1027: f64 = *var_guard1027_slot;
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
        let mut var_t1__blk900: f64 = *var_t1__blk900_slot;
        let mut var_t1__blk900_dn0: f64 = *var_t1__blk900_dn0_slot;
        let mut var_t1__blk900_dn10: f64 = *var_t1__blk900_dn10_slot;
        let mut var_t1__blk900_dn11: f64 = *var_t1__blk900_dn11_slot;
        let mut var_t1__blk900_dn12: f64 = *var_t1__blk900_dn12_slot;
        let mut var_t1__blk900_dn17: f64 = *var_t1__blk900_dn17_slot;
        let mut var_t1__blk900_dn2: f64 = *var_t1__blk900_dn2_slot;
        let mut var_t1__blk900_dn6: f64 = *var_t1__blk900_dn6_slot;
        let mut var_t1__blk900_dn7: f64 = *var_t1__blk900_dn7_slot;
        let mut var_t4__blk903: f64 = *var_t4__blk903_slot;
        let mut var_t4__blk903_dn0: f64 = *var_t4__blk903_dn0_slot;
        let mut var_t4__blk903_dn10: f64 = *var_t4__blk903_dn10_slot;
        let mut var_t4__blk903_dn11: f64 = *var_t4__blk903_dn11_slot;
        let mut var_t4__blk903_dn12: f64 = *var_t4__blk903_dn12_slot;
        let mut var_t4__blk903_dn17: f64 = *var_t4__blk903_dn17_slot;
        let mut var_t4__blk903_dn2: f64 = *var_t4__blk903_dn2_slot;
        let mut var_t4__blk903_dn6: f64 = *var_t4__blk903_dn6_slot;
        let mut var_t4__blk903_dn7: f64 = *var_t4__blk903_dn7_slot;
        let mut var_xi0__blk980: f64 = *var_xi0__blk980_slot;
        let mut var_xi0__blk980_dn0: f64 = *var_xi0__blk980_dn0_slot;
        let mut var_xi0__blk980_dn10: f64 = *var_xi0__blk980_dn10_slot;
        let mut var_xi0__blk980_dn11: f64 = *var_xi0__blk980_dn11_slot;
        let mut var_xi0__blk980_dn12: f64 = *var_xi0__blk980_dn12_slot;
        let mut var_xi0__blk980_dn17: f64 = *var_xi0__blk980_dn17_slot;
        let mut var_xi0__blk980_dn2: f64 = *var_xi0__blk980_dn2_slot;
        let mut var_xi0__blk980_dn6: f64 = *var_xi0__blk980_dn6_slot;
        let mut var_xi0__blk980_dn7: f64 = *var_xi0__blk980_dn7_slot;
        let mut var_xi0p12__blk981: f64 = *var_xi0p12__blk981_slot;
        let mut var_xi0p12__blk981_dn0: f64 = *var_xi0p12__blk981_dn0_slot;
        let mut var_xi0p12__blk981_dn10: f64 = *var_xi0p12__blk981_dn10_slot;
        let mut var_xi0p12__blk981_dn11: f64 = *var_xi0p12__blk981_dn11_slot;
        let mut var_xi0p12__blk981_dn12: f64 = *var_xi0p12__blk981_dn12_slot;
        let mut var_xi0p12__blk981_dn17: f64 = *var_xi0p12__blk981_dn17_slot;
        let mut var_xi0p12__blk981_dn2: f64 = *var_xi0p12__blk981_dn2_slot;
        let mut var_xi0p12__blk981_dn6: f64 = *var_xi0p12__blk981_dn6_slot;
        let mut var_xi0p12__blk981_dn7: f64 = *var_xi0p12__blk981_dn7_slot;

        let assign31410_e46198: f64 = if var_chi__blk947 < 5.0 { 1.0 } else { 0.0 };
        var_guard1020 = assign31410_e46198;

        let (assign31450_e46260, assign31450_e46260_d_n0, assign31450_e46260_d_n2, assign31450_e46260_d_n6, assign31450_e46260_d_n7, assign31450_e46260_d_n10, assign31450_e46260_d_n11, assign31450_e46260_d_n12, assign31450_e46260_d_n17,) = {
    if ((((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) && (var_guard1006 == 0.0)) && (var_guard1013 != 0.0)) && (var_guard1020 != 0.0)) {
        let assign31450_e46254: f64 = (var_fb__blk971 * var_fb__blk971);
        let assign31450_e46257: f64 = (10.0 * 2.220446049250313e-16);
        let assign31450_e46258: f64 = (assign31450_e46254 + assign31450_e46257);
        (assign31450_e46258, ((var_fb__blk971_dn0 * var_fb__blk971) + (var_fb__blk971 * var_fb__blk971_dn0)), ((var_fb__blk971_dn2 * var_fb__blk971) + (var_fb__blk971 * var_fb__blk971_dn2)), ((var_fb__blk971_dn6 * var_fb__blk971) + (var_fb__blk971 * var_fb__blk971_dn6)), ((var_fb__blk971_dn7 * var_fb__blk971) + (var_fb__blk971 * var_fb__blk971_dn7)), ((var_fb__blk971_dn10 * var_fb__blk971) + (var_fb__blk971 * var_fb__blk971_dn10)), ((var_fb__blk971_dn11 * var_fb__blk971) + (var_fb__blk971 * var_fb__blk971_dn11)), ((var_fb__blk971_dn12 * var_fb__blk971) + (var_fb__blk971 * var_fb__blk971_dn12)), ((var_fb__blk971_dn17 * var_fb__blk971) + (var_fb__blk971 * var_fb__blk971_dn17)),)
    } else {
        (var_xi0__blk980, var_xi0__blk980_dn0, var_xi0__blk980_dn2, var_xi0__blk980_dn6, var_xi0__blk980_dn7, var_xi0__blk980_dn10, var_xi0__blk980_dn11, var_xi0__blk980_dn12, var_xi0__blk980_dn17,)
    }
};
        var_xi0__blk980 = assign31450_e46260;
        var_xi0__blk980_dn0 = assign31450_e46260_d_n0;
        var_xi0__blk980_dn2 = assign31450_e46260_d_n2;
        var_xi0__blk980_dn6 = assign31450_e46260_d_n6;
        var_xi0__blk980_dn7 = assign31450_e46260_d_n7;
        var_xi0__blk980_dn10 = assign31450_e46260_d_n10;
        var_xi0__blk980_dn11 = assign31450_e46260_d_n11;
        var_xi0__blk980_dn12 = assign31450_e46260_d_n12;
        var_xi0__blk980_dn17 = assign31450_e46260_d_n17;

        let (assign31460_e46280, assign31460_e46280_d_n0, assign31460_e46280_d_n2, assign31460_e46280_d_n6, assign31460_e46280_d_n7, assign31460_e46280_d_n10, assign31460_e46280_d_n11, assign31460_e46280_d_n12, assign31460_e46280_d_n17,) = {
    if ((((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) && (var_guard1006 == 0.0)) && (var_guard1013 != 0.0)) && (var_guard1020 != 0.0)) {
        let assign31460_e46277: f64 = (10.0 * 2.220446049250313e-16);
        let assign31460_e46278: f64 = (var_fb__blk971 + assign31460_e46277);
        (assign31460_e46278, var_fb__blk971_dn0, var_fb__blk971_dn2, var_fb__blk971_dn6, var_fb__blk971_dn7, var_fb__blk971_dn10, var_fb__blk971_dn11, var_fb__blk971_dn12, var_fb__blk971_dn17,)
    } else {
        (var_xi0p12__blk981, var_xi0p12__blk981_dn0, var_xi0p12__blk981_dn2, var_xi0p12__blk981_dn6, var_xi0p12__blk981_dn7, var_xi0p12__blk981_dn10, var_xi0p12__blk981_dn11, var_xi0p12__blk981_dn12, var_xi0p12__blk981_dn17,)
    }
};
        var_xi0p12__blk981 = assign31460_e46280;
        var_xi0p12__blk981_dn0 = assign31460_e46280_d_n0;
        var_xi0p12__blk981_dn2 = assign31460_e46280_d_n2;
        var_xi0p12__blk981_dn6 = assign31460_e46280_d_n6;
        var_xi0p12__blk981_dn7 = assign31460_e46280_d_n7;
        var_xi0p12__blk981_dn10 = assign31460_e46280_d_n10;
        var_xi0p12__blk981_dn11 = assign31460_e46280_d_n11;
        var_xi0p12__blk981_dn12 = assign31460_e46280_d_n12;
        var_xi0p12__blk981_dn17 = assign31460_e46280_d_n17;

        let (assign31480_e46316, assign31480_e46316_d_n0, assign31480_e46316_d_n2, assign31480_e46316_d_n6, assign31480_e46316_d_n7, assign31480_e46316_d_n10, assign31480_e46316_d_n11, assign31480_e46316_d_n12, assign31480_e46316_d_n17,) = {
    if ((((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) && (var_guard1006 == 0.0)) && (var_guard1013 != 0.0)) && (var_guard1020 == 0.0)) {
        let assign31480_e46314: f64 = (var_chi__blk947 - 1.0);
        (assign31480_e46314, var_chi__blk947_dn0, var_chi__blk947_dn2, var_chi__blk947_dn6, var_chi__blk947_dn7, var_chi__blk947_dn10, var_chi__blk947_dn11, var_chi__blk947_dn12, var_chi__blk947_dn17,)
    } else {
        (var_xi0__blk980, var_xi0__blk980_dn0, var_xi0__blk980_dn2, var_xi0__blk980_dn6, var_xi0__blk980_dn7, var_xi0__blk980_dn10, var_xi0__blk980_dn11, var_xi0__blk980_dn12, var_xi0__blk980_dn17,)
    }
};
        var_xi0__blk980 = assign31480_e46316;
        var_xi0__blk980_dn0 = assign31480_e46316_d_n0;
        var_xi0__blk980_dn2 = assign31480_e46316_d_n2;
        var_xi0__blk980_dn6 = assign31480_e46316_d_n6;
        var_xi0__blk980_dn7 = assign31480_e46316_d_n7;
        var_xi0__blk980_dn10 = assign31480_e46316_d_n10;
        var_xi0__blk980_dn11 = assign31480_e46316_d_n11;
        var_xi0__blk980_dn12 = assign31480_e46316_d_n12;
        var_xi0__blk980_dn17 = assign31480_e46316_d_n17;

        let (assign31490_e46334, assign31490_e46334_d_n0, assign31490_e46334_d_n2, assign31490_e46334_d_n6, assign31490_e46334_d_n7, assign31490_e46334_d_n10, assign31490_e46334_d_n11, assign31490_e46334_d_n12, assign31490_e46334_d_n17,) = {
    if ((((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) && (var_guard1006 == 0.0)) && (var_guard1013 != 0.0)) && (var_guard1020 == 0.0)) {
        let assign31490_e46332: f64 = (var_xi0__blk980).sqrt();
        (assign31490_e46332, (var_xi0__blk980_dn0 / (2.0 * assign31490_e46332)), (var_xi0__blk980_dn2 / (2.0 * assign31490_e46332)), (var_xi0__blk980_dn6 / (2.0 * assign31490_e46332)), (var_xi0__blk980_dn7 / (2.0 * assign31490_e46332)), (var_xi0__blk980_dn10 / (2.0 * assign31490_e46332)), (var_xi0__blk980_dn11 / (2.0 * assign31490_e46332)), (var_xi0__blk980_dn12 / (2.0 * assign31490_e46332)), (var_xi0__blk980_dn17 / (2.0 * assign31490_e46332)),)
    } else {
        (var_xi0p12__blk981, var_xi0p12__blk981_dn0, var_xi0p12__blk981_dn2, var_xi0p12__blk981_dn6, var_xi0p12__blk981_dn7, var_xi0p12__blk981_dn10, var_xi0p12__blk981_dn11, var_xi0p12__blk981_dn12, var_xi0p12__blk981_dn17,)
    }
};
        var_xi0p12__blk981 = assign31490_e46334;
        var_xi0p12__blk981_dn0 = assign31490_e46334_d_n0;
        var_xi0p12__blk981_dn2 = assign31490_e46334_d_n2;
        var_xi0p12__blk981_dn6 = assign31490_e46334_d_n6;
        var_xi0p12__blk981_dn7 = assign31490_e46334_d_n7;
        var_xi0p12__blk981_dn10 = assign31490_e46334_d_n10;
        var_xi0p12__blk981_dn11 = assign31490_e46334_d_n11;
        var_xi0p12__blk981_dn12 = assign31490_e46334_d_n12;
        var_xi0p12__blk981_dn17 = assign31490_e46334_d_n17;

        let (assign31500_e46350, assign31500_e46350_d_n0, assign31500_e46350_d_n2, assign31500_e46350_d_n6, assign31500_e46350_d_n7, assign31500_e46350_d_n10, assign31500_e46350_d_n11, assign31500_e46350_d_n12, assign31500_e46350_d_n17,) = {
    if (((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) && (var_guard1006 == 0.0)) && (var_guard1013 != 0.0)) {
        let assign31500_e46348: f64 = (var_cnst0over__blk932 * var_xi0p12__blk981);
        (assign31500_e46348, ((var_cnst0over__blk932_dn0 * var_xi0p12__blk981) + (var_cnst0over__blk932 * var_xi0p12__blk981_dn0)), ((var_cnst0over__blk932_dn2 * var_xi0p12__blk981) + (var_cnst0over__blk932 * var_xi0p12__blk981_dn2)), ((var_cnst0over__blk932_dn6 * var_xi0p12__blk981) + (var_cnst0over__blk932 * var_xi0p12__blk981_dn6)), ((var_cnst0over__blk932_dn7 * var_xi0p12__blk981) + (var_cnst0over__blk932 * var_xi0p12__blk981_dn7)), ((var_cnst0over__blk932_dn10 * var_xi0p12__blk981) + (var_cnst0over__blk932 * var_xi0p12__blk981_dn10)), ((var_cnst0over__blk932_dn11 * var_xi0p12__blk981) + (var_cnst0over__blk932 * var_xi0p12__blk981_dn11)), ((var_cnst0over__blk932_dn12 * var_xi0p12__blk981) + (var_cnst0over__blk932 * var_xi0p12__blk981_dn12)), ((var_cnst0over__blk932_dn17 * var_xi0p12__blk981) + (var_cnst0over__blk932 * var_xi0p12__blk981_dn17)),)
    } else {
        (var_qbuld, var_qbuld_dn0, var_qbuld_dn2, var_qbuld_dn6, var_qbuld_dn7, var_qbuld_dn10, var_qbuld_dn11, var_qbuld_dn12, var_qbuld_dn17,)
    }
};
        var_qbuld = assign31500_e46350;
        var_qbuld_dn0 = assign31500_e46350_d_n0;
        var_qbuld_dn2 = assign31500_e46350_d_n2;
        var_qbuld_dn6 = assign31500_e46350_d_n6;
        var_qbuld_dn7 = assign31500_e46350_d_n7;
        var_qbuld_dn10 = assign31500_e46350_d_n10;
        var_qbuld_dn11 = assign31500_e46350_d_n11;
        var_qbuld_dn12 = assign31500_e46350_d_n12;
        var_qbuld_dn17 = assign31500_e46350_d_n17;

        let (assign31510_e46368, assign31510_e46368_d_n0, assign31510_e46368_d_n2, assign31510_e46368_d_n6, assign31510_e46368_d_n7, assign31510_e46368_d_n10, assign31510_e46368_d_n11, assign31510_e46368_d_n12, assign31510_e46368_d_n17,) = {
    if (((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) && (var_guard1006 == 0.0)) && (var_guard1013 != 0.0)) {
        let assign31510_e46365: f64 = (var_fs02__blk973 + var_xi0p12__blk981);
        let assign31510_e46366: f64 = (1.0 / assign31510_e46365);
        (assign31510_e46366, (-((var_fs02__blk973_dn0 + var_xi0p12__blk981_dn0) / (assign31510_e46365 * assign31510_e46365))), (-((var_fs02__blk973_dn2 + var_xi0p12__blk981_dn2) / (assign31510_e46365 * assign31510_e46365))), (-((var_fs02__blk973_dn6 + var_xi0p12__blk981_dn6) / (assign31510_e46365 * assign31510_e46365))), (-((var_fs02__blk973_dn7 + var_xi0p12__blk981_dn7) / (assign31510_e46365 * assign31510_e46365))), (-((var_fs02__blk973_dn10 + var_xi0p12__blk981_dn10) / (assign31510_e46365 * assign31510_e46365))), (-((var_fs02__blk973_dn11 + var_xi0p12__blk981_dn11) / (assign31510_e46365 * assign31510_e46365))), (-((var_fs02__blk973_dn12 + var_xi0p12__blk981_dn12) / (assign31510_e46365 * assign31510_e46365))), (-((var_fs02__blk973_dn17 + var_xi0p12__blk981_dn17) / (assign31510_e46365 * assign31510_e46365))),)
    } else {
        (var_t1__blk900, var_t1__blk900_dn0, var_t1__blk900_dn2, var_t1__blk900_dn6, var_t1__blk900_dn7, var_t1__blk900_dn10, var_t1__blk900_dn11, var_t1__blk900_dn12, var_t1__blk900_dn17,)
    }
};
        var_t1__blk900 = assign31510_e46368;
        var_t1__blk900_dn0 = assign31510_e46368_d_n0;
        var_t1__blk900_dn2 = assign31510_e46368_d_n2;
        var_t1__blk900_dn6 = assign31510_e46368_d_n6;
        var_t1__blk900_dn7 = assign31510_e46368_d_n7;
        var_t1__blk900_dn10 = assign31510_e46368_d_n10;
        var_t1__blk900_dn11 = assign31510_e46368_d_n11;
        var_t1__blk900_dn12 = assign31510_e46368_d_n12;
        var_t1__blk900_dn17 = assign31510_e46368_d_n17;

        let (assign31520_e46386, assign31520_e46386_d_n0, assign31520_e46386_d_n2, assign31520_e46386_d_n6, assign31520_e46386_d_n7, assign31520_e46386_d_n10, assign31520_e46386_d_n11, assign31520_e46386_d_n12, assign31520_e46386_d_n17,) = {
    if (((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) && (var_guard1006 == 0.0)) && (var_guard1013 != 0.0)) {
        let assign31520_e46382: f64 = (var_cnst0over__blk932 * var_fs01__blk969);
        let assign31520_e46384: f64 = (assign31520_e46382 * var_t1__blk900);
        (assign31520_e46384, ((((var_cnst0over__blk932_dn0 * var_fs01__blk969) + (var_cnst0over__blk932 * var_fs01__blk969_dn0)) * var_t1__blk900) + (assign31520_e46382 * var_t1__blk900_dn0)), ((((var_cnst0over__blk932_dn2 * var_fs01__blk969) + (var_cnst0over__blk932 * var_fs01__blk969_dn2)) * var_t1__blk900) + (assign31520_e46382 * var_t1__blk900_dn2)), ((((var_cnst0over__blk932_dn6 * var_fs01__blk969) + (var_cnst0over__blk932 * var_fs01__blk969_dn6)) * var_t1__blk900) + (assign31520_e46382 * var_t1__blk900_dn6)), ((((var_cnst0over__blk932_dn7 * var_fs01__blk969) + (var_cnst0over__blk932 * var_fs01__blk969_dn7)) * var_t1__blk900) + (assign31520_e46382 * var_t1__blk900_dn7)), ((((var_cnst0over__blk932_dn10 * var_fs01__blk969) + (var_cnst0over__blk932 * var_fs01__blk969_dn10)) * var_t1__blk900) + (assign31520_e46382 * var_t1__blk900_dn10)), ((((var_cnst0over__blk932_dn11 * var_fs01__blk969) + (var_cnst0over__blk932 * var_fs01__blk969_dn11)) * var_t1__blk900) + (assign31520_e46382 * var_t1__blk900_dn11)), ((((var_cnst0over__blk932_dn12 * var_fs01__blk969) + (var_cnst0over__blk932 * var_fs01__blk969_dn12)) * var_t1__blk900) + (assign31520_e46382 * var_t1__blk900_dn12)), ((((var_cnst0over__blk932_dn17 * var_fs01__blk969) + (var_cnst0over__blk932 * var_fs01__blk969_dn17)) * var_t1__blk900) + (assign31520_e46382 * var_t1__blk900_dn17)),)
    } else {
        (var_qiuld, var_qiuld_dn0, var_qiuld_dn2, var_qiuld_dn6, var_qiuld_dn7, var_qiuld_dn10, var_qiuld_dn11, var_qiuld_dn12, var_qiuld_dn17,)
    }
};
        var_qiuld = assign31520_e46386;
        var_qiuld_dn0 = assign31520_e46386_d_n0;
        var_qiuld_dn2 = assign31520_e46386_d_n2;
        var_qiuld_dn6 = assign31520_e46386_d_n6;
        var_qiuld_dn7 = assign31520_e46386_d_n7;
        var_qiuld_dn10 = assign31520_e46386_d_n10;
        var_qiuld_dn11 = assign31520_e46386_d_n11;
        var_qiuld_dn12 = assign31520_e46386_d_n12;
        var_qiuld_dn17 = assign31520_e46386_d_n17;

        let (assign31530_e46402, assign31530_e46402_d_n0, assign31530_e46402_d_n2, assign31530_e46402_d_n6, assign31530_e46402_d_n7, assign31530_e46402_d_n10, assign31530_e46402_d_n11, assign31530_e46402_d_n12, assign31530_e46402_d_n17,) = {
    if (((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) && (var_guard1006 == 0.0)) && (var_guard1013 != 0.0)) {
        let assign31530_e46400: f64 = (var_qbuld + var_qiuld);
        (assign31530_e46400, (var_qbuld_dn0 + var_qiuld_dn0), (var_qbuld_dn2 + var_qiuld_dn2), (var_qbuld_dn6 + var_qiuld_dn6), (var_qbuld_dn7 + var_qiuld_dn7), (var_qbuld_dn10 + var_qiuld_dn10), (var_qbuld_dn11 + var_qiuld_dn11), (var_qbuld_dn12 + var_qiuld_dn12), (var_qbuld_dn17 + var_qiuld_dn17),)
    } else {
        (var_qsuld, var_qsuld_dn0, var_qsuld_dn2, var_qsuld_dn6, var_qsuld_dn7, var_qsuld_dn10, var_qsuld_dn11, var_qsuld_dn12, var_qsuld_dn17,)
    }
};
        var_qsuld = assign31530_e46402;
        var_qsuld_dn0 = assign31530_e46402_d_n0;
        var_qsuld_dn2 = assign31530_e46402_d_n2;
        var_qsuld_dn6 = assign31530_e46402_d_n6;
        var_qsuld_dn7 = assign31530_e46402_d_n7;
        var_qsuld_dn10 = assign31530_e46402_d_n10;
        var_qsuld_dn11 = assign31530_e46402_d_n11;
        var_qsuld_dn12 = assign31530_e46402_d_n12;
        var_qsuld_dn17 = assign31530_e46402_d_n17;

        let (assign31540_e46413, assign31540_e46413_d_n0, assign31540_e46413_d_n2, assign31540_e46413_d_n6, assign31540_e46413_d_n7, assign31540_e46413_d_n10, assign31540_e46413_d_n11, assign31540_e46413_d_n12, assign31540_e46413_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) {
        let assign31540_e46411: f64 = (var_qsuld - var_qbuld);
        (assign31540_e46411, (var_qsuld_dn0 - var_qbuld_dn0), (var_qsuld_dn2 - var_qbuld_dn2), (var_qsuld_dn6 - var_qbuld_dn6), (var_qsuld_dn7 - var_qbuld_dn7), (var_qsuld_dn10 - var_qbuld_dn10), (var_qsuld_dn11 - var_qbuld_dn11), (var_qsuld_dn12 - var_qbuld_dn12), (var_qsuld_dn17 - var_qbuld_dn17),)
    } else {
        (var_qiuld, var_qiuld_dn0, var_qiuld_dn2, var_qiuld_dn6, var_qiuld_dn7, var_qiuld_dn10, var_qiuld_dn11, var_qiuld_dn12, var_qiuld_dn17,)
    }
};
        var_qiuld = assign31540_e46413;
        var_qiuld_dn0 = assign31540_e46413_d_n0;
        var_qiuld_dn2 = assign31540_e46413_d_n2;
        var_qiuld_dn6 = assign31540_e46413_d_n6;
        var_qiuld_dn7 = assign31540_e46413_d_n7;
        var_qiuld_dn10 = assign31540_e46413_d_n10;
        var_qiuld_dn11 = assign31540_e46413_d_n11;
        var_qiuld_dn12 = assign31540_e46413_d_n12;
        var_qiuld_dn17 = assign31540_e46413_d_n17;

        let (assign31550_e46431, assign31550_e46431_d_n0, assign31550_e46431_d_n2, assign31550_e46431_d_n6, assign31550_e46431_d_n7, assign31550_e46431_d_n10, assign31550_e46431_d_n11, assign31550_e46431_d_n12, assign31550_e46431_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) {
        let (assign31550_e46429,) = {
            if (p.p43 == 1.0) {
                let assign31550_e46425: f64 = (var_w_dioscv * var_lov);
                (assign31550_e46425,)
            } else {
                let assign31550_e46428: f64 = (var_weffcv_nf * var_lov);
                (assign31550_e46428,)
            }
        };
        (assign31550_e46429, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4__blk903, var_t4__blk903_dn0, var_t4__blk903_dn2, var_t4__blk903_dn6, var_t4__blk903_dn7, var_t4__blk903_dn10, var_t4__blk903_dn11, var_t4__blk903_dn12, var_t4__blk903_dn17,)
    }
};
        var_t4__blk903 = assign31550_e46431;
        var_t4__blk903_dn0 = assign31550_e46431_d_n0;
        var_t4__blk903_dn2 = assign31550_e46431_d_n2;
        var_t4__blk903_dn6 = assign31550_e46431_d_n6;
        var_t4__blk903_dn7 = assign31550_e46431_d_n7;
        var_t4__blk903_dn10 = assign31550_e46431_d_n10;
        var_t4__blk903_dn11 = assign31550_e46431_d_n11;
        var_t4__blk903_dn12 = assign31550_e46431_d_n12;
        var_t4__blk903_dn17 = assign31550_e46431_d_n17;

        let assign31560_e46442: f64 = if (((var_flg_overs__blk918 != 0.0) && (p.p43 == 0.0)) || ((var_flg_ovloops__blk916 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        var_guard1022 = assign31560_e46442;

        let (assign31570_e46455, assign31570_e46455_d_n0, assign31570_e46455_d_n2, assign31570_e46455_d_n6, assign31570_e46455_d_n7, assign31570_e46455_d_n10, assign31570_e46455_d_n11, assign31570_e46455_d_n12, assign31570_e46455_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) && (var_guard1022 != 0.0)) {
        let assign31570_e46453: f64 = (var_t4__blk903 * var_qsuld);
        (assign31570_e46453, ((var_t4__blk903_dn0 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn0)), ((var_t4__blk903_dn2 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn2)), ((var_t4__blk903_dn6 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn6)), ((var_t4__blk903_dn7 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn7)), ((var_t4__blk903_dn10 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn10)), ((var_t4__blk903_dn11 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn11)), ((var_t4__blk903_dn12 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn12)), ((var_t4__blk903_dn17 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn17)),)
    } else {
        (var_qovs, var_qovs_dn0, var_qovs_dn2, var_qovs_dn6, var_qovs_dn7, var_qovs_dn10, var_qovs_dn11, var_qovs_dn12, var_qovs_dn17,)
    }
};
        var_qovs = assign31570_e46455;
        var_qovs_dn0 = assign31570_e46455_d_n0;
        var_qovs_dn2 = assign31570_e46455_d_n2;
        var_qovs_dn6 = assign31570_e46455_d_n6;
        var_qovs_dn7 = assign31570_e46455_d_n7;
        var_qovs_dn10 = assign31570_e46455_d_n10;
        var_qovs_dn11 = assign31570_e46455_d_n11;
        var_qovs_dn12 = assign31570_e46455_d_n12;
        var_qovs_dn17 = assign31570_e46455_d_n17;

        let (assign31580_e46468, assign31580_e46468_d_n0, assign31580_e46468_d_n2, assign31580_e46468_d_n6, assign31580_e46468_d_n7, assign31580_e46468_d_n10, assign31580_e46468_d_n11, assign31580_e46468_d_n12, assign31580_e46468_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) && (var_guard1022 != 0.0)) {
        let assign31580_e46466: f64 = (var_t4__blk903 * var_qbuld);
        (assign31580_e46466, ((var_t4__blk903_dn0 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn0)), ((var_t4__blk903_dn2 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn2)), ((var_t4__blk903_dn6 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn6)), ((var_t4__blk903_dn7 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn7)), ((var_t4__blk903_dn10 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn10)), ((var_t4__blk903_dn11 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn11)), ((var_t4__blk903_dn12 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn12)), ((var_t4__blk903_dn17 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn17)),)
    } else {
        (var_qbsld, var_qbsld_dn0, var_qbsld_dn2, var_qbsld_dn6, var_qbsld_dn7, var_qbsld_dn10, var_qbsld_dn11, var_qbsld_dn12, var_qbsld_dn17,)
    }
};
        var_qbsld = assign31580_e46468;
        var_qbsld_dn0 = assign31580_e46468_d_n0;
        var_qbsld_dn2 = assign31580_e46468_d_n2;
        var_qbsld_dn6 = assign31580_e46468_d_n6;
        var_qbsld_dn7 = assign31580_e46468_d_n7;
        var_qbsld_dn10 = assign31580_e46468_d_n10;
        var_qbsld_dn11 = assign31580_e46468_d_n11;
        var_qbsld_dn12 = assign31580_e46468_d_n12;
        var_qbsld_dn17 = assign31580_e46468_d_n17;

        let assign31590_e46479: f64 = if (((var_flg_overd__blk919 != 0.0) && (p.p43 == 0.0)) || ((var_flg_ovloopd__blk917 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        var_guard1023 = assign31590_e46479;

        let (assign31600_e46492, assign31600_e46492_d_n0, assign31600_e46492_d_n2, assign31600_e46492_d_n6, assign31600_e46492_d_n7, assign31600_e46492_d_n10, assign31600_e46492_d_n11, assign31600_e46492_d_n12, assign31600_e46492_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) && (var_guard1023 != 0.0)) {
        let assign31600_e46490: f64 = (var_t4__blk903 * var_qsuld);
        (assign31600_e46490, ((var_t4__blk903_dn0 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn0)), ((var_t4__blk903_dn2 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn2)), ((var_t4__blk903_dn6 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn6)), ((var_t4__blk903_dn7 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn7)), ((var_t4__blk903_dn10 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn10)), ((var_t4__blk903_dn11 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn11)), ((var_t4__blk903_dn12 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn12)), ((var_t4__blk903_dn17 * var_qsuld) + (var_t4__blk903 * var_qsuld_dn17)),)
    } else {
        (var_qovd, var_qovd_dn0, var_qovd_dn2, var_qovd_dn6, var_qovd_dn7, var_qovd_dn10, var_qovd_dn11, var_qovd_dn12, var_qovd_dn17,)
    }
};
        var_qovd = assign31600_e46492;
        var_qovd_dn0 = assign31600_e46492_d_n0;
        var_qovd_dn2 = assign31600_e46492_d_n2;
        var_qovd_dn6 = assign31600_e46492_d_n6;
        var_qovd_dn7 = assign31600_e46492_d_n7;
        var_qovd_dn10 = assign31600_e46492_d_n10;
        var_qovd_dn11 = assign31600_e46492_d_n11;
        var_qovd_dn12 = assign31600_e46492_d_n12;
        var_qovd_dn17 = assign31600_e46492_d_n17;

        let (assign31610_e46505, assign31610_e46505_d_n0, assign31610_e46505_d_n2, assign31610_e46505_d_n6, assign31610_e46505_d_n7, assign31610_e46505_d_n10, assign31610_e46505_d_n11, assign31610_e46505_d_n12, assign31610_e46505_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_guard983 == 0.0)) && (var_guard1023 != 0.0)) {
        let assign31610_e46503: f64 = (var_t4__blk903 * var_qbuld);
        (assign31610_e46503, ((var_t4__blk903_dn0 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn0)), ((var_t4__blk903_dn2 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn2)), ((var_t4__blk903_dn6 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn6)), ((var_t4__blk903_dn7 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn7)), ((var_t4__blk903_dn10 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn10)), ((var_t4__blk903_dn11 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn11)), ((var_t4__blk903_dn12 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn12)), ((var_t4__blk903_dn17 * var_qbuld) + (var_t4__blk903 * var_qbuld_dn17)),)
    } else {
        (var_qbdld, var_qbdld_dn0, var_qbdld_dn2, var_qbdld_dn6, var_qbdld_dn7, var_qbdld_dn10, var_qbdld_dn11, var_qbdld_dn12, var_qbdld_dn17,)
    }
};
        var_qbdld = assign31610_e46505;
        var_qbdld_dn0 = assign31610_e46505_d_n0;
        var_qbdld_dn2 = assign31610_e46505_d_n2;
        var_qbdld_dn6 = assign31610_e46505_d_n6;
        var_qbdld_dn7 = assign31610_e46505_d_n7;
        var_qbdld_dn10 = assign31610_e46505_d_n10;
        var_qbdld_dn11 = assign31610_e46505_d_n11;
        var_qbdld_dn12 = assign31610_e46505_d_n12;
        var_qbdld_dn17 = assign31610_e46505_d_n17;

        let (assign31620_e46517,) = {
    if ((p.p24 != 0.0) && (var_guard982 != 0.0)) {
        let assign31620_e46511: f64 = (var_modervs * var_cgso_given);
        let assign31620_e46514: f64 = (var_modenml * var_cgdo_given);
        let assign31620_e46515: f64 = (assign31620_e46511 + assign31620_e46514);
        (assign31620_e46515,)
    } else {
        (var_flg_overgiven,)
    }
};
        var_flg_overgiven = assign31620_e46517;

        let (assign31630_e46531, assign31630_e46531_d_n0, assign31630_e46531_d_n2, assign31630_e46531_d_n6, assign31630_e46531_d_n7, assign31630_e46531_d_n10, assign31630_e46531_d_n11, assign31630_e46531_d_n12, assign31630_e46531_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_flg_overgiven != 0.0)) {
        let assign31630_e46525: f64 = (var_modervs * p.p170);
        let assign31630_e46528: f64 = (var_modenml * p.p169);
        let assign31630_e46529: f64 = (assign31630_e46525 + assign31630_e46528);
        (assign31630_e46529, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31630_e46531;
        var_cgdoe_dn0 = assign31630_e46531_d_n0;
        var_cgdoe_dn2 = assign31630_e46531_d_n2;
        var_cgdoe_dn6 = assign31630_e46531_d_n6;
        var_cgdoe_dn7 = assign31630_e46531_d_n7;
        var_cgdoe_dn10 = assign31630_e46531_d_n10;
        var_cgdoe_dn11 = assign31630_e46531_d_n11;
        var_cgdoe_dn12 = assign31630_e46531_d_n12;
        var_cgdoe_dn17 = assign31630_e46531_d_n17;

        let assign31640_e46534: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1024 = assign31640_e46534;

        let (assign31650_e46550, assign31650_e46550_d_n0, assign31650_e46550_d_n2, assign31650_e46550_d_n6, assign31650_e46550_d_n7, assign31650_e46550_d_n10, assign31650_e46550_d_n11, assign31650_e46550_d_n12, assign31650_e46550_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_flg_overgiven != 0.0)) && (var_guard1024 != 0.0)) {
        let assign31650_e46544: f64 = (var_modervs * var_w_dioscv);
        let assign31650_e46547: f64 = (var_modenml * var_w_diodcv);
        let assign31650_e46548: f64 = (assign31650_e46544 + assign31650_e46547);
        (assign31650_e46548, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk900, var_t1__blk900_dn0, var_t1__blk900_dn2, var_t1__blk900_dn6, var_t1__blk900_dn7, var_t1__blk900_dn10, var_t1__blk900_dn11, var_t1__blk900_dn12, var_t1__blk900_dn17,)
    }
};
        var_t1__blk900 = assign31650_e46550;
        var_t1__blk900_dn0 = assign31650_e46550_d_n0;
        var_t1__blk900_dn2 = assign31650_e46550_d_n2;
        var_t1__blk900_dn6 = assign31650_e46550_d_n6;
        var_t1__blk900_dn7 = assign31650_e46550_d_n7;
        var_t1__blk900_dn10 = assign31650_e46550_d_n10;
        var_t1__blk900_dn11 = assign31650_e46550_d_n11;
        var_t1__blk900_dn12 = assign31650_e46550_d_n12;
        var_t1__blk900_dn17 = assign31650_e46550_d_n17;

        let (assign31660_e46563, assign31660_e46563_d_n0, assign31660_e46563_d_n2, assign31660_e46563_d_n6, assign31660_e46563_d_n7, assign31660_e46563_d_n10, assign31660_e46563_d_n11, assign31660_e46563_d_n12, assign31660_e46563_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_flg_overgiven != 0.0)) && (var_guard1024 != 0.0)) {
        let assign31660_e46560: f64 = (-var_t1__blk900);
        let assign31660_e46561: f64 = (var_cgdoe * assign31660_e46560);
        (assign31660_e46561, ((var_cgdoe_dn0 * assign31660_e46560) + (var_cgdoe * (-var_t1__blk900_dn0))), ((var_cgdoe_dn2 * assign31660_e46560) + (var_cgdoe * (-var_t1__blk900_dn2))), ((var_cgdoe_dn6 * assign31660_e46560) + (var_cgdoe * (-var_t1__blk900_dn6))), ((var_cgdoe_dn7 * assign31660_e46560) + (var_cgdoe * (-var_t1__blk900_dn7))), ((var_cgdoe_dn10 * assign31660_e46560) + (var_cgdoe * (-var_t1__blk900_dn10))), ((var_cgdoe_dn11 * assign31660_e46560) + (var_cgdoe * (-var_t1__blk900_dn11))), ((var_cgdoe_dn12 * assign31660_e46560) + (var_cgdoe * (-var_t1__blk900_dn12))), ((var_cgdoe_dn17 * assign31660_e46560) + (var_cgdoe * (-var_t1__blk900_dn17))),)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31660_e46563;
        var_cgdoe_dn0 = assign31660_e46563_d_n0;
        var_cgdoe_dn2 = assign31660_e46563_d_n2;
        var_cgdoe_dn6 = assign31660_e46563_d_n6;
        var_cgdoe_dn7 = assign31660_e46563_d_n7;
        var_cgdoe_dn10 = assign31660_e46563_d_n10;
        var_cgdoe_dn11 = assign31660_e46563_d_n11;
        var_cgdoe_dn12 = assign31660_e46563_d_n12;
        var_cgdoe_dn17 = assign31660_e46563_d_n17;

        let (assign31670_e46577, assign31670_e46577_d_n0, assign31670_e46577_d_n2, assign31670_e46577_d_n6, assign31670_e46577_d_n7, assign31670_e46577_d_n10, assign31670_e46577_d_n11, assign31670_e46577_d_n12, assign31670_e46577_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_flg_overgiven != 0.0)) && (var_guard1024 == 0.0)) {
        let assign31670_e46574: f64 = (-var_weffcv_nf);
        let assign31670_e46575: f64 = (var_cgdoe * assign31670_e46574);
        (assign31670_e46575, (var_cgdoe_dn0 * assign31670_e46574), (var_cgdoe_dn2 * assign31670_e46574), (var_cgdoe_dn6 * assign31670_e46574), (var_cgdoe_dn7 * assign31670_e46574), (var_cgdoe_dn10 * assign31670_e46574), (var_cgdoe_dn11 * assign31670_e46574), (var_cgdoe_dn12 * assign31670_e46574), (var_cgdoe_dn17 * assign31670_e46574),)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31670_e46577;
        var_cgdoe_dn0 = assign31670_e46577_d_n0;
        var_cgdoe_dn2 = assign31670_e46577_d_n2;
        var_cgdoe_dn6 = assign31670_e46577_d_n6;
        var_cgdoe_dn7 = assign31670_e46577_d_n7;
        var_cgdoe_dn10 = assign31670_e46577_d_n10;
        var_cgdoe_dn11 = assign31670_e46577_d_n11;
        var_cgdoe_dn12 = assign31670_e46577_d_n12;
        var_cgdoe_dn17 = assign31670_e46577_d_n17;

        let (assign31680_e46592, assign31680_e46592_d_n0, assign31680_e46592_d_n2, assign31680_e46592_d_n6, assign31680_e46592_d_n7, assign31680_e46592_d_n10, assign31680_e46592_d_n11, assign31680_e46592_d_n12, assign31680_e46592_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_flg_overgiven != 0.0)) {
        let assign31680_e46585: f64 = (-var_cgdoe);
        let assign31680_e46588: f64 = (var_vgs - var_vds);
        let assign31680_e46589: f64 = (assign31680_e46585 * assign31680_e46588);
        let assign31680_e46590: f64 = (var_qgod + assign31680_e46589);
        (assign31680_e46590, (var_qgod_dn0 + (((-var_cgdoe_dn0) * assign31680_e46588) + (assign31680_e46585 * (-var_vds_dn0)))), (var_qgod_dn2 + (((-var_cgdoe_dn2) * assign31680_e46588) + (assign31680_e46585 * (-var_vds_dn2)))), (var_qgod_dn6 + (((-var_cgdoe_dn6) * assign31680_e46588) + (assign31680_e46585 * (var_vgs_dn6 - var_vds_dn6)))), (var_qgod_dn7 + (((-var_cgdoe_dn7) * assign31680_e46588) + (assign31680_e46585 * (var_vgs_dn7 - var_vds_dn7)))), (var_qgod_dn10 + (((-var_cgdoe_dn10) * assign31680_e46588) + (assign31680_e46585 * (-var_vds_dn10)))), (var_qgod_dn11 + (((-var_cgdoe_dn11) * assign31680_e46588) + (assign31680_e46585 * (var_vgs_dn11 - var_vds_dn11)))), (var_qgod_dn12 + (((-var_cgdoe_dn12) * assign31680_e46588) + (assign31680_e46585 * (-var_vds_dn12)))), (var_qgod_dn17 + (((-var_cgdoe_dn17) * assign31680_e46588) + (assign31680_e46585 * (-var_vds_dn17)))),)
    } else {
        (var_qgod, var_qgod_dn0, var_qgod_dn2, var_qgod_dn6, var_qgod_dn7, var_qgod_dn10, var_qgod_dn11, var_qgod_dn12, var_qgod_dn17,)
    }
};
        var_qgod = assign31680_e46592;
        var_qgod_dn0 = assign31680_e46592_d_n0;
        var_qgod_dn2 = assign31680_e46592_d_n2;
        var_qgod_dn6 = assign31680_e46592_d_n6;
        var_qgod_dn7 = assign31680_e46592_d_n7;
        var_qgod_dn10 = assign31680_e46592_d_n10;
        var_qgod_dn11 = assign31680_e46592_d_n11;
        var_qgod_dn12 = assign31680_e46592_d_n12;
        var_qgod_dn17 = assign31680_e46592_d_n17;

        let (assign31690_e46604,) = {
    if ((p.p24 != 0.0) && (var_guard982 != 0.0)) {
        let assign31690_e46598: f64 = (var_modenml * var_cgso_given);
        let assign31690_e46601: f64 = (var_modervs * var_cgdo_given);
        let assign31690_e46602: f64 = (assign31690_e46598 + assign31690_e46601);
        (assign31690_e46602,)
    } else {
        (var_flg_overgiven,)
    }
};
        var_flg_overgiven = assign31690_e46604;

        let (assign31700_e46618, assign31700_e46618_d_n0, assign31700_e46618_d_n2, assign31700_e46618_d_n6, assign31700_e46618_d_n7, assign31700_e46618_d_n10, assign31700_e46618_d_n11, assign31700_e46618_d_n12, assign31700_e46618_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_flg_overgiven != 0.0)) {
        let assign31700_e46612: f64 = (var_modenml * p.p170);
        let assign31700_e46615: f64 = (var_modervs * p.p169);
        let assign31700_e46616: f64 = (assign31700_e46612 + assign31700_e46615);
        (assign31700_e46616, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31700_e46618;
        var_cgsoe_dn0 = assign31700_e46618_d_n0;
        var_cgsoe_dn2 = assign31700_e46618_d_n2;
        var_cgsoe_dn6 = assign31700_e46618_d_n6;
        var_cgsoe_dn7 = assign31700_e46618_d_n7;
        var_cgsoe_dn10 = assign31700_e46618_d_n10;
        var_cgsoe_dn11 = assign31700_e46618_d_n11;
        var_cgsoe_dn12 = assign31700_e46618_d_n12;
        var_cgsoe_dn17 = assign31700_e46618_d_n17;

        let assign31710_e46621: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1025 = assign31710_e46621;

        let (assign31720_e46637, assign31720_e46637_d_n0, assign31720_e46637_d_n2, assign31720_e46637_d_n6, assign31720_e46637_d_n7, assign31720_e46637_d_n10, assign31720_e46637_d_n11, assign31720_e46637_d_n12, assign31720_e46637_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_flg_overgiven != 0.0)) && (var_guard1025 != 0.0)) {
        let assign31720_e46631: f64 = (var_modenml * var_w_dioscv);
        let assign31720_e46634: f64 = (var_modervs * var_w_diodcv);
        let assign31720_e46635: f64 = (assign31720_e46631 + assign31720_e46634);
        (assign31720_e46635, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk900, var_t1__blk900_dn0, var_t1__blk900_dn2, var_t1__blk900_dn6, var_t1__blk900_dn7, var_t1__blk900_dn10, var_t1__blk900_dn11, var_t1__blk900_dn12, var_t1__blk900_dn17,)
    }
};
        var_t1__blk900 = assign31720_e46637;
        var_t1__blk900_dn0 = assign31720_e46637_d_n0;
        var_t1__blk900_dn2 = assign31720_e46637_d_n2;
        var_t1__blk900_dn6 = assign31720_e46637_d_n6;
        var_t1__blk900_dn7 = assign31720_e46637_d_n7;
        var_t1__blk900_dn10 = assign31720_e46637_d_n10;
        var_t1__blk900_dn11 = assign31720_e46637_d_n11;
        var_t1__blk900_dn12 = assign31720_e46637_d_n12;
        var_t1__blk900_dn17 = assign31720_e46637_d_n17;

        let (assign31730_e46650, assign31730_e46650_d_n0, assign31730_e46650_d_n2, assign31730_e46650_d_n6, assign31730_e46650_d_n7, assign31730_e46650_d_n10, assign31730_e46650_d_n11, assign31730_e46650_d_n12, assign31730_e46650_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_flg_overgiven != 0.0)) && (var_guard1025 != 0.0)) {
        let assign31730_e46647: f64 = (-var_t1__blk900);
        let assign31730_e46648: f64 = (var_cgsoe * assign31730_e46647);
        (assign31730_e46648, ((var_cgsoe_dn0 * assign31730_e46647) + (var_cgsoe * (-var_t1__blk900_dn0))), ((var_cgsoe_dn2 * assign31730_e46647) + (var_cgsoe * (-var_t1__blk900_dn2))), ((var_cgsoe_dn6 * assign31730_e46647) + (var_cgsoe * (-var_t1__blk900_dn6))), ((var_cgsoe_dn7 * assign31730_e46647) + (var_cgsoe * (-var_t1__blk900_dn7))), ((var_cgsoe_dn10 * assign31730_e46647) + (var_cgsoe * (-var_t1__blk900_dn10))), ((var_cgsoe_dn11 * assign31730_e46647) + (var_cgsoe * (-var_t1__blk900_dn11))), ((var_cgsoe_dn12 * assign31730_e46647) + (var_cgsoe * (-var_t1__blk900_dn12))), ((var_cgsoe_dn17 * assign31730_e46647) + (var_cgsoe * (-var_t1__blk900_dn17))),)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31730_e46650;
        var_cgsoe_dn0 = assign31730_e46650_d_n0;
        var_cgsoe_dn2 = assign31730_e46650_d_n2;
        var_cgsoe_dn6 = assign31730_e46650_d_n6;
        var_cgsoe_dn7 = assign31730_e46650_d_n7;
        var_cgsoe_dn10 = assign31730_e46650_d_n10;
        var_cgsoe_dn11 = assign31730_e46650_d_n11;
        var_cgsoe_dn12 = assign31730_e46650_d_n12;
        var_cgsoe_dn17 = assign31730_e46650_d_n17;

        let (assign31740_e46664, assign31740_e46664_d_n0, assign31740_e46664_d_n2, assign31740_e46664_d_n6, assign31740_e46664_d_n7, assign31740_e46664_d_n10, assign31740_e46664_d_n11, assign31740_e46664_d_n12, assign31740_e46664_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_flg_overgiven != 0.0)) && (var_guard1025 == 0.0)) {
        let assign31740_e46661: f64 = (-var_weffcv_nf);
        let assign31740_e46662: f64 = (var_cgsoe * assign31740_e46661);
        (assign31740_e46662, (var_cgsoe_dn0 * assign31740_e46661), (var_cgsoe_dn2 * assign31740_e46661), (var_cgsoe_dn6 * assign31740_e46661), (var_cgsoe_dn7 * assign31740_e46661), (var_cgsoe_dn10 * assign31740_e46661), (var_cgsoe_dn11 * assign31740_e46661), (var_cgsoe_dn12 * assign31740_e46661), (var_cgsoe_dn17 * assign31740_e46661),)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31740_e46664;
        var_cgsoe_dn0 = assign31740_e46664_d_n0;
        var_cgsoe_dn2 = assign31740_e46664_d_n2;
        var_cgsoe_dn6 = assign31740_e46664_d_n6;
        var_cgsoe_dn7 = assign31740_e46664_d_n7;
        var_cgsoe_dn10 = assign31740_e46664_d_n10;
        var_cgsoe_dn11 = assign31740_e46664_d_n11;
        var_cgsoe_dn12 = assign31740_e46664_d_n12;
        var_cgsoe_dn17 = assign31740_e46664_d_n17;

        let (assign31750_e46677, assign31750_e46677_d_n0, assign31750_e46677_d_n2, assign31750_e46677_d_n6, assign31750_e46677_d_n7, assign31750_e46677_d_n10, assign31750_e46677_d_n11, assign31750_e46677_d_n12, assign31750_e46677_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard982 != 0.0)) && (var_flg_overgiven != 0.0)) {
        let assign31750_e46672: f64 = (-var_cgsoe);
        let assign31750_e46674: f64 = (assign31750_e46672 * var_vgs);
        let assign31750_e46675: f64 = (var_qgos + assign31750_e46674);
        (assign31750_e46675, (var_qgos_dn0 + ((-var_cgsoe_dn0) * var_vgs)), (var_qgos_dn2 + ((-var_cgsoe_dn2) * var_vgs)), (var_qgos_dn6 + (((-var_cgsoe_dn6) * var_vgs) + (assign31750_e46672 * var_vgs_dn6))), (var_qgos_dn7 + (((-var_cgsoe_dn7) * var_vgs) + (assign31750_e46672 * var_vgs_dn7))), (var_qgos_dn10 + ((-var_cgsoe_dn10) * var_vgs)), (var_qgos_dn11 + (((-var_cgsoe_dn11) * var_vgs) + (assign31750_e46672 * var_vgs_dn11))), (var_qgos_dn12 + ((-var_cgsoe_dn12) * var_vgs)), (var_qgos_dn17 + ((-var_cgsoe_dn17) * var_vgs)),)
    } else {
        (var_qgos, var_qgos_dn0, var_qgos_dn2, var_qgos_dn6, var_qgos_dn7, var_qgos_dn10, var_qgos_dn11, var_qgos_dn12, var_qgos_dn17,)
    }
};
        var_qgos = assign31750_e46677;
        var_qgos_dn0 = assign31750_e46677_d_n0;
        var_qgos_dn2 = assign31750_e46677_d_n2;
        var_qgos_dn6 = assign31750_e46677_d_n6;
        var_qgos_dn7 = assign31750_e46677_d_n7;
        var_qgos_dn10 = assign31750_e46677_d_n10;
        var_qgos_dn11 = assign31750_e46677_d_n11;
        var_qgos_dn12 = assign31750_e46677_d_n12;
        var_qgos_dn17 = assign31750_e46677_d_n17;

        let assign31760_e46690: f64 = if (((var_mode == 1.0) && (var_cgdo_given == 0.0)) || ((var_mode != 1.0) && (var_cgso_given == 0.0))) { 1.0 } else { 0.0 };
        var_guard1026 = assign31760_e46690;

        let assign31770_e46693: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1027 = assign31770_e46693;

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
        *var_guard1020_slot = var_guard1020;
        *var_guard1022_slot = var_guard1022;
        *var_guard1023_slot = var_guard1023;
        *var_guard1024_slot = var_guard1024;
        *var_guard1025_slot = var_guard1025;
        *var_guard1026_slot = var_guard1026;
        *var_guard1027_slot = var_guard1027;
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
        *var_t1__blk900_slot = var_t1__blk900;
        *var_t1__blk900_dn0_slot = var_t1__blk900_dn0;
        *var_t1__blk900_dn10_slot = var_t1__blk900_dn10;
        *var_t1__blk900_dn11_slot = var_t1__blk900_dn11;
        *var_t1__blk900_dn12_slot = var_t1__blk900_dn12;
        *var_t1__blk900_dn17_slot = var_t1__blk900_dn17;
        *var_t1__blk900_dn2_slot = var_t1__blk900_dn2;
        *var_t1__blk900_dn6_slot = var_t1__blk900_dn6;
        *var_t1__blk900_dn7_slot = var_t1__blk900_dn7;
        *var_t4__blk903_slot = var_t4__blk903;
        *var_t4__blk903_dn0_slot = var_t4__blk903_dn0;
        *var_t4__blk903_dn10_slot = var_t4__blk903_dn10;
        *var_t4__blk903_dn11_slot = var_t4__blk903_dn11;
        *var_t4__blk903_dn12_slot = var_t4__blk903_dn12;
        *var_t4__blk903_dn17_slot = var_t4__blk903_dn17;
        *var_t4__blk903_dn2_slot = var_t4__blk903_dn2;
        *var_t4__blk903_dn6_slot = var_t4__blk903_dn6;
        *var_t4__blk903_dn7_slot = var_t4__blk903_dn7;
        *var_xi0__blk980_slot = var_xi0__blk980;
        *var_xi0__blk980_dn0_slot = var_xi0__blk980_dn0;
        *var_xi0__blk980_dn10_slot = var_xi0__blk980_dn10;
        *var_xi0__blk980_dn11_slot = var_xi0__blk980_dn11;
        *var_xi0__blk980_dn12_slot = var_xi0__blk980_dn12;
        *var_xi0__blk980_dn17_slot = var_xi0__blk980_dn17;
        *var_xi0__blk980_dn2_slot = var_xi0__blk980_dn2;
        *var_xi0__blk980_dn6_slot = var_xi0__blk980_dn6;
        *var_xi0__blk980_dn7_slot = var_xi0__blk980_dn7;
        *var_xi0p12__blk981_slot = var_xi0p12__blk981;
        *var_xi0p12__blk981_dn0_slot = var_xi0p12__blk981_dn0;
        *var_xi0p12__blk981_dn10_slot = var_xi0p12__blk981_dn10;
        *var_xi0p12__blk981_dn11_slot = var_xi0p12__blk981_dn11;
        *var_xi0p12__blk981_dn12_slot = var_xi0p12__blk981_dn12;
        *var_xi0p12__blk981_dn17_slot = var_xi0p12__blk981_dn17;
        *var_xi0p12__blk981_dn2_slot = var_xi0p12__blk981_dn2;
        *var_xi0p12__blk981_dn6_slot = var_xi0p12__blk981_dn6;
        *var_xi0p12__blk981_dn7_slot = var_xi0p12__blk981_dn7;
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
        var_cox0__blk910: f64,
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
        var_guard1026: f64,
        var_guard1027: f64,
        var_guard982: f64,
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
        var_guard1028_slot: &mut f64,
        var_guard1029_slot: &mut f64,
        var_guard1030_slot: &mut f64,
        var_guard1031_slot: &mut f64,
        var_guard1032_slot: &mut f64,
        var_guard1061_slot: &mut f64,
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
        var_t1__blk1034_slot: &mut f64,
        var_t1__blk1034_dn10_slot: &mut f64,
        var_t1__blk1034_dn12_slot: &mut f64,
        var_t1__blk1034_dn6_slot: &mut f64,
        var_t1__blk1034_dn7_slot: &mut f64,
        var_t1__blk900_slot: &mut f64,
        var_t1__blk900_dn0_slot: &mut f64,
        var_t1__blk900_dn10_slot: &mut f64,
        var_t1__blk900_dn11_slot: &mut f64,
        var_t1__blk900_dn12_slot: &mut f64,
        var_t1__blk900_dn17_slot: &mut f64,
        var_t1__blk900_dn2_slot: &mut f64,
        var_t1__blk900_dn6_slot: &mut f64,
        var_t1__blk900_dn7_slot: &mut f64,
        var_t2__blk1035_slot: &mut f64,
        var_t2__blk1035_dn0_slot: &mut f64,
        var_t2__blk1035_dn10_slot: &mut f64,
        var_t2__blk1035_dn11_slot: &mut f64,
        var_t2__blk1035_dn12_slot: &mut f64,
        var_t2__blk1035_dn17_slot: &mut f64,
        var_t2__blk1035_dn2_slot: &mut f64,
        var_t2__blk1035_dn6_slot: &mut f64,
        var_t2__blk1035_dn7_slot: &mut f64,
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
        let mut var_guard1028: f64 = *var_guard1028_slot;
        let mut var_guard1029: f64 = *var_guard1029_slot;
        let mut var_guard1030: f64 = *var_guard1030_slot;
        let mut var_guard1031: f64 = *var_guard1031_slot;
        let mut var_guard1032: f64 = *var_guard1032_slot;
        let mut var_guard1061: f64 = *var_guard1061_slot;
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
        let mut var_t1__blk1034: f64 = *var_t1__blk1034_slot;
        let mut var_t1__blk1034_dn10: f64 = *var_t1__blk1034_dn10_slot;
        let mut var_t1__blk1034_dn12: f64 = *var_t1__blk1034_dn12_slot;
        let mut var_t1__blk1034_dn6: f64 = *var_t1__blk1034_dn6_slot;
        let mut var_t1__blk1034_dn7: f64 = *var_t1__blk1034_dn7_slot;
        let mut var_t1__blk900: f64 = *var_t1__blk900_slot;
        let mut var_t1__blk900_dn0: f64 = *var_t1__blk900_dn0_slot;
        let mut var_t1__blk900_dn10: f64 = *var_t1__blk900_dn10_slot;
        let mut var_t1__blk900_dn11: f64 = *var_t1__blk900_dn11_slot;
        let mut var_t1__blk900_dn12: f64 = *var_t1__blk900_dn12_slot;
        let mut var_t1__blk900_dn17: f64 = *var_t1__blk900_dn17_slot;
        let mut var_t1__blk900_dn2: f64 = *var_t1__blk900_dn2_slot;
        let mut var_t1__blk900_dn6: f64 = *var_t1__blk900_dn6_slot;
        let mut var_t1__blk900_dn7: f64 = *var_t1__blk900_dn7_slot;
        let mut var_t2__blk1035: f64 = *var_t2__blk1035_slot;
        let mut var_t2__blk1035_dn0: f64 = *var_t2__blk1035_dn0_slot;
        let mut var_t2__blk1035_dn10: f64 = *var_t2__blk1035_dn10_slot;
        let mut var_t2__blk1035_dn11: f64 = *var_t2__blk1035_dn11_slot;
        let mut var_t2__blk1035_dn12: f64 = *var_t2__blk1035_dn12_slot;
        let mut var_t2__blk1035_dn17: f64 = *var_t2__blk1035_dn17_slot;
        let mut var_t2__blk1035_dn2: f64 = *var_t2__blk1035_dn2_slot;
        let mut var_t2__blk1035_dn6: f64 = *var_t2__blk1035_dn6_slot;
        let mut var_t2__blk1035_dn7: f64 = *var_t2__blk1035_dn7_slot;
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

        let (assign31780_e46709, assign31780_e46709_d_n0, assign31780_e46709_d_n2, assign31780_e46709_d_n6, assign31780_e46709_d_n7, assign31780_e46709_d_n10, assign31780_e46709_d_n11, assign31780_e46709_d_n12, assign31780_e46709_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 == 0.0)) && (var_guard1026 != 0.0)) && (var_guard1027 != 0.0)) {
        let assign31780_e46703: f64 = (-var_cox0__blk910);
        let assign31780_e46705: f64 = (assign31780_e46703 * p.p188);
        let assign31780_e46707: f64 = (assign31780_e46705 * var_w_diodcv);
        (assign31780_e46707, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31780_e46709;
        var_cgdoe_dn0 = assign31780_e46709_d_n0;
        var_cgdoe_dn2 = assign31780_e46709_d_n2;
        var_cgdoe_dn6 = assign31780_e46709_d_n6;
        var_cgdoe_dn7 = assign31780_e46709_d_n7;
        var_cgdoe_dn10 = assign31780_e46709_d_n10;
        var_cgdoe_dn11 = assign31780_e46709_d_n11;
        var_cgdoe_dn12 = assign31780_e46709_d_n12;
        var_cgdoe_dn17 = assign31780_e46709_d_n17;

        let (assign31790_e46726, assign31790_e46726_d_n0, assign31790_e46726_d_n2, assign31790_e46726_d_n6, assign31790_e46726_d_n7, assign31790_e46726_d_n10, assign31790_e46726_d_n11, assign31790_e46726_d_n12, assign31790_e46726_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 == 0.0)) && (var_guard1026 != 0.0)) && (var_guard1027 == 0.0)) {
        let assign31790_e46720: f64 = (-var_cox0__blk910);
        let assign31790_e46722: f64 = (assign31790_e46720 * p.p188);
        let assign31790_e46724: f64 = (assign31790_e46722 * var_weffcv_nf);
        (assign31790_e46724, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31790_e46726;
        var_cgdoe_dn0 = assign31790_e46726_d_n0;
        var_cgdoe_dn2 = assign31790_e46726_d_n2;
        var_cgdoe_dn6 = assign31790_e46726_d_n6;
        var_cgdoe_dn7 = assign31790_e46726_d_n7;
        var_cgdoe_dn10 = assign31790_e46726_d_n10;
        var_cgdoe_dn11 = assign31790_e46726_d_n11;
        var_cgdoe_dn12 = assign31790_e46726_d_n12;
        var_cgdoe_dn17 = assign31790_e46726_d_n17;

        let (assign31800_e46742, assign31800_e46742_d_n0, assign31800_e46742_d_n2, assign31800_e46742_d_n6, assign31800_e46742_d_n7, assign31800_e46742_d_n10, assign31800_e46742_d_n11, assign31800_e46742_d_n12, assign31800_e46742_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard982 == 0.0)) && (var_guard1026 == 0.0)) {
        let assign31800_e46736: f64 = (var_modervs * p.p170);
        let assign31800_e46739: f64 = (var_modenml * p.p169);
        let assign31800_e46740: f64 = (assign31800_e46736 + assign31800_e46739);
        (assign31800_e46740, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31800_e46742;
        var_cgdoe_dn0 = assign31800_e46742_d_n0;
        var_cgdoe_dn2 = assign31800_e46742_d_n2;
        var_cgdoe_dn6 = assign31800_e46742_d_n6;
        var_cgdoe_dn7 = assign31800_e46742_d_n7;
        var_cgdoe_dn10 = assign31800_e46742_d_n10;
        var_cgdoe_dn11 = assign31800_e46742_d_n11;
        var_cgdoe_dn12 = assign31800_e46742_d_n12;
        var_cgdoe_dn17 = assign31800_e46742_d_n17;

        let assign31810_e46745: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1028 = assign31810_e46745;

        let (assign31820_e46763, assign31820_e46763_d_n0, assign31820_e46763_d_n2, assign31820_e46763_d_n6, assign31820_e46763_d_n7, assign31820_e46763_d_n10, assign31820_e46763_d_n11, assign31820_e46763_d_n12, assign31820_e46763_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 == 0.0)) && (var_guard1026 == 0.0)) && (var_guard1028 != 0.0)) {
        let assign31820_e46757: f64 = (var_modervs * var_w_dioscv);
        let assign31820_e46760: f64 = (var_modenml * var_w_diodcv);
        let assign31820_e46761: f64 = (assign31820_e46757 + assign31820_e46760);
        (assign31820_e46761, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk900, var_t1__blk900_dn0, var_t1__blk900_dn2, var_t1__blk900_dn6, var_t1__blk900_dn7, var_t1__blk900_dn10, var_t1__blk900_dn11, var_t1__blk900_dn12, var_t1__blk900_dn17,)
    }
};
        var_t1__blk900 = assign31820_e46763;
        var_t1__blk900_dn0 = assign31820_e46763_d_n0;
        var_t1__blk900_dn2 = assign31820_e46763_d_n2;
        var_t1__blk900_dn6 = assign31820_e46763_d_n6;
        var_t1__blk900_dn7 = assign31820_e46763_d_n7;
        var_t1__blk900_dn10 = assign31820_e46763_d_n10;
        var_t1__blk900_dn11 = assign31820_e46763_d_n11;
        var_t1__blk900_dn12 = assign31820_e46763_d_n12;
        var_t1__blk900_dn17 = assign31820_e46763_d_n17;

        let (assign31830_e46778, assign31830_e46778_d_n0, assign31830_e46778_d_n2, assign31830_e46778_d_n6, assign31830_e46778_d_n7, assign31830_e46778_d_n10, assign31830_e46778_d_n11, assign31830_e46778_d_n12, assign31830_e46778_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 == 0.0)) && (var_guard1026 == 0.0)) && (var_guard1028 != 0.0)) {
        let assign31830_e46775: f64 = (-var_t1__blk900);
        let assign31830_e46776: f64 = (var_cgdoe * assign31830_e46775);
        (assign31830_e46776, ((var_cgdoe_dn0 * assign31830_e46775) + (var_cgdoe * (-var_t1__blk900_dn0))), ((var_cgdoe_dn2 * assign31830_e46775) + (var_cgdoe * (-var_t1__blk900_dn2))), ((var_cgdoe_dn6 * assign31830_e46775) + (var_cgdoe * (-var_t1__blk900_dn6))), ((var_cgdoe_dn7 * assign31830_e46775) + (var_cgdoe * (-var_t1__blk900_dn7))), ((var_cgdoe_dn10 * assign31830_e46775) + (var_cgdoe * (-var_t1__blk900_dn10))), ((var_cgdoe_dn11 * assign31830_e46775) + (var_cgdoe * (-var_t1__blk900_dn11))), ((var_cgdoe_dn12 * assign31830_e46775) + (var_cgdoe * (-var_t1__blk900_dn12))), ((var_cgdoe_dn17 * assign31830_e46775) + (var_cgdoe * (-var_t1__blk900_dn17))),)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31830_e46778;
        var_cgdoe_dn0 = assign31830_e46778_d_n0;
        var_cgdoe_dn2 = assign31830_e46778_d_n2;
        var_cgdoe_dn6 = assign31830_e46778_d_n6;
        var_cgdoe_dn7 = assign31830_e46778_d_n7;
        var_cgdoe_dn10 = assign31830_e46778_d_n10;
        var_cgdoe_dn11 = assign31830_e46778_d_n11;
        var_cgdoe_dn12 = assign31830_e46778_d_n12;
        var_cgdoe_dn17 = assign31830_e46778_d_n17;

        let (assign31840_e46794, assign31840_e46794_d_n0, assign31840_e46794_d_n2, assign31840_e46794_d_n6, assign31840_e46794_d_n7, assign31840_e46794_d_n10, assign31840_e46794_d_n11, assign31840_e46794_d_n12, assign31840_e46794_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 == 0.0)) && (var_guard1026 == 0.0)) && (var_guard1028 == 0.0)) {
        let assign31840_e46791: f64 = (-var_weffcv_nf);
        let assign31840_e46792: f64 = (var_cgdoe * assign31840_e46791);
        (assign31840_e46792, (var_cgdoe_dn0 * assign31840_e46791), (var_cgdoe_dn2 * assign31840_e46791), (var_cgdoe_dn6 * assign31840_e46791), (var_cgdoe_dn7 * assign31840_e46791), (var_cgdoe_dn10 * assign31840_e46791), (var_cgdoe_dn11 * assign31840_e46791), (var_cgdoe_dn12 * assign31840_e46791), (var_cgdoe_dn17 * assign31840_e46791),)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn6, var_cgdoe_dn7, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12, var_cgdoe_dn17,)
    }
};
        var_cgdoe = assign31840_e46794;
        var_cgdoe_dn0 = assign31840_e46794_d_n0;
        var_cgdoe_dn2 = assign31840_e46794_d_n2;
        var_cgdoe_dn6 = assign31840_e46794_d_n6;
        var_cgdoe_dn7 = assign31840_e46794_d_n7;
        var_cgdoe_dn10 = assign31840_e46794_d_n10;
        var_cgdoe_dn11 = assign31840_e46794_d_n11;
        var_cgdoe_dn12 = assign31840_e46794_d_n12;
        var_cgdoe_dn17 = assign31840_e46794_d_n17;

        let (assign31850_e46806, assign31850_e46806_d_n0, assign31850_e46806_d_n2, assign31850_e46806_d_n6, assign31850_e46806_d_n7, assign31850_e46806_d_n10, assign31850_e46806_d_n11, assign31850_e46806_d_n12, assign31850_e46806_d_n17,) = {
    if ((p.p24 != 0.0) && (var_guard982 == 0.0)) {
        let assign31850_e46800: f64 = (-var_cgdoe);
        let assign31850_e46803: f64 = (var_vgs - var_vds);
        let assign31850_e46804: f64 = (assign31850_e46800 * assign31850_e46803);
        (assign31850_e46804, (((-var_cgdoe_dn0) * assign31850_e46803) + (assign31850_e46800 * (-var_vds_dn0))), (((-var_cgdoe_dn2) * assign31850_e46803) + (assign31850_e46800 * (-var_vds_dn2))), (((-var_cgdoe_dn6) * assign31850_e46803) + (assign31850_e46800 * (var_vgs_dn6 - var_vds_dn6))), (((-var_cgdoe_dn7) * assign31850_e46803) + (assign31850_e46800 * (var_vgs_dn7 - var_vds_dn7))), (((-var_cgdoe_dn10) * assign31850_e46803) + (assign31850_e46800 * (-var_vds_dn10))), (((-var_cgdoe_dn11) * assign31850_e46803) + (assign31850_e46800 * (var_vgs_dn11 - var_vds_dn11))), (((-var_cgdoe_dn12) * assign31850_e46803) + (assign31850_e46800 * (-var_vds_dn12))), (((-var_cgdoe_dn17) * assign31850_e46803) + (assign31850_e46800 * (-var_vds_dn17))),)
    } else {
        (var_qgod, var_qgod_dn0, var_qgod_dn2, var_qgod_dn6, var_qgod_dn7, var_qgod_dn10, var_qgod_dn11, var_qgod_dn12, var_qgod_dn17,)
    }
};
        var_qgod = assign31850_e46806;
        var_qgod_dn0 = assign31850_e46806_d_n0;
        var_qgod_dn2 = assign31850_e46806_d_n2;
        var_qgod_dn6 = assign31850_e46806_d_n6;
        var_qgod_dn7 = assign31850_e46806_d_n7;
        var_qgod_dn10 = assign31850_e46806_d_n10;
        var_qgod_dn11 = assign31850_e46806_d_n11;
        var_qgod_dn12 = assign31850_e46806_d_n12;
        var_qgod_dn17 = assign31850_e46806_d_n17;

        let assign31860_e46819: f64 = if (((var_mode == 1.0) && (var_cgso_given == 0.0)) || ((var_mode != 1.0) && (var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        var_guard1029 = assign31860_e46819;

        let assign31870_e46822: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1030 = assign31870_e46822;

        let (assign31880_e46838, assign31880_e46838_d_n0, assign31880_e46838_d_n2, assign31880_e46838_d_n6, assign31880_e46838_d_n7, assign31880_e46838_d_n10, assign31880_e46838_d_n11, assign31880_e46838_d_n12, assign31880_e46838_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 == 0.0)) && (var_guard1029 != 0.0)) && (var_guard1030 != 0.0)) {
        let assign31880_e46832: f64 = (-var_cox0__blk910);
        let assign31880_e46834: f64 = (assign31880_e46832 * p.p188);
        let assign31880_e46836: f64 = (assign31880_e46834 * var_w_dioscv);
        (assign31880_e46836, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31880_e46838;
        var_cgsoe_dn0 = assign31880_e46838_d_n0;
        var_cgsoe_dn2 = assign31880_e46838_d_n2;
        var_cgsoe_dn6 = assign31880_e46838_d_n6;
        var_cgsoe_dn7 = assign31880_e46838_d_n7;
        var_cgsoe_dn10 = assign31880_e46838_d_n10;
        var_cgsoe_dn11 = assign31880_e46838_d_n11;
        var_cgsoe_dn12 = assign31880_e46838_d_n12;
        var_cgsoe_dn17 = assign31880_e46838_d_n17;

        let (assign31890_e46855, assign31890_e46855_d_n0, assign31890_e46855_d_n2, assign31890_e46855_d_n6, assign31890_e46855_d_n7, assign31890_e46855_d_n10, assign31890_e46855_d_n11, assign31890_e46855_d_n12, assign31890_e46855_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 == 0.0)) && (var_guard1029 != 0.0)) && (var_guard1030 == 0.0)) {
        let assign31890_e46849: f64 = (-var_cox0__blk910);
        let assign31890_e46851: f64 = (assign31890_e46849 * p.p188);
        let assign31890_e46853: f64 = (assign31890_e46851 * var_weffcv_nf);
        (assign31890_e46853, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31890_e46855;
        var_cgsoe_dn0 = assign31890_e46855_d_n0;
        var_cgsoe_dn2 = assign31890_e46855_d_n2;
        var_cgsoe_dn6 = assign31890_e46855_d_n6;
        var_cgsoe_dn7 = assign31890_e46855_d_n7;
        var_cgsoe_dn10 = assign31890_e46855_d_n10;
        var_cgsoe_dn11 = assign31890_e46855_d_n11;
        var_cgsoe_dn12 = assign31890_e46855_d_n12;
        var_cgsoe_dn17 = assign31890_e46855_d_n17;

        let (assign31900_e46871, assign31900_e46871_d_n0, assign31900_e46871_d_n2, assign31900_e46871_d_n6, assign31900_e46871_d_n7, assign31900_e46871_d_n10, assign31900_e46871_d_n11, assign31900_e46871_d_n12, assign31900_e46871_d_n17,) = {
    if (((p.p24 != 0.0) && (var_guard982 == 0.0)) && (var_guard1029 == 0.0)) {
        let assign31900_e46865: f64 = (var_modenml * p.p170);
        let assign31900_e46868: f64 = (var_modervs * p.p169);
        let assign31900_e46869: f64 = (assign31900_e46865 + assign31900_e46868);
        (assign31900_e46869, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31900_e46871;
        var_cgsoe_dn0 = assign31900_e46871_d_n0;
        var_cgsoe_dn2 = assign31900_e46871_d_n2;
        var_cgsoe_dn6 = assign31900_e46871_d_n6;
        var_cgsoe_dn7 = assign31900_e46871_d_n7;
        var_cgsoe_dn10 = assign31900_e46871_d_n10;
        var_cgsoe_dn11 = assign31900_e46871_d_n11;
        var_cgsoe_dn12 = assign31900_e46871_d_n12;
        var_cgsoe_dn17 = assign31900_e46871_d_n17;

        let assign31910_e46874: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1031 = assign31910_e46874;

        let (assign31920_e46892, assign31920_e46892_d_n0, assign31920_e46892_d_n2, assign31920_e46892_d_n6, assign31920_e46892_d_n7, assign31920_e46892_d_n10, assign31920_e46892_d_n11, assign31920_e46892_d_n12, assign31920_e46892_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 == 0.0)) && (var_guard1029 == 0.0)) && (var_guard1031 != 0.0)) {
        let assign31920_e46886: f64 = (var_modenml * var_w_dioscv);
        let assign31920_e46889: f64 = (var_modervs * var_w_diodcv);
        let assign31920_e46890: f64 = (assign31920_e46886 + assign31920_e46889);
        (assign31920_e46890, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk900, var_t1__blk900_dn0, var_t1__blk900_dn2, var_t1__blk900_dn6, var_t1__blk900_dn7, var_t1__blk900_dn10, var_t1__blk900_dn11, var_t1__blk900_dn12, var_t1__blk900_dn17,)
    }
};
        var_t1__blk900 = assign31920_e46892;
        var_t1__blk900_dn0 = assign31920_e46892_d_n0;
        var_t1__blk900_dn2 = assign31920_e46892_d_n2;
        var_t1__blk900_dn6 = assign31920_e46892_d_n6;
        var_t1__blk900_dn7 = assign31920_e46892_d_n7;
        var_t1__blk900_dn10 = assign31920_e46892_d_n10;
        var_t1__blk900_dn11 = assign31920_e46892_d_n11;
        var_t1__blk900_dn12 = assign31920_e46892_d_n12;
        var_t1__blk900_dn17 = assign31920_e46892_d_n17;

        let (assign31930_e46907, assign31930_e46907_d_n0, assign31930_e46907_d_n2, assign31930_e46907_d_n6, assign31930_e46907_d_n7, assign31930_e46907_d_n10, assign31930_e46907_d_n11, assign31930_e46907_d_n12, assign31930_e46907_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 == 0.0)) && (var_guard1029 == 0.0)) && (var_guard1031 != 0.0)) {
        let assign31930_e46904: f64 = (-var_t1__blk900);
        let assign31930_e46905: f64 = (var_cgsoe * assign31930_e46904);
        (assign31930_e46905, ((var_cgsoe_dn0 * assign31930_e46904) + (var_cgsoe * (-var_t1__blk900_dn0))), ((var_cgsoe_dn2 * assign31930_e46904) + (var_cgsoe * (-var_t1__blk900_dn2))), ((var_cgsoe_dn6 * assign31930_e46904) + (var_cgsoe * (-var_t1__blk900_dn6))), ((var_cgsoe_dn7 * assign31930_e46904) + (var_cgsoe * (-var_t1__blk900_dn7))), ((var_cgsoe_dn10 * assign31930_e46904) + (var_cgsoe * (-var_t1__blk900_dn10))), ((var_cgsoe_dn11 * assign31930_e46904) + (var_cgsoe * (-var_t1__blk900_dn11))), ((var_cgsoe_dn12 * assign31930_e46904) + (var_cgsoe * (-var_t1__blk900_dn12))), ((var_cgsoe_dn17 * assign31930_e46904) + (var_cgsoe * (-var_t1__blk900_dn17))),)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31930_e46907;
        var_cgsoe_dn0 = assign31930_e46907_d_n0;
        var_cgsoe_dn2 = assign31930_e46907_d_n2;
        var_cgsoe_dn6 = assign31930_e46907_d_n6;
        var_cgsoe_dn7 = assign31930_e46907_d_n7;
        var_cgsoe_dn10 = assign31930_e46907_d_n10;
        var_cgsoe_dn11 = assign31930_e46907_d_n11;
        var_cgsoe_dn12 = assign31930_e46907_d_n12;
        var_cgsoe_dn17 = assign31930_e46907_d_n17;

        let (assign31940_e46923, assign31940_e46923_d_n0, assign31940_e46923_d_n2, assign31940_e46923_d_n6, assign31940_e46923_d_n7, assign31940_e46923_d_n10, assign31940_e46923_d_n11, assign31940_e46923_d_n12, assign31940_e46923_d_n17,) = {
    if ((((p.p24 != 0.0) && (var_guard982 == 0.0)) && (var_guard1029 == 0.0)) && (var_guard1031 == 0.0)) {
        let assign31940_e46920: f64 = (-var_weffcv_nf);
        let assign31940_e46921: f64 = (var_cgsoe * assign31940_e46920);
        (assign31940_e46921, (var_cgsoe_dn0 * assign31940_e46920), (var_cgsoe_dn2 * assign31940_e46920), (var_cgsoe_dn6 * assign31940_e46920), (var_cgsoe_dn7 * assign31940_e46920), (var_cgsoe_dn10 * assign31940_e46920), (var_cgsoe_dn11 * assign31940_e46920), (var_cgsoe_dn12 * assign31940_e46920), (var_cgsoe_dn17 * assign31940_e46920),)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn6, var_cgsoe_dn7, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12, var_cgsoe_dn17,)
    }
};
        var_cgsoe = assign31940_e46923;
        var_cgsoe_dn0 = assign31940_e46923_d_n0;
        var_cgsoe_dn2 = assign31940_e46923_d_n2;
        var_cgsoe_dn6 = assign31940_e46923_d_n6;
        var_cgsoe_dn7 = assign31940_e46923_d_n7;
        var_cgsoe_dn10 = assign31940_e46923_d_n10;
        var_cgsoe_dn11 = assign31940_e46923_d_n11;
        var_cgsoe_dn12 = assign31940_e46923_d_n12;
        var_cgsoe_dn17 = assign31940_e46923_d_n17;

        let (assign31950_e46933, assign31950_e46933_d_n0, assign31950_e46933_d_n2, assign31950_e46933_d_n6, assign31950_e46933_d_n7, assign31950_e46933_d_n10, assign31950_e46933_d_n11, assign31950_e46933_d_n12, assign31950_e46933_d_n17,) = {
    if ((p.p24 != 0.0) && (var_guard982 == 0.0)) {
        let assign31950_e46929: f64 = (-var_cgsoe);
        let assign31950_e46931: f64 = (assign31950_e46929 * var_vgs);
        (assign31950_e46931, ((-var_cgsoe_dn0) * var_vgs), ((-var_cgsoe_dn2) * var_vgs), (((-var_cgsoe_dn6) * var_vgs) + (assign31950_e46929 * var_vgs_dn6)), (((-var_cgsoe_dn7) * var_vgs) + (assign31950_e46929 * var_vgs_dn7)), ((-var_cgsoe_dn10) * var_vgs), (((-var_cgsoe_dn11) * var_vgs) + (assign31950_e46929 * var_vgs_dn11)), ((-var_cgsoe_dn12) * var_vgs), ((-var_cgsoe_dn17) * var_vgs),)
    } else {
        (var_qgos, var_qgos_dn0, var_qgos_dn2, var_qgos_dn6, var_qgos_dn7, var_qgos_dn10, var_qgos_dn11, var_qgos_dn12, var_qgos_dn17,)
    }
};
        var_qgos = assign31950_e46933;
        var_qgos_dn0 = assign31950_e46933_d_n0;
        var_qgos_dn2 = assign31950_e46933_d_n2;
        var_qgos_dn6 = assign31950_e46933_d_n6;
        var_qgos_dn7 = assign31950_e46933_d_n7;
        var_qgos_dn10 = assign31950_e46933_d_n10;
        var_qgos_dn11 = assign31950_e46933_d_n11;
        var_qgos_dn12 = assign31950_e46933_d_n12;
        var_qgos_dn17 = assign31950_e46933_d_n17;

        let assign31960_e46936: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1032 = assign31960_e46936;

        let (assign31970_e46940, assign31970_e46940_d_n6, assign31970_e46940_d_n12,) = {
    if (var_guard1032 != 0.0) {
        (var_vbcd, var_vbcd_dn6, var_vbcd_dn12,)
    } else {
        (var_vbdj, var_vbdj_dn6, var_vbdj_dn12,)
    }
};
        var_vbdj = assign31970_e46940;
        var_vbdj_dn6 = assign31970_e46940_d_n6;
        var_vbdj_dn12 = assign31970_e46940_d_n12;

        let (assign31980_e46944, assign31980_e46944_d_n7, assign31980_e46944_d_n12,) = {
    if (var_guard1032 != 0.0) {
        (var_vbcs, var_vbcs_dn7, var_vbcs_dn12,)
    } else {
        (var_vbsj, var_vbsj_dn7, var_vbsj_dn12,)
    }
};
        var_vbsj = assign31980_e46944;
        var_vbsj_dn7 = assign31980_e46944_d_n7;
        var_vbsj_dn12 = assign31980_e46944_d_n12;

        let (assign31990_e46966, assign31990_e46966_d_n0, assign31990_e46966_d_n2, assign31990_e46966_d_n6, assign31990_e46966_d_n7, assign31990_e46966_d_n10, assign31990_e46966_d_n11, assign31990_e46966_d_n12, assign31990_e46966_d_n17,) = {
    if (var_guard1032 != 0.0) {
        let assign31990_e46949: f64 = (var_egtnom * var_betatnom);
        let assign31990_e46952: f64 = (var_eg * var_beta);
        let assign31990_e46953: f64 = (assign31990_e46949 - assign31990_e46952);
        let assign31990_e46957: f64 = (var_ttemp / var_uc_tnom);
        let assign31990_e46958: f64 = (assign31990_e46957).ln();
        let assign31990_e46959: f64 = (p.p175 * assign31990_e46958);
        let assign31990_e46960: f64 = (assign31990_e46953 + assign31990_e46959);
        let assign31990_e46962: f64 = (assign31990_e46960 / p.p174);
        let assign31990_e46963: f64 = (assign31990_e46962).exp();
        let assign31990_e46964: f64 = (p.p173 * assign31990_e46963);
        (assign31990_e46964, (p.p173 * (assign31990_e46963 * ((-(var_eg_dn0 * var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(var_eg_dn2 * var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(var_eg_dn6 * var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(var_eg_dn7 * var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * (((-((var_eg_dn10 * var_beta) + (var_eg * var_beta_dn10))) + (p.p175 * ((var_ttemp_dn10 / var_uc_tnom) / assign31990_e46957))) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(var_eg_dn11 * var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(var_eg_dn12 * var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(var_eg_dn17 * var_beta)) / p.p174))),)
    } else {
        (var_js, var_js_dn0, var_js_dn2, var_js_dn6, var_js_dn7, var_js_dn10, var_js_dn11, var_js_dn12, var_js_dn17,)
    }
};
        var_js = assign31990_e46966;
        var_js_dn0 = assign31990_e46966_d_n0;
        var_js_dn2 = assign31990_e46966_d_n2;
        var_js_dn6 = assign31990_e46966_d_n6;
        var_js_dn7 = assign31990_e46966_d_n7;
        var_js_dn10 = assign31990_e46966_d_n10;
        var_js_dn11 = assign31990_e46966_d_n11;
        var_js_dn12 = assign31990_e46966_d_n12;
        var_js_dn17 = assign31990_e46966_d_n17;

        let (assign32000_e46988, assign32000_e46988_d_n0, assign32000_e46988_d_n2, assign32000_e46988_d_n6, assign32000_e46988_d_n7, assign32000_e46988_d_n10, assign32000_e46988_d_n11, assign32000_e46988_d_n12, assign32000_e46988_d_n17,) = {
    if (var_guard1032 != 0.0) {
        let assign32000_e46971: f64 = (var_egtnom * var_betatnom);
        let assign32000_e46974: f64 = (var_eg * var_beta);
        let assign32000_e46975: f64 = (assign32000_e46971 - assign32000_e46974);
        let assign32000_e46979: f64 = (var_ttemp / var_uc_tnom);
        let assign32000_e46980: f64 = (assign32000_e46979).ln();
        let assign32000_e46981: f64 = (p.p176 * assign32000_e46980);
        let assign32000_e46982: f64 = (assign32000_e46975 + assign32000_e46981);
        let assign32000_e46984: f64 = (assign32000_e46982 / p.p174);
        let assign32000_e46985: f64 = (assign32000_e46984).exp();
        let assign32000_e46986: f64 = (p.p173 * assign32000_e46985);
        (assign32000_e46986, (p.p173 * (assign32000_e46985 * ((-(var_eg_dn0 * var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(var_eg_dn2 * var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(var_eg_dn6 * var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(var_eg_dn7 * var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * (((-((var_eg_dn10 * var_beta) + (var_eg * var_beta_dn10))) + (p.p176 * ((var_ttemp_dn10 / var_uc_tnom) / assign32000_e46979))) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(var_eg_dn11 * var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(var_eg_dn12 * var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(var_eg_dn17 * var_beta)) / p.p174))),)
    } else {
        (var_js2, var_js2_dn0, var_js2_dn2, var_js2_dn6, var_js2_dn7, var_js2_dn10, var_js2_dn11, var_js2_dn12, var_js2_dn17,)
    }
};
        var_js2 = assign32000_e46988;
        var_js2_dn0 = assign32000_e46988_d_n0;
        var_js2_dn2 = assign32000_e46988_d_n2;
        var_js2_dn6 = assign32000_e46988_d_n6;
        var_js2_dn7 = assign32000_e46988_d_n7;
        var_js2_dn10 = assign32000_e46988_d_n10;
        var_js2_dn11 = assign32000_e46988_d_n11;
        var_js2_dn12 = assign32000_e46988_d_n12;
        var_js2_dn17 = assign32000_e46988_d_n17;

        let (assign32010_e46996, assign32010_e46996_d_n0, assign32010_e46996_d_n2, assign32010_e46996_d_n6, assign32010_e46996_d_n7, assign32010_e46996_d_n10, assign32010_e46996_d_n11, assign32010_e46996_d_n12, assign32010_e46996_d_n17,) = {
    if (var_guard1032 != 0.0) {
        let assign32010_e46992: f64 = (var_w_diod * p.p237);
        let assign32010_e46994: f64 = (assign32010_e46992 * var_js);
        (assign32010_e46994, (assign32010_e46992 * var_js_dn0), (assign32010_e46992 * var_js_dn2), (assign32010_e46992 * var_js_dn6), (assign32010_e46992 * var_js_dn7), (assign32010_e46992 * var_js_dn10), (assign32010_e46992 * var_js_dn11), (assign32010_e46992 * var_js_dn12), (assign32010_e46992 * var_js_dn17),)
    } else {
        (var_isbd, var_isbd_dn0, var_isbd_dn2, var_isbd_dn6, var_isbd_dn7, var_isbd_dn10, var_isbd_dn11, var_isbd_dn12, var_isbd_dn17,)
    }
};
        var_isbd = assign32010_e46996;
        var_isbd_dn0 = assign32010_e46996_d_n0;
        var_isbd_dn2 = assign32010_e46996_d_n2;
        var_isbd_dn6 = assign32010_e46996_d_n6;
        var_isbd_dn7 = assign32010_e46996_d_n7;
        var_isbd_dn10 = assign32010_e46996_d_n10;
        var_isbd_dn11 = assign32010_e46996_d_n11;
        var_isbd_dn12 = assign32010_e46996_d_n12;
        var_isbd_dn17 = assign32010_e46996_d_n17;

        let (assign32020_e47004, assign32020_e47004_d_n0, assign32020_e47004_d_n2, assign32020_e47004_d_n6, assign32020_e47004_d_n7, assign32020_e47004_d_n10, assign32020_e47004_d_n11, assign32020_e47004_d_n12, assign32020_e47004_d_n17,) = {
    if (var_guard1032 != 0.0) {
        let assign32020_e47000: f64 = (var_w_diod * p.p237);
        let assign32020_e47002: f64 = (assign32020_e47000 * var_js2);
        (assign32020_e47002, (assign32020_e47000 * var_js2_dn0), (assign32020_e47000 * var_js2_dn2), (assign32020_e47000 * var_js2_dn6), (assign32020_e47000 * var_js2_dn7), (assign32020_e47000 * var_js2_dn10), (assign32020_e47000 * var_js2_dn11), (assign32020_e47000 * var_js2_dn12), (assign32020_e47000 * var_js2_dn17),)
    } else {
        (var_isbd2, var_isbd2_dn0, var_isbd2_dn2, var_isbd2_dn6, var_isbd2_dn7, var_isbd2_dn10, var_isbd2_dn11, var_isbd2_dn12, var_isbd2_dn17,)
    }
};
        var_isbd2 = assign32020_e47004;
        var_isbd2_dn0 = assign32020_e47004_d_n0;
        var_isbd2_dn2 = assign32020_e47004_d_n2;
        var_isbd2_dn6 = assign32020_e47004_d_n6;
        var_isbd2_dn7 = assign32020_e47004_d_n7;
        var_isbd2_dn10 = assign32020_e47004_d_n10;
        var_isbd2_dn11 = assign32020_e47004_d_n11;
        var_isbd2_dn12 = assign32020_e47004_d_n12;
        var_isbd2_dn17 = assign32020_e47004_d_n17;

        let (assign32030_e47012, assign32030_e47012_d_n0, assign32030_e47012_d_n2, assign32030_e47012_d_n6, assign32030_e47012_d_n7, assign32030_e47012_d_n10, assign32030_e47012_d_n11, assign32030_e47012_d_n12, assign32030_e47012_d_n17,) = {
    if (var_guard1032 != 0.0) {
        let assign32030_e47008: f64 = (var_w_dios * p.p237);
        let assign32030_e47010: f64 = (assign32030_e47008 * var_js);
        (assign32030_e47010, (assign32030_e47008 * var_js_dn0), (assign32030_e47008 * var_js_dn2), (assign32030_e47008 * var_js_dn6), (assign32030_e47008 * var_js_dn7), (assign32030_e47008 * var_js_dn10), (assign32030_e47008 * var_js_dn11), (assign32030_e47008 * var_js_dn12), (assign32030_e47008 * var_js_dn17),)
    } else {
        (var_isbs, var_isbs_dn0, var_isbs_dn2, var_isbs_dn6, var_isbs_dn7, var_isbs_dn10, var_isbs_dn11, var_isbs_dn12, var_isbs_dn17,)
    }
};
        var_isbs = assign32030_e47012;
        var_isbs_dn0 = assign32030_e47012_d_n0;
        var_isbs_dn2 = assign32030_e47012_d_n2;
        var_isbs_dn6 = assign32030_e47012_d_n6;
        var_isbs_dn7 = assign32030_e47012_d_n7;
        var_isbs_dn10 = assign32030_e47012_d_n10;
        var_isbs_dn11 = assign32030_e47012_d_n11;
        var_isbs_dn12 = assign32030_e47012_d_n12;
        var_isbs_dn17 = assign32030_e47012_d_n17;

        let (assign32040_e47020, assign32040_e47020_d_n0, assign32040_e47020_d_n2, assign32040_e47020_d_n6, assign32040_e47020_d_n7, assign32040_e47020_d_n10, assign32040_e47020_d_n11, assign32040_e47020_d_n12, assign32040_e47020_d_n17,) = {
    if (var_guard1032 != 0.0) {
        let assign32040_e47016: f64 = (var_w_dios * p.p237);
        let assign32040_e47018: f64 = (assign32040_e47016 * var_js2);
        (assign32040_e47018, (assign32040_e47016 * var_js2_dn0), (assign32040_e47016 * var_js2_dn2), (assign32040_e47016 * var_js2_dn6), (assign32040_e47016 * var_js2_dn7), (assign32040_e47016 * var_js2_dn10), (assign32040_e47016 * var_js2_dn11), (assign32040_e47016 * var_js2_dn12), (assign32040_e47016 * var_js2_dn17),)
    } else {
        (var_isbs2, var_isbs2_dn0, var_isbs2_dn2, var_isbs2_dn6, var_isbs2_dn7, var_isbs2_dn10, var_isbs2_dn11, var_isbs2_dn12, var_isbs2_dn17,)
    }
};
        var_isbs2 = assign32040_e47020;
        var_isbs2_dn0 = assign32040_e47020_d_n0;
        var_isbs2_dn2 = assign32040_e47020_d_n2;
        var_isbs2_dn6 = assign32040_e47020_d_n6;
        var_isbs2_dn7 = assign32040_e47020_d_n7;
        var_isbs2_dn10 = assign32040_e47020_d_n10;
        var_isbs2_dn11 = assign32040_e47020_d_n11;
        var_isbs2_dn12 = assign32040_e47020_d_n12;
        var_isbs2_dn17 = assign32040_e47020_d_n17;

        let (assign32050_e47026, assign32050_e47026_d_n6, assign32050_e47026_d_n7, assign32050_e47026_d_n10, assign32050_e47026_d_n12,) = {
    if (var_guard1032 != 0.0) {
        let assign32050_e47024: f64 = (var_ttemp / var_uc_tnom);
        (assign32050_e47024, 0.0, 0.0, (var_ttemp_dn10 / var_uc_tnom), 0.0,)
    } else {
        (var_t1__blk1034, var_t1__blk1034_dn6, var_t1__blk1034_dn7, var_t1__blk1034_dn10, var_t1__blk1034_dn12,)
    }
};
        var_t1__blk1034 = assign32050_e47026;
        var_t1__blk1034_dn6 = assign32050_e47026_d_n6;
        var_t1__blk1034_dn7 = assign32050_e47026_d_n7;
        var_t1__blk1034_dn10 = assign32050_e47026_d_n10;
        var_t1__blk1034_dn12 = assign32050_e47026_d_n12;

        let (assign32070_e47038, assign32070_e47038_d_n0, assign32070_e47038_d_n2, assign32070_e47038_d_n6, assign32070_e47038_d_n7, assign32070_e47038_d_n10, assign32070_e47038_d_n11, assign32070_e47038_d_n12, assign32070_e47038_d_n17,) = {
    if (var_guard1032 != 0.0) {
        let assign32070_e47036: f64 = (var_isbd + 1e-50);
        (assign32070_e47036, var_isbd_dn0, var_isbd_dn2, var_isbd_dn6, var_isbd_dn7, var_isbd_dn10, var_isbd_dn11, var_isbd_dn12, var_isbd_dn17,)
    } else {
        (var_t2__blk1035, var_t2__blk1035_dn0, var_t2__blk1035_dn2, var_t2__blk1035_dn6, var_t2__blk1035_dn7, var_t2__blk1035_dn10, var_t2__blk1035_dn11, var_t2__blk1035_dn12, var_t2__blk1035_dn17,)
    }
};
        var_t2__blk1035 = assign32070_e47038;
        var_t2__blk1035_dn0 = assign32070_e47038_d_n0;
        var_t2__blk1035_dn2 = assign32070_e47038_d_n2;
        var_t2__blk1035_dn6 = assign32070_e47038_d_n6;
        var_t2__blk1035_dn7 = assign32070_e47038_d_n7;
        var_t2__blk1035_dn10 = assign32070_e47038_d_n10;
        var_t2__blk1035_dn11 = assign32070_e47038_d_n11;
        var_t2__blk1035_dn12 = assign32070_e47038_d_n12;
        var_t2__blk1035_dn17 = assign32070_e47038_d_n17;

        let (assign32090_e47052, assign32090_e47052_d_n10,) = {
    if (var_guard1032 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_vbdt, var_vbdt_dn10,)
    }
};
        var_vbdt = assign32090_e47052;
        var_vbdt_dn10 = assign32090_e47052_d_n10;

        let (assign32100_e47060, assign32100_e47060_d_n10,) = {
    if (var_guard1032 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_vbst, var_vbst_dn10,)
    }
};
        var_vbst = assign32100_e47060;
        var_vbst_dn10 = assign32100_e47060_d_n10;

        let (assign32110_e47066, assign32110_e47066_d_n10,) = {
    if (var_guard1032 != 0.0) {
        let assign32110_e47064: f64 = (p.p174 * var_beta_inv);
        (assign32110_e47064, (p.p174 * var_beta_inv_dn10),)
    } else {
        (var_nvtm, var_nvtm_dn10,)
    }
};
        var_nvtm = assign32110_e47066;
        var_nvtm_dn10 = assign32110_e47066_d_n10;

        let assign32120_e47069: f64 = if var_vbdj < var_vbdt { 1.0 } else { 0.0 };
        var_guard1061 = assign32120_e47069;

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
        *var_guard1028_slot = var_guard1028;
        *var_guard1029_slot = var_guard1029;
        *var_guard1030_slot = var_guard1030;
        *var_guard1031_slot = var_guard1031;
        *var_guard1032_slot = var_guard1032;
        *var_guard1061_slot = var_guard1061;
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
        *var_t1__blk1034_slot = var_t1__blk1034;
        *var_t1__blk1034_dn10_slot = var_t1__blk1034_dn10;
        *var_t1__blk1034_dn12_slot = var_t1__blk1034_dn12;
        *var_t1__blk1034_dn6_slot = var_t1__blk1034_dn6;
        *var_t1__blk1034_dn7_slot = var_t1__blk1034_dn7;
        *var_t1__blk900_slot = var_t1__blk900;
        *var_t1__blk900_dn0_slot = var_t1__blk900_dn0;
        *var_t1__blk900_dn10_slot = var_t1__blk900_dn10;
        *var_t1__blk900_dn11_slot = var_t1__blk900_dn11;
        *var_t1__blk900_dn12_slot = var_t1__blk900_dn12;
        *var_t1__blk900_dn17_slot = var_t1__blk900_dn17;
        *var_t1__blk900_dn2_slot = var_t1__blk900_dn2;
        *var_t1__blk900_dn6_slot = var_t1__blk900_dn6;
        *var_t1__blk900_dn7_slot = var_t1__blk900_dn7;
        *var_t2__blk1035_slot = var_t2__blk1035;
        *var_t2__blk1035_dn0_slot = var_t2__blk1035_dn0;
        *var_t2__blk1035_dn10_slot = var_t2__blk1035_dn10;
        *var_t2__blk1035_dn11_slot = var_t2__blk1035_dn11;
        *var_t2__blk1035_dn12_slot = var_t2__blk1035_dn12;
        *var_t2__blk1035_dn17_slot = var_t2__blk1035_dn17;
        *var_t2__blk1035_dn2_slot = var_t2__blk1035_dn2;
        *var_t2__blk1035_dn6_slot = var_t2__blk1035_dn6;
        *var_t2__blk1035_dn7_slot = var_t2__blk1035_dn7;
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
        var_guard1032: f64,
        var_guard1061: f64,
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
        var_arg__blk1059_slot: &mut f64,
        var_arg__blk1059_dn12_slot: &mut f64,
        var_arg__blk1059_dn6_slot: &mut f64,
        var_arg__blk1059_dn7_slot: &mut f64,
        var_czbd_slot: &mut f64,
        var_czbs_slot: &mut f64,
        var_czbssw_slot: &mut f64,
        var_czbsswg_slot: &mut f64,
        var_guard1062_slot: &mut f64,
        var_guard1063_slot: &mut f64,
        var_guard1064_slot: &mut f64,
        var_guard1065_slot: &mut f64,
        var_guard1066_slot: &mut f64,
        var_guard1067_slot: &mut f64,
        var_guard1068_slot: &mut f64,
        var_guard1069_slot: &mut f64,
        var_guard1070_slot: &mut f64,
        var_guard1071_slot: &mut f64,
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
        var_t1__blk1034_slot: &mut f64,
        var_t1__blk1034_dn10_slot: &mut f64,
        var_t1__blk1034_dn12_slot: &mut f64,
        var_t1__blk1034_dn6_slot: &mut f64,
        var_t1__blk1034_dn7_slot: &mut f64,
        var_xp_max_slot: &mut f64,
    ) {
        let mut var_arg__blk1059: f64 = *var_arg__blk1059_slot;
        let mut var_arg__blk1059_dn12: f64 = *var_arg__blk1059_dn12_slot;
        let mut var_arg__blk1059_dn6: f64 = *var_arg__blk1059_dn6_slot;
        let mut var_arg__blk1059_dn7: f64 = *var_arg__blk1059_dn7_slot;
        let mut var_czbd: f64 = *var_czbd_slot;
        let mut var_czbs: f64 = *var_czbs_slot;
        let mut var_czbssw: f64 = *var_czbssw_slot;
        let mut var_czbsswg: f64 = *var_czbsswg_slot;
        let mut var_guard1062: f64 = *var_guard1062_slot;
        let mut var_guard1063: f64 = *var_guard1063_slot;
        let mut var_guard1064: f64 = *var_guard1064_slot;
        let mut var_guard1065: f64 = *var_guard1065_slot;
        let mut var_guard1066: f64 = *var_guard1066_slot;
        let mut var_guard1067: f64 = *var_guard1067_slot;
        let mut var_guard1068: f64 = *var_guard1068_slot;
        let mut var_guard1069: f64 = *var_guard1069_slot;
        let mut var_guard1070: f64 = *var_guard1070_slot;
        let mut var_guard1071: f64 = *var_guard1071_slot;
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
        let mut var_t1__blk1034: f64 = *var_t1__blk1034_slot;
        let mut var_t1__blk1034_dn10: f64 = *var_t1__blk1034_dn10_slot;
        let mut var_t1__blk1034_dn12: f64 = *var_t1__blk1034_dn12_slot;
        let mut var_t1__blk1034_dn6: f64 = *var_t1__blk1034_dn6_slot;
        let mut var_t1__blk1034_dn7: f64 = *var_t1__blk1034_dn7_slot;
        let mut var_xp_max: f64 = *var_xp_max_slot;

        let (assign32130_e47078, assign32130_e47078_d_n6, assign32130_e47078_d_n7, assign32130_e47078_d_n10, assign32130_e47078_d_n12,) = {
    if ((var_guard1032 != 0.0) && (var_guard1061 != 0.0)) {
        let assign32130_e47075: f64 = (var_vbdj / var_nvtm);
        let assign32130_e47076: f64 = (assign32130_e47075).exp();
        (assign32130_e47076, (assign32130_e47076 * (var_vbdj_dn6 / var_nvtm)), 0.0, (assign32130_e47076 * (-((var_vbdj * var_nvtm_dn10) / (var_nvtm * var_nvtm)))), (assign32130_e47076 * (var_vbdj_dn12 / var_nvtm)),)
    } else {
        (var_t1__blk1034, var_t1__blk1034_dn6, var_t1__blk1034_dn7, var_t1__blk1034_dn10, var_t1__blk1034_dn12,)
    }
};
        var_t1__blk1034 = assign32130_e47078;
        var_t1__blk1034_dn6 = assign32130_e47078_d_n6;
        var_t1__blk1034_dn7 = assign32130_e47078_d_n7;
        var_t1__blk1034_dn10 = assign32130_e47078_d_n10;
        var_t1__blk1034_dn12 = assign32130_e47078_d_n12;

        let (assign32140_e47088, assign32140_e47088_d_n0, assign32140_e47088_d_n2, assign32140_e47088_d_n6, assign32140_e47088_d_n7, assign32140_e47088_d_n10, assign32140_e47088_d_n11, assign32140_e47088_d_n12, assign32140_e47088_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1061 != 0.0)) {
        let assign32140_e47085: f64 = (var_t1__blk1034 - 1.0);
        let assign32140_e47086: f64 = (var_isbd * assign32140_e47085);
        (assign32140_e47086, (var_isbd_dn0 * assign32140_e47085), (var_isbd_dn2 * assign32140_e47085), ((var_isbd_dn6 * assign32140_e47085) + (var_isbd * var_t1__blk1034_dn6)), ((var_isbd_dn7 * assign32140_e47085) + (var_isbd * var_t1__blk1034_dn7)), ((var_isbd_dn10 * assign32140_e47085) + (var_isbd * var_t1__blk1034_dn10)), (var_isbd_dn11 * assign32140_e47085), ((var_isbd_dn12 * assign32140_e47085) + (var_isbd * var_t1__blk1034_dn12)), (var_isbd_dn17 * assign32140_e47085),)
    } else {
        (var_ibd, var_ibd_dn0, var_ibd_dn2, var_ibd_dn6, var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, var_ibd_dn12, var_ibd_dn17,)
    }
};
        var_ibd = assign32140_e47088;
        var_ibd_dn0 = assign32140_e47088_d_n0;
        var_ibd_dn2 = assign32140_e47088_d_n2;
        var_ibd_dn6 = assign32140_e47088_d_n6;
        var_ibd_dn7 = assign32140_e47088_d_n7;
        var_ibd_dn10 = assign32140_e47088_d_n10;
        var_ibd_dn11 = assign32140_e47088_d_n11;
        var_ibd_dn12 = assign32140_e47088_d_n12;
        var_ibd_dn17 = assign32140_e47088_d_n17;

        let (assign32150_e47098, assign32150_e47098_d_n6, assign32150_e47098_d_n7, assign32150_e47098_d_n10, assign32150_e47098_d_n12,) = {
    if ((var_guard1032 != 0.0) && (var_guard1061 == 0.0)) {
        let assign32150_e47095: f64 = (var_vbdt / var_nvtm);
        let assign32150_e47096: f64 = (assign32150_e47095).exp();
        (assign32150_e47096, 0.0, 0.0, (assign32150_e47096 * (((var_vbdt_dn10 * var_nvtm) - (var_vbdt * var_nvtm_dn10)) / (var_nvtm * var_nvtm))), 0.0,)
    } else {
        (var_t1__blk1034, var_t1__blk1034_dn6, var_t1__blk1034_dn7, var_t1__blk1034_dn10, var_t1__blk1034_dn12,)
    }
};
        var_t1__blk1034 = assign32150_e47098;
        var_t1__blk1034_dn6 = assign32150_e47098_d_n6;
        var_t1__blk1034_dn7 = assign32150_e47098_d_n7;
        var_t1__blk1034_dn10 = assign32150_e47098_d_n10;
        var_t1__blk1034_dn12 = assign32150_e47098_d_n12;

        let (assign32160_e47119, assign32160_e47119_d_n0, assign32160_e47119_d_n2, assign32160_e47119_d_n6, assign32160_e47119_d_n7, assign32160_e47119_d_n10, assign32160_e47119_d_n11, assign32160_e47119_d_n12, assign32160_e47119_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1061 == 0.0)) {
        let assign32160_e47106: f64 = (var_t1__blk1034 - 1.0);
        let assign32160_e47107: f64 = (var_isbd * assign32160_e47106);
        let assign32160_e47110: f64 = (var_isbd / var_nvtm);
        let assign32160_e47112: f64 = (assign32160_e47110 * var_t1__blk1034);
        let assign32160_e47115: f64 = (var_vbdj - var_vbdt);
        let assign32160_e47116: f64 = (assign32160_e47112 * assign32160_e47115);
        let assign32160_e47117: f64 = (assign32160_e47107 + assign32160_e47116);
        (assign32160_e47117, ((var_isbd_dn0 * assign32160_e47106) + (((var_isbd_dn0 / var_nvtm) * var_t1__blk1034) * assign32160_e47115)), ((var_isbd_dn2 * assign32160_e47106) + (((var_isbd_dn2 / var_nvtm) * var_t1__blk1034) * assign32160_e47115)), (((var_isbd_dn6 * assign32160_e47106) + (var_isbd * var_t1__blk1034_dn6)) + (((((var_isbd_dn6 / var_nvtm) * var_t1__blk1034) + (assign32160_e47110 * var_t1__blk1034_dn6)) * assign32160_e47115) + (assign32160_e47112 * var_vbdj_dn6))), (((var_isbd_dn7 * assign32160_e47106) + (var_isbd * var_t1__blk1034_dn7)) + ((((var_isbd_dn7 / var_nvtm) * var_t1__blk1034) + (assign32160_e47110 * var_t1__blk1034_dn7)) * assign32160_e47115)), (((var_isbd_dn10 * assign32160_e47106) + (var_isbd * var_t1__blk1034_dn10)) + (((((((var_isbd_dn10 * var_nvtm) - (var_isbd * var_nvtm_dn10)) / (var_nvtm * var_nvtm)) * var_t1__blk1034) + (assign32160_e47110 * var_t1__blk1034_dn10)) * assign32160_e47115) + (assign32160_e47112 * (-var_vbdt_dn10)))), ((var_isbd_dn11 * assign32160_e47106) + (((var_isbd_dn11 / var_nvtm) * var_t1__blk1034) * assign32160_e47115)), (((var_isbd_dn12 * assign32160_e47106) + (var_isbd * var_t1__blk1034_dn12)) + (((((var_isbd_dn12 / var_nvtm) * var_t1__blk1034) + (assign32160_e47110 * var_t1__blk1034_dn12)) * assign32160_e47115) + (assign32160_e47112 * var_vbdj_dn12))), ((var_isbd_dn17 * assign32160_e47106) + (((var_isbd_dn17 / var_nvtm) * var_t1__blk1034) * assign32160_e47115)),)
    } else {
        (var_ibd, var_ibd_dn0, var_ibd_dn2, var_ibd_dn6, var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, var_ibd_dn12, var_ibd_dn17,)
    }
};
        var_ibd = assign32160_e47119;
        var_ibd_dn0 = assign32160_e47119_d_n0;
        var_ibd_dn2 = assign32160_e47119_d_n2;
        var_ibd_dn6 = assign32160_e47119_d_n6;
        var_ibd_dn7 = assign32160_e47119_d_n7;
        var_ibd_dn10 = assign32160_e47119_d_n10;
        var_ibd_dn11 = assign32160_e47119_d_n11;
        var_ibd_dn12 = assign32160_e47119_d_n12;
        var_ibd_dn17 = assign32160_e47119_d_n17;

        let (assign32170_e47129, assign32170_e47129_d_n0, assign32170_e47129_d_n2, assign32170_e47129_d_n6, assign32170_e47129_d_n7, assign32170_e47129_d_n10, assign32170_e47129_d_n11, assign32170_e47129_d_n12, assign32170_e47129_d_n17,) = {
    if (var_guard1032 != 0.0) {
        let assign32170_e47124: f64 = (p.p178 * var_vbdj);
        let assign32170_e47126: f64 = (assign32170_e47124 * var_isbd2);
        let assign32170_e47127: f64 = (var_ibd + assign32170_e47126);
        (assign32170_e47127, (var_ibd_dn0 + (assign32170_e47124 * var_isbd2_dn0)), (var_ibd_dn2 + (assign32170_e47124 * var_isbd2_dn2)), (var_ibd_dn6 + (((p.p178 * var_vbdj_dn6) * var_isbd2) + (assign32170_e47124 * var_isbd2_dn6))), (var_ibd_dn7 + (assign32170_e47124 * var_isbd2_dn7)), (var_ibd_dn10 + (assign32170_e47124 * var_isbd2_dn10)), (var_ibd_dn11 + (assign32170_e47124 * var_isbd2_dn11)), (var_ibd_dn12 + (((p.p178 * var_vbdj_dn12) * var_isbd2) + (assign32170_e47124 * var_isbd2_dn12))), (var_ibd_dn17 + (assign32170_e47124 * var_isbd2_dn17)),)
    } else {
        (var_ibd, var_ibd_dn0, var_ibd_dn2, var_ibd_dn6, var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, var_ibd_dn12, var_ibd_dn17,)
    }
};
        var_ibd = assign32170_e47129;
        var_ibd_dn0 = assign32170_e47129_d_n0;
        var_ibd_dn2 = assign32170_e47129_d_n2;
        var_ibd_dn6 = assign32170_e47129_d_n6;
        var_ibd_dn7 = assign32170_e47129_d_n7;
        var_ibd_dn10 = assign32170_e47129_d_n10;
        var_ibd_dn11 = assign32170_e47129_d_n11;
        var_ibd_dn12 = assign32170_e47129_d_n12;
        var_ibd_dn17 = assign32170_e47129_d_n17;

        let assign32180_e47132: f64 = if var_vbsj < var_vbst { 1.0 } else { 0.0 };
        var_guard1062 = assign32180_e47132;

        let (assign32190_e47141, assign32190_e47141_d_n6, assign32190_e47141_d_n7, assign32190_e47141_d_n10, assign32190_e47141_d_n12,) = {
    if ((var_guard1032 != 0.0) && (var_guard1062 != 0.0)) {
        let assign32190_e47138: f64 = (var_vbsj / var_nvtm);
        let assign32190_e47139: f64 = (assign32190_e47138).exp();
        (assign32190_e47139, 0.0, (assign32190_e47139 * (var_vbsj_dn7 / var_nvtm)), (assign32190_e47139 * (-((var_vbsj * var_nvtm_dn10) / (var_nvtm * var_nvtm)))), (assign32190_e47139 * (var_vbsj_dn12 / var_nvtm)),)
    } else {
        (var_t1__blk1034, var_t1__blk1034_dn6, var_t1__blk1034_dn7, var_t1__blk1034_dn10, var_t1__blk1034_dn12,)
    }
};
        var_t1__blk1034 = assign32190_e47141;
        var_t1__blk1034_dn6 = assign32190_e47141_d_n6;
        var_t1__blk1034_dn7 = assign32190_e47141_d_n7;
        var_t1__blk1034_dn10 = assign32190_e47141_d_n10;
        var_t1__blk1034_dn12 = assign32190_e47141_d_n12;

        let (assign32200_e47151, assign32200_e47151_d_n0, assign32200_e47151_d_n2, assign32200_e47151_d_n6, assign32200_e47151_d_n7, assign32200_e47151_d_n10, assign32200_e47151_d_n11, assign32200_e47151_d_n12, assign32200_e47151_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1062 != 0.0)) {
        let assign32200_e47148: f64 = (var_t1__blk1034 - 1.0);
        let assign32200_e47149: f64 = (var_isbs * assign32200_e47148);
        (assign32200_e47149, (var_isbs_dn0 * assign32200_e47148), (var_isbs_dn2 * assign32200_e47148), ((var_isbs_dn6 * assign32200_e47148) + (var_isbs * var_t1__blk1034_dn6)), ((var_isbs_dn7 * assign32200_e47148) + (var_isbs * var_t1__blk1034_dn7)), ((var_isbs_dn10 * assign32200_e47148) + (var_isbs * var_t1__blk1034_dn10)), (var_isbs_dn11 * assign32200_e47148), ((var_isbs_dn12 * assign32200_e47148) + (var_isbs * var_t1__blk1034_dn12)), (var_isbs_dn17 * assign32200_e47148),)
    } else {
        (var_ibs, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, var_ibs_dn7, var_ibs_dn10, var_ibs_dn11, var_ibs_dn12, var_ibs_dn17,)
    }
};
        var_ibs = assign32200_e47151;
        var_ibs_dn0 = assign32200_e47151_d_n0;
        var_ibs_dn2 = assign32200_e47151_d_n2;
        var_ibs_dn6 = assign32200_e47151_d_n6;
        var_ibs_dn7 = assign32200_e47151_d_n7;
        var_ibs_dn10 = assign32200_e47151_d_n10;
        var_ibs_dn11 = assign32200_e47151_d_n11;
        var_ibs_dn12 = assign32200_e47151_d_n12;
        var_ibs_dn17 = assign32200_e47151_d_n17;

        let (assign32210_e47161, assign32210_e47161_d_n6, assign32210_e47161_d_n7, assign32210_e47161_d_n10, assign32210_e47161_d_n12,) = {
    if ((var_guard1032 != 0.0) && (var_guard1062 == 0.0)) {
        let assign32210_e47158: f64 = (var_vbst / var_nvtm);
        let assign32210_e47159: f64 = (assign32210_e47158).exp();
        (assign32210_e47159, 0.0, 0.0, (assign32210_e47159 * (((var_vbst_dn10 * var_nvtm) - (var_vbst * var_nvtm_dn10)) / (var_nvtm * var_nvtm))), 0.0,)
    } else {
        (var_t1__blk1034, var_t1__blk1034_dn6, var_t1__blk1034_dn7, var_t1__blk1034_dn10, var_t1__blk1034_dn12,)
    }
};
        var_t1__blk1034 = assign32210_e47161;
        var_t1__blk1034_dn6 = assign32210_e47161_d_n6;
        var_t1__blk1034_dn7 = assign32210_e47161_d_n7;
        var_t1__blk1034_dn10 = assign32210_e47161_d_n10;
        var_t1__blk1034_dn12 = assign32210_e47161_d_n12;

        let (assign32220_e47182, assign32220_e47182_d_n0, assign32220_e47182_d_n2, assign32220_e47182_d_n6, assign32220_e47182_d_n7, assign32220_e47182_d_n10, assign32220_e47182_d_n11, assign32220_e47182_d_n12, assign32220_e47182_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1062 == 0.0)) {
        let assign32220_e47169: f64 = (var_t1__blk1034 - 1.0);
        let assign32220_e47170: f64 = (var_isbs * assign32220_e47169);
        let assign32220_e47173: f64 = (var_isbs / var_nvtm);
        let assign32220_e47175: f64 = (assign32220_e47173 * var_t1__blk1034);
        let assign32220_e47178: f64 = (var_vbsj - var_vbst);
        let assign32220_e47179: f64 = (assign32220_e47175 * assign32220_e47178);
        let assign32220_e47180: f64 = (assign32220_e47170 + assign32220_e47179);
        (assign32220_e47180, ((var_isbs_dn0 * assign32220_e47169) + (((var_isbs_dn0 / var_nvtm) * var_t1__blk1034) * assign32220_e47178)), ((var_isbs_dn2 * assign32220_e47169) + (((var_isbs_dn2 / var_nvtm) * var_t1__blk1034) * assign32220_e47178)), (((var_isbs_dn6 * assign32220_e47169) + (var_isbs * var_t1__blk1034_dn6)) + ((((var_isbs_dn6 / var_nvtm) * var_t1__blk1034) + (assign32220_e47173 * var_t1__blk1034_dn6)) * assign32220_e47178)), (((var_isbs_dn7 * assign32220_e47169) + (var_isbs * var_t1__blk1034_dn7)) + (((((var_isbs_dn7 / var_nvtm) * var_t1__blk1034) + (assign32220_e47173 * var_t1__blk1034_dn7)) * assign32220_e47178) + (assign32220_e47175 * var_vbsj_dn7))), (((var_isbs_dn10 * assign32220_e47169) + (var_isbs * var_t1__blk1034_dn10)) + (((((((var_isbs_dn10 * var_nvtm) - (var_isbs * var_nvtm_dn10)) / (var_nvtm * var_nvtm)) * var_t1__blk1034) + (assign32220_e47173 * var_t1__blk1034_dn10)) * assign32220_e47178) + (assign32220_e47175 * (-var_vbst_dn10)))), ((var_isbs_dn11 * assign32220_e47169) + (((var_isbs_dn11 / var_nvtm) * var_t1__blk1034) * assign32220_e47178)), (((var_isbs_dn12 * assign32220_e47169) + (var_isbs * var_t1__blk1034_dn12)) + (((((var_isbs_dn12 / var_nvtm) * var_t1__blk1034) + (assign32220_e47173 * var_t1__blk1034_dn12)) * assign32220_e47178) + (assign32220_e47175 * var_vbsj_dn12))), ((var_isbs_dn17 * assign32220_e47169) + (((var_isbs_dn17 / var_nvtm) * var_t1__blk1034) * assign32220_e47178)),)
    } else {
        (var_ibs, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, var_ibs_dn7, var_ibs_dn10, var_ibs_dn11, var_ibs_dn12, var_ibs_dn17,)
    }
};
        var_ibs = assign32220_e47182;
        var_ibs_dn0 = assign32220_e47182_d_n0;
        var_ibs_dn2 = assign32220_e47182_d_n2;
        var_ibs_dn6 = assign32220_e47182_d_n6;
        var_ibs_dn7 = assign32220_e47182_d_n7;
        var_ibs_dn10 = assign32220_e47182_d_n10;
        var_ibs_dn11 = assign32220_e47182_d_n11;
        var_ibs_dn12 = assign32220_e47182_d_n12;
        var_ibs_dn17 = assign32220_e47182_d_n17;

        let (assign32230_e47192, assign32230_e47192_d_n0, assign32230_e47192_d_n2, assign32230_e47192_d_n6, assign32230_e47192_d_n7, assign32230_e47192_d_n10, assign32230_e47192_d_n11, assign32230_e47192_d_n12, assign32230_e47192_d_n17,) = {
    if (var_guard1032 != 0.0) {
        let assign32230_e47187: f64 = (p.p178 * var_vbsj);
        let assign32230_e47189: f64 = (assign32230_e47187 * var_isbs2);
        let assign32230_e47190: f64 = (var_ibs + assign32230_e47189);
        (assign32230_e47190, (var_ibs_dn0 + (assign32230_e47187 * var_isbs2_dn0)), (var_ibs_dn2 + (assign32230_e47187 * var_isbs2_dn2)), (var_ibs_dn6 + (assign32230_e47187 * var_isbs2_dn6)), (var_ibs_dn7 + (((p.p178 * var_vbsj_dn7) * var_isbs2) + (assign32230_e47187 * var_isbs2_dn7))), (var_ibs_dn10 + (assign32230_e47187 * var_isbs2_dn10)), (var_ibs_dn11 + (assign32230_e47187 * var_isbs2_dn11)), (var_ibs_dn12 + (((p.p178 * var_vbsj_dn12) * var_isbs2) + (assign32230_e47187 * var_isbs2_dn12))), (var_ibs_dn17 + (assign32230_e47187 * var_isbs2_dn17)),)
    } else {
        (var_ibs, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, var_ibs_dn7, var_ibs_dn10, var_ibs_dn11, var_ibs_dn12, var_ibs_dn17,)
    }
};
        var_ibs = assign32230_e47192;
        var_ibs_dn0 = assign32230_e47192_d_n0;
        var_ibs_dn2 = assign32230_e47192_d_n2;
        var_ibs_dn6 = assign32230_e47192_d_n6;
        var_ibs_dn7 = assign32230_e47192_d_n7;
        var_ibs_dn10 = assign32230_e47192_d_n10;
        var_ibs_dn11 = assign32230_e47192_d_n11;
        var_ibs_dn12 = assign32230_e47192_d_n12;
        var_ibs_dn17 = assign32230_e47192_d_n17;

        let (assign32240_e47200, assign32240_e47200_d_n0, assign32240_e47200_d_n2, assign32240_e47200_d_n6, assign32240_e47200_d_n7, assign32240_e47200_d_n10, assign32240_e47200_d_n11, assign32240_e47200_d_n12, assign32240_e47200_d_n17,) = {
    if (var_guard1032 != 0.0) {
        let assign32240_e47197: f64 = (var_gjmin * var_vbdj);
        let assign32240_e47198: f64 = (var_ibd + assign32240_e47197);
        (assign32240_e47198, var_ibd_dn0, var_ibd_dn2, (var_ibd_dn6 + (var_gjmin * var_vbdj_dn6)), var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, (var_ibd_dn12 + (var_gjmin * var_vbdj_dn12)), var_ibd_dn17,)
    } else {
        (var_ibd, var_ibd_dn0, var_ibd_dn2, var_ibd_dn6, var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, var_ibd_dn12, var_ibd_dn17,)
    }
};
        var_ibd = assign32240_e47200;
        var_ibd_dn0 = assign32240_e47200_d_n0;
        var_ibd_dn2 = assign32240_e47200_d_n2;
        var_ibd_dn6 = assign32240_e47200_d_n6;
        var_ibd_dn7 = assign32240_e47200_d_n7;
        var_ibd_dn10 = assign32240_e47200_d_n10;
        var_ibd_dn11 = assign32240_e47200_d_n11;
        var_ibd_dn12 = assign32240_e47200_d_n12;
        var_ibd_dn17 = assign32240_e47200_d_n17;

        let (assign32250_e47208, assign32250_e47208_d_n0, assign32250_e47208_d_n2, assign32250_e47208_d_n6, assign32250_e47208_d_n7, assign32250_e47208_d_n10, assign32250_e47208_d_n11, assign32250_e47208_d_n12, assign32250_e47208_d_n17,) = {
    if (var_guard1032 != 0.0) {
        let assign32250_e47205: f64 = (var_gjmin * var_vbsj);
        let assign32250_e47206: f64 = (var_ibs + assign32250_e47205);
        (assign32250_e47206, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, (var_ibs_dn7 + (var_gjmin * var_vbsj_dn7)), var_ibs_dn10, var_ibs_dn11, (var_ibs_dn12 + (var_gjmin * var_vbsj_dn12)), var_ibs_dn17,)
    } else {
        (var_ibs, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, var_ibs_dn7, var_ibs_dn10, var_ibs_dn11, var_ibs_dn12, var_ibs_dn17,)
    }
};
        var_ibs = assign32250_e47208;
        var_ibs_dn0 = assign32250_e47208_d_n0;
        var_ibs_dn2 = assign32250_e47208_d_n2;
        var_ibs_dn6 = assign32250_e47208_d_n6;
        var_ibs_dn7 = assign32250_e47208_d_n7;
        var_ibs_dn10 = assign32250_e47208_d_n10;
        var_ibs_dn11 = assign32250_e47208_d_n11;
        var_ibs_dn12 = assign32250_e47208_d_n12;
        var_ibs_dn17 = assign32250_e47208_d_n17;

        let (assign32260_e47214,) = {
    if (var_guard1032 != 0.0) {
        let assign32260_e47212: f64 = (p.p179 * p.p2);
        (assign32260_e47212,)
    } else {
        (var_czbd,)
    }
};
        var_czbd = assign32260_e47214;

        let (assign32270_e47220,) = {
    if (var_guard1032 != 0.0) {
        let assign32270_e47218: f64 = (p.p179 * p.p3);
        (assign32270_e47218,)
    } else {
        (var_czbs,)
    }
};
        var_czbs = assign32270_e47220;

        let (assign32280_e47226,) = {
    if (var_guard1032 != 0.0) {
        let assign32280_e47224: f64 = (p.p237 - p.p238);
        (assign32280_e47224,)
    } else {
        (var_xp_max,)
    }
};
        var_xp_max = assign32280_e47226;

        let assign32290_e47229: f64 = if var_xp_max <= 0.0 { 1.0 } else { 0.0 };
        var_guard1063 = assign32290_e47229;

        let (assign32300_e47235,) = {
    if ((var_guard1032 != 0.0) && (var_guard1063 != 0.0)) {
        (0.0,)
    } else {
        (var_czbd,)
    }
};
        var_czbd = assign32300_e47235;

        let (assign32310_e47241,) = {
    if ((var_guard1032 != 0.0) && (var_guard1063 != 0.0)) {
        (0.0,)
    } else {
        (var_czbs,)
    }
};
        var_czbs = assign32310_e47241;

        let assign32320_e47244: f64 = if p.p5 > var_w_dioscv { 1.0 } else { 0.0 };
        var_guard1064 = assign32320_e47244;

        let (assign32330_e47254,) = {
    if ((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) {
        let assign32330_e47251: f64 = (p.p5 - var_w_dioscv);
        let assign32330_e47252: f64 = (p.p180 * assign32330_e47251);
        (assign32330_e47252,)
    } else {
        (var_czbssw,)
    }
};
        var_czbssw = assign32330_e47254;

        let (assign32340_e47262,) = {
    if ((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) {
        let assign32340_e47260: f64 = (p.p181 * var_w_dioscv);
        (assign32340_e47260,)
    } else {
        (var_czbsswg,)
    }
};
        var_czbsswg = assign32340_e47262;

        let assign32350_e47265: f64 = if var_vbsj < 0.0 { 1.0 } else { 0.0 };
        var_guard1065 = assign32350_e47265;

        let assign32360_e47268: f64 = if var_czbs > 0.0 { 1.0 } else { 0.0 };
        var_guard1066 = assign32360_e47268;

        let (assign32370_e47282, assign32370_e47282_d_n6, assign32370_e47282_d_n7, assign32370_e47282_d_n12,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1066 != 0.0)) {
        let assign32370_e47279: f64 = (var_vbsj / p.p185);
        let assign32370_e47280: f64 = (1.0 - assign32370_e47279);
        (assign32370_e47280, 0.0, (-(var_vbsj_dn7 / p.p185)), (-(var_vbsj_dn12 / p.p185)),)
    } else {
        (var_arg__blk1059, var_arg__blk1059_dn6, var_arg__blk1059_dn7, var_arg__blk1059_dn12,)
    }
};
        var_arg__blk1059 = assign32370_e47282;
        var_arg__blk1059_dn6 = assign32370_e47282_d_n6;
        var_arg__blk1059_dn7 = assign32370_e47282_d_n7;
        var_arg__blk1059_dn12 = assign32370_e47282_d_n12;

        let assign32380_e47285: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        var_guard1067 = assign32380_e47285;

        let (assign32390_e47300, assign32390_e47300_d_n6, assign32390_e47300_d_n7, assign32390_e47300_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1066 != 0.0)) && (var_guard1067 != 0.0)) {
        let assign32390_e47297: f64 = (var_arg__blk1059).sqrt();
        let assign32390_e47298: f64 = (1.0 / assign32390_e47297);
        (assign32390_e47298, (-((var_arg__blk1059_dn6 / (2.0 * assign32390_e47297)) / (assign32390_e47297 * assign32390_e47297))), (-((var_arg__blk1059_dn7 / (2.0 * assign32390_e47297)) / (assign32390_e47297 * assign32390_e47297))), (-((var_arg__blk1059_dn12 / (2.0 * assign32390_e47297)) / (assign32390_e47297 * assign32390_e47297))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32390_e47300;
        var_sarg_dn6 = assign32390_e47300_d_n6;
        var_sarg_dn7 = assign32390_e47300_d_n7;
        var_sarg_dn12 = assign32390_e47300_d_n12;

        let (assign32400_e47316, assign32400_e47316_d_n6, assign32400_e47316_d_n7, assign32400_e47316_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1066 != 0.0)) && (var_guard1067 == 0.0)) {
        let assign32400_e47313: f64 = (-p.p182);
        let assign32400_e47314: f64 = (var_arg__blk1059).powf(assign32400_e47313);
        (assign32400_e47314, if 0.0 == 0.0 && ((assign32400_e47313) as f64).is_finite() && ((assign32400_e47313) as f64).fract() == 0.0 { if assign32400_e47313 == 0.0 { 0.0 } else { (assign32400_e47313 * ((var_arg__blk1059).powf(assign32400_e47313 - 1.0) * var_arg__blk1059_dn6)) } } else { (assign32400_e47314 * (assign32400_e47313 * (var_arg__blk1059_dn6 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32400_e47313) as f64).is_finite() && ((assign32400_e47313) as f64).fract() == 0.0 { if assign32400_e47313 == 0.0 { 0.0 } else { (assign32400_e47313 * ((var_arg__blk1059).powf(assign32400_e47313 - 1.0) * var_arg__blk1059_dn7)) } } else { (assign32400_e47314 * (assign32400_e47313 * (var_arg__blk1059_dn7 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32400_e47313) as f64).is_finite() && ((assign32400_e47313) as f64).fract() == 0.0 { if assign32400_e47313 == 0.0 { 0.0 } else { (assign32400_e47313 * ((var_arg__blk1059).powf(assign32400_e47313 - 1.0) * var_arg__blk1059_dn12)) } } else { (assign32400_e47314 * (assign32400_e47313 * (var_arg__blk1059_dn12 / var_arg__blk1059))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32400_e47316;
        var_sarg_dn6 = assign32400_e47316_d_n6;
        var_sarg_dn7 = assign32400_e47316_d_n7;
        var_sarg_dn12 = assign32400_e47316_d_n12;

        let (assign32410_e47338, assign32410_e47338_d_n0, assign32410_e47338_d_n2, assign32410_e47338_d_n6, assign32410_e47338_d_n7, assign32410_e47338_d_n10, assign32410_e47338_d_n11, assign32410_e47338_d_n12, assign32410_e47338_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1066 != 0.0)) {
        let assign32410_e47326: f64 = (p.p185 * var_czbs);
        let assign32410_e47330: f64 = (var_arg__blk1059 * var_sarg);
        let assign32410_e47331: f64 = (1.0 - assign32410_e47330);
        let assign32410_e47332: f64 = (assign32410_e47326 * assign32410_e47331);
        let assign32410_e47335: f64 = (1.0 - p.p182);
        let assign32410_e47336: f64 = (assign32410_e47332 / assign32410_e47335);
        (assign32410_e47336, 0.0, 0.0, ((assign32410_e47326 * (-((var_arg__blk1059_dn6 * var_sarg) + (var_arg__blk1059 * var_sarg_dn6)))) / assign32410_e47335), ((assign32410_e47326 * (-((var_arg__blk1059_dn7 * var_sarg) + (var_arg__blk1059 * var_sarg_dn7)))) / assign32410_e47335), 0.0, 0.0, ((assign32410_e47326 * (-((var_arg__blk1059_dn12 * var_sarg) + (var_arg__blk1059 * var_sarg_dn12)))) / assign32410_e47335), 0.0,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32410_e47338;
        var_qbs_dn0 = assign32410_e47338_d_n0;
        var_qbs_dn2 = assign32410_e47338_d_n2;
        var_qbs_dn6 = assign32410_e47338_d_n6;
        var_qbs_dn7 = assign32410_e47338_d_n7;
        var_qbs_dn10 = assign32410_e47338_d_n10;
        var_qbs_dn11 = assign32410_e47338_d_n11;
        var_qbs_dn12 = assign32410_e47338_d_n12;
        var_qbs_dn17 = assign32410_e47338_d_n17;

        let (assign32420_e47349, assign32420_e47349_d_n0, assign32420_e47349_d_n2, assign32420_e47349_d_n6, assign32420_e47349_d_n7, assign32420_e47349_d_n10, assign32420_e47349_d_n11, assign32420_e47349_d_n12, assign32420_e47349_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1066 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32420_e47349;
        var_qbs_dn0 = assign32420_e47349_d_n0;
        var_qbs_dn2 = assign32420_e47349_d_n2;
        var_qbs_dn6 = assign32420_e47349_d_n6;
        var_qbs_dn7 = assign32420_e47349_d_n7;
        var_qbs_dn10 = assign32420_e47349_d_n10;
        var_qbs_dn11 = assign32420_e47349_d_n11;
        var_qbs_dn12 = assign32420_e47349_d_n12;
        var_qbs_dn17 = assign32420_e47349_d_n17;

        let assign32430_e47352: f64 = if var_czbssw > 0.0 { 1.0 } else { 0.0 };
        var_guard1068 = assign32430_e47352;

        let (assign32440_e47366, assign32440_e47366_d_n6, assign32440_e47366_d_n7, assign32440_e47366_d_n12,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1068 != 0.0)) {
        let assign32440_e47363: f64 = (var_vbsj / p.p186);
        let assign32440_e47364: f64 = (1.0 - assign32440_e47363);
        (assign32440_e47364, 0.0, (-(var_vbsj_dn7 / p.p186)), (-(var_vbsj_dn12 / p.p186)),)
    } else {
        (var_arg__blk1059, var_arg__blk1059_dn6, var_arg__blk1059_dn7, var_arg__blk1059_dn12,)
    }
};
        var_arg__blk1059 = assign32440_e47366;
        var_arg__blk1059_dn6 = assign32440_e47366_d_n6;
        var_arg__blk1059_dn7 = assign32440_e47366_d_n7;
        var_arg__blk1059_dn12 = assign32440_e47366_d_n12;

        let assign32450_e47369: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        var_guard1069 = assign32450_e47369;

        let (assign32460_e47384, assign32460_e47384_d_n6, assign32460_e47384_d_n7, assign32460_e47384_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1068 != 0.0)) && (var_guard1069 != 0.0)) {
        let assign32460_e47381: f64 = (var_arg__blk1059).sqrt();
        let assign32460_e47382: f64 = (1.0 / assign32460_e47381);
        (assign32460_e47382, (-((var_arg__blk1059_dn6 / (2.0 * assign32460_e47381)) / (assign32460_e47381 * assign32460_e47381))), (-((var_arg__blk1059_dn7 / (2.0 * assign32460_e47381)) / (assign32460_e47381 * assign32460_e47381))), (-((var_arg__blk1059_dn12 / (2.0 * assign32460_e47381)) / (assign32460_e47381 * assign32460_e47381))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32460_e47384;
        var_sarg_dn6 = assign32460_e47384_d_n6;
        var_sarg_dn7 = assign32460_e47384_d_n7;
        var_sarg_dn12 = assign32460_e47384_d_n12;

        let (assign32470_e47400, assign32470_e47400_d_n6, assign32470_e47400_d_n7, assign32470_e47400_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1068 != 0.0)) && (var_guard1069 == 0.0)) {
        let assign32470_e47397: f64 = (-p.p183);
        let assign32470_e47398: f64 = (var_arg__blk1059).powf(assign32470_e47397);
        (assign32470_e47398, if 0.0 == 0.0 && ((assign32470_e47397) as f64).is_finite() && ((assign32470_e47397) as f64).fract() == 0.0 { if assign32470_e47397 == 0.0 { 0.0 } else { (assign32470_e47397 * ((var_arg__blk1059).powf(assign32470_e47397 - 1.0) * var_arg__blk1059_dn6)) } } else { (assign32470_e47398 * (assign32470_e47397 * (var_arg__blk1059_dn6 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32470_e47397) as f64).is_finite() && ((assign32470_e47397) as f64).fract() == 0.0 { if assign32470_e47397 == 0.0 { 0.0 } else { (assign32470_e47397 * ((var_arg__blk1059).powf(assign32470_e47397 - 1.0) * var_arg__blk1059_dn7)) } } else { (assign32470_e47398 * (assign32470_e47397 * (var_arg__blk1059_dn7 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32470_e47397) as f64).is_finite() && ((assign32470_e47397) as f64).fract() == 0.0 { if assign32470_e47397 == 0.0 { 0.0 } else { (assign32470_e47397 * ((var_arg__blk1059).powf(assign32470_e47397 - 1.0) * var_arg__blk1059_dn12)) } } else { (assign32470_e47398 * (assign32470_e47397 * (var_arg__blk1059_dn12 / var_arg__blk1059))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32470_e47400;
        var_sarg_dn6 = assign32470_e47400_d_n6;
        var_sarg_dn7 = assign32470_e47400_d_n7;
        var_sarg_dn12 = assign32470_e47400_d_n12;

        let (assign32480_e47424, assign32480_e47424_d_n0, assign32480_e47424_d_n2, assign32480_e47424_d_n6, assign32480_e47424_d_n7, assign32480_e47424_d_n10, assign32480_e47424_d_n11, assign32480_e47424_d_n12, assign32480_e47424_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1068 != 0.0)) {
        let assign32480_e47411: f64 = (p.p186 * var_czbssw);
        let assign32480_e47415: f64 = (var_arg__blk1059 * var_sarg);
        let assign32480_e47416: f64 = (1.0 - assign32480_e47415);
        let assign32480_e47417: f64 = (assign32480_e47411 * assign32480_e47416);
        let assign32480_e47420: f64 = (1.0 - p.p183);
        let assign32480_e47421: f64 = (assign32480_e47417 / assign32480_e47420);
        let assign32480_e47422: f64 = (var_qbs + assign32480_e47421);
        (assign32480_e47422, var_qbs_dn0, var_qbs_dn2, (var_qbs_dn6 + ((assign32480_e47411 * (-((var_arg__blk1059_dn6 * var_sarg) + (var_arg__blk1059 * var_sarg_dn6)))) / assign32480_e47420)), (var_qbs_dn7 + ((assign32480_e47411 * (-((var_arg__blk1059_dn7 * var_sarg) + (var_arg__blk1059 * var_sarg_dn7)))) / assign32480_e47420)), var_qbs_dn10, var_qbs_dn11, (var_qbs_dn12 + ((assign32480_e47411 * (-((var_arg__blk1059_dn12 * var_sarg) + (var_arg__blk1059 * var_sarg_dn12)))) / assign32480_e47420)), var_qbs_dn17,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32480_e47424;
        var_qbs_dn0 = assign32480_e47424_d_n0;
        var_qbs_dn2 = assign32480_e47424_d_n2;
        var_qbs_dn6 = assign32480_e47424_d_n6;
        var_qbs_dn7 = assign32480_e47424_d_n7;
        var_qbs_dn10 = assign32480_e47424_d_n10;
        var_qbs_dn11 = assign32480_e47424_d_n11;
        var_qbs_dn12 = assign32480_e47424_d_n12;
        var_qbs_dn17 = assign32480_e47424_d_n17;

        let assign32490_e47427: f64 = if var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        var_guard1070 = assign32490_e47427;

        let (assign32500_e47441, assign32500_e47441_d_n6, assign32500_e47441_d_n7, assign32500_e47441_d_n12,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1070 != 0.0)) {
        let assign32500_e47438: f64 = (var_vbsj / p.p187);
        let assign32500_e47439: f64 = (1.0 - assign32500_e47438);
        (assign32500_e47439, 0.0, (-(var_vbsj_dn7 / p.p187)), (-(var_vbsj_dn12 / p.p187)),)
    } else {
        (var_arg__blk1059, var_arg__blk1059_dn6, var_arg__blk1059_dn7, var_arg__blk1059_dn12,)
    }
};
        var_arg__blk1059 = assign32500_e47441;
        var_arg__blk1059_dn6 = assign32500_e47441_d_n6;
        var_arg__blk1059_dn7 = assign32500_e47441_d_n7;
        var_arg__blk1059_dn12 = assign32500_e47441_d_n12;

        let assign32510_e47444: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        var_guard1071 = assign32510_e47444;

        let (assign32520_e47459, assign32520_e47459_d_n6, assign32520_e47459_d_n7, assign32520_e47459_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1070 != 0.0)) && (var_guard1071 != 0.0)) {
        let assign32520_e47456: f64 = (var_arg__blk1059).sqrt();
        let assign32520_e47457: f64 = (1.0 / assign32520_e47456);
        (assign32520_e47457, (-((var_arg__blk1059_dn6 / (2.0 * assign32520_e47456)) / (assign32520_e47456 * assign32520_e47456))), (-((var_arg__blk1059_dn7 / (2.0 * assign32520_e47456)) / (assign32520_e47456 * assign32520_e47456))), (-((var_arg__blk1059_dn12 / (2.0 * assign32520_e47456)) / (assign32520_e47456 * assign32520_e47456))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32520_e47459;
        var_sarg_dn6 = assign32520_e47459_d_n6;
        var_sarg_dn7 = assign32520_e47459_d_n7;
        var_sarg_dn12 = assign32520_e47459_d_n12;

        *var_arg__blk1059_slot = var_arg__blk1059;
        *var_arg__blk1059_dn12_slot = var_arg__blk1059_dn12;
        *var_arg__blk1059_dn6_slot = var_arg__blk1059_dn6;
        *var_arg__blk1059_dn7_slot = var_arg__blk1059_dn7;
        *var_czbd_slot = var_czbd;
        *var_czbs_slot = var_czbs;
        *var_czbssw_slot = var_czbssw;
        *var_czbsswg_slot = var_czbsswg;
        *var_guard1062_slot = var_guard1062;
        *var_guard1063_slot = var_guard1063;
        *var_guard1064_slot = var_guard1064;
        *var_guard1065_slot = var_guard1065;
        *var_guard1066_slot = var_guard1066;
        *var_guard1067_slot = var_guard1067;
        *var_guard1068_slot = var_guard1068;
        *var_guard1069_slot = var_guard1069;
        *var_guard1070_slot = var_guard1070;
        *var_guard1071_slot = var_guard1071;
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
        *var_t1__blk1034_slot = var_t1__blk1034;
        *var_t1__blk1034_dn10_slot = var_t1__blk1034_dn10;
        *var_t1__blk1034_dn12_slot = var_t1__blk1034_dn12;
        *var_t1__blk1034_dn6_slot = var_t1__blk1034_dn6;
        *var_t1__blk1034_dn7_slot = var_t1__blk1034_dn7;
        *var_xp_max_slot = var_xp_max;
    }

    pub(super) fn stamp_transient_block_115(
        p: &Parameters,
        var_czbd: f64,
        var_czbs: f64,
        var_czbssw: f64,
        var_guard1032: f64,
        var_guard1064: f64,
        var_guard1065: f64,
        var_guard1070: f64,
        var_guard1071: f64,
        var_vbdj: f64,
        var_vbdj_dn12: f64,
        var_vbdj_dn6: f64,
        var_vbsj: f64,
        var_vbsj_dn12: f64,
        var_vbsj_dn7: f64,
        var_w_diodcv: f64,
        var_arg__blk1059_slot: &mut f64,
        var_arg__blk1059_dn12_slot: &mut f64,
        var_arg__blk1059_dn6_slot: &mut f64,
        var_arg__blk1059_dn7_slot: &mut f64,
        var_czbdsw_slot: &mut f64,
        var_czbdswg_slot: &mut f64,
        var_czbsswg_slot: &mut f64,
        var_guard1072_slot: &mut f64,
        var_guard1073_slot: &mut f64,
        var_guard1074_slot: &mut f64,
        var_guard1075_slot: &mut f64,
        var_guard1076_slot: &mut f64,
        var_guard1077_slot: &mut f64,
        var_guard1078_slot: &mut f64,
        var_guard1079_slot: &mut f64,
        var_guard1080_slot: &mut f64,
        var_guard1081_slot: &mut f64,
        var_guard1082_slot: &mut f64,
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
        var_t1__blk1034_slot: &mut f64,
        var_t1__blk1034_dn10_slot: &mut f64,
        var_t1__blk1034_dn12_slot: &mut f64,
        var_t1__blk1034_dn6_slot: &mut f64,
        var_t1__blk1034_dn7_slot: &mut f64,
        var_t2__blk1035_slot: &mut f64,
        var_t2__blk1035_dn0_slot: &mut f64,
        var_t2__blk1035_dn10_slot: &mut f64,
        var_t2__blk1035_dn11_slot: &mut f64,
        var_t2__blk1035_dn12_slot: &mut f64,
        var_t2__blk1035_dn17_slot: &mut f64,
        var_t2__blk1035_dn2_slot: &mut f64,
        var_t2__blk1035_dn6_slot: &mut f64,
        var_t2__blk1035_dn7_slot: &mut f64,
    ) {
        let mut var_arg__blk1059: f64 = *var_arg__blk1059_slot;
        let mut var_arg__blk1059_dn12: f64 = *var_arg__blk1059_dn12_slot;
        let mut var_arg__blk1059_dn6: f64 = *var_arg__blk1059_dn6_slot;
        let mut var_arg__blk1059_dn7: f64 = *var_arg__blk1059_dn7_slot;
        let mut var_czbdsw: f64 = *var_czbdsw_slot;
        let mut var_czbdswg: f64 = *var_czbdswg_slot;
        let mut var_czbsswg: f64 = *var_czbsswg_slot;
        let mut var_guard1072: f64 = *var_guard1072_slot;
        let mut var_guard1073: f64 = *var_guard1073_slot;
        let mut var_guard1074: f64 = *var_guard1074_slot;
        let mut var_guard1075: f64 = *var_guard1075_slot;
        let mut var_guard1076: f64 = *var_guard1076_slot;
        let mut var_guard1077: f64 = *var_guard1077_slot;
        let mut var_guard1078: f64 = *var_guard1078_slot;
        let mut var_guard1079: f64 = *var_guard1079_slot;
        let mut var_guard1080: f64 = *var_guard1080_slot;
        let mut var_guard1081: f64 = *var_guard1081_slot;
        let mut var_guard1082: f64 = *var_guard1082_slot;
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
        let mut var_t1__blk1034: f64 = *var_t1__blk1034_slot;
        let mut var_t1__blk1034_dn10: f64 = *var_t1__blk1034_dn10_slot;
        let mut var_t1__blk1034_dn12: f64 = *var_t1__blk1034_dn12_slot;
        let mut var_t1__blk1034_dn6: f64 = *var_t1__blk1034_dn6_slot;
        let mut var_t1__blk1034_dn7: f64 = *var_t1__blk1034_dn7_slot;
        let mut var_t2__blk1035: f64 = *var_t2__blk1035_slot;
        let mut var_t2__blk1035_dn0: f64 = *var_t2__blk1035_dn0_slot;
        let mut var_t2__blk1035_dn10: f64 = *var_t2__blk1035_dn10_slot;
        let mut var_t2__blk1035_dn11: f64 = *var_t2__blk1035_dn11_slot;
        let mut var_t2__blk1035_dn12: f64 = *var_t2__blk1035_dn12_slot;
        let mut var_t2__blk1035_dn17: f64 = *var_t2__blk1035_dn17_slot;
        let mut var_t2__blk1035_dn2: f64 = *var_t2__blk1035_dn2_slot;
        let mut var_t2__blk1035_dn6: f64 = *var_t2__blk1035_dn6_slot;
        let mut var_t2__blk1035_dn7: f64 = *var_t2__blk1035_dn7_slot;

        let (assign32530_e47475, assign32530_e47475_d_n6, assign32530_e47475_d_n7, assign32530_e47475_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1070 != 0.0)) && (var_guard1071 == 0.0)) {
        let assign32530_e47472: f64 = (-p.p184);
        let assign32530_e47473: f64 = (var_arg__blk1059).powf(assign32530_e47472);
        (assign32530_e47473, if 0.0 == 0.0 && ((assign32530_e47472) as f64).is_finite() && ((assign32530_e47472) as f64).fract() == 0.0 { if assign32530_e47472 == 0.0 { 0.0 } else { (assign32530_e47472 * ((var_arg__blk1059).powf(assign32530_e47472 - 1.0) * var_arg__blk1059_dn6)) } } else { (assign32530_e47473 * (assign32530_e47472 * (var_arg__blk1059_dn6 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32530_e47472) as f64).is_finite() && ((assign32530_e47472) as f64).fract() == 0.0 { if assign32530_e47472 == 0.0 { 0.0 } else { (assign32530_e47472 * ((var_arg__blk1059).powf(assign32530_e47472 - 1.0) * var_arg__blk1059_dn7)) } } else { (assign32530_e47473 * (assign32530_e47472 * (var_arg__blk1059_dn7 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32530_e47472) as f64).is_finite() && ((assign32530_e47472) as f64).fract() == 0.0 { if assign32530_e47472 == 0.0 { 0.0 } else { (assign32530_e47472 * ((var_arg__blk1059).powf(assign32530_e47472 - 1.0) * var_arg__blk1059_dn12)) } } else { (assign32530_e47473 * (assign32530_e47472 * (var_arg__blk1059_dn12 / var_arg__blk1059))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32530_e47475;
        var_sarg_dn6 = assign32530_e47475_d_n6;
        var_sarg_dn7 = assign32530_e47475_d_n7;
        var_sarg_dn12 = assign32530_e47475_d_n12;

        let (assign32540_e47499, assign32540_e47499_d_n0, assign32540_e47499_d_n2, assign32540_e47499_d_n6, assign32540_e47499_d_n7, assign32540_e47499_d_n10, assign32540_e47499_d_n11, assign32540_e47499_d_n12, assign32540_e47499_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 != 0.0)) && (var_guard1070 != 0.0)) {
        let assign32540_e47486: f64 = (p.p187 * var_czbsswg);
        let assign32540_e47490: f64 = (var_arg__blk1059 * var_sarg);
        let assign32540_e47491: f64 = (1.0 - assign32540_e47490);
        let assign32540_e47492: f64 = (assign32540_e47486 * assign32540_e47491);
        let assign32540_e47495: f64 = (1.0 - p.p184);
        let assign32540_e47496: f64 = (assign32540_e47492 / assign32540_e47495);
        let assign32540_e47497: f64 = (var_qbs + assign32540_e47496);
        (assign32540_e47497, var_qbs_dn0, var_qbs_dn2, (var_qbs_dn6 + ((assign32540_e47486 * (-((var_arg__blk1059_dn6 * var_sarg) + (var_arg__blk1059 * var_sarg_dn6)))) / assign32540_e47495)), (var_qbs_dn7 + ((assign32540_e47486 * (-((var_arg__blk1059_dn7 * var_sarg) + (var_arg__blk1059 * var_sarg_dn7)))) / assign32540_e47495)), var_qbs_dn10, var_qbs_dn11, (var_qbs_dn12 + ((assign32540_e47486 * (-((var_arg__blk1059_dn12 * var_sarg) + (var_arg__blk1059 * var_sarg_dn12)))) / assign32540_e47495)), var_qbs_dn17,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32540_e47499;
        var_qbs_dn0 = assign32540_e47499_d_n0;
        var_qbs_dn2 = assign32540_e47499_d_n2;
        var_qbs_dn6 = assign32540_e47499_d_n6;
        var_qbs_dn7 = assign32540_e47499_d_n7;
        var_qbs_dn10 = assign32540_e47499_d_n10;
        var_qbs_dn11 = assign32540_e47499_d_n11;
        var_qbs_dn12 = assign32540_e47499_d_n12;
        var_qbs_dn17 = assign32540_e47499_d_n17;

        let (assign32550_e47512, assign32550_e47512_d_n6, assign32550_e47512_d_n7, assign32550_e47512_d_n10, assign32550_e47512_d_n12,) = {
    if (((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 == 0.0)) {
        let assign32550_e47508: f64 = (var_czbs + var_czbssw);
        let assign32550_e47510: f64 = (assign32550_e47508 + var_czbsswg);
        (assign32550_e47510, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk1034, var_t1__blk1034_dn6, var_t1__blk1034_dn7, var_t1__blk1034_dn10, var_t1__blk1034_dn12,)
    }
};
        var_t1__blk1034 = assign32550_e47512;
        var_t1__blk1034_dn6 = assign32550_e47512_d_n6;
        var_t1__blk1034_dn7 = assign32550_e47512_d_n7;
        var_t1__blk1034_dn10 = assign32550_e47512_d_n10;
        var_t1__blk1034_dn12 = assign32550_e47512_d_n12;

        let (assign32560_e47537, assign32560_e47537_d_n0, assign32560_e47537_d_n2, assign32560_e47537_d_n6, assign32560_e47537_d_n7, assign32560_e47537_d_n10, assign32560_e47537_d_n11, assign32560_e47537_d_n12, assign32560_e47537_d_n17,) = {
    if (((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 == 0.0)) {
        let assign32560_e47521: f64 = (var_czbs * p.p182);
        let assign32560_e47523: f64 = (assign32560_e47521 / p.p185);
        let assign32560_e47526: f64 = (var_czbssw * p.p183);
        let assign32560_e47528: f64 = (assign32560_e47526 / p.p186);
        let assign32560_e47529: f64 = (assign32560_e47523 + assign32560_e47528);
        let assign32560_e47532: f64 = (var_czbsswg * p.p184);
        let assign32560_e47534: f64 = (assign32560_e47532 / p.p187);
        let assign32560_e47535: f64 = (assign32560_e47529 + assign32560_e47534);
        (assign32560_e47535, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2__blk1035, var_t2__blk1035_dn0, var_t2__blk1035_dn2, var_t2__blk1035_dn6, var_t2__blk1035_dn7, var_t2__blk1035_dn10, var_t2__blk1035_dn11, var_t2__blk1035_dn12, var_t2__blk1035_dn17,)
    }
};
        var_t2__blk1035 = assign32560_e47537;
        var_t2__blk1035_dn0 = assign32560_e47537_d_n0;
        var_t2__blk1035_dn2 = assign32560_e47537_d_n2;
        var_t2__blk1035_dn6 = assign32560_e47537_d_n6;
        var_t2__blk1035_dn7 = assign32560_e47537_d_n7;
        var_t2__blk1035_dn10 = assign32560_e47537_d_n10;
        var_t2__blk1035_dn11 = assign32560_e47537_d_n11;
        var_t2__blk1035_dn12 = assign32560_e47537_d_n12;
        var_t2__blk1035_dn17 = assign32560_e47537_d_n17;

        let (assign32570_e47554, assign32570_e47554_d_n0, assign32570_e47554_d_n2, assign32570_e47554_d_n6, assign32570_e47554_d_n7, assign32570_e47554_d_n10, assign32570_e47554_d_n11, assign32570_e47554_d_n12, assign32570_e47554_d_n17,) = {
    if (((var_guard1032 != 0.0) && (var_guard1064 != 0.0)) && (var_guard1065 == 0.0)) {
        let assign32570_e47548: f64 = (var_vbsj * 0.5);
        let assign32570_e47550: f64 = (assign32570_e47548 * var_t2__blk1035);
        let assign32570_e47551: f64 = (var_t1__blk1034 + assign32570_e47550);
        let assign32570_e47552: f64 = (var_vbsj * assign32570_e47551);
        (assign32570_e47552, (var_vbsj * (assign32570_e47548 * var_t2__blk1035_dn0)), (var_vbsj * (assign32570_e47548 * var_t2__blk1035_dn2)), (var_vbsj * (var_t1__blk1034_dn6 + (assign32570_e47548 * var_t2__blk1035_dn6))), ((var_vbsj_dn7 * assign32570_e47551) + (var_vbsj * (var_t1__blk1034_dn7 + (((var_vbsj_dn7 * 0.5) * var_t2__blk1035) + (assign32570_e47548 * var_t2__blk1035_dn7))))), (var_vbsj * (var_t1__blk1034_dn10 + (assign32570_e47548 * var_t2__blk1035_dn10))), (var_vbsj * (assign32570_e47548 * var_t2__blk1035_dn11)), ((var_vbsj_dn12 * assign32570_e47551) + (var_vbsj * (var_t1__blk1034_dn12 + (((var_vbsj_dn12 * 0.5) * var_t2__blk1035) + (assign32570_e47548 * var_t2__blk1035_dn12))))), (var_vbsj * (assign32570_e47548 * var_t2__blk1035_dn17)),)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32570_e47554;
        var_qbs_dn0 = assign32570_e47554_d_n0;
        var_qbs_dn2 = assign32570_e47554_d_n2;
        var_qbs_dn6 = assign32570_e47554_d_n6;
        var_qbs_dn7 = assign32570_e47554_d_n7;
        var_qbs_dn10 = assign32570_e47554_d_n10;
        var_qbs_dn11 = assign32570_e47554_d_n11;
        var_qbs_dn12 = assign32570_e47554_d_n12;
        var_qbs_dn17 = assign32570_e47554_d_n17;

        let (assign32580_e47563,) = {
    if ((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) {
        let assign32580_e47561: f64 = (p.p181 * p.p5);
        (assign32580_e47561,)
    } else {
        (var_czbsswg,)
    }
};
        var_czbsswg = assign32580_e47563;

        let assign32590_e47566: f64 = if var_vbsj < 0.0 { 1.0 } else { 0.0 };
        var_guard1072 = assign32590_e47566;

        let assign32600_e47569: f64 = if var_czbs > 0.0 { 1.0 } else { 0.0 };
        var_guard1073 = assign32600_e47569;

        let (assign32610_e47584, assign32610_e47584_d_n6, assign32610_e47584_d_n7, assign32610_e47584_d_n12,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) && (var_guard1072 != 0.0)) && (var_guard1073 != 0.0)) {
        let assign32610_e47581: f64 = (var_vbsj / p.p185);
        let assign32610_e47582: f64 = (1.0 - assign32610_e47581);
        (assign32610_e47582, 0.0, (-(var_vbsj_dn7 / p.p185)), (-(var_vbsj_dn12 / p.p185)),)
    } else {
        (var_arg__blk1059, var_arg__blk1059_dn6, var_arg__blk1059_dn7, var_arg__blk1059_dn12,)
    }
};
        var_arg__blk1059 = assign32610_e47584;
        var_arg__blk1059_dn6 = assign32610_e47584_d_n6;
        var_arg__blk1059_dn7 = assign32610_e47584_d_n7;
        var_arg__blk1059_dn12 = assign32610_e47584_d_n12;

        let assign32620_e47587: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        var_guard1074 = assign32620_e47587;

        let (assign32630_e47603, assign32630_e47603_d_n6, assign32630_e47603_d_n7, assign32630_e47603_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) && (var_guard1072 != 0.0)) && (var_guard1073 != 0.0)) && (var_guard1074 != 0.0)) {
        let assign32630_e47600: f64 = (var_arg__blk1059).sqrt();
        let assign32630_e47601: f64 = (1.0 / assign32630_e47600);
        (assign32630_e47601, (-((var_arg__blk1059_dn6 / (2.0 * assign32630_e47600)) / (assign32630_e47600 * assign32630_e47600))), (-((var_arg__blk1059_dn7 / (2.0 * assign32630_e47600)) / (assign32630_e47600 * assign32630_e47600))), (-((var_arg__blk1059_dn12 / (2.0 * assign32630_e47600)) / (assign32630_e47600 * assign32630_e47600))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32630_e47603;
        var_sarg_dn6 = assign32630_e47603_d_n6;
        var_sarg_dn7 = assign32630_e47603_d_n7;
        var_sarg_dn12 = assign32630_e47603_d_n12;

        let (assign32640_e47620, assign32640_e47620_d_n6, assign32640_e47620_d_n7, assign32640_e47620_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) && (var_guard1072 != 0.0)) && (var_guard1073 != 0.0)) && (var_guard1074 == 0.0)) {
        let assign32640_e47617: f64 = (-p.p182);
        let assign32640_e47618: f64 = (var_arg__blk1059).powf(assign32640_e47617);
        (assign32640_e47618, if 0.0 == 0.0 && ((assign32640_e47617) as f64).is_finite() && ((assign32640_e47617) as f64).fract() == 0.0 { if assign32640_e47617 == 0.0 { 0.0 } else { (assign32640_e47617 * ((var_arg__blk1059).powf(assign32640_e47617 - 1.0) * var_arg__blk1059_dn6)) } } else { (assign32640_e47618 * (assign32640_e47617 * (var_arg__blk1059_dn6 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32640_e47617) as f64).is_finite() && ((assign32640_e47617) as f64).fract() == 0.0 { if assign32640_e47617 == 0.0 { 0.0 } else { (assign32640_e47617 * ((var_arg__blk1059).powf(assign32640_e47617 - 1.0) * var_arg__blk1059_dn7)) } } else { (assign32640_e47618 * (assign32640_e47617 * (var_arg__blk1059_dn7 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32640_e47617) as f64).is_finite() && ((assign32640_e47617) as f64).fract() == 0.0 { if assign32640_e47617 == 0.0 { 0.0 } else { (assign32640_e47617 * ((var_arg__blk1059).powf(assign32640_e47617 - 1.0) * var_arg__blk1059_dn12)) } } else { (assign32640_e47618 * (assign32640_e47617 * (var_arg__blk1059_dn12 / var_arg__blk1059))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32640_e47620;
        var_sarg_dn6 = assign32640_e47620_d_n6;
        var_sarg_dn7 = assign32640_e47620_d_n7;
        var_sarg_dn12 = assign32640_e47620_d_n12;

        let (assign32650_e47643, assign32650_e47643_d_n0, assign32650_e47643_d_n2, assign32650_e47643_d_n6, assign32650_e47643_d_n7, assign32650_e47643_d_n10, assign32650_e47643_d_n11, assign32650_e47643_d_n12, assign32650_e47643_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) && (var_guard1072 != 0.0)) && (var_guard1073 != 0.0)) {
        let assign32650_e47631: f64 = (p.p185 * var_czbs);
        let assign32650_e47635: f64 = (var_arg__blk1059 * var_sarg);
        let assign32650_e47636: f64 = (1.0 - assign32650_e47635);
        let assign32650_e47637: f64 = (assign32650_e47631 * assign32650_e47636);
        let assign32650_e47640: f64 = (1.0 - p.p182);
        let assign32650_e47641: f64 = (assign32650_e47637 / assign32650_e47640);
        (assign32650_e47641, 0.0, 0.0, ((assign32650_e47631 * (-((var_arg__blk1059_dn6 * var_sarg) + (var_arg__blk1059 * var_sarg_dn6)))) / assign32650_e47640), ((assign32650_e47631 * (-((var_arg__blk1059_dn7 * var_sarg) + (var_arg__blk1059 * var_sarg_dn7)))) / assign32650_e47640), 0.0, 0.0, ((assign32650_e47631 * (-((var_arg__blk1059_dn12 * var_sarg) + (var_arg__blk1059 * var_sarg_dn12)))) / assign32650_e47640), 0.0,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32650_e47643;
        var_qbs_dn0 = assign32650_e47643_d_n0;
        var_qbs_dn2 = assign32650_e47643_d_n2;
        var_qbs_dn6 = assign32650_e47643_d_n6;
        var_qbs_dn7 = assign32650_e47643_d_n7;
        var_qbs_dn10 = assign32650_e47643_d_n10;
        var_qbs_dn11 = assign32650_e47643_d_n11;
        var_qbs_dn12 = assign32650_e47643_d_n12;
        var_qbs_dn17 = assign32650_e47643_d_n17;

        let (assign32660_e47655, assign32660_e47655_d_n0, assign32660_e47655_d_n2, assign32660_e47655_d_n6, assign32660_e47655_d_n7, assign32660_e47655_d_n10, assign32660_e47655_d_n11, assign32660_e47655_d_n12, assign32660_e47655_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) && (var_guard1072 != 0.0)) && (var_guard1073 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32660_e47655;
        var_qbs_dn0 = assign32660_e47655_d_n0;
        var_qbs_dn2 = assign32660_e47655_d_n2;
        var_qbs_dn6 = assign32660_e47655_d_n6;
        var_qbs_dn7 = assign32660_e47655_d_n7;
        var_qbs_dn10 = assign32660_e47655_d_n10;
        var_qbs_dn11 = assign32660_e47655_d_n11;
        var_qbs_dn12 = assign32660_e47655_d_n12;
        var_qbs_dn17 = assign32660_e47655_d_n17;

        let assign32670_e47658: f64 = if var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        var_guard1075 = assign32670_e47658;

        let (assign32680_e47673, assign32680_e47673_d_n6, assign32680_e47673_d_n7, assign32680_e47673_d_n12,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) && (var_guard1072 != 0.0)) && (var_guard1075 != 0.0)) {
        let assign32680_e47670: f64 = (var_vbsj / p.p187);
        let assign32680_e47671: f64 = (1.0 - assign32680_e47670);
        (assign32680_e47671, 0.0, (-(var_vbsj_dn7 / p.p187)), (-(var_vbsj_dn12 / p.p187)),)
    } else {
        (var_arg__blk1059, var_arg__blk1059_dn6, var_arg__blk1059_dn7, var_arg__blk1059_dn12,)
    }
};
        var_arg__blk1059 = assign32680_e47673;
        var_arg__blk1059_dn6 = assign32680_e47673_d_n6;
        var_arg__blk1059_dn7 = assign32680_e47673_d_n7;
        var_arg__blk1059_dn12 = assign32680_e47673_d_n12;

        let assign32690_e47676: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        var_guard1076 = assign32690_e47676;

        let (assign32700_e47692, assign32700_e47692_d_n6, assign32700_e47692_d_n7, assign32700_e47692_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) && (var_guard1072 != 0.0)) && (var_guard1075 != 0.0)) && (var_guard1076 != 0.0)) {
        let assign32700_e47689: f64 = (var_arg__blk1059).sqrt();
        let assign32700_e47690: f64 = (1.0 / assign32700_e47689);
        (assign32700_e47690, (-((var_arg__blk1059_dn6 / (2.0 * assign32700_e47689)) / (assign32700_e47689 * assign32700_e47689))), (-((var_arg__blk1059_dn7 / (2.0 * assign32700_e47689)) / (assign32700_e47689 * assign32700_e47689))), (-((var_arg__blk1059_dn12 / (2.0 * assign32700_e47689)) / (assign32700_e47689 * assign32700_e47689))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32700_e47692;
        var_sarg_dn6 = assign32700_e47692_d_n6;
        var_sarg_dn7 = assign32700_e47692_d_n7;
        var_sarg_dn12 = assign32700_e47692_d_n12;

        let (assign32710_e47709, assign32710_e47709_d_n6, assign32710_e47709_d_n7, assign32710_e47709_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) && (var_guard1072 != 0.0)) && (var_guard1075 != 0.0)) && (var_guard1076 == 0.0)) {
        let assign32710_e47706: f64 = (-p.p184);
        let assign32710_e47707: f64 = (var_arg__blk1059).powf(assign32710_e47706);
        (assign32710_e47707, if 0.0 == 0.0 && ((assign32710_e47706) as f64).is_finite() && ((assign32710_e47706) as f64).fract() == 0.0 { if assign32710_e47706 == 0.0 { 0.0 } else { (assign32710_e47706 * ((var_arg__blk1059).powf(assign32710_e47706 - 1.0) * var_arg__blk1059_dn6)) } } else { (assign32710_e47707 * (assign32710_e47706 * (var_arg__blk1059_dn6 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32710_e47706) as f64).is_finite() && ((assign32710_e47706) as f64).fract() == 0.0 { if assign32710_e47706 == 0.0 { 0.0 } else { (assign32710_e47706 * ((var_arg__blk1059).powf(assign32710_e47706 - 1.0) * var_arg__blk1059_dn7)) } } else { (assign32710_e47707 * (assign32710_e47706 * (var_arg__blk1059_dn7 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32710_e47706) as f64).is_finite() && ((assign32710_e47706) as f64).fract() == 0.0 { if assign32710_e47706 == 0.0 { 0.0 } else { (assign32710_e47706 * ((var_arg__blk1059).powf(assign32710_e47706 - 1.0) * var_arg__blk1059_dn12)) } } else { (assign32710_e47707 * (assign32710_e47706 * (var_arg__blk1059_dn12 / var_arg__blk1059))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32710_e47709;
        var_sarg_dn6 = assign32710_e47709_d_n6;
        var_sarg_dn7 = assign32710_e47709_d_n7;
        var_sarg_dn12 = assign32710_e47709_d_n12;

        let (assign32720_e47734, assign32720_e47734_d_n0, assign32720_e47734_d_n2, assign32720_e47734_d_n6, assign32720_e47734_d_n7, assign32720_e47734_d_n10, assign32720_e47734_d_n11, assign32720_e47734_d_n12, assign32720_e47734_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) && (var_guard1072 != 0.0)) && (var_guard1075 != 0.0)) {
        let assign32720_e47721: f64 = (p.p187 * var_czbsswg);
        let assign32720_e47725: f64 = (var_arg__blk1059 * var_sarg);
        let assign32720_e47726: f64 = (1.0 - assign32720_e47725);
        let assign32720_e47727: f64 = (assign32720_e47721 * assign32720_e47726);
        let assign32720_e47730: f64 = (1.0 - p.p184);
        let assign32720_e47731: f64 = (assign32720_e47727 / assign32720_e47730);
        let assign32720_e47732: f64 = (var_qbs + assign32720_e47731);
        (assign32720_e47732, var_qbs_dn0, var_qbs_dn2, (var_qbs_dn6 + ((assign32720_e47721 * (-((var_arg__blk1059_dn6 * var_sarg) + (var_arg__blk1059 * var_sarg_dn6)))) / assign32720_e47730)), (var_qbs_dn7 + ((assign32720_e47721 * (-((var_arg__blk1059_dn7 * var_sarg) + (var_arg__blk1059 * var_sarg_dn7)))) / assign32720_e47730)), var_qbs_dn10, var_qbs_dn11, (var_qbs_dn12 + ((assign32720_e47721 * (-((var_arg__blk1059_dn12 * var_sarg) + (var_arg__blk1059 * var_sarg_dn12)))) / assign32720_e47730)), var_qbs_dn17,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32720_e47734;
        var_qbs_dn0 = assign32720_e47734_d_n0;
        var_qbs_dn2 = assign32720_e47734_d_n2;
        var_qbs_dn6 = assign32720_e47734_d_n6;
        var_qbs_dn7 = assign32720_e47734_d_n7;
        var_qbs_dn10 = assign32720_e47734_d_n10;
        var_qbs_dn11 = assign32720_e47734_d_n11;
        var_qbs_dn12 = assign32720_e47734_d_n12;
        var_qbs_dn17 = assign32720_e47734_d_n17;

        let (assign32730_e47746, assign32730_e47746_d_n6, assign32730_e47746_d_n7, assign32730_e47746_d_n10, assign32730_e47746_d_n12,) = {
    if (((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) && (var_guard1072 == 0.0)) {
        let assign32730_e47744: f64 = (var_czbs + var_czbsswg);
        (assign32730_e47744, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk1034, var_t1__blk1034_dn6, var_t1__blk1034_dn7, var_t1__blk1034_dn10, var_t1__blk1034_dn12,)
    }
};
        var_t1__blk1034 = assign32730_e47746;
        var_t1__blk1034_dn6 = assign32730_e47746_d_n6;
        var_t1__blk1034_dn7 = assign32730_e47746_d_n7;
        var_t1__blk1034_dn10 = assign32730_e47746_d_n10;
        var_t1__blk1034_dn12 = assign32730_e47746_d_n12;

        let (assign32740_e47766, assign32740_e47766_d_n0, assign32740_e47766_d_n2, assign32740_e47766_d_n6, assign32740_e47766_d_n7, assign32740_e47766_d_n10, assign32740_e47766_d_n11, assign32740_e47766_d_n12, assign32740_e47766_d_n17,) = {
    if (((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) && (var_guard1072 == 0.0)) {
        let assign32740_e47756: f64 = (var_czbs * p.p182);
        let assign32740_e47758: f64 = (assign32740_e47756 / p.p185);
        let assign32740_e47761: f64 = (var_czbsswg * p.p184);
        let assign32740_e47763: f64 = (assign32740_e47761 / p.p187);
        let assign32740_e47764: f64 = (assign32740_e47758 + assign32740_e47763);
        (assign32740_e47764, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2__blk1035, var_t2__blk1035_dn0, var_t2__blk1035_dn2, var_t2__blk1035_dn6, var_t2__blk1035_dn7, var_t2__blk1035_dn10, var_t2__blk1035_dn11, var_t2__blk1035_dn12, var_t2__blk1035_dn17,)
    }
};
        var_t2__blk1035 = assign32740_e47766;
        var_t2__blk1035_dn0 = assign32740_e47766_d_n0;
        var_t2__blk1035_dn2 = assign32740_e47766_d_n2;
        var_t2__blk1035_dn6 = assign32740_e47766_d_n6;
        var_t2__blk1035_dn7 = assign32740_e47766_d_n7;
        var_t2__blk1035_dn10 = assign32740_e47766_d_n10;
        var_t2__blk1035_dn11 = assign32740_e47766_d_n11;
        var_t2__blk1035_dn12 = assign32740_e47766_d_n12;
        var_t2__blk1035_dn17 = assign32740_e47766_d_n17;

        let (assign32750_e47784, assign32750_e47784_d_n0, assign32750_e47784_d_n2, assign32750_e47784_d_n6, assign32750_e47784_d_n7, assign32750_e47784_d_n10, assign32750_e47784_d_n11, assign32750_e47784_d_n12, assign32750_e47784_d_n17,) = {
    if (((var_guard1032 != 0.0) && (var_guard1064 == 0.0)) && (var_guard1072 == 0.0)) {
        let assign32750_e47778: f64 = (var_vbsj * 0.5);
        let assign32750_e47780: f64 = (assign32750_e47778 * var_t2__blk1035);
        let assign32750_e47781: f64 = (var_t1__blk1034 + assign32750_e47780);
        let assign32750_e47782: f64 = (var_vbsj * assign32750_e47781);
        (assign32750_e47782, (var_vbsj * (assign32750_e47778 * var_t2__blk1035_dn0)), (var_vbsj * (assign32750_e47778 * var_t2__blk1035_dn2)), (var_vbsj * (var_t1__blk1034_dn6 + (assign32750_e47778 * var_t2__blk1035_dn6))), ((var_vbsj_dn7 * assign32750_e47781) + (var_vbsj * (var_t1__blk1034_dn7 + (((var_vbsj_dn7 * 0.5) * var_t2__blk1035) + (assign32750_e47778 * var_t2__blk1035_dn7))))), (var_vbsj * (var_t1__blk1034_dn10 + (assign32750_e47778 * var_t2__blk1035_dn10))), (var_vbsj * (assign32750_e47778 * var_t2__blk1035_dn11)), ((var_vbsj_dn12 * assign32750_e47781) + (var_vbsj * (var_t1__blk1034_dn12 + (((var_vbsj_dn12 * 0.5) * var_t2__blk1035) + (assign32750_e47778 * var_t2__blk1035_dn12))))), (var_vbsj * (assign32750_e47778 * var_t2__blk1035_dn17)),)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign32750_e47784;
        var_qbs_dn0 = assign32750_e47784_d_n0;
        var_qbs_dn2 = assign32750_e47784_d_n2;
        var_qbs_dn6 = assign32750_e47784_d_n6;
        var_qbs_dn7 = assign32750_e47784_d_n7;
        var_qbs_dn10 = assign32750_e47784_d_n10;
        var_qbs_dn11 = assign32750_e47784_d_n11;
        var_qbs_dn12 = assign32750_e47784_d_n12;
        var_qbs_dn17 = assign32750_e47784_d_n17;

        let assign32760_e47787: f64 = if p.p4 > var_w_diodcv { 1.0 } else { 0.0 };
        var_guard1077 = assign32760_e47787;

        let (assign32770_e47797,) = {
    if ((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) {
        let assign32770_e47794: f64 = (p.p4 - var_w_diodcv);
        let assign32770_e47795: f64 = (p.p180 * assign32770_e47794);
        (assign32770_e47795,)
    } else {
        (var_czbdsw,)
    }
};
        var_czbdsw = assign32770_e47797;

        let (assign32780_e47805,) = {
    if ((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) {
        let assign32780_e47803: f64 = (p.p181 * var_w_diodcv);
        (assign32780_e47803,)
    } else {
        (var_czbdswg,)
    }
};
        var_czbdswg = assign32780_e47805;

        let assign32790_e47808: f64 = if var_vbdj < 0.0 { 1.0 } else { 0.0 };
        var_guard1078 = assign32790_e47808;

        let assign32800_e47811: f64 = if var_czbd > 0.0 { 1.0 } else { 0.0 };
        var_guard1079 = assign32800_e47811;

        let (assign32810_e47825, assign32810_e47825_d_n6, assign32810_e47825_d_n7, assign32810_e47825_d_n12,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1079 != 0.0)) {
        let assign32810_e47822: f64 = (var_vbdj / p.p185);
        let assign32810_e47823: f64 = (1.0 - assign32810_e47822);
        (assign32810_e47823, (-(var_vbdj_dn6 / p.p185)), 0.0, (-(var_vbdj_dn12 / p.p185)),)
    } else {
        (var_arg__blk1059, var_arg__blk1059_dn6, var_arg__blk1059_dn7, var_arg__blk1059_dn12,)
    }
};
        var_arg__blk1059 = assign32810_e47825;
        var_arg__blk1059_dn6 = assign32810_e47825_d_n6;
        var_arg__blk1059_dn7 = assign32810_e47825_d_n7;
        var_arg__blk1059_dn12 = assign32810_e47825_d_n12;

        let assign32820_e47828: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        var_guard1080 = assign32820_e47828;

        let (assign32830_e47843, assign32830_e47843_d_n6, assign32830_e47843_d_n7, assign32830_e47843_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1079 != 0.0)) && (var_guard1080 != 0.0)) {
        let assign32830_e47840: f64 = (var_arg__blk1059).sqrt();
        let assign32830_e47841: f64 = (1.0 / assign32830_e47840);
        (assign32830_e47841, (-((var_arg__blk1059_dn6 / (2.0 * assign32830_e47840)) / (assign32830_e47840 * assign32830_e47840))), (-((var_arg__blk1059_dn7 / (2.0 * assign32830_e47840)) / (assign32830_e47840 * assign32830_e47840))), (-((var_arg__blk1059_dn12 / (2.0 * assign32830_e47840)) / (assign32830_e47840 * assign32830_e47840))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32830_e47843;
        var_sarg_dn6 = assign32830_e47843_d_n6;
        var_sarg_dn7 = assign32830_e47843_d_n7;
        var_sarg_dn12 = assign32830_e47843_d_n12;

        let (assign32840_e47859, assign32840_e47859_d_n6, assign32840_e47859_d_n7, assign32840_e47859_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1079 != 0.0)) && (var_guard1080 == 0.0)) {
        let assign32840_e47856: f64 = (-p.p182);
        let assign32840_e47857: f64 = (var_arg__blk1059).powf(assign32840_e47856);
        (assign32840_e47857, if 0.0 == 0.0 && ((assign32840_e47856) as f64).is_finite() && ((assign32840_e47856) as f64).fract() == 0.0 { if assign32840_e47856 == 0.0 { 0.0 } else { (assign32840_e47856 * ((var_arg__blk1059).powf(assign32840_e47856 - 1.0) * var_arg__blk1059_dn6)) } } else { (assign32840_e47857 * (assign32840_e47856 * (var_arg__blk1059_dn6 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32840_e47856) as f64).is_finite() && ((assign32840_e47856) as f64).fract() == 0.0 { if assign32840_e47856 == 0.0 { 0.0 } else { (assign32840_e47856 * ((var_arg__blk1059).powf(assign32840_e47856 - 1.0) * var_arg__blk1059_dn7)) } } else { (assign32840_e47857 * (assign32840_e47856 * (var_arg__blk1059_dn7 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32840_e47856) as f64).is_finite() && ((assign32840_e47856) as f64).fract() == 0.0 { if assign32840_e47856 == 0.0 { 0.0 } else { (assign32840_e47856 * ((var_arg__blk1059).powf(assign32840_e47856 - 1.0) * var_arg__blk1059_dn12)) } } else { (assign32840_e47857 * (assign32840_e47856 * (var_arg__blk1059_dn12 / var_arg__blk1059))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32840_e47859;
        var_sarg_dn6 = assign32840_e47859_d_n6;
        var_sarg_dn7 = assign32840_e47859_d_n7;
        var_sarg_dn12 = assign32840_e47859_d_n12;

        let (assign32850_e47881, assign32850_e47881_d_n0, assign32850_e47881_d_n2, assign32850_e47881_d_n6, assign32850_e47881_d_n7, assign32850_e47881_d_n10, assign32850_e47881_d_n11, assign32850_e47881_d_n12, assign32850_e47881_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1079 != 0.0)) {
        let assign32850_e47869: f64 = (p.p185 * var_czbd);
        let assign32850_e47873: f64 = (var_arg__blk1059 * var_sarg);
        let assign32850_e47874: f64 = (1.0 - assign32850_e47873);
        let assign32850_e47875: f64 = (assign32850_e47869 * assign32850_e47874);
        let assign32850_e47878: f64 = (1.0 - p.p182);
        let assign32850_e47879: f64 = (assign32850_e47875 / assign32850_e47878);
        (assign32850_e47879, 0.0, 0.0, ((assign32850_e47869 * (-((var_arg__blk1059_dn6 * var_sarg) + (var_arg__blk1059 * var_sarg_dn6)))) / assign32850_e47878), ((assign32850_e47869 * (-((var_arg__blk1059_dn7 * var_sarg) + (var_arg__blk1059 * var_sarg_dn7)))) / assign32850_e47878), 0.0, 0.0, ((assign32850_e47869 * (-((var_arg__blk1059_dn12 * var_sarg) + (var_arg__blk1059 * var_sarg_dn12)))) / assign32850_e47878), 0.0,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign32850_e47881;
        var_qbd_dn0 = assign32850_e47881_d_n0;
        var_qbd_dn2 = assign32850_e47881_d_n2;
        var_qbd_dn6 = assign32850_e47881_d_n6;
        var_qbd_dn7 = assign32850_e47881_d_n7;
        var_qbd_dn10 = assign32850_e47881_d_n10;
        var_qbd_dn11 = assign32850_e47881_d_n11;
        var_qbd_dn12 = assign32850_e47881_d_n12;
        var_qbd_dn17 = assign32850_e47881_d_n17;

        let (assign32860_e47892, assign32860_e47892_d_n0, assign32860_e47892_d_n2, assign32860_e47892_d_n6, assign32860_e47892_d_n7, assign32860_e47892_d_n10, assign32860_e47892_d_n11, assign32860_e47892_d_n12, assign32860_e47892_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1079 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign32860_e47892;
        var_qbd_dn0 = assign32860_e47892_d_n0;
        var_qbd_dn2 = assign32860_e47892_d_n2;
        var_qbd_dn6 = assign32860_e47892_d_n6;
        var_qbd_dn7 = assign32860_e47892_d_n7;
        var_qbd_dn10 = assign32860_e47892_d_n10;
        var_qbd_dn11 = assign32860_e47892_d_n11;
        var_qbd_dn12 = assign32860_e47892_d_n12;
        var_qbd_dn17 = assign32860_e47892_d_n17;

        let assign32870_e47895: f64 = if var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        var_guard1081 = assign32870_e47895;

        let (assign32880_e47909, assign32880_e47909_d_n6, assign32880_e47909_d_n7, assign32880_e47909_d_n12,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1081 != 0.0)) {
        let assign32880_e47906: f64 = (var_vbdj / p.p186);
        let assign32880_e47907: f64 = (1.0 - assign32880_e47906);
        (assign32880_e47907, (-(var_vbdj_dn6 / p.p186)), 0.0, (-(var_vbdj_dn12 / p.p186)),)
    } else {
        (var_arg__blk1059, var_arg__blk1059_dn6, var_arg__blk1059_dn7, var_arg__blk1059_dn12,)
    }
};
        var_arg__blk1059 = assign32880_e47909;
        var_arg__blk1059_dn6 = assign32880_e47909_d_n6;
        var_arg__blk1059_dn7 = assign32880_e47909_d_n7;
        var_arg__blk1059_dn12 = assign32880_e47909_d_n12;

        let assign32890_e47912: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        var_guard1082 = assign32890_e47912;

        let (assign32900_e47927, assign32900_e47927_d_n6, assign32900_e47927_d_n7, assign32900_e47927_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1081 != 0.0)) && (var_guard1082 != 0.0)) {
        let assign32900_e47924: f64 = (var_arg__blk1059).sqrt();
        let assign32900_e47925: f64 = (1.0 / assign32900_e47924);
        (assign32900_e47925, (-((var_arg__blk1059_dn6 / (2.0 * assign32900_e47924)) / (assign32900_e47924 * assign32900_e47924))), (-((var_arg__blk1059_dn7 / (2.0 * assign32900_e47924)) / (assign32900_e47924 * assign32900_e47924))), (-((var_arg__blk1059_dn12 / (2.0 * assign32900_e47924)) / (assign32900_e47924 * assign32900_e47924))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32900_e47927;
        var_sarg_dn6 = assign32900_e47927_d_n6;
        var_sarg_dn7 = assign32900_e47927_d_n7;
        var_sarg_dn12 = assign32900_e47927_d_n12;

        let (assign32910_e47943, assign32910_e47943_d_n6, assign32910_e47943_d_n7, assign32910_e47943_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1081 != 0.0)) && (var_guard1082 == 0.0)) {
        let assign32910_e47940: f64 = (-p.p183);
        let assign32910_e47941: f64 = (var_arg__blk1059).powf(assign32910_e47940);
        (assign32910_e47941, if 0.0 == 0.0 && ((assign32910_e47940) as f64).is_finite() && ((assign32910_e47940) as f64).fract() == 0.0 { if assign32910_e47940 == 0.0 { 0.0 } else { (assign32910_e47940 * ((var_arg__blk1059).powf(assign32910_e47940 - 1.0) * var_arg__blk1059_dn6)) } } else { (assign32910_e47941 * (assign32910_e47940 * (var_arg__blk1059_dn6 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32910_e47940) as f64).is_finite() && ((assign32910_e47940) as f64).fract() == 0.0 { if assign32910_e47940 == 0.0 { 0.0 } else { (assign32910_e47940 * ((var_arg__blk1059).powf(assign32910_e47940 - 1.0) * var_arg__blk1059_dn7)) } } else { (assign32910_e47941 * (assign32910_e47940 * (var_arg__blk1059_dn7 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32910_e47940) as f64).is_finite() && ((assign32910_e47940) as f64).fract() == 0.0 { if assign32910_e47940 == 0.0 { 0.0 } else { (assign32910_e47940 * ((var_arg__blk1059).powf(assign32910_e47940 - 1.0) * var_arg__blk1059_dn12)) } } else { (assign32910_e47941 * (assign32910_e47940 * (var_arg__blk1059_dn12 / var_arg__blk1059))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32910_e47943;
        var_sarg_dn6 = assign32910_e47943_d_n6;
        var_sarg_dn7 = assign32910_e47943_d_n7;
        var_sarg_dn12 = assign32910_e47943_d_n12;

        *var_arg__blk1059_slot = var_arg__blk1059;
        *var_arg__blk1059_dn12_slot = var_arg__blk1059_dn12;
        *var_arg__blk1059_dn6_slot = var_arg__blk1059_dn6;
        *var_arg__blk1059_dn7_slot = var_arg__blk1059_dn7;
        *var_czbdsw_slot = var_czbdsw;
        *var_czbdswg_slot = var_czbdswg;
        *var_czbsswg_slot = var_czbsswg;
        *var_guard1072_slot = var_guard1072;
        *var_guard1073_slot = var_guard1073;
        *var_guard1074_slot = var_guard1074;
        *var_guard1075_slot = var_guard1075;
        *var_guard1076_slot = var_guard1076;
        *var_guard1077_slot = var_guard1077;
        *var_guard1078_slot = var_guard1078;
        *var_guard1079_slot = var_guard1079;
        *var_guard1080_slot = var_guard1080;
        *var_guard1081_slot = var_guard1081;
        *var_guard1082_slot = var_guard1082;
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
        *var_t1__blk1034_slot = var_t1__blk1034;
        *var_t1__blk1034_dn10_slot = var_t1__blk1034_dn10;
        *var_t1__blk1034_dn12_slot = var_t1__blk1034_dn12;
        *var_t1__blk1034_dn6_slot = var_t1__blk1034_dn6;
        *var_t1__blk1034_dn7_slot = var_t1__blk1034_dn7;
        *var_t2__blk1035_slot = var_t2__blk1035;
        *var_t2__blk1035_dn0_slot = var_t2__blk1035_dn0;
        *var_t2__blk1035_dn10_slot = var_t2__blk1035_dn10;
        *var_t2__blk1035_dn11_slot = var_t2__blk1035_dn11;
        *var_t2__blk1035_dn12_slot = var_t2__blk1035_dn12;
        *var_t2__blk1035_dn17_slot = var_t2__blk1035_dn17;
        *var_t2__blk1035_dn2_slot = var_t2__blk1035_dn2;
        *var_t2__blk1035_dn6_slot = var_t2__blk1035_dn6;
        *var_t2__blk1035_dn7_slot = var_t2__blk1035_dn7;
    }

    pub(super) fn stamp_transient_block_116(
        p: &Parameters,
        var_czbd: f64,
        var_czbdsw: f64,
        var_czbs: f64,
        var_guard1032: f64,
        var_guard1077: f64,
        var_guard1078: f64,
        var_guard1081: f64,
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
        var_arg__blk1059_slot: &mut f64,
        var_arg__blk1059_dn12_slot: &mut f64,
        var_arg__blk1059_dn6_slot: &mut f64,
        var_arg__blk1059_dn7_slot: &mut f64,
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
        var_guard1083_slot: &mut f64,
        var_guard1084_slot: &mut f64,
        var_guard1085_slot: &mut f64,
        var_guard1086_slot: &mut f64,
        var_guard1087_slot: &mut f64,
        var_guard1088_slot: &mut f64,
        var_guard1089_slot: &mut f64,
        var_guard1090_slot: &mut f64,
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
        var_t1__blk1034_slot: &mut f64,
        var_t1__blk1034_dn10_slot: &mut f64,
        var_t1__blk1034_dn12_slot: &mut f64,
        var_t1__blk1034_dn6_slot: &mut f64,
        var_t1__blk1034_dn7_slot: &mut f64,
        var_t2__blk1035_slot: &mut f64,
        var_t2__blk1035_dn0_slot: &mut f64,
        var_t2__blk1035_dn10_slot: &mut f64,
        var_t2__blk1035_dn11_slot: &mut f64,
        var_t2__blk1035_dn12_slot: &mut f64,
        var_t2__blk1035_dn17_slot: &mut f64,
        var_t2__blk1035_dn2_slot: &mut f64,
        var_t2__blk1035_dn6_slot: &mut f64,
        var_t2__blk1035_dn7_slot: &mut f64,
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
        let mut var_arg__blk1059: f64 = *var_arg__blk1059_slot;
        let mut var_arg__blk1059_dn12: f64 = *var_arg__blk1059_dn12_slot;
        let mut var_arg__blk1059_dn6: f64 = *var_arg__blk1059_dn6_slot;
        let mut var_arg__blk1059_dn7: f64 = *var_arg__blk1059_dn7_slot;
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
        let mut var_guard1083: f64 = *var_guard1083_slot;
        let mut var_guard1084: f64 = *var_guard1084_slot;
        let mut var_guard1085: f64 = *var_guard1085_slot;
        let mut var_guard1086: f64 = *var_guard1086_slot;
        let mut var_guard1087: f64 = *var_guard1087_slot;
        let mut var_guard1088: f64 = *var_guard1088_slot;
        let mut var_guard1089: f64 = *var_guard1089_slot;
        let mut var_guard1090: f64 = *var_guard1090_slot;
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
        let mut var_t1__blk1034: f64 = *var_t1__blk1034_slot;
        let mut var_t1__blk1034_dn10: f64 = *var_t1__blk1034_dn10_slot;
        let mut var_t1__blk1034_dn12: f64 = *var_t1__blk1034_dn12_slot;
        let mut var_t1__blk1034_dn6: f64 = *var_t1__blk1034_dn6_slot;
        let mut var_t1__blk1034_dn7: f64 = *var_t1__blk1034_dn7_slot;
        let mut var_t2__blk1035: f64 = *var_t2__blk1035_slot;
        let mut var_t2__blk1035_dn0: f64 = *var_t2__blk1035_dn0_slot;
        let mut var_t2__blk1035_dn10: f64 = *var_t2__blk1035_dn10_slot;
        let mut var_t2__blk1035_dn11: f64 = *var_t2__blk1035_dn11_slot;
        let mut var_t2__blk1035_dn12: f64 = *var_t2__blk1035_dn12_slot;
        let mut var_t2__blk1035_dn17: f64 = *var_t2__blk1035_dn17_slot;
        let mut var_t2__blk1035_dn2: f64 = *var_t2__blk1035_dn2_slot;
        let mut var_t2__blk1035_dn6: f64 = *var_t2__blk1035_dn6_slot;
        let mut var_t2__blk1035_dn7: f64 = *var_t2__blk1035_dn7_slot;
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

        let (assign32920_e47967, assign32920_e47967_d_n0, assign32920_e47967_d_n2, assign32920_e47967_d_n6, assign32920_e47967_d_n7, assign32920_e47967_d_n10, assign32920_e47967_d_n11, assign32920_e47967_d_n12, assign32920_e47967_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1081 != 0.0)) {
        let assign32920_e47954: f64 = (p.p186 * var_czbdsw);
        let assign32920_e47958: f64 = (var_arg__blk1059 * var_sarg);
        let assign32920_e47959: f64 = (1.0 - assign32920_e47958);
        let assign32920_e47960: f64 = (assign32920_e47954 * assign32920_e47959);
        let assign32920_e47963: f64 = (1.0 - p.p183);
        let assign32920_e47964: f64 = (assign32920_e47960 / assign32920_e47963);
        let assign32920_e47965: f64 = (var_qbd + assign32920_e47964);
        (assign32920_e47965, var_qbd_dn0, var_qbd_dn2, (var_qbd_dn6 + ((assign32920_e47954 * (-((var_arg__blk1059_dn6 * var_sarg) + (var_arg__blk1059 * var_sarg_dn6)))) / assign32920_e47963)), (var_qbd_dn7 + ((assign32920_e47954 * (-((var_arg__blk1059_dn7 * var_sarg) + (var_arg__blk1059 * var_sarg_dn7)))) / assign32920_e47963)), var_qbd_dn10, var_qbd_dn11, (var_qbd_dn12 + ((assign32920_e47954 * (-((var_arg__blk1059_dn12 * var_sarg) + (var_arg__blk1059 * var_sarg_dn12)))) / assign32920_e47963)), var_qbd_dn17,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign32920_e47967;
        var_qbd_dn0 = assign32920_e47967_d_n0;
        var_qbd_dn2 = assign32920_e47967_d_n2;
        var_qbd_dn6 = assign32920_e47967_d_n6;
        var_qbd_dn7 = assign32920_e47967_d_n7;
        var_qbd_dn10 = assign32920_e47967_d_n10;
        var_qbd_dn11 = assign32920_e47967_d_n11;
        var_qbd_dn12 = assign32920_e47967_d_n12;
        var_qbd_dn17 = assign32920_e47967_d_n17;

        let assign32930_e47970: f64 = if var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        var_guard1083 = assign32930_e47970;

        let (assign32940_e47984, assign32940_e47984_d_n6, assign32940_e47984_d_n7, assign32940_e47984_d_n12,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1083 != 0.0)) {
        let assign32940_e47981: f64 = (var_vbdj / p.p187);
        let assign32940_e47982: f64 = (1.0 - assign32940_e47981);
        (assign32940_e47982, (-(var_vbdj_dn6 / p.p187)), 0.0, (-(var_vbdj_dn12 / p.p187)),)
    } else {
        (var_arg__blk1059, var_arg__blk1059_dn6, var_arg__blk1059_dn7, var_arg__blk1059_dn12,)
    }
};
        var_arg__blk1059 = assign32940_e47984;
        var_arg__blk1059_dn6 = assign32940_e47984_d_n6;
        var_arg__blk1059_dn7 = assign32940_e47984_d_n7;
        var_arg__blk1059_dn12 = assign32940_e47984_d_n12;

        let assign32950_e47987: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        var_guard1084 = assign32950_e47987;

        let (assign32960_e48002, assign32960_e48002_d_n6, assign32960_e48002_d_n7, assign32960_e48002_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1083 != 0.0)) && (var_guard1084 != 0.0)) {
        let assign32960_e47999: f64 = (var_arg__blk1059).sqrt();
        let assign32960_e48000: f64 = (1.0 / assign32960_e47999);
        (assign32960_e48000, (-((var_arg__blk1059_dn6 / (2.0 * assign32960_e47999)) / (assign32960_e47999 * assign32960_e47999))), (-((var_arg__blk1059_dn7 / (2.0 * assign32960_e47999)) / (assign32960_e47999 * assign32960_e47999))), (-((var_arg__blk1059_dn12 / (2.0 * assign32960_e47999)) / (assign32960_e47999 * assign32960_e47999))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32960_e48002;
        var_sarg_dn6 = assign32960_e48002_d_n6;
        var_sarg_dn7 = assign32960_e48002_d_n7;
        var_sarg_dn12 = assign32960_e48002_d_n12;

        let (assign32970_e48018, assign32970_e48018_d_n6, assign32970_e48018_d_n7, assign32970_e48018_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1083 != 0.0)) && (var_guard1084 == 0.0)) {
        let assign32970_e48015: f64 = (-p.p184);
        let assign32970_e48016: f64 = (var_arg__blk1059).powf(assign32970_e48015);
        (assign32970_e48016, if 0.0 == 0.0 && ((assign32970_e48015) as f64).is_finite() && ((assign32970_e48015) as f64).fract() == 0.0 { if assign32970_e48015 == 0.0 { 0.0 } else { (assign32970_e48015 * ((var_arg__blk1059).powf(assign32970_e48015 - 1.0) * var_arg__blk1059_dn6)) } } else { (assign32970_e48016 * (assign32970_e48015 * (var_arg__blk1059_dn6 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32970_e48015) as f64).is_finite() && ((assign32970_e48015) as f64).fract() == 0.0 { if assign32970_e48015 == 0.0 { 0.0 } else { (assign32970_e48015 * ((var_arg__blk1059).powf(assign32970_e48015 - 1.0) * var_arg__blk1059_dn7)) } } else { (assign32970_e48016 * (assign32970_e48015 * (var_arg__blk1059_dn7 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32970_e48015) as f64).is_finite() && ((assign32970_e48015) as f64).fract() == 0.0 { if assign32970_e48015 == 0.0 { 0.0 } else { (assign32970_e48015 * ((var_arg__blk1059).powf(assign32970_e48015 - 1.0) * var_arg__blk1059_dn12)) } } else { (assign32970_e48016 * (assign32970_e48015 * (var_arg__blk1059_dn12 / var_arg__blk1059))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign32970_e48018;
        var_sarg_dn6 = assign32970_e48018_d_n6;
        var_sarg_dn7 = assign32970_e48018_d_n7;
        var_sarg_dn12 = assign32970_e48018_d_n12;

        let (assign32980_e48042, assign32980_e48042_d_n0, assign32980_e48042_d_n2, assign32980_e48042_d_n6, assign32980_e48042_d_n7, assign32980_e48042_d_n10, assign32980_e48042_d_n11, assign32980_e48042_d_n12, assign32980_e48042_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 != 0.0)) && (var_guard1083 != 0.0)) {
        let assign32980_e48029: f64 = (p.p187 * var_czbdswg);
        let assign32980_e48033: f64 = (var_arg__blk1059 * var_sarg);
        let assign32980_e48034: f64 = (1.0 - assign32980_e48033);
        let assign32980_e48035: f64 = (assign32980_e48029 * assign32980_e48034);
        let assign32980_e48038: f64 = (1.0 - p.p184);
        let assign32980_e48039: f64 = (assign32980_e48035 / assign32980_e48038);
        let assign32980_e48040: f64 = (var_qbd + assign32980_e48039);
        (assign32980_e48040, var_qbd_dn0, var_qbd_dn2, (var_qbd_dn6 + ((assign32980_e48029 * (-((var_arg__blk1059_dn6 * var_sarg) + (var_arg__blk1059 * var_sarg_dn6)))) / assign32980_e48038)), (var_qbd_dn7 + ((assign32980_e48029 * (-((var_arg__blk1059_dn7 * var_sarg) + (var_arg__blk1059 * var_sarg_dn7)))) / assign32980_e48038)), var_qbd_dn10, var_qbd_dn11, (var_qbd_dn12 + ((assign32980_e48029 * (-((var_arg__blk1059_dn12 * var_sarg) + (var_arg__blk1059 * var_sarg_dn12)))) / assign32980_e48038)), var_qbd_dn17,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign32980_e48042;
        var_qbd_dn0 = assign32980_e48042_d_n0;
        var_qbd_dn2 = assign32980_e48042_d_n2;
        var_qbd_dn6 = assign32980_e48042_d_n6;
        var_qbd_dn7 = assign32980_e48042_d_n7;
        var_qbd_dn10 = assign32980_e48042_d_n10;
        var_qbd_dn11 = assign32980_e48042_d_n11;
        var_qbd_dn12 = assign32980_e48042_d_n12;
        var_qbd_dn17 = assign32980_e48042_d_n17;

        let (assign32990_e48055, assign32990_e48055_d_n6, assign32990_e48055_d_n7, assign32990_e48055_d_n10, assign32990_e48055_d_n12,) = {
    if (((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 == 0.0)) {
        let assign32990_e48051: f64 = (var_czbd + var_czbdsw);
        let assign32990_e48053: f64 = (assign32990_e48051 + var_czbdswg);
        (assign32990_e48053, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk1034, var_t1__blk1034_dn6, var_t1__blk1034_dn7, var_t1__blk1034_dn10, var_t1__blk1034_dn12,)
    }
};
        var_t1__blk1034 = assign32990_e48055;
        var_t1__blk1034_dn6 = assign32990_e48055_d_n6;
        var_t1__blk1034_dn7 = assign32990_e48055_d_n7;
        var_t1__blk1034_dn10 = assign32990_e48055_d_n10;
        var_t1__blk1034_dn12 = assign32990_e48055_d_n12;

        let (assign33000_e48080, assign33000_e48080_d_n0, assign33000_e48080_d_n2, assign33000_e48080_d_n6, assign33000_e48080_d_n7, assign33000_e48080_d_n10, assign33000_e48080_d_n11, assign33000_e48080_d_n12, assign33000_e48080_d_n17,) = {
    if (((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 == 0.0)) {
        let assign33000_e48064: f64 = (var_czbd * p.p182);
        let assign33000_e48066: f64 = (assign33000_e48064 / p.p185);
        let assign33000_e48069: f64 = (var_czbdsw * p.p183);
        let assign33000_e48071: f64 = (assign33000_e48069 / p.p186);
        let assign33000_e48072: f64 = (assign33000_e48066 + assign33000_e48071);
        let assign33000_e48075: f64 = (var_czbdswg * p.p184);
        let assign33000_e48077: f64 = (assign33000_e48075 / p.p187);
        let assign33000_e48078: f64 = (assign33000_e48072 + assign33000_e48077);
        (assign33000_e48078, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2__blk1035, var_t2__blk1035_dn0, var_t2__blk1035_dn2, var_t2__blk1035_dn6, var_t2__blk1035_dn7, var_t2__blk1035_dn10, var_t2__blk1035_dn11, var_t2__blk1035_dn12, var_t2__blk1035_dn17,)
    }
};
        var_t2__blk1035 = assign33000_e48080;
        var_t2__blk1035_dn0 = assign33000_e48080_d_n0;
        var_t2__blk1035_dn2 = assign33000_e48080_d_n2;
        var_t2__blk1035_dn6 = assign33000_e48080_d_n6;
        var_t2__blk1035_dn7 = assign33000_e48080_d_n7;
        var_t2__blk1035_dn10 = assign33000_e48080_d_n10;
        var_t2__blk1035_dn11 = assign33000_e48080_d_n11;
        var_t2__blk1035_dn12 = assign33000_e48080_d_n12;
        var_t2__blk1035_dn17 = assign33000_e48080_d_n17;

        let (assign33010_e48097, assign33010_e48097_d_n0, assign33010_e48097_d_n2, assign33010_e48097_d_n6, assign33010_e48097_d_n7, assign33010_e48097_d_n10, assign33010_e48097_d_n11, assign33010_e48097_d_n12, assign33010_e48097_d_n17,) = {
    if (((var_guard1032 != 0.0) && (var_guard1077 != 0.0)) && (var_guard1078 == 0.0)) {
        let assign33010_e48091: f64 = (var_vbdj * 0.5);
        let assign33010_e48093: f64 = (assign33010_e48091 * var_t2__blk1035);
        let assign33010_e48094: f64 = (var_t1__blk1034 + assign33010_e48093);
        let assign33010_e48095: f64 = (var_vbdj * assign33010_e48094);
        (assign33010_e48095, (var_vbdj * (assign33010_e48091 * var_t2__blk1035_dn0)), (var_vbdj * (assign33010_e48091 * var_t2__blk1035_dn2)), ((var_vbdj_dn6 * assign33010_e48094) + (var_vbdj * (var_t1__blk1034_dn6 + (((var_vbdj_dn6 * 0.5) * var_t2__blk1035) + (assign33010_e48091 * var_t2__blk1035_dn6))))), (var_vbdj * (var_t1__blk1034_dn7 + (assign33010_e48091 * var_t2__blk1035_dn7))), (var_vbdj * (var_t1__blk1034_dn10 + (assign33010_e48091 * var_t2__blk1035_dn10))), (var_vbdj * (assign33010_e48091 * var_t2__blk1035_dn11)), ((var_vbdj_dn12 * assign33010_e48094) + (var_vbdj * (var_t1__blk1034_dn12 + (((var_vbdj_dn12 * 0.5) * var_t2__blk1035) + (assign33010_e48091 * var_t2__blk1035_dn12))))), (var_vbdj * (assign33010_e48091 * var_t2__blk1035_dn17)),)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33010_e48097;
        var_qbd_dn0 = assign33010_e48097_d_n0;
        var_qbd_dn2 = assign33010_e48097_d_n2;
        var_qbd_dn6 = assign33010_e48097_d_n6;
        var_qbd_dn7 = assign33010_e48097_d_n7;
        var_qbd_dn10 = assign33010_e48097_d_n10;
        var_qbd_dn11 = assign33010_e48097_d_n11;
        var_qbd_dn12 = assign33010_e48097_d_n12;
        var_qbd_dn17 = assign33010_e48097_d_n17;

        let (assign33020_e48106,) = {
    if ((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) {
        let assign33020_e48104: f64 = (p.p181 * p.p4);
        (assign33020_e48104,)
    } else {
        (var_czbdswg,)
    }
};
        var_czbdswg = assign33020_e48106;

        let assign33030_e48109: f64 = if var_vbdj < 0.0 { 1.0 } else { 0.0 };
        var_guard1085 = assign33030_e48109;

        let assign33040_e48112: f64 = if var_czbd > 0.0 { 1.0 } else { 0.0 };
        var_guard1086 = assign33040_e48112;

        let (assign33050_e48127, assign33050_e48127_d_n6, assign33050_e48127_d_n7, assign33050_e48127_d_n12,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) && (var_guard1085 != 0.0)) && (var_guard1086 != 0.0)) {
        let assign33050_e48124: f64 = (var_vbdj / p.p185);
        let assign33050_e48125: f64 = (1.0 - assign33050_e48124);
        (assign33050_e48125, (-(var_vbdj_dn6 / p.p185)), 0.0, (-(var_vbdj_dn12 / p.p185)),)
    } else {
        (var_arg__blk1059, var_arg__blk1059_dn6, var_arg__blk1059_dn7, var_arg__blk1059_dn12,)
    }
};
        var_arg__blk1059 = assign33050_e48127;
        var_arg__blk1059_dn6 = assign33050_e48127_d_n6;
        var_arg__blk1059_dn7 = assign33050_e48127_d_n7;
        var_arg__blk1059_dn12 = assign33050_e48127_d_n12;

        let assign33060_e48130: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        var_guard1087 = assign33060_e48130;

        let (assign33070_e48146, assign33070_e48146_d_n6, assign33070_e48146_d_n7, assign33070_e48146_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) && (var_guard1085 != 0.0)) && (var_guard1086 != 0.0)) && (var_guard1087 != 0.0)) {
        let assign33070_e48143: f64 = (var_arg__blk1059).sqrt();
        let assign33070_e48144: f64 = (1.0 / assign33070_e48143);
        (assign33070_e48144, (-((var_arg__blk1059_dn6 / (2.0 * assign33070_e48143)) / (assign33070_e48143 * assign33070_e48143))), (-((var_arg__blk1059_dn7 / (2.0 * assign33070_e48143)) / (assign33070_e48143 * assign33070_e48143))), (-((var_arg__blk1059_dn12 / (2.0 * assign33070_e48143)) / (assign33070_e48143 * assign33070_e48143))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign33070_e48146;
        var_sarg_dn6 = assign33070_e48146_d_n6;
        var_sarg_dn7 = assign33070_e48146_d_n7;
        var_sarg_dn12 = assign33070_e48146_d_n12;

        let (assign33080_e48163, assign33080_e48163_d_n6, assign33080_e48163_d_n7, assign33080_e48163_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) && (var_guard1085 != 0.0)) && (var_guard1086 != 0.0)) && (var_guard1087 == 0.0)) {
        let assign33080_e48160: f64 = (-p.p182);
        let assign33080_e48161: f64 = (var_arg__blk1059).powf(assign33080_e48160);
        (assign33080_e48161, if 0.0 == 0.0 && ((assign33080_e48160) as f64).is_finite() && ((assign33080_e48160) as f64).fract() == 0.0 { if assign33080_e48160 == 0.0 { 0.0 } else { (assign33080_e48160 * ((var_arg__blk1059).powf(assign33080_e48160 - 1.0) * var_arg__blk1059_dn6)) } } else { (assign33080_e48161 * (assign33080_e48160 * (var_arg__blk1059_dn6 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign33080_e48160) as f64).is_finite() && ((assign33080_e48160) as f64).fract() == 0.0 { if assign33080_e48160 == 0.0 { 0.0 } else { (assign33080_e48160 * ((var_arg__blk1059).powf(assign33080_e48160 - 1.0) * var_arg__blk1059_dn7)) } } else { (assign33080_e48161 * (assign33080_e48160 * (var_arg__blk1059_dn7 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign33080_e48160) as f64).is_finite() && ((assign33080_e48160) as f64).fract() == 0.0 { if assign33080_e48160 == 0.0 { 0.0 } else { (assign33080_e48160 * ((var_arg__blk1059).powf(assign33080_e48160 - 1.0) * var_arg__blk1059_dn12)) } } else { (assign33080_e48161 * (assign33080_e48160 * (var_arg__blk1059_dn12 / var_arg__blk1059))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign33080_e48163;
        var_sarg_dn6 = assign33080_e48163_d_n6;
        var_sarg_dn7 = assign33080_e48163_d_n7;
        var_sarg_dn12 = assign33080_e48163_d_n12;

        let (assign33090_e48186, assign33090_e48186_d_n0, assign33090_e48186_d_n2, assign33090_e48186_d_n6, assign33090_e48186_d_n7, assign33090_e48186_d_n10, assign33090_e48186_d_n11, assign33090_e48186_d_n12, assign33090_e48186_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) && (var_guard1085 != 0.0)) && (var_guard1086 != 0.0)) {
        let assign33090_e48174: f64 = (p.p185 * var_czbd);
        let assign33090_e48178: f64 = (var_arg__blk1059 * var_sarg);
        let assign33090_e48179: f64 = (1.0 - assign33090_e48178);
        let assign33090_e48180: f64 = (assign33090_e48174 * assign33090_e48179);
        let assign33090_e48183: f64 = (1.0 - p.p182);
        let assign33090_e48184: f64 = (assign33090_e48180 / assign33090_e48183);
        (assign33090_e48184, 0.0, 0.0, ((assign33090_e48174 * (-((var_arg__blk1059_dn6 * var_sarg) + (var_arg__blk1059 * var_sarg_dn6)))) / assign33090_e48183), ((assign33090_e48174 * (-((var_arg__blk1059_dn7 * var_sarg) + (var_arg__blk1059 * var_sarg_dn7)))) / assign33090_e48183), 0.0, 0.0, ((assign33090_e48174 * (-((var_arg__blk1059_dn12 * var_sarg) + (var_arg__blk1059 * var_sarg_dn12)))) / assign33090_e48183), 0.0,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33090_e48186;
        var_qbd_dn0 = assign33090_e48186_d_n0;
        var_qbd_dn2 = assign33090_e48186_d_n2;
        var_qbd_dn6 = assign33090_e48186_d_n6;
        var_qbd_dn7 = assign33090_e48186_d_n7;
        var_qbd_dn10 = assign33090_e48186_d_n10;
        var_qbd_dn11 = assign33090_e48186_d_n11;
        var_qbd_dn12 = assign33090_e48186_d_n12;
        var_qbd_dn17 = assign33090_e48186_d_n17;

        let (assign33100_e48198, assign33100_e48198_d_n0, assign33100_e48198_d_n2, assign33100_e48198_d_n6, assign33100_e48198_d_n7, assign33100_e48198_d_n10, assign33100_e48198_d_n11, assign33100_e48198_d_n12, assign33100_e48198_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) && (var_guard1085 != 0.0)) && (var_guard1086 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33100_e48198;
        var_qbd_dn0 = assign33100_e48198_d_n0;
        var_qbd_dn2 = assign33100_e48198_d_n2;
        var_qbd_dn6 = assign33100_e48198_d_n6;
        var_qbd_dn7 = assign33100_e48198_d_n7;
        var_qbd_dn10 = assign33100_e48198_d_n10;
        var_qbd_dn11 = assign33100_e48198_d_n11;
        var_qbd_dn12 = assign33100_e48198_d_n12;
        var_qbd_dn17 = assign33100_e48198_d_n17;

        let assign33110_e48201: f64 = if var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        var_guard1088 = assign33110_e48201;

        let (assign33120_e48216, assign33120_e48216_d_n6, assign33120_e48216_d_n7, assign33120_e48216_d_n12,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) && (var_guard1085 != 0.0)) && (var_guard1088 != 0.0)) {
        let assign33120_e48213: f64 = (var_vbdj / p.p187);
        let assign33120_e48214: f64 = (1.0 - assign33120_e48213);
        (assign33120_e48214, (-(var_vbdj_dn6 / p.p187)), 0.0, (-(var_vbdj_dn12 / p.p187)),)
    } else {
        (var_arg__blk1059, var_arg__blk1059_dn6, var_arg__blk1059_dn7, var_arg__blk1059_dn12,)
    }
};
        var_arg__blk1059 = assign33120_e48216;
        var_arg__blk1059_dn6 = assign33120_e48216_d_n6;
        var_arg__blk1059_dn7 = assign33120_e48216_d_n7;
        var_arg__blk1059_dn12 = assign33120_e48216_d_n12;

        let assign33130_e48219: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        var_guard1089 = assign33130_e48219;

        let (assign33140_e48235, assign33140_e48235_d_n6, assign33140_e48235_d_n7, assign33140_e48235_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) && (var_guard1085 != 0.0)) && (var_guard1088 != 0.0)) && (var_guard1089 != 0.0)) {
        let assign33140_e48232: f64 = (var_arg__blk1059).sqrt();
        let assign33140_e48233: f64 = (1.0 / assign33140_e48232);
        (assign33140_e48233, (-((var_arg__blk1059_dn6 / (2.0 * assign33140_e48232)) / (assign33140_e48232 * assign33140_e48232))), (-((var_arg__blk1059_dn7 / (2.0 * assign33140_e48232)) / (assign33140_e48232 * assign33140_e48232))), (-((var_arg__blk1059_dn12 / (2.0 * assign33140_e48232)) / (assign33140_e48232 * assign33140_e48232))),)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign33140_e48235;
        var_sarg_dn6 = assign33140_e48235_d_n6;
        var_sarg_dn7 = assign33140_e48235_d_n7;
        var_sarg_dn12 = assign33140_e48235_d_n12;

        let (assign33150_e48252, assign33150_e48252_d_n6, assign33150_e48252_d_n7, assign33150_e48252_d_n12,) = {
    if (((((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) && (var_guard1085 != 0.0)) && (var_guard1088 != 0.0)) && (var_guard1089 == 0.0)) {
        let assign33150_e48249: f64 = (-p.p184);
        let assign33150_e48250: f64 = (var_arg__blk1059).powf(assign33150_e48249);
        (assign33150_e48250, if 0.0 == 0.0 && ((assign33150_e48249) as f64).is_finite() && ((assign33150_e48249) as f64).fract() == 0.0 { if assign33150_e48249 == 0.0 { 0.0 } else { (assign33150_e48249 * ((var_arg__blk1059).powf(assign33150_e48249 - 1.0) * var_arg__blk1059_dn6)) } } else { (assign33150_e48250 * (assign33150_e48249 * (var_arg__blk1059_dn6 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign33150_e48249) as f64).is_finite() && ((assign33150_e48249) as f64).fract() == 0.0 { if assign33150_e48249 == 0.0 { 0.0 } else { (assign33150_e48249 * ((var_arg__blk1059).powf(assign33150_e48249 - 1.0) * var_arg__blk1059_dn7)) } } else { (assign33150_e48250 * (assign33150_e48249 * (var_arg__blk1059_dn7 / var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign33150_e48249) as f64).is_finite() && ((assign33150_e48249) as f64).fract() == 0.0 { if assign33150_e48249 == 0.0 { 0.0 } else { (assign33150_e48249 * ((var_arg__blk1059).powf(assign33150_e48249 - 1.0) * var_arg__blk1059_dn12)) } } else { (assign33150_e48250 * (assign33150_e48249 * (var_arg__blk1059_dn12 / var_arg__blk1059))) },)
    } else {
        (var_sarg, var_sarg_dn6, var_sarg_dn7, var_sarg_dn12,)
    }
};
        var_sarg = assign33150_e48252;
        var_sarg_dn6 = assign33150_e48252_d_n6;
        var_sarg_dn7 = assign33150_e48252_d_n7;
        var_sarg_dn12 = assign33150_e48252_d_n12;

        let (assign33160_e48277, assign33160_e48277_d_n0, assign33160_e48277_d_n2, assign33160_e48277_d_n6, assign33160_e48277_d_n7, assign33160_e48277_d_n10, assign33160_e48277_d_n11, assign33160_e48277_d_n12, assign33160_e48277_d_n17,) = {
    if ((((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) && (var_guard1085 != 0.0)) && (var_guard1088 != 0.0)) {
        let assign33160_e48264: f64 = (p.p187 * var_czbdswg);
        let assign33160_e48268: f64 = (var_arg__blk1059 * var_sarg);
        let assign33160_e48269: f64 = (1.0 - assign33160_e48268);
        let assign33160_e48270: f64 = (assign33160_e48264 * assign33160_e48269);
        let assign33160_e48273: f64 = (1.0 - p.p184);
        let assign33160_e48274: f64 = (assign33160_e48270 / assign33160_e48273);
        let assign33160_e48275: f64 = (var_qbd + assign33160_e48274);
        (assign33160_e48275, var_qbd_dn0, var_qbd_dn2, (var_qbd_dn6 + ((assign33160_e48264 * (-((var_arg__blk1059_dn6 * var_sarg) + (var_arg__blk1059 * var_sarg_dn6)))) / assign33160_e48273)), (var_qbd_dn7 + ((assign33160_e48264 * (-((var_arg__blk1059_dn7 * var_sarg) + (var_arg__blk1059 * var_sarg_dn7)))) / assign33160_e48273)), var_qbd_dn10, var_qbd_dn11, (var_qbd_dn12 + ((assign33160_e48264 * (-((var_arg__blk1059_dn12 * var_sarg) + (var_arg__blk1059 * var_sarg_dn12)))) / assign33160_e48273)), var_qbd_dn17,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33160_e48277;
        var_qbd_dn0 = assign33160_e48277_d_n0;
        var_qbd_dn2 = assign33160_e48277_d_n2;
        var_qbd_dn6 = assign33160_e48277_d_n6;
        var_qbd_dn7 = assign33160_e48277_d_n7;
        var_qbd_dn10 = assign33160_e48277_d_n10;
        var_qbd_dn11 = assign33160_e48277_d_n11;
        var_qbd_dn12 = assign33160_e48277_d_n12;
        var_qbd_dn17 = assign33160_e48277_d_n17;

        let (assign33170_e48289, assign33170_e48289_d_n6, assign33170_e48289_d_n7, assign33170_e48289_d_n10, assign33170_e48289_d_n12,) = {
    if (((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) && (var_guard1085 == 0.0)) {
        let assign33170_e48287: f64 = (var_czbd + var_czbdswg);
        (assign33170_e48287, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk1034, var_t1__blk1034_dn6, var_t1__blk1034_dn7, var_t1__blk1034_dn10, var_t1__blk1034_dn12,)
    }
};
        var_t1__blk1034 = assign33170_e48289;
        var_t1__blk1034_dn6 = assign33170_e48289_d_n6;
        var_t1__blk1034_dn7 = assign33170_e48289_d_n7;
        var_t1__blk1034_dn10 = assign33170_e48289_d_n10;
        var_t1__blk1034_dn12 = assign33170_e48289_d_n12;

        let (assign33180_e48309, assign33180_e48309_d_n0, assign33180_e48309_d_n2, assign33180_e48309_d_n6, assign33180_e48309_d_n7, assign33180_e48309_d_n10, assign33180_e48309_d_n11, assign33180_e48309_d_n12, assign33180_e48309_d_n17,) = {
    if (((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) && (var_guard1085 == 0.0)) {
        let assign33180_e48299: f64 = (var_czbd * p.p182);
        let assign33180_e48301: f64 = (assign33180_e48299 / p.p185);
        let assign33180_e48304: f64 = (var_czbdswg * p.p184);
        let assign33180_e48306: f64 = (assign33180_e48304 / p.p187);
        let assign33180_e48307: f64 = (assign33180_e48301 + assign33180_e48306);
        (assign33180_e48307, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2__blk1035, var_t2__blk1035_dn0, var_t2__blk1035_dn2, var_t2__blk1035_dn6, var_t2__blk1035_dn7, var_t2__blk1035_dn10, var_t2__blk1035_dn11, var_t2__blk1035_dn12, var_t2__blk1035_dn17,)
    }
};
        var_t2__blk1035 = assign33180_e48309;
        var_t2__blk1035_dn0 = assign33180_e48309_d_n0;
        var_t2__blk1035_dn2 = assign33180_e48309_d_n2;
        var_t2__blk1035_dn6 = assign33180_e48309_d_n6;
        var_t2__blk1035_dn7 = assign33180_e48309_d_n7;
        var_t2__blk1035_dn10 = assign33180_e48309_d_n10;
        var_t2__blk1035_dn11 = assign33180_e48309_d_n11;
        var_t2__blk1035_dn12 = assign33180_e48309_d_n12;
        var_t2__blk1035_dn17 = assign33180_e48309_d_n17;

        let (assign33190_e48327, assign33190_e48327_d_n0, assign33190_e48327_d_n2, assign33190_e48327_d_n6, assign33190_e48327_d_n7, assign33190_e48327_d_n10, assign33190_e48327_d_n11, assign33190_e48327_d_n12, assign33190_e48327_d_n17,) = {
    if (((var_guard1032 != 0.0) && (var_guard1077 == 0.0)) && (var_guard1085 == 0.0)) {
        let assign33190_e48321: f64 = (var_vbdj * 0.5);
        let assign33190_e48323: f64 = (assign33190_e48321 * var_t2__blk1035);
        let assign33190_e48324: f64 = (var_t1__blk1034 + assign33190_e48323);
        let assign33190_e48325: f64 = (var_vbdj * assign33190_e48324);
        (assign33190_e48325, (var_vbdj * (assign33190_e48321 * var_t2__blk1035_dn0)), (var_vbdj * (assign33190_e48321 * var_t2__blk1035_dn2)), ((var_vbdj_dn6 * assign33190_e48324) + (var_vbdj * (var_t1__blk1034_dn6 + (((var_vbdj_dn6 * 0.5) * var_t2__blk1035) + (assign33190_e48321 * var_t2__blk1035_dn6))))), (var_vbdj * (var_t1__blk1034_dn7 + (assign33190_e48321 * var_t2__blk1035_dn7))), (var_vbdj * (var_t1__blk1034_dn10 + (assign33190_e48321 * var_t2__blk1035_dn10))), (var_vbdj * (assign33190_e48321 * var_t2__blk1035_dn11)), ((var_vbdj_dn12 * assign33190_e48324) + (var_vbdj * (var_t1__blk1034_dn12 + (((var_vbdj_dn12 * 0.5) * var_t2__blk1035) + (assign33190_e48321 * var_t2__blk1035_dn12))))), (var_vbdj * (assign33190_e48321 * var_t2__blk1035_dn17)),)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33190_e48327;
        var_qbd_dn0 = assign33190_e48327_d_n0;
        var_qbd_dn2 = assign33190_e48327_d_n2;
        var_qbd_dn6 = assign33190_e48327_d_n6;
        var_qbd_dn7 = assign33190_e48327_d_n7;
        var_qbd_dn10 = assign33190_e48327_d_n10;
        var_qbd_dn11 = assign33190_e48327_d_n11;
        var_qbd_dn12 = assign33190_e48327_d_n12;
        var_qbd_dn17 = assign33190_e48327_d_n17;

        let assign33200_e48330: f64 = if var_czbs > 0.0 { 1.0 } else { 0.0 };
        var_guard1090 = assign33200_e48330;

        let (assign33210_e48343, assign33210_e48343_d_n0, assign33210_e48343_d_n2, assign33210_e48343_d_n6, assign33210_e48343_d_n7, assign33210_e48343_d_n10, assign33210_e48343_d_n11, assign33210_e48343_d_n12, assign33210_e48343_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1090 != 0.0)) {
        let assign33210_e48335: f64 = (-1.6021918e-19);
        let assign33210_e48337: f64 = (assign33210_e48335 * var_uc_nsubs);
        let assign33210_e48339: f64 = (assign33210_e48337 * var_xp_max);
        let assign33210_e48341: f64 = (assign33210_e48339 * p.p3);
        (assign33210_e48341, (((assign33210_e48335 * var_uc_nsubs_dn0) * var_xp_max) * p.p3), (((assign33210_e48335 * var_uc_nsubs_dn2) * var_xp_max) * p.p3), (((assign33210_e48335 * var_uc_nsubs_dn6) * var_xp_max) * p.p3), (((assign33210_e48335 * var_uc_nsubs_dn7) * var_xp_max) * p.p3), (((assign33210_e48335 * var_uc_nsubs_dn10) * var_xp_max) * p.p3), (((assign33210_e48335 * var_uc_nsubs_dn11) * var_xp_max) * p.p3), (((assign33210_e48335 * var_uc_nsubs_dn12) * var_xp_max) * p.p3), (((assign33210_e48335 * var_uc_nsubs_dn17) * var_xp_max) * p.p3),)
    } else {
        (var_qbs_max, var_qbs_max_dn0, var_qbs_max_dn2, var_qbs_max_dn6, var_qbs_max_dn7, var_qbs_max_dn10, var_qbs_max_dn11, var_qbs_max_dn12, var_qbs_max_dn17,)
    }
};
        var_qbs_max = assign33210_e48343;
        var_qbs_max_dn0 = assign33210_e48343_d_n0;
        var_qbs_max_dn2 = assign33210_e48343_d_n2;
        var_qbs_max_dn6 = assign33210_e48343_d_n6;
        var_qbs_max_dn7 = assign33210_e48343_d_n7;
        var_qbs_max_dn10 = assign33210_e48343_d_n10;
        var_qbs_max_dn11 = assign33210_e48343_d_n11;
        var_qbs_max_dn12 = assign33210_e48343_d_n12;
        var_qbs_max_dn17 = assign33210_e48343_d_n17;

        let (assign33220_e48352, assign33220_e48352_d_n0, assign33220_e48352_d_n2, assign33220_e48352_d_n6, assign33220_e48352_d_n7, assign33220_e48352_d_n10, assign33220_e48352_d_n11, assign33220_e48352_d_n12, assign33220_e48352_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1090 != 0.0)) {
        let assign33220_e48349: f64 = (-var_qbs_max);
        let assign33220_e48350: f64 = (0.001 * assign33220_e48349);
        (assign33220_e48350, (0.001 * (-var_qbs_max_dn0)), (0.001 * (-var_qbs_max_dn2)), (0.001 * (-var_qbs_max_dn6)), (0.001 * (-var_qbs_max_dn7)), (0.001 * (-var_qbs_max_dn10)), (0.001 * (-var_qbs_max_dn11)), (0.001 * (-var_qbs_max_dn12)), (0.001 * (-var_qbs_max_dn17)),)
    } else {
        (var_dlt_qbs, var_dlt_qbs_dn0, var_dlt_qbs_dn2, var_dlt_qbs_dn6, var_dlt_qbs_dn7, var_dlt_qbs_dn10, var_dlt_qbs_dn11, var_dlt_qbs_dn12, var_dlt_qbs_dn17,)
    }
};
        var_dlt_qbs = assign33220_e48352;
        var_dlt_qbs_dn0 = assign33220_e48352_d_n0;
        var_dlt_qbs_dn2 = assign33220_e48352_d_n2;
        var_dlt_qbs_dn6 = assign33220_e48352_d_n6;
        var_dlt_qbs_dn7 = assign33220_e48352_d_n7;
        var_dlt_qbs_dn10 = assign33220_e48352_d_n10;
        var_dlt_qbs_dn11 = assign33220_e48352_d_n11;
        var_dlt_qbs_dn12 = assign33220_e48352_d_n12;
        var_dlt_qbs_dn17 = assign33220_e48352_d_n17;

        let (assign33230_e48364, assign33230_e48364_d_n0, assign33230_e48364_d_n2, assign33230_e48364_d_n6, assign33230_e48364_d_n7, assign33230_e48364_d_n10, assign33230_e48364_d_n11, assign33230_e48364_d_n12, assign33230_e48364_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1090 != 0.0)) {
        let assign33230_e48357: f64 = (-var_qbs_max);
        let assign33230_e48359: f64 = (-var_qbs);
        let assign33230_e48360: f64 = (assign33230_e48357 - assign33230_e48359);
        let assign33230_e48362: f64 = (assign33230_e48360 - var_dlt_qbs);
        (assign33230_e48362, (((-var_qbs_max_dn0) - (-var_qbs_dn0)) - var_dlt_qbs_dn0), (((-var_qbs_max_dn2) - (-var_qbs_dn2)) - var_dlt_qbs_dn2), (((-var_qbs_max_dn6) - (-var_qbs_dn6)) - var_dlt_qbs_dn6), (((-var_qbs_max_dn7) - (-var_qbs_dn7)) - var_dlt_qbs_dn7), (((-var_qbs_max_dn10) - (-var_qbs_dn10)) - var_dlt_qbs_dn10), (((-var_qbs_max_dn11) - (-var_qbs_dn11)) - var_dlt_qbs_dn11), (((-var_qbs_max_dn12) - (-var_qbs_dn12)) - var_dlt_qbs_dn12), (((-var_qbs_max_dn17) - (-var_qbs_dn17)) - var_dlt_qbs_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign33230_e48364;
        var_tmf1_dn0 = assign33230_e48364_d_n0;
        var_tmf1_dn2 = assign33230_e48364_d_n2;
        var_tmf1_dn6 = assign33230_e48364_d_n6;
        var_tmf1_dn7 = assign33230_e48364_d_n7;
        var_tmf1_dn10 = assign33230_e48364_d_n10;
        var_tmf1_dn11 = assign33230_e48364_d_n11;
        var_tmf1_dn12 = assign33230_e48364_d_n12;
        var_tmf1_dn17 = assign33230_e48364_d_n17;

        let (assign33240_e48375, assign33240_e48375_d_n0, assign33240_e48375_d_n2, assign33240_e48375_d_n6, assign33240_e48375_d_n7, assign33240_e48375_d_n10, assign33240_e48375_d_n11, assign33240_e48375_d_n12, assign33240_e48375_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1090 != 0.0)) {
        let assign33240_e48370: f64 = (-var_qbs_max);
        let assign33240_e48371: f64 = (4.0 * assign33240_e48370);
        let assign33240_e48373: f64 = (assign33240_e48371 * var_dlt_qbs);
        (assign33240_e48373, (((4.0 * (-var_qbs_max_dn0)) * var_dlt_qbs) + (assign33240_e48371 * var_dlt_qbs_dn0)), (((4.0 * (-var_qbs_max_dn2)) * var_dlt_qbs) + (assign33240_e48371 * var_dlt_qbs_dn2)), (((4.0 * (-var_qbs_max_dn6)) * var_dlt_qbs) + (assign33240_e48371 * var_dlt_qbs_dn6)), (((4.0 * (-var_qbs_max_dn7)) * var_dlt_qbs) + (assign33240_e48371 * var_dlt_qbs_dn7)), (((4.0 * (-var_qbs_max_dn10)) * var_dlt_qbs) + (assign33240_e48371 * var_dlt_qbs_dn10)), (((4.0 * (-var_qbs_max_dn11)) * var_dlt_qbs) + (assign33240_e48371 * var_dlt_qbs_dn11)), (((4.0 * (-var_qbs_max_dn12)) * var_dlt_qbs) + (assign33240_e48371 * var_dlt_qbs_dn12)), (((4.0 * (-var_qbs_max_dn17)) * var_dlt_qbs) + (assign33240_e48371 * var_dlt_qbs_dn17)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign33240_e48375;
        var_tmf2_dn0 = assign33240_e48375_d_n0;
        var_tmf2_dn2 = assign33240_e48375_d_n2;
        var_tmf2_dn6 = assign33240_e48375_d_n6;
        var_tmf2_dn7 = assign33240_e48375_d_n7;
        var_tmf2_dn10 = assign33240_e48375_d_n10;
        var_tmf2_dn11 = assign33240_e48375_d_n11;
        var_tmf2_dn12 = assign33240_e48375_d_n12;
        var_tmf2_dn17 = assign33240_e48375_d_n17;

        let (assign33250_e48387, assign33250_e48387_d_n0, assign33250_e48387_d_n2, assign33250_e48387_d_n6, assign33250_e48387_d_n7, assign33250_e48387_d_n10, assign33250_e48387_d_n11, assign33250_e48387_d_n12, assign33250_e48387_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1090 != 0.0)) {
        let (assign33250_e48385, assign33250_e48385_d_n0, assign33250_e48385_d_n2, assign33250_e48385_d_n6, assign33250_e48385_d_n7, assign33250_e48385_d_n10, assign33250_e48385_d_n11, assign33250_e48385_d_n12, assign33250_e48385_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign33250_e48384: f64 = (-var_tmf2);
                (assign33250_e48384, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign33250_e48385, assign33250_e48385_d_n0, assign33250_e48385_d_n2, assign33250_e48385_d_n6, assign33250_e48385_d_n7, assign33250_e48385_d_n10, assign33250_e48385_d_n11, assign33250_e48385_d_n12, assign33250_e48385_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign33250_e48387;
        var_tmf2_dn0 = assign33250_e48387_d_n0;
        var_tmf2_dn2 = assign33250_e48387_d_n2;
        var_tmf2_dn6 = assign33250_e48387_d_n6;
        var_tmf2_dn7 = assign33250_e48387_d_n7;
        var_tmf2_dn10 = assign33250_e48387_d_n10;
        var_tmf2_dn11 = assign33250_e48387_d_n11;
        var_tmf2_dn12 = assign33250_e48387_d_n12;
        var_tmf2_dn17 = assign33250_e48387_d_n17;

        *var_arg__blk1059_slot = var_arg__blk1059;
        *var_arg__blk1059_dn12_slot = var_arg__blk1059_dn12;
        *var_arg__blk1059_dn6_slot = var_arg__blk1059_dn6;
        *var_arg__blk1059_dn7_slot = var_arg__blk1059_dn7;
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
        *var_guard1083_slot = var_guard1083;
        *var_guard1084_slot = var_guard1084;
        *var_guard1085_slot = var_guard1085;
        *var_guard1086_slot = var_guard1086;
        *var_guard1087_slot = var_guard1087;
        *var_guard1088_slot = var_guard1088;
        *var_guard1089_slot = var_guard1089;
        *var_guard1090_slot = var_guard1090;
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
        *var_t1__blk1034_slot = var_t1__blk1034;
        *var_t1__blk1034_dn10_slot = var_t1__blk1034_dn10;
        *var_t1__blk1034_dn12_slot = var_t1__blk1034_dn12;
        *var_t1__blk1034_dn6_slot = var_t1__blk1034_dn6;
        *var_t1__blk1034_dn7_slot = var_t1__blk1034_dn7;
        *var_t2__blk1035_slot = var_t2__blk1035;
        *var_t2__blk1035_dn0_slot = var_t2__blk1035_dn0;
        *var_t2__blk1035_dn10_slot = var_t2__blk1035_dn10;
        *var_t2__blk1035_dn11_slot = var_t2__blk1035_dn11;
        *var_t2__blk1035_dn12_slot = var_t2__blk1035_dn12;
        *var_t2__blk1035_dn17_slot = var_t2__blk1035_dn17;
        *var_t2__blk1035_dn2_slot = var_t2__blk1035_dn2;
        *var_t2__blk1035_dn6_slot = var_t2__blk1035_dn6;
        *var_t2__blk1035_dn7_slot = var_t2__blk1035_dn7;
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
        var_guard1032: f64,
        var_guard1090: f64,
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
        var_guard1091_slot: &mut f64,
        var_guard1097_slot: &mut f64,
        var_guard1124_slot: &mut f64,
        var_guard1125_slot: &mut f64,
        var_guard1126_slot: &mut f64,
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
        var_t10__blk1092_slot: &mut f64,
        var_t11__blk1093_slot: &mut f64,
        var_t12_slot: &mut f64,
        var_t12__blk1108_slot: &mut f64,
        var_t12__blk1108_dn0_slot: &mut f64,
        var_t12__blk1108_dn10_slot: &mut f64,
        var_t12__blk1108_dn11_slot: &mut f64,
        var_t12__blk1108_dn12_slot: &mut f64,
        var_t12__blk1108_dn17_slot: &mut f64,
        var_t12__blk1108_dn2_slot: &mut f64,
        var_t12__blk1108_dn6_slot: &mut f64,
        var_t12__blk1108_dn7_slot: &mut f64,
        var_t12_dn0_slot: &mut f64,
        var_t12_dn10_slot: &mut f64,
        var_t12_dn11_slot: &mut f64,
        var_t12_dn12_slot: &mut f64,
        var_t12_dn17_slot: &mut f64,
        var_t12_dn2_slot: &mut f64,
        var_t12_dn6_slot: &mut f64,
        var_t12_dn7_slot: &mut f64,
        var_t1__blk1095_slot: &mut f64,
        var_t1__blk1095_dn0_slot: &mut f64,
        var_t1__blk1095_dn10_slot: &mut f64,
        var_t1__blk1095_dn11_slot: &mut f64,
        var_t1__blk1095_dn12_slot: &mut f64,
        var_t1__blk1095_dn17_slot: &mut f64,
        var_t1__blk1095_dn2_slot: &mut f64,
        var_t1__blk1095_dn6_slot: &mut f64,
        var_t1__blk1095_dn7_slot: &mut f64,
        var_t2__blk1096_slot: &mut f64,
        var_t2__blk1096_dn0_slot: &mut f64,
        var_t2__blk1096_dn10_slot: &mut f64,
        var_t2__blk1096_dn11_slot: &mut f64,
        var_t2__blk1096_dn12_slot: &mut f64,
        var_t2__blk1096_dn17_slot: &mut f64,
        var_t2__blk1096_dn2_slot: &mut f64,
        var_t2__blk1096_dn6_slot: &mut f64,
        var_t2__blk1096_dn7_slot: &mut f64,
        var_t7__blk1109_slot: &mut f64,
        var_t7__blk1109_dn0_slot: &mut f64,
        var_t7__blk1109_dn10_slot: &mut f64,
        var_t7__blk1109_dn11_slot: &mut f64,
        var_t7__blk1109_dn12_slot: &mut f64,
        var_t7__blk1109_dn17_slot: &mut f64,
        var_t7__blk1109_dn2_slot: &mut f64,
        var_t7__blk1109_dn6_slot: &mut f64,
        var_t7__blk1109_dn7_slot: &mut f64,
        var_t8__blk1110_slot: &mut f64,
        var_t8__blk1110_dn0_slot: &mut f64,
        var_t8__blk1110_dn10_slot: &mut f64,
        var_t8__blk1110_dn11_slot: &mut f64,
        var_t8__blk1110_dn12_slot: &mut f64,
        var_t8__blk1110_dn17_slot: &mut f64,
        var_t8__blk1110_dn2_slot: &mut f64,
        var_t8__blk1110_dn6_slot: &mut f64,
        var_t8__blk1110_dn7_slot: &mut f64,
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
        let mut var_guard1091: f64 = *var_guard1091_slot;
        let mut var_guard1097: f64 = *var_guard1097_slot;
        let mut var_guard1124: f64 = *var_guard1124_slot;
        let mut var_guard1125: f64 = *var_guard1125_slot;
        let mut var_guard1126: f64 = *var_guard1126_slot;
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
        let mut var_t10__blk1092: f64 = *var_t10__blk1092_slot;
        let mut var_t11__blk1093: f64 = *var_t11__blk1093_slot;
        let mut var_t12: f64 = *var_t12_slot;
        let mut var_t12__blk1108: f64 = *var_t12__blk1108_slot;
        let mut var_t12__blk1108_dn0: f64 = *var_t12__blk1108_dn0_slot;
        let mut var_t12__blk1108_dn10: f64 = *var_t12__blk1108_dn10_slot;
        let mut var_t12__blk1108_dn11: f64 = *var_t12__blk1108_dn11_slot;
        let mut var_t12__blk1108_dn12: f64 = *var_t12__blk1108_dn12_slot;
        let mut var_t12__blk1108_dn17: f64 = *var_t12__blk1108_dn17_slot;
        let mut var_t12__blk1108_dn2: f64 = *var_t12__blk1108_dn2_slot;
        let mut var_t12__blk1108_dn6: f64 = *var_t12__blk1108_dn6_slot;
        let mut var_t12__blk1108_dn7: f64 = *var_t12__blk1108_dn7_slot;
        let mut var_t12_dn0: f64 = *var_t12_dn0_slot;
        let mut var_t12_dn10: f64 = *var_t12_dn10_slot;
        let mut var_t12_dn11: f64 = *var_t12_dn11_slot;
        let mut var_t12_dn12: f64 = *var_t12_dn12_slot;
        let mut var_t12_dn17: f64 = *var_t12_dn17_slot;
        let mut var_t12_dn2: f64 = *var_t12_dn2_slot;
        let mut var_t12_dn6: f64 = *var_t12_dn6_slot;
        let mut var_t12_dn7: f64 = *var_t12_dn7_slot;
        let mut var_t1__blk1095: f64 = *var_t1__blk1095_slot;
        let mut var_t1__blk1095_dn0: f64 = *var_t1__blk1095_dn0_slot;
        let mut var_t1__blk1095_dn10: f64 = *var_t1__blk1095_dn10_slot;
        let mut var_t1__blk1095_dn11: f64 = *var_t1__blk1095_dn11_slot;
        let mut var_t1__blk1095_dn12: f64 = *var_t1__blk1095_dn12_slot;
        let mut var_t1__blk1095_dn17: f64 = *var_t1__blk1095_dn17_slot;
        let mut var_t1__blk1095_dn2: f64 = *var_t1__blk1095_dn2_slot;
        let mut var_t1__blk1095_dn6: f64 = *var_t1__blk1095_dn6_slot;
        let mut var_t1__blk1095_dn7: f64 = *var_t1__blk1095_dn7_slot;
        let mut var_t2__blk1096: f64 = *var_t2__blk1096_slot;
        let mut var_t2__blk1096_dn0: f64 = *var_t2__blk1096_dn0_slot;
        let mut var_t2__blk1096_dn10: f64 = *var_t2__blk1096_dn10_slot;
        let mut var_t2__blk1096_dn11: f64 = *var_t2__blk1096_dn11_slot;
        let mut var_t2__blk1096_dn12: f64 = *var_t2__blk1096_dn12_slot;
        let mut var_t2__blk1096_dn17: f64 = *var_t2__blk1096_dn17_slot;
        let mut var_t2__blk1096_dn2: f64 = *var_t2__blk1096_dn2_slot;
        let mut var_t2__blk1096_dn6: f64 = *var_t2__blk1096_dn6_slot;
        let mut var_t2__blk1096_dn7: f64 = *var_t2__blk1096_dn7_slot;
        let mut var_t7__blk1109: f64 = *var_t7__blk1109_slot;
        let mut var_t7__blk1109_dn0: f64 = *var_t7__blk1109_dn0_slot;
        let mut var_t7__blk1109_dn10: f64 = *var_t7__blk1109_dn10_slot;
        let mut var_t7__blk1109_dn11: f64 = *var_t7__blk1109_dn11_slot;
        let mut var_t7__blk1109_dn12: f64 = *var_t7__blk1109_dn12_slot;
        let mut var_t7__blk1109_dn17: f64 = *var_t7__blk1109_dn17_slot;
        let mut var_t7__blk1109_dn2: f64 = *var_t7__blk1109_dn2_slot;
        let mut var_t7__blk1109_dn6: f64 = *var_t7__blk1109_dn6_slot;
        let mut var_t7__blk1109_dn7: f64 = *var_t7__blk1109_dn7_slot;
        let mut var_t8__blk1110: f64 = *var_t8__blk1110_slot;
        let mut var_t8__blk1110_dn0: f64 = *var_t8__blk1110_dn0_slot;
        let mut var_t8__blk1110_dn10: f64 = *var_t8__blk1110_dn10_slot;
        let mut var_t8__blk1110_dn11: f64 = *var_t8__blk1110_dn11_slot;
        let mut var_t8__blk1110_dn12: f64 = *var_t8__blk1110_dn12_slot;
        let mut var_t8__blk1110_dn17: f64 = *var_t8__blk1110_dn17_slot;
        let mut var_t8__blk1110_dn2: f64 = *var_t8__blk1110_dn2_slot;
        let mut var_t8__blk1110_dn6: f64 = *var_t8__blk1110_dn6_slot;
        let mut var_t8__blk1110_dn7: f64 = *var_t8__blk1110_dn7_slot;
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

        let (assign33260_e48398, assign33260_e48398_d_n0, assign33260_e48398_d_n2, assign33260_e48398_d_n6, assign33260_e48398_d_n7, assign33260_e48398_d_n10, assign33260_e48398_d_n11, assign33260_e48398_d_n12, assign33260_e48398_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1090 != 0.0)) {
        let assign33260_e48393: f64 = (var_tmf1 * var_tmf1);
        let assign33260_e48395: f64 = (assign33260_e48393 + var_tmf2);
        let assign33260_e48396: f64 = (assign33260_e48395).sqrt();
        (assign33260_e48396, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign33260_e48396)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign33260_e48396)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign33260_e48396)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign33260_e48396)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign33260_e48396)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign33260_e48396)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign33260_e48396)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign33260_e48396)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign33260_e48398;
        var_tmf2_dn0 = assign33260_e48398_d_n0;
        var_tmf2_dn2 = assign33260_e48398_d_n2;
        var_tmf2_dn6 = assign33260_e48398_d_n6;
        var_tmf2_dn7 = assign33260_e48398_d_n7;
        var_tmf2_dn10 = assign33260_e48398_d_n10;
        var_tmf2_dn11 = assign33260_e48398_d_n11;
        var_tmf2_dn12 = assign33260_e48398_d_n12;
        var_tmf2_dn17 = assign33260_e48398_d_n17;

        let (assign33270_e48411, assign33270_e48411_d_n0, assign33270_e48411_d_n2, assign33270_e48411_d_n6, assign33270_e48411_d_n7, assign33270_e48411_d_n10, assign33270_e48411_d_n11, assign33270_e48411_d_n12, assign33270_e48411_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1090 != 0.0)) {
        let assign33270_e48403: f64 = (-var_qbs_max);
        let assign33270_e48407: f64 = (var_tmf1 + var_tmf2);
        let assign33270_e48408: f64 = (0.5 * assign33270_e48407);
        let assign33270_e48409: f64 = (assign33270_e48403 - assign33270_e48408);
        (assign33270_e48409, ((-var_qbs_max_dn0) - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), ((-var_qbs_max_dn2) - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), ((-var_qbs_max_dn6) - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), ((-var_qbs_max_dn7) - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), ((-var_qbs_max_dn10) - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), ((-var_qbs_max_dn11) - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), ((-var_qbs_max_dn12) - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), ((-var_qbs_max_dn17) - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign33270_e48411;
        var_qbs_dn0 = assign33270_e48411_d_n0;
        var_qbs_dn2 = assign33270_e48411_d_n2;
        var_qbs_dn6 = assign33270_e48411_d_n6;
        var_qbs_dn7 = assign33270_e48411_d_n7;
        var_qbs_dn10 = assign33270_e48411_d_n10;
        var_qbs_dn11 = assign33270_e48411_d_n11;
        var_qbs_dn12 = assign33270_e48411_d_n12;
        var_qbs_dn17 = assign33270_e48411_d_n17;

        let (assign33280_e48420, assign33280_e48420_d_n0, assign33280_e48420_d_n2, assign33280_e48420_d_n6, assign33280_e48420_d_n7, assign33280_e48420_d_n10, assign33280_e48420_d_n11, assign33280_e48420_d_n12, assign33280_e48420_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1090 != 0.0)) {
        let assign33280_e48417: f64 = (-1.0);
        let assign33280_e48418: f64 = (var_qbs * assign33280_e48417);
        (assign33280_e48418, (var_qbs_dn0 * assign33280_e48417), (var_qbs_dn2 * assign33280_e48417), (var_qbs_dn6 * assign33280_e48417), (var_qbs_dn7 * assign33280_e48417), (var_qbs_dn10 * assign33280_e48417), (var_qbs_dn11 * assign33280_e48417), (var_qbs_dn12 * assign33280_e48417), (var_qbs_dn17 * assign33280_e48417),)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign33280_e48420;
        var_qbs_dn0 = assign33280_e48420_d_n0;
        var_qbs_dn2 = assign33280_e48420_d_n2;
        var_qbs_dn6 = assign33280_e48420_d_n6;
        var_qbs_dn7 = assign33280_e48420_d_n7;
        var_qbs_dn10 = assign33280_e48420_d_n10;
        var_qbs_dn11 = assign33280_e48420_d_n11;
        var_qbs_dn12 = assign33280_e48420_d_n12;
        var_qbs_dn17 = assign33280_e48420_d_n17;

        let assign33290_e48423: f64 = if var_czbd > 0.0 { 1.0 } else { 0.0 };
        var_guard1091 = assign33290_e48423;

        let (assign33300_e48436, assign33300_e48436_d_n0, assign33300_e48436_d_n2, assign33300_e48436_d_n6, assign33300_e48436_d_n7, assign33300_e48436_d_n10, assign33300_e48436_d_n11, assign33300_e48436_d_n12, assign33300_e48436_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1091 != 0.0)) {
        let assign33300_e48428: f64 = (-1.6021918e-19);
        let assign33300_e48430: f64 = (assign33300_e48428 * var_uc_nsubs);
        let assign33300_e48432: f64 = (assign33300_e48430 * var_xp_max);
        let assign33300_e48434: f64 = (assign33300_e48432 * p.p2);
        (assign33300_e48434, (((assign33300_e48428 * var_uc_nsubs_dn0) * var_xp_max) * p.p2), (((assign33300_e48428 * var_uc_nsubs_dn2) * var_xp_max) * p.p2), (((assign33300_e48428 * var_uc_nsubs_dn6) * var_xp_max) * p.p2), (((assign33300_e48428 * var_uc_nsubs_dn7) * var_xp_max) * p.p2), (((assign33300_e48428 * var_uc_nsubs_dn10) * var_xp_max) * p.p2), (((assign33300_e48428 * var_uc_nsubs_dn11) * var_xp_max) * p.p2), (((assign33300_e48428 * var_uc_nsubs_dn12) * var_xp_max) * p.p2), (((assign33300_e48428 * var_uc_nsubs_dn17) * var_xp_max) * p.p2),)
    } else {
        (var_qbd_max, var_qbd_max_dn0, var_qbd_max_dn2, var_qbd_max_dn6, var_qbd_max_dn7, var_qbd_max_dn10, var_qbd_max_dn11, var_qbd_max_dn12, var_qbd_max_dn17,)
    }
};
        var_qbd_max = assign33300_e48436;
        var_qbd_max_dn0 = assign33300_e48436_d_n0;
        var_qbd_max_dn2 = assign33300_e48436_d_n2;
        var_qbd_max_dn6 = assign33300_e48436_d_n6;
        var_qbd_max_dn7 = assign33300_e48436_d_n7;
        var_qbd_max_dn10 = assign33300_e48436_d_n10;
        var_qbd_max_dn11 = assign33300_e48436_d_n11;
        var_qbd_max_dn12 = assign33300_e48436_d_n12;
        var_qbd_max_dn17 = assign33300_e48436_d_n17;

        let (assign33310_e48445, assign33310_e48445_d_n0, assign33310_e48445_d_n2, assign33310_e48445_d_n6, assign33310_e48445_d_n7, assign33310_e48445_d_n10, assign33310_e48445_d_n11, assign33310_e48445_d_n12, assign33310_e48445_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1091 != 0.0)) {
        let assign33310_e48442: f64 = (-var_qbd_max);
        let assign33310_e48443: f64 = (0.001 * assign33310_e48442);
        (assign33310_e48443, (0.001 * (-var_qbd_max_dn0)), (0.001 * (-var_qbd_max_dn2)), (0.001 * (-var_qbd_max_dn6)), (0.001 * (-var_qbd_max_dn7)), (0.001 * (-var_qbd_max_dn10)), (0.001 * (-var_qbd_max_dn11)), (0.001 * (-var_qbd_max_dn12)), (0.001 * (-var_qbd_max_dn17)),)
    } else {
        (var_dlt_qbd, var_dlt_qbd_dn0, var_dlt_qbd_dn2, var_dlt_qbd_dn6, var_dlt_qbd_dn7, var_dlt_qbd_dn10, var_dlt_qbd_dn11, var_dlt_qbd_dn12, var_dlt_qbd_dn17,)
    }
};
        var_dlt_qbd = assign33310_e48445;
        var_dlt_qbd_dn0 = assign33310_e48445_d_n0;
        var_dlt_qbd_dn2 = assign33310_e48445_d_n2;
        var_dlt_qbd_dn6 = assign33310_e48445_d_n6;
        var_dlt_qbd_dn7 = assign33310_e48445_d_n7;
        var_dlt_qbd_dn10 = assign33310_e48445_d_n10;
        var_dlt_qbd_dn11 = assign33310_e48445_d_n11;
        var_dlt_qbd_dn12 = assign33310_e48445_d_n12;
        var_dlt_qbd_dn17 = assign33310_e48445_d_n17;

        let (assign33320_e48457, assign33320_e48457_d_n0, assign33320_e48457_d_n2, assign33320_e48457_d_n6, assign33320_e48457_d_n7, assign33320_e48457_d_n10, assign33320_e48457_d_n11, assign33320_e48457_d_n12, assign33320_e48457_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1091 != 0.0)) {
        let assign33320_e48450: f64 = (-var_qbd_max);
        let assign33320_e48452: f64 = (-var_qbd);
        let assign33320_e48453: f64 = (assign33320_e48450 - assign33320_e48452);
        let assign33320_e48455: f64 = (assign33320_e48453 - var_dlt_qbd);
        (assign33320_e48455, (((-var_qbd_max_dn0) - (-var_qbd_dn0)) - var_dlt_qbd_dn0), (((-var_qbd_max_dn2) - (-var_qbd_dn2)) - var_dlt_qbd_dn2), (((-var_qbd_max_dn6) - (-var_qbd_dn6)) - var_dlt_qbd_dn6), (((-var_qbd_max_dn7) - (-var_qbd_dn7)) - var_dlt_qbd_dn7), (((-var_qbd_max_dn10) - (-var_qbd_dn10)) - var_dlt_qbd_dn10), (((-var_qbd_max_dn11) - (-var_qbd_dn11)) - var_dlt_qbd_dn11), (((-var_qbd_max_dn12) - (-var_qbd_dn12)) - var_dlt_qbd_dn12), (((-var_qbd_max_dn17) - (-var_qbd_dn17)) - var_dlt_qbd_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign33320_e48457;
        var_tmf1_dn0 = assign33320_e48457_d_n0;
        var_tmf1_dn2 = assign33320_e48457_d_n2;
        var_tmf1_dn6 = assign33320_e48457_d_n6;
        var_tmf1_dn7 = assign33320_e48457_d_n7;
        var_tmf1_dn10 = assign33320_e48457_d_n10;
        var_tmf1_dn11 = assign33320_e48457_d_n11;
        var_tmf1_dn12 = assign33320_e48457_d_n12;
        var_tmf1_dn17 = assign33320_e48457_d_n17;

        let (assign33330_e48468, assign33330_e48468_d_n0, assign33330_e48468_d_n2, assign33330_e48468_d_n6, assign33330_e48468_d_n7, assign33330_e48468_d_n10, assign33330_e48468_d_n11, assign33330_e48468_d_n12, assign33330_e48468_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1091 != 0.0)) {
        let assign33330_e48463: f64 = (-var_qbd_max);
        let assign33330_e48464: f64 = (4.0 * assign33330_e48463);
        let assign33330_e48466: f64 = (assign33330_e48464 * var_dlt_qbd);
        (assign33330_e48466, (((4.0 * (-var_qbd_max_dn0)) * var_dlt_qbd) + (assign33330_e48464 * var_dlt_qbd_dn0)), (((4.0 * (-var_qbd_max_dn2)) * var_dlt_qbd) + (assign33330_e48464 * var_dlt_qbd_dn2)), (((4.0 * (-var_qbd_max_dn6)) * var_dlt_qbd) + (assign33330_e48464 * var_dlt_qbd_dn6)), (((4.0 * (-var_qbd_max_dn7)) * var_dlt_qbd) + (assign33330_e48464 * var_dlt_qbd_dn7)), (((4.0 * (-var_qbd_max_dn10)) * var_dlt_qbd) + (assign33330_e48464 * var_dlt_qbd_dn10)), (((4.0 * (-var_qbd_max_dn11)) * var_dlt_qbd) + (assign33330_e48464 * var_dlt_qbd_dn11)), (((4.0 * (-var_qbd_max_dn12)) * var_dlt_qbd) + (assign33330_e48464 * var_dlt_qbd_dn12)), (((4.0 * (-var_qbd_max_dn17)) * var_dlt_qbd) + (assign33330_e48464 * var_dlt_qbd_dn17)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign33330_e48468;
        var_tmf2_dn0 = assign33330_e48468_d_n0;
        var_tmf2_dn2 = assign33330_e48468_d_n2;
        var_tmf2_dn6 = assign33330_e48468_d_n6;
        var_tmf2_dn7 = assign33330_e48468_d_n7;
        var_tmf2_dn10 = assign33330_e48468_d_n10;
        var_tmf2_dn11 = assign33330_e48468_d_n11;
        var_tmf2_dn12 = assign33330_e48468_d_n12;
        var_tmf2_dn17 = assign33330_e48468_d_n17;

        let (assign33340_e48480, assign33340_e48480_d_n0, assign33340_e48480_d_n2, assign33340_e48480_d_n6, assign33340_e48480_d_n7, assign33340_e48480_d_n10, assign33340_e48480_d_n11, assign33340_e48480_d_n12, assign33340_e48480_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1091 != 0.0)) {
        let (assign33340_e48478, assign33340_e48478_d_n0, assign33340_e48478_d_n2, assign33340_e48478_d_n6, assign33340_e48478_d_n7, assign33340_e48478_d_n10, assign33340_e48478_d_n11, assign33340_e48478_d_n12, assign33340_e48478_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign33340_e48477: f64 = (-var_tmf2);
                (assign33340_e48477, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign33340_e48478, assign33340_e48478_d_n0, assign33340_e48478_d_n2, assign33340_e48478_d_n6, assign33340_e48478_d_n7, assign33340_e48478_d_n10, assign33340_e48478_d_n11, assign33340_e48478_d_n12, assign33340_e48478_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign33340_e48480;
        var_tmf2_dn0 = assign33340_e48480_d_n0;
        var_tmf2_dn2 = assign33340_e48480_d_n2;
        var_tmf2_dn6 = assign33340_e48480_d_n6;
        var_tmf2_dn7 = assign33340_e48480_d_n7;
        var_tmf2_dn10 = assign33340_e48480_d_n10;
        var_tmf2_dn11 = assign33340_e48480_d_n11;
        var_tmf2_dn12 = assign33340_e48480_d_n12;
        var_tmf2_dn17 = assign33340_e48480_d_n17;

        let (assign33350_e48491, assign33350_e48491_d_n0, assign33350_e48491_d_n2, assign33350_e48491_d_n6, assign33350_e48491_d_n7, assign33350_e48491_d_n10, assign33350_e48491_d_n11, assign33350_e48491_d_n12, assign33350_e48491_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1091 != 0.0)) {
        let assign33350_e48486: f64 = (var_tmf1 * var_tmf1);
        let assign33350_e48488: f64 = (assign33350_e48486 + var_tmf2);
        let assign33350_e48489: f64 = (assign33350_e48488).sqrt();
        (assign33350_e48489, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign33350_e48489)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign33350_e48489)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign33350_e48489)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign33350_e48489)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign33350_e48489)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign33350_e48489)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign33350_e48489)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign33350_e48489)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign33350_e48491;
        var_tmf2_dn0 = assign33350_e48491_d_n0;
        var_tmf2_dn2 = assign33350_e48491_d_n2;
        var_tmf2_dn6 = assign33350_e48491_d_n6;
        var_tmf2_dn7 = assign33350_e48491_d_n7;
        var_tmf2_dn10 = assign33350_e48491_d_n10;
        var_tmf2_dn11 = assign33350_e48491_d_n11;
        var_tmf2_dn12 = assign33350_e48491_d_n12;
        var_tmf2_dn17 = assign33350_e48491_d_n17;

        let (assign33360_e48504, assign33360_e48504_d_n0, assign33360_e48504_d_n2, assign33360_e48504_d_n6, assign33360_e48504_d_n7, assign33360_e48504_d_n10, assign33360_e48504_d_n11, assign33360_e48504_d_n12, assign33360_e48504_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1091 != 0.0)) {
        let assign33360_e48496: f64 = (-var_qbd_max);
        let assign33360_e48500: f64 = (var_tmf1 + var_tmf2);
        let assign33360_e48501: f64 = (0.5 * assign33360_e48500);
        let assign33360_e48502: f64 = (assign33360_e48496 - assign33360_e48501);
        (assign33360_e48502, ((-var_qbd_max_dn0) - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), ((-var_qbd_max_dn2) - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), ((-var_qbd_max_dn6) - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), ((-var_qbd_max_dn7) - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), ((-var_qbd_max_dn10) - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), ((-var_qbd_max_dn11) - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), ((-var_qbd_max_dn12) - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), ((-var_qbd_max_dn17) - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33360_e48504;
        var_qbd_dn0 = assign33360_e48504_d_n0;
        var_qbd_dn2 = assign33360_e48504_d_n2;
        var_qbd_dn6 = assign33360_e48504_d_n6;
        var_qbd_dn7 = assign33360_e48504_d_n7;
        var_qbd_dn10 = assign33360_e48504_d_n10;
        var_qbd_dn11 = assign33360_e48504_d_n11;
        var_qbd_dn12 = assign33360_e48504_d_n12;
        var_qbd_dn17 = assign33360_e48504_d_n17;

        let (assign33370_e48513, assign33370_e48513_d_n0, assign33370_e48513_d_n2, assign33370_e48513_d_n6, assign33370_e48513_d_n7, assign33370_e48513_d_n10, assign33370_e48513_d_n11, assign33370_e48513_d_n12, assign33370_e48513_d_n17,) = {
    if ((var_guard1032 != 0.0) && (var_guard1091 != 0.0)) {
        let assign33370_e48510: f64 = (-1.0);
        let assign33370_e48511: f64 = (var_qbd * assign33370_e48510);
        (assign33370_e48511, (var_qbd_dn0 * assign33370_e48510), (var_qbd_dn2 * assign33370_e48510), (var_qbd_dn6 * assign33370_e48510), (var_qbd_dn7 * assign33370_e48510), (var_qbd_dn10 * assign33370_e48510), (var_qbd_dn11 * assign33370_e48510), (var_qbd_dn12 * assign33370_e48510), (var_qbd_dn17 * assign33370_e48510),)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign33370_e48513;
        var_qbd_dn0 = assign33370_e48513_d_n0;
        var_qbd_dn2 = assign33370_e48513_d_n2;
        var_qbd_dn6 = assign33370_e48513_d_n6;
        var_qbd_dn7 = assign33370_e48513_d_n7;
        var_qbd_dn10 = assign33370_e48513_d_n10;
        var_qbd_dn11 = assign33370_e48513_d_n11;
        var_qbd_dn12 = assign33370_e48513_d_n12;
        var_qbd_dn17 = assign33370_e48513_d_n17;

        let assign33380_e48516: f64 = if var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        var_guard1097 = assign33380_e48516;

        let (assign33390_e48522,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1097 != 0.0)) {
        (p.p233,)
    } else {
        (var_t10__blk1092,)
    }
};
        var_t10__blk1092 = assign33390_e48522;

        let (assign33400_e48528,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1097 != 0.0)) {
        (p.p234,)
    } else {
        (var_t11__blk1093,)
    }
};
        var_t11__blk1093 = assign33400_e48528;

        let (assign33410_e48534, assign33410_e48534_d_n0, assign33410_e48534_d_n2, assign33410_e48534_d_n6, assign33410_e48534_d_n7, assign33410_e48534_d_n10, assign33410_e48534_d_n11, assign33410_e48534_d_n12, assign33410_e48534_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1097 != 0.0)) {
        (var_lch, var_lch_dn0, var_lch_dn2, var_lch_dn6, var_lch_dn7, var_lch_dn10, var_lch_dn11, var_lch_dn12, var_lch_dn17,)
    } else {
        (var_t12, var_t12_dn0, var_t12_dn2, var_t12_dn6, var_t12_dn7, var_t12_dn10, var_t12_dn11, var_t12_dn12, var_t12_dn17,)
    }
};
        var_t12 = assign33410_e48534;
        var_t12_dn0 = assign33410_e48534_d_n0;
        var_t12_dn2 = assign33410_e48534_d_n2;
        var_t12_dn6 = assign33410_e48534_d_n6;
        var_t12_dn7 = assign33410_e48534_d_n7;
        var_t12_dn10 = assign33410_e48534_d_n10;
        var_t12_dn11 = assign33410_e48534_d_n11;
        var_t12_dn12 = assign33410_e48534_d_n12;
        var_t12_dn17 = assign33410_e48534_d_n17;

        let (assign33420_e48546, assign33420_e48546_d_n0, assign33420_e48546_d_n2, assign33420_e48546_d_n6, assign33420_e48546_d_n7, assign33420_e48546_d_n10, assign33420_e48546_d_n11, assign33420_e48546_d_n12, assign33420_e48546_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1097 != 0.0)) {
        let assign33420_e48540: f64 = (var_t10__blk1092 * var_t11__blk1093);
        let assign33420_e48542: f64 = (assign33420_e48540 * var_t12);
        let assign33420_e48544: f64 = (assign33420_e48542 * var_t12);
        (assign33420_e48544, (((assign33420_e48540 * var_t12_dn0) * var_t12) + (assign33420_e48542 * var_t12_dn0)), (((assign33420_e48540 * var_t12_dn2) * var_t12) + (assign33420_e48542 * var_t12_dn2)), (((assign33420_e48540 * var_t12_dn6) * var_t12) + (assign33420_e48542 * var_t12_dn6)), (((assign33420_e48540 * var_t12_dn7) * var_t12) + (assign33420_e48542 * var_t12_dn7)), (((assign33420_e48540 * var_t12_dn10) * var_t12) + (assign33420_e48542 * var_t12_dn10)), (((assign33420_e48540 * var_t12_dn11) * var_t12) + (assign33420_e48542 * var_t12_dn11)), (((assign33420_e48540 * var_t12_dn12) * var_t12) + (assign33420_e48542 * var_t12_dn12)), (((assign33420_e48540 * var_t12_dn17) * var_t12) + (assign33420_e48542 * var_t12_dn17)),)
    } else {
        (var_t1__blk1095, var_t1__blk1095_dn0, var_t1__blk1095_dn2, var_t1__blk1095_dn6, var_t1__blk1095_dn7, var_t1__blk1095_dn10, var_t1__blk1095_dn11, var_t1__blk1095_dn12, var_t1__blk1095_dn17,)
    }
};
        var_t1__blk1095 = assign33420_e48546;
        var_t1__blk1095_dn0 = assign33420_e48546_d_n0;
        var_t1__blk1095_dn2 = assign33420_e48546_d_n2;
        var_t1__blk1095_dn6 = assign33420_e48546_d_n6;
        var_t1__blk1095_dn7 = assign33420_e48546_d_n7;
        var_t1__blk1095_dn10 = assign33420_e48546_d_n10;
        var_t1__blk1095_dn11 = assign33420_e48546_d_n11;
        var_t1__blk1095_dn12 = assign33420_e48546_d_n12;
        var_t1__blk1095_dn17 = assign33420_e48546_d_n17;

        let (assign33430_e48564, assign33430_e48564_d_n0, assign33430_e48564_d_n2, assign33430_e48564_d_n6, assign33430_e48564_d_n7, assign33430_e48564_d_n10, assign33430_e48564_d_n11, assign33430_e48564_d_n12, assign33430_e48564_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1097 != 0.0)) {
        let assign33430_e48552: f64 = (var_mu * var_vgvt);
        let assign33430_e48554: f64 = (assign33430_e48552 * var_t10__blk1092);
        let assign33430_e48557: f64 = (var_t11__blk1093 * var_t12);
        let assign33430_e48559: f64 = (assign33430_e48557 * var_t12);
        let assign33430_e48560: f64 = (assign33430_e48554 + assign33430_e48559);
        let assign33430_e48562: f64 = (assign33430_e48560 + 1e-50);
        (assign33430_e48562, ((((var_mu_dn0 * var_vgvt) + (var_mu * var_vgvt_dn0)) * var_t10__blk1092) + (((var_t11__blk1093 * var_t12_dn0) * var_t12) + (assign33430_e48557 * var_t12_dn0))), ((((var_mu_dn2 * var_vgvt) + (var_mu * var_vgvt_dn2)) * var_t10__blk1092) + (((var_t11__blk1093 * var_t12_dn2) * var_t12) + (assign33430_e48557 * var_t12_dn2))), ((((var_mu_dn6 * var_vgvt) + (var_mu * var_vgvt_dn6)) * var_t10__blk1092) + (((var_t11__blk1093 * var_t12_dn6) * var_t12) + (assign33430_e48557 * var_t12_dn6))), ((((var_mu_dn7 * var_vgvt) + (var_mu * var_vgvt_dn7)) * var_t10__blk1092) + (((var_t11__blk1093 * var_t12_dn7) * var_t12) + (assign33430_e48557 * var_t12_dn7))), ((((var_mu_dn10 * var_vgvt) + (var_mu * var_vgvt_dn10)) * var_t10__blk1092) + (((var_t11__blk1093 * var_t12_dn10) * var_t12) + (assign33430_e48557 * var_t12_dn10))), ((((var_mu_dn11 * var_vgvt) + (var_mu * var_vgvt_dn11)) * var_t10__blk1092) + (((var_t11__blk1093 * var_t12_dn11) * var_t12) + (assign33430_e48557 * var_t12_dn11))), ((((var_mu_dn12 * var_vgvt) + (var_mu * var_vgvt_dn12)) * var_t10__blk1092) + (((var_t11__blk1093 * var_t12_dn12) * var_t12) + (assign33430_e48557 * var_t12_dn12))), ((((var_mu_dn17 * var_vgvt) + (var_mu * var_vgvt_dn17)) * var_t10__blk1092) + (((var_t11__blk1093 * var_t12_dn17) * var_t12) + (assign33430_e48557 * var_t12_dn17))),)
    } else {
        (var_t2__blk1096, var_t2__blk1096_dn0, var_t2__blk1096_dn2, var_t2__blk1096_dn6, var_t2__blk1096_dn7, var_t2__blk1096_dn10, var_t2__blk1096_dn11, var_t2__blk1096_dn12, var_t2__blk1096_dn17,)
    }
};
        var_t2__blk1096 = assign33430_e48564;
        var_t2__blk1096_dn0 = assign33430_e48564_d_n0;
        var_t2__blk1096_dn2 = assign33430_e48564_d_n2;
        var_t2__blk1096_dn6 = assign33430_e48564_d_n6;
        var_t2__blk1096_dn7 = assign33430_e48564_d_n7;
        var_t2__blk1096_dn10 = assign33430_e48564_d_n10;
        var_t2__blk1096_dn11 = assign33430_e48564_d_n11;
        var_t2__blk1096_dn12 = assign33430_e48564_d_n12;
        var_t2__blk1096_dn17 = assign33430_e48564_d_n17;

        let (assign33440_e48572, assign33440_e48572_d_n0, assign33440_e48572_d_n2, assign33440_e48572_d_n6, assign33440_e48572_d_n7, assign33440_e48572_d_n10, assign33440_e48572_d_n11, assign33440_e48572_d_n12, assign33440_e48572_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1097 != 0.0)) {
        let assign33440_e48570: f64 = (var_t1__blk1095 / var_t2__blk1096);
        (assign33440_e48570, (((var_t1__blk1095_dn0 * var_t2__blk1096) - (var_t1__blk1095 * var_t2__blk1096_dn0)) / (var_t2__blk1096 * var_t2__blk1096)), (((var_t1__blk1095_dn2 * var_t2__blk1096) - (var_t1__blk1095 * var_t2__blk1096_dn2)) / (var_t2__blk1096 * var_t2__blk1096)), (((var_t1__blk1095_dn6 * var_t2__blk1096) - (var_t1__blk1095 * var_t2__blk1096_dn6)) / (var_t2__blk1096 * var_t2__blk1096)), (((var_t1__blk1095_dn7 * var_t2__blk1096) - (var_t1__blk1095 * var_t2__blk1096_dn7)) / (var_t2__blk1096 * var_t2__blk1096)), (((var_t1__blk1095_dn10 * var_t2__blk1096) - (var_t1__blk1095 * var_t2__blk1096_dn10)) / (var_t2__blk1096 * var_t2__blk1096)), (((var_t1__blk1095_dn11 * var_t2__blk1096) - (var_t1__blk1095 * var_t2__blk1096_dn11)) / (var_t2__blk1096 * var_t2__blk1096)), (((var_t1__blk1095_dn12 * var_t2__blk1096) - (var_t1__blk1095 * var_t2__blk1096_dn12)) / (var_t2__blk1096 * var_t2__blk1096)), (((var_t1__blk1095_dn17 * var_t2__blk1096) - (var_t1__blk1095 * var_t2__blk1096_dn17)) / (var_t2__blk1096 * var_t2__blk1096)),)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn6, var_tau_dn7, var_tau_dn10, var_tau_dn11, var_tau_dn12, var_tau_dn17,)
    }
};
        var_tau = assign33440_e48572;
        var_tau_dn0 = assign33440_e48572_d_n0;
        var_tau_dn2 = assign33440_e48572_d_n2;
        var_tau_dn6 = assign33440_e48572_d_n6;
        var_tau_dn7 = assign33440_e48572_d_n7;
        var_tau_dn10 = assign33440_e48572_d_n10;
        var_tau_dn11 = assign33440_e48572_d_n11;
        var_tau_dn12 = assign33440_e48572_d_n12;
        var_tau_dn17 = assign33440_e48572_d_n17;

        let (assign33450_e48581, assign33450_e48581_d_n0, assign33450_e48581_d_n2, assign33450_e48581_d_n6, assign33450_e48581_d_n7, assign33450_e48581_d_n10, assign33450_e48581_d_n11, assign33450_e48581_d_n12, assign33450_e48581_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1097 == 0.0)) {
        let assign33450_e48579: f64 = (p.p233 + 1e-50);
        (assign33450_e48579, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn6, var_tau_dn7, var_tau_dn10, var_tau_dn11, var_tau_dn12, var_tau_dn17,)
    }
};
        var_tau = assign33450_e48581;
        var_tau_dn0 = assign33450_e48581_d_n0;
        var_tau_dn2 = assign33450_e48581_d_n2;
        var_tau_dn6 = assign33450_e48581_d_n6;
        var_tau_dn7 = assign33450_e48581_d_n7;
        var_tau_dn10 = assign33450_e48581_d_n10;
        var_tau_dn11 = assign33450_e48581_d_n11;
        var_tau_dn12 = assign33450_e48581_d_n12;
        var_tau_dn17 = assign33450_e48581_d_n17;

        let (assign33460_e48585, assign33460_e48585_d_n0, assign33460_e48585_d_n2, assign33460_e48585_d_n6, assign33460_e48585_d_n7, assign33460_e48585_d_n10, assign33460_e48585_d_n11, assign33460_e48585_d_n12, assign33460_e48585_d_n17,) = {
    if (var_flg_nqs != 0.0) {
        (p.p235, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1__blk1095, var_t1__blk1095_dn0, var_t1__blk1095_dn2, var_t1__blk1095_dn6, var_t1__blk1095_dn7, var_t1__blk1095_dn10, var_t1__blk1095_dn11, var_t1__blk1095_dn12, var_t1__blk1095_dn17,)
    }
};
        var_t1__blk1095 = assign33460_e48585;
        var_t1__blk1095_dn0 = assign33460_e48585_d_n0;
        var_t1__blk1095_dn2 = assign33460_e48585_d_n2;
        var_t1__blk1095_dn6 = assign33460_e48585_d_n6;
        var_t1__blk1095_dn7 = assign33460_e48585_d_n7;
        var_t1__blk1095_dn10 = assign33460_e48585_d_n10;
        var_t1__blk1095_dn11 = assign33460_e48585_d_n11;
        var_t1__blk1095_dn12 = assign33460_e48585_d_n12;
        var_t1__blk1095_dn17 = assign33460_e48585_d_n17;

        let (assign33470_e48591, assign33470_e48591_d_n0, assign33470_e48591_d_n2, assign33470_e48591_d_n6, assign33470_e48591_d_n7, assign33470_e48591_d_n10, assign33470_e48591_d_n11, assign33470_e48591_d_n12, assign33470_e48591_d_n17,) = {
    if (var_flg_nqs != 0.0) {
        let assign33470_e48589: f64 = (var_t1__blk1095 * var_c_fox);
        (assign33470_e48589, ((var_t1__blk1095_dn0 * var_c_fox) + (var_t1__blk1095 * var_c_fox_dn0)), ((var_t1__blk1095_dn2 * var_c_fox) + (var_t1__blk1095 * var_c_fox_dn2)), ((var_t1__blk1095_dn6 * var_c_fox) + (var_t1__blk1095 * var_c_fox_dn6)), ((var_t1__blk1095_dn7 * var_c_fox) + (var_t1__blk1095 * var_c_fox_dn7)), ((var_t1__blk1095_dn10 * var_c_fox) + (var_t1__blk1095 * var_c_fox_dn10)), ((var_t1__blk1095_dn11 * var_c_fox) + (var_t1__blk1095 * var_c_fox_dn11)), ((var_t1__blk1095_dn12 * var_c_fox) + (var_t1__blk1095 * var_c_fox_dn12)), ((var_t1__blk1095_dn17 * var_c_fox) + (var_t1__blk1095 * var_c_fox_dn17)),)
    } else {
        (var_taub, var_taub_dn0, var_taub_dn2, var_taub_dn6, var_taub_dn7, var_taub_dn10, var_taub_dn11, var_taub_dn12, var_taub_dn17,)
    }
};
        var_taub = assign33470_e48591;
        var_taub_dn0 = assign33470_e48591_d_n0;
        var_taub_dn2 = assign33470_e48591_d_n2;
        var_taub_dn6 = assign33470_e48591_d_n6;
        var_taub_dn7 = assign33470_e48591_d_n7;
        var_taub_dn10 = assign33470_e48591_d_n10;
        var_taub_dn11 = assign33470_e48591_d_n11;
        var_taub_dn12 = assign33470_e48591_d_n12;
        var_taub_dn17 = assign33470_e48591_d_n17;

        let assign33600_e48767: f64 = if ((p.p32 != 0.0) && (var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        var_guard1124 = assign33600_e48767;

        let (assign33610_e48775, assign33610_e48775_d_n0, assign33610_e48775_d_n2, assign33610_e48775_d_n6, assign33610_e48775_d_n7, assign33610_e48775_d_n10, assign33610_e48775_d_n11, assign33610_e48775_d_n12, assign33610_e48775_d_n17,) = {
    if (var_guard1124 != 0.0) {
        let assign33610_e48771: f64 = (var_psdl - var_ps0);
        let assign33610_e48773: f64 = (assign33610_e48771 / var_lch);
        (assign33610_e48773, ((((var_psdl_dn0 - var_ps0_dn0) * var_lch) - (assign33610_e48771 * var_lch_dn0)) / (var_lch * var_lch)), ((((var_psdl_dn2 - var_ps0_dn2) * var_lch) - (assign33610_e48771 * var_lch_dn2)) / (var_lch * var_lch)), ((((var_psdl_dn6 - var_ps0_dn6) * var_lch) - (assign33610_e48771 * var_lch_dn6)) / (var_lch * var_lch)), ((((var_psdl_dn7 - var_ps0_dn7) * var_lch) - (assign33610_e48771 * var_lch_dn7)) / (var_lch * var_lch)), ((((var_psdl_dn10 - var_ps0_dn10) * var_lch) - (assign33610_e48771 * var_lch_dn10)) / (var_lch * var_lch)), ((((var_psdl_dn11 - var_ps0_dn11) * var_lch) - (assign33610_e48771 * var_lch_dn11)) / (var_lch * var_lch)), ((((var_psdl_dn12 - var_ps0_dn12) * var_lch) - (assign33610_e48771 * var_lch_dn12)) / (var_lch * var_lch)), ((((var_psdl_dn17 - var_ps0_dn17) * var_lch) - (assign33610_e48771 * var_lch_dn17)) / (var_lch * var_lch)),)
    } else {
        (var_eyd, var_eyd_dn0, var_eyd_dn2, var_eyd_dn6, var_eyd_dn7, var_eyd_dn10, var_eyd_dn11, var_eyd_dn12, var_eyd_dn17,)
    }
};
        var_eyd = assign33610_e48775;
        var_eyd_dn0 = assign33610_e48775_d_n0;
        var_eyd_dn2 = assign33610_e48775_d_n2;
        var_eyd_dn6 = assign33610_e48775_d_n6;
        var_eyd_dn7 = assign33610_e48775_d_n7;
        var_eyd_dn10 = assign33610_e48775_d_n10;
        var_eyd_dn11 = assign33610_e48775_d_n11;
        var_eyd_dn12 = assign33610_e48775_d_n12;
        var_eyd_dn17 = assign33610_e48775_d_n17;

        let (assign33620_e48783, assign33620_e48783_d_n0, assign33620_e48783_d_n2, assign33620_e48783_d_n6, assign33620_e48783_d_n7, assign33620_e48783_d_n10, assign33620_e48783_d_n11, assign33620_e48783_d_n12, assign33620_e48783_d_n17,) = {
    if (var_guard1124 != 0.0) {
        let assign33620_e48779: f64 = (var_muun * var_eyd);
        let assign33620_e48781: f64 = (assign33620_e48779 / 100000.0);
        (assign33620_e48781, (((var_muun_dn0 * var_eyd) + (var_muun * var_eyd_dn0)) / 100000.0), (((var_muun_dn2 * var_eyd) + (var_muun * var_eyd_dn2)) / 100000.0), (((var_muun_dn6 * var_eyd) + (var_muun * var_eyd_dn6)) / 100000.0), (((var_muun_dn7 * var_eyd) + (var_muun * var_eyd_dn7)) / 100000.0), (((var_muun_dn10 * var_eyd) + (var_muun * var_eyd_dn10)) / 100000.0), (((var_muun_dn11 * var_eyd) + (var_muun * var_eyd_dn11)) / 100000.0), (((var_muun_dn12 * var_eyd) + (var_muun * var_eyd_dn12)) / 100000.0), (((var_muun_dn17 * var_eyd) + (var_muun * var_eyd_dn17)) / 100000.0),)
    } else {
        (var_t12__blk1108, var_t12__blk1108_dn0, var_t12__blk1108_dn2, var_t12__blk1108_dn6, var_t12__blk1108_dn7, var_t12__blk1108_dn10, var_t12__blk1108_dn11, var_t12__blk1108_dn12, var_t12__blk1108_dn17,)
    }
};
        var_t12__blk1108 = assign33620_e48783;
        var_t12__blk1108_dn0 = assign33620_e48783_d_n0;
        var_t12__blk1108_dn2 = assign33620_e48783_d_n2;
        var_t12__blk1108_dn6 = assign33620_e48783_d_n6;
        var_t12__blk1108_dn7 = assign33620_e48783_d_n7;
        var_t12__blk1108_dn10 = assign33620_e48783_d_n10;
        var_t12__blk1108_dn11 = assign33620_e48783_d_n11;
        var_t12__blk1108_dn12 = assign33620_e48783_d_n12;
        var_t12__blk1108_dn17 = assign33620_e48783_d_n17;

        let assign33630_e48787: f64 = (10.0 * 2.220446049250313e-16);
        let assign33630_e48788: f64 = (1.0 - assign33630_e48787);
        let assign33630_e48795: f64 = (10.0 * 2.220446049250313e-16);
        let assign33630_e48796: f64 = (1.0 + assign33630_e48795);
        let assign33630_e48798: f64 = if ((assign33630_e48788 <= p.p113) && (p.p113 <= assign33630_e48796)) { 1.0 } else { 0.0 };
        var_guard1125 = assign33630_e48798;

        let (assign33640_e48804, assign33640_e48804_d_n0, assign33640_e48804_d_n2, assign33640_e48804_d_n6, assign33640_e48804_d_n7, assign33640_e48804_d_n10, assign33640_e48804_d_n11, assign33640_e48804_d_n12, assign33640_e48804_d_n17,) = {
    if ((var_guard1124 != 0.0) && (var_guard1125 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7__blk1109, var_t7__blk1109_dn0, var_t7__blk1109_dn2, var_t7__blk1109_dn6, var_t7__blk1109_dn7, var_t7__blk1109_dn10, var_t7__blk1109_dn11, var_t7__blk1109_dn12, var_t7__blk1109_dn17,)
    }
};
        var_t7__blk1109 = assign33640_e48804;
        var_t7__blk1109_dn0 = assign33640_e48804_d_n0;
        var_t7__blk1109_dn2 = assign33640_e48804_d_n2;
        var_t7__blk1109_dn6 = assign33640_e48804_d_n6;
        var_t7__blk1109_dn7 = assign33640_e48804_d_n7;
        var_t7__blk1109_dn10 = assign33640_e48804_d_n10;
        var_t7__blk1109_dn11 = assign33640_e48804_d_n11;
        var_t7__blk1109_dn12 = assign33640_e48804_d_n12;
        var_t7__blk1109_dn17 = assign33640_e48804_d_n17;

        let assign33650_e48808: f64 = (10.0 * 2.220446049250313e-16);
        let assign33650_e48809: f64 = (2.0 - assign33650_e48808);
        let assign33650_e48816: f64 = (10.0 * 2.220446049250313e-16);
        let assign33650_e48817: f64 = (2.0 + assign33650_e48816);
        let assign33650_e48819: f64 = if ((assign33650_e48809 <= p.p113) && (p.p113 <= assign33650_e48817)) { 1.0 } else { 0.0 };
        var_guard1126 = assign33650_e48819;

        let (assign33660_e48828, assign33660_e48828_d_n0, assign33660_e48828_d_n2, assign33660_e48828_d_n6, assign33660_e48828_d_n7, assign33660_e48828_d_n10, assign33660_e48828_d_n11, assign33660_e48828_d_n12, assign33660_e48828_d_n17,) = {
    if (((var_guard1124 != 0.0) && (var_guard1125 == 0.0)) && (var_guard1126 != 0.0)) {
        (var_t12__blk1108, var_t12__blk1108_dn0, var_t12__blk1108_dn2, var_t12__blk1108_dn6, var_t12__blk1108_dn7, var_t12__blk1108_dn10, var_t12__blk1108_dn11, var_t12__blk1108_dn12, var_t12__blk1108_dn17,)
    } else {
        (var_t7__blk1109, var_t7__blk1109_dn0, var_t7__blk1109_dn2, var_t7__blk1109_dn6, var_t7__blk1109_dn7, var_t7__blk1109_dn10, var_t7__blk1109_dn11, var_t7__blk1109_dn12, var_t7__blk1109_dn17,)
    }
};
        var_t7__blk1109 = assign33660_e48828;
        var_t7__blk1109_dn0 = assign33660_e48828_d_n0;
        var_t7__blk1109_dn2 = assign33660_e48828_d_n2;
        var_t7__blk1109_dn6 = assign33660_e48828_d_n6;
        var_t7__blk1109_dn7 = assign33660_e48828_d_n7;
        var_t7__blk1109_dn10 = assign33660_e48828_d_n10;
        var_t7__blk1109_dn11 = assign33660_e48828_d_n11;
        var_t7__blk1109_dn12 = assign33660_e48828_d_n12;
        var_t7__blk1109_dn17 = assign33660_e48828_d_n17;

        let (assign33670_e48842, assign33670_e48842_d_n0, assign33670_e48842_d_n2, assign33670_e48842_d_n6, assign33670_e48842_d_n7, assign33670_e48842_d_n10, assign33670_e48842_d_n11, assign33670_e48842_d_n12, assign33670_e48842_d_n17,) = {
    if (((var_guard1124 != 0.0) && (var_guard1125 == 0.0)) && (var_guard1126 == 0.0)) {
        let assign33670_e48839: f64 = (p.p113 - 1.0);
        let assign33670_e48840: f64 = (var_t12__blk1108).powf(assign33670_e48839);
        (assign33670_e48840, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((var_t12__blk1108).powf(assign33670_e48839 - 1.0) * var_t12__blk1108_dn0)) } } else { (assign33670_e48840 * (assign33670_e48839 * (var_t12__blk1108_dn0 / var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((var_t12__blk1108).powf(assign33670_e48839 - 1.0) * var_t12__blk1108_dn2)) } } else { (assign33670_e48840 * (assign33670_e48839 * (var_t12__blk1108_dn2 / var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((var_t12__blk1108).powf(assign33670_e48839 - 1.0) * var_t12__blk1108_dn6)) } } else { (assign33670_e48840 * (assign33670_e48839 * (var_t12__blk1108_dn6 / var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((var_t12__blk1108).powf(assign33670_e48839 - 1.0) * var_t12__blk1108_dn7)) } } else { (assign33670_e48840 * (assign33670_e48839 * (var_t12__blk1108_dn7 / var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((var_t12__blk1108).powf(assign33670_e48839 - 1.0) * var_t12__blk1108_dn10)) } } else { (assign33670_e48840 * (assign33670_e48839 * (var_t12__blk1108_dn10 / var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((var_t12__blk1108).powf(assign33670_e48839 - 1.0) * var_t12__blk1108_dn11)) } } else { (assign33670_e48840 * (assign33670_e48839 * (var_t12__blk1108_dn11 / var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((var_t12__blk1108).powf(assign33670_e48839 - 1.0) * var_t12__blk1108_dn12)) } } else { (assign33670_e48840 * (assign33670_e48839 * (var_t12__blk1108_dn12 / var_t12__blk1108))) }, if 0.0 == 0.0 && ((assign33670_e48839) as f64).is_finite() && ((assign33670_e48839) as f64).fract() == 0.0 { if assign33670_e48839 == 0.0 { 0.0 } else { (assign33670_e48839 * ((var_t12__blk1108).powf(assign33670_e48839 - 1.0) * var_t12__blk1108_dn17)) } } else { (assign33670_e48840 * (assign33670_e48839 * (var_t12__blk1108_dn17 / var_t12__blk1108))) },)
    } else {
        (var_t7__blk1109, var_t7__blk1109_dn0, var_t7__blk1109_dn2, var_t7__blk1109_dn6, var_t7__blk1109_dn7, var_t7__blk1109_dn10, var_t7__blk1109_dn11, var_t7__blk1109_dn12, var_t7__blk1109_dn17,)
    }
};
        var_t7__blk1109 = assign33670_e48842;
        var_t7__blk1109_dn0 = assign33670_e48842_d_n0;
        var_t7__blk1109_dn2 = assign33670_e48842_d_n2;
        var_t7__blk1109_dn6 = assign33670_e48842_d_n6;
        var_t7__blk1109_dn7 = assign33670_e48842_d_n7;
        var_t7__blk1109_dn10 = assign33670_e48842_d_n10;
        var_t7__blk1109_dn11 = assign33670_e48842_d_n11;
        var_t7__blk1109_dn12 = assign33670_e48842_d_n12;
        var_t7__blk1109_dn17 = assign33670_e48842_d_n17;

        let (assign33680_e48848, assign33680_e48848_d_n0, assign33680_e48848_d_n2, assign33680_e48848_d_n6, assign33680_e48848_d_n7, assign33680_e48848_d_n10, assign33680_e48848_d_n11, assign33680_e48848_d_n12, assign33680_e48848_d_n17,) = {
    if (var_guard1124 != 0.0) {
        let assign33680_e48846: f64 = (var_t12__blk1108 * var_t7__blk1109);
        (assign33680_e48846, ((var_t12__blk1108_dn0 * var_t7__blk1109) + (var_t12__blk1108 * var_t7__blk1109_dn0)), ((var_t12__blk1108_dn2 * var_t7__blk1109) + (var_t12__blk1108 * var_t7__blk1109_dn2)), ((var_t12__blk1108_dn6 * var_t7__blk1109) + (var_t12__blk1108 * var_t7__blk1109_dn6)), ((var_t12__blk1108_dn7 * var_t7__blk1109) + (var_t12__blk1108 * var_t7__blk1109_dn7)), ((var_t12__blk1108_dn10 * var_t7__blk1109) + (var_t12__blk1108 * var_t7__blk1109_dn10)), ((var_t12__blk1108_dn11 * var_t7__blk1109) + (var_t12__blk1108 * var_t7__blk1109_dn11)), ((var_t12__blk1108_dn12 * var_t7__blk1109) + (var_t12__blk1108 * var_t7__blk1109_dn12)), ((var_t12__blk1108_dn17 * var_t7__blk1109) + (var_t12__blk1108 * var_t7__blk1109_dn17)),)
    } else {
        (var_t8__blk1110, var_t8__blk1110_dn0, var_t8__blk1110_dn2, var_t8__blk1110_dn6, var_t8__blk1110_dn7, var_t8__blk1110_dn10, var_t8__blk1110_dn11, var_t8__blk1110_dn12, var_t8__blk1110_dn17,)
    }
};
        var_t8__blk1110 = assign33680_e48848;
        var_t8__blk1110_dn0 = assign33680_e48848_d_n0;
        var_t8__blk1110_dn2 = assign33680_e48848_d_n2;
        var_t8__blk1110_dn6 = assign33680_e48848_d_n6;
        var_t8__blk1110_dn7 = assign33680_e48848_d_n7;
        var_t8__blk1110_dn10 = assign33680_e48848_d_n10;
        var_t8__blk1110_dn11 = assign33680_e48848_d_n11;
        var_t8__blk1110_dn12 = assign33680_e48848_d_n12;
        var_t8__blk1110_dn17 = assign33680_e48848_d_n17;

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
        *var_guard1091_slot = var_guard1091;
        *var_guard1097_slot = var_guard1097;
        *var_guard1124_slot = var_guard1124;
        *var_guard1125_slot = var_guard1125;
        *var_guard1126_slot = var_guard1126;
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
        *var_t10__blk1092_slot = var_t10__blk1092;
        *var_t11__blk1093_slot = var_t11__blk1093;
        *var_t12_slot = var_t12;
        *var_t12__blk1108_slot = var_t12__blk1108;
        *var_t12__blk1108_dn0_slot = var_t12__blk1108_dn0;
        *var_t12__blk1108_dn10_slot = var_t12__blk1108_dn10;
        *var_t12__blk1108_dn11_slot = var_t12__blk1108_dn11;
        *var_t12__blk1108_dn12_slot = var_t12__blk1108_dn12;
        *var_t12__blk1108_dn17_slot = var_t12__blk1108_dn17;
        *var_t12__blk1108_dn2_slot = var_t12__blk1108_dn2;
        *var_t12__blk1108_dn6_slot = var_t12__blk1108_dn6;
        *var_t12__blk1108_dn7_slot = var_t12__blk1108_dn7;
        *var_t12_dn0_slot = var_t12_dn0;
        *var_t12_dn10_slot = var_t12_dn10;
        *var_t12_dn11_slot = var_t12_dn11;
        *var_t12_dn12_slot = var_t12_dn12;
        *var_t12_dn17_slot = var_t12_dn17;
        *var_t12_dn2_slot = var_t12_dn2;
        *var_t12_dn6_slot = var_t12_dn6;
        *var_t12_dn7_slot = var_t12_dn7;
        *var_t1__blk1095_slot = var_t1__blk1095;
        *var_t1__blk1095_dn0_slot = var_t1__blk1095_dn0;
        *var_t1__blk1095_dn10_slot = var_t1__blk1095_dn10;
        *var_t1__blk1095_dn11_slot = var_t1__blk1095_dn11;
        *var_t1__blk1095_dn12_slot = var_t1__blk1095_dn12;
        *var_t1__blk1095_dn17_slot = var_t1__blk1095_dn17;
        *var_t1__blk1095_dn2_slot = var_t1__blk1095_dn2;
        *var_t1__blk1095_dn6_slot = var_t1__blk1095_dn6;
        *var_t1__blk1095_dn7_slot = var_t1__blk1095_dn7;
        *var_t2__blk1096_slot = var_t2__blk1096;
        *var_t2__blk1096_dn0_slot = var_t2__blk1096_dn0;
        *var_t2__blk1096_dn10_slot = var_t2__blk1096_dn10;
        *var_t2__blk1096_dn11_slot = var_t2__blk1096_dn11;
        *var_t2__blk1096_dn12_slot = var_t2__blk1096_dn12;
        *var_t2__blk1096_dn17_slot = var_t2__blk1096_dn17;
        *var_t2__blk1096_dn2_slot = var_t2__blk1096_dn2;
        *var_t2__blk1096_dn6_slot = var_t2__blk1096_dn6;
        *var_t2__blk1096_dn7_slot = var_t2__blk1096_dn7;
        *var_t7__blk1109_slot = var_t7__blk1109;
        *var_t7__blk1109_dn0_slot = var_t7__blk1109_dn0;
        *var_t7__blk1109_dn10_slot = var_t7__blk1109_dn10;
        *var_t7__blk1109_dn11_slot = var_t7__blk1109_dn11;
        *var_t7__blk1109_dn12_slot = var_t7__blk1109_dn12;
        *var_t7__blk1109_dn17_slot = var_t7__blk1109_dn17;
        *var_t7__blk1109_dn2_slot = var_t7__blk1109_dn2;
        *var_t7__blk1109_dn6_slot = var_t7__blk1109_dn6;
        *var_t7__blk1109_dn7_slot = var_t7__blk1109_dn7;
        *var_t8__blk1110_slot = var_t8__blk1110;
        *var_t8__blk1110_dn0_slot = var_t8__blk1110_dn0;
        *var_t8__blk1110_dn10_slot = var_t8__blk1110_dn10;
        *var_t8__blk1110_dn11_slot = var_t8__blk1110_dn11;
        *var_t8__blk1110_dn12_slot = var_t8__blk1110_dn12;
        *var_t8__blk1110_dn17_slot = var_t8__blk1110_dn17;
        *var_t8__blk1110_dn2_slot = var_t8__blk1110_dn2;
        *var_t8__blk1110_dn6_slot = var_t8__blk1110_dn6;
        *var_t8__blk1110_dn7_slot = var_t8__blk1110_dn7;
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
        var_guard1124: f64,
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
        var_t8__blk1110: f64,
        var_t8__blk1110_dn0: f64,
        var_t8__blk1110_dn10: f64,
        var_t8__blk1110_dn11: f64,
        var_t8__blk1110_dn12: f64,
        var_t8__blk1110_dn17: f64,
        var_t8__blk1110_dn2: f64,
        var_t8__blk1110_dn6: f64,
        var_t8__blk1110_dn7: f64,
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
        var_guard1127_slot: &mut f64,
        var_guard1128_slot: &mut f64,
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
        var_t0__blk1114_slot: &mut f64,
        var_t0__blk1114_dn0_slot: &mut f64,
        var_t0__blk1114_dn10_slot: &mut f64,
        var_t0__blk1114_dn11_slot: &mut f64,
        var_t0__blk1114_dn12_slot: &mut f64,
        var_t0__blk1114_dn17_slot: &mut f64,
        var_t0__blk1114_dn2_slot: &mut f64,
        var_t0__blk1114_dn6_slot: &mut f64,
        var_t0__blk1114_dn7_slot: &mut f64,
        var_t10__blk1112_slot: &mut f64,
        var_t10__blk1112_dn0_slot: &mut f64,
        var_t10__blk1112_dn10_slot: &mut f64,
        var_t10__blk1112_dn11_slot: &mut f64,
        var_t10__blk1112_dn12_slot: &mut f64,
        var_t10__blk1112_dn17_slot: &mut f64,
        var_t10__blk1112_dn2_slot: &mut f64,
        var_t10__blk1112_dn6_slot: &mut f64,
        var_t10__blk1112_dn7_slot: &mut f64,
        var_t10w_slot: &mut f64,
        var_t10w_dn0_slot: &mut f64,
        var_t10w_dn10_slot: &mut f64,
        var_t10w_dn11_slot: &mut f64,
        var_t10w_dn12_slot: &mut f64,
        var_t10w_dn17_slot: &mut f64,
        var_t10w_dn2_slot: &mut f64,
        var_t10w_dn6_slot: &mut f64,
        var_t10w_dn7_slot: &mut f64,
        var_t11__blk1113_slot: &mut f64,
        var_t11__blk1113_dn0_slot: &mut f64,
        var_t11__blk1113_dn10_slot: &mut f64,
        var_t11__blk1113_dn11_slot: &mut f64,
        var_t11__blk1113_dn12_slot: &mut f64,
        var_t11__blk1113_dn17_slot: &mut f64,
        var_t11__blk1113_dn2_slot: &mut f64,
        var_t11__blk1113_dn6_slot: &mut f64,
        var_t11__blk1113_dn7_slot: &mut f64,
        var_t2__blk1116_slot: &mut f64,
        var_t2__blk1116_dn0_slot: &mut f64,
        var_t2__blk1116_dn10_slot: &mut f64,
        var_t2__blk1116_dn11_slot: &mut f64,
        var_t2__blk1116_dn12_slot: &mut f64,
        var_t2__blk1116_dn17_slot: &mut f64,
        var_t2__blk1116_dn2_slot: &mut f64,
        var_t2__blk1116_dn6_slot: &mut f64,
        var_t2__blk1116_dn7_slot: &mut f64,
        var_t3__blk1117_slot: &mut f64,
        var_t3__blk1117_dn0_slot: &mut f64,
        var_t3__blk1117_dn10_slot: &mut f64,
        var_t3__blk1117_dn11_slot: &mut f64,
        var_t3__blk1117_dn12_slot: &mut f64,
        var_t3__blk1117_dn17_slot: &mut f64,
        var_t3__blk1117_dn2_slot: &mut f64,
        var_t3__blk1117_dn6_slot: &mut f64,
        var_t3__blk1117_dn7_slot: &mut f64,
        var_t4__blk1118_slot: &mut f64,
        var_t4__blk1118_dn0_slot: &mut f64,
        var_t4__blk1118_dn10_slot: &mut f64,
        var_t4__blk1118_dn11_slot: &mut f64,
        var_t4__blk1118_dn12_slot: &mut f64,
        var_t4__blk1118_dn17_slot: &mut f64,
        var_t4__blk1118_dn2_slot: &mut f64,
        var_t4__blk1118_dn6_slot: &mut f64,
        var_t4__blk1118_dn7_slot: &mut f64,
        var_t5__blk1119_slot: &mut f64,
        var_t5__blk1119_dn0_slot: &mut f64,
        var_t5__blk1119_dn10_slot: &mut f64,
        var_t5__blk1119_dn11_slot: &mut f64,
        var_t5__blk1119_dn12_slot: &mut f64,
        var_t5__blk1119_dn17_slot: &mut f64,
        var_t5__blk1119_dn2_slot: &mut f64,
        var_t5__blk1119_dn6_slot: &mut f64,
        var_t5__blk1119_dn7_slot: &mut f64,
        var_t7w_slot: &mut f64,
        var_t7w_dn0_slot: &mut f64,
        var_t7w_dn10_slot: &mut f64,
        var_t7w_dn11_slot: &mut f64,
        var_t7w_dn12_slot: &mut f64,
        var_t7w_dn17_slot: &mut f64,
        var_t7w_dn2_slot: &mut f64,
        var_t7w_dn6_slot: &mut f64,
        var_t7w_dn7_slot: &mut f64,
        var_t9__blk1111_slot: &mut f64,
        var_t9__blk1111_dn0_slot: &mut f64,
        var_t9__blk1111_dn10_slot: &mut f64,
        var_t9__blk1111_dn11_slot: &mut f64,
        var_t9__blk1111_dn12_slot: &mut f64,
        var_t9__blk1111_dn17_slot: &mut f64,
        var_t9__blk1111_dn2_slot: &mut f64,
        var_t9__blk1111_dn6_slot: &mut f64,
        var_t9__blk1111_dn7_slot: &mut f64,
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
        let mut var_guard1127: f64 = *var_guard1127_slot;
        let mut var_guard1128: f64 = *var_guard1128_slot;
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
        let mut var_t0__blk1114: f64 = *var_t0__blk1114_slot;
        let mut var_t0__blk1114_dn0: f64 = *var_t0__blk1114_dn0_slot;
        let mut var_t0__blk1114_dn10: f64 = *var_t0__blk1114_dn10_slot;
        let mut var_t0__blk1114_dn11: f64 = *var_t0__blk1114_dn11_slot;
        let mut var_t0__blk1114_dn12: f64 = *var_t0__blk1114_dn12_slot;
        let mut var_t0__blk1114_dn17: f64 = *var_t0__blk1114_dn17_slot;
        let mut var_t0__blk1114_dn2: f64 = *var_t0__blk1114_dn2_slot;
        let mut var_t0__blk1114_dn6: f64 = *var_t0__blk1114_dn6_slot;
        let mut var_t0__blk1114_dn7: f64 = *var_t0__blk1114_dn7_slot;
        let mut var_t10__blk1112: f64 = *var_t10__blk1112_slot;
        let mut var_t10__blk1112_dn0: f64 = *var_t10__blk1112_dn0_slot;
        let mut var_t10__blk1112_dn10: f64 = *var_t10__blk1112_dn10_slot;
        let mut var_t10__blk1112_dn11: f64 = *var_t10__blk1112_dn11_slot;
        let mut var_t10__blk1112_dn12: f64 = *var_t10__blk1112_dn12_slot;
        let mut var_t10__blk1112_dn17: f64 = *var_t10__blk1112_dn17_slot;
        let mut var_t10__blk1112_dn2: f64 = *var_t10__blk1112_dn2_slot;
        let mut var_t10__blk1112_dn6: f64 = *var_t10__blk1112_dn6_slot;
        let mut var_t10__blk1112_dn7: f64 = *var_t10__blk1112_dn7_slot;
        let mut var_t10w: f64 = *var_t10w_slot;
        let mut var_t10w_dn0: f64 = *var_t10w_dn0_slot;
        let mut var_t10w_dn10: f64 = *var_t10w_dn10_slot;
        let mut var_t10w_dn11: f64 = *var_t10w_dn11_slot;
        let mut var_t10w_dn12: f64 = *var_t10w_dn12_slot;
        let mut var_t10w_dn17: f64 = *var_t10w_dn17_slot;
        let mut var_t10w_dn2: f64 = *var_t10w_dn2_slot;
        let mut var_t10w_dn6: f64 = *var_t10w_dn6_slot;
        let mut var_t10w_dn7: f64 = *var_t10w_dn7_slot;
        let mut var_t11__blk1113: f64 = *var_t11__blk1113_slot;
        let mut var_t11__blk1113_dn0: f64 = *var_t11__blk1113_dn0_slot;
        let mut var_t11__blk1113_dn10: f64 = *var_t11__blk1113_dn10_slot;
        let mut var_t11__blk1113_dn11: f64 = *var_t11__blk1113_dn11_slot;
        let mut var_t11__blk1113_dn12: f64 = *var_t11__blk1113_dn12_slot;
        let mut var_t11__blk1113_dn17: f64 = *var_t11__blk1113_dn17_slot;
        let mut var_t11__blk1113_dn2: f64 = *var_t11__blk1113_dn2_slot;
        let mut var_t11__blk1113_dn6: f64 = *var_t11__blk1113_dn6_slot;
        let mut var_t11__blk1113_dn7: f64 = *var_t11__blk1113_dn7_slot;
        let mut var_t2__blk1116: f64 = *var_t2__blk1116_slot;
        let mut var_t2__blk1116_dn0: f64 = *var_t2__blk1116_dn0_slot;
        let mut var_t2__blk1116_dn10: f64 = *var_t2__blk1116_dn10_slot;
        let mut var_t2__blk1116_dn11: f64 = *var_t2__blk1116_dn11_slot;
        let mut var_t2__blk1116_dn12: f64 = *var_t2__blk1116_dn12_slot;
        let mut var_t2__blk1116_dn17: f64 = *var_t2__blk1116_dn17_slot;
        let mut var_t2__blk1116_dn2: f64 = *var_t2__blk1116_dn2_slot;
        let mut var_t2__blk1116_dn6: f64 = *var_t2__blk1116_dn6_slot;
        let mut var_t2__blk1116_dn7: f64 = *var_t2__blk1116_dn7_slot;
        let mut var_t3__blk1117: f64 = *var_t3__blk1117_slot;
        let mut var_t3__blk1117_dn0: f64 = *var_t3__blk1117_dn0_slot;
        let mut var_t3__blk1117_dn10: f64 = *var_t3__blk1117_dn10_slot;
        let mut var_t3__blk1117_dn11: f64 = *var_t3__blk1117_dn11_slot;
        let mut var_t3__blk1117_dn12: f64 = *var_t3__blk1117_dn12_slot;
        let mut var_t3__blk1117_dn17: f64 = *var_t3__blk1117_dn17_slot;
        let mut var_t3__blk1117_dn2: f64 = *var_t3__blk1117_dn2_slot;
        let mut var_t3__blk1117_dn6: f64 = *var_t3__blk1117_dn6_slot;
        let mut var_t3__blk1117_dn7: f64 = *var_t3__blk1117_dn7_slot;
        let mut var_t4__blk1118: f64 = *var_t4__blk1118_slot;
        let mut var_t4__blk1118_dn0: f64 = *var_t4__blk1118_dn0_slot;
        let mut var_t4__blk1118_dn10: f64 = *var_t4__blk1118_dn10_slot;
        let mut var_t4__blk1118_dn11: f64 = *var_t4__blk1118_dn11_slot;
        let mut var_t4__blk1118_dn12: f64 = *var_t4__blk1118_dn12_slot;
        let mut var_t4__blk1118_dn17: f64 = *var_t4__blk1118_dn17_slot;
        let mut var_t4__blk1118_dn2: f64 = *var_t4__blk1118_dn2_slot;
        let mut var_t4__blk1118_dn6: f64 = *var_t4__blk1118_dn6_slot;
        let mut var_t4__blk1118_dn7: f64 = *var_t4__blk1118_dn7_slot;
        let mut var_t5__blk1119: f64 = *var_t5__blk1119_slot;
        let mut var_t5__blk1119_dn0: f64 = *var_t5__blk1119_dn0_slot;
        let mut var_t5__blk1119_dn10: f64 = *var_t5__blk1119_dn10_slot;
        let mut var_t5__blk1119_dn11: f64 = *var_t5__blk1119_dn11_slot;
        let mut var_t5__blk1119_dn12: f64 = *var_t5__blk1119_dn12_slot;
        let mut var_t5__blk1119_dn17: f64 = *var_t5__blk1119_dn17_slot;
        let mut var_t5__blk1119_dn2: f64 = *var_t5__blk1119_dn2_slot;
        let mut var_t5__blk1119_dn6: f64 = *var_t5__blk1119_dn6_slot;
        let mut var_t5__blk1119_dn7: f64 = *var_t5__blk1119_dn7_slot;
        let mut var_t7w: f64 = *var_t7w_slot;
        let mut var_t7w_dn0: f64 = *var_t7w_dn0_slot;
        let mut var_t7w_dn10: f64 = *var_t7w_dn10_slot;
        let mut var_t7w_dn11: f64 = *var_t7w_dn11_slot;
        let mut var_t7w_dn12: f64 = *var_t7w_dn12_slot;
        let mut var_t7w_dn17: f64 = *var_t7w_dn17_slot;
        let mut var_t7w_dn2: f64 = *var_t7w_dn2_slot;
        let mut var_t7w_dn6: f64 = *var_t7w_dn6_slot;
        let mut var_t7w_dn7: f64 = *var_t7w_dn7_slot;
        let mut var_t9__blk1111: f64 = *var_t9__blk1111_slot;
        let mut var_t9__blk1111_dn0: f64 = *var_t9__blk1111_dn0_slot;
        let mut var_t9__blk1111_dn10: f64 = *var_t9__blk1111_dn10_slot;
        let mut var_t9__blk1111_dn11: f64 = *var_t9__blk1111_dn11_slot;
        let mut var_t9__blk1111_dn12: f64 = *var_t9__blk1111_dn12_slot;
        let mut var_t9__blk1111_dn17: f64 = *var_t9__blk1111_dn17_slot;
        let mut var_t9__blk1111_dn2: f64 = *var_t9__blk1111_dn2_slot;
        let mut var_t9__blk1111_dn6: f64 = *var_t9__blk1111_dn6_slot;
        let mut var_t9__blk1111_dn7: f64 = *var_t9__blk1111_dn7_slot;

        let (assign33690_e48854, assign33690_e48854_d_n0, assign33690_e48854_d_n2, assign33690_e48854_d_n6, assign33690_e48854_d_n7, assign33690_e48854_d_n10, assign33690_e48854_d_n11, assign33690_e48854_d_n12, assign33690_e48854_d_n17,) = {
    if (var_guard1124 != 0.0) {
        let assign33690_e48852: f64 = (1.0 + var_t8__blk1110);
        (assign33690_e48852, var_t8__blk1110_dn0, var_t8__blk1110_dn2, var_t8__blk1110_dn6, var_t8__blk1110_dn7, var_t8__blk1110_dn10, var_t8__blk1110_dn11, var_t8__blk1110_dn12, var_t8__blk1110_dn17,)
    } else {
        (var_t9__blk1111, var_t9__blk1111_dn0, var_t9__blk1111_dn2, var_t9__blk1111_dn6, var_t9__blk1111_dn7, var_t9__blk1111_dn10, var_t9__blk1111_dn11, var_t9__blk1111_dn12, var_t9__blk1111_dn17,)
    }
};
        var_t9__blk1111 = assign33690_e48854;
        var_t9__blk1111_dn0 = assign33690_e48854_d_n0;
        var_t9__blk1111_dn2 = assign33690_e48854_d_n2;
        var_t9__blk1111_dn6 = assign33690_e48854_d_n6;
        var_t9__blk1111_dn7 = assign33690_e48854_d_n7;
        var_t9__blk1111_dn10 = assign33690_e48854_d_n10;
        var_t9__blk1111_dn11 = assign33690_e48854_d_n11;
        var_t9__blk1111_dn12 = assign33690_e48854_d_n12;
        var_t9__blk1111_dn17 = assign33690_e48854_d_n17;

        let (assign33700_e48865, assign33700_e48865_d_n0, assign33700_e48865_d_n2, assign33700_e48865_d_n6, assign33700_e48865_d_n7, assign33700_e48865_d_n10, assign33700_e48865_d_n11, assign33700_e48865_d_n12, assign33700_e48865_d_n17,) = {
    if (var_guard1124 != 0.0) {
        let assign33700_e48858: f64 = (-1.0);
        let assign33700_e48860: f64 = (assign33700_e48858 / p.p113);
        let assign33700_e48862: f64 = (assign33700_e48860 - 1.0);
        let assign33700_e48863: f64 = (var_t9__blk1111).powf(assign33700_e48862);
        (assign33700_e48863, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((var_t9__blk1111).powf(assign33700_e48862 - 1.0) * var_t9__blk1111_dn0)) } } else { (assign33700_e48863 * (assign33700_e48862 * (var_t9__blk1111_dn0 / var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((var_t9__blk1111).powf(assign33700_e48862 - 1.0) * var_t9__blk1111_dn2)) } } else { (assign33700_e48863 * (assign33700_e48862 * (var_t9__blk1111_dn2 / var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((var_t9__blk1111).powf(assign33700_e48862 - 1.0) * var_t9__blk1111_dn6)) } } else { (assign33700_e48863 * (assign33700_e48862 * (var_t9__blk1111_dn6 / var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((var_t9__blk1111).powf(assign33700_e48862 - 1.0) * var_t9__blk1111_dn7)) } } else { (assign33700_e48863 * (assign33700_e48862 * (var_t9__blk1111_dn7 / var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((var_t9__blk1111).powf(assign33700_e48862 - 1.0) * var_t9__blk1111_dn10)) } } else { (assign33700_e48863 * (assign33700_e48862 * (var_t9__blk1111_dn10 / var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((var_t9__blk1111).powf(assign33700_e48862 - 1.0) * var_t9__blk1111_dn11)) } } else { (assign33700_e48863 * (assign33700_e48862 * (var_t9__blk1111_dn11 / var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((var_t9__blk1111).powf(assign33700_e48862 - 1.0) * var_t9__blk1111_dn12)) } } else { (assign33700_e48863 * (assign33700_e48862 * (var_t9__blk1111_dn12 / var_t9__blk1111))) }, if 0.0 == 0.0 && ((assign33700_e48862) as f64).is_finite() && ((assign33700_e48862) as f64).fract() == 0.0 { if assign33700_e48862 == 0.0 { 0.0 } else { (assign33700_e48862 * ((var_t9__blk1111).powf(assign33700_e48862 - 1.0) * var_t9__blk1111_dn17)) } } else { (assign33700_e48863 * (assign33700_e48862 * (var_t9__blk1111_dn17 / var_t9__blk1111))) },)
    } else {
        (var_t10__blk1112, var_t10__blk1112_dn0, var_t10__blk1112_dn2, var_t10__blk1112_dn6, var_t10__blk1112_dn7, var_t10__blk1112_dn10, var_t10__blk1112_dn11, var_t10__blk1112_dn12, var_t10__blk1112_dn17,)
    }
};
        var_t10__blk1112 = assign33700_e48865;
        var_t10__blk1112_dn0 = assign33700_e48865_d_n0;
        var_t10__blk1112_dn2 = assign33700_e48865_d_n2;
        var_t10__blk1112_dn6 = assign33700_e48865_d_n6;
        var_t10__blk1112_dn7 = assign33700_e48865_d_n7;
        var_t10__blk1112_dn10 = assign33700_e48865_d_n10;
        var_t10__blk1112_dn11 = assign33700_e48865_d_n11;
        var_t10__blk1112_dn12 = assign33700_e48865_d_n12;
        var_t10__blk1112_dn17 = assign33700_e48865_d_n17;

        let (assign33710_e48871, assign33710_e48871_d_n0, assign33710_e48871_d_n2, assign33710_e48871_d_n6, assign33710_e48871_d_n7, assign33710_e48871_d_n10, assign33710_e48871_d_n11, assign33710_e48871_d_n12, assign33710_e48871_d_n17,) = {
    if (var_guard1124 != 0.0) {
        let assign33710_e48869: f64 = (var_t9__blk1111 * var_t10__blk1112);
        (assign33710_e48869, ((var_t9__blk1111_dn0 * var_t10__blk1112) + (var_t9__blk1111 * var_t10__blk1112_dn0)), ((var_t9__blk1111_dn2 * var_t10__blk1112) + (var_t9__blk1111 * var_t10__blk1112_dn2)), ((var_t9__blk1111_dn6 * var_t10__blk1112) + (var_t9__blk1111 * var_t10__blk1112_dn6)), ((var_t9__blk1111_dn7 * var_t10__blk1112) + (var_t9__blk1111 * var_t10__blk1112_dn7)), ((var_t9__blk1111_dn10 * var_t10__blk1112) + (var_t9__blk1111 * var_t10__blk1112_dn10)), ((var_t9__blk1111_dn11 * var_t10__blk1112) + (var_t9__blk1111 * var_t10__blk1112_dn11)), ((var_t9__blk1111_dn12 * var_t10__blk1112) + (var_t9__blk1111 * var_t10__blk1112_dn12)), ((var_t9__blk1111_dn17 * var_t10__blk1112) + (var_t9__blk1111 * var_t10__blk1112_dn17)),)
    } else {
        (var_t11__blk1113, var_t11__blk1113_dn0, var_t11__blk1113_dn2, var_t11__blk1113_dn6, var_t11__blk1113_dn7, var_t11__blk1113_dn10, var_t11__blk1113_dn11, var_t11__blk1113_dn12, var_t11__blk1113_dn17,)
    }
};
        var_t11__blk1113 = assign33710_e48871;
        var_t11__blk1113_dn0 = assign33710_e48871_d_n0;
        var_t11__blk1113_dn2 = assign33710_e48871_d_n2;
        var_t11__blk1113_dn6 = assign33710_e48871_d_n6;
        var_t11__blk1113_dn7 = assign33710_e48871_d_n7;
        var_t11__blk1113_dn10 = assign33710_e48871_d_n10;
        var_t11__blk1113_dn11 = assign33710_e48871_d_n11;
        var_t11__blk1113_dn12 = assign33710_e48871_d_n12;
        var_t11__blk1113_dn17 = assign33710_e48871_d_n17;

        let (assign33720_e48877, assign33720_e48877_d_n0, assign33720_e48877_d_n2, assign33720_e48877_d_n6, assign33720_e48877_d_n7, assign33720_e48877_d_n10, assign33720_e48877_d_n11, assign33720_e48877_d_n12, assign33720_e48877_d_n17,) = {
    if (var_guard1124 != 0.0) {
        let assign33720_e48875: f64 = (var_muun * var_t11__blk1113);
        (assign33720_e48875, ((var_muun_dn0 * var_t11__blk1113) + (var_muun * var_t11__blk1113_dn0)), ((var_muun_dn2 * var_t11__blk1113) + (var_muun * var_t11__blk1113_dn2)), ((var_muun_dn6 * var_t11__blk1113) + (var_muun * var_t11__blk1113_dn6)), ((var_muun_dn7 * var_t11__blk1113) + (var_muun * var_t11__blk1113_dn7)), ((var_muun_dn10 * var_t11__blk1113) + (var_muun * var_t11__blk1113_dn10)), ((var_muun_dn11 * var_t11__blk1113) + (var_muun * var_t11__blk1113_dn11)), ((var_muun_dn12 * var_t11__blk1113) + (var_muun * var_t11__blk1113_dn12)), ((var_muun_dn17 * var_t11__blk1113) + (var_muun * var_t11__blk1113_dn17)),)
    } else {
        (var_mud_hoso, var_mud_hoso_dn0, var_mud_hoso_dn2, var_mud_hoso_dn6, var_mud_hoso_dn7, var_mud_hoso_dn10, var_mud_hoso_dn11, var_mud_hoso_dn12, var_mud_hoso_dn17,)
    }
};
        var_mud_hoso = assign33720_e48877;
        var_mud_hoso_dn0 = assign33720_e48877_d_n0;
        var_mud_hoso_dn2 = assign33720_e48877_d_n2;
        var_mud_hoso_dn6 = assign33720_e48877_d_n6;
        var_mud_hoso_dn7 = assign33720_e48877_d_n7;
        var_mud_hoso_dn10 = assign33720_e48877_d_n10;
        var_mud_hoso_dn11 = assign33720_e48877_d_n11;
        var_mud_hoso_dn12 = assign33720_e48877_d_n12;
        var_mud_hoso_dn17 = assign33720_e48877_d_n17;

        let (assign33730_e48885, assign33730_e48885_d_n0, assign33730_e48885_d_n2, assign33730_e48885_d_n6, assign33730_e48885_d_n7, assign33730_e48885_d_n10, assign33730_e48885_d_n11, assign33730_e48885_d_n12, assign33730_e48885_d_n17,) = {
    if (var_guard1124 != 0.0) {
        let assign33730_e48881: f64 = (var_mu + var_mud_hoso);
        let assign33730_e48883: f64 = (assign33730_e48881 / 2.0);
        (assign33730_e48883, ((var_mu_dn0 + var_mud_hoso_dn0) / 2.0), ((var_mu_dn2 + var_mud_hoso_dn2) / 2.0), ((var_mu_dn6 + var_mud_hoso_dn6) / 2.0), ((var_mu_dn7 + var_mud_hoso_dn7) / 2.0), ((var_mu_dn10 + var_mud_hoso_dn10) / 2.0), ((var_mu_dn11 + var_mud_hoso_dn11) / 2.0), ((var_mu_dn12 + var_mud_hoso_dn12) / 2.0), ((var_mu_dn17 + var_mud_hoso_dn17) / 2.0),)
    } else {
        (var_mu_ave, var_mu_ave_dn0, var_mu_ave_dn2, var_mu_ave_dn6, var_mu_ave_dn7, var_mu_ave_dn10, var_mu_ave_dn11, var_mu_ave_dn12, var_mu_ave_dn17,)
    }
};
        var_mu_ave = assign33730_e48885;
        var_mu_ave_dn0 = assign33730_e48885_d_n0;
        var_mu_ave_dn2 = assign33730_e48885_d_n2;
        var_mu_ave_dn6 = assign33730_e48885_d_n6;
        var_mu_ave_dn7 = assign33730_e48885_d_n7;
        var_mu_ave_dn10 = assign33730_e48885_d_n10;
        var_mu_ave_dn11 = assign33730_e48885_d_n11;
        var_mu_ave_dn12 = assign33730_e48885_d_n12;
        var_mu_ave_dn17 = assign33730_e48885_d_n17;

        let (assign33740_e48891, assign33740_e48891_d_n0, assign33740_e48891_d_n2, assign33740_e48891_d_n6, assign33740_e48891_d_n7, assign33740_e48891_d_n10, assign33740_e48891_d_n11, assign33740_e48891_d_n12, assign33740_e48891_d_n17,) = {
    if (var_guard1124 != 0.0) {
        let assign33740_e48889: f64 = (var_alpha * var_alpha);
        (assign33740_e48889, ((var_alpha_dn0 * var_alpha) + (var_alpha * var_alpha_dn0)), ((var_alpha_dn2 * var_alpha) + (var_alpha * var_alpha_dn2)), ((var_alpha_dn6 * var_alpha) + (var_alpha * var_alpha_dn6)), ((var_alpha_dn7 * var_alpha) + (var_alpha * var_alpha_dn7)), ((var_alpha_dn10 * var_alpha) + (var_alpha * var_alpha_dn10)), ((var_alpha_dn11 * var_alpha) + (var_alpha * var_alpha_dn11)), ((var_alpha_dn12 * var_alpha) + (var_alpha * var_alpha_dn12)), ((var_alpha_dn17 * var_alpha) + (var_alpha * var_alpha_dn17)),)
    } else {
        (var_t0__blk1114, var_t0__blk1114_dn0, var_t0__blk1114_dn2, var_t0__blk1114_dn6, var_t0__blk1114_dn7, var_t0__blk1114_dn10, var_t0__blk1114_dn11, var_t0__blk1114_dn12, var_t0__blk1114_dn17,)
    }
};
        var_t0__blk1114 = assign33740_e48891;
        var_t0__blk1114_dn0 = assign33740_e48891_d_n0;
        var_t0__blk1114_dn2 = assign33740_e48891_d_n2;
        var_t0__blk1114_dn6 = assign33740_e48891_d_n6;
        var_t0__blk1114_dn7 = assign33740_e48891_d_n7;
        var_t0__blk1114_dn10 = assign33740_e48891_d_n10;
        var_t0__blk1114_dn11 = assign33740_e48891_d_n11;
        var_t0__blk1114_dn12 = assign33740_e48891_d_n12;
        var_t0__blk1114_dn17 = assign33740_e48891_d_n17;

        let (assign33750_e48953, assign33750_e48953_d_n0, assign33750_e48953_d_n2, assign33750_e48953_d_n6, assign33750_e48953_d_n7, assign33750_e48953_d_n10, assign33750_e48953_d_n11, assign33750_e48953_d_n12, assign33750_e48953_d_n17,) = {
    if (var_guard1124 != 0.0) {
        let assign33750_e48895: f64 = (var_weff_nf * var_c_fox);
        let assign33750_e48897: f64 = (assign33750_e48895 * var_vgvt);
        let assign33750_e48899: f64 = (assign33750_e48897 * var_mu);
        let assign33750_e48903: f64 = (3.0 * var_alpha);
        let assign33750_e48904: f64 = (1.0 + assign33750_e48903);
        let assign33750_e48907: f64 = (6.0 * var_t0__blk1114);
        let assign33750_e48908: f64 = (assign33750_e48904 + assign33750_e48907);
        let assign33750_e48910: f64 = (assign33750_e48908 * var_mud_hoso);
        let assign33750_e48912: f64 = (assign33750_e48910 * var_mud_hoso);
        let assign33750_e48916: f64 = (4.0 * var_alpha);
        let assign33750_e48917: f64 = (3.0 + assign33750_e48916);
        let assign33750_e48920: f64 = (3.0 * var_t0__blk1114);
        let assign33750_e48921: f64 = (assign33750_e48917 + assign33750_e48920);
        let assign33750_e48923: f64 = (assign33750_e48921 * var_mud_hoso);
        let assign33750_e48925: f64 = (assign33750_e48923 * var_mu);
        let assign33750_e48926: f64 = (assign33750_e48912 + assign33750_e48925);
        let assign33750_e48930: f64 = (3.0 * var_alpha);
        let assign33750_e48931: f64 = (6.0 + assign33750_e48930);
        let assign33750_e48933: f64 = (assign33750_e48931 + var_t0__blk1114);
        let assign33750_e48935: f64 = (assign33750_e48933 * var_mu);
        let assign33750_e48937: f64 = (assign33750_e48935 * var_mu);
        let assign33750_e48938: f64 = (assign33750_e48926 + assign33750_e48937);
        let assign33750_e48939: f64 = (assign33750_e48899 * assign33750_e48938);
        let assign33750_e48942: f64 = (15.0 * var_lch);
        let assign33750_e48945: f64 = (1.0 + var_alpha);
        let assign33750_e48946: f64 = (assign33750_e48942 * assign33750_e48945);
        let assign33750_e48948: f64 = (assign33750_e48946 * var_mu_ave);
        let assign33750_e48950: f64 = (assign33750_e48948 * var_mu_ave);
        let assign33750_e48951: f64 = (assign33750_e48939 / assign33750_e48950);
        (assign33750_e48951, ((((((((((var_weff_nf * var_c_fox_dn0) * var_vgvt) + (assign33750_e48895 * var_vgvt_dn0)) * var_mu) + (assign33750_e48897 * var_mu_dn0)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * var_alpha_dn0) + (6.0 * var_t0__blk1114_dn0)) * var_mud_hoso) + (assign33750_e48908 * var_mud_hoso_dn0)) * var_mud_hoso) + (assign33750_e48910 * var_mud_hoso_dn0)) + ((((((4.0 * var_alpha_dn0) + (3.0 * var_t0__blk1114_dn0)) * var_mud_hoso) + (assign33750_e48921 * var_mud_hoso_dn0)) * var_mu) + (assign33750_e48923 * var_mu_dn0))) + ((((((3.0 * var_alpha_dn0) + var_t0__blk1114_dn0) * var_mu) + (assign33750_e48933 * var_mu_dn0)) * var_mu) + (assign33750_e48935 * var_mu_dn0))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * var_lch_dn0) * assign33750_e48945) + (assign33750_e48942 * var_alpha_dn0)) * var_mu_ave) + (assign33750_e48946 * var_mu_ave_dn0)) * var_mu_ave) + (assign33750_e48948 * var_mu_ave_dn0)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((var_weff_nf * var_c_fox_dn2) * var_vgvt) + (assign33750_e48895 * var_vgvt_dn2)) * var_mu) + (assign33750_e48897 * var_mu_dn2)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * var_alpha_dn2) + (6.0 * var_t0__blk1114_dn2)) * var_mud_hoso) + (assign33750_e48908 * var_mud_hoso_dn2)) * var_mud_hoso) + (assign33750_e48910 * var_mud_hoso_dn2)) + ((((((4.0 * var_alpha_dn2) + (3.0 * var_t0__blk1114_dn2)) * var_mud_hoso) + (assign33750_e48921 * var_mud_hoso_dn2)) * var_mu) + (assign33750_e48923 * var_mu_dn2))) + ((((((3.0 * var_alpha_dn2) + var_t0__blk1114_dn2) * var_mu) + (assign33750_e48933 * var_mu_dn2)) * var_mu) + (assign33750_e48935 * var_mu_dn2))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * var_lch_dn2) * assign33750_e48945) + (assign33750_e48942 * var_alpha_dn2)) * var_mu_ave) + (assign33750_e48946 * var_mu_ave_dn2)) * var_mu_ave) + (assign33750_e48948 * var_mu_ave_dn2)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((var_weff_nf * var_c_fox_dn6) * var_vgvt) + (assign33750_e48895 * var_vgvt_dn6)) * var_mu) + (assign33750_e48897 * var_mu_dn6)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * var_alpha_dn6) + (6.0 * var_t0__blk1114_dn6)) * var_mud_hoso) + (assign33750_e48908 * var_mud_hoso_dn6)) * var_mud_hoso) + (assign33750_e48910 * var_mud_hoso_dn6)) + ((((((4.0 * var_alpha_dn6) + (3.0 * var_t0__blk1114_dn6)) * var_mud_hoso) + (assign33750_e48921 * var_mud_hoso_dn6)) * var_mu) + (assign33750_e48923 * var_mu_dn6))) + ((((((3.0 * var_alpha_dn6) + var_t0__blk1114_dn6) * var_mu) + (assign33750_e48933 * var_mu_dn6)) * var_mu) + (assign33750_e48935 * var_mu_dn6))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * var_lch_dn6) * assign33750_e48945) + (assign33750_e48942 * var_alpha_dn6)) * var_mu_ave) + (assign33750_e48946 * var_mu_ave_dn6)) * var_mu_ave) + (assign33750_e48948 * var_mu_ave_dn6)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((var_weff_nf * var_c_fox_dn7) * var_vgvt) + (assign33750_e48895 * var_vgvt_dn7)) * var_mu) + (assign33750_e48897 * var_mu_dn7)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * var_alpha_dn7) + (6.0 * var_t0__blk1114_dn7)) * var_mud_hoso) + (assign33750_e48908 * var_mud_hoso_dn7)) * var_mud_hoso) + (assign33750_e48910 * var_mud_hoso_dn7)) + ((((((4.0 * var_alpha_dn7) + (3.0 * var_t0__blk1114_dn7)) * var_mud_hoso) + (assign33750_e48921 * var_mud_hoso_dn7)) * var_mu) + (assign33750_e48923 * var_mu_dn7))) + ((((((3.0 * var_alpha_dn7) + var_t0__blk1114_dn7) * var_mu) + (assign33750_e48933 * var_mu_dn7)) * var_mu) + (assign33750_e48935 * var_mu_dn7))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * var_lch_dn7) * assign33750_e48945) + (assign33750_e48942 * var_alpha_dn7)) * var_mu_ave) + (assign33750_e48946 * var_mu_ave_dn7)) * var_mu_ave) + (assign33750_e48948 * var_mu_ave_dn7)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((var_weff_nf * var_c_fox_dn10) * var_vgvt) + (assign33750_e48895 * var_vgvt_dn10)) * var_mu) + (assign33750_e48897 * var_mu_dn10)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * var_alpha_dn10) + (6.0 * var_t0__blk1114_dn10)) * var_mud_hoso) + (assign33750_e48908 * var_mud_hoso_dn10)) * var_mud_hoso) + (assign33750_e48910 * var_mud_hoso_dn10)) + ((((((4.0 * var_alpha_dn10) + (3.0 * var_t0__blk1114_dn10)) * var_mud_hoso) + (assign33750_e48921 * var_mud_hoso_dn10)) * var_mu) + (assign33750_e48923 * var_mu_dn10))) + ((((((3.0 * var_alpha_dn10) + var_t0__blk1114_dn10) * var_mu) + (assign33750_e48933 * var_mu_dn10)) * var_mu) + (assign33750_e48935 * var_mu_dn10))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * var_lch_dn10) * assign33750_e48945) + (assign33750_e48942 * var_alpha_dn10)) * var_mu_ave) + (assign33750_e48946 * var_mu_ave_dn10)) * var_mu_ave) + (assign33750_e48948 * var_mu_ave_dn10)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((var_weff_nf * var_c_fox_dn11) * var_vgvt) + (assign33750_e48895 * var_vgvt_dn11)) * var_mu) + (assign33750_e48897 * var_mu_dn11)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * var_alpha_dn11) + (6.0 * var_t0__blk1114_dn11)) * var_mud_hoso) + (assign33750_e48908 * var_mud_hoso_dn11)) * var_mud_hoso) + (assign33750_e48910 * var_mud_hoso_dn11)) + ((((((4.0 * var_alpha_dn11) + (3.0 * var_t0__blk1114_dn11)) * var_mud_hoso) + (assign33750_e48921 * var_mud_hoso_dn11)) * var_mu) + (assign33750_e48923 * var_mu_dn11))) + ((((((3.0 * var_alpha_dn11) + var_t0__blk1114_dn11) * var_mu) + (assign33750_e48933 * var_mu_dn11)) * var_mu) + (assign33750_e48935 * var_mu_dn11))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * var_lch_dn11) * assign33750_e48945) + (assign33750_e48942 * var_alpha_dn11)) * var_mu_ave) + (assign33750_e48946 * var_mu_ave_dn11)) * var_mu_ave) + (assign33750_e48948 * var_mu_ave_dn11)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((var_weff_nf * var_c_fox_dn12) * var_vgvt) + (assign33750_e48895 * var_vgvt_dn12)) * var_mu) + (assign33750_e48897 * var_mu_dn12)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * var_alpha_dn12) + (6.0 * var_t0__blk1114_dn12)) * var_mud_hoso) + (assign33750_e48908 * var_mud_hoso_dn12)) * var_mud_hoso) + (assign33750_e48910 * var_mud_hoso_dn12)) + ((((((4.0 * var_alpha_dn12) + (3.0 * var_t0__blk1114_dn12)) * var_mud_hoso) + (assign33750_e48921 * var_mud_hoso_dn12)) * var_mu) + (assign33750_e48923 * var_mu_dn12))) + ((((((3.0 * var_alpha_dn12) + var_t0__blk1114_dn12) * var_mu) + (assign33750_e48933 * var_mu_dn12)) * var_mu) + (assign33750_e48935 * var_mu_dn12))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * var_lch_dn12) * assign33750_e48945) + (assign33750_e48942 * var_alpha_dn12)) * var_mu_ave) + (assign33750_e48946 * var_mu_ave_dn12)) * var_mu_ave) + (assign33750_e48948 * var_mu_ave_dn12)))) / (assign33750_e48950 * assign33750_e48950)), ((((((((((var_weff_nf * var_c_fox_dn17) * var_vgvt) + (assign33750_e48895 * var_vgvt_dn17)) * var_mu) + (assign33750_e48897 * var_mu_dn17)) * assign33750_e48938) + (assign33750_e48899 * ((((((((3.0 * var_alpha_dn17) + (6.0 * var_t0__blk1114_dn17)) * var_mud_hoso) + (assign33750_e48908 * var_mud_hoso_dn17)) * var_mud_hoso) + (assign33750_e48910 * var_mud_hoso_dn17)) + ((((((4.0 * var_alpha_dn17) + (3.0 * var_t0__blk1114_dn17)) * var_mud_hoso) + (assign33750_e48921 * var_mud_hoso_dn17)) * var_mu) + (assign33750_e48923 * var_mu_dn17))) + ((((((3.0 * var_alpha_dn17) + var_t0__blk1114_dn17) * var_mu) + (assign33750_e48933 * var_mu_dn17)) * var_mu) + (assign33750_e48935 * var_mu_dn17))))) * assign33750_e48950) - (assign33750_e48939 * (((((((15.0 * var_lch_dn17) * assign33750_e48945) + (assign33750_e48942 * var_alpha_dn17)) * var_mu_ave) + (assign33750_e48946 * var_mu_ave_dn17)) * var_mu_ave) + (assign33750_e48948 * var_mu_ave_dn17)))) / (assign33750_e48950 * assign33750_e48950)),)
    } else {
        (var_nthrml, var_nthrml_dn0, var_nthrml_dn2, var_nthrml_dn6, var_nthrml_dn7, var_nthrml_dn10, var_nthrml_dn11, var_nthrml_dn12, var_nthrml_dn17,)
    }
};
        var_nthrml = assign33750_e48953;
        var_nthrml_dn0 = assign33750_e48953_d_n0;
        var_nthrml_dn2 = assign33750_e48953_d_n2;
        var_nthrml_dn6 = assign33750_e48953_d_n6;
        var_nthrml_dn7 = assign33750_e48953_d_n7;
        var_nthrml_dn10 = assign33750_e48953_d_n10;
        var_nthrml_dn11 = assign33750_e48953_d_n11;
        var_nthrml_dn12 = assign33750_e48953_d_n12;
        var_nthrml_dn17 = assign33750_e48953_d_n17;

        let (assign33760_e48958, assign33760_e48958_d_n0, assign33760_e48958_d_n2, assign33760_e48958_d_n6, assign33760_e48958_d_n7, assign33760_e48958_d_n10, assign33760_e48958_d_n11, assign33760_e48958_d_n12, assign33760_e48958_d_n17,) = {
    if (var_guard1124 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nthrml, var_nthrml_dn0, var_nthrml_dn2, var_nthrml_dn6, var_nthrml_dn7, var_nthrml_dn10, var_nthrml_dn11, var_nthrml_dn12, var_nthrml_dn17,)
    }
};
        var_nthrml = assign33760_e48958;
        var_nthrml_dn0 = assign33760_e48958_d_n0;
        var_nthrml_dn2 = assign33760_e48958_d_n2;
        var_nthrml_dn6 = assign33760_e48958_d_n6;
        var_nthrml_dn7 = assign33760_e48958_d_n7;
        var_nthrml_dn10 = assign33760_e48958_d_n10;
        var_nthrml_dn11 = assign33760_e48958_d_n11;
        var_nthrml_dn12 = assign33760_e48958_d_n12;
        var_nthrml_dn17 = assign33760_e48958_d_n17;

        let assign33770_e48972: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (var_flg_ign == 1.0)) && (var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        var_guard1127 = assign33770_e48972;

        let (assign33780_e48977, assign33780_e48977_d_n0, assign33780_e48977_d_n2, assign33780_e48977_d_n6, assign33780_e48977_d_n7, assign33780_e48977_d_n10, assign33780_e48977_d_n11, assign33780_e48977_d_n12, assign33780_e48977_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33780_e48975: f64 = (var_kusail).sqrt();
        (assign33780_e48975, (var_kusail_dn0 / (2.0 * assign33780_e48975)), (var_kusail_dn2 / (2.0 * assign33780_e48975)), (var_kusail_dn6 / (2.0 * assign33780_e48975)), (var_kusail_dn7 / (2.0 * assign33780_e48975)), (var_kusail_dn10 / (2.0 * assign33780_e48975)), (var_kusail_dn11 / (2.0 * assign33780_e48975)), (var_kusail_dn12 / (2.0 * assign33780_e48975)), (var_kusail_dn17 / (2.0 * assign33780_e48975)),)
    } else {
        (var_sqrtkusail, var_sqrtkusail_dn0, var_sqrtkusail_dn2, var_sqrtkusail_dn6, var_sqrtkusail_dn7, var_sqrtkusail_dn10, var_sqrtkusail_dn11, var_sqrtkusail_dn12, var_sqrtkusail_dn17,)
    }
};
        var_sqrtkusail = assign33780_e48977;
        var_sqrtkusail_dn0 = assign33780_e48977_d_n0;
        var_sqrtkusail_dn2 = assign33780_e48977_d_n2;
        var_sqrtkusail_dn6 = assign33780_e48977_d_n6;
        var_sqrtkusail_dn7 = assign33780_e48977_d_n7;
        var_sqrtkusail_dn10 = assign33780_e48977_d_n10;
        var_sqrtkusail_dn11 = assign33780_e48977_d_n11;
        var_sqrtkusail_dn12 = assign33780_e48977_d_n12;
        var_sqrtkusail_dn17 = assign33780_e48977_d_n17;

        let (assign33790_e48983, assign33790_e48983_d_n0, assign33790_e48983_d_n2, assign33790_e48983_d_n6, assign33790_e48983_d_n7, assign33790_e48983_d_n10, assign33790_e48983_d_n11, assign33790_e48983_d_n12, assign33790_e48983_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33790_e48981: f64 = (var_vgvt + var_sqrtkusail);
        (assign33790_e48981, (var_vgvt_dn0 + var_sqrtkusail_dn0), (var_vgvt_dn2 + var_sqrtkusail_dn2), (var_vgvt_dn6 + var_sqrtkusail_dn6), (var_vgvt_dn7 + var_sqrtkusail_dn7), (var_vgvt_dn10 + var_sqrtkusail_dn10), (var_vgvt_dn11 + var_sqrtkusail_dn11), (var_vgvt_dn12 + var_sqrtkusail_dn12), (var_vgvt_dn17 + var_sqrtkusail_dn17),)
    } else {
        (var_t2__blk1116, var_t2__blk1116_dn0, var_t2__blk1116_dn2, var_t2__blk1116_dn6, var_t2__blk1116_dn7, var_t2__blk1116_dn10, var_t2__blk1116_dn11, var_t2__blk1116_dn12, var_t2__blk1116_dn17,)
    }
};
        var_t2__blk1116 = assign33790_e48983;
        var_t2__blk1116_dn0 = assign33790_e48983_d_n0;
        var_t2__blk1116_dn2 = assign33790_e48983_d_n2;
        var_t2__blk1116_dn6 = assign33790_e48983_d_n6;
        var_t2__blk1116_dn7 = assign33790_e48983_d_n7;
        var_t2__blk1116_dn10 = assign33790_e48983_d_n10;
        var_t2__blk1116_dn11 = assign33790_e48983_d_n11;
        var_t2__blk1116_dn12 = assign33790_e48983_d_n12;
        var_t2__blk1116_dn17 = assign33790_e48983_d_n17;

        let (assign33800_e48989, assign33800_e48989_d_n0, assign33800_e48989_d_n2, assign33800_e48989_d_n6, assign33800_e48989_d_n7, assign33800_e48989_d_n10, assign33800_e48989_d_n11, assign33800_e48989_d_n12, assign33800_e48989_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33800_e48987: f64 = (var_kusai00 * var_kusai00);
        (assign33800_e48987, ((var_kusai00_dn0 * var_kusai00) + (var_kusai00 * var_kusai00_dn0)), ((var_kusai00_dn2 * var_kusai00) + (var_kusai00 * var_kusai00_dn2)), ((var_kusai00_dn6 * var_kusai00) + (var_kusai00 * var_kusai00_dn6)), ((var_kusai00_dn7 * var_kusai00) + (var_kusai00 * var_kusai00_dn7)), ((var_kusai00_dn10 * var_kusai00) + (var_kusai00 * var_kusai00_dn10)), ((var_kusai00_dn11 * var_kusai00) + (var_kusai00 * var_kusai00_dn11)), ((var_kusai00_dn12 * var_kusai00) + (var_kusai00 * var_kusai00_dn12)), ((var_kusai00_dn17 * var_kusai00) + (var_kusai00 * var_kusai00_dn17)),)
    } else {
        (var_t3__blk1117, var_t3__blk1117_dn0, var_t3__blk1117_dn2, var_t3__blk1117_dn6, var_t3__blk1117_dn7, var_t3__blk1117_dn10, var_t3__blk1117_dn11, var_t3__blk1117_dn12, var_t3__blk1117_dn17,)
    }
};
        var_t3__blk1117 = assign33800_e48989;
        var_t3__blk1117_dn0 = assign33800_e48989_d_n0;
        var_t3__blk1117_dn2 = assign33800_e48989_d_n2;
        var_t3__blk1117_dn6 = assign33800_e48989_d_n6;
        var_t3__blk1117_dn7 = assign33800_e48989_d_n7;
        var_t3__blk1117_dn10 = assign33800_e48989_d_n10;
        var_t3__blk1117_dn11 = assign33800_e48989_d_n11;
        var_t3__blk1117_dn12 = assign33800_e48989_d_n12;
        var_t3__blk1117_dn17 = assign33800_e48989_d_n17;

        let (assign33810_e48995, assign33810_e48995_d_n0, assign33810_e48995_d_n2, assign33810_e48995_d_n6, assign33810_e48995_d_n7, assign33810_e48995_d_n10, assign33810_e48995_d_n11, assign33810_e48995_d_n12, assign33810_e48995_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33810_e48993: f64 = (var_kusail * var_kusail);
        (assign33810_e48993, ((var_kusail_dn0 * var_kusail) + (var_kusail * var_kusail_dn0)), ((var_kusail_dn2 * var_kusail) + (var_kusail * var_kusail_dn2)), ((var_kusail_dn6 * var_kusail) + (var_kusail * var_kusail_dn6)), ((var_kusail_dn7 * var_kusail) + (var_kusail * var_kusail_dn7)), ((var_kusail_dn10 * var_kusail) + (var_kusail * var_kusail_dn10)), ((var_kusail_dn11 * var_kusail) + (var_kusail * var_kusail_dn11)), ((var_kusail_dn12 * var_kusail) + (var_kusail * var_kusail_dn12)), ((var_kusail_dn17 * var_kusail) + (var_kusail * var_kusail_dn17)),)
    } else {
        (var_t4__blk1118, var_t4__blk1118_dn0, var_t4__blk1118_dn2, var_t4__blk1118_dn6, var_t4__blk1118_dn7, var_t4__blk1118_dn10, var_t4__blk1118_dn11, var_t4__blk1118_dn12, var_t4__blk1118_dn17,)
    }
};
        var_t4__blk1118 = assign33810_e48995;
        var_t4__blk1118_dn0 = assign33810_e48995_d_n0;
        var_t4__blk1118_dn2 = assign33810_e48995_d_n2;
        var_t4__blk1118_dn6 = assign33810_e48995_d_n6;
        var_t4__blk1118_dn7 = assign33810_e48995_d_n7;
        var_t4__blk1118_dn10 = assign33810_e48995_d_n10;
        var_t4__blk1118_dn11 = assign33810_e48995_d_n11;
        var_t4__blk1118_dn12 = assign33810_e48995_d_n12;
        var_t4__blk1118_dn17 = assign33810_e48995_d_n17;

        let (assign33820_e49003, assign33820_e49003_d_n0, assign33820_e49003_d_n2, assign33820_e49003_d_n6, assign33820_e49003_d_n7, assign33820_e49003_d_n10, assign33820_e49003_d_n11, assign33820_e49003_d_n12, assign33820_e49003_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33820_e48999: f64 = (42.0 * var_kusai00);
        let assign33820_e49001: f64 = (assign33820_e48999 * var_kusail);
        (assign33820_e49001, (((42.0 * var_kusai00_dn0) * var_kusail) + (assign33820_e48999 * var_kusail_dn0)), (((42.0 * var_kusai00_dn2) * var_kusail) + (assign33820_e48999 * var_kusail_dn2)), (((42.0 * var_kusai00_dn6) * var_kusail) + (assign33820_e48999 * var_kusail_dn6)), (((42.0 * var_kusai00_dn7) * var_kusail) + (assign33820_e48999 * var_kusail_dn7)), (((42.0 * var_kusai00_dn10) * var_kusail) + (assign33820_e48999 * var_kusail_dn10)), (((42.0 * var_kusai00_dn11) * var_kusail) + (assign33820_e48999 * var_kusail_dn11)), (((42.0 * var_kusai00_dn12) * var_kusail) + (assign33820_e48999 * var_kusail_dn12)), (((42.0 * var_kusai00_dn17) * var_kusail) + (assign33820_e48999 * var_kusail_dn17)),)
    } else {
        (var_t5__blk1119, var_t5__blk1119_dn0, var_t5__blk1119_dn2, var_t5__blk1119_dn6, var_t5__blk1119_dn7, var_t5__blk1119_dn10, var_t5__blk1119_dn11, var_t5__blk1119_dn12, var_t5__blk1119_dn17,)
    }
};
        var_t5__blk1119 = assign33820_e49003;
        var_t5__blk1119_dn0 = assign33820_e49003_d_n0;
        var_t5__blk1119_dn2 = assign33820_e49003_d_n2;
        var_t5__blk1119_dn6 = assign33820_e49003_d_n6;
        var_t5__blk1119_dn7 = assign33820_e49003_d_n7;
        var_t5__blk1119_dn10 = assign33820_e49003_d_n10;
        var_t5__blk1119_dn11 = assign33820_e49003_d_n11;
        var_t5__blk1119_dn12 = assign33820_e49003_d_n12;
        var_t5__blk1119_dn17 = assign33820_e49003_d_n17;

        let (assign33830_e49013, assign33830_e49013_d_n0, assign33830_e49013_d_n2, assign33830_e49013_d_n6, assign33830_e49013_d_n7, assign33830_e49013_d_n10, assign33830_e49013_d_n11, assign33830_e49013_d_n12, assign33830_e49013_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33830_e49009: f64 = (var_t3__blk1117 + var_t4__blk1118);
        let assign33830_e49010: f64 = (4.0 * assign33830_e49009);
        let assign33830_e49011: f64 = (var_t5__blk1119 + assign33830_e49010);
        (assign33830_e49011, (var_t5__blk1119_dn0 + (4.0 * (var_t3__blk1117_dn0 + var_t4__blk1118_dn0))), (var_t5__blk1119_dn2 + (4.0 * (var_t3__blk1117_dn2 + var_t4__blk1118_dn2))), (var_t5__blk1119_dn6 + (4.0 * (var_t3__blk1117_dn6 + var_t4__blk1118_dn6))), (var_t5__blk1119_dn7 + (4.0 * (var_t3__blk1117_dn7 + var_t4__blk1118_dn7))), (var_t5__blk1119_dn10 + (4.0 * (var_t3__blk1117_dn10 + var_t4__blk1118_dn10))), (var_t5__blk1119_dn11 + (4.0 * (var_t3__blk1117_dn11 + var_t4__blk1118_dn11))), (var_t5__blk1119_dn12 + (4.0 * (var_t3__blk1117_dn12 + var_t4__blk1118_dn12))), (var_t5__blk1119_dn17 + (4.0 * (var_t3__blk1117_dn17 + var_t4__blk1118_dn17))),)
    } else {
        (var_t5__blk1119, var_t5__blk1119_dn0, var_t5__blk1119_dn2, var_t5__blk1119_dn6, var_t5__blk1119_dn7, var_t5__blk1119_dn10, var_t5__blk1119_dn11, var_t5__blk1119_dn12, var_t5__blk1119_dn17,)
    }
};
        var_t5__blk1119 = assign33830_e49013;
        var_t5__blk1119_dn0 = assign33830_e49013_d_n0;
        var_t5__blk1119_dn2 = assign33830_e49013_d_n2;
        var_t5__blk1119_dn6 = assign33830_e49013_d_n6;
        var_t5__blk1119_dn7 = assign33830_e49013_d_n7;
        var_t5__blk1119_dn10 = assign33830_e49013_d_n10;
        var_t5__blk1119_dn11 = assign33830_e49013_d_n11;
        var_t5__blk1119_dn12 = assign33830_e49013_d_n12;
        var_t5__blk1119_dn17 = assign33830_e49013_d_n17;

        let (assign33840_e49027, assign33840_e49027_d_n0, assign33840_e49027_d_n2, assign33840_e49027_d_n6, assign33840_e49027_d_n7, assign33840_e49027_d_n10, assign33840_e49027_d_n11, assign33840_e49027_d_n12, assign33840_e49027_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33840_e49018: f64 = (20.0 * var_sqrtkusail);
        let assign33840_e49020: f64 = (assign33840_e49018 * var_vgvt);
        let assign33840_e49023: f64 = (var_kusai00 + var_kusail);
        let assign33840_e49024: f64 = (assign33840_e49020 * assign33840_e49023);
        let assign33840_e49025: f64 = (var_t5__blk1119 + assign33840_e49024);
        (assign33840_e49025, (var_t5__blk1119_dn0 + (((((20.0 * var_sqrtkusail_dn0) * var_vgvt) + (assign33840_e49018 * var_vgvt_dn0)) * assign33840_e49023) + (assign33840_e49020 * (var_kusai00_dn0 + var_kusail_dn0)))), (var_t5__blk1119_dn2 + (((((20.0 * var_sqrtkusail_dn2) * var_vgvt) + (assign33840_e49018 * var_vgvt_dn2)) * assign33840_e49023) + (assign33840_e49020 * (var_kusai00_dn2 + var_kusail_dn2)))), (var_t5__blk1119_dn6 + (((((20.0 * var_sqrtkusail_dn6) * var_vgvt) + (assign33840_e49018 * var_vgvt_dn6)) * assign33840_e49023) + (assign33840_e49020 * (var_kusai00_dn6 + var_kusail_dn6)))), (var_t5__blk1119_dn7 + (((((20.0 * var_sqrtkusail_dn7) * var_vgvt) + (assign33840_e49018 * var_vgvt_dn7)) * assign33840_e49023) + (assign33840_e49020 * (var_kusai00_dn7 + var_kusail_dn7)))), (var_t5__blk1119_dn10 + (((((20.0 * var_sqrtkusail_dn10) * var_vgvt) + (assign33840_e49018 * var_vgvt_dn10)) * assign33840_e49023) + (assign33840_e49020 * (var_kusai00_dn10 + var_kusail_dn10)))), (var_t5__blk1119_dn11 + (((((20.0 * var_sqrtkusail_dn11) * var_vgvt) + (assign33840_e49018 * var_vgvt_dn11)) * assign33840_e49023) + (assign33840_e49020 * (var_kusai00_dn11 + var_kusail_dn11)))), (var_t5__blk1119_dn12 + (((((20.0 * var_sqrtkusail_dn12) * var_vgvt) + (assign33840_e49018 * var_vgvt_dn12)) * assign33840_e49023) + (assign33840_e49020 * (var_kusai00_dn12 + var_kusail_dn12)))), (var_t5__blk1119_dn17 + (((((20.0 * var_sqrtkusail_dn17) * var_vgvt) + (assign33840_e49018 * var_vgvt_dn17)) * assign33840_e49023) + (assign33840_e49020 * (var_kusai00_dn17 + var_kusail_dn17)))),)
    } else {
        (var_t5__blk1119, var_t5__blk1119_dn0, var_t5__blk1119_dn2, var_t5__blk1119_dn6, var_t5__blk1119_dn7, var_t5__blk1119_dn10, var_t5__blk1119_dn11, var_t5__blk1119_dn12, var_t5__blk1119_dn17,)
    }
};
        var_t5__blk1119 = assign33840_e49027;
        var_t5__blk1119_dn0 = assign33840_e49027_d_n0;
        var_t5__blk1119_dn2 = assign33840_e49027_d_n2;
        var_t5__blk1119_dn6 = assign33840_e49027_d_n6;
        var_t5__blk1119_dn7 = assign33840_e49027_d_n7;
        var_t5__blk1119_dn10 = assign33840_e49027_d_n10;
        var_t5__blk1119_dn11 = assign33840_e49027_d_n11;
        var_t5__blk1119_dn12 = assign33840_e49027_d_n12;
        var_t5__blk1119_dn17 = assign33840_e49027_d_n17;

        let (assign33850_e49033, assign33850_e49033_d_n0, assign33850_e49033_d_n2, assign33850_e49033_d_n6, assign33850_e49033_d_n7, assign33850_e49033_d_n10, assign33850_e49033_d_n11, assign33850_e49033_d_n12, assign33850_e49033_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33850_e49031: f64 = (var_t2__blk1116 * var_t2__blk1116);
        (assign33850_e49031, ((var_t2__blk1116_dn0 * var_t2__blk1116) + (var_t2__blk1116 * var_t2__blk1116_dn0)), ((var_t2__blk1116_dn2 * var_t2__blk1116) + (var_t2__blk1116 * var_t2__blk1116_dn2)), ((var_t2__blk1116_dn6 * var_t2__blk1116) + (var_t2__blk1116 * var_t2__blk1116_dn6)), ((var_t2__blk1116_dn7 * var_t2__blk1116) + (var_t2__blk1116 * var_t2__blk1116_dn7)), ((var_t2__blk1116_dn10 * var_t2__blk1116) + (var_t2__blk1116 * var_t2__blk1116_dn10)), ((var_t2__blk1116_dn11 * var_t2__blk1116) + (var_t2__blk1116 * var_t2__blk1116_dn11)), ((var_t2__blk1116_dn12 * var_t2__blk1116) + (var_t2__blk1116 * var_t2__blk1116_dn12)), ((var_t2__blk1116_dn17 * var_t2__blk1116) + (var_t2__blk1116 * var_t2__blk1116_dn17)),)
    } else {
        (var_t10w, var_t10w_dn0, var_t10w_dn2, var_t10w_dn6, var_t10w_dn7, var_t10w_dn10, var_t10w_dn11, var_t10w_dn12, var_t10w_dn17,)
    }
};
        var_t10w = assign33850_e49033;
        var_t10w_dn0 = assign33850_e49033_d_n0;
        var_t10w_dn2 = assign33850_e49033_d_n2;
        var_t10w_dn6 = assign33850_e49033_d_n6;
        var_t10w_dn7 = assign33850_e49033_d_n7;
        var_t10w_dn10 = assign33850_e49033_d_n10;
        var_t10w_dn11 = assign33850_e49033_d_n11;
        var_t10w_dn12 = assign33850_e49033_d_n12;
        var_t10w_dn17 = assign33850_e49033_d_n17;

        let (assign33860_e49039, assign33860_e49039_d_n0, assign33860_e49039_d_n2, assign33860_e49039_d_n6, assign33860_e49039_d_n7, assign33860_e49039_d_n10, assign33860_e49039_d_n11, assign33860_e49039_d_n12, assign33860_e49039_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33860_e49037: f64 = (var_t10w * var_t10w);
        (assign33860_e49037, ((var_t10w_dn0 * var_t10w) + (var_t10w * var_t10w_dn0)), ((var_t10w_dn2 * var_t10w) + (var_t10w * var_t10w_dn2)), ((var_t10w_dn6 * var_t10w) + (var_t10w * var_t10w_dn6)), ((var_t10w_dn7 * var_t10w) + (var_t10w * var_t10w_dn7)), ((var_t10w_dn10 * var_t10w) + (var_t10w * var_t10w_dn10)), ((var_t10w_dn11 * var_t10w) + (var_t10w * var_t10w_dn11)), ((var_t10w_dn12 * var_t10w) + (var_t10w * var_t10w_dn12)), ((var_t10w_dn17 * var_t10w) + (var_t10w * var_t10w_dn17)),)
    } else {
        (var_t10__blk1112, var_t10__blk1112_dn0, var_t10__blk1112_dn2, var_t10__blk1112_dn6, var_t10__blk1112_dn7, var_t10__blk1112_dn10, var_t10__blk1112_dn11, var_t10__blk1112_dn12, var_t10__blk1112_dn17,)
    }
};
        var_t10__blk1112 = assign33860_e49039;
        var_t10__blk1112_dn0 = assign33860_e49039_d_n0;
        var_t10__blk1112_dn2 = assign33860_e49039_d_n2;
        var_t10__blk1112_dn6 = assign33860_e49039_d_n6;
        var_t10__blk1112_dn7 = assign33860_e49039_d_n7;
        var_t10__blk1112_dn10 = assign33860_e49039_d_n10;
        var_t10__blk1112_dn11 = assign33860_e49039_d_n11;
        var_t10__blk1112_dn12 = assign33860_e49039_d_n12;
        var_t10__blk1112_dn17 = assign33860_e49039_d_n17;

        let (assign33870_e49047, assign33870_e49047_d_n0, assign33870_e49047_d_n2, assign33870_e49047_d_n6, assign33870_e49047_d_n7, assign33870_e49047_d_n10, assign33870_e49047_d_n11, assign33870_e49047_d_n12, assign33870_e49047_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33870_e49044: f64 = (var_t10__blk1112 * var_t2__blk1116);
        let assign33870_e49045: f64 = (var_t5__blk1119 / assign33870_e49044);
        (assign33870_e49045, (((var_t5__blk1119_dn0 * assign33870_e49044) - (var_t5__blk1119 * ((var_t10__blk1112_dn0 * var_t2__blk1116) + (var_t10__blk1112 * var_t2__blk1116_dn0)))) / (assign33870_e49044 * assign33870_e49044)), (((var_t5__blk1119_dn2 * assign33870_e49044) - (var_t5__blk1119 * ((var_t10__blk1112_dn2 * var_t2__blk1116) + (var_t10__blk1112 * var_t2__blk1116_dn2)))) / (assign33870_e49044 * assign33870_e49044)), (((var_t5__blk1119_dn6 * assign33870_e49044) - (var_t5__blk1119 * ((var_t10__blk1112_dn6 * var_t2__blk1116) + (var_t10__blk1112 * var_t2__blk1116_dn6)))) / (assign33870_e49044 * assign33870_e49044)), (((var_t5__blk1119_dn7 * assign33870_e49044) - (var_t5__blk1119 * ((var_t10__blk1112_dn7 * var_t2__blk1116) + (var_t10__blk1112 * var_t2__blk1116_dn7)))) / (assign33870_e49044 * assign33870_e49044)), (((var_t5__blk1119_dn10 * assign33870_e49044) - (var_t5__blk1119 * ((var_t10__blk1112_dn10 * var_t2__blk1116) + (var_t10__blk1112 * var_t2__blk1116_dn10)))) / (assign33870_e49044 * assign33870_e49044)), (((var_t5__blk1119_dn11 * assign33870_e49044) - (var_t5__blk1119 * ((var_t10__blk1112_dn11 * var_t2__blk1116) + (var_t10__blk1112 * var_t2__blk1116_dn11)))) / (assign33870_e49044 * assign33870_e49044)), (((var_t5__blk1119_dn12 * assign33870_e49044) - (var_t5__blk1119 * ((var_t10__blk1112_dn12 * var_t2__blk1116) + (var_t10__blk1112 * var_t2__blk1116_dn12)))) / (assign33870_e49044 * assign33870_e49044)), (((var_t5__blk1119_dn17 * assign33870_e49044) - (var_t5__blk1119 * ((var_t10__blk1112_dn17 * var_t2__blk1116) + (var_t10__blk1112 * var_t2__blk1116_dn17)))) / (assign33870_e49044 * assign33870_e49044)),)
    } else {
        (var_kusai_ig, var_kusai_ig_dn0, var_kusai_ig_dn2, var_kusai_ig_dn6, var_kusai_ig_dn7, var_kusai_ig_dn10, var_kusai_ig_dn11, var_kusai_ig_dn12, var_kusai_ig_dn17,)
    }
};
        var_kusai_ig = assign33870_e49047;
        var_kusai_ig_dn0 = assign33870_e49047_d_n0;
        var_kusai_ig_dn2 = assign33870_e49047_d_n2;
        var_kusai_ig_dn6 = assign33870_e49047_d_n6;
        var_kusai_ig_dn7 = assign33870_e49047_d_n7;
        var_kusai_ig_dn10 = assign33870_e49047_d_n10;
        var_kusai_ig_dn11 = assign33870_e49047_d_n11;
        var_kusai_ig_dn12 = assign33870_e49047_d_n12;
        var_kusai_ig_dn17 = assign33870_e49047_d_n17;

        let (assign33880_e49057, assign33880_e49057_d_n0, assign33880_e49057_d_n2, assign33880_e49057_d_n6, assign33880_e49057_d_n7, assign33880_e49057_d_n10, assign33880_e49057_d_n11, assign33880_e49057_d_n12, assign33880_e49057_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33880_e49051: f64 = (var_weff_nf / var_lch);
        let assign33880_e49053: f64 = (assign33880_e49051 * var_mu);
        let assign33880_e49055: f64 = (assign33880_e49053 * var_c_fox);
        (assign33880_e49055, (((((-((var_weff_nf * var_lch_dn0) / (var_lch * var_lch))) * var_mu) + (assign33880_e49051 * var_mu_dn0)) * var_c_fox) + (assign33880_e49053 * var_c_fox_dn0)), (((((-((var_weff_nf * var_lch_dn2) / (var_lch * var_lch))) * var_mu) + (assign33880_e49051 * var_mu_dn2)) * var_c_fox) + (assign33880_e49053 * var_c_fox_dn2)), (((((-((var_weff_nf * var_lch_dn6) / (var_lch * var_lch))) * var_mu) + (assign33880_e49051 * var_mu_dn6)) * var_c_fox) + (assign33880_e49053 * var_c_fox_dn6)), (((((-((var_weff_nf * var_lch_dn7) / (var_lch * var_lch))) * var_mu) + (assign33880_e49051 * var_mu_dn7)) * var_c_fox) + (assign33880_e49053 * var_c_fox_dn7)), (((((-((var_weff_nf * var_lch_dn10) / (var_lch * var_lch))) * var_mu) + (assign33880_e49051 * var_mu_dn10)) * var_c_fox) + (assign33880_e49053 * var_c_fox_dn10)), (((((-((var_weff_nf * var_lch_dn11) / (var_lch * var_lch))) * var_mu) + (assign33880_e49051 * var_mu_dn11)) * var_c_fox) + (assign33880_e49053 * var_c_fox_dn11)), (((((-((var_weff_nf * var_lch_dn12) / (var_lch * var_lch))) * var_mu) + (assign33880_e49051 * var_mu_dn12)) * var_c_fox) + (assign33880_e49053 * var_c_fox_dn12)), (((((-((var_weff_nf * var_lch_dn17) / (var_lch * var_lch))) * var_mu) + (assign33880_e49051 * var_mu_dn17)) * var_c_fox) + (assign33880_e49053 * var_c_fox_dn17)),)
    } else {
        (var_gds0_ign, var_gds0_ign_dn0, var_gds0_ign_dn2, var_gds0_ign_dn6, var_gds0_ign_dn7, var_gds0_ign_dn10, var_gds0_ign_dn11, var_gds0_ign_dn12, var_gds0_ign_dn17,)
    }
};
        var_gds0_ign = assign33880_e49057;
        var_gds0_ign_dn0 = assign33880_e49057_d_n0;
        var_gds0_ign_dn2 = assign33880_e49057_d_n2;
        var_gds0_ign_dn6 = assign33880_e49057_d_n6;
        var_gds0_ign_dn7 = assign33880_e49057_d_n7;
        var_gds0_ign_dn10 = assign33880_e49057_d_n10;
        var_gds0_ign_dn11 = assign33880_e49057_d_n11;
        var_gds0_ign_dn12 = assign33880_e49057_d_n12;
        var_gds0_ign_dn17 = assign33880_e49057_d_n17;

        let (assign33890_e49063, assign33890_e49063_d_n0, assign33890_e49063_d_n2, assign33890_e49063_d_n6, assign33890_e49063_d_n7, assign33890_e49063_d_n10, assign33890_e49063_d_n11, assign33890_e49063_d_n12, assign33890_e49063_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33890_e49061: f64 = (var_gds0_ign * var_vgvt);
        (assign33890_e49061, ((var_gds0_ign_dn0 * var_vgvt) + (var_gds0_ign * var_vgvt_dn0)), ((var_gds0_ign_dn2 * var_vgvt) + (var_gds0_ign * var_vgvt_dn2)), ((var_gds0_ign_dn6 * var_vgvt) + (var_gds0_ign * var_vgvt_dn6)), ((var_gds0_ign_dn7 * var_vgvt) + (var_gds0_ign * var_vgvt_dn7)), ((var_gds0_ign_dn10 * var_vgvt) + (var_gds0_ign * var_vgvt_dn10)), ((var_gds0_ign_dn11 * var_vgvt) + (var_gds0_ign * var_vgvt_dn11)), ((var_gds0_ign_dn12 * var_vgvt) + (var_gds0_ign * var_vgvt_dn12)), ((var_gds0_ign_dn17 * var_vgvt) + (var_gds0_ign * var_vgvt_dn17)),)
    } else {
        (var_gds0_h2, var_gds0_h2_dn0, var_gds0_h2_dn2, var_gds0_h2_dn6, var_gds0_h2_dn7, var_gds0_h2_dn10, var_gds0_h2_dn11, var_gds0_h2_dn12, var_gds0_h2_dn17,)
    }
};
        var_gds0_h2 = assign33890_e49063;
        var_gds0_h2_dn0 = assign33890_e49063_d_n0;
        var_gds0_h2_dn2 = assign33890_e49063_d_n2;
        var_gds0_h2_dn6 = assign33890_e49063_d_n6;
        var_gds0_h2_dn7 = assign33890_e49063_d_n7;
        var_gds0_h2_dn10 = assign33890_e49063_d_n10;
        var_gds0_h2_dn11 = assign33890_e49063_d_n11;
        var_gds0_h2_dn12 = assign33890_e49063_d_n12;
        var_gds0_h2_dn17 = assign33890_e49063_d_n17;

        let (assign33900_e49069, assign33900_e49069_d_n0, assign33900_e49069_d_n2, assign33900_e49069_d_n6, assign33900_e49069_d_n7, assign33900_e49069_d_n10, assign33900_e49069_d_n11, assign33900_e49069_d_n12, assign33900_e49069_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33900_e49067: f64 = (var_nthrml / var_gds0_h2);
        (assign33900_e49067, (((var_nthrml_dn0 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn0)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn2 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn2)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn6 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn6)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn7 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn7)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn10 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn10)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn11 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn11)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn12 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn12)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn17 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn17)) / (var_gds0_h2 * var_gds0_h2)),)
    } else {
        (var_gamma, var_gamma_dn0, var_gamma_dn2, var_gamma_dn6, var_gamma_dn7, var_gamma_dn10, var_gamma_dn11, var_gamma_dn12, var_gamma_dn17,)
    }
};
        var_gamma = assign33900_e49069;
        var_gamma_dn0 = assign33900_e49069_d_n0;
        var_gamma_dn2 = assign33900_e49069_d_n2;
        var_gamma_dn6 = assign33900_e49069_d_n6;
        var_gamma_dn7 = assign33900_e49069_d_n7;
        var_gamma_dn10 = assign33900_e49069_d_n10;
        var_gamma_dn11 = assign33900_e49069_d_n11;
        var_gamma_dn12 = assign33900_e49069_d_n12;
        var_gamma_dn17 = assign33900_e49069_d_n17;

        let (assign33910_e49081, assign33910_e49081_d_n0, assign33910_e49081_d_n2, assign33910_e49081_d_n6, assign33910_e49081_d_n7, assign33910_e49081_d_n10, assign33910_e49081_d_n11, assign33910_e49081_d_n12, assign33910_e49081_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33910_e49074: f64 = (4.0 * var_vgvt);
        let assign33910_e49076: f64 = (assign33910_e49074 * var_sqrtkusail);
        let assign33910_e49077: f64 = (var_kusai00 + assign33910_e49076);
        let assign33910_e49079: f64 = (assign33910_e49077 + var_kusail);
        (assign33910_e49079, ((var_kusai00_dn0 + (((4.0 * var_vgvt_dn0) * var_sqrtkusail) + (assign33910_e49074 * var_sqrtkusail_dn0))) + var_kusail_dn0), ((var_kusai00_dn2 + (((4.0 * var_vgvt_dn2) * var_sqrtkusail) + (assign33910_e49074 * var_sqrtkusail_dn2))) + var_kusail_dn2), ((var_kusai00_dn6 + (((4.0 * var_vgvt_dn6) * var_sqrtkusail) + (assign33910_e49074 * var_sqrtkusail_dn6))) + var_kusail_dn6), ((var_kusai00_dn7 + (((4.0 * var_vgvt_dn7) * var_sqrtkusail) + (assign33910_e49074 * var_sqrtkusail_dn7))) + var_kusail_dn7), ((var_kusai00_dn10 + (((4.0 * var_vgvt_dn10) * var_sqrtkusail) + (assign33910_e49074 * var_sqrtkusail_dn10))) + var_kusail_dn10), ((var_kusai00_dn11 + (((4.0 * var_vgvt_dn11) * var_sqrtkusail) + (assign33910_e49074 * var_sqrtkusail_dn11))) + var_kusail_dn11), ((var_kusai00_dn12 + (((4.0 * var_vgvt_dn12) * var_sqrtkusail) + (assign33910_e49074 * var_sqrtkusail_dn12))) + var_kusail_dn12), ((var_kusai00_dn17 + (((4.0 * var_vgvt_dn17) * var_sqrtkusail) + (assign33910_e49074 * var_sqrtkusail_dn17))) + var_kusail_dn17),)
    } else {
        (var_t7w, var_t7w_dn0, var_t7w_dn2, var_t7w_dn6, var_t7w_dn7, var_t7w_dn10, var_t7w_dn11, var_t7w_dn12, var_t7w_dn17,)
    }
};
        var_t7w = assign33910_e49081;
        var_t7w_dn0 = assign33910_e49081_d_n0;
        var_t7w_dn2 = assign33910_e49081_d_n2;
        var_t7w_dn6 = assign33910_e49081_d_n6;
        var_t7w_dn7 = assign33910_e49081_d_n7;
        var_t7w_dn10 = assign33910_e49081_d_n10;
        var_t7w_dn11 = assign33910_e49081_d_n11;
        var_t7w_dn12 = assign33910_e49081_d_n12;
        var_t7w_dn17 = assign33910_e49081_d_n17;

        let (assign33920_e49102, assign33920_e49102_d_n0, assign33920_e49102_d_n2, assign33920_e49102_d_n6, assign33920_e49102_d_n7, assign33920_e49102_d_n10, assign33920_e49102_d_n11, assign33920_e49102_d_n12, assign33920_e49102_d_n17,) = {
    if (var_guard1127 != 0.0) {
        let assign33920_e49085: f64 = (3.872983346207417 * var_kusai00l);
        let assign33920_e49087: f64 = (assign33920_e49085 * var_t7w);
        let assign33920_e49090: f64 = (6.0 * var_t2__blk1116);
        let assign33920_e49093: f64 = (var_gamma * var_t2__blk1116);
        let assign33920_e49095: f64 = (assign33920_e49093 * var_vgvt);
        let assign33920_e49097: f64 = (assign33920_e49095 * var_t5__blk1119);
        let assign33920_e49098: f64 = (assign33920_e49097).sqrt();
        let assign33920_e49099: f64 = (assign33920_e49090 * assign33920_e49098);
        let assign33920_e49100: f64 = (assign33920_e49087 / assign33920_e49099);
        (assign33920_e49100, ((((((3.872983346207417 * var_kusai00l_dn0) * var_t7w) + (assign33920_e49085 * var_t7w_dn0)) * assign33920_e49099) - (assign33920_e49087 * (((6.0 * var_t2__blk1116_dn0) * assign33920_e49098) + (assign33920_e49090 * (((((((var_gamma_dn0 * var_t2__blk1116) + (var_gamma * var_t2__blk1116_dn0)) * var_vgvt) + (assign33920_e49093 * var_vgvt_dn0)) * var_t5__blk1119) + (assign33920_e49095 * var_t5__blk1119_dn0)) / (2.0 * assign33920_e49098)))))) / (assign33920_e49099 * assign33920_e49099)), ((((((3.872983346207417 * var_kusai00l_dn2) * var_t7w) + (assign33920_e49085 * var_t7w_dn2)) * assign33920_e49099) - (assign33920_e49087 * (((6.0 * var_t2__blk1116_dn2) * assign33920_e49098) + (assign33920_e49090 * (((((((var_gamma_dn2 * var_t2__blk1116) + (var_gamma * var_t2__blk1116_dn2)) * var_vgvt) + (assign33920_e49093 * var_vgvt_dn2)) * var_t5__blk1119) + (assign33920_e49095 * var_t5__blk1119_dn2)) / (2.0 * assign33920_e49098)))))) / (assign33920_e49099 * assign33920_e49099)), ((((((3.872983346207417 * var_kusai00l_dn6) * var_t7w) + (assign33920_e49085 * var_t7w_dn6)) * assign33920_e49099) - (assign33920_e49087 * (((6.0 * var_t2__blk1116_dn6) * assign33920_e49098) + (assign33920_e49090 * (((((((var_gamma_dn6 * var_t2__blk1116) + (var_gamma * var_t2__blk1116_dn6)) * var_vgvt) + (assign33920_e49093 * var_vgvt_dn6)) * var_t5__blk1119) + (assign33920_e49095 * var_t5__blk1119_dn6)) / (2.0 * assign33920_e49098)))))) / (assign33920_e49099 * assign33920_e49099)), ((((((3.872983346207417 * var_kusai00l_dn7) * var_t7w) + (assign33920_e49085 * var_t7w_dn7)) * assign33920_e49099) - (assign33920_e49087 * (((6.0 * var_t2__blk1116_dn7) * assign33920_e49098) + (assign33920_e49090 * (((((((var_gamma_dn7 * var_t2__blk1116) + (var_gamma * var_t2__blk1116_dn7)) * var_vgvt) + (assign33920_e49093 * var_vgvt_dn7)) * var_t5__blk1119) + (assign33920_e49095 * var_t5__blk1119_dn7)) / (2.0 * assign33920_e49098)))))) / (assign33920_e49099 * assign33920_e49099)), ((((((3.872983346207417 * var_kusai00l_dn10) * var_t7w) + (assign33920_e49085 * var_t7w_dn10)) * assign33920_e49099) - (assign33920_e49087 * (((6.0 * var_t2__blk1116_dn10) * assign33920_e49098) + (assign33920_e49090 * (((((((var_gamma_dn10 * var_t2__blk1116) + (var_gamma * var_t2__blk1116_dn10)) * var_vgvt) + (assign33920_e49093 * var_vgvt_dn10)) * var_t5__blk1119) + (assign33920_e49095 * var_t5__blk1119_dn10)) / (2.0 * assign33920_e49098)))))) / (assign33920_e49099 * assign33920_e49099)), ((((((3.872983346207417 * var_kusai00l_dn11) * var_t7w) + (assign33920_e49085 * var_t7w_dn11)) * assign33920_e49099) - (assign33920_e49087 * (((6.0 * var_t2__blk1116_dn11) * assign33920_e49098) + (assign33920_e49090 * (((((((var_gamma_dn11 * var_t2__blk1116) + (var_gamma * var_t2__blk1116_dn11)) * var_vgvt) + (assign33920_e49093 * var_vgvt_dn11)) * var_t5__blk1119) + (assign33920_e49095 * var_t5__blk1119_dn11)) / (2.0 * assign33920_e49098)))))) / (assign33920_e49099 * assign33920_e49099)), ((((((3.872983346207417 * var_kusai00l_dn12) * var_t7w) + (assign33920_e49085 * var_t7w_dn12)) * assign33920_e49099) - (assign33920_e49087 * (((6.0 * var_t2__blk1116_dn12) * assign33920_e49098) + (assign33920_e49090 * (((((((var_gamma_dn12 * var_t2__blk1116) + (var_gamma * var_t2__blk1116_dn12)) * var_vgvt) + (assign33920_e49093 * var_vgvt_dn12)) * var_t5__blk1119) + (assign33920_e49095 * var_t5__blk1119_dn12)) / (2.0 * assign33920_e49098)))))) / (assign33920_e49099 * assign33920_e49099)), ((((((3.872983346207417 * var_kusai00l_dn17) * var_t7w) + (assign33920_e49085 * var_t7w_dn17)) * assign33920_e49099) - (assign33920_e49087 * (((6.0 * var_t2__blk1116_dn17) * assign33920_e49098) + (assign33920_e49090 * (((((((var_gamma_dn17 * var_t2__blk1116) + (var_gamma * var_t2__blk1116_dn17)) * var_vgvt) + (assign33920_e49093 * var_vgvt_dn17)) * var_t5__blk1119) + (assign33920_e49095 * var_t5__blk1119_dn17)) / (2.0 * assign33920_e49098)))))) / (assign33920_e49099 * assign33920_e49099)),)
    } else {
        (var_crl_f, var_crl_f_dn0, var_crl_f_dn2, var_crl_f_dn6, var_crl_f_dn7, var_crl_f_dn10, var_crl_f_dn11, var_crl_f_dn12, var_crl_f_dn17,)
    }
};
        var_crl_f = assign33920_e49102;
        var_crl_f_dn0 = assign33920_e49102_d_n0;
        var_crl_f_dn2 = assign33920_e49102_d_n2;
        var_crl_f_dn6 = assign33920_e49102_d_n6;
        var_crl_f_dn7 = assign33920_e49102_d_n7;
        var_crl_f_dn10 = assign33920_e49102_d_n10;
        var_crl_f_dn11 = assign33920_e49102_d_n11;
        var_crl_f_dn12 = assign33920_e49102_d_n12;
        var_crl_f_dn17 = assign33920_e49102_d_n17;

        let assign33930_e49105: f64 = (var_ids + var_idsibpc);
        var_ids = assign33930_e49105;
        var_ids_dn0 = (var_ids_dn0 + var_idsibpc_dn0);
        var_ids_dn2 = (var_ids_dn2 + var_idsibpc_dn2);
        var_ids_dn6 = (var_ids_dn6 + var_idsibpc_dn6);
        var_ids_dn7 = (var_ids_dn7 + var_idsibpc_dn7);
        var_ids_dn10 = (var_ids_dn10 + var_idsibpc_dn10);
        var_ids_dn11 = (var_ids_dn11 + var_idsibpc_dn11);
        var_ids_dn12 = (var_ids_dn12 + var_idsibpc_dn12);
        var_ids_dn17 = (var_ids_dn17 + var_idsibpc_dn17);

        let assign33940_e49108: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1128 = assign33940_e49108;

        let (assign33950_e49114,) = {
    if (var_guard1128 != 0.0) {
        let assign33950_e49112: f64 = (var_cbtp + var_cbtn);
        (assign33950_e49112,)
    } else {
        (var_cgbe,)
    }
};
        var_cgbe = assign33950_e49114;

        let (assign33960_e49124,) = {
    if ((var_guard1128 != 0.0) && (var_cgbo_given != 0.0)) {
        let assign33960_e49121: f64 = (p.p168 * var_lgleff);
        let assign33960_e49122: f64 = (var_cgbe - assign33960_e49121);
        (assign33960_e49122,)
    } else {
        (var_cgbe,)
    }
};
        var_cgbe = assign33960_e49124;

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
        *var_guard1127_slot = var_guard1127;
        *var_guard1128_slot = var_guard1128;
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
        *var_t0__blk1114_slot = var_t0__blk1114;
        *var_t0__blk1114_dn0_slot = var_t0__blk1114_dn0;
        *var_t0__blk1114_dn10_slot = var_t0__blk1114_dn10;
        *var_t0__blk1114_dn11_slot = var_t0__blk1114_dn11;
        *var_t0__blk1114_dn12_slot = var_t0__blk1114_dn12;
        *var_t0__blk1114_dn17_slot = var_t0__blk1114_dn17;
        *var_t0__blk1114_dn2_slot = var_t0__blk1114_dn2;
        *var_t0__blk1114_dn6_slot = var_t0__blk1114_dn6;
        *var_t0__blk1114_dn7_slot = var_t0__blk1114_dn7;
        *var_t10__blk1112_slot = var_t10__blk1112;
        *var_t10__blk1112_dn0_slot = var_t10__blk1112_dn0;
        *var_t10__blk1112_dn10_slot = var_t10__blk1112_dn10;
        *var_t10__blk1112_dn11_slot = var_t10__blk1112_dn11;
        *var_t10__blk1112_dn12_slot = var_t10__blk1112_dn12;
        *var_t10__blk1112_dn17_slot = var_t10__blk1112_dn17;
        *var_t10__blk1112_dn2_slot = var_t10__blk1112_dn2;
        *var_t10__blk1112_dn6_slot = var_t10__blk1112_dn6;
        *var_t10__blk1112_dn7_slot = var_t10__blk1112_dn7;
        *var_t10w_slot = var_t10w;
        *var_t10w_dn0_slot = var_t10w_dn0;
        *var_t10w_dn10_slot = var_t10w_dn10;
        *var_t10w_dn11_slot = var_t10w_dn11;
        *var_t10w_dn12_slot = var_t10w_dn12;
        *var_t10w_dn17_slot = var_t10w_dn17;
        *var_t10w_dn2_slot = var_t10w_dn2;
        *var_t10w_dn6_slot = var_t10w_dn6;
        *var_t10w_dn7_slot = var_t10w_dn7;
        *var_t11__blk1113_slot = var_t11__blk1113;
        *var_t11__blk1113_dn0_slot = var_t11__blk1113_dn0;
        *var_t11__blk1113_dn10_slot = var_t11__blk1113_dn10;
        *var_t11__blk1113_dn11_slot = var_t11__blk1113_dn11;
        *var_t11__blk1113_dn12_slot = var_t11__blk1113_dn12;
        *var_t11__blk1113_dn17_slot = var_t11__blk1113_dn17;
        *var_t11__blk1113_dn2_slot = var_t11__blk1113_dn2;
        *var_t11__blk1113_dn6_slot = var_t11__blk1113_dn6;
        *var_t11__blk1113_dn7_slot = var_t11__blk1113_dn7;
        *var_t2__blk1116_slot = var_t2__blk1116;
        *var_t2__blk1116_dn0_slot = var_t2__blk1116_dn0;
        *var_t2__blk1116_dn10_slot = var_t2__blk1116_dn10;
        *var_t2__blk1116_dn11_slot = var_t2__blk1116_dn11;
        *var_t2__blk1116_dn12_slot = var_t2__blk1116_dn12;
        *var_t2__blk1116_dn17_slot = var_t2__blk1116_dn17;
        *var_t2__blk1116_dn2_slot = var_t2__blk1116_dn2;
        *var_t2__blk1116_dn6_slot = var_t2__blk1116_dn6;
        *var_t2__blk1116_dn7_slot = var_t2__blk1116_dn7;
        *var_t3__blk1117_slot = var_t3__blk1117;
        *var_t3__blk1117_dn0_slot = var_t3__blk1117_dn0;
        *var_t3__blk1117_dn10_slot = var_t3__blk1117_dn10;
        *var_t3__blk1117_dn11_slot = var_t3__blk1117_dn11;
        *var_t3__blk1117_dn12_slot = var_t3__blk1117_dn12;
        *var_t3__blk1117_dn17_slot = var_t3__blk1117_dn17;
        *var_t3__blk1117_dn2_slot = var_t3__blk1117_dn2;
        *var_t3__blk1117_dn6_slot = var_t3__blk1117_dn6;
        *var_t3__blk1117_dn7_slot = var_t3__blk1117_dn7;
        *var_t4__blk1118_slot = var_t4__blk1118;
        *var_t4__blk1118_dn0_slot = var_t4__blk1118_dn0;
        *var_t4__blk1118_dn10_slot = var_t4__blk1118_dn10;
        *var_t4__blk1118_dn11_slot = var_t4__blk1118_dn11;
        *var_t4__blk1118_dn12_slot = var_t4__blk1118_dn12;
        *var_t4__blk1118_dn17_slot = var_t4__blk1118_dn17;
        *var_t4__blk1118_dn2_slot = var_t4__blk1118_dn2;
        *var_t4__blk1118_dn6_slot = var_t4__blk1118_dn6;
        *var_t4__blk1118_dn7_slot = var_t4__blk1118_dn7;
        *var_t5__blk1119_slot = var_t5__blk1119;
        *var_t5__blk1119_dn0_slot = var_t5__blk1119_dn0;
        *var_t5__blk1119_dn10_slot = var_t5__blk1119_dn10;
        *var_t5__blk1119_dn11_slot = var_t5__blk1119_dn11;
        *var_t5__blk1119_dn12_slot = var_t5__blk1119_dn12;
        *var_t5__blk1119_dn17_slot = var_t5__blk1119_dn17;
        *var_t5__blk1119_dn2_slot = var_t5__blk1119_dn2;
        *var_t5__blk1119_dn6_slot = var_t5__blk1119_dn6;
        *var_t5__blk1119_dn7_slot = var_t5__blk1119_dn7;
        *var_t7w_slot = var_t7w;
        *var_t7w_dn0_slot = var_t7w_dn0;
        *var_t7w_dn10_slot = var_t7w_dn10;
        *var_t7w_dn11_slot = var_t7w_dn11;
        *var_t7w_dn12_slot = var_t7w_dn12;
        *var_t7w_dn17_slot = var_t7w_dn17;
        *var_t7w_dn2_slot = var_t7w_dn2;
        *var_t7w_dn6_slot = var_t7w_dn6;
        *var_t7w_dn7_slot = var_t7w_dn7;
        *var_t9__blk1111_slot = var_t9__blk1111;
        *var_t9__blk1111_dn0_slot = var_t9__blk1111_dn0;
        *var_t9__blk1111_dn10_slot = var_t9__blk1111_dn10;
        *var_t9__blk1111_dn11_slot = var_t9__blk1111_dn11;
        *var_t9__blk1111_dn12_slot = var_t9__blk1111_dn12;
        *var_t9__blk1111_dn17_slot = var_t9__blk1111_dn17;
        *var_t9__blk1111_dn2_slot = var_t9__blk1111_dn2;
        *var_t9__blk1111_dn6_slot = var_t9__blk1111_dn6;
        *var_t9__blk1111_dn7_slot = var_t9__blk1111_dn7;
    }

    pub(super) fn stamp_transient_block_119(
        p: &Parameters,
        var_cgbo_given: f64,
        var_flg_nqs: f64,
        var_guard1128: f64,
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
        var_guard1129_slot: &mut f64,
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
        let mut var_guard1129: f64 = *var_guard1129_slot;
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

        let (assign33970_e49133, assign33970_e49133_d_n0, assign33970_e49133_d_n2, assign33970_e49133_d_n6, assign33970_e49133_d_n7, assign33970_e49133_d_n10, assign33970_e49133_d_n11, assign33970_e49133_d_n12, assign33970_e49133_d_n17,) = {
    if (var_guard1128 != 0.0) {
        let assign33970_e49127: f64 = (-var_cgbe);
        let assign33970_e49130: f64 = (var_vgs - var_vbsp);
        let assign33970_e49131: f64 = (assign33970_e49127 * assign33970_e49130);
        (assign33970_e49131, (assign33970_e49127 * (-var_vbsp_dn0)), (assign33970_e49127 * (-var_vbsp_dn2)), (assign33970_e49127 * (var_vgs_dn6 - var_vbsp_dn6)), (assign33970_e49127 * (var_vgs_dn7 - var_vbsp_dn7)), (assign33970_e49127 * (-var_vbsp_dn10)), (assign33970_e49127 * (var_vgs_dn11 - var_vbsp_dn11)), (assign33970_e49127 * (-var_vbsp_dn12)), (assign33970_e49127 * (-var_vbsp_dn17)),)
    } else {
        (var_qgob, var_qgob_dn0, var_qgob_dn2, var_qgob_dn6, var_qgob_dn7, var_qgob_dn10, var_qgob_dn11, var_qgob_dn12, var_qgob_dn17,)
    }
};
        var_qgob = assign33970_e49133;
        var_qgob_dn0 = assign33970_e49133_d_n0;
        var_qgob_dn2 = assign33970_e49133_d_n2;
        var_qgob_dn6 = assign33970_e49133_d_n6;
        var_qgob_dn7 = assign33970_e49133_d_n7;
        var_qgob_dn10 = assign33970_e49133_d_n10;
        var_qgob_dn11 = assign33970_e49133_d_n11;
        var_qgob_dn12 = assign33970_e49133_d_n12;
        var_qgob_dn17 = assign33970_e49133_d_n17;

        let (assign33980_e49143,) = {
    if (var_guard1128 != 0.0) {
        (0.0,)
    } else {
        (var_cfu,)
    }
};
        var_cfu = assign33980_e49143;

        let (assign33990_e49153,) = {
    if (var_guard1128 != 0.0) {
        let assign33990_e49147: f64 = (var_cfu * p.p9);
        let assign33990_e49150: f64 = (var_wgate + var_uc_pdbcp);
        let assign33990_e49151: f64 = (assign33990_e49147 * assign33990_e49150);
        (assign33990_e49151,)
    } else {
        (var_cfd,)
    }
};
        var_cfd = assign33990_e49153;

        let (assign34000_e49163,) = {
    if (var_guard1128 != 0.0) {
        let assign34000_e49157: f64 = (var_cfu * p.p9);
        let assign34000_e49160: f64 = (var_wgate + var_uc_psbcp);
        let assign34000_e49161: f64 = (assign34000_e49157 * assign34000_e49160);
        (assign34000_e49161,)
    } else {
        (var_cfs,)
    }
};
        var_cfs = assign34000_e49163;

        let (assign34010_e49171, assign34010_e49171_d_n0, assign34010_e49171_d_n2, assign34010_e49171_d_n6, assign34010_e49171_d_n7, assign34010_e49171_d_n10, assign34010_e49171_d_n11, assign34010_e49171_d_n12, assign34010_e49171_d_n17,) = {
    if (var_guard1128 != 0.0) {
        let assign34010_e49168: f64 = (var_vgs - var_vds);
        let assign34010_e49169: f64 = (var_cfd * assign34010_e49168);
        (assign34010_e49169, (var_cfd * (-var_vds_dn0)), (var_cfd * (-var_vds_dn2)), (var_cfd * (var_vgs_dn6 - var_vds_dn6)), (var_cfd * (var_vgs_dn7 - var_vds_dn7)), (var_cfd * (-var_vds_dn10)), (var_cfd * (var_vgs_dn11 - var_vds_dn11)), (var_cfd * (-var_vds_dn12)), (var_cfd * (-var_vds_dn17)),)
    } else {
        (var_qfd, var_qfd_dn0, var_qfd_dn2, var_qfd_dn6, var_qfd_dn7, var_qfd_dn10, var_qfd_dn11, var_qfd_dn12, var_qfd_dn17,)
    }
};
        var_qfd = assign34010_e49171;
        var_qfd_dn0 = assign34010_e49171_d_n0;
        var_qfd_dn2 = assign34010_e49171_d_n2;
        var_qfd_dn6 = assign34010_e49171_d_n6;
        var_qfd_dn7 = assign34010_e49171_d_n7;
        var_qfd_dn10 = assign34010_e49171_d_n10;
        var_qfd_dn11 = assign34010_e49171_d_n11;
        var_qfd_dn12 = assign34010_e49171_d_n12;
        var_qfd_dn17 = assign34010_e49171_d_n17;

        let (assign34020_e49177, assign34020_e49177_d_n6, assign34020_e49177_d_n7, assign34020_e49177_d_n11,) = {
    if (var_guard1128 != 0.0) {
        let assign34020_e49175: f64 = (var_cfs * var_vgs);
        (assign34020_e49175, (var_cfs * var_vgs_dn6), (var_cfs * var_vgs_dn7), (var_cfs * var_vgs_dn11),)
    } else {
        (var_qfs, var_qfs_dn6, var_qfs_dn7, var_qfs_dn11,)
    }
};
        var_qfs = assign34020_e49177;
        var_qfs_dn6 = assign34020_e49177_d_n6;
        var_qfs_dn7 = assign34020_e49177_d_n7;
        var_qfs_dn11 = assign34020_e49177_d_n11;

        let (assign34030_e49189, assign34030_e49189_d_n0, assign34030_e49189_d_n2, assign34030_e49189_d_n6, assign34030_e49189_d_n7, assign34030_e49189_d_n10, assign34030_e49189_d_n11, assign34030_e49189_d_n12, assign34030_e49189_d_n17,) = {
    if (var_guard1128 != 0.0) {
        let assign34030_e49181: f64 = (var_cfu * p.p19);
        let assign34030_e49183: f64 = (assign34030_e49181 * p.p9);
        let assign34030_e49186: f64 = (var_vgs - var_vbsp);
        let assign34030_e49187: f64 = (assign34030_e49183 * assign34030_e49186);
        (assign34030_e49187, (assign34030_e49183 * (-var_vbsp_dn0)), (assign34030_e49183 * (-var_vbsp_dn2)), (assign34030_e49183 * (var_vgs_dn6 - var_vbsp_dn6)), (assign34030_e49183 * (var_vgs_dn7 - var_vbsp_dn7)), (assign34030_e49183 * (-var_vbsp_dn10)), (assign34030_e49183 * (var_vgs_dn11 - var_vbsp_dn11)), (assign34030_e49183 * (-var_vbsp_dn12)), (assign34030_e49183 * (-var_vbsp_dn17)),)
    } else {
        (var_qfbc, var_qfbc_dn0, var_qfbc_dn2, var_qfbc_dn6, var_qfbc_dn7, var_qfbc_dn10, var_qfbc_dn11, var_qfbc_dn12, var_qfbc_dn17,)
    }
};
        var_qfbc = assign34030_e49189;
        var_qfbc_dn0 = assign34030_e49189_d_n0;
        var_qfbc_dn2 = assign34030_e49189_d_n2;
        var_qfbc_dn6 = assign34030_e49189_d_n6;
        var_qfbc_dn7 = assign34030_e49189_d_n7;
        var_qfbc_dn10 = assign34030_e49189_d_n10;
        var_qfbc_dn11 = assign34030_e49189_d_n11;
        var_qfbc_dn12 = assign34030_e49189_d_n12;
        var_qfbc_dn17 = assign34030_e49189_d_n17;

        let (assign34040_e49195, assign34040_e49195_d_n0, assign34040_e49195_d_n2, assign34040_e49195_d_n6, assign34040_e49195_d_n7, assign34040_e49195_d_n10, assign34040_e49195_d_n11, assign34040_e49195_d_n12, assign34040_e49195_d_n17,) = {
    if (var_guard1128 != 0.0) {
        let assign34040_e49193: f64 = (var_qgod + var_qfd);
        (assign34040_e49193, (var_qgod_dn0 + var_qfd_dn0), (var_qgod_dn2 + var_qfd_dn2), (var_qgod_dn6 + var_qfd_dn6), (var_qgod_dn7 + var_qfd_dn7), (var_qgod_dn10 + var_qfd_dn10), (var_qgod_dn11 + var_qfd_dn11), (var_qgod_dn12 + var_qfd_dn12), (var_qgod_dn17 + var_qfd_dn17),)
    } else {
        (var_qgod, var_qgod_dn0, var_qgod_dn2, var_qgod_dn6, var_qgod_dn7, var_qgod_dn10, var_qgod_dn11, var_qgod_dn12, var_qgod_dn17,)
    }
};
        var_qgod = assign34040_e49195;
        var_qgod_dn0 = assign34040_e49195_d_n0;
        var_qgod_dn2 = assign34040_e49195_d_n2;
        var_qgod_dn6 = assign34040_e49195_d_n6;
        var_qgod_dn7 = assign34040_e49195_d_n7;
        var_qgod_dn10 = assign34040_e49195_d_n10;
        var_qgod_dn11 = assign34040_e49195_d_n11;
        var_qgod_dn12 = assign34040_e49195_d_n12;
        var_qgod_dn17 = assign34040_e49195_d_n17;

        let (assign34050_e49201, assign34050_e49201_d_n0, assign34050_e49201_d_n2, assign34050_e49201_d_n6, assign34050_e49201_d_n7, assign34050_e49201_d_n10, assign34050_e49201_d_n11, assign34050_e49201_d_n12, assign34050_e49201_d_n17,) = {
    if (var_guard1128 != 0.0) {
        let assign34050_e49199: f64 = (var_qgos + var_qfs);
        (assign34050_e49199, var_qgos_dn0, var_qgos_dn2, (var_qgos_dn6 + var_qfs_dn6), (var_qgos_dn7 + var_qfs_dn7), var_qgos_dn10, (var_qgos_dn11 + var_qfs_dn11), var_qgos_dn12, var_qgos_dn17,)
    } else {
        (var_qgos, var_qgos_dn0, var_qgos_dn2, var_qgos_dn6, var_qgos_dn7, var_qgos_dn10, var_qgos_dn11, var_qgos_dn12, var_qgos_dn17,)
    }
};
        var_qgos = assign34050_e49201;
        var_qgos_dn0 = assign34050_e49201_d_n0;
        var_qgos_dn2 = assign34050_e49201_d_n2;
        var_qgos_dn6 = assign34050_e49201_d_n6;
        var_qgos_dn7 = assign34050_e49201_d_n7;
        var_qgos_dn10 = assign34050_e49201_d_n10;
        var_qgos_dn11 = assign34050_e49201_d_n11;
        var_qgos_dn12 = assign34050_e49201_d_n12;
        var_qgos_dn17 = assign34050_e49201_d_n17;

        let (assign34060_e49207, assign34060_e49207_d_n0, assign34060_e49207_d_n2, assign34060_e49207_d_n6, assign34060_e49207_d_n7, assign34060_e49207_d_n10, assign34060_e49207_d_n11, assign34060_e49207_d_n12, assign34060_e49207_d_n17,) = {
    if (var_guard1128 != 0.0) {
        let assign34060_e49205: f64 = (var_qgob + var_qfbc);
        (assign34060_e49205, (var_qgob_dn0 + var_qfbc_dn0), (var_qgob_dn2 + var_qfbc_dn2), (var_qgob_dn6 + var_qfbc_dn6), (var_qgob_dn7 + var_qfbc_dn7), (var_qgob_dn10 + var_qfbc_dn10), (var_qgob_dn11 + var_qfbc_dn11), (var_qgob_dn12 + var_qfbc_dn12), (var_qgob_dn17 + var_qfbc_dn17),)
    } else {
        (var_qgob, var_qgob_dn0, var_qgob_dn2, var_qgob_dn6, var_qgob_dn7, var_qgob_dn10, var_qgob_dn11, var_qgob_dn12, var_qgob_dn17,)
    }
};
        var_qgob = assign34060_e49207;
        var_qgob_dn0 = assign34060_e49207_d_n0;
        var_qgob_dn2 = assign34060_e49207_d_n2;
        var_qgob_dn6 = assign34060_e49207_d_n6;
        var_qgob_dn7 = assign34060_e49207_d_n7;
        var_qgob_dn10 = assign34060_e49207_d_n10;
        var_qgob_dn11 = assign34060_e49207_d_n11;
        var_qgob_dn12 = assign34060_e49207_d_n12;
        var_qgob_dn17 = assign34060_e49207_d_n17;

        let (assign34070_e49217,) = {
    if ((var_guard1128 == 0.0) && (var_cgbo_given != 0.0)) {
        let assign34070_e49213: f64 = (-p.p168);
        let assign34070_e49215: f64 = (assign34070_e49213 * var_lgleff);
        (assign34070_e49215,)
    } else {
        (var_cgbe,)
    }
};
        var_cgbe = assign34070_e49217;

        let (assign34080_e49229, assign34080_e49229_d_n0, assign34080_e49229_d_n2, assign34080_e49229_d_n6, assign34080_e49229_d_n7, assign34080_e49229_d_n10, assign34080_e49229_d_n11, assign34080_e49229_d_n12, assign34080_e49229_d_n17,) = {
    if ((var_guard1128 == 0.0) && (var_cgbo_given != 0.0)) {
        let assign34080_e49223: f64 = (-var_cgbe);
        let assign34080_e49226: f64 = (var_vgs - var_vbsp);
        let assign34080_e49227: f64 = (assign34080_e49223 * assign34080_e49226);
        (assign34080_e49227, (assign34080_e49223 * (-var_vbsp_dn0)), (assign34080_e49223 * (-var_vbsp_dn2)), (assign34080_e49223 * (var_vgs_dn6 - var_vbsp_dn6)), (assign34080_e49223 * (var_vgs_dn7 - var_vbsp_dn7)), (assign34080_e49223 * (-var_vbsp_dn10)), (assign34080_e49223 * (var_vgs_dn11 - var_vbsp_dn11)), (assign34080_e49223 * (-var_vbsp_dn12)), (assign34080_e49223 * (-var_vbsp_dn17)),)
    } else {
        (var_qgob, var_qgob_dn0, var_qgob_dn2, var_qgob_dn6, var_qgob_dn7, var_qgob_dn10, var_qgob_dn11, var_qgob_dn12, var_qgob_dn17,)
    }
};
        var_qgob = assign34080_e49229;
        var_qgob_dn0 = assign34080_e49229_d_n0;
        var_qgob_dn2 = assign34080_e49229_d_n2;
        var_qgob_dn6 = assign34080_e49229_d_n6;
        var_qgob_dn7 = assign34080_e49229_d_n7;
        var_qgob_dn10 = assign34080_e49229_d_n10;
        var_qgob_dn11 = assign34080_e49229_d_n11;
        var_qgob_dn12 = assign34080_e49229_d_n12;
        var_qgob_dn17 = assign34080_e49229_d_n17;

        let (assign34090_e49237,) = {
    if ((var_guard1128 == 0.0) && (var_cgbo_given == 0.0)) {
        (0.0,)
    } else {
        (var_cgbe,)
    }
};
        var_cgbe = assign34090_e49237;

        let (assign34100_e49245, assign34100_e49245_d_n0, assign34100_e49245_d_n2, assign34100_e49245_d_n6, assign34100_e49245_d_n7, assign34100_e49245_d_n10, assign34100_e49245_d_n11, assign34100_e49245_d_n12, assign34100_e49245_d_n17,) = {
    if ((var_guard1128 == 0.0) && (var_cgbo_given == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qgob, var_qgob_dn0, var_qgob_dn2, var_qgob_dn6, var_qgob_dn7, var_qgob_dn10, var_qgob_dn11, var_qgob_dn12, var_qgob_dn17,)
    }
};
        var_qgob = assign34100_e49245;
        var_qgob_dn0 = assign34100_e49245_d_n0;
        var_qgob_dn2 = assign34100_e49245_d_n2;
        var_qgob_dn6 = assign34100_e49245_d_n6;
        var_qgob_dn7 = assign34100_e49245_d_n7;
        var_qgob_dn10 = assign34100_e49245_d_n10;
        var_qgob_dn11 = assign34100_e49245_d_n11;
        var_qgob_dn12 = assign34100_e49245_d_n12;
        var_qgob_dn17 = assign34100_e49245_d_n17;

        let (assign34110_e49260,) = {
    if (var_guard1128 == 0.0) {
        (0.0,)
    } else {
        (var_cf,)
    }
};
        var_cf = assign34110_e49260;

        let (assign34120_e49265,) = {
    if (var_guard1128 == 0.0) {
        (var_cf,)
    } else {
        (var_cfd,)
    }
};
        var_cfd = assign34120_e49265;

        let (assign34130_e49270,) = {
    if (var_guard1128 == 0.0) {
        (var_cf,)
    } else {
        (var_cfs,)
    }
};
        var_cfs = assign34130_e49270;

        let (assign34140_e49279, assign34140_e49279_d_n0, assign34140_e49279_d_n2, assign34140_e49279_d_n6, assign34140_e49279_d_n7, assign34140_e49279_d_n10, assign34140_e49279_d_n11, assign34140_e49279_d_n12, assign34140_e49279_d_n17,) = {
    if (var_guard1128 == 0.0) {
        let assign34140_e49276: f64 = (var_vgs - var_vds);
        let assign34140_e49277: f64 = (var_cfd * assign34140_e49276);
        (assign34140_e49277, (var_cfd * (-var_vds_dn0)), (var_cfd * (-var_vds_dn2)), (var_cfd * (var_vgs_dn6 - var_vds_dn6)), (var_cfd * (var_vgs_dn7 - var_vds_dn7)), (var_cfd * (-var_vds_dn10)), (var_cfd * (var_vgs_dn11 - var_vds_dn11)), (var_cfd * (-var_vds_dn12)), (var_cfd * (-var_vds_dn17)),)
    } else {
        (var_qfd, var_qfd_dn0, var_qfd_dn2, var_qfd_dn6, var_qfd_dn7, var_qfd_dn10, var_qfd_dn11, var_qfd_dn12, var_qfd_dn17,)
    }
};
        var_qfd = assign34140_e49279;
        var_qfd_dn0 = assign34140_e49279_d_n0;
        var_qfd_dn2 = assign34140_e49279_d_n2;
        var_qfd_dn6 = assign34140_e49279_d_n6;
        var_qfd_dn7 = assign34140_e49279_d_n7;
        var_qfd_dn10 = assign34140_e49279_d_n10;
        var_qfd_dn11 = assign34140_e49279_d_n11;
        var_qfd_dn12 = assign34140_e49279_d_n12;
        var_qfd_dn17 = assign34140_e49279_d_n17;

        let (assign34150_e49286, assign34150_e49286_d_n6, assign34150_e49286_d_n7, assign34150_e49286_d_n11,) = {
    if (var_guard1128 == 0.0) {
        let assign34150_e49284: f64 = (var_cfs * var_vgs);
        (assign34150_e49284, (var_cfs * var_vgs_dn6), (var_cfs * var_vgs_dn7), (var_cfs * var_vgs_dn11),)
    } else {
        (var_qfs, var_qfs_dn6, var_qfs_dn7, var_qfs_dn11,)
    }
};
        var_qfs = assign34150_e49286;
        var_qfs_dn6 = assign34150_e49286_d_n6;
        var_qfs_dn7 = assign34150_e49286_d_n7;
        var_qfs_dn11 = assign34150_e49286_d_n11;

        let (assign34160_e49293, assign34160_e49293_d_n0, assign34160_e49293_d_n2, assign34160_e49293_d_n6, assign34160_e49293_d_n7, assign34160_e49293_d_n10, assign34160_e49293_d_n11, assign34160_e49293_d_n12, assign34160_e49293_d_n17,) = {
    if (var_guard1128 == 0.0) {
        let assign34160_e49291: f64 = (var_qgod + var_qfd);
        (assign34160_e49291, (var_qgod_dn0 + var_qfd_dn0), (var_qgod_dn2 + var_qfd_dn2), (var_qgod_dn6 + var_qfd_dn6), (var_qgod_dn7 + var_qfd_dn7), (var_qgod_dn10 + var_qfd_dn10), (var_qgod_dn11 + var_qfd_dn11), (var_qgod_dn12 + var_qfd_dn12), (var_qgod_dn17 + var_qfd_dn17),)
    } else {
        (var_qgod, var_qgod_dn0, var_qgod_dn2, var_qgod_dn6, var_qgod_dn7, var_qgod_dn10, var_qgod_dn11, var_qgod_dn12, var_qgod_dn17,)
    }
};
        var_qgod = assign34160_e49293;
        var_qgod_dn0 = assign34160_e49293_d_n0;
        var_qgod_dn2 = assign34160_e49293_d_n2;
        var_qgod_dn6 = assign34160_e49293_d_n6;
        var_qgod_dn7 = assign34160_e49293_d_n7;
        var_qgod_dn10 = assign34160_e49293_d_n10;
        var_qgod_dn11 = assign34160_e49293_d_n11;
        var_qgod_dn12 = assign34160_e49293_d_n12;
        var_qgod_dn17 = assign34160_e49293_d_n17;

        let (assign34170_e49300, assign34170_e49300_d_n0, assign34170_e49300_d_n2, assign34170_e49300_d_n6, assign34170_e49300_d_n7, assign34170_e49300_d_n10, assign34170_e49300_d_n11, assign34170_e49300_d_n12, assign34170_e49300_d_n17,) = {
    if (var_guard1128 == 0.0) {
        let assign34170_e49298: f64 = (var_qgos + var_qfs);
        (assign34170_e49298, var_qgos_dn0, var_qgos_dn2, (var_qgos_dn6 + var_qfs_dn6), (var_qgos_dn7 + var_qfs_dn7), var_qgos_dn10, (var_qgos_dn11 + var_qfs_dn11), var_qgos_dn12, var_qgos_dn17,)
    } else {
        (var_qgos, var_qgos_dn0, var_qgos_dn2, var_qgos_dn6, var_qgos_dn7, var_qgos_dn10, var_qgos_dn11, var_qgos_dn12, var_qgos_dn17,)
    }
};
        var_qgos = assign34170_e49300;
        var_qgos_dn0 = assign34170_e49300_d_n0;
        var_qgos_dn2 = assign34170_e49300_d_n2;
        var_qgos_dn6 = assign34170_e49300_d_n6;
        var_qgos_dn7 = assign34170_e49300_d_n7;
        var_qgos_dn10 = assign34170_e49300_d_n10;
        var_qgos_dn11 = assign34170_e49300_d_n11;
        var_qgos_dn12 = assign34170_e49300_d_n12;
        var_qgos_dn17 = assign34170_e49300_d_n17;

        let assign34180_e49303: f64 = (var_mfactor * var_ids);
        var_idse = assign34180_e49303;
        var_idse_dn0 = (var_mfactor * var_ids_dn0);
        var_idse_dn2 = (var_mfactor * var_ids_dn2);
        var_idse_dn6 = (var_mfactor * var_ids_dn6);
        var_idse_dn7 = (var_mfactor * var_ids_dn7);
        var_idse_dn10 = (var_mfactor * var_ids_dn10);
        var_idse_dn11 = (var_mfactor * var_ids_dn11);
        var_idse_dn12 = (var_mfactor * var_ids_dn12);
        var_idse_dn17 = (var_mfactor * var_ids_dn17);

        let (assign34190_e49307, assign34190_e49307_d_n0, assign34190_e49307_d_n2, assign34190_e49307_d_n6, assign34190_e49307_d_n7, assign34190_e49307_d_n10, assign34190_e49307_d_n11, assign34190_e49307_d_n12, assign34190_e49307_d_n13, assign34190_e49307_d_n15, assign34190_e49307_d_n16, assign34190_e49307_d_n17, assign34190_e49307_d_n18,) = {
    if (var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn6, var_qde_dn7, var_qde_dn10, var_qde_dn11, var_qde_dn12, var_qde_dn13, var_qde_dn15, var_qde_dn16, var_qde_dn17, var_qde_dn18,)
    }
};
        var_qde = assign34190_e49307;
        var_qde_dn0 = assign34190_e49307_d_n0;
        var_qde_dn2 = assign34190_e49307_d_n2;
        var_qde_dn6 = assign34190_e49307_d_n6;
        var_qde_dn7 = assign34190_e49307_d_n7;
        var_qde_dn10 = assign34190_e49307_d_n10;
        var_qde_dn11 = assign34190_e49307_d_n11;
        var_qde_dn12 = assign34190_e49307_d_n12;
        var_qde_dn13 = assign34190_e49307_d_n13;
        var_qde_dn15 = assign34190_e49307_d_n15;
        var_qde_dn16 = assign34190_e49307_d_n16;
        var_qde_dn17 = assign34190_e49307_d_n17;
        var_qde_dn18 = assign34190_e49307_d_n18;

        let (assign34200_e49311, assign34200_e49311_d_n0, assign34200_e49311_d_n2, assign34200_e49311_d_n6, assign34200_e49311_d_n7, assign34200_e49311_d_n10, assign34200_e49311_d_n11, assign34200_e49311_d_n12, assign34200_e49311_d_n13, assign34200_e49311_d_n15, assign34200_e49311_d_n16, assign34200_e49311_d_n17, assign34200_e49311_d_n18,) = {
    if (var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn6, var_qge_dn7, var_qge_dn10, var_qge_dn11, var_qge_dn12, var_qge_dn13, var_qge_dn15, var_qge_dn16, var_qge_dn17, var_qge_dn18,)
    }
};
        var_qge = assign34200_e49311;
        var_qge_dn0 = assign34200_e49311_d_n0;
        var_qge_dn2 = assign34200_e49311_d_n2;
        var_qge_dn6 = assign34200_e49311_d_n6;
        var_qge_dn7 = assign34200_e49311_d_n7;
        var_qge_dn10 = assign34200_e49311_d_n10;
        var_qge_dn11 = assign34200_e49311_d_n11;
        var_qge_dn12 = assign34200_e49311_d_n12;
        var_qge_dn13 = assign34200_e49311_d_n13;
        var_qge_dn15 = assign34200_e49311_d_n15;
        var_qge_dn16 = assign34200_e49311_d_n16;
        var_qge_dn17 = assign34200_e49311_d_n17;
        var_qge_dn18 = assign34200_e49311_d_n18;

        let assign34210_e49314: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1129 = assign34210_e49314;

        let (assign34220_e49320, assign34220_e49320_d_n0, assign34220_e49320_d_n2, assign34220_e49320_d_n6, assign34220_e49320_d_n7, assign34220_e49320_d_n10, assign34220_e49320_d_n11, assign34220_e49320_d_n12, assign34220_e49320_d_n13, assign34220_e49320_d_n15, assign34220_e49320_d_n16, assign34220_e49320_d_n17, assign34220_e49320_d_n18,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1129 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn6, var_qse_dn7, var_qse_dn10, var_qse_dn11, var_qse_dn12, var_qse_dn13, var_qse_dn15, var_qse_dn16, var_qse_dn17, var_qse_dn18,)
    }
};
        var_qse = assign34220_e49320;
        var_qse_dn0 = assign34220_e49320_d_n0;
        var_qse_dn2 = assign34220_e49320_d_n2;
        var_qse_dn6 = assign34220_e49320_d_n6;
        var_qse_dn7 = assign34220_e49320_d_n7;
        var_qse_dn10 = assign34220_e49320_d_n10;
        var_qse_dn11 = assign34220_e49320_d_n11;
        var_qse_dn12 = assign34220_e49320_d_n12;
        var_qse_dn13 = assign34220_e49320_d_n13;
        var_qse_dn15 = assign34220_e49320_d_n15;
        var_qse_dn16 = assign34220_e49320_d_n16;
        var_qse_dn17 = assign34220_e49320_d_n17;
        var_qse_dn18 = assign34220_e49320_d_n18;

        let (assign34230_e49326, assign34230_e49326_d_n0, assign34230_e49326_d_n2, assign34230_e49326_d_n6, assign34230_e49326_d_n7, assign34230_e49326_d_n10, assign34230_e49326_d_n11, assign34230_e49326_d_n12, assign34230_e49326_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1129 != 0.0)) {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn10, var_qdrat_dn11, var_qdrat_dn12, var_qdrat_dn17,)
    } else {
        (var_xd, var_xd_dn0, var_xd_dn2, var_xd_dn6, var_xd_dn7, var_xd_dn10, var_xd_dn11, var_xd_dn12, var_xd_dn17,)
    }
};
        var_xd = assign34230_e49326;
        var_xd_dn0 = assign34230_e49326_d_n0;
        var_xd_dn2 = assign34230_e49326_d_n2;
        var_xd_dn6 = assign34230_e49326_d_n6;
        var_xd_dn7 = assign34230_e49326_d_n7;
        var_xd_dn10 = assign34230_e49326_d_n10;
        var_xd_dn11 = assign34230_e49326_d_n11;
        var_xd_dn12 = assign34230_e49326_d_n12;
        var_xd_dn17 = assign34230_e49326_d_n17;

        let (assign34240_e49334, assign34240_e49334_d_n0, assign34240_e49334_d_n2, assign34240_e49334_d_n6, assign34240_e49334_d_n7, assign34240_e49334_d_n10, assign34240_e49334_d_n11, assign34240_e49334_d_n12, assign34240_e49334_d_n13, assign34240_e49334_d_n15, assign34240_e49334_d_n16, assign34240_e49334_d_n17, assign34240_e49334_d_n18,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1129 != 0.0)) {
        let assign34240_e49332: f64 = (var_mfactor * var_qb);
        (assign34240_e49332, (var_mfactor * var_qb_dn0), (var_mfactor * var_qb_dn2), (var_mfactor * var_qb_dn6), (var_mfactor * var_qb_dn7), (var_mfactor * var_qb_dn10), (var_mfactor * var_qb_dn11), (var_mfactor * var_qb_dn12), (var_mfactor * var_qb_dn13), (var_mfactor * var_qb_dn15), (var_mfactor * var_qb_dn16), (var_mfactor * var_qb_dn17), (var_mfactor * var_qb_dn18),)
    } else {
        (var_qb_qs, var_qb_qs_dn0, var_qb_qs_dn2, var_qb_qs_dn6, var_qb_qs_dn7, var_qb_qs_dn10, var_qb_qs_dn11, var_qb_qs_dn12, var_qb_qs_dn13, var_qb_qs_dn15, var_qb_qs_dn16, var_qb_qs_dn17, var_qb_qs_dn18,)
    }
};
        var_qb_qs = assign34240_e49334;
        var_qb_qs_dn0 = assign34240_e49334_d_n0;
        var_qb_qs_dn2 = assign34240_e49334_d_n2;
        var_qb_qs_dn6 = assign34240_e49334_d_n6;
        var_qb_qs_dn7 = assign34240_e49334_d_n7;
        var_qb_qs_dn10 = assign34240_e49334_d_n10;
        var_qb_qs_dn11 = assign34240_e49334_d_n11;
        var_qb_qs_dn12 = assign34240_e49334_d_n12;
        var_qb_qs_dn13 = assign34240_e49334_d_n13;
        var_qb_qs_dn15 = assign34240_e49334_d_n15;
        var_qb_qs_dn16 = assign34240_e49334_d_n16;
        var_qb_qs_dn17 = assign34240_e49334_d_n17;
        var_qb_qs_dn18 = assign34240_e49334_d_n18;

        let (assign34250_e49342, assign34250_e49342_d_n0, assign34250_e49342_d_n2, assign34250_e49342_d_n6, assign34250_e49342_d_n7, assign34250_e49342_d_n10, assign34250_e49342_d_n11, assign34250_e49342_d_n12, assign34250_e49342_d_n17,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1129 != 0.0)) {
        let assign34250_e49340: f64 = (var_mfactor * var_qi);
        (assign34250_e49340, (var_mfactor * var_qi_dn0), (var_mfactor * var_qi_dn2), (var_mfactor * var_qi_dn6), (var_mfactor * var_qi_dn7), (var_mfactor * var_qi_dn10), (var_mfactor * var_qi_dn11), (var_mfactor * var_qi_dn12), (var_mfactor * var_qi_dn17),)
    } else {
        (var_qi_qs, var_qi_qs_dn0, var_qi_qs_dn2, var_qi_qs_dn6, var_qi_qs_dn7, var_qi_qs_dn10, var_qi_qs_dn11, var_qi_qs_dn12, var_qi_qs_dn17,)
    }
};
        var_qi_qs = assign34250_e49342;
        var_qi_qs_dn0 = assign34250_e49342_d_n0;
        var_qi_qs_dn2 = assign34250_e49342_d_n2;
        var_qi_qs_dn6 = assign34250_e49342_d_n6;
        var_qi_qs_dn7 = assign34250_e49342_d_n7;
        var_qi_qs_dn10 = assign34250_e49342_d_n10;
        var_qi_qs_dn11 = assign34250_e49342_d_n11;
        var_qi_qs_dn12 = assign34250_e49342_d_n12;
        var_qi_qs_dn17 = assign34250_e49342_d_n17;

        let (assign34260_e49349, assign34260_e49349_d_n0, assign34260_e49349_d_n2, assign34260_e49349_d_n6, assign34260_e49349_d_n7, assign34260_e49349_d_n10, assign34260_e49349_d_n11, assign34260_e49349_d_n12, assign34260_e49349_d_n13, assign34260_e49349_d_n15, assign34260_e49349_d_n16, assign34260_e49349_d_n17, assign34260_e49349_d_n18,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1129 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_dn13, var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    }
};
        var_qbe = assign34260_e49349;
        var_qbe_dn0 = assign34260_e49349_d_n0;
        var_qbe_dn2 = assign34260_e49349_d_n2;
        var_qbe_dn6 = assign34260_e49349_d_n6;
        var_qbe_dn7 = assign34260_e49349_d_n7;
        var_qbe_dn10 = assign34260_e49349_d_n10;
        var_qbe_dn11 = assign34260_e49349_d_n11;
        var_qbe_dn12 = assign34260_e49349_d_n12;
        var_qbe_dn13 = assign34260_e49349_d_n13;
        var_qbe_dn15 = assign34260_e49349_d_n15;
        var_qbe_dn16 = assign34260_e49349_d_n16;
        var_qbe_dn17 = assign34260_e49349_d_n17;
        var_qbe_dn18 = assign34260_e49349_d_n18;

        let (assign34270_e49358, assign34270_e49358_d_n0, assign34270_e49358_d_n2, assign34270_e49358_d_n6, assign34270_e49358_d_n7, assign34270_e49358_d_n10, assign34270_e49358_d_n11, assign34270_e49358_d_n12, assign34270_e49358_d_n13, assign34270_e49358_d_n15, assign34270_e49358_d_n16, assign34270_e49358_d_n17, assign34270_e49358_d_n18,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1129 == 0.0)) {
        let assign34270_e49356: f64 = (var_mfactor * var_qsub);
        (assign34270_e49356, (var_mfactor * var_qsub_dn0), (var_mfactor * var_qsub_dn2), (var_mfactor * var_qsub_dn6), (var_mfactor * var_qsub_dn7), (var_mfactor * var_qsub_dn10), (var_mfactor * var_qsub_dn11), (var_mfactor * var_qsub_dn12), 0.0, 0.0, 0.0, (var_mfactor * var_qsub_dn17), 0.0,)
    } else {
        (var_qb_qs, var_qb_qs_dn0, var_qb_qs_dn2, var_qb_qs_dn6, var_qb_qs_dn7, var_qb_qs_dn10, var_qb_qs_dn11, var_qb_qs_dn12, var_qb_qs_dn13, var_qb_qs_dn15, var_qb_qs_dn16, var_qb_qs_dn17, var_qb_qs_dn18,)
    }
};
        var_qb_qs = assign34270_e49358;
        var_qb_qs_dn0 = assign34270_e49358_d_n0;
        var_qb_qs_dn2 = assign34270_e49358_d_n2;
        var_qb_qs_dn6 = assign34270_e49358_d_n6;
        var_qb_qs_dn7 = assign34270_e49358_d_n7;
        var_qb_qs_dn10 = assign34270_e49358_d_n10;
        var_qb_qs_dn11 = assign34270_e49358_d_n11;
        var_qb_qs_dn12 = assign34270_e49358_d_n12;
        var_qb_qs_dn13 = assign34270_e49358_d_n13;
        var_qb_qs_dn15 = assign34270_e49358_d_n15;
        var_qb_qs_dn16 = assign34270_e49358_d_n16;
        var_qb_qs_dn17 = assign34270_e49358_d_n17;
        var_qb_qs_dn18 = assign34270_e49358_d_n18;

        let (assign34280_e49369, assign34280_e49369_d_n0, assign34280_e49369_d_n2, assign34280_e49369_d_n6, assign34280_e49369_d_n7, assign34280_e49369_d_n10, assign34280_e49369_d_n11, assign34280_e49369_d_n12, assign34280_e49369_d_n13, assign34280_e49369_d_n15, assign34280_e49369_d_n16, assign34280_e49369_d_n17, assign34280_e49369_d_n18,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1129 == 0.0)) {
        let assign34280_e49366: f64 = (var_qd + var_qd_fb);
        let assign34280_e49367: f64 = (var_mfactor * assign34280_e49366);
        (assign34280_e49367, (var_mfactor * (var_qd_dn0 + var_qd_fb_dn0)), (var_mfactor * (var_qd_dn2 + var_qd_fb_dn2)), (var_mfactor * (var_qd_dn6 + var_qd_fb_dn6)), (var_mfactor * (var_qd_dn7 + var_qd_fb_dn7)), (var_mfactor * (var_qd_dn10 + var_qd_fb_dn10)), (var_mfactor * (var_qd_dn11 + var_qd_fb_dn11)), (var_mfactor * (var_qd_dn12 + var_qd_fb_dn12)), (var_mfactor * (var_qd_dn13 + var_qd_fb_dn13)), (var_mfactor * (var_qd_dn15 + var_qd_fb_dn15)), (var_mfactor * (var_qd_dn16 + var_qd_fb_dn16)), (var_mfactor * (var_qd_dn17 + var_qd_fb_dn17)), (var_mfactor * (var_qd_dn18 + var_qd_fb_dn18)),)
    } else {
        (var_qd_qs, var_qd_qs_dn0, var_qd_qs_dn2, var_qd_qs_dn6, var_qd_qs_dn7, var_qd_qs_dn10, var_qd_qs_dn11, var_qd_qs_dn12, var_qd_qs_dn13, var_qd_qs_dn15, var_qd_qs_dn16, var_qd_qs_dn17, var_qd_qs_dn18,)
    }
};
        var_qd_qs = assign34280_e49369;
        var_qd_qs_dn0 = assign34280_e49369_d_n0;
        var_qd_qs_dn2 = assign34280_e49369_d_n2;
        var_qd_qs_dn6 = assign34280_e49369_d_n6;
        var_qd_qs_dn7 = assign34280_e49369_d_n7;
        var_qd_qs_dn10 = assign34280_e49369_d_n10;
        var_qd_qs_dn11 = assign34280_e49369_d_n11;
        var_qd_qs_dn12 = assign34280_e49369_d_n12;
        var_qd_qs_dn13 = assign34280_e49369_d_n13;
        var_qd_qs_dn15 = assign34280_e49369_d_n15;
        var_qd_qs_dn16 = assign34280_e49369_d_n16;
        var_qd_qs_dn17 = assign34280_e49369_d_n17;
        var_qd_qs_dn18 = assign34280_e49369_d_n18;

        *var_cf_slot = var_cf;
        *var_cfd_slot = var_cfd;
        *var_cfs_slot = var_cfs;
        *var_cfu_slot = var_cfu;
        *var_cgbe_slot = var_cgbe;
        *var_guard1129_slot = var_guard1129;
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
        var_guard1129: f64,
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
        var_guard1130_slot: &mut f64,
        var_guard1136_slot: &mut f64,
        var_guard1137_slot: &mut f64,
        var_guard1138_slot: &mut f64,
        var_guard1139_slot: &mut f64,
        var_guard1140_slot: &mut f64,
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
        var_t10__blk1133_slot: &mut f64,
        var_t10__blk1133_dn0_slot: &mut f64,
        var_t10__blk1133_dn10_slot: &mut f64,
        var_t10__blk1133_dn11_slot: &mut f64,
        var_t10__blk1133_dn12_slot: &mut f64,
        var_t10__blk1133_dn17_slot: &mut f64,
        var_t10__blk1133_dn2_slot: &mut f64,
        var_t10__blk1133_dn6_slot: &mut f64,
        var_t10__blk1133_dn7_slot: &mut f64,
        var_t1__blk1132_slot: &mut f64,
        var_t1__blk1132_dn0_slot: &mut f64,
        var_t1__blk1132_dn10_slot: &mut f64,
        var_t1__blk1132_dn11_slot: &mut f64,
        var_t1__blk1132_dn12_slot: &mut f64,
        var_t1__blk1132_dn17_slot: &mut f64,
        var_t1__blk1132_dn2_slot: &mut f64,
        var_t1__blk1132_dn6_slot: &mut f64,
        var_t1__blk1132_dn7_slot: &mut f64,
        var_t2__blk1135_slot: &mut f64,
        var_t2__blk1135_dn0_slot: &mut f64,
        var_t2__blk1135_dn10_slot: &mut f64,
        var_t2__blk1135_dn11_slot: &mut f64,
        var_t2__blk1135_dn12_slot: &mut f64,
        var_t2__blk1135_dn17_slot: &mut f64,
        var_t2__blk1135_dn2_slot: &mut f64,
        var_t2__blk1135_dn6_slot: &mut f64,
        var_t2__blk1135_dn7_slot: &mut f64,
        var_t3__blk1134_slot: &mut f64,
        var_t3__blk1134_dn0_slot: &mut f64,
        var_t3__blk1134_dn10_slot: &mut f64,
        var_t3__blk1134_dn11_slot: &mut f64,
        var_t3__blk1134_dn12_slot: &mut f64,
        var_t3__blk1134_dn17_slot: &mut f64,
        var_t3__blk1134_dn2_slot: &mut f64,
        var_t3__blk1134_dn6_slot: &mut f64,
        var_t3__blk1134_dn7_slot: &mut f64,
    ) {
        let mut var_guard1130: f64 = *var_guard1130_slot;
        let mut var_guard1136: f64 = *var_guard1136_slot;
        let mut var_guard1137: f64 = *var_guard1137_slot;
        let mut var_guard1138: f64 = *var_guard1138_slot;
        let mut var_guard1139: f64 = *var_guard1139_slot;
        let mut var_guard1140: f64 = *var_guard1140_slot;
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
        let mut var_t10__blk1133: f64 = *var_t10__blk1133_slot;
        let mut var_t10__blk1133_dn0: f64 = *var_t10__blk1133_dn0_slot;
        let mut var_t10__blk1133_dn10: f64 = *var_t10__blk1133_dn10_slot;
        let mut var_t10__blk1133_dn11: f64 = *var_t10__blk1133_dn11_slot;
        let mut var_t10__blk1133_dn12: f64 = *var_t10__blk1133_dn12_slot;
        let mut var_t10__blk1133_dn17: f64 = *var_t10__blk1133_dn17_slot;
        let mut var_t10__blk1133_dn2: f64 = *var_t10__blk1133_dn2_slot;
        let mut var_t10__blk1133_dn6: f64 = *var_t10__blk1133_dn6_slot;
        let mut var_t10__blk1133_dn7: f64 = *var_t10__blk1133_dn7_slot;
        let mut var_t1__blk1132: f64 = *var_t1__blk1132_slot;
        let mut var_t1__blk1132_dn0: f64 = *var_t1__blk1132_dn0_slot;
        let mut var_t1__blk1132_dn10: f64 = *var_t1__blk1132_dn10_slot;
        let mut var_t1__blk1132_dn11: f64 = *var_t1__blk1132_dn11_slot;
        let mut var_t1__blk1132_dn12: f64 = *var_t1__blk1132_dn12_slot;
        let mut var_t1__blk1132_dn17: f64 = *var_t1__blk1132_dn17_slot;
        let mut var_t1__blk1132_dn2: f64 = *var_t1__blk1132_dn2_slot;
        let mut var_t1__blk1132_dn6: f64 = *var_t1__blk1132_dn6_slot;
        let mut var_t1__blk1132_dn7: f64 = *var_t1__blk1132_dn7_slot;
        let mut var_t2__blk1135: f64 = *var_t2__blk1135_slot;
        let mut var_t2__blk1135_dn0: f64 = *var_t2__blk1135_dn0_slot;
        let mut var_t2__blk1135_dn10: f64 = *var_t2__blk1135_dn10_slot;
        let mut var_t2__blk1135_dn11: f64 = *var_t2__blk1135_dn11_slot;
        let mut var_t2__blk1135_dn12: f64 = *var_t2__blk1135_dn12_slot;
        let mut var_t2__blk1135_dn17: f64 = *var_t2__blk1135_dn17_slot;
        let mut var_t2__blk1135_dn2: f64 = *var_t2__blk1135_dn2_slot;
        let mut var_t2__blk1135_dn6: f64 = *var_t2__blk1135_dn6_slot;
        let mut var_t2__blk1135_dn7: f64 = *var_t2__blk1135_dn7_slot;
        let mut var_t3__blk1134: f64 = *var_t3__blk1134_slot;
        let mut var_t3__blk1134_dn0: f64 = *var_t3__blk1134_dn0_slot;
        let mut var_t3__blk1134_dn10: f64 = *var_t3__blk1134_dn10_slot;
        let mut var_t3__blk1134_dn11: f64 = *var_t3__blk1134_dn11_slot;
        let mut var_t3__blk1134_dn12: f64 = *var_t3__blk1134_dn12_slot;
        let mut var_t3__blk1134_dn17: f64 = *var_t3__blk1134_dn17_slot;
        let mut var_t3__blk1134_dn2: f64 = *var_t3__blk1134_dn2_slot;
        let mut var_t3__blk1134_dn6: f64 = *var_t3__blk1134_dn6_slot;
        let mut var_t3__blk1134_dn7: f64 = *var_t3__blk1134_dn7_slot;

        let (assign34290_e49382, assign34290_e49382_d_n0, assign34290_e49382_d_n2, assign34290_e49382_d_n6, assign34290_e49382_d_n7, assign34290_e49382_d_n10, assign34290_e49382_d_n11, assign34290_e49382_d_n12, assign34290_e49382_d_n13, assign34290_e49382_d_n15, assign34290_e49382_d_n16, assign34290_e49382_d_n17, assign34290_e49382_d_n18,) = {
    if ((var_flg_nqs != 0.0) && (var_guard1129 == 0.0)) {
        let assign34290_e49377: f64 = (var_qi - var_qd);
        let assign34290_e49379: f64 = (assign34290_e49377 + var_qs_fb);
        let assign34290_e49380: f64 = (var_mfactor * assign34290_e49379);
        (assign34290_e49380, (var_mfactor * ((var_qi_dn0 - var_qd_dn0) + var_qs_fb_dn0)), (var_mfactor * ((var_qi_dn2 - var_qd_dn2) + var_qs_fb_dn2)), (var_mfactor * ((var_qi_dn6 - var_qd_dn6) + var_qs_fb_dn6)), (var_mfactor * ((var_qi_dn7 - var_qd_dn7) + var_qs_fb_dn7)), (var_mfactor * ((var_qi_dn10 - var_qd_dn10) + var_qs_fb_dn10)), (var_mfactor * ((var_qi_dn11 - var_qd_dn11) + var_qs_fb_dn11)), (var_mfactor * ((var_qi_dn12 - var_qd_dn12) + var_qs_fb_dn12)), (var_mfactor * ((-var_qd_dn13) + var_qs_fb_dn13)), (var_mfactor * ((-var_qd_dn15) + var_qs_fb_dn15)), (var_mfactor * ((-var_qd_dn16) + var_qs_fb_dn16)), (var_mfactor * ((var_qi_dn17 - var_qd_dn17) + var_qs_fb_dn17)), (var_mfactor * ((-var_qd_dn18) + var_qs_fb_dn18)),)
    } else {
        (var_qs_qs, var_qs_qs_dn0, var_qs_qs_dn2, var_qs_qs_dn6, var_qs_qs_dn7, var_qs_qs_dn10, var_qs_qs_dn11, var_qs_qs_dn12, var_qs_qs_dn13, var_qs_qs_dn15, var_qs_qs_dn16, var_qs_qs_dn17, var_qs_qs_dn18,)
    }
};
        var_qs_qs = assign34290_e49382;
        var_qs_qs_dn0 = assign34290_e49382_d_n0;
        var_qs_qs_dn2 = assign34290_e49382_d_n2;
        var_qs_qs_dn6 = assign34290_e49382_d_n6;
        var_qs_qs_dn7 = assign34290_e49382_d_n7;
        var_qs_qs_dn10 = assign34290_e49382_d_n10;
        var_qs_qs_dn11 = assign34290_e49382_d_n11;
        var_qs_qs_dn12 = assign34290_e49382_d_n12;
        var_qs_qs_dn13 = assign34290_e49382_d_n13;
        var_qs_qs_dn15 = assign34290_e49382_d_n15;
        var_qs_qs_dn16 = assign34290_e49382_d_n16;
        var_qs_qs_dn17 = assign34290_e49382_d_n17;
        var_qs_qs_dn18 = assign34290_e49382_d_n18;

        let assign34300_e49385: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1130 = assign34300_e49385;

        let (assign34310_e49397, assign34310_e49397_d_n0, assign34310_e49397_d_n2, assign34310_e49397_d_n6, assign34310_e49397_d_n7, assign34310_e49397_d_n10, assign34310_e49397_d_n11, assign34310_e49397_d_n12, assign34310_e49397_d_n13, assign34310_e49397_d_n15, assign34310_e49397_d_n16, assign34310_e49397_d_n17, assign34310_e49397_d_n18,) = {
    if ((var_flg_nqs == 0.0) && (var_guard1130 != 0.0)) {
        let assign34310_e49392: f64 = (-var_qb);
        let assign34310_e49394: f64 = (assign34310_e49392 - var_qi);
        let assign34310_e49395: f64 = (var_mfactor * assign34310_e49394);
        (assign34310_e49395, (var_mfactor * ((-var_qb_dn0) - var_qi_dn0)), (var_mfactor * ((-var_qb_dn2) - var_qi_dn2)), (var_mfactor * ((-var_qb_dn6) - var_qi_dn6)), (var_mfactor * ((-var_qb_dn7) - var_qi_dn7)), (var_mfactor * ((-var_qb_dn10) - var_qi_dn10)), (var_mfactor * ((-var_qb_dn11) - var_qi_dn11)), (var_mfactor * ((-var_qb_dn12) - var_qi_dn12)), (var_mfactor * (-var_qb_dn13)), (var_mfactor * (-var_qb_dn15)), (var_mfactor * (-var_qb_dn16)), (var_mfactor * ((-var_qb_dn17) - var_qi_dn17)), (var_mfactor * (-var_qb_dn18)),)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn6, var_qge_dn7, var_qge_dn10, var_qge_dn11, var_qge_dn12, var_qge_dn13, var_qge_dn15, var_qge_dn16, var_qge_dn17, var_qge_dn18,)
    }
};
        var_qge = assign34310_e49397;
        var_qge_dn0 = assign34310_e49397_d_n0;
        var_qge_dn2 = assign34310_e49397_d_n2;
        var_qge_dn6 = assign34310_e49397_d_n6;
        var_qge_dn7 = assign34310_e49397_d_n7;
        var_qge_dn10 = assign34310_e49397_d_n10;
        var_qge_dn11 = assign34310_e49397_d_n11;
        var_qge_dn12 = assign34310_e49397_d_n12;
        var_qge_dn13 = assign34310_e49397_d_n13;
        var_qge_dn15 = assign34310_e49397_d_n15;
        var_qge_dn16 = assign34310_e49397_d_n16;
        var_qge_dn17 = assign34310_e49397_d_n17;
        var_qge_dn18 = assign34310_e49397_d_n18;

        let (assign34320_e49406, assign34320_e49406_d_n0, assign34320_e49406_d_n2, assign34320_e49406_d_n6, assign34320_e49406_d_n7, assign34320_e49406_d_n10, assign34320_e49406_d_n11, assign34320_e49406_d_n12, assign34320_e49406_d_n13, assign34320_e49406_d_n15, assign34320_e49406_d_n16, assign34320_e49406_d_n17, assign34320_e49406_d_n18,) = {
    if ((var_flg_nqs == 0.0) && (var_guard1130 != 0.0)) {
        let assign34320_e49404: f64 = (var_mfactor * var_qd);
        (assign34320_e49404, (var_mfactor * var_qd_dn0), (var_mfactor * var_qd_dn2), (var_mfactor * var_qd_dn6), (var_mfactor * var_qd_dn7), (var_mfactor * var_qd_dn10), (var_mfactor * var_qd_dn11), (var_mfactor * var_qd_dn12), (var_mfactor * var_qd_dn13), (var_mfactor * var_qd_dn15), (var_mfactor * var_qd_dn16), (var_mfactor * var_qd_dn17), (var_mfactor * var_qd_dn18),)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn6, var_qde_dn7, var_qde_dn10, var_qde_dn11, var_qde_dn12, var_qde_dn13, var_qde_dn15, var_qde_dn16, var_qde_dn17, var_qde_dn18,)
    }
};
        var_qde = assign34320_e49406;
        var_qde_dn0 = assign34320_e49406_d_n0;
        var_qde_dn2 = assign34320_e49406_d_n2;
        var_qde_dn6 = assign34320_e49406_d_n6;
        var_qde_dn7 = assign34320_e49406_d_n7;
        var_qde_dn10 = assign34320_e49406_d_n10;
        var_qde_dn11 = assign34320_e49406_d_n11;
        var_qde_dn12 = assign34320_e49406_d_n12;
        var_qde_dn13 = assign34320_e49406_d_n13;
        var_qde_dn15 = assign34320_e49406_d_n15;
        var_qde_dn16 = assign34320_e49406_d_n16;
        var_qde_dn17 = assign34320_e49406_d_n17;
        var_qde_dn18 = assign34320_e49406_d_n18;

        let (assign34330_e49417, assign34330_e49417_d_n0, assign34330_e49417_d_n2, assign34330_e49417_d_n6, assign34330_e49417_d_n7, assign34330_e49417_d_n10, assign34330_e49417_d_n11, assign34330_e49417_d_n12, assign34330_e49417_d_n13, assign34330_e49417_d_n15, assign34330_e49417_d_n16, assign34330_e49417_d_n17, assign34330_e49417_d_n18,) = {
    if ((var_flg_nqs == 0.0) && (var_guard1130 != 0.0)) {
        let assign34330_e49414: f64 = (var_qi - var_qd);
        let assign34330_e49415: f64 = (var_mfactor * assign34330_e49414);
        (assign34330_e49415, (var_mfactor * (var_qi_dn0 - var_qd_dn0)), (var_mfactor * (var_qi_dn2 - var_qd_dn2)), (var_mfactor * (var_qi_dn6 - var_qd_dn6)), (var_mfactor * (var_qi_dn7 - var_qd_dn7)), (var_mfactor * (var_qi_dn10 - var_qd_dn10)), (var_mfactor * (var_qi_dn11 - var_qd_dn11)), (var_mfactor * (var_qi_dn12 - var_qd_dn12)), (var_mfactor * (-var_qd_dn13)), (var_mfactor * (-var_qd_dn15)), (var_mfactor * (-var_qd_dn16)), (var_mfactor * (var_qi_dn17 - var_qd_dn17)), (var_mfactor * (-var_qd_dn18)),)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn6, var_qse_dn7, var_qse_dn10, var_qse_dn11, var_qse_dn12, var_qse_dn13, var_qse_dn15, var_qse_dn16, var_qse_dn17, var_qse_dn18,)
    }
};
        var_qse = assign34330_e49417;
        var_qse_dn0 = assign34330_e49417_d_n0;
        var_qse_dn2 = assign34330_e49417_d_n2;
        var_qse_dn6 = assign34330_e49417_d_n6;
        var_qse_dn7 = assign34330_e49417_d_n7;
        var_qse_dn10 = assign34330_e49417_d_n10;
        var_qse_dn11 = assign34330_e49417_d_n11;
        var_qse_dn12 = assign34330_e49417_d_n12;
        var_qse_dn13 = assign34330_e49417_d_n13;
        var_qse_dn15 = assign34330_e49417_d_n15;
        var_qse_dn16 = assign34330_e49417_d_n16;
        var_qse_dn17 = assign34330_e49417_d_n17;
        var_qse_dn18 = assign34330_e49417_d_n18;

        let (assign34340_e49434, assign34340_e49434_d_n0, assign34340_e49434_d_n2, assign34340_e49434_d_n6, assign34340_e49434_d_n7, assign34340_e49434_d_n10, assign34340_e49434_d_n11, assign34340_e49434_d_n12, assign34340_e49434_d_n13, assign34340_e49434_d_n15, assign34340_e49434_d_n16, assign34340_e49434_d_n17, assign34340_e49434_d_n18,) = {
    if ((var_flg_nqs == 0.0) && (var_guard1130 == 0.0)) {
        let assign34340_e49425: f64 = (-var_qsub);
        let assign34340_e49427: f64 = (assign34340_e49425 - var_qi);
        let assign34340_e49429: f64 = (assign34340_e49427 - var_qs_fb);
        let assign34340_e49431: f64 = (assign34340_e49429 - var_qd_fb);
        let assign34340_e49432: f64 = (var_mfactor * assign34340_e49431);
        (assign34340_e49432, (var_mfactor * ((((-var_qsub_dn0) - var_qi_dn0) - var_qs_fb_dn0) - var_qd_fb_dn0)), (var_mfactor * ((((-var_qsub_dn2) - var_qi_dn2) - var_qs_fb_dn2) - var_qd_fb_dn2)), (var_mfactor * ((((-var_qsub_dn6) - var_qi_dn6) - var_qs_fb_dn6) - var_qd_fb_dn6)), (var_mfactor * ((((-var_qsub_dn7) - var_qi_dn7) - var_qs_fb_dn7) - var_qd_fb_dn7)), (var_mfactor * ((((-var_qsub_dn10) - var_qi_dn10) - var_qs_fb_dn10) - var_qd_fb_dn10)), (var_mfactor * ((((-var_qsub_dn11) - var_qi_dn11) - var_qs_fb_dn11) - var_qd_fb_dn11)), (var_mfactor * ((((-var_qsub_dn12) - var_qi_dn12) - var_qs_fb_dn12) - var_qd_fb_dn12)), (var_mfactor * ((-var_qs_fb_dn13) - var_qd_fb_dn13)), (var_mfactor * ((-var_qs_fb_dn15) - var_qd_fb_dn15)), (var_mfactor * ((-var_qs_fb_dn16) - var_qd_fb_dn16)), (var_mfactor * ((((-var_qsub_dn17) - var_qi_dn17) - var_qs_fb_dn17) - var_qd_fb_dn17)), (var_mfactor * ((-var_qs_fb_dn18) - var_qd_fb_dn18)),)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn6, var_qge_dn7, var_qge_dn10, var_qge_dn11, var_qge_dn12, var_qge_dn13, var_qge_dn15, var_qge_dn16, var_qge_dn17, var_qge_dn18,)
    }
};
        var_qge = assign34340_e49434;
        var_qge_dn0 = assign34340_e49434_d_n0;
        var_qge_dn2 = assign34340_e49434_d_n2;
        var_qge_dn6 = assign34340_e49434_d_n6;
        var_qge_dn7 = assign34340_e49434_d_n7;
        var_qge_dn10 = assign34340_e49434_d_n10;
        var_qge_dn11 = assign34340_e49434_d_n11;
        var_qge_dn12 = assign34340_e49434_d_n12;
        var_qge_dn13 = assign34340_e49434_d_n13;
        var_qge_dn15 = assign34340_e49434_d_n15;
        var_qge_dn16 = assign34340_e49434_d_n16;
        var_qge_dn17 = assign34340_e49434_d_n17;
        var_qge_dn18 = assign34340_e49434_d_n18;

        let (assign34350_e49446, assign34350_e49446_d_n0, assign34350_e49446_d_n2, assign34350_e49446_d_n6, assign34350_e49446_d_n7, assign34350_e49446_d_n10, assign34350_e49446_d_n11, assign34350_e49446_d_n12, assign34350_e49446_d_n13, assign34350_e49446_d_n15, assign34350_e49446_d_n16, assign34350_e49446_d_n17, assign34350_e49446_d_n18,) = {
    if ((var_flg_nqs == 0.0) && (var_guard1130 == 0.0)) {
        let assign34350_e49443: f64 = (var_qd + var_qd_fb);
        let assign34350_e49444: f64 = (var_mfactor * assign34350_e49443);
        (assign34350_e49444, (var_mfactor * (var_qd_dn0 + var_qd_fb_dn0)), (var_mfactor * (var_qd_dn2 + var_qd_fb_dn2)), (var_mfactor * (var_qd_dn6 + var_qd_fb_dn6)), (var_mfactor * (var_qd_dn7 + var_qd_fb_dn7)), (var_mfactor * (var_qd_dn10 + var_qd_fb_dn10)), (var_mfactor * (var_qd_dn11 + var_qd_fb_dn11)), (var_mfactor * (var_qd_dn12 + var_qd_fb_dn12)), (var_mfactor * (var_qd_dn13 + var_qd_fb_dn13)), (var_mfactor * (var_qd_dn15 + var_qd_fb_dn15)), (var_mfactor * (var_qd_dn16 + var_qd_fb_dn16)), (var_mfactor * (var_qd_dn17 + var_qd_fb_dn17)), (var_mfactor * (var_qd_dn18 + var_qd_fb_dn18)),)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn6, var_qde_dn7, var_qde_dn10, var_qde_dn11, var_qde_dn12, var_qde_dn13, var_qde_dn15, var_qde_dn16, var_qde_dn17, var_qde_dn18,)
    }
};
        var_qde = assign34350_e49446;
        var_qde_dn0 = assign34350_e49446_d_n0;
        var_qde_dn2 = assign34350_e49446_d_n2;
        var_qde_dn6 = assign34350_e49446_d_n6;
        var_qde_dn7 = assign34350_e49446_d_n7;
        var_qde_dn10 = assign34350_e49446_d_n10;
        var_qde_dn11 = assign34350_e49446_d_n11;
        var_qde_dn12 = assign34350_e49446_d_n12;
        var_qde_dn13 = assign34350_e49446_d_n13;
        var_qde_dn15 = assign34350_e49446_d_n15;
        var_qde_dn16 = assign34350_e49446_d_n16;
        var_qde_dn17 = assign34350_e49446_d_n17;
        var_qde_dn18 = assign34350_e49446_d_n18;

        let (assign34360_e49460, assign34360_e49460_d_n0, assign34360_e49460_d_n2, assign34360_e49460_d_n6, assign34360_e49460_d_n7, assign34360_e49460_d_n10, assign34360_e49460_d_n11, assign34360_e49460_d_n12, assign34360_e49460_d_n13, assign34360_e49460_d_n15, assign34360_e49460_d_n16, assign34360_e49460_d_n17, assign34360_e49460_d_n18,) = {
    if ((var_flg_nqs == 0.0) && (var_guard1130 == 0.0)) {
        let assign34360_e49455: f64 = (var_qi - var_qd);
        let assign34360_e49457: f64 = (assign34360_e49455 + var_qs_fb);
        let assign34360_e49458: f64 = (var_mfactor * assign34360_e49457);
        (assign34360_e49458, (var_mfactor * ((var_qi_dn0 - var_qd_dn0) + var_qs_fb_dn0)), (var_mfactor * ((var_qi_dn2 - var_qd_dn2) + var_qs_fb_dn2)), (var_mfactor * ((var_qi_dn6 - var_qd_dn6) + var_qs_fb_dn6)), (var_mfactor * ((var_qi_dn7 - var_qd_dn7) + var_qs_fb_dn7)), (var_mfactor * ((var_qi_dn10 - var_qd_dn10) + var_qs_fb_dn10)), (var_mfactor * ((var_qi_dn11 - var_qd_dn11) + var_qs_fb_dn11)), (var_mfactor * ((var_qi_dn12 - var_qd_dn12) + var_qs_fb_dn12)), (var_mfactor * ((-var_qd_dn13) + var_qs_fb_dn13)), (var_mfactor * ((-var_qd_dn15) + var_qs_fb_dn15)), (var_mfactor * ((-var_qd_dn16) + var_qs_fb_dn16)), (var_mfactor * ((var_qi_dn17 - var_qd_dn17) + var_qs_fb_dn17)), (var_mfactor * ((-var_qd_dn18) + var_qs_fb_dn18)),)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn6, var_qse_dn7, var_qse_dn10, var_qse_dn11, var_qse_dn12, var_qse_dn13, var_qse_dn15, var_qse_dn16, var_qse_dn17, var_qse_dn18,)
    }
};
        var_qse = assign34360_e49460;
        var_qse_dn0 = assign34360_e49460_d_n0;
        var_qse_dn2 = assign34360_e49460_d_n2;
        var_qse_dn6 = assign34360_e49460_d_n6;
        var_qse_dn7 = assign34360_e49460_d_n7;
        var_qse_dn10 = assign34360_e49460_d_n10;
        var_qse_dn11 = assign34360_e49460_d_n11;
        var_qse_dn12 = assign34360_e49460_d_n12;
        var_qse_dn13 = assign34360_e49460_d_n13;
        var_qse_dn15 = assign34360_e49460_d_n15;
        var_qse_dn16 = assign34360_e49460_d_n16;
        var_qse_dn17 = assign34360_e49460_d_n17;
        var_qse_dn18 = assign34360_e49460_d_n18;

        let assign34370_e49463: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        var_guard1136 = assign34370_e49463;

        let (assign34380_e49467, assign34380_e49467_d_n0, assign34380_e49467_d_n2, assign34380_e49467_d_n6, assign34380_e49467_d_n7, assign34380_e49467_d_n10, assign34380_e49467_d_n11, assign34380_e49467_d_n12, assign34380_e49467_d_n17,) = {
    if (var_guard1136 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qy, var_qy_dn0, var_qy_dn2, var_qy_dn6, var_qy_dn7, var_qy_dn10, var_qy_dn11, var_qy_dn12, var_qy_dn17,)
    }
};
        var_qy = assign34380_e49467;
        var_qy_dn0 = assign34380_e49467_d_n0;
        var_qy_dn2 = assign34380_e49467_d_n2;
        var_qy_dn6 = assign34380_e49467_d_n6;
        var_qy_dn7 = assign34380_e49467_d_n7;
        var_qy_dn10 = assign34380_e49467_d_n10;
        var_qy_dn11 = assign34380_e49467_d_n11;
        var_qy_dn12 = assign34380_e49467_d_n12;
        var_qy_dn17 = assign34380_e49467_d_n17;

        let (assign34390_e49476, assign34390_e49476_d_n0, assign34390_e49476_d_n2, assign34390_e49476_d_n6, assign34390_e49476_d_n7, assign34390_e49476_d_n10, assign34390_e49476_d_n11, assign34390_e49476_d_n12, assign34390_e49476_d_n17,) = {
    if (var_guard1136 == 0.0) {
        let assign34390_e49472: f64 = (var_ec * var_leff);
        let assign34390_e49474: f64 = (assign34390_e49472 + var_ps0);
        (assign34390_e49474, ((var_ec_dn0 * var_leff) + var_ps0_dn0), ((var_ec_dn2 * var_leff) + var_ps0_dn2), ((var_ec_dn6 * var_leff) + var_ps0_dn6), ((var_ec_dn7 * var_leff) + var_ps0_dn7), ((var_ec_dn10 * var_leff) + var_ps0_dn10), ((var_ec_dn11 * var_leff) + var_ps0_dn11), ((var_ec_dn12 * var_leff) + var_ps0_dn12), ((var_ec_dn17 * var_leff) + var_ps0_dn17),)
    } else {
        (var_pslk, var_pslk_dn0, var_pslk_dn2, var_pslk_dn6, var_pslk_dn7, var_pslk_dn10, var_pslk_dn11, var_pslk_dn12, var_pslk_dn17,)
    }
};
        var_pslk = assign34390_e49476;
        var_pslk_dn0 = assign34390_e49476_d_n0;
        var_pslk_dn2 = assign34390_e49476_d_n2;
        var_pslk_dn6 = assign34390_e49476_d_n6;
        var_pslk_dn7 = assign34390_e49476_d_n7;
        var_pslk_dn10 = assign34390_e49476_d_n10;
        var_pslk_dn11 = assign34390_e49476_d_n11;
        var_pslk_dn12 = assign34390_e49476_d_n12;
        var_pslk_dn17 = assign34390_e49476_d_n17;

        let assign34400_e49479: f64 = if var_pslk > var_psdl { 1.0 } else { 0.0 };
        var_guard1137 = assign34400_e49479;

        let (assign34410_e49486, assign34410_e49486_d_n0, assign34410_e49486_d_n2, assign34410_e49486_d_n6, assign34410_e49486_d_n7, assign34410_e49486_d_n10, assign34410_e49486_d_n11, assign34410_e49486_d_n12, assign34410_e49486_d_n17,) = {
    if ((var_guard1136 == 0.0) && (var_guard1137 != 0.0)) {
        (var_psdl, var_psdl_dn0, var_psdl_dn2, var_psdl_dn6, var_psdl_dn7, var_psdl_dn10, var_psdl_dn11, var_psdl_dn12, var_psdl_dn17,)
    } else {
        (var_pslk, var_pslk_dn0, var_pslk_dn2, var_pslk_dn6, var_pslk_dn7, var_pslk_dn10, var_pslk_dn11, var_pslk_dn12, var_pslk_dn17,)
    }
};
        var_pslk = assign34410_e49486;
        var_pslk_dn0 = assign34410_e49486_d_n0;
        var_pslk_dn2 = assign34410_e49486_d_n2;
        var_pslk_dn6 = assign34410_e49486_d_n6;
        var_pslk_dn7 = assign34410_e49486_d_n7;
        var_pslk_dn10 = assign34410_e49486_d_n10;
        var_pslk_dn11 = assign34410_e49486_d_n11;
        var_pslk_dn12 = assign34410_e49486_d_n12;
        var_pslk_dn17 = assign34410_e49486_d_n17;

        let (assign34420_e49501, assign34420_e49501_d_n0, assign34420_e49501_d_n2, assign34420_e49501_d_n6, assign34420_e49501_d_n7, assign34420_e49501_d_n10, assign34420_e49501_d_n11, assign34420_e49501_d_n12, assign34420_e49501_d_n17,) = {
    if (var_guard1136 == 0.0) {
        let assign34420_e49492: f64 = (var_vds + var_ps0);
        let assign34420_e49493: f64 = (var_aclm * assign34420_e49492);
        let assign34420_e49496: f64 = (1.0 - var_aclm);
        let assign34420_e49498: f64 = (assign34420_e49496 * var_pslk);
        let assign34420_e49499: f64 = (assign34420_e49493 + assign34420_e49498);
        (assign34420_e49499, ((var_aclm * (var_vds_dn0 + var_ps0_dn0)) + (assign34420_e49496 * var_pslk_dn0)), ((var_aclm * (var_vds_dn2 + var_ps0_dn2)) + (assign34420_e49496 * var_pslk_dn2)), ((var_aclm * (var_vds_dn6 + var_ps0_dn6)) + (assign34420_e49496 * var_pslk_dn6)), ((var_aclm * (var_vds_dn7 + var_ps0_dn7)) + (assign34420_e49496 * var_pslk_dn7)), ((var_aclm * (var_vds_dn10 + var_ps0_dn10)) + (assign34420_e49496 * var_pslk_dn10)), ((var_aclm * (var_vds_dn11 + var_ps0_dn11)) + (assign34420_e49496 * var_pslk_dn11)), ((var_aclm * (var_vds_dn12 + var_ps0_dn12)) + (assign34420_e49496 * var_pslk_dn12)), ((var_aclm * (var_vds_dn17 + var_ps0_dn17)) + (assign34420_e49496 * var_pslk_dn17)),)
    } else {
        (var_t1__blk1132, var_t1__blk1132_dn0, var_t1__blk1132_dn2, var_t1__blk1132_dn6, var_t1__blk1132_dn7, var_t1__blk1132_dn10, var_t1__blk1132_dn11, var_t1__blk1132_dn12, var_t1__blk1132_dn17,)
    }
};
        var_t1__blk1132 = assign34420_e49501;
        var_t1__blk1132_dn0 = assign34420_e49501_d_n0;
        var_t1__blk1132_dn2 = assign34420_e49501_d_n2;
        var_t1__blk1132_dn6 = assign34420_e49501_d_n6;
        var_t1__blk1132_dn7 = assign34420_e49501_d_n7;
        var_t1__blk1132_dn10 = assign34420_e49501_d_n10;
        var_t1__blk1132_dn11 = assign34420_e49501_d_n11;
        var_t1__blk1132_dn12 = assign34420_e49501_d_n12;
        var_t1__blk1132_dn17 = assign34420_e49501_d_n17;

        let (assign34430_e49511, assign34430_e49511_d_n0, assign34430_e49511_d_n2, assign34430_e49511_d_n6, assign34430_e49511_d_n7, assign34430_e49511_d_n10, assign34430_e49511_d_n11, assign34430_e49511_d_n12, assign34430_e49511_d_n17,) = {
    if (var_guard1136 == 0.0) {
        let assign34430_e49506: f64 = (2.0 * 1.034943e-10);
        let assign34430_e49508: f64 = (assign34430_e49506 / var_q_nsub);
        let assign34430_e49509: f64 = (assign34430_e49508).sqrt();
        (assign34430_e49509, ((-((assign34430_e49506 * var_q_nsub_dn0) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * var_q_nsub_dn2) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * var_q_nsub_dn6) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * var_q_nsub_dn7) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * var_q_nsub_dn10) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * var_q_nsub_dn11) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * var_q_nsub_dn12) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * var_q_nsub_dn17) / (var_q_nsub * var_q_nsub))) / (2.0 * assign34430_e49509)),)
    } else {
        (var_t10__blk1133, var_t10__blk1133_dn0, var_t10__blk1133_dn2, var_t10__blk1133_dn6, var_t10__blk1133_dn7, var_t10__blk1133_dn10, var_t10__blk1133_dn11, var_t10__blk1133_dn12, var_t10__blk1133_dn17,)
    }
};
        var_t10__blk1133 = assign34430_e49511;
        var_t10__blk1133_dn0 = assign34430_e49511_d_n0;
        var_t10__blk1133_dn2 = assign34430_e49511_d_n2;
        var_t10__blk1133_dn6 = assign34430_e49511_d_n6;
        var_t10__blk1133_dn7 = assign34430_e49511_d_n7;
        var_t10__blk1133_dn10 = assign34430_e49511_d_n10;
        var_t10__blk1133_dn11 = assign34430_e49511_d_n11;
        var_t10__blk1133_dn12 = assign34430_e49511_d_n12;
        var_t10__blk1133_dn17 = assign34430_e49511_d_n17;

        let (assign34440_e49518, assign34440_e49518_d_n0, assign34440_e49518_d_n2, assign34440_e49518_d_n6, assign34440_e49518_d_n7, assign34440_e49518_d_n10, assign34440_e49518_d_n11, assign34440_e49518_d_n12, assign34440_e49518_d_n17,) = {
    if (var_guard1136 == 0.0) {
        let assign34440_e49516: f64 = (var_t10__blk1133 * 1.3);
        (assign34440_e49516, (var_t10__blk1133_dn0 * 1.3), (var_t10__blk1133_dn2 * 1.3), (var_t10__blk1133_dn6 * 1.3), (var_t10__blk1133_dn7 * 1.3), (var_t10__blk1133_dn10 * 1.3), (var_t10__blk1133_dn11 * 1.3), (var_t10__blk1133_dn12 * 1.3), (var_t10__blk1133_dn17 * 1.3),)
    } else {
        (var_t3__blk1134, var_t3__blk1134_dn0, var_t3__blk1134_dn2, var_t3__blk1134_dn6, var_t3__blk1134_dn7, var_t3__blk1134_dn10, var_t3__blk1134_dn11, var_t3__blk1134_dn12, var_t3__blk1134_dn17,)
    }
};
        var_t3__blk1134 = assign34440_e49518;
        var_t3__blk1134_dn0 = assign34440_e49518_d_n0;
        var_t3__blk1134_dn2 = assign34440_e49518_d_n2;
        var_t3__blk1134_dn6 = assign34440_e49518_d_n6;
        var_t3__blk1134_dn7 = assign34440_e49518_d_n7;
        var_t3__blk1134_dn10 = assign34440_e49518_d_n10;
        var_t3__blk1134_dn11 = assign34440_e49518_d_n11;
        var_t3__blk1134_dn12 = assign34440_e49518_d_n12;
        var_t3__blk1134_dn17 = assign34440_e49518_d_n17;

        let (assign34450_e49527, assign34450_e49527_d_n0, assign34450_e49527_d_n2, assign34450_e49527_d_n6, assign34450_e49527_d_n7, assign34450_e49527_d_n10, assign34450_e49527_d_n11, assign34450_e49527_d_n12, assign34450_e49527_d_n17,) = {
    if (var_guard1136 == 0.0) {
        let assign34450_e49523: f64 = (1.034943e-10 * var_weffcv_nf);
        let assign34450_e49525: f64 = (assign34450_e49523 * var_t3__blk1134);
        (assign34450_e49525, (assign34450_e49523 * var_t3__blk1134_dn0), (assign34450_e49523 * var_t3__blk1134_dn2), (assign34450_e49523 * var_t3__blk1134_dn6), (assign34450_e49523 * var_t3__blk1134_dn7), (assign34450_e49523 * var_t3__blk1134_dn10), (assign34450_e49523 * var_t3__blk1134_dn11), (assign34450_e49523 * var_t3__blk1134_dn12), (assign34450_e49523 * var_t3__blk1134_dn17),)
    } else {
        (var_t2__blk1135, var_t2__blk1135_dn0, var_t2__blk1135_dn2, var_t2__blk1135_dn6, var_t2__blk1135_dn7, var_t2__blk1135_dn10, var_t2__blk1135_dn11, var_t2__blk1135_dn12, var_t2__blk1135_dn17,)
    }
};
        var_t2__blk1135 = assign34450_e49527;
        var_t2__blk1135_dn0 = assign34450_e49527_d_n0;
        var_t2__blk1135_dn2 = assign34450_e49527_d_n2;
        var_t2__blk1135_dn6 = assign34450_e49527_d_n6;
        var_t2__blk1135_dn7 = assign34450_e49527_d_n7;
        var_t2__blk1135_dn10 = assign34450_e49527_d_n10;
        var_t2__blk1135_dn11 = assign34450_e49527_d_n11;
        var_t2__blk1135_dn12 = assign34450_e49527_d_n12;
        var_t2__blk1135_dn17 = assign34450_e49527_d_n17;

        let (assign34460_e49542, assign34460_e49542_d_n0, assign34460_e49542_d_n2, assign34460_e49542_d_n6, assign34460_e49542_d_n7, assign34460_e49542_d_n10, assign34460_e49542_d_n11, assign34460_e49542_d_n12, assign34460_e49542_d_n17,) = {
    if (var_guard1136 == 0.0) {
        let assign34460_e49532: f64 = (var_ps0 + var_vds);
        let assign34460_e49534: f64 = (assign34460_e49532 - var_t1__blk1132);
        let assign34460_e49536: f64 = (assign34460_e49534 / p.p64);
        let assign34460_e49538: f64 = (assign34460_e49536 - var_ec);
        let assign34460_e49540: f64 = (assign34460_e49538 * var_t2__blk1135);
        (assign34460_e49540, ((((((var_ps0_dn0 + var_vds_dn0) - var_t1__blk1132_dn0) / p.p64) - var_ec_dn0) * var_t2__blk1135) + (assign34460_e49538 * var_t2__blk1135_dn0)), ((((((var_ps0_dn2 + var_vds_dn2) - var_t1__blk1132_dn2) / p.p64) - var_ec_dn2) * var_t2__blk1135) + (assign34460_e49538 * var_t2__blk1135_dn2)), ((((((var_ps0_dn6 + var_vds_dn6) - var_t1__blk1132_dn6) / p.p64) - var_ec_dn6) * var_t2__blk1135) + (assign34460_e49538 * var_t2__blk1135_dn6)), ((((((var_ps0_dn7 + var_vds_dn7) - var_t1__blk1132_dn7) / p.p64) - var_ec_dn7) * var_t2__blk1135) + (assign34460_e49538 * var_t2__blk1135_dn7)), ((((((var_ps0_dn10 + var_vds_dn10) - var_t1__blk1132_dn10) / p.p64) - var_ec_dn10) * var_t2__blk1135) + (assign34460_e49538 * var_t2__blk1135_dn10)), ((((((var_ps0_dn11 + var_vds_dn11) - var_t1__blk1132_dn11) / p.p64) - var_ec_dn11) * var_t2__blk1135) + (assign34460_e49538 * var_t2__blk1135_dn11)), ((((((var_ps0_dn12 + var_vds_dn12) - var_t1__blk1132_dn12) / p.p64) - var_ec_dn12) * var_t2__blk1135) + (assign34460_e49538 * var_t2__blk1135_dn12)), ((((((var_ps0_dn17 + var_vds_dn17) - var_t1__blk1132_dn17) / p.p64) - var_ec_dn17) * var_t2__blk1135) + (assign34460_e49538 * var_t2__blk1135_dn17)),)
    } else {
        (var_qy, var_qy_dn0, var_qy_dn2, var_qy_dn6, var_qy_dn7, var_qy_dn10, var_qy_dn11, var_qy_dn12, var_qy_dn17,)
    }
};
        var_qy = assign34460_e49542;
        var_qy_dn0 = assign34460_e49542_d_n0;
        var_qy_dn2 = assign34460_e49542_d_n2;
        var_qy_dn6 = assign34460_e49542_d_n6;
        var_qy_dn7 = assign34460_e49542_d_n7;
        var_qy_dn10 = assign34460_e49542_d_n10;
        var_qy_dn11 = assign34460_e49542_d_n11;
        var_qy_dn12 = assign34460_e49542_d_n12;
        var_qy_dn17 = assign34460_e49542_d_n17;

        let assign34470_e49545: f64 = if p.p65 != 0.0 { 1.0 } else { 0.0 };
        var_guard1138 = assign34470_e49545;

        let (assign34480_e49553, assign34480_e49553_d_n0, assign34480_e49553_d_n2, assign34480_e49553_d_n6, assign34480_e49553_d_n7, assign34480_e49553_d_n10, assign34480_e49553_d_n11, assign34480_e49553_d_n12, assign34480_e49553_d_n17,) = {
    if (var_guard1138 != 0.0) {
        let assign34480_e49550: f64 = (var_cqyb0 * var_vbsp);
        let assign34480_e49551: f64 = (var_qy + assign34480_e49550);
        (assign34480_e49551, (var_qy_dn0 + (var_cqyb0 * var_vbsp_dn0)), (var_qy_dn2 + (var_cqyb0 * var_vbsp_dn2)), (var_qy_dn6 + (var_cqyb0 * var_vbsp_dn6)), (var_qy_dn7 + (var_cqyb0 * var_vbsp_dn7)), (var_qy_dn10 + (var_cqyb0 * var_vbsp_dn10)), (var_qy_dn11 + (var_cqyb0 * var_vbsp_dn11)), (var_qy_dn12 + (var_cqyb0 * var_vbsp_dn12)), (var_qy_dn17 + (var_cqyb0 * var_vbsp_dn17)),)
    } else {
        (var_qy, var_qy_dn0, var_qy_dn2, var_qy_dn6, var_qy_dn7, var_qy_dn10, var_qy_dn11, var_qy_dn12, var_qy_dn17,)
    }
};
        var_qy = assign34480_e49553;
        var_qy_dn0 = assign34480_e49553_d_n0;
        var_qy_dn2 = assign34480_e49553_d_n2;
        var_qy_dn6 = assign34480_e49553_d_n6;
        var_qy_dn7 = assign34480_e49553_d_n7;
        var_qy_dn10 = assign34480_e49553_d_n10;
        var_qy_dn11 = assign34480_e49553_d_n11;
        var_qy_dn12 = assign34480_e49553_d_n12;
        var_qy_dn17 = assign34480_e49553_d_n17;

        let assign34490_e49556: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        var_guard1139 = assign34490_e49556;

        let assign34500_e49559: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1140 = assign34500_e49559;

        let (assign34510_e49572, assign34510_e49572_d_n0, assign34510_e49572_d_n2, assign34510_e49572_d_n6, assign34510_e49572_d_n7, assign34510_e49572_d_n10, assign34510_e49572_d_n11, assign34510_e49572_d_n12, assign34510_e49572_d_n17,) = {
    if ((var_guard1139 != 0.0) && (var_guard1140 != 0.0)) {
        let assign34510_e49564: f64 = (-var_qbody_bt_p_sus);
        let assign34510_e49566: f64 = (assign34510_e49564 - var_qbody_bt_p_sud);
        let assign34510_e49568: f64 = (assign34510_e49566 - var_qbody_bt_n_sus);
        let assign34510_e49570: f64 = (assign34510_e49568 - var_qbody_bt_n_sud);
        (assign34510_e49570, ((((-var_qbody_bt_p_sus_dn0) - var_qbody_bt_p_sud_dn0) - var_qbody_bt_n_sus_dn0) - var_qbody_bt_n_sud_dn0), ((((-var_qbody_bt_p_sus_dn2) - var_qbody_bt_p_sud_dn2) - var_qbody_bt_n_sus_dn2) - var_qbody_bt_n_sud_dn2), ((((-var_qbody_bt_p_sus_dn6) - var_qbody_bt_p_sud_dn6) - var_qbody_bt_n_sus_dn6) - var_qbody_bt_n_sud_dn6), ((((-var_qbody_bt_p_sus_dn7) - var_qbody_bt_p_sud_dn7) - var_qbody_bt_n_sus_dn7) - var_qbody_bt_n_sud_dn7), ((((-var_qbody_bt_p_sus_dn10) - var_qbody_bt_p_sud_dn10) - var_qbody_bt_n_sus_dn10) - var_qbody_bt_n_sud_dn10), ((((-var_qbody_bt_p_sus_dn11) - var_qbody_bt_p_sud_dn11) - var_qbody_bt_n_sus_dn11) - var_qbody_bt_n_sud_dn11), ((((-var_qbody_bt_p_sus_dn12) - var_qbody_bt_p_sud_dn12) - var_qbody_bt_n_sus_dn12) - var_qbody_bt_n_sud_dn12), ((((-var_qbody_bt_p_sus_dn17) - var_qbody_bt_p_sud_dn17) - var_qbody_bt_n_sus_dn17) - var_qbody_bt_n_sud_dn17),)
    } else {
        (var_q_bt_ge, var_q_bt_ge_dn0, var_q_bt_ge_dn2, var_q_bt_ge_dn6, var_q_bt_ge_dn7, var_q_bt_ge_dn10, var_q_bt_ge_dn11, var_q_bt_ge_dn12, var_q_bt_ge_dn17,)
    }
};
        var_q_bt_ge = assign34510_e49572;
        var_q_bt_ge_dn0 = assign34510_e49572_d_n0;
        var_q_bt_ge_dn2 = assign34510_e49572_d_n2;
        var_q_bt_ge_dn6 = assign34510_e49572_d_n6;
        var_q_bt_ge_dn7 = assign34510_e49572_d_n7;
        var_q_bt_ge_dn10 = assign34510_e49572_d_n10;
        var_q_bt_ge_dn11 = assign34510_e49572_d_n11;
        var_q_bt_ge_dn12 = assign34510_e49572_d_n12;
        var_q_bt_ge_dn17 = assign34510_e49572_d_n17;

        let (assign34520_e49580, assign34520_e49580_d_n0, assign34520_e49580_d_n2, assign34520_e49580_d_n6, assign34520_e49580_d_n7, assign34520_e49580_d_n10, assign34520_e49580_d_n11, assign34520_e49580_d_n12, assign34520_e49580_d_n17,) = {
    if ((var_guard1139 != 0.0) && (var_guard1140 != 0.0)) {
        let assign34520_e49578: f64 = (var_qbody_bt_p_iud + var_qbody_bt_n_iud);
        (assign34520_e49578, (var_qbody_bt_p_iud_dn0 + var_qbody_bt_n_iud_dn0), (var_qbody_bt_p_iud_dn2 + var_qbody_bt_n_iud_dn2), (var_qbody_bt_p_iud_dn6 + var_qbody_bt_n_iud_dn6), (var_qbody_bt_p_iud_dn7 + var_qbody_bt_n_iud_dn7), (var_qbody_bt_p_iud_dn10 + var_qbody_bt_n_iud_dn10), (var_qbody_bt_p_iud_dn11 + var_qbody_bt_n_iud_dn11), (var_qbody_bt_p_iud_dn12 + var_qbody_bt_n_iud_dn12), (var_qbody_bt_p_iud_dn17 + var_qbody_bt_n_iud_dn17),)
    } else {
        (var_q_bt_de, var_q_bt_de_dn0, var_q_bt_de_dn2, var_q_bt_de_dn6, var_q_bt_de_dn7, var_q_bt_de_dn10, var_q_bt_de_dn11, var_q_bt_de_dn12, var_q_bt_de_dn17,)
    }
};
        var_q_bt_de = assign34520_e49580;
        var_q_bt_de_dn0 = assign34520_e49580_d_n0;
        var_q_bt_de_dn2 = assign34520_e49580_d_n2;
        var_q_bt_de_dn6 = assign34520_e49580_d_n6;
        var_q_bt_de_dn7 = assign34520_e49580_d_n7;
        var_q_bt_de_dn10 = assign34520_e49580_d_n10;
        var_q_bt_de_dn11 = assign34520_e49580_d_n11;
        var_q_bt_de_dn12 = assign34520_e49580_d_n12;
        var_q_bt_de_dn17 = assign34520_e49580_d_n17;

        let (assign34530_e49588, assign34530_e49588_d_n0, assign34530_e49588_d_n2, assign34530_e49588_d_n6, assign34530_e49588_d_n7, assign34530_e49588_d_n10, assign34530_e49588_d_n11, assign34530_e49588_d_n12, assign34530_e49588_d_n17,) = {
    if ((var_guard1139 != 0.0) && (var_guard1140 != 0.0)) {
        let assign34530_e49586: f64 = (var_qbody_bt_p_ius + var_qbody_bt_n_ius);
        (assign34530_e49586, (var_qbody_bt_p_ius_dn0 + var_qbody_bt_n_ius_dn0), (var_qbody_bt_p_ius_dn2 + var_qbody_bt_n_ius_dn2), (var_qbody_bt_p_ius_dn6 + var_qbody_bt_n_ius_dn6), (var_qbody_bt_p_ius_dn7 + var_qbody_bt_n_ius_dn7), (var_qbody_bt_p_ius_dn10 + var_qbody_bt_n_ius_dn10), (var_qbody_bt_p_ius_dn11 + var_qbody_bt_n_ius_dn11), (var_qbody_bt_p_ius_dn12 + var_qbody_bt_n_ius_dn12), (var_qbody_bt_p_ius_dn17 + var_qbody_bt_n_ius_dn17),)
    } else {
        (var_q_bt_se, var_q_bt_se_dn0, var_q_bt_se_dn2, var_q_bt_se_dn6, var_q_bt_se_dn7, var_q_bt_se_dn10, var_q_bt_se_dn11, var_q_bt_se_dn12, var_q_bt_se_dn17,)
    }
};
        var_q_bt_se = assign34530_e49588;
        var_q_bt_se_dn0 = assign34530_e49588_d_n0;
        var_q_bt_se_dn2 = assign34530_e49588_d_n2;
        var_q_bt_se_dn6 = assign34530_e49588_d_n6;
        var_q_bt_se_dn7 = assign34530_e49588_d_n7;
        var_q_bt_se_dn10 = assign34530_e49588_d_n10;
        var_q_bt_se_dn11 = assign34530_e49588_d_n11;
        var_q_bt_se_dn12 = assign34530_e49588_d_n12;
        var_q_bt_se_dn17 = assign34530_e49588_d_n17;

        let (assign34540_e49610, assign34540_e49610_d_n0, assign34540_e49610_d_n2, assign34540_e49610_d_n6, assign34540_e49610_d_n7, assign34540_e49610_d_n10, assign34540_e49610_d_n11, assign34540_e49610_d_n12, assign34540_e49610_d_n13, assign34540_e49610_d_n15, assign34540_e49610_d_n16, assign34540_e49610_d_n17, assign34540_e49610_d_n18,) = {
    if ((var_guard1139 != 0.0) && (var_guard1140 != 0.0)) {
        let assign34540_e49596: f64 = (var_qgod + var_qgos);
        let assign34540_e49598: f64 = (assign34540_e49596 + var_qgob);
        let assign34540_e49600: f64 = (assign34540_e49598 - var_qy);
        let assign34540_e49602: f64 = (assign34540_e49600 - var_qovs);
        let assign34540_e49604: f64 = (assign34540_e49602 - var_qovd);
        let assign34540_e49606: f64 = (assign34540_e49604 + var_q_bt_ge);
        let assign34540_e49607: f64 = (var_mfactor * assign34540_e49606);
        let assign34540_e49608: f64 = (var_qge + assign34540_e49607);
        (assign34540_e49608, (var_qge_dn0 + (var_mfactor * ((((((var_qgod_dn0 + var_qgos_dn0) + var_qgob_dn0) - var_qy_dn0) - var_qovs_dn0) - var_qovd_dn0) + var_q_bt_ge_dn0))), (var_qge_dn2 + (var_mfactor * ((((((var_qgod_dn2 + var_qgos_dn2) + var_qgob_dn2) - var_qy_dn2) - var_qovs_dn2) - var_qovd_dn2) + var_q_bt_ge_dn2))), (var_qge_dn6 + (var_mfactor * ((((((var_qgod_dn6 + var_qgos_dn6) + var_qgob_dn6) - var_qy_dn6) - var_qovs_dn6) - var_qovd_dn6) + var_q_bt_ge_dn6))), (var_qge_dn7 + (var_mfactor * ((((((var_qgod_dn7 + var_qgos_dn7) + var_qgob_dn7) - var_qy_dn7) - var_qovs_dn7) - var_qovd_dn7) + var_q_bt_ge_dn7))), (var_qge_dn10 + (var_mfactor * ((((((var_qgod_dn10 + var_qgos_dn10) + var_qgob_dn10) - var_qy_dn10) - var_qovs_dn10) - var_qovd_dn10) + var_q_bt_ge_dn10))), (var_qge_dn11 + (var_mfactor * ((((((var_qgod_dn11 + var_qgos_dn11) + var_qgob_dn11) - var_qy_dn11) - var_qovs_dn11) - var_qovd_dn11) + var_q_bt_ge_dn11))), (var_qge_dn12 + (var_mfactor * ((((((var_qgod_dn12 + var_qgos_dn12) + var_qgob_dn12) - var_qy_dn12) - var_qovs_dn12) - var_qovd_dn12) + var_q_bt_ge_dn12))), var_qge_dn13, var_qge_dn15, var_qge_dn16, (var_qge_dn17 + (var_mfactor * ((((((var_qgod_dn17 + var_qgos_dn17) + var_qgob_dn17) - var_qy_dn17) - var_qovs_dn17) - var_qovd_dn17) + var_q_bt_ge_dn17))), var_qge_dn18,)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn6, var_qge_dn7, var_qge_dn10, var_qge_dn11, var_qge_dn12, var_qge_dn13, var_qge_dn15, var_qge_dn16, var_qge_dn17, var_qge_dn18,)
    }
};
        var_qge = assign34540_e49610;
        var_qge_dn0 = assign34540_e49610_d_n0;
        var_qge_dn2 = assign34540_e49610_d_n2;
        var_qge_dn6 = assign34540_e49610_d_n6;
        var_qge_dn7 = assign34540_e49610_d_n7;
        var_qge_dn10 = assign34540_e49610_d_n10;
        var_qge_dn11 = assign34540_e49610_d_n11;
        var_qge_dn12 = assign34540_e49610_d_n12;
        var_qge_dn13 = assign34540_e49610_d_n13;
        var_qge_dn15 = assign34540_e49610_d_n15;
        var_qge_dn16 = assign34540_e49610_d_n16;
        var_qge_dn17 = assign34540_e49610_d_n17;
        var_qge_dn18 = assign34540_e49610_d_n18;

        let (assign34550_e49627, assign34550_e49627_d_n0, assign34550_e49627_d_n2, assign34550_e49627_d_n6, assign34550_e49627_d_n7, assign34550_e49627_d_n10, assign34550_e49627_d_n11, assign34550_e49627_d_n12, assign34550_e49627_d_n13, assign34550_e49627_d_n15, assign34550_e49627_d_n16, assign34550_e49627_d_n17, assign34550_e49627_d_n18,) = {
    if ((var_guard1139 != 0.0) && (var_guard1140 != 0.0)) {
        let assign34550_e49617: f64 = (-var_qgod);
        let assign34550_e49619: f64 = (assign34550_e49617 + var_qy);
        let assign34550_e49621: f64 = (assign34550_e49619 + var_qbdld);
        let assign34550_e49623: f64 = (assign34550_e49621 + var_q_bt_de);
        let assign34550_e49624: f64 = (var_mfactor * assign34550_e49623);
        let assign34550_e49625: f64 = (var_qde + assign34550_e49624);
        (assign34550_e49625, (var_qde_dn0 + (var_mfactor * ((((-var_qgod_dn0) + var_qy_dn0) + var_qbdld_dn0) + var_q_bt_de_dn0))), (var_qde_dn2 + (var_mfactor * ((((-var_qgod_dn2) + var_qy_dn2) + var_qbdld_dn2) + var_q_bt_de_dn2))), (var_qde_dn6 + (var_mfactor * ((((-var_qgod_dn6) + var_qy_dn6) + var_qbdld_dn6) + var_q_bt_de_dn6))), (var_qde_dn7 + (var_mfactor * ((((-var_qgod_dn7) + var_qy_dn7) + var_qbdld_dn7) + var_q_bt_de_dn7))), (var_qde_dn10 + (var_mfactor * ((((-var_qgod_dn10) + var_qy_dn10) + var_qbdld_dn10) + var_q_bt_de_dn10))), (var_qde_dn11 + (var_mfactor * ((((-var_qgod_dn11) + var_qy_dn11) + var_qbdld_dn11) + var_q_bt_de_dn11))), (var_qde_dn12 + (var_mfactor * ((((-var_qgod_dn12) + var_qy_dn12) + var_qbdld_dn12) + var_q_bt_de_dn12))), var_qde_dn13, var_qde_dn15, var_qde_dn16, (var_qde_dn17 + (var_mfactor * ((((-var_qgod_dn17) + var_qy_dn17) + var_qbdld_dn17) + var_q_bt_de_dn17))), var_qde_dn18,)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn6, var_qde_dn7, var_qde_dn10, var_qde_dn11, var_qde_dn12, var_qde_dn13, var_qde_dn15, var_qde_dn16, var_qde_dn17, var_qde_dn18,)
    }
};
        var_qde = assign34550_e49627;
        var_qde_dn0 = assign34550_e49627_d_n0;
        var_qde_dn2 = assign34550_e49627_d_n2;
        var_qde_dn6 = assign34550_e49627_d_n6;
        var_qde_dn7 = assign34550_e49627_d_n7;
        var_qde_dn10 = assign34550_e49627_d_n10;
        var_qde_dn11 = assign34550_e49627_d_n11;
        var_qde_dn12 = assign34550_e49627_d_n12;
        var_qde_dn13 = assign34550_e49627_d_n13;
        var_qde_dn15 = assign34550_e49627_d_n15;
        var_qde_dn16 = assign34550_e49627_d_n16;
        var_qde_dn17 = assign34550_e49627_d_n17;
        var_qde_dn18 = assign34550_e49627_d_n18;

        let (assign34560_e49642, assign34560_e49642_d_n0, assign34560_e49642_d_n2, assign34560_e49642_d_n6, assign34560_e49642_d_n7, assign34560_e49642_d_n10, assign34560_e49642_d_n11, assign34560_e49642_d_n12, assign34560_e49642_d_n13, assign34560_e49642_d_n15, assign34560_e49642_d_n16, assign34560_e49642_d_n17, assign34560_e49642_d_n18,) = {
    if ((var_guard1139 != 0.0) && (var_guard1140 != 0.0)) {
        let assign34560_e49634: f64 = (-var_qgos);
        let assign34560_e49636: f64 = (assign34560_e49634 + var_qbsld);
        let assign34560_e49638: f64 = (assign34560_e49636 + var_q_bt_se);
        let assign34560_e49639: f64 = (var_mfactor * assign34560_e49638);
        let assign34560_e49640: f64 = (var_qse + assign34560_e49639);
        (assign34560_e49640, (var_qse_dn0 + (var_mfactor * (((-var_qgos_dn0) + var_qbsld_dn0) + var_q_bt_se_dn0))), (var_qse_dn2 + (var_mfactor * (((-var_qgos_dn2) + var_qbsld_dn2) + var_q_bt_se_dn2))), (var_qse_dn6 + (var_mfactor * (((-var_qgos_dn6) + var_qbsld_dn6) + var_q_bt_se_dn6))), (var_qse_dn7 + (var_mfactor * (((-var_qgos_dn7) + var_qbsld_dn7) + var_q_bt_se_dn7))), (var_qse_dn10 + (var_mfactor * (((-var_qgos_dn10) + var_qbsld_dn10) + var_q_bt_se_dn10))), (var_qse_dn11 + (var_mfactor * (((-var_qgos_dn11) + var_qbsld_dn11) + var_q_bt_se_dn11))), (var_qse_dn12 + (var_mfactor * (((-var_qgos_dn12) + var_qbsld_dn12) + var_q_bt_se_dn12))), var_qse_dn13, var_qse_dn15, var_qse_dn16, (var_qse_dn17 + (var_mfactor * (((-var_qgos_dn17) + var_qbsld_dn17) + var_q_bt_se_dn17))), var_qse_dn18,)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn6, var_qse_dn7, var_qse_dn10, var_qse_dn11, var_qse_dn12, var_qse_dn13, var_qse_dn15, var_qse_dn16, var_qse_dn17, var_qse_dn18,)
    }
};
        var_qse = assign34560_e49642;
        var_qse_dn0 = assign34560_e49642_d_n0;
        var_qse_dn2 = assign34560_e49642_d_n2;
        var_qse_dn6 = assign34560_e49642_d_n6;
        var_qse_dn7 = assign34560_e49642_d_n7;
        var_qse_dn10 = assign34560_e49642_d_n10;
        var_qse_dn11 = assign34560_e49642_d_n11;
        var_qse_dn12 = assign34560_e49642_d_n12;
        var_qse_dn13 = assign34560_e49642_d_n13;
        var_qse_dn15 = assign34560_e49642_d_n15;
        var_qse_dn16 = assign34560_e49642_d_n16;
        var_qse_dn17 = assign34560_e49642_d_n17;
        var_qse_dn18 = assign34560_e49642_d_n18;

        *var_guard1130_slot = var_guard1130;
        *var_guard1136_slot = var_guard1136;
        *var_guard1137_slot = var_guard1137;
        *var_guard1138_slot = var_guard1138;
        *var_guard1139_slot = var_guard1139;
        *var_guard1140_slot = var_guard1140;
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
        *var_t10__blk1133_slot = var_t10__blk1133;
        *var_t10__blk1133_dn0_slot = var_t10__blk1133_dn0;
        *var_t10__blk1133_dn10_slot = var_t10__blk1133_dn10;
        *var_t10__blk1133_dn11_slot = var_t10__blk1133_dn11;
        *var_t10__blk1133_dn12_slot = var_t10__blk1133_dn12;
        *var_t10__blk1133_dn17_slot = var_t10__blk1133_dn17;
        *var_t10__blk1133_dn2_slot = var_t10__blk1133_dn2;
        *var_t10__blk1133_dn6_slot = var_t10__blk1133_dn6;
        *var_t10__blk1133_dn7_slot = var_t10__blk1133_dn7;
        *var_t1__blk1132_slot = var_t1__blk1132;
        *var_t1__blk1132_dn0_slot = var_t1__blk1132_dn0;
        *var_t1__blk1132_dn10_slot = var_t1__blk1132_dn10;
        *var_t1__blk1132_dn11_slot = var_t1__blk1132_dn11;
        *var_t1__blk1132_dn12_slot = var_t1__blk1132_dn12;
        *var_t1__blk1132_dn17_slot = var_t1__blk1132_dn17;
        *var_t1__blk1132_dn2_slot = var_t1__blk1132_dn2;
        *var_t1__blk1132_dn6_slot = var_t1__blk1132_dn6;
        *var_t1__blk1132_dn7_slot = var_t1__blk1132_dn7;
        *var_t2__blk1135_slot = var_t2__blk1135;
        *var_t2__blk1135_dn0_slot = var_t2__blk1135_dn0;
        *var_t2__blk1135_dn10_slot = var_t2__blk1135_dn10;
        *var_t2__blk1135_dn11_slot = var_t2__blk1135_dn11;
        *var_t2__blk1135_dn12_slot = var_t2__blk1135_dn12;
        *var_t2__blk1135_dn17_slot = var_t2__blk1135_dn17;
        *var_t2__blk1135_dn2_slot = var_t2__blk1135_dn2;
        *var_t2__blk1135_dn6_slot = var_t2__blk1135_dn6;
        *var_t2__blk1135_dn7_slot = var_t2__blk1135_dn7;
        *var_t3__blk1134_slot = var_t3__blk1134;
        *var_t3__blk1134_dn0_slot = var_t3__blk1134_dn0;
        *var_t3__blk1134_dn10_slot = var_t3__blk1134_dn10;
        *var_t3__blk1134_dn11_slot = var_t3__blk1134_dn11;
        *var_t3__blk1134_dn12_slot = var_t3__blk1134_dn12;
        *var_t3__blk1134_dn17_slot = var_t3__blk1134_dn17;
        *var_t3__blk1134_dn2_slot = var_t3__blk1134_dn2;
        *var_t3__blk1134_dn6_slot = var_t3__blk1134_dn6;
        *var_t3__blk1134_dn7_slot = var_t3__blk1134_dn7;
    }

    pub(super) fn stamp_transient_block_121(
        p: &Parameters,
        var_c_fox: f64,
        var_flg_ign: f64,
        var_flg_noqi: f64,
        var_glpart1: f64,
        var_guard1139: f64,
        var_guard1140: f64,
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
        var_guard1141_slot: &mut f64,
        var_guard1142_slot: &mut f64,
        var_guard1143_slot: &mut f64,
        var_guard1144_slot: &mut f64,
        var_guard1151_slot: &mut f64,
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
        var_t0__blk1145_slot: &mut f64,
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
        let mut var_guard1141: f64 = *var_guard1141_slot;
        let mut var_guard1142: f64 = *var_guard1142_slot;
        let mut var_guard1143: f64 = *var_guard1143_slot;
        let mut var_guard1144: f64 = *var_guard1144_slot;
        let mut var_guard1151: f64 = *var_guard1151_slot;
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
        let mut var_t0__blk1145: f64 = *var_t0__blk1145_slot;

        let (assign34570_e49663, assign34570_e49663_d_n0, assign34570_e49663_d_n2, assign34570_e49663_d_n6, assign34570_e49663_d_n7, assign34570_e49663_d_n10, assign34570_e49663_d_n11, assign34570_e49663_d_n12, assign34570_e49663_d_n13, assign34570_e49663_d_n15, assign34570_e49663_d_n16, assign34570_e49663_d_n17, assign34570_e49663_d_n18,) = {
    if ((var_guard1139 != 0.0) && (var_guard1140 == 0.0)) {
        let assign34570_e49651: f64 = (var_qgod + var_qgos);
        let assign34570_e49653: f64 = (assign34570_e49651 + var_qgob);
        let assign34570_e49655: f64 = (assign34570_e49653 - var_qy);
        let assign34570_e49657: f64 = (assign34570_e49655 - var_qovs);
        let assign34570_e49659: f64 = (assign34570_e49657 - var_qovd);
        let assign34570_e49660: f64 = (var_mfactor * assign34570_e49659);
        let assign34570_e49661: f64 = (var_qge + assign34570_e49660);
        (assign34570_e49661, (var_qge_dn0 + (var_mfactor * (((((var_qgod_dn0 + var_qgos_dn0) + var_qgob_dn0) - var_qy_dn0) - var_qovs_dn0) - var_qovd_dn0))), (var_qge_dn2 + (var_mfactor * (((((var_qgod_dn2 + var_qgos_dn2) + var_qgob_dn2) - var_qy_dn2) - var_qovs_dn2) - var_qovd_dn2))), (var_qge_dn6 + (var_mfactor * (((((var_qgod_dn6 + var_qgos_dn6) + var_qgob_dn6) - var_qy_dn6) - var_qovs_dn6) - var_qovd_dn6))), (var_qge_dn7 + (var_mfactor * (((((var_qgod_dn7 + var_qgos_dn7) + var_qgob_dn7) - var_qy_dn7) - var_qovs_dn7) - var_qovd_dn7))), (var_qge_dn10 + (var_mfactor * (((((var_qgod_dn10 + var_qgos_dn10) + var_qgob_dn10) - var_qy_dn10) - var_qovs_dn10) - var_qovd_dn10))), (var_qge_dn11 + (var_mfactor * (((((var_qgod_dn11 + var_qgos_dn11) + var_qgob_dn11) - var_qy_dn11) - var_qovs_dn11) - var_qovd_dn11))), (var_qge_dn12 + (var_mfactor * (((((var_qgod_dn12 + var_qgos_dn12) + var_qgob_dn12) - var_qy_dn12) - var_qovs_dn12) - var_qovd_dn12))), var_qge_dn13, var_qge_dn15, var_qge_dn16, (var_qge_dn17 + (var_mfactor * (((((var_qgod_dn17 + var_qgos_dn17) + var_qgob_dn17) - var_qy_dn17) - var_qovs_dn17) - var_qovd_dn17))), var_qge_dn18,)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn6, var_qge_dn7, var_qge_dn10, var_qge_dn11, var_qge_dn12, var_qge_dn13, var_qge_dn15, var_qge_dn16, var_qge_dn17, var_qge_dn18,)
    }
};
        var_qge = assign34570_e49663;
        var_qge_dn0 = assign34570_e49663_d_n0;
        var_qge_dn2 = assign34570_e49663_d_n2;
        var_qge_dn6 = assign34570_e49663_d_n6;
        var_qge_dn7 = assign34570_e49663_d_n7;
        var_qge_dn10 = assign34570_e49663_d_n10;
        var_qge_dn11 = assign34570_e49663_d_n11;
        var_qge_dn12 = assign34570_e49663_d_n12;
        var_qge_dn13 = assign34570_e49663_d_n13;
        var_qge_dn15 = assign34570_e49663_d_n15;
        var_qge_dn16 = assign34570_e49663_d_n16;
        var_qge_dn17 = assign34570_e49663_d_n17;
        var_qge_dn18 = assign34570_e49663_d_n18;

        let (assign34580_e49679, assign34580_e49679_d_n0, assign34580_e49679_d_n2, assign34580_e49679_d_n6, assign34580_e49679_d_n7, assign34580_e49679_d_n10, assign34580_e49679_d_n11, assign34580_e49679_d_n12, assign34580_e49679_d_n13, assign34580_e49679_d_n15, assign34580_e49679_d_n16, assign34580_e49679_d_n17, assign34580_e49679_d_n18,) = {
    if ((var_guard1139 != 0.0) && (var_guard1140 == 0.0)) {
        let assign34580_e49671: f64 = (-var_qgod);
        let assign34580_e49673: f64 = (assign34580_e49671 + var_qy);
        let assign34580_e49675: f64 = (assign34580_e49673 + var_qbdld);
        let assign34580_e49676: f64 = (var_mfactor * assign34580_e49675);
        let assign34580_e49677: f64 = (var_qde + assign34580_e49676);
        (assign34580_e49677, (var_qde_dn0 + (var_mfactor * (((-var_qgod_dn0) + var_qy_dn0) + var_qbdld_dn0))), (var_qde_dn2 + (var_mfactor * (((-var_qgod_dn2) + var_qy_dn2) + var_qbdld_dn2))), (var_qde_dn6 + (var_mfactor * (((-var_qgod_dn6) + var_qy_dn6) + var_qbdld_dn6))), (var_qde_dn7 + (var_mfactor * (((-var_qgod_dn7) + var_qy_dn7) + var_qbdld_dn7))), (var_qde_dn10 + (var_mfactor * (((-var_qgod_dn10) + var_qy_dn10) + var_qbdld_dn10))), (var_qde_dn11 + (var_mfactor * (((-var_qgod_dn11) + var_qy_dn11) + var_qbdld_dn11))), (var_qde_dn12 + (var_mfactor * (((-var_qgod_dn12) + var_qy_dn12) + var_qbdld_dn12))), var_qde_dn13, var_qde_dn15, var_qde_dn16, (var_qde_dn17 + (var_mfactor * (((-var_qgod_dn17) + var_qy_dn17) + var_qbdld_dn17))), var_qde_dn18,)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn6, var_qde_dn7, var_qde_dn10, var_qde_dn11, var_qde_dn12, var_qde_dn13, var_qde_dn15, var_qde_dn16, var_qde_dn17, var_qde_dn18,)
    }
};
        var_qde = assign34580_e49679;
        var_qde_dn0 = assign34580_e49679_d_n0;
        var_qde_dn2 = assign34580_e49679_d_n2;
        var_qde_dn6 = assign34580_e49679_d_n6;
        var_qde_dn7 = assign34580_e49679_d_n7;
        var_qde_dn10 = assign34580_e49679_d_n10;
        var_qde_dn11 = assign34580_e49679_d_n11;
        var_qde_dn12 = assign34580_e49679_d_n12;
        var_qde_dn13 = assign34580_e49679_d_n13;
        var_qde_dn15 = assign34580_e49679_d_n15;
        var_qde_dn16 = assign34580_e49679_d_n16;
        var_qde_dn17 = assign34580_e49679_d_n17;
        var_qde_dn18 = assign34580_e49679_d_n18;

        let (assign34590_e49693, assign34590_e49693_d_n0, assign34590_e49693_d_n2, assign34590_e49693_d_n6, assign34590_e49693_d_n7, assign34590_e49693_d_n10, assign34590_e49693_d_n11, assign34590_e49693_d_n12, assign34590_e49693_d_n13, assign34590_e49693_d_n15, assign34590_e49693_d_n16, assign34590_e49693_d_n17, assign34590_e49693_d_n18,) = {
    if ((var_guard1139 != 0.0) && (var_guard1140 == 0.0)) {
        let assign34590_e49687: f64 = (-var_qgos);
        let assign34590_e49689: f64 = (assign34590_e49687 + var_qbsld);
        let assign34590_e49690: f64 = (var_mfactor * assign34590_e49689);
        let assign34590_e49691: f64 = (var_qse + assign34590_e49690);
        (assign34590_e49691, (var_qse_dn0 + (var_mfactor * ((-var_qgos_dn0) + var_qbsld_dn0))), (var_qse_dn2 + (var_mfactor * ((-var_qgos_dn2) + var_qbsld_dn2))), (var_qse_dn6 + (var_mfactor * ((-var_qgos_dn6) + var_qbsld_dn6))), (var_qse_dn7 + (var_mfactor * ((-var_qgos_dn7) + var_qbsld_dn7))), (var_qse_dn10 + (var_mfactor * ((-var_qgos_dn10) + var_qbsld_dn10))), (var_qse_dn11 + (var_mfactor * ((-var_qgos_dn11) + var_qbsld_dn11))), (var_qse_dn12 + (var_mfactor * ((-var_qgos_dn12) + var_qbsld_dn12))), var_qse_dn13, var_qse_dn15, var_qse_dn16, (var_qse_dn17 + (var_mfactor * ((-var_qgos_dn17) + var_qbsld_dn17))), var_qse_dn18,)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn6, var_qse_dn7, var_qse_dn10, var_qse_dn11, var_qse_dn12, var_qse_dn13, var_qse_dn15, var_qse_dn16, var_qse_dn17, var_qse_dn18,)
    }
};
        var_qse = assign34590_e49693;
        var_qse_dn0 = assign34590_e49693_d_n0;
        var_qse_dn2 = assign34590_e49693_d_n2;
        var_qse_dn6 = assign34590_e49693_d_n6;
        var_qse_dn7 = assign34590_e49693_d_n7;
        var_qse_dn10 = assign34590_e49693_d_n10;
        var_qse_dn11 = assign34590_e49693_d_n11;
        var_qse_dn12 = assign34590_e49693_d_n12;
        var_qse_dn13 = assign34590_e49693_d_n13;
        var_qse_dn15 = assign34590_e49693_d_n15;
        var_qse_dn16 = assign34590_e49693_d_n16;
        var_qse_dn17 = assign34590_e49693_d_n17;
        var_qse_dn18 = assign34590_e49693_d_n18;

        let assign34620_e49698: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1141 = assign34620_e49698;

        let (assign34630_e49704, assign34630_e49704_d_n0, assign34630_e49704_d_n2, assign34630_e49704_d_n6, assign34630_e49704_d_n7, assign34630_e49704_d_n10, assign34630_e49704_d_n11, assign34630_e49704_d_n12, assign34630_e49704_d_n17,) = {
    if (var_guard1141 != 0.0) {
        let assign34630_e49702: f64 = (var_mfactor * var_ibs);
        (assign34630_e49702, (var_mfactor * var_ibs_dn0), (var_mfactor * var_ibs_dn2), (var_mfactor * var_ibs_dn6), (var_mfactor * var_ibs_dn7), (var_mfactor * var_ibs_dn10), (var_mfactor * var_ibs_dn11), (var_mfactor * var_ibs_dn12), (var_mfactor * var_ibs_dn17),)
    } else {
        (var_ibsb, var_ibsb_dn0, var_ibsb_dn2, var_ibsb_dn6, var_ibsb_dn7, var_ibsb_dn10, var_ibsb_dn11, var_ibsb_dn12, var_ibsb_dn17,)
    }
};
        var_ibsb = assign34630_e49704;
        var_ibsb_dn0 = assign34630_e49704_d_n0;
        var_ibsb_dn2 = assign34630_e49704_d_n2;
        var_ibsb_dn6 = assign34630_e49704_d_n6;
        var_ibsb_dn7 = assign34630_e49704_d_n7;
        var_ibsb_dn10 = assign34630_e49704_d_n10;
        var_ibsb_dn11 = assign34630_e49704_d_n11;
        var_ibsb_dn12 = assign34630_e49704_d_n12;
        var_ibsb_dn17 = assign34630_e49704_d_n17;

        let (assign34640_e49710, assign34640_e49710_d_n0, assign34640_e49710_d_n2, assign34640_e49710_d_n6, assign34640_e49710_d_n7, assign34640_e49710_d_n10, assign34640_e49710_d_n11, assign34640_e49710_d_n12, assign34640_e49710_d_n17,) = {
    if (var_guard1141 != 0.0) {
        let assign34640_e49708: f64 = (var_mfactor * var_ibd);
        (assign34640_e49708, (var_mfactor * var_ibd_dn0), (var_mfactor * var_ibd_dn2), (var_mfactor * var_ibd_dn6), (var_mfactor * var_ibd_dn7), (var_mfactor * var_ibd_dn10), (var_mfactor * var_ibd_dn11), (var_mfactor * var_ibd_dn12), (var_mfactor * var_ibd_dn17),)
    } else {
        (var_ibdb, var_ibdb_dn0, var_ibdb_dn2, var_ibdb_dn6, var_ibdb_dn7, var_ibdb_dn10, var_ibdb_dn11, var_ibdb_dn12, var_ibdb_dn17,)
    }
};
        var_ibdb = assign34640_e49710;
        var_ibdb_dn0 = assign34640_e49710_d_n0;
        var_ibdb_dn2 = assign34640_e49710_d_n2;
        var_ibdb_dn6 = assign34640_e49710_d_n6;
        var_ibdb_dn7 = assign34640_e49710_d_n7;
        var_ibdb_dn10 = assign34640_e49710_d_n10;
        var_ibdb_dn11 = assign34640_e49710_d_n11;
        var_ibdb_dn12 = assign34640_e49710_d_n12;
        var_ibdb_dn17 = assign34640_e49710_d_n17;

        let (assign34650_e49716, assign34650_e49716_d_n0, assign34650_e49716_d_n2, assign34650_e49716_d_n6, assign34650_e49716_d_n7, assign34650_e49716_d_n10, assign34650_e49716_d_n11, assign34650_e49716_d_n12, assign34650_e49716_d_n17,) = {
    if (var_guard1141 != 0.0) {
        let assign34650_e49714: f64 = (var_mfactor * var_qbd);
        (assign34650_e49714, (var_mfactor * var_qbd_dn0), (var_mfactor * var_qbd_dn2), (var_mfactor * var_qbd_dn6), (var_mfactor * var_qbd_dn7), (var_mfactor * var_qbd_dn10), (var_mfactor * var_qbd_dn11), (var_mfactor * var_qbd_dn12), (var_mfactor * var_qbd_dn17),)
    } else {
        (var_qbd_s0, var_qbd_s0_dn0, var_qbd_s0_dn2, var_qbd_s0_dn6, var_qbd_s0_dn7, var_qbd_s0_dn10, var_qbd_s0_dn11, var_qbd_s0_dn12, var_qbd_s0_dn17,)
    }
};
        var_qbd_s0 = assign34650_e49716;
        var_qbd_s0_dn0 = assign34650_e49716_d_n0;
        var_qbd_s0_dn2 = assign34650_e49716_d_n2;
        var_qbd_s0_dn6 = assign34650_e49716_d_n6;
        var_qbd_s0_dn7 = assign34650_e49716_d_n7;
        var_qbd_s0_dn10 = assign34650_e49716_d_n10;
        var_qbd_s0_dn11 = assign34650_e49716_d_n11;
        var_qbd_s0_dn12 = assign34650_e49716_d_n12;
        var_qbd_s0_dn17 = assign34650_e49716_d_n17;

        let (assign34660_e49722, assign34660_e49722_d_n0, assign34660_e49722_d_n2, assign34660_e49722_d_n6, assign34660_e49722_d_n7, assign34660_e49722_d_n10, assign34660_e49722_d_n11, assign34660_e49722_d_n12, assign34660_e49722_d_n17,) = {
    if (var_guard1141 != 0.0) {
        let assign34660_e49720: f64 = (var_mfactor * var_qbs);
        (assign34660_e49720, (var_mfactor * var_qbs_dn0), (var_mfactor * var_qbs_dn2), (var_mfactor * var_qbs_dn6), (var_mfactor * var_qbs_dn7), (var_mfactor * var_qbs_dn10), (var_mfactor * var_qbs_dn11), (var_mfactor * var_qbs_dn12), (var_mfactor * var_qbs_dn17),)
    } else {
        (var_qbs_s0, var_qbs_s0_dn0, var_qbs_s0_dn2, var_qbs_s0_dn6, var_qbs_s0_dn7, var_qbs_s0_dn10, var_qbs_s0_dn11, var_qbs_s0_dn12, var_qbs_s0_dn17,)
    }
};
        var_qbs_s0 = assign34660_e49722;
        var_qbs_s0_dn0 = assign34660_e49722_d_n0;
        var_qbs_s0_dn2 = assign34660_e49722_d_n2;
        var_qbs_s0_dn6 = assign34660_e49722_d_n6;
        var_qbs_s0_dn7 = assign34660_e49722_d_n7;
        var_qbs_s0_dn10 = assign34660_e49722_d_n10;
        var_qbs_s0_dn11 = assign34660_e49722_d_n11;
        var_qbs_s0_dn12 = assign34660_e49722_d_n12;
        var_qbs_s0_dn17 = assign34660_e49722_d_n17;

        let (assign34670_e49727, assign34670_e49727_d_n0, assign34670_e49727_d_n2, assign34670_e49727_d_n6, assign34670_e49727_d_n7, assign34670_e49727_d_n10, assign34670_e49727_d_n11, assign34670_e49727_d_n12, assign34670_e49727_d_n17,) = {
    if (var_guard1141 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibsb, var_ibsb_dn0, var_ibsb_dn2, var_ibsb_dn6, var_ibsb_dn7, var_ibsb_dn10, var_ibsb_dn11, var_ibsb_dn12, var_ibsb_dn17,)
    }
};
        var_ibsb = assign34670_e49727;
        var_ibsb_dn0 = assign34670_e49727_d_n0;
        var_ibsb_dn2 = assign34670_e49727_d_n2;
        var_ibsb_dn6 = assign34670_e49727_d_n6;
        var_ibsb_dn7 = assign34670_e49727_d_n7;
        var_ibsb_dn10 = assign34670_e49727_d_n10;
        var_ibsb_dn11 = assign34670_e49727_d_n11;
        var_ibsb_dn12 = assign34670_e49727_d_n12;
        var_ibsb_dn17 = assign34670_e49727_d_n17;

        let (assign34680_e49732, assign34680_e49732_d_n0, assign34680_e49732_d_n2, assign34680_e49732_d_n6, assign34680_e49732_d_n7, assign34680_e49732_d_n10, assign34680_e49732_d_n11, assign34680_e49732_d_n12, assign34680_e49732_d_n17,) = {
    if (var_guard1141 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibdb, var_ibdb_dn0, var_ibdb_dn2, var_ibdb_dn6, var_ibdb_dn7, var_ibdb_dn10, var_ibdb_dn11, var_ibdb_dn12, var_ibdb_dn17,)
    }
};
        var_ibdb = assign34680_e49732;
        var_ibdb_dn0 = assign34680_e49732_d_n0;
        var_ibdb_dn2 = assign34680_e49732_d_n2;
        var_ibdb_dn6 = assign34680_e49732_d_n6;
        var_ibdb_dn7 = assign34680_e49732_d_n7;
        var_ibdb_dn10 = assign34680_e49732_d_n10;
        var_ibdb_dn11 = assign34680_e49732_d_n11;
        var_ibdb_dn12 = assign34680_e49732_d_n12;
        var_ibdb_dn17 = assign34680_e49732_d_n17;

        let (assign34690_e49737, assign34690_e49737_d_n0, assign34690_e49737_d_n2, assign34690_e49737_d_n6, assign34690_e49737_d_n7, assign34690_e49737_d_n10, assign34690_e49737_d_n11, assign34690_e49737_d_n12, assign34690_e49737_d_n17,) = {
    if (var_guard1141 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbd_s0, var_qbd_s0_dn0, var_qbd_s0_dn2, var_qbd_s0_dn6, var_qbd_s0_dn7, var_qbd_s0_dn10, var_qbd_s0_dn11, var_qbd_s0_dn12, var_qbd_s0_dn17,)
    }
};
        var_qbd_s0 = assign34690_e49737;
        var_qbd_s0_dn0 = assign34690_e49737_d_n0;
        var_qbd_s0_dn2 = assign34690_e49737_d_n2;
        var_qbd_s0_dn6 = assign34690_e49737_d_n6;
        var_qbd_s0_dn7 = assign34690_e49737_d_n7;
        var_qbd_s0_dn10 = assign34690_e49737_d_n10;
        var_qbd_s0_dn11 = assign34690_e49737_d_n11;
        var_qbd_s0_dn12 = assign34690_e49737_d_n12;
        var_qbd_s0_dn17 = assign34690_e49737_d_n17;

        let (assign34700_e49742, assign34700_e49742_d_n0, assign34700_e49742_d_n2, assign34700_e49742_d_n6, assign34700_e49742_d_n7, assign34700_e49742_d_n10, assign34700_e49742_d_n11, assign34700_e49742_d_n12, assign34700_e49742_d_n17,) = {
    if (var_guard1141 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbs_s0, var_qbs_s0_dn0, var_qbs_s0_dn2, var_qbs_s0_dn6, var_qbs_s0_dn7, var_qbs_s0_dn10, var_qbs_s0_dn11, var_qbs_s0_dn12, var_qbs_s0_dn17,)
    }
};
        var_qbs_s0 = assign34700_e49742;
        var_qbs_s0_dn0 = assign34700_e49742_d_n0;
        var_qbs_s0_dn2 = assign34700_e49742_d_n2;
        var_qbs_s0_dn6 = assign34700_e49742_d_n6;
        var_qbs_s0_dn7 = assign34700_e49742_d_n7;
        var_qbs_s0_dn10 = assign34700_e49742_d_n10;
        var_qbs_s0_dn11 = assign34700_e49742_d_n11;
        var_qbs_s0_dn12 = assign34700_e49742_d_n12;
        var_qbs_s0_dn17 = assign34700_e49742_d_n17;

        let assign34710_e49745: f64 = if p.p25 != 1.0 { 1.0 } else { 0.0 };
        var_guard1142 = assign34710_e49745;

        let (assign34720_e49749, assign34720_e49749_d_n0, assign34720_e49749_d_n2, assign34720_e49749_d_n6, assign34720_e49749_d_n7, assign34720_e49749_d_n10, assign34720_e49749_d_n11, assign34720_e49749_d_n12, assign34720_e49749_d_n17,) = {
    if (var_guard1142 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn6, var_isube_dn7, var_isube_dn10, var_isube_dn11, var_isube_dn12, var_isube_dn17,)
    }
};
        var_isube = assign34720_e49749;
        var_isube_dn0 = assign34720_e49749_d_n0;
        var_isube_dn2 = assign34720_e49749_d_n2;
        var_isube_dn6 = assign34720_e49749_d_n6;
        var_isube_dn7 = assign34720_e49749_d_n7;
        var_isube_dn10 = assign34720_e49749_d_n10;
        var_isube_dn11 = assign34720_e49749_d_n11;
        var_isube_dn12 = assign34720_e49749_d_n12;
        var_isube_dn17 = assign34720_e49749_d_n17;

        let (assign34730_e49756, assign34730_e49756_d_n0, assign34730_e49756_d_n2, assign34730_e49756_d_n6, assign34730_e49756_d_n7, assign34730_e49756_d_n10, assign34730_e49756_d_n11, assign34730_e49756_d_n12, assign34730_e49756_d_n17,) = {
    if (var_guard1142 == 0.0) {
        let assign34730_e49754: f64 = (var_mfactor * var_isub);
        (assign34730_e49754, (var_mfactor * var_isub_dn0), (var_mfactor * var_isub_dn2), (var_mfactor * var_isub_dn6), (var_mfactor * var_isub_dn7), (var_mfactor * var_isub_dn10), (var_mfactor * var_isub_dn11), (var_mfactor * var_isub_dn12), (var_mfactor * var_isub_dn17),)
    } else {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn6, var_isube_dn7, var_isube_dn10, var_isube_dn11, var_isube_dn12, var_isube_dn17,)
    }
};
        var_isube = assign34730_e49756;
        var_isube_dn0 = assign34730_e49756_d_n0;
        var_isube_dn2 = assign34730_e49756_d_n2;
        var_isube_dn6 = assign34730_e49756_d_n6;
        var_isube_dn7 = assign34730_e49756_d_n7;
        var_isube_dn10 = assign34730_e49756_d_n10;
        var_isube_dn11 = assign34730_e49756_d_n11;
        var_isube_dn12 = assign34730_e49756_d_n12;
        var_isube_dn17 = assign34730_e49756_d_n17;

        let assign34740_e49759: f64 = (-var_igb);
        let assign34740_e49760: f64 = (var_mfactor * assign34740_e49759);
        var_igbe = assign34740_e49760;
        var_igbe_dn0 = (var_mfactor * (-var_igb_dn0));
        var_igbe_dn2 = (var_mfactor * (-var_igb_dn2));
        var_igbe_dn6 = (var_mfactor * (-var_igb_dn6));
        var_igbe_dn7 = (var_mfactor * (-var_igb_dn7));
        var_igbe_dn10 = (var_mfactor * (-var_igb_dn10));
        var_igbe_dn11 = (var_mfactor * (-var_igb_dn11));
        var_igbe_dn12 = (var_mfactor * (-var_igb_dn12));
        var_igbe_dn17 = (var_mfactor * (-var_igb_dn17));

        let assign34750_e49763: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard1143 = assign34750_e49763;

        let (assign34760_e49773, assign34760_e49773_d_n0, assign34760_e49773_d_n2, assign34760_e49773_d_n6, assign34760_e49773_d_n7, assign34760_e49773_d_n10, assign34760_e49773_d_n11, assign34760_e49773_d_n12, assign34760_e49773_d_n17,) = {
    if (var_guard1143 != 0.0) {
        let assign34760_e49768: f64 = (var_glpart1 * var_igate);
        let assign34760_e49770: f64 = (assign34760_e49768 - var_igd);
        let assign34760_e49771: f64 = (var_mfactor * assign34760_e49770);
        (assign34760_e49771, (var_mfactor * ((var_glpart1 * var_igate_dn0) - var_igd_dn0)), (var_mfactor * ((var_glpart1 * var_igate_dn2) - var_igd_dn2)), (var_mfactor * ((var_glpart1 * var_igate_dn6) - var_igd_dn6)), (var_mfactor * ((var_glpart1 * var_igate_dn7) - var_igd_dn7)), (var_mfactor * ((var_glpart1 * var_igate_dn10) - var_igd_dn10)), (var_mfactor * ((var_glpart1 * var_igate_dn11) - var_igd_dn11)), (var_mfactor * ((var_glpart1 * var_igate_dn12) - var_igd_dn12)), (var_mfactor * ((var_glpart1 * var_igate_dn17) - var_igd_dn17)),)
    } else {
        (var_igde, var_igde_dn0, var_igde_dn2, var_igde_dn6, var_igde_dn7, var_igde_dn10, var_igde_dn11, var_igde_dn12, var_igde_dn17,)
    }
};
        var_igde = assign34760_e49773;
        var_igde_dn0 = assign34760_e49773_d_n0;
        var_igde_dn2 = assign34760_e49773_d_n2;
        var_igde_dn6 = assign34760_e49773_d_n6;
        var_igde_dn7 = assign34760_e49773_d_n7;
        var_igde_dn10 = assign34760_e49773_d_n10;
        var_igde_dn11 = assign34760_e49773_d_n11;
        var_igde_dn12 = assign34760_e49773_d_n12;
        var_igde_dn17 = assign34760_e49773_d_n17;

        let (assign34770_e49786, assign34770_e49786_d_n0, assign34770_e49786_d_n2, assign34770_e49786_d_n6, assign34770_e49786_d_n7, assign34770_e49786_d_n10, assign34770_e49786_d_n11, assign34770_e49786_d_n12, assign34770_e49786_d_n17,) = {
    if (var_guard1143 == 0.0) {
        let assign34770_e49779: f64 = (1.0 - var_glpart1);
        let assign34770_e49781: f64 = (assign34770_e49779 * var_igate);
        let assign34770_e49783: f64 = (assign34770_e49781 - var_igs);
        let assign34770_e49784: f64 = (var_mfactor * assign34770_e49783);
        (assign34770_e49784, (var_mfactor * ((assign34770_e49779 * var_igate_dn0) - var_igs_dn0)), (var_mfactor * ((assign34770_e49779 * var_igate_dn2) - var_igs_dn2)), (var_mfactor * ((assign34770_e49779 * var_igate_dn6) - var_igs_dn6)), (var_mfactor * ((assign34770_e49779 * var_igate_dn7) - var_igs_dn7)), (var_mfactor * ((assign34770_e49779 * var_igate_dn10) - var_igs_dn10)), (var_mfactor * ((assign34770_e49779 * var_igate_dn11) - var_igs_dn11)), (var_mfactor * ((assign34770_e49779 * var_igate_dn12) - var_igs_dn12)), (var_mfactor * ((assign34770_e49779 * var_igate_dn17) - var_igs_dn17)),)
    } else {
        (var_igde, var_igde_dn0, var_igde_dn2, var_igde_dn6, var_igde_dn7, var_igde_dn10, var_igde_dn11, var_igde_dn12, var_igde_dn17,)
    }
};
        var_igde = assign34770_e49786;
        var_igde_dn0 = assign34770_e49786_d_n0;
        var_igde_dn2 = assign34770_e49786_d_n2;
        var_igde_dn6 = assign34770_e49786_d_n6;
        var_igde_dn7 = assign34770_e49786_d_n7;
        var_igde_dn10 = assign34770_e49786_d_n10;
        var_igde_dn11 = assign34770_e49786_d_n11;
        var_igde_dn12 = assign34770_e49786_d_n12;
        var_igde_dn17 = assign34770_e49786_d_n17;

        let assign34780_e49789: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard1144 = assign34780_e49789;

        let (assign34790_e49801, assign34790_e49801_d_n0, assign34790_e49801_d_n2, assign34790_e49801_d_n6, assign34790_e49801_d_n7, assign34790_e49801_d_n10, assign34790_e49801_d_n11, assign34790_e49801_d_n12, assign34790_e49801_d_n17,) = {
    if (var_guard1144 != 0.0) {
        let assign34790_e49794: f64 = (1.0 - var_glpart1);
        let assign34790_e49796: f64 = (assign34790_e49794 * var_igate);
        let assign34790_e49798: f64 = (assign34790_e49796 - var_igs);
        let assign34790_e49799: f64 = (var_mfactor * assign34790_e49798);
        (assign34790_e49799, (var_mfactor * ((assign34790_e49794 * var_igate_dn0) - var_igs_dn0)), (var_mfactor * ((assign34790_e49794 * var_igate_dn2) - var_igs_dn2)), (var_mfactor * ((assign34790_e49794 * var_igate_dn6) - var_igs_dn6)), (var_mfactor * ((assign34790_e49794 * var_igate_dn7) - var_igs_dn7)), (var_mfactor * ((assign34790_e49794 * var_igate_dn10) - var_igs_dn10)), (var_mfactor * ((assign34790_e49794 * var_igate_dn11) - var_igs_dn11)), (var_mfactor * ((assign34790_e49794 * var_igate_dn12) - var_igs_dn12)), (var_mfactor * ((assign34790_e49794 * var_igate_dn17) - var_igs_dn17)),)
    } else {
        (var_igse, var_igse_dn0, var_igse_dn2, var_igse_dn6, var_igse_dn7, var_igse_dn10, var_igse_dn11, var_igse_dn12, var_igse_dn17,)
    }
};
        var_igse = assign34790_e49801;
        var_igse_dn0 = assign34790_e49801_d_n0;
        var_igse_dn2 = assign34790_e49801_d_n2;
        var_igse_dn6 = assign34790_e49801_d_n6;
        var_igse_dn7 = assign34790_e49801_d_n7;
        var_igse_dn10 = assign34790_e49801_d_n10;
        var_igse_dn11 = assign34790_e49801_d_n11;
        var_igse_dn12 = assign34790_e49801_d_n12;
        var_igse_dn17 = assign34790_e49801_d_n17;

        let (assign34800_e49812, assign34800_e49812_d_n0, assign34800_e49812_d_n2, assign34800_e49812_d_n6, assign34800_e49812_d_n7, assign34800_e49812_d_n10, assign34800_e49812_d_n11, assign34800_e49812_d_n12, assign34800_e49812_d_n17,) = {
    if (var_guard1144 == 0.0) {
        let assign34800_e49807: f64 = (var_glpart1 * var_igate);
        let assign34800_e49809: f64 = (assign34800_e49807 - var_igd);
        let assign34800_e49810: f64 = (var_mfactor * assign34800_e49809);
        (assign34800_e49810, (var_mfactor * ((var_glpart1 * var_igate_dn0) - var_igd_dn0)), (var_mfactor * ((var_glpart1 * var_igate_dn2) - var_igd_dn2)), (var_mfactor * ((var_glpart1 * var_igate_dn6) - var_igd_dn6)), (var_mfactor * ((var_glpart1 * var_igate_dn7) - var_igd_dn7)), (var_mfactor * ((var_glpart1 * var_igate_dn10) - var_igd_dn10)), (var_mfactor * ((var_glpart1 * var_igate_dn11) - var_igd_dn11)), (var_mfactor * ((var_glpart1 * var_igate_dn12) - var_igd_dn12)), (var_mfactor * ((var_glpart1 * var_igate_dn17) - var_igd_dn17)),)
    } else {
        (var_igse, var_igse_dn0, var_igse_dn2, var_igse_dn6, var_igse_dn7, var_igse_dn10, var_igse_dn11, var_igse_dn12, var_igse_dn17,)
    }
};
        var_igse = assign34800_e49812;
        var_igse_dn0 = assign34800_e49812_d_n0;
        var_igse_dn2 = assign34800_e49812_d_n2;
        var_igse_dn6 = assign34800_e49812_d_n6;
        var_igse_dn7 = assign34800_e49812_d_n7;
        var_igse_dn10 = assign34800_e49812_d_n10;
        var_igse_dn11 = assign34800_e49812_d_n11;
        var_igse_dn12 = assign34800_e49812_d_n12;
        var_igse_dn17 = assign34800_e49812_d_n17;

        let (assign34810_e49822, assign34810_e49822_d_n0, assign34810_e49822_d_n2, assign34810_e49822_d_n6, assign34810_e49822_d_n7, assign34810_e49822_d_n10, assign34810_e49822_d_n11, assign34810_e49822_d_n12, assign34810_e49822_d_n17,) = {
    if (var_mode == 1.0) {
        let assign34810_e49818: f64 = (var_mfactor * var_igidl);
        (assign34810_e49818, (var_mfactor * var_igidl_dn0), (var_mfactor * var_igidl_dn2), (var_mfactor * var_igidl_dn6), (var_mfactor * var_igidl_dn7), (var_mfactor * var_igidl_dn10), (var_mfactor * var_igidl_dn11), (var_mfactor * var_igidl_dn12), (var_mfactor * var_igidl_dn17),)
    } else {
        let assign34810_e49821: f64 = (var_mfactor * var_igisl);
        (assign34810_e49821, (var_mfactor * var_igisl_dn0), (var_mfactor * var_igisl_dn2), (var_mfactor * var_igisl_dn6), (var_mfactor * var_igisl_dn7), (var_mfactor * var_igisl_dn10), (var_mfactor * var_igisl_dn11), (var_mfactor * var_igisl_dn12), (var_mfactor * var_igisl_dn17),)
    }
};
        var_igidle = assign34810_e49822;
        var_igidle_dn0 = assign34810_e49822_d_n0;
        var_igidle_dn2 = assign34810_e49822_d_n2;
        var_igidle_dn6 = assign34810_e49822_d_n6;
        var_igidle_dn7 = assign34810_e49822_d_n7;
        var_igidle_dn10 = assign34810_e49822_d_n10;
        var_igidle_dn11 = assign34810_e49822_d_n11;
        var_igidle_dn12 = assign34810_e49822_d_n12;
        var_igidle_dn17 = assign34810_e49822_d_n17;

        let (assign34820_e49832, assign34820_e49832_d_n0, assign34820_e49832_d_n2, assign34820_e49832_d_n6, assign34820_e49832_d_n7, assign34820_e49832_d_n10, assign34820_e49832_d_n11, assign34820_e49832_d_n12, assign34820_e49832_d_n17,) = {
    if (var_mode == 1.0) {
        let assign34820_e49828: f64 = (var_mfactor * var_igisl);
        (assign34820_e49828, (var_mfactor * var_igisl_dn0), (var_mfactor * var_igisl_dn2), (var_mfactor * var_igisl_dn6), (var_mfactor * var_igisl_dn7), (var_mfactor * var_igisl_dn10), (var_mfactor * var_igisl_dn11), (var_mfactor * var_igisl_dn12), (var_mfactor * var_igisl_dn17),)
    } else {
        let assign34820_e49831: f64 = (var_mfactor * var_igidl);
        (assign34820_e49831, (var_mfactor * var_igidl_dn0), (var_mfactor * var_igidl_dn2), (var_mfactor * var_igidl_dn6), (var_mfactor * var_igidl_dn7), (var_mfactor * var_igidl_dn10), (var_mfactor * var_igidl_dn11), (var_mfactor * var_igidl_dn12), (var_mfactor * var_igidl_dn17),)
    }
};
        var_igisle = assign34820_e49832;
        var_igisle_dn0 = assign34820_e49832_d_n0;
        var_igisle_dn2 = assign34820_e49832_d_n2;
        var_igisle_dn6 = assign34820_e49832_d_n6;
        var_igisle_dn7 = assign34820_e49832_d_n7;
        var_igisle_dn10 = assign34820_e49832_d_n10;
        var_igisle_dn11 = assign34820_e49832_d_n11;
        var_igisle_dn12 = assign34820_e49832_d_n12;
        var_igisle_dn17 = assign34820_e49832_d_n17;

        let assign34840_e49838: f64 = (var_mfactor * var_nthrml);
        var_noithrml = assign34840_e49838;
        var_noithrml_dn0 = (var_mfactor * var_nthrml_dn0);
        var_noithrml_dn2 = (var_mfactor * var_nthrml_dn2);
        var_noithrml_dn6 = (var_mfactor * var_nthrml_dn6);
        var_noithrml_dn7 = (var_mfactor * var_nthrml_dn7);
        var_noithrml_dn10 = (var_mfactor * var_nthrml_dn10);
        var_noithrml_dn11 = (var_mfactor * var_nthrml_dn11);
        var_noithrml_dn12 = (var_mfactor * var_nthrml_dn12);
        var_noithrml_dn17 = (var_mfactor * var_nthrml_dn17);

        let assign34850_e49841: f64 = var_qge_dn6;
        var_cgdbd = assign34850_e49841;
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

        let assign34860_e49844: f64 = (p.p50 * var_cgdbd);
        var_cgdbd = assign34860_e49844;
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

        let assign34870_e49847: f64 = var_qge_dn7;
        var_cgsbd = assign34870_e49847;
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

        let assign34880_e49850: f64 = (p.p50 * var_cgsbd);
        var_cgsbd = assign34880_e49850;
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

        let (assign34890_e49856, assign34890_e49856_d_n0, assign34890_e49856_d_n2, assign34890_e49856_d_n6, assign34890_e49856_d_n7, assign34890_e49856_d_n10, assign34890_e49856_d_n11, assign34890_e49856_d_n12, assign34890_e49856_d_n13, assign34890_e49856_d_n15, assign34890_e49856_d_n16, assign34890_e49856_d_n17, assign34890_e49856_d_n18,) = {
    if (var_mode > 0.0) {
        (var_cgsbd, var_cgsbd_dn0, var_cgsbd_dn2, var_cgsbd_dn6, var_cgsbd_dn7, var_cgsbd_dn10, var_cgsbd_dn11, var_cgsbd_dn12, var_cgsbd_dn13, var_cgsbd_dn15, var_cgsbd_dn16, var_cgsbd_dn17, var_cgsbd_dn18,)
    } else {
        (var_cgdbd, var_cgdbd_dn0, var_cgdbd_dn2, var_cgdbd_dn6, var_cgdbd_dn7, var_cgdbd_dn10, var_cgdbd_dn11, var_cgdbd_dn12, var_cgdbd_dn13, var_cgdbd_dn15, var_cgdbd_dn16, var_cgdbd_dn17, var_cgdbd_dn18,)
    }
};
        var_cgsb = assign34890_e49856;
        var_cgsb_dn0 = assign34890_e49856_d_n0;
        var_cgsb_dn2 = assign34890_e49856_d_n2;
        var_cgsb_dn6 = assign34890_e49856_d_n6;
        var_cgsb_dn7 = assign34890_e49856_d_n7;
        var_cgsb_dn10 = assign34890_e49856_d_n10;
        var_cgsb_dn11 = assign34890_e49856_d_n11;
        var_cgsb_dn12 = assign34890_e49856_d_n12;
        var_cgsb_dn13 = assign34890_e49856_d_n13;
        var_cgsb_dn15 = assign34890_e49856_d_n15;
        var_cgsb_dn16 = assign34890_e49856_d_n16;
        var_cgsb_dn17 = assign34890_e49856_d_n17;
        var_cgsb_dn18 = assign34890_e49856_d_n18;

        let assign34900_e49870: f64 = if ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (var_flg_ign == 1.0)) && (var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        var_guard1151 = assign34900_e49870;

        let (assign34910_e49880,) = {
    if (var_guard1151 != 0.0) {
        let assign34910_e49874: f64 = (1e-6 * var_c_fox);
        let assign34910_e49876: f64 = (assign34910_e49874 * var_weffcv_nf);
        let assign34910_e49878: f64 = (assign34910_e49876 * var_leff_cv);
        (assign34910_e49878,)
    } else {
        (var_t0__blk1145,)
    }
};
        var_t0__blk1145 = assign34910_e49880;

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
        *var_guard1141_slot = var_guard1141;
        *var_guard1142_slot = var_guard1142;
        *var_guard1143_slot = var_guard1143;
        *var_guard1144_slot = var_guard1144;
        *var_guard1151_slot = var_guard1151;
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
        *var_t0__blk1145_slot = var_t0__blk1145;
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
        var_guard1151: f64,
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
        var_t0__blk1145: f64,
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
        var_guard1152_slot: &mut f64,
        var_guard1153_slot: &mut f64,
        var_guard1173_slot: &mut f64,
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
        var_t1__blk1146_slot: &mut f64,
        var_t1__blk1146_dn0_slot: &mut f64,
        var_t1__blk1146_dn10_slot: &mut f64,
        var_t1__blk1146_dn11_slot: &mut f64,
        var_t1__blk1146_dn12_slot: &mut f64,
        var_t1__blk1146_dn13_slot: &mut f64,
        var_t1__blk1146_dn15_slot: &mut f64,
        var_t1__blk1146_dn16_slot: &mut f64,
        var_t1__blk1146_dn17_slot: &mut f64,
        var_t1__blk1146_dn18_slot: &mut f64,
        var_t1__blk1146_dn2_slot: &mut f64,
        var_t1__blk1146_dn6_slot: &mut f64,
        var_t1__blk1146_dn7_slot: &mut f64,
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
        let mut var_guard1152: f64 = *var_guard1152_slot;
        let mut var_guard1153: f64 = *var_guard1153_slot;
        let mut var_guard1173: f64 = *var_guard1173_slot;
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
        let mut var_t1__blk1146: f64 = *var_t1__blk1146_slot;
        let mut var_t1__blk1146_dn0: f64 = *var_t1__blk1146_dn0_slot;
        let mut var_t1__blk1146_dn10: f64 = *var_t1__blk1146_dn10_slot;
        let mut var_t1__blk1146_dn11: f64 = *var_t1__blk1146_dn11_slot;
        let mut var_t1__blk1146_dn12: f64 = *var_t1__blk1146_dn12_slot;
        let mut var_t1__blk1146_dn13: f64 = *var_t1__blk1146_dn13_slot;
        let mut var_t1__blk1146_dn15: f64 = *var_t1__blk1146_dn15_slot;
        let mut var_t1__blk1146_dn16: f64 = *var_t1__blk1146_dn16_slot;
        let mut var_t1__blk1146_dn17: f64 = *var_t1__blk1146_dn17_slot;
        let mut var_t1__blk1146_dn18: f64 = *var_t1__blk1146_dn18_slot;
        let mut var_t1__blk1146_dn2: f64 = *var_t1__blk1146_dn2_slot;
        let mut var_t1__blk1146_dn6: f64 = *var_t1__blk1146_dn6_slot;
        let mut var_t1__blk1146_dn7: f64 = *var_t1__blk1146_dn7_slot;
        let mut var_tratio: f64 = *var_tratio_slot;
        let mut var_tratio_dn10: f64 = *var_tratio_dn10_slot;
        let mut var_vrdr: f64 = *var_vrdr_slot;
        let mut var_vrdr_dn0: f64 = *var_vrdr_dn0_slot;
        let mut var_vrdr_dn2: f64 = *var_vrdr_dn2_slot;
        let mut var_vrdr_dn6: f64 = *var_vrdr_dn6_slot;
        let mut var_vrdr_dn7: f64 = *var_vrdr_dn7_slot;
        let mut var_weff_nf_1: f64 = *var_weff_nf_1_slot;
        let mut var_xov: f64 = *var_xov_slot;

        let (assign34920_e49886, assign34920_e49886_d_n0, assign34920_e49886_d_n2, assign34920_e49886_d_n6, assign34920_e49886_d_n7, assign34920_e49886_d_n10, assign34920_e49886_d_n11, assign34920_e49886_d_n12, assign34920_e49886_d_n13, assign34920_e49886_d_n15, assign34920_e49886_d_n16, assign34920_e49886_d_n17, assign34920_e49886_d_n18,) = {
    if (var_guard1151 != 0.0) {
        let assign34920_e49884: f64 = (var_cgsb / var_mfactor);
        (assign34920_e49884, (var_cgsb_dn0 / var_mfactor), (var_cgsb_dn2 / var_mfactor), (var_cgsb_dn6 / var_mfactor), (var_cgsb_dn7 / var_mfactor), (var_cgsb_dn10 / var_mfactor), (var_cgsb_dn11 / var_mfactor), (var_cgsb_dn12 / var_mfactor), (var_cgsb_dn13 / var_mfactor), (var_cgsb_dn15 / var_mfactor), (var_cgsb_dn16 / var_mfactor), (var_cgsb_dn17 / var_mfactor), (var_cgsb_dn18 / var_mfactor),)
    } else {
        (var_t1__blk1146, var_t1__blk1146_dn0, var_t1__blk1146_dn2, var_t1__blk1146_dn6, var_t1__blk1146_dn7, var_t1__blk1146_dn10, var_t1__blk1146_dn11, var_t1__blk1146_dn12, var_t1__blk1146_dn13, var_t1__blk1146_dn15, var_t1__blk1146_dn16, var_t1__blk1146_dn17, var_t1__blk1146_dn18,)
    }
};
        var_t1__blk1146 = assign34920_e49886;
        var_t1__blk1146_dn0 = assign34920_e49886_d_n0;
        var_t1__blk1146_dn2 = assign34920_e49886_d_n2;
        var_t1__blk1146_dn6 = assign34920_e49886_d_n6;
        var_t1__blk1146_dn7 = assign34920_e49886_d_n7;
        var_t1__blk1146_dn10 = assign34920_e49886_d_n10;
        var_t1__blk1146_dn11 = assign34920_e49886_d_n11;
        var_t1__blk1146_dn12 = assign34920_e49886_d_n12;
        var_t1__blk1146_dn13 = assign34920_e49886_d_n13;
        var_t1__blk1146_dn15 = assign34920_e49886_d_n15;
        var_t1__blk1146_dn16 = assign34920_e49886_d_n16;
        var_t1__blk1146_dn17 = assign34920_e49886_d_n17;
        var_t1__blk1146_dn18 = assign34920_e49886_d_n18;

        let (assign34930_e49900, assign34930_e49900_d_n0, assign34930_e49900_d_n2, assign34930_e49900_d_n6, assign34930_e49900_d_n7, assign34930_e49900_d_n10, assign34930_e49900_d_n11, assign34930_e49900_d_n12, assign34930_e49900_d_n13, assign34930_e49900_d_n15, assign34930_e49900_d_n16, assign34930_e49900_d_n17, assign34930_e49900_d_n18,) = {
    if (var_guard1151 != 0.0) {
        let assign34930_e49890: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign34930_e49892: f64 = (assign34930_e49890 * var_beta_inv);
        let assign34930_e49894: f64 = (assign34930_e49892 * var_t1__blk1146);
        let assign34930_e49896: f64 = (assign34930_e49894 * var_t1__blk1146);
        let assign34930_e49898: f64 = (assign34930_e49896 / var_gds0_ign);
        (assign34930_e49898, ((((((assign34930_e49892 * var_t1__blk1146_dn0) * var_t1__blk1146) + (assign34930_e49894 * var_t1__blk1146_dn0)) * var_gds0_ign) - (assign34930_e49896 * var_gds0_ign_dn0)) / (var_gds0_ign * var_gds0_ign)), ((((((assign34930_e49892 * var_t1__blk1146_dn2) * var_t1__blk1146) + (assign34930_e49894 * var_t1__blk1146_dn2)) * var_gds0_ign) - (assign34930_e49896 * var_gds0_ign_dn2)) / (var_gds0_ign * var_gds0_ign)), ((((((assign34930_e49892 * var_t1__blk1146_dn6) * var_t1__blk1146) + (assign34930_e49894 * var_t1__blk1146_dn6)) * var_gds0_ign) - (assign34930_e49896 * var_gds0_ign_dn6)) / (var_gds0_ign * var_gds0_ign)), ((((((assign34930_e49892 * var_t1__blk1146_dn7) * var_t1__blk1146) + (assign34930_e49894 * var_t1__blk1146_dn7)) * var_gds0_ign) - (assign34930_e49896 * var_gds0_ign_dn7)) / (var_gds0_ign * var_gds0_ign)), ((((((((assign34930_e49890 * var_beta_inv_dn10) * var_t1__blk1146) + (assign34930_e49892 * var_t1__blk1146_dn10)) * var_t1__blk1146) + (assign34930_e49894 * var_t1__blk1146_dn10)) * var_gds0_ign) - (assign34930_e49896 * var_gds0_ign_dn10)) / (var_gds0_ign * var_gds0_ign)), ((((((assign34930_e49892 * var_t1__blk1146_dn11) * var_t1__blk1146) + (assign34930_e49894 * var_t1__blk1146_dn11)) * var_gds0_ign) - (assign34930_e49896 * var_gds0_ign_dn11)) / (var_gds0_ign * var_gds0_ign)), ((((((assign34930_e49892 * var_t1__blk1146_dn12) * var_t1__blk1146) + (assign34930_e49894 * var_t1__blk1146_dn12)) * var_gds0_ign) - (assign34930_e49896 * var_gds0_ign_dn12)) / (var_gds0_ign * var_gds0_ign)), ((((assign34930_e49892 * var_t1__blk1146_dn13) * var_t1__blk1146) + (assign34930_e49894 * var_t1__blk1146_dn13)) / var_gds0_ign), ((((assign34930_e49892 * var_t1__blk1146_dn15) * var_t1__blk1146) + (assign34930_e49894 * var_t1__blk1146_dn15)) / var_gds0_ign), ((((assign34930_e49892 * var_t1__blk1146_dn16) * var_t1__blk1146) + (assign34930_e49894 * var_t1__blk1146_dn16)) / var_gds0_ign), ((((((assign34930_e49892 * var_t1__blk1146_dn17) * var_t1__blk1146) + (assign34930_e49894 * var_t1__blk1146_dn17)) * var_gds0_ign) - (assign34930_e49896 * var_gds0_ign_dn17)) / (var_gds0_ign * var_gds0_ign)), ((((assign34930_e49892 * var_t1__blk1146_dn18) * var_t1__blk1146) + (assign34930_e49894 * var_t1__blk1146_dn18)) / var_gds0_ign),)
    } else {
        (var_nign0, var_nign0_dn0, var_nign0_dn2, var_nign0_dn6, var_nign0_dn7, var_nign0_dn10, var_nign0_dn11, var_nign0_dn12, var_nign0_dn13, var_nign0_dn15, var_nign0_dn16, var_nign0_dn17, var_nign0_dn18,)
    }
};
        var_nign0 = assign34930_e49900;
        var_nign0_dn0 = assign34930_e49900_d_n0;
        var_nign0_dn2 = assign34930_e49900_d_n2;
        var_nign0_dn6 = assign34930_e49900_d_n6;
        var_nign0_dn7 = assign34930_e49900_d_n7;
        var_nign0_dn10 = assign34930_e49900_d_n10;
        var_nign0_dn11 = assign34930_e49900_d_n11;
        var_nign0_dn12 = assign34930_e49900_d_n12;
        var_nign0_dn13 = assign34930_e49900_d_n13;
        var_nign0_dn15 = assign34930_e49900_d_n15;
        var_nign0_dn16 = assign34930_e49900_d_n16;
        var_nign0_dn17 = assign34930_e49900_d_n17;
        var_nign0_dn18 = assign34930_e49900_d_n18;

        let assign34940_e49904: f64 = (10.0 * 2.220446049250313e-16);
        let assign34940_e49909: f64 = (10.0 * 2.220446049250313e-16);
        let assign34940_e49911: f64 = if ((var_kusai00l > assign34940_e49904) && (var_vds > assign34940_e49909)) { 1.0 } else { 0.0 };
        var_guard1152 = assign34940_e49911;

        let (assign34950_e49919, assign34950_e49919_d_n0, assign34950_e49919_d_n2, assign34950_e49919_d_n6, assign34950_e49919_d_n7, assign34950_e49919_d_n10, assign34950_e49919_d_n11, assign34950_e49919_d_n12, assign34950_e49919_d_n17,) = {
    if ((var_guard1151 != 0.0) && (var_guard1152 != 0.0)) {
        let assign34950_e49917: f64 = (var_muun / var_mu);
        (assign34950_e49917, (((var_muun_dn0 * var_mu) - (var_muun * var_mu_dn0)) / (var_mu * var_mu)), (((var_muun_dn2 * var_mu) - (var_muun * var_mu_dn2)) / (var_mu * var_mu)), (((var_muun_dn6 * var_mu) - (var_muun * var_mu_dn6)) / (var_mu * var_mu)), (((var_muun_dn7 * var_mu) - (var_muun * var_mu_dn7)) / (var_mu * var_mu)), (((var_muun_dn10 * var_mu) - (var_muun * var_mu_dn10)) / (var_mu * var_mu)), (((var_muun_dn11 * var_mu) - (var_muun * var_mu_dn11)) / (var_mu * var_mu)), (((var_muun_dn12 * var_mu) - (var_muun * var_mu_dn12)) / (var_mu * var_mu)), (((var_muun_dn17 * var_mu) - (var_muun * var_mu_dn17)) / (var_mu * var_mu)),)
    } else {
        (var_mumoda, var_mumoda_dn0, var_mumoda_dn2, var_mumoda_dn6, var_mumoda_dn7, var_mumoda_dn10, var_mumoda_dn11, var_mumoda_dn12, var_mumoda_dn17,)
    }
};
        var_mumoda = assign34950_e49919;
        var_mumoda_dn0 = assign34950_e49919_d_n0;
        var_mumoda_dn2 = assign34950_e49919_d_n2;
        var_mumoda_dn6 = assign34950_e49919_d_n6;
        var_mumoda_dn7 = assign34950_e49919_d_n7;
        var_mumoda_dn10 = assign34950_e49919_d_n10;
        var_mumoda_dn11 = assign34950_e49919_d_n11;
        var_mumoda_dn12 = assign34950_e49919_d_n12;
        var_mumoda_dn17 = assign34950_e49919_d_n17;

        let (assign34960_e49931, assign34960_e49931_d_n0, assign34960_e49931_d_n2, assign34960_e49931_d_n6, assign34960_e49931_d_n7, assign34960_e49931_d_n10, assign34960_e49931_d_n11, assign34960_e49931_d_n12, assign34960_e49931_d_n17,) = {
    if ((var_guard1151 != 0.0) && (var_guard1152 != 0.0)) {
        let assign34960_e49925: f64 = (var_muun / var_mud_hoso);
        let assign34960_e49927: f64 = (assign34960_e49925 - var_mumoda);
        let assign34960_e49929: f64 = (assign34960_e49927 / var_vds);
        (assign34960_e49929, (((((((var_muun_dn0 * var_mud_hoso) - (var_muun * var_mud_hoso_dn0)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn0) * var_vds) - (assign34960_e49927 * var_vds_dn0)) / (var_vds * var_vds)), (((((((var_muun_dn2 * var_mud_hoso) - (var_muun * var_mud_hoso_dn2)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn2) * var_vds) - (assign34960_e49927 * var_vds_dn2)) / (var_vds * var_vds)), (((((((var_muun_dn6 * var_mud_hoso) - (var_muun * var_mud_hoso_dn6)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn6) * var_vds) - (assign34960_e49927 * var_vds_dn6)) / (var_vds * var_vds)), (((((((var_muun_dn7 * var_mud_hoso) - (var_muun * var_mud_hoso_dn7)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn7) * var_vds) - (assign34960_e49927 * var_vds_dn7)) / (var_vds * var_vds)), (((((((var_muun_dn10 * var_mud_hoso) - (var_muun * var_mud_hoso_dn10)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn10) * var_vds) - (assign34960_e49927 * var_vds_dn10)) / (var_vds * var_vds)), (((((((var_muun_dn11 * var_mud_hoso) - (var_muun * var_mud_hoso_dn11)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn11) * var_vds) - (assign34960_e49927 * var_vds_dn11)) / (var_vds * var_vds)), (((((((var_muun_dn12 * var_mud_hoso) - (var_muun * var_mud_hoso_dn12)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn12) * var_vds) - (assign34960_e49927 * var_vds_dn12)) / (var_vds * var_vds)), (((((((var_muun_dn17 * var_mud_hoso) - (var_muun * var_mud_hoso_dn17)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn17) * var_vds) - (assign34960_e49927 * var_vds_dn17)) / (var_vds * var_vds)),)
    } else {
        (var_mumodb, var_mumodb_dn0, var_mumodb_dn2, var_mumodb_dn6, var_mumodb_dn7, var_mumodb_dn10, var_mumodb_dn11, var_mumodb_dn12, var_mumodb_dn17,)
    }
};
        var_mumodb = assign34960_e49931;
        var_mumodb_dn0 = assign34960_e49931_d_n0;
        var_mumodb_dn2 = assign34960_e49931_d_n2;
        var_mumodb_dn6 = assign34960_e49931_d_n6;
        var_mumodb_dn7 = assign34960_e49931_d_n7;
        var_mumodb_dn10 = assign34960_e49931_d_n10;
        var_mumodb_dn11 = assign34960_e49931_d_n11;
        var_mumodb_dn12 = assign34960_e49931_d_n12;
        var_mumodb_dn17 = assign34960_e49931_d_n17;

        let (assign34970_e49953, assign34970_e49953_d_n0, assign34970_e49953_d_n2, assign34970_e49953_d_n6, assign34970_e49953_d_n7, assign34970_e49953_d_n10, assign34970_e49953_d_n11, assign34970_e49953_d_n12, assign34970_e49953_d_n17,) = {
    if ((var_guard1151 != 0.0) && (var_guard1152 != 0.0)) {
        let assign34970_e49938: f64 = (0.6666666666666667 * var_mumodb);
        let assign34970_e49942: f64 = (var_vgvt * var_sqrtkusail);
        let assign34970_e49943: f64 = (var_kusai00 + assign34970_e49942);
        let assign34970_e49945: f64 = (assign34970_e49943 + var_kusail);
        let assign34970_e49946: f64 = (assign34970_e49938 * assign34970_e49945);
        let assign34970_e49949: f64 = (var_vgvt + var_sqrtkusail);
        let assign34970_e49950: f64 = (assign34970_e49946 / assign34970_e49949);
        let assign34970_e49951: f64 = (var_mumoda + assign34970_e49950);
        (assign34970_e49951, (var_mumoda_dn0 + ((((((0.6666666666666667 * var_mumodb_dn0) * assign34970_e49945) + (assign34970_e49938 * ((var_kusai00_dn0 + ((var_vgvt_dn0 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn0))) + var_kusail_dn0))) * assign34970_e49949) - (assign34970_e49946 * (var_vgvt_dn0 + var_sqrtkusail_dn0))) / (assign34970_e49949 * assign34970_e49949))), (var_mumoda_dn2 + ((((((0.6666666666666667 * var_mumodb_dn2) * assign34970_e49945) + (assign34970_e49938 * ((var_kusai00_dn2 + ((var_vgvt_dn2 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn2))) + var_kusail_dn2))) * assign34970_e49949) - (assign34970_e49946 * (var_vgvt_dn2 + var_sqrtkusail_dn2))) / (assign34970_e49949 * assign34970_e49949))), (var_mumoda_dn6 + ((((((0.6666666666666667 * var_mumodb_dn6) * assign34970_e49945) + (assign34970_e49938 * ((var_kusai00_dn6 + ((var_vgvt_dn6 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn6))) + var_kusail_dn6))) * assign34970_e49949) - (assign34970_e49946 * (var_vgvt_dn6 + var_sqrtkusail_dn6))) / (assign34970_e49949 * assign34970_e49949))), (var_mumoda_dn7 + ((((((0.6666666666666667 * var_mumodb_dn7) * assign34970_e49945) + (assign34970_e49938 * ((var_kusai00_dn7 + ((var_vgvt_dn7 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn7))) + var_kusail_dn7))) * assign34970_e49949) - (assign34970_e49946 * (var_vgvt_dn7 + var_sqrtkusail_dn7))) / (assign34970_e49949 * assign34970_e49949))), (var_mumoda_dn10 + ((((((0.6666666666666667 * var_mumodb_dn10) * assign34970_e49945) + (assign34970_e49938 * ((var_kusai00_dn10 + ((var_vgvt_dn10 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn10))) + var_kusail_dn10))) * assign34970_e49949) - (assign34970_e49946 * (var_vgvt_dn10 + var_sqrtkusail_dn10))) / (assign34970_e49949 * assign34970_e49949))), (var_mumoda_dn11 + ((((((0.6666666666666667 * var_mumodb_dn11) * assign34970_e49945) + (assign34970_e49938 * ((var_kusai00_dn11 + ((var_vgvt_dn11 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn11))) + var_kusail_dn11))) * assign34970_e49949) - (assign34970_e49946 * (var_vgvt_dn11 + var_sqrtkusail_dn11))) / (assign34970_e49949 * assign34970_e49949))), (var_mumoda_dn12 + ((((((0.6666666666666667 * var_mumodb_dn12) * assign34970_e49945) + (assign34970_e49938 * ((var_kusai00_dn12 + ((var_vgvt_dn12 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn12))) + var_kusail_dn12))) * assign34970_e49949) - (assign34970_e49946 * (var_vgvt_dn12 + var_sqrtkusail_dn12))) / (assign34970_e49949 * assign34970_e49949))), (var_mumoda_dn17 + ((((((0.6666666666666667 * var_mumodb_dn17) * assign34970_e49945) + (assign34970_e49938 * ((var_kusai00_dn17 + ((var_vgvt_dn17 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn17))) + var_kusail_dn17))) * assign34970_e49949) - (assign34970_e49946 * (var_vgvt_dn17 + var_sqrtkusail_dn17))) / (assign34970_e49949 * assign34970_e49949))),)
    } else {
        (var_correct_w1, var_correct_w1_dn0, var_correct_w1_dn2, var_correct_w1_dn6, var_correct_w1_dn7, var_correct_w1_dn10, var_correct_w1_dn11, var_correct_w1_dn12, var_correct_w1_dn17,)
    }
};
        var_correct_w1 = assign34970_e49953;
        var_correct_w1_dn0 = assign34970_e49953_d_n0;
        var_correct_w1_dn2 = assign34970_e49953_d_n2;
        var_correct_w1_dn6 = assign34970_e49953_d_n6;
        var_correct_w1_dn7 = assign34970_e49953_d_n7;
        var_correct_w1_dn10 = assign34970_e49953_d_n10;
        var_correct_w1_dn11 = assign34970_e49953_d_n11;
        var_correct_w1_dn12 = assign34970_e49953_d_n12;
        var_correct_w1_dn17 = assign34970_e49953_d_n17;

        let (assign34980_e49962, assign34980_e49962_d_n0, assign34980_e49962_d_n2, assign34980_e49962_d_n6, assign34980_e49962_d_n7, assign34980_e49962_d_n10, assign34980_e49962_d_n11, assign34980_e49962_d_n12, assign34980_e49962_d_n17,) = {
    if ((var_guard1151 != 0.0) && (var_guard1152 == 0.0)) {
        let assign34980_e49960: f64 = (var_muun / var_mud_hoso);
        (assign34980_e49960, (((var_muun_dn0 * var_mud_hoso) - (var_muun * var_mud_hoso_dn0)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn2 * var_mud_hoso) - (var_muun * var_mud_hoso_dn2)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn6 * var_mud_hoso) - (var_muun * var_mud_hoso_dn6)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn7 * var_mud_hoso) - (var_muun * var_mud_hoso_dn7)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn10 * var_mud_hoso) - (var_muun * var_mud_hoso_dn10)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn11 * var_mud_hoso) - (var_muun * var_mud_hoso_dn11)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn12 * var_mud_hoso) - (var_muun * var_mud_hoso_dn12)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn17 * var_mud_hoso) - (var_muun * var_mud_hoso_dn17)) / (var_mud_hoso * var_mud_hoso)),)
    } else {
        (var_correct_w1, var_correct_w1_dn0, var_correct_w1_dn2, var_correct_w1_dn6, var_correct_w1_dn7, var_correct_w1_dn10, var_correct_w1_dn11, var_correct_w1_dn12, var_correct_w1_dn17,)
    }
};
        var_correct_w1 = assign34980_e49962;
        var_correct_w1_dn0 = assign34980_e49962_d_n0;
        var_correct_w1_dn2 = assign34980_e49962_d_n2;
        var_correct_w1_dn6 = assign34980_e49962_d_n6;
        var_correct_w1_dn7 = assign34980_e49962_d_n7;
        var_correct_w1_dn10 = assign34980_e49962_d_n10;
        var_correct_w1_dn11 = assign34980_e49962_d_n11;
        var_correct_w1_dn12 = assign34980_e49962_d_n12;
        var_correct_w1_dn17 = assign34980_e49962_d_n17;

        let (assign34990_e49972, assign34990_e49972_d_n0, assign34990_e49972_d_n2, assign34990_e49972_d_n6, assign34990_e49972_d_n7, assign34990_e49972_d_n10, assign34990_e49972_d_n11, assign34990_e49972_d_n12, assign34990_e49972_d_n13, assign34990_e49972_d_n15, assign34990_e49972_d_n16, assign34990_e49972_d_n17, assign34990_e49972_d_n18,) = {
    if (var_guard1151 != 0.0) {
        let assign34990_e49966: f64 = (var_mfactor * var_nign0);
        let assign34990_e49968: f64 = (assign34990_e49966 * var_kusai_ig);
        let assign34990_e49970: f64 = (assign34990_e49968 * var_correct_w1);
        (assign34990_e49970, (((((var_mfactor * var_nign0_dn0) * var_kusai_ig) + (assign34990_e49966 * var_kusai_ig_dn0)) * var_correct_w1) + (assign34990_e49968 * var_correct_w1_dn0)), (((((var_mfactor * var_nign0_dn2) * var_kusai_ig) + (assign34990_e49966 * var_kusai_ig_dn2)) * var_correct_w1) + (assign34990_e49968 * var_correct_w1_dn2)), (((((var_mfactor * var_nign0_dn6) * var_kusai_ig) + (assign34990_e49966 * var_kusai_ig_dn6)) * var_correct_w1) + (assign34990_e49968 * var_correct_w1_dn6)), (((((var_mfactor * var_nign0_dn7) * var_kusai_ig) + (assign34990_e49966 * var_kusai_ig_dn7)) * var_correct_w1) + (assign34990_e49968 * var_correct_w1_dn7)), (((((var_mfactor * var_nign0_dn10) * var_kusai_ig) + (assign34990_e49966 * var_kusai_ig_dn10)) * var_correct_w1) + (assign34990_e49968 * var_correct_w1_dn10)), (((((var_mfactor * var_nign0_dn11) * var_kusai_ig) + (assign34990_e49966 * var_kusai_ig_dn11)) * var_correct_w1) + (assign34990_e49968 * var_correct_w1_dn11)), (((((var_mfactor * var_nign0_dn12) * var_kusai_ig) + (assign34990_e49966 * var_kusai_ig_dn12)) * var_correct_w1) + (assign34990_e49968 * var_correct_w1_dn12)), (((var_mfactor * var_nign0_dn13) * var_kusai_ig) * var_correct_w1), (((var_mfactor * var_nign0_dn15) * var_kusai_ig) * var_correct_w1), (((var_mfactor * var_nign0_dn16) * var_kusai_ig) * var_correct_w1), (((((var_mfactor * var_nign0_dn17) * var_kusai_ig) + (assign34990_e49966 * var_kusai_ig_dn17)) * var_correct_w1) + (assign34990_e49968 * var_correct_w1_dn17)), (((var_mfactor * var_nign0_dn18) * var_kusai_ig) * var_correct_w1),)
    } else {
        (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn6, var_noiigate_dn7, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12, var_noiigate_dn13, var_noiigate_dn15, var_noiigate_dn16, var_noiigate_dn17, var_noiigate_dn18,)
    }
};
        var_noiigate = assign34990_e49972;
        var_noiigate_dn0 = assign34990_e49972_d_n0;
        var_noiigate_dn2 = assign34990_e49972_d_n2;
        var_noiigate_dn6 = assign34990_e49972_d_n6;
        var_noiigate_dn7 = assign34990_e49972_d_n7;
        var_noiigate_dn10 = assign34990_e49972_d_n10;
        var_noiigate_dn11 = assign34990_e49972_d_n11;
        var_noiigate_dn12 = assign34990_e49972_d_n12;
        var_noiigate_dn13 = assign34990_e49972_d_n13;
        var_noiigate_dn15 = assign34990_e49972_d_n15;
        var_noiigate_dn16 = assign34990_e49972_d_n16;
        var_noiigate_dn17 = assign34990_e49972_d_n17;
        var_noiigate_dn18 = assign34990_e49972_d_n18;

        let (assign35000_e49976, assign35000_e49976_d_n0, assign35000_e49976_d_n2, assign35000_e49976_d_n6, assign35000_e49976_d_n7, assign35000_e49976_d_n10, assign35000_e49976_d_n11, assign35000_e49976_d_n12, assign35000_e49976_d_n17,) = {
    if (var_guard1151 != 0.0) {
        (var_crl_f, var_crl_f_dn0, var_crl_f_dn2, var_crl_f_dn6, var_crl_f_dn7, var_crl_f_dn10, var_crl_f_dn11, var_crl_f_dn12, var_crl_f_dn17,)
    } else {
        (var_noicross, var_noicross_dn0, var_noicross_dn2, var_noicross_dn6, var_noicross_dn7, var_noicross_dn10, var_noicross_dn11, var_noicross_dn12, var_noicross_dn17,)
    }
};
        var_noicross = assign35000_e49976;
        var_noicross_dn0 = assign35000_e49976_d_n0;
        var_noicross_dn2 = assign35000_e49976_d_n2;
        var_noicross_dn6 = assign35000_e49976_d_n6;
        var_noicross_dn7 = assign35000_e49976_d_n7;
        var_noicross_dn10 = assign35000_e49976_d_n10;
        var_noicross_dn11 = assign35000_e49976_d_n11;
        var_noicross_dn12 = assign35000_e49976_d_n12;
        var_noicross_dn17 = assign35000_e49976_d_n17;

        let (assign35010_e49990, assign35010_e49990_d_n0, assign35010_e49990_d_n2, assign35010_e49990_d_n6, assign35010_e49990_d_n7, assign35010_e49990_d_n10, assign35010_e49990_d_n11, assign35010_e49990_d_n12, assign35010_e49990_d_n13, assign35010_e49990_d_n15, assign35010_e49990_d_n16, assign35010_e49990_d_n17, assign35010_e49990_d_n18,) = {
    if (var_guard1151 != 0.0) {
        let assign35010_e49979: f64 = (-var_t1__blk1146);
        let (assign35010_e49988, assign35010_e49988_d_n0, assign35010_e49988_d_n2, assign35010_e49988_d_n6, assign35010_e49988_d_n7, assign35010_e49988_d_n10, assign35010_e49988_d_n11, assign35010_e49988_d_n12, assign35010_e49988_d_n13, assign35010_e49988_d_n15, assign35010_e49988_d_n16, assign35010_e49988_d_n17, assign35010_e49988_d_n18,) = {
            if ((assign35010_e49979 > var_t0__blk1145) && (var_noiigate > 0.0)) {
                (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn6, var_noiigate_dn7, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12, var_noiigate_dn13, var_noiigate_dn15, var_noiigate_dn16, var_noiigate_dn17, var_noiigate_dn18,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign35010_e49988, assign35010_e49988_d_n0, assign35010_e49988_d_n2, assign35010_e49988_d_n6, assign35010_e49988_d_n7, assign35010_e49988_d_n10, assign35010_e49988_d_n11, assign35010_e49988_d_n12, assign35010_e49988_d_n13, assign35010_e49988_d_n15, assign35010_e49988_d_n16, assign35010_e49988_d_n17, assign35010_e49988_d_n18,)
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

        let (assign35020_e50000, assign35020_e50000_d_n0, assign35020_e50000_d_n2, assign35020_e50000_d_n6, assign35020_e50000_d_n7, assign35020_e50000_d_n10, assign35020_e50000_d_n11, assign35020_e50000_d_n12, assign35020_e50000_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35020_e49993: f64 = (-var_t1__blk1146);
        let (assign35020_e49998, assign35020_e49998_d_n0, assign35020_e49998_d_n2, assign35020_e49998_d_n6, assign35020_e49998_d_n7, assign35020_e49998_d_n10, assign35020_e49998_d_n11, assign35020_e49998_d_n12, assign35020_e49998_d_n17,) = {
            if (assign35020_e49993 > var_t0__blk1145) {
                (var_noicross, var_noicross_dn0, var_noicross_dn2, var_noicross_dn6, var_noicross_dn7, var_noicross_dn10, var_noicross_dn11, var_noicross_dn12, var_noicross_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign35020_e49998, assign35020_e49998_d_n0, assign35020_e49998_d_n2, assign35020_e49998_d_n6, assign35020_e49998_d_n7, assign35020_e49998_d_n10, assign35020_e49998_d_n11, assign35020_e49998_d_n12, assign35020_e49998_d_n17,)
    } else {
        (var_noicross, var_noicross_dn0, var_noicross_dn2, var_noicross_dn6, var_noicross_dn7, var_noicross_dn10, var_noicross_dn11, var_noicross_dn12, var_noicross_dn17,)
    }
};
        var_noicross = assign35020_e50000;
        var_noicross_dn0 = assign35020_e50000_d_n0;
        var_noicross_dn2 = assign35020_e50000_d_n2;
        var_noicross_dn6 = assign35020_e50000_d_n6;
        var_noicross_dn7 = assign35020_e50000_d_n7;
        var_noicross_dn10 = assign35020_e50000_d_n10;
        var_noicross_dn11 = assign35020_e50000_d_n11;
        var_noicross_dn12 = assign35020_e50000_d_n12;
        var_noicross_dn17 = assign35020_e50000_d_n17;

        let (assign35030_e50005, assign35030_e50005_d_n0, assign35030_e50005_d_n2, assign35030_e50005_d_n6, assign35030_e50005_d_n7, assign35030_e50005_d_n10, assign35030_e50005_d_n11, assign35030_e50005_d_n12, assign35030_e50005_d_n13, assign35030_e50005_d_n15, assign35030_e50005_d_n16, assign35030_e50005_d_n17, assign35030_e50005_d_n18,) = {
    if (var_guard1151 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn6, var_noiigate_dn7, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12, var_noiigate_dn13, var_noiigate_dn15, var_noiigate_dn16, var_noiigate_dn17, var_noiigate_dn18,)
    }
};
        var_noiigate = assign35030_e50005;
        var_noiigate_dn0 = assign35030_e50005_d_n0;
        var_noiigate_dn2 = assign35030_e50005_d_n2;
        var_noiigate_dn6 = assign35030_e50005_d_n6;
        var_noiigate_dn7 = assign35030_e50005_d_n7;
        var_noiigate_dn10 = assign35030_e50005_d_n10;
        var_noiigate_dn11 = assign35030_e50005_d_n11;
        var_noiigate_dn12 = assign35030_e50005_d_n12;
        var_noiigate_dn13 = assign35030_e50005_d_n13;
        var_noiigate_dn15 = assign35030_e50005_d_n15;
        var_noiigate_dn16 = assign35030_e50005_d_n16;
        var_noiigate_dn17 = assign35030_e50005_d_n17;
        var_noiigate_dn18 = assign35030_e50005_d_n18;

        let (assign35040_e50010, assign35040_e50010_d_n0, assign35040_e50010_d_n2, assign35040_e50010_d_n6, assign35040_e50010_d_n7, assign35040_e50010_d_n10, assign35040_e50010_d_n11, assign35040_e50010_d_n12, assign35040_e50010_d_n17,) = {
    if (var_guard1151 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_noicross, var_noicross_dn0, var_noicross_dn2, var_noicross_dn6, var_noicross_dn7, var_noicross_dn10, var_noicross_dn11, var_noicross_dn12, var_noicross_dn17,)
    }
};
        var_noicross = assign35040_e50010;
        var_noicross_dn0 = assign35040_e50010_d_n0;
        var_noicross_dn2 = assign35040_e50010_d_n2;
        var_noicross_dn6 = assign35040_e50010_d_n6;
        var_noicross_dn7 = assign35040_e50010_d_n7;
        var_noicross_dn10 = assign35040_e50010_d_n10;
        var_noicross_dn11 = assign35040_e50010_d_n11;
        var_noicross_dn12 = assign35040_e50010_d_n12;
        var_noicross_dn17 = assign35040_e50010_d_n17;

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

        let assign35090_e50017: f64 = if p.p259 == 1.0 { 1.0 } else { 0.0 };
        var_guard1153 = assign35090_e50017;

        let (assign35100_e50021,) = {
    if (var_guard1153 != 0.0) {
        (1.0,)
    } else {
        (var_rdmod,)
    }
};
        var_rdmod = assign35100_e50021;

        let assign35110_e50024: f64 = if var_rdmod == 1.0 { 1.0 } else { 0.0 };
        var_guard1173 = assign35110_e50024;

        let (assign35120_e50032,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 != 0.0)) {
        let assign35120_e50030: f64 = (p.p264 / 1e-6);
        (assign35120_e50030,)
    } else {
        (var_nover,)
    }
};
        var_nover = assign35120_e50032;

        let (assign35130_e50038,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 != 0.0)) {
        (p.p266,)
    } else {
        (var_mks_rdrmue,)
    }
};
        var_mks_rdrmue = assign35130_e50038;

        let (assign35140_e50044,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 != 0.0)) {
        (p.p268,)
    } else {
        (var_mks_rdrvmax,)
    }
};
        var_mks_rdrvmax = assign35140_e50044;

        let (assign35150_e50050, assign35150_e50050_d_n10,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (var_rrdrbb, var_rrdrbb_dn10,)
    }
};
        var_rrdrbb = assign35150_e50050;
        var_rrdrbb_dn10 = assign35150_e50050_d_n10;

        let (assign35160_e50063,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 != 0.0)) {
        let (assign35160_e50061,) = {
            if (p.p263 > 0.0) {
                let assign35160_e50059: f64 = (p.p263 * p.p255);
                (assign35160_e50059,)
            } else {
                (0.0,)
            }
        };
        (assign35160_e50061,)
    } else {
        (var_rsd0,)
    }
};
        var_rsd0 = assign35160_e50063;

        let (assign35170_e50069,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 != 0.0)) {
        (p.p258,)
    } else {
        (var_ldrifte,)
    }
};
        var_ldrifte = assign35170_e50069;

        let (assign35180_e50077, assign35180_e50077_d_n0, assign35180_e50077_d_n2, assign35180_e50077_d_n6, assign35180_e50077_d_n7,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 != 0.0)) {
        let assign35180_e50075: f64 = (p.p50 * (nv7 - nv2));
        (assign35180_e50075, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (var_vrdr, var_vrdr_dn0, var_vrdr_dn2, var_vrdr_dn6, var_vrdr_dn7,)
    }
};
        var_vrdr = assign35180_e50077;
        var_vrdr_dn0 = assign35180_e50077_d_n0;
        var_vrdr_dn2 = assign35180_e50077_d_n2;
        var_vrdr_dn6 = assign35180_e50077_d_n6;
        var_vrdr_dn7 = assign35180_e50077_d_n7;

        let (assign35190_e50086,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 == 0.0)) {
        let assign35190_e50084: f64 = (p.p59 / 1e-6);
        (assign35190_e50084,)
    } else {
        (var_nover,)
    }
};
        var_nover = assign35190_e50086;

        let (assign35200_e50093,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 == 0.0)) {
        (p.p265,)
    } else {
        (var_mks_rdrmue,)
    }
};
        var_mks_rdrmue = assign35200_e50093;

        let (assign35210_e50100,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 == 0.0)) {
        (p.p267,)
    } else {
        (var_mks_rdrvmax,)
    }
};
        var_mks_rdrvmax = assign35210_e50100;

        let (assign35220_e50107, assign35220_e50107_d_n10,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (var_rrdrbb, var_rrdrbb_dn10,)
    }
};
        var_rrdrbb = assign35220_e50107;
        var_rrdrbb_dn10 = assign35220_e50107_d_n10;

        let (assign35230_e50121,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 == 0.0)) {
        let (assign35230_e50119,) = {
            if (p.p263 > 0.0) {
                let assign35230_e50117: f64 = (p.p263 * p.p256);
                (assign35230_e50117,)
            } else {
                (0.0,)
            }
        };
        (assign35230_e50119,)
    } else {
        (var_rsd0,)
    }
};
        var_rsd0 = assign35230_e50121;

        let (assign35240_e50128,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 == 0.0)) {
        (p.p257,)
    } else {
        (var_ldrifte,)
    }
};
        var_ldrifte = assign35240_e50128;

        let (assign35250_e50137, assign35250_e50137_d_n0, assign35250_e50137_d_n2, assign35250_e50137_d_n6, assign35250_e50137_d_n7,) = {
    if ((var_guard1153 != 0.0) && (var_guard1173 == 0.0)) {
        let assign35250_e50135: f64 = (p.p50 * (nv0 - nv6));
        (assign35250_e50135, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (var_vrdr, var_vrdr_dn0, var_vrdr_dn2, var_vrdr_dn6, var_vrdr_dn7,)
    }
};
        var_vrdr = assign35250_e50137;
        var_vrdr_dn0 = assign35250_e50137_d_n0;
        var_vrdr_dn2 = assign35250_e50137_d_n2;
        var_vrdr_dn6 = assign35250_e50137_d_n6;
        var_vrdr_dn7 = assign35250_e50137_d_n7;

        let (assign35260_e50148,) = {
    if (var_guard1153 != 0.0) {
        let assign35260_e50141: f64 = (p.p271 * p.p271);
        let assign35260_e50144: f64 = (p.p56 * p.p56);
        let assign35260_e50145: f64 = (assign35260_e50141 + assign35260_e50144);
        let assign35260_e50146: f64 = (assign35260_e50145).sqrt();
        (assign35260_e50146,)
    } else {
        (var_xov,)
    }
};
        var_xov = assign35260_e50148;

        let (assign35270_e50154,) = {
    if (var_guard1153 != 0.0) {
        let assign35270_e50152: f64 = (var_weff * p.p9);
        (assign35270_e50152,)
    } else {
        (var_weff_nf_1,)
    }
};
        var_weff_nf_1 = assign35270_e50154;

        let (assign35280_e50160,) = {
    if (var_guard1153 != 0.0) {
        let assign35280_e50158: f64 = (var_mks_rdrmue / 10000.0);
        (assign35280_e50158,)
    } else {
        (var_mks_rdrmue,)
    }
};
        var_mks_rdrmue = assign35280_e50160;

        let (assign35290_e50166,) = {
    if (var_guard1153 != 0.0) {
        let assign35290_e50164: f64 = (var_mks_rdrvmax / 100.0);
        (assign35290_e50164,)
    } else {
        (var_mks_rdrvmax,)
    }
};
        var_mks_rdrvmax = assign35290_e50166;

        let (assign35300_e50172, assign35300_e50172_d_n10,) = {
    if (var_guard1153 != 0.0) {
        let assign35300_e50170: f64 = (var_ttemp / var_uc_tnom);
        (assign35300_e50170, (var_ttemp_dn10 / var_uc_tnom),)
    } else {
        (var_tratio, var_tratio_dn10,)
    }
};
        var_tratio = assign35300_e50172;
        var_tratio_dn10 = assign35300_e50172_d_n10;

        *var_correct_w1_slot = var_correct_w1;
        *var_correct_w1_dn0_slot = var_correct_w1_dn0;
        *var_correct_w1_dn10_slot = var_correct_w1_dn10;
        *var_correct_w1_dn11_slot = var_correct_w1_dn11;
        *var_correct_w1_dn12_slot = var_correct_w1_dn12;
        *var_correct_w1_dn17_slot = var_correct_w1_dn17;
        *var_correct_w1_dn2_slot = var_correct_w1_dn2;
        *var_correct_w1_dn6_slot = var_correct_w1_dn6;
        *var_correct_w1_dn7_slot = var_correct_w1_dn7;
        *var_guard1152_slot = var_guard1152;
        *var_guard1153_slot = var_guard1153;
        *var_guard1173_slot = var_guard1173;
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
        *var_t1__blk1146_slot = var_t1__blk1146;
        *var_t1__blk1146_dn0_slot = var_t1__blk1146_dn0;
        *var_t1__blk1146_dn10_slot = var_t1__blk1146_dn10;
        *var_t1__blk1146_dn11_slot = var_t1__blk1146_dn11;
        *var_t1__blk1146_dn12_slot = var_t1__blk1146_dn12;
        *var_t1__blk1146_dn13_slot = var_t1__blk1146_dn13;
        *var_t1__blk1146_dn15_slot = var_t1__blk1146_dn15;
        *var_t1__blk1146_dn16_slot = var_t1__blk1146_dn16;
        *var_t1__blk1146_dn17_slot = var_t1__blk1146_dn17;
        *var_t1__blk1146_dn18_slot = var_t1__blk1146_dn18;
        *var_t1__blk1146_dn2_slot = var_t1__blk1146_dn2;
        *var_t1__blk1146_dn6_slot = var_t1__blk1146_dn6;
        *var_t1__blk1146_dn7_slot = var_t1__blk1146_dn7;
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
        var_guard1153: f64,
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
        var_guard1174_slot: &mut f64,
        var_guard1175_slot: &mut f64,
        var_guard1176_slot: &mut f64,
        var_guard1177_slot: &mut f64,
        var_guard1178_slot: &mut f64,
        var_guard1179_slot: &mut f64,
        var_mu0_slot: &mut f64,
        var_mu0_dn0_slot: &mut f64,
        var_mu0_dn10_slot: &mut f64,
        var_mu0_dn11_slot: &mut f64,
        var_mu0_dn12_slot: &mut f64,
        var_mu0_dn17_slot: &mut f64,
        var_mu0_dn2_slot: &mut f64,
        var_mu0_dn6_slot: &mut f64,
        var_mu0_dn7_slot: &mut f64,
        var_mu__blk1169_slot: &mut f64,
        var_mu__blk1169_dn0_slot: &mut f64,
        var_mu__blk1169_dn10_slot: &mut f64,
        var_mu__blk1169_dn11_slot: &mut f64,
        var_mu__blk1169_dn12_slot: &mut f64,
        var_mu__blk1169_dn17_slot: &mut f64,
        var_mu__blk1169_dn2_slot: &mut f64,
        var_mu__blk1169_dn6_slot: &mut f64,
        var_mu__blk1169_dn7_slot: &mut f64,
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
        var_vmaxe__blk1166_slot: &mut f64,
        var_vmaxe__blk1166_dn0_slot: &mut f64,
        var_vmaxe__blk1166_dn10_slot: &mut f64,
        var_vmaxe__blk1166_dn11_slot: &mut f64,
        var_vmaxe__blk1166_dn12_slot: &mut f64,
        var_vmaxe__blk1166_dn17_slot: &mut f64,
        var_vmaxe__blk1166_dn2_slot: &mut f64,
        var_vmaxe__blk1166_dn6_slot: &mut f64,
        var_vmaxe__blk1166_dn7_slot: &mut f64,
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
        let mut var_guard1174: f64 = *var_guard1174_slot;
        let mut var_guard1175: f64 = *var_guard1175_slot;
        let mut var_guard1176: f64 = *var_guard1176_slot;
        let mut var_guard1177: f64 = *var_guard1177_slot;
        let mut var_guard1178: f64 = *var_guard1178_slot;
        let mut var_guard1179: f64 = *var_guard1179_slot;
        let mut var_mu0: f64 = *var_mu0_slot;
        let mut var_mu0_dn0: f64 = *var_mu0_dn0_slot;
        let mut var_mu0_dn10: f64 = *var_mu0_dn10_slot;
        let mut var_mu0_dn11: f64 = *var_mu0_dn11_slot;
        let mut var_mu0_dn12: f64 = *var_mu0_dn12_slot;
        let mut var_mu0_dn17: f64 = *var_mu0_dn17_slot;
        let mut var_mu0_dn2: f64 = *var_mu0_dn2_slot;
        let mut var_mu0_dn6: f64 = *var_mu0_dn6_slot;
        let mut var_mu0_dn7: f64 = *var_mu0_dn7_slot;
        let mut var_mu__blk1169: f64 = *var_mu__blk1169_slot;
        let mut var_mu__blk1169_dn0: f64 = *var_mu__blk1169_dn0_slot;
        let mut var_mu__blk1169_dn10: f64 = *var_mu__blk1169_dn10_slot;
        let mut var_mu__blk1169_dn11: f64 = *var_mu__blk1169_dn11_slot;
        let mut var_mu__blk1169_dn12: f64 = *var_mu__blk1169_dn12_slot;
        let mut var_mu__blk1169_dn17: f64 = *var_mu__blk1169_dn17_slot;
        let mut var_mu__blk1169_dn2: f64 = *var_mu__blk1169_dn2_slot;
        let mut var_mu__blk1169_dn6: f64 = *var_mu__blk1169_dn6_slot;
        let mut var_mu__blk1169_dn7: f64 = *var_mu__blk1169_dn7_slot;
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
        let mut var_vmaxe__blk1166: f64 = *var_vmaxe__blk1166_slot;
        let mut var_vmaxe__blk1166_dn0: f64 = *var_vmaxe__blk1166_dn0_slot;
        let mut var_vmaxe__blk1166_dn10: f64 = *var_vmaxe__blk1166_dn10_slot;
        let mut var_vmaxe__blk1166_dn11: f64 = *var_vmaxe__blk1166_dn11_slot;
        let mut var_vmaxe__blk1166_dn12: f64 = *var_vmaxe__blk1166_dn12_slot;
        let mut var_vmaxe__blk1166_dn17: f64 = *var_vmaxe__blk1166_dn17_slot;
        let mut var_vmaxe__blk1166_dn2: f64 = *var_vmaxe__blk1166_dn2_slot;
        let mut var_vmaxe__blk1166_dn6: f64 = *var_vmaxe__blk1166_dn6_slot;
        let mut var_vmaxe__blk1166_dn7: f64 = *var_vmaxe__blk1166_dn7_slot;

        let (assign35310_e50178, assign35310_e50178_d_n0, assign35310_e50178_d_n2, assign35310_e50178_d_n6, assign35310_e50178_d_n7, assign35310_e50178_d_n10, assign35310_e50178_d_n11, assign35310_e50178_d_n12, assign35310_e50178_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35310_e50176: f64 = (var_tratio).powf(p.p269);
        (assign35310_e50176, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((var_tratio).powf(p.p269 - 1.0) * var_tratio_dn10)) } } else { (assign35310_e50176 * (p.p269 * (var_tratio_dn10 / var_tratio))) }, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35310_e50178;
        var_t1_dn0 = assign35310_e50178_d_n0;
        var_t1_dn2 = assign35310_e50178_d_n2;
        var_t1_dn6 = assign35310_e50178_d_n6;
        var_t1_dn7 = assign35310_e50178_d_n7;
        var_t1_dn10 = assign35310_e50178_d_n10;
        var_t1_dn11 = assign35310_e50178_d_n11;
        var_t1_dn12 = assign35310_e50178_d_n12;
        var_t1_dn17 = assign35310_e50178_d_n17;

        let (assign35320_e50184, assign35320_e50184_d_n0, assign35320_e50184_d_n2, assign35320_e50184_d_n6, assign35320_e50184_d_n7, assign35320_e50184_d_n10, assign35320_e50184_d_n11, assign35320_e50184_d_n12, assign35320_e50184_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35320_e50182: f64 = (var_mks_rdrmue / var_t1);
        (assign35320_e50182, (-((var_mks_rdrmue * var_t1_dn0) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn2) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn6) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn7) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn10) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn11) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn12) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn17) / (var_t1 * var_t1))),)
    } else {
        (var_mu0, var_mu0_dn0, var_mu0_dn2, var_mu0_dn6, var_mu0_dn7, var_mu0_dn10, var_mu0_dn11, var_mu0_dn12, var_mu0_dn17,)
    }
};
        var_mu0 = assign35320_e50184;
        var_mu0_dn0 = assign35320_e50184_d_n0;
        var_mu0_dn2 = assign35320_e50184_d_n2;
        var_mu0_dn6 = assign35320_e50184_d_n6;
        var_mu0_dn7 = assign35320_e50184_d_n7;
        var_mu0_dn10 = assign35320_e50184_d_n10;
        var_mu0_dn11 = assign35320_e50184_d_n11;
        var_mu0_dn12 = assign35320_e50184_d_n12;
        var_mu0_dn17 = assign35320_e50184_d_n17;

        let (assign35330_e50204, assign35330_e50204_d_n0, assign35330_e50204_d_n2, assign35330_e50204_d_n6, assign35330_e50204_d_n7, assign35330_e50204_d_n10, assign35330_e50204_d_n11, assign35330_e50204_d_n12, assign35330_e50204_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35330_e50189: f64 = (0.4 * var_tratio);
        let assign35330_e50190: f64 = (1.8 + assign35330_e50189);
        let assign35330_e50193: f64 = (0.1 * var_tratio);
        let assign35330_e50195: f64 = (assign35330_e50193 * var_tratio);
        let assign35330_e50196: f64 = (assign35330_e50190 + assign35330_e50195);
        let assign35330_e50200: f64 = (1.0 - var_tratio);
        let assign35330_e50201: f64 = (p.p270 * assign35330_e50200);
        let assign35330_e50202: f64 = (assign35330_e50196 - assign35330_e50201);
        (assign35330_e50202, 0.0, 0.0, 0.0, 0.0, (((0.4 * var_tratio_dn10) + (((0.1 * var_tratio_dn10) * var_tratio) + (assign35330_e50193 * var_tratio_dn10))) - (p.p270 * (-var_tratio_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn6, var_t0_dn7, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn17,)
    }
};
        var_t0 = assign35330_e50204;
        var_t0_dn0 = assign35330_e50204_d_n0;
        var_t0_dn2 = assign35330_e50204_d_n2;
        var_t0_dn6 = assign35330_e50204_d_n6;
        var_t0_dn7 = assign35330_e50204_d_n7;
        var_t0_dn10 = assign35330_e50204_d_n10;
        var_t0_dn11 = assign35330_e50204_d_n11;
        var_t0_dn12 = assign35330_e50204_d_n12;
        var_t0_dn17 = assign35330_e50204_d_n17;

        let (assign35340_e50210, assign35340_e50210_d_n0, assign35340_e50210_d_n2, assign35340_e50210_d_n6, assign35340_e50210_d_n7, assign35340_e50210_d_n10, assign35340_e50210_d_n11, assign35340_e50210_d_n12, assign35340_e50210_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35340_e50208: f64 = (var_mks_rdrvmax / var_t0);
        (assign35340_e50208, (-((var_mks_rdrvmax * var_t0_dn0) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn2) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn6) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn7) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn10) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn11) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn12) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn17) / (var_t0 * var_t0))),)
    } else {
        (var_vmaxe__blk1166, var_vmaxe__blk1166_dn0, var_vmaxe__blk1166_dn2, var_vmaxe__blk1166_dn6, var_vmaxe__blk1166_dn7, var_vmaxe__blk1166_dn10, var_vmaxe__blk1166_dn11, var_vmaxe__blk1166_dn12, var_vmaxe__blk1166_dn17,)
    }
};
        var_vmaxe__blk1166 = assign35340_e50210;
        var_vmaxe__blk1166_dn0 = assign35340_e50210_d_n0;
        var_vmaxe__blk1166_dn2 = assign35340_e50210_d_n2;
        var_vmaxe__blk1166_dn6 = assign35340_e50210_d_n6;
        var_vmaxe__blk1166_dn7 = assign35340_e50210_d_n7;
        var_vmaxe__blk1166_dn10 = assign35340_e50210_d_n10;
        var_vmaxe__blk1166_dn11 = assign35340_e50210_d_n11;
        var_vmaxe__blk1166_dn12 = assign35340_e50210_d_n12;
        var_vmaxe__blk1166_dn17 = assign35340_e50210_d_n17;

        let (assign35350_e50220, assign35350_e50220_d_n10,) = {
    if (var_guard1153 != 0.0) {
        let assign35350_e50216: f64 = (var_ttemp - var_uc_tnom);
        let assign35350_e50217: f64 = (p.p274 * assign35350_e50216);
        let assign35350_e50218: f64 = (var_rrdrbb + assign35350_e50217);
        (assign35350_e50218, (var_rrdrbb_dn10 + (p.p274 * var_ttemp_dn10)),)
    } else {
        (var_rrdrbb, var_rrdrbb_dn10,)
    }
};
        var_rrdrbb = assign35350_e50220;
        var_rrdrbb_dn10 = assign35350_e50220_d_n10;

        let (assign35360_e50230,) = {
    if (var_guard1153 != 0.0) {
        let assign35360_e50226: f64 = (var_lgle).powf(p.p280);
        let assign35360_e50227: f64 = (p.p279 / assign35360_e50226);
        let assign35360_e50228: f64 = (1.0 + assign35360_e50227);
        (assign35360_e50228,)
    } else {
        (var_rdrmuele,)
    }
};
        var_rdrmuele = assign35360_e50230;

        let (assign35370_e50240,) = {
    if (var_guard1153 != 0.0) {
        let assign35370_e50236: f64 = (var_lgle).powf(p.p278);
        let assign35370_e50237: f64 = (p.p277 / assign35370_e50236);
        let assign35370_e50238: f64 = (1.0 + assign35370_e50237);
        (assign35370_e50238,)
    } else {
        (var_rdrvmaxle,)
    }
};
        var_rdrvmaxle = assign35370_e50240;

        let (assign35380_e50250,) = {
    if (var_guard1153 != 0.0) {
        let assign35380_e50246: f64 = (var_wg).powf(p.p276);
        let assign35380_e50247: f64 = (p.p275 / assign35380_e50246);
        let assign35380_e50248: f64 = (1.0 + assign35380_e50247);
        (assign35380_e50248,)
    } else {
        (var_rdrvmaxwe,)
    }
};
        var_rdrvmaxwe = assign35380_e50250;

        let (assign35390_e50256, assign35390_e50256_d_n0, assign35390_e50256_d_n2, assign35390_e50256_d_n6, assign35390_e50256_d_n7, assign35390_e50256_d_n10, assign35390_e50256_d_n11, assign35390_e50256_d_n12, assign35390_e50256_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35390_e50254: f64 = (var_mu0 * var_rdrmuele);
        (assign35390_e50254, (var_mu0_dn0 * var_rdrmuele), (var_mu0_dn2 * var_rdrmuele), (var_mu0_dn6 * var_rdrmuele), (var_mu0_dn7 * var_rdrmuele), (var_mu0_dn10 * var_rdrmuele), (var_mu0_dn11 * var_rdrmuele), (var_mu0_dn12 * var_rdrmuele), (var_mu0_dn17 * var_rdrmuele),)
    } else {
        (var_mu0, var_mu0_dn0, var_mu0_dn2, var_mu0_dn6, var_mu0_dn7, var_mu0_dn10, var_mu0_dn11, var_mu0_dn12, var_mu0_dn17,)
    }
};
        var_mu0 = assign35390_e50256;
        var_mu0_dn0 = assign35390_e50256_d_n0;
        var_mu0_dn2 = assign35390_e50256_d_n2;
        var_mu0_dn6 = assign35390_e50256_d_n6;
        var_mu0_dn7 = assign35390_e50256_d_n7;
        var_mu0_dn10 = assign35390_e50256_d_n10;
        var_mu0_dn11 = assign35390_e50256_d_n11;
        var_mu0_dn12 = assign35390_e50256_d_n12;
        var_mu0_dn17 = assign35390_e50256_d_n17;

        let (assign35400_e50266, assign35400_e50266_d_n0, assign35400_e50266_d_n2, assign35400_e50266_d_n6, assign35400_e50266_d_n7, assign35400_e50266_d_n10, assign35400_e50266_d_n11, assign35400_e50266_d_n12, assign35400_e50266_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35400_e50260: f64 = (var_vmaxe__blk1166 * var_rdrvmaxwe);
        let assign35400_e50262: f64 = (assign35400_e50260 * var_rdrvmaxle);
        let assign35400_e50264: f64 = (assign35400_e50262 + 1e-50);
        (assign35400_e50264, ((var_vmaxe__blk1166_dn0 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1166_dn2 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1166_dn6 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1166_dn7 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1166_dn10 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1166_dn11 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1166_dn12 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk1166_dn17 * var_rdrvmaxwe) * var_rdrvmaxle),)
    } else {
        (var_vmaxe__blk1166, var_vmaxe__blk1166_dn0, var_vmaxe__blk1166_dn2, var_vmaxe__blk1166_dn6, var_vmaxe__blk1166_dn7, var_vmaxe__blk1166_dn10, var_vmaxe__blk1166_dn11, var_vmaxe__blk1166_dn12, var_vmaxe__blk1166_dn17,)
    }
};
        var_vmaxe__blk1166 = assign35400_e50266;
        var_vmaxe__blk1166_dn0 = assign35400_e50266_d_n0;
        var_vmaxe__blk1166_dn2 = assign35400_e50266_d_n2;
        var_vmaxe__blk1166_dn6 = assign35400_e50266_d_n6;
        var_vmaxe__blk1166_dn7 = assign35400_e50266_d_n7;
        var_vmaxe__blk1166_dn10 = assign35400_e50266_d_n10;
        var_vmaxe__blk1166_dn11 = assign35400_e50266_d_n11;
        var_vmaxe__blk1166_dn12 = assign35400_e50266_d_n12;
        var_vmaxe__blk1166_dn17 = assign35400_e50266_d_n17;

        let (assign35410_e50272, assign35410_e50272_d_n0, assign35410_e50272_d_n2, assign35410_e50272_d_n6, assign35410_e50272_d_n7,) = {
    if (var_guard1153 != 0.0) {
        let assign35410_e50270: f64 = (var_vrdr / var_ldrifte);
        (assign35410_e50270, (var_vrdr_dn0 / var_ldrifte), (var_vrdr_dn2 / var_ldrifte), (var_vrdr_dn6 / var_ldrifte), (var_vrdr_dn7 / var_ldrifte),)
    } else {
        (var_edri, var_edri_dn0, var_edri_dn2, var_edri_dn6, var_edri_dn7,)
    }
};
        var_edri = assign35410_e50272;
        var_edri_dn0 = assign35410_e50272_d_n0;
        var_edri_dn2 = assign35410_e50272_d_n2;
        var_edri_dn6 = assign35410_e50272_d_n6;
        var_edri_dn7 = assign35410_e50272_d_n7;

        let (assign35420_e50278, assign35420_e50278_d_n0, assign35420_e50278_d_n2, assign35420_e50278_d_n6, assign35420_e50278_d_n7, assign35420_e50278_d_n10, assign35420_e50278_d_n11, assign35420_e50278_d_n12, assign35420_e50278_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35420_e50276: f64 = (var_mu0 * var_edri);
        (assign35420_e50276, ((var_mu0_dn0 * var_edri) + (var_mu0 * var_edri_dn0)), ((var_mu0_dn2 * var_edri) + (var_mu0 * var_edri_dn2)), ((var_mu0_dn6 * var_edri) + (var_mu0 * var_edri_dn6)), ((var_mu0_dn7 * var_edri) + (var_mu0 * var_edri_dn7)), (var_mu0_dn10 * var_edri), (var_mu0_dn11 * var_edri), (var_mu0_dn12 * var_edri), (var_mu0_dn17 * var_edri),)
    } else {
        (var_vdri, var_vdri_dn0, var_vdri_dn2, var_vdri_dn6, var_vdri_dn7, var_vdri_dn10, var_vdri_dn11, var_vdri_dn12, var_vdri_dn17,)
    }
};
        var_vdri = assign35420_e50278;
        var_vdri_dn0 = assign35420_e50278_d_n0;
        var_vdri_dn2 = assign35420_e50278_d_n2;
        var_vdri_dn6 = assign35420_e50278_d_n6;
        var_vdri_dn7 = assign35420_e50278_d_n7;
        var_vdri_dn10 = assign35420_e50278_d_n10;
        var_vdri_dn11 = assign35420_e50278_d_n11;
        var_vdri_dn12 = assign35420_e50278_d_n12;
        var_vdri_dn17 = assign35420_e50278_d_n17;

        let assign35430_e50281: f64 = if var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        var_guard1174 = assign35430_e50281;

        let (assign35440_e50289, assign35440_e50289_d_n0, assign35440_e50289_d_n2, assign35440_e50289_d_n6, assign35440_e50289_d_n7, assign35440_e50289_d_n10, assign35440_e50289_d_n11, assign35440_e50289_d_n12, assign35440_e50289_d_n17,) = {
    if ((var_guard1153 != 0.0) && (var_guard1174 != 0.0)) {
        let assign35440_e50287: f64 = (var_vdri / var_vmaxe__blk1166);
        (assign35440_e50287, (((var_vdri_dn0 * var_vmaxe__blk1166) - (var_vdri * var_vmaxe__blk1166_dn0)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), (((var_vdri_dn2 * var_vmaxe__blk1166) - (var_vdri * var_vmaxe__blk1166_dn2)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), (((var_vdri_dn6 * var_vmaxe__blk1166) - (var_vdri * var_vmaxe__blk1166_dn6)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), (((var_vdri_dn7 * var_vmaxe__blk1166) - (var_vdri * var_vmaxe__blk1166_dn7)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), (((var_vdri_dn10 * var_vmaxe__blk1166) - (var_vdri * var_vmaxe__blk1166_dn10)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), (((var_vdri_dn11 * var_vmaxe__blk1166) - (var_vdri * var_vmaxe__blk1166_dn11)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), (((var_vdri_dn12 * var_vmaxe__blk1166) - (var_vdri * var_vmaxe__blk1166_dn12)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), (((var_vdri_dn17 * var_vmaxe__blk1166) - (var_vdri * var_vmaxe__blk1166_dn17)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35440_e50289;
        var_t1_dn0 = assign35440_e50289_d_n0;
        var_t1_dn2 = assign35440_e50289_d_n2;
        var_t1_dn6 = assign35440_e50289_d_n6;
        var_t1_dn7 = assign35440_e50289_d_n7;
        var_t1_dn10 = assign35440_e50289_d_n10;
        var_t1_dn11 = assign35440_e50289_d_n11;
        var_t1_dn12 = assign35440_e50289_d_n12;
        var_t1_dn17 = assign35440_e50289_d_n17;

        let (assign35450_e50299, assign35450_e50299_d_n0, assign35450_e50299_d_n2, assign35450_e50299_d_n6, assign35450_e50299_d_n7, assign35450_e50299_d_n10, assign35450_e50299_d_n11, assign35450_e50299_d_n12, assign35450_e50299_d_n17,) = {
    if ((var_guard1153 != 0.0) && (var_guard1174 == 0.0)) {
        let assign35450_e50295: f64 = (-var_vdri);
        let assign35450_e50297: f64 = (assign35450_e50295 / var_vmaxe__blk1166);
        (assign35450_e50297, ((((-var_vdri_dn0) * var_vmaxe__blk1166) - (assign35450_e50295 * var_vmaxe__blk1166_dn0)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), ((((-var_vdri_dn2) * var_vmaxe__blk1166) - (assign35450_e50295 * var_vmaxe__blk1166_dn2)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), ((((-var_vdri_dn6) * var_vmaxe__blk1166) - (assign35450_e50295 * var_vmaxe__blk1166_dn6)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), ((((-var_vdri_dn7) * var_vmaxe__blk1166) - (assign35450_e50295 * var_vmaxe__blk1166_dn7)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), ((((-var_vdri_dn10) * var_vmaxe__blk1166) - (assign35450_e50295 * var_vmaxe__blk1166_dn10)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), ((((-var_vdri_dn11) * var_vmaxe__blk1166) - (assign35450_e50295 * var_vmaxe__blk1166_dn11)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), ((((-var_vdri_dn12) * var_vmaxe__blk1166) - (assign35450_e50295 * var_vmaxe__blk1166_dn12)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)), ((((-var_vdri_dn17) * var_vmaxe__blk1166) - (assign35450_e50295 * var_vmaxe__blk1166_dn17)) / (var_vmaxe__blk1166 * var_vmaxe__blk1166)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35450_e50299;
        var_t1_dn0 = assign35450_e50299_d_n0;
        var_t1_dn2 = assign35450_e50299_d_n2;
        var_t1_dn6 = assign35450_e50299_d_n6;
        var_t1_dn7 = assign35450_e50299_d_n7;
        var_t1_dn10 = assign35450_e50299_d_n10;
        var_t1_dn11 = assign35450_e50299_d_n11;
        var_t1_dn12 = assign35450_e50299_d_n12;
        var_t1_dn17 = assign35450_e50299_d_n17;

        let assign35460_e50303: f64 = (10.0 * 2.220446049250313e-16);
        let assign35460_e50304: f64 = (1.0 - assign35460_e50303);
        let assign35460_e50311: f64 = (10.0 * 2.220446049250313e-16);
        let assign35460_e50312: f64 = (1.0 + assign35460_e50311);
        let assign35460_e50314: f64 = if ((assign35460_e50304 <= var_rrdrbb) && (var_rrdrbb <= assign35460_e50312)) { 1.0 } else { 0.0 };
        var_guard1175 = assign35460_e50314;

        let (assign35470_e50320, assign35470_e50320_d_n0, assign35470_e50320_d_n2, assign35470_e50320_d_n6, assign35470_e50320_d_n7, assign35470_e50320_d_n10, assign35470_e50320_d_n11, assign35470_e50320_d_n12, assign35470_e50320_d_n17,) = {
    if ((var_guard1153 != 0.0) && (var_guard1175 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign35470_e50320;
        var_t3_dn0 = assign35470_e50320_d_n0;
        var_t3_dn2 = assign35470_e50320_d_n2;
        var_t3_dn6 = assign35470_e50320_d_n6;
        var_t3_dn7 = assign35470_e50320_d_n7;
        var_t3_dn10 = assign35470_e50320_d_n10;
        var_t3_dn11 = assign35470_e50320_d_n11;
        var_t3_dn12 = assign35470_e50320_d_n12;
        var_t3_dn17 = assign35470_e50320_d_n17;

        let assign35480_e50324: f64 = (10.0 * 2.220446049250313e-16);
        let assign35480_e50325: f64 = (2.0 - assign35480_e50324);
        let assign35480_e50332: f64 = (10.0 * 2.220446049250313e-16);
        let assign35480_e50333: f64 = (2.0 + assign35480_e50332);
        let assign35480_e50335: f64 = if ((assign35480_e50325 <= var_rrdrbb) && (var_rrdrbb <= assign35480_e50333)) { 1.0 } else { 0.0 };
        var_guard1176 = assign35480_e50335;

        let (assign35490_e50344, assign35490_e50344_d_n0, assign35490_e50344_d_n2, assign35490_e50344_d_n6, assign35490_e50344_d_n7, assign35490_e50344_d_n10, assign35490_e50344_d_n11, assign35490_e50344_d_n12, assign35490_e50344_d_n17,) = {
    if (((var_guard1153 != 0.0) && (var_guard1175 == 0.0)) && (var_guard1176 != 0.0)) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign35490_e50344;
        var_t3_dn0 = assign35490_e50344_d_n0;
        var_t3_dn2 = assign35490_e50344_d_n2;
        var_t3_dn6 = assign35490_e50344_d_n6;
        var_t3_dn7 = assign35490_e50344_d_n7;
        var_t3_dn10 = assign35490_e50344_d_n10;
        var_t3_dn11 = assign35490_e50344_d_n11;
        var_t3_dn12 = assign35490_e50344_d_n12;
        var_t3_dn17 = assign35490_e50344_d_n17;

        let (assign35500_e50358, assign35500_e50358_d_n0, assign35500_e50358_d_n2, assign35500_e50358_d_n6, assign35500_e50358_d_n7, assign35500_e50358_d_n10, assign35500_e50358_d_n11, assign35500_e50358_d_n12, assign35500_e50358_d_n17,) = {
    if (((var_guard1153 != 0.0) && (var_guard1175 == 0.0)) && (var_guard1176 == 0.0)) {
        let assign35500_e50355: f64 = (var_rrdrbb - 1.0);
        let assign35500_e50356: f64 = (var_t1).powf(assign35500_e50355);
        (assign35500_e50356, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((var_t1).powf(assign35500_e50355 - 1.0) * var_t1_dn0)) } } else { (assign35500_e50356 * (assign35500_e50355 * (var_t1_dn0 / var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((var_t1).powf(assign35500_e50355 - 1.0) * var_t1_dn2)) } } else { (assign35500_e50356 * (assign35500_e50355 * (var_t1_dn2 / var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((var_t1).powf(assign35500_e50355 - 1.0) * var_t1_dn6)) } } else { (assign35500_e50356 * (assign35500_e50355 * (var_t1_dn6 / var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((var_t1).powf(assign35500_e50355 - 1.0) * var_t1_dn7)) } } else { (assign35500_e50356 * (assign35500_e50355 * (var_t1_dn7 / var_t1))) }, if var_rrdrbb_dn10 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((var_t1).powf(assign35500_e50355 - 1.0) * var_t1_dn10)) } } else { (assign35500_e50356 * ((var_rrdrbb_dn10 * (var_t1).ln()) + (assign35500_e50355 * (var_t1_dn10 / var_t1)))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((var_t1).powf(assign35500_e50355 - 1.0) * var_t1_dn11)) } } else { (assign35500_e50356 * (assign35500_e50355 * (var_t1_dn11 / var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((var_t1).powf(assign35500_e50355 - 1.0) * var_t1_dn12)) } } else { (assign35500_e50356 * (assign35500_e50355 * (var_t1_dn12 / var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((var_t1).powf(assign35500_e50355 - 1.0) * var_t1_dn17)) } } else { (assign35500_e50356 * (assign35500_e50355 * (var_t1_dn17 / var_t1))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign35500_e50358;
        var_t3_dn0 = assign35500_e50358_d_n0;
        var_t3_dn2 = assign35500_e50358_d_n2;
        var_t3_dn6 = assign35500_e50358_d_n6;
        var_t3_dn7 = assign35500_e50358_d_n7;
        var_t3_dn10 = assign35500_e50358_d_n10;
        var_t3_dn11 = assign35500_e50358_d_n11;
        var_t3_dn12 = assign35500_e50358_d_n12;
        var_t3_dn17 = assign35500_e50358_d_n17;

        let (assign35510_e50364, assign35510_e50364_d_n0, assign35510_e50364_d_n2, assign35510_e50364_d_n6, assign35510_e50364_d_n7, assign35510_e50364_d_n10, assign35510_e50364_d_n11, assign35510_e50364_d_n12, assign35510_e50364_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35510_e50362: f64 = (var_t1 * var_t3);
        (assign35510_e50362, ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)), ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)), ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)), ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)), ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)), ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)), ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)), ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign35510_e50364;
        var_t2_dn0 = assign35510_e50364_d_n0;
        var_t2_dn2 = assign35510_e50364_d_n2;
        var_t2_dn6 = assign35510_e50364_d_n6;
        var_t2_dn7 = assign35510_e50364_d_n7;
        var_t2_dn10 = assign35510_e50364_d_n10;
        var_t2_dn11 = assign35510_e50364_d_n11;
        var_t2_dn12 = assign35510_e50364_d_n12;
        var_t2_dn17 = assign35510_e50364_d_n17;

        let (assign35520_e50370, assign35520_e50370_d_n0, assign35520_e50370_d_n2, assign35520_e50370_d_n6, assign35520_e50370_d_n7, assign35520_e50370_d_n10, assign35520_e50370_d_n11, assign35520_e50370_d_n12, assign35520_e50370_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35520_e50368: f64 = (1.0 + var_t2);
        (assign35520_e50368, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign35520_e50370;
        var_t4_dn0 = assign35520_e50370_d_n0;
        var_t4_dn2 = assign35520_e50370_d_n2;
        var_t4_dn6 = assign35520_e50370_d_n6;
        var_t4_dn7 = assign35520_e50370_d_n7;
        var_t4_dn10 = assign35520_e50370_d_n10;
        var_t4_dn11 = assign35520_e50370_d_n11;
        var_t4_dn12 = assign35520_e50370_d_n12;
        var_t4_dn17 = assign35520_e50370_d_n17;

        let assign35530_e50374: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50375: f64 = (1.0 - assign35530_e50374);
        let assign35530_e50382: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50383: f64 = (1.0 + assign35530_e50382);
        let assign35530_e50385: f64 = if ((assign35530_e50375 <= var_rrdrbb) && (var_rrdrbb <= assign35530_e50383)) { 1.0 } else { 0.0 };
        var_guard1177 = assign35530_e50385;

        let (assign35540_e50393, assign35540_e50393_d_n0, assign35540_e50393_d_n2, assign35540_e50393_d_n6, assign35540_e50393_d_n7, assign35540_e50393_d_n10, assign35540_e50393_d_n11, assign35540_e50393_d_n12, assign35540_e50393_d_n17,) = {
    if ((var_guard1153 != 0.0) && (var_guard1177 != 0.0)) {
        let assign35540_e50391: f64 = (1.0 / var_t4);
        (assign35540_e50391, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn11 / (var_t4 * var_t4))), (-(var_t4_dn12 / (var_t4 * var_t4))), (-(var_t4_dn17 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign35540_e50393;
        var_t5_dn0 = assign35540_e50393_d_n0;
        var_t5_dn2 = assign35540_e50393_d_n2;
        var_t5_dn6 = assign35540_e50393_d_n6;
        var_t5_dn7 = assign35540_e50393_d_n7;
        var_t5_dn10 = assign35540_e50393_d_n10;
        var_t5_dn11 = assign35540_e50393_d_n11;
        var_t5_dn12 = assign35540_e50393_d_n12;
        var_t5_dn17 = assign35540_e50393_d_n17;

        let assign35550_e50397: f64 = (10.0 * 2.220446049250313e-16);
        let assign35550_e50398: f64 = (2.0 - assign35550_e50397);
        let assign35550_e50405: f64 = (10.0 * 2.220446049250313e-16);
        let assign35550_e50406: f64 = (2.0 + assign35550_e50405);
        let assign35550_e50408: f64 = if ((assign35550_e50398 <= var_rrdrbb) && (var_rrdrbb <= assign35550_e50406)) { 1.0 } else { 0.0 };
        var_guard1178 = assign35550_e50408;

        let (assign35560_e50420, assign35560_e50420_d_n0, assign35560_e50420_d_n2, assign35560_e50420_d_n6, assign35560_e50420_d_n7, assign35560_e50420_d_n10, assign35560_e50420_d_n11, assign35560_e50420_d_n12, assign35560_e50420_d_n17,) = {
    if (((var_guard1153 != 0.0) && (var_guard1177 == 0.0)) && (var_guard1178 != 0.0)) {
        let assign35560_e50417: f64 = (var_t4).sqrt();
        let assign35560_e50418: f64 = (1.0 / assign35560_e50417);
        (assign35560_e50418, (-((var_t4_dn0 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn2 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn6 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn7 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn10 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn11 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn12 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn17 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign35560_e50420;
        var_t5_dn0 = assign35560_e50420_d_n0;
        var_t5_dn2 = assign35560_e50420_d_n2;
        var_t5_dn6 = assign35560_e50420_d_n6;
        var_t5_dn7 = assign35560_e50420_d_n7;
        var_t5_dn10 = assign35560_e50420_d_n10;
        var_t5_dn11 = assign35560_e50420_d_n11;
        var_t5_dn12 = assign35560_e50420_d_n12;
        var_t5_dn17 = assign35560_e50420_d_n17;

        let (assign35570_e50437, assign35570_e50437_d_n0, assign35570_e50437_d_n2, assign35570_e50437_d_n6, assign35570_e50437_d_n7, assign35570_e50437_d_n10, assign35570_e50437_d_n11, assign35570_e50437_d_n12, assign35570_e50437_d_n17,) = {
    if (((var_guard1153 != 0.0) && (var_guard1177 == 0.0)) && (var_guard1178 == 0.0)) {
        let assign35570_e50430: f64 = (-1.0);
        let assign35570_e50432: f64 = (assign35570_e50430 / var_rrdrbb);
        let assign35570_e50434: f64 = (assign35570_e50432 - 1.0);
        let assign35570_e50435: f64 = (var_t4).powf(assign35570_e50434);
        (assign35570_e50435, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn0)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn0 / var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn2)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn2 / var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn6)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn6 / var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn7)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn7 / var_t4))) }, if (-((assign35570_e50430 * var_rrdrbb_dn10) / (var_rrdrbb * var_rrdrbb))) == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn10)) } } else { (assign35570_e50435 * (((-((assign35570_e50430 * var_rrdrbb_dn10) / (var_rrdrbb * var_rrdrbb))) * (var_t4).ln()) + (assign35570_e50434 * (var_t4_dn10 / var_t4)))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn11)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn11 / var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn12)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn12 / var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn17)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn17 / var_t4))) },)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn6, var_t6_dn7, var_t6_dn10, var_t6_dn11, var_t6_dn12, var_t6_dn17,)
    }
};
        var_t6 = assign35570_e50437;
        var_t6_dn0 = assign35570_e50437_d_n0;
        var_t6_dn2 = assign35570_e50437_d_n2;
        var_t6_dn6 = assign35570_e50437_d_n6;
        var_t6_dn7 = assign35570_e50437_d_n7;
        var_t6_dn10 = assign35570_e50437_d_n10;
        var_t6_dn11 = assign35570_e50437_d_n11;
        var_t6_dn12 = assign35570_e50437_d_n12;
        var_t6_dn17 = assign35570_e50437_d_n17;

        let (assign35580_e50449, assign35580_e50449_d_n0, assign35580_e50449_d_n2, assign35580_e50449_d_n6, assign35580_e50449_d_n7, assign35580_e50449_d_n10, assign35580_e50449_d_n11, assign35580_e50449_d_n12, assign35580_e50449_d_n17,) = {
    if (((var_guard1153 != 0.0) && (var_guard1177 == 0.0)) && (var_guard1178 == 0.0)) {
        let assign35580_e50447: f64 = (var_t4 * var_t6);
        (assign35580_e50447, ((var_t4_dn0 * var_t6) + (var_t4 * var_t6_dn0)), ((var_t4_dn2 * var_t6) + (var_t4 * var_t6_dn2)), ((var_t4_dn6 * var_t6) + (var_t4 * var_t6_dn6)), ((var_t4_dn7 * var_t6) + (var_t4 * var_t6_dn7)), ((var_t4_dn10 * var_t6) + (var_t4 * var_t6_dn10)), ((var_t4_dn11 * var_t6) + (var_t4 * var_t6_dn11)), ((var_t4_dn12 * var_t6) + (var_t4 * var_t6_dn12)), ((var_t4_dn17 * var_t6) + (var_t4 * var_t6_dn17)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign35580_e50449;
        var_t5_dn0 = assign35580_e50449_d_n0;
        var_t5_dn2 = assign35580_e50449_d_n2;
        var_t5_dn6 = assign35580_e50449_d_n6;
        var_t5_dn7 = assign35580_e50449_d_n7;
        var_t5_dn10 = assign35580_e50449_d_n10;
        var_t5_dn11 = assign35580_e50449_d_n11;
        var_t5_dn12 = assign35580_e50449_d_n12;
        var_t5_dn17 = assign35580_e50449_d_n17;

        let (assign35590_e50455, assign35590_e50455_d_n0, assign35590_e50455_d_n2, assign35590_e50455_d_n6, assign35590_e50455_d_n7, assign35590_e50455_d_n10, assign35590_e50455_d_n11, assign35590_e50455_d_n12, assign35590_e50455_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35590_e50453: f64 = (var_mu0 * var_t5);
        (assign35590_e50453, ((var_mu0_dn0 * var_t5) + (var_mu0 * var_t5_dn0)), ((var_mu0_dn2 * var_t5) + (var_mu0 * var_t5_dn2)), ((var_mu0_dn6 * var_t5) + (var_mu0 * var_t5_dn6)), ((var_mu0_dn7 * var_t5) + (var_mu0 * var_t5_dn7)), ((var_mu0_dn10 * var_t5) + (var_mu0 * var_t5_dn10)), ((var_mu0_dn11 * var_t5) + (var_mu0 * var_t5_dn11)), ((var_mu0_dn12 * var_t5) + (var_mu0 * var_t5_dn12)), ((var_mu0_dn17 * var_t5) + (var_mu0 * var_t5_dn17)),)
    } else {
        (var_mu__blk1169, var_mu__blk1169_dn0, var_mu__blk1169_dn2, var_mu__blk1169_dn6, var_mu__blk1169_dn7, var_mu__blk1169_dn10, var_mu__blk1169_dn11, var_mu__blk1169_dn12, var_mu__blk1169_dn17,)
    }
};
        var_mu__blk1169 = assign35590_e50455;
        var_mu__blk1169_dn0 = assign35590_e50455_d_n0;
        var_mu__blk1169_dn2 = assign35590_e50455_d_n2;
        var_mu__blk1169_dn6 = assign35590_e50455_d_n6;
        var_mu__blk1169_dn7 = assign35590_e50455_d_n7;
        var_mu__blk1169_dn10 = assign35590_e50455_d_n10;
        var_mu__blk1169_dn11 = assign35590_e50455_d_n11;
        var_mu__blk1169_dn12 = assign35590_e50455_d_n12;
        var_mu__blk1169_dn17 = assign35590_e50455_d_n17;

        let (assign35600_e50461, assign35600_e50461_d_n0, assign35600_e50461_d_n2, assign35600_e50461_d_n6, assign35600_e50461_d_n7, assign35600_e50461_d_n10, assign35600_e50461_d_n11, assign35600_e50461_d_n12, assign35600_e50461_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35600_e50459: f64 = (1.6021918e-19 / var_ldrifte);
        (assign35600_e50459, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35600_e50461;
        var_t1_dn0 = assign35600_e50461_d_n0;
        var_t1_dn2 = assign35600_e50461_d_n2;
        var_t1_dn6 = assign35600_e50461_d_n6;
        var_t1_dn7 = assign35600_e50461_d_n7;
        var_t1_dn10 = assign35600_e50461_d_n10;
        var_t1_dn11 = assign35600_e50461_d_n11;
        var_t1_dn12 = assign35600_e50461_d_n12;
        var_t1_dn17 = assign35600_e50461_d_n17;

        let (assign35610_e50471, assign35610_e50471_d_n0, assign35610_e50471_d_n2, assign35610_e50471_d_n6, assign35610_e50471_d_n7, assign35610_e50471_d_n10, assign35610_e50471_d_n11, assign35610_e50471_d_n12, assign35610_e50471_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35610_e50465: f64 = (var_t1 * var_xov);
        let assign35610_e50467: f64 = (assign35610_e50465 * var_mu__blk1169);
        let assign35610_e50469: f64 = (assign35610_e50467 * var_nover);
        (assign35610_e50469, ((((var_t1_dn0 * var_xov) * var_mu__blk1169) + (assign35610_e50465 * var_mu__blk1169_dn0)) * var_nover), ((((var_t1_dn2 * var_xov) * var_mu__blk1169) + (assign35610_e50465 * var_mu__blk1169_dn2)) * var_nover), ((((var_t1_dn6 * var_xov) * var_mu__blk1169) + (assign35610_e50465 * var_mu__blk1169_dn6)) * var_nover), ((((var_t1_dn7 * var_xov) * var_mu__blk1169) + (assign35610_e50465 * var_mu__blk1169_dn7)) * var_nover), ((((var_t1_dn10 * var_xov) * var_mu__blk1169) + (assign35610_e50465 * var_mu__blk1169_dn10)) * var_nover), ((((var_t1_dn11 * var_xov) * var_mu__blk1169) + (assign35610_e50465 * var_mu__blk1169_dn11)) * var_nover), ((((var_t1_dn12 * var_xov) * var_mu__blk1169) + (assign35610_e50465 * var_mu__blk1169_dn12)) * var_nover), ((((var_t1_dn17 * var_xov) * var_mu__blk1169) + (assign35610_e50465 * var_mu__blk1169_dn17)) * var_nover),)
    } else {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn6, var_gd_dn7, var_gd_dn10, var_gd_dn11, var_gd_dn12, var_gd_dn17,)
    }
};
        var_gd = assign35610_e50471;
        var_gd_dn0 = assign35610_e50471_d_n0;
        var_gd_dn2 = assign35610_e50471_d_n2;
        var_gd_dn6 = assign35610_e50471_d_n6;
        var_gd_dn7 = assign35610_e50471_d_n7;
        var_gd_dn10 = assign35610_e50471_d_n10;
        var_gd_dn11 = assign35610_e50471_d_n11;
        var_gd_dn12 = assign35610_e50471_d_n12;
        var_gd_dn17 = assign35610_e50471_d_n17;

        let assign35620_e50474: f64 = if var_gd <= 0.0 { 1.0 } else { 0.0 };
        var_guard1179 = assign35620_e50474;

        let (assign35630_e50480, assign35630_e50480_d_n0, assign35630_e50480_d_n2, assign35630_e50480_d_n6, assign35630_e50480_d_n7, assign35630_e50480_d_n10, assign35630_e50480_d_n11, assign35630_e50480_d_n12, assign35630_e50480_d_n17,) = {
    if ((var_guard1153 != 0.0) && (var_guard1179 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn6, var_gd_dn7, var_gd_dn10, var_gd_dn11, var_gd_dn12, var_gd_dn17,)
    }
};
        var_gd = assign35630_e50480;
        var_gd_dn0 = assign35630_e50480_d_n0;
        var_gd_dn2 = assign35630_e50480_d_n2;
        var_gd_dn6 = assign35630_e50480_d_n6;
        var_gd_dn7 = assign35630_e50480_d_n7;
        var_gd_dn10 = assign35630_e50480_d_n10;
        var_gd_dn11 = assign35630_e50480_d_n11;
        var_gd_dn12 = assign35630_e50480_d_n12;
        var_gd_dn17 = assign35630_e50480_d_n17;

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
        *var_guard1174_slot = var_guard1174;
        *var_guard1175_slot = var_guard1175;
        *var_guard1176_slot = var_guard1176;
        *var_guard1177_slot = var_guard1177;
        *var_guard1178_slot = var_guard1178;
        *var_guard1179_slot = var_guard1179;
        *var_mu0_slot = var_mu0;
        *var_mu0_dn0_slot = var_mu0_dn0;
        *var_mu0_dn10_slot = var_mu0_dn10;
        *var_mu0_dn11_slot = var_mu0_dn11;
        *var_mu0_dn12_slot = var_mu0_dn12;
        *var_mu0_dn17_slot = var_mu0_dn17;
        *var_mu0_dn2_slot = var_mu0_dn2;
        *var_mu0_dn6_slot = var_mu0_dn6;
        *var_mu0_dn7_slot = var_mu0_dn7;
        *var_mu__blk1169_slot = var_mu__blk1169;
        *var_mu__blk1169_dn0_slot = var_mu__blk1169_dn0;
        *var_mu__blk1169_dn10_slot = var_mu__blk1169_dn10;
        *var_mu__blk1169_dn11_slot = var_mu__blk1169_dn11;
        *var_mu__blk1169_dn12_slot = var_mu__blk1169_dn12;
        *var_mu__blk1169_dn17_slot = var_mu__blk1169_dn17;
        *var_mu__blk1169_dn2_slot = var_mu__blk1169_dn2;
        *var_mu__blk1169_dn6_slot = var_mu__blk1169_dn6;
        *var_mu__blk1169_dn7_slot = var_mu__blk1169_dn7;
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
        *var_vmaxe__blk1166_slot = var_vmaxe__blk1166;
        *var_vmaxe__blk1166_dn0_slot = var_vmaxe__blk1166_dn0;
        *var_vmaxe__blk1166_dn10_slot = var_vmaxe__blk1166_dn10;
        *var_vmaxe__blk1166_dn11_slot = var_vmaxe__blk1166_dn11;
        *var_vmaxe__blk1166_dn12_slot = var_vmaxe__blk1166_dn12;
        *var_vmaxe__blk1166_dn17_slot = var_vmaxe__blk1166_dn17;
        *var_vmaxe__blk1166_dn2_slot = var_vmaxe__blk1166_dn2;
        *var_vmaxe__blk1166_dn6_slot = var_vmaxe__blk1166_dn6;
        *var_vmaxe__blk1166_dn7_slot = var_vmaxe__blk1166_dn7;
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
        var_guard1153: f64,
        var_lgle: f64,
        var_mfactor: f64,
        var_rsd0: f64,
        var_ttemp: f64,
        var_ttemp_dn10: f64,
        var_uc_tnom: f64,
        var_weff: f64,
        var_weff_nf_1: f64,
        var_wg: f64,
        var_edri__blk1195_slot: &mut f64,
        var_edri__blk1195_dn0_slot: &mut f64,
        var_edri__blk1195_dn2_slot: &mut f64,
        var_edri__blk1195_dn6_slot: &mut f64,
        var_edri__blk1195_dn7_slot: &mut f64,
        var_guard1180_slot: &mut f64,
        var_guard1181_slot: &mut f64,
        var_guard1201_slot: &mut f64,
        var_ldrifte__blk1191_slot: &mut f64,
        var_mks_rdrmue__blk1185_slot: &mut f64,
        var_mks_rdrvmax__blk1186_slot: &mut f64,
        var_mu0__blk1193_slot: &mut f64,
        var_mu0__blk1193_dn0_slot: &mut f64,
        var_mu0__blk1193_dn10_slot: &mut f64,
        var_mu0__blk1193_dn11_slot: &mut f64,
        var_mu0__blk1193_dn12_slot: &mut f64,
        var_mu0__blk1193_dn17_slot: &mut f64,
        var_mu0__blk1193_dn2_slot: &mut f64,
        var_mu0__blk1193_dn6_slot: &mut f64,
        var_mu0__blk1193_dn7_slot: &mut f64,
        var_nover__blk1192_slot: &mut f64,
        var_rdmod_slot: &mut f64,
        var_rdrmuele__blk1182_slot: &mut f64,
        var_rdrvmaxle__blk1184_slot: &mut f64,
        var_rdrvmaxwe__blk1183_slot: &mut f64,
        var_rrdrbb__blk1187_slot: &mut f64,
        var_rrdrbb__blk1187_dn10_slot: &mut f64,
        var_rsd_slot: &mut f64,
        var_rsd0__blk1188_slot: &mut f64,
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
        var_tratio__blk1190_slot: &mut f64,
        var_tratio__blk1190_dn10_slot: &mut f64,
        var_vmaxe__blk1194_slot: &mut f64,
        var_vmaxe__blk1194_dn0_slot: &mut f64,
        var_vmaxe__blk1194_dn10_slot: &mut f64,
        var_vmaxe__blk1194_dn11_slot: &mut f64,
        var_vmaxe__blk1194_dn12_slot: &mut f64,
        var_vmaxe__blk1194_dn17_slot: &mut f64,
        var_vmaxe__blk1194_dn2_slot: &mut f64,
        var_vmaxe__blk1194_dn6_slot: &mut f64,
        var_vmaxe__blk1194_dn7_slot: &mut f64,
        var_vrdr__blk1189_slot: &mut f64,
        var_vrdr__blk1189_dn0_slot: &mut f64,
        var_vrdr__blk1189_dn2_slot: &mut f64,
        var_vrdr__blk1189_dn6_slot: &mut f64,
        var_vrdr__blk1189_dn7_slot: &mut f64,
        var_weff_nf__blk1200_slot: &mut f64,
        var_xov__blk1198_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let mut var_edri__blk1195: f64 = *var_edri__blk1195_slot;
        let mut var_edri__blk1195_dn0: f64 = *var_edri__blk1195_dn0_slot;
        let mut var_edri__blk1195_dn2: f64 = *var_edri__blk1195_dn2_slot;
        let mut var_edri__blk1195_dn6: f64 = *var_edri__blk1195_dn6_slot;
        let mut var_edri__blk1195_dn7: f64 = *var_edri__blk1195_dn7_slot;
        let mut var_guard1180: f64 = *var_guard1180_slot;
        let mut var_guard1181: f64 = *var_guard1181_slot;
        let mut var_guard1201: f64 = *var_guard1201_slot;
        let mut var_ldrifte__blk1191: f64 = *var_ldrifte__blk1191_slot;
        let mut var_mks_rdrmue__blk1185: f64 = *var_mks_rdrmue__blk1185_slot;
        let mut var_mks_rdrvmax__blk1186: f64 = *var_mks_rdrvmax__blk1186_slot;
        let mut var_mu0__blk1193: f64 = *var_mu0__blk1193_slot;
        let mut var_mu0__blk1193_dn0: f64 = *var_mu0__blk1193_dn0_slot;
        let mut var_mu0__blk1193_dn10: f64 = *var_mu0__blk1193_dn10_slot;
        let mut var_mu0__blk1193_dn11: f64 = *var_mu0__blk1193_dn11_slot;
        let mut var_mu0__blk1193_dn12: f64 = *var_mu0__blk1193_dn12_slot;
        let mut var_mu0__blk1193_dn17: f64 = *var_mu0__blk1193_dn17_slot;
        let mut var_mu0__blk1193_dn2: f64 = *var_mu0__blk1193_dn2_slot;
        let mut var_mu0__blk1193_dn6: f64 = *var_mu0__blk1193_dn6_slot;
        let mut var_mu0__blk1193_dn7: f64 = *var_mu0__blk1193_dn7_slot;
        let mut var_nover__blk1192: f64 = *var_nover__blk1192_slot;
        let mut var_rdmod: f64 = *var_rdmod_slot;
        let mut var_rdrmuele__blk1182: f64 = *var_rdrmuele__blk1182_slot;
        let mut var_rdrvmaxle__blk1184: f64 = *var_rdrvmaxle__blk1184_slot;
        let mut var_rdrvmaxwe__blk1183: f64 = *var_rdrvmaxwe__blk1183_slot;
        let mut var_rrdrbb__blk1187: f64 = *var_rrdrbb__blk1187_slot;
        let mut var_rrdrbb__blk1187_dn10: f64 = *var_rrdrbb__blk1187_dn10_slot;
        let mut var_rsd: f64 = *var_rsd_slot;
        let mut var_rsd0__blk1188: f64 = *var_rsd0__blk1188_slot;
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
        let mut var_tratio__blk1190: f64 = *var_tratio__blk1190_slot;
        let mut var_tratio__blk1190_dn10: f64 = *var_tratio__blk1190_dn10_slot;
        let mut var_vmaxe__blk1194: f64 = *var_vmaxe__blk1194_slot;
        let mut var_vmaxe__blk1194_dn0: f64 = *var_vmaxe__blk1194_dn0_slot;
        let mut var_vmaxe__blk1194_dn10: f64 = *var_vmaxe__blk1194_dn10_slot;
        let mut var_vmaxe__blk1194_dn11: f64 = *var_vmaxe__blk1194_dn11_slot;
        let mut var_vmaxe__blk1194_dn12: f64 = *var_vmaxe__blk1194_dn12_slot;
        let mut var_vmaxe__blk1194_dn17: f64 = *var_vmaxe__blk1194_dn17_slot;
        let mut var_vmaxe__blk1194_dn2: f64 = *var_vmaxe__blk1194_dn2_slot;
        let mut var_vmaxe__blk1194_dn6: f64 = *var_vmaxe__blk1194_dn6_slot;
        let mut var_vmaxe__blk1194_dn7: f64 = *var_vmaxe__blk1194_dn7_slot;
        let mut var_vrdr__blk1189: f64 = *var_vrdr__blk1189_slot;
        let mut var_vrdr__blk1189_dn0: f64 = *var_vrdr__blk1189_dn0_slot;
        let mut var_vrdr__blk1189_dn2: f64 = *var_vrdr__blk1189_dn2_slot;
        let mut var_vrdr__blk1189_dn6: f64 = *var_vrdr__blk1189_dn6_slot;
        let mut var_vrdr__blk1189_dn7: f64 = *var_vrdr__blk1189_dn7_slot;
        let mut var_weff_nf__blk1200: f64 = *var_weff_nf__blk1200_slot;
        let mut var_xov__blk1198: f64 = *var_xov__blk1198_slot;

        let (assign35640_e50486, assign35640_e50486_d_n0, assign35640_e50486_d_n2, assign35640_e50486_d_n6, assign35640_e50486_d_n7, assign35640_e50486_d_n10, assign35640_e50486_d_n11, assign35640_e50486_d_n12, assign35640_e50486_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35640_e50484: f64 = (1.0 / var_gd);
        (assign35640_e50484, (-(var_gd_dn0 / (var_gd * var_gd))), (-(var_gd_dn2 / (var_gd * var_gd))), (-(var_gd_dn6 / (var_gd * var_gd))), (-(var_gd_dn7 / (var_gd * var_gd))), (-(var_gd_dn10 / (var_gd * var_gd))), (-(var_gd_dn11 / (var_gd * var_gd))), (-(var_gd_dn12 / (var_gd * var_gd))), (-(var_gd_dn17 / (var_gd * var_gd))),)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign35640_e50486;
        var_rsd_dn0 = assign35640_e50486_d_n0;
        var_rsd_dn2 = assign35640_e50486_d_n2;
        var_rsd_dn6 = assign35640_e50486_d_n6;
        var_rsd_dn7 = assign35640_e50486_d_n7;
        var_rsd_dn10 = assign35640_e50486_d_n10;
        var_rsd_dn11 = assign35640_e50486_d_n11;
        var_rsd_dn12 = assign35640_e50486_d_n12;
        var_rsd_dn17 = assign35640_e50486_d_n17;

        let (assign35650_e50492, assign35650_e50492_d_n0, assign35650_e50492_d_n2, assign35650_e50492_d_n6, assign35650_e50492_d_n7, assign35650_e50492_d_n10, assign35650_e50492_d_n11, assign35650_e50492_d_n12, assign35650_e50492_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35650_e50490: f64 = (var_rsd / var_weff_nf_1);
        (assign35650_e50490, (var_rsd_dn0 / var_weff_nf_1), (var_rsd_dn2 / var_weff_nf_1), (var_rsd_dn6 / var_weff_nf_1), (var_rsd_dn7 / var_weff_nf_1), (var_rsd_dn10 / var_weff_nf_1), (var_rsd_dn11 / var_weff_nf_1), (var_rsd_dn12 / var_weff_nf_1), (var_rsd_dn17 / var_weff_nf_1),)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign35650_e50492;
        var_rsd_dn0 = assign35650_e50492_d_n0;
        var_rsd_dn2 = assign35650_e50492_d_n2;
        var_rsd_dn6 = assign35650_e50492_d_n6;
        var_rsd_dn7 = assign35650_e50492_d_n7;
        var_rsd_dn10 = assign35650_e50492_d_n10;
        var_rsd_dn11 = assign35650_e50492_d_n11;
        var_rsd_dn12 = assign35650_e50492_d_n12;
        var_rsd_dn17 = assign35650_e50492_d_n17;

        let (assign35660_e50498, assign35660_e50498_d_n0, assign35660_e50498_d_n2, assign35660_e50498_d_n6, assign35660_e50498_d_n7, assign35660_e50498_d_n10, assign35660_e50498_d_n11, assign35660_e50498_d_n12, assign35660_e50498_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35660_e50496: f64 = (var_rsd + var_rsd0);
        (assign35660_e50496, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign35660_e50498;
        var_rsd_dn0 = assign35660_e50498_d_n0;
        var_rsd_dn2 = assign35660_e50498_d_n2;
        var_rsd_dn6 = assign35660_e50498_d_n6;
        var_rsd_dn7 = assign35660_e50498_d_n7;
        var_rsd_dn10 = assign35660_e50498_d_n10;
        var_rsd_dn11 = assign35660_e50498_d_n11;
        var_rsd_dn12 = assign35660_e50498_d_n12;
        var_rsd_dn17 = assign35660_e50498_d_n17;

        let assign35680_e50516: f64 = if var_rsd < 0.0001 { 1.0 } else { 0.0 };
        var_guard1180 = assign35680_e50516;

        let (assign35690_e50522, assign35690_e50522_d_n0, assign35690_e50522_d_n2, assign35690_e50522_d_n6, assign35690_e50522_d_n7, assign35690_e50522_d_n10, assign35690_e50522_d_n11, assign35690_e50522_d_n12, assign35690_e50522_d_n17,) = {
    if ((var_guard1153 != 0.0) && (var_guard1180 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign35690_e50522;
        var_rsd_dn0 = assign35690_e50522_d_n0;
        var_rsd_dn2 = assign35690_e50522_d_n2;
        var_rsd_dn6 = assign35690_e50522_d_n6;
        var_rsd_dn7 = assign35690_e50522_d_n7;
        var_rsd_dn10 = assign35690_e50522_d_n10;
        var_rsd_dn11 = assign35690_e50522_d_n11;
        var_rsd_dn12 = assign35690_e50522_d_n12;
        var_rsd_dn17 = assign35690_e50522_d_n17;

        let (assign35700_e50528, assign35700_e50528_d_n0, assign35700_e50528_d_n2, assign35700_e50528_d_n6, assign35700_e50528_d_n7, assign35700_e50528_d_n10, assign35700_e50528_d_n11, assign35700_e50528_d_n12, assign35700_e50528_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35700_e50526: f64 = (var_rsd / var_mfactor);
        (assign35700_e50526, (var_rsd_dn0 / var_mfactor), (var_rsd_dn2 / var_mfactor), (var_rsd_dn6 / var_mfactor), (var_rsd_dn7 / var_mfactor), (var_rsd_dn10 / var_mfactor), (var_rsd_dn11 / var_mfactor), (var_rsd_dn12 / var_mfactor), (var_rsd_dn17 / var_mfactor),)
    } else {
        (var_rsde, var_rsde_dn0, var_rsde_dn2, var_rsde_dn6, var_rsde_dn7, var_rsde_dn10, var_rsde_dn11, var_rsde_dn12, var_rsde_dn17,)
    }
};
        var_rsde = assign35700_e50528;
        var_rsde_dn0 = assign35700_e50528_d_n0;
        var_rsde_dn2 = assign35700_e50528_d_n2;
        var_rsde_dn6 = assign35700_e50528_d_n6;
        var_rsde_dn7 = assign35700_e50528_d_n7;
        var_rsde_dn10 = assign35700_e50528_d_n10;
        var_rsde_dn11 = assign35700_e50528_d_n11;
        var_rsde_dn12 = assign35700_e50528_d_n12;
        var_rsde_dn17 = assign35700_e50528_d_n17;

        let assign35720_e50535: f64 = if p.p260 == 1.0 { 1.0 } else { 0.0 };
        var_guard1181 = assign35720_e50535;

        let (assign35730_e50539,) = {
    if (var_guard1181 != 0.0) {
        (2.0,)
    } else {
        (var_rdmod,)
    }
};
        var_rdmod = assign35730_e50539;

        let assign35740_e50542: f64 = if var_rdmod == 1.0 { 1.0 } else { 0.0 };
        var_guard1201 = assign35740_e50542;

        let (assign35750_e50550,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 != 0.0)) {
        let assign35750_e50548: f64 = (p.p264 / 1e-6);
        (assign35750_e50548,)
    } else {
        (var_nover__blk1192,)
    }
};
        var_nover__blk1192 = assign35750_e50550;

        let (assign35760_e50556,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 != 0.0)) {
        (p.p266,)
    } else {
        (var_mks_rdrmue__blk1185,)
    }
};
        var_mks_rdrmue__blk1185 = assign35760_e50556;

        let (assign35770_e50562,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 != 0.0)) {
        (p.p268,)
    } else {
        (var_mks_rdrvmax__blk1186,)
    }
};
        var_mks_rdrvmax__blk1186 = assign35770_e50562;

        let (assign35780_e50568, assign35780_e50568_d_n10,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (var_rrdrbb__blk1187, var_rrdrbb__blk1187_dn10,)
    }
};
        var_rrdrbb__blk1187 = assign35780_e50568;
        var_rrdrbb__blk1187_dn10 = assign35780_e50568_d_n10;

        let (assign35790_e50581,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 != 0.0)) {
        let (assign35790_e50579,) = {
            if (p.p263 > 0.0) {
                let assign35790_e50577: f64 = (p.p263 * p.p255);
                (assign35790_e50577,)
            } else {
                (0.0,)
            }
        };
        (assign35790_e50579,)
    } else {
        (var_rsd0__blk1188,)
    }
};
        var_rsd0__blk1188 = assign35790_e50581;

        let (assign35800_e50587,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 != 0.0)) {
        (p.p258,)
    } else {
        (var_ldrifte__blk1191,)
    }
};
        var_ldrifte__blk1191 = assign35800_e50587;

        let (assign35810_e50595, assign35810_e50595_d_n0, assign35810_e50595_d_n2, assign35810_e50595_d_n6, assign35810_e50595_d_n7,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 != 0.0)) {
        let assign35810_e50593: f64 = (p.p50 * (nv7 - nv2));
        (assign35810_e50593, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (var_vrdr__blk1189, var_vrdr__blk1189_dn0, var_vrdr__blk1189_dn2, var_vrdr__blk1189_dn6, var_vrdr__blk1189_dn7,)
    }
};
        var_vrdr__blk1189 = assign35810_e50595;
        var_vrdr__blk1189_dn0 = assign35810_e50595_d_n0;
        var_vrdr__blk1189_dn2 = assign35810_e50595_d_n2;
        var_vrdr__blk1189_dn6 = assign35810_e50595_d_n6;
        var_vrdr__blk1189_dn7 = assign35810_e50595_d_n7;

        let (assign35820_e50604,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 == 0.0)) {
        let assign35820_e50602: f64 = (p.p59 / 1e-6);
        (assign35820_e50602,)
    } else {
        (var_nover__blk1192,)
    }
};
        var_nover__blk1192 = assign35820_e50604;

        let (assign35830_e50611,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 == 0.0)) {
        (p.p265,)
    } else {
        (var_mks_rdrmue__blk1185,)
    }
};
        var_mks_rdrmue__blk1185 = assign35830_e50611;

        let (assign35840_e50618,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 == 0.0)) {
        (p.p267,)
    } else {
        (var_mks_rdrvmax__blk1186,)
    }
};
        var_mks_rdrvmax__blk1186 = assign35840_e50618;

        let (assign35850_e50625, assign35850_e50625_d_n10,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (var_rrdrbb__blk1187, var_rrdrbb__blk1187_dn10,)
    }
};
        var_rrdrbb__blk1187 = assign35850_e50625;
        var_rrdrbb__blk1187_dn10 = assign35850_e50625_d_n10;

        let (assign35860_e50639,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 == 0.0)) {
        let (assign35860_e50637,) = {
            if (p.p263 > 0.0) {
                let assign35860_e50635: f64 = (p.p263 * p.p256);
                (assign35860_e50635,)
            } else {
                (0.0,)
            }
        };
        (assign35860_e50637,)
    } else {
        (var_rsd0__blk1188,)
    }
};
        var_rsd0__blk1188 = assign35860_e50639;

        let (assign35870_e50646,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 == 0.0)) {
        (p.p257,)
    } else {
        (var_ldrifte__blk1191,)
    }
};
        var_ldrifte__blk1191 = assign35870_e50646;

        let (assign35880_e50655, assign35880_e50655_d_n0, assign35880_e50655_d_n2, assign35880_e50655_d_n6, assign35880_e50655_d_n7,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 == 0.0)) {
        let assign35880_e50653: f64 = (p.p50 * (nv0 - nv6));
        (assign35880_e50653, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (var_vrdr__blk1189, var_vrdr__blk1189_dn0, var_vrdr__blk1189_dn2, var_vrdr__blk1189_dn6, var_vrdr__blk1189_dn7,)
    }
};
        var_vrdr__blk1189 = assign35880_e50655;
        var_vrdr__blk1189_dn0 = assign35880_e50655_d_n0;
        var_vrdr__blk1189_dn2 = assign35880_e50655_d_n2;
        var_vrdr__blk1189_dn6 = assign35880_e50655_d_n6;
        var_vrdr__blk1189_dn7 = assign35880_e50655_d_n7;

        let (assign35890_e50666,) = {
    if (var_guard1181 != 0.0) {
        let assign35890_e50659: f64 = (p.p271 * p.p271);
        let assign35890_e50662: f64 = (p.p56 * p.p56);
        let assign35890_e50663: f64 = (assign35890_e50659 + assign35890_e50662);
        let assign35890_e50664: f64 = (assign35890_e50663).sqrt();
        (assign35890_e50664,)
    } else {
        (var_xov__blk1198,)
    }
};
        var_xov__blk1198 = assign35890_e50666;

        let (assign35900_e50672,) = {
    if (var_guard1181 != 0.0) {
        let assign35900_e50670: f64 = (var_weff * p.p9);
        (assign35900_e50670,)
    } else {
        (var_weff_nf__blk1200,)
    }
};
        var_weff_nf__blk1200 = assign35900_e50672;

        let (assign35910_e50678,) = {
    if (var_guard1181 != 0.0) {
        let assign35910_e50676: f64 = (var_mks_rdrmue__blk1185 / 10000.0);
        (assign35910_e50676,)
    } else {
        (var_mks_rdrmue__blk1185,)
    }
};
        var_mks_rdrmue__blk1185 = assign35910_e50678;

        let (assign35920_e50684,) = {
    if (var_guard1181 != 0.0) {
        let assign35920_e50682: f64 = (var_mks_rdrvmax__blk1186 / 100.0);
        (assign35920_e50682,)
    } else {
        (var_mks_rdrvmax__blk1186,)
    }
};
        var_mks_rdrvmax__blk1186 = assign35920_e50684;

        let (assign35930_e50690, assign35930_e50690_d_n10,) = {
    if (var_guard1181 != 0.0) {
        let assign35930_e50688: f64 = (var_ttemp / var_uc_tnom);
        (assign35930_e50688, (var_ttemp_dn10 / var_uc_tnom),)
    } else {
        (var_tratio__blk1190, var_tratio__blk1190_dn10,)
    }
};
        var_tratio__blk1190 = assign35930_e50690;
        var_tratio__blk1190_dn10 = assign35930_e50690_d_n10;

        let (assign35940_e50696, assign35940_e50696_d_n0, assign35940_e50696_d_n2, assign35940_e50696_d_n6, assign35940_e50696_d_n7, assign35940_e50696_d_n10, assign35940_e50696_d_n11, assign35940_e50696_d_n12, assign35940_e50696_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign35940_e50694: f64 = (var_tratio__blk1190).powf(p.p269);
        (assign35940_e50694, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((var_tratio__blk1190).powf(p.p269 - 1.0) * var_tratio__blk1190_dn10)) } } else { (assign35940_e50694 * (p.p269 * (var_tratio__blk1190_dn10 / var_tratio__blk1190))) }, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35940_e50696;
        var_t1_dn0 = assign35940_e50696_d_n0;
        var_t1_dn2 = assign35940_e50696_d_n2;
        var_t1_dn6 = assign35940_e50696_d_n6;
        var_t1_dn7 = assign35940_e50696_d_n7;
        var_t1_dn10 = assign35940_e50696_d_n10;
        var_t1_dn11 = assign35940_e50696_d_n11;
        var_t1_dn12 = assign35940_e50696_d_n12;
        var_t1_dn17 = assign35940_e50696_d_n17;

        let (assign35950_e50702, assign35950_e50702_d_n0, assign35950_e50702_d_n2, assign35950_e50702_d_n6, assign35950_e50702_d_n7, assign35950_e50702_d_n10, assign35950_e50702_d_n11, assign35950_e50702_d_n12, assign35950_e50702_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign35950_e50700: f64 = (var_mks_rdrmue__blk1185 / var_t1);
        (assign35950_e50700, (-((var_mks_rdrmue__blk1185 * var_t1_dn0) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn2) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn6) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn7) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn10) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn11) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn12) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn17) / (var_t1 * var_t1))),)
    } else {
        (var_mu0__blk1193, var_mu0__blk1193_dn0, var_mu0__blk1193_dn2, var_mu0__blk1193_dn6, var_mu0__blk1193_dn7, var_mu0__blk1193_dn10, var_mu0__blk1193_dn11, var_mu0__blk1193_dn12, var_mu0__blk1193_dn17,)
    }
};
        var_mu0__blk1193 = assign35950_e50702;
        var_mu0__blk1193_dn0 = assign35950_e50702_d_n0;
        var_mu0__blk1193_dn2 = assign35950_e50702_d_n2;
        var_mu0__blk1193_dn6 = assign35950_e50702_d_n6;
        var_mu0__blk1193_dn7 = assign35950_e50702_d_n7;
        var_mu0__blk1193_dn10 = assign35950_e50702_d_n10;
        var_mu0__blk1193_dn11 = assign35950_e50702_d_n11;
        var_mu0__blk1193_dn12 = assign35950_e50702_d_n12;
        var_mu0__blk1193_dn17 = assign35950_e50702_d_n17;

        let (assign35960_e50722, assign35960_e50722_d_n0, assign35960_e50722_d_n2, assign35960_e50722_d_n6, assign35960_e50722_d_n7, assign35960_e50722_d_n10, assign35960_e50722_d_n11, assign35960_e50722_d_n12, assign35960_e50722_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign35960_e50707: f64 = (0.4 * var_tratio__blk1190);
        let assign35960_e50708: f64 = (1.8 + assign35960_e50707);
        let assign35960_e50711: f64 = (0.1 * var_tratio__blk1190);
        let assign35960_e50713: f64 = (assign35960_e50711 * var_tratio__blk1190);
        let assign35960_e50714: f64 = (assign35960_e50708 + assign35960_e50713);
        let assign35960_e50718: f64 = (1.0 - var_tratio__blk1190);
        let assign35960_e50719: f64 = (p.p270 * assign35960_e50718);
        let assign35960_e50720: f64 = (assign35960_e50714 - assign35960_e50719);
        (assign35960_e50720, 0.0, 0.0, 0.0, 0.0, (((0.4 * var_tratio__blk1190_dn10) + (((0.1 * var_tratio__blk1190_dn10) * var_tratio__blk1190) + (assign35960_e50711 * var_tratio__blk1190_dn10))) - (p.p270 * (-var_tratio__blk1190_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn6, var_t0_dn7, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn17,)
    }
};
        var_t0 = assign35960_e50722;
        var_t0_dn0 = assign35960_e50722_d_n0;
        var_t0_dn2 = assign35960_e50722_d_n2;
        var_t0_dn6 = assign35960_e50722_d_n6;
        var_t0_dn7 = assign35960_e50722_d_n7;
        var_t0_dn10 = assign35960_e50722_d_n10;
        var_t0_dn11 = assign35960_e50722_d_n11;
        var_t0_dn12 = assign35960_e50722_d_n12;
        var_t0_dn17 = assign35960_e50722_d_n17;

        let (assign35970_e50728, assign35970_e50728_d_n0, assign35970_e50728_d_n2, assign35970_e50728_d_n6, assign35970_e50728_d_n7, assign35970_e50728_d_n10, assign35970_e50728_d_n11, assign35970_e50728_d_n12, assign35970_e50728_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign35970_e50726: f64 = (var_mks_rdrvmax__blk1186 / var_t0);
        (assign35970_e50726, (-((var_mks_rdrvmax__blk1186 * var_t0_dn0) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn2) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn6) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn7) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn10) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn11) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn12) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn17) / (var_t0 * var_t0))),)
    } else {
        (var_vmaxe__blk1194, var_vmaxe__blk1194_dn0, var_vmaxe__blk1194_dn2, var_vmaxe__blk1194_dn6, var_vmaxe__blk1194_dn7, var_vmaxe__blk1194_dn10, var_vmaxe__blk1194_dn11, var_vmaxe__blk1194_dn12, var_vmaxe__blk1194_dn17,)
    }
};
        var_vmaxe__blk1194 = assign35970_e50728;
        var_vmaxe__blk1194_dn0 = assign35970_e50728_d_n0;
        var_vmaxe__blk1194_dn2 = assign35970_e50728_d_n2;
        var_vmaxe__blk1194_dn6 = assign35970_e50728_d_n6;
        var_vmaxe__blk1194_dn7 = assign35970_e50728_d_n7;
        var_vmaxe__blk1194_dn10 = assign35970_e50728_d_n10;
        var_vmaxe__blk1194_dn11 = assign35970_e50728_d_n11;
        var_vmaxe__blk1194_dn12 = assign35970_e50728_d_n12;
        var_vmaxe__blk1194_dn17 = assign35970_e50728_d_n17;

        let (assign35980_e50738, assign35980_e50738_d_n10,) = {
    if (var_guard1181 != 0.0) {
        let assign35980_e50734: f64 = (var_ttemp - var_uc_tnom);
        let assign35980_e50735: f64 = (p.p274 * assign35980_e50734);
        let assign35980_e50736: f64 = (var_rrdrbb__blk1187 + assign35980_e50735);
        (assign35980_e50736, (var_rrdrbb__blk1187_dn10 + (p.p274 * var_ttemp_dn10)),)
    } else {
        (var_rrdrbb__blk1187, var_rrdrbb__blk1187_dn10,)
    }
};
        var_rrdrbb__blk1187 = assign35980_e50738;
        var_rrdrbb__blk1187_dn10 = assign35980_e50738_d_n10;

        let (assign35990_e50748,) = {
    if (var_guard1181 != 0.0) {
        let assign35990_e50744: f64 = (var_lgle).powf(p.p280);
        let assign35990_e50745: f64 = (p.p279 / assign35990_e50744);
        let assign35990_e50746: f64 = (1.0 + assign35990_e50745);
        (assign35990_e50746,)
    } else {
        (var_rdrmuele__blk1182,)
    }
};
        var_rdrmuele__blk1182 = assign35990_e50748;

        let (assign36000_e50758,) = {
    if (var_guard1181 != 0.0) {
        let assign36000_e50754: f64 = (var_lgle).powf(p.p278);
        let assign36000_e50755: f64 = (p.p277 / assign36000_e50754);
        let assign36000_e50756: f64 = (1.0 + assign36000_e50755);
        (assign36000_e50756,)
    } else {
        (var_rdrvmaxle__blk1184,)
    }
};
        var_rdrvmaxle__blk1184 = assign36000_e50758;

        let (assign36010_e50768,) = {
    if (var_guard1181 != 0.0) {
        let assign36010_e50764: f64 = (var_wg).powf(p.p276);
        let assign36010_e50765: f64 = (p.p275 / assign36010_e50764);
        let assign36010_e50766: f64 = (1.0 + assign36010_e50765);
        (assign36010_e50766,)
    } else {
        (var_rdrvmaxwe__blk1183,)
    }
};
        var_rdrvmaxwe__blk1183 = assign36010_e50768;

        let (assign36020_e50774, assign36020_e50774_d_n0, assign36020_e50774_d_n2, assign36020_e50774_d_n6, assign36020_e50774_d_n7, assign36020_e50774_d_n10, assign36020_e50774_d_n11, assign36020_e50774_d_n12, assign36020_e50774_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36020_e50772: f64 = (var_mu0__blk1193 * var_rdrmuele__blk1182);
        (assign36020_e50772, (var_mu0__blk1193_dn0 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn2 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn6 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn7 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn10 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn11 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn12 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn17 * var_rdrmuele__blk1182),)
    } else {
        (var_mu0__blk1193, var_mu0__blk1193_dn0, var_mu0__blk1193_dn2, var_mu0__blk1193_dn6, var_mu0__blk1193_dn7, var_mu0__blk1193_dn10, var_mu0__blk1193_dn11, var_mu0__blk1193_dn12, var_mu0__blk1193_dn17,)
    }
};
        var_mu0__blk1193 = assign36020_e50774;
        var_mu0__blk1193_dn0 = assign36020_e50774_d_n0;
        var_mu0__blk1193_dn2 = assign36020_e50774_d_n2;
        var_mu0__blk1193_dn6 = assign36020_e50774_d_n6;
        var_mu0__blk1193_dn7 = assign36020_e50774_d_n7;
        var_mu0__blk1193_dn10 = assign36020_e50774_d_n10;
        var_mu0__blk1193_dn11 = assign36020_e50774_d_n11;
        var_mu0__blk1193_dn12 = assign36020_e50774_d_n12;
        var_mu0__blk1193_dn17 = assign36020_e50774_d_n17;

        let (assign36030_e50784, assign36030_e50784_d_n0, assign36030_e50784_d_n2, assign36030_e50784_d_n6, assign36030_e50784_d_n7, assign36030_e50784_d_n10, assign36030_e50784_d_n11, assign36030_e50784_d_n12, assign36030_e50784_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36030_e50778: f64 = (var_vmaxe__blk1194 * var_rdrvmaxwe__blk1183);
        let assign36030_e50780: f64 = (assign36030_e50778 * var_rdrvmaxle__blk1184);
        let assign36030_e50782: f64 = (assign36030_e50780 + 1e-50);
        (assign36030_e50782, ((var_vmaxe__blk1194_dn0 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn2 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn6 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn7 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn10 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn11 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn12 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn17 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184),)
    } else {
        (var_vmaxe__blk1194, var_vmaxe__blk1194_dn0, var_vmaxe__blk1194_dn2, var_vmaxe__blk1194_dn6, var_vmaxe__blk1194_dn7, var_vmaxe__blk1194_dn10, var_vmaxe__blk1194_dn11, var_vmaxe__blk1194_dn12, var_vmaxe__blk1194_dn17,)
    }
};
        var_vmaxe__blk1194 = assign36030_e50784;
        var_vmaxe__blk1194_dn0 = assign36030_e50784_d_n0;
        var_vmaxe__blk1194_dn2 = assign36030_e50784_d_n2;
        var_vmaxe__blk1194_dn6 = assign36030_e50784_d_n6;
        var_vmaxe__blk1194_dn7 = assign36030_e50784_d_n7;
        var_vmaxe__blk1194_dn10 = assign36030_e50784_d_n10;
        var_vmaxe__blk1194_dn11 = assign36030_e50784_d_n11;
        var_vmaxe__blk1194_dn12 = assign36030_e50784_d_n12;
        var_vmaxe__blk1194_dn17 = assign36030_e50784_d_n17;

        let (assign36040_e50790, assign36040_e50790_d_n0, assign36040_e50790_d_n2, assign36040_e50790_d_n6, assign36040_e50790_d_n7,) = {
    if (var_guard1181 != 0.0) {
        let assign36040_e50788: f64 = (var_vrdr__blk1189 / var_ldrifte__blk1191);
        (assign36040_e50788, (var_vrdr__blk1189_dn0 / var_ldrifte__blk1191), (var_vrdr__blk1189_dn2 / var_ldrifte__blk1191), (var_vrdr__blk1189_dn6 / var_ldrifte__blk1191), (var_vrdr__blk1189_dn7 / var_ldrifte__blk1191),)
    } else {
        (var_edri__blk1195, var_edri__blk1195_dn0, var_edri__blk1195_dn2, var_edri__blk1195_dn6, var_edri__blk1195_dn7,)
    }
};
        var_edri__blk1195 = assign36040_e50790;
        var_edri__blk1195_dn0 = assign36040_e50790_d_n0;
        var_edri__blk1195_dn2 = assign36040_e50790_d_n2;
        var_edri__blk1195_dn6 = assign36040_e50790_d_n6;
        var_edri__blk1195_dn7 = assign36040_e50790_d_n7;

        *var_edri__blk1195_slot = var_edri__blk1195;
        *var_edri__blk1195_dn0_slot = var_edri__blk1195_dn0;
        *var_edri__blk1195_dn2_slot = var_edri__blk1195_dn2;
        *var_edri__blk1195_dn6_slot = var_edri__blk1195_dn6;
        *var_edri__blk1195_dn7_slot = var_edri__blk1195_dn7;
        *var_guard1180_slot = var_guard1180;
        *var_guard1181_slot = var_guard1181;
        *var_guard1201_slot = var_guard1201;
        *var_ldrifte__blk1191_slot = var_ldrifte__blk1191;
        *var_mks_rdrmue__blk1185_slot = var_mks_rdrmue__blk1185;
        *var_mks_rdrvmax__blk1186_slot = var_mks_rdrvmax__blk1186;
        *var_mu0__blk1193_slot = var_mu0__blk1193;
        *var_mu0__blk1193_dn0_slot = var_mu0__blk1193_dn0;
        *var_mu0__blk1193_dn10_slot = var_mu0__blk1193_dn10;
        *var_mu0__blk1193_dn11_slot = var_mu0__blk1193_dn11;
        *var_mu0__blk1193_dn12_slot = var_mu0__blk1193_dn12;
        *var_mu0__blk1193_dn17_slot = var_mu0__blk1193_dn17;
        *var_mu0__blk1193_dn2_slot = var_mu0__blk1193_dn2;
        *var_mu0__blk1193_dn6_slot = var_mu0__blk1193_dn6;
        *var_mu0__blk1193_dn7_slot = var_mu0__blk1193_dn7;
        *var_nover__blk1192_slot = var_nover__blk1192;
        *var_rdmod_slot = var_rdmod;
        *var_rdrmuele__blk1182_slot = var_rdrmuele__blk1182;
        *var_rdrvmaxle__blk1184_slot = var_rdrvmaxle__blk1184;
        *var_rdrvmaxwe__blk1183_slot = var_rdrvmaxwe__blk1183;
        *var_rrdrbb__blk1187_slot = var_rrdrbb__blk1187;
        *var_rrdrbb__blk1187_dn10_slot = var_rrdrbb__blk1187_dn10;
        *var_rsd_slot = var_rsd;
        *var_rsd0__blk1188_slot = var_rsd0__blk1188;
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
        *var_tratio__blk1190_slot = var_tratio__blk1190;
        *var_tratio__blk1190_dn10_slot = var_tratio__blk1190_dn10;
        *var_vmaxe__blk1194_slot = var_vmaxe__blk1194;
        *var_vmaxe__blk1194_dn0_slot = var_vmaxe__blk1194_dn0;
        *var_vmaxe__blk1194_dn10_slot = var_vmaxe__blk1194_dn10;
        *var_vmaxe__blk1194_dn11_slot = var_vmaxe__blk1194_dn11;
        *var_vmaxe__blk1194_dn12_slot = var_vmaxe__blk1194_dn12;
        *var_vmaxe__blk1194_dn17_slot = var_vmaxe__blk1194_dn17;
        *var_vmaxe__blk1194_dn2_slot = var_vmaxe__blk1194_dn2;
        *var_vmaxe__blk1194_dn6_slot = var_vmaxe__blk1194_dn6;
        *var_vmaxe__blk1194_dn7_slot = var_vmaxe__blk1194_dn7;
        *var_vrdr__blk1189_slot = var_vrdr__blk1189;
        *var_vrdr__blk1189_dn0_slot = var_vrdr__blk1189_dn0;
        *var_vrdr__blk1189_dn2_slot = var_vrdr__blk1189_dn2;
        *var_vrdr__blk1189_dn6_slot = var_vrdr__blk1189_dn6;
        *var_vrdr__blk1189_dn7_slot = var_vrdr__blk1189_dn7;
        *var_weff_nf__blk1200_slot = var_weff_nf__blk1200;
        *var_xov__blk1198_slot = var_xov__blk1198;
    }

    pub(super) fn stamp_transient_block_125(
        p: &Parameters,
        var_edri__blk1195: f64,
        var_edri__blk1195_dn0: f64,
        var_edri__blk1195_dn2: f64,
        var_edri__blk1195_dn6: f64,
        var_edri__blk1195_dn7: f64,
        var_flg_nqs: f64,
        var_guard1181: f64,
        var_ldrifte__blk1191: f64,
        var_mfactor: f64,
        var_mode: f64,
        var_mu0__blk1193: f64,
        var_mu0__blk1193_dn0: f64,
        var_mu0__blk1193_dn10: f64,
        var_mu0__blk1193_dn11: f64,
        var_mu0__blk1193_dn12: f64,
        var_mu0__blk1193_dn17: f64,
        var_mu0__blk1193_dn2: f64,
        var_mu0__blk1193_dn6: f64,
        var_mu0__blk1193_dn7: f64,
        var_nover__blk1192: f64,
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
        var_rrdrbb__blk1187: f64,
        var_rrdrbb__blk1187_dn10: f64,
        var_rsd0__blk1188: f64,
        var_vmaxe__blk1194: f64,
        var_vmaxe__blk1194_dn0: f64,
        var_vmaxe__blk1194_dn10: f64,
        var_vmaxe__blk1194_dn11: f64,
        var_vmaxe__blk1194_dn12: f64,
        var_vmaxe__blk1194_dn17: f64,
        var_vmaxe__blk1194_dn2: f64,
        var_vmaxe__blk1194_dn6: f64,
        var_vmaxe__blk1194_dn7: f64,
        var_vrdr__blk1189: f64,
        var_weff_nf__blk1200: f64,
        var_xd: f64,
        var_xd_dn0: f64,
        var_xd_dn10: f64,
        var_xd_dn11: f64,
        var_xd_dn12: f64,
        var_xd_dn17: f64,
        var_xd_dn2: f64,
        var_xd_dn6: f64,
        var_xd_dn7: f64,
        var_xov__blk1198: f64,
        var_gd__blk1199_slot: &mut f64,
        var_gd__blk1199_dn0_slot: &mut f64,
        var_gd__blk1199_dn10_slot: &mut f64,
        var_gd__blk1199_dn11_slot: &mut f64,
        var_gd__blk1199_dn12_slot: &mut f64,
        var_gd__blk1199_dn17_slot: &mut f64,
        var_gd__blk1199_dn2_slot: &mut f64,
        var_gd__blk1199_dn6_slot: &mut f64,
        var_gd__blk1199_dn7_slot: &mut f64,
        var_guard1202_slot: &mut f64,
        var_guard1203_slot: &mut f64,
        var_guard1204_slot: &mut f64,
        var_guard1205_slot: &mut f64,
        var_guard1206_slot: &mut f64,
        var_guard1207_slot: &mut f64,
        var_guard1208_slot: &mut f64,
        var_guard1209_slot: &mut f64,
        var_guard1210_slot: &mut f64,
        var_guard1211_slot: &mut f64,
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
        var_mu__blk1197_slot: &mut f64,
        var_mu__blk1197_dn0_slot: &mut f64,
        var_mu__blk1197_dn10_slot: &mut f64,
        var_mu__blk1197_dn11_slot: &mut f64,
        var_mu__blk1197_dn12_slot: &mut f64,
        var_mu__blk1197_dn17_slot: &mut f64,
        var_mu__blk1197_dn2_slot: &mut f64,
        var_mu__blk1197_dn6_slot: &mut f64,
        var_mu__blk1197_dn7_slot: &mut f64,
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
        var_vdri__blk1196_slot: &mut f64,
        var_vdri__blk1196_dn0_slot: &mut f64,
        var_vdri__blk1196_dn10_slot: &mut f64,
        var_vdri__blk1196_dn11_slot: &mut f64,
        var_vdri__blk1196_dn12_slot: &mut f64,
        var_vdri__blk1196_dn17_slot: &mut f64,
        var_vdri__blk1196_dn2_slot: &mut f64,
        var_vdri__blk1196_dn6_slot: &mut f64,
        var_vdri__blk1196_dn7_slot: &mut f64,
    ) {
        let mut var_gd__blk1199: f64 = *var_gd__blk1199_slot;
        let mut var_gd__blk1199_dn0: f64 = *var_gd__blk1199_dn0_slot;
        let mut var_gd__blk1199_dn10: f64 = *var_gd__blk1199_dn10_slot;
        let mut var_gd__blk1199_dn11: f64 = *var_gd__blk1199_dn11_slot;
        let mut var_gd__blk1199_dn12: f64 = *var_gd__blk1199_dn12_slot;
        let mut var_gd__blk1199_dn17: f64 = *var_gd__blk1199_dn17_slot;
        let mut var_gd__blk1199_dn2: f64 = *var_gd__blk1199_dn2_slot;
        let mut var_gd__blk1199_dn6: f64 = *var_gd__blk1199_dn6_slot;
        let mut var_gd__blk1199_dn7: f64 = *var_gd__blk1199_dn7_slot;
        let mut var_guard1202: f64 = *var_guard1202_slot;
        let mut var_guard1203: f64 = *var_guard1203_slot;
        let mut var_guard1204: f64 = *var_guard1204_slot;
        let mut var_guard1205: f64 = *var_guard1205_slot;
        let mut var_guard1206: f64 = *var_guard1206_slot;
        let mut var_guard1207: f64 = *var_guard1207_slot;
        let mut var_guard1208: f64 = *var_guard1208_slot;
        let mut var_guard1209: f64 = *var_guard1209_slot;
        let mut var_guard1210: f64 = *var_guard1210_slot;
        let mut var_guard1211: f64 = *var_guard1211_slot;
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
        let mut var_mu__blk1197: f64 = *var_mu__blk1197_slot;
        let mut var_mu__blk1197_dn0: f64 = *var_mu__blk1197_dn0_slot;
        let mut var_mu__blk1197_dn10: f64 = *var_mu__blk1197_dn10_slot;
        let mut var_mu__blk1197_dn11: f64 = *var_mu__blk1197_dn11_slot;
        let mut var_mu__blk1197_dn12: f64 = *var_mu__blk1197_dn12_slot;
        let mut var_mu__blk1197_dn17: f64 = *var_mu__blk1197_dn17_slot;
        let mut var_mu__blk1197_dn2: f64 = *var_mu__blk1197_dn2_slot;
        let mut var_mu__blk1197_dn6: f64 = *var_mu__blk1197_dn6_slot;
        let mut var_mu__blk1197_dn7: f64 = *var_mu__blk1197_dn7_slot;
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
        let mut var_vdri__blk1196: f64 = *var_vdri__blk1196_slot;
        let mut var_vdri__blk1196_dn0: f64 = *var_vdri__blk1196_dn0_slot;
        let mut var_vdri__blk1196_dn10: f64 = *var_vdri__blk1196_dn10_slot;
        let mut var_vdri__blk1196_dn11: f64 = *var_vdri__blk1196_dn11_slot;
        let mut var_vdri__blk1196_dn12: f64 = *var_vdri__blk1196_dn12_slot;
        let mut var_vdri__blk1196_dn17: f64 = *var_vdri__blk1196_dn17_slot;
        let mut var_vdri__blk1196_dn2: f64 = *var_vdri__blk1196_dn2_slot;
        let mut var_vdri__blk1196_dn6: f64 = *var_vdri__blk1196_dn6_slot;
        let mut var_vdri__blk1196_dn7: f64 = *var_vdri__blk1196_dn7_slot;

        let (assign36050_e50796, assign36050_e50796_d_n0, assign36050_e50796_d_n2, assign36050_e50796_d_n6, assign36050_e50796_d_n7, assign36050_e50796_d_n10, assign36050_e50796_d_n11, assign36050_e50796_d_n12, assign36050_e50796_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36050_e50794: f64 = (var_mu0__blk1193 * var_edri__blk1195);
        (assign36050_e50794, ((var_mu0__blk1193_dn0 * var_edri__blk1195) + (var_mu0__blk1193 * var_edri__blk1195_dn0)), ((var_mu0__blk1193_dn2 * var_edri__blk1195) + (var_mu0__blk1193 * var_edri__blk1195_dn2)), ((var_mu0__blk1193_dn6 * var_edri__blk1195) + (var_mu0__blk1193 * var_edri__blk1195_dn6)), ((var_mu0__blk1193_dn7 * var_edri__blk1195) + (var_mu0__blk1193 * var_edri__blk1195_dn7)), (var_mu0__blk1193_dn10 * var_edri__blk1195), (var_mu0__blk1193_dn11 * var_edri__blk1195), (var_mu0__blk1193_dn12 * var_edri__blk1195), (var_mu0__blk1193_dn17 * var_edri__blk1195),)
    } else {
        (var_vdri__blk1196, var_vdri__blk1196_dn0, var_vdri__blk1196_dn2, var_vdri__blk1196_dn6, var_vdri__blk1196_dn7, var_vdri__blk1196_dn10, var_vdri__blk1196_dn11, var_vdri__blk1196_dn12, var_vdri__blk1196_dn17,)
    }
};
        var_vdri__blk1196 = assign36050_e50796;
        var_vdri__blk1196_dn0 = assign36050_e50796_d_n0;
        var_vdri__blk1196_dn2 = assign36050_e50796_d_n2;
        var_vdri__blk1196_dn6 = assign36050_e50796_d_n6;
        var_vdri__blk1196_dn7 = assign36050_e50796_d_n7;
        var_vdri__blk1196_dn10 = assign36050_e50796_d_n10;
        var_vdri__blk1196_dn11 = assign36050_e50796_d_n11;
        var_vdri__blk1196_dn12 = assign36050_e50796_d_n12;
        var_vdri__blk1196_dn17 = assign36050_e50796_d_n17;

        let assign36060_e50799: f64 = if var_vrdr__blk1189 >= 0.0 { 1.0 } else { 0.0 };
        var_guard1202 = assign36060_e50799;

        let (assign36070_e50807, assign36070_e50807_d_n0, assign36070_e50807_d_n2, assign36070_e50807_d_n6, assign36070_e50807_d_n7, assign36070_e50807_d_n10, assign36070_e50807_d_n11, assign36070_e50807_d_n12, assign36070_e50807_d_n17,) = {
    if ((var_guard1181 != 0.0) && (var_guard1202 != 0.0)) {
        let assign36070_e50805: f64 = (var_vdri__blk1196 / var_vmaxe__blk1194);
        (assign36070_e50805, (((var_vdri__blk1196_dn0 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn0)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn2 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn2)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn6 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn6)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn7 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn7)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn10 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn10)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn11 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn11)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn12 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn12)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn17 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn17)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign36070_e50807;
        var_t1_dn0 = assign36070_e50807_d_n0;
        var_t1_dn2 = assign36070_e50807_d_n2;
        var_t1_dn6 = assign36070_e50807_d_n6;
        var_t1_dn7 = assign36070_e50807_d_n7;
        var_t1_dn10 = assign36070_e50807_d_n10;
        var_t1_dn11 = assign36070_e50807_d_n11;
        var_t1_dn12 = assign36070_e50807_d_n12;
        var_t1_dn17 = assign36070_e50807_d_n17;

        let (assign36080_e50817, assign36080_e50817_d_n0, assign36080_e50817_d_n2, assign36080_e50817_d_n6, assign36080_e50817_d_n7, assign36080_e50817_d_n10, assign36080_e50817_d_n11, assign36080_e50817_d_n12, assign36080_e50817_d_n17,) = {
    if ((var_guard1181 != 0.0) && (var_guard1202 == 0.0)) {
        let assign36080_e50813: f64 = (-var_vdri__blk1196);
        let assign36080_e50815: f64 = (assign36080_e50813 / var_vmaxe__blk1194);
        (assign36080_e50815, ((((-var_vdri__blk1196_dn0) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn0)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn2) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn2)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn6) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn6)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn7) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn7)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn10) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn10)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn11) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn11)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn12) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn12)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn17) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn17)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign36080_e50817;
        var_t1_dn0 = assign36080_e50817_d_n0;
        var_t1_dn2 = assign36080_e50817_d_n2;
        var_t1_dn6 = assign36080_e50817_d_n6;
        var_t1_dn7 = assign36080_e50817_d_n7;
        var_t1_dn10 = assign36080_e50817_d_n10;
        var_t1_dn11 = assign36080_e50817_d_n11;
        var_t1_dn12 = assign36080_e50817_d_n12;
        var_t1_dn17 = assign36080_e50817_d_n17;

        let assign36090_e50821: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50822: f64 = (1.0 - assign36090_e50821);
        let assign36090_e50829: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50830: f64 = (1.0 + assign36090_e50829);
        let assign36090_e50832: f64 = if ((assign36090_e50822 <= var_rrdrbb__blk1187) && (var_rrdrbb__blk1187 <= assign36090_e50830)) { 1.0 } else { 0.0 };
        var_guard1203 = assign36090_e50832;

        let (assign36100_e50838, assign36100_e50838_d_n0, assign36100_e50838_d_n2, assign36100_e50838_d_n6, assign36100_e50838_d_n7, assign36100_e50838_d_n10, assign36100_e50838_d_n11, assign36100_e50838_d_n12, assign36100_e50838_d_n17,) = {
    if ((var_guard1181 != 0.0) && (var_guard1203 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign36100_e50838;
        var_t3_dn0 = assign36100_e50838_d_n0;
        var_t3_dn2 = assign36100_e50838_d_n2;
        var_t3_dn6 = assign36100_e50838_d_n6;
        var_t3_dn7 = assign36100_e50838_d_n7;
        var_t3_dn10 = assign36100_e50838_d_n10;
        var_t3_dn11 = assign36100_e50838_d_n11;
        var_t3_dn12 = assign36100_e50838_d_n12;
        var_t3_dn17 = assign36100_e50838_d_n17;

        let assign36110_e50842: f64 = (10.0 * 2.220446049250313e-16);
        let assign36110_e50843: f64 = (2.0 - assign36110_e50842);
        let assign36110_e50850: f64 = (10.0 * 2.220446049250313e-16);
        let assign36110_e50851: f64 = (2.0 + assign36110_e50850);
        let assign36110_e50853: f64 = if ((assign36110_e50843 <= var_rrdrbb__blk1187) && (var_rrdrbb__blk1187 <= assign36110_e50851)) { 1.0 } else { 0.0 };
        var_guard1204 = assign36110_e50853;

        let (assign36120_e50862, assign36120_e50862_d_n0, assign36120_e50862_d_n2, assign36120_e50862_d_n6, assign36120_e50862_d_n7, assign36120_e50862_d_n10, assign36120_e50862_d_n11, assign36120_e50862_d_n12, assign36120_e50862_d_n17,) = {
    if (((var_guard1181 != 0.0) && (var_guard1203 == 0.0)) && (var_guard1204 != 0.0)) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign36120_e50862;
        var_t3_dn0 = assign36120_e50862_d_n0;
        var_t3_dn2 = assign36120_e50862_d_n2;
        var_t3_dn6 = assign36120_e50862_d_n6;
        var_t3_dn7 = assign36120_e50862_d_n7;
        var_t3_dn10 = assign36120_e50862_d_n10;
        var_t3_dn11 = assign36120_e50862_d_n11;
        var_t3_dn12 = assign36120_e50862_d_n12;
        var_t3_dn17 = assign36120_e50862_d_n17;

        let (assign36130_e50876, assign36130_e50876_d_n0, assign36130_e50876_d_n2, assign36130_e50876_d_n6, assign36130_e50876_d_n7, assign36130_e50876_d_n10, assign36130_e50876_d_n11, assign36130_e50876_d_n12, assign36130_e50876_d_n17,) = {
    if (((var_guard1181 != 0.0) && (var_guard1203 == 0.0)) && (var_guard1204 == 0.0)) {
        let assign36130_e50873: f64 = (var_rrdrbb__blk1187 - 1.0);
        let assign36130_e50874: f64 = (var_t1).powf(assign36130_e50873);
        (assign36130_e50874, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn0)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn0 / var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn2)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn2 / var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn6)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn6 / var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn7)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn7 / var_t1))) }, if var_rrdrbb__blk1187_dn10 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn10)) } } else { (assign36130_e50874 * ((var_rrdrbb__blk1187_dn10 * (var_t1).ln()) + (assign36130_e50873 * (var_t1_dn10 / var_t1)))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn11)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn11 / var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn12)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn12 / var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn17)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn17 / var_t1))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign36130_e50876;
        var_t3_dn0 = assign36130_e50876_d_n0;
        var_t3_dn2 = assign36130_e50876_d_n2;
        var_t3_dn6 = assign36130_e50876_d_n6;
        var_t3_dn7 = assign36130_e50876_d_n7;
        var_t3_dn10 = assign36130_e50876_d_n10;
        var_t3_dn11 = assign36130_e50876_d_n11;
        var_t3_dn12 = assign36130_e50876_d_n12;
        var_t3_dn17 = assign36130_e50876_d_n17;

        let (assign36140_e50882, assign36140_e50882_d_n0, assign36140_e50882_d_n2, assign36140_e50882_d_n6, assign36140_e50882_d_n7, assign36140_e50882_d_n10, assign36140_e50882_d_n11, assign36140_e50882_d_n12, assign36140_e50882_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36140_e50880: f64 = (var_t1 * var_t3);
        (assign36140_e50880, ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)), ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)), ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)), ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)), ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)), ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)), ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)), ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign36140_e50882;
        var_t2_dn0 = assign36140_e50882_d_n0;
        var_t2_dn2 = assign36140_e50882_d_n2;
        var_t2_dn6 = assign36140_e50882_d_n6;
        var_t2_dn7 = assign36140_e50882_d_n7;
        var_t2_dn10 = assign36140_e50882_d_n10;
        var_t2_dn11 = assign36140_e50882_d_n11;
        var_t2_dn12 = assign36140_e50882_d_n12;
        var_t2_dn17 = assign36140_e50882_d_n17;

        let (assign36150_e50888, assign36150_e50888_d_n0, assign36150_e50888_d_n2, assign36150_e50888_d_n6, assign36150_e50888_d_n7, assign36150_e50888_d_n10, assign36150_e50888_d_n11, assign36150_e50888_d_n12, assign36150_e50888_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36150_e50886: f64 = (1.0 + var_t2);
        (assign36150_e50886, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign36150_e50888;
        var_t4_dn0 = assign36150_e50888_d_n0;
        var_t4_dn2 = assign36150_e50888_d_n2;
        var_t4_dn6 = assign36150_e50888_d_n6;
        var_t4_dn7 = assign36150_e50888_d_n7;
        var_t4_dn10 = assign36150_e50888_d_n10;
        var_t4_dn11 = assign36150_e50888_d_n11;
        var_t4_dn12 = assign36150_e50888_d_n12;
        var_t4_dn17 = assign36150_e50888_d_n17;

        let assign36160_e50892: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50893: f64 = (1.0 - assign36160_e50892);
        let assign36160_e50900: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50901: f64 = (1.0 + assign36160_e50900);
        let assign36160_e50903: f64 = if ((assign36160_e50893 <= var_rrdrbb__blk1187) && (var_rrdrbb__blk1187 <= assign36160_e50901)) { 1.0 } else { 0.0 };
        var_guard1205 = assign36160_e50903;

        let (assign36170_e50911, assign36170_e50911_d_n0, assign36170_e50911_d_n2, assign36170_e50911_d_n6, assign36170_e50911_d_n7, assign36170_e50911_d_n10, assign36170_e50911_d_n11, assign36170_e50911_d_n12, assign36170_e50911_d_n17,) = {
    if ((var_guard1181 != 0.0) && (var_guard1205 != 0.0)) {
        let assign36170_e50909: f64 = (1.0 / var_t4);
        (assign36170_e50909, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn11 / (var_t4 * var_t4))), (-(var_t4_dn12 / (var_t4 * var_t4))), (-(var_t4_dn17 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign36170_e50911;
        var_t5_dn0 = assign36170_e50911_d_n0;
        var_t5_dn2 = assign36170_e50911_d_n2;
        var_t5_dn6 = assign36170_e50911_d_n6;
        var_t5_dn7 = assign36170_e50911_d_n7;
        var_t5_dn10 = assign36170_e50911_d_n10;
        var_t5_dn11 = assign36170_e50911_d_n11;
        var_t5_dn12 = assign36170_e50911_d_n12;
        var_t5_dn17 = assign36170_e50911_d_n17;

        let assign36180_e50915: f64 = (10.0 * 2.220446049250313e-16);
        let assign36180_e50916: f64 = (2.0 - assign36180_e50915);
        let assign36180_e50923: f64 = (10.0 * 2.220446049250313e-16);
        let assign36180_e50924: f64 = (2.0 + assign36180_e50923);
        let assign36180_e50926: f64 = if ((assign36180_e50916 <= var_rrdrbb__blk1187) && (var_rrdrbb__blk1187 <= assign36180_e50924)) { 1.0 } else { 0.0 };
        var_guard1206 = assign36180_e50926;

        let (assign36190_e50938, assign36190_e50938_d_n0, assign36190_e50938_d_n2, assign36190_e50938_d_n6, assign36190_e50938_d_n7, assign36190_e50938_d_n10, assign36190_e50938_d_n11, assign36190_e50938_d_n12, assign36190_e50938_d_n17,) = {
    if (((var_guard1181 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 != 0.0)) {
        let assign36190_e50935: f64 = (var_t4).sqrt();
        let assign36190_e50936: f64 = (1.0 / assign36190_e50935);
        (assign36190_e50936, (-((var_t4_dn0 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn2 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn6 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn7 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn10 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn11 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn12 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn17 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign36190_e50938;
        var_t5_dn0 = assign36190_e50938_d_n0;
        var_t5_dn2 = assign36190_e50938_d_n2;
        var_t5_dn6 = assign36190_e50938_d_n6;
        var_t5_dn7 = assign36190_e50938_d_n7;
        var_t5_dn10 = assign36190_e50938_d_n10;
        var_t5_dn11 = assign36190_e50938_d_n11;
        var_t5_dn12 = assign36190_e50938_d_n12;
        var_t5_dn17 = assign36190_e50938_d_n17;

        let (assign36200_e50955, assign36200_e50955_d_n0, assign36200_e50955_d_n2, assign36200_e50955_d_n6, assign36200_e50955_d_n7, assign36200_e50955_d_n10, assign36200_e50955_d_n11, assign36200_e50955_d_n12, assign36200_e50955_d_n17,) = {
    if (((var_guard1181 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 == 0.0)) {
        let assign36200_e50948: f64 = (-1.0);
        let assign36200_e50950: f64 = (assign36200_e50948 / var_rrdrbb__blk1187);
        let assign36200_e50952: f64 = (assign36200_e50950 - 1.0);
        let assign36200_e50953: f64 = (var_t4).powf(assign36200_e50952);
        (assign36200_e50953, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn0)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn0 / var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn2)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn2 / var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn6)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn6 / var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn7)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn7 / var_t4))) }, if (-((assign36200_e50948 * var_rrdrbb__blk1187_dn10) / (var_rrdrbb__blk1187 * var_rrdrbb__blk1187))) == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn10)) } } else { (assign36200_e50953 * (((-((assign36200_e50948 * var_rrdrbb__blk1187_dn10) / (var_rrdrbb__blk1187 * var_rrdrbb__blk1187))) * (var_t4).ln()) + (assign36200_e50952 * (var_t4_dn10 / var_t4)))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn11)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn11 / var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn12)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn12 / var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn17)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn17 / var_t4))) },)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn6, var_t6_dn7, var_t6_dn10, var_t6_dn11, var_t6_dn12, var_t6_dn17,)
    }
};
        var_t6 = assign36200_e50955;
        var_t6_dn0 = assign36200_e50955_d_n0;
        var_t6_dn2 = assign36200_e50955_d_n2;
        var_t6_dn6 = assign36200_e50955_d_n6;
        var_t6_dn7 = assign36200_e50955_d_n7;
        var_t6_dn10 = assign36200_e50955_d_n10;
        var_t6_dn11 = assign36200_e50955_d_n11;
        var_t6_dn12 = assign36200_e50955_d_n12;
        var_t6_dn17 = assign36200_e50955_d_n17;

        let (assign36210_e50967, assign36210_e50967_d_n0, assign36210_e50967_d_n2, assign36210_e50967_d_n6, assign36210_e50967_d_n7, assign36210_e50967_d_n10, assign36210_e50967_d_n11, assign36210_e50967_d_n12, assign36210_e50967_d_n17,) = {
    if (((var_guard1181 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 == 0.0)) {
        let assign36210_e50965: f64 = (var_t4 * var_t6);
        (assign36210_e50965, ((var_t4_dn0 * var_t6) + (var_t4 * var_t6_dn0)), ((var_t4_dn2 * var_t6) + (var_t4 * var_t6_dn2)), ((var_t4_dn6 * var_t6) + (var_t4 * var_t6_dn6)), ((var_t4_dn7 * var_t6) + (var_t4 * var_t6_dn7)), ((var_t4_dn10 * var_t6) + (var_t4 * var_t6_dn10)), ((var_t4_dn11 * var_t6) + (var_t4 * var_t6_dn11)), ((var_t4_dn12 * var_t6) + (var_t4 * var_t6_dn12)), ((var_t4_dn17 * var_t6) + (var_t4 * var_t6_dn17)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign36210_e50967;
        var_t5_dn0 = assign36210_e50967_d_n0;
        var_t5_dn2 = assign36210_e50967_d_n2;
        var_t5_dn6 = assign36210_e50967_d_n6;
        var_t5_dn7 = assign36210_e50967_d_n7;
        var_t5_dn10 = assign36210_e50967_d_n10;
        var_t5_dn11 = assign36210_e50967_d_n11;
        var_t5_dn12 = assign36210_e50967_d_n12;
        var_t5_dn17 = assign36210_e50967_d_n17;

        let (assign36220_e50973, assign36220_e50973_d_n0, assign36220_e50973_d_n2, assign36220_e50973_d_n6, assign36220_e50973_d_n7, assign36220_e50973_d_n10, assign36220_e50973_d_n11, assign36220_e50973_d_n12, assign36220_e50973_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36220_e50971: f64 = (var_mu0__blk1193 * var_t5);
        (assign36220_e50971, ((var_mu0__blk1193_dn0 * var_t5) + (var_mu0__blk1193 * var_t5_dn0)), ((var_mu0__blk1193_dn2 * var_t5) + (var_mu0__blk1193 * var_t5_dn2)), ((var_mu0__blk1193_dn6 * var_t5) + (var_mu0__blk1193 * var_t5_dn6)), ((var_mu0__blk1193_dn7 * var_t5) + (var_mu0__blk1193 * var_t5_dn7)), ((var_mu0__blk1193_dn10 * var_t5) + (var_mu0__blk1193 * var_t5_dn10)), ((var_mu0__blk1193_dn11 * var_t5) + (var_mu0__blk1193 * var_t5_dn11)), ((var_mu0__blk1193_dn12 * var_t5) + (var_mu0__blk1193 * var_t5_dn12)), ((var_mu0__blk1193_dn17 * var_t5) + (var_mu0__blk1193 * var_t5_dn17)),)
    } else {
        (var_mu__blk1197, var_mu__blk1197_dn0, var_mu__blk1197_dn2, var_mu__blk1197_dn6, var_mu__blk1197_dn7, var_mu__blk1197_dn10, var_mu__blk1197_dn11, var_mu__blk1197_dn12, var_mu__blk1197_dn17,)
    }
};
        var_mu__blk1197 = assign36220_e50973;
        var_mu__blk1197_dn0 = assign36220_e50973_d_n0;
        var_mu__blk1197_dn2 = assign36220_e50973_d_n2;
        var_mu__blk1197_dn6 = assign36220_e50973_d_n6;
        var_mu__blk1197_dn7 = assign36220_e50973_d_n7;
        var_mu__blk1197_dn10 = assign36220_e50973_d_n10;
        var_mu__blk1197_dn11 = assign36220_e50973_d_n11;
        var_mu__blk1197_dn12 = assign36220_e50973_d_n12;
        var_mu__blk1197_dn17 = assign36220_e50973_d_n17;

        let (assign36230_e50979, assign36230_e50979_d_n0, assign36230_e50979_d_n2, assign36230_e50979_d_n6, assign36230_e50979_d_n7, assign36230_e50979_d_n10, assign36230_e50979_d_n11, assign36230_e50979_d_n12, assign36230_e50979_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36230_e50977: f64 = (1.6021918e-19 / var_ldrifte__blk1191);
        (assign36230_e50977, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign36230_e50979;
        var_t1_dn0 = assign36230_e50979_d_n0;
        var_t1_dn2 = assign36230_e50979_d_n2;
        var_t1_dn6 = assign36230_e50979_d_n6;
        var_t1_dn7 = assign36230_e50979_d_n7;
        var_t1_dn10 = assign36230_e50979_d_n10;
        var_t1_dn11 = assign36230_e50979_d_n11;
        var_t1_dn12 = assign36230_e50979_d_n12;
        var_t1_dn17 = assign36230_e50979_d_n17;

        let (assign36240_e50989, assign36240_e50989_d_n0, assign36240_e50989_d_n2, assign36240_e50989_d_n6, assign36240_e50989_d_n7, assign36240_e50989_d_n10, assign36240_e50989_d_n11, assign36240_e50989_d_n12, assign36240_e50989_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36240_e50983: f64 = (var_t1 * var_xov__blk1198);
        let assign36240_e50985: f64 = (assign36240_e50983 * var_mu__blk1197);
        let assign36240_e50987: f64 = (assign36240_e50985 * var_nover__blk1192);
        (assign36240_e50987, ((((var_t1_dn0 * var_xov__blk1198) * var_mu__blk1197) + (assign36240_e50983 * var_mu__blk1197_dn0)) * var_nover__blk1192), ((((var_t1_dn2 * var_xov__blk1198) * var_mu__blk1197) + (assign36240_e50983 * var_mu__blk1197_dn2)) * var_nover__blk1192), ((((var_t1_dn6 * var_xov__blk1198) * var_mu__blk1197) + (assign36240_e50983 * var_mu__blk1197_dn6)) * var_nover__blk1192), ((((var_t1_dn7 * var_xov__blk1198) * var_mu__blk1197) + (assign36240_e50983 * var_mu__blk1197_dn7)) * var_nover__blk1192), ((((var_t1_dn10 * var_xov__blk1198) * var_mu__blk1197) + (assign36240_e50983 * var_mu__blk1197_dn10)) * var_nover__blk1192), ((((var_t1_dn11 * var_xov__blk1198) * var_mu__blk1197) + (assign36240_e50983 * var_mu__blk1197_dn11)) * var_nover__blk1192), ((((var_t1_dn12 * var_xov__blk1198) * var_mu__blk1197) + (assign36240_e50983 * var_mu__blk1197_dn12)) * var_nover__blk1192), ((((var_t1_dn17 * var_xov__blk1198) * var_mu__blk1197) + (assign36240_e50983 * var_mu__blk1197_dn17)) * var_nover__blk1192),)
    } else {
        (var_gd__blk1199, var_gd__blk1199_dn0, var_gd__blk1199_dn2, var_gd__blk1199_dn6, var_gd__blk1199_dn7, var_gd__blk1199_dn10, var_gd__blk1199_dn11, var_gd__blk1199_dn12, var_gd__blk1199_dn17,)
    }
};
        var_gd__blk1199 = assign36240_e50989;
        var_gd__blk1199_dn0 = assign36240_e50989_d_n0;
        var_gd__blk1199_dn2 = assign36240_e50989_d_n2;
        var_gd__blk1199_dn6 = assign36240_e50989_d_n6;
        var_gd__blk1199_dn7 = assign36240_e50989_d_n7;
        var_gd__blk1199_dn10 = assign36240_e50989_d_n10;
        var_gd__blk1199_dn11 = assign36240_e50989_d_n11;
        var_gd__blk1199_dn12 = assign36240_e50989_d_n12;
        var_gd__blk1199_dn17 = assign36240_e50989_d_n17;

        let assign36250_e50992: f64 = if var_gd__blk1199 <= 0.0 { 1.0 } else { 0.0 };
        var_guard1207 = assign36250_e50992;

        let (assign36260_e50998, assign36260_e50998_d_n0, assign36260_e50998_d_n2, assign36260_e50998_d_n6, assign36260_e50998_d_n7, assign36260_e50998_d_n10, assign36260_e50998_d_n11, assign36260_e50998_d_n12, assign36260_e50998_d_n17,) = {
    if ((var_guard1181 != 0.0) && (var_guard1207 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gd__blk1199, var_gd__blk1199_dn0, var_gd__blk1199_dn2, var_gd__blk1199_dn6, var_gd__blk1199_dn7, var_gd__blk1199_dn10, var_gd__blk1199_dn11, var_gd__blk1199_dn12, var_gd__blk1199_dn17,)
    }
};
        var_gd__blk1199 = assign36260_e50998;
        var_gd__blk1199_dn0 = assign36260_e50998_d_n0;
        var_gd__blk1199_dn2 = assign36260_e50998_d_n2;
        var_gd__blk1199_dn6 = assign36260_e50998_d_n6;
        var_gd__blk1199_dn7 = assign36260_e50998_d_n7;
        var_gd__blk1199_dn10 = assign36260_e50998_d_n10;
        var_gd__blk1199_dn11 = assign36260_e50998_d_n11;
        var_gd__blk1199_dn12 = assign36260_e50998_d_n12;
        var_gd__blk1199_dn17 = assign36260_e50998_d_n17;

        let (assign36270_e51004, assign36270_e51004_d_n0, assign36270_e51004_d_n2, assign36270_e51004_d_n6, assign36270_e51004_d_n7, assign36270_e51004_d_n10, assign36270_e51004_d_n11, assign36270_e51004_d_n12, assign36270_e51004_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36270_e51002: f64 = (1.0 / var_gd__blk1199);
        (assign36270_e51002, (-(var_gd__blk1199_dn0 / (var_gd__blk1199 * var_gd__blk1199))), (-(var_gd__blk1199_dn2 / (var_gd__blk1199 * var_gd__blk1199))), (-(var_gd__blk1199_dn6 / (var_gd__blk1199 * var_gd__blk1199))), (-(var_gd__blk1199_dn7 / (var_gd__blk1199 * var_gd__blk1199))), (-(var_gd__blk1199_dn10 / (var_gd__blk1199 * var_gd__blk1199))), (-(var_gd__blk1199_dn11 / (var_gd__blk1199 * var_gd__blk1199))), (-(var_gd__blk1199_dn12 / (var_gd__blk1199 * var_gd__blk1199))), (-(var_gd__blk1199_dn17 / (var_gd__blk1199 * var_gd__blk1199))),)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign36270_e51004;
        var_rsd_dn0 = assign36270_e51004_d_n0;
        var_rsd_dn2 = assign36270_e51004_d_n2;
        var_rsd_dn6 = assign36270_e51004_d_n6;
        var_rsd_dn7 = assign36270_e51004_d_n7;
        var_rsd_dn10 = assign36270_e51004_d_n10;
        var_rsd_dn11 = assign36270_e51004_d_n11;
        var_rsd_dn12 = assign36270_e51004_d_n12;
        var_rsd_dn17 = assign36270_e51004_d_n17;

        let (assign36280_e51010, assign36280_e51010_d_n0, assign36280_e51010_d_n2, assign36280_e51010_d_n6, assign36280_e51010_d_n7, assign36280_e51010_d_n10, assign36280_e51010_d_n11, assign36280_e51010_d_n12, assign36280_e51010_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36280_e51008: f64 = (var_rsd / var_weff_nf__blk1200);
        (assign36280_e51008, (var_rsd_dn0 / var_weff_nf__blk1200), (var_rsd_dn2 / var_weff_nf__blk1200), (var_rsd_dn6 / var_weff_nf__blk1200), (var_rsd_dn7 / var_weff_nf__blk1200), (var_rsd_dn10 / var_weff_nf__blk1200), (var_rsd_dn11 / var_weff_nf__blk1200), (var_rsd_dn12 / var_weff_nf__blk1200), (var_rsd_dn17 / var_weff_nf__blk1200),)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign36280_e51010;
        var_rsd_dn0 = assign36280_e51010_d_n0;
        var_rsd_dn2 = assign36280_e51010_d_n2;
        var_rsd_dn6 = assign36280_e51010_d_n6;
        var_rsd_dn7 = assign36280_e51010_d_n7;
        var_rsd_dn10 = assign36280_e51010_d_n10;
        var_rsd_dn11 = assign36280_e51010_d_n11;
        var_rsd_dn12 = assign36280_e51010_d_n12;
        var_rsd_dn17 = assign36280_e51010_d_n17;

        let (assign36290_e51016, assign36290_e51016_d_n0, assign36290_e51016_d_n2, assign36290_e51016_d_n6, assign36290_e51016_d_n7, assign36290_e51016_d_n10, assign36290_e51016_d_n11, assign36290_e51016_d_n12, assign36290_e51016_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36290_e51014: f64 = (var_rsd + var_rsd0__blk1188);
        (assign36290_e51014, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign36290_e51016;
        var_rsd_dn0 = assign36290_e51016_d_n0;
        var_rsd_dn2 = assign36290_e51016_d_n2;
        var_rsd_dn6 = assign36290_e51016_d_n6;
        var_rsd_dn7 = assign36290_e51016_d_n7;
        var_rsd_dn10 = assign36290_e51016_d_n10;
        var_rsd_dn11 = assign36290_e51016_d_n11;
        var_rsd_dn12 = assign36290_e51016_d_n12;
        var_rsd_dn17 = assign36290_e51016_d_n17;

        let assign36310_e51034: f64 = if var_rsd < 0.0001 { 1.0 } else { 0.0 };
        var_guard1208 = assign36310_e51034;

        let (assign36320_e51040, assign36320_e51040_d_n0, assign36320_e51040_d_n2, assign36320_e51040_d_n6, assign36320_e51040_d_n7, assign36320_e51040_d_n10, assign36320_e51040_d_n11, assign36320_e51040_d_n12, assign36320_e51040_d_n17,) = {
    if ((var_guard1181 != 0.0) && (var_guard1208 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn6, var_rsd_dn7, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12, var_rsd_dn17,)
    }
};
        var_rsd = assign36320_e51040;
        var_rsd_dn0 = assign36320_e51040_d_n0;
        var_rsd_dn2 = assign36320_e51040_d_n2;
        var_rsd_dn6 = assign36320_e51040_d_n6;
        var_rsd_dn7 = assign36320_e51040_d_n7;
        var_rsd_dn10 = assign36320_e51040_d_n10;
        var_rsd_dn11 = assign36320_e51040_d_n11;
        var_rsd_dn12 = assign36320_e51040_d_n12;
        var_rsd_dn17 = assign36320_e51040_d_n17;

        let (assign36330_e51046, assign36330_e51046_d_n0, assign36330_e51046_d_n2, assign36330_e51046_d_n6, assign36330_e51046_d_n7, assign36330_e51046_d_n10, assign36330_e51046_d_n11, assign36330_e51046_d_n12, assign36330_e51046_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36330_e51044: f64 = (var_rsd / var_mfactor);
        (assign36330_e51044, (var_rsd_dn0 / var_mfactor), (var_rsd_dn2 / var_mfactor), (var_rsd_dn6 / var_mfactor), (var_rsd_dn7 / var_mfactor), (var_rsd_dn10 / var_mfactor), (var_rsd_dn11 / var_mfactor), (var_rsd_dn12 / var_mfactor), (var_rsd_dn17 / var_mfactor),)
    } else {
        (var_rdde, var_rdde_dn0, var_rdde_dn2, var_rdde_dn6, var_rdde_dn7, var_rdde_dn10, var_rdde_dn11, var_rdde_dn12, var_rdde_dn17,)
    }
};
        var_rdde = assign36330_e51046;
        var_rdde_dn0 = assign36330_e51046_d_n0;
        var_rdde_dn2 = assign36330_e51046_d_n2;
        var_rdde_dn6 = assign36330_e51046_d_n6;
        var_rdde_dn7 = assign36330_e51046_d_n7;
        var_rdde_dn10 = assign36330_e51046_d_n10;
        var_rdde_dn11 = assign36330_e51046_d_n11;
        var_rdde_dn12 = assign36330_e51046_d_n12;
        var_rdde_dn17 = assign36330_e51046_d_n17;

        let assign36350_e51053: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1209 = assign36350_e51053;

        let assign36360_e51057: f64 = (1e-15 / 0.0001);
        let assign36360_e51058: f64 = if var_tau < assign36360_e51057 { 1.0 } else { 0.0 };
        var_guard1210 = assign36360_e51058;

        let (assign36370_e51068, assign36370_e51068_d_n0, assign36370_e51068_d_n2, assign36370_e51068_d_n6, assign36370_e51068_d_n7, assign36370_e51068_d_n10, assign36370_e51068_d_n11, assign36370_e51068_d_n12, assign36370_e51068_d_n17,) = {
    if (((var_guard1209 != 0.0) && (var_flg_nqs != 0.0)) && (var_guard1210 != 0.0)) {
        let assign36370_e51066: f64 = (1e-15 / 0.0001);
        (assign36370_e51066, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn6, var_tau_dn7, var_tau_dn10, var_tau_dn11, var_tau_dn12, var_tau_dn17,)
    }
};
        var_tau = assign36370_e51068;
        var_tau_dn0 = assign36370_e51068_d_n0;
        var_tau_dn2 = assign36370_e51068_d_n2;
        var_tau_dn6 = assign36370_e51068_d_n6;
        var_tau_dn7 = assign36370_e51068_d_n7;
        var_tau_dn10 = assign36370_e51068_d_n10;
        var_tau_dn11 = assign36370_e51068_d_n11;
        var_tau_dn12 = assign36370_e51068_d_n12;
        var_tau_dn17 = assign36370_e51068_d_n17;

        let assign36380_e51072: f64 = (1e-15 / 0.0001);
        let assign36380_e51073: f64 = if var_taub < assign36380_e51072 { 1.0 } else { 0.0 };
        var_guard1211 = assign36380_e51073;

        let (assign36390_e51083, assign36390_e51083_d_n0, assign36390_e51083_d_n2, assign36390_e51083_d_n6, assign36390_e51083_d_n7, assign36390_e51083_d_n10, assign36390_e51083_d_n11, assign36390_e51083_d_n12, assign36390_e51083_d_n17,) = {
    if (((var_guard1209 != 0.0) && (var_flg_nqs != 0.0)) && (var_guard1211 != 0.0)) {
        let assign36390_e51081: f64 = (1e-15 / 0.0001);
        (assign36390_e51081, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taub, var_taub_dn0, var_taub_dn2, var_taub_dn6, var_taub_dn7, var_taub_dn10, var_taub_dn11, var_taub_dn12, var_taub_dn17,)
    }
};
        var_taub = assign36390_e51083;
        var_taub_dn0 = assign36390_e51083_d_n0;
        var_taub_dn2 = assign36390_e51083_d_n2;
        var_taub_dn6 = assign36390_e51083_d_n6;
        var_taub_dn7 = assign36390_e51083_d_n7;
        var_taub_dn10 = assign36390_e51083_d_n10;
        var_taub_dn11 = assign36390_e51083_d_n11;
        var_taub_dn12 = assign36390_e51083_d_n12;
        var_taub_dn17 = assign36390_e51083_d_n17;

        let (assign36400_e51096, assign36400_e51096_d_n0, assign36400_e51096_d_n2, assign36400_e51096_d_n6, assign36400_e51096_d_n7, assign36400_e51096_d_n10, assign36400_e51096_d_n11, assign36400_e51096_d_n12, assign36400_e51096_d_n17,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs != 0.0)) {
        let (assign36400_e51094, assign36400_e51094_d_n0, assign36400_e51094_d_n2, assign36400_e51094_d_n6, assign36400_e51094_d_n7, assign36400_e51094_d_n10, assign36400_e51094_d_n11, assign36400_e51094_d_n12, assign36400_e51094_d_n17,) = {
            if (var_mode == 1.0) {
                (var_xd, var_xd_dn0, var_xd_dn2, var_xd_dn6, var_xd_dn7, var_xd_dn10, var_xd_dn11, var_xd_dn12, var_xd_dn17,)
            } else {
                let assign36400_e51093: f64 = (1.0 - var_xd);
                (assign36400_e51093, (-var_xd_dn0), (-var_xd_dn2), (-var_xd_dn6), (-var_xd_dn7), (-var_xd_dn10), (-var_xd_dn11), (-var_xd_dn12), (-var_xd_dn17),)
            }
        };
        (assign36400_e51094, assign36400_e51094_d_n0, assign36400_e51094_d_n2, assign36400_e51094_d_n6, assign36400_e51094_d_n7, assign36400_e51094_d_n10, assign36400_e51094_d_n11, assign36400_e51094_d_n12, assign36400_e51094_d_n17,)
    } else {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn10, var_qdrat_dn11, var_qdrat_dn12, var_qdrat_dn17,)
    }
};
        var_qdrat = assign36400_e51096;
        var_qdrat_dn0 = assign36400_e51096_d_n0;
        var_qdrat_dn2 = assign36400_e51096_d_n2;
        var_qdrat_dn6 = assign36400_e51096_d_n6;
        var_qdrat_dn7 = assign36400_e51096_d_n7;
        var_qdrat_dn10 = assign36400_e51096_d_n10;
        var_qdrat_dn11 = assign36400_e51096_d_n11;
        var_qdrat_dn12 = assign36400_e51096_d_n12;
        var_qdrat_dn17 = assign36400_e51096_d_n17;

        let (assign36410_e51106, assign36410_e51106_d_n0, assign36410_e51106_d_n2, assign36410_e51106_d_n6, assign36410_e51106_d_n7, assign36410_e51106_d_n10, assign36410_e51106_d_n11, assign36410_e51106_d_n12, assign36410_e51106_d_n17, assign36410_e51106_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36410_e51102: f64 = (var_qi_nqs - var_qi_qs);
        let assign36410_e51104: f64 = (assign36410_e51102 / var_tau);
        (assign36410_e51104, ((((-var_qi_qs_dn0) * var_tau) - (assign36410_e51102 * var_tau_dn0)) / (var_tau * var_tau)), ((((-var_qi_qs_dn2) * var_tau) - (assign36410_e51102 * var_tau_dn2)) / (var_tau * var_tau)), ((((-var_qi_qs_dn6) * var_tau) - (assign36410_e51102 * var_tau_dn6)) / (var_tau * var_tau)), ((((-var_qi_qs_dn7) * var_tau) - (assign36410_e51102 * var_tau_dn7)) / (var_tau * var_tau)), ((((-var_qi_qs_dn10) * var_tau) - (assign36410_e51102 * var_tau_dn10)) / (var_tau * var_tau)), ((((-var_qi_qs_dn11) * var_tau) - (assign36410_e51102 * var_tau_dn11)) / (var_tau * var_tau)), ((((-var_qi_qs_dn12) * var_tau) - (assign36410_e51102 * var_tau_dn12)) / (var_tau * var_tau)), ((((-var_qi_qs_dn17) * var_tau) - (assign36410_e51102 * var_tau_dn17)) / (var_tau * var_tau)), (var_qi_nqs_dn18 / var_tau),)
    } else {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn6, var_iqi_nqs_dn7, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12, var_iqi_nqs_dn17, var_iqi_nqs_dn18,)
    }
};
        var_iqi_nqs = assign36410_e51106;
        var_iqi_nqs_dn0 = assign36410_e51106_d_n0;
        var_iqi_nqs_dn2 = assign36410_e51106_d_n2;
        var_iqi_nqs_dn6 = assign36410_e51106_d_n6;
        var_iqi_nqs_dn7 = assign36410_e51106_d_n7;
        var_iqi_nqs_dn10 = assign36410_e51106_d_n10;
        var_iqi_nqs_dn11 = assign36410_e51106_d_n11;
        var_iqi_nqs_dn12 = assign36410_e51106_d_n12;
        var_iqi_nqs_dn17 = assign36410_e51106_d_n17;
        var_iqi_nqs_dn18 = assign36410_e51106_d_n18;

        *var_gd__blk1199_slot = var_gd__blk1199;
        *var_gd__blk1199_dn0_slot = var_gd__blk1199_dn0;
        *var_gd__blk1199_dn10_slot = var_gd__blk1199_dn10;
        *var_gd__blk1199_dn11_slot = var_gd__blk1199_dn11;
        *var_gd__blk1199_dn12_slot = var_gd__blk1199_dn12;
        *var_gd__blk1199_dn17_slot = var_gd__blk1199_dn17;
        *var_gd__blk1199_dn2_slot = var_gd__blk1199_dn2;
        *var_gd__blk1199_dn6_slot = var_gd__blk1199_dn6;
        *var_gd__blk1199_dn7_slot = var_gd__blk1199_dn7;
        *var_guard1202_slot = var_guard1202;
        *var_guard1203_slot = var_guard1203;
        *var_guard1204_slot = var_guard1204;
        *var_guard1205_slot = var_guard1205;
        *var_guard1206_slot = var_guard1206;
        *var_guard1207_slot = var_guard1207;
        *var_guard1208_slot = var_guard1208;
        *var_guard1209_slot = var_guard1209;
        *var_guard1210_slot = var_guard1210;
        *var_guard1211_slot = var_guard1211;
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
        *var_mu__blk1197_slot = var_mu__blk1197;
        *var_mu__blk1197_dn0_slot = var_mu__blk1197_dn0;
        *var_mu__blk1197_dn10_slot = var_mu__blk1197_dn10;
        *var_mu__blk1197_dn11_slot = var_mu__blk1197_dn11;
        *var_mu__blk1197_dn12_slot = var_mu__blk1197_dn12;
        *var_mu__blk1197_dn17_slot = var_mu__blk1197_dn17;
        *var_mu__blk1197_dn2_slot = var_mu__blk1197_dn2;
        *var_mu__blk1197_dn6_slot = var_mu__blk1197_dn6;
        *var_mu__blk1197_dn7_slot = var_mu__blk1197_dn7;
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
        *var_vdri__blk1196_slot = var_vdri__blk1196;
        *var_vdri__blk1196_dn0_slot = var_vdri__blk1196_dn0;
        *var_vdri__blk1196_dn10_slot = var_vdri__blk1196_dn10;
        *var_vdri__blk1196_dn11_slot = var_vdri__blk1196_dn11;
        *var_vdri__blk1196_dn12_slot = var_vdri__blk1196_dn12;
        *var_vdri__blk1196_dn17_slot = var_vdri__blk1196_dn17;
        *var_vdri__blk1196_dn2_slot = var_vdri__blk1196_dn2;
        *var_vdri__blk1196_dn6_slot = var_vdri__blk1196_dn6;
        *var_vdri__blk1196_dn7_slot = var_vdri__blk1196_dn7;
    }

    pub(super) fn stamp_transient_block_126(
        var_flg_nqs: f64,
        var_guard1209: f64,
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
        var_guard1212_slot: &mut f64,
        var_guard1213_slot: &mut f64,
        var_guard1214_slot: &mut f64,
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
        let mut var_guard1212: f64 = *var_guard1212_slot;
        let mut var_guard1213: f64 = *var_guard1213_slot;
        let mut var_guard1214: f64 = *var_guard1214_slot;
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

        let (assign36420_e51116, assign36420_e51116_d_n0, assign36420_e51116_d_n2, assign36420_e51116_d_n6, assign36420_e51116_d_n7, assign36420_e51116_d_n10, assign36420_e51116_d_n11, assign36420_e51116_d_n12, assign36420_e51116_d_n13, assign36420_e51116_d_n15, assign36420_e51116_d_n16, assign36420_e51116_d_n17, assign36420_e51116_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36420_e51112: f64 = (var_qb_nqs - var_qb_qs);
        let assign36420_e51114: f64 = (assign36420_e51112 / var_taub);
        (assign36420_e51114, ((((-var_qb_qs_dn0) * var_taub) - (assign36420_e51112 * var_taub_dn0)) / (var_taub * var_taub)), ((((-var_qb_qs_dn2) * var_taub) - (assign36420_e51112 * var_taub_dn2)) / (var_taub * var_taub)), ((((-var_qb_qs_dn6) * var_taub) - (assign36420_e51112 * var_taub_dn6)) / (var_taub * var_taub)), ((((-var_qb_qs_dn7) * var_taub) - (assign36420_e51112 * var_taub_dn7)) / (var_taub * var_taub)), ((((-var_qb_qs_dn10) * var_taub) - (assign36420_e51112 * var_taub_dn10)) / (var_taub * var_taub)), ((((-var_qb_qs_dn11) * var_taub) - (assign36420_e51112 * var_taub_dn11)) / (var_taub * var_taub)), ((((-var_qb_qs_dn12) * var_taub) - (assign36420_e51112 * var_taub_dn12)) / (var_taub * var_taub)), ((var_qb_nqs_dn13 - var_qb_qs_dn13) / var_taub), ((-var_qb_qs_dn15) / var_taub), ((-var_qb_qs_dn16) / var_taub), ((((-var_qb_qs_dn17) * var_taub) - (assign36420_e51112 * var_taub_dn17)) / (var_taub * var_taub)), ((-var_qb_qs_dn18) / var_taub),)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    }
};
        var_iqb_nqs = assign36420_e51116;
        var_iqb_nqs_dn0 = assign36420_e51116_d_n0;
        var_iqb_nqs_dn2 = assign36420_e51116_d_n2;
        var_iqb_nqs_dn6 = assign36420_e51116_d_n6;
        var_iqb_nqs_dn7 = assign36420_e51116_d_n7;
        var_iqb_nqs_dn10 = assign36420_e51116_d_n10;
        var_iqb_nqs_dn11 = assign36420_e51116_d_n11;
        var_iqb_nqs_dn12 = assign36420_e51116_d_n12;
        var_iqb_nqs_dn13 = assign36420_e51116_d_n13;
        var_iqb_nqs_dn15 = assign36420_e51116_d_n15;
        var_iqb_nqs_dn16 = assign36420_e51116_d_n16;
        var_iqb_nqs_dn17 = assign36420_e51116_d_n17;
        var_iqb_nqs_dn18 = assign36420_e51116_d_n18;

        let (assign36430_e51126, assign36430_e51126_d_n0, assign36430_e51126_d_n2, assign36430_e51126_d_n6, assign36430_e51126_d_n7, assign36430_e51126_d_n10, assign36430_e51126_d_n11, assign36430_e51126_d_n12, assign36430_e51126_d_n15, assign36430_e51126_d_n17, assign36430_e51126_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36430_e51122: f64 = (var_qi_nqs * var_qdrat);
        let assign36430_e51124: f64 = (assign36430_e51122 + var_q_bt_se);
        (assign36430_e51124, ((var_qi_nqs * var_qdrat_dn0) + var_q_bt_se_dn0), ((var_qi_nqs * var_qdrat_dn2) + var_q_bt_se_dn2), ((var_qi_nqs * var_qdrat_dn6) + var_q_bt_se_dn6), ((var_qi_nqs * var_qdrat_dn7) + var_q_bt_se_dn7), ((var_qi_nqs * var_qdrat_dn10) + var_q_bt_se_dn10), ((var_qi_nqs * var_qdrat_dn11) + var_q_bt_se_dn11), ((var_qi_nqs * var_qdrat_dn12) + var_q_bt_se_dn12), 0.0, ((var_qi_nqs * var_qdrat_dn17) + var_q_bt_se_dn17), (var_qi_nqs_dn18 * var_qdrat),)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign36430_e51126;
        var_qd_nqs_dn0 = assign36430_e51126_d_n0;
        var_qd_nqs_dn2 = assign36430_e51126_d_n2;
        var_qd_nqs_dn6 = assign36430_e51126_d_n6;
        var_qd_nqs_dn7 = assign36430_e51126_d_n7;
        var_qd_nqs_dn10 = assign36430_e51126_d_n10;
        var_qd_nqs_dn11 = assign36430_e51126_d_n11;
        var_qd_nqs_dn12 = assign36430_e51126_d_n12;
        var_qd_nqs_dn15 = assign36430_e51126_d_n15;
        var_qd_nqs_dn17 = assign36430_e51126_d_n17;
        var_qd_nqs_dn18 = assign36430_e51126_d_n18;

        let (assign36440_e51138, assign36440_e51138_d_n0, assign36440_e51138_d_n2, assign36440_e51138_d_n6, assign36440_e51138_d_n7, assign36440_e51138_d_n10, assign36440_e51138_d_n11, assign36440_e51138_d_n12, assign36440_e51138_d_n16, assign36440_e51138_d_n17, assign36440_e51138_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36440_e51133: f64 = (1.0 - var_qdrat);
        let assign36440_e51134: f64 = (var_qi_nqs * assign36440_e51133);
        let assign36440_e51136: f64 = (assign36440_e51134 + var_q_bt_se);
        (assign36440_e51136, ((var_qi_nqs * (-var_qdrat_dn0)) + var_q_bt_se_dn0), ((var_qi_nqs * (-var_qdrat_dn2)) + var_q_bt_se_dn2), ((var_qi_nqs * (-var_qdrat_dn6)) + var_q_bt_se_dn6), ((var_qi_nqs * (-var_qdrat_dn7)) + var_q_bt_se_dn7), ((var_qi_nqs * (-var_qdrat_dn10)) + var_q_bt_se_dn10), ((var_qi_nqs * (-var_qdrat_dn11)) + var_q_bt_se_dn11), ((var_qi_nqs * (-var_qdrat_dn12)) + var_q_bt_se_dn12), 0.0, ((var_qi_nqs * (-var_qdrat_dn17)) + var_q_bt_se_dn17), (var_qi_nqs_dn18 * assign36440_e51133),)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign36440_e51138;
        var_qs_nqs_dn0 = assign36440_e51138_d_n0;
        var_qs_nqs_dn2 = assign36440_e51138_d_n2;
        var_qs_nqs_dn6 = assign36440_e51138_d_n6;
        var_qs_nqs_dn7 = assign36440_e51138_d_n7;
        var_qs_nqs_dn10 = assign36440_e51138_d_n10;
        var_qs_nqs_dn11 = assign36440_e51138_d_n11;
        var_qs_nqs_dn12 = assign36440_e51138_d_n12;
        var_qs_nqs_dn16 = assign36440_e51138_d_n16;
        var_qs_nqs_dn17 = assign36440_e51138_d_n17;
        var_qs_nqs_dn18 = assign36440_e51138_d_n18;

        let (assign36450_e51149, assign36450_e51149_d_n0, assign36450_e51149_d_n2, assign36450_e51149_d_n6, assign36450_e51149_d_n7, assign36450_e51149_d_n10, assign36450_e51149_d_n11, assign36450_e51149_d_n12, assign36450_e51149_d_n13, assign36450_e51149_d_n15, assign36450_e51149_d_n16, assign36450_e51149_d_n17, assign36450_e51149_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36450_e51143: f64 = (-var_qi_nqs);
        let assign36450_e51145: f64 = (assign36450_e51143 - var_qb_nqs);
        let assign36450_e51147: f64 = (assign36450_e51145 + var_q_bt_ge);
        (assign36450_e51147, var_q_bt_ge_dn0, var_q_bt_ge_dn2, var_q_bt_ge_dn6, var_q_bt_ge_dn7, var_q_bt_ge_dn10, var_q_bt_ge_dn11, var_q_bt_ge_dn12, (-var_qb_nqs_dn13), 0.0, 0.0, var_q_bt_ge_dn17, (-var_qi_nqs_dn18),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36450_e51149;
        var_qg_nqs_dn0 = assign36450_e51149_d_n0;
        var_qg_nqs_dn2 = assign36450_e51149_d_n2;
        var_qg_nqs_dn6 = assign36450_e51149_d_n6;
        var_qg_nqs_dn7 = assign36450_e51149_d_n7;
        var_qg_nqs_dn10 = assign36450_e51149_d_n10;
        var_qg_nqs_dn11 = assign36450_e51149_d_n11;
        var_qg_nqs_dn12 = assign36450_e51149_d_n12;
        var_qg_nqs_dn13 = assign36450_e51149_d_n13;
        var_qg_nqs_dn15 = assign36450_e51149_d_n15;
        var_qg_nqs_dn16 = assign36450_e51149_d_n16;
        var_qg_nqs_dn17 = assign36450_e51149_d_n17;
        var_qg_nqs_dn18 = assign36450_e51149_d_n18;

        let (assign36460_e51156, assign36460_e51156_d_n0, assign36460_e51156_d_n2, assign36460_e51156_d_n6, assign36460_e51156_d_n7, assign36460_e51156_d_n10, assign36460_e51156_d_n11, assign36460_e51156_d_n12, assign36460_e51156_d_n17, assign36460_e51156_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn6, var_iqi_nqs_dn7, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12, var_iqi_nqs_dn17, var_iqi_nqs_dn18,)
    }
};
        var_iqi_nqs = assign36460_e51156;
        var_iqi_nqs_dn0 = assign36460_e51156_d_n0;
        var_iqi_nqs_dn2 = assign36460_e51156_d_n2;
        var_iqi_nqs_dn6 = assign36460_e51156_d_n6;
        var_iqi_nqs_dn7 = assign36460_e51156_d_n7;
        var_iqi_nqs_dn10 = assign36460_e51156_d_n10;
        var_iqi_nqs_dn11 = assign36460_e51156_d_n11;
        var_iqi_nqs_dn12 = assign36460_e51156_d_n12;
        var_iqi_nqs_dn17 = assign36460_e51156_d_n17;
        var_iqi_nqs_dn18 = assign36460_e51156_d_n18;

        let (assign36470_e51163, assign36470_e51163_d_n0, assign36470_e51163_d_n2, assign36470_e51163_d_n6, assign36470_e51163_d_n7, assign36470_e51163_d_n10, assign36470_e51163_d_n11, assign36470_e51163_d_n12, assign36470_e51163_d_n13, assign36470_e51163_d_n15, assign36470_e51163_d_n16, assign36470_e51163_d_n17, assign36470_e51163_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    }
};
        var_iqb_nqs = assign36470_e51163;
        var_iqb_nqs_dn0 = assign36470_e51163_d_n0;
        var_iqb_nqs_dn2 = assign36470_e51163_d_n2;
        var_iqb_nqs_dn6 = assign36470_e51163_d_n6;
        var_iqb_nqs_dn7 = assign36470_e51163_d_n7;
        var_iqb_nqs_dn10 = assign36470_e51163_d_n10;
        var_iqb_nqs_dn11 = assign36470_e51163_d_n11;
        var_iqb_nqs_dn12 = assign36470_e51163_d_n12;
        var_iqb_nqs_dn13 = assign36470_e51163_d_n13;
        var_iqb_nqs_dn15 = assign36470_e51163_d_n15;
        var_iqb_nqs_dn16 = assign36470_e51163_d_n16;
        var_iqb_nqs_dn17 = assign36470_e51163_d_n17;
        var_iqb_nqs_dn18 = assign36470_e51163_d_n18;

        let (assign36480_e51170, assign36480_e51170_d_n0, assign36480_e51170_d_n2, assign36480_e51170_d_n6, assign36480_e51170_d_n7, assign36480_e51170_d_n10, assign36480_e51170_d_n11, assign36480_e51170_d_n12, assign36480_e51170_d_n15, assign36480_e51170_d_n17, assign36480_e51170_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign36480_e51170;
        var_qd_nqs_dn0 = assign36480_e51170_d_n0;
        var_qd_nqs_dn2 = assign36480_e51170_d_n2;
        var_qd_nqs_dn6 = assign36480_e51170_d_n6;
        var_qd_nqs_dn7 = assign36480_e51170_d_n7;
        var_qd_nqs_dn10 = assign36480_e51170_d_n10;
        var_qd_nqs_dn11 = assign36480_e51170_d_n11;
        var_qd_nqs_dn12 = assign36480_e51170_d_n12;
        var_qd_nqs_dn15 = assign36480_e51170_d_n15;
        var_qd_nqs_dn17 = assign36480_e51170_d_n17;
        var_qd_nqs_dn18 = assign36480_e51170_d_n18;

        let (assign36490_e51177, assign36490_e51177_d_n0, assign36490_e51177_d_n2, assign36490_e51177_d_n6, assign36490_e51177_d_n7, assign36490_e51177_d_n10, assign36490_e51177_d_n11, assign36490_e51177_d_n12, assign36490_e51177_d_n16, assign36490_e51177_d_n17, assign36490_e51177_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign36490_e51177;
        var_qs_nqs_dn0 = assign36490_e51177_d_n0;
        var_qs_nqs_dn2 = assign36490_e51177_d_n2;
        var_qs_nqs_dn6 = assign36490_e51177_d_n6;
        var_qs_nqs_dn7 = assign36490_e51177_d_n7;
        var_qs_nqs_dn10 = assign36490_e51177_d_n10;
        var_qs_nqs_dn11 = assign36490_e51177_d_n11;
        var_qs_nqs_dn12 = assign36490_e51177_d_n12;
        var_qs_nqs_dn16 = assign36490_e51177_d_n16;
        var_qs_nqs_dn17 = assign36490_e51177_d_n17;
        var_qs_nqs_dn18 = assign36490_e51177_d_n18;

        let (assign36500_e51184, assign36500_e51184_d_n0, assign36500_e51184_d_n2, assign36500_e51184_d_n6, assign36500_e51184_d_n7, assign36500_e51184_d_n10, assign36500_e51184_d_n11, assign36500_e51184_d_n12, assign36500_e51184_d_n13, assign36500_e51184_d_n15, assign36500_e51184_d_n16, assign36500_e51184_d_n17, assign36500_e51184_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36500_e51184;
        var_qg_nqs_dn0 = assign36500_e51184_d_n0;
        var_qg_nqs_dn2 = assign36500_e51184_d_n2;
        var_qg_nqs_dn6 = assign36500_e51184_d_n6;
        var_qg_nqs_dn7 = assign36500_e51184_d_n7;
        var_qg_nqs_dn10 = assign36500_e51184_d_n10;
        var_qg_nqs_dn11 = assign36500_e51184_d_n11;
        var_qg_nqs_dn12 = assign36500_e51184_d_n12;
        var_qg_nqs_dn13 = assign36500_e51184_d_n13;
        var_qg_nqs_dn15 = assign36500_e51184_d_n15;
        var_qg_nqs_dn16 = assign36500_e51184_d_n16;
        var_qg_nqs_dn17 = assign36500_e51184_d_n17;
        var_qg_nqs_dn18 = assign36500_e51184_d_n18;

        let (assign36510_e51191, assign36510_e51191_d_n13,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign36510_e51191;
        var_qb_nqs_dn13 = assign36510_e51191_d_n13;

        let assign36520_e51195: f64 = (1e-15 / 0.0001);
        let assign36520_e51196: f64 = if var_tau < assign36520_e51195 { 1.0 } else { 0.0 };
        var_guard1212 = assign36520_e51196;

        let (assign36530_e51207, assign36530_e51207_d_n0, assign36530_e51207_d_n2, assign36530_e51207_d_n6, assign36530_e51207_d_n7, assign36530_e51207_d_n10, assign36530_e51207_d_n11, assign36530_e51207_d_n12, assign36530_e51207_d_n17,) = {
    if (((var_guard1209 == 0.0) && (var_flg_nqs != 0.0)) && (var_guard1212 != 0.0)) {
        let assign36530_e51205: f64 = (1e-15 / 0.0001);
        (assign36530_e51205, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn6, var_tau_dn7, var_tau_dn10, var_tau_dn11, var_tau_dn12, var_tau_dn17,)
    }
};
        var_tau = assign36530_e51207;
        var_tau_dn0 = assign36530_e51207_d_n0;
        var_tau_dn2 = assign36530_e51207_d_n2;
        var_tau_dn6 = assign36530_e51207_d_n6;
        var_tau_dn7 = assign36530_e51207_d_n7;
        var_tau_dn10 = assign36530_e51207_d_n10;
        var_tau_dn11 = assign36530_e51207_d_n11;
        var_tau_dn12 = assign36530_e51207_d_n12;
        var_tau_dn17 = assign36530_e51207_d_n17;

        let assign36540_e51211: f64 = (1e-15 / 0.0001);
        let assign36540_e51212: f64 = if var_taub < assign36540_e51211 { 1.0 } else { 0.0 };
        var_guard1213 = assign36540_e51212;

        let (assign36550_e51223, assign36550_e51223_d_n0, assign36550_e51223_d_n2, assign36550_e51223_d_n6, assign36550_e51223_d_n7, assign36550_e51223_d_n10, assign36550_e51223_d_n11, assign36550_e51223_d_n12, assign36550_e51223_d_n17,) = {
    if (((var_guard1209 == 0.0) && (var_flg_nqs != 0.0)) && (var_guard1213 != 0.0)) {
        let assign36550_e51221: f64 = (1e-15 / 0.0001);
        (assign36550_e51221, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taub, var_taub_dn0, var_taub_dn2, var_taub_dn6, var_taub_dn7, var_taub_dn10, var_taub_dn11, var_taub_dn12, var_taub_dn17,)
    }
};
        var_taub = assign36550_e51223;
        var_taub_dn0 = assign36550_e51223_d_n0;
        var_taub_dn2 = assign36550_e51223_d_n2;
        var_taub_dn6 = assign36550_e51223_d_n6;
        var_taub_dn7 = assign36550_e51223_d_n7;
        var_taub_dn10 = assign36550_e51223_d_n10;
        var_taub_dn11 = assign36550_e51223_d_n11;
        var_taub_dn12 = assign36550_e51223_d_n12;
        var_taub_dn17 = assign36550_e51223_d_n17;

        let (assign36560_e51234, assign36560_e51234_d_n0, assign36560_e51234_d_n2, assign36560_e51234_d_n6, assign36560_e51234_d_n7, assign36560_e51234_d_n10, assign36560_e51234_d_n11, assign36560_e51234_d_n12, assign36560_e51234_d_n13, assign36560_e51234_d_n15, assign36560_e51234_d_n16, assign36560_e51234_d_n17, assign36560_e51234_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign36560_e51230: f64 = (var_qd_nqs - var_qd_qs);
        let assign36560_e51232: f64 = (assign36560_e51230 / var_tau);
        (assign36560_e51232, ((((var_qd_nqs_dn0 - var_qd_qs_dn0) * var_tau) - (assign36560_e51230 * var_tau_dn0)) / (var_tau * var_tau)), ((((var_qd_nqs_dn2 - var_qd_qs_dn2) * var_tau) - (assign36560_e51230 * var_tau_dn2)) / (var_tau * var_tau)), ((((var_qd_nqs_dn6 - var_qd_qs_dn6) * var_tau) - (assign36560_e51230 * var_tau_dn6)) / (var_tau * var_tau)), ((((var_qd_nqs_dn7 - var_qd_qs_dn7) * var_tau) - (assign36560_e51230 * var_tau_dn7)) / (var_tau * var_tau)), ((((var_qd_nqs_dn10 - var_qd_qs_dn10) * var_tau) - (assign36560_e51230 * var_tau_dn10)) / (var_tau * var_tau)), ((((var_qd_nqs_dn11 - var_qd_qs_dn11) * var_tau) - (assign36560_e51230 * var_tau_dn11)) / (var_tau * var_tau)), ((((var_qd_nqs_dn12 - var_qd_qs_dn12) * var_tau) - (assign36560_e51230 * var_tau_dn12)) / (var_tau * var_tau)), ((-var_qd_qs_dn13) / var_tau), ((var_qd_nqs_dn15 - var_qd_qs_dn15) / var_tau), ((-var_qd_qs_dn16) / var_tau), ((((var_qd_nqs_dn17 - var_qd_qs_dn17) * var_tau) - (assign36560_e51230 * var_tau_dn17)) / (var_tau * var_tau)), ((var_qd_nqs_dn18 - var_qd_qs_dn18) / var_tau),)
    } else {
        (var_iqd_nqs, var_iqd_nqs_dn0, var_iqd_nqs_dn2, var_iqd_nqs_dn6, var_iqd_nqs_dn7, var_iqd_nqs_dn10, var_iqd_nqs_dn11, var_iqd_nqs_dn12, var_iqd_nqs_dn13, var_iqd_nqs_dn15, var_iqd_nqs_dn16, var_iqd_nqs_dn17, var_iqd_nqs_dn18,)
    }
};
        var_iqd_nqs = assign36560_e51234;
        var_iqd_nqs_dn0 = assign36560_e51234_d_n0;
        var_iqd_nqs_dn2 = assign36560_e51234_d_n2;
        var_iqd_nqs_dn6 = assign36560_e51234_d_n6;
        var_iqd_nqs_dn7 = assign36560_e51234_d_n7;
        var_iqd_nqs_dn10 = assign36560_e51234_d_n10;
        var_iqd_nqs_dn11 = assign36560_e51234_d_n11;
        var_iqd_nqs_dn12 = assign36560_e51234_d_n12;
        var_iqd_nqs_dn13 = assign36560_e51234_d_n13;
        var_iqd_nqs_dn15 = assign36560_e51234_d_n15;
        var_iqd_nqs_dn16 = assign36560_e51234_d_n16;
        var_iqd_nqs_dn17 = assign36560_e51234_d_n17;
        var_iqd_nqs_dn18 = assign36560_e51234_d_n18;

        let (assign36570_e51245, assign36570_e51245_d_n0, assign36570_e51245_d_n2, assign36570_e51245_d_n6, assign36570_e51245_d_n7, assign36570_e51245_d_n10, assign36570_e51245_d_n11, assign36570_e51245_d_n12, assign36570_e51245_d_n13, assign36570_e51245_d_n15, assign36570_e51245_d_n16, assign36570_e51245_d_n17, assign36570_e51245_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign36570_e51241: f64 = (var_qs_nqs - var_qs_qs);
        let assign36570_e51243: f64 = (assign36570_e51241 / var_tau);
        (assign36570_e51243, ((((var_qs_nqs_dn0 - var_qs_qs_dn0) * var_tau) - (assign36570_e51241 * var_tau_dn0)) / (var_tau * var_tau)), ((((var_qs_nqs_dn2 - var_qs_qs_dn2) * var_tau) - (assign36570_e51241 * var_tau_dn2)) / (var_tau * var_tau)), ((((var_qs_nqs_dn6 - var_qs_qs_dn6) * var_tau) - (assign36570_e51241 * var_tau_dn6)) / (var_tau * var_tau)), ((((var_qs_nqs_dn7 - var_qs_qs_dn7) * var_tau) - (assign36570_e51241 * var_tau_dn7)) / (var_tau * var_tau)), ((((var_qs_nqs_dn10 - var_qs_qs_dn10) * var_tau) - (assign36570_e51241 * var_tau_dn10)) / (var_tau * var_tau)), ((((var_qs_nqs_dn11 - var_qs_qs_dn11) * var_tau) - (assign36570_e51241 * var_tau_dn11)) / (var_tau * var_tau)), ((((var_qs_nqs_dn12 - var_qs_qs_dn12) * var_tau) - (assign36570_e51241 * var_tau_dn12)) / (var_tau * var_tau)), ((-var_qs_qs_dn13) / var_tau), ((-var_qs_qs_dn15) / var_tau), ((var_qs_nqs_dn16 - var_qs_qs_dn16) / var_tau), ((((var_qs_nqs_dn17 - var_qs_qs_dn17) * var_tau) - (assign36570_e51241 * var_tau_dn17)) / (var_tau * var_tau)), ((var_qs_nqs_dn18 - var_qs_qs_dn18) / var_tau),)
    } else {
        (var_iqs_nqs, var_iqs_nqs_dn0, var_iqs_nqs_dn2, var_iqs_nqs_dn6, var_iqs_nqs_dn7, var_iqs_nqs_dn10, var_iqs_nqs_dn11, var_iqs_nqs_dn12, var_iqs_nqs_dn13, var_iqs_nqs_dn15, var_iqs_nqs_dn16, var_iqs_nqs_dn17, var_iqs_nqs_dn18,)
    }
};
        var_iqs_nqs = assign36570_e51245;
        var_iqs_nqs_dn0 = assign36570_e51245_d_n0;
        var_iqs_nqs_dn2 = assign36570_e51245_d_n2;
        var_iqs_nqs_dn6 = assign36570_e51245_d_n6;
        var_iqs_nqs_dn7 = assign36570_e51245_d_n7;
        var_iqs_nqs_dn10 = assign36570_e51245_d_n10;
        var_iqs_nqs_dn11 = assign36570_e51245_d_n11;
        var_iqs_nqs_dn12 = assign36570_e51245_d_n12;
        var_iqs_nqs_dn13 = assign36570_e51245_d_n13;
        var_iqs_nqs_dn15 = assign36570_e51245_d_n15;
        var_iqs_nqs_dn16 = assign36570_e51245_d_n16;
        var_iqs_nqs_dn17 = assign36570_e51245_d_n17;
        var_iqs_nqs_dn18 = assign36570_e51245_d_n18;

        let (assign36580_e51256, assign36580_e51256_d_n0, assign36580_e51256_d_n2, assign36580_e51256_d_n6, assign36580_e51256_d_n7, assign36580_e51256_d_n10, assign36580_e51256_d_n11, assign36580_e51256_d_n12, assign36580_e51256_d_n13, assign36580_e51256_d_n15, assign36580_e51256_d_n16, assign36580_e51256_d_n17, assign36580_e51256_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign36580_e51252: f64 = (var_qb_nqs - var_qb_qs);
        let assign36580_e51254: f64 = (assign36580_e51252 / var_taub);
        (assign36580_e51254, ((((-var_qb_qs_dn0) * var_taub) - (assign36580_e51252 * var_taub_dn0)) / (var_taub * var_taub)), ((((-var_qb_qs_dn2) * var_taub) - (assign36580_e51252 * var_taub_dn2)) / (var_taub * var_taub)), ((((-var_qb_qs_dn6) * var_taub) - (assign36580_e51252 * var_taub_dn6)) / (var_taub * var_taub)), ((((-var_qb_qs_dn7) * var_taub) - (assign36580_e51252 * var_taub_dn7)) / (var_taub * var_taub)), ((((-var_qb_qs_dn10) * var_taub) - (assign36580_e51252 * var_taub_dn10)) / (var_taub * var_taub)), ((((-var_qb_qs_dn11) * var_taub) - (assign36580_e51252 * var_taub_dn11)) / (var_taub * var_taub)), ((((-var_qb_qs_dn12) * var_taub) - (assign36580_e51252 * var_taub_dn12)) / (var_taub * var_taub)), ((var_qb_nqs_dn13 - var_qb_qs_dn13) / var_taub), ((-var_qb_qs_dn15) / var_taub), ((-var_qb_qs_dn16) / var_taub), ((((-var_qb_qs_dn17) * var_taub) - (assign36580_e51252 * var_taub_dn17)) / (var_taub * var_taub)), ((-var_qb_qs_dn18) / var_taub),)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    }
};
        var_iqb_nqs = assign36580_e51256;
        var_iqb_nqs_dn0 = assign36580_e51256_d_n0;
        var_iqb_nqs_dn2 = assign36580_e51256_d_n2;
        var_iqb_nqs_dn6 = assign36580_e51256_d_n6;
        var_iqb_nqs_dn7 = assign36580_e51256_d_n7;
        var_iqb_nqs_dn10 = assign36580_e51256_d_n10;
        var_iqb_nqs_dn11 = assign36580_e51256_d_n11;
        var_iqb_nqs_dn12 = assign36580_e51256_d_n12;
        var_iqb_nqs_dn13 = assign36580_e51256_d_n13;
        var_iqb_nqs_dn15 = assign36580_e51256_d_n15;
        var_iqb_nqs_dn16 = assign36580_e51256_d_n16;
        var_iqb_nqs_dn17 = assign36580_e51256_d_n17;
        var_iqb_nqs_dn18 = assign36580_e51256_d_n18;

        let (assign36590_e51263, assign36590_e51263_d_n0, assign36590_e51263_d_n2, assign36590_e51263_d_n6, assign36590_e51263_d_n7, assign36590_e51263_d_n10, assign36590_e51263_d_n11, assign36590_e51263_d_n12, assign36590_e51263_d_n13, assign36590_e51263_d_n15, assign36590_e51263_d_n16, assign36590_e51263_d_n17, assign36590_e51263_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    }
};
        var_iqb_nqs = assign36590_e51263;
        var_iqb_nqs_dn0 = assign36590_e51263_d_n0;
        var_iqb_nqs_dn2 = assign36590_e51263_d_n2;
        var_iqb_nqs_dn6 = assign36590_e51263_d_n6;
        var_iqb_nqs_dn7 = assign36590_e51263_d_n7;
        var_iqb_nqs_dn10 = assign36590_e51263_d_n10;
        var_iqb_nqs_dn11 = assign36590_e51263_d_n11;
        var_iqb_nqs_dn12 = assign36590_e51263_d_n12;
        var_iqb_nqs_dn13 = assign36590_e51263_d_n13;
        var_iqb_nqs_dn15 = assign36590_e51263_d_n15;
        var_iqb_nqs_dn16 = assign36590_e51263_d_n16;
        var_iqb_nqs_dn17 = assign36590_e51263_d_n17;
        var_iqb_nqs_dn18 = assign36590_e51263_d_n18;

        let (assign36600_e51275, assign36600_e51275_d_n0, assign36600_e51275_d_n2, assign36600_e51275_d_n6, assign36600_e51275_d_n7, assign36600_e51275_d_n10, assign36600_e51275_d_n11, assign36600_e51275_d_n12, assign36600_e51275_d_n13, assign36600_e51275_d_n15, assign36600_e51275_d_n16, assign36600_e51275_d_n17, assign36600_e51275_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign36600_e51269: f64 = (-var_qd_nqs);
        let assign36600_e51271: f64 = (assign36600_e51269 - var_qs_nqs);
        let assign36600_e51273: f64 = (assign36600_e51271 - var_qb_nqs);
        (assign36600_e51273, ((-var_qd_nqs_dn0) - var_qs_nqs_dn0), ((-var_qd_nqs_dn2) - var_qs_nqs_dn2), ((-var_qd_nqs_dn6) - var_qs_nqs_dn6), ((-var_qd_nqs_dn7) - var_qs_nqs_dn7), ((-var_qd_nqs_dn10) - var_qs_nqs_dn10), ((-var_qd_nqs_dn11) - var_qs_nqs_dn11), ((-var_qd_nqs_dn12) - var_qs_nqs_dn12), (-var_qb_nqs_dn13), (-var_qd_nqs_dn15), (-var_qs_nqs_dn16), ((-var_qd_nqs_dn17) - var_qs_nqs_dn17), ((-var_qd_nqs_dn18) - var_qs_nqs_dn18),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36600_e51275;
        var_qg_nqs_dn0 = assign36600_e51275_d_n0;
        var_qg_nqs_dn2 = assign36600_e51275_d_n2;
        var_qg_nqs_dn6 = assign36600_e51275_d_n6;
        var_qg_nqs_dn7 = assign36600_e51275_d_n7;
        var_qg_nqs_dn10 = assign36600_e51275_d_n10;
        var_qg_nqs_dn11 = assign36600_e51275_d_n11;
        var_qg_nqs_dn12 = assign36600_e51275_d_n12;
        var_qg_nqs_dn13 = assign36600_e51275_d_n13;
        var_qg_nqs_dn15 = assign36600_e51275_d_n15;
        var_qg_nqs_dn16 = assign36600_e51275_d_n16;
        var_qg_nqs_dn17 = assign36600_e51275_d_n17;
        var_qg_nqs_dn18 = assign36600_e51275_d_n18;

        let (assign36610_e51283, assign36610_e51283_d_n0, assign36610_e51283_d_n2, assign36610_e51283_d_n6, assign36610_e51283_d_n7, assign36610_e51283_d_n10, assign36610_e51283_d_n11, assign36610_e51283_d_n12, assign36610_e51283_d_n13, assign36610_e51283_d_n15, assign36610_e51283_d_n16, assign36610_e51283_d_n17, assign36610_e51283_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqd_nqs, var_iqd_nqs_dn0, var_iqd_nqs_dn2, var_iqd_nqs_dn6, var_iqd_nqs_dn7, var_iqd_nqs_dn10, var_iqd_nqs_dn11, var_iqd_nqs_dn12, var_iqd_nqs_dn13, var_iqd_nqs_dn15, var_iqd_nqs_dn16, var_iqd_nqs_dn17, var_iqd_nqs_dn18,)
    }
};
        var_iqd_nqs = assign36610_e51283;
        var_iqd_nqs_dn0 = assign36610_e51283_d_n0;
        var_iqd_nqs_dn2 = assign36610_e51283_d_n2;
        var_iqd_nqs_dn6 = assign36610_e51283_d_n6;
        var_iqd_nqs_dn7 = assign36610_e51283_d_n7;
        var_iqd_nqs_dn10 = assign36610_e51283_d_n10;
        var_iqd_nqs_dn11 = assign36610_e51283_d_n11;
        var_iqd_nqs_dn12 = assign36610_e51283_d_n12;
        var_iqd_nqs_dn13 = assign36610_e51283_d_n13;
        var_iqd_nqs_dn15 = assign36610_e51283_d_n15;
        var_iqd_nqs_dn16 = assign36610_e51283_d_n16;
        var_iqd_nqs_dn17 = assign36610_e51283_d_n17;
        var_iqd_nqs_dn18 = assign36610_e51283_d_n18;

        let (assign36620_e51291, assign36620_e51291_d_n0, assign36620_e51291_d_n2, assign36620_e51291_d_n6, assign36620_e51291_d_n7, assign36620_e51291_d_n10, assign36620_e51291_d_n11, assign36620_e51291_d_n12, assign36620_e51291_d_n13, assign36620_e51291_d_n15, assign36620_e51291_d_n16, assign36620_e51291_d_n17, assign36620_e51291_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqs_nqs, var_iqs_nqs_dn0, var_iqs_nqs_dn2, var_iqs_nqs_dn6, var_iqs_nqs_dn7, var_iqs_nqs_dn10, var_iqs_nqs_dn11, var_iqs_nqs_dn12, var_iqs_nqs_dn13, var_iqs_nqs_dn15, var_iqs_nqs_dn16, var_iqs_nqs_dn17, var_iqs_nqs_dn18,)
    }
};
        var_iqs_nqs = assign36620_e51291;
        var_iqs_nqs_dn0 = assign36620_e51291_d_n0;
        var_iqs_nqs_dn2 = assign36620_e51291_d_n2;
        var_iqs_nqs_dn6 = assign36620_e51291_d_n6;
        var_iqs_nqs_dn7 = assign36620_e51291_d_n7;
        var_iqs_nqs_dn10 = assign36620_e51291_d_n10;
        var_iqs_nqs_dn11 = assign36620_e51291_d_n11;
        var_iqs_nqs_dn12 = assign36620_e51291_d_n12;
        var_iqs_nqs_dn13 = assign36620_e51291_d_n13;
        var_iqs_nqs_dn15 = assign36620_e51291_d_n15;
        var_iqs_nqs_dn16 = assign36620_e51291_d_n16;
        var_iqs_nqs_dn17 = assign36620_e51291_d_n17;
        var_iqs_nqs_dn18 = assign36620_e51291_d_n18;

        let (assign36630_e51299, assign36630_e51299_d_n0, assign36630_e51299_d_n2, assign36630_e51299_d_n6, assign36630_e51299_d_n7, assign36630_e51299_d_n10, assign36630_e51299_d_n11, assign36630_e51299_d_n12, assign36630_e51299_d_n13, assign36630_e51299_d_n15, assign36630_e51299_d_n16, assign36630_e51299_d_n17, assign36630_e51299_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    }
};
        var_iqb_nqs = assign36630_e51299;
        var_iqb_nqs_dn0 = assign36630_e51299_d_n0;
        var_iqb_nqs_dn2 = assign36630_e51299_d_n2;
        var_iqb_nqs_dn6 = assign36630_e51299_d_n6;
        var_iqb_nqs_dn7 = assign36630_e51299_d_n7;
        var_iqb_nqs_dn10 = assign36630_e51299_d_n10;
        var_iqb_nqs_dn11 = assign36630_e51299_d_n11;
        var_iqb_nqs_dn12 = assign36630_e51299_d_n12;
        var_iqb_nqs_dn13 = assign36630_e51299_d_n13;
        var_iqb_nqs_dn15 = assign36630_e51299_d_n15;
        var_iqb_nqs_dn16 = assign36630_e51299_d_n16;
        var_iqb_nqs_dn17 = assign36630_e51299_d_n17;
        var_iqb_nqs_dn18 = assign36630_e51299_d_n18;

        let (assign36640_e51307, assign36640_e51307_d_n0, assign36640_e51307_d_n2, assign36640_e51307_d_n6, assign36640_e51307_d_n7, assign36640_e51307_d_n10, assign36640_e51307_d_n11, assign36640_e51307_d_n12, assign36640_e51307_d_n15, assign36640_e51307_d_n17, assign36640_e51307_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign36640_e51307;
        var_qd_nqs_dn0 = assign36640_e51307_d_n0;
        var_qd_nqs_dn2 = assign36640_e51307_d_n2;
        var_qd_nqs_dn6 = assign36640_e51307_d_n6;
        var_qd_nqs_dn7 = assign36640_e51307_d_n7;
        var_qd_nqs_dn10 = assign36640_e51307_d_n10;
        var_qd_nqs_dn11 = assign36640_e51307_d_n11;
        var_qd_nqs_dn12 = assign36640_e51307_d_n12;
        var_qd_nqs_dn15 = assign36640_e51307_d_n15;
        var_qd_nqs_dn17 = assign36640_e51307_d_n17;
        var_qd_nqs_dn18 = assign36640_e51307_d_n18;

        let (assign36650_e51315, assign36650_e51315_d_n0, assign36650_e51315_d_n2, assign36650_e51315_d_n6, assign36650_e51315_d_n7, assign36650_e51315_d_n10, assign36650_e51315_d_n11, assign36650_e51315_d_n12, assign36650_e51315_d_n16, assign36650_e51315_d_n17, assign36650_e51315_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign36650_e51315;
        var_qs_nqs_dn0 = assign36650_e51315_d_n0;
        var_qs_nqs_dn2 = assign36650_e51315_d_n2;
        var_qs_nqs_dn6 = assign36650_e51315_d_n6;
        var_qs_nqs_dn7 = assign36650_e51315_d_n7;
        var_qs_nqs_dn10 = assign36650_e51315_d_n10;
        var_qs_nqs_dn11 = assign36650_e51315_d_n11;
        var_qs_nqs_dn12 = assign36650_e51315_d_n12;
        var_qs_nqs_dn16 = assign36650_e51315_d_n16;
        var_qs_nqs_dn17 = assign36650_e51315_d_n17;
        var_qs_nqs_dn18 = assign36650_e51315_d_n18;

        let (assign36660_e51323, assign36660_e51323_d_n0, assign36660_e51323_d_n2, assign36660_e51323_d_n6, assign36660_e51323_d_n7, assign36660_e51323_d_n10, assign36660_e51323_d_n11, assign36660_e51323_d_n12, assign36660_e51323_d_n13, assign36660_e51323_d_n15, assign36660_e51323_d_n16, assign36660_e51323_d_n17, assign36660_e51323_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36660_e51323;
        var_qg_nqs_dn0 = assign36660_e51323_d_n0;
        var_qg_nqs_dn2 = assign36660_e51323_d_n2;
        var_qg_nqs_dn6 = assign36660_e51323_d_n6;
        var_qg_nqs_dn7 = assign36660_e51323_d_n7;
        var_qg_nqs_dn10 = assign36660_e51323_d_n10;
        var_qg_nqs_dn11 = assign36660_e51323_d_n11;
        var_qg_nqs_dn12 = assign36660_e51323_d_n12;
        var_qg_nqs_dn13 = assign36660_e51323_d_n13;
        var_qg_nqs_dn15 = assign36660_e51323_d_n15;
        var_qg_nqs_dn16 = assign36660_e51323_d_n16;
        var_qg_nqs_dn17 = assign36660_e51323_d_n17;
        var_qg_nqs_dn18 = assign36660_e51323_d_n18;

        let (assign36670_e51331, assign36670_e51331_d_n13,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign36670_e51331;
        var_qb_nqs_dn13 = assign36670_e51331_d_n13;

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

        let assign36700_e51336: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard1214 = assign36700_e51336;

        *var_guard1212_slot = var_guard1212;
        *var_guard1213_slot = var_guard1213;
        *var_guard1214_slot = var_guard1214;
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
        var_guard1214: f64,
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
        var_guard1215_slot: &mut f64,
        var_guard1216_slot: &mut f64,
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
        let mut var_guard1215: f64 = *var_guard1215_slot;
        let mut var_guard1216: f64 = *var_guard1216_slot;
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

        let (assign36710_e51340, assign36710_e51340_d_n0, assign36710_e51340_d_n2, assign36710_e51340_d_n6, assign36710_e51340_d_n7, assign36710_e51340_d_n10, assign36710_e51340_d_n11, assign36710_e51340_d_n12, assign36710_e51340_d_n17,) = {
    if (var_guard1214 != 0.0) {
        (var_idse, var_idse_dn0, var_idse_dn2, var_idse_dn6, var_idse_dn7, var_idse_dn10, var_idse_dn11, var_idse_dn12, var_idse_dn17,)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn6, var_ids_dn7, var_ids_dn10, var_ids_dn11, var_ids_dn12, var_ids_dn17,)
    }
};
        var_ids = assign36710_e51340;
        var_ids_dn0 = assign36710_e51340_d_n0;
        var_ids_dn2 = assign36710_e51340_d_n2;
        var_ids_dn6 = assign36710_e51340_d_n6;
        var_ids_dn7 = assign36710_e51340_d_n7;
        var_ids_dn10 = assign36710_e51340_d_n10;
        var_ids_dn11 = assign36710_e51340_d_n11;
        var_ids_dn12 = assign36710_e51340_d_n12;
        var_ids_dn17 = assign36710_e51340_d_n17;

        let (assign36720_e51344, assign36720_e51344_d_n0, assign36720_e51344_d_n2, assign36720_e51344_d_n6, assign36720_e51344_d_n7, assign36720_e51344_d_n10, assign36720_e51344_d_n11, assign36720_e51344_d_n12, assign36720_e51344_d_n17,) = {
    if (var_guard1214 != 0.0) {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn6, var_isube_dn7, var_isube_dn10, var_isube_dn11, var_isube_dn12, var_isube_dn17,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn6, var_isub_dn7, var_isub_dn10, var_isub_dn11, var_isub_dn12, var_isub_dn17,)
    }
};
        var_isub = assign36720_e51344;
        var_isub_dn0 = assign36720_e51344_d_n0;
        var_isub_dn2 = assign36720_e51344_d_n2;
        var_isub_dn6 = assign36720_e51344_d_n6;
        var_isub_dn7 = assign36720_e51344_d_n7;
        var_isub_dn10 = assign36720_e51344_d_n10;
        var_isub_dn11 = assign36720_e51344_d_n11;
        var_isub_dn12 = assign36720_e51344_d_n12;
        var_isub_dn17 = assign36720_e51344_d_n17;

        let (assign36730_e51348, assign36730_e51348_d_n0, assign36730_e51348_d_n2, assign36730_e51348_d_n6, assign36730_e51348_d_n7, assign36730_e51348_d_n10, assign36730_e51348_d_n11, assign36730_e51348_d_n12, assign36730_e51348_d_n17,) = {
    if (var_guard1214 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isubs, var_isubs_dn0, var_isubs_dn2, var_isubs_dn6, var_isubs_dn7, var_isubs_dn10, var_isubs_dn11, var_isubs_dn12, var_isubs_dn17,)
    }
};
        var_isubs = assign36730_e51348;
        var_isubs_dn0 = assign36730_e51348_d_n0;
        var_isubs_dn2 = assign36730_e51348_d_n2;
        var_isubs_dn6 = assign36730_e51348_d_n6;
        var_isubs_dn7 = assign36730_e51348_d_n7;
        var_isubs_dn10 = assign36730_e51348_d_n10;
        var_isubs_dn11 = assign36730_e51348_d_n11;
        var_isubs_dn12 = assign36730_e51348_d_n12;
        var_isubs_dn17 = assign36730_e51348_d_n17;

        let (assign36740_e51354, assign36740_e51354_d_n0, assign36740_e51354_d_n2, assign36740_e51354_d_n6, assign36740_e51354_d_n7, assign36740_e51354_d_n10, assign36740_e51354_d_n11, assign36740_e51354_d_n12, assign36740_e51354_d_n13, assign36740_e51354_d_n15, assign36740_e51354_d_n16, assign36740_e51354_d_n17, assign36740_e51354_d_n18,) = {
    if (var_guard1214 != 0.0) {
        let assign36740_e51352: f64 = (var_qge + var_qg_nqs);
        (assign36740_e51352, (var_qge_dn0 + var_qg_nqs_dn0), (var_qge_dn2 + var_qg_nqs_dn2), (var_qge_dn6 + var_qg_nqs_dn6), (var_qge_dn7 + var_qg_nqs_dn7), (var_qge_dn10 + var_qg_nqs_dn10), (var_qge_dn11 + var_qg_nqs_dn11), (var_qge_dn12 + var_qg_nqs_dn12), (var_qge_dn13 + var_qg_nqs_dn13), (var_qge_dn15 + var_qg_nqs_dn15), (var_qge_dn16 + var_qg_nqs_dn16), (var_qge_dn17 + var_qg_nqs_dn17), (var_qge_dn18 + var_qg_nqs_dn18),)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn6, var_qg_dn7, var_qg_dn10, var_qg_dn11, var_qg_dn12, var_qg_dn13, var_qg_dn15, var_qg_dn16, var_qg_dn17, var_qg_dn18,)
    }
};
        var_qg = assign36740_e51354;
        var_qg_dn0 = assign36740_e51354_d_n0;
        var_qg_dn2 = assign36740_e51354_d_n2;
        var_qg_dn6 = assign36740_e51354_d_n6;
        var_qg_dn7 = assign36740_e51354_d_n7;
        var_qg_dn10 = assign36740_e51354_d_n10;
        var_qg_dn11 = assign36740_e51354_d_n11;
        var_qg_dn12 = assign36740_e51354_d_n12;
        var_qg_dn13 = assign36740_e51354_d_n13;
        var_qg_dn15 = assign36740_e51354_d_n15;
        var_qg_dn16 = assign36740_e51354_d_n16;
        var_qg_dn17 = assign36740_e51354_d_n17;
        var_qg_dn18 = assign36740_e51354_d_n18;

        let (assign36750_e51360, assign36750_e51360_d_n0, assign36750_e51360_d_n2, assign36750_e51360_d_n6, assign36750_e51360_d_n7, assign36750_e51360_d_n10, assign36750_e51360_d_n11, assign36750_e51360_d_n12, assign36750_e51360_d_n13, assign36750_e51360_d_n15, assign36750_e51360_d_n16, assign36750_e51360_d_n17, assign36750_e51360_d_n18,) = {
    if (var_guard1214 != 0.0) {
        let assign36750_e51358: f64 = (var_qde + var_qd_nqs);
        (assign36750_e51358, (var_qde_dn0 + var_qd_nqs_dn0), (var_qde_dn2 + var_qd_nqs_dn2), (var_qde_dn6 + var_qd_nqs_dn6), (var_qde_dn7 + var_qd_nqs_dn7), (var_qde_dn10 + var_qd_nqs_dn10), (var_qde_dn11 + var_qd_nqs_dn11), (var_qde_dn12 + var_qd_nqs_dn12), var_qde_dn13, (var_qde_dn15 + var_qd_nqs_dn15), var_qde_dn16, (var_qde_dn17 + var_qd_nqs_dn17), (var_qde_dn18 + var_qd_nqs_dn18),)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn6, var_qd_dn7, var_qd_dn10, var_qd_dn11, var_qd_dn12, var_qd_dn13, var_qd_dn15, var_qd_dn16, var_qd_dn17, var_qd_dn18,)
    }
};
        var_qd = assign36750_e51360;
        var_qd_dn0 = assign36750_e51360_d_n0;
        var_qd_dn2 = assign36750_e51360_d_n2;
        var_qd_dn6 = assign36750_e51360_d_n6;
        var_qd_dn7 = assign36750_e51360_d_n7;
        var_qd_dn10 = assign36750_e51360_d_n10;
        var_qd_dn11 = assign36750_e51360_d_n11;
        var_qd_dn12 = assign36750_e51360_d_n12;
        var_qd_dn13 = assign36750_e51360_d_n13;
        var_qd_dn15 = assign36750_e51360_d_n15;
        var_qd_dn16 = assign36750_e51360_d_n16;
        var_qd_dn17 = assign36750_e51360_d_n17;
        var_qd_dn18 = assign36750_e51360_d_n18;

        let (assign36770_e51375, assign36770_e51375_d_n0, assign36770_e51375_d_n2, assign36770_e51375_d_n6, assign36770_e51375_d_n7, assign36770_e51375_d_n10, assign36770_e51375_d_n11, assign36770_e51375_d_n12, assign36770_e51375_d_n13, assign36770_e51375_d_n15, assign36770_e51375_d_n16, assign36770_e51375_d_n17, assign36770_e51375_d_n18,) = {
    if (var_guard1214 != 0.0) {
        let assign36770_e51370: f64 = (var_qge + var_qde);
        let assign36770_e51372: f64 = (assign36770_e51370 + var_qse);
        let assign36770_e51373: f64 = (-assign36770_e51372);
        (assign36770_e51373, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn7 + var_qde_dn7) + var_qse_dn7)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn11 + var_qde_dn11) + var_qse_dn11)), (-((var_qge_dn12 + var_qde_dn12) + var_qse_dn12)), (-((var_qge_dn13 + var_qde_dn13) + var_qse_dn13)), (-((var_qge_dn15 + var_qde_dn15) + var_qse_dn15)), (-((var_qge_dn16 + var_qde_dn16) + var_qse_dn16)), (-((var_qge_dn17 + var_qde_dn17) + var_qse_dn17)), (-((var_qge_dn18 + var_qde_dn18) + var_qse_dn18)),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_dn13, var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    }
};
        var_qbe = assign36770_e51375;
        var_qbe_dn0 = assign36770_e51375_d_n0;
        var_qbe_dn2 = assign36770_e51375_d_n2;
        var_qbe_dn6 = assign36770_e51375_d_n6;
        var_qbe_dn7 = assign36770_e51375_d_n7;
        var_qbe_dn10 = assign36770_e51375_d_n10;
        var_qbe_dn11 = assign36770_e51375_d_n11;
        var_qbe_dn12 = assign36770_e51375_d_n12;
        var_qbe_dn13 = assign36770_e51375_d_n13;
        var_qbe_dn15 = assign36770_e51375_d_n15;
        var_qbe_dn16 = assign36770_e51375_d_n16;
        var_qbe_dn17 = assign36770_e51375_d_n17;
        var_qbe_dn18 = assign36770_e51375_d_n18;

        let (assign36780_e51381, assign36780_e51381_d_n0, assign36780_e51381_d_n2, assign36780_e51381_d_n6, assign36780_e51381_d_n7, assign36780_e51381_d_n10, assign36780_e51381_d_n11, assign36780_e51381_d_n12, assign36780_e51381_d_n13, assign36780_e51381_d_n15, assign36780_e51381_d_n16, assign36780_e51381_d_n17, assign36780_e51381_d_n18,) = {
    if (var_guard1214 != 0.0) {
        let assign36780_e51379: f64 = (var_qbe + var_qb_nqs);
        (assign36780_e51379, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, (var_qbe_dn13 + var_qb_nqs_dn13), var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn6, var_qb_dn7, var_qb_dn10, var_qb_dn11, var_qb_dn12, var_qb_dn13, var_qb_dn15, var_qb_dn16, var_qb_dn17, var_qb_dn18,)
    }
};
        var_qb = assign36780_e51381;
        var_qb_dn0 = assign36780_e51381_d_n0;
        var_qb_dn2 = assign36780_e51381_d_n2;
        var_qb_dn6 = assign36780_e51381_d_n6;
        var_qb_dn7 = assign36780_e51381_d_n7;
        var_qb_dn10 = assign36780_e51381_d_n10;
        var_qb_dn11 = assign36780_e51381_d_n11;
        var_qb_dn12 = assign36780_e51381_d_n12;
        var_qb_dn13 = assign36780_e51381_d_n13;
        var_qb_dn15 = assign36780_e51381_d_n15;
        var_qb_dn16 = assign36780_e51381_d_n16;
        var_qb_dn17 = assign36780_e51381_d_n17;
        var_qb_dn18 = assign36780_e51381_d_n18;

        let (assign36790_e51387, assign36790_e51387_d_n0, assign36790_e51387_d_n2, assign36790_e51387_d_n6, assign36790_e51387_d_n7, assign36790_e51387_d_n10, assign36790_e51387_d_n11, assign36790_e51387_d_n12, assign36790_e51387_d_n17,) = {
    if (var_guard1214 == 0.0) {
        let assign36790_e51385: f64 = (-var_idse);
        (assign36790_e51385, (-var_idse_dn0), (-var_idse_dn2), (-var_idse_dn6), (-var_idse_dn7), (-var_idse_dn10), (-var_idse_dn11), (-var_idse_dn12), (-var_idse_dn17),)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn6, var_ids_dn7, var_ids_dn10, var_ids_dn11, var_ids_dn12, var_ids_dn17,)
    }
};
        var_ids = assign36790_e51387;
        var_ids_dn0 = assign36790_e51387_d_n0;
        var_ids_dn2 = assign36790_e51387_d_n2;
        var_ids_dn6 = assign36790_e51387_d_n6;
        var_ids_dn7 = assign36790_e51387_d_n7;
        var_ids_dn10 = assign36790_e51387_d_n10;
        var_ids_dn11 = assign36790_e51387_d_n11;
        var_ids_dn12 = assign36790_e51387_d_n12;
        var_ids_dn17 = assign36790_e51387_d_n17;

        let (assign36800_e51392, assign36800_e51392_d_n0, assign36800_e51392_d_n2, assign36800_e51392_d_n6, assign36800_e51392_d_n7, assign36800_e51392_d_n10, assign36800_e51392_d_n11, assign36800_e51392_d_n12, assign36800_e51392_d_n17,) = {
    if (var_guard1214 == 0.0) {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn6, var_isube_dn7, var_isube_dn10, var_isube_dn11, var_isube_dn12, var_isube_dn17,)
    } else {
        (var_isubs, var_isubs_dn0, var_isubs_dn2, var_isubs_dn6, var_isubs_dn7, var_isubs_dn10, var_isubs_dn11, var_isubs_dn12, var_isubs_dn17,)
    }
};
        var_isubs = assign36800_e51392;
        var_isubs_dn0 = assign36800_e51392_d_n0;
        var_isubs_dn2 = assign36800_e51392_d_n2;
        var_isubs_dn6 = assign36800_e51392_d_n6;
        var_isubs_dn7 = assign36800_e51392_d_n7;
        var_isubs_dn10 = assign36800_e51392_d_n10;
        var_isubs_dn11 = assign36800_e51392_d_n11;
        var_isubs_dn12 = assign36800_e51392_d_n12;
        var_isubs_dn17 = assign36800_e51392_d_n17;

        let (assign36810_e51397, assign36810_e51397_d_n0, assign36810_e51397_d_n2, assign36810_e51397_d_n6, assign36810_e51397_d_n7, assign36810_e51397_d_n10, assign36810_e51397_d_n11, assign36810_e51397_d_n12, assign36810_e51397_d_n17,) = {
    if (var_guard1214 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn6, var_isub_dn7, var_isub_dn10, var_isub_dn11, var_isub_dn12, var_isub_dn17,)
    }
};
        var_isub = assign36810_e51397;
        var_isub_dn0 = assign36810_e51397_d_n0;
        var_isub_dn2 = assign36810_e51397_d_n2;
        var_isub_dn6 = assign36810_e51397_d_n6;
        var_isub_dn7 = assign36810_e51397_d_n7;
        var_isub_dn10 = assign36810_e51397_d_n10;
        var_isub_dn11 = assign36810_e51397_d_n11;
        var_isub_dn12 = assign36810_e51397_d_n12;
        var_isub_dn17 = assign36810_e51397_d_n17;

        let (assign36820_e51404, assign36820_e51404_d_n0, assign36820_e51404_d_n2, assign36820_e51404_d_n6, assign36820_e51404_d_n7, assign36820_e51404_d_n10, assign36820_e51404_d_n11, assign36820_e51404_d_n12, assign36820_e51404_d_n13, assign36820_e51404_d_n15, assign36820_e51404_d_n16, assign36820_e51404_d_n17, assign36820_e51404_d_n18,) = {
    if (var_guard1214 == 0.0) {
        let assign36820_e51402: f64 = (var_qge + var_qg_nqs);
        (assign36820_e51402, (var_qge_dn0 + var_qg_nqs_dn0), (var_qge_dn2 + var_qg_nqs_dn2), (var_qge_dn6 + var_qg_nqs_dn6), (var_qge_dn7 + var_qg_nqs_dn7), (var_qge_dn10 + var_qg_nqs_dn10), (var_qge_dn11 + var_qg_nqs_dn11), (var_qge_dn12 + var_qg_nqs_dn12), (var_qge_dn13 + var_qg_nqs_dn13), (var_qge_dn15 + var_qg_nqs_dn15), (var_qge_dn16 + var_qg_nqs_dn16), (var_qge_dn17 + var_qg_nqs_dn17), (var_qge_dn18 + var_qg_nqs_dn18),)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn6, var_qg_dn7, var_qg_dn10, var_qg_dn11, var_qg_dn12, var_qg_dn13, var_qg_dn15, var_qg_dn16, var_qg_dn17, var_qg_dn18,)
    }
};
        var_qg = assign36820_e51404;
        var_qg_dn0 = assign36820_e51404_d_n0;
        var_qg_dn2 = assign36820_e51404_d_n2;
        var_qg_dn6 = assign36820_e51404_d_n6;
        var_qg_dn7 = assign36820_e51404_d_n7;
        var_qg_dn10 = assign36820_e51404_d_n10;
        var_qg_dn11 = assign36820_e51404_d_n11;
        var_qg_dn12 = assign36820_e51404_d_n12;
        var_qg_dn13 = assign36820_e51404_d_n13;
        var_qg_dn15 = assign36820_e51404_d_n15;
        var_qg_dn16 = assign36820_e51404_d_n16;
        var_qg_dn17 = assign36820_e51404_d_n17;
        var_qg_dn18 = assign36820_e51404_d_n18;

        let (assign36830_e51411, assign36830_e51411_d_n0, assign36830_e51411_d_n2, assign36830_e51411_d_n6, assign36830_e51411_d_n7, assign36830_e51411_d_n10, assign36830_e51411_d_n11, assign36830_e51411_d_n12, assign36830_e51411_d_n13, assign36830_e51411_d_n15, assign36830_e51411_d_n16, assign36830_e51411_d_n17, assign36830_e51411_d_n18,) = {
    if (var_guard1214 == 0.0) {
        let assign36830_e51409: f64 = (var_qse + var_qs_nqs);
        (assign36830_e51409, (var_qse_dn0 + var_qs_nqs_dn0), (var_qse_dn2 + var_qs_nqs_dn2), (var_qse_dn6 + var_qs_nqs_dn6), (var_qse_dn7 + var_qs_nqs_dn7), (var_qse_dn10 + var_qs_nqs_dn10), (var_qse_dn11 + var_qs_nqs_dn11), (var_qse_dn12 + var_qs_nqs_dn12), var_qse_dn13, var_qse_dn15, (var_qse_dn16 + var_qs_nqs_dn16), (var_qse_dn17 + var_qs_nqs_dn17), (var_qse_dn18 + var_qs_nqs_dn18),)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn6, var_qd_dn7, var_qd_dn10, var_qd_dn11, var_qd_dn12, var_qd_dn13, var_qd_dn15, var_qd_dn16, var_qd_dn17, var_qd_dn18,)
    }
};
        var_qd = assign36830_e51411;
        var_qd_dn0 = assign36830_e51411_d_n0;
        var_qd_dn2 = assign36830_e51411_d_n2;
        var_qd_dn6 = assign36830_e51411_d_n6;
        var_qd_dn7 = assign36830_e51411_d_n7;
        var_qd_dn10 = assign36830_e51411_d_n10;
        var_qd_dn11 = assign36830_e51411_d_n11;
        var_qd_dn12 = assign36830_e51411_d_n12;
        var_qd_dn13 = assign36830_e51411_d_n13;
        var_qd_dn15 = assign36830_e51411_d_n15;
        var_qd_dn16 = assign36830_e51411_d_n16;
        var_qd_dn17 = assign36830_e51411_d_n17;
        var_qd_dn18 = assign36830_e51411_d_n18;

        let (assign36850_e51428, assign36850_e51428_d_n0, assign36850_e51428_d_n2, assign36850_e51428_d_n6, assign36850_e51428_d_n7, assign36850_e51428_d_n10, assign36850_e51428_d_n11, assign36850_e51428_d_n12, assign36850_e51428_d_n13, assign36850_e51428_d_n15, assign36850_e51428_d_n16, assign36850_e51428_d_n17, assign36850_e51428_d_n18,) = {
    if (var_guard1214 == 0.0) {
        let assign36850_e51423: f64 = (var_qge + var_qde);
        let assign36850_e51425: f64 = (assign36850_e51423 + var_qse);
        let assign36850_e51426: f64 = (-assign36850_e51425);
        (assign36850_e51426, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn7 + var_qde_dn7) + var_qse_dn7)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn11 + var_qde_dn11) + var_qse_dn11)), (-((var_qge_dn12 + var_qde_dn12) + var_qse_dn12)), (-((var_qge_dn13 + var_qde_dn13) + var_qse_dn13)), (-((var_qge_dn15 + var_qde_dn15) + var_qse_dn15)), (-((var_qge_dn16 + var_qde_dn16) + var_qse_dn16)), (-((var_qge_dn17 + var_qde_dn17) + var_qse_dn17)), (-((var_qge_dn18 + var_qde_dn18) + var_qse_dn18)),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_dn13, var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    }
};
        var_qbe = assign36850_e51428;
        var_qbe_dn0 = assign36850_e51428_d_n0;
        var_qbe_dn2 = assign36850_e51428_d_n2;
        var_qbe_dn6 = assign36850_e51428_d_n6;
        var_qbe_dn7 = assign36850_e51428_d_n7;
        var_qbe_dn10 = assign36850_e51428_d_n10;
        var_qbe_dn11 = assign36850_e51428_d_n11;
        var_qbe_dn12 = assign36850_e51428_d_n12;
        var_qbe_dn13 = assign36850_e51428_d_n13;
        var_qbe_dn15 = assign36850_e51428_d_n15;
        var_qbe_dn16 = assign36850_e51428_d_n16;
        var_qbe_dn17 = assign36850_e51428_d_n17;
        var_qbe_dn18 = assign36850_e51428_d_n18;

        let (assign36860_e51435, assign36860_e51435_d_n0, assign36860_e51435_d_n2, assign36860_e51435_d_n6, assign36860_e51435_d_n7, assign36860_e51435_d_n10, assign36860_e51435_d_n11, assign36860_e51435_d_n12, assign36860_e51435_d_n13, assign36860_e51435_d_n15, assign36860_e51435_d_n16, assign36860_e51435_d_n17, assign36860_e51435_d_n18,) = {
    if (var_guard1214 == 0.0) {
        let assign36860_e51433: f64 = (var_qbe + var_qb_nqs);
        (assign36860_e51433, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, (var_qbe_dn13 + var_qb_nqs_dn13), var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn6, var_qb_dn7, var_qb_dn10, var_qb_dn11, var_qb_dn12, var_qb_dn13, var_qb_dn15, var_qb_dn16, var_qb_dn17, var_qb_dn18,)
    }
};
        var_qb = assign36860_e51435;
        var_qb_dn0 = assign36860_e51435_d_n0;
        var_qb_dn2 = assign36860_e51435_d_n2;
        var_qb_dn6 = assign36860_e51435_d_n6;
        var_qb_dn7 = assign36860_e51435_d_n7;
        var_qb_dn10 = assign36860_e51435_d_n10;
        var_qb_dn11 = assign36860_e51435_d_n11;
        var_qb_dn12 = assign36860_e51435_d_n12;
        var_qb_dn13 = assign36860_e51435_d_n13;
        var_qb_dn15 = assign36860_e51435_d_n15;
        var_qb_dn16 = assign36860_e51435_d_n16;
        var_qb_dn17 = assign36860_e51435_d_n17;
        var_qb_dn18 = assign36860_e51435_d_n18;

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

        let assign36920_e51443: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1215 = assign36920_e51443;

        let (assign36930_e51447, assign36930_e51447_d_n0, assign36930_e51447_d_n2, assign36930_e51447_d_n6, assign36930_e51447_d_n7, assign36930_e51447_d_n10, assign36930_e51447_d_n11, assign36930_e51447_d_n12, assign36930_e51447_d_n17,) = {
    if (var_guard1215 != 0.0) {
        (var_ibdb, var_ibdb_dn0, var_ibdb_dn2, var_ibdb_dn6, var_ibdb_dn7, var_ibdb_dn10, var_ibdb_dn11, var_ibdb_dn12, var_ibdb_dn17,)
    } else {
        (var_ibd, var_ibd_dn0, var_ibd_dn2, var_ibd_dn6, var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, var_ibd_dn12, var_ibd_dn17,)
    }
};
        var_ibd = assign36930_e51447;
        var_ibd_dn0 = assign36930_e51447_d_n0;
        var_ibd_dn2 = assign36930_e51447_d_n2;
        var_ibd_dn6 = assign36930_e51447_d_n6;
        var_ibd_dn7 = assign36930_e51447_d_n7;
        var_ibd_dn10 = assign36930_e51447_d_n10;
        var_ibd_dn11 = assign36930_e51447_d_n11;
        var_ibd_dn12 = assign36930_e51447_d_n12;
        var_ibd_dn17 = assign36930_e51447_d_n17;

        let (assign36940_e51451, assign36940_e51451_d_n0, assign36940_e51451_d_n2, assign36940_e51451_d_n6, assign36940_e51451_d_n7, assign36940_e51451_d_n10, assign36940_e51451_d_n11, assign36940_e51451_d_n12, assign36940_e51451_d_n17,) = {
    if (var_guard1215 != 0.0) {
        (var_qbd_s0, var_qbd_s0_dn0, var_qbd_s0_dn2, var_qbd_s0_dn6, var_qbd_s0_dn7, var_qbd_s0_dn10, var_qbd_s0_dn11, var_qbd_s0_dn12, var_qbd_s0_dn17,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign36940_e51451;
        var_qbd_dn0 = assign36940_e51451_d_n0;
        var_qbd_dn2 = assign36940_e51451_d_n2;
        var_qbd_dn6 = assign36940_e51451_d_n6;
        var_qbd_dn7 = assign36940_e51451_d_n7;
        var_qbd_dn10 = assign36940_e51451_d_n10;
        var_qbd_dn11 = assign36940_e51451_d_n11;
        var_qbd_dn12 = assign36940_e51451_d_n12;
        var_qbd_dn17 = assign36940_e51451_d_n17;

        let (assign36950_e51455, assign36950_e51455_d_n0, assign36950_e51455_d_n2, assign36950_e51455_d_n6, assign36950_e51455_d_n7, assign36950_e51455_d_n10, assign36950_e51455_d_n11, assign36950_e51455_d_n12, assign36950_e51455_d_n17,) = {
    if (var_guard1215 != 0.0) {
        (var_ibsb, var_ibsb_dn0, var_ibsb_dn2, var_ibsb_dn6, var_ibsb_dn7, var_ibsb_dn10, var_ibsb_dn11, var_ibsb_dn12, var_ibsb_dn17,)
    } else {
        (var_ibs, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, var_ibs_dn7, var_ibs_dn10, var_ibs_dn11, var_ibs_dn12, var_ibs_dn17,)
    }
};
        var_ibs = assign36950_e51455;
        var_ibs_dn0 = assign36950_e51455_d_n0;
        var_ibs_dn2 = assign36950_e51455_d_n2;
        var_ibs_dn6 = assign36950_e51455_d_n6;
        var_ibs_dn7 = assign36950_e51455_d_n7;
        var_ibs_dn10 = assign36950_e51455_d_n10;
        var_ibs_dn11 = assign36950_e51455_d_n11;
        var_ibs_dn12 = assign36950_e51455_d_n12;
        var_ibs_dn17 = assign36950_e51455_d_n17;

        let (assign36960_e51459, assign36960_e51459_d_n0, assign36960_e51459_d_n2, assign36960_e51459_d_n6, assign36960_e51459_d_n7, assign36960_e51459_d_n10, assign36960_e51459_d_n11, assign36960_e51459_d_n12, assign36960_e51459_d_n17,) = {
    if (var_guard1215 != 0.0) {
        (var_qbs_s0, var_qbs_s0_dn0, var_qbs_s0_dn2, var_qbs_s0_dn6, var_qbs_s0_dn7, var_qbs_s0_dn10, var_qbs_s0_dn11, var_qbs_s0_dn12, var_qbs_s0_dn17,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign36960_e51459;
        var_qbs_dn0 = assign36960_e51459_d_n0;
        var_qbs_dn2 = assign36960_e51459_d_n2;
        var_qbs_dn6 = assign36960_e51459_d_n6;
        var_qbs_dn7 = assign36960_e51459_d_n7;
        var_qbs_dn10 = assign36960_e51459_d_n10;
        var_qbs_dn11 = assign36960_e51459_d_n11;
        var_qbs_dn12 = assign36960_e51459_d_n12;
        var_qbs_dn17 = assign36960_e51459_d_n17;

        let assign36970_e51466: f64 = if ((p.p38 == 1.0) && (var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1216 = assign36970_e51466;

        let (assign36980_e51472, assign36980_e51472_d_n0, assign36980_e51472_d_n2, assign36980_e51472_d_n6, assign36980_e51472_d_n7, assign36980_e51472_d_n10, assign36980_e51472_d_n11, assign36980_e51472_d_n12, assign36980_e51472_d_n17,) = {
    if (var_guard1216 != 0.0) {
        let assign36980_e51470: f64 = (var_ids * var_vds);
        (assign36980_e51470, ((var_ids_dn0 * var_vds) + (var_ids * var_vds_dn0)), ((var_ids_dn2 * var_vds) + (var_ids * var_vds_dn2)), ((var_ids_dn6 * var_vds) + (var_ids * var_vds_dn6)), ((var_ids_dn7 * var_vds) + (var_ids * var_vds_dn7)), ((var_ids_dn10 * var_vds) + (var_ids * var_vds_dn10)), ((var_ids_dn11 * var_vds) + (var_ids * var_vds_dn11)), ((var_ids_dn12 * var_vds) + (var_ids * var_vds_dn12)), ((var_ids_dn17 * var_vds) + (var_ids * var_vds_dn17)),)
    } else {
        (var_rpower, var_rpower_dn0, var_rpower_dn2, var_rpower_dn6, var_rpower_dn7, var_rpower_dn10, var_rpower_dn11, var_rpower_dn12, var_rpower_dn17,)
    }
};
        var_rpower = assign36980_e51472;
        var_rpower_dn0 = assign36980_e51472_d_n0;
        var_rpower_dn2 = assign36980_e51472_d_n2;
        var_rpower_dn6 = assign36980_e51472_d_n6;
        var_rpower_dn7 = assign36980_e51472_d_n7;
        var_rpower_dn10 = assign36980_e51472_d_n10;
        var_rpower_dn11 = assign36980_e51472_d_n11;
        var_rpower_dn12 = assign36980_e51472_d_n12;
        var_rpower_dn17 = assign36980_e51472_d_n17;

        let (assign36990_e51476,) = {
    if (var_guard1216 != 0.0) {
        (var_cth,)
    } else {
        (var_cthe,)
    }
};
        var_cthe = assign36990_e51476;

        let (assign37000_e51482,) = {
    if (var_guard1216 != 0.0) {
        let assign37000_e51480: f64 = (1.0 / var_rth);
        (assign37000_e51480,)
    } else {
        (var_gth,)
    }
};
        var_gth = assign37000_e51482;

        let (assign37010_e51487, assign37010_e51487_d_n0, assign37010_e51487_d_n2, assign37010_e51487_d_n6, assign37010_e51487_d_n7, assign37010_e51487_d_n10, assign37010_e51487_d_n11, assign37010_e51487_d_n12, assign37010_e51487_d_n17,) = {
    if (var_guard1216 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rpower, var_rpower_dn0, var_rpower_dn2, var_rpower_dn6, var_rpower_dn7, var_rpower_dn10, var_rpower_dn11, var_rpower_dn12, var_rpower_dn17,)
    }
};
        var_rpower = assign37010_e51487;
        var_rpower_dn0 = assign37010_e51487_d_n0;
        var_rpower_dn2 = assign37010_e51487_d_n2;
        var_rpower_dn6 = assign37010_e51487_d_n6;
        var_rpower_dn7 = assign37010_e51487_d_n7;
        var_rpower_dn10 = assign37010_e51487_d_n10;
        var_rpower_dn11 = assign37010_e51487_d_n11;
        var_rpower_dn12 = assign37010_e51487_d_n12;
        var_rpower_dn17 = assign37010_e51487_d_n17;

        let (assign37020_e51492,) = {
    if (var_guard1216 == 0.0) {
        (0.0,)
    } else {
        (var_cthe,)
    }
};
        var_cthe = assign37020_e51492;

        let (assign37030_e51497,) = {
    if (var_guard1216 == 0.0) {
        (0.0,)
    } else {
        (var_gth,)
    }
};
        var_gth = assign37030_e51497;

        var_idse = var_ids;
        var_idse_dn0 = var_ids_dn0;
        var_idse_dn2 = var_ids_dn2;
        var_idse_dn6 = var_ids_dn6;
        var_idse_dn7 = var_ids_dn7;
        var_idse_dn10 = var_ids_dn10;
        var_idse_dn11 = var_ids_dn11;
        var_idse_dn12 = var_ids_dn12;
        var_idse_dn17 = var_ids_dn17;

        let assign37190_e51546: f64 = var_qg_dn6;
        var_cgdbd = assign37190_e51546;
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
        *var_guard1215_slot = var_guard1215;
        *var_guard1216_slot = var_guard1216;
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
