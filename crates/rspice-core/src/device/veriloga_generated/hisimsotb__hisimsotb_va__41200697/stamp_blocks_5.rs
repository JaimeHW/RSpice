#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        p: &Parameters,
        var_cgdo_given: f64,
        var_cgso_given: f64,
        var_chi: f64,
        var_chi_dn0: f64,
        var_chi_dn10: f64,
        var_chi_dn11: f64,
        var_chi_dn12: f64,
        var_chi_dn2: f64,
        var_chi_dn4: f64,
        var_chi_dn5: f64,
        var_chi_dn6: f64,
        var_chi_dn8: f64,
        var_cnst0over: f64,
        var_cnst0over_dn0: f64,
        var_cnst0over_dn10: f64,
        var_cnst0over_dn11: f64,
        var_cnst0over_dn12: f64,
        var_cnst0over_dn2: f64,
        var_cnst0over_dn4: f64,
        var_cnst0over_dn5: f64,
        var_cnst0over_dn6: f64,
        var_cnst0over_dn8: f64,
        var_cox0: f64,
        var_fb: f64,
        var_fb_dn0: f64,
        var_fb_dn10: f64,
        var_fb_dn11: f64,
        var_fb_dn12: f64,
        var_fb_dn2: f64,
        var_fb_dn4: f64,
        var_fb_dn5: f64,
        var_fb_dn6: f64,
        var_fb_dn8: f64,
        var_flg_overd: f64,
        var_flg_overs: f64,
        var_fs01: f64,
        var_fs01_dn0: f64,
        var_fs01_dn10: f64,
        var_fs01_dn11: f64,
        var_fs01_dn12: f64,
        var_fs01_dn2: f64,
        var_fs01_dn4: f64,
        var_fs01_dn5: f64,
        var_fs01_dn6: f64,
        var_fs01_dn8: f64,
        var_fs02: f64,
        var_fs02_dn0: f64,
        var_fs02_dn10: f64,
        var_fs02_dn11: f64,
        var_fs02_dn12: f64,
        var_fs02_dn2: f64,
        var_fs02_dn4: f64,
        var_fs02_dn5: f64,
        var_fs02_dn6: f64,
        var_fs02_dn8: f64,
        var_guard327: f64,
        var_guard346: f64,
        var_guard353: f64,
        var_lov: f64,
        var_mode: f64,
        var_modenml: f64,
        var_modervs: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn8: f64,
        var_vgs: f64,
        var_vgs_dn11: f64,
        var_vgs_dn12: f64,
        var_vgs_dn5: f64,
        var_weffcv_nf: f64,
        var_weffcv_nf_dn0: f64,
        var_weffcv_nf_dn10: f64,
        var_weffcv_nf_dn11: f64,
        var_weffcv_nf_dn12: f64,
        var_weffcv_nf_dn2: f64,
        var_weffcv_nf_dn4: f64,
        var_weffcv_nf_dn5: f64,
        var_weffcv_nf_dn6: f64,
        var_weffcv_nf_dn8: f64,
        var_cgdoe_slot: &mut f64,
        var_cgdoe_dn0_slot: &mut f64,
        var_cgdoe_dn10_slot: &mut f64,
        var_cgdoe_dn11_slot: &mut f64,
        var_cgdoe_dn12_slot: &mut f64,
        var_cgdoe_dn2_slot: &mut f64,
        var_cgdoe_dn4_slot: &mut f64,
        var_cgdoe_dn5_slot: &mut f64,
        var_cgdoe_dn6_slot: &mut f64,
        var_cgdoe_dn8_slot: &mut f64,
        var_cgsoe_slot: &mut f64,
        var_cgsoe_dn0_slot: &mut f64,
        var_cgsoe_dn10_slot: &mut f64,
        var_cgsoe_dn11_slot: &mut f64,
        var_cgsoe_dn12_slot: &mut f64,
        var_cgsoe_dn2_slot: &mut f64,
        var_cgsoe_dn4_slot: &mut f64,
        var_cgsoe_dn5_slot: &mut f64,
        var_cgsoe_dn6_slot: &mut f64,
        var_cgsoe_dn8_slot: &mut f64,
        var_flg_overgiven_slot: &mut f64,
        var_guard360_slot: &mut f64,
        var_guard362_slot: &mut f64,
        var_guard363_slot: &mut f64,
        var_qbdld_slot: &mut f64,
        var_qbdld_dn0_slot: &mut f64,
        var_qbdld_dn10_slot: &mut f64,
        var_qbdld_dn11_slot: &mut f64,
        var_qbdld_dn12_slot: &mut f64,
        var_qbdld_dn2_slot: &mut f64,
        var_qbdld_dn4_slot: &mut f64,
        var_qbdld_dn5_slot: &mut f64,
        var_qbdld_dn6_slot: &mut f64,
        var_qbdld_dn8_slot: &mut f64,
        var_qbsld_slot: &mut f64,
        var_qbsld_dn0_slot: &mut f64,
        var_qbsld_dn10_slot: &mut f64,
        var_qbsld_dn11_slot: &mut f64,
        var_qbsld_dn12_slot: &mut f64,
        var_qbsld_dn2_slot: &mut f64,
        var_qbsld_dn4_slot: &mut f64,
        var_qbsld_dn5_slot: &mut f64,
        var_qbsld_dn6_slot: &mut f64,
        var_qbsld_dn8_slot: &mut f64,
        var_qbuld_slot: &mut f64,
        var_qbuld_dn0_slot: &mut f64,
        var_qbuld_dn10_slot: &mut f64,
        var_qbuld_dn11_slot: &mut f64,
        var_qbuld_dn12_slot: &mut f64,
        var_qbuld_dn2_slot: &mut f64,
        var_qbuld_dn4_slot: &mut f64,
        var_qbuld_dn5_slot: &mut f64,
        var_qbuld_dn6_slot: &mut f64,
        var_qbuld_dn8_slot: &mut f64,
        var_qgod_slot: &mut f64,
        var_qgod_dn0_slot: &mut f64,
        var_qgod_dn10_slot: &mut f64,
        var_qgod_dn11_slot: &mut f64,
        var_qgod_dn12_slot: &mut f64,
        var_qgod_dn2_slot: &mut f64,
        var_qgod_dn4_slot: &mut f64,
        var_qgod_dn5_slot: &mut f64,
        var_qgod_dn6_slot: &mut f64,
        var_qgod_dn8_slot: &mut f64,
        var_qgos_slot: &mut f64,
        var_qgos_dn0_slot: &mut f64,
        var_qgos_dn10_slot: &mut f64,
        var_qgos_dn11_slot: &mut f64,
        var_qgos_dn12_slot: &mut f64,
        var_qgos_dn2_slot: &mut f64,
        var_qgos_dn4_slot: &mut f64,
        var_qgos_dn5_slot: &mut f64,
        var_qgos_dn6_slot: &mut f64,
        var_qgos_dn8_slot: &mut f64,
        var_qiuld_slot: &mut f64,
        var_qiuld_dn0_slot: &mut f64,
        var_qiuld_dn10_slot: &mut f64,
        var_qiuld_dn11_slot: &mut f64,
        var_qiuld_dn12_slot: &mut f64,
        var_qiuld_dn2_slot: &mut f64,
        var_qiuld_dn4_slot: &mut f64,
        var_qiuld_dn5_slot: &mut f64,
        var_qiuld_dn6_slot: &mut f64,
        var_qiuld_dn8_slot: &mut f64,
        var_qovd_slot: &mut f64,
        var_qovd_dn0_slot: &mut f64,
        var_qovd_dn10_slot: &mut f64,
        var_qovd_dn11_slot: &mut f64,
        var_qovd_dn12_slot: &mut f64,
        var_qovd_dn2_slot: &mut f64,
        var_qovd_dn4_slot: &mut f64,
        var_qovd_dn5_slot: &mut f64,
        var_qovd_dn6_slot: &mut f64,
        var_qovd_dn8_slot: &mut f64,
        var_qovs_slot: &mut f64,
        var_qovs_dn0_slot: &mut f64,
        var_qovs_dn10_slot: &mut f64,
        var_qovs_dn11_slot: &mut f64,
        var_qovs_dn12_slot: &mut f64,
        var_qovs_dn2_slot: &mut f64,
        var_qovs_dn4_slot: &mut f64,
        var_qovs_dn5_slot: &mut f64,
        var_qovs_dn6_slot: &mut f64,
        var_qovs_dn8_slot: &mut f64,
        var_qsuld_slot: &mut f64,
        var_qsuld_dn0_slot: &mut f64,
        var_qsuld_dn10_slot: &mut f64,
        var_qsuld_dn11_slot: &mut f64,
        var_qsuld_dn12_slot: &mut f64,
        var_qsuld_dn2_slot: &mut f64,
        var_qsuld_dn4_slot: &mut f64,
        var_qsuld_dn5_slot: &mut f64,
        var_qsuld_dn6_slot: &mut f64,
        var_qsuld_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_xi0_slot: &mut f64,
        var_xi0_dn0_slot: &mut f64,
        var_xi0_dn10_slot: &mut f64,
        var_xi0_dn11_slot: &mut f64,
        var_xi0_dn12_slot: &mut f64,
        var_xi0_dn2_slot: &mut f64,
        var_xi0_dn4_slot: &mut f64,
        var_xi0_dn5_slot: &mut f64,
        var_xi0_dn6_slot: &mut f64,
        var_xi0_dn8_slot: &mut f64,
        var_xi0p12_slot: &mut f64,
        var_xi0p12_dn0_slot: &mut f64,
        var_xi0p12_dn10_slot: &mut f64,
        var_xi0p12_dn11_slot: &mut f64,
        var_xi0p12_dn12_slot: &mut f64,
        var_xi0p12_dn2_slot: &mut f64,
        var_xi0p12_dn4_slot: &mut f64,
        var_xi0p12_dn5_slot: &mut f64,
        var_xi0p12_dn6_slot: &mut f64,
        var_xi0p12_dn8_slot: &mut f64,
    ) {
        let mut var_cgdoe: f64 = *var_cgdoe_slot;
        let mut var_cgdoe_dn0: f64 = *var_cgdoe_dn0_slot;
        let mut var_cgdoe_dn10: f64 = *var_cgdoe_dn10_slot;
        let mut var_cgdoe_dn11: f64 = *var_cgdoe_dn11_slot;
        let mut var_cgdoe_dn12: f64 = *var_cgdoe_dn12_slot;
        let mut var_cgdoe_dn2: f64 = *var_cgdoe_dn2_slot;
        let mut var_cgdoe_dn4: f64 = *var_cgdoe_dn4_slot;
        let mut var_cgdoe_dn5: f64 = *var_cgdoe_dn5_slot;
        let mut var_cgdoe_dn6: f64 = *var_cgdoe_dn6_slot;
        let mut var_cgdoe_dn8: f64 = *var_cgdoe_dn8_slot;
        let mut var_cgsoe: f64 = *var_cgsoe_slot;
        let mut var_cgsoe_dn0: f64 = *var_cgsoe_dn0_slot;
        let mut var_cgsoe_dn10: f64 = *var_cgsoe_dn10_slot;
        let mut var_cgsoe_dn11: f64 = *var_cgsoe_dn11_slot;
        let mut var_cgsoe_dn12: f64 = *var_cgsoe_dn12_slot;
        let mut var_cgsoe_dn2: f64 = *var_cgsoe_dn2_slot;
        let mut var_cgsoe_dn4: f64 = *var_cgsoe_dn4_slot;
        let mut var_cgsoe_dn5: f64 = *var_cgsoe_dn5_slot;
        let mut var_cgsoe_dn6: f64 = *var_cgsoe_dn6_slot;
        let mut var_cgsoe_dn8: f64 = *var_cgsoe_dn8_slot;
        let mut var_flg_overgiven: f64 = *var_flg_overgiven_slot;
        let mut var_guard360: f64 = *var_guard360_slot;
        let mut var_guard362: f64 = *var_guard362_slot;
        let mut var_guard363: f64 = *var_guard363_slot;
        let mut var_qbdld: f64 = *var_qbdld_slot;
        let mut var_qbdld_dn0: f64 = *var_qbdld_dn0_slot;
        let mut var_qbdld_dn10: f64 = *var_qbdld_dn10_slot;
        let mut var_qbdld_dn11: f64 = *var_qbdld_dn11_slot;
        let mut var_qbdld_dn12: f64 = *var_qbdld_dn12_slot;
        let mut var_qbdld_dn2: f64 = *var_qbdld_dn2_slot;
        let mut var_qbdld_dn4: f64 = *var_qbdld_dn4_slot;
        let mut var_qbdld_dn5: f64 = *var_qbdld_dn5_slot;
        let mut var_qbdld_dn6: f64 = *var_qbdld_dn6_slot;
        let mut var_qbdld_dn8: f64 = *var_qbdld_dn8_slot;
        let mut var_qbsld: f64 = *var_qbsld_slot;
        let mut var_qbsld_dn0: f64 = *var_qbsld_dn0_slot;
        let mut var_qbsld_dn10: f64 = *var_qbsld_dn10_slot;
        let mut var_qbsld_dn11: f64 = *var_qbsld_dn11_slot;
        let mut var_qbsld_dn12: f64 = *var_qbsld_dn12_slot;
        let mut var_qbsld_dn2: f64 = *var_qbsld_dn2_slot;
        let mut var_qbsld_dn4: f64 = *var_qbsld_dn4_slot;
        let mut var_qbsld_dn5: f64 = *var_qbsld_dn5_slot;
        let mut var_qbsld_dn6: f64 = *var_qbsld_dn6_slot;
        let mut var_qbsld_dn8: f64 = *var_qbsld_dn8_slot;
        let mut var_qbuld: f64 = *var_qbuld_slot;
        let mut var_qbuld_dn0: f64 = *var_qbuld_dn0_slot;
        let mut var_qbuld_dn10: f64 = *var_qbuld_dn10_slot;
        let mut var_qbuld_dn11: f64 = *var_qbuld_dn11_slot;
        let mut var_qbuld_dn12: f64 = *var_qbuld_dn12_slot;
        let mut var_qbuld_dn2: f64 = *var_qbuld_dn2_slot;
        let mut var_qbuld_dn4: f64 = *var_qbuld_dn4_slot;
        let mut var_qbuld_dn5: f64 = *var_qbuld_dn5_slot;
        let mut var_qbuld_dn6: f64 = *var_qbuld_dn6_slot;
        let mut var_qbuld_dn8: f64 = *var_qbuld_dn8_slot;
        let mut var_qgod: f64 = *var_qgod_slot;
        let mut var_qgod_dn0: f64 = *var_qgod_dn0_slot;
        let mut var_qgod_dn10: f64 = *var_qgod_dn10_slot;
        let mut var_qgod_dn11: f64 = *var_qgod_dn11_slot;
        let mut var_qgod_dn12: f64 = *var_qgod_dn12_slot;
        let mut var_qgod_dn2: f64 = *var_qgod_dn2_slot;
        let mut var_qgod_dn4: f64 = *var_qgod_dn4_slot;
        let mut var_qgod_dn5: f64 = *var_qgod_dn5_slot;
        let mut var_qgod_dn6: f64 = *var_qgod_dn6_slot;
        let mut var_qgod_dn8: f64 = *var_qgod_dn8_slot;
        let mut var_qgos: f64 = *var_qgos_slot;
        let mut var_qgos_dn0: f64 = *var_qgos_dn0_slot;
        let mut var_qgos_dn10: f64 = *var_qgos_dn10_slot;
        let mut var_qgos_dn11: f64 = *var_qgos_dn11_slot;
        let mut var_qgos_dn12: f64 = *var_qgos_dn12_slot;
        let mut var_qgos_dn2: f64 = *var_qgos_dn2_slot;
        let mut var_qgos_dn4: f64 = *var_qgos_dn4_slot;
        let mut var_qgos_dn5: f64 = *var_qgos_dn5_slot;
        let mut var_qgos_dn6: f64 = *var_qgos_dn6_slot;
        let mut var_qgos_dn8: f64 = *var_qgos_dn8_slot;
        let mut var_qiuld: f64 = *var_qiuld_slot;
        let mut var_qiuld_dn0: f64 = *var_qiuld_dn0_slot;
        let mut var_qiuld_dn10: f64 = *var_qiuld_dn10_slot;
        let mut var_qiuld_dn11: f64 = *var_qiuld_dn11_slot;
        let mut var_qiuld_dn12: f64 = *var_qiuld_dn12_slot;
        let mut var_qiuld_dn2: f64 = *var_qiuld_dn2_slot;
        let mut var_qiuld_dn4: f64 = *var_qiuld_dn4_slot;
        let mut var_qiuld_dn5: f64 = *var_qiuld_dn5_slot;
        let mut var_qiuld_dn6: f64 = *var_qiuld_dn6_slot;
        let mut var_qiuld_dn8: f64 = *var_qiuld_dn8_slot;
        let mut var_qovd: f64 = *var_qovd_slot;
        let mut var_qovd_dn0: f64 = *var_qovd_dn0_slot;
        let mut var_qovd_dn10: f64 = *var_qovd_dn10_slot;
        let mut var_qovd_dn11: f64 = *var_qovd_dn11_slot;
        let mut var_qovd_dn12: f64 = *var_qovd_dn12_slot;
        let mut var_qovd_dn2: f64 = *var_qovd_dn2_slot;
        let mut var_qovd_dn4: f64 = *var_qovd_dn4_slot;
        let mut var_qovd_dn5: f64 = *var_qovd_dn5_slot;
        let mut var_qovd_dn6: f64 = *var_qovd_dn6_slot;
        let mut var_qovd_dn8: f64 = *var_qovd_dn8_slot;
        let mut var_qovs: f64 = *var_qovs_slot;
        let mut var_qovs_dn0: f64 = *var_qovs_dn0_slot;
        let mut var_qovs_dn10: f64 = *var_qovs_dn10_slot;
        let mut var_qovs_dn11: f64 = *var_qovs_dn11_slot;
        let mut var_qovs_dn12: f64 = *var_qovs_dn12_slot;
        let mut var_qovs_dn2: f64 = *var_qovs_dn2_slot;
        let mut var_qovs_dn4: f64 = *var_qovs_dn4_slot;
        let mut var_qovs_dn5: f64 = *var_qovs_dn5_slot;
        let mut var_qovs_dn6: f64 = *var_qovs_dn6_slot;
        let mut var_qovs_dn8: f64 = *var_qovs_dn8_slot;
        let mut var_qsuld: f64 = *var_qsuld_slot;
        let mut var_qsuld_dn0: f64 = *var_qsuld_dn0_slot;
        let mut var_qsuld_dn10: f64 = *var_qsuld_dn10_slot;
        let mut var_qsuld_dn11: f64 = *var_qsuld_dn11_slot;
        let mut var_qsuld_dn12: f64 = *var_qsuld_dn12_slot;
        let mut var_qsuld_dn2: f64 = *var_qsuld_dn2_slot;
        let mut var_qsuld_dn4: f64 = *var_qsuld_dn4_slot;
        let mut var_qsuld_dn5: f64 = *var_qsuld_dn5_slot;
        let mut var_qsuld_dn6: f64 = *var_qsuld_dn6_slot;
        let mut var_qsuld_dn8: f64 = *var_qsuld_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_xi0: f64 = *var_xi0_slot;
        let mut var_xi0_dn0: f64 = *var_xi0_dn0_slot;
        let mut var_xi0_dn10: f64 = *var_xi0_dn10_slot;
        let mut var_xi0_dn11: f64 = *var_xi0_dn11_slot;
        let mut var_xi0_dn12: f64 = *var_xi0_dn12_slot;
        let mut var_xi0_dn2: f64 = *var_xi0_dn2_slot;
        let mut var_xi0_dn4: f64 = *var_xi0_dn4_slot;
        let mut var_xi0_dn5: f64 = *var_xi0_dn5_slot;
        let mut var_xi0_dn6: f64 = *var_xi0_dn6_slot;
        let mut var_xi0_dn8: f64 = *var_xi0_dn8_slot;
        let mut var_xi0p12: f64 = *var_xi0p12_slot;
        let mut var_xi0p12_dn0: f64 = *var_xi0p12_dn0_slot;
        let mut var_xi0p12_dn10: f64 = *var_xi0p12_dn10_slot;
        let mut var_xi0p12_dn11: f64 = *var_xi0p12_dn11_slot;
        let mut var_xi0p12_dn12: f64 = *var_xi0p12_dn12_slot;
        let mut var_xi0p12_dn2: f64 = *var_xi0p12_dn2_slot;
        let mut var_xi0p12_dn4: f64 = *var_xi0p12_dn4_slot;
        let mut var_xi0p12_dn5: f64 = *var_xi0p12_dn5_slot;
        let mut var_xi0p12_dn6: f64 = *var_xi0p12_dn6_slot;
        let mut var_xi0p12_dn8: f64 = *var_xi0p12_dn8_slot;

        let assign20640_e26131: f64 = if var_chi < 5.0 { 1.0 } else { 0.0 };
        var_guard360 = assign20640_e26131;

        let (assign20680_e26178, assign20680_e26178_d_n0, assign20680_e26178_d_n2, assign20680_e26178_d_n4, assign20680_e26178_d_n5, assign20680_e26178_d_n6, assign20680_e26178_d_n8, assign20680_e26178_d_n10, assign20680_e26178_d_n11, assign20680_e26178_d_n12,) = {
    if ((((var_guard327 != 0.0) && (var_guard346 == 0.0)) && (var_guard353 != 0.0)) && (var_guard360 != 0.0)) {
        let assign20680_e26172: f64 = (var_fb * var_fb);
        let assign20680_e26175: f64 = (10.0 * 2.220446049250313e-16);
        let assign20680_e26176: f64 = (assign20680_e26172 + assign20680_e26175);
        (assign20680_e26176, ((var_fb_dn0 * var_fb) + (var_fb * var_fb_dn0)), ((var_fb_dn2 * var_fb) + (var_fb * var_fb_dn2)), ((var_fb_dn4 * var_fb) + (var_fb * var_fb_dn4)), ((var_fb_dn5 * var_fb) + (var_fb * var_fb_dn5)), ((var_fb_dn6 * var_fb) + (var_fb * var_fb_dn6)), ((var_fb_dn8 * var_fb) + (var_fb * var_fb_dn8)), ((var_fb_dn10 * var_fb) + (var_fb * var_fb_dn10)), ((var_fb_dn11 * var_fb) + (var_fb * var_fb_dn11)), ((var_fb_dn12 * var_fb) + (var_fb * var_fb_dn12)),)
    } else {
        (var_xi0, var_xi0_dn0, var_xi0_dn2, var_xi0_dn4, var_xi0_dn5, var_xi0_dn6, var_xi0_dn8, var_xi0_dn10, var_xi0_dn11, var_xi0_dn12,)
    }
};
        var_xi0 = assign20680_e26178;
        var_xi0_dn0 = assign20680_e26178_d_n0;
        var_xi0_dn2 = assign20680_e26178_d_n2;
        var_xi0_dn4 = assign20680_e26178_d_n4;
        var_xi0_dn5 = assign20680_e26178_d_n5;
        var_xi0_dn6 = assign20680_e26178_d_n6;
        var_xi0_dn8 = assign20680_e26178_d_n8;
        var_xi0_dn10 = assign20680_e26178_d_n10;
        var_xi0_dn11 = assign20680_e26178_d_n11;
        var_xi0_dn12 = assign20680_e26178_d_n12;

        let (assign20690_e26193, assign20690_e26193_d_n0, assign20690_e26193_d_n2, assign20690_e26193_d_n4, assign20690_e26193_d_n5, assign20690_e26193_d_n6, assign20690_e26193_d_n8, assign20690_e26193_d_n10, assign20690_e26193_d_n11, assign20690_e26193_d_n12,) = {
    if ((((var_guard327 != 0.0) && (var_guard346 == 0.0)) && (var_guard353 != 0.0)) && (var_guard360 != 0.0)) {
        let assign20690_e26190: f64 = (10.0 * 2.220446049250313e-16);
        let assign20690_e26191: f64 = (var_fb + assign20690_e26190);
        (assign20690_e26191, var_fb_dn0, var_fb_dn2, var_fb_dn4, var_fb_dn5, var_fb_dn6, var_fb_dn8, var_fb_dn10, var_fb_dn11, var_fb_dn12,)
    } else {
        (var_xi0p12, var_xi0p12_dn0, var_xi0p12_dn2, var_xi0p12_dn4, var_xi0p12_dn5, var_xi0p12_dn6, var_xi0p12_dn8, var_xi0p12_dn10, var_xi0p12_dn11, var_xi0p12_dn12,)
    }
};
        var_xi0p12 = assign20690_e26193;
        var_xi0p12_dn0 = assign20690_e26193_d_n0;
        var_xi0p12_dn2 = assign20690_e26193_d_n2;
        var_xi0p12_dn4 = assign20690_e26193_d_n4;
        var_xi0p12_dn5 = assign20690_e26193_d_n5;
        var_xi0p12_dn6 = assign20690_e26193_d_n6;
        var_xi0p12_dn8 = assign20690_e26193_d_n8;
        var_xi0p12_dn10 = assign20690_e26193_d_n10;
        var_xi0p12_dn11 = assign20690_e26193_d_n11;
        var_xi0p12_dn12 = assign20690_e26193_d_n12;

        let (assign20710_e26219, assign20710_e26219_d_n0, assign20710_e26219_d_n2, assign20710_e26219_d_n4, assign20710_e26219_d_n5, assign20710_e26219_d_n6, assign20710_e26219_d_n8, assign20710_e26219_d_n10, assign20710_e26219_d_n11, assign20710_e26219_d_n12,) = {
    if ((((var_guard327 != 0.0) && (var_guard346 == 0.0)) && (var_guard353 != 0.0)) && (var_guard360 == 0.0)) {
        let assign20710_e26217: f64 = (var_chi - 1.0);
        (assign20710_e26217, var_chi_dn0, var_chi_dn2, var_chi_dn4, var_chi_dn5, var_chi_dn6, var_chi_dn8, var_chi_dn10, var_chi_dn11, var_chi_dn12,)
    } else {
        (var_xi0, var_xi0_dn0, var_xi0_dn2, var_xi0_dn4, var_xi0_dn5, var_xi0_dn6, var_xi0_dn8, var_xi0_dn10, var_xi0_dn11, var_xi0_dn12,)
    }
};
        var_xi0 = assign20710_e26219;
        var_xi0_dn0 = assign20710_e26219_d_n0;
        var_xi0_dn2 = assign20710_e26219_d_n2;
        var_xi0_dn4 = assign20710_e26219_d_n4;
        var_xi0_dn5 = assign20710_e26219_d_n5;
        var_xi0_dn6 = assign20710_e26219_d_n6;
        var_xi0_dn8 = assign20710_e26219_d_n8;
        var_xi0_dn10 = assign20710_e26219_d_n10;
        var_xi0_dn11 = assign20710_e26219_d_n11;
        var_xi0_dn12 = assign20710_e26219_d_n12;

        let (assign20720_e26232, assign20720_e26232_d_n0, assign20720_e26232_d_n2, assign20720_e26232_d_n4, assign20720_e26232_d_n5, assign20720_e26232_d_n6, assign20720_e26232_d_n8, assign20720_e26232_d_n10, assign20720_e26232_d_n11, assign20720_e26232_d_n12,) = {
    if ((((var_guard327 != 0.0) && (var_guard346 == 0.0)) && (var_guard353 != 0.0)) && (var_guard360 == 0.0)) {
        let assign20720_e26230: f64 = (var_xi0).sqrt();
        (assign20720_e26230, (var_xi0_dn0 / (2.0 * assign20720_e26230)), (var_xi0_dn2 / (2.0 * assign20720_e26230)), (var_xi0_dn4 / (2.0 * assign20720_e26230)), (var_xi0_dn5 / (2.0 * assign20720_e26230)), (var_xi0_dn6 / (2.0 * assign20720_e26230)), (var_xi0_dn8 / (2.0 * assign20720_e26230)), (var_xi0_dn10 / (2.0 * assign20720_e26230)), (var_xi0_dn11 / (2.0 * assign20720_e26230)), (var_xi0_dn12 / (2.0 * assign20720_e26230)),)
    } else {
        (var_xi0p12, var_xi0p12_dn0, var_xi0p12_dn2, var_xi0p12_dn4, var_xi0p12_dn5, var_xi0p12_dn6, var_xi0p12_dn8, var_xi0p12_dn10, var_xi0p12_dn11, var_xi0p12_dn12,)
    }
};
        var_xi0p12 = assign20720_e26232;
        var_xi0p12_dn0 = assign20720_e26232_d_n0;
        var_xi0p12_dn2 = assign20720_e26232_d_n2;
        var_xi0p12_dn4 = assign20720_e26232_d_n4;
        var_xi0p12_dn5 = assign20720_e26232_d_n5;
        var_xi0p12_dn6 = assign20720_e26232_d_n6;
        var_xi0p12_dn8 = assign20720_e26232_d_n8;
        var_xi0p12_dn10 = assign20720_e26232_d_n10;
        var_xi0p12_dn11 = assign20720_e26232_d_n11;
        var_xi0p12_dn12 = assign20720_e26232_d_n12;

        let (assign20730_e26243, assign20730_e26243_d_n0, assign20730_e26243_d_n2, assign20730_e26243_d_n4, assign20730_e26243_d_n5, assign20730_e26243_d_n6, assign20730_e26243_d_n8, assign20730_e26243_d_n10, assign20730_e26243_d_n11, assign20730_e26243_d_n12,) = {
    if (((var_guard327 != 0.0) && (var_guard346 == 0.0)) && (var_guard353 != 0.0)) {
        let assign20730_e26241: f64 = (var_cnst0over * var_xi0p12);
        (assign20730_e26241, ((var_cnst0over_dn0 * var_xi0p12) + (var_cnst0over * var_xi0p12_dn0)), ((var_cnst0over_dn2 * var_xi0p12) + (var_cnst0over * var_xi0p12_dn2)), ((var_cnst0over_dn4 * var_xi0p12) + (var_cnst0over * var_xi0p12_dn4)), ((var_cnst0over_dn5 * var_xi0p12) + (var_cnst0over * var_xi0p12_dn5)), ((var_cnst0over_dn6 * var_xi0p12) + (var_cnst0over * var_xi0p12_dn6)), ((var_cnst0over_dn8 * var_xi0p12) + (var_cnst0over * var_xi0p12_dn8)), ((var_cnst0over_dn10 * var_xi0p12) + (var_cnst0over * var_xi0p12_dn10)), ((var_cnst0over_dn11 * var_xi0p12) + (var_cnst0over * var_xi0p12_dn11)), ((var_cnst0over_dn12 * var_xi0p12) + (var_cnst0over * var_xi0p12_dn12)),)
    } else {
        (var_qbuld, var_qbuld_dn0, var_qbuld_dn2, var_qbuld_dn4, var_qbuld_dn5, var_qbuld_dn6, var_qbuld_dn8, var_qbuld_dn10, var_qbuld_dn11, var_qbuld_dn12,)
    }
};
        var_qbuld = assign20730_e26243;
        var_qbuld_dn0 = assign20730_e26243_d_n0;
        var_qbuld_dn2 = assign20730_e26243_d_n2;
        var_qbuld_dn4 = assign20730_e26243_d_n4;
        var_qbuld_dn5 = assign20730_e26243_d_n5;
        var_qbuld_dn6 = assign20730_e26243_d_n6;
        var_qbuld_dn8 = assign20730_e26243_d_n8;
        var_qbuld_dn10 = assign20730_e26243_d_n10;
        var_qbuld_dn11 = assign20730_e26243_d_n11;
        var_qbuld_dn12 = assign20730_e26243_d_n12;

        let (assign20740_e26256, assign20740_e26256_d_n0, assign20740_e26256_d_n2, assign20740_e26256_d_n4, assign20740_e26256_d_n5, assign20740_e26256_d_n6, assign20740_e26256_d_n8, assign20740_e26256_d_n10, assign20740_e26256_d_n11, assign20740_e26256_d_n12,) = {
    if (((var_guard327 != 0.0) && (var_guard346 == 0.0)) && (var_guard353 != 0.0)) {
        let assign20740_e26253: f64 = (var_fs02 + var_xi0p12);
        let assign20740_e26254: f64 = (1.0 / assign20740_e26253);
        (assign20740_e26254, (-((var_fs02_dn0 + var_xi0p12_dn0) / (assign20740_e26253 * assign20740_e26253))), (-((var_fs02_dn2 + var_xi0p12_dn2) / (assign20740_e26253 * assign20740_e26253))), (-((var_fs02_dn4 + var_xi0p12_dn4) / (assign20740_e26253 * assign20740_e26253))), (-((var_fs02_dn5 + var_xi0p12_dn5) / (assign20740_e26253 * assign20740_e26253))), (-((var_fs02_dn6 + var_xi0p12_dn6) / (assign20740_e26253 * assign20740_e26253))), (-((var_fs02_dn8 + var_xi0p12_dn8) / (assign20740_e26253 * assign20740_e26253))), (-((var_fs02_dn10 + var_xi0p12_dn10) / (assign20740_e26253 * assign20740_e26253))), (-((var_fs02_dn11 + var_xi0p12_dn11) / (assign20740_e26253 * assign20740_e26253))), (-((var_fs02_dn12 + var_xi0p12_dn12) / (assign20740_e26253 * assign20740_e26253))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign20740_e26256;
        var_t1_dn0 = assign20740_e26256_d_n0;
        var_t1_dn2 = assign20740_e26256_d_n2;
        var_t1_dn4 = assign20740_e26256_d_n4;
        var_t1_dn5 = assign20740_e26256_d_n5;
        var_t1_dn6 = assign20740_e26256_d_n6;
        var_t1_dn8 = assign20740_e26256_d_n8;
        var_t1_dn10 = assign20740_e26256_d_n10;
        var_t1_dn11 = assign20740_e26256_d_n11;
        var_t1_dn12 = assign20740_e26256_d_n12;

        let (assign20750_e26269, assign20750_e26269_d_n0, assign20750_e26269_d_n2, assign20750_e26269_d_n4, assign20750_e26269_d_n5, assign20750_e26269_d_n6, assign20750_e26269_d_n8, assign20750_e26269_d_n10, assign20750_e26269_d_n11, assign20750_e26269_d_n12,) = {
    if (((var_guard327 != 0.0) && (var_guard346 == 0.0)) && (var_guard353 != 0.0)) {
        let assign20750_e26265: f64 = (var_cnst0over * var_fs01);
        let assign20750_e26267: f64 = (assign20750_e26265 * var_t1);
        (assign20750_e26267, ((((var_cnst0over_dn0 * var_fs01) + (var_cnst0over * var_fs01_dn0)) * var_t1) + (assign20750_e26265 * var_t1_dn0)), ((((var_cnst0over_dn2 * var_fs01) + (var_cnst0over * var_fs01_dn2)) * var_t1) + (assign20750_e26265 * var_t1_dn2)), ((((var_cnst0over_dn4 * var_fs01) + (var_cnst0over * var_fs01_dn4)) * var_t1) + (assign20750_e26265 * var_t1_dn4)), ((((var_cnst0over_dn5 * var_fs01) + (var_cnst0over * var_fs01_dn5)) * var_t1) + (assign20750_e26265 * var_t1_dn5)), ((((var_cnst0over_dn6 * var_fs01) + (var_cnst0over * var_fs01_dn6)) * var_t1) + (assign20750_e26265 * var_t1_dn6)), ((((var_cnst0over_dn8 * var_fs01) + (var_cnst0over * var_fs01_dn8)) * var_t1) + (assign20750_e26265 * var_t1_dn8)), ((((var_cnst0over_dn10 * var_fs01) + (var_cnst0over * var_fs01_dn10)) * var_t1) + (assign20750_e26265 * var_t1_dn10)), ((((var_cnst0over_dn11 * var_fs01) + (var_cnst0over * var_fs01_dn11)) * var_t1) + (assign20750_e26265 * var_t1_dn11)), ((((var_cnst0over_dn12 * var_fs01) + (var_cnst0over * var_fs01_dn12)) * var_t1) + (assign20750_e26265 * var_t1_dn12)),)
    } else {
        (var_qiuld, var_qiuld_dn0, var_qiuld_dn2, var_qiuld_dn4, var_qiuld_dn5, var_qiuld_dn6, var_qiuld_dn8, var_qiuld_dn10, var_qiuld_dn11, var_qiuld_dn12,)
    }
};
        var_qiuld = assign20750_e26269;
        var_qiuld_dn0 = assign20750_e26269_d_n0;
        var_qiuld_dn2 = assign20750_e26269_d_n2;
        var_qiuld_dn4 = assign20750_e26269_d_n4;
        var_qiuld_dn5 = assign20750_e26269_d_n5;
        var_qiuld_dn6 = assign20750_e26269_d_n6;
        var_qiuld_dn8 = assign20750_e26269_d_n8;
        var_qiuld_dn10 = assign20750_e26269_d_n10;
        var_qiuld_dn11 = assign20750_e26269_d_n11;
        var_qiuld_dn12 = assign20750_e26269_d_n12;

        let (assign20760_e26280, assign20760_e26280_d_n0, assign20760_e26280_d_n2, assign20760_e26280_d_n4, assign20760_e26280_d_n5, assign20760_e26280_d_n6, assign20760_e26280_d_n8, assign20760_e26280_d_n10, assign20760_e26280_d_n11, assign20760_e26280_d_n12,) = {
    if (((var_guard327 != 0.0) && (var_guard346 == 0.0)) && (var_guard353 != 0.0)) {
        let assign20760_e26278: f64 = (var_qbuld + var_qiuld);
        (assign20760_e26278, (var_qbuld_dn0 + var_qiuld_dn0), (var_qbuld_dn2 + var_qiuld_dn2), (var_qbuld_dn4 + var_qiuld_dn4), (var_qbuld_dn5 + var_qiuld_dn5), (var_qbuld_dn6 + var_qiuld_dn6), (var_qbuld_dn8 + var_qiuld_dn8), (var_qbuld_dn10 + var_qiuld_dn10), (var_qbuld_dn11 + var_qiuld_dn11), (var_qbuld_dn12 + var_qiuld_dn12),)
    } else {
        (var_qsuld, var_qsuld_dn0, var_qsuld_dn2, var_qsuld_dn4, var_qsuld_dn5, var_qsuld_dn6, var_qsuld_dn8, var_qsuld_dn10, var_qsuld_dn11, var_qsuld_dn12,)
    }
};
        var_qsuld = assign20760_e26280;
        var_qsuld_dn0 = assign20760_e26280_d_n0;
        var_qsuld_dn2 = assign20760_e26280_d_n2;
        var_qsuld_dn4 = assign20760_e26280_d_n4;
        var_qsuld_dn5 = assign20760_e26280_d_n5;
        var_qsuld_dn6 = assign20760_e26280_d_n6;
        var_qsuld_dn8 = assign20760_e26280_d_n8;
        var_qsuld_dn10 = assign20760_e26280_d_n10;
        var_qsuld_dn11 = assign20760_e26280_d_n11;
        var_qsuld_dn12 = assign20760_e26280_d_n12;

        let (assign20770_e26286, assign20770_e26286_d_n0, assign20770_e26286_d_n2, assign20770_e26286_d_n4, assign20770_e26286_d_n5, assign20770_e26286_d_n6, assign20770_e26286_d_n8, assign20770_e26286_d_n10, assign20770_e26286_d_n11, assign20770_e26286_d_n12,) = {
    if (var_guard327 != 0.0) {
        let assign20770_e26284: f64 = (var_qsuld - var_qbuld);
        (assign20770_e26284, (var_qsuld_dn0 - var_qbuld_dn0), (var_qsuld_dn2 - var_qbuld_dn2), (var_qsuld_dn4 - var_qbuld_dn4), (var_qsuld_dn5 - var_qbuld_dn5), (var_qsuld_dn6 - var_qbuld_dn6), (var_qsuld_dn8 - var_qbuld_dn8), (var_qsuld_dn10 - var_qbuld_dn10), (var_qsuld_dn11 - var_qbuld_dn11), (var_qsuld_dn12 - var_qbuld_dn12),)
    } else {
        (var_qiuld, var_qiuld_dn0, var_qiuld_dn2, var_qiuld_dn4, var_qiuld_dn5, var_qiuld_dn6, var_qiuld_dn8, var_qiuld_dn10, var_qiuld_dn11, var_qiuld_dn12,)
    }
};
        var_qiuld = assign20770_e26286;
        var_qiuld_dn0 = assign20770_e26286_d_n0;
        var_qiuld_dn2 = assign20770_e26286_d_n2;
        var_qiuld_dn4 = assign20770_e26286_d_n4;
        var_qiuld_dn5 = assign20770_e26286_d_n5;
        var_qiuld_dn6 = assign20770_e26286_d_n6;
        var_qiuld_dn8 = assign20770_e26286_d_n8;
        var_qiuld_dn10 = assign20770_e26286_d_n10;
        var_qiuld_dn11 = assign20770_e26286_d_n11;
        var_qiuld_dn12 = assign20770_e26286_d_n12;

        let (assign20780_e26292, assign20780_e26292_d_n0, assign20780_e26292_d_n2, assign20780_e26292_d_n4, assign20780_e26292_d_n5, assign20780_e26292_d_n6, assign20780_e26292_d_n8, assign20780_e26292_d_n10, assign20780_e26292_d_n11, assign20780_e26292_d_n12,) = {
    if (var_guard327 != 0.0) {
        let assign20780_e26290: f64 = (var_weffcv_nf * var_lov);
        (assign20780_e26290, (var_weffcv_nf_dn0 * var_lov), (var_weffcv_nf_dn2 * var_lov), (var_weffcv_nf_dn4 * var_lov), (var_weffcv_nf_dn5 * var_lov), (var_weffcv_nf_dn6 * var_lov), (var_weffcv_nf_dn8 * var_lov), (var_weffcv_nf_dn10 * var_lov), (var_weffcv_nf_dn11 * var_lov), (var_weffcv_nf_dn12 * var_lov),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
        var_t4 = assign20780_e26292;
        var_t4_dn0 = assign20780_e26292_d_n0;
        var_t4_dn2 = assign20780_e26292_d_n2;
        var_t4_dn4 = assign20780_e26292_d_n4;
        var_t4_dn5 = assign20780_e26292_d_n5;
        var_t4_dn6 = assign20780_e26292_d_n6;
        var_t4_dn8 = assign20780_e26292_d_n8;
        var_t4_dn10 = assign20780_e26292_d_n10;
        var_t4_dn11 = assign20780_e26292_d_n11;
        var_t4_dn12 = assign20780_e26292_d_n12;

        let (assign20790_e26300, assign20790_e26300_d_n0, assign20790_e26300_d_n2, assign20790_e26300_d_n4, assign20790_e26300_d_n5, assign20790_e26300_d_n6, assign20790_e26300_d_n8, assign20790_e26300_d_n10, assign20790_e26300_d_n11, assign20790_e26300_d_n12,) = {
    if ((var_guard327 != 0.0) && (var_flg_overs != 0.0)) {
        let assign20790_e26298: f64 = (var_t4 * var_qsuld);
        (assign20790_e26298, ((var_t4_dn0 * var_qsuld) + (var_t4 * var_qsuld_dn0)), ((var_t4_dn2 * var_qsuld) + (var_t4 * var_qsuld_dn2)), ((var_t4_dn4 * var_qsuld) + (var_t4 * var_qsuld_dn4)), ((var_t4_dn5 * var_qsuld) + (var_t4 * var_qsuld_dn5)), ((var_t4_dn6 * var_qsuld) + (var_t4 * var_qsuld_dn6)), ((var_t4_dn8 * var_qsuld) + (var_t4 * var_qsuld_dn8)), ((var_t4_dn10 * var_qsuld) + (var_t4 * var_qsuld_dn10)), ((var_t4_dn11 * var_qsuld) + (var_t4 * var_qsuld_dn11)), ((var_t4_dn12 * var_qsuld) + (var_t4 * var_qsuld_dn12)),)
    } else {
        (var_qovs, var_qovs_dn0, var_qovs_dn2, var_qovs_dn4, var_qovs_dn5, var_qovs_dn6, var_qovs_dn8, var_qovs_dn10, var_qovs_dn11, var_qovs_dn12,)
    }
};
        var_qovs = assign20790_e26300;
        var_qovs_dn0 = assign20790_e26300_d_n0;
        var_qovs_dn2 = assign20790_e26300_d_n2;
        var_qovs_dn4 = assign20790_e26300_d_n4;
        var_qovs_dn5 = assign20790_e26300_d_n5;
        var_qovs_dn6 = assign20790_e26300_d_n6;
        var_qovs_dn8 = assign20790_e26300_d_n8;
        var_qovs_dn10 = assign20790_e26300_d_n10;
        var_qovs_dn11 = assign20790_e26300_d_n11;
        var_qovs_dn12 = assign20790_e26300_d_n12;

        let (assign20800_e26308, assign20800_e26308_d_n0, assign20800_e26308_d_n2, assign20800_e26308_d_n4, assign20800_e26308_d_n5, assign20800_e26308_d_n6, assign20800_e26308_d_n8, assign20800_e26308_d_n10, assign20800_e26308_d_n11, assign20800_e26308_d_n12,) = {
    if ((var_guard327 != 0.0) && (var_flg_overs != 0.0)) {
        let assign20800_e26306: f64 = (var_t4 * var_qbuld);
        (assign20800_e26306, ((var_t4_dn0 * var_qbuld) + (var_t4 * var_qbuld_dn0)), ((var_t4_dn2 * var_qbuld) + (var_t4 * var_qbuld_dn2)), ((var_t4_dn4 * var_qbuld) + (var_t4 * var_qbuld_dn4)), ((var_t4_dn5 * var_qbuld) + (var_t4 * var_qbuld_dn5)), ((var_t4_dn6 * var_qbuld) + (var_t4 * var_qbuld_dn6)), ((var_t4_dn8 * var_qbuld) + (var_t4 * var_qbuld_dn8)), ((var_t4_dn10 * var_qbuld) + (var_t4 * var_qbuld_dn10)), ((var_t4_dn11 * var_qbuld) + (var_t4 * var_qbuld_dn11)), ((var_t4_dn12 * var_qbuld) + (var_t4 * var_qbuld_dn12)),)
    } else {
        (var_qbsld, var_qbsld_dn0, var_qbsld_dn2, var_qbsld_dn4, var_qbsld_dn5, var_qbsld_dn6, var_qbsld_dn8, var_qbsld_dn10, var_qbsld_dn11, var_qbsld_dn12,)
    }
};
        var_qbsld = assign20800_e26308;
        var_qbsld_dn0 = assign20800_e26308_d_n0;
        var_qbsld_dn2 = assign20800_e26308_d_n2;
        var_qbsld_dn4 = assign20800_e26308_d_n4;
        var_qbsld_dn5 = assign20800_e26308_d_n5;
        var_qbsld_dn6 = assign20800_e26308_d_n6;
        var_qbsld_dn8 = assign20800_e26308_d_n8;
        var_qbsld_dn10 = assign20800_e26308_d_n10;
        var_qbsld_dn11 = assign20800_e26308_d_n11;
        var_qbsld_dn12 = assign20800_e26308_d_n12;

        let (assign20810_e26316, assign20810_e26316_d_n0, assign20810_e26316_d_n2, assign20810_e26316_d_n4, assign20810_e26316_d_n5, assign20810_e26316_d_n6, assign20810_e26316_d_n8, assign20810_e26316_d_n10, assign20810_e26316_d_n11, assign20810_e26316_d_n12,) = {
    if ((var_guard327 != 0.0) && (var_flg_overd != 0.0)) {
        let assign20810_e26314: f64 = (var_t4 * var_qsuld);
        (assign20810_e26314, ((var_t4_dn0 * var_qsuld) + (var_t4 * var_qsuld_dn0)), ((var_t4_dn2 * var_qsuld) + (var_t4 * var_qsuld_dn2)), ((var_t4_dn4 * var_qsuld) + (var_t4 * var_qsuld_dn4)), ((var_t4_dn5 * var_qsuld) + (var_t4 * var_qsuld_dn5)), ((var_t4_dn6 * var_qsuld) + (var_t4 * var_qsuld_dn6)), ((var_t4_dn8 * var_qsuld) + (var_t4 * var_qsuld_dn8)), ((var_t4_dn10 * var_qsuld) + (var_t4 * var_qsuld_dn10)), ((var_t4_dn11 * var_qsuld) + (var_t4 * var_qsuld_dn11)), ((var_t4_dn12 * var_qsuld) + (var_t4 * var_qsuld_dn12)),)
    } else {
        (var_qovd, var_qovd_dn0, var_qovd_dn2, var_qovd_dn4, var_qovd_dn5, var_qovd_dn6, var_qovd_dn8, var_qovd_dn10, var_qovd_dn11, var_qovd_dn12,)
    }
};
        var_qovd = assign20810_e26316;
        var_qovd_dn0 = assign20810_e26316_d_n0;
        var_qovd_dn2 = assign20810_e26316_d_n2;
        var_qovd_dn4 = assign20810_e26316_d_n4;
        var_qovd_dn5 = assign20810_e26316_d_n5;
        var_qovd_dn6 = assign20810_e26316_d_n6;
        var_qovd_dn8 = assign20810_e26316_d_n8;
        var_qovd_dn10 = assign20810_e26316_d_n10;
        var_qovd_dn11 = assign20810_e26316_d_n11;
        var_qovd_dn12 = assign20810_e26316_d_n12;

        let (assign20820_e26324, assign20820_e26324_d_n0, assign20820_e26324_d_n2, assign20820_e26324_d_n4, assign20820_e26324_d_n5, assign20820_e26324_d_n6, assign20820_e26324_d_n8, assign20820_e26324_d_n10, assign20820_e26324_d_n11, assign20820_e26324_d_n12,) = {
    if ((var_guard327 != 0.0) && (var_flg_overd != 0.0)) {
        let assign20820_e26322: f64 = (var_t4 * var_qbuld);
        (assign20820_e26322, ((var_t4_dn0 * var_qbuld) + (var_t4 * var_qbuld_dn0)), ((var_t4_dn2 * var_qbuld) + (var_t4 * var_qbuld_dn2)), ((var_t4_dn4 * var_qbuld) + (var_t4 * var_qbuld_dn4)), ((var_t4_dn5 * var_qbuld) + (var_t4 * var_qbuld_dn5)), ((var_t4_dn6 * var_qbuld) + (var_t4 * var_qbuld_dn6)), ((var_t4_dn8 * var_qbuld) + (var_t4 * var_qbuld_dn8)), ((var_t4_dn10 * var_qbuld) + (var_t4 * var_qbuld_dn10)), ((var_t4_dn11 * var_qbuld) + (var_t4 * var_qbuld_dn11)), ((var_t4_dn12 * var_qbuld) + (var_t4 * var_qbuld_dn12)),)
    } else {
        (var_qbdld, var_qbdld_dn0, var_qbdld_dn2, var_qbdld_dn4, var_qbdld_dn5, var_qbdld_dn6, var_qbdld_dn8, var_qbdld_dn10, var_qbdld_dn11, var_qbdld_dn12,)
    }
};
        var_qbdld = assign20820_e26324;
        var_qbdld_dn0 = assign20820_e26324_d_n0;
        var_qbdld_dn2 = assign20820_e26324_d_n2;
        var_qbdld_dn4 = assign20820_e26324_d_n4;
        var_qbdld_dn5 = assign20820_e26324_d_n5;
        var_qbdld_dn6 = assign20820_e26324_d_n6;
        var_qbdld_dn8 = assign20820_e26324_d_n8;
        var_qbdld_dn10 = assign20820_e26324_d_n10;
        var_qbdld_dn11 = assign20820_e26324_d_n11;
        var_qbdld_dn12 = assign20820_e26324_d_n12;

        let (assign20830_e26334,) = {
    if (var_guard327 != 0.0) {
        let assign20830_e26328: f64 = (var_modervs * var_cgso_given);
        let assign20830_e26331: f64 = (var_modenml * var_cgdo_given);
        let assign20830_e26332: f64 = (assign20830_e26328 + assign20830_e26331);
        (assign20830_e26332,)
    } else {
        (var_flg_overgiven,)
    }
};
        var_flg_overgiven = assign20830_e26334;

        let (assign20840_e26346, assign20840_e26346_d_n0, assign20840_e26346_d_n2, assign20840_e26346_d_n4, assign20840_e26346_d_n5, assign20840_e26346_d_n6, assign20840_e26346_d_n8, assign20840_e26346_d_n10, assign20840_e26346_d_n11, assign20840_e26346_d_n12,) = {
    if ((var_guard327 != 0.0) && (var_flg_overgiven != 0.0)) {
        let assign20840_e26340: f64 = (var_modervs * p.p174);
        let assign20840_e26343: f64 = (var_modenml * p.p173);
        let assign20840_e26344: f64 = (assign20840_e26340 + assign20840_e26343);
        (assign20840_e26344, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn4, var_cgdoe_dn5, var_cgdoe_dn6, var_cgdoe_dn8, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12,)
    }
};
        var_cgdoe = assign20840_e26346;
        var_cgdoe_dn0 = assign20840_e26346_d_n0;
        var_cgdoe_dn2 = assign20840_e26346_d_n2;
        var_cgdoe_dn4 = assign20840_e26346_d_n4;
        var_cgdoe_dn5 = assign20840_e26346_d_n5;
        var_cgdoe_dn6 = assign20840_e26346_d_n6;
        var_cgdoe_dn8 = assign20840_e26346_d_n8;
        var_cgdoe_dn10 = assign20840_e26346_d_n10;
        var_cgdoe_dn11 = assign20840_e26346_d_n11;
        var_cgdoe_dn12 = assign20840_e26346_d_n12;

        let (assign20850_e26355, assign20850_e26355_d_n0, assign20850_e26355_d_n2, assign20850_e26355_d_n4, assign20850_e26355_d_n5, assign20850_e26355_d_n6, assign20850_e26355_d_n8, assign20850_e26355_d_n10, assign20850_e26355_d_n11, assign20850_e26355_d_n12,) = {
    if ((var_guard327 != 0.0) && (var_flg_overgiven != 0.0)) {
        let assign20850_e26352: f64 = (-var_weffcv_nf);
        let assign20850_e26353: f64 = (var_cgdoe * assign20850_e26352);
        (assign20850_e26353, ((var_cgdoe_dn0 * assign20850_e26352) + (var_cgdoe * (-var_weffcv_nf_dn0))), ((var_cgdoe_dn2 * assign20850_e26352) + (var_cgdoe * (-var_weffcv_nf_dn2))), ((var_cgdoe_dn4 * assign20850_e26352) + (var_cgdoe * (-var_weffcv_nf_dn4))), ((var_cgdoe_dn5 * assign20850_e26352) + (var_cgdoe * (-var_weffcv_nf_dn5))), ((var_cgdoe_dn6 * assign20850_e26352) + (var_cgdoe * (-var_weffcv_nf_dn6))), ((var_cgdoe_dn8 * assign20850_e26352) + (var_cgdoe * (-var_weffcv_nf_dn8))), ((var_cgdoe_dn10 * assign20850_e26352) + (var_cgdoe * (-var_weffcv_nf_dn10))), ((var_cgdoe_dn11 * assign20850_e26352) + (var_cgdoe * (-var_weffcv_nf_dn11))), ((var_cgdoe_dn12 * assign20850_e26352) + (var_cgdoe * (-var_weffcv_nf_dn12))),)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn4, var_cgdoe_dn5, var_cgdoe_dn6, var_cgdoe_dn8, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12,)
    }
};
        var_cgdoe = assign20850_e26355;
        var_cgdoe_dn0 = assign20850_e26355_d_n0;
        var_cgdoe_dn2 = assign20850_e26355_d_n2;
        var_cgdoe_dn4 = assign20850_e26355_d_n4;
        var_cgdoe_dn5 = assign20850_e26355_d_n5;
        var_cgdoe_dn6 = assign20850_e26355_d_n6;
        var_cgdoe_dn8 = assign20850_e26355_d_n8;
        var_cgdoe_dn10 = assign20850_e26355_d_n10;
        var_cgdoe_dn11 = assign20850_e26355_d_n11;
        var_cgdoe_dn12 = assign20850_e26355_d_n12;

        let (assign20860_e26368, assign20860_e26368_d_n0, assign20860_e26368_d_n2, assign20860_e26368_d_n4, assign20860_e26368_d_n5, assign20860_e26368_d_n6, assign20860_e26368_d_n8, assign20860_e26368_d_n10, assign20860_e26368_d_n11, assign20860_e26368_d_n12,) = {
    if ((var_guard327 != 0.0) && (var_flg_overgiven != 0.0)) {
        let assign20860_e26361: f64 = (-var_cgdoe);
        let assign20860_e26364: f64 = (var_vgs - var_vds);
        let assign20860_e26365: f64 = (assign20860_e26361 * assign20860_e26364);
        let assign20860_e26366: f64 = (var_qgod + assign20860_e26365);
        (assign20860_e26366, (var_qgod_dn0 + (((-var_cgdoe_dn0) * assign20860_e26364) + (assign20860_e26361 * (-var_vds_dn0)))), (var_qgod_dn2 + (((-var_cgdoe_dn2) * assign20860_e26364) + (assign20860_e26361 * (-var_vds_dn2)))), (var_qgod_dn4 + (((-var_cgdoe_dn4) * assign20860_e26364) + (assign20860_e26361 * (-var_vds_dn4)))), (var_qgod_dn5 + (((-var_cgdoe_dn5) * assign20860_e26364) + (assign20860_e26361 * (var_vgs_dn5 - var_vds_dn5)))), (var_qgod_dn6 + (((-var_cgdoe_dn6) * assign20860_e26364) + (assign20860_e26361 * (-var_vds_dn6)))), (var_qgod_dn8 + (((-var_cgdoe_dn8) * assign20860_e26364) + (assign20860_e26361 * (-var_vds_dn8)))), (var_qgod_dn10 + (((-var_cgdoe_dn10) * assign20860_e26364) + (assign20860_e26361 * (-var_vds_dn10)))), (var_qgod_dn11 + (((-var_cgdoe_dn11) * assign20860_e26364) + (assign20860_e26361 * (var_vgs_dn11 - var_vds_dn11)))), (var_qgod_dn12 + (((-var_cgdoe_dn12) * assign20860_e26364) + (assign20860_e26361 * (var_vgs_dn12 - var_vds_dn12)))),)
    } else {
        (var_qgod, var_qgod_dn0, var_qgod_dn2, var_qgod_dn4, var_qgod_dn5, var_qgod_dn6, var_qgod_dn8, var_qgod_dn10, var_qgod_dn11, var_qgod_dn12,)
    }
};
        var_qgod = assign20860_e26368;
        var_qgod_dn0 = assign20860_e26368_d_n0;
        var_qgod_dn2 = assign20860_e26368_d_n2;
        var_qgod_dn4 = assign20860_e26368_d_n4;
        var_qgod_dn5 = assign20860_e26368_d_n5;
        var_qgod_dn6 = assign20860_e26368_d_n6;
        var_qgod_dn8 = assign20860_e26368_d_n8;
        var_qgod_dn10 = assign20860_e26368_d_n10;
        var_qgod_dn11 = assign20860_e26368_d_n11;
        var_qgod_dn12 = assign20860_e26368_d_n12;

        let (assign20870_e26378,) = {
    if (var_guard327 != 0.0) {
        let assign20870_e26372: f64 = (var_modenml * var_cgso_given);
        let assign20870_e26375: f64 = (var_modervs * var_cgdo_given);
        let assign20870_e26376: f64 = (assign20870_e26372 + assign20870_e26375);
        (assign20870_e26376,)
    } else {
        (var_flg_overgiven,)
    }
};
        var_flg_overgiven = assign20870_e26378;

        let (assign20880_e26390, assign20880_e26390_d_n0, assign20880_e26390_d_n2, assign20880_e26390_d_n4, assign20880_e26390_d_n5, assign20880_e26390_d_n6, assign20880_e26390_d_n8, assign20880_e26390_d_n10, assign20880_e26390_d_n11, assign20880_e26390_d_n12,) = {
    if ((var_guard327 != 0.0) && (var_flg_overgiven != 0.0)) {
        let assign20880_e26384: f64 = (var_modenml * p.p174);
        let assign20880_e26387: f64 = (var_modervs * p.p173);
        let assign20880_e26388: f64 = (assign20880_e26384 + assign20880_e26387);
        (assign20880_e26388, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn4, var_cgsoe_dn5, var_cgsoe_dn6, var_cgsoe_dn8, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12,)
    }
};
        var_cgsoe = assign20880_e26390;
        var_cgsoe_dn0 = assign20880_e26390_d_n0;
        var_cgsoe_dn2 = assign20880_e26390_d_n2;
        var_cgsoe_dn4 = assign20880_e26390_d_n4;
        var_cgsoe_dn5 = assign20880_e26390_d_n5;
        var_cgsoe_dn6 = assign20880_e26390_d_n6;
        var_cgsoe_dn8 = assign20880_e26390_d_n8;
        var_cgsoe_dn10 = assign20880_e26390_d_n10;
        var_cgsoe_dn11 = assign20880_e26390_d_n11;
        var_cgsoe_dn12 = assign20880_e26390_d_n12;

        let (assign20890_e26399, assign20890_e26399_d_n0, assign20890_e26399_d_n2, assign20890_e26399_d_n4, assign20890_e26399_d_n5, assign20890_e26399_d_n6, assign20890_e26399_d_n8, assign20890_e26399_d_n10, assign20890_e26399_d_n11, assign20890_e26399_d_n12,) = {
    if ((var_guard327 != 0.0) && (var_flg_overgiven != 0.0)) {
        let assign20890_e26396: f64 = (-var_weffcv_nf);
        let assign20890_e26397: f64 = (var_cgsoe * assign20890_e26396);
        (assign20890_e26397, ((var_cgsoe_dn0 * assign20890_e26396) + (var_cgsoe * (-var_weffcv_nf_dn0))), ((var_cgsoe_dn2 * assign20890_e26396) + (var_cgsoe * (-var_weffcv_nf_dn2))), ((var_cgsoe_dn4 * assign20890_e26396) + (var_cgsoe * (-var_weffcv_nf_dn4))), ((var_cgsoe_dn5 * assign20890_e26396) + (var_cgsoe * (-var_weffcv_nf_dn5))), ((var_cgsoe_dn6 * assign20890_e26396) + (var_cgsoe * (-var_weffcv_nf_dn6))), ((var_cgsoe_dn8 * assign20890_e26396) + (var_cgsoe * (-var_weffcv_nf_dn8))), ((var_cgsoe_dn10 * assign20890_e26396) + (var_cgsoe * (-var_weffcv_nf_dn10))), ((var_cgsoe_dn11 * assign20890_e26396) + (var_cgsoe * (-var_weffcv_nf_dn11))), ((var_cgsoe_dn12 * assign20890_e26396) + (var_cgsoe * (-var_weffcv_nf_dn12))),)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn4, var_cgsoe_dn5, var_cgsoe_dn6, var_cgsoe_dn8, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12,)
    }
};
        var_cgsoe = assign20890_e26399;
        var_cgsoe_dn0 = assign20890_e26399_d_n0;
        var_cgsoe_dn2 = assign20890_e26399_d_n2;
        var_cgsoe_dn4 = assign20890_e26399_d_n4;
        var_cgsoe_dn5 = assign20890_e26399_d_n5;
        var_cgsoe_dn6 = assign20890_e26399_d_n6;
        var_cgsoe_dn8 = assign20890_e26399_d_n8;
        var_cgsoe_dn10 = assign20890_e26399_d_n10;
        var_cgsoe_dn11 = assign20890_e26399_d_n11;
        var_cgsoe_dn12 = assign20890_e26399_d_n12;

        let (assign20900_e26410, assign20900_e26410_d_n0, assign20900_e26410_d_n2, assign20900_e26410_d_n4, assign20900_e26410_d_n5, assign20900_e26410_d_n6, assign20900_e26410_d_n8, assign20900_e26410_d_n10, assign20900_e26410_d_n11, assign20900_e26410_d_n12,) = {
    if ((var_guard327 != 0.0) && (var_flg_overgiven != 0.0)) {
        let assign20900_e26405: f64 = (-var_cgsoe);
        let assign20900_e26407: f64 = (assign20900_e26405 * var_vgs);
        let assign20900_e26408: f64 = (var_qgos + assign20900_e26407);
        (assign20900_e26408, (var_qgos_dn0 + ((-var_cgsoe_dn0) * var_vgs)), (var_qgos_dn2 + ((-var_cgsoe_dn2) * var_vgs)), (var_qgos_dn4 + ((-var_cgsoe_dn4) * var_vgs)), (var_qgos_dn5 + (((-var_cgsoe_dn5) * var_vgs) + (assign20900_e26405 * var_vgs_dn5))), (var_qgos_dn6 + ((-var_cgsoe_dn6) * var_vgs)), (var_qgos_dn8 + ((-var_cgsoe_dn8) * var_vgs)), (var_qgos_dn10 + ((-var_cgsoe_dn10) * var_vgs)), (var_qgos_dn11 + (((-var_cgsoe_dn11) * var_vgs) + (assign20900_e26405 * var_vgs_dn11))), (var_qgos_dn12 + (((-var_cgsoe_dn12) * var_vgs) + (assign20900_e26405 * var_vgs_dn12))),)
    } else {
        (var_qgos, var_qgos_dn0, var_qgos_dn2, var_qgos_dn4, var_qgos_dn5, var_qgos_dn6, var_qgos_dn8, var_qgos_dn10, var_qgos_dn11, var_qgos_dn12,)
    }
};
        var_qgos = assign20900_e26410;
        var_qgos_dn0 = assign20900_e26410_d_n0;
        var_qgos_dn2 = assign20900_e26410_d_n2;
        var_qgos_dn4 = assign20900_e26410_d_n4;
        var_qgos_dn5 = assign20900_e26410_d_n5;
        var_qgos_dn6 = assign20900_e26410_d_n6;
        var_qgos_dn8 = assign20900_e26410_d_n8;
        var_qgos_dn10 = assign20900_e26410_d_n10;
        var_qgos_dn11 = assign20900_e26410_d_n11;
        var_qgos_dn12 = assign20900_e26410_d_n12;

        let assign20910_e26423: f64 = if (((var_mode == 1.0) && (var_cgdo_given == 0.0)) || ((var_mode != 1.0) && (var_cgso_given == 0.0))) { 1.0 } else { 0.0 };
        var_guard362 = assign20910_e26423;

        let assign20920_e26426: f64 = if p.p175 > 0.0 { 1.0 } else { 0.0 };
        var_guard363 = assign20920_e26426;

        let (assign20930_e26440, assign20930_e26440_d_n0, assign20930_e26440_d_n2, assign20930_e26440_d_n4, assign20930_e26440_d_n5, assign20930_e26440_d_n6, assign20930_e26440_d_n8, assign20930_e26440_d_n10, assign20930_e26440_d_n11, assign20930_e26440_d_n12,) = {
    if (((var_guard327 == 0.0) && (var_guard362 != 0.0)) && (var_guard363 != 0.0)) {
        let assign20930_e26434: f64 = (-var_cox0);
        let assign20930_e26436: f64 = (assign20930_e26434 * p.p175);
        let assign20930_e26438: f64 = (assign20930_e26436 * var_weffcv_nf);
        (assign20930_e26438, (assign20930_e26436 * var_weffcv_nf_dn0), (assign20930_e26436 * var_weffcv_nf_dn2), (assign20930_e26436 * var_weffcv_nf_dn4), (assign20930_e26436 * var_weffcv_nf_dn5), (assign20930_e26436 * var_weffcv_nf_dn6), (assign20930_e26436 * var_weffcv_nf_dn8), (assign20930_e26436 * var_weffcv_nf_dn10), (assign20930_e26436 * var_weffcv_nf_dn11), (assign20930_e26436 * var_weffcv_nf_dn12),)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn4, var_cgdoe_dn5, var_cgdoe_dn6, var_cgdoe_dn8, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12,)
    }
};
        var_cgdoe = assign20930_e26440;
        var_cgdoe_dn0 = assign20930_e26440_d_n0;
        var_cgdoe_dn2 = assign20930_e26440_d_n2;
        var_cgdoe_dn4 = assign20930_e26440_d_n4;
        var_cgdoe_dn5 = assign20930_e26440_d_n5;
        var_cgdoe_dn6 = assign20930_e26440_d_n6;
        var_cgdoe_dn8 = assign20930_e26440_d_n8;
        var_cgdoe_dn10 = assign20930_e26440_d_n10;
        var_cgdoe_dn11 = assign20930_e26440_d_n11;
        var_cgdoe_dn12 = assign20930_e26440_d_n12;

        let (assign20940_e26450, assign20940_e26450_d_n0, assign20940_e26450_d_n2, assign20940_e26450_d_n4, assign20940_e26450_d_n5, assign20940_e26450_d_n6, assign20940_e26450_d_n8, assign20940_e26450_d_n10, assign20940_e26450_d_n11, assign20940_e26450_d_n12,) = {
    if (((var_guard327 == 0.0) && (var_guard362 != 0.0)) && (var_guard363 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn4, var_cgdoe_dn5, var_cgdoe_dn6, var_cgdoe_dn8, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12,)
    }
};
        var_cgdoe = assign20940_e26450;
        var_cgdoe_dn0 = assign20940_e26450_d_n0;
        var_cgdoe_dn2 = assign20940_e26450_d_n2;
        var_cgdoe_dn4 = assign20940_e26450_d_n4;
        var_cgdoe_dn5 = assign20940_e26450_d_n5;
        var_cgdoe_dn6 = assign20940_e26450_d_n6;
        var_cgdoe_dn8 = assign20940_e26450_d_n8;
        var_cgdoe_dn10 = assign20940_e26450_d_n10;
        var_cgdoe_dn11 = assign20940_e26450_d_n11;
        var_cgdoe_dn12 = assign20940_e26450_d_n12;

        let (assign20950_e26464, assign20950_e26464_d_n0, assign20950_e26464_d_n2, assign20950_e26464_d_n4, assign20950_e26464_d_n5, assign20950_e26464_d_n6, assign20950_e26464_d_n8, assign20950_e26464_d_n10, assign20950_e26464_d_n11, assign20950_e26464_d_n12,) = {
    if ((var_guard327 == 0.0) && (var_guard362 == 0.0)) {
        let assign20950_e26458: f64 = (var_modervs * p.p174);
        let assign20950_e26461: f64 = (var_modenml * p.p173);
        let assign20950_e26462: f64 = (assign20950_e26458 + assign20950_e26461);
        (assign20950_e26462, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn4, var_cgdoe_dn5, var_cgdoe_dn6, var_cgdoe_dn8, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12,)
    }
};
        var_cgdoe = assign20950_e26464;
        var_cgdoe_dn0 = assign20950_e26464_d_n0;
        var_cgdoe_dn2 = assign20950_e26464_d_n2;
        var_cgdoe_dn4 = assign20950_e26464_d_n4;
        var_cgdoe_dn5 = assign20950_e26464_d_n5;
        var_cgdoe_dn6 = assign20950_e26464_d_n6;
        var_cgdoe_dn8 = assign20950_e26464_d_n8;
        var_cgdoe_dn10 = assign20950_e26464_d_n10;
        var_cgdoe_dn11 = assign20950_e26464_d_n11;
        var_cgdoe_dn12 = assign20950_e26464_d_n12;

        let (assign20960_e26475, assign20960_e26475_d_n0, assign20960_e26475_d_n2, assign20960_e26475_d_n4, assign20960_e26475_d_n5, assign20960_e26475_d_n6, assign20960_e26475_d_n8, assign20960_e26475_d_n10, assign20960_e26475_d_n11, assign20960_e26475_d_n12,) = {
    if ((var_guard327 == 0.0) && (var_guard362 == 0.0)) {
        let assign20960_e26472: f64 = (-var_weffcv_nf);
        let assign20960_e26473: f64 = (var_cgdoe * assign20960_e26472);
        (assign20960_e26473, ((var_cgdoe_dn0 * assign20960_e26472) + (var_cgdoe * (-var_weffcv_nf_dn0))), ((var_cgdoe_dn2 * assign20960_e26472) + (var_cgdoe * (-var_weffcv_nf_dn2))), ((var_cgdoe_dn4 * assign20960_e26472) + (var_cgdoe * (-var_weffcv_nf_dn4))), ((var_cgdoe_dn5 * assign20960_e26472) + (var_cgdoe * (-var_weffcv_nf_dn5))), ((var_cgdoe_dn6 * assign20960_e26472) + (var_cgdoe * (-var_weffcv_nf_dn6))), ((var_cgdoe_dn8 * assign20960_e26472) + (var_cgdoe * (-var_weffcv_nf_dn8))), ((var_cgdoe_dn10 * assign20960_e26472) + (var_cgdoe * (-var_weffcv_nf_dn10))), ((var_cgdoe_dn11 * assign20960_e26472) + (var_cgdoe * (-var_weffcv_nf_dn11))), ((var_cgdoe_dn12 * assign20960_e26472) + (var_cgdoe * (-var_weffcv_nf_dn12))),)
    } else {
        (var_cgdoe, var_cgdoe_dn0, var_cgdoe_dn2, var_cgdoe_dn4, var_cgdoe_dn5, var_cgdoe_dn6, var_cgdoe_dn8, var_cgdoe_dn10, var_cgdoe_dn11, var_cgdoe_dn12,)
    }
};
        var_cgdoe = assign20960_e26475;
        var_cgdoe_dn0 = assign20960_e26475_d_n0;
        var_cgdoe_dn2 = assign20960_e26475_d_n2;
        var_cgdoe_dn4 = assign20960_e26475_d_n4;
        var_cgdoe_dn5 = assign20960_e26475_d_n5;
        var_cgdoe_dn6 = assign20960_e26475_d_n6;
        var_cgdoe_dn8 = assign20960_e26475_d_n8;
        var_cgdoe_dn10 = assign20960_e26475_d_n10;
        var_cgdoe_dn11 = assign20960_e26475_d_n11;
        var_cgdoe_dn12 = assign20960_e26475_d_n12;

        *var_cgdoe_slot = var_cgdoe;
        *var_cgdoe_dn0_slot = var_cgdoe_dn0;
        *var_cgdoe_dn10_slot = var_cgdoe_dn10;
        *var_cgdoe_dn11_slot = var_cgdoe_dn11;
        *var_cgdoe_dn12_slot = var_cgdoe_dn12;
        *var_cgdoe_dn2_slot = var_cgdoe_dn2;
        *var_cgdoe_dn4_slot = var_cgdoe_dn4;
        *var_cgdoe_dn5_slot = var_cgdoe_dn5;
        *var_cgdoe_dn6_slot = var_cgdoe_dn6;
        *var_cgdoe_dn8_slot = var_cgdoe_dn8;
        *var_cgsoe_slot = var_cgsoe;
        *var_cgsoe_dn0_slot = var_cgsoe_dn0;
        *var_cgsoe_dn10_slot = var_cgsoe_dn10;
        *var_cgsoe_dn11_slot = var_cgsoe_dn11;
        *var_cgsoe_dn12_slot = var_cgsoe_dn12;
        *var_cgsoe_dn2_slot = var_cgsoe_dn2;
        *var_cgsoe_dn4_slot = var_cgsoe_dn4;
        *var_cgsoe_dn5_slot = var_cgsoe_dn5;
        *var_cgsoe_dn6_slot = var_cgsoe_dn6;
        *var_cgsoe_dn8_slot = var_cgsoe_dn8;
        *var_flg_overgiven_slot = var_flg_overgiven;
        *var_guard360_slot = var_guard360;
        *var_guard362_slot = var_guard362;
        *var_guard363_slot = var_guard363;
        *var_qbdld_slot = var_qbdld;
        *var_qbdld_dn0_slot = var_qbdld_dn0;
        *var_qbdld_dn10_slot = var_qbdld_dn10;
        *var_qbdld_dn11_slot = var_qbdld_dn11;
        *var_qbdld_dn12_slot = var_qbdld_dn12;
        *var_qbdld_dn2_slot = var_qbdld_dn2;
        *var_qbdld_dn4_slot = var_qbdld_dn4;
        *var_qbdld_dn5_slot = var_qbdld_dn5;
        *var_qbdld_dn6_slot = var_qbdld_dn6;
        *var_qbdld_dn8_slot = var_qbdld_dn8;
        *var_qbsld_slot = var_qbsld;
        *var_qbsld_dn0_slot = var_qbsld_dn0;
        *var_qbsld_dn10_slot = var_qbsld_dn10;
        *var_qbsld_dn11_slot = var_qbsld_dn11;
        *var_qbsld_dn12_slot = var_qbsld_dn12;
        *var_qbsld_dn2_slot = var_qbsld_dn2;
        *var_qbsld_dn4_slot = var_qbsld_dn4;
        *var_qbsld_dn5_slot = var_qbsld_dn5;
        *var_qbsld_dn6_slot = var_qbsld_dn6;
        *var_qbsld_dn8_slot = var_qbsld_dn8;
        *var_qbuld_slot = var_qbuld;
        *var_qbuld_dn0_slot = var_qbuld_dn0;
        *var_qbuld_dn10_slot = var_qbuld_dn10;
        *var_qbuld_dn11_slot = var_qbuld_dn11;
        *var_qbuld_dn12_slot = var_qbuld_dn12;
        *var_qbuld_dn2_slot = var_qbuld_dn2;
        *var_qbuld_dn4_slot = var_qbuld_dn4;
        *var_qbuld_dn5_slot = var_qbuld_dn5;
        *var_qbuld_dn6_slot = var_qbuld_dn6;
        *var_qbuld_dn8_slot = var_qbuld_dn8;
        *var_qgod_slot = var_qgod;
        *var_qgod_dn0_slot = var_qgod_dn0;
        *var_qgod_dn10_slot = var_qgod_dn10;
        *var_qgod_dn11_slot = var_qgod_dn11;
        *var_qgod_dn12_slot = var_qgod_dn12;
        *var_qgod_dn2_slot = var_qgod_dn2;
        *var_qgod_dn4_slot = var_qgod_dn4;
        *var_qgod_dn5_slot = var_qgod_dn5;
        *var_qgod_dn6_slot = var_qgod_dn6;
        *var_qgod_dn8_slot = var_qgod_dn8;
        *var_qgos_slot = var_qgos;
        *var_qgos_dn0_slot = var_qgos_dn0;
        *var_qgos_dn10_slot = var_qgos_dn10;
        *var_qgos_dn11_slot = var_qgos_dn11;
        *var_qgos_dn12_slot = var_qgos_dn12;
        *var_qgos_dn2_slot = var_qgos_dn2;
        *var_qgos_dn4_slot = var_qgos_dn4;
        *var_qgos_dn5_slot = var_qgos_dn5;
        *var_qgos_dn6_slot = var_qgos_dn6;
        *var_qgos_dn8_slot = var_qgos_dn8;
        *var_qiuld_slot = var_qiuld;
        *var_qiuld_dn0_slot = var_qiuld_dn0;
        *var_qiuld_dn10_slot = var_qiuld_dn10;
        *var_qiuld_dn11_slot = var_qiuld_dn11;
        *var_qiuld_dn12_slot = var_qiuld_dn12;
        *var_qiuld_dn2_slot = var_qiuld_dn2;
        *var_qiuld_dn4_slot = var_qiuld_dn4;
        *var_qiuld_dn5_slot = var_qiuld_dn5;
        *var_qiuld_dn6_slot = var_qiuld_dn6;
        *var_qiuld_dn8_slot = var_qiuld_dn8;
        *var_qovd_slot = var_qovd;
        *var_qovd_dn0_slot = var_qovd_dn0;
        *var_qovd_dn10_slot = var_qovd_dn10;
        *var_qovd_dn11_slot = var_qovd_dn11;
        *var_qovd_dn12_slot = var_qovd_dn12;
        *var_qovd_dn2_slot = var_qovd_dn2;
        *var_qovd_dn4_slot = var_qovd_dn4;
        *var_qovd_dn5_slot = var_qovd_dn5;
        *var_qovd_dn6_slot = var_qovd_dn6;
        *var_qovd_dn8_slot = var_qovd_dn8;
        *var_qovs_slot = var_qovs;
        *var_qovs_dn0_slot = var_qovs_dn0;
        *var_qovs_dn10_slot = var_qovs_dn10;
        *var_qovs_dn11_slot = var_qovs_dn11;
        *var_qovs_dn12_slot = var_qovs_dn12;
        *var_qovs_dn2_slot = var_qovs_dn2;
        *var_qovs_dn4_slot = var_qovs_dn4;
        *var_qovs_dn5_slot = var_qovs_dn5;
        *var_qovs_dn6_slot = var_qovs_dn6;
        *var_qovs_dn8_slot = var_qovs_dn8;
        *var_qsuld_slot = var_qsuld;
        *var_qsuld_dn0_slot = var_qsuld_dn0;
        *var_qsuld_dn10_slot = var_qsuld_dn10;
        *var_qsuld_dn11_slot = var_qsuld_dn11;
        *var_qsuld_dn12_slot = var_qsuld_dn12;
        *var_qsuld_dn2_slot = var_qsuld_dn2;
        *var_qsuld_dn4_slot = var_qsuld_dn4;
        *var_qsuld_dn5_slot = var_qsuld_dn5;
        *var_qsuld_dn6_slot = var_qsuld_dn6;
        *var_qsuld_dn8_slot = var_qsuld_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_xi0_slot = var_xi0;
        *var_xi0_dn0_slot = var_xi0_dn0;
        *var_xi0_dn10_slot = var_xi0_dn10;
        *var_xi0_dn11_slot = var_xi0_dn11;
        *var_xi0_dn12_slot = var_xi0_dn12;
        *var_xi0_dn2_slot = var_xi0_dn2;
        *var_xi0_dn4_slot = var_xi0_dn4;
        *var_xi0_dn5_slot = var_xi0_dn5;
        *var_xi0_dn6_slot = var_xi0_dn6;
        *var_xi0_dn8_slot = var_xi0_dn8;
        *var_xi0p12_slot = var_xi0p12;
        *var_xi0p12_dn0_slot = var_xi0p12_dn0;
        *var_xi0p12_dn10_slot = var_xi0p12_dn10;
        *var_xi0p12_dn11_slot = var_xi0p12_dn11;
        *var_xi0p12_dn12_slot = var_xi0p12_dn12;
        *var_xi0p12_dn2_slot = var_xi0p12_dn2;
        *var_xi0p12_dn4_slot = var_xi0p12_dn4;
        *var_xi0p12_dn5_slot = var_xi0p12_dn5;
        *var_xi0p12_dn6_slot = var_xi0p12_dn6;
        *var_xi0p12_dn8_slot = var_xi0p12_dn8;
    }

    pub(super) fn stamp_transient_block_81(
        p: &Parameters,
        var_beta_inv: f64,
        var_beta_inv_dn4: f64,
        var_c_fox: f64,
        var_c_fox_dn0: f64,
        var_c_fox_dn10: f64,
        var_c_fox_dn11: f64,
        var_c_fox_dn12: f64,
        var_c_fox_dn2: f64,
        var_c_fox_dn4: f64,
        var_c_fox_dn5: f64,
        var_c_fox_dn6: f64,
        var_c_fox_dn8: f64,
        var_cgdo_given: f64,
        var_cgdoe: f64,
        var_cgdoe_dn0: f64,
        var_cgdoe_dn10: f64,
        var_cgdoe_dn11: f64,
        var_cgdoe_dn12: f64,
        var_cgdoe_dn2: f64,
        var_cgdoe_dn4: f64,
        var_cgdoe_dn5: f64,
        var_cgdoe_dn6: f64,
        var_cgdoe_dn8: f64,
        var_cgso_given: f64,
        var_cox0: f64,
        var_ey: f64,
        var_ey_dn0: f64,
        var_ey_dn10: f64,
        var_ey_dn11: f64,
        var_ey_dn12: f64,
        var_ey_dn2: f64,
        var_ey_dn4: f64,
        var_ey_dn5: f64,
        var_ey_dn6: f64,
        var_ey_dn8: f64,
        var_flg_noqi: f64,
        var_flg_nqs: f64,
        var_guard327: f64,
        var_lch: f64,
        var_lch_dn0: f64,
        var_lch_dn10: f64,
        var_lch_dn11: f64,
        var_lch_dn12: f64,
        var_lch_dn2: f64,
        var_lch_dn4: f64,
        var_lch_dn5: f64,
        var_lch_dn6: f64,
        var_lch_dn8: f64,
        var_mks_cit: f64,
        var_mks_nfalp: f64,
        var_mode: f64,
        var_modenml: f64,
        var_modervs: f64,
        var_mu: f64,
        var_mu_dn0: f64,
        var_mu_dn10: f64,
        var_mu_dn11: f64,
        var_mu_dn12: f64,
        var_mu_dn2: f64,
        var_mu_dn4: f64,
        var_mu_dn5: f64,
        var_mu_dn6: f64,
        var_mu_dn8: f64,
        var_muun: f64,
        var_muun_dn0: f64,
        var_muun_dn10: f64,
        var_muun_dn11: f64,
        var_muun_dn12: f64,
        var_muun_dn2: f64,
        var_muun_dn4: f64,
        var_muun_dn5: f64,
        var_muun_dn6: f64,
        var_muun_dn8: f64,
        var_ps0: f64,
        var_ps0_dn0: f64,
        var_ps0_dn10: f64,
        var_ps0_dn11: f64,
        var_ps0_dn12: f64,
        var_ps0_dn2: f64,
        var_ps0_dn4: f64,
        var_ps0_dn5: f64,
        var_ps0_dn6: f64,
        var_ps0_dn8: f64,
        var_psdl: f64,
        var_psdl_dn0: f64,
        var_psdl_dn10: f64,
        var_psdl_dn11: f64,
        var_psdl_dn12: f64,
        var_psdl_dn2: f64,
        var_psdl_dn4: f64,
        var_psdl_dn5: f64,
        var_psdl_dn6: f64,
        var_psdl_dn8: f64,
        var_qi: f64,
        var_qi_dn0: f64,
        var_qi_dn10: f64,
        var_qi_dn11: f64,
        var_qi_dn12: f64,
        var_qi_dn2: f64,
        var_qi_dn4: f64,
        var_qi_dn5: f64,
        var_qi_dn6: f64,
        var_qi_dn8: f64,
        var_qn0: f64,
        var_qn0_dn0: f64,
        var_qn0_dn10: f64,
        var_qn0_dn11: f64,
        var_qn0_dn12: f64,
        var_qn0_dn2: f64,
        var_qn0_dn4: f64,
        var_qn0_dn5: f64,
        var_qn0_dn6: f64,
        var_qn0_dn8: f64,
        var_vbs: f64,
        var_vbs_dn0: f64,
        var_vbs_dn10: f64,
        var_vbs_dn11: f64,
        var_vbs_dn12: f64,
        var_vbs_dn2: f64,
        var_vbs_dn4: f64,
        var_vbs_dn5: f64,
        var_vbs_dn6: f64,
        var_vbs_dn8: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn8: f64,
        var_vgs: f64,
        var_vgs_dn11: f64,
        var_vgs_dn12: f64,
        var_vgs_dn5: f64,
        var_vgvt: f64,
        var_vgvt_dn0: f64,
        var_vgvt_dn10: f64,
        var_vgvt_dn11: f64,
        var_vgvt_dn12: f64,
        var_vgvt_dn2: f64,
        var_vgvt_dn4: f64,
        var_vgvt_dn5: f64,
        var_vgvt_dn6: f64,
        var_vgvt_dn8: f64,
        var_weffcv_nf: f64,
        var_weffcv_nf_dn0: f64,
        var_weffcv_nf_dn10: f64,
        var_weffcv_nf_dn11: f64,
        var_weffcv_nf_dn12: f64,
        var_weffcv_nf_dn2: f64,
        var_weffcv_nf_dn4: f64,
        var_weffcv_nf_dn5: f64,
        var_weffcv_nf_dn6: f64,
        var_weffcv_nf_dn8: f64,
        var_cgsoe_slot: &mut f64,
        var_cgsoe_dn0_slot: &mut f64,
        var_cgsoe_dn10_slot: &mut f64,
        var_cgsoe_dn11_slot: &mut f64,
        var_cgsoe_dn12_slot: &mut f64,
        var_cgsoe_dn2_slot: &mut f64,
        var_cgsoe_dn4_slot: &mut f64,
        var_cgsoe_dn5_slot: &mut f64,
        var_cgsoe_dn6_slot: &mut f64,
        var_cgsoe_dn8_slot: &mut f64,
        var_cite_slot: &mut f64,
        var_eyd_slot: &mut f64,
        var_eyd_dn0_slot: &mut f64,
        var_eyd_dn10_slot: &mut f64,
        var_eyd_dn11_slot: &mut f64,
        var_eyd_dn12_slot: &mut f64,
        var_eyd_dn2_slot: &mut f64,
        var_eyd_dn4_slot: &mut f64,
        var_eyd_dn5_slot: &mut f64,
        var_eyd_dn6_slot: &mut f64,
        var_eyd_dn8_slot: &mut f64,
        var_guard364_slot: &mut f64,
        var_guard365_slot: &mut f64,
        var_guard366_slot: &mut f64,
        var_guard367_slot: &mut f64,
        var_guard368_slot: &mut f64,
        var_guard369_slot: &mut f64,
        var_guard370_slot: &mut f64,
        var_nfalpe_slot: &mut f64,
        var_qgod_slot: &mut f64,
        var_qgod_dn0_slot: &mut f64,
        var_qgod_dn10_slot: &mut f64,
        var_qgod_dn11_slot: &mut f64,
        var_qgod_dn12_slot: &mut f64,
        var_qgod_dn2_slot: &mut f64,
        var_qgod_dn4_slot: &mut f64,
        var_qgod_dn5_slot: &mut f64,
        var_qgod_dn6_slot: &mut f64,
        var_qgod_dn8_slot: &mut f64,
        var_qgos_slot: &mut f64,
        var_qgos_dn0_slot: &mut f64,
        var_qgos_dn10_slot: &mut f64,
        var_qgos_dn11_slot: &mut f64,
        var_qgos_dn12_slot: &mut f64,
        var_qgos_dn2_slot: &mut f64,
        var_qgos_dn4_slot: &mut f64,
        var_qgos_dn5_slot: &mut f64,
        var_qgos_dn6_slot: &mut f64,
        var_qgos_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t12_slot: &mut f64,
        var_t12_dn0_slot: &mut f64,
        var_t12_dn10_slot: &mut f64,
        var_t12_dn11_slot: &mut f64,
        var_t12_dn12_slot: &mut f64,
        var_t12_dn2_slot: &mut f64,
        var_t12_dn4_slot: &mut f64,
        var_t12_dn5_slot: &mut f64,
        var_t12_dn6_slot: &mut f64,
        var_t12_dn8_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn12_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_tau_slot: &mut f64,
        var_tau_dn0_slot: &mut f64,
        var_tau_dn10_slot: &mut f64,
        var_tau_dn11_slot: &mut f64,
        var_tau_dn12_slot: &mut f64,
        var_tau_dn2_slot: &mut f64,
        var_tau_dn4_slot: &mut f64,
        var_tau_dn5_slot: &mut f64,
        var_tau_dn6_slot: &mut f64,
        var_tau_dn8_slot: &mut f64,
        var_taub_slot: &mut f64,
        var_taub_dn0_slot: &mut f64,
        var_taub_dn10_slot: &mut f64,
        var_taub_dn11_slot: &mut f64,
        var_taub_dn12_slot: &mut f64,
        var_taub_dn2_slot: &mut f64,
        var_taub_dn4_slot: &mut f64,
        var_taub_dn5_slot: &mut f64,
        var_taub_dn6_slot: &mut f64,
        var_taub_dn8_slot: &mut f64,
    ) {
        let mut var_cgsoe: f64 = *var_cgsoe_slot;
        let mut var_cgsoe_dn0: f64 = *var_cgsoe_dn0_slot;
        let mut var_cgsoe_dn10: f64 = *var_cgsoe_dn10_slot;
        let mut var_cgsoe_dn11: f64 = *var_cgsoe_dn11_slot;
        let mut var_cgsoe_dn12: f64 = *var_cgsoe_dn12_slot;
        let mut var_cgsoe_dn2: f64 = *var_cgsoe_dn2_slot;
        let mut var_cgsoe_dn4: f64 = *var_cgsoe_dn4_slot;
        let mut var_cgsoe_dn5: f64 = *var_cgsoe_dn5_slot;
        let mut var_cgsoe_dn6: f64 = *var_cgsoe_dn6_slot;
        let mut var_cgsoe_dn8: f64 = *var_cgsoe_dn8_slot;
        let mut var_cite: f64 = *var_cite_slot;
        let mut var_eyd: f64 = *var_eyd_slot;
        let mut var_eyd_dn0: f64 = *var_eyd_dn0_slot;
        let mut var_eyd_dn10: f64 = *var_eyd_dn10_slot;
        let mut var_eyd_dn11: f64 = *var_eyd_dn11_slot;
        let mut var_eyd_dn12: f64 = *var_eyd_dn12_slot;
        let mut var_eyd_dn2: f64 = *var_eyd_dn2_slot;
        let mut var_eyd_dn4: f64 = *var_eyd_dn4_slot;
        let mut var_eyd_dn5: f64 = *var_eyd_dn5_slot;
        let mut var_eyd_dn6: f64 = *var_eyd_dn6_slot;
        let mut var_eyd_dn8: f64 = *var_eyd_dn8_slot;
        let mut var_guard364: f64 = *var_guard364_slot;
        let mut var_guard365: f64 = *var_guard365_slot;
        let mut var_guard366: f64 = *var_guard366_slot;
        let mut var_guard367: f64 = *var_guard367_slot;
        let mut var_guard368: f64 = *var_guard368_slot;
        let mut var_guard369: f64 = *var_guard369_slot;
        let mut var_guard370: f64 = *var_guard370_slot;
        let mut var_nfalpe: f64 = *var_nfalpe_slot;
        let mut var_qgod: f64 = *var_qgod_slot;
        let mut var_qgod_dn0: f64 = *var_qgod_dn0_slot;
        let mut var_qgod_dn10: f64 = *var_qgod_dn10_slot;
        let mut var_qgod_dn11: f64 = *var_qgod_dn11_slot;
        let mut var_qgod_dn12: f64 = *var_qgod_dn12_slot;
        let mut var_qgod_dn2: f64 = *var_qgod_dn2_slot;
        let mut var_qgod_dn4: f64 = *var_qgod_dn4_slot;
        let mut var_qgod_dn5: f64 = *var_qgod_dn5_slot;
        let mut var_qgod_dn6: f64 = *var_qgod_dn6_slot;
        let mut var_qgod_dn8: f64 = *var_qgod_dn8_slot;
        let mut var_qgos: f64 = *var_qgos_slot;
        let mut var_qgos_dn0: f64 = *var_qgos_dn0_slot;
        let mut var_qgos_dn10: f64 = *var_qgos_dn10_slot;
        let mut var_qgos_dn11: f64 = *var_qgos_dn11_slot;
        let mut var_qgos_dn12: f64 = *var_qgos_dn12_slot;
        let mut var_qgos_dn2: f64 = *var_qgos_dn2_slot;
        let mut var_qgos_dn4: f64 = *var_qgos_dn4_slot;
        let mut var_qgos_dn5: f64 = *var_qgos_dn5_slot;
        let mut var_qgos_dn6: f64 = *var_qgos_dn6_slot;
        let mut var_qgos_dn8: f64 = *var_qgos_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t12: f64 = *var_t12_slot;
        let mut var_t12_dn0: f64 = *var_t12_dn0_slot;
        let mut var_t12_dn10: f64 = *var_t12_dn10_slot;
        let mut var_t12_dn11: f64 = *var_t12_dn11_slot;
        let mut var_t12_dn12: f64 = *var_t12_dn12_slot;
        let mut var_t12_dn2: f64 = *var_t12_dn2_slot;
        let mut var_t12_dn4: f64 = *var_t12_dn4_slot;
        let mut var_t12_dn5: f64 = *var_t12_dn5_slot;
        let mut var_t12_dn6: f64 = *var_t12_dn6_slot;
        let mut var_t12_dn8: f64 = *var_t12_dn8_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn12: f64 = *var_t7_dn12_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_tau: f64 = *var_tau_slot;
        let mut var_tau_dn0: f64 = *var_tau_dn0_slot;
        let mut var_tau_dn10: f64 = *var_tau_dn10_slot;
        let mut var_tau_dn11: f64 = *var_tau_dn11_slot;
        let mut var_tau_dn12: f64 = *var_tau_dn12_slot;
        let mut var_tau_dn2: f64 = *var_tau_dn2_slot;
        let mut var_tau_dn4: f64 = *var_tau_dn4_slot;
        let mut var_tau_dn5: f64 = *var_tau_dn5_slot;
        let mut var_tau_dn6: f64 = *var_tau_dn6_slot;
        let mut var_tau_dn8: f64 = *var_tau_dn8_slot;
        let mut var_taub: f64 = *var_taub_slot;
        let mut var_taub_dn0: f64 = *var_taub_dn0_slot;
        let mut var_taub_dn10: f64 = *var_taub_dn10_slot;
        let mut var_taub_dn11: f64 = *var_taub_dn11_slot;
        let mut var_taub_dn12: f64 = *var_taub_dn12_slot;
        let mut var_taub_dn2: f64 = *var_taub_dn2_slot;
        let mut var_taub_dn4: f64 = *var_taub_dn4_slot;
        let mut var_taub_dn5: f64 = *var_taub_dn5_slot;
        let mut var_taub_dn6: f64 = *var_taub_dn6_slot;
        let mut var_taub_dn8: f64 = *var_taub_dn8_slot;

        let (assign20970_e26485, assign20970_e26485_d_n0, assign20970_e26485_d_n2, assign20970_e26485_d_n4, assign20970_e26485_d_n5, assign20970_e26485_d_n6, assign20970_e26485_d_n8, assign20970_e26485_d_n10, assign20970_e26485_d_n11, assign20970_e26485_d_n12,) = {
    if (var_guard327 == 0.0) {
        let assign20970_e26479: f64 = (-var_cgdoe);
        let assign20970_e26482: f64 = (var_vgs - var_vds);
        let assign20970_e26483: f64 = (assign20970_e26479 * assign20970_e26482);
        (assign20970_e26483, (((-var_cgdoe_dn0) * assign20970_e26482) + (assign20970_e26479 * (-var_vds_dn0))), (((-var_cgdoe_dn2) * assign20970_e26482) + (assign20970_e26479 * (-var_vds_dn2))), (((-var_cgdoe_dn4) * assign20970_e26482) + (assign20970_e26479 * (-var_vds_dn4))), (((-var_cgdoe_dn5) * assign20970_e26482) + (assign20970_e26479 * (var_vgs_dn5 - var_vds_dn5))), (((-var_cgdoe_dn6) * assign20970_e26482) + (assign20970_e26479 * (-var_vds_dn6))), (((-var_cgdoe_dn8) * assign20970_e26482) + (assign20970_e26479 * (-var_vds_dn8))), (((-var_cgdoe_dn10) * assign20970_e26482) + (assign20970_e26479 * (-var_vds_dn10))), (((-var_cgdoe_dn11) * assign20970_e26482) + (assign20970_e26479 * (var_vgs_dn11 - var_vds_dn11))), (((-var_cgdoe_dn12) * assign20970_e26482) + (assign20970_e26479 * (var_vgs_dn12 - var_vds_dn12))),)
    } else {
        (var_qgod, var_qgod_dn0, var_qgod_dn2, var_qgod_dn4, var_qgod_dn5, var_qgod_dn6, var_qgod_dn8, var_qgod_dn10, var_qgod_dn11, var_qgod_dn12,)
    }
};
        var_qgod = assign20970_e26485;
        var_qgod_dn0 = assign20970_e26485_d_n0;
        var_qgod_dn2 = assign20970_e26485_d_n2;
        var_qgod_dn4 = assign20970_e26485_d_n4;
        var_qgod_dn5 = assign20970_e26485_d_n5;
        var_qgod_dn6 = assign20970_e26485_d_n6;
        var_qgod_dn8 = assign20970_e26485_d_n8;
        var_qgod_dn10 = assign20970_e26485_d_n10;
        var_qgod_dn11 = assign20970_e26485_d_n11;
        var_qgod_dn12 = assign20970_e26485_d_n12;

        let assign20980_e26498: f64 = if (((var_mode == 1.0) && (var_cgso_given == 0.0)) || ((var_mode != 1.0) && (var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        var_guard364 = assign20980_e26498;

        let (assign20990_e26510, assign20990_e26510_d_n0, assign20990_e26510_d_n2, assign20990_e26510_d_n4, assign20990_e26510_d_n5, assign20990_e26510_d_n6, assign20990_e26510_d_n8, assign20990_e26510_d_n10, assign20990_e26510_d_n11, assign20990_e26510_d_n12,) = {
    if ((var_guard327 == 0.0) && (var_guard364 != 0.0)) {
        let assign20990_e26504: f64 = (-var_cox0);
        let assign20990_e26506: f64 = (assign20990_e26504 * p.p175);
        let assign20990_e26508: f64 = (assign20990_e26506 * var_weffcv_nf);
        (assign20990_e26508, (assign20990_e26506 * var_weffcv_nf_dn0), (assign20990_e26506 * var_weffcv_nf_dn2), (assign20990_e26506 * var_weffcv_nf_dn4), (assign20990_e26506 * var_weffcv_nf_dn5), (assign20990_e26506 * var_weffcv_nf_dn6), (assign20990_e26506 * var_weffcv_nf_dn8), (assign20990_e26506 * var_weffcv_nf_dn10), (assign20990_e26506 * var_weffcv_nf_dn11), (assign20990_e26506 * var_weffcv_nf_dn12),)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn4, var_cgsoe_dn5, var_cgsoe_dn6, var_cgsoe_dn8, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12,)
    }
};
        var_cgsoe = assign20990_e26510;
        var_cgsoe_dn0 = assign20990_e26510_d_n0;
        var_cgsoe_dn2 = assign20990_e26510_d_n2;
        var_cgsoe_dn4 = assign20990_e26510_d_n4;
        var_cgsoe_dn5 = assign20990_e26510_d_n5;
        var_cgsoe_dn6 = assign20990_e26510_d_n6;
        var_cgsoe_dn8 = assign20990_e26510_d_n8;
        var_cgsoe_dn10 = assign20990_e26510_d_n10;
        var_cgsoe_dn11 = assign20990_e26510_d_n11;
        var_cgsoe_dn12 = assign20990_e26510_d_n12;

        let (assign21000_e26524, assign21000_e26524_d_n0, assign21000_e26524_d_n2, assign21000_e26524_d_n4, assign21000_e26524_d_n5, assign21000_e26524_d_n6, assign21000_e26524_d_n8, assign21000_e26524_d_n10, assign21000_e26524_d_n11, assign21000_e26524_d_n12,) = {
    if ((var_guard327 == 0.0) && (var_guard364 == 0.0)) {
        let assign21000_e26518: f64 = (var_modenml * p.p174);
        let assign21000_e26521: f64 = (var_modervs * p.p173);
        let assign21000_e26522: f64 = (assign21000_e26518 + assign21000_e26521);
        (assign21000_e26522, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn4, var_cgsoe_dn5, var_cgsoe_dn6, var_cgsoe_dn8, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12,)
    }
};
        var_cgsoe = assign21000_e26524;
        var_cgsoe_dn0 = assign21000_e26524_d_n0;
        var_cgsoe_dn2 = assign21000_e26524_d_n2;
        var_cgsoe_dn4 = assign21000_e26524_d_n4;
        var_cgsoe_dn5 = assign21000_e26524_d_n5;
        var_cgsoe_dn6 = assign21000_e26524_d_n6;
        var_cgsoe_dn8 = assign21000_e26524_d_n8;
        var_cgsoe_dn10 = assign21000_e26524_d_n10;
        var_cgsoe_dn11 = assign21000_e26524_d_n11;
        var_cgsoe_dn12 = assign21000_e26524_d_n12;

        let (assign21010_e26535, assign21010_e26535_d_n0, assign21010_e26535_d_n2, assign21010_e26535_d_n4, assign21010_e26535_d_n5, assign21010_e26535_d_n6, assign21010_e26535_d_n8, assign21010_e26535_d_n10, assign21010_e26535_d_n11, assign21010_e26535_d_n12,) = {
    if ((var_guard327 == 0.0) && (var_guard364 == 0.0)) {
        let assign21010_e26532: f64 = (-var_weffcv_nf);
        let assign21010_e26533: f64 = (var_cgsoe * assign21010_e26532);
        (assign21010_e26533, ((var_cgsoe_dn0 * assign21010_e26532) + (var_cgsoe * (-var_weffcv_nf_dn0))), ((var_cgsoe_dn2 * assign21010_e26532) + (var_cgsoe * (-var_weffcv_nf_dn2))), ((var_cgsoe_dn4 * assign21010_e26532) + (var_cgsoe * (-var_weffcv_nf_dn4))), ((var_cgsoe_dn5 * assign21010_e26532) + (var_cgsoe * (-var_weffcv_nf_dn5))), ((var_cgsoe_dn6 * assign21010_e26532) + (var_cgsoe * (-var_weffcv_nf_dn6))), ((var_cgsoe_dn8 * assign21010_e26532) + (var_cgsoe * (-var_weffcv_nf_dn8))), ((var_cgsoe_dn10 * assign21010_e26532) + (var_cgsoe * (-var_weffcv_nf_dn10))), ((var_cgsoe_dn11 * assign21010_e26532) + (var_cgsoe * (-var_weffcv_nf_dn11))), ((var_cgsoe_dn12 * assign21010_e26532) + (var_cgsoe * (-var_weffcv_nf_dn12))),)
    } else {
        (var_cgsoe, var_cgsoe_dn0, var_cgsoe_dn2, var_cgsoe_dn4, var_cgsoe_dn5, var_cgsoe_dn6, var_cgsoe_dn8, var_cgsoe_dn10, var_cgsoe_dn11, var_cgsoe_dn12,)
    }
};
        var_cgsoe = assign21010_e26535;
        var_cgsoe_dn0 = assign21010_e26535_d_n0;
        var_cgsoe_dn2 = assign21010_e26535_d_n2;
        var_cgsoe_dn4 = assign21010_e26535_d_n4;
        var_cgsoe_dn5 = assign21010_e26535_d_n5;
        var_cgsoe_dn6 = assign21010_e26535_d_n6;
        var_cgsoe_dn8 = assign21010_e26535_d_n8;
        var_cgsoe_dn10 = assign21010_e26535_d_n10;
        var_cgsoe_dn11 = assign21010_e26535_d_n11;
        var_cgsoe_dn12 = assign21010_e26535_d_n12;

        let (assign21020_e26543, assign21020_e26543_d_n0, assign21020_e26543_d_n2, assign21020_e26543_d_n4, assign21020_e26543_d_n5, assign21020_e26543_d_n6, assign21020_e26543_d_n8, assign21020_e26543_d_n10, assign21020_e26543_d_n11, assign21020_e26543_d_n12,) = {
    if (var_guard327 == 0.0) {
        let assign21020_e26539: f64 = (-var_cgsoe);
        let assign21020_e26541: f64 = (assign21020_e26539 * var_vgs);
        (assign21020_e26541, ((-var_cgsoe_dn0) * var_vgs), ((-var_cgsoe_dn2) * var_vgs), ((-var_cgsoe_dn4) * var_vgs), (((-var_cgsoe_dn5) * var_vgs) + (assign21020_e26539 * var_vgs_dn5)), ((-var_cgsoe_dn6) * var_vgs), ((-var_cgsoe_dn8) * var_vgs), ((-var_cgsoe_dn10) * var_vgs), (((-var_cgsoe_dn11) * var_vgs) + (assign21020_e26539 * var_vgs_dn11)), (((-var_cgsoe_dn12) * var_vgs) + (assign21020_e26539 * var_vgs_dn12)),)
    } else {
        (var_qgos, var_qgos_dn0, var_qgos_dn2, var_qgos_dn4, var_qgos_dn5, var_qgos_dn6, var_qgos_dn8, var_qgos_dn10, var_qgos_dn11, var_qgos_dn12,)
    }
};
        var_qgos = assign21020_e26543;
        var_qgos_dn0 = assign21020_e26543_d_n0;
        var_qgos_dn2 = assign21020_e26543_d_n2;
        var_qgos_dn4 = assign21020_e26543_d_n4;
        var_qgos_dn5 = assign21020_e26543_d_n5;
        var_qgos_dn6 = assign21020_e26543_d_n6;
        var_qgos_dn8 = assign21020_e26543_d_n8;
        var_qgos_dn10 = assign21020_e26543_d_n10;
        var_qgos_dn11 = assign21020_e26543_d_n11;
        var_qgos_dn12 = assign21020_e26543_d_n12;

        let assign21030_e26546: f64 = if var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        var_guard365 = assign21030_e26546;

        let (assign21040_e26558, assign21040_e26558_d_n0, assign21040_e26558_d_n2, assign21040_e26558_d_n4, assign21040_e26558_d_n5, assign21040_e26558_d_n6, assign21040_e26558_d_n8, assign21040_e26558_d_n10, assign21040_e26558_d_n11, assign21040_e26558_d_n12,) = {
    if ((var_flg_nqs != 0.0) && (var_guard365 != 0.0)) {
        let assign21040_e26552: f64 = (p.p223 * p.p224);
        let assign21040_e26554: f64 = (assign21040_e26552 * var_lch);
        let assign21040_e26556: f64 = (assign21040_e26554 * var_lch);
        (assign21040_e26556, (((assign21040_e26552 * var_lch_dn0) * var_lch) + (assign21040_e26554 * var_lch_dn0)), (((assign21040_e26552 * var_lch_dn2) * var_lch) + (assign21040_e26554 * var_lch_dn2)), (((assign21040_e26552 * var_lch_dn4) * var_lch) + (assign21040_e26554 * var_lch_dn4)), (((assign21040_e26552 * var_lch_dn5) * var_lch) + (assign21040_e26554 * var_lch_dn5)), (((assign21040_e26552 * var_lch_dn6) * var_lch) + (assign21040_e26554 * var_lch_dn6)), (((assign21040_e26552 * var_lch_dn8) * var_lch) + (assign21040_e26554 * var_lch_dn8)), (((assign21040_e26552 * var_lch_dn10) * var_lch) + (assign21040_e26554 * var_lch_dn10)), (((assign21040_e26552 * var_lch_dn11) * var_lch) + (assign21040_e26554 * var_lch_dn11)), (((assign21040_e26552 * var_lch_dn12) * var_lch) + (assign21040_e26554 * var_lch_dn12)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign21040_e26558;
        var_t1_dn0 = assign21040_e26558_d_n0;
        var_t1_dn2 = assign21040_e26558_d_n2;
        var_t1_dn4 = assign21040_e26558_d_n4;
        var_t1_dn5 = assign21040_e26558_d_n5;
        var_t1_dn6 = assign21040_e26558_d_n6;
        var_t1_dn8 = assign21040_e26558_d_n8;
        var_t1_dn10 = assign21040_e26558_d_n10;
        var_t1_dn11 = assign21040_e26558_d_n11;
        var_t1_dn12 = assign21040_e26558_d_n12;

        let (assign21050_e26576, assign21050_e26576_d_n0, assign21050_e26576_d_n2, assign21050_e26576_d_n4, assign21050_e26576_d_n5, assign21050_e26576_d_n6, assign21050_e26576_d_n8, assign21050_e26576_d_n10, assign21050_e26576_d_n11, assign21050_e26576_d_n12,) = {
    if ((var_flg_nqs != 0.0) && (var_guard365 != 0.0)) {
        let assign21050_e26564: f64 = (var_mu * var_vgvt);
        let assign21050_e26566: f64 = (assign21050_e26564 * p.p223);
        let assign21050_e26569: f64 = (p.p224 * var_lch);
        let assign21050_e26571: f64 = (assign21050_e26569 * var_lch);
        let assign21050_e26572: f64 = (assign21050_e26566 + assign21050_e26571);
        let assign21050_e26574: f64 = (assign21050_e26572 + 1e-50);
        (assign21050_e26574, ((((var_mu_dn0 * var_vgvt) + (var_mu * var_vgvt_dn0)) * p.p223) + (((p.p224 * var_lch_dn0) * var_lch) + (assign21050_e26569 * var_lch_dn0))), ((((var_mu_dn2 * var_vgvt) + (var_mu * var_vgvt_dn2)) * p.p223) + (((p.p224 * var_lch_dn2) * var_lch) + (assign21050_e26569 * var_lch_dn2))), ((((var_mu_dn4 * var_vgvt) + (var_mu * var_vgvt_dn4)) * p.p223) + (((p.p224 * var_lch_dn4) * var_lch) + (assign21050_e26569 * var_lch_dn4))), ((((var_mu_dn5 * var_vgvt) + (var_mu * var_vgvt_dn5)) * p.p223) + (((p.p224 * var_lch_dn5) * var_lch) + (assign21050_e26569 * var_lch_dn5))), ((((var_mu_dn6 * var_vgvt) + (var_mu * var_vgvt_dn6)) * p.p223) + (((p.p224 * var_lch_dn6) * var_lch) + (assign21050_e26569 * var_lch_dn6))), ((((var_mu_dn8 * var_vgvt) + (var_mu * var_vgvt_dn8)) * p.p223) + (((p.p224 * var_lch_dn8) * var_lch) + (assign21050_e26569 * var_lch_dn8))), ((((var_mu_dn10 * var_vgvt) + (var_mu * var_vgvt_dn10)) * p.p223) + (((p.p224 * var_lch_dn10) * var_lch) + (assign21050_e26569 * var_lch_dn10))), ((((var_mu_dn11 * var_vgvt) + (var_mu * var_vgvt_dn11)) * p.p223) + (((p.p224 * var_lch_dn11) * var_lch) + (assign21050_e26569 * var_lch_dn11))), ((((var_mu_dn12 * var_vgvt) + (var_mu * var_vgvt_dn12)) * p.p223) + (((p.p224 * var_lch_dn12) * var_lch) + (assign21050_e26569 * var_lch_dn12))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign21050_e26576;
        var_t2_dn0 = assign21050_e26576_d_n0;
        var_t2_dn2 = assign21050_e26576_d_n2;
        var_t2_dn4 = assign21050_e26576_d_n4;
        var_t2_dn5 = assign21050_e26576_d_n5;
        var_t2_dn6 = assign21050_e26576_d_n6;
        var_t2_dn8 = assign21050_e26576_d_n8;
        var_t2_dn10 = assign21050_e26576_d_n10;
        var_t2_dn11 = assign21050_e26576_d_n11;
        var_t2_dn12 = assign21050_e26576_d_n12;

        let (assign21060_e26584, assign21060_e26584_d_n0, assign21060_e26584_d_n2, assign21060_e26584_d_n4, assign21060_e26584_d_n5, assign21060_e26584_d_n6, assign21060_e26584_d_n8, assign21060_e26584_d_n10, assign21060_e26584_d_n11, assign21060_e26584_d_n12,) = {
    if ((var_flg_nqs != 0.0) && (var_guard365 != 0.0)) {
        let assign21060_e26582: f64 = (var_t1 / var_t2);
        (assign21060_e26582, (((var_t1_dn0 * var_t2) - (var_t1 * var_t2_dn0)) / (var_t2 * var_t2)), (((var_t1_dn2 * var_t2) - (var_t1 * var_t2_dn2)) / (var_t2 * var_t2)), (((var_t1_dn4 * var_t2) - (var_t1 * var_t2_dn4)) / (var_t2 * var_t2)), (((var_t1_dn5 * var_t2) - (var_t1 * var_t2_dn5)) / (var_t2 * var_t2)), (((var_t1_dn6 * var_t2) - (var_t1 * var_t2_dn6)) / (var_t2 * var_t2)), (((var_t1_dn8 * var_t2) - (var_t1 * var_t2_dn8)) / (var_t2 * var_t2)), (((var_t1_dn10 * var_t2) - (var_t1 * var_t2_dn10)) / (var_t2 * var_t2)), (((var_t1_dn11 * var_t2) - (var_t1 * var_t2_dn11)) / (var_t2 * var_t2)), (((var_t1_dn12 * var_t2) - (var_t1 * var_t2_dn12)) / (var_t2 * var_t2)),)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn4, var_tau_dn5, var_tau_dn6, var_tau_dn8, var_tau_dn10, var_tau_dn11, var_tau_dn12,)
    }
};
        var_tau = assign21060_e26584;
        var_tau_dn0 = assign21060_e26584_d_n0;
        var_tau_dn2 = assign21060_e26584_d_n2;
        var_tau_dn4 = assign21060_e26584_d_n4;
        var_tau_dn5 = assign21060_e26584_d_n5;
        var_tau_dn6 = assign21060_e26584_d_n6;
        var_tau_dn8 = assign21060_e26584_d_n8;
        var_tau_dn10 = assign21060_e26584_d_n10;
        var_tau_dn11 = assign21060_e26584_d_n11;
        var_tau_dn12 = assign21060_e26584_d_n12;

        let (assign21070_e26593, assign21070_e26593_d_n0, assign21070_e26593_d_n2, assign21070_e26593_d_n4, assign21070_e26593_d_n5, assign21070_e26593_d_n6, assign21070_e26593_d_n8, assign21070_e26593_d_n10, assign21070_e26593_d_n11, assign21070_e26593_d_n12,) = {
    if ((var_flg_nqs != 0.0) && (var_guard365 == 0.0)) {
        let assign21070_e26591: f64 = (p.p223 + 1e-50);
        (assign21070_e26591, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn4, var_tau_dn5, var_tau_dn6, var_tau_dn8, var_tau_dn10, var_tau_dn11, var_tau_dn12,)
    }
};
        var_tau = assign21070_e26593;
        var_tau_dn0 = assign21070_e26593_d_n0;
        var_tau_dn2 = assign21070_e26593_d_n2;
        var_tau_dn4 = assign21070_e26593_d_n4;
        var_tau_dn5 = assign21070_e26593_d_n5;
        var_tau_dn6 = assign21070_e26593_d_n6;
        var_tau_dn8 = assign21070_e26593_d_n8;
        var_tau_dn10 = assign21070_e26593_d_n10;
        var_tau_dn11 = assign21070_e26593_d_n11;
        var_tau_dn12 = assign21070_e26593_d_n12;

        let (assign21080_e26601, assign21080_e26601_d_n0, assign21080_e26601_d_n2, assign21080_e26601_d_n4, assign21080_e26601_d_n5, assign21080_e26601_d_n6, assign21080_e26601_d_n8, assign21080_e26601_d_n10, assign21080_e26601_d_n11, assign21080_e26601_d_n12,) = {
    if (var_flg_nqs != 0.0) {
        let assign21080_e26597: f64 = (p.p225 * var_c_fox);
        let assign21080_e26599: f64 = (assign21080_e26597 / 10000.0);
        (assign21080_e26599, ((p.p225 * var_c_fox_dn0) / 10000.0), ((p.p225 * var_c_fox_dn2) / 10000.0), ((p.p225 * var_c_fox_dn4) / 10000.0), ((p.p225 * var_c_fox_dn5) / 10000.0), ((p.p225 * var_c_fox_dn6) / 10000.0), ((p.p225 * var_c_fox_dn8) / 10000.0), ((p.p225 * var_c_fox_dn10) / 10000.0), ((p.p225 * var_c_fox_dn11) / 10000.0), ((p.p225 * var_c_fox_dn12) / 10000.0),)
    } else {
        (var_taub, var_taub_dn0, var_taub_dn2, var_taub_dn4, var_taub_dn5, var_taub_dn6, var_taub_dn8, var_taub_dn10, var_taub_dn11, var_taub_dn12,)
    }
};
        var_taub = assign21080_e26601;
        var_taub_dn0 = assign21080_e26601_d_n0;
        var_taub_dn2 = assign21080_e26601_d_n2;
        var_taub_dn4 = assign21080_e26601_d_n4;
        var_taub_dn5 = assign21080_e26601_d_n5;
        var_taub_dn6 = assign21080_e26601_d_n6;
        var_taub_dn8 = assign21080_e26601_d_n8;
        var_taub_dn10 = assign21080_e26601_d_n10;
        var_taub_dn11 = assign21080_e26601_d_n11;
        var_taub_dn12 = assign21080_e26601_d_n12;

        let assign21090_e26607: f64 = if ((p.p21 != 0.0) && (var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        var_guard366 = assign21090_e26607;

        let (assign21100_e26611,) = {
    if (var_guard366 != 0.0) {
        (var_mks_nfalp,)
    } else {
        (var_nfalpe,)
    }
};
        var_nfalpe = assign21100_e26611;

        let (assign21120_e26619,) = {
    if (var_guard366 != 0.0) {
        (var_mks_cit,)
    } else {
        (var_cite,)
    }
};
        var_cite = assign21120_e26619;

        let (assign21130_e26625, assign21130_e26625_d_n0, assign21130_e26625_d_n2, assign21130_e26625_d_n4, assign21130_e26625_d_n5, assign21130_e26625_d_n6, assign21130_e26625_d_n8, assign21130_e26625_d_n10, assign21130_e26625_d_n11, assign21130_e26625_d_n12,) = {
    if (var_guard366 != 0.0) {
        let assign21130_e26623: f64 = (var_qn0 / 1.6021918e-19);
        (assign21130_e26623, (var_qn0_dn0 / 1.6021918e-19), (var_qn0_dn2 / 1.6021918e-19), (var_qn0_dn4 / 1.6021918e-19), (var_qn0_dn5 / 1.6021918e-19), (var_qn0_dn6 / 1.6021918e-19), (var_qn0_dn8 / 1.6021918e-19), (var_qn0_dn10 / 1.6021918e-19), (var_qn0_dn11 / 1.6021918e-19), (var_qn0_dn12 / 1.6021918e-19),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign21130_e26625;
        var_t1_dn0 = assign21130_e26625_d_n0;
        var_t1_dn2 = assign21130_e26625_d_n2;
        var_t1_dn4 = assign21130_e26625_d_n4;
        var_t1_dn5 = assign21130_e26625_d_n5;
        var_t1_dn6 = assign21130_e26625_d_n6;
        var_t1_dn8 = assign21130_e26625_d_n8;
        var_t1_dn10 = assign21130_e26625_d_n10;
        var_t1_dn11 = assign21130_e26625_d_n11;
        var_t1_dn12 = assign21130_e26625_d_n12;

        let (assign21140_e26641, assign21140_e26641_d_n0, assign21140_e26641_d_n2, assign21140_e26641_d_n4, assign21140_e26641_d_n5, assign21140_e26641_d_n6, assign21140_e26641_d_n8, assign21140_e26641_d_n10, assign21140_e26641_d_n11, assign21140_e26641_d_n12,) = {
    if (var_guard366 != 0.0) {
        let assign21140_e26631: f64 = (var_ps0 - var_vbs);
        let assign21140_e26632: f64 = (var_qn0 / assign21140_e26631);
        let assign21140_e26633: f64 = (var_c_fox + assign21140_e26632);
        let assign21140_e26635: f64 = (assign21140_e26633 + var_cite);
        let assign21140_e26637: f64 = (assign21140_e26635 * var_beta_inv);
        let assign21140_e26639: f64 = (assign21140_e26637 / 1.6021918e-19);
        (assign21140_e26639, (((var_c_fox_dn0 + (((var_qn0_dn0 * assign21140_e26631) - (var_qn0 * (var_ps0_dn0 - var_vbs_dn0))) / (assign21140_e26631 * assign21140_e26631))) * var_beta_inv) / 1.6021918e-19), (((var_c_fox_dn2 + (((var_qn0_dn2 * assign21140_e26631) - (var_qn0 * (var_ps0_dn2 - var_vbs_dn2))) / (assign21140_e26631 * assign21140_e26631))) * var_beta_inv) / 1.6021918e-19), ((((var_c_fox_dn4 + (((var_qn0_dn4 * assign21140_e26631) - (var_qn0 * (var_ps0_dn4 - var_vbs_dn4))) / (assign21140_e26631 * assign21140_e26631))) * var_beta_inv) + (assign21140_e26635 * var_beta_inv_dn4)) / 1.6021918e-19), (((var_c_fox_dn5 + (((var_qn0_dn5 * assign21140_e26631) - (var_qn0 * (var_ps0_dn5 - var_vbs_dn5))) / (assign21140_e26631 * assign21140_e26631))) * var_beta_inv) / 1.6021918e-19), (((var_c_fox_dn6 + (((var_qn0_dn6 * assign21140_e26631) - (var_qn0 * (var_ps0_dn6 - var_vbs_dn6))) / (assign21140_e26631 * assign21140_e26631))) * var_beta_inv) / 1.6021918e-19), (((var_c_fox_dn8 + (((var_qn0_dn8 * assign21140_e26631) - (var_qn0 * (var_ps0_dn8 - var_vbs_dn8))) / (assign21140_e26631 * assign21140_e26631))) * var_beta_inv) / 1.6021918e-19), (((var_c_fox_dn10 + (((var_qn0_dn10 * assign21140_e26631) - (var_qn0 * (var_ps0_dn10 - var_vbs_dn10))) / (assign21140_e26631 * assign21140_e26631))) * var_beta_inv) / 1.6021918e-19), (((var_c_fox_dn11 + (((var_qn0_dn11 * assign21140_e26631) - (var_qn0 * (var_ps0_dn11 - var_vbs_dn11))) / (assign21140_e26631 * assign21140_e26631))) * var_beta_inv) / 1.6021918e-19), (((var_c_fox_dn12 + (((var_qn0_dn12 * assign21140_e26631) - (var_qn0 * (var_ps0_dn12 - var_vbs_dn12))) / (assign21140_e26631 * assign21140_e26631))) * var_beta_inv) / 1.6021918e-19),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign21140_e26641;
        var_t2_dn0 = assign21140_e26641_d_n0;
        var_t2_dn2 = assign21140_e26641_d_n2;
        var_t2_dn4 = assign21140_e26641_d_n4;
        var_t2_dn5 = assign21140_e26641_d_n5;
        var_t2_dn6 = assign21140_e26641_d_n6;
        var_t2_dn8 = assign21140_e26641_d_n8;
        var_t2_dn10 = assign21140_e26641_d_n10;
        var_t2_dn11 = assign21140_e26641_d_n11;
        var_t2_dn12 = assign21140_e26641_d_n12;

        let (assign21150_e26656, assign21150_e26656_d_n0, assign21150_e26656_d_n2, assign21150_e26656_d_n4, assign21150_e26656_d_n5, assign21150_e26656_d_n6, assign21150_e26656_d_n8, assign21150_e26656_d_n10, assign21150_e26656_d_n11, assign21150_e26656_d_n12,) = {
    if (var_guard366 != 0.0) {
        let assign21150_e26644: f64 = (-2.0);
        let assign21150_e26646: f64 = (assign21150_e26644 * var_qi);
        let assign21150_e26648: f64 = (assign21150_e26646 / 1.6021918e-19);
        let assign21150_e26650: f64 = (assign21150_e26648 / var_lch);
        let assign21150_e26652: f64 = (assign21150_e26650 / var_weffcv_nf);
        let assign21150_e26654: f64 = (assign21150_e26652 - var_t1);
        (assign21150_e26654, (((((((((assign21150_e26644 * var_qi_dn0) / 1.6021918e-19) * var_lch) - (assign21150_e26648 * var_lch_dn0)) / (var_lch * var_lch)) * var_weffcv_nf) - (assign21150_e26650 * var_weffcv_nf_dn0)) / (var_weffcv_nf * var_weffcv_nf)) - var_t1_dn0), (((((((((assign21150_e26644 * var_qi_dn2) / 1.6021918e-19) * var_lch) - (assign21150_e26648 * var_lch_dn2)) / (var_lch * var_lch)) * var_weffcv_nf) - (assign21150_e26650 * var_weffcv_nf_dn2)) / (var_weffcv_nf * var_weffcv_nf)) - var_t1_dn2), (((((((((assign21150_e26644 * var_qi_dn4) / 1.6021918e-19) * var_lch) - (assign21150_e26648 * var_lch_dn4)) / (var_lch * var_lch)) * var_weffcv_nf) - (assign21150_e26650 * var_weffcv_nf_dn4)) / (var_weffcv_nf * var_weffcv_nf)) - var_t1_dn4), (((((((((assign21150_e26644 * var_qi_dn5) / 1.6021918e-19) * var_lch) - (assign21150_e26648 * var_lch_dn5)) / (var_lch * var_lch)) * var_weffcv_nf) - (assign21150_e26650 * var_weffcv_nf_dn5)) / (var_weffcv_nf * var_weffcv_nf)) - var_t1_dn5), (((((((((assign21150_e26644 * var_qi_dn6) / 1.6021918e-19) * var_lch) - (assign21150_e26648 * var_lch_dn6)) / (var_lch * var_lch)) * var_weffcv_nf) - (assign21150_e26650 * var_weffcv_nf_dn6)) / (var_weffcv_nf * var_weffcv_nf)) - var_t1_dn6), (((((((((assign21150_e26644 * var_qi_dn8) / 1.6021918e-19) * var_lch) - (assign21150_e26648 * var_lch_dn8)) / (var_lch * var_lch)) * var_weffcv_nf) - (assign21150_e26650 * var_weffcv_nf_dn8)) / (var_weffcv_nf * var_weffcv_nf)) - var_t1_dn8), (((((((((assign21150_e26644 * var_qi_dn10) / 1.6021918e-19) * var_lch) - (assign21150_e26648 * var_lch_dn10)) / (var_lch * var_lch)) * var_weffcv_nf) - (assign21150_e26650 * var_weffcv_nf_dn10)) / (var_weffcv_nf * var_weffcv_nf)) - var_t1_dn10), (((((((((assign21150_e26644 * var_qi_dn11) / 1.6021918e-19) * var_lch) - (assign21150_e26648 * var_lch_dn11)) / (var_lch * var_lch)) * var_weffcv_nf) - (assign21150_e26650 * var_weffcv_nf_dn11)) / (var_weffcv_nf * var_weffcv_nf)) - var_t1_dn11), (((((((((assign21150_e26644 * var_qi_dn12) / 1.6021918e-19) * var_lch) - (assign21150_e26648 * var_lch_dn12)) / (var_lch * var_lch)) * var_weffcv_nf) - (assign21150_e26650 * var_weffcv_nf_dn12)) / (var_weffcv_nf * var_weffcv_nf)) - var_t1_dn12),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign21150_e26656;
        var_t3_dn0 = assign21150_e26656_d_n0;
        var_t3_dn2 = assign21150_e26656_d_n2;
        var_t3_dn4 = assign21150_e26656_d_n4;
        var_t3_dn5 = assign21150_e26656_d_n5;
        var_t3_dn6 = assign21150_e26656_d_n6;
        var_t3_dn8 = assign21150_e26656_d_n8;
        var_t3_dn10 = assign21150_e26656_d_n10;
        var_t3_dn11 = assign21150_e26656_d_n11;
        var_t3_dn12 = assign21150_e26656_d_n12;

        let assign21160_e26659: f64 = (var_t3 - var_t1);
        let assign21160_e26660: f64 = (assign21160_e26659).abs();
        let assign21160_e26663: f64 = (10.0 * 2.220446049250313e-16);
        let assign21160_e26664: f64 = if assign21160_e26660 > assign21160_e26663 { 1.0 } else { 0.0 };
        var_guard367 = assign21160_e26664;

        let (assign21170_e26711, assign21170_e26711_d_n0, assign21170_e26711_d_n2, assign21170_e26711_d_n4, assign21170_e26711_d_n5, assign21170_e26711_d_n6, assign21170_e26711_d_n8, assign21170_e26711_d_n10, assign21170_e26711_d_n11, assign21170_e26711_d_n12,) = {
    if ((var_guard366 != 0.0) && (var_guard367 != 0.0)) {
        let assign21170_e26671: f64 = (var_t1 + var_t2);
        let assign21170_e26672: f64 = (1.0 / assign21170_e26671);
        let assign21170_e26675: f64 = (var_t3 + var_t2);
        let assign21170_e26676: f64 = (assign21170_e26672 / assign21170_e26675);
        let assign21170_e26679: f64 = (2.0 * var_nfalpe);
        let assign21170_e26681: f64 = (assign21170_e26679 * var_ey);
        let assign21170_e26683: f64 = (assign21170_e26681 * var_mu);
        let assign21170_e26686: f64 = (var_t3 - var_t1);
        let assign21170_e26687: f64 = (assign21170_e26683 / assign21170_e26686);
        let assign21170_e26690: f64 = (var_t3 + var_t2);
        let assign21170_e26693: f64 = (var_t1 + var_t2);
        let assign21170_e26694: f64 = (assign21170_e26690 / assign21170_e26693);
        let assign21170_e26695: f64 = (assign21170_e26694).ln();
        let assign21170_e26696: f64 = (assign21170_e26687 * assign21170_e26695);
        let assign21170_e26697: f64 = (assign21170_e26676 + assign21170_e26696);
        let assign21170_e26700: f64 = (var_nfalpe * var_ey);
        let assign21170_e26702: f64 = (assign21170_e26700 * var_mu);
        let assign21170_e26704: f64 = (assign21170_e26702 * var_nfalpe);
        let assign21170_e26706: f64 = (assign21170_e26704 * var_ey);
        let assign21170_e26708: f64 = (assign21170_e26706 * var_mu);
        let assign21170_e26709: f64 = (assign21170_e26697 + assign21170_e26708);
        (assign21170_e26709, ((((((-((var_t1_dn0 + var_t2_dn0) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (var_t3_dn0 + var_t2_dn0))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * var_ey_dn0) * var_mu) + (assign21170_e26681 * var_mu_dn0)) * assign21170_e26686) - (assign21170_e26683 * (var_t3_dn0 - var_t1_dn0))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((var_t3_dn0 + var_t2_dn0) * assign21170_e26693) - (assign21170_e26690 * (var_t1_dn0 + var_t2_dn0))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((var_nfalpe * var_ey_dn0) * var_mu) + (assign21170_e26700 * var_mu_dn0)) * var_nfalpe) * var_ey) + (assign21170_e26704 * var_ey_dn0)) * var_mu) + (assign21170_e26706 * var_mu_dn0))), ((((((-((var_t1_dn2 + var_t2_dn2) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (var_t3_dn2 + var_t2_dn2))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * var_ey_dn2) * var_mu) + (assign21170_e26681 * var_mu_dn2)) * assign21170_e26686) - (assign21170_e26683 * (var_t3_dn2 - var_t1_dn2))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((var_t3_dn2 + var_t2_dn2) * assign21170_e26693) - (assign21170_e26690 * (var_t1_dn2 + var_t2_dn2))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((var_nfalpe * var_ey_dn2) * var_mu) + (assign21170_e26700 * var_mu_dn2)) * var_nfalpe) * var_ey) + (assign21170_e26704 * var_ey_dn2)) * var_mu) + (assign21170_e26706 * var_mu_dn2))), ((((((-((var_t1_dn4 + var_t2_dn4) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (var_t3_dn4 + var_t2_dn4))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * var_ey_dn4) * var_mu) + (assign21170_e26681 * var_mu_dn4)) * assign21170_e26686) - (assign21170_e26683 * (var_t3_dn4 - var_t1_dn4))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((var_t3_dn4 + var_t2_dn4) * assign21170_e26693) - (assign21170_e26690 * (var_t1_dn4 + var_t2_dn4))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((var_nfalpe * var_ey_dn4) * var_mu) + (assign21170_e26700 * var_mu_dn4)) * var_nfalpe) * var_ey) + (assign21170_e26704 * var_ey_dn4)) * var_mu) + (assign21170_e26706 * var_mu_dn4))), ((((((-((var_t1_dn5 + var_t2_dn5) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (var_t3_dn5 + var_t2_dn5))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * var_ey_dn5) * var_mu) + (assign21170_e26681 * var_mu_dn5)) * assign21170_e26686) - (assign21170_e26683 * (var_t3_dn5 - var_t1_dn5))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((var_t3_dn5 + var_t2_dn5) * assign21170_e26693) - (assign21170_e26690 * (var_t1_dn5 + var_t2_dn5))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((var_nfalpe * var_ey_dn5) * var_mu) + (assign21170_e26700 * var_mu_dn5)) * var_nfalpe) * var_ey) + (assign21170_e26704 * var_ey_dn5)) * var_mu) + (assign21170_e26706 * var_mu_dn5))), ((((((-((var_t1_dn6 + var_t2_dn6) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (var_t3_dn6 + var_t2_dn6))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * var_ey_dn6) * var_mu) + (assign21170_e26681 * var_mu_dn6)) * assign21170_e26686) - (assign21170_e26683 * (var_t3_dn6 - var_t1_dn6))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((var_t3_dn6 + var_t2_dn6) * assign21170_e26693) - (assign21170_e26690 * (var_t1_dn6 + var_t2_dn6))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((var_nfalpe * var_ey_dn6) * var_mu) + (assign21170_e26700 * var_mu_dn6)) * var_nfalpe) * var_ey) + (assign21170_e26704 * var_ey_dn6)) * var_mu) + (assign21170_e26706 * var_mu_dn6))), ((((((-((var_t1_dn8 + var_t2_dn8) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (var_t3_dn8 + var_t2_dn8))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * var_ey_dn8) * var_mu) + (assign21170_e26681 * var_mu_dn8)) * assign21170_e26686) - (assign21170_e26683 * (var_t3_dn8 - var_t1_dn8))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((var_t3_dn8 + var_t2_dn8) * assign21170_e26693) - (assign21170_e26690 * (var_t1_dn8 + var_t2_dn8))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((var_nfalpe * var_ey_dn8) * var_mu) + (assign21170_e26700 * var_mu_dn8)) * var_nfalpe) * var_ey) + (assign21170_e26704 * var_ey_dn8)) * var_mu) + (assign21170_e26706 * var_mu_dn8))), ((((((-((var_t1_dn10 + var_t2_dn10) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (var_t3_dn10 + var_t2_dn10))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * var_ey_dn10) * var_mu) + (assign21170_e26681 * var_mu_dn10)) * assign21170_e26686) - (assign21170_e26683 * (var_t3_dn10 - var_t1_dn10))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((var_t3_dn10 + var_t2_dn10) * assign21170_e26693) - (assign21170_e26690 * (var_t1_dn10 + var_t2_dn10))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((var_nfalpe * var_ey_dn10) * var_mu) + (assign21170_e26700 * var_mu_dn10)) * var_nfalpe) * var_ey) + (assign21170_e26704 * var_ey_dn10)) * var_mu) + (assign21170_e26706 * var_mu_dn10))), ((((((-((var_t1_dn11 + var_t2_dn11) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (var_t3_dn11 + var_t2_dn11))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * var_ey_dn11) * var_mu) + (assign21170_e26681 * var_mu_dn11)) * assign21170_e26686) - (assign21170_e26683 * (var_t3_dn11 - var_t1_dn11))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((var_t3_dn11 + var_t2_dn11) * assign21170_e26693) - (assign21170_e26690 * (var_t1_dn11 + var_t2_dn11))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((var_nfalpe * var_ey_dn11) * var_mu) + (assign21170_e26700 * var_mu_dn11)) * var_nfalpe) * var_ey) + (assign21170_e26704 * var_ey_dn11)) * var_mu) + (assign21170_e26706 * var_mu_dn11))), ((((((-((var_t1_dn12 + var_t2_dn12) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (var_t3_dn12 + var_t2_dn12))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * var_ey_dn12) * var_mu) + (assign21170_e26681 * var_mu_dn12)) * assign21170_e26686) - (assign21170_e26683 * (var_t3_dn12 - var_t1_dn12))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((var_t3_dn12 + var_t2_dn12) * assign21170_e26693) - (assign21170_e26690 * (var_t1_dn12 + var_t2_dn12))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((var_nfalpe * var_ey_dn12) * var_mu) + (assign21170_e26700 * var_mu_dn12)) * var_nfalpe) * var_ey) + (assign21170_e26704 * var_ey_dn12)) * var_mu) + (assign21170_e26706 * var_mu_dn12))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
        var_t4 = assign21170_e26711;
        var_t4_dn0 = assign21170_e26711_d_n0;
        var_t4_dn2 = assign21170_e26711_d_n2;
        var_t4_dn4 = assign21170_e26711_d_n4;
        var_t4_dn5 = assign21170_e26711_d_n5;
        var_t4_dn6 = assign21170_e26711_d_n6;
        var_t4_dn8 = assign21170_e26711_d_n8;
        var_t4_dn10 = assign21170_e26711_d_n10;
        var_t4_dn11 = assign21170_e26711_d_n11;
        var_t4_dn12 = assign21170_e26711_d_n12;

        let (assign21180_e26750, assign21180_e26750_d_n0, assign21180_e26750_d_n2, assign21180_e26750_d_n4, assign21180_e26750_d_n5, assign21180_e26750_d_n6, assign21180_e26750_d_n8, assign21180_e26750_d_n10, assign21180_e26750_d_n11, assign21180_e26750_d_n12,) = {
    if ((var_guard366 != 0.0) && (var_guard367 == 0.0)) {
        let assign21180_e26719: f64 = (var_t1 + var_t2);
        let assign21180_e26720: f64 = (1.0 / assign21180_e26719);
        let assign21180_e26723: f64 = (var_t3 + var_t2);
        let assign21180_e26724: f64 = (assign21180_e26720 / assign21180_e26723);
        let assign21180_e26727: f64 = (2.0 * var_nfalpe);
        let assign21180_e26729: f64 = (assign21180_e26727 * var_ey);
        let assign21180_e26731: f64 = (assign21180_e26729 * var_mu);
        let assign21180_e26734: f64 = (var_t1 + var_t2);
        let assign21180_e26735: f64 = (assign21180_e26731 / assign21180_e26734);
        let assign21180_e26736: f64 = (assign21180_e26724 + assign21180_e26735);
        let assign21180_e26739: f64 = (var_nfalpe * var_ey);
        let assign21180_e26741: f64 = (assign21180_e26739 * var_mu);
        let assign21180_e26743: f64 = (assign21180_e26741 * var_nfalpe);
        let assign21180_e26745: f64 = (assign21180_e26743 * var_ey);
        let assign21180_e26747: f64 = (assign21180_e26745 * var_mu);
        let assign21180_e26748: f64 = (assign21180_e26736 + assign21180_e26747);
        (assign21180_e26748, ((((((-((var_t1_dn0 + var_t2_dn0) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (var_t3_dn0 + var_t2_dn0))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * var_ey_dn0) * var_mu) + (assign21180_e26729 * var_mu_dn0)) * assign21180_e26734) - (assign21180_e26731 * (var_t1_dn0 + var_t2_dn0))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((var_nfalpe * var_ey_dn0) * var_mu) + (assign21180_e26739 * var_mu_dn0)) * var_nfalpe) * var_ey) + (assign21180_e26743 * var_ey_dn0)) * var_mu) + (assign21180_e26745 * var_mu_dn0))), ((((((-((var_t1_dn2 + var_t2_dn2) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (var_t3_dn2 + var_t2_dn2))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * var_ey_dn2) * var_mu) + (assign21180_e26729 * var_mu_dn2)) * assign21180_e26734) - (assign21180_e26731 * (var_t1_dn2 + var_t2_dn2))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((var_nfalpe * var_ey_dn2) * var_mu) + (assign21180_e26739 * var_mu_dn2)) * var_nfalpe) * var_ey) + (assign21180_e26743 * var_ey_dn2)) * var_mu) + (assign21180_e26745 * var_mu_dn2))), ((((((-((var_t1_dn4 + var_t2_dn4) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (var_t3_dn4 + var_t2_dn4))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * var_ey_dn4) * var_mu) + (assign21180_e26729 * var_mu_dn4)) * assign21180_e26734) - (assign21180_e26731 * (var_t1_dn4 + var_t2_dn4))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((var_nfalpe * var_ey_dn4) * var_mu) + (assign21180_e26739 * var_mu_dn4)) * var_nfalpe) * var_ey) + (assign21180_e26743 * var_ey_dn4)) * var_mu) + (assign21180_e26745 * var_mu_dn4))), ((((((-((var_t1_dn5 + var_t2_dn5) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (var_t3_dn5 + var_t2_dn5))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * var_ey_dn5) * var_mu) + (assign21180_e26729 * var_mu_dn5)) * assign21180_e26734) - (assign21180_e26731 * (var_t1_dn5 + var_t2_dn5))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((var_nfalpe * var_ey_dn5) * var_mu) + (assign21180_e26739 * var_mu_dn5)) * var_nfalpe) * var_ey) + (assign21180_e26743 * var_ey_dn5)) * var_mu) + (assign21180_e26745 * var_mu_dn5))), ((((((-((var_t1_dn6 + var_t2_dn6) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (var_t3_dn6 + var_t2_dn6))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * var_ey_dn6) * var_mu) + (assign21180_e26729 * var_mu_dn6)) * assign21180_e26734) - (assign21180_e26731 * (var_t1_dn6 + var_t2_dn6))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((var_nfalpe * var_ey_dn6) * var_mu) + (assign21180_e26739 * var_mu_dn6)) * var_nfalpe) * var_ey) + (assign21180_e26743 * var_ey_dn6)) * var_mu) + (assign21180_e26745 * var_mu_dn6))), ((((((-((var_t1_dn8 + var_t2_dn8) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (var_t3_dn8 + var_t2_dn8))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * var_ey_dn8) * var_mu) + (assign21180_e26729 * var_mu_dn8)) * assign21180_e26734) - (assign21180_e26731 * (var_t1_dn8 + var_t2_dn8))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((var_nfalpe * var_ey_dn8) * var_mu) + (assign21180_e26739 * var_mu_dn8)) * var_nfalpe) * var_ey) + (assign21180_e26743 * var_ey_dn8)) * var_mu) + (assign21180_e26745 * var_mu_dn8))), ((((((-((var_t1_dn10 + var_t2_dn10) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (var_t3_dn10 + var_t2_dn10))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * var_ey_dn10) * var_mu) + (assign21180_e26729 * var_mu_dn10)) * assign21180_e26734) - (assign21180_e26731 * (var_t1_dn10 + var_t2_dn10))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((var_nfalpe * var_ey_dn10) * var_mu) + (assign21180_e26739 * var_mu_dn10)) * var_nfalpe) * var_ey) + (assign21180_e26743 * var_ey_dn10)) * var_mu) + (assign21180_e26745 * var_mu_dn10))), ((((((-((var_t1_dn11 + var_t2_dn11) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (var_t3_dn11 + var_t2_dn11))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * var_ey_dn11) * var_mu) + (assign21180_e26729 * var_mu_dn11)) * assign21180_e26734) - (assign21180_e26731 * (var_t1_dn11 + var_t2_dn11))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((var_nfalpe * var_ey_dn11) * var_mu) + (assign21180_e26739 * var_mu_dn11)) * var_nfalpe) * var_ey) + (assign21180_e26743 * var_ey_dn11)) * var_mu) + (assign21180_e26745 * var_mu_dn11))), ((((((-((var_t1_dn12 + var_t2_dn12) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (var_t3_dn12 + var_t2_dn12))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * var_ey_dn12) * var_mu) + (assign21180_e26729 * var_mu_dn12)) * assign21180_e26734) - (assign21180_e26731 * (var_t1_dn12 + var_t2_dn12))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((var_nfalpe * var_ey_dn12) * var_mu) + (assign21180_e26739 * var_mu_dn12)) * var_nfalpe) * var_ey) + (assign21180_e26743 * var_ey_dn12)) * var_mu) + (assign21180_e26745 * var_mu_dn12))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
        var_t4 = assign21180_e26750;
        var_t4_dn0 = assign21180_e26750_d_n0;
        var_t4_dn2 = assign21180_e26750_d_n2;
        var_t4_dn4 = assign21180_e26750_d_n4;
        var_t4_dn5 = assign21180_e26750_d_n5;
        var_t4_dn6 = assign21180_e26750_d_n6;
        var_t4_dn8 = assign21180_e26750_d_n8;
        var_t4_dn10 = assign21180_e26750_d_n10;
        var_t4_dn11 = assign21180_e26750_d_n11;
        var_t4_dn12 = assign21180_e26750_d_n12;

        let assign21210_e26777: f64 = if ((p.p23 != 0.0) && (var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        var_guard368 = assign21210_e26777;

        let (assign21220_e26785, assign21220_e26785_d_n0, assign21220_e26785_d_n2, assign21220_e26785_d_n4, assign21220_e26785_d_n5, assign21220_e26785_d_n6, assign21220_e26785_d_n8, assign21220_e26785_d_n10, assign21220_e26785_d_n11, assign21220_e26785_d_n12,) = {
    if (var_guard368 != 0.0) {
        let assign21220_e26781: f64 = (var_psdl - var_ps0);
        let assign21220_e26783: f64 = (assign21220_e26781 / var_lch);
        (assign21220_e26783, ((((var_psdl_dn0 - var_ps0_dn0) * var_lch) - (assign21220_e26781 * var_lch_dn0)) / (var_lch * var_lch)), ((((var_psdl_dn2 - var_ps0_dn2) * var_lch) - (assign21220_e26781 * var_lch_dn2)) / (var_lch * var_lch)), ((((var_psdl_dn4 - var_ps0_dn4) * var_lch) - (assign21220_e26781 * var_lch_dn4)) / (var_lch * var_lch)), ((((var_psdl_dn5 - var_ps0_dn5) * var_lch) - (assign21220_e26781 * var_lch_dn5)) / (var_lch * var_lch)), ((((var_psdl_dn6 - var_ps0_dn6) * var_lch) - (assign21220_e26781 * var_lch_dn6)) / (var_lch * var_lch)), ((((var_psdl_dn8 - var_ps0_dn8) * var_lch) - (assign21220_e26781 * var_lch_dn8)) / (var_lch * var_lch)), ((((var_psdl_dn10 - var_ps0_dn10) * var_lch) - (assign21220_e26781 * var_lch_dn10)) / (var_lch * var_lch)), ((((var_psdl_dn11 - var_ps0_dn11) * var_lch) - (assign21220_e26781 * var_lch_dn11)) / (var_lch * var_lch)), ((((var_psdl_dn12 - var_ps0_dn12) * var_lch) - (assign21220_e26781 * var_lch_dn12)) / (var_lch * var_lch)),)
    } else {
        (var_eyd, var_eyd_dn0, var_eyd_dn2, var_eyd_dn4, var_eyd_dn5, var_eyd_dn6, var_eyd_dn8, var_eyd_dn10, var_eyd_dn11, var_eyd_dn12,)
    }
};
        var_eyd = assign21220_e26785;
        var_eyd_dn0 = assign21220_e26785_d_n0;
        var_eyd_dn2 = assign21220_e26785_d_n2;
        var_eyd_dn4 = assign21220_e26785_d_n4;
        var_eyd_dn5 = assign21220_e26785_d_n5;
        var_eyd_dn6 = assign21220_e26785_d_n6;
        var_eyd_dn8 = assign21220_e26785_d_n8;
        var_eyd_dn10 = assign21220_e26785_d_n10;
        var_eyd_dn11 = assign21220_e26785_d_n11;
        var_eyd_dn12 = assign21220_e26785_d_n12;

        let (assign21230_e26795, assign21230_e26795_d_n0, assign21230_e26795_d_n2, assign21230_e26795_d_n4, assign21230_e26795_d_n5, assign21230_e26795_d_n6, assign21230_e26795_d_n8, assign21230_e26795_d_n10, assign21230_e26795_d_n11, assign21230_e26795_d_n12,) = {
    if (var_guard368 != 0.0) {
        let assign21230_e26789: f64 = (var_muun * var_eyd);
        let assign21230_e26792: f64 = (10000000.0 * 0.01);
        let assign21230_e26793: f64 = (assign21230_e26789 / assign21230_e26792);
        (assign21230_e26793, (((var_muun_dn0 * var_eyd) + (var_muun * var_eyd_dn0)) / assign21230_e26792), (((var_muun_dn2 * var_eyd) + (var_muun * var_eyd_dn2)) / assign21230_e26792), (((var_muun_dn4 * var_eyd) + (var_muun * var_eyd_dn4)) / assign21230_e26792), (((var_muun_dn5 * var_eyd) + (var_muun * var_eyd_dn5)) / assign21230_e26792), (((var_muun_dn6 * var_eyd) + (var_muun * var_eyd_dn6)) / assign21230_e26792), (((var_muun_dn8 * var_eyd) + (var_muun * var_eyd_dn8)) / assign21230_e26792), (((var_muun_dn10 * var_eyd) + (var_muun * var_eyd_dn10)) / assign21230_e26792), (((var_muun_dn11 * var_eyd) + (var_muun * var_eyd_dn11)) / assign21230_e26792), (((var_muun_dn12 * var_eyd) + (var_muun * var_eyd_dn12)) / assign21230_e26792),)
    } else {
        (var_t12, var_t12_dn0, var_t12_dn2, var_t12_dn4, var_t12_dn5, var_t12_dn6, var_t12_dn8, var_t12_dn10, var_t12_dn11, var_t12_dn12,)
    }
};
        var_t12 = assign21230_e26795;
        var_t12_dn0 = assign21230_e26795_d_n0;
        var_t12_dn2 = assign21230_e26795_d_n2;
        var_t12_dn4 = assign21230_e26795_d_n4;
        var_t12_dn5 = assign21230_e26795_d_n5;
        var_t12_dn6 = assign21230_e26795_d_n6;
        var_t12_dn8 = assign21230_e26795_d_n8;
        var_t12_dn10 = assign21230_e26795_d_n10;
        var_t12_dn11 = assign21230_e26795_d_n11;
        var_t12_dn12 = assign21230_e26795_d_n12;

        let assign21240_e26799: f64 = (10.0 * 2.220446049250313e-16);
        let assign21240_e26800: f64 = (1.0 - assign21240_e26799);
        let assign21240_e26807: f64 = (10.0 * 2.220446049250313e-16);
        let assign21240_e26808: f64 = (1.0 + assign21240_e26807);
        let assign21240_e26810: f64 = if ((assign21240_e26800 <= p.p114) && (p.p114 <= assign21240_e26808)) { 1.0 } else { 0.0 };
        var_guard369 = assign21240_e26810;

        let (assign21250_e26816, assign21250_e26816_d_n0, assign21250_e26816_d_n2, assign21250_e26816_d_n4, assign21250_e26816_d_n5, assign21250_e26816_d_n6, assign21250_e26816_d_n8, assign21250_e26816_d_n10, assign21250_e26816_d_n11, assign21250_e26816_d_n12,) = {
    if ((var_guard368 != 0.0) && (var_guard369 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
        var_t7 = assign21250_e26816;
        var_t7_dn0 = assign21250_e26816_d_n0;
        var_t7_dn2 = assign21250_e26816_d_n2;
        var_t7_dn4 = assign21250_e26816_d_n4;
        var_t7_dn5 = assign21250_e26816_d_n5;
        var_t7_dn6 = assign21250_e26816_d_n6;
        var_t7_dn8 = assign21250_e26816_d_n8;
        var_t7_dn10 = assign21250_e26816_d_n10;
        var_t7_dn11 = assign21250_e26816_d_n11;
        var_t7_dn12 = assign21250_e26816_d_n12;

        let assign21260_e26820: f64 = (10.0 * 2.220446049250313e-16);
        let assign21260_e26821: f64 = (2.0 - assign21260_e26820);
        let assign21260_e26828: f64 = (10.0 * 2.220446049250313e-16);
        let assign21260_e26829: f64 = (2.0 + assign21260_e26828);
        let assign21260_e26831: f64 = if ((assign21260_e26821 <= p.p114) && (p.p114 <= assign21260_e26829)) { 1.0 } else { 0.0 };
        var_guard370 = assign21260_e26831;

        let (assign21270_e26840, assign21270_e26840_d_n0, assign21270_e26840_d_n2, assign21270_e26840_d_n4, assign21270_e26840_d_n5, assign21270_e26840_d_n6, assign21270_e26840_d_n8, assign21270_e26840_d_n10, assign21270_e26840_d_n11, assign21270_e26840_d_n12,) = {
    if (((var_guard368 != 0.0) && (var_guard369 == 0.0)) && (var_guard370 != 0.0)) {
        (var_t12, var_t12_dn0, var_t12_dn2, var_t12_dn4, var_t12_dn5, var_t12_dn6, var_t12_dn8, var_t12_dn10, var_t12_dn11, var_t12_dn12,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
        var_t7 = assign21270_e26840;
        var_t7_dn0 = assign21270_e26840_d_n0;
        var_t7_dn2 = assign21270_e26840_d_n2;
        var_t7_dn4 = assign21270_e26840_d_n4;
        var_t7_dn5 = assign21270_e26840_d_n5;
        var_t7_dn6 = assign21270_e26840_d_n6;
        var_t7_dn8 = assign21270_e26840_d_n8;
        var_t7_dn10 = assign21270_e26840_d_n10;
        var_t7_dn11 = assign21270_e26840_d_n11;
        var_t7_dn12 = assign21270_e26840_d_n12;

        let (assign21280_e26854, assign21280_e26854_d_n0, assign21280_e26854_d_n2, assign21280_e26854_d_n4, assign21280_e26854_d_n5, assign21280_e26854_d_n6, assign21280_e26854_d_n8, assign21280_e26854_d_n10, assign21280_e26854_d_n11, assign21280_e26854_d_n12,) = {
    if (((var_guard368 != 0.0) && (var_guard369 == 0.0)) && (var_guard370 == 0.0)) {
        let assign21280_e26851: f64 = (p.p114 - 1.0);
        let assign21280_e26852: f64 = (var_t12).powf(assign21280_e26851);
        (assign21280_e26852, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((var_t12).powf(assign21280_e26851 - 1.0) * var_t12_dn0)) } } else { (assign21280_e26852 * (assign21280_e26851 * (var_t12_dn0 / var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((var_t12).powf(assign21280_e26851 - 1.0) * var_t12_dn2)) } } else { (assign21280_e26852 * (assign21280_e26851 * (var_t12_dn2 / var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((var_t12).powf(assign21280_e26851 - 1.0) * var_t12_dn4)) } } else { (assign21280_e26852 * (assign21280_e26851 * (var_t12_dn4 / var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((var_t12).powf(assign21280_e26851 - 1.0) * var_t12_dn5)) } } else { (assign21280_e26852 * (assign21280_e26851 * (var_t12_dn5 / var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((var_t12).powf(assign21280_e26851 - 1.0) * var_t12_dn6)) } } else { (assign21280_e26852 * (assign21280_e26851 * (var_t12_dn6 / var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((var_t12).powf(assign21280_e26851 - 1.0) * var_t12_dn8)) } } else { (assign21280_e26852 * (assign21280_e26851 * (var_t12_dn8 / var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((var_t12).powf(assign21280_e26851 - 1.0) * var_t12_dn10)) } } else { (assign21280_e26852 * (assign21280_e26851 * (var_t12_dn10 / var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((var_t12).powf(assign21280_e26851 - 1.0) * var_t12_dn11)) } } else { (assign21280_e26852 * (assign21280_e26851 * (var_t12_dn11 / var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((var_t12).powf(assign21280_e26851 - 1.0) * var_t12_dn12)) } } else { (assign21280_e26852 * (assign21280_e26851 * (var_t12_dn12 / var_t12))) },)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
        var_t7 = assign21280_e26854;
        var_t7_dn0 = assign21280_e26854_d_n0;
        var_t7_dn2 = assign21280_e26854_d_n2;
        var_t7_dn4 = assign21280_e26854_d_n4;
        var_t7_dn5 = assign21280_e26854_d_n5;
        var_t7_dn6 = assign21280_e26854_d_n6;
        var_t7_dn8 = assign21280_e26854_d_n8;
        var_t7_dn10 = assign21280_e26854_d_n10;
        var_t7_dn11 = assign21280_e26854_d_n11;
        var_t7_dn12 = assign21280_e26854_d_n12;

        *var_cgsoe_slot = var_cgsoe;
        *var_cgsoe_dn0_slot = var_cgsoe_dn0;
        *var_cgsoe_dn10_slot = var_cgsoe_dn10;
        *var_cgsoe_dn11_slot = var_cgsoe_dn11;
        *var_cgsoe_dn12_slot = var_cgsoe_dn12;
        *var_cgsoe_dn2_slot = var_cgsoe_dn2;
        *var_cgsoe_dn4_slot = var_cgsoe_dn4;
        *var_cgsoe_dn5_slot = var_cgsoe_dn5;
        *var_cgsoe_dn6_slot = var_cgsoe_dn6;
        *var_cgsoe_dn8_slot = var_cgsoe_dn8;
        *var_cite_slot = var_cite;
        *var_eyd_slot = var_eyd;
        *var_eyd_dn0_slot = var_eyd_dn0;
        *var_eyd_dn10_slot = var_eyd_dn10;
        *var_eyd_dn11_slot = var_eyd_dn11;
        *var_eyd_dn12_slot = var_eyd_dn12;
        *var_eyd_dn2_slot = var_eyd_dn2;
        *var_eyd_dn4_slot = var_eyd_dn4;
        *var_eyd_dn5_slot = var_eyd_dn5;
        *var_eyd_dn6_slot = var_eyd_dn6;
        *var_eyd_dn8_slot = var_eyd_dn8;
        *var_guard364_slot = var_guard364;
        *var_guard365_slot = var_guard365;
        *var_guard366_slot = var_guard366;
        *var_guard367_slot = var_guard367;
        *var_guard368_slot = var_guard368;
        *var_guard369_slot = var_guard369;
        *var_guard370_slot = var_guard370;
        *var_nfalpe_slot = var_nfalpe;
        *var_qgod_slot = var_qgod;
        *var_qgod_dn0_slot = var_qgod_dn0;
        *var_qgod_dn10_slot = var_qgod_dn10;
        *var_qgod_dn11_slot = var_qgod_dn11;
        *var_qgod_dn12_slot = var_qgod_dn12;
        *var_qgod_dn2_slot = var_qgod_dn2;
        *var_qgod_dn4_slot = var_qgod_dn4;
        *var_qgod_dn5_slot = var_qgod_dn5;
        *var_qgod_dn6_slot = var_qgod_dn6;
        *var_qgod_dn8_slot = var_qgod_dn8;
        *var_qgos_slot = var_qgos;
        *var_qgos_dn0_slot = var_qgos_dn0;
        *var_qgos_dn10_slot = var_qgos_dn10;
        *var_qgos_dn11_slot = var_qgos_dn11;
        *var_qgos_dn12_slot = var_qgos_dn12;
        *var_qgos_dn2_slot = var_qgos_dn2;
        *var_qgos_dn4_slot = var_qgos_dn4;
        *var_qgos_dn5_slot = var_qgos_dn5;
        *var_qgos_dn6_slot = var_qgos_dn6;
        *var_qgos_dn8_slot = var_qgos_dn8;
        *var_t1_slot = var_t1;
        *var_t12_slot = var_t12;
        *var_t12_dn0_slot = var_t12_dn0;
        *var_t12_dn10_slot = var_t12_dn10;
        *var_t12_dn11_slot = var_t12_dn11;
        *var_t12_dn12_slot = var_t12_dn12;
        *var_t12_dn2_slot = var_t12_dn2;
        *var_t12_dn4_slot = var_t12_dn4;
        *var_t12_dn5_slot = var_t12_dn5;
        *var_t12_dn6_slot = var_t12_dn6;
        *var_t12_dn8_slot = var_t12_dn8;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn12_slot = var_t7_dn12;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_tau_slot = var_tau;
        *var_tau_dn0_slot = var_tau_dn0;
        *var_tau_dn10_slot = var_tau_dn10;
        *var_tau_dn11_slot = var_tau_dn11;
        *var_tau_dn12_slot = var_tau_dn12;
        *var_tau_dn2_slot = var_tau_dn2;
        *var_tau_dn4_slot = var_tau_dn4;
        *var_tau_dn5_slot = var_tau_dn5;
        *var_tau_dn6_slot = var_tau_dn6;
        *var_tau_dn8_slot = var_tau_dn8;
        *var_taub_slot = var_taub;
        *var_taub_dn0_slot = var_taub_dn0;
        *var_taub_dn10_slot = var_taub_dn10;
        *var_taub_dn11_slot = var_taub_dn11;
        *var_taub_dn12_slot = var_taub_dn12;
        *var_taub_dn2_slot = var_taub_dn2;
        *var_taub_dn4_slot = var_taub_dn4;
        *var_taub_dn5_slot = var_taub_dn5;
        *var_taub_dn6_slot = var_taub_dn6;
        *var_taub_dn8_slot = var_taub_dn8;
    }

    pub(super) fn stamp_transient_block_82(
        p: &Parameters,
        var_alpha: f64,
        var_alpha_dn0: f64,
        var_alpha_dn10: f64,
        var_alpha_dn11: f64,
        var_alpha_dn12: f64,
        var_alpha_dn2: f64,
        var_alpha_dn4: f64,
        var_alpha_dn5: f64,
        var_alpha_dn6: f64,
        var_alpha_dn8: f64,
        var_c_fox: f64,
        var_c_fox_dn0: f64,
        var_c_fox_dn10: f64,
        var_c_fox_dn11: f64,
        var_c_fox_dn12: f64,
        var_c_fox_dn2: f64,
        var_c_fox_dn4: f64,
        var_c_fox_dn5: f64,
        var_c_fox_dn6: f64,
        var_c_fox_dn8: f64,
        var_cgbo_given: f64,
        var_flg_ign: f64,
        var_flg_noqi: f64,
        var_guard368: f64,
        var_idsibpc: f64,
        var_idsibpc_dn0: f64,
        var_idsibpc_dn10: f64,
        var_idsibpc_dn11: f64,
        var_idsibpc_dn12: f64,
        var_idsibpc_dn2: f64,
        var_idsibpc_dn4: f64,
        var_idsibpc_dn5: f64,
        var_idsibpc_dn6: f64,
        var_idsibpc_dn8: f64,
        var_kusai00: f64,
        var_kusai00_dn0: f64,
        var_kusai00_dn10: f64,
        var_kusai00_dn11: f64,
        var_kusai00_dn12: f64,
        var_kusai00_dn2: f64,
        var_kusai00_dn4: f64,
        var_kusai00_dn5: f64,
        var_kusai00_dn6: f64,
        var_kusai00_dn8: f64,
        var_kusai00l: f64,
        var_kusai00l_dn0: f64,
        var_kusai00l_dn10: f64,
        var_kusai00l_dn11: f64,
        var_kusai00l_dn12: f64,
        var_kusai00l_dn2: f64,
        var_kusai00l_dn4: f64,
        var_kusai00l_dn5: f64,
        var_kusai00l_dn6: f64,
        var_kusai00l_dn8: f64,
        var_kusail: f64,
        var_kusail_dn0: f64,
        var_kusail_dn10: f64,
        var_kusail_dn11: f64,
        var_kusail_dn12: f64,
        var_kusail_dn2: f64,
        var_kusail_dn4: f64,
        var_kusail_dn5: f64,
        var_kusail_dn6: f64,
        var_kusail_dn8: f64,
        var_lch: f64,
        var_lch_dn0: f64,
        var_lch_dn10: f64,
        var_lch_dn11: f64,
        var_lch_dn12: f64,
        var_lch_dn2: f64,
        var_lch_dn4: f64,
        var_lch_dn5: f64,
        var_lch_dn6: f64,
        var_lch_dn8: f64,
        var_lgate: f64,
        var_mu: f64,
        var_mu_dn0: f64,
        var_mu_dn10: f64,
        var_mu_dn11: f64,
        var_mu_dn12: f64,
        var_mu_dn2: f64,
        var_mu_dn4: f64,
        var_mu_dn5: f64,
        var_mu_dn6: f64,
        var_mu_dn8: f64,
        var_muun: f64,
        var_muun_dn0: f64,
        var_muun_dn10: f64,
        var_muun_dn11: f64,
        var_muun_dn12: f64,
        var_muun_dn2: f64,
        var_muun_dn4: f64,
        var_muun_dn5: f64,
        var_muun_dn6: f64,
        var_muun_dn8: f64,
        var_t12: f64,
        var_t12_dn0: f64,
        var_t12_dn10: f64,
        var_t12_dn11: f64,
        var_t12_dn12: f64,
        var_t12_dn2: f64,
        var_t12_dn4: f64,
        var_t12_dn5: f64,
        var_t12_dn6: f64,
        var_t12_dn8: f64,
        var_vbse: f64,
        var_vbse_dn0: f64,
        var_vbse_dn2: f64,
        var_vbse_dn6: f64,
        var_vgse: f64,
        var_vgse_dn0: f64,
        var_vgse_dn2: f64,
        var_vgse_dn5: f64,
        var_vgvt: f64,
        var_vgvt_dn0: f64,
        var_vgvt_dn10: f64,
        var_vgvt_dn11: f64,
        var_vgvt_dn12: f64,
        var_vgvt_dn2: f64,
        var_vgvt_dn4: f64,
        var_vgvt_dn5: f64,
        var_vgvt_dn6: f64,
        var_vgvt_dn8: f64,
        var_weff_nf: f64,
        var_weff_nf_dn0: f64,
        var_weff_nf_dn10: f64,
        var_weff_nf_dn11: f64,
        var_weff_nf_dn12: f64,
        var_weff_nf_dn2: f64,
        var_weff_nf_dn4: f64,
        var_weff_nf_dn5: f64,
        var_weff_nf_dn6: f64,
        var_weff_nf_dn8: f64,
        var_cgbe_slot: &mut f64,
        var_crl_f_slot: &mut f64,
        var_crl_f_dn0_slot: &mut f64,
        var_crl_f_dn10_slot: &mut f64,
        var_crl_f_dn11_slot: &mut f64,
        var_crl_f_dn12_slot: &mut f64,
        var_crl_f_dn2_slot: &mut f64,
        var_crl_f_dn4_slot: &mut f64,
        var_crl_f_dn5_slot: &mut f64,
        var_crl_f_dn6_slot: &mut f64,
        var_crl_f_dn8_slot: &mut f64,
        var_gamma_slot: &mut f64,
        var_gamma_dn0_slot: &mut f64,
        var_gamma_dn10_slot: &mut f64,
        var_gamma_dn11_slot: &mut f64,
        var_gamma_dn12_slot: &mut f64,
        var_gamma_dn2_slot: &mut f64,
        var_gamma_dn4_slot: &mut f64,
        var_gamma_dn5_slot: &mut f64,
        var_gamma_dn6_slot: &mut f64,
        var_gamma_dn8_slot: &mut f64,
        var_gds0_h2_slot: &mut f64,
        var_gds0_h2_dn0_slot: &mut f64,
        var_gds0_h2_dn10_slot: &mut f64,
        var_gds0_h2_dn11_slot: &mut f64,
        var_gds0_h2_dn12_slot: &mut f64,
        var_gds0_h2_dn2_slot: &mut f64,
        var_gds0_h2_dn4_slot: &mut f64,
        var_gds0_h2_dn5_slot: &mut f64,
        var_gds0_h2_dn6_slot: &mut f64,
        var_gds0_h2_dn8_slot: &mut f64,
        var_gds0_ign_slot: &mut f64,
        var_gds0_ign_dn0_slot: &mut f64,
        var_gds0_ign_dn10_slot: &mut f64,
        var_gds0_ign_dn11_slot: &mut f64,
        var_gds0_ign_dn12_slot: &mut f64,
        var_gds0_ign_dn2_slot: &mut f64,
        var_gds0_ign_dn4_slot: &mut f64,
        var_gds0_ign_dn5_slot: &mut f64,
        var_gds0_ign_dn6_slot: &mut f64,
        var_gds0_ign_dn8_slot: &mut f64,
        var_guard371_slot: &mut f64,
        var_ids_slot: &mut f64,
        var_ids_dn0_slot: &mut f64,
        var_ids_dn10_slot: &mut f64,
        var_ids_dn11_slot: &mut f64,
        var_ids_dn12_slot: &mut f64,
        var_ids_dn2_slot: &mut f64,
        var_ids_dn4_slot: &mut f64,
        var_ids_dn5_slot: &mut f64,
        var_ids_dn6_slot: &mut f64,
        var_ids_dn8_slot: &mut f64,
        var_kusai_ig_slot: &mut f64,
        var_kusai_ig_dn0_slot: &mut f64,
        var_kusai_ig_dn10_slot: &mut f64,
        var_kusai_ig_dn11_slot: &mut f64,
        var_kusai_ig_dn12_slot: &mut f64,
        var_kusai_ig_dn2_slot: &mut f64,
        var_kusai_ig_dn4_slot: &mut f64,
        var_kusai_ig_dn5_slot: &mut f64,
        var_kusai_ig_dn6_slot: &mut f64,
        var_kusai_ig_dn8_slot: &mut f64,
        var_mu_ave_slot: &mut f64,
        var_mu_ave_dn0_slot: &mut f64,
        var_mu_ave_dn10_slot: &mut f64,
        var_mu_ave_dn11_slot: &mut f64,
        var_mu_ave_dn12_slot: &mut f64,
        var_mu_ave_dn2_slot: &mut f64,
        var_mu_ave_dn4_slot: &mut f64,
        var_mu_ave_dn5_slot: &mut f64,
        var_mu_ave_dn6_slot: &mut f64,
        var_mu_ave_dn8_slot: &mut f64,
        var_mud_hoso_slot: &mut f64,
        var_mud_hoso_dn0_slot: &mut f64,
        var_mud_hoso_dn10_slot: &mut f64,
        var_mud_hoso_dn11_slot: &mut f64,
        var_mud_hoso_dn12_slot: &mut f64,
        var_mud_hoso_dn2_slot: &mut f64,
        var_mud_hoso_dn4_slot: &mut f64,
        var_mud_hoso_dn5_slot: &mut f64,
        var_mud_hoso_dn6_slot: &mut f64,
        var_mud_hoso_dn8_slot: &mut f64,
        var_nthrml_slot: &mut f64,
        var_nthrml_dn0_slot: &mut f64,
        var_nthrml_dn10_slot: &mut f64,
        var_nthrml_dn11_slot: &mut f64,
        var_nthrml_dn12_slot: &mut f64,
        var_nthrml_dn2_slot: &mut f64,
        var_nthrml_dn4_slot: &mut f64,
        var_nthrml_dn5_slot: &mut f64,
        var_nthrml_dn6_slot: &mut f64,
        var_nthrml_dn8_slot: &mut f64,
        var_qgob_slot: &mut f64,
        var_qgob_dn0_slot: &mut f64,
        var_qgob_dn2_slot: &mut f64,
        var_qgob_dn5_slot: &mut f64,
        var_qgob_dn6_slot: &mut f64,
        var_sqrtkusail_slot: &mut f64,
        var_sqrtkusail_dn0_slot: &mut f64,
        var_sqrtkusail_dn10_slot: &mut f64,
        var_sqrtkusail_dn11_slot: &mut f64,
        var_sqrtkusail_dn12_slot: &mut f64,
        var_sqrtkusail_dn2_slot: &mut f64,
        var_sqrtkusail_dn4_slot: &mut f64,
        var_sqrtkusail_dn5_slot: &mut f64,
        var_sqrtkusail_dn6_slot: &mut f64,
        var_sqrtkusail_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t10_slot: &mut f64,
        var_t10_dn0_slot: &mut f64,
        var_t10_dn10_slot: &mut f64,
        var_t10_dn11_slot: &mut f64,
        var_t10_dn12_slot: &mut f64,
        var_t10_dn2_slot: &mut f64,
        var_t10_dn4_slot: &mut f64,
        var_t10_dn5_slot: &mut f64,
        var_t10_dn6_slot: &mut f64,
        var_t10_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn12_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t9_slot: &mut f64,
        var_t9_dn0_slot: &mut f64,
        var_t9_dn10_slot: &mut f64,
        var_t9_dn11_slot: &mut f64,
        var_t9_dn12_slot: &mut f64,
        var_t9_dn2_slot: &mut f64,
        var_t9_dn4_slot: &mut f64,
        var_t9_dn5_slot: &mut f64,
        var_t9_dn6_slot: &mut f64,
        var_t9_dn8_slot: &mut f64,
    ) {
        let mut var_cgbe: f64 = *var_cgbe_slot;
        let mut var_crl_f: f64 = *var_crl_f_slot;
        let mut var_crl_f_dn0: f64 = *var_crl_f_dn0_slot;
        let mut var_crl_f_dn10: f64 = *var_crl_f_dn10_slot;
        let mut var_crl_f_dn11: f64 = *var_crl_f_dn11_slot;
        let mut var_crl_f_dn12: f64 = *var_crl_f_dn12_slot;
        let mut var_crl_f_dn2: f64 = *var_crl_f_dn2_slot;
        let mut var_crl_f_dn4: f64 = *var_crl_f_dn4_slot;
        let mut var_crl_f_dn5: f64 = *var_crl_f_dn5_slot;
        let mut var_crl_f_dn6: f64 = *var_crl_f_dn6_slot;
        let mut var_crl_f_dn8: f64 = *var_crl_f_dn8_slot;
        let mut var_gamma: f64 = *var_gamma_slot;
        let mut var_gamma_dn0: f64 = *var_gamma_dn0_slot;
        let mut var_gamma_dn10: f64 = *var_gamma_dn10_slot;
        let mut var_gamma_dn11: f64 = *var_gamma_dn11_slot;
        let mut var_gamma_dn12: f64 = *var_gamma_dn12_slot;
        let mut var_gamma_dn2: f64 = *var_gamma_dn2_slot;
        let mut var_gamma_dn4: f64 = *var_gamma_dn4_slot;
        let mut var_gamma_dn5: f64 = *var_gamma_dn5_slot;
        let mut var_gamma_dn6: f64 = *var_gamma_dn6_slot;
        let mut var_gamma_dn8: f64 = *var_gamma_dn8_slot;
        let mut var_gds0_h2: f64 = *var_gds0_h2_slot;
        let mut var_gds0_h2_dn0: f64 = *var_gds0_h2_dn0_slot;
        let mut var_gds0_h2_dn10: f64 = *var_gds0_h2_dn10_slot;
        let mut var_gds0_h2_dn11: f64 = *var_gds0_h2_dn11_slot;
        let mut var_gds0_h2_dn12: f64 = *var_gds0_h2_dn12_slot;
        let mut var_gds0_h2_dn2: f64 = *var_gds0_h2_dn2_slot;
        let mut var_gds0_h2_dn4: f64 = *var_gds0_h2_dn4_slot;
        let mut var_gds0_h2_dn5: f64 = *var_gds0_h2_dn5_slot;
        let mut var_gds0_h2_dn6: f64 = *var_gds0_h2_dn6_slot;
        let mut var_gds0_h2_dn8: f64 = *var_gds0_h2_dn8_slot;
        let mut var_gds0_ign: f64 = *var_gds0_ign_slot;
        let mut var_gds0_ign_dn0: f64 = *var_gds0_ign_dn0_slot;
        let mut var_gds0_ign_dn10: f64 = *var_gds0_ign_dn10_slot;
        let mut var_gds0_ign_dn11: f64 = *var_gds0_ign_dn11_slot;
        let mut var_gds0_ign_dn12: f64 = *var_gds0_ign_dn12_slot;
        let mut var_gds0_ign_dn2: f64 = *var_gds0_ign_dn2_slot;
        let mut var_gds0_ign_dn4: f64 = *var_gds0_ign_dn4_slot;
        let mut var_gds0_ign_dn5: f64 = *var_gds0_ign_dn5_slot;
        let mut var_gds0_ign_dn6: f64 = *var_gds0_ign_dn6_slot;
        let mut var_gds0_ign_dn8: f64 = *var_gds0_ign_dn8_slot;
        let mut var_guard371: f64 = *var_guard371_slot;
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn0: f64 = *var_ids_dn0_slot;
        let mut var_ids_dn10: f64 = *var_ids_dn10_slot;
        let mut var_ids_dn11: f64 = *var_ids_dn11_slot;
        let mut var_ids_dn12: f64 = *var_ids_dn12_slot;
        let mut var_ids_dn2: f64 = *var_ids_dn2_slot;
        let mut var_ids_dn4: f64 = *var_ids_dn4_slot;
        let mut var_ids_dn5: f64 = *var_ids_dn5_slot;
        let mut var_ids_dn6: f64 = *var_ids_dn6_slot;
        let mut var_ids_dn8: f64 = *var_ids_dn8_slot;
        let mut var_kusai_ig: f64 = *var_kusai_ig_slot;
        let mut var_kusai_ig_dn0: f64 = *var_kusai_ig_dn0_slot;
        let mut var_kusai_ig_dn10: f64 = *var_kusai_ig_dn10_slot;
        let mut var_kusai_ig_dn11: f64 = *var_kusai_ig_dn11_slot;
        let mut var_kusai_ig_dn12: f64 = *var_kusai_ig_dn12_slot;
        let mut var_kusai_ig_dn2: f64 = *var_kusai_ig_dn2_slot;
        let mut var_kusai_ig_dn4: f64 = *var_kusai_ig_dn4_slot;
        let mut var_kusai_ig_dn5: f64 = *var_kusai_ig_dn5_slot;
        let mut var_kusai_ig_dn6: f64 = *var_kusai_ig_dn6_slot;
        let mut var_kusai_ig_dn8: f64 = *var_kusai_ig_dn8_slot;
        let mut var_mu_ave: f64 = *var_mu_ave_slot;
        let mut var_mu_ave_dn0: f64 = *var_mu_ave_dn0_slot;
        let mut var_mu_ave_dn10: f64 = *var_mu_ave_dn10_slot;
        let mut var_mu_ave_dn11: f64 = *var_mu_ave_dn11_slot;
        let mut var_mu_ave_dn12: f64 = *var_mu_ave_dn12_slot;
        let mut var_mu_ave_dn2: f64 = *var_mu_ave_dn2_slot;
        let mut var_mu_ave_dn4: f64 = *var_mu_ave_dn4_slot;
        let mut var_mu_ave_dn5: f64 = *var_mu_ave_dn5_slot;
        let mut var_mu_ave_dn6: f64 = *var_mu_ave_dn6_slot;
        let mut var_mu_ave_dn8: f64 = *var_mu_ave_dn8_slot;
        let mut var_mud_hoso: f64 = *var_mud_hoso_slot;
        let mut var_mud_hoso_dn0: f64 = *var_mud_hoso_dn0_slot;
        let mut var_mud_hoso_dn10: f64 = *var_mud_hoso_dn10_slot;
        let mut var_mud_hoso_dn11: f64 = *var_mud_hoso_dn11_slot;
        let mut var_mud_hoso_dn12: f64 = *var_mud_hoso_dn12_slot;
        let mut var_mud_hoso_dn2: f64 = *var_mud_hoso_dn2_slot;
        let mut var_mud_hoso_dn4: f64 = *var_mud_hoso_dn4_slot;
        let mut var_mud_hoso_dn5: f64 = *var_mud_hoso_dn5_slot;
        let mut var_mud_hoso_dn6: f64 = *var_mud_hoso_dn6_slot;
        let mut var_mud_hoso_dn8: f64 = *var_mud_hoso_dn8_slot;
        let mut var_nthrml: f64 = *var_nthrml_slot;
        let mut var_nthrml_dn0: f64 = *var_nthrml_dn0_slot;
        let mut var_nthrml_dn10: f64 = *var_nthrml_dn10_slot;
        let mut var_nthrml_dn11: f64 = *var_nthrml_dn11_slot;
        let mut var_nthrml_dn12: f64 = *var_nthrml_dn12_slot;
        let mut var_nthrml_dn2: f64 = *var_nthrml_dn2_slot;
        let mut var_nthrml_dn4: f64 = *var_nthrml_dn4_slot;
        let mut var_nthrml_dn5: f64 = *var_nthrml_dn5_slot;
        let mut var_nthrml_dn6: f64 = *var_nthrml_dn6_slot;
        let mut var_nthrml_dn8: f64 = *var_nthrml_dn8_slot;
        let mut var_qgob: f64 = *var_qgob_slot;
        let mut var_qgob_dn0: f64 = *var_qgob_dn0_slot;
        let mut var_qgob_dn2: f64 = *var_qgob_dn2_slot;
        let mut var_qgob_dn5: f64 = *var_qgob_dn5_slot;
        let mut var_qgob_dn6: f64 = *var_qgob_dn6_slot;
        let mut var_sqrtkusail: f64 = *var_sqrtkusail_slot;
        let mut var_sqrtkusail_dn0: f64 = *var_sqrtkusail_dn0_slot;
        let mut var_sqrtkusail_dn10: f64 = *var_sqrtkusail_dn10_slot;
        let mut var_sqrtkusail_dn11: f64 = *var_sqrtkusail_dn11_slot;
        let mut var_sqrtkusail_dn12: f64 = *var_sqrtkusail_dn12_slot;
        let mut var_sqrtkusail_dn2: f64 = *var_sqrtkusail_dn2_slot;
        let mut var_sqrtkusail_dn4: f64 = *var_sqrtkusail_dn4_slot;
        let mut var_sqrtkusail_dn5: f64 = *var_sqrtkusail_dn5_slot;
        let mut var_sqrtkusail_dn6: f64 = *var_sqrtkusail_dn6_slot;
        let mut var_sqrtkusail_dn8: f64 = *var_sqrtkusail_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t10: f64 = *var_t10_slot;
        let mut var_t10_dn0: f64 = *var_t10_dn0_slot;
        let mut var_t10_dn10: f64 = *var_t10_dn10_slot;
        let mut var_t10_dn11: f64 = *var_t10_dn11_slot;
        let mut var_t10_dn12: f64 = *var_t10_dn12_slot;
        let mut var_t10_dn2: f64 = *var_t10_dn2_slot;
        let mut var_t10_dn4: f64 = *var_t10_dn4_slot;
        let mut var_t10_dn5: f64 = *var_t10_dn5_slot;
        let mut var_t10_dn6: f64 = *var_t10_dn6_slot;
        let mut var_t10_dn8: f64 = *var_t10_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn12: f64 = *var_t7_dn12_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t9: f64 = *var_t9_slot;
        let mut var_t9_dn0: f64 = *var_t9_dn0_slot;
        let mut var_t9_dn10: f64 = *var_t9_dn10_slot;
        let mut var_t9_dn11: f64 = *var_t9_dn11_slot;
        let mut var_t9_dn12: f64 = *var_t9_dn12_slot;
        let mut var_t9_dn2: f64 = *var_t9_dn2_slot;
        let mut var_t9_dn4: f64 = *var_t9_dn4_slot;
        let mut var_t9_dn5: f64 = *var_t9_dn5_slot;
        let mut var_t9_dn6: f64 = *var_t9_dn6_slot;
        let mut var_t9_dn8: f64 = *var_t9_dn8_slot;

        let (assign21290_e26862, assign21290_e26862_d_n0, assign21290_e26862_d_n2, assign21290_e26862_d_n4, assign21290_e26862_d_n5, assign21290_e26862_d_n6, assign21290_e26862_d_n8, assign21290_e26862_d_n10, assign21290_e26862_d_n11, assign21290_e26862_d_n12,) = {
    if (var_guard368 != 0.0) {
        let assign21290_e26859: f64 = (var_t12 * var_t7);
        let assign21290_e26860: f64 = (1.0 + assign21290_e26859);
        (assign21290_e26860, ((var_t12_dn0 * var_t7) + (var_t12 * var_t7_dn0)), ((var_t12_dn2 * var_t7) + (var_t12 * var_t7_dn2)), ((var_t12_dn4 * var_t7) + (var_t12 * var_t7_dn4)), ((var_t12_dn5 * var_t7) + (var_t12 * var_t7_dn5)), ((var_t12_dn6 * var_t7) + (var_t12 * var_t7_dn6)), ((var_t12_dn8 * var_t7) + (var_t12 * var_t7_dn8)), ((var_t12_dn10 * var_t7) + (var_t12 * var_t7_dn10)), ((var_t12_dn11 * var_t7) + (var_t12 * var_t7_dn11)), ((var_t12_dn12 * var_t7) + (var_t12 * var_t7_dn12)),)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn8, var_t9_dn10, var_t9_dn11, var_t9_dn12,)
    }
};
        var_t9 = assign21290_e26862;
        var_t9_dn0 = assign21290_e26862_d_n0;
        var_t9_dn2 = assign21290_e26862_d_n2;
        var_t9_dn4 = assign21290_e26862_d_n4;
        var_t9_dn5 = assign21290_e26862_d_n5;
        var_t9_dn6 = assign21290_e26862_d_n6;
        var_t9_dn8 = assign21290_e26862_d_n8;
        var_t9_dn10 = assign21290_e26862_d_n10;
        var_t9_dn11 = assign21290_e26862_d_n11;
        var_t9_dn12 = assign21290_e26862_d_n12;

        let (assign21300_e26873, assign21300_e26873_d_n0, assign21300_e26873_d_n2, assign21300_e26873_d_n4, assign21300_e26873_d_n5, assign21300_e26873_d_n6, assign21300_e26873_d_n8, assign21300_e26873_d_n10, assign21300_e26873_d_n11, assign21300_e26873_d_n12,) = {
    if (var_guard368 != 0.0) {
        let assign21300_e26866: f64 = (-1.0);
        let assign21300_e26868: f64 = (assign21300_e26866 / p.p114);
        let assign21300_e26870: f64 = (assign21300_e26868 - 1.0);
        let assign21300_e26871: f64 = (var_t9).powf(assign21300_e26870);
        (assign21300_e26871, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((var_t9).powf(assign21300_e26870 - 1.0) * var_t9_dn0)) } } else { (assign21300_e26871 * (assign21300_e26870 * (var_t9_dn0 / var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((var_t9).powf(assign21300_e26870 - 1.0) * var_t9_dn2)) } } else { (assign21300_e26871 * (assign21300_e26870 * (var_t9_dn2 / var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((var_t9).powf(assign21300_e26870 - 1.0) * var_t9_dn4)) } } else { (assign21300_e26871 * (assign21300_e26870 * (var_t9_dn4 / var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((var_t9).powf(assign21300_e26870 - 1.0) * var_t9_dn5)) } } else { (assign21300_e26871 * (assign21300_e26870 * (var_t9_dn5 / var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((var_t9).powf(assign21300_e26870 - 1.0) * var_t9_dn6)) } } else { (assign21300_e26871 * (assign21300_e26870 * (var_t9_dn6 / var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((var_t9).powf(assign21300_e26870 - 1.0) * var_t9_dn8)) } } else { (assign21300_e26871 * (assign21300_e26870 * (var_t9_dn8 / var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((var_t9).powf(assign21300_e26870 - 1.0) * var_t9_dn10)) } } else { (assign21300_e26871 * (assign21300_e26870 * (var_t9_dn10 / var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((var_t9).powf(assign21300_e26870 - 1.0) * var_t9_dn11)) } } else { (assign21300_e26871 * (assign21300_e26870 * (var_t9_dn11 / var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((var_t9).powf(assign21300_e26870 - 1.0) * var_t9_dn12)) } } else { (assign21300_e26871 * (assign21300_e26870 * (var_t9_dn12 / var_t9))) },)
    } else {
        (var_t10, var_t10_dn0, var_t10_dn2, var_t10_dn4, var_t10_dn5, var_t10_dn6, var_t10_dn8, var_t10_dn10, var_t10_dn11, var_t10_dn12,)
    }
};
        var_t10 = assign21300_e26873;
        var_t10_dn0 = assign21300_e26873_d_n0;
        var_t10_dn2 = assign21300_e26873_d_n2;
        var_t10_dn4 = assign21300_e26873_d_n4;
        var_t10_dn5 = assign21300_e26873_d_n5;
        var_t10_dn6 = assign21300_e26873_d_n6;
        var_t10_dn8 = assign21300_e26873_d_n8;
        var_t10_dn10 = assign21300_e26873_d_n10;
        var_t10_dn11 = assign21300_e26873_d_n11;
        var_t10_dn12 = assign21300_e26873_d_n12;

        let (assign21310_e26881, assign21310_e26881_d_n0, assign21310_e26881_d_n2, assign21310_e26881_d_n4, assign21310_e26881_d_n5, assign21310_e26881_d_n6, assign21310_e26881_d_n8, assign21310_e26881_d_n10, assign21310_e26881_d_n11, assign21310_e26881_d_n12,) = {
    if (var_guard368 != 0.0) {
        let assign21310_e26877: f64 = (var_muun * var_t9);
        let assign21310_e26879: f64 = (assign21310_e26877 * var_t10);
        (assign21310_e26879, ((((var_muun_dn0 * var_t9) + (var_muun * var_t9_dn0)) * var_t10) + (assign21310_e26877 * var_t10_dn0)), ((((var_muun_dn2 * var_t9) + (var_muun * var_t9_dn2)) * var_t10) + (assign21310_e26877 * var_t10_dn2)), ((((var_muun_dn4 * var_t9) + (var_muun * var_t9_dn4)) * var_t10) + (assign21310_e26877 * var_t10_dn4)), ((((var_muun_dn5 * var_t9) + (var_muun * var_t9_dn5)) * var_t10) + (assign21310_e26877 * var_t10_dn5)), ((((var_muun_dn6 * var_t9) + (var_muun * var_t9_dn6)) * var_t10) + (assign21310_e26877 * var_t10_dn6)), ((((var_muun_dn8 * var_t9) + (var_muun * var_t9_dn8)) * var_t10) + (assign21310_e26877 * var_t10_dn8)), ((((var_muun_dn10 * var_t9) + (var_muun * var_t9_dn10)) * var_t10) + (assign21310_e26877 * var_t10_dn10)), ((((var_muun_dn11 * var_t9) + (var_muun * var_t9_dn11)) * var_t10) + (assign21310_e26877 * var_t10_dn11)), ((((var_muun_dn12 * var_t9) + (var_muun * var_t9_dn12)) * var_t10) + (assign21310_e26877 * var_t10_dn12)),)
    } else {
        (var_mud_hoso, var_mud_hoso_dn0, var_mud_hoso_dn2, var_mud_hoso_dn4, var_mud_hoso_dn5, var_mud_hoso_dn6, var_mud_hoso_dn8, var_mud_hoso_dn10, var_mud_hoso_dn11, var_mud_hoso_dn12,)
    }
};
        var_mud_hoso = assign21310_e26881;
        var_mud_hoso_dn0 = assign21310_e26881_d_n0;
        var_mud_hoso_dn2 = assign21310_e26881_d_n2;
        var_mud_hoso_dn4 = assign21310_e26881_d_n4;
        var_mud_hoso_dn5 = assign21310_e26881_d_n5;
        var_mud_hoso_dn6 = assign21310_e26881_d_n6;
        var_mud_hoso_dn8 = assign21310_e26881_d_n8;
        var_mud_hoso_dn10 = assign21310_e26881_d_n10;
        var_mud_hoso_dn11 = assign21310_e26881_d_n11;
        var_mud_hoso_dn12 = assign21310_e26881_d_n12;

        let (assign21320_e26889, assign21320_e26889_d_n0, assign21320_e26889_d_n2, assign21320_e26889_d_n4, assign21320_e26889_d_n5, assign21320_e26889_d_n6, assign21320_e26889_d_n8, assign21320_e26889_d_n10, assign21320_e26889_d_n11, assign21320_e26889_d_n12,) = {
    if (var_guard368 != 0.0) {
        let assign21320_e26885: f64 = (var_mu + var_mud_hoso);
        let assign21320_e26887: f64 = (assign21320_e26885 / 2.0);
        (assign21320_e26887, ((var_mu_dn0 + var_mud_hoso_dn0) / 2.0), ((var_mu_dn2 + var_mud_hoso_dn2) / 2.0), ((var_mu_dn4 + var_mud_hoso_dn4) / 2.0), ((var_mu_dn5 + var_mud_hoso_dn5) / 2.0), ((var_mu_dn6 + var_mud_hoso_dn6) / 2.0), ((var_mu_dn8 + var_mud_hoso_dn8) / 2.0), ((var_mu_dn10 + var_mud_hoso_dn10) / 2.0), ((var_mu_dn11 + var_mud_hoso_dn11) / 2.0), ((var_mu_dn12 + var_mud_hoso_dn12) / 2.0),)
    } else {
        (var_mu_ave, var_mu_ave_dn0, var_mu_ave_dn2, var_mu_ave_dn4, var_mu_ave_dn5, var_mu_ave_dn6, var_mu_ave_dn8, var_mu_ave_dn10, var_mu_ave_dn11, var_mu_ave_dn12,)
    }
};
        var_mu_ave = assign21320_e26889;
        var_mu_ave_dn0 = assign21320_e26889_d_n0;
        var_mu_ave_dn2 = assign21320_e26889_d_n2;
        var_mu_ave_dn4 = assign21320_e26889_d_n4;
        var_mu_ave_dn5 = assign21320_e26889_d_n5;
        var_mu_ave_dn6 = assign21320_e26889_d_n6;
        var_mu_ave_dn8 = assign21320_e26889_d_n8;
        var_mu_ave_dn10 = assign21320_e26889_d_n10;
        var_mu_ave_dn11 = assign21320_e26889_d_n11;
        var_mu_ave_dn12 = assign21320_e26889_d_n12;

        let (assign21330_e26895, assign21330_e26895_d_n0, assign21330_e26895_d_n2, assign21330_e26895_d_n4, assign21330_e26895_d_n5, assign21330_e26895_d_n6, assign21330_e26895_d_n8, assign21330_e26895_d_n10, assign21330_e26895_d_n11, assign21330_e26895_d_n12,) = {
    if (var_guard368 != 0.0) {
        let assign21330_e26893: f64 = (var_alpha * var_alpha);
        (assign21330_e26893, ((var_alpha_dn0 * var_alpha) + (var_alpha * var_alpha_dn0)), ((var_alpha_dn2 * var_alpha) + (var_alpha * var_alpha_dn2)), ((var_alpha_dn4 * var_alpha) + (var_alpha * var_alpha_dn4)), ((var_alpha_dn5 * var_alpha) + (var_alpha * var_alpha_dn5)), ((var_alpha_dn6 * var_alpha) + (var_alpha * var_alpha_dn6)), ((var_alpha_dn8 * var_alpha) + (var_alpha * var_alpha_dn8)), ((var_alpha_dn10 * var_alpha) + (var_alpha * var_alpha_dn10)), ((var_alpha_dn11 * var_alpha) + (var_alpha * var_alpha_dn11)), ((var_alpha_dn12 * var_alpha) + (var_alpha * var_alpha_dn12)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
        var_t0 = assign21330_e26895;
        var_t0_dn0 = assign21330_e26895_d_n0;
        var_t0_dn2 = assign21330_e26895_d_n2;
        var_t0_dn4 = assign21330_e26895_d_n4;
        var_t0_dn5 = assign21330_e26895_d_n5;
        var_t0_dn6 = assign21330_e26895_d_n6;
        var_t0_dn8 = assign21330_e26895_d_n8;
        var_t0_dn10 = assign21330_e26895_d_n10;
        var_t0_dn11 = assign21330_e26895_d_n11;
        var_t0_dn12 = assign21330_e26895_d_n12;

        let (assign21340_e26957, assign21340_e26957_d_n0, assign21340_e26957_d_n2, assign21340_e26957_d_n4, assign21340_e26957_d_n5, assign21340_e26957_d_n6, assign21340_e26957_d_n8, assign21340_e26957_d_n10, assign21340_e26957_d_n11, assign21340_e26957_d_n12,) = {
    if (var_guard368 != 0.0) {
        let assign21340_e26899: f64 = (var_weff_nf * var_c_fox);
        let assign21340_e26901: f64 = (assign21340_e26899 * var_vgvt);
        let assign21340_e26903: f64 = (assign21340_e26901 * var_mu);
        let assign21340_e26907: f64 = (3.0 * var_alpha);
        let assign21340_e26908: f64 = (1.0 + assign21340_e26907);
        let assign21340_e26911: f64 = (6.0 * var_t0);
        let assign21340_e26912: f64 = (assign21340_e26908 + assign21340_e26911);
        let assign21340_e26914: f64 = (assign21340_e26912 * var_mud_hoso);
        let assign21340_e26916: f64 = (assign21340_e26914 * var_mud_hoso);
        let assign21340_e26920: f64 = (4.0 * var_alpha);
        let assign21340_e26921: f64 = (3.0 + assign21340_e26920);
        let assign21340_e26924: f64 = (3.0 * var_t0);
        let assign21340_e26925: f64 = (assign21340_e26921 + assign21340_e26924);
        let assign21340_e26927: f64 = (assign21340_e26925 * var_mud_hoso);
        let assign21340_e26929: f64 = (assign21340_e26927 * var_mu);
        let assign21340_e26930: f64 = (assign21340_e26916 + assign21340_e26929);
        let assign21340_e26934: f64 = (3.0 * var_alpha);
        let assign21340_e26935: f64 = (6.0 + assign21340_e26934);
        let assign21340_e26937: f64 = (assign21340_e26935 + var_t0);
        let assign21340_e26939: f64 = (assign21340_e26937 * var_mu);
        let assign21340_e26941: f64 = (assign21340_e26939 * var_mu);
        let assign21340_e26942: f64 = (assign21340_e26930 + assign21340_e26941);
        let assign21340_e26943: f64 = (assign21340_e26903 * assign21340_e26942);
        let assign21340_e26946: f64 = (15.0 * var_lch);
        let assign21340_e26949: f64 = (1.0 + var_alpha);
        let assign21340_e26950: f64 = (assign21340_e26946 * assign21340_e26949);
        let assign21340_e26952: f64 = (assign21340_e26950 * var_mu_ave);
        let assign21340_e26954: f64 = (assign21340_e26952 * var_mu_ave);
        let assign21340_e26955: f64 = (assign21340_e26943 / assign21340_e26954);
        (assign21340_e26955, (((((((((((var_weff_nf_dn0 * var_c_fox) + (var_weff_nf * var_c_fox_dn0)) * var_vgvt) + (assign21340_e26899 * var_vgvt_dn0)) * var_mu) + (assign21340_e26901 * var_mu_dn0)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * var_alpha_dn0) + (6.0 * var_t0_dn0)) * var_mud_hoso) + (assign21340_e26912 * var_mud_hoso_dn0)) * var_mud_hoso) + (assign21340_e26914 * var_mud_hoso_dn0)) + ((((((4.0 * var_alpha_dn0) + (3.0 * var_t0_dn0)) * var_mud_hoso) + (assign21340_e26925 * var_mud_hoso_dn0)) * var_mu) + (assign21340_e26927 * var_mu_dn0))) + ((((((3.0 * var_alpha_dn0) + var_t0_dn0) * var_mu) + (assign21340_e26937 * var_mu_dn0)) * var_mu) + (assign21340_e26939 * var_mu_dn0))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * var_lch_dn0) * assign21340_e26949) + (assign21340_e26946 * var_alpha_dn0)) * var_mu_ave) + (assign21340_e26950 * var_mu_ave_dn0)) * var_mu_ave) + (assign21340_e26952 * var_mu_ave_dn0)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((var_weff_nf_dn2 * var_c_fox) + (var_weff_nf * var_c_fox_dn2)) * var_vgvt) + (assign21340_e26899 * var_vgvt_dn2)) * var_mu) + (assign21340_e26901 * var_mu_dn2)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * var_alpha_dn2) + (6.0 * var_t0_dn2)) * var_mud_hoso) + (assign21340_e26912 * var_mud_hoso_dn2)) * var_mud_hoso) + (assign21340_e26914 * var_mud_hoso_dn2)) + ((((((4.0 * var_alpha_dn2) + (3.0 * var_t0_dn2)) * var_mud_hoso) + (assign21340_e26925 * var_mud_hoso_dn2)) * var_mu) + (assign21340_e26927 * var_mu_dn2))) + ((((((3.0 * var_alpha_dn2) + var_t0_dn2) * var_mu) + (assign21340_e26937 * var_mu_dn2)) * var_mu) + (assign21340_e26939 * var_mu_dn2))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * var_lch_dn2) * assign21340_e26949) + (assign21340_e26946 * var_alpha_dn2)) * var_mu_ave) + (assign21340_e26950 * var_mu_ave_dn2)) * var_mu_ave) + (assign21340_e26952 * var_mu_ave_dn2)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((var_weff_nf_dn4 * var_c_fox) + (var_weff_nf * var_c_fox_dn4)) * var_vgvt) + (assign21340_e26899 * var_vgvt_dn4)) * var_mu) + (assign21340_e26901 * var_mu_dn4)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * var_alpha_dn4) + (6.0 * var_t0_dn4)) * var_mud_hoso) + (assign21340_e26912 * var_mud_hoso_dn4)) * var_mud_hoso) + (assign21340_e26914 * var_mud_hoso_dn4)) + ((((((4.0 * var_alpha_dn4) + (3.0 * var_t0_dn4)) * var_mud_hoso) + (assign21340_e26925 * var_mud_hoso_dn4)) * var_mu) + (assign21340_e26927 * var_mu_dn4))) + ((((((3.0 * var_alpha_dn4) + var_t0_dn4) * var_mu) + (assign21340_e26937 * var_mu_dn4)) * var_mu) + (assign21340_e26939 * var_mu_dn4))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * var_lch_dn4) * assign21340_e26949) + (assign21340_e26946 * var_alpha_dn4)) * var_mu_ave) + (assign21340_e26950 * var_mu_ave_dn4)) * var_mu_ave) + (assign21340_e26952 * var_mu_ave_dn4)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((var_weff_nf_dn5 * var_c_fox) + (var_weff_nf * var_c_fox_dn5)) * var_vgvt) + (assign21340_e26899 * var_vgvt_dn5)) * var_mu) + (assign21340_e26901 * var_mu_dn5)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * var_alpha_dn5) + (6.0 * var_t0_dn5)) * var_mud_hoso) + (assign21340_e26912 * var_mud_hoso_dn5)) * var_mud_hoso) + (assign21340_e26914 * var_mud_hoso_dn5)) + ((((((4.0 * var_alpha_dn5) + (3.0 * var_t0_dn5)) * var_mud_hoso) + (assign21340_e26925 * var_mud_hoso_dn5)) * var_mu) + (assign21340_e26927 * var_mu_dn5))) + ((((((3.0 * var_alpha_dn5) + var_t0_dn5) * var_mu) + (assign21340_e26937 * var_mu_dn5)) * var_mu) + (assign21340_e26939 * var_mu_dn5))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * var_lch_dn5) * assign21340_e26949) + (assign21340_e26946 * var_alpha_dn5)) * var_mu_ave) + (assign21340_e26950 * var_mu_ave_dn5)) * var_mu_ave) + (assign21340_e26952 * var_mu_ave_dn5)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((var_weff_nf_dn6 * var_c_fox) + (var_weff_nf * var_c_fox_dn6)) * var_vgvt) + (assign21340_e26899 * var_vgvt_dn6)) * var_mu) + (assign21340_e26901 * var_mu_dn6)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * var_alpha_dn6) + (6.0 * var_t0_dn6)) * var_mud_hoso) + (assign21340_e26912 * var_mud_hoso_dn6)) * var_mud_hoso) + (assign21340_e26914 * var_mud_hoso_dn6)) + ((((((4.0 * var_alpha_dn6) + (3.0 * var_t0_dn6)) * var_mud_hoso) + (assign21340_e26925 * var_mud_hoso_dn6)) * var_mu) + (assign21340_e26927 * var_mu_dn6))) + ((((((3.0 * var_alpha_dn6) + var_t0_dn6) * var_mu) + (assign21340_e26937 * var_mu_dn6)) * var_mu) + (assign21340_e26939 * var_mu_dn6))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * var_lch_dn6) * assign21340_e26949) + (assign21340_e26946 * var_alpha_dn6)) * var_mu_ave) + (assign21340_e26950 * var_mu_ave_dn6)) * var_mu_ave) + (assign21340_e26952 * var_mu_ave_dn6)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((var_weff_nf_dn8 * var_c_fox) + (var_weff_nf * var_c_fox_dn8)) * var_vgvt) + (assign21340_e26899 * var_vgvt_dn8)) * var_mu) + (assign21340_e26901 * var_mu_dn8)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * var_alpha_dn8) + (6.0 * var_t0_dn8)) * var_mud_hoso) + (assign21340_e26912 * var_mud_hoso_dn8)) * var_mud_hoso) + (assign21340_e26914 * var_mud_hoso_dn8)) + ((((((4.0 * var_alpha_dn8) + (3.0 * var_t0_dn8)) * var_mud_hoso) + (assign21340_e26925 * var_mud_hoso_dn8)) * var_mu) + (assign21340_e26927 * var_mu_dn8))) + ((((((3.0 * var_alpha_dn8) + var_t0_dn8) * var_mu) + (assign21340_e26937 * var_mu_dn8)) * var_mu) + (assign21340_e26939 * var_mu_dn8))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * var_lch_dn8) * assign21340_e26949) + (assign21340_e26946 * var_alpha_dn8)) * var_mu_ave) + (assign21340_e26950 * var_mu_ave_dn8)) * var_mu_ave) + (assign21340_e26952 * var_mu_ave_dn8)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((var_weff_nf_dn10 * var_c_fox) + (var_weff_nf * var_c_fox_dn10)) * var_vgvt) + (assign21340_e26899 * var_vgvt_dn10)) * var_mu) + (assign21340_e26901 * var_mu_dn10)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * var_alpha_dn10) + (6.0 * var_t0_dn10)) * var_mud_hoso) + (assign21340_e26912 * var_mud_hoso_dn10)) * var_mud_hoso) + (assign21340_e26914 * var_mud_hoso_dn10)) + ((((((4.0 * var_alpha_dn10) + (3.0 * var_t0_dn10)) * var_mud_hoso) + (assign21340_e26925 * var_mud_hoso_dn10)) * var_mu) + (assign21340_e26927 * var_mu_dn10))) + ((((((3.0 * var_alpha_dn10) + var_t0_dn10) * var_mu) + (assign21340_e26937 * var_mu_dn10)) * var_mu) + (assign21340_e26939 * var_mu_dn10))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * var_lch_dn10) * assign21340_e26949) + (assign21340_e26946 * var_alpha_dn10)) * var_mu_ave) + (assign21340_e26950 * var_mu_ave_dn10)) * var_mu_ave) + (assign21340_e26952 * var_mu_ave_dn10)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((var_weff_nf_dn11 * var_c_fox) + (var_weff_nf * var_c_fox_dn11)) * var_vgvt) + (assign21340_e26899 * var_vgvt_dn11)) * var_mu) + (assign21340_e26901 * var_mu_dn11)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * var_alpha_dn11) + (6.0 * var_t0_dn11)) * var_mud_hoso) + (assign21340_e26912 * var_mud_hoso_dn11)) * var_mud_hoso) + (assign21340_e26914 * var_mud_hoso_dn11)) + ((((((4.0 * var_alpha_dn11) + (3.0 * var_t0_dn11)) * var_mud_hoso) + (assign21340_e26925 * var_mud_hoso_dn11)) * var_mu) + (assign21340_e26927 * var_mu_dn11))) + ((((((3.0 * var_alpha_dn11) + var_t0_dn11) * var_mu) + (assign21340_e26937 * var_mu_dn11)) * var_mu) + (assign21340_e26939 * var_mu_dn11))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * var_lch_dn11) * assign21340_e26949) + (assign21340_e26946 * var_alpha_dn11)) * var_mu_ave) + (assign21340_e26950 * var_mu_ave_dn11)) * var_mu_ave) + (assign21340_e26952 * var_mu_ave_dn11)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((var_weff_nf_dn12 * var_c_fox) + (var_weff_nf * var_c_fox_dn12)) * var_vgvt) + (assign21340_e26899 * var_vgvt_dn12)) * var_mu) + (assign21340_e26901 * var_mu_dn12)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * var_alpha_dn12) + (6.0 * var_t0_dn12)) * var_mud_hoso) + (assign21340_e26912 * var_mud_hoso_dn12)) * var_mud_hoso) + (assign21340_e26914 * var_mud_hoso_dn12)) + ((((((4.0 * var_alpha_dn12) + (3.0 * var_t0_dn12)) * var_mud_hoso) + (assign21340_e26925 * var_mud_hoso_dn12)) * var_mu) + (assign21340_e26927 * var_mu_dn12))) + ((((((3.0 * var_alpha_dn12) + var_t0_dn12) * var_mu) + (assign21340_e26937 * var_mu_dn12)) * var_mu) + (assign21340_e26939 * var_mu_dn12))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * var_lch_dn12) * assign21340_e26949) + (assign21340_e26946 * var_alpha_dn12)) * var_mu_ave) + (assign21340_e26950 * var_mu_ave_dn12)) * var_mu_ave) + (assign21340_e26952 * var_mu_ave_dn12)))) / (assign21340_e26954 * assign21340_e26954)),)
    } else {
        (var_nthrml, var_nthrml_dn0, var_nthrml_dn2, var_nthrml_dn4, var_nthrml_dn5, var_nthrml_dn6, var_nthrml_dn8, var_nthrml_dn10, var_nthrml_dn11, var_nthrml_dn12,)
    }
};
        var_nthrml = assign21340_e26957;
        var_nthrml_dn0 = assign21340_e26957_d_n0;
        var_nthrml_dn2 = assign21340_e26957_d_n2;
        var_nthrml_dn4 = assign21340_e26957_d_n4;
        var_nthrml_dn5 = assign21340_e26957_d_n5;
        var_nthrml_dn6 = assign21340_e26957_d_n6;
        var_nthrml_dn8 = assign21340_e26957_d_n8;
        var_nthrml_dn10 = assign21340_e26957_d_n10;
        var_nthrml_dn11 = assign21340_e26957_d_n11;
        var_nthrml_dn12 = assign21340_e26957_d_n12;

        let (assign21350_e26962, assign21350_e26962_d_n0, assign21350_e26962_d_n2, assign21350_e26962_d_n4, assign21350_e26962_d_n5, assign21350_e26962_d_n6, assign21350_e26962_d_n8, assign21350_e26962_d_n10, assign21350_e26962_d_n11, assign21350_e26962_d_n12,) = {
    if (var_guard368 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nthrml, var_nthrml_dn0, var_nthrml_dn2, var_nthrml_dn4, var_nthrml_dn5, var_nthrml_dn6, var_nthrml_dn8, var_nthrml_dn10, var_nthrml_dn11, var_nthrml_dn12,)
    }
};
        var_nthrml = assign21350_e26962;
        var_nthrml_dn0 = assign21350_e26962_d_n0;
        var_nthrml_dn2 = assign21350_e26962_d_n2;
        var_nthrml_dn4 = assign21350_e26962_d_n4;
        var_nthrml_dn5 = assign21350_e26962_d_n5;
        var_nthrml_dn6 = assign21350_e26962_d_n6;
        var_nthrml_dn8 = assign21350_e26962_d_n8;
        var_nthrml_dn10 = assign21350_e26962_d_n10;
        var_nthrml_dn11 = assign21350_e26962_d_n11;
        var_nthrml_dn12 = assign21350_e26962_d_n12;

        let assign21360_e26976: f64 = if ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (var_flg_ign == 1.0)) && (var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        var_guard371 = assign21360_e26976;

        let (assign21370_e26981, assign21370_e26981_d_n0, assign21370_e26981_d_n2, assign21370_e26981_d_n4, assign21370_e26981_d_n5, assign21370_e26981_d_n6, assign21370_e26981_d_n8, assign21370_e26981_d_n10, assign21370_e26981_d_n11, assign21370_e26981_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21370_e26979: f64 = (var_kusail).sqrt();
        (assign21370_e26979, (var_kusail_dn0 / (2.0 * assign21370_e26979)), (var_kusail_dn2 / (2.0 * assign21370_e26979)), (var_kusail_dn4 / (2.0 * assign21370_e26979)), (var_kusail_dn5 / (2.0 * assign21370_e26979)), (var_kusail_dn6 / (2.0 * assign21370_e26979)), (var_kusail_dn8 / (2.0 * assign21370_e26979)), (var_kusail_dn10 / (2.0 * assign21370_e26979)), (var_kusail_dn11 / (2.0 * assign21370_e26979)), (var_kusail_dn12 / (2.0 * assign21370_e26979)),)
    } else {
        (var_sqrtkusail, var_sqrtkusail_dn0, var_sqrtkusail_dn2, var_sqrtkusail_dn4, var_sqrtkusail_dn5, var_sqrtkusail_dn6, var_sqrtkusail_dn8, var_sqrtkusail_dn10, var_sqrtkusail_dn11, var_sqrtkusail_dn12,)
    }
};
        var_sqrtkusail = assign21370_e26981;
        var_sqrtkusail_dn0 = assign21370_e26981_d_n0;
        var_sqrtkusail_dn2 = assign21370_e26981_d_n2;
        var_sqrtkusail_dn4 = assign21370_e26981_d_n4;
        var_sqrtkusail_dn5 = assign21370_e26981_d_n5;
        var_sqrtkusail_dn6 = assign21370_e26981_d_n6;
        var_sqrtkusail_dn8 = assign21370_e26981_d_n8;
        var_sqrtkusail_dn10 = assign21370_e26981_d_n10;
        var_sqrtkusail_dn11 = assign21370_e26981_d_n11;
        var_sqrtkusail_dn12 = assign21370_e26981_d_n12;

        let (assign21380_e26987, assign21380_e26987_d_n0, assign21380_e26987_d_n2, assign21380_e26987_d_n4, assign21380_e26987_d_n5, assign21380_e26987_d_n6, assign21380_e26987_d_n8, assign21380_e26987_d_n10, assign21380_e26987_d_n11, assign21380_e26987_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21380_e26985: f64 = (var_vgvt + var_sqrtkusail);
        (assign21380_e26985, (var_vgvt_dn0 + var_sqrtkusail_dn0), (var_vgvt_dn2 + var_sqrtkusail_dn2), (var_vgvt_dn4 + var_sqrtkusail_dn4), (var_vgvt_dn5 + var_sqrtkusail_dn5), (var_vgvt_dn6 + var_sqrtkusail_dn6), (var_vgvt_dn8 + var_sqrtkusail_dn8), (var_vgvt_dn10 + var_sqrtkusail_dn10), (var_vgvt_dn11 + var_sqrtkusail_dn11), (var_vgvt_dn12 + var_sqrtkusail_dn12),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign21380_e26987;
        var_t2_dn0 = assign21380_e26987_d_n0;
        var_t2_dn2 = assign21380_e26987_d_n2;
        var_t2_dn4 = assign21380_e26987_d_n4;
        var_t2_dn5 = assign21380_e26987_d_n5;
        var_t2_dn6 = assign21380_e26987_d_n6;
        var_t2_dn8 = assign21380_e26987_d_n8;
        var_t2_dn10 = assign21380_e26987_d_n10;
        var_t2_dn11 = assign21380_e26987_d_n11;
        var_t2_dn12 = assign21380_e26987_d_n12;

        let (assign21390_e26993, assign21390_e26993_d_n0, assign21390_e26993_d_n2, assign21390_e26993_d_n4, assign21390_e26993_d_n5, assign21390_e26993_d_n6, assign21390_e26993_d_n8, assign21390_e26993_d_n10, assign21390_e26993_d_n11, assign21390_e26993_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21390_e26991: f64 = (var_kusai00 * var_kusai00);
        (assign21390_e26991, ((var_kusai00_dn0 * var_kusai00) + (var_kusai00 * var_kusai00_dn0)), ((var_kusai00_dn2 * var_kusai00) + (var_kusai00 * var_kusai00_dn2)), ((var_kusai00_dn4 * var_kusai00) + (var_kusai00 * var_kusai00_dn4)), ((var_kusai00_dn5 * var_kusai00) + (var_kusai00 * var_kusai00_dn5)), ((var_kusai00_dn6 * var_kusai00) + (var_kusai00 * var_kusai00_dn6)), ((var_kusai00_dn8 * var_kusai00) + (var_kusai00 * var_kusai00_dn8)), ((var_kusai00_dn10 * var_kusai00) + (var_kusai00 * var_kusai00_dn10)), ((var_kusai00_dn11 * var_kusai00) + (var_kusai00 * var_kusai00_dn11)), ((var_kusai00_dn12 * var_kusai00) + (var_kusai00 * var_kusai00_dn12)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign21390_e26993;
        var_t3_dn0 = assign21390_e26993_d_n0;
        var_t3_dn2 = assign21390_e26993_d_n2;
        var_t3_dn4 = assign21390_e26993_d_n4;
        var_t3_dn5 = assign21390_e26993_d_n5;
        var_t3_dn6 = assign21390_e26993_d_n6;
        var_t3_dn8 = assign21390_e26993_d_n8;
        var_t3_dn10 = assign21390_e26993_d_n10;
        var_t3_dn11 = assign21390_e26993_d_n11;
        var_t3_dn12 = assign21390_e26993_d_n12;

        let (assign21400_e26999, assign21400_e26999_d_n0, assign21400_e26999_d_n2, assign21400_e26999_d_n4, assign21400_e26999_d_n5, assign21400_e26999_d_n6, assign21400_e26999_d_n8, assign21400_e26999_d_n10, assign21400_e26999_d_n11, assign21400_e26999_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21400_e26997: f64 = (var_kusail * var_kusail);
        (assign21400_e26997, ((var_kusail_dn0 * var_kusail) + (var_kusail * var_kusail_dn0)), ((var_kusail_dn2 * var_kusail) + (var_kusail * var_kusail_dn2)), ((var_kusail_dn4 * var_kusail) + (var_kusail * var_kusail_dn4)), ((var_kusail_dn5 * var_kusail) + (var_kusail * var_kusail_dn5)), ((var_kusail_dn6 * var_kusail) + (var_kusail * var_kusail_dn6)), ((var_kusail_dn8 * var_kusail) + (var_kusail * var_kusail_dn8)), ((var_kusail_dn10 * var_kusail) + (var_kusail * var_kusail_dn10)), ((var_kusail_dn11 * var_kusail) + (var_kusail * var_kusail_dn11)), ((var_kusail_dn12 * var_kusail) + (var_kusail * var_kusail_dn12)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
        var_t4 = assign21400_e26999;
        var_t4_dn0 = assign21400_e26999_d_n0;
        var_t4_dn2 = assign21400_e26999_d_n2;
        var_t4_dn4 = assign21400_e26999_d_n4;
        var_t4_dn5 = assign21400_e26999_d_n5;
        var_t4_dn6 = assign21400_e26999_d_n6;
        var_t4_dn8 = assign21400_e26999_d_n8;
        var_t4_dn10 = assign21400_e26999_d_n10;
        var_t4_dn11 = assign21400_e26999_d_n11;
        var_t4_dn12 = assign21400_e26999_d_n12;

        let (assign21410_e27007, assign21410_e27007_d_n0, assign21410_e27007_d_n2, assign21410_e27007_d_n4, assign21410_e27007_d_n5, assign21410_e27007_d_n6, assign21410_e27007_d_n8, assign21410_e27007_d_n10, assign21410_e27007_d_n11, assign21410_e27007_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21410_e27003: f64 = (42.0 * var_kusai00);
        let assign21410_e27005: f64 = (assign21410_e27003 * var_kusail);
        (assign21410_e27005, (((42.0 * var_kusai00_dn0) * var_kusail) + (assign21410_e27003 * var_kusail_dn0)), (((42.0 * var_kusai00_dn2) * var_kusail) + (assign21410_e27003 * var_kusail_dn2)), (((42.0 * var_kusai00_dn4) * var_kusail) + (assign21410_e27003 * var_kusail_dn4)), (((42.0 * var_kusai00_dn5) * var_kusail) + (assign21410_e27003 * var_kusail_dn5)), (((42.0 * var_kusai00_dn6) * var_kusail) + (assign21410_e27003 * var_kusail_dn6)), (((42.0 * var_kusai00_dn8) * var_kusail) + (assign21410_e27003 * var_kusail_dn8)), (((42.0 * var_kusai00_dn10) * var_kusail) + (assign21410_e27003 * var_kusail_dn10)), (((42.0 * var_kusai00_dn11) * var_kusail) + (assign21410_e27003 * var_kusail_dn11)), (((42.0 * var_kusai00_dn12) * var_kusail) + (assign21410_e27003 * var_kusail_dn12)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign21410_e27007;
        var_t5_dn0 = assign21410_e27007_d_n0;
        var_t5_dn2 = assign21410_e27007_d_n2;
        var_t5_dn4 = assign21410_e27007_d_n4;
        var_t5_dn5 = assign21410_e27007_d_n5;
        var_t5_dn6 = assign21410_e27007_d_n6;
        var_t5_dn8 = assign21410_e27007_d_n8;
        var_t5_dn10 = assign21410_e27007_d_n10;
        var_t5_dn11 = assign21410_e27007_d_n11;
        var_t5_dn12 = assign21410_e27007_d_n12;

        let (assign21420_e27017, assign21420_e27017_d_n0, assign21420_e27017_d_n2, assign21420_e27017_d_n4, assign21420_e27017_d_n5, assign21420_e27017_d_n6, assign21420_e27017_d_n8, assign21420_e27017_d_n10, assign21420_e27017_d_n11, assign21420_e27017_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21420_e27013: f64 = (var_t3 + var_t4);
        let assign21420_e27014: f64 = (4.0 * assign21420_e27013);
        let assign21420_e27015: f64 = (var_t5 + assign21420_e27014);
        (assign21420_e27015, (var_t5_dn0 + (4.0 * (var_t3_dn0 + var_t4_dn0))), (var_t5_dn2 + (4.0 * (var_t3_dn2 + var_t4_dn2))), (var_t5_dn4 + (4.0 * (var_t3_dn4 + var_t4_dn4))), (var_t5_dn5 + (4.0 * (var_t3_dn5 + var_t4_dn5))), (var_t5_dn6 + (4.0 * (var_t3_dn6 + var_t4_dn6))), (var_t5_dn8 + (4.0 * (var_t3_dn8 + var_t4_dn8))), (var_t5_dn10 + (4.0 * (var_t3_dn10 + var_t4_dn10))), (var_t5_dn11 + (4.0 * (var_t3_dn11 + var_t4_dn11))), (var_t5_dn12 + (4.0 * (var_t3_dn12 + var_t4_dn12))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign21420_e27017;
        var_t5_dn0 = assign21420_e27017_d_n0;
        var_t5_dn2 = assign21420_e27017_d_n2;
        var_t5_dn4 = assign21420_e27017_d_n4;
        var_t5_dn5 = assign21420_e27017_d_n5;
        var_t5_dn6 = assign21420_e27017_d_n6;
        var_t5_dn8 = assign21420_e27017_d_n8;
        var_t5_dn10 = assign21420_e27017_d_n10;
        var_t5_dn11 = assign21420_e27017_d_n11;
        var_t5_dn12 = assign21420_e27017_d_n12;

        let (assign21430_e27031, assign21430_e27031_d_n0, assign21430_e27031_d_n2, assign21430_e27031_d_n4, assign21430_e27031_d_n5, assign21430_e27031_d_n6, assign21430_e27031_d_n8, assign21430_e27031_d_n10, assign21430_e27031_d_n11, assign21430_e27031_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21430_e27022: f64 = (20.0 * var_sqrtkusail);
        let assign21430_e27024: f64 = (assign21430_e27022 * var_vgvt);
        let assign21430_e27027: f64 = (var_kusai00 + var_kusail);
        let assign21430_e27028: f64 = (assign21430_e27024 * assign21430_e27027);
        let assign21430_e27029: f64 = (var_t5 + assign21430_e27028);
        (assign21430_e27029, (var_t5_dn0 + (((((20.0 * var_sqrtkusail_dn0) * var_vgvt) + (assign21430_e27022 * var_vgvt_dn0)) * assign21430_e27027) + (assign21430_e27024 * (var_kusai00_dn0 + var_kusail_dn0)))), (var_t5_dn2 + (((((20.0 * var_sqrtkusail_dn2) * var_vgvt) + (assign21430_e27022 * var_vgvt_dn2)) * assign21430_e27027) + (assign21430_e27024 * (var_kusai00_dn2 + var_kusail_dn2)))), (var_t5_dn4 + (((((20.0 * var_sqrtkusail_dn4) * var_vgvt) + (assign21430_e27022 * var_vgvt_dn4)) * assign21430_e27027) + (assign21430_e27024 * (var_kusai00_dn4 + var_kusail_dn4)))), (var_t5_dn5 + (((((20.0 * var_sqrtkusail_dn5) * var_vgvt) + (assign21430_e27022 * var_vgvt_dn5)) * assign21430_e27027) + (assign21430_e27024 * (var_kusai00_dn5 + var_kusail_dn5)))), (var_t5_dn6 + (((((20.0 * var_sqrtkusail_dn6) * var_vgvt) + (assign21430_e27022 * var_vgvt_dn6)) * assign21430_e27027) + (assign21430_e27024 * (var_kusai00_dn6 + var_kusail_dn6)))), (var_t5_dn8 + (((((20.0 * var_sqrtkusail_dn8) * var_vgvt) + (assign21430_e27022 * var_vgvt_dn8)) * assign21430_e27027) + (assign21430_e27024 * (var_kusai00_dn8 + var_kusail_dn8)))), (var_t5_dn10 + (((((20.0 * var_sqrtkusail_dn10) * var_vgvt) + (assign21430_e27022 * var_vgvt_dn10)) * assign21430_e27027) + (assign21430_e27024 * (var_kusai00_dn10 + var_kusail_dn10)))), (var_t5_dn11 + (((((20.0 * var_sqrtkusail_dn11) * var_vgvt) + (assign21430_e27022 * var_vgvt_dn11)) * assign21430_e27027) + (assign21430_e27024 * (var_kusai00_dn11 + var_kusail_dn11)))), (var_t5_dn12 + (((((20.0 * var_sqrtkusail_dn12) * var_vgvt) + (assign21430_e27022 * var_vgvt_dn12)) * assign21430_e27027) + (assign21430_e27024 * (var_kusai00_dn12 + var_kusail_dn12)))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign21430_e27031;
        var_t5_dn0 = assign21430_e27031_d_n0;
        var_t5_dn2 = assign21430_e27031_d_n2;
        var_t5_dn4 = assign21430_e27031_d_n4;
        var_t5_dn5 = assign21430_e27031_d_n5;
        var_t5_dn6 = assign21430_e27031_d_n6;
        var_t5_dn8 = assign21430_e27031_d_n8;
        var_t5_dn10 = assign21430_e27031_d_n10;
        var_t5_dn11 = assign21430_e27031_d_n11;
        var_t5_dn12 = assign21430_e27031_d_n12;

        let (assign21440_e27037, assign21440_e27037_d_n0, assign21440_e27037_d_n2, assign21440_e27037_d_n4, assign21440_e27037_d_n5, assign21440_e27037_d_n6, assign21440_e27037_d_n8, assign21440_e27037_d_n10, assign21440_e27037_d_n11, assign21440_e27037_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21440_e27035: f64 = (var_t2 * var_t2);
        (assign21440_e27035, ((var_t2_dn0 * var_t2) + (var_t2 * var_t2_dn0)), ((var_t2_dn2 * var_t2) + (var_t2 * var_t2_dn2)), ((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4)), ((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5)), ((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)), ((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8)), ((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10)), ((var_t2_dn11 * var_t2) + (var_t2 * var_t2_dn11)), ((var_t2_dn12 * var_t2) + (var_t2 * var_t2_dn12)),)
    } else {
        (var_t10, var_t10_dn0, var_t10_dn2, var_t10_dn4, var_t10_dn5, var_t10_dn6, var_t10_dn8, var_t10_dn10, var_t10_dn11, var_t10_dn12,)
    }
};
        var_t10 = assign21440_e27037;
        var_t10_dn0 = assign21440_e27037_d_n0;
        var_t10_dn2 = assign21440_e27037_d_n2;
        var_t10_dn4 = assign21440_e27037_d_n4;
        var_t10_dn5 = assign21440_e27037_d_n5;
        var_t10_dn6 = assign21440_e27037_d_n6;
        var_t10_dn8 = assign21440_e27037_d_n8;
        var_t10_dn10 = assign21440_e27037_d_n10;
        var_t10_dn11 = assign21440_e27037_d_n11;
        var_t10_dn12 = assign21440_e27037_d_n12;

        let (assign21450_e27047, assign21450_e27047_d_n0, assign21450_e27047_d_n2, assign21450_e27047_d_n4, assign21450_e27047_d_n5, assign21450_e27047_d_n6, assign21450_e27047_d_n8, assign21450_e27047_d_n10, assign21450_e27047_d_n11, assign21450_e27047_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21450_e27042: f64 = (var_t10 * var_t10);
        let assign21450_e27044: f64 = (assign21450_e27042 * var_t2);
        let assign21450_e27045: f64 = (var_t5 / assign21450_e27044);
        (assign21450_e27045, (((var_t5_dn0 * assign21450_e27044) - (var_t5 * ((((var_t10_dn0 * var_t10) + (var_t10 * var_t10_dn0)) * var_t2) + (assign21450_e27042 * var_t2_dn0)))) / (assign21450_e27044 * assign21450_e27044)), (((var_t5_dn2 * assign21450_e27044) - (var_t5 * ((((var_t10_dn2 * var_t10) + (var_t10 * var_t10_dn2)) * var_t2) + (assign21450_e27042 * var_t2_dn2)))) / (assign21450_e27044 * assign21450_e27044)), (((var_t5_dn4 * assign21450_e27044) - (var_t5 * ((((var_t10_dn4 * var_t10) + (var_t10 * var_t10_dn4)) * var_t2) + (assign21450_e27042 * var_t2_dn4)))) / (assign21450_e27044 * assign21450_e27044)), (((var_t5_dn5 * assign21450_e27044) - (var_t5 * ((((var_t10_dn5 * var_t10) + (var_t10 * var_t10_dn5)) * var_t2) + (assign21450_e27042 * var_t2_dn5)))) / (assign21450_e27044 * assign21450_e27044)), (((var_t5_dn6 * assign21450_e27044) - (var_t5 * ((((var_t10_dn6 * var_t10) + (var_t10 * var_t10_dn6)) * var_t2) + (assign21450_e27042 * var_t2_dn6)))) / (assign21450_e27044 * assign21450_e27044)), (((var_t5_dn8 * assign21450_e27044) - (var_t5 * ((((var_t10_dn8 * var_t10) + (var_t10 * var_t10_dn8)) * var_t2) + (assign21450_e27042 * var_t2_dn8)))) / (assign21450_e27044 * assign21450_e27044)), (((var_t5_dn10 * assign21450_e27044) - (var_t5 * ((((var_t10_dn10 * var_t10) + (var_t10 * var_t10_dn10)) * var_t2) + (assign21450_e27042 * var_t2_dn10)))) / (assign21450_e27044 * assign21450_e27044)), (((var_t5_dn11 * assign21450_e27044) - (var_t5 * ((((var_t10_dn11 * var_t10) + (var_t10 * var_t10_dn11)) * var_t2) + (assign21450_e27042 * var_t2_dn11)))) / (assign21450_e27044 * assign21450_e27044)), (((var_t5_dn12 * assign21450_e27044) - (var_t5 * ((((var_t10_dn12 * var_t10) + (var_t10 * var_t10_dn12)) * var_t2) + (assign21450_e27042 * var_t2_dn12)))) / (assign21450_e27044 * assign21450_e27044)),)
    } else {
        (var_kusai_ig, var_kusai_ig_dn0, var_kusai_ig_dn2, var_kusai_ig_dn4, var_kusai_ig_dn5, var_kusai_ig_dn6, var_kusai_ig_dn8, var_kusai_ig_dn10, var_kusai_ig_dn11, var_kusai_ig_dn12,)
    }
};
        var_kusai_ig = assign21450_e27047;
        var_kusai_ig_dn0 = assign21450_e27047_d_n0;
        var_kusai_ig_dn2 = assign21450_e27047_d_n2;
        var_kusai_ig_dn4 = assign21450_e27047_d_n4;
        var_kusai_ig_dn5 = assign21450_e27047_d_n5;
        var_kusai_ig_dn6 = assign21450_e27047_d_n6;
        var_kusai_ig_dn8 = assign21450_e27047_d_n8;
        var_kusai_ig_dn10 = assign21450_e27047_d_n10;
        var_kusai_ig_dn11 = assign21450_e27047_d_n11;
        var_kusai_ig_dn12 = assign21450_e27047_d_n12;

        let (assign21460_e27057, assign21460_e27057_d_n0, assign21460_e27057_d_n2, assign21460_e27057_d_n4, assign21460_e27057_d_n5, assign21460_e27057_d_n6, assign21460_e27057_d_n8, assign21460_e27057_d_n10, assign21460_e27057_d_n11, assign21460_e27057_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21460_e27051: f64 = (var_weff_nf / var_lch);
        let assign21460_e27053: f64 = (assign21460_e27051 * var_mu);
        let assign21460_e27055: f64 = (assign21460_e27053 * var_c_fox);
        (assign21460_e27055, (((((((var_weff_nf_dn0 * var_lch) - (var_weff_nf * var_lch_dn0)) / (var_lch * var_lch)) * var_mu) + (assign21460_e27051 * var_mu_dn0)) * var_c_fox) + (assign21460_e27053 * var_c_fox_dn0)), (((((((var_weff_nf_dn2 * var_lch) - (var_weff_nf * var_lch_dn2)) / (var_lch * var_lch)) * var_mu) + (assign21460_e27051 * var_mu_dn2)) * var_c_fox) + (assign21460_e27053 * var_c_fox_dn2)), (((((((var_weff_nf_dn4 * var_lch) - (var_weff_nf * var_lch_dn4)) / (var_lch * var_lch)) * var_mu) + (assign21460_e27051 * var_mu_dn4)) * var_c_fox) + (assign21460_e27053 * var_c_fox_dn4)), (((((((var_weff_nf_dn5 * var_lch) - (var_weff_nf * var_lch_dn5)) / (var_lch * var_lch)) * var_mu) + (assign21460_e27051 * var_mu_dn5)) * var_c_fox) + (assign21460_e27053 * var_c_fox_dn5)), (((((((var_weff_nf_dn6 * var_lch) - (var_weff_nf * var_lch_dn6)) / (var_lch * var_lch)) * var_mu) + (assign21460_e27051 * var_mu_dn6)) * var_c_fox) + (assign21460_e27053 * var_c_fox_dn6)), (((((((var_weff_nf_dn8 * var_lch) - (var_weff_nf * var_lch_dn8)) / (var_lch * var_lch)) * var_mu) + (assign21460_e27051 * var_mu_dn8)) * var_c_fox) + (assign21460_e27053 * var_c_fox_dn8)), (((((((var_weff_nf_dn10 * var_lch) - (var_weff_nf * var_lch_dn10)) / (var_lch * var_lch)) * var_mu) + (assign21460_e27051 * var_mu_dn10)) * var_c_fox) + (assign21460_e27053 * var_c_fox_dn10)), (((((((var_weff_nf_dn11 * var_lch) - (var_weff_nf * var_lch_dn11)) / (var_lch * var_lch)) * var_mu) + (assign21460_e27051 * var_mu_dn11)) * var_c_fox) + (assign21460_e27053 * var_c_fox_dn11)), (((((((var_weff_nf_dn12 * var_lch) - (var_weff_nf * var_lch_dn12)) / (var_lch * var_lch)) * var_mu) + (assign21460_e27051 * var_mu_dn12)) * var_c_fox) + (assign21460_e27053 * var_c_fox_dn12)),)
    } else {
        (var_gds0_ign, var_gds0_ign_dn0, var_gds0_ign_dn2, var_gds0_ign_dn4, var_gds0_ign_dn5, var_gds0_ign_dn6, var_gds0_ign_dn8, var_gds0_ign_dn10, var_gds0_ign_dn11, var_gds0_ign_dn12,)
    }
};
        var_gds0_ign = assign21460_e27057;
        var_gds0_ign_dn0 = assign21460_e27057_d_n0;
        var_gds0_ign_dn2 = assign21460_e27057_d_n2;
        var_gds0_ign_dn4 = assign21460_e27057_d_n4;
        var_gds0_ign_dn5 = assign21460_e27057_d_n5;
        var_gds0_ign_dn6 = assign21460_e27057_d_n6;
        var_gds0_ign_dn8 = assign21460_e27057_d_n8;
        var_gds0_ign_dn10 = assign21460_e27057_d_n10;
        var_gds0_ign_dn11 = assign21460_e27057_d_n11;
        var_gds0_ign_dn12 = assign21460_e27057_d_n12;

        let (assign21470_e27063, assign21470_e27063_d_n0, assign21470_e27063_d_n2, assign21470_e27063_d_n4, assign21470_e27063_d_n5, assign21470_e27063_d_n6, assign21470_e27063_d_n8, assign21470_e27063_d_n10, assign21470_e27063_d_n11, assign21470_e27063_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21470_e27061: f64 = (var_gds0_ign * var_vgvt);
        (assign21470_e27061, ((var_gds0_ign_dn0 * var_vgvt) + (var_gds0_ign * var_vgvt_dn0)), ((var_gds0_ign_dn2 * var_vgvt) + (var_gds0_ign * var_vgvt_dn2)), ((var_gds0_ign_dn4 * var_vgvt) + (var_gds0_ign * var_vgvt_dn4)), ((var_gds0_ign_dn5 * var_vgvt) + (var_gds0_ign * var_vgvt_dn5)), ((var_gds0_ign_dn6 * var_vgvt) + (var_gds0_ign * var_vgvt_dn6)), ((var_gds0_ign_dn8 * var_vgvt) + (var_gds0_ign * var_vgvt_dn8)), ((var_gds0_ign_dn10 * var_vgvt) + (var_gds0_ign * var_vgvt_dn10)), ((var_gds0_ign_dn11 * var_vgvt) + (var_gds0_ign * var_vgvt_dn11)), ((var_gds0_ign_dn12 * var_vgvt) + (var_gds0_ign * var_vgvt_dn12)),)
    } else {
        (var_gds0_h2, var_gds0_h2_dn0, var_gds0_h2_dn2, var_gds0_h2_dn4, var_gds0_h2_dn5, var_gds0_h2_dn6, var_gds0_h2_dn8, var_gds0_h2_dn10, var_gds0_h2_dn11, var_gds0_h2_dn12,)
    }
};
        var_gds0_h2 = assign21470_e27063;
        var_gds0_h2_dn0 = assign21470_e27063_d_n0;
        var_gds0_h2_dn2 = assign21470_e27063_d_n2;
        var_gds0_h2_dn4 = assign21470_e27063_d_n4;
        var_gds0_h2_dn5 = assign21470_e27063_d_n5;
        var_gds0_h2_dn6 = assign21470_e27063_d_n6;
        var_gds0_h2_dn8 = assign21470_e27063_d_n8;
        var_gds0_h2_dn10 = assign21470_e27063_d_n10;
        var_gds0_h2_dn11 = assign21470_e27063_d_n11;
        var_gds0_h2_dn12 = assign21470_e27063_d_n12;

        let (assign21480_e27069, assign21480_e27069_d_n0, assign21480_e27069_d_n2, assign21480_e27069_d_n4, assign21480_e27069_d_n5, assign21480_e27069_d_n6, assign21480_e27069_d_n8, assign21480_e27069_d_n10, assign21480_e27069_d_n11, assign21480_e27069_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21480_e27067: f64 = (var_nthrml / var_gds0_h2);
        (assign21480_e27067, (((var_nthrml_dn0 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn0)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn2 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn2)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn4 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn4)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn5 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn5)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn6 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn6)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn8 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn8)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn10 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn10)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn11 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn11)) / (var_gds0_h2 * var_gds0_h2)), (((var_nthrml_dn12 * var_gds0_h2) - (var_nthrml * var_gds0_h2_dn12)) / (var_gds0_h2 * var_gds0_h2)),)
    } else {
        (var_gamma, var_gamma_dn0, var_gamma_dn2, var_gamma_dn4, var_gamma_dn5, var_gamma_dn6, var_gamma_dn8, var_gamma_dn10, var_gamma_dn11, var_gamma_dn12,)
    }
};
        var_gamma = assign21480_e27069;
        var_gamma_dn0 = assign21480_e27069_d_n0;
        var_gamma_dn2 = assign21480_e27069_d_n2;
        var_gamma_dn4 = assign21480_e27069_d_n4;
        var_gamma_dn5 = assign21480_e27069_d_n5;
        var_gamma_dn6 = assign21480_e27069_d_n6;
        var_gamma_dn8 = assign21480_e27069_d_n8;
        var_gamma_dn10 = assign21480_e27069_d_n10;
        var_gamma_dn11 = assign21480_e27069_d_n11;
        var_gamma_dn12 = assign21480_e27069_d_n12;

        let (assign21490_e27081, assign21490_e27081_d_n0, assign21490_e27081_d_n2, assign21490_e27081_d_n4, assign21490_e27081_d_n5, assign21490_e27081_d_n6, assign21490_e27081_d_n8, assign21490_e27081_d_n10, assign21490_e27081_d_n11, assign21490_e27081_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21490_e27074: f64 = (4.0 * var_vgvt);
        let assign21490_e27076: f64 = (assign21490_e27074 * var_sqrtkusail);
        let assign21490_e27077: f64 = (var_kusai00 + assign21490_e27076);
        let assign21490_e27079: f64 = (assign21490_e27077 + var_kusail);
        (assign21490_e27079, ((var_kusai00_dn0 + (((4.0 * var_vgvt_dn0) * var_sqrtkusail) + (assign21490_e27074 * var_sqrtkusail_dn0))) + var_kusail_dn0), ((var_kusai00_dn2 + (((4.0 * var_vgvt_dn2) * var_sqrtkusail) + (assign21490_e27074 * var_sqrtkusail_dn2))) + var_kusail_dn2), ((var_kusai00_dn4 + (((4.0 * var_vgvt_dn4) * var_sqrtkusail) + (assign21490_e27074 * var_sqrtkusail_dn4))) + var_kusail_dn4), ((var_kusai00_dn5 + (((4.0 * var_vgvt_dn5) * var_sqrtkusail) + (assign21490_e27074 * var_sqrtkusail_dn5))) + var_kusail_dn5), ((var_kusai00_dn6 + (((4.0 * var_vgvt_dn6) * var_sqrtkusail) + (assign21490_e27074 * var_sqrtkusail_dn6))) + var_kusail_dn6), ((var_kusai00_dn8 + (((4.0 * var_vgvt_dn8) * var_sqrtkusail) + (assign21490_e27074 * var_sqrtkusail_dn8))) + var_kusail_dn8), ((var_kusai00_dn10 + (((4.0 * var_vgvt_dn10) * var_sqrtkusail) + (assign21490_e27074 * var_sqrtkusail_dn10))) + var_kusail_dn10), ((var_kusai00_dn11 + (((4.0 * var_vgvt_dn11) * var_sqrtkusail) + (assign21490_e27074 * var_sqrtkusail_dn11))) + var_kusail_dn11), ((var_kusai00_dn12 + (((4.0 * var_vgvt_dn12) * var_sqrtkusail) + (assign21490_e27074 * var_sqrtkusail_dn12))) + var_kusail_dn12),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
        var_t7 = assign21490_e27081;
        var_t7_dn0 = assign21490_e27081_d_n0;
        var_t7_dn2 = assign21490_e27081_d_n2;
        var_t7_dn4 = assign21490_e27081_d_n4;
        var_t7_dn5 = assign21490_e27081_d_n5;
        var_t7_dn6 = assign21490_e27081_d_n6;
        var_t7_dn8 = assign21490_e27081_d_n8;
        var_t7_dn10 = assign21490_e27081_d_n10;
        var_t7_dn11 = assign21490_e27081_d_n11;
        var_t7_dn12 = assign21490_e27081_d_n12;

        let (assign21500_e27102, assign21500_e27102_d_n0, assign21500_e27102_d_n2, assign21500_e27102_d_n4, assign21500_e27102_d_n5, assign21500_e27102_d_n6, assign21500_e27102_d_n8, assign21500_e27102_d_n10, assign21500_e27102_d_n11, assign21500_e27102_d_n12,) = {
    if (var_guard371 != 0.0) {
        let assign21500_e27085: f64 = (3.872983346207417 * var_kusai00l);
        let assign21500_e27087: f64 = (assign21500_e27085 * var_t7);
        let assign21500_e27090: f64 = (6.0 * var_t2);
        let assign21500_e27093: f64 = (var_gamma * var_t2);
        let assign21500_e27095: f64 = (assign21500_e27093 * var_vgvt);
        let assign21500_e27097: f64 = (assign21500_e27095 * var_t5);
        let assign21500_e27098: f64 = (assign21500_e27097).sqrt();
        let assign21500_e27099: f64 = (assign21500_e27090 * assign21500_e27098);
        let assign21500_e27100: f64 = (assign21500_e27087 / assign21500_e27099);
        (assign21500_e27100, ((((((3.872983346207417 * var_kusai00l_dn0) * var_t7) + (assign21500_e27085 * var_t7_dn0)) * assign21500_e27099) - (assign21500_e27087 * (((6.0 * var_t2_dn0) * assign21500_e27098) + (assign21500_e27090 * (((((((var_gamma_dn0 * var_t2) + (var_gamma * var_t2_dn0)) * var_vgvt) + (assign21500_e27093 * var_vgvt_dn0)) * var_t5) + (assign21500_e27095 * var_t5_dn0)) / (2.0 * assign21500_e27098)))))) / (assign21500_e27099 * assign21500_e27099)), ((((((3.872983346207417 * var_kusai00l_dn2) * var_t7) + (assign21500_e27085 * var_t7_dn2)) * assign21500_e27099) - (assign21500_e27087 * (((6.0 * var_t2_dn2) * assign21500_e27098) + (assign21500_e27090 * (((((((var_gamma_dn2 * var_t2) + (var_gamma * var_t2_dn2)) * var_vgvt) + (assign21500_e27093 * var_vgvt_dn2)) * var_t5) + (assign21500_e27095 * var_t5_dn2)) / (2.0 * assign21500_e27098)))))) / (assign21500_e27099 * assign21500_e27099)), ((((((3.872983346207417 * var_kusai00l_dn4) * var_t7) + (assign21500_e27085 * var_t7_dn4)) * assign21500_e27099) - (assign21500_e27087 * (((6.0 * var_t2_dn4) * assign21500_e27098) + (assign21500_e27090 * (((((((var_gamma_dn4 * var_t2) + (var_gamma * var_t2_dn4)) * var_vgvt) + (assign21500_e27093 * var_vgvt_dn4)) * var_t5) + (assign21500_e27095 * var_t5_dn4)) / (2.0 * assign21500_e27098)))))) / (assign21500_e27099 * assign21500_e27099)), ((((((3.872983346207417 * var_kusai00l_dn5) * var_t7) + (assign21500_e27085 * var_t7_dn5)) * assign21500_e27099) - (assign21500_e27087 * (((6.0 * var_t2_dn5) * assign21500_e27098) + (assign21500_e27090 * (((((((var_gamma_dn5 * var_t2) + (var_gamma * var_t2_dn5)) * var_vgvt) + (assign21500_e27093 * var_vgvt_dn5)) * var_t5) + (assign21500_e27095 * var_t5_dn5)) / (2.0 * assign21500_e27098)))))) / (assign21500_e27099 * assign21500_e27099)), ((((((3.872983346207417 * var_kusai00l_dn6) * var_t7) + (assign21500_e27085 * var_t7_dn6)) * assign21500_e27099) - (assign21500_e27087 * (((6.0 * var_t2_dn6) * assign21500_e27098) + (assign21500_e27090 * (((((((var_gamma_dn6 * var_t2) + (var_gamma * var_t2_dn6)) * var_vgvt) + (assign21500_e27093 * var_vgvt_dn6)) * var_t5) + (assign21500_e27095 * var_t5_dn6)) / (2.0 * assign21500_e27098)))))) / (assign21500_e27099 * assign21500_e27099)), ((((((3.872983346207417 * var_kusai00l_dn8) * var_t7) + (assign21500_e27085 * var_t7_dn8)) * assign21500_e27099) - (assign21500_e27087 * (((6.0 * var_t2_dn8) * assign21500_e27098) + (assign21500_e27090 * (((((((var_gamma_dn8 * var_t2) + (var_gamma * var_t2_dn8)) * var_vgvt) + (assign21500_e27093 * var_vgvt_dn8)) * var_t5) + (assign21500_e27095 * var_t5_dn8)) / (2.0 * assign21500_e27098)))))) / (assign21500_e27099 * assign21500_e27099)), ((((((3.872983346207417 * var_kusai00l_dn10) * var_t7) + (assign21500_e27085 * var_t7_dn10)) * assign21500_e27099) - (assign21500_e27087 * (((6.0 * var_t2_dn10) * assign21500_e27098) + (assign21500_e27090 * (((((((var_gamma_dn10 * var_t2) + (var_gamma * var_t2_dn10)) * var_vgvt) + (assign21500_e27093 * var_vgvt_dn10)) * var_t5) + (assign21500_e27095 * var_t5_dn10)) / (2.0 * assign21500_e27098)))))) / (assign21500_e27099 * assign21500_e27099)), ((((((3.872983346207417 * var_kusai00l_dn11) * var_t7) + (assign21500_e27085 * var_t7_dn11)) * assign21500_e27099) - (assign21500_e27087 * (((6.0 * var_t2_dn11) * assign21500_e27098) + (assign21500_e27090 * (((((((var_gamma_dn11 * var_t2) + (var_gamma * var_t2_dn11)) * var_vgvt) + (assign21500_e27093 * var_vgvt_dn11)) * var_t5) + (assign21500_e27095 * var_t5_dn11)) / (2.0 * assign21500_e27098)))))) / (assign21500_e27099 * assign21500_e27099)), ((((((3.872983346207417 * var_kusai00l_dn12) * var_t7) + (assign21500_e27085 * var_t7_dn12)) * assign21500_e27099) - (assign21500_e27087 * (((6.0 * var_t2_dn12) * assign21500_e27098) + (assign21500_e27090 * (((((((var_gamma_dn12 * var_t2) + (var_gamma * var_t2_dn12)) * var_vgvt) + (assign21500_e27093 * var_vgvt_dn12)) * var_t5) + (assign21500_e27095 * var_t5_dn12)) / (2.0 * assign21500_e27098)))))) / (assign21500_e27099 * assign21500_e27099)),)
    } else {
        (var_crl_f, var_crl_f_dn0, var_crl_f_dn2, var_crl_f_dn4, var_crl_f_dn5, var_crl_f_dn6, var_crl_f_dn8, var_crl_f_dn10, var_crl_f_dn11, var_crl_f_dn12,)
    }
};
        var_crl_f = assign21500_e27102;
        var_crl_f_dn0 = assign21500_e27102_d_n0;
        var_crl_f_dn2 = assign21500_e27102_d_n2;
        var_crl_f_dn4 = assign21500_e27102_d_n4;
        var_crl_f_dn5 = assign21500_e27102_d_n5;
        var_crl_f_dn6 = assign21500_e27102_d_n6;
        var_crl_f_dn8 = assign21500_e27102_d_n8;
        var_crl_f_dn10 = assign21500_e27102_d_n10;
        var_crl_f_dn11 = assign21500_e27102_d_n11;
        var_crl_f_dn12 = assign21500_e27102_d_n12;

        let assign21510_e27105: f64 = (var_ids + var_idsibpc);
        var_ids = assign21510_e27105;
        var_ids_dn0 = (var_ids_dn0 + var_idsibpc_dn0);
        var_ids_dn2 = (var_ids_dn2 + var_idsibpc_dn2);
        var_ids_dn4 = (var_ids_dn4 + var_idsibpc_dn4);
        var_ids_dn5 = (var_ids_dn5 + var_idsibpc_dn5);
        var_ids_dn6 = (var_ids_dn6 + var_idsibpc_dn6);
        var_ids_dn8 = (var_ids_dn8 + var_idsibpc_dn8);
        var_ids_dn10 = (var_ids_dn10 + var_idsibpc_dn10);
        var_ids_dn11 = (var_ids_dn11 + var_idsibpc_dn11);
        var_ids_dn12 = (var_ids_dn12 + var_idsibpc_dn12);

        let (assign21520_e27112,) = {
    if (var_cgbo_given != 0.0) {
        let assign21520_e27108: f64 = (-p.p172);
        let assign21520_e27110: f64 = (assign21520_e27108 * var_lgate);
        (assign21520_e27110,)
    } else {
        (var_cgbe,)
    }
};
        var_cgbe = assign21520_e27112;

        let (assign21530_e27120, assign21530_e27120_d_n0, assign21530_e27120_d_n2, assign21530_e27120_d_n5, assign21530_e27120_d_n6,) = {
    if (var_cgbo_given != 0.0) {
        let assign21530_e27117: f64 = (var_vgse - var_vbse);
        let assign21530_e27118: f64 = (var_cgbe * assign21530_e27117);
        (assign21530_e27118, (var_cgbe * (var_vgse_dn0 - var_vbse_dn0)), (var_cgbe * (var_vgse_dn2 - var_vbse_dn2)), (var_cgbe * var_vgse_dn5), (var_cgbe * (-var_vbse_dn6)),)
    } else {
        (var_qgob, var_qgob_dn0, var_qgob_dn2, var_qgob_dn5, var_qgob_dn6,)
    }
};
        var_qgob = assign21530_e27120;
        var_qgob_dn0 = assign21530_e27120_d_n0;
        var_qgob_dn2 = assign21530_e27120_d_n2;
        var_qgob_dn5 = assign21530_e27120_d_n5;
        var_qgob_dn6 = assign21530_e27120_d_n6;

        let (assign21540_e27125,) = {
    if (var_cgbo_given == 0.0) {
        (0.0,)
    } else {
        (var_cgbe,)
    }
};
        var_cgbe = assign21540_e27125;

        *var_cgbe_slot = var_cgbe;
        *var_crl_f_slot = var_crl_f;
        *var_crl_f_dn0_slot = var_crl_f_dn0;
        *var_crl_f_dn10_slot = var_crl_f_dn10;
        *var_crl_f_dn11_slot = var_crl_f_dn11;
        *var_crl_f_dn12_slot = var_crl_f_dn12;
        *var_crl_f_dn2_slot = var_crl_f_dn2;
        *var_crl_f_dn4_slot = var_crl_f_dn4;
        *var_crl_f_dn5_slot = var_crl_f_dn5;
        *var_crl_f_dn6_slot = var_crl_f_dn6;
        *var_crl_f_dn8_slot = var_crl_f_dn8;
        *var_gamma_slot = var_gamma;
        *var_gamma_dn0_slot = var_gamma_dn0;
        *var_gamma_dn10_slot = var_gamma_dn10;
        *var_gamma_dn11_slot = var_gamma_dn11;
        *var_gamma_dn12_slot = var_gamma_dn12;
        *var_gamma_dn2_slot = var_gamma_dn2;
        *var_gamma_dn4_slot = var_gamma_dn4;
        *var_gamma_dn5_slot = var_gamma_dn5;
        *var_gamma_dn6_slot = var_gamma_dn6;
        *var_gamma_dn8_slot = var_gamma_dn8;
        *var_gds0_h2_slot = var_gds0_h2;
        *var_gds0_h2_dn0_slot = var_gds0_h2_dn0;
        *var_gds0_h2_dn10_slot = var_gds0_h2_dn10;
        *var_gds0_h2_dn11_slot = var_gds0_h2_dn11;
        *var_gds0_h2_dn12_slot = var_gds0_h2_dn12;
        *var_gds0_h2_dn2_slot = var_gds0_h2_dn2;
        *var_gds0_h2_dn4_slot = var_gds0_h2_dn4;
        *var_gds0_h2_dn5_slot = var_gds0_h2_dn5;
        *var_gds0_h2_dn6_slot = var_gds0_h2_dn6;
        *var_gds0_h2_dn8_slot = var_gds0_h2_dn8;
        *var_gds0_ign_slot = var_gds0_ign;
        *var_gds0_ign_dn0_slot = var_gds0_ign_dn0;
        *var_gds0_ign_dn10_slot = var_gds0_ign_dn10;
        *var_gds0_ign_dn11_slot = var_gds0_ign_dn11;
        *var_gds0_ign_dn12_slot = var_gds0_ign_dn12;
        *var_gds0_ign_dn2_slot = var_gds0_ign_dn2;
        *var_gds0_ign_dn4_slot = var_gds0_ign_dn4;
        *var_gds0_ign_dn5_slot = var_gds0_ign_dn5;
        *var_gds0_ign_dn6_slot = var_gds0_ign_dn6;
        *var_gds0_ign_dn8_slot = var_gds0_ign_dn8;
        *var_guard371_slot = var_guard371;
        *var_ids_slot = var_ids;
        *var_ids_dn0_slot = var_ids_dn0;
        *var_ids_dn10_slot = var_ids_dn10;
        *var_ids_dn11_slot = var_ids_dn11;
        *var_ids_dn12_slot = var_ids_dn12;
        *var_ids_dn2_slot = var_ids_dn2;
        *var_ids_dn4_slot = var_ids_dn4;
        *var_ids_dn5_slot = var_ids_dn5;
        *var_ids_dn6_slot = var_ids_dn6;
        *var_ids_dn8_slot = var_ids_dn8;
        *var_kusai_ig_slot = var_kusai_ig;
        *var_kusai_ig_dn0_slot = var_kusai_ig_dn0;
        *var_kusai_ig_dn10_slot = var_kusai_ig_dn10;
        *var_kusai_ig_dn11_slot = var_kusai_ig_dn11;
        *var_kusai_ig_dn12_slot = var_kusai_ig_dn12;
        *var_kusai_ig_dn2_slot = var_kusai_ig_dn2;
        *var_kusai_ig_dn4_slot = var_kusai_ig_dn4;
        *var_kusai_ig_dn5_slot = var_kusai_ig_dn5;
        *var_kusai_ig_dn6_slot = var_kusai_ig_dn6;
        *var_kusai_ig_dn8_slot = var_kusai_ig_dn8;
        *var_mu_ave_slot = var_mu_ave;
        *var_mu_ave_dn0_slot = var_mu_ave_dn0;
        *var_mu_ave_dn10_slot = var_mu_ave_dn10;
        *var_mu_ave_dn11_slot = var_mu_ave_dn11;
        *var_mu_ave_dn12_slot = var_mu_ave_dn12;
        *var_mu_ave_dn2_slot = var_mu_ave_dn2;
        *var_mu_ave_dn4_slot = var_mu_ave_dn4;
        *var_mu_ave_dn5_slot = var_mu_ave_dn5;
        *var_mu_ave_dn6_slot = var_mu_ave_dn6;
        *var_mu_ave_dn8_slot = var_mu_ave_dn8;
        *var_mud_hoso_slot = var_mud_hoso;
        *var_mud_hoso_dn0_slot = var_mud_hoso_dn0;
        *var_mud_hoso_dn10_slot = var_mud_hoso_dn10;
        *var_mud_hoso_dn11_slot = var_mud_hoso_dn11;
        *var_mud_hoso_dn12_slot = var_mud_hoso_dn12;
        *var_mud_hoso_dn2_slot = var_mud_hoso_dn2;
        *var_mud_hoso_dn4_slot = var_mud_hoso_dn4;
        *var_mud_hoso_dn5_slot = var_mud_hoso_dn5;
        *var_mud_hoso_dn6_slot = var_mud_hoso_dn6;
        *var_mud_hoso_dn8_slot = var_mud_hoso_dn8;
        *var_nthrml_slot = var_nthrml;
        *var_nthrml_dn0_slot = var_nthrml_dn0;
        *var_nthrml_dn10_slot = var_nthrml_dn10;
        *var_nthrml_dn11_slot = var_nthrml_dn11;
        *var_nthrml_dn12_slot = var_nthrml_dn12;
        *var_nthrml_dn2_slot = var_nthrml_dn2;
        *var_nthrml_dn4_slot = var_nthrml_dn4;
        *var_nthrml_dn5_slot = var_nthrml_dn5;
        *var_nthrml_dn6_slot = var_nthrml_dn6;
        *var_nthrml_dn8_slot = var_nthrml_dn8;
        *var_qgob_slot = var_qgob;
        *var_qgob_dn0_slot = var_qgob_dn0;
        *var_qgob_dn2_slot = var_qgob_dn2;
        *var_qgob_dn5_slot = var_qgob_dn5;
        *var_qgob_dn6_slot = var_qgob_dn6;
        *var_sqrtkusail_slot = var_sqrtkusail;
        *var_sqrtkusail_dn0_slot = var_sqrtkusail_dn0;
        *var_sqrtkusail_dn10_slot = var_sqrtkusail_dn10;
        *var_sqrtkusail_dn11_slot = var_sqrtkusail_dn11;
        *var_sqrtkusail_dn12_slot = var_sqrtkusail_dn12;
        *var_sqrtkusail_dn2_slot = var_sqrtkusail_dn2;
        *var_sqrtkusail_dn4_slot = var_sqrtkusail_dn4;
        *var_sqrtkusail_dn5_slot = var_sqrtkusail_dn5;
        *var_sqrtkusail_dn6_slot = var_sqrtkusail_dn6;
        *var_sqrtkusail_dn8_slot = var_sqrtkusail_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t10_slot = var_t10;
        *var_t10_dn0_slot = var_t10_dn0;
        *var_t10_dn10_slot = var_t10_dn10;
        *var_t10_dn11_slot = var_t10_dn11;
        *var_t10_dn12_slot = var_t10_dn12;
        *var_t10_dn2_slot = var_t10_dn2;
        *var_t10_dn4_slot = var_t10_dn4;
        *var_t10_dn5_slot = var_t10_dn5;
        *var_t10_dn6_slot = var_t10_dn6;
        *var_t10_dn8_slot = var_t10_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn12_slot = var_t7_dn12;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t9_slot = var_t9;
        *var_t9_dn0_slot = var_t9_dn0;
        *var_t9_dn10_slot = var_t9_dn10;
        *var_t9_dn11_slot = var_t9_dn11;
        *var_t9_dn12_slot = var_t9_dn12;
        *var_t9_dn2_slot = var_t9_dn2;
        *var_t9_dn4_slot = var_t9_dn4;
        *var_t9_dn5_slot = var_t9_dn5;
        *var_t9_dn6_slot = var_t9_dn6;
        *var_t9_dn8_slot = var_t9_dn8;
    }

    pub(super) fn stamp_transient_block_83(
        p: &Parameters,
        var_aclm: f64,
        var_c_box: f64,
        var_cgbo_given: f64,
        var_ec: f64,
        var_ec_dn0: f64,
        var_ec_dn10: f64,
        var_ec_dn11: f64,
        var_ec_dn12: f64,
        var_ec_dn2: f64,
        var_ec_dn4: f64,
        var_ec_dn5: f64,
        var_ec_dn6: f64,
        var_ec_dn8: f64,
        var_flg_nqs: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn12: f64,
        var_ids_dn2: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn8: f64,
        var_leff: f64,
        var_leff_dn0: f64,
        var_leff_dn10: f64,
        var_leff_dn11: f64,
        var_leff_dn12: f64,
        var_leff_dn2: f64,
        var_leff_dn4: f64,
        var_leff_dn5: f64,
        var_leff_dn6: f64,
        var_leff_dn8: f64,
        var_mfactor: f64,
        var_ps0: f64,
        var_ps0_dn0: f64,
        var_ps0_dn10: f64,
        var_ps0_dn11: f64,
        var_ps0_dn12: f64,
        var_ps0_dn2: f64,
        var_ps0_dn4: f64,
        var_ps0_dn5: f64,
        var_ps0_dn6: f64,
        var_ps0_dn8: f64,
        var_psdl: f64,
        var_psdl_dn0: f64,
        var_psdl_dn10: f64,
        var_psdl_dn11: f64,
        var_psdl_dn12: f64,
        var_psdl_dn2: f64,
        var_psdl_dn4: f64,
        var_psdl_dn5: f64,
        var_psdl_dn6: f64,
        var_psdl_dn8: f64,
        var_q_b0_dep: f64,
        var_q_b0_dep_dn0: f64,
        var_q_b0_dep_dn10: f64,
        var_q_b0_dep_dn11: f64,
        var_q_b0_dep_dn12: f64,
        var_q_b0_dep_dn2: f64,
        var_q_b0_dep_dn4: f64,
        var_q_b0_dep_dn5: f64,
        var_q_b0_dep_dn6: f64,
        var_q_b0_dep_dn8: f64,
        var_q_bl_dep: f64,
        var_q_bl_dep_dn0: f64,
        var_q_bl_dep_dn10: f64,
        var_q_bl_dep_dn11: f64,
        var_q_bl_dep_dn12: f64,
        var_q_bl_dep_dn2: f64,
        var_q_bl_dep_dn4: f64,
        var_q_bl_dep_dn5: f64,
        var_q_bl_dep_dn6: f64,
        var_q_bl_dep_dn8: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn11: f64,
        var_q_nsub_dn12: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn4: f64,
        var_q_nsub_dn5: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn8: f64,
        var_q_s0_dep: f64,
        var_q_s0_dep_dn0: f64,
        var_q_s0_dep_dn10: f64,
        var_q_s0_dep_dn11: f64,
        var_q_s0_dep_dn12: f64,
        var_q_s0_dep_dn2: f64,
        var_q_s0_dep_dn4: f64,
        var_q_s0_dep_dn5: f64,
        var_q_s0_dep_dn6: f64,
        var_q_s0_dep_dn8: f64,
        var_q_sl_dep: f64,
        var_q_sl_dep_dn0: f64,
        var_q_sl_dep_dn10: f64,
        var_q_sl_dep_dn11: f64,
        var_q_sl_dep_dn12: f64,
        var_q_sl_dep_dn2: f64,
        var_q_sl_dep_dn4: f64,
        var_q_sl_dep_dn5: f64,
        var_q_sl_dep_dn6: f64,
        var_q_sl_dep_dn8: f64,
        var_qdrat: f64,
        var_qi: f64,
        var_qi_dn0: f64,
        var_qi_dn10: f64,
        var_qi_dn11: f64,
        var_qi_dn12: f64,
        var_qi_dn2: f64,
        var_qi_dn4: f64,
        var_qi_dn5: f64,
        var_qi_dn6: f64,
        var_qi_dn8: f64,
        var_vbse: f64,
        var_vbse_dn0: f64,
        var_vbse_dn2: f64,
        var_vbse_dn6: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn8: f64,
        var_vdse: f64,
        var_vdse_dn0: f64,
        var_vdse_dn2: f64,
        var_vgse: f64,
        var_vgse_dn0: f64,
        var_vgse_dn2: f64,
        var_vgse_dn5: f64,
        var_weffcv_nf: f64,
        var_weffcv_nf_dn0: f64,
        var_weffcv_nf_dn10: f64,
        var_weffcv_nf_dn11: f64,
        var_weffcv_nf_dn12: f64,
        var_weffcv_nf_dn2: f64,
        var_weffcv_nf_dn4: f64,
        var_weffcv_nf_dn5: f64,
        var_weffcv_nf_dn6: f64,
        var_weffcv_nf_dn8: f64,
        var_cf_slot: &mut f64,
        var_cf_dn0_slot: &mut f64,
        var_cf_dn10_slot: &mut f64,
        var_cf_dn11_slot: &mut f64,
        var_cf_dn12_slot: &mut f64,
        var_cf_dn2_slot: &mut f64,
        var_cf_dn4_slot: &mut f64,
        var_cf_dn5_slot: &mut f64,
        var_cf_dn6_slot: &mut f64,
        var_cf_dn8_slot: &mut f64,
        var_guard372_slot: &mut f64,
        var_guard373_slot: &mut f64,
        var_idse_slot: &mut f64,
        var_idse_dn0_slot: &mut f64,
        var_idse_dn10_slot: &mut f64,
        var_idse_dn11_slot: &mut f64,
        var_idse_dn12_slot: &mut f64,
        var_idse_dn2_slot: &mut f64,
        var_idse_dn4_slot: &mut f64,
        var_idse_dn5_slot: &mut f64,
        var_idse_dn6_slot: &mut f64,
        var_idse_dn8_slot: &mut f64,
        var_pslk_slot: &mut f64,
        var_pslk_dn0_slot: &mut f64,
        var_pslk_dn10_slot: &mut f64,
        var_pslk_dn11_slot: &mut f64,
        var_pslk_dn12_slot: &mut f64,
        var_pslk_dn2_slot: &mut f64,
        var_pslk_dn4_slot: &mut f64,
        var_pslk_dn5_slot: &mut f64,
        var_pslk_dn6_slot: &mut f64,
        var_pslk_dn8_slot: &mut f64,
        var_qb_dep_slot: &mut f64,
        var_qb_dep_dn0_slot: &mut f64,
        var_qb_dep_dn10_slot: &mut f64,
        var_qb_dep_dn11_slot: &mut f64,
        var_qb_dep_dn12_slot: &mut f64,
        var_qb_dep_dn2_slot: &mut f64,
        var_qb_dep_dn4_slot: &mut f64,
        var_qb_dep_dn5_slot: &mut f64,
        var_qb_dep_dn6_slot: &mut f64,
        var_qb_dep_dn8_slot: &mut f64,
        var_qb_qs_slot: &mut f64,
        var_qb_qs_dn0_slot: &mut f64,
        var_qb_qs_dn10_slot: &mut f64,
        var_qb_qs_dn11_slot: &mut f64,
        var_qb_qs_dn12_slot: &mut f64,
        var_qb_qs_dn2_slot: &mut f64,
        var_qb_qs_dn4_slot: &mut f64,
        var_qb_qs_dn5_slot: &mut f64,
        var_qb_qs_dn6_slot: &mut f64,
        var_qb_qs_dn8_slot: &mut f64,
        var_qbe_slot: &mut f64,
        var_qbe_dn0_slot: &mut f64,
        var_qbe_dn10_slot: &mut f64,
        var_qbe_dn11_slot: &mut f64,
        var_qbe_dn12_slot: &mut f64,
        var_qbe_dn2_slot: &mut f64,
        var_qbe_dn4_slot: &mut f64,
        var_qbe_dn5_slot: &mut f64,
        var_qbe_dn6_slot: &mut f64,
        var_qbe_dn8_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn0_slot: &mut f64,
        var_qd_dn10_slot: &mut f64,
        var_qd_dn11_slot: &mut f64,
        var_qd_dn12_slot: &mut f64,
        var_qd_dn2_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_dn0_slot: &mut f64,
        var_qde_dn10_slot: &mut f64,
        var_qde_dn11_slot: &mut f64,
        var_qde_dn12_slot: &mut f64,
        var_qde_dn2_slot: &mut f64,
        var_qde_dn4_slot: &mut f64,
        var_qde_dn5_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qde_dn8_slot: &mut f64,
        var_qfd_slot: &mut f64,
        var_qfd_box_slot: &mut f64,
        var_qfd_box_dn0_slot: &mut f64,
        var_qfd_box_dn10_slot: &mut f64,
        var_qfd_box_dn11_slot: &mut f64,
        var_qfd_box_dn12_slot: &mut f64,
        var_qfd_box_dn2_slot: &mut f64,
        var_qfd_box_dn4_slot: &mut f64,
        var_qfd_box_dn5_slot: &mut f64,
        var_qfd_box_dn6_slot: &mut f64,
        var_qfd_box_dn8_slot: &mut f64,
        var_qfd_dn0_slot: &mut f64,
        var_qfd_dn10_slot: &mut f64,
        var_qfd_dn11_slot: &mut f64,
        var_qfd_dn12_slot: &mut f64,
        var_qfd_dn2_slot: &mut f64,
        var_qfd_dn4_slot: &mut f64,
        var_qfd_dn5_slot: &mut f64,
        var_qfd_dn6_slot: &mut f64,
        var_qfd_dn8_slot: &mut f64,
        var_qfs_slot: &mut f64,
        var_qfs_box_slot: &mut f64,
        var_qfs_box_dn0_slot: &mut f64,
        var_qfs_box_dn10_slot: &mut f64,
        var_qfs_box_dn11_slot: &mut f64,
        var_qfs_box_dn12_slot: &mut f64,
        var_qfs_box_dn2_slot: &mut f64,
        var_qfs_box_dn4_slot: &mut f64,
        var_qfs_box_dn5_slot: &mut f64,
        var_qfs_box_dn6_slot: &mut f64,
        var_qfs_box_dn8_slot: &mut f64,
        var_qfs_dn0_slot: &mut f64,
        var_qfs_dn10_slot: &mut f64,
        var_qfs_dn11_slot: &mut f64,
        var_qfs_dn12_slot: &mut f64,
        var_qfs_dn2_slot: &mut f64,
        var_qfs_dn4_slot: &mut f64,
        var_qfs_dn5_slot: &mut f64,
        var_qfs_dn6_slot: &mut f64,
        var_qfs_dn8_slot: &mut f64,
        var_qge_slot: &mut f64,
        var_qge_dn0_slot: &mut f64,
        var_qge_dn10_slot: &mut f64,
        var_qge_dn11_slot: &mut f64,
        var_qge_dn12_slot: &mut f64,
        var_qge_dn2_slot: &mut f64,
        var_qge_dn4_slot: &mut f64,
        var_qge_dn5_slot: &mut f64,
        var_qge_dn6_slot: &mut f64,
        var_qge_dn8_slot: &mut f64,
        var_qgob_slot: &mut f64,
        var_qgob_dn0_slot: &mut f64,
        var_qgob_dn2_slot: &mut f64,
        var_qgob_dn5_slot: &mut f64,
        var_qgob_dn6_slot: &mut f64,
        var_qgod_slot: &mut f64,
        var_qgod_dn0_slot: &mut f64,
        var_qgod_dn10_slot: &mut f64,
        var_qgod_dn11_slot: &mut f64,
        var_qgod_dn12_slot: &mut f64,
        var_qgod_dn2_slot: &mut f64,
        var_qgod_dn4_slot: &mut f64,
        var_qgod_dn5_slot: &mut f64,
        var_qgod_dn6_slot: &mut f64,
        var_qgod_dn8_slot: &mut f64,
        var_qgos_slot: &mut f64,
        var_qgos_dn0_slot: &mut f64,
        var_qgos_dn10_slot: &mut f64,
        var_qgos_dn11_slot: &mut f64,
        var_qgos_dn12_slot: &mut f64,
        var_qgos_dn2_slot: &mut f64,
        var_qgos_dn4_slot: &mut f64,
        var_qgos_dn5_slot: &mut f64,
        var_qgos_dn6_slot: &mut f64,
        var_qgos_dn8_slot: &mut f64,
        var_qi_qs_slot: &mut f64,
        var_qi_qs_dn0_slot: &mut f64,
        var_qi_qs_dn10_slot: &mut f64,
        var_qi_qs_dn11_slot: &mut f64,
        var_qi_qs_dn12_slot: &mut f64,
        var_qi_qs_dn2_slot: &mut f64,
        var_qi_qs_dn4_slot: &mut f64,
        var_qi_qs_dn5_slot: &mut f64,
        var_qi_qs_dn6_slot: &mut f64,
        var_qi_qs_dn8_slot: &mut f64,
        var_qidep_slot: &mut f64,
        var_qidep_dn0_slot: &mut f64,
        var_qidep_dn10_slot: &mut f64,
        var_qidep_dn11_slot: &mut f64,
        var_qidep_dn12_slot: &mut f64,
        var_qidep_dn2_slot: &mut f64,
        var_qidep_dn4_slot: &mut f64,
        var_qidep_dn5_slot: &mut f64,
        var_qidep_dn6_slot: &mut f64,
        var_qidep_dn8_slot: &mut f64,
        var_qs_dep_slot: &mut f64,
        var_qs_dep_dn0_slot: &mut f64,
        var_qs_dep_dn10_slot: &mut f64,
        var_qs_dep_dn11_slot: &mut f64,
        var_qs_dep_dn12_slot: &mut f64,
        var_qs_dep_dn2_slot: &mut f64,
        var_qs_dep_dn4_slot: &mut f64,
        var_qs_dep_dn5_slot: &mut f64,
        var_qs_dep_dn6_slot: &mut f64,
        var_qs_dep_dn8_slot: &mut f64,
        var_qse_slot: &mut f64,
        var_qse_dn0_slot: &mut f64,
        var_qse_dn10_slot: &mut f64,
        var_qse_dn11_slot: &mut f64,
        var_qse_dn12_slot: &mut f64,
        var_qse_dn2_slot: &mut f64,
        var_qse_dn4_slot: &mut f64,
        var_qse_dn5_slot: &mut f64,
        var_qse_dn6_slot: &mut f64,
        var_qse_dn8_slot: &mut f64,
        var_qsub_slot: &mut f64,
        var_qsub_dn0_slot: &mut f64,
        var_qsub_dn10_slot: &mut f64,
        var_qsub_dn11_slot: &mut f64,
        var_qsub_dn12_slot: &mut f64,
        var_qsub_dn2_slot: &mut f64,
        var_qsub_dn4_slot: &mut f64,
        var_qsub_dn5_slot: &mut f64,
        var_qsub_dn6_slot: &mut f64,
        var_qsub_dn8_slot: &mut f64,
        var_qy_slot: &mut f64,
        var_qy_dn0_slot: &mut f64,
        var_qy_dn10_slot: &mut f64,
        var_qy_dn11_slot: &mut f64,
        var_qy_dn12_slot: &mut f64,
        var_qy_dn2_slot: &mut f64,
        var_qy_dn4_slot: &mut f64,
        var_qy_dn5_slot: &mut f64,
        var_qy_dn6_slot: &mut f64,
        var_qy_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t10_slot: &mut f64,
        var_t10_dn0_slot: &mut f64,
        var_t10_dn10_slot: &mut f64,
        var_t10_dn11_slot: &mut f64,
        var_t10_dn12_slot: &mut f64,
        var_t10_dn2_slot: &mut f64,
        var_t10_dn4_slot: &mut f64,
        var_t10_dn5_slot: &mut f64,
        var_t10_dn6_slot: &mut f64,
        var_t10_dn8_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
    ) {
        let mut var_cf: f64 = *var_cf_slot;
        let mut var_cf_dn0: f64 = *var_cf_dn0_slot;
        let mut var_cf_dn10: f64 = *var_cf_dn10_slot;
        let mut var_cf_dn11: f64 = *var_cf_dn11_slot;
        let mut var_cf_dn12: f64 = *var_cf_dn12_slot;
        let mut var_cf_dn2: f64 = *var_cf_dn2_slot;
        let mut var_cf_dn4: f64 = *var_cf_dn4_slot;
        let mut var_cf_dn5: f64 = *var_cf_dn5_slot;
        let mut var_cf_dn6: f64 = *var_cf_dn6_slot;
        let mut var_cf_dn8: f64 = *var_cf_dn8_slot;
        let mut var_guard372: f64 = *var_guard372_slot;
        let mut var_guard373: f64 = *var_guard373_slot;
        let mut var_idse: f64 = *var_idse_slot;
        let mut var_idse_dn0: f64 = *var_idse_dn0_slot;
        let mut var_idse_dn10: f64 = *var_idse_dn10_slot;
        let mut var_idse_dn11: f64 = *var_idse_dn11_slot;
        let mut var_idse_dn12: f64 = *var_idse_dn12_slot;
        let mut var_idse_dn2: f64 = *var_idse_dn2_slot;
        let mut var_idse_dn4: f64 = *var_idse_dn4_slot;
        let mut var_idse_dn5: f64 = *var_idse_dn5_slot;
        let mut var_idse_dn6: f64 = *var_idse_dn6_slot;
        let mut var_idse_dn8: f64 = *var_idse_dn8_slot;
        let mut var_pslk: f64 = *var_pslk_slot;
        let mut var_pslk_dn0: f64 = *var_pslk_dn0_slot;
        let mut var_pslk_dn10: f64 = *var_pslk_dn10_slot;
        let mut var_pslk_dn11: f64 = *var_pslk_dn11_slot;
        let mut var_pslk_dn12: f64 = *var_pslk_dn12_slot;
        let mut var_pslk_dn2: f64 = *var_pslk_dn2_slot;
        let mut var_pslk_dn4: f64 = *var_pslk_dn4_slot;
        let mut var_pslk_dn5: f64 = *var_pslk_dn5_slot;
        let mut var_pslk_dn6: f64 = *var_pslk_dn6_slot;
        let mut var_pslk_dn8: f64 = *var_pslk_dn8_slot;
        let mut var_qb_dep: f64 = *var_qb_dep_slot;
        let mut var_qb_dep_dn0: f64 = *var_qb_dep_dn0_slot;
        let mut var_qb_dep_dn10: f64 = *var_qb_dep_dn10_slot;
        let mut var_qb_dep_dn11: f64 = *var_qb_dep_dn11_slot;
        let mut var_qb_dep_dn12: f64 = *var_qb_dep_dn12_slot;
        let mut var_qb_dep_dn2: f64 = *var_qb_dep_dn2_slot;
        let mut var_qb_dep_dn4: f64 = *var_qb_dep_dn4_slot;
        let mut var_qb_dep_dn5: f64 = *var_qb_dep_dn5_slot;
        let mut var_qb_dep_dn6: f64 = *var_qb_dep_dn6_slot;
        let mut var_qb_dep_dn8: f64 = *var_qb_dep_dn8_slot;
        let mut var_qb_qs: f64 = *var_qb_qs_slot;
        let mut var_qb_qs_dn0: f64 = *var_qb_qs_dn0_slot;
        let mut var_qb_qs_dn10: f64 = *var_qb_qs_dn10_slot;
        let mut var_qb_qs_dn11: f64 = *var_qb_qs_dn11_slot;
        let mut var_qb_qs_dn12: f64 = *var_qb_qs_dn12_slot;
        let mut var_qb_qs_dn2: f64 = *var_qb_qs_dn2_slot;
        let mut var_qb_qs_dn4: f64 = *var_qb_qs_dn4_slot;
        let mut var_qb_qs_dn5: f64 = *var_qb_qs_dn5_slot;
        let mut var_qb_qs_dn6: f64 = *var_qb_qs_dn6_slot;
        let mut var_qb_qs_dn8: f64 = *var_qb_qs_dn8_slot;
        let mut var_qbe: f64 = *var_qbe_slot;
        let mut var_qbe_dn0: f64 = *var_qbe_dn0_slot;
        let mut var_qbe_dn10: f64 = *var_qbe_dn10_slot;
        let mut var_qbe_dn11: f64 = *var_qbe_dn11_slot;
        let mut var_qbe_dn12: f64 = *var_qbe_dn12_slot;
        let mut var_qbe_dn2: f64 = *var_qbe_dn2_slot;
        let mut var_qbe_dn4: f64 = *var_qbe_dn4_slot;
        let mut var_qbe_dn5: f64 = *var_qbe_dn5_slot;
        let mut var_qbe_dn6: f64 = *var_qbe_dn6_slot;
        let mut var_qbe_dn8: f64 = *var_qbe_dn8_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn0: f64 = *var_qd_dn0_slot;
        let mut var_qd_dn10: f64 = *var_qd_dn10_slot;
        let mut var_qd_dn11: f64 = *var_qd_dn11_slot;
        let mut var_qd_dn12: f64 = *var_qd_dn12_slot;
        let mut var_qd_dn2: f64 = *var_qd_dn2_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_dn0: f64 = *var_qde_dn0_slot;
        let mut var_qde_dn10: f64 = *var_qde_dn10_slot;
        let mut var_qde_dn11: f64 = *var_qde_dn11_slot;
        let mut var_qde_dn12: f64 = *var_qde_dn12_slot;
        let mut var_qde_dn2: f64 = *var_qde_dn2_slot;
        let mut var_qde_dn4: f64 = *var_qde_dn4_slot;
        let mut var_qde_dn5: f64 = *var_qde_dn5_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qde_dn8: f64 = *var_qde_dn8_slot;
        let mut var_qfd: f64 = *var_qfd_slot;
        let mut var_qfd_box: f64 = *var_qfd_box_slot;
        let mut var_qfd_box_dn0: f64 = *var_qfd_box_dn0_slot;
        let mut var_qfd_box_dn10: f64 = *var_qfd_box_dn10_slot;
        let mut var_qfd_box_dn11: f64 = *var_qfd_box_dn11_slot;
        let mut var_qfd_box_dn12: f64 = *var_qfd_box_dn12_slot;
        let mut var_qfd_box_dn2: f64 = *var_qfd_box_dn2_slot;
        let mut var_qfd_box_dn4: f64 = *var_qfd_box_dn4_slot;
        let mut var_qfd_box_dn5: f64 = *var_qfd_box_dn5_slot;
        let mut var_qfd_box_dn6: f64 = *var_qfd_box_dn6_slot;
        let mut var_qfd_box_dn8: f64 = *var_qfd_box_dn8_slot;
        let mut var_qfd_dn0: f64 = *var_qfd_dn0_slot;
        let mut var_qfd_dn10: f64 = *var_qfd_dn10_slot;
        let mut var_qfd_dn11: f64 = *var_qfd_dn11_slot;
        let mut var_qfd_dn12: f64 = *var_qfd_dn12_slot;
        let mut var_qfd_dn2: f64 = *var_qfd_dn2_slot;
        let mut var_qfd_dn4: f64 = *var_qfd_dn4_slot;
        let mut var_qfd_dn5: f64 = *var_qfd_dn5_slot;
        let mut var_qfd_dn6: f64 = *var_qfd_dn6_slot;
        let mut var_qfd_dn8: f64 = *var_qfd_dn8_slot;
        let mut var_qfs: f64 = *var_qfs_slot;
        let mut var_qfs_box: f64 = *var_qfs_box_slot;
        let mut var_qfs_box_dn0: f64 = *var_qfs_box_dn0_slot;
        let mut var_qfs_box_dn10: f64 = *var_qfs_box_dn10_slot;
        let mut var_qfs_box_dn11: f64 = *var_qfs_box_dn11_slot;
        let mut var_qfs_box_dn12: f64 = *var_qfs_box_dn12_slot;
        let mut var_qfs_box_dn2: f64 = *var_qfs_box_dn2_slot;
        let mut var_qfs_box_dn4: f64 = *var_qfs_box_dn4_slot;
        let mut var_qfs_box_dn5: f64 = *var_qfs_box_dn5_slot;
        let mut var_qfs_box_dn6: f64 = *var_qfs_box_dn6_slot;
        let mut var_qfs_box_dn8: f64 = *var_qfs_box_dn8_slot;
        let mut var_qfs_dn0: f64 = *var_qfs_dn0_slot;
        let mut var_qfs_dn10: f64 = *var_qfs_dn10_slot;
        let mut var_qfs_dn11: f64 = *var_qfs_dn11_slot;
        let mut var_qfs_dn12: f64 = *var_qfs_dn12_slot;
        let mut var_qfs_dn2: f64 = *var_qfs_dn2_slot;
        let mut var_qfs_dn4: f64 = *var_qfs_dn4_slot;
        let mut var_qfs_dn5: f64 = *var_qfs_dn5_slot;
        let mut var_qfs_dn6: f64 = *var_qfs_dn6_slot;
        let mut var_qfs_dn8: f64 = *var_qfs_dn8_slot;
        let mut var_qge: f64 = *var_qge_slot;
        let mut var_qge_dn0: f64 = *var_qge_dn0_slot;
        let mut var_qge_dn10: f64 = *var_qge_dn10_slot;
        let mut var_qge_dn11: f64 = *var_qge_dn11_slot;
        let mut var_qge_dn12: f64 = *var_qge_dn12_slot;
        let mut var_qge_dn2: f64 = *var_qge_dn2_slot;
        let mut var_qge_dn4: f64 = *var_qge_dn4_slot;
        let mut var_qge_dn5: f64 = *var_qge_dn5_slot;
        let mut var_qge_dn6: f64 = *var_qge_dn6_slot;
        let mut var_qge_dn8: f64 = *var_qge_dn8_slot;
        let mut var_qgob: f64 = *var_qgob_slot;
        let mut var_qgob_dn0: f64 = *var_qgob_dn0_slot;
        let mut var_qgob_dn2: f64 = *var_qgob_dn2_slot;
        let mut var_qgob_dn5: f64 = *var_qgob_dn5_slot;
        let mut var_qgob_dn6: f64 = *var_qgob_dn6_slot;
        let mut var_qgod: f64 = *var_qgod_slot;
        let mut var_qgod_dn0: f64 = *var_qgod_dn0_slot;
        let mut var_qgod_dn10: f64 = *var_qgod_dn10_slot;
        let mut var_qgod_dn11: f64 = *var_qgod_dn11_slot;
        let mut var_qgod_dn12: f64 = *var_qgod_dn12_slot;
        let mut var_qgod_dn2: f64 = *var_qgod_dn2_slot;
        let mut var_qgod_dn4: f64 = *var_qgod_dn4_slot;
        let mut var_qgod_dn5: f64 = *var_qgod_dn5_slot;
        let mut var_qgod_dn6: f64 = *var_qgod_dn6_slot;
        let mut var_qgod_dn8: f64 = *var_qgod_dn8_slot;
        let mut var_qgos: f64 = *var_qgos_slot;
        let mut var_qgos_dn0: f64 = *var_qgos_dn0_slot;
        let mut var_qgos_dn10: f64 = *var_qgos_dn10_slot;
        let mut var_qgos_dn11: f64 = *var_qgos_dn11_slot;
        let mut var_qgos_dn12: f64 = *var_qgos_dn12_slot;
        let mut var_qgos_dn2: f64 = *var_qgos_dn2_slot;
        let mut var_qgos_dn4: f64 = *var_qgos_dn4_slot;
        let mut var_qgos_dn5: f64 = *var_qgos_dn5_slot;
        let mut var_qgos_dn6: f64 = *var_qgos_dn6_slot;
        let mut var_qgos_dn8: f64 = *var_qgos_dn8_slot;
        let mut var_qi_qs: f64 = *var_qi_qs_slot;
        let mut var_qi_qs_dn0: f64 = *var_qi_qs_dn0_slot;
        let mut var_qi_qs_dn10: f64 = *var_qi_qs_dn10_slot;
        let mut var_qi_qs_dn11: f64 = *var_qi_qs_dn11_slot;
        let mut var_qi_qs_dn12: f64 = *var_qi_qs_dn12_slot;
        let mut var_qi_qs_dn2: f64 = *var_qi_qs_dn2_slot;
        let mut var_qi_qs_dn4: f64 = *var_qi_qs_dn4_slot;
        let mut var_qi_qs_dn5: f64 = *var_qi_qs_dn5_slot;
        let mut var_qi_qs_dn6: f64 = *var_qi_qs_dn6_slot;
        let mut var_qi_qs_dn8: f64 = *var_qi_qs_dn8_slot;
        let mut var_qidep: f64 = *var_qidep_slot;
        let mut var_qidep_dn0: f64 = *var_qidep_dn0_slot;
        let mut var_qidep_dn10: f64 = *var_qidep_dn10_slot;
        let mut var_qidep_dn11: f64 = *var_qidep_dn11_slot;
        let mut var_qidep_dn12: f64 = *var_qidep_dn12_slot;
        let mut var_qidep_dn2: f64 = *var_qidep_dn2_slot;
        let mut var_qidep_dn4: f64 = *var_qidep_dn4_slot;
        let mut var_qidep_dn5: f64 = *var_qidep_dn5_slot;
        let mut var_qidep_dn6: f64 = *var_qidep_dn6_slot;
        let mut var_qidep_dn8: f64 = *var_qidep_dn8_slot;
        let mut var_qs_dep: f64 = *var_qs_dep_slot;
        let mut var_qs_dep_dn0: f64 = *var_qs_dep_dn0_slot;
        let mut var_qs_dep_dn10: f64 = *var_qs_dep_dn10_slot;
        let mut var_qs_dep_dn11: f64 = *var_qs_dep_dn11_slot;
        let mut var_qs_dep_dn12: f64 = *var_qs_dep_dn12_slot;
        let mut var_qs_dep_dn2: f64 = *var_qs_dep_dn2_slot;
        let mut var_qs_dep_dn4: f64 = *var_qs_dep_dn4_slot;
        let mut var_qs_dep_dn5: f64 = *var_qs_dep_dn5_slot;
        let mut var_qs_dep_dn6: f64 = *var_qs_dep_dn6_slot;
        let mut var_qs_dep_dn8: f64 = *var_qs_dep_dn8_slot;
        let mut var_qse: f64 = *var_qse_slot;
        let mut var_qse_dn0: f64 = *var_qse_dn0_slot;
        let mut var_qse_dn10: f64 = *var_qse_dn10_slot;
        let mut var_qse_dn11: f64 = *var_qse_dn11_slot;
        let mut var_qse_dn12: f64 = *var_qse_dn12_slot;
        let mut var_qse_dn2: f64 = *var_qse_dn2_slot;
        let mut var_qse_dn4: f64 = *var_qse_dn4_slot;
        let mut var_qse_dn5: f64 = *var_qse_dn5_slot;
        let mut var_qse_dn6: f64 = *var_qse_dn6_slot;
        let mut var_qse_dn8: f64 = *var_qse_dn8_slot;
        let mut var_qsub: f64 = *var_qsub_slot;
        let mut var_qsub_dn0: f64 = *var_qsub_dn0_slot;
        let mut var_qsub_dn10: f64 = *var_qsub_dn10_slot;
        let mut var_qsub_dn11: f64 = *var_qsub_dn11_slot;
        let mut var_qsub_dn12: f64 = *var_qsub_dn12_slot;
        let mut var_qsub_dn2: f64 = *var_qsub_dn2_slot;
        let mut var_qsub_dn4: f64 = *var_qsub_dn4_slot;
        let mut var_qsub_dn5: f64 = *var_qsub_dn5_slot;
        let mut var_qsub_dn6: f64 = *var_qsub_dn6_slot;
        let mut var_qsub_dn8: f64 = *var_qsub_dn8_slot;
        let mut var_qy: f64 = *var_qy_slot;
        let mut var_qy_dn0: f64 = *var_qy_dn0_slot;
        let mut var_qy_dn10: f64 = *var_qy_dn10_slot;
        let mut var_qy_dn11: f64 = *var_qy_dn11_slot;
        let mut var_qy_dn12: f64 = *var_qy_dn12_slot;
        let mut var_qy_dn2: f64 = *var_qy_dn2_slot;
        let mut var_qy_dn4: f64 = *var_qy_dn4_slot;
        let mut var_qy_dn5: f64 = *var_qy_dn5_slot;
        let mut var_qy_dn6: f64 = *var_qy_dn6_slot;
        let mut var_qy_dn8: f64 = *var_qy_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t10: f64 = *var_t10_slot;
        let mut var_t10_dn0: f64 = *var_t10_dn0_slot;
        let mut var_t10_dn10: f64 = *var_t10_dn10_slot;
        let mut var_t10_dn11: f64 = *var_t10_dn11_slot;
        let mut var_t10_dn12: f64 = *var_t10_dn12_slot;
        let mut var_t10_dn2: f64 = *var_t10_dn2_slot;
        let mut var_t10_dn4: f64 = *var_t10_dn4_slot;
        let mut var_t10_dn5: f64 = *var_t10_dn5_slot;
        let mut var_t10_dn6: f64 = *var_t10_dn6_slot;
        let mut var_t10_dn8: f64 = *var_t10_dn8_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;

        let (assign21550_e27130, assign21550_e27130_d_n0, assign21550_e27130_d_n2, assign21550_e27130_d_n5, assign21550_e27130_d_n6,) = {
    if (var_cgbo_given == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qgob, var_qgob_dn0, var_qgob_dn2, var_qgob_dn5, var_qgob_dn6,)
    }
};
        var_qgob = assign21550_e27130;
        var_qgob_dn0 = assign21550_e27130_d_n0;
        var_qgob_dn2 = assign21550_e27130_d_n2;
        var_qgob_dn5 = assign21550_e27130_d_n5;
        var_qgob_dn6 = assign21550_e27130_d_n6;

        var_cf = 0.0;
        var_cf_dn0 = 0.0;
        var_cf_dn2 = 0.0;
        var_cf_dn4 = 0.0;
        var_cf_dn5 = 0.0;
        var_cf_dn6 = 0.0;
        var_cf_dn8 = 0.0;
        var_cf_dn10 = 0.0;
        var_cf_dn11 = 0.0;
        var_cf_dn12 = 0.0;

        let assign21570_e27143: f64 = (var_vgse - var_vdse);
        let assign21570_e27144: f64 = (var_cf * assign21570_e27143);
        var_qfd = assign21570_e27144;
        var_qfd_dn0 = ((var_cf_dn0 * assign21570_e27143) + (var_cf * (var_vgse_dn0 - var_vdse_dn0)));
        var_qfd_dn2 = ((var_cf_dn2 * assign21570_e27143) + (var_cf * (var_vgse_dn2 - var_vdse_dn2)));
        var_qfd_dn4 = (var_cf_dn4 * assign21570_e27143);
        var_qfd_dn5 = ((var_cf_dn5 * assign21570_e27143) + (var_cf * var_vgse_dn5));
        var_qfd_dn6 = (var_cf_dn6 * assign21570_e27143);
        var_qfd_dn8 = (var_cf_dn8 * assign21570_e27143);
        var_qfd_dn10 = (var_cf_dn10 * assign21570_e27143);
        var_qfd_dn11 = (var_cf_dn11 * assign21570_e27143);
        var_qfd_dn12 = (var_cf_dn12 * assign21570_e27143);

        let assign21580_e27147: f64 = (var_cf * var_vgse);
        var_qfs = assign21580_e27147;
        var_qfs_dn0 = ((var_cf_dn0 * var_vgse) + (var_cf * var_vgse_dn0));
        var_qfs_dn2 = ((var_cf_dn2 * var_vgse) + (var_cf * var_vgse_dn2));
        var_qfs_dn4 = (var_cf_dn4 * var_vgse);
        var_qfs_dn5 = ((var_cf_dn5 * var_vgse) + (var_cf * var_vgse_dn5));
        var_qfs_dn6 = (var_cf_dn6 * var_vgse);
        var_qfs_dn8 = (var_cf_dn8 * var_vgse);
        var_qfs_dn10 = (var_cf_dn10 * var_vgse);
        var_qfs_dn11 = (var_cf_dn11 * var_vgse);
        var_qfs_dn12 = (var_cf_dn12 * var_vgse);

        let assign21590_e27150: f64 = (var_qgod + var_qfd);
        var_qgod = assign21590_e27150;
        var_qgod_dn0 = (var_qgod_dn0 + var_qfd_dn0);
        var_qgod_dn2 = (var_qgod_dn2 + var_qfd_dn2);
        var_qgod_dn4 = (var_qgod_dn4 + var_qfd_dn4);
        var_qgod_dn5 = (var_qgod_dn5 + var_qfd_dn5);
        var_qgod_dn6 = (var_qgod_dn6 + var_qfd_dn6);
        var_qgod_dn8 = (var_qgod_dn8 + var_qfd_dn8);
        var_qgod_dn10 = (var_qgod_dn10 + var_qfd_dn10);
        var_qgod_dn11 = (var_qgod_dn11 + var_qfd_dn11);
        var_qgod_dn12 = (var_qgod_dn12 + var_qfd_dn12);

        let assign21600_e27153: f64 = (var_qgos + var_qfs);
        var_qgos = assign21600_e27153;
        var_qgos_dn0 = (var_qgos_dn0 + var_qfs_dn0);
        var_qgos_dn2 = (var_qgos_dn2 + var_qfs_dn2);
        var_qgos_dn4 = (var_qgos_dn4 + var_qfs_dn4);
        var_qgos_dn5 = (var_qgos_dn5 + var_qfs_dn5);
        var_qgos_dn6 = (var_qgos_dn6 + var_qfs_dn6);
        var_qgos_dn8 = (var_qgos_dn8 + var_qfs_dn8);
        var_qgos_dn10 = (var_qgos_dn10 + var_qfs_dn10);
        var_qgos_dn11 = (var_qgos_dn11 + var_qfs_dn11);
        var_qgos_dn12 = (var_qgos_dn12 + var_qfs_dn12);

        let assign21610_e27156: f64 = (var_mfactor * var_ids);
        var_idse = assign21610_e27156;
        var_idse_dn0 = (var_mfactor * var_ids_dn0);
        var_idse_dn2 = (var_mfactor * var_ids_dn2);
        var_idse_dn4 = (var_mfactor * var_ids_dn4);
        var_idse_dn5 = (var_mfactor * var_ids_dn5);
        var_idse_dn6 = (var_mfactor * var_ids_dn6);
        var_idse_dn8 = (var_mfactor * var_ids_dn8);
        var_idse_dn10 = (var_mfactor * var_ids_dn10);
        var_idse_dn11 = (var_mfactor * var_ids_dn11);
        var_idse_dn12 = (var_mfactor * var_ids_dn12);

        let assign21620_e27158: f64 = (-var_weffcv_nf);
        let assign21620_e27160: f64 = (assign21620_e27158 * var_leff);
        var_t1 = assign21620_e27160;
        var_t1_dn0 = (((-var_weffcv_nf_dn0) * var_leff) + (assign21620_e27158 * var_leff_dn0));
        var_t1_dn2 = (((-var_weffcv_nf_dn2) * var_leff) + (assign21620_e27158 * var_leff_dn2));
        var_t1_dn4 = (((-var_weffcv_nf_dn4) * var_leff) + (assign21620_e27158 * var_leff_dn4));
        var_t1_dn5 = (((-var_weffcv_nf_dn5) * var_leff) + (assign21620_e27158 * var_leff_dn5));
        var_t1_dn6 = (((-var_weffcv_nf_dn6) * var_leff) + (assign21620_e27158 * var_leff_dn6));
        var_t1_dn8 = (((-var_weffcv_nf_dn8) * var_leff) + (assign21620_e27158 * var_leff_dn8));
        var_t1_dn10 = (((-var_weffcv_nf_dn10) * var_leff) + (assign21620_e27158 * var_leff_dn10));
        var_t1_dn11 = (((-var_weffcv_nf_dn11) * var_leff) + (assign21620_e27158 * var_leff_dn11));
        var_t1_dn12 = (((-var_weffcv_nf_dn12) * var_leff) + (assign21620_e27158 * var_leff_dn12));

        let assign21630_e27162: f64 = (-0.5);
        let assign21630_e27165: f64 = (var_q_s0_dep + var_q_sl_dep);
        let assign21630_e27166: f64 = (assign21630_e27162 * assign21630_e27165);
        var_t2 = assign21630_e27166;
        var_t2_dn0 = (assign21630_e27162 * (var_q_s0_dep_dn0 + var_q_sl_dep_dn0));
        var_t2_dn2 = (assign21630_e27162 * (var_q_s0_dep_dn2 + var_q_sl_dep_dn2));
        var_t2_dn4 = (assign21630_e27162 * (var_q_s0_dep_dn4 + var_q_sl_dep_dn4));
        var_t2_dn5 = (assign21630_e27162 * (var_q_s0_dep_dn5 + var_q_sl_dep_dn5));
        var_t2_dn6 = (assign21630_e27162 * (var_q_s0_dep_dn6 + var_q_sl_dep_dn6));
        var_t2_dn8 = (assign21630_e27162 * (var_q_s0_dep_dn8 + var_q_sl_dep_dn8));
        var_t2_dn10 = (assign21630_e27162 * (var_q_s0_dep_dn10 + var_q_sl_dep_dn10));
        var_t2_dn11 = (assign21630_e27162 * (var_q_s0_dep_dn11 + var_q_sl_dep_dn11));
        var_t2_dn12 = (assign21630_e27162 * (var_q_s0_dep_dn12 + var_q_sl_dep_dn12));

        let assign21640_e27168: f64 = (-0.5);
        let assign21640_e27171: f64 = (var_q_b0_dep + var_q_bl_dep);
        let assign21640_e27172: f64 = (assign21640_e27168 * assign21640_e27171);
        var_t3 = assign21640_e27172;
        var_t3_dn0 = (assign21640_e27168 * (var_q_b0_dep_dn0 + var_q_bl_dep_dn0));
        var_t3_dn2 = (assign21640_e27168 * (var_q_b0_dep_dn2 + var_q_bl_dep_dn2));
        var_t3_dn4 = (assign21640_e27168 * (var_q_b0_dep_dn4 + var_q_bl_dep_dn4));
        var_t3_dn5 = (assign21640_e27168 * (var_q_b0_dep_dn5 + var_q_bl_dep_dn5));
        var_t3_dn6 = (assign21640_e27168 * (var_q_b0_dep_dn6 + var_q_bl_dep_dn6));
        var_t3_dn8 = (assign21640_e27168 * (var_q_b0_dep_dn8 + var_q_bl_dep_dn8));
        var_t3_dn10 = (assign21640_e27168 * (var_q_b0_dep_dn10 + var_q_bl_dep_dn10));
        var_t3_dn11 = (assign21640_e27168 * (var_q_b0_dep_dn11 + var_q_bl_dep_dn11));
        var_t3_dn12 = (assign21640_e27168 * (var_q_b0_dep_dn12 + var_q_bl_dep_dn12));

        let assign21650_e27176: f64 = (0.1 * var_c_box);
        let assign21650_e27177: f64 = (var_t1 * assign21650_e27176);
        let assign21650_e27179: f64 = (assign21650_e27177 * var_vbse);
        var_qfs_box = assign21650_e27179;
        var_qfs_box_dn0 = (((var_t1_dn0 * assign21650_e27176) * var_vbse) + (assign21650_e27177 * var_vbse_dn0));
        var_qfs_box_dn2 = (((var_t1_dn2 * assign21650_e27176) * var_vbse) + (assign21650_e27177 * var_vbse_dn2));
        var_qfs_box_dn4 = ((var_t1_dn4 * assign21650_e27176) * var_vbse);
        var_qfs_box_dn5 = ((var_t1_dn5 * assign21650_e27176) * var_vbse);
        var_qfs_box_dn6 = (((var_t1_dn6 * assign21650_e27176) * var_vbse) + (assign21650_e27177 * var_vbse_dn6));
        var_qfs_box_dn8 = ((var_t1_dn8 * assign21650_e27176) * var_vbse);
        var_qfs_box_dn10 = ((var_t1_dn10 * assign21650_e27176) * var_vbse);
        var_qfs_box_dn11 = ((var_t1_dn11 * assign21650_e27176) * var_vbse);
        var_qfs_box_dn12 = ((var_t1_dn12 * assign21650_e27176) * var_vbse);

        let assign21660_e27183: f64 = (0.1 * var_c_box);
        let assign21660_e27184: f64 = (var_t1 * assign21660_e27183);
        let assign21660_e27187: f64 = (var_vbse - var_vdse);
        let assign21660_e27188: f64 = (assign21660_e27184 * assign21660_e27187);
        var_qfd_box = assign21660_e27188;
        var_qfd_box_dn0 = (((var_t1_dn0 * assign21660_e27183) * assign21660_e27187) + (assign21660_e27184 * (var_vbse_dn0 - var_vdse_dn0)));
        var_qfd_box_dn2 = (((var_t1_dn2 * assign21660_e27183) * assign21660_e27187) + (assign21660_e27184 * (var_vbse_dn2 - var_vdse_dn2)));
        var_qfd_box_dn4 = ((var_t1_dn4 * assign21660_e27183) * assign21660_e27187);
        var_qfd_box_dn5 = ((var_t1_dn5 * assign21660_e27183) * assign21660_e27187);
        var_qfd_box_dn6 = (((var_t1_dn6 * assign21660_e27183) * assign21660_e27187) + (assign21660_e27184 * var_vbse_dn6));
        var_qfd_box_dn8 = ((var_t1_dn8 * assign21660_e27183) * assign21660_e27187);
        var_qfd_box_dn10 = ((var_t1_dn10 * assign21660_e27183) * assign21660_e27187);
        var_qfd_box_dn11 = ((var_t1_dn11 * assign21660_e27183) * assign21660_e27187);
        var_qfd_box_dn12 = ((var_t1_dn12 * assign21660_e27183) * assign21660_e27187);

        let assign21670_e27191: f64 = (var_t1 * var_t2);
        var_qs_dep = assign21670_e27191;
        var_qs_dep_dn0 = ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0));
        var_qs_dep_dn2 = ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2));
        var_qs_dep_dn4 = ((var_t1_dn4 * var_t2) + (var_t1 * var_t2_dn4));
        var_qs_dep_dn5 = ((var_t1_dn5 * var_t2) + (var_t1 * var_t2_dn5));
        var_qs_dep_dn6 = ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6));
        var_qs_dep_dn8 = ((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8));
        var_qs_dep_dn10 = ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10));
        var_qs_dep_dn11 = ((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11));
        var_qs_dep_dn12 = ((var_t1_dn12 * var_t2) + (var_t1 * var_t2_dn12));

        let assign21680_e27194: f64 = (var_t1 * var_t3);
        var_qb_dep = assign21680_e27194;
        var_qb_dep_dn0 = ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0));
        var_qb_dep_dn2 = ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2));
        var_qb_dep_dn4 = ((var_t1_dn4 * var_t3) + (var_t1 * var_t3_dn4));
        var_qb_dep_dn5 = ((var_t1_dn5 * var_t3) + (var_t1 * var_t3_dn5));
        var_qb_dep_dn6 = ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6));
        var_qb_dep_dn8 = ((var_t1_dn8 * var_t3) + (var_t1 * var_t3_dn8));
        var_qb_dep_dn10 = ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10));
        var_qb_dep_dn11 = ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11));
        var_qb_dep_dn12 = ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12));

        let (assign21690_e27198, assign21690_e27198_d_n0, assign21690_e27198_d_n2, assign21690_e27198_d_n4, assign21690_e27198_d_n5, assign21690_e27198_d_n6, assign21690_e27198_d_n8, assign21690_e27198_d_n10, assign21690_e27198_d_n11, assign21690_e27198_d_n12,) = {
    if (p.p303 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qsub, var_qsub_dn0, var_qsub_dn2, var_qsub_dn4, var_qsub_dn5, var_qsub_dn6, var_qsub_dn8, var_qsub_dn10, var_qsub_dn11, var_qsub_dn12,)
    }
};
        var_qsub = assign21690_e27198;
        var_qsub_dn0 = assign21690_e27198_d_n0;
        var_qsub_dn2 = assign21690_e27198_d_n2;
        var_qsub_dn4 = assign21690_e27198_d_n4;
        var_qsub_dn5 = assign21690_e27198_d_n5;
        var_qsub_dn6 = assign21690_e27198_d_n6;
        var_qsub_dn8 = assign21690_e27198_d_n8;
        var_qsub_dn10 = assign21690_e27198_d_n10;
        var_qsub_dn11 = assign21690_e27198_d_n11;
        var_qsub_dn12 = assign21690_e27198_d_n12;

        let (assign21700_e27202, assign21700_e27202_d_n0, assign21700_e27202_d_n2, assign21700_e27202_d_n4, assign21700_e27202_d_n5, assign21700_e27202_d_n6, assign21700_e27202_d_n8, assign21700_e27202_d_n10, assign21700_e27202_d_n11, assign21700_e27202_d_n12,) = {
    if (p.p303 != 0.0) {
        (var_qi, var_qi_dn0, var_qi_dn2, var_qi_dn4, var_qi_dn5, var_qi_dn6, var_qi_dn8, var_qi_dn10, var_qi_dn11, var_qi_dn12,)
    } else {
        (var_qidep, var_qidep_dn0, var_qidep_dn2, var_qidep_dn4, var_qidep_dn5, var_qidep_dn6, var_qidep_dn8, var_qidep_dn10, var_qidep_dn11, var_qidep_dn12,)
    }
};
        var_qidep = assign21700_e27202;
        var_qidep_dn0 = assign21700_e27202_d_n0;
        var_qidep_dn2 = assign21700_e27202_d_n2;
        var_qidep_dn4 = assign21700_e27202_d_n4;
        var_qidep_dn5 = assign21700_e27202_d_n5;
        var_qidep_dn6 = assign21700_e27202_d_n6;
        var_qidep_dn8 = assign21700_e27202_d_n8;
        var_qidep_dn10 = assign21700_e27202_d_n10;
        var_qidep_dn11 = assign21700_e27202_d_n11;
        var_qidep_dn12 = assign21700_e27202_d_n12;

        let (assign21710_e27211, assign21710_e27211_d_n0, assign21710_e27211_d_n2, assign21710_e27211_d_n4, assign21710_e27211_d_n5, assign21710_e27211_d_n6, assign21710_e27211_d_n8, assign21710_e27211_d_n10, assign21710_e27211_d_n11, assign21710_e27211_d_n12,) = {
    if (p.p303 == 0.0) {
        let assign21710_e27207: f64 = (var_qi + var_qs_dep);
        let assign21710_e27209: f64 = (assign21710_e27207 + var_qb_dep);
        (assign21710_e27209, ((var_qi_dn0 + var_qs_dep_dn0) + var_qb_dep_dn0), ((var_qi_dn2 + var_qs_dep_dn2) + var_qb_dep_dn2), ((var_qi_dn4 + var_qs_dep_dn4) + var_qb_dep_dn4), ((var_qi_dn5 + var_qs_dep_dn5) + var_qb_dep_dn5), ((var_qi_dn6 + var_qs_dep_dn6) + var_qb_dep_dn6), ((var_qi_dn8 + var_qs_dep_dn8) + var_qb_dep_dn8), ((var_qi_dn10 + var_qs_dep_dn10) + var_qb_dep_dn10), ((var_qi_dn11 + var_qs_dep_dn11) + var_qb_dep_dn11), ((var_qi_dn12 + var_qs_dep_dn12) + var_qb_dep_dn12),)
    } else {
        (var_qidep, var_qidep_dn0, var_qidep_dn2, var_qidep_dn4, var_qidep_dn5, var_qidep_dn6, var_qidep_dn8, var_qidep_dn10, var_qidep_dn11, var_qidep_dn12,)
    }
};
        var_qidep = assign21710_e27211;
        var_qidep_dn0 = assign21710_e27211_d_n0;
        var_qidep_dn2 = assign21710_e27211_d_n2;
        var_qidep_dn4 = assign21710_e27211_d_n4;
        var_qidep_dn5 = assign21710_e27211_d_n5;
        var_qidep_dn6 = assign21710_e27211_d_n6;
        var_qidep_dn8 = assign21710_e27211_d_n8;
        var_qidep_dn10 = assign21710_e27211_d_n10;
        var_qidep_dn11 = assign21710_e27211_d_n11;
        var_qidep_dn12 = assign21710_e27211_d_n12;

        let assign21720_e27214: f64 = (var_qidep * var_qdrat);
        var_qd = assign21720_e27214;
        var_qd_dn0 = (var_qidep_dn0 * var_qdrat);
        var_qd_dn2 = (var_qidep_dn2 * var_qdrat);
        var_qd_dn4 = (var_qidep_dn4 * var_qdrat);
        var_qd_dn5 = (var_qidep_dn5 * var_qdrat);
        var_qd_dn6 = (var_qidep_dn6 * var_qdrat);
        var_qd_dn8 = (var_qidep_dn8 * var_qdrat);
        var_qd_dn10 = (var_qidep_dn10 * var_qdrat);
        var_qd_dn11 = (var_qidep_dn11 * var_qdrat);
        var_qd_dn12 = (var_qidep_dn12 * var_qdrat);

        let (assign21730_e27218, assign21730_e27218_d_n0, assign21730_e27218_d_n2, assign21730_e27218_d_n4, assign21730_e27218_d_n5, assign21730_e27218_d_n6, assign21730_e27218_d_n8, assign21730_e27218_d_n10, assign21730_e27218_d_n11, assign21730_e27218_d_n12,) = {
    if (var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn8, var_qde_dn10, var_qde_dn11, var_qde_dn12,)
    }
};
        var_qde = assign21730_e27218;
        var_qde_dn0 = assign21730_e27218_d_n0;
        var_qde_dn2 = assign21730_e27218_d_n2;
        var_qde_dn4 = assign21730_e27218_d_n4;
        var_qde_dn5 = assign21730_e27218_d_n5;
        var_qde_dn6 = assign21730_e27218_d_n6;
        var_qde_dn8 = assign21730_e27218_d_n8;
        var_qde_dn10 = assign21730_e27218_d_n10;
        var_qde_dn11 = assign21730_e27218_d_n11;
        var_qde_dn12 = assign21730_e27218_d_n12;

        let (assign21740_e27222, assign21740_e27222_d_n0, assign21740_e27222_d_n2, assign21740_e27222_d_n4, assign21740_e27222_d_n5, assign21740_e27222_d_n6, assign21740_e27222_d_n8, assign21740_e27222_d_n10, assign21740_e27222_d_n11, assign21740_e27222_d_n12,) = {
    if (var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn4, var_qge_dn5, var_qge_dn6, var_qge_dn8, var_qge_dn10, var_qge_dn11, var_qge_dn12,)
    }
};
        var_qge = assign21740_e27222;
        var_qge_dn0 = assign21740_e27222_d_n0;
        var_qge_dn2 = assign21740_e27222_d_n2;
        var_qge_dn4 = assign21740_e27222_d_n4;
        var_qge_dn5 = assign21740_e27222_d_n5;
        var_qge_dn6 = assign21740_e27222_d_n6;
        var_qge_dn8 = assign21740_e27222_d_n8;
        var_qge_dn10 = assign21740_e27222_d_n10;
        var_qge_dn11 = assign21740_e27222_d_n11;
        var_qge_dn12 = assign21740_e27222_d_n12;

        let (assign21750_e27226, assign21750_e27226_d_n0, assign21750_e27226_d_n2, assign21750_e27226_d_n4, assign21750_e27226_d_n5, assign21750_e27226_d_n6, assign21750_e27226_d_n8, assign21750_e27226_d_n10, assign21750_e27226_d_n11, assign21750_e27226_d_n12,) = {
    if (var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn8, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12,)
    }
};
        var_qbe = assign21750_e27226;
        var_qbe_dn0 = assign21750_e27226_d_n0;
        var_qbe_dn2 = assign21750_e27226_d_n2;
        var_qbe_dn4 = assign21750_e27226_d_n4;
        var_qbe_dn5 = assign21750_e27226_d_n5;
        var_qbe_dn6 = assign21750_e27226_d_n6;
        var_qbe_dn8 = assign21750_e27226_d_n8;
        var_qbe_dn10 = assign21750_e27226_d_n10;
        var_qbe_dn11 = assign21750_e27226_d_n11;
        var_qbe_dn12 = assign21750_e27226_d_n12;

        let (assign21760_e27232, assign21760_e27232_d_n0, assign21760_e27232_d_n2, assign21760_e27232_d_n4, assign21760_e27232_d_n5, assign21760_e27232_d_n6, assign21760_e27232_d_n8, assign21760_e27232_d_n10, assign21760_e27232_d_n11, assign21760_e27232_d_n12,) = {
    if (var_flg_nqs != 0.0) {
        let assign21760_e27230: f64 = (var_mfactor * var_qsub);
        (assign21760_e27230, (var_mfactor * var_qsub_dn0), (var_mfactor * var_qsub_dn2), (var_mfactor * var_qsub_dn4), (var_mfactor * var_qsub_dn5), (var_mfactor * var_qsub_dn6), (var_mfactor * var_qsub_dn8), (var_mfactor * var_qsub_dn10), (var_mfactor * var_qsub_dn11), (var_mfactor * var_qsub_dn12),)
    } else {
        (var_qb_qs, var_qb_qs_dn0, var_qb_qs_dn2, var_qb_qs_dn4, var_qb_qs_dn5, var_qb_qs_dn6, var_qb_qs_dn8, var_qb_qs_dn10, var_qb_qs_dn11, var_qb_qs_dn12,)
    }
};
        var_qb_qs = assign21760_e27232;
        var_qb_qs_dn0 = assign21760_e27232_d_n0;
        var_qb_qs_dn2 = assign21760_e27232_d_n2;
        var_qb_qs_dn4 = assign21760_e27232_d_n4;
        var_qb_qs_dn5 = assign21760_e27232_d_n5;
        var_qb_qs_dn6 = assign21760_e27232_d_n6;
        var_qb_qs_dn8 = assign21760_e27232_d_n8;
        var_qb_qs_dn10 = assign21760_e27232_d_n10;
        var_qb_qs_dn11 = assign21760_e27232_d_n11;
        var_qb_qs_dn12 = assign21760_e27232_d_n12;

        let (assign21770_e27238, assign21770_e27238_d_n0, assign21770_e27238_d_n2, assign21770_e27238_d_n4, assign21770_e27238_d_n5, assign21770_e27238_d_n6, assign21770_e27238_d_n8, assign21770_e27238_d_n10, assign21770_e27238_d_n11, assign21770_e27238_d_n12,) = {
    if (var_flg_nqs != 0.0) {
        let assign21770_e27236: f64 = (var_mfactor * var_qidep);
        (assign21770_e27236, (var_mfactor * var_qidep_dn0), (var_mfactor * var_qidep_dn2), (var_mfactor * var_qidep_dn4), (var_mfactor * var_qidep_dn5), (var_mfactor * var_qidep_dn6), (var_mfactor * var_qidep_dn8), (var_mfactor * var_qidep_dn10), (var_mfactor * var_qidep_dn11), (var_mfactor * var_qidep_dn12),)
    } else {
        (var_qi_qs, var_qi_qs_dn0, var_qi_qs_dn2, var_qi_qs_dn4, var_qi_qs_dn5, var_qi_qs_dn6, var_qi_qs_dn8, var_qi_qs_dn10, var_qi_qs_dn11, var_qi_qs_dn12,)
    }
};
        var_qi_qs = assign21770_e27238;
        var_qi_qs_dn0 = assign21770_e27238_d_n0;
        var_qi_qs_dn2 = assign21770_e27238_d_n2;
        var_qi_qs_dn4 = assign21770_e27238_d_n4;
        var_qi_qs_dn5 = assign21770_e27238_d_n5;
        var_qi_qs_dn6 = assign21770_e27238_d_n6;
        var_qi_qs_dn8 = assign21770_e27238_d_n8;
        var_qi_qs_dn10 = assign21770_e27238_d_n10;
        var_qi_qs_dn11 = assign21770_e27238_d_n11;
        var_qi_qs_dn12 = assign21770_e27238_d_n12;

        let (assign21780_e27248, assign21780_e27248_d_n0, assign21780_e27248_d_n2, assign21780_e27248_d_n4, assign21780_e27248_d_n5, assign21780_e27248_d_n6, assign21780_e27248_d_n8, assign21780_e27248_d_n10, assign21780_e27248_d_n11, assign21780_e27248_d_n12,) = {
    if (var_flg_nqs == 0.0) {
        let assign21780_e27243: f64 = (-var_qsub);
        let assign21780_e27245: f64 = (assign21780_e27243 - var_qidep);
        let assign21780_e27246: f64 = (var_mfactor * assign21780_e27245);
        (assign21780_e27246, (var_mfactor * ((-var_qsub_dn0) - var_qidep_dn0)), (var_mfactor * ((-var_qsub_dn2) - var_qidep_dn2)), (var_mfactor * ((-var_qsub_dn4) - var_qidep_dn4)), (var_mfactor * ((-var_qsub_dn5) - var_qidep_dn5)), (var_mfactor * ((-var_qsub_dn6) - var_qidep_dn6)), (var_mfactor * ((-var_qsub_dn8) - var_qidep_dn8)), (var_mfactor * ((-var_qsub_dn10) - var_qidep_dn10)), (var_mfactor * ((-var_qsub_dn11) - var_qidep_dn11)), (var_mfactor * ((-var_qsub_dn12) - var_qidep_dn12)),)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn4, var_qge_dn5, var_qge_dn6, var_qge_dn8, var_qge_dn10, var_qge_dn11, var_qge_dn12,)
    }
};
        var_qge = assign21780_e27248;
        var_qge_dn0 = assign21780_e27248_d_n0;
        var_qge_dn2 = assign21780_e27248_d_n2;
        var_qge_dn4 = assign21780_e27248_d_n4;
        var_qge_dn5 = assign21780_e27248_d_n5;
        var_qge_dn6 = assign21780_e27248_d_n6;
        var_qge_dn8 = assign21780_e27248_d_n8;
        var_qge_dn10 = assign21780_e27248_d_n10;
        var_qge_dn11 = assign21780_e27248_d_n11;
        var_qge_dn12 = assign21780_e27248_d_n12;

        let (assign21790_e27257, assign21790_e27257_d_n0, assign21790_e27257_d_n2, assign21790_e27257_d_n4, assign21790_e27257_d_n5, assign21790_e27257_d_n6, assign21790_e27257_d_n8, assign21790_e27257_d_n10, assign21790_e27257_d_n11, assign21790_e27257_d_n12,) = {
    if (var_flg_nqs == 0.0) {
        let assign21790_e27254: f64 = (var_qd + var_qfd_box);
        let assign21790_e27255: f64 = (var_mfactor * assign21790_e27254);
        (assign21790_e27255, (var_mfactor * (var_qd_dn0 + var_qfd_box_dn0)), (var_mfactor * (var_qd_dn2 + var_qfd_box_dn2)), (var_mfactor * (var_qd_dn4 + var_qfd_box_dn4)), (var_mfactor * (var_qd_dn5 + var_qfd_box_dn5)), (var_mfactor * (var_qd_dn6 + var_qfd_box_dn6)), (var_mfactor * (var_qd_dn8 + var_qfd_box_dn8)), (var_mfactor * (var_qd_dn10 + var_qfd_box_dn10)), (var_mfactor * (var_qd_dn11 + var_qfd_box_dn11)), (var_mfactor * (var_qd_dn12 + var_qfd_box_dn12)),)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn8, var_qde_dn10, var_qde_dn11, var_qde_dn12,)
    }
};
        var_qde = assign21790_e27257;
        var_qde_dn0 = assign21790_e27257_d_n0;
        var_qde_dn2 = assign21790_e27257_d_n2;
        var_qde_dn4 = assign21790_e27257_d_n4;
        var_qde_dn5 = assign21790_e27257_d_n5;
        var_qde_dn6 = assign21790_e27257_d_n6;
        var_qde_dn8 = assign21790_e27257_d_n8;
        var_qde_dn10 = assign21790_e27257_d_n10;
        var_qde_dn11 = assign21790_e27257_d_n11;
        var_qde_dn12 = assign21790_e27257_d_n12;

        let (assign21800_e27268, assign21800_e27268_d_n0, assign21800_e27268_d_n2, assign21800_e27268_d_n4, assign21800_e27268_d_n5, assign21800_e27268_d_n6, assign21800_e27268_d_n8, assign21800_e27268_d_n10, assign21800_e27268_d_n11, assign21800_e27268_d_n12,) = {
    if (var_flg_nqs == 0.0) {
        let assign21800_e27263: f64 = (var_qidep - var_qd);
        let assign21800_e27265: f64 = (assign21800_e27263 + var_qfs_box);
        let assign21800_e27266: f64 = (var_mfactor * assign21800_e27265);
        (assign21800_e27266, (var_mfactor * ((var_qidep_dn0 - var_qd_dn0) + var_qfs_box_dn0)), (var_mfactor * ((var_qidep_dn2 - var_qd_dn2) + var_qfs_box_dn2)), (var_mfactor * ((var_qidep_dn4 - var_qd_dn4) + var_qfs_box_dn4)), (var_mfactor * ((var_qidep_dn5 - var_qd_dn5) + var_qfs_box_dn5)), (var_mfactor * ((var_qidep_dn6 - var_qd_dn6) + var_qfs_box_dn6)), (var_mfactor * ((var_qidep_dn8 - var_qd_dn8) + var_qfs_box_dn8)), (var_mfactor * ((var_qidep_dn10 - var_qd_dn10) + var_qfs_box_dn10)), (var_mfactor * ((var_qidep_dn11 - var_qd_dn11) + var_qfs_box_dn11)), (var_mfactor * ((var_qidep_dn12 - var_qd_dn12) + var_qfs_box_dn12)),)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn4, var_qse_dn5, var_qse_dn6, var_qse_dn8, var_qse_dn10, var_qse_dn11, var_qse_dn12,)
    }
};
        var_qse = assign21800_e27268;
        var_qse_dn0 = assign21800_e27268_d_n0;
        var_qse_dn2 = assign21800_e27268_d_n2;
        var_qse_dn4 = assign21800_e27268_d_n4;
        var_qse_dn5 = assign21800_e27268_d_n5;
        var_qse_dn6 = assign21800_e27268_d_n6;
        var_qse_dn8 = assign21800_e27268_d_n8;
        var_qse_dn10 = assign21800_e27268_d_n10;
        var_qse_dn11 = assign21800_e27268_d_n11;
        var_qse_dn12 = assign21800_e27268_d_n12;

        let assign21810_e27271: f64 = if p.p45 == 0.0 { 1.0 } else { 0.0 };
        var_guard372 = assign21810_e27271;

        let (assign21820_e27275, assign21820_e27275_d_n0, assign21820_e27275_d_n2, assign21820_e27275_d_n4, assign21820_e27275_d_n5, assign21820_e27275_d_n6, assign21820_e27275_d_n8, assign21820_e27275_d_n10, assign21820_e27275_d_n11, assign21820_e27275_d_n12,) = {
    if (var_guard372 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qy, var_qy_dn0, var_qy_dn2, var_qy_dn4, var_qy_dn5, var_qy_dn6, var_qy_dn8, var_qy_dn10, var_qy_dn11, var_qy_dn12,)
    }
};
        var_qy = assign21820_e27275;
        var_qy_dn0 = assign21820_e27275_d_n0;
        var_qy_dn2 = assign21820_e27275_d_n2;
        var_qy_dn4 = assign21820_e27275_d_n4;
        var_qy_dn5 = assign21820_e27275_d_n5;
        var_qy_dn6 = assign21820_e27275_d_n6;
        var_qy_dn8 = assign21820_e27275_d_n8;
        var_qy_dn10 = assign21820_e27275_d_n10;
        var_qy_dn11 = assign21820_e27275_d_n11;
        var_qy_dn12 = assign21820_e27275_d_n12;

        let (assign21830_e27284, assign21830_e27284_d_n0, assign21830_e27284_d_n2, assign21830_e27284_d_n4, assign21830_e27284_d_n5, assign21830_e27284_d_n6, assign21830_e27284_d_n8, assign21830_e27284_d_n10, assign21830_e27284_d_n11, assign21830_e27284_d_n12,) = {
    if (var_guard372 == 0.0) {
        let assign21830_e27280: f64 = (var_ec * var_leff);
        let assign21830_e27282: f64 = (assign21830_e27280 + var_ps0);
        (assign21830_e27282, (((var_ec_dn0 * var_leff) + (var_ec * var_leff_dn0)) + var_ps0_dn0), (((var_ec_dn2 * var_leff) + (var_ec * var_leff_dn2)) + var_ps0_dn2), (((var_ec_dn4 * var_leff) + (var_ec * var_leff_dn4)) + var_ps0_dn4), (((var_ec_dn5 * var_leff) + (var_ec * var_leff_dn5)) + var_ps0_dn5), (((var_ec_dn6 * var_leff) + (var_ec * var_leff_dn6)) + var_ps0_dn6), (((var_ec_dn8 * var_leff) + (var_ec * var_leff_dn8)) + var_ps0_dn8), (((var_ec_dn10 * var_leff) + (var_ec * var_leff_dn10)) + var_ps0_dn10), (((var_ec_dn11 * var_leff) + (var_ec * var_leff_dn11)) + var_ps0_dn11), (((var_ec_dn12 * var_leff) + (var_ec * var_leff_dn12)) + var_ps0_dn12),)
    } else {
        (var_pslk, var_pslk_dn0, var_pslk_dn2, var_pslk_dn4, var_pslk_dn5, var_pslk_dn6, var_pslk_dn8, var_pslk_dn10, var_pslk_dn11, var_pslk_dn12,)
    }
};
        var_pslk = assign21830_e27284;
        var_pslk_dn0 = assign21830_e27284_d_n0;
        var_pslk_dn2 = assign21830_e27284_d_n2;
        var_pslk_dn4 = assign21830_e27284_d_n4;
        var_pslk_dn5 = assign21830_e27284_d_n5;
        var_pslk_dn6 = assign21830_e27284_d_n6;
        var_pslk_dn8 = assign21830_e27284_d_n8;
        var_pslk_dn10 = assign21830_e27284_d_n10;
        var_pslk_dn11 = assign21830_e27284_d_n11;
        var_pslk_dn12 = assign21830_e27284_d_n12;

        let assign21840_e27287: f64 = if var_pslk > var_psdl { 1.0 } else { 0.0 };
        var_guard373 = assign21840_e27287;

        let (assign21850_e27294, assign21850_e27294_d_n0, assign21850_e27294_d_n2, assign21850_e27294_d_n4, assign21850_e27294_d_n5, assign21850_e27294_d_n6, assign21850_e27294_d_n8, assign21850_e27294_d_n10, assign21850_e27294_d_n11, assign21850_e27294_d_n12,) = {
    if ((var_guard372 == 0.0) && (var_guard373 != 0.0)) {
        (var_psdl, var_psdl_dn0, var_psdl_dn2, var_psdl_dn4, var_psdl_dn5, var_psdl_dn6, var_psdl_dn8, var_psdl_dn10, var_psdl_dn11, var_psdl_dn12,)
    } else {
        (var_pslk, var_pslk_dn0, var_pslk_dn2, var_pslk_dn4, var_pslk_dn5, var_pslk_dn6, var_pslk_dn8, var_pslk_dn10, var_pslk_dn11, var_pslk_dn12,)
    }
};
        var_pslk = assign21850_e27294;
        var_pslk_dn0 = assign21850_e27294_d_n0;
        var_pslk_dn2 = assign21850_e27294_d_n2;
        var_pslk_dn4 = assign21850_e27294_d_n4;
        var_pslk_dn5 = assign21850_e27294_d_n5;
        var_pslk_dn6 = assign21850_e27294_d_n6;
        var_pslk_dn8 = assign21850_e27294_d_n8;
        var_pslk_dn10 = assign21850_e27294_d_n10;
        var_pslk_dn11 = assign21850_e27294_d_n11;
        var_pslk_dn12 = assign21850_e27294_d_n12;

        let (assign21860_e27309, assign21860_e27309_d_n0, assign21860_e27309_d_n2, assign21860_e27309_d_n4, assign21860_e27309_d_n5, assign21860_e27309_d_n6, assign21860_e27309_d_n8, assign21860_e27309_d_n10, assign21860_e27309_d_n11, assign21860_e27309_d_n12,) = {
    if (var_guard372 == 0.0) {
        let assign21860_e27300: f64 = (var_vds + var_ps0);
        let assign21860_e27301: f64 = (var_aclm * assign21860_e27300);
        let assign21860_e27304: f64 = (1.0 - var_aclm);
        let assign21860_e27306: f64 = (assign21860_e27304 * var_pslk);
        let assign21860_e27307: f64 = (assign21860_e27301 + assign21860_e27306);
        (assign21860_e27307, ((var_aclm * (var_vds_dn0 + var_ps0_dn0)) + (assign21860_e27304 * var_pslk_dn0)), ((var_aclm * (var_vds_dn2 + var_ps0_dn2)) + (assign21860_e27304 * var_pslk_dn2)), ((var_aclm * (var_vds_dn4 + var_ps0_dn4)) + (assign21860_e27304 * var_pslk_dn4)), ((var_aclm * (var_vds_dn5 + var_ps0_dn5)) + (assign21860_e27304 * var_pslk_dn5)), ((var_aclm * (var_vds_dn6 + var_ps0_dn6)) + (assign21860_e27304 * var_pslk_dn6)), ((var_aclm * (var_vds_dn8 + var_ps0_dn8)) + (assign21860_e27304 * var_pslk_dn8)), ((var_aclm * (var_vds_dn10 + var_ps0_dn10)) + (assign21860_e27304 * var_pslk_dn10)), ((var_aclm * (var_vds_dn11 + var_ps0_dn11)) + (assign21860_e27304 * var_pslk_dn11)), ((var_aclm * (var_vds_dn12 + var_ps0_dn12)) + (assign21860_e27304 * var_pslk_dn12)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign21860_e27309;
        var_t1_dn0 = assign21860_e27309_d_n0;
        var_t1_dn2 = assign21860_e27309_d_n2;
        var_t1_dn4 = assign21860_e27309_d_n4;
        var_t1_dn5 = assign21860_e27309_d_n5;
        var_t1_dn6 = assign21860_e27309_d_n6;
        var_t1_dn8 = assign21860_e27309_d_n8;
        var_t1_dn10 = assign21860_e27309_d_n10;
        var_t1_dn11 = assign21860_e27309_d_n11;
        var_t1_dn12 = assign21860_e27309_d_n12;

        let (assign21870_e27319, assign21870_e27319_d_n0, assign21870_e27319_d_n2, assign21870_e27319_d_n4, assign21870_e27319_d_n5, assign21870_e27319_d_n6, assign21870_e27319_d_n8, assign21870_e27319_d_n10, assign21870_e27319_d_n11, assign21870_e27319_d_n12,) = {
    if (var_guard372 == 0.0) {
        let assign21870_e27314: f64 = (2.0 * 1.034943e-10);
        let assign21870_e27316: f64 = (assign21870_e27314 / var_q_nsub);
        let assign21870_e27317: f64 = (assign21870_e27316).sqrt();
        (assign21870_e27317, ((-((assign21870_e27314 * var_q_nsub_dn0) / (var_q_nsub * var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * var_q_nsub_dn2) / (var_q_nsub * var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * var_q_nsub_dn4) / (var_q_nsub * var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * var_q_nsub_dn5) / (var_q_nsub * var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * var_q_nsub_dn6) / (var_q_nsub * var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * var_q_nsub_dn8) / (var_q_nsub * var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * var_q_nsub_dn10) / (var_q_nsub * var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * var_q_nsub_dn11) / (var_q_nsub * var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * var_q_nsub_dn12) / (var_q_nsub * var_q_nsub))) / (2.0 * assign21870_e27317)),)
    } else {
        (var_t10, var_t10_dn0, var_t10_dn2, var_t10_dn4, var_t10_dn5, var_t10_dn6, var_t10_dn8, var_t10_dn10, var_t10_dn11, var_t10_dn12,)
    }
};
        var_t10 = assign21870_e27319;
        var_t10_dn0 = assign21870_e27319_d_n0;
        var_t10_dn2 = assign21870_e27319_d_n2;
        var_t10_dn4 = assign21870_e27319_d_n4;
        var_t10_dn5 = assign21870_e27319_d_n5;
        var_t10_dn6 = assign21870_e27319_d_n6;
        var_t10_dn8 = assign21870_e27319_d_n8;
        var_t10_dn10 = assign21870_e27319_d_n10;
        var_t10_dn11 = assign21870_e27319_d_n11;
        var_t10_dn12 = assign21870_e27319_d_n12;

        *var_cf_slot = var_cf;
        *var_cf_dn0_slot = var_cf_dn0;
        *var_cf_dn10_slot = var_cf_dn10;
        *var_cf_dn11_slot = var_cf_dn11;
        *var_cf_dn12_slot = var_cf_dn12;
        *var_cf_dn2_slot = var_cf_dn2;
        *var_cf_dn4_slot = var_cf_dn4;
        *var_cf_dn5_slot = var_cf_dn5;
        *var_cf_dn6_slot = var_cf_dn6;
        *var_cf_dn8_slot = var_cf_dn8;
        *var_guard372_slot = var_guard372;
        *var_guard373_slot = var_guard373;
        *var_idse_slot = var_idse;
        *var_idse_dn0_slot = var_idse_dn0;
        *var_idse_dn10_slot = var_idse_dn10;
        *var_idse_dn11_slot = var_idse_dn11;
        *var_idse_dn12_slot = var_idse_dn12;
        *var_idse_dn2_slot = var_idse_dn2;
        *var_idse_dn4_slot = var_idse_dn4;
        *var_idse_dn5_slot = var_idse_dn5;
        *var_idse_dn6_slot = var_idse_dn6;
        *var_idse_dn8_slot = var_idse_dn8;
        *var_pslk_slot = var_pslk;
        *var_pslk_dn0_slot = var_pslk_dn0;
        *var_pslk_dn10_slot = var_pslk_dn10;
        *var_pslk_dn11_slot = var_pslk_dn11;
        *var_pslk_dn12_slot = var_pslk_dn12;
        *var_pslk_dn2_slot = var_pslk_dn2;
        *var_pslk_dn4_slot = var_pslk_dn4;
        *var_pslk_dn5_slot = var_pslk_dn5;
        *var_pslk_dn6_slot = var_pslk_dn6;
        *var_pslk_dn8_slot = var_pslk_dn8;
        *var_qb_dep_slot = var_qb_dep;
        *var_qb_dep_dn0_slot = var_qb_dep_dn0;
        *var_qb_dep_dn10_slot = var_qb_dep_dn10;
        *var_qb_dep_dn11_slot = var_qb_dep_dn11;
        *var_qb_dep_dn12_slot = var_qb_dep_dn12;
        *var_qb_dep_dn2_slot = var_qb_dep_dn2;
        *var_qb_dep_dn4_slot = var_qb_dep_dn4;
        *var_qb_dep_dn5_slot = var_qb_dep_dn5;
        *var_qb_dep_dn6_slot = var_qb_dep_dn6;
        *var_qb_dep_dn8_slot = var_qb_dep_dn8;
        *var_qb_qs_slot = var_qb_qs;
        *var_qb_qs_dn0_slot = var_qb_qs_dn0;
        *var_qb_qs_dn10_slot = var_qb_qs_dn10;
        *var_qb_qs_dn11_slot = var_qb_qs_dn11;
        *var_qb_qs_dn12_slot = var_qb_qs_dn12;
        *var_qb_qs_dn2_slot = var_qb_qs_dn2;
        *var_qb_qs_dn4_slot = var_qb_qs_dn4;
        *var_qb_qs_dn5_slot = var_qb_qs_dn5;
        *var_qb_qs_dn6_slot = var_qb_qs_dn6;
        *var_qb_qs_dn8_slot = var_qb_qs_dn8;
        *var_qbe_slot = var_qbe;
        *var_qbe_dn0_slot = var_qbe_dn0;
        *var_qbe_dn10_slot = var_qbe_dn10;
        *var_qbe_dn11_slot = var_qbe_dn11;
        *var_qbe_dn12_slot = var_qbe_dn12;
        *var_qbe_dn2_slot = var_qbe_dn2;
        *var_qbe_dn4_slot = var_qbe_dn4;
        *var_qbe_dn5_slot = var_qbe_dn5;
        *var_qbe_dn6_slot = var_qbe_dn6;
        *var_qbe_dn8_slot = var_qbe_dn8;
        *var_qd_slot = var_qd;
        *var_qd_dn0_slot = var_qd_dn0;
        *var_qd_dn10_slot = var_qd_dn10;
        *var_qd_dn11_slot = var_qd_dn11;
        *var_qd_dn12_slot = var_qd_dn12;
        *var_qd_dn2_slot = var_qd_dn2;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qde_slot = var_qde;
        *var_qde_dn0_slot = var_qde_dn0;
        *var_qde_dn10_slot = var_qde_dn10;
        *var_qde_dn11_slot = var_qde_dn11;
        *var_qde_dn12_slot = var_qde_dn12;
        *var_qde_dn2_slot = var_qde_dn2;
        *var_qde_dn4_slot = var_qde_dn4;
        *var_qde_dn5_slot = var_qde_dn5;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qde_dn8_slot = var_qde_dn8;
        *var_qfd_slot = var_qfd;
        *var_qfd_box_slot = var_qfd_box;
        *var_qfd_box_dn0_slot = var_qfd_box_dn0;
        *var_qfd_box_dn10_slot = var_qfd_box_dn10;
        *var_qfd_box_dn11_slot = var_qfd_box_dn11;
        *var_qfd_box_dn12_slot = var_qfd_box_dn12;
        *var_qfd_box_dn2_slot = var_qfd_box_dn2;
        *var_qfd_box_dn4_slot = var_qfd_box_dn4;
        *var_qfd_box_dn5_slot = var_qfd_box_dn5;
        *var_qfd_box_dn6_slot = var_qfd_box_dn6;
        *var_qfd_box_dn8_slot = var_qfd_box_dn8;
        *var_qfd_dn0_slot = var_qfd_dn0;
        *var_qfd_dn10_slot = var_qfd_dn10;
        *var_qfd_dn11_slot = var_qfd_dn11;
        *var_qfd_dn12_slot = var_qfd_dn12;
        *var_qfd_dn2_slot = var_qfd_dn2;
        *var_qfd_dn4_slot = var_qfd_dn4;
        *var_qfd_dn5_slot = var_qfd_dn5;
        *var_qfd_dn6_slot = var_qfd_dn6;
        *var_qfd_dn8_slot = var_qfd_dn8;
        *var_qfs_slot = var_qfs;
        *var_qfs_box_slot = var_qfs_box;
        *var_qfs_box_dn0_slot = var_qfs_box_dn0;
        *var_qfs_box_dn10_slot = var_qfs_box_dn10;
        *var_qfs_box_dn11_slot = var_qfs_box_dn11;
        *var_qfs_box_dn12_slot = var_qfs_box_dn12;
        *var_qfs_box_dn2_slot = var_qfs_box_dn2;
        *var_qfs_box_dn4_slot = var_qfs_box_dn4;
        *var_qfs_box_dn5_slot = var_qfs_box_dn5;
        *var_qfs_box_dn6_slot = var_qfs_box_dn6;
        *var_qfs_box_dn8_slot = var_qfs_box_dn8;
        *var_qfs_dn0_slot = var_qfs_dn0;
        *var_qfs_dn10_slot = var_qfs_dn10;
        *var_qfs_dn11_slot = var_qfs_dn11;
        *var_qfs_dn12_slot = var_qfs_dn12;
        *var_qfs_dn2_slot = var_qfs_dn2;
        *var_qfs_dn4_slot = var_qfs_dn4;
        *var_qfs_dn5_slot = var_qfs_dn5;
        *var_qfs_dn6_slot = var_qfs_dn6;
        *var_qfs_dn8_slot = var_qfs_dn8;
        *var_qge_slot = var_qge;
        *var_qge_dn0_slot = var_qge_dn0;
        *var_qge_dn10_slot = var_qge_dn10;
        *var_qge_dn11_slot = var_qge_dn11;
        *var_qge_dn12_slot = var_qge_dn12;
        *var_qge_dn2_slot = var_qge_dn2;
        *var_qge_dn4_slot = var_qge_dn4;
        *var_qge_dn5_slot = var_qge_dn5;
        *var_qge_dn6_slot = var_qge_dn6;
        *var_qge_dn8_slot = var_qge_dn8;
        *var_qgob_slot = var_qgob;
        *var_qgob_dn0_slot = var_qgob_dn0;
        *var_qgob_dn2_slot = var_qgob_dn2;
        *var_qgob_dn5_slot = var_qgob_dn5;
        *var_qgob_dn6_slot = var_qgob_dn6;
        *var_qgod_slot = var_qgod;
        *var_qgod_dn0_slot = var_qgod_dn0;
        *var_qgod_dn10_slot = var_qgod_dn10;
        *var_qgod_dn11_slot = var_qgod_dn11;
        *var_qgod_dn12_slot = var_qgod_dn12;
        *var_qgod_dn2_slot = var_qgod_dn2;
        *var_qgod_dn4_slot = var_qgod_dn4;
        *var_qgod_dn5_slot = var_qgod_dn5;
        *var_qgod_dn6_slot = var_qgod_dn6;
        *var_qgod_dn8_slot = var_qgod_dn8;
        *var_qgos_slot = var_qgos;
        *var_qgos_dn0_slot = var_qgos_dn0;
        *var_qgos_dn10_slot = var_qgos_dn10;
        *var_qgos_dn11_slot = var_qgos_dn11;
        *var_qgos_dn12_slot = var_qgos_dn12;
        *var_qgos_dn2_slot = var_qgos_dn2;
        *var_qgos_dn4_slot = var_qgos_dn4;
        *var_qgos_dn5_slot = var_qgos_dn5;
        *var_qgos_dn6_slot = var_qgos_dn6;
        *var_qgos_dn8_slot = var_qgos_dn8;
        *var_qi_qs_slot = var_qi_qs;
        *var_qi_qs_dn0_slot = var_qi_qs_dn0;
        *var_qi_qs_dn10_slot = var_qi_qs_dn10;
        *var_qi_qs_dn11_slot = var_qi_qs_dn11;
        *var_qi_qs_dn12_slot = var_qi_qs_dn12;
        *var_qi_qs_dn2_slot = var_qi_qs_dn2;
        *var_qi_qs_dn4_slot = var_qi_qs_dn4;
        *var_qi_qs_dn5_slot = var_qi_qs_dn5;
        *var_qi_qs_dn6_slot = var_qi_qs_dn6;
        *var_qi_qs_dn8_slot = var_qi_qs_dn8;
        *var_qidep_slot = var_qidep;
        *var_qidep_dn0_slot = var_qidep_dn0;
        *var_qidep_dn10_slot = var_qidep_dn10;
        *var_qidep_dn11_slot = var_qidep_dn11;
        *var_qidep_dn12_slot = var_qidep_dn12;
        *var_qidep_dn2_slot = var_qidep_dn2;
        *var_qidep_dn4_slot = var_qidep_dn4;
        *var_qidep_dn5_slot = var_qidep_dn5;
        *var_qidep_dn6_slot = var_qidep_dn6;
        *var_qidep_dn8_slot = var_qidep_dn8;
        *var_qs_dep_slot = var_qs_dep;
        *var_qs_dep_dn0_slot = var_qs_dep_dn0;
        *var_qs_dep_dn10_slot = var_qs_dep_dn10;
        *var_qs_dep_dn11_slot = var_qs_dep_dn11;
        *var_qs_dep_dn12_slot = var_qs_dep_dn12;
        *var_qs_dep_dn2_slot = var_qs_dep_dn2;
        *var_qs_dep_dn4_slot = var_qs_dep_dn4;
        *var_qs_dep_dn5_slot = var_qs_dep_dn5;
        *var_qs_dep_dn6_slot = var_qs_dep_dn6;
        *var_qs_dep_dn8_slot = var_qs_dep_dn8;
        *var_qse_slot = var_qse;
        *var_qse_dn0_slot = var_qse_dn0;
        *var_qse_dn10_slot = var_qse_dn10;
        *var_qse_dn11_slot = var_qse_dn11;
        *var_qse_dn12_slot = var_qse_dn12;
        *var_qse_dn2_slot = var_qse_dn2;
        *var_qse_dn4_slot = var_qse_dn4;
        *var_qse_dn5_slot = var_qse_dn5;
        *var_qse_dn6_slot = var_qse_dn6;
        *var_qse_dn8_slot = var_qse_dn8;
        *var_qsub_slot = var_qsub;
        *var_qsub_dn0_slot = var_qsub_dn0;
        *var_qsub_dn10_slot = var_qsub_dn10;
        *var_qsub_dn11_slot = var_qsub_dn11;
        *var_qsub_dn12_slot = var_qsub_dn12;
        *var_qsub_dn2_slot = var_qsub_dn2;
        *var_qsub_dn4_slot = var_qsub_dn4;
        *var_qsub_dn5_slot = var_qsub_dn5;
        *var_qsub_dn6_slot = var_qsub_dn6;
        *var_qsub_dn8_slot = var_qsub_dn8;
        *var_qy_slot = var_qy;
        *var_qy_dn0_slot = var_qy_dn0;
        *var_qy_dn10_slot = var_qy_dn10;
        *var_qy_dn11_slot = var_qy_dn11;
        *var_qy_dn12_slot = var_qy_dn12;
        *var_qy_dn2_slot = var_qy_dn2;
        *var_qy_dn4_slot = var_qy_dn4;
        *var_qy_dn5_slot = var_qy_dn5;
        *var_qy_dn6_slot = var_qy_dn6;
        *var_qy_dn8_slot = var_qy_dn8;
        *var_t1_slot = var_t1;
        *var_t10_slot = var_t10;
        *var_t10_dn0_slot = var_t10_dn0;
        *var_t10_dn10_slot = var_t10_dn10;
        *var_t10_dn11_slot = var_t10_dn11;
        *var_t10_dn12_slot = var_t10_dn12;
        *var_t10_dn2_slot = var_t10_dn2;
        *var_t10_dn4_slot = var_t10_dn4;
        *var_t10_dn5_slot = var_t10_dn5;
        *var_t10_dn6_slot = var_t10_dn6;
        *var_t10_dn8_slot = var_t10_dn8;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
    }

    pub(super) fn stamp_transient_block_84(
        p: &Parameters,
        var_beta_inv: f64,
        var_beta_inv_dn4: f64,
        var_c_fox: f64,
        var_c_fox_dn0: f64,
        var_c_fox_dn10: f64,
        var_c_fox_dn11: f64,
        var_c_fox_dn12: f64,
        var_c_fox_dn2: f64,
        var_c_fox_dn4: f64,
        var_c_fox_dn5: f64,
        var_c_fox_dn6: f64,
        var_c_fox_dn8: f64,
        var_cqyb0: f64,
        var_cqyb0_dn0: f64,
        var_cqyb0_dn10: f64,
        var_cqyb0_dn11: f64,
        var_cqyb0_dn12: f64,
        var_cqyb0_dn2: f64,
        var_cqyb0_dn4: f64,
        var_cqyb0_dn5: f64,
        var_cqyb0_dn6: f64,
        var_cqyb0_dn8: f64,
        var_ec: f64,
        var_ec_dn0: f64,
        var_ec_dn10: f64,
        var_ec_dn11: f64,
        var_ec_dn12: f64,
        var_ec_dn2: f64,
        var_ec_dn4: f64,
        var_ec_dn5: f64,
        var_ec_dn6: f64,
        var_ec_dn8: f64,
        var_flg_ign: f64,
        var_flg_noqi: f64,
        var_gds0_ign: f64,
        var_gds0_ign_dn0: f64,
        var_gds0_ign_dn10: f64,
        var_gds0_ign_dn11: f64,
        var_gds0_ign_dn12: f64,
        var_gds0_ign_dn2: f64,
        var_gds0_ign_dn4: f64,
        var_gds0_ign_dn5: f64,
        var_gds0_ign_dn6: f64,
        var_gds0_ign_dn8: f64,
        var_glpart1: f64,
        var_guard372: f64,
        var_igate: f64,
        var_igate_dn0: f64,
        var_igate_dn10: f64,
        var_igate_dn11: f64,
        var_igate_dn12: f64,
        var_igate_dn2: f64,
        var_igate_dn4: f64,
        var_igate_dn5: f64,
        var_igate_dn6: f64,
        var_igate_dn8: f64,
        var_igb: f64,
        var_igb_dn0: f64,
        var_igb_dn10: f64,
        var_igb_dn11: f64,
        var_igb_dn12: f64,
        var_igb_dn2: f64,
        var_igb_dn4: f64,
        var_igb_dn5: f64,
        var_igb_dn6: f64,
        var_igb_dn8: f64,
        var_igd: f64,
        var_igd_dn0: f64,
        var_igd_dn10: f64,
        var_igd_dn11: f64,
        var_igd_dn12: f64,
        var_igd_dn2: f64,
        var_igd_dn4: f64,
        var_igd_dn5: f64,
        var_igd_dn6: f64,
        var_igd_dn8: f64,
        var_igidl: f64,
        var_igidl_dn0: f64,
        var_igidl_dn10: f64,
        var_igidl_dn11: f64,
        var_igidl_dn12: f64,
        var_igidl_dn2: f64,
        var_igidl_dn4: f64,
        var_igidl_dn5: f64,
        var_igidl_dn6: f64,
        var_igidl_dn8: f64,
        var_igisl: f64,
        var_igisl_dn0: f64,
        var_igisl_dn10: f64,
        var_igisl_dn11: f64,
        var_igisl_dn12: f64,
        var_igisl_dn2: f64,
        var_igisl_dn4: f64,
        var_igisl_dn5: f64,
        var_igisl_dn6: f64,
        var_igisl_dn8: f64,
        var_igs: f64,
        var_igs_dn0: f64,
        var_igs_dn10: f64,
        var_igs_dn11: f64,
        var_igs_dn12: f64,
        var_igs_dn2: f64,
        var_igs_dn4: f64,
        var_igs_dn5: f64,
        var_igs_dn6: f64,
        var_igs_dn8: f64,
        var_isub: f64,
        var_isub_dn0: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn12: f64,
        var_isub_dn2: f64,
        var_isub_dn4: f64,
        var_isub_dn5: f64,
        var_isub_dn6: f64,
        var_isub_dn8: f64,
        var_kusai00l: f64,
        var_leff: f64,
        var_leff_dn0: f64,
        var_leff_dn10: f64,
        var_leff_dn11: f64,
        var_leff_dn12: f64,
        var_leff_dn2: f64,
        var_leff_dn4: f64,
        var_leff_dn5: f64,
        var_leff_dn6: f64,
        var_leff_dn8: f64,
        var_mfactor: f64,
        var_mode: f64,
        var_nthrml: f64,
        var_nthrml_dn0: f64,
        var_nthrml_dn10: f64,
        var_nthrml_dn11: f64,
        var_nthrml_dn12: f64,
        var_nthrml_dn2: f64,
        var_nthrml_dn4: f64,
        var_nthrml_dn5: f64,
        var_nthrml_dn6: f64,
        var_nthrml_dn8: f64,
        var_ps0: f64,
        var_ps0_dn0: f64,
        var_ps0_dn10: f64,
        var_ps0_dn11: f64,
        var_ps0_dn12: f64,
        var_ps0_dn2: f64,
        var_ps0_dn4: f64,
        var_ps0_dn5: f64,
        var_ps0_dn6: f64,
        var_ps0_dn8: f64,
        var_qbdld: f64,
        var_qbdld_dn0: f64,
        var_qbdld_dn10: f64,
        var_qbdld_dn11: f64,
        var_qbdld_dn12: f64,
        var_qbdld_dn2: f64,
        var_qbdld_dn4: f64,
        var_qbdld_dn5: f64,
        var_qbdld_dn6: f64,
        var_qbdld_dn8: f64,
        var_qbsld: f64,
        var_qbsld_dn0: f64,
        var_qbsld_dn10: f64,
        var_qbsld_dn11: f64,
        var_qbsld_dn12: f64,
        var_qbsld_dn2: f64,
        var_qbsld_dn4: f64,
        var_qbsld_dn5: f64,
        var_qbsld_dn6: f64,
        var_qbsld_dn8: f64,
        var_qgob: f64,
        var_qgob_dn0: f64,
        var_qgob_dn2: f64,
        var_qgob_dn5: f64,
        var_qgob_dn6: f64,
        var_qgod: f64,
        var_qgod_dn0: f64,
        var_qgod_dn10: f64,
        var_qgod_dn11: f64,
        var_qgod_dn12: f64,
        var_qgod_dn2: f64,
        var_qgod_dn4: f64,
        var_qgod_dn5: f64,
        var_qgod_dn6: f64,
        var_qgod_dn8: f64,
        var_qgos: f64,
        var_qgos_dn0: f64,
        var_qgos_dn10: f64,
        var_qgos_dn11: f64,
        var_qgos_dn12: f64,
        var_qgos_dn2: f64,
        var_qgos_dn4: f64,
        var_qgos_dn5: f64,
        var_qgos_dn6: f64,
        var_qgos_dn8: f64,
        var_qovd: f64,
        var_qovd_dn0: f64,
        var_qovd_dn10: f64,
        var_qovd_dn11: f64,
        var_qovd_dn12: f64,
        var_qovd_dn2: f64,
        var_qovd_dn4: f64,
        var_qovd_dn5: f64,
        var_qovd_dn6: f64,
        var_qovd_dn8: f64,
        var_qovs: f64,
        var_qovs_dn0: f64,
        var_qovs_dn10: f64,
        var_qovs_dn11: f64,
        var_qovs_dn12: f64,
        var_qovs_dn2: f64,
        var_qovs_dn4: f64,
        var_qovs_dn5: f64,
        var_qovs_dn6: f64,
        var_qovs_dn8: f64,
        var_ttemp: f64,
        var_ttemp_dn4: f64,
        var_vbs: f64,
        var_vbs_dn0: f64,
        var_vbs_dn10: f64,
        var_vbs_dn11: f64,
        var_vbs_dn12: f64,
        var_vbs_dn2: f64,
        var_vbs_dn4: f64,
        var_vbs_dn5: f64,
        var_vbs_dn6: f64,
        var_vbs_dn8: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn8: f64,
        var_weffcv_nf: f64,
        var_weffcv_nf_dn0: f64,
        var_weffcv_nf_dn10: f64,
        var_weffcv_nf_dn11: f64,
        var_weffcv_nf_dn12: f64,
        var_weffcv_nf_dn2: f64,
        var_weffcv_nf_dn4: f64,
        var_weffcv_nf_dn5: f64,
        var_weffcv_nf_dn6: f64,
        var_weffcv_nf_dn8: f64,
        var_cgdbd_slot: &mut f64,
        var_cgdbd_dn0_slot: &mut f64,
        var_cgdbd_dn10_slot: &mut f64,
        var_cgdbd_dn11_slot: &mut f64,
        var_cgdbd_dn12_slot: &mut f64,
        var_cgdbd_dn2_slot: &mut f64,
        var_cgdbd_dn4_slot: &mut f64,
        var_cgdbd_dn5_slot: &mut f64,
        var_cgdbd_dn6_slot: &mut f64,
        var_cgdbd_dn8_slot: &mut f64,
        var_cgsb_slot: &mut f64,
        var_cgsb_dn0_slot: &mut f64,
        var_cgsb_dn10_slot: &mut f64,
        var_cgsb_dn11_slot: &mut f64,
        var_cgsb_dn12_slot: &mut f64,
        var_cgsb_dn2_slot: &mut f64,
        var_cgsb_dn4_slot: &mut f64,
        var_cgsb_dn5_slot: &mut f64,
        var_cgsb_dn6_slot: &mut f64,
        var_cgsb_dn8_slot: &mut f64,
        var_cgsbd_slot: &mut f64,
        var_cgsbd_dn0_slot: &mut f64,
        var_cgsbd_dn10_slot: &mut f64,
        var_cgsbd_dn11_slot: &mut f64,
        var_cgsbd_dn12_slot: &mut f64,
        var_cgsbd_dn2_slot: &mut f64,
        var_cgsbd_dn4_slot: &mut f64,
        var_cgsbd_dn5_slot: &mut f64,
        var_cgsbd_dn6_slot: &mut f64,
        var_cgsbd_dn8_slot: &mut f64,
        var_guard374_slot: &mut f64,
        var_guard375_slot: &mut f64,
        var_guard376_slot: &mut f64,
        var_guard377_slot: &mut f64,
        var_guard378_slot: &mut f64,
        var_guard379_slot: &mut f64,
        var_igbe_slot: &mut f64,
        var_igbe_dn0_slot: &mut f64,
        var_igbe_dn10_slot: &mut f64,
        var_igbe_dn11_slot: &mut f64,
        var_igbe_dn12_slot: &mut f64,
        var_igbe_dn2_slot: &mut f64,
        var_igbe_dn4_slot: &mut f64,
        var_igbe_dn5_slot: &mut f64,
        var_igbe_dn6_slot: &mut f64,
        var_igbe_dn8_slot: &mut f64,
        var_igde_slot: &mut f64,
        var_igde_dn0_slot: &mut f64,
        var_igde_dn10_slot: &mut f64,
        var_igde_dn11_slot: &mut f64,
        var_igde_dn12_slot: &mut f64,
        var_igde_dn2_slot: &mut f64,
        var_igde_dn4_slot: &mut f64,
        var_igde_dn5_slot: &mut f64,
        var_igde_dn6_slot: &mut f64,
        var_igde_dn8_slot: &mut f64,
        var_igidle_slot: &mut f64,
        var_igidle_dn0_slot: &mut f64,
        var_igidle_dn10_slot: &mut f64,
        var_igidle_dn11_slot: &mut f64,
        var_igidle_dn12_slot: &mut f64,
        var_igidle_dn2_slot: &mut f64,
        var_igidle_dn4_slot: &mut f64,
        var_igidle_dn5_slot: &mut f64,
        var_igidle_dn6_slot: &mut f64,
        var_igidle_dn8_slot: &mut f64,
        var_igisle_slot: &mut f64,
        var_igisle_dn0_slot: &mut f64,
        var_igisle_dn10_slot: &mut f64,
        var_igisle_dn11_slot: &mut f64,
        var_igisle_dn12_slot: &mut f64,
        var_igisle_dn2_slot: &mut f64,
        var_igisle_dn4_slot: &mut f64,
        var_igisle_dn5_slot: &mut f64,
        var_igisle_dn6_slot: &mut f64,
        var_igisle_dn8_slot: &mut f64,
        var_igse_slot: &mut f64,
        var_igse_dn0_slot: &mut f64,
        var_igse_dn10_slot: &mut f64,
        var_igse_dn11_slot: &mut f64,
        var_igse_dn12_slot: &mut f64,
        var_igse_dn2_slot: &mut f64,
        var_igse_dn4_slot: &mut f64,
        var_igse_dn5_slot: &mut f64,
        var_igse_dn6_slot: &mut f64,
        var_igse_dn8_slot: &mut f64,
        var_isube_slot: &mut f64,
        var_isube_dn0_slot: &mut f64,
        var_isube_dn10_slot: &mut f64,
        var_isube_dn11_slot: &mut f64,
        var_isube_dn12_slot: &mut f64,
        var_isube_dn2_slot: &mut f64,
        var_isube_dn4_slot: &mut f64,
        var_isube_dn5_slot: &mut f64,
        var_isube_dn6_slot: &mut f64,
        var_isube_dn8_slot: &mut f64,
        var_nign0_slot: &mut f64,
        var_nign0_dn0_slot: &mut f64,
        var_nign0_dn10_slot: &mut f64,
        var_nign0_dn11_slot: &mut f64,
        var_nign0_dn12_slot: &mut f64,
        var_nign0_dn2_slot: &mut f64,
        var_nign0_dn4_slot: &mut f64,
        var_nign0_dn5_slot: &mut f64,
        var_nign0_dn6_slot: &mut f64,
        var_nign0_dn8_slot: &mut f64,
        var_noithrml_slot: &mut f64,
        var_noithrml_dn0_slot: &mut f64,
        var_noithrml_dn10_slot: &mut f64,
        var_noithrml_dn11_slot: &mut f64,
        var_noithrml_dn12_slot: &mut f64,
        var_noithrml_dn2_slot: &mut f64,
        var_noithrml_dn4_slot: &mut f64,
        var_noithrml_dn5_slot: &mut f64,
        var_noithrml_dn6_slot: &mut f64,
        var_noithrml_dn8_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_dn0_slot: &mut f64,
        var_qde_dn10_slot: &mut f64,
        var_qde_dn11_slot: &mut f64,
        var_qde_dn12_slot: &mut f64,
        var_qde_dn2_slot: &mut f64,
        var_qde_dn4_slot: &mut f64,
        var_qde_dn5_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qde_dn8_slot: &mut f64,
        var_qge_slot: &mut f64,
        var_qge_dn0_slot: &mut f64,
        var_qge_dn10_slot: &mut f64,
        var_qge_dn11_slot: &mut f64,
        var_qge_dn12_slot: &mut f64,
        var_qge_dn2_slot: &mut f64,
        var_qge_dn4_slot: &mut f64,
        var_qge_dn5_slot: &mut f64,
        var_qge_dn6_slot: &mut f64,
        var_qge_dn8_slot: &mut f64,
        var_qse_slot: &mut f64,
        var_qse_dn0_slot: &mut f64,
        var_qse_dn10_slot: &mut f64,
        var_qse_dn11_slot: &mut f64,
        var_qse_dn12_slot: &mut f64,
        var_qse_dn2_slot: &mut f64,
        var_qse_dn4_slot: &mut f64,
        var_qse_dn5_slot: &mut f64,
        var_qse_dn6_slot: &mut f64,
        var_qse_dn8_slot: &mut f64,
        var_qy_slot: &mut f64,
        var_qy_dn0_slot: &mut f64,
        var_qy_dn10_slot: &mut f64,
        var_qy_dn11_slot: &mut f64,
        var_qy_dn12_slot: &mut f64,
        var_qy_dn2_slot: &mut f64,
        var_qy_dn4_slot: &mut f64,
        var_qy_dn5_slot: &mut f64,
        var_qy_dn6_slot: &mut f64,
        var_qy_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t10_slot: &mut f64,
        var_t10_dn0_slot: &mut f64,
        var_t10_dn10_slot: &mut f64,
        var_t10_dn11_slot: &mut f64,
        var_t10_dn12_slot: &mut f64,
        var_t10_dn2_slot: &mut f64,
        var_t10_dn4_slot: &mut f64,
        var_t10_dn5_slot: &mut f64,
        var_t10_dn6_slot: &mut f64,
        var_t10_dn8_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_whi_noise_slot: &mut f64,
        var_whi_noise_dn4_slot: &mut f64,
    ) {
        let mut var_cgdbd: f64 = *var_cgdbd_slot;
        let mut var_cgdbd_dn0: f64 = *var_cgdbd_dn0_slot;
        let mut var_cgdbd_dn10: f64 = *var_cgdbd_dn10_slot;
        let mut var_cgdbd_dn11: f64 = *var_cgdbd_dn11_slot;
        let mut var_cgdbd_dn12: f64 = *var_cgdbd_dn12_slot;
        let mut var_cgdbd_dn2: f64 = *var_cgdbd_dn2_slot;
        let mut var_cgdbd_dn4: f64 = *var_cgdbd_dn4_slot;
        let mut var_cgdbd_dn5: f64 = *var_cgdbd_dn5_slot;
        let mut var_cgdbd_dn6: f64 = *var_cgdbd_dn6_slot;
        let mut var_cgdbd_dn8: f64 = *var_cgdbd_dn8_slot;
        let mut var_cgsb: f64 = *var_cgsb_slot;
        let mut var_cgsb_dn0: f64 = *var_cgsb_dn0_slot;
        let mut var_cgsb_dn10: f64 = *var_cgsb_dn10_slot;
        let mut var_cgsb_dn11: f64 = *var_cgsb_dn11_slot;
        let mut var_cgsb_dn12: f64 = *var_cgsb_dn12_slot;
        let mut var_cgsb_dn2: f64 = *var_cgsb_dn2_slot;
        let mut var_cgsb_dn4: f64 = *var_cgsb_dn4_slot;
        let mut var_cgsb_dn5: f64 = *var_cgsb_dn5_slot;
        let mut var_cgsb_dn6: f64 = *var_cgsb_dn6_slot;
        let mut var_cgsb_dn8: f64 = *var_cgsb_dn8_slot;
        let mut var_cgsbd: f64 = *var_cgsbd_slot;
        let mut var_cgsbd_dn0: f64 = *var_cgsbd_dn0_slot;
        let mut var_cgsbd_dn10: f64 = *var_cgsbd_dn10_slot;
        let mut var_cgsbd_dn11: f64 = *var_cgsbd_dn11_slot;
        let mut var_cgsbd_dn12: f64 = *var_cgsbd_dn12_slot;
        let mut var_cgsbd_dn2: f64 = *var_cgsbd_dn2_slot;
        let mut var_cgsbd_dn4: f64 = *var_cgsbd_dn4_slot;
        let mut var_cgsbd_dn5: f64 = *var_cgsbd_dn5_slot;
        let mut var_cgsbd_dn6: f64 = *var_cgsbd_dn6_slot;
        let mut var_cgsbd_dn8: f64 = *var_cgsbd_dn8_slot;
        let mut var_guard374: f64 = *var_guard374_slot;
        let mut var_guard375: f64 = *var_guard375_slot;
        let mut var_guard376: f64 = *var_guard376_slot;
        let mut var_guard377: f64 = *var_guard377_slot;
        let mut var_guard378: f64 = *var_guard378_slot;
        let mut var_guard379: f64 = *var_guard379_slot;
        let mut var_igbe: f64 = *var_igbe_slot;
        let mut var_igbe_dn0: f64 = *var_igbe_dn0_slot;
        let mut var_igbe_dn10: f64 = *var_igbe_dn10_slot;
        let mut var_igbe_dn11: f64 = *var_igbe_dn11_slot;
        let mut var_igbe_dn12: f64 = *var_igbe_dn12_slot;
        let mut var_igbe_dn2: f64 = *var_igbe_dn2_slot;
        let mut var_igbe_dn4: f64 = *var_igbe_dn4_slot;
        let mut var_igbe_dn5: f64 = *var_igbe_dn5_slot;
        let mut var_igbe_dn6: f64 = *var_igbe_dn6_slot;
        let mut var_igbe_dn8: f64 = *var_igbe_dn8_slot;
        let mut var_igde: f64 = *var_igde_slot;
        let mut var_igde_dn0: f64 = *var_igde_dn0_slot;
        let mut var_igde_dn10: f64 = *var_igde_dn10_slot;
        let mut var_igde_dn11: f64 = *var_igde_dn11_slot;
        let mut var_igde_dn12: f64 = *var_igde_dn12_slot;
        let mut var_igde_dn2: f64 = *var_igde_dn2_slot;
        let mut var_igde_dn4: f64 = *var_igde_dn4_slot;
        let mut var_igde_dn5: f64 = *var_igde_dn5_slot;
        let mut var_igde_dn6: f64 = *var_igde_dn6_slot;
        let mut var_igde_dn8: f64 = *var_igde_dn8_slot;
        let mut var_igidle: f64 = *var_igidle_slot;
        let mut var_igidle_dn0: f64 = *var_igidle_dn0_slot;
        let mut var_igidle_dn10: f64 = *var_igidle_dn10_slot;
        let mut var_igidle_dn11: f64 = *var_igidle_dn11_slot;
        let mut var_igidle_dn12: f64 = *var_igidle_dn12_slot;
        let mut var_igidle_dn2: f64 = *var_igidle_dn2_slot;
        let mut var_igidle_dn4: f64 = *var_igidle_dn4_slot;
        let mut var_igidle_dn5: f64 = *var_igidle_dn5_slot;
        let mut var_igidle_dn6: f64 = *var_igidle_dn6_slot;
        let mut var_igidle_dn8: f64 = *var_igidle_dn8_slot;
        let mut var_igisle: f64 = *var_igisle_slot;
        let mut var_igisle_dn0: f64 = *var_igisle_dn0_slot;
        let mut var_igisle_dn10: f64 = *var_igisle_dn10_slot;
        let mut var_igisle_dn11: f64 = *var_igisle_dn11_slot;
        let mut var_igisle_dn12: f64 = *var_igisle_dn12_slot;
        let mut var_igisle_dn2: f64 = *var_igisle_dn2_slot;
        let mut var_igisle_dn4: f64 = *var_igisle_dn4_slot;
        let mut var_igisle_dn5: f64 = *var_igisle_dn5_slot;
        let mut var_igisle_dn6: f64 = *var_igisle_dn6_slot;
        let mut var_igisle_dn8: f64 = *var_igisle_dn8_slot;
        let mut var_igse: f64 = *var_igse_slot;
        let mut var_igse_dn0: f64 = *var_igse_dn0_slot;
        let mut var_igse_dn10: f64 = *var_igse_dn10_slot;
        let mut var_igse_dn11: f64 = *var_igse_dn11_slot;
        let mut var_igse_dn12: f64 = *var_igse_dn12_slot;
        let mut var_igse_dn2: f64 = *var_igse_dn2_slot;
        let mut var_igse_dn4: f64 = *var_igse_dn4_slot;
        let mut var_igse_dn5: f64 = *var_igse_dn5_slot;
        let mut var_igse_dn6: f64 = *var_igse_dn6_slot;
        let mut var_igse_dn8: f64 = *var_igse_dn8_slot;
        let mut var_isube: f64 = *var_isube_slot;
        let mut var_isube_dn0: f64 = *var_isube_dn0_slot;
        let mut var_isube_dn10: f64 = *var_isube_dn10_slot;
        let mut var_isube_dn11: f64 = *var_isube_dn11_slot;
        let mut var_isube_dn12: f64 = *var_isube_dn12_slot;
        let mut var_isube_dn2: f64 = *var_isube_dn2_slot;
        let mut var_isube_dn4: f64 = *var_isube_dn4_slot;
        let mut var_isube_dn5: f64 = *var_isube_dn5_slot;
        let mut var_isube_dn6: f64 = *var_isube_dn6_slot;
        let mut var_isube_dn8: f64 = *var_isube_dn8_slot;
        let mut var_nign0: f64 = *var_nign0_slot;
        let mut var_nign0_dn0: f64 = *var_nign0_dn0_slot;
        let mut var_nign0_dn10: f64 = *var_nign0_dn10_slot;
        let mut var_nign0_dn11: f64 = *var_nign0_dn11_slot;
        let mut var_nign0_dn12: f64 = *var_nign0_dn12_slot;
        let mut var_nign0_dn2: f64 = *var_nign0_dn2_slot;
        let mut var_nign0_dn4: f64 = *var_nign0_dn4_slot;
        let mut var_nign0_dn5: f64 = *var_nign0_dn5_slot;
        let mut var_nign0_dn6: f64 = *var_nign0_dn6_slot;
        let mut var_nign0_dn8: f64 = *var_nign0_dn8_slot;
        let mut var_noithrml: f64 = *var_noithrml_slot;
        let mut var_noithrml_dn0: f64 = *var_noithrml_dn0_slot;
        let mut var_noithrml_dn10: f64 = *var_noithrml_dn10_slot;
        let mut var_noithrml_dn11: f64 = *var_noithrml_dn11_slot;
        let mut var_noithrml_dn12: f64 = *var_noithrml_dn12_slot;
        let mut var_noithrml_dn2: f64 = *var_noithrml_dn2_slot;
        let mut var_noithrml_dn4: f64 = *var_noithrml_dn4_slot;
        let mut var_noithrml_dn5: f64 = *var_noithrml_dn5_slot;
        let mut var_noithrml_dn6: f64 = *var_noithrml_dn6_slot;
        let mut var_noithrml_dn8: f64 = *var_noithrml_dn8_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_dn0: f64 = *var_qde_dn0_slot;
        let mut var_qde_dn10: f64 = *var_qde_dn10_slot;
        let mut var_qde_dn11: f64 = *var_qde_dn11_slot;
        let mut var_qde_dn12: f64 = *var_qde_dn12_slot;
        let mut var_qde_dn2: f64 = *var_qde_dn2_slot;
        let mut var_qde_dn4: f64 = *var_qde_dn4_slot;
        let mut var_qde_dn5: f64 = *var_qde_dn5_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qde_dn8: f64 = *var_qde_dn8_slot;
        let mut var_qge: f64 = *var_qge_slot;
        let mut var_qge_dn0: f64 = *var_qge_dn0_slot;
        let mut var_qge_dn10: f64 = *var_qge_dn10_slot;
        let mut var_qge_dn11: f64 = *var_qge_dn11_slot;
        let mut var_qge_dn12: f64 = *var_qge_dn12_slot;
        let mut var_qge_dn2: f64 = *var_qge_dn2_slot;
        let mut var_qge_dn4: f64 = *var_qge_dn4_slot;
        let mut var_qge_dn5: f64 = *var_qge_dn5_slot;
        let mut var_qge_dn6: f64 = *var_qge_dn6_slot;
        let mut var_qge_dn8: f64 = *var_qge_dn8_slot;
        let mut var_qse: f64 = *var_qse_slot;
        let mut var_qse_dn0: f64 = *var_qse_dn0_slot;
        let mut var_qse_dn10: f64 = *var_qse_dn10_slot;
        let mut var_qse_dn11: f64 = *var_qse_dn11_slot;
        let mut var_qse_dn12: f64 = *var_qse_dn12_slot;
        let mut var_qse_dn2: f64 = *var_qse_dn2_slot;
        let mut var_qse_dn4: f64 = *var_qse_dn4_slot;
        let mut var_qse_dn5: f64 = *var_qse_dn5_slot;
        let mut var_qse_dn6: f64 = *var_qse_dn6_slot;
        let mut var_qse_dn8: f64 = *var_qse_dn8_slot;
        let mut var_qy: f64 = *var_qy_slot;
        let mut var_qy_dn0: f64 = *var_qy_dn0_slot;
        let mut var_qy_dn10: f64 = *var_qy_dn10_slot;
        let mut var_qy_dn11: f64 = *var_qy_dn11_slot;
        let mut var_qy_dn12: f64 = *var_qy_dn12_slot;
        let mut var_qy_dn2: f64 = *var_qy_dn2_slot;
        let mut var_qy_dn4: f64 = *var_qy_dn4_slot;
        let mut var_qy_dn5: f64 = *var_qy_dn5_slot;
        let mut var_qy_dn6: f64 = *var_qy_dn6_slot;
        let mut var_qy_dn8: f64 = *var_qy_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t10: f64 = *var_t10_slot;
        let mut var_t10_dn0: f64 = *var_t10_dn0_slot;
        let mut var_t10_dn10: f64 = *var_t10_dn10_slot;
        let mut var_t10_dn11: f64 = *var_t10_dn11_slot;
        let mut var_t10_dn12: f64 = *var_t10_dn12_slot;
        let mut var_t10_dn2: f64 = *var_t10_dn2_slot;
        let mut var_t10_dn4: f64 = *var_t10_dn4_slot;
        let mut var_t10_dn5: f64 = *var_t10_dn5_slot;
        let mut var_t10_dn6: f64 = *var_t10_dn6_slot;
        let mut var_t10_dn8: f64 = *var_t10_dn8_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_whi_noise: f64 = *var_whi_noise_slot;
        let mut var_whi_noise_dn4: f64 = *var_whi_noise_dn4_slot;

        let (assign21880_e27326, assign21880_e27326_d_n0, assign21880_e27326_d_n2, assign21880_e27326_d_n4, assign21880_e27326_d_n5, assign21880_e27326_d_n6, assign21880_e27326_d_n8, assign21880_e27326_d_n10, assign21880_e27326_d_n11, assign21880_e27326_d_n12,) = {
    if (var_guard372 == 0.0) {
        let assign21880_e27324: f64 = (var_t10 * 1.3);
        (assign21880_e27324, (var_t10_dn0 * 1.3), (var_t10_dn2 * 1.3), (var_t10_dn4 * 1.3), (var_t10_dn5 * 1.3), (var_t10_dn6 * 1.3), (var_t10_dn8 * 1.3), (var_t10_dn10 * 1.3), (var_t10_dn11 * 1.3), (var_t10_dn12 * 1.3),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign21880_e27326;
        var_t3_dn0 = assign21880_e27326_d_n0;
        var_t3_dn2 = assign21880_e27326_d_n2;
        var_t3_dn4 = assign21880_e27326_d_n4;
        var_t3_dn5 = assign21880_e27326_d_n5;
        var_t3_dn6 = assign21880_e27326_d_n6;
        var_t3_dn8 = assign21880_e27326_d_n8;
        var_t3_dn10 = assign21880_e27326_d_n10;
        var_t3_dn11 = assign21880_e27326_d_n11;
        var_t3_dn12 = assign21880_e27326_d_n12;

        let (assign21890_e27335, assign21890_e27335_d_n0, assign21890_e27335_d_n2, assign21890_e27335_d_n4, assign21890_e27335_d_n5, assign21890_e27335_d_n6, assign21890_e27335_d_n8, assign21890_e27335_d_n10, assign21890_e27335_d_n11, assign21890_e27335_d_n12,) = {
    if (var_guard372 == 0.0) {
        let assign21890_e27331: f64 = (1.034943e-10 * var_weffcv_nf);
        let assign21890_e27333: f64 = (assign21890_e27331 * var_t3);
        (assign21890_e27333, (((1.034943e-10 * var_weffcv_nf_dn0) * var_t3) + (assign21890_e27331 * var_t3_dn0)), (((1.034943e-10 * var_weffcv_nf_dn2) * var_t3) + (assign21890_e27331 * var_t3_dn2)), (((1.034943e-10 * var_weffcv_nf_dn4) * var_t3) + (assign21890_e27331 * var_t3_dn4)), (((1.034943e-10 * var_weffcv_nf_dn5) * var_t3) + (assign21890_e27331 * var_t3_dn5)), (((1.034943e-10 * var_weffcv_nf_dn6) * var_t3) + (assign21890_e27331 * var_t3_dn6)), (((1.034943e-10 * var_weffcv_nf_dn8) * var_t3) + (assign21890_e27331 * var_t3_dn8)), (((1.034943e-10 * var_weffcv_nf_dn10) * var_t3) + (assign21890_e27331 * var_t3_dn10)), (((1.034943e-10 * var_weffcv_nf_dn11) * var_t3) + (assign21890_e27331 * var_t3_dn11)), (((1.034943e-10 * var_weffcv_nf_dn12) * var_t3) + (assign21890_e27331 * var_t3_dn12)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign21890_e27335;
        var_t2_dn0 = assign21890_e27335_d_n0;
        var_t2_dn2 = assign21890_e27335_d_n2;
        var_t2_dn4 = assign21890_e27335_d_n4;
        var_t2_dn5 = assign21890_e27335_d_n5;
        var_t2_dn6 = assign21890_e27335_d_n6;
        var_t2_dn8 = assign21890_e27335_d_n8;
        var_t2_dn10 = assign21890_e27335_d_n10;
        var_t2_dn11 = assign21890_e27335_d_n11;
        var_t2_dn12 = assign21890_e27335_d_n12;

        let (assign21900_e27350, assign21900_e27350_d_n0, assign21900_e27350_d_n2, assign21900_e27350_d_n4, assign21900_e27350_d_n5, assign21900_e27350_d_n6, assign21900_e27350_d_n8, assign21900_e27350_d_n10, assign21900_e27350_d_n11, assign21900_e27350_d_n12,) = {
    if (var_guard372 == 0.0) {
        let assign21900_e27340: f64 = (var_ps0 + var_vds);
        let assign21900_e27342: f64 = (assign21900_e27340 - var_t1);
        let assign21900_e27344: f64 = (assign21900_e27342 / p.p45);
        let assign21900_e27346: f64 = (assign21900_e27344 - var_ec);
        let assign21900_e27348: f64 = (assign21900_e27346 * var_t2);
        (assign21900_e27348, ((((((var_ps0_dn0 + var_vds_dn0) - var_t1_dn0) / p.p45) - var_ec_dn0) * var_t2) + (assign21900_e27346 * var_t2_dn0)), ((((((var_ps0_dn2 + var_vds_dn2) - var_t1_dn2) / p.p45) - var_ec_dn2) * var_t2) + (assign21900_e27346 * var_t2_dn2)), ((((((var_ps0_dn4 + var_vds_dn4) - var_t1_dn4) / p.p45) - var_ec_dn4) * var_t2) + (assign21900_e27346 * var_t2_dn4)), ((((((var_ps0_dn5 + var_vds_dn5) - var_t1_dn5) / p.p45) - var_ec_dn5) * var_t2) + (assign21900_e27346 * var_t2_dn5)), ((((((var_ps0_dn6 + var_vds_dn6) - var_t1_dn6) / p.p45) - var_ec_dn6) * var_t2) + (assign21900_e27346 * var_t2_dn6)), ((((((var_ps0_dn8 + var_vds_dn8) - var_t1_dn8) / p.p45) - var_ec_dn8) * var_t2) + (assign21900_e27346 * var_t2_dn8)), ((((((var_ps0_dn10 + var_vds_dn10) - var_t1_dn10) / p.p45) - var_ec_dn10) * var_t2) + (assign21900_e27346 * var_t2_dn10)), ((((((var_ps0_dn11 + var_vds_dn11) - var_t1_dn11) / p.p45) - var_ec_dn11) * var_t2) + (assign21900_e27346 * var_t2_dn11)), ((((((var_ps0_dn12 + var_vds_dn12) - var_t1_dn12) / p.p45) - var_ec_dn12) * var_t2) + (assign21900_e27346 * var_t2_dn12)),)
    } else {
        (var_qy, var_qy_dn0, var_qy_dn2, var_qy_dn4, var_qy_dn5, var_qy_dn6, var_qy_dn8, var_qy_dn10, var_qy_dn11, var_qy_dn12,)
    }
};
        var_qy = assign21900_e27350;
        var_qy_dn0 = assign21900_e27350_d_n0;
        var_qy_dn2 = assign21900_e27350_d_n2;
        var_qy_dn4 = assign21900_e27350_d_n4;
        var_qy_dn5 = assign21900_e27350_d_n5;
        var_qy_dn6 = assign21900_e27350_d_n6;
        var_qy_dn8 = assign21900_e27350_d_n8;
        var_qy_dn10 = assign21900_e27350_d_n10;
        var_qy_dn11 = assign21900_e27350_d_n11;
        var_qy_dn12 = assign21900_e27350_d_n12;

        let assign21910_e27353: f64 = if p.p46 != 0.0 { 1.0 } else { 0.0 };
        var_guard374 = assign21910_e27353;

        let (assign21920_e27361, assign21920_e27361_d_n0, assign21920_e27361_d_n2, assign21920_e27361_d_n4, assign21920_e27361_d_n5, assign21920_e27361_d_n6, assign21920_e27361_d_n8, assign21920_e27361_d_n10, assign21920_e27361_d_n11, assign21920_e27361_d_n12,) = {
    if (var_guard374 != 0.0) {
        let assign21920_e27358: f64 = (var_cqyb0 * var_vbs);
        let assign21920_e27359: f64 = (var_qy + assign21920_e27358);
        (assign21920_e27359, (var_qy_dn0 + ((var_cqyb0_dn0 * var_vbs) + (var_cqyb0 * var_vbs_dn0))), (var_qy_dn2 + ((var_cqyb0_dn2 * var_vbs) + (var_cqyb0 * var_vbs_dn2))), (var_qy_dn4 + ((var_cqyb0_dn4 * var_vbs) + (var_cqyb0 * var_vbs_dn4))), (var_qy_dn5 + ((var_cqyb0_dn5 * var_vbs) + (var_cqyb0 * var_vbs_dn5))), (var_qy_dn6 + ((var_cqyb0_dn6 * var_vbs) + (var_cqyb0 * var_vbs_dn6))), (var_qy_dn8 + ((var_cqyb0_dn8 * var_vbs) + (var_cqyb0 * var_vbs_dn8))), (var_qy_dn10 + ((var_cqyb0_dn10 * var_vbs) + (var_cqyb0 * var_vbs_dn10))), (var_qy_dn11 + ((var_cqyb0_dn11 * var_vbs) + (var_cqyb0 * var_vbs_dn11))), (var_qy_dn12 + ((var_cqyb0_dn12 * var_vbs) + (var_cqyb0 * var_vbs_dn12))),)
    } else {
        (var_qy, var_qy_dn0, var_qy_dn2, var_qy_dn4, var_qy_dn5, var_qy_dn6, var_qy_dn8, var_qy_dn10, var_qy_dn11, var_qy_dn12,)
    }
};
        var_qy = assign21920_e27361;
        var_qy_dn0 = assign21920_e27361_d_n0;
        var_qy_dn2 = assign21920_e27361_d_n2;
        var_qy_dn4 = assign21920_e27361_d_n4;
        var_qy_dn5 = assign21920_e27361_d_n5;
        var_qy_dn6 = assign21920_e27361_d_n6;
        var_qy_dn8 = assign21920_e27361_d_n8;
        var_qy_dn10 = assign21920_e27361_d_n10;
        var_qy_dn11 = assign21920_e27361_d_n11;
        var_qy_dn12 = assign21920_e27361_d_n12;

        let assign21930_e27364: f64 = if p.p14 == 1.0 { 1.0 } else { 0.0 };
        var_guard375 = assign21930_e27364;

        let (assign21940_e27382, assign21940_e27382_d_n0, assign21940_e27382_d_n2, assign21940_e27382_d_n4, assign21940_e27382_d_n5, assign21940_e27382_d_n6, assign21940_e27382_d_n8, assign21940_e27382_d_n10, assign21940_e27382_d_n11, assign21940_e27382_d_n12,) = {
    if (var_guard375 != 0.0) {
        let assign21940_e27370: f64 = (var_qgod + var_qgos);
        let assign21940_e27372: f64 = (assign21940_e27370 - var_qgob);
        let assign21940_e27374: f64 = (assign21940_e27372 - var_qy);
        let assign21940_e27376: f64 = (assign21940_e27374 - var_qovs);
        let assign21940_e27378: f64 = (assign21940_e27376 - var_qovd);
        let assign21940_e27379: f64 = (var_mfactor * assign21940_e27378);
        let assign21940_e27380: f64 = (var_qge + assign21940_e27379);
        (assign21940_e27380, (var_qge_dn0 + (var_mfactor * (((((var_qgod_dn0 + var_qgos_dn0) - var_qgob_dn0) - var_qy_dn0) - var_qovs_dn0) - var_qovd_dn0))), (var_qge_dn2 + (var_mfactor * (((((var_qgod_dn2 + var_qgos_dn2) - var_qgob_dn2) - var_qy_dn2) - var_qovs_dn2) - var_qovd_dn2))), (var_qge_dn4 + (var_mfactor * ((((var_qgod_dn4 + var_qgos_dn4) - var_qy_dn4) - var_qovs_dn4) - var_qovd_dn4))), (var_qge_dn5 + (var_mfactor * (((((var_qgod_dn5 + var_qgos_dn5) - var_qgob_dn5) - var_qy_dn5) - var_qovs_dn5) - var_qovd_dn5))), (var_qge_dn6 + (var_mfactor * (((((var_qgod_dn6 + var_qgos_dn6) - var_qgob_dn6) - var_qy_dn6) - var_qovs_dn6) - var_qovd_dn6))), (var_qge_dn8 + (var_mfactor * ((((var_qgod_dn8 + var_qgos_dn8) - var_qy_dn8) - var_qovs_dn8) - var_qovd_dn8))), (var_qge_dn10 + (var_mfactor * ((((var_qgod_dn10 + var_qgos_dn10) - var_qy_dn10) - var_qovs_dn10) - var_qovd_dn10))), (var_qge_dn11 + (var_mfactor * ((((var_qgod_dn11 + var_qgos_dn11) - var_qy_dn11) - var_qovs_dn11) - var_qovd_dn11))), (var_qge_dn12 + (var_mfactor * ((((var_qgod_dn12 + var_qgos_dn12) - var_qy_dn12) - var_qovs_dn12) - var_qovd_dn12))),)
    } else {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn4, var_qge_dn5, var_qge_dn6, var_qge_dn8, var_qge_dn10, var_qge_dn11, var_qge_dn12,)
    }
};
        var_qge = assign21940_e27382;
        var_qge_dn0 = assign21940_e27382_d_n0;
        var_qge_dn2 = assign21940_e27382_d_n2;
        var_qge_dn4 = assign21940_e27382_d_n4;
        var_qge_dn5 = assign21940_e27382_d_n5;
        var_qge_dn6 = assign21940_e27382_d_n6;
        var_qge_dn8 = assign21940_e27382_d_n8;
        var_qge_dn10 = assign21940_e27382_d_n10;
        var_qge_dn11 = assign21940_e27382_d_n11;
        var_qge_dn12 = assign21940_e27382_d_n12;

        let (assign21950_e27395, assign21950_e27395_d_n0, assign21950_e27395_d_n2, assign21950_e27395_d_n4, assign21950_e27395_d_n5, assign21950_e27395_d_n6, assign21950_e27395_d_n8, assign21950_e27395_d_n10, assign21950_e27395_d_n11, assign21950_e27395_d_n12,) = {
    if (var_guard375 != 0.0) {
        let assign21950_e27387: f64 = (-var_qgod);
        let assign21950_e27389: f64 = (assign21950_e27387 + var_qy);
        let assign21950_e27391: f64 = (assign21950_e27389 + var_qbdld);
        let assign21950_e27392: f64 = (var_mfactor * assign21950_e27391);
        let assign21950_e27393: f64 = (var_qde + assign21950_e27392);
        (assign21950_e27393, (var_qde_dn0 + (var_mfactor * (((-var_qgod_dn0) + var_qy_dn0) + var_qbdld_dn0))), (var_qde_dn2 + (var_mfactor * (((-var_qgod_dn2) + var_qy_dn2) + var_qbdld_dn2))), (var_qde_dn4 + (var_mfactor * (((-var_qgod_dn4) + var_qy_dn4) + var_qbdld_dn4))), (var_qde_dn5 + (var_mfactor * (((-var_qgod_dn5) + var_qy_dn5) + var_qbdld_dn5))), (var_qde_dn6 + (var_mfactor * (((-var_qgod_dn6) + var_qy_dn6) + var_qbdld_dn6))), (var_qde_dn8 + (var_mfactor * (((-var_qgod_dn8) + var_qy_dn8) + var_qbdld_dn8))), (var_qde_dn10 + (var_mfactor * (((-var_qgod_dn10) + var_qy_dn10) + var_qbdld_dn10))), (var_qde_dn11 + (var_mfactor * (((-var_qgod_dn11) + var_qy_dn11) + var_qbdld_dn11))), (var_qde_dn12 + (var_mfactor * (((-var_qgod_dn12) + var_qy_dn12) + var_qbdld_dn12))),)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn8, var_qde_dn10, var_qde_dn11, var_qde_dn12,)
    }
};
        var_qde = assign21950_e27395;
        var_qde_dn0 = assign21950_e27395_d_n0;
        var_qde_dn2 = assign21950_e27395_d_n2;
        var_qde_dn4 = assign21950_e27395_d_n4;
        var_qde_dn5 = assign21950_e27395_d_n5;
        var_qde_dn6 = assign21950_e27395_d_n6;
        var_qde_dn8 = assign21950_e27395_d_n8;
        var_qde_dn10 = assign21950_e27395_d_n10;
        var_qde_dn11 = assign21950_e27395_d_n11;
        var_qde_dn12 = assign21950_e27395_d_n12;

        let (assign21960_e27406, assign21960_e27406_d_n0, assign21960_e27406_d_n2, assign21960_e27406_d_n4, assign21960_e27406_d_n5, assign21960_e27406_d_n6, assign21960_e27406_d_n8, assign21960_e27406_d_n10, assign21960_e27406_d_n11, assign21960_e27406_d_n12,) = {
    if (var_guard375 != 0.0) {
        let assign21960_e27400: f64 = (-var_qgos);
        let assign21960_e27402: f64 = (assign21960_e27400 + var_qbsld);
        let assign21960_e27403: f64 = (var_mfactor * assign21960_e27402);
        let assign21960_e27404: f64 = (var_qse + assign21960_e27403);
        (assign21960_e27404, (var_qse_dn0 + (var_mfactor * ((-var_qgos_dn0) + var_qbsld_dn0))), (var_qse_dn2 + (var_mfactor * ((-var_qgos_dn2) + var_qbsld_dn2))), (var_qse_dn4 + (var_mfactor * ((-var_qgos_dn4) + var_qbsld_dn4))), (var_qse_dn5 + (var_mfactor * ((-var_qgos_dn5) + var_qbsld_dn5))), (var_qse_dn6 + (var_mfactor * ((-var_qgos_dn6) + var_qbsld_dn6))), (var_qse_dn8 + (var_mfactor * ((-var_qgos_dn8) + var_qbsld_dn8))), (var_qse_dn10 + (var_mfactor * ((-var_qgos_dn10) + var_qbsld_dn10))), (var_qse_dn11 + (var_mfactor * ((-var_qgos_dn11) + var_qbsld_dn11))), (var_qse_dn12 + (var_mfactor * ((-var_qgos_dn12) + var_qbsld_dn12))),)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn4, var_qse_dn5, var_qse_dn6, var_qse_dn8, var_qse_dn10, var_qse_dn11, var_qse_dn12,)
    }
};
        var_qse = assign21960_e27406;
        var_qse_dn0 = assign21960_e27406_d_n0;
        var_qse_dn2 = assign21960_e27406_d_n2;
        var_qse_dn4 = assign21960_e27406_d_n4;
        var_qse_dn5 = assign21960_e27406_d_n5;
        var_qse_dn6 = assign21960_e27406_d_n6;
        var_qse_dn8 = assign21960_e27406_d_n8;
        var_qse_dn10 = assign21960_e27406_d_n10;
        var_qse_dn11 = assign21960_e27406_d_n11;
        var_qse_dn12 = assign21960_e27406_d_n12;

        let assign21970_e27409: f64 = (var_mfactor * var_isub);
        var_isube = assign21970_e27409;
        var_isube_dn0 = (var_mfactor * var_isub_dn0);
        var_isube_dn2 = (var_mfactor * var_isub_dn2);
        var_isube_dn4 = (var_mfactor * var_isub_dn4);
        var_isube_dn5 = (var_mfactor * var_isub_dn5);
        var_isube_dn6 = (var_mfactor * var_isub_dn6);
        var_isube_dn8 = (var_mfactor * var_isub_dn8);
        var_isube_dn10 = (var_mfactor * var_isub_dn10);
        var_isube_dn11 = (var_mfactor * var_isub_dn11);
        var_isube_dn12 = (var_mfactor * var_isub_dn12);

        let assign22000_e27414: f64 = (-var_igb);
        let assign22000_e27415: f64 = (var_mfactor * assign22000_e27414);
        var_igbe = assign22000_e27415;
        var_igbe_dn0 = (var_mfactor * (-var_igb_dn0));
        var_igbe_dn2 = (var_mfactor * (-var_igb_dn2));
        var_igbe_dn4 = (var_mfactor * (-var_igb_dn4));
        var_igbe_dn5 = (var_mfactor * (-var_igb_dn5));
        var_igbe_dn6 = (var_mfactor * (-var_igb_dn6));
        var_igbe_dn8 = (var_mfactor * (-var_igb_dn8));
        var_igbe_dn10 = (var_mfactor * (-var_igb_dn10));
        var_igbe_dn11 = (var_mfactor * (-var_igb_dn11));
        var_igbe_dn12 = (var_mfactor * (-var_igb_dn12));

        let assign22010_e27418: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard376 = assign22010_e27418;

        let (assign22020_e27428, assign22020_e27428_d_n0, assign22020_e27428_d_n2, assign22020_e27428_d_n4, assign22020_e27428_d_n5, assign22020_e27428_d_n6, assign22020_e27428_d_n8, assign22020_e27428_d_n10, assign22020_e27428_d_n11, assign22020_e27428_d_n12,) = {
    if (var_guard376 != 0.0) {
        let assign22020_e27423: f64 = (var_glpart1 * var_igate);
        let assign22020_e27425: f64 = (assign22020_e27423 - var_igd);
        let assign22020_e27426: f64 = (var_mfactor * assign22020_e27425);
        (assign22020_e27426, (var_mfactor * ((var_glpart1 * var_igate_dn0) - var_igd_dn0)), (var_mfactor * ((var_glpart1 * var_igate_dn2) - var_igd_dn2)), (var_mfactor * ((var_glpart1 * var_igate_dn4) - var_igd_dn4)), (var_mfactor * ((var_glpart1 * var_igate_dn5) - var_igd_dn5)), (var_mfactor * ((var_glpart1 * var_igate_dn6) - var_igd_dn6)), (var_mfactor * ((var_glpart1 * var_igate_dn8) - var_igd_dn8)), (var_mfactor * ((var_glpart1 * var_igate_dn10) - var_igd_dn10)), (var_mfactor * ((var_glpart1 * var_igate_dn11) - var_igd_dn11)), (var_mfactor * ((var_glpart1 * var_igate_dn12) - var_igd_dn12)),)
    } else {
        (var_igde, var_igde_dn0, var_igde_dn2, var_igde_dn4, var_igde_dn5, var_igde_dn6, var_igde_dn8, var_igde_dn10, var_igde_dn11, var_igde_dn12,)
    }
};
        var_igde = assign22020_e27428;
        var_igde_dn0 = assign22020_e27428_d_n0;
        var_igde_dn2 = assign22020_e27428_d_n2;
        var_igde_dn4 = assign22020_e27428_d_n4;
        var_igde_dn5 = assign22020_e27428_d_n5;
        var_igde_dn6 = assign22020_e27428_d_n6;
        var_igde_dn8 = assign22020_e27428_d_n8;
        var_igde_dn10 = assign22020_e27428_d_n10;
        var_igde_dn11 = assign22020_e27428_d_n11;
        var_igde_dn12 = assign22020_e27428_d_n12;

        let (assign22030_e27435, assign22030_e27435_d_n0, assign22030_e27435_d_n2, assign22030_e27435_d_n4, assign22030_e27435_d_n5, assign22030_e27435_d_n6, assign22030_e27435_d_n8, assign22030_e27435_d_n10, assign22030_e27435_d_n11, assign22030_e27435_d_n12,) = {
    if (var_guard376 == 0.0) {
        let assign22030_e27433: f64 = (1.0 - var_glpart1);
        (assign22030_e27433, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign22030_e27435;
        var_t1_dn0 = assign22030_e27435_d_n0;
        var_t1_dn2 = assign22030_e27435_d_n2;
        var_t1_dn4 = assign22030_e27435_d_n4;
        var_t1_dn5 = assign22030_e27435_d_n5;
        var_t1_dn6 = assign22030_e27435_d_n6;
        var_t1_dn8 = assign22030_e27435_d_n8;
        var_t1_dn10 = assign22030_e27435_d_n10;
        var_t1_dn11 = assign22030_e27435_d_n11;
        var_t1_dn12 = assign22030_e27435_d_n12;

        let (assign22040_e27446, assign22040_e27446_d_n0, assign22040_e27446_d_n2, assign22040_e27446_d_n4, assign22040_e27446_d_n5, assign22040_e27446_d_n6, assign22040_e27446_d_n8, assign22040_e27446_d_n10, assign22040_e27446_d_n11, assign22040_e27446_d_n12,) = {
    if (var_guard376 == 0.0) {
        let assign22040_e27441: f64 = (var_t1 * var_igate);
        let assign22040_e27443: f64 = (assign22040_e27441 - var_igs);
        let assign22040_e27444: f64 = (var_mfactor * assign22040_e27443);
        (assign22040_e27444, (var_mfactor * (((var_t1_dn0 * var_igate) + (var_t1 * var_igate_dn0)) - var_igs_dn0)), (var_mfactor * (((var_t1_dn2 * var_igate) + (var_t1 * var_igate_dn2)) - var_igs_dn2)), (var_mfactor * (((var_t1_dn4 * var_igate) + (var_t1 * var_igate_dn4)) - var_igs_dn4)), (var_mfactor * (((var_t1_dn5 * var_igate) + (var_t1 * var_igate_dn5)) - var_igs_dn5)), (var_mfactor * (((var_t1_dn6 * var_igate) + (var_t1 * var_igate_dn6)) - var_igs_dn6)), (var_mfactor * (((var_t1_dn8 * var_igate) + (var_t1 * var_igate_dn8)) - var_igs_dn8)), (var_mfactor * (((var_t1_dn10 * var_igate) + (var_t1 * var_igate_dn10)) - var_igs_dn10)), (var_mfactor * (((var_t1_dn11 * var_igate) + (var_t1 * var_igate_dn11)) - var_igs_dn11)), (var_mfactor * (((var_t1_dn12 * var_igate) + (var_t1 * var_igate_dn12)) - var_igs_dn12)),)
    } else {
        (var_igde, var_igde_dn0, var_igde_dn2, var_igde_dn4, var_igde_dn5, var_igde_dn6, var_igde_dn8, var_igde_dn10, var_igde_dn11, var_igde_dn12,)
    }
};
        var_igde = assign22040_e27446;
        var_igde_dn0 = assign22040_e27446_d_n0;
        var_igde_dn2 = assign22040_e27446_d_n2;
        var_igde_dn4 = assign22040_e27446_d_n4;
        var_igde_dn5 = assign22040_e27446_d_n5;
        var_igde_dn6 = assign22040_e27446_d_n6;
        var_igde_dn8 = assign22040_e27446_d_n8;
        var_igde_dn10 = assign22040_e27446_d_n10;
        var_igde_dn11 = assign22040_e27446_d_n11;
        var_igde_dn12 = assign22040_e27446_d_n12;

        let assign22050_e27449: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard377 = assign22050_e27449;

        let (assign22060_e27455, assign22060_e27455_d_n0, assign22060_e27455_d_n2, assign22060_e27455_d_n4, assign22060_e27455_d_n5, assign22060_e27455_d_n6, assign22060_e27455_d_n8, assign22060_e27455_d_n10, assign22060_e27455_d_n11, assign22060_e27455_d_n12,) = {
    if (var_guard377 != 0.0) {
        let assign22060_e27453: f64 = (1.0 - var_glpart1);
        (assign22060_e27453, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign22060_e27455;
        var_t1_dn0 = assign22060_e27455_d_n0;
        var_t1_dn2 = assign22060_e27455_d_n2;
        var_t1_dn4 = assign22060_e27455_d_n4;
        var_t1_dn5 = assign22060_e27455_d_n5;
        var_t1_dn6 = assign22060_e27455_d_n6;
        var_t1_dn8 = assign22060_e27455_d_n8;
        var_t1_dn10 = assign22060_e27455_d_n10;
        var_t1_dn11 = assign22060_e27455_d_n11;
        var_t1_dn12 = assign22060_e27455_d_n12;

        let (assign22070_e27465, assign22070_e27465_d_n0, assign22070_e27465_d_n2, assign22070_e27465_d_n4, assign22070_e27465_d_n5, assign22070_e27465_d_n6, assign22070_e27465_d_n8, assign22070_e27465_d_n10, assign22070_e27465_d_n11, assign22070_e27465_d_n12,) = {
    if (var_guard377 != 0.0) {
        let assign22070_e27460: f64 = (var_t1 * var_igate);
        let assign22070_e27462: f64 = (assign22070_e27460 - var_igs);
        let assign22070_e27463: f64 = (var_mfactor * assign22070_e27462);
        (assign22070_e27463, (var_mfactor * (((var_t1_dn0 * var_igate) + (var_t1 * var_igate_dn0)) - var_igs_dn0)), (var_mfactor * (((var_t1_dn2 * var_igate) + (var_t1 * var_igate_dn2)) - var_igs_dn2)), (var_mfactor * (((var_t1_dn4 * var_igate) + (var_t1 * var_igate_dn4)) - var_igs_dn4)), (var_mfactor * (((var_t1_dn5 * var_igate) + (var_t1 * var_igate_dn5)) - var_igs_dn5)), (var_mfactor * (((var_t1_dn6 * var_igate) + (var_t1 * var_igate_dn6)) - var_igs_dn6)), (var_mfactor * (((var_t1_dn8 * var_igate) + (var_t1 * var_igate_dn8)) - var_igs_dn8)), (var_mfactor * (((var_t1_dn10 * var_igate) + (var_t1 * var_igate_dn10)) - var_igs_dn10)), (var_mfactor * (((var_t1_dn11 * var_igate) + (var_t1 * var_igate_dn11)) - var_igs_dn11)), (var_mfactor * (((var_t1_dn12 * var_igate) + (var_t1 * var_igate_dn12)) - var_igs_dn12)),)
    } else {
        (var_igse, var_igse_dn0, var_igse_dn2, var_igse_dn4, var_igse_dn5, var_igse_dn6, var_igse_dn8, var_igse_dn10, var_igse_dn11, var_igse_dn12,)
    }
};
        var_igse = assign22070_e27465;
        var_igse_dn0 = assign22070_e27465_d_n0;
        var_igse_dn2 = assign22070_e27465_d_n2;
        var_igse_dn4 = assign22070_e27465_d_n4;
        var_igse_dn5 = assign22070_e27465_d_n5;
        var_igse_dn6 = assign22070_e27465_d_n6;
        var_igse_dn8 = assign22070_e27465_d_n8;
        var_igse_dn10 = assign22070_e27465_d_n10;
        var_igse_dn11 = assign22070_e27465_d_n11;
        var_igse_dn12 = assign22070_e27465_d_n12;

        let (assign22080_e27476, assign22080_e27476_d_n0, assign22080_e27476_d_n2, assign22080_e27476_d_n4, assign22080_e27476_d_n5, assign22080_e27476_d_n6, assign22080_e27476_d_n8, assign22080_e27476_d_n10, assign22080_e27476_d_n11, assign22080_e27476_d_n12,) = {
    if (var_guard377 == 0.0) {
        let assign22080_e27471: f64 = (var_glpart1 * var_igate);
        let assign22080_e27473: f64 = (assign22080_e27471 - var_igd);
        let assign22080_e27474: f64 = (var_mfactor * assign22080_e27473);
        (assign22080_e27474, (var_mfactor * ((var_glpart1 * var_igate_dn0) - var_igd_dn0)), (var_mfactor * ((var_glpart1 * var_igate_dn2) - var_igd_dn2)), (var_mfactor * ((var_glpart1 * var_igate_dn4) - var_igd_dn4)), (var_mfactor * ((var_glpart1 * var_igate_dn5) - var_igd_dn5)), (var_mfactor * ((var_glpart1 * var_igate_dn6) - var_igd_dn6)), (var_mfactor * ((var_glpart1 * var_igate_dn8) - var_igd_dn8)), (var_mfactor * ((var_glpart1 * var_igate_dn10) - var_igd_dn10)), (var_mfactor * ((var_glpart1 * var_igate_dn11) - var_igd_dn11)), (var_mfactor * ((var_glpart1 * var_igate_dn12) - var_igd_dn12)),)
    } else {
        (var_igse, var_igse_dn0, var_igse_dn2, var_igse_dn4, var_igse_dn5, var_igse_dn6, var_igse_dn8, var_igse_dn10, var_igse_dn11, var_igse_dn12,)
    }
};
        var_igse = assign22080_e27476;
        var_igse_dn0 = assign22080_e27476_d_n0;
        var_igse_dn2 = assign22080_e27476_d_n2;
        var_igse_dn4 = assign22080_e27476_d_n4;
        var_igse_dn5 = assign22080_e27476_d_n5;
        var_igse_dn6 = assign22080_e27476_d_n6;
        var_igse_dn8 = assign22080_e27476_d_n8;
        var_igse_dn10 = assign22080_e27476_d_n10;
        var_igse_dn11 = assign22080_e27476_d_n11;
        var_igse_dn12 = assign22080_e27476_d_n12;

        let (assign22090_e27486, assign22090_e27486_d_n0, assign22090_e27486_d_n2, assign22090_e27486_d_n4, assign22090_e27486_d_n5, assign22090_e27486_d_n6, assign22090_e27486_d_n8, assign22090_e27486_d_n10, assign22090_e27486_d_n11, assign22090_e27486_d_n12,) = {
    if (var_mode == 1.0) {
        let assign22090_e27482: f64 = (var_mfactor * var_igidl);
        (assign22090_e27482, (var_mfactor * var_igidl_dn0), (var_mfactor * var_igidl_dn2), (var_mfactor * var_igidl_dn4), (var_mfactor * var_igidl_dn5), (var_mfactor * var_igidl_dn6), (var_mfactor * var_igidl_dn8), (var_mfactor * var_igidl_dn10), (var_mfactor * var_igidl_dn11), (var_mfactor * var_igidl_dn12),)
    } else {
        let assign22090_e27485: f64 = (var_mfactor * var_igisl);
        (assign22090_e27485, (var_mfactor * var_igisl_dn0), (var_mfactor * var_igisl_dn2), (var_mfactor * var_igisl_dn4), (var_mfactor * var_igisl_dn5), (var_mfactor * var_igisl_dn6), (var_mfactor * var_igisl_dn8), (var_mfactor * var_igisl_dn10), (var_mfactor * var_igisl_dn11), (var_mfactor * var_igisl_dn12),)
    }
};
        var_igidle = assign22090_e27486;
        var_igidle_dn0 = assign22090_e27486_d_n0;
        var_igidle_dn2 = assign22090_e27486_d_n2;
        var_igidle_dn4 = assign22090_e27486_d_n4;
        var_igidle_dn5 = assign22090_e27486_d_n5;
        var_igidle_dn6 = assign22090_e27486_d_n6;
        var_igidle_dn8 = assign22090_e27486_d_n8;
        var_igidle_dn10 = assign22090_e27486_d_n10;
        var_igidle_dn11 = assign22090_e27486_d_n11;
        var_igidle_dn12 = assign22090_e27486_d_n12;

        let (assign22100_e27496, assign22100_e27496_d_n0, assign22100_e27496_d_n2, assign22100_e27496_d_n4, assign22100_e27496_d_n5, assign22100_e27496_d_n6, assign22100_e27496_d_n8, assign22100_e27496_d_n10, assign22100_e27496_d_n11, assign22100_e27496_d_n12,) = {
    if (var_mode == 1.0) {
        let assign22100_e27492: f64 = (var_mfactor * var_igisl);
        (assign22100_e27492, (var_mfactor * var_igisl_dn0), (var_mfactor * var_igisl_dn2), (var_mfactor * var_igisl_dn4), (var_mfactor * var_igisl_dn5), (var_mfactor * var_igisl_dn6), (var_mfactor * var_igisl_dn8), (var_mfactor * var_igisl_dn10), (var_mfactor * var_igisl_dn11), (var_mfactor * var_igisl_dn12),)
    } else {
        let assign22100_e27495: f64 = (var_mfactor * var_igidl);
        (assign22100_e27495, (var_mfactor * var_igidl_dn0), (var_mfactor * var_igidl_dn2), (var_mfactor * var_igidl_dn4), (var_mfactor * var_igidl_dn5), (var_mfactor * var_igidl_dn6), (var_mfactor * var_igidl_dn8), (var_mfactor * var_igidl_dn10), (var_mfactor * var_igidl_dn11), (var_mfactor * var_igidl_dn12),)
    }
};
        var_igisle = assign22100_e27496;
        var_igisle_dn0 = assign22100_e27496_d_n0;
        var_igisle_dn2 = assign22100_e27496_d_n2;
        var_igisle_dn4 = assign22100_e27496_d_n4;
        var_igisle_dn5 = assign22100_e27496_d_n5;
        var_igisle_dn6 = assign22100_e27496_d_n6;
        var_igisle_dn8 = assign22100_e27496_d_n8;
        var_igisle_dn10 = assign22100_e27496_d_n10;
        var_igisle_dn11 = assign22100_e27496_d_n11;
        var_igisle_dn12 = assign22100_e27496_d_n12;

        let assign22110_e27499: f64 = (4.0 * 1.3806226e-23);
        let assign22110_e27501: f64 = (assign22110_e27499 * var_ttemp);
        let assign22110_e27503: f64 = assign22110_e27501;
        var_whi_noise = assign22110_e27503;
        var_whi_noise_dn4 = (assign22110_e27499 * var_ttemp_dn4);

        let assign22130_e27509: f64 = (var_mfactor * var_nthrml);
        var_noithrml = assign22130_e27509;
        var_noithrml_dn0 = (var_mfactor * var_nthrml_dn0);
        var_noithrml_dn2 = (var_mfactor * var_nthrml_dn2);
        var_noithrml_dn4 = (var_mfactor * var_nthrml_dn4);
        var_noithrml_dn5 = (var_mfactor * var_nthrml_dn5);
        var_noithrml_dn6 = (var_mfactor * var_nthrml_dn6);
        var_noithrml_dn8 = (var_mfactor * var_nthrml_dn8);
        var_noithrml_dn10 = (var_mfactor * var_nthrml_dn10);
        var_noithrml_dn11 = (var_mfactor * var_nthrml_dn11);
        var_noithrml_dn12 = (var_mfactor * var_nthrml_dn12);

        let assign22140_e27512: f64 = var_qge_dn11;
        var_cgdbd = assign22140_e27512;
        var_cgdbd_dn0 = 0.0;
        var_cgdbd_dn2 = 0.0;
        var_cgdbd_dn4 = 0.0;
        var_cgdbd_dn5 = 0.0;
        var_cgdbd_dn6 = 0.0;
        var_cgdbd_dn8 = 0.0;
        var_cgdbd_dn10 = 0.0;
        var_cgdbd_dn11 = 0.0;
        var_cgdbd_dn12 = 0.0;

        let assign22150_e27515: f64 = (p.p33 * var_cgdbd);
        var_cgdbd = assign22150_e27515;
        var_cgdbd_dn0 = (p.p33 * var_cgdbd_dn0);
        var_cgdbd_dn2 = (p.p33 * var_cgdbd_dn2);
        var_cgdbd_dn4 = (p.p33 * var_cgdbd_dn4);
        var_cgdbd_dn5 = (p.p33 * var_cgdbd_dn5);
        var_cgdbd_dn6 = (p.p33 * var_cgdbd_dn6);
        var_cgdbd_dn8 = (p.p33 * var_cgdbd_dn8);
        var_cgdbd_dn10 = (p.p33 * var_cgdbd_dn10);
        var_cgdbd_dn11 = (p.p33 * var_cgdbd_dn11);
        var_cgdbd_dn12 = (p.p33 * var_cgdbd_dn12);

        let assign22160_e27518: f64 = var_qge_dn12;
        var_cgsbd = assign22160_e27518;
        var_cgsbd_dn0 = 0.0;
        var_cgsbd_dn2 = 0.0;
        var_cgsbd_dn4 = 0.0;
        var_cgsbd_dn5 = 0.0;
        var_cgsbd_dn6 = 0.0;
        var_cgsbd_dn8 = 0.0;
        var_cgsbd_dn10 = 0.0;
        var_cgsbd_dn11 = 0.0;
        var_cgsbd_dn12 = 0.0;

        let assign22170_e27521: f64 = (p.p33 * var_cgsbd);
        var_cgsbd = assign22170_e27521;
        var_cgsbd_dn0 = (p.p33 * var_cgsbd_dn0);
        var_cgsbd_dn2 = (p.p33 * var_cgsbd_dn2);
        var_cgsbd_dn4 = (p.p33 * var_cgsbd_dn4);
        var_cgsbd_dn5 = (p.p33 * var_cgsbd_dn5);
        var_cgsbd_dn6 = (p.p33 * var_cgsbd_dn6);
        var_cgsbd_dn8 = (p.p33 * var_cgsbd_dn8);
        var_cgsbd_dn10 = (p.p33 * var_cgsbd_dn10);
        var_cgsbd_dn11 = (p.p33 * var_cgsbd_dn11);
        var_cgsbd_dn12 = (p.p33 * var_cgsbd_dn12);

        let (assign22180_e27527, assign22180_e27527_d_n0, assign22180_e27527_d_n2, assign22180_e27527_d_n4, assign22180_e27527_d_n5, assign22180_e27527_d_n6, assign22180_e27527_d_n8, assign22180_e27527_d_n10, assign22180_e27527_d_n11, assign22180_e27527_d_n12,) = {
    if (var_mode > 0.0) {
        (var_cgsbd, var_cgsbd_dn0, var_cgsbd_dn2, var_cgsbd_dn4, var_cgsbd_dn5, var_cgsbd_dn6, var_cgsbd_dn8, var_cgsbd_dn10, var_cgsbd_dn11, var_cgsbd_dn12,)
    } else {
        (var_cgdbd, var_cgdbd_dn0, var_cgdbd_dn2, var_cgdbd_dn4, var_cgdbd_dn5, var_cgdbd_dn6, var_cgdbd_dn8, var_cgdbd_dn10, var_cgdbd_dn11, var_cgdbd_dn12,)
    }
};
        var_cgsb = assign22180_e27527;
        var_cgsb_dn0 = assign22180_e27527_d_n0;
        var_cgsb_dn2 = assign22180_e27527_d_n2;
        var_cgsb_dn4 = assign22180_e27527_d_n4;
        var_cgsb_dn5 = assign22180_e27527_d_n5;
        var_cgsb_dn6 = assign22180_e27527_d_n6;
        var_cgsb_dn8 = assign22180_e27527_d_n8;
        var_cgsb_dn10 = assign22180_e27527_d_n10;
        var_cgsb_dn11 = assign22180_e27527_d_n11;
        var_cgsb_dn12 = assign22180_e27527_d_n12;

        let assign22190_e27541: f64 = if ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (var_flg_ign == 1.0)) && (var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        var_guard378 = assign22190_e27541;

        let (assign22200_e27551, assign22200_e27551_d_n0, assign22200_e27551_d_n2, assign22200_e27551_d_n4, assign22200_e27551_d_n5, assign22200_e27551_d_n6, assign22200_e27551_d_n8, assign22200_e27551_d_n10, assign22200_e27551_d_n11, assign22200_e27551_d_n12,) = {
    if (var_guard378 != 0.0) {
        let assign22200_e27545: f64 = (1e-6 * var_c_fox);
        let assign22200_e27547: f64 = (assign22200_e27545 * var_weffcv_nf);
        let assign22200_e27549: f64 = (assign22200_e27547 * var_leff);
        (assign22200_e27549, (((((1e-6 * var_c_fox_dn0) * var_weffcv_nf) + (assign22200_e27545 * var_weffcv_nf_dn0)) * var_leff) + (assign22200_e27547 * var_leff_dn0)), (((((1e-6 * var_c_fox_dn2) * var_weffcv_nf) + (assign22200_e27545 * var_weffcv_nf_dn2)) * var_leff) + (assign22200_e27547 * var_leff_dn2)), (((((1e-6 * var_c_fox_dn4) * var_weffcv_nf) + (assign22200_e27545 * var_weffcv_nf_dn4)) * var_leff) + (assign22200_e27547 * var_leff_dn4)), (((((1e-6 * var_c_fox_dn5) * var_weffcv_nf) + (assign22200_e27545 * var_weffcv_nf_dn5)) * var_leff) + (assign22200_e27547 * var_leff_dn5)), (((((1e-6 * var_c_fox_dn6) * var_weffcv_nf) + (assign22200_e27545 * var_weffcv_nf_dn6)) * var_leff) + (assign22200_e27547 * var_leff_dn6)), (((((1e-6 * var_c_fox_dn8) * var_weffcv_nf) + (assign22200_e27545 * var_weffcv_nf_dn8)) * var_leff) + (assign22200_e27547 * var_leff_dn8)), (((((1e-6 * var_c_fox_dn10) * var_weffcv_nf) + (assign22200_e27545 * var_weffcv_nf_dn10)) * var_leff) + (assign22200_e27547 * var_leff_dn10)), (((((1e-6 * var_c_fox_dn11) * var_weffcv_nf) + (assign22200_e27545 * var_weffcv_nf_dn11)) * var_leff) + (assign22200_e27547 * var_leff_dn11)), (((((1e-6 * var_c_fox_dn12) * var_weffcv_nf) + (assign22200_e27545 * var_weffcv_nf_dn12)) * var_leff) + (assign22200_e27547 * var_leff_dn12)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
        var_t0 = assign22200_e27551;
        var_t0_dn0 = assign22200_e27551_d_n0;
        var_t0_dn2 = assign22200_e27551_d_n2;
        var_t0_dn4 = assign22200_e27551_d_n4;
        var_t0_dn5 = assign22200_e27551_d_n5;
        var_t0_dn6 = assign22200_e27551_d_n6;
        var_t0_dn8 = assign22200_e27551_d_n8;
        var_t0_dn10 = assign22200_e27551_d_n10;
        var_t0_dn11 = assign22200_e27551_d_n11;
        var_t0_dn12 = assign22200_e27551_d_n12;

        let (assign22210_e27557, assign22210_e27557_d_n0, assign22210_e27557_d_n2, assign22210_e27557_d_n4, assign22210_e27557_d_n5, assign22210_e27557_d_n6, assign22210_e27557_d_n8, assign22210_e27557_d_n10, assign22210_e27557_d_n11, assign22210_e27557_d_n12,) = {
    if (var_guard378 != 0.0) {
        let assign22210_e27555: f64 = (var_cgsb / var_mfactor);
        (assign22210_e27555, (var_cgsb_dn0 / var_mfactor), (var_cgsb_dn2 / var_mfactor), (var_cgsb_dn4 / var_mfactor), (var_cgsb_dn5 / var_mfactor), (var_cgsb_dn6 / var_mfactor), (var_cgsb_dn8 / var_mfactor), (var_cgsb_dn10 / var_mfactor), (var_cgsb_dn11 / var_mfactor), (var_cgsb_dn12 / var_mfactor),)
    } else {
        (var_t10, var_t10_dn0, var_t10_dn2, var_t10_dn4, var_t10_dn5, var_t10_dn6, var_t10_dn8, var_t10_dn10, var_t10_dn11, var_t10_dn12,)
    }
};
        var_t10 = assign22210_e27557;
        var_t10_dn0 = assign22210_e27557_d_n0;
        var_t10_dn2 = assign22210_e27557_d_n2;
        var_t10_dn4 = assign22210_e27557_d_n4;
        var_t10_dn5 = assign22210_e27557_d_n5;
        var_t10_dn6 = assign22210_e27557_d_n6;
        var_t10_dn8 = assign22210_e27557_d_n8;
        var_t10_dn10 = assign22210_e27557_d_n10;
        var_t10_dn11 = assign22210_e27557_d_n11;
        var_t10_dn12 = assign22210_e27557_d_n12;

        let (assign22220_e27571, assign22220_e27571_d_n0, assign22220_e27571_d_n2, assign22220_e27571_d_n4, assign22220_e27571_d_n5, assign22220_e27571_d_n6, assign22220_e27571_d_n8, assign22220_e27571_d_n10, assign22220_e27571_d_n11, assign22220_e27571_d_n12,) = {
    if (var_guard378 != 0.0) {
        let assign22220_e27561: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign22220_e27563: f64 = (assign22220_e27561 * var_beta_inv);
        let assign22220_e27565: f64 = (assign22220_e27563 * var_t10);
        let assign22220_e27567: f64 = (assign22220_e27565 * var_t10);
        let assign22220_e27569: f64 = (assign22220_e27567 / var_gds0_ign);
        (assign22220_e27569, ((((((assign22220_e27563 * var_t10_dn0) * var_t10) + (assign22220_e27565 * var_t10_dn0)) * var_gds0_ign) - (assign22220_e27567 * var_gds0_ign_dn0)) / (var_gds0_ign * var_gds0_ign)), ((((((assign22220_e27563 * var_t10_dn2) * var_t10) + (assign22220_e27565 * var_t10_dn2)) * var_gds0_ign) - (assign22220_e27567 * var_gds0_ign_dn2)) / (var_gds0_ign * var_gds0_ign)), ((((((((assign22220_e27561 * var_beta_inv_dn4) * var_t10) + (assign22220_e27563 * var_t10_dn4)) * var_t10) + (assign22220_e27565 * var_t10_dn4)) * var_gds0_ign) - (assign22220_e27567 * var_gds0_ign_dn4)) / (var_gds0_ign * var_gds0_ign)), ((((((assign22220_e27563 * var_t10_dn5) * var_t10) + (assign22220_e27565 * var_t10_dn5)) * var_gds0_ign) - (assign22220_e27567 * var_gds0_ign_dn5)) / (var_gds0_ign * var_gds0_ign)), ((((((assign22220_e27563 * var_t10_dn6) * var_t10) + (assign22220_e27565 * var_t10_dn6)) * var_gds0_ign) - (assign22220_e27567 * var_gds0_ign_dn6)) / (var_gds0_ign * var_gds0_ign)), ((((((assign22220_e27563 * var_t10_dn8) * var_t10) + (assign22220_e27565 * var_t10_dn8)) * var_gds0_ign) - (assign22220_e27567 * var_gds0_ign_dn8)) / (var_gds0_ign * var_gds0_ign)), ((((((assign22220_e27563 * var_t10_dn10) * var_t10) + (assign22220_e27565 * var_t10_dn10)) * var_gds0_ign) - (assign22220_e27567 * var_gds0_ign_dn10)) / (var_gds0_ign * var_gds0_ign)), ((((((assign22220_e27563 * var_t10_dn11) * var_t10) + (assign22220_e27565 * var_t10_dn11)) * var_gds0_ign) - (assign22220_e27567 * var_gds0_ign_dn11)) / (var_gds0_ign * var_gds0_ign)), ((((((assign22220_e27563 * var_t10_dn12) * var_t10) + (assign22220_e27565 * var_t10_dn12)) * var_gds0_ign) - (assign22220_e27567 * var_gds0_ign_dn12)) / (var_gds0_ign * var_gds0_ign)),)
    } else {
        (var_nign0, var_nign0_dn0, var_nign0_dn2, var_nign0_dn4, var_nign0_dn5, var_nign0_dn6, var_nign0_dn8, var_nign0_dn10, var_nign0_dn11, var_nign0_dn12,)
    }
};
        var_nign0 = assign22220_e27571;
        var_nign0_dn0 = assign22220_e27571_d_n0;
        var_nign0_dn2 = assign22220_e27571_d_n2;
        var_nign0_dn4 = assign22220_e27571_d_n4;
        var_nign0_dn5 = assign22220_e27571_d_n5;
        var_nign0_dn6 = assign22220_e27571_d_n6;
        var_nign0_dn8 = assign22220_e27571_d_n8;
        var_nign0_dn10 = assign22220_e27571_d_n10;
        var_nign0_dn11 = assign22220_e27571_d_n11;
        var_nign0_dn12 = assign22220_e27571_d_n12;

        let assign22230_e27575: f64 = (10.0 * 2.220446049250313e-16);
        let assign22230_e27580: f64 = (10.0 * 2.220446049250313e-16);
        let assign22230_e27582: f64 = if ((var_kusai00l > assign22230_e27575) && (var_vds > assign22230_e27580)) { 1.0 } else { 0.0 };
        var_guard379 = assign22230_e27582;

        *var_cgdbd_slot = var_cgdbd;
        *var_cgdbd_dn0_slot = var_cgdbd_dn0;
        *var_cgdbd_dn10_slot = var_cgdbd_dn10;
        *var_cgdbd_dn11_slot = var_cgdbd_dn11;
        *var_cgdbd_dn12_slot = var_cgdbd_dn12;
        *var_cgdbd_dn2_slot = var_cgdbd_dn2;
        *var_cgdbd_dn4_slot = var_cgdbd_dn4;
        *var_cgdbd_dn5_slot = var_cgdbd_dn5;
        *var_cgdbd_dn6_slot = var_cgdbd_dn6;
        *var_cgdbd_dn8_slot = var_cgdbd_dn8;
        *var_cgsb_slot = var_cgsb;
        *var_cgsb_dn0_slot = var_cgsb_dn0;
        *var_cgsb_dn10_slot = var_cgsb_dn10;
        *var_cgsb_dn11_slot = var_cgsb_dn11;
        *var_cgsb_dn12_slot = var_cgsb_dn12;
        *var_cgsb_dn2_slot = var_cgsb_dn2;
        *var_cgsb_dn4_slot = var_cgsb_dn4;
        *var_cgsb_dn5_slot = var_cgsb_dn5;
        *var_cgsb_dn6_slot = var_cgsb_dn6;
        *var_cgsb_dn8_slot = var_cgsb_dn8;
        *var_cgsbd_slot = var_cgsbd;
        *var_cgsbd_dn0_slot = var_cgsbd_dn0;
        *var_cgsbd_dn10_slot = var_cgsbd_dn10;
        *var_cgsbd_dn11_slot = var_cgsbd_dn11;
        *var_cgsbd_dn12_slot = var_cgsbd_dn12;
        *var_cgsbd_dn2_slot = var_cgsbd_dn2;
        *var_cgsbd_dn4_slot = var_cgsbd_dn4;
        *var_cgsbd_dn5_slot = var_cgsbd_dn5;
        *var_cgsbd_dn6_slot = var_cgsbd_dn6;
        *var_cgsbd_dn8_slot = var_cgsbd_dn8;
        *var_guard374_slot = var_guard374;
        *var_guard375_slot = var_guard375;
        *var_guard376_slot = var_guard376;
        *var_guard377_slot = var_guard377;
        *var_guard378_slot = var_guard378;
        *var_guard379_slot = var_guard379;
        *var_igbe_slot = var_igbe;
        *var_igbe_dn0_slot = var_igbe_dn0;
        *var_igbe_dn10_slot = var_igbe_dn10;
        *var_igbe_dn11_slot = var_igbe_dn11;
        *var_igbe_dn12_slot = var_igbe_dn12;
        *var_igbe_dn2_slot = var_igbe_dn2;
        *var_igbe_dn4_slot = var_igbe_dn4;
        *var_igbe_dn5_slot = var_igbe_dn5;
        *var_igbe_dn6_slot = var_igbe_dn6;
        *var_igbe_dn8_slot = var_igbe_dn8;
        *var_igde_slot = var_igde;
        *var_igde_dn0_slot = var_igde_dn0;
        *var_igde_dn10_slot = var_igde_dn10;
        *var_igde_dn11_slot = var_igde_dn11;
        *var_igde_dn12_slot = var_igde_dn12;
        *var_igde_dn2_slot = var_igde_dn2;
        *var_igde_dn4_slot = var_igde_dn4;
        *var_igde_dn5_slot = var_igde_dn5;
        *var_igde_dn6_slot = var_igde_dn6;
        *var_igde_dn8_slot = var_igde_dn8;
        *var_igidle_slot = var_igidle;
        *var_igidle_dn0_slot = var_igidle_dn0;
        *var_igidle_dn10_slot = var_igidle_dn10;
        *var_igidle_dn11_slot = var_igidle_dn11;
        *var_igidle_dn12_slot = var_igidle_dn12;
        *var_igidle_dn2_slot = var_igidle_dn2;
        *var_igidle_dn4_slot = var_igidle_dn4;
        *var_igidle_dn5_slot = var_igidle_dn5;
        *var_igidle_dn6_slot = var_igidle_dn6;
        *var_igidle_dn8_slot = var_igidle_dn8;
        *var_igisle_slot = var_igisle;
        *var_igisle_dn0_slot = var_igisle_dn0;
        *var_igisle_dn10_slot = var_igisle_dn10;
        *var_igisle_dn11_slot = var_igisle_dn11;
        *var_igisle_dn12_slot = var_igisle_dn12;
        *var_igisle_dn2_slot = var_igisle_dn2;
        *var_igisle_dn4_slot = var_igisle_dn4;
        *var_igisle_dn5_slot = var_igisle_dn5;
        *var_igisle_dn6_slot = var_igisle_dn6;
        *var_igisle_dn8_slot = var_igisle_dn8;
        *var_igse_slot = var_igse;
        *var_igse_dn0_slot = var_igse_dn0;
        *var_igse_dn10_slot = var_igse_dn10;
        *var_igse_dn11_slot = var_igse_dn11;
        *var_igse_dn12_slot = var_igse_dn12;
        *var_igse_dn2_slot = var_igse_dn2;
        *var_igse_dn4_slot = var_igse_dn4;
        *var_igse_dn5_slot = var_igse_dn5;
        *var_igse_dn6_slot = var_igse_dn6;
        *var_igse_dn8_slot = var_igse_dn8;
        *var_isube_slot = var_isube;
        *var_isube_dn0_slot = var_isube_dn0;
        *var_isube_dn10_slot = var_isube_dn10;
        *var_isube_dn11_slot = var_isube_dn11;
        *var_isube_dn12_slot = var_isube_dn12;
        *var_isube_dn2_slot = var_isube_dn2;
        *var_isube_dn4_slot = var_isube_dn4;
        *var_isube_dn5_slot = var_isube_dn5;
        *var_isube_dn6_slot = var_isube_dn6;
        *var_isube_dn8_slot = var_isube_dn8;
        *var_nign0_slot = var_nign0;
        *var_nign0_dn0_slot = var_nign0_dn0;
        *var_nign0_dn10_slot = var_nign0_dn10;
        *var_nign0_dn11_slot = var_nign0_dn11;
        *var_nign0_dn12_slot = var_nign0_dn12;
        *var_nign0_dn2_slot = var_nign0_dn2;
        *var_nign0_dn4_slot = var_nign0_dn4;
        *var_nign0_dn5_slot = var_nign0_dn5;
        *var_nign0_dn6_slot = var_nign0_dn6;
        *var_nign0_dn8_slot = var_nign0_dn8;
        *var_noithrml_slot = var_noithrml;
        *var_noithrml_dn0_slot = var_noithrml_dn0;
        *var_noithrml_dn10_slot = var_noithrml_dn10;
        *var_noithrml_dn11_slot = var_noithrml_dn11;
        *var_noithrml_dn12_slot = var_noithrml_dn12;
        *var_noithrml_dn2_slot = var_noithrml_dn2;
        *var_noithrml_dn4_slot = var_noithrml_dn4;
        *var_noithrml_dn5_slot = var_noithrml_dn5;
        *var_noithrml_dn6_slot = var_noithrml_dn6;
        *var_noithrml_dn8_slot = var_noithrml_dn8;
        *var_qde_slot = var_qde;
        *var_qde_dn0_slot = var_qde_dn0;
        *var_qde_dn10_slot = var_qde_dn10;
        *var_qde_dn11_slot = var_qde_dn11;
        *var_qde_dn12_slot = var_qde_dn12;
        *var_qde_dn2_slot = var_qde_dn2;
        *var_qde_dn4_slot = var_qde_dn4;
        *var_qde_dn5_slot = var_qde_dn5;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qde_dn8_slot = var_qde_dn8;
        *var_qge_slot = var_qge;
        *var_qge_dn0_slot = var_qge_dn0;
        *var_qge_dn10_slot = var_qge_dn10;
        *var_qge_dn11_slot = var_qge_dn11;
        *var_qge_dn12_slot = var_qge_dn12;
        *var_qge_dn2_slot = var_qge_dn2;
        *var_qge_dn4_slot = var_qge_dn4;
        *var_qge_dn5_slot = var_qge_dn5;
        *var_qge_dn6_slot = var_qge_dn6;
        *var_qge_dn8_slot = var_qge_dn8;
        *var_qse_slot = var_qse;
        *var_qse_dn0_slot = var_qse_dn0;
        *var_qse_dn10_slot = var_qse_dn10;
        *var_qse_dn11_slot = var_qse_dn11;
        *var_qse_dn12_slot = var_qse_dn12;
        *var_qse_dn2_slot = var_qse_dn2;
        *var_qse_dn4_slot = var_qse_dn4;
        *var_qse_dn5_slot = var_qse_dn5;
        *var_qse_dn6_slot = var_qse_dn6;
        *var_qse_dn8_slot = var_qse_dn8;
        *var_qy_slot = var_qy;
        *var_qy_dn0_slot = var_qy_dn0;
        *var_qy_dn10_slot = var_qy_dn10;
        *var_qy_dn11_slot = var_qy_dn11;
        *var_qy_dn12_slot = var_qy_dn12;
        *var_qy_dn2_slot = var_qy_dn2;
        *var_qy_dn4_slot = var_qy_dn4;
        *var_qy_dn5_slot = var_qy_dn5;
        *var_qy_dn6_slot = var_qy_dn6;
        *var_qy_dn8_slot = var_qy_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t10_slot = var_t10;
        *var_t10_dn0_slot = var_t10_dn0;
        *var_t10_dn10_slot = var_t10_dn10;
        *var_t10_dn11_slot = var_t10_dn11;
        *var_t10_dn12_slot = var_t10_dn12;
        *var_t10_dn2_slot = var_t10_dn2;
        *var_t10_dn4_slot = var_t10_dn4;
        *var_t10_dn5_slot = var_t10_dn5;
        *var_t10_dn6_slot = var_t10_dn6;
        *var_t10_dn8_slot = var_t10_dn8;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_whi_noise_slot = var_whi_noise;
        *var_whi_noise_dn4_slot = var_whi_noise_dn4;
    }

    pub(super) fn stamp_transient_block_85(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_crl_f: f64,
        var_crl_f_dn0: f64,
        var_crl_f_dn10: f64,
        var_crl_f_dn11: f64,
        var_crl_f_dn12: f64,
        var_crl_f_dn2: f64,
        var_crl_f_dn4: f64,
        var_crl_f_dn5: f64,
        var_crl_f_dn6: f64,
        var_crl_f_dn8: f64,
        var_guard378: f64,
        var_guard379: f64,
        var_kusai00: f64,
        var_kusai00_dn0: f64,
        var_kusai00_dn10: f64,
        var_kusai00_dn11: f64,
        var_kusai00_dn12: f64,
        var_kusai00_dn2: f64,
        var_kusai00_dn4: f64,
        var_kusai00_dn5: f64,
        var_kusai00_dn6: f64,
        var_kusai00_dn8: f64,
        var_kusai_ig: f64,
        var_kusai_ig_dn0: f64,
        var_kusai_ig_dn10: f64,
        var_kusai_ig_dn11: f64,
        var_kusai_ig_dn12: f64,
        var_kusai_ig_dn2: f64,
        var_kusai_ig_dn4: f64,
        var_kusai_ig_dn5: f64,
        var_kusai_ig_dn6: f64,
        var_kusai_ig_dn8: f64,
        var_kusail: f64,
        var_kusail_dn0: f64,
        var_kusail_dn10: f64,
        var_kusail_dn11: f64,
        var_kusail_dn12: f64,
        var_kusail_dn2: f64,
        var_kusail_dn4: f64,
        var_kusail_dn5: f64,
        var_kusail_dn6: f64,
        var_kusail_dn8: f64,
        var_mfactor: f64,
        var_mode: f64,
        var_mu: f64,
        var_mu_dn0: f64,
        var_mu_dn10: f64,
        var_mu_dn11: f64,
        var_mu_dn12: f64,
        var_mu_dn2: f64,
        var_mu_dn4: f64,
        var_mu_dn5: f64,
        var_mu_dn6: f64,
        var_mu_dn8: f64,
        var_mud_hoso: f64,
        var_mud_hoso_dn0: f64,
        var_mud_hoso_dn10: f64,
        var_mud_hoso_dn11: f64,
        var_mud_hoso_dn12: f64,
        var_mud_hoso_dn2: f64,
        var_mud_hoso_dn4: f64,
        var_mud_hoso_dn5: f64,
        var_mud_hoso_dn6: f64,
        var_mud_hoso_dn8: f64,
        var_muun: f64,
        var_muun_dn0: f64,
        var_muun_dn10: f64,
        var_muun_dn11: f64,
        var_muun_dn12: f64,
        var_muun_dn2: f64,
        var_muun_dn4: f64,
        var_muun_dn5: f64,
        var_muun_dn6: f64,
        var_muun_dn8: f64,
        var_nign0: f64,
        var_nign0_dn0: f64,
        var_nign0_dn10: f64,
        var_nign0_dn11: f64,
        var_nign0_dn12: f64,
        var_nign0_dn2: f64,
        var_nign0_dn4: f64,
        var_nign0_dn5: f64,
        var_nign0_dn6: f64,
        var_nign0_dn8: f64,
        var_noithrml: f64,
        var_noithrml_dn0: f64,
        var_noithrml_dn10: f64,
        var_noithrml_dn11: f64,
        var_noithrml_dn12: f64,
        var_noithrml_dn2: f64,
        var_noithrml_dn4: f64,
        var_noithrml_dn5: f64,
        var_noithrml_dn6: f64,
        var_noithrml_dn8: f64,
        var_qdrat: f64,
        var_sqrtkusail: f64,
        var_sqrtkusail_dn0: f64,
        var_sqrtkusail_dn10: f64,
        var_sqrtkusail_dn11: f64,
        var_sqrtkusail_dn12: f64,
        var_sqrtkusail_dn2: f64,
        var_sqrtkusail_dn4: f64,
        var_sqrtkusail_dn5: f64,
        var_sqrtkusail_dn6: f64,
        var_sqrtkusail_dn8: f64,
        var_t0: f64,
        var_t10: f64,
        var_ttemp: f64,
        var_ttemp_dn4: f64,
        var_uc_tnom: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn8: f64,
        var_vgvt: f64,
        var_vgvt_dn0: f64,
        var_vgvt_dn10: f64,
        var_vgvt_dn11: f64,
        var_vgvt_dn12: f64,
        var_vgvt_dn2: f64,
        var_vgvt_dn4: f64,
        var_vgvt_dn5: f64,
        var_vgvt_dn6: f64,
        var_vgvt_dn8: f64,
        var_weff: f64,
        var_weff_dn0: f64,
        var_weff_dn10: f64,
        var_weff_dn11: f64,
        var_weff_dn12: f64,
        var_weff_dn2: f64,
        var_weff_dn4: f64,
        var_weff_dn5: f64,
        var_weff_dn6: f64,
        var_weff_dn8: f64,
        var_whi_noise: f64,
        var_whi_noise_dn4: f64,
        var_ci_slot: &mut f64,
        var_ci_dn0_slot: &mut f64,
        var_ci_dn10_slot: &mut f64,
        var_ci_dn11_slot: &mut f64,
        var_ci_dn12_slot: &mut f64,
        var_ci_dn2_slot: &mut f64,
        var_ci_dn4_slot: &mut f64,
        var_ci_dn5_slot: &mut f64,
        var_ci_dn6_slot: &mut f64,
        var_ci_dn8_slot: &mut f64,
        var_correct_w1_slot: &mut f64,
        var_correct_w1_dn0_slot: &mut f64,
        var_correct_w1_dn10_slot: &mut f64,
        var_correct_w1_dn11_slot: &mut f64,
        var_correct_w1_dn12_slot: &mut f64,
        var_correct_w1_dn2_slot: &mut f64,
        var_correct_w1_dn4_slot: &mut f64,
        var_correct_w1_dn5_slot: &mut f64,
        var_correct_w1_dn6_slot: &mut f64,
        var_correct_w1_dn8_slot: &mut f64,
        var_guard380_slot: &mut f64,
        var_ldrifte_slot: &mut f64,
        var_mks_rdrmue_slot: &mut f64,
        var_mks_rdrvmax_slot: &mut f64,
        var_mumoda_slot: &mut f64,
        var_mumoda_dn0_slot: &mut f64,
        var_mumoda_dn10_slot: &mut f64,
        var_mumoda_dn11_slot: &mut f64,
        var_mumoda_dn12_slot: &mut f64,
        var_mumoda_dn2_slot: &mut f64,
        var_mumoda_dn4_slot: &mut f64,
        var_mumoda_dn5_slot: &mut f64,
        var_mumoda_dn6_slot: &mut f64,
        var_mumoda_dn8_slot: &mut f64,
        var_mumodb_slot: &mut f64,
        var_mumodb_dn0_slot: &mut f64,
        var_mumodb_dn10_slot: &mut f64,
        var_mumodb_dn11_slot: &mut f64,
        var_mumodb_dn12_slot: &mut f64,
        var_mumodb_dn2_slot: &mut f64,
        var_mumodb_dn4_slot: &mut f64,
        var_mumodb_dn5_slot: &mut f64,
        var_mumodb_dn6_slot: &mut f64,
        var_mumodb_dn8_slot: &mut f64,
        var_noicross_slot: &mut f64,
        var_noicross_dn0_slot: &mut f64,
        var_noicross_dn10_slot: &mut f64,
        var_noicross_dn11_slot: &mut f64,
        var_noicross_dn12_slot: &mut f64,
        var_noicross_dn2_slot: &mut f64,
        var_noicross_dn4_slot: &mut f64,
        var_noicross_dn5_slot: &mut f64,
        var_noicross_dn6_slot: &mut f64,
        var_noicross_dn8_slot: &mut f64,
        var_noiigate_slot: &mut f64,
        var_noiigate_dn0_slot: &mut f64,
        var_noiigate_dn10_slot: &mut f64,
        var_noiigate_dn11_slot: &mut f64,
        var_noiigate_dn12_slot: &mut f64,
        var_noiigate_dn2_slot: &mut f64,
        var_noiigate_dn4_slot: &mut f64,
        var_noiigate_dn5_slot: &mut f64,
        var_noiigate_dn6_slot: &mut f64,
        var_noiigate_dn8_slot: &mut f64,
        var_nover_slot: &mut f64,
        var_rdde_slot: &mut f64,
        var_rdde_dn0_slot: &mut f64,
        var_rdde_dn10_slot: &mut f64,
        var_rdde_dn11_slot: &mut f64,
        var_rdde_dn12_slot: &mut f64,
        var_rdde_dn2_slot: &mut f64,
        var_rdde_dn4_slot: &mut f64,
        var_rdde_dn5_slot: &mut f64,
        var_rdde_dn6_slot: &mut f64,
        var_rdde_dn8_slot: &mut f64,
        var_rrdrbb_slot: &mut f64,
        var_rrdrbb_dn4_slot: &mut f64,
        var_rsd0_slot: &mut f64,
        var_rsde_slot: &mut f64,
        var_rsde_dn0_slot: &mut f64,
        var_rsde_dn10_slot: &mut f64,
        var_rsde_dn11_slot: &mut f64,
        var_rsde_dn12_slot: &mut f64,
        var_rsde_dn2_slot: &mut f64,
        var_rsde_dn4_slot: &mut f64,
        var_rsde_dn5_slot: &mut f64,
        var_rsde_dn6_slot: &mut f64,
        var_rsde_dn8_slot: &mut f64,
        var_sid_slot: &mut f64,
        var_sid_dn0_slot: &mut f64,
        var_sid_dn10_slot: &mut f64,
        var_sid_dn11_slot: &mut f64,
        var_sid_dn12_slot: &mut f64,
        var_sid_dn2_slot: &mut f64,
        var_sid_dn4_slot: &mut f64,
        var_sid_dn5_slot: &mut f64,
        var_sid_dn6_slot: &mut f64,
        var_sid_dn8_slot: &mut f64,
        var_sigrat_slot: &mut f64,
        var_sigrat_d_slot: &mut f64,
        var_sigrat_d_dn0_slot: &mut f64,
        var_sigrat_d_dn10_slot: &mut f64,
        var_sigrat_d_dn11_slot: &mut f64,
        var_sigrat_d_dn12_slot: &mut f64,
        var_sigrat_d_dn2_slot: &mut f64,
        var_sigrat_d_dn4_slot: &mut f64,
        var_sigrat_d_dn5_slot: &mut f64,
        var_sigrat_d_dn6_slot: &mut f64,
        var_sigrat_d_dn8_slot: &mut f64,
        var_sigrat_dn0_slot: &mut f64,
        var_sigrat_dn10_slot: &mut f64,
        var_sigrat_dn11_slot: &mut f64,
        var_sigrat_dn12_slot: &mut f64,
        var_sigrat_dn2_slot: &mut f64,
        var_sigrat_dn4_slot: &mut f64,
        var_sigrat_dn5_slot: &mut f64,
        var_sigrat_dn6_slot: &mut f64,
        var_sigrat_dn8_slot: &mut f64,
        var_sigrat_s_slot: &mut f64,
        var_sigrat_s_dn0_slot: &mut f64,
        var_sigrat_s_dn10_slot: &mut f64,
        var_sigrat_s_dn11_slot: &mut f64,
        var_sigrat_s_dn12_slot: &mut f64,
        var_sigrat_s_dn2_slot: &mut f64,
        var_sigrat_s_dn4_slot: &mut f64,
        var_sigrat_s_dn5_slot: &mut f64,
        var_sigrat_s_dn6_slot: &mut f64,
        var_sigrat_s_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_tratio_slot: &mut f64,
        var_tratio_dn4_slot: &mut f64,
        var_vrdr_slot: &mut f64,
        var_vrdr_dn12_slot: &mut f64,
        var_vrdr_dn2_slot: &mut f64,
        var_weff_nf_1_slot: &mut f64,
        var_weff_nf_1_dn0_slot: &mut f64,
        var_weff_nf_1_dn10_slot: &mut f64,
        var_weff_nf_1_dn11_slot: &mut f64,
        var_weff_nf_1_dn12_slot: &mut f64,
        var_weff_nf_1_dn2_slot: &mut f64,
        var_weff_nf_1_dn4_slot: &mut f64,
        var_weff_nf_1_dn5_slot: &mut f64,
        var_weff_nf_1_dn6_slot: &mut f64,
        var_weff_nf_1_dn8_slot: &mut f64,
        var_xov_slot: &mut f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let mut var_ci: f64 = *var_ci_slot;
        let mut var_ci_dn0: f64 = *var_ci_dn0_slot;
        let mut var_ci_dn10: f64 = *var_ci_dn10_slot;
        let mut var_ci_dn11: f64 = *var_ci_dn11_slot;
        let mut var_ci_dn12: f64 = *var_ci_dn12_slot;
        let mut var_ci_dn2: f64 = *var_ci_dn2_slot;
        let mut var_ci_dn4: f64 = *var_ci_dn4_slot;
        let mut var_ci_dn5: f64 = *var_ci_dn5_slot;
        let mut var_ci_dn6: f64 = *var_ci_dn6_slot;
        let mut var_ci_dn8: f64 = *var_ci_dn8_slot;
        let mut var_correct_w1: f64 = *var_correct_w1_slot;
        let mut var_correct_w1_dn0: f64 = *var_correct_w1_dn0_slot;
        let mut var_correct_w1_dn10: f64 = *var_correct_w1_dn10_slot;
        let mut var_correct_w1_dn11: f64 = *var_correct_w1_dn11_slot;
        let mut var_correct_w1_dn12: f64 = *var_correct_w1_dn12_slot;
        let mut var_correct_w1_dn2: f64 = *var_correct_w1_dn2_slot;
        let mut var_correct_w1_dn4: f64 = *var_correct_w1_dn4_slot;
        let mut var_correct_w1_dn5: f64 = *var_correct_w1_dn5_slot;
        let mut var_correct_w1_dn6: f64 = *var_correct_w1_dn6_slot;
        let mut var_correct_w1_dn8: f64 = *var_correct_w1_dn8_slot;
        let mut var_guard380: f64 = *var_guard380_slot;
        let mut var_ldrifte: f64 = *var_ldrifte_slot;
        let mut var_mks_rdrmue: f64 = *var_mks_rdrmue_slot;
        let mut var_mks_rdrvmax: f64 = *var_mks_rdrvmax_slot;
        let mut var_mumoda: f64 = *var_mumoda_slot;
        let mut var_mumoda_dn0: f64 = *var_mumoda_dn0_slot;
        let mut var_mumoda_dn10: f64 = *var_mumoda_dn10_slot;
        let mut var_mumoda_dn11: f64 = *var_mumoda_dn11_slot;
        let mut var_mumoda_dn12: f64 = *var_mumoda_dn12_slot;
        let mut var_mumoda_dn2: f64 = *var_mumoda_dn2_slot;
        let mut var_mumoda_dn4: f64 = *var_mumoda_dn4_slot;
        let mut var_mumoda_dn5: f64 = *var_mumoda_dn5_slot;
        let mut var_mumoda_dn6: f64 = *var_mumoda_dn6_slot;
        let mut var_mumoda_dn8: f64 = *var_mumoda_dn8_slot;
        let mut var_mumodb: f64 = *var_mumodb_slot;
        let mut var_mumodb_dn0: f64 = *var_mumodb_dn0_slot;
        let mut var_mumodb_dn10: f64 = *var_mumodb_dn10_slot;
        let mut var_mumodb_dn11: f64 = *var_mumodb_dn11_slot;
        let mut var_mumodb_dn12: f64 = *var_mumodb_dn12_slot;
        let mut var_mumodb_dn2: f64 = *var_mumodb_dn2_slot;
        let mut var_mumodb_dn4: f64 = *var_mumodb_dn4_slot;
        let mut var_mumodb_dn5: f64 = *var_mumodb_dn5_slot;
        let mut var_mumodb_dn6: f64 = *var_mumodb_dn6_slot;
        let mut var_mumodb_dn8: f64 = *var_mumodb_dn8_slot;
        let mut var_noicross: f64 = *var_noicross_slot;
        let mut var_noicross_dn0: f64 = *var_noicross_dn0_slot;
        let mut var_noicross_dn10: f64 = *var_noicross_dn10_slot;
        let mut var_noicross_dn11: f64 = *var_noicross_dn11_slot;
        let mut var_noicross_dn12: f64 = *var_noicross_dn12_slot;
        let mut var_noicross_dn2: f64 = *var_noicross_dn2_slot;
        let mut var_noicross_dn4: f64 = *var_noicross_dn4_slot;
        let mut var_noicross_dn5: f64 = *var_noicross_dn5_slot;
        let mut var_noicross_dn6: f64 = *var_noicross_dn6_slot;
        let mut var_noicross_dn8: f64 = *var_noicross_dn8_slot;
        let mut var_noiigate: f64 = *var_noiigate_slot;
        let mut var_noiigate_dn0: f64 = *var_noiigate_dn0_slot;
        let mut var_noiigate_dn10: f64 = *var_noiigate_dn10_slot;
        let mut var_noiigate_dn11: f64 = *var_noiigate_dn11_slot;
        let mut var_noiigate_dn12: f64 = *var_noiigate_dn12_slot;
        let mut var_noiigate_dn2: f64 = *var_noiigate_dn2_slot;
        let mut var_noiigate_dn4: f64 = *var_noiigate_dn4_slot;
        let mut var_noiigate_dn5: f64 = *var_noiigate_dn5_slot;
        let mut var_noiigate_dn6: f64 = *var_noiigate_dn6_slot;
        let mut var_noiigate_dn8: f64 = *var_noiigate_dn8_slot;
        let mut var_nover: f64 = *var_nover_slot;
        let mut var_rdde: f64 = *var_rdde_slot;
        let mut var_rdde_dn0: f64 = *var_rdde_dn0_slot;
        let mut var_rdde_dn10: f64 = *var_rdde_dn10_slot;
        let mut var_rdde_dn11: f64 = *var_rdde_dn11_slot;
        let mut var_rdde_dn12: f64 = *var_rdde_dn12_slot;
        let mut var_rdde_dn2: f64 = *var_rdde_dn2_slot;
        let mut var_rdde_dn4: f64 = *var_rdde_dn4_slot;
        let mut var_rdde_dn5: f64 = *var_rdde_dn5_slot;
        let mut var_rdde_dn6: f64 = *var_rdde_dn6_slot;
        let mut var_rdde_dn8: f64 = *var_rdde_dn8_slot;
        let mut var_rrdrbb: f64 = *var_rrdrbb_slot;
        let mut var_rrdrbb_dn4: f64 = *var_rrdrbb_dn4_slot;
        let mut var_rsd0: f64 = *var_rsd0_slot;
        let mut var_rsde: f64 = *var_rsde_slot;
        let mut var_rsde_dn0: f64 = *var_rsde_dn0_slot;
        let mut var_rsde_dn10: f64 = *var_rsde_dn10_slot;
        let mut var_rsde_dn11: f64 = *var_rsde_dn11_slot;
        let mut var_rsde_dn12: f64 = *var_rsde_dn12_slot;
        let mut var_rsde_dn2: f64 = *var_rsde_dn2_slot;
        let mut var_rsde_dn4: f64 = *var_rsde_dn4_slot;
        let mut var_rsde_dn5: f64 = *var_rsde_dn5_slot;
        let mut var_rsde_dn6: f64 = *var_rsde_dn6_slot;
        let mut var_rsde_dn8: f64 = *var_rsde_dn8_slot;
        let mut var_sid: f64 = *var_sid_slot;
        let mut var_sid_dn0: f64 = *var_sid_dn0_slot;
        let mut var_sid_dn10: f64 = *var_sid_dn10_slot;
        let mut var_sid_dn11: f64 = *var_sid_dn11_slot;
        let mut var_sid_dn12: f64 = *var_sid_dn12_slot;
        let mut var_sid_dn2: f64 = *var_sid_dn2_slot;
        let mut var_sid_dn4: f64 = *var_sid_dn4_slot;
        let mut var_sid_dn5: f64 = *var_sid_dn5_slot;
        let mut var_sid_dn6: f64 = *var_sid_dn6_slot;
        let mut var_sid_dn8: f64 = *var_sid_dn8_slot;
        let mut var_sigrat: f64 = *var_sigrat_slot;
        let mut var_sigrat_d: f64 = *var_sigrat_d_slot;
        let mut var_sigrat_d_dn0: f64 = *var_sigrat_d_dn0_slot;
        let mut var_sigrat_d_dn10: f64 = *var_sigrat_d_dn10_slot;
        let mut var_sigrat_d_dn11: f64 = *var_sigrat_d_dn11_slot;
        let mut var_sigrat_d_dn12: f64 = *var_sigrat_d_dn12_slot;
        let mut var_sigrat_d_dn2: f64 = *var_sigrat_d_dn2_slot;
        let mut var_sigrat_d_dn4: f64 = *var_sigrat_d_dn4_slot;
        let mut var_sigrat_d_dn5: f64 = *var_sigrat_d_dn5_slot;
        let mut var_sigrat_d_dn6: f64 = *var_sigrat_d_dn6_slot;
        let mut var_sigrat_d_dn8: f64 = *var_sigrat_d_dn8_slot;
        let mut var_sigrat_dn0: f64 = *var_sigrat_dn0_slot;
        let mut var_sigrat_dn10: f64 = *var_sigrat_dn10_slot;
        let mut var_sigrat_dn11: f64 = *var_sigrat_dn11_slot;
        let mut var_sigrat_dn12: f64 = *var_sigrat_dn12_slot;
        let mut var_sigrat_dn2: f64 = *var_sigrat_dn2_slot;
        let mut var_sigrat_dn4: f64 = *var_sigrat_dn4_slot;
        let mut var_sigrat_dn5: f64 = *var_sigrat_dn5_slot;
        let mut var_sigrat_dn6: f64 = *var_sigrat_dn6_slot;
        let mut var_sigrat_dn8: f64 = *var_sigrat_dn8_slot;
        let mut var_sigrat_s: f64 = *var_sigrat_s_slot;
        let mut var_sigrat_s_dn0: f64 = *var_sigrat_s_dn0_slot;
        let mut var_sigrat_s_dn10: f64 = *var_sigrat_s_dn10_slot;
        let mut var_sigrat_s_dn11: f64 = *var_sigrat_s_dn11_slot;
        let mut var_sigrat_s_dn12: f64 = *var_sigrat_s_dn12_slot;
        let mut var_sigrat_s_dn2: f64 = *var_sigrat_s_dn2_slot;
        let mut var_sigrat_s_dn4: f64 = *var_sigrat_s_dn4_slot;
        let mut var_sigrat_s_dn5: f64 = *var_sigrat_s_dn5_slot;
        let mut var_sigrat_s_dn6: f64 = *var_sigrat_s_dn6_slot;
        let mut var_sigrat_s_dn8: f64 = *var_sigrat_s_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_tratio: f64 = *var_tratio_slot;
        let mut var_tratio_dn4: f64 = *var_tratio_dn4_slot;
        let mut var_vrdr: f64 = *var_vrdr_slot;
        let mut var_vrdr_dn12: f64 = *var_vrdr_dn12_slot;
        let mut var_vrdr_dn2: f64 = *var_vrdr_dn2_slot;
        let mut var_weff_nf_1: f64 = *var_weff_nf_1_slot;
        let mut var_weff_nf_1_dn0: f64 = *var_weff_nf_1_dn0_slot;
        let mut var_weff_nf_1_dn10: f64 = *var_weff_nf_1_dn10_slot;
        let mut var_weff_nf_1_dn11: f64 = *var_weff_nf_1_dn11_slot;
        let mut var_weff_nf_1_dn12: f64 = *var_weff_nf_1_dn12_slot;
        let mut var_weff_nf_1_dn2: f64 = *var_weff_nf_1_dn2_slot;
        let mut var_weff_nf_1_dn4: f64 = *var_weff_nf_1_dn4_slot;
        let mut var_weff_nf_1_dn5: f64 = *var_weff_nf_1_dn5_slot;
        let mut var_weff_nf_1_dn6: f64 = *var_weff_nf_1_dn6_slot;
        let mut var_weff_nf_1_dn8: f64 = *var_weff_nf_1_dn8_slot;
        let mut var_xov: f64 = *var_xov_slot;

        let (assign22240_e27590, assign22240_e27590_d_n0, assign22240_e27590_d_n2, assign22240_e27590_d_n4, assign22240_e27590_d_n5, assign22240_e27590_d_n6, assign22240_e27590_d_n8, assign22240_e27590_d_n10, assign22240_e27590_d_n11, assign22240_e27590_d_n12,) = {
    if ((var_guard378 != 0.0) && (var_guard379 != 0.0)) {
        let assign22240_e27588: f64 = (var_muun / var_mu);
        (assign22240_e27588, (((var_muun_dn0 * var_mu) - (var_muun * var_mu_dn0)) / (var_mu * var_mu)), (((var_muun_dn2 * var_mu) - (var_muun * var_mu_dn2)) / (var_mu * var_mu)), (((var_muun_dn4 * var_mu) - (var_muun * var_mu_dn4)) / (var_mu * var_mu)), (((var_muun_dn5 * var_mu) - (var_muun * var_mu_dn5)) / (var_mu * var_mu)), (((var_muun_dn6 * var_mu) - (var_muun * var_mu_dn6)) / (var_mu * var_mu)), (((var_muun_dn8 * var_mu) - (var_muun * var_mu_dn8)) / (var_mu * var_mu)), (((var_muun_dn10 * var_mu) - (var_muun * var_mu_dn10)) / (var_mu * var_mu)), (((var_muun_dn11 * var_mu) - (var_muun * var_mu_dn11)) / (var_mu * var_mu)), (((var_muun_dn12 * var_mu) - (var_muun * var_mu_dn12)) / (var_mu * var_mu)),)
    } else {
        (var_mumoda, var_mumoda_dn0, var_mumoda_dn2, var_mumoda_dn4, var_mumoda_dn5, var_mumoda_dn6, var_mumoda_dn8, var_mumoda_dn10, var_mumoda_dn11, var_mumoda_dn12,)
    }
};
        var_mumoda = assign22240_e27590;
        var_mumoda_dn0 = assign22240_e27590_d_n0;
        var_mumoda_dn2 = assign22240_e27590_d_n2;
        var_mumoda_dn4 = assign22240_e27590_d_n4;
        var_mumoda_dn5 = assign22240_e27590_d_n5;
        var_mumoda_dn6 = assign22240_e27590_d_n6;
        var_mumoda_dn8 = assign22240_e27590_d_n8;
        var_mumoda_dn10 = assign22240_e27590_d_n10;
        var_mumoda_dn11 = assign22240_e27590_d_n11;
        var_mumoda_dn12 = assign22240_e27590_d_n12;

        let (assign22250_e27602, assign22250_e27602_d_n0, assign22250_e27602_d_n2, assign22250_e27602_d_n4, assign22250_e27602_d_n5, assign22250_e27602_d_n6, assign22250_e27602_d_n8, assign22250_e27602_d_n10, assign22250_e27602_d_n11, assign22250_e27602_d_n12,) = {
    if ((var_guard378 != 0.0) && (var_guard379 != 0.0)) {
        let assign22250_e27596: f64 = (var_muun / var_mud_hoso);
        let assign22250_e27598: f64 = (assign22250_e27596 - var_mumoda);
        let assign22250_e27600: f64 = (assign22250_e27598 / var_vds);
        (assign22250_e27600, (((((((var_muun_dn0 * var_mud_hoso) - (var_muun * var_mud_hoso_dn0)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn0) * var_vds) - (assign22250_e27598 * var_vds_dn0)) / (var_vds * var_vds)), (((((((var_muun_dn2 * var_mud_hoso) - (var_muun * var_mud_hoso_dn2)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn2) * var_vds) - (assign22250_e27598 * var_vds_dn2)) / (var_vds * var_vds)), (((((((var_muun_dn4 * var_mud_hoso) - (var_muun * var_mud_hoso_dn4)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn4) * var_vds) - (assign22250_e27598 * var_vds_dn4)) / (var_vds * var_vds)), (((((((var_muun_dn5 * var_mud_hoso) - (var_muun * var_mud_hoso_dn5)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn5) * var_vds) - (assign22250_e27598 * var_vds_dn5)) / (var_vds * var_vds)), (((((((var_muun_dn6 * var_mud_hoso) - (var_muun * var_mud_hoso_dn6)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn6) * var_vds) - (assign22250_e27598 * var_vds_dn6)) / (var_vds * var_vds)), (((((((var_muun_dn8 * var_mud_hoso) - (var_muun * var_mud_hoso_dn8)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn8) * var_vds) - (assign22250_e27598 * var_vds_dn8)) / (var_vds * var_vds)), (((((((var_muun_dn10 * var_mud_hoso) - (var_muun * var_mud_hoso_dn10)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn10) * var_vds) - (assign22250_e27598 * var_vds_dn10)) / (var_vds * var_vds)), (((((((var_muun_dn11 * var_mud_hoso) - (var_muun * var_mud_hoso_dn11)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn11) * var_vds) - (assign22250_e27598 * var_vds_dn11)) / (var_vds * var_vds)), (((((((var_muun_dn12 * var_mud_hoso) - (var_muun * var_mud_hoso_dn12)) / (var_mud_hoso * var_mud_hoso)) - var_mumoda_dn12) * var_vds) - (assign22250_e27598 * var_vds_dn12)) / (var_vds * var_vds)),)
    } else {
        (var_mumodb, var_mumodb_dn0, var_mumodb_dn2, var_mumodb_dn4, var_mumodb_dn5, var_mumodb_dn6, var_mumodb_dn8, var_mumodb_dn10, var_mumodb_dn11, var_mumodb_dn12,)
    }
};
        var_mumodb = assign22250_e27602;
        var_mumodb_dn0 = assign22250_e27602_d_n0;
        var_mumodb_dn2 = assign22250_e27602_d_n2;
        var_mumodb_dn4 = assign22250_e27602_d_n4;
        var_mumodb_dn5 = assign22250_e27602_d_n5;
        var_mumodb_dn6 = assign22250_e27602_d_n6;
        var_mumodb_dn8 = assign22250_e27602_d_n8;
        var_mumodb_dn10 = assign22250_e27602_d_n10;
        var_mumodb_dn11 = assign22250_e27602_d_n11;
        var_mumodb_dn12 = assign22250_e27602_d_n12;

        let (assign22260_e27624, assign22260_e27624_d_n0, assign22260_e27624_d_n2, assign22260_e27624_d_n4, assign22260_e27624_d_n5, assign22260_e27624_d_n6, assign22260_e27624_d_n8, assign22260_e27624_d_n10, assign22260_e27624_d_n11, assign22260_e27624_d_n12,) = {
    if ((var_guard378 != 0.0) && (var_guard379 != 0.0)) {
        let assign22260_e27609: f64 = (0.6666666666666667 * var_mumodb);
        let assign22260_e27613: f64 = (var_vgvt * var_sqrtkusail);
        let assign22260_e27614: f64 = (var_kusai00 + assign22260_e27613);
        let assign22260_e27616: f64 = (assign22260_e27614 + var_kusail);
        let assign22260_e27617: f64 = (assign22260_e27609 * assign22260_e27616);
        let assign22260_e27620: f64 = (var_vgvt + var_sqrtkusail);
        let assign22260_e27621: f64 = (assign22260_e27617 / assign22260_e27620);
        let assign22260_e27622: f64 = (var_mumoda + assign22260_e27621);
        (assign22260_e27622, (var_mumoda_dn0 + ((((((0.6666666666666667 * var_mumodb_dn0) * assign22260_e27616) + (assign22260_e27609 * ((var_kusai00_dn0 + ((var_vgvt_dn0 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn0))) + var_kusail_dn0))) * assign22260_e27620) - (assign22260_e27617 * (var_vgvt_dn0 + var_sqrtkusail_dn0))) / (assign22260_e27620 * assign22260_e27620))), (var_mumoda_dn2 + ((((((0.6666666666666667 * var_mumodb_dn2) * assign22260_e27616) + (assign22260_e27609 * ((var_kusai00_dn2 + ((var_vgvt_dn2 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn2))) + var_kusail_dn2))) * assign22260_e27620) - (assign22260_e27617 * (var_vgvt_dn2 + var_sqrtkusail_dn2))) / (assign22260_e27620 * assign22260_e27620))), (var_mumoda_dn4 + ((((((0.6666666666666667 * var_mumodb_dn4) * assign22260_e27616) + (assign22260_e27609 * ((var_kusai00_dn4 + ((var_vgvt_dn4 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn4))) + var_kusail_dn4))) * assign22260_e27620) - (assign22260_e27617 * (var_vgvt_dn4 + var_sqrtkusail_dn4))) / (assign22260_e27620 * assign22260_e27620))), (var_mumoda_dn5 + ((((((0.6666666666666667 * var_mumodb_dn5) * assign22260_e27616) + (assign22260_e27609 * ((var_kusai00_dn5 + ((var_vgvt_dn5 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn5))) + var_kusail_dn5))) * assign22260_e27620) - (assign22260_e27617 * (var_vgvt_dn5 + var_sqrtkusail_dn5))) / (assign22260_e27620 * assign22260_e27620))), (var_mumoda_dn6 + ((((((0.6666666666666667 * var_mumodb_dn6) * assign22260_e27616) + (assign22260_e27609 * ((var_kusai00_dn6 + ((var_vgvt_dn6 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn6))) + var_kusail_dn6))) * assign22260_e27620) - (assign22260_e27617 * (var_vgvt_dn6 + var_sqrtkusail_dn6))) / (assign22260_e27620 * assign22260_e27620))), (var_mumoda_dn8 + ((((((0.6666666666666667 * var_mumodb_dn8) * assign22260_e27616) + (assign22260_e27609 * ((var_kusai00_dn8 + ((var_vgvt_dn8 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn8))) + var_kusail_dn8))) * assign22260_e27620) - (assign22260_e27617 * (var_vgvt_dn8 + var_sqrtkusail_dn8))) / (assign22260_e27620 * assign22260_e27620))), (var_mumoda_dn10 + ((((((0.6666666666666667 * var_mumodb_dn10) * assign22260_e27616) + (assign22260_e27609 * ((var_kusai00_dn10 + ((var_vgvt_dn10 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn10))) + var_kusail_dn10))) * assign22260_e27620) - (assign22260_e27617 * (var_vgvt_dn10 + var_sqrtkusail_dn10))) / (assign22260_e27620 * assign22260_e27620))), (var_mumoda_dn11 + ((((((0.6666666666666667 * var_mumodb_dn11) * assign22260_e27616) + (assign22260_e27609 * ((var_kusai00_dn11 + ((var_vgvt_dn11 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn11))) + var_kusail_dn11))) * assign22260_e27620) - (assign22260_e27617 * (var_vgvt_dn11 + var_sqrtkusail_dn11))) / (assign22260_e27620 * assign22260_e27620))), (var_mumoda_dn12 + ((((((0.6666666666666667 * var_mumodb_dn12) * assign22260_e27616) + (assign22260_e27609 * ((var_kusai00_dn12 + ((var_vgvt_dn12 * var_sqrtkusail) + (var_vgvt * var_sqrtkusail_dn12))) + var_kusail_dn12))) * assign22260_e27620) - (assign22260_e27617 * (var_vgvt_dn12 + var_sqrtkusail_dn12))) / (assign22260_e27620 * assign22260_e27620))),)
    } else {
        (var_correct_w1, var_correct_w1_dn0, var_correct_w1_dn2, var_correct_w1_dn4, var_correct_w1_dn5, var_correct_w1_dn6, var_correct_w1_dn8, var_correct_w1_dn10, var_correct_w1_dn11, var_correct_w1_dn12,)
    }
};
        var_correct_w1 = assign22260_e27624;
        var_correct_w1_dn0 = assign22260_e27624_d_n0;
        var_correct_w1_dn2 = assign22260_e27624_d_n2;
        var_correct_w1_dn4 = assign22260_e27624_d_n4;
        var_correct_w1_dn5 = assign22260_e27624_d_n5;
        var_correct_w1_dn6 = assign22260_e27624_d_n6;
        var_correct_w1_dn8 = assign22260_e27624_d_n8;
        var_correct_w1_dn10 = assign22260_e27624_d_n10;
        var_correct_w1_dn11 = assign22260_e27624_d_n11;
        var_correct_w1_dn12 = assign22260_e27624_d_n12;

        let (assign22270_e27633, assign22270_e27633_d_n0, assign22270_e27633_d_n2, assign22270_e27633_d_n4, assign22270_e27633_d_n5, assign22270_e27633_d_n6, assign22270_e27633_d_n8, assign22270_e27633_d_n10, assign22270_e27633_d_n11, assign22270_e27633_d_n12,) = {
    if ((var_guard378 != 0.0) && (var_guard379 == 0.0)) {
        let assign22270_e27631: f64 = (var_muun / var_mud_hoso);
        (assign22270_e27631, (((var_muun_dn0 * var_mud_hoso) - (var_muun * var_mud_hoso_dn0)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn2 * var_mud_hoso) - (var_muun * var_mud_hoso_dn2)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn4 * var_mud_hoso) - (var_muun * var_mud_hoso_dn4)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn5 * var_mud_hoso) - (var_muun * var_mud_hoso_dn5)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn6 * var_mud_hoso) - (var_muun * var_mud_hoso_dn6)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn8 * var_mud_hoso) - (var_muun * var_mud_hoso_dn8)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn10 * var_mud_hoso) - (var_muun * var_mud_hoso_dn10)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn11 * var_mud_hoso) - (var_muun * var_mud_hoso_dn11)) / (var_mud_hoso * var_mud_hoso)), (((var_muun_dn12 * var_mud_hoso) - (var_muun * var_mud_hoso_dn12)) / (var_mud_hoso * var_mud_hoso)),)
    } else {
        (var_correct_w1, var_correct_w1_dn0, var_correct_w1_dn2, var_correct_w1_dn4, var_correct_w1_dn5, var_correct_w1_dn6, var_correct_w1_dn8, var_correct_w1_dn10, var_correct_w1_dn11, var_correct_w1_dn12,)
    }
};
        var_correct_w1 = assign22270_e27633;
        var_correct_w1_dn0 = assign22270_e27633_d_n0;
        var_correct_w1_dn2 = assign22270_e27633_d_n2;
        var_correct_w1_dn4 = assign22270_e27633_d_n4;
        var_correct_w1_dn5 = assign22270_e27633_d_n5;
        var_correct_w1_dn6 = assign22270_e27633_d_n6;
        var_correct_w1_dn8 = assign22270_e27633_d_n8;
        var_correct_w1_dn10 = assign22270_e27633_d_n10;
        var_correct_w1_dn11 = assign22270_e27633_d_n11;
        var_correct_w1_dn12 = assign22270_e27633_d_n12;

        let (assign22280_e27643, assign22280_e27643_d_n0, assign22280_e27643_d_n2, assign22280_e27643_d_n4, assign22280_e27643_d_n5, assign22280_e27643_d_n6, assign22280_e27643_d_n8, assign22280_e27643_d_n10, assign22280_e27643_d_n11, assign22280_e27643_d_n12,) = {
    if (var_guard378 != 0.0) {
        let assign22280_e27637: f64 = (var_mfactor * var_nign0);
        let assign22280_e27639: f64 = (assign22280_e27637 * var_kusai_ig);
        let assign22280_e27641: f64 = (assign22280_e27639 * var_correct_w1);
        (assign22280_e27641, (((((var_mfactor * var_nign0_dn0) * var_kusai_ig) + (assign22280_e27637 * var_kusai_ig_dn0)) * var_correct_w1) + (assign22280_e27639 * var_correct_w1_dn0)), (((((var_mfactor * var_nign0_dn2) * var_kusai_ig) + (assign22280_e27637 * var_kusai_ig_dn2)) * var_correct_w1) + (assign22280_e27639 * var_correct_w1_dn2)), (((((var_mfactor * var_nign0_dn4) * var_kusai_ig) + (assign22280_e27637 * var_kusai_ig_dn4)) * var_correct_w1) + (assign22280_e27639 * var_correct_w1_dn4)), (((((var_mfactor * var_nign0_dn5) * var_kusai_ig) + (assign22280_e27637 * var_kusai_ig_dn5)) * var_correct_w1) + (assign22280_e27639 * var_correct_w1_dn5)), (((((var_mfactor * var_nign0_dn6) * var_kusai_ig) + (assign22280_e27637 * var_kusai_ig_dn6)) * var_correct_w1) + (assign22280_e27639 * var_correct_w1_dn6)), (((((var_mfactor * var_nign0_dn8) * var_kusai_ig) + (assign22280_e27637 * var_kusai_ig_dn8)) * var_correct_w1) + (assign22280_e27639 * var_correct_w1_dn8)), (((((var_mfactor * var_nign0_dn10) * var_kusai_ig) + (assign22280_e27637 * var_kusai_ig_dn10)) * var_correct_w1) + (assign22280_e27639 * var_correct_w1_dn10)), (((((var_mfactor * var_nign0_dn11) * var_kusai_ig) + (assign22280_e27637 * var_kusai_ig_dn11)) * var_correct_w1) + (assign22280_e27639 * var_correct_w1_dn11)), (((((var_mfactor * var_nign0_dn12) * var_kusai_ig) + (assign22280_e27637 * var_kusai_ig_dn12)) * var_correct_w1) + (assign22280_e27639 * var_correct_w1_dn12)),)
    } else {
        (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn4, var_noiigate_dn5, var_noiigate_dn6, var_noiigate_dn8, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12,)
    }
};
        var_noiigate = assign22280_e27643;
        var_noiigate_dn0 = assign22280_e27643_d_n0;
        var_noiigate_dn2 = assign22280_e27643_d_n2;
        var_noiigate_dn4 = assign22280_e27643_d_n4;
        var_noiigate_dn5 = assign22280_e27643_d_n5;
        var_noiigate_dn6 = assign22280_e27643_d_n6;
        var_noiigate_dn8 = assign22280_e27643_d_n8;
        var_noiigate_dn10 = assign22280_e27643_d_n10;
        var_noiigate_dn11 = assign22280_e27643_d_n11;
        var_noiigate_dn12 = assign22280_e27643_d_n12;

        let (assign22290_e27647, assign22290_e27647_d_n0, assign22290_e27647_d_n2, assign22290_e27647_d_n4, assign22290_e27647_d_n5, assign22290_e27647_d_n6, assign22290_e27647_d_n8, assign22290_e27647_d_n10, assign22290_e27647_d_n11, assign22290_e27647_d_n12,) = {
    if (var_guard378 != 0.0) {
        (var_crl_f, var_crl_f_dn0, var_crl_f_dn2, var_crl_f_dn4, var_crl_f_dn5, var_crl_f_dn6, var_crl_f_dn8, var_crl_f_dn10, var_crl_f_dn11, var_crl_f_dn12,)
    } else {
        (var_noicross, var_noicross_dn0, var_noicross_dn2, var_noicross_dn4, var_noicross_dn5, var_noicross_dn6, var_noicross_dn8, var_noicross_dn10, var_noicross_dn11, var_noicross_dn12,)
    }
};
        var_noicross = assign22290_e27647;
        var_noicross_dn0 = assign22290_e27647_d_n0;
        var_noicross_dn2 = assign22290_e27647_d_n2;
        var_noicross_dn4 = assign22290_e27647_d_n4;
        var_noicross_dn5 = assign22290_e27647_d_n5;
        var_noicross_dn6 = assign22290_e27647_d_n6;
        var_noicross_dn8 = assign22290_e27647_d_n8;
        var_noicross_dn10 = assign22290_e27647_d_n10;
        var_noicross_dn11 = assign22290_e27647_d_n11;
        var_noicross_dn12 = assign22290_e27647_d_n12;

        let (assign22300_e27656, assign22300_e27656_d_n0, assign22300_e27656_d_n2, assign22300_e27656_d_n4, assign22300_e27656_d_n5, assign22300_e27656_d_n6, assign22300_e27656_d_n8, assign22300_e27656_d_n10, assign22300_e27656_d_n11, assign22300_e27656_d_n12,) = {
    if (var_guard378 != 0.0) {
        let (assign22300_e27654, assign22300_e27654_d_n0, assign22300_e27654_d_n2, assign22300_e27654_d_n4, assign22300_e27654_d_n5, assign22300_e27654_d_n6, assign22300_e27654_d_n8, assign22300_e27654_d_n10, assign22300_e27654_d_n11, assign22300_e27654_d_n12,) = {
            if (var_noiigate < 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn4, var_noiigate_dn5, var_noiigate_dn6, var_noiigate_dn8, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12,)
            }
        };
        (assign22300_e27654, assign22300_e27654_d_n0, assign22300_e27654_d_n2, assign22300_e27654_d_n4, assign22300_e27654_d_n5, assign22300_e27654_d_n6, assign22300_e27654_d_n8, assign22300_e27654_d_n10, assign22300_e27654_d_n11, assign22300_e27654_d_n12,)
    } else {
        (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn4, var_noiigate_dn5, var_noiigate_dn6, var_noiigate_dn8, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12,)
    }
};
        var_noiigate = assign22300_e27656;
        var_noiigate_dn0 = assign22300_e27656_d_n0;
        var_noiigate_dn2 = assign22300_e27656_d_n2;
        var_noiigate_dn4 = assign22300_e27656_d_n4;
        var_noiigate_dn5 = assign22300_e27656_d_n5;
        var_noiigate_dn6 = assign22300_e27656_d_n6;
        var_noiigate_dn8 = assign22300_e27656_d_n8;
        var_noiigate_dn10 = assign22300_e27656_d_n10;
        var_noiigate_dn11 = assign22300_e27656_d_n11;
        var_noiigate_dn12 = assign22300_e27656_d_n12;

        let (assign22310_e27666, assign22310_e27666_d_n0, assign22310_e27666_d_n2, assign22310_e27666_d_n4, assign22310_e27666_d_n5, assign22310_e27666_d_n6, assign22310_e27666_d_n8, assign22310_e27666_d_n10, assign22310_e27666_d_n11, assign22310_e27666_d_n12,) = {
    if (var_guard378 != 0.0) {
        let assign22310_e27659: f64 = (-var_t10);
        let (assign22310_e27664, assign22310_e27664_d_n0, assign22310_e27664_d_n2, assign22310_e27664_d_n4, assign22310_e27664_d_n5, assign22310_e27664_d_n6, assign22310_e27664_d_n8, assign22310_e27664_d_n10, assign22310_e27664_d_n11, assign22310_e27664_d_n12,) = {
            if (assign22310_e27659 > var_t0) {
                (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn4, var_noiigate_dn5, var_noiigate_dn6, var_noiigate_dn8, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign22310_e27664, assign22310_e27664_d_n0, assign22310_e27664_d_n2, assign22310_e27664_d_n4, assign22310_e27664_d_n5, assign22310_e27664_d_n6, assign22310_e27664_d_n8, assign22310_e27664_d_n10, assign22310_e27664_d_n11, assign22310_e27664_d_n12,)
    } else {
        (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn4, var_noiigate_dn5, var_noiigate_dn6, var_noiigate_dn8, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12,)
    }
};
        var_noiigate = assign22310_e27666;
        var_noiigate_dn0 = assign22310_e27666_d_n0;
        var_noiigate_dn2 = assign22310_e27666_d_n2;
        var_noiigate_dn4 = assign22310_e27666_d_n4;
        var_noiigate_dn5 = assign22310_e27666_d_n5;
        var_noiigate_dn6 = assign22310_e27666_d_n6;
        var_noiigate_dn8 = assign22310_e27666_d_n8;
        var_noiigate_dn10 = assign22310_e27666_d_n10;
        var_noiigate_dn11 = assign22310_e27666_d_n11;
        var_noiigate_dn12 = assign22310_e27666_d_n12;

        let (assign22320_e27676, assign22320_e27676_d_n0, assign22320_e27676_d_n2, assign22320_e27676_d_n4, assign22320_e27676_d_n5, assign22320_e27676_d_n6, assign22320_e27676_d_n8, assign22320_e27676_d_n10, assign22320_e27676_d_n11, assign22320_e27676_d_n12,) = {
    if (var_guard378 != 0.0) {
        let assign22320_e27669: f64 = (-var_t10);
        let (assign22320_e27674, assign22320_e27674_d_n0, assign22320_e27674_d_n2, assign22320_e27674_d_n4, assign22320_e27674_d_n5, assign22320_e27674_d_n6, assign22320_e27674_d_n8, assign22320_e27674_d_n10, assign22320_e27674_d_n11, assign22320_e27674_d_n12,) = {
            if (assign22320_e27669 > var_t0) {
                (var_noicross, var_noicross_dn0, var_noicross_dn2, var_noicross_dn4, var_noicross_dn5, var_noicross_dn6, var_noicross_dn8, var_noicross_dn10, var_noicross_dn11, var_noicross_dn12,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign22320_e27674, assign22320_e27674_d_n0, assign22320_e27674_d_n2, assign22320_e27674_d_n4, assign22320_e27674_d_n5, assign22320_e27674_d_n6, assign22320_e27674_d_n8, assign22320_e27674_d_n10, assign22320_e27674_d_n11, assign22320_e27674_d_n12,)
    } else {
        (var_noicross, var_noicross_dn0, var_noicross_dn2, var_noicross_dn4, var_noicross_dn5, var_noicross_dn6, var_noicross_dn8, var_noicross_dn10, var_noicross_dn11, var_noicross_dn12,)
    }
};
        var_noicross = assign22320_e27676;
        var_noicross_dn0 = assign22320_e27676_d_n0;
        var_noicross_dn2 = assign22320_e27676_d_n2;
        var_noicross_dn4 = assign22320_e27676_d_n4;
        var_noicross_dn5 = assign22320_e27676_d_n5;
        var_noicross_dn6 = assign22320_e27676_d_n6;
        var_noicross_dn8 = assign22320_e27676_d_n8;
        var_noicross_dn10 = assign22320_e27676_d_n10;
        var_noicross_dn11 = assign22320_e27676_d_n11;
        var_noicross_dn12 = assign22320_e27676_d_n12;

        let (assign22330_e27681, assign22330_e27681_d_n0, assign22330_e27681_d_n2, assign22330_e27681_d_n4, assign22330_e27681_d_n5, assign22330_e27681_d_n6, assign22330_e27681_d_n8, assign22330_e27681_d_n10, assign22330_e27681_d_n11, assign22330_e27681_d_n12,) = {
    if (var_guard378 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_noiigate, var_noiigate_dn0, var_noiigate_dn2, var_noiigate_dn4, var_noiigate_dn5, var_noiigate_dn6, var_noiigate_dn8, var_noiigate_dn10, var_noiigate_dn11, var_noiigate_dn12,)
    }
};
        var_noiigate = assign22330_e27681;
        var_noiigate_dn0 = assign22330_e27681_d_n0;
        var_noiigate_dn2 = assign22330_e27681_d_n2;
        var_noiigate_dn4 = assign22330_e27681_d_n4;
        var_noiigate_dn5 = assign22330_e27681_d_n5;
        var_noiigate_dn6 = assign22330_e27681_d_n6;
        var_noiigate_dn8 = assign22330_e27681_d_n8;
        var_noiigate_dn10 = assign22330_e27681_d_n10;
        var_noiigate_dn11 = assign22330_e27681_d_n11;
        var_noiigate_dn12 = assign22330_e27681_d_n12;

        let (assign22340_e27686, assign22340_e27686_d_n0, assign22340_e27686_d_n2, assign22340_e27686_d_n4, assign22340_e27686_d_n5, assign22340_e27686_d_n6, assign22340_e27686_d_n8, assign22340_e27686_d_n10, assign22340_e27686_d_n11, assign22340_e27686_d_n12,) = {
    if (var_guard378 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_noicross, var_noicross_dn0, var_noicross_dn2, var_noicross_dn4, var_noicross_dn5, var_noicross_dn6, var_noicross_dn8, var_noicross_dn10, var_noicross_dn11, var_noicross_dn12,)
    }
};
        var_noicross = assign22340_e27686;
        var_noicross_dn0 = assign22340_e27686_d_n0;
        var_noicross_dn2 = assign22340_e27686_d_n2;
        var_noicross_dn4 = assign22340_e27686_d_n4;
        var_noicross_dn5 = assign22340_e27686_d_n5;
        var_noicross_dn6 = assign22340_e27686_d_n6;
        var_noicross_dn8 = assign22340_e27686_d_n8;
        var_noicross_dn10 = assign22340_e27686_d_n10;
        var_noicross_dn11 = assign22340_e27686_d_n11;
        var_noicross_dn12 = assign22340_e27686_d_n12;

        let assign22350_e27689: f64 = (var_whi_noise * var_noithrml);
        var_sid = assign22350_e27689;
        var_sid_dn0 = (var_whi_noise * var_noithrml_dn0);
        var_sid_dn2 = (var_whi_noise * var_noithrml_dn2);
        var_sid_dn4 = ((var_whi_noise_dn4 * var_noithrml) + (var_whi_noise * var_noithrml_dn4));
        var_sid_dn5 = (var_whi_noise * var_noithrml_dn5);
        var_sid_dn6 = (var_whi_noise * var_noithrml_dn6);
        var_sid_dn8 = (var_whi_noise * var_noithrml_dn8);
        var_sid_dn10 = (var_whi_noise * var_noithrml_dn10);
        var_sid_dn11 = (var_whi_noise * var_noithrml_dn11);
        var_sid_dn12 = (var_whi_noise * var_noithrml_dn12);

        var_ci = var_noicross;
        var_ci_dn0 = var_noicross_dn0;
        var_ci_dn2 = var_noicross_dn2;
        var_ci_dn4 = var_noicross_dn4;
        var_ci_dn5 = var_noicross_dn5;
        var_ci_dn6 = var_noicross_dn6;
        var_ci_dn8 = var_noicross_dn8;
        var_ci_dn10 = var_noicross_dn10;
        var_ci_dn11 = var_noicross_dn11;
        var_ci_dn12 = var_noicross_dn12;

        let (assign22370_e27703, assign22370_e27703_d_n0, assign22370_e27703_d_n2, assign22370_e27703_d_n4, assign22370_e27703_d_n5, assign22370_e27703_d_n6, assign22370_e27703_d_n8, assign22370_e27703_d_n10, assign22370_e27703_d_n11, assign22370_e27703_d_n12,) = {
    if ((var_sid > 0.0) && (var_noiigate > 0.0)) {
        let assign22370_e27700: f64 = (var_noiigate / var_sid);
        let assign22370_e27701: f64 = (assign22370_e27700).sqrt();
        (assign22370_e27701, ((((var_noiigate_dn0 * var_sid) - (var_noiigate * var_sid_dn0)) / (var_sid * var_sid)) / (2.0 * assign22370_e27701)), ((((var_noiigate_dn2 * var_sid) - (var_noiigate * var_sid_dn2)) / (var_sid * var_sid)) / (2.0 * assign22370_e27701)), ((((var_noiigate_dn4 * var_sid) - (var_noiigate * var_sid_dn4)) / (var_sid * var_sid)) / (2.0 * assign22370_e27701)), ((((var_noiigate_dn5 * var_sid) - (var_noiigate * var_sid_dn5)) / (var_sid * var_sid)) / (2.0 * assign22370_e27701)), ((((var_noiigate_dn6 * var_sid) - (var_noiigate * var_sid_dn6)) / (var_sid * var_sid)) / (2.0 * assign22370_e27701)), ((((var_noiigate_dn8 * var_sid) - (var_noiigate * var_sid_dn8)) / (var_sid * var_sid)) / (2.0 * assign22370_e27701)), ((((var_noiigate_dn10 * var_sid) - (var_noiigate * var_sid_dn10)) / (var_sid * var_sid)) / (2.0 * assign22370_e27701)), ((((var_noiigate_dn11 * var_sid) - (var_noiigate * var_sid_dn11)) / (var_sid * var_sid)) / (2.0 * assign22370_e27701)), ((((var_noiigate_dn12 * var_sid) - (var_noiigate * var_sid_dn12)) / (var_sid * var_sid)) / (2.0 * assign22370_e27701)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_sigrat = assign22370_e27703;
        var_sigrat_dn0 = assign22370_e27703_d_n0;
        var_sigrat_dn2 = assign22370_e27703_d_n2;
        var_sigrat_dn4 = assign22370_e27703_d_n4;
        var_sigrat_dn5 = assign22370_e27703_d_n5;
        var_sigrat_dn6 = assign22370_e27703_d_n6;
        var_sigrat_dn8 = assign22370_e27703_d_n8;
        var_sigrat_dn10 = assign22370_e27703_d_n10;
        var_sigrat_dn11 = assign22370_e27703_d_n11;
        var_sigrat_dn12 = assign22370_e27703_d_n12;

        let (assign22380_e27715, assign22380_e27715_d_n0, assign22380_e27715_d_n2, assign22380_e27715_d_n4, assign22380_e27715_d_n5, assign22380_e27715_d_n6, assign22380_e27715_d_n8, assign22380_e27715_d_n10, assign22380_e27715_d_n11, assign22380_e27715_d_n12,) = {
    if (var_mode > 0.0) {
        let assign22380_e27710: f64 = (1.0 - var_qdrat);
        let assign22380_e27711: f64 = (var_sigrat * assign22380_e27710);
        (assign22380_e27711, (var_sigrat_dn0 * assign22380_e27710), (var_sigrat_dn2 * assign22380_e27710), (var_sigrat_dn4 * assign22380_e27710), (var_sigrat_dn5 * assign22380_e27710), (var_sigrat_dn6 * assign22380_e27710), (var_sigrat_dn8 * assign22380_e27710), (var_sigrat_dn10 * assign22380_e27710), (var_sigrat_dn11 * assign22380_e27710), (var_sigrat_dn12 * assign22380_e27710),)
    } else {
        let assign22380_e27714: f64 = (var_sigrat * var_qdrat);
        (assign22380_e27714, (var_sigrat_dn0 * var_qdrat), (var_sigrat_dn2 * var_qdrat), (var_sigrat_dn4 * var_qdrat), (var_sigrat_dn5 * var_qdrat), (var_sigrat_dn6 * var_qdrat), (var_sigrat_dn8 * var_qdrat), (var_sigrat_dn10 * var_qdrat), (var_sigrat_dn11 * var_qdrat), (var_sigrat_dn12 * var_qdrat),)
    }
};
        var_sigrat_s = assign22380_e27715;
        var_sigrat_s_dn0 = assign22380_e27715_d_n0;
        var_sigrat_s_dn2 = assign22380_e27715_d_n2;
        var_sigrat_s_dn4 = assign22380_e27715_d_n4;
        var_sigrat_s_dn5 = assign22380_e27715_d_n5;
        var_sigrat_s_dn6 = assign22380_e27715_d_n6;
        var_sigrat_s_dn8 = assign22380_e27715_d_n8;
        var_sigrat_s_dn10 = assign22380_e27715_d_n10;
        var_sigrat_s_dn11 = assign22380_e27715_d_n11;
        var_sigrat_s_dn12 = assign22380_e27715_d_n12;

        let (assign22390_e27727, assign22390_e27727_d_n0, assign22390_e27727_d_n2, assign22390_e27727_d_n4, assign22390_e27727_d_n5, assign22390_e27727_d_n6, assign22390_e27727_d_n8, assign22390_e27727_d_n10, assign22390_e27727_d_n11, assign22390_e27727_d_n12,) = {
    if (var_mode > 0.0) {
        let assign22390_e27721: f64 = (var_sigrat * var_qdrat);
        (assign22390_e27721, (var_sigrat_dn0 * var_qdrat), (var_sigrat_dn2 * var_qdrat), (var_sigrat_dn4 * var_qdrat), (var_sigrat_dn5 * var_qdrat), (var_sigrat_dn6 * var_qdrat), (var_sigrat_dn8 * var_qdrat), (var_sigrat_dn10 * var_qdrat), (var_sigrat_dn11 * var_qdrat), (var_sigrat_dn12 * var_qdrat),)
    } else {
        let assign22390_e27725: f64 = (1.0 - var_qdrat);
        let assign22390_e27726: f64 = (var_sigrat * assign22390_e27725);
        (assign22390_e27726, (var_sigrat_dn0 * assign22390_e27725), (var_sigrat_dn2 * assign22390_e27725), (var_sigrat_dn4 * assign22390_e27725), (var_sigrat_dn5 * assign22390_e27725), (var_sigrat_dn6 * assign22390_e27725), (var_sigrat_dn8 * assign22390_e27725), (var_sigrat_dn10 * assign22390_e27725), (var_sigrat_dn11 * assign22390_e27725), (var_sigrat_dn12 * assign22390_e27725),)
    }
};
        var_sigrat_d = assign22390_e27727;
        var_sigrat_d_dn0 = assign22390_e27727_d_n0;
        var_sigrat_d_dn2 = assign22390_e27727_d_n2;
        var_sigrat_d_dn4 = assign22390_e27727_d_n4;
        var_sigrat_d_dn5 = assign22390_e27727_d_n5;
        var_sigrat_d_dn6 = assign22390_e27727_d_n6;
        var_sigrat_d_dn8 = assign22390_e27727_d_n8;
        var_sigrat_d_dn10 = assign22390_e27727_d_n10;
        var_sigrat_d_dn11 = assign22390_e27727_d_n11;
        var_sigrat_d_dn12 = assign22390_e27727_d_n12;

        var_rdde = 0.0;
        var_rdde_dn0 = 0.0;
        var_rdde_dn2 = 0.0;
        var_rdde_dn4 = 0.0;
        var_rdde_dn5 = 0.0;
        var_rdde_dn6 = 0.0;
        var_rdde_dn8 = 0.0;
        var_rdde_dn10 = 0.0;
        var_rdde_dn11 = 0.0;
        var_rdde_dn12 = 0.0;

        var_rsde = 0.0;
        var_rsde_dn0 = 0.0;
        var_rsde_dn2 = 0.0;
        var_rsde_dn4 = 0.0;
        var_rsde_dn5 = 0.0;
        var_rsde_dn6 = 0.0;
        var_rsde_dn8 = 0.0;
        var_rsde_dn10 = 0.0;
        var_rsde_dn11 = 0.0;
        var_rsde_dn12 = 0.0;

        let assign22440_e27734: f64 = if p.p312 == 1.0 { 1.0 } else { 0.0 };
        var_guard380 = assign22440_e27734;

        let (assign22450_e27740,) = {
    if (var_guard380 != 0.0) {
        let assign22450_e27738: f64 = (p.p315 / 1e-6);
        (assign22450_e27738,)
    } else {
        (var_nover,)
    }
};
        var_nover = assign22450_e27740;

        let (assign22460_e27744,) = {
    if (var_guard380 != 0.0) {
        (p.p317,)
    } else {
        (var_mks_rdrmue,)
    }
};
        var_mks_rdrmue = assign22460_e27744;

        let (assign22470_e27748,) = {
    if (var_guard380 != 0.0) {
        (p.p319,)
    } else {
        (var_mks_rdrvmax,)
    }
};
        var_mks_rdrvmax = assign22470_e27748;

        let (assign22480_e27752, assign22480_e27752_d_n4,) = {
    if (var_guard380 != 0.0) {
        (p.p324, 0.0,)
    } else {
        (var_rrdrbb, var_rrdrbb_dn4,)
    }
};
        var_rrdrbb = assign22480_e27752;
        var_rrdrbb_dn4 = assign22480_e27752_d_n4;

        let (assign22490_e27763,) = {
    if (var_guard380 != 0.0) {
        let (assign22490_e27761,) = {
            if (p.p314 > 0.0) {
                let assign22490_e27759: f64 = (p.p314 * p.p308);
                (assign22490_e27759,)
            } else {
                (0.0,)
            }
        };
        (assign22490_e27761,)
    } else {
        (var_rsd0,)
    }
};
        var_rsd0 = assign22490_e27763;

        let (assign22500_e27767,) = {
    if (var_guard380 != 0.0) {
        (p.p311,)
    } else {
        (var_ldrifte,)
    }
};
        var_ldrifte = assign22500_e27767;

        let (assign22510_e27773, assign22510_e27773_d_n2, assign22510_e27773_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22510_e27771: f64 = (p.p33 * (nv12 - nv2));
        (assign22510_e27771, (-p.p33), p.p33,)
    } else {
        (var_vrdr, var_vrdr_dn2, var_vrdr_dn12,)
    }
};
        var_vrdr = assign22510_e27773;
        var_vrdr_dn2 = assign22510_e27773_d_n2;
        var_vrdr_dn12 = assign22510_e27773_d_n12;

        let (assign22520_e27784,) = {
    if (var_guard380 != 0.0) {
        let assign22520_e27777: f64 = (p.p322 * p.p322);
        let assign22520_e27780: f64 = (p.p38 * p.p38);
        let assign22520_e27781: f64 = (assign22520_e27777 + assign22520_e27780);
        let assign22520_e27782: f64 = (assign22520_e27781).sqrt();
        (assign22520_e27782,)
    } else {
        (var_xov,)
    }
};
        var_xov = assign22520_e27784;

        let (assign22530_e27790, assign22530_e27790_d_n0, assign22530_e27790_d_n2, assign22530_e27790_d_n4, assign22530_e27790_d_n5, assign22530_e27790_d_n6, assign22530_e27790_d_n8, assign22530_e27790_d_n10, assign22530_e27790_d_n11, assign22530_e27790_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22530_e27788: f64 = (var_weff * p.p5);
        (assign22530_e27788, (var_weff_dn0 * p.p5), (var_weff_dn2 * p.p5), (var_weff_dn4 * p.p5), (var_weff_dn5 * p.p5), (var_weff_dn6 * p.p5), (var_weff_dn8 * p.p5), (var_weff_dn10 * p.p5), (var_weff_dn11 * p.p5), (var_weff_dn12 * p.p5),)
    } else {
        (var_weff_nf_1, var_weff_nf_1_dn0, var_weff_nf_1_dn2, var_weff_nf_1_dn4, var_weff_nf_1_dn5, var_weff_nf_1_dn6, var_weff_nf_1_dn8, var_weff_nf_1_dn10, var_weff_nf_1_dn11, var_weff_nf_1_dn12,)
    }
};
        var_weff_nf_1 = assign22530_e27790;
        var_weff_nf_1_dn0 = assign22530_e27790_d_n0;
        var_weff_nf_1_dn2 = assign22530_e27790_d_n2;
        var_weff_nf_1_dn4 = assign22530_e27790_d_n4;
        var_weff_nf_1_dn5 = assign22530_e27790_d_n5;
        var_weff_nf_1_dn6 = assign22530_e27790_d_n6;
        var_weff_nf_1_dn8 = assign22530_e27790_d_n8;
        var_weff_nf_1_dn10 = assign22530_e27790_d_n10;
        var_weff_nf_1_dn11 = assign22530_e27790_d_n11;
        var_weff_nf_1_dn12 = assign22530_e27790_d_n12;

        let (assign22540_e27796,) = {
    if (var_guard380 != 0.0) {
        let assign22540_e27794: f64 = (var_mks_rdrmue / 10000.0);
        (assign22540_e27794,)
    } else {
        (var_mks_rdrmue,)
    }
};
        var_mks_rdrmue = assign22540_e27796;

        let (assign22550_e27802,) = {
    if (var_guard380 != 0.0) {
        let assign22550_e27800: f64 = (var_mks_rdrvmax / 100.0);
        (assign22550_e27800,)
    } else {
        (var_mks_rdrvmax,)
    }
};
        var_mks_rdrvmax = assign22550_e27802;

        let (assign22560_e27808, assign22560_e27808_d_n4,) = {
    if (var_guard380 != 0.0) {
        let assign22560_e27806: f64 = (var_ttemp / var_uc_tnom);
        (assign22560_e27806, (var_ttemp_dn4 / var_uc_tnom),)
    } else {
        (var_tratio, var_tratio_dn4,)
    }
};
        var_tratio = assign22560_e27808;
        var_tratio_dn4 = assign22560_e27808_d_n4;

        let (assign22570_e27814, assign22570_e27814_d_n0, assign22570_e27814_d_n2, assign22570_e27814_d_n4, assign22570_e27814_d_n5, assign22570_e27814_d_n6, assign22570_e27814_d_n8, assign22570_e27814_d_n10, assign22570_e27814_d_n11, assign22570_e27814_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22570_e27812: f64 = (var_tratio).powf(p.p320);
        (assign22570_e27812, 0.0, 0.0, if 0.0 == 0.0 && ((p.p320) as f64).is_finite() && ((p.p320) as f64).fract() == 0.0 { if p.p320 == 0.0 { 0.0 } else { (p.p320 * ((var_tratio).powf(p.p320 - 1.0) * var_tratio_dn4)) } } else { (assign22570_e27812 * (p.p320 * (var_tratio_dn4 / var_tratio))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign22570_e27814;
        var_t1_dn0 = assign22570_e27814_d_n0;
        var_t1_dn2 = assign22570_e27814_d_n2;
        var_t1_dn4 = assign22570_e27814_d_n4;
        var_t1_dn5 = assign22570_e27814_d_n5;
        var_t1_dn6 = assign22570_e27814_d_n6;
        var_t1_dn8 = assign22570_e27814_d_n8;
        var_t1_dn10 = assign22570_e27814_d_n10;
        var_t1_dn11 = assign22570_e27814_d_n11;
        var_t1_dn12 = assign22570_e27814_d_n12;

        *var_ci_slot = var_ci;
        *var_ci_dn0_slot = var_ci_dn0;
        *var_ci_dn10_slot = var_ci_dn10;
        *var_ci_dn11_slot = var_ci_dn11;
        *var_ci_dn12_slot = var_ci_dn12;
        *var_ci_dn2_slot = var_ci_dn2;
        *var_ci_dn4_slot = var_ci_dn4;
        *var_ci_dn5_slot = var_ci_dn5;
        *var_ci_dn6_slot = var_ci_dn6;
        *var_ci_dn8_slot = var_ci_dn8;
        *var_correct_w1_slot = var_correct_w1;
        *var_correct_w1_dn0_slot = var_correct_w1_dn0;
        *var_correct_w1_dn10_slot = var_correct_w1_dn10;
        *var_correct_w1_dn11_slot = var_correct_w1_dn11;
        *var_correct_w1_dn12_slot = var_correct_w1_dn12;
        *var_correct_w1_dn2_slot = var_correct_w1_dn2;
        *var_correct_w1_dn4_slot = var_correct_w1_dn4;
        *var_correct_w1_dn5_slot = var_correct_w1_dn5;
        *var_correct_w1_dn6_slot = var_correct_w1_dn6;
        *var_correct_w1_dn8_slot = var_correct_w1_dn8;
        *var_guard380_slot = var_guard380;
        *var_ldrifte_slot = var_ldrifte;
        *var_mks_rdrmue_slot = var_mks_rdrmue;
        *var_mks_rdrvmax_slot = var_mks_rdrvmax;
        *var_mumoda_slot = var_mumoda;
        *var_mumoda_dn0_slot = var_mumoda_dn0;
        *var_mumoda_dn10_slot = var_mumoda_dn10;
        *var_mumoda_dn11_slot = var_mumoda_dn11;
        *var_mumoda_dn12_slot = var_mumoda_dn12;
        *var_mumoda_dn2_slot = var_mumoda_dn2;
        *var_mumoda_dn4_slot = var_mumoda_dn4;
        *var_mumoda_dn5_slot = var_mumoda_dn5;
        *var_mumoda_dn6_slot = var_mumoda_dn6;
        *var_mumoda_dn8_slot = var_mumoda_dn8;
        *var_mumodb_slot = var_mumodb;
        *var_mumodb_dn0_slot = var_mumodb_dn0;
        *var_mumodb_dn10_slot = var_mumodb_dn10;
        *var_mumodb_dn11_slot = var_mumodb_dn11;
        *var_mumodb_dn12_slot = var_mumodb_dn12;
        *var_mumodb_dn2_slot = var_mumodb_dn2;
        *var_mumodb_dn4_slot = var_mumodb_dn4;
        *var_mumodb_dn5_slot = var_mumodb_dn5;
        *var_mumodb_dn6_slot = var_mumodb_dn6;
        *var_mumodb_dn8_slot = var_mumodb_dn8;
        *var_noicross_slot = var_noicross;
        *var_noicross_dn0_slot = var_noicross_dn0;
        *var_noicross_dn10_slot = var_noicross_dn10;
        *var_noicross_dn11_slot = var_noicross_dn11;
        *var_noicross_dn12_slot = var_noicross_dn12;
        *var_noicross_dn2_slot = var_noicross_dn2;
        *var_noicross_dn4_slot = var_noicross_dn4;
        *var_noicross_dn5_slot = var_noicross_dn5;
        *var_noicross_dn6_slot = var_noicross_dn6;
        *var_noicross_dn8_slot = var_noicross_dn8;
        *var_noiigate_slot = var_noiigate;
        *var_noiigate_dn0_slot = var_noiigate_dn0;
        *var_noiigate_dn10_slot = var_noiigate_dn10;
        *var_noiigate_dn11_slot = var_noiigate_dn11;
        *var_noiigate_dn12_slot = var_noiigate_dn12;
        *var_noiigate_dn2_slot = var_noiigate_dn2;
        *var_noiigate_dn4_slot = var_noiigate_dn4;
        *var_noiigate_dn5_slot = var_noiigate_dn5;
        *var_noiigate_dn6_slot = var_noiigate_dn6;
        *var_noiigate_dn8_slot = var_noiigate_dn8;
        *var_nover_slot = var_nover;
        *var_rdde_slot = var_rdde;
        *var_rdde_dn0_slot = var_rdde_dn0;
        *var_rdde_dn10_slot = var_rdde_dn10;
        *var_rdde_dn11_slot = var_rdde_dn11;
        *var_rdde_dn12_slot = var_rdde_dn12;
        *var_rdde_dn2_slot = var_rdde_dn2;
        *var_rdde_dn4_slot = var_rdde_dn4;
        *var_rdde_dn5_slot = var_rdde_dn5;
        *var_rdde_dn6_slot = var_rdde_dn6;
        *var_rdde_dn8_slot = var_rdde_dn8;
        *var_rrdrbb_slot = var_rrdrbb;
        *var_rrdrbb_dn4_slot = var_rrdrbb_dn4;
        *var_rsd0_slot = var_rsd0;
        *var_rsde_slot = var_rsde;
        *var_rsde_dn0_slot = var_rsde_dn0;
        *var_rsde_dn10_slot = var_rsde_dn10;
        *var_rsde_dn11_slot = var_rsde_dn11;
        *var_rsde_dn12_slot = var_rsde_dn12;
        *var_rsde_dn2_slot = var_rsde_dn2;
        *var_rsde_dn4_slot = var_rsde_dn4;
        *var_rsde_dn5_slot = var_rsde_dn5;
        *var_rsde_dn6_slot = var_rsde_dn6;
        *var_rsde_dn8_slot = var_rsde_dn8;
        *var_sid_slot = var_sid;
        *var_sid_dn0_slot = var_sid_dn0;
        *var_sid_dn10_slot = var_sid_dn10;
        *var_sid_dn11_slot = var_sid_dn11;
        *var_sid_dn12_slot = var_sid_dn12;
        *var_sid_dn2_slot = var_sid_dn2;
        *var_sid_dn4_slot = var_sid_dn4;
        *var_sid_dn5_slot = var_sid_dn5;
        *var_sid_dn6_slot = var_sid_dn6;
        *var_sid_dn8_slot = var_sid_dn8;
        *var_sigrat_slot = var_sigrat;
        *var_sigrat_d_slot = var_sigrat_d;
        *var_sigrat_d_dn0_slot = var_sigrat_d_dn0;
        *var_sigrat_d_dn10_slot = var_sigrat_d_dn10;
        *var_sigrat_d_dn11_slot = var_sigrat_d_dn11;
        *var_sigrat_d_dn12_slot = var_sigrat_d_dn12;
        *var_sigrat_d_dn2_slot = var_sigrat_d_dn2;
        *var_sigrat_d_dn4_slot = var_sigrat_d_dn4;
        *var_sigrat_d_dn5_slot = var_sigrat_d_dn5;
        *var_sigrat_d_dn6_slot = var_sigrat_d_dn6;
        *var_sigrat_d_dn8_slot = var_sigrat_d_dn8;
        *var_sigrat_dn0_slot = var_sigrat_dn0;
        *var_sigrat_dn10_slot = var_sigrat_dn10;
        *var_sigrat_dn11_slot = var_sigrat_dn11;
        *var_sigrat_dn12_slot = var_sigrat_dn12;
        *var_sigrat_dn2_slot = var_sigrat_dn2;
        *var_sigrat_dn4_slot = var_sigrat_dn4;
        *var_sigrat_dn5_slot = var_sigrat_dn5;
        *var_sigrat_dn6_slot = var_sigrat_dn6;
        *var_sigrat_dn8_slot = var_sigrat_dn8;
        *var_sigrat_s_slot = var_sigrat_s;
        *var_sigrat_s_dn0_slot = var_sigrat_s_dn0;
        *var_sigrat_s_dn10_slot = var_sigrat_s_dn10;
        *var_sigrat_s_dn11_slot = var_sigrat_s_dn11;
        *var_sigrat_s_dn12_slot = var_sigrat_s_dn12;
        *var_sigrat_s_dn2_slot = var_sigrat_s_dn2;
        *var_sigrat_s_dn4_slot = var_sigrat_s_dn4;
        *var_sigrat_s_dn5_slot = var_sigrat_s_dn5;
        *var_sigrat_s_dn6_slot = var_sigrat_s_dn6;
        *var_sigrat_s_dn8_slot = var_sigrat_s_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_tratio_slot = var_tratio;
        *var_tratio_dn4_slot = var_tratio_dn4;
        *var_vrdr_slot = var_vrdr;
        *var_vrdr_dn12_slot = var_vrdr_dn12;
        *var_vrdr_dn2_slot = var_vrdr_dn2;
        *var_weff_nf_1_slot = var_weff_nf_1;
        *var_weff_nf_1_dn0_slot = var_weff_nf_1_dn0;
        *var_weff_nf_1_dn10_slot = var_weff_nf_1_dn10;
        *var_weff_nf_1_dn11_slot = var_weff_nf_1_dn11;
        *var_weff_nf_1_dn12_slot = var_weff_nf_1_dn12;
        *var_weff_nf_1_dn2_slot = var_weff_nf_1_dn2;
        *var_weff_nf_1_dn4_slot = var_weff_nf_1_dn4;
        *var_weff_nf_1_dn5_slot = var_weff_nf_1_dn5;
        *var_weff_nf_1_dn6_slot = var_weff_nf_1_dn6;
        *var_weff_nf_1_dn8_slot = var_weff_nf_1_dn8;
        *var_xov_slot = var_xov;
    }

    pub(super) fn stamp_transient_block_86(
        p: &Parameters,
        var_guard380: f64,
        var_ldrifte: f64,
        var_lg: f64,
        var_mks_rdrmue: f64,
        var_mks_rdrvmax: f64,
        var_nover: f64,
        var_tratio: f64,
        var_tratio_dn4: f64,
        var_ttemp: f64,
        var_ttemp_dn4: f64,
        var_uc_tnom: f64,
        var_vrdr: f64,
        var_vrdr_dn12: f64,
        var_vrdr_dn2: f64,
        var_wg: f64,
        var_xov: f64,
        var_edri_slot: &mut f64,
        var_edri_dn12_slot: &mut f64,
        var_edri_dn2_slot: &mut f64,
        var_gd_slot: &mut f64,
        var_gd_dn0_slot: &mut f64,
        var_gd_dn10_slot: &mut f64,
        var_gd_dn11_slot: &mut f64,
        var_gd_dn12_slot: &mut f64,
        var_gd_dn2_slot: &mut f64,
        var_gd_dn4_slot: &mut f64,
        var_gd_dn5_slot: &mut f64,
        var_gd_dn6_slot: &mut f64,
        var_gd_dn8_slot: &mut f64,
        var_guard400_slot: &mut f64,
        var_guard401_slot: &mut f64,
        var_guard402_slot: &mut f64,
        var_guard403_slot: &mut f64,
        var_guard404_slot: &mut f64,
        var_guard405_slot: &mut f64,
        var_mu0_slot: &mut f64,
        var_mu0_dn0_slot: &mut f64,
        var_mu0_dn10_slot: &mut f64,
        var_mu0_dn11_slot: &mut f64,
        var_mu0_dn12_slot: &mut f64,
        var_mu0_dn2_slot: &mut f64,
        var_mu0_dn4_slot: &mut f64,
        var_mu0_dn5_slot: &mut f64,
        var_mu0_dn6_slot: &mut f64,
        var_mu0_dn8_slot: &mut f64,
        var_mu__blk396_slot: &mut f64,
        var_mu__blk396_dn0_slot: &mut f64,
        var_mu__blk396_dn10_slot: &mut f64,
        var_mu__blk396_dn11_slot: &mut f64,
        var_mu__blk396_dn12_slot: &mut f64,
        var_mu__blk396_dn2_slot: &mut f64,
        var_mu__blk396_dn4_slot: &mut f64,
        var_mu__blk396_dn5_slot: &mut f64,
        var_mu__blk396_dn6_slot: &mut f64,
        var_mu__blk396_dn8_slot: &mut f64,
        var_rdrmuele_slot: &mut f64,
        var_rdrvmaxle_slot: &mut f64,
        var_rdrvmaxwe_slot: &mut f64,
        var_rrdrbb_slot: &mut f64,
        var_rrdrbb_dn4_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_vdri_slot: &mut f64,
        var_vdri_dn0_slot: &mut f64,
        var_vdri_dn10_slot: &mut f64,
        var_vdri_dn11_slot: &mut f64,
        var_vdri_dn12_slot: &mut f64,
        var_vdri_dn2_slot: &mut f64,
        var_vdri_dn4_slot: &mut f64,
        var_vdri_dn5_slot: &mut f64,
        var_vdri_dn6_slot: &mut f64,
        var_vdri_dn8_slot: &mut f64,
        var_vmaxe__blk393_slot: &mut f64,
        var_vmaxe__blk393_dn0_slot: &mut f64,
        var_vmaxe__blk393_dn10_slot: &mut f64,
        var_vmaxe__blk393_dn11_slot: &mut f64,
        var_vmaxe__blk393_dn12_slot: &mut f64,
        var_vmaxe__blk393_dn2_slot: &mut f64,
        var_vmaxe__blk393_dn4_slot: &mut f64,
        var_vmaxe__blk393_dn5_slot: &mut f64,
        var_vmaxe__blk393_dn6_slot: &mut f64,
        var_vmaxe__blk393_dn8_slot: &mut f64,
    ) {
        let mut var_edri: f64 = *var_edri_slot;
        let mut var_edri_dn12: f64 = *var_edri_dn12_slot;
        let mut var_edri_dn2: f64 = *var_edri_dn2_slot;
        let mut var_gd: f64 = *var_gd_slot;
        let mut var_gd_dn0: f64 = *var_gd_dn0_slot;
        let mut var_gd_dn10: f64 = *var_gd_dn10_slot;
        let mut var_gd_dn11: f64 = *var_gd_dn11_slot;
        let mut var_gd_dn12: f64 = *var_gd_dn12_slot;
        let mut var_gd_dn2: f64 = *var_gd_dn2_slot;
        let mut var_gd_dn4: f64 = *var_gd_dn4_slot;
        let mut var_gd_dn5: f64 = *var_gd_dn5_slot;
        let mut var_gd_dn6: f64 = *var_gd_dn6_slot;
        let mut var_gd_dn8: f64 = *var_gd_dn8_slot;
        let mut var_guard400: f64 = *var_guard400_slot;
        let mut var_guard401: f64 = *var_guard401_slot;
        let mut var_guard402: f64 = *var_guard402_slot;
        let mut var_guard403: f64 = *var_guard403_slot;
        let mut var_guard404: f64 = *var_guard404_slot;
        let mut var_guard405: f64 = *var_guard405_slot;
        let mut var_mu0: f64 = *var_mu0_slot;
        let mut var_mu0_dn0: f64 = *var_mu0_dn0_slot;
        let mut var_mu0_dn10: f64 = *var_mu0_dn10_slot;
        let mut var_mu0_dn11: f64 = *var_mu0_dn11_slot;
        let mut var_mu0_dn12: f64 = *var_mu0_dn12_slot;
        let mut var_mu0_dn2: f64 = *var_mu0_dn2_slot;
        let mut var_mu0_dn4: f64 = *var_mu0_dn4_slot;
        let mut var_mu0_dn5: f64 = *var_mu0_dn5_slot;
        let mut var_mu0_dn6: f64 = *var_mu0_dn6_slot;
        let mut var_mu0_dn8: f64 = *var_mu0_dn8_slot;
        let mut var_mu__blk396: f64 = *var_mu__blk396_slot;
        let mut var_mu__blk396_dn0: f64 = *var_mu__blk396_dn0_slot;
        let mut var_mu__blk396_dn10: f64 = *var_mu__blk396_dn10_slot;
        let mut var_mu__blk396_dn11: f64 = *var_mu__blk396_dn11_slot;
        let mut var_mu__blk396_dn12: f64 = *var_mu__blk396_dn12_slot;
        let mut var_mu__blk396_dn2: f64 = *var_mu__blk396_dn2_slot;
        let mut var_mu__blk396_dn4: f64 = *var_mu__blk396_dn4_slot;
        let mut var_mu__blk396_dn5: f64 = *var_mu__blk396_dn5_slot;
        let mut var_mu__blk396_dn6: f64 = *var_mu__blk396_dn6_slot;
        let mut var_mu__blk396_dn8: f64 = *var_mu__blk396_dn8_slot;
        let mut var_rdrmuele: f64 = *var_rdrmuele_slot;
        let mut var_rdrvmaxle: f64 = *var_rdrvmaxle_slot;
        let mut var_rdrvmaxwe: f64 = *var_rdrvmaxwe_slot;
        let mut var_rrdrbb: f64 = *var_rrdrbb_slot;
        let mut var_rrdrbb_dn4: f64 = *var_rrdrbb_dn4_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_vdri: f64 = *var_vdri_slot;
        let mut var_vdri_dn0: f64 = *var_vdri_dn0_slot;
        let mut var_vdri_dn10: f64 = *var_vdri_dn10_slot;
        let mut var_vdri_dn11: f64 = *var_vdri_dn11_slot;
        let mut var_vdri_dn12: f64 = *var_vdri_dn12_slot;
        let mut var_vdri_dn2: f64 = *var_vdri_dn2_slot;
        let mut var_vdri_dn4: f64 = *var_vdri_dn4_slot;
        let mut var_vdri_dn5: f64 = *var_vdri_dn5_slot;
        let mut var_vdri_dn6: f64 = *var_vdri_dn6_slot;
        let mut var_vdri_dn8: f64 = *var_vdri_dn8_slot;
        let mut var_vmaxe__blk393: f64 = *var_vmaxe__blk393_slot;
        let mut var_vmaxe__blk393_dn0: f64 = *var_vmaxe__blk393_dn0_slot;
        let mut var_vmaxe__blk393_dn10: f64 = *var_vmaxe__blk393_dn10_slot;
        let mut var_vmaxe__blk393_dn11: f64 = *var_vmaxe__blk393_dn11_slot;
        let mut var_vmaxe__blk393_dn12: f64 = *var_vmaxe__blk393_dn12_slot;
        let mut var_vmaxe__blk393_dn2: f64 = *var_vmaxe__blk393_dn2_slot;
        let mut var_vmaxe__blk393_dn4: f64 = *var_vmaxe__blk393_dn4_slot;
        let mut var_vmaxe__blk393_dn5: f64 = *var_vmaxe__blk393_dn5_slot;
        let mut var_vmaxe__blk393_dn6: f64 = *var_vmaxe__blk393_dn6_slot;
        let mut var_vmaxe__blk393_dn8: f64 = *var_vmaxe__blk393_dn8_slot;

        let (assign22580_e27820, assign22580_e27820_d_n0, assign22580_e27820_d_n2, assign22580_e27820_d_n4, assign22580_e27820_d_n5, assign22580_e27820_d_n6, assign22580_e27820_d_n8, assign22580_e27820_d_n10, assign22580_e27820_d_n11, assign22580_e27820_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22580_e27818: f64 = (var_mks_rdrmue / var_t1);
        (assign22580_e27818, (-((var_mks_rdrmue * var_t1_dn0) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn2) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn4) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn5) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn6) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn8) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn10) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn11) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn12) / (var_t1 * var_t1))),)
    } else {
        (var_mu0, var_mu0_dn0, var_mu0_dn2, var_mu0_dn4, var_mu0_dn5, var_mu0_dn6, var_mu0_dn8, var_mu0_dn10, var_mu0_dn11, var_mu0_dn12,)
    }
};
        var_mu0 = assign22580_e27820;
        var_mu0_dn0 = assign22580_e27820_d_n0;
        var_mu0_dn2 = assign22580_e27820_d_n2;
        var_mu0_dn4 = assign22580_e27820_d_n4;
        var_mu0_dn5 = assign22580_e27820_d_n5;
        var_mu0_dn6 = assign22580_e27820_d_n6;
        var_mu0_dn8 = assign22580_e27820_d_n8;
        var_mu0_dn10 = assign22580_e27820_d_n10;
        var_mu0_dn11 = assign22580_e27820_d_n11;
        var_mu0_dn12 = assign22580_e27820_d_n12;

        let (assign22590_e27840, assign22590_e27840_d_n0, assign22590_e27840_d_n2, assign22590_e27840_d_n4, assign22590_e27840_d_n5, assign22590_e27840_d_n6, assign22590_e27840_d_n8, assign22590_e27840_d_n10, assign22590_e27840_d_n11, assign22590_e27840_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22590_e27825: f64 = (0.4 * var_tratio);
        let assign22590_e27826: f64 = (1.8 + assign22590_e27825);
        let assign22590_e27829: f64 = (0.1 * var_tratio);
        let assign22590_e27831: f64 = (assign22590_e27829 * var_tratio);
        let assign22590_e27832: f64 = (assign22590_e27826 + assign22590_e27831);
        let assign22590_e27836: f64 = (1.0 - var_tratio);
        let assign22590_e27837: f64 = (p.p321 * assign22590_e27836);
        let assign22590_e27838: f64 = (assign22590_e27832 - assign22590_e27837);
        (assign22590_e27838, 0.0, 0.0, (((0.4 * var_tratio_dn4) + (((0.1 * var_tratio_dn4) * var_tratio) + (assign22590_e27829 * var_tratio_dn4))) - (p.p321 * (-var_tratio_dn4))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
        var_t0 = assign22590_e27840;
        var_t0_dn0 = assign22590_e27840_d_n0;
        var_t0_dn2 = assign22590_e27840_d_n2;
        var_t0_dn4 = assign22590_e27840_d_n4;
        var_t0_dn5 = assign22590_e27840_d_n5;
        var_t0_dn6 = assign22590_e27840_d_n6;
        var_t0_dn8 = assign22590_e27840_d_n8;
        var_t0_dn10 = assign22590_e27840_d_n10;
        var_t0_dn11 = assign22590_e27840_d_n11;
        var_t0_dn12 = assign22590_e27840_d_n12;

        let (assign22600_e27846, assign22600_e27846_d_n0, assign22600_e27846_d_n2, assign22600_e27846_d_n4, assign22600_e27846_d_n5, assign22600_e27846_d_n6, assign22600_e27846_d_n8, assign22600_e27846_d_n10, assign22600_e27846_d_n11, assign22600_e27846_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22600_e27844: f64 = (var_mks_rdrvmax / var_t0);
        (assign22600_e27844, (-((var_mks_rdrvmax * var_t0_dn0) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn2) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn4) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn5) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn6) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn8) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn10) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn11) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn12) / (var_t0 * var_t0))),)
    } else {
        (var_vmaxe__blk393, var_vmaxe__blk393_dn0, var_vmaxe__blk393_dn2, var_vmaxe__blk393_dn4, var_vmaxe__blk393_dn5, var_vmaxe__blk393_dn6, var_vmaxe__blk393_dn8, var_vmaxe__blk393_dn10, var_vmaxe__blk393_dn11, var_vmaxe__blk393_dn12,)
    }
};
        var_vmaxe__blk393 = assign22600_e27846;
        var_vmaxe__blk393_dn0 = assign22600_e27846_d_n0;
        var_vmaxe__blk393_dn2 = assign22600_e27846_d_n2;
        var_vmaxe__blk393_dn4 = assign22600_e27846_d_n4;
        var_vmaxe__blk393_dn5 = assign22600_e27846_d_n5;
        var_vmaxe__blk393_dn6 = assign22600_e27846_d_n6;
        var_vmaxe__blk393_dn8 = assign22600_e27846_d_n8;
        var_vmaxe__blk393_dn10 = assign22600_e27846_d_n10;
        var_vmaxe__blk393_dn11 = assign22600_e27846_d_n11;
        var_vmaxe__blk393_dn12 = assign22600_e27846_d_n12;

        let (assign22610_e27856, assign22610_e27856_d_n4,) = {
    if (var_guard380 != 0.0) {
        let assign22610_e27852: f64 = (var_ttemp - var_uc_tnom);
        let assign22610_e27853: f64 = (p.p325 * assign22610_e27852);
        let assign22610_e27854: f64 = (var_rrdrbb + assign22610_e27853);
        (assign22610_e27854, (var_rrdrbb_dn4 + (p.p325 * var_ttemp_dn4)),)
    } else {
        (var_rrdrbb, var_rrdrbb_dn4,)
    }
};
        var_rrdrbb = assign22610_e27856;
        var_rrdrbb_dn4 = assign22610_e27856_d_n4;

        let (assign22620_e27866,) = {
    if (var_guard380 != 0.0) {
        let assign22620_e27862: f64 = (var_lg).powf(p.p331);
        let assign22620_e27863: f64 = (p.p330 / assign22620_e27862);
        let assign22620_e27864: f64 = (1.0 + assign22620_e27863);
        (assign22620_e27864,)
    } else {
        (var_rdrmuele,)
    }
};
        var_rdrmuele = assign22620_e27866;

        let (assign22630_e27876,) = {
    if (var_guard380 != 0.0) {
        let assign22630_e27872: f64 = (var_lg).powf(p.p329);
        let assign22630_e27873: f64 = (p.p328 / assign22630_e27872);
        let assign22630_e27874: f64 = (1.0 + assign22630_e27873);
        (assign22630_e27874,)
    } else {
        (var_rdrvmaxle,)
    }
};
        var_rdrvmaxle = assign22630_e27876;

        let (assign22640_e27886,) = {
    if (var_guard380 != 0.0) {
        let assign22640_e27882: f64 = (var_wg).powf(p.p327);
        let assign22640_e27883: f64 = (p.p326 / assign22640_e27882);
        let assign22640_e27884: f64 = (1.0 + assign22640_e27883);
        (assign22640_e27884,)
    } else {
        (var_rdrvmaxwe,)
    }
};
        var_rdrvmaxwe = assign22640_e27886;

        let (assign22650_e27892, assign22650_e27892_d_n0, assign22650_e27892_d_n2, assign22650_e27892_d_n4, assign22650_e27892_d_n5, assign22650_e27892_d_n6, assign22650_e27892_d_n8, assign22650_e27892_d_n10, assign22650_e27892_d_n11, assign22650_e27892_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22650_e27890: f64 = (var_mu0 * var_rdrmuele);
        (assign22650_e27890, (var_mu0_dn0 * var_rdrmuele), (var_mu0_dn2 * var_rdrmuele), (var_mu0_dn4 * var_rdrmuele), (var_mu0_dn5 * var_rdrmuele), (var_mu0_dn6 * var_rdrmuele), (var_mu0_dn8 * var_rdrmuele), (var_mu0_dn10 * var_rdrmuele), (var_mu0_dn11 * var_rdrmuele), (var_mu0_dn12 * var_rdrmuele),)
    } else {
        (var_mu0, var_mu0_dn0, var_mu0_dn2, var_mu0_dn4, var_mu0_dn5, var_mu0_dn6, var_mu0_dn8, var_mu0_dn10, var_mu0_dn11, var_mu0_dn12,)
    }
};
        var_mu0 = assign22650_e27892;
        var_mu0_dn0 = assign22650_e27892_d_n0;
        var_mu0_dn2 = assign22650_e27892_d_n2;
        var_mu0_dn4 = assign22650_e27892_d_n4;
        var_mu0_dn5 = assign22650_e27892_d_n5;
        var_mu0_dn6 = assign22650_e27892_d_n6;
        var_mu0_dn8 = assign22650_e27892_d_n8;
        var_mu0_dn10 = assign22650_e27892_d_n10;
        var_mu0_dn11 = assign22650_e27892_d_n11;
        var_mu0_dn12 = assign22650_e27892_d_n12;

        let (assign22660_e27902, assign22660_e27902_d_n0, assign22660_e27902_d_n2, assign22660_e27902_d_n4, assign22660_e27902_d_n5, assign22660_e27902_d_n6, assign22660_e27902_d_n8, assign22660_e27902_d_n10, assign22660_e27902_d_n11, assign22660_e27902_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22660_e27896: f64 = (var_vmaxe__blk393 * var_rdrvmaxwe);
        let assign22660_e27898: f64 = (assign22660_e27896 * var_rdrvmaxle);
        let assign22660_e27900: f64 = (assign22660_e27898 + 1e-50);
        (assign22660_e27900, ((var_vmaxe__blk393_dn0 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk393_dn2 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk393_dn4 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk393_dn5 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk393_dn6 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk393_dn8 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk393_dn10 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk393_dn11 * var_rdrvmaxwe) * var_rdrvmaxle), ((var_vmaxe__blk393_dn12 * var_rdrvmaxwe) * var_rdrvmaxle),)
    } else {
        (var_vmaxe__blk393, var_vmaxe__blk393_dn0, var_vmaxe__blk393_dn2, var_vmaxe__blk393_dn4, var_vmaxe__blk393_dn5, var_vmaxe__blk393_dn6, var_vmaxe__blk393_dn8, var_vmaxe__blk393_dn10, var_vmaxe__blk393_dn11, var_vmaxe__blk393_dn12,)
    }
};
        var_vmaxe__blk393 = assign22660_e27902;
        var_vmaxe__blk393_dn0 = assign22660_e27902_d_n0;
        var_vmaxe__blk393_dn2 = assign22660_e27902_d_n2;
        var_vmaxe__blk393_dn4 = assign22660_e27902_d_n4;
        var_vmaxe__blk393_dn5 = assign22660_e27902_d_n5;
        var_vmaxe__blk393_dn6 = assign22660_e27902_d_n6;
        var_vmaxe__blk393_dn8 = assign22660_e27902_d_n8;
        var_vmaxe__blk393_dn10 = assign22660_e27902_d_n10;
        var_vmaxe__blk393_dn11 = assign22660_e27902_d_n11;
        var_vmaxe__blk393_dn12 = assign22660_e27902_d_n12;

        let (assign22670_e27908, assign22670_e27908_d_n2, assign22670_e27908_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22670_e27906: f64 = (var_vrdr / var_ldrifte);
        (assign22670_e27906, (var_vrdr_dn2 / var_ldrifte), (var_vrdr_dn12 / var_ldrifte),)
    } else {
        (var_edri, var_edri_dn2, var_edri_dn12,)
    }
};
        var_edri = assign22670_e27908;
        var_edri_dn2 = assign22670_e27908_d_n2;
        var_edri_dn12 = assign22670_e27908_d_n12;

        let (assign22680_e27914, assign22680_e27914_d_n0, assign22680_e27914_d_n2, assign22680_e27914_d_n4, assign22680_e27914_d_n5, assign22680_e27914_d_n6, assign22680_e27914_d_n8, assign22680_e27914_d_n10, assign22680_e27914_d_n11, assign22680_e27914_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22680_e27912: f64 = (var_mu0 * var_edri);
        (assign22680_e27912, (var_mu0_dn0 * var_edri), ((var_mu0_dn2 * var_edri) + (var_mu0 * var_edri_dn2)), (var_mu0_dn4 * var_edri), (var_mu0_dn5 * var_edri), (var_mu0_dn6 * var_edri), (var_mu0_dn8 * var_edri), (var_mu0_dn10 * var_edri), (var_mu0_dn11 * var_edri), ((var_mu0_dn12 * var_edri) + (var_mu0 * var_edri_dn12)),)
    } else {
        (var_vdri, var_vdri_dn0, var_vdri_dn2, var_vdri_dn4, var_vdri_dn5, var_vdri_dn6, var_vdri_dn8, var_vdri_dn10, var_vdri_dn11, var_vdri_dn12,)
    }
};
        var_vdri = assign22680_e27914;
        var_vdri_dn0 = assign22680_e27914_d_n0;
        var_vdri_dn2 = assign22680_e27914_d_n2;
        var_vdri_dn4 = assign22680_e27914_d_n4;
        var_vdri_dn5 = assign22680_e27914_d_n5;
        var_vdri_dn6 = assign22680_e27914_d_n6;
        var_vdri_dn8 = assign22680_e27914_d_n8;
        var_vdri_dn10 = assign22680_e27914_d_n10;
        var_vdri_dn11 = assign22680_e27914_d_n11;
        var_vdri_dn12 = assign22680_e27914_d_n12;

        let assign22690_e27917: f64 = if var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        var_guard400 = assign22690_e27917;

        let (assign22700_e27925, assign22700_e27925_d_n0, assign22700_e27925_d_n2, assign22700_e27925_d_n4, assign22700_e27925_d_n5, assign22700_e27925_d_n6, assign22700_e27925_d_n8, assign22700_e27925_d_n10, assign22700_e27925_d_n11, assign22700_e27925_d_n12,) = {
    if ((var_guard380 != 0.0) && (var_guard400 != 0.0)) {
        let assign22700_e27923: f64 = (var_vdri / var_vmaxe__blk393);
        (assign22700_e27923, (((var_vdri_dn0 * var_vmaxe__blk393) - (var_vdri * var_vmaxe__blk393_dn0)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), (((var_vdri_dn2 * var_vmaxe__blk393) - (var_vdri * var_vmaxe__blk393_dn2)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), (((var_vdri_dn4 * var_vmaxe__blk393) - (var_vdri * var_vmaxe__blk393_dn4)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), (((var_vdri_dn5 * var_vmaxe__blk393) - (var_vdri * var_vmaxe__blk393_dn5)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), (((var_vdri_dn6 * var_vmaxe__blk393) - (var_vdri * var_vmaxe__blk393_dn6)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), (((var_vdri_dn8 * var_vmaxe__blk393) - (var_vdri * var_vmaxe__blk393_dn8)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), (((var_vdri_dn10 * var_vmaxe__blk393) - (var_vdri * var_vmaxe__blk393_dn10)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), (((var_vdri_dn11 * var_vmaxe__blk393) - (var_vdri * var_vmaxe__blk393_dn11)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), (((var_vdri_dn12 * var_vmaxe__blk393) - (var_vdri * var_vmaxe__blk393_dn12)) / (var_vmaxe__blk393 * var_vmaxe__blk393)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign22700_e27925;
        var_t1_dn0 = assign22700_e27925_d_n0;
        var_t1_dn2 = assign22700_e27925_d_n2;
        var_t1_dn4 = assign22700_e27925_d_n4;
        var_t1_dn5 = assign22700_e27925_d_n5;
        var_t1_dn6 = assign22700_e27925_d_n6;
        var_t1_dn8 = assign22700_e27925_d_n8;
        var_t1_dn10 = assign22700_e27925_d_n10;
        var_t1_dn11 = assign22700_e27925_d_n11;
        var_t1_dn12 = assign22700_e27925_d_n12;

        let (assign22710_e27935, assign22710_e27935_d_n0, assign22710_e27935_d_n2, assign22710_e27935_d_n4, assign22710_e27935_d_n5, assign22710_e27935_d_n6, assign22710_e27935_d_n8, assign22710_e27935_d_n10, assign22710_e27935_d_n11, assign22710_e27935_d_n12,) = {
    if ((var_guard380 != 0.0) && (var_guard400 == 0.0)) {
        let assign22710_e27931: f64 = (-var_vdri);
        let assign22710_e27933: f64 = (assign22710_e27931 / var_vmaxe__blk393);
        (assign22710_e27933, ((((-var_vdri_dn0) * var_vmaxe__blk393) - (assign22710_e27931 * var_vmaxe__blk393_dn0)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), ((((-var_vdri_dn2) * var_vmaxe__blk393) - (assign22710_e27931 * var_vmaxe__blk393_dn2)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), ((((-var_vdri_dn4) * var_vmaxe__blk393) - (assign22710_e27931 * var_vmaxe__blk393_dn4)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), ((((-var_vdri_dn5) * var_vmaxe__blk393) - (assign22710_e27931 * var_vmaxe__blk393_dn5)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), ((((-var_vdri_dn6) * var_vmaxe__blk393) - (assign22710_e27931 * var_vmaxe__blk393_dn6)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), ((((-var_vdri_dn8) * var_vmaxe__blk393) - (assign22710_e27931 * var_vmaxe__blk393_dn8)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), ((((-var_vdri_dn10) * var_vmaxe__blk393) - (assign22710_e27931 * var_vmaxe__blk393_dn10)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), ((((-var_vdri_dn11) * var_vmaxe__blk393) - (assign22710_e27931 * var_vmaxe__blk393_dn11)) / (var_vmaxe__blk393 * var_vmaxe__blk393)), ((((-var_vdri_dn12) * var_vmaxe__blk393) - (assign22710_e27931 * var_vmaxe__blk393_dn12)) / (var_vmaxe__blk393 * var_vmaxe__blk393)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign22710_e27935;
        var_t1_dn0 = assign22710_e27935_d_n0;
        var_t1_dn2 = assign22710_e27935_d_n2;
        var_t1_dn4 = assign22710_e27935_d_n4;
        var_t1_dn5 = assign22710_e27935_d_n5;
        var_t1_dn6 = assign22710_e27935_d_n6;
        var_t1_dn8 = assign22710_e27935_d_n8;
        var_t1_dn10 = assign22710_e27935_d_n10;
        var_t1_dn11 = assign22710_e27935_d_n11;
        var_t1_dn12 = assign22710_e27935_d_n12;

        let assign22720_e27939: f64 = (10.0 * 2.220446049250313e-16);
        let assign22720_e27940: f64 = (1.0 - assign22720_e27939);
        let assign22720_e27947: f64 = (10.0 * 2.220446049250313e-16);
        let assign22720_e27948: f64 = (1.0 + assign22720_e27947);
        let assign22720_e27950: f64 = if ((assign22720_e27940 <= var_rrdrbb) && (var_rrdrbb <= assign22720_e27948)) { 1.0 } else { 0.0 };
        var_guard401 = assign22720_e27950;

        let (assign22730_e27956, assign22730_e27956_d_n0, assign22730_e27956_d_n2, assign22730_e27956_d_n4, assign22730_e27956_d_n5, assign22730_e27956_d_n6, assign22730_e27956_d_n8, assign22730_e27956_d_n10, assign22730_e27956_d_n11, assign22730_e27956_d_n12,) = {
    if ((var_guard380 != 0.0) && (var_guard401 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign22730_e27956;
        var_t3_dn0 = assign22730_e27956_d_n0;
        var_t3_dn2 = assign22730_e27956_d_n2;
        var_t3_dn4 = assign22730_e27956_d_n4;
        var_t3_dn5 = assign22730_e27956_d_n5;
        var_t3_dn6 = assign22730_e27956_d_n6;
        var_t3_dn8 = assign22730_e27956_d_n8;
        var_t3_dn10 = assign22730_e27956_d_n10;
        var_t3_dn11 = assign22730_e27956_d_n11;
        var_t3_dn12 = assign22730_e27956_d_n12;

        let assign22740_e27960: f64 = (10.0 * 2.220446049250313e-16);
        let assign22740_e27961: f64 = (2.0 - assign22740_e27960);
        let assign22740_e27968: f64 = (10.0 * 2.220446049250313e-16);
        let assign22740_e27969: f64 = (2.0 + assign22740_e27968);
        let assign22740_e27971: f64 = if ((assign22740_e27961 <= var_rrdrbb) && (var_rrdrbb <= assign22740_e27969)) { 1.0 } else { 0.0 };
        var_guard402 = assign22740_e27971;

        let (assign22750_e27980, assign22750_e27980_d_n0, assign22750_e27980_d_n2, assign22750_e27980_d_n4, assign22750_e27980_d_n5, assign22750_e27980_d_n6, assign22750_e27980_d_n8, assign22750_e27980_d_n10, assign22750_e27980_d_n11, assign22750_e27980_d_n12,) = {
    if (((var_guard380 != 0.0) && (var_guard401 == 0.0)) && (var_guard402 != 0.0)) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign22750_e27980;
        var_t3_dn0 = assign22750_e27980_d_n0;
        var_t3_dn2 = assign22750_e27980_d_n2;
        var_t3_dn4 = assign22750_e27980_d_n4;
        var_t3_dn5 = assign22750_e27980_d_n5;
        var_t3_dn6 = assign22750_e27980_d_n6;
        var_t3_dn8 = assign22750_e27980_d_n8;
        var_t3_dn10 = assign22750_e27980_d_n10;
        var_t3_dn11 = assign22750_e27980_d_n11;
        var_t3_dn12 = assign22750_e27980_d_n12;

        let (assign22760_e27994, assign22760_e27994_d_n0, assign22760_e27994_d_n2, assign22760_e27994_d_n4, assign22760_e27994_d_n5, assign22760_e27994_d_n6, assign22760_e27994_d_n8, assign22760_e27994_d_n10, assign22760_e27994_d_n11, assign22760_e27994_d_n12,) = {
    if (((var_guard380 != 0.0) && (var_guard401 == 0.0)) && (var_guard402 == 0.0)) {
        let assign22760_e27991: f64 = (var_rrdrbb - 1.0);
        let assign22760_e27992: f64 = (var_t1).powf(assign22760_e27991);
        (assign22760_e27992, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((var_t1).powf(assign22760_e27991 - 1.0) * var_t1_dn0)) } } else { (assign22760_e27992 * (assign22760_e27991 * (var_t1_dn0 / var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((var_t1).powf(assign22760_e27991 - 1.0) * var_t1_dn2)) } } else { (assign22760_e27992 * (assign22760_e27991 * (var_t1_dn2 / var_t1))) }, if var_rrdrbb_dn4 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((var_t1).powf(assign22760_e27991 - 1.0) * var_t1_dn4)) } } else { (assign22760_e27992 * ((var_rrdrbb_dn4 * (var_t1).ln()) + (assign22760_e27991 * (var_t1_dn4 / var_t1)))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((var_t1).powf(assign22760_e27991 - 1.0) * var_t1_dn5)) } } else { (assign22760_e27992 * (assign22760_e27991 * (var_t1_dn5 / var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((var_t1).powf(assign22760_e27991 - 1.0) * var_t1_dn6)) } } else { (assign22760_e27992 * (assign22760_e27991 * (var_t1_dn6 / var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((var_t1).powf(assign22760_e27991 - 1.0) * var_t1_dn8)) } } else { (assign22760_e27992 * (assign22760_e27991 * (var_t1_dn8 / var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((var_t1).powf(assign22760_e27991 - 1.0) * var_t1_dn10)) } } else { (assign22760_e27992 * (assign22760_e27991 * (var_t1_dn10 / var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((var_t1).powf(assign22760_e27991 - 1.0) * var_t1_dn11)) } } else { (assign22760_e27992 * (assign22760_e27991 * (var_t1_dn11 / var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((var_t1).powf(assign22760_e27991 - 1.0) * var_t1_dn12)) } } else { (assign22760_e27992 * (assign22760_e27991 * (var_t1_dn12 / var_t1))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign22760_e27994;
        var_t3_dn0 = assign22760_e27994_d_n0;
        var_t3_dn2 = assign22760_e27994_d_n2;
        var_t3_dn4 = assign22760_e27994_d_n4;
        var_t3_dn5 = assign22760_e27994_d_n5;
        var_t3_dn6 = assign22760_e27994_d_n6;
        var_t3_dn8 = assign22760_e27994_d_n8;
        var_t3_dn10 = assign22760_e27994_d_n10;
        var_t3_dn11 = assign22760_e27994_d_n11;
        var_t3_dn12 = assign22760_e27994_d_n12;

        let (assign22770_e28000, assign22770_e28000_d_n0, assign22770_e28000_d_n2, assign22770_e28000_d_n4, assign22770_e28000_d_n5, assign22770_e28000_d_n6, assign22770_e28000_d_n8, assign22770_e28000_d_n10, assign22770_e28000_d_n11, assign22770_e28000_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22770_e27998: f64 = (var_t1 * var_t3);
        (assign22770_e27998, ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)), ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)), ((var_t1_dn4 * var_t3) + (var_t1 * var_t3_dn4)), ((var_t1_dn5 * var_t3) + (var_t1 * var_t3_dn5)), ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)), ((var_t1_dn8 * var_t3) + (var_t1 * var_t3_dn8)), ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)), ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)), ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign22770_e28000;
        var_t2_dn0 = assign22770_e28000_d_n0;
        var_t2_dn2 = assign22770_e28000_d_n2;
        var_t2_dn4 = assign22770_e28000_d_n4;
        var_t2_dn5 = assign22770_e28000_d_n5;
        var_t2_dn6 = assign22770_e28000_d_n6;
        var_t2_dn8 = assign22770_e28000_d_n8;
        var_t2_dn10 = assign22770_e28000_d_n10;
        var_t2_dn11 = assign22770_e28000_d_n11;
        var_t2_dn12 = assign22770_e28000_d_n12;

        let (assign22780_e28006, assign22780_e28006_d_n0, assign22780_e28006_d_n2, assign22780_e28006_d_n4, assign22780_e28006_d_n5, assign22780_e28006_d_n6, assign22780_e28006_d_n8, assign22780_e28006_d_n10, assign22780_e28006_d_n11, assign22780_e28006_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22780_e28004: f64 = (1.0 + var_t2);
        (assign22780_e28004, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
        var_t4 = assign22780_e28006;
        var_t4_dn0 = assign22780_e28006_d_n0;
        var_t4_dn2 = assign22780_e28006_d_n2;
        var_t4_dn4 = assign22780_e28006_d_n4;
        var_t4_dn5 = assign22780_e28006_d_n5;
        var_t4_dn6 = assign22780_e28006_d_n6;
        var_t4_dn8 = assign22780_e28006_d_n8;
        var_t4_dn10 = assign22780_e28006_d_n10;
        var_t4_dn11 = assign22780_e28006_d_n11;
        var_t4_dn12 = assign22780_e28006_d_n12;

        let assign22790_e28010: f64 = (10.0 * 2.220446049250313e-16);
        let assign22790_e28011: f64 = (1.0 - assign22790_e28010);
        let assign22790_e28018: f64 = (10.0 * 2.220446049250313e-16);
        let assign22790_e28019: f64 = (1.0 + assign22790_e28018);
        let assign22790_e28021: f64 = if ((assign22790_e28011 <= var_rrdrbb) && (var_rrdrbb <= assign22790_e28019)) { 1.0 } else { 0.0 };
        var_guard403 = assign22790_e28021;

        let (assign22800_e28029, assign22800_e28029_d_n0, assign22800_e28029_d_n2, assign22800_e28029_d_n4, assign22800_e28029_d_n5, assign22800_e28029_d_n6, assign22800_e28029_d_n8, assign22800_e28029_d_n10, assign22800_e28029_d_n11, assign22800_e28029_d_n12,) = {
    if ((var_guard380 != 0.0) && (var_guard403 != 0.0)) {
        let assign22800_e28027: f64 = (1.0 / var_t4);
        (assign22800_e28027, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn4 / (var_t4 * var_t4))), (-(var_t4_dn5 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn8 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn11 / (var_t4 * var_t4))), (-(var_t4_dn12 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign22800_e28029;
        var_t5_dn0 = assign22800_e28029_d_n0;
        var_t5_dn2 = assign22800_e28029_d_n2;
        var_t5_dn4 = assign22800_e28029_d_n4;
        var_t5_dn5 = assign22800_e28029_d_n5;
        var_t5_dn6 = assign22800_e28029_d_n6;
        var_t5_dn8 = assign22800_e28029_d_n8;
        var_t5_dn10 = assign22800_e28029_d_n10;
        var_t5_dn11 = assign22800_e28029_d_n11;
        var_t5_dn12 = assign22800_e28029_d_n12;

        let assign22810_e28033: f64 = (10.0 * 2.220446049250313e-16);
        let assign22810_e28034: f64 = (2.0 - assign22810_e28033);
        let assign22810_e28041: f64 = (10.0 * 2.220446049250313e-16);
        let assign22810_e28042: f64 = (2.0 + assign22810_e28041);
        let assign22810_e28044: f64 = if ((assign22810_e28034 <= var_rrdrbb) && (var_rrdrbb <= assign22810_e28042)) { 1.0 } else { 0.0 };
        var_guard404 = assign22810_e28044;

        let (assign22820_e28056, assign22820_e28056_d_n0, assign22820_e28056_d_n2, assign22820_e28056_d_n4, assign22820_e28056_d_n5, assign22820_e28056_d_n6, assign22820_e28056_d_n8, assign22820_e28056_d_n10, assign22820_e28056_d_n11, assign22820_e28056_d_n12,) = {
    if (((var_guard380 != 0.0) && (var_guard403 == 0.0)) && (var_guard404 != 0.0)) {
        let assign22820_e28053: f64 = (var_t4).sqrt();
        let assign22820_e28054: f64 = (1.0 / assign22820_e28053);
        (assign22820_e28054, (-((var_t4_dn0 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((var_t4_dn2 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((var_t4_dn4 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((var_t4_dn5 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((var_t4_dn6 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((var_t4_dn8 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((var_t4_dn10 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((var_t4_dn11 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((var_t4_dn12 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign22820_e28056;
        var_t5_dn0 = assign22820_e28056_d_n0;
        var_t5_dn2 = assign22820_e28056_d_n2;
        var_t5_dn4 = assign22820_e28056_d_n4;
        var_t5_dn5 = assign22820_e28056_d_n5;
        var_t5_dn6 = assign22820_e28056_d_n6;
        var_t5_dn8 = assign22820_e28056_d_n8;
        var_t5_dn10 = assign22820_e28056_d_n10;
        var_t5_dn11 = assign22820_e28056_d_n11;
        var_t5_dn12 = assign22820_e28056_d_n12;

        let (assign22830_e28073, assign22830_e28073_d_n0, assign22830_e28073_d_n2, assign22830_e28073_d_n4, assign22830_e28073_d_n5, assign22830_e28073_d_n6, assign22830_e28073_d_n8, assign22830_e28073_d_n10, assign22830_e28073_d_n11, assign22830_e28073_d_n12,) = {
    if (((var_guard380 != 0.0) && (var_guard403 == 0.0)) && (var_guard404 == 0.0)) {
        let assign22830_e28066: f64 = (-1.0);
        let assign22830_e28068: f64 = (assign22830_e28066 / var_rrdrbb);
        let assign22830_e28070: f64 = (assign22830_e28068 - 1.0);
        let assign22830_e28071: f64 = (var_t4).powf(assign22830_e28070);
        (assign22830_e28071, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((var_t4).powf(assign22830_e28070 - 1.0) * var_t4_dn0)) } } else { (assign22830_e28071 * (assign22830_e28070 * (var_t4_dn0 / var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((var_t4).powf(assign22830_e28070 - 1.0) * var_t4_dn2)) } } else { (assign22830_e28071 * (assign22830_e28070 * (var_t4_dn2 / var_t4))) }, if (-((assign22830_e28066 * var_rrdrbb_dn4) / (var_rrdrbb * var_rrdrbb))) == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((var_t4).powf(assign22830_e28070 - 1.0) * var_t4_dn4)) } } else { (assign22830_e28071 * (((-((assign22830_e28066 * var_rrdrbb_dn4) / (var_rrdrbb * var_rrdrbb))) * (var_t4).ln()) + (assign22830_e28070 * (var_t4_dn4 / var_t4)))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((var_t4).powf(assign22830_e28070 - 1.0) * var_t4_dn5)) } } else { (assign22830_e28071 * (assign22830_e28070 * (var_t4_dn5 / var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((var_t4).powf(assign22830_e28070 - 1.0) * var_t4_dn6)) } } else { (assign22830_e28071 * (assign22830_e28070 * (var_t4_dn6 / var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((var_t4).powf(assign22830_e28070 - 1.0) * var_t4_dn8)) } } else { (assign22830_e28071 * (assign22830_e28070 * (var_t4_dn8 / var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((var_t4).powf(assign22830_e28070 - 1.0) * var_t4_dn10)) } } else { (assign22830_e28071 * (assign22830_e28070 * (var_t4_dn10 / var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((var_t4).powf(assign22830_e28070 - 1.0) * var_t4_dn11)) } } else { (assign22830_e28071 * (assign22830_e28070 * (var_t4_dn11 / var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((var_t4).powf(assign22830_e28070 - 1.0) * var_t4_dn12)) } } else { (assign22830_e28071 * (assign22830_e28070 * (var_t4_dn12 / var_t4))) },)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
        var_t6 = assign22830_e28073;
        var_t6_dn0 = assign22830_e28073_d_n0;
        var_t6_dn2 = assign22830_e28073_d_n2;
        var_t6_dn4 = assign22830_e28073_d_n4;
        var_t6_dn5 = assign22830_e28073_d_n5;
        var_t6_dn6 = assign22830_e28073_d_n6;
        var_t6_dn8 = assign22830_e28073_d_n8;
        var_t6_dn10 = assign22830_e28073_d_n10;
        var_t6_dn11 = assign22830_e28073_d_n11;
        var_t6_dn12 = assign22830_e28073_d_n12;

        let (assign22840_e28085, assign22840_e28085_d_n0, assign22840_e28085_d_n2, assign22840_e28085_d_n4, assign22840_e28085_d_n5, assign22840_e28085_d_n6, assign22840_e28085_d_n8, assign22840_e28085_d_n10, assign22840_e28085_d_n11, assign22840_e28085_d_n12,) = {
    if (((var_guard380 != 0.0) && (var_guard403 == 0.0)) && (var_guard404 == 0.0)) {
        let assign22840_e28083: f64 = (var_t4 * var_t6);
        (assign22840_e28083, ((var_t4_dn0 * var_t6) + (var_t4 * var_t6_dn0)), ((var_t4_dn2 * var_t6) + (var_t4 * var_t6_dn2)), ((var_t4_dn4 * var_t6) + (var_t4 * var_t6_dn4)), ((var_t4_dn5 * var_t6) + (var_t4 * var_t6_dn5)), ((var_t4_dn6 * var_t6) + (var_t4 * var_t6_dn6)), ((var_t4_dn8 * var_t6) + (var_t4 * var_t6_dn8)), ((var_t4_dn10 * var_t6) + (var_t4 * var_t6_dn10)), ((var_t4_dn11 * var_t6) + (var_t4 * var_t6_dn11)), ((var_t4_dn12 * var_t6) + (var_t4 * var_t6_dn12)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign22840_e28085;
        var_t5_dn0 = assign22840_e28085_d_n0;
        var_t5_dn2 = assign22840_e28085_d_n2;
        var_t5_dn4 = assign22840_e28085_d_n4;
        var_t5_dn5 = assign22840_e28085_d_n5;
        var_t5_dn6 = assign22840_e28085_d_n6;
        var_t5_dn8 = assign22840_e28085_d_n8;
        var_t5_dn10 = assign22840_e28085_d_n10;
        var_t5_dn11 = assign22840_e28085_d_n11;
        var_t5_dn12 = assign22840_e28085_d_n12;

        let (assign22850_e28091, assign22850_e28091_d_n0, assign22850_e28091_d_n2, assign22850_e28091_d_n4, assign22850_e28091_d_n5, assign22850_e28091_d_n6, assign22850_e28091_d_n8, assign22850_e28091_d_n10, assign22850_e28091_d_n11, assign22850_e28091_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22850_e28089: f64 = (var_mu0 * var_t5);
        (assign22850_e28089, ((var_mu0_dn0 * var_t5) + (var_mu0 * var_t5_dn0)), ((var_mu0_dn2 * var_t5) + (var_mu0 * var_t5_dn2)), ((var_mu0_dn4 * var_t5) + (var_mu0 * var_t5_dn4)), ((var_mu0_dn5 * var_t5) + (var_mu0 * var_t5_dn5)), ((var_mu0_dn6 * var_t5) + (var_mu0 * var_t5_dn6)), ((var_mu0_dn8 * var_t5) + (var_mu0 * var_t5_dn8)), ((var_mu0_dn10 * var_t5) + (var_mu0 * var_t5_dn10)), ((var_mu0_dn11 * var_t5) + (var_mu0 * var_t5_dn11)), ((var_mu0_dn12 * var_t5) + (var_mu0 * var_t5_dn12)),)
    } else {
        (var_mu__blk396, var_mu__blk396_dn0, var_mu__blk396_dn2, var_mu__blk396_dn4, var_mu__blk396_dn5, var_mu__blk396_dn6, var_mu__blk396_dn8, var_mu__blk396_dn10, var_mu__blk396_dn11, var_mu__blk396_dn12,)
    }
};
        var_mu__blk396 = assign22850_e28091;
        var_mu__blk396_dn0 = assign22850_e28091_d_n0;
        var_mu__blk396_dn2 = assign22850_e28091_d_n2;
        var_mu__blk396_dn4 = assign22850_e28091_d_n4;
        var_mu__blk396_dn5 = assign22850_e28091_d_n5;
        var_mu__blk396_dn6 = assign22850_e28091_d_n6;
        var_mu__blk396_dn8 = assign22850_e28091_d_n8;
        var_mu__blk396_dn10 = assign22850_e28091_d_n10;
        var_mu__blk396_dn11 = assign22850_e28091_d_n11;
        var_mu__blk396_dn12 = assign22850_e28091_d_n12;

        let (assign22860_e28097, assign22860_e28097_d_n0, assign22860_e28097_d_n2, assign22860_e28097_d_n4, assign22860_e28097_d_n5, assign22860_e28097_d_n6, assign22860_e28097_d_n8, assign22860_e28097_d_n10, assign22860_e28097_d_n11, assign22860_e28097_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22860_e28095: f64 = (1.6021918e-19 / var_ldrifte);
        (assign22860_e28095, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign22860_e28097;
        var_t1_dn0 = assign22860_e28097_d_n0;
        var_t1_dn2 = assign22860_e28097_d_n2;
        var_t1_dn4 = assign22860_e28097_d_n4;
        var_t1_dn5 = assign22860_e28097_d_n5;
        var_t1_dn6 = assign22860_e28097_d_n6;
        var_t1_dn8 = assign22860_e28097_d_n8;
        var_t1_dn10 = assign22860_e28097_d_n10;
        var_t1_dn11 = assign22860_e28097_d_n11;
        var_t1_dn12 = assign22860_e28097_d_n12;

        let (assign22870_e28107, assign22870_e28107_d_n0, assign22870_e28107_d_n2, assign22870_e28107_d_n4, assign22870_e28107_d_n5, assign22870_e28107_d_n6, assign22870_e28107_d_n8, assign22870_e28107_d_n10, assign22870_e28107_d_n11, assign22870_e28107_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22870_e28101: f64 = (var_t1 * var_xov);
        let assign22870_e28103: f64 = (assign22870_e28101 * var_mu__blk396);
        let assign22870_e28105: f64 = (assign22870_e28103 * var_nover);
        (assign22870_e28105, ((((var_t1_dn0 * var_xov) * var_mu__blk396) + (assign22870_e28101 * var_mu__blk396_dn0)) * var_nover), ((((var_t1_dn2 * var_xov) * var_mu__blk396) + (assign22870_e28101 * var_mu__blk396_dn2)) * var_nover), ((((var_t1_dn4 * var_xov) * var_mu__blk396) + (assign22870_e28101 * var_mu__blk396_dn4)) * var_nover), ((((var_t1_dn5 * var_xov) * var_mu__blk396) + (assign22870_e28101 * var_mu__blk396_dn5)) * var_nover), ((((var_t1_dn6 * var_xov) * var_mu__blk396) + (assign22870_e28101 * var_mu__blk396_dn6)) * var_nover), ((((var_t1_dn8 * var_xov) * var_mu__blk396) + (assign22870_e28101 * var_mu__blk396_dn8)) * var_nover), ((((var_t1_dn10 * var_xov) * var_mu__blk396) + (assign22870_e28101 * var_mu__blk396_dn10)) * var_nover), ((((var_t1_dn11 * var_xov) * var_mu__blk396) + (assign22870_e28101 * var_mu__blk396_dn11)) * var_nover), ((((var_t1_dn12 * var_xov) * var_mu__blk396) + (assign22870_e28101 * var_mu__blk396_dn12)) * var_nover),)
    } else {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn4, var_gd_dn5, var_gd_dn6, var_gd_dn8, var_gd_dn10, var_gd_dn11, var_gd_dn12,)
    }
};
        var_gd = assign22870_e28107;
        var_gd_dn0 = assign22870_e28107_d_n0;
        var_gd_dn2 = assign22870_e28107_d_n2;
        var_gd_dn4 = assign22870_e28107_d_n4;
        var_gd_dn5 = assign22870_e28107_d_n5;
        var_gd_dn6 = assign22870_e28107_d_n6;
        var_gd_dn8 = assign22870_e28107_d_n8;
        var_gd_dn10 = assign22870_e28107_d_n10;
        var_gd_dn11 = assign22870_e28107_d_n11;
        var_gd_dn12 = assign22870_e28107_d_n12;

        let assign22880_e28110: f64 = if var_gd <= 0.0 { 1.0 } else { 0.0 };
        var_guard405 = assign22880_e28110;

        let (assign22890_e28116, assign22890_e28116_d_n0, assign22890_e28116_d_n2, assign22890_e28116_d_n4, assign22890_e28116_d_n5, assign22890_e28116_d_n6, assign22890_e28116_d_n8, assign22890_e28116_d_n10, assign22890_e28116_d_n11, assign22890_e28116_d_n12,) = {
    if ((var_guard380 != 0.0) && (var_guard405 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn4, var_gd_dn5, var_gd_dn6, var_gd_dn8, var_gd_dn10, var_gd_dn11, var_gd_dn12,)
    }
};
        var_gd = assign22890_e28116;
        var_gd_dn0 = assign22890_e28116_d_n0;
        var_gd_dn2 = assign22890_e28116_d_n2;
        var_gd_dn4 = assign22890_e28116_d_n4;
        var_gd_dn5 = assign22890_e28116_d_n5;
        var_gd_dn6 = assign22890_e28116_d_n6;
        var_gd_dn8 = assign22890_e28116_d_n8;
        var_gd_dn10 = assign22890_e28116_d_n10;
        var_gd_dn11 = assign22890_e28116_d_n11;
        var_gd_dn12 = assign22890_e28116_d_n12;

        *var_edri_slot = var_edri;
        *var_edri_dn12_slot = var_edri_dn12;
        *var_edri_dn2_slot = var_edri_dn2;
        *var_gd_slot = var_gd;
        *var_gd_dn0_slot = var_gd_dn0;
        *var_gd_dn10_slot = var_gd_dn10;
        *var_gd_dn11_slot = var_gd_dn11;
        *var_gd_dn12_slot = var_gd_dn12;
        *var_gd_dn2_slot = var_gd_dn2;
        *var_gd_dn4_slot = var_gd_dn4;
        *var_gd_dn5_slot = var_gd_dn5;
        *var_gd_dn6_slot = var_gd_dn6;
        *var_gd_dn8_slot = var_gd_dn8;
        *var_guard400_slot = var_guard400;
        *var_guard401_slot = var_guard401;
        *var_guard402_slot = var_guard402;
        *var_guard403_slot = var_guard403;
        *var_guard404_slot = var_guard404;
        *var_guard405_slot = var_guard405;
        *var_mu0_slot = var_mu0;
        *var_mu0_dn0_slot = var_mu0_dn0;
        *var_mu0_dn10_slot = var_mu0_dn10;
        *var_mu0_dn11_slot = var_mu0_dn11;
        *var_mu0_dn12_slot = var_mu0_dn12;
        *var_mu0_dn2_slot = var_mu0_dn2;
        *var_mu0_dn4_slot = var_mu0_dn4;
        *var_mu0_dn5_slot = var_mu0_dn5;
        *var_mu0_dn6_slot = var_mu0_dn6;
        *var_mu0_dn8_slot = var_mu0_dn8;
        *var_mu__blk396_slot = var_mu__blk396;
        *var_mu__blk396_dn0_slot = var_mu__blk396_dn0;
        *var_mu__blk396_dn10_slot = var_mu__blk396_dn10;
        *var_mu__blk396_dn11_slot = var_mu__blk396_dn11;
        *var_mu__blk396_dn12_slot = var_mu__blk396_dn12;
        *var_mu__blk396_dn2_slot = var_mu__blk396_dn2;
        *var_mu__blk396_dn4_slot = var_mu__blk396_dn4;
        *var_mu__blk396_dn5_slot = var_mu__blk396_dn5;
        *var_mu__blk396_dn6_slot = var_mu__blk396_dn6;
        *var_mu__blk396_dn8_slot = var_mu__blk396_dn8;
        *var_rdrmuele_slot = var_rdrmuele;
        *var_rdrvmaxle_slot = var_rdrvmaxle;
        *var_rdrvmaxwe_slot = var_rdrvmaxwe;
        *var_rrdrbb_slot = var_rrdrbb;
        *var_rrdrbb_dn4_slot = var_rrdrbb_dn4;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_vdri_slot = var_vdri;
        *var_vdri_dn0_slot = var_vdri_dn0;
        *var_vdri_dn10_slot = var_vdri_dn10;
        *var_vdri_dn11_slot = var_vdri_dn11;
        *var_vdri_dn12_slot = var_vdri_dn12;
        *var_vdri_dn2_slot = var_vdri_dn2;
        *var_vdri_dn4_slot = var_vdri_dn4;
        *var_vdri_dn5_slot = var_vdri_dn5;
        *var_vdri_dn6_slot = var_vdri_dn6;
        *var_vdri_dn8_slot = var_vdri_dn8;
        *var_vmaxe__blk393_slot = var_vmaxe__blk393;
        *var_vmaxe__blk393_dn0_slot = var_vmaxe__blk393_dn0;
        *var_vmaxe__blk393_dn10_slot = var_vmaxe__blk393_dn10;
        *var_vmaxe__blk393_dn11_slot = var_vmaxe__blk393_dn11;
        *var_vmaxe__blk393_dn12_slot = var_vmaxe__blk393_dn12;
        *var_vmaxe__blk393_dn2_slot = var_vmaxe__blk393_dn2;
        *var_vmaxe__blk393_dn4_slot = var_vmaxe__blk393_dn4;
        *var_vmaxe__blk393_dn5_slot = var_vmaxe__blk393_dn5;
        *var_vmaxe__blk393_dn6_slot = var_vmaxe__blk393_dn6;
        *var_vmaxe__blk393_dn8_slot = var_vmaxe__blk393_dn8;
    }

    pub(super) fn stamp_transient_block_87(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_gd: f64,
        var_gd_dn0: f64,
        var_gd_dn10: f64,
        var_gd_dn11: f64,
        var_gd_dn12: f64,
        var_gd_dn2: f64,
        var_gd_dn4: f64,
        var_gd_dn5: f64,
        var_gd_dn6: f64,
        var_gd_dn8: f64,
        var_guard380: f64,
        var_lg: f64,
        var_mfactor: f64,
        var_rsd0: f64,
        var_ttemp: f64,
        var_ttemp_dn4: f64,
        var_uc_tnom: f64,
        var_weff: f64,
        var_weff_dn0: f64,
        var_weff_dn10: f64,
        var_weff_dn11: f64,
        var_weff_dn12: f64,
        var_weff_dn2: f64,
        var_weff_dn4: f64,
        var_weff_dn5: f64,
        var_weff_dn6: f64,
        var_weff_dn8: f64,
        var_weff_nf_1: f64,
        var_weff_nf_1_dn0: f64,
        var_weff_nf_1_dn10: f64,
        var_weff_nf_1_dn11: f64,
        var_weff_nf_1_dn12: f64,
        var_weff_nf_1_dn2: f64,
        var_weff_nf_1_dn4: f64,
        var_weff_nf_1_dn5: f64,
        var_weff_nf_1_dn6: f64,
        var_weff_nf_1_dn8: f64,
        var_wg: f64,
        var_edri__blk421_slot: &mut f64,
        var_edri__blk421_dn0_slot: &mut f64,
        var_edri__blk421_dn11_slot: &mut f64,
        var_guard406_slot: &mut f64,
        var_guard407_slot: &mut f64,
        var_guard427_slot: &mut f64,
        var_guard428_slot: &mut f64,
        var_ldrifte__blk417_slot: &mut f64,
        var_mks_rdrmue__blk411_slot: &mut f64,
        var_mks_rdrvmax__blk412_slot: &mut f64,
        var_mu0__blk419_slot: &mut f64,
        var_mu0__blk419_dn0_slot: &mut f64,
        var_mu0__blk419_dn10_slot: &mut f64,
        var_mu0__blk419_dn11_slot: &mut f64,
        var_mu0__blk419_dn12_slot: &mut f64,
        var_mu0__blk419_dn2_slot: &mut f64,
        var_mu0__blk419_dn4_slot: &mut f64,
        var_mu0__blk419_dn5_slot: &mut f64,
        var_mu0__blk419_dn6_slot: &mut f64,
        var_mu0__blk419_dn8_slot: &mut f64,
        var_nover__blk418_slot: &mut f64,
        var_rdrmuele__blk408_slot: &mut f64,
        var_rdrvmaxle__blk410_slot: &mut f64,
        var_rdrvmaxwe__blk409_slot: &mut f64,
        var_rrdrbb__blk413_slot: &mut f64,
        var_rrdrbb__blk413_dn4_slot: &mut f64,
        var_rsd_slot: &mut f64,
        var_rsd0__blk414_slot: &mut f64,
        var_rsd_dn0_slot: &mut f64,
        var_rsd_dn10_slot: &mut f64,
        var_rsd_dn11_slot: &mut f64,
        var_rsd_dn12_slot: &mut f64,
        var_rsd_dn2_slot: &mut f64,
        var_rsd_dn4_slot: &mut f64,
        var_rsd_dn5_slot: &mut f64,
        var_rsd_dn6_slot: &mut f64,
        var_rsd_dn8_slot: &mut f64,
        var_rsde_slot: &mut f64,
        var_rsde_dn0_slot: &mut f64,
        var_rsde_dn10_slot: &mut f64,
        var_rsde_dn11_slot: &mut f64,
        var_rsde_dn12_slot: &mut f64,
        var_rsde_dn2_slot: &mut f64,
        var_rsde_dn4_slot: &mut f64,
        var_rsde_dn5_slot: &mut f64,
        var_rsde_dn6_slot: &mut f64,
        var_rsde_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_tratio__blk416_slot: &mut f64,
        var_tratio__blk416_dn4_slot: &mut f64,
        var_vdri__blk422_slot: &mut f64,
        var_vdri__blk422_dn0_slot: &mut f64,
        var_vdri__blk422_dn10_slot: &mut f64,
        var_vdri__blk422_dn11_slot: &mut f64,
        var_vdri__blk422_dn12_slot: &mut f64,
        var_vdri__blk422_dn2_slot: &mut f64,
        var_vdri__blk422_dn4_slot: &mut f64,
        var_vdri__blk422_dn5_slot: &mut f64,
        var_vdri__blk422_dn6_slot: &mut f64,
        var_vdri__blk422_dn8_slot: &mut f64,
        var_vmaxe__blk420_slot: &mut f64,
        var_vmaxe__blk420_dn0_slot: &mut f64,
        var_vmaxe__blk420_dn10_slot: &mut f64,
        var_vmaxe__blk420_dn11_slot: &mut f64,
        var_vmaxe__blk420_dn12_slot: &mut f64,
        var_vmaxe__blk420_dn2_slot: &mut f64,
        var_vmaxe__blk420_dn4_slot: &mut f64,
        var_vmaxe__blk420_dn5_slot: &mut f64,
        var_vmaxe__blk420_dn6_slot: &mut f64,
        var_vmaxe__blk420_dn8_slot: &mut f64,
        var_vrdr__blk415_slot: &mut f64,
        var_vrdr__blk415_dn0_slot: &mut f64,
        var_vrdr__blk415_dn11_slot: &mut f64,
        var_weff_nf__blk426_slot: &mut f64,
        var_weff_nf__blk426_dn0_slot: &mut f64,
        var_weff_nf__blk426_dn10_slot: &mut f64,
        var_weff_nf__blk426_dn11_slot: &mut f64,
        var_weff_nf__blk426_dn12_slot: &mut f64,
        var_weff_nf__blk426_dn2_slot: &mut f64,
        var_weff_nf__blk426_dn4_slot: &mut f64,
        var_weff_nf__blk426_dn5_slot: &mut f64,
        var_weff_nf__blk426_dn6_slot: &mut f64,
        var_weff_nf__blk426_dn8_slot: &mut f64,
        var_xov__blk424_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let mut var_edri__blk421: f64 = *var_edri__blk421_slot;
        let mut var_edri__blk421_dn0: f64 = *var_edri__blk421_dn0_slot;
        let mut var_edri__blk421_dn11: f64 = *var_edri__blk421_dn11_slot;
        let mut var_guard406: f64 = *var_guard406_slot;
        let mut var_guard407: f64 = *var_guard407_slot;
        let mut var_guard427: f64 = *var_guard427_slot;
        let mut var_guard428: f64 = *var_guard428_slot;
        let mut var_ldrifte__blk417: f64 = *var_ldrifte__blk417_slot;
        let mut var_mks_rdrmue__blk411: f64 = *var_mks_rdrmue__blk411_slot;
        let mut var_mks_rdrvmax__blk412: f64 = *var_mks_rdrvmax__blk412_slot;
        let mut var_mu0__blk419: f64 = *var_mu0__blk419_slot;
        let mut var_mu0__blk419_dn0: f64 = *var_mu0__blk419_dn0_slot;
        let mut var_mu0__blk419_dn10: f64 = *var_mu0__blk419_dn10_slot;
        let mut var_mu0__blk419_dn11: f64 = *var_mu0__blk419_dn11_slot;
        let mut var_mu0__blk419_dn12: f64 = *var_mu0__blk419_dn12_slot;
        let mut var_mu0__blk419_dn2: f64 = *var_mu0__blk419_dn2_slot;
        let mut var_mu0__blk419_dn4: f64 = *var_mu0__blk419_dn4_slot;
        let mut var_mu0__blk419_dn5: f64 = *var_mu0__blk419_dn5_slot;
        let mut var_mu0__blk419_dn6: f64 = *var_mu0__blk419_dn6_slot;
        let mut var_mu0__blk419_dn8: f64 = *var_mu0__blk419_dn8_slot;
        let mut var_nover__blk418: f64 = *var_nover__blk418_slot;
        let mut var_rdrmuele__blk408: f64 = *var_rdrmuele__blk408_slot;
        let mut var_rdrvmaxle__blk410: f64 = *var_rdrvmaxle__blk410_slot;
        let mut var_rdrvmaxwe__blk409: f64 = *var_rdrvmaxwe__blk409_slot;
        let mut var_rrdrbb__blk413: f64 = *var_rrdrbb__blk413_slot;
        let mut var_rrdrbb__blk413_dn4: f64 = *var_rrdrbb__blk413_dn4_slot;
        let mut var_rsd: f64 = *var_rsd_slot;
        let mut var_rsd0__blk414: f64 = *var_rsd0__blk414_slot;
        let mut var_rsd_dn0: f64 = *var_rsd_dn0_slot;
        let mut var_rsd_dn10: f64 = *var_rsd_dn10_slot;
        let mut var_rsd_dn11: f64 = *var_rsd_dn11_slot;
        let mut var_rsd_dn12: f64 = *var_rsd_dn12_slot;
        let mut var_rsd_dn2: f64 = *var_rsd_dn2_slot;
        let mut var_rsd_dn4: f64 = *var_rsd_dn4_slot;
        let mut var_rsd_dn5: f64 = *var_rsd_dn5_slot;
        let mut var_rsd_dn6: f64 = *var_rsd_dn6_slot;
        let mut var_rsd_dn8: f64 = *var_rsd_dn8_slot;
        let mut var_rsde: f64 = *var_rsde_slot;
        let mut var_rsde_dn0: f64 = *var_rsde_dn0_slot;
        let mut var_rsde_dn10: f64 = *var_rsde_dn10_slot;
        let mut var_rsde_dn11: f64 = *var_rsde_dn11_slot;
        let mut var_rsde_dn12: f64 = *var_rsde_dn12_slot;
        let mut var_rsde_dn2: f64 = *var_rsde_dn2_slot;
        let mut var_rsde_dn4: f64 = *var_rsde_dn4_slot;
        let mut var_rsde_dn5: f64 = *var_rsde_dn5_slot;
        let mut var_rsde_dn6: f64 = *var_rsde_dn6_slot;
        let mut var_rsde_dn8: f64 = *var_rsde_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_tratio__blk416: f64 = *var_tratio__blk416_slot;
        let mut var_tratio__blk416_dn4: f64 = *var_tratio__blk416_dn4_slot;
        let mut var_vdri__blk422: f64 = *var_vdri__blk422_slot;
        let mut var_vdri__blk422_dn0: f64 = *var_vdri__blk422_dn0_slot;
        let mut var_vdri__blk422_dn10: f64 = *var_vdri__blk422_dn10_slot;
        let mut var_vdri__blk422_dn11: f64 = *var_vdri__blk422_dn11_slot;
        let mut var_vdri__blk422_dn12: f64 = *var_vdri__blk422_dn12_slot;
        let mut var_vdri__blk422_dn2: f64 = *var_vdri__blk422_dn2_slot;
        let mut var_vdri__blk422_dn4: f64 = *var_vdri__blk422_dn4_slot;
        let mut var_vdri__blk422_dn5: f64 = *var_vdri__blk422_dn5_slot;
        let mut var_vdri__blk422_dn6: f64 = *var_vdri__blk422_dn6_slot;
        let mut var_vdri__blk422_dn8: f64 = *var_vdri__blk422_dn8_slot;
        let mut var_vmaxe__blk420: f64 = *var_vmaxe__blk420_slot;
        let mut var_vmaxe__blk420_dn0: f64 = *var_vmaxe__blk420_dn0_slot;
        let mut var_vmaxe__blk420_dn10: f64 = *var_vmaxe__blk420_dn10_slot;
        let mut var_vmaxe__blk420_dn11: f64 = *var_vmaxe__blk420_dn11_slot;
        let mut var_vmaxe__blk420_dn12: f64 = *var_vmaxe__blk420_dn12_slot;
        let mut var_vmaxe__blk420_dn2: f64 = *var_vmaxe__blk420_dn2_slot;
        let mut var_vmaxe__blk420_dn4: f64 = *var_vmaxe__blk420_dn4_slot;
        let mut var_vmaxe__blk420_dn5: f64 = *var_vmaxe__blk420_dn5_slot;
        let mut var_vmaxe__blk420_dn6: f64 = *var_vmaxe__blk420_dn6_slot;
        let mut var_vmaxe__blk420_dn8: f64 = *var_vmaxe__blk420_dn8_slot;
        let mut var_vrdr__blk415: f64 = *var_vrdr__blk415_slot;
        let mut var_vrdr__blk415_dn0: f64 = *var_vrdr__blk415_dn0_slot;
        let mut var_vrdr__blk415_dn11: f64 = *var_vrdr__blk415_dn11_slot;
        let mut var_weff_nf__blk426: f64 = *var_weff_nf__blk426_slot;
        let mut var_weff_nf__blk426_dn0: f64 = *var_weff_nf__blk426_dn0_slot;
        let mut var_weff_nf__blk426_dn10: f64 = *var_weff_nf__blk426_dn10_slot;
        let mut var_weff_nf__blk426_dn11: f64 = *var_weff_nf__blk426_dn11_slot;
        let mut var_weff_nf__blk426_dn12: f64 = *var_weff_nf__blk426_dn12_slot;
        let mut var_weff_nf__blk426_dn2: f64 = *var_weff_nf__blk426_dn2_slot;
        let mut var_weff_nf__blk426_dn4: f64 = *var_weff_nf__blk426_dn4_slot;
        let mut var_weff_nf__blk426_dn5: f64 = *var_weff_nf__blk426_dn5_slot;
        let mut var_weff_nf__blk426_dn6: f64 = *var_weff_nf__blk426_dn6_slot;
        let mut var_weff_nf__blk426_dn8: f64 = *var_weff_nf__blk426_dn8_slot;
        let mut var_xov__blk424: f64 = *var_xov__blk424_slot;

        let (assign22900_e28122, assign22900_e28122_d_n0, assign22900_e28122_d_n2, assign22900_e28122_d_n4, assign22900_e28122_d_n5, assign22900_e28122_d_n6, assign22900_e28122_d_n8, assign22900_e28122_d_n10, assign22900_e28122_d_n11, assign22900_e28122_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22900_e28120: f64 = (1.0 / var_gd);
        (assign22900_e28120, (-(var_gd_dn0 / (var_gd * var_gd))), (-(var_gd_dn2 / (var_gd * var_gd))), (-(var_gd_dn4 / (var_gd * var_gd))), (-(var_gd_dn5 / (var_gd * var_gd))), (-(var_gd_dn6 / (var_gd * var_gd))), (-(var_gd_dn8 / (var_gd * var_gd))), (-(var_gd_dn10 / (var_gd * var_gd))), (-(var_gd_dn11 / (var_gd * var_gd))), (-(var_gd_dn12 / (var_gd * var_gd))),)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn4, var_rsd_dn5, var_rsd_dn6, var_rsd_dn8, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12,)
    }
};
        var_rsd = assign22900_e28122;
        var_rsd_dn0 = assign22900_e28122_d_n0;
        var_rsd_dn2 = assign22900_e28122_d_n2;
        var_rsd_dn4 = assign22900_e28122_d_n4;
        var_rsd_dn5 = assign22900_e28122_d_n5;
        var_rsd_dn6 = assign22900_e28122_d_n6;
        var_rsd_dn8 = assign22900_e28122_d_n8;
        var_rsd_dn10 = assign22900_e28122_d_n10;
        var_rsd_dn11 = assign22900_e28122_d_n11;
        var_rsd_dn12 = assign22900_e28122_d_n12;

        let (assign22910_e28128, assign22910_e28128_d_n0, assign22910_e28128_d_n2, assign22910_e28128_d_n4, assign22910_e28128_d_n5, assign22910_e28128_d_n6, assign22910_e28128_d_n8, assign22910_e28128_d_n10, assign22910_e28128_d_n11, assign22910_e28128_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22910_e28126: f64 = (var_rsd / var_weff_nf_1);
        (assign22910_e28126, (((var_rsd_dn0 * var_weff_nf_1) - (var_rsd * var_weff_nf_1_dn0)) / (var_weff_nf_1 * var_weff_nf_1)), (((var_rsd_dn2 * var_weff_nf_1) - (var_rsd * var_weff_nf_1_dn2)) / (var_weff_nf_1 * var_weff_nf_1)), (((var_rsd_dn4 * var_weff_nf_1) - (var_rsd * var_weff_nf_1_dn4)) / (var_weff_nf_1 * var_weff_nf_1)), (((var_rsd_dn5 * var_weff_nf_1) - (var_rsd * var_weff_nf_1_dn5)) / (var_weff_nf_1 * var_weff_nf_1)), (((var_rsd_dn6 * var_weff_nf_1) - (var_rsd * var_weff_nf_1_dn6)) / (var_weff_nf_1 * var_weff_nf_1)), (((var_rsd_dn8 * var_weff_nf_1) - (var_rsd * var_weff_nf_1_dn8)) / (var_weff_nf_1 * var_weff_nf_1)), (((var_rsd_dn10 * var_weff_nf_1) - (var_rsd * var_weff_nf_1_dn10)) / (var_weff_nf_1 * var_weff_nf_1)), (((var_rsd_dn11 * var_weff_nf_1) - (var_rsd * var_weff_nf_1_dn11)) / (var_weff_nf_1 * var_weff_nf_1)), (((var_rsd_dn12 * var_weff_nf_1) - (var_rsd * var_weff_nf_1_dn12)) / (var_weff_nf_1 * var_weff_nf_1)),)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn4, var_rsd_dn5, var_rsd_dn6, var_rsd_dn8, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12,)
    }
};
        var_rsd = assign22910_e28128;
        var_rsd_dn0 = assign22910_e28128_d_n0;
        var_rsd_dn2 = assign22910_e28128_d_n2;
        var_rsd_dn4 = assign22910_e28128_d_n4;
        var_rsd_dn5 = assign22910_e28128_d_n5;
        var_rsd_dn6 = assign22910_e28128_d_n6;
        var_rsd_dn8 = assign22910_e28128_d_n8;
        var_rsd_dn10 = assign22910_e28128_d_n10;
        var_rsd_dn11 = assign22910_e28128_d_n11;
        var_rsd_dn12 = assign22910_e28128_d_n12;

        let (assign22920_e28134, assign22920_e28134_d_n0, assign22920_e28134_d_n2, assign22920_e28134_d_n4, assign22920_e28134_d_n5, assign22920_e28134_d_n6, assign22920_e28134_d_n8, assign22920_e28134_d_n10, assign22920_e28134_d_n11, assign22920_e28134_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22920_e28132: f64 = (var_rsd + var_rsd0);
        (assign22920_e28132, var_rsd_dn0, var_rsd_dn2, var_rsd_dn4, var_rsd_dn5, var_rsd_dn6, var_rsd_dn8, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn4, var_rsd_dn5, var_rsd_dn6, var_rsd_dn8, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12,)
    }
};
        var_rsd = assign22920_e28134;
        var_rsd_dn0 = assign22920_e28134_d_n0;
        var_rsd_dn2 = assign22920_e28134_d_n2;
        var_rsd_dn4 = assign22920_e28134_d_n4;
        var_rsd_dn5 = assign22920_e28134_d_n5;
        var_rsd_dn6 = assign22920_e28134_d_n6;
        var_rsd_dn8 = assign22920_e28134_d_n8;
        var_rsd_dn10 = assign22920_e28134_d_n10;
        var_rsd_dn11 = assign22920_e28134_d_n11;
        var_rsd_dn12 = assign22920_e28134_d_n12;

        let assign22940_e28152: f64 = if var_rsd < 0.0001 { 1.0 } else { 0.0 };
        var_guard406 = assign22940_e28152;

        let (assign22950_e28158, assign22950_e28158_d_n0, assign22950_e28158_d_n2, assign22950_e28158_d_n4, assign22950_e28158_d_n5, assign22950_e28158_d_n6, assign22950_e28158_d_n8, assign22950_e28158_d_n10, assign22950_e28158_d_n11, assign22950_e28158_d_n12,) = {
    if ((var_guard380 != 0.0) && (var_guard406 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn4, var_rsd_dn5, var_rsd_dn6, var_rsd_dn8, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12,)
    }
};
        var_rsd = assign22950_e28158;
        var_rsd_dn0 = assign22950_e28158_d_n0;
        var_rsd_dn2 = assign22950_e28158_d_n2;
        var_rsd_dn4 = assign22950_e28158_d_n4;
        var_rsd_dn5 = assign22950_e28158_d_n5;
        var_rsd_dn6 = assign22950_e28158_d_n6;
        var_rsd_dn8 = assign22950_e28158_d_n8;
        var_rsd_dn10 = assign22950_e28158_d_n10;
        var_rsd_dn11 = assign22950_e28158_d_n11;
        var_rsd_dn12 = assign22950_e28158_d_n12;

        let (assign22960_e28164, assign22960_e28164_d_n0, assign22960_e28164_d_n2, assign22960_e28164_d_n4, assign22960_e28164_d_n5, assign22960_e28164_d_n6, assign22960_e28164_d_n8, assign22960_e28164_d_n10, assign22960_e28164_d_n11, assign22960_e28164_d_n12,) = {
    if (var_guard380 != 0.0) {
        let assign22960_e28162: f64 = (var_rsd / var_mfactor);
        (assign22960_e28162, (var_rsd_dn0 / var_mfactor), (var_rsd_dn2 / var_mfactor), (var_rsd_dn4 / var_mfactor), (var_rsd_dn5 / var_mfactor), (var_rsd_dn6 / var_mfactor), (var_rsd_dn8 / var_mfactor), (var_rsd_dn10 / var_mfactor), (var_rsd_dn11 / var_mfactor), (var_rsd_dn12 / var_mfactor),)
    } else {
        (var_rsde, var_rsde_dn0, var_rsde_dn2, var_rsde_dn4, var_rsde_dn5, var_rsde_dn6, var_rsde_dn8, var_rsde_dn10, var_rsde_dn11, var_rsde_dn12,)
    }
};
        var_rsde = assign22960_e28164;
        var_rsde_dn0 = assign22960_e28164_d_n0;
        var_rsde_dn2 = assign22960_e28164_d_n2;
        var_rsde_dn4 = assign22960_e28164_d_n4;
        var_rsde_dn5 = assign22960_e28164_d_n5;
        var_rsde_dn6 = assign22960_e28164_d_n6;
        var_rsde_dn8 = assign22960_e28164_d_n8;
        var_rsde_dn10 = assign22960_e28164_d_n10;
        var_rsde_dn11 = assign22960_e28164_d_n11;
        var_rsde_dn12 = assign22960_e28164_d_n12;

        let assign22980_e28171: f64 = if p.p313 == 1.0 { 1.0 } else { 0.0 };
        var_guard407 = assign22980_e28171;

        let (assign22990_e28177,) = {
    if (var_guard407 != 0.0) {
        let assign22990_e28175: f64 = (p.p40 / 1e-6);
        (assign22990_e28175,)
    } else {
        (var_nover__blk418,)
    }
};
        var_nover__blk418 = assign22990_e28177;

        let (assign23000_e28181,) = {
    if (var_guard407 != 0.0) {
        (p.p316,)
    } else {
        (var_mks_rdrmue__blk411,)
    }
};
        var_mks_rdrmue__blk411 = assign23000_e28181;

        let (assign23010_e28185,) = {
    if (var_guard407 != 0.0) {
        (p.p318,)
    } else {
        (var_mks_rdrvmax__blk412,)
    }
};
        var_mks_rdrvmax__blk412 = assign23010_e28185;

        let (assign23020_e28189, assign23020_e28189_d_n4,) = {
    if (var_guard407 != 0.0) {
        (p.p323, 0.0,)
    } else {
        (var_rrdrbb__blk413, var_rrdrbb__blk413_dn4,)
    }
};
        var_rrdrbb__blk413 = assign23020_e28189;
        var_rrdrbb__blk413_dn4 = assign23020_e28189_d_n4;

        let (assign23030_e28200,) = {
    if (var_guard407 != 0.0) {
        let (assign23030_e28198,) = {
            if (p.p314 > 0.0) {
                let assign23030_e28196: f64 = (p.p314 * p.p309);
                (assign23030_e28196,)
            } else {
                (0.0,)
            }
        };
        (assign23030_e28198,)
    } else {
        (var_rsd0__blk414,)
    }
};
        var_rsd0__blk414 = assign23030_e28200;

        let (assign23040_e28204,) = {
    if (var_guard407 != 0.0) {
        (p.p310,)
    } else {
        (var_ldrifte__blk417,)
    }
};
        var_ldrifte__blk417 = assign23040_e28204;

        let (assign23050_e28210, assign23050_e28210_d_n0, assign23050_e28210_d_n11,) = {
    if (var_guard407 != 0.0) {
        let assign23050_e28208: f64 = (p.p33 * (nv0 - nv11));
        (assign23050_e28208, p.p33, (-p.p33),)
    } else {
        (var_vrdr__blk415, var_vrdr__blk415_dn0, var_vrdr__blk415_dn11,)
    }
};
        var_vrdr__blk415 = assign23050_e28210;
        var_vrdr__blk415_dn0 = assign23050_e28210_d_n0;
        var_vrdr__blk415_dn11 = assign23050_e28210_d_n11;

        let (assign23060_e28221,) = {
    if (var_guard407 != 0.0) {
        let assign23060_e28214: f64 = (p.p322 * p.p322);
        let assign23060_e28217: f64 = (p.p38 * p.p38);
        let assign23060_e28218: f64 = (assign23060_e28214 + assign23060_e28217);
        let assign23060_e28219: f64 = (assign23060_e28218).sqrt();
        (assign23060_e28219,)
    } else {
        (var_xov__blk424,)
    }
};
        var_xov__blk424 = assign23060_e28221;

        let (assign23070_e28227, assign23070_e28227_d_n0, assign23070_e28227_d_n2, assign23070_e28227_d_n4, assign23070_e28227_d_n5, assign23070_e28227_d_n6, assign23070_e28227_d_n8, assign23070_e28227_d_n10, assign23070_e28227_d_n11, assign23070_e28227_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23070_e28225: f64 = (var_weff * p.p5);
        (assign23070_e28225, (var_weff_dn0 * p.p5), (var_weff_dn2 * p.p5), (var_weff_dn4 * p.p5), (var_weff_dn5 * p.p5), (var_weff_dn6 * p.p5), (var_weff_dn8 * p.p5), (var_weff_dn10 * p.p5), (var_weff_dn11 * p.p5), (var_weff_dn12 * p.p5),)
    } else {
        (var_weff_nf__blk426, var_weff_nf__blk426_dn0, var_weff_nf__blk426_dn2, var_weff_nf__blk426_dn4, var_weff_nf__blk426_dn5, var_weff_nf__blk426_dn6, var_weff_nf__blk426_dn8, var_weff_nf__blk426_dn10, var_weff_nf__blk426_dn11, var_weff_nf__blk426_dn12,)
    }
};
        var_weff_nf__blk426 = assign23070_e28227;
        var_weff_nf__blk426_dn0 = assign23070_e28227_d_n0;
        var_weff_nf__blk426_dn2 = assign23070_e28227_d_n2;
        var_weff_nf__blk426_dn4 = assign23070_e28227_d_n4;
        var_weff_nf__blk426_dn5 = assign23070_e28227_d_n5;
        var_weff_nf__blk426_dn6 = assign23070_e28227_d_n6;
        var_weff_nf__blk426_dn8 = assign23070_e28227_d_n8;
        var_weff_nf__blk426_dn10 = assign23070_e28227_d_n10;
        var_weff_nf__blk426_dn11 = assign23070_e28227_d_n11;
        var_weff_nf__blk426_dn12 = assign23070_e28227_d_n12;

        let (assign23080_e28233,) = {
    if (var_guard407 != 0.0) {
        let assign23080_e28231: f64 = (var_mks_rdrmue__blk411 / 10000.0);
        (assign23080_e28231,)
    } else {
        (var_mks_rdrmue__blk411,)
    }
};
        var_mks_rdrmue__blk411 = assign23080_e28233;

        let (assign23090_e28239,) = {
    if (var_guard407 != 0.0) {
        let assign23090_e28237: f64 = (var_mks_rdrvmax__blk412 / 100.0);
        (assign23090_e28237,)
    } else {
        (var_mks_rdrvmax__blk412,)
    }
};
        var_mks_rdrvmax__blk412 = assign23090_e28239;

        let (assign23100_e28245, assign23100_e28245_d_n4,) = {
    if (var_guard407 != 0.0) {
        let assign23100_e28243: f64 = (var_ttemp / var_uc_tnom);
        (assign23100_e28243, (var_ttemp_dn4 / var_uc_tnom),)
    } else {
        (var_tratio__blk416, var_tratio__blk416_dn4,)
    }
};
        var_tratio__blk416 = assign23100_e28245;
        var_tratio__blk416_dn4 = assign23100_e28245_d_n4;

        let (assign23110_e28251, assign23110_e28251_d_n0, assign23110_e28251_d_n2, assign23110_e28251_d_n4, assign23110_e28251_d_n5, assign23110_e28251_d_n6, assign23110_e28251_d_n8, assign23110_e28251_d_n10, assign23110_e28251_d_n11, assign23110_e28251_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23110_e28249: f64 = (var_tratio__blk416).powf(p.p320);
        (assign23110_e28249, 0.0, 0.0, if 0.0 == 0.0 && ((p.p320) as f64).is_finite() && ((p.p320) as f64).fract() == 0.0 { if p.p320 == 0.0 { 0.0 } else { (p.p320 * ((var_tratio__blk416).powf(p.p320 - 1.0) * var_tratio__blk416_dn4)) } } else { (assign23110_e28249 * (p.p320 * (var_tratio__blk416_dn4 / var_tratio__blk416))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign23110_e28251;
        var_t1_dn0 = assign23110_e28251_d_n0;
        var_t1_dn2 = assign23110_e28251_d_n2;
        var_t1_dn4 = assign23110_e28251_d_n4;
        var_t1_dn5 = assign23110_e28251_d_n5;
        var_t1_dn6 = assign23110_e28251_d_n6;
        var_t1_dn8 = assign23110_e28251_d_n8;
        var_t1_dn10 = assign23110_e28251_d_n10;
        var_t1_dn11 = assign23110_e28251_d_n11;
        var_t1_dn12 = assign23110_e28251_d_n12;

        let (assign23120_e28257, assign23120_e28257_d_n0, assign23120_e28257_d_n2, assign23120_e28257_d_n4, assign23120_e28257_d_n5, assign23120_e28257_d_n6, assign23120_e28257_d_n8, assign23120_e28257_d_n10, assign23120_e28257_d_n11, assign23120_e28257_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23120_e28255: f64 = (var_mks_rdrmue__blk411 / var_t1);
        (assign23120_e28255, (-((var_mks_rdrmue__blk411 * var_t1_dn0) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk411 * var_t1_dn2) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk411 * var_t1_dn4) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk411 * var_t1_dn5) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk411 * var_t1_dn6) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk411 * var_t1_dn8) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk411 * var_t1_dn10) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk411 * var_t1_dn11) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk411 * var_t1_dn12) / (var_t1 * var_t1))),)
    } else {
        (var_mu0__blk419, var_mu0__blk419_dn0, var_mu0__blk419_dn2, var_mu0__blk419_dn4, var_mu0__blk419_dn5, var_mu0__blk419_dn6, var_mu0__blk419_dn8, var_mu0__blk419_dn10, var_mu0__blk419_dn11, var_mu0__blk419_dn12,)
    }
};
        var_mu0__blk419 = assign23120_e28257;
        var_mu0__blk419_dn0 = assign23120_e28257_d_n0;
        var_mu0__blk419_dn2 = assign23120_e28257_d_n2;
        var_mu0__blk419_dn4 = assign23120_e28257_d_n4;
        var_mu0__blk419_dn5 = assign23120_e28257_d_n5;
        var_mu0__blk419_dn6 = assign23120_e28257_d_n6;
        var_mu0__blk419_dn8 = assign23120_e28257_d_n8;
        var_mu0__blk419_dn10 = assign23120_e28257_d_n10;
        var_mu0__blk419_dn11 = assign23120_e28257_d_n11;
        var_mu0__blk419_dn12 = assign23120_e28257_d_n12;

        let (assign23130_e28277, assign23130_e28277_d_n0, assign23130_e28277_d_n2, assign23130_e28277_d_n4, assign23130_e28277_d_n5, assign23130_e28277_d_n6, assign23130_e28277_d_n8, assign23130_e28277_d_n10, assign23130_e28277_d_n11, assign23130_e28277_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23130_e28262: f64 = (0.4 * var_tratio__blk416);
        let assign23130_e28263: f64 = (1.8 + assign23130_e28262);
        let assign23130_e28266: f64 = (0.1 * var_tratio__blk416);
        let assign23130_e28268: f64 = (assign23130_e28266 * var_tratio__blk416);
        let assign23130_e28269: f64 = (assign23130_e28263 + assign23130_e28268);
        let assign23130_e28273: f64 = (1.0 - var_tratio__blk416);
        let assign23130_e28274: f64 = (p.p321 * assign23130_e28273);
        let assign23130_e28275: f64 = (assign23130_e28269 - assign23130_e28274);
        (assign23130_e28275, 0.0, 0.0, (((0.4 * var_tratio__blk416_dn4) + (((0.1 * var_tratio__blk416_dn4) * var_tratio__blk416) + (assign23130_e28266 * var_tratio__blk416_dn4))) - (p.p321 * (-var_tratio__blk416_dn4))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
        var_t0 = assign23130_e28277;
        var_t0_dn0 = assign23130_e28277_d_n0;
        var_t0_dn2 = assign23130_e28277_d_n2;
        var_t0_dn4 = assign23130_e28277_d_n4;
        var_t0_dn5 = assign23130_e28277_d_n5;
        var_t0_dn6 = assign23130_e28277_d_n6;
        var_t0_dn8 = assign23130_e28277_d_n8;
        var_t0_dn10 = assign23130_e28277_d_n10;
        var_t0_dn11 = assign23130_e28277_d_n11;
        var_t0_dn12 = assign23130_e28277_d_n12;

        let (assign23140_e28283, assign23140_e28283_d_n0, assign23140_e28283_d_n2, assign23140_e28283_d_n4, assign23140_e28283_d_n5, assign23140_e28283_d_n6, assign23140_e28283_d_n8, assign23140_e28283_d_n10, assign23140_e28283_d_n11, assign23140_e28283_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23140_e28281: f64 = (var_mks_rdrvmax__blk412 / var_t0);
        (assign23140_e28281, (-((var_mks_rdrvmax__blk412 * var_t0_dn0) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk412 * var_t0_dn2) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk412 * var_t0_dn4) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk412 * var_t0_dn5) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk412 * var_t0_dn6) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk412 * var_t0_dn8) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk412 * var_t0_dn10) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk412 * var_t0_dn11) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk412 * var_t0_dn12) / (var_t0 * var_t0))),)
    } else {
        (var_vmaxe__blk420, var_vmaxe__blk420_dn0, var_vmaxe__blk420_dn2, var_vmaxe__blk420_dn4, var_vmaxe__blk420_dn5, var_vmaxe__blk420_dn6, var_vmaxe__blk420_dn8, var_vmaxe__blk420_dn10, var_vmaxe__blk420_dn11, var_vmaxe__blk420_dn12,)
    }
};
        var_vmaxe__blk420 = assign23140_e28283;
        var_vmaxe__blk420_dn0 = assign23140_e28283_d_n0;
        var_vmaxe__blk420_dn2 = assign23140_e28283_d_n2;
        var_vmaxe__blk420_dn4 = assign23140_e28283_d_n4;
        var_vmaxe__blk420_dn5 = assign23140_e28283_d_n5;
        var_vmaxe__blk420_dn6 = assign23140_e28283_d_n6;
        var_vmaxe__blk420_dn8 = assign23140_e28283_d_n8;
        var_vmaxe__blk420_dn10 = assign23140_e28283_d_n10;
        var_vmaxe__blk420_dn11 = assign23140_e28283_d_n11;
        var_vmaxe__blk420_dn12 = assign23140_e28283_d_n12;

        let (assign23150_e28293, assign23150_e28293_d_n4,) = {
    if (var_guard407 != 0.0) {
        let assign23150_e28289: f64 = (var_ttemp - var_uc_tnom);
        let assign23150_e28290: f64 = (p.p325 * assign23150_e28289);
        let assign23150_e28291: f64 = (var_rrdrbb__blk413 + assign23150_e28290);
        (assign23150_e28291, (var_rrdrbb__blk413_dn4 + (p.p325 * var_ttemp_dn4)),)
    } else {
        (var_rrdrbb__blk413, var_rrdrbb__blk413_dn4,)
    }
};
        var_rrdrbb__blk413 = assign23150_e28293;
        var_rrdrbb__blk413_dn4 = assign23150_e28293_d_n4;

        let (assign23160_e28303,) = {
    if (var_guard407 != 0.0) {
        let assign23160_e28299: f64 = (var_lg).powf(p.p331);
        let assign23160_e28300: f64 = (p.p330 / assign23160_e28299);
        let assign23160_e28301: f64 = (1.0 + assign23160_e28300);
        (assign23160_e28301,)
    } else {
        (var_rdrmuele__blk408,)
    }
};
        var_rdrmuele__blk408 = assign23160_e28303;

        let (assign23170_e28313,) = {
    if (var_guard407 != 0.0) {
        let assign23170_e28309: f64 = (var_lg).powf(p.p329);
        let assign23170_e28310: f64 = (p.p328 / assign23170_e28309);
        let assign23170_e28311: f64 = (1.0 + assign23170_e28310);
        (assign23170_e28311,)
    } else {
        (var_rdrvmaxle__blk410,)
    }
};
        var_rdrvmaxle__blk410 = assign23170_e28313;

        let (assign23180_e28323,) = {
    if (var_guard407 != 0.0) {
        let assign23180_e28319: f64 = (var_wg).powf(p.p327);
        let assign23180_e28320: f64 = (p.p326 / assign23180_e28319);
        let assign23180_e28321: f64 = (1.0 + assign23180_e28320);
        (assign23180_e28321,)
    } else {
        (var_rdrvmaxwe__blk409,)
    }
};
        var_rdrvmaxwe__blk409 = assign23180_e28323;

        let (assign23190_e28329, assign23190_e28329_d_n0, assign23190_e28329_d_n2, assign23190_e28329_d_n4, assign23190_e28329_d_n5, assign23190_e28329_d_n6, assign23190_e28329_d_n8, assign23190_e28329_d_n10, assign23190_e28329_d_n11, assign23190_e28329_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23190_e28327: f64 = (var_mu0__blk419 * var_rdrmuele__blk408);
        (assign23190_e28327, (var_mu0__blk419_dn0 * var_rdrmuele__blk408), (var_mu0__blk419_dn2 * var_rdrmuele__blk408), (var_mu0__blk419_dn4 * var_rdrmuele__blk408), (var_mu0__blk419_dn5 * var_rdrmuele__blk408), (var_mu0__blk419_dn6 * var_rdrmuele__blk408), (var_mu0__blk419_dn8 * var_rdrmuele__blk408), (var_mu0__blk419_dn10 * var_rdrmuele__blk408), (var_mu0__blk419_dn11 * var_rdrmuele__blk408), (var_mu0__blk419_dn12 * var_rdrmuele__blk408),)
    } else {
        (var_mu0__blk419, var_mu0__blk419_dn0, var_mu0__blk419_dn2, var_mu0__blk419_dn4, var_mu0__blk419_dn5, var_mu0__blk419_dn6, var_mu0__blk419_dn8, var_mu0__blk419_dn10, var_mu0__blk419_dn11, var_mu0__blk419_dn12,)
    }
};
        var_mu0__blk419 = assign23190_e28329;
        var_mu0__blk419_dn0 = assign23190_e28329_d_n0;
        var_mu0__blk419_dn2 = assign23190_e28329_d_n2;
        var_mu0__blk419_dn4 = assign23190_e28329_d_n4;
        var_mu0__blk419_dn5 = assign23190_e28329_d_n5;
        var_mu0__blk419_dn6 = assign23190_e28329_d_n6;
        var_mu0__blk419_dn8 = assign23190_e28329_d_n8;
        var_mu0__blk419_dn10 = assign23190_e28329_d_n10;
        var_mu0__blk419_dn11 = assign23190_e28329_d_n11;
        var_mu0__blk419_dn12 = assign23190_e28329_d_n12;

        let (assign23200_e28339, assign23200_e28339_d_n0, assign23200_e28339_d_n2, assign23200_e28339_d_n4, assign23200_e28339_d_n5, assign23200_e28339_d_n6, assign23200_e28339_d_n8, assign23200_e28339_d_n10, assign23200_e28339_d_n11, assign23200_e28339_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23200_e28333: f64 = (var_vmaxe__blk420 * var_rdrvmaxwe__blk409);
        let assign23200_e28335: f64 = (assign23200_e28333 * var_rdrvmaxle__blk410);
        let assign23200_e28337: f64 = (assign23200_e28335 + 1e-50);
        (assign23200_e28337, ((var_vmaxe__blk420_dn0 * var_rdrvmaxwe__blk409) * var_rdrvmaxle__blk410), ((var_vmaxe__blk420_dn2 * var_rdrvmaxwe__blk409) * var_rdrvmaxle__blk410), ((var_vmaxe__blk420_dn4 * var_rdrvmaxwe__blk409) * var_rdrvmaxle__blk410), ((var_vmaxe__blk420_dn5 * var_rdrvmaxwe__blk409) * var_rdrvmaxle__blk410), ((var_vmaxe__blk420_dn6 * var_rdrvmaxwe__blk409) * var_rdrvmaxle__blk410), ((var_vmaxe__blk420_dn8 * var_rdrvmaxwe__blk409) * var_rdrvmaxle__blk410), ((var_vmaxe__blk420_dn10 * var_rdrvmaxwe__blk409) * var_rdrvmaxle__blk410), ((var_vmaxe__blk420_dn11 * var_rdrvmaxwe__blk409) * var_rdrvmaxle__blk410), ((var_vmaxe__blk420_dn12 * var_rdrvmaxwe__blk409) * var_rdrvmaxle__blk410),)
    } else {
        (var_vmaxe__blk420, var_vmaxe__blk420_dn0, var_vmaxe__blk420_dn2, var_vmaxe__blk420_dn4, var_vmaxe__blk420_dn5, var_vmaxe__blk420_dn6, var_vmaxe__blk420_dn8, var_vmaxe__blk420_dn10, var_vmaxe__blk420_dn11, var_vmaxe__blk420_dn12,)
    }
};
        var_vmaxe__blk420 = assign23200_e28339;
        var_vmaxe__blk420_dn0 = assign23200_e28339_d_n0;
        var_vmaxe__blk420_dn2 = assign23200_e28339_d_n2;
        var_vmaxe__blk420_dn4 = assign23200_e28339_d_n4;
        var_vmaxe__blk420_dn5 = assign23200_e28339_d_n5;
        var_vmaxe__blk420_dn6 = assign23200_e28339_d_n6;
        var_vmaxe__blk420_dn8 = assign23200_e28339_d_n8;
        var_vmaxe__blk420_dn10 = assign23200_e28339_d_n10;
        var_vmaxe__blk420_dn11 = assign23200_e28339_d_n11;
        var_vmaxe__blk420_dn12 = assign23200_e28339_d_n12;

        let (assign23210_e28345, assign23210_e28345_d_n0, assign23210_e28345_d_n11,) = {
    if (var_guard407 != 0.0) {
        let assign23210_e28343: f64 = (var_vrdr__blk415 / var_ldrifte__blk417);
        (assign23210_e28343, (var_vrdr__blk415_dn0 / var_ldrifte__blk417), (var_vrdr__blk415_dn11 / var_ldrifte__blk417),)
    } else {
        (var_edri__blk421, var_edri__blk421_dn0, var_edri__blk421_dn11,)
    }
};
        var_edri__blk421 = assign23210_e28345;
        var_edri__blk421_dn0 = assign23210_e28345_d_n0;
        var_edri__blk421_dn11 = assign23210_e28345_d_n11;

        let (assign23220_e28351, assign23220_e28351_d_n0, assign23220_e28351_d_n2, assign23220_e28351_d_n4, assign23220_e28351_d_n5, assign23220_e28351_d_n6, assign23220_e28351_d_n8, assign23220_e28351_d_n10, assign23220_e28351_d_n11, assign23220_e28351_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23220_e28349: f64 = (var_mu0__blk419 * var_edri__blk421);
        (assign23220_e28349, ((var_mu0__blk419_dn0 * var_edri__blk421) + (var_mu0__blk419 * var_edri__blk421_dn0)), (var_mu0__blk419_dn2 * var_edri__blk421), (var_mu0__blk419_dn4 * var_edri__blk421), (var_mu0__blk419_dn5 * var_edri__blk421), (var_mu0__blk419_dn6 * var_edri__blk421), (var_mu0__blk419_dn8 * var_edri__blk421), (var_mu0__blk419_dn10 * var_edri__blk421), ((var_mu0__blk419_dn11 * var_edri__blk421) + (var_mu0__blk419 * var_edri__blk421_dn11)), (var_mu0__blk419_dn12 * var_edri__blk421),)
    } else {
        (var_vdri__blk422, var_vdri__blk422_dn0, var_vdri__blk422_dn2, var_vdri__blk422_dn4, var_vdri__blk422_dn5, var_vdri__blk422_dn6, var_vdri__blk422_dn8, var_vdri__blk422_dn10, var_vdri__blk422_dn11, var_vdri__blk422_dn12,)
    }
};
        var_vdri__blk422 = assign23220_e28351;
        var_vdri__blk422_dn0 = assign23220_e28351_d_n0;
        var_vdri__blk422_dn2 = assign23220_e28351_d_n2;
        var_vdri__blk422_dn4 = assign23220_e28351_d_n4;
        var_vdri__blk422_dn5 = assign23220_e28351_d_n5;
        var_vdri__blk422_dn6 = assign23220_e28351_d_n6;
        var_vdri__blk422_dn8 = assign23220_e28351_d_n8;
        var_vdri__blk422_dn10 = assign23220_e28351_d_n10;
        var_vdri__blk422_dn11 = assign23220_e28351_d_n11;
        var_vdri__blk422_dn12 = assign23220_e28351_d_n12;

        let assign23230_e28354: f64 = if var_vrdr__blk415 >= 0.0 { 1.0 } else { 0.0 };
        var_guard427 = assign23230_e28354;

        let (assign23240_e28362, assign23240_e28362_d_n0, assign23240_e28362_d_n2, assign23240_e28362_d_n4, assign23240_e28362_d_n5, assign23240_e28362_d_n6, assign23240_e28362_d_n8, assign23240_e28362_d_n10, assign23240_e28362_d_n11, assign23240_e28362_d_n12,) = {
    if ((var_guard407 != 0.0) && (var_guard427 != 0.0)) {
        let assign23240_e28360: f64 = (var_vdri__blk422 / var_vmaxe__blk420);
        (assign23240_e28360, (((var_vdri__blk422_dn0 * var_vmaxe__blk420) - (var_vdri__blk422 * var_vmaxe__blk420_dn0)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), (((var_vdri__blk422_dn2 * var_vmaxe__blk420) - (var_vdri__blk422 * var_vmaxe__blk420_dn2)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), (((var_vdri__blk422_dn4 * var_vmaxe__blk420) - (var_vdri__blk422 * var_vmaxe__blk420_dn4)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), (((var_vdri__blk422_dn5 * var_vmaxe__blk420) - (var_vdri__blk422 * var_vmaxe__blk420_dn5)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), (((var_vdri__blk422_dn6 * var_vmaxe__blk420) - (var_vdri__blk422 * var_vmaxe__blk420_dn6)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), (((var_vdri__blk422_dn8 * var_vmaxe__blk420) - (var_vdri__blk422 * var_vmaxe__blk420_dn8)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), (((var_vdri__blk422_dn10 * var_vmaxe__blk420) - (var_vdri__blk422 * var_vmaxe__blk420_dn10)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), (((var_vdri__blk422_dn11 * var_vmaxe__blk420) - (var_vdri__blk422 * var_vmaxe__blk420_dn11)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), (((var_vdri__blk422_dn12 * var_vmaxe__blk420) - (var_vdri__blk422 * var_vmaxe__blk420_dn12)) / (var_vmaxe__blk420 * var_vmaxe__blk420)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign23240_e28362;
        var_t1_dn0 = assign23240_e28362_d_n0;
        var_t1_dn2 = assign23240_e28362_d_n2;
        var_t1_dn4 = assign23240_e28362_d_n4;
        var_t1_dn5 = assign23240_e28362_d_n5;
        var_t1_dn6 = assign23240_e28362_d_n6;
        var_t1_dn8 = assign23240_e28362_d_n8;
        var_t1_dn10 = assign23240_e28362_d_n10;
        var_t1_dn11 = assign23240_e28362_d_n11;
        var_t1_dn12 = assign23240_e28362_d_n12;

        let (assign23250_e28372, assign23250_e28372_d_n0, assign23250_e28372_d_n2, assign23250_e28372_d_n4, assign23250_e28372_d_n5, assign23250_e28372_d_n6, assign23250_e28372_d_n8, assign23250_e28372_d_n10, assign23250_e28372_d_n11, assign23250_e28372_d_n12,) = {
    if ((var_guard407 != 0.0) && (var_guard427 == 0.0)) {
        let assign23250_e28368: f64 = (-var_vdri__blk422);
        let assign23250_e28370: f64 = (assign23250_e28368 / var_vmaxe__blk420);
        (assign23250_e28370, ((((-var_vdri__blk422_dn0) * var_vmaxe__blk420) - (assign23250_e28368 * var_vmaxe__blk420_dn0)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), ((((-var_vdri__blk422_dn2) * var_vmaxe__blk420) - (assign23250_e28368 * var_vmaxe__blk420_dn2)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), ((((-var_vdri__blk422_dn4) * var_vmaxe__blk420) - (assign23250_e28368 * var_vmaxe__blk420_dn4)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), ((((-var_vdri__blk422_dn5) * var_vmaxe__blk420) - (assign23250_e28368 * var_vmaxe__blk420_dn5)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), ((((-var_vdri__blk422_dn6) * var_vmaxe__blk420) - (assign23250_e28368 * var_vmaxe__blk420_dn6)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), ((((-var_vdri__blk422_dn8) * var_vmaxe__blk420) - (assign23250_e28368 * var_vmaxe__blk420_dn8)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), ((((-var_vdri__blk422_dn10) * var_vmaxe__blk420) - (assign23250_e28368 * var_vmaxe__blk420_dn10)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), ((((-var_vdri__blk422_dn11) * var_vmaxe__blk420) - (assign23250_e28368 * var_vmaxe__blk420_dn11)) / (var_vmaxe__blk420 * var_vmaxe__blk420)), ((((-var_vdri__blk422_dn12) * var_vmaxe__blk420) - (assign23250_e28368 * var_vmaxe__blk420_dn12)) / (var_vmaxe__blk420 * var_vmaxe__blk420)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign23250_e28372;
        var_t1_dn0 = assign23250_e28372_d_n0;
        var_t1_dn2 = assign23250_e28372_d_n2;
        var_t1_dn4 = assign23250_e28372_d_n4;
        var_t1_dn5 = assign23250_e28372_d_n5;
        var_t1_dn6 = assign23250_e28372_d_n6;
        var_t1_dn8 = assign23250_e28372_d_n8;
        var_t1_dn10 = assign23250_e28372_d_n10;
        var_t1_dn11 = assign23250_e28372_d_n11;
        var_t1_dn12 = assign23250_e28372_d_n12;

        let assign23260_e28376: f64 = (10.0 * 2.220446049250313e-16);
        let assign23260_e28377: f64 = (1.0 - assign23260_e28376);
        let assign23260_e28384: f64 = (10.0 * 2.220446049250313e-16);
        let assign23260_e28385: f64 = (1.0 + assign23260_e28384);
        let assign23260_e28387: f64 = if ((assign23260_e28377 <= var_rrdrbb__blk413) && (var_rrdrbb__blk413 <= assign23260_e28385)) { 1.0 } else { 0.0 };
        var_guard428 = assign23260_e28387;

        let (assign23270_e28393, assign23270_e28393_d_n0, assign23270_e28393_d_n2, assign23270_e28393_d_n4, assign23270_e28393_d_n5, assign23270_e28393_d_n6, assign23270_e28393_d_n8, assign23270_e28393_d_n10, assign23270_e28393_d_n11, assign23270_e28393_d_n12,) = {
    if ((var_guard407 != 0.0) && (var_guard428 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign23270_e28393;
        var_t3_dn0 = assign23270_e28393_d_n0;
        var_t3_dn2 = assign23270_e28393_d_n2;
        var_t3_dn4 = assign23270_e28393_d_n4;
        var_t3_dn5 = assign23270_e28393_d_n5;
        var_t3_dn6 = assign23270_e28393_d_n6;
        var_t3_dn8 = assign23270_e28393_d_n8;
        var_t3_dn10 = assign23270_e28393_d_n10;
        var_t3_dn11 = assign23270_e28393_d_n11;
        var_t3_dn12 = assign23270_e28393_d_n12;

        *var_edri__blk421_slot = var_edri__blk421;
        *var_edri__blk421_dn0_slot = var_edri__blk421_dn0;
        *var_edri__blk421_dn11_slot = var_edri__blk421_dn11;
        *var_guard406_slot = var_guard406;
        *var_guard407_slot = var_guard407;
        *var_guard427_slot = var_guard427;
        *var_guard428_slot = var_guard428;
        *var_ldrifte__blk417_slot = var_ldrifte__blk417;
        *var_mks_rdrmue__blk411_slot = var_mks_rdrmue__blk411;
        *var_mks_rdrvmax__blk412_slot = var_mks_rdrvmax__blk412;
        *var_mu0__blk419_slot = var_mu0__blk419;
        *var_mu0__blk419_dn0_slot = var_mu0__blk419_dn0;
        *var_mu0__blk419_dn10_slot = var_mu0__blk419_dn10;
        *var_mu0__blk419_dn11_slot = var_mu0__blk419_dn11;
        *var_mu0__blk419_dn12_slot = var_mu0__blk419_dn12;
        *var_mu0__blk419_dn2_slot = var_mu0__blk419_dn2;
        *var_mu0__blk419_dn4_slot = var_mu0__blk419_dn4;
        *var_mu0__blk419_dn5_slot = var_mu0__blk419_dn5;
        *var_mu0__blk419_dn6_slot = var_mu0__blk419_dn6;
        *var_mu0__blk419_dn8_slot = var_mu0__blk419_dn8;
        *var_nover__blk418_slot = var_nover__blk418;
        *var_rdrmuele__blk408_slot = var_rdrmuele__blk408;
        *var_rdrvmaxle__blk410_slot = var_rdrvmaxle__blk410;
        *var_rdrvmaxwe__blk409_slot = var_rdrvmaxwe__blk409;
        *var_rrdrbb__blk413_slot = var_rrdrbb__blk413;
        *var_rrdrbb__blk413_dn4_slot = var_rrdrbb__blk413_dn4;
        *var_rsd_slot = var_rsd;
        *var_rsd0__blk414_slot = var_rsd0__blk414;
        *var_rsd_dn0_slot = var_rsd_dn0;
        *var_rsd_dn10_slot = var_rsd_dn10;
        *var_rsd_dn11_slot = var_rsd_dn11;
        *var_rsd_dn12_slot = var_rsd_dn12;
        *var_rsd_dn2_slot = var_rsd_dn2;
        *var_rsd_dn4_slot = var_rsd_dn4;
        *var_rsd_dn5_slot = var_rsd_dn5;
        *var_rsd_dn6_slot = var_rsd_dn6;
        *var_rsd_dn8_slot = var_rsd_dn8;
        *var_rsde_slot = var_rsde;
        *var_rsde_dn0_slot = var_rsde_dn0;
        *var_rsde_dn10_slot = var_rsde_dn10;
        *var_rsde_dn11_slot = var_rsde_dn11;
        *var_rsde_dn12_slot = var_rsde_dn12;
        *var_rsde_dn2_slot = var_rsde_dn2;
        *var_rsde_dn4_slot = var_rsde_dn4;
        *var_rsde_dn5_slot = var_rsde_dn5;
        *var_rsde_dn6_slot = var_rsde_dn6;
        *var_rsde_dn8_slot = var_rsde_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_tratio__blk416_slot = var_tratio__blk416;
        *var_tratio__blk416_dn4_slot = var_tratio__blk416_dn4;
        *var_vdri__blk422_slot = var_vdri__blk422;
        *var_vdri__blk422_dn0_slot = var_vdri__blk422_dn0;
        *var_vdri__blk422_dn10_slot = var_vdri__blk422_dn10;
        *var_vdri__blk422_dn11_slot = var_vdri__blk422_dn11;
        *var_vdri__blk422_dn12_slot = var_vdri__blk422_dn12;
        *var_vdri__blk422_dn2_slot = var_vdri__blk422_dn2;
        *var_vdri__blk422_dn4_slot = var_vdri__blk422_dn4;
        *var_vdri__blk422_dn5_slot = var_vdri__blk422_dn5;
        *var_vdri__blk422_dn6_slot = var_vdri__blk422_dn6;
        *var_vdri__blk422_dn8_slot = var_vdri__blk422_dn8;
        *var_vmaxe__blk420_slot = var_vmaxe__blk420;
        *var_vmaxe__blk420_dn0_slot = var_vmaxe__blk420_dn0;
        *var_vmaxe__blk420_dn10_slot = var_vmaxe__blk420_dn10;
        *var_vmaxe__blk420_dn11_slot = var_vmaxe__blk420_dn11;
        *var_vmaxe__blk420_dn12_slot = var_vmaxe__blk420_dn12;
        *var_vmaxe__blk420_dn2_slot = var_vmaxe__blk420_dn2;
        *var_vmaxe__blk420_dn4_slot = var_vmaxe__blk420_dn4;
        *var_vmaxe__blk420_dn5_slot = var_vmaxe__blk420_dn5;
        *var_vmaxe__blk420_dn6_slot = var_vmaxe__blk420_dn6;
        *var_vmaxe__blk420_dn8_slot = var_vmaxe__blk420_dn8;
        *var_vrdr__blk415_slot = var_vrdr__blk415;
        *var_vrdr__blk415_dn0_slot = var_vrdr__blk415_dn0;
        *var_vrdr__blk415_dn11_slot = var_vrdr__blk415_dn11;
        *var_weff_nf__blk426_slot = var_weff_nf__blk426;
        *var_weff_nf__blk426_dn0_slot = var_weff_nf__blk426_dn0;
        *var_weff_nf__blk426_dn10_slot = var_weff_nf__blk426_dn10;
        *var_weff_nf__blk426_dn11_slot = var_weff_nf__blk426_dn11;
        *var_weff_nf__blk426_dn12_slot = var_weff_nf__blk426_dn12;
        *var_weff_nf__blk426_dn2_slot = var_weff_nf__blk426_dn2;
        *var_weff_nf__blk426_dn4_slot = var_weff_nf__blk426_dn4;
        *var_weff_nf__blk426_dn5_slot = var_weff_nf__blk426_dn5;
        *var_weff_nf__blk426_dn6_slot = var_weff_nf__blk426_dn6;
        *var_weff_nf__blk426_dn8_slot = var_weff_nf__blk426_dn8;
        *var_xov__blk424_slot = var_xov__blk424;
    }

    pub(super) fn stamp_transient_block_88(
        var_flg_nqs: f64,
        var_guard407: f64,
        var_guard428: f64,
        var_ldrifte__blk417: f64,
        var_mfactor: f64,
        var_mu0__blk419: f64,
        var_mu0__blk419_dn0: f64,
        var_mu0__blk419_dn10: f64,
        var_mu0__blk419_dn11: f64,
        var_mu0__blk419_dn12: f64,
        var_mu0__blk419_dn2: f64,
        var_mu0__blk419_dn4: f64,
        var_mu0__blk419_dn5: f64,
        var_mu0__blk419_dn6: f64,
        var_mu0__blk419_dn8: f64,
        var_nover__blk418: f64,
        var_qb_nqs: f64,
        var_qb_nqs_dn9: f64,
        var_qb_qs: f64,
        var_qb_qs_dn0: f64,
        var_qb_qs_dn10: f64,
        var_qb_qs_dn11: f64,
        var_qb_qs_dn12: f64,
        var_qb_qs_dn2: f64,
        var_qb_qs_dn4: f64,
        var_qb_qs_dn5: f64,
        var_qb_qs_dn6: f64,
        var_qb_qs_dn8: f64,
        var_qdrat: f64,
        var_qi_nqs: f64,
        var_qi_nqs_dn8: f64,
        var_qi_qs: f64,
        var_qi_qs_dn0: f64,
        var_qi_qs_dn10: f64,
        var_qi_qs_dn11: f64,
        var_qi_qs_dn12: f64,
        var_qi_qs_dn2: f64,
        var_qi_qs_dn4: f64,
        var_qi_qs_dn5: f64,
        var_qi_qs_dn6: f64,
        var_qi_qs_dn8: f64,
        var_rrdrbb__blk413: f64,
        var_rrdrbb__blk413_dn4: f64,
        var_rsd0__blk414: f64,
        var_weff_nf__blk426: f64,
        var_weff_nf__blk426_dn0: f64,
        var_weff_nf__blk426_dn10: f64,
        var_weff_nf__blk426_dn11: f64,
        var_weff_nf__blk426_dn12: f64,
        var_weff_nf__blk426_dn2: f64,
        var_weff_nf__blk426_dn4: f64,
        var_weff_nf__blk426_dn5: f64,
        var_weff_nf__blk426_dn6: f64,
        var_weff_nf__blk426_dn8: f64,
        var_xov__blk424: f64,
        var_gd__blk425_slot: &mut f64,
        var_gd__blk425_dn0_slot: &mut f64,
        var_gd__blk425_dn10_slot: &mut f64,
        var_gd__blk425_dn11_slot: &mut f64,
        var_gd__blk425_dn12_slot: &mut f64,
        var_gd__blk425_dn2_slot: &mut f64,
        var_gd__blk425_dn4_slot: &mut f64,
        var_gd__blk425_dn5_slot: &mut f64,
        var_gd__blk425_dn6_slot: &mut f64,
        var_gd__blk425_dn8_slot: &mut f64,
        var_guard429_slot: &mut f64,
        var_guard430_slot: &mut f64,
        var_guard431_slot: &mut f64,
        var_guard432_slot: &mut f64,
        var_guard433_slot: &mut f64,
        var_guard434_slot: &mut f64,
        var_guard435_slot: &mut f64,
        var_iqb_nqs_slot: &mut f64,
        var_iqb_nqs_dn0_slot: &mut f64,
        var_iqb_nqs_dn10_slot: &mut f64,
        var_iqb_nqs_dn11_slot: &mut f64,
        var_iqb_nqs_dn12_slot: &mut f64,
        var_iqb_nqs_dn2_slot: &mut f64,
        var_iqb_nqs_dn4_slot: &mut f64,
        var_iqb_nqs_dn5_slot: &mut f64,
        var_iqb_nqs_dn6_slot: &mut f64,
        var_iqb_nqs_dn8_slot: &mut f64,
        var_iqb_nqs_dn9_slot: &mut f64,
        var_iqi_nqs_slot: &mut f64,
        var_iqi_nqs_dn0_slot: &mut f64,
        var_iqi_nqs_dn10_slot: &mut f64,
        var_iqi_nqs_dn11_slot: &mut f64,
        var_iqi_nqs_dn12_slot: &mut f64,
        var_iqi_nqs_dn2_slot: &mut f64,
        var_iqi_nqs_dn4_slot: &mut f64,
        var_iqi_nqs_dn5_slot: &mut f64,
        var_iqi_nqs_dn6_slot: &mut f64,
        var_iqi_nqs_dn8_slot: &mut f64,
        var_mu__blk423_slot: &mut f64,
        var_mu__blk423_dn0_slot: &mut f64,
        var_mu__blk423_dn10_slot: &mut f64,
        var_mu__blk423_dn11_slot: &mut f64,
        var_mu__blk423_dn12_slot: &mut f64,
        var_mu__blk423_dn2_slot: &mut f64,
        var_mu__blk423_dn4_slot: &mut f64,
        var_mu__blk423_dn5_slot: &mut f64,
        var_mu__blk423_dn6_slot: &mut f64,
        var_mu__blk423_dn8_slot: &mut f64,
        var_qd_nqs_slot: &mut f64,
        var_qd_nqs_dn0_slot: &mut f64,
        var_qd_nqs_dn10_slot: &mut f64,
        var_qd_nqs_dn11_slot: &mut f64,
        var_qd_nqs_dn12_slot: &mut f64,
        var_qd_nqs_dn2_slot: &mut f64,
        var_qd_nqs_dn4_slot: &mut f64,
        var_qd_nqs_dn5_slot: &mut f64,
        var_qd_nqs_dn6_slot: &mut f64,
        var_qd_nqs_dn8_slot: &mut f64,
        var_qg_nqs_slot: &mut f64,
        var_qg_nqs_dn8_slot: &mut f64,
        var_qg_nqs_dn9_slot: &mut f64,
        var_qs_nqs_slot: &mut f64,
        var_qs_nqs_dn0_slot: &mut f64,
        var_qs_nqs_dn10_slot: &mut f64,
        var_qs_nqs_dn11_slot: &mut f64,
        var_qs_nqs_dn12_slot: &mut f64,
        var_qs_nqs_dn2_slot: &mut f64,
        var_qs_nqs_dn4_slot: &mut f64,
        var_qs_nqs_dn5_slot: &mut f64,
        var_qs_nqs_dn6_slot: &mut f64,
        var_qs_nqs_dn8_slot: &mut f64,
        var_rdde_slot: &mut f64,
        var_rdde_dn0_slot: &mut f64,
        var_rdde_dn10_slot: &mut f64,
        var_rdde_dn11_slot: &mut f64,
        var_rdde_dn12_slot: &mut f64,
        var_rdde_dn2_slot: &mut f64,
        var_rdde_dn4_slot: &mut f64,
        var_rdde_dn5_slot: &mut f64,
        var_rdde_dn6_slot: &mut f64,
        var_rdde_dn8_slot: &mut f64,
        var_rsd_slot: &mut f64,
        var_rsd_dn0_slot: &mut f64,
        var_rsd_dn10_slot: &mut f64,
        var_rsd_dn11_slot: &mut f64,
        var_rsd_dn12_slot: &mut f64,
        var_rsd_dn2_slot: &mut f64,
        var_rsd_dn4_slot: &mut f64,
        var_rsd_dn5_slot: &mut f64,
        var_rsd_dn6_slot: &mut f64,
        var_rsd_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_tau_slot: &mut f64,
        var_tau_dn0_slot: &mut f64,
        var_tau_dn10_slot: &mut f64,
        var_tau_dn11_slot: &mut f64,
        var_tau_dn12_slot: &mut f64,
        var_tau_dn2_slot: &mut f64,
        var_tau_dn4_slot: &mut f64,
        var_tau_dn5_slot: &mut f64,
        var_tau_dn6_slot: &mut f64,
        var_tau_dn8_slot: &mut f64,
        var_taub_slot: &mut f64,
        var_taub_dn0_slot: &mut f64,
        var_taub_dn10_slot: &mut f64,
        var_taub_dn11_slot: &mut f64,
        var_taub_dn12_slot: &mut f64,
        var_taub_dn2_slot: &mut f64,
        var_taub_dn4_slot: &mut f64,
        var_taub_dn5_slot: &mut f64,
        var_taub_dn6_slot: &mut f64,
        var_taub_dn8_slot: &mut f64,
    ) {
        let mut var_gd__blk425: f64 = *var_gd__blk425_slot;
        let mut var_gd__blk425_dn0: f64 = *var_gd__blk425_dn0_slot;
        let mut var_gd__blk425_dn10: f64 = *var_gd__blk425_dn10_slot;
        let mut var_gd__blk425_dn11: f64 = *var_gd__blk425_dn11_slot;
        let mut var_gd__blk425_dn12: f64 = *var_gd__blk425_dn12_slot;
        let mut var_gd__blk425_dn2: f64 = *var_gd__blk425_dn2_slot;
        let mut var_gd__blk425_dn4: f64 = *var_gd__blk425_dn4_slot;
        let mut var_gd__blk425_dn5: f64 = *var_gd__blk425_dn5_slot;
        let mut var_gd__blk425_dn6: f64 = *var_gd__blk425_dn6_slot;
        let mut var_gd__blk425_dn8: f64 = *var_gd__blk425_dn8_slot;
        let mut var_guard429: f64 = *var_guard429_slot;
        let mut var_guard430: f64 = *var_guard430_slot;
        let mut var_guard431: f64 = *var_guard431_slot;
        let mut var_guard432: f64 = *var_guard432_slot;
        let mut var_guard433: f64 = *var_guard433_slot;
        let mut var_guard434: f64 = *var_guard434_slot;
        let mut var_guard435: f64 = *var_guard435_slot;
        let mut var_iqb_nqs: f64 = *var_iqb_nqs_slot;
        let mut var_iqb_nqs_dn0: f64 = *var_iqb_nqs_dn0_slot;
        let mut var_iqb_nqs_dn10: f64 = *var_iqb_nqs_dn10_slot;
        let mut var_iqb_nqs_dn11: f64 = *var_iqb_nqs_dn11_slot;
        let mut var_iqb_nqs_dn12: f64 = *var_iqb_nqs_dn12_slot;
        let mut var_iqb_nqs_dn2: f64 = *var_iqb_nqs_dn2_slot;
        let mut var_iqb_nqs_dn4: f64 = *var_iqb_nqs_dn4_slot;
        let mut var_iqb_nqs_dn5: f64 = *var_iqb_nqs_dn5_slot;
        let mut var_iqb_nqs_dn6: f64 = *var_iqb_nqs_dn6_slot;
        let mut var_iqb_nqs_dn8: f64 = *var_iqb_nqs_dn8_slot;
        let mut var_iqb_nqs_dn9: f64 = *var_iqb_nqs_dn9_slot;
        let mut var_iqi_nqs: f64 = *var_iqi_nqs_slot;
        let mut var_iqi_nqs_dn0: f64 = *var_iqi_nqs_dn0_slot;
        let mut var_iqi_nqs_dn10: f64 = *var_iqi_nqs_dn10_slot;
        let mut var_iqi_nqs_dn11: f64 = *var_iqi_nqs_dn11_slot;
        let mut var_iqi_nqs_dn12: f64 = *var_iqi_nqs_dn12_slot;
        let mut var_iqi_nqs_dn2: f64 = *var_iqi_nqs_dn2_slot;
        let mut var_iqi_nqs_dn4: f64 = *var_iqi_nqs_dn4_slot;
        let mut var_iqi_nqs_dn5: f64 = *var_iqi_nqs_dn5_slot;
        let mut var_iqi_nqs_dn6: f64 = *var_iqi_nqs_dn6_slot;
        let mut var_iqi_nqs_dn8: f64 = *var_iqi_nqs_dn8_slot;
        let mut var_mu__blk423: f64 = *var_mu__blk423_slot;
        let mut var_mu__blk423_dn0: f64 = *var_mu__blk423_dn0_slot;
        let mut var_mu__blk423_dn10: f64 = *var_mu__blk423_dn10_slot;
        let mut var_mu__blk423_dn11: f64 = *var_mu__blk423_dn11_slot;
        let mut var_mu__blk423_dn12: f64 = *var_mu__blk423_dn12_slot;
        let mut var_mu__blk423_dn2: f64 = *var_mu__blk423_dn2_slot;
        let mut var_mu__blk423_dn4: f64 = *var_mu__blk423_dn4_slot;
        let mut var_mu__blk423_dn5: f64 = *var_mu__blk423_dn5_slot;
        let mut var_mu__blk423_dn6: f64 = *var_mu__blk423_dn6_slot;
        let mut var_mu__blk423_dn8: f64 = *var_mu__blk423_dn8_slot;
        let mut var_qd_nqs: f64 = *var_qd_nqs_slot;
        let mut var_qd_nqs_dn0: f64 = *var_qd_nqs_dn0_slot;
        let mut var_qd_nqs_dn10: f64 = *var_qd_nqs_dn10_slot;
        let mut var_qd_nqs_dn11: f64 = *var_qd_nqs_dn11_slot;
        let mut var_qd_nqs_dn12: f64 = *var_qd_nqs_dn12_slot;
        let mut var_qd_nqs_dn2: f64 = *var_qd_nqs_dn2_slot;
        let mut var_qd_nqs_dn4: f64 = *var_qd_nqs_dn4_slot;
        let mut var_qd_nqs_dn5: f64 = *var_qd_nqs_dn5_slot;
        let mut var_qd_nqs_dn6: f64 = *var_qd_nqs_dn6_slot;
        let mut var_qd_nqs_dn8: f64 = *var_qd_nqs_dn8_slot;
        let mut var_qg_nqs: f64 = *var_qg_nqs_slot;
        let mut var_qg_nqs_dn8: f64 = *var_qg_nqs_dn8_slot;
        let mut var_qg_nqs_dn9: f64 = *var_qg_nqs_dn9_slot;
        let mut var_qs_nqs: f64 = *var_qs_nqs_slot;
        let mut var_qs_nqs_dn0: f64 = *var_qs_nqs_dn0_slot;
        let mut var_qs_nqs_dn10: f64 = *var_qs_nqs_dn10_slot;
        let mut var_qs_nqs_dn11: f64 = *var_qs_nqs_dn11_slot;
        let mut var_qs_nqs_dn12: f64 = *var_qs_nqs_dn12_slot;
        let mut var_qs_nqs_dn2: f64 = *var_qs_nqs_dn2_slot;
        let mut var_qs_nqs_dn4: f64 = *var_qs_nqs_dn4_slot;
        let mut var_qs_nqs_dn5: f64 = *var_qs_nqs_dn5_slot;
        let mut var_qs_nqs_dn6: f64 = *var_qs_nqs_dn6_slot;
        let mut var_qs_nqs_dn8: f64 = *var_qs_nqs_dn8_slot;
        let mut var_rdde: f64 = *var_rdde_slot;
        let mut var_rdde_dn0: f64 = *var_rdde_dn0_slot;
        let mut var_rdde_dn10: f64 = *var_rdde_dn10_slot;
        let mut var_rdde_dn11: f64 = *var_rdde_dn11_slot;
        let mut var_rdde_dn12: f64 = *var_rdde_dn12_slot;
        let mut var_rdde_dn2: f64 = *var_rdde_dn2_slot;
        let mut var_rdde_dn4: f64 = *var_rdde_dn4_slot;
        let mut var_rdde_dn5: f64 = *var_rdde_dn5_slot;
        let mut var_rdde_dn6: f64 = *var_rdde_dn6_slot;
        let mut var_rdde_dn8: f64 = *var_rdde_dn8_slot;
        let mut var_rsd: f64 = *var_rsd_slot;
        let mut var_rsd_dn0: f64 = *var_rsd_dn0_slot;
        let mut var_rsd_dn10: f64 = *var_rsd_dn10_slot;
        let mut var_rsd_dn11: f64 = *var_rsd_dn11_slot;
        let mut var_rsd_dn12: f64 = *var_rsd_dn12_slot;
        let mut var_rsd_dn2: f64 = *var_rsd_dn2_slot;
        let mut var_rsd_dn4: f64 = *var_rsd_dn4_slot;
        let mut var_rsd_dn5: f64 = *var_rsd_dn5_slot;
        let mut var_rsd_dn6: f64 = *var_rsd_dn6_slot;
        let mut var_rsd_dn8: f64 = *var_rsd_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_tau: f64 = *var_tau_slot;
        let mut var_tau_dn0: f64 = *var_tau_dn0_slot;
        let mut var_tau_dn10: f64 = *var_tau_dn10_slot;
        let mut var_tau_dn11: f64 = *var_tau_dn11_slot;
        let mut var_tau_dn12: f64 = *var_tau_dn12_slot;
        let mut var_tau_dn2: f64 = *var_tau_dn2_slot;
        let mut var_tau_dn4: f64 = *var_tau_dn4_slot;
        let mut var_tau_dn5: f64 = *var_tau_dn5_slot;
        let mut var_tau_dn6: f64 = *var_tau_dn6_slot;
        let mut var_tau_dn8: f64 = *var_tau_dn8_slot;
        let mut var_taub: f64 = *var_taub_slot;
        let mut var_taub_dn0: f64 = *var_taub_dn0_slot;
        let mut var_taub_dn10: f64 = *var_taub_dn10_slot;
        let mut var_taub_dn11: f64 = *var_taub_dn11_slot;
        let mut var_taub_dn12: f64 = *var_taub_dn12_slot;
        let mut var_taub_dn2: f64 = *var_taub_dn2_slot;
        let mut var_taub_dn4: f64 = *var_taub_dn4_slot;
        let mut var_taub_dn5: f64 = *var_taub_dn5_slot;
        let mut var_taub_dn6: f64 = *var_taub_dn6_slot;
        let mut var_taub_dn8: f64 = *var_taub_dn8_slot;

        let assign23280_e28397: f64 = (10.0 * 2.220446049250313e-16);
        let assign23280_e28398: f64 = (2.0 - assign23280_e28397);
        let assign23280_e28405: f64 = (10.0 * 2.220446049250313e-16);
        let assign23280_e28406: f64 = (2.0 + assign23280_e28405);
        let assign23280_e28408: f64 = if ((assign23280_e28398 <= var_rrdrbb__blk413) && (var_rrdrbb__blk413 <= assign23280_e28406)) { 1.0 } else { 0.0 };
        var_guard429 = assign23280_e28408;

        let (assign23290_e28417, assign23290_e28417_d_n0, assign23290_e28417_d_n2, assign23290_e28417_d_n4, assign23290_e28417_d_n5, assign23290_e28417_d_n6, assign23290_e28417_d_n8, assign23290_e28417_d_n10, assign23290_e28417_d_n11, assign23290_e28417_d_n12,) = {
    if (((var_guard407 != 0.0) && (var_guard428 == 0.0)) && (var_guard429 != 0.0)) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign23290_e28417;
        var_t3_dn0 = assign23290_e28417_d_n0;
        var_t3_dn2 = assign23290_e28417_d_n2;
        var_t3_dn4 = assign23290_e28417_d_n4;
        var_t3_dn5 = assign23290_e28417_d_n5;
        var_t3_dn6 = assign23290_e28417_d_n6;
        var_t3_dn8 = assign23290_e28417_d_n8;
        var_t3_dn10 = assign23290_e28417_d_n10;
        var_t3_dn11 = assign23290_e28417_d_n11;
        var_t3_dn12 = assign23290_e28417_d_n12;

        let (assign23300_e28431, assign23300_e28431_d_n0, assign23300_e28431_d_n2, assign23300_e28431_d_n4, assign23300_e28431_d_n5, assign23300_e28431_d_n6, assign23300_e28431_d_n8, assign23300_e28431_d_n10, assign23300_e28431_d_n11, assign23300_e28431_d_n12,) = {
    if (((var_guard407 != 0.0) && (var_guard428 == 0.0)) && (var_guard429 == 0.0)) {
        let assign23300_e28428: f64 = (var_rrdrbb__blk413 - 1.0);
        let assign23300_e28429: f64 = (var_t1).powf(assign23300_e28428);
        (assign23300_e28429, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((var_t1).powf(assign23300_e28428 - 1.0) * var_t1_dn0)) } } else { (assign23300_e28429 * (assign23300_e28428 * (var_t1_dn0 / var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((var_t1).powf(assign23300_e28428 - 1.0) * var_t1_dn2)) } } else { (assign23300_e28429 * (assign23300_e28428 * (var_t1_dn2 / var_t1))) }, if var_rrdrbb__blk413_dn4 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((var_t1).powf(assign23300_e28428 - 1.0) * var_t1_dn4)) } } else { (assign23300_e28429 * ((var_rrdrbb__blk413_dn4 * (var_t1).ln()) + (assign23300_e28428 * (var_t1_dn4 / var_t1)))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((var_t1).powf(assign23300_e28428 - 1.0) * var_t1_dn5)) } } else { (assign23300_e28429 * (assign23300_e28428 * (var_t1_dn5 / var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((var_t1).powf(assign23300_e28428 - 1.0) * var_t1_dn6)) } } else { (assign23300_e28429 * (assign23300_e28428 * (var_t1_dn6 / var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((var_t1).powf(assign23300_e28428 - 1.0) * var_t1_dn8)) } } else { (assign23300_e28429 * (assign23300_e28428 * (var_t1_dn8 / var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((var_t1).powf(assign23300_e28428 - 1.0) * var_t1_dn10)) } } else { (assign23300_e28429 * (assign23300_e28428 * (var_t1_dn10 / var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((var_t1).powf(assign23300_e28428 - 1.0) * var_t1_dn11)) } } else { (assign23300_e28429 * (assign23300_e28428 * (var_t1_dn11 / var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((var_t1).powf(assign23300_e28428 - 1.0) * var_t1_dn12)) } } else { (assign23300_e28429 * (assign23300_e28428 * (var_t1_dn12 / var_t1))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign23300_e28431;
        var_t3_dn0 = assign23300_e28431_d_n0;
        var_t3_dn2 = assign23300_e28431_d_n2;
        var_t3_dn4 = assign23300_e28431_d_n4;
        var_t3_dn5 = assign23300_e28431_d_n5;
        var_t3_dn6 = assign23300_e28431_d_n6;
        var_t3_dn8 = assign23300_e28431_d_n8;
        var_t3_dn10 = assign23300_e28431_d_n10;
        var_t3_dn11 = assign23300_e28431_d_n11;
        var_t3_dn12 = assign23300_e28431_d_n12;

        let (assign23310_e28437, assign23310_e28437_d_n0, assign23310_e28437_d_n2, assign23310_e28437_d_n4, assign23310_e28437_d_n5, assign23310_e28437_d_n6, assign23310_e28437_d_n8, assign23310_e28437_d_n10, assign23310_e28437_d_n11, assign23310_e28437_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23310_e28435: f64 = (var_t1 * var_t3);
        (assign23310_e28435, ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)), ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)), ((var_t1_dn4 * var_t3) + (var_t1 * var_t3_dn4)), ((var_t1_dn5 * var_t3) + (var_t1 * var_t3_dn5)), ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)), ((var_t1_dn8 * var_t3) + (var_t1 * var_t3_dn8)), ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)), ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)), ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign23310_e28437;
        var_t2_dn0 = assign23310_e28437_d_n0;
        var_t2_dn2 = assign23310_e28437_d_n2;
        var_t2_dn4 = assign23310_e28437_d_n4;
        var_t2_dn5 = assign23310_e28437_d_n5;
        var_t2_dn6 = assign23310_e28437_d_n6;
        var_t2_dn8 = assign23310_e28437_d_n8;
        var_t2_dn10 = assign23310_e28437_d_n10;
        var_t2_dn11 = assign23310_e28437_d_n11;
        var_t2_dn12 = assign23310_e28437_d_n12;

        let (assign23320_e28443, assign23320_e28443_d_n0, assign23320_e28443_d_n2, assign23320_e28443_d_n4, assign23320_e28443_d_n5, assign23320_e28443_d_n6, assign23320_e28443_d_n8, assign23320_e28443_d_n10, assign23320_e28443_d_n11, assign23320_e28443_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23320_e28441: f64 = (1.0 + var_t2);
        (assign23320_e28441, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
        var_t4 = assign23320_e28443;
        var_t4_dn0 = assign23320_e28443_d_n0;
        var_t4_dn2 = assign23320_e28443_d_n2;
        var_t4_dn4 = assign23320_e28443_d_n4;
        var_t4_dn5 = assign23320_e28443_d_n5;
        var_t4_dn6 = assign23320_e28443_d_n6;
        var_t4_dn8 = assign23320_e28443_d_n8;
        var_t4_dn10 = assign23320_e28443_d_n10;
        var_t4_dn11 = assign23320_e28443_d_n11;
        var_t4_dn12 = assign23320_e28443_d_n12;

        let assign23330_e28447: f64 = (10.0 * 2.220446049250313e-16);
        let assign23330_e28448: f64 = (1.0 - assign23330_e28447);
        let assign23330_e28455: f64 = (10.0 * 2.220446049250313e-16);
        let assign23330_e28456: f64 = (1.0 + assign23330_e28455);
        let assign23330_e28458: f64 = if ((assign23330_e28448 <= var_rrdrbb__blk413) && (var_rrdrbb__blk413 <= assign23330_e28456)) { 1.0 } else { 0.0 };
        var_guard430 = assign23330_e28458;

        let (assign23340_e28466, assign23340_e28466_d_n0, assign23340_e28466_d_n2, assign23340_e28466_d_n4, assign23340_e28466_d_n5, assign23340_e28466_d_n6, assign23340_e28466_d_n8, assign23340_e28466_d_n10, assign23340_e28466_d_n11, assign23340_e28466_d_n12,) = {
    if ((var_guard407 != 0.0) && (var_guard430 != 0.0)) {
        let assign23340_e28464: f64 = (1.0 / var_t4);
        (assign23340_e28464, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn4 / (var_t4 * var_t4))), (-(var_t4_dn5 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn8 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn11 / (var_t4 * var_t4))), (-(var_t4_dn12 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign23340_e28466;
        var_t5_dn0 = assign23340_e28466_d_n0;
        var_t5_dn2 = assign23340_e28466_d_n2;
        var_t5_dn4 = assign23340_e28466_d_n4;
        var_t5_dn5 = assign23340_e28466_d_n5;
        var_t5_dn6 = assign23340_e28466_d_n6;
        var_t5_dn8 = assign23340_e28466_d_n8;
        var_t5_dn10 = assign23340_e28466_d_n10;
        var_t5_dn11 = assign23340_e28466_d_n11;
        var_t5_dn12 = assign23340_e28466_d_n12;

        let assign23350_e28470: f64 = (10.0 * 2.220446049250313e-16);
        let assign23350_e28471: f64 = (2.0 - assign23350_e28470);
        let assign23350_e28478: f64 = (10.0 * 2.220446049250313e-16);
        let assign23350_e28479: f64 = (2.0 + assign23350_e28478);
        let assign23350_e28481: f64 = if ((assign23350_e28471 <= var_rrdrbb__blk413) && (var_rrdrbb__blk413 <= assign23350_e28479)) { 1.0 } else { 0.0 };
        var_guard431 = assign23350_e28481;

        let (assign23360_e28493, assign23360_e28493_d_n0, assign23360_e28493_d_n2, assign23360_e28493_d_n4, assign23360_e28493_d_n5, assign23360_e28493_d_n6, assign23360_e28493_d_n8, assign23360_e28493_d_n10, assign23360_e28493_d_n11, assign23360_e28493_d_n12,) = {
    if (((var_guard407 != 0.0) && (var_guard430 == 0.0)) && (var_guard431 != 0.0)) {
        let assign23360_e28490: f64 = (var_t4).sqrt();
        let assign23360_e28491: f64 = (1.0 / assign23360_e28490);
        (assign23360_e28491, (-((var_t4_dn0 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((var_t4_dn2 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((var_t4_dn4 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((var_t4_dn5 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((var_t4_dn6 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((var_t4_dn8 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((var_t4_dn10 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((var_t4_dn11 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((var_t4_dn12 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign23360_e28493;
        var_t5_dn0 = assign23360_e28493_d_n0;
        var_t5_dn2 = assign23360_e28493_d_n2;
        var_t5_dn4 = assign23360_e28493_d_n4;
        var_t5_dn5 = assign23360_e28493_d_n5;
        var_t5_dn6 = assign23360_e28493_d_n6;
        var_t5_dn8 = assign23360_e28493_d_n8;
        var_t5_dn10 = assign23360_e28493_d_n10;
        var_t5_dn11 = assign23360_e28493_d_n11;
        var_t5_dn12 = assign23360_e28493_d_n12;

        let (assign23370_e28510, assign23370_e28510_d_n0, assign23370_e28510_d_n2, assign23370_e28510_d_n4, assign23370_e28510_d_n5, assign23370_e28510_d_n6, assign23370_e28510_d_n8, assign23370_e28510_d_n10, assign23370_e28510_d_n11, assign23370_e28510_d_n12,) = {
    if (((var_guard407 != 0.0) && (var_guard430 == 0.0)) && (var_guard431 == 0.0)) {
        let assign23370_e28503: f64 = (-1.0);
        let assign23370_e28505: f64 = (assign23370_e28503 / var_rrdrbb__blk413);
        let assign23370_e28507: f64 = (assign23370_e28505 - 1.0);
        let assign23370_e28508: f64 = (var_t4).powf(assign23370_e28507);
        (assign23370_e28508, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((var_t4).powf(assign23370_e28507 - 1.0) * var_t4_dn0)) } } else { (assign23370_e28508 * (assign23370_e28507 * (var_t4_dn0 / var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((var_t4).powf(assign23370_e28507 - 1.0) * var_t4_dn2)) } } else { (assign23370_e28508 * (assign23370_e28507 * (var_t4_dn2 / var_t4))) }, if (-((assign23370_e28503 * var_rrdrbb__blk413_dn4) / (var_rrdrbb__blk413 * var_rrdrbb__blk413))) == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((var_t4).powf(assign23370_e28507 - 1.0) * var_t4_dn4)) } } else { (assign23370_e28508 * (((-((assign23370_e28503 * var_rrdrbb__blk413_dn4) / (var_rrdrbb__blk413 * var_rrdrbb__blk413))) * (var_t4).ln()) + (assign23370_e28507 * (var_t4_dn4 / var_t4)))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((var_t4).powf(assign23370_e28507 - 1.0) * var_t4_dn5)) } } else { (assign23370_e28508 * (assign23370_e28507 * (var_t4_dn5 / var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((var_t4).powf(assign23370_e28507 - 1.0) * var_t4_dn6)) } } else { (assign23370_e28508 * (assign23370_e28507 * (var_t4_dn6 / var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((var_t4).powf(assign23370_e28507 - 1.0) * var_t4_dn8)) } } else { (assign23370_e28508 * (assign23370_e28507 * (var_t4_dn8 / var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((var_t4).powf(assign23370_e28507 - 1.0) * var_t4_dn10)) } } else { (assign23370_e28508 * (assign23370_e28507 * (var_t4_dn10 / var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((var_t4).powf(assign23370_e28507 - 1.0) * var_t4_dn11)) } } else { (assign23370_e28508 * (assign23370_e28507 * (var_t4_dn11 / var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((var_t4).powf(assign23370_e28507 - 1.0) * var_t4_dn12)) } } else { (assign23370_e28508 * (assign23370_e28507 * (var_t4_dn12 / var_t4))) },)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
        var_t6 = assign23370_e28510;
        var_t6_dn0 = assign23370_e28510_d_n0;
        var_t6_dn2 = assign23370_e28510_d_n2;
        var_t6_dn4 = assign23370_e28510_d_n4;
        var_t6_dn5 = assign23370_e28510_d_n5;
        var_t6_dn6 = assign23370_e28510_d_n6;
        var_t6_dn8 = assign23370_e28510_d_n8;
        var_t6_dn10 = assign23370_e28510_d_n10;
        var_t6_dn11 = assign23370_e28510_d_n11;
        var_t6_dn12 = assign23370_e28510_d_n12;

        let (assign23380_e28522, assign23380_e28522_d_n0, assign23380_e28522_d_n2, assign23380_e28522_d_n4, assign23380_e28522_d_n5, assign23380_e28522_d_n6, assign23380_e28522_d_n8, assign23380_e28522_d_n10, assign23380_e28522_d_n11, assign23380_e28522_d_n12,) = {
    if (((var_guard407 != 0.0) && (var_guard430 == 0.0)) && (var_guard431 == 0.0)) {
        let assign23380_e28520: f64 = (var_t4 * var_t6);
        (assign23380_e28520, ((var_t4_dn0 * var_t6) + (var_t4 * var_t6_dn0)), ((var_t4_dn2 * var_t6) + (var_t4 * var_t6_dn2)), ((var_t4_dn4 * var_t6) + (var_t4 * var_t6_dn4)), ((var_t4_dn5 * var_t6) + (var_t4 * var_t6_dn5)), ((var_t4_dn6 * var_t6) + (var_t4 * var_t6_dn6)), ((var_t4_dn8 * var_t6) + (var_t4 * var_t6_dn8)), ((var_t4_dn10 * var_t6) + (var_t4 * var_t6_dn10)), ((var_t4_dn11 * var_t6) + (var_t4 * var_t6_dn11)), ((var_t4_dn12 * var_t6) + (var_t4 * var_t6_dn12)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign23380_e28522;
        var_t5_dn0 = assign23380_e28522_d_n0;
        var_t5_dn2 = assign23380_e28522_d_n2;
        var_t5_dn4 = assign23380_e28522_d_n4;
        var_t5_dn5 = assign23380_e28522_d_n5;
        var_t5_dn6 = assign23380_e28522_d_n6;
        var_t5_dn8 = assign23380_e28522_d_n8;
        var_t5_dn10 = assign23380_e28522_d_n10;
        var_t5_dn11 = assign23380_e28522_d_n11;
        var_t5_dn12 = assign23380_e28522_d_n12;

        let (assign23390_e28528, assign23390_e28528_d_n0, assign23390_e28528_d_n2, assign23390_e28528_d_n4, assign23390_e28528_d_n5, assign23390_e28528_d_n6, assign23390_e28528_d_n8, assign23390_e28528_d_n10, assign23390_e28528_d_n11, assign23390_e28528_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23390_e28526: f64 = (var_mu0__blk419 * var_t5);
        (assign23390_e28526, ((var_mu0__blk419_dn0 * var_t5) + (var_mu0__blk419 * var_t5_dn0)), ((var_mu0__blk419_dn2 * var_t5) + (var_mu0__blk419 * var_t5_dn2)), ((var_mu0__blk419_dn4 * var_t5) + (var_mu0__blk419 * var_t5_dn4)), ((var_mu0__blk419_dn5 * var_t5) + (var_mu0__blk419 * var_t5_dn5)), ((var_mu0__blk419_dn6 * var_t5) + (var_mu0__blk419 * var_t5_dn6)), ((var_mu0__blk419_dn8 * var_t5) + (var_mu0__blk419 * var_t5_dn8)), ((var_mu0__blk419_dn10 * var_t5) + (var_mu0__blk419 * var_t5_dn10)), ((var_mu0__blk419_dn11 * var_t5) + (var_mu0__blk419 * var_t5_dn11)), ((var_mu0__blk419_dn12 * var_t5) + (var_mu0__blk419 * var_t5_dn12)),)
    } else {
        (var_mu__blk423, var_mu__blk423_dn0, var_mu__blk423_dn2, var_mu__blk423_dn4, var_mu__blk423_dn5, var_mu__blk423_dn6, var_mu__blk423_dn8, var_mu__blk423_dn10, var_mu__blk423_dn11, var_mu__blk423_dn12,)
    }
};
        var_mu__blk423 = assign23390_e28528;
        var_mu__blk423_dn0 = assign23390_e28528_d_n0;
        var_mu__blk423_dn2 = assign23390_e28528_d_n2;
        var_mu__blk423_dn4 = assign23390_e28528_d_n4;
        var_mu__blk423_dn5 = assign23390_e28528_d_n5;
        var_mu__blk423_dn6 = assign23390_e28528_d_n6;
        var_mu__blk423_dn8 = assign23390_e28528_d_n8;
        var_mu__blk423_dn10 = assign23390_e28528_d_n10;
        var_mu__blk423_dn11 = assign23390_e28528_d_n11;
        var_mu__blk423_dn12 = assign23390_e28528_d_n12;

        let (assign23400_e28534, assign23400_e28534_d_n0, assign23400_e28534_d_n2, assign23400_e28534_d_n4, assign23400_e28534_d_n5, assign23400_e28534_d_n6, assign23400_e28534_d_n8, assign23400_e28534_d_n10, assign23400_e28534_d_n11, assign23400_e28534_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23400_e28532: f64 = (1.6021918e-19 / var_ldrifte__blk417);
        (assign23400_e28532, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign23400_e28534;
        var_t1_dn0 = assign23400_e28534_d_n0;
        var_t1_dn2 = assign23400_e28534_d_n2;
        var_t1_dn4 = assign23400_e28534_d_n4;
        var_t1_dn5 = assign23400_e28534_d_n5;
        var_t1_dn6 = assign23400_e28534_d_n6;
        var_t1_dn8 = assign23400_e28534_d_n8;
        var_t1_dn10 = assign23400_e28534_d_n10;
        var_t1_dn11 = assign23400_e28534_d_n11;
        var_t1_dn12 = assign23400_e28534_d_n12;

        let (assign23410_e28544, assign23410_e28544_d_n0, assign23410_e28544_d_n2, assign23410_e28544_d_n4, assign23410_e28544_d_n5, assign23410_e28544_d_n6, assign23410_e28544_d_n8, assign23410_e28544_d_n10, assign23410_e28544_d_n11, assign23410_e28544_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23410_e28538: f64 = (var_t1 * var_xov__blk424);
        let assign23410_e28540: f64 = (assign23410_e28538 * var_mu__blk423);
        let assign23410_e28542: f64 = (assign23410_e28540 * var_nover__blk418);
        (assign23410_e28542, ((((var_t1_dn0 * var_xov__blk424) * var_mu__blk423) + (assign23410_e28538 * var_mu__blk423_dn0)) * var_nover__blk418), ((((var_t1_dn2 * var_xov__blk424) * var_mu__blk423) + (assign23410_e28538 * var_mu__blk423_dn2)) * var_nover__blk418), ((((var_t1_dn4 * var_xov__blk424) * var_mu__blk423) + (assign23410_e28538 * var_mu__blk423_dn4)) * var_nover__blk418), ((((var_t1_dn5 * var_xov__blk424) * var_mu__blk423) + (assign23410_e28538 * var_mu__blk423_dn5)) * var_nover__blk418), ((((var_t1_dn6 * var_xov__blk424) * var_mu__blk423) + (assign23410_e28538 * var_mu__blk423_dn6)) * var_nover__blk418), ((((var_t1_dn8 * var_xov__blk424) * var_mu__blk423) + (assign23410_e28538 * var_mu__blk423_dn8)) * var_nover__blk418), ((((var_t1_dn10 * var_xov__blk424) * var_mu__blk423) + (assign23410_e28538 * var_mu__blk423_dn10)) * var_nover__blk418), ((((var_t1_dn11 * var_xov__blk424) * var_mu__blk423) + (assign23410_e28538 * var_mu__blk423_dn11)) * var_nover__blk418), ((((var_t1_dn12 * var_xov__blk424) * var_mu__blk423) + (assign23410_e28538 * var_mu__blk423_dn12)) * var_nover__blk418),)
    } else {
        (var_gd__blk425, var_gd__blk425_dn0, var_gd__blk425_dn2, var_gd__blk425_dn4, var_gd__blk425_dn5, var_gd__blk425_dn6, var_gd__blk425_dn8, var_gd__blk425_dn10, var_gd__blk425_dn11, var_gd__blk425_dn12,)
    }
};
        var_gd__blk425 = assign23410_e28544;
        var_gd__blk425_dn0 = assign23410_e28544_d_n0;
        var_gd__blk425_dn2 = assign23410_e28544_d_n2;
        var_gd__blk425_dn4 = assign23410_e28544_d_n4;
        var_gd__blk425_dn5 = assign23410_e28544_d_n5;
        var_gd__blk425_dn6 = assign23410_e28544_d_n6;
        var_gd__blk425_dn8 = assign23410_e28544_d_n8;
        var_gd__blk425_dn10 = assign23410_e28544_d_n10;
        var_gd__blk425_dn11 = assign23410_e28544_d_n11;
        var_gd__blk425_dn12 = assign23410_e28544_d_n12;

        let assign23420_e28547: f64 = if var_gd__blk425 <= 0.0 { 1.0 } else { 0.0 };
        var_guard432 = assign23420_e28547;

        let (assign23430_e28553, assign23430_e28553_d_n0, assign23430_e28553_d_n2, assign23430_e28553_d_n4, assign23430_e28553_d_n5, assign23430_e28553_d_n6, assign23430_e28553_d_n8, assign23430_e28553_d_n10, assign23430_e28553_d_n11, assign23430_e28553_d_n12,) = {
    if ((var_guard407 != 0.0) && (var_guard432 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gd__blk425, var_gd__blk425_dn0, var_gd__blk425_dn2, var_gd__blk425_dn4, var_gd__blk425_dn5, var_gd__blk425_dn6, var_gd__blk425_dn8, var_gd__blk425_dn10, var_gd__blk425_dn11, var_gd__blk425_dn12,)
    }
};
        var_gd__blk425 = assign23430_e28553;
        var_gd__blk425_dn0 = assign23430_e28553_d_n0;
        var_gd__blk425_dn2 = assign23430_e28553_d_n2;
        var_gd__blk425_dn4 = assign23430_e28553_d_n4;
        var_gd__blk425_dn5 = assign23430_e28553_d_n5;
        var_gd__blk425_dn6 = assign23430_e28553_d_n6;
        var_gd__blk425_dn8 = assign23430_e28553_d_n8;
        var_gd__blk425_dn10 = assign23430_e28553_d_n10;
        var_gd__blk425_dn11 = assign23430_e28553_d_n11;
        var_gd__blk425_dn12 = assign23430_e28553_d_n12;

        let (assign23440_e28559, assign23440_e28559_d_n0, assign23440_e28559_d_n2, assign23440_e28559_d_n4, assign23440_e28559_d_n5, assign23440_e28559_d_n6, assign23440_e28559_d_n8, assign23440_e28559_d_n10, assign23440_e28559_d_n11, assign23440_e28559_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23440_e28557: f64 = (1.0 / var_gd__blk425);
        (assign23440_e28557, (-(var_gd__blk425_dn0 / (var_gd__blk425 * var_gd__blk425))), (-(var_gd__blk425_dn2 / (var_gd__blk425 * var_gd__blk425))), (-(var_gd__blk425_dn4 / (var_gd__blk425 * var_gd__blk425))), (-(var_gd__blk425_dn5 / (var_gd__blk425 * var_gd__blk425))), (-(var_gd__blk425_dn6 / (var_gd__blk425 * var_gd__blk425))), (-(var_gd__blk425_dn8 / (var_gd__blk425 * var_gd__blk425))), (-(var_gd__blk425_dn10 / (var_gd__blk425 * var_gd__blk425))), (-(var_gd__blk425_dn11 / (var_gd__blk425 * var_gd__blk425))), (-(var_gd__blk425_dn12 / (var_gd__blk425 * var_gd__blk425))),)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn4, var_rsd_dn5, var_rsd_dn6, var_rsd_dn8, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12,)
    }
};
        var_rsd = assign23440_e28559;
        var_rsd_dn0 = assign23440_e28559_d_n0;
        var_rsd_dn2 = assign23440_e28559_d_n2;
        var_rsd_dn4 = assign23440_e28559_d_n4;
        var_rsd_dn5 = assign23440_e28559_d_n5;
        var_rsd_dn6 = assign23440_e28559_d_n6;
        var_rsd_dn8 = assign23440_e28559_d_n8;
        var_rsd_dn10 = assign23440_e28559_d_n10;
        var_rsd_dn11 = assign23440_e28559_d_n11;
        var_rsd_dn12 = assign23440_e28559_d_n12;

        let (assign23450_e28565, assign23450_e28565_d_n0, assign23450_e28565_d_n2, assign23450_e28565_d_n4, assign23450_e28565_d_n5, assign23450_e28565_d_n6, assign23450_e28565_d_n8, assign23450_e28565_d_n10, assign23450_e28565_d_n11, assign23450_e28565_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23450_e28563: f64 = (var_rsd / var_weff_nf__blk426);
        (assign23450_e28563, (((var_rsd_dn0 * var_weff_nf__blk426) - (var_rsd * var_weff_nf__blk426_dn0)) / (var_weff_nf__blk426 * var_weff_nf__blk426)), (((var_rsd_dn2 * var_weff_nf__blk426) - (var_rsd * var_weff_nf__blk426_dn2)) / (var_weff_nf__blk426 * var_weff_nf__blk426)), (((var_rsd_dn4 * var_weff_nf__blk426) - (var_rsd * var_weff_nf__blk426_dn4)) / (var_weff_nf__blk426 * var_weff_nf__blk426)), (((var_rsd_dn5 * var_weff_nf__blk426) - (var_rsd * var_weff_nf__blk426_dn5)) / (var_weff_nf__blk426 * var_weff_nf__blk426)), (((var_rsd_dn6 * var_weff_nf__blk426) - (var_rsd * var_weff_nf__blk426_dn6)) / (var_weff_nf__blk426 * var_weff_nf__blk426)), (((var_rsd_dn8 * var_weff_nf__blk426) - (var_rsd * var_weff_nf__blk426_dn8)) / (var_weff_nf__blk426 * var_weff_nf__blk426)), (((var_rsd_dn10 * var_weff_nf__blk426) - (var_rsd * var_weff_nf__blk426_dn10)) / (var_weff_nf__blk426 * var_weff_nf__blk426)), (((var_rsd_dn11 * var_weff_nf__blk426) - (var_rsd * var_weff_nf__blk426_dn11)) / (var_weff_nf__blk426 * var_weff_nf__blk426)), (((var_rsd_dn12 * var_weff_nf__blk426) - (var_rsd * var_weff_nf__blk426_dn12)) / (var_weff_nf__blk426 * var_weff_nf__blk426)),)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn4, var_rsd_dn5, var_rsd_dn6, var_rsd_dn8, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12,)
    }
};
        var_rsd = assign23450_e28565;
        var_rsd_dn0 = assign23450_e28565_d_n0;
        var_rsd_dn2 = assign23450_e28565_d_n2;
        var_rsd_dn4 = assign23450_e28565_d_n4;
        var_rsd_dn5 = assign23450_e28565_d_n5;
        var_rsd_dn6 = assign23450_e28565_d_n6;
        var_rsd_dn8 = assign23450_e28565_d_n8;
        var_rsd_dn10 = assign23450_e28565_d_n10;
        var_rsd_dn11 = assign23450_e28565_d_n11;
        var_rsd_dn12 = assign23450_e28565_d_n12;

        let (assign23460_e28571, assign23460_e28571_d_n0, assign23460_e28571_d_n2, assign23460_e28571_d_n4, assign23460_e28571_d_n5, assign23460_e28571_d_n6, assign23460_e28571_d_n8, assign23460_e28571_d_n10, assign23460_e28571_d_n11, assign23460_e28571_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23460_e28569: f64 = (var_rsd + var_rsd0__blk414);
        (assign23460_e28569, var_rsd_dn0, var_rsd_dn2, var_rsd_dn4, var_rsd_dn5, var_rsd_dn6, var_rsd_dn8, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn4, var_rsd_dn5, var_rsd_dn6, var_rsd_dn8, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12,)
    }
};
        var_rsd = assign23460_e28571;
        var_rsd_dn0 = assign23460_e28571_d_n0;
        var_rsd_dn2 = assign23460_e28571_d_n2;
        var_rsd_dn4 = assign23460_e28571_d_n4;
        var_rsd_dn5 = assign23460_e28571_d_n5;
        var_rsd_dn6 = assign23460_e28571_d_n6;
        var_rsd_dn8 = assign23460_e28571_d_n8;
        var_rsd_dn10 = assign23460_e28571_d_n10;
        var_rsd_dn11 = assign23460_e28571_d_n11;
        var_rsd_dn12 = assign23460_e28571_d_n12;

        let assign23480_e28589: f64 = if var_rsd < 0.0001 { 1.0 } else { 0.0 };
        var_guard433 = assign23480_e28589;

        let (assign23490_e28595, assign23490_e28595_d_n0, assign23490_e28595_d_n2, assign23490_e28595_d_n4, assign23490_e28595_d_n5, assign23490_e28595_d_n6, assign23490_e28595_d_n8, assign23490_e28595_d_n10, assign23490_e28595_d_n11, assign23490_e28595_d_n12,) = {
    if ((var_guard407 != 0.0) && (var_guard433 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn4, var_rsd_dn5, var_rsd_dn6, var_rsd_dn8, var_rsd_dn10, var_rsd_dn11, var_rsd_dn12,)
    }
};
        var_rsd = assign23490_e28595;
        var_rsd_dn0 = assign23490_e28595_d_n0;
        var_rsd_dn2 = assign23490_e28595_d_n2;
        var_rsd_dn4 = assign23490_e28595_d_n4;
        var_rsd_dn5 = assign23490_e28595_d_n5;
        var_rsd_dn6 = assign23490_e28595_d_n6;
        var_rsd_dn8 = assign23490_e28595_d_n8;
        var_rsd_dn10 = assign23490_e28595_d_n10;
        var_rsd_dn11 = assign23490_e28595_d_n11;
        var_rsd_dn12 = assign23490_e28595_d_n12;

        let (assign23500_e28601, assign23500_e28601_d_n0, assign23500_e28601_d_n2, assign23500_e28601_d_n4, assign23500_e28601_d_n5, assign23500_e28601_d_n6, assign23500_e28601_d_n8, assign23500_e28601_d_n10, assign23500_e28601_d_n11, assign23500_e28601_d_n12,) = {
    if (var_guard407 != 0.0) {
        let assign23500_e28599: f64 = (var_rsd / var_mfactor);
        (assign23500_e28599, (var_rsd_dn0 / var_mfactor), (var_rsd_dn2 / var_mfactor), (var_rsd_dn4 / var_mfactor), (var_rsd_dn5 / var_mfactor), (var_rsd_dn6 / var_mfactor), (var_rsd_dn8 / var_mfactor), (var_rsd_dn10 / var_mfactor), (var_rsd_dn11 / var_mfactor), (var_rsd_dn12 / var_mfactor),)
    } else {
        (var_rdde, var_rdde_dn0, var_rdde_dn2, var_rdde_dn4, var_rdde_dn5, var_rdde_dn6, var_rdde_dn8, var_rdde_dn10, var_rdde_dn11, var_rdde_dn12,)
    }
};
        var_rdde = assign23500_e28601;
        var_rdde_dn0 = assign23500_e28601_d_n0;
        var_rdde_dn2 = assign23500_e28601_d_n2;
        var_rdde_dn4 = assign23500_e28601_d_n4;
        var_rdde_dn5 = assign23500_e28601_d_n5;
        var_rdde_dn6 = assign23500_e28601_d_n6;
        var_rdde_dn8 = assign23500_e28601_d_n8;
        var_rdde_dn10 = assign23500_e28601_d_n10;
        var_rdde_dn11 = assign23500_e28601_d_n11;
        var_rdde_dn12 = assign23500_e28601_d_n12;

        let assign23520_e28608: f64 = if var_tau < 1e-18 { 1.0 } else { 0.0 };
        var_guard434 = assign23520_e28608;

        let (assign23530_e28614, assign23530_e28614_d_n0, assign23530_e28614_d_n2, assign23530_e28614_d_n4, assign23530_e28614_d_n5, assign23530_e28614_d_n6, assign23530_e28614_d_n8, assign23530_e28614_d_n10, assign23530_e28614_d_n11, assign23530_e28614_d_n12,) = {
    if ((var_flg_nqs != 0.0) && (var_guard434 != 0.0)) {
        (1e-18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn4, var_tau_dn5, var_tau_dn6, var_tau_dn8, var_tau_dn10, var_tau_dn11, var_tau_dn12,)
    }
};
        var_tau = assign23530_e28614;
        var_tau_dn0 = assign23530_e28614_d_n0;
        var_tau_dn2 = assign23530_e28614_d_n2;
        var_tau_dn4 = assign23530_e28614_d_n4;
        var_tau_dn5 = assign23530_e28614_d_n5;
        var_tau_dn6 = assign23530_e28614_d_n6;
        var_tau_dn8 = assign23530_e28614_d_n8;
        var_tau_dn10 = assign23530_e28614_d_n10;
        var_tau_dn11 = assign23530_e28614_d_n11;
        var_tau_dn12 = assign23530_e28614_d_n12;

        let assign23540_e28617: f64 = if var_taub < 1e-18 { 1.0 } else { 0.0 };
        var_guard435 = assign23540_e28617;

        let (assign23550_e28623, assign23550_e28623_d_n0, assign23550_e28623_d_n2, assign23550_e28623_d_n4, assign23550_e28623_d_n5, assign23550_e28623_d_n6, assign23550_e28623_d_n8, assign23550_e28623_d_n10, assign23550_e28623_d_n11, assign23550_e28623_d_n12,) = {
    if ((var_flg_nqs != 0.0) && (var_guard435 != 0.0)) {
        (1e-18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taub, var_taub_dn0, var_taub_dn2, var_taub_dn4, var_taub_dn5, var_taub_dn6, var_taub_dn8, var_taub_dn10, var_taub_dn11, var_taub_dn12,)
    }
};
        var_taub = assign23550_e28623;
        var_taub_dn0 = assign23550_e28623_d_n0;
        var_taub_dn2 = assign23550_e28623_d_n2;
        var_taub_dn4 = assign23550_e28623_d_n4;
        var_taub_dn5 = assign23550_e28623_d_n5;
        var_taub_dn6 = assign23550_e28623_d_n6;
        var_taub_dn8 = assign23550_e28623_d_n8;
        var_taub_dn10 = assign23550_e28623_d_n10;
        var_taub_dn11 = assign23550_e28623_d_n11;
        var_taub_dn12 = assign23550_e28623_d_n12;

        let (assign23560_e28631, assign23560_e28631_d_n0, assign23560_e28631_d_n2, assign23560_e28631_d_n4, assign23560_e28631_d_n5, assign23560_e28631_d_n6, assign23560_e28631_d_n8, assign23560_e28631_d_n10, assign23560_e28631_d_n11, assign23560_e28631_d_n12,) = {
    if (var_flg_nqs != 0.0) {
        let assign23560_e28627: f64 = (var_qi_nqs - var_qi_qs);
        let assign23560_e28629: f64 = (assign23560_e28627 / var_tau);
        (assign23560_e28629, ((((-var_qi_qs_dn0) * var_tau) - (assign23560_e28627 * var_tau_dn0)) / (var_tau * var_tau)), ((((-var_qi_qs_dn2) * var_tau) - (assign23560_e28627 * var_tau_dn2)) / (var_tau * var_tau)), ((((-var_qi_qs_dn4) * var_tau) - (assign23560_e28627 * var_tau_dn4)) / (var_tau * var_tau)), ((((-var_qi_qs_dn5) * var_tau) - (assign23560_e28627 * var_tau_dn5)) / (var_tau * var_tau)), ((((-var_qi_qs_dn6) * var_tau) - (assign23560_e28627 * var_tau_dn6)) / (var_tau * var_tau)), ((((var_qi_nqs_dn8 - var_qi_qs_dn8) * var_tau) - (assign23560_e28627 * var_tau_dn8)) / (var_tau * var_tau)), ((((-var_qi_qs_dn10) * var_tau) - (assign23560_e28627 * var_tau_dn10)) / (var_tau * var_tau)), ((((-var_qi_qs_dn11) * var_tau) - (assign23560_e28627 * var_tau_dn11)) / (var_tau * var_tau)), ((((-var_qi_qs_dn12) * var_tau) - (assign23560_e28627 * var_tau_dn12)) / (var_tau * var_tau)),)
    } else {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn4, var_iqi_nqs_dn5, var_iqi_nqs_dn6, var_iqi_nqs_dn8, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12,)
    }
};
        var_iqi_nqs = assign23560_e28631;
        var_iqi_nqs_dn0 = assign23560_e28631_d_n0;
        var_iqi_nqs_dn2 = assign23560_e28631_d_n2;
        var_iqi_nqs_dn4 = assign23560_e28631_d_n4;
        var_iqi_nqs_dn5 = assign23560_e28631_d_n5;
        var_iqi_nqs_dn6 = assign23560_e28631_d_n6;
        var_iqi_nqs_dn8 = assign23560_e28631_d_n8;
        var_iqi_nqs_dn10 = assign23560_e28631_d_n10;
        var_iqi_nqs_dn11 = assign23560_e28631_d_n11;
        var_iqi_nqs_dn12 = assign23560_e28631_d_n12;

        let (assign23570_e28639, assign23570_e28639_d_n0, assign23570_e28639_d_n2, assign23570_e28639_d_n4, assign23570_e28639_d_n5, assign23570_e28639_d_n6, assign23570_e28639_d_n8, assign23570_e28639_d_n9, assign23570_e28639_d_n10, assign23570_e28639_d_n11, assign23570_e28639_d_n12,) = {
    if (var_flg_nqs != 0.0) {
        let assign23570_e28635: f64 = (var_qb_nqs - var_qb_qs);
        let assign23570_e28637: f64 = (assign23570_e28635 / var_taub);
        (assign23570_e28637, ((((-var_qb_qs_dn0) * var_taub) - (assign23570_e28635 * var_taub_dn0)) / (var_taub * var_taub)), ((((-var_qb_qs_dn2) * var_taub) - (assign23570_e28635 * var_taub_dn2)) / (var_taub * var_taub)), ((((-var_qb_qs_dn4) * var_taub) - (assign23570_e28635 * var_taub_dn4)) / (var_taub * var_taub)), ((((-var_qb_qs_dn5) * var_taub) - (assign23570_e28635 * var_taub_dn5)) / (var_taub * var_taub)), ((((-var_qb_qs_dn6) * var_taub) - (assign23570_e28635 * var_taub_dn6)) / (var_taub * var_taub)), ((((-var_qb_qs_dn8) * var_taub) - (assign23570_e28635 * var_taub_dn8)) / (var_taub * var_taub)), (var_qb_nqs_dn9 / var_taub), ((((-var_qb_qs_dn10) * var_taub) - (assign23570_e28635 * var_taub_dn10)) / (var_taub * var_taub)), ((((-var_qb_qs_dn11) * var_taub) - (assign23570_e28635 * var_taub_dn11)) / (var_taub * var_taub)), ((((-var_qb_qs_dn12) * var_taub) - (assign23570_e28635 * var_taub_dn12)) / (var_taub * var_taub)),)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn4, var_iqb_nqs_dn5, var_iqb_nqs_dn6, var_iqb_nqs_dn8, var_iqb_nqs_dn9, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12,)
    }
};
        var_iqb_nqs = assign23570_e28639;
        var_iqb_nqs_dn0 = assign23570_e28639_d_n0;
        var_iqb_nqs_dn2 = assign23570_e28639_d_n2;
        var_iqb_nqs_dn4 = assign23570_e28639_d_n4;
        var_iqb_nqs_dn5 = assign23570_e28639_d_n5;
        var_iqb_nqs_dn6 = assign23570_e28639_d_n6;
        var_iqb_nqs_dn8 = assign23570_e28639_d_n8;
        var_iqb_nqs_dn9 = assign23570_e28639_d_n9;
        var_iqb_nqs_dn10 = assign23570_e28639_d_n10;
        var_iqb_nqs_dn11 = assign23570_e28639_d_n11;
        var_iqb_nqs_dn12 = assign23570_e28639_d_n12;

        let (assign23580_e28646, assign23580_e28646_d_n8, assign23580_e28646_d_n9,) = {
    if (var_flg_nqs != 0.0) {
        let assign23580_e28642: f64 = (-var_qi_nqs);
        let assign23580_e28644: f64 = (assign23580_e28642 - var_qb_nqs);
        (assign23580_e28644, (-var_qi_nqs_dn8), (-var_qb_nqs_dn9),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn8, var_qg_nqs_dn9,)
    }
};
        var_qg_nqs = assign23580_e28646;
        var_qg_nqs_dn8 = assign23580_e28646_d_n8;
        var_qg_nqs_dn9 = assign23580_e28646_d_n9;

        let (assign23590_e28652, assign23590_e28652_d_n0, assign23590_e28652_d_n2, assign23590_e28652_d_n4, assign23590_e28652_d_n5, assign23590_e28652_d_n6, assign23590_e28652_d_n8, assign23590_e28652_d_n10, assign23590_e28652_d_n11, assign23590_e28652_d_n12,) = {
    if (var_flg_nqs != 0.0) {
        let assign23590_e28650: f64 = (var_qi_nqs * var_qdrat);
        (assign23590_e28650, 0.0, 0.0, 0.0, 0.0, 0.0, (var_qi_nqs_dn8 * var_qdrat), 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn4, var_qd_nqs_dn5, var_qd_nqs_dn6, var_qd_nqs_dn8, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12,)
    }
};
        var_qd_nqs = assign23590_e28652;
        var_qd_nqs_dn0 = assign23590_e28652_d_n0;
        var_qd_nqs_dn2 = assign23590_e28652_d_n2;
        var_qd_nqs_dn4 = assign23590_e28652_d_n4;
        var_qd_nqs_dn5 = assign23590_e28652_d_n5;
        var_qd_nqs_dn6 = assign23590_e28652_d_n6;
        var_qd_nqs_dn8 = assign23590_e28652_d_n8;
        var_qd_nqs_dn10 = assign23590_e28652_d_n10;
        var_qd_nqs_dn11 = assign23590_e28652_d_n11;
        var_qd_nqs_dn12 = assign23590_e28652_d_n12;

        let (assign23600_e28660, assign23600_e28660_d_n0, assign23600_e28660_d_n2, assign23600_e28660_d_n4, assign23600_e28660_d_n5, assign23600_e28660_d_n6, assign23600_e28660_d_n8, assign23600_e28660_d_n10, assign23600_e28660_d_n11, assign23600_e28660_d_n12,) = {
    if (var_flg_nqs != 0.0) {
        let assign23600_e28657: f64 = (1.0 - var_qdrat);
        let assign23600_e28658: f64 = (var_qi_nqs * assign23600_e28657);
        (assign23600_e28658, 0.0, 0.0, 0.0, 0.0, 0.0, (var_qi_nqs_dn8 * assign23600_e28657), 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn4, var_qs_nqs_dn5, var_qs_nqs_dn6, var_qs_nqs_dn8, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12,)
    }
};
        var_qs_nqs = assign23600_e28660;
        var_qs_nqs_dn0 = assign23600_e28660_d_n0;
        var_qs_nqs_dn2 = assign23600_e28660_d_n2;
        var_qs_nqs_dn4 = assign23600_e28660_d_n4;
        var_qs_nqs_dn5 = assign23600_e28660_d_n5;
        var_qs_nqs_dn6 = assign23600_e28660_d_n6;
        var_qs_nqs_dn8 = assign23600_e28660_d_n8;
        var_qs_nqs_dn10 = assign23600_e28660_d_n10;
        var_qs_nqs_dn11 = assign23600_e28660_d_n11;
        var_qs_nqs_dn12 = assign23600_e28660_d_n12;

        let (assign23610_e28665, assign23610_e28665_d_n0, assign23610_e28665_d_n2, assign23610_e28665_d_n4, assign23610_e28665_d_n5, assign23610_e28665_d_n6, assign23610_e28665_d_n8, assign23610_e28665_d_n10, assign23610_e28665_d_n11, assign23610_e28665_d_n12,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn4, var_iqi_nqs_dn5, var_iqi_nqs_dn6, var_iqi_nqs_dn8, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12,)
    }
};
        var_iqi_nqs = assign23610_e28665;
        var_iqi_nqs_dn0 = assign23610_e28665_d_n0;
        var_iqi_nqs_dn2 = assign23610_e28665_d_n2;
        var_iqi_nqs_dn4 = assign23610_e28665_d_n4;
        var_iqi_nqs_dn5 = assign23610_e28665_d_n5;
        var_iqi_nqs_dn6 = assign23610_e28665_d_n6;
        var_iqi_nqs_dn8 = assign23610_e28665_d_n8;
        var_iqi_nqs_dn10 = assign23610_e28665_d_n10;
        var_iqi_nqs_dn11 = assign23610_e28665_d_n11;
        var_iqi_nqs_dn12 = assign23610_e28665_d_n12;

        *var_gd__blk425_slot = var_gd__blk425;
        *var_gd__blk425_dn0_slot = var_gd__blk425_dn0;
        *var_gd__blk425_dn10_slot = var_gd__blk425_dn10;
        *var_gd__blk425_dn11_slot = var_gd__blk425_dn11;
        *var_gd__blk425_dn12_slot = var_gd__blk425_dn12;
        *var_gd__blk425_dn2_slot = var_gd__blk425_dn2;
        *var_gd__blk425_dn4_slot = var_gd__blk425_dn4;
        *var_gd__blk425_dn5_slot = var_gd__blk425_dn5;
        *var_gd__blk425_dn6_slot = var_gd__blk425_dn6;
        *var_gd__blk425_dn8_slot = var_gd__blk425_dn8;
        *var_guard429_slot = var_guard429;
        *var_guard430_slot = var_guard430;
        *var_guard431_slot = var_guard431;
        *var_guard432_slot = var_guard432;
        *var_guard433_slot = var_guard433;
        *var_guard434_slot = var_guard434;
        *var_guard435_slot = var_guard435;
        *var_iqb_nqs_slot = var_iqb_nqs;
        *var_iqb_nqs_dn0_slot = var_iqb_nqs_dn0;
        *var_iqb_nqs_dn10_slot = var_iqb_nqs_dn10;
        *var_iqb_nqs_dn11_slot = var_iqb_nqs_dn11;
        *var_iqb_nqs_dn12_slot = var_iqb_nqs_dn12;
        *var_iqb_nqs_dn2_slot = var_iqb_nqs_dn2;
        *var_iqb_nqs_dn4_slot = var_iqb_nqs_dn4;
        *var_iqb_nqs_dn5_slot = var_iqb_nqs_dn5;
        *var_iqb_nqs_dn6_slot = var_iqb_nqs_dn6;
        *var_iqb_nqs_dn8_slot = var_iqb_nqs_dn8;
        *var_iqb_nqs_dn9_slot = var_iqb_nqs_dn9;
        *var_iqi_nqs_slot = var_iqi_nqs;
        *var_iqi_nqs_dn0_slot = var_iqi_nqs_dn0;
        *var_iqi_nqs_dn10_slot = var_iqi_nqs_dn10;
        *var_iqi_nqs_dn11_slot = var_iqi_nqs_dn11;
        *var_iqi_nqs_dn12_slot = var_iqi_nqs_dn12;
        *var_iqi_nqs_dn2_slot = var_iqi_nqs_dn2;
        *var_iqi_nqs_dn4_slot = var_iqi_nqs_dn4;
        *var_iqi_nqs_dn5_slot = var_iqi_nqs_dn5;
        *var_iqi_nqs_dn6_slot = var_iqi_nqs_dn6;
        *var_iqi_nqs_dn8_slot = var_iqi_nqs_dn8;
        *var_mu__blk423_slot = var_mu__blk423;
        *var_mu__blk423_dn0_slot = var_mu__blk423_dn0;
        *var_mu__blk423_dn10_slot = var_mu__blk423_dn10;
        *var_mu__blk423_dn11_slot = var_mu__blk423_dn11;
        *var_mu__blk423_dn12_slot = var_mu__blk423_dn12;
        *var_mu__blk423_dn2_slot = var_mu__blk423_dn2;
        *var_mu__blk423_dn4_slot = var_mu__blk423_dn4;
        *var_mu__blk423_dn5_slot = var_mu__blk423_dn5;
        *var_mu__blk423_dn6_slot = var_mu__blk423_dn6;
        *var_mu__blk423_dn8_slot = var_mu__blk423_dn8;
        *var_qd_nqs_slot = var_qd_nqs;
        *var_qd_nqs_dn0_slot = var_qd_nqs_dn0;
        *var_qd_nqs_dn10_slot = var_qd_nqs_dn10;
        *var_qd_nqs_dn11_slot = var_qd_nqs_dn11;
        *var_qd_nqs_dn12_slot = var_qd_nqs_dn12;
        *var_qd_nqs_dn2_slot = var_qd_nqs_dn2;
        *var_qd_nqs_dn4_slot = var_qd_nqs_dn4;
        *var_qd_nqs_dn5_slot = var_qd_nqs_dn5;
        *var_qd_nqs_dn6_slot = var_qd_nqs_dn6;
        *var_qd_nqs_dn8_slot = var_qd_nqs_dn8;
        *var_qg_nqs_slot = var_qg_nqs;
        *var_qg_nqs_dn8_slot = var_qg_nqs_dn8;
        *var_qg_nqs_dn9_slot = var_qg_nqs_dn9;
        *var_qs_nqs_slot = var_qs_nqs;
        *var_qs_nqs_dn0_slot = var_qs_nqs_dn0;
        *var_qs_nqs_dn10_slot = var_qs_nqs_dn10;
        *var_qs_nqs_dn11_slot = var_qs_nqs_dn11;
        *var_qs_nqs_dn12_slot = var_qs_nqs_dn12;
        *var_qs_nqs_dn2_slot = var_qs_nqs_dn2;
        *var_qs_nqs_dn4_slot = var_qs_nqs_dn4;
        *var_qs_nqs_dn5_slot = var_qs_nqs_dn5;
        *var_qs_nqs_dn6_slot = var_qs_nqs_dn6;
        *var_qs_nqs_dn8_slot = var_qs_nqs_dn8;
        *var_rdde_slot = var_rdde;
        *var_rdde_dn0_slot = var_rdde_dn0;
        *var_rdde_dn10_slot = var_rdde_dn10;
        *var_rdde_dn11_slot = var_rdde_dn11;
        *var_rdde_dn12_slot = var_rdde_dn12;
        *var_rdde_dn2_slot = var_rdde_dn2;
        *var_rdde_dn4_slot = var_rdde_dn4;
        *var_rdde_dn5_slot = var_rdde_dn5;
        *var_rdde_dn6_slot = var_rdde_dn6;
        *var_rdde_dn8_slot = var_rdde_dn8;
        *var_rsd_slot = var_rsd;
        *var_rsd_dn0_slot = var_rsd_dn0;
        *var_rsd_dn10_slot = var_rsd_dn10;
        *var_rsd_dn11_slot = var_rsd_dn11;
        *var_rsd_dn12_slot = var_rsd_dn12;
        *var_rsd_dn2_slot = var_rsd_dn2;
        *var_rsd_dn4_slot = var_rsd_dn4;
        *var_rsd_dn5_slot = var_rsd_dn5;
        *var_rsd_dn6_slot = var_rsd_dn6;
        *var_rsd_dn8_slot = var_rsd_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_tau_slot = var_tau;
        *var_tau_dn0_slot = var_tau_dn0;
        *var_tau_dn10_slot = var_tau_dn10;
        *var_tau_dn11_slot = var_tau_dn11;
        *var_tau_dn12_slot = var_tau_dn12;
        *var_tau_dn2_slot = var_tau_dn2;
        *var_tau_dn4_slot = var_tau_dn4;
        *var_tau_dn5_slot = var_tau_dn5;
        *var_tau_dn6_slot = var_tau_dn6;
        *var_tau_dn8_slot = var_tau_dn8;
        *var_taub_slot = var_taub;
        *var_taub_dn0_slot = var_taub_dn0;
        *var_taub_dn10_slot = var_taub_dn10;
        *var_taub_dn11_slot = var_taub_dn11;
        *var_taub_dn12_slot = var_taub_dn12;
        *var_taub_dn2_slot = var_taub_dn2;
        *var_taub_dn4_slot = var_taub_dn4;
        *var_taub_dn5_slot = var_taub_dn5;
        *var_taub_dn6_slot = var_taub_dn6;
        *var_taub_dn8_slot = var_taub_dn8;
    }

    pub(super) fn stamp_transient_block_89(
        p: &Parameters,
        var_cth: f64,
        var_cth_dn0: f64,
        var_cth_dn10: f64,
        var_cth_dn11: f64,
        var_cth_dn12: f64,
        var_cth_dn2: f64,
        var_cth_dn4: f64,
        var_cth_dn5: f64,
        var_cth_dn6: f64,
        var_cth_dn8: f64,
        var_flg_nqs: f64,
        var_idse: f64,
        var_idse_dn0: f64,
        var_idse_dn10: f64,
        var_idse_dn11: f64,
        var_idse_dn12: f64,
        var_idse_dn2: f64,
        var_idse_dn4: f64,
        var_idse_dn5: f64,
        var_idse_dn6: f64,
        var_idse_dn8: f64,
        var_isube: f64,
        var_isube_dn0: f64,
        var_isube_dn10: f64,
        var_isube_dn11: f64,
        var_isube_dn12: f64,
        var_isube_dn2: f64,
        var_isube_dn4: f64,
        var_isube_dn5: f64,
        var_isube_dn6: f64,
        var_isube_dn8: f64,
        var_mode: f64,
        var_qge: f64,
        var_qge_dn0: f64,
        var_qge_dn10: f64,
        var_qge_dn11: f64,
        var_qge_dn12: f64,
        var_qge_dn2: f64,
        var_qge_dn4: f64,
        var_qge_dn5: f64,
        var_qge_dn6: f64,
        var_qge_dn8: f64,
        var_rdde: f64,
        var_rdde_dn0: f64,
        var_rdde_dn10: f64,
        var_rdde_dn11: f64,
        var_rdde_dn12: f64,
        var_rdde_dn2: f64,
        var_rdde_dn4: f64,
        var_rdde_dn5: f64,
        var_rdde_dn6: f64,
        var_rdde_dn8: f64,
        var_rsde: f64,
        var_rsde_dn0: f64,
        var_rsde_dn10: f64,
        var_rsde_dn11: f64,
        var_rsde_dn12: f64,
        var_rsde_dn2: f64,
        var_rsde_dn4: f64,
        var_rsde_dn5: f64,
        var_rsde_dn6: f64,
        var_rsde_dn8: f64,
        var_rth: f64,
        var_rth_dn0: f64,
        var_rth_dn10: f64,
        var_rth_dn11: f64,
        var_rth_dn12: f64,
        var_rth_dn2: f64,
        var_rth_dn4: f64,
        var_rth_dn5: f64,
        var_rth_dn6: f64,
        var_rth_dn8: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn8: f64,
        var_cthe_slot: &mut f64,
        var_cthe_dn0_slot: &mut f64,
        var_cthe_dn10_slot: &mut f64,
        var_cthe_dn11_slot: &mut f64,
        var_cthe_dn12_slot: &mut f64,
        var_cthe_dn2_slot: &mut f64,
        var_cthe_dn4_slot: &mut f64,
        var_cthe_dn5_slot: &mut f64,
        var_cthe_dn6_slot: &mut f64,
        var_cthe_dn8_slot: &mut f64,
        var_gth_slot: &mut f64,
        var_gth_dn0_slot: &mut f64,
        var_gth_dn10_slot: &mut f64,
        var_gth_dn11_slot: &mut f64,
        var_gth_dn12_slot: &mut f64,
        var_gth_dn2_slot: &mut f64,
        var_gth_dn4_slot: &mut f64,
        var_gth_dn5_slot: &mut f64,
        var_gth_dn6_slot: &mut f64,
        var_gth_dn8_slot: &mut f64,
        var_guard436_slot: &mut f64,
        var_guard437_slot: &mut f64,
        var_ids_slot: &mut f64,
        var_ids_dn0_slot: &mut f64,
        var_ids_dn10_slot: &mut f64,
        var_ids_dn11_slot: &mut f64,
        var_ids_dn12_slot: &mut f64,
        var_ids_dn2_slot: &mut f64,
        var_ids_dn4_slot: &mut f64,
        var_ids_dn5_slot: &mut f64,
        var_ids_dn6_slot: &mut f64,
        var_ids_dn8_slot: &mut f64,
        var_iqb_nqs_slot: &mut f64,
        var_iqb_nqs_dn0_slot: &mut f64,
        var_iqb_nqs_dn10_slot: &mut f64,
        var_iqb_nqs_dn11_slot: &mut f64,
        var_iqb_nqs_dn12_slot: &mut f64,
        var_iqb_nqs_dn2_slot: &mut f64,
        var_iqb_nqs_dn4_slot: &mut f64,
        var_iqb_nqs_dn5_slot: &mut f64,
        var_iqb_nqs_dn6_slot: &mut f64,
        var_iqb_nqs_dn8_slot: &mut f64,
        var_iqb_nqs_dn9_slot: &mut f64,
        var_isub_slot: &mut f64,
        var_isub_dn0_slot: &mut f64,
        var_isub_dn10_slot: &mut f64,
        var_isub_dn11_slot: &mut f64,
        var_isub_dn12_slot: &mut f64,
        var_isub_dn2_slot: &mut f64,
        var_isub_dn4_slot: &mut f64,
        var_isub_dn5_slot: &mut f64,
        var_isub_dn6_slot: &mut f64,
        var_isub_dn8_slot: &mut f64,
        var_isubs_slot: &mut f64,
        var_isubs_dn0_slot: &mut f64,
        var_isubs_dn10_slot: &mut f64,
        var_isubs_dn11_slot: &mut f64,
        var_isubs_dn12_slot: &mut f64,
        var_isubs_dn2_slot: &mut f64,
        var_isubs_dn4_slot: &mut f64,
        var_isubs_dn5_slot: &mut f64,
        var_isubs_dn6_slot: &mut f64,
        var_isubs_dn8_slot: &mut f64,
        var_qb_slot: &mut f64,
        var_qb_dn0_slot: &mut f64,
        var_qb_dn10_slot: &mut f64,
        var_qb_dn11_slot: &mut f64,
        var_qb_dn12_slot: &mut f64,
        var_qb_dn2_slot: &mut f64,
        var_qb_dn4_slot: &mut f64,
        var_qb_dn5_slot: &mut f64,
        var_qb_dn6_slot: &mut f64,
        var_qb_dn8_slot: &mut f64,
        var_qb_nqs_slot: &mut f64,
        var_qb_nqs_dn9_slot: &mut f64,
        var_qbe_slot: &mut f64,
        var_qbe_dn0_slot: &mut f64,
        var_qbe_dn10_slot: &mut f64,
        var_qbe_dn11_slot: &mut f64,
        var_qbe_dn12_slot: &mut f64,
        var_qbe_dn2_slot: &mut f64,
        var_qbe_dn4_slot: &mut f64,
        var_qbe_dn5_slot: &mut f64,
        var_qbe_dn6_slot: &mut f64,
        var_qbe_dn8_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn0_slot: &mut f64,
        var_qd_dn10_slot: &mut f64,
        var_qd_dn11_slot: &mut f64,
        var_qd_dn12_slot: &mut f64,
        var_qd_dn2_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qd_nqs_slot: &mut f64,
        var_qd_nqs_dn0_slot: &mut f64,
        var_qd_nqs_dn10_slot: &mut f64,
        var_qd_nqs_dn11_slot: &mut f64,
        var_qd_nqs_dn12_slot: &mut f64,
        var_qd_nqs_dn2_slot: &mut f64,
        var_qd_nqs_dn4_slot: &mut f64,
        var_qd_nqs_dn5_slot: &mut f64,
        var_qd_nqs_dn6_slot: &mut f64,
        var_qd_nqs_dn8_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_dn0_slot: &mut f64,
        var_qde_dn10_slot: &mut f64,
        var_qde_dn11_slot: &mut f64,
        var_qde_dn12_slot: &mut f64,
        var_qde_dn2_slot: &mut f64,
        var_qde_dn4_slot: &mut f64,
        var_qde_dn5_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qde_dn8_slot: &mut f64,
        var_qg_slot: &mut f64,
        var_qg_dn0_slot: &mut f64,
        var_qg_dn10_slot: &mut f64,
        var_qg_dn11_slot: &mut f64,
        var_qg_dn12_slot: &mut f64,
        var_qg_dn2_slot: &mut f64,
        var_qg_dn4_slot: &mut f64,
        var_qg_dn5_slot: &mut f64,
        var_qg_dn6_slot: &mut f64,
        var_qg_dn8_slot: &mut f64,
        var_qg_nqs_slot: &mut f64,
        var_qg_nqs_dn8_slot: &mut f64,
        var_qg_nqs_dn9_slot: &mut f64,
        var_qs_nqs_slot: &mut f64,
        var_qs_nqs_dn0_slot: &mut f64,
        var_qs_nqs_dn10_slot: &mut f64,
        var_qs_nqs_dn11_slot: &mut f64,
        var_qs_nqs_dn12_slot: &mut f64,
        var_qs_nqs_dn2_slot: &mut f64,
        var_qs_nqs_dn4_slot: &mut f64,
        var_qs_nqs_dn5_slot: &mut f64,
        var_qs_nqs_dn6_slot: &mut f64,
        var_qs_nqs_dn8_slot: &mut f64,
        var_qse_slot: &mut f64,
        var_qse_dn0_slot: &mut f64,
        var_qse_dn10_slot: &mut f64,
        var_qse_dn11_slot: &mut f64,
        var_qse_dn12_slot: &mut f64,
        var_qse_dn2_slot: &mut f64,
        var_qse_dn4_slot: &mut f64,
        var_qse_dn5_slot: &mut f64,
        var_qse_dn6_slot: &mut f64,
        var_qse_dn8_slot: &mut f64,
        var_rdd_slot: &mut f64,
        var_rdd_dn0_slot: &mut f64,
        var_rdd_dn10_slot: &mut f64,
        var_rdd_dn11_slot: &mut f64,
        var_rdd_dn12_slot: &mut f64,
        var_rdd_dn2_slot: &mut f64,
        var_rdd_dn4_slot: &mut f64,
        var_rdd_dn5_slot: &mut f64,
        var_rdd_dn6_slot: &mut f64,
        var_rdd_dn8_slot: &mut f64,
        var_rpower_slot: &mut f64,
        var_rpower_dn0_slot: &mut f64,
        var_rpower_dn10_slot: &mut f64,
        var_rpower_dn11_slot: &mut f64,
        var_rpower_dn12_slot: &mut f64,
        var_rpower_dn2_slot: &mut f64,
        var_rpower_dn4_slot: &mut f64,
        var_rpower_dn5_slot: &mut f64,
        var_rpower_dn6_slot: &mut f64,
        var_rpower_dn8_slot: &mut f64,
        var_rsd_slot: &mut f64,
        var_rsd_dn0_slot: &mut f64,
        var_rsd_dn10_slot: &mut f64,
        var_rsd_dn11_slot: &mut f64,
        var_rsd_dn12_slot: &mut f64,
        var_rsd_dn2_slot: &mut f64,
        var_rsd_dn4_slot: &mut f64,
        var_rsd_dn5_slot: &mut f64,
        var_rsd_dn6_slot: &mut f64,
        var_rsd_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
    ) {
        let mut var_cthe: f64 = *var_cthe_slot;
        let mut var_cthe_dn0: f64 = *var_cthe_dn0_slot;
        let mut var_cthe_dn10: f64 = *var_cthe_dn10_slot;
        let mut var_cthe_dn11: f64 = *var_cthe_dn11_slot;
        let mut var_cthe_dn12: f64 = *var_cthe_dn12_slot;
        let mut var_cthe_dn2: f64 = *var_cthe_dn2_slot;
        let mut var_cthe_dn4: f64 = *var_cthe_dn4_slot;
        let mut var_cthe_dn5: f64 = *var_cthe_dn5_slot;
        let mut var_cthe_dn6: f64 = *var_cthe_dn6_slot;
        let mut var_cthe_dn8: f64 = *var_cthe_dn8_slot;
        let mut var_gth: f64 = *var_gth_slot;
        let mut var_gth_dn0: f64 = *var_gth_dn0_slot;
        let mut var_gth_dn10: f64 = *var_gth_dn10_slot;
        let mut var_gth_dn11: f64 = *var_gth_dn11_slot;
        let mut var_gth_dn12: f64 = *var_gth_dn12_slot;
        let mut var_gth_dn2: f64 = *var_gth_dn2_slot;
        let mut var_gth_dn4: f64 = *var_gth_dn4_slot;
        let mut var_gth_dn5: f64 = *var_gth_dn5_slot;
        let mut var_gth_dn6: f64 = *var_gth_dn6_slot;
        let mut var_gth_dn8: f64 = *var_gth_dn8_slot;
        let mut var_guard436: f64 = *var_guard436_slot;
        let mut var_guard437: f64 = *var_guard437_slot;
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn0: f64 = *var_ids_dn0_slot;
        let mut var_ids_dn10: f64 = *var_ids_dn10_slot;
        let mut var_ids_dn11: f64 = *var_ids_dn11_slot;
        let mut var_ids_dn12: f64 = *var_ids_dn12_slot;
        let mut var_ids_dn2: f64 = *var_ids_dn2_slot;
        let mut var_ids_dn4: f64 = *var_ids_dn4_slot;
        let mut var_ids_dn5: f64 = *var_ids_dn5_slot;
        let mut var_ids_dn6: f64 = *var_ids_dn6_slot;
        let mut var_ids_dn8: f64 = *var_ids_dn8_slot;
        let mut var_iqb_nqs: f64 = *var_iqb_nqs_slot;
        let mut var_iqb_nqs_dn0: f64 = *var_iqb_nqs_dn0_slot;
        let mut var_iqb_nqs_dn10: f64 = *var_iqb_nqs_dn10_slot;
        let mut var_iqb_nqs_dn11: f64 = *var_iqb_nqs_dn11_slot;
        let mut var_iqb_nqs_dn12: f64 = *var_iqb_nqs_dn12_slot;
        let mut var_iqb_nqs_dn2: f64 = *var_iqb_nqs_dn2_slot;
        let mut var_iqb_nqs_dn4: f64 = *var_iqb_nqs_dn4_slot;
        let mut var_iqb_nqs_dn5: f64 = *var_iqb_nqs_dn5_slot;
        let mut var_iqb_nqs_dn6: f64 = *var_iqb_nqs_dn6_slot;
        let mut var_iqb_nqs_dn8: f64 = *var_iqb_nqs_dn8_slot;
        let mut var_iqb_nqs_dn9: f64 = *var_iqb_nqs_dn9_slot;
        let mut var_isub: f64 = *var_isub_slot;
        let mut var_isub_dn0: f64 = *var_isub_dn0_slot;
        let mut var_isub_dn10: f64 = *var_isub_dn10_slot;
        let mut var_isub_dn11: f64 = *var_isub_dn11_slot;
        let mut var_isub_dn12: f64 = *var_isub_dn12_slot;
        let mut var_isub_dn2: f64 = *var_isub_dn2_slot;
        let mut var_isub_dn4: f64 = *var_isub_dn4_slot;
        let mut var_isub_dn5: f64 = *var_isub_dn5_slot;
        let mut var_isub_dn6: f64 = *var_isub_dn6_slot;
        let mut var_isub_dn8: f64 = *var_isub_dn8_slot;
        let mut var_isubs: f64 = *var_isubs_slot;
        let mut var_isubs_dn0: f64 = *var_isubs_dn0_slot;
        let mut var_isubs_dn10: f64 = *var_isubs_dn10_slot;
        let mut var_isubs_dn11: f64 = *var_isubs_dn11_slot;
        let mut var_isubs_dn12: f64 = *var_isubs_dn12_slot;
        let mut var_isubs_dn2: f64 = *var_isubs_dn2_slot;
        let mut var_isubs_dn4: f64 = *var_isubs_dn4_slot;
        let mut var_isubs_dn5: f64 = *var_isubs_dn5_slot;
        let mut var_isubs_dn6: f64 = *var_isubs_dn6_slot;
        let mut var_isubs_dn8: f64 = *var_isubs_dn8_slot;
        let mut var_qb: f64 = *var_qb_slot;
        let mut var_qb_dn0: f64 = *var_qb_dn0_slot;
        let mut var_qb_dn10: f64 = *var_qb_dn10_slot;
        let mut var_qb_dn11: f64 = *var_qb_dn11_slot;
        let mut var_qb_dn12: f64 = *var_qb_dn12_slot;
        let mut var_qb_dn2: f64 = *var_qb_dn2_slot;
        let mut var_qb_dn4: f64 = *var_qb_dn4_slot;
        let mut var_qb_dn5: f64 = *var_qb_dn5_slot;
        let mut var_qb_dn6: f64 = *var_qb_dn6_slot;
        let mut var_qb_dn8: f64 = *var_qb_dn8_slot;
        let mut var_qb_nqs: f64 = *var_qb_nqs_slot;
        let mut var_qb_nqs_dn9: f64 = *var_qb_nqs_dn9_slot;
        let mut var_qbe: f64 = *var_qbe_slot;
        let mut var_qbe_dn0: f64 = *var_qbe_dn0_slot;
        let mut var_qbe_dn10: f64 = *var_qbe_dn10_slot;
        let mut var_qbe_dn11: f64 = *var_qbe_dn11_slot;
        let mut var_qbe_dn12: f64 = *var_qbe_dn12_slot;
        let mut var_qbe_dn2: f64 = *var_qbe_dn2_slot;
        let mut var_qbe_dn4: f64 = *var_qbe_dn4_slot;
        let mut var_qbe_dn5: f64 = *var_qbe_dn5_slot;
        let mut var_qbe_dn6: f64 = *var_qbe_dn6_slot;
        let mut var_qbe_dn8: f64 = *var_qbe_dn8_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn0: f64 = *var_qd_dn0_slot;
        let mut var_qd_dn10: f64 = *var_qd_dn10_slot;
        let mut var_qd_dn11: f64 = *var_qd_dn11_slot;
        let mut var_qd_dn12: f64 = *var_qd_dn12_slot;
        let mut var_qd_dn2: f64 = *var_qd_dn2_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qd_nqs: f64 = *var_qd_nqs_slot;
        let mut var_qd_nqs_dn0: f64 = *var_qd_nqs_dn0_slot;
        let mut var_qd_nqs_dn10: f64 = *var_qd_nqs_dn10_slot;
        let mut var_qd_nqs_dn11: f64 = *var_qd_nqs_dn11_slot;
        let mut var_qd_nqs_dn12: f64 = *var_qd_nqs_dn12_slot;
        let mut var_qd_nqs_dn2: f64 = *var_qd_nqs_dn2_slot;
        let mut var_qd_nqs_dn4: f64 = *var_qd_nqs_dn4_slot;
        let mut var_qd_nqs_dn5: f64 = *var_qd_nqs_dn5_slot;
        let mut var_qd_nqs_dn6: f64 = *var_qd_nqs_dn6_slot;
        let mut var_qd_nqs_dn8: f64 = *var_qd_nqs_dn8_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_dn0: f64 = *var_qde_dn0_slot;
        let mut var_qde_dn10: f64 = *var_qde_dn10_slot;
        let mut var_qde_dn11: f64 = *var_qde_dn11_slot;
        let mut var_qde_dn12: f64 = *var_qde_dn12_slot;
        let mut var_qde_dn2: f64 = *var_qde_dn2_slot;
        let mut var_qde_dn4: f64 = *var_qde_dn4_slot;
        let mut var_qde_dn5: f64 = *var_qde_dn5_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qde_dn8: f64 = *var_qde_dn8_slot;
        let mut var_qg: f64 = *var_qg_slot;
        let mut var_qg_dn0: f64 = *var_qg_dn0_slot;
        let mut var_qg_dn10: f64 = *var_qg_dn10_slot;
        let mut var_qg_dn11: f64 = *var_qg_dn11_slot;
        let mut var_qg_dn12: f64 = *var_qg_dn12_slot;
        let mut var_qg_dn2: f64 = *var_qg_dn2_slot;
        let mut var_qg_dn4: f64 = *var_qg_dn4_slot;
        let mut var_qg_dn5: f64 = *var_qg_dn5_slot;
        let mut var_qg_dn6: f64 = *var_qg_dn6_slot;
        let mut var_qg_dn8: f64 = *var_qg_dn8_slot;
        let mut var_qg_nqs: f64 = *var_qg_nqs_slot;
        let mut var_qg_nqs_dn8: f64 = *var_qg_nqs_dn8_slot;
        let mut var_qg_nqs_dn9: f64 = *var_qg_nqs_dn9_slot;
        let mut var_qs_nqs: f64 = *var_qs_nqs_slot;
        let mut var_qs_nqs_dn0: f64 = *var_qs_nqs_dn0_slot;
        let mut var_qs_nqs_dn10: f64 = *var_qs_nqs_dn10_slot;
        let mut var_qs_nqs_dn11: f64 = *var_qs_nqs_dn11_slot;
        let mut var_qs_nqs_dn12: f64 = *var_qs_nqs_dn12_slot;
        let mut var_qs_nqs_dn2: f64 = *var_qs_nqs_dn2_slot;
        let mut var_qs_nqs_dn4: f64 = *var_qs_nqs_dn4_slot;
        let mut var_qs_nqs_dn5: f64 = *var_qs_nqs_dn5_slot;
        let mut var_qs_nqs_dn6: f64 = *var_qs_nqs_dn6_slot;
        let mut var_qs_nqs_dn8: f64 = *var_qs_nqs_dn8_slot;
        let mut var_qse: f64 = *var_qse_slot;
        let mut var_qse_dn0: f64 = *var_qse_dn0_slot;
        let mut var_qse_dn10: f64 = *var_qse_dn10_slot;
        let mut var_qse_dn11: f64 = *var_qse_dn11_slot;
        let mut var_qse_dn12: f64 = *var_qse_dn12_slot;
        let mut var_qse_dn2: f64 = *var_qse_dn2_slot;
        let mut var_qse_dn4: f64 = *var_qse_dn4_slot;
        let mut var_qse_dn5: f64 = *var_qse_dn5_slot;
        let mut var_qse_dn6: f64 = *var_qse_dn6_slot;
        let mut var_qse_dn8: f64 = *var_qse_dn8_slot;
        let mut var_rdd: f64 = *var_rdd_slot;
        let mut var_rdd_dn0: f64 = *var_rdd_dn0_slot;
        let mut var_rdd_dn10: f64 = *var_rdd_dn10_slot;
        let mut var_rdd_dn11: f64 = *var_rdd_dn11_slot;
        let mut var_rdd_dn12: f64 = *var_rdd_dn12_slot;
        let mut var_rdd_dn2: f64 = *var_rdd_dn2_slot;
        let mut var_rdd_dn4: f64 = *var_rdd_dn4_slot;
        let mut var_rdd_dn5: f64 = *var_rdd_dn5_slot;
        let mut var_rdd_dn6: f64 = *var_rdd_dn6_slot;
        let mut var_rdd_dn8: f64 = *var_rdd_dn8_slot;
        let mut var_rpower: f64 = *var_rpower_slot;
        let mut var_rpower_dn0: f64 = *var_rpower_dn0_slot;
        let mut var_rpower_dn10: f64 = *var_rpower_dn10_slot;
        let mut var_rpower_dn11: f64 = *var_rpower_dn11_slot;
        let mut var_rpower_dn12: f64 = *var_rpower_dn12_slot;
        let mut var_rpower_dn2: f64 = *var_rpower_dn2_slot;
        let mut var_rpower_dn4: f64 = *var_rpower_dn4_slot;
        let mut var_rpower_dn5: f64 = *var_rpower_dn5_slot;
        let mut var_rpower_dn6: f64 = *var_rpower_dn6_slot;
        let mut var_rpower_dn8: f64 = *var_rpower_dn8_slot;
        let mut var_rsd: f64 = *var_rsd_slot;
        let mut var_rsd_dn0: f64 = *var_rsd_dn0_slot;
        let mut var_rsd_dn10: f64 = *var_rsd_dn10_slot;
        let mut var_rsd_dn11: f64 = *var_rsd_dn11_slot;
        let mut var_rsd_dn12: f64 = *var_rsd_dn12_slot;
        let mut var_rsd_dn2: f64 = *var_rsd_dn2_slot;
        let mut var_rsd_dn4: f64 = *var_rsd_dn4_slot;
        let mut var_rsd_dn5: f64 = *var_rsd_dn5_slot;
        let mut var_rsd_dn6: f64 = *var_rsd_dn6_slot;
        let mut var_rsd_dn8: f64 = *var_rsd_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;

        let (assign23620_e28670, assign23620_e28670_d_n0, assign23620_e28670_d_n2, assign23620_e28670_d_n4, assign23620_e28670_d_n5, assign23620_e28670_d_n6, assign23620_e28670_d_n8, assign23620_e28670_d_n9, assign23620_e28670_d_n10, assign23620_e28670_d_n11, assign23620_e28670_d_n12,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn4, var_iqb_nqs_dn5, var_iqb_nqs_dn6, var_iqb_nqs_dn8, var_iqb_nqs_dn9, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12,)
    }
};
        var_iqb_nqs = assign23620_e28670;
        var_iqb_nqs_dn0 = assign23620_e28670_d_n0;
        var_iqb_nqs_dn2 = assign23620_e28670_d_n2;
        var_iqb_nqs_dn4 = assign23620_e28670_d_n4;
        var_iqb_nqs_dn5 = assign23620_e28670_d_n5;
        var_iqb_nqs_dn6 = assign23620_e28670_d_n6;
        var_iqb_nqs_dn8 = assign23620_e28670_d_n8;
        var_iqb_nqs_dn9 = assign23620_e28670_d_n9;
        var_iqb_nqs_dn10 = assign23620_e28670_d_n10;
        var_iqb_nqs_dn11 = assign23620_e28670_d_n11;
        var_iqb_nqs_dn12 = assign23620_e28670_d_n12;

        let (assign23630_e28675, assign23630_e28675_d_n0, assign23630_e28675_d_n2, assign23630_e28675_d_n4, assign23630_e28675_d_n5, assign23630_e28675_d_n6, assign23630_e28675_d_n8, assign23630_e28675_d_n10, assign23630_e28675_d_n11, assign23630_e28675_d_n12,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn4, var_qd_nqs_dn5, var_qd_nqs_dn6, var_qd_nqs_dn8, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12,)
    }
};
        var_qd_nqs = assign23630_e28675;
        var_qd_nqs_dn0 = assign23630_e28675_d_n0;
        var_qd_nqs_dn2 = assign23630_e28675_d_n2;
        var_qd_nqs_dn4 = assign23630_e28675_d_n4;
        var_qd_nqs_dn5 = assign23630_e28675_d_n5;
        var_qd_nqs_dn6 = assign23630_e28675_d_n6;
        var_qd_nqs_dn8 = assign23630_e28675_d_n8;
        var_qd_nqs_dn10 = assign23630_e28675_d_n10;
        var_qd_nqs_dn11 = assign23630_e28675_d_n11;
        var_qd_nqs_dn12 = assign23630_e28675_d_n12;

        let (assign23640_e28680, assign23640_e28680_d_n0, assign23640_e28680_d_n2, assign23640_e28680_d_n4, assign23640_e28680_d_n5, assign23640_e28680_d_n6, assign23640_e28680_d_n8, assign23640_e28680_d_n10, assign23640_e28680_d_n11, assign23640_e28680_d_n12,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn4, var_qs_nqs_dn5, var_qs_nqs_dn6, var_qs_nqs_dn8, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12,)
    }
};
        var_qs_nqs = assign23640_e28680;
        var_qs_nqs_dn0 = assign23640_e28680_d_n0;
        var_qs_nqs_dn2 = assign23640_e28680_d_n2;
        var_qs_nqs_dn4 = assign23640_e28680_d_n4;
        var_qs_nqs_dn5 = assign23640_e28680_d_n5;
        var_qs_nqs_dn6 = assign23640_e28680_d_n6;
        var_qs_nqs_dn8 = assign23640_e28680_d_n8;
        var_qs_nqs_dn10 = assign23640_e28680_d_n10;
        var_qs_nqs_dn11 = assign23640_e28680_d_n11;
        var_qs_nqs_dn12 = assign23640_e28680_d_n12;

        let (assign23650_e28685, assign23650_e28685_d_n8, assign23650_e28685_d_n9,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_qg_nqs, var_qg_nqs_dn8, var_qg_nqs_dn9,)
    }
};
        var_qg_nqs = assign23650_e28685;
        var_qg_nqs_dn8 = assign23650_e28685_d_n8;
        var_qg_nqs_dn9 = assign23650_e28685_d_n9;

        let (assign23660_e28690, assign23660_e28690_d_n9,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn9,)
    }
};
        var_qb_nqs = assign23660_e28690;
        var_qb_nqs_dn9 = assign23660_e28690_d_n9;

        var_rdd = var_rdde;
        var_rdd_dn0 = var_rdde_dn0;
        var_rdd_dn2 = var_rdde_dn2;
        var_rdd_dn4 = var_rdde_dn4;
        var_rdd_dn5 = var_rdde_dn5;
        var_rdd_dn6 = var_rdde_dn6;
        var_rdd_dn8 = var_rdde_dn8;
        var_rdd_dn10 = var_rdde_dn10;
        var_rdd_dn11 = var_rdde_dn11;
        var_rdd_dn12 = var_rdde_dn12;

        var_rsd = var_rsde;
        var_rsd_dn0 = var_rsde_dn0;
        var_rsd_dn2 = var_rsde_dn2;
        var_rsd_dn4 = var_rsde_dn4;
        var_rsd_dn5 = var_rsde_dn5;
        var_rsd_dn6 = var_rsde_dn6;
        var_rsd_dn8 = var_rsde_dn8;
        var_rsd_dn10 = var_rsde_dn10;
        var_rsd_dn11 = var_rsde_dn11;
        var_rsd_dn12 = var_rsde_dn12;

        let assign23690_e28695: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard436 = assign23690_e28695;

        let (assign23700_e28699, assign23700_e28699_d_n0, assign23700_e28699_d_n2, assign23700_e28699_d_n4, assign23700_e28699_d_n5, assign23700_e28699_d_n6, assign23700_e28699_d_n8, assign23700_e28699_d_n10, assign23700_e28699_d_n11, assign23700_e28699_d_n12,) = {
    if (var_guard436 != 0.0) {
        (var_idse, var_idse_dn0, var_idse_dn2, var_idse_dn4, var_idse_dn5, var_idse_dn6, var_idse_dn8, var_idse_dn10, var_idse_dn11, var_idse_dn12,)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn4, var_ids_dn5, var_ids_dn6, var_ids_dn8, var_ids_dn10, var_ids_dn11, var_ids_dn12,)
    }
};
        var_ids = assign23700_e28699;
        var_ids_dn0 = assign23700_e28699_d_n0;
        var_ids_dn2 = assign23700_e28699_d_n2;
        var_ids_dn4 = assign23700_e28699_d_n4;
        var_ids_dn5 = assign23700_e28699_d_n5;
        var_ids_dn6 = assign23700_e28699_d_n6;
        var_ids_dn8 = assign23700_e28699_d_n8;
        var_ids_dn10 = assign23700_e28699_d_n10;
        var_ids_dn11 = assign23700_e28699_d_n11;
        var_ids_dn12 = assign23700_e28699_d_n12;

        let (assign23710_e28703, assign23710_e28703_d_n0, assign23710_e28703_d_n2, assign23710_e28703_d_n4, assign23710_e28703_d_n5, assign23710_e28703_d_n6, assign23710_e28703_d_n8, assign23710_e28703_d_n10, assign23710_e28703_d_n11, assign23710_e28703_d_n12,) = {
    if (var_guard436 != 0.0) {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn4, var_isube_dn5, var_isube_dn6, var_isube_dn8, var_isube_dn10, var_isube_dn11, var_isube_dn12,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn4, var_isub_dn5, var_isub_dn6, var_isub_dn8, var_isub_dn10, var_isub_dn11, var_isub_dn12,)
    }
};
        var_isub = assign23710_e28703;
        var_isub_dn0 = assign23710_e28703_d_n0;
        var_isub_dn2 = assign23710_e28703_d_n2;
        var_isub_dn4 = assign23710_e28703_d_n4;
        var_isub_dn5 = assign23710_e28703_d_n5;
        var_isub_dn6 = assign23710_e28703_d_n6;
        var_isub_dn8 = assign23710_e28703_d_n8;
        var_isub_dn10 = assign23710_e28703_d_n10;
        var_isub_dn11 = assign23710_e28703_d_n11;
        var_isub_dn12 = assign23710_e28703_d_n12;

        let (assign23720_e28707, assign23720_e28707_d_n0, assign23720_e28707_d_n2, assign23720_e28707_d_n4, assign23720_e28707_d_n5, assign23720_e28707_d_n6, assign23720_e28707_d_n8, assign23720_e28707_d_n10, assign23720_e28707_d_n11, assign23720_e28707_d_n12,) = {
    if (var_guard436 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isubs, var_isubs_dn0, var_isubs_dn2, var_isubs_dn4, var_isubs_dn5, var_isubs_dn6, var_isubs_dn8, var_isubs_dn10, var_isubs_dn11, var_isubs_dn12,)
    }
};
        var_isubs = assign23720_e28707;
        var_isubs_dn0 = assign23720_e28707_d_n0;
        var_isubs_dn2 = assign23720_e28707_d_n2;
        var_isubs_dn4 = assign23720_e28707_d_n4;
        var_isubs_dn5 = assign23720_e28707_d_n5;
        var_isubs_dn6 = assign23720_e28707_d_n6;
        var_isubs_dn8 = assign23720_e28707_d_n8;
        var_isubs_dn10 = assign23720_e28707_d_n10;
        var_isubs_dn11 = assign23720_e28707_d_n11;
        var_isubs_dn12 = assign23720_e28707_d_n12;

        let (assign23730_e28711, assign23730_e28711_d_n0, assign23730_e28711_d_n2, assign23730_e28711_d_n4, assign23730_e28711_d_n5, assign23730_e28711_d_n6, assign23730_e28711_d_n8, assign23730_e28711_d_n10, assign23730_e28711_d_n11, assign23730_e28711_d_n12,) = {
    if (var_guard436 != 0.0) {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn4, var_qge_dn5, var_qge_dn6, var_qge_dn8, var_qge_dn10, var_qge_dn11, var_qge_dn12,)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn4, var_qg_dn5, var_qg_dn6, var_qg_dn8, var_qg_dn10, var_qg_dn11, var_qg_dn12,)
    }
};
        var_qg = assign23730_e28711;
        var_qg_dn0 = assign23730_e28711_d_n0;
        var_qg_dn2 = assign23730_e28711_d_n2;
        var_qg_dn4 = assign23730_e28711_d_n4;
        var_qg_dn5 = assign23730_e28711_d_n5;
        var_qg_dn6 = assign23730_e28711_d_n6;
        var_qg_dn8 = assign23730_e28711_d_n8;
        var_qg_dn10 = assign23730_e28711_d_n10;
        var_qg_dn11 = assign23730_e28711_d_n11;
        var_qg_dn12 = assign23730_e28711_d_n12;

        let (assign23740_e28715, assign23740_e28715_d_n0, assign23740_e28715_d_n2, assign23740_e28715_d_n4, assign23740_e28715_d_n5, assign23740_e28715_d_n6, assign23740_e28715_d_n8, assign23740_e28715_d_n10, assign23740_e28715_d_n11, assign23740_e28715_d_n12,) = {
    if (var_guard436 != 0.0) {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn8, var_qde_dn10, var_qde_dn11, var_qde_dn12,)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn8, var_qd_dn10, var_qd_dn11, var_qd_dn12,)
    }
};
        var_qd = assign23740_e28715;
        var_qd_dn0 = assign23740_e28715_d_n0;
        var_qd_dn2 = assign23740_e28715_d_n2;
        var_qd_dn4 = assign23740_e28715_d_n4;
        var_qd_dn5 = assign23740_e28715_d_n5;
        var_qd_dn6 = assign23740_e28715_d_n6;
        var_qd_dn8 = assign23740_e28715_d_n8;
        var_qd_dn10 = assign23740_e28715_d_n10;
        var_qd_dn11 = assign23740_e28715_d_n11;
        var_qd_dn12 = assign23740_e28715_d_n12;

        let (assign23750_e28724, assign23750_e28724_d_n0, assign23750_e28724_d_n2, assign23750_e28724_d_n4, assign23750_e28724_d_n5, assign23750_e28724_d_n6, assign23750_e28724_d_n8, assign23750_e28724_d_n10, assign23750_e28724_d_n11, assign23750_e28724_d_n12,) = {
    if (var_guard436 != 0.0) {
        let assign23750_e28719: f64 = (var_qge + var_qde);
        let assign23750_e28721: f64 = (assign23750_e28719 + var_qse);
        let assign23750_e28722: f64 = (-assign23750_e28721);
        (assign23750_e28722, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn4 + var_qde_dn4) + var_qse_dn4)), (-((var_qge_dn5 + var_qde_dn5) + var_qse_dn5)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn8 + var_qde_dn8) + var_qse_dn8)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn11 + var_qde_dn11) + var_qse_dn11)), (-((var_qge_dn12 + var_qde_dn12) + var_qse_dn12)),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn8, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12,)
    }
};
        var_qbe = assign23750_e28724;
        var_qbe_dn0 = assign23750_e28724_d_n0;
        var_qbe_dn2 = assign23750_e28724_d_n2;
        var_qbe_dn4 = assign23750_e28724_d_n4;
        var_qbe_dn5 = assign23750_e28724_d_n5;
        var_qbe_dn6 = assign23750_e28724_d_n6;
        var_qbe_dn8 = assign23750_e28724_d_n8;
        var_qbe_dn10 = assign23750_e28724_d_n10;
        var_qbe_dn11 = assign23750_e28724_d_n11;
        var_qbe_dn12 = assign23750_e28724_d_n12;

        let (assign23760_e28728, assign23760_e28728_d_n0, assign23760_e28728_d_n2, assign23760_e28728_d_n4, assign23760_e28728_d_n5, assign23760_e28728_d_n6, assign23760_e28728_d_n8, assign23760_e28728_d_n10, assign23760_e28728_d_n11, assign23760_e28728_d_n12,) = {
    if (var_guard436 != 0.0) {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn8, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12,)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn4, var_qb_dn5, var_qb_dn6, var_qb_dn8, var_qb_dn10, var_qb_dn11, var_qb_dn12,)
    }
};
        var_qb = assign23760_e28728;
        var_qb_dn0 = assign23760_e28728_d_n0;
        var_qb_dn2 = assign23760_e28728_d_n2;
        var_qb_dn4 = assign23760_e28728_d_n4;
        var_qb_dn5 = assign23760_e28728_d_n5;
        var_qb_dn6 = assign23760_e28728_d_n6;
        var_qb_dn8 = assign23760_e28728_d_n8;
        var_qb_dn10 = assign23760_e28728_d_n10;
        var_qb_dn11 = assign23760_e28728_d_n11;
        var_qb_dn12 = assign23760_e28728_d_n12;

        let (assign23770_e28734, assign23770_e28734_d_n0, assign23770_e28734_d_n2, assign23770_e28734_d_n4, assign23770_e28734_d_n5, assign23770_e28734_d_n6, assign23770_e28734_d_n8, assign23770_e28734_d_n10, assign23770_e28734_d_n11, assign23770_e28734_d_n12,) = {
    if (var_guard436 == 0.0) {
        let assign23770_e28732: f64 = (-var_idse);
        (assign23770_e28732, (-var_idse_dn0), (-var_idse_dn2), (-var_idse_dn4), (-var_idse_dn5), (-var_idse_dn6), (-var_idse_dn8), (-var_idse_dn10), (-var_idse_dn11), (-var_idse_dn12),)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn4, var_ids_dn5, var_ids_dn6, var_ids_dn8, var_ids_dn10, var_ids_dn11, var_ids_dn12,)
    }
};
        var_ids = assign23770_e28734;
        var_ids_dn0 = assign23770_e28734_d_n0;
        var_ids_dn2 = assign23770_e28734_d_n2;
        var_ids_dn4 = assign23770_e28734_d_n4;
        var_ids_dn5 = assign23770_e28734_d_n5;
        var_ids_dn6 = assign23770_e28734_d_n6;
        var_ids_dn8 = assign23770_e28734_d_n8;
        var_ids_dn10 = assign23770_e28734_d_n10;
        var_ids_dn11 = assign23770_e28734_d_n11;
        var_ids_dn12 = assign23770_e28734_d_n12;

        let (assign23780_e28739, assign23780_e28739_d_n0, assign23780_e28739_d_n2, assign23780_e28739_d_n4, assign23780_e28739_d_n5, assign23780_e28739_d_n6, assign23780_e28739_d_n8, assign23780_e28739_d_n10, assign23780_e28739_d_n11, assign23780_e28739_d_n12,) = {
    if (var_guard436 == 0.0) {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn4, var_isube_dn5, var_isube_dn6, var_isube_dn8, var_isube_dn10, var_isube_dn11, var_isube_dn12,)
    } else {
        (var_isubs, var_isubs_dn0, var_isubs_dn2, var_isubs_dn4, var_isubs_dn5, var_isubs_dn6, var_isubs_dn8, var_isubs_dn10, var_isubs_dn11, var_isubs_dn12,)
    }
};
        var_isubs = assign23780_e28739;
        var_isubs_dn0 = assign23780_e28739_d_n0;
        var_isubs_dn2 = assign23780_e28739_d_n2;
        var_isubs_dn4 = assign23780_e28739_d_n4;
        var_isubs_dn5 = assign23780_e28739_d_n5;
        var_isubs_dn6 = assign23780_e28739_d_n6;
        var_isubs_dn8 = assign23780_e28739_d_n8;
        var_isubs_dn10 = assign23780_e28739_d_n10;
        var_isubs_dn11 = assign23780_e28739_d_n11;
        var_isubs_dn12 = assign23780_e28739_d_n12;

        let (assign23790_e28744, assign23790_e28744_d_n0, assign23790_e28744_d_n2, assign23790_e28744_d_n4, assign23790_e28744_d_n5, assign23790_e28744_d_n6, assign23790_e28744_d_n8, assign23790_e28744_d_n10, assign23790_e28744_d_n11, assign23790_e28744_d_n12,) = {
    if (var_guard436 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn4, var_isub_dn5, var_isub_dn6, var_isub_dn8, var_isub_dn10, var_isub_dn11, var_isub_dn12,)
    }
};
        var_isub = assign23790_e28744;
        var_isub_dn0 = assign23790_e28744_d_n0;
        var_isub_dn2 = assign23790_e28744_d_n2;
        var_isub_dn4 = assign23790_e28744_d_n4;
        var_isub_dn5 = assign23790_e28744_d_n5;
        var_isub_dn6 = assign23790_e28744_d_n6;
        var_isub_dn8 = assign23790_e28744_d_n8;
        var_isub_dn10 = assign23790_e28744_d_n10;
        var_isub_dn11 = assign23790_e28744_d_n11;
        var_isub_dn12 = assign23790_e28744_d_n12;

        let (assign23800_e28749, assign23800_e28749_d_n0, assign23800_e28749_d_n2, assign23800_e28749_d_n4, assign23800_e28749_d_n5, assign23800_e28749_d_n6, assign23800_e28749_d_n8, assign23800_e28749_d_n10, assign23800_e28749_d_n11, assign23800_e28749_d_n12,) = {
    if (var_guard436 == 0.0) {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn4, var_qge_dn5, var_qge_dn6, var_qge_dn8, var_qge_dn10, var_qge_dn11, var_qge_dn12,)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn4, var_qg_dn5, var_qg_dn6, var_qg_dn8, var_qg_dn10, var_qg_dn11, var_qg_dn12,)
    }
};
        var_qg = assign23800_e28749;
        var_qg_dn0 = assign23800_e28749_d_n0;
        var_qg_dn2 = assign23800_e28749_d_n2;
        var_qg_dn4 = assign23800_e28749_d_n4;
        var_qg_dn5 = assign23800_e28749_d_n5;
        var_qg_dn6 = assign23800_e28749_d_n6;
        var_qg_dn8 = assign23800_e28749_d_n8;
        var_qg_dn10 = assign23800_e28749_d_n10;
        var_qg_dn11 = assign23800_e28749_d_n11;
        var_qg_dn12 = assign23800_e28749_d_n12;

        let (assign23810_e28754, assign23810_e28754_d_n0, assign23810_e28754_d_n2, assign23810_e28754_d_n4, assign23810_e28754_d_n5, assign23810_e28754_d_n6, assign23810_e28754_d_n8, assign23810_e28754_d_n10, assign23810_e28754_d_n11, assign23810_e28754_d_n12,) = {
    if (var_guard436 == 0.0) {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn4, var_qse_dn5, var_qse_dn6, var_qse_dn8, var_qse_dn10, var_qse_dn11, var_qse_dn12,)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn8, var_qd_dn10, var_qd_dn11, var_qd_dn12,)
    }
};
        var_qd = assign23810_e28754;
        var_qd_dn0 = assign23810_e28754_d_n0;
        var_qd_dn2 = assign23810_e28754_d_n2;
        var_qd_dn4 = assign23810_e28754_d_n4;
        var_qd_dn5 = assign23810_e28754_d_n5;
        var_qd_dn6 = assign23810_e28754_d_n6;
        var_qd_dn8 = assign23810_e28754_d_n8;
        var_qd_dn10 = assign23810_e28754_d_n10;
        var_qd_dn11 = assign23810_e28754_d_n11;
        var_qd_dn12 = assign23810_e28754_d_n12;

        let (assign23820_e28764, assign23820_e28764_d_n0, assign23820_e28764_d_n2, assign23820_e28764_d_n4, assign23820_e28764_d_n5, assign23820_e28764_d_n6, assign23820_e28764_d_n8, assign23820_e28764_d_n10, assign23820_e28764_d_n11, assign23820_e28764_d_n12,) = {
    if (var_guard436 == 0.0) {
        let assign23820_e28759: f64 = (var_qge + var_qde);
        let assign23820_e28761: f64 = (assign23820_e28759 + var_qse);
        let assign23820_e28762: f64 = (-assign23820_e28761);
        (assign23820_e28762, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn4 + var_qde_dn4) + var_qse_dn4)), (-((var_qge_dn5 + var_qde_dn5) + var_qse_dn5)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn8 + var_qde_dn8) + var_qse_dn8)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn11 + var_qde_dn11) + var_qse_dn11)), (-((var_qge_dn12 + var_qde_dn12) + var_qse_dn12)),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn8, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12,)
    }
};
        var_qbe = assign23820_e28764;
        var_qbe_dn0 = assign23820_e28764_d_n0;
        var_qbe_dn2 = assign23820_e28764_d_n2;
        var_qbe_dn4 = assign23820_e28764_d_n4;
        var_qbe_dn5 = assign23820_e28764_d_n5;
        var_qbe_dn6 = assign23820_e28764_d_n6;
        var_qbe_dn8 = assign23820_e28764_d_n8;
        var_qbe_dn10 = assign23820_e28764_d_n10;
        var_qbe_dn11 = assign23820_e28764_d_n11;
        var_qbe_dn12 = assign23820_e28764_d_n12;

        let (assign23830_e28769, assign23830_e28769_d_n0, assign23830_e28769_d_n2, assign23830_e28769_d_n4, assign23830_e28769_d_n5, assign23830_e28769_d_n6, assign23830_e28769_d_n8, assign23830_e28769_d_n10, assign23830_e28769_d_n11, assign23830_e28769_d_n12,) = {
    if (var_guard436 == 0.0) {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn8, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12,)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn4, var_qb_dn5, var_qb_dn6, var_qb_dn8, var_qb_dn10, var_qb_dn11, var_qb_dn12,)
    }
};
        var_qb = assign23830_e28769;
        var_qb_dn0 = assign23830_e28769_d_n0;
        var_qb_dn2 = assign23830_e28769_d_n2;
        var_qb_dn4 = assign23830_e28769_d_n4;
        var_qb_dn5 = assign23830_e28769_d_n5;
        var_qb_dn6 = assign23830_e28769_d_n6;
        var_qb_dn8 = assign23830_e28769_d_n8;
        var_qb_dn10 = assign23830_e28769_d_n10;
        var_qb_dn11 = assign23830_e28769_d_n11;
        var_qb_dn12 = assign23830_e28769_d_n12;

        let (assign23840_e28774, assign23840_e28774_d_n0, assign23840_e28774_d_n2, assign23840_e28774_d_n4, assign23840_e28774_d_n5, assign23840_e28774_d_n6, assign23840_e28774_d_n8, assign23840_e28774_d_n10, assign23840_e28774_d_n11, assign23840_e28774_d_n12,) = {
    if (var_guard436 == 0.0) {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn8, var_qde_dn10, var_qde_dn11, var_qde_dn12,)
    } else {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn4, var_qse_dn5, var_qse_dn6, var_qse_dn8, var_qse_dn10, var_qse_dn11, var_qse_dn12,)
    }
};
        var_qse = assign23840_e28774;
        var_qse_dn0 = assign23840_e28774_d_n0;
        var_qse_dn2 = assign23840_e28774_d_n2;
        var_qse_dn4 = assign23840_e28774_d_n4;
        var_qse_dn5 = assign23840_e28774_d_n5;
        var_qse_dn6 = assign23840_e28774_d_n6;
        var_qse_dn8 = assign23840_e28774_d_n8;
        var_qse_dn10 = assign23840_e28774_d_n10;
        var_qse_dn11 = assign23840_e28774_d_n11;
        var_qse_dn12 = assign23840_e28774_d_n12;

        let (assign23850_e28779, assign23850_e28779_d_n0, assign23850_e28779_d_n2, assign23850_e28779_d_n4, assign23850_e28779_d_n5, assign23850_e28779_d_n6, assign23850_e28779_d_n8, assign23850_e28779_d_n10, assign23850_e28779_d_n11, assign23850_e28779_d_n12,) = {
    if (var_guard436 == 0.0) {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn8, var_qd_dn10, var_qd_dn11, var_qd_dn12,)
    } else {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn8, var_qde_dn10, var_qde_dn11, var_qde_dn12,)
    }
};
        var_qde = assign23850_e28779;
        var_qde_dn0 = assign23850_e28779_d_n0;
        var_qde_dn2 = assign23850_e28779_d_n2;
        var_qde_dn4 = assign23850_e28779_d_n4;
        var_qde_dn5 = assign23850_e28779_d_n5;
        var_qde_dn6 = assign23850_e28779_d_n6;
        var_qde_dn8 = assign23850_e28779_d_n8;
        var_qde_dn10 = assign23850_e28779_d_n10;
        var_qde_dn11 = assign23850_e28779_d_n11;
        var_qde_dn12 = assign23850_e28779_d_n12;

        let (assign23860_e28786, assign23860_e28786_d_n0, assign23860_e28786_d_n2, assign23860_e28786_d_n4, assign23860_e28786_d_n5, assign23860_e28786_d_n6, assign23860_e28786_d_n8, assign23860_e28786_d_n10, assign23860_e28786_d_n11, assign23860_e28786_d_n12,) = {
    if ((var_guard436 == 0.0) && (var_flg_nqs != 0.0)) {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn4, var_qd_nqs_dn5, var_qd_nqs_dn6, var_qd_nqs_dn8, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign23860_e28786;
        var_t1_dn0 = assign23860_e28786_d_n0;
        var_t1_dn2 = assign23860_e28786_d_n2;
        var_t1_dn4 = assign23860_e28786_d_n4;
        var_t1_dn5 = assign23860_e28786_d_n5;
        var_t1_dn6 = assign23860_e28786_d_n6;
        var_t1_dn8 = assign23860_e28786_d_n8;
        var_t1_dn10 = assign23860_e28786_d_n10;
        var_t1_dn11 = assign23860_e28786_d_n11;
        var_t1_dn12 = assign23860_e28786_d_n12;

        let (assign23870_e28793, assign23870_e28793_d_n0, assign23870_e28793_d_n2, assign23870_e28793_d_n4, assign23870_e28793_d_n5, assign23870_e28793_d_n6, assign23870_e28793_d_n8, assign23870_e28793_d_n10, assign23870_e28793_d_n11, assign23870_e28793_d_n12,) = {
    if ((var_guard436 == 0.0) && (var_flg_nqs != 0.0)) {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn4, var_qs_nqs_dn5, var_qs_nqs_dn6, var_qs_nqs_dn8, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn4, var_qd_nqs_dn5, var_qd_nqs_dn6, var_qd_nqs_dn8, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12,)
    }
};
        var_qd_nqs = assign23870_e28793;
        var_qd_nqs_dn0 = assign23870_e28793_d_n0;
        var_qd_nqs_dn2 = assign23870_e28793_d_n2;
        var_qd_nqs_dn4 = assign23870_e28793_d_n4;
        var_qd_nqs_dn5 = assign23870_e28793_d_n5;
        var_qd_nqs_dn6 = assign23870_e28793_d_n6;
        var_qd_nqs_dn8 = assign23870_e28793_d_n8;
        var_qd_nqs_dn10 = assign23870_e28793_d_n10;
        var_qd_nqs_dn11 = assign23870_e28793_d_n11;
        var_qd_nqs_dn12 = assign23870_e28793_d_n12;

        let (assign23880_e28800, assign23880_e28800_d_n0, assign23880_e28800_d_n2, assign23880_e28800_d_n4, assign23880_e28800_d_n5, assign23880_e28800_d_n6, assign23880_e28800_d_n8, assign23880_e28800_d_n10, assign23880_e28800_d_n11, assign23880_e28800_d_n12,) = {
    if ((var_guard436 == 0.0) && (var_flg_nqs != 0.0)) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn4, var_qs_nqs_dn5, var_qs_nqs_dn6, var_qs_nqs_dn8, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12,)
    }
};
        var_qs_nqs = assign23880_e28800;
        var_qs_nqs_dn0 = assign23880_e28800_d_n0;
        var_qs_nqs_dn2 = assign23880_e28800_d_n2;
        var_qs_nqs_dn4 = assign23880_e28800_d_n4;
        var_qs_nqs_dn5 = assign23880_e28800_d_n5;
        var_qs_nqs_dn6 = assign23880_e28800_d_n6;
        var_qs_nqs_dn8 = assign23880_e28800_d_n8;
        var_qs_nqs_dn10 = assign23880_e28800_d_n10;
        var_qs_nqs_dn11 = assign23880_e28800_d_n11;
        var_qs_nqs_dn12 = assign23880_e28800_d_n12;

        let assign23890_e28805: f64 = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };
        var_guard437 = assign23890_e28805;

        let (assign23900_e28811, assign23900_e28811_d_n0, assign23900_e28811_d_n2, assign23900_e28811_d_n4, assign23900_e28811_d_n5, assign23900_e28811_d_n6, assign23900_e28811_d_n8, assign23900_e28811_d_n10, assign23900_e28811_d_n11, assign23900_e28811_d_n12,) = {
    if (var_guard437 != 0.0) {
        let assign23900_e28809: f64 = (var_idse * var_vds);
        (assign23900_e28809, ((var_idse_dn0 * var_vds) + (var_idse * var_vds_dn0)), ((var_idse_dn2 * var_vds) + (var_idse * var_vds_dn2)), ((var_idse_dn4 * var_vds) + (var_idse * var_vds_dn4)), ((var_idse_dn5 * var_vds) + (var_idse * var_vds_dn5)), ((var_idse_dn6 * var_vds) + (var_idse * var_vds_dn6)), ((var_idse_dn8 * var_vds) + (var_idse * var_vds_dn8)), ((var_idse_dn10 * var_vds) + (var_idse * var_vds_dn10)), ((var_idse_dn11 * var_vds) + (var_idse * var_vds_dn11)), ((var_idse_dn12 * var_vds) + (var_idse * var_vds_dn12)),)
    } else {
        (var_rpower, var_rpower_dn0, var_rpower_dn2, var_rpower_dn4, var_rpower_dn5, var_rpower_dn6, var_rpower_dn8, var_rpower_dn10, var_rpower_dn11, var_rpower_dn12,)
    }
};
        var_rpower = assign23900_e28811;
        var_rpower_dn0 = assign23900_e28811_d_n0;
        var_rpower_dn2 = assign23900_e28811_d_n2;
        var_rpower_dn4 = assign23900_e28811_d_n4;
        var_rpower_dn5 = assign23900_e28811_d_n5;
        var_rpower_dn6 = assign23900_e28811_d_n6;
        var_rpower_dn8 = assign23900_e28811_d_n8;
        var_rpower_dn10 = assign23900_e28811_d_n10;
        var_rpower_dn11 = assign23900_e28811_d_n11;
        var_rpower_dn12 = assign23900_e28811_d_n12;

        let (assign23910_e28815, assign23910_e28815_d_n0, assign23910_e28815_d_n2, assign23910_e28815_d_n4, assign23910_e28815_d_n5, assign23910_e28815_d_n6, assign23910_e28815_d_n8, assign23910_e28815_d_n10, assign23910_e28815_d_n11, assign23910_e28815_d_n12,) = {
    if (var_guard437 != 0.0) {
        (var_cth, var_cth_dn0, var_cth_dn2, var_cth_dn4, var_cth_dn5, var_cth_dn6, var_cth_dn8, var_cth_dn10, var_cth_dn11, var_cth_dn12,)
    } else {
        (var_cthe, var_cthe_dn0, var_cthe_dn2, var_cthe_dn4, var_cthe_dn5, var_cthe_dn6, var_cthe_dn8, var_cthe_dn10, var_cthe_dn11, var_cthe_dn12,)
    }
};
        var_cthe = assign23910_e28815;
        var_cthe_dn0 = assign23910_e28815_d_n0;
        var_cthe_dn2 = assign23910_e28815_d_n2;
        var_cthe_dn4 = assign23910_e28815_d_n4;
        var_cthe_dn5 = assign23910_e28815_d_n5;
        var_cthe_dn6 = assign23910_e28815_d_n6;
        var_cthe_dn8 = assign23910_e28815_d_n8;
        var_cthe_dn10 = assign23910_e28815_d_n10;
        var_cthe_dn11 = assign23910_e28815_d_n11;
        var_cthe_dn12 = assign23910_e28815_d_n12;

        let (assign23920_e28821, assign23920_e28821_d_n0, assign23920_e28821_d_n2, assign23920_e28821_d_n4, assign23920_e28821_d_n5, assign23920_e28821_d_n6, assign23920_e28821_d_n8, assign23920_e28821_d_n10, assign23920_e28821_d_n11, assign23920_e28821_d_n12,) = {
    if (var_guard437 != 0.0) {
        let assign23920_e28819: f64 = (1.0 / var_rth);
        (assign23920_e28819, (-(var_rth_dn0 / (var_rth * var_rth))), (-(var_rth_dn2 / (var_rth * var_rth))), (-(var_rth_dn4 / (var_rth * var_rth))), (-(var_rth_dn5 / (var_rth * var_rth))), (-(var_rth_dn6 / (var_rth * var_rth))), (-(var_rth_dn8 / (var_rth * var_rth))), (-(var_rth_dn10 / (var_rth * var_rth))), (-(var_rth_dn11 / (var_rth * var_rth))), (-(var_rth_dn12 / (var_rth * var_rth))),)
    } else {
        (var_gth, var_gth_dn0, var_gth_dn2, var_gth_dn4, var_gth_dn5, var_gth_dn6, var_gth_dn8, var_gth_dn10, var_gth_dn11, var_gth_dn12,)
    }
};
        var_gth = assign23920_e28821;
        var_gth_dn0 = assign23920_e28821_d_n0;
        var_gth_dn2 = assign23920_e28821_d_n2;
        var_gth_dn4 = assign23920_e28821_d_n4;
        var_gth_dn5 = assign23920_e28821_d_n5;
        var_gth_dn6 = assign23920_e28821_d_n6;
        var_gth_dn8 = assign23920_e28821_d_n8;
        var_gth_dn10 = assign23920_e28821_d_n10;
        var_gth_dn11 = assign23920_e28821_d_n11;
        var_gth_dn12 = assign23920_e28821_d_n12;

        *var_cthe_slot = var_cthe;
        *var_cthe_dn0_slot = var_cthe_dn0;
        *var_cthe_dn10_slot = var_cthe_dn10;
        *var_cthe_dn11_slot = var_cthe_dn11;
        *var_cthe_dn12_slot = var_cthe_dn12;
        *var_cthe_dn2_slot = var_cthe_dn2;
        *var_cthe_dn4_slot = var_cthe_dn4;
        *var_cthe_dn5_slot = var_cthe_dn5;
        *var_cthe_dn6_slot = var_cthe_dn6;
        *var_cthe_dn8_slot = var_cthe_dn8;
        *var_gth_slot = var_gth;
        *var_gth_dn0_slot = var_gth_dn0;
        *var_gth_dn10_slot = var_gth_dn10;
        *var_gth_dn11_slot = var_gth_dn11;
        *var_gth_dn12_slot = var_gth_dn12;
        *var_gth_dn2_slot = var_gth_dn2;
        *var_gth_dn4_slot = var_gth_dn4;
        *var_gth_dn5_slot = var_gth_dn5;
        *var_gth_dn6_slot = var_gth_dn6;
        *var_gth_dn8_slot = var_gth_dn8;
        *var_guard436_slot = var_guard436;
        *var_guard437_slot = var_guard437;
        *var_ids_slot = var_ids;
        *var_ids_dn0_slot = var_ids_dn0;
        *var_ids_dn10_slot = var_ids_dn10;
        *var_ids_dn11_slot = var_ids_dn11;
        *var_ids_dn12_slot = var_ids_dn12;
        *var_ids_dn2_slot = var_ids_dn2;
        *var_ids_dn4_slot = var_ids_dn4;
        *var_ids_dn5_slot = var_ids_dn5;
        *var_ids_dn6_slot = var_ids_dn6;
        *var_ids_dn8_slot = var_ids_dn8;
        *var_iqb_nqs_slot = var_iqb_nqs;
        *var_iqb_nqs_dn0_slot = var_iqb_nqs_dn0;
        *var_iqb_nqs_dn10_slot = var_iqb_nqs_dn10;
        *var_iqb_nqs_dn11_slot = var_iqb_nqs_dn11;
        *var_iqb_nqs_dn12_slot = var_iqb_nqs_dn12;
        *var_iqb_nqs_dn2_slot = var_iqb_nqs_dn2;
        *var_iqb_nqs_dn4_slot = var_iqb_nqs_dn4;
        *var_iqb_nqs_dn5_slot = var_iqb_nqs_dn5;
        *var_iqb_nqs_dn6_slot = var_iqb_nqs_dn6;
        *var_iqb_nqs_dn8_slot = var_iqb_nqs_dn8;
        *var_iqb_nqs_dn9_slot = var_iqb_nqs_dn9;
        *var_isub_slot = var_isub;
        *var_isub_dn0_slot = var_isub_dn0;
        *var_isub_dn10_slot = var_isub_dn10;
        *var_isub_dn11_slot = var_isub_dn11;
        *var_isub_dn12_slot = var_isub_dn12;
        *var_isub_dn2_slot = var_isub_dn2;
        *var_isub_dn4_slot = var_isub_dn4;
        *var_isub_dn5_slot = var_isub_dn5;
        *var_isub_dn6_slot = var_isub_dn6;
        *var_isub_dn8_slot = var_isub_dn8;
        *var_isubs_slot = var_isubs;
        *var_isubs_dn0_slot = var_isubs_dn0;
        *var_isubs_dn10_slot = var_isubs_dn10;
        *var_isubs_dn11_slot = var_isubs_dn11;
        *var_isubs_dn12_slot = var_isubs_dn12;
        *var_isubs_dn2_slot = var_isubs_dn2;
        *var_isubs_dn4_slot = var_isubs_dn4;
        *var_isubs_dn5_slot = var_isubs_dn5;
        *var_isubs_dn6_slot = var_isubs_dn6;
        *var_isubs_dn8_slot = var_isubs_dn8;
        *var_qb_slot = var_qb;
        *var_qb_dn0_slot = var_qb_dn0;
        *var_qb_dn10_slot = var_qb_dn10;
        *var_qb_dn11_slot = var_qb_dn11;
        *var_qb_dn12_slot = var_qb_dn12;
        *var_qb_dn2_slot = var_qb_dn2;
        *var_qb_dn4_slot = var_qb_dn4;
        *var_qb_dn5_slot = var_qb_dn5;
        *var_qb_dn6_slot = var_qb_dn6;
        *var_qb_dn8_slot = var_qb_dn8;
        *var_qb_nqs_slot = var_qb_nqs;
        *var_qb_nqs_dn9_slot = var_qb_nqs_dn9;
        *var_qbe_slot = var_qbe;
        *var_qbe_dn0_slot = var_qbe_dn0;
        *var_qbe_dn10_slot = var_qbe_dn10;
        *var_qbe_dn11_slot = var_qbe_dn11;
        *var_qbe_dn12_slot = var_qbe_dn12;
        *var_qbe_dn2_slot = var_qbe_dn2;
        *var_qbe_dn4_slot = var_qbe_dn4;
        *var_qbe_dn5_slot = var_qbe_dn5;
        *var_qbe_dn6_slot = var_qbe_dn6;
        *var_qbe_dn8_slot = var_qbe_dn8;
        *var_qd_slot = var_qd;
        *var_qd_dn0_slot = var_qd_dn0;
        *var_qd_dn10_slot = var_qd_dn10;
        *var_qd_dn11_slot = var_qd_dn11;
        *var_qd_dn12_slot = var_qd_dn12;
        *var_qd_dn2_slot = var_qd_dn2;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qd_nqs_slot = var_qd_nqs;
        *var_qd_nqs_dn0_slot = var_qd_nqs_dn0;
        *var_qd_nqs_dn10_slot = var_qd_nqs_dn10;
        *var_qd_nqs_dn11_slot = var_qd_nqs_dn11;
        *var_qd_nqs_dn12_slot = var_qd_nqs_dn12;
        *var_qd_nqs_dn2_slot = var_qd_nqs_dn2;
        *var_qd_nqs_dn4_slot = var_qd_nqs_dn4;
        *var_qd_nqs_dn5_slot = var_qd_nqs_dn5;
        *var_qd_nqs_dn6_slot = var_qd_nqs_dn6;
        *var_qd_nqs_dn8_slot = var_qd_nqs_dn8;
        *var_qde_slot = var_qde;
        *var_qde_dn0_slot = var_qde_dn0;
        *var_qde_dn10_slot = var_qde_dn10;
        *var_qde_dn11_slot = var_qde_dn11;
        *var_qde_dn12_slot = var_qde_dn12;
        *var_qde_dn2_slot = var_qde_dn2;
        *var_qde_dn4_slot = var_qde_dn4;
        *var_qde_dn5_slot = var_qde_dn5;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qde_dn8_slot = var_qde_dn8;
        *var_qg_slot = var_qg;
        *var_qg_dn0_slot = var_qg_dn0;
        *var_qg_dn10_slot = var_qg_dn10;
        *var_qg_dn11_slot = var_qg_dn11;
        *var_qg_dn12_slot = var_qg_dn12;
        *var_qg_dn2_slot = var_qg_dn2;
        *var_qg_dn4_slot = var_qg_dn4;
        *var_qg_dn5_slot = var_qg_dn5;
        *var_qg_dn6_slot = var_qg_dn6;
        *var_qg_dn8_slot = var_qg_dn8;
        *var_qg_nqs_slot = var_qg_nqs;
        *var_qg_nqs_dn8_slot = var_qg_nqs_dn8;
        *var_qg_nqs_dn9_slot = var_qg_nqs_dn9;
        *var_qs_nqs_slot = var_qs_nqs;
        *var_qs_nqs_dn0_slot = var_qs_nqs_dn0;
        *var_qs_nqs_dn10_slot = var_qs_nqs_dn10;
        *var_qs_nqs_dn11_slot = var_qs_nqs_dn11;
        *var_qs_nqs_dn12_slot = var_qs_nqs_dn12;
        *var_qs_nqs_dn2_slot = var_qs_nqs_dn2;
        *var_qs_nqs_dn4_slot = var_qs_nqs_dn4;
        *var_qs_nqs_dn5_slot = var_qs_nqs_dn5;
        *var_qs_nqs_dn6_slot = var_qs_nqs_dn6;
        *var_qs_nqs_dn8_slot = var_qs_nqs_dn8;
        *var_qse_slot = var_qse;
        *var_qse_dn0_slot = var_qse_dn0;
        *var_qse_dn10_slot = var_qse_dn10;
        *var_qse_dn11_slot = var_qse_dn11;
        *var_qse_dn12_slot = var_qse_dn12;
        *var_qse_dn2_slot = var_qse_dn2;
        *var_qse_dn4_slot = var_qse_dn4;
        *var_qse_dn5_slot = var_qse_dn5;
        *var_qse_dn6_slot = var_qse_dn6;
        *var_qse_dn8_slot = var_qse_dn8;
        *var_rdd_slot = var_rdd;
        *var_rdd_dn0_slot = var_rdd_dn0;
        *var_rdd_dn10_slot = var_rdd_dn10;
        *var_rdd_dn11_slot = var_rdd_dn11;
        *var_rdd_dn12_slot = var_rdd_dn12;
        *var_rdd_dn2_slot = var_rdd_dn2;
        *var_rdd_dn4_slot = var_rdd_dn4;
        *var_rdd_dn5_slot = var_rdd_dn5;
        *var_rdd_dn6_slot = var_rdd_dn6;
        *var_rdd_dn8_slot = var_rdd_dn8;
        *var_rpower_slot = var_rpower;
        *var_rpower_dn0_slot = var_rpower_dn0;
        *var_rpower_dn10_slot = var_rpower_dn10;
        *var_rpower_dn11_slot = var_rpower_dn11;
        *var_rpower_dn12_slot = var_rpower_dn12;
        *var_rpower_dn2_slot = var_rpower_dn2;
        *var_rpower_dn4_slot = var_rpower_dn4;
        *var_rpower_dn5_slot = var_rpower_dn5;
        *var_rpower_dn6_slot = var_rpower_dn6;
        *var_rpower_dn8_slot = var_rpower_dn8;
        *var_rsd_slot = var_rsd;
        *var_rsd_dn0_slot = var_rsd_dn0;
        *var_rsd_dn10_slot = var_rsd_dn10;
        *var_rsd_dn11_slot = var_rsd_dn11;
        *var_rsd_dn12_slot = var_rsd_dn12;
        *var_rsd_dn2_slot = var_rsd_dn2;
        *var_rsd_dn4_slot = var_rsd_dn4;
        *var_rsd_dn5_slot = var_rsd_dn5;
        *var_rsd_dn6_slot = var_rsd_dn6;
        *var_rsd_dn8_slot = var_rsd_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
    }

    pub(super) fn stamp_transient_block_90(
        p: &Parameters,
        var_guard437: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn12: f64,
        var_ids_dn2: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn8: f64,
        var_igbe: f64,
        var_igbe_dn0: f64,
        var_igbe_dn10: f64,
        var_igbe_dn11: f64,
        var_igbe_dn12: f64,
        var_igbe_dn2: f64,
        var_igbe_dn4: f64,
        var_igbe_dn5: f64,
        var_igbe_dn6: f64,
        var_igbe_dn8: f64,
        var_igde: f64,
        var_igde_dn0: f64,
        var_igde_dn10: f64,
        var_igde_dn11: f64,
        var_igde_dn12: f64,
        var_igde_dn2: f64,
        var_igde_dn4: f64,
        var_igde_dn5: f64,
        var_igde_dn6: f64,
        var_igde_dn8: f64,
        var_igidle: f64,
        var_igidle_dn0: f64,
        var_igidle_dn10: f64,
        var_igidle_dn11: f64,
        var_igidle_dn12: f64,
        var_igidle_dn2: f64,
        var_igidle_dn4: f64,
        var_igidle_dn5: f64,
        var_igidle_dn6: f64,
        var_igidle_dn8: f64,
        var_igisle: f64,
        var_igisle_dn0: f64,
        var_igisle_dn10: f64,
        var_igisle_dn11: f64,
        var_igisle_dn12: f64,
        var_igisle_dn2: f64,
        var_igisle_dn4: f64,
        var_igisle_dn5: f64,
        var_igisle_dn6: f64,
        var_igisle_dn8: f64,
        var_igse: f64,
        var_igse_dn0: f64,
        var_igse_dn10: f64,
        var_igse_dn11: f64,
        var_igse_dn12: f64,
        var_igse_dn2: f64,
        var_igse_dn4: f64,
        var_igse_dn5: f64,
        var_igse_dn6: f64,
        var_igse_dn8: f64,
        var_qge_dn11: f64,
        var_qge_dn12: f64,
        var_cgdbd_slot: &mut f64,
        var_cgdbd_dn0_slot: &mut f64,
        var_cgdbd_dn10_slot: &mut f64,
        var_cgdbd_dn11_slot: &mut f64,
        var_cgdbd_dn12_slot: &mut f64,
        var_cgdbd_dn2_slot: &mut f64,
        var_cgdbd_dn4_slot: &mut f64,
        var_cgdbd_dn5_slot: &mut f64,
        var_cgdbd_dn6_slot: &mut f64,
        var_cgdbd_dn8_slot: &mut f64,
        var_cgsbd_slot: &mut f64,
        var_cgsbd_dn0_slot: &mut f64,
        var_cgsbd_dn10_slot: &mut f64,
        var_cgsbd_dn11_slot: &mut f64,
        var_cgsbd_dn12_slot: &mut f64,
        var_cgsbd_dn2_slot: &mut f64,
        var_cgsbd_dn4_slot: &mut f64,
        var_cgsbd_dn5_slot: &mut f64,
        var_cgsbd_dn6_slot: &mut f64,
        var_cgsbd_dn8_slot: &mut f64,
        var_cthe_slot: &mut f64,
        var_cthe_dn0_slot: &mut f64,
        var_cthe_dn10_slot: &mut f64,
        var_cthe_dn11_slot: &mut f64,
        var_cthe_dn12_slot: &mut f64,
        var_cthe_dn2_slot: &mut f64,
        var_cthe_dn4_slot: &mut f64,
        var_cthe_dn5_slot: &mut f64,
        var_cthe_dn6_slot: &mut f64,
        var_cthe_dn8_slot: &mut f64,
        var_gth_slot: &mut f64,
        var_gth_dn0_slot: &mut f64,
        var_gth_dn10_slot: &mut f64,
        var_gth_dn11_slot: &mut f64,
        var_gth_dn12_slot: &mut f64,
        var_gth_dn2_slot: &mut f64,
        var_gth_dn4_slot: &mut f64,
        var_gth_dn5_slot: &mut f64,
        var_gth_dn6_slot: &mut f64,
        var_gth_dn8_slot: &mut f64,
        var_guard443_slot: &mut f64,
        var_guard444_slot: &mut f64,
        var_idse_slot: &mut f64,
        var_idse_dn0_slot: &mut f64,
        var_idse_dn10_slot: &mut f64,
        var_idse_dn11_slot: &mut f64,
        var_idse_dn12_slot: &mut f64,
        var_idse_dn2_slot: &mut f64,
        var_idse_dn4_slot: &mut f64,
        var_idse_dn5_slot: &mut f64,
        var_idse_dn6_slot: &mut f64,
        var_idse_dn8_slot: &mut f64,
        var_igb_slot: &mut f64,
        var_igb_dn0_slot: &mut f64,
        var_igb_dn10_slot: &mut f64,
        var_igb_dn11_slot: &mut f64,
        var_igb_dn12_slot: &mut f64,
        var_igb_dn2_slot: &mut f64,
        var_igb_dn4_slot: &mut f64,
        var_igb_dn5_slot: &mut f64,
        var_igb_dn6_slot: &mut f64,
        var_igb_dn8_slot: &mut f64,
        var_igd_slot: &mut f64,
        var_igd_dn0_slot: &mut f64,
        var_igd_dn10_slot: &mut f64,
        var_igd_dn11_slot: &mut f64,
        var_igd_dn12_slot: &mut f64,
        var_igd_dn2_slot: &mut f64,
        var_igd_dn4_slot: &mut f64,
        var_igd_dn5_slot: &mut f64,
        var_igd_dn6_slot: &mut f64,
        var_igd_dn8_slot: &mut f64,
        var_igidl_slot: &mut f64,
        var_igidl_dn0_slot: &mut f64,
        var_igidl_dn10_slot: &mut f64,
        var_igidl_dn11_slot: &mut f64,
        var_igidl_dn12_slot: &mut f64,
        var_igidl_dn2_slot: &mut f64,
        var_igidl_dn4_slot: &mut f64,
        var_igidl_dn5_slot: &mut f64,
        var_igidl_dn6_slot: &mut f64,
        var_igidl_dn8_slot: &mut f64,
        var_igisl_slot: &mut f64,
        var_igisl_dn0_slot: &mut f64,
        var_igisl_dn10_slot: &mut f64,
        var_igisl_dn11_slot: &mut f64,
        var_igisl_dn12_slot: &mut f64,
        var_igisl_dn2_slot: &mut f64,
        var_igisl_dn4_slot: &mut f64,
        var_igisl_dn5_slot: &mut f64,
        var_igisl_dn6_slot: &mut f64,
        var_igisl_dn8_slot: &mut f64,
        var_igs_slot: &mut f64,
        var_igs_dn0_slot: &mut f64,
        var_igs_dn10_slot: &mut f64,
        var_igs_dn11_slot: &mut f64,
        var_igs_dn12_slot: &mut f64,
        var_igs_dn2_slot: &mut f64,
        var_igs_dn4_slot: &mut f64,
        var_igs_dn5_slot: &mut f64,
        var_igs_dn6_slot: &mut f64,
        var_igs_dn8_slot: &mut f64,
        var_rpower_slot: &mut f64,
        var_rpower_dn0_slot: &mut f64,
        var_rpower_dn10_slot: &mut f64,
        var_rpower_dn11_slot: &mut f64,
        var_rpower_dn12_slot: &mut f64,
        var_rpower_dn2_slot: &mut f64,
        var_rpower_dn4_slot: &mut f64,
        var_rpower_dn5_slot: &mut f64,
        var_rpower_dn6_slot: &mut f64,
        var_rpower_dn8_slot: &mut f64,
    ) {
        let mut var_cgdbd: f64 = *var_cgdbd_slot;
        let mut var_cgdbd_dn0: f64 = *var_cgdbd_dn0_slot;
        let mut var_cgdbd_dn10: f64 = *var_cgdbd_dn10_slot;
        let mut var_cgdbd_dn11: f64 = *var_cgdbd_dn11_slot;
        let mut var_cgdbd_dn12: f64 = *var_cgdbd_dn12_slot;
        let mut var_cgdbd_dn2: f64 = *var_cgdbd_dn2_slot;
        let mut var_cgdbd_dn4: f64 = *var_cgdbd_dn4_slot;
        let mut var_cgdbd_dn5: f64 = *var_cgdbd_dn5_slot;
        let mut var_cgdbd_dn6: f64 = *var_cgdbd_dn6_slot;
        let mut var_cgdbd_dn8: f64 = *var_cgdbd_dn8_slot;
        let mut var_cgsbd: f64 = *var_cgsbd_slot;
        let mut var_cgsbd_dn0: f64 = *var_cgsbd_dn0_slot;
        let mut var_cgsbd_dn10: f64 = *var_cgsbd_dn10_slot;
        let mut var_cgsbd_dn11: f64 = *var_cgsbd_dn11_slot;
        let mut var_cgsbd_dn12: f64 = *var_cgsbd_dn12_slot;
        let mut var_cgsbd_dn2: f64 = *var_cgsbd_dn2_slot;
        let mut var_cgsbd_dn4: f64 = *var_cgsbd_dn4_slot;
        let mut var_cgsbd_dn5: f64 = *var_cgsbd_dn5_slot;
        let mut var_cgsbd_dn6: f64 = *var_cgsbd_dn6_slot;
        let mut var_cgsbd_dn8: f64 = *var_cgsbd_dn8_slot;
        let mut var_cthe: f64 = *var_cthe_slot;
        let mut var_cthe_dn0: f64 = *var_cthe_dn0_slot;
        let mut var_cthe_dn10: f64 = *var_cthe_dn10_slot;
        let mut var_cthe_dn11: f64 = *var_cthe_dn11_slot;
        let mut var_cthe_dn12: f64 = *var_cthe_dn12_slot;
        let mut var_cthe_dn2: f64 = *var_cthe_dn2_slot;
        let mut var_cthe_dn4: f64 = *var_cthe_dn4_slot;
        let mut var_cthe_dn5: f64 = *var_cthe_dn5_slot;
        let mut var_cthe_dn6: f64 = *var_cthe_dn6_slot;
        let mut var_cthe_dn8: f64 = *var_cthe_dn8_slot;
        let mut var_gth: f64 = *var_gth_slot;
        let mut var_gth_dn0: f64 = *var_gth_dn0_slot;
        let mut var_gth_dn10: f64 = *var_gth_dn10_slot;
        let mut var_gth_dn11: f64 = *var_gth_dn11_slot;
        let mut var_gth_dn12: f64 = *var_gth_dn12_slot;
        let mut var_gth_dn2: f64 = *var_gth_dn2_slot;
        let mut var_gth_dn4: f64 = *var_gth_dn4_slot;
        let mut var_gth_dn5: f64 = *var_gth_dn5_slot;
        let mut var_gth_dn6: f64 = *var_gth_dn6_slot;
        let mut var_gth_dn8: f64 = *var_gth_dn8_slot;
        let mut var_guard443: f64 = *var_guard443_slot;
        let mut var_guard444: f64 = *var_guard444_slot;
        let mut var_idse: f64 = *var_idse_slot;
        let mut var_idse_dn0: f64 = *var_idse_dn0_slot;
        let mut var_idse_dn10: f64 = *var_idse_dn10_slot;
        let mut var_idse_dn11: f64 = *var_idse_dn11_slot;
        let mut var_idse_dn12: f64 = *var_idse_dn12_slot;
        let mut var_idse_dn2: f64 = *var_idse_dn2_slot;
        let mut var_idse_dn4: f64 = *var_idse_dn4_slot;
        let mut var_idse_dn5: f64 = *var_idse_dn5_slot;
        let mut var_idse_dn6: f64 = *var_idse_dn6_slot;
        let mut var_idse_dn8: f64 = *var_idse_dn8_slot;
        let mut var_igb: f64 = *var_igb_slot;
        let mut var_igb_dn0: f64 = *var_igb_dn0_slot;
        let mut var_igb_dn10: f64 = *var_igb_dn10_slot;
        let mut var_igb_dn11: f64 = *var_igb_dn11_slot;
        let mut var_igb_dn12: f64 = *var_igb_dn12_slot;
        let mut var_igb_dn2: f64 = *var_igb_dn2_slot;
        let mut var_igb_dn4: f64 = *var_igb_dn4_slot;
        let mut var_igb_dn5: f64 = *var_igb_dn5_slot;
        let mut var_igb_dn6: f64 = *var_igb_dn6_slot;
        let mut var_igb_dn8: f64 = *var_igb_dn8_slot;
        let mut var_igd: f64 = *var_igd_slot;
        let mut var_igd_dn0: f64 = *var_igd_dn0_slot;
        let mut var_igd_dn10: f64 = *var_igd_dn10_slot;
        let mut var_igd_dn11: f64 = *var_igd_dn11_slot;
        let mut var_igd_dn12: f64 = *var_igd_dn12_slot;
        let mut var_igd_dn2: f64 = *var_igd_dn2_slot;
        let mut var_igd_dn4: f64 = *var_igd_dn4_slot;
        let mut var_igd_dn5: f64 = *var_igd_dn5_slot;
        let mut var_igd_dn6: f64 = *var_igd_dn6_slot;
        let mut var_igd_dn8: f64 = *var_igd_dn8_slot;
        let mut var_igidl: f64 = *var_igidl_slot;
        let mut var_igidl_dn0: f64 = *var_igidl_dn0_slot;
        let mut var_igidl_dn10: f64 = *var_igidl_dn10_slot;
        let mut var_igidl_dn11: f64 = *var_igidl_dn11_slot;
        let mut var_igidl_dn12: f64 = *var_igidl_dn12_slot;
        let mut var_igidl_dn2: f64 = *var_igidl_dn2_slot;
        let mut var_igidl_dn4: f64 = *var_igidl_dn4_slot;
        let mut var_igidl_dn5: f64 = *var_igidl_dn5_slot;
        let mut var_igidl_dn6: f64 = *var_igidl_dn6_slot;
        let mut var_igidl_dn8: f64 = *var_igidl_dn8_slot;
        let mut var_igisl: f64 = *var_igisl_slot;
        let mut var_igisl_dn0: f64 = *var_igisl_dn0_slot;
        let mut var_igisl_dn10: f64 = *var_igisl_dn10_slot;
        let mut var_igisl_dn11: f64 = *var_igisl_dn11_slot;
        let mut var_igisl_dn12: f64 = *var_igisl_dn12_slot;
        let mut var_igisl_dn2: f64 = *var_igisl_dn2_slot;
        let mut var_igisl_dn4: f64 = *var_igisl_dn4_slot;
        let mut var_igisl_dn5: f64 = *var_igisl_dn5_slot;
        let mut var_igisl_dn6: f64 = *var_igisl_dn6_slot;
        let mut var_igisl_dn8: f64 = *var_igisl_dn8_slot;
        let mut var_igs: f64 = *var_igs_slot;
        let mut var_igs_dn0: f64 = *var_igs_dn0_slot;
        let mut var_igs_dn10: f64 = *var_igs_dn10_slot;
        let mut var_igs_dn11: f64 = *var_igs_dn11_slot;
        let mut var_igs_dn12: f64 = *var_igs_dn12_slot;
        let mut var_igs_dn2: f64 = *var_igs_dn2_slot;
        let mut var_igs_dn4: f64 = *var_igs_dn4_slot;
        let mut var_igs_dn5: f64 = *var_igs_dn5_slot;
        let mut var_igs_dn6: f64 = *var_igs_dn6_slot;
        let mut var_igs_dn8: f64 = *var_igs_dn8_slot;
        let mut var_rpower: f64 = *var_rpower_slot;
        let mut var_rpower_dn0: f64 = *var_rpower_dn0_slot;
        let mut var_rpower_dn10: f64 = *var_rpower_dn10_slot;
        let mut var_rpower_dn11: f64 = *var_rpower_dn11_slot;
        let mut var_rpower_dn12: f64 = *var_rpower_dn12_slot;
        let mut var_rpower_dn2: f64 = *var_rpower_dn2_slot;
        let mut var_rpower_dn4: f64 = *var_rpower_dn4_slot;
        let mut var_rpower_dn5: f64 = *var_rpower_dn5_slot;
        let mut var_rpower_dn6: f64 = *var_rpower_dn6_slot;
        let mut var_rpower_dn8: f64 = *var_rpower_dn8_slot;

        let (assign23930_e28826, assign23930_e28826_d_n0, assign23930_e28826_d_n2, assign23930_e28826_d_n4, assign23930_e28826_d_n5, assign23930_e28826_d_n6, assign23930_e28826_d_n8, assign23930_e28826_d_n10, assign23930_e28826_d_n11, assign23930_e28826_d_n12,) = {
    if (var_guard437 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rpower, var_rpower_dn0, var_rpower_dn2, var_rpower_dn4, var_rpower_dn5, var_rpower_dn6, var_rpower_dn8, var_rpower_dn10, var_rpower_dn11, var_rpower_dn12,)
    }
};
        var_rpower = assign23930_e28826;
        var_rpower_dn0 = assign23930_e28826_d_n0;
        var_rpower_dn2 = assign23930_e28826_d_n2;
        var_rpower_dn4 = assign23930_e28826_d_n4;
        var_rpower_dn5 = assign23930_e28826_d_n5;
        var_rpower_dn6 = assign23930_e28826_d_n6;
        var_rpower_dn8 = assign23930_e28826_d_n8;
        var_rpower_dn10 = assign23930_e28826_d_n10;
        var_rpower_dn11 = assign23930_e28826_d_n11;
        var_rpower_dn12 = assign23930_e28826_d_n12;

        let (assign23940_e28831, assign23940_e28831_d_n0, assign23940_e28831_d_n2, assign23940_e28831_d_n4, assign23940_e28831_d_n5, assign23940_e28831_d_n6, assign23940_e28831_d_n8, assign23940_e28831_d_n10, assign23940_e28831_d_n11, assign23940_e28831_d_n12,) = {
    if (var_guard437 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cthe, var_cthe_dn0, var_cthe_dn2, var_cthe_dn4, var_cthe_dn5, var_cthe_dn6, var_cthe_dn8, var_cthe_dn10, var_cthe_dn11, var_cthe_dn12,)
    }
};
        var_cthe = assign23940_e28831;
        var_cthe_dn0 = assign23940_e28831_d_n0;
        var_cthe_dn2 = assign23940_e28831_d_n2;
        var_cthe_dn4 = assign23940_e28831_d_n4;
        var_cthe_dn5 = assign23940_e28831_d_n5;
        var_cthe_dn6 = assign23940_e28831_d_n6;
        var_cthe_dn8 = assign23940_e28831_d_n8;
        var_cthe_dn10 = assign23940_e28831_d_n10;
        var_cthe_dn11 = assign23940_e28831_d_n11;
        var_cthe_dn12 = assign23940_e28831_d_n12;

        let (assign23950_e28836, assign23950_e28836_d_n0, assign23950_e28836_d_n2, assign23950_e28836_d_n4, assign23950_e28836_d_n5, assign23950_e28836_d_n6, assign23950_e28836_d_n8, assign23950_e28836_d_n10, assign23950_e28836_d_n11, assign23950_e28836_d_n12,) = {
    if (var_guard437 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gth, var_gth_dn0, var_gth_dn2, var_gth_dn4, var_gth_dn5, var_gth_dn6, var_gth_dn8, var_gth_dn10, var_gth_dn11, var_gth_dn12,)
    }
};
        var_gth = assign23950_e28836;
        var_gth_dn0 = assign23950_e28836_d_n0;
        var_gth_dn2 = assign23950_e28836_d_n2;
        var_gth_dn4 = assign23950_e28836_d_n4;
        var_gth_dn5 = assign23950_e28836_d_n5;
        var_gth_dn6 = assign23950_e28836_d_n6;
        var_gth_dn8 = assign23950_e28836_d_n8;
        var_gth_dn10 = assign23950_e28836_d_n10;
        var_gth_dn11 = assign23950_e28836_d_n11;
        var_gth_dn12 = assign23950_e28836_d_n12;

        var_igd = var_igde;
        var_igd_dn0 = var_igde_dn0;
        var_igd_dn2 = var_igde_dn2;
        var_igd_dn4 = var_igde_dn4;
        var_igd_dn5 = var_igde_dn5;
        var_igd_dn6 = var_igde_dn6;
        var_igd_dn8 = var_igde_dn8;
        var_igd_dn10 = var_igde_dn10;
        var_igd_dn11 = var_igde_dn11;
        var_igd_dn12 = var_igde_dn12;

        var_igs = var_igse;
        var_igs_dn0 = var_igse_dn0;
        var_igs_dn2 = var_igse_dn2;
        var_igs_dn4 = var_igse_dn4;
        var_igs_dn5 = var_igse_dn5;
        var_igs_dn6 = var_igse_dn6;
        var_igs_dn8 = var_igse_dn8;
        var_igs_dn10 = var_igse_dn10;
        var_igs_dn11 = var_igse_dn11;
        var_igs_dn12 = var_igse_dn12;

        var_igb = var_igbe;
        var_igb_dn0 = var_igbe_dn0;
        var_igb_dn2 = var_igbe_dn2;
        var_igb_dn4 = var_igbe_dn4;
        var_igb_dn5 = var_igbe_dn5;
        var_igb_dn6 = var_igbe_dn6;
        var_igb_dn8 = var_igbe_dn8;
        var_igb_dn10 = var_igbe_dn10;
        var_igb_dn11 = var_igbe_dn11;
        var_igb_dn12 = var_igbe_dn12;

        var_igidl = var_igidle;
        var_igidl_dn0 = var_igidle_dn0;
        var_igidl_dn2 = var_igidle_dn2;
        var_igidl_dn4 = var_igidle_dn4;
        var_igidl_dn5 = var_igidle_dn5;
        var_igidl_dn6 = var_igidle_dn6;
        var_igidl_dn8 = var_igidle_dn8;
        var_igidl_dn10 = var_igidle_dn10;
        var_igidl_dn11 = var_igidle_dn11;
        var_igidl_dn12 = var_igidle_dn12;

        var_igisl = var_igisle;
        var_igisl_dn0 = var_igisle_dn0;
        var_igisl_dn2 = var_igisle_dn2;
        var_igisl_dn4 = var_igisle_dn4;
        var_igisl_dn5 = var_igisle_dn5;
        var_igisl_dn6 = var_igisle_dn6;
        var_igisl_dn8 = var_igisle_dn8;
        var_igisl_dn10 = var_igisle_dn10;
        var_igisl_dn11 = var_igisle_dn11;
        var_igisl_dn12 = var_igisle_dn12;

        var_idse = var_ids;
        var_idse_dn0 = var_ids_dn0;
        var_idse_dn2 = var_ids_dn2;
        var_idse_dn4 = var_ids_dn4;
        var_idse_dn5 = var_ids_dn5;
        var_idse_dn6 = var_ids_dn6;
        var_idse_dn8 = var_ids_dn8;
        var_idse_dn10 = var_ids_dn10;
        var_idse_dn11 = var_ids_dn11;
        var_idse_dn12 = var_ids_dn12;

        let assign24160_e28890: f64 = var_qge_dn11;
        var_cgdbd = assign24160_e28890;
        var_cgdbd_dn0 = 0.0;
        var_cgdbd_dn2 = 0.0;
        var_cgdbd_dn4 = 0.0;
        var_cgdbd_dn5 = 0.0;
        var_cgdbd_dn6 = 0.0;
        var_cgdbd_dn8 = 0.0;
        var_cgdbd_dn10 = 0.0;
        var_cgdbd_dn11 = 0.0;
        var_cgdbd_dn12 = 0.0;

        let assign24170_e28893: f64 = (p.p33 * var_cgdbd);
        var_cgdbd = assign24170_e28893;
        var_cgdbd_dn0 = (p.p33 * var_cgdbd_dn0);
        var_cgdbd_dn2 = (p.p33 * var_cgdbd_dn2);
        var_cgdbd_dn4 = (p.p33 * var_cgdbd_dn4);
        var_cgdbd_dn5 = (p.p33 * var_cgdbd_dn5);
        var_cgdbd_dn6 = (p.p33 * var_cgdbd_dn6);
        var_cgdbd_dn8 = (p.p33 * var_cgdbd_dn8);
        var_cgdbd_dn10 = (p.p33 * var_cgdbd_dn10);
        var_cgdbd_dn11 = (p.p33 * var_cgdbd_dn11);
        var_cgdbd_dn12 = (p.p33 * var_cgdbd_dn12);

        let assign24180_e28896: f64 = var_qge_dn12;
        var_cgsbd = assign24180_e28896;
        var_cgsbd_dn0 = 0.0;
        var_cgsbd_dn2 = 0.0;
        var_cgsbd_dn4 = 0.0;
        var_cgsbd_dn5 = 0.0;
        var_cgsbd_dn6 = 0.0;
        var_cgsbd_dn8 = 0.0;
        var_cgsbd_dn10 = 0.0;
        var_cgsbd_dn11 = 0.0;
        var_cgsbd_dn12 = 0.0;

        let assign24190_e28899: f64 = (p.p33 * var_cgsbd);
        var_cgsbd = assign24190_e28899;
        var_cgsbd_dn0 = (p.p33 * var_cgsbd_dn0);
        var_cgsbd_dn2 = (p.p33 * var_cgsbd_dn2);
        var_cgsbd_dn4 = (p.p33 * var_cgsbd_dn4);
        var_cgsbd_dn5 = (p.p33 * var_cgsbd_dn5);
        var_cgsbd_dn6 = (p.p33 * var_cgsbd_dn6);
        var_cgsbd_dn8 = (p.p33 * var_cgsbd_dn8);
        var_cgsbd_dn10 = (p.p33 * var_cgsbd_dn10);
        var_cgsbd_dn11 = (p.p33 * var_cgsbd_dn11);
        var_cgsbd_dn12 = (p.p33 * var_cgsbd_dn12);

        let assign24500_e28994: f64 = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };
        var_guard443 = assign24500_e28994;

        let assign24510_e28999: f64 = if (((p.p27 != 0.0) && (p.p15 != 0.0)) && (p.p16 != 0.0)) { 1.0 } else { 0.0 };
        var_guard444 = assign24510_e28999;

        *var_cgdbd_slot = var_cgdbd;
        *var_cgdbd_dn0_slot = var_cgdbd_dn0;
        *var_cgdbd_dn10_slot = var_cgdbd_dn10;
        *var_cgdbd_dn11_slot = var_cgdbd_dn11;
        *var_cgdbd_dn12_slot = var_cgdbd_dn12;
        *var_cgdbd_dn2_slot = var_cgdbd_dn2;
        *var_cgdbd_dn4_slot = var_cgdbd_dn4;
        *var_cgdbd_dn5_slot = var_cgdbd_dn5;
        *var_cgdbd_dn6_slot = var_cgdbd_dn6;
        *var_cgdbd_dn8_slot = var_cgdbd_dn8;
        *var_cgsbd_slot = var_cgsbd;
        *var_cgsbd_dn0_slot = var_cgsbd_dn0;
        *var_cgsbd_dn10_slot = var_cgsbd_dn10;
        *var_cgsbd_dn11_slot = var_cgsbd_dn11;
        *var_cgsbd_dn12_slot = var_cgsbd_dn12;
        *var_cgsbd_dn2_slot = var_cgsbd_dn2;
        *var_cgsbd_dn4_slot = var_cgsbd_dn4;
        *var_cgsbd_dn5_slot = var_cgsbd_dn5;
        *var_cgsbd_dn6_slot = var_cgsbd_dn6;
        *var_cgsbd_dn8_slot = var_cgsbd_dn8;
        *var_cthe_slot = var_cthe;
        *var_cthe_dn0_slot = var_cthe_dn0;
        *var_cthe_dn10_slot = var_cthe_dn10;
        *var_cthe_dn11_slot = var_cthe_dn11;
        *var_cthe_dn12_slot = var_cthe_dn12;
        *var_cthe_dn2_slot = var_cthe_dn2;
        *var_cthe_dn4_slot = var_cthe_dn4;
        *var_cthe_dn5_slot = var_cthe_dn5;
        *var_cthe_dn6_slot = var_cthe_dn6;
        *var_cthe_dn8_slot = var_cthe_dn8;
        *var_gth_slot = var_gth;
        *var_gth_dn0_slot = var_gth_dn0;
        *var_gth_dn10_slot = var_gth_dn10;
        *var_gth_dn11_slot = var_gth_dn11;
        *var_gth_dn12_slot = var_gth_dn12;
        *var_gth_dn2_slot = var_gth_dn2;
        *var_gth_dn4_slot = var_gth_dn4;
        *var_gth_dn5_slot = var_gth_dn5;
        *var_gth_dn6_slot = var_gth_dn6;
        *var_gth_dn8_slot = var_gth_dn8;
        *var_guard443_slot = var_guard443;
        *var_guard444_slot = var_guard444;
        *var_idse_slot = var_idse;
        *var_idse_dn0_slot = var_idse_dn0;
        *var_idse_dn10_slot = var_idse_dn10;
        *var_idse_dn11_slot = var_idse_dn11;
        *var_idse_dn12_slot = var_idse_dn12;
        *var_idse_dn2_slot = var_idse_dn2;
        *var_idse_dn4_slot = var_idse_dn4;
        *var_idse_dn5_slot = var_idse_dn5;
        *var_idse_dn6_slot = var_idse_dn6;
        *var_idse_dn8_slot = var_idse_dn8;
        *var_igb_slot = var_igb;
        *var_igb_dn0_slot = var_igb_dn0;
        *var_igb_dn10_slot = var_igb_dn10;
        *var_igb_dn11_slot = var_igb_dn11;
        *var_igb_dn12_slot = var_igb_dn12;
        *var_igb_dn2_slot = var_igb_dn2;
        *var_igb_dn4_slot = var_igb_dn4;
        *var_igb_dn5_slot = var_igb_dn5;
        *var_igb_dn6_slot = var_igb_dn6;
        *var_igb_dn8_slot = var_igb_dn8;
        *var_igd_slot = var_igd;
        *var_igd_dn0_slot = var_igd_dn0;
        *var_igd_dn10_slot = var_igd_dn10;
        *var_igd_dn11_slot = var_igd_dn11;
        *var_igd_dn12_slot = var_igd_dn12;
        *var_igd_dn2_slot = var_igd_dn2;
        *var_igd_dn4_slot = var_igd_dn4;
        *var_igd_dn5_slot = var_igd_dn5;
        *var_igd_dn6_slot = var_igd_dn6;
        *var_igd_dn8_slot = var_igd_dn8;
        *var_igidl_slot = var_igidl;
        *var_igidl_dn0_slot = var_igidl_dn0;
        *var_igidl_dn10_slot = var_igidl_dn10;
        *var_igidl_dn11_slot = var_igidl_dn11;
        *var_igidl_dn12_slot = var_igidl_dn12;
        *var_igidl_dn2_slot = var_igidl_dn2;
        *var_igidl_dn4_slot = var_igidl_dn4;
        *var_igidl_dn5_slot = var_igidl_dn5;
        *var_igidl_dn6_slot = var_igidl_dn6;
        *var_igidl_dn8_slot = var_igidl_dn8;
        *var_igisl_slot = var_igisl;
        *var_igisl_dn0_slot = var_igisl_dn0;
        *var_igisl_dn10_slot = var_igisl_dn10;
        *var_igisl_dn11_slot = var_igisl_dn11;
        *var_igisl_dn12_slot = var_igisl_dn12;
        *var_igisl_dn2_slot = var_igisl_dn2;
        *var_igisl_dn4_slot = var_igisl_dn4;
        *var_igisl_dn5_slot = var_igisl_dn5;
        *var_igisl_dn6_slot = var_igisl_dn6;
        *var_igisl_dn8_slot = var_igisl_dn8;
        *var_igs_slot = var_igs;
        *var_igs_dn0_slot = var_igs_dn0;
        *var_igs_dn10_slot = var_igs_dn10;
        *var_igs_dn11_slot = var_igs_dn11;
        *var_igs_dn12_slot = var_igs_dn12;
        *var_igs_dn2_slot = var_igs_dn2;
        *var_igs_dn4_slot = var_igs_dn4;
        *var_igs_dn5_slot = var_igs_dn5;
        *var_igs_dn6_slot = var_igs_dn6;
        *var_igs_dn8_slot = var_igs_dn8;
        *var_rpower_slot = var_rpower;
        *var_rpower_dn0_slot = var_rpower_dn0;
        *var_rpower_dn10_slot = var_rpower_dn10;
        *var_rpower_dn11_slot = var_rpower_dn11;
        *var_rpower_dn12_slot = var_rpower_dn12;
        *var_rpower_dn2_slot = var_rpower_dn2;
        *var_rpower_dn4_slot = var_rpower_dn4;
        *var_rpower_dn5_slot = var_rpower_dn5;
        *var_rpower_dn6_slot = var_rpower_dn6;
        *var_rpower_dn8_slot = var_rpower_dn8;
    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(649, 2.0);

        s.store_scalar(650, 0.1);

        s.store_scalar(651, 0.1);

        s.store_scalar(514, 0.0);

        s.store_scalar(574, 0.0);

        s.store_scalar(237, 1e-12);

        s.store_scalar(28, 500.0);

        s.store_scalar(29, 200.0);

        s.store_scalar(32, 0.002);

        s.store_scalar(38, p.p24);

        s.store_scalar(46, 1.0);

        s.store_scalar(36, 1.0);

        s.store_scalar(305, 0.0);

        s.store_scalar(306, 0.0);

        s.store_scalar(307, 0.0);

        s.store_scalar(308, 0.0);

        s.store_scalar(309, 0.0);

        s.store_scalar(310, 0.0);

        s.store_scalar(312, 0.0);

        s.store_scalar(314, 0.0);

        s.store_scalar(311, 0.0);

        s.store_scalar(313, 0.0);

        s.store_scalar(207, 0.0);

        s.store_scalar(209, 0.0);

        s.store_scalar(531, 0.0);

        s.store_scalar(528, 0.0);

        s.store_scalar(585, 0.0);

        s.store_scalar(588, 0.0);

        s.store_scalar(523, 0.0);

        s.store_scalar(576, 0.0);

        s.store_scalar(555, 0.0);

        s.store_scalar(556, 0.0);

        s.store_scalar(322, 0.0);

        s.store_scalar(327, 0.0);

        s.store_scalar(329, 0.0);

        s.store_scalar(330, 0.0);

        s.store_scalar(331, 0.0);

        s.store_scalar(334, 0.0);

        s.store_scalar(336, 0.0);

        s.store_scalar(337, 0.0);

        s.store_scalar(345, 0.0);

        s.store_scalar(383, 0.0);

        s.store_scalar(385, 0.5);

        s.store_scalar(441, 0.0);

        s.store_scalar(442, 0.0);

        s.store_scalar(558, 0.0);

        s.store_scalar(405, 0.0);

        s.store_scalar(406, 0.0);

        s.store_scalar(397, 0.0);

        s.store_scalar(398, 0.0);

        s.store_scalar(414, 0.0);

        s.store_scalar(34, 0.0);

        s.store_scalar(35, 0.0);

        s.store_scalar(292, 0.0);

        s.store_scalar(16, 0.0);

        s.store_scalar(60, 0.0);

        s.store_scalar(58, 0.0);

        s.store_scalar(74, 1.0);

        s.store_scalar(85, 0.0);

        s.store_scalar(91, 0.0);

        s.store_scalar(93, 0.0);

        s.store_scalar(94, 0.0);

        s.store_scalar(151, 0.0);

        s.store_scalar(158, 0.0);

        s.store_scalar(159, 0.0);

        s.store_scalar(160, 0.0);

        s.store_scalar(185, 0.0);

        s.store_scalar(189, 1.0);

        s.store_scalar(193, 0.0);

        s.store_scalar(196, 0.0);

        s.store_scalar(197, 0.0);

        s.store_scalar(221, 0.0);

        s.store_scalar(222, 0.0);

        s.store_scalar(146, 0.0);

        s.store_scalar(260, 0.0);

        s.store_scalar(89, 0.0);

        s.store_scalar(230, 0.0);

        s.store_scalar(231, 0.0);

        s.store_scalar(233, 0.0);

        s.store_scalar(234, 0.0);

        s.store_scalar(235, 0.0);

        s.store_scalar(236, 0.0);

        s.store_scalar(55, 0.0);

        s.store_scalar(77, 0.0);

        s.store_scalar(339, 0.0);

        s.store_scalar(388, 0.0);

        s.store_scalar(316, 0.0);

        s.b[517] = param_given[172];
        s.store_scalar(517, if s.b[517] { 1.0 } else { 0.0 });

        s.b[518] = param_given[173];
        s.store_scalar(518, if s.b[518] { 1.0 } else { 0.0 });

        s.b[519] = param_given[174];
        s.store_scalar(519, if s.b[519] { 1.0 } else { 0.0 });

        s.b[463] = param_given[9];
        s.store_scalar(463, if s.b[463] { 1.0 } else { 0.0 });

        s.store_scalar(394, 1.0);

        s.store_scalar(446, (if param_given[177] { p.p177 } else { (5000000000.0 / (p.p227 * p.p230)) }));

        s.b[660] = ((s.v[446] < (2.0 + 0.1)) && (0.1 >= 0.0));
        s.store_scalar(660, if s.b[660] { 1.0 } else { 0.0 });

        if s.b[660] {
            s.store_scalar(638, ((2.0 + 0.1) - s.v[446]));
            s.store_square(642, 638);
            s.store_scalar(643, (0.1 * 0.1));
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[661] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(661, if s.b[661] { 1.0 } else { 0.0 });

        s.b[662] = (2.0 == 1.0);
        s.store_scalar(662, if s.b[662] { 1.0 } else { 0.0 });

        if ((s.b[660] && s.b[661]) && s.b[662]) {
            s.store_scalar(648, 1.0);
        }

        s.b[663] = (2.0 == 2.0);
        s.store_scalar(663, if s.b[663] { 1.0 } else { 0.0 });

        if (((s.b[660] && s.b[661]) && (!s.b[662])) && s.b[663]) {
            s.store_scalar(648, 2.0);
        }

        s.b[664] = (2.0 == 4.0);
        s.store_scalar(664, if s.b[664] { 1.0 } else { 0.0 });

        if ((((s.b[660] && s.b[661]) && (!s.b[662])) && (!s.b[663])) && s.b[664]) {
            s.store_scalar(648, 3.0);
        }

        s.b[665] = (2.0 == 8.0);
        s.store_scalar(665, if s.b[665] { 1.0 } else { 0.0 });

        if (((((s.b[660] && s.b[661]) && (!s.b[662])) && (!s.b[663])) && (!s.b[664])) && s.b[665]) {
            s.store_scalar(648, 4.0);
        }

        if (s.b[660] && s.b[661]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign1360_loop_guard: usize = 0;
        while {
            let assign1360_cond_e892: f64 = if ((s.b[660] && s.b[661]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign1360_cond_e892 != 0.0
        } {
            assign1360_loop_guard += 1;
            assert!(assign1360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[660] && s.b[661]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if (s.b[660] && (!s.b[661])) {
            s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));
        }

        if s.b[660] {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_scaled_mul(637, 638, 646, 0.1);
            s.store_div_scaled_product_offset_denominator(278, s.ad_value(645), s.ad_value(646), 0.1, s.ad_value(220), 1e-50, 1.0);
            s.store_sub_from_scalar(446, (2.0 + 0.1), 637);
        }

        if s.b[660] {
        }

        if (!s.b[660]) {
        }

        if (!s.b[660]) {
            s.store_scalar(278, 1.0);
        }

        s.store_scalar(613, (p.p34 * 0.01));

        s.store_scalar(614, (p.p59 / 1e-6));

        s.store_scalar(615, (p.p101 * 0.01));

        s.store_scalar(616, (p.p192 / 1e-6));

        s.store_scalar(617, (p.p219 * 0.01));

        s.store_scalar(619, (p.p220 / 0.0001));

        s.store_scalar(620, (p.p230 / 1e-6));

        s.store_scalar(621, (p.p231 / 1e-6));

        s.store_scalar(622, (p.p237 * 0.01));

        s.store_scalar(623, (p.p238 / 0.01));

        s.store_scalar(624, (p.p40 / 1e-6));

        s.store_scalar(625, (p.p236 / 1e-6));

        s.store_scalar(627, (p.p197 / 0.01));

        s.store_scalar(630, (p.p306 / 1e-6));

        s.store_scalar(631, (p.p307 / 1e-6));

        s.store_scalar(626, (p.p189 * 10000.0));

        s.store_scalar(452, (p.p147 / 1e-6));

        s.store_scalar(628, (p.p196 / 10.0));

        s.store_scalar(445, (p.p222 + 273.15));

        s.store_scalar(447, (p.p9 + 273.15));

        s.store_scalar(509, p.p41);

        s.store_scalar(510, p.p42);

        s.store_scalar(277, p.p0);

        s.store_scalar(456, (p.p1 / p.p5));

        s.store_scalar(375, (s.v[277] * 1000000.0));

        s.store_scalar(376, (s.v[456] * 1000000.0));

        s.store_scalar(377, (s.v[376] * s.v[375]));

        s.store_scalar(279, (p.p62 / ((s.v[377]) as f64).powf(p.p63)));

        s.store_scalar(133, (s.v[277] + s.v[279]));

        s.store_scalar(134, (s.v[456] + s.v[279]));

        s.store_scalar(482, (p.p64 / ((s.v[377]) as f64).powf(p.p65)));

        s.store_scalar(279, (1.0 + (p.p148 / (((s.v[133] * 1000000.0)) as f64).powf(p.p149))));

        s.store_scalar(280, (1.0 + (p.p150 / (((s.v[134] * 1000000.0)) as f64).powf(p.p151))));

        s.store_scalar(452, ((s.v[452] * s.v[279]) * s.v[280]));

        s.store_scalar(279, (1.0 + (p.p154 / (((s.v[133] * 1000000.0)) as f64).powf(p.p155))));

        s.store_scalar(280, (1.0 + (p.p156 / (((s.v[134] * 1000000.0)) as f64).powf(p.p157))));

        s.store_scalar(453, ((p.p152 * s.v[279]) * s.v[280]));

        s.store_scalar(511, ((2.0 * s.v[453]) * p.p153));

        s.store_scalar(124, ((s.v[456] - (2.0 * s.v[509])) - s.v[511]));

        s.store_scalar(512, ((s.v[456] - (2.0 * s.v[510])) - s.v[511]));

        s.store_scalar(466, (s.v[124] * p.p5));

        s.store_scalar(513, (s.v[512] * p.p5));

        s.store_scalar(467, (s.v[622] / (s.v[394] * s.v[466])));

        s.store_scalar(468, (s.v[623] * (s.v[394] * s.v[513])));

        s.store_scalar(278, (s.v[630] * ((p.p11 + (p.p304 * p.p12)) + (p.p305 * p.p13))));

        s.store_scalar(620, (s.v[620] + s.v[278]));

        s.store_scalar(638, ((s.v[620] - (1000000000000000.0 / 1e-6)) - (0.01 / 1e-6)));

        s.store_scalar(639, ((4.0 * (1000000000000000.0 / 1e-6)) * (0.01 / 1e-6)));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));

        s.store_offset_scaled(620, 639, 0.5, ((((s.v[638]) * (0.5))) + ((1000000000000000.0 / 1e-6))));

        s.store_scalar(278, (s.v[631] * ((p.p11 + (p.p304 * p.p12)) + (p.p305 * p.p13))));

        s.store_scalar(614, (s.v[614] + s.v[278]));

        s.store_scalar(638, ((s.v[614] - (1000000000000000.0 / 1e-6)) - (0.01 / 1e-6)));

        s.store_scalar(639, ((4.0 * (1000000000000000.0 / 1e-6)) * (0.01 / 1e-6)));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));

        s.store_offset_scaled(614, 639, 0.5, ((((s.v[638]) * (0.5))) + ((1000000000000000.0 / 1e-6))));

        s.store_scalar(448, ((p.p86 * ((s.v[375]) as f64).powf(p.p88)) * (1.0 + (p.p90 / ((s.v[375]) as f64).powf(p.p91)))));

        s.store_scalar(449, ((p.p87 * ((s.v[375]) as f64).powf(p.p89)) * (1.0 + (p.p92 / ((s.v[375]) as f64).powf(p.p93)))));

        s.store_scalar(450, ((p.p289 * ((s.v[375]) as f64).powf(p.p291)) * (1.0 + (p.p293 / ((s.v[375]) as f64).powf(p.p294)))));

        s.store_scalar(451, ((p.p290 * ((s.v[375]) as f64).powf(p.p292)) * (1.0 + (p.p295 / ((s.v[375]) as f64).powf(p.p296)))));

        s.store_scalar(470, ((p.p106 * (1.0 + (p.p107 / ((s.v[375]) as f64).powf(p.p110)))) * (1.0 + (p.p108 / ((s.v[376]) as f64).powf(p.p109)))));

        s.store_scalar(594, ((p.p283 * (1.0 + (p.p285 / ((s.v[375]) as f64).powf(p.p286)))) * (1.0 + (p.p287 / ((s.v[376]) as f64).powf(p.p288)))));

        s.store_scalar(279, (s.v[621] * (1.0 + (p.p232 / ((s.v[375]) as f64).powf(p.p233)))));

        s.store_scalar(638, ((s.v[279] - s.v[625]) - (s.v[621] * 0.001)));

        s.store_scalar(639, ((4.0 * s.v[625]) * (s.v[621] * 0.001)));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));

        s.store_offset_scaled(462, 639, 0.5, ((((s.v[638]) * (0.5))) + (s.v[625])));

        if (p.p32 != 0.0) {
            s.store_scale(279, 462, (1.0 + (p.p234 / ((s.v[376]) as f64).powf(p.p235))));
            s.store_offset(638, 279, (((-s.v[625])) + ((-(s.v[621] * 0.001)))));
            s.store_scalar(639, ((4.0 * s.v[625]) * (s.v[621] * 0.001)));
        }

        if (p.p32 != 0.0) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (p.p32 != 0.0) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_add_scaled_inputs_indices(462, 638, 0.5, 639, 0.5, s.v[625]);
        }

        s.store_scale(460, 614, (1.0 + (p.p60 / ((s.v[376]) as f64).powf(p.p61))));

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        s.copy_ad(461, 460);

        s.store_scalar(279, ((1.0 / (p.p43 + (0.5 * p.p0))) + (1.0 / (p.p44 + (0.5 * p.p0)))));

        s.store_scalar(459, (2.0 / s.v[279]));

        s.b[666] = (((p.p6 > 0.0) && (p.p7 > 0.0)) && ((p.p5 == 1.0) || ((p.p5 > 1.0) && (p.p8 > 0.0))));
        s.store_scalar(666, if s.b[666] { 1.0 } else { 0.0 });

        if s.b[666] {
            s.store_scalar(279, 0.0);
            s.store_scalar(514, 0.0);
        }

        let mut assign2290_loop_guard: usize = 0;
        while {
            let assign2290_cond_e1503: f64 = if (s.b[666] && (s.v[514] < p.p5)) { 1.0 } else { 0.0 };
            assign2290_cond_e1503 != 0.0
        } {
            assign2290_loop_guard += 1;
            assert!(assign2290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[666] {
                s.store_add_scaled_inputs3_mixed_iaa(279, 279, 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(514), (p.p8 + p.p0), (p.p6 + (0.5 * p.p0)))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(514), (p.p8 + p.p0), (p.p7 + (0.5 * p.p0)))), 1.0);
                s.store_offset(514, 514, 1.0);
            }
        }

        if s.b[666] {
            s.store_div_from_scalar(458, (2.0 * p.p5), 279);
        }

        if (!s.b[666]) {
            s.store_scalar(458, 0.0);
        }

        s.b[667] = (s.v[458] > 0.0);
        s.store_scalar(667, if s.b[667] { 1.0 } else { 0.0 });

        if s.b[667] {
            s.store_scalar(279, (1.0 / (1.0 + p.p166)));
            s.store_scalar(280, 0.0);
            s.store_scalar(281, 0.0);
            s.store_div_scaled_product_offset_denominator(461, s.ad_value(460), A::offset(A::mul(s.ad_value(279), s.ad_value(280)), 1.0), 1.0, A::mul(s.ad_value(279), s.ad_value(281)), 1.0, 1.0);
            s.store_scalar(279, (1.0 / (1.0 + p.p169)));
            s.store_powf_ad(280, A::div_from_scalar(p.p168, s.ad_value(458)), p.p170);
            s.store_scalar(281, (((p.p168 / s.v[459])) as f64).powf(p.p170));
            s.store_div_scaled_product_offset_denominator(620, s.ad_value(620), A::offset(A::mul(s.ad_value(279), s.ad_value(280)), 1.0), 1.0, A::mul(s.ad_value(279), s.ad_value(281)), 1.0, 1.0);
        }

        if (!s.b[667]) {
            s.copy_ad(461, 460);
        }

        s.store_scalar(280, (1.0 + (p.p190 / ((s.v[376]) as f64).powf(p.p191))));

        s.store_div_from_scalar(281, s.v[616], 620);

        s.store_offset(638, 281, (((-s.v[280])) + ((-0.01))));

        s.store_scale(639, 281, (4.0 * 0.01));

        if (!(s.v[639] > 0.0)) {
            s.store_neg(639, 639);
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_add_scaled_inputs3_indices(279, 281, 1.0, 638, (-0.5), 639, (-0.5));

        s.store_mul(471, 620, 279);

        s.b[668] = ((s.v[277] > p.p58) || (p.p58 <= 0.0));
        s.store_scalar(668, if s.b[668] { 1.0 } else { 0.0 });

        if s.b[668] {
            s.store_add_scaled_inputs(457, 471, ((s.v[277] - p.p58) * 1.0 / (s.v[277])), 461, (p.p58 * 1.0 / (s.v[277])));
        }

        if (!s.b[668]) {
            s.store_add_scaled_inputs3_indices(457, 461, 1.0, 461, ((p.p58 - s.v[277]) * 1.0 / (p.p58)), 471, (-((p.p58 - s.v[277]) * 1.0 / (p.p58))));
        }

        s.store_scale(126, 457, 1.6021918e-19);

        s.store_scale(472, 126, 1.034943e-10);

        s.store_scale(473, 472, 2.0);

        s.store_scale(474, 462, (1.6021918e-19 * 1.034943e-10));

        s.store_scalar(475, (p.p239 * ((s.v[375]) as f64).powf((-p.p242))));

        s.store_scalar(476, (p.p243 * ((s.v[375]) as f64).powf((-p.p244))));

        s.store_scalar(477, (p.p246 * (((s.v[375] + p.p248)) as f64).powf((-p.p247))));

        s.b[669] = ((s.v[277] <= (2.0 * p.p58)) && (p.p58 > 0.0));
        s.store_scalar(669, if s.b[669] { 1.0 } else { 0.0 });

        if s.b[669] {
            s.store_add_scaled_inputs4_indices(560, 461, 2.0, 461, (-(s.v[277] * 1.0 / (p.p58))), 471, (-(-(s.v[277] * 1.0 / (p.p58)))), 471, -1.0);
            s.store_ln_div(478, 560, 471);
        }

        if (!s.b[669]) {
            s.store_scalar(478, 0.0);
        }

        s.store_scaled_ln_scaled_input(129, 457, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.store_scaled_ln_scaled_input(136, 471, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.store_scalar(479, ((((1.0 + (1.0 / s.v[375]))) as f64).powf(p.p77) * p.p75));

        s.store_scalar(279, (p.p116 * s.v[375]));

        s.store_scalar(481, ((((s.v[279] * p.p115) / (s.v[279] + p.p115)) + p.p117) + 1e-50));

        s.store_scalar(483, (1.0 + (((s.v[375]) as f64).powf(p.p179) * p.p180)));

        s.b[670] = (p.p25 == 1.0);
        s.store_scalar(670, if s.b[670] { 1.0 } else { 0.0 });

        if s.b[670] {
            s.store_scalar(279, (p.p3 + (s.v[124] / (3.0 * p.p2))));
        }

        s.store_scalar(485, (1.0 + (p.p131 / ((s.v[376]) as f64).powf(p.p132))));

        s.store_scalar(486, (p.p125 * (1.0 + (p.p126 / ((s.v[375]) as f64).powf(p.p127)))));

        s.store_scalar(487, (s.v[375] / (s.v[375] + p.p124)));

        s.store_scalar(488, (p.p118 * (1.0 + (p.p120 / ((s.v[375]) as f64).powf(p.p121)))));

        s.store_scalar(489, (p.p119 * (1.0 + (p.p122 / s.v[375]))));

        s.store_scalar(490, (((10000.0 * s.v[513]) * p.p46) / ((s.v[375]) as f64).powf(p.p47)));

        s.store_scalar(559, (p.p133 * (1.0 + (p.p134 / ((s.v[375]) as f64).powf(p.p135)))));

        s.store_scalar(491, (p.p128 * (1.0 + (p.p129 / ((s.v[375]) as f64).powf(p.p130)))));

        s.store_scalar(279, ((2.0 * 1.034943e-10) / 1.6021918e-19));

        s.store_sqrt_div_from_scalar_ad(132, s.v[279], s.ad_value(457));

        s.store_scaled_voltage(540, ctx, nodes, Some(5), Some(12), p.p33);

        s.store_scaled_voltage(541, ctx, nodes, Some(11), Some(12), p.p33);

        s.store_scaled_voltage(542, ctx, nodes, Some(6), Some(12), p.p33);

        s.store_scaled_voltage(543, ctx, nodes, Some(5), Some(2), p.p33);

        s.store_scaled_voltage(544, ctx, nodes, Some(0), Some(2), p.p33);

        s.store_scaled_voltage(545, ctx, nodes, Some(6), Some(2), p.p33);

        s.b[672] = ((p.p28 != 0.0) && (p.p237 > 0.0));
        s.store_scalar(672, if s.b[672] { 1.0 } else { 0.0 });

        if s.b[672] {
            if (nv4 > 0.0) {
                s.store_voltage(11, ctx, nodes, Some(4), None);
            } else {
                s.store_scalar(11, 0.0);
            }
        }

        if (!s.b[672]) {
            s.store_scalar(11, 0.0);
        }

        if (s.v[38] != 0.0) {
            s.store_scaled_voltage(551, ctx, nodes, Some(8), None, 1e-9);
            s.store_scaled_voltage(548, ctx, nodes, Some(9), None, 1e-9);
        }

        if (s.v[38] == 0.0) {
            s.store_scalar(551, 0.0);
            s.store_scalar(548, 0.0);
        }

        s.b[673] = (s.v[541] >= 0.0);
        s.store_scalar(673, if s.b[673] { 1.0 } else { 0.0 });

        if s.b[673] {
            s.store_scalar(575, 1.0);
            s.store_scalar(412, 1.0);
            s.store_scalar(413, 0.0);
            s.copy_ad(49, 540);
            s.copy_ad(48, 541);
            s.copy_ad(47, 542);
            s.copy_ad(42, 543);
            s.copy_ad(41, 544);
            s.copy_ad(40, 545);
        }

        if (!s.b[673]) {
            s.store_scalar(575, (-1.0));
            s.store_scalar(412, 0.0);
            s.store_scalar(413, 1.0);
            s.store_sub(49, 540, 541);
            s.store_neg(48, 541);
            s.store_sub(47, 542, 541);
            s.store_sub(42, 543, 544);
            s.store_neg(41, 544);
            s.store_sub(40, 545, 544);
        }

        s.store_scalar(374, ctx_temp);

        if s.b[463] {
            s.store_scalar(374, s.v[447]);
        }

        s.store_add_offset_lhs(374, 374, p.p10, 11);

        s.store_scalar(465, (p.p37 - (s.v[445] * (9.025e-5 + (s.v[445] * 1e-7)))));

        s.store_offset_square(279, 374, (-(s.v[445] * s.v[445])));

        s.store_sub_scaled_ad_lhs(137, A::sub_from_scalar(s.v[465], A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p35)), 279, p.p36);

        s.store_div_from_scalar_scaled_input(120, 1.6021918e-19, 374, 1.3806226e-23);

        s.store_square(121, 120);

        s.store_div_from_scalar(122, 1.0, 120);

        s.store_scalar(464, (1.6021918e-19 / (1.3806226e-23 * s.v[445])));

        s.store_scalar(676, (((p.p249 * (1.0 + (p.p95 / ((s.v[376]) as f64).powf(p.p96)))) * (1.0 + (p.p97 / ((s.v[375]) as f64).powf(p.p98)))) * (1.0 + (p.p99 / ((s.v[377]) as f64).powf(p.p100)))));

        s.store_scalar(677, (((p.p276 * (1.0 + (p.p277 / ((s.v[376]) as f64).powf(p.p278)))) * (1.0 + (p.p281 / ((s.v[375]) as f64).powf(p.p282)))) * (1.0 + (p.p279 / ((s.v[377]) as f64).powf(p.p280)))));

        s.b[681] = (s.v[458] > 0.0);
        s.store_scalar(681, if s.b[681] { 1.0 } else { 0.0 });

        if s.b[681] {
            s.store_scalar(678, (1.0 / (1.0 + p.p163)));
            s.store_powf_ad(679, A::div_from_scalar(p.p162, s.ad_value(458)), p.p164);
            s.store_scalar(680, (((p.p162 / s.v[459])) as f64).powf(p.p164));
            s.store_div_scaled_offset_numerator(676, A::mul(s.ad_value(678), s.ad_value(679)), s.v[676], s.v[676], A::offset(A::mul(s.ad_value(678), s.ad_value(680)), 1.0), 1.0);
            s.store_div_scaled_offset_numerator(677, A::mul(s.ad_value(678), s.ad_value(679)), s.v[677], s.v[677], A::offset(A::mul(s.ad_value(678), s.ad_value(680)), 1.0), 1.0);
        }

        s.store_scalar(678, (1.0 + (p.p112 / ((s.v[375]) as f64).powf(p.p113))));

        s.store_offset_ad(378, A::mul_scaled_lhs(A::scale_offset(s.ad_value(374), 1.0 / (s.v[445]), (-1.0)), p.p253, A::scale_offset(s.ad_value(374), 1.0 / (s.v[445]), (-1.0))), (p.p111 * s.v[678]));

        s.store_pow_ad(678, A::scale(s.ad_value(374), 1.0 / (s.v[445])), s.ad_value(378));

        s.store_div(469, 678, 676);

        s.store_div(595, 678, 677);

        s.store_mul(380, 478, 122);

        s.store_scalar(279, ((((1.0 + (p.p181 / ((s.v[375]) as f64).powf(p.p182))) * (1.0 + (p.p185 / ((s.v[375]) as f64).powf(p.p186)))) * (1.0 + (p.p187 / ((s.v[376]) as f64).powf(p.p188)))) * (1.0 + (p.p183 / ((s.v[377]) as f64).powf(p.p184)))));

        s.store_scalar(639, ((((s.v[279] * s.v[279]) + ((4.0 * 0.001) * 0.001))) as f64).sqrt());

        s.store_scalar(280, (0.5 * (1.0 + (s.v[279] / s.v[639]))));

        s.store_scalar(480, ((0.5 * (s.v[279] + s.v[639])) + (1e-10 * 0.001)));

        s.b[682] = (s.v[480] < 0.0);
        s.store_scalar(682, if s.b[682] { 1.0 } else { 0.0 });

        if s.b[682] {
            s.store_scalar(480, 0.0);
            s.store_scalar(280, 0.0);
        }

        s.store_scale(279, 374, 1.0 / (s.v[445]));

        s.store_scalar(280, (1.0 + (p.p102 / ((s.v[375]) as f64).powf(p.p103))));

        s.store_div_scaled_inputs_mixed_ia(162, 480, (s.v[613] * 0.01), A::sub(A::add_scaled_product(A::scale_offset(s.ad_value(279), (0.4 * 0.01), (1.8 * 0.01)), 1.0, s.ad_value(279), s.ad_value(279), (0.1 * 0.01)), A::scale_offset(s.ad_value(279), (-(s.v[615] * s.v[280])), (s.v[615] * s.v[280]))), 1.0);

        s.store_sqrt(245, 137);

        s.store_mul(246, 137, 245);

        s.store_scaled_mul_ad(127, A::powf(A::scale(s.ad_value(374), 1.0 / (s.v[445])), 1.5), A::exp(A::offset(A::mul_scaled_lhs(s.ad_value(137), (-1.0 / (2.0)), s.ad_value(120)), ((s.v[465] / 2.0) * s.v[464]))), 1.04e16);

        s.store_scalar(117, (((((2.0 * 1.6021918e-19) * s.v[452]) * 1.034943e-10)) as f64).sqrt());

        s.store_scalar(118, (1.0 / (s.v[452] * s.v[452])));

        s.store_scaled_sqrt(100, 122, s.v[117]);

        s.store_square(119, 100);

        s.store_scaled_square(101, 127, s.v[118]);

        s.store_scalar(279, ((p.p38 / (p.p251 + p.p252)) * p.p0));

        s.store_scalar(281, ((((p.p38 * 0.001) + ((10.0 * 2.220446049250313e-16) / 100.0))) as f64).abs());

        s.b[683] = (p.p38 > 0.0);
        s.store_scalar(683, if s.b[683] { 1.0 } else { 0.0 });

        if s.b[683] {
            s.store_scalar(638, ((p.p38 - s.v[279]) - s.v[281]));
            s.store_scalar(639, ((4.0 * p.p38) * s.v[281]));
        }

        if s.b[683] {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if s.b[683] {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(280, 638, (-0.5), 639, (-0.5), p.p38);
        }

        if (!s.b[683]) {
            s.store_offset(638, 279, (((-p.p38)) + ((-s.v[281]))));
            s.store_scalar(639, ((4.0 * p.p38) * s.v[281]));
        }

        if (!s.b[683]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (!s.b[683]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(280, 638, 0.5, 639, 0.5, p.p38);
        }

        s.store_sub_from_scalar_scaled_input(123, p.p0, 280, 2.0);

        s.store_scalar(279, ((-p.p49) * (1.0 + (p.p50 / ((s.v[375]) as f64).powf(p.p51)))));

        s.store_scalar(280, ((-p.p49) * (1.0 + (p.p52 / ((s.v[375]) as f64).powf(p.p53)))));

        s.store_scalar(281, (-(p.p49 + (p.p54 * s.v[375]))));

        s.store_scalar(638, ((s.v[279] - s.v[280]) - 1e-12));

        s.store_scalar(639, ((4.0 * s.v[280]) * 1e-12));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));

        s.store_scaled_offset_ad(279, A::div_from_scalar(s.v[638], s.ad_value(639)), 1.0, 0.5);

        s.store_offset_scaled(138, 639, 0.5, ((((s.v[638]) * (0.5))) + (s.v[280])));

        s.store_offset(638, 138, (((-s.v[281])) + ((-1e-12))));

        s.store_scalar(639, ((4.0 * s.v[281]) * 1e-12));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(138, 638, 0.5, 639, 0.5, s.v[281]);

        s.store_neg(138, 138);

        s.store_mul_scaled_ln_ad_rhs(128, 122, 2.0, A::div(s.ad_value(471), s.ad_value(127)));

        s.store_sqrt_mul_ad(125, A::div_from_scalar(1.034943e-10, s.ad_value(126)), s.ad_value(122));

        s.store_scaled_mul(141, 126, 125, 1.414213562373095);

        s.copy_ad(438, 474);

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_sqrt_mul_scaled_lhs(439, 438, 2.0, 122);

        s.store_div(279, 127, 471);

        s.store_square(142, 279);

        s.store_div(279, 127, 462);

        s.store_square(143, 279);

        s.store_scalar(272, p.p226);

        s.store_scalar(273, (3.453133e-11 / s.v[272]));

        s.store_scalar(274, (s.v[272] / 3.453133e-11));

        s.store_scalar(294, (3.453133e-11 / p.p229));

        s.store_scalar(295, (p.p229 / 3.453133e-11));

        s.store_scale(296, 471, ((-1.6021918e-19) * p.p227));

        s.store_scalar(535, (1.034943e-10 / p.p227));

        s.store_scalar(536, (1.0 / s.v[535]));

        s.store_scalar(293, (s.v[295] + s.v[536]));

        s.store_scalar(31, p.p254);

        s.store_scalar(30, p.p255);

        s.b[688] = (s.v[31] > (s.v[30] * 0.5));
        s.store_scalar(688, if s.b[688] { 1.0 } else { 0.0 });

        if s.b[688] {
            s.store_scalar(31, (0.5 * s.v[30]));
        }

        s.b[689] = (s.v[47] > s.v[31]);
        s.store_scalar(689, if s.b[689] { 1.0 } else { 0.0 });

        if s.b[689] {
            s.store_sub(280, 47, 31);
            s.store_sub_from_scalar(281, s.v[30], 31);
            s.store_square(642, 280);
            s.store_square(643, 281);
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[690] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.store_scalar(690, if s.b[690] { 1.0 } else { 0.0 });

        s.b[691] = (4.0 == 1.0);
        s.store_scalar(691, if s.b[691] { 1.0 } else { 0.0 });

        if ((s.b[689] && s.b[690]) && s.b[691]) {
            s.store_scalar(648, 1.0);
        }

        s.b[692] = (4.0 == 2.0);
        s.store_scalar(692, if s.b[692] { 1.0 } else { 0.0 });

        if (((s.b[689] && s.b[690]) && (!s.b[691])) && s.b[692]) {
            s.store_scalar(648, 2.0);
        }

        s.b[693] = (4.0 == 4.0);
        s.store_scalar(693, if s.b[693] { 1.0 } else { 0.0 });

        if ((((s.b[689] && s.b[690]) && (!s.b[691])) && (!s.b[692])) && s.b[693]) {
            s.store_scalar(648, 3.0);
        }

        s.b[694] = (4.0 == 8.0);
        s.store_scalar(694, if s.b[694] { 1.0 } else { 0.0 });

        if (((((s.b[689] && s.b[690]) && (!s.b[691])) && (!s.b[692])) && (!s.b[693])) && s.b[694]) {
            s.store_scalar(648, 4.0);
        }

        if (s.b[689] && s.b[690]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign4560_loop_guard: usize = 0;
        while {
            let assign4560_cond_e3027: f64 = if ((s.b[689] && s.b[690]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign4560_cond_e3027 != 0.0
        } {
            assign4560_loop_guard += 1;
            assert!(assign4560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[689] && s.b[690]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if (s.b[689] && (!s.b[690])) {
            s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));
        }

        if s.b[689] {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_mul3_lhs(282, 280, 281, 646);
            s.store_div_scaled_product3_mixed_iiia(286, 281, 645, 646, 1.0, A::offset(s.ad_value(220), 1e-50), 1.0);
            s.store_add(43, 31, 282);
            s.copy_ad(46, 286);
        }

        if (!s.b[689]) {
            s.copy_ad(43, 47);
            s.store_scalar(46, 1.0);
        }

        s.copy_ad(44, 48);

        s.copy_ad(45, 49);

        s.store_scalar(33, 0.0);

        s.store_scalar(695, 0.0);

        s.store_scalar(696, 0.0);

        s.store_scalar(697, 0.0);

        s.store_scalar(698, 0.0);

        s.store_scalar(699, 0.0);

        s.store_scalar(700, 0.0);

        s.copy_ad(50, 43);

        s.copy_ad(51, 44);

        s.copy_ad(52, 45);

        s.store_scalar(62, 0.0);

        s.store_scalar(63, 0.0);

        s.store_scaled_mul(279, 46, 51, 0.5);

        s.store_scale(638, 279, (2.0 * 1.0 / (p.p216)));

        s.store_offset_mul_offset_rhs_ad_rhs(639, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(640, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(73, p.p216, 639);

        s.store_div_scaled_inputs_square_rhs(280, 640, (-2.0), 639, 1.0);

        s.b[701] = (s.v[73] < 1e-12);
        s.store_scalar(701, if s.b[701] { 1.0 } else { 0.0 });

        if s.b[701] {
            s.store_scalar(73, 1e-12);
        }

        s.store_add(70, 50, 73);

        s.store_add_scaled_inputs(71, 51, 1.0, 73, 2.0);

        s.store_add(72, 52, 73);

        s.store_scale(279, 126, (2.0 * (1.034943e-10 * (s.v[274] * s.v[274]))));

        s.store_sub(280, 52, 138);

        s.store_offset_mul_ad(281, A::div_from_scalar(2.0, s.ad_value(279)), A::add_scaled_inputs3(s.ad_value(280), 1.0, s.ad_value(122), (-1.0), s.ad_value(50), -1.0), 1.0);

        s.store_sqrt_square_offset(639, 281, ((4.0 * 0.001) * 0.001));

        s.store_offset_scaled_div(283, 281, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(282, 281, 0.5, 639, 0.5, (1e-10 * 0.001));

        s.b[702] = (s.v[282] < 0.0);
        s.store_scalar(702, if s.b[702] { 1.0 } else { 0.0 });

        if s.b[702] {
            s.store_scalar(282, 0.0);
            s.store_scalar(283, 0.0);
        }

        s.store_sqrt_offset_input(290, 282, 1e-50);

        s.store_add_mul_sub_from_scalar_rhs_indices(87, 280, 279, 1.0, 290);

        s.store_sub(88, 87, 128);

        s.store_offset(638, 88, (((-0.1)) + ((-0.05))));

        s.store_scalar(639, ((4.0 * 0.1) * 0.05));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_offset_scaled_div(284, 638, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(88, 638, 0.5, 639, 0.5, 0.1);

        s.store_div(279, 51, 88);

        s.copy_ad(638, 279);

        s.store_square(639, 638);

        s.store_mul(640, 639, 638);

        s.store_square(641, 639);

        s.store_div_from_scalar_ad(290, 1.0, A::add_scaled_inputs4_offset(s.ad_value(638), 1.0, s.ad_value(639), 1.0, s.ad_value(640), 1.0, s.ad_value(641), 1.0, 1.0));

        s.store_mul_ad_affine_product_lhs(278, A::add_scaled_inputs3_offset(s.ad_value(638), 2.0, s.ad_value(639), 3.0, s.ad_value(640), 4.0, 1.0), s.ad_value(290), -1.0, 0.0, 290);

        s.store_sub_from_scalar(290, 1.0, 290);

        s.store_neg(278, 278);

        s.store_square(276, 290);

        s.b[703] = (((p.p193 == 0.0) && (p.p195 == 0.0)) || (p.p194 == 0.0));
        s.store_scalar(703, if s.b[703] { 1.0 } else { 0.0 });

        if s.b[703] {
            s.store_scalar(37, 0.0);
        }

        if (!s.b[703]) {
            s.store_scalar(37, 1.0);
        }

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(275, 129, 1.0, 138, 1.0, A::mul_scaled_lhs(s.ad_value(126), (2.0 * 1.034943e-10), s.ad_value(129)), 1.0 / (s.v[273]));

        s.b[704] = (s.v[37] == 0.0);
        s.store_scalar(704, if s.b[704] { 1.0 } else { 0.0 });

        if s.b[704] {
            s.store_scalar(268, s.v[272]);
            s.store_scalar(270, s.v[273]);
            s.store_scalar(271, s.v[274]);
            s.store_scale(278, 141, (s.v[274] * s.v[274]));
            s.store_mul(381, 278, 141);
        }

        if (!s.b[704]) {
            s.store_add_scaled_inputs3_offset_indices(283, 52, 1.0, 50, (-1.0), 275, -1.0, p.p194);
            s.store_sqrt_square_offset(639, 283, ((4.0 * 0.0001) * 0.0001));
            s.store_offset_scaled_div(281, 283, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(280, 283, 0.5, 639, 0.5, (1e-10 * 0.0001));
        }

        s.b[705] = (s.v[280] < 0.0);
        s.store_scalar(705, if s.b[705] { 1.0 } else { 0.0 });

        if ((!s.b[704]) && s.b[705]) {
            s.store_scalar(280, 0.0);
            s.store_scalar(281, 0.0);
        }

        if (!s.b[704]) {
            s.store_div_from_scalar(281, 1.0, 280);
            s.store_scaled_abs(282, 275, 2.0);
            s.store_offset_sub(284, 138, 275, p.p194);
        }

        s.b[706] = (s.v[284] > s.v[282]);
        s.store_scalar(706, if s.b[706] { 1.0 } else { 0.0 });

        if ((!s.b[704]) && s.b[706]) {
            s.copy_ad(282, 284);
        }

        if (!s.b[704]) {
            s.store_offset_sub_ad(638, A::div_from_scalar(1.0, s.ad_value(282)), s.ad_value(281), (-0.0001));
            s.store_scale_ad(639, A::div_from_scalar(1.0, s.ad_value(282)), (4.0 * 0.0001));
        }

        if (!s.b[704]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (!s.b[704]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(284, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_mixed_aii(280, A::div_from_scalar(1.0, s.ad_value(282)), 1.0, 638, (-0.5), 639, (-0.5));
            s.store_offset_scaled(269, 280, p.p193, p.p195);
        }

        s.b[707] = ((s.v[269] * 1000000000000.0) < s.v[272]);
        s.store_scalar(707, if s.b[707] { 1.0 } else { 0.0 });

        if ((!s.b[704]) && s.b[707]) {
            s.store_scalar(269, 0.0);
            s.store_scalar(37, 0.0);
        }

        if (!s.b[704]) {
            s.store_offset(268, 269, s.v[272]);
            s.store_div_from_scalar(270, 3.453133e-11, 268);
            s.store_scale(271, 268, 28959208927.08158);
            s.store_mul_ad_product_lhs_mixed_ai(381, A::square(s.ad_value(141)), 271, 271);
        }

        s.store_offset_sub_from_scalar_ad(638, 0.5, s.ad_value(70), (-0.001));

        s.store_scalar(639, ((4.0 * 0.5) * 0.001));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_offset_scaled_div(278, 638, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(382, 638, (-0.5), 639, (-0.5), 0.5);

        s.store_sqrt_mul(150, 473, 129);

        s.store_add_ad_lhs(265, A::add_scaled_inputs_product(s.ad_value(129), 1.0, s.ad_value(138), 1.0, s.ad_value(150), s.ad_value(271), 1.0), 380);

        s.copy_ad(130, 129);

        s.store_scalar(278, 0.95);

        s.store_offset_sub_scaled_inputs_indices(279, 130, s.v[278], 382, 1.0, (-0.001));

        s.store_sqrt_add_scaled_square_input(280, 279, 1.0, 130, ((4.0 * s.v[278]) * 0.001));

        s.store_add_scaled_inputs4_indices(131, 130, 1.0, 130, (-s.v[278]), 279, (-(-0.5)), 280, (-(-0.5)));

        s.store_sqrt(135, 131);

        s.b[708] = (p.p58 != 0.0);
        s.store_scalar(708, if s.b[708] { 1.0 } else { 0.0 });

        if s.b[708] {
            s.store_sqrt_mul_scaled_lhs(278, 471, ((2.0 * 1.6021918e-19) * 1.034943e-10), 136);
            s.store_add_scaled_inputs_product_indices(79, 136, 1.0, 138, 1.0, 278, 271, 1.0);
            s.store_scalar(278, ((2.0 * p.p227) / (p.p58 * p.p58)));
            s.store_mul_ad_affine_product_rhs(81, 271, s.ad_value(278), A::sub_from_scalar(p.p55, s.ad_value(130)), 1.034943e-10, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[708] {
            s.store_add_scaled_ad_lhs(278, A::scale_offset(s.ad_value(131), (p.p68 / p.p58), p.p66), 71, p.p67);
            s.store_mul_ad_product_lhs_mixed_ai(266, A::sub(s.ad_value(265), s.ad_value(79)), 81, 278);
        }

        if (!s.b[708]) {
            s.store_scalar(266, 0.0);
        }

        s.b[709] = (p.p297 != 0.0);
        s.store_scalar(709, if s.b[709] { 1.0 } else { 0.0 });

        if s.b[709] {
            s.store_offset_add_ad(288, A::add_scaled_product(s.ad_value(122), 1.0, s.ad_value(381), s.ad_value(120), (-0.25)), s.ad_value(138), 1e-50);
            s.store_offset_sub(279, 72, 288, (-0.005));
        }

        if s.b[709] {
            s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[709] {
            s.store_sqrt_add_scaled_square_product(280, 279, 1.0, 278, 288, (4.0 * 0.005));
            s.store_add_scaled_inputs4_indices(281, 288, 1.0, 279, 0.5, 280, 0.5, 138, -1.0);
            s.store_mul_ad_product_lhs_mixed_ai(282, A::div_from_scalar(4.0, s.ad_value(381)), 122, 122);
            s.store_offset_mul(283, 120, 281, (-1.0));
            s.store_offset_mul(279, 283, 282, 1.0);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(285, 279, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[710] = (s.v[279] < 0.0);
        s.store_scalar(710, if s.b[710] { 1.0 } else { 0.0 });

        if (s.b[709] && s.b[710]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(285, 0.0);
        }

        if s.b[709] {
            s.store_sqrt_offset_input(280, 279, (10.0 * 2.220446049250313e-16));
            s.store_add_product3_rhs_mixed_iia(139, 281, 381, 120, A::sub_from_scalar(1.0, s.ad_value(280)), 0.5);
            s.store_offset_sub(638, 129, 139, (-0.005));
            s.store_scale(639, 129, (4.0 * 0.005));
        }

        if s.b[709] {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if s.b[709] {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(280, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(140, 129, 1.0, 638, (-0.5), 639, (-0.5));
            s.store_add_scaled_inputs3_indices(130, 129, 1.0, 140, p.p297, 129, (-p.p297));
        }

        s.store_scale(279, 271, (1.034943e-10 * (p.p227 * 2.0)));

        s.store_sub_from_scalar(280, p.p55, 130);

        s.store_scalar(281, (s.v[277] - p.p57));

        s.store_scaled_mul(81, 279, 280, 1.0 / ((s.v[281] * s.v[281])));

        s.store_sqrt_square_offset(639, 50, ((4.0 * 0.001) * 0.001));

        s.store_offset_scaled_div(278, 50, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(593, 50, 0.5, 639, 0.5, (1e-10 * 0.001));

        s.b[711] = (s.v[593] < 0.0);
        s.store_scalar(711, if s.b[711] { 1.0 } else { 0.0 });

        if s.b[711] {
            s.store_scalar(593, 0.0);
            s.store_scalar(278, 0.0);
        }

        s.store_add_scaled_inputs3_offset_indices(283, 131, (p.p71 / s.v[277]), 71, p.p70, 593, p.p250, p.p69);

        s.store_mul(82, 81, 283);

        s.b[712] = (p.p72 > 0.0);
        s.store_scalar(712, if s.b[712] { 1.0 } else { 0.0 });

        if s.b[712] {
            s.store_add_scaled_inputs3_offset_indices(279, 137, 1.0, 128, 1.0, 71, p.p73, (-(2.0 * p.p74)));
            s.store_scalar(280, ((s.v[277] * 0.5) + p.p56));
            s.store_div_from_scalar(281, (p.p72 * p.p227), 280);
            s.store_mul(83, 279, 281);
        }

        if (!s.b[712]) {
            s.store_scalar(83, 0.0);
        }

        s.store_div_from_scalar_offset_input(281, 1.0, 270, (s.v[626] / s.v[124]));

        s.store_sub(283, 271, 281);

        s.store_offset_mul(84, 150, 283, (p.p104 / s.v[376]));

        s.store_add_scaled_inputs4_offset_indices(80, 82, 1.0, 266, 1.0, 84, 1.0, 83, 1.0, s.v[482]);

        s.store_sub(78, 265, 80);

        s.b[713] = (p.p75 == 0.0);
        s.store_scalar(713, if s.b[713] { 1.0 } else { 0.0 });

        if s.b[713] {
            s.store_scalar(36, 0.0);
        }

        if (!s.b[713]) {
            s.store_scalar(36, 1.0);
        }

        s.b[714] = (s.v[36] == 0.0);
        s.store_scalar(714, if s.b[714] { 1.0 } else { 0.0 });

        if s.b[714] {
            s.store_scalar(267, 0.0);
        }

        if (!s.b[714]) {
            s.store_offset(281, 72, (-p.p76));
        }

        s.b[715] = (s.v[281] < (-3.0));
        s.store_scalar(715, if s.b[715] { 1.0 } else { 0.0 });

        if ((!s.b[714]) && s.b[715]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(267, 0.0);
        }

        s.b[716] = (s.v[281] < 0.0);
        s.store_scalar(716, if s.b[716] { 1.0 } else { 0.0 });

        if (((!s.b[714]) && (!s.b[715])) && s.b[716]) {
            s.store_offset_mul_ad(284, s.ad_value(281), A::scale_offset(s.ad_value(281), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(267, 281, A::mul(s.ad_value(281), A::scale_offset(s.ad_value(281), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);
        }

        if (((!s.b[714]) && (!s.b[715])) && (!s.b[716])) {
            s.store_offset_mul_offset_rhs_ad_rhs(284, 281, A::mul(s.ad_value(281), A::scale_offset(s.ad_value(281), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(267, 281, A::mul_offset_rhs(s.ad_value(281), A::mul(s.ad_value(281), A::scale_offset(s.ad_value(281), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);
        }

        if (!s.b[714]) {
            s.store_sqrt_offset_square_offset(639, 267, (-1.0), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(284, A::div_scaled_offset_numerator(s.ad_value(267), 1.0, (-1.0), s.ad_value(639), 1.0), 1.0, 0.5);
            s.store_offset_add_scaled_inputs_mixed_ai(267, A::offset(s.ad_value(267), (-1.0)), 0.5, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[717] = (s.v[267] < 0.0);
        s.store_scalar(717, if s.b[717] { 1.0 } else { 0.0 });

        if ((!s.b[714]) && s.b[717]) {
            s.store_scalar(267, 0.0);
            s.store_scalar(284, 0.0);
        }

        if (!s.b[714]) {
            s.store_scale(267, 267, s.v[479]);
            s.store_offset_sub_from_scalar_ad(638, 1.0, s.ad_value(267), (-0.05));
            s.store_scalar(639, (4.0 * 0.05));
        }

        if (!s.b[714]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (!s.b[714]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(287, 638, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(267, 638, (-0.5), 639, (-0.5), 1.0);
        }

        s.store_add_scaled_inputs4_indices(53, 52, 1.0, 138, (-1.0), 80, 1.0, 267, -1.0);

        s.copy_ad(76, 53);

        s.store_mul_ln_ad_rhs(298, 122, A::div(s.ad_value(471), s.ad_value(462)));

        s.store_add_scaled_inputs3_indices(54, 138, 1.0, 80, (-1.0), 267, 1.0);

        s.store_mul(144, 141, 271);

        s.store_square(145, 144);

        if (p.p29 != 0.0) {
            s.store_add(440, 70, 298);
        }

        if (p.p29 == 0.0) {
            s.store_add(440, 50, 298);
        }

        s.b[718] = (s.v[440] < 0.0);
        s.store_scalar(718, if s.b[718] { 1.0 } else { 0.0 });

        if s.b[718] {
            s.store_div(278, 462, 471);
            s.store_offset(279, 278, 1.0);
            s.store_add_scaled_inputs_product_right_ad(280, 122, 1.0, 440, (-1.0), 278, A::add(s.ad_value(122), s.ad_value(440)), 1.0);
            s.store_scaled_square(281, 439, (s.v[295] * s.v[295]));
            s.store_add_scaled_products_indices(282, 280, 279, 2.0, 281, 120, (-1.0));
            s.store_add_scaled_inputs3_mixed_aai(283, A::square(s.ad_value(280)), 1.0, A::mul3(s.ad_value(281), s.ad_value(120), s.ad_value(440)), 1.0, 281, 1.0);
        }

        if s.b[718] {
            if (((s.v[282] * s.v[282]) - (((4.0 * s.v[279]) * s.v[279]) * s.v[283])) >= 1e-50) {
                s.store_sub_ad(285, A::square(s.ad_value(282)), A::mul3_scaled_output(s.ad_value(279), s.ad_value(279), s.ad_value(283), 4.0));
            } else {
                s.store_scalar(285, 1e-50);
            }
        }

        if s.b[718] {
            s.store_div_scaled_inputs2_mixed_iaa(331, 282, 1.0, A::sqrt(s.ad_value(285)), 1.0, A::offset(A::square(s.ad_value(279)), 2.0), 1.0);
        }

        if (!s.b[718]) {
            s.store_mul_square_lhs(279, 439, 120);
            s.store_mul_square_lhs(280, 141, 120);
            s.store_neg_ad(281, A::add_scaled_inputs(s.ad_value(122), 1.0, s.ad_value(440), 2.0));
            s.store_offset_div(282, 280, 279, 1.0);
            s.store_scaled_square(283, 141, (s.v[295] * s.v[295]));
            s.store_add_scaled_products_indices(284, 283, 120, 1.0, 281, 282, (-2.0));
        }

        if (!s.b[718]) {
            if (((s.v[284] * s.v[284]) - ((((4.0 * s.v[282]) * s.v[282]) * s.v[281]) * s.v[281])) >= 1e-50) {
                s.store_add_scaled_square_product_mixed_iai(285, 284, 1.0, A::mul3_scaled_output(s.ad_value(282), s.ad_value(282), s.ad_value(281), 4.0), 281, (-1.0));
            } else {
                s.store_scalar(285, 1e-50);
            }
        }

        if (!s.b[718]) {
            s.store_div_scaled_inputs2_mixed_iaa(331, 284, 1.0, A::sqrt(s.ad_value(285)), 1.0, A::mul_scaled_lhs(s.ad_value(282), 2.0, s.ad_value(282)), 1.0);
        }

        s.store_mul_div_from_scalar_lhs_ad_mixed_ia(326, 2.0, 120, A::ln(A::div(s.ad_value(462), s.ad_value(127))));

        s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));

        s.store_neg(279, 440);

        s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));

        if (!(s.v[280] >= (10.0 * 2.220446049250313e-16))) {
            s.store_scalar(280, (10.0 * 2.220446049250313e-16));
        }

        s.store_sqrt(280, 280);

        s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);

        s.store_scaled_sub(324, 281, 280, 0.5);

        s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));

        s.b[719] = (s.v[324] < s.v[326]);
        s.store_scalar(719, if s.b[719] { 1.0 } else { 0.0 });

        if s.b[719] {
            s.copy_ad(331, 324);
        }

        if (!s.b[719]) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if (!s.b[719]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (!s.b[719]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(331, 325, 1.0, 638, (-0.5), 639, (-0.5));
        }

        s.store_scalar(62, 0.0);

        let mut assign6930_loop_guard: usize = 0;
        while {
            let assign6930_cond_e4908: f64 = if s.v[62] < s.v[28] { 1.0 } else { 0.0 };
            assign6930_cond_e4908 != 0.0
        } {
            assign6930_loop_guard += 1;
            assert!(assign6930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 439);
            s.store_mul(280, 120, 331);
            s.store_exp_neg_input(281, 280);
            s.b[720] = (s.v[331] > 1e-8);
            s.store_scalar(720, if s.b[720] { 1.0 } else { 0.0 });
            if s.b[720] {
                s.store_exp_mul(278, 120, 331);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);
            }
            s.b[721] = (s.v[331] < (-1e-8));
            s.store_scalar(721, if s.b[721] { 1.0 } else { 0.0 });
            if ((!s.b[720]) && s.b[721]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if ((!s.b[720]) && (!s.b[721])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 331);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-6));
            s.b[722] = (s.v[284] < 0.0);
            s.store_scalar(722, if s.b[722] { 1.0 } else { 0.0 });
            if s.b[722] {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-9));
            s.store_scale(639, 296, (-(4.0 * 1e-9)));
            if (!(s.v[639] > 0.0)) {
                s.store_neg(639, 639);
            }
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));
            s.store_mul3_lhs(285, 285, 283, 286);
            s.store_div_scaled_inputs_mixed_ai(334, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);
            s.store_div_scaled_product_indices(335, 334, 285, 2.0, 284, 1.0);
            s.store_sub_ad_rhs(284, 331, A::div_scaled_inputs4(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(331), (-1.0), s.ad_value(440), -1.0, s.ad_value(334), 1.0, A::add(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), s.ad_value(335)), 1.0));
            s.b[723] = ((((s.v[284] - s.v[331])) as f64).abs() < 0.001);
            s.store_scalar(723, if s.b[723] { 1.0 } else { 0.0 });
            if s.b[723] {
                s.store_scalar(62, s.v[28]);
            }
            s.copy_ad(331, 284);
            s.copy_ad(330, 282);
            s.store_offset(62, 62, 1.0);
        }

        s.copy_ad(332, 334);

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_sqrt_div_scaled_inputs(279, 332, ((2.0 * 1.034943e-10) / 1.6021918e-19), 471, 1.0);

        s.b[724] = (s.v[279] > (0.99 * p.p227));
        s.store_scalar(724, if s.b[724] { 1.0 } else { 0.0 });

        if s.b[724] {
            s.store_div_from_scalar(278, 1.0, 270);
            s.store_scalar(280, (1.0 / s.v[294]));
            s.store_div_from_scalar_add_ad(281, 1.0, A::offset(s.ad_value(278), s.v[536]), s.ad_value(280));
            s.store_sub_from_scalar_scaled_mul(282, 1.0, 281, 278, 1.0);
            s.store_mul_ad_product_rhs_mixed_ia(283, 278, 281, A::sub(A::mul_scaled_rhs(A::offset(s.ad_value(280), (0.5 * s.v[536])), s.ad_value(296), -1.0), s.ad_value(440)));
            s.store_div(327, 283, 282);
            s.store_add(54, 54, 327);
            s.store_sub_scaled_inputs(53, 53, 1.0, 327, p.p298);
            s.copy_ad(76, 53);
        }

        s.b[725] = (s.v[33] >= 1.0);
        s.store_scalar(725, if s.b[725] { 1.0 } else { 0.0 });

        if s.b[725] {
            s.store_scalar(305, s.v[695]);
            s.store_scalar(306, s.v[696]);
            s.store_offset(307, 440, s.v[697]);
            s.store_add_scaled_inputs(328, 296, (-(s.v[536] * 0.5)), 122, 1.0);
            s.store_sub_scaled_inputs(329, 328, 1.0, 330, s.v[536]);
        }

        s.b[726] = (s.v[440] < 0.0);
        s.store_scalar(726, if s.b[726] { 1.0 } else { 0.0 });

        if ((!s.b[725]) && s.b[726]) {
            s.store_scalar(55, 0.0);
            s.store_scalar(62, 1.0);
        }

        let mut assign7150_loop_guard: usize = 0;
        while {
            let assign7150_cond_e5303: f64 = if (((!s.b[725]) && s.b[726]) && (s.v[62] <= s.v[28])) { 1.0 } else { 0.0 };
            assign7150_cond_e5303 != 0.0
        } {
            assign7150_loop_guard += 1;
            assert!(assign7150_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && s.b[726]) {
                s.store_div_from_scalar_scaled_input(278, s.v[294], 462, ((2.0 * 1.6021918e-19) * 1.034943e-10));
                s.store_scalar(279, (1.0 + (s.v[294] * s.v[536])));
                s.store_add_scaled_inputs3_indices(280, 296, ((-(0.5 * s.v[536])) * s.v[294]), 122, s.v[294], 440, s.v[294]);
                s.store_mul3_affine_lhs(285, 278, 270, 2.0, 0.0, 270);
                s.store_add_scaled_inputs_product_mixed_aaii(282, A::offset(A::mul(s.ad_value(279), s.ad_value(270)), s.v[294]), 1.0, A::mul3_scaled_output(s.ad_value(278), s.ad_value(270), s.ad_value(296), 2.0), 1.0, 285, 55, 1.0);
                s.store_mul3_affine_lhs(286, 270, 278, ((2.0 * s.v[294]) * 2.0), 0.0, 270);
                s.store_add_scaled_value_products(283, A::offset(A::mul3(A::add_scaled_square_product(s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(280), (-4.0)), s.ad_value(270), s.ad_value(270)), (s.v[294] * s.v[294])), 1.0, s.ad_value(270), A::add_scaled_product(s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(296), 2.0), (2.0 * s.v[294]), s.ad_value(286), s.ad_value(55), 1.0);
                s.store_sqrt(283, 283);
                s.store_div_scaled_inputs_indices(286, 286, 1.0, 283, 2.0);
                s.store_div_from_scalar_ad(284, 1.0, A::mul3_scaled_output(s.ad_value(278), s.ad_value(270), s.ad_value(270), 2.0));
                s.store_mul_sub_rhs(346, 284, 282, 283);
                s.store_mul_sub_rhs(347, 284, 285, 286);
                s.store_div_scaled_inputs_indices(370, 346, -1.0, 347, 1.0);
            }
            s.b[727] = (((s.v[370]) as f64).abs() < 1e-12);
            s.store_scalar(727, if s.b[727] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && s.b[726]) && s.b[727]) {
                s.store_scalar(62, s.v[28]);
            }
            s.b[728] = (s.v[370] > 0.1);
            s.store_scalar(728, if s.b[728] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[726]) && (!s.b[727])) && s.b[728]) {
                s.store_scalar(370, 0.1);
            }
            s.b[729] = (s.v[370] < (-0.1));
            s.store_scalar(729, if s.b[729] { 1.0 } else { 0.0 });
            if (((((!s.b[725]) && s.b[726]) && (!s.b[727])) && (!s.b[728])) && s.b[729]) {
                s.store_scalar(370, (-0.1));
            }
            if ((!s.b[725]) && s.b[726]) {
                s.store_add(55, 55, 370);
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[730] = (s.v[52] < (s.v[54] + s.v[55]));
        s.store_scalar(730, if s.b[730] { 1.0 } else { 0.0 });

        if ((!s.b[725]) && s.b[730]) {
            s.store_scalar(39, 1.0);
            s.store_scalar(292, (-1.0));
            s.copy_ad(332, 334);
            s.store_sqrt_div_scaled_inputs(279, 332, ((2.0 * 1.034943e-10) / 1.6021918e-19), 471, 1.0);
            s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));
        }

        s.b[731] = ((s.v[345] + s.v[279]) < p.p227);
        s.store_scalar(731, if s.b[731] { 1.0 } else { 0.0 });

        if (((!s.b[725]) && s.b[730]) && s.b[731]) {
            s.store_sub_from_scalar(279, (10.0 * 2.220446049250313e-16), 440);
            s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));
        }

        if (((!s.b[725]) && s.b[730]) && s.b[731]) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[725]) && s.b[730]) && s.b[731]) {
            s.store_sqrt(280, 280);
            s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);
            s.store_scaled_sub(324, 281, 280, 0.5);
            s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[732] = (s.v[324] < s.v[326]);
        s.store_scalar(732, if s.b[732] { 1.0 } else { 0.0 });

        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && s.b[732]) {
            s.copy_ad(307, 324);
        }

        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && (!s.b[732])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && (!s.b[732])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && (!s.b[732])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(307, 325, 1.0, 638, (-0.5), 639, (-0.5));
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {
            s.store_add_scaled_inputs3_indices(279, 440, (-1.0), 305, (-(-1.0)), 296, (-(-(0.5 * (p.p227 * 9662367879.197212)))));
            s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {
            s.store_sqrt(280, 280);
            s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);
            s.store_scaled_sub(324, 281, 280, 0.5);
            s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[733] = (s.v[324] < s.v[326]);
        s.store_scalar(733, if s.b[733] { 1.0 } else { 0.0 });

        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && s.b[733]) {
            s.copy_ad(307, 324);
        }

        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && (!s.b[733])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && (!s.b[733])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && (!s.b[733])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(307, 325, 1.0, 638, (-0.5), 639, (-0.5));
        }

        if ((!s.b[725]) && s.b[730]) {
            s.store_sqrt_div_scaled_inputs(279, 332, ((2.0 * 1.034943e-10) / 1.6021918e-19), 471, 1.0);
        }

        s.b[734] = ((s.v[345] + s.v[279]) < p.p227);
        s.store_scalar(734, if s.b[734] { 1.0 } else { 0.0 });

        if (((!s.b[725]) && s.b[730]) && s.b[734]) {
            s.store_scalar(62, 0.0);
        }

        let mut assign7560_loop_guard: usize = 0;
        while {
            let assign7560_cond_e6174: f64 = if ((((!s.b[725]) && s.b[730]) && s.b[734]) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign7560_cond_e6174 != 0.0
        } {
            assign7560_loop_guard += 1;
            assert!(assign7560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 307);
                s.store_exp_neg_input(281, 280);
            }
            s.b[735] = (s.v[307] > 1e-8);
            s.store_scalar(735, if s.b[735] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[735]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);
            }
            s.b[736] = (s.v[307] < (-1e-8));
            s.store_scalar(736, if s.b[736] { 1.0 } else { 0.0 });
            if (((((!s.b[725]) && s.b[730]) && s.b[734]) && (!s.b[735])) && s.b[736]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if (((((!s.b[725]) && s.b[730]) && s.b[734]) && (!s.b[735])) && (!s.b[736])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-10) * 1e-10));
                s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
                s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-10));
            }
            s.b[737] = (s.v[284] < 0.0);
            s.store_scalar(737, if s.b[737] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[737]) {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-13));
                s.store_scale(639, 296, (-(4.0 * 1e-13)));
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                if (s.v[639] > 0.0) {
                } else {
                    s.store_neg(639, 639);
                }
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.store_sqrt_square_add(639, 638, 639);
                s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));
                s.store_mul3_lhs(285, 285, 283, 286);
                s.store_div_scaled_inputs_mixed_ai(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);
                s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);
                s.store_sub_ad_rhs(284, 307, A::div_scaled_inputs4(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(307), (-1.0), s.ad_value(440), -1.0, s.ad_value(332), 1.0, A::add(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), s.ad_value(333)), 1.0));
            }
            s.b[738] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);
            s.store_scalar(738, if s.b[738] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[738]) {
                s.store_scalar(62, s.v[28]);
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.copy_ad(307, 284);
                s.copy_ad(312, 282);
                s.store_offset(62, 62, 1.0);
            }
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
            s.store_scalar(62, 0.0);
        }

    }
}
