#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_144(
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_alpha_dc: f64,
        var_alpha_dc_dn5: f64,
        var_alpha_dc_dn6: f64,
        var_alpha_dc_dn7: f64,
        var_alpha_dc_dn8: f64,
        var_bet_i: f64,
        var_cox_qm: f64,
        var_cox_qm_dn5: f64,
        var_cox_qm_dn6: f64,
        var_cox_qm_dn7: f64,
        var_cox_qm_dn8: f64,
        var_eta_p_ac: f64,
        var_eta_p_ac_dn5: f64,
        var_eta_p_ac_dn6: f64,
        var_eta_p_ac_dn7: f64,
        var_eta_p_ac_dn8: f64,
        var_guard1572: f64,
        var_guard1573: f64,
        var_guard1697: f64,
        var_guard1714: f64,
        var_guard1715: f64,
        var_ijunbot_d: f64,
        var_ijunbot_d_dn10: f64,
        var_ijunbot_d_dn11: f64,
        var_ijunbot_d_dn5: f64,
        var_ijunbot_d_dn6: f64,
        var_ijunbot_d_dn7: f64,
        var_ijunbot_d_dn8: f64,
        var_ijungat_d: f64,
        var_ijungat_d_dn10: f64,
        var_ijungat_d_dn11: f64,
        var_ijungat_d_dn5: f64,
        var_ijungat_d_dn6: f64,
        var_ijungat_d_dn7: f64,
        var_ijungat_d_dn8: f64,
        var_ijunsti_d: f64,
        var_ijunsti_d_dn10: f64,
        var_ijunsti_d_dn11: f64,
        var_ijunsti_d_dn5: f64,
        var_ijunsti_d_dn6: f64,
        var_ijunsti_d_dn7: f64,
        var_ijunsti_d_dn8: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_one_minus_pgat2nd_d: f64,
        var_one_minus_pgat_d: f64,
        var_qb: f64,
        var_qb_dn5: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qg: f64,
        var_qg_dn5: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qgd_ov: f64,
        var_qgd_ov_dn5: f64,
        var_qgd_ov_dn6: f64,
        var_qgd_ov_dn7: f64,
        var_qgs_ov: f64,
        var_qgs_ov_dn5: f64,
        var_qgs_ov_dn6: f64,
        var_qgs_ov_dn7: f64,
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
        var_qjunbot_d: f64,
        var_qjunbot_d_dn10: f64,
        var_qjunbot_d_dn11: f64,
        var_qjunbot_d_dn5: f64,
        var_qjunbot_d_dn6: f64,
        var_qjunbot_d_dn7: f64,
        var_qjunbot_d_dn8: f64,
        var_qjunbot_s: f64,
        var_qjunbot_s_dn10: f64,
        var_qjunbot_s_dn11: f64,
        var_qjunbot_s_dn5: f64,
        var_qjunbot_s_dn6: f64,
        var_qjunbot_s_dn7: f64,
        var_qjunbot_s_dn8: f64,
        var_qjungat_s: f64,
        var_qjungat_s_dn10: f64,
        var_qjungat_s_dn11: f64,
        var_qjungat_s_dn5: f64,
        var_qjungat_s_dn6: f64,
        var_qjungat_s_dn7: f64,
        var_qjungat_s_dn8: f64,
        var_qjunsti_d: f64,
        var_qjunsti_d_dn10: f64,
        var_qjunsti_d_dn11: f64,
        var_qjunsti_d_dn5: f64,
        var_qjunsti_d_dn6: f64,
        var_qjunsti_d_dn7: f64,
        var_qjunsti_d_dn8: f64,
        var_qjunsti_s: f64,
        var_qjunsti_s_dn10: f64,
        var_qjunsti_s_dn11: f64,
        var_qjunsti_s_dn5: f64,
        var_qjunsti_s_dn6: f64,
        var_qjunsti_s_dn7: f64,
        var_qjunsti_s_dn8: f64,
        var_qpref2gat2nd_d: f64,
        var_qpref2gat_d: f64,
        var_qprefgat2nd_d: f64,
        var_qprefgat_d: f64,
        var_rbulk_i: f64,
        var_rde_i: f64,
        var_rg_i: f64,
        var_rjund_i: f64,
        var_rjuns_i: f64,
        var_rse_i: f64,
        var_rwell_i: f64,
        var_sigvds: f64,
        var_vbiinvgat2nd_d: f64,
        var_vbiinvgat_d: f64,
        var_vch_d: f64,
        var_vfmin_d: f64,
        var_vj__blk1535: f64,
        var_vj__blk1535_dn10: f64,
        var_vj__blk1535_dn11: f64,
        var_vj__blk1535_dn6: f64,
        var_vj__blk1535_dn7: f64,
        var_vjun_d: f64,
        var_vjun_d_dn11: f64,
        var_vjun_d_dn7: f64,
        var_vtrgatd_i: f64,
        var_xg_dc: f64,
        var_c_igid_slot: &mut f64,
        var_c_igid_dn5_slot: &mut f64,
        var_c_igid_dn6_slot: &mut f64,
        var_c_igid_dn7_slot: &mut f64,
        var_c_igid_dn8_slot: &mut f64,
        var_cgeff_slot: &mut f64,
        var_cgeff_dn5_slot: &mut f64,
        var_cgeff_dn6_slot: &mut f64,
        var_cgeff_dn7_slot: &mut f64,
        var_cgeff_dn8_slot: &mut f64,
        var_guard1716_slot: &mut f64,
        var_guard1717_slot: &mut f64,
        var_guard1718_slot: &mut f64,
        var_guard1719_slot: &mut f64,
        var_guard1720_slot: &mut f64,
        var_guard1721_slot: &mut f64,
        var_guard1722_slot: &mut f64,
        var_guard1723_slot: &mut f64,
        var_guard1724_slot: &mut f64,
        var_guard1725_slot: &mut f64,
        var_guard1727_slot: &mut f64,
        var_guard1760_slot: &mut f64,
        var_guard1762_slot: &mut f64,
        var_h0_slot: &mut f64,
        var_h0_dn5_slot: &mut f64,
        var_h0_dn6_slot: &mut f64,
        var_h0_dn7_slot: &mut f64,
        var_h0_dn8_slot: &mut f64,
        var_h1__blk1528_slot: &mut f64,
        var_h2__blk1529_slot: &mut f64,
        var_h2d__blk1530_slot: &mut f64,
        var_h2d__blk1530_dn10_slot: &mut f64,
        var_h2d__blk1530_dn11_slot: &mut f64,
        var_h2d__blk1530_dn6_slot: &mut f64,
        var_h2d__blk1530_dn7_slot: &mut f64,
        var_h3__blk1531_slot: &mut f64,
        var_h3__blk1531_dn10_slot: &mut f64,
        var_h3__blk1531_dn11_slot: &mut f64,
        var_h3__blk1531_dn6_slot: &mut f64,
        var_h3__blk1531_dn7_slot: &mut f64,
        var_h4__blk1532_slot: &mut f64,
        var_h4__blk1532_dn10_slot: &mut f64,
        var_h4__blk1532_dn11_slot: &mut f64,
        var_h4__blk1532_dn6_slot: &mut f64,
        var_h4__blk1532_dn7_slot: &mut f64,
        var_h5__blk1533_slot: &mut f64,
        var_h5__blk1533_dn10_slot: &mut f64,
        var_h5__blk1533_dn11_slot: &mut f64,
        var_h5__blk1533_dn6_slot: &mut f64,
        var_h5__blk1533_dn7_slot: &mut f64,
        var_ijun_d_slot: &mut f64,
        var_ijun_d_dn10_slot: &mut f64,
        var_ijun_d_dn11_slot: &mut f64,
        var_ijun_d_dn5_slot: &mut f64,
        var_ijun_d_dn6_slot: &mut f64,
        var_ijun_d_dn7_slot: &mut f64,
        var_ijun_d_dn8_slot: &mut f64,
        var_mid_slot: &mut f64,
        var_mid_dn5_slot: &mut f64,
        var_mid_dn6_slot: &mut f64,
        var_mid_dn7_slot: &mut f64,
        var_mid_dn8_slot: &mut f64,
        var_mig_slot: &mut f64,
        var_mig_dn5_slot: &mut f64,
        var_mig_dn6_slot: &mut f64,
        var_mig_dn7_slot: &mut f64,
        var_mig_dn8_slot: &mut f64,
        var_migid_slot: &mut f64,
        var_migid_dn5_slot: &mut f64,
        var_migid_dn6_slot: &mut f64,
        var_migid_dn7_slot: &mut f64,
        var_migid_dn8_slot: &mut f64,
        var_nu__blk1570_slot: &mut f64,
        var_nu__blk1570_dn10_slot: &mut f64,
        var_nu__blk1570_dn11_slot: &mut f64,
        var_nu__blk1570_dn6_slot: &mut f64,
        var_nu__blk1570_dn7_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qfgd_slot: &mut f64,
        var_qfgd_dn5_slot: &mut f64,
        var_qfgd_dn6_slot: &mut f64,
        var_qfgd_dn7_slot: &mut f64,
        var_qfgs_slot: &mut f64,
        var_qfgs_dn5_slot: &mut f64,
        var_qfgs_dn6_slot: &mut f64,
        var_qfgs_dn7_slot: &mut f64,
        var_qjun_d_slot: &mut f64,
        var_qjun_d_dn10_slot: &mut f64,
        var_qjun_d_dn11_slot: &mut f64,
        var_qjun_d_dn5_slot: &mut f64,
        var_qjun_d_dn6_slot: &mut f64,
        var_qjun_d_dn7_slot: &mut f64,
        var_qjun_d_dn8_slot: &mut f64,
        var_qjun_s_slot: &mut f64,
        var_qjun_s_dn10_slot: &mut f64,
        var_qjun_s_dn11_slot: &mut f64,
        var_qjun_s_dn5_slot: &mut f64,
        var_qjun_s_dn6_slot: &mut f64,
        var_qjun_s_dn7_slot: &mut f64,
        var_qjun_s_dn8_slot: &mut f64,
        var_qjungat2nd_slot: &mut f64,
        var_qjungat2nd_dn10_slot: &mut f64,
        var_qjungat2nd_dn11_slot: &mut f64,
        var_qjungat2nd_dn5_slot: &mut f64,
        var_qjungat2nd_dn6_slot: &mut f64,
        var_qjungat2nd_dn7_slot: &mut f64,
        var_qjungat2nd_dn8_slot: &mut f64,
        var_qjungat_d_slot: &mut f64,
        var_qjungat_d_dn10_slot: &mut f64,
        var_qjungat_d_dn11_slot: &mut f64,
        var_qjungat_d_dn5_slot: &mut f64,
        var_qjungat_d_dn6_slot: &mut f64,
        var_qjungat_d_dn7_slot: &mut f64,
        var_qjungat_d_dn8_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn5_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_sidexc_slot: &mut f64,
        var_sidexc_dn5_slot: &mut f64,
        var_sidexc_dn6_slot: &mut f64,
        var_sidexc_dn7_slot: &mut f64,
        var_sidexc_dn8_slot: &mut f64,
        var_sqid_slot: &mut f64,
        var_sqid_dn5_slot: &mut f64,
        var_sqid_dn6_slot: &mut f64,
        var_sqid_dn7_slot: &mut f64,
        var_sqid_dn8_slot: &mut f64,
        var_sqig_slot: &mut f64,
        var_sqig_dn5_slot: &mut f64,
        var_sqig_dn6_slot: &mut f64,
        var_sqig_dn7_slot: &mut f64,
        var_sqig_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_temp__blk1726_slot: &mut f64,
        var_temp__blk1726_dn5_slot: &mut f64,
        var_temp__blk1726_dn6_slot: &mut f64,
        var_temp__blk1726_dn7_slot: &mut f64,
        var_temp__blk1726_dn8_slot: &mut f64,
        var_tmp__blk1543_slot: &mut f64,
        var_tmp__blk1543_dn10_slot: &mut f64,
        var_tmp__blk1543_dn11_slot: &mut f64,
        var_tmp__blk1543_dn5_slot: &mut f64,
        var_tmp__blk1543_dn6_slot: &mut f64,
        var_tmp__blk1543_dn7_slot: &mut f64,
        var_tmp__blk1543_dn8_slot: &mut f64,
        var_vjtmp_slot: &mut f64,
        var_vjtmp_dn10_slot: &mut f64,
        var_vjtmp_dn11_slot: &mut f64,
        var_vjtmp_dn6_slot: &mut f64,
        var_vjtmp_dn7_slot: &mut f64,
    ) {
        let mut var_c_igid: f64 = *var_c_igid_slot;
        let mut var_c_igid_dn5: f64 = *var_c_igid_dn5_slot;
        let mut var_c_igid_dn6: f64 = *var_c_igid_dn6_slot;
        let mut var_c_igid_dn7: f64 = *var_c_igid_dn7_slot;
        let mut var_c_igid_dn8: f64 = *var_c_igid_dn8_slot;
        let mut var_cgeff: f64 = *var_cgeff_slot;
        let mut var_cgeff_dn5: f64 = *var_cgeff_dn5_slot;
        let mut var_cgeff_dn6: f64 = *var_cgeff_dn6_slot;
        let mut var_cgeff_dn7: f64 = *var_cgeff_dn7_slot;
        let mut var_cgeff_dn8: f64 = *var_cgeff_dn8_slot;
        let mut var_guard1716: f64 = *var_guard1716_slot;
        let mut var_guard1717: f64 = *var_guard1717_slot;
        let mut var_guard1718: f64 = *var_guard1718_slot;
        let mut var_guard1719: f64 = *var_guard1719_slot;
        let mut var_guard1720: f64 = *var_guard1720_slot;
        let mut var_guard1721: f64 = *var_guard1721_slot;
        let mut var_guard1722: f64 = *var_guard1722_slot;
        let mut var_guard1723: f64 = *var_guard1723_slot;
        let mut var_guard1724: f64 = *var_guard1724_slot;
        let mut var_guard1725: f64 = *var_guard1725_slot;
        let mut var_guard1727: f64 = *var_guard1727_slot;
        let mut var_guard1760: f64 = *var_guard1760_slot;
        let mut var_guard1762: f64 = *var_guard1762_slot;
        let mut var_h0: f64 = *var_h0_slot;
        let mut var_h0_dn5: f64 = *var_h0_dn5_slot;
        let mut var_h0_dn6: f64 = *var_h0_dn6_slot;
        let mut var_h0_dn7: f64 = *var_h0_dn7_slot;
        let mut var_h0_dn8: f64 = *var_h0_dn8_slot;
        let mut var_h1__blk1528: f64 = *var_h1__blk1528_slot;
        let mut var_h2__blk1529: f64 = *var_h2__blk1529_slot;
        let mut var_h2d__blk1530: f64 = *var_h2d__blk1530_slot;
        let mut var_h2d__blk1530_dn10: f64 = *var_h2d__blk1530_dn10_slot;
        let mut var_h2d__blk1530_dn11: f64 = *var_h2d__blk1530_dn11_slot;
        let mut var_h2d__blk1530_dn6: f64 = *var_h2d__blk1530_dn6_slot;
        let mut var_h2d__blk1530_dn7: f64 = *var_h2d__blk1530_dn7_slot;
        let mut var_h3__blk1531: f64 = *var_h3__blk1531_slot;
        let mut var_h3__blk1531_dn10: f64 = *var_h3__blk1531_dn10_slot;
        let mut var_h3__blk1531_dn11: f64 = *var_h3__blk1531_dn11_slot;
        let mut var_h3__blk1531_dn6: f64 = *var_h3__blk1531_dn6_slot;
        let mut var_h3__blk1531_dn7: f64 = *var_h3__blk1531_dn7_slot;
        let mut var_h4__blk1532: f64 = *var_h4__blk1532_slot;
        let mut var_h4__blk1532_dn10: f64 = *var_h4__blk1532_dn10_slot;
        let mut var_h4__blk1532_dn11: f64 = *var_h4__blk1532_dn11_slot;
        let mut var_h4__blk1532_dn6: f64 = *var_h4__blk1532_dn6_slot;
        let mut var_h4__blk1532_dn7: f64 = *var_h4__blk1532_dn7_slot;
        let mut var_h5__blk1533: f64 = *var_h5__blk1533_slot;
        let mut var_h5__blk1533_dn10: f64 = *var_h5__blk1533_dn10_slot;
        let mut var_h5__blk1533_dn11: f64 = *var_h5__blk1533_dn11_slot;
        let mut var_h5__blk1533_dn6: f64 = *var_h5__blk1533_dn6_slot;
        let mut var_h5__blk1533_dn7: f64 = *var_h5__blk1533_dn7_slot;
        let mut var_ijun_d: f64 = *var_ijun_d_slot;
        let mut var_ijun_d_dn10: f64 = *var_ijun_d_dn10_slot;
        let mut var_ijun_d_dn11: f64 = *var_ijun_d_dn11_slot;
        let mut var_ijun_d_dn5: f64 = *var_ijun_d_dn5_slot;
        let mut var_ijun_d_dn6: f64 = *var_ijun_d_dn6_slot;
        let mut var_ijun_d_dn7: f64 = *var_ijun_d_dn7_slot;
        let mut var_ijun_d_dn8: f64 = *var_ijun_d_dn8_slot;
        let mut var_mid: f64 = *var_mid_slot;
        let mut var_mid_dn5: f64 = *var_mid_dn5_slot;
        let mut var_mid_dn6: f64 = *var_mid_dn6_slot;
        let mut var_mid_dn7: f64 = *var_mid_dn7_slot;
        let mut var_mid_dn8: f64 = *var_mid_dn8_slot;
        let mut var_mig: f64 = *var_mig_slot;
        let mut var_mig_dn5: f64 = *var_mig_dn5_slot;
        let mut var_mig_dn6: f64 = *var_mig_dn6_slot;
        let mut var_mig_dn7: f64 = *var_mig_dn7_slot;
        let mut var_mig_dn8: f64 = *var_mig_dn8_slot;
        let mut var_migid: f64 = *var_migid_slot;
        let mut var_migid_dn5: f64 = *var_migid_dn5_slot;
        let mut var_migid_dn6: f64 = *var_migid_dn6_slot;
        let mut var_migid_dn7: f64 = *var_migid_dn7_slot;
        let mut var_migid_dn8: f64 = *var_migid_dn8_slot;
        let mut var_nu__blk1570: f64 = *var_nu__blk1570_slot;
        let mut var_nu__blk1570_dn10: f64 = *var_nu__blk1570_dn10_slot;
        let mut var_nu__blk1570_dn11: f64 = *var_nu__blk1570_dn11_slot;
        let mut var_nu__blk1570_dn6: f64 = *var_nu__blk1570_dn6_slot;
        let mut var_nu__blk1570_dn7: f64 = *var_nu__blk1570_dn7_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qfgd: f64 = *var_qfgd_slot;
        let mut var_qfgd_dn5: f64 = *var_qfgd_dn5_slot;
        let mut var_qfgd_dn6: f64 = *var_qfgd_dn6_slot;
        let mut var_qfgd_dn7: f64 = *var_qfgd_dn7_slot;
        let mut var_qfgs: f64 = *var_qfgs_slot;
        let mut var_qfgs_dn5: f64 = *var_qfgs_dn5_slot;
        let mut var_qfgs_dn6: f64 = *var_qfgs_dn6_slot;
        let mut var_qfgs_dn7: f64 = *var_qfgs_dn7_slot;
        let mut var_qjun_d: f64 = *var_qjun_d_slot;
        let mut var_qjun_d_dn10: f64 = *var_qjun_d_dn10_slot;
        let mut var_qjun_d_dn11: f64 = *var_qjun_d_dn11_slot;
        let mut var_qjun_d_dn5: f64 = *var_qjun_d_dn5_slot;
        let mut var_qjun_d_dn6: f64 = *var_qjun_d_dn6_slot;
        let mut var_qjun_d_dn7: f64 = *var_qjun_d_dn7_slot;
        let mut var_qjun_d_dn8: f64 = *var_qjun_d_dn8_slot;
        let mut var_qjun_s: f64 = *var_qjun_s_slot;
        let mut var_qjun_s_dn10: f64 = *var_qjun_s_dn10_slot;
        let mut var_qjun_s_dn11: f64 = *var_qjun_s_dn11_slot;
        let mut var_qjun_s_dn5: f64 = *var_qjun_s_dn5_slot;
        let mut var_qjun_s_dn6: f64 = *var_qjun_s_dn6_slot;
        let mut var_qjun_s_dn7: f64 = *var_qjun_s_dn7_slot;
        let mut var_qjun_s_dn8: f64 = *var_qjun_s_dn8_slot;
        let mut var_qjungat2nd: f64 = *var_qjungat2nd_slot;
        let mut var_qjungat2nd_dn10: f64 = *var_qjungat2nd_dn10_slot;
        let mut var_qjungat2nd_dn11: f64 = *var_qjungat2nd_dn11_slot;
        let mut var_qjungat2nd_dn5: f64 = *var_qjungat2nd_dn5_slot;
        let mut var_qjungat2nd_dn6: f64 = *var_qjungat2nd_dn6_slot;
        let mut var_qjungat2nd_dn7: f64 = *var_qjungat2nd_dn7_slot;
        let mut var_qjungat2nd_dn8: f64 = *var_qjungat2nd_dn8_slot;
        let mut var_qjungat_d: f64 = *var_qjungat_d_slot;
        let mut var_qjungat_d_dn10: f64 = *var_qjungat_d_dn10_slot;
        let mut var_qjungat_d_dn11: f64 = *var_qjungat_d_dn11_slot;
        let mut var_qjungat_d_dn5: f64 = *var_qjungat_d_dn5_slot;
        let mut var_qjungat_d_dn6: f64 = *var_qjungat_d_dn6_slot;
        let mut var_qjungat_d_dn7: f64 = *var_qjungat_d_dn7_slot;
        let mut var_qjungat_d_dn8: f64 = *var_qjungat_d_dn8_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn5: f64 = *var_qs_dn5_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_sidexc: f64 = *var_sidexc_slot;
        let mut var_sidexc_dn5: f64 = *var_sidexc_dn5_slot;
        let mut var_sidexc_dn6: f64 = *var_sidexc_dn6_slot;
        let mut var_sidexc_dn7: f64 = *var_sidexc_dn7_slot;
        let mut var_sidexc_dn8: f64 = *var_sidexc_dn8_slot;
        let mut var_sqid: f64 = *var_sqid_slot;
        let mut var_sqid_dn5: f64 = *var_sqid_dn5_slot;
        let mut var_sqid_dn6: f64 = *var_sqid_dn6_slot;
        let mut var_sqid_dn7: f64 = *var_sqid_dn7_slot;
        let mut var_sqid_dn8: f64 = *var_sqid_dn8_slot;
        let mut var_sqig: f64 = *var_sqig_slot;
        let mut var_sqig_dn5: f64 = *var_sqig_dn5_slot;
        let mut var_sqig_dn6: f64 = *var_sqig_dn6_slot;
        let mut var_sqig_dn7: f64 = *var_sqig_dn7_slot;
        let mut var_sqig_dn8: f64 = *var_sqig_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_temp__blk1726: f64 = *var_temp__blk1726_slot;
        let mut var_temp__blk1726_dn5: f64 = *var_temp__blk1726_dn5_slot;
        let mut var_temp__blk1726_dn6: f64 = *var_temp__blk1726_dn6_slot;
        let mut var_temp__blk1726_dn7: f64 = *var_temp__blk1726_dn7_slot;
        let mut var_temp__blk1726_dn8: f64 = *var_temp__blk1726_dn8_slot;
        let mut var_tmp__blk1543: f64 = *var_tmp__blk1543_slot;
        let mut var_tmp__blk1543_dn10: f64 = *var_tmp__blk1543_dn10_slot;
        let mut var_tmp__blk1543_dn11: f64 = *var_tmp__blk1543_dn11_slot;
        let mut var_tmp__blk1543_dn5: f64 = *var_tmp__blk1543_dn5_slot;
        let mut var_tmp__blk1543_dn6: f64 = *var_tmp__blk1543_dn6_slot;
        let mut var_tmp__blk1543_dn7: f64 = *var_tmp__blk1543_dn7_slot;
        let mut var_tmp__blk1543_dn8: f64 = *var_tmp__blk1543_dn8_slot;
        let mut var_vjtmp: f64 = *var_vjtmp_slot;
        let mut var_vjtmp_dn10: f64 = *var_vjtmp_dn10_slot;
        let mut var_vjtmp_dn11: f64 = *var_vjtmp_dn11_slot;
        let mut var_vjtmp_dn6: f64 = *var_vjtmp_dn6_slot;
        let mut var_vjtmp_dn7: f64 = *var_vjtmp_dn7_slot;

        let (assign61620_e80166, assign61620_e80166_d_n5, assign61620_e80166_d_n6, assign61620_e80166_d_n7, assign61620_e80166_d_n8, assign61620_e80166_d_n10, assign61620_e80166_d_n11,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) && (var_guard1715 == 0.0)) {
        let assign61620_e80161: f64 = (var_vjtmp * var_vbiinvgat_d);
        let assign61620_e80162: f64 = (1.0 - assign61620_e80161);
        let assign61620_e80164: f64 = (assign61620_e80162).powf(var_one_minus_pgat_d);
        (assign61620_e80164, 0.0, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn6 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn6 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn7 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn7 * var_vbiinvgat_d)) / assign61620_e80162))) }, 0.0, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn10 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn10 * var_vbiinvgat_d)) / assign61620_e80162))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61620_e80162).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn11 * var_vbiinvgat_d)))) } } else { (assign61620_e80164 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn11 * var_vbiinvgat_d)) / assign61620_e80162))) },)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11,)
    }
};
        var_tmp__blk1543 = assign61620_e80166;
        var_tmp__blk1543_dn5 = assign61620_e80166_d_n5;
        var_tmp__blk1543_dn6 = assign61620_e80166_d_n6;
        var_tmp__blk1543_dn7 = assign61620_e80166_d_n7;
        var_tmp__blk1543_dn8 = assign61620_e80166_d_n8;
        var_tmp__blk1543_dn10 = assign61620_e80166_d_n10;
        var_tmp__blk1543_dn11 = assign61620_e80166_d_n11;

        let (assign61630_e80190, assign61630_e80190_d_n5, assign61630_e80190_d_n6, assign61630_e80190_d_n7, assign61630_e80190_d_n8, assign61630_e80190_d_n10, assign61630_e80190_d_n11,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61630_e80180: f64 = (1.0 - var_tmp__blk1543);
        let assign61630_e80181: f64 = (var_qprefgat_d * assign61630_e80180);
        let assign61630_e80185: f64 = (var_nu__blk1570 - var_vjtmp);
        let assign61630_e80186: f64 = (var_qpref2gat_d * assign61630_e80185);
        let assign61630_e80187: f64 = (assign61630_e80181 + assign61630_e80186);
        let assign61630_e80188: f64 = (p.p30 * assign61630_e80187);
        (assign61630_e80188, (p.p30 * (var_qprefgat_d * (-var_tmp__blk1543_dn5))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn6)) + (var_qpref2gat_d * (var_nu__blk1570_dn6 - var_vjtmp_dn6)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn7)) + (var_qpref2gat_d * (var_nu__blk1570_dn7 - var_vjtmp_dn7)))), (p.p30 * (var_qprefgat_d * (-var_tmp__blk1543_dn8))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn10)) + (var_qpref2gat_d * (var_nu__blk1570_dn10 - var_vjtmp_dn10)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn11)) + (var_qpref2gat_d * (var_nu__blk1570_dn11 - var_vjtmp_dn11)))),)
    } else {
        (var_qjungat_d, var_qjungat_d_dn5, var_qjungat_d_dn6, var_qjungat_d_dn7, var_qjungat_d_dn8, var_qjungat_d_dn10, var_qjungat_d_dn11,)
    }
};
        var_qjungat_d = assign61630_e80190;
        var_qjungat_d_dn5 = assign61630_e80190_d_n5;
        var_qjungat_d_dn6 = assign61630_e80190_d_n6;
        var_qjungat_d_dn7 = assign61630_e80190_d_n7;
        var_qjungat_d_dn8 = assign61630_e80190_d_n8;
        var_qjungat_d_dn10 = assign61630_e80190_d_n10;
        var_qjungat_d_dn11 = assign61630_e80190_d_n11;

        let (assign61640_e80206, assign61640_e80206_d_n6, assign61640_e80206_d_n7, assign61640_e80206_d_n10, assign61640_e80206_d_n11,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61640_e80202: f64 = (var_vjun_d + var_vtrgatd_i);
        let assign61640_e80204: f64 = (assign61640_e80202 - var_nu__blk1570);
        (assign61640_e80204, (-var_nu__blk1570_dn6), (var_vjun_d_dn7 - var_nu__blk1570_dn7), (-var_nu__blk1570_dn10), (var_vjun_d_dn11 - var_nu__blk1570_dn11),)
    } else {
        (var_nu__blk1570, var_nu__blk1570_dn6, var_nu__blk1570_dn7, var_nu__blk1570_dn10, var_nu__blk1570_dn11,)
    }
};
        var_nu__blk1570 = assign61640_e80206;
        var_nu__blk1570_dn6 = assign61640_e80206_d_n6;
        var_nu__blk1570_dn7 = assign61640_e80206_d_n7;
        var_nu__blk1570_dn10 = assign61640_e80206_d_n10;
        var_nu__blk1570_dn11 = assign61640_e80206_d_n11;

        let (assign61650_e80222,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61650_e80218: f64 = (4.0 * var_vch_d);
        let assign61650_e80220: f64 = (assign61650_e80218 * var_vch_d);
        (assign61650_e80220,)
    } else {
        (var_h1__blk1528,)
    }
};
        var_h1__blk1528 = assign61650_e80222;

        let (assign61660_e80236,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61660_e80234: f64 = (var_vch_d / var_vfmin_d);
        (assign61660_e80234,)
    } else {
        (var_h2__blk1529,)
    }
};
        var_h2__blk1529 = assign61660_e80236;

        let (assign61670_e80252, assign61670_e80252_d_n6, assign61670_e80252_d_n7, assign61670_e80252_d_n10, assign61670_e80252_d_n11,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61670_e80249: f64 = (var_vch_d * var_h2__blk1529);
        let assign61670_e80250: f64 = (var_nu__blk1570 + assign61670_e80249);
        (assign61670_e80250, var_nu__blk1570_dn6, var_nu__blk1570_dn7, var_nu__blk1570_dn10, var_nu__blk1570_dn11,)
    } else {
        (var_h2d__blk1530, var_h2d__blk1530_dn6, var_h2d__blk1530_dn7, var_h2d__blk1530_dn10, var_h2d__blk1530_dn11,)
    }
};
        var_h2d__blk1530 = assign61670_e80252;
        var_h2d__blk1530_dn6 = assign61670_e80252_d_n6;
        var_h2d__blk1530_dn7 = assign61670_e80252_d_n7;
        var_h2d__blk1530_dn10 = assign61670_e80252_d_n10;
        var_h2d__blk1530_dn11 = assign61670_e80252_d_n11;

        let (assign61680_e80266, assign61680_e80266_d_n6, assign61680_e80266_d_n7, assign61680_e80266_d_n10, assign61680_e80266_d_n11,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61680_e80264: f64 = (var_vfmin_d + var_h2d__blk1530);
        (assign61680_e80264, var_h2d__blk1530_dn6, var_h2d__blk1530_dn7, var_h2d__blk1530_dn10, var_h2d__blk1530_dn11,)
    } else {
        (var_h3__blk1531, var_h3__blk1531_dn6, var_h3__blk1531_dn7, var_h3__blk1531_dn10, var_h3__blk1531_dn11,)
    }
};
        var_h3__blk1531 = assign61680_e80266;
        var_h3__blk1531_dn6 = assign61680_e80266_d_n6;
        var_h3__blk1531_dn7 = assign61680_e80266_d_n7;
        var_h3__blk1531_dn10 = assign61680_e80266_d_n10;
        var_h3__blk1531_dn11 = assign61680_e80266_d_n11;

        let (assign61690_e80280, assign61690_e80280_d_n6, assign61690_e80280_d_n7, assign61690_e80280_d_n10, assign61690_e80280_d_n11,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61690_e80278: f64 = (var_vfmin_d - var_h2d__blk1530);
        (assign61690_e80278, (-var_h2d__blk1530_dn6), (-var_h2d__blk1530_dn7), (-var_h2d__blk1530_dn10), (-var_h2d__blk1530_dn11),)
    } else {
        (var_h4__blk1532, var_h4__blk1532_dn6, var_h4__blk1532_dn7, var_h4__blk1532_dn10, var_h4__blk1532_dn11,)
    }
};
        var_h4__blk1532 = assign61690_e80280;
        var_h4__blk1532_dn6 = assign61690_e80280_d_n6;
        var_h4__blk1532_dn7 = assign61690_e80280_d_n7;
        var_h4__blk1532_dn10 = assign61690_e80280_d_n10;
        var_h4__blk1532_dn11 = assign61690_e80280_d_n11;

        let (assign61700_e80297, assign61700_e80297_d_n6, assign61700_e80297_d_n7, assign61700_e80297_d_n10, assign61700_e80297_d_n11,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61700_e80292: f64 = (var_h4__blk1532 * var_h4__blk1532);
        let assign61700_e80294: f64 = (assign61700_e80292 + var_h1__blk1528);
        let assign61700_e80295: f64 = (assign61700_e80294).sqrt();
        (assign61700_e80295, (((var_h4__blk1532_dn6 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn6)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn7 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn7)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn10 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn10)) / (2.0 * assign61700_e80295)), (((var_h4__blk1532_dn11 * var_h4__blk1532) + (var_h4__blk1532 * var_h4__blk1532_dn11)) / (2.0 * assign61700_e80295)),)
    } else {
        (var_h5__blk1533, var_h5__blk1533_dn6, var_h5__blk1533_dn7, var_h5__blk1533_dn10, var_h5__blk1533_dn11,)
    }
};
        var_h5__blk1533 = assign61700_e80297;
        var_h5__blk1533_dn6 = assign61700_e80297_d_n6;
        var_h5__blk1533_dn7 = assign61700_e80297_d_n7;
        var_h5__blk1533_dn10 = assign61700_e80297_d_n10;
        var_h5__blk1533_dn11 = assign61700_e80297_d_n11;

        let (assign61710_e80317, assign61710_e80317_d_n6, assign61710_e80317_d_n7, assign61710_e80317_d_n10, assign61710_e80317_d_n11,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61710_e80310: f64 = (var_nu__blk1570 * var_vfmin_d);
        let assign61710_e80313: f64 = (var_h3__blk1531 + var_h5__blk1533);
        let assign61710_e80314: f64 = (assign61710_e80310 / assign61710_e80313);
        let assign61710_e80315: f64 = (2.0 * assign61710_e80314);
        (assign61710_e80315, (2.0 * ((((var_nu__blk1570_dn6 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn6 + var_h5__blk1533_dn6))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn7 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn7 + var_h5__blk1533_dn7))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn10 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn10 + var_h5__blk1533_dn10))) / (assign61710_e80313 * assign61710_e80313))), (2.0 * ((((var_nu__blk1570_dn11 * var_vfmin_d) * assign61710_e80313) - (assign61710_e80310 * (var_h3__blk1531_dn11 + var_h5__blk1533_dn11))) / (assign61710_e80313 * assign61710_e80313))),)
    } else {
        (var_vjtmp, var_vjtmp_dn6, var_vjtmp_dn7, var_vjtmp_dn10, var_vjtmp_dn11,)
    }
};
        var_vjtmp = assign61710_e80317;
        var_vjtmp_dn6 = assign61710_e80317_d_n6;
        var_vjtmp_dn7 = assign61710_e80317_d_n7;
        var_vjtmp_dn10 = assign61710_e80317_d_n10;
        var_vjtmp_dn11 = assign61710_e80317_d_n11;

        let assign61720_e80320: f64 = if var_one_minus_pgat2nd_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1716 = assign61720_e80320;

        let (assign61730_e80339, assign61730_e80339_d_n5, assign61730_e80339_d_n6, assign61730_e80339_d_n7, assign61730_e80339_d_n8, assign61730_e80339_d_n10, assign61730_e80339_d_n11,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) && (var_guard1716 != 0.0)) {
        let assign61730_e80335: f64 = (var_vjtmp * var_vbiinvgat2nd_d);
        let assign61730_e80336: f64 = (1.0 - assign61730_e80335);
        let assign61730_e80337: f64 = (assign61730_e80336).sqrt();
        (assign61730_e80337, 0.0, ((-(var_vjtmp_dn6 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn7 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), 0.0, ((-(var_vjtmp_dn10 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)), ((-(var_vjtmp_dn11 * var_vbiinvgat2nd_d)) / (2.0 * assign61730_e80337)),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11,)
    }
};
        var_tmp__blk1543 = assign61730_e80339;
        var_tmp__blk1543_dn5 = assign61730_e80339_d_n5;
        var_tmp__blk1543_dn6 = assign61730_e80339_d_n6;
        var_tmp__blk1543_dn7 = assign61730_e80339_d_n7;
        var_tmp__blk1543_dn8 = assign61730_e80339_d_n8;
        var_tmp__blk1543_dn10 = assign61730_e80339_d_n10;
        var_tmp__blk1543_dn11 = assign61730_e80339_d_n11;

        let (assign61740_e80360, assign61740_e80360_d_n5, assign61740_e80360_d_n6, assign61740_e80360_d_n7, assign61740_e80360_d_n8, assign61740_e80360_d_n10, assign61740_e80360_d_n11,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) && (var_guard1716 == 0.0)) {
        let assign61740_e80355: f64 = (var_vjtmp * var_vbiinvgat2nd_d);
        let assign61740_e80356: f64 = (1.0 - assign61740_e80355);
        let assign61740_e80358: f64 = (assign61740_e80356).powf(var_one_minus_pgat2nd_d);
        (assign61740_e80358, 0.0, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn6 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn6 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn7 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn7 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, 0.0, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn10 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn10 * var_vbiinvgat2nd_d)) / assign61740_e80356))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61740_e80356).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn11 * var_vbiinvgat2nd_d)))) } } else { (assign61740_e80358 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn11 * var_vbiinvgat2nd_d)) / assign61740_e80356))) },)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11,)
    }
};
        var_tmp__blk1543 = assign61740_e80360;
        var_tmp__blk1543_dn5 = assign61740_e80360_d_n5;
        var_tmp__blk1543_dn6 = assign61740_e80360_d_n6;
        var_tmp__blk1543_dn7 = assign61740_e80360_d_n7;
        var_tmp__blk1543_dn8 = assign61740_e80360_d_n8;
        var_tmp__blk1543_dn10 = assign61740_e80360_d_n10;
        var_tmp__blk1543_dn11 = assign61740_e80360_d_n11;

        let (assign61750_e80384, assign61750_e80384_d_n5, assign61750_e80384_d_n6, assign61750_e80384_d_n7, assign61750_e80384_d_n8, assign61750_e80384_d_n10, assign61750_e80384_d_n11,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61750_e80374: f64 = (1.0 - var_tmp__blk1543);
        let assign61750_e80375: f64 = (var_qprefgat2nd_d * assign61750_e80374);
        let assign61750_e80379: f64 = (var_nu__blk1570 - var_vjtmp);
        let assign61750_e80380: f64 = (var_qpref2gat2nd_d * assign61750_e80379);
        let assign61750_e80381: f64 = (assign61750_e80375 + assign61750_e80380);
        let assign61750_e80382: f64 = (p.p30 * assign61750_e80381);
        (assign61750_e80382, (p.p30 * (var_qprefgat2nd_d * (-var_tmp__blk1543_dn5))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn6)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn6 - var_vjtmp_dn6)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn7)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn7 - var_vjtmp_dn7)))), (p.p30 * (var_qprefgat2nd_d * (-var_tmp__blk1543_dn8))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn10)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn10 - var_vjtmp_dn10)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1543_dn11)) + (var_qpref2gat2nd_d * (var_nu__blk1570_dn11 - var_vjtmp_dn11)))),)
    } else {
        (var_qjungat2nd, var_qjungat2nd_dn5, var_qjungat2nd_dn6, var_qjungat2nd_dn7, var_qjungat2nd_dn8, var_qjungat2nd_dn10, var_qjungat2nd_dn11,)
    }
};
        var_qjungat2nd = assign61750_e80384;
        var_qjungat2nd_dn5 = assign61750_e80384_d_n5;
        var_qjungat2nd_dn6 = assign61750_e80384_d_n6;
        var_qjungat2nd_dn7 = assign61750_e80384_d_n7;
        var_qjungat2nd_dn8 = assign61750_e80384_d_n8;
        var_qjungat2nd_dn10 = assign61750_e80384_d_n10;
        var_qjungat2nd_dn11 = assign61750_e80384_d_n11;

        let (assign61760_e80398, assign61760_e80398_d_n5, assign61760_e80398_d_n6, assign61760_e80398_d_n7, assign61760_e80398_d_n8, assign61760_e80398_d_n10, assign61760_e80398_d_n11,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 != 0.0)) {
        let assign61760_e80396: f64 = (var_qjungat_d + var_qjungat2nd);
        (assign61760_e80396, (var_qjungat_d_dn5 + var_qjungat2nd_dn5), (var_qjungat_d_dn6 + var_qjungat2nd_dn6), (var_qjungat_d_dn7 + var_qjungat2nd_dn7), (var_qjungat_d_dn8 + var_qjungat2nd_dn8), (var_qjungat_d_dn10 + var_qjungat2nd_dn10), (var_qjungat_d_dn11 + var_qjungat2nd_dn11),)
    } else {
        (var_qjungat_d, var_qjungat_d_dn5, var_qjungat_d_dn6, var_qjungat_d_dn7, var_qjungat_d_dn8, var_qjungat_d_dn10, var_qjungat_d_dn11,)
    }
};
        var_qjungat_d = assign61760_e80398;
        var_qjungat_d_dn5 = assign61760_e80398_d_n5;
        var_qjungat_d_dn6 = assign61760_e80398_d_n6;
        var_qjungat_d_dn7 = assign61760_e80398_d_n7;
        var_qjungat_d_dn8 = assign61760_e80398_d_n8;
        var_qjungat_d_dn10 = assign61760_e80398_d_n10;
        var_qjungat_d_dn11 = assign61760_e80398_d_n11;

        let assign61770_e80401: f64 = if var_one_minus_pgat_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1717 = assign61770_e80401;

        let (assign61780_e80421, assign61780_e80421_d_n5, assign61780_e80421_d_n6, assign61780_e80421_d_n7, assign61780_e80421_d_n8, assign61780_e80421_d_n10, assign61780_e80421_d_n11,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1717 != 0.0)) {
        let assign61780_e80417: f64 = (var_vj__blk1535 * var_vbiinvgat_d);
        let assign61780_e80418: f64 = (1.0 - assign61780_e80417);
        let assign61780_e80419: f64 = (assign61780_e80418).sqrt();
        (assign61780_e80419, 0.0, ((-(var_vj__blk1535_dn6 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn7 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), 0.0, ((-(var_vj__blk1535_dn10 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)), ((-(var_vj__blk1535_dn11 * var_vbiinvgat_d)) / (2.0 * assign61780_e80419)),)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11,)
    }
};
        var_tmp__blk1543 = assign61780_e80421;
        var_tmp__blk1543_dn5 = assign61780_e80421_d_n5;
        var_tmp__blk1543_dn6 = assign61780_e80421_d_n6;
        var_tmp__blk1543_dn7 = assign61780_e80421_d_n7;
        var_tmp__blk1543_dn8 = assign61780_e80421_d_n8;
        var_tmp__blk1543_dn10 = assign61780_e80421_d_n10;
        var_tmp__blk1543_dn11 = assign61780_e80421_d_n11;

        let (assign61790_e80443, assign61790_e80443_d_n5, assign61790_e80443_d_n6, assign61790_e80443_d_n7, assign61790_e80443_d_n8, assign61790_e80443_d_n10, assign61790_e80443_d_n11,) = {
    if (((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1717 == 0.0)) {
        let assign61790_e80438: f64 = (var_vj__blk1535 * var_vbiinvgat_d);
        let assign61790_e80439: f64 = (1.0 - assign61790_e80438);
        let assign61790_e80441: f64 = (assign61790_e80439).powf(var_one_minus_pgat_d);
        (assign61790_e80441, 0.0, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn6 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn6 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn7 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn7 * var_vbiinvgat_d)) / assign61790_e80439))) }, 0.0, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn10 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn10 * var_vbiinvgat_d)) / assign61790_e80439))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61790_e80439).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1535_dn11 * var_vbiinvgat_d)))) } } else { (assign61790_e80441 * (var_one_minus_pgat_d * ((-(var_vj__blk1535_dn11 * var_vbiinvgat_d)) / assign61790_e80439))) },)
    } else {
        (var_tmp__blk1543, var_tmp__blk1543_dn5, var_tmp__blk1543_dn6, var_tmp__blk1543_dn7, var_tmp__blk1543_dn8, var_tmp__blk1543_dn10, var_tmp__blk1543_dn11,)
    }
};
        var_tmp__blk1543 = assign61790_e80443;
        var_tmp__blk1543_dn5 = assign61790_e80443_d_n5;
        var_tmp__blk1543_dn6 = assign61790_e80443_d_n6;
        var_tmp__blk1543_dn7 = assign61790_e80443_d_n7;
        var_tmp__blk1543_dn8 = assign61790_e80443_d_n8;
        var_tmp__blk1543_dn10 = assign61790_e80443_d_n10;
        var_tmp__blk1543_dn11 = assign61790_e80443_d_n11;

        let (assign61800_e80468, assign61800_e80468_d_n5, assign61800_e80468_d_n6, assign61800_e80468_d_n7, assign61800_e80468_d_n8, assign61800_e80468_d_n10, assign61800_e80468_d_n11,) = {
    if ((((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) && (var_guard1697 == 0.0)) && (var_guard1714 == 0.0)) {
        let assign61800_e80458: f64 = (1.0 - var_tmp__blk1543);
        let assign61800_e80459: f64 = (var_qprefgat_d * assign61800_e80458);
        let assign61800_e80463: f64 = (var_vjun_d - var_vj__blk1535);
        let assign61800_e80464: f64 = (var_qpref2gat_d * assign61800_e80463);
        let assign61800_e80465: f64 = (assign61800_e80459 + assign61800_e80464);
        let assign61800_e80466: f64 = (p.p30 * assign61800_e80465);
        (assign61800_e80466, (p.p30 * (var_qprefgat_d * (-var_tmp__blk1543_dn5))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn6)) + (var_qpref2gat_d * (-var_vj__blk1535_dn6)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn7)) + (var_qpref2gat_d * (var_vjun_d_dn7 - var_vj__blk1535_dn7)))), (p.p30 * (var_qprefgat_d * (-var_tmp__blk1543_dn8))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn10)) + (var_qpref2gat_d * (-var_vj__blk1535_dn10)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1543_dn11)) + (var_qpref2gat_d * (var_vjun_d_dn11 - var_vj__blk1535_dn11)))),)
    } else {
        (var_qjungat_d, var_qjungat_d_dn5, var_qjungat_d_dn6, var_qjungat_d_dn7, var_qjungat_d_dn8, var_qjungat_d_dn10, var_qjungat_d_dn11,)
    }
};
        var_qjungat_d = assign61800_e80468;
        var_qjungat_d_dn5 = assign61800_e80468_d_n5;
        var_qjungat_d_dn6 = assign61800_e80468_d_n6;
        var_qjungat_d_dn7 = assign61800_e80468_d_n7;
        var_qjungat_d_dn8 = assign61800_e80468_d_n8;
        var_qjungat_d_dn10 = assign61800_e80468_d_n10;
        var_qjungat_d_dn11 = assign61800_e80468_d_n11;

        let (assign61810_e80485, assign61810_e80485_d_n5, assign61810_e80485_d_n6, assign61810_e80485_d_n7, assign61810_e80485_d_n8, assign61810_e80485_d_n10, assign61810_e80485_d_n11,) = {
    if ((var_guard1572 != 0.0) && (var_guard1573 == 0.0)) {
        let assign61810_e80475: f64 = (var_abdrain_i * var_ijunbot_d);
        let assign61810_e80478: f64 = (var_lsdrain_i * var_ijunsti_d);
        let assign61810_e80479: f64 = (assign61810_e80475 + assign61810_e80478);
        let assign61810_e80482: f64 = (var_lgdrain_i * var_ijungat_d);
        let assign61810_e80483: f64 = (assign61810_e80479 + assign61810_e80482);
        (assign61810_e80483, (((var_abdrain_i * var_ijunbot_d_dn5) + (var_lsdrain_i * var_ijunsti_d_dn5)) + (var_lgdrain_i * var_ijungat_d_dn5)), (((var_abdrain_i * var_ijunbot_d_dn6) + (var_lsdrain_i * var_ijunsti_d_dn6)) + (var_lgdrain_i * var_ijungat_d_dn6)), (((var_abdrain_i * var_ijunbot_d_dn7) + (var_lsdrain_i * var_ijunsti_d_dn7)) + (var_lgdrain_i * var_ijungat_d_dn7)), (((var_abdrain_i * var_ijunbot_d_dn8) + (var_lsdrain_i * var_ijunsti_d_dn8)) + (var_lgdrain_i * var_ijungat_d_dn8)), (((var_abdrain_i * var_ijunbot_d_dn10) + (var_lsdrain_i * var_ijunsti_d_dn10)) + (var_lgdrain_i * var_ijungat_d_dn10)), (((var_abdrain_i * var_ijunbot_d_dn11) + (var_lsdrain_i * var_ijunsti_d_dn11)) + (var_lgdrain_i * var_ijungat_d_dn11)),)
    } else {
        (var_ijun_d, var_ijun_d_dn5, var_ijun_d_dn6, var_ijun_d_dn7, var_ijun_d_dn8, var_ijun_d_dn10, var_ijun_d_dn11,)
    }
};
        var_ijun_d = assign61810_e80485;
        var_ijun_d_dn5 = assign61810_e80485_d_n5;
        var_ijun_d_dn6 = assign61810_e80485_d_n6;
        var_ijun_d_dn7 = assign61810_e80485_d_n7;
        var_ijun_d_dn8 = assign61810_e80485_d_n8;
        var_ijun_d_dn10 = assign61810_e80485_d_n10;
        var_ijun_d_dn11 = assign61810_e80485_d_n11;

        let assign61890_e80509: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard1718 = assign61890_e80509;

        let assign61900_e80512: f64 = if var_rg_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1719 = assign61900_e80512;

        let assign61910_e80515: f64 = if var_rse_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1720 = assign61910_e80515;

        let assign61920_e80518: f64 = if var_rde_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1721 = assign61920_e80518;

        let assign61930_e80521: f64 = if var_rbulk_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1722 = assign61930_e80521;

        let assign61940_e80524: f64 = if var_rjuns_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1723 = assign61940_e80524;

        let assign61950_e80527: f64 = if var_rjund_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1724 = assign61950_e80527;

        let assign61960_e80530: f64 = if var_rwell_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1725 = assign61960_e80530;

        let assign61970_e80533: f64 = (var_qg + var_qb);
        let assign61970_e80535: f64 = (assign61970_e80533 + var_qd);
        let assign61970_e80536: f64 = (-assign61970_e80535);
        var_qs = assign61970_e80536;
        var_qs_dn5 = (-((var_qg_dn5 + var_qb_dn5) + var_qd_dn5));
        var_qs_dn6 = (-((var_qg_dn6 + var_qb_dn6) + var_qd_dn6));
        var_qs_dn7 = (-((var_qg_dn7 + var_qb_dn7) + var_qd_dn7));
        var_qs_dn8 = (-((var_qg_dn8 + var_qb_dn8) + var_qd_dn8));

        let assign61980_e80539: f64 = (var_qfgs + var_qgs_ov);
        var_qfgs = assign61980_e80539;
        var_qfgs_dn5 = (var_qfgs_dn5 + var_qgs_ov_dn5);
        var_qfgs_dn6 = (var_qfgs_dn6 + var_qgs_ov_dn6);
        var_qfgs_dn7 = (var_qfgs_dn7 + var_qgs_ov_dn7);

        let assign61990_e80542: f64 = (var_qfgd + var_qgd_ov);
        var_qfgd = assign61990_e80542;
        var_qfgd_dn5 = (var_qfgd_dn5 + var_qgd_ov_dn5);
        var_qfgd_dn6 = (var_qfgd_dn6 + var_qgd_ov_dn6);
        var_qfgd_dn7 = (var_qfgd_dn7 + var_qgd_ov_dn7);

        let assign62000_e80545: f64 = (var_absource_i * var_qjunbot_s);
        let assign62000_e80548: f64 = (var_lssource_i * var_qjunsti_s);
        let assign62000_e80549: f64 = (assign62000_e80545 + assign62000_e80548);
        let assign62000_e80552: f64 = (var_lgsource_i * var_qjungat_s);
        let assign62000_e80553: f64 = (assign62000_e80549 + assign62000_e80552);
        var_qjun_s = assign62000_e80553;
        var_qjun_s_dn5 = (((var_absource_i * var_qjunbot_s_dn5) + (var_lssource_i * var_qjunsti_s_dn5)) + (var_lgsource_i * var_qjungat_s_dn5));
        var_qjun_s_dn6 = (((var_absource_i * var_qjunbot_s_dn6) + (var_lssource_i * var_qjunsti_s_dn6)) + (var_lgsource_i * var_qjungat_s_dn6));
        var_qjun_s_dn7 = (((var_absource_i * var_qjunbot_s_dn7) + (var_lssource_i * var_qjunsti_s_dn7)) + (var_lgsource_i * var_qjungat_s_dn7));
        var_qjun_s_dn8 = (((var_absource_i * var_qjunbot_s_dn8) + (var_lssource_i * var_qjunsti_s_dn8)) + (var_lgsource_i * var_qjungat_s_dn8));
        var_qjun_s_dn10 = (((var_absource_i * var_qjunbot_s_dn10) + (var_lssource_i * var_qjunsti_s_dn10)) + (var_lgsource_i * var_qjungat_s_dn10));
        var_qjun_s_dn11 = (((var_absource_i * var_qjunbot_s_dn11) + (var_lssource_i * var_qjunsti_s_dn11)) + (var_lgsource_i * var_qjungat_s_dn11));

        let assign62010_e80556: f64 = (var_abdrain_i * var_qjunbot_d);
        let assign62010_e80559: f64 = (var_lsdrain_i * var_qjunsti_d);
        let assign62010_e80560: f64 = (assign62010_e80556 + assign62010_e80559);
        let assign62010_e80563: f64 = (var_lgdrain_i * var_qjungat_d);
        let assign62010_e80564: f64 = (assign62010_e80560 + assign62010_e80563);
        var_qjun_d = assign62010_e80564;
        var_qjun_d_dn5 = (((var_abdrain_i * var_qjunbot_d_dn5) + (var_lsdrain_i * var_qjunsti_d_dn5)) + (var_lgdrain_i * var_qjungat_d_dn5));
        var_qjun_d_dn6 = (((var_abdrain_i * var_qjunbot_d_dn6) + (var_lsdrain_i * var_qjunsti_d_dn6)) + (var_lgdrain_i * var_qjungat_d_dn6));
        var_qjun_d_dn7 = (((var_abdrain_i * var_qjunbot_d_dn7) + (var_lsdrain_i * var_qjunsti_d_dn7)) + (var_lgdrain_i * var_qjungat_d_dn7));
        var_qjun_d_dn8 = (((var_abdrain_i * var_qjunbot_d_dn8) + (var_lsdrain_i * var_qjunsti_d_dn8)) + (var_lgdrain_i * var_qjungat_d_dn8));
        var_qjun_d_dn10 = (((var_abdrain_i * var_qjunbot_d_dn10) + (var_lsdrain_i * var_qjunsti_d_dn10)) + (var_lgdrain_i * var_qjungat_d_dn10));
        var_qjun_d_dn11 = (((var_abdrain_i * var_qjunbot_d_dn11) + (var_lsdrain_i * var_qjunsti_d_dn11)) + (var_lgdrain_i * var_qjungat_d_dn11));

        let assign62020_e80567: f64 = if var_sigvds < 0.0 { 1.0 } else { 0.0 };
        var_guard1727 = assign62020_e80567;

        let (assign62030_e80571, assign62030_e80571_d_n5, assign62030_e80571_d_n6, assign62030_e80571_d_n7, assign62030_e80571_d_n8,) = {
    if (var_guard1727 != 0.0) {
        (var_qd, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8,)
    } else {
        (var_temp__blk1726, var_temp__blk1726_dn5, var_temp__blk1726_dn6, var_temp__blk1726_dn7, var_temp__blk1726_dn8,)
    }
};
        var_temp__blk1726 = assign62030_e80571;
        var_temp__blk1726_dn5 = assign62030_e80571_d_n5;
        var_temp__blk1726_dn6 = assign62030_e80571_d_n6;
        var_temp__blk1726_dn7 = assign62030_e80571_d_n7;
        var_temp__blk1726_dn8 = assign62030_e80571_d_n8;

        let (assign62040_e80575, assign62040_e80575_d_n5, assign62040_e80575_d_n6, assign62040_e80575_d_n7, assign62040_e80575_d_n8,) = {
    if (var_guard1727 != 0.0) {
        (var_qs, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8,)
    } else {
        (var_qd, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8,)
    }
};
        var_qd = assign62040_e80575;
        var_qd_dn5 = assign62040_e80575_d_n5;
        var_qd_dn6 = assign62040_e80575_d_n6;
        var_qd_dn7 = assign62040_e80575_d_n7;
        var_qd_dn8 = assign62040_e80575_d_n8;

        let (assign62050_e80579, assign62050_e80579_d_n5, assign62050_e80579_d_n6, assign62050_e80579_d_n7, assign62050_e80579_d_n8,) = {
    if (var_guard1727 != 0.0) {
        (var_temp__blk1726, var_temp__blk1726_dn5, var_temp__blk1726_dn6, var_temp__blk1726_dn7, var_temp__blk1726_dn8,)
    } else {
        (var_qs, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8,)
    }
};
        var_qs = assign62050_e80579;
        var_qs_dn5 = assign62050_e80579_d_n5;
        var_qs_dn6 = assign62050_e80579_d_n6;
        var_qs_dn7 = assign62050_e80579_d_n7;
        var_qs_dn8 = assign62050_e80579_d_n8;

        var_sidexc = 0.0;
        var_sidexc_dn5 = 0.0;
        var_sidexc_dn6 = 0.0;
        var_sidexc_dn7 = 0.0;
        var_sidexc_dn8 = 0.0;

        var_mid = 0.0;
        var_mid_dn5 = 0.0;
        var_mid_dn6 = 0.0;
        var_mid_dn7 = 0.0;
        var_mid_dn8 = 0.0;

        var_mig = 1e-40;
        var_mig_dn5 = 0.0;
        var_mig_dn6 = 0.0;
        var_mig_dn7 = 0.0;
        var_mig_dn8 = 0.0;

        var_migid = 0.0;
        var_migid_dn5 = 0.0;
        var_migid_dn6 = 0.0;
        var_migid_dn7 = 0.0;
        var_migid_dn8 = 0.0;

        var_c_igid = 0.0;
        var_c_igid_dn5 = 0.0;
        var_c_igid_dn6 = 0.0;
        var_c_igid_dn7 = 0.0;
        var_c_igid_dn8 = 0.0;

        let assign62120_e80588: f64 = (var_cox_qm * var_eta_p_ac);
        var_cgeff = assign62120_e80588;
        var_cgeff_dn5 = ((var_cox_qm_dn5 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn5));
        var_cgeff_dn6 = ((var_cox_qm_dn6 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn6));
        var_cgeff_dn7 = ((var_cox_qm_dn7 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn7));
        var_cgeff_dn8 = ((var_cox_qm_dn8 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn8));

        var_sqid = 0.0;
        var_sqid_dn5 = 0.0;
        var_sqid_dn6 = 0.0;
        var_sqid_dn7 = 0.0;
        var_sqid_dn8 = 0.0;

        var_sqig = 0.0;
        var_sqig_dn5 = 0.0;
        var_sqig_dn6 = 0.0;
        var_sqig_dn7 = 0.0;
        var_sqig_dn8 = 0.0;

        let assign62180_e80600: f64 = if ((var_xg_dc > 0.0) && (var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard1760 = assign62180_e80600;

        let assign62270_e80706: f64 = if p.p32 > 0.0 { 1.0 } else { 0.0 };
        var_guard1762 = assign62270_e80706;

        let (assign62280_e80714, assign62280_e80714_d_n5, assign62280_e80714_d_n6, assign62280_e80714_d_n7, assign62280_e80714_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62280_e80712: f64 = (var_qim1_dc / var_alpha_dc);
        (assign62280_e80712, (((var_qim1_dc_dn5 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn5)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn6 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn6)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn7 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn7)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn8 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn8)) / (var_alpha_dc * var_alpha_dc)),)
    } else {
        (var_h0, var_h0_dn5, var_h0_dn6, var_h0_dn7, var_h0_dn8,)
    }
};
        var_h0 = assign62280_e80714;
        var_h0_dn5 = assign62280_e80714_d_n5;
        var_h0_dn6 = assign62280_e80714_d_n6;
        var_h0_dn7 = assign62280_e80714_d_n7;
        var_h0_dn8 = assign62280_e80714_d_n8;

        let (assign62290_e80722, assign62290_e80722_d_n5, assign62290_e80722_d_n6, assign62290_e80722_d_n7, assign62290_e80722_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62290_e80720: f64 = (var_qim_dc / var_qim1_dc);
        (assign62290_e80720, (((var_qim_dc_dn5 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn5)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn6 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn6)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn7 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn7)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn8 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn8)) / (var_qim1_dc * var_qim1_dc)),)
    } else {
        (var_t1, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign62290_e80722;
        var_t1_dn5 = assign62290_e80722_d_n5;
        var_t1_dn6 = assign62290_e80722_d_n6;
        var_t1_dn7 = assign62290_e80722_d_n7;
        var_t1_dn8 = assign62290_e80722_d_n8;

        *var_c_igid_slot = var_c_igid;
        *var_c_igid_dn5_slot = var_c_igid_dn5;
        *var_c_igid_dn6_slot = var_c_igid_dn6;
        *var_c_igid_dn7_slot = var_c_igid_dn7;
        *var_c_igid_dn8_slot = var_c_igid_dn8;
        *var_cgeff_slot = var_cgeff;
        *var_cgeff_dn5_slot = var_cgeff_dn5;
        *var_cgeff_dn6_slot = var_cgeff_dn6;
        *var_cgeff_dn7_slot = var_cgeff_dn7;
        *var_cgeff_dn8_slot = var_cgeff_dn8;
        *var_guard1716_slot = var_guard1716;
        *var_guard1717_slot = var_guard1717;
        *var_guard1718_slot = var_guard1718;
        *var_guard1719_slot = var_guard1719;
        *var_guard1720_slot = var_guard1720;
        *var_guard1721_slot = var_guard1721;
        *var_guard1722_slot = var_guard1722;
        *var_guard1723_slot = var_guard1723;
        *var_guard1724_slot = var_guard1724;
        *var_guard1725_slot = var_guard1725;
        *var_guard1727_slot = var_guard1727;
        *var_guard1760_slot = var_guard1760;
        *var_guard1762_slot = var_guard1762;
        *var_h0_slot = var_h0;
        *var_h0_dn5_slot = var_h0_dn5;
        *var_h0_dn6_slot = var_h0_dn6;
        *var_h0_dn7_slot = var_h0_dn7;
        *var_h0_dn8_slot = var_h0_dn8;
        *var_h1__blk1528_slot = var_h1__blk1528;
        *var_h2__blk1529_slot = var_h2__blk1529;
        *var_h2d__blk1530_slot = var_h2d__blk1530;
        *var_h2d__blk1530_dn10_slot = var_h2d__blk1530_dn10;
        *var_h2d__blk1530_dn11_slot = var_h2d__blk1530_dn11;
        *var_h2d__blk1530_dn6_slot = var_h2d__blk1530_dn6;
        *var_h2d__blk1530_dn7_slot = var_h2d__blk1530_dn7;
        *var_h3__blk1531_slot = var_h3__blk1531;
        *var_h3__blk1531_dn10_slot = var_h3__blk1531_dn10;
        *var_h3__blk1531_dn11_slot = var_h3__blk1531_dn11;
        *var_h3__blk1531_dn6_slot = var_h3__blk1531_dn6;
        *var_h3__blk1531_dn7_slot = var_h3__blk1531_dn7;
        *var_h4__blk1532_slot = var_h4__blk1532;
        *var_h4__blk1532_dn10_slot = var_h4__blk1532_dn10;
        *var_h4__blk1532_dn11_slot = var_h4__blk1532_dn11;
        *var_h4__blk1532_dn6_slot = var_h4__blk1532_dn6;
        *var_h4__blk1532_dn7_slot = var_h4__blk1532_dn7;
        *var_h5__blk1533_slot = var_h5__blk1533;
        *var_h5__blk1533_dn10_slot = var_h5__blk1533_dn10;
        *var_h5__blk1533_dn11_slot = var_h5__blk1533_dn11;
        *var_h5__blk1533_dn6_slot = var_h5__blk1533_dn6;
        *var_h5__blk1533_dn7_slot = var_h5__blk1533_dn7;
        *var_ijun_d_slot = var_ijun_d;
        *var_ijun_d_dn10_slot = var_ijun_d_dn10;
        *var_ijun_d_dn11_slot = var_ijun_d_dn11;
        *var_ijun_d_dn5_slot = var_ijun_d_dn5;
        *var_ijun_d_dn6_slot = var_ijun_d_dn6;
        *var_ijun_d_dn7_slot = var_ijun_d_dn7;
        *var_ijun_d_dn8_slot = var_ijun_d_dn8;
        *var_mid_slot = var_mid;
        *var_mid_dn5_slot = var_mid_dn5;
        *var_mid_dn6_slot = var_mid_dn6;
        *var_mid_dn7_slot = var_mid_dn7;
        *var_mid_dn8_slot = var_mid_dn8;
        *var_mig_slot = var_mig;
        *var_mig_dn5_slot = var_mig_dn5;
        *var_mig_dn6_slot = var_mig_dn6;
        *var_mig_dn7_slot = var_mig_dn7;
        *var_mig_dn8_slot = var_mig_dn8;
        *var_migid_slot = var_migid;
        *var_migid_dn5_slot = var_migid_dn5;
        *var_migid_dn6_slot = var_migid_dn6;
        *var_migid_dn7_slot = var_migid_dn7;
        *var_migid_dn8_slot = var_migid_dn8;
        *var_nu__blk1570_slot = var_nu__blk1570;
        *var_nu__blk1570_dn10_slot = var_nu__blk1570_dn10;
        *var_nu__blk1570_dn11_slot = var_nu__blk1570_dn11;
        *var_nu__blk1570_dn6_slot = var_nu__blk1570_dn6;
        *var_nu__blk1570_dn7_slot = var_nu__blk1570_dn7;
        *var_qd_slot = var_qd;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qfgd_slot = var_qfgd;
        *var_qfgd_dn5_slot = var_qfgd_dn5;
        *var_qfgd_dn6_slot = var_qfgd_dn6;
        *var_qfgd_dn7_slot = var_qfgd_dn7;
        *var_qfgs_slot = var_qfgs;
        *var_qfgs_dn5_slot = var_qfgs_dn5;
        *var_qfgs_dn6_slot = var_qfgs_dn6;
        *var_qfgs_dn7_slot = var_qfgs_dn7;
        *var_qjun_d_slot = var_qjun_d;
        *var_qjun_d_dn10_slot = var_qjun_d_dn10;
        *var_qjun_d_dn11_slot = var_qjun_d_dn11;
        *var_qjun_d_dn5_slot = var_qjun_d_dn5;
        *var_qjun_d_dn6_slot = var_qjun_d_dn6;
        *var_qjun_d_dn7_slot = var_qjun_d_dn7;
        *var_qjun_d_dn8_slot = var_qjun_d_dn8;
        *var_qjun_s_slot = var_qjun_s;
        *var_qjun_s_dn10_slot = var_qjun_s_dn10;
        *var_qjun_s_dn11_slot = var_qjun_s_dn11;
        *var_qjun_s_dn5_slot = var_qjun_s_dn5;
        *var_qjun_s_dn6_slot = var_qjun_s_dn6;
        *var_qjun_s_dn7_slot = var_qjun_s_dn7;
        *var_qjun_s_dn8_slot = var_qjun_s_dn8;
        *var_qjungat2nd_slot = var_qjungat2nd;
        *var_qjungat2nd_dn10_slot = var_qjungat2nd_dn10;
        *var_qjungat2nd_dn11_slot = var_qjungat2nd_dn11;
        *var_qjungat2nd_dn5_slot = var_qjungat2nd_dn5;
        *var_qjungat2nd_dn6_slot = var_qjungat2nd_dn6;
        *var_qjungat2nd_dn7_slot = var_qjungat2nd_dn7;
        *var_qjungat2nd_dn8_slot = var_qjungat2nd_dn8;
        *var_qjungat_d_slot = var_qjungat_d;
        *var_qjungat_d_dn10_slot = var_qjungat_d_dn10;
        *var_qjungat_d_dn11_slot = var_qjungat_d_dn11;
        *var_qjungat_d_dn5_slot = var_qjungat_d_dn5;
        *var_qjungat_d_dn6_slot = var_qjungat_d_dn6;
        *var_qjungat_d_dn7_slot = var_qjungat_d_dn7;
        *var_qjungat_d_dn8_slot = var_qjungat_d_dn8;
        *var_qs_slot = var_qs;
        *var_qs_dn5_slot = var_qs_dn5;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_sidexc_slot = var_sidexc;
        *var_sidexc_dn5_slot = var_sidexc_dn5;
        *var_sidexc_dn6_slot = var_sidexc_dn6;
        *var_sidexc_dn7_slot = var_sidexc_dn7;
        *var_sidexc_dn8_slot = var_sidexc_dn8;
        *var_sqid_slot = var_sqid;
        *var_sqid_dn5_slot = var_sqid_dn5;
        *var_sqid_dn6_slot = var_sqid_dn6;
        *var_sqid_dn7_slot = var_sqid_dn7;
        *var_sqid_dn8_slot = var_sqid_dn8;
        *var_sqig_slot = var_sqig;
        *var_sqig_dn5_slot = var_sqig_dn5;
        *var_sqig_dn6_slot = var_sqig_dn6;
        *var_sqig_dn7_slot = var_sqig_dn7;
        *var_sqig_dn8_slot = var_sqig_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_temp__blk1726_slot = var_temp__blk1726;
        *var_temp__blk1726_dn5_slot = var_temp__blk1726_dn5;
        *var_temp__blk1726_dn6_slot = var_temp__blk1726_dn6;
        *var_temp__blk1726_dn7_slot = var_temp__blk1726_dn7;
        *var_temp__blk1726_dn8_slot = var_temp__blk1726_dn8;
        *var_tmp__blk1543_slot = var_tmp__blk1543;
        *var_tmp__blk1543_dn10_slot = var_tmp__blk1543_dn10;
        *var_tmp__blk1543_dn11_slot = var_tmp__blk1543_dn11;
        *var_tmp__blk1543_dn5_slot = var_tmp__blk1543_dn5;
        *var_tmp__blk1543_dn6_slot = var_tmp__blk1543_dn6;
        *var_tmp__blk1543_dn7_slot = var_tmp__blk1543_dn7;
        *var_tmp__blk1543_dn8_slot = var_tmp__blk1543_dn8;
        *var_vjtmp_slot = var_vjtmp;
        *var_vjtmp_dn10_slot = var_vjtmp_dn10;
        *var_vjtmp_dn11_slot = var_vjtmp_dn11;
        *var_vjtmp_dn6_slot = var_vjtmp_dn6;
        *var_vjtmp_dn7_slot = var_vjtmp_dn7;
    }

    pub(super) fn stamp_transient_block_145(
        p: &Parameters,
        var_bet_i: f64,
        var_chnl_type: f64,
        var_cox_qm: f64,
        var_cox_qm_dn5: f64,
        var_cox_qm_dn6: f64,
        var_cox_qm_dn7: f64,
        var_cox_qm_dn8: f64,
        var_dps_dc: f64,
        var_dps_dc_dn5: f64,
        var_dps_dc_dn6: f64,
        var_dps_dc_dn7: f64,
        var_dps_dc_dn8: f64,
        var_eta_p_ac: f64,
        var_eta_p_ac_dn5: f64,
        var_eta_p_ac_dn6: f64,
        var_eta_p_ac_dn7: f64,
        var_eta_p_ac_dn8: f64,
        var_fac_exc: f64,
        var_fntexc_i: f64,
        var_gmob_dc: f64,
        var_gmob_dc_dn5: f64,
        var_gmob_dc_dn6: f64,
        var_gmob_dc_dn7: f64,
        var_gmob_dc_dn8: f64,
        var_gmob_dl_ac: f64,
        var_gmob_dl_ac_dn5: f64,
        var_gmob_dl_ac_dn6: f64,
        var_gmob_dl_ac_dn7: f64,
        var_gmob_dl_ac_dn8: f64,
        var_guard1760: f64,
        var_guard1762: f64,
        var_gvsat_ac: f64,
        var_gvsat_ac_dn5: f64,
        var_gvsat_ac_dn6: f64,
        var_gvsat_ac_dn7: f64,
        var_gvsat_ac_dn8: f64,
        var_gvsatinv_dc: f64,
        var_gvsatinv_dc_dn5: f64,
        var_gvsatinv_dc_dn6: f64,
        var_gvsatinv_dc_dn7: f64,
        var_gvsatinv_dc_dn8: f64,
        var_h0: f64,
        var_h0_dn5: f64,
        var_h0_dn6: f64,
        var_h0_dn7: f64,
        var_h0_dn8: f64,
        var_h_dc: f64,
        var_h_dc_dn5: f64,
        var_h_dc_dn6: f64,
        var_h_dc_dn7: f64,
        var_h_dc_dn8: f64,
        var_i_ds: f64,
        var_i_ds_dn5: f64,
        var_i_ds_dn6: f64,
        var_i_ds_dn7: f64,
        var_i_ds_dn8: f64,
        var_nt: f64,
        var_nt0: f64,
        var_qim1_dc: f64,
        var_qim1_dc_dn5: f64,
        var_qim1_dc_dn6: f64,
        var_qim1_dc_dn7: f64,
        var_qim1_dc_dn8: f64,
        var_t1: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_thesateff_dc: f64,
        var_thesateff_dc_dn5: f64,
        var_thesateff_dc_dn6: f64,
        var_thesateff_dc_dn7: f64,
        var_thesateff_dc_dn8: f64,
        var_vdse_dc: f64,
        var_vdse_dc_dn5: f64,
        var_vdse_dc_dn6: f64,
        var_vdse_dc_dn7: f64,
        var_vdse_dc_dn8: f64,
        var_c_igid_slot: &mut f64,
        var_c_igid_dn5_slot: &mut f64,
        var_c_igid_dn6_slot: &mut f64,
        var_c_igid_dn7_slot: &mut f64,
        var_c_igid_dn8_slot: &mut f64,
        var_cgeff_slot: &mut f64,
        var_cgeff_dn5_slot: &mut f64,
        var_cgeff_dn6_slot: &mut f64,
        var_cgeff_dn7_slot: &mut f64,
        var_cgeff_dn8_slot: &mut f64,
        var_g_ideal_slot: &mut f64,
        var_g_ideal_dn5_slot: &mut f64,
        var_g_ideal_dn6_slot: &mut f64,
        var_g_ideal_dn7_slot: &mut f64,
        var_g_ideal_dn8_slot: &mut f64,
        var_gfac_slot: &mut f64,
        var_gfac_dn5_slot: &mut f64,
        var_gfac_dn6_slot: &mut f64,
        var_gfac_dn7_slot: &mut f64,
        var_gfac_dn8_slot: &mut f64,
        var_guard1763_slot: &mut f64,
        var_guard1764_slot: &mut f64,
        var_guard1765_slot: &mut f64,
        var_guard1766_slot: &mut f64,
        var_guard1767_slot: &mut f64,
        var_gvsat_exc_slot: &mut f64,
        var_gvsat_exc_dn5_slot: &mut f64,
        var_gvsat_exc_dn6_slot: &mut f64,
        var_gvsat_exc_dn7_slot: &mut f64,
        var_gvsat_exc_dn8_slot: &mut f64,
        var_lc_slot: &mut f64,
        var_lc_dn5_slot: &mut f64,
        var_lc_dn6_slot: &mut f64,
        var_lc_dn7_slot: &mut f64,
        var_lc_dn8_slot: &mut f64,
        var_lcinv2_slot: &mut f64,
        var_lcinv2_dn5_slot: &mut f64,
        var_lcinv2_dn6_slot: &mut f64,
        var_lcinv2_dn7_slot: &mut f64,
        var_lcinv2_dn8_slot: &mut f64,
        var_mid_slot: &mut f64,
        var_mid_dn5_slot: &mut f64,
        var_mid_dn6_slot: &mut f64,
        var_mid_dn7_slot: &mut f64,
        var_mid_dn8_slot: &mut f64,
        var_mig_slot: &mut f64,
        var_mig_dn5_slot: &mut f64,
        var_mig_dn6_slot: &mut f64,
        var_mig_dn7_slot: &mut f64,
        var_mig_dn8_slot: &mut f64,
        var_migid0_slot: &mut f64,
        var_migid0_dn5_slot: &mut f64,
        var_migid0_dn6_slot: &mut f64,
        var_migid0_dn7_slot: &mut f64,
        var_migid0_dn8_slot: &mut f64,
        var_r_slot: &mut f64,
        var_r_dn5_slot: &mut f64,
        var_r_dn6_slot: &mut f64,
        var_r_dn7_slot: &mut f64,
        var_r_dn8_slot: &mut f64,
        var_sidexc_slot: &mut f64,
        var_sidexc_dn5_slot: &mut f64,
        var_sidexc_dn6_slot: &mut f64,
        var_sidexc_dn7_slot: &mut f64,
        var_sidexc_dn8_slot: &mut f64,
        var_sqid_slot: &mut f64,
        var_sqid_dn5_slot: &mut f64,
        var_sqid_dn6_slot: &mut f64,
        var_sqid_dn7_slot: &mut f64,
        var_sqid_dn8_slot: &mut f64,
        var_sqig_slot: &mut f64,
        var_sqig_dn5_slot: &mut f64,
        var_sqig_dn6_slot: &mut f64,
        var_sqig_dn7_slot: &mut f64,
        var_sqig_dn8_slot: &mut f64,
        var_sqt2_slot: &mut f64,
        var_sqt2_dn5_slot: &mut f64,
        var_sqt2_dn6_slot: &mut f64,
        var_sqt2_dn7_slot: &mut f64,
        var_sqt2_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_thesat1_exc_slot: &mut f64,
        var_thesat1_exc_dn5_slot: &mut f64,
        var_thesat1_exc_dn6_slot: &mut f64,
        var_thesat1_exc_dn7_slot: &mut f64,
        var_thesat1_exc_dn8_slot: &mut f64,
        var_zsat_exc_slot: &mut f64,
        var_zsat_exc_dn5_slot: &mut f64,
        var_zsat_exc_dn6_slot: &mut f64,
        var_zsat_exc_dn7_slot: &mut f64,
        var_zsat_exc_dn8_slot: &mut f64,
    ) {
        let mut var_c_igid: f64 = *var_c_igid_slot;
        let mut var_c_igid_dn5: f64 = *var_c_igid_dn5_slot;
        let mut var_c_igid_dn6: f64 = *var_c_igid_dn6_slot;
        let mut var_c_igid_dn7: f64 = *var_c_igid_dn7_slot;
        let mut var_c_igid_dn8: f64 = *var_c_igid_dn8_slot;
        let mut var_cgeff: f64 = *var_cgeff_slot;
        let mut var_cgeff_dn5: f64 = *var_cgeff_dn5_slot;
        let mut var_cgeff_dn6: f64 = *var_cgeff_dn6_slot;
        let mut var_cgeff_dn7: f64 = *var_cgeff_dn7_slot;
        let mut var_cgeff_dn8: f64 = *var_cgeff_dn8_slot;
        let mut var_g_ideal: f64 = *var_g_ideal_slot;
        let mut var_g_ideal_dn5: f64 = *var_g_ideal_dn5_slot;
        let mut var_g_ideal_dn6: f64 = *var_g_ideal_dn6_slot;
        let mut var_g_ideal_dn7: f64 = *var_g_ideal_dn7_slot;
        let mut var_g_ideal_dn8: f64 = *var_g_ideal_dn8_slot;
        let mut var_gfac: f64 = *var_gfac_slot;
        let mut var_gfac_dn5: f64 = *var_gfac_dn5_slot;
        let mut var_gfac_dn6: f64 = *var_gfac_dn6_slot;
        let mut var_gfac_dn7: f64 = *var_gfac_dn7_slot;
        let mut var_gfac_dn8: f64 = *var_gfac_dn8_slot;
        let mut var_guard1763: f64 = *var_guard1763_slot;
        let mut var_guard1764: f64 = *var_guard1764_slot;
        let mut var_guard1765: f64 = *var_guard1765_slot;
        let mut var_guard1766: f64 = *var_guard1766_slot;
        let mut var_guard1767: f64 = *var_guard1767_slot;
        let mut var_gvsat_exc: f64 = *var_gvsat_exc_slot;
        let mut var_gvsat_exc_dn5: f64 = *var_gvsat_exc_dn5_slot;
        let mut var_gvsat_exc_dn6: f64 = *var_gvsat_exc_dn6_slot;
        let mut var_gvsat_exc_dn7: f64 = *var_gvsat_exc_dn7_slot;
        let mut var_gvsat_exc_dn8: f64 = *var_gvsat_exc_dn8_slot;
        let mut var_lc: f64 = *var_lc_slot;
        let mut var_lc_dn5: f64 = *var_lc_dn5_slot;
        let mut var_lc_dn6: f64 = *var_lc_dn6_slot;
        let mut var_lc_dn7: f64 = *var_lc_dn7_slot;
        let mut var_lc_dn8: f64 = *var_lc_dn8_slot;
        let mut var_lcinv2: f64 = *var_lcinv2_slot;
        let mut var_lcinv2_dn5: f64 = *var_lcinv2_dn5_slot;
        let mut var_lcinv2_dn6: f64 = *var_lcinv2_dn6_slot;
        let mut var_lcinv2_dn7: f64 = *var_lcinv2_dn7_slot;
        let mut var_lcinv2_dn8: f64 = *var_lcinv2_dn8_slot;
        let mut var_mid: f64 = *var_mid_slot;
        let mut var_mid_dn5: f64 = *var_mid_dn5_slot;
        let mut var_mid_dn6: f64 = *var_mid_dn6_slot;
        let mut var_mid_dn7: f64 = *var_mid_dn7_slot;
        let mut var_mid_dn8: f64 = *var_mid_dn8_slot;
        let mut var_mig: f64 = *var_mig_slot;
        let mut var_mig_dn5: f64 = *var_mig_dn5_slot;
        let mut var_mig_dn6: f64 = *var_mig_dn6_slot;
        let mut var_mig_dn7: f64 = *var_mig_dn7_slot;
        let mut var_mig_dn8: f64 = *var_mig_dn8_slot;
        let mut var_migid0: f64 = *var_migid0_slot;
        let mut var_migid0_dn5: f64 = *var_migid0_dn5_slot;
        let mut var_migid0_dn6: f64 = *var_migid0_dn6_slot;
        let mut var_migid0_dn7: f64 = *var_migid0_dn7_slot;
        let mut var_migid0_dn8: f64 = *var_migid0_dn8_slot;
        let mut var_r: f64 = *var_r_slot;
        let mut var_r_dn5: f64 = *var_r_dn5_slot;
        let mut var_r_dn6: f64 = *var_r_dn6_slot;
        let mut var_r_dn7: f64 = *var_r_dn7_slot;
        let mut var_r_dn8: f64 = *var_r_dn8_slot;
        let mut var_sidexc: f64 = *var_sidexc_slot;
        let mut var_sidexc_dn5: f64 = *var_sidexc_dn5_slot;
        let mut var_sidexc_dn6: f64 = *var_sidexc_dn6_slot;
        let mut var_sidexc_dn7: f64 = *var_sidexc_dn7_slot;
        let mut var_sidexc_dn8: f64 = *var_sidexc_dn8_slot;
        let mut var_sqid: f64 = *var_sqid_slot;
        let mut var_sqid_dn5: f64 = *var_sqid_dn5_slot;
        let mut var_sqid_dn6: f64 = *var_sqid_dn6_slot;
        let mut var_sqid_dn7: f64 = *var_sqid_dn7_slot;
        let mut var_sqid_dn8: f64 = *var_sqid_dn8_slot;
        let mut var_sqig: f64 = *var_sqig_slot;
        let mut var_sqig_dn5: f64 = *var_sqig_dn5_slot;
        let mut var_sqig_dn6: f64 = *var_sqig_dn6_slot;
        let mut var_sqig_dn7: f64 = *var_sqig_dn7_slot;
        let mut var_sqig_dn8: f64 = *var_sqig_dn8_slot;
        let mut var_sqt2: f64 = *var_sqt2_slot;
        let mut var_sqt2_dn5: f64 = *var_sqt2_dn5_slot;
        let mut var_sqt2_dn6: f64 = *var_sqt2_dn6_slot;
        let mut var_sqt2_dn7: f64 = *var_sqt2_dn7_slot;
        let mut var_sqt2_dn8: f64 = *var_sqt2_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_thesat1_exc: f64 = *var_thesat1_exc_slot;
        let mut var_thesat1_exc_dn5: f64 = *var_thesat1_exc_dn5_slot;
        let mut var_thesat1_exc_dn6: f64 = *var_thesat1_exc_dn6_slot;
        let mut var_thesat1_exc_dn7: f64 = *var_thesat1_exc_dn7_slot;
        let mut var_thesat1_exc_dn8: f64 = *var_thesat1_exc_dn8_slot;
        let mut var_zsat_exc: f64 = *var_zsat_exc_slot;
        let mut var_zsat_exc_dn5: f64 = *var_zsat_exc_dn5_slot;
        let mut var_zsat_exc_dn6: f64 = *var_zsat_exc_dn6_slot;
        let mut var_zsat_exc_dn7: f64 = *var_zsat_exc_dn7_slot;
        let mut var_zsat_exc_dn8: f64 = *var_zsat_exc_dn8_slot;

        let (assign62300_e80734, assign62300_e80734_d_n5, assign62300_e80734_d_n6, assign62300_e80734_d_n7, assign62300_e80734_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62300_e80728: f64 = (0.5 * 0.16666666666666666);
        let assign62300_e80731: f64 = (var_dps_dc / var_h0);
        let assign62300_e80732: f64 = (assign62300_e80728 * assign62300_e80731);
        (assign62300_e80732, (assign62300_e80728 * (((var_dps_dc_dn5 * var_h0) - (var_dps_dc * var_h0_dn5)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn6 * var_h0) - (var_dps_dc * var_h0_dn6)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn7 * var_h0) - (var_dps_dc * var_h0_dn7)) / (var_h0 * var_h0))), (assign62300_e80728 * (((var_dps_dc_dn8 * var_h0) - (var_dps_dc * var_h0_dn8)) / (var_h0 * var_h0))),)
    } else {
        (var_sqt2, var_sqt2_dn5, var_sqt2_dn6, var_sqt2_dn7, var_sqt2_dn8,)
    }
};
        var_sqt2 = assign62300_e80734;
        var_sqt2_dn5 = assign62300_e80734_d_n5;
        var_sqt2_dn6 = assign62300_e80734_d_n6;
        var_sqt2_dn7 = assign62300_e80734_d_n7;
        var_sqt2_dn8 = assign62300_e80734_d_n8;

        let (assign62310_e80742, assign62310_e80742_d_n5, assign62310_e80742_d_n6, assign62310_e80742_d_n7, assign62310_e80742_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62310_e80740: f64 = (var_sqt2 * var_sqt2);
        (assign62310_e80740, ((var_sqt2_dn5 * var_sqt2) + (var_sqt2 * var_sqt2_dn5)), ((var_sqt2_dn6 * var_sqt2) + (var_sqt2 * var_sqt2_dn6)), ((var_sqt2_dn7 * var_sqt2) + (var_sqt2 * var_sqt2_dn7)), ((var_sqt2_dn8 * var_sqt2) + (var_sqt2 * var_sqt2_dn8)),)
    } else {
        (var_t2, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign62310_e80742;
        var_t2_dn5 = assign62310_e80742_d_n5;
        var_t2_dn6 = assign62310_e80742_d_n6;
        var_t2_dn7 = assign62310_e80742_d_n7;
        var_t2_dn8 = assign62310_e80742_d_n8;

        let (assign62320_e80752, assign62320_e80752_d_n5, assign62320_e80752_d_n6, assign62320_e80752_d_n7, assign62320_e80752_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62320_e80748: f64 = (var_h0 / var_h_dc);
        let assign62320_e80750: f64 = (assign62320_e80748 - 1.0);
        (assign62320_e80750, (((var_h0_dn5 * var_h_dc) - (var_h0 * var_h_dc_dn5)) / (var_h_dc * var_h_dc)), (((var_h0_dn6 * var_h_dc) - (var_h0 * var_h_dc_dn6)) / (var_h_dc * var_h_dc)), (((var_h0_dn7 * var_h_dc) - (var_h0 * var_h_dc_dn7)) / (var_h_dc * var_h_dc)), (((var_h0_dn8 * var_h_dc) - (var_h0 * var_h_dc_dn8)) / (var_h_dc * var_h_dc)),)
    } else {
        (var_r, var_r_dn5, var_r_dn6, var_r_dn7, var_r_dn8,)
    }
};
        var_r = assign62320_e80752;
        var_r_dn5 = assign62320_e80752_d_n5;
        var_r_dn6 = assign62320_e80752_d_n6;
        var_r_dn7 = assign62320_e80752_d_n7;
        var_r_dn8 = assign62320_e80752_d_n8;

        let (assign62330_e80775, assign62330_e80775_d_n5, assign62330_e80775_d_n6, assign62330_e80775_d_n7, assign62330_e80775_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62330_e80760: f64 = (var_r * var_t2);
        let assign62330_e80761: f64 = (12.0 * assign62330_e80760);
        let assign62330_e80762: f64 = (1.0 - assign62330_e80761);
        let (assign62330_e80773, assign62330_e80773_d_n5, assign62330_e80773_d_n6, assign62330_e80773_d_n7, assign62330_e80773_d_n8,) = {
            if (assign62330_e80762 > 1e-20) {
                let assign62330_e80769: f64 = (var_r * var_t2);
                let assign62330_e80770: f64 = (12.0 * assign62330_e80769);
                let assign62330_e80771: f64 = (1.0 - assign62330_e80770);
                (assign62330_e80771, (-(12.0 * ((var_r_dn5 * var_t2) + (var_r * var_t2_dn5)))), (-(12.0 * ((var_r_dn6 * var_t2) + (var_r * var_t2_dn6)))), (-(12.0 * ((var_r_dn7 * var_t2) + (var_r * var_t2_dn7)))), (-(12.0 * ((var_r_dn8 * var_t2) + (var_r * var_t2_dn8)))),)
            } else {
                (1e-20, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62330_e80773, assign62330_e80773_d_n5, assign62330_e80773_d_n6, assign62330_e80773_d_n7, assign62330_e80773_d_n8,)
    } else {
        (var_lc, var_lc_dn5, var_lc_dn6, var_lc_dn7, var_lc_dn8,)
    }
};
        var_lc = assign62330_e80775;
        var_lc_dn5 = assign62330_e80775_d_n5;
        var_lc_dn6 = assign62330_e80775_d_n6;
        var_lc_dn7 = assign62330_e80775_d_n7;
        var_lc_dn8 = assign62330_e80775_d_n8;

        let (assign62340_e80785, assign62340_e80785_d_n5, assign62340_e80785_d_n6, assign62340_e80785_d_n7, assign62340_e80785_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62340_e80782: f64 = (var_lc * var_lc);
        let assign62340_e80783: f64 = (1.0 / assign62340_e80782);
        (assign62340_e80783, (-(((var_lc_dn5 * var_lc) + (var_lc * var_lc_dn5)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn6 * var_lc) + (var_lc * var_lc_dn6)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn7 * var_lc) + (var_lc * var_lc_dn7)) / (assign62340_e80782 * assign62340_e80782))), (-(((var_lc_dn8 * var_lc) + (var_lc * var_lc_dn8)) / (assign62340_e80782 * assign62340_e80782))),)
    } else {
        (var_lcinv2, var_lcinv2_dn5, var_lcinv2_dn6, var_lcinv2_dn7, var_lcinv2_dn8,)
    }
};
        var_lcinv2 = assign62340_e80785;
        var_lcinv2_dn5 = assign62340_e80785_d_n5;
        var_lcinv2_dn6 = assign62340_e80785_d_n6;
        var_lcinv2_dn7 = assign62340_e80785_d_n7;
        var_lcinv2_dn8 = assign62340_e80785_d_n8;

        let (assign62350_e80795, assign62350_e80795_d_n5, assign62350_e80795_d_n6, assign62350_e80795_d_n7, assign62350_e80795_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62350_e80791: f64 = (var_bet_i * var_qim1_dc);
        let assign62350_e80793: f64 = (assign62350_e80791 * var_gvsatinv_dc);
        (assign62350_e80793, (((var_bet_i * var_qim1_dc_dn5) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn5)), (((var_bet_i * var_qim1_dc_dn6) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn6)), (((var_bet_i * var_qim1_dc_dn7) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn7)), (((var_bet_i * var_qim1_dc_dn8) * var_gvsatinv_dc) + (assign62350_e80791 * var_gvsatinv_dc_dn8)),)
    } else {
        (var_g_ideal, var_g_ideal_dn5, var_g_ideal_dn6, var_g_ideal_dn7, var_g_ideal_dn8,)
    }
};
        var_g_ideal = assign62350_e80795;
        var_g_ideal_dn5 = assign62350_e80795_d_n5;
        var_g_ideal_dn6 = assign62350_e80795_d_n6;
        var_g_ideal_dn7 = assign62350_e80795_d_n7;
        var_g_ideal_dn8 = assign62350_e80795_d_n8;

        let (assign62360_e80815, assign62360_e80815_d_n5, assign62360_e80815_d_n6, assign62360_e80815_d_n7, assign62360_e80815_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62360_e80802: f64 = (12.0 * var_t2);
        let assign62360_e80803: f64 = (var_t1 + assign62360_e80802);
        let assign62360_e80807: f64 = (1.0 + var_t1);
        let assign62360_e80809: f64 = (assign62360_e80807 * var_t2);
        let assign62360_e80811: f64 = (assign62360_e80809 * var_r);
        let assign62360_e80812: f64 = (24.0 * assign62360_e80811);
        let assign62360_e80813: f64 = (assign62360_e80803 - assign62360_e80812);
        (assign62360_e80813, ((var_t1_dn5 + (12.0 * var_t2_dn5)) - (24.0 * ((((var_t1_dn5 * var_t2) + (assign62360_e80807 * var_t2_dn5)) * var_r) + (assign62360_e80809 * var_r_dn5)))), ((var_t1_dn6 + (12.0 * var_t2_dn6)) - (24.0 * ((((var_t1_dn6 * var_t2) + (assign62360_e80807 * var_t2_dn6)) * var_r) + (assign62360_e80809 * var_r_dn6)))), ((var_t1_dn7 + (12.0 * var_t2_dn7)) - (24.0 * ((((var_t1_dn7 * var_t2) + (assign62360_e80807 * var_t2_dn7)) * var_r) + (assign62360_e80809 * var_r_dn7)))), ((var_t1_dn8 + (12.0 * var_t2_dn8)) - (24.0 * ((((var_t1_dn8 * var_t2) + (assign62360_e80807 * var_t2_dn8)) * var_r) + (assign62360_e80809 * var_r_dn8)))),)
    } else {
        (var_mid, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8,)
    }
};
        var_mid = assign62360_e80815;
        var_mid_dn5 = assign62360_e80815_d_n5;
        var_mid_dn6 = assign62360_e80815_d_n6;
        var_mid_dn7 = assign62360_e80815_d_n7;
        var_mid_dn8 = assign62360_e80815_d_n8;

        let (assign62370_e80826, assign62370_e80826_d_n5, assign62370_e80826_d_n6, assign62370_e80826_d_n7, assign62370_e80826_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let (assign62370_e80824, assign62370_e80824_d_n5, assign62370_e80824_d_n6, assign62370_e80824_d_n7, assign62370_e80824_d_n8,) = {
            if (var_mid > 1e-40) {
                (var_mid, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62370_e80824, assign62370_e80824_d_n5, assign62370_e80824_d_n6, assign62370_e80824_d_n7, assign62370_e80824_d_n8,)
    } else {
        (var_mid, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8,)
    }
};
        var_mid = assign62370_e80826;
        var_mid_dn5 = assign62370_e80826_d_n5;
        var_mid_dn6 = assign62370_e80826_d_n6;
        var_mid_dn7 = assign62370_e80826_d_n7;
        var_mid_dn8 = assign62370_e80826_d_n8;

        let (assign62380_e80836, assign62380_e80836_d_n5, assign62380_e80836_d_n6, assign62380_e80836_d_n7, assign62380_e80836_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62380_e80832: f64 = (var_g_ideal * var_lcinv2);
        let assign62380_e80834: f64 = (assign62380_e80832 * var_mid);
        (assign62380_e80834, ((((var_g_ideal_dn5 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn5)) * var_mid) + (assign62380_e80832 * var_mid_dn5)), ((((var_g_ideal_dn6 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn6)) * var_mid) + (assign62380_e80832 * var_mid_dn6)), ((((var_g_ideal_dn7 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn7)) * var_mid) + (assign62380_e80832 * var_mid_dn7)), ((((var_g_ideal_dn8 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn8)) * var_mid) + (assign62380_e80832 * var_mid_dn8)),)
    } else {
        (var_mid, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8,)
    }
};
        var_mid = assign62380_e80836;
        var_mid_dn5 = assign62380_e80836_d_n5;
        var_mid_dn6 = assign62380_e80836_d_n6;
        var_mid_dn7 = assign62380_e80836_d_n7;
        var_mid_dn8 = assign62380_e80836_d_n8;

        let assign62390_e80839: f64 = if var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1763 = assign62390_e80839;

        let (assign62400_e80849, assign62400_e80849_d_n5, assign62400_e80849_d_n6, assign62400_e80849_d_n7, assign62400_e80849_d_n8,) = {
    if (((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) {
        let assign62400_e80847: f64 = (var_thesateff_dc / var_gmob_dc);
        (assign62400_e80847, (((var_thesateff_dc_dn5 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn5)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn6 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn6)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn7 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn7)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn8 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn8)) / (var_gmob_dc * var_gmob_dc)),)
    } else {
        (var_thesat1_exc, var_thesat1_exc_dn5, var_thesat1_exc_dn6, var_thesat1_exc_dn7, var_thesat1_exc_dn8,)
    }
};
        var_thesat1_exc = assign62400_e80849;
        var_thesat1_exc_dn5 = assign62400_e80849_d_n5;
        var_thesat1_exc_dn6 = assign62400_e80849_d_n6;
        var_thesat1_exc_dn7 = assign62400_e80849_d_n7;
        var_thesat1_exc_dn8 = assign62400_e80849_d_n8;

        let (assign62410_e80863, assign62410_e80863_d_n5, assign62410_e80863_d_n6, assign62410_e80863_d_n7, assign62410_e80863_d_n8,) = {
    if (((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) {
        let assign62410_e80857: f64 = (var_thesat1_exc * var_thesat1_exc);
        let assign62410_e80859: f64 = (assign62410_e80857 * var_dps_dc);
        let assign62410_e80861: f64 = (assign62410_e80859 * var_dps_dc);
        (assign62410_e80861, ((((((var_thesat1_exc_dn5 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn5)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn5)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn5)), ((((((var_thesat1_exc_dn6 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn6)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn6)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn6)), ((((((var_thesat1_exc_dn7 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn7)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn7)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn7)), ((((((var_thesat1_exc_dn8 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn8)) * var_dps_dc) + (assign62410_e80857 * var_dps_dc_dn8)) * var_dps_dc) + (assign62410_e80859 * var_dps_dc_dn8)),)
    } else {
        (var_zsat_exc, var_zsat_exc_dn5, var_zsat_exc_dn6, var_zsat_exc_dn7, var_zsat_exc_dn8,)
    }
};
        var_zsat_exc = assign62410_e80863;
        var_zsat_exc_dn5 = assign62410_e80863_d_n5;
        var_zsat_exc_dn6 = assign62410_e80863_d_n6;
        var_zsat_exc_dn7 = assign62410_e80863_d_n7;
        var_zsat_exc_dn8 = assign62410_e80863_d_n8;

        let assign62420_e80866: f64 = (-1.0);
        let assign62420_e80867: f64 = if var_chnl_type == assign62420_e80866 { 1.0 } else { 0.0 };
        var_guard1764 = assign62420_e80867;

        let (assign62430_e80883, assign62430_e80883_d_n5, assign62430_e80883_d_n6, assign62430_e80883_d_n7, assign62430_e80883_d_n8,) = {
    if ((((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) && (var_guard1764 != 0.0)) {
        let assign62430_e80879: f64 = (var_thesat1_exc * var_dps_dc);
        let assign62430_e80880: f64 = (1.0 + assign62430_e80879);
        let assign62430_e80881: f64 = (var_zsat_exc / assign62430_e80880);
        (assign62430_e80881, (((var_zsat_exc_dn5 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn5 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn5)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn6 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn6 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn6)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn7 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn7 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn7)))) / (assign62430_e80880 * assign62430_e80880)), (((var_zsat_exc_dn8 * assign62430_e80880) - (var_zsat_exc * ((var_thesat1_exc_dn8 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn8)))) / (assign62430_e80880 * assign62430_e80880)),)
    } else {
        (var_zsat_exc, var_zsat_exc_dn5, var_zsat_exc_dn6, var_zsat_exc_dn7, var_zsat_exc_dn8,)
    }
};
        var_zsat_exc = assign62430_e80883;
        var_zsat_exc_dn5 = assign62430_e80883_d_n5;
        var_zsat_exc_dn6 = assign62430_e80883_d_n6;
        var_zsat_exc_dn7 = assign62430_e80883_d_n7;
        var_zsat_exc_dn8 = assign62430_e80883_d_n8;

        let (assign62440_e80902, assign62440_e80902_d_n5, assign62440_e80902_d_n6, assign62440_e80902_d_n7, assign62440_e80902_d_n8,) = {
    if (((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) {
        let assign62440_e80895: f64 = (2.0 * var_zsat_exc);
        let assign62440_e80896: f64 = (1.0 + assign62440_e80895);
        let assign62440_e80897: f64 = (assign62440_e80896).sqrt();
        let assign62440_e80898: f64 = (1.0 + assign62440_e80897);
        let assign62440_e80899: f64 = (var_gmob_dc * assign62440_e80898);
        let assign62440_e80900: f64 = (0.5 * assign62440_e80899);
        (assign62440_e80900, (0.5 * ((var_gmob_dc_dn5 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn5) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn6 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn6) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn7 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn7) / (2.0 * assign62440_e80897))))), (0.5 * ((var_gmob_dc_dn8 * assign62440_e80898) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn8) / (2.0 * assign62440_e80897))))),)
    } else {
        (var_gvsat_exc, var_gvsat_exc_dn5, var_gvsat_exc_dn6, var_gvsat_exc_dn7, var_gvsat_exc_dn8,)
    }
};
        var_gvsat_exc = assign62440_e80902;
        var_gvsat_exc_dn5 = assign62440_e80902_d_n5;
        var_gvsat_exc_dn6 = assign62440_e80902_d_n6;
        var_gvsat_exc_dn7 = assign62440_e80902_d_n7;
        var_gvsat_exc_dn8 = assign62440_e80902_d_n8;

        let (assign62450_e80914, assign62450_e80914_d_n5, assign62450_e80914_d_n6, assign62450_e80914_d_n7, assign62450_e80914_d_n8,) = {
    if (((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) {
        let assign62450_e80911: f64 = (var_gvsat_exc * var_lc);
        let assign62450_e80912: f64 = (var_gmob_dc / assign62450_e80911);
        (assign62450_e80912, (((var_gmob_dc_dn5 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn5 * var_lc) + (var_gvsat_exc * var_lc_dn5)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn6 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn6 * var_lc) + (var_gvsat_exc * var_lc_dn6)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn7 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn7 * var_lc) + (var_gvsat_exc * var_lc_dn7)))) / (assign62450_e80911 * assign62450_e80911)), (((var_gmob_dc_dn8 * assign62450_e80911) - (var_gmob_dc * ((var_gvsat_exc_dn8 * var_lc) + (var_gvsat_exc * var_lc_dn8)))) / (assign62450_e80911 * assign62450_e80911)),)
    } else {
        (var_gfac, var_gfac_dn5, var_gfac_dn6, var_gfac_dn7, var_gfac_dn8,)
    }
};
        var_gfac = assign62450_e80914;
        var_gfac_dn5 = assign62450_e80914_d_n5;
        var_gfac_dn6 = assign62450_e80914_d_n6;
        var_gfac_dn7 = assign62450_e80914_d_n7;
        var_gfac_dn8 = assign62450_e80914_d_n8;

        let (assign62460_e80930, assign62460_e80930_d_n5, assign62460_e80930_d_n6, assign62460_e80930_d_n7, assign62460_e80930_d_n8,) = {
    if (((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) {
        let assign62460_e80922: f64 = (var_fac_exc * var_i_ds);
        let assign62460_e80924: f64 = (assign62460_e80922 * var_vdse_dc);
        let assign62460_e80926: f64 = (assign62460_e80924 * var_gfac);
        let assign62460_e80928: f64 = (assign62460_e80926 * var_gfac);
        (assign62460_e80928, (((((((var_fac_exc * var_i_ds_dn5) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn5)) * var_gfac) + (assign62460_e80924 * var_gfac_dn5)) * var_gfac) + (assign62460_e80926 * var_gfac_dn5)), (((((((var_fac_exc * var_i_ds_dn6) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn6)) * var_gfac) + (assign62460_e80924 * var_gfac_dn6)) * var_gfac) + (assign62460_e80926 * var_gfac_dn6)), (((((((var_fac_exc * var_i_ds_dn7) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn7)) * var_gfac) + (assign62460_e80924 * var_gfac_dn7)) * var_gfac) + (assign62460_e80926 * var_gfac_dn7)), (((((((var_fac_exc * var_i_ds_dn8) * var_vdse_dc) + (assign62460_e80922 * var_vdse_dc_dn8)) * var_gfac) + (assign62460_e80924 * var_gfac_dn8)) * var_gfac) + (assign62460_e80926 * var_gfac_dn8)),)
    } else {
        (var_sidexc, var_sidexc_dn5, var_sidexc_dn6, var_sidexc_dn7, var_sidexc_dn8,)
    }
};
        var_sidexc = assign62460_e80930;
        var_sidexc_dn5 = assign62460_e80930_d_n5;
        var_sidexc_dn6 = assign62460_e80930_d_n6;
        var_sidexc_dn7 = assign62460_e80930_d_n7;
        var_sidexc_dn8 = assign62460_e80930_d_n8;

        let (assign62470_e80942, assign62470_e80942_d_n5, assign62470_e80942_d_n6, assign62470_e80942_d_n7, assign62470_e80942_d_n8,) = {
    if (((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) && (var_guard1763 != 0.0)) {
        let assign62470_e80939: f64 = (var_sidexc / var_nt0);
        let assign62470_e80940: f64 = (var_mid + assign62470_e80939);
        (assign62470_e80940, (var_mid_dn5 + (var_sidexc_dn5 / var_nt0)), (var_mid_dn6 + (var_sidexc_dn6 / var_nt0)), (var_mid_dn7 + (var_sidexc_dn7 / var_nt0)), (var_mid_dn8 + (var_sidexc_dn8 / var_nt0)),)
    } else {
        (var_mid, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8,)
    }
};
        var_mid = assign62470_e80942;
        var_mid_dn5 = assign62470_e80942_d_n5;
        var_mid_dn6 = assign62470_e80942_d_n6;
        var_mid_dn7 = assign62470_e80942_d_n7;
        var_mid_dn8 = assign62470_e80942_d_n8;

        let (assign62480_e80951, assign62480_e80951_d_n5, assign62480_e80951_d_n6, assign62480_e80951_d_n7, assign62480_e80951_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1762 != 0.0)) {
        let assign62480_e80948: f64 = (var_nt * var_mid);
        let assign62480_e80949: f64 = (assign62480_e80948).sqrt();
        (assign62480_e80949, ((var_nt * var_mid_dn5) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn6) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn7) / (2.0 * assign62480_e80949)), ((var_nt * var_mid_dn8) / (2.0 * assign62480_e80949)),)
    } else {
        (var_sqid, var_sqid_dn5, var_sqid_dn6, var_sqid_dn7, var_sqid_dn8,)
    }
};
        var_sqid = assign62480_e80951;
        var_sqid_dn5 = assign62480_e80951_d_n5;
        var_sqid_dn6 = assign62480_e80951_d_n6;
        var_sqid_dn7 = assign62480_e80951_d_n7;
        var_sqid_dn8 = assign62480_e80951_d_n8;

        let assign62490_e80966: f64 = if ((((p.p50 == 1.0) && (var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1765 = assign62490_e80966;

        let (assign62500_e80998, assign62500_e80998_d_n5, assign62500_e80998_d_n6, assign62500_e80998_d_n7, assign62500_e80998_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let assign62500_e80972: f64 = (var_t1 / 12.0);
        let assign62500_e80976: f64 = (var_t1 + 0.2);
        let assign62500_e80979: f64 = (12.0 * var_t2);
        let assign62500_e80980: f64 = (assign62500_e80976 - assign62500_e80979);
        let assign62500_e80981: f64 = (var_t2 * assign62500_e80980);
        let assign62500_e80982: f64 = (assign62500_e80972 - assign62500_e80981);
        let assign62500_e80987: f64 = (var_t1 + 1.0);
        let assign62500_e80990: f64 = (12.0 * var_t2);
        let assign62500_e80991: f64 = (assign62500_e80987 - assign62500_e80990);
        let assign62500_e80992: f64 = (var_t2 * assign62500_e80991);
        let assign62500_e80994: f64 = (assign62500_e80992 * var_r);
        let assign62500_e80995: f64 = (1.6 * assign62500_e80994);
        let assign62500_e80996: f64 = (assign62500_e80982 - assign62500_e80995);
        (assign62500_e80996, (((var_t1_dn5 / 12.0) - ((var_t2_dn5 * assign62500_e80980) + (var_t2 * (var_t1_dn5 - (12.0 * var_t2_dn5))))) - (1.6 * ((((var_t2_dn5 * assign62500_e80991) + (var_t2 * (var_t1_dn5 - (12.0 * var_t2_dn5)))) * var_r) + (assign62500_e80992 * var_r_dn5)))), (((var_t1_dn6 / 12.0) - ((var_t2_dn6 * assign62500_e80980) + (var_t2 * (var_t1_dn6 - (12.0 * var_t2_dn6))))) - (1.6 * ((((var_t2_dn6 * assign62500_e80991) + (var_t2 * (var_t1_dn6 - (12.0 * var_t2_dn6)))) * var_r) + (assign62500_e80992 * var_r_dn6)))), (((var_t1_dn7 / 12.0) - ((var_t2_dn7 * assign62500_e80980) + (var_t2 * (var_t1_dn7 - (12.0 * var_t2_dn7))))) - (1.6 * ((((var_t2_dn7 * assign62500_e80991) + (var_t2 * (var_t1_dn7 - (12.0 * var_t2_dn7)))) * var_r) + (assign62500_e80992 * var_r_dn7)))), (((var_t1_dn8 / 12.0) - ((var_t2_dn8 * assign62500_e80980) + (var_t2 * (var_t1_dn8 - (12.0 * var_t2_dn8))))) - (1.6 * ((((var_t2_dn8 * assign62500_e80991) + (var_t2 * (var_t1_dn8 - (12.0 * var_t2_dn8)))) * var_r) + (assign62500_e80992 * var_r_dn8)))),)
    } else {
        (var_mig, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8,)
    }
};
        var_mig = assign62500_e80998;
        var_mig_dn5 = assign62500_e80998_d_n5;
        var_mig_dn6 = assign62500_e80998_d_n6;
        var_mig_dn7 = assign62500_e80998_d_n7;
        var_mig_dn8 = assign62500_e80998_d_n8;

        let (assign62510_e81009, assign62510_e81009_d_n5, assign62510_e81009_d_n6, assign62510_e81009_d_n7, assign62510_e81009_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let (assign62510_e81007, assign62510_e81007_d_n5, assign62510_e81007_d_n6, assign62510_e81007_d_n7, assign62510_e81007_d_n8,) = {
            if (var_mig > 1e-40) {
                (var_mig, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62510_e81007, assign62510_e81007_d_n5, assign62510_e81007_d_n6, assign62510_e81007_d_n7, assign62510_e81007_d_n8,)
    } else {
        (var_mig, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8,)
    }
};
        var_mig = assign62510_e81009;
        var_mig_dn5 = assign62510_e81009_d_n5;
        var_mig_dn6 = assign62510_e81009_d_n6;
        var_mig_dn7 = assign62510_e81009_d_n7;
        var_mig_dn8 = assign62510_e81009_d_n8;

        let (assign62520_e81019, assign62520_e81019_d_n5, assign62520_e81019_d_n6, assign62520_e81019_d_n7, assign62520_e81019_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let assign62520_e81015: f64 = (var_lcinv2 / var_g_ideal);
        let assign62520_e81017: f64 = (assign62520_e81015 * var_mig);
        (assign62520_e81017, (((((var_lcinv2_dn5 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn5)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn5)), (((((var_lcinv2_dn6 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn6)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn6)), (((((var_lcinv2_dn7 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn7)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn7)), (((((var_lcinv2_dn8 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn8)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62520_e81015 * var_mig_dn8)),)
    } else {
        (var_mig, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8,)
    }
};
        var_mig = assign62520_e81019;
        var_mig_dn5 = assign62520_e81019_d_n5;
        var_mig_dn6 = assign62520_e81019_d_n6;
        var_mig_dn7 = assign62520_e81019_d_n7;
        var_mig_dn8 = assign62520_e81019_d_n8;

        let (assign62530_e81047, assign62530_e81047_d_n5, assign62530_e81047_d_n6, assign62530_e81047_d_n7, assign62530_e81047_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let assign62530_e81025: f64 = (var_lcinv2 * var_sqt2);
        let assign62530_e81029: f64 = (12.0 * var_t2);
        let assign62530_e81030: f64 = (1.0 - assign62530_e81029);
        let assign62530_e81034: f64 = (19.2 * var_t2);
        let assign62530_e81035: f64 = (var_t1 + assign62530_e81034);
        let assign62530_e81039: f64 = (var_t1 * var_t2);
        let assign62530_e81040: f64 = (12.0 * assign62530_e81039);
        let assign62530_e81041: f64 = (assign62530_e81035 - assign62530_e81040);
        let assign62530_e81043: f64 = (assign62530_e81041 * var_r);
        let assign62530_e81044: f64 = (assign62530_e81030 - assign62530_e81043);
        let assign62530_e81045: f64 = (assign62530_e81025 * assign62530_e81044);
        (assign62530_e81045, ((((var_lcinv2_dn5 * var_sqt2) + (var_lcinv2 * var_sqt2_dn5)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn5)) - ((((var_t1_dn5 + (19.2 * var_t2_dn5)) - (12.0 * ((var_t1_dn5 * var_t2) + (var_t1 * var_t2_dn5)))) * var_r) + (assign62530_e81041 * var_r_dn5))))), ((((var_lcinv2_dn6 * var_sqt2) + (var_lcinv2 * var_sqt2_dn6)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn6)) - ((((var_t1_dn6 + (19.2 * var_t2_dn6)) - (12.0 * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * var_r) + (assign62530_e81041 * var_r_dn6))))), ((((var_lcinv2_dn7 * var_sqt2) + (var_lcinv2 * var_sqt2_dn7)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn7)) - ((((var_t1_dn7 + (19.2 * var_t2_dn7)) - (12.0 * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * var_r) + (assign62530_e81041 * var_r_dn7))))), ((((var_lcinv2_dn8 * var_sqt2) + (var_lcinv2 * var_sqt2_dn8)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * var_t2_dn8)) - ((((var_t1_dn8 + (19.2 * var_t2_dn8)) - (12.0 * ((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8)))) * var_r) + (assign62530_e81041 * var_r_dn8))))),)
    } else {
        (var_migid0, var_migid0_dn5, var_migid0_dn6, var_migid0_dn7, var_migid0_dn8,)
    }
};
        var_migid0 = assign62530_e81047;
        var_migid0_dn5 = assign62530_e81047_d_n5;
        var_migid0_dn6 = assign62530_e81047_d_n6;
        var_migid0_dn7 = assign62530_e81047_d_n7;
        var_migid0_dn8 = assign62530_e81047_d_n8;

        let (assign62540_e81063, assign62540_e81063_d_n5, assign62540_e81063_d_n6, assign62540_e81063_d_n7, assign62540_e81063_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let assign62540_e81053: f64 = (var_gvsat_ac * var_gvsat_ac);
        let assign62540_e81055: f64 = (assign62540_e81053 * var_cox_qm);
        let assign62540_e81057: f64 = (assign62540_e81055 * var_eta_p_ac);
        let assign62540_e81060: f64 = (var_gmob_dl_ac * var_gmob_dl_ac);
        let assign62540_e81061: f64 = (assign62540_e81057 / assign62540_e81060);
        (assign62540_e81061, (((((((((var_gvsat_ac_dn5 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn5)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn5)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn5)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn5 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn5)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn6 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn6)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn6)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn6)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn6 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn6)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn7 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn7)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn7)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn7)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn7 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn7)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((var_gvsat_ac_dn8 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn8)) * var_cox_qm) + (assign62540_e81053 * var_cox_qm_dn8)) * var_eta_p_ac) + (assign62540_e81055 * var_eta_p_ac_dn8)) * assign62540_e81060) - (assign62540_e81057 * ((var_gmob_dl_ac_dn8 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn8)))) / (assign62540_e81060 * assign62540_e81060)),)
    } else {
        (var_cgeff, var_cgeff_dn5, var_cgeff_dn6, var_cgeff_dn7, var_cgeff_dn8,)
    }
};
        var_cgeff = assign62540_e81063;
        var_cgeff_dn5 = assign62540_e81063_d_n5;
        var_cgeff_dn6 = assign62540_e81063_d_n6;
        var_cgeff_dn7 = assign62540_e81063_d_n7;
        var_cgeff_dn8 = assign62540_e81063_d_n8;

        let assign62550_e81066: f64 = if var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1766 = assign62550_e81066;

        let (assign62560_e81090, assign62560_e81090_d_n5, assign62560_e81090_d_n6, assign62560_e81090_d_n7, assign62560_e81090_d_n8,) = {
    if (((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) && (var_guard1766 != 0.0)) {
        let assign62560_e81077: f64 = (12.0 * var_t2);
        let assign62560_e81078: f64 = (1.0 + assign62560_e81077);
        let assign62560_e81079: f64 = (var_sidexc * assign62560_e81078);
        let assign62560_e81082: f64 = (12.0 * var_g_ideal);
        let assign62560_e81084: f64 = (assign62560_e81082 * var_g_ideal);
        let assign62560_e81086: f64 = (assign62560_e81084 * var_nt0);
        let assign62560_e81087: f64 = (assign62560_e81079 / assign62560_e81086);
        let assign62560_e81088: f64 = (var_mig + assign62560_e81087);
        (assign62560_e81088, (var_mig_dn5 + (((((var_sidexc_dn5 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn5))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn5) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn5)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn6 + (((((var_sidexc_dn6 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn6))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn6) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn6)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn7 + (((((var_sidexc_dn7 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn7))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn7) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn7)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (var_mig_dn8 + (((((var_sidexc_dn8 * assign62560_e81078) + (var_sidexc * (12.0 * var_t2_dn8))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * var_g_ideal_dn8) * var_g_ideal) + (assign62560_e81082 * var_g_ideal_dn8)) * var_nt0))) / (assign62560_e81086 * assign62560_e81086))),)
    } else {
        (var_mig, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8,)
    }
};
        var_mig = assign62560_e81090;
        var_mig_dn5 = assign62560_e81090_d_n5;
        var_mig_dn6 = assign62560_e81090_d_n6;
        var_mig_dn7 = assign62560_e81090_d_n7;
        var_mig_dn8 = assign62560_e81090_d_n8;

        let (assign62570_e81110, assign62570_e81110_d_n5, assign62570_e81110_d_n6, assign62570_e81110_d_n7, assign62570_e81110_d_n8,) = {
    if (((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) && (var_guard1766 != 0.0)) {
        let assign62570_e81099: f64 = (var_sidexc * var_sqt2);
        let assign62570_e81102: f64 = (1.0 + var_r);
        let assign62570_e81103: f64 = (assign62570_e81099 * assign62570_e81102);
        let assign62570_e81106: f64 = (var_g_ideal * var_nt0);
        let assign62570_e81107: f64 = (assign62570_e81103 / assign62570_e81106);
        let assign62570_e81108: f64 = (var_migid0 - assign62570_e81107);
        (assign62570_e81108, (var_migid0_dn5 - (((((((var_sidexc_dn5 * var_sqt2) + (var_sidexc * var_sqt2_dn5)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn5)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn5 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn6 - (((((((var_sidexc_dn6 * var_sqt2) + (var_sidexc * var_sqt2_dn6)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn6)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn6 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn7 - (((((((var_sidexc_dn7 * var_sqt2) + (var_sidexc * var_sqt2_dn7)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn7)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn7 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (var_migid0_dn8 - (((((((var_sidexc_dn8 * var_sqt2) + (var_sidexc * var_sqt2_dn8)) * assign62570_e81102) + (assign62570_e81099 * var_r_dn8)) * assign62570_e81106) - (assign62570_e81103 * (var_g_ideal_dn8 * var_nt0))) / (assign62570_e81106 * assign62570_e81106))),)
    } else {
        (var_migid0, var_migid0_dn5, var_migid0_dn6, var_migid0_dn7, var_migid0_dn8,)
    }
};
        var_migid0 = assign62570_e81110;
        var_migid0_dn5 = assign62570_e81110_d_n5;
        var_migid0_dn6 = assign62570_e81110_d_n6;
        var_migid0_dn7 = assign62570_e81110_d_n7;
        var_migid0_dn8 = assign62570_e81110_d_n8;

        let (assign62580_e81119, assign62580_e81119_d_n5, assign62580_e81119_d_n6, assign62580_e81119_d_n7, assign62580_e81119_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let assign62580_e81116: f64 = (var_nt / var_mig);
        let assign62580_e81117: f64 = (assign62580_e81116).sqrt();
        (assign62580_e81117, ((-((var_nt * var_mig_dn5) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn6) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn7) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)), ((-((var_nt * var_mig_dn8) / (var_mig * var_mig))) / (2.0 * assign62580_e81117)),)
    } else {
        (var_sqig, var_sqig_dn5, var_sqig_dn6, var_sqig_dn7, var_sqig_dn8,)
    }
};
        var_sqig = assign62580_e81119;
        var_sqig_dn5 = assign62580_e81119_d_n5;
        var_sqig_dn6 = assign62580_e81119_d_n6;
        var_sqig_dn7 = assign62580_e81119_d_n7;
        var_sqig_dn8 = assign62580_e81119_d_n8;

        let assign62590_e81122: f64 = if var_sqid <= 0.0 { 1.0 } else { 0.0 };
        var_guard1767 = assign62590_e81122;

        let (assign62600_e81130, assign62600_e81130_d_n5, assign62600_e81130_d_n6, assign62600_e81130_d_n7, assign62600_e81130_d_n8,) = {
    if (((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) && (var_guard1767 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_c_igid, var_c_igid_dn5, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8,)
    }
};
        var_c_igid = assign62600_e81130;
        var_c_igid_dn5 = assign62600_e81130_d_n5;
        var_c_igid_dn6 = assign62600_e81130_d_n6;
        var_c_igid_dn7 = assign62600_e81130_d_n7;
        var_c_igid_dn8 = assign62600_e81130_d_n8;

        let (assign62610_e81143, assign62610_e81143_d_n5, assign62610_e81143_d_n6, assign62610_e81143_d_n7, assign62610_e81143_d_n8,) = {
    if (((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) && (var_guard1767 == 0.0)) {
        let assign62610_e81139: f64 = (var_migid0 * var_sqig);
        let assign62610_e81141: f64 = (assign62610_e81139 / var_sqid);
        (assign62610_e81141, (((((var_migid0_dn5 * var_sqig) + (var_migid0 * var_sqig_dn5)) * var_sqid) - (assign62610_e81139 * var_sqid_dn5)) / (var_sqid * var_sqid)), (((((var_migid0_dn6 * var_sqig) + (var_migid0 * var_sqig_dn6)) * var_sqid) - (assign62610_e81139 * var_sqid_dn6)) / (var_sqid * var_sqid)), (((((var_migid0_dn7 * var_sqig) + (var_migid0 * var_sqig_dn7)) * var_sqid) - (assign62610_e81139 * var_sqid_dn7)) / (var_sqid * var_sqid)), (((((var_migid0_dn8 * var_sqig) + (var_migid0 * var_sqig_dn8)) * var_sqid) - (assign62610_e81139 * var_sqid_dn8)) / (var_sqid * var_sqid)),)
    } else {
        (var_c_igid, var_c_igid_dn5, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8,)
    }
};
        var_c_igid = assign62610_e81143;
        var_c_igid_dn5 = assign62610_e81143_d_n5;
        var_c_igid_dn6 = assign62610_e81143_d_n6;
        var_c_igid_dn7 = assign62610_e81143_d_n7;
        var_c_igid_dn8 = assign62610_e81143_d_n8;

        let (assign62620_e81159, assign62620_e81159_d_n5, assign62620_e81159_d_n6, assign62620_e81159_d_n7, assign62620_e81159_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let (assign62620_e81157, assign62620_e81157_d_n5, assign62620_e81157_d_n6, assign62620_e81157_d_n7, assign62620_e81157_d_n8,) = {
            if (var_c_igid > 0.0) {
                let (assign62620_e81155, assign62620_e81155_d_n5, assign62620_e81155_d_n6, assign62620_e81155_d_n7, assign62620_e81155_d_n8,) = {
                    if (var_c_igid < 1.0) {
                        (var_c_igid, var_c_igid_dn5, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8,)
                    } else {
                        (1.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign62620_e81155, assign62620_e81155_d_n5, assign62620_e81155_d_n6, assign62620_e81155_d_n7, assign62620_e81155_d_n8,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62620_e81157, assign62620_e81157_d_n5, assign62620_e81157_d_n6, assign62620_e81157_d_n7, assign62620_e81157_d_n8,)
    } else {
        (var_c_igid, var_c_igid_dn5, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8,)
    }
};
        var_c_igid = assign62620_e81159;
        var_c_igid_dn5 = assign62620_e81159_d_n5;
        var_c_igid_dn6 = assign62620_e81159_d_n6;
        var_c_igid_dn7 = assign62620_e81159_d_n7;
        var_c_igid_dn8 = assign62620_e81159_d_n8;

        *var_c_igid_slot = var_c_igid;
        *var_c_igid_dn5_slot = var_c_igid_dn5;
        *var_c_igid_dn6_slot = var_c_igid_dn6;
        *var_c_igid_dn7_slot = var_c_igid_dn7;
        *var_c_igid_dn8_slot = var_c_igid_dn8;
        *var_cgeff_slot = var_cgeff;
        *var_cgeff_dn5_slot = var_cgeff_dn5;
        *var_cgeff_dn6_slot = var_cgeff_dn6;
        *var_cgeff_dn7_slot = var_cgeff_dn7;
        *var_cgeff_dn8_slot = var_cgeff_dn8;
        *var_g_ideal_slot = var_g_ideal;
        *var_g_ideal_dn5_slot = var_g_ideal_dn5;
        *var_g_ideal_dn6_slot = var_g_ideal_dn6;
        *var_g_ideal_dn7_slot = var_g_ideal_dn7;
        *var_g_ideal_dn8_slot = var_g_ideal_dn8;
        *var_gfac_slot = var_gfac;
        *var_gfac_dn5_slot = var_gfac_dn5;
        *var_gfac_dn6_slot = var_gfac_dn6;
        *var_gfac_dn7_slot = var_gfac_dn7;
        *var_gfac_dn8_slot = var_gfac_dn8;
        *var_guard1763_slot = var_guard1763;
        *var_guard1764_slot = var_guard1764;
        *var_guard1765_slot = var_guard1765;
        *var_guard1766_slot = var_guard1766;
        *var_guard1767_slot = var_guard1767;
        *var_gvsat_exc_slot = var_gvsat_exc;
        *var_gvsat_exc_dn5_slot = var_gvsat_exc_dn5;
        *var_gvsat_exc_dn6_slot = var_gvsat_exc_dn6;
        *var_gvsat_exc_dn7_slot = var_gvsat_exc_dn7;
        *var_gvsat_exc_dn8_slot = var_gvsat_exc_dn8;
        *var_lc_slot = var_lc;
        *var_lc_dn5_slot = var_lc_dn5;
        *var_lc_dn6_slot = var_lc_dn6;
        *var_lc_dn7_slot = var_lc_dn7;
        *var_lc_dn8_slot = var_lc_dn8;
        *var_lcinv2_slot = var_lcinv2;
        *var_lcinv2_dn5_slot = var_lcinv2_dn5;
        *var_lcinv2_dn6_slot = var_lcinv2_dn6;
        *var_lcinv2_dn7_slot = var_lcinv2_dn7;
        *var_lcinv2_dn8_slot = var_lcinv2_dn8;
        *var_mid_slot = var_mid;
        *var_mid_dn5_slot = var_mid_dn5;
        *var_mid_dn6_slot = var_mid_dn6;
        *var_mid_dn7_slot = var_mid_dn7;
        *var_mid_dn8_slot = var_mid_dn8;
        *var_mig_slot = var_mig;
        *var_mig_dn5_slot = var_mig_dn5;
        *var_mig_dn6_slot = var_mig_dn6;
        *var_mig_dn7_slot = var_mig_dn7;
        *var_mig_dn8_slot = var_mig_dn8;
        *var_migid0_slot = var_migid0;
        *var_migid0_dn5_slot = var_migid0_dn5;
        *var_migid0_dn6_slot = var_migid0_dn6;
        *var_migid0_dn7_slot = var_migid0_dn7;
        *var_migid0_dn8_slot = var_migid0_dn8;
        *var_r_slot = var_r;
        *var_r_dn5_slot = var_r_dn5;
        *var_r_dn6_slot = var_r_dn6;
        *var_r_dn7_slot = var_r_dn7;
        *var_r_dn8_slot = var_r_dn8;
        *var_sidexc_slot = var_sidexc;
        *var_sidexc_dn5_slot = var_sidexc_dn5;
        *var_sidexc_dn6_slot = var_sidexc_dn6;
        *var_sidexc_dn7_slot = var_sidexc_dn7;
        *var_sidexc_dn8_slot = var_sidexc_dn8;
        *var_sqid_slot = var_sqid;
        *var_sqid_dn5_slot = var_sqid_dn5;
        *var_sqid_dn6_slot = var_sqid_dn6;
        *var_sqid_dn7_slot = var_sqid_dn7;
        *var_sqid_dn8_slot = var_sqid_dn8;
        *var_sqig_slot = var_sqig;
        *var_sqig_dn5_slot = var_sqig_dn5;
        *var_sqig_dn6_slot = var_sqig_dn6;
        *var_sqig_dn7_slot = var_sqig_dn7;
        *var_sqig_dn8_slot = var_sqig_dn8;
        *var_sqt2_slot = var_sqt2;
        *var_sqt2_dn5_slot = var_sqt2_dn5;
        *var_sqt2_dn6_slot = var_sqt2_dn6;
        *var_sqt2_dn7_slot = var_sqt2_dn7;
        *var_sqt2_dn8_slot = var_sqt2_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_thesat1_exc_slot = var_thesat1_exc;
        *var_thesat1_exc_dn5_slot = var_thesat1_exc_dn5;
        *var_thesat1_exc_dn6_slot = var_thesat1_exc_dn6;
        *var_thesat1_exc_dn7_slot = var_thesat1_exc_dn7;
        *var_thesat1_exc_dn8_slot = var_thesat1_exc_dn8;
        *var_zsat_exc_slot = var_zsat_exc;
        *var_zsat_exc_dn5_slot = var_zsat_exc_dn5;
        *var_zsat_exc_dn6_slot = var_zsat_exc_dn6;
        *var_zsat_exc_dn7_slot = var_zsat_exc_dn7;
        *var_zsat_exc_dn8_slot = var_zsat_exc_dn8;
    }

    pub(super) fn stamp_transient_block_146(
        p: &Parameters,
        var_alpha_dc: f64,
        var_alpha_dc_dn5: f64,
        var_alpha_dc_dn6: f64,
        var_alpha_dc_dn7: f64,
        var_alpha_dc_dn8: f64,
        var_betnedge_i: f64,
        var_c_igid: f64,
        var_c_igid_dn5: f64,
        var_c_igid_dn6: f64,
        var_c_igid_dn7: f64,
        var_c_igid_dn8: f64,
        var_cox_over_q: f64,
        var_dsqredge: f64,
        var_dsqredge_dn5: f64,
        var_dsqredge_dn6: f64,
        var_dsqredge_dn7: f64,
        var_dsqredge_dn8: f64,
        var_gfedge2: f64,
        var_guard1760: f64,
        var_guard1765: f64,
        var_h_dc: f64,
        var_h_dc_dn5: f64,
        var_h_dc_dn6: f64,
        var_h_dc_dn7: f64,
        var_h_dc_dn8: f64,
        var_phit: f64,
        var_sqid: f64,
        var_sqid_dn5: f64,
        var_sqid_dn6: f64,
        var_sqid_dn7: f64,
        var_sqid_dn8: f64,
        var_sqig: f64,
        var_sqig_dn5: f64,
        var_sqig_dn6: f64,
        var_sqig_dn7: f64,
        var_sqig_dn8: f64,
        var_xgedge: f64,
        var_guard1769_slot: &mut f64,
        var_migid_slot: &mut f64,
        var_migid_dn5_slot: &mut f64,
        var_migid_dn6_slot: &mut f64,
        var_migid_dn7_slot: &mut f64,
        var_migid_dn8_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
    ) {
        let mut var_guard1769: f64 = *var_guard1769_slot;
        let mut var_migid: f64 = *var_migid_slot;
        let mut var_migid_dn5: f64 = *var_migid_dn5_slot;
        let mut var_migid_dn6: f64 = *var_migid_dn6_slot;
        let mut var_migid_dn7: f64 = *var_migid_dn7_slot;
        let mut var_migid_dn8: f64 = *var_migid_dn8_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;

        let (assign62630_e81169, assign62630_e81169_d_n5, assign62630_e81169_d_n6, assign62630_e81169_d_n7, assign62630_e81169_d_n8,) = {
    if ((var_guard1760 != 0.0) && (var_guard1765 != 0.0)) {
        let assign62630_e81165: f64 = (var_c_igid * var_sqid);
        let assign62630_e81167: f64 = (assign62630_e81165 / var_sqig);
        (assign62630_e81167, (((((var_c_igid_dn5 * var_sqid) + (var_c_igid * var_sqid_dn5)) * var_sqig) - (assign62630_e81165 * var_sqig_dn5)) / (var_sqig * var_sqig)), (((((var_c_igid_dn6 * var_sqid) + (var_c_igid * var_sqid_dn6)) * var_sqig) - (assign62630_e81165 * var_sqig_dn6)) / (var_sqig * var_sqig)), (((((var_c_igid_dn7 * var_sqid) + (var_c_igid * var_sqid_dn7)) * var_sqig) - (assign62630_e81165 * var_sqig_dn7)) / (var_sqig * var_sqig)), (((((var_c_igid_dn8 * var_sqid) + (var_c_igid * var_sqid_dn8)) * var_sqig) - (assign62630_e81165 * var_sqig_dn8)) / (var_sqig * var_sqig)),)
    } else {
        (var_migid, var_migid_dn5, var_migid_dn6, var_migid_dn7, var_migid_dn8,)
    }
};
        var_migid = assign62630_e81169;
        var_migid_dn5 = assign62630_e81169_d_n5;
        var_migid_dn6 = assign62630_e81169_d_n6;
        var_migid_dn7 = assign62630_e81169_d_n7;
        var_migid_dn8 = assign62630_e81169_d_n8;

        let assign62800_e81277: f64 = if (((p.p46 != 0.0) && (var_betnedge_i > 0.0)) && (var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        var_guard1769 = assign62800_e81277;

        let (assign62810_e81285, assign62810_e81285_d_n5, assign62810_e81285_d_n6, assign62810_e81285_d_n7, assign62810_e81285_d_n8,) = {
    if (var_guard1769 != 0.0) {
        let assign62810_e81281: f64 = (4.0 * var_dsqredge);
        let assign62810_e81283: f64 = (assign62810_e81281 / var_gfedge2);
        (assign62810_e81283, ((4.0 * var_dsqredge_dn5) / var_gfedge2), ((4.0 * var_dsqredge_dn6) / var_gfedge2), ((4.0 * var_dsqredge_dn7) / var_gfedge2), ((4.0 * var_dsqredge_dn8) / var_gfedge2),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign62810_e81285;
        var_temp1_dn5 = assign62810_e81285_d_n5;
        var_temp1_dn6 = assign62810_e81285_d_n6;
        var_temp1_dn7 = assign62810_e81285_d_n7;
        var_temp1_dn8 = assign62810_e81285_d_n8;

        let (assign62830_e81305, assign62830_e81305_d_n5, assign62830_e81305_d_n6, assign62830_e81305_d_n7, assign62830_e81305_d_n8,) = {
    if (var_guard1769 != 0.0) {
        let assign62830_e81303: f64 = (var_cox_over_q * var_phit);
        (assign62830_e81303, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign62830_e81305;
        var_temp1_dn5 = assign62830_e81305_d_n5;
        var_temp1_dn6 = assign62830_e81305_d_n6;
        var_temp1_dn7 = assign62830_e81305_d_n7;
        var_temp1_dn8 = assign62830_e81305_d_n8;

        let (assign62960_e81445, assign62960_e81445_d_n5, assign62960_e81445_d_n6, assign62960_e81445_d_n7, assign62960_e81445_d_n8,) = {
    if (var_guard1769 != 0.0) {
        let assign62960_e81443: f64 = (var_alpha_dc * var_h_dc);
        (assign62960_e81443, ((var_alpha_dc_dn5 * var_h_dc) + (var_alpha_dc * var_h_dc_dn5)), ((var_alpha_dc_dn6 * var_h_dc) + (var_alpha_dc * var_h_dc_dn6)), ((var_alpha_dc_dn7 * var_h_dc) + (var_alpha_dc * var_h_dc_dn7)), ((var_alpha_dc_dn8 * var_h_dc) + (var_alpha_dc * var_h_dc_dn8)),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign62960_e81445;
        var_temp1_dn5 = assign62960_e81445_d_n5;
        var_temp1_dn6 = assign62960_e81445_d_n6;
        var_temp1_dn7 = assign62960_e81445_d_n7;
        var_temp1_dn8 = assign62960_e81445_d_n8;

        *var_guard1769_slot = var_guard1769;
        *var_migid_slot = var_migid;
        *var_migid_dn5_slot = var_migid_dn5;
        *var_migid_dn6_slot = var_migid_dn6;
        *var_migid_dn7_slot = var_migid_dn7;
        *var_migid_dn8_slot = var_migid_dn8;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
    }

    pub(super) fn stamp_reactive_block_0(
        p: &Parameters,
        var_alphaav_slot: &mut f64,
        var_alphaav_rv_slot: &mut f64,
        var_chnl_type_slot: &mut f64,
        var_chnl_type_rv_slot: &mut f64,
        var_cjorbotd_i_slot: &mut f64,
        var_cjorbotd_i_rv_slot: &mut f64,
        var_cjorgat2nd_slot: &mut f64,
        var_cjorgat2nd_rv_slot: &mut f64,
        var_cjorgatd_i_slot: &mut f64,
        var_cjorgatd_i_rv_slot: &mut f64,
        var_cjorstid_i_slot: &mut f64,
        var_cjorstid_i_rv_slot: &mut f64,
        var_csrhbotd_i_slot: &mut f64,
        var_csrhbotd_i_rv_slot: &mut f64,
        var_csrhgatd_i_slot: &mut f64,
        var_csrhgatd_i_rv_slot: &mut f64,
        var_csrhstid_i_slot: &mut f64,
        var_csrhstid_i_rv_slot: &mut f64,
        var_deltaphigr_slot: &mut f64,
        var_deltaphigr_rv_slot: &mut f64,
        var_epssi_slot: &mut f64,
        var_epssi_rv_slot: &mut f64,
        var_guard1_slot: &mut f64,
        var_guard1_rv_slot: &mut f64,
        var_guard2_slot: &mut f64,
        var_guard2_rv_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard3_rv_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard4_rv_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard5_rv_slot: &mut f64,
        var_idsatrbotd_i_slot: &mut f64,
        var_idsatrbotd_i_rv_slot: &mut f64,
        var_idsatrgatd_i_slot: &mut f64,
        var_idsatrgatd_i_rv_slot: &mut f64,
        var_idsatrstid_i_slot: &mut f64,
        var_idsatrstid_i_rv_slot: &mut f64,
        var_kbol_over_qele_slot: &mut f64,
        var_kbol_over_qele_rv_slot: &mut f64,
        var_one_minus_pbot_slot: &mut f64,
        var_one_minus_pbot_rv_slot: &mut f64,
        var_one_minus_pgat_slot: &mut f64,
        var_one_minus_pgat2nd_slot: &mut f64,
        var_one_minus_pgat2nd_rv_slot: &mut f64,
        var_one_minus_pgat_rv_slot: &mut f64,
        var_one_minus_psti_slot: &mut f64,
        var_one_minus_psti_rv_slot: &mut f64,
        var_one_over_one_minus_pbot_slot: &mut f64,
        var_one_over_one_minus_pbot_rv_slot: &mut f64,
        var_one_over_one_minus_pgat_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_rv_slot: &mut f64,
        var_one_over_one_minus_pgat_rv_slot: &mut f64,
        var_one_over_one_minus_psti_slot: &mut f64,
        var_one_over_one_minus_psti_rv_slot: &mut f64,
        var_pbotd_i_slot: &mut f64,
        var_pbotd_i_rv_slot: &mut f64,
        var_pgat2nd_slot: &mut f64,
        var_pgat2nd_rv_slot: &mut f64,
        var_pgatd_i_slot: &mut f64,
        var_pgatd_i_rv_slot: &mut f64,
        var_phigbotd_i_slot: &mut f64,
        var_phigbotd_i_rv_slot: &mut f64,
        var_phiggat2nd_slot: &mut f64,
        var_phiggat2nd_rv_slot: &mut f64,
        var_phiggatd_i_slot: &mut f64,
        var_phiggatd_i_rv_slot: &mut f64,
        var_phigrbot_slot: &mut f64,
        var_phigrbot_rv_slot: &mut f64,
        var_phigrgat_slot: &mut f64,
        var_phigrgat2nd_slot: &mut f64,
        var_phigrgat2nd_rv_slot: &mut f64,
        var_phigrgat_rv_slot: &mut f64,
        var_phigrsti_slot: &mut f64,
        var_phigrsti_rv_slot: &mut f64,
        var_phigstid_i_slot: &mut f64,
        var_phigstid_i_rv_slot: &mut f64,
        var_phitr_slot: &mut f64,
        var_phitr_rv_slot: &mut f64,
        var_phitrinv_slot: &mut f64,
        var_phitrinv_rv_slot: &mut f64,
        var_pstid_i_slot: &mut f64,
        var_pstid_i_rv_slot: &mut f64,
        var_swgat2nd_slot: &mut f64,
        var_swgat2nd_rv_slot: &mut f64,
        var_swjunexp_i_slot: &mut f64,
        var_swjunexp_i_rv_slot: &mut f64,
        var_tkr_slot: &mut f64,
        var_tkr_1_slot: &mut f64,
        var_tkr_1_rv_slot: &mut f64,
        var_tkr_rv_slot: &mut f64,
        var_vbirbotd_i_slot: &mut f64,
        var_vbirbotd_i_rv_slot: &mut f64,
        var_vbirbotinv_slot: &mut f64,
        var_vbirbotinv_rv_slot: &mut f64,
        var_vbirgat2nd_slot: &mut f64,
        var_vbirgat2nd_rv_slot: &mut f64,
        var_vbirgatd_i_slot: &mut f64,
        var_vbirgatd_i_rv_slot: &mut f64,
        var_vbirgatinv_slot: &mut f64,
        var_vbirgatinv_rv_slot: &mut f64,
        var_vbirstid_i_slot: &mut f64,
        var_vbirstid_i_rv_slot: &mut f64,
        var_vbirstiinv_slot: &mut f64,
        var_vbirstiinv_rv_slot: &mut f64,
        var_vbrinvbot_slot: &mut f64,
        var_vbrinvbot_rv_slot: &mut f64,
        var_vbrinvgat_slot: &mut f64,
        var_vbrinvgat_dn5_slot: &mut f64,
        var_vbrinvgat_dn6_slot: &mut f64,
        var_vbrinvgat_dn7_slot: &mut f64,
        var_vbrinvgat_dn8_slot: &mut f64,
        var_vbrinvgat_rv_slot: &mut f64,
        var_vbrinvsti_slot: &mut f64,
        var_vbrinvsti_rv_slot: &mut f64,
        var_wdepnulrbot_slot: &mut f64,
        var_wdepnulrbot_rv_slot: &mut f64,
        var_wdepnulrgat_slot: &mut f64,
        var_wdepnulrgat_rv_slot: &mut f64,
        var_wdepnulrinvbot_slot: &mut f64,
        var_wdepnulrinvbot_rv_slot: &mut f64,
        var_wdepnulrinvgat_slot: &mut f64,
        var_wdepnulrinvgat_rv_slot: &mut f64,
        var_wdepnulrinvsti_slot: &mut f64,
        var_wdepnulrinvsti_rv_slot: &mut f64,
        var_wdepnulrsti_slot: &mut f64,
        var_wdepnulrsti_rv_slot: &mut f64,
    ) {
        let mut var_alphaav: f64 = *var_alphaav_slot;
        let mut var_alphaav_rv: f64 = *var_alphaav_rv_slot;
        let mut var_chnl_type: f64 = *var_chnl_type_slot;
        let mut var_chnl_type_rv: f64 = *var_chnl_type_rv_slot;
        let mut var_cjorbotd_i: f64 = *var_cjorbotd_i_slot;
        let mut var_cjorbotd_i_rv: f64 = *var_cjorbotd_i_rv_slot;
        let mut var_cjorgat2nd: f64 = *var_cjorgat2nd_slot;
        let mut var_cjorgat2nd_rv: f64 = *var_cjorgat2nd_rv_slot;
        let mut var_cjorgatd_i: f64 = *var_cjorgatd_i_slot;
        let mut var_cjorgatd_i_rv: f64 = *var_cjorgatd_i_rv_slot;
        let mut var_cjorstid_i: f64 = *var_cjorstid_i_slot;
        let mut var_cjorstid_i_rv: f64 = *var_cjorstid_i_rv_slot;
        let mut var_csrhbotd_i: f64 = *var_csrhbotd_i_slot;
        let mut var_csrhbotd_i_rv: f64 = *var_csrhbotd_i_rv_slot;
        let mut var_csrhgatd_i: f64 = *var_csrhgatd_i_slot;
        let mut var_csrhgatd_i_rv: f64 = *var_csrhgatd_i_rv_slot;
        let mut var_csrhstid_i: f64 = *var_csrhstid_i_slot;
        let mut var_csrhstid_i_rv: f64 = *var_csrhstid_i_rv_slot;
        let mut var_deltaphigr: f64 = *var_deltaphigr_slot;
        let mut var_deltaphigr_rv: f64 = *var_deltaphigr_rv_slot;
        let mut var_epssi: f64 = *var_epssi_slot;
        let mut var_epssi_rv: f64 = *var_epssi_rv_slot;
        let mut var_guard1: f64 = *var_guard1_slot;
        let mut var_guard1_rv: f64 = *var_guard1_rv_slot;
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_guard2_rv: f64 = *var_guard2_rv_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard3_rv: f64 = *var_guard3_rv_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard4_rv: f64 = *var_guard4_rv_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard5_rv: f64 = *var_guard5_rv_slot;
        let mut var_idsatrbotd_i: f64 = *var_idsatrbotd_i_slot;
        let mut var_idsatrbotd_i_rv: f64 = *var_idsatrbotd_i_rv_slot;
        let mut var_idsatrgatd_i: f64 = *var_idsatrgatd_i_slot;
        let mut var_idsatrgatd_i_rv: f64 = *var_idsatrgatd_i_rv_slot;
        let mut var_idsatrstid_i: f64 = *var_idsatrstid_i_slot;
        let mut var_idsatrstid_i_rv: f64 = *var_idsatrstid_i_rv_slot;
        let mut var_kbol_over_qele: f64 = *var_kbol_over_qele_slot;
        let mut var_kbol_over_qele_rv: f64 = *var_kbol_over_qele_rv_slot;
        let mut var_one_minus_pbot: f64 = *var_one_minus_pbot_slot;
        let mut var_one_minus_pbot_rv: f64 = *var_one_minus_pbot_rv_slot;
        let mut var_one_minus_pgat: f64 = *var_one_minus_pgat_slot;
        let mut var_one_minus_pgat2nd: f64 = *var_one_minus_pgat2nd_slot;
        let mut var_one_minus_pgat2nd_rv: f64 = *var_one_minus_pgat2nd_rv_slot;
        let mut var_one_minus_pgat_rv: f64 = *var_one_minus_pgat_rv_slot;
        let mut var_one_minus_psti: f64 = *var_one_minus_psti_slot;
        let mut var_one_minus_psti_rv: f64 = *var_one_minus_psti_rv_slot;
        let mut var_one_over_one_minus_pbot: f64 = *var_one_over_one_minus_pbot_slot;
        let mut var_one_over_one_minus_pbot_rv: f64 = *var_one_over_one_minus_pbot_rv_slot;
        let mut var_one_over_one_minus_pgat: f64 = *var_one_over_one_minus_pgat_slot;
        let mut var_one_over_one_minus_pgat2nd: f64 = *var_one_over_one_minus_pgat2nd_slot;
        let mut var_one_over_one_minus_pgat2nd_rv: f64 = *var_one_over_one_minus_pgat2nd_rv_slot;
        let mut var_one_over_one_minus_pgat_rv: f64 = *var_one_over_one_minus_pgat_rv_slot;
        let mut var_one_over_one_minus_psti: f64 = *var_one_over_one_minus_psti_slot;
        let mut var_one_over_one_minus_psti_rv: f64 = *var_one_over_one_minus_psti_rv_slot;
        let mut var_pbotd_i: f64 = *var_pbotd_i_slot;
        let mut var_pbotd_i_rv: f64 = *var_pbotd_i_rv_slot;
        let mut var_pgat2nd: f64 = *var_pgat2nd_slot;
        let mut var_pgat2nd_rv: f64 = *var_pgat2nd_rv_slot;
        let mut var_pgatd_i: f64 = *var_pgatd_i_slot;
        let mut var_pgatd_i_rv: f64 = *var_pgatd_i_rv_slot;
        let mut var_phigbotd_i: f64 = *var_phigbotd_i_slot;
        let mut var_phigbotd_i_rv: f64 = *var_phigbotd_i_rv_slot;
        let mut var_phiggat2nd: f64 = *var_phiggat2nd_slot;
        let mut var_phiggat2nd_rv: f64 = *var_phiggat2nd_rv_slot;
        let mut var_phiggatd_i: f64 = *var_phiggatd_i_slot;
        let mut var_phiggatd_i_rv: f64 = *var_phiggatd_i_rv_slot;
        let mut var_phigrbot: f64 = *var_phigrbot_slot;
        let mut var_phigrbot_rv: f64 = *var_phigrbot_rv_slot;
        let mut var_phigrgat: f64 = *var_phigrgat_slot;
        let mut var_phigrgat2nd: f64 = *var_phigrgat2nd_slot;
        let mut var_phigrgat2nd_rv: f64 = *var_phigrgat2nd_rv_slot;
        let mut var_phigrgat_rv: f64 = *var_phigrgat_rv_slot;
        let mut var_phigrsti: f64 = *var_phigrsti_slot;
        let mut var_phigrsti_rv: f64 = *var_phigrsti_rv_slot;
        let mut var_phigstid_i: f64 = *var_phigstid_i_slot;
        let mut var_phigstid_i_rv: f64 = *var_phigstid_i_rv_slot;
        let mut var_phitr: f64 = *var_phitr_slot;
        let mut var_phitr_rv: f64 = *var_phitr_rv_slot;
        let mut var_phitrinv: f64 = *var_phitrinv_slot;
        let mut var_phitrinv_rv: f64 = *var_phitrinv_rv_slot;
        let mut var_pstid_i: f64 = *var_pstid_i_slot;
        let mut var_pstid_i_rv: f64 = *var_pstid_i_rv_slot;
        let mut var_swgat2nd: f64 = *var_swgat2nd_slot;
        let mut var_swgat2nd_rv: f64 = *var_swgat2nd_rv_slot;
        let mut var_swjunexp_i: f64 = *var_swjunexp_i_slot;
        let mut var_swjunexp_i_rv: f64 = *var_swjunexp_i_rv_slot;
        let mut var_tkr: f64 = *var_tkr_slot;
        let mut var_tkr_1: f64 = *var_tkr_1_slot;
        let mut var_tkr_1_rv: f64 = *var_tkr_1_rv_slot;
        let mut var_tkr_rv: f64 = *var_tkr_rv_slot;
        let mut var_vbirbotd_i: f64 = *var_vbirbotd_i_slot;
        let mut var_vbirbotd_i_rv: f64 = *var_vbirbotd_i_rv_slot;
        let mut var_vbirbotinv: f64 = *var_vbirbotinv_slot;
        let mut var_vbirbotinv_rv: f64 = *var_vbirbotinv_rv_slot;
        let mut var_vbirgat2nd: f64 = *var_vbirgat2nd_slot;
        let mut var_vbirgat2nd_rv: f64 = *var_vbirgat2nd_rv_slot;
        let mut var_vbirgatd_i: f64 = *var_vbirgatd_i_slot;
        let mut var_vbirgatd_i_rv: f64 = *var_vbirgatd_i_rv_slot;
        let mut var_vbirgatinv: f64 = *var_vbirgatinv_slot;
        let mut var_vbirgatinv_rv: f64 = *var_vbirgatinv_rv_slot;
        let mut var_vbirstid_i: f64 = *var_vbirstid_i_slot;
        let mut var_vbirstid_i_rv: f64 = *var_vbirstid_i_rv_slot;
        let mut var_vbirstiinv: f64 = *var_vbirstiinv_slot;
        let mut var_vbirstiinv_rv: f64 = *var_vbirstiinv_rv_slot;
        let mut var_vbrinvbot: f64 = *var_vbrinvbot_slot;
        let mut var_vbrinvbot_rv: f64 = *var_vbrinvbot_rv_slot;
        let mut var_vbrinvgat: f64 = *var_vbrinvgat_slot;
        let mut var_vbrinvgat_dn5: f64 = *var_vbrinvgat_dn5_slot;
        let mut var_vbrinvgat_dn6: f64 = *var_vbrinvgat_dn6_slot;
        let mut var_vbrinvgat_dn7: f64 = *var_vbrinvgat_dn7_slot;
        let mut var_vbrinvgat_dn8: f64 = *var_vbrinvgat_dn8_slot;
        let mut var_vbrinvgat_rv: f64 = *var_vbrinvgat_rv_slot;
        let mut var_vbrinvsti: f64 = *var_vbrinvsti_slot;
        let mut var_vbrinvsti_rv: f64 = *var_vbrinvsti_rv_slot;
        let mut var_wdepnulrbot: f64 = *var_wdepnulrbot_slot;
        let mut var_wdepnulrbot_rv: f64 = *var_wdepnulrbot_rv_slot;
        let mut var_wdepnulrgat: f64 = *var_wdepnulrgat_slot;
        let mut var_wdepnulrgat_rv: f64 = *var_wdepnulrgat_rv_slot;
        let mut var_wdepnulrinvbot: f64 = *var_wdepnulrinvbot_slot;
        let mut var_wdepnulrinvbot_rv: f64 = *var_wdepnulrinvbot_rv_slot;
        let mut var_wdepnulrinvgat: f64 = *var_wdepnulrinvgat_slot;
        let mut var_wdepnulrinvgat_rv: f64 = *var_wdepnulrinvgat_rv_slot;
        let mut var_wdepnulrinvsti: f64 = *var_wdepnulrinvsti_slot;
        let mut var_wdepnulrinvsti_rv: f64 = *var_wdepnulrinvsti_rv_slot;
        let mut var_wdepnulrsti: f64 = *var_wdepnulrsti_slot;
        let mut var_wdepnulrsti_rv: f64 = *var_wdepnulrsti_rv_slot;

        let assign00_e1445: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        var_guard1 = assign00_e1445;
        var_guard1_rv = 0.0;

        let (assign10_e1450,) = {
    if (var_guard1 != 0.0) {
        let assign10_e1448: f64 = 1.0;
        (assign10_e1448,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign10_e1450;
        var_chnl_type_rv = 0.0;

        let (assign20_e1456,) = {
    if (var_guard1 == 0.0) {
        let assign20_e1454: f64 = (-1.0);
        (assign20_e1454,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign20_e1456;
        var_chnl_type_rv = 0.0;

        let assign30_e1459: f64 = (8.8541878176e-12 * 11.8);
        var_epssi = assign30_e1459;
        var_epssi_rv = 0.0;

        let assign40_e1462: f64 = (273.15 + p.p38);
        var_tkr = assign40_e1462;
        var_tkr_rv = 0.0;

        var_swjunexp_i = 0.0;
        var_swjunexp_i_rv = 0.0;

        let assign60_e1466: f64 = if p.p920 > 0.5 { 1.0 } else { 0.0 };
        var_guard2 = assign60_e1466;
        var_guard2_rv = 0.0;

        let (assign70_e1470,) = {
    if (var_guard2 != 0.0) {
        (1.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign70_e1470;
        var_swjunexp_i_rv = 0.0;

        let (assign80_e1475,) = {
    if (var_guard2 == 0.0) {
        (0.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign80_e1475;
        var_swjunexp_i_rv = 0.0;

        let assign90_e1478: f64 = (273.15 + p.p816);
        var_tkr_1 = assign90_e1478;
        var_tkr_1_rv = 0.0;

        let assign100_e1481: f64 = (1.3806505e-23 / 1.6021918e-19);
        var_kbol_over_qele = assign100_e1481;
        var_kbol_over_qele_rv = 0.0;

        let assign110_e1484: f64 = (var_kbol_over_qele * var_tkr_1);
        var_phitr = assign110_e1484;
        var_phitr_rv = 0.0;

        let assign120_e1487: f64 = (1.0 / var_phitr);
        var_phitrinv = assign120_e1487;
        var_phitrinv_rv = 0.0;

        let assign130_e1490: f64 = (0.000702 * var_tkr_1);
        let assign130_e1492: f64 = (assign130_e1490 * var_tkr_1);
        let assign130_e1493: f64 = (-assign130_e1492);
        let assign130_e1496: f64 = (1108.0 + var_tkr_1);
        let assign130_e1497: f64 = (assign130_e1493 / assign130_e1496);
        var_deltaphigr = assign130_e1497;
        var_deltaphigr_rv = 0.0;

        let assign140_e1500: f64 = (p.p827 + var_deltaphigr);
        var_phigrbot = assign140_e1500;
        var_phigrbot_rv = 0.0;

        let assign150_e1503: f64 = (p.p828 + var_deltaphigr);
        var_phigrsti = assign150_e1503;
        var_phigrsti_rv = 0.0;

        let assign160_e1506: f64 = (p.p829 + var_deltaphigr);
        var_phigrgat = assign160_e1506;
        var_phigrgat_rv = 0.0;

        let assign170_e1509: f64 = (1.0 - p.p824);
        var_one_minus_pbot = assign170_e1509;
        var_one_minus_pbot_rv = 0.0;

        let assign180_e1512: f64 = (1.0 - p.p825);
        var_one_minus_psti = assign180_e1512;
        var_one_minus_psti_rv = 0.0;

        let assign190_e1515: f64 = (1.0 - p.p826);
        var_one_minus_pgat = assign190_e1515;
        var_one_minus_pgat_rv = 0.0;

        let assign200_e1518: f64 = (1.0 / var_one_minus_pbot);
        var_one_over_one_minus_pbot = assign200_e1518;
        var_one_over_one_minus_pbot_rv = 0.0;

        let assign210_e1521: f64 = (1.0 / var_one_minus_psti);
        var_one_over_one_minus_psti = assign210_e1521;
        var_one_over_one_minus_psti_rv = 0.0;

        let assign220_e1524: f64 = (1.0 / var_one_minus_pgat);
        var_one_over_one_minus_pgat = assign220_e1524;
        var_one_over_one_minus_pgat_rv = 0.0;

        let assign230_e1527: f64 = (var_epssi / p.p818);
        var_wdepnulrbot = assign230_e1527;
        var_wdepnulrbot_rv = 0.0;

        let assign240_e1530: f64 = (p.p836 * var_epssi);
        let assign240_e1532: f64 = (assign240_e1530 / p.p819);
        var_wdepnulrsti = assign240_e1532;
        var_wdepnulrsti_rv = 0.0;

        let assign250_e1535: f64 = (p.p837 * var_epssi);
        let assign250_e1537: f64 = (assign250_e1535 / p.p820);
        var_wdepnulrgat = assign250_e1537;
        var_wdepnulrgat_rv = 0.0;

        let assign260_e1540: f64 = (1.0 / var_wdepnulrbot);
        var_wdepnulrinvbot = assign260_e1540;
        var_wdepnulrinvbot_rv = 0.0;

        let assign270_e1543: f64 = (1.0 / var_wdepnulrsti);
        var_wdepnulrinvsti = assign270_e1543;
        var_wdepnulrinvsti_rv = 0.0;

        let assign280_e1546: f64 = (1.0 / var_wdepnulrgat);
        var_wdepnulrinvgat = assign280_e1546;
        var_wdepnulrinvgat_rv = 0.0;

        let assign290_e1549: f64 = (1.0 / p.p821);
        var_vbirbotinv = assign290_e1549;
        var_vbirbotinv_rv = 0.0;

        let assign300_e1552: f64 = (1.0 / p.p822);
        var_vbirstiinv = assign300_e1552;
        var_vbirstiinv_rv = 0.0;

        let assign310_e1555: f64 = (1.0 / p.p823);
        var_vbirgatinv = assign310_e1555;
        var_vbirgatinv_rv = 0.0;

        let assign350_e1580: f64 = (1.0 / p.p817);
        let assign350_e1581: f64 = (1.0 - assign350_e1580);
        var_alphaav = assign350_e1581;
        var_alphaav_rv = 0.0;

        let assign390_e1605: f64 = (1.0 / p.p853);
        var_vbrinvbot = assign390_e1605;
        var_vbrinvbot_rv = 0.0;

        let assign400_e1608: f64 = (1.0 / p.p854);
        var_vbrinvsti = assign400_e1608;
        var_vbrinvsti_rv = 0.0;

        let assign410_e1611: f64 = (1.0 / p.p855);
        var_vbrinvgat = assign410_e1611;
        var_vbrinvgat_dn5 = 0.0;
        var_vbrinvgat_dn6 = 0.0;
        var_vbrinvgat_dn7 = 0.0;
        var_vbrinvgat_dn8 = 0.0;
        var_vbrinvgat_rv = 0.0;

        let assign450_e1668: f64 = if ((((p.p859 != 1.0) || (p.p860 != 1.0)) || (p.p861 != 1.0)) || (p.p862 != 1.0)) { 1.0 } else { 0.0 };
        var_guard3 = assign450_e1668;
        var_guard3_rv = 0.0;

        let (assign460_e1672,) = {
    if (var_guard3 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign460_e1672;
        var_swgat2nd_rv = 0.0;

        let (assign470_e1677,) = {
    if (var_guard3 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign470_e1677;
        var_swgat2nd_rv = 0.0;

        let assign480_e1680: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard4 = assign480_e1680;
        var_guard4_rv = 0.0;

        let (assign490_e1693,) = {
    if (var_guard4 != 0.0) {
        let assign490_e1684: f64 = (p.p820 * p.p859);
        let (assign490_e1691,) = {
            if (assign490_e1684 > 1e-18) {
                let assign490_e1689: f64 = (p.p820 * p.p859);
                (assign490_e1689,)
            } else {
                (1e-18,)
            }
        };
        (assign490_e1691,)
    } else {
        (var_cjorgat2nd,)
    }
};
        var_cjorgat2nd = assign490_e1693;
        var_cjorgat2nd_rv = 0.0;

        let (assign500_e1706,) = {
    if (var_guard4 != 0.0) {
        let assign500_e1697: f64 = (p.p823 * p.p860);
        let (assign500_e1704,) = {
            if (assign500_e1697 > 0.05) {
                let assign500_e1702: f64 = (p.p823 * p.p860);
                (assign500_e1702,)
            } else {
                (0.05,)
            }
        };
        (assign500_e1704,)
    } else {
        (var_vbirgat2nd,)
    }
};
        var_vbirgat2nd = assign500_e1706;
        var_vbirgat2nd_rv = 0.0;

        let (assign510_e1733,) = {
    if (var_guard4 != 0.0) {
        let assign510_e1710: f64 = (p.p826 * p.p861);
        let (assign510_e1717,) = {
            if (assign510_e1710 > 0.05) {
                let assign510_e1715: f64 = (p.p826 * p.p861);
                (assign510_e1715,)
            } else {
                (0.05,)
            }
        };
        let (assign510_e1731,) = {
            if (assign510_e1717 < 0.95) {
                let assign510_e1722: f64 = (p.p826 * p.p861);
                let (assign510_e1729,) = {
                    if (assign510_e1722 > 0.05) {
                        let assign510_e1727: f64 = (p.p826 * p.p861);
                        (assign510_e1727,)
                    } else {
                        (0.05,)
                    }
                };
                (assign510_e1729,)
            } else {
                (0.95,)
            }
        };
        (assign510_e1731,)
    } else {
        (var_pgat2nd,)
    }
};
        var_pgat2nd = assign510_e1733;
        var_pgat2nd_rv = 0.0;

        let (assign520_e1739,) = {
    if (var_guard4 != 0.0) {
        let assign520_e1737: f64 = (p.p829 * p.p862);
        (assign520_e1737,)
    } else {
        (var_phiggat2nd,)
    }
};
        var_phiggat2nd = assign520_e1739;
        var_phiggat2nd_rv = 0.0;

        let (assign530_e1745,) = {
    if (var_guard4 != 0.0) {
        let assign530_e1743: f64 = (var_phiggat2nd + var_deltaphigr);
        (assign530_e1743,)
    } else {
        (var_phigrgat2nd,)
    }
};
        var_phigrgat2nd = assign530_e1745;
        var_phigrgat2nd_rv = 0.0;

        let (assign540_e1751,) = {
    if (var_guard4 != 0.0) {
        let assign540_e1749: f64 = (1.0 - var_pgat2nd);
        (assign540_e1749,)
    } else {
        (var_one_minus_pgat2nd,)
    }
};
        var_one_minus_pgat2nd = assign540_e1751;
        var_one_minus_pgat2nd_rv = 0.0;

        let (assign550_e1757,) = {
    if (var_guard4 != 0.0) {
        let assign550_e1755: f64 = (1.0 / var_one_minus_pgat2nd);
        (assign550_e1755,)
    } else {
        (var_one_over_one_minus_pgat2nd,)
    }
};
        var_one_over_one_minus_pgat2nd = assign550_e1757;
        var_one_over_one_minus_pgat2nd_rv = 0.0;

        let assign560_e1760: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign560_e1760;
        var_guard5_rv = 0.0;

        let (assign570_e1764,) = {
    if (var_guard5 != 0.0) {
        (p.p818,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign570_e1764;
        var_cjorbotd_i_rv = 0.0;

        let (assign580_e1768,) = {
    if (var_guard5 != 0.0) {
        (p.p819,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign580_e1768;
        var_cjorstid_i_rv = 0.0;

        let (assign590_e1772,) = {
    if (var_guard5 != 0.0) {
        (p.p820,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign590_e1772;
        var_cjorgatd_i_rv = 0.0;

        let (assign600_e1776,) = {
    if (var_guard5 != 0.0) {
        (p.p821,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign600_e1776;
        var_vbirbotd_i_rv = 0.0;

        let (assign610_e1780,) = {
    if (var_guard5 != 0.0) {
        (p.p822,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign610_e1780;
        var_vbirstid_i_rv = 0.0;

        let (assign620_e1784,) = {
    if (var_guard5 != 0.0) {
        (p.p823,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign620_e1784;
        var_vbirgatd_i_rv = 0.0;

        let (assign630_e1788,) = {
    if (var_guard5 != 0.0) {
        (p.p824,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign630_e1788;
        var_pbotd_i_rv = 0.0;

        let (assign640_e1792,) = {
    if (var_guard5 != 0.0) {
        (p.p825,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign640_e1792;
        var_pstid_i_rv = 0.0;

        let (assign650_e1796,) = {
    if (var_guard5 != 0.0) {
        (p.p826,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign650_e1796;
        var_pgatd_i_rv = 0.0;

        let (assign660_e1800,) = {
    if (var_guard5 != 0.0) {
        (p.p827,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign660_e1800;
        var_phigbotd_i_rv = 0.0;

        let (assign670_e1804,) = {
    if (var_guard5 != 0.0) {
        (p.p828,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign670_e1804;
        var_phigstid_i_rv = 0.0;

        let (assign680_e1808,) = {
    if (var_guard5 != 0.0) {
        (p.p829,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign680_e1808;
        var_phiggatd_i_rv = 0.0;

        let (assign690_e1812,) = {
    if (var_guard5 != 0.0) {
        (p.p830,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign690_e1812;
        var_idsatrbotd_i_rv = 0.0;

        let (assign700_e1816,) = {
    if (var_guard5 != 0.0) {
        (p.p831,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign700_e1816;
        var_idsatrstid_i_rv = 0.0;

        let (assign710_e1820,) = {
    if (var_guard5 != 0.0) {
        (p.p832,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign710_e1820;
        var_idsatrgatd_i_rv = 0.0;

        let (assign720_e1824,) = {
    if (var_guard5 != 0.0) {
        (p.p833,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign720_e1824;
        var_csrhbotd_i_rv = 0.0;

        let (assign730_e1828,) = {
    if (var_guard5 != 0.0) {
        (p.p834,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign730_e1828;
        var_csrhstid_i_rv = 0.0;

        let (assign740_e1832,) = {
    if (var_guard5 != 0.0) {
        (p.p835,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign740_e1832;
        var_csrhgatd_i_rv = 0.0;

        *var_alphaav_slot = var_alphaav;
        *var_alphaav_rv_slot = var_alphaav_rv;
        *var_chnl_type_slot = var_chnl_type;
        *var_chnl_type_rv_slot = var_chnl_type_rv;
        *var_cjorbotd_i_slot = var_cjorbotd_i;
        *var_cjorbotd_i_rv_slot = var_cjorbotd_i_rv;
        *var_cjorgat2nd_slot = var_cjorgat2nd;
        *var_cjorgat2nd_rv_slot = var_cjorgat2nd_rv;
        *var_cjorgatd_i_slot = var_cjorgatd_i;
        *var_cjorgatd_i_rv_slot = var_cjorgatd_i_rv;
        *var_cjorstid_i_slot = var_cjorstid_i;
        *var_cjorstid_i_rv_slot = var_cjorstid_i_rv;
        *var_csrhbotd_i_slot = var_csrhbotd_i;
        *var_csrhbotd_i_rv_slot = var_csrhbotd_i_rv;
        *var_csrhgatd_i_slot = var_csrhgatd_i;
        *var_csrhgatd_i_rv_slot = var_csrhgatd_i_rv;
        *var_csrhstid_i_slot = var_csrhstid_i;
        *var_csrhstid_i_rv_slot = var_csrhstid_i_rv;
        *var_deltaphigr_slot = var_deltaphigr;
        *var_deltaphigr_rv_slot = var_deltaphigr_rv;
        *var_epssi_slot = var_epssi;
        *var_epssi_rv_slot = var_epssi_rv;
        *var_guard1_slot = var_guard1;
        *var_guard1_rv_slot = var_guard1_rv;
        *var_guard2_slot = var_guard2;
        *var_guard2_rv_slot = var_guard2_rv;
        *var_guard3_slot = var_guard3;
        *var_guard3_rv_slot = var_guard3_rv;
        *var_guard4_slot = var_guard4;
        *var_guard4_rv_slot = var_guard4_rv;
        *var_guard5_slot = var_guard5;
        *var_guard5_rv_slot = var_guard5_rv;
        *var_idsatrbotd_i_slot = var_idsatrbotd_i;
        *var_idsatrbotd_i_rv_slot = var_idsatrbotd_i_rv;
        *var_idsatrgatd_i_slot = var_idsatrgatd_i;
        *var_idsatrgatd_i_rv_slot = var_idsatrgatd_i_rv;
        *var_idsatrstid_i_slot = var_idsatrstid_i;
        *var_idsatrstid_i_rv_slot = var_idsatrstid_i_rv;
        *var_kbol_over_qele_slot = var_kbol_over_qele;
        *var_kbol_over_qele_rv_slot = var_kbol_over_qele_rv;
        *var_one_minus_pbot_slot = var_one_minus_pbot;
        *var_one_minus_pbot_rv_slot = var_one_minus_pbot_rv;
        *var_one_minus_pgat_slot = var_one_minus_pgat;
        *var_one_minus_pgat2nd_slot = var_one_minus_pgat2nd;
        *var_one_minus_pgat2nd_rv_slot = var_one_minus_pgat2nd_rv;
        *var_one_minus_pgat_rv_slot = var_one_minus_pgat_rv;
        *var_one_minus_psti_slot = var_one_minus_psti;
        *var_one_minus_psti_rv_slot = var_one_minus_psti_rv;
        *var_one_over_one_minus_pbot_slot = var_one_over_one_minus_pbot;
        *var_one_over_one_minus_pbot_rv_slot = var_one_over_one_minus_pbot_rv;
        *var_one_over_one_minus_pgat_slot = var_one_over_one_minus_pgat;
        *var_one_over_one_minus_pgat2nd_slot = var_one_over_one_minus_pgat2nd;
        *var_one_over_one_minus_pgat2nd_rv_slot = var_one_over_one_minus_pgat2nd_rv;
        *var_one_over_one_minus_pgat_rv_slot = var_one_over_one_minus_pgat_rv;
        *var_one_over_one_minus_psti_slot = var_one_over_one_minus_psti;
        *var_one_over_one_minus_psti_rv_slot = var_one_over_one_minus_psti_rv;
        *var_pbotd_i_slot = var_pbotd_i;
        *var_pbotd_i_rv_slot = var_pbotd_i_rv;
        *var_pgat2nd_slot = var_pgat2nd;
        *var_pgat2nd_rv_slot = var_pgat2nd_rv;
        *var_pgatd_i_slot = var_pgatd_i;
        *var_pgatd_i_rv_slot = var_pgatd_i_rv;
        *var_phigbotd_i_slot = var_phigbotd_i;
        *var_phigbotd_i_rv_slot = var_phigbotd_i_rv;
        *var_phiggat2nd_slot = var_phiggat2nd;
        *var_phiggat2nd_rv_slot = var_phiggat2nd_rv;
        *var_phiggatd_i_slot = var_phiggatd_i;
        *var_phiggatd_i_rv_slot = var_phiggatd_i_rv;
        *var_phigrbot_slot = var_phigrbot;
        *var_phigrbot_rv_slot = var_phigrbot_rv;
        *var_phigrgat_slot = var_phigrgat;
        *var_phigrgat2nd_slot = var_phigrgat2nd;
        *var_phigrgat2nd_rv_slot = var_phigrgat2nd_rv;
        *var_phigrgat_rv_slot = var_phigrgat_rv;
        *var_phigrsti_slot = var_phigrsti;
        *var_phigrsti_rv_slot = var_phigrsti_rv;
        *var_phigstid_i_slot = var_phigstid_i;
        *var_phigstid_i_rv_slot = var_phigstid_i_rv;
        *var_phitr_slot = var_phitr;
        *var_phitr_rv_slot = var_phitr_rv;
        *var_phitrinv_slot = var_phitrinv;
        *var_phitrinv_rv_slot = var_phitrinv_rv;
        *var_pstid_i_slot = var_pstid_i;
        *var_pstid_i_rv_slot = var_pstid_i_rv;
        *var_swgat2nd_slot = var_swgat2nd;
        *var_swgat2nd_rv_slot = var_swgat2nd_rv;
        *var_swjunexp_i_slot = var_swjunexp_i;
        *var_swjunexp_i_rv_slot = var_swjunexp_i_rv;
        *var_tkr_slot = var_tkr;
        *var_tkr_1_slot = var_tkr_1;
        *var_tkr_1_rv_slot = var_tkr_1_rv;
        *var_tkr_rv_slot = var_tkr_rv;
        *var_vbirbotd_i_slot = var_vbirbotd_i;
        *var_vbirbotd_i_rv_slot = var_vbirbotd_i_rv;
        *var_vbirbotinv_slot = var_vbirbotinv;
        *var_vbirbotinv_rv_slot = var_vbirbotinv_rv;
        *var_vbirgat2nd_slot = var_vbirgat2nd;
        *var_vbirgat2nd_rv_slot = var_vbirgat2nd_rv;
        *var_vbirgatd_i_slot = var_vbirgatd_i;
        *var_vbirgatd_i_rv_slot = var_vbirgatd_i_rv;
        *var_vbirgatinv_slot = var_vbirgatinv;
        *var_vbirgatinv_rv_slot = var_vbirgatinv_rv;
        *var_vbirstid_i_slot = var_vbirstid_i;
        *var_vbirstid_i_rv_slot = var_vbirstid_i_rv;
        *var_vbirstiinv_slot = var_vbirstiinv;
        *var_vbirstiinv_rv_slot = var_vbirstiinv_rv;
        *var_vbrinvbot_slot = var_vbrinvbot;
        *var_vbrinvbot_rv_slot = var_vbrinvbot_rv;
        *var_vbrinvgat_slot = var_vbrinvgat;
        *var_vbrinvgat_dn5_slot = var_vbrinvgat_dn5;
        *var_vbrinvgat_dn6_slot = var_vbrinvgat_dn6;
        *var_vbrinvgat_dn7_slot = var_vbrinvgat_dn7;
        *var_vbrinvgat_dn8_slot = var_vbrinvgat_dn8;
        *var_vbrinvgat_rv_slot = var_vbrinvgat_rv;
        *var_vbrinvsti_slot = var_vbrinvsti;
        *var_vbrinvsti_rv_slot = var_vbrinvsti_rv;
        *var_wdepnulrbot_slot = var_wdepnulrbot;
        *var_wdepnulrbot_rv_slot = var_wdepnulrbot_rv;
        *var_wdepnulrgat_slot = var_wdepnulrgat;
        *var_wdepnulrgat_rv_slot = var_wdepnulrgat_rv;
        *var_wdepnulrinvbot_slot = var_wdepnulrinvbot;
        *var_wdepnulrinvbot_rv_slot = var_wdepnulrinvbot_rv;
        *var_wdepnulrinvgat_slot = var_wdepnulrinvgat;
        *var_wdepnulrinvgat_rv_slot = var_wdepnulrinvgat_rv;
        *var_wdepnulrinvsti_slot = var_wdepnulrinvsti;
        *var_wdepnulrinvsti_rv_slot = var_wdepnulrinvsti_rv;
        *var_wdepnulrsti_slot = var_wdepnulrsti;
        *var_wdepnulrsti_rv_slot = var_wdepnulrsti_rv;
    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        var_guard5: f64,
        var_adbbtgatd_i_slot: &mut f64,
        var_adbbtgatd_i_rv_slot: &mut f64,
        var_advbrgatd_i_slot: &mut f64,
        var_advbrgatd_i_rv_slot: &mut f64,
        var_anugatd_i_slot: &mut f64,
        var_anugatd_i_rv_slot: &mut f64,
        var_bdbbtgatd_i_slot: &mut f64,
        var_bdbbtgatd_i_rv_slot: &mut f64,
        var_bdvbrgatd_i_slot: &mut f64,
        var_bdvbrgatd_i_rv_slot: &mut f64,
        var_cbbtbotd_i_slot: &mut f64,
        var_cbbtbotd_i_rv_slot: &mut f64,
        var_cbbtgatd_i_slot: &mut f64,
        var_cbbtgatd_i_rv_slot: &mut f64,
        var_cbbtstid_i_slot: &mut f64,
        var_cbbtstid_i_rv_slot: &mut f64,
        var_cjorbotd_i_slot: &mut f64,
        var_cjorbotd_i_rv_slot: &mut f64,
        var_cjorgatd_i_slot: &mut f64,
        var_cjorgatd_i_rv_slot: &mut f64,
        var_cjorstid_i_slot: &mut f64,
        var_cjorstid_i_rv_slot: &mut f64,
        var_csrhbotd_i_slot: &mut f64,
        var_csrhbotd_i_rv_slot: &mut f64,
        var_csrhstid_i_slot: &mut f64,
        var_csrhstid_i_rv_slot: &mut f64,
        var_ctatbotd_i_slot: &mut f64,
        var_ctatbotd_i_rv_slot: &mut f64,
        var_ctatgatd_i_slot: &mut f64,
        var_ctatgatd_i_rv_slot: &mut f64,
        var_ctatstid_i_slot: &mut f64,
        var_ctatstid_i_rv_slot: &mut f64,
        var_fbbtrbotd_i_slot: &mut f64,
        var_fbbtrbotd_i_rv_slot: &mut f64,
        var_fbbtrgatd_i_slot: &mut f64,
        var_fbbtrgatd_i_rv_slot: &mut f64,
        var_fbbtrstid_i_slot: &mut f64,
        var_fbbtrstid_i_rv_slot: &mut f64,
        var_fcjorgat2d_i_slot: &mut f64,
        var_fcjorgat2d_i_rv_slot: &mut f64,
        var_fjunqd_i_slot: &mut f64,
        var_fjunqd_i_rv_slot: &mut f64,
        var_fpgat2d_i_slot: &mut f64,
        var_fpgat2d_i_rv_slot: &mut f64,
        var_fphiggat2d_i_slot: &mut f64,
        var_fphiggat2d_i_rv_slot: &mut f64,
        var_fvbirgat2d_i_slot: &mut f64,
        var_fvbirgat2d_i_rv_slot: &mut f64,
        var_idsatrbotd_i_slot: &mut f64,
        var_idsatrbotd_i_rv_slot: &mut f64,
        var_idsatrgatd_i_slot: &mut f64,
        var_idsatrgatd_i_rv_slot: &mut f64,
        var_idsatrstid_i_slot: &mut f64,
        var_idsatrstid_i_rv_slot: &mut f64,
        var_mefftatbotd_i_slot: &mut f64,
        var_mefftatbotd_i_rv_slot: &mut f64,
        var_mefftatgatd_i_slot: &mut f64,
        var_mefftatgatd_i_rv_slot: &mut f64,
        var_mefftatstid_i_slot: &mut f64,
        var_mefftatstid_i_rv_slot: &mut f64,
        var_pbotd_i_slot: &mut f64,
        var_pbotd_i_rv_slot: &mut f64,
        var_pbrbotd_i_slot: &mut f64,
        var_pbrbotd_i_rv_slot: &mut f64,
        var_pbrgatd_i_slot: &mut f64,
        var_pbrgatd_i_rv_slot: &mut f64,
        var_pbrstid_i_slot: &mut f64,
        var_pbrstid_i_rv_slot: &mut f64,
        var_pgatd_i_slot: &mut f64,
        var_pgatd_i_rv_slot: &mut f64,
        var_phigbotd_i_slot: &mut f64,
        var_phigbotd_i_rv_slot: &mut f64,
        var_phiggatd_i_slot: &mut f64,
        var_phiggatd_i_rv_slot: &mut f64,
        var_phigstid_i_slot: &mut f64,
        var_phigstid_i_rv_slot: &mut f64,
        var_pstid_i_slot: &mut f64,
        var_pstid_i_rv_slot: &mut f64,
        var_stfbbtbotd_i_slot: &mut f64,
        var_stfbbtbotd_i_rv_slot: &mut f64,
        var_stfbbtgatd_i_slot: &mut f64,
        var_stfbbtgatd_i_rv_slot: &mut f64,
        var_stfbbtstid_i_slot: &mut f64,
        var_stfbbtstid_i_rv_slot: &mut f64,
        var_vbirbotd_i_slot: &mut f64,
        var_vbirbotd_i_rv_slot: &mut f64,
        var_vbirgatd_i_slot: &mut f64,
        var_vbirgatd_i_rv_slot: &mut f64,
        var_vbirstid_i_slot: &mut f64,
        var_vbirstid_i_rv_slot: &mut f64,
        var_vbrbotd_i_slot: &mut f64,
        var_vbrbotd_i_rv_slot: &mut f64,
        var_vbrgatd_i_slot: &mut f64,
        var_vbrgatd_i_rv_slot: &mut f64,
        var_vbrstid_i_slot: &mut f64,
        var_vbrstid_i_rv_slot: &mut f64,
        var_vtrgatd_i_slot: &mut f64,
        var_vtrgatd_i_rv_slot: &mut f64,
        var_xjungatd_i_slot: &mut f64,
        var_xjungatd_i_rv_slot: &mut f64,
        var_xjunstid_i_slot: &mut f64,
        var_xjunstid_i_rv_slot: &mut f64,
    ) {
        let mut var_adbbtgatd_i: f64 = *var_adbbtgatd_i_slot;
        let mut var_adbbtgatd_i_rv: f64 = *var_adbbtgatd_i_rv_slot;
        let mut var_advbrgatd_i: f64 = *var_advbrgatd_i_slot;
        let mut var_advbrgatd_i_rv: f64 = *var_advbrgatd_i_rv_slot;
        let mut var_anugatd_i: f64 = *var_anugatd_i_slot;
        let mut var_anugatd_i_rv: f64 = *var_anugatd_i_rv_slot;
        let mut var_bdbbtgatd_i: f64 = *var_bdbbtgatd_i_slot;
        let mut var_bdbbtgatd_i_rv: f64 = *var_bdbbtgatd_i_rv_slot;
        let mut var_bdvbrgatd_i: f64 = *var_bdvbrgatd_i_slot;
        let mut var_bdvbrgatd_i_rv: f64 = *var_bdvbrgatd_i_rv_slot;
        let mut var_cbbtbotd_i: f64 = *var_cbbtbotd_i_slot;
        let mut var_cbbtbotd_i_rv: f64 = *var_cbbtbotd_i_rv_slot;
        let mut var_cbbtgatd_i: f64 = *var_cbbtgatd_i_slot;
        let mut var_cbbtgatd_i_rv: f64 = *var_cbbtgatd_i_rv_slot;
        let mut var_cbbtstid_i: f64 = *var_cbbtstid_i_slot;
        let mut var_cbbtstid_i_rv: f64 = *var_cbbtstid_i_rv_slot;
        let mut var_cjorbotd_i: f64 = *var_cjorbotd_i_slot;
        let mut var_cjorbotd_i_rv: f64 = *var_cjorbotd_i_rv_slot;
        let mut var_cjorgatd_i: f64 = *var_cjorgatd_i_slot;
        let mut var_cjorgatd_i_rv: f64 = *var_cjorgatd_i_rv_slot;
        let mut var_cjorstid_i: f64 = *var_cjorstid_i_slot;
        let mut var_cjorstid_i_rv: f64 = *var_cjorstid_i_rv_slot;
        let mut var_csrhbotd_i: f64 = *var_csrhbotd_i_slot;
        let mut var_csrhbotd_i_rv: f64 = *var_csrhbotd_i_rv_slot;
        let mut var_csrhstid_i: f64 = *var_csrhstid_i_slot;
        let mut var_csrhstid_i_rv: f64 = *var_csrhstid_i_rv_slot;
        let mut var_ctatbotd_i: f64 = *var_ctatbotd_i_slot;
        let mut var_ctatbotd_i_rv: f64 = *var_ctatbotd_i_rv_slot;
        let mut var_ctatgatd_i: f64 = *var_ctatgatd_i_slot;
        let mut var_ctatgatd_i_rv: f64 = *var_ctatgatd_i_rv_slot;
        let mut var_ctatstid_i: f64 = *var_ctatstid_i_slot;
        let mut var_ctatstid_i_rv: f64 = *var_ctatstid_i_rv_slot;
        let mut var_fbbtrbotd_i: f64 = *var_fbbtrbotd_i_slot;
        let mut var_fbbtrbotd_i_rv: f64 = *var_fbbtrbotd_i_rv_slot;
        let mut var_fbbtrgatd_i: f64 = *var_fbbtrgatd_i_slot;
        let mut var_fbbtrgatd_i_rv: f64 = *var_fbbtrgatd_i_rv_slot;
        let mut var_fbbtrstid_i: f64 = *var_fbbtrstid_i_slot;
        let mut var_fbbtrstid_i_rv: f64 = *var_fbbtrstid_i_rv_slot;
        let mut var_fcjorgat2d_i: f64 = *var_fcjorgat2d_i_slot;
        let mut var_fcjorgat2d_i_rv: f64 = *var_fcjorgat2d_i_rv_slot;
        let mut var_fjunqd_i: f64 = *var_fjunqd_i_slot;
        let mut var_fjunqd_i_rv: f64 = *var_fjunqd_i_rv_slot;
        let mut var_fpgat2d_i: f64 = *var_fpgat2d_i_slot;
        let mut var_fpgat2d_i_rv: f64 = *var_fpgat2d_i_rv_slot;
        let mut var_fphiggat2d_i: f64 = *var_fphiggat2d_i_slot;
        let mut var_fphiggat2d_i_rv: f64 = *var_fphiggat2d_i_rv_slot;
        let mut var_fvbirgat2d_i: f64 = *var_fvbirgat2d_i_slot;
        let mut var_fvbirgat2d_i_rv: f64 = *var_fvbirgat2d_i_rv_slot;
        let mut var_idsatrbotd_i: f64 = *var_idsatrbotd_i_slot;
        let mut var_idsatrbotd_i_rv: f64 = *var_idsatrbotd_i_rv_slot;
        let mut var_idsatrgatd_i: f64 = *var_idsatrgatd_i_slot;
        let mut var_idsatrgatd_i_rv: f64 = *var_idsatrgatd_i_rv_slot;
        let mut var_idsatrstid_i: f64 = *var_idsatrstid_i_slot;
        let mut var_idsatrstid_i_rv: f64 = *var_idsatrstid_i_rv_slot;
        let mut var_mefftatbotd_i: f64 = *var_mefftatbotd_i_slot;
        let mut var_mefftatbotd_i_rv: f64 = *var_mefftatbotd_i_rv_slot;
        let mut var_mefftatgatd_i: f64 = *var_mefftatgatd_i_slot;
        let mut var_mefftatgatd_i_rv: f64 = *var_mefftatgatd_i_rv_slot;
        let mut var_mefftatstid_i: f64 = *var_mefftatstid_i_slot;
        let mut var_mefftatstid_i_rv: f64 = *var_mefftatstid_i_rv_slot;
        let mut var_pbotd_i: f64 = *var_pbotd_i_slot;
        let mut var_pbotd_i_rv: f64 = *var_pbotd_i_rv_slot;
        let mut var_pbrbotd_i: f64 = *var_pbrbotd_i_slot;
        let mut var_pbrbotd_i_rv: f64 = *var_pbrbotd_i_rv_slot;
        let mut var_pbrgatd_i: f64 = *var_pbrgatd_i_slot;
        let mut var_pbrgatd_i_rv: f64 = *var_pbrgatd_i_rv_slot;
        let mut var_pbrstid_i: f64 = *var_pbrstid_i_slot;
        let mut var_pbrstid_i_rv: f64 = *var_pbrstid_i_rv_slot;
        let mut var_pgatd_i: f64 = *var_pgatd_i_slot;
        let mut var_pgatd_i_rv: f64 = *var_pgatd_i_rv_slot;
        let mut var_phigbotd_i: f64 = *var_phigbotd_i_slot;
        let mut var_phigbotd_i_rv: f64 = *var_phigbotd_i_rv_slot;
        let mut var_phiggatd_i: f64 = *var_phiggatd_i_slot;
        let mut var_phiggatd_i_rv: f64 = *var_phiggatd_i_rv_slot;
        let mut var_phigstid_i: f64 = *var_phigstid_i_slot;
        let mut var_phigstid_i_rv: f64 = *var_phigstid_i_rv_slot;
        let mut var_pstid_i: f64 = *var_pstid_i_slot;
        let mut var_pstid_i_rv: f64 = *var_pstid_i_rv_slot;
        let mut var_stfbbtbotd_i: f64 = *var_stfbbtbotd_i_slot;
        let mut var_stfbbtbotd_i_rv: f64 = *var_stfbbtbotd_i_rv_slot;
        let mut var_stfbbtgatd_i: f64 = *var_stfbbtgatd_i_slot;
        let mut var_stfbbtgatd_i_rv: f64 = *var_stfbbtgatd_i_rv_slot;
        let mut var_stfbbtstid_i: f64 = *var_stfbbtstid_i_slot;
        let mut var_stfbbtstid_i_rv: f64 = *var_stfbbtstid_i_rv_slot;
        let mut var_vbirbotd_i: f64 = *var_vbirbotd_i_slot;
        let mut var_vbirbotd_i_rv: f64 = *var_vbirbotd_i_rv_slot;
        let mut var_vbirgatd_i: f64 = *var_vbirgatd_i_slot;
        let mut var_vbirgatd_i_rv: f64 = *var_vbirgatd_i_rv_slot;
        let mut var_vbirstid_i: f64 = *var_vbirstid_i_slot;
        let mut var_vbirstid_i_rv: f64 = *var_vbirstid_i_rv_slot;
        let mut var_vbrbotd_i: f64 = *var_vbrbotd_i_slot;
        let mut var_vbrbotd_i_rv: f64 = *var_vbrbotd_i_rv_slot;
        let mut var_vbrgatd_i: f64 = *var_vbrgatd_i_slot;
        let mut var_vbrgatd_i_rv: f64 = *var_vbrgatd_i_rv_slot;
        let mut var_vbrstid_i: f64 = *var_vbrstid_i_slot;
        let mut var_vbrstid_i_rv: f64 = *var_vbrstid_i_rv_slot;
        let mut var_vtrgatd_i: f64 = *var_vtrgatd_i_slot;
        let mut var_vtrgatd_i_rv: f64 = *var_vtrgatd_i_rv_slot;
        let mut var_xjungatd_i: f64 = *var_xjungatd_i_slot;
        let mut var_xjungatd_i_rv: f64 = *var_xjungatd_i_rv_slot;
        let mut var_xjunstid_i: f64 = *var_xjunstid_i_slot;
        let mut var_xjunstid_i_rv: f64 = *var_xjunstid_i_rv_slot;

        let (assign750_e1836,) = {
    if (var_guard5 != 0.0) {
        (p.p836,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign750_e1836;
        var_xjunstid_i_rv = 0.0;

        let (assign760_e1840,) = {
    if (var_guard5 != 0.0) {
        (p.p837,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign760_e1840;
        var_xjungatd_i_rv = 0.0;

        let (assign770_e1844,) = {
    if (var_guard5 != 0.0) {
        (p.p838,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign770_e1844;
        var_ctatbotd_i_rv = 0.0;

        let (assign780_e1848,) = {
    if (var_guard5 != 0.0) {
        (p.p839,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign780_e1848;
        var_ctatstid_i_rv = 0.0;

        let (assign790_e1852,) = {
    if (var_guard5 != 0.0) {
        (p.p840,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign790_e1852;
        var_ctatgatd_i_rv = 0.0;

        let (assign800_e1856,) = {
    if (var_guard5 != 0.0) {
        (p.p841,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign800_e1856;
        var_mefftatbotd_i_rv = 0.0;

        let (assign810_e1860,) = {
    if (var_guard5 != 0.0) {
        (p.p842,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign810_e1860;
        var_mefftatstid_i_rv = 0.0;

        let (assign820_e1864,) = {
    if (var_guard5 != 0.0) {
        (p.p843,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign820_e1864;
        var_mefftatgatd_i_rv = 0.0;

        let (assign830_e1868,) = {
    if (var_guard5 != 0.0) {
        (p.p844,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign830_e1868;
        var_cbbtbotd_i_rv = 0.0;

        let (assign840_e1872,) = {
    if (var_guard5 != 0.0) {
        (p.p845,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign840_e1872;
        var_cbbtstid_i_rv = 0.0;

        let (assign850_e1876,) = {
    if (var_guard5 != 0.0) {
        (p.p846,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign850_e1876;
        var_cbbtgatd_i_rv = 0.0;

        let (assign860_e1880,) = {
    if (var_guard5 != 0.0) {
        (p.p847,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign860_e1880;
        var_fbbtrbotd_i_rv = 0.0;

        let (assign870_e1884,) = {
    if (var_guard5 != 0.0) {
        (p.p848,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign870_e1884;
        var_fbbtrstid_i_rv = 0.0;

        let (assign880_e1888,) = {
    if (var_guard5 != 0.0) {
        (p.p849,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign880_e1888;
        var_fbbtrgatd_i_rv = 0.0;

        let (assign890_e1892,) = {
    if (var_guard5 != 0.0) {
        (p.p850,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign890_e1892;
        var_stfbbtbotd_i_rv = 0.0;

        let (assign900_e1896,) = {
    if (var_guard5 != 0.0) {
        (p.p851,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign900_e1896;
        var_stfbbtstid_i_rv = 0.0;

        let (assign910_e1900,) = {
    if (var_guard5 != 0.0) {
        (p.p852,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign910_e1900;
        var_stfbbtgatd_i_rv = 0.0;

        let (assign920_e1904,) = {
    if (var_guard5 != 0.0) {
        (p.p853,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign920_e1904;
        var_vbrbotd_i_rv = 0.0;

        let (assign930_e1908,) = {
    if (var_guard5 != 0.0) {
        (p.p854,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign930_e1908;
        var_vbrstid_i_rv = 0.0;

        let (assign940_e1912,) = {
    if (var_guard5 != 0.0) {
        (p.p855,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign940_e1912;
        var_vbrgatd_i_rv = 0.0;

        let (assign950_e1916,) = {
    if (var_guard5 != 0.0) {
        (p.p856,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign950_e1916;
        var_pbrbotd_i_rv = 0.0;

        let (assign960_e1920,) = {
    if (var_guard5 != 0.0) {
        (p.p857,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign960_e1920;
        var_pbrstid_i_rv = 0.0;

        let (assign970_e1924,) = {
    if (var_guard5 != 0.0) {
        (p.p858,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign970_e1924;
        var_pbrgatd_i_rv = 0.0;

        let (assign990_e1932,) = {
    if (var_guard5 != 0.0) {
        (p.p922,)
    } else {
        (var_fjunqd_i,)
    }
};
        var_fjunqd_i = assign990_e1932;
        var_fjunqd_i_rv = 0.0;

        let (assign1000_e1936,) = {
    if (var_guard5 != 0.0) {
        (p.p865,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1000_e1936;
        var_advbrgatd_i_rv = 0.0;

        let (assign1010_e1940,) = {
    if (var_guard5 != 0.0) {
        (p.p866,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1010_e1940;
        var_bdvbrgatd_i_rv = 0.0;

        let (assign1020_e1944,) = {
    if (var_guard5 != 0.0) {
        (p.p867,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1020_e1944;
        var_adbbtgatd_i_rv = 0.0;

        let (assign1030_e1948,) = {
    if (var_guard5 != 0.0) {
        (p.p868,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1030_e1948;
        var_bdbbtgatd_i_rv = 0.0;

        let (assign1040_e1952,) = {
    if (var_guard5 != 0.0) {
        (p.p859,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1040_e1952;
        var_fcjorgat2d_i_rv = 0.0;

        let (assign1050_e1956,) = {
    if (var_guard5 != 0.0) {
        (p.p860,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1050_e1956;
        var_fvbirgat2d_i_rv = 0.0;

        let (assign1060_e1960,) = {
    if (var_guard5 != 0.0) {
        (p.p861,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1060_e1960;
        var_fpgat2d_i_rv = 0.0;

        let (assign1070_e1964,) = {
    if (var_guard5 != 0.0) {
        (p.p862,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1070_e1964;
        var_fphiggat2d_i_rv = 0.0;

        let (assign1080_e1968,) = {
    if (var_guard5 != 0.0) {
        (p.p863,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1080_e1968;
        var_vtrgatd_i_rv = 0.0;

        let (assign1090_e1972,) = {
    if (var_guard5 != 0.0) {
        (p.p864,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1090_e1972;
        var_anugatd_i_rv = 0.0;

        let (assign1100_e1977,) = {
    if (var_guard5 == 0.0) {
        (p.p869,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign1100_e1977;
        var_cjorbotd_i_rv = 0.0;

        let (assign1110_e1982,) = {
    if (var_guard5 == 0.0) {
        (p.p870,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign1110_e1982;
        var_cjorstid_i_rv = 0.0;

        let (assign1120_e1987,) = {
    if (var_guard5 == 0.0) {
        (p.p871,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign1120_e1987;
        var_cjorgatd_i_rv = 0.0;

        let (assign1130_e1992,) = {
    if (var_guard5 == 0.0) {
        (p.p872,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign1130_e1992;
        var_vbirbotd_i_rv = 0.0;

        let (assign1140_e1997,) = {
    if (var_guard5 == 0.0) {
        (p.p873,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign1140_e1997;
        var_vbirstid_i_rv = 0.0;

        let (assign1150_e2002,) = {
    if (var_guard5 == 0.0) {
        (p.p874,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign1150_e2002;
        var_vbirgatd_i_rv = 0.0;

        let (assign1160_e2007,) = {
    if (var_guard5 == 0.0) {
        (p.p875,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign1160_e2007;
        var_pbotd_i_rv = 0.0;

        let (assign1170_e2012,) = {
    if (var_guard5 == 0.0) {
        (p.p876,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign1170_e2012;
        var_pstid_i_rv = 0.0;

        let (assign1180_e2017,) = {
    if (var_guard5 == 0.0) {
        (p.p877,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign1180_e2017;
        var_pgatd_i_rv = 0.0;

        let (assign1190_e2022,) = {
    if (var_guard5 == 0.0) {
        (p.p878,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign1190_e2022;
        var_phigbotd_i_rv = 0.0;

        let (assign1200_e2027,) = {
    if (var_guard5 == 0.0) {
        (p.p879,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign1200_e2027;
        var_phigstid_i_rv = 0.0;

        let (assign1210_e2032,) = {
    if (var_guard5 == 0.0) {
        (p.p880,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign1210_e2032;
        var_phiggatd_i_rv = 0.0;

        let (assign1220_e2037,) = {
    if (var_guard5 == 0.0) {
        (p.p881,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign1220_e2037;
        var_idsatrbotd_i_rv = 0.0;

        let (assign1230_e2042,) = {
    if (var_guard5 == 0.0) {
        (p.p882,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign1230_e2042;
        var_idsatrstid_i_rv = 0.0;

        let (assign1240_e2047,) = {
    if (var_guard5 == 0.0) {
        (p.p883,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign1240_e2047;
        var_idsatrgatd_i_rv = 0.0;

        let (assign1250_e2052,) = {
    if (var_guard5 == 0.0) {
        (p.p884,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign1250_e2052;
        var_csrhbotd_i_rv = 0.0;

        let (assign1260_e2057,) = {
    if (var_guard5 == 0.0) {
        (p.p885,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign1260_e2057;
        var_csrhstid_i_rv = 0.0;

        *var_adbbtgatd_i_slot = var_adbbtgatd_i;
        *var_adbbtgatd_i_rv_slot = var_adbbtgatd_i_rv;
        *var_advbrgatd_i_slot = var_advbrgatd_i;
        *var_advbrgatd_i_rv_slot = var_advbrgatd_i_rv;
        *var_anugatd_i_slot = var_anugatd_i;
        *var_anugatd_i_rv_slot = var_anugatd_i_rv;
        *var_bdbbtgatd_i_slot = var_bdbbtgatd_i;
        *var_bdbbtgatd_i_rv_slot = var_bdbbtgatd_i_rv;
        *var_bdvbrgatd_i_slot = var_bdvbrgatd_i;
        *var_bdvbrgatd_i_rv_slot = var_bdvbrgatd_i_rv;
        *var_cbbtbotd_i_slot = var_cbbtbotd_i;
        *var_cbbtbotd_i_rv_slot = var_cbbtbotd_i_rv;
        *var_cbbtgatd_i_slot = var_cbbtgatd_i;
        *var_cbbtgatd_i_rv_slot = var_cbbtgatd_i_rv;
        *var_cbbtstid_i_slot = var_cbbtstid_i;
        *var_cbbtstid_i_rv_slot = var_cbbtstid_i_rv;
        *var_cjorbotd_i_slot = var_cjorbotd_i;
        *var_cjorbotd_i_rv_slot = var_cjorbotd_i_rv;
        *var_cjorgatd_i_slot = var_cjorgatd_i;
        *var_cjorgatd_i_rv_slot = var_cjorgatd_i_rv;
        *var_cjorstid_i_slot = var_cjorstid_i;
        *var_cjorstid_i_rv_slot = var_cjorstid_i_rv;
        *var_csrhbotd_i_slot = var_csrhbotd_i;
        *var_csrhbotd_i_rv_slot = var_csrhbotd_i_rv;
        *var_csrhstid_i_slot = var_csrhstid_i;
        *var_csrhstid_i_rv_slot = var_csrhstid_i_rv;
        *var_ctatbotd_i_slot = var_ctatbotd_i;
        *var_ctatbotd_i_rv_slot = var_ctatbotd_i_rv;
        *var_ctatgatd_i_slot = var_ctatgatd_i;
        *var_ctatgatd_i_rv_slot = var_ctatgatd_i_rv;
        *var_ctatstid_i_slot = var_ctatstid_i;
        *var_ctatstid_i_rv_slot = var_ctatstid_i_rv;
        *var_fbbtrbotd_i_slot = var_fbbtrbotd_i;
        *var_fbbtrbotd_i_rv_slot = var_fbbtrbotd_i_rv;
        *var_fbbtrgatd_i_slot = var_fbbtrgatd_i;
        *var_fbbtrgatd_i_rv_slot = var_fbbtrgatd_i_rv;
        *var_fbbtrstid_i_slot = var_fbbtrstid_i;
        *var_fbbtrstid_i_rv_slot = var_fbbtrstid_i_rv;
        *var_fcjorgat2d_i_slot = var_fcjorgat2d_i;
        *var_fcjorgat2d_i_rv_slot = var_fcjorgat2d_i_rv;
        *var_fjunqd_i_slot = var_fjunqd_i;
        *var_fjunqd_i_rv_slot = var_fjunqd_i_rv;
        *var_fpgat2d_i_slot = var_fpgat2d_i;
        *var_fpgat2d_i_rv_slot = var_fpgat2d_i_rv;
        *var_fphiggat2d_i_slot = var_fphiggat2d_i;
        *var_fphiggat2d_i_rv_slot = var_fphiggat2d_i_rv;
        *var_fvbirgat2d_i_slot = var_fvbirgat2d_i;
        *var_fvbirgat2d_i_rv_slot = var_fvbirgat2d_i_rv;
        *var_idsatrbotd_i_slot = var_idsatrbotd_i;
        *var_idsatrbotd_i_rv_slot = var_idsatrbotd_i_rv;
        *var_idsatrgatd_i_slot = var_idsatrgatd_i;
        *var_idsatrgatd_i_rv_slot = var_idsatrgatd_i_rv;
        *var_idsatrstid_i_slot = var_idsatrstid_i;
        *var_idsatrstid_i_rv_slot = var_idsatrstid_i_rv;
        *var_mefftatbotd_i_slot = var_mefftatbotd_i;
        *var_mefftatbotd_i_rv_slot = var_mefftatbotd_i_rv;
        *var_mefftatgatd_i_slot = var_mefftatgatd_i;
        *var_mefftatgatd_i_rv_slot = var_mefftatgatd_i_rv;
        *var_mefftatstid_i_slot = var_mefftatstid_i;
        *var_mefftatstid_i_rv_slot = var_mefftatstid_i_rv;
        *var_pbotd_i_slot = var_pbotd_i;
        *var_pbotd_i_rv_slot = var_pbotd_i_rv;
        *var_pbrbotd_i_slot = var_pbrbotd_i;
        *var_pbrbotd_i_rv_slot = var_pbrbotd_i_rv;
        *var_pbrgatd_i_slot = var_pbrgatd_i;
        *var_pbrgatd_i_rv_slot = var_pbrgatd_i_rv;
        *var_pbrstid_i_slot = var_pbrstid_i;
        *var_pbrstid_i_rv_slot = var_pbrstid_i_rv;
        *var_pgatd_i_slot = var_pgatd_i;
        *var_pgatd_i_rv_slot = var_pgatd_i_rv;
        *var_phigbotd_i_slot = var_phigbotd_i;
        *var_phigbotd_i_rv_slot = var_phigbotd_i_rv;
        *var_phiggatd_i_slot = var_phiggatd_i;
        *var_phiggatd_i_rv_slot = var_phiggatd_i_rv;
        *var_phigstid_i_slot = var_phigstid_i;
        *var_phigstid_i_rv_slot = var_phigstid_i_rv;
        *var_pstid_i_slot = var_pstid_i;
        *var_pstid_i_rv_slot = var_pstid_i_rv;
        *var_stfbbtbotd_i_slot = var_stfbbtbotd_i;
        *var_stfbbtbotd_i_rv_slot = var_stfbbtbotd_i_rv;
        *var_stfbbtgatd_i_slot = var_stfbbtgatd_i;
        *var_stfbbtgatd_i_rv_slot = var_stfbbtgatd_i_rv;
        *var_stfbbtstid_i_slot = var_stfbbtstid_i;
        *var_stfbbtstid_i_rv_slot = var_stfbbtstid_i_rv;
        *var_vbirbotd_i_slot = var_vbirbotd_i;
        *var_vbirbotd_i_rv_slot = var_vbirbotd_i_rv;
        *var_vbirgatd_i_slot = var_vbirgatd_i;
        *var_vbirgatd_i_rv_slot = var_vbirgatd_i_rv;
        *var_vbirstid_i_slot = var_vbirstid_i;
        *var_vbirstid_i_rv_slot = var_vbirstid_i_rv;
        *var_vbrbotd_i_slot = var_vbrbotd_i;
        *var_vbrbotd_i_rv_slot = var_vbrbotd_i_rv;
        *var_vbrgatd_i_slot = var_vbrgatd_i;
        *var_vbrgatd_i_rv_slot = var_vbrgatd_i_rv;
        *var_vbrstid_i_slot = var_vbrstid_i;
        *var_vbrstid_i_rv_slot = var_vbrstid_i_rv;
        *var_vtrgatd_i_slot = var_vtrgatd_i;
        *var_vtrgatd_i_rv_slot = var_vtrgatd_i_rv;
        *var_xjungatd_i_slot = var_xjungatd_i;
        *var_xjungatd_i_rv_slot = var_xjungatd_i_rv;
        *var_xjunstid_i_slot = var_xjunstid_i;
        *var_xjunstid_i_rv_slot = var_xjunstid_i_rv;
    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        var_cjorbotd_i: f64,
        var_cjorgatd_i: f64,
        var_cjorstid_i: f64,
        var_deltaphigr: f64,
        var_epssi: f64,
        var_guard5: f64,
        var_pbotd_i: f64,
        var_pgatd_i: f64,
        var_phigbotd_i: f64,
        var_phiggatd_i: f64,
        var_phigstid_i: f64,
        var_pstid_i: f64,
        var_vbirbotd_i: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_adbbtgatd_i_slot: &mut f64,
        var_adbbtgatd_i_rv_slot: &mut f64,
        var_advbrgatd_i_slot: &mut f64,
        var_advbrgatd_i_rv_slot: &mut f64,
        var_anugatd_i_slot: &mut f64,
        var_anugatd_i_rv_slot: &mut f64,
        var_bdbbtgatd_i_slot: &mut f64,
        var_bdbbtgatd_i_rv_slot: &mut f64,
        var_bdvbrgatd_i_slot: &mut f64,
        var_bdvbrgatd_i_rv_slot: &mut f64,
        var_cbbtbotd_i_slot: &mut f64,
        var_cbbtbotd_i_rv_slot: &mut f64,
        var_cbbtgatd_i_slot: &mut f64,
        var_cbbtgatd_i_rv_slot: &mut f64,
        var_cbbtstid_i_slot: &mut f64,
        var_cbbtstid_i_rv_slot: &mut f64,
        var_cjorgat2nd_d_slot: &mut f64,
        var_cjorgat2nd_d_rv_slot: &mut f64,
        var_csrhgatd_i_slot: &mut f64,
        var_csrhgatd_i_rv_slot: &mut f64,
        var_ctatbotd_i_slot: &mut f64,
        var_ctatbotd_i_rv_slot: &mut f64,
        var_ctatgatd_i_slot: &mut f64,
        var_ctatgatd_i_rv_slot: &mut f64,
        var_ctatstid_i_slot: &mut f64,
        var_ctatstid_i_rv_slot: &mut f64,
        var_fbbtrbotd_i_slot: &mut f64,
        var_fbbtrbotd_i_rv_slot: &mut f64,
        var_fbbtrgatd_i_slot: &mut f64,
        var_fbbtrgatd_i_rv_slot: &mut f64,
        var_fbbtrstid_i_slot: &mut f64,
        var_fbbtrstid_i_rv_slot: &mut f64,
        var_fcjorgat2d_i_slot: &mut f64,
        var_fcjorgat2d_i_rv_slot: &mut f64,
        var_fjunqd_i_slot: &mut f64,
        var_fjunqd_i_rv_slot: &mut f64,
        var_fpgat2d_i_slot: &mut f64,
        var_fpgat2d_i_rv_slot: &mut f64,
        var_fphiggat2d_i_slot: &mut f64,
        var_fphiggat2d_i_rv_slot: &mut f64,
        var_fvbirgat2d_i_slot: &mut f64,
        var_fvbirgat2d_i_rv_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard6_rv_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard7_rv_slot: &mut f64,
        var_mefftatbotd_i_slot: &mut f64,
        var_mefftatbotd_i_rv_slot: &mut f64,
        var_mefftatgatd_i_slot: &mut f64,
        var_mefftatgatd_i_rv_slot: &mut f64,
        var_mefftatstid_i_slot: &mut f64,
        var_mefftatstid_i_rv_slot: &mut f64,
        var_one_minus_pbot_d_slot: &mut f64,
        var_one_minus_pbot_d_rv_slot: &mut f64,
        var_one_minus_pgat_d_slot: &mut f64,
        var_one_minus_pgat_d_rv_slot: &mut f64,
        var_one_minus_psti_d_slot: &mut f64,
        var_one_minus_psti_d_rv_slot: &mut f64,
        var_one_over_one_minus_pbot_d_slot: &mut f64,
        var_one_over_one_minus_pbot_d_rv_slot: &mut f64,
        var_one_over_one_minus_pgat_d_slot: &mut f64,
        var_one_over_one_minus_pgat_d_rv_slot: &mut f64,
        var_one_over_one_minus_psti_d_slot: &mut f64,
        var_one_over_one_minus_psti_d_rv_slot: &mut f64,
        var_pbrbotd_i_slot: &mut f64,
        var_pbrbotd_i_rv_slot: &mut f64,
        var_pbrgatd_i_slot: &mut f64,
        var_pbrgatd_i_rv_slot: &mut f64,
        var_pbrstid_i_slot: &mut f64,
        var_pbrstid_i_rv_slot: &mut f64,
        var_phigrbot_d_slot: &mut f64,
        var_phigrbot_d_rv_slot: &mut f64,
        var_phigrgat_d_slot: &mut f64,
        var_phigrgat_d_rv_slot: &mut f64,
        var_phigrsti_d_slot: &mut f64,
        var_phigrsti_d_rv_slot: &mut f64,
        var_stfbbtbotd_i_slot: &mut f64,
        var_stfbbtbotd_i_rv_slot: &mut f64,
        var_stfbbtgatd_i_slot: &mut f64,
        var_stfbbtgatd_i_rv_slot: &mut f64,
        var_stfbbtstid_i_slot: &mut f64,
        var_stfbbtstid_i_rv_slot: &mut f64,
        var_swgat2nd_d_slot: &mut f64,
        var_swgat2nd_d_rv_slot: &mut f64,
        var_vbirbotinv_d_slot: &mut f64,
        var_vbirbotinv_d_rv_slot: &mut f64,
        var_vbirgat2nd_d_slot: &mut f64,
        var_vbirgat2nd_d_rv_slot: &mut f64,
        var_vbirgatinv_d_slot: &mut f64,
        var_vbirgatinv_d_rv_slot: &mut f64,
        var_vbirstiinv_d_slot: &mut f64,
        var_vbirstiinv_d_rv_slot: &mut f64,
        var_vbrbotd_i_slot: &mut f64,
        var_vbrbotd_i_rv_slot: &mut f64,
        var_vbrgatd_i_slot: &mut f64,
        var_vbrgatd_i_rv_slot: &mut f64,
        var_vbrinvbot_d_slot: &mut f64,
        var_vbrinvbot_d_rv_slot: &mut f64,
        var_vbrinvgat_d_slot: &mut f64,
        var_vbrinvgat_d_dn5_slot: &mut f64,
        var_vbrinvgat_d_dn6_slot: &mut f64,
        var_vbrinvgat_d_dn7_slot: &mut f64,
        var_vbrinvgat_d_dn8_slot: &mut f64,
        var_vbrinvgat_d_rv_slot: &mut f64,
        var_vbrinvsti_d_slot: &mut f64,
        var_vbrinvsti_d_rv_slot: &mut f64,
        var_vbrstid_i_slot: &mut f64,
        var_vbrstid_i_rv_slot: &mut f64,
        var_vtrgatd_i_slot: &mut f64,
        var_vtrgatd_i_rv_slot: &mut f64,
        var_wdepnulrbot_d_slot: &mut f64,
        var_wdepnulrbot_d_rv_slot: &mut f64,
        var_wdepnulrgat_d_slot: &mut f64,
        var_wdepnulrgat_d_rv_slot: &mut f64,
        var_wdepnulrinvbot_d_slot: &mut f64,
        var_wdepnulrinvbot_d_rv_slot: &mut f64,
        var_wdepnulrinvgat_d_slot: &mut f64,
        var_wdepnulrinvgat_d_rv_slot: &mut f64,
        var_wdepnulrinvsti_d_slot: &mut f64,
        var_wdepnulrinvsti_d_rv_slot: &mut f64,
        var_wdepnulrsti_d_slot: &mut f64,
        var_wdepnulrsti_d_rv_slot: &mut f64,
        var_xjungatd_i_slot: &mut f64,
        var_xjungatd_i_rv_slot: &mut f64,
        var_xjunstid_i_slot: &mut f64,
        var_xjunstid_i_rv_slot: &mut f64,
    ) {
        let mut var_adbbtgatd_i: f64 = *var_adbbtgatd_i_slot;
        let mut var_adbbtgatd_i_rv: f64 = *var_adbbtgatd_i_rv_slot;
        let mut var_advbrgatd_i: f64 = *var_advbrgatd_i_slot;
        let mut var_advbrgatd_i_rv: f64 = *var_advbrgatd_i_rv_slot;
        let mut var_anugatd_i: f64 = *var_anugatd_i_slot;
        let mut var_anugatd_i_rv: f64 = *var_anugatd_i_rv_slot;
        let mut var_bdbbtgatd_i: f64 = *var_bdbbtgatd_i_slot;
        let mut var_bdbbtgatd_i_rv: f64 = *var_bdbbtgatd_i_rv_slot;
        let mut var_bdvbrgatd_i: f64 = *var_bdvbrgatd_i_slot;
        let mut var_bdvbrgatd_i_rv: f64 = *var_bdvbrgatd_i_rv_slot;
        let mut var_cbbtbotd_i: f64 = *var_cbbtbotd_i_slot;
        let mut var_cbbtbotd_i_rv: f64 = *var_cbbtbotd_i_rv_slot;
        let mut var_cbbtgatd_i: f64 = *var_cbbtgatd_i_slot;
        let mut var_cbbtgatd_i_rv: f64 = *var_cbbtgatd_i_rv_slot;
        let mut var_cbbtstid_i: f64 = *var_cbbtstid_i_slot;
        let mut var_cbbtstid_i_rv: f64 = *var_cbbtstid_i_rv_slot;
        let mut var_cjorgat2nd_d: f64 = *var_cjorgat2nd_d_slot;
        let mut var_cjorgat2nd_d_rv: f64 = *var_cjorgat2nd_d_rv_slot;
        let mut var_csrhgatd_i: f64 = *var_csrhgatd_i_slot;
        let mut var_csrhgatd_i_rv: f64 = *var_csrhgatd_i_rv_slot;
        let mut var_ctatbotd_i: f64 = *var_ctatbotd_i_slot;
        let mut var_ctatbotd_i_rv: f64 = *var_ctatbotd_i_rv_slot;
        let mut var_ctatgatd_i: f64 = *var_ctatgatd_i_slot;
        let mut var_ctatgatd_i_rv: f64 = *var_ctatgatd_i_rv_slot;
        let mut var_ctatstid_i: f64 = *var_ctatstid_i_slot;
        let mut var_ctatstid_i_rv: f64 = *var_ctatstid_i_rv_slot;
        let mut var_fbbtrbotd_i: f64 = *var_fbbtrbotd_i_slot;
        let mut var_fbbtrbotd_i_rv: f64 = *var_fbbtrbotd_i_rv_slot;
        let mut var_fbbtrgatd_i: f64 = *var_fbbtrgatd_i_slot;
        let mut var_fbbtrgatd_i_rv: f64 = *var_fbbtrgatd_i_rv_slot;
        let mut var_fbbtrstid_i: f64 = *var_fbbtrstid_i_slot;
        let mut var_fbbtrstid_i_rv: f64 = *var_fbbtrstid_i_rv_slot;
        let mut var_fcjorgat2d_i: f64 = *var_fcjorgat2d_i_slot;
        let mut var_fcjorgat2d_i_rv: f64 = *var_fcjorgat2d_i_rv_slot;
        let mut var_fjunqd_i: f64 = *var_fjunqd_i_slot;
        let mut var_fjunqd_i_rv: f64 = *var_fjunqd_i_rv_slot;
        let mut var_fpgat2d_i: f64 = *var_fpgat2d_i_slot;
        let mut var_fpgat2d_i_rv: f64 = *var_fpgat2d_i_rv_slot;
        let mut var_fphiggat2d_i: f64 = *var_fphiggat2d_i_slot;
        let mut var_fphiggat2d_i_rv: f64 = *var_fphiggat2d_i_rv_slot;
        let mut var_fvbirgat2d_i: f64 = *var_fvbirgat2d_i_slot;
        let mut var_fvbirgat2d_i_rv: f64 = *var_fvbirgat2d_i_rv_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard6_rv: f64 = *var_guard6_rv_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard7_rv: f64 = *var_guard7_rv_slot;
        let mut var_mefftatbotd_i: f64 = *var_mefftatbotd_i_slot;
        let mut var_mefftatbotd_i_rv: f64 = *var_mefftatbotd_i_rv_slot;
        let mut var_mefftatgatd_i: f64 = *var_mefftatgatd_i_slot;
        let mut var_mefftatgatd_i_rv: f64 = *var_mefftatgatd_i_rv_slot;
        let mut var_mefftatstid_i: f64 = *var_mefftatstid_i_slot;
        let mut var_mefftatstid_i_rv: f64 = *var_mefftatstid_i_rv_slot;
        let mut var_one_minus_pbot_d: f64 = *var_one_minus_pbot_d_slot;
        let mut var_one_minus_pbot_d_rv: f64 = *var_one_minus_pbot_d_rv_slot;
        let mut var_one_minus_pgat_d: f64 = *var_one_minus_pgat_d_slot;
        let mut var_one_minus_pgat_d_rv: f64 = *var_one_minus_pgat_d_rv_slot;
        let mut var_one_minus_psti_d: f64 = *var_one_minus_psti_d_slot;
        let mut var_one_minus_psti_d_rv: f64 = *var_one_minus_psti_d_rv_slot;
        let mut var_one_over_one_minus_pbot_d: f64 = *var_one_over_one_minus_pbot_d_slot;
        let mut var_one_over_one_minus_pbot_d_rv: f64 = *var_one_over_one_minus_pbot_d_rv_slot;
        let mut var_one_over_one_minus_pgat_d: f64 = *var_one_over_one_minus_pgat_d_slot;
        let mut var_one_over_one_minus_pgat_d_rv: f64 = *var_one_over_one_minus_pgat_d_rv_slot;
        let mut var_one_over_one_minus_psti_d: f64 = *var_one_over_one_minus_psti_d_slot;
        let mut var_one_over_one_minus_psti_d_rv: f64 = *var_one_over_one_minus_psti_d_rv_slot;
        let mut var_pbrbotd_i: f64 = *var_pbrbotd_i_slot;
        let mut var_pbrbotd_i_rv: f64 = *var_pbrbotd_i_rv_slot;
        let mut var_pbrgatd_i: f64 = *var_pbrgatd_i_slot;
        let mut var_pbrgatd_i_rv: f64 = *var_pbrgatd_i_rv_slot;
        let mut var_pbrstid_i: f64 = *var_pbrstid_i_slot;
        let mut var_pbrstid_i_rv: f64 = *var_pbrstid_i_rv_slot;
        let mut var_phigrbot_d: f64 = *var_phigrbot_d_slot;
        let mut var_phigrbot_d_rv: f64 = *var_phigrbot_d_rv_slot;
        let mut var_phigrgat_d: f64 = *var_phigrgat_d_slot;
        let mut var_phigrgat_d_rv: f64 = *var_phigrgat_d_rv_slot;
        let mut var_phigrsti_d: f64 = *var_phigrsti_d_slot;
        let mut var_phigrsti_d_rv: f64 = *var_phigrsti_d_rv_slot;
        let mut var_stfbbtbotd_i: f64 = *var_stfbbtbotd_i_slot;
        let mut var_stfbbtbotd_i_rv: f64 = *var_stfbbtbotd_i_rv_slot;
        let mut var_stfbbtgatd_i: f64 = *var_stfbbtgatd_i_slot;
        let mut var_stfbbtgatd_i_rv: f64 = *var_stfbbtgatd_i_rv_slot;
        let mut var_stfbbtstid_i: f64 = *var_stfbbtstid_i_slot;
        let mut var_stfbbtstid_i_rv: f64 = *var_stfbbtstid_i_rv_slot;
        let mut var_swgat2nd_d: f64 = *var_swgat2nd_d_slot;
        let mut var_swgat2nd_d_rv: f64 = *var_swgat2nd_d_rv_slot;
        let mut var_vbirbotinv_d: f64 = *var_vbirbotinv_d_slot;
        let mut var_vbirbotinv_d_rv: f64 = *var_vbirbotinv_d_rv_slot;
        let mut var_vbirgat2nd_d: f64 = *var_vbirgat2nd_d_slot;
        let mut var_vbirgat2nd_d_rv: f64 = *var_vbirgat2nd_d_rv_slot;
        let mut var_vbirgatinv_d: f64 = *var_vbirgatinv_d_slot;
        let mut var_vbirgatinv_d_rv: f64 = *var_vbirgatinv_d_rv_slot;
        let mut var_vbirstiinv_d: f64 = *var_vbirstiinv_d_slot;
        let mut var_vbirstiinv_d_rv: f64 = *var_vbirstiinv_d_rv_slot;
        let mut var_vbrbotd_i: f64 = *var_vbrbotd_i_slot;
        let mut var_vbrbotd_i_rv: f64 = *var_vbrbotd_i_rv_slot;
        let mut var_vbrgatd_i: f64 = *var_vbrgatd_i_slot;
        let mut var_vbrgatd_i_rv: f64 = *var_vbrgatd_i_rv_slot;
        let mut var_vbrinvbot_d: f64 = *var_vbrinvbot_d_slot;
        let mut var_vbrinvbot_d_rv: f64 = *var_vbrinvbot_d_rv_slot;
        let mut var_vbrinvgat_d: f64 = *var_vbrinvgat_d_slot;
        let mut var_vbrinvgat_d_dn5: f64 = *var_vbrinvgat_d_dn5_slot;
        let mut var_vbrinvgat_d_dn6: f64 = *var_vbrinvgat_d_dn6_slot;
        let mut var_vbrinvgat_d_dn7: f64 = *var_vbrinvgat_d_dn7_slot;
        let mut var_vbrinvgat_d_dn8: f64 = *var_vbrinvgat_d_dn8_slot;
        let mut var_vbrinvgat_d_rv: f64 = *var_vbrinvgat_d_rv_slot;
        let mut var_vbrinvsti_d: f64 = *var_vbrinvsti_d_slot;
        let mut var_vbrinvsti_d_rv: f64 = *var_vbrinvsti_d_rv_slot;
        let mut var_vbrstid_i: f64 = *var_vbrstid_i_slot;
        let mut var_vbrstid_i_rv: f64 = *var_vbrstid_i_rv_slot;
        let mut var_vtrgatd_i: f64 = *var_vtrgatd_i_slot;
        let mut var_vtrgatd_i_rv: f64 = *var_vtrgatd_i_rv_slot;
        let mut var_wdepnulrbot_d: f64 = *var_wdepnulrbot_d_slot;
        let mut var_wdepnulrbot_d_rv: f64 = *var_wdepnulrbot_d_rv_slot;
        let mut var_wdepnulrgat_d: f64 = *var_wdepnulrgat_d_slot;
        let mut var_wdepnulrgat_d_rv: f64 = *var_wdepnulrgat_d_rv_slot;
        let mut var_wdepnulrinvbot_d: f64 = *var_wdepnulrinvbot_d_slot;
        let mut var_wdepnulrinvbot_d_rv: f64 = *var_wdepnulrinvbot_d_rv_slot;
        let mut var_wdepnulrinvgat_d: f64 = *var_wdepnulrinvgat_d_slot;
        let mut var_wdepnulrinvgat_d_rv: f64 = *var_wdepnulrinvgat_d_rv_slot;
        let mut var_wdepnulrinvsti_d: f64 = *var_wdepnulrinvsti_d_slot;
        let mut var_wdepnulrinvsti_d_rv: f64 = *var_wdepnulrinvsti_d_rv_slot;
        let mut var_wdepnulrsti_d: f64 = *var_wdepnulrsti_d_slot;
        let mut var_wdepnulrsti_d_rv: f64 = *var_wdepnulrsti_d_rv_slot;
        let mut var_xjungatd_i: f64 = *var_xjungatd_i_slot;
        let mut var_xjungatd_i_rv: f64 = *var_xjungatd_i_rv_slot;
        let mut var_xjunstid_i: f64 = *var_xjunstid_i_slot;
        let mut var_xjunstid_i_rv: f64 = *var_xjunstid_i_rv_slot;

        let (assign1270_e2062,) = {
    if (var_guard5 == 0.0) {
        (p.p886,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign1270_e2062;
        var_csrhgatd_i_rv = 0.0;

        let (assign1280_e2067,) = {
    if (var_guard5 == 0.0) {
        (p.p887,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign1280_e2067;
        var_xjunstid_i_rv = 0.0;

        let (assign1290_e2072,) = {
    if (var_guard5 == 0.0) {
        (p.p888,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign1290_e2072;
        var_xjungatd_i_rv = 0.0;

        let (assign1300_e2077,) = {
    if (var_guard5 == 0.0) {
        (p.p889,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign1300_e2077;
        var_ctatbotd_i_rv = 0.0;

        let (assign1310_e2082,) = {
    if (var_guard5 == 0.0) {
        (p.p890,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign1310_e2082;
        var_ctatstid_i_rv = 0.0;

        let (assign1320_e2087,) = {
    if (var_guard5 == 0.0) {
        (p.p891,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign1320_e2087;
        var_ctatgatd_i_rv = 0.0;

        let (assign1330_e2092,) = {
    if (var_guard5 == 0.0) {
        (p.p892,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign1330_e2092;
        var_mefftatbotd_i_rv = 0.0;

        let (assign1340_e2097,) = {
    if (var_guard5 == 0.0) {
        (p.p893,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign1340_e2097;
        var_mefftatstid_i_rv = 0.0;

        let (assign1350_e2102,) = {
    if (var_guard5 == 0.0) {
        (p.p894,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign1350_e2102;
        var_mefftatgatd_i_rv = 0.0;

        let (assign1360_e2107,) = {
    if (var_guard5 == 0.0) {
        (p.p895,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign1360_e2107;
        var_cbbtbotd_i_rv = 0.0;

        let (assign1370_e2112,) = {
    if (var_guard5 == 0.0) {
        (p.p896,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign1370_e2112;
        var_cbbtstid_i_rv = 0.0;

        let (assign1380_e2117,) = {
    if (var_guard5 == 0.0) {
        (p.p897,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign1380_e2117;
        var_cbbtgatd_i_rv = 0.0;

        let (assign1390_e2122,) = {
    if (var_guard5 == 0.0) {
        (p.p898,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign1390_e2122;
        var_fbbtrbotd_i_rv = 0.0;

        let (assign1400_e2127,) = {
    if (var_guard5 == 0.0) {
        (p.p899,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign1400_e2127;
        var_fbbtrstid_i_rv = 0.0;

        let (assign1410_e2132,) = {
    if (var_guard5 == 0.0) {
        (p.p900,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign1410_e2132;
        var_fbbtrgatd_i_rv = 0.0;

        let (assign1420_e2137,) = {
    if (var_guard5 == 0.0) {
        (p.p901,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign1420_e2137;
        var_stfbbtbotd_i_rv = 0.0;

        let (assign1430_e2142,) = {
    if (var_guard5 == 0.0) {
        (p.p902,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign1430_e2142;
        var_stfbbtstid_i_rv = 0.0;

        let (assign1440_e2147,) = {
    if (var_guard5 == 0.0) {
        (p.p903,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign1440_e2147;
        var_stfbbtgatd_i_rv = 0.0;

        let (assign1450_e2152,) = {
    if (var_guard5 == 0.0) {
        (p.p904,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign1450_e2152;
        var_vbrbotd_i_rv = 0.0;

        let (assign1460_e2157,) = {
    if (var_guard5 == 0.0) {
        (p.p905,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign1460_e2157;
        var_vbrstid_i_rv = 0.0;

        let (assign1470_e2162,) = {
    if (var_guard5 == 0.0) {
        (p.p906,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign1470_e2162;
        var_vbrgatd_i_rv = 0.0;

        let (assign1480_e2167,) = {
    if (var_guard5 == 0.0) {
        (p.p907,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign1480_e2167;
        var_pbrbotd_i_rv = 0.0;

        let (assign1490_e2172,) = {
    if (var_guard5 == 0.0) {
        (p.p908,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign1490_e2172;
        var_pbrstid_i_rv = 0.0;

        let (assign1500_e2177,) = {
    if (var_guard5 == 0.0) {
        (p.p909,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign1500_e2177;
        var_pbrgatd_i_rv = 0.0;

        let (assign1520_e2187,) = {
    if (var_guard5 == 0.0) {
        (p.p924,)
    } else {
        (var_fjunqd_i,)
    }
};
        var_fjunqd_i = assign1520_e2187;
        var_fjunqd_i_rv = 0.0;

        let (assign1530_e2192,) = {
    if (var_guard5 == 0.0) {
        (p.p916,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1530_e2192;
        var_advbrgatd_i_rv = 0.0;

        let (assign1540_e2197,) = {
    if (var_guard5 == 0.0) {
        (p.p917,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1540_e2197;
        var_bdvbrgatd_i_rv = 0.0;

        let (assign1550_e2202,) = {
    if (var_guard5 == 0.0) {
        (p.p918,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1550_e2202;
        var_adbbtgatd_i_rv = 0.0;

        let (assign1560_e2207,) = {
    if (var_guard5 == 0.0) {
        (p.p919,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1560_e2207;
        var_bdbbtgatd_i_rv = 0.0;

        let (assign1570_e2212,) = {
    if (var_guard5 == 0.0) {
        (p.p910,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1570_e2212;
        var_fcjorgat2d_i_rv = 0.0;

        let (assign1580_e2217,) = {
    if (var_guard5 == 0.0) {
        (p.p911,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1580_e2217;
        var_fvbirgat2d_i_rv = 0.0;

        let (assign1590_e2222,) = {
    if (var_guard5 == 0.0) {
        (p.p912,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1590_e2222;
        var_fpgat2d_i_rv = 0.0;

        let (assign1600_e2227,) = {
    if (var_guard5 == 0.0) {
        (p.p913,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1600_e2227;
        var_fphiggat2d_i_rv = 0.0;

        let (assign1610_e2232,) = {
    if (var_guard5 == 0.0) {
        (p.p914,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1610_e2232;
        var_vtrgatd_i_rv = 0.0;

        let (assign1620_e2237,) = {
    if (var_guard5 == 0.0) {
        (p.p915,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1620_e2237;
        var_anugatd_i_rv = 0.0;

        let assign1630_e2240: f64 = (var_phigbotd_i + var_deltaphigr);
        var_phigrbot_d = assign1630_e2240;
        var_phigrbot_d_rv = 0.0;

        let assign1640_e2243: f64 = (var_phigstid_i + var_deltaphigr);
        var_phigrsti_d = assign1640_e2243;
        var_phigrsti_d_rv = 0.0;

        let assign1650_e2246: f64 = (var_phiggatd_i + var_deltaphigr);
        var_phigrgat_d = assign1650_e2246;
        var_phigrgat_d_rv = 0.0;

        let assign1660_e2249: f64 = (1.0 - var_pbotd_i);
        var_one_minus_pbot_d = assign1660_e2249;
        var_one_minus_pbot_d_rv = 0.0;

        let assign1670_e2252: f64 = (1.0 - var_pstid_i);
        var_one_minus_psti_d = assign1670_e2252;
        var_one_minus_psti_d_rv = 0.0;

        let assign1680_e2255: f64 = (1.0 - var_pgatd_i);
        var_one_minus_pgat_d = assign1680_e2255;
        var_one_minus_pgat_d_rv = 0.0;

        let assign1690_e2258: f64 = (1.0 / var_one_minus_pbot_d);
        var_one_over_one_minus_pbot_d = assign1690_e2258;
        var_one_over_one_minus_pbot_d_rv = 0.0;

        let assign1700_e2261: f64 = (1.0 / var_one_minus_psti_d);
        var_one_over_one_minus_psti_d = assign1700_e2261;
        var_one_over_one_minus_psti_d_rv = 0.0;

        let assign1710_e2264: f64 = (1.0 / var_one_minus_pgat_d);
        var_one_over_one_minus_pgat_d = assign1710_e2264;
        var_one_over_one_minus_pgat_d_rv = 0.0;

        let assign1720_e2267: f64 = (var_epssi / var_cjorbotd_i);
        var_wdepnulrbot_d = assign1720_e2267;
        var_wdepnulrbot_d_rv = 0.0;

        let assign1730_e2270: f64 = (var_xjunstid_i * var_epssi);
        let assign1730_e2272: f64 = (assign1730_e2270 / var_cjorstid_i);
        var_wdepnulrsti_d = assign1730_e2272;
        var_wdepnulrsti_d_rv = 0.0;

        let assign1740_e2275: f64 = (var_xjungatd_i * var_epssi);
        let assign1740_e2277: f64 = (assign1740_e2275 / var_cjorgatd_i);
        var_wdepnulrgat_d = assign1740_e2277;
        var_wdepnulrgat_d_rv = 0.0;

        let assign1750_e2280: f64 = (1.0 / var_wdepnulrbot_d);
        var_wdepnulrinvbot_d = assign1750_e2280;
        var_wdepnulrinvbot_d_rv = 0.0;

        let assign1760_e2283: f64 = (1.0 / var_wdepnulrsti_d);
        var_wdepnulrinvsti_d = assign1760_e2283;
        var_wdepnulrinvsti_d_rv = 0.0;

        let assign1770_e2286: f64 = (1.0 / var_wdepnulrgat_d);
        var_wdepnulrinvgat_d = assign1770_e2286;
        var_wdepnulrinvgat_d_rv = 0.0;

        let assign1780_e2289: f64 = (1.0 / var_vbirbotd_i);
        var_vbirbotinv_d = assign1780_e2289;
        var_vbirbotinv_d_rv = 0.0;

        let assign1790_e2292: f64 = (1.0 / var_vbirstid_i);
        var_vbirstiinv_d = assign1790_e2292;
        var_vbirstiinv_d_rv = 0.0;

        let assign1800_e2295: f64 = (1.0 / var_vbirgatd_i);
        var_vbirgatinv_d = assign1800_e2295;
        var_vbirgatinv_d_rv = 0.0;

        let assign1840_e2319: f64 = (1.0 / var_vbrbotd_i);
        var_vbrinvbot_d = assign1840_e2319;
        var_vbrinvbot_d_rv = 0.0;

        let assign1850_e2322: f64 = (1.0 / var_vbrstid_i);
        var_vbrinvsti_d = assign1850_e2322;
        var_vbrinvsti_d_rv = 0.0;

        let assign1860_e2325: f64 = (1.0 / var_vbrgatd_i);
        var_vbrinvgat_d = assign1860_e2325;
        var_vbrinvgat_d_dn5 = 0.0;
        var_vbrinvgat_d_dn6 = 0.0;
        var_vbrinvgat_d_dn7 = 0.0;
        var_vbrinvgat_d_dn8 = 0.0;
        var_vbrinvgat_d_rv = 0.0;

        let assign1900_e2382: f64 = if ((((var_fcjorgat2d_i != 1.0) || (var_fvbirgat2d_i != 1.0)) || (var_fpgat2d_i != 1.0)) || (var_fphiggat2d_i != 1.0)) { 1.0 } else { 0.0 };
        var_guard6 = assign1900_e2382;
        var_guard6_rv = 0.0;

        let (assign1910_e2386,) = {
    if (var_guard6 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign1910_e2386;
        var_swgat2nd_d_rv = 0.0;

        let (assign1920_e2391,) = {
    if (var_guard6 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign1920_e2391;
        var_swgat2nd_d_rv = 0.0;

        let assign1930_e2394: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard7 = assign1930_e2394;
        var_guard7_rv = 0.0;

        let (assign1940_e2407,) = {
    if (var_guard7 != 0.0) {
        let assign1940_e2398: f64 = (var_cjorgatd_i * var_fcjorgat2d_i);
        let (assign1940_e2405,) = {
            if (assign1940_e2398 > 1e-18) {
                let assign1940_e2403: f64 = (var_cjorgatd_i * var_fcjorgat2d_i);
                (assign1940_e2403,)
            } else {
                (1e-18,)
            }
        };
        (assign1940_e2405,)
    } else {
        (var_cjorgat2nd_d,)
    }
};
        var_cjorgat2nd_d = assign1940_e2407;
        var_cjorgat2nd_d_rv = 0.0;

        let (assign1950_e2420,) = {
    if (var_guard7 != 0.0) {
        let assign1950_e2411: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
        let (assign1950_e2418,) = {
            if (assign1950_e2411 > 0.05) {
                let assign1950_e2416: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
                (assign1950_e2416,)
            } else {
                (0.05,)
            }
        };
        (assign1950_e2418,)
    } else {
        (var_vbirgat2nd_d,)
    }
};
        var_vbirgat2nd_d = assign1950_e2420;
        var_vbirgat2nd_d_rv = 0.0;

        *var_adbbtgatd_i_slot = var_adbbtgatd_i;
        *var_adbbtgatd_i_rv_slot = var_adbbtgatd_i_rv;
        *var_advbrgatd_i_slot = var_advbrgatd_i;
        *var_advbrgatd_i_rv_slot = var_advbrgatd_i_rv;
        *var_anugatd_i_slot = var_anugatd_i;
        *var_anugatd_i_rv_slot = var_anugatd_i_rv;
        *var_bdbbtgatd_i_slot = var_bdbbtgatd_i;
        *var_bdbbtgatd_i_rv_slot = var_bdbbtgatd_i_rv;
        *var_bdvbrgatd_i_slot = var_bdvbrgatd_i;
        *var_bdvbrgatd_i_rv_slot = var_bdvbrgatd_i_rv;
        *var_cbbtbotd_i_slot = var_cbbtbotd_i;
        *var_cbbtbotd_i_rv_slot = var_cbbtbotd_i_rv;
        *var_cbbtgatd_i_slot = var_cbbtgatd_i;
        *var_cbbtgatd_i_rv_slot = var_cbbtgatd_i_rv;
        *var_cbbtstid_i_slot = var_cbbtstid_i;
        *var_cbbtstid_i_rv_slot = var_cbbtstid_i_rv;
        *var_cjorgat2nd_d_slot = var_cjorgat2nd_d;
        *var_cjorgat2nd_d_rv_slot = var_cjorgat2nd_d_rv;
        *var_csrhgatd_i_slot = var_csrhgatd_i;
        *var_csrhgatd_i_rv_slot = var_csrhgatd_i_rv;
        *var_ctatbotd_i_slot = var_ctatbotd_i;
        *var_ctatbotd_i_rv_slot = var_ctatbotd_i_rv;
        *var_ctatgatd_i_slot = var_ctatgatd_i;
        *var_ctatgatd_i_rv_slot = var_ctatgatd_i_rv;
        *var_ctatstid_i_slot = var_ctatstid_i;
        *var_ctatstid_i_rv_slot = var_ctatstid_i_rv;
        *var_fbbtrbotd_i_slot = var_fbbtrbotd_i;
        *var_fbbtrbotd_i_rv_slot = var_fbbtrbotd_i_rv;
        *var_fbbtrgatd_i_slot = var_fbbtrgatd_i;
        *var_fbbtrgatd_i_rv_slot = var_fbbtrgatd_i_rv;
        *var_fbbtrstid_i_slot = var_fbbtrstid_i;
        *var_fbbtrstid_i_rv_slot = var_fbbtrstid_i_rv;
        *var_fcjorgat2d_i_slot = var_fcjorgat2d_i;
        *var_fcjorgat2d_i_rv_slot = var_fcjorgat2d_i_rv;
        *var_fjunqd_i_slot = var_fjunqd_i;
        *var_fjunqd_i_rv_slot = var_fjunqd_i_rv;
        *var_fpgat2d_i_slot = var_fpgat2d_i;
        *var_fpgat2d_i_rv_slot = var_fpgat2d_i_rv;
        *var_fphiggat2d_i_slot = var_fphiggat2d_i;
        *var_fphiggat2d_i_rv_slot = var_fphiggat2d_i_rv;
        *var_fvbirgat2d_i_slot = var_fvbirgat2d_i;
        *var_fvbirgat2d_i_rv_slot = var_fvbirgat2d_i_rv;
        *var_guard6_slot = var_guard6;
        *var_guard6_rv_slot = var_guard6_rv;
        *var_guard7_slot = var_guard7;
        *var_guard7_rv_slot = var_guard7_rv;
        *var_mefftatbotd_i_slot = var_mefftatbotd_i;
        *var_mefftatbotd_i_rv_slot = var_mefftatbotd_i_rv;
        *var_mefftatgatd_i_slot = var_mefftatgatd_i;
        *var_mefftatgatd_i_rv_slot = var_mefftatgatd_i_rv;
        *var_mefftatstid_i_slot = var_mefftatstid_i;
        *var_mefftatstid_i_rv_slot = var_mefftatstid_i_rv;
        *var_one_minus_pbot_d_slot = var_one_minus_pbot_d;
        *var_one_minus_pbot_d_rv_slot = var_one_minus_pbot_d_rv;
        *var_one_minus_pgat_d_slot = var_one_minus_pgat_d;
        *var_one_minus_pgat_d_rv_slot = var_one_minus_pgat_d_rv;
        *var_one_minus_psti_d_slot = var_one_minus_psti_d;
        *var_one_minus_psti_d_rv_slot = var_one_minus_psti_d_rv;
        *var_one_over_one_minus_pbot_d_slot = var_one_over_one_minus_pbot_d;
        *var_one_over_one_minus_pbot_d_rv_slot = var_one_over_one_minus_pbot_d_rv;
        *var_one_over_one_minus_pgat_d_slot = var_one_over_one_minus_pgat_d;
        *var_one_over_one_minus_pgat_d_rv_slot = var_one_over_one_minus_pgat_d_rv;
        *var_one_over_one_minus_psti_d_slot = var_one_over_one_minus_psti_d;
        *var_one_over_one_minus_psti_d_rv_slot = var_one_over_one_minus_psti_d_rv;
        *var_pbrbotd_i_slot = var_pbrbotd_i;
        *var_pbrbotd_i_rv_slot = var_pbrbotd_i_rv;
        *var_pbrgatd_i_slot = var_pbrgatd_i;
        *var_pbrgatd_i_rv_slot = var_pbrgatd_i_rv;
        *var_pbrstid_i_slot = var_pbrstid_i;
        *var_pbrstid_i_rv_slot = var_pbrstid_i_rv;
        *var_phigrbot_d_slot = var_phigrbot_d;
        *var_phigrbot_d_rv_slot = var_phigrbot_d_rv;
        *var_phigrgat_d_slot = var_phigrgat_d;
        *var_phigrgat_d_rv_slot = var_phigrgat_d_rv;
        *var_phigrsti_d_slot = var_phigrsti_d;
        *var_phigrsti_d_rv_slot = var_phigrsti_d_rv;
        *var_stfbbtbotd_i_slot = var_stfbbtbotd_i;
        *var_stfbbtbotd_i_rv_slot = var_stfbbtbotd_i_rv;
        *var_stfbbtgatd_i_slot = var_stfbbtgatd_i;
        *var_stfbbtgatd_i_rv_slot = var_stfbbtgatd_i_rv;
        *var_stfbbtstid_i_slot = var_stfbbtstid_i;
        *var_stfbbtstid_i_rv_slot = var_stfbbtstid_i_rv;
        *var_swgat2nd_d_slot = var_swgat2nd_d;
        *var_swgat2nd_d_rv_slot = var_swgat2nd_d_rv;
        *var_vbirbotinv_d_slot = var_vbirbotinv_d;
        *var_vbirbotinv_d_rv_slot = var_vbirbotinv_d_rv;
        *var_vbirgat2nd_d_slot = var_vbirgat2nd_d;
        *var_vbirgat2nd_d_rv_slot = var_vbirgat2nd_d_rv;
        *var_vbirgatinv_d_slot = var_vbirgatinv_d;
        *var_vbirgatinv_d_rv_slot = var_vbirgatinv_d_rv;
        *var_vbirstiinv_d_slot = var_vbirstiinv_d;
        *var_vbirstiinv_d_rv_slot = var_vbirstiinv_d_rv;
        *var_vbrbotd_i_slot = var_vbrbotd_i;
        *var_vbrbotd_i_rv_slot = var_vbrbotd_i_rv;
        *var_vbrgatd_i_slot = var_vbrgatd_i;
        *var_vbrgatd_i_rv_slot = var_vbrgatd_i_rv;
        *var_vbrinvbot_d_slot = var_vbrinvbot_d;
        *var_vbrinvbot_d_rv_slot = var_vbrinvbot_d_rv;
        *var_vbrinvgat_d_slot = var_vbrinvgat_d;
        *var_vbrinvgat_d_dn5_slot = var_vbrinvgat_d_dn5;
        *var_vbrinvgat_d_dn6_slot = var_vbrinvgat_d_dn6;
        *var_vbrinvgat_d_dn7_slot = var_vbrinvgat_d_dn7;
        *var_vbrinvgat_d_dn8_slot = var_vbrinvgat_d_dn8;
        *var_vbrinvgat_d_rv_slot = var_vbrinvgat_d_rv;
        *var_vbrinvsti_d_slot = var_vbrinvsti_d;
        *var_vbrinvsti_d_rv_slot = var_vbrinvsti_d_rv;
        *var_vbrstid_i_slot = var_vbrstid_i;
        *var_vbrstid_i_rv_slot = var_vbrstid_i_rv;
        *var_vtrgatd_i_slot = var_vtrgatd_i;
        *var_vtrgatd_i_rv_slot = var_vtrgatd_i_rv;
        *var_wdepnulrbot_d_slot = var_wdepnulrbot_d;
        *var_wdepnulrbot_d_rv_slot = var_wdepnulrbot_d_rv;
        *var_wdepnulrgat_d_slot = var_wdepnulrgat_d;
        *var_wdepnulrgat_d_rv_slot = var_wdepnulrgat_d_rv;
        *var_wdepnulrinvbot_d_slot = var_wdepnulrinvbot_d;
        *var_wdepnulrinvbot_d_rv_slot = var_wdepnulrinvbot_d_rv;
        *var_wdepnulrinvgat_d_slot = var_wdepnulrinvgat_d;
        *var_wdepnulrinvgat_d_rv_slot = var_wdepnulrinvgat_d_rv;
        *var_wdepnulrinvsti_d_slot = var_wdepnulrinvsti_d;
        *var_wdepnulrinvsti_d_rv_slot = var_wdepnulrinvsti_d_rv;
        *var_wdepnulrsti_d_slot = var_wdepnulrsti_d;
        *var_wdepnulrsti_d_rv_slot = var_wdepnulrsti_d_rv;
        *var_xjungatd_i_slot = var_xjungatd_i;
        *var_xjungatd_i_rv_slot = var_xjungatd_i_rv;
        *var_xjunstid_i_slot = var_xjunstid_i;
        *var_xjunstid_i_rv_slot = var_xjunstid_i_rv;
    }

    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        var_deltaphigr: f64,
        var_fpgat2d_i: f64,
        var_fphiggat2d_i: f64,
        var_guard7: f64,
        var_kbol_over_qele: f64,
        var_one_over_one_minus_pbot: f64,
        var_one_over_one_minus_pgat: f64,
        var_one_over_one_minus_psti: f64,
        var_pgatd_i: f64,
        var_phiggat2nd: f64,
        var_phiggatd_i: f64,
        var_phigrbot: f64,
        var_phigrgat: f64,
        var_phigrgat2nd: f64,
        var_phigrsti: f64,
        var_phitrinv: f64,
        var_swgat2nd: f64,
        var_tkr: f64,
        var_tkr_1: f64,
        var_atatbot_slot: &mut f64,
        var_atatbot_rv_slot: &mut f64,
        var_atatgat_slot: &mut f64,
        var_atatgat_rv_slot: &mut f64,
        var_atatsti_slot: &mut f64,
        var_atatsti_rv_slot: &mut f64,
        var_auxt_slot: &mut f64,
        var_auxt_rv_slot: &mut f64,
        var_btatpartbot_slot: &mut f64,
        var_btatpartbot_rv_slot: &mut f64,
        var_btatpartgat_slot: &mut f64,
        var_btatpartgat_rv_slot: &mut f64,
        var_btatpartsti_slot: &mut f64,
        var_btatpartsti_rv_slot: &mut f64,
        var_cjobot_slot: &mut f64,
        var_cjobot_rv_slot: &mut f64,
        var_cjogat_slot: &mut f64,
        var_cjogat_rv_slot: &mut f64,
        var_cjosti_slot: &mut f64,
        var_cjosti_rv_slot: &mut f64,
        var_delt_slot: &mut f64,
        var_delt_rv_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_rv_slot: &mut f64,
        var_deltaebot_slot: &mut f64,
        var_deltaebot_rv_slot: &mut f64,
        var_deltaegat_slot: &mut f64,
        var_deltaegat_rv_slot: &mut f64,
        var_deltaesti_slot: &mut f64,
        var_deltaesti_rv_slot: &mut f64,
        var_deltaphigd_slot: &mut f64,
        var_deltaphigd_rv_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_rv_slot: &mut f64,
        var_fbbtbot_slot: &mut f64,
        var_fbbtbot_rv_slot: &mut f64,
        var_fbbtgat_slot: &mut f64,
        var_fbbtgat_dn5_slot: &mut f64,
        var_fbbtgat_dn6_slot: &mut f64,
        var_fbbtgat_dn7_slot: &mut f64,
        var_fbbtgat_dn8_slot: &mut f64,
        var_fbbtgat_rv_slot: &mut f64,
        var_fbbtsti_slot: &mut f64,
        var_fbbtsti_rv_slot: &mut f64,
        var_ftdbot_slot: &mut f64,
        var_ftdbot_rv_slot: &mut f64,
        var_ftdgat_slot: &mut f64,
        var_ftdgat2nd_slot: &mut f64,
        var_ftdgat2nd_rv_slot: &mut f64,
        var_ftdgat_rv_slot: &mut f64,
        var_ftdsti_slot: &mut f64,
        var_ftdsti_rv_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_guard27_rv_slot: &mut f64,
        var_idsatbot_slot: &mut f64,
        var_idsatbot_rv_slot: &mut f64,
        var_idsatgat_slot: &mut f64,
        var_idsatgat_rv_slot: &mut f64,
        var_idsatsti_slot: &mut f64,
        var_idsatsti_rv_slot: &mut f64,
        var_inv_phit_slot: &mut f64,
        var_inv_phit_rv_slot: &mut f64,
        var_inv_phita_slot: &mut f64,
        var_inv_phita_rv_slot: &mut f64,
        var_ln_rtn_slot: &mut f64,
        var_ln_rtn_rv_slot: &mut f64,
        var_one_minus_pgat2nd_d_slot: &mut f64,
        var_one_minus_pgat2nd_d_rv_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_d_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_d_rv_slot: &mut f64,
        var_pgat2nd_d_slot: &mut f64,
        var_pgat2nd_d_rv_slot: &mut f64,
        var_phibfac_slot: &mut f64,
        var_phibfac_rv_slot: &mut f64,
        var_phigdbot_slot: &mut f64,
        var_phigdbot_rv_slot: &mut f64,
        var_phigdgat_slot: &mut f64,
        var_phigdgat2nd_slot: &mut f64,
        var_phigdgat2nd_rv_slot: &mut f64,
        var_phigdgat_rv_slot: &mut f64,
        var_phigdsti_slot: &mut f64,
        var_phigdsti_rv_slot: &mut f64,
        var_phiggat2nd_d_slot: &mut f64,
        var_phiggat2nd_d_rv_slot: &mut f64,
        var_phigrgat2nd_d_slot: &mut f64,
        var_phigrgat2nd_d_rv_slot: &mut f64,
        var_phit_slot: &mut f64,
        var_phit_rv_slot: &mut f64,
        var_phita_slot: &mut f64,
        var_phita_rv_slot: &mut f64,
        var_phitd_slot: &mut f64,
        var_phitd_rv_slot: &mut f64,
        var_phitdinv_slot: &mut f64,
        var_phitdinv_rv_slot: &mut f64,
        var_qpref2bot_slot: &mut f64,
        var_qpref2bot_rv_slot: &mut f64,
        var_qpref2gat_slot: &mut f64,
        var_qpref2gat_rv_slot: &mut f64,
        var_qpref2sti_slot: &mut f64,
        var_qpref2sti_rv_slot: &mut f64,
        var_qprefbot_slot: &mut f64,
        var_qprefbot_rv_slot: &mut f64,
        var_qprefgat_slot: &mut f64,
        var_qprefgat_rv_slot: &mut f64,
        var_qprefsti_slot: &mut f64,
        var_qprefsti_rv_slot: &mut f64,
        var_rta_slot: &mut f64,
        var_rta_rv_slot: &mut f64,
        var_rtn_slot: &mut f64,
        var_rtn_rv_slot: &mut f64,
        var_tka_slot: &mut f64,
        var_tka_rv_slot: &mut f64,
        var_tkd_slot: &mut f64,
        var_tkd_1_slot: &mut f64,
        var_tkd_1_rv_slot: &mut f64,
        var_tkd_rv_slot: &mut f64,
        var_tkd_sq_slot: &mut f64,
        var_tkd_sq_rv_slot: &mut f64,
        var_ubibot_slot: &mut f64,
        var_ubibot_rv_slot: &mut f64,
        var_ubigat_slot: &mut f64,
        var_ubigat_rv_slot: &mut f64,
        var_ubisti_slot: &mut f64,
        var_ubisti_rv_slot: &mut f64,
        var_vbibot_slot: &mut f64,
        var_vbibot_rv_slot: &mut f64,
        var_vbigat_slot: &mut f64,
        var_vbigat_rv_slot: &mut f64,
        var_vbiinvbot_slot: &mut f64,
        var_vbiinvbot_rv_slot: &mut f64,
        var_vbiinvgat_slot: &mut f64,
        var_vbiinvgat_rv_slot: &mut f64,
        var_vbiinvsti_slot: &mut f64,
        var_vbiinvsti_rv_slot: &mut f64,
        var_vbisti_slot: &mut f64,
        var_vbisti_rv_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_atatbot: f64 = *var_atatbot_slot;
        let mut var_atatbot_rv: f64 = *var_atatbot_rv_slot;
        let mut var_atatgat: f64 = *var_atatgat_slot;
        let mut var_atatgat_rv: f64 = *var_atatgat_rv_slot;
        let mut var_atatsti: f64 = *var_atatsti_slot;
        let mut var_atatsti_rv: f64 = *var_atatsti_rv_slot;
        let mut var_auxt: f64 = *var_auxt_slot;
        let mut var_auxt_rv: f64 = *var_auxt_rv_slot;
        let mut var_btatpartbot: f64 = *var_btatpartbot_slot;
        let mut var_btatpartbot_rv: f64 = *var_btatpartbot_rv_slot;
        let mut var_btatpartgat: f64 = *var_btatpartgat_slot;
        let mut var_btatpartgat_rv: f64 = *var_btatpartgat_rv_slot;
        let mut var_btatpartsti: f64 = *var_btatpartsti_slot;
        let mut var_btatpartsti_rv: f64 = *var_btatpartsti_rv_slot;
        let mut var_cjobot: f64 = *var_cjobot_slot;
        let mut var_cjobot_rv: f64 = *var_cjobot_rv_slot;
        let mut var_cjogat: f64 = *var_cjogat_slot;
        let mut var_cjogat_rv: f64 = *var_cjogat_rv_slot;
        let mut var_cjosti: f64 = *var_cjosti_slot;
        let mut var_cjosti_rv: f64 = *var_cjosti_rv_slot;
        let mut var_delt: f64 = *var_delt_slot;
        let mut var_delt_rv: f64 = *var_delt_rv_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_rv: f64 = *var_delta_rv_slot;
        let mut var_deltaebot: f64 = *var_deltaebot_slot;
        let mut var_deltaebot_rv: f64 = *var_deltaebot_rv_slot;
        let mut var_deltaegat: f64 = *var_deltaegat_slot;
        let mut var_deltaegat_rv: f64 = *var_deltaegat_rv_slot;
        let mut var_deltaesti: f64 = *var_deltaesti_slot;
        let mut var_deltaesti_rv: f64 = *var_deltaesti_rv_slot;
        let mut var_deltaphigd: f64 = *var_deltaphigd_slot;
        let mut var_deltaphigd_rv: f64 = *var_deltaphigd_rv_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_rv: f64 = *var_eg_rv_slot;
        let mut var_fbbtbot: f64 = *var_fbbtbot_slot;
        let mut var_fbbtbot_rv: f64 = *var_fbbtbot_rv_slot;
        let mut var_fbbtgat: f64 = *var_fbbtgat_slot;
        let mut var_fbbtgat_dn5: f64 = *var_fbbtgat_dn5_slot;
        let mut var_fbbtgat_dn6: f64 = *var_fbbtgat_dn6_slot;
        let mut var_fbbtgat_dn7: f64 = *var_fbbtgat_dn7_slot;
        let mut var_fbbtgat_dn8: f64 = *var_fbbtgat_dn8_slot;
        let mut var_fbbtgat_rv: f64 = *var_fbbtgat_rv_slot;
        let mut var_fbbtsti: f64 = *var_fbbtsti_slot;
        let mut var_fbbtsti_rv: f64 = *var_fbbtsti_rv_slot;
        let mut var_ftdbot: f64 = *var_ftdbot_slot;
        let mut var_ftdbot_rv: f64 = *var_ftdbot_rv_slot;
        let mut var_ftdgat: f64 = *var_ftdgat_slot;
        let mut var_ftdgat2nd: f64 = *var_ftdgat2nd_slot;
        let mut var_ftdgat2nd_rv: f64 = *var_ftdgat2nd_rv_slot;
        let mut var_ftdgat_rv: f64 = *var_ftdgat_rv_slot;
        let mut var_ftdsti: f64 = *var_ftdsti_slot;
        let mut var_ftdsti_rv: f64 = *var_ftdsti_rv_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_guard27_rv: f64 = *var_guard27_rv_slot;
        let mut var_idsatbot: f64 = *var_idsatbot_slot;
        let mut var_idsatbot_rv: f64 = *var_idsatbot_rv_slot;
        let mut var_idsatgat: f64 = *var_idsatgat_slot;
        let mut var_idsatgat_rv: f64 = *var_idsatgat_rv_slot;
        let mut var_idsatsti: f64 = *var_idsatsti_slot;
        let mut var_idsatsti_rv: f64 = *var_idsatsti_rv_slot;
        let mut var_inv_phit: f64 = *var_inv_phit_slot;
        let mut var_inv_phit_rv: f64 = *var_inv_phit_rv_slot;
        let mut var_inv_phita: f64 = *var_inv_phita_slot;
        let mut var_inv_phita_rv: f64 = *var_inv_phita_rv_slot;
        let mut var_ln_rtn: f64 = *var_ln_rtn_slot;
        let mut var_ln_rtn_rv: f64 = *var_ln_rtn_rv_slot;
        let mut var_one_minus_pgat2nd_d: f64 = *var_one_minus_pgat2nd_d_slot;
        let mut var_one_minus_pgat2nd_d_rv: f64 = *var_one_minus_pgat2nd_d_rv_slot;
        let mut var_one_over_one_minus_pgat2nd_d: f64 = *var_one_over_one_minus_pgat2nd_d_slot;
        let mut var_one_over_one_minus_pgat2nd_d_rv: f64 = *var_one_over_one_minus_pgat2nd_d_rv_slot;
        let mut var_pgat2nd_d: f64 = *var_pgat2nd_d_slot;
        let mut var_pgat2nd_d_rv: f64 = *var_pgat2nd_d_rv_slot;
        let mut var_phibfac: f64 = *var_phibfac_slot;
        let mut var_phibfac_rv: f64 = *var_phibfac_rv_slot;
        let mut var_phigdbot: f64 = *var_phigdbot_slot;
        let mut var_phigdbot_rv: f64 = *var_phigdbot_rv_slot;
        let mut var_phigdgat: f64 = *var_phigdgat_slot;
        let mut var_phigdgat2nd: f64 = *var_phigdgat2nd_slot;
        let mut var_phigdgat2nd_rv: f64 = *var_phigdgat2nd_rv_slot;
        let mut var_phigdgat_rv: f64 = *var_phigdgat_rv_slot;
        let mut var_phigdsti: f64 = *var_phigdsti_slot;
        let mut var_phigdsti_rv: f64 = *var_phigdsti_rv_slot;
        let mut var_phiggat2nd_d: f64 = *var_phiggat2nd_d_slot;
        let mut var_phiggat2nd_d_rv: f64 = *var_phiggat2nd_d_rv_slot;
        let mut var_phigrgat2nd_d: f64 = *var_phigrgat2nd_d_slot;
        let mut var_phigrgat2nd_d_rv: f64 = *var_phigrgat2nd_d_rv_slot;
        let mut var_phit: f64 = *var_phit_slot;
        let mut var_phit_rv: f64 = *var_phit_rv_slot;
        let mut var_phita: f64 = *var_phita_slot;
        let mut var_phita_rv: f64 = *var_phita_rv_slot;
        let mut var_phitd: f64 = *var_phitd_slot;
        let mut var_phitd_rv: f64 = *var_phitd_rv_slot;
        let mut var_phitdinv: f64 = *var_phitdinv_slot;
        let mut var_phitdinv_rv: f64 = *var_phitdinv_rv_slot;
        let mut var_qpref2bot: f64 = *var_qpref2bot_slot;
        let mut var_qpref2bot_rv: f64 = *var_qpref2bot_rv_slot;
        let mut var_qpref2gat: f64 = *var_qpref2gat_slot;
        let mut var_qpref2gat_rv: f64 = *var_qpref2gat_rv_slot;
        let mut var_qpref2sti: f64 = *var_qpref2sti_slot;
        let mut var_qpref2sti_rv: f64 = *var_qpref2sti_rv_slot;
        let mut var_qprefbot: f64 = *var_qprefbot_slot;
        let mut var_qprefbot_rv: f64 = *var_qprefbot_rv_slot;
        let mut var_qprefgat: f64 = *var_qprefgat_slot;
        let mut var_qprefgat_rv: f64 = *var_qprefgat_rv_slot;
        let mut var_qprefsti: f64 = *var_qprefsti_slot;
        let mut var_qprefsti_rv: f64 = *var_qprefsti_rv_slot;
        let mut var_rta: f64 = *var_rta_slot;
        let mut var_rta_rv: f64 = *var_rta_rv_slot;
        let mut var_rtn: f64 = *var_rtn_slot;
        let mut var_rtn_rv: f64 = *var_rtn_rv_slot;
        let mut var_tka: f64 = *var_tka_slot;
        let mut var_tka_rv: f64 = *var_tka_rv_slot;
        let mut var_tkd: f64 = *var_tkd_slot;
        let mut var_tkd_1: f64 = *var_tkd_1_slot;
        let mut var_tkd_1_rv: f64 = *var_tkd_1_rv_slot;
        let mut var_tkd_rv: f64 = *var_tkd_rv_slot;
        let mut var_tkd_sq: f64 = *var_tkd_sq_slot;
        let mut var_tkd_sq_rv: f64 = *var_tkd_sq_rv_slot;
        let mut var_ubibot: f64 = *var_ubibot_slot;
        let mut var_ubibot_rv: f64 = *var_ubibot_rv_slot;
        let mut var_ubigat: f64 = *var_ubigat_slot;
        let mut var_ubigat_rv: f64 = *var_ubigat_rv_slot;
        let mut var_ubisti: f64 = *var_ubisti_slot;
        let mut var_ubisti_rv: f64 = *var_ubisti_rv_slot;
        let mut var_vbibot: f64 = *var_vbibot_slot;
        let mut var_vbibot_rv: f64 = *var_vbibot_rv_slot;
        let mut var_vbigat: f64 = *var_vbigat_slot;
        let mut var_vbigat_rv: f64 = *var_vbigat_rv_slot;
        let mut var_vbiinvbot: f64 = *var_vbiinvbot_slot;
        let mut var_vbiinvbot_rv: f64 = *var_vbiinvbot_rv_slot;
        let mut var_vbiinvgat: f64 = *var_vbiinvgat_slot;
        let mut var_vbiinvgat_rv: f64 = *var_vbiinvgat_rv_slot;
        let mut var_vbiinvsti: f64 = *var_vbiinvsti_slot;
        let mut var_vbiinvsti_rv: f64 = *var_vbiinvsti_rv_slot;
        let mut var_vbisti: f64 = *var_vbisti_slot;
        let mut var_vbisti_rv: f64 = *var_vbisti_rv_slot;

        let (assign1960_e2447,) = {
    if (var_guard7 != 0.0) {
        let assign1960_e2424: f64 = (var_pgatd_i * var_fpgat2d_i);
        let (assign1960_e2431,) = {
            if (assign1960_e2424 > 0.05) {
                let assign1960_e2429: f64 = (var_pgatd_i * var_fpgat2d_i);
                (assign1960_e2429,)
            } else {
                (0.05,)
            }
        };
        let (assign1960_e2445,) = {
            if (assign1960_e2431 < 0.95) {
                let assign1960_e2436: f64 = (var_pgatd_i * var_fpgat2d_i);
                let (assign1960_e2443,) = {
                    if (assign1960_e2436 > 0.05) {
                        let assign1960_e2441: f64 = (var_pgatd_i * var_fpgat2d_i);
                        (assign1960_e2441,)
                    } else {
                        (0.05,)
                    }
                };
                (assign1960_e2443,)
            } else {
                (0.95,)
            }
        };
        (assign1960_e2445,)
    } else {
        (var_pgat2nd_d,)
    }
};
        var_pgat2nd_d = assign1960_e2447;
        var_pgat2nd_d_rv = 0.0;

        let (assign1970_e2453,) = {
    if (var_guard7 != 0.0) {
        let assign1970_e2451: f64 = (var_phiggatd_i * var_fphiggat2d_i);
        (assign1970_e2451,)
    } else {
        (var_phiggat2nd_d,)
    }
};
        var_phiggat2nd_d = assign1970_e2453;
        var_phiggat2nd_d_rv = 0.0;

        let (assign1980_e2459,) = {
    if (var_guard7 != 0.0) {
        let assign1980_e2457: f64 = (var_phiggat2nd_d + var_deltaphigr);
        (assign1980_e2457,)
    } else {
        (var_phigrgat2nd_d,)
    }
};
        var_phigrgat2nd_d = assign1980_e2459;
        var_phigrgat2nd_d_rv = 0.0;

        let (assign1990_e2465,) = {
    if (var_guard7 != 0.0) {
        let assign1990_e2463: f64 = (1.0 - var_pgat2nd_d);
        (assign1990_e2463,)
    } else {
        (var_one_minus_pgat2nd_d,)
    }
};
        var_one_minus_pgat2nd_d = assign1990_e2465;
        var_one_minus_pgat2nd_d_rv = 0.0;

        let (assign2000_e2471,) = {
    if (var_guard7 != 0.0) {
        let assign2000_e2469: f64 = (1.0 / var_one_minus_pgat2nd_d);
        (assign2000_e2469,)
    } else {
        (var_one_over_one_minus_pgat2nd_d,)
    }
};
        var_one_over_one_minus_pgat2nd_d = assign2000_e2471;
        var_one_over_one_minus_pgat2nd_d_rv = 0.0;

        let assign2050_e2493: f64 = ctx_temp;
        let assign2050_e2495: f64 = (assign2050_e2493 + p.p55);
        let assign2050_e2497: f64 = (assign2050_e2495 + p.p35);
        var_tka = assign2050_e2497;
        var_tka_rv = 0.0;

        let assign2060_e2500: f64 = (var_tka / var_tkr);
        var_rta = assign2060_e2500;
        var_rta_rv = 0.0;

        let assign2070_e2503: f64 = (var_tka - var_tkr);
        var_delta = assign2070_e2503;
        var_delta_rv = 0.0;

        let assign2080_e2506: f64 = (var_tka * 1.3806505e-23);
        let assign2080_e2508: f64 = (assign2080_e2506 / 1.6021918e-19);
        var_phita = assign2080_e2508;
        var_phita_rv = 0.0;

        let assign2090_e2511: f64 = (1.0 / var_phita);
        var_inv_phita = assign2090_e2511;
        var_inv_phita_rv = 0.0;

        var_tkd = var_tka;
        var_tkd_rv = 0.0;

        let assign2110_e2515: f64 = (var_tkd * var_tkd);
        var_tkd_sq = assign2110_e2515;
        var_tkd_sq_rv = 0.0;

        let assign2120_e2518: f64 = (var_tkd - var_tkr);
        var_delt = assign2120_e2518;
        var_delt_rv = 0.0;

        let assign2130_e2521: f64 = (var_tkr / var_tkd);
        var_rtn = assign2130_e2521;
        var_rtn_rv = 0.0;

        let assign2140_e2523: f64 = (var_rtn).ln();
        var_ln_rtn = assign2140_e2523;
        var_ln_rtn_rv = 0.0;

        let assign2150_e2526: f64 = (var_tkd * 1.3806505e-23);
        let assign2150_e2528: f64 = (assign2150_e2526 / 1.6021918e-19);
        var_phit = assign2150_e2528;
        var_phit_rv = 0.0;

        let assign2160_e2531: f64 = (1.0 / var_phit);
        var_inv_phit = assign2160_e2531;
        var_inv_phit_rv = 0.0;

        let assign2170_e2535: f64 = (9.025e-5 * var_tkd);
        let assign2170_e2536: f64 = (1.179 - assign2170_e2535);
        let assign2170_e2539: f64 = (3.05e-7 * var_tkd_sq);
        let assign2170_e2540: f64 = (assign2170_e2536 - assign2170_e2539);
        var_eg = assign2170_e2540;
        var_eg_rv = 0.0;

        let assign2180_e2544: f64 = (0.00045 * var_tkd);
        let assign2180_e2545: f64 = (1.045 + assign2180_e2544);
        let assign2180_e2549: f64 = (0.0014 * var_tkd);
        let assign2180_e2550: f64 = (0.523 + assign2180_e2549);
        let assign2180_e2553: f64 = (1.48e-6 * var_tkd_sq);
        let assign2180_e2554: f64 = (assign2180_e2550 - assign2180_e2553);
        let assign2180_e2555: f64 = (assign2180_e2545 * assign2180_e2554);
        let assign2180_e2557: f64 = (assign2180_e2555 * var_tkd_sq);
        let assign2180_e2559: f64 = (assign2180_e2557 / 90000.0);
        var_phibfac = assign2180_e2559;
        var_phibfac_rv = 0.0;

        let (assign2190_e2565,) = {
    if (var_phibfac > 0.001) {
        (var_phibfac,)
    } else {
        (0.001,)
    }
};
        var_phibfac = assign2190_e2565;
        var_phibfac_rv = 0.0;

        let assign2210_e2571: f64 = ctx_temp;
        let assign2210_e2573: f64 = (assign2210_e2571 + p.p55);
        let assign2210_e2575: f64 = (assign2210_e2573 + p.p35);
        let assign2210_e2578: f64 = (-250.0);
        let assign2210_e2579: f64 = (273.15 + assign2210_e2578);
        let assign2210_e2580: f64 = (assign2210_e2575).max(assign2210_e2579);
        var_tkd_1 = assign2210_e2580;
        var_tkd_1_rv = 0.0;

        let assign2220_e2583: f64 = (var_tkd_1 / var_tkr_1);
        var_auxt = assign2220_e2583;
        var_auxt_rv = 0.0;

        let assign2230_e2586: f64 = (var_kbol_over_qele * var_tkd_1);
        var_phitd = assign2230_e2586;
        var_phitd_rv = 0.0;

        let assign2240_e2589: f64 = (1.0 / var_phitd);
        var_phitdinv = assign2240_e2589;
        var_phitdinv_rv = 0.0;

        let assign2250_e2592: f64 = (0.000702 * var_tkd_1);
        let assign2250_e2594: f64 = (assign2250_e2592 * var_tkd_1);
        let assign2250_e2595: f64 = (-assign2250_e2594);
        let assign2250_e2598: f64 = (1108.0 + var_tkd_1);
        let assign2250_e2599: f64 = (assign2250_e2595 / assign2250_e2598);
        var_deltaphigd = assign2250_e2599;
        var_deltaphigd_rv = 0.0;

        let assign2260_e2602: f64 = (p.p827 + var_deltaphigd);
        var_phigdbot = assign2260_e2602;
        var_phigdbot_rv = 0.0;

        let assign2270_e2605: f64 = (p.p828 + var_deltaphigd);
        var_phigdsti = assign2270_e2605;
        var_phigdsti_rv = 0.0;

        let assign2280_e2608: f64 = (p.p829 + var_deltaphigd);
        var_phigdgat = assign2280_e2608;
        var_phigdgat_rv = 0.0;

        let assign2290_e2611: f64 = (var_auxt).powf(1.5);
        let assign2290_e2615: f64 = (var_phigrbot * var_phitrinv);
        let assign2290_e2618: f64 = (var_phigdbot * var_phitdinv);
        let assign2290_e2619: f64 = (assign2290_e2615 - assign2290_e2618);
        let assign2290_e2620: f64 = (0.5 * assign2290_e2619);
        let assign2290_e2621: f64 = (assign2290_e2620).exp();
        let assign2290_e2622: f64 = (assign2290_e2611 * assign2290_e2621);
        var_ftdbot = assign2290_e2622;
        var_ftdbot_rv = 0.0;

        let assign2300_e2625: f64 = (var_auxt).powf(1.5);
        let assign2300_e2629: f64 = (var_phigrsti * var_phitrinv);
        let assign2300_e2632: f64 = (var_phigdsti * var_phitdinv);
        let assign2300_e2633: f64 = (assign2300_e2629 - assign2300_e2632);
        let assign2300_e2634: f64 = (0.5 * assign2300_e2633);
        let assign2300_e2635: f64 = (assign2300_e2634).exp();
        let assign2300_e2636: f64 = (assign2300_e2625 * assign2300_e2635);
        var_ftdsti = assign2300_e2636;
        var_ftdsti_rv = 0.0;

        let assign2310_e2639: f64 = (var_auxt).powf(1.5);
        let assign2310_e2643: f64 = (var_phigrgat * var_phitrinv);
        let assign2310_e2646: f64 = (var_phigdgat * var_phitdinv);
        let assign2310_e2647: f64 = (assign2310_e2643 - assign2310_e2646);
        let assign2310_e2648: f64 = (0.5 * assign2310_e2647);
        let assign2310_e2649: f64 = (assign2310_e2648).exp();
        let assign2310_e2650: f64 = (assign2310_e2639 * assign2310_e2649);
        var_ftdgat = assign2310_e2650;
        var_ftdgat_rv = 0.0;

        let assign2320_e2653: f64 = (p.p830 * var_ftdbot);
        let assign2320_e2655: f64 = (assign2320_e2653 * var_ftdbot);
        var_idsatbot = assign2320_e2655;
        var_idsatbot_rv = 0.0;

        let assign2330_e2658: f64 = (p.p831 * var_ftdsti);
        let assign2330_e2660: f64 = (assign2330_e2658 * var_ftdsti);
        var_idsatsti = assign2330_e2660;
        var_idsatsti_rv = 0.0;

        let assign2340_e2663: f64 = (p.p832 * var_ftdgat);
        let assign2340_e2665: f64 = (assign2340_e2663 * var_ftdgat);
        var_idsatgat = assign2340_e2665;
        var_idsatgat_rv = 0.0;

        let assign2350_e2668: f64 = (p.p821 * var_auxt);
        let assign2350_e2671: f64 = (2.0 * var_phitd);
        let assign2350_e2673: f64 = (var_ftdbot).ln();
        let assign2350_e2674: f64 = (assign2350_e2671 * assign2350_e2673);
        let assign2350_e2675: f64 = (assign2350_e2668 - assign2350_e2674);
        var_ubibot = assign2350_e2675;
        var_ubibot_rv = 0.0;

        let assign2360_e2678: f64 = (p.p822 * var_auxt);
        let assign2360_e2681: f64 = (2.0 * var_phitd);
        let assign2360_e2683: f64 = (var_ftdsti).ln();
        let assign2360_e2684: f64 = (assign2360_e2681 * assign2360_e2683);
        let assign2360_e2685: f64 = (assign2360_e2678 - assign2360_e2684);
        var_ubisti = assign2360_e2685;
        var_ubisti_rv = 0.0;

        let assign2370_e2688: f64 = (p.p823 * var_auxt);
        let assign2370_e2691: f64 = (2.0 * var_phitd);
        let assign2370_e2693: f64 = (var_ftdgat).ln();
        let assign2370_e2694: f64 = (assign2370_e2691 * assign2370_e2693);
        let assign2370_e2695: f64 = (assign2370_e2688 - assign2370_e2694);
        var_ubigat = assign2370_e2695;
        var_ubigat_rv = 0.0;

        let assign2380_e2701: f64 = (0.05 - var_ubibot);
        let assign2380_e2703: f64 = (assign2380_e2701 * var_phitdinv);
        let assign2380_e2704: f64 = (assign2380_e2703).exp();
        let assign2380_e2705: f64 = (1.0 + assign2380_e2704);
        let assign2380_e2706: f64 = (assign2380_e2705).ln();
        let assign2380_e2707: f64 = (var_phitd * assign2380_e2706);
        let assign2380_e2708: f64 = (var_ubibot + assign2380_e2707);
        var_vbibot = assign2380_e2708;
        var_vbibot_rv = 0.0;

        let assign2390_e2714: f64 = (0.05 - var_ubisti);
        let assign2390_e2716: f64 = (assign2390_e2714 * var_phitdinv);
        let assign2390_e2717: f64 = (assign2390_e2716).exp();
        let assign2390_e2718: f64 = (1.0 + assign2390_e2717);
        let assign2390_e2719: f64 = (assign2390_e2718).ln();
        let assign2390_e2720: f64 = (var_phitd * assign2390_e2719);
        let assign2390_e2721: f64 = (var_ubisti + assign2390_e2720);
        var_vbisti = assign2390_e2721;
        var_vbisti_rv = 0.0;

        let assign2400_e2727: f64 = (0.05 - var_ubigat);
        let assign2400_e2729: f64 = (assign2400_e2727 * var_phitdinv);
        let assign2400_e2730: f64 = (assign2400_e2729).exp();
        let assign2400_e2731: f64 = (1.0 + assign2400_e2730);
        let assign2400_e2732: f64 = (assign2400_e2731).ln();
        let assign2400_e2733: f64 = (var_phitd * assign2400_e2732);
        let assign2400_e2734: f64 = (var_ubigat + assign2400_e2733);
        var_vbigat = assign2400_e2734;
        var_vbigat_rv = 0.0;

        let assign2410_e2737: f64 = (1.0 / var_vbibot);
        var_vbiinvbot = assign2410_e2737;
        var_vbiinvbot_rv = 0.0;

        let assign2420_e2740: f64 = (1.0 / var_vbisti);
        var_vbiinvsti = assign2420_e2740;
        var_vbiinvsti_rv = 0.0;

        let assign2430_e2743: f64 = (1.0 / var_vbigat);
        var_vbiinvgat = assign2430_e2743;
        var_vbiinvgat_rv = 0.0;

        let assign2440_e2747: f64 = (p.p821 * var_vbiinvbot);
        let assign2440_e2749: f64 = (assign2440_e2747).powf(p.p824);
        let assign2440_e2750: f64 = (p.p818 * assign2440_e2749);
        var_cjobot = assign2440_e2750;
        var_cjobot_rv = 0.0;

        let assign2450_e2754: f64 = (p.p822 * var_vbiinvsti);
        let assign2450_e2756: f64 = (assign2450_e2754).powf(p.p825);
        let assign2450_e2757: f64 = (p.p819 * assign2450_e2756);
        var_cjosti = assign2450_e2757;
        var_cjosti_rv = 0.0;

        let assign2460_e2761: f64 = (p.p823 * var_vbiinvgat);
        let assign2460_e2763: f64 = (assign2460_e2761).powf(p.p826);
        let assign2460_e2764: f64 = (p.p820 * assign2460_e2763);
        var_cjogat = assign2460_e2764;
        var_cjogat_rv = 0.0;

        let assign2470_e2767: f64 = (var_cjobot * var_vbibot);
        let assign2470_e2769: f64 = (assign2470_e2767 * var_one_over_one_minus_pbot);
        var_qprefbot = assign2470_e2769;
        var_qprefbot_rv = 0.0;

        let assign2480_e2772: f64 = (var_cjosti * var_vbisti);
        let assign2480_e2774: f64 = (assign2480_e2772 * var_one_over_one_minus_psti);
        var_qprefsti = assign2480_e2774;
        var_qprefsti_rv = 0.0;

        let assign2490_e2777: f64 = (var_cjogat * var_vbigat);
        let assign2490_e2779: f64 = (assign2490_e2777 * var_one_over_one_minus_pgat);
        var_qprefgat = assign2490_e2779;
        var_qprefgat_rv = 0.0;

        let assign2500_e2782: f64 = (2.0 * var_cjobot);
        var_qpref2bot = assign2500_e2782;
        var_qpref2bot_rv = 0.0;

        let assign2510_e2785: f64 = (2.0 * var_cjosti);
        var_qpref2sti = assign2510_e2785;
        var_qpref2sti_rv = 0.0;

        let assign2520_e2788: f64 = (2.0 * var_cjogat);
        var_qpref2gat = assign2520_e2788;
        var_qpref2gat_rv = 0.0;

        let assign2530_e2791: f64 = (0.5 * var_phigdbot);
        let assign2530_e2793: f64 = (assign2530_e2791).max(var_phitd);
        var_deltaebot = assign2530_e2793;
        var_deltaebot_rv = 0.0;

        let assign2540_e2796: f64 = (0.5 * var_phigdsti);
        let assign2540_e2798: f64 = (assign2540_e2796).max(var_phitd);
        var_deltaesti = assign2540_e2798;
        var_deltaesti_rv = 0.0;

        let assign2550_e2801: f64 = (0.5 * var_phigdgat);
        let assign2550_e2803: f64 = (assign2550_e2801).max(var_phitd);
        var_deltaegat = assign2550_e2803;
        var_deltaegat_rv = 0.0;

        let assign2560_e2806: f64 = (var_deltaebot * var_phitdinv);
        var_atatbot = assign2560_e2806;
        var_atatbot_rv = 0.0;

        let assign2570_e2809: f64 = (var_deltaesti * var_phitdinv);
        var_atatsti = assign2570_e2809;
        var_atatsti_rv = 0.0;

        let assign2580_e2812: f64 = (var_deltaegat * var_phitdinv);
        var_atatgat = assign2580_e2812;
        var_atatgat_rv = 0.0;

        let assign2590_e2815: f64 = (32.0 * p.p841);
        let assign2590_e2817: f64 = (assign2590_e2815 * 9.1093826e-31);
        let assign2590_e2819: f64 = (assign2590_e2817 * 1.6021918e-19);
        let assign2590_e2822: f64 = (var_deltaebot * var_deltaebot);
        let assign2590_e2824: f64 = (assign2590_e2822 * var_deltaebot);
        let assign2590_e2825: f64 = (assign2590_e2819 * assign2590_e2824);
        let assign2590_e2826: f64 = (assign2590_e2825).sqrt();
        let assign2590_e2829: f64 = (3.0 * 1.05457168e-34);
        let assign2590_e2830: f64 = (assign2590_e2826 / assign2590_e2829);
        var_btatpartbot = assign2590_e2830;
        var_btatpartbot_rv = 0.0;

        let assign2600_e2833: f64 = (32.0 * p.p842);
        let assign2600_e2835: f64 = (assign2600_e2833 * 9.1093826e-31);
        let assign2600_e2837: f64 = (assign2600_e2835 * 1.6021918e-19);
        let assign2600_e2840: f64 = (var_deltaesti * var_deltaesti);
        let assign2600_e2842: f64 = (assign2600_e2840 * var_deltaesti);
        let assign2600_e2843: f64 = (assign2600_e2837 * assign2600_e2842);
        let assign2600_e2844: f64 = (assign2600_e2843).sqrt();
        let assign2600_e2847: f64 = (3.0 * 1.05457168e-34);
        let assign2600_e2848: f64 = (assign2600_e2844 / assign2600_e2847);
        var_btatpartsti = assign2600_e2848;
        var_btatpartsti_rv = 0.0;

        let assign2610_e2851: f64 = (32.0 * p.p843);
        let assign2610_e2853: f64 = (assign2610_e2851 * 9.1093826e-31);
        let assign2610_e2855: f64 = (assign2610_e2853 * 1.6021918e-19);
        let assign2610_e2858: f64 = (var_deltaegat * var_deltaegat);
        let assign2610_e2860: f64 = (assign2610_e2858 * var_deltaegat);
        let assign2610_e2861: f64 = (assign2610_e2855 * assign2610_e2860);
        let assign2610_e2862: f64 = (assign2610_e2861).sqrt();
        let assign2610_e2865: f64 = (3.0 * 1.05457168e-34);
        let assign2610_e2866: f64 = (assign2610_e2862 / assign2610_e2865);
        var_btatpartgat = assign2610_e2866;
        var_btatpartgat_rv = 0.0;

        let assign2620_e2872: f64 = (var_tkd_1 - var_tkr_1);
        let assign2620_e2873: f64 = (p.p850 * assign2620_e2872);
        let assign2620_e2874: f64 = (1.0 + assign2620_e2873);
        let assign2620_e2875: f64 = (p.p847 * assign2620_e2874);
        var_fbbtbot = assign2620_e2875;
        var_fbbtbot_rv = 0.0;

        let assign2630_e2881: f64 = (var_tkd_1 - var_tkr_1);
        let assign2630_e2882: f64 = (p.p851 * assign2630_e2881);
        let assign2630_e2883: f64 = (1.0 + assign2630_e2882);
        let assign2630_e2884: f64 = (p.p848 * assign2630_e2883);
        var_fbbtsti = assign2630_e2884;
        var_fbbtsti_rv = 0.0;

        let assign2640_e2890: f64 = (var_tkd_1 - var_tkr_1);
        let assign2640_e2891: f64 = (p.p852 * assign2640_e2890);
        let assign2640_e2892: f64 = (1.0 + assign2640_e2891);
        let assign2640_e2893: f64 = (p.p849 * assign2640_e2892);
        var_fbbtgat = assign2640_e2893;
        var_fbbtgat_dn5 = 0.0;
        var_fbbtgat_dn6 = 0.0;
        var_fbbtgat_dn7 = 0.0;
        var_fbbtgat_dn8 = 0.0;
        var_fbbtgat_rv = 0.0;

        let (assign2650_e2899,) = {
    if (var_fbbtbot > 0.0) {
        (var_fbbtbot,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot = assign2650_e2899;
        var_fbbtbot_rv = 0.0;

        let (assign2660_e2905,) = {
    if (var_fbbtsti > 0.0) {
        (var_fbbtsti,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti = assign2660_e2905;
        var_fbbtsti_rv = 0.0;

        let (assign2670_e2911, assign2670_e2911_d_n5, assign2670_e2911_d_n6, assign2670_e2911_d_n7, assign2670_e2911_d_n8,) = {
    if (var_fbbtgat > 0.0) {
        (var_fbbtgat, var_fbbtgat_dn5, var_fbbtgat_dn6, var_fbbtgat_dn7, var_fbbtgat_dn8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat = assign2670_e2911;
        var_fbbtgat_dn5 = assign2670_e2911_d_n5;
        var_fbbtgat_dn6 = assign2670_e2911_d_n6;
        var_fbbtgat_dn7 = assign2670_e2911_d_n7;
        var_fbbtgat_dn8 = assign2670_e2911_d_n8;
        var_fbbtgat_rv = 0.0;

        let assign2680_e2914: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard27 = assign2680_e2914;
        var_guard27_rv = 0.0;

        let (assign2690_e2920,) = {
    if (var_guard27 != 0.0) {
        let assign2690_e2918: f64 = (var_phiggat2nd + var_deltaphigd);
        (assign2690_e2918,)
    } else {
        (var_phigdgat2nd,)
    }
};
        var_phigdgat2nd = assign2690_e2920;
        var_phigdgat2nd_rv = 0.0;

        let (assign2700_e2937,) = {
    if (var_guard27 != 0.0) {
        let assign2700_e2924: f64 = (var_auxt).powf(1.5);
        let assign2700_e2928: f64 = (var_phigrgat2nd * var_phitrinv);
        let assign2700_e2931: f64 = (var_phigdgat2nd * var_phitdinv);
        let assign2700_e2932: f64 = (assign2700_e2928 - assign2700_e2931);
        let assign2700_e2933: f64 = (0.5 * assign2700_e2932);
        let assign2700_e2934: f64 = (assign2700_e2933).exp();
        let assign2700_e2935: f64 = (assign2700_e2924 * assign2700_e2934);
        (assign2700_e2935,)
    } else {
        (var_ftdgat2nd,)
    }
};
        var_ftdgat2nd = assign2700_e2937;
        var_ftdgat2nd_rv = 0.0;

        *var_atatbot_slot = var_atatbot;
        *var_atatbot_rv_slot = var_atatbot_rv;
        *var_atatgat_slot = var_atatgat;
        *var_atatgat_rv_slot = var_atatgat_rv;
        *var_atatsti_slot = var_atatsti;
        *var_atatsti_rv_slot = var_atatsti_rv;
        *var_auxt_slot = var_auxt;
        *var_auxt_rv_slot = var_auxt_rv;
        *var_btatpartbot_slot = var_btatpartbot;
        *var_btatpartbot_rv_slot = var_btatpartbot_rv;
        *var_btatpartgat_slot = var_btatpartgat;
        *var_btatpartgat_rv_slot = var_btatpartgat_rv;
        *var_btatpartsti_slot = var_btatpartsti;
        *var_btatpartsti_rv_slot = var_btatpartsti_rv;
        *var_cjobot_slot = var_cjobot;
        *var_cjobot_rv_slot = var_cjobot_rv;
        *var_cjogat_slot = var_cjogat;
        *var_cjogat_rv_slot = var_cjogat_rv;
        *var_cjosti_slot = var_cjosti;
        *var_cjosti_rv_slot = var_cjosti_rv;
        *var_delt_slot = var_delt;
        *var_delt_rv_slot = var_delt_rv;
        *var_delta_slot = var_delta;
        *var_delta_rv_slot = var_delta_rv;
        *var_deltaebot_slot = var_deltaebot;
        *var_deltaebot_rv_slot = var_deltaebot_rv;
        *var_deltaegat_slot = var_deltaegat;
        *var_deltaegat_rv_slot = var_deltaegat_rv;
        *var_deltaesti_slot = var_deltaesti;
        *var_deltaesti_rv_slot = var_deltaesti_rv;
        *var_deltaphigd_slot = var_deltaphigd;
        *var_deltaphigd_rv_slot = var_deltaphigd_rv;
        *var_eg_slot = var_eg;
        *var_eg_rv_slot = var_eg_rv;
        *var_fbbtbot_slot = var_fbbtbot;
        *var_fbbtbot_rv_slot = var_fbbtbot_rv;
        *var_fbbtgat_slot = var_fbbtgat;
        *var_fbbtgat_dn5_slot = var_fbbtgat_dn5;
        *var_fbbtgat_dn6_slot = var_fbbtgat_dn6;
        *var_fbbtgat_dn7_slot = var_fbbtgat_dn7;
        *var_fbbtgat_dn8_slot = var_fbbtgat_dn8;
        *var_fbbtgat_rv_slot = var_fbbtgat_rv;
        *var_fbbtsti_slot = var_fbbtsti;
        *var_fbbtsti_rv_slot = var_fbbtsti_rv;
        *var_ftdbot_slot = var_ftdbot;
        *var_ftdbot_rv_slot = var_ftdbot_rv;
        *var_ftdgat_slot = var_ftdgat;
        *var_ftdgat2nd_slot = var_ftdgat2nd;
        *var_ftdgat2nd_rv_slot = var_ftdgat2nd_rv;
        *var_ftdgat_rv_slot = var_ftdgat_rv;
        *var_ftdsti_slot = var_ftdsti;
        *var_ftdsti_rv_slot = var_ftdsti_rv;
        *var_guard27_slot = var_guard27;
        *var_guard27_rv_slot = var_guard27_rv;
        *var_idsatbot_slot = var_idsatbot;
        *var_idsatbot_rv_slot = var_idsatbot_rv;
        *var_idsatgat_slot = var_idsatgat;
        *var_idsatgat_rv_slot = var_idsatgat_rv;
        *var_idsatsti_slot = var_idsatsti;
        *var_idsatsti_rv_slot = var_idsatsti_rv;
        *var_inv_phit_slot = var_inv_phit;
        *var_inv_phit_rv_slot = var_inv_phit_rv;
        *var_inv_phita_slot = var_inv_phita;
        *var_inv_phita_rv_slot = var_inv_phita_rv;
        *var_ln_rtn_slot = var_ln_rtn;
        *var_ln_rtn_rv_slot = var_ln_rtn_rv;
        *var_one_minus_pgat2nd_d_slot = var_one_minus_pgat2nd_d;
        *var_one_minus_pgat2nd_d_rv_slot = var_one_minus_pgat2nd_d_rv;
        *var_one_over_one_minus_pgat2nd_d_slot = var_one_over_one_minus_pgat2nd_d;
        *var_one_over_one_minus_pgat2nd_d_rv_slot = var_one_over_one_minus_pgat2nd_d_rv;
        *var_pgat2nd_d_slot = var_pgat2nd_d;
        *var_pgat2nd_d_rv_slot = var_pgat2nd_d_rv;
        *var_phibfac_slot = var_phibfac;
        *var_phibfac_rv_slot = var_phibfac_rv;
        *var_phigdbot_slot = var_phigdbot;
        *var_phigdbot_rv_slot = var_phigdbot_rv;
        *var_phigdgat_slot = var_phigdgat;
        *var_phigdgat2nd_slot = var_phigdgat2nd;
        *var_phigdgat2nd_rv_slot = var_phigdgat2nd_rv;
        *var_phigdgat_rv_slot = var_phigdgat_rv;
        *var_phigdsti_slot = var_phigdsti;
        *var_phigdsti_rv_slot = var_phigdsti_rv;
        *var_phiggat2nd_d_slot = var_phiggat2nd_d;
        *var_phiggat2nd_d_rv_slot = var_phiggat2nd_d_rv;
        *var_phigrgat2nd_d_slot = var_phigrgat2nd_d;
        *var_phigrgat2nd_d_rv_slot = var_phigrgat2nd_d_rv;
        *var_phit_slot = var_phit;
        *var_phit_rv_slot = var_phit_rv;
        *var_phita_slot = var_phita;
        *var_phita_rv_slot = var_phita_rv;
        *var_phitd_slot = var_phitd;
        *var_phitd_rv_slot = var_phitd_rv;
        *var_phitdinv_slot = var_phitdinv;
        *var_phitdinv_rv_slot = var_phitdinv_rv;
        *var_qpref2bot_slot = var_qpref2bot;
        *var_qpref2bot_rv_slot = var_qpref2bot_rv;
        *var_qpref2gat_slot = var_qpref2gat;
        *var_qpref2gat_rv_slot = var_qpref2gat_rv;
        *var_qpref2sti_slot = var_qpref2sti;
        *var_qpref2sti_rv_slot = var_qpref2sti_rv;
        *var_qprefbot_slot = var_qprefbot;
        *var_qprefbot_rv_slot = var_qprefbot_rv;
        *var_qprefgat_slot = var_qprefgat;
        *var_qprefgat_rv_slot = var_qprefgat_rv;
        *var_qprefsti_slot = var_qprefsti;
        *var_qprefsti_rv_slot = var_qprefsti_rv;
        *var_rta_slot = var_rta;
        *var_rta_rv_slot = var_rta_rv;
        *var_rtn_slot = var_rtn;
        *var_rtn_rv_slot = var_rtn_rv;
        *var_tka_slot = var_tka;
        *var_tka_rv_slot = var_tka_rv;
        *var_tkd_slot = var_tkd;
        *var_tkd_1_slot = var_tkd_1;
        *var_tkd_1_rv_slot = var_tkd_1_rv;
        *var_tkd_rv_slot = var_tkd_rv;
        *var_tkd_sq_slot = var_tkd_sq;
        *var_tkd_sq_rv_slot = var_tkd_sq_rv;
        *var_ubibot_slot = var_ubibot;
        *var_ubibot_rv_slot = var_ubibot_rv;
        *var_ubigat_slot = var_ubigat;
        *var_ubigat_rv_slot = var_ubigat_rv;
        *var_ubisti_slot = var_ubisti;
        *var_ubisti_rv_slot = var_ubisti_rv;
        *var_vbibot_slot = var_vbibot;
        *var_vbibot_rv_slot = var_vbibot_rv;
        *var_vbigat_slot = var_vbigat;
        *var_vbigat_rv_slot = var_vbigat_rv;
        *var_vbiinvbot_slot = var_vbiinvbot;
        *var_vbiinvbot_rv_slot = var_vbiinvbot_rv;
        *var_vbiinvgat_slot = var_vbiinvgat;
        *var_vbiinvgat_rv_slot = var_vbiinvgat_rv;
        *var_vbiinvsti_slot = var_vbiinvsti;
        *var_vbiinvsti_rv_slot = var_vbiinvsti_rv;
        *var_vbisti_slot = var_vbisti;
        *var_vbisti_rv_slot = var_vbisti_rv;
    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        var_auxt: f64,
        var_cjorbotd_i: f64,
        var_cjorgat2nd: f64,
        var_cjorgat2nd_d: f64,
        var_cjorgatd_i: f64,
        var_cjorstid_i: f64,
        var_deltaphigd: f64,
        var_fbbtrbotd_i: f64,
        var_fbbtrgatd_i: f64,
        var_fbbtrstid_i: f64,
        var_ftdgat2nd: f64,
        var_guard27: f64,
        var_idsatrbotd_i: f64,
        var_idsatrgatd_i: f64,
        var_idsatrstid_i: f64,
        var_mefftatbotd_i: f64,
        var_mefftatgatd_i: f64,
        var_mefftatstid_i: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_one_over_one_minus_pgat2nd: f64,
        var_one_over_one_minus_pgat2nd_d: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbotd_i: f64,
        var_pgat2nd: f64,
        var_pgat2nd_d: f64,
        var_pgatd_i: f64,
        var_phigbotd_i: f64,
        var_phiggat2nd_d: f64,
        var_phiggatd_i: f64,
        var_phigrbot_d: f64,
        var_phigrgat2nd_d: f64,
        var_phigrgat_d: f64,
        var_phigrsti_d: f64,
        var_phigstid_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitrinv: f64,
        var_pstid_i: f64,
        var_stfbbtbotd_i: f64,
        var_stfbbtgatd_i: f64,
        var_stfbbtstid_i: f64,
        var_swgat2nd_d: f64,
        var_tkd_1: f64,
        var_tkr_1: f64,
        var_vbirbotd_i: f64,
        var_vbirgat2nd: f64,
        var_vbirgat2nd_d: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_atatbot_d_slot: &mut f64,
        var_atatbot_d_rv_slot: &mut f64,
        var_atatgat_d_slot: &mut f64,
        var_atatgat_d_rv_slot: &mut f64,
        var_atatsti_d_slot: &mut f64,
        var_atatsti_d_rv_slot: &mut f64,
        var_btatpartbot_d_slot: &mut f64,
        var_btatpartbot_d_rv_slot: &mut f64,
        var_btatpartgat_d_slot: &mut f64,
        var_btatpartgat_d_rv_slot: &mut f64,
        var_btatpartsti_d_slot: &mut f64,
        var_btatpartsti_d_rv_slot: &mut f64,
        var_cjobot_d_slot: &mut f64,
        var_cjobot_d_rv_slot: &mut f64,
        var_cjogat2nd_slot: &mut f64,
        var_cjogat2nd_d_slot: &mut f64,
        var_cjogat2nd_d_rv_slot: &mut f64,
        var_cjogat2nd_rv_slot: &mut f64,
        var_cjogat_d_slot: &mut f64,
        var_cjogat_d_rv_slot: &mut f64,
        var_cjosti_d_slot: &mut f64,
        var_cjosti_d_rv_slot: &mut f64,
        var_deltaebot_d_slot: &mut f64,
        var_deltaebot_d_rv_slot: &mut f64,
        var_deltaegat_d_slot: &mut f64,
        var_deltaegat_d_rv_slot: &mut f64,
        var_deltaesti_d_slot: &mut f64,
        var_deltaesti_d_rv_slot: &mut f64,
        var_fbbtbot_d_slot: &mut f64,
        var_fbbtbot_d_rv_slot: &mut f64,
        var_fbbtgat_d_slot: &mut f64,
        var_fbbtgat_d_dn5_slot: &mut f64,
        var_fbbtgat_d_dn6_slot: &mut f64,
        var_fbbtgat_d_dn7_slot: &mut f64,
        var_fbbtgat_d_dn8_slot: &mut f64,
        var_fbbtgat_d_rv_slot: &mut f64,
        var_fbbtsti_d_slot: &mut f64,
        var_fbbtsti_d_rv_slot: &mut f64,
        var_ftdbot_d_slot: &mut f64,
        var_ftdbot_d_rv_slot: &mut f64,
        var_ftdgat2nd_d_slot: &mut f64,
        var_ftdgat2nd_d_rv_slot: &mut f64,
        var_ftdgat_d_slot: &mut f64,
        var_ftdgat_d_rv_slot: &mut f64,
        var_ftdsti_d_slot: &mut f64,
        var_ftdsti_d_rv_slot: &mut f64,
        var_guard28_slot: &mut f64,
        var_guard28_rv_slot: &mut f64,
        var_idsatbot_d_slot: &mut f64,
        var_idsatbot_d_rv_slot: &mut f64,
        var_idsatgat_d_slot: &mut f64,
        var_idsatgat_d_rv_slot: &mut f64,
        var_idsatsti_d_slot: &mut f64,
        var_idsatsti_d_rv_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_invnf_rv_slot: &mut f64,
        var_l_i_slot: &mut f64,
        var_l_i_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_nf_i_slot: &mut f64,
        var_nf_i_rv_slot: &mut f64,
        var_phigdbot_d_slot: &mut f64,
        var_phigdbot_d_rv_slot: &mut f64,
        var_phigdgat2nd_d_slot: &mut f64,
        var_phigdgat2nd_d_rv_slot: &mut f64,
        var_phigdgat_d_slot: &mut f64,
        var_phigdgat_d_rv_slot: &mut f64,
        var_phigdsti_d_slot: &mut f64,
        var_phigdsti_d_rv_slot: &mut f64,
        var_qpref2bot_d_slot: &mut f64,
        var_qpref2bot_d_rv_slot: &mut f64,
        var_qpref2gat2nd_slot: &mut f64,
        var_qpref2gat2nd_d_slot: &mut f64,
        var_qpref2gat2nd_d_rv_slot: &mut f64,
        var_qpref2gat2nd_rv_slot: &mut f64,
        var_qpref2gat_d_slot: &mut f64,
        var_qpref2gat_d_rv_slot: &mut f64,
        var_qpref2sti_d_slot: &mut f64,
        var_qpref2sti_d_rv_slot: &mut f64,
        var_qprefbot_d_slot: &mut f64,
        var_qprefbot_d_rv_slot: &mut f64,
        var_qprefgat2nd_slot: &mut f64,
        var_qprefgat2nd_d_slot: &mut f64,
        var_qprefgat2nd_d_rv_slot: &mut f64,
        var_qprefgat2nd_rv_slot: &mut f64,
        var_qprefgat_d_slot: &mut f64,
        var_qprefgat_d_rv_slot: &mut f64,
        var_qprefsti_d_slot: &mut f64,
        var_qprefsti_d_rv_slot: &mut f64,
        var_sa_i_slot: &mut f64,
        var_sa_i_rv_slot: &mut f64,
        var_sb_i_slot: &mut f64,
        var_sb_i_rv_slot: &mut f64,
        var_sc_i_slot: &mut f64,
        var_sc_i_rv_slot: &mut f64,
        var_sd_i_slot: &mut f64,
        var_sd_i_rv_slot: &mut f64,
        var_ubibot_d_slot: &mut f64,
        var_ubibot_d_rv_slot: &mut f64,
        var_ubigat2nd_slot: &mut f64,
        var_ubigat2nd_d_slot: &mut f64,
        var_ubigat2nd_d_rv_slot: &mut f64,
        var_ubigat2nd_rv_slot: &mut f64,
        var_ubigat_d_slot: &mut f64,
        var_ubigat_d_rv_slot: &mut f64,
        var_ubisti_d_slot: &mut f64,
        var_ubisti_d_rv_slot: &mut f64,
        var_vbibot_d_slot: &mut f64,
        var_vbibot_d_rv_slot: &mut f64,
        var_vbigat2nd_slot: &mut f64,
        var_vbigat2nd_d_slot: &mut f64,
        var_vbigat2nd_d_rv_slot: &mut f64,
        var_vbigat2nd_rv_slot: &mut f64,
        var_vbigat_d_slot: &mut f64,
        var_vbigat_d_rv_slot: &mut f64,
        var_vbiinvbot_d_slot: &mut f64,
        var_vbiinvbot_d_rv_slot: &mut f64,
        var_vbiinvgat2nd_slot: &mut f64,
        var_vbiinvgat2nd_d_slot: &mut f64,
        var_vbiinvgat2nd_d_rv_slot: &mut f64,
        var_vbiinvgat2nd_rv_slot: &mut f64,
        var_vbiinvgat_d_slot: &mut f64,
        var_vbiinvgat_d_rv_slot: &mut f64,
        var_vbiinvsti_d_slot: &mut f64,
        var_vbiinvsti_d_rv_slot: &mut f64,
        var_vbisti_d_slot: &mut f64,
        var_vbisti_d_rv_slot: &mut f64,
        var_w_i_slot: &mut f64,
        var_w_i_rv_slot: &mut f64,
        var_we_slot: &mut f64,
        var_we_rv_slot: &mut f64,
    ) {
        let mut var_atatbot_d: f64 = *var_atatbot_d_slot;
        let mut var_atatbot_d_rv: f64 = *var_atatbot_d_rv_slot;
        let mut var_atatgat_d: f64 = *var_atatgat_d_slot;
        let mut var_atatgat_d_rv: f64 = *var_atatgat_d_rv_slot;
        let mut var_atatsti_d: f64 = *var_atatsti_d_slot;
        let mut var_atatsti_d_rv: f64 = *var_atatsti_d_rv_slot;
        let mut var_btatpartbot_d: f64 = *var_btatpartbot_d_slot;
        let mut var_btatpartbot_d_rv: f64 = *var_btatpartbot_d_rv_slot;
        let mut var_btatpartgat_d: f64 = *var_btatpartgat_d_slot;
        let mut var_btatpartgat_d_rv: f64 = *var_btatpartgat_d_rv_slot;
        let mut var_btatpartsti_d: f64 = *var_btatpartsti_d_slot;
        let mut var_btatpartsti_d_rv: f64 = *var_btatpartsti_d_rv_slot;
        let mut var_cjobot_d: f64 = *var_cjobot_d_slot;
        let mut var_cjobot_d_rv: f64 = *var_cjobot_d_rv_slot;
        let mut var_cjogat2nd: f64 = *var_cjogat2nd_slot;
        let mut var_cjogat2nd_d: f64 = *var_cjogat2nd_d_slot;
        let mut var_cjogat2nd_d_rv: f64 = *var_cjogat2nd_d_rv_slot;
        let mut var_cjogat2nd_rv: f64 = *var_cjogat2nd_rv_slot;
        let mut var_cjogat_d: f64 = *var_cjogat_d_slot;
        let mut var_cjogat_d_rv: f64 = *var_cjogat_d_rv_slot;
        let mut var_cjosti_d: f64 = *var_cjosti_d_slot;
        let mut var_cjosti_d_rv: f64 = *var_cjosti_d_rv_slot;
        let mut var_deltaebot_d: f64 = *var_deltaebot_d_slot;
        let mut var_deltaebot_d_rv: f64 = *var_deltaebot_d_rv_slot;
        let mut var_deltaegat_d: f64 = *var_deltaegat_d_slot;
        let mut var_deltaegat_d_rv: f64 = *var_deltaegat_d_rv_slot;
        let mut var_deltaesti_d: f64 = *var_deltaesti_d_slot;
        let mut var_deltaesti_d_rv: f64 = *var_deltaesti_d_rv_slot;
        let mut var_fbbtbot_d: f64 = *var_fbbtbot_d_slot;
        let mut var_fbbtbot_d_rv: f64 = *var_fbbtbot_d_rv_slot;
        let mut var_fbbtgat_d: f64 = *var_fbbtgat_d_slot;
        let mut var_fbbtgat_d_dn5: f64 = *var_fbbtgat_d_dn5_slot;
        let mut var_fbbtgat_d_dn6: f64 = *var_fbbtgat_d_dn6_slot;
        let mut var_fbbtgat_d_dn7: f64 = *var_fbbtgat_d_dn7_slot;
        let mut var_fbbtgat_d_dn8: f64 = *var_fbbtgat_d_dn8_slot;
        let mut var_fbbtgat_d_rv: f64 = *var_fbbtgat_d_rv_slot;
        let mut var_fbbtsti_d: f64 = *var_fbbtsti_d_slot;
        let mut var_fbbtsti_d_rv: f64 = *var_fbbtsti_d_rv_slot;
        let mut var_ftdbot_d: f64 = *var_ftdbot_d_slot;
        let mut var_ftdbot_d_rv: f64 = *var_ftdbot_d_rv_slot;
        let mut var_ftdgat2nd_d: f64 = *var_ftdgat2nd_d_slot;
        let mut var_ftdgat2nd_d_rv: f64 = *var_ftdgat2nd_d_rv_slot;
        let mut var_ftdgat_d: f64 = *var_ftdgat_d_slot;
        let mut var_ftdgat_d_rv: f64 = *var_ftdgat_d_rv_slot;
        let mut var_ftdsti_d: f64 = *var_ftdsti_d_slot;
        let mut var_ftdsti_d_rv: f64 = *var_ftdsti_d_rv_slot;
        let mut var_guard28: f64 = *var_guard28_slot;
        let mut var_guard28_rv: f64 = *var_guard28_rv_slot;
        let mut var_idsatbot_d: f64 = *var_idsatbot_d_slot;
        let mut var_idsatbot_d_rv: f64 = *var_idsatbot_d_rv_slot;
        let mut var_idsatgat_d: f64 = *var_idsatgat_d_slot;
        let mut var_idsatgat_d_rv: f64 = *var_idsatgat_d_rv_slot;
        let mut var_idsatsti_d: f64 = *var_idsatsti_d_slot;
        let mut var_idsatsti_d_rv: f64 = *var_idsatsti_d_rv_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_invnf_rv: f64 = *var_invnf_rv_slot;
        let mut var_l_i: f64 = *var_l_i_slot;
        let mut var_l_i_rv: f64 = *var_l_i_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_nf_i: f64 = *var_nf_i_slot;
        let mut var_nf_i_rv: f64 = *var_nf_i_rv_slot;
        let mut var_phigdbot_d: f64 = *var_phigdbot_d_slot;
        let mut var_phigdbot_d_rv: f64 = *var_phigdbot_d_rv_slot;
        let mut var_phigdgat2nd_d: f64 = *var_phigdgat2nd_d_slot;
        let mut var_phigdgat2nd_d_rv: f64 = *var_phigdgat2nd_d_rv_slot;
        let mut var_phigdgat_d: f64 = *var_phigdgat_d_slot;
        let mut var_phigdgat_d_rv: f64 = *var_phigdgat_d_rv_slot;
        let mut var_phigdsti_d: f64 = *var_phigdsti_d_slot;
        let mut var_phigdsti_d_rv: f64 = *var_phigdsti_d_rv_slot;
        let mut var_qpref2bot_d: f64 = *var_qpref2bot_d_slot;
        let mut var_qpref2bot_d_rv: f64 = *var_qpref2bot_d_rv_slot;
        let mut var_qpref2gat2nd: f64 = *var_qpref2gat2nd_slot;
        let mut var_qpref2gat2nd_d: f64 = *var_qpref2gat2nd_d_slot;
        let mut var_qpref2gat2nd_d_rv: f64 = *var_qpref2gat2nd_d_rv_slot;
        let mut var_qpref2gat2nd_rv: f64 = *var_qpref2gat2nd_rv_slot;
        let mut var_qpref2gat_d: f64 = *var_qpref2gat_d_slot;
        let mut var_qpref2gat_d_rv: f64 = *var_qpref2gat_d_rv_slot;
        let mut var_qpref2sti_d: f64 = *var_qpref2sti_d_slot;
        let mut var_qpref2sti_d_rv: f64 = *var_qpref2sti_d_rv_slot;
        let mut var_qprefbot_d: f64 = *var_qprefbot_d_slot;
        let mut var_qprefbot_d_rv: f64 = *var_qprefbot_d_rv_slot;
        let mut var_qprefgat2nd: f64 = *var_qprefgat2nd_slot;
        let mut var_qprefgat2nd_d: f64 = *var_qprefgat2nd_d_slot;
        let mut var_qprefgat2nd_d_rv: f64 = *var_qprefgat2nd_d_rv_slot;
        let mut var_qprefgat2nd_rv: f64 = *var_qprefgat2nd_rv_slot;
        let mut var_qprefgat_d: f64 = *var_qprefgat_d_slot;
        let mut var_qprefgat_d_rv: f64 = *var_qprefgat_d_rv_slot;
        let mut var_qprefsti_d: f64 = *var_qprefsti_d_slot;
        let mut var_qprefsti_d_rv: f64 = *var_qprefsti_d_rv_slot;
        let mut var_sa_i: f64 = *var_sa_i_slot;
        let mut var_sa_i_rv: f64 = *var_sa_i_rv_slot;
        let mut var_sb_i: f64 = *var_sb_i_slot;
        let mut var_sb_i_rv: f64 = *var_sb_i_rv_slot;
        let mut var_sc_i: f64 = *var_sc_i_slot;
        let mut var_sc_i_rv: f64 = *var_sc_i_rv_slot;
        let mut var_sd_i: f64 = *var_sd_i_slot;
        let mut var_sd_i_rv: f64 = *var_sd_i_rv_slot;
        let mut var_ubibot_d: f64 = *var_ubibot_d_slot;
        let mut var_ubibot_d_rv: f64 = *var_ubibot_d_rv_slot;
        let mut var_ubigat2nd: f64 = *var_ubigat2nd_slot;
        let mut var_ubigat2nd_d: f64 = *var_ubigat2nd_d_slot;
        let mut var_ubigat2nd_d_rv: f64 = *var_ubigat2nd_d_rv_slot;
        let mut var_ubigat2nd_rv: f64 = *var_ubigat2nd_rv_slot;
        let mut var_ubigat_d: f64 = *var_ubigat_d_slot;
        let mut var_ubigat_d_rv: f64 = *var_ubigat_d_rv_slot;
        let mut var_ubisti_d: f64 = *var_ubisti_d_slot;
        let mut var_ubisti_d_rv: f64 = *var_ubisti_d_rv_slot;
        let mut var_vbibot_d: f64 = *var_vbibot_d_slot;
        let mut var_vbibot_d_rv: f64 = *var_vbibot_d_rv_slot;
        let mut var_vbigat2nd: f64 = *var_vbigat2nd_slot;
        let mut var_vbigat2nd_d: f64 = *var_vbigat2nd_d_slot;
        let mut var_vbigat2nd_d_rv: f64 = *var_vbigat2nd_d_rv_slot;
        let mut var_vbigat2nd_rv: f64 = *var_vbigat2nd_rv_slot;
        let mut var_vbigat_d: f64 = *var_vbigat_d_slot;
        let mut var_vbigat_d_rv: f64 = *var_vbigat_d_rv_slot;
        let mut var_vbiinvbot_d: f64 = *var_vbiinvbot_d_slot;
        let mut var_vbiinvbot_d_rv: f64 = *var_vbiinvbot_d_rv_slot;
        let mut var_vbiinvgat2nd: f64 = *var_vbiinvgat2nd_slot;
        let mut var_vbiinvgat2nd_d: f64 = *var_vbiinvgat2nd_d_slot;
        let mut var_vbiinvgat2nd_d_rv: f64 = *var_vbiinvgat2nd_d_rv_slot;
        let mut var_vbiinvgat2nd_rv: f64 = *var_vbiinvgat2nd_rv_slot;
        let mut var_vbiinvgat_d: f64 = *var_vbiinvgat_d_slot;
        let mut var_vbiinvgat_d_rv: f64 = *var_vbiinvgat_d_rv_slot;
        let mut var_vbiinvsti_d: f64 = *var_vbiinvsti_d_slot;
        let mut var_vbiinvsti_d_rv: f64 = *var_vbiinvsti_d_rv_slot;
        let mut var_vbisti_d: f64 = *var_vbisti_d_slot;
        let mut var_vbisti_d_rv: f64 = *var_vbisti_d_rv_slot;
        let mut var_w_i: f64 = *var_w_i_slot;
        let mut var_w_i_rv: f64 = *var_w_i_rv_slot;
        let mut var_we: f64 = *var_we_slot;
        let mut var_we_rv: f64 = *var_we_rv_slot;

        let (assign2710_e2950,) = {
    if (var_guard27 != 0.0) {
        let assign2710_e2941: f64 = (var_vbirgat2nd * var_auxt);
        let assign2710_e2944: f64 = (2.0 * var_phitd);
        let assign2710_e2946: f64 = (var_ftdgat2nd).ln();
        let assign2710_e2947: f64 = (assign2710_e2944 * assign2710_e2946);
        let assign2710_e2948: f64 = (assign2710_e2941 - assign2710_e2947);
        (assign2710_e2948,)
    } else {
        (var_ubigat2nd,)
    }
};
        var_ubigat2nd = assign2710_e2950;
        var_ubigat2nd_rv = 0.0;

        let (assign2720_e2966,) = {
    if (var_guard27 != 0.0) {
        let assign2720_e2957: f64 = (0.05 - var_ubigat2nd);
        let assign2720_e2959: f64 = (assign2720_e2957 * var_phitdinv);
        let assign2720_e2960: f64 = (assign2720_e2959).exp();
        let assign2720_e2961: f64 = (1.0 + assign2720_e2960);
        let assign2720_e2962: f64 = (assign2720_e2961).ln();
        let assign2720_e2963: f64 = (var_phitd * assign2720_e2962);
        let assign2720_e2964: f64 = (var_ubigat2nd + assign2720_e2963);
        (assign2720_e2964,)
    } else {
        (var_vbigat2nd,)
    }
};
        var_vbigat2nd = assign2720_e2966;
        var_vbigat2nd_rv = 0.0;

        let (assign2730_e2972,) = {
    if (var_guard27 != 0.0) {
        let assign2730_e2970: f64 = (1.0 / var_vbigat2nd);
        (assign2730_e2970,)
    } else {
        (var_vbiinvgat2nd,)
    }
};
        var_vbiinvgat2nd = assign2730_e2972;
        var_vbiinvgat2nd_rv = 0.0;

        let (assign2740_e2982,) = {
    if (var_guard27 != 0.0) {
        let assign2740_e2977: f64 = (var_vbirgat2nd * var_vbiinvgat2nd);
        let assign2740_e2979: f64 = (assign2740_e2977).powf(var_pgat2nd);
        let assign2740_e2980: f64 = (var_cjorgat2nd * assign2740_e2979);
        (assign2740_e2980,)
    } else {
        (var_cjogat2nd,)
    }
};
        var_cjogat2nd = assign2740_e2982;
        var_cjogat2nd_rv = 0.0;

        let (assign2750_e2990,) = {
    if (var_guard27 != 0.0) {
        let assign2750_e2986: f64 = (var_cjogat2nd * var_vbigat2nd);
        let assign2750_e2988: f64 = (assign2750_e2986 * var_one_over_one_minus_pgat2nd);
        (assign2750_e2988,)
    } else {
        (var_qprefgat2nd,)
    }
};
        var_qprefgat2nd = assign2750_e2990;
        var_qprefgat2nd_rv = 0.0;

        let (assign2760_e2996,) = {
    if (var_guard27 != 0.0) {
        let assign2760_e2994: f64 = (2.0 * var_cjogat2nd);
        (assign2760_e2994,)
    } else {
        (var_qpref2gat2nd,)
    }
};
        var_qpref2gat2nd = assign2760_e2996;
        var_qpref2gat2nd_rv = 0.0;

        let assign2770_e2999: f64 = (var_phigbotd_i + var_deltaphigd);
        var_phigdbot_d = assign2770_e2999;
        var_phigdbot_d_rv = 0.0;

        let assign2780_e3002: f64 = (var_phigstid_i + var_deltaphigd);
        var_phigdsti_d = assign2780_e3002;
        var_phigdsti_d_rv = 0.0;

        let assign2790_e3005: f64 = (var_phiggatd_i + var_deltaphigd);
        var_phigdgat_d = assign2790_e3005;
        var_phigdgat_d_rv = 0.0;

        let assign2800_e3008: f64 = (var_auxt).powf(1.5);
        let assign2800_e3012: f64 = (var_phigrbot_d * var_phitrinv);
        let assign2800_e3015: f64 = (var_phigdbot_d * var_phitdinv);
        let assign2800_e3016: f64 = (assign2800_e3012 - assign2800_e3015);
        let assign2800_e3017: f64 = (0.5 * assign2800_e3016);
        let assign2800_e3018: f64 = (assign2800_e3017).exp();
        let assign2800_e3019: f64 = (assign2800_e3008 * assign2800_e3018);
        var_ftdbot_d = assign2800_e3019;
        var_ftdbot_d_rv = 0.0;

        let assign2810_e3022: f64 = (var_auxt).powf(1.5);
        let assign2810_e3026: f64 = (var_phigrsti_d * var_phitrinv);
        let assign2810_e3029: f64 = (var_phigdsti_d * var_phitdinv);
        let assign2810_e3030: f64 = (assign2810_e3026 - assign2810_e3029);
        let assign2810_e3031: f64 = (0.5 * assign2810_e3030);
        let assign2810_e3032: f64 = (assign2810_e3031).exp();
        let assign2810_e3033: f64 = (assign2810_e3022 * assign2810_e3032);
        var_ftdsti_d = assign2810_e3033;
        var_ftdsti_d_rv = 0.0;

        let assign2820_e3036: f64 = (var_auxt).powf(1.5);
        let assign2820_e3040: f64 = (var_phigrgat_d * var_phitrinv);
        let assign2820_e3043: f64 = (var_phigdgat_d * var_phitdinv);
        let assign2820_e3044: f64 = (assign2820_e3040 - assign2820_e3043);
        let assign2820_e3045: f64 = (0.5 * assign2820_e3044);
        let assign2820_e3046: f64 = (assign2820_e3045).exp();
        let assign2820_e3047: f64 = (assign2820_e3036 * assign2820_e3046);
        var_ftdgat_d = assign2820_e3047;
        var_ftdgat_d_rv = 0.0;

        let assign2830_e3050: f64 = (var_idsatrbotd_i * var_ftdbot_d);
        let assign2830_e3052: f64 = (assign2830_e3050 * var_ftdbot_d);
        var_idsatbot_d = assign2830_e3052;
        var_idsatbot_d_rv = 0.0;

        let assign2840_e3055: f64 = (var_idsatrstid_i * var_ftdsti_d);
        let assign2840_e3057: f64 = (assign2840_e3055 * var_ftdsti_d);
        var_idsatsti_d = assign2840_e3057;
        var_idsatsti_d_rv = 0.0;

        let assign2850_e3060: f64 = (var_idsatrgatd_i * var_ftdgat_d);
        let assign2850_e3062: f64 = (assign2850_e3060 * var_ftdgat_d);
        var_idsatgat_d = assign2850_e3062;
        var_idsatgat_d_rv = 0.0;

        let assign2860_e3065: f64 = (var_vbirbotd_i * var_auxt);
        let assign2860_e3068: f64 = (2.0 * var_phitd);
        let assign2860_e3070: f64 = (var_ftdbot_d).ln();
        let assign2860_e3071: f64 = (assign2860_e3068 * assign2860_e3070);
        let assign2860_e3072: f64 = (assign2860_e3065 - assign2860_e3071);
        var_ubibot_d = assign2860_e3072;
        var_ubibot_d_rv = 0.0;

        let assign2870_e3075: f64 = (var_vbirstid_i * var_auxt);
        let assign2870_e3078: f64 = (2.0 * var_phitd);
        let assign2870_e3080: f64 = (var_ftdsti_d).ln();
        let assign2870_e3081: f64 = (assign2870_e3078 * assign2870_e3080);
        let assign2870_e3082: f64 = (assign2870_e3075 - assign2870_e3081);
        var_ubisti_d = assign2870_e3082;
        var_ubisti_d_rv = 0.0;

        let assign2880_e3085: f64 = (var_vbirgatd_i * var_auxt);
        let assign2880_e3088: f64 = (2.0 * var_phitd);
        let assign2880_e3090: f64 = (var_ftdgat_d).ln();
        let assign2880_e3091: f64 = (assign2880_e3088 * assign2880_e3090);
        let assign2880_e3092: f64 = (assign2880_e3085 - assign2880_e3091);
        var_ubigat_d = assign2880_e3092;
        var_ubigat_d_rv = 0.0;

        let assign2890_e3098: f64 = (0.05 - var_ubibot_d);
        let assign2890_e3100: f64 = (assign2890_e3098 * var_phitdinv);
        let assign2890_e3101: f64 = (assign2890_e3100).exp();
        let assign2890_e3102: f64 = (1.0 + assign2890_e3101);
        let assign2890_e3103: f64 = (assign2890_e3102).ln();
        let assign2890_e3104: f64 = (var_phitd * assign2890_e3103);
        let assign2890_e3105: f64 = (var_ubibot_d + assign2890_e3104);
        var_vbibot_d = assign2890_e3105;
        var_vbibot_d_rv = 0.0;

        let assign2900_e3111: f64 = (0.05 - var_ubisti_d);
        let assign2900_e3113: f64 = (assign2900_e3111 * var_phitdinv);
        let assign2900_e3114: f64 = (assign2900_e3113).exp();
        let assign2900_e3115: f64 = (1.0 + assign2900_e3114);
        let assign2900_e3116: f64 = (assign2900_e3115).ln();
        let assign2900_e3117: f64 = (var_phitd * assign2900_e3116);
        let assign2900_e3118: f64 = (var_ubisti_d + assign2900_e3117);
        var_vbisti_d = assign2900_e3118;
        var_vbisti_d_rv = 0.0;

        let assign2910_e3124: f64 = (0.05 - var_ubigat_d);
        let assign2910_e3126: f64 = (assign2910_e3124 * var_phitdinv);
        let assign2910_e3127: f64 = (assign2910_e3126).exp();
        let assign2910_e3128: f64 = (1.0 + assign2910_e3127);
        let assign2910_e3129: f64 = (assign2910_e3128).ln();
        let assign2910_e3130: f64 = (var_phitd * assign2910_e3129);
        let assign2910_e3131: f64 = (var_ubigat_d + assign2910_e3130);
        var_vbigat_d = assign2910_e3131;
        var_vbigat_d_rv = 0.0;

        let assign2920_e3134: f64 = (1.0 / var_vbibot_d);
        var_vbiinvbot_d = assign2920_e3134;
        var_vbiinvbot_d_rv = 0.0;

        let assign2930_e3137: f64 = (1.0 / var_vbisti_d);
        var_vbiinvsti_d = assign2930_e3137;
        var_vbiinvsti_d_rv = 0.0;

        let assign2940_e3140: f64 = (1.0 / var_vbigat_d);
        var_vbiinvgat_d = assign2940_e3140;
        var_vbiinvgat_d_rv = 0.0;

        let assign2950_e3144: f64 = (var_vbirbotd_i * var_vbiinvbot_d);
        let assign2950_e3146: f64 = (assign2950_e3144).powf(var_pbotd_i);
        let assign2950_e3147: f64 = (var_cjorbotd_i * assign2950_e3146);
        var_cjobot_d = assign2950_e3147;
        var_cjobot_d_rv = 0.0;

        let assign2960_e3151: f64 = (var_vbirstid_i * var_vbiinvsti_d);
        let assign2960_e3153: f64 = (assign2960_e3151).powf(var_pstid_i);
        let assign2960_e3154: f64 = (var_cjorstid_i * assign2960_e3153);
        var_cjosti_d = assign2960_e3154;
        var_cjosti_d_rv = 0.0;

        let assign2970_e3158: f64 = (var_vbirgatd_i * var_vbiinvgat_d);
        let assign2970_e3160: f64 = (assign2970_e3158).powf(var_pgatd_i);
        let assign2970_e3161: f64 = (var_cjorgatd_i * assign2970_e3160);
        var_cjogat_d = assign2970_e3161;
        var_cjogat_d_rv = 0.0;

        let assign2980_e3164: f64 = (var_cjobot_d * var_vbibot_d);
        let assign2980_e3166: f64 = (assign2980_e3164 * var_one_over_one_minus_pbot_d);
        var_qprefbot_d = assign2980_e3166;
        var_qprefbot_d_rv = 0.0;

        let assign2990_e3169: f64 = (var_cjosti_d * var_vbisti_d);
        let assign2990_e3171: f64 = (assign2990_e3169 * var_one_over_one_minus_psti_d);
        var_qprefsti_d = assign2990_e3171;
        var_qprefsti_d_rv = 0.0;

        let assign3000_e3174: f64 = (var_cjogat_d * var_vbigat_d);
        let assign3000_e3176: f64 = (assign3000_e3174 * var_one_over_one_minus_pgat_d);
        var_qprefgat_d = assign3000_e3176;
        var_qprefgat_d_rv = 0.0;

        let assign3010_e3179: f64 = (2.0 * var_cjobot_d);
        var_qpref2bot_d = assign3010_e3179;
        var_qpref2bot_d_rv = 0.0;

        let assign3020_e3182: f64 = (2.0 * var_cjosti_d);
        var_qpref2sti_d = assign3020_e3182;
        var_qpref2sti_d_rv = 0.0;

        let assign3030_e3185: f64 = (2.0 * var_cjogat_d);
        var_qpref2gat_d = assign3030_e3185;
        var_qpref2gat_d_rv = 0.0;

        let assign3040_e3188: f64 = (0.5 * var_phigdbot_d);
        let assign3040_e3190: f64 = (assign3040_e3188).max(var_phitd);
        var_deltaebot_d = assign3040_e3190;
        var_deltaebot_d_rv = 0.0;

        let assign3050_e3193: f64 = (0.5 * var_phigdsti_d);
        let assign3050_e3195: f64 = (assign3050_e3193).max(var_phitd);
        var_deltaesti_d = assign3050_e3195;
        var_deltaesti_d_rv = 0.0;

        let assign3060_e3198: f64 = (0.5 * var_phigdgat_d);
        let assign3060_e3200: f64 = (assign3060_e3198).max(var_phitd);
        var_deltaegat_d = assign3060_e3200;
        var_deltaegat_d_rv = 0.0;

        let assign3070_e3203: f64 = (var_deltaebot_d * var_phitdinv);
        var_atatbot_d = assign3070_e3203;
        var_atatbot_d_rv = 0.0;

        let assign3080_e3206: f64 = (var_deltaesti_d * var_phitdinv);
        var_atatsti_d = assign3080_e3206;
        var_atatsti_d_rv = 0.0;

        let assign3090_e3209: f64 = (var_deltaegat_d * var_phitdinv);
        var_atatgat_d = assign3090_e3209;
        var_atatgat_d_rv = 0.0;

        let assign3100_e3212: f64 = (32.0 * var_mefftatbotd_i);
        let assign3100_e3214: f64 = (assign3100_e3212 * 9.1093826e-31);
        let assign3100_e3216: f64 = (assign3100_e3214 * 1.6021918e-19);
        let assign3100_e3219: f64 = (var_deltaebot_d * var_deltaebot_d);
        let assign3100_e3221: f64 = (assign3100_e3219 * var_deltaebot_d);
        let assign3100_e3222: f64 = (assign3100_e3216 * assign3100_e3221);
        let assign3100_e3223: f64 = (assign3100_e3222).sqrt();
        let assign3100_e3226: f64 = (3.0 * 1.05457168e-34);
        let assign3100_e3227: f64 = (assign3100_e3223 / assign3100_e3226);
        var_btatpartbot_d = assign3100_e3227;
        var_btatpartbot_d_rv = 0.0;

        let assign3110_e3230: f64 = (32.0 * var_mefftatstid_i);
        let assign3110_e3232: f64 = (assign3110_e3230 * 9.1093826e-31);
        let assign3110_e3234: f64 = (assign3110_e3232 * 1.6021918e-19);
        let assign3110_e3237: f64 = (var_deltaesti_d * var_deltaesti_d);
        let assign3110_e3239: f64 = (assign3110_e3237 * var_deltaesti_d);
        let assign3110_e3240: f64 = (assign3110_e3234 * assign3110_e3239);
        let assign3110_e3241: f64 = (assign3110_e3240).sqrt();
        let assign3110_e3244: f64 = (3.0 * 1.05457168e-34);
        let assign3110_e3245: f64 = (assign3110_e3241 / assign3110_e3244);
        var_btatpartsti_d = assign3110_e3245;
        var_btatpartsti_d_rv = 0.0;

        let assign3120_e3248: f64 = (32.0 * var_mefftatgatd_i);
        let assign3120_e3250: f64 = (assign3120_e3248 * 9.1093826e-31);
        let assign3120_e3252: f64 = (assign3120_e3250 * 1.6021918e-19);
        let assign3120_e3255: f64 = (var_deltaegat_d * var_deltaegat_d);
        let assign3120_e3257: f64 = (assign3120_e3255 * var_deltaegat_d);
        let assign3120_e3258: f64 = (assign3120_e3252 * assign3120_e3257);
        let assign3120_e3259: f64 = (assign3120_e3258).sqrt();
        let assign3120_e3262: f64 = (3.0 * 1.05457168e-34);
        let assign3120_e3263: f64 = (assign3120_e3259 / assign3120_e3262);
        var_btatpartgat_d = assign3120_e3263;
        var_btatpartgat_d_rv = 0.0;

        let assign3130_e3269: f64 = (var_tkd_1 - var_tkr_1);
        let assign3130_e3270: f64 = (var_stfbbtbotd_i * assign3130_e3269);
        let assign3130_e3271: f64 = (1.0 + assign3130_e3270);
        let assign3130_e3272: f64 = (var_fbbtrbotd_i * assign3130_e3271);
        var_fbbtbot_d = assign3130_e3272;
        var_fbbtbot_d_rv = 0.0;

        let assign3140_e3278: f64 = (var_tkd_1 - var_tkr_1);
        let assign3140_e3279: f64 = (var_stfbbtstid_i * assign3140_e3278);
        let assign3140_e3280: f64 = (1.0 + assign3140_e3279);
        let assign3140_e3281: f64 = (var_fbbtrstid_i * assign3140_e3280);
        var_fbbtsti_d = assign3140_e3281;
        var_fbbtsti_d_rv = 0.0;

        let assign3150_e3287: f64 = (var_tkd_1 - var_tkr_1);
        let assign3150_e3288: f64 = (var_stfbbtgatd_i * assign3150_e3287);
        let assign3150_e3289: f64 = (1.0 + assign3150_e3288);
        let assign3150_e3290: f64 = (var_fbbtrgatd_i * assign3150_e3289);
        var_fbbtgat_d = assign3150_e3290;
        var_fbbtgat_d_dn5 = 0.0;
        var_fbbtgat_d_dn6 = 0.0;
        var_fbbtgat_d_dn7 = 0.0;
        var_fbbtgat_d_dn8 = 0.0;
        var_fbbtgat_d_rv = 0.0;

        let (assign3160_e3296,) = {
    if (var_fbbtbot_d > 0.0) {
        (var_fbbtbot_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot_d = assign3160_e3296;
        var_fbbtbot_d_rv = 0.0;

        let (assign3170_e3302,) = {
    if (var_fbbtsti_d > 0.0) {
        (var_fbbtsti_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti_d = assign3170_e3302;
        var_fbbtsti_d_rv = 0.0;

        let (assign3180_e3308, assign3180_e3308_d_n5, assign3180_e3308_d_n6, assign3180_e3308_d_n7, assign3180_e3308_d_n8,) = {
    if (var_fbbtgat_d > 0.0) {
        (var_fbbtgat_d, var_fbbtgat_d_dn5, var_fbbtgat_d_dn6, var_fbbtgat_d_dn7, var_fbbtgat_d_dn8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat_d = assign3180_e3308;
        var_fbbtgat_d_dn5 = assign3180_e3308_d_n5;
        var_fbbtgat_d_dn6 = assign3180_e3308_d_n6;
        var_fbbtgat_d_dn7 = assign3180_e3308_d_n7;
        var_fbbtgat_d_dn8 = assign3180_e3308_d_n8;
        var_fbbtgat_d_rv = 0.0;

        let assign3190_e3311: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard28 = assign3190_e3311;
        var_guard28_rv = 0.0;

        let (assign3200_e3317,) = {
    if (var_guard28 != 0.0) {
        let assign3200_e3315: f64 = (var_phiggat2nd_d + var_deltaphigd);
        (assign3200_e3315,)
    } else {
        (var_phigdgat2nd_d,)
    }
};
        var_phigdgat2nd_d = assign3200_e3317;
        var_phigdgat2nd_d_rv = 0.0;

        let (assign3210_e3334,) = {
    if (var_guard28 != 0.0) {
        let assign3210_e3321: f64 = (var_auxt).powf(1.5);
        let assign3210_e3325: f64 = (var_phigrgat2nd_d * var_phitrinv);
        let assign3210_e3328: f64 = (var_phigdgat2nd_d * var_phitdinv);
        let assign3210_e3329: f64 = (assign3210_e3325 - assign3210_e3328);
        let assign3210_e3330: f64 = (0.5 * assign3210_e3329);
        let assign3210_e3331: f64 = (assign3210_e3330).exp();
        let assign3210_e3332: f64 = (assign3210_e3321 * assign3210_e3331);
        (assign3210_e3332,)
    } else {
        (var_ftdgat2nd_d,)
    }
};
        var_ftdgat2nd_d = assign3210_e3334;
        var_ftdgat2nd_d_rv = 0.0;

        let (assign3220_e3347,) = {
    if (var_guard28 != 0.0) {
        let assign3220_e3338: f64 = (var_vbirgat2nd_d * var_auxt);
        let assign3220_e3341: f64 = (2.0 * var_phitd);
        let assign3220_e3343: f64 = (var_ftdgat2nd_d).ln();
        let assign3220_e3344: f64 = (assign3220_e3341 * assign3220_e3343);
        let assign3220_e3345: f64 = (assign3220_e3338 - assign3220_e3344);
        (assign3220_e3345,)
    } else {
        (var_ubigat2nd_d,)
    }
};
        var_ubigat2nd_d = assign3220_e3347;
        var_ubigat2nd_d_rv = 0.0;

        let (assign3230_e3363,) = {
    if (var_guard28 != 0.0) {
        let assign3230_e3354: f64 = (0.05 - var_ubigat2nd_d);
        let assign3230_e3356: f64 = (assign3230_e3354 * var_phitdinv);
        let assign3230_e3357: f64 = (assign3230_e3356).exp();
        let assign3230_e3358: f64 = (1.0 + assign3230_e3357);
        let assign3230_e3359: f64 = (assign3230_e3358).ln();
        let assign3230_e3360: f64 = (var_phitd * assign3230_e3359);
        let assign3230_e3361: f64 = (var_ubigat2nd_d + assign3230_e3360);
        (assign3230_e3361,)
    } else {
        (var_vbigat2nd_d,)
    }
};
        var_vbigat2nd_d = assign3230_e3363;
        var_vbigat2nd_d_rv = 0.0;

        let (assign3240_e3369,) = {
    if (var_guard28 != 0.0) {
        let assign3240_e3367: f64 = (1.0 / var_vbigat2nd_d);
        (assign3240_e3367,)
    } else {
        (var_vbiinvgat2nd_d,)
    }
};
        var_vbiinvgat2nd_d = assign3240_e3369;
        var_vbiinvgat2nd_d_rv = 0.0;

        let (assign3250_e3379,) = {
    if (var_guard28 != 0.0) {
        let assign3250_e3374: f64 = (var_vbirgat2nd_d * var_vbiinvgat2nd_d);
        let assign3250_e3376: f64 = (assign3250_e3374).powf(var_pgat2nd_d);
        let assign3250_e3377: f64 = (var_cjorgat2nd_d * assign3250_e3376);
        (assign3250_e3377,)
    } else {
        (var_cjogat2nd_d,)
    }
};
        var_cjogat2nd_d = assign3250_e3379;
        var_cjogat2nd_d_rv = 0.0;

        let (assign3260_e3387,) = {
    if (var_guard28 != 0.0) {
        let assign3260_e3383: f64 = (var_cjogat2nd_d * var_vbigat2nd_d);
        let assign3260_e3385: f64 = (assign3260_e3383 * var_one_over_one_minus_pgat2nd_d);
        (assign3260_e3385,)
    } else {
        (var_qprefgat2nd_d,)
    }
};
        var_qprefgat2nd_d = assign3260_e3387;
        var_qprefgat2nd_d_rv = 0.0;

        let (assign3270_e3393,) = {
    if (var_guard28 != 0.0) {
        let assign3270_e3391: f64 = (2.0 * var_cjogat2nd_d);
        (assign3270_e3391,)
    } else {
        (var_qpref2gat2nd_d,)
    }
};
        var_qpref2gat2nd_d = assign3270_e3393;
        var_qpref2gat2nd_d_rv = 0.0;

        var_nf_i = 1.0;
        var_nf_i_rv = 0.0;

        var_invnf = 1.0;
        var_invnf_rv = 0.0;

        var_le = 0.0;
        var_le_rv = 0.0;

        var_we = 0.0;
        var_we_rv = 0.0;

        var_l_i = p.p0;
        var_l_i_rv = 0.0;

        var_w_i = p.p1;
        var_w_i_rv = 0.0;

        var_sa_i = p.p2;
        var_sa_i_rv = 0.0;

        var_sb_i = p.p3;
        var_sb_i_rv = 0.0;

        var_sd_i = p.p4;
        var_sd_i_rv = 0.0;

        var_sc_i = p.p8;
        var_sc_i_rv = 0.0;

        *var_atatbot_d_slot = var_atatbot_d;
        *var_atatbot_d_rv_slot = var_atatbot_d_rv;
        *var_atatgat_d_slot = var_atatgat_d;
        *var_atatgat_d_rv_slot = var_atatgat_d_rv;
        *var_atatsti_d_slot = var_atatsti_d;
        *var_atatsti_d_rv_slot = var_atatsti_d_rv;
        *var_btatpartbot_d_slot = var_btatpartbot_d;
        *var_btatpartbot_d_rv_slot = var_btatpartbot_d_rv;
        *var_btatpartgat_d_slot = var_btatpartgat_d;
        *var_btatpartgat_d_rv_slot = var_btatpartgat_d_rv;
        *var_btatpartsti_d_slot = var_btatpartsti_d;
        *var_btatpartsti_d_rv_slot = var_btatpartsti_d_rv;
        *var_cjobot_d_slot = var_cjobot_d;
        *var_cjobot_d_rv_slot = var_cjobot_d_rv;
        *var_cjogat2nd_slot = var_cjogat2nd;
        *var_cjogat2nd_d_slot = var_cjogat2nd_d;
        *var_cjogat2nd_d_rv_slot = var_cjogat2nd_d_rv;
        *var_cjogat2nd_rv_slot = var_cjogat2nd_rv;
        *var_cjogat_d_slot = var_cjogat_d;
        *var_cjogat_d_rv_slot = var_cjogat_d_rv;
        *var_cjosti_d_slot = var_cjosti_d;
        *var_cjosti_d_rv_slot = var_cjosti_d_rv;
        *var_deltaebot_d_slot = var_deltaebot_d;
        *var_deltaebot_d_rv_slot = var_deltaebot_d_rv;
        *var_deltaegat_d_slot = var_deltaegat_d;
        *var_deltaegat_d_rv_slot = var_deltaegat_d_rv;
        *var_deltaesti_d_slot = var_deltaesti_d;
        *var_deltaesti_d_rv_slot = var_deltaesti_d_rv;
        *var_fbbtbot_d_slot = var_fbbtbot_d;
        *var_fbbtbot_d_rv_slot = var_fbbtbot_d_rv;
        *var_fbbtgat_d_slot = var_fbbtgat_d;
        *var_fbbtgat_d_dn5_slot = var_fbbtgat_d_dn5;
        *var_fbbtgat_d_dn6_slot = var_fbbtgat_d_dn6;
        *var_fbbtgat_d_dn7_slot = var_fbbtgat_d_dn7;
        *var_fbbtgat_d_dn8_slot = var_fbbtgat_d_dn8;
        *var_fbbtgat_d_rv_slot = var_fbbtgat_d_rv;
        *var_fbbtsti_d_slot = var_fbbtsti_d;
        *var_fbbtsti_d_rv_slot = var_fbbtsti_d_rv;
        *var_ftdbot_d_slot = var_ftdbot_d;
        *var_ftdbot_d_rv_slot = var_ftdbot_d_rv;
        *var_ftdgat2nd_d_slot = var_ftdgat2nd_d;
        *var_ftdgat2nd_d_rv_slot = var_ftdgat2nd_d_rv;
        *var_ftdgat_d_slot = var_ftdgat_d;
        *var_ftdgat_d_rv_slot = var_ftdgat_d_rv;
        *var_ftdsti_d_slot = var_ftdsti_d;
        *var_ftdsti_d_rv_slot = var_ftdsti_d_rv;
        *var_guard28_slot = var_guard28;
        *var_guard28_rv_slot = var_guard28_rv;
        *var_idsatbot_d_slot = var_idsatbot_d;
        *var_idsatbot_d_rv_slot = var_idsatbot_d_rv;
        *var_idsatgat_d_slot = var_idsatgat_d;
        *var_idsatgat_d_rv_slot = var_idsatgat_d_rv;
        *var_idsatsti_d_slot = var_idsatsti_d;
        *var_idsatsti_d_rv_slot = var_idsatsti_d_rv;
        *var_invnf_slot = var_invnf;
        *var_invnf_rv_slot = var_invnf_rv;
        *var_l_i_slot = var_l_i;
        *var_l_i_rv_slot = var_l_i_rv;
        *var_le_slot = var_le;
        *var_le_rv_slot = var_le_rv;
        *var_nf_i_slot = var_nf_i;
        *var_nf_i_rv_slot = var_nf_i_rv;
        *var_phigdbot_d_slot = var_phigdbot_d;
        *var_phigdbot_d_rv_slot = var_phigdbot_d_rv;
        *var_phigdgat2nd_d_slot = var_phigdgat2nd_d;
        *var_phigdgat2nd_d_rv_slot = var_phigdgat2nd_d_rv;
        *var_phigdgat_d_slot = var_phigdgat_d;
        *var_phigdgat_d_rv_slot = var_phigdgat_d_rv;
        *var_phigdsti_d_slot = var_phigdsti_d;
        *var_phigdsti_d_rv_slot = var_phigdsti_d_rv;
        *var_qpref2bot_d_slot = var_qpref2bot_d;
        *var_qpref2bot_d_rv_slot = var_qpref2bot_d_rv;
        *var_qpref2gat2nd_slot = var_qpref2gat2nd;
        *var_qpref2gat2nd_d_slot = var_qpref2gat2nd_d;
        *var_qpref2gat2nd_d_rv_slot = var_qpref2gat2nd_d_rv;
        *var_qpref2gat2nd_rv_slot = var_qpref2gat2nd_rv;
        *var_qpref2gat_d_slot = var_qpref2gat_d;
        *var_qpref2gat_d_rv_slot = var_qpref2gat_d_rv;
        *var_qpref2sti_d_slot = var_qpref2sti_d;
        *var_qpref2sti_d_rv_slot = var_qpref2sti_d_rv;
        *var_qprefbot_d_slot = var_qprefbot_d;
        *var_qprefbot_d_rv_slot = var_qprefbot_d_rv;
        *var_qprefgat2nd_slot = var_qprefgat2nd;
        *var_qprefgat2nd_d_slot = var_qprefgat2nd_d;
        *var_qprefgat2nd_d_rv_slot = var_qprefgat2nd_d_rv;
        *var_qprefgat2nd_rv_slot = var_qprefgat2nd_rv;
        *var_qprefgat_d_slot = var_qprefgat_d;
        *var_qprefgat_d_rv_slot = var_qprefgat_d_rv;
        *var_qprefsti_d_slot = var_qprefsti_d;
        *var_qprefsti_d_rv_slot = var_qprefsti_d_rv;
        *var_sa_i_slot = var_sa_i;
        *var_sa_i_rv_slot = var_sa_i_rv;
        *var_sb_i_slot = var_sb_i;
        *var_sb_i_rv_slot = var_sb_i_rv;
        *var_sc_i_slot = var_sc_i;
        *var_sc_i_rv_slot = var_sc_i_rv;
        *var_sd_i_slot = var_sd_i;
        *var_sd_i_rv_slot = var_sd_i_rv;
        *var_ubibot_d_slot = var_ubibot_d;
        *var_ubibot_d_rv_slot = var_ubibot_d_rv;
        *var_ubigat2nd_slot = var_ubigat2nd;
        *var_ubigat2nd_d_slot = var_ubigat2nd_d;
        *var_ubigat2nd_d_rv_slot = var_ubigat2nd_d_rv;
        *var_ubigat2nd_rv_slot = var_ubigat2nd_rv;
        *var_ubigat_d_slot = var_ubigat_d;
        *var_ubigat_d_rv_slot = var_ubigat_d_rv;
        *var_ubisti_d_slot = var_ubisti_d;
        *var_ubisti_d_rv_slot = var_ubisti_d_rv;
        *var_vbibot_d_slot = var_vbibot_d;
        *var_vbibot_d_rv_slot = var_vbibot_d_rv;
        *var_vbigat2nd_slot = var_vbigat2nd;
        *var_vbigat2nd_d_slot = var_vbigat2nd_d;
        *var_vbigat2nd_d_rv_slot = var_vbigat2nd_d_rv;
        *var_vbigat2nd_rv_slot = var_vbigat2nd_rv;
        *var_vbigat_d_slot = var_vbigat_d;
        *var_vbigat_d_rv_slot = var_vbigat_d_rv;
        *var_vbiinvbot_d_slot = var_vbiinvbot_d;
        *var_vbiinvbot_d_rv_slot = var_vbiinvbot_d_rv;
        *var_vbiinvgat2nd_slot = var_vbiinvgat2nd;
        *var_vbiinvgat2nd_d_slot = var_vbiinvgat2nd_d;
        *var_vbiinvgat2nd_d_rv_slot = var_vbiinvgat2nd_d_rv;
        *var_vbiinvgat2nd_rv_slot = var_vbiinvgat2nd_rv;
        *var_vbiinvgat_d_slot = var_vbiinvgat_d;
        *var_vbiinvgat_d_rv_slot = var_vbiinvgat_d_rv;
        *var_vbiinvsti_d_slot = var_vbiinvsti_d;
        *var_vbiinvsti_d_rv_slot = var_vbiinvsti_d_rv;
        *var_vbisti_d_slot = var_vbisti_d;
        *var_vbisti_d_rv_slot = var_vbisti_d_rv;
        *var_w_i_slot = var_w_i;
        *var_w_i_rv_slot = var_w_i_rv;
        *var_we_slot = var_we;
        *var_we_rv_slot = var_we_rv;
    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_l_i: f64,
        var_a1_p_slot: &mut f64,
        var_a1_p_rv_slot: &mut f64,
        var_a2_p_slot: &mut f64,
        var_a2_p_rv_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_a3_p_rv_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_a4_p_rv_slot: &mut f64,
        var_abdrain_i_slot: &mut f64,
        var_abdrain_i_rv_slot: &mut f64,
        var_absource_i_slot: &mut f64,
        var_absource_i_rv_slot: &mut f64,
        var_ad_i_slot: &mut f64,
        var_ad_i_rv_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp1_p_rv_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp2_p_rv_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_alp_p_rv_slot: &mut f64,
        var_as_i_slot: &mut f64,
        var_as_i_rv_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_ax_p_rv_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfb_p_rv_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_cfd_p_rv_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_cs_p_rv_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ct_p_rv_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctb_p_rv_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_ctg_p_rv_slot: &mut f64,
        var_dellps_slot: &mut f64,
        var_dellps_rv_slot: &mut f64,
        var_delwod_slot: &mut f64,
        var_delwod_rv_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dphib_p_rv_slot: &mut f64,
        var_dvsbnud_p_slot: &mut f64,
        var_dvsbnud_p_rv_slot: &mut f64,
        var_epsrox_p_slot: &mut f64,
        var_epsrox_p_rv_slot: &mut f64,
        var_feta_p_slot: &mut f64,
        var_feta_p_rv_slot: &mut f64,
        var_gc2_p_slot: &mut f64,
        var_gc2_p_rv_slot: &mut f64,
        var_gc2ov_p_slot: &mut f64,
        var_gc2ov_p_rv_slot: &mut f64,
        var_gc2ovd_p_slot: &mut f64,
        var_gc2ovd_p_rv_slot: &mut f64,
        var_gc3_p_slot: &mut f64,
        var_gc3_p_rv_slot: &mut f64,
        var_gc3ov_p_slot: &mut f64,
        var_gc3ov_p_rv_slot: &mut f64,
        var_gc3ovd_p_slot: &mut f64,
        var_gc3ovd_p_rv_slot: &mut f64,
        var_gco_p_slot: &mut f64,
        var_gco_p_rv_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_gfacnud_p_rv_slot: &mut f64,
        var_guard29_slot: &mut f64,
        var_guard29_rv_slot: &mut f64,
        var_guard30_slot: &mut f64,
        var_guard30_rv_slot: &mut f64,
        var_guard31_slot: &mut f64,
        var_guard31_rv_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_guard32_rv_slot: &mut f64,
        var_iae_slot: &mut f64,
        var_iae_rv_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_iginv_p_rv_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igov_p_rv_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_igovd_p_rv_slot: &mut f64,
        var_iiae_slot: &mut f64,
        var_iiae_rv_slot: &mut f64,
        var_iilcv_slot: &mut f64,
        var_iilcv_rv_slot: &mut f64,
        var_iiwcv_slot: &mut f64,
        var_iiwcv_rv_slot: &mut f64,
        var_iiwe_slot: &mut f64,
        var_iiwe_rv_slot: &mut f64,
        var_iiwecv_slot: &mut f64,
        var_iiwecv_rv_slot: &mut f64,
        var_il_slot: &mut f64,
        var_il_rv_slot: &mut f64,
        var_ile_slot: &mut f64,
        var_ile2_slot: &mut f64,
        var_ile2_rv_slot: &mut f64,
        var_ile_rv_slot: &mut f64,
        var_imaxii_p_slot: &mut f64,
        var_imaxii_p_rv_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_invnf_rv_slot: &mut f64,
        var_iw_slot: &mut f64,
        var_iw_rv_slot: &mut f64,
        var_iwe_slot: &mut f64,
        var_iwe_rv_slot: &mut f64,
        var_jw_i_slot: &mut f64,
        var_jw_i_rv_slot: &mut f64,
        var_lcv_slot: &mut f64,
        var_lcv_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_lecv_slot: &mut f64,
        var_lecv_rv_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgdrain_i_rv_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lgsource_i_rv_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lsdrain_i_rv_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_lssource_i_rv_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_mue_p_rv_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_neff_p_rv_slot: &mut f64,
        var_nf_i_slot: &mut f64,
        var_nf_i_rv_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_nov_p_rv_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_novd_p_rv_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_np_p_rv_slot: &mut f64,
        var_pd_i_slot: &mut f64,
        var_pd_i_rv_slot: &mut f64,
        var_ps_i_slot: &mut f64,
        var_ps_i_rv_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psce_p_rv_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psceb_p_rv_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_psced_p_rv_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rs_p_rv_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsb_p_rv_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_rsg_p_rv_slot: &mut f64,
        var_sca_i_slot: &mut f64,
        var_sca_i_rv_slot: &mut f64,
        var_scb_i_slot: &mut f64,
        var_scb_i_rv_slot: &mut f64,
        var_scc_i_slot: &mut f64,
        var_scc_i_rv_slot: &mut f64,
        var_st2vfb_p_slot: &mut f64,
        var_st2vfb_p_rv_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_sta2_p_rv_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stbet_p_rv_slot: &mut f64,
        var_stcs_p_slot: &mut f64,
        var_stcs_p_rv_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stct_p_rv_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_stig_p_rv_slot: &mut f64,
        var_stmue_p_slot: &mut f64,
        var_stmue_p_rv_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_strs_p_rv_slot: &mut f64,
        var_stthecs_p_slot: &mut f64,
        var_stthecs_p_rv_slot: &mut f64,
        var_stthemu_p_slot: &mut f64,
        var_stthemu_p_rv_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stthesat_p_rv_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_stvfb_p_rv_slot: &mut f64,
        var_stxcor_p_slot: &mut f64,
        var_stxcor_p_rv_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_thecs_p_rv_slot: &mut f64,
        var_themu_p_slot: &mut f64,
        var_themu_p_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesatb_p_slot: &mut f64,
        var_thesatb_p_rv_slot: &mut f64,
        var_thesatg_p_slot: &mut f64,
        var_thesatg_p_rv_slot: &mut f64,
        var_thesatt_p_slot: &mut f64,
        var_thesatt_p_rv_slot: &mut f64,
        var_tox_p_slot: &mut f64,
        var_tox_p_rv_slot: &mut f64,
        var_toxov_p_slot: &mut f64,
        var_toxov_p_rv_slot: &mut f64,
        var_toxovd_p_slot: &mut f64,
        var_toxovd_p_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vp_p_slot: &mut f64,
        var_vp_p_rv_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
        var_vsbnud_p_rv_slot: &mut f64,
        var_w_i_slot: &mut f64,
        var_w_i_rv_slot: &mut f64,
        var_wcv_slot: &mut f64,
        var_wcv_rv_slot: &mut f64,
        var_we_slot: &mut f64,
        var_we_rv_slot: &mut f64,
        var_wecv_slot: &mut f64,
        var_wecv_rv_slot: &mut f64,
        var_xcor_p_slot: &mut f64,
        var_xcor_p_rv_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a1_p_rv: f64 = *var_a1_p_rv_slot;
        let mut var_a2_p: f64 = *var_a2_p_slot;
        let mut var_a2_p_rv: f64 = *var_a2_p_rv_slot;
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_a3_p_rv: f64 = *var_a3_p_rv_slot;
        let mut var_a4_p: f64 = *var_a4_p_slot;
        let mut var_a4_p_rv: f64 = *var_a4_p_rv_slot;
        let mut var_abdrain_i: f64 = *var_abdrain_i_slot;
        let mut var_abdrain_i_rv: f64 = *var_abdrain_i_rv_slot;
        let mut var_absource_i: f64 = *var_absource_i_slot;
        let mut var_absource_i_rv: f64 = *var_absource_i_rv_slot;
        let mut var_ad_i: f64 = *var_ad_i_slot;
        let mut var_ad_i_rv: f64 = *var_ad_i_rv_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp1_p_rv: f64 = *var_alp1_p_rv_slot;
        let mut var_alp2_p: f64 = *var_alp2_p_slot;
        let mut var_alp2_p_rv: f64 = *var_alp2_p_rv_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_alp_p_rv: f64 = *var_alp_p_rv_slot;
        let mut var_as_i: f64 = *var_as_i_slot;
        let mut var_as_i_rv: f64 = *var_as_i_rv_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_ax_p_rv: f64 = *var_ax_p_rv_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfb_p_rv: f64 = *var_cfb_p_rv_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_cfd_p_rv: f64 = *var_cfd_p_rv_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_cs_p_rv: f64 = *var_cs_p_rv_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ct_p_rv: f64 = *var_ct_p_rv_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctb_p_rv: f64 = *var_ctb_p_rv_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_ctg_p_rv: f64 = *var_ctg_p_rv_slot;
        let mut var_dellps: f64 = *var_dellps_slot;
        let mut var_dellps_rv: f64 = *var_dellps_rv_slot;
        let mut var_delwod: f64 = *var_delwod_slot;
        let mut var_delwod_rv: f64 = *var_delwod_rv_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dphib_p_rv: f64 = *var_dphib_p_rv_slot;
        let mut var_dvsbnud_p: f64 = *var_dvsbnud_p_slot;
        let mut var_dvsbnud_p_rv: f64 = *var_dvsbnud_p_rv_slot;
        let mut var_epsrox_p: f64 = *var_epsrox_p_slot;
        let mut var_epsrox_p_rv: f64 = *var_epsrox_p_rv_slot;
        let mut var_feta_p: f64 = *var_feta_p_slot;
        let mut var_feta_p_rv: f64 = *var_feta_p_rv_slot;
        let mut var_gc2_p: f64 = *var_gc2_p_slot;
        let mut var_gc2_p_rv: f64 = *var_gc2_p_rv_slot;
        let mut var_gc2ov_p: f64 = *var_gc2ov_p_slot;
        let mut var_gc2ov_p_rv: f64 = *var_gc2ov_p_rv_slot;
        let mut var_gc2ovd_p: f64 = *var_gc2ovd_p_slot;
        let mut var_gc2ovd_p_rv: f64 = *var_gc2ovd_p_rv_slot;
        let mut var_gc3_p: f64 = *var_gc3_p_slot;
        let mut var_gc3_p_rv: f64 = *var_gc3_p_rv_slot;
        let mut var_gc3ov_p: f64 = *var_gc3ov_p_slot;
        let mut var_gc3ov_p_rv: f64 = *var_gc3ov_p_rv_slot;
        let mut var_gc3ovd_p: f64 = *var_gc3ovd_p_slot;
        let mut var_gc3ovd_p_rv: f64 = *var_gc3ovd_p_rv_slot;
        let mut var_gco_p: f64 = *var_gco_p_slot;
        let mut var_gco_p_rv: f64 = *var_gco_p_rv_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_gfacnud_p_rv: f64 = *var_gfacnud_p_rv_slot;
        let mut var_guard29: f64 = *var_guard29_slot;
        let mut var_guard29_rv: f64 = *var_guard29_rv_slot;
        let mut var_guard30: f64 = *var_guard30_slot;
        let mut var_guard30_rv: f64 = *var_guard30_rv_slot;
        let mut var_guard31: f64 = *var_guard31_slot;
        let mut var_guard31_rv: f64 = *var_guard31_rv_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_guard32_rv: f64 = *var_guard32_rv_slot;
        let mut var_iae: f64 = *var_iae_slot;
        let mut var_iae_rv: f64 = *var_iae_rv_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_iginv_p_rv: f64 = *var_iginv_p_rv_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igov_p_rv: f64 = *var_igov_p_rv_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_igovd_p_rv: f64 = *var_igovd_p_rv_slot;
        let mut var_iiae: f64 = *var_iiae_slot;
        let mut var_iiae_rv: f64 = *var_iiae_rv_slot;
        let mut var_iilcv: f64 = *var_iilcv_slot;
        let mut var_iilcv_rv: f64 = *var_iilcv_rv_slot;
        let mut var_iiwcv: f64 = *var_iiwcv_slot;
        let mut var_iiwcv_rv: f64 = *var_iiwcv_rv_slot;
        let mut var_iiwe: f64 = *var_iiwe_slot;
        let mut var_iiwe_rv: f64 = *var_iiwe_rv_slot;
        let mut var_iiwecv: f64 = *var_iiwecv_slot;
        let mut var_iiwecv_rv: f64 = *var_iiwecv_rv_slot;
        let mut var_il: f64 = *var_il_slot;
        let mut var_il_rv: f64 = *var_il_rv_slot;
        let mut var_ile: f64 = *var_ile_slot;
        let mut var_ile2: f64 = *var_ile2_slot;
        let mut var_ile2_rv: f64 = *var_ile2_rv_slot;
        let mut var_ile_rv: f64 = *var_ile_rv_slot;
        let mut var_imaxii_p: f64 = *var_imaxii_p_slot;
        let mut var_imaxii_p_rv: f64 = *var_imaxii_p_rv_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_invnf_rv: f64 = *var_invnf_rv_slot;
        let mut var_iw: f64 = *var_iw_slot;
        let mut var_iw_rv: f64 = *var_iw_rv_slot;
        let mut var_iwe: f64 = *var_iwe_slot;
        let mut var_iwe_rv: f64 = *var_iwe_rv_slot;
        let mut var_jw_i: f64 = *var_jw_i_slot;
        let mut var_jw_i_rv: f64 = *var_jw_i_rv_slot;
        let mut var_lcv: f64 = *var_lcv_slot;
        let mut var_lcv_rv: f64 = *var_lcv_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_lecv: f64 = *var_lecv_slot;
        let mut var_lecv_rv: f64 = *var_lecv_rv_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgdrain_i_rv: f64 = *var_lgdrain_i_rv_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lgsource_i_rv: f64 = *var_lgsource_i_rv_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lsdrain_i_rv: f64 = *var_lsdrain_i_rv_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_lssource_i_rv: f64 = *var_lssource_i_rv_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_mue_p_rv: f64 = *var_mue_p_rv_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_neff_p_rv: f64 = *var_neff_p_rv_slot;
        let mut var_nf_i: f64 = *var_nf_i_slot;
        let mut var_nf_i_rv: f64 = *var_nf_i_rv_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_nov_p_rv: f64 = *var_nov_p_rv_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_novd_p_rv: f64 = *var_novd_p_rv_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_np_p_rv: f64 = *var_np_p_rv_slot;
        let mut var_pd_i: f64 = *var_pd_i_slot;
        let mut var_pd_i_rv: f64 = *var_pd_i_rv_slot;
        let mut var_ps_i: f64 = *var_ps_i_slot;
        let mut var_ps_i_rv: f64 = *var_ps_i_rv_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psce_p_rv: f64 = *var_psce_p_rv_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psceb_p_rv: f64 = *var_psceb_p_rv_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_psced_p_rv: f64 = *var_psced_p_rv_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rs_p_rv: f64 = *var_rs_p_rv_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsb_p_rv: f64 = *var_rsb_p_rv_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_rsg_p_rv: f64 = *var_rsg_p_rv_slot;
        let mut var_sca_i: f64 = *var_sca_i_slot;
        let mut var_sca_i_rv: f64 = *var_sca_i_rv_slot;
        let mut var_scb_i: f64 = *var_scb_i_slot;
        let mut var_scb_i_rv: f64 = *var_scb_i_rv_slot;
        let mut var_scc_i: f64 = *var_scc_i_slot;
        let mut var_scc_i_rv: f64 = *var_scc_i_rv_slot;
        let mut var_st2vfb_p: f64 = *var_st2vfb_p_slot;
        let mut var_st2vfb_p_rv: f64 = *var_st2vfb_p_rv_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_sta2_p_rv: f64 = *var_sta2_p_rv_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stbet_p_rv: f64 = *var_stbet_p_rv_slot;
        let mut var_stcs_p: f64 = *var_stcs_p_slot;
        let mut var_stcs_p_rv: f64 = *var_stcs_p_rv_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stct_p_rv: f64 = *var_stct_p_rv_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_stig_p_rv: f64 = *var_stig_p_rv_slot;
        let mut var_stmue_p: f64 = *var_stmue_p_slot;
        let mut var_stmue_p_rv: f64 = *var_stmue_p_rv_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_strs_p_rv: f64 = *var_strs_p_rv_slot;
        let mut var_stthecs_p: f64 = *var_stthecs_p_slot;
        let mut var_stthecs_p_rv: f64 = *var_stthecs_p_rv_slot;
        let mut var_stthemu_p: f64 = *var_stthemu_p_slot;
        let mut var_stthemu_p_rv: f64 = *var_stthemu_p_rv_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stthesat_p_rv: f64 = *var_stthesat_p_rv_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_stvfb_p_rv: f64 = *var_stvfb_p_rv_slot;
        let mut var_stxcor_p: f64 = *var_stxcor_p_slot;
        let mut var_stxcor_p_rv: f64 = *var_stxcor_p_rv_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_thecs_p_rv: f64 = *var_thecs_p_rv_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_themu_p_rv: f64 = *var_themu_p_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_thesatb_p_rv: f64 = *var_thesatb_p_rv_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;
        let mut var_thesatg_p_rv: f64 = *var_thesatg_p_rv_slot;
        let mut var_thesatt_p: f64 = *var_thesatt_p_slot;
        let mut var_thesatt_p_rv: f64 = *var_thesatt_p_rv_slot;
        let mut var_tox_p: f64 = *var_tox_p_slot;
        let mut var_tox_p_rv: f64 = *var_tox_p_rv_slot;
        let mut var_toxov_p: f64 = *var_toxov_p_slot;
        let mut var_toxov_p_rv: f64 = *var_toxov_p_rv_slot;
        let mut var_toxovd_p: f64 = *var_toxovd_p_slot;
        let mut var_toxovd_p_rv: f64 = *var_toxovd_p_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vp_p: f64 = *var_vp_p_slot;
        let mut var_vp_p_rv: f64 = *var_vp_p_rv_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_vsbnud_p_rv: f64 = *var_vsbnud_p_rv_slot;
        let mut var_w_i: f64 = *var_w_i_slot;
        let mut var_w_i_rv: f64 = *var_w_i_rv_slot;
        let mut var_wcv: f64 = *var_wcv_slot;
        let mut var_wcv_rv: f64 = *var_wcv_rv_slot;
        let mut var_we: f64 = *var_we_slot;
        let mut var_we_rv: f64 = *var_we_rv_slot;
        let mut var_wecv: f64 = *var_wecv_slot;
        let mut var_wecv_rv: f64 = *var_wecv_rv_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;
        let mut var_xcor_p_rv: f64 = *var_xcor_p_rv_slot;

        var_absource_i = p.p19;
        var_absource_i_rv = 0.0;

        var_lssource_i = p.p20;
        var_lssource_i_rv = 0.0;

        var_lgsource_i = p.p21;
        var_lgsource_i_rv = 0.0;

        var_abdrain_i = p.p22;
        var_abdrain_i_rv = 0.0;

        var_lsdrain_i = p.p23;
        var_lsdrain_i_rv = 0.0;

        var_lgdrain_i = p.p24;
        var_lgdrain_i_rv = 0.0;

        var_as_i = p.p25;
        var_as_i_rv = 0.0;

        var_ps_i = p.p26;
        var_ps_i_rv = 0.0;

        var_ad_i = p.p27;
        var_ad_i_rv = 0.0;

        var_pd_i = p.p28;
        var_pd_i_rv = 0.0;

        var_jw_i = p.p14;
        var_jw_i_rv = 0.0;

        let assign3500_e3418: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard29 = assign3500_e3418;
        var_guard29_rv = 0.0;

        let (assign3510_e3427,) = {
    if (var_guard29 != 0.0) {
        let (assign3510_e3425,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3510_e3425,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3510_e3427;
        var_nf_i_rv = 0.0;

        let (assign3520_e3434,) = {
    if (var_guard29 != 0.0) {
        let assign3520_e3431: f64 = (var_nf_i + 0.5);
        let assign3520_e3432: f64 = (assign3520_e3431).floor();
        (assign3520_e3432,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3520_e3434;
        var_nf_i_rv = 0.0;

        let (assign3530_e3440,) = {
    if (var_guard29 != 0.0) {
        let assign3530_e3438: f64 = (1.0 / var_nf_i);
        (assign3530_e3438,)
    } else {
        (var_invnf,)
    }
};
        var_invnf = assign3530_e3440;
        var_invnf_rv = 0.0;

        let assign3540_e3443: f64 = (var_w_i * var_invnf);
        let (assign3540_e3450,) = {
    if (assign3540_e3443 > 1e-9) {
        let assign3540_e3448: f64 = (var_w_i * var_invnf);
        (assign3540_e3448,)
    } else {
        (1e-9,)
    }
};
        var_w_i = assign3540_e3450;
        var_w_i_rv = 0.0;

        var_sca_i = p.p5;
        var_sca_i_rv = 0.0;

        var_scb_i = p.p6;
        var_scb_i_rv = 0.0;

        var_scc_i = p.p7;
        var_scc_i_rv = 0.0;

        let assign3590_e3462: f64 = (1e-6 / var_l_i);
        var_il = assign3590_e3462;
        var_il_rv = 0.0;

        let assign3600_e3465: f64 = (1e-6 / var_w_i);
        var_iw = assign3600_e3465;
        var_iw_rv = 0.0;

        let assign3610_e3470: f64 = (p.p187 * var_il);
        let assign3610_e3471: f64 = (1.0 + assign3610_e3470);
        let assign3610_e3472: f64 = (p.p186 * assign3610_e3471);
        let assign3610_e3476: f64 = (p.p188 * var_iw);
        let assign3610_e3477: f64 = (1.0 + assign3610_e3476);
        let assign3610_e3478: f64 = (assign3610_e3472 * assign3610_e3477);
        var_dellps = assign3610_e3478;
        var_dellps_rv = 0.0;

        let assign3620_e3483: f64 = (p.p191 * var_il);
        let assign3620_e3484: f64 = (1.0 + assign3620_e3483);
        let assign3620_e3485: f64 = (p.p190 * assign3620_e3484);
        let assign3620_e3489: f64 = (p.p192 * var_iw);
        let assign3620_e3490: f64 = (1.0 + assign3620_e3489);
        let assign3620_e3491: f64 = (assign3620_e3485 * assign3620_e3490);
        var_delwod = assign3620_e3491;
        var_delwod_rv = 0.0;

        let assign3630_e3494: f64 = (var_l_i + var_dellps);
        let assign3630_e3497: f64 = (2.0 * p.p189);
        let assign3630_e3498: f64 = (assign3630_e3494 - assign3630_e3497);
        let (assign3630_e3509,) = {
    if (assign3630_e3498 > 1e-9) {
        let assign3630_e3503: f64 = (var_l_i + var_dellps);
        let assign3630_e3506: f64 = (2.0 * p.p189);
        let assign3630_e3507: f64 = (assign3630_e3503 - assign3630_e3506);
        (assign3630_e3507,)
    } else {
        (1e-9,)
    }
};
        var_le = assign3630_e3509;
        var_le_rv = 0.0;

        let assign3640_e3512: f64 = (var_w_i + var_delwod);
        let assign3640_e3515: f64 = (2.0 * p.p193);
        let assign3640_e3516: f64 = (assign3640_e3512 - assign3640_e3515);
        let (assign3640_e3527,) = {
    if (assign3640_e3516 > 1e-9) {
        let assign3640_e3521: f64 = (var_w_i + var_delwod);
        let assign3640_e3524: f64 = (2.0 * p.p193);
        let assign3640_e3525: f64 = (assign3640_e3521 - assign3640_e3524);
        (assign3640_e3525,)
    } else {
        (1e-9,)
    }
};
        var_we = assign3640_e3527;
        var_we_rv = 0.0;

        let assign3650_e3530: f64 = (1e-6 / var_le);
        var_ile = assign3650_e3530;
        var_ile_rv = 0.0;

        let assign3660_e3533: f64 = (var_ile * var_ile);
        var_ile2 = assign3660_e3533;
        var_ile2_rv = 0.0;

        let assign3670_e3536: f64 = (1e-6 / var_we);
        var_iwe = assign3670_e3536;
        var_iwe_rv = 0.0;

        let assign3680_e3539: f64 = (1.0 / var_iwe);
        var_iiwe = assign3680_e3539;
        var_iiwe_rv = 0.0;

        let assign3690_e3542: f64 = (var_ile * var_iwe);
        var_iae = assign3690_e3542;
        var_iae_rv = 0.0;

        let assign3700_e3545: f64 = (1.0 / var_iae);
        var_iiae = assign3700_e3545;
        var_iiae_rv = 0.0;

        let assign3710_e3548: f64 = (var_l_i + var_dellps);
        let assign3710_e3551: f64 = (2.0 * p.p189);
        let assign3710_e3552: f64 = (assign3710_e3548 - assign3710_e3551);
        let assign3710_e3554: f64 = (assign3710_e3552 + p.p194);
        let (assign3710_e3567,) = {
    if (assign3710_e3554 > 1e-9) {
        let assign3710_e3559: f64 = (var_l_i + var_dellps);
        let assign3710_e3562: f64 = (2.0 * p.p189);
        let assign3710_e3563: f64 = (assign3710_e3559 - assign3710_e3562);
        let assign3710_e3565: f64 = (assign3710_e3563 + p.p194);
        (assign3710_e3565,)
    } else {
        (1e-9,)
    }
};
        var_lecv = assign3710_e3567;
        var_lecv_rv = 0.0;

        let assign3720_e3570: f64 = (var_w_i + var_delwod);
        let assign3720_e3573: f64 = (2.0 * p.p193);
        let assign3720_e3574: f64 = (assign3720_e3570 - assign3720_e3573);
        let assign3720_e3576: f64 = (assign3720_e3574 + p.p195);
        let (assign3720_e3589,) = {
    if (assign3720_e3576 > 1e-9) {
        let assign3720_e3581: f64 = (var_w_i + var_delwod);
        let assign3720_e3584: f64 = (2.0 * p.p193);
        let assign3720_e3585: f64 = (assign3720_e3581 - assign3720_e3584);
        let assign3720_e3587: f64 = (assign3720_e3585 + p.p195);
        (assign3720_e3587,)
    } else {
        (1e-9,)
    }
};
        var_wecv = assign3720_e3589;
        var_wecv_rv = 0.0;

        let assign3730_e3592: f64 = (var_wecv / 1e-6);
        var_iiwecv = assign3730_e3592;
        var_iiwecv_rv = 0.0;

        let assign3740_e3595: f64 = (var_l_i + var_dellps);
        let assign3740_e3597: f64 = (assign3740_e3595 + p.p194);
        let (assign3740_e3606,) = {
    if (assign3740_e3597 > 1e-9) {
        let assign3740_e3602: f64 = (var_l_i + var_dellps);
        let assign3740_e3604: f64 = (assign3740_e3602 + p.p194);
        (assign3740_e3604,)
    } else {
        (1e-9,)
    }
};
        var_lcv = assign3740_e3606;
        var_lcv_rv = 0.0;

        let assign3750_e3609: f64 = (var_w_i + var_delwod);
        let assign3750_e3611: f64 = (assign3750_e3609 + p.p195);
        let (assign3750_e3620,) = {
    if (assign3750_e3611 > 1e-9) {
        let assign3750_e3616: f64 = (var_w_i + var_delwod);
        let assign3750_e3618: f64 = (assign3750_e3616 + p.p195);
        (assign3750_e3618,)
    } else {
        (1e-9,)
    }
};
        var_wcv = assign3750_e3620;
        var_wcv_rv = 0.0;

        let assign3760_e3623: f64 = (var_lcv / 1e-6);
        var_iilcv = assign3760_e3623;
        var_iilcv_rv = 0.0;

        let assign3770_e3626: f64 = (var_wcv / 1e-6);
        var_iiwcv = assign3770_e3626;
        var_iiwcv_rv = 0.0;

        var_vfb_p = p.p56;
        var_vfb_p_rv = 0.0;

        var_stvfb_p = p.p57;
        var_stvfb_p_rv = 0.0;

        var_st2vfb_p = p.p58;
        var_st2vfb_p_rv = 0.0;

        var_tox_p = p.p59;
        var_tox_p_rv = 0.0;

        var_epsrox_p = p.p60;
        var_epsrox_p_rv = 0.0;

        var_neff_p = p.p61;
        var_neff_p_rv = 0.0;

        var_gfacnud_p = p.p62;
        var_gfacnud_p_rv = 0.0;

        var_vsbnud_p = p.p63;
        var_vsbnud_p_rv = 0.0;

        var_dvsbnud_p = p.p64;
        var_dvsbnud_p_rv = 0.0;

        var_dphib_p = p.p65;
        var_dphib_p_rv = 0.0;

        var_np_p = p.p66;
        var_np_p_rv = 0.0;

        var_toxov_p = p.p67;
        var_toxov_p_rv = 0.0;

        var_toxovd_p = p.p68;
        var_toxovd_p_rv = 0.0;

        var_nov_p = p.p69;
        var_nov_p_rv = 0.0;

        var_novd_p = p.p70;
        var_novd_p_rv = 0.0;

        var_ct_p = p.p71;
        var_ct_p_rv = 0.0;

        var_ctg_p = p.p73;
        var_ctg_p_rv = 0.0;

        var_ctb_p = p.p72;
        var_ctb_p_rv = 0.0;

        var_stct_p = p.p74;
        var_stct_p_rv = 0.0;

        var_psce_p = p.p78;
        var_psce_p_rv = 0.0;

        var_psced_p = p.p80;
        var_psced_p_rv = 0.0;

        var_psceb_p = p.p79;
        var_psceb_p_rv = 0.0;

        var_cf_p = p.p75;
        var_cf_p_rv = 0.0;

        var_cfd_p = p.p77;
        var_cfd_p_rv = 0.0;

        var_cfb_p = p.p76;
        var_cfb_p_rv = 0.0;

        var_betn_p = p.p81;
        var_betn_p_rv = 0.0;

        var_stbet_p = p.p82;
        var_stbet_p_rv = 0.0;

        var_mue_p = p.p83;
        var_mue_p_rv = 0.0;

        var_stmue_p = p.p84;
        var_stmue_p_rv = 0.0;

        var_themu_p = p.p85;
        var_themu_p_rv = 0.0;

        var_stthemu_p = p.p86;
        var_stthemu_p_rv = 0.0;

        var_cs_p = p.p87;
        var_cs_p_rv = 0.0;

        var_stcs_p = p.p88;
        var_stcs_p_rv = 0.0;

        var_thecs_p = p.p89;
        var_thecs_p_rv = 0.0;

        var_stthecs_p = p.p90;
        var_stthecs_p_rv = 0.0;

        var_xcor_p = p.p91;
        var_xcor_p_rv = 0.0;

        var_stxcor_p = p.p92;
        var_stxcor_p_rv = 0.0;

        var_feta_p = p.p93;
        var_feta_p_rv = 0.0;

        var_rs_p = p.p94;
        var_rs_p_rv = 0.0;

        var_strs_p = p.p95;
        var_strs_p_rv = 0.0;

        var_rsb_p = p.p96;
        var_rsb_p_rv = 0.0;

        var_rsg_p = p.p97;
        var_rsg_p_rv = 0.0;

        var_thesat_p = p.p98;
        var_thesat_p_rv = 0.0;

        var_stthesat_p = p.p99;
        var_stthesat_p_rv = 0.0;

        var_thesatb_p = p.p100;
        var_thesatb_p_rv = 0.0;

        var_thesatg_p = p.p101;
        var_thesatg_p_rv = 0.0;

        var_thesatt_p = p.p102;
        var_thesatt_p_rv = 0.0;

        var_ax_p = p.p103;
        var_ax_p_rv = 0.0;

        var_alp_p = p.p104;
        var_alp_p_rv = 0.0;

        var_alp1_p = p.p105;
        var_alp1_p_rv = 0.0;

        var_alp2_p = p.p106;
        var_alp2_p_rv = 0.0;

        var_vp_p = p.p107;
        var_vp_p_rv = 0.0;

        var_a1_p = p.p108;
        var_a1_p_rv = 0.0;

        var_a2_p = p.p109;
        var_a2_p_rv = 0.0;

        var_sta2_p = p.p110;
        var_sta2_p_rv = 0.0;

        var_a3_p = p.p111;
        var_a3_p_rv = 0.0;

        var_a4_p = p.p112;
        var_a4_p_rv = 0.0;

        var_imaxii_p = p.p113;
        var_imaxii_p_rv = 0.0;

        var_gco_p = p.p114;
        var_gco_p_rv = 0.0;

        var_iginv_p = p.p115;
        var_iginv_p_rv = 0.0;

        var_igov_p = p.p116;
        var_igov_p_rv = 0.0;

        var_igovd_p = p.p117;
        var_igovd_p_rv = 0.0;

        var_stig_p = p.p118;
        var_stig_p_rv = 0.0;

        var_gc2_p = p.p119;
        var_gc2_p_rv = 0.0;

        var_gc3_p = p.p120;
        var_gc3_p_rv = 0.0;

        var_gc2ov_p = p.p119;
        var_gc2ov_p_rv = 0.0;

        let assign4480_e3738: f64 = if param_given[121] { 1.0 } else { 0.0 };
        let assign4480_e3740: f64 = if assign4480_e3738 == 1.0 { 1.0 } else { 0.0 };
        var_guard30 = assign4480_e3740;
        var_guard30_rv = 0.0;

        let (assign4490_e3744,) = {
    if (var_guard30 != 0.0) {
        (p.p121,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign4490_e3744;
        var_gc2ov_p_rv = 0.0;

        var_gc3ov_p = p.p120;
        var_gc3ov_p_rv = 0.0;

        let assign4510_e3747: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4510_e3749: f64 = if assign4510_e3747 == 1.0 { 1.0 } else { 0.0 };
        var_guard31 = assign4510_e3749;
        var_guard31_rv = 0.0;

        let (assign4520_e3753,) = {
    if (var_guard31 != 0.0) {
        (p.p122,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign4520_e3753;
        var_gc3ov_p_rv = 0.0;

        var_gc2ovd_p = var_gc2ov_p;
        var_gc2ovd_p_rv = 0.0;

        let assign4540_e3756: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4540_e3758: f64 = if assign4540_e3756 == 1.0 { 1.0 } else { 0.0 };
        var_guard32 = assign4540_e3758;
        var_guard32_rv = 0.0;

        let (assign4550_e3762,) = {
    if (var_guard32 != 0.0) {
        (p.p123,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign4550_e3762;
        var_gc2ovd_p_rv = 0.0;

        var_gc3ovd_p = var_gc3ov_p;
        var_gc3ovd_p_rv = 0.0;

        *var_a1_p_slot = var_a1_p;
        *var_a1_p_rv_slot = var_a1_p_rv;
        *var_a2_p_slot = var_a2_p;
        *var_a2_p_rv_slot = var_a2_p_rv;
        *var_a3_p_slot = var_a3_p;
        *var_a3_p_rv_slot = var_a3_p_rv;
        *var_a4_p_slot = var_a4_p;
        *var_a4_p_rv_slot = var_a4_p_rv;
        *var_abdrain_i_slot = var_abdrain_i;
        *var_abdrain_i_rv_slot = var_abdrain_i_rv;
        *var_absource_i_slot = var_absource_i;
        *var_absource_i_rv_slot = var_absource_i_rv;
        *var_ad_i_slot = var_ad_i;
        *var_ad_i_rv_slot = var_ad_i_rv;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp1_p_rv_slot = var_alp1_p_rv;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp2_p_rv_slot = var_alp2_p_rv;
        *var_alp_p_slot = var_alp_p;
        *var_alp_p_rv_slot = var_alp_p_rv;
        *var_as_i_slot = var_as_i;
        *var_as_i_rv_slot = var_as_i_rv;
        *var_ax_p_slot = var_ax_p;
        *var_ax_p_rv_slot = var_ax_p_rv;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfb_p_rv_slot = var_cfb_p_rv;
        *var_cfd_p_slot = var_cfd_p;
        *var_cfd_p_rv_slot = var_cfd_p_rv;
        *var_cs_p_slot = var_cs_p;
        *var_cs_p_rv_slot = var_cs_p_rv;
        *var_ct_p_slot = var_ct_p;
        *var_ct_p_rv_slot = var_ct_p_rv;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctb_p_rv_slot = var_ctb_p_rv;
        *var_ctg_p_slot = var_ctg_p;
        *var_ctg_p_rv_slot = var_ctg_p_rv;
        *var_dellps_slot = var_dellps;
        *var_dellps_rv_slot = var_dellps_rv;
        *var_delwod_slot = var_delwod;
        *var_delwod_rv_slot = var_delwod_rv;
        *var_dphib_p_slot = var_dphib_p;
        *var_dphib_p_rv_slot = var_dphib_p_rv;
        *var_dvsbnud_p_slot = var_dvsbnud_p;
        *var_dvsbnud_p_rv_slot = var_dvsbnud_p_rv;
        *var_epsrox_p_slot = var_epsrox_p;
        *var_epsrox_p_rv_slot = var_epsrox_p_rv;
        *var_feta_p_slot = var_feta_p;
        *var_feta_p_rv_slot = var_feta_p_rv;
        *var_gc2_p_slot = var_gc2_p;
        *var_gc2_p_rv_slot = var_gc2_p_rv;
        *var_gc2ov_p_slot = var_gc2ov_p;
        *var_gc2ov_p_rv_slot = var_gc2ov_p_rv;
        *var_gc2ovd_p_slot = var_gc2ovd_p;
        *var_gc2ovd_p_rv_slot = var_gc2ovd_p_rv;
        *var_gc3_p_slot = var_gc3_p;
        *var_gc3_p_rv_slot = var_gc3_p_rv;
        *var_gc3ov_p_slot = var_gc3ov_p;
        *var_gc3ov_p_rv_slot = var_gc3ov_p_rv;
        *var_gc3ovd_p_slot = var_gc3ovd_p;
        *var_gc3ovd_p_rv_slot = var_gc3ovd_p_rv;
        *var_gco_p_slot = var_gco_p;
        *var_gco_p_rv_slot = var_gco_p_rv;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_gfacnud_p_rv_slot = var_gfacnud_p_rv;
        *var_guard29_slot = var_guard29;
        *var_guard29_rv_slot = var_guard29_rv;
        *var_guard30_slot = var_guard30;
        *var_guard30_rv_slot = var_guard30_rv;
        *var_guard31_slot = var_guard31;
        *var_guard31_rv_slot = var_guard31_rv;
        *var_guard32_slot = var_guard32;
        *var_guard32_rv_slot = var_guard32_rv;
        *var_iae_slot = var_iae;
        *var_iae_rv_slot = var_iae_rv;
        *var_iginv_p_slot = var_iginv_p;
        *var_iginv_p_rv_slot = var_iginv_p_rv;
        *var_igov_p_slot = var_igov_p;
        *var_igov_p_rv_slot = var_igov_p_rv;
        *var_igovd_p_slot = var_igovd_p;
        *var_igovd_p_rv_slot = var_igovd_p_rv;
        *var_iiae_slot = var_iiae;
        *var_iiae_rv_slot = var_iiae_rv;
        *var_iilcv_slot = var_iilcv;
        *var_iilcv_rv_slot = var_iilcv_rv;
        *var_iiwcv_slot = var_iiwcv;
        *var_iiwcv_rv_slot = var_iiwcv_rv;
        *var_iiwe_slot = var_iiwe;
        *var_iiwe_rv_slot = var_iiwe_rv;
        *var_iiwecv_slot = var_iiwecv;
        *var_iiwecv_rv_slot = var_iiwecv_rv;
        *var_il_slot = var_il;
        *var_il_rv_slot = var_il_rv;
        *var_ile_slot = var_ile;
        *var_ile2_slot = var_ile2;
        *var_ile2_rv_slot = var_ile2_rv;
        *var_ile_rv_slot = var_ile_rv;
        *var_imaxii_p_slot = var_imaxii_p;
        *var_imaxii_p_rv_slot = var_imaxii_p_rv;
        *var_invnf_slot = var_invnf;
        *var_invnf_rv_slot = var_invnf_rv;
        *var_iw_slot = var_iw;
        *var_iw_rv_slot = var_iw_rv;
        *var_iwe_slot = var_iwe;
        *var_iwe_rv_slot = var_iwe_rv;
        *var_jw_i_slot = var_jw_i;
        *var_jw_i_rv_slot = var_jw_i_rv;
        *var_lcv_slot = var_lcv;
        *var_lcv_rv_slot = var_lcv_rv;
        *var_le_slot = var_le;
        *var_le_rv_slot = var_le_rv;
        *var_lecv_slot = var_lecv;
        *var_lecv_rv_slot = var_lecv_rv;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgdrain_i_rv_slot = var_lgdrain_i_rv;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lgsource_i_rv_slot = var_lgsource_i_rv;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lsdrain_i_rv_slot = var_lsdrain_i_rv;
        *var_lssource_i_slot = var_lssource_i;
        *var_lssource_i_rv_slot = var_lssource_i_rv;
        *var_mue_p_slot = var_mue_p;
        *var_mue_p_rv_slot = var_mue_p_rv;
        *var_neff_p_slot = var_neff_p;
        *var_neff_p_rv_slot = var_neff_p_rv;
        *var_nf_i_slot = var_nf_i;
        *var_nf_i_rv_slot = var_nf_i_rv;
        *var_nov_p_slot = var_nov_p;
        *var_nov_p_rv_slot = var_nov_p_rv;
        *var_novd_p_slot = var_novd_p;
        *var_novd_p_rv_slot = var_novd_p_rv;
        *var_np_p_slot = var_np_p;
        *var_np_p_rv_slot = var_np_p_rv;
        *var_pd_i_slot = var_pd_i;
        *var_pd_i_rv_slot = var_pd_i_rv;
        *var_ps_i_slot = var_ps_i;
        *var_ps_i_rv_slot = var_ps_i_rv;
        *var_psce_p_slot = var_psce_p;
        *var_psce_p_rv_slot = var_psce_p_rv;
        *var_psceb_p_slot = var_psceb_p;
        *var_psceb_p_rv_slot = var_psceb_p_rv;
        *var_psced_p_slot = var_psced_p;
        *var_psced_p_rv_slot = var_psced_p_rv;
        *var_rs_p_slot = var_rs_p;
        *var_rs_p_rv_slot = var_rs_p_rv;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsb_p_rv_slot = var_rsb_p_rv;
        *var_rsg_p_slot = var_rsg_p;
        *var_rsg_p_rv_slot = var_rsg_p_rv;
        *var_sca_i_slot = var_sca_i;
        *var_sca_i_rv_slot = var_sca_i_rv;
        *var_scb_i_slot = var_scb_i;
        *var_scb_i_rv_slot = var_scb_i_rv;
        *var_scc_i_slot = var_scc_i;
        *var_scc_i_rv_slot = var_scc_i_rv;
        *var_st2vfb_p_slot = var_st2vfb_p;
        *var_st2vfb_p_rv_slot = var_st2vfb_p_rv;
        *var_sta2_p_slot = var_sta2_p;
        *var_sta2_p_rv_slot = var_sta2_p_rv;
        *var_stbet_p_slot = var_stbet_p;
        *var_stbet_p_rv_slot = var_stbet_p_rv;
        *var_stcs_p_slot = var_stcs_p;
        *var_stcs_p_rv_slot = var_stcs_p_rv;
        *var_stct_p_slot = var_stct_p;
        *var_stct_p_rv_slot = var_stct_p_rv;
        *var_stig_p_slot = var_stig_p;
        *var_stig_p_rv_slot = var_stig_p_rv;
        *var_stmue_p_slot = var_stmue_p;
        *var_stmue_p_rv_slot = var_stmue_p_rv;
        *var_strs_p_slot = var_strs_p;
        *var_strs_p_rv_slot = var_strs_p_rv;
        *var_stthecs_p_slot = var_stthecs_p;
        *var_stthecs_p_rv_slot = var_stthecs_p_rv;
        *var_stthemu_p_slot = var_stthemu_p;
        *var_stthemu_p_rv_slot = var_stthemu_p_rv;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stthesat_p_rv_slot = var_stthesat_p_rv;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_stvfb_p_rv_slot = var_stvfb_p_rv;
        *var_stxcor_p_slot = var_stxcor_p;
        *var_stxcor_p_rv_slot = var_stxcor_p_rv;
        *var_thecs_p_slot = var_thecs_p;
        *var_thecs_p_rv_slot = var_thecs_p_rv;
        *var_themu_p_slot = var_themu_p;
        *var_themu_p_rv_slot = var_themu_p_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_thesatb_p_rv_slot = var_thesatb_p_rv;
        *var_thesatg_p_slot = var_thesatg_p;
        *var_thesatg_p_rv_slot = var_thesatg_p_rv;
        *var_thesatt_p_slot = var_thesatt_p;
        *var_thesatt_p_rv_slot = var_thesatt_p_rv;
        *var_tox_p_slot = var_tox_p;
        *var_tox_p_rv_slot = var_tox_p_rv;
        *var_toxov_p_slot = var_toxov_p;
        *var_toxov_p_rv_slot = var_toxov_p_rv;
        *var_toxovd_p_slot = var_toxovd_p;
        *var_toxovd_p_rv_slot = var_toxovd_p_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vp_p_slot = var_vp_p;
        *var_vp_p_rv_slot = var_vp_p_rv;
        *var_vsbnud_p_slot = var_vsbnud_p;
        *var_vsbnud_p_rv_slot = var_vsbnud_p_rv;
        *var_w_i_slot = var_w_i;
        *var_w_i_rv_slot = var_w_i_rv;
        *var_wcv_slot = var_wcv;
        *var_wcv_rv_slot = var_wcv_rv;
        *var_we_slot = var_we;
        *var_we_rv_slot = var_we_rv;
        *var_wecv_slot = var_wecv;
        *var_wecv_rv_slot = var_wecv_rv;
        *var_xcor_p_slot = var_xcor_p;
        *var_xcor_p_rv_slot = var_xcor_p_rv;
    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_aa_slot: &mut f64,
        var_aa_rv_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidl_p_rv_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_agidld_p_rv_slot: &mut f64,
        var_alp1ac_p_slot: &mut f64,
        var_alp1ac_p_rv_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_alpac_p_rv_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axac_p_rv_slot: &mut f64,
        var_axinr_p_slot: &mut f64,
        var_axinr_p_rv_slot: &mut f64,
        var_bb_slot: &mut f64,
        var_bb_rv_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_bgidl_p_slot: &mut f64,
        var_bgidl_p_rv_slot: &mut f64,
        var_bgidld_p_slot: &mut f64,
        var_bgidld_p_rv_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfbedge_p_rv_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfdedge_p_rv_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfedge_p_rv_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfr_p_rv_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cfrd_p_rv_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgbov_p_rv_slot: &mut f64,
        var_cgidl_p_slot: &mut f64,
        var_cgidl_p_rv_slot: &mut f64,
        var_cgidld_p_slot: &mut f64,
        var_cgidld_p_rv_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgov_p_rv_slot: &mut f64,
        var_cgovaccg_p_slot: &mut f64,
        var_cgovaccg_p_rv_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_cgovd_p_rv_slot: &mut f64,
        var_chib_p_slot: &mut f64,
        var_chib_p_rv_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinr_p_rv_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_cinrd_p_rv_slot: &mut f64,
        var_cox_p_slot: &mut f64,
        var_cox_p_rv_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_ctedge_p_rv_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_delvtac_p_rv_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dphib_p_rv_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_dphibedge_p_rv_slot: &mut f64,
        var_dvfbinr_p_slot: &mut f64,
        var_dvfbinr_p_rv_slot: &mut f64,
        var_dvsbnud_p_slot: &mut f64,
        var_dvsbnud_p_rv_slot: &mut f64,
        var_epsrox_p_slot: &mut f64,
        var_epsrox_p_rv_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_facneffac_p_rv_slot: &mut f64,
        var_fcgovacc_p_slot: &mut f64,
        var_fcgovacc_p_rv_slot: &mut f64,
        var_fcgovaccd_p_slot: &mut f64,
        var_fcgovaccd_p_rv_slot: &mut f64,
        var_fcinracc_p_slot: &mut f64,
        var_fcinracc_p_rv_slot: &mut f64,
        var_fcinrdep_p_slot: &mut f64,
        var_fcinrdep_p_rv_slot: &mut f64,
        var_fnt_p_slot: &mut f64,
        var_fnt_p_rv_slot: &mut f64,
        var_gc3ovd_p_slot: &mut f64,
        var_gc3ovd_p_rv_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_gfacnud_p_rv_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard33_rv_slot: &mut f64,
        var_guard34_slot: &mut f64,
        var_guard34_rv_slot: &mut f64,
        var_guard35_slot: &mut f64,
        var_guard35_rv_slot: &mut f64,
        var_guard36_slot: &mut f64,
        var_guard36_rv_slot: &mut f64,
        var_guard37_slot: &mut f64,
        var_guard37_rv_slot: &mut f64,
        var_guard38_slot: &mut f64,
        var_guard38_rv_slot: &mut f64,
        var_lpcke_slot: &mut f64,
        var_lpcke_rv_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_neff_p_rv_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_neffedge_p_rv_slot: &mut f64,
        var_npcke_slot: &mut f64,
        var_npcke_rv_slot: &mut f64,
        var_nsub_slot: &mut f64,
        var_nsub0e_slot: &mut f64,
        var_nsub0e_rv_slot: &mut f64,
        var_nsub_rv_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscebedge_p_rv_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_pscededge_p_rv_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_psceedge_p_rv_slot: &mut f64,
        var_st2vfb_p_slot: &mut f64,
        var_st2vfb_p_rv_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stbetedge_p_rv_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidl_p_rv_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stbgidld_p_rv_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_stvfb_p_rv_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_stvfbedge_p_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_tox_p_slot: &mut f64,
        var_tox_p_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
        var_vsbnud_p_rv_slot: &mut f64,
    ) {
        let mut var_aa: f64 = *var_aa_slot;
        let mut var_aa_rv: f64 = *var_aa_rv_slot;
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidl_p_rv: f64 = *var_agidl_p_rv_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_agidld_p_rv: f64 = *var_agidld_p_rv_slot;
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alp1ac_p_rv: f64 = *var_alp1ac_p_rv_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_alpac_p_rv: f64 = *var_alpac_p_rv_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axac_p_rv: f64 = *var_axac_p_rv_slot;
        let mut var_axinr_p: f64 = *var_axinr_p_slot;
        let mut var_axinr_p_rv: f64 = *var_axinr_p_rv_slot;
        let mut var_bb: f64 = *var_bb_slot;
        let mut var_bb_rv: f64 = *var_bb_rv_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_bgidl_p: f64 = *var_bgidl_p_slot;
        let mut var_bgidl_p_rv: f64 = *var_bgidl_p_rv_slot;
        let mut var_bgidld_p: f64 = *var_bgidld_p_slot;
        let mut var_bgidld_p_rv: f64 = *var_bgidld_p_rv_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfbedge_p_rv: f64 = *var_cfbedge_p_rv_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfdedge_p_rv: f64 = *var_cfdedge_p_rv_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfedge_p_rv: f64 = *var_cfedge_p_rv_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfr_p_rv: f64 = *var_cfr_p_rv_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cfrd_p_rv: f64 = *var_cfrd_p_rv_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgbov_p_rv: f64 = *var_cgbov_p_rv_slot;
        let mut var_cgidl_p: f64 = *var_cgidl_p_slot;
        let mut var_cgidl_p_rv: f64 = *var_cgidl_p_rv_slot;
        let mut var_cgidld_p: f64 = *var_cgidld_p_slot;
        let mut var_cgidld_p_rv: f64 = *var_cgidld_p_rv_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgov_p_rv: f64 = *var_cgov_p_rv_slot;
        let mut var_cgovaccg_p: f64 = *var_cgovaccg_p_slot;
        let mut var_cgovaccg_p_rv: f64 = *var_cgovaccg_p_rv_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_cgovd_p_rv: f64 = *var_cgovd_p_rv_slot;
        let mut var_chib_p: f64 = *var_chib_p_slot;
        let mut var_chib_p_rv: f64 = *var_chib_p_rv_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinr_p_rv: f64 = *var_cinr_p_rv_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_cinrd_p_rv: f64 = *var_cinrd_p_rv_slot;
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_cox_p_rv: f64 = *var_cox_p_rv_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_ctedge_p_rv: f64 = *var_ctedge_p_rv_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_delvtac_p_rv: f64 = *var_delvtac_p_rv_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dphib_p_rv: f64 = *var_dphib_p_rv_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_dphibedge_p_rv: f64 = *var_dphibedge_p_rv_slot;
        let mut var_dvfbinr_p: f64 = *var_dvfbinr_p_slot;
        let mut var_dvfbinr_p_rv: f64 = *var_dvfbinr_p_rv_slot;
        let mut var_dvsbnud_p: f64 = *var_dvsbnud_p_slot;
        let mut var_dvsbnud_p_rv: f64 = *var_dvsbnud_p_rv_slot;
        let mut var_epsrox_p: f64 = *var_epsrox_p_slot;
        let mut var_epsrox_p_rv: f64 = *var_epsrox_p_rv_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_facneffac_p_rv: f64 = *var_facneffac_p_rv_slot;
        let mut var_fcgovacc_p: f64 = *var_fcgovacc_p_slot;
        let mut var_fcgovacc_p_rv: f64 = *var_fcgovacc_p_rv_slot;
        let mut var_fcgovaccd_p: f64 = *var_fcgovaccd_p_slot;
        let mut var_fcgovaccd_p_rv: f64 = *var_fcgovaccd_p_rv_slot;
        let mut var_fcinracc_p: f64 = *var_fcinracc_p_slot;
        let mut var_fcinracc_p_rv: f64 = *var_fcinracc_p_rv_slot;
        let mut var_fcinrdep_p: f64 = *var_fcinrdep_p_slot;
        let mut var_fcinrdep_p_rv: f64 = *var_fcinrdep_p_rv_slot;
        let mut var_fnt_p: f64 = *var_fnt_p_slot;
        let mut var_fnt_p_rv: f64 = *var_fnt_p_rv_slot;
        let mut var_gc3ovd_p: f64 = *var_gc3ovd_p_slot;
        let mut var_gc3ovd_p_rv: f64 = *var_gc3ovd_p_rv_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_gfacnud_p_rv: f64 = *var_gfacnud_p_rv_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard33_rv: f64 = *var_guard33_rv_slot;
        let mut var_guard34: f64 = *var_guard34_slot;
        let mut var_guard34_rv: f64 = *var_guard34_rv_slot;
        let mut var_guard35: f64 = *var_guard35_slot;
        let mut var_guard35_rv: f64 = *var_guard35_rv_slot;
        let mut var_guard36: f64 = *var_guard36_slot;
        let mut var_guard36_rv: f64 = *var_guard36_rv_slot;
        let mut var_guard37: f64 = *var_guard37_slot;
        let mut var_guard37_rv: f64 = *var_guard37_rv_slot;
        let mut var_guard38: f64 = *var_guard38_slot;
        let mut var_guard38_rv: f64 = *var_guard38_rv_slot;
        let mut var_lpcke: f64 = *var_lpcke_slot;
        let mut var_lpcke_rv: f64 = *var_lpcke_rv_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_neff_p_rv: f64 = *var_neff_p_rv_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_neffedge_p_rv: f64 = *var_neffedge_p_rv_slot;
        let mut var_npcke: f64 = *var_npcke_slot;
        let mut var_npcke_rv: f64 = *var_npcke_rv_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
        let mut var_nsub0e: f64 = *var_nsub0e_slot;
        let mut var_nsub0e_rv: f64 = *var_nsub0e_rv_slot;
        let mut var_nsub_rv: f64 = *var_nsub_rv_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscebedge_p_rv: f64 = *var_pscebedge_p_rv_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_pscededge_p_rv: f64 = *var_pscededge_p_rv_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_psceedge_p_rv: f64 = *var_psceedge_p_rv_slot;
        let mut var_st2vfb_p: f64 = *var_st2vfb_p_slot;
        let mut var_st2vfb_p_rv: f64 = *var_st2vfb_p_rv_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stbetedge_p_rv: f64 = *var_stbetedge_p_rv_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidl_p_rv: f64 = *var_stbgidl_p_rv_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stbgidld_p_rv: f64 = *var_stbgidld_p_rv_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_stvfb_p_rv: f64 = *var_stvfb_p_rv_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_stvfbedge_p_rv: f64 = *var_stvfbedge_p_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_tox_p: f64 = *var_tox_p_slot;
        let mut var_tox_p_rv: f64 = *var_tox_p_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_vsbnud_p_rv: f64 = *var_vsbnud_p_rv_slot;

        let assign4570_e3765: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4570_e3767: f64 = if assign4570_e3765 == 1.0 { 1.0 } else { 0.0 };
        var_guard33 = assign4570_e3767;
        var_guard33_rv = 0.0;

        let (assign4580_e3771,) = {
    if (var_guard33 != 0.0) {
        (p.p124,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign4580_e3771;
        var_gc3ovd_p_rv = 0.0;

        var_chib_p = p.p125;
        var_chib_p_rv = 0.0;

        var_agidl_p = p.p126;
        var_agidl_p_rv = 0.0;

        var_agidld_p = p.p127;
        var_agidld_p_rv = 0.0;

        var_bgidl_p = p.p128;
        var_bgidl_p_rv = 0.0;

        var_bgidld_p = p.p129;
        var_bgidld_p_rv = 0.0;

        var_stbgidl_p = p.p130;
        var_stbgidl_p_rv = 0.0;

        var_stbgidld_p = p.p131;
        var_stbgidld_p_rv = 0.0;

        var_cgidl_p = p.p132;
        var_cgidl_p_rv = 0.0;

        var_cgidld_p = p.p133;
        var_cgidld_p_rv = 0.0;

        var_cox_p = p.p134;
        var_cox_p_rv = 0.0;

        var_delvtac_p = p.p135;
        var_delvtac_p_rv = 0.0;

        var_facneffac_p = p.p136;
        var_facneffac_p_rv = 0.0;

        var_thesatac_p = p.p98;
        var_thesatac_p_rv = 0.0;

        let assign4720_e3786: f64 = if param_given[137] { 1.0 } else { 0.0 };
        let assign4720_e3788: f64 = if assign4720_e3786 == 1.0 { 1.0 } else { 0.0 };
        var_guard34 = assign4720_e3788;
        var_guard34_rv = 0.0;

        let (assign4730_e3792,) = {
    if (var_guard34 != 0.0) {
        (p.p137,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign4730_e3792;
        var_thesatac_p_rv = 0.0;

        var_axac_p = p.p103;
        var_axac_p_rv = 0.0;

        let assign4750_e3795: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4750_e3797: f64 = if assign4750_e3795 == 1.0 { 1.0 } else { 0.0 };
        var_guard35 = assign4750_e3797;
        var_guard35_rv = 0.0;

        let (assign4760_e3801,) = {
    if (var_guard35 != 0.0) {
        (p.p138,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign4760_e3801;
        var_axac_p_rv = 0.0;

        var_alpac_p = p.p139;
        var_alpac_p_rv = 0.0;

        var_alp1ac_p = p.p140;
        var_alp1ac_p_rv = 0.0;

        var_cgov_p = p.p141;
        var_cgov_p_rv = 0.0;

        var_cgovd_p = p.p142;
        var_cgovd_p_rv = 0.0;

        var_fcgovacc_p = p.p143;
        var_fcgovacc_p_rv = 0.0;

        var_fcgovaccd_p = p.p144;
        var_fcgovaccd_p_rv = 0.0;

        var_cgovaccg_p = p.p145;
        var_cgovaccg_p_rv = 0.0;

        var_cgbov_p = p.p146;
        var_cgbov_p_rv = 0.0;

        var_cinr_p = p.p147;
        var_cinr_p_rv = 0.0;

        var_cinrd_p = p.p148;
        var_cinrd_p_rv = 0.0;

        var_dvfbinr_p = p.p149;
        var_dvfbinr_p_rv = 0.0;

        var_fcinrdep_p = p.p150;
        var_fcinrdep_p_rv = 0.0;

        var_fcinracc_p = p.p151;
        var_fcinracc_p_rv = 0.0;

        var_axinr_p = p.p152;
        var_axinr_p_rv = 0.0;

        var_cfr_p = p.p153;
        var_cfr_p_rv = 0.0;

        var_cfrd_p = p.p154;
        var_cfrd_p_rv = 0.0;

        var_fnt_p = p.p155;
        var_fnt_p_rv = 0.0;

        var_vfbedge_p = p.p161;
        var_vfbedge_p_rv = 0.0;

        var_stvfbedge_p = p.p162;
        var_stvfbedge_p_rv = 0.0;

        var_dphibedge_p = p.p163;
        var_dphibedge_p_rv = 0.0;

        var_neffedge_p = p.p164;
        var_neffedge_p_rv = 0.0;

        var_ctedge_p = p.p165;
        var_ctedge_p_rv = 0.0;

        var_betnedge_p = p.p166;
        var_betnedge_p_rv = 0.0;

        var_stbetedge_p = p.p167;
        var_stbetedge_p_rv = 0.0;

        var_psceedge_p = p.p168;
        var_psceedge_p_rv = 0.0;

        var_pscebedge_p = p.p169;
        var_pscebedge_p_rv = 0.0;

        var_pscededge_p = p.p170;
        var_pscededge_p_rv = 0.0;

        var_cfedge_p = p.p171;
        var_cfedge_p_rv = 0.0;

        var_cfdedge_p = p.p173;
        var_cfdedge_p_rv = 0.0;

        var_cfbedge_p = p.p172;
        var_cfbedge_p_rv = 0.0;

        let assign5240_e3851: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard36 = assign5240_e3851;
        var_guard36_rv = 0.0;

        let (assign5250_e3869,) = {
    if (var_guard36 != 0.0) {
        let assign5250_e3857: f64 = (var_ile).powf(p.p198);
        let assign5250_e3858: f64 = (p.p197 * assign5250_e3857);
        let assign5250_e3859: f64 = (p.p196 + assign5250_e3858);
        let assign5250_e3862: f64 = (p.p199 * var_iwe);
        let assign5250_e3863: f64 = (assign5250_e3859 + assign5250_e3862);
        let assign5250_e3866: f64 = (p.p200 * var_iae);
        let assign5250_e3867: f64 = (assign5250_e3863 + assign5250_e3866);
        (assign5250_e3867,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign5250_e3869;
        var_vfb_p_rv = 0.0;

        let (assign5260_e3885,) = {
    if (var_guard36 != 0.0) {
        let assign5260_e3874: f64 = (p.p202 * var_ile);
        let assign5260_e3875: f64 = (p.p201 + assign5260_e3874);
        let assign5260_e3878: f64 = (p.p203 * var_iwe);
        let assign5260_e3879: f64 = (assign5260_e3875 + assign5260_e3878);
        let assign5260_e3882: f64 = (p.p204 * var_iae);
        let assign5260_e3883: f64 = (assign5260_e3879 + assign5260_e3882);
        (assign5260_e3883,)
    } else {
        (var_stvfb_p,)
    }
};
        var_stvfb_p = assign5260_e3885;
        var_stvfb_p_rv = 0.0;

        let (assign5270_e3889,) = {
    if (var_guard36 != 0.0) {
        (p.p205,)
    } else {
        (var_st2vfb_p,)
    }
};
        var_st2vfb_p = assign5270_e3889;
        var_st2vfb_p_rv = 0.0;

        let (assign5280_e3893,) = {
    if (var_guard36 != 0.0) {
        (p.p206,)
    } else {
        (var_tox_p,)
    }
};
        var_tox_p = assign5280_e3893;
        var_tox_p_rv = 0.0;

        let (assign5290_e3897,) = {
    if (var_guard36 != 0.0) {
        (p.p207,)
    } else {
        (var_epsrox_p,)
    }
};
        var_epsrox_p = assign5290_e3897;
        var_epsrox_p_rv = 0.0;

        let (assign5300_e3930,) = {
    if (var_guard36 != 0.0) {
        let assign5300_e3903: f64 = (p.p209 * var_iwe);
        let assign5300_e3907: f64 = (var_we / p.p210);
        let assign5300_e3908: f64 = (1.0 + assign5300_e3907);
        let assign5300_e3909: f64 = (assign5300_e3908).ln();
        let assign5300_e3910: f64 = (assign5300_e3903 * assign5300_e3909);
        let assign5300_e3911: f64 = (1.0 + assign5300_e3910);
        let (assign5300_e3927,) = {
            if (assign5300_e3911 > 0.001) {
                let assign5300_e3917: f64 = (p.p209 * var_iwe);
                let assign5300_e3921: f64 = (var_we / p.p210);
                let assign5300_e3922: f64 = (1.0 + assign5300_e3921);
                let assign5300_e3923: f64 = (assign5300_e3922).ln();
                let assign5300_e3924: f64 = (assign5300_e3917 * assign5300_e3923);
                let assign5300_e3925: f64 = (1.0 + assign5300_e3924);
                (assign5300_e3925,)
            } else {
                (0.001,)
            }
        };
        let assign5300_e3928: f64 = (p.p208 * assign5300_e3927);
        (assign5300_e3928,)
    } else {
        (var_nsub0e,)
    }
};
        var_nsub0e = assign5300_e3930;
        var_nsub0e_rv = 0.0;

        let (assign5310_e3963,) = {
    if (var_guard36 != 0.0) {
        let assign5310_e3936: f64 = (p.p212 * var_iwe);
        let assign5310_e3940: f64 = (var_we / p.p213);
        let assign5310_e3941: f64 = (1.0 + assign5310_e3940);
        let assign5310_e3942: f64 = (assign5310_e3941).ln();
        let assign5310_e3943: f64 = (assign5310_e3936 * assign5310_e3942);
        let assign5310_e3944: f64 = (1.0 + assign5310_e3943);
        let (assign5310_e3960,) = {
            if (assign5310_e3944 > 0.001) {
                let assign5310_e3950: f64 = (p.p212 * var_iwe);
                let assign5310_e3954: f64 = (var_we / p.p213);
                let assign5310_e3955: f64 = (1.0 + assign5310_e3954);
                let assign5310_e3956: f64 = (assign5310_e3955).ln();
                let assign5310_e3957: f64 = (assign5310_e3950 * assign5310_e3956);
                let assign5310_e3958: f64 = (1.0 + assign5310_e3957);
                (assign5310_e3958,)
            } else {
                (0.001,)
            }
        };
        let assign5310_e3961: f64 = (p.p211 * assign5310_e3960);
        (assign5310_e3961,)
    } else {
        (var_npcke,)
    }
};
        var_npcke = assign5310_e3963;
        var_npcke_rv = 0.0;

        let (assign5320_e3996,) = {
    if (var_guard36 != 0.0) {
        let assign5320_e3969: f64 = (p.p215 * var_iwe);
        let assign5320_e3973: f64 = (var_we / p.p213);
        let assign5320_e3974: f64 = (1.0 + assign5320_e3973);
        let assign5320_e3975: f64 = (assign5320_e3974).ln();
        let assign5320_e3976: f64 = (assign5320_e3969 * assign5320_e3975);
        let assign5320_e3977: f64 = (1.0 + assign5320_e3976);
        let (assign5320_e3993,) = {
            if (assign5320_e3977 > 0.001) {
                let assign5320_e3983: f64 = (p.p215 * var_iwe);
                let assign5320_e3987: f64 = (var_we / p.p213);
                let assign5320_e3988: f64 = (1.0 + assign5320_e3987);
                let assign5320_e3989: f64 = (assign5320_e3988).ln();
                let assign5320_e3990: f64 = (assign5320_e3983 * assign5320_e3989);
                let assign5320_e3991: f64 = (1.0 + assign5320_e3990);
                (assign5320_e3991,)
            } else {
                (0.001,)
            }
        };
        let assign5320_e3994: f64 = (p.p214 * assign5320_e3993);
        (assign5320_e3994,)
    } else {
        (var_lpcke,)
    }
};
        var_lpcke = assign5320_e3996;
        var_lpcke_rv = 0.0;

        let assign5330_e4000: f64 = (2.0 * var_lpcke);
        let assign5330_e4001: f64 = if var_le > assign5330_e4000 { 1.0 } else { 0.0 };
        var_guard37 = assign5330_e4001;
        var_guard37_rv = 0.0;

        let (assign5340_e4007,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        (75000000000.0,)
    } else {
        (var_aa,)
    }
};
        var_aa = assign5340_e4007;
        var_aa_rv = 0.0;

        let (assign5350_e4021,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        let assign5350_e4014: f64 = (0.5 * var_npcke);
        let assign5350_e4015: f64 = (var_nsub0e + assign5350_e4014);
        let assign5350_e4016: f64 = (assign5350_e4015).sqrt();
        let assign5350_e4018: f64 = (var_nsub0e).sqrt();
        let assign5350_e4019: f64 = (assign5350_e4016 - assign5350_e4018);
        (assign5350_e4019,)
    } else {
        (var_bb,)
    }
};
        var_bb = assign5350_e4021;
        var_bb_rv = 0.0;

        let (assign5360_e4046,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        let assign5360_e4026: f64 = (var_nsub0e).sqrt();
        let assign5360_e4031: f64 = (2.0 * var_lpcke);
        let assign5360_e4033: f64 = (assign5360_e4031 / var_le);
        let assign5360_e4036: f64 = (var_bb / var_aa);
        let assign5360_e4037: f64 = (assign5360_e4036).exp();
        let assign5360_e4039: f64 = (assign5360_e4037 - 1.0);
        let assign5360_e4040: f64 = (assign5360_e4033 * assign5360_e4039);
        let assign5360_e4041: f64 = (1.0 + assign5360_e4040);
        let assign5360_e4042: f64 = (assign5360_e4041).ln();
        let assign5360_e4043: f64 = (var_aa * assign5360_e4042);
        let assign5360_e4044: f64 = (assign5360_e4026 + assign5360_e4043);
        (assign5360_e4044,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5360_e4046;
        var_nsub_rv = 0.0;

        let (assign5370_e4054,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        let assign5370_e4052: f64 = (var_nsub * var_nsub);
        (assign5370_e4052,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5370_e4054;
        var_nsub_rv = 0.0;

        let assign5380_e4057: f64 = if var_le >= var_lpcke { 1.0 } else { 0.0 };
        var_guard38 = assign5380_e4057;
        var_guard38_rv = 0.0;

        let (assign5390_e4072,) = {
    if (((var_guard36 != 0.0) && (var_guard37 == 0.0)) && (var_guard38 != 0.0)) {
        let assign5390_e4067: f64 = (var_npcke * var_lpcke);
        let assign5390_e4069: f64 = (assign5390_e4067 / var_le);
        let assign5390_e4070: f64 = (var_nsub0e + assign5390_e4069);
        (assign5390_e4070,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5390_e4072;
        var_nsub_rv = 0.0;

        let (assign5400_e4090,) = {
    if (((var_guard36 != 0.0) && (var_guard37 == 0.0)) && (var_guard38 == 0.0)) {
        let assign5400_e4085: f64 = (var_le / var_lpcke);
        let assign5400_e4086: f64 = (2.0 - assign5400_e4085);
        let assign5400_e4087: f64 = (var_npcke * assign5400_e4086);
        let assign5400_e4088: f64 = (var_nsub0e + assign5400_e4087);
        (assign5400_e4088,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5400_e4090;
        var_nsub_rv = 0.0;

        let (assign5410_e4104,) = {
    if (var_guard36 != 0.0) {
        let assign5410_e4096: f64 = (p.p216 * var_ile);
        let assign5410_e4097: f64 = (1.0 - assign5410_e4096);
        let assign5410_e4100: f64 = (p.p217 * var_ile2);
        let assign5410_e4101: f64 = (assign5410_e4097 - assign5410_e4100);
        let assign5410_e4102: f64 = (var_nsub * assign5410_e4101);
        (assign5410_e4102,)
    } else {
        (var_neff_p,)
    }
};
        var_neff_p = assign5410_e4104;
        var_neff_p_rv = 0.0;

        let (assign5420_e4122,) = {
    if (var_guard36 != 0.0) {
        let assign5420_e4110: f64 = (var_ile).powf(p.p220);
        let assign5420_e4111: f64 = (p.p219 * assign5420_e4110);
        let assign5420_e4112: f64 = (p.p218 + assign5420_e4111);
        let assign5420_e4115: f64 = (p.p221 * var_iwe);
        let assign5420_e4116: f64 = (assign5420_e4112 + assign5420_e4115);
        let assign5420_e4119: f64 = (p.p222 * var_iae);
        let assign5420_e4120: f64 = (assign5420_e4116 + assign5420_e4119);
        (assign5420_e4120,)
    } else {
        (var_gfacnud_p,)
    }
};
        var_gfacnud_p = assign5420_e4122;
        var_gfacnud_p_rv = 0.0;

        let (assign5430_e4126,) = {
    if (var_guard36 != 0.0) {
        (p.p223,)
    } else {
        (var_vsbnud_p,)
    }
};
        var_vsbnud_p = assign5430_e4126;
        var_vsbnud_p_rv = 0.0;

        let (assign5440_e4130,) = {
    if (var_guard36 != 0.0) {
        (p.p224,)
    } else {
        (var_dvsbnud_p,)
    }
};
        var_dvsbnud_p = assign5440_e4130;
        var_dvsbnud_p_rv = 0.0;

        let (assign5450_e4148,) = {
    if (var_guard36 != 0.0) {
        let assign5450_e4136: f64 = (var_ile).powf(p.p227);
        let assign5450_e4137: f64 = (p.p226 * assign5450_e4136);
        let assign5450_e4138: f64 = (p.p225 + assign5450_e4137);
        let assign5450_e4141: f64 = (p.p228 * var_iwe);
        let assign5450_e4142: f64 = (assign5450_e4138 + assign5450_e4141);
        let assign5450_e4145: f64 = (p.p229 * var_iae);
        let assign5450_e4146: f64 = (assign5450_e4142 + assign5450_e4145);
        (assign5450_e4146,)
    } else {
        (var_dphib_p,)
    }
};
        var_dphib_p = assign5450_e4148;
        var_dphib_p_rv = 0.0;

        *var_aa_slot = var_aa;
        *var_aa_rv_slot = var_aa_rv;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidl_p_rv_slot = var_agidl_p_rv;
        *var_agidld_p_slot = var_agidld_p;
        *var_agidld_p_rv_slot = var_agidld_p_rv;
        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alp1ac_p_rv_slot = var_alp1ac_p_rv;
        *var_alpac_p_slot = var_alpac_p;
        *var_alpac_p_rv_slot = var_alpac_p_rv;
        *var_axac_p_slot = var_axac_p;
        *var_axac_p_rv_slot = var_axac_p_rv;
        *var_axinr_p_slot = var_axinr_p;
        *var_axinr_p_rv_slot = var_axinr_p_rv;
        *var_bb_slot = var_bb;
        *var_bb_rv_slot = var_bb_rv;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_bgidl_p_slot = var_bgidl_p;
        *var_bgidl_p_rv_slot = var_bgidl_p_rv;
        *var_bgidld_p_slot = var_bgidld_p;
        *var_bgidld_p_rv_slot = var_bgidld_p_rv;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfbedge_p_rv_slot = var_cfbedge_p_rv;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfdedge_p_rv_slot = var_cfdedge_p_rv;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfedge_p_rv_slot = var_cfedge_p_rv;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfr_p_rv_slot = var_cfr_p_rv;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cfrd_p_rv_slot = var_cfrd_p_rv;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgbov_p_rv_slot = var_cgbov_p_rv;
        *var_cgidl_p_slot = var_cgidl_p;
        *var_cgidl_p_rv_slot = var_cgidl_p_rv;
        *var_cgidld_p_slot = var_cgidld_p;
        *var_cgidld_p_rv_slot = var_cgidld_p_rv;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgov_p_rv_slot = var_cgov_p_rv;
        *var_cgovaccg_p_slot = var_cgovaccg_p;
        *var_cgovaccg_p_rv_slot = var_cgovaccg_p_rv;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_cgovd_p_rv_slot = var_cgovd_p_rv;
        *var_chib_p_slot = var_chib_p;
        *var_chib_p_rv_slot = var_chib_p_rv;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinr_p_rv_slot = var_cinr_p_rv;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_cinrd_p_rv_slot = var_cinrd_p_rv;
        *var_cox_p_slot = var_cox_p;
        *var_cox_p_rv_slot = var_cox_p_rv;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_ctedge_p_rv_slot = var_ctedge_p_rv;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_delvtac_p_rv_slot = var_delvtac_p_rv;
        *var_dphib_p_slot = var_dphib_p;
        *var_dphib_p_rv_slot = var_dphib_p_rv;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_dphibedge_p_rv_slot = var_dphibedge_p_rv;
        *var_dvfbinr_p_slot = var_dvfbinr_p;
        *var_dvfbinr_p_rv_slot = var_dvfbinr_p_rv;
        *var_dvsbnud_p_slot = var_dvsbnud_p;
        *var_dvsbnud_p_rv_slot = var_dvsbnud_p_rv;
        *var_epsrox_p_slot = var_epsrox_p;
        *var_epsrox_p_rv_slot = var_epsrox_p_rv;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_facneffac_p_rv_slot = var_facneffac_p_rv;
        *var_fcgovacc_p_slot = var_fcgovacc_p;
        *var_fcgovacc_p_rv_slot = var_fcgovacc_p_rv;
        *var_fcgovaccd_p_slot = var_fcgovaccd_p;
        *var_fcgovaccd_p_rv_slot = var_fcgovaccd_p_rv;
        *var_fcinracc_p_slot = var_fcinracc_p;
        *var_fcinracc_p_rv_slot = var_fcinracc_p_rv;
        *var_fcinrdep_p_slot = var_fcinrdep_p;
        *var_fcinrdep_p_rv_slot = var_fcinrdep_p_rv;
        *var_fnt_p_slot = var_fnt_p;
        *var_fnt_p_rv_slot = var_fnt_p_rv;
        *var_gc3ovd_p_slot = var_gc3ovd_p;
        *var_gc3ovd_p_rv_slot = var_gc3ovd_p_rv;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_gfacnud_p_rv_slot = var_gfacnud_p_rv;
        *var_guard33_slot = var_guard33;
        *var_guard33_rv_slot = var_guard33_rv;
        *var_guard34_slot = var_guard34;
        *var_guard34_rv_slot = var_guard34_rv;
        *var_guard35_slot = var_guard35;
        *var_guard35_rv_slot = var_guard35_rv;
        *var_guard36_slot = var_guard36;
        *var_guard36_rv_slot = var_guard36_rv;
        *var_guard37_slot = var_guard37;
        *var_guard37_rv_slot = var_guard37_rv;
        *var_guard38_slot = var_guard38;
        *var_guard38_rv_slot = var_guard38_rv;
        *var_lpcke_slot = var_lpcke;
        *var_lpcke_rv_slot = var_lpcke_rv;
        *var_neff_p_slot = var_neff_p;
        *var_neff_p_rv_slot = var_neff_p_rv;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_neffedge_p_rv_slot = var_neffedge_p_rv;
        *var_npcke_slot = var_npcke;
        *var_npcke_rv_slot = var_npcke_rv;
        *var_nsub_slot = var_nsub;
        *var_nsub0e_slot = var_nsub0e;
        *var_nsub0e_rv_slot = var_nsub0e_rv;
        *var_nsub_rv_slot = var_nsub_rv;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscebedge_p_rv_slot = var_pscebedge_p_rv;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_pscededge_p_rv_slot = var_pscededge_p_rv;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_psceedge_p_rv_slot = var_psceedge_p_rv;
        *var_st2vfb_p_slot = var_st2vfb_p;
        *var_st2vfb_p_rv_slot = var_st2vfb_p_rv;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stbetedge_p_rv_slot = var_stbetedge_p_rv;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidl_p_rv_slot = var_stbgidl_p_rv;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stbgidld_p_rv_slot = var_stbgidld_p_rv;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_stvfb_p_rv_slot = var_stvfb_p_rv;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_stvfbedge_p_rv_slot = var_stvfbedge_p_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_tox_p_slot = var_tox_p;
        *var_tox_p_rv_slot = var_tox_p_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
        *var_vsbnud_p_slot = var_vsbnud_p;
        *var_vsbnud_p_rv_slot = var_vsbnud_p_rv;
    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        var_guard36: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfb_p_rv_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_cfd_p_rv_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_cs_p_rv_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ct_p_rv_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctb_p_rv_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_ctg_p_rv_slot: &mut f64,
        var_fbet1e_slot: &mut f64,
        var_fbet1e_rv_slot: &mut f64,
        var_feta_p_slot: &mut f64,
        var_feta_p_rv_slot: &mut f64,
        var_gpe_slot: &mut f64,
        var_gpe_rv_slot: &mut f64,
        var_gwe_slot: &mut f64,
        var_gwe_rv_slot: &mut f64,
        var_lp1e_slot: &mut f64,
        var_lp1e_rv_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_mue_p_rv_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_nov_p_rv_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_novd_p_rv_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_np_p_rv_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psce_p_rv_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psceb_p_rv_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_psced_p_rv_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rs_p_rv_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsb_p_rv_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_rsg_p_rv_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stbet_p_rv_slot: &mut f64,
        var_stcs_p_slot: &mut f64,
        var_stcs_p_rv_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stct_p_rv_slot: &mut f64,
        var_stmue_p_slot: &mut f64,
        var_stmue_p_rv_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_strs_p_rv_slot: &mut f64,
        var_stthecs_p_slot: &mut f64,
        var_stthecs_p_rv_slot: &mut f64,
        var_stthemu_p_slot: &mut f64,
        var_stthemu_p_rv_slot: &mut f64,
        var_stxcor_p_slot: &mut f64,
        var_stxcor_p_rv_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_thecs_p_rv_slot: &mut f64,
        var_themu_p_slot: &mut f64,
        var_themu_p_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_toxov_p_slot: &mut f64,
        var_toxov_p_rv_slot: &mut f64,
        var_toxovd_p_slot: &mut f64,
        var_toxovd_p_rv_slot: &mut f64,
        var_xcor_p_slot: &mut f64,
        var_xcor_p_rv_slot: &mut f64,
    ) {
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfb_p_rv: f64 = *var_cfb_p_rv_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_cfd_p_rv: f64 = *var_cfd_p_rv_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_cs_p_rv: f64 = *var_cs_p_rv_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ct_p_rv: f64 = *var_ct_p_rv_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctb_p_rv: f64 = *var_ctb_p_rv_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_ctg_p_rv: f64 = *var_ctg_p_rv_slot;
        let mut var_fbet1e: f64 = *var_fbet1e_slot;
        let mut var_fbet1e_rv: f64 = *var_fbet1e_rv_slot;
        let mut var_feta_p: f64 = *var_feta_p_slot;
        let mut var_feta_p_rv: f64 = *var_feta_p_rv_slot;
        let mut var_gpe: f64 = *var_gpe_slot;
        let mut var_gpe_rv: f64 = *var_gpe_rv_slot;
        let mut var_gwe: f64 = *var_gwe_slot;
        let mut var_gwe_rv: f64 = *var_gwe_rv_slot;
        let mut var_lp1e: f64 = *var_lp1e_slot;
        let mut var_lp1e_rv: f64 = *var_lp1e_rv_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_mue_p_rv: f64 = *var_mue_p_rv_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_nov_p_rv: f64 = *var_nov_p_rv_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_novd_p_rv: f64 = *var_novd_p_rv_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_np_p_rv: f64 = *var_np_p_rv_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psce_p_rv: f64 = *var_psce_p_rv_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psceb_p_rv: f64 = *var_psceb_p_rv_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_psced_p_rv: f64 = *var_psced_p_rv_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rs_p_rv: f64 = *var_rs_p_rv_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsb_p_rv: f64 = *var_rsb_p_rv_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_rsg_p_rv: f64 = *var_rsg_p_rv_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stbet_p_rv: f64 = *var_stbet_p_rv_slot;
        let mut var_stcs_p: f64 = *var_stcs_p_slot;
        let mut var_stcs_p_rv: f64 = *var_stcs_p_rv_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stct_p_rv: f64 = *var_stct_p_rv_slot;
        let mut var_stmue_p: f64 = *var_stmue_p_slot;
        let mut var_stmue_p_rv: f64 = *var_stmue_p_rv_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_strs_p_rv: f64 = *var_strs_p_rv_slot;
        let mut var_stthecs_p: f64 = *var_stthecs_p_slot;
        let mut var_stthecs_p_rv: f64 = *var_stthecs_p_rv_slot;
        let mut var_stthemu_p: f64 = *var_stthemu_p_slot;
        let mut var_stthemu_p_rv: f64 = *var_stthemu_p_rv_slot;
        let mut var_stxcor_p: f64 = *var_stxcor_p_slot;
        let mut var_stxcor_p_rv: f64 = *var_stxcor_p_rv_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_thecs_p_rv: f64 = *var_thecs_p_rv_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_themu_p_rv: f64 = *var_themu_p_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_toxov_p: f64 = *var_toxov_p_slot;
        let mut var_toxov_p_rv: f64 = *var_toxov_p_rv_slot;
        let mut var_toxovd_p: f64 = *var_toxovd_p_slot;
        let mut var_toxovd_p_rv: f64 = *var_toxovd_p_rv_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;
        let mut var_xcor_p_rv: f64 = *var_xcor_p_rv_slot;

        let (assign5460_e4167,) = {
    if (var_guard36 != 0.0) {
        let assign5460_e4155: f64 = (p.p231 * var_ile);
        let assign5460_e4156: f64 = (1.0 + assign5460_e4155);
        let (assign5460_e4164,) = {
            if (1e-6 > assign5460_e4156) {
                (1e-6,)
            } else {
                let assign5460_e4162: f64 = (p.p231 * var_ile);
                let assign5460_e4163: f64 = (1.0 + assign5460_e4162);
                (assign5460_e4163,)
            }
        };
        let assign5460_e4165: f64 = (p.p230 * assign5460_e4164);
        (assign5460_e4165,)
    } else {
        (var_np_p,)
    }
};
        var_np_p = assign5460_e4167;
        var_np_p_rv = 0.0;

        let (assign5470_e4171,) = {
    if (var_guard36 != 0.0) {
        (p.p232,)
    } else {
        (var_toxov_p,)
    }
};
        var_toxov_p = assign5470_e4171;
        var_toxov_p_rv = 0.0;

        let (assign5480_e4175,) = {
    if (var_guard36 != 0.0) {
        (p.p233,)
    } else {
        (var_toxovd_p,)
    }
};
        var_toxovd_p = assign5480_e4175;
        var_toxovd_p_rv = 0.0;

        let (assign5490_e4179,) = {
    if (var_guard36 != 0.0) {
        (p.p236,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign5490_e4179;
        var_nov_p_rv = 0.0;

        let (assign5500_e4183,) = {
    if (var_guard36 != 0.0) {
        (p.p237,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign5500_e4183;
        var_novd_p_rv = 0.0;

        let (assign5510_e4205,) = {
    if (var_guard36 != 0.0) {
        let assign5510_e4189: f64 = (var_ile).powf(p.p240);
        let assign5510_e4190: f64 = (p.p239 * assign5510_e4189);
        let assign5510_e4191: f64 = (p.p238 + assign5510_e4190);
        let assign5510_e4195: f64 = (p.p241 * var_iwe);
        let assign5510_e4196: f64 = (1.0 + assign5510_e4195);
        let assign5510_e4197: f64 = (assign5510_e4191 * assign5510_e4196);
        let assign5510_e4201: f64 = (p.p242 * var_iae);
        let assign5510_e4202: f64 = (1.0 + assign5510_e4201);
        let assign5510_e4203: f64 = (assign5510_e4197 * assign5510_e4202);
        (assign5510_e4203,)
    } else {
        (var_ct_p,)
    }
};
        var_ct_p = assign5510_e4205;
        var_ct_p_rv = 0.0;

        let (assign5520_e4209,) = {
    if (var_guard36 != 0.0) {
        (p.p244,)
    } else {
        (var_ctg_p,)
    }
};
        var_ctg_p = assign5520_e4209;
        var_ctg_p_rv = 0.0;

        let (assign5530_e4213,) = {
    if (var_guard36 != 0.0) {
        (p.p243,)
    } else {
        (var_ctb_p,)
    }
};
        var_ctb_p = assign5530_e4213;
        var_ctb_p_rv = 0.0;

        let (assign5540_e4217,) = {
    if (var_guard36 != 0.0) {
        (p.p245,)
    } else {
        (var_stct_p,)
    }
};
        var_stct_p = assign5540_e4217;
        var_stct_p_rv = 0.0;

        let (assign5550_e4231,) = {
    if (var_guard36 != 0.0) {
        let assign5550_e4222: f64 = (var_ile).powf(p.p247);
        let assign5550_e4223: f64 = (p.p246 * assign5550_e4222);
        let assign5550_e4227: f64 = (p.p248 * var_iwe);
        let assign5550_e4228: f64 = (1.0 + assign5550_e4227);
        let assign5550_e4229: f64 = (assign5550_e4223 * assign5550_e4228);
        (assign5550_e4229,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign5550_e4231;
        var_cf_p_rv = 0.0;

        let (assign5560_e4235,) = {
    if (var_guard36 != 0.0) {
        (p.p250,)
    } else {
        (var_cfd_p,)
    }
};
        var_cfd_p = assign5560_e4235;
        var_cfd_p_rv = 0.0;

        let (assign5570_e4239,) = {
    if (var_guard36 != 0.0) {
        (p.p249,)
    } else {
        (var_cfb_p,)
    }
};
        var_cfb_p = assign5570_e4239;
        var_cfb_p_rv = 0.0;

        let (assign5580_e4253,) = {
    if (var_guard36 != 0.0) {
        let assign5580_e4244: f64 = (var_ile).powf(p.p252);
        let assign5580_e4245: f64 = (p.p251 * assign5580_e4244);
        let assign5580_e4249: f64 = (p.p253 * var_iwe);
        let assign5580_e4250: f64 = (1.0 + assign5580_e4249);
        let assign5580_e4251: f64 = (assign5580_e4245 * assign5580_e4250);
        (assign5580_e4251,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign5580_e4253;
        var_psce_p_rv = 0.0;

        let (assign5590_e4257,) = {
    if (var_guard36 != 0.0) {
        (p.p255,)
    } else {
        (var_psced_p,)
    }
};
        var_psced_p = assign5590_e4257;
        var_psced_p_rv = 0.0;

        let (assign5600_e4261,) = {
    if (var_guard36 != 0.0) {
        (p.p254,)
    } else {
        (var_psceb_p,)
    }
};
        var_psceb_p = assign5600_e4261;
        var_psceb_p_rv = 0.0;

        let (assign5610_e4271,) = {
    if (var_guard36 != 0.0) {
        let assign5610_e4267: f64 = (p.p258 * var_iwe);
        let assign5610_e4268: f64 = (1.0 + assign5610_e4267);
        let assign5610_e4269: f64 = (p.p257 * assign5610_e4268);
        (assign5610_e4269,)
    } else {
        (var_fbet1e,)
    }
};
        var_fbet1e = assign5610_e4271;
        var_fbet1e_rv = 0.0;

        let (assign5620_e4290,) = {
    if (var_guard36 != 0.0) {
        let assign5620_e4277: f64 = (p.p260 * var_iwe);
        let assign5620_e4278: f64 = (1.0 + assign5620_e4277);
        let (assign5620_e4287,) = {
            if (assign5620_e4278 > 0.001) {
                let assign5620_e4284: f64 = (p.p260 * var_iwe);
                let assign5620_e4285: f64 = (1.0 + assign5620_e4284);
                (assign5620_e4285,)
            } else {
                (0.001,)
            }
        };
        let assign5620_e4288: f64 = (p.p259 * assign5620_e4287);
        (assign5620_e4288,)
    } else {
        (var_lp1e,)
    }
};
        var_lp1e = assign5620_e4290;
        var_lp1e_rv = 0.0;

        let (assign5630_e4322,) = {
    if (var_guard36 != 0.0) {
        let assign5630_e4295: f64 = (var_fbet1e * var_lp1e);
        let assign5630_e4297: f64 = (assign5630_e4295 / var_le);
        let assign5630_e4300: f64 = (-var_le);
        let assign5630_e4302: f64 = (assign5630_e4300 / var_lp1e);
        let assign5630_e4303: f64 = (assign5630_e4302).exp();
        let assign5630_e4304: f64 = (1.0 - assign5630_e4303);
        let assign5630_e4305: f64 = (assign5630_e4297 * assign5630_e4304);
        let assign5630_e4306: f64 = (1.0 + assign5630_e4305);
        let assign5630_e4309: f64 = (p.p261 * p.p262);
        let assign5630_e4311: f64 = (assign5630_e4309 / var_le);
        let assign5630_e4314: f64 = (-var_le);
        let assign5630_e4316: f64 = (assign5630_e4314 / p.p262);
        let assign5630_e4317: f64 = (assign5630_e4316).exp();
        let assign5630_e4318: f64 = (1.0 - assign5630_e4317);
        let assign5630_e4319: f64 = (assign5630_e4311 * assign5630_e4318);
        let assign5630_e4320: f64 = (assign5630_e4306 + assign5630_e4319);
        (assign5630_e4320,)
    } else {
        (var_gpe,)
    }
};
        var_gpe = assign5630_e4322;
        var_gpe_rv = 0.0;

        let (assign5640_e4331,) = {
    if (var_guard36 != 0.0) {
        let (assign5640_e4329,) = {
            if (var_gpe > 1e-15) {
                (var_gpe,)
            } else {
                (1e-15,)
            }
        };
        (assign5640_e4329,)
    } else {
        (var_gpe,)
    }
};
        var_gpe = assign5640_e4331;
        var_gpe_rv = 0.0;

        let (assign5650_e4350,) = {
    if (var_guard36 != 0.0) {
        let assign5650_e4336: f64 = (p.p263 * var_iwe);
        let assign5650_e4337: f64 = (1.0 + assign5650_e4336);
        let assign5650_e4340: f64 = (p.p264 * var_iwe);
        let assign5650_e4344: f64 = (var_we / p.p265);
        let assign5650_e4345: f64 = (1.0 + assign5650_e4344);
        let assign5650_e4346: f64 = (assign5650_e4345).ln();
        let assign5650_e4347: f64 = (assign5650_e4340 * assign5650_e4346);
        let assign5650_e4348: f64 = (assign5650_e4337 + assign5650_e4347);
        (assign5650_e4348,)
    } else {
        (var_gwe,)
    }
};
        var_gwe = assign5650_e4350;
        var_gwe_rv = 0.0;

        let (assign5660_e4362,) = {
    if (var_guard36 != 0.0) {
        let assign5660_e4354: f64 = (p.p256 * var_we);
        let assign5660_e4357: f64 = (var_gpe * var_le);
        let assign5660_e4358: f64 = (assign5660_e4354 / assign5660_e4357);
        let assign5660_e4360: f64 = (assign5660_e4358 * var_gwe);
        (assign5660_e4360,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign5660_e4362;
        var_betn_p_rv = 0.0;

        let (assign5670_e4378,) = {
    if (var_guard36 != 0.0) {
        let assign5670_e4367: f64 = (p.p267 * var_ile);
        let assign5670_e4368: f64 = (p.p266 + assign5670_e4367);
        let assign5670_e4371: f64 = (p.p268 * var_iwe);
        let assign5670_e4372: f64 = (assign5670_e4368 + assign5670_e4371);
        let assign5670_e4375: f64 = (p.p269 * var_iae);
        let assign5670_e4376: f64 = (assign5670_e4372 + assign5670_e4375);
        (assign5670_e4376,)
    } else {
        (var_stbet_p,)
    }
};
        var_stbet_p = assign5670_e4378;
        var_stbet_p_rv = 0.0;

        let (assign5680_e4388,) = {
    if (var_guard36 != 0.0) {
        let assign5680_e4384: f64 = (p.p271 * var_iwe);
        let assign5680_e4385: f64 = (1.0 + assign5680_e4384);
        let assign5680_e4386: f64 = (p.p270 * assign5680_e4385);
        (assign5680_e4386,)
    } else {
        (var_mue_p,)
    }
};
        var_mue_p = assign5680_e4388;
        var_mue_p_rv = 0.0;

        let (assign5690_e4392,) = {
    if (var_guard36 != 0.0) {
        (p.p272,)
    } else {
        (var_stmue_p,)
    }
};
        var_stmue_p = assign5690_e4392;
        var_stmue_p_rv = 0.0;

        let (assign5700_e4396,) = {
    if (var_guard36 != 0.0) {
        (p.p273,)
    } else {
        (var_themu_p,)
    }
};
        var_themu_p = assign5700_e4396;
        var_themu_p_rv = 0.0;

        let (assign5710_e4400,) = {
    if (var_guard36 != 0.0) {
        (p.p274,)
    } else {
        (var_stthemu_p,)
    }
};
        var_stthemu_p = assign5710_e4400;
        var_stthemu_p_rv = 0.0;

        let (assign5720_e4422,) = {
    if (var_guard36 != 0.0) {
        let assign5720_e4406: f64 = (var_ile).powf(p.p277);
        let assign5720_e4407: f64 = (p.p276 * assign5720_e4406);
        let assign5720_e4408: f64 = (p.p275 + assign5720_e4407);
        let assign5720_e4412: f64 = (p.p278 * var_iwe);
        let assign5720_e4413: f64 = (1.0 + assign5720_e4412);
        let assign5720_e4414: f64 = (assign5720_e4408 * assign5720_e4413);
        let assign5720_e4418: f64 = (p.p279 * var_iae);
        let assign5720_e4419: f64 = (1.0 + assign5720_e4418);
        let assign5720_e4420: f64 = (assign5720_e4414 * assign5720_e4419);
        (assign5720_e4420,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign5720_e4422;
        var_cs_p_rv = 0.0;

        let (assign5730_e4426,) = {
    if (var_guard36 != 0.0) {
        (p.p280,)
    } else {
        (var_stcs_p,)
    }
};
        var_stcs_p = assign5730_e4426;
        var_stcs_p_rv = 0.0;

        let (assign5740_e4430,) = {
    if (var_guard36 != 0.0) {
        (p.p281,)
    } else {
        (var_thecs_p,)
    }
};
        var_thecs_p = assign5740_e4430;
        var_thecs_p_rv = 0.0;

        let (assign5750_e4434,) = {
    if (var_guard36 != 0.0) {
        (p.p282,)
    } else {
        (var_stthecs_p,)
    }
};
        var_stthecs_p = assign5750_e4434;
        var_stthecs_p_rv = 0.0;

        let (assign5760_e4456,) = {
    if (var_guard36 != 0.0) {
        let assign5760_e4440: f64 = (p.p284 * var_ile);
        let assign5760_e4441: f64 = (1.0 + assign5760_e4440);
        let assign5760_e4442: f64 = (p.p283 * assign5760_e4441);
        let assign5760_e4446: f64 = (p.p285 * var_iwe);
        let assign5760_e4447: f64 = (1.0 + assign5760_e4446);
        let assign5760_e4448: f64 = (assign5760_e4442 * assign5760_e4447);
        let assign5760_e4452: f64 = (p.p286 * var_iae);
        let assign5760_e4453: f64 = (1.0 + assign5760_e4452);
        let assign5760_e4454: f64 = (assign5760_e4448 * assign5760_e4453);
        (assign5760_e4454,)
    } else {
        (var_xcor_p,)
    }
};
        var_xcor_p = assign5760_e4456;
        var_xcor_p_rv = 0.0;

        let (assign5770_e4460,) = {
    if (var_guard36 != 0.0) {
        (p.p287,)
    } else {
        (var_stxcor_p,)
    }
};
        var_stxcor_p = assign5770_e4460;
        var_stxcor_p_rv = 0.0;

        let (assign5780_e4464,) = {
    if (var_guard36 != 0.0) {
        (p.p288,)
    } else {
        (var_feta_p,)
    }
};
        var_feta_p = assign5780_e4464;
        var_feta_p_rv = 0.0;

        let (assign5790_e4476,) = {
    if (var_guard36 != 0.0) {
        let assign5790_e4468: f64 = (p.p289 * var_iwe);
        let assign5790_e4472: f64 = (p.p290 * var_iwe);
        let assign5790_e4473: f64 = (1.0 + assign5790_e4472);
        let assign5790_e4474: f64 = (assign5790_e4468 * assign5790_e4473);
        (assign5790_e4474,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign5790_e4476;
        var_rs_p_rv = 0.0;

        let (assign5800_e4480,) = {
    if (var_guard36 != 0.0) {
        (p.p291,)
    } else {
        (var_strs_p,)
    }
};
        var_strs_p = assign5800_e4480;
        var_strs_p_rv = 0.0;

        let (assign5810_e4484,) = {
    if (var_guard36 != 0.0) {
        (p.p292,)
    } else {
        (var_rsb_p,)
    }
};
        var_rsb_p = assign5810_e4484;
        var_rsb_p_rv = 0.0;

        let (assign5820_e4488,) = {
    if (var_guard36 != 0.0) {
        (p.p293,)
    } else {
        (var_rsg_p,)
    }
};
        var_rsg_p = assign5820_e4488;
        var_rsg_p_rv = 0.0;

        let (assign5830_e4514,) = {
    if (var_guard36 != 0.0) {
        let assign5830_e4493: f64 = (p.p295 * var_gwe);
        let assign5830_e4495: f64 = (assign5830_e4493 / var_gpe);
        let assign5830_e4498: f64 = (var_ile).powf(p.p296);
        let assign5830_e4499: f64 = (assign5830_e4495 * assign5830_e4498);
        let assign5830_e4500: f64 = (p.p294 + assign5830_e4499);
        let assign5830_e4504: f64 = (p.p297 * var_iwe);
        let assign5830_e4505: f64 = (1.0 + assign5830_e4504);
        let assign5830_e4506: f64 = (assign5830_e4500 * assign5830_e4505);
        let assign5830_e4510: f64 = (p.p298 * var_iae);
        let assign5830_e4511: f64 = (1.0 + assign5830_e4510);
        let assign5830_e4512: f64 = (assign5830_e4506 * assign5830_e4511);
        (assign5830_e4512,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign5830_e4514;
        var_thesat_p_rv = 0.0;

        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfb_p_rv_slot = var_cfb_p_rv;
        *var_cfd_p_slot = var_cfd_p;
        *var_cfd_p_rv_slot = var_cfd_p_rv;
        *var_cs_p_slot = var_cs_p;
        *var_cs_p_rv_slot = var_cs_p_rv;
        *var_ct_p_slot = var_ct_p;
        *var_ct_p_rv_slot = var_ct_p_rv;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctb_p_rv_slot = var_ctb_p_rv;
        *var_ctg_p_slot = var_ctg_p;
        *var_ctg_p_rv_slot = var_ctg_p_rv;
        *var_fbet1e_slot = var_fbet1e;
        *var_fbet1e_rv_slot = var_fbet1e_rv;
        *var_feta_p_slot = var_feta_p;
        *var_feta_p_rv_slot = var_feta_p_rv;
        *var_gpe_slot = var_gpe;
        *var_gpe_rv_slot = var_gpe_rv;
        *var_gwe_slot = var_gwe;
        *var_gwe_rv_slot = var_gwe_rv;
        *var_lp1e_slot = var_lp1e;
        *var_lp1e_rv_slot = var_lp1e_rv;
        *var_mue_p_slot = var_mue_p;
        *var_mue_p_rv_slot = var_mue_p_rv;
        *var_nov_p_slot = var_nov_p;
        *var_nov_p_rv_slot = var_nov_p_rv;
        *var_novd_p_slot = var_novd_p;
        *var_novd_p_rv_slot = var_novd_p_rv;
        *var_np_p_slot = var_np_p;
        *var_np_p_rv_slot = var_np_p_rv;
        *var_psce_p_slot = var_psce_p;
        *var_psce_p_rv_slot = var_psce_p_rv;
        *var_psceb_p_slot = var_psceb_p;
        *var_psceb_p_rv_slot = var_psceb_p_rv;
        *var_psced_p_slot = var_psced_p;
        *var_psced_p_rv_slot = var_psced_p_rv;
        *var_rs_p_slot = var_rs_p;
        *var_rs_p_rv_slot = var_rs_p_rv;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsb_p_rv_slot = var_rsb_p_rv;
        *var_rsg_p_slot = var_rsg_p;
        *var_rsg_p_rv_slot = var_rsg_p_rv;
        *var_stbet_p_slot = var_stbet_p;
        *var_stbet_p_rv_slot = var_stbet_p_rv;
        *var_stcs_p_slot = var_stcs_p;
        *var_stcs_p_rv_slot = var_stcs_p_rv;
        *var_stct_p_slot = var_stct_p;
        *var_stct_p_rv_slot = var_stct_p_rv;
        *var_stmue_p_slot = var_stmue_p;
        *var_stmue_p_rv_slot = var_stmue_p_rv;
        *var_strs_p_slot = var_strs_p;
        *var_strs_p_rv_slot = var_strs_p_rv;
        *var_stthecs_p_slot = var_stthecs_p;
        *var_stthecs_p_rv_slot = var_stthecs_p_rv;
        *var_stthemu_p_slot = var_stthemu_p;
        *var_stthemu_p_rv_slot = var_stthemu_p_rv;
        *var_stxcor_p_slot = var_stxcor_p;
        *var_stxcor_p_rv_slot = var_stxcor_p_rv;
        *var_thecs_p_slot = var_thecs_p;
        *var_thecs_p_rv_slot = var_thecs_p_rv;
        *var_themu_p_slot = var_themu_p;
        *var_themu_p_rv_slot = var_themu_p_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_toxov_p_slot = var_toxov_p;
        *var_toxov_p_rv_slot = var_toxov_p_rv;
        *var_toxovd_p_slot = var_toxovd_p;
        *var_toxovd_p_rv_slot = var_toxovd_p_rv;
        *var_xcor_p_slot = var_xcor_p;
        *var_xcor_p_rv_slot = var_xcor_p_rv;
    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lecv: f64,
        var_wecv: f64,
        var_a1_p_slot: &mut f64,
        var_a1_p_rv_slot: &mut f64,
        var_a2_p_slot: &mut f64,
        var_a2_p_rv_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_a3_p_rv_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_a4_p_rv_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidl_p_rv_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_agidld_p_rv_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp1_p_rv_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp2_p_rv_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_alp_p_rv_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_ax_p_rv_slot: &mut f64,
        var_bgidl_p_slot: &mut f64,
        var_bgidl_p_rv_slot: &mut f64,
        var_bgidld_p_slot: &mut f64,
        var_bgidld_p_rv_slot: &mut f64,
        var_cgidl_p_slot: &mut f64,
        var_cgidl_p_rv_slot: &mut f64,
        var_cgidld_p_slot: &mut f64,
        var_cgidld_p_rv_slot: &mut f64,
        var_chib_p_slot: &mut f64,
        var_chib_p_rv_slot: &mut f64,
        var_cox_p_slot: &mut f64,
        var_cox_p_rv_slot: &mut f64,
        var_gc2_p_slot: &mut f64,
        var_gc2_p_rv_slot: &mut f64,
        var_gc2ov_p_slot: &mut f64,
        var_gc2ov_p_rv_slot: &mut f64,
        var_gc2ovd_p_slot: &mut f64,
        var_gc2ovd_p_rv_slot: &mut f64,
        var_gc3_p_slot: &mut f64,
        var_gc3_p_rv_slot: &mut f64,
        var_gc3ov_p_slot: &mut f64,
        var_gc3ov_p_rv_slot: &mut f64,
        var_gc3ovd_p_slot: &mut f64,
        var_gc3ovd_p_rv_slot: &mut f64,
        var_gco_p_slot: &mut f64,
        var_gco_p_rv_slot: &mut f64,
        var_guard39_slot: &mut f64,
        var_guard39_rv_slot: &mut f64,
        var_guard40_slot: &mut f64,
        var_guard40_rv_slot: &mut f64,
        var_guard41_slot: &mut f64,
        var_guard41_rv_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_guard42_rv_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_iginv_p_rv_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igov_p_rv_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_igovd_p_rv_slot: &mut f64,
        var_imaxii_p_slot: &mut f64,
        var_imaxii_p_rv_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_sta2_p_rv_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidl_p_rv_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stbgidld_p_rv_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_stig_p_rv_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stthesat_p_rv_slot: &mut f64,
        var_thesatb_p_slot: &mut f64,
        var_thesatb_p_rv_slot: &mut f64,
        var_thesatg_p_slot: &mut f64,
        var_thesatg_p_rv_slot: &mut f64,
        var_thesatt_p_slot: &mut f64,
        var_thesatt_p_rv_slot: &mut f64,
        var_tmpx_slot: &mut f64,
        var_tmpx_rv_slot: &mut f64,
        var_vp_p_slot: &mut f64,
        var_vp_p_rv_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a1_p_rv: f64 = *var_a1_p_rv_slot;
        let mut var_a2_p: f64 = *var_a2_p_slot;
        let mut var_a2_p_rv: f64 = *var_a2_p_rv_slot;
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_a3_p_rv: f64 = *var_a3_p_rv_slot;
        let mut var_a4_p: f64 = *var_a4_p_slot;
        let mut var_a4_p_rv: f64 = *var_a4_p_rv_slot;
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidl_p_rv: f64 = *var_agidl_p_rv_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_agidld_p_rv: f64 = *var_agidld_p_rv_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp1_p_rv: f64 = *var_alp1_p_rv_slot;
        let mut var_alp2_p: f64 = *var_alp2_p_slot;
        let mut var_alp2_p_rv: f64 = *var_alp2_p_rv_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_alp_p_rv: f64 = *var_alp_p_rv_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_ax_p_rv: f64 = *var_ax_p_rv_slot;
        let mut var_bgidl_p: f64 = *var_bgidl_p_slot;
        let mut var_bgidl_p_rv: f64 = *var_bgidl_p_rv_slot;
        let mut var_bgidld_p: f64 = *var_bgidld_p_slot;
        let mut var_bgidld_p_rv: f64 = *var_bgidld_p_rv_slot;
        let mut var_cgidl_p: f64 = *var_cgidl_p_slot;
        let mut var_cgidl_p_rv: f64 = *var_cgidl_p_rv_slot;
        let mut var_cgidld_p: f64 = *var_cgidld_p_slot;
        let mut var_cgidld_p_rv: f64 = *var_cgidld_p_rv_slot;
        let mut var_chib_p: f64 = *var_chib_p_slot;
        let mut var_chib_p_rv: f64 = *var_chib_p_rv_slot;
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_cox_p_rv: f64 = *var_cox_p_rv_slot;
        let mut var_gc2_p: f64 = *var_gc2_p_slot;
        let mut var_gc2_p_rv: f64 = *var_gc2_p_rv_slot;
        let mut var_gc2ov_p: f64 = *var_gc2ov_p_slot;
        let mut var_gc2ov_p_rv: f64 = *var_gc2ov_p_rv_slot;
        let mut var_gc2ovd_p: f64 = *var_gc2ovd_p_slot;
        let mut var_gc2ovd_p_rv: f64 = *var_gc2ovd_p_rv_slot;
        let mut var_gc3_p: f64 = *var_gc3_p_slot;
        let mut var_gc3_p_rv: f64 = *var_gc3_p_rv_slot;
        let mut var_gc3ov_p: f64 = *var_gc3ov_p_slot;
        let mut var_gc3ov_p_rv: f64 = *var_gc3ov_p_rv_slot;
        let mut var_gc3ovd_p: f64 = *var_gc3ovd_p_slot;
        let mut var_gc3ovd_p_rv: f64 = *var_gc3ovd_p_rv_slot;
        let mut var_gco_p: f64 = *var_gco_p_slot;
        let mut var_gco_p_rv: f64 = *var_gco_p_rv_slot;
        let mut var_guard39: f64 = *var_guard39_slot;
        let mut var_guard39_rv: f64 = *var_guard39_rv_slot;
        let mut var_guard40: f64 = *var_guard40_slot;
        let mut var_guard40_rv: f64 = *var_guard40_rv_slot;
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_guard41_rv: f64 = *var_guard41_rv_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_guard42_rv: f64 = *var_guard42_rv_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_iginv_p_rv: f64 = *var_iginv_p_rv_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igov_p_rv: f64 = *var_igov_p_rv_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_igovd_p_rv: f64 = *var_igovd_p_rv_slot;
        let mut var_imaxii_p: f64 = *var_imaxii_p_slot;
        let mut var_imaxii_p_rv: f64 = *var_imaxii_p_rv_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_sta2_p_rv: f64 = *var_sta2_p_rv_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidl_p_rv: f64 = *var_stbgidl_p_rv_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stbgidld_p_rv: f64 = *var_stbgidld_p_rv_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_stig_p_rv: f64 = *var_stig_p_rv_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stthesat_p_rv: f64 = *var_stthesat_p_rv_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_thesatb_p_rv: f64 = *var_thesatb_p_rv_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;
        let mut var_thesatg_p_rv: f64 = *var_thesatg_p_rv_slot;
        let mut var_thesatt_p: f64 = *var_thesatt_p_slot;
        let mut var_thesatt_p_rv: f64 = *var_thesatt_p_rv_slot;
        let mut var_tmpx: f64 = *var_tmpx_slot;
        let mut var_tmpx_rv: f64 = *var_tmpx_rv_slot;
        let mut var_vp_p: f64 = *var_vp_p_slot;
        let mut var_vp_p_rv: f64 = *var_vp_p_rv_slot;

        let (assign5840_e4530,) = {
    if (var_guard36 != 0.0) {
        let assign5840_e4519: f64 = (p.p300 * var_ile);
        let assign5840_e4520: f64 = (p.p299 + assign5840_e4519);
        let assign5840_e4523: f64 = (p.p301 * var_iwe);
        let assign5840_e4524: f64 = (assign5840_e4520 + assign5840_e4523);
        let assign5840_e4527: f64 = (p.p302 * var_iae);
        let assign5840_e4528: f64 = (assign5840_e4524 + assign5840_e4527);
        (assign5840_e4528,)
    } else {
        (var_stthesat_p,)
    }
};
        var_stthesat_p = assign5840_e4530;
        var_stthesat_p_rv = 0.0;

        let (assign5850_e4534,) = {
    if (var_guard36 != 0.0) {
        (p.p303,)
    } else {
        (var_thesatb_p,)
    }
};
        var_thesatb_p = assign5850_e4534;
        var_thesatb_p_rv = 0.0;

        let (assign5860_e4538,) = {
    if (var_guard36 != 0.0) {
        (p.p304,)
    } else {
        (var_thesatg_p,)
    }
};
        var_thesatg_p = assign5860_e4538;
        var_thesatg_p_rv = 0.0;

        let (assign5870_e4542,) = {
    if (var_guard36 != 0.0) {
        (p.p305,)
    } else {
        (var_thesatt_p,)
    }
};
        var_thesatt_p = assign5870_e4542;
        var_thesatt_p_rv = 0.0;

        let (assign5880_e4552,) = {
    if (var_guard36 != 0.0) {
        let assign5880_e4548: f64 = (p.p307 * var_ile);
        let assign5880_e4549: f64 = (1.0 + assign5880_e4548);
        let assign5880_e4550: f64 = (p.p306 / assign5880_e4549);
        (assign5880_e4550,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign5880_e4552;
        var_ax_p_rv = 0.0;

        let (assign5890_e4566,) = {
    if (var_guard36 != 0.0) {
        let assign5890_e4557: f64 = (var_ile).powf(p.p309);
        let assign5890_e4558: f64 = (p.p308 * assign5890_e4557);
        let assign5890_e4562: f64 = (p.p310 * var_iwe);
        let assign5890_e4563: f64 = (1.0 + assign5890_e4562);
        let assign5890_e4564: f64 = (assign5890_e4558 * assign5890_e4563);
        (assign5890_e4564,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign5890_e4566;
        var_alp_p_rv = 0.0;

        let (assign5900_e4572,) = {
    if (var_guard36 != 0.0) {
        let assign5900_e4570: f64 = (var_ile).powf(p.p312);
        (assign5900_e4570,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign5900_e4572;
        var_tmpx_rv = 0.0;

        let (assign5910_e4592,) = {
    if (var_guard36 != 0.0) {
        let assign5910_e4576: f64 = (p.p311 * var_tmpx);
        let assign5910_e4580: f64 = (p.p314 * var_iwe);
        let assign5910_e4581: f64 = (1.0 + assign5910_e4580);
        let assign5910_e4582: f64 = (assign5910_e4576 * assign5910_e4581);
        let assign5910_e4586: f64 = (p.p313 * var_ile);
        let assign5910_e4588: f64 = (assign5910_e4586 * var_tmpx);
        let assign5910_e4589: f64 = (1.0 + assign5910_e4588);
        let assign5910_e4590: f64 = (assign5910_e4582 / assign5910_e4589);
        (assign5910_e4590,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign5910_e4592;
        var_alp1_p_rv = 0.0;

        let (assign5920_e4598,) = {
    if (var_guard36 != 0.0) {
        let assign5920_e4596: f64 = (var_ile).powf(p.p316);
        (assign5920_e4596,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign5920_e4598;
        var_tmpx_rv = 0.0;

        let (assign5930_e4618,) = {
    if (var_guard36 != 0.0) {
        let assign5930_e4602: f64 = (p.p315 * var_tmpx);
        let assign5930_e4606: f64 = (p.p318 * var_iwe);
        let assign5930_e4607: f64 = (1.0 + assign5930_e4606);
        let assign5930_e4608: f64 = (assign5930_e4602 * assign5930_e4607);
        let assign5930_e4612: f64 = (p.p317 * var_ile);
        let assign5930_e4614: f64 = (assign5930_e4612 * var_tmpx);
        let assign5930_e4615: f64 = (1.0 + assign5930_e4614);
        let assign5930_e4616: f64 = (assign5930_e4608 / assign5930_e4615);
        (assign5930_e4616,)
    } else {
        (var_alp2_p,)
    }
};
        var_alp2_p = assign5930_e4618;
        var_alp2_p_rv = 0.0;

        let (assign5940_e4622,) = {
    if (var_guard36 != 0.0) {
        (p.p319,)
    } else {
        (var_vp_p,)
    }
};
        var_vp_p = assign5940_e4622;
        var_vp_p_rv = 0.0;

        let (assign5950_e4638,) = {
    if (var_guard36 != 0.0) {
        let assign5950_e4628: f64 = (p.p321 * var_ile);
        let assign5950_e4629: f64 = (1.0 + assign5950_e4628);
        let assign5950_e4630: f64 = (p.p320 * assign5950_e4629);
        let assign5950_e4634: f64 = (p.p322 * var_iwe);
        let assign5950_e4635: f64 = (1.0 + assign5950_e4634);
        let assign5950_e4636: f64 = (assign5950_e4630 * assign5950_e4635);
        (assign5950_e4636,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign5950_e4638;
        var_a1_p_rv = 0.0;

        let (assign5960_e4642,) = {
    if (var_guard36 != 0.0) {
        (p.p323,)
    } else {
        (var_a2_p,)
    }
};
        var_a2_p = assign5960_e4642;
        var_a2_p_rv = 0.0;

        let (assign5970_e4646,) = {
    if (var_guard36 != 0.0) {
        (p.p324,)
    } else {
        (var_sta2_p,)
    }
};
        var_sta2_p = assign5970_e4646;
        var_sta2_p_rv = 0.0;

        let (assign5980_e4662,) = {
    if (var_guard36 != 0.0) {
        let assign5980_e4652: f64 = (p.p326 * var_ile);
        let assign5980_e4653: f64 = (1.0 + assign5980_e4652);
        let assign5980_e4654: f64 = (p.p325 * assign5980_e4653);
        let assign5980_e4658: f64 = (p.p327 * var_iwe);
        let assign5980_e4659: f64 = (1.0 + assign5980_e4658);
        let assign5980_e4660: f64 = (assign5980_e4654 * assign5980_e4659);
        (assign5980_e4660,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign5980_e4662;
        var_a3_p_rv = 0.0;

        let (assign5990_e4678,) = {
    if (var_guard36 != 0.0) {
        let assign5990_e4668: f64 = (p.p329 * var_ile);
        let assign5990_e4669: f64 = (1.0 + assign5990_e4668);
        let assign5990_e4670: f64 = (p.p328 * assign5990_e4669);
        let assign5990_e4674: f64 = (p.p330 * var_iwe);
        let assign5990_e4675: f64 = (1.0 + assign5990_e4674);
        let assign5990_e4676: f64 = (assign5990_e4670 * assign5990_e4675);
        (assign5990_e4676,)
    } else {
        (var_a4_p,)
    }
};
        var_a4_p = assign5990_e4678;
        var_a4_p_rv = 0.0;

        let (assign6000_e4682,) = {
    if (var_guard36 != 0.0) {
        (p.p331,)
    } else {
        (var_imaxii_p,)
    }
};
        var_imaxii_p = assign6000_e4682;
        var_imaxii_p_rv = 0.0;

        let (assign6010_e4686,) = {
    if (var_guard36 != 0.0) {
        (p.p332,)
    } else {
        (var_gco_p,)
    }
};
        var_gco_p = assign6010_e4686;
        var_gco_p_rv = 0.0;

        let (assign6020_e4692,) = {
    if (var_guard36 != 0.0) {
        let assign6020_e4690: f64 = (p.p333 / var_iae);
        (assign6020_e4690,)
    } else {
        (var_iginv_p,)
    }
};
        var_iginv_p = assign6020_e4692;
        var_iginv_p_rv = 0.0;

        let (assign6030_e4702,) = {
    if (var_guard36 != 0.0) {
        let assign6030_e4696: f64 = (p.p334 * p.p234);
        let assign6030_e4699: f64 = (1e-6 * var_iwe);
        let assign6030_e4700: f64 = (assign6030_e4696 / assign6030_e4699);
        (assign6030_e4700,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign6030_e4702;
        var_igov_p_rv = 0.0;

        let (assign6040_e4712,) = {
    if (var_guard36 != 0.0) {
        let assign6040_e4706: f64 = (p.p335 * p.p235);
        let assign6040_e4709: f64 = (1e-6 * var_iwe);
        let assign6040_e4710: f64 = (assign6040_e4706 / assign6040_e4709);
        (assign6040_e4710,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign6040_e4712;
        var_igovd_p_rv = 0.0;

        let (assign6050_e4716,) = {
    if (var_guard36 != 0.0) {
        (p.p336,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign6050_e4716;
        var_stig_p_rv = 0.0;

        let (assign6060_e4720,) = {
    if (var_guard36 != 0.0) {
        (p.p337,)
    } else {
        (var_gc2_p,)
    }
};
        var_gc2_p = assign6060_e4720;
        var_gc2_p_rv = 0.0;

        let (assign6070_e4724,) = {
    if (var_guard36 != 0.0) {
        (p.p338,)
    } else {
        (var_gc3_p,)
    }
};
        var_gc3_p = assign6070_e4724;
        var_gc3_p_rv = 0.0;

        let (assign6080_e4728,) = {
    if (var_guard36 != 0.0) {
        (p.p337,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign6080_e4728;
        var_gc2ov_p_rv = 0.0;

        let assign6090_e4730: f64 = if param_given[339] { 1.0 } else { 0.0 };
        let assign6090_e4732: f64 = if assign6090_e4730 == 1.0 { 1.0 } else { 0.0 };
        var_guard39 = assign6090_e4732;
        var_guard39_rv = 0.0;

        let (assign6100_e4738,) = {
    if ((var_guard36 != 0.0) && (var_guard39 != 0.0)) {
        (p.p339,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign6100_e4738;
        var_gc2ov_p_rv = 0.0;

        let (assign6110_e4742,) = {
    if (var_guard36 != 0.0) {
        (p.p338,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign6110_e4742;
        var_gc3ov_p_rv = 0.0;

        let assign6120_e4744: f64 = if param_given[340] { 1.0 } else { 0.0 };
        let assign6120_e4746: f64 = if assign6120_e4744 == 1.0 { 1.0 } else { 0.0 };
        var_guard40 = assign6120_e4746;
        var_guard40_rv = 0.0;

        let (assign6130_e4752,) = {
    if ((var_guard36 != 0.0) && (var_guard40 != 0.0)) {
        (p.p340,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign6130_e4752;
        var_gc3ov_p_rv = 0.0;

        let (assign6140_e4756,) = {
    if (var_guard36 != 0.0) {
        (var_gc2ov_p,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign6140_e4756;
        var_gc2ovd_p_rv = 0.0;

        let assign6150_e4758: f64 = if param_given[341] { 1.0 } else { 0.0 };
        let assign6150_e4760: f64 = if assign6150_e4758 == 1.0 { 1.0 } else { 0.0 };
        var_guard41 = assign6150_e4760;
        var_guard41_rv = 0.0;

        let (assign6160_e4766,) = {
    if ((var_guard36 != 0.0) && (var_guard41 != 0.0)) {
        (p.p341,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign6160_e4766;
        var_gc2ovd_p_rv = 0.0;

        let (assign6170_e4770,) = {
    if (var_guard36 != 0.0) {
        (var_gc3ov_p,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign6170_e4770;
        var_gc3ovd_p_rv = 0.0;

        let assign6180_e4772: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6180_e4774: f64 = if assign6180_e4772 == 1.0 { 1.0 } else { 0.0 };
        var_guard42 = assign6180_e4774;
        var_guard42_rv = 0.0;

        let (assign6190_e4780,) = {
    if ((var_guard36 != 0.0) && (var_guard42 != 0.0)) {
        (p.p342,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign6190_e4780;
        var_gc3ovd_p_rv = 0.0;

        let (assign6200_e4784,) = {
    if (var_guard36 != 0.0) {
        (p.p343,)
    } else {
        (var_chib_p,)
    }
};
        var_chib_p = assign6200_e4784;
        var_chib_p_rv = 0.0;

        let (assign6210_e4794,) = {
    if (var_guard36 != 0.0) {
        let assign6210_e4788: f64 = (p.p344 * p.p234);
        let assign6210_e4791: f64 = (1e-6 * var_iwe);
        let assign6210_e4792: f64 = (assign6210_e4788 / assign6210_e4791);
        (assign6210_e4792,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign6210_e4794;
        var_agidl_p_rv = 0.0;

        let (assign6220_e4804,) = {
    if (var_guard36 != 0.0) {
        let assign6220_e4798: f64 = (p.p345 * p.p235);
        let assign6220_e4801: f64 = (1e-6 * var_iwe);
        let assign6220_e4802: f64 = (assign6220_e4798 / assign6220_e4801);
        (assign6220_e4802,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign6220_e4804;
        var_agidld_p_rv = 0.0;

        let (assign6230_e4808,) = {
    if (var_guard36 != 0.0) {
        (p.p346,)
    } else {
        (var_bgidl_p,)
    }
};
        var_bgidl_p = assign6230_e4808;
        var_bgidl_p_rv = 0.0;

        let (assign6240_e4812,) = {
    if (var_guard36 != 0.0) {
        (p.p347,)
    } else {
        (var_bgidld_p,)
    }
};
        var_bgidld_p = assign6240_e4812;
        var_bgidld_p_rv = 0.0;

        let (assign6250_e4816,) = {
    if (var_guard36 != 0.0) {
        (p.p348,)
    } else {
        (var_stbgidl_p,)
    }
};
        var_stbgidl_p = assign6250_e4816;
        var_stbgidl_p_rv = 0.0;

        let (assign6260_e4820,) = {
    if (var_guard36 != 0.0) {
        (p.p349,)
    } else {
        (var_stbgidld_p,)
    }
};
        var_stbgidld_p = assign6260_e4820;
        var_stbgidld_p_rv = 0.0;

        let (assign6270_e4824,) = {
    if (var_guard36 != 0.0) {
        (p.p350,)
    } else {
        (var_cgidl_p,)
    }
};
        var_cgidl_p = assign6270_e4824;
        var_cgidl_p_rv = 0.0;

        let (assign6280_e4828,) = {
    if (var_guard36 != 0.0) {
        (p.p351,)
    } else {
        (var_cgidld_p,)
    }
};
        var_cgidld_p = assign6280_e4828;
        var_cgidld_p_rv = 0.0;

        let (assign6290_e4840,) = {
    if (var_guard36 != 0.0) {
        let assign6290_e4832: f64 = (8.8541878176e-12 * p.p207);
        let assign6290_e4834: f64 = (assign6290_e4832 * var_wecv);
        let assign6290_e4836: f64 = (assign6290_e4834 * var_lecv);
        let assign6290_e4838: f64 = (assign6290_e4836 / p.p206);
        (assign6290_e4838,)
    } else {
        (var_cox_p,)
    }
};
        var_cox_p = assign6290_e4840;
        var_cox_p_rv = 0.0;

        *var_a1_p_slot = var_a1_p;
        *var_a1_p_rv_slot = var_a1_p_rv;
        *var_a2_p_slot = var_a2_p;
        *var_a2_p_rv_slot = var_a2_p_rv;
        *var_a3_p_slot = var_a3_p;
        *var_a3_p_rv_slot = var_a3_p_rv;
        *var_a4_p_slot = var_a4_p;
        *var_a4_p_rv_slot = var_a4_p_rv;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidl_p_rv_slot = var_agidl_p_rv;
        *var_agidld_p_slot = var_agidld_p;
        *var_agidld_p_rv_slot = var_agidld_p_rv;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp1_p_rv_slot = var_alp1_p_rv;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp2_p_rv_slot = var_alp2_p_rv;
        *var_alp_p_slot = var_alp_p;
        *var_alp_p_rv_slot = var_alp_p_rv;
        *var_ax_p_slot = var_ax_p;
        *var_ax_p_rv_slot = var_ax_p_rv;
        *var_bgidl_p_slot = var_bgidl_p;
        *var_bgidl_p_rv_slot = var_bgidl_p_rv;
        *var_bgidld_p_slot = var_bgidld_p;
        *var_bgidld_p_rv_slot = var_bgidld_p_rv;
        *var_cgidl_p_slot = var_cgidl_p;
        *var_cgidl_p_rv_slot = var_cgidl_p_rv;
        *var_cgidld_p_slot = var_cgidld_p;
        *var_cgidld_p_rv_slot = var_cgidld_p_rv;
        *var_chib_p_slot = var_chib_p;
        *var_chib_p_rv_slot = var_chib_p_rv;
        *var_cox_p_slot = var_cox_p;
        *var_cox_p_rv_slot = var_cox_p_rv;
        *var_gc2_p_slot = var_gc2_p;
        *var_gc2_p_rv_slot = var_gc2_p_rv;
        *var_gc2ov_p_slot = var_gc2ov_p;
        *var_gc2ov_p_rv_slot = var_gc2ov_p_rv;
        *var_gc2ovd_p_slot = var_gc2ovd_p;
        *var_gc2ovd_p_rv_slot = var_gc2ovd_p_rv;
        *var_gc3_p_slot = var_gc3_p;
        *var_gc3_p_rv_slot = var_gc3_p_rv;
        *var_gc3ov_p_slot = var_gc3ov_p;
        *var_gc3ov_p_rv_slot = var_gc3ov_p_rv;
        *var_gc3ovd_p_slot = var_gc3ovd_p;
        *var_gc3ovd_p_rv_slot = var_gc3ovd_p_rv;
        *var_gco_p_slot = var_gco_p;
        *var_gco_p_rv_slot = var_gco_p_rv;
        *var_guard39_slot = var_guard39;
        *var_guard39_rv_slot = var_guard39_rv;
        *var_guard40_slot = var_guard40;
        *var_guard40_rv_slot = var_guard40_rv;
        *var_guard41_slot = var_guard41;
        *var_guard41_rv_slot = var_guard41_rv;
        *var_guard42_slot = var_guard42;
        *var_guard42_rv_slot = var_guard42_rv;
        *var_iginv_p_slot = var_iginv_p;
        *var_iginv_p_rv_slot = var_iginv_p_rv;
        *var_igov_p_slot = var_igov_p;
        *var_igov_p_rv_slot = var_igov_p_rv;
        *var_igovd_p_slot = var_igovd_p;
        *var_igovd_p_rv_slot = var_igovd_p_rv;
        *var_imaxii_p_slot = var_imaxii_p;
        *var_imaxii_p_rv_slot = var_imaxii_p_rv;
        *var_sta2_p_slot = var_sta2_p;
        *var_sta2_p_rv_slot = var_sta2_p_rv;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidl_p_rv_slot = var_stbgidl_p_rv;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stbgidld_p_rv_slot = var_stbgidld_p_rv;
        *var_stig_p_slot = var_stig_p;
        *var_stig_p_rv_slot = var_stig_p_rv;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stthesat_p_rv_slot = var_stthesat_p_rv;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_thesatb_p_rv_slot = var_thesatb_p_rv;
        *var_thesatg_p_slot = var_thesatg_p;
        *var_thesatg_p_rv_slot = var_thesatg_p_rv;
        *var_thesatt_p_slot = var_thesatt_p;
        *var_thesatt_p_rv_slot = var_thesatt_p_rv;
        *var_tmpx_slot = var_tmpx;
        *var_tmpx_rv_slot = var_tmpx_rv;
        *var_vp_p_slot = var_vp_p;
        *var_vp_p_rv_slot = var_vp_p_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_gpe: f64,
        var_guard36: f64,
        var_gwe: f64,
        var_iae: f64,
        var_iilcv: f64,
        var_iiwcv: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_wecv: f64,
        var_alp1ac_p_slot: &mut f64,
        var_alp1ac_p_rv_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_alpac_p_rv_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axac_p_rv_slot: &mut f64,
        var_axacl_i_slot: &mut f64,
        var_axacl_i_rv_slot: &mut f64,
        var_axaco_i_slot: &mut f64,
        var_axaco_i_rv_slot: &mut f64,
        var_axinr_p_slot: &mut f64,
        var_axinr_p_rv_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfr_p_rv_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cfrd_p_rv_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgbov_p_rv_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgov_p_rv_slot: &mut f64,
        var_cgovaccg_p_slot: &mut f64,
        var_cgovaccg_p_rv_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_cgovd_p_rv_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinr_p_rv_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_cinrd_p_rv_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_delvtac_p_rv_slot: &mut f64,
        var_dvfbinr_p_slot: &mut f64,
        var_dvfbinr_p_rv_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_facneffac_p_rv_slot: &mut f64,
        var_fcgovacc_p_slot: &mut f64,
        var_fcgovacc_p_rv_slot: &mut f64,
        var_fcgovaccd_p_slot: &mut f64,
        var_fcgovaccd_p_rv_slot: &mut f64,
        var_fcinracc_p_slot: &mut f64,
        var_fcinracc_p_rv_slot: &mut f64,
        var_fcinrdep_p_slot: &mut f64,
        var_fcinrdep_p_rv_slot: &mut f64,
        var_fnt_p_slot: &mut f64,
        var_fnt_p_rv_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_guard43_rv_slot: &mut f64,
        var_guard44_slot: &mut f64,
        var_guard44_rv_slot: &mut f64,
        var_guard45_slot: &mut f64,
        var_guard45_rv_slot: &mut f64,
        var_guard46_slot: &mut f64,
        var_guard46_rv_slot: &mut f64,
        var_guard47_slot: &mut f64,
        var_guard47_rv_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_guard48_rv_slot: &mut f64,
        var_guard49_slot: &mut f64,
        var_guard49_rv_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_stvfbedge_p_rv_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_temp0_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_thesatacl_i_slot: &mut f64,
        var_thesatacl_i_rv_slot: &mut f64,
        var_thesataclexp_i_slot: &mut f64,
        var_thesataclexp_i_rv_slot: &mut f64,
        var_thesataclw_i_slot: &mut f64,
        var_thesataclw_i_rv_slot: &mut f64,
        var_thesataco_i_slot: &mut f64,
        var_thesataco_i_rv_slot: &mut f64,
        var_thesatacw_i_slot: &mut f64,
        var_thesatacw_i_rv_slot: &mut f64,
        var_tmpx_slot: &mut f64,
        var_tmpx_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
        var_we_edge_slot: &mut f64,
        var_we_edge_rv_slot: &mut f64,
    ) {
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alp1ac_p_rv: f64 = *var_alp1ac_p_rv_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_alpac_p_rv: f64 = *var_alpac_p_rv_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axac_p_rv: f64 = *var_axac_p_rv_slot;
        let mut var_axacl_i: f64 = *var_axacl_i_slot;
        let mut var_axacl_i_rv: f64 = *var_axacl_i_rv_slot;
        let mut var_axaco_i: f64 = *var_axaco_i_slot;
        let mut var_axaco_i_rv: f64 = *var_axaco_i_rv_slot;
        let mut var_axinr_p: f64 = *var_axinr_p_slot;
        let mut var_axinr_p_rv: f64 = *var_axinr_p_rv_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfr_p_rv: f64 = *var_cfr_p_rv_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cfrd_p_rv: f64 = *var_cfrd_p_rv_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgbov_p_rv: f64 = *var_cgbov_p_rv_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgov_p_rv: f64 = *var_cgov_p_rv_slot;
        let mut var_cgovaccg_p: f64 = *var_cgovaccg_p_slot;
        let mut var_cgovaccg_p_rv: f64 = *var_cgovaccg_p_rv_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_cgovd_p_rv: f64 = *var_cgovd_p_rv_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinr_p_rv: f64 = *var_cinr_p_rv_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_cinrd_p_rv: f64 = *var_cinrd_p_rv_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_delvtac_p_rv: f64 = *var_delvtac_p_rv_slot;
        let mut var_dvfbinr_p: f64 = *var_dvfbinr_p_slot;
        let mut var_dvfbinr_p_rv: f64 = *var_dvfbinr_p_rv_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_facneffac_p_rv: f64 = *var_facneffac_p_rv_slot;
        let mut var_fcgovacc_p: f64 = *var_fcgovacc_p_slot;
        let mut var_fcgovacc_p_rv: f64 = *var_fcgovacc_p_rv_slot;
        let mut var_fcgovaccd_p: f64 = *var_fcgovaccd_p_slot;
        let mut var_fcgovaccd_p_rv: f64 = *var_fcgovaccd_p_rv_slot;
        let mut var_fcinracc_p: f64 = *var_fcinracc_p_slot;
        let mut var_fcinracc_p_rv: f64 = *var_fcinracc_p_rv_slot;
        let mut var_fcinrdep_p: f64 = *var_fcinrdep_p_slot;
        let mut var_fcinrdep_p_rv: f64 = *var_fcinrdep_p_rv_slot;
        let mut var_fnt_p: f64 = *var_fnt_p_slot;
        let mut var_fnt_p_rv: f64 = *var_fnt_p_rv_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_guard43_rv: f64 = *var_guard43_rv_slot;
        let mut var_guard44: f64 = *var_guard44_slot;
        let mut var_guard44_rv: f64 = *var_guard44_rv_slot;
        let mut var_guard45: f64 = *var_guard45_slot;
        let mut var_guard45_rv: f64 = *var_guard45_rv_slot;
        let mut var_guard46: f64 = *var_guard46_slot;
        let mut var_guard46_rv: f64 = *var_guard46_rv_slot;
        let mut var_guard47: f64 = *var_guard47_slot;
        let mut var_guard47_rv: f64 = *var_guard47_rv_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_guard48_rv: f64 = *var_guard48_rv_slot;
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_guard49_rv: f64 = *var_guard49_rv_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_stvfbedge_p_rv: f64 = *var_stvfbedge_p_rv_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_temp0_rv: f64 = *var_temp0_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_thesatacl_i: f64 = *var_thesatacl_i_slot;
        let mut var_thesatacl_i_rv: f64 = *var_thesatacl_i_rv_slot;
        let mut var_thesataclexp_i: f64 = *var_thesataclexp_i_slot;
        let mut var_thesataclexp_i_rv: f64 = *var_thesataclexp_i_rv_slot;
        let mut var_thesataclw_i: f64 = *var_thesataclw_i_slot;
        let mut var_thesataclw_i_rv: f64 = *var_thesataclw_i_rv_slot;
        let mut var_thesataco_i: f64 = *var_thesataco_i_slot;
        let mut var_thesataco_i_rv: f64 = *var_thesataco_i_rv_slot;
        let mut var_thesatacw_i: f64 = *var_thesatacw_i_slot;
        let mut var_thesatacw_i_rv: f64 = *var_thesatacw_i_rv_slot;
        let mut var_tmpx: f64 = *var_tmpx_slot;
        let mut var_tmpx_rv: f64 = *var_tmpx_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;
        let mut var_we_edge: f64 = *var_we_edge_slot;
        let mut var_we_edge_rv: f64 = *var_we_edge_rv_slot;

        let (assign6300_e4852,) = {
    if (var_guard36 != 0.0) {
        let assign6300_e4844: f64 = (8.8541878176e-12 * p.p207);
        let assign6300_e4846: f64 = (assign6300_e4844 * var_wecv);
        let assign6300_e4848: f64 = (assign6300_e4846 * p.p234);
        let assign6300_e4850: f64 = (assign6300_e4848 / p.p232);
        (assign6300_e4850,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign6300_e4852;
        var_cgov_p_rv = 0.0;

        let (assign6310_e4864,) = {
    if (var_guard36 != 0.0) {
        let assign6310_e4856: f64 = (8.8541878176e-12 * p.p207);
        let assign6310_e4858: f64 = (assign6310_e4856 * var_wecv);
        let assign6310_e4860: f64 = (assign6310_e4858 * p.p235);
        let assign6310_e4862: f64 = (assign6310_e4860 / p.p233);
        (assign6310_e4862,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign6310_e4864;
        var_cgovd_p_rv = 0.0;

        let (assign6320_e4882,) = {
    if (var_guard36 != 0.0) {
        let assign6320_e4870: f64 = (var_ile).powf(p.p354);
        let assign6320_e4871: f64 = (p.p353 * assign6320_e4870);
        let assign6320_e4872: f64 = (p.p352 + assign6320_e4871);
        let assign6320_e4875: f64 = (p.p355 * var_iwe);
        let assign6320_e4876: f64 = (assign6320_e4872 + assign6320_e4875);
        let assign6320_e4879: f64 = (p.p356 * var_iae);
        let assign6320_e4880: f64 = (assign6320_e4876 + assign6320_e4879);
        (assign6320_e4880,)
    } else {
        (var_delvtac_p,)
    }
};
        var_delvtac_p = assign6320_e4882;
        var_delvtac_p_rv = 0.0;

        let (assign6330_e4898,) = {
    if (var_guard36 != 0.0) {
        let assign6330_e4887: f64 = (p.p358 * var_ile);
        let assign6330_e4888: f64 = (p.p357 + assign6330_e4887);
        let assign6330_e4891: f64 = (p.p359 * var_iwe);
        let assign6330_e4892: f64 = (assign6330_e4888 + assign6330_e4891);
        let assign6330_e4895: f64 = (p.p360 * var_iae);
        let assign6330_e4896: f64 = (assign6330_e4892 + assign6330_e4895);
        (assign6330_e4896,)
    } else {
        (var_facneffac_p,)
    }
};
        var_facneffac_p = assign6330_e4898;
        var_facneffac_p_rv = 0.0;

        let (assign6340_e4902,) = {
    if (var_guard36 != 0.0) {
        (p.p294,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign6340_e4902;
        var_thesataco_i_rv = 0.0;

        let assign6350_e4904: f64 = if param_given[361] { 1.0 } else { 0.0 };
        let assign6350_e4906: f64 = if assign6350_e4904 == 1.0 { 1.0 } else { 0.0 };
        var_guard43 = assign6350_e4906;
        var_guard43_rv = 0.0;

        let (assign6360_e4912,) = {
    if ((var_guard36 != 0.0) && (var_guard43 != 0.0)) {
        (p.p361,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign6360_e4912;
        var_thesataco_i_rv = 0.0;

        let (assign6370_e4916,) = {
    if (var_guard36 != 0.0) {
        (p.p295,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign6370_e4916;
        var_thesatacl_i_rv = 0.0;

        let assign6380_e4918: f64 = if param_given[362] { 1.0 } else { 0.0 };
        let assign6380_e4920: f64 = if assign6380_e4918 == 1.0 { 1.0 } else { 0.0 };
        var_guard44 = assign6380_e4920;
        var_guard44_rv = 0.0;

        let (assign6390_e4926,) = {
    if ((var_guard36 != 0.0) && (var_guard44 != 0.0)) {
        (p.p362,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign6390_e4926;
        var_thesatacl_i_rv = 0.0;

        let (assign6400_e4930,) = {
    if (var_guard36 != 0.0) {
        (p.p296,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign6400_e4930;
        var_thesataclexp_i_rv = 0.0;

        let assign6410_e4932: f64 = if param_given[363] { 1.0 } else { 0.0 };
        let assign6410_e4934: f64 = if assign6410_e4932 == 1.0 { 1.0 } else { 0.0 };
        var_guard45 = assign6410_e4934;
        var_guard45_rv = 0.0;

        let (assign6420_e4940,) = {
    if ((var_guard36 != 0.0) && (var_guard45 != 0.0)) {
        (p.p363,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign6420_e4940;
        var_thesataclexp_i_rv = 0.0;

        let (assign6430_e4944,) = {
    if (var_guard36 != 0.0) {
        (p.p297,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign6430_e4944;
        var_thesatacw_i_rv = 0.0;

        let assign6440_e4946: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6440_e4948: f64 = if assign6440_e4946 == 1.0 { 1.0 } else { 0.0 };
        var_guard46 = assign6440_e4948;
        var_guard46_rv = 0.0;

        let (assign6450_e4954,) = {
    if ((var_guard36 != 0.0) && (var_guard46 != 0.0)) {
        (p.p364,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign6450_e4954;
        var_thesatacw_i_rv = 0.0;

        let (assign6460_e4958,) = {
    if (var_guard36 != 0.0) {
        (p.p298,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign6460_e4958;
        var_thesataclw_i_rv = 0.0;

        let assign6470_e4960: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6470_e4962: f64 = if assign6470_e4960 == 1.0 { 1.0 } else { 0.0 };
        var_guard47 = assign6470_e4962;
        var_guard47_rv = 0.0;

        let (assign6480_e4968,) = {
    if ((var_guard36 != 0.0) && (var_guard47 != 0.0)) {
        (p.p365,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign6480_e4968;
        var_thesataclw_i_rv = 0.0;

        let (assign6490_e4994,) = {
    if (var_guard36 != 0.0) {
        let assign6490_e4973: f64 = (var_thesatacl_i * var_gwe);
        let assign6490_e4975: f64 = (assign6490_e4973 / var_gpe);
        let assign6490_e4978: f64 = (var_ile).powf(var_thesataclexp_i);
        let assign6490_e4979: f64 = (assign6490_e4975 * assign6490_e4978);
        let assign6490_e4980: f64 = (var_thesataco_i + assign6490_e4979);
        let assign6490_e4984: f64 = (var_thesatacw_i * var_iwe);
        let assign6490_e4985: f64 = (1.0 + assign6490_e4984);
        let assign6490_e4986: f64 = (assign6490_e4980 * assign6490_e4985);
        let assign6490_e4990: f64 = (var_thesataclw_i * var_iae);
        let assign6490_e4991: f64 = (1.0 + assign6490_e4990);
        let assign6490_e4992: f64 = (assign6490_e4986 * assign6490_e4991);
        (assign6490_e4992,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign6490_e4994;
        var_thesatac_p_rv = 0.0;

        let (assign6500_e4998,) = {
    if (var_guard36 != 0.0) {
        (p.p306,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign6500_e4998;
        var_axaco_i_rv = 0.0;

        let assign6510_e5000: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6510_e5002: f64 = if assign6510_e5000 == 1.0 { 1.0 } else { 0.0 };
        var_guard48 = assign6510_e5002;
        var_guard48_rv = 0.0;

        let (assign6520_e5008,) = {
    if ((var_guard36 != 0.0) && (var_guard48 != 0.0)) {
        (p.p366,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign6520_e5008;
        var_axaco_i_rv = 0.0;

        let (assign6530_e5012,) = {
    if (var_guard36 != 0.0) {
        (p.p307,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign6530_e5012;
        var_axacl_i_rv = 0.0;

        let assign6540_e5014: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6540_e5016: f64 = if assign6540_e5014 == 1.0 { 1.0 } else { 0.0 };
        var_guard49 = assign6540_e5016;
        var_guard49_rv = 0.0;

        let (assign6550_e5022,) = {
    if ((var_guard36 != 0.0) && (var_guard49 != 0.0)) {
        (p.p367,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign6550_e5022;
        var_axacl_i_rv = 0.0;

        let (assign6560_e5032,) = {
    if (var_guard36 != 0.0) {
        let assign6560_e5028: f64 = (var_axacl_i * var_ile);
        let assign6560_e5029: f64 = (1.0 + assign6560_e5028);
        let assign6560_e5030: f64 = (var_axaco_i / assign6560_e5029);
        (assign6560_e5030,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign6560_e5032;
        var_axac_p_rv = 0.0;

        let (assign6570_e5046,) = {
    if (var_guard36 != 0.0) {
        let assign6570_e5037: f64 = (var_ile).powf(p.p369);
        let assign6570_e5038: f64 = (p.p368 * assign6570_e5037);
        let assign6570_e5042: f64 = (p.p370 * var_iwe);
        let assign6570_e5043: f64 = (1.0 + assign6570_e5042);
        let assign6570_e5044: f64 = (assign6570_e5038 * assign6570_e5043);
        (assign6570_e5044,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign6570_e5046;
        var_alpac_p_rv = 0.0;

        let (assign6580_e5052,) = {
    if (var_guard36 != 0.0) {
        let assign6580_e5050: f64 = (var_ile).powf(p.p372);
        (assign6580_e5050,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign6580_e5052;
        var_tmpx_rv = 0.0;

        let (assign6590_e5072,) = {
    if (var_guard36 != 0.0) {
        let assign6590_e5056: f64 = (p.p371 * var_tmpx);
        let assign6590_e5060: f64 = (p.p374 * var_iwe);
        let assign6590_e5061: f64 = (1.0 + assign6590_e5060);
        let assign6590_e5062: f64 = (assign6590_e5056 * assign6590_e5061);
        let assign6590_e5066: f64 = (p.p373 * var_ile);
        let assign6590_e5068: f64 = (assign6590_e5066 * var_tmpx);
        let assign6590_e5069: f64 = (1.0 + assign6590_e5068);
        let assign6590_e5070: f64 = (assign6590_e5062 / assign6590_e5069);
        (assign6590_e5070,)
    } else {
        (var_alp1ac_p,)
    }
};
        var_alp1ac_p = assign6590_e5072;
        var_alp1ac_p_rv = 0.0;

        let (assign6600_e5076,) = {
    if (var_guard36 != 0.0) {
        (p.p375,)
    } else {
        (var_fcgovacc_p,)
    }
};
        var_fcgovacc_p = assign6600_e5076;
        var_fcgovacc_p_rv = 0.0;

        let (assign6610_e5080,) = {
    if (var_guard36 != 0.0) {
        (p.p376,)
    } else {
        (var_fcgovaccd_p,)
    }
};
        var_fcgovaccd_p = assign6610_e5080;
        var_fcgovaccd_p_rv = 0.0;

        let (assign6620_e5084,) = {
    if (var_guard36 != 0.0) {
        (p.p377,)
    } else {
        (var_cgovaccg_p,)
    }
};
        var_cgovaccg_p = assign6620_e5084;
        var_cgovaccg_p_rv = 0.0;

        let (assign6630_e5090,) = {
    if (var_guard36 != 0.0) {
        let assign6630_e5088: f64 = (p.p378 * var_iilcv);
        (assign6630_e5088,)
    } else {
        (var_cgbov_p,)
    }
};
        var_cgbov_p = assign6630_e5090;
        var_cgbov_p_rv = 0.0;

        let (assign6640_e5096,) = {
    if (var_guard36 != 0.0) {
        let assign6640_e5094: f64 = (p.p379 * var_iiwecv);
        (assign6640_e5094,)
    } else {
        (var_cinr_p,)
    }
};
        var_cinr_p = assign6640_e5096;
        var_cinr_p_rv = 0.0;

        let (assign6650_e5102,) = {
    if (var_guard36 != 0.0) {
        let assign6650_e5100: f64 = (p.p380 * var_iiwecv);
        (assign6650_e5100,)
    } else {
        (var_cinrd_p,)
    }
};
        var_cinrd_p = assign6650_e5102;
        var_cinrd_p_rv = 0.0;

        let (assign6660_e5106,) = {
    if (var_guard36 != 0.0) {
        (p.p381,)
    } else {
        (var_dvfbinr_p,)
    }
};
        var_dvfbinr_p = assign6660_e5106;
        var_dvfbinr_p_rv = 0.0;

        let (assign6670_e5110,) = {
    if (var_guard36 != 0.0) {
        (p.p382,)
    } else {
        (var_fcinrdep_p,)
    }
};
        var_fcinrdep_p = assign6670_e5110;
        var_fcinrdep_p_rv = 0.0;

        let (assign6680_e5114,) = {
    if (var_guard36 != 0.0) {
        (p.p383,)
    } else {
        (var_fcinracc_p,)
    }
};
        var_fcinracc_p = assign6680_e5114;
        var_fcinracc_p_rv = 0.0;

        let (assign6690_e5118,) = {
    if (var_guard36 != 0.0) {
        (p.p384,)
    } else {
        (var_axinr_p,)
    }
};
        var_axinr_p = assign6690_e5118;
        var_axinr_p_rv = 0.0;

        let (assign6700_e5124,) = {
    if (var_guard36 != 0.0) {
        let assign6700_e5122: f64 = (p.p385 * var_iiwcv);
        (assign6700_e5122,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign6700_e5124;
        var_cfr_p_rv = 0.0;

        let (assign6710_e5130,) = {
    if (var_guard36 != 0.0) {
        let assign6710_e5128: f64 = (p.p386 * var_iiwcv);
        (assign6710_e5128,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign6710_e5130;
        var_cfrd_p_rv = 0.0;

        let (assign6720_e5140,) = {
    if (var_guard36 != 0.0) {
        let assign6720_e5135: f64 = (2.0 * p.p393);
        let assign6720_e5137: f64 = (assign6720_e5135 / var_le);
        let assign6720_e5138: f64 = (1.0 - assign6720_e5137);
        (assign6720_e5138,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign6720_e5140;
        var_temp0_rv = 0.0;

        let (assign6750_e5161,) = {
    if (var_guard36 != 0.0) {
        (p.p387,)
    } else {
        (var_fnt_p,)
    }
};
        var_fnt_p = assign6750_e5161;
        var_fnt_p_rv = 0.0;

        let (assign6810_e5211,) = {
    if (var_guard36 != 0.0) {
        let assign6810_e5205: f64 = (2.0 * p.p395);
        let assign6810_e5208: f64 = (p.p396 * var_we);
        let assign6810_e5209: f64 = (assign6810_e5205 + assign6810_e5208);
        (assign6810_e5209,)
    } else {
        (var_we_edge,)
    }
};
        var_we_edge = assign6810_e5211;
        var_we_edge_rv = 0.0;

        let (assign6840_e5227,) = {
    if (var_guard36 != 0.0) {
        (p.p397,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign6840_e5227;
        var_vfbedge_p_rv = 0.0;

        let (assign6850_e5243,) = {
    if (var_guard36 != 0.0) {
        let assign6850_e5232: f64 = (p.p399 * var_ile);
        let assign6850_e5233: f64 = (p.p398 + assign6850_e5232);
        let assign6850_e5236: f64 = (p.p400 * var_iwe);
        let assign6850_e5237: f64 = (assign6850_e5233 + assign6850_e5236);
        let assign6850_e5240: f64 = (p.p401 * var_iae);
        let assign6850_e5241: f64 = (assign6850_e5237 + assign6850_e5240);
        (assign6850_e5241,)
    } else {
        (var_stvfbedge_p,)
    }
};
        var_stvfbedge_p = assign6850_e5243;
        var_stvfbedge_p_rv = 0.0;

        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alp1ac_p_rv_slot = var_alp1ac_p_rv;
        *var_alpac_p_slot = var_alpac_p;
        *var_alpac_p_rv_slot = var_alpac_p_rv;
        *var_axac_p_slot = var_axac_p;
        *var_axac_p_rv_slot = var_axac_p_rv;
        *var_axacl_i_slot = var_axacl_i;
        *var_axacl_i_rv_slot = var_axacl_i_rv;
        *var_axaco_i_slot = var_axaco_i;
        *var_axaco_i_rv_slot = var_axaco_i_rv;
        *var_axinr_p_slot = var_axinr_p;
        *var_axinr_p_rv_slot = var_axinr_p_rv;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfr_p_rv_slot = var_cfr_p_rv;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cfrd_p_rv_slot = var_cfrd_p_rv;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgbov_p_rv_slot = var_cgbov_p_rv;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgov_p_rv_slot = var_cgov_p_rv;
        *var_cgovaccg_p_slot = var_cgovaccg_p;
        *var_cgovaccg_p_rv_slot = var_cgovaccg_p_rv;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_cgovd_p_rv_slot = var_cgovd_p_rv;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinr_p_rv_slot = var_cinr_p_rv;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_cinrd_p_rv_slot = var_cinrd_p_rv;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_delvtac_p_rv_slot = var_delvtac_p_rv;
        *var_dvfbinr_p_slot = var_dvfbinr_p;
        *var_dvfbinr_p_rv_slot = var_dvfbinr_p_rv;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_facneffac_p_rv_slot = var_facneffac_p_rv;
        *var_fcgovacc_p_slot = var_fcgovacc_p;
        *var_fcgovacc_p_rv_slot = var_fcgovacc_p_rv;
        *var_fcgovaccd_p_slot = var_fcgovaccd_p;
        *var_fcgovaccd_p_rv_slot = var_fcgovaccd_p_rv;
        *var_fcinracc_p_slot = var_fcinracc_p;
        *var_fcinracc_p_rv_slot = var_fcinracc_p_rv;
        *var_fcinrdep_p_slot = var_fcinrdep_p;
        *var_fcinrdep_p_rv_slot = var_fcinrdep_p_rv;
        *var_fnt_p_slot = var_fnt_p;
        *var_fnt_p_rv_slot = var_fnt_p_rv;
        *var_guard43_slot = var_guard43;
        *var_guard43_rv_slot = var_guard43_rv;
        *var_guard44_slot = var_guard44;
        *var_guard44_rv_slot = var_guard44_rv;
        *var_guard45_slot = var_guard45;
        *var_guard45_rv_slot = var_guard45_rv;
        *var_guard46_slot = var_guard46;
        *var_guard46_rv_slot = var_guard46_rv;
        *var_guard47_slot = var_guard47;
        *var_guard47_rv_slot = var_guard47_rv;
        *var_guard48_slot = var_guard48;
        *var_guard48_rv_slot = var_guard48_rv;
        *var_guard49_slot = var_guard49;
        *var_guard49_rv_slot = var_guard49_rv;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_stvfbedge_p_rv_slot = var_stvfbedge_p_rv;
        *var_temp0_slot = var_temp0;
        *var_temp0_rv_slot = var_temp0_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_thesatacl_i_slot = var_thesatacl_i;
        *var_thesatacl_i_rv_slot = var_thesatacl_i_rv;
        *var_thesataclexp_i_slot = var_thesataclexp_i;
        *var_thesataclexp_i_rv_slot = var_thesataclexp_i_rv;
        *var_thesataclw_i_slot = var_thesataclw_i;
        *var_thesataclw_i_rv_slot = var_thesataclw_i_rv;
        *var_thesataco_i_slot = var_thesataco_i;
        *var_thesataco_i_rv_slot = var_thesataco_i_rv;
        *var_thesatacw_i_slot = var_thesatacw_i;
        *var_thesatacw_i_rv_slot = var_thesatacw_i_rv;
        *var_tmpx_slot = var_tmpx;
        *var_tmpx_rv_slot = var_tmpx_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
        *var_we_edge_slot = var_we_edge;
        *var_we_edge_rv_slot = var_we_edge_rv;
    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_we_edge: f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfbedge_p_rv_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfdedge_p_rv_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfedge_p_rv_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ct_p_rv_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctb_p_rv_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_ctedge_p_rv_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_ctg_p_rv_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dphib_p_rv_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_dphibedge_p_rv_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_gfacnud_p_rv_slot: &mut f64,
        var_gpe_edge_slot: &mut f64,
        var_gpe_edge_rv_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard51_rv_slot: &mut f64,
        var_guard52_slot: &mut f64,
        var_guard52_rv_slot: &mut f64,
        var_guard53_slot: &mut f64,
        var_guard53_rv_slot: &mut f64,
        var_guard54_slot: &mut f64,
        var_guard54_rv_slot: &mut f64,
        var_guard55_slot: &mut f64,
        var_guard55_rv_slot: &mut f64,
        var_guard56_slot: &mut f64,
        var_guard56_rv_slot: &mut f64,
        var_guard57_slot: &mut f64,
        var_guard57_rv_slot: &mut f64,
        var_guard58_slot: &mut f64,
        var_guard58_rv_slot: &mut f64,
        var_guard59_slot: &mut f64,
        var_guard59_rv_slot: &mut f64,
        var_guard60_slot: &mut f64,
        var_guard60_rv_slot: &mut f64,
        var_guard61_slot: &mut f64,
        var_guard61_rv_slot: &mut f64,
        var_guard62_slot: &mut f64,
        var_guard62_rv_slot: &mut f64,
        var_guard63_slot: &mut f64,
        var_guard63_rv_slot: &mut f64,
        var_guard64_slot: &mut f64,
        var_guard64_rv_slot: &mut f64,
        var_guard65_slot: &mut f64,
        var_guard65_rv_slot: &mut f64,
        var_kuowe_slot: &mut f64,
        var_kuowe_rv_slot: &mut f64,
        var_kvthowe_slot: &mut f64,
        var_kvthowe_rv_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_neff_p_rv_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_neffedge_p_rv_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_nov_p_rv_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_novd_p_rv_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_np_p_rv_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscebedge_p_rv_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_pscededge_p_rv_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_psceedge_p_rv_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stbetedge_p_rv_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stct_p_rv_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_stvfb_p_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
        var_vsbnud_p_rv_slot: &mut f64,
    ) {
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfbedge_p_rv: f64 = *var_cfbedge_p_rv_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfdedge_p_rv: f64 = *var_cfdedge_p_rv_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfedge_p_rv: f64 = *var_cfedge_p_rv_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ct_p_rv: f64 = *var_ct_p_rv_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctb_p_rv: f64 = *var_ctb_p_rv_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_ctedge_p_rv: f64 = *var_ctedge_p_rv_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_ctg_p_rv: f64 = *var_ctg_p_rv_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dphib_p_rv: f64 = *var_dphib_p_rv_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_dphibedge_p_rv: f64 = *var_dphibedge_p_rv_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_gfacnud_p_rv: f64 = *var_gfacnud_p_rv_slot;
        let mut var_gpe_edge: f64 = *var_gpe_edge_slot;
        let mut var_gpe_edge_rv: f64 = *var_gpe_edge_rv_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard51_rv: f64 = *var_guard51_rv_slot;
        let mut var_guard52: f64 = *var_guard52_slot;
        let mut var_guard52_rv: f64 = *var_guard52_rv_slot;
        let mut var_guard53: f64 = *var_guard53_slot;
        let mut var_guard53_rv: f64 = *var_guard53_rv_slot;
        let mut var_guard54: f64 = *var_guard54_slot;
        let mut var_guard54_rv: f64 = *var_guard54_rv_slot;
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_guard55_rv: f64 = *var_guard55_rv_slot;
        let mut var_guard56: f64 = *var_guard56_slot;
        let mut var_guard56_rv: f64 = *var_guard56_rv_slot;
        let mut var_guard57: f64 = *var_guard57_slot;
        let mut var_guard57_rv: f64 = *var_guard57_rv_slot;
        let mut var_guard58: f64 = *var_guard58_slot;
        let mut var_guard58_rv: f64 = *var_guard58_rv_slot;
        let mut var_guard59: f64 = *var_guard59_slot;
        let mut var_guard59_rv: f64 = *var_guard59_rv_slot;
        let mut var_guard60: f64 = *var_guard60_slot;
        let mut var_guard60_rv: f64 = *var_guard60_rv_slot;
        let mut var_guard61: f64 = *var_guard61_slot;
        let mut var_guard61_rv: f64 = *var_guard61_rv_slot;
        let mut var_guard62: f64 = *var_guard62_slot;
        let mut var_guard62_rv: f64 = *var_guard62_rv_slot;
        let mut var_guard63: f64 = *var_guard63_slot;
        let mut var_guard63_rv: f64 = *var_guard63_rv_slot;
        let mut var_guard64: f64 = *var_guard64_slot;
        let mut var_guard64_rv: f64 = *var_guard64_rv_slot;
        let mut var_guard65: f64 = *var_guard65_slot;
        let mut var_guard65_rv: f64 = *var_guard65_rv_slot;
        let mut var_kuowe: f64 = *var_kuowe_slot;
        let mut var_kuowe_rv: f64 = *var_kuowe_rv_slot;
        let mut var_kvthowe: f64 = *var_kvthowe_slot;
        let mut var_kvthowe_rv: f64 = *var_kvthowe_rv_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_neff_p_rv: f64 = *var_neff_p_rv_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_neffedge_p_rv: f64 = *var_neffedge_p_rv_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_nov_p_rv: f64 = *var_nov_p_rv_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_novd_p_rv: f64 = *var_novd_p_rv_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_np_p_rv: f64 = *var_np_p_rv_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscebedge_p_rv: f64 = *var_pscebedge_p_rv_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_pscededge_p_rv: f64 = *var_pscededge_p_rv_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_psceedge_p_rv: f64 = *var_psceedge_p_rv_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stbetedge_p_rv: f64 = *var_stbetedge_p_rv_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stct_p_rv: f64 = *var_stct_p_rv_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_stvfb_p_rv: f64 = *var_stvfb_p_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_vsbnud_p_rv: f64 = *var_vsbnud_p_rv_slot;

        let (assign6860_e5261,) = {
    if (var_guard36 != 0.0) {
        let assign6860_e5249: f64 = (var_ile).powf(p.p404);
        let assign6860_e5250: f64 = (p.p403 * assign6860_e5249);
        let assign6860_e5251: f64 = (p.p402 + assign6860_e5250);
        let assign6860_e5254: f64 = (p.p405 * var_iwe);
        let assign6860_e5255: f64 = (assign6860_e5251 + assign6860_e5254);
        let assign6860_e5258: f64 = (p.p406 * var_iae);
        let assign6860_e5259: f64 = (assign6860_e5255 + assign6860_e5258);
        (assign6860_e5259,)
    } else {
        (var_dphibedge_p,)
    }
};
        var_dphibedge_p = assign6860_e5261;
        var_dphibedge_p_rv = 0.0;

        let (assign6870_e5285,) = {
    if (var_guard36 != 0.0) {
        let assign6870_e5268: f64 = (var_ile).powf(p.p409);
        let assign6870_e5269: f64 = (p.p408 * assign6870_e5268);
        let assign6870_e5270: f64 = (1.0 + assign6870_e5269);
        let assign6870_e5271: f64 = (p.p407 * assign6870_e5270);
        let assign6870_e5275: f64 = (p.p410 * var_iwe);
        let assign6870_e5276: f64 = (1.0 + assign6870_e5275);
        let assign6870_e5277: f64 = (assign6870_e5271 * assign6870_e5276);
        let assign6870_e5281: f64 = (p.p411 * var_iae);
        let assign6870_e5282: f64 = (1.0 + assign6870_e5281);
        let assign6870_e5283: f64 = (assign6870_e5277 * assign6870_e5282);
        (assign6870_e5283,)
    } else {
        (var_neffedge_p,)
    }
};
        var_neffedge_p = assign6870_e5285;
        var_neffedge_p_rv = 0.0;

        let (assign6880_e5295,) = {
    if (var_guard36 != 0.0) {
        let assign6880_e5291: f64 = (var_ile).powf(p.p414);
        let assign6880_e5292: f64 = (p.p413 * assign6880_e5291);
        let assign6880_e5293: f64 = (p.p412 + assign6880_e5292);
        (assign6880_e5293,)
    } else {
        (var_ctedge_p,)
    }
};
        var_ctedge_p = assign6880_e5295;
        var_ctedge_p_rv = 0.0;

        let (assign6890_e5313,) = {
    if (var_guard36 != 0.0) {
        let assign6890_e5300: f64 = (p.p415 * p.p416);
        let assign6890_e5302: f64 = (assign6890_e5300 / var_le);
        let assign6890_e5305: f64 = (-var_le);
        let assign6890_e5307: f64 = (assign6890_e5305 / p.p416);
        let assign6890_e5308: f64 = (assign6890_e5307).exp();
        let assign6890_e5309: f64 = (1.0 - assign6890_e5308);
        let assign6890_e5310: f64 = (assign6890_e5302 * assign6890_e5309);
        let assign6890_e5311: f64 = (1.0 + assign6890_e5310);
        (assign6890_e5311,)
    } else {
        (var_gpe_edge,)
    }
};
        var_gpe_edge = assign6890_e5313;
        var_gpe_edge_rv = 0.0;

        let (assign6900_e5322,) = {
    if (var_guard36 != 0.0) {
        let (assign6900_e5320,) = {
            if (var_gpe_edge > 1e-15) {
                (var_gpe_edge,)
            } else {
                (1e-15,)
            }
        };
        (assign6900_e5320,)
    } else {
        (var_gpe_edge,)
    }
};
        var_gpe_edge = assign6900_e5322;
        var_gpe_edge_rv = 0.0;

        let (assign6910_e5338,) = {
    if (var_guard36 != 0.0) {
        let assign6910_e5326: f64 = (p.p256 * var_we_edge);
        let assign6910_e5329: f64 = (var_gpe_edge * var_le);
        let assign6910_e5330: f64 = (assign6910_e5326 / assign6910_e5329);
        let assign6910_e5334: f64 = (p.p417 * var_iwe);
        let assign6910_e5335: f64 = (1.0 + assign6910_e5334);
        let assign6910_e5336: f64 = (assign6910_e5330 * assign6910_e5335);
        (assign6910_e5336,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign6910_e5338;
        var_betnedge_p_rv = 0.0;

        let (assign6920_e5354,) = {
    if (var_guard36 != 0.0) {
        let assign6920_e5343: f64 = (p.p419 * var_ile);
        let assign6920_e5344: f64 = (p.p418 + assign6920_e5343);
        let assign6920_e5347: f64 = (p.p420 * var_iwe);
        let assign6920_e5348: f64 = (assign6920_e5344 + assign6920_e5347);
        let assign6920_e5351: f64 = (p.p421 * var_iae);
        let assign6920_e5352: f64 = (assign6920_e5348 + assign6920_e5351);
        (assign6920_e5352,)
    } else {
        (var_stbetedge_p,)
    }
};
        var_stbetedge_p = assign6920_e5354;
        var_stbetedge_p_rv = 0.0;

        let (assign6930_e5368,) = {
    if (var_guard36 != 0.0) {
        let assign6930_e5359: f64 = (var_ile).powf(p.p423);
        let assign6930_e5360: f64 = (p.p422 * assign6930_e5359);
        let assign6930_e5364: f64 = (p.p424 * var_iwe);
        let assign6930_e5365: f64 = (1.0 + assign6930_e5364);
        let assign6930_e5366: f64 = (assign6930_e5360 * assign6930_e5365);
        (assign6930_e5366,)
    } else {
        (var_psceedge_p,)
    }
};
        var_psceedge_p = assign6930_e5368;
        var_psceedge_p_rv = 0.0;

        let (assign6940_e5372,) = {
    if (var_guard36 != 0.0) {
        (p.p425,)
    } else {
        (var_pscebedge_p,)
    }
};
        var_pscebedge_p = assign6940_e5372;
        var_pscebedge_p_rv = 0.0;

        let (assign6950_e5376,) = {
    if (var_guard36 != 0.0) {
        (p.p426,)
    } else {
        (var_pscededge_p,)
    }
};
        var_pscededge_p = assign6950_e5376;
        var_pscededge_p_rv = 0.0;

        let (assign6960_e5390,) = {
    if (var_guard36 != 0.0) {
        let assign6960_e5381: f64 = (var_ile).powf(p.p428);
        let assign6960_e5382: f64 = (p.p427 * assign6960_e5381);
        let assign6960_e5386: f64 = (p.p429 * var_iwe);
        let assign6960_e5387: f64 = (1.0 + assign6960_e5386);
        let assign6960_e5388: f64 = (assign6960_e5382 * assign6960_e5387);
        (assign6960_e5388,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign6960_e5390;
        var_cfedge_p_rv = 0.0;

        let (assign6970_e5394,) = {
    if (var_guard36 != 0.0) {
        (p.p431,)
    } else {
        (var_cfdedge_p,)
    }
};
        var_cfdedge_p = assign6970_e5394;
        var_cfdedge_p_rv = 0.0;

        let (assign6980_e5398,) = {
    if (var_guard36 != 0.0) {
        (p.p430,)
    } else {
        (var_cfbedge_p,)
    }
};
        var_cfbedge_p = assign6980_e5398;
        var_cfbedge_p_rv = 0.0;

        let (assign7040_e5440,) = {
    if (var_guard36 != 0.0) {
        let assign7040_e5429: f64 = (p.p808 * var_ile);
        let assign7040_e5430: f64 = (p.p807 + assign7040_e5429);
        let assign7040_e5433: f64 = (p.p809 * var_iwe);
        let assign7040_e5434: f64 = (assign7040_e5430 + assign7040_e5433);
        let assign7040_e5437: f64 = (p.p810 * var_iae);
        let assign7040_e5438: f64 = (assign7040_e5434 + assign7040_e5437);
        (assign7040_e5438,)
    } else {
        (var_kvthowe,)
    }
};
        var_kvthowe = assign7040_e5440;
        var_kvthowe_rv = 0.0;

        let (assign7050_e5456,) = {
    if (var_guard36 != 0.0) {
        let assign7050_e5445: f64 = (p.p812 * var_ile);
        let assign7050_e5446: f64 = (p.p811 + assign7050_e5445);
        let assign7050_e5449: f64 = (p.p813 * var_iwe);
        let assign7050_e5450: f64 = (assign7050_e5446 + assign7050_e5449);
        let assign7050_e5453: f64 = (p.p814 * var_iae);
        let assign7050_e5454: f64 = (assign7050_e5450 + assign7050_e5453);
        (assign7050_e5454,)
    } else {
        (var_kuowe,)
    }
};
        var_kuowe = assign7050_e5456;
        var_kuowe_rv = 0.0;

        let assign7170_e5570: f64 = if (((param_given[448] || param_given[449]) || param_given[450]) || param_given[451]) { 1.0 } else { 0.0 };
        var_guard51 = assign7170_e5570;
        var_guard51_rv = 0.0;

        let (assign7180_e5588,) = {
    if ((var_guard36 != 0.0) && (var_guard51 != 0.0)) {
        let assign7180_e5577: f64 = (p.p449 * var_ile);
        let assign7180_e5578: f64 = (p.p448 + assign7180_e5577);
        let assign7180_e5581: f64 = (p.p450 * var_iwe);
        let assign7180_e5582: f64 = (assign7180_e5578 + assign7180_e5581);
        let assign7180_e5585: f64 = (p.p451 * var_iae);
        let assign7180_e5586: f64 = (assign7180_e5582 + assign7180_e5585);
        (assign7180_e5586,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign7180_e5588;
        var_vfb_p_rv = 0.0;

        let assign7190_e5607: f64 = if (((param_given[452] || param_given[453]) || param_given[454]) || param_given[455]) { 1.0 } else { 0.0 };
        var_guard52 = assign7190_e5607;
        var_guard52_rv = 0.0;

        let (assign7200_e5625,) = {
    if ((var_guard36 != 0.0) && (var_guard52 != 0.0)) {
        let assign7200_e5614: f64 = (p.p453 * var_ile);
        let assign7200_e5615: f64 = (p.p452 + assign7200_e5614);
        let assign7200_e5618: f64 = (p.p454 * var_iwe);
        let assign7200_e5619: f64 = (assign7200_e5615 + assign7200_e5618);
        let assign7200_e5622: f64 = (p.p455 * var_iae);
        let assign7200_e5623: f64 = (assign7200_e5619 + assign7200_e5622);
        (assign7200_e5623,)
    } else {
        (var_stvfb_p,)
    }
};
        var_stvfb_p = assign7200_e5625;
        var_stvfb_p_rv = 0.0;

        let assign7210_e5644: f64 = if (((param_given[456] || param_given[457]) || param_given[458]) || param_given[459]) { 1.0 } else { 0.0 };
        var_guard53 = assign7210_e5644;
        var_guard53_rv = 0.0;

        let (assign7220_e5662,) = {
    if ((var_guard36 != 0.0) && (var_guard53 != 0.0)) {
        let assign7220_e5651: f64 = (p.p457 * var_ile);
        let assign7220_e5652: f64 = (p.p456 + assign7220_e5651);
        let assign7220_e5655: f64 = (p.p458 * var_iwe);
        let assign7220_e5656: f64 = (assign7220_e5652 + assign7220_e5655);
        let assign7220_e5659: f64 = (p.p459 * var_iae);
        let assign7220_e5660: f64 = (assign7220_e5656 + assign7220_e5659);
        (assign7220_e5660,)
    } else {
        (var_neff_p,)
    }
};
        var_neff_p = assign7220_e5662;
        var_neff_p_rv = 0.0;

        let assign7230_e5681: f64 = if (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]) { 1.0 } else { 0.0 };
        var_guard54 = assign7230_e5681;
        var_guard54_rv = 0.0;

        let (assign7240_e5699,) = {
    if ((var_guard36 != 0.0) && (var_guard54 != 0.0)) {
        let assign7240_e5688: f64 = (p.p461 * var_ile);
        let assign7240_e5689: f64 = (p.p460 + assign7240_e5688);
        let assign7240_e5692: f64 = (p.p462 * var_iwe);
        let assign7240_e5693: f64 = (assign7240_e5689 + assign7240_e5692);
        let assign7240_e5696: f64 = (p.p463 * var_iae);
        let assign7240_e5697: f64 = (assign7240_e5693 + assign7240_e5696);
        (assign7240_e5697,)
    } else {
        (var_gfacnud_p,)
    }
};
        var_gfacnud_p = assign7240_e5699;
        var_gfacnud_p_rv = 0.0;

        let assign7250_e5718: f64 = if (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]) { 1.0 } else { 0.0 };
        var_guard55 = assign7250_e5718;
        var_guard55_rv = 0.0;

        let (assign7260_e5736,) = {
    if ((var_guard36 != 0.0) && (var_guard55 != 0.0)) {
        let assign7260_e5725: f64 = (p.p465 * var_ile);
        let assign7260_e5726: f64 = (p.p464 + assign7260_e5725);
        let assign7260_e5729: f64 = (p.p466 * var_iwe);
        let assign7260_e5730: f64 = (assign7260_e5726 + assign7260_e5729);
        let assign7260_e5733: f64 = (p.p467 * var_iae);
        let assign7260_e5734: f64 = (assign7260_e5730 + assign7260_e5733);
        (assign7260_e5734,)
    } else {
        (var_vsbnud_p,)
    }
};
        var_vsbnud_p = assign7260_e5736;
        var_vsbnud_p_rv = 0.0;

        let assign7270_e5755: f64 = if (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]) { 1.0 } else { 0.0 };
        var_guard56 = assign7270_e5755;
        var_guard56_rv = 0.0;

        let (assign7280_e5773,) = {
    if ((var_guard36 != 0.0) && (var_guard56 != 0.0)) {
        let assign7280_e5762: f64 = (p.p469 * var_ile);
        let assign7280_e5763: f64 = (p.p468 + assign7280_e5762);
        let assign7280_e5766: f64 = (p.p470 * var_iwe);
        let assign7280_e5767: f64 = (assign7280_e5763 + assign7280_e5766);
        let assign7280_e5770: f64 = (p.p471 * var_iae);
        let assign7280_e5771: f64 = (assign7280_e5767 + assign7280_e5770);
        (assign7280_e5771,)
    } else {
        (var_dphib_p,)
    }
};
        var_dphib_p = assign7280_e5773;
        var_dphib_p_rv = 0.0;

        let assign7290_e5792: f64 = if (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]) { 1.0 } else { 0.0 };
        var_guard57 = assign7290_e5792;
        var_guard57_rv = 0.0;

        let (assign7300_e5810,) = {
    if ((var_guard36 != 0.0) && (var_guard57 != 0.0)) {
        let assign7300_e5799: f64 = (p.p473 * var_ile);
        let assign7300_e5800: f64 = (p.p472 + assign7300_e5799);
        let assign7300_e5803: f64 = (p.p474 * var_iwe);
        let assign7300_e5804: f64 = (assign7300_e5800 + assign7300_e5803);
        let assign7300_e5807: f64 = (p.p475 * var_iae);
        let assign7300_e5808: f64 = (assign7300_e5804 + assign7300_e5807);
        (assign7300_e5808,)
    } else {
        (var_np_p,)
    }
};
        var_np_p = assign7300_e5810;
        var_np_p_rv = 0.0;

        let assign7310_e5829: f64 = if (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]) { 1.0 } else { 0.0 };
        var_guard58 = assign7310_e5829;
        var_guard58_rv = 0.0;

        let (assign7320_e5847,) = {
    if ((var_guard36 != 0.0) && (var_guard58 != 0.0)) {
        let assign7320_e5836: f64 = (p.p477 * var_ile);
        let assign7320_e5837: f64 = (p.p476 + assign7320_e5836);
        let assign7320_e5840: f64 = (p.p478 * var_iwe);
        let assign7320_e5841: f64 = (assign7320_e5837 + assign7320_e5840);
        let assign7320_e5844: f64 = (p.p479 * var_iae);
        let assign7320_e5845: f64 = (assign7320_e5841 + assign7320_e5844);
        (assign7320_e5845,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign7320_e5847;
        var_nov_p_rv = 0.0;

        let assign7330_e5866: f64 = if (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]) { 1.0 } else { 0.0 };
        var_guard59 = assign7330_e5866;
        var_guard59_rv = 0.0;

        let (assign7340_e5884,) = {
    if ((var_guard36 != 0.0) && (var_guard59 != 0.0)) {
        let assign7340_e5873: f64 = (p.p481 * var_ile);
        let assign7340_e5874: f64 = (p.p480 + assign7340_e5873);
        let assign7340_e5877: f64 = (p.p482 * var_iwe);
        let assign7340_e5878: f64 = (assign7340_e5874 + assign7340_e5877);
        let assign7340_e5881: f64 = (p.p483 * var_iae);
        let assign7340_e5882: f64 = (assign7340_e5878 + assign7340_e5881);
        (assign7340_e5882,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign7340_e5884;
        var_novd_p_rv = 0.0;

        let assign7350_e5903: f64 = if (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]) { 1.0 } else { 0.0 };
        var_guard60 = assign7350_e5903;
        var_guard60_rv = 0.0;

        let (assign7360_e5921,) = {
    if ((var_guard36 != 0.0) && (var_guard60 != 0.0)) {
        let assign7360_e5910: f64 = (p.p485 * var_ile);
        let assign7360_e5911: f64 = (p.p484 + assign7360_e5910);
        let assign7360_e5914: f64 = (p.p486 * var_iwe);
        let assign7360_e5915: f64 = (assign7360_e5911 + assign7360_e5914);
        let assign7360_e5918: f64 = (p.p487 * var_iae);
        let assign7360_e5919: f64 = (assign7360_e5915 + assign7360_e5918);
        (assign7360_e5919,)
    } else {
        (var_ct_p,)
    }
};
        var_ct_p = assign7360_e5921;
        var_ct_p_rv = 0.0;

        let assign7370_e5940: f64 = if (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]) { 1.0 } else { 0.0 };
        var_guard61 = assign7370_e5940;
        var_guard61_rv = 0.0;

        let (assign7380_e5958,) = {
    if ((var_guard36 != 0.0) && (var_guard61 != 0.0)) {
        let assign7380_e5947: f64 = (p.p493 * var_ile);
        let assign7380_e5948: f64 = (p.p492 + assign7380_e5947);
        let assign7380_e5951: f64 = (p.p494 * var_iwe);
        let assign7380_e5952: f64 = (assign7380_e5948 + assign7380_e5951);
        let assign7380_e5955: f64 = (p.p495 * var_iae);
        let assign7380_e5956: f64 = (assign7380_e5952 + assign7380_e5955);
        (assign7380_e5956,)
    } else {
        (var_ctg_p,)
    }
};
        var_ctg_p = assign7380_e5958;
        var_ctg_p_rv = 0.0;

        let assign7390_e5977: f64 = if (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]) { 1.0 } else { 0.0 };
        var_guard62 = assign7390_e5977;
        var_guard62_rv = 0.0;

        let (assign7400_e5995,) = {
    if ((var_guard36 != 0.0) && (var_guard62 != 0.0)) {
        let assign7400_e5984: f64 = (p.p489 * var_ile);
        let assign7400_e5985: f64 = (p.p488 + assign7400_e5984);
        let assign7400_e5988: f64 = (p.p490 * var_iwe);
        let assign7400_e5989: f64 = (assign7400_e5985 + assign7400_e5988);
        let assign7400_e5992: f64 = (p.p491 * var_iae);
        let assign7400_e5993: f64 = (assign7400_e5989 + assign7400_e5992);
        (assign7400_e5993,)
    } else {
        (var_ctb_p,)
    }
};
        var_ctb_p = assign7400_e5995;
        var_ctb_p_rv = 0.0;

        let assign7410_e6014: f64 = if (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]) { 1.0 } else { 0.0 };
        var_guard63 = assign7410_e6014;
        var_guard63_rv = 0.0;

        let (assign7420_e6032,) = {
    if ((var_guard36 != 0.0) && (var_guard63 != 0.0)) {
        let assign7420_e6021: f64 = (p.p497 * var_ile);
        let assign7420_e6022: f64 = (p.p496 + assign7420_e6021);
        let assign7420_e6025: f64 = (p.p498 * var_iwe);
        let assign7420_e6026: f64 = (assign7420_e6022 + assign7420_e6025);
        let assign7420_e6029: f64 = (p.p499 * var_iae);
        let assign7420_e6030: f64 = (assign7420_e6026 + assign7420_e6029);
        (assign7420_e6030,)
    } else {
        (var_stct_p,)
    }
};
        var_stct_p = assign7420_e6032;
        var_stct_p_rv = 0.0;

        let assign7430_e6051: f64 = if (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]) { 1.0 } else { 0.0 };
        var_guard64 = assign7430_e6051;
        var_guard64_rv = 0.0;

        let (assign7440_e6071,) = {
    if ((var_guard36 != 0.0) && (var_guard64 != 0.0)) {
        let assign7440_e6059: f64 = (p.p501 * var_ile);
        let assign7440_e6060: f64 = (p.p500 + assign7440_e6059);
        let assign7440_e6063: f64 = (p.p502 * var_iwe);
        let assign7440_e6064: f64 = (assign7440_e6060 + assign7440_e6063);
        let assign7440_e6067: f64 = (p.p503 * var_iae);
        let assign7440_e6068: f64 = (assign7440_e6064 + assign7440_e6067);
        let assign7440_e6069: f64 = (var_ile2 * assign7440_e6068);
        (assign7440_e6069,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign7440_e6071;
        var_cf_p_rv = 0.0;

        let assign7450_e6090: f64 = if (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]) { 1.0 } else { 0.0 };
        var_guard65 = assign7450_e6090;
        var_guard65_rv = 0.0;

        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfbedge_p_rv_slot = var_cfbedge_p_rv;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfdedge_p_rv_slot = var_cfdedge_p_rv;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfedge_p_rv_slot = var_cfedge_p_rv;
        *var_ct_p_slot = var_ct_p;
        *var_ct_p_rv_slot = var_ct_p_rv;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctb_p_rv_slot = var_ctb_p_rv;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_ctedge_p_rv_slot = var_ctedge_p_rv;
        *var_ctg_p_slot = var_ctg_p;
        *var_ctg_p_rv_slot = var_ctg_p_rv;
        *var_dphib_p_slot = var_dphib_p;
        *var_dphib_p_rv_slot = var_dphib_p_rv;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_dphibedge_p_rv_slot = var_dphibedge_p_rv;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_gfacnud_p_rv_slot = var_gfacnud_p_rv;
        *var_gpe_edge_slot = var_gpe_edge;
        *var_gpe_edge_rv_slot = var_gpe_edge_rv;
        *var_guard51_slot = var_guard51;
        *var_guard51_rv_slot = var_guard51_rv;
        *var_guard52_slot = var_guard52;
        *var_guard52_rv_slot = var_guard52_rv;
        *var_guard53_slot = var_guard53;
        *var_guard53_rv_slot = var_guard53_rv;
        *var_guard54_slot = var_guard54;
        *var_guard54_rv_slot = var_guard54_rv;
        *var_guard55_slot = var_guard55;
        *var_guard55_rv_slot = var_guard55_rv;
        *var_guard56_slot = var_guard56;
        *var_guard56_rv_slot = var_guard56_rv;
        *var_guard57_slot = var_guard57;
        *var_guard57_rv_slot = var_guard57_rv;
        *var_guard58_slot = var_guard58;
        *var_guard58_rv_slot = var_guard58_rv;
        *var_guard59_slot = var_guard59;
        *var_guard59_rv_slot = var_guard59_rv;
        *var_guard60_slot = var_guard60;
        *var_guard60_rv_slot = var_guard60_rv;
        *var_guard61_slot = var_guard61;
        *var_guard61_rv_slot = var_guard61_rv;
        *var_guard62_slot = var_guard62;
        *var_guard62_rv_slot = var_guard62_rv;
        *var_guard63_slot = var_guard63;
        *var_guard63_rv_slot = var_guard63_rv;
        *var_guard64_slot = var_guard64;
        *var_guard64_rv_slot = var_guard64_rv;
        *var_guard65_slot = var_guard65;
        *var_guard65_rv_slot = var_guard65_rv;
        *var_kuowe_slot = var_kuowe;
        *var_kuowe_rv_slot = var_kuowe_rv;
        *var_kvthowe_slot = var_kvthowe;
        *var_kvthowe_rv_slot = var_kvthowe_rv;
        *var_neff_p_slot = var_neff_p;
        *var_neff_p_rv_slot = var_neff_p_rv;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_neffedge_p_rv_slot = var_neffedge_p_rv;
        *var_nov_p_slot = var_nov_p;
        *var_nov_p_rv_slot = var_nov_p_rv;
        *var_novd_p_slot = var_novd_p;
        *var_novd_p_rv_slot = var_novd_p_rv;
        *var_np_p_slot = var_np_p;
        *var_np_p_rv_slot = var_np_p_rv;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscebedge_p_rv_slot = var_pscebedge_p_rv;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_pscededge_p_rv_slot = var_pscededge_p_rv;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_psceedge_p_rv_slot = var_psceedge_p_rv;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stbetedge_p_rv_slot = var_stbetedge_p_rv;
        *var_stct_p_slot = var_stct_p;
        *var_stct_p_rv_slot = var_stct_p_rv;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_stvfb_p_rv_slot = var_stvfb_p_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vsbnud_p_slot = var_vsbnud_p;
        *var_vsbnud_p_rv_slot = var_vsbnud_p_rv;
    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_guard65: f64,
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_a1_p_slot: &mut f64,
        var_a1_p_rv_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp1_p_rv_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp2_p_rv_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_alp_p_rv_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_ax_p_rv_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfb_p_rv_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_cfd_p_rv_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_cs_p_rv_slot: &mut f64,
        var_guard66_slot: &mut f64,
        var_guard66_rv_slot: &mut f64,
        var_guard67_slot: &mut f64,
        var_guard67_rv_slot: &mut f64,
        var_guard68_slot: &mut f64,
        var_guard68_rv_slot: &mut f64,
        var_guard69_slot: &mut f64,
        var_guard69_rv_slot: &mut f64,
        var_guard70_slot: &mut f64,
        var_guard70_rv_slot: &mut f64,
        var_guard71_slot: &mut f64,
        var_guard71_rv_slot: &mut f64,
        var_guard72_slot: &mut f64,
        var_guard72_rv_slot: &mut f64,
        var_guard73_slot: &mut f64,
        var_guard73_rv_slot: &mut f64,
        var_guard74_slot: &mut f64,
        var_guard74_rv_slot: &mut f64,
        var_guard75_slot: &mut f64,
        var_guard75_rv_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard76_rv_slot: &mut f64,
        var_guard77_slot: &mut f64,
        var_guard77_rv_slot: &mut f64,
        var_guard78_slot: &mut f64,
        var_guard78_rv_slot: &mut f64,
        var_guard79_slot: &mut f64,
        var_guard79_rv_slot: &mut f64,
        var_guard80_slot: &mut f64,
        var_guard80_rv_slot: &mut f64,
        var_guard81_slot: &mut f64,
        var_guard81_rv_slot: &mut f64,
        var_guard82_slot: &mut f64,
        var_guard82_rv_slot: &mut f64,
        var_guard83_slot: &mut f64,
        var_guard83_rv_slot: &mut f64,
        var_guard84_slot: &mut f64,
        var_guard84_rv_slot: &mut f64,
        var_guard85_slot: &mut f64,
        var_guard85_rv_slot: &mut f64,
        var_guard86_slot: &mut f64,
        var_guard86_rv_slot: &mut f64,
        var_guard87_slot: &mut f64,
        var_guard87_rv_slot: &mut f64,
        var_guard88_slot: &mut f64,
        var_guard88_rv_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_guard89_rv_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard90_rv_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_mue_p_rv_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psce_p_rv_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psceb_p_rv_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_psced_p_rv_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rs_p_rv_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsb_p_rv_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_rsg_p_rv_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stbet_p_rv_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_strs_p_rv_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stthesat_p_rv_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_thecs_p_rv_slot: &mut f64,
        var_themu_p_slot: &mut f64,
        var_themu_p_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesatb_p_slot: &mut f64,
        var_thesatb_p_rv_slot: &mut f64,
        var_thesatg_p_slot: &mut f64,
        var_thesatg_p_rv_slot: &mut f64,
        var_xcor_p_slot: &mut f64,
        var_xcor_p_rv_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a1_p_rv: f64 = *var_a1_p_rv_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp1_p_rv: f64 = *var_alp1_p_rv_slot;
        let mut var_alp2_p: f64 = *var_alp2_p_slot;
        let mut var_alp2_p_rv: f64 = *var_alp2_p_rv_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_alp_p_rv: f64 = *var_alp_p_rv_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_ax_p_rv: f64 = *var_ax_p_rv_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfb_p_rv: f64 = *var_cfb_p_rv_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_cfd_p_rv: f64 = *var_cfd_p_rv_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_cs_p_rv: f64 = *var_cs_p_rv_slot;
        let mut var_guard66: f64 = *var_guard66_slot;
        let mut var_guard66_rv: f64 = *var_guard66_rv_slot;
        let mut var_guard67: f64 = *var_guard67_slot;
        let mut var_guard67_rv: f64 = *var_guard67_rv_slot;
        let mut var_guard68: f64 = *var_guard68_slot;
        let mut var_guard68_rv: f64 = *var_guard68_rv_slot;
        let mut var_guard69: f64 = *var_guard69_slot;
        let mut var_guard69_rv: f64 = *var_guard69_rv_slot;
        let mut var_guard70: f64 = *var_guard70_slot;
        let mut var_guard70_rv: f64 = *var_guard70_rv_slot;
        let mut var_guard71: f64 = *var_guard71_slot;
        let mut var_guard71_rv: f64 = *var_guard71_rv_slot;
        let mut var_guard72: f64 = *var_guard72_slot;
        let mut var_guard72_rv: f64 = *var_guard72_rv_slot;
        let mut var_guard73: f64 = *var_guard73_slot;
        let mut var_guard73_rv: f64 = *var_guard73_rv_slot;
        let mut var_guard74: f64 = *var_guard74_slot;
        let mut var_guard74_rv: f64 = *var_guard74_rv_slot;
        let mut var_guard75: f64 = *var_guard75_slot;
        let mut var_guard75_rv: f64 = *var_guard75_rv_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard76_rv: f64 = *var_guard76_rv_slot;
        let mut var_guard77: f64 = *var_guard77_slot;
        let mut var_guard77_rv: f64 = *var_guard77_rv_slot;
        let mut var_guard78: f64 = *var_guard78_slot;
        let mut var_guard78_rv: f64 = *var_guard78_rv_slot;
        let mut var_guard79: f64 = *var_guard79_slot;
        let mut var_guard79_rv: f64 = *var_guard79_rv_slot;
        let mut var_guard80: f64 = *var_guard80_slot;
        let mut var_guard80_rv: f64 = *var_guard80_rv_slot;
        let mut var_guard81: f64 = *var_guard81_slot;
        let mut var_guard81_rv: f64 = *var_guard81_rv_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
        let mut var_guard82_rv: f64 = *var_guard82_rv_slot;
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_guard83_rv: f64 = *var_guard83_rv_slot;
        let mut var_guard84: f64 = *var_guard84_slot;
        let mut var_guard84_rv: f64 = *var_guard84_rv_slot;
        let mut var_guard85: f64 = *var_guard85_slot;
        let mut var_guard85_rv: f64 = *var_guard85_rv_slot;
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard86_rv: f64 = *var_guard86_rv_slot;
        let mut var_guard87: f64 = *var_guard87_slot;
        let mut var_guard87_rv: f64 = *var_guard87_rv_slot;
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_guard88_rv: f64 = *var_guard88_rv_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_guard89_rv: f64 = *var_guard89_rv_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard90_rv: f64 = *var_guard90_rv_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_mue_p_rv: f64 = *var_mue_p_rv_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psce_p_rv: f64 = *var_psce_p_rv_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psceb_p_rv: f64 = *var_psceb_p_rv_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_psced_p_rv: f64 = *var_psced_p_rv_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rs_p_rv: f64 = *var_rs_p_rv_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsb_p_rv: f64 = *var_rsb_p_rv_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_rsg_p_rv: f64 = *var_rsg_p_rv_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stbet_p_rv: f64 = *var_stbet_p_rv_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_strs_p_rv: f64 = *var_strs_p_rv_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stthesat_p_rv: f64 = *var_stthesat_p_rv_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_thecs_p_rv: f64 = *var_thecs_p_rv_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_themu_p_rv: f64 = *var_themu_p_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_thesatb_p_rv: f64 = *var_thesatb_p_rv_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;
        let mut var_thesatg_p_rv: f64 = *var_thesatg_p_rv_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;
        let mut var_xcor_p_rv: f64 = *var_xcor_p_rv_slot;

        let (assign7460_e6108,) = {
    if ((var_guard36 != 0.0) && (var_guard65 != 0.0)) {
        let assign7460_e6097: f64 = (p.p509 * var_ile);
        let assign7460_e6098: f64 = (p.p508 + assign7460_e6097);
        let assign7460_e6101: f64 = (p.p510 * var_iwe);
        let assign7460_e6102: f64 = (assign7460_e6098 + assign7460_e6101);
        let assign7460_e6105: f64 = (p.p511 * var_iae);
        let assign7460_e6106: f64 = (assign7460_e6102 + assign7460_e6105);
        (assign7460_e6106,)
    } else {
        (var_cfd_p,)
    }
};
        var_cfd_p = assign7460_e6108;
        var_cfd_p_rv = 0.0;

        let assign7470_e6127: f64 = if (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]) { 1.0 } else { 0.0 };
        var_guard66 = assign7470_e6127;
        var_guard66_rv = 0.0;

        let (assign7480_e6145,) = {
    if ((var_guard36 != 0.0) && (var_guard66 != 0.0)) {
        let assign7480_e6134: f64 = (p.p505 * var_ile);
        let assign7480_e6135: f64 = (p.p504 + assign7480_e6134);
        let assign7480_e6138: f64 = (p.p506 * var_iwe);
        let assign7480_e6139: f64 = (assign7480_e6135 + assign7480_e6138);
        let assign7480_e6142: f64 = (p.p507 * var_iae);
        let assign7480_e6143: f64 = (assign7480_e6139 + assign7480_e6142);
        (assign7480_e6143,)
    } else {
        (var_cfb_p,)
    }
};
        var_cfb_p = assign7480_e6145;
        var_cfb_p_rv = 0.0;

        let assign7490_e6164: f64 = if (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]) { 1.0 } else { 0.0 };
        var_guard67 = assign7490_e6164;
        var_guard67_rv = 0.0;

        let (assign7500_e6184,) = {
    if ((var_guard36 != 0.0) && (var_guard67 != 0.0)) {
        let assign7500_e6172: f64 = (p.p513 * var_ile);
        let assign7500_e6173: f64 = (p.p512 + assign7500_e6172);
        let assign7500_e6176: f64 = (p.p514 * var_iwe);
        let assign7500_e6177: f64 = (assign7500_e6173 + assign7500_e6176);
        let assign7500_e6180: f64 = (p.p515 * var_iae);
        let assign7500_e6181: f64 = (assign7500_e6177 + assign7500_e6180);
        let assign7500_e6182: f64 = (var_ile2 * assign7500_e6181);
        (assign7500_e6182,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign7500_e6184;
        var_psce_p_rv = 0.0;

        let assign7510_e6203: f64 = if (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]) { 1.0 } else { 0.0 };
        var_guard68 = assign7510_e6203;
        var_guard68_rv = 0.0;

        let (assign7520_e6221,) = {
    if ((var_guard36 != 0.0) && (var_guard68 != 0.0)) {
        let assign7520_e6210: f64 = (p.p521 * var_ile);
        let assign7520_e6211: f64 = (p.p520 + assign7520_e6210);
        let assign7520_e6214: f64 = (p.p522 * var_iwe);
        let assign7520_e6215: f64 = (assign7520_e6211 + assign7520_e6214);
        let assign7520_e6218: f64 = (p.p523 * var_iae);
        let assign7520_e6219: f64 = (assign7520_e6215 + assign7520_e6218);
        (assign7520_e6219,)
    } else {
        (var_psced_p,)
    }
};
        var_psced_p = assign7520_e6221;
        var_psced_p_rv = 0.0;

        let assign7530_e6240: f64 = if (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]) { 1.0 } else { 0.0 };
        var_guard69 = assign7530_e6240;
        var_guard69_rv = 0.0;

        let (assign7540_e6258,) = {
    if ((var_guard36 != 0.0) && (var_guard69 != 0.0)) {
        let assign7540_e6247: f64 = (p.p517 * var_ile);
        let assign7540_e6248: f64 = (p.p516 + assign7540_e6247);
        let assign7540_e6251: f64 = (p.p518 * var_iwe);
        let assign7540_e6252: f64 = (assign7540_e6248 + assign7540_e6251);
        let assign7540_e6255: f64 = (p.p519 * var_iae);
        let assign7540_e6256: f64 = (assign7540_e6252 + assign7540_e6255);
        (assign7540_e6256,)
    } else {
        (var_psceb_p,)
    }
};
        var_psceb_p = assign7540_e6258;
        var_psceb_p_rv = 0.0;

        let assign7550_e6277: f64 = if (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]) { 1.0 } else { 0.0 };
        var_guard70 = assign7550_e6277;
        var_guard70_rv = 0.0;

        let (assign7560_e6299,) = {
    if ((var_guard36 != 0.0) && (var_guard70 != 0.0)) {
        let assign7560_e6283: f64 = (var_we / var_le);
        let assign7560_e6287: f64 = (p.p525 * var_ile);
        let assign7560_e6288: f64 = (p.p524 + assign7560_e6287);
        let assign7560_e6291: f64 = (p.p526 * var_iwe);
        let assign7560_e6292: f64 = (assign7560_e6288 + assign7560_e6291);
        let assign7560_e6295: f64 = (p.p527 * var_iae);
        let assign7560_e6296: f64 = (assign7560_e6292 + assign7560_e6295);
        let assign7560_e6297: f64 = (assign7560_e6283 * assign7560_e6296);
        (assign7560_e6297,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign7560_e6299;
        var_betn_p_rv = 0.0;

        let assign7570_e6318: f64 = if (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]) { 1.0 } else { 0.0 };
        var_guard71 = assign7570_e6318;
        var_guard71_rv = 0.0;

        let (assign7580_e6336,) = {
    if ((var_guard36 != 0.0) && (var_guard71 != 0.0)) {
        let assign7580_e6325: f64 = (p.p529 * var_ile);
        let assign7580_e6326: f64 = (p.p528 + assign7580_e6325);
        let assign7580_e6329: f64 = (p.p530 * var_iwe);
        let assign7580_e6330: f64 = (assign7580_e6326 + assign7580_e6329);
        let assign7580_e6333: f64 = (p.p531 * var_iae);
        let assign7580_e6334: f64 = (assign7580_e6330 + assign7580_e6333);
        (assign7580_e6334,)
    } else {
        (var_stbet_p,)
    }
};
        var_stbet_p = assign7580_e6336;
        var_stbet_p_rv = 0.0;

        let assign7590_e6355: f64 = if (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]) { 1.0 } else { 0.0 };
        var_guard72 = assign7590_e6355;
        var_guard72_rv = 0.0;

        let (assign7600_e6373,) = {
    if ((var_guard36 != 0.0) && (var_guard72 != 0.0)) {
        let assign7600_e6362: f64 = (p.p533 * var_ile);
        let assign7600_e6363: f64 = (p.p532 + assign7600_e6362);
        let assign7600_e6366: f64 = (p.p534 * var_iwe);
        let assign7600_e6367: f64 = (assign7600_e6363 + assign7600_e6366);
        let assign7600_e6370: f64 = (p.p535 * var_iae);
        let assign7600_e6371: f64 = (assign7600_e6367 + assign7600_e6370);
        (assign7600_e6371,)
    } else {
        (var_mue_p,)
    }
};
        var_mue_p = assign7600_e6373;
        var_mue_p_rv = 0.0;

        let assign7610_e6392: f64 = if (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]) { 1.0 } else { 0.0 };
        var_guard73 = assign7610_e6392;
        var_guard73_rv = 0.0;

        let (assign7620_e6410,) = {
    if ((var_guard36 != 0.0) && (var_guard73 != 0.0)) {
        let assign7620_e6399: f64 = (p.p537 * var_ile);
        let assign7620_e6400: f64 = (p.p536 + assign7620_e6399);
        let assign7620_e6403: f64 = (p.p538 * var_iwe);
        let assign7620_e6404: f64 = (assign7620_e6400 + assign7620_e6403);
        let assign7620_e6407: f64 = (p.p539 * var_iae);
        let assign7620_e6408: f64 = (assign7620_e6404 + assign7620_e6407);
        (assign7620_e6408,)
    } else {
        (var_themu_p,)
    }
};
        var_themu_p = assign7620_e6410;
        var_themu_p_rv = 0.0;

        let assign7630_e6429: f64 = if (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]) { 1.0 } else { 0.0 };
        var_guard74 = assign7630_e6429;
        var_guard74_rv = 0.0;

        let (assign7640_e6447,) = {
    if ((var_guard36 != 0.0) && (var_guard74 != 0.0)) {
        let assign7640_e6436: f64 = (p.p541 * var_ile);
        let assign7640_e6437: f64 = (p.p540 + assign7640_e6436);
        let assign7640_e6440: f64 = (p.p542 * var_iwe);
        let assign7640_e6441: f64 = (assign7640_e6437 + assign7640_e6440);
        let assign7640_e6444: f64 = (p.p543 * var_iae);
        let assign7640_e6445: f64 = (assign7640_e6441 + assign7640_e6444);
        (assign7640_e6445,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign7640_e6447;
        var_cs_p_rv = 0.0;

        let assign7650_e6466: f64 = if (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]) { 1.0 } else { 0.0 };
        var_guard75 = assign7650_e6466;
        var_guard75_rv = 0.0;

        let (assign7660_e6484,) = {
    if ((var_guard36 != 0.0) && (var_guard75 != 0.0)) {
        let assign7660_e6473: f64 = (p.p545 * var_ile);
        let assign7660_e6474: f64 = (p.p544 + assign7660_e6473);
        let assign7660_e6477: f64 = (p.p546 * var_iwe);
        let assign7660_e6478: f64 = (assign7660_e6474 + assign7660_e6477);
        let assign7660_e6481: f64 = (p.p547 * var_iae);
        let assign7660_e6482: f64 = (assign7660_e6478 + assign7660_e6481);
        (assign7660_e6482,)
    } else {
        (var_thecs_p,)
    }
};
        var_thecs_p = assign7660_e6484;
        var_thecs_p_rv = 0.0;

        let assign7670_e6503: f64 = if (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]) { 1.0 } else { 0.0 };
        var_guard76 = assign7670_e6503;
        var_guard76_rv = 0.0;

        let (assign7680_e6521,) = {
    if ((var_guard36 != 0.0) && (var_guard76 != 0.0)) {
        let assign7680_e6510: f64 = (p.p549 * var_ile);
        let assign7680_e6511: f64 = (p.p548 + assign7680_e6510);
        let assign7680_e6514: f64 = (p.p550 * var_iwe);
        let assign7680_e6515: f64 = (assign7680_e6511 + assign7680_e6514);
        let assign7680_e6518: f64 = (p.p551 * var_iae);
        let assign7680_e6519: f64 = (assign7680_e6515 + assign7680_e6518);
        (assign7680_e6519,)
    } else {
        (var_xcor_p,)
    }
};
        var_xcor_p = assign7680_e6521;
        var_xcor_p_rv = 0.0;

        let assign7690_e6540: f64 = if (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]) { 1.0 } else { 0.0 };
        var_guard77 = assign7690_e6540;
        var_guard77_rv = 0.0;

        let (assign7700_e6560,) = {
    if ((var_guard36 != 0.0) && (var_guard77 != 0.0)) {
        let assign7700_e6548: f64 = (p.p553 * var_ile);
        let assign7700_e6549: f64 = (p.p552 + assign7700_e6548);
        let assign7700_e6552: f64 = (p.p554 * var_iwe);
        let assign7700_e6553: f64 = (assign7700_e6549 + assign7700_e6552);
        let assign7700_e6556: f64 = (p.p555 * var_iae);
        let assign7700_e6557: f64 = (assign7700_e6553 + assign7700_e6556);
        let assign7700_e6558: f64 = (var_iwe * assign7700_e6557);
        (assign7700_e6558,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign7700_e6560;
        var_rs_p_rv = 0.0;

        let assign7710_e6579: f64 = if (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]) { 1.0 } else { 0.0 };
        var_guard78 = assign7710_e6579;
        var_guard78_rv = 0.0;

        let (assign7720_e6597,) = {
    if ((var_guard36 != 0.0) && (var_guard78 != 0.0)) {
        let assign7720_e6586: f64 = (p.p557 * var_ile);
        let assign7720_e6587: f64 = (p.p556 + assign7720_e6586);
        let assign7720_e6590: f64 = (p.p558 * var_iwe);
        let assign7720_e6591: f64 = (assign7720_e6587 + assign7720_e6590);
        let assign7720_e6594: f64 = (p.p559 * var_iae);
        let assign7720_e6595: f64 = (assign7720_e6591 + assign7720_e6594);
        (assign7720_e6595,)
    } else {
        (var_strs_p,)
    }
};
        var_strs_p = assign7720_e6597;
        var_strs_p_rv = 0.0;

        let assign7730_e6616: f64 = if (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]) { 1.0 } else { 0.0 };
        var_guard79 = assign7730_e6616;
        var_guard79_rv = 0.0;

        let (assign7740_e6634,) = {
    if ((var_guard36 != 0.0) && (var_guard79 != 0.0)) {
        let assign7740_e6623: f64 = (p.p561 * var_ile);
        let assign7740_e6624: f64 = (p.p560 + assign7740_e6623);
        let assign7740_e6627: f64 = (p.p562 * var_iwe);
        let assign7740_e6628: f64 = (assign7740_e6624 + assign7740_e6627);
        let assign7740_e6631: f64 = (p.p563 * var_iae);
        let assign7740_e6632: f64 = (assign7740_e6628 + assign7740_e6631);
        (assign7740_e6632,)
    } else {
        (var_rsb_p,)
    }
};
        var_rsb_p = assign7740_e6634;
        var_rsb_p_rv = 0.0;

        let assign7750_e6653: f64 = if (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]) { 1.0 } else { 0.0 };
        var_guard80 = assign7750_e6653;
        var_guard80_rv = 0.0;

        let (assign7760_e6671,) = {
    if ((var_guard36 != 0.0) && (var_guard80 != 0.0)) {
        let assign7760_e6660: f64 = (p.p565 * var_ile);
        let assign7760_e6661: f64 = (p.p564 + assign7760_e6660);
        let assign7760_e6664: f64 = (p.p566 * var_iwe);
        let assign7760_e6665: f64 = (assign7760_e6661 + assign7760_e6664);
        let assign7760_e6668: f64 = (p.p567 * var_iae);
        let assign7760_e6669: f64 = (assign7760_e6665 + assign7760_e6668);
        (assign7760_e6669,)
    } else {
        (var_rsg_p,)
    }
};
        var_rsg_p = assign7760_e6671;
        var_rsg_p_rv = 0.0;

        let assign7770_e6690: f64 = if (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        var_guard81 = assign7770_e6690;
        var_guard81_rv = 0.0;

        let (assign7780_e6710,) = {
    if ((var_guard36 != 0.0) && (var_guard81 != 0.0)) {
        let assign7780_e6698: f64 = (p.p569 * var_ile);
        let assign7780_e6699: f64 = (p.p568 + assign7780_e6698);
        let assign7780_e6702: f64 = (p.p570 * var_iwe);
        let assign7780_e6703: f64 = (assign7780_e6699 + assign7780_e6702);
        let assign7780_e6706: f64 = (p.p571 * var_iae);
        let assign7780_e6707: f64 = (assign7780_e6703 + assign7780_e6706);
        let assign7780_e6708: f64 = (var_ile * assign7780_e6707);
        (assign7780_e6708,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign7780_e6710;
        var_thesat_p_rv = 0.0;

        let assign7790_e6729: f64 = if (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]) { 1.0 } else { 0.0 };
        var_guard82 = assign7790_e6729;
        var_guard82_rv = 0.0;

        let (assign7800_e6747,) = {
    if ((var_guard36 != 0.0) && (var_guard82 != 0.0)) {
        let assign7800_e6736: f64 = (p.p573 * var_ile);
        let assign7800_e6737: f64 = (p.p572 + assign7800_e6736);
        let assign7800_e6740: f64 = (p.p574 * var_iwe);
        let assign7800_e6741: f64 = (assign7800_e6737 + assign7800_e6740);
        let assign7800_e6744: f64 = (p.p575 * var_iae);
        let assign7800_e6745: f64 = (assign7800_e6741 + assign7800_e6744);
        (assign7800_e6745,)
    } else {
        (var_stthesat_p,)
    }
};
        var_stthesat_p = assign7800_e6747;
        var_stthesat_p_rv = 0.0;

        let assign7810_e6766: f64 = if (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]) { 1.0 } else { 0.0 };
        var_guard83 = assign7810_e6766;
        var_guard83_rv = 0.0;

        let (assign7820_e6784,) = {
    if ((var_guard36 != 0.0) && (var_guard83 != 0.0)) {
        let assign7820_e6773: f64 = (p.p577 * var_ile);
        let assign7820_e6774: f64 = (p.p576 + assign7820_e6773);
        let assign7820_e6777: f64 = (p.p578 * var_iwe);
        let assign7820_e6778: f64 = (assign7820_e6774 + assign7820_e6777);
        let assign7820_e6781: f64 = (p.p579 * var_iae);
        let assign7820_e6782: f64 = (assign7820_e6778 + assign7820_e6781);
        (assign7820_e6782,)
    } else {
        (var_thesatb_p,)
    }
};
        var_thesatb_p = assign7820_e6784;
        var_thesatb_p_rv = 0.0;

        let assign7830_e6803: f64 = if (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        var_guard84 = assign7830_e6803;
        var_guard84_rv = 0.0;

        let (assign7840_e6821,) = {
    if ((var_guard36 != 0.0) && (var_guard84 != 0.0)) {
        let assign7840_e6810: f64 = (p.p581 * var_ile);
        let assign7840_e6811: f64 = (p.p580 + assign7840_e6810);
        let assign7840_e6814: f64 = (p.p582 * var_iwe);
        let assign7840_e6815: f64 = (assign7840_e6811 + assign7840_e6814);
        let assign7840_e6818: f64 = (p.p583 * var_iae);
        let assign7840_e6819: f64 = (assign7840_e6815 + assign7840_e6818);
        (assign7840_e6819,)
    } else {
        (var_thesatg_p,)
    }
};
        var_thesatg_p = assign7840_e6821;
        var_thesatg_p_rv = 0.0;

        let assign7850_e6840: f64 = if (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        var_guard85 = assign7850_e6840;
        var_guard85_rv = 0.0;

        let (assign7860_e6858,) = {
    if ((var_guard36 != 0.0) && (var_guard85 != 0.0)) {
        let assign7860_e6847: f64 = (p.p585 * var_ile);
        let assign7860_e6848: f64 = (p.p584 + assign7860_e6847);
        let assign7860_e6851: f64 = (p.p586 * var_iwe);
        let assign7860_e6852: f64 = (assign7860_e6848 + assign7860_e6851);
        let assign7860_e6855: f64 = (p.p587 * var_iae);
        let assign7860_e6856: f64 = (assign7860_e6852 + assign7860_e6855);
        (assign7860_e6856,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign7860_e6858;
        var_ax_p_rv = 0.0;

        let assign7870_e6877: f64 = if (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]) { 1.0 } else { 0.0 };
        var_guard86 = assign7870_e6877;
        var_guard86_rv = 0.0;

        let (assign7880_e6897,) = {
    if ((var_guard36 != 0.0) && (var_guard86 != 0.0)) {
        let assign7880_e6885: f64 = (p.p589 * var_ile);
        let assign7880_e6886: f64 = (p.p588 + assign7880_e6885);
        let assign7880_e6889: f64 = (p.p590 * var_iwe);
        let assign7880_e6890: f64 = (assign7880_e6886 + assign7880_e6889);
        let assign7880_e6893: f64 = (p.p591 * var_iae);
        let assign7880_e6894: f64 = (assign7880_e6890 + assign7880_e6893);
        let assign7880_e6895: f64 = (var_ile * assign7880_e6894);
        (assign7880_e6895,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign7880_e6897;
        var_alp_p_rv = 0.0;

        let assign7890_e6916: f64 = if (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]) { 1.0 } else { 0.0 };
        var_guard87 = assign7890_e6916;
        var_guard87_rv = 0.0;

        let (assign7900_e6934,) = {
    if ((var_guard36 != 0.0) && (var_guard87 != 0.0)) {
        let assign7900_e6923: f64 = (p.p593 * var_ile);
        let assign7900_e6924: f64 = (p.p592 + assign7900_e6923);
        let assign7900_e6927: f64 = (p.p594 * var_iwe);
        let assign7900_e6928: f64 = (assign7900_e6924 + assign7900_e6927);
        let assign7900_e6931: f64 = (p.p595 * var_iae);
        let assign7900_e6932: f64 = (assign7900_e6928 + assign7900_e6931);
        (assign7900_e6932,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign7900_e6934;
        var_alp1_p_rv = 0.0;

        let assign7910_e6953: f64 = if (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        var_guard88 = assign7910_e6953;
        var_guard88_rv = 0.0;

        let (assign7920_e6971,) = {
    if ((var_guard36 != 0.0) && (var_guard88 != 0.0)) {
        let assign7920_e6960: f64 = (p.p597 * var_ile);
        let assign7920_e6961: f64 = (p.p596 + assign7920_e6960);
        let assign7920_e6964: f64 = (p.p598 * var_iwe);
        let assign7920_e6965: f64 = (assign7920_e6961 + assign7920_e6964);
        let assign7920_e6968: f64 = (p.p599 * var_iae);
        let assign7920_e6969: f64 = (assign7920_e6965 + assign7920_e6968);
        (assign7920_e6969,)
    } else {
        (var_alp2_p,)
    }
};
        var_alp2_p = assign7920_e6971;
        var_alp2_p_rv = 0.0;

        let assign7930_e6990: f64 = if (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]) { 1.0 } else { 0.0 };
        var_guard89 = assign7930_e6990;
        var_guard89_rv = 0.0;

        let (assign7940_e7008,) = {
    if ((var_guard36 != 0.0) && (var_guard89 != 0.0)) {
        let assign7940_e6997: f64 = (p.p601 * var_ile);
        let assign7940_e6998: f64 = (p.p600 + assign7940_e6997);
        let assign7940_e7001: f64 = (p.p602 * var_iwe);
        let assign7940_e7002: f64 = (assign7940_e6998 + assign7940_e7001);
        let assign7940_e7005: f64 = (p.p603 * var_iae);
        let assign7940_e7006: f64 = (assign7940_e7002 + assign7940_e7005);
        (assign7940_e7006,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign7940_e7008;
        var_a1_p_rv = 0.0;

        let assign7950_e7027: f64 = if (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]) { 1.0 } else { 0.0 };
        var_guard90 = assign7950_e7027;
        var_guard90_rv = 0.0;

        *var_a1_p_slot = var_a1_p;
        *var_a1_p_rv_slot = var_a1_p_rv;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp1_p_rv_slot = var_alp1_p_rv;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp2_p_rv_slot = var_alp2_p_rv;
        *var_alp_p_slot = var_alp_p;
        *var_alp_p_rv_slot = var_alp_p_rv;
        *var_ax_p_slot = var_ax_p;
        *var_ax_p_rv_slot = var_ax_p_rv;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfb_p_rv_slot = var_cfb_p_rv;
        *var_cfd_p_slot = var_cfd_p;
        *var_cfd_p_rv_slot = var_cfd_p_rv;
        *var_cs_p_slot = var_cs_p;
        *var_cs_p_rv_slot = var_cs_p_rv;
        *var_guard66_slot = var_guard66;
        *var_guard66_rv_slot = var_guard66_rv;
        *var_guard67_slot = var_guard67;
        *var_guard67_rv_slot = var_guard67_rv;
        *var_guard68_slot = var_guard68;
        *var_guard68_rv_slot = var_guard68_rv;
        *var_guard69_slot = var_guard69;
        *var_guard69_rv_slot = var_guard69_rv;
        *var_guard70_slot = var_guard70;
        *var_guard70_rv_slot = var_guard70_rv;
        *var_guard71_slot = var_guard71;
        *var_guard71_rv_slot = var_guard71_rv;
        *var_guard72_slot = var_guard72;
        *var_guard72_rv_slot = var_guard72_rv;
        *var_guard73_slot = var_guard73;
        *var_guard73_rv_slot = var_guard73_rv;
        *var_guard74_slot = var_guard74;
        *var_guard74_rv_slot = var_guard74_rv;
        *var_guard75_slot = var_guard75;
        *var_guard75_rv_slot = var_guard75_rv;
        *var_guard76_slot = var_guard76;
        *var_guard76_rv_slot = var_guard76_rv;
        *var_guard77_slot = var_guard77;
        *var_guard77_rv_slot = var_guard77_rv;
        *var_guard78_slot = var_guard78;
        *var_guard78_rv_slot = var_guard78_rv;
        *var_guard79_slot = var_guard79;
        *var_guard79_rv_slot = var_guard79_rv;
        *var_guard80_slot = var_guard80;
        *var_guard80_rv_slot = var_guard80_rv;
        *var_guard81_slot = var_guard81;
        *var_guard81_rv_slot = var_guard81_rv;
        *var_guard82_slot = var_guard82;
        *var_guard82_rv_slot = var_guard82_rv;
        *var_guard83_slot = var_guard83;
        *var_guard83_rv_slot = var_guard83_rv;
        *var_guard84_slot = var_guard84;
        *var_guard84_rv_slot = var_guard84_rv;
        *var_guard85_slot = var_guard85;
        *var_guard85_rv_slot = var_guard85_rv;
        *var_guard86_slot = var_guard86;
        *var_guard86_rv_slot = var_guard86_rv;
        *var_guard87_slot = var_guard87;
        *var_guard87_rv_slot = var_guard87_rv;
        *var_guard88_slot = var_guard88;
        *var_guard88_rv_slot = var_guard88_rv;
        *var_guard89_slot = var_guard89;
        *var_guard89_rv_slot = var_guard89_rv;
        *var_guard90_slot = var_guard90;
        *var_guard90_rv_slot = var_guard90_rv;
        *var_mue_p_slot = var_mue_p;
        *var_mue_p_rv_slot = var_mue_p_rv;
        *var_psce_p_slot = var_psce_p;
        *var_psce_p_rv_slot = var_psce_p_rv;
        *var_psceb_p_slot = var_psceb_p;
        *var_psceb_p_rv_slot = var_psceb_p_rv;
        *var_psced_p_slot = var_psced_p;
        *var_psced_p_rv_slot = var_psced_p_rv;
        *var_rs_p_slot = var_rs_p;
        *var_rs_p_rv_slot = var_rs_p_rv;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsb_p_rv_slot = var_rsb_p_rv;
        *var_rsg_p_slot = var_rsg_p;
        *var_rsg_p_rv_slot = var_rsg_p_rv;
        *var_stbet_p_slot = var_stbet_p;
        *var_stbet_p_rv_slot = var_stbet_p_rv;
        *var_strs_p_slot = var_strs_p;
        *var_strs_p_rv_slot = var_strs_p_rv;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stthesat_p_rv_slot = var_stthesat_p_rv;
        *var_thecs_p_slot = var_thecs_p;
        *var_thecs_p_rv_slot = var_thecs_p_rv;
        *var_themu_p_slot = var_themu_p;
        *var_themu_p_rv_slot = var_themu_p_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_thesatb_p_rv_slot = var_thesatb_p_rv;
        *var_thesatg_p_slot = var_thesatg_p;
        *var_thesatg_p_rv_slot = var_thesatg_p_rv;
        *var_xcor_p_slot = var_xcor_p;
        *var_xcor_p_rv_slot = var_xcor_p_rv;
    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_guard90: f64,
        var_iae: f64,
        var_iiae: f64,
        var_iiwe: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lecv: f64,
        var_a3_p_slot: &mut f64,
        var_a3_p_rv_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_a4_p_rv_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidl_p_rv_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_agidld_p_rv_slot: &mut f64,
        var_cox_p_slot: &mut f64,
        var_cox_p_rv_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_delvtac_p_rv_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_facneffac_p_rv_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard100_rv_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard101_rv_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard102_rv_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard103_rv_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard104_rv_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard105_rv_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard106_rv_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard107_rv_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard108_rv_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard109_rv_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard110_rv_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard111_rv_slot: &mut f64,
        var_guard112_slot: &mut f64,
        var_guard112_rv_slot: &mut f64,
        var_guard113_slot: &mut f64,
        var_guard113_rv_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_guard91_rv_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_guard92_rv_slot: &mut f64,
        var_guard93_slot: &mut f64,
        var_guard93_rv_slot: &mut f64,
        var_guard94_slot: &mut f64,
        var_guard94_rv_slot: &mut f64,
        var_guard95_slot: &mut f64,
        var_guard95_rv_slot: &mut f64,
        var_guard96_slot: &mut f64,
        var_guard96_rv_slot: &mut f64,
        var_guard97_slot: &mut f64,
        var_guard97_rv_slot: &mut f64,
        var_guard98_slot: &mut f64,
        var_guard98_rv_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_guard99_rv_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_iginv_p_rv_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igov_p_rv_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_igovd_p_rv_slot: &mut f64,
        var_plparam_i_slot: &mut f64,
        var_plparam_i_rv_slot: &mut f64,
        var_plwparam_i_slot: &mut f64,
        var_plwparam_i_rv_slot: &mut f64,
        var_poparam_i_slot: &mut f64,
        var_poparam_i_rv_slot: &mut f64,
        var_pwparam_i_slot: &mut f64,
        var_pwparam_i_rv_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_sta2_p_rv_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidl_p_rv_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stbgidld_p_rv_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_stig_p_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
    ) {
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_a3_p_rv: f64 = *var_a3_p_rv_slot;
        let mut var_a4_p: f64 = *var_a4_p_slot;
        let mut var_a4_p_rv: f64 = *var_a4_p_rv_slot;
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidl_p_rv: f64 = *var_agidl_p_rv_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_agidld_p_rv: f64 = *var_agidld_p_rv_slot;
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_cox_p_rv: f64 = *var_cox_p_rv_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_delvtac_p_rv: f64 = *var_delvtac_p_rv_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_facneffac_p_rv: f64 = *var_facneffac_p_rv_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard100_rv: f64 = *var_guard100_rv_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard101_rv: f64 = *var_guard101_rv_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard102_rv: f64 = *var_guard102_rv_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard103_rv: f64 = *var_guard103_rv_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard104_rv: f64 = *var_guard104_rv_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard105_rv: f64 = *var_guard105_rv_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard106_rv: f64 = *var_guard106_rv_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard107_rv: f64 = *var_guard107_rv_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard108_rv: f64 = *var_guard108_rv_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard109_rv: f64 = *var_guard109_rv_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard110_rv: f64 = *var_guard110_rv_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard111_rv: f64 = *var_guard111_rv_slot;
        let mut var_guard112: f64 = *var_guard112_slot;
        let mut var_guard112_rv: f64 = *var_guard112_rv_slot;
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_guard113_rv: f64 = *var_guard113_rv_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_guard91_rv: f64 = *var_guard91_rv_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard92_rv: f64 = *var_guard92_rv_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_guard93_rv: f64 = *var_guard93_rv_slot;
        let mut var_guard94: f64 = *var_guard94_slot;
        let mut var_guard94_rv: f64 = *var_guard94_rv_slot;
        let mut var_guard95: f64 = *var_guard95_slot;
        let mut var_guard95_rv: f64 = *var_guard95_rv_slot;
        let mut var_guard96: f64 = *var_guard96_slot;
        let mut var_guard96_rv: f64 = *var_guard96_rv_slot;
        let mut var_guard97: f64 = *var_guard97_slot;
        let mut var_guard97_rv: f64 = *var_guard97_rv_slot;
        let mut var_guard98: f64 = *var_guard98_slot;
        let mut var_guard98_rv: f64 = *var_guard98_rv_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_guard99_rv: f64 = *var_guard99_rv_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_iginv_p_rv: f64 = *var_iginv_p_rv_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igov_p_rv: f64 = *var_igov_p_rv_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_igovd_p_rv: f64 = *var_igovd_p_rv_slot;
        let mut var_plparam_i: f64 = *var_plparam_i_slot;
        let mut var_plparam_i_rv: f64 = *var_plparam_i_rv_slot;
        let mut var_plwparam_i: f64 = *var_plwparam_i_slot;
        let mut var_plwparam_i_rv: f64 = *var_plwparam_i_rv_slot;
        let mut var_poparam_i: f64 = *var_poparam_i_slot;
        let mut var_poparam_i_rv: f64 = *var_poparam_i_rv_slot;
        let mut var_pwparam_i: f64 = *var_pwparam_i_slot;
        let mut var_pwparam_i_rv: f64 = *var_pwparam_i_rv_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_sta2_p_rv: f64 = *var_sta2_p_rv_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidl_p_rv: f64 = *var_stbgidl_p_rv_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stbgidld_p_rv: f64 = *var_stbgidld_p_rv_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_stig_p_rv: f64 = *var_stig_p_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;

        let (assign7960_e7045,) = {
    if ((var_guard36 != 0.0) && (var_guard90 != 0.0)) {
        let assign7960_e7034: f64 = (p.p605 * var_ile);
        let assign7960_e7035: f64 = (p.p604 + assign7960_e7034);
        let assign7960_e7038: f64 = (p.p606 * var_iwe);
        let assign7960_e7039: f64 = (assign7960_e7035 + assign7960_e7038);
        let assign7960_e7042: f64 = (p.p607 * var_iae);
        let assign7960_e7043: f64 = (assign7960_e7039 + assign7960_e7042);
        (assign7960_e7043,)
    } else {
        (var_sta2_p,)
    }
};
        var_sta2_p = assign7960_e7045;
        var_sta2_p_rv = 0.0;

        let assign7970_e7064: f64 = if (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]) { 1.0 } else { 0.0 };
        var_guard91 = assign7970_e7064;
        var_guard91_rv = 0.0;

        let (assign7980_e7082,) = {
    if ((var_guard36 != 0.0) && (var_guard91 != 0.0)) {
        let assign7980_e7071: f64 = (p.p609 * var_ile);
        let assign7980_e7072: f64 = (p.p608 + assign7980_e7071);
        let assign7980_e7075: f64 = (p.p610 * var_iwe);
        let assign7980_e7076: f64 = (assign7980_e7072 + assign7980_e7075);
        let assign7980_e7079: f64 = (p.p611 * var_iae);
        let assign7980_e7080: f64 = (assign7980_e7076 + assign7980_e7079);
        (assign7980_e7080,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign7980_e7082;
        var_a3_p_rv = 0.0;

        let assign7990_e7101: f64 = if (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]) { 1.0 } else { 0.0 };
        var_guard92 = assign7990_e7101;
        var_guard92_rv = 0.0;

        let (assign8000_e7119,) = {
    if ((var_guard36 != 0.0) && (var_guard92 != 0.0)) {
        let assign8000_e7108: f64 = (p.p613 * var_ile);
        let assign8000_e7109: f64 = (p.p612 + assign8000_e7108);
        let assign8000_e7112: f64 = (p.p614 * var_iwe);
        let assign8000_e7113: f64 = (assign8000_e7109 + assign8000_e7112);
        let assign8000_e7116: f64 = (p.p615 * var_iae);
        let assign8000_e7117: f64 = (assign8000_e7113 + assign8000_e7116);
        (assign8000_e7117,)
    } else {
        (var_a4_p,)
    }
};
        var_a4_p = assign8000_e7119;
        var_a4_p_rv = 0.0;

        let assign8010_e7138: f64 = if (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]) { 1.0 } else { 0.0 };
        var_guard93 = assign8010_e7138;
        var_guard93_rv = 0.0;

        let (assign8020_e7158,) = {
    if ((var_guard36 != 0.0) && (var_guard93 != 0.0)) {
        let assign8020_e7146: f64 = (p.p617 * var_ile);
        let assign8020_e7147: f64 = (p.p616 + assign8020_e7146);
        let assign8020_e7150: f64 = (p.p618 * var_iwe);
        let assign8020_e7151: f64 = (assign8020_e7147 + assign8020_e7150);
        let assign8020_e7154: f64 = (p.p619 * var_iae);
        let assign8020_e7155: f64 = (assign8020_e7151 + assign8020_e7154);
        let assign8020_e7156: f64 = (var_iiae * assign8020_e7155);
        (assign8020_e7156,)
    } else {
        (var_iginv_p,)
    }
};
        var_iginv_p = assign8020_e7158;
        var_iginv_p_rv = 0.0;

        let assign8030_e7177: f64 = if (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]) { 1.0 } else { 0.0 };
        var_guard94 = assign8030_e7177;
        var_guard94_rv = 0.0;

        let (assign8040_e7197,) = {
    if ((var_guard36 != 0.0) && (var_guard94 != 0.0)) {
        let assign8040_e7185: f64 = (p.p621 * var_ile);
        let assign8040_e7186: f64 = (p.p620 + assign8040_e7185);
        let assign8040_e7189: f64 = (p.p622 * var_iwe);
        let assign8040_e7190: f64 = (assign8040_e7186 + assign8040_e7189);
        let assign8040_e7193: f64 = (p.p623 * var_iae);
        let assign8040_e7194: f64 = (assign8040_e7190 + assign8040_e7193);
        let assign8040_e7195: f64 = (var_iiwe * assign8040_e7194);
        (assign8040_e7195,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign8040_e7197;
        var_igov_p_rv = 0.0;

        let assign8050_e7216: f64 = if (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]) { 1.0 } else { 0.0 };
        var_guard95 = assign8050_e7216;
        var_guard95_rv = 0.0;

        let (assign8060_e7236,) = {
    if ((var_guard36 != 0.0) && (var_guard95 != 0.0)) {
        let assign8060_e7224: f64 = (p.p625 * var_ile);
        let assign8060_e7225: f64 = (p.p624 + assign8060_e7224);
        let assign8060_e7228: f64 = (p.p626 * var_iwe);
        let assign8060_e7229: f64 = (assign8060_e7225 + assign8060_e7228);
        let assign8060_e7232: f64 = (p.p627 * var_iae);
        let assign8060_e7233: f64 = (assign8060_e7229 + assign8060_e7232);
        let assign8060_e7234: f64 = (var_iiwe * assign8060_e7233);
        (assign8060_e7234,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign8060_e7236;
        var_igovd_p_rv = 0.0;

        let assign8070_e7255: f64 = if (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]) { 1.0 } else { 0.0 };
        var_guard96 = assign8070_e7255;
        var_guard96_rv = 0.0;

        let (assign8080_e7273,) = {
    if ((var_guard36 != 0.0) && (var_guard96 != 0.0)) {
        let assign8080_e7262: f64 = (p.p629 * var_ile);
        let assign8080_e7263: f64 = (p.p628 + assign8080_e7262);
        let assign8080_e7266: f64 = (p.p630 * var_iwe);
        let assign8080_e7267: f64 = (assign8080_e7263 + assign8080_e7266);
        let assign8080_e7270: f64 = (p.p631 * var_iae);
        let assign8080_e7271: f64 = (assign8080_e7267 + assign8080_e7270);
        (assign8080_e7271,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign8080_e7273;
        var_stig_p_rv = 0.0;

        let assign8090_e7292: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };
        var_guard97 = assign8090_e7292;
        var_guard97_rv = 0.0;

        let (assign8100_e7312,) = {
    if ((var_guard36 != 0.0) && (var_guard97 != 0.0)) {
        let assign8100_e7300: f64 = (p.p633 * var_ile);
        let assign8100_e7301: f64 = (p.p632 + assign8100_e7300);
        let assign8100_e7304: f64 = (p.p634 * var_iwe);
        let assign8100_e7305: f64 = (assign8100_e7301 + assign8100_e7304);
        let assign8100_e7308: f64 = (p.p635 * var_iae);
        let assign8100_e7309: f64 = (assign8100_e7305 + assign8100_e7308);
        let assign8100_e7310: f64 = (var_iiwe * assign8100_e7309);
        (assign8100_e7310,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign8100_e7312;
        var_agidl_p_rv = 0.0;

        let assign8110_e7331: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };
        var_guard98 = assign8110_e7331;
        var_guard98_rv = 0.0;

        let (assign8120_e7351,) = {
    if ((var_guard36 != 0.0) && (var_guard98 != 0.0)) {
        let assign8120_e7339: f64 = (p.p637 * var_ile);
        let assign8120_e7340: f64 = (p.p636 + assign8120_e7339);
        let assign8120_e7343: f64 = (p.p638 * var_iwe);
        let assign8120_e7344: f64 = (assign8120_e7340 + assign8120_e7343);
        let assign8120_e7347: f64 = (p.p639 * var_iae);
        let assign8120_e7348: f64 = (assign8120_e7344 + assign8120_e7347);
        let assign8120_e7349: f64 = (var_iiwe * assign8120_e7348);
        (assign8120_e7349,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign8120_e7351;
        var_agidld_p_rv = 0.0;

        let assign8130_e7370: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };
        var_guard99 = assign8130_e7370;
        var_guard99_rv = 0.0;

        let (assign8140_e7388,) = {
    if ((var_guard36 != 0.0) && (var_guard99 != 0.0)) {
        let assign8140_e7377: f64 = (p.p641 * var_ile);
        let assign8140_e7378: f64 = (p.p640 + assign8140_e7377);
        let assign8140_e7381: f64 = (p.p642 * var_iwe);
        let assign8140_e7382: f64 = (assign8140_e7378 + assign8140_e7381);
        let assign8140_e7385: f64 = (p.p643 * var_iae);
        let assign8140_e7386: f64 = (assign8140_e7382 + assign8140_e7385);
        (assign8140_e7386,)
    } else {
        (var_stbgidl_p,)
    }
};
        var_stbgidl_p = assign8140_e7388;
        var_stbgidl_p_rv = 0.0;

        let assign8150_e7407: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };
        var_guard100 = assign8150_e7407;
        var_guard100_rv = 0.0;

        let (assign8160_e7425,) = {
    if ((var_guard36 != 0.0) && (var_guard100 != 0.0)) {
        let assign8160_e7414: f64 = (p.p645 * var_ile);
        let assign8160_e7415: f64 = (p.p644 + assign8160_e7414);
        let assign8160_e7418: f64 = (p.p646 * var_iwe);
        let assign8160_e7419: f64 = (assign8160_e7415 + assign8160_e7418);
        let assign8160_e7422: f64 = (p.p647 * var_iae);
        let assign8160_e7423: f64 = (assign8160_e7419 + assign8160_e7422);
        (assign8160_e7423,)
    } else {
        (var_stbgidld_p,)
    }
};
        var_stbgidld_p = assign8160_e7425;
        var_stbgidld_p_rv = 0.0;

        let assign8170_e7444: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };
        var_guard101 = assign8170_e7444;
        var_guard101_rv = 0.0;

        let (assign8180_e7468,) = {
    if ((var_guard36 != 0.0) && (var_guard101 != 0.0)) {
        let assign8180_e7450: f64 = (var_iiwecv * var_lecv);
        let assign8180_e7452: f64 = (assign8180_e7450 / 1e-6);
        let assign8180_e7456: f64 = (p.p649 * var_ile);
        let assign8180_e7457: f64 = (p.p648 + assign8180_e7456);
        let assign8180_e7460: f64 = (p.p650 * var_iwe);
        let assign8180_e7461: f64 = (assign8180_e7457 + assign8180_e7460);
        let assign8180_e7464: f64 = (p.p651 * var_iae);
        let assign8180_e7465: f64 = (assign8180_e7461 + assign8180_e7464);
        let assign8180_e7466: f64 = (assign8180_e7452 * assign8180_e7465);
        (assign8180_e7466,)
    } else {
        (var_cox_p,)
    }
};
        var_cox_p = assign8180_e7468;
        var_cox_p_rv = 0.0;

        let assign8190_e7487: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };
        var_guard102 = assign8190_e7487;
        var_guard102_rv = 0.0;

        let (assign8200_e7505,) = {
    if ((var_guard36 != 0.0) && (var_guard102 != 0.0)) {
        let assign8200_e7494: f64 = (p.p653 * var_ile);
        let assign8200_e7495: f64 = (p.p652 + assign8200_e7494);
        let assign8200_e7498: f64 = (p.p654 * var_iwe);
        let assign8200_e7499: f64 = (assign8200_e7495 + assign8200_e7498);
        let assign8200_e7502: f64 = (p.p655 * var_iae);
        let assign8200_e7503: f64 = (assign8200_e7499 + assign8200_e7502);
        (assign8200_e7503,)
    } else {
        (var_delvtac_p,)
    }
};
        var_delvtac_p = assign8200_e7505;
        var_delvtac_p_rv = 0.0;

        let assign8210_e7524: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };
        var_guard103 = assign8210_e7524;
        var_guard103_rv = 0.0;

        let (assign8220_e7542,) = {
    if ((var_guard36 != 0.0) && (var_guard103 != 0.0)) {
        let assign8220_e7531: f64 = (p.p657 * var_ile);
        let assign8220_e7532: f64 = (p.p656 + assign8220_e7531);
        let assign8220_e7535: f64 = (p.p658 * var_iwe);
        let assign8220_e7536: f64 = (assign8220_e7532 + assign8220_e7535);
        let assign8220_e7539: f64 = (p.p659 * var_iae);
        let assign8220_e7540: f64 = (assign8220_e7536 + assign8220_e7539);
        (assign8220_e7540,)
    } else {
        (var_facneffac_p,)
    }
};
        var_facneffac_p = assign8220_e7542;
        var_facneffac_p_rv = 0.0;

        let assign8230_e7581: f64 = if (((((((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) || param_given[568]) || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        var_guard104 = assign8230_e7581;
        var_guard104_rv = 0.0;

        let (assign8240_e7587,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p568,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8240_e7587;
        var_poparam_i_rv = 0.0;

        let assign8250_e7589: f64 = if param_given[660] { 1.0 } else { 0.0 };
        let assign8250_e7591: f64 = if assign8250_e7589 == 1.0 { 1.0 } else { 0.0 };
        var_guard105 = assign8250_e7591;
        var_guard105_rv = 0.0;

        let (assign8260_e7599,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard105 != 0.0)) {
        (p.p660,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8260_e7599;
        var_poparam_i_rv = 0.0;

        let (assign8270_e7605,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p569,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8270_e7605;
        var_plparam_i_rv = 0.0;

        let assign8280_e7607: f64 = if param_given[661] { 1.0 } else { 0.0 };
        let assign8280_e7609: f64 = if assign8280_e7607 == 1.0 { 1.0 } else { 0.0 };
        var_guard106 = assign8280_e7609;
        var_guard106_rv = 0.0;

        let (assign8290_e7617,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard106 != 0.0)) {
        (p.p661,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8290_e7617;
        var_plparam_i_rv = 0.0;

        let (assign8300_e7623,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p570,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8300_e7623;
        var_pwparam_i_rv = 0.0;

        let assign8310_e7625: f64 = if param_given[662] { 1.0 } else { 0.0 };
        let assign8310_e7627: f64 = if assign8310_e7625 == 1.0 { 1.0 } else { 0.0 };
        var_guard107 = assign8310_e7627;
        var_guard107_rv = 0.0;

        let (assign8320_e7635,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard107 != 0.0)) {
        (p.p662,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8320_e7635;
        var_pwparam_i_rv = 0.0;

        let (assign8330_e7641,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p571,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8330_e7641;
        var_plwparam_i_rv = 0.0;

        let assign8340_e7643: f64 = if param_given[663] { 1.0 } else { 0.0 };
        let assign8340_e7645: f64 = if assign8340_e7643 == 1.0 { 1.0 } else { 0.0 };
        var_guard108 = assign8340_e7645;
        var_guard108_rv = 0.0;

        let (assign8350_e7653,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard108 != 0.0)) {
        (p.p663,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8350_e7653;
        var_plwparam_i_rv = 0.0;

        let (assign8360_e7673,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        let assign8360_e7661: f64 = (var_plparam_i * var_ile);
        let assign8360_e7662: f64 = (var_poparam_i + assign8360_e7661);
        let assign8360_e7665: f64 = (var_pwparam_i * var_iwe);
        let assign8360_e7666: f64 = (assign8360_e7662 + assign8360_e7665);
        let assign8360_e7669: f64 = (var_plwparam_i * var_iae);
        let assign8360_e7670: f64 = (assign8360_e7666 + assign8360_e7669);
        let assign8360_e7671: f64 = (var_ile * assign8360_e7670);
        (assign8360_e7671,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign8360_e7673;
        var_thesatac_p_rv = 0.0;

        let assign8370_e7712: f64 = if (((((((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) || param_given[584]) || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        var_guard109 = assign8370_e7712;
        var_guard109_rv = 0.0;

        let (assign8380_e7718,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p584,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8380_e7718;
        var_poparam_i_rv = 0.0;

        let assign8390_e7720: f64 = if param_given[664] { 1.0 } else { 0.0 };
        let assign8390_e7722: f64 = if assign8390_e7720 == 1.0 { 1.0 } else { 0.0 };
        var_guard110 = assign8390_e7722;
        var_guard110_rv = 0.0;

        let (assign8400_e7730,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard110 != 0.0)) {
        (p.p664,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8400_e7730;
        var_poparam_i_rv = 0.0;

        let (assign8410_e7736,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p585,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8410_e7736;
        var_plparam_i_rv = 0.0;

        let assign8420_e7738: f64 = if param_given[665] { 1.0 } else { 0.0 };
        let assign8420_e7740: f64 = if assign8420_e7738 == 1.0 { 1.0 } else { 0.0 };
        var_guard111 = assign8420_e7740;
        var_guard111_rv = 0.0;

        let (assign8430_e7748,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard111 != 0.0)) {
        (p.p665,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8430_e7748;
        var_plparam_i_rv = 0.0;

        let (assign8440_e7754,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p586,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8440_e7754;
        var_pwparam_i_rv = 0.0;

        let assign8450_e7756: f64 = if param_given[666] { 1.0 } else { 0.0 };
        let assign8450_e7758: f64 = if assign8450_e7756 == 1.0 { 1.0 } else { 0.0 };
        var_guard112 = assign8450_e7758;
        var_guard112_rv = 0.0;

        let (assign8460_e7766,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard112 != 0.0)) {
        (p.p666,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8460_e7766;
        var_pwparam_i_rv = 0.0;

        let (assign8470_e7772,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p587,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8470_e7772;
        var_plwparam_i_rv = 0.0;

        let assign8480_e7774: f64 = if param_given[667] { 1.0 } else { 0.0 };
        let assign8480_e7776: f64 = if assign8480_e7774 == 1.0 { 1.0 } else { 0.0 };
        var_guard113 = assign8480_e7776;
        var_guard113_rv = 0.0;

        let (assign8490_e7784,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard113 != 0.0)) {
        (p.p667,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8490_e7784;
        var_plwparam_i_rv = 0.0;

        *var_a3_p_slot = var_a3_p;
        *var_a3_p_rv_slot = var_a3_p_rv;
        *var_a4_p_slot = var_a4_p;
        *var_a4_p_rv_slot = var_a4_p_rv;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidl_p_rv_slot = var_agidl_p_rv;
        *var_agidld_p_slot = var_agidld_p;
        *var_agidld_p_rv_slot = var_agidld_p_rv;
        *var_cox_p_slot = var_cox_p;
        *var_cox_p_rv_slot = var_cox_p_rv;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_delvtac_p_rv_slot = var_delvtac_p_rv;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_facneffac_p_rv_slot = var_facneffac_p_rv;
        *var_guard100_slot = var_guard100;
        *var_guard100_rv_slot = var_guard100_rv;
        *var_guard101_slot = var_guard101;
        *var_guard101_rv_slot = var_guard101_rv;
        *var_guard102_slot = var_guard102;
        *var_guard102_rv_slot = var_guard102_rv;
        *var_guard103_slot = var_guard103;
        *var_guard103_rv_slot = var_guard103_rv;
        *var_guard104_slot = var_guard104;
        *var_guard104_rv_slot = var_guard104_rv;
        *var_guard105_slot = var_guard105;
        *var_guard105_rv_slot = var_guard105_rv;
        *var_guard106_slot = var_guard106;
        *var_guard106_rv_slot = var_guard106_rv;
        *var_guard107_slot = var_guard107;
        *var_guard107_rv_slot = var_guard107_rv;
        *var_guard108_slot = var_guard108;
        *var_guard108_rv_slot = var_guard108_rv;
        *var_guard109_slot = var_guard109;
        *var_guard109_rv_slot = var_guard109_rv;
        *var_guard110_slot = var_guard110;
        *var_guard110_rv_slot = var_guard110_rv;
        *var_guard111_slot = var_guard111;
        *var_guard111_rv_slot = var_guard111_rv;
        *var_guard112_slot = var_guard112;
        *var_guard112_rv_slot = var_guard112_rv;
        *var_guard113_slot = var_guard113;
        *var_guard113_rv_slot = var_guard113_rv;
        *var_guard91_slot = var_guard91;
        *var_guard91_rv_slot = var_guard91_rv;
        *var_guard92_slot = var_guard92;
        *var_guard92_rv_slot = var_guard92_rv;
        *var_guard93_slot = var_guard93;
        *var_guard93_rv_slot = var_guard93_rv;
        *var_guard94_slot = var_guard94;
        *var_guard94_rv_slot = var_guard94_rv;
        *var_guard95_slot = var_guard95;
        *var_guard95_rv_slot = var_guard95_rv;
        *var_guard96_slot = var_guard96;
        *var_guard96_rv_slot = var_guard96_rv;
        *var_guard97_slot = var_guard97;
        *var_guard97_rv_slot = var_guard97_rv;
        *var_guard98_slot = var_guard98;
        *var_guard98_rv_slot = var_guard98_rv;
        *var_guard99_slot = var_guard99;
        *var_guard99_rv_slot = var_guard99_rv;
        *var_iginv_p_slot = var_iginv_p;
        *var_iginv_p_rv_slot = var_iginv_p_rv;
        *var_igov_p_slot = var_igov_p;
        *var_igov_p_rv_slot = var_igov_p_rv;
        *var_igovd_p_slot = var_igovd_p;
        *var_igovd_p_rv_slot = var_igovd_p_rv;
        *var_plparam_i_slot = var_plparam_i;
        *var_plparam_i_rv_slot = var_plparam_i_rv;
        *var_plwparam_i_slot = var_plwparam_i;
        *var_plwparam_i_rv_slot = var_plwparam_i_rv;
        *var_poparam_i_slot = var_poparam_i;
        *var_poparam_i_rv_slot = var_poparam_i_rv;
        *var_pwparam_i_slot = var_pwparam_i;
        *var_pwparam_i_rv_slot = var_pwparam_i_rv;
        *var_sta2_p_slot = var_sta2_p;
        *var_sta2_p_rv_slot = var_sta2_p_rv;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidl_p_rv_slot = var_stbgidl_p_rv;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stbgidld_p_rv_slot = var_stbgidld_p_rv;
        *var_stig_p_slot = var_stig_p;
        *var_stig_p_rv_slot = var_stig_p_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
    }
}
