#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_6(
        var_eg_slot: &mut f64,
        var_eg_dn0_slot: &mut f64,
        var_eg_dn10_slot: &mut f64,
        var_eg_dn13_slot: &mut f64,
        var_eg_dn2_slot: &mut f64,
        var_eg_dn4_slot: &mut f64,
        var_eg_dn5_slot: &mut f64,
        var_eg_dn6_slot: &mut f64,
        var_eg_dn7_slot: &mut f64,
        var_eg_dn8_slot: &mut f64,
        var_eg_dn9_slot: &mut f64,
        var_eg_rv_slot: &mut f64,
        var_idsibpc_slot: &mut f64,
        var_idsibpc_dn0_slot: &mut f64,
        var_idsibpc_dn10_slot: &mut f64,
        var_idsibpc_dn13_slot: &mut f64,
        var_idsibpc_dn2_slot: &mut f64,
        var_idsibpc_dn4_slot: &mut f64,
        var_idsibpc_dn5_slot: &mut f64,
        var_idsibpc_dn6_slot: &mut f64,
        var_idsibpc_dn7_slot: &mut f64,
        var_idsibpc_dn8_slot: &mut f64,
        var_idsibpc_dn9_slot: &mut f64,
        var_idsibpc_rv_slot: &mut f64,
        var_nin_slot: &mut f64,
        var_nin_dn0_slot: &mut f64,
        var_nin_dn10_slot: &mut f64,
        var_nin_dn13_slot: &mut f64,
        var_nin_dn2_slot: &mut f64,
        var_nin_dn4_slot: &mut f64,
        var_nin_dn5_slot: &mut f64,
        var_nin_dn6_slot: &mut f64,
        var_nin_dn7_slot: &mut f64,
        var_nin_dn8_slot: &mut f64,
        var_nin_dn9_slot: &mut f64,
        var_nin_rv_slot: &mut f64,
        var_qbdld_slot: &mut f64,
        var_qbdld_dn0_slot: &mut f64,
        var_qbdld_dn10_slot: &mut f64,
        var_qbdld_dn13_slot: &mut f64,
        var_qbdld_dn2_slot: &mut f64,
        var_qbdld_dn4_slot: &mut f64,
        var_qbdld_dn5_slot: &mut f64,
        var_qbdld_dn6_slot: &mut f64,
        var_qbdld_dn7_slot: &mut f64,
        var_qbdld_dn8_slot: &mut f64,
        var_qbdld_dn9_slot: &mut f64,
        var_qbdld_rv_slot: &mut f64,
        var_qbdldext_slot: &mut f64,
        var_qbdldext_dn0_slot: &mut f64,
        var_qbdldext_dn10_slot: &mut f64,
        var_qbdldext_dn13_slot: &mut f64,
        var_qbdldext_dn2_slot: &mut f64,
        var_qbdldext_dn4_slot: &mut f64,
        var_qbdldext_dn5_slot: &mut f64,
        var_qbdldext_dn6_slot: &mut f64,
        var_qbdldext_dn7_slot: &mut f64,
        var_qbdldext_dn8_slot: &mut f64,
        var_qbdldext_dn9_slot: &mut f64,
        var_qbdldext_rv_slot: &mut f64,
        var_qbsld_slot: &mut f64,
        var_qbsld_dn0_slot: &mut f64,
        var_qbsld_dn10_slot: &mut f64,
        var_qbsld_dn13_slot: &mut f64,
        var_qbsld_dn2_slot: &mut f64,
        var_qbsld_dn4_slot: &mut f64,
        var_qbsld_dn5_slot: &mut f64,
        var_qbsld_dn6_slot: &mut f64,
        var_qbsld_dn7_slot: &mut f64,
        var_qbsld_dn8_slot: &mut f64,
        var_qbsld_dn9_slot: &mut f64,
        var_qbsld_rv_slot: &mut f64,
        var_qbsldext_slot: &mut f64,
        var_qbsldext_dn0_slot: &mut f64,
        var_qbsldext_dn10_slot: &mut f64,
        var_qbsldext_dn13_slot: &mut f64,
        var_qbsldext_dn2_slot: &mut f64,
        var_qbsldext_dn4_slot: &mut f64,
        var_qbsldext_dn5_slot: &mut f64,
        var_qbsldext_dn6_slot: &mut f64,
        var_qbsldext_dn7_slot: &mut f64,
        var_qbsldext_dn8_slot: &mut f64,
        var_qbsldext_dn9_slot: &mut f64,
        var_qbsldext_rv_slot: &mut f64,
        var_qbuld_slot: &mut f64,
        var_qbuld_dn0_slot: &mut f64,
        var_qbuld_dn10_slot: &mut f64,
        var_qbuld_dn13_slot: &mut f64,
        var_qbuld_dn2_slot: &mut f64,
        var_qbuld_dn4_slot: &mut f64,
        var_qbuld_dn5_slot: &mut f64,
        var_qbuld_dn6_slot: &mut f64,
        var_qbuld_dn7_slot: &mut f64,
        var_qbuld_dn8_slot: &mut f64,
        var_qbuld_dn9_slot: &mut f64,
        var_qbuld_rv_slot: &mut f64,
        var_qiuld_slot: &mut f64,
        var_qiuld_dn0_slot: &mut f64,
        var_qiuld_dn10_slot: &mut f64,
        var_qiuld_dn13_slot: &mut f64,
        var_qiuld_dn2_slot: &mut f64,
        var_qiuld_dn4_slot: &mut f64,
        var_qiuld_dn5_slot: &mut f64,
        var_qiuld_dn6_slot: &mut f64,
        var_qiuld_dn7_slot: &mut f64,
        var_qiuld_dn8_slot: &mut f64,
        var_qiuld_dn9_slot: &mut f64,
        var_qiuld_rv_slot: &mut f64,
        var_qodad_slot: &mut f64,
        var_qodad_dn0_slot: &mut f64,
        var_qodad_dn10_slot: &mut f64,
        var_qodad_dn13_slot: &mut f64,
        var_qodad_dn2_slot: &mut f64,
        var_qodad_dn4_slot: &mut f64,
        var_qodad_dn5_slot: &mut f64,
        var_qodad_dn6_slot: &mut f64,
        var_qodad_dn7_slot: &mut f64,
        var_qodad_dn8_slot: &mut f64,
        var_qodad_dn9_slot: &mut f64,
        var_qodad_rv_slot: &mut f64,
        var_qovd_slot: &mut f64,
        var_qovd_dn0_slot: &mut f64,
        var_qovd_dn10_slot: &mut f64,
        var_qovd_dn13_slot: &mut f64,
        var_qovd_dn2_slot: &mut f64,
        var_qovd_dn4_slot: &mut f64,
        var_qovd_dn5_slot: &mut f64,
        var_qovd_dn6_slot: &mut f64,
        var_qovd_dn7_slot: &mut f64,
        var_qovd_dn8_slot: &mut f64,
        var_qovd_dn9_slot: &mut f64,
        var_qovd_rv_slot: &mut f64,
        var_qovdext_slot: &mut f64,
        var_qovdext_dn0_slot: &mut f64,
        var_qovdext_dn10_slot: &mut f64,
        var_qovdext_dn13_slot: &mut f64,
        var_qovdext_dn2_slot: &mut f64,
        var_qovdext_dn4_slot: &mut f64,
        var_qovdext_dn5_slot: &mut f64,
        var_qovdext_dn6_slot: &mut f64,
        var_qovdext_dn7_slot: &mut f64,
        var_qovdext_dn8_slot: &mut f64,
        var_qovdext_dn9_slot: &mut f64,
        var_qovdext_rv_slot: &mut f64,
        var_qovs_slot: &mut f64,
        var_qovs_dn0_slot: &mut f64,
        var_qovs_dn10_slot: &mut f64,
        var_qovs_dn13_slot: &mut f64,
        var_qovs_dn2_slot: &mut f64,
        var_qovs_dn4_slot: &mut f64,
        var_qovs_dn5_slot: &mut f64,
        var_qovs_dn6_slot: &mut f64,
        var_qovs_dn7_slot: &mut f64,
        var_qovs_dn8_slot: &mut f64,
        var_qovs_dn9_slot: &mut f64,
        var_qovs_rv_slot: &mut f64,
        var_qovsext_slot: &mut f64,
        var_qovsext_dn0_slot: &mut f64,
        var_qovsext_dn10_slot: &mut f64,
        var_qovsext_dn13_slot: &mut f64,
        var_qovsext_dn2_slot: &mut f64,
        var_qovsext_dn4_slot: &mut f64,
        var_qovsext_dn5_slot: &mut f64,
        var_qovsext_dn6_slot: &mut f64,
        var_qovsext_dn7_slot: &mut f64,
        var_qovsext_dn8_slot: &mut f64,
        var_qovsext_dn9_slot: &mut f64,
        var_qovsext_rv_slot: &mut f64,
        var_qsuld_slot: &mut f64,
        var_qsuld_dn0_slot: &mut f64,
        var_qsuld_dn10_slot: &mut f64,
        var_qsuld_dn13_slot: &mut f64,
        var_qsuld_dn2_slot: &mut f64,
        var_qsuld_dn4_slot: &mut f64,
        var_qsuld_dn5_slot: &mut f64,
        var_qsuld_dn6_slot: &mut f64,
        var_qsuld_dn7_slot: &mut f64,
        var_qsuld_dn8_slot: &mut f64,
        var_qsuld_dn9_slot: &mut f64,
        var_qsuld_rv_slot: &mut f64,
        var_ra_slot: &mut f64,
        var_ra_dn0_slot: &mut f64,
        var_ra_dn10_slot: &mut f64,
        var_ra_dn13_slot: &mut f64,
        var_ra_dn2_slot: &mut f64,
        var_ra_dn4_slot: &mut f64,
        var_ra_dn5_slot: &mut f64,
        var_ra_dn6_slot: &mut f64,
        var_ra_dn7_slot: &mut f64,
        var_ra_dn8_slot: &mut f64,
        var_ra_dn9_slot: &mut f64,
        var_ra_rv_slot: &mut f64,
        var_rdrift_slot: &mut f64,
        var_rdrift_dn0_slot: &mut f64,
        var_rdrift_dn10_slot: &mut f64,
        var_rdrift_dn13_slot: &mut f64,
        var_rdrift_dn2_slot: &mut f64,
        var_rdrift_dn4_slot: &mut f64,
        var_rdrift_dn5_slot: &mut f64,
        var_rdrift_dn6_slot: &mut f64,
        var_rdrift_dn7_slot: &mut f64,
        var_rdrift_dn8_slot: &mut f64,
        var_rdrift_dn9_slot: &mut f64,
        var_rdrift_rv_slot: &mut f64,
        var_rsdrift_slot: &mut f64,
        var_rsdrift_dn0_slot: &mut f64,
        var_rsdrift_dn10_slot: &mut f64,
        var_rsdrift_dn13_slot: &mut f64,
        var_rsdrift_dn2_slot: &mut f64,
        var_rsdrift_dn4_slot: &mut f64,
        var_rsdrift_dn5_slot: &mut f64,
        var_rsdrift_dn6_slot: &mut f64,
        var_rsdrift_dn7_slot: &mut f64,
        var_rsdrift_dn8_slot: &mut f64,
        var_rsdrift_dn9_slot: &mut f64,
        var_rsdrift_rv_slot: &mut f64,
        var_t11_slot: &mut f64,
        var_t11_dn0_slot: &mut f64,
        var_t11_dn10_slot: &mut f64,
        var_t11_dn13_slot: &mut f64,
        var_t11_dn2_slot: &mut f64,
        var_t11_dn4_slot: &mut f64,
        var_t11_dn5_slot: &mut f64,
        var_t11_dn6_slot: &mut f64,
        var_t11_dn7_slot: &mut f64,
        var_t11_dn8_slot: &mut f64,
        var_t11_dn9_slot: &mut f64,
        var_t11_rv_slot: &mut f64,
        var_t12_slot: &mut f64,
        var_t12_dn0_slot: &mut f64,
        var_t12_dn10_slot: &mut f64,
        var_t12_dn13_slot: &mut f64,
        var_t12_dn2_slot: &mut f64,
        var_t12_dn4_slot: &mut f64,
        var_t12_dn5_slot: &mut f64,
        var_t12_dn6_slot: &mut f64,
        var_t12_dn7_slot: &mut f64,
        var_t12_dn8_slot: &mut f64,
        var_t12_dn9_slot: &mut f64,
        var_t12_rv_slot: &mut f64,
        var_tdiff_slot: &mut f64,
        var_tdiff0_slot: &mut f64,
        var_tdiff0_2_slot: &mut f64,
        var_tdiff0_2_dn0_slot: &mut f64,
        var_tdiff0_2_dn10_slot: &mut f64,
        var_tdiff0_2_dn13_slot: &mut f64,
        var_tdiff0_2_dn2_slot: &mut f64,
        var_tdiff0_2_dn4_slot: &mut f64,
        var_tdiff0_2_dn5_slot: &mut f64,
        var_tdiff0_2_dn6_slot: &mut f64,
        var_tdiff0_2_dn7_slot: &mut f64,
        var_tdiff0_2_dn8_slot: &mut f64,
        var_tdiff0_2_dn9_slot: &mut f64,
        var_tdiff0_2_rv_slot: &mut f64,
        var_tdiff0_dn0_slot: &mut f64,
        var_tdiff0_dn10_slot: &mut f64,
        var_tdiff0_dn13_slot: &mut f64,
        var_tdiff0_dn2_slot: &mut f64,
        var_tdiff0_dn4_slot: &mut f64,
        var_tdiff0_dn5_slot: &mut f64,
        var_tdiff0_dn6_slot: &mut f64,
        var_tdiff0_dn7_slot: &mut f64,
        var_tdiff0_dn8_slot: &mut f64,
        var_tdiff0_dn9_slot: &mut f64,
        var_tdiff0_rv_slot: &mut f64,
        var_tdiff_2_slot: &mut f64,
        var_tdiff_2_dn0_slot: &mut f64,
        var_tdiff_2_dn10_slot: &mut f64,
        var_tdiff_2_dn13_slot: &mut f64,
        var_tdiff_2_dn2_slot: &mut f64,
        var_tdiff_2_dn4_slot: &mut f64,
        var_tdiff_2_dn5_slot: &mut f64,
        var_tdiff_2_dn6_slot: &mut f64,
        var_tdiff_2_dn7_slot: &mut f64,
        var_tdiff_2_dn8_slot: &mut f64,
        var_tdiff_2_dn9_slot: &mut f64,
        var_tdiff_2_rv_slot: &mut f64,
        var_tdiff_dn0_slot: &mut f64,
        var_tdiff_dn10_slot: &mut f64,
        var_tdiff_dn13_slot: &mut f64,
        var_tdiff_dn2_slot: &mut f64,
        var_tdiff_dn4_slot: &mut f64,
        var_tdiff_dn5_slot: &mut f64,
        var_tdiff_dn6_slot: &mut f64,
        var_tdiff_dn7_slot: &mut f64,
        var_tdiff_dn8_slot: &mut f64,
        var_tdiff_dn9_slot: &mut f64,
        var_tdiff_rv_slot: &mut f64,
        var_ttemp_slot: &mut f64,
        var_ttemp0_slot: &mut f64,
        var_ttemp0_dn0_slot: &mut f64,
        var_ttemp0_dn10_slot: &mut f64,
        var_ttemp0_dn13_slot: &mut f64,
        var_ttemp0_dn2_slot: &mut f64,
        var_ttemp0_dn4_slot: &mut f64,
        var_ttemp0_dn5_slot: &mut f64,
        var_ttemp0_dn6_slot: &mut f64,
        var_ttemp0_dn7_slot: &mut f64,
        var_ttemp0_dn8_slot: &mut f64,
        var_ttemp0_dn9_slot: &mut f64,
        var_ttemp0_rv_slot: &mut f64,
        var_ttemp_dn0_slot: &mut f64,
        var_ttemp_dn10_slot: &mut f64,
        var_ttemp_dn13_slot: &mut f64,
        var_ttemp_dn2_slot: &mut f64,
        var_ttemp_dn4_slot: &mut f64,
        var_ttemp_dn5_slot: &mut f64,
        var_ttemp_dn6_slot: &mut f64,
        var_ttemp_dn7_slot: &mut f64,
        var_ttemp_dn8_slot: &mut f64,
        var_ttemp_dn9_slot: &mut f64,
        var_ttemp_rv_slot: &mut f64,
        var_vbsegmt_slot: &mut f64,
        var_vbsegmt_dn2_slot: &mut f64,
        var_vbsegmt_dn8_slot: &mut f64,
        var_vbsegmt_rv_slot: &mut f64,
        var_vbserev_slot: &mut f64,
        var_vbserev_dn0_slot: &mut f64,
        var_vbserev_dn2_slot: &mut f64,
        var_vbserev_dn8_slot: &mut f64,
        var_vbserev_rv_slot: &mut f64,
        var_vbserevz_slot: &mut f64,
        var_vbserevz_dn0_slot: &mut f64,
        var_vbserevz_dn10_slot: &mut f64,
        var_vbserevz_dn13_slot: &mut f64,
        var_vbserevz_dn2_slot: &mut f64,
        var_vbserevz_dn4_slot: &mut f64,
        var_vbserevz_dn5_slot: &mut f64,
        var_vbserevz_dn6_slot: &mut f64,
        var_vbserevz_dn7_slot: &mut f64,
        var_vbserevz_dn8_slot: &mut f64,
        var_vbserevz_dn9_slot: &mut f64,
        var_vbserevz_rv_slot: &mut f64,
        var_vbsz2_slot: &mut f64,
        var_vbsz2_dn0_slot: &mut f64,
        var_vbsz2_dn10_slot: &mut f64,
        var_vbsz2_dn13_slot: &mut f64,
        var_vbsz2_dn2_slot: &mut f64,
        var_vbsz2_dn4_slot: &mut f64,
        var_vbsz2_dn5_slot: &mut f64,
        var_vbsz2_dn6_slot: &mut f64,
        var_vbsz2_dn7_slot: &mut f64,
        var_vbsz2_dn8_slot: &mut f64,
        var_vbsz2_dn9_slot: &mut f64,
        var_vbsz2_rv_slot: &mut f64,
        var_vdse_eff_slot: &mut f64,
        var_vdse_eff_dn0_slot: &mut f64,
        var_vdse_eff_dn2_slot: &mut f64,
        var_vdse_eff_rv_slot: &mut f64,
        var_vdseff_slot: &mut f64,
        var_vdseff_dn0_slot: &mut f64,
        var_vdseff_dn10_slot: &mut f64,
        var_vdseff_dn13_slot: &mut f64,
        var_vdseff_dn2_slot: &mut f64,
        var_vdseff_dn4_slot: &mut f64,
        var_vdseff_dn5_slot: &mut f64,
        var_vdseff_dn6_slot: &mut f64,
        var_vdseff_dn7_slot: &mut f64,
        var_vdseff_dn8_slot: &mut f64,
        var_vdseff_dn9_slot: &mut f64,
        var_vdseff_rv_slot: &mut f64,
        var_vdsegmt_slot: &mut f64,
        var_vdsegmt_dn0_slot: &mut f64,
        var_vdsegmt_dn2_slot: &mut f64,
        var_vdsegmt_rv_slot: &mut f64,
        var_vdsemodenml_slot: &mut f64,
        var_vdsemodenml_rv_slot: &mut f64,
        var_vdsemodervs_slot: &mut f64,
        var_vdsemodervs_rv_slot: &mut f64,
        var_vdserev_slot: &mut f64,
        var_vdserev_dn0_slot: &mut f64,
        var_vdserev_dn2_slot: &mut f64,
        var_vdserev_rv_slot: &mut f64,
        var_vdserevz_slot: &mut f64,
        var_vdserevz_dn0_slot: &mut f64,
        var_vdserevz_dn10_slot: &mut f64,
        var_vdserevz_dn13_slot: &mut f64,
        var_vdserevz_dn2_slot: &mut f64,
        var_vdserevz_dn4_slot: &mut f64,
        var_vdserevz_dn5_slot: &mut f64,
        var_vdserevz_dn6_slot: &mut f64,
        var_vdserevz_dn7_slot: &mut f64,
        var_vdserevz_dn8_slot: &mut f64,
        var_vdserevz_dn9_slot: &mut f64,
        var_vdserevz_rv_slot: &mut f64,
        var_vdsorg_slot: &mut f64,
        var_vdsorg_dn0_slot: &mut f64,
        var_vdsorg_dn10_slot: &mut f64,
        var_vdsorg_dn13_slot: &mut f64,
        var_vdsorg_dn2_slot: &mut f64,
        var_vdsorg_dn4_slot: &mut f64,
        var_vdsorg_dn5_slot: &mut f64,
        var_vdsorg_dn6_slot: &mut f64,
        var_vdsorg_dn7_slot: &mut f64,
        var_vdsorg_dn8_slot: &mut f64,
        var_vdsorg_dn9_slot: &mut f64,
        var_vdsorg_rv_slot: &mut f64,
        var_vgbgmt_slot: &mut f64,
        var_vgbgmt_dn2_slot: &mut f64,
        var_vgbgmt_dn6_slot: &mut f64,
        var_vgbgmt_dn7_slot: &mut f64,
        var_vgbgmt_dn8_slot: &mut f64,
        var_vgbgmt_rv_slot: &mut f64,
        var_vgpld_slot: &mut f64,
        var_vgpld_dn2_slot: &mut f64,
        var_vgpld_dn6_slot: &mut f64,
        var_vgpld_dn7_slot: &mut f64,
        var_vgpld_dn8_slot: &mut f64,
        var_vgpld_rv_slot: &mut f64,
        var_vgsegmt_slot: &mut f64,
        var_vgsegmt_dn2_slot: &mut f64,
        var_vgsegmt_dn6_slot: &mut f64,
        var_vgsegmt_rv_slot: &mut f64,
        var_vgserev_slot: &mut f64,
        var_vgserev_dn0_slot: &mut f64,
        var_vgserev_dn2_slot: &mut f64,
        var_vgserev_dn6_slot: &mut f64,
        var_vgserev_rv_slot: &mut f64,
        var_vgserevz_slot: &mut f64,
        var_vgserevz_dn0_slot: &mut f64,
        var_vgserevz_dn10_slot: &mut f64,
        var_vgserevz_dn13_slot: &mut f64,
        var_vgserevz_dn2_slot: &mut f64,
        var_vgserevz_dn4_slot: &mut f64,
        var_vgserevz_dn5_slot: &mut f64,
        var_vgserevz_dn6_slot: &mut f64,
        var_vgserevz_dn7_slot: &mut f64,
        var_vgserevz_dn8_slot: &mut f64,
        var_vgserevz_dn9_slot: &mut f64,
        var_vgserevz_rv_slot: &mut f64,
        var_vsubsrev_slot: &mut f64,
        var_vsubsrev_dn0_slot: &mut f64,
        var_vsubsrev_dn2_slot: &mut f64,
        var_vsubsrev_rv_slot: &mut f64,
        var_vxbgmt_slot: &mut f64,
        var_vxbgmt_dn0_slot: &mut f64,
        var_vxbgmt_dn10_slot: &mut f64,
        var_vxbgmt_dn13_slot: &mut f64,
        var_vxbgmt_dn2_slot: &mut f64,
        var_vxbgmt_dn4_slot: &mut f64,
        var_vxbgmt_dn5_slot: &mut f64,
        var_vxbgmt_dn6_slot: &mut f64,
        var_vxbgmt_dn7_slot: &mut f64,
        var_vxbgmt_dn8_slot: &mut f64,
        var_vxbgmt_dn9_slot: &mut f64,
        var_vxbgmt_rv_slot: &mut f64,
        var_vxbgmtcl_slot: &mut f64,
        var_vxbgmtcl_dn0_slot: &mut f64,
        var_vxbgmtcl_dn10_slot: &mut f64,
        var_vxbgmtcl_dn13_slot: &mut f64,
        var_vxbgmtcl_dn2_slot: &mut f64,
        var_vxbgmtcl_dn4_slot: &mut f64,
        var_vxbgmtcl_dn5_slot: &mut f64,
        var_vxbgmtcl_dn6_slot: &mut f64,
        var_vxbgmtcl_dn7_slot: &mut f64,
        var_vxbgmtcl_dn8_slot: &mut f64,
        var_vxbgmtcl_dn9_slot: &mut f64,
        var_vxbgmtcl_rv_slot: &mut f64,
    ) {
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_dn0: f64 = *var_eg_dn0_slot;
        let mut var_eg_dn10: f64 = *var_eg_dn10_slot;
        let mut var_eg_dn13: f64 = *var_eg_dn13_slot;
        let mut var_eg_dn2: f64 = *var_eg_dn2_slot;
        let mut var_eg_dn4: f64 = *var_eg_dn4_slot;
        let mut var_eg_dn5: f64 = *var_eg_dn5_slot;
        let mut var_eg_dn6: f64 = *var_eg_dn6_slot;
        let mut var_eg_dn7: f64 = *var_eg_dn7_slot;
        let mut var_eg_dn8: f64 = *var_eg_dn8_slot;
        let mut var_eg_dn9: f64 = *var_eg_dn9_slot;
        let mut var_eg_rv: f64 = *var_eg_rv_slot;
        let mut var_idsibpc: f64 = *var_idsibpc_slot;
        let mut var_idsibpc_dn0: f64 = *var_idsibpc_dn0_slot;
        let mut var_idsibpc_dn10: f64 = *var_idsibpc_dn10_slot;
        let mut var_idsibpc_dn13: f64 = *var_idsibpc_dn13_slot;
        let mut var_idsibpc_dn2: f64 = *var_idsibpc_dn2_slot;
        let mut var_idsibpc_dn4: f64 = *var_idsibpc_dn4_slot;
        let mut var_idsibpc_dn5: f64 = *var_idsibpc_dn5_slot;
        let mut var_idsibpc_dn6: f64 = *var_idsibpc_dn6_slot;
        let mut var_idsibpc_dn7: f64 = *var_idsibpc_dn7_slot;
        let mut var_idsibpc_dn8: f64 = *var_idsibpc_dn8_slot;
        let mut var_idsibpc_dn9: f64 = *var_idsibpc_dn9_slot;
        let mut var_idsibpc_rv: f64 = *var_idsibpc_rv_slot;
        let mut var_nin: f64 = *var_nin_slot;
        let mut var_nin_dn0: f64 = *var_nin_dn0_slot;
        let mut var_nin_dn10: f64 = *var_nin_dn10_slot;
        let mut var_nin_dn13: f64 = *var_nin_dn13_slot;
        let mut var_nin_dn2: f64 = *var_nin_dn2_slot;
        let mut var_nin_dn4: f64 = *var_nin_dn4_slot;
        let mut var_nin_dn5: f64 = *var_nin_dn5_slot;
        let mut var_nin_dn6: f64 = *var_nin_dn6_slot;
        let mut var_nin_dn7: f64 = *var_nin_dn7_slot;
        let mut var_nin_dn8: f64 = *var_nin_dn8_slot;
        let mut var_nin_dn9: f64 = *var_nin_dn9_slot;
        let mut var_nin_rv: f64 = *var_nin_rv_slot;
        let mut var_qbdld: f64 = *var_qbdld_slot;
        let mut var_qbdld_dn0: f64 = *var_qbdld_dn0_slot;
        let mut var_qbdld_dn10: f64 = *var_qbdld_dn10_slot;
        let mut var_qbdld_dn13: f64 = *var_qbdld_dn13_slot;
        let mut var_qbdld_dn2: f64 = *var_qbdld_dn2_slot;
        let mut var_qbdld_dn4: f64 = *var_qbdld_dn4_slot;
        let mut var_qbdld_dn5: f64 = *var_qbdld_dn5_slot;
        let mut var_qbdld_dn6: f64 = *var_qbdld_dn6_slot;
        let mut var_qbdld_dn7: f64 = *var_qbdld_dn7_slot;
        let mut var_qbdld_dn8: f64 = *var_qbdld_dn8_slot;
        let mut var_qbdld_dn9: f64 = *var_qbdld_dn9_slot;
        let mut var_qbdld_rv: f64 = *var_qbdld_rv_slot;
        let mut var_qbdldext: f64 = *var_qbdldext_slot;
        let mut var_qbdldext_dn0: f64 = *var_qbdldext_dn0_slot;
        let mut var_qbdldext_dn10: f64 = *var_qbdldext_dn10_slot;
        let mut var_qbdldext_dn13: f64 = *var_qbdldext_dn13_slot;
        let mut var_qbdldext_dn2: f64 = *var_qbdldext_dn2_slot;
        let mut var_qbdldext_dn4: f64 = *var_qbdldext_dn4_slot;
        let mut var_qbdldext_dn5: f64 = *var_qbdldext_dn5_slot;
        let mut var_qbdldext_dn6: f64 = *var_qbdldext_dn6_slot;
        let mut var_qbdldext_dn7: f64 = *var_qbdldext_dn7_slot;
        let mut var_qbdldext_dn8: f64 = *var_qbdldext_dn8_slot;
        let mut var_qbdldext_dn9: f64 = *var_qbdldext_dn9_slot;
        let mut var_qbdldext_rv: f64 = *var_qbdldext_rv_slot;
        let mut var_qbsld: f64 = *var_qbsld_slot;
        let mut var_qbsld_dn0: f64 = *var_qbsld_dn0_slot;
        let mut var_qbsld_dn10: f64 = *var_qbsld_dn10_slot;
        let mut var_qbsld_dn13: f64 = *var_qbsld_dn13_slot;
        let mut var_qbsld_dn2: f64 = *var_qbsld_dn2_slot;
        let mut var_qbsld_dn4: f64 = *var_qbsld_dn4_slot;
        let mut var_qbsld_dn5: f64 = *var_qbsld_dn5_slot;
        let mut var_qbsld_dn6: f64 = *var_qbsld_dn6_slot;
        let mut var_qbsld_dn7: f64 = *var_qbsld_dn7_slot;
        let mut var_qbsld_dn8: f64 = *var_qbsld_dn8_slot;
        let mut var_qbsld_dn9: f64 = *var_qbsld_dn9_slot;
        let mut var_qbsld_rv: f64 = *var_qbsld_rv_slot;
        let mut var_qbsldext: f64 = *var_qbsldext_slot;
        let mut var_qbsldext_dn0: f64 = *var_qbsldext_dn0_slot;
        let mut var_qbsldext_dn10: f64 = *var_qbsldext_dn10_slot;
        let mut var_qbsldext_dn13: f64 = *var_qbsldext_dn13_slot;
        let mut var_qbsldext_dn2: f64 = *var_qbsldext_dn2_slot;
        let mut var_qbsldext_dn4: f64 = *var_qbsldext_dn4_slot;
        let mut var_qbsldext_dn5: f64 = *var_qbsldext_dn5_slot;
        let mut var_qbsldext_dn6: f64 = *var_qbsldext_dn6_slot;
        let mut var_qbsldext_dn7: f64 = *var_qbsldext_dn7_slot;
        let mut var_qbsldext_dn8: f64 = *var_qbsldext_dn8_slot;
        let mut var_qbsldext_dn9: f64 = *var_qbsldext_dn9_slot;
        let mut var_qbsldext_rv: f64 = *var_qbsldext_rv_slot;
        let mut var_qbuld: f64 = *var_qbuld_slot;
        let mut var_qbuld_dn0: f64 = *var_qbuld_dn0_slot;
        let mut var_qbuld_dn10: f64 = *var_qbuld_dn10_slot;
        let mut var_qbuld_dn13: f64 = *var_qbuld_dn13_slot;
        let mut var_qbuld_dn2: f64 = *var_qbuld_dn2_slot;
        let mut var_qbuld_dn4: f64 = *var_qbuld_dn4_slot;
        let mut var_qbuld_dn5: f64 = *var_qbuld_dn5_slot;
        let mut var_qbuld_dn6: f64 = *var_qbuld_dn6_slot;
        let mut var_qbuld_dn7: f64 = *var_qbuld_dn7_slot;
        let mut var_qbuld_dn8: f64 = *var_qbuld_dn8_slot;
        let mut var_qbuld_dn9: f64 = *var_qbuld_dn9_slot;
        let mut var_qbuld_rv: f64 = *var_qbuld_rv_slot;
        let mut var_qiuld: f64 = *var_qiuld_slot;
        let mut var_qiuld_dn0: f64 = *var_qiuld_dn0_slot;
        let mut var_qiuld_dn10: f64 = *var_qiuld_dn10_slot;
        let mut var_qiuld_dn13: f64 = *var_qiuld_dn13_slot;
        let mut var_qiuld_dn2: f64 = *var_qiuld_dn2_slot;
        let mut var_qiuld_dn4: f64 = *var_qiuld_dn4_slot;
        let mut var_qiuld_dn5: f64 = *var_qiuld_dn5_slot;
        let mut var_qiuld_dn6: f64 = *var_qiuld_dn6_slot;
        let mut var_qiuld_dn7: f64 = *var_qiuld_dn7_slot;
        let mut var_qiuld_dn8: f64 = *var_qiuld_dn8_slot;
        let mut var_qiuld_dn9: f64 = *var_qiuld_dn9_slot;
        let mut var_qiuld_rv: f64 = *var_qiuld_rv_slot;
        let mut var_qodad: f64 = *var_qodad_slot;
        let mut var_qodad_dn0: f64 = *var_qodad_dn0_slot;
        let mut var_qodad_dn10: f64 = *var_qodad_dn10_slot;
        let mut var_qodad_dn13: f64 = *var_qodad_dn13_slot;
        let mut var_qodad_dn2: f64 = *var_qodad_dn2_slot;
        let mut var_qodad_dn4: f64 = *var_qodad_dn4_slot;
        let mut var_qodad_dn5: f64 = *var_qodad_dn5_slot;
        let mut var_qodad_dn6: f64 = *var_qodad_dn6_slot;
        let mut var_qodad_dn7: f64 = *var_qodad_dn7_slot;
        let mut var_qodad_dn8: f64 = *var_qodad_dn8_slot;
        let mut var_qodad_dn9: f64 = *var_qodad_dn9_slot;
        let mut var_qodad_rv: f64 = *var_qodad_rv_slot;
        let mut var_qovd: f64 = *var_qovd_slot;
        let mut var_qovd_dn0: f64 = *var_qovd_dn0_slot;
        let mut var_qovd_dn10: f64 = *var_qovd_dn10_slot;
        let mut var_qovd_dn13: f64 = *var_qovd_dn13_slot;
        let mut var_qovd_dn2: f64 = *var_qovd_dn2_slot;
        let mut var_qovd_dn4: f64 = *var_qovd_dn4_slot;
        let mut var_qovd_dn5: f64 = *var_qovd_dn5_slot;
        let mut var_qovd_dn6: f64 = *var_qovd_dn6_slot;
        let mut var_qovd_dn7: f64 = *var_qovd_dn7_slot;
        let mut var_qovd_dn8: f64 = *var_qovd_dn8_slot;
        let mut var_qovd_dn9: f64 = *var_qovd_dn9_slot;
        let mut var_qovd_rv: f64 = *var_qovd_rv_slot;
        let mut var_qovdext: f64 = *var_qovdext_slot;
        let mut var_qovdext_dn0: f64 = *var_qovdext_dn0_slot;
        let mut var_qovdext_dn10: f64 = *var_qovdext_dn10_slot;
        let mut var_qovdext_dn13: f64 = *var_qovdext_dn13_slot;
        let mut var_qovdext_dn2: f64 = *var_qovdext_dn2_slot;
        let mut var_qovdext_dn4: f64 = *var_qovdext_dn4_slot;
        let mut var_qovdext_dn5: f64 = *var_qovdext_dn5_slot;
        let mut var_qovdext_dn6: f64 = *var_qovdext_dn6_slot;
        let mut var_qovdext_dn7: f64 = *var_qovdext_dn7_slot;
        let mut var_qovdext_dn8: f64 = *var_qovdext_dn8_slot;
        let mut var_qovdext_dn9: f64 = *var_qovdext_dn9_slot;
        let mut var_qovdext_rv: f64 = *var_qovdext_rv_slot;
        let mut var_qovs: f64 = *var_qovs_slot;
        let mut var_qovs_dn0: f64 = *var_qovs_dn0_slot;
        let mut var_qovs_dn10: f64 = *var_qovs_dn10_slot;
        let mut var_qovs_dn13: f64 = *var_qovs_dn13_slot;
        let mut var_qovs_dn2: f64 = *var_qovs_dn2_slot;
        let mut var_qovs_dn4: f64 = *var_qovs_dn4_slot;
        let mut var_qovs_dn5: f64 = *var_qovs_dn5_slot;
        let mut var_qovs_dn6: f64 = *var_qovs_dn6_slot;
        let mut var_qovs_dn7: f64 = *var_qovs_dn7_slot;
        let mut var_qovs_dn8: f64 = *var_qovs_dn8_slot;
        let mut var_qovs_dn9: f64 = *var_qovs_dn9_slot;
        let mut var_qovs_rv: f64 = *var_qovs_rv_slot;
        let mut var_qovsext: f64 = *var_qovsext_slot;
        let mut var_qovsext_dn0: f64 = *var_qovsext_dn0_slot;
        let mut var_qovsext_dn10: f64 = *var_qovsext_dn10_slot;
        let mut var_qovsext_dn13: f64 = *var_qovsext_dn13_slot;
        let mut var_qovsext_dn2: f64 = *var_qovsext_dn2_slot;
        let mut var_qovsext_dn4: f64 = *var_qovsext_dn4_slot;
        let mut var_qovsext_dn5: f64 = *var_qovsext_dn5_slot;
        let mut var_qovsext_dn6: f64 = *var_qovsext_dn6_slot;
        let mut var_qovsext_dn7: f64 = *var_qovsext_dn7_slot;
        let mut var_qovsext_dn8: f64 = *var_qovsext_dn8_slot;
        let mut var_qovsext_dn9: f64 = *var_qovsext_dn9_slot;
        let mut var_qovsext_rv: f64 = *var_qovsext_rv_slot;
        let mut var_qsuld: f64 = *var_qsuld_slot;
        let mut var_qsuld_dn0: f64 = *var_qsuld_dn0_slot;
        let mut var_qsuld_dn10: f64 = *var_qsuld_dn10_slot;
        let mut var_qsuld_dn13: f64 = *var_qsuld_dn13_slot;
        let mut var_qsuld_dn2: f64 = *var_qsuld_dn2_slot;
        let mut var_qsuld_dn4: f64 = *var_qsuld_dn4_slot;
        let mut var_qsuld_dn5: f64 = *var_qsuld_dn5_slot;
        let mut var_qsuld_dn6: f64 = *var_qsuld_dn6_slot;
        let mut var_qsuld_dn7: f64 = *var_qsuld_dn7_slot;
        let mut var_qsuld_dn8: f64 = *var_qsuld_dn8_slot;
        let mut var_qsuld_dn9: f64 = *var_qsuld_dn9_slot;
        let mut var_qsuld_rv: f64 = *var_qsuld_rv_slot;
        let mut var_ra: f64 = *var_ra_slot;
        let mut var_ra_dn0: f64 = *var_ra_dn0_slot;
        let mut var_ra_dn10: f64 = *var_ra_dn10_slot;
        let mut var_ra_dn13: f64 = *var_ra_dn13_slot;
        let mut var_ra_dn2: f64 = *var_ra_dn2_slot;
        let mut var_ra_dn4: f64 = *var_ra_dn4_slot;
        let mut var_ra_dn5: f64 = *var_ra_dn5_slot;
        let mut var_ra_dn6: f64 = *var_ra_dn6_slot;
        let mut var_ra_dn7: f64 = *var_ra_dn7_slot;
        let mut var_ra_dn8: f64 = *var_ra_dn8_slot;
        let mut var_ra_dn9: f64 = *var_ra_dn9_slot;
        let mut var_ra_rv: f64 = *var_ra_rv_slot;
        let mut var_rdrift: f64 = *var_rdrift_slot;
        let mut var_rdrift_dn0: f64 = *var_rdrift_dn0_slot;
        let mut var_rdrift_dn10: f64 = *var_rdrift_dn10_slot;
        let mut var_rdrift_dn13: f64 = *var_rdrift_dn13_slot;
        let mut var_rdrift_dn2: f64 = *var_rdrift_dn2_slot;
        let mut var_rdrift_dn4: f64 = *var_rdrift_dn4_slot;
        let mut var_rdrift_dn5: f64 = *var_rdrift_dn5_slot;
        let mut var_rdrift_dn6: f64 = *var_rdrift_dn6_slot;
        let mut var_rdrift_dn7: f64 = *var_rdrift_dn7_slot;
        let mut var_rdrift_dn8: f64 = *var_rdrift_dn8_slot;
        let mut var_rdrift_dn9: f64 = *var_rdrift_dn9_slot;
        let mut var_rdrift_rv: f64 = *var_rdrift_rv_slot;
        let mut var_rsdrift: f64 = *var_rsdrift_slot;
        let mut var_rsdrift_dn0: f64 = *var_rsdrift_dn0_slot;
        let mut var_rsdrift_dn10: f64 = *var_rsdrift_dn10_slot;
        let mut var_rsdrift_dn13: f64 = *var_rsdrift_dn13_slot;
        let mut var_rsdrift_dn2: f64 = *var_rsdrift_dn2_slot;
        let mut var_rsdrift_dn4: f64 = *var_rsdrift_dn4_slot;
        let mut var_rsdrift_dn5: f64 = *var_rsdrift_dn5_slot;
        let mut var_rsdrift_dn6: f64 = *var_rsdrift_dn6_slot;
        let mut var_rsdrift_dn7: f64 = *var_rsdrift_dn7_slot;
        let mut var_rsdrift_dn8: f64 = *var_rsdrift_dn8_slot;
        let mut var_rsdrift_dn9: f64 = *var_rsdrift_dn9_slot;
        let mut var_rsdrift_rv: f64 = *var_rsdrift_rv_slot;
        let mut var_t11: f64 = *var_t11_slot;
        let mut var_t11_dn0: f64 = *var_t11_dn0_slot;
        let mut var_t11_dn10: f64 = *var_t11_dn10_slot;
        let mut var_t11_dn13: f64 = *var_t11_dn13_slot;
        let mut var_t11_dn2: f64 = *var_t11_dn2_slot;
        let mut var_t11_dn4: f64 = *var_t11_dn4_slot;
        let mut var_t11_dn5: f64 = *var_t11_dn5_slot;
        let mut var_t11_dn6: f64 = *var_t11_dn6_slot;
        let mut var_t11_dn7: f64 = *var_t11_dn7_slot;
        let mut var_t11_dn8: f64 = *var_t11_dn8_slot;
        let mut var_t11_dn9: f64 = *var_t11_dn9_slot;
        let mut var_t11_rv: f64 = *var_t11_rv_slot;
        let mut var_t12: f64 = *var_t12_slot;
        let mut var_t12_dn0: f64 = *var_t12_dn0_slot;
        let mut var_t12_dn10: f64 = *var_t12_dn10_slot;
        let mut var_t12_dn13: f64 = *var_t12_dn13_slot;
        let mut var_t12_dn2: f64 = *var_t12_dn2_slot;
        let mut var_t12_dn4: f64 = *var_t12_dn4_slot;
        let mut var_t12_dn5: f64 = *var_t12_dn5_slot;
        let mut var_t12_dn6: f64 = *var_t12_dn6_slot;
        let mut var_t12_dn7: f64 = *var_t12_dn7_slot;
        let mut var_t12_dn8: f64 = *var_t12_dn8_slot;
        let mut var_t12_dn9: f64 = *var_t12_dn9_slot;
        let mut var_t12_rv: f64 = *var_t12_rv_slot;
        let mut var_tdiff: f64 = *var_tdiff_slot;
        let mut var_tdiff0: f64 = *var_tdiff0_slot;
        let mut var_tdiff0_2: f64 = *var_tdiff0_2_slot;
        let mut var_tdiff0_2_dn0: f64 = *var_tdiff0_2_dn0_slot;
        let mut var_tdiff0_2_dn10: f64 = *var_tdiff0_2_dn10_slot;
        let mut var_tdiff0_2_dn13: f64 = *var_tdiff0_2_dn13_slot;
        let mut var_tdiff0_2_dn2: f64 = *var_tdiff0_2_dn2_slot;
        let mut var_tdiff0_2_dn4: f64 = *var_tdiff0_2_dn4_slot;
        let mut var_tdiff0_2_dn5: f64 = *var_tdiff0_2_dn5_slot;
        let mut var_tdiff0_2_dn6: f64 = *var_tdiff0_2_dn6_slot;
        let mut var_tdiff0_2_dn7: f64 = *var_tdiff0_2_dn7_slot;
        let mut var_tdiff0_2_dn8: f64 = *var_tdiff0_2_dn8_slot;
        let mut var_tdiff0_2_dn9: f64 = *var_tdiff0_2_dn9_slot;
        let mut var_tdiff0_2_rv: f64 = *var_tdiff0_2_rv_slot;
        let mut var_tdiff0_dn0: f64 = *var_tdiff0_dn0_slot;
        let mut var_tdiff0_dn10: f64 = *var_tdiff0_dn10_slot;
        let mut var_tdiff0_dn13: f64 = *var_tdiff0_dn13_slot;
        let mut var_tdiff0_dn2: f64 = *var_tdiff0_dn2_slot;
        let mut var_tdiff0_dn4: f64 = *var_tdiff0_dn4_slot;
        let mut var_tdiff0_dn5: f64 = *var_tdiff0_dn5_slot;
        let mut var_tdiff0_dn6: f64 = *var_tdiff0_dn6_slot;
        let mut var_tdiff0_dn7: f64 = *var_tdiff0_dn7_slot;
        let mut var_tdiff0_dn8: f64 = *var_tdiff0_dn8_slot;
        let mut var_tdiff0_dn9: f64 = *var_tdiff0_dn9_slot;
        let mut var_tdiff0_rv: f64 = *var_tdiff0_rv_slot;
        let mut var_tdiff_2: f64 = *var_tdiff_2_slot;
        let mut var_tdiff_2_dn0: f64 = *var_tdiff_2_dn0_slot;
        let mut var_tdiff_2_dn10: f64 = *var_tdiff_2_dn10_slot;
        let mut var_tdiff_2_dn13: f64 = *var_tdiff_2_dn13_slot;
        let mut var_tdiff_2_dn2: f64 = *var_tdiff_2_dn2_slot;
        let mut var_tdiff_2_dn4: f64 = *var_tdiff_2_dn4_slot;
        let mut var_tdiff_2_dn5: f64 = *var_tdiff_2_dn5_slot;
        let mut var_tdiff_2_dn6: f64 = *var_tdiff_2_dn6_slot;
        let mut var_tdiff_2_dn7: f64 = *var_tdiff_2_dn7_slot;
        let mut var_tdiff_2_dn8: f64 = *var_tdiff_2_dn8_slot;
        let mut var_tdiff_2_dn9: f64 = *var_tdiff_2_dn9_slot;
        let mut var_tdiff_2_rv: f64 = *var_tdiff_2_rv_slot;
        let mut var_tdiff_dn0: f64 = *var_tdiff_dn0_slot;
        let mut var_tdiff_dn10: f64 = *var_tdiff_dn10_slot;
        let mut var_tdiff_dn13: f64 = *var_tdiff_dn13_slot;
        let mut var_tdiff_dn2: f64 = *var_tdiff_dn2_slot;
        let mut var_tdiff_dn4: f64 = *var_tdiff_dn4_slot;
        let mut var_tdiff_dn5: f64 = *var_tdiff_dn5_slot;
        let mut var_tdiff_dn6: f64 = *var_tdiff_dn6_slot;
        let mut var_tdiff_dn7: f64 = *var_tdiff_dn7_slot;
        let mut var_tdiff_dn8: f64 = *var_tdiff_dn8_slot;
        let mut var_tdiff_dn9: f64 = *var_tdiff_dn9_slot;
        let mut var_tdiff_rv: f64 = *var_tdiff_rv_slot;
        let mut var_ttemp: f64 = *var_ttemp_slot;
        let mut var_ttemp0: f64 = *var_ttemp0_slot;
        let mut var_ttemp0_dn0: f64 = *var_ttemp0_dn0_slot;
        let mut var_ttemp0_dn10: f64 = *var_ttemp0_dn10_slot;
        let mut var_ttemp0_dn13: f64 = *var_ttemp0_dn13_slot;
        let mut var_ttemp0_dn2: f64 = *var_ttemp0_dn2_slot;
        let mut var_ttemp0_dn4: f64 = *var_ttemp0_dn4_slot;
        let mut var_ttemp0_dn5: f64 = *var_ttemp0_dn5_slot;
        let mut var_ttemp0_dn6: f64 = *var_ttemp0_dn6_slot;
        let mut var_ttemp0_dn7: f64 = *var_ttemp0_dn7_slot;
        let mut var_ttemp0_dn8: f64 = *var_ttemp0_dn8_slot;
        let mut var_ttemp0_dn9: f64 = *var_ttemp0_dn9_slot;
        let mut var_ttemp0_rv: f64 = *var_ttemp0_rv_slot;
        let mut var_ttemp_dn0: f64 = *var_ttemp_dn0_slot;
        let mut var_ttemp_dn10: f64 = *var_ttemp_dn10_slot;
        let mut var_ttemp_dn13: f64 = *var_ttemp_dn13_slot;
        let mut var_ttemp_dn2: f64 = *var_ttemp_dn2_slot;
        let mut var_ttemp_dn4: f64 = *var_ttemp_dn4_slot;
        let mut var_ttemp_dn5: f64 = *var_ttemp_dn5_slot;
        let mut var_ttemp_dn6: f64 = *var_ttemp_dn6_slot;
        let mut var_ttemp_dn7: f64 = *var_ttemp_dn7_slot;
        let mut var_ttemp_dn8: f64 = *var_ttemp_dn8_slot;
        let mut var_ttemp_dn9: f64 = *var_ttemp_dn9_slot;
        let mut var_ttemp_rv: f64 = *var_ttemp_rv_slot;
        let mut var_vbsegmt: f64 = *var_vbsegmt_slot;
        let mut var_vbsegmt_dn2: f64 = *var_vbsegmt_dn2_slot;
        let mut var_vbsegmt_dn8: f64 = *var_vbsegmt_dn8_slot;
        let mut var_vbsegmt_rv: f64 = *var_vbsegmt_rv_slot;
        let mut var_vbserev: f64 = *var_vbserev_slot;
        let mut var_vbserev_dn0: f64 = *var_vbserev_dn0_slot;
        let mut var_vbserev_dn2: f64 = *var_vbserev_dn2_slot;
        let mut var_vbserev_dn8: f64 = *var_vbserev_dn8_slot;
        let mut var_vbserev_rv: f64 = *var_vbserev_rv_slot;
        let mut var_vbserevz: f64 = *var_vbserevz_slot;
        let mut var_vbserevz_dn0: f64 = *var_vbserevz_dn0_slot;
        let mut var_vbserevz_dn10: f64 = *var_vbserevz_dn10_slot;
        let mut var_vbserevz_dn13: f64 = *var_vbserevz_dn13_slot;
        let mut var_vbserevz_dn2: f64 = *var_vbserevz_dn2_slot;
        let mut var_vbserevz_dn4: f64 = *var_vbserevz_dn4_slot;
        let mut var_vbserevz_dn5: f64 = *var_vbserevz_dn5_slot;
        let mut var_vbserevz_dn6: f64 = *var_vbserevz_dn6_slot;
        let mut var_vbserevz_dn7: f64 = *var_vbserevz_dn7_slot;
        let mut var_vbserevz_dn8: f64 = *var_vbserevz_dn8_slot;
        let mut var_vbserevz_dn9: f64 = *var_vbserevz_dn9_slot;
        let mut var_vbserevz_rv: f64 = *var_vbserevz_rv_slot;
        let mut var_vbsz2: f64 = *var_vbsz2_slot;
        let mut var_vbsz2_dn0: f64 = *var_vbsz2_dn0_slot;
        let mut var_vbsz2_dn10: f64 = *var_vbsz2_dn10_slot;
        let mut var_vbsz2_dn13: f64 = *var_vbsz2_dn13_slot;
        let mut var_vbsz2_dn2: f64 = *var_vbsz2_dn2_slot;
        let mut var_vbsz2_dn4: f64 = *var_vbsz2_dn4_slot;
        let mut var_vbsz2_dn5: f64 = *var_vbsz2_dn5_slot;
        let mut var_vbsz2_dn6: f64 = *var_vbsz2_dn6_slot;
        let mut var_vbsz2_dn7: f64 = *var_vbsz2_dn7_slot;
        let mut var_vbsz2_dn8: f64 = *var_vbsz2_dn8_slot;
        let mut var_vbsz2_dn9: f64 = *var_vbsz2_dn9_slot;
        let mut var_vbsz2_rv: f64 = *var_vbsz2_rv_slot;
        let mut var_vdse_eff: f64 = *var_vdse_eff_slot;
        let mut var_vdse_eff_dn0: f64 = *var_vdse_eff_dn0_slot;
        let mut var_vdse_eff_dn2: f64 = *var_vdse_eff_dn2_slot;
        let mut var_vdse_eff_rv: f64 = *var_vdse_eff_rv_slot;
        let mut var_vdseff: f64 = *var_vdseff_slot;
        let mut var_vdseff_dn0: f64 = *var_vdseff_dn0_slot;
        let mut var_vdseff_dn10: f64 = *var_vdseff_dn10_slot;
        let mut var_vdseff_dn13: f64 = *var_vdseff_dn13_slot;
        let mut var_vdseff_dn2: f64 = *var_vdseff_dn2_slot;
        let mut var_vdseff_dn4: f64 = *var_vdseff_dn4_slot;
        let mut var_vdseff_dn5: f64 = *var_vdseff_dn5_slot;
        let mut var_vdseff_dn6: f64 = *var_vdseff_dn6_slot;
        let mut var_vdseff_dn7: f64 = *var_vdseff_dn7_slot;
        let mut var_vdseff_dn8: f64 = *var_vdseff_dn8_slot;
        let mut var_vdseff_dn9: f64 = *var_vdseff_dn9_slot;
        let mut var_vdseff_rv: f64 = *var_vdseff_rv_slot;
        let mut var_vdsegmt: f64 = *var_vdsegmt_slot;
        let mut var_vdsegmt_dn0: f64 = *var_vdsegmt_dn0_slot;
        let mut var_vdsegmt_dn2: f64 = *var_vdsegmt_dn2_slot;
        let mut var_vdsegmt_rv: f64 = *var_vdsegmt_rv_slot;
        let mut var_vdsemodenml: f64 = *var_vdsemodenml_slot;
        let mut var_vdsemodenml_rv: f64 = *var_vdsemodenml_rv_slot;
        let mut var_vdsemodervs: f64 = *var_vdsemodervs_slot;
        let mut var_vdsemodervs_rv: f64 = *var_vdsemodervs_rv_slot;
        let mut var_vdserev: f64 = *var_vdserev_slot;
        let mut var_vdserev_dn0: f64 = *var_vdserev_dn0_slot;
        let mut var_vdserev_dn2: f64 = *var_vdserev_dn2_slot;
        let mut var_vdserev_rv: f64 = *var_vdserev_rv_slot;
        let mut var_vdserevz: f64 = *var_vdserevz_slot;
        let mut var_vdserevz_dn0: f64 = *var_vdserevz_dn0_slot;
        let mut var_vdserevz_dn10: f64 = *var_vdserevz_dn10_slot;
        let mut var_vdserevz_dn13: f64 = *var_vdserevz_dn13_slot;
        let mut var_vdserevz_dn2: f64 = *var_vdserevz_dn2_slot;
        let mut var_vdserevz_dn4: f64 = *var_vdserevz_dn4_slot;
        let mut var_vdserevz_dn5: f64 = *var_vdserevz_dn5_slot;
        let mut var_vdserevz_dn6: f64 = *var_vdserevz_dn6_slot;
        let mut var_vdserevz_dn7: f64 = *var_vdserevz_dn7_slot;
        let mut var_vdserevz_dn8: f64 = *var_vdserevz_dn8_slot;
        let mut var_vdserevz_dn9: f64 = *var_vdserevz_dn9_slot;
        let mut var_vdserevz_rv: f64 = *var_vdserevz_rv_slot;
        let mut var_vdsorg: f64 = *var_vdsorg_slot;
        let mut var_vdsorg_dn0: f64 = *var_vdsorg_dn0_slot;
        let mut var_vdsorg_dn10: f64 = *var_vdsorg_dn10_slot;
        let mut var_vdsorg_dn13: f64 = *var_vdsorg_dn13_slot;
        let mut var_vdsorg_dn2: f64 = *var_vdsorg_dn2_slot;
        let mut var_vdsorg_dn4: f64 = *var_vdsorg_dn4_slot;
        let mut var_vdsorg_dn5: f64 = *var_vdsorg_dn5_slot;
        let mut var_vdsorg_dn6: f64 = *var_vdsorg_dn6_slot;
        let mut var_vdsorg_dn7: f64 = *var_vdsorg_dn7_slot;
        let mut var_vdsorg_dn8: f64 = *var_vdsorg_dn8_slot;
        let mut var_vdsorg_dn9: f64 = *var_vdsorg_dn9_slot;
        let mut var_vdsorg_rv: f64 = *var_vdsorg_rv_slot;
        let mut var_vgbgmt: f64 = *var_vgbgmt_slot;
        let mut var_vgbgmt_dn2: f64 = *var_vgbgmt_dn2_slot;
        let mut var_vgbgmt_dn6: f64 = *var_vgbgmt_dn6_slot;
        let mut var_vgbgmt_dn7: f64 = *var_vgbgmt_dn7_slot;
        let mut var_vgbgmt_dn8: f64 = *var_vgbgmt_dn8_slot;
        let mut var_vgbgmt_rv: f64 = *var_vgbgmt_rv_slot;
        let mut var_vgpld: f64 = *var_vgpld_slot;
        let mut var_vgpld_dn2: f64 = *var_vgpld_dn2_slot;
        let mut var_vgpld_dn6: f64 = *var_vgpld_dn6_slot;
        let mut var_vgpld_dn7: f64 = *var_vgpld_dn7_slot;
        let mut var_vgpld_dn8: f64 = *var_vgpld_dn8_slot;
        let mut var_vgpld_rv: f64 = *var_vgpld_rv_slot;
        let mut var_vgsegmt: f64 = *var_vgsegmt_slot;
        let mut var_vgsegmt_dn2: f64 = *var_vgsegmt_dn2_slot;
        let mut var_vgsegmt_dn6: f64 = *var_vgsegmt_dn6_slot;
        let mut var_vgsegmt_rv: f64 = *var_vgsegmt_rv_slot;
        let mut var_vgserev: f64 = *var_vgserev_slot;
        let mut var_vgserev_dn0: f64 = *var_vgserev_dn0_slot;
        let mut var_vgserev_dn2: f64 = *var_vgserev_dn2_slot;
        let mut var_vgserev_dn6: f64 = *var_vgserev_dn6_slot;
        let mut var_vgserev_rv: f64 = *var_vgserev_rv_slot;
        let mut var_vgserevz: f64 = *var_vgserevz_slot;
        let mut var_vgserevz_dn0: f64 = *var_vgserevz_dn0_slot;
        let mut var_vgserevz_dn10: f64 = *var_vgserevz_dn10_slot;
        let mut var_vgserevz_dn13: f64 = *var_vgserevz_dn13_slot;
        let mut var_vgserevz_dn2: f64 = *var_vgserevz_dn2_slot;
        let mut var_vgserevz_dn4: f64 = *var_vgserevz_dn4_slot;
        let mut var_vgserevz_dn5: f64 = *var_vgserevz_dn5_slot;
        let mut var_vgserevz_dn6: f64 = *var_vgserevz_dn6_slot;
        let mut var_vgserevz_dn7: f64 = *var_vgserevz_dn7_slot;
        let mut var_vgserevz_dn8: f64 = *var_vgserevz_dn8_slot;
        let mut var_vgserevz_dn9: f64 = *var_vgserevz_dn9_slot;
        let mut var_vgserevz_rv: f64 = *var_vgserevz_rv_slot;
        let mut var_vsubsrev: f64 = *var_vsubsrev_slot;
        let mut var_vsubsrev_dn0: f64 = *var_vsubsrev_dn0_slot;
        let mut var_vsubsrev_dn2: f64 = *var_vsubsrev_dn2_slot;
        let mut var_vsubsrev_rv: f64 = *var_vsubsrev_rv_slot;
        let mut var_vxbgmt: f64 = *var_vxbgmt_slot;
        let mut var_vxbgmt_dn0: f64 = *var_vxbgmt_dn0_slot;
        let mut var_vxbgmt_dn10: f64 = *var_vxbgmt_dn10_slot;
        let mut var_vxbgmt_dn13: f64 = *var_vxbgmt_dn13_slot;
        let mut var_vxbgmt_dn2: f64 = *var_vxbgmt_dn2_slot;
        let mut var_vxbgmt_dn4: f64 = *var_vxbgmt_dn4_slot;
        let mut var_vxbgmt_dn5: f64 = *var_vxbgmt_dn5_slot;
        let mut var_vxbgmt_dn6: f64 = *var_vxbgmt_dn6_slot;
        let mut var_vxbgmt_dn7: f64 = *var_vxbgmt_dn7_slot;
        let mut var_vxbgmt_dn8: f64 = *var_vxbgmt_dn8_slot;
        let mut var_vxbgmt_dn9: f64 = *var_vxbgmt_dn9_slot;
        let mut var_vxbgmt_rv: f64 = *var_vxbgmt_rv_slot;
        let mut var_vxbgmtcl: f64 = *var_vxbgmtcl_slot;
        let mut var_vxbgmtcl_dn0: f64 = *var_vxbgmtcl_dn0_slot;
        let mut var_vxbgmtcl_dn10: f64 = *var_vxbgmtcl_dn10_slot;
        let mut var_vxbgmtcl_dn13: f64 = *var_vxbgmtcl_dn13_slot;
        let mut var_vxbgmtcl_dn2: f64 = *var_vxbgmtcl_dn2_slot;
        let mut var_vxbgmtcl_dn4: f64 = *var_vxbgmtcl_dn4_slot;
        let mut var_vxbgmtcl_dn5: f64 = *var_vxbgmtcl_dn5_slot;
        let mut var_vxbgmtcl_dn6: f64 = *var_vxbgmtcl_dn6_slot;
        let mut var_vxbgmtcl_dn7: f64 = *var_vxbgmtcl_dn7_slot;
        let mut var_vxbgmtcl_dn8: f64 = *var_vxbgmtcl_dn8_slot;
        let mut var_vxbgmtcl_dn9: f64 = *var_vxbgmtcl_dn9_slot;
        let mut var_vxbgmtcl_rv: f64 = *var_vxbgmtcl_rv_slot;

        var_t11 = 0.0;
        var_t11_dn0 = 0.0;
        var_t11_dn2 = 0.0;
        var_t11_dn4 = 0.0;
        var_t11_dn5 = 0.0;
        var_t11_dn6 = 0.0;
        var_t11_dn7 = 0.0;
        var_t11_dn8 = 0.0;
        var_t11_dn9 = 0.0;
        var_t11_dn10 = 0.0;
        var_t11_dn13 = 0.0;
        var_t11_rv = 0.0;

        var_t12 = 0.0;
        var_t12_dn0 = 0.0;
        var_t12_dn2 = 0.0;
        var_t12_dn4 = 0.0;
        var_t12_dn5 = 0.0;
        var_t12_dn6 = 0.0;
        var_t12_dn7 = 0.0;
        var_t12_dn8 = 0.0;
        var_t12_dn9 = 0.0;
        var_t12_dn10 = 0.0;
        var_t12_dn13 = 0.0;
        var_t12_rv = 0.0;

        var_vdseff = 0.0;
        var_vdseff_dn0 = 0.0;
        var_vdseff_dn2 = 0.0;
        var_vdseff_dn4 = 0.0;
        var_vdseff_dn5 = 0.0;
        var_vdseff_dn6 = 0.0;
        var_vdseff_dn7 = 0.0;
        var_vdseff_dn8 = 0.0;
        var_vdseff_dn9 = 0.0;
        var_vdseff_dn10 = 0.0;
        var_vdseff_dn13 = 0.0;
        var_vdseff_rv = 0.0;

        var_vdsorg = 0.0;
        var_vdsorg_dn0 = 0.0;
        var_vdsorg_dn2 = 0.0;
        var_vdsorg_dn4 = 0.0;
        var_vdsorg_dn5 = 0.0;
        var_vdsorg_dn6 = 0.0;
        var_vdsorg_dn7 = 0.0;
        var_vdsorg_dn8 = 0.0;
        var_vdsorg_dn9 = 0.0;
        var_vdsorg_dn10 = 0.0;
        var_vdsorg_dn13 = 0.0;
        var_vdsorg_rv = 0.0;

        var_qovdext = 0.0;
        var_qovdext_dn0 = 0.0;
        var_qovdext_dn2 = 0.0;
        var_qovdext_dn4 = 0.0;
        var_qovdext_dn5 = 0.0;
        var_qovdext_dn6 = 0.0;
        var_qovdext_dn7 = 0.0;
        var_qovdext_dn8 = 0.0;
        var_qovdext_dn9 = 0.0;
        var_qovdext_dn10 = 0.0;
        var_qovdext_dn13 = 0.0;
        var_qovdext_rv = 0.0;

        var_qovsext = 0.0;
        var_qovsext_dn0 = 0.0;
        var_qovsext_dn2 = 0.0;
        var_qovsext_dn4 = 0.0;
        var_qovsext_dn5 = 0.0;
        var_qovsext_dn6 = 0.0;
        var_qovsext_dn7 = 0.0;
        var_qovsext_dn8 = 0.0;
        var_qovsext_dn9 = 0.0;
        var_qovsext_dn10 = 0.0;
        var_qovsext_dn13 = 0.0;
        var_qovsext_rv = 0.0;

        var_qovd = 0.0;
        var_qovd_dn0 = 0.0;
        var_qovd_dn2 = 0.0;
        var_qovd_dn4 = 0.0;
        var_qovd_dn5 = 0.0;
        var_qovd_dn6 = 0.0;
        var_qovd_dn7 = 0.0;
        var_qovd_dn8 = 0.0;
        var_qovd_dn9 = 0.0;
        var_qovd_dn10 = 0.0;
        var_qovd_dn13 = 0.0;
        var_qovd_rv = 0.0;

        var_qovs = 0.0;
        var_qovs_dn0 = 0.0;
        var_qovs_dn2 = 0.0;
        var_qovs_dn4 = 0.0;
        var_qovs_dn5 = 0.0;
        var_qovs_dn6 = 0.0;
        var_qovs_dn7 = 0.0;
        var_qovs_dn8 = 0.0;
        var_qovs_dn9 = 0.0;
        var_qovs_dn10 = 0.0;
        var_qovs_dn13 = 0.0;
        var_qovs_rv = 0.0;

        var_qbuld = 0.0;
        var_qbuld_dn0 = 0.0;
        var_qbuld_dn2 = 0.0;
        var_qbuld_dn4 = 0.0;
        var_qbuld_dn5 = 0.0;
        var_qbuld_dn6 = 0.0;
        var_qbuld_dn7 = 0.0;
        var_qbuld_dn8 = 0.0;
        var_qbuld_dn9 = 0.0;
        var_qbuld_dn10 = 0.0;
        var_qbuld_dn13 = 0.0;
        var_qbuld_rv = 0.0;

        var_qbdld = 0.0;
        var_qbdld_dn0 = 0.0;
        var_qbdld_dn2 = 0.0;
        var_qbdld_dn4 = 0.0;
        var_qbdld_dn5 = 0.0;
        var_qbdld_dn6 = 0.0;
        var_qbdld_dn7 = 0.0;
        var_qbdld_dn8 = 0.0;
        var_qbdld_dn9 = 0.0;
        var_qbdld_dn10 = 0.0;
        var_qbdld_dn13 = 0.0;
        var_qbdld_rv = 0.0;

        var_qbsld = 0.0;
        var_qbsld_dn0 = 0.0;
        var_qbsld_dn2 = 0.0;
        var_qbsld_dn4 = 0.0;
        var_qbsld_dn5 = 0.0;
        var_qbsld_dn6 = 0.0;
        var_qbsld_dn7 = 0.0;
        var_qbsld_dn8 = 0.0;
        var_qbsld_dn9 = 0.0;
        var_qbsld_dn10 = 0.0;
        var_qbsld_dn13 = 0.0;
        var_qbsld_rv = 0.0;

        var_qodad = 0.0;
        var_qodad_dn0 = 0.0;
        var_qodad_dn2 = 0.0;
        var_qodad_dn4 = 0.0;
        var_qodad_dn5 = 0.0;
        var_qodad_dn6 = 0.0;
        var_qodad_dn7 = 0.0;
        var_qodad_dn8 = 0.0;
        var_qodad_dn9 = 0.0;
        var_qodad_dn10 = 0.0;
        var_qodad_dn13 = 0.0;
        var_qodad_rv = 0.0;

        var_qbdldext = 0.0;
        var_qbdldext_dn0 = 0.0;
        var_qbdldext_dn2 = 0.0;
        var_qbdldext_dn4 = 0.0;
        var_qbdldext_dn5 = 0.0;
        var_qbdldext_dn6 = 0.0;
        var_qbdldext_dn7 = 0.0;
        var_qbdldext_dn8 = 0.0;
        var_qbdldext_dn9 = 0.0;
        var_qbdldext_dn10 = 0.0;
        var_qbdldext_dn13 = 0.0;
        var_qbdldext_rv = 0.0;

        var_qbsldext = 0.0;
        var_qbsldext_dn0 = 0.0;
        var_qbsldext_dn2 = 0.0;
        var_qbsldext_dn4 = 0.0;
        var_qbsldext_dn5 = 0.0;
        var_qbsldext_dn6 = 0.0;
        var_qbsldext_dn7 = 0.0;
        var_qbsldext_dn8 = 0.0;
        var_qbsldext_dn9 = 0.0;
        var_qbsldext_dn10 = 0.0;
        var_qbsldext_dn13 = 0.0;
        var_qbsldext_rv = 0.0;

        var_vbsz2 = 0.0;
        var_vbsz2_dn0 = 0.0;
        var_vbsz2_dn2 = 0.0;
        var_vbsz2_dn4 = 0.0;
        var_vbsz2_dn5 = 0.0;
        var_vbsz2_dn6 = 0.0;
        var_vbsz2_dn7 = 0.0;
        var_vbsz2_dn8 = 0.0;
        var_vbsz2_dn9 = 0.0;
        var_vbsz2_dn10 = 0.0;
        var_vbsz2_dn13 = 0.0;
        var_vbsz2_rv = 0.0;

        var_rdrift = 0.0;
        var_rdrift_dn0 = 0.0;
        var_rdrift_dn2 = 0.0;
        var_rdrift_dn4 = 0.0;
        var_rdrift_dn5 = 0.0;
        var_rdrift_dn6 = 0.0;
        var_rdrift_dn7 = 0.0;
        var_rdrift_dn8 = 0.0;
        var_rdrift_dn9 = 0.0;
        var_rdrift_dn10 = 0.0;
        var_rdrift_dn13 = 0.0;
        var_rdrift_rv = 0.0;

        var_rsdrift = 0.0;
        var_rsdrift_dn0 = 0.0;
        var_rsdrift_dn2 = 0.0;
        var_rsdrift_dn4 = 0.0;
        var_rsdrift_dn5 = 0.0;
        var_rsdrift_dn6 = 0.0;
        var_rsdrift_dn7 = 0.0;
        var_rsdrift_dn8 = 0.0;
        var_rsdrift_dn9 = 0.0;
        var_rsdrift_dn10 = 0.0;
        var_rsdrift_dn13 = 0.0;
        var_rsdrift_rv = 0.0;

        var_ra = 0.0;
        var_ra_dn0 = 0.0;
        var_ra_dn2 = 0.0;
        var_ra_dn4 = 0.0;
        var_ra_dn5 = 0.0;
        var_ra_dn6 = 0.0;
        var_ra_dn7 = 0.0;
        var_ra_dn8 = 0.0;
        var_ra_dn9 = 0.0;
        var_ra_dn10 = 0.0;
        var_ra_dn13 = 0.0;
        var_ra_rv = 0.0;

        var_vdse_eff = 0.0;
        var_vdse_eff_dn0 = 0.0;
        var_vdse_eff_dn2 = 0.0;
        var_vdse_eff_rv = 0.0;

        var_vdsemodenml = 0.0;
        var_vdsemodenml_rv = 0.0;

        var_vdsemodervs = 0.0;
        var_vdsemodervs_rv = 0.0;

        var_vbsegmt = 0.0;
        var_vbsegmt_dn2 = 0.0;
        var_vbsegmt_dn8 = 0.0;
        var_vbsegmt_rv = 0.0;

        var_vdsegmt = 0.0;
        var_vdsegmt_dn0 = 0.0;
        var_vdsegmt_dn2 = 0.0;
        var_vdsegmt_rv = 0.0;

        var_vgsegmt = 0.0;
        var_vgsegmt_dn2 = 0.0;
        var_vgsegmt_dn6 = 0.0;
        var_vgsegmt_rv = 0.0;

        var_vbserev = 0.0;
        var_vbserev_dn0 = 0.0;
        var_vbserev_dn2 = 0.0;
        var_vbserev_dn8 = 0.0;
        var_vbserev_rv = 0.0;

        var_vdserev = 0.0;
        var_vdserev_dn0 = 0.0;
        var_vdserev_dn2 = 0.0;
        var_vdserev_rv = 0.0;

        var_vgserev = 0.0;
        var_vgserev_dn0 = 0.0;
        var_vgserev_dn2 = 0.0;
        var_vgserev_dn6 = 0.0;
        var_vgserev_rv = 0.0;

        var_vdserevz = 0.0;
        var_vdserevz_dn0 = 0.0;
        var_vdserevz_dn2 = 0.0;
        var_vdserevz_dn4 = 0.0;
        var_vdserevz_dn5 = 0.0;
        var_vdserevz_dn6 = 0.0;
        var_vdserevz_dn7 = 0.0;
        var_vdserevz_dn8 = 0.0;
        var_vdserevz_dn9 = 0.0;
        var_vdserevz_dn10 = 0.0;
        var_vdserevz_dn13 = 0.0;
        var_vdserevz_rv = 0.0;

        var_vgserevz = 0.0;
        var_vgserevz_dn0 = 0.0;
        var_vgserevz_dn2 = 0.0;
        var_vgserevz_dn4 = 0.0;
        var_vgserevz_dn5 = 0.0;
        var_vgserevz_dn6 = 0.0;
        var_vgserevz_dn7 = 0.0;
        var_vgserevz_dn8 = 0.0;
        var_vgserevz_dn9 = 0.0;
        var_vgserevz_dn10 = 0.0;
        var_vgserevz_dn13 = 0.0;
        var_vgserevz_rv = 0.0;

        var_vbserevz = 0.0;
        var_vbserevz_dn0 = 0.0;
        var_vbserevz_dn2 = 0.0;
        var_vbserevz_dn4 = 0.0;
        var_vbserevz_dn5 = 0.0;
        var_vbserevz_dn6 = 0.0;
        var_vbserevz_dn7 = 0.0;
        var_vbserevz_dn8 = 0.0;
        var_vbserevz_dn9 = 0.0;
        var_vbserevz_dn10 = 0.0;
        var_vbserevz_dn13 = 0.0;
        var_vbserevz_rv = 0.0;

        var_vsubsrev = 0.0;
        var_vsubsrev_dn0 = 0.0;
        var_vsubsrev_dn2 = 0.0;
        var_vsubsrev_rv = 0.0;

        var_ttemp = 0.0;
        var_ttemp_dn0 = 0.0;
        var_ttemp_dn2 = 0.0;
        var_ttemp_dn4 = 0.0;
        var_ttemp_dn5 = 0.0;
        var_ttemp_dn6 = 0.0;
        var_ttemp_dn7 = 0.0;
        var_ttemp_dn8 = 0.0;
        var_ttemp_dn9 = 0.0;
        var_ttemp_dn10 = 0.0;
        var_ttemp_dn13 = 0.0;
        var_ttemp_rv = 0.0;

        var_ttemp0 = 0.0;
        var_ttemp0_dn0 = 0.0;
        var_ttemp0_dn2 = 0.0;
        var_ttemp0_dn4 = 0.0;
        var_ttemp0_dn5 = 0.0;
        var_ttemp0_dn6 = 0.0;
        var_ttemp0_dn7 = 0.0;
        var_ttemp0_dn8 = 0.0;
        var_ttemp0_dn9 = 0.0;
        var_ttemp0_dn10 = 0.0;
        var_ttemp0_dn13 = 0.0;
        var_ttemp0_rv = 0.0;

        var_tdiff0 = 0.0;
        var_tdiff0_dn0 = 0.0;
        var_tdiff0_dn2 = 0.0;
        var_tdiff0_dn4 = 0.0;
        var_tdiff0_dn5 = 0.0;
        var_tdiff0_dn6 = 0.0;
        var_tdiff0_dn7 = 0.0;
        var_tdiff0_dn8 = 0.0;
        var_tdiff0_dn9 = 0.0;
        var_tdiff0_dn10 = 0.0;
        var_tdiff0_dn13 = 0.0;
        var_tdiff0_rv = 0.0;

        var_tdiff0_2 = 0.0;
        var_tdiff0_2_dn0 = 0.0;
        var_tdiff0_2_dn2 = 0.0;
        var_tdiff0_2_dn4 = 0.0;
        var_tdiff0_2_dn5 = 0.0;
        var_tdiff0_2_dn6 = 0.0;
        var_tdiff0_2_dn7 = 0.0;
        var_tdiff0_2_dn8 = 0.0;
        var_tdiff0_2_dn9 = 0.0;
        var_tdiff0_2_dn10 = 0.0;
        var_tdiff0_2_dn13 = 0.0;
        var_tdiff0_2_rv = 0.0;

        var_tdiff = 0.0;
        var_tdiff_dn0 = 0.0;
        var_tdiff_dn2 = 0.0;
        var_tdiff_dn4 = 0.0;
        var_tdiff_dn5 = 0.0;
        var_tdiff_dn6 = 0.0;
        var_tdiff_dn7 = 0.0;
        var_tdiff_dn8 = 0.0;
        var_tdiff_dn9 = 0.0;
        var_tdiff_dn10 = 0.0;
        var_tdiff_dn13 = 0.0;
        var_tdiff_rv = 0.0;

        var_tdiff_2 = 0.0;
        var_tdiff_2_dn0 = 0.0;
        var_tdiff_2_dn2 = 0.0;
        var_tdiff_2_dn4 = 0.0;
        var_tdiff_2_dn5 = 0.0;
        var_tdiff_2_dn6 = 0.0;
        var_tdiff_2_dn7 = 0.0;
        var_tdiff_2_dn8 = 0.0;
        var_tdiff_2_dn9 = 0.0;
        var_tdiff_2_dn10 = 0.0;
        var_tdiff_2_dn13 = 0.0;
        var_tdiff_2_rv = 0.0;

        var_eg = 0.0;
        var_eg_dn0 = 0.0;
        var_eg_dn2 = 0.0;
        var_eg_dn4 = 0.0;
        var_eg_dn5 = 0.0;
        var_eg_dn6 = 0.0;
        var_eg_dn7 = 0.0;
        var_eg_dn8 = 0.0;
        var_eg_dn9 = 0.0;
        var_eg_dn10 = 0.0;
        var_eg_dn13 = 0.0;
        var_eg_rv = 0.0;

        var_nin = 0.0;
        var_nin_dn0 = 0.0;
        var_nin_dn2 = 0.0;
        var_nin_dn4 = 0.0;
        var_nin_dn5 = 0.0;
        var_nin_dn6 = 0.0;
        var_nin_dn7 = 0.0;
        var_nin_dn8 = 0.0;
        var_nin_dn9 = 0.0;
        var_nin_dn10 = 0.0;
        var_nin_dn13 = 0.0;
        var_nin_rv = 0.0;

        var_vgbgmt = 0.0;
        var_vgbgmt_dn2 = 0.0;
        var_vgbgmt_dn6 = 0.0;
        var_vgbgmt_dn7 = 0.0;
        var_vgbgmt_dn8 = 0.0;
        var_vgbgmt_rv = 0.0;

        var_vxbgmt = 0.0;
        var_vxbgmt_dn0 = 0.0;
        var_vxbgmt_dn2 = 0.0;
        var_vxbgmt_dn4 = 0.0;
        var_vxbgmt_dn5 = 0.0;
        var_vxbgmt_dn6 = 0.0;
        var_vxbgmt_dn7 = 0.0;
        var_vxbgmt_dn8 = 0.0;
        var_vxbgmt_dn9 = 0.0;
        var_vxbgmt_dn10 = 0.0;
        var_vxbgmt_dn13 = 0.0;
        var_vxbgmt_rv = 0.0;

        var_vxbgmtcl = 0.0;
        var_vxbgmtcl_dn0 = 0.0;
        var_vxbgmtcl_dn2 = 0.0;
        var_vxbgmtcl_dn4 = 0.0;
        var_vxbgmtcl_dn5 = 0.0;
        var_vxbgmtcl_dn6 = 0.0;
        var_vxbgmtcl_dn7 = 0.0;
        var_vxbgmtcl_dn8 = 0.0;
        var_vxbgmtcl_dn9 = 0.0;
        var_vxbgmtcl_dn10 = 0.0;
        var_vxbgmtcl_dn13 = 0.0;
        var_vxbgmtcl_rv = 0.0;

        var_qsuld = 0.0;
        var_qsuld_dn0 = 0.0;
        var_qsuld_dn2 = 0.0;
        var_qsuld_dn4 = 0.0;
        var_qsuld_dn5 = 0.0;
        var_qsuld_dn6 = 0.0;
        var_qsuld_dn7 = 0.0;
        var_qsuld_dn8 = 0.0;
        var_qsuld_dn9 = 0.0;
        var_qsuld_dn10 = 0.0;
        var_qsuld_dn13 = 0.0;
        var_qsuld_rv = 0.0;

        var_qiuld = 0.0;
        var_qiuld_dn0 = 0.0;
        var_qiuld_dn2 = 0.0;
        var_qiuld_dn4 = 0.0;
        var_qiuld_dn5 = 0.0;
        var_qiuld_dn6 = 0.0;
        var_qiuld_dn7 = 0.0;
        var_qiuld_dn8 = 0.0;
        var_qiuld_dn9 = 0.0;
        var_qiuld_dn10 = 0.0;
        var_qiuld_dn13 = 0.0;
        var_qiuld_rv = 0.0;

        var_idsibpc = 0.0;
        var_idsibpc_dn0 = 0.0;
        var_idsibpc_dn2 = 0.0;
        var_idsibpc_dn4 = 0.0;
        var_idsibpc_dn5 = 0.0;
        var_idsibpc_dn6 = 0.0;
        var_idsibpc_dn7 = 0.0;
        var_idsibpc_dn8 = 0.0;
        var_idsibpc_dn9 = 0.0;
        var_idsibpc_dn10 = 0.0;
        var_idsibpc_dn13 = 0.0;
        var_idsibpc_rv = 0.0;

        var_vgpld = 0.0;
        var_vgpld_dn2 = 0.0;
        var_vgpld_dn6 = 0.0;
        var_vgpld_dn7 = 0.0;
        var_vgpld_dn8 = 0.0;
        var_vgpld_rv = 0.0;

        *var_eg_slot = var_eg;
        *var_eg_dn0_slot = var_eg_dn0;
        *var_eg_dn10_slot = var_eg_dn10;
        *var_eg_dn13_slot = var_eg_dn13;
        *var_eg_dn2_slot = var_eg_dn2;
        *var_eg_dn4_slot = var_eg_dn4;
        *var_eg_dn5_slot = var_eg_dn5;
        *var_eg_dn6_slot = var_eg_dn6;
        *var_eg_dn7_slot = var_eg_dn7;
        *var_eg_dn8_slot = var_eg_dn8;
        *var_eg_dn9_slot = var_eg_dn9;
        *var_eg_rv_slot = var_eg_rv;
        *var_idsibpc_slot = var_idsibpc;
        *var_idsibpc_dn0_slot = var_idsibpc_dn0;
        *var_idsibpc_dn10_slot = var_idsibpc_dn10;
        *var_idsibpc_dn13_slot = var_idsibpc_dn13;
        *var_idsibpc_dn2_slot = var_idsibpc_dn2;
        *var_idsibpc_dn4_slot = var_idsibpc_dn4;
        *var_idsibpc_dn5_slot = var_idsibpc_dn5;
        *var_idsibpc_dn6_slot = var_idsibpc_dn6;
        *var_idsibpc_dn7_slot = var_idsibpc_dn7;
        *var_idsibpc_dn8_slot = var_idsibpc_dn8;
        *var_idsibpc_dn9_slot = var_idsibpc_dn9;
        *var_idsibpc_rv_slot = var_idsibpc_rv;
        *var_nin_slot = var_nin;
        *var_nin_dn0_slot = var_nin_dn0;
        *var_nin_dn10_slot = var_nin_dn10;
        *var_nin_dn13_slot = var_nin_dn13;
        *var_nin_dn2_slot = var_nin_dn2;
        *var_nin_dn4_slot = var_nin_dn4;
        *var_nin_dn5_slot = var_nin_dn5;
        *var_nin_dn6_slot = var_nin_dn6;
        *var_nin_dn7_slot = var_nin_dn7;
        *var_nin_dn8_slot = var_nin_dn8;
        *var_nin_dn9_slot = var_nin_dn9;
        *var_nin_rv_slot = var_nin_rv;
        *var_qbdld_slot = var_qbdld;
        *var_qbdld_dn0_slot = var_qbdld_dn0;
        *var_qbdld_dn10_slot = var_qbdld_dn10;
        *var_qbdld_dn13_slot = var_qbdld_dn13;
        *var_qbdld_dn2_slot = var_qbdld_dn2;
        *var_qbdld_dn4_slot = var_qbdld_dn4;
        *var_qbdld_dn5_slot = var_qbdld_dn5;
        *var_qbdld_dn6_slot = var_qbdld_dn6;
        *var_qbdld_dn7_slot = var_qbdld_dn7;
        *var_qbdld_dn8_slot = var_qbdld_dn8;
        *var_qbdld_dn9_slot = var_qbdld_dn9;
        *var_qbdld_rv_slot = var_qbdld_rv;
        *var_qbdldext_slot = var_qbdldext;
        *var_qbdldext_dn0_slot = var_qbdldext_dn0;
        *var_qbdldext_dn10_slot = var_qbdldext_dn10;
        *var_qbdldext_dn13_slot = var_qbdldext_dn13;
        *var_qbdldext_dn2_slot = var_qbdldext_dn2;
        *var_qbdldext_dn4_slot = var_qbdldext_dn4;
        *var_qbdldext_dn5_slot = var_qbdldext_dn5;
        *var_qbdldext_dn6_slot = var_qbdldext_dn6;
        *var_qbdldext_dn7_slot = var_qbdldext_dn7;
        *var_qbdldext_dn8_slot = var_qbdldext_dn8;
        *var_qbdldext_dn9_slot = var_qbdldext_dn9;
        *var_qbdldext_rv_slot = var_qbdldext_rv;
        *var_qbsld_slot = var_qbsld;
        *var_qbsld_dn0_slot = var_qbsld_dn0;
        *var_qbsld_dn10_slot = var_qbsld_dn10;
        *var_qbsld_dn13_slot = var_qbsld_dn13;
        *var_qbsld_dn2_slot = var_qbsld_dn2;
        *var_qbsld_dn4_slot = var_qbsld_dn4;
        *var_qbsld_dn5_slot = var_qbsld_dn5;
        *var_qbsld_dn6_slot = var_qbsld_dn6;
        *var_qbsld_dn7_slot = var_qbsld_dn7;
        *var_qbsld_dn8_slot = var_qbsld_dn8;
        *var_qbsld_dn9_slot = var_qbsld_dn9;
        *var_qbsld_rv_slot = var_qbsld_rv;
        *var_qbsldext_slot = var_qbsldext;
        *var_qbsldext_dn0_slot = var_qbsldext_dn0;
        *var_qbsldext_dn10_slot = var_qbsldext_dn10;
        *var_qbsldext_dn13_slot = var_qbsldext_dn13;
        *var_qbsldext_dn2_slot = var_qbsldext_dn2;
        *var_qbsldext_dn4_slot = var_qbsldext_dn4;
        *var_qbsldext_dn5_slot = var_qbsldext_dn5;
        *var_qbsldext_dn6_slot = var_qbsldext_dn6;
        *var_qbsldext_dn7_slot = var_qbsldext_dn7;
        *var_qbsldext_dn8_slot = var_qbsldext_dn8;
        *var_qbsldext_dn9_slot = var_qbsldext_dn9;
        *var_qbsldext_rv_slot = var_qbsldext_rv;
        *var_qbuld_slot = var_qbuld;
        *var_qbuld_dn0_slot = var_qbuld_dn0;
        *var_qbuld_dn10_slot = var_qbuld_dn10;
        *var_qbuld_dn13_slot = var_qbuld_dn13;
        *var_qbuld_dn2_slot = var_qbuld_dn2;
        *var_qbuld_dn4_slot = var_qbuld_dn4;
        *var_qbuld_dn5_slot = var_qbuld_dn5;
        *var_qbuld_dn6_slot = var_qbuld_dn6;
        *var_qbuld_dn7_slot = var_qbuld_dn7;
        *var_qbuld_dn8_slot = var_qbuld_dn8;
        *var_qbuld_dn9_slot = var_qbuld_dn9;
        *var_qbuld_rv_slot = var_qbuld_rv;
        *var_qiuld_slot = var_qiuld;
        *var_qiuld_dn0_slot = var_qiuld_dn0;
        *var_qiuld_dn10_slot = var_qiuld_dn10;
        *var_qiuld_dn13_slot = var_qiuld_dn13;
        *var_qiuld_dn2_slot = var_qiuld_dn2;
        *var_qiuld_dn4_slot = var_qiuld_dn4;
        *var_qiuld_dn5_slot = var_qiuld_dn5;
        *var_qiuld_dn6_slot = var_qiuld_dn6;
        *var_qiuld_dn7_slot = var_qiuld_dn7;
        *var_qiuld_dn8_slot = var_qiuld_dn8;
        *var_qiuld_dn9_slot = var_qiuld_dn9;
        *var_qiuld_rv_slot = var_qiuld_rv;
        *var_qodad_slot = var_qodad;
        *var_qodad_dn0_slot = var_qodad_dn0;
        *var_qodad_dn10_slot = var_qodad_dn10;
        *var_qodad_dn13_slot = var_qodad_dn13;
        *var_qodad_dn2_slot = var_qodad_dn2;
        *var_qodad_dn4_slot = var_qodad_dn4;
        *var_qodad_dn5_slot = var_qodad_dn5;
        *var_qodad_dn6_slot = var_qodad_dn6;
        *var_qodad_dn7_slot = var_qodad_dn7;
        *var_qodad_dn8_slot = var_qodad_dn8;
        *var_qodad_dn9_slot = var_qodad_dn9;
        *var_qodad_rv_slot = var_qodad_rv;
        *var_qovd_slot = var_qovd;
        *var_qovd_dn0_slot = var_qovd_dn0;
        *var_qovd_dn10_slot = var_qovd_dn10;
        *var_qovd_dn13_slot = var_qovd_dn13;
        *var_qovd_dn2_slot = var_qovd_dn2;
        *var_qovd_dn4_slot = var_qovd_dn4;
        *var_qovd_dn5_slot = var_qovd_dn5;
        *var_qovd_dn6_slot = var_qovd_dn6;
        *var_qovd_dn7_slot = var_qovd_dn7;
        *var_qovd_dn8_slot = var_qovd_dn8;
        *var_qovd_dn9_slot = var_qovd_dn9;
        *var_qovd_rv_slot = var_qovd_rv;
        *var_qovdext_slot = var_qovdext;
        *var_qovdext_dn0_slot = var_qovdext_dn0;
        *var_qovdext_dn10_slot = var_qovdext_dn10;
        *var_qovdext_dn13_slot = var_qovdext_dn13;
        *var_qovdext_dn2_slot = var_qovdext_dn2;
        *var_qovdext_dn4_slot = var_qovdext_dn4;
        *var_qovdext_dn5_slot = var_qovdext_dn5;
        *var_qovdext_dn6_slot = var_qovdext_dn6;
        *var_qovdext_dn7_slot = var_qovdext_dn7;
        *var_qovdext_dn8_slot = var_qovdext_dn8;
        *var_qovdext_dn9_slot = var_qovdext_dn9;
        *var_qovdext_rv_slot = var_qovdext_rv;
        *var_qovs_slot = var_qovs;
        *var_qovs_dn0_slot = var_qovs_dn0;
        *var_qovs_dn10_slot = var_qovs_dn10;
        *var_qovs_dn13_slot = var_qovs_dn13;
        *var_qovs_dn2_slot = var_qovs_dn2;
        *var_qovs_dn4_slot = var_qovs_dn4;
        *var_qovs_dn5_slot = var_qovs_dn5;
        *var_qovs_dn6_slot = var_qovs_dn6;
        *var_qovs_dn7_slot = var_qovs_dn7;
        *var_qovs_dn8_slot = var_qovs_dn8;
        *var_qovs_dn9_slot = var_qovs_dn9;
        *var_qovs_rv_slot = var_qovs_rv;
        *var_qovsext_slot = var_qovsext;
        *var_qovsext_dn0_slot = var_qovsext_dn0;
        *var_qovsext_dn10_slot = var_qovsext_dn10;
        *var_qovsext_dn13_slot = var_qovsext_dn13;
        *var_qovsext_dn2_slot = var_qovsext_dn2;
        *var_qovsext_dn4_slot = var_qovsext_dn4;
        *var_qovsext_dn5_slot = var_qovsext_dn5;
        *var_qovsext_dn6_slot = var_qovsext_dn6;
        *var_qovsext_dn7_slot = var_qovsext_dn7;
        *var_qovsext_dn8_slot = var_qovsext_dn8;
        *var_qovsext_dn9_slot = var_qovsext_dn9;
        *var_qovsext_rv_slot = var_qovsext_rv;
        *var_qsuld_slot = var_qsuld;
        *var_qsuld_dn0_slot = var_qsuld_dn0;
        *var_qsuld_dn10_slot = var_qsuld_dn10;
        *var_qsuld_dn13_slot = var_qsuld_dn13;
        *var_qsuld_dn2_slot = var_qsuld_dn2;
        *var_qsuld_dn4_slot = var_qsuld_dn4;
        *var_qsuld_dn5_slot = var_qsuld_dn5;
        *var_qsuld_dn6_slot = var_qsuld_dn6;
        *var_qsuld_dn7_slot = var_qsuld_dn7;
        *var_qsuld_dn8_slot = var_qsuld_dn8;
        *var_qsuld_dn9_slot = var_qsuld_dn9;
        *var_qsuld_rv_slot = var_qsuld_rv;
        *var_ra_slot = var_ra;
        *var_ra_dn0_slot = var_ra_dn0;
        *var_ra_dn10_slot = var_ra_dn10;
        *var_ra_dn13_slot = var_ra_dn13;
        *var_ra_dn2_slot = var_ra_dn2;
        *var_ra_dn4_slot = var_ra_dn4;
        *var_ra_dn5_slot = var_ra_dn5;
        *var_ra_dn6_slot = var_ra_dn6;
        *var_ra_dn7_slot = var_ra_dn7;
        *var_ra_dn8_slot = var_ra_dn8;
        *var_ra_dn9_slot = var_ra_dn9;
        *var_ra_rv_slot = var_ra_rv;
        *var_rdrift_slot = var_rdrift;
        *var_rdrift_dn0_slot = var_rdrift_dn0;
        *var_rdrift_dn10_slot = var_rdrift_dn10;
        *var_rdrift_dn13_slot = var_rdrift_dn13;
        *var_rdrift_dn2_slot = var_rdrift_dn2;
        *var_rdrift_dn4_slot = var_rdrift_dn4;
        *var_rdrift_dn5_slot = var_rdrift_dn5;
        *var_rdrift_dn6_slot = var_rdrift_dn6;
        *var_rdrift_dn7_slot = var_rdrift_dn7;
        *var_rdrift_dn8_slot = var_rdrift_dn8;
        *var_rdrift_dn9_slot = var_rdrift_dn9;
        *var_rdrift_rv_slot = var_rdrift_rv;
        *var_rsdrift_slot = var_rsdrift;
        *var_rsdrift_dn0_slot = var_rsdrift_dn0;
        *var_rsdrift_dn10_slot = var_rsdrift_dn10;
        *var_rsdrift_dn13_slot = var_rsdrift_dn13;
        *var_rsdrift_dn2_slot = var_rsdrift_dn2;
        *var_rsdrift_dn4_slot = var_rsdrift_dn4;
        *var_rsdrift_dn5_slot = var_rsdrift_dn5;
        *var_rsdrift_dn6_slot = var_rsdrift_dn6;
        *var_rsdrift_dn7_slot = var_rsdrift_dn7;
        *var_rsdrift_dn8_slot = var_rsdrift_dn8;
        *var_rsdrift_dn9_slot = var_rsdrift_dn9;
        *var_rsdrift_rv_slot = var_rsdrift_rv;
        *var_t11_slot = var_t11;
        *var_t11_dn0_slot = var_t11_dn0;
        *var_t11_dn10_slot = var_t11_dn10;
        *var_t11_dn13_slot = var_t11_dn13;
        *var_t11_dn2_slot = var_t11_dn2;
        *var_t11_dn4_slot = var_t11_dn4;
        *var_t11_dn5_slot = var_t11_dn5;
        *var_t11_dn6_slot = var_t11_dn6;
        *var_t11_dn7_slot = var_t11_dn7;
        *var_t11_dn8_slot = var_t11_dn8;
        *var_t11_dn9_slot = var_t11_dn9;
        *var_t11_rv_slot = var_t11_rv;
        *var_t12_slot = var_t12;
        *var_t12_dn0_slot = var_t12_dn0;
        *var_t12_dn10_slot = var_t12_dn10;
        *var_t12_dn13_slot = var_t12_dn13;
        *var_t12_dn2_slot = var_t12_dn2;
        *var_t12_dn4_slot = var_t12_dn4;
        *var_t12_dn5_slot = var_t12_dn5;
        *var_t12_dn6_slot = var_t12_dn6;
        *var_t12_dn7_slot = var_t12_dn7;
        *var_t12_dn8_slot = var_t12_dn8;
        *var_t12_dn9_slot = var_t12_dn9;
        *var_t12_rv_slot = var_t12_rv;
        *var_tdiff_slot = var_tdiff;
        *var_tdiff0_slot = var_tdiff0;
        *var_tdiff0_2_slot = var_tdiff0_2;
        *var_tdiff0_2_dn0_slot = var_tdiff0_2_dn0;
        *var_tdiff0_2_dn10_slot = var_tdiff0_2_dn10;
        *var_tdiff0_2_dn13_slot = var_tdiff0_2_dn13;
        *var_tdiff0_2_dn2_slot = var_tdiff0_2_dn2;
        *var_tdiff0_2_dn4_slot = var_tdiff0_2_dn4;
        *var_tdiff0_2_dn5_slot = var_tdiff0_2_dn5;
        *var_tdiff0_2_dn6_slot = var_tdiff0_2_dn6;
        *var_tdiff0_2_dn7_slot = var_tdiff0_2_dn7;
        *var_tdiff0_2_dn8_slot = var_tdiff0_2_dn8;
        *var_tdiff0_2_dn9_slot = var_tdiff0_2_dn9;
        *var_tdiff0_2_rv_slot = var_tdiff0_2_rv;
        *var_tdiff0_dn0_slot = var_tdiff0_dn0;
        *var_tdiff0_dn10_slot = var_tdiff0_dn10;
        *var_tdiff0_dn13_slot = var_tdiff0_dn13;
        *var_tdiff0_dn2_slot = var_tdiff0_dn2;
        *var_tdiff0_dn4_slot = var_tdiff0_dn4;
        *var_tdiff0_dn5_slot = var_tdiff0_dn5;
        *var_tdiff0_dn6_slot = var_tdiff0_dn6;
        *var_tdiff0_dn7_slot = var_tdiff0_dn7;
        *var_tdiff0_dn8_slot = var_tdiff0_dn8;
        *var_tdiff0_dn9_slot = var_tdiff0_dn9;
        *var_tdiff0_rv_slot = var_tdiff0_rv;
        *var_tdiff_2_slot = var_tdiff_2;
        *var_tdiff_2_dn0_slot = var_tdiff_2_dn0;
        *var_tdiff_2_dn10_slot = var_tdiff_2_dn10;
        *var_tdiff_2_dn13_slot = var_tdiff_2_dn13;
        *var_tdiff_2_dn2_slot = var_tdiff_2_dn2;
        *var_tdiff_2_dn4_slot = var_tdiff_2_dn4;
        *var_tdiff_2_dn5_slot = var_tdiff_2_dn5;
        *var_tdiff_2_dn6_slot = var_tdiff_2_dn6;
        *var_tdiff_2_dn7_slot = var_tdiff_2_dn7;
        *var_tdiff_2_dn8_slot = var_tdiff_2_dn8;
        *var_tdiff_2_dn9_slot = var_tdiff_2_dn9;
        *var_tdiff_2_rv_slot = var_tdiff_2_rv;
        *var_tdiff_dn0_slot = var_tdiff_dn0;
        *var_tdiff_dn10_slot = var_tdiff_dn10;
        *var_tdiff_dn13_slot = var_tdiff_dn13;
        *var_tdiff_dn2_slot = var_tdiff_dn2;
        *var_tdiff_dn4_slot = var_tdiff_dn4;
        *var_tdiff_dn5_slot = var_tdiff_dn5;
        *var_tdiff_dn6_slot = var_tdiff_dn6;
        *var_tdiff_dn7_slot = var_tdiff_dn7;
        *var_tdiff_dn8_slot = var_tdiff_dn8;
        *var_tdiff_dn9_slot = var_tdiff_dn9;
        *var_tdiff_rv_slot = var_tdiff_rv;
        *var_ttemp_slot = var_ttemp;
        *var_ttemp0_slot = var_ttemp0;
        *var_ttemp0_dn0_slot = var_ttemp0_dn0;
        *var_ttemp0_dn10_slot = var_ttemp0_dn10;
        *var_ttemp0_dn13_slot = var_ttemp0_dn13;
        *var_ttemp0_dn2_slot = var_ttemp0_dn2;
        *var_ttemp0_dn4_slot = var_ttemp0_dn4;
        *var_ttemp0_dn5_slot = var_ttemp0_dn5;
        *var_ttemp0_dn6_slot = var_ttemp0_dn6;
        *var_ttemp0_dn7_slot = var_ttemp0_dn7;
        *var_ttemp0_dn8_slot = var_ttemp0_dn8;
        *var_ttemp0_dn9_slot = var_ttemp0_dn9;
        *var_ttemp0_rv_slot = var_ttemp0_rv;
        *var_ttemp_dn0_slot = var_ttemp_dn0;
        *var_ttemp_dn10_slot = var_ttemp_dn10;
        *var_ttemp_dn13_slot = var_ttemp_dn13;
        *var_ttemp_dn2_slot = var_ttemp_dn2;
        *var_ttemp_dn4_slot = var_ttemp_dn4;
        *var_ttemp_dn5_slot = var_ttemp_dn5;
        *var_ttemp_dn6_slot = var_ttemp_dn6;
        *var_ttemp_dn7_slot = var_ttemp_dn7;
        *var_ttemp_dn8_slot = var_ttemp_dn8;
        *var_ttemp_dn9_slot = var_ttemp_dn9;
        *var_ttemp_rv_slot = var_ttemp_rv;
        *var_vbsegmt_slot = var_vbsegmt;
        *var_vbsegmt_dn2_slot = var_vbsegmt_dn2;
        *var_vbsegmt_dn8_slot = var_vbsegmt_dn8;
        *var_vbsegmt_rv_slot = var_vbsegmt_rv;
        *var_vbserev_slot = var_vbserev;
        *var_vbserev_dn0_slot = var_vbserev_dn0;
        *var_vbserev_dn2_slot = var_vbserev_dn2;
        *var_vbserev_dn8_slot = var_vbserev_dn8;
        *var_vbserev_rv_slot = var_vbserev_rv;
        *var_vbserevz_slot = var_vbserevz;
        *var_vbserevz_dn0_slot = var_vbserevz_dn0;
        *var_vbserevz_dn10_slot = var_vbserevz_dn10;
        *var_vbserevz_dn13_slot = var_vbserevz_dn13;
        *var_vbserevz_dn2_slot = var_vbserevz_dn2;
        *var_vbserevz_dn4_slot = var_vbserevz_dn4;
        *var_vbserevz_dn5_slot = var_vbserevz_dn5;
        *var_vbserevz_dn6_slot = var_vbserevz_dn6;
        *var_vbserevz_dn7_slot = var_vbserevz_dn7;
        *var_vbserevz_dn8_slot = var_vbserevz_dn8;
        *var_vbserevz_dn9_slot = var_vbserevz_dn9;
        *var_vbserevz_rv_slot = var_vbserevz_rv;
        *var_vbsz2_slot = var_vbsz2;
        *var_vbsz2_dn0_slot = var_vbsz2_dn0;
        *var_vbsz2_dn10_slot = var_vbsz2_dn10;
        *var_vbsz2_dn13_slot = var_vbsz2_dn13;
        *var_vbsz2_dn2_slot = var_vbsz2_dn2;
        *var_vbsz2_dn4_slot = var_vbsz2_dn4;
        *var_vbsz2_dn5_slot = var_vbsz2_dn5;
        *var_vbsz2_dn6_slot = var_vbsz2_dn6;
        *var_vbsz2_dn7_slot = var_vbsz2_dn7;
        *var_vbsz2_dn8_slot = var_vbsz2_dn8;
        *var_vbsz2_dn9_slot = var_vbsz2_dn9;
        *var_vbsz2_rv_slot = var_vbsz2_rv;
        *var_vdse_eff_slot = var_vdse_eff;
        *var_vdse_eff_dn0_slot = var_vdse_eff_dn0;
        *var_vdse_eff_dn2_slot = var_vdse_eff_dn2;
        *var_vdse_eff_rv_slot = var_vdse_eff_rv;
        *var_vdseff_slot = var_vdseff;
        *var_vdseff_dn0_slot = var_vdseff_dn0;
        *var_vdseff_dn10_slot = var_vdseff_dn10;
        *var_vdseff_dn13_slot = var_vdseff_dn13;
        *var_vdseff_dn2_slot = var_vdseff_dn2;
        *var_vdseff_dn4_slot = var_vdseff_dn4;
        *var_vdseff_dn5_slot = var_vdseff_dn5;
        *var_vdseff_dn6_slot = var_vdseff_dn6;
        *var_vdseff_dn7_slot = var_vdseff_dn7;
        *var_vdseff_dn8_slot = var_vdseff_dn8;
        *var_vdseff_dn9_slot = var_vdseff_dn9;
        *var_vdseff_rv_slot = var_vdseff_rv;
        *var_vdsegmt_slot = var_vdsegmt;
        *var_vdsegmt_dn0_slot = var_vdsegmt_dn0;
        *var_vdsegmt_dn2_slot = var_vdsegmt_dn2;
        *var_vdsegmt_rv_slot = var_vdsegmt_rv;
        *var_vdsemodenml_slot = var_vdsemodenml;
        *var_vdsemodenml_rv_slot = var_vdsemodenml_rv;
        *var_vdsemodervs_slot = var_vdsemodervs;
        *var_vdsemodervs_rv_slot = var_vdsemodervs_rv;
        *var_vdserev_slot = var_vdserev;
        *var_vdserev_dn0_slot = var_vdserev_dn0;
        *var_vdserev_dn2_slot = var_vdserev_dn2;
        *var_vdserev_rv_slot = var_vdserev_rv;
        *var_vdserevz_slot = var_vdserevz;
        *var_vdserevz_dn0_slot = var_vdserevz_dn0;
        *var_vdserevz_dn10_slot = var_vdserevz_dn10;
        *var_vdserevz_dn13_slot = var_vdserevz_dn13;
        *var_vdserevz_dn2_slot = var_vdserevz_dn2;
        *var_vdserevz_dn4_slot = var_vdserevz_dn4;
        *var_vdserevz_dn5_slot = var_vdserevz_dn5;
        *var_vdserevz_dn6_slot = var_vdserevz_dn6;
        *var_vdserevz_dn7_slot = var_vdserevz_dn7;
        *var_vdserevz_dn8_slot = var_vdserevz_dn8;
        *var_vdserevz_dn9_slot = var_vdserevz_dn9;
        *var_vdserevz_rv_slot = var_vdserevz_rv;
        *var_vdsorg_slot = var_vdsorg;
        *var_vdsorg_dn0_slot = var_vdsorg_dn0;
        *var_vdsorg_dn10_slot = var_vdsorg_dn10;
        *var_vdsorg_dn13_slot = var_vdsorg_dn13;
        *var_vdsorg_dn2_slot = var_vdsorg_dn2;
        *var_vdsorg_dn4_slot = var_vdsorg_dn4;
        *var_vdsorg_dn5_slot = var_vdsorg_dn5;
        *var_vdsorg_dn6_slot = var_vdsorg_dn6;
        *var_vdsorg_dn7_slot = var_vdsorg_dn7;
        *var_vdsorg_dn8_slot = var_vdsorg_dn8;
        *var_vdsorg_dn9_slot = var_vdsorg_dn9;
        *var_vdsorg_rv_slot = var_vdsorg_rv;
        *var_vgbgmt_slot = var_vgbgmt;
        *var_vgbgmt_dn2_slot = var_vgbgmt_dn2;
        *var_vgbgmt_dn6_slot = var_vgbgmt_dn6;
        *var_vgbgmt_dn7_slot = var_vgbgmt_dn7;
        *var_vgbgmt_dn8_slot = var_vgbgmt_dn8;
        *var_vgbgmt_rv_slot = var_vgbgmt_rv;
        *var_vgpld_slot = var_vgpld;
        *var_vgpld_dn2_slot = var_vgpld_dn2;
        *var_vgpld_dn6_slot = var_vgpld_dn6;
        *var_vgpld_dn7_slot = var_vgpld_dn7;
        *var_vgpld_dn8_slot = var_vgpld_dn8;
        *var_vgpld_rv_slot = var_vgpld_rv;
        *var_vgsegmt_slot = var_vgsegmt;
        *var_vgsegmt_dn2_slot = var_vgsegmt_dn2;
        *var_vgsegmt_dn6_slot = var_vgsegmt_dn6;
        *var_vgsegmt_rv_slot = var_vgsegmt_rv;
        *var_vgserev_slot = var_vgserev;
        *var_vgserev_dn0_slot = var_vgserev_dn0;
        *var_vgserev_dn2_slot = var_vgserev_dn2;
        *var_vgserev_dn6_slot = var_vgserev_dn6;
        *var_vgserev_rv_slot = var_vgserev_rv;
        *var_vgserevz_slot = var_vgserevz;
        *var_vgserevz_dn0_slot = var_vgserevz_dn0;
        *var_vgserevz_dn10_slot = var_vgserevz_dn10;
        *var_vgserevz_dn13_slot = var_vgserevz_dn13;
        *var_vgserevz_dn2_slot = var_vgserevz_dn2;
        *var_vgserevz_dn4_slot = var_vgserevz_dn4;
        *var_vgserevz_dn5_slot = var_vgserevz_dn5;
        *var_vgserevz_dn6_slot = var_vgserevz_dn6;
        *var_vgserevz_dn7_slot = var_vgserevz_dn7;
        *var_vgserevz_dn8_slot = var_vgserevz_dn8;
        *var_vgserevz_dn9_slot = var_vgserevz_dn9;
        *var_vgserevz_rv_slot = var_vgserevz_rv;
        *var_vsubsrev_slot = var_vsubsrev;
        *var_vsubsrev_dn0_slot = var_vsubsrev_dn0;
        *var_vsubsrev_dn2_slot = var_vsubsrev_dn2;
        *var_vsubsrev_rv_slot = var_vsubsrev_rv;
        *var_vxbgmt_slot = var_vxbgmt;
        *var_vxbgmt_dn0_slot = var_vxbgmt_dn0;
        *var_vxbgmt_dn10_slot = var_vxbgmt_dn10;
        *var_vxbgmt_dn13_slot = var_vxbgmt_dn13;
        *var_vxbgmt_dn2_slot = var_vxbgmt_dn2;
        *var_vxbgmt_dn4_slot = var_vxbgmt_dn4;
        *var_vxbgmt_dn5_slot = var_vxbgmt_dn5;
        *var_vxbgmt_dn6_slot = var_vxbgmt_dn6;
        *var_vxbgmt_dn7_slot = var_vxbgmt_dn7;
        *var_vxbgmt_dn8_slot = var_vxbgmt_dn8;
        *var_vxbgmt_dn9_slot = var_vxbgmt_dn9;
        *var_vxbgmt_rv_slot = var_vxbgmt_rv;
        *var_vxbgmtcl_slot = var_vxbgmtcl;
        *var_vxbgmtcl_dn0_slot = var_vxbgmtcl_dn0;
        *var_vxbgmtcl_dn10_slot = var_vxbgmtcl_dn10;
        *var_vxbgmtcl_dn13_slot = var_vxbgmtcl_dn13;
        *var_vxbgmtcl_dn2_slot = var_vxbgmtcl_dn2;
        *var_vxbgmtcl_dn4_slot = var_vxbgmtcl_dn4;
        *var_vxbgmtcl_dn5_slot = var_vxbgmtcl_dn5;
        *var_vxbgmtcl_dn6_slot = var_vxbgmtcl_dn6;
        *var_vxbgmtcl_dn7_slot = var_vxbgmtcl_dn7;
        *var_vxbgmtcl_dn8_slot = var_vxbgmtcl_dn8;
        *var_vxbgmtcl_dn9_slot = var_vxbgmtcl_dn9;
        *var_vxbgmtcl_rv_slot = var_vxbgmtcl_rv;
    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn13_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_car_slot: &mut f64,
        var_car_rv_slot: &mut f64,
        var_carr_slot: &mut f64,
        var_carr_dn0_slot: &mut f64,
        var_carr_dn10_slot: &mut f64,
        var_carr_dn13_slot: &mut f64,
        var_carr_dn2_slot: &mut f64,
        var_carr_dn4_slot: &mut f64,
        var_carr_dn5_slot: &mut f64,
        var_carr_dn6_slot: &mut f64,
        var_carr_dn7_slot: &mut f64,
        var_carr_dn8_slot: &mut f64,
        var_carr_dn9_slot: &mut f64,
        var_carr_rv_slot: &mut f64,
        var_chi_1_slot: &mut f64,
        var_chi_1_dn0_slot: &mut f64,
        var_chi_1_dn10_slot: &mut f64,
        var_chi_1_dn13_slot: &mut f64,
        var_chi_1_dn2_slot: &mut f64,
        var_chi_1_dn4_slot: &mut f64,
        var_chi_1_dn5_slot: &mut f64,
        var_chi_1_dn6_slot: &mut f64,
        var_chi_1_dn7_slot: &mut f64,
        var_chi_1_dn8_slot: &mut f64,
        var_chi_1_dn9_slot: &mut f64,
        var_chi_1_rv_slot: &mut f64,
        var_cnst0over_func_slot: &mut f64,
        var_cnst0over_func_dn0_slot: &mut f64,
        var_cnst0over_func_dn10_slot: &mut f64,
        var_cnst0over_func_dn13_slot: &mut f64,
        var_cnst0over_func_dn2_slot: &mut f64,
        var_cnst0over_func_dn4_slot: &mut f64,
        var_cnst0over_func_dn5_slot: &mut f64,
        var_cnst0over_func_dn6_slot: &mut f64,
        var_cnst0over_func_dn7_slot: &mut f64,
        var_cnst0over_func_dn8_slot: &mut f64,
        var_cnst0over_func_dn9_slot: &mut f64,
        var_cnst0over_func_rv_slot: &mut f64,
        var_cnst1over_slot: &mut f64,
        var_cnst1over_dn0_slot: &mut f64,
        var_cnst1over_dn10_slot: &mut f64,
        var_cnst1over_dn13_slot: &mut f64,
        var_cnst1over_dn2_slot: &mut f64,
        var_cnst1over_dn4_slot: &mut f64,
        var_cnst1over_dn5_slot: &mut f64,
        var_cnst1over_dn6_slot: &mut f64,
        var_cnst1over_dn7_slot: &mut f64,
        var_cnst1over_dn8_slot: &mut f64,
        var_cnst1over_dn9_slot: &mut f64,
        var_cnst1over_rv_slot: &mut f64,
        var_cx_slot: &mut f64,
        var_cx_rv_slot: &mut f64,
        var_ddriftld_slot: &mut f64,
        var_ddriftld_dn0_slot: &mut f64,
        var_ddriftld_dn10_slot: &mut f64,
        var_ddriftld_dn13_slot: &mut f64,
        var_ddriftld_dn2_slot: &mut f64,
        var_ddriftld_dn4_slot: &mut f64,
        var_ddriftld_dn5_slot: &mut f64,
        var_ddriftld_dn6_slot: &mut f64,
        var_ddriftld_dn7_slot: &mut f64,
        var_ddriftld_dn8_slot: &mut f64,
        var_ddriftld_dn9_slot: &mut f64,
        var_ddriftld_rv_slot: &mut f64,
        var_ddriftldc_slot: &mut f64,
        var_ddriftldc_dn0_slot: &mut f64,
        var_ddriftldc_dn10_slot: &mut f64,
        var_ddriftldc_dn13_slot: &mut f64,
        var_ddriftldc_dn2_slot: &mut f64,
        var_ddriftldc_dn4_slot: &mut f64,
        var_ddriftldc_dn5_slot: &mut f64,
        var_ddriftldc_dn6_slot: &mut f64,
        var_ddriftldc_dn7_slot: &mut f64,
        var_ddriftldc_dn8_slot: &mut f64,
        var_ddriftldc_dn9_slot: &mut f64,
        var_ddriftldc_rv_slot: &mut f64,
        var_deltemp_slot: &mut f64,
        var_deltemp_dn0_slot: &mut f64,
        var_deltemp_dn10_slot: &mut f64,
        var_deltemp_dn13_slot: &mut f64,
        var_deltemp_dn2_slot: &mut f64,
        var_deltemp_dn4_slot: &mut f64,
        var_deltemp_dn5_slot: &mut f64,
        var_deltemp_dn6_slot: &mut f64,
        var_deltemp_dn7_slot: &mut f64,
        var_deltemp_dn8_slot: &mut f64,
        var_deltemp_dn9_slot: &mut f64,
        var_deltemp_rv_slot: &mut f64,
        var_dl_slot: &mut f64,
        var_dl_rv_slot: &mut f64,
        var_dlld_slot: &mut f64,
        var_dlld_rv_slot: &mut f64,
        var_dw_slot: &mut f64,
        var_dw_rv_slot: &mut f64,
        var_dwcv_slot: &mut f64,
        var_dwcv_rv_slot: &mut f64,
        var_dwld_slot: &mut f64,
        var_dwld_rv_slot: &mut f64,
        var_edri_slot: &mut f64,
        var_edri_dn0_slot: &mut f64,
        var_edri_dn10_slot: &mut f64,
        var_edri_dn13_slot: &mut f64,
        var_edri_dn2_slot: &mut f64,
        var_edri_dn4_slot: &mut f64,
        var_edri_dn5_slot: &mut f64,
        var_edri_dn6_slot: &mut f64,
        var_edri_dn7_slot: &mut f64,
        var_edri_dn8_slot: &mut f64,
        var_edri_dn9_slot: &mut f64,
        var_edri_rv_slot: &mut f64,
        var_gd_slot: &mut f64,
        var_gd_dn0_slot: &mut f64,
        var_gd_dn10_slot: &mut f64,
        var_gd_dn13_slot: &mut f64,
        var_gd_dn2_slot: &mut f64,
        var_gd_dn4_slot: &mut f64,
        var_gd_dn5_slot: &mut f64,
        var_gd_dn6_slot: &mut f64,
        var_gd_dn7_slot: &mut f64,
        var_gd_dn8_slot: &mut f64,
        var_gd_dn9_slot: &mut f64,
        var_gd_rv_slot: &mut f64,
        var_gth_slot: &mut f64,
        var_gth_dn0_slot: &mut f64,
        var_gth_dn10_slot: &mut f64,
        var_gth_dn13_slot: &mut f64,
        var_gth_dn2_slot: &mut f64,
        var_gth_dn4_slot: &mut f64,
        var_gth_dn5_slot: &mut f64,
        var_gth_dn6_slot: &mut f64,
        var_gth_dn7_slot: &mut f64,
        var_gth_dn8_slot: &mut f64,
        var_gth_dn9_slot: &mut f64,
        var_gth_rv_slot: &mut f64,
        var_lg_slot: &mut f64,
        var_lg_rv_slot: &mut f64,
        var_lgate_slot: &mut f64,
        var_lgate_rv_slot: &mut f64,
        var_lod_half_slot: &mut f64,
        var_lod_half_dn0_slot: &mut f64,
        var_lod_half_dn10_slot: &mut f64,
        var_lod_half_dn13_slot: &mut f64,
        var_lod_half_dn2_slot: &mut f64,
        var_lod_half_dn4_slot: &mut f64,
        var_lod_half_dn5_slot: &mut f64,
        var_lod_half_dn6_slot: &mut f64,
        var_lod_half_dn7_slot: &mut f64,
        var_lod_half_dn8_slot: &mut f64,
        var_lod_half_dn9_slot: &mut f64,
        var_lod_half_ref_slot: &mut f64,
        var_lod_half_ref_dn0_slot: &mut f64,
        var_lod_half_ref_dn10_slot: &mut f64,
        var_lod_half_ref_dn13_slot: &mut f64,
        var_lod_half_ref_dn2_slot: &mut f64,
        var_lod_half_ref_dn4_slot: &mut f64,
        var_lod_half_ref_dn5_slot: &mut f64,
        var_lod_half_ref_dn6_slot: &mut f64,
        var_lod_half_ref_dn7_slot: &mut f64,
        var_lod_half_ref_dn8_slot: &mut f64,
        var_lod_half_ref_dn9_slot: &mut f64,
        var_lod_half_ref_rv_slot: &mut f64,
        var_lod_half_rv_slot: &mut f64,
        var_log_tratio_slot: &mut f64,
        var_log_tratio_dn0_slot: &mut f64,
        var_log_tratio_dn10_slot: &mut f64,
        var_log_tratio_dn13_slot: &mut f64,
        var_log_tratio_dn2_slot: &mut f64,
        var_log_tratio_dn4_slot: &mut f64,
        var_log_tratio_dn5_slot: &mut f64,
        var_log_tratio_dn6_slot: &mut f64,
        var_log_tratio_dn7_slot: &mut f64,
        var_log_tratio_dn8_slot: &mut f64,
        var_log_tratio_dn9_slot: &mut f64,
        var_log_tratio_rv_slot: &mut f64,
        var_mu0_slot: &mut f64,
        var_mu0_dn0_slot: &mut f64,
        var_mu0_dn10_slot: &mut f64,
        var_mu0_dn13_slot: &mut f64,
        var_mu0_dn2_slot: &mut f64,
        var_mu0_dn4_slot: &mut f64,
        var_mu0_dn5_slot: &mut f64,
        var_mu0_dn6_slot: &mut f64,
        var_mu0_dn7_slot: &mut f64,
        var_mu0_dn8_slot: &mut f64,
        var_mu0_dn9_slot: &mut f64,
        var_mu0_rv_slot: &mut f64,
        var_mueph_slot: &mut f64,
        var_mueph_dn0_slot: &mut f64,
        var_mueph_dn10_slot: &mut f64,
        var_mueph_dn13_slot: &mut f64,
        var_mueph_dn2_slot: &mut f64,
        var_mueph_dn4_slot: &mut f64,
        var_mueph_dn5_slot: &mut f64,
        var_mueph_dn6_slot: &mut f64,
        var_mueph_dn7_slot: &mut f64,
        var_mueph_dn8_slot: &mut f64,
        var_mueph_dn9_slot: &mut f64,
        var_mueph_rv_slot: &mut f64,
        var_nover_func_slot: &mut f64,
        var_nover_func_rv_slot: &mut f64,
        var_nsub_slot: &mut f64,
        var_nsub_dn0_slot: &mut f64,
        var_nsub_dn10_slot: &mut f64,
        var_nsub_dn13_slot: &mut f64,
        var_nsub_dn2_slot: &mut f64,
        var_nsub_dn4_slot: &mut f64,
        var_nsub_dn5_slot: &mut f64,
        var_nsub_dn6_slot: &mut f64,
        var_nsub_dn7_slot: &mut f64,
        var_nsub_dn8_slot: &mut f64,
        var_nsub_dn9_slot: &mut f64,
        var_nsub_rv_slot: &mut f64,
        var_nsubb_slot: &mut f64,
        var_nsubb_dn0_slot: &mut f64,
        var_nsubb_dn10_slot: &mut f64,
        var_nsubb_dn13_slot: &mut f64,
        var_nsubb_dn2_slot: &mut f64,
        var_nsubb_dn4_slot: &mut f64,
        var_nsubb_dn5_slot: &mut f64,
        var_nsubb_dn6_slot: &mut f64,
        var_nsubb_dn7_slot: &mut f64,
        var_nsubb_dn8_slot: &mut f64,
        var_nsubb_dn9_slot: &mut f64,
        var_nsubb_rv_slot: &mut f64,
        var_nsubpp_slot: &mut f64,
        var_nsubpp_dn0_slot: &mut f64,
        var_nsubpp_dn10_slot: &mut f64,
        var_nsubpp_dn13_slot: &mut f64,
        var_nsubpp_dn2_slot: &mut f64,
        var_nsubpp_dn4_slot: &mut f64,
        var_nsubpp_dn5_slot: &mut f64,
        var_nsubpp_dn6_slot: &mut f64,
        var_nsubpp_dn7_slot: &mut f64,
        var_nsubpp_dn8_slot: &mut f64,
        var_nsubpp_dn9_slot: &mut f64,
        var_nsubpp_rv_slot: &mut f64,
        var_nsubps_slot: &mut f64,
        var_nsubps_dn0_slot: &mut f64,
        var_nsubps_dn10_slot: &mut f64,
        var_nsubps_dn13_slot: &mut f64,
        var_nsubps_dn2_slot: &mut f64,
        var_nsubps_dn4_slot: &mut f64,
        var_nsubps_dn5_slot: &mut f64,
        var_nsubps_dn6_slot: &mut f64,
        var_nsubps_dn7_slot: &mut f64,
        var_nsubps_dn8_slot: &mut f64,
        var_nsubps_dn9_slot: &mut f64,
        var_nsubps_rv_slot: &mut f64,
        var_p_slot: &mut f64,
        var_p_dn0_slot: &mut f64,
        var_p_dn10_slot: &mut f64,
        var_p_dn13_slot: &mut f64,
        var_p_dn2_slot: &mut f64,
        var_p_dn4_slot: &mut f64,
        var_p_dn5_slot: &mut f64,
        var_p_dn6_slot: &mut f64,
        var_p_dn7_slot: &mut f64,
        var_p_dn8_slot: &mut f64,
        var_p_dn9_slot: &mut f64,
        var_p_rv_slot: &mut f64,
        var_ps0ld_slot: &mut f64,
        var_ps0ld_dn0_slot: &mut f64,
        var_ps0ld_dn10_slot: &mut f64,
        var_ps0ld_dn13_slot: &mut f64,
        var_ps0ld_dn2_slot: &mut f64,
        var_ps0ld_dn4_slot: &mut f64,
        var_ps0ld_dn5_slot: &mut f64,
        var_ps0ld_dn6_slot: &mut f64,
        var_ps0ld_dn7_slot: &mut f64,
        var_ps0ld_dn8_slot: &mut f64,
        var_ps0ld_dn9_slot: &mut f64,
        var_ps0ld_rv_slot: &mut f64,
        var_qb_nqs_slot: &mut f64,
        var_qb_nqs_dn12_slot: &mut f64,
        var_qb_nqs_rv_slot: &mut f64,
        var_qd_nqs_slot: &mut f64,
        var_qd_nqs_dn0_slot: &mut f64,
        var_qd_nqs_dn10_slot: &mut f64,
        var_qd_nqs_dn11_slot: &mut f64,
        var_qd_nqs_dn13_slot: &mut f64,
        var_qd_nqs_dn2_slot: &mut f64,
        var_qd_nqs_dn4_slot: &mut f64,
        var_qd_nqs_dn5_slot: &mut f64,
        var_qd_nqs_dn6_slot: &mut f64,
        var_qd_nqs_dn7_slot: &mut f64,
        var_qd_nqs_dn8_slot: &mut f64,
        var_qd_nqs_dn9_slot: &mut f64,
        var_qd_nqs_rv_slot: &mut f64,
        var_qg_slot: &mut f64,
        var_qg_dn0_slot: &mut f64,
        var_qg_dn10_slot: &mut f64,
        var_qg_dn13_slot: &mut f64,
        var_qg_dn2_slot: &mut f64,
        var_qg_dn4_slot: &mut f64,
        var_qg_dn5_slot: &mut f64,
        var_qg_dn6_slot: &mut f64,
        var_qg_dn7_slot: &mut f64,
        var_qg_dn8_slot: &mut f64,
        var_qg_dn9_slot: &mut f64,
        var_qg_nqs_slot: &mut f64,
        var_qg_nqs_dn11_slot: &mut f64,
        var_qg_nqs_dn12_slot: &mut f64,
        var_qg_nqs_rv_slot: &mut f64,
        var_qg_rv_slot: &mut f64,
        var_qi_nqs_slot: &mut f64,
        var_qi_nqs_dn11_slot: &mut f64,
        var_qi_nqs_rv_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn0_slot: &mut f64,
        var_qs_dn10_slot: &mut f64,
        var_qs_dn13_slot: &mut f64,
        var_qs_dn2_slot: &mut f64,
        var_qs_dn4_slot: &mut f64,
        var_qs_dn5_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_qs_dn9_slot: &mut f64,
        var_qs_nqs_slot: &mut f64,
        var_qs_nqs_dn0_slot: &mut f64,
        var_qs_nqs_dn10_slot: &mut f64,
        var_qs_nqs_dn11_slot: &mut f64,
        var_qs_nqs_dn13_slot: &mut f64,
        var_qs_nqs_dn2_slot: &mut f64,
        var_qs_nqs_dn4_slot: &mut f64,
        var_qs_nqs_dn5_slot: &mut f64,
        var_qs_nqs_dn6_slot: &mut f64,
        var_qs_nqs_dn7_slot: &mut f64,
        var_qs_nqs_dn8_slot: &mut f64,
        var_qs_nqs_dn9_slot: &mut f64,
        var_qs_nqs_rv_slot: &mut f64,
        var_qs_rv_slot: &mut f64,
        var_ta_slot: &mut f64,
        var_ta_rv_slot: &mut f64,
        var_tb_slot: &mut f64,
        var_tb_rv_slot: &mut f64,
        var_vbd_slot: &mut f64,
        var_vbd_dn5_slot: &mut f64,
        var_vbd_dn7_slot: &mut f64,
        var_vbd_dn8_slot: &mut f64,
        var_vbd_rv_slot: &mut f64,
        var_vbsei_slot: &mut f64,
        var_vbsei_dn2_slot: &mut f64,
        var_vbsei_dn8_slot: &mut f64,
        var_vbsei_rv_slot: &mut f64,
        var_vbsi_slot: &mut f64,
        var_vbsi_dn7_slot: &mut f64,
        var_vbsi_dn8_slot: &mut f64,
        var_vbsi_rv_slot: &mut f64,
        var_vddpz_slot: &mut f64,
        var_vddpz_dn0_slot: &mut f64,
        var_vddpz_dn10_slot: &mut f64,
        var_vddpz_dn13_slot: &mut f64,
        var_vddpz_dn2_slot: &mut f64,
        var_vddpz_dn4_slot: &mut f64,
        var_vddpz_dn5_slot: &mut f64,
        var_vddpz_dn6_slot: &mut f64,
        var_vddpz_dn7_slot: &mut f64,
        var_vddpz_dn8_slot: &mut f64,
        var_vddpz_dn9_slot: &mut f64,
        var_vddpz_rv_slot: &mut f64,
        var_vdri_slot: &mut f64,
        var_vdri_dn0_slot: &mut f64,
        var_vdri_dn10_slot: &mut f64,
        var_vdri_dn13_slot: &mut f64,
        var_vdri_dn2_slot: &mut f64,
        var_vdri_dn4_slot: &mut f64,
        var_vdri_dn5_slot: &mut f64,
        var_vdri_dn6_slot: &mut f64,
        var_vdri_dn7_slot: &mut f64,
        var_vdri_dn8_slot: &mut f64,
        var_vdri_dn9_slot: &mut f64,
        var_vdri_rv_slot: &mut f64,
        var_vdsei_slot: &mut f64,
        var_vdsei_dn0_slot: &mut f64,
        var_vdsei_dn2_slot: &mut f64,
        var_vdsei_rv_slot: &mut f64,
        var_vdsi_slot: &mut f64,
        var_vdsi_dn5_slot: &mut f64,
        var_vdsi_dn7_slot: &mut f64,
        var_vdsi_rv_slot: &mut f64,
        var_veffpower_slot: &mut f64,
        var_veffpower_dn0_slot: &mut f64,
        var_veffpower_dn10_slot: &mut f64,
        var_veffpower_dn13_slot: &mut f64,
        var_veffpower_dn2_slot: &mut f64,
        var_veffpower_dn4_slot: &mut f64,
        var_veffpower_dn5_slot: &mut f64,
        var_veffpower_dn6_slot: &mut f64,
        var_veffpower_dn7_slot: &mut f64,
        var_veffpower_dn8_slot: &mut f64,
        var_veffpower_dn9_slot: &mut f64,
        var_veffpower_rv_slot: &mut f64,
        var_vgb_fb_ld_slot: &mut f64,
        var_vgb_fb_ld_dn0_slot: &mut f64,
        var_vgb_fb_ld_dn10_slot: &mut f64,
        var_vgb_fb_ld_dn13_slot: &mut f64,
        var_vgb_fb_ld_dn2_slot: &mut f64,
        var_vgb_fb_ld_dn4_slot: &mut f64,
        var_vgb_fb_ld_dn5_slot: &mut f64,
        var_vgb_fb_ld_dn6_slot: &mut f64,
        var_vgb_fb_ld_dn7_slot: &mut f64,
        var_vgb_fb_ld_dn8_slot: &mut f64,
        var_vgb_fb_ld_dn9_slot: &mut f64,
        var_vgb_fb_ld_rv_slot: &mut f64,
        var_vgd_slot: &mut f64,
        var_vgd_dn5_slot: &mut f64,
        var_vgd_dn6_slot: &mut f64,
        var_vgd_dn7_slot: &mut f64,
        var_vgd_rv_slot: &mut f64,
        var_vgsei_slot: &mut f64,
        var_vgsei_dn2_slot: &mut f64,
        var_vgsei_dn6_slot: &mut f64,
        var_vgsei_rv_slot: &mut f64,
        var_vgsi_slot: &mut f64,
        var_vgsi_dn6_slot: &mut f64,
        var_vgsi_dn7_slot: &mut f64,
        var_vgsi_rv_slot: &mut f64,
        var_wg_slot: &mut f64,
        var_wg_rv_slot: &mut f64,
        var_wgate_slot: &mut f64,
        var_wgate_rv_slot: &mut f64,
        var_wlg_slot: &mut f64,
        var_wlg_rv_slot: &mut f64,
        var_xov_slot: &mut f64,
        var_xov_dn0_slot: &mut f64,
        var_xov_dn10_slot: &mut f64,
        var_xov_dn13_slot: &mut f64,
        var_xov_dn2_slot: &mut f64,
        var_xov_dn4_slot: &mut f64,
        var_xov_dn5_slot: &mut f64,
        var_xov_dn6_slot: &mut f64,
        var_xov_dn7_slot: &mut f64,
        var_xov_dn8_slot: &mut f64,
        var_xov_dn9_slot: &mut f64,
        var_xov_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn13: f64 = *var_arg_dn13_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_car: f64 = *var_car_slot;
        let mut var_car_rv: f64 = *var_car_rv_slot;
        let mut var_carr: f64 = *var_carr_slot;
        let mut var_carr_dn0: f64 = *var_carr_dn0_slot;
        let mut var_carr_dn10: f64 = *var_carr_dn10_slot;
        let mut var_carr_dn13: f64 = *var_carr_dn13_slot;
        let mut var_carr_dn2: f64 = *var_carr_dn2_slot;
        let mut var_carr_dn4: f64 = *var_carr_dn4_slot;
        let mut var_carr_dn5: f64 = *var_carr_dn5_slot;
        let mut var_carr_dn6: f64 = *var_carr_dn6_slot;
        let mut var_carr_dn7: f64 = *var_carr_dn7_slot;
        let mut var_carr_dn8: f64 = *var_carr_dn8_slot;
        let mut var_carr_dn9: f64 = *var_carr_dn9_slot;
        let mut var_carr_rv: f64 = *var_carr_rv_slot;
        let mut var_chi_1: f64 = *var_chi_1_slot;
        let mut var_chi_1_dn0: f64 = *var_chi_1_dn0_slot;
        let mut var_chi_1_dn10: f64 = *var_chi_1_dn10_slot;
        let mut var_chi_1_dn13: f64 = *var_chi_1_dn13_slot;
        let mut var_chi_1_dn2: f64 = *var_chi_1_dn2_slot;
        let mut var_chi_1_dn4: f64 = *var_chi_1_dn4_slot;
        let mut var_chi_1_dn5: f64 = *var_chi_1_dn5_slot;
        let mut var_chi_1_dn6: f64 = *var_chi_1_dn6_slot;
        let mut var_chi_1_dn7: f64 = *var_chi_1_dn7_slot;
        let mut var_chi_1_dn8: f64 = *var_chi_1_dn8_slot;
        let mut var_chi_1_dn9: f64 = *var_chi_1_dn9_slot;
        let mut var_chi_1_rv: f64 = *var_chi_1_rv_slot;
        let mut var_cnst0over_func: f64 = *var_cnst0over_func_slot;
        let mut var_cnst0over_func_dn0: f64 = *var_cnst0over_func_dn0_slot;
        let mut var_cnst0over_func_dn10: f64 = *var_cnst0over_func_dn10_slot;
        let mut var_cnst0over_func_dn13: f64 = *var_cnst0over_func_dn13_slot;
        let mut var_cnst0over_func_dn2: f64 = *var_cnst0over_func_dn2_slot;
        let mut var_cnst0over_func_dn4: f64 = *var_cnst0over_func_dn4_slot;
        let mut var_cnst0over_func_dn5: f64 = *var_cnst0over_func_dn5_slot;
        let mut var_cnst0over_func_dn6: f64 = *var_cnst0over_func_dn6_slot;
        let mut var_cnst0over_func_dn7: f64 = *var_cnst0over_func_dn7_slot;
        let mut var_cnst0over_func_dn8: f64 = *var_cnst0over_func_dn8_slot;
        let mut var_cnst0over_func_dn9: f64 = *var_cnst0over_func_dn9_slot;
        let mut var_cnst0over_func_rv: f64 = *var_cnst0over_func_rv_slot;
        let mut var_cnst1over: f64 = *var_cnst1over_slot;
        let mut var_cnst1over_dn0: f64 = *var_cnst1over_dn0_slot;
        let mut var_cnst1over_dn10: f64 = *var_cnst1over_dn10_slot;
        let mut var_cnst1over_dn13: f64 = *var_cnst1over_dn13_slot;
        let mut var_cnst1over_dn2: f64 = *var_cnst1over_dn2_slot;
        let mut var_cnst1over_dn4: f64 = *var_cnst1over_dn4_slot;
        let mut var_cnst1over_dn5: f64 = *var_cnst1over_dn5_slot;
        let mut var_cnst1over_dn6: f64 = *var_cnst1over_dn6_slot;
        let mut var_cnst1over_dn7: f64 = *var_cnst1over_dn7_slot;
        let mut var_cnst1over_dn8: f64 = *var_cnst1over_dn8_slot;
        let mut var_cnst1over_dn9: f64 = *var_cnst1over_dn9_slot;
        let mut var_cnst1over_rv: f64 = *var_cnst1over_rv_slot;
        let mut var_cx: f64 = *var_cx_slot;
        let mut var_cx_rv: f64 = *var_cx_rv_slot;
        let mut var_ddriftld: f64 = *var_ddriftld_slot;
        let mut var_ddriftld_dn0: f64 = *var_ddriftld_dn0_slot;
        let mut var_ddriftld_dn10: f64 = *var_ddriftld_dn10_slot;
        let mut var_ddriftld_dn13: f64 = *var_ddriftld_dn13_slot;
        let mut var_ddriftld_dn2: f64 = *var_ddriftld_dn2_slot;
        let mut var_ddriftld_dn4: f64 = *var_ddriftld_dn4_slot;
        let mut var_ddriftld_dn5: f64 = *var_ddriftld_dn5_slot;
        let mut var_ddriftld_dn6: f64 = *var_ddriftld_dn6_slot;
        let mut var_ddriftld_dn7: f64 = *var_ddriftld_dn7_slot;
        let mut var_ddriftld_dn8: f64 = *var_ddriftld_dn8_slot;
        let mut var_ddriftld_dn9: f64 = *var_ddriftld_dn9_slot;
        let mut var_ddriftld_rv: f64 = *var_ddriftld_rv_slot;
        let mut var_ddriftldc: f64 = *var_ddriftldc_slot;
        let mut var_ddriftldc_dn0: f64 = *var_ddriftldc_dn0_slot;
        let mut var_ddriftldc_dn10: f64 = *var_ddriftldc_dn10_slot;
        let mut var_ddriftldc_dn13: f64 = *var_ddriftldc_dn13_slot;
        let mut var_ddriftldc_dn2: f64 = *var_ddriftldc_dn2_slot;
        let mut var_ddriftldc_dn4: f64 = *var_ddriftldc_dn4_slot;
        let mut var_ddriftldc_dn5: f64 = *var_ddriftldc_dn5_slot;
        let mut var_ddriftldc_dn6: f64 = *var_ddriftldc_dn6_slot;
        let mut var_ddriftldc_dn7: f64 = *var_ddriftldc_dn7_slot;
        let mut var_ddriftldc_dn8: f64 = *var_ddriftldc_dn8_slot;
        let mut var_ddriftldc_dn9: f64 = *var_ddriftldc_dn9_slot;
        let mut var_ddriftldc_rv: f64 = *var_ddriftldc_rv_slot;
        let mut var_deltemp: f64 = *var_deltemp_slot;
        let mut var_deltemp_dn0: f64 = *var_deltemp_dn0_slot;
        let mut var_deltemp_dn10: f64 = *var_deltemp_dn10_slot;
        let mut var_deltemp_dn13: f64 = *var_deltemp_dn13_slot;
        let mut var_deltemp_dn2: f64 = *var_deltemp_dn2_slot;
        let mut var_deltemp_dn4: f64 = *var_deltemp_dn4_slot;
        let mut var_deltemp_dn5: f64 = *var_deltemp_dn5_slot;
        let mut var_deltemp_dn6: f64 = *var_deltemp_dn6_slot;
        let mut var_deltemp_dn7: f64 = *var_deltemp_dn7_slot;
        let mut var_deltemp_dn8: f64 = *var_deltemp_dn8_slot;
        let mut var_deltemp_dn9: f64 = *var_deltemp_dn9_slot;
        let mut var_deltemp_rv: f64 = *var_deltemp_rv_slot;
        let mut var_dl: f64 = *var_dl_slot;
        let mut var_dl_rv: f64 = *var_dl_rv_slot;
        let mut var_dlld: f64 = *var_dlld_slot;
        let mut var_dlld_rv: f64 = *var_dlld_rv_slot;
        let mut var_dw: f64 = *var_dw_slot;
        let mut var_dw_rv: f64 = *var_dw_rv_slot;
        let mut var_dwcv: f64 = *var_dwcv_slot;
        let mut var_dwcv_rv: f64 = *var_dwcv_rv_slot;
        let mut var_dwld: f64 = *var_dwld_slot;
        let mut var_dwld_rv: f64 = *var_dwld_rv_slot;
        let mut var_edri: f64 = *var_edri_slot;
        let mut var_edri_dn0: f64 = *var_edri_dn0_slot;
        let mut var_edri_dn10: f64 = *var_edri_dn10_slot;
        let mut var_edri_dn13: f64 = *var_edri_dn13_slot;
        let mut var_edri_dn2: f64 = *var_edri_dn2_slot;
        let mut var_edri_dn4: f64 = *var_edri_dn4_slot;
        let mut var_edri_dn5: f64 = *var_edri_dn5_slot;
        let mut var_edri_dn6: f64 = *var_edri_dn6_slot;
        let mut var_edri_dn7: f64 = *var_edri_dn7_slot;
        let mut var_edri_dn8: f64 = *var_edri_dn8_slot;
        let mut var_edri_dn9: f64 = *var_edri_dn9_slot;
        let mut var_edri_rv: f64 = *var_edri_rv_slot;
        let mut var_gd: f64 = *var_gd_slot;
        let mut var_gd_dn0: f64 = *var_gd_dn0_slot;
        let mut var_gd_dn10: f64 = *var_gd_dn10_slot;
        let mut var_gd_dn13: f64 = *var_gd_dn13_slot;
        let mut var_gd_dn2: f64 = *var_gd_dn2_slot;
        let mut var_gd_dn4: f64 = *var_gd_dn4_slot;
        let mut var_gd_dn5: f64 = *var_gd_dn5_slot;
        let mut var_gd_dn6: f64 = *var_gd_dn6_slot;
        let mut var_gd_dn7: f64 = *var_gd_dn7_slot;
        let mut var_gd_dn8: f64 = *var_gd_dn8_slot;
        let mut var_gd_dn9: f64 = *var_gd_dn9_slot;
        let mut var_gd_rv: f64 = *var_gd_rv_slot;
        let mut var_gth: f64 = *var_gth_slot;
        let mut var_gth_dn0: f64 = *var_gth_dn0_slot;
        let mut var_gth_dn10: f64 = *var_gth_dn10_slot;
        let mut var_gth_dn13: f64 = *var_gth_dn13_slot;
        let mut var_gth_dn2: f64 = *var_gth_dn2_slot;
        let mut var_gth_dn4: f64 = *var_gth_dn4_slot;
        let mut var_gth_dn5: f64 = *var_gth_dn5_slot;
        let mut var_gth_dn6: f64 = *var_gth_dn6_slot;
        let mut var_gth_dn7: f64 = *var_gth_dn7_slot;
        let mut var_gth_dn8: f64 = *var_gth_dn8_slot;
        let mut var_gth_dn9: f64 = *var_gth_dn9_slot;
        let mut var_gth_rv: f64 = *var_gth_rv_slot;
        let mut var_lg: f64 = *var_lg_slot;
        let mut var_lg_rv: f64 = *var_lg_rv_slot;
        let mut var_lgate: f64 = *var_lgate_slot;
        let mut var_lgate_rv: f64 = *var_lgate_rv_slot;
        let mut var_lod_half: f64 = *var_lod_half_slot;
        let mut var_lod_half_dn0: f64 = *var_lod_half_dn0_slot;
        let mut var_lod_half_dn10: f64 = *var_lod_half_dn10_slot;
        let mut var_lod_half_dn13: f64 = *var_lod_half_dn13_slot;
        let mut var_lod_half_dn2: f64 = *var_lod_half_dn2_slot;
        let mut var_lod_half_dn4: f64 = *var_lod_half_dn4_slot;
        let mut var_lod_half_dn5: f64 = *var_lod_half_dn5_slot;
        let mut var_lod_half_dn6: f64 = *var_lod_half_dn6_slot;
        let mut var_lod_half_dn7: f64 = *var_lod_half_dn7_slot;
        let mut var_lod_half_dn8: f64 = *var_lod_half_dn8_slot;
        let mut var_lod_half_dn9: f64 = *var_lod_half_dn9_slot;
        let mut var_lod_half_ref: f64 = *var_lod_half_ref_slot;
        let mut var_lod_half_ref_dn0: f64 = *var_lod_half_ref_dn0_slot;
        let mut var_lod_half_ref_dn10: f64 = *var_lod_half_ref_dn10_slot;
        let mut var_lod_half_ref_dn13: f64 = *var_lod_half_ref_dn13_slot;
        let mut var_lod_half_ref_dn2: f64 = *var_lod_half_ref_dn2_slot;
        let mut var_lod_half_ref_dn4: f64 = *var_lod_half_ref_dn4_slot;
        let mut var_lod_half_ref_dn5: f64 = *var_lod_half_ref_dn5_slot;
        let mut var_lod_half_ref_dn6: f64 = *var_lod_half_ref_dn6_slot;
        let mut var_lod_half_ref_dn7: f64 = *var_lod_half_ref_dn7_slot;
        let mut var_lod_half_ref_dn8: f64 = *var_lod_half_ref_dn8_slot;
        let mut var_lod_half_ref_dn9: f64 = *var_lod_half_ref_dn9_slot;
        let mut var_lod_half_ref_rv: f64 = *var_lod_half_ref_rv_slot;
        let mut var_lod_half_rv: f64 = *var_lod_half_rv_slot;
        let mut var_log_tratio: f64 = *var_log_tratio_slot;
        let mut var_log_tratio_dn0: f64 = *var_log_tratio_dn0_slot;
        let mut var_log_tratio_dn10: f64 = *var_log_tratio_dn10_slot;
        let mut var_log_tratio_dn13: f64 = *var_log_tratio_dn13_slot;
        let mut var_log_tratio_dn2: f64 = *var_log_tratio_dn2_slot;
        let mut var_log_tratio_dn4: f64 = *var_log_tratio_dn4_slot;
        let mut var_log_tratio_dn5: f64 = *var_log_tratio_dn5_slot;
        let mut var_log_tratio_dn6: f64 = *var_log_tratio_dn6_slot;
        let mut var_log_tratio_dn7: f64 = *var_log_tratio_dn7_slot;
        let mut var_log_tratio_dn8: f64 = *var_log_tratio_dn8_slot;
        let mut var_log_tratio_dn9: f64 = *var_log_tratio_dn9_slot;
        let mut var_log_tratio_rv: f64 = *var_log_tratio_rv_slot;
        let mut var_mu0: f64 = *var_mu0_slot;
        let mut var_mu0_dn0: f64 = *var_mu0_dn0_slot;
        let mut var_mu0_dn10: f64 = *var_mu0_dn10_slot;
        let mut var_mu0_dn13: f64 = *var_mu0_dn13_slot;
        let mut var_mu0_dn2: f64 = *var_mu0_dn2_slot;
        let mut var_mu0_dn4: f64 = *var_mu0_dn4_slot;
        let mut var_mu0_dn5: f64 = *var_mu0_dn5_slot;
        let mut var_mu0_dn6: f64 = *var_mu0_dn6_slot;
        let mut var_mu0_dn7: f64 = *var_mu0_dn7_slot;
        let mut var_mu0_dn8: f64 = *var_mu0_dn8_slot;
        let mut var_mu0_dn9: f64 = *var_mu0_dn9_slot;
        let mut var_mu0_rv: f64 = *var_mu0_rv_slot;
        let mut var_mueph: f64 = *var_mueph_slot;
        let mut var_mueph_dn0: f64 = *var_mueph_dn0_slot;
        let mut var_mueph_dn10: f64 = *var_mueph_dn10_slot;
        let mut var_mueph_dn13: f64 = *var_mueph_dn13_slot;
        let mut var_mueph_dn2: f64 = *var_mueph_dn2_slot;
        let mut var_mueph_dn4: f64 = *var_mueph_dn4_slot;
        let mut var_mueph_dn5: f64 = *var_mueph_dn5_slot;
        let mut var_mueph_dn6: f64 = *var_mueph_dn6_slot;
        let mut var_mueph_dn7: f64 = *var_mueph_dn7_slot;
        let mut var_mueph_dn8: f64 = *var_mueph_dn8_slot;
        let mut var_mueph_dn9: f64 = *var_mueph_dn9_slot;
        let mut var_mueph_rv: f64 = *var_mueph_rv_slot;
        let mut var_nover_func: f64 = *var_nover_func_slot;
        let mut var_nover_func_rv: f64 = *var_nover_func_rv_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
        let mut var_nsub_dn0: f64 = *var_nsub_dn0_slot;
        let mut var_nsub_dn10: f64 = *var_nsub_dn10_slot;
        let mut var_nsub_dn13: f64 = *var_nsub_dn13_slot;
        let mut var_nsub_dn2: f64 = *var_nsub_dn2_slot;
        let mut var_nsub_dn4: f64 = *var_nsub_dn4_slot;
        let mut var_nsub_dn5: f64 = *var_nsub_dn5_slot;
        let mut var_nsub_dn6: f64 = *var_nsub_dn6_slot;
        let mut var_nsub_dn7: f64 = *var_nsub_dn7_slot;
        let mut var_nsub_dn8: f64 = *var_nsub_dn8_slot;
        let mut var_nsub_dn9: f64 = *var_nsub_dn9_slot;
        let mut var_nsub_rv: f64 = *var_nsub_rv_slot;
        let mut var_nsubb: f64 = *var_nsubb_slot;
        let mut var_nsubb_dn0: f64 = *var_nsubb_dn0_slot;
        let mut var_nsubb_dn10: f64 = *var_nsubb_dn10_slot;
        let mut var_nsubb_dn13: f64 = *var_nsubb_dn13_slot;
        let mut var_nsubb_dn2: f64 = *var_nsubb_dn2_slot;
        let mut var_nsubb_dn4: f64 = *var_nsubb_dn4_slot;
        let mut var_nsubb_dn5: f64 = *var_nsubb_dn5_slot;
        let mut var_nsubb_dn6: f64 = *var_nsubb_dn6_slot;
        let mut var_nsubb_dn7: f64 = *var_nsubb_dn7_slot;
        let mut var_nsubb_dn8: f64 = *var_nsubb_dn8_slot;
        let mut var_nsubb_dn9: f64 = *var_nsubb_dn9_slot;
        let mut var_nsubb_rv: f64 = *var_nsubb_rv_slot;
        let mut var_nsubpp: f64 = *var_nsubpp_slot;
        let mut var_nsubpp_dn0: f64 = *var_nsubpp_dn0_slot;
        let mut var_nsubpp_dn10: f64 = *var_nsubpp_dn10_slot;
        let mut var_nsubpp_dn13: f64 = *var_nsubpp_dn13_slot;
        let mut var_nsubpp_dn2: f64 = *var_nsubpp_dn2_slot;
        let mut var_nsubpp_dn4: f64 = *var_nsubpp_dn4_slot;
        let mut var_nsubpp_dn5: f64 = *var_nsubpp_dn5_slot;
        let mut var_nsubpp_dn6: f64 = *var_nsubpp_dn6_slot;
        let mut var_nsubpp_dn7: f64 = *var_nsubpp_dn7_slot;
        let mut var_nsubpp_dn8: f64 = *var_nsubpp_dn8_slot;
        let mut var_nsubpp_dn9: f64 = *var_nsubpp_dn9_slot;
        let mut var_nsubpp_rv: f64 = *var_nsubpp_rv_slot;
        let mut var_nsubps: f64 = *var_nsubps_slot;
        let mut var_nsubps_dn0: f64 = *var_nsubps_dn0_slot;
        let mut var_nsubps_dn10: f64 = *var_nsubps_dn10_slot;
        let mut var_nsubps_dn13: f64 = *var_nsubps_dn13_slot;
        let mut var_nsubps_dn2: f64 = *var_nsubps_dn2_slot;
        let mut var_nsubps_dn4: f64 = *var_nsubps_dn4_slot;
        let mut var_nsubps_dn5: f64 = *var_nsubps_dn5_slot;
        let mut var_nsubps_dn6: f64 = *var_nsubps_dn6_slot;
        let mut var_nsubps_dn7: f64 = *var_nsubps_dn7_slot;
        let mut var_nsubps_dn8: f64 = *var_nsubps_dn8_slot;
        let mut var_nsubps_dn9: f64 = *var_nsubps_dn9_slot;
        let mut var_nsubps_rv: f64 = *var_nsubps_rv_slot;
        let mut var_p: f64 = *var_p_slot;
        let mut var_p_dn0: f64 = *var_p_dn0_slot;
        let mut var_p_dn10: f64 = *var_p_dn10_slot;
        let mut var_p_dn13: f64 = *var_p_dn13_slot;
        let mut var_p_dn2: f64 = *var_p_dn2_slot;
        let mut var_p_dn4: f64 = *var_p_dn4_slot;
        let mut var_p_dn5: f64 = *var_p_dn5_slot;
        let mut var_p_dn6: f64 = *var_p_dn6_slot;
        let mut var_p_dn7: f64 = *var_p_dn7_slot;
        let mut var_p_dn8: f64 = *var_p_dn8_slot;
        let mut var_p_dn9: f64 = *var_p_dn9_slot;
        let mut var_p_rv: f64 = *var_p_rv_slot;
        let mut var_ps0ld: f64 = *var_ps0ld_slot;
        let mut var_ps0ld_dn0: f64 = *var_ps0ld_dn0_slot;
        let mut var_ps0ld_dn10: f64 = *var_ps0ld_dn10_slot;
        let mut var_ps0ld_dn13: f64 = *var_ps0ld_dn13_slot;
        let mut var_ps0ld_dn2: f64 = *var_ps0ld_dn2_slot;
        let mut var_ps0ld_dn4: f64 = *var_ps0ld_dn4_slot;
        let mut var_ps0ld_dn5: f64 = *var_ps0ld_dn5_slot;
        let mut var_ps0ld_dn6: f64 = *var_ps0ld_dn6_slot;
        let mut var_ps0ld_dn7: f64 = *var_ps0ld_dn7_slot;
        let mut var_ps0ld_dn8: f64 = *var_ps0ld_dn8_slot;
        let mut var_ps0ld_dn9: f64 = *var_ps0ld_dn9_slot;
        let mut var_ps0ld_rv: f64 = *var_ps0ld_rv_slot;
        let mut var_qb_nqs: f64 = *var_qb_nqs_slot;
        let mut var_qb_nqs_dn12: f64 = *var_qb_nqs_dn12_slot;
        let mut var_qb_nqs_rv: f64 = *var_qb_nqs_rv_slot;
        let mut var_qd_nqs: f64 = *var_qd_nqs_slot;
        let mut var_qd_nqs_dn0: f64 = *var_qd_nqs_dn0_slot;
        let mut var_qd_nqs_dn10: f64 = *var_qd_nqs_dn10_slot;
        let mut var_qd_nqs_dn11: f64 = *var_qd_nqs_dn11_slot;
        let mut var_qd_nqs_dn13: f64 = *var_qd_nqs_dn13_slot;
        let mut var_qd_nqs_dn2: f64 = *var_qd_nqs_dn2_slot;
        let mut var_qd_nqs_dn4: f64 = *var_qd_nqs_dn4_slot;
        let mut var_qd_nqs_dn5: f64 = *var_qd_nqs_dn5_slot;
        let mut var_qd_nqs_dn6: f64 = *var_qd_nqs_dn6_slot;
        let mut var_qd_nqs_dn7: f64 = *var_qd_nqs_dn7_slot;
        let mut var_qd_nqs_dn8: f64 = *var_qd_nqs_dn8_slot;
        let mut var_qd_nqs_dn9: f64 = *var_qd_nqs_dn9_slot;
        let mut var_qd_nqs_rv: f64 = *var_qd_nqs_rv_slot;
        let mut var_qg: f64 = *var_qg_slot;
        let mut var_qg_dn0: f64 = *var_qg_dn0_slot;
        let mut var_qg_dn10: f64 = *var_qg_dn10_slot;
        let mut var_qg_dn13: f64 = *var_qg_dn13_slot;
        let mut var_qg_dn2: f64 = *var_qg_dn2_slot;
        let mut var_qg_dn4: f64 = *var_qg_dn4_slot;
        let mut var_qg_dn5: f64 = *var_qg_dn5_slot;
        let mut var_qg_dn6: f64 = *var_qg_dn6_slot;
        let mut var_qg_dn7: f64 = *var_qg_dn7_slot;
        let mut var_qg_dn8: f64 = *var_qg_dn8_slot;
        let mut var_qg_dn9: f64 = *var_qg_dn9_slot;
        let mut var_qg_nqs: f64 = *var_qg_nqs_slot;
        let mut var_qg_nqs_dn11: f64 = *var_qg_nqs_dn11_slot;
        let mut var_qg_nqs_dn12: f64 = *var_qg_nqs_dn12_slot;
        let mut var_qg_nqs_rv: f64 = *var_qg_nqs_rv_slot;
        let mut var_qg_rv: f64 = *var_qg_rv_slot;
        let mut var_qi_nqs: f64 = *var_qi_nqs_slot;
        let mut var_qi_nqs_dn11: f64 = *var_qi_nqs_dn11_slot;
        let mut var_qi_nqs_rv: f64 = *var_qi_nqs_rv_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn0: f64 = *var_qs_dn0_slot;
        let mut var_qs_dn10: f64 = *var_qs_dn10_slot;
        let mut var_qs_dn13: f64 = *var_qs_dn13_slot;
        let mut var_qs_dn2: f64 = *var_qs_dn2_slot;
        let mut var_qs_dn4: f64 = *var_qs_dn4_slot;
        let mut var_qs_dn5: f64 = *var_qs_dn5_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_qs_dn9: f64 = *var_qs_dn9_slot;
        let mut var_qs_nqs: f64 = *var_qs_nqs_slot;
        let mut var_qs_nqs_dn0: f64 = *var_qs_nqs_dn0_slot;
        let mut var_qs_nqs_dn10: f64 = *var_qs_nqs_dn10_slot;
        let mut var_qs_nqs_dn11: f64 = *var_qs_nqs_dn11_slot;
        let mut var_qs_nqs_dn13: f64 = *var_qs_nqs_dn13_slot;
        let mut var_qs_nqs_dn2: f64 = *var_qs_nqs_dn2_slot;
        let mut var_qs_nqs_dn4: f64 = *var_qs_nqs_dn4_slot;
        let mut var_qs_nqs_dn5: f64 = *var_qs_nqs_dn5_slot;
        let mut var_qs_nqs_dn6: f64 = *var_qs_nqs_dn6_slot;
        let mut var_qs_nqs_dn7: f64 = *var_qs_nqs_dn7_slot;
        let mut var_qs_nqs_dn8: f64 = *var_qs_nqs_dn8_slot;
        let mut var_qs_nqs_dn9: f64 = *var_qs_nqs_dn9_slot;
        let mut var_qs_nqs_rv: f64 = *var_qs_nqs_rv_slot;
        let mut var_qs_rv: f64 = *var_qs_rv_slot;
        let mut var_ta: f64 = *var_ta_slot;
        let mut var_ta_rv: f64 = *var_ta_rv_slot;
        let mut var_tb: f64 = *var_tb_slot;
        let mut var_tb_rv: f64 = *var_tb_rv_slot;
        let mut var_vbd: f64 = *var_vbd_slot;
        let mut var_vbd_dn5: f64 = *var_vbd_dn5_slot;
        let mut var_vbd_dn7: f64 = *var_vbd_dn7_slot;
        let mut var_vbd_dn8: f64 = *var_vbd_dn8_slot;
        let mut var_vbd_rv: f64 = *var_vbd_rv_slot;
        let mut var_vbsei: f64 = *var_vbsei_slot;
        let mut var_vbsei_dn2: f64 = *var_vbsei_dn2_slot;
        let mut var_vbsei_dn8: f64 = *var_vbsei_dn8_slot;
        let mut var_vbsei_rv: f64 = *var_vbsei_rv_slot;
        let mut var_vbsi: f64 = *var_vbsi_slot;
        let mut var_vbsi_dn7: f64 = *var_vbsi_dn7_slot;
        let mut var_vbsi_dn8: f64 = *var_vbsi_dn8_slot;
        let mut var_vbsi_rv: f64 = *var_vbsi_rv_slot;
        let mut var_vddpz: f64 = *var_vddpz_slot;
        let mut var_vddpz_dn0: f64 = *var_vddpz_dn0_slot;
        let mut var_vddpz_dn10: f64 = *var_vddpz_dn10_slot;
        let mut var_vddpz_dn13: f64 = *var_vddpz_dn13_slot;
        let mut var_vddpz_dn2: f64 = *var_vddpz_dn2_slot;
        let mut var_vddpz_dn4: f64 = *var_vddpz_dn4_slot;
        let mut var_vddpz_dn5: f64 = *var_vddpz_dn5_slot;
        let mut var_vddpz_dn6: f64 = *var_vddpz_dn6_slot;
        let mut var_vddpz_dn7: f64 = *var_vddpz_dn7_slot;
        let mut var_vddpz_dn8: f64 = *var_vddpz_dn8_slot;
        let mut var_vddpz_dn9: f64 = *var_vddpz_dn9_slot;
        let mut var_vddpz_rv: f64 = *var_vddpz_rv_slot;
        let mut var_vdri: f64 = *var_vdri_slot;
        let mut var_vdri_dn0: f64 = *var_vdri_dn0_slot;
        let mut var_vdri_dn10: f64 = *var_vdri_dn10_slot;
        let mut var_vdri_dn13: f64 = *var_vdri_dn13_slot;
        let mut var_vdri_dn2: f64 = *var_vdri_dn2_slot;
        let mut var_vdri_dn4: f64 = *var_vdri_dn4_slot;
        let mut var_vdri_dn5: f64 = *var_vdri_dn5_slot;
        let mut var_vdri_dn6: f64 = *var_vdri_dn6_slot;
        let mut var_vdri_dn7: f64 = *var_vdri_dn7_slot;
        let mut var_vdri_dn8: f64 = *var_vdri_dn8_slot;
        let mut var_vdri_dn9: f64 = *var_vdri_dn9_slot;
        let mut var_vdri_rv: f64 = *var_vdri_rv_slot;
        let mut var_vdsei: f64 = *var_vdsei_slot;
        let mut var_vdsei_dn0: f64 = *var_vdsei_dn0_slot;
        let mut var_vdsei_dn2: f64 = *var_vdsei_dn2_slot;
        let mut var_vdsei_rv: f64 = *var_vdsei_rv_slot;
        let mut var_vdsi: f64 = *var_vdsi_slot;
        let mut var_vdsi_dn5: f64 = *var_vdsi_dn5_slot;
        let mut var_vdsi_dn7: f64 = *var_vdsi_dn7_slot;
        let mut var_vdsi_rv: f64 = *var_vdsi_rv_slot;
        let mut var_veffpower: f64 = *var_veffpower_slot;
        let mut var_veffpower_dn0: f64 = *var_veffpower_dn0_slot;
        let mut var_veffpower_dn10: f64 = *var_veffpower_dn10_slot;
        let mut var_veffpower_dn13: f64 = *var_veffpower_dn13_slot;
        let mut var_veffpower_dn2: f64 = *var_veffpower_dn2_slot;
        let mut var_veffpower_dn4: f64 = *var_veffpower_dn4_slot;
        let mut var_veffpower_dn5: f64 = *var_veffpower_dn5_slot;
        let mut var_veffpower_dn6: f64 = *var_veffpower_dn6_slot;
        let mut var_veffpower_dn7: f64 = *var_veffpower_dn7_slot;
        let mut var_veffpower_dn8: f64 = *var_veffpower_dn8_slot;
        let mut var_veffpower_dn9: f64 = *var_veffpower_dn9_slot;
        let mut var_veffpower_rv: f64 = *var_veffpower_rv_slot;
        let mut var_vgb_fb_ld: f64 = *var_vgb_fb_ld_slot;
        let mut var_vgb_fb_ld_dn0: f64 = *var_vgb_fb_ld_dn0_slot;
        let mut var_vgb_fb_ld_dn10: f64 = *var_vgb_fb_ld_dn10_slot;
        let mut var_vgb_fb_ld_dn13: f64 = *var_vgb_fb_ld_dn13_slot;
        let mut var_vgb_fb_ld_dn2: f64 = *var_vgb_fb_ld_dn2_slot;
        let mut var_vgb_fb_ld_dn4: f64 = *var_vgb_fb_ld_dn4_slot;
        let mut var_vgb_fb_ld_dn5: f64 = *var_vgb_fb_ld_dn5_slot;
        let mut var_vgb_fb_ld_dn6: f64 = *var_vgb_fb_ld_dn6_slot;
        let mut var_vgb_fb_ld_dn7: f64 = *var_vgb_fb_ld_dn7_slot;
        let mut var_vgb_fb_ld_dn8: f64 = *var_vgb_fb_ld_dn8_slot;
        let mut var_vgb_fb_ld_dn9: f64 = *var_vgb_fb_ld_dn9_slot;
        let mut var_vgb_fb_ld_rv: f64 = *var_vgb_fb_ld_rv_slot;
        let mut var_vgd: f64 = *var_vgd_slot;
        let mut var_vgd_dn5: f64 = *var_vgd_dn5_slot;
        let mut var_vgd_dn6: f64 = *var_vgd_dn6_slot;
        let mut var_vgd_dn7: f64 = *var_vgd_dn7_slot;
        let mut var_vgd_rv: f64 = *var_vgd_rv_slot;
        let mut var_vgsei: f64 = *var_vgsei_slot;
        let mut var_vgsei_dn2: f64 = *var_vgsei_dn2_slot;
        let mut var_vgsei_dn6: f64 = *var_vgsei_dn6_slot;
        let mut var_vgsei_rv: f64 = *var_vgsei_rv_slot;
        let mut var_vgsi: f64 = *var_vgsi_slot;
        let mut var_vgsi_dn6: f64 = *var_vgsi_dn6_slot;
        let mut var_vgsi_dn7: f64 = *var_vgsi_dn7_slot;
        let mut var_vgsi_rv: f64 = *var_vgsi_rv_slot;
        let mut var_wg: f64 = *var_wg_slot;
        let mut var_wg_rv: f64 = *var_wg_rv_slot;
        let mut var_wgate: f64 = *var_wgate_slot;
        let mut var_wgate_rv: f64 = *var_wgate_rv_slot;
        let mut var_wlg: f64 = *var_wlg_slot;
        let mut var_wlg_rv: f64 = *var_wlg_rv_slot;
        let mut var_xov: f64 = *var_xov_slot;
        let mut var_xov_dn0: f64 = *var_xov_dn0_slot;
        let mut var_xov_dn10: f64 = *var_xov_dn10_slot;
        let mut var_xov_dn13: f64 = *var_xov_dn13_slot;
        let mut var_xov_dn2: f64 = *var_xov_dn2_slot;
        let mut var_xov_dn4: f64 = *var_xov_dn4_slot;
        let mut var_xov_dn5: f64 = *var_xov_dn5_slot;
        let mut var_xov_dn6: f64 = *var_xov_dn6_slot;
        let mut var_xov_dn7: f64 = *var_xov_dn7_slot;
        let mut var_xov_dn8: f64 = *var_xov_dn8_slot;
        let mut var_xov_dn9: f64 = *var_xov_dn9_slot;
        let mut var_xov_rv: f64 = *var_xov_rv_slot;

        var_vgb_fb_ld = 0.0;
        var_vgb_fb_ld_dn0 = 0.0;
        var_vgb_fb_ld_dn2 = 0.0;
        var_vgb_fb_ld_dn4 = 0.0;
        var_vgb_fb_ld_dn5 = 0.0;
        var_vgb_fb_ld_dn6 = 0.0;
        var_vgb_fb_ld_dn7 = 0.0;
        var_vgb_fb_ld_dn8 = 0.0;
        var_vgb_fb_ld_dn9 = 0.0;
        var_vgb_fb_ld_dn10 = 0.0;
        var_vgb_fb_ld_dn13 = 0.0;
        var_vgb_fb_ld_rv = 0.0;

        var_ps0ld = 0.0;
        var_ps0ld_dn0 = 0.0;
        var_ps0ld_dn2 = 0.0;
        var_ps0ld_dn4 = 0.0;
        var_ps0ld_dn5 = 0.0;
        var_ps0ld_dn6 = 0.0;
        var_ps0ld_dn7 = 0.0;
        var_ps0ld_dn8 = 0.0;
        var_ps0ld_dn9 = 0.0;
        var_ps0ld_dn10 = 0.0;
        var_ps0ld_dn13 = 0.0;
        var_ps0ld_rv = 0.0;

        var_cnst1over = 0.0;
        var_cnst1over_dn0 = 0.0;
        var_cnst1over_dn2 = 0.0;
        var_cnst1over_dn4 = 0.0;
        var_cnst1over_dn5 = 0.0;
        var_cnst1over_dn6 = 0.0;
        var_cnst1over_dn7 = 0.0;
        var_cnst1over_dn8 = 0.0;
        var_cnst1over_dn9 = 0.0;
        var_cnst1over_dn10 = 0.0;
        var_cnst1over_dn13 = 0.0;
        var_cnst1over_rv = 0.0;

        var_ddriftld = p.p334;
        var_ddriftld_dn0 = 0.0;
        var_ddriftld_dn2 = 0.0;
        var_ddriftld_dn4 = 0.0;
        var_ddriftld_dn5 = 0.0;
        var_ddriftld_dn6 = 0.0;
        var_ddriftld_dn7 = 0.0;
        var_ddriftld_dn8 = 0.0;
        var_ddriftld_dn9 = 0.0;
        var_ddriftld_dn10 = 0.0;
        var_ddriftld_dn13 = 0.0;
        var_ddriftld_rv = 0.0;

        var_ddriftldc = p.p334;
        var_ddriftldc_dn0 = 0.0;
        var_ddriftldc_dn2 = 0.0;
        var_ddriftldc_dn4 = 0.0;
        var_ddriftldc_dn5 = 0.0;
        var_ddriftldc_dn6 = 0.0;
        var_ddriftldc_dn7 = 0.0;
        var_ddriftldc_dn8 = 0.0;
        var_ddriftldc_dn9 = 0.0;
        var_ddriftldc_dn10 = 0.0;
        var_ddriftldc_dn13 = 0.0;
        var_ddriftldc_rv = 0.0;

        var_nover_func = 0.0;
        var_nover_func_rv = 0.0;

        var_cnst0over_func = 0.0;
        var_cnst0over_func_dn0 = 0.0;
        var_cnst0over_func_dn2 = 0.0;
        var_cnst0over_func_dn4 = 0.0;
        var_cnst0over_func_dn5 = 0.0;
        var_cnst0over_func_dn6 = 0.0;
        var_cnst0over_func_dn7 = 0.0;
        var_cnst0over_func_dn8 = 0.0;
        var_cnst0over_func_dn9 = 0.0;
        var_cnst0over_func_dn10 = 0.0;
        var_cnst0over_func_dn13 = 0.0;
        var_cnst0over_func_rv = 0.0;

        var_ta = 0.0093868;
        var_ta_rv = 0.0;

        let assign3320_e1728: f64 = (-0.1047839);
        var_tb = assign3320_e1728;
        var_tb_rv = 0.0;

        var_chi_1 = 0.0;
        var_chi_1_dn0 = 0.0;
        var_chi_1_dn2 = 0.0;
        var_chi_1_dn4 = 0.0;
        var_chi_1_dn5 = 0.0;
        var_chi_1_dn6 = 0.0;
        var_chi_1_dn7 = 0.0;
        var_chi_1_dn8 = 0.0;
        var_chi_1_dn9 = 0.0;
        var_chi_1_dn10 = 0.0;
        var_chi_1_dn13 = 0.0;
        var_chi_1_rv = 0.0;

        var_mueph = 0.0;
        var_mueph_dn0 = 0.0;
        var_mueph_dn2 = 0.0;
        var_mueph_dn4 = 0.0;
        var_mueph_dn5 = 0.0;
        var_mueph_dn6 = 0.0;
        var_mueph_dn7 = 0.0;
        var_mueph_dn8 = 0.0;
        var_mueph_dn9 = 0.0;
        var_mueph_dn10 = 0.0;
        var_mueph_dn13 = 0.0;
        var_mueph_rv = 0.0;

        var_dl = 0.0;
        var_dl_rv = 0.0;

        var_dlld = 0.0;
        var_dlld_rv = 0.0;

        var_lg = 0.0;
        var_lg_rv = 0.0;

        var_dw = 0.0;
        var_dw_rv = 0.0;

        var_dwld = 0.0;
        var_dwld_rv = 0.0;

        var_dwcv = 0.0;
        var_dwcv_rv = 0.0;

        var_wg = 0.0;
        var_wg_rv = 0.0;

        var_wlg = 0.0;
        var_wlg_rv = 0.0;

        var_lgate = 0.0;
        var_lgate_rv = 0.0;

        var_wgate = 0.0;
        var_wgate_rv = 0.0;

        var_nsubpp = 0.0;
        var_nsubpp_dn0 = 0.0;
        var_nsubpp_dn2 = 0.0;
        var_nsubpp_dn4 = 0.0;
        var_nsubpp_dn5 = 0.0;
        var_nsubpp_dn6 = 0.0;
        var_nsubpp_dn7 = 0.0;
        var_nsubpp_dn8 = 0.0;
        var_nsubpp_dn9 = 0.0;
        var_nsubpp_dn10 = 0.0;
        var_nsubpp_dn13 = 0.0;
        var_nsubpp_rv = 0.0;

        var_nsubps = 0.0;
        var_nsubps_dn0 = 0.0;
        var_nsubps_dn2 = 0.0;
        var_nsubps_dn4 = 0.0;
        var_nsubps_dn5 = 0.0;
        var_nsubps_dn6 = 0.0;
        var_nsubps_dn7 = 0.0;
        var_nsubps_dn8 = 0.0;
        var_nsubps_dn9 = 0.0;
        var_nsubps_dn10 = 0.0;
        var_nsubps_dn13 = 0.0;
        var_nsubps_rv = 0.0;

        var_nsub = 0.0;
        var_nsub_dn0 = 0.0;
        var_nsub_dn2 = 0.0;
        var_nsub_dn4 = 0.0;
        var_nsub_dn5 = 0.0;
        var_nsub_dn6 = 0.0;
        var_nsub_dn7 = 0.0;
        var_nsub_dn8 = 0.0;
        var_nsub_dn9 = 0.0;
        var_nsub_dn10 = 0.0;
        var_nsub_dn13 = 0.0;
        var_nsub_rv = 0.0;

        var_nsubb = 0.0;
        var_nsubb_dn0 = 0.0;
        var_nsubb_dn2 = 0.0;
        var_nsubb_dn4 = 0.0;
        var_nsubb_dn5 = 0.0;
        var_nsubb_dn6 = 0.0;
        var_nsubb_dn7 = 0.0;
        var_nsubb_dn8 = 0.0;
        var_nsubb_dn9 = 0.0;
        var_nsubb_dn10 = 0.0;
        var_nsubb_dn13 = 0.0;
        var_nsubb_rv = 0.0;

        var_lod_half = 0.0;
        var_lod_half_dn0 = 0.0;
        var_lod_half_dn2 = 0.0;
        var_lod_half_dn4 = 0.0;
        var_lod_half_dn5 = 0.0;
        var_lod_half_dn6 = 0.0;
        var_lod_half_dn7 = 0.0;
        var_lod_half_dn8 = 0.0;
        var_lod_half_dn9 = 0.0;
        var_lod_half_dn10 = 0.0;
        var_lod_half_dn13 = 0.0;
        var_lod_half_rv = 0.0;

        var_lod_half_ref = 0.0;
        var_lod_half_ref_dn0 = 0.0;
        var_lod_half_ref_dn2 = 0.0;
        var_lod_half_ref_dn4 = 0.0;
        var_lod_half_ref_dn5 = 0.0;
        var_lod_half_ref_dn6 = 0.0;
        var_lod_half_ref_dn7 = 0.0;
        var_lod_half_ref_dn8 = 0.0;
        var_lod_half_ref_dn9 = 0.0;
        var_lod_half_ref_dn10 = 0.0;
        var_lod_half_ref_dn13 = 0.0;
        var_lod_half_ref_rv = 0.0;

        var_log_tratio = 0.0;
        var_log_tratio_dn0 = 0.0;
        var_log_tratio_dn2 = 0.0;
        var_log_tratio_dn4 = 0.0;
        var_log_tratio_dn5 = 0.0;
        var_log_tratio_dn6 = 0.0;
        var_log_tratio_dn7 = 0.0;
        var_log_tratio_dn8 = 0.0;
        var_log_tratio_dn9 = 0.0;
        var_log_tratio_dn10 = 0.0;
        var_log_tratio_dn13 = 0.0;
        var_log_tratio_rv = 0.0;

        var_edri = 0.0;
        var_edri_dn0 = 0.0;
        var_edri_dn2 = 0.0;
        var_edri_dn4 = 0.0;
        var_edri_dn5 = 0.0;
        var_edri_dn6 = 0.0;
        var_edri_dn7 = 0.0;
        var_edri_dn8 = 0.0;
        var_edri_dn9 = 0.0;
        var_edri_dn10 = 0.0;
        var_edri_dn13 = 0.0;
        var_edri_rv = 0.0;

        var_vdri = 0.0;
        var_vdri_dn0 = 0.0;
        var_vdri_dn2 = 0.0;
        var_vdri_dn4 = 0.0;
        var_vdri_dn5 = 0.0;
        var_vdri_dn6 = 0.0;
        var_vdri_dn7 = 0.0;
        var_vdri_dn8 = 0.0;
        var_vdri_dn9 = 0.0;
        var_vdri_dn10 = 0.0;
        var_vdri_dn13 = 0.0;
        var_vdri_rv = 0.0;

        var_mu0 = 0.0;
        var_mu0_dn0 = 0.0;
        var_mu0_dn2 = 0.0;
        var_mu0_dn4 = 0.0;
        var_mu0_dn5 = 0.0;
        var_mu0_dn6 = 0.0;
        var_mu0_dn7 = 0.0;
        var_mu0_dn8 = 0.0;
        var_mu0_dn9 = 0.0;
        var_mu0_dn10 = 0.0;
        var_mu0_dn13 = 0.0;
        var_mu0_rv = 0.0;

        var_cx = 0.0;
        var_cx_rv = 0.0;

        var_car = 0.0;
        var_car_rv = 0.0;

        var_xov = 0.0;
        var_xov_dn0 = 0.0;
        var_xov_dn2 = 0.0;
        var_xov_dn4 = 0.0;
        var_xov_dn5 = 0.0;
        var_xov_dn6 = 0.0;
        var_xov_dn7 = 0.0;
        var_xov_dn8 = 0.0;
        var_xov_dn9 = 0.0;
        var_xov_dn10 = 0.0;
        var_xov_dn13 = 0.0;
        var_xov_rv = 0.0;

        var_carr = 0.0;
        var_carr_dn0 = 0.0;
        var_carr_dn2 = 0.0;
        var_carr_dn4 = 0.0;
        var_carr_dn5 = 0.0;
        var_carr_dn6 = 0.0;
        var_carr_dn7 = 0.0;
        var_carr_dn8 = 0.0;
        var_carr_dn9 = 0.0;
        var_carr_dn10 = 0.0;
        var_carr_dn13 = 0.0;
        var_carr_rv = 0.0;

        var_gd = 0.0;
        var_gd_dn0 = 0.0;
        var_gd_dn2 = 0.0;
        var_gd_dn4 = 0.0;
        var_gd_dn5 = 0.0;
        var_gd_dn6 = 0.0;
        var_gd_dn7 = 0.0;
        var_gd_dn8 = 0.0;
        var_gd_dn9 = 0.0;
        var_gd_dn10 = 0.0;
        var_gd_dn13 = 0.0;
        var_gd_rv = 0.0;

        var_vddpz = 0.0;
        var_vddpz_dn0 = 0.0;
        var_vddpz_dn2 = 0.0;
        var_vddpz_dn4 = 0.0;
        var_vddpz_dn5 = 0.0;
        var_vddpz_dn6 = 0.0;
        var_vddpz_dn7 = 0.0;
        var_vddpz_dn8 = 0.0;
        var_vddpz_dn9 = 0.0;
        var_vddpz_dn10 = 0.0;
        var_vddpz_dn13 = 0.0;
        var_vddpz_rv = 0.0;

        var_arg = 0.0;
        var_arg_dn0 = 0.0;
        var_arg_dn2 = 0.0;
        var_arg_dn4 = 0.0;
        var_arg_dn5 = 0.0;
        var_arg_dn6 = 0.0;
        var_arg_dn7 = 0.0;
        var_arg_dn8 = 0.0;
        var_arg_dn9 = 0.0;
        var_arg_dn10 = 0.0;
        var_arg_dn13 = 0.0;
        var_arg_rv = 0.0;

        var_vbd = 0.0;
        var_vbd_dn5 = 0.0;
        var_vbd_dn7 = 0.0;
        var_vbd_dn8 = 0.0;
        var_vbd_rv = 0.0;

        var_vbsi = 0.0;
        var_vbsi_dn7 = 0.0;
        var_vbsi_dn8 = 0.0;
        var_vbsi_rv = 0.0;

        var_vdsi = 0.0;
        var_vdsi_dn5 = 0.0;
        var_vdsi_dn7 = 0.0;
        var_vdsi_rv = 0.0;

        var_vgd = 0.0;
        var_vgd_dn5 = 0.0;
        var_vgd_dn6 = 0.0;
        var_vgd_dn7 = 0.0;
        var_vgd_rv = 0.0;

        var_vgsi = 0.0;
        var_vgsi_dn6 = 0.0;
        var_vgsi_dn7 = 0.0;
        var_vgsi_rv = 0.0;

        var_deltemp = 0.0;
        var_deltemp_dn0 = 0.0;
        var_deltemp_dn2 = 0.0;
        var_deltemp_dn4 = 0.0;
        var_deltemp_dn5 = 0.0;
        var_deltemp_dn6 = 0.0;
        var_deltemp_dn7 = 0.0;
        var_deltemp_dn8 = 0.0;
        var_deltemp_dn9 = 0.0;
        var_deltemp_dn10 = 0.0;
        var_deltemp_dn13 = 0.0;
        var_deltemp_rv = 0.0;

        var_vdsei = 0.0;
        var_vdsei_dn0 = 0.0;
        var_vdsei_dn2 = 0.0;
        var_vdsei_rv = 0.0;

        var_vgsei = 0.0;
        var_vgsei_dn2 = 0.0;
        var_vgsei_dn6 = 0.0;
        var_vgsei_rv = 0.0;

        var_vbsei = 0.0;
        var_vbsei_dn2 = 0.0;
        var_vbsei_dn8 = 0.0;
        var_vbsei_rv = 0.0;

        var_gth = 0.0;
        var_gth_dn0 = 0.0;
        var_gth_dn2 = 0.0;
        var_gth_dn4 = 0.0;
        var_gth_dn5 = 0.0;
        var_gth_dn6 = 0.0;
        var_gth_dn7 = 0.0;
        var_gth_dn8 = 0.0;
        var_gth_dn9 = 0.0;
        var_gth_dn10 = 0.0;
        var_gth_dn13 = 0.0;
        var_gth_rv = 0.0;

        var_qg = 0.0;
        var_qg_dn0 = 0.0;
        var_qg_dn2 = 0.0;
        var_qg_dn4 = 0.0;
        var_qg_dn5 = 0.0;
        var_qg_dn6 = 0.0;
        var_qg_dn7 = 0.0;
        var_qg_dn8 = 0.0;
        var_qg_dn9 = 0.0;
        var_qg_dn10 = 0.0;
        var_qg_dn13 = 0.0;
        var_qg_rv = 0.0;

        var_qs = 0.0;
        var_qs_dn0 = 0.0;
        var_qs_dn2 = 0.0;
        var_qs_dn4 = 0.0;
        var_qs_dn5 = 0.0;
        var_qs_dn6 = 0.0;
        var_qs_dn7 = 0.0;
        var_qs_dn8 = 0.0;
        var_qs_dn9 = 0.0;
        var_qs_dn10 = 0.0;
        var_qs_dn13 = 0.0;
        var_qs_rv = 0.0;

        var_veffpower = 0.0;
        var_veffpower_dn0 = 0.0;
        var_veffpower_dn2 = 0.0;
        var_veffpower_dn4 = 0.0;
        var_veffpower_dn5 = 0.0;
        var_veffpower_dn6 = 0.0;
        var_veffpower_dn7 = 0.0;
        var_veffpower_dn8 = 0.0;
        var_veffpower_dn9 = 0.0;
        var_veffpower_dn10 = 0.0;
        var_veffpower_dn13 = 0.0;
        var_veffpower_rv = 0.0;

        var_p = 0.0;
        var_p_dn0 = 0.0;
        var_p_dn2 = 0.0;
        var_p_dn4 = 0.0;
        var_p_dn5 = 0.0;
        var_p_dn6 = 0.0;
        var_p_dn7 = 0.0;
        var_p_dn8 = 0.0;
        var_p_dn9 = 0.0;
        var_p_dn10 = 0.0;
        var_p_dn13 = 0.0;
        var_p_rv = 0.0;

        var_qi_nqs = 0.0;
        var_qi_nqs_dn11 = 0.0;
        var_qi_nqs_rv = 0.0;

        var_qb_nqs = 0.0;
        var_qb_nqs_dn12 = 0.0;
        var_qb_nqs_rv = 0.0;

        var_qd_nqs = 0.0;
        var_qd_nqs_dn0 = 0.0;
        var_qd_nqs_dn2 = 0.0;
        var_qd_nqs_dn4 = 0.0;
        var_qd_nqs_dn5 = 0.0;
        var_qd_nqs_dn6 = 0.0;
        var_qd_nqs_dn7 = 0.0;
        var_qd_nqs_dn8 = 0.0;
        var_qd_nqs_dn9 = 0.0;
        var_qd_nqs_dn10 = 0.0;
        var_qd_nqs_dn11 = 0.0;
        var_qd_nqs_dn13 = 0.0;
        var_qd_nqs_rv = 0.0;

        var_qs_nqs = 0.0;
        var_qs_nqs_dn0 = 0.0;
        var_qs_nqs_dn2 = 0.0;
        var_qs_nqs_dn4 = 0.0;
        var_qs_nqs_dn5 = 0.0;
        var_qs_nqs_dn6 = 0.0;
        var_qs_nqs_dn7 = 0.0;
        var_qs_nqs_dn8 = 0.0;
        var_qs_nqs_dn9 = 0.0;
        var_qs_nqs_dn10 = 0.0;
        var_qs_nqs_dn11 = 0.0;
        var_qs_nqs_dn13 = 0.0;
        var_qs_nqs_rv = 0.0;

        var_qg_nqs = 0.0;
        var_qg_nqs_dn11 = 0.0;
        var_qg_nqs_dn12 = 0.0;
        var_qg_nqs_rv = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn13_slot = var_arg_dn13;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_arg_rv_slot = var_arg_rv;
        *var_car_slot = var_car;
        *var_car_rv_slot = var_car_rv;
        *var_carr_slot = var_carr;
        *var_carr_dn0_slot = var_carr_dn0;
        *var_carr_dn10_slot = var_carr_dn10;
        *var_carr_dn13_slot = var_carr_dn13;
        *var_carr_dn2_slot = var_carr_dn2;
        *var_carr_dn4_slot = var_carr_dn4;
        *var_carr_dn5_slot = var_carr_dn5;
        *var_carr_dn6_slot = var_carr_dn6;
        *var_carr_dn7_slot = var_carr_dn7;
        *var_carr_dn8_slot = var_carr_dn8;
        *var_carr_dn9_slot = var_carr_dn9;
        *var_carr_rv_slot = var_carr_rv;
        *var_chi_1_slot = var_chi_1;
        *var_chi_1_dn0_slot = var_chi_1_dn0;
        *var_chi_1_dn10_slot = var_chi_1_dn10;
        *var_chi_1_dn13_slot = var_chi_1_dn13;
        *var_chi_1_dn2_slot = var_chi_1_dn2;
        *var_chi_1_dn4_slot = var_chi_1_dn4;
        *var_chi_1_dn5_slot = var_chi_1_dn5;
        *var_chi_1_dn6_slot = var_chi_1_dn6;
        *var_chi_1_dn7_slot = var_chi_1_dn7;
        *var_chi_1_dn8_slot = var_chi_1_dn8;
        *var_chi_1_dn9_slot = var_chi_1_dn9;
        *var_chi_1_rv_slot = var_chi_1_rv;
        *var_cnst0over_func_slot = var_cnst0over_func;
        *var_cnst0over_func_dn0_slot = var_cnst0over_func_dn0;
        *var_cnst0over_func_dn10_slot = var_cnst0over_func_dn10;
        *var_cnst0over_func_dn13_slot = var_cnst0over_func_dn13;
        *var_cnst0over_func_dn2_slot = var_cnst0over_func_dn2;
        *var_cnst0over_func_dn4_slot = var_cnst0over_func_dn4;
        *var_cnst0over_func_dn5_slot = var_cnst0over_func_dn5;
        *var_cnst0over_func_dn6_slot = var_cnst0over_func_dn6;
        *var_cnst0over_func_dn7_slot = var_cnst0over_func_dn7;
        *var_cnst0over_func_dn8_slot = var_cnst0over_func_dn8;
        *var_cnst0over_func_dn9_slot = var_cnst0over_func_dn9;
        *var_cnst0over_func_rv_slot = var_cnst0over_func_rv;
        *var_cnst1over_slot = var_cnst1over;
        *var_cnst1over_dn0_slot = var_cnst1over_dn0;
        *var_cnst1over_dn10_slot = var_cnst1over_dn10;
        *var_cnst1over_dn13_slot = var_cnst1over_dn13;
        *var_cnst1over_dn2_slot = var_cnst1over_dn2;
        *var_cnst1over_dn4_slot = var_cnst1over_dn4;
        *var_cnst1over_dn5_slot = var_cnst1over_dn5;
        *var_cnst1over_dn6_slot = var_cnst1over_dn6;
        *var_cnst1over_dn7_slot = var_cnst1over_dn7;
        *var_cnst1over_dn8_slot = var_cnst1over_dn8;
        *var_cnst1over_dn9_slot = var_cnst1over_dn9;
        *var_cnst1over_rv_slot = var_cnst1over_rv;
        *var_cx_slot = var_cx;
        *var_cx_rv_slot = var_cx_rv;
        *var_ddriftld_slot = var_ddriftld;
        *var_ddriftld_dn0_slot = var_ddriftld_dn0;
        *var_ddriftld_dn10_slot = var_ddriftld_dn10;
        *var_ddriftld_dn13_slot = var_ddriftld_dn13;
        *var_ddriftld_dn2_slot = var_ddriftld_dn2;
        *var_ddriftld_dn4_slot = var_ddriftld_dn4;
        *var_ddriftld_dn5_slot = var_ddriftld_dn5;
        *var_ddriftld_dn6_slot = var_ddriftld_dn6;
        *var_ddriftld_dn7_slot = var_ddriftld_dn7;
        *var_ddriftld_dn8_slot = var_ddriftld_dn8;
        *var_ddriftld_dn9_slot = var_ddriftld_dn9;
        *var_ddriftld_rv_slot = var_ddriftld_rv;
        *var_ddriftldc_slot = var_ddriftldc;
        *var_ddriftldc_dn0_slot = var_ddriftldc_dn0;
        *var_ddriftldc_dn10_slot = var_ddriftldc_dn10;
        *var_ddriftldc_dn13_slot = var_ddriftldc_dn13;
        *var_ddriftldc_dn2_slot = var_ddriftldc_dn2;
        *var_ddriftldc_dn4_slot = var_ddriftldc_dn4;
        *var_ddriftldc_dn5_slot = var_ddriftldc_dn5;
        *var_ddriftldc_dn6_slot = var_ddriftldc_dn6;
        *var_ddriftldc_dn7_slot = var_ddriftldc_dn7;
        *var_ddriftldc_dn8_slot = var_ddriftldc_dn8;
        *var_ddriftldc_dn9_slot = var_ddriftldc_dn9;
        *var_ddriftldc_rv_slot = var_ddriftldc_rv;
        *var_deltemp_slot = var_deltemp;
        *var_deltemp_dn0_slot = var_deltemp_dn0;
        *var_deltemp_dn10_slot = var_deltemp_dn10;
        *var_deltemp_dn13_slot = var_deltemp_dn13;
        *var_deltemp_dn2_slot = var_deltemp_dn2;
        *var_deltemp_dn4_slot = var_deltemp_dn4;
        *var_deltemp_dn5_slot = var_deltemp_dn5;
        *var_deltemp_dn6_slot = var_deltemp_dn6;
        *var_deltemp_dn7_slot = var_deltemp_dn7;
        *var_deltemp_dn8_slot = var_deltemp_dn8;
        *var_deltemp_dn9_slot = var_deltemp_dn9;
        *var_deltemp_rv_slot = var_deltemp_rv;
        *var_dl_slot = var_dl;
        *var_dl_rv_slot = var_dl_rv;
        *var_dlld_slot = var_dlld;
        *var_dlld_rv_slot = var_dlld_rv;
        *var_dw_slot = var_dw;
        *var_dw_rv_slot = var_dw_rv;
        *var_dwcv_slot = var_dwcv;
        *var_dwcv_rv_slot = var_dwcv_rv;
        *var_dwld_slot = var_dwld;
        *var_dwld_rv_slot = var_dwld_rv;
        *var_edri_slot = var_edri;
        *var_edri_dn0_slot = var_edri_dn0;
        *var_edri_dn10_slot = var_edri_dn10;
        *var_edri_dn13_slot = var_edri_dn13;
        *var_edri_dn2_slot = var_edri_dn2;
        *var_edri_dn4_slot = var_edri_dn4;
        *var_edri_dn5_slot = var_edri_dn5;
        *var_edri_dn6_slot = var_edri_dn6;
        *var_edri_dn7_slot = var_edri_dn7;
        *var_edri_dn8_slot = var_edri_dn8;
        *var_edri_dn9_slot = var_edri_dn9;
        *var_edri_rv_slot = var_edri_rv;
        *var_gd_slot = var_gd;
        *var_gd_dn0_slot = var_gd_dn0;
        *var_gd_dn10_slot = var_gd_dn10;
        *var_gd_dn13_slot = var_gd_dn13;
        *var_gd_dn2_slot = var_gd_dn2;
        *var_gd_dn4_slot = var_gd_dn4;
        *var_gd_dn5_slot = var_gd_dn5;
        *var_gd_dn6_slot = var_gd_dn6;
        *var_gd_dn7_slot = var_gd_dn7;
        *var_gd_dn8_slot = var_gd_dn8;
        *var_gd_dn9_slot = var_gd_dn9;
        *var_gd_rv_slot = var_gd_rv;
        *var_gth_slot = var_gth;
        *var_gth_dn0_slot = var_gth_dn0;
        *var_gth_dn10_slot = var_gth_dn10;
        *var_gth_dn13_slot = var_gth_dn13;
        *var_gth_dn2_slot = var_gth_dn2;
        *var_gth_dn4_slot = var_gth_dn4;
        *var_gth_dn5_slot = var_gth_dn5;
        *var_gth_dn6_slot = var_gth_dn6;
        *var_gth_dn7_slot = var_gth_dn7;
        *var_gth_dn8_slot = var_gth_dn8;
        *var_gth_dn9_slot = var_gth_dn9;
        *var_gth_rv_slot = var_gth_rv;
        *var_lg_slot = var_lg;
        *var_lg_rv_slot = var_lg_rv;
        *var_lgate_slot = var_lgate;
        *var_lgate_rv_slot = var_lgate_rv;
        *var_lod_half_slot = var_lod_half;
        *var_lod_half_dn0_slot = var_lod_half_dn0;
        *var_lod_half_dn10_slot = var_lod_half_dn10;
        *var_lod_half_dn13_slot = var_lod_half_dn13;
        *var_lod_half_dn2_slot = var_lod_half_dn2;
        *var_lod_half_dn4_slot = var_lod_half_dn4;
        *var_lod_half_dn5_slot = var_lod_half_dn5;
        *var_lod_half_dn6_slot = var_lod_half_dn6;
        *var_lod_half_dn7_slot = var_lod_half_dn7;
        *var_lod_half_dn8_slot = var_lod_half_dn8;
        *var_lod_half_dn9_slot = var_lod_half_dn9;
        *var_lod_half_ref_slot = var_lod_half_ref;
        *var_lod_half_ref_dn0_slot = var_lod_half_ref_dn0;
        *var_lod_half_ref_dn10_slot = var_lod_half_ref_dn10;
        *var_lod_half_ref_dn13_slot = var_lod_half_ref_dn13;
        *var_lod_half_ref_dn2_slot = var_lod_half_ref_dn2;
        *var_lod_half_ref_dn4_slot = var_lod_half_ref_dn4;
        *var_lod_half_ref_dn5_slot = var_lod_half_ref_dn5;
        *var_lod_half_ref_dn6_slot = var_lod_half_ref_dn6;
        *var_lod_half_ref_dn7_slot = var_lod_half_ref_dn7;
        *var_lod_half_ref_dn8_slot = var_lod_half_ref_dn8;
        *var_lod_half_ref_dn9_slot = var_lod_half_ref_dn9;
        *var_lod_half_ref_rv_slot = var_lod_half_ref_rv;
        *var_lod_half_rv_slot = var_lod_half_rv;
        *var_log_tratio_slot = var_log_tratio;
        *var_log_tratio_dn0_slot = var_log_tratio_dn0;
        *var_log_tratio_dn10_slot = var_log_tratio_dn10;
        *var_log_tratio_dn13_slot = var_log_tratio_dn13;
        *var_log_tratio_dn2_slot = var_log_tratio_dn2;
        *var_log_tratio_dn4_slot = var_log_tratio_dn4;
        *var_log_tratio_dn5_slot = var_log_tratio_dn5;
        *var_log_tratio_dn6_slot = var_log_tratio_dn6;
        *var_log_tratio_dn7_slot = var_log_tratio_dn7;
        *var_log_tratio_dn8_slot = var_log_tratio_dn8;
        *var_log_tratio_dn9_slot = var_log_tratio_dn9;
        *var_log_tratio_rv_slot = var_log_tratio_rv;
        *var_mu0_slot = var_mu0;
        *var_mu0_dn0_slot = var_mu0_dn0;
        *var_mu0_dn10_slot = var_mu0_dn10;
        *var_mu0_dn13_slot = var_mu0_dn13;
        *var_mu0_dn2_slot = var_mu0_dn2;
        *var_mu0_dn4_slot = var_mu0_dn4;
        *var_mu0_dn5_slot = var_mu0_dn5;
        *var_mu0_dn6_slot = var_mu0_dn6;
        *var_mu0_dn7_slot = var_mu0_dn7;
        *var_mu0_dn8_slot = var_mu0_dn8;
        *var_mu0_dn9_slot = var_mu0_dn9;
        *var_mu0_rv_slot = var_mu0_rv;
        *var_mueph_slot = var_mueph;
        *var_mueph_dn0_slot = var_mueph_dn0;
        *var_mueph_dn10_slot = var_mueph_dn10;
        *var_mueph_dn13_slot = var_mueph_dn13;
        *var_mueph_dn2_slot = var_mueph_dn2;
        *var_mueph_dn4_slot = var_mueph_dn4;
        *var_mueph_dn5_slot = var_mueph_dn5;
        *var_mueph_dn6_slot = var_mueph_dn6;
        *var_mueph_dn7_slot = var_mueph_dn7;
        *var_mueph_dn8_slot = var_mueph_dn8;
        *var_mueph_dn9_slot = var_mueph_dn9;
        *var_mueph_rv_slot = var_mueph_rv;
        *var_nover_func_slot = var_nover_func;
        *var_nover_func_rv_slot = var_nover_func_rv;
        *var_nsub_slot = var_nsub;
        *var_nsub_dn0_slot = var_nsub_dn0;
        *var_nsub_dn10_slot = var_nsub_dn10;
        *var_nsub_dn13_slot = var_nsub_dn13;
        *var_nsub_dn2_slot = var_nsub_dn2;
        *var_nsub_dn4_slot = var_nsub_dn4;
        *var_nsub_dn5_slot = var_nsub_dn5;
        *var_nsub_dn6_slot = var_nsub_dn6;
        *var_nsub_dn7_slot = var_nsub_dn7;
        *var_nsub_dn8_slot = var_nsub_dn8;
        *var_nsub_dn9_slot = var_nsub_dn9;
        *var_nsub_rv_slot = var_nsub_rv;
        *var_nsubb_slot = var_nsubb;
        *var_nsubb_dn0_slot = var_nsubb_dn0;
        *var_nsubb_dn10_slot = var_nsubb_dn10;
        *var_nsubb_dn13_slot = var_nsubb_dn13;
        *var_nsubb_dn2_slot = var_nsubb_dn2;
        *var_nsubb_dn4_slot = var_nsubb_dn4;
        *var_nsubb_dn5_slot = var_nsubb_dn5;
        *var_nsubb_dn6_slot = var_nsubb_dn6;
        *var_nsubb_dn7_slot = var_nsubb_dn7;
        *var_nsubb_dn8_slot = var_nsubb_dn8;
        *var_nsubb_dn9_slot = var_nsubb_dn9;
        *var_nsubb_rv_slot = var_nsubb_rv;
        *var_nsubpp_slot = var_nsubpp;
        *var_nsubpp_dn0_slot = var_nsubpp_dn0;
        *var_nsubpp_dn10_slot = var_nsubpp_dn10;
        *var_nsubpp_dn13_slot = var_nsubpp_dn13;
        *var_nsubpp_dn2_slot = var_nsubpp_dn2;
        *var_nsubpp_dn4_slot = var_nsubpp_dn4;
        *var_nsubpp_dn5_slot = var_nsubpp_dn5;
        *var_nsubpp_dn6_slot = var_nsubpp_dn6;
        *var_nsubpp_dn7_slot = var_nsubpp_dn7;
        *var_nsubpp_dn8_slot = var_nsubpp_dn8;
        *var_nsubpp_dn9_slot = var_nsubpp_dn9;
        *var_nsubpp_rv_slot = var_nsubpp_rv;
        *var_nsubps_slot = var_nsubps;
        *var_nsubps_dn0_slot = var_nsubps_dn0;
        *var_nsubps_dn10_slot = var_nsubps_dn10;
        *var_nsubps_dn13_slot = var_nsubps_dn13;
        *var_nsubps_dn2_slot = var_nsubps_dn2;
        *var_nsubps_dn4_slot = var_nsubps_dn4;
        *var_nsubps_dn5_slot = var_nsubps_dn5;
        *var_nsubps_dn6_slot = var_nsubps_dn6;
        *var_nsubps_dn7_slot = var_nsubps_dn7;
        *var_nsubps_dn8_slot = var_nsubps_dn8;
        *var_nsubps_dn9_slot = var_nsubps_dn9;
        *var_nsubps_rv_slot = var_nsubps_rv;
        *var_p_slot = var_p;
        *var_p_dn0_slot = var_p_dn0;
        *var_p_dn10_slot = var_p_dn10;
        *var_p_dn13_slot = var_p_dn13;
        *var_p_dn2_slot = var_p_dn2;
        *var_p_dn4_slot = var_p_dn4;
        *var_p_dn5_slot = var_p_dn5;
        *var_p_dn6_slot = var_p_dn6;
        *var_p_dn7_slot = var_p_dn7;
        *var_p_dn8_slot = var_p_dn8;
        *var_p_dn9_slot = var_p_dn9;
        *var_p_rv_slot = var_p_rv;
        *var_ps0ld_slot = var_ps0ld;
        *var_ps0ld_dn0_slot = var_ps0ld_dn0;
        *var_ps0ld_dn10_slot = var_ps0ld_dn10;
        *var_ps0ld_dn13_slot = var_ps0ld_dn13;
        *var_ps0ld_dn2_slot = var_ps0ld_dn2;
        *var_ps0ld_dn4_slot = var_ps0ld_dn4;
        *var_ps0ld_dn5_slot = var_ps0ld_dn5;
        *var_ps0ld_dn6_slot = var_ps0ld_dn6;
        *var_ps0ld_dn7_slot = var_ps0ld_dn7;
        *var_ps0ld_dn8_slot = var_ps0ld_dn8;
        *var_ps0ld_dn9_slot = var_ps0ld_dn9;
        *var_ps0ld_rv_slot = var_ps0ld_rv;
        *var_qb_nqs_slot = var_qb_nqs;
        *var_qb_nqs_dn12_slot = var_qb_nqs_dn12;
        *var_qb_nqs_rv_slot = var_qb_nqs_rv;
        *var_qd_nqs_slot = var_qd_nqs;
        *var_qd_nqs_dn0_slot = var_qd_nqs_dn0;
        *var_qd_nqs_dn10_slot = var_qd_nqs_dn10;
        *var_qd_nqs_dn11_slot = var_qd_nqs_dn11;
        *var_qd_nqs_dn13_slot = var_qd_nqs_dn13;
        *var_qd_nqs_dn2_slot = var_qd_nqs_dn2;
        *var_qd_nqs_dn4_slot = var_qd_nqs_dn4;
        *var_qd_nqs_dn5_slot = var_qd_nqs_dn5;
        *var_qd_nqs_dn6_slot = var_qd_nqs_dn6;
        *var_qd_nqs_dn7_slot = var_qd_nqs_dn7;
        *var_qd_nqs_dn8_slot = var_qd_nqs_dn8;
        *var_qd_nqs_dn9_slot = var_qd_nqs_dn9;
        *var_qd_nqs_rv_slot = var_qd_nqs_rv;
        *var_qg_slot = var_qg;
        *var_qg_dn0_slot = var_qg_dn0;
        *var_qg_dn10_slot = var_qg_dn10;
        *var_qg_dn13_slot = var_qg_dn13;
        *var_qg_dn2_slot = var_qg_dn2;
        *var_qg_dn4_slot = var_qg_dn4;
        *var_qg_dn5_slot = var_qg_dn5;
        *var_qg_dn6_slot = var_qg_dn6;
        *var_qg_dn7_slot = var_qg_dn7;
        *var_qg_dn8_slot = var_qg_dn8;
        *var_qg_dn9_slot = var_qg_dn9;
        *var_qg_nqs_slot = var_qg_nqs;
        *var_qg_nqs_dn11_slot = var_qg_nqs_dn11;
        *var_qg_nqs_dn12_slot = var_qg_nqs_dn12;
        *var_qg_nqs_rv_slot = var_qg_nqs_rv;
        *var_qg_rv_slot = var_qg_rv;
        *var_qi_nqs_slot = var_qi_nqs;
        *var_qi_nqs_dn11_slot = var_qi_nqs_dn11;
        *var_qi_nqs_rv_slot = var_qi_nqs_rv;
        *var_qs_slot = var_qs;
        *var_qs_dn0_slot = var_qs_dn0;
        *var_qs_dn10_slot = var_qs_dn10;
        *var_qs_dn13_slot = var_qs_dn13;
        *var_qs_dn2_slot = var_qs_dn2;
        *var_qs_dn4_slot = var_qs_dn4;
        *var_qs_dn5_slot = var_qs_dn5;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_qs_dn9_slot = var_qs_dn9;
        *var_qs_nqs_slot = var_qs_nqs;
        *var_qs_nqs_dn0_slot = var_qs_nqs_dn0;
        *var_qs_nqs_dn10_slot = var_qs_nqs_dn10;
        *var_qs_nqs_dn11_slot = var_qs_nqs_dn11;
        *var_qs_nqs_dn13_slot = var_qs_nqs_dn13;
        *var_qs_nqs_dn2_slot = var_qs_nqs_dn2;
        *var_qs_nqs_dn4_slot = var_qs_nqs_dn4;
        *var_qs_nqs_dn5_slot = var_qs_nqs_dn5;
        *var_qs_nqs_dn6_slot = var_qs_nqs_dn6;
        *var_qs_nqs_dn7_slot = var_qs_nqs_dn7;
        *var_qs_nqs_dn8_slot = var_qs_nqs_dn8;
        *var_qs_nqs_dn9_slot = var_qs_nqs_dn9;
        *var_qs_nqs_rv_slot = var_qs_nqs_rv;
        *var_qs_rv_slot = var_qs_rv;
        *var_ta_slot = var_ta;
        *var_ta_rv_slot = var_ta_rv;
        *var_tb_slot = var_tb;
        *var_tb_rv_slot = var_tb_rv;
        *var_vbd_slot = var_vbd;
        *var_vbd_dn5_slot = var_vbd_dn5;
        *var_vbd_dn7_slot = var_vbd_dn7;
        *var_vbd_dn8_slot = var_vbd_dn8;
        *var_vbd_rv_slot = var_vbd_rv;
        *var_vbsei_slot = var_vbsei;
        *var_vbsei_dn2_slot = var_vbsei_dn2;
        *var_vbsei_dn8_slot = var_vbsei_dn8;
        *var_vbsei_rv_slot = var_vbsei_rv;
        *var_vbsi_slot = var_vbsi;
        *var_vbsi_dn7_slot = var_vbsi_dn7;
        *var_vbsi_dn8_slot = var_vbsi_dn8;
        *var_vbsi_rv_slot = var_vbsi_rv;
        *var_vddpz_slot = var_vddpz;
        *var_vddpz_dn0_slot = var_vddpz_dn0;
        *var_vddpz_dn10_slot = var_vddpz_dn10;
        *var_vddpz_dn13_slot = var_vddpz_dn13;
        *var_vddpz_dn2_slot = var_vddpz_dn2;
        *var_vddpz_dn4_slot = var_vddpz_dn4;
        *var_vddpz_dn5_slot = var_vddpz_dn5;
        *var_vddpz_dn6_slot = var_vddpz_dn6;
        *var_vddpz_dn7_slot = var_vddpz_dn7;
        *var_vddpz_dn8_slot = var_vddpz_dn8;
        *var_vddpz_dn9_slot = var_vddpz_dn9;
        *var_vddpz_rv_slot = var_vddpz_rv;
        *var_vdri_slot = var_vdri;
        *var_vdri_dn0_slot = var_vdri_dn0;
        *var_vdri_dn10_slot = var_vdri_dn10;
        *var_vdri_dn13_slot = var_vdri_dn13;
        *var_vdri_dn2_slot = var_vdri_dn2;
        *var_vdri_dn4_slot = var_vdri_dn4;
        *var_vdri_dn5_slot = var_vdri_dn5;
        *var_vdri_dn6_slot = var_vdri_dn6;
        *var_vdri_dn7_slot = var_vdri_dn7;
        *var_vdri_dn8_slot = var_vdri_dn8;
        *var_vdri_dn9_slot = var_vdri_dn9;
        *var_vdri_rv_slot = var_vdri_rv;
        *var_vdsei_slot = var_vdsei;
        *var_vdsei_dn0_slot = var_vdsei_dn0;
        *var_vdsei_dn2_slot = var_vdsei_dn2;
        *var_vdsei_rv_slot = var_vdsei_rv;
        *var_vdsi_slot = var_vdsi;
        *var_vdsi_dn5_slot = var_vdsi_dn5;
        *var_vdsi_dn7_slot = var_vdsi_dn7;
        *var_vdsi_rv_slot = var_vdsi_rv;
        *var_veffpower_slot = var_veffpower;
        *var_veffpower_dn0_slot = var_veffpower_dn0;
        *var_veffpower_dn10_slot = var_veffpower_dn10;
        *var_veffpower_dn13_slot = var_veffpower_dn13;
        *var_veffpower_dn2_slot = var_veffpower_dn2;
        *var_veffpower_dn4_slot = var_veffpower_dn4;
        *var_veffpower_dn5_slot = var_veffpower_dn5;
        *var_veffpower_dn6_slot = var_veffpower_dn6;
        *var_veffpower_dn7_slot = var_veffpower_dn7;
        *var_veffpower_dn8_slot = var_veffpower_dn8;
        *var_veffpower_dn9_slot = var_veffpower_dn9;
        *var_veffpower_rv_slot = var_veffpower_rv;
        *var_vgb_fb_ld_slot = var_vgb_fb_ld;
        *var_vgb_fb_ld_dn0_slot = var_vgb_fb_ld_dn0;
        *var_vgb_fb_ld_dn10_slot = var_vgb_fb_ld_dn10;
        *var_vgb_fb_ld_dn13_slot = var_vgb_fb_ld_dn13;
        *var_vgb_fb_ld_dn2_slot = var_vgb_fb_ld_dn2;
        *var_vgb_fb_ld_dn4_slot = var_vgb_fb_ld_dn4;
        *var_vgb_fb_ld_dn5_slot = var_vgb_fb_ld_dn5;
        *var_vgb_fb_ld_dn6_slot = var_vgb_fb_ld_dn6;
        *var_vgb_fb_ld_dn7_slot = var_vgb_fb_ld_dn7;
        *var_vgb_fb_ld_dn8_slot = var_vgb_fb_ld_dn8;
        *var_vgb_fb_ld_dn9_slot = var_vgb_fb_ld_dn9;
        *var_vgb_fb_ld_rv_slot = var_vgb_fb_ld_rv;
        *var_vgd_slot = var_vgd;
        *var_vgd_dn5_slot = var_vgd_dn5;
        *var_vgd_dn6_slot = var_vgd_dn6;
        *var_vgd_dn7_slot = var_vgd_dn7;
        *var_vgd_rv_slot = var_vgd_rv;
        *var_vgsei_slot = var_vgsei;
        *var_vgsei_dn2_slot = var_vgsei_dn2;
        *var_vgsei_dn6_slot = var_vgsei_dn6;
        *var_vgsei_rv_slot = var_vgsei_rv;
        *var_vgsi_slot = var_vgsi;
        *var_vgsi_dn6_slot = var_vgsi_dn6;
        *var_vgsi_dn7_slot = var_vgsi_dn7;
        *var_vgsi_rv_slot = var_vgsi_rv;
        *var_wg_slot = var_wg;
        *var_wg_rv_slot = var_wg_rv;
        *var_wgate_slot = var_wgate;
        *var_wgate_rv_slot = var_wgate_rv;
        *var_wlg_slot = var_wlg;
        *var_wlg_rv_slot = var_wlg_rv;
        *var_xov_slot = var_xov;
        *var_xov_dn0_slot = var_xov_dn0;
        *var_xov_dn10_slot = var_xov_dn10;
        *var_xov_dn13_slot = var_xov_dn13;
        *var_xov_dn2_slot = var_xov_dn2;
        *var_xov_dn4_slot = var_xov_dn4;
        *var_xov_dn5_slot = var_xov_dn5;
        *var_xov_dn6_slot = var_xov_dn6;
        *var_xov_dn7_slot = var_xov_dn7;
        *var_xov_dn8_slot = var_xov_dn8;
        *var_xov_dn9_slot = var_xov_dn9;
        *var_xov_rv_slot = var_xov_rv;
    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        var_betatnom_slot: &mut f64,
        var_betatnom_rv_slot: &mut f64,
        var_cgsb_slot: &mut f64,
        var_cgsb_dn0_slot: &mut f64,
        var_cgsb_dn10_slot: &mut f64,
        var_cgsb_dn13_slot: &mut f64,
        var_cgsb_dn2_slot: &mut f64,
        var_cgsb_dn4_slot: &mut f64,
        var_cgsb_dn5_slot: &mut f64,
        var_cgsb_dn6_slot: &mut f64,
        var_cgsb_dn7_slot: &mut f64,
        var_cgsb_dn8_slot: &mut f64,
        var_cgsb_dn9_slot: &mut f64,
        var_cgsb_rv_slot: &mut f64,
        var_cnst0over_slot: &mut f64,
        var_cnst0over_dn0_slot: &mut f64,
        var_cnst0over_dn10_slot: &mut f64,
        var_cnst0over_dn13_slot: &mut f64,
        var_cnst0over_dn2_slot: &mut f64,
        var_cnst0over_dn4_slot: &mut f64,
        var_cnst0over_dn5_slot: &mut f64,
        var_cnst0over_dn6_slot: &mut f64,
        var_cnst0over_dn7_slot: &mut f64,
        var_cnst0over_dn8_slot: &mut f64,
        var_cnst0over_dn9_slot: &mut f64,
        var_cnst0over_rv_slot: &mut f64,
        var_cnst0overs_slot: &mut f64,
        var_cnst0overs_dn0_slot: &mut f64,
        var_cnst0overs_dn10_slot: &mut f64,
        var_cnst0overs_dn13_slot: &mut f64,
        var_cnst0overs_dn2_slot: &mut f64,
        var_cnst0overs_dn4_slot: &mut f64,
        var_cnst0overs_dn5_slot: &mut f64,
        var_cnst0overs_dn6_slot: &mut f64,
        var_cnst0overs_dn7_slot: &mut f64,
        var_cnst0overs_dn8_slot: &mut f64,
        var_cnst0overs_dn9_slot: &mut f64,
        var_cnst0overs_rv_slot: &mut f64,
        var_costi0_p2_slot: &mut f64,
        var_costi0_p2_dn0_slot: &mut f64,
        var_costi0_p2_dn10_slot: &mut f64,
        var_costi0_p2_dn13_slot: &mut f64,
        var_costi0_p2_dn2_slot: &mut f64,
        var_costi0_p2_dn4_slot: &mut f64,
        var_costi0_p2_dn5_slot: &mut f64,
        var_costi0_p2_dn6_slot: &mut f64,
        var_costi0_p2_dn7_slot: &mut f64,
        var_costi0_p2_dn8_slot: &mut f64,
        var_costi0_p2_dn9_slot: &mut f64,
        var_costi0_p2_rv_slot: &mut f64,
        var_depmphn0_slot: &mut f64,
        var_depmphn0_dn0_slot: &mut f64,
        var_depmphn0_dn10_slot: &mut f64,
        var_depmphn0_dn13_slot: &mut f64,
        var_depmphn0_dn2_slot: &mut f64,
        var_depmphn0_dn4_slot: &mut f64,
        var_depmphn0_dn5_slot: &mut f64,
        var_depmphn0_dn6_slot: &mut f64,
        var_depmphn0_dn7_slot: &mut f64,
        var_depmphn0_dn8_slot: &mut f64,
        var_depmphn0_dn9_slot: &mut f64,
        var_depmphn0_rv_slot: &mut f64,
        var_hbdceff_slot: &mut f64,
        var_hbdceff_dn0_slot: &mut f64,
        var_hbdceff_dn10_slot: &mut f64,
        var_hbdceff_dn13_slot: &mut f64,
        var_hbdceff_dn2_slot: &mut f64,
        var_hbdceff_dn4_slot: &mut f64,
        var_hbdceff_dn5_slot: &mut f64,
        var_hbdceff_dn6_slot: &mut f64,
        var_hbdceff_dn7_slot: &mut f64,
        var_hbdceff_dn8_slot: &mut f64,
        var_hbdceff_dn9_slot: &mut f64,
        var_hbdceff_rv_slot: &mut f64,
        var_ids_acc_slot: &mut f64,
        var_ids_acc_dn0_slot: &mut f64,
        var_ids_acc_dn10_slot: &mut f64,
        var_ids_acc_dn13_slot: &mut f64,
        var_ids_acc_dn2_slot: &mut f64,
        var_ids_acc_dn4_slot: &mut f64,
        var_ids_acc_dn5_slot: &mut f64,
        var_ids_acc_dn6_slot: &mut f64,
        var_ids_acc_dn7_slot: &mut f64,
        var_ids_acc_dn8_slot: &mut f64,
        var_ids_acc_dn9_slot: &mut f64,
        var_ids_acc_rv_slot: &mut f64,
        var_ids_res_slot: &mut f64,
        var_ids_res_dn0_slot: &mut f64,
        var_ids_res_dn10_slot: &mut f64,
        var_ids_res_dn13_slot: &mut f64,
        var_ids_res_dn2_slot: &mut f64,
        var_ids_res_dn4_slot: &mut f64,
        var_ids_res_dn5_slot: &mut f64,
        var_ids_res_dn6_slot: &mut f64,
        var_ids_res_dn7_slot: &mut f64,
        var_ids_res_dn8_slot: &mut f64,
        var_ids_res_dn9_slot: &mut f64,
        var_ids_res_rv_slot: &mut f64,
        var_ires_leak_slot: &mut f64,
        var_ires_leak_dn0_slot: &mut f64,
        var_ires_leak_dn10_slot: &mut f64,
        var_ires_leak_dn13_slot: &mut f64,
        var_ires_leak_dn2_slot: &mut f64,
        var_ires_leak_dn4_slot: &mut f64,
        var_ires_leak_dn5_slot: &mut f64,
        var_ires_leak_dn6_slot: &mut f64,
        var_ires_leak_dn7_slot: &mut f64,
        var_ires_leak_dn8_slot: &mut f64,
        var_ires_leak_dn9_slot: &mut f64,
        var_ires_leak_rv_slot: &mut f64,
        var_js_slot: &mut f64,
        var_js2_slot: &mut f64,
        var_js2_dn0_slot: &mut f64,
        var_js2_dn10_slot: &mut f64,
        var_js2_dn13_slot: &mut f64,
        var_js2_dn2_slot: &mut f64,
        var_js2_dn4_slot: &mut f64,
        var_js2_dn5_slot: &mut f64,
        var_js2_dn6_slot: &mut f64,
        var_js2_dn7_slot: &mut f64,
        var_js2_dn8_slot: &mut f64,
        var_js2_dn9_slot: &mut f64,
        var_js2_rv_slot: &mut f64,
        var_js_dn0_slot: &mut f64,
        var_js_dn10_slot: &mut f64,
        var_js_dn13_slot: &mut f64,
        var_js_dn2_slot: &mut f64,
        var_js_dn4_slot: &mut f64,
        var_js_dn5_slot: &mut f64,
        var_js_dn6_slot: &mut f64,
        var_js_dn7_slot: &mut f64,
        var_js_dn8_slot: &mut f64,
        var_js_dn9_slot: &mut f64,
        var_js_rv_slot: &mut f64,
        var_jssw_slot: &mut f64,
        var_jssw2_slot: &mut f64,
        var_jssw2_dn0_slot: &mut f64,
        var_jssw2_dn10_slot: &mut f64,
        var_jssw2_dn13_slot: &mut f64,
        var_jssw2_dn2_slot: &mut f64,
        var_jssw2_dn4_slot: &mut f64,
        var_jssw2_dn5_slot: &mut f64,
        var_jssw2_dn6_slot: &mut f64,
        var_jssw2_dn7_slot: &mut f64,
        var_jssw2_dn8_slot: &mut f64,
        var_jssw2_dn9_slot: &mut f64,
        var_jssw2_rv_slot: &mut f64,
        var_jssw_dn0_slot: &mut f64,
        var_jssw_dn10_slot: &mut f64,
        var_jssw_dn13_slot: &mut f64,
        var_jssw_dn2_slot: &mut f64,
        var_jssw_dn4_slot: &mut f64,
        var_jssw_dn5_slot: &mut f64,
        var_jssw_dn6_slot: &mut f64,
        var_jssw_dn7_slot: &mut f64,
        var_jssw_dn8_slot: &mut f64,
        var_jssw_dn9_slot: &mut f64,
        var_jssw_rv_slot: &mut f64,
        var_lp_s0_max_slot: &mut f64,
        var_lp_s0_max_rv_slot: &mut f64,
        var_mphn0_slot: &mut f64,
        var_mphn0_dn0_slot: &mut f64,
        var_mphn0_dn10_slot: &mut f64,
        var_mphn0_dn13_slot: &mut f64,
        var_mphn0_dn2_slot: &mut f64,
        var_mphn0_dn4_slot: &mut f64,
        var_mphn0_dn5_slot: &mut f64,
        var_mphn0_dn6_slot: &mut f64,
        var_mphn0_dn7_slot: &mut f64,
        var_mphn0_dn8_slot: &mut f64,
        var_mphn0_dn9_slot: &mut f64,
        var_mphn0_rv_slot: &mut f64,
        var_ninvde_slot: &mut f64,
        var_ninvde_dn0_slot: &mut f64,
        var_ninvde_dn10_slot: &mut f64,
        var_ninvde_dn13_slot: &mut f64,
        var_ninvde_dn2_slot: &mut f64,
        var_ninvde_dn4_slot: &mut f64,
        var_ninvde_dn5_slot: &mut f64,
        var_ninvde_dn6_slot: &mut f64,
        var_ninvde_dn7_slot: &mut f64,
        var_ninvde_dn8_slot: &mut f64,
        var_ninvde_dn9_slot: &mut f64,
        var_ninvde_rv_slot: &mut f64,
        var_ninvdecres_slot: &mut f64,
        var_ninvdecres_dn0_slot: &mut f64,
        var_ninvdecres_dn10_slot: &mut f64,
        var_ninvdecres_dn13_slot: &mut f64,
        var_ninvdecres_dn2_slot: &mut f64,
        var_ninvdecres_dn4_slot: &mut f64,
        var_ninvdecres_dn5_slot: &mut f64,
        var_ninvdecres_dn6_slot: &mut f64,
        var_ninvdecres_dn7_slot: &mut f64,
        var_ninvdecres_dn8_slot: &mut f64,
        var_ninvdecres_dn9_slot: &mut f64,
        var_ninvdecres_rv_slot: &mut f64,
        var_ninvdehres_slot: &mut f64,
        var_ninvdehres_dn0_slot: &mut f64,
        var_ninvdehres_dn10_slot: &mut f64,
        var_ninvdehres_dn13_slot: &mut f64,
        var_ninvdehres_dn2_slot: &mut f64,
        var_ninvdehres_dn4_slot: &mut f64,
        var_ninvdehres_dn5_slot: &mut f64,
        var_ninvdehres_dn6_slot: &mut f64,
        var_ninvdehres_dn7_slot: &mut f64,
        var_ninvdehres_dn8_slot: &mut f64,
        var_ninvdehres_dn9_slot: &mut f64,
        var_ninvdehres_rv_slot: &mut f64,
        var_pb2n_slot: &mut f64,
        var_pb2n_dn0_slot: &mut f64,
        var_pb2n_dn10_slot: &mut f64,
        var_pb2n_dn13_slot: &mut f64,
        var_pb2n_dn2_slot: &mut f64,
        var_pb2n_dn4_slot: &mut f64,
        var_pb2n_dn5_slot: &mut f64,
        var_pb2n_dn6_slot: &mut f64,
        var_pb2n_dn7_slot: &mut f64,
        var_pb2n_dn8_slot: &mut f64,
        var_pb2n_dn9_slot: &mut f64,
        var_pb2n_rv_slot: &mut f64,
        var_powratio_slot: &mut f64,
        var_powratio_dn0_slot: &mut f64,
        var_powratio_dn10_slot: &mut f64,
        var_powratio_dn13_slot: &mut f64,
        var_powratio_dn2_slot: &mut f64,
        var_powratio_dn4_slot: &mut f64,
        var_powratio_dn5_slot: &mut f64,
        var_powratio_dn6_slot: &mut f64,
        var_powratio_dn7_slot: &mut f64,
        var_powratio_dn8_slot: &mut f64,
        var_powratio_dn9_slot: &mut f64,
        var_powratio_rv_slot: &mut f64,
        var_ptovr_slot: &mut f64,
        var_ptovr_dn0_slot: &mut f64,
        var_ptovr_dn10_slot: &mut f64,
        var_ptovr_dn13_slot: &mut f64,
        var_ptovr_dn2_slot: &mut f64,
        var_ptovr_dn4_slot: &mut f64,
        var_ptovr_dn5_slot: &mut f64,
        var_ptovr_dn6_slot: &mut f64,
        var_ptovr_dn7_slot: &mut f64,
        var_ptovr_dn8_slot: &mut f64,
        var_ptovr_dn9_slot: &mut f64,
        var_ptovr_rv_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn0_slot: &mut f64,
        var_qbs_dn10_slot: &mut f64,
        var_qbs_dn13_slot: &mut f64,
        var_qbs_dn2_slot: &mut f64,
        var_qbs_dn4_slot: &mut f64,
        var_qbs_dn5_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_qbs_dn8_slot: &mut f64,
        var_qbs_dn9_slot: &mut f64,
        var_qbs_rv_slot: &mut f64,
        var_qiu_noi_slot: &mut f64,
        var_qiu_noi_dn0_slot: &mut f64,
        var_qiu_noi_dn10_slot: &mut f64,
        var_qiu_noi_dn13_slot: &mut f64,
        var_qiu_noi_dn2_slot: &mut f64,
        var_qiu_noi_dn4_slot: &mut f64,
        var_qiu_noi_dn5_slot: &mut f64,
        var_qiu_noi_dn6_slot: &mut f64,
        var_qiu_noi_dn7_slot: &mut f64,
        var_qiu_noi_dn8_slot: &mut f64,
        var_qiu_noi_dn9_slot: &mut f64,
        var_qiu_noi_rv_slot: &mut f64,
        var_rde_slot: &mut f64,
        var_rde_dn0_slot: &mut f64,
        var_rde_dn10_slot: &mut f64,
        var_rde_dn13_slot: &mut f64,
        var_rde_dn2_slot: &mut f64,
        var_rde_dn4_slot: &mut f64,
        var_rde_dn5_slot: &mut f64,
        var_rde_dn6_slot: &mut f64,
        var_rde_dn7_slot: &mut f64,
        var_rde_dn8_slot: &mut f64,
        var_rde_dn9_slot: &mut f64,
        var_rde_rv_slot: &mut f64,
        var_rdvde_slot: &mut f64,
        var_rdvde_dn0_slot: &mut f64,
        var_rdvde_dn10_slot: &mut f64,
        var_rdvde_dn13_slot: &mut f64,
        var_rdvde_dn2_slot: &mut f64,
        var_rdvde_dn4_slot: &mut f64,
        var_rdvde_dn5_slot: &mut f64,
        var_rdvde_dn6_slot: &mut f64,
        var_rdvde_dn7_slot: &mut f64,
        var_rdvde_dn8_slot: &mut f64,
        var_rdvde_dn9_slot: &mut f64,
        var_rdvde_rv_slot: &mut f64,
        var_rrdrmue_slot: &mut f64,
        var_rrdrmue_dn0_slot: &mut f64,
        var_rrdrmue_dn10_slot: &mut f64,
        var_rrdrmue_dn13_slot: &mut f64,
        var_rrdrmue_dn2_slot: &mut f64,
        var_rrdrmue_dn4_slot: &mut f64,
        var_rrdrmue_dn5_slot: &mut f64,
        var_rrdrmue_dn6_slot: &mut f64,
        var_rrdrmue_dn7_slot: &mut f64,
        var_rrdrmue_dn8_slot: &mut f64,
        var_rrdrmue_dn9_slot: &mut f64,
        var_rrdrmue_rv_slot: &mut f64,
        var_rrdrmues_slot: &mut f64,
        var_rrdrmues_dn0_slot: &mut f64,
        var_rrdrmues_dn10_slot: &mut f64,
        var_rrdrmues_dn13_slot: &mut f64,
        var_rrdrmues_dn2_slot: &mut f64,
        var_rrdrmues_dn4_slot: &mut f64,
        var_rrdrmues_dn5_slot: &mut f64,
        var_rrdrmues_dn6_slot: &mut f64,
        var_rrdrmues_dn7_slot: &mut f64,
        var_rrdrmues_dn8_slot: &mut f64,
        var_rrdrmues_dn9_slot: &mut f64,
        var_rrdrmues_rv_slot: &mut f64,
        var_rrdrvmax_slot: &mut f64,
        var_rrdrvmax_dn0_slot: &mut f64,
        var_rrdrvmax_dn10_slot: &mut f64,
        var_rrdrvmax_dn13_slot: &mut f64,
        var_rrdrvmax_dn2_slot: &mut f64,
        var_rrdrvmax_dn4_slot: &mut f64,
        var_rrdrvmax_dn5_slot: &mut f64,
        var_rrdrvmax_dn6_slot: &mut f64,
        var_rrdrvmax_dn7_slot: &mut f64,
        var_rrdrvmax_dn8_slot: &mut f64,
        var_rrdrvmax_dn9_slot: &mut f64,
        var_rrdrvmax_rv_slot: &mut f64,
        var_rrdrvmaxs_slot: &mut f64,
        var_rrdrvmaxs_dn0_slot: &mut f64,
        var_rrdrvmaxs_dn10_slot: &mut f64,
        var_rrdrvmaxs_dn13_slot: &mut f64,
        var_rrdrvmaxs_dn2_slot: &mut f64,
        var_rrdrvmaxs_dn4_slot: &mut f64,
        var_rrdrvmaxs_dn5_slot: &mut f64,
        var_rrdrvmaxs_dn6_slot: &mut f64,
        var_rrdrvmaxs_dn7_slot: &mut f64,
        var_rrdrvmaxs_dn8_slot: &mut f64,
        var_rrdrvmaxs_dn9_slot: &mut f64,
        var_rrdrvmaxs_rv_slot: &mut f64,
        var_rse_slot: &mut f64,
        var_rse_dn0_slot: &mut f64,
        var_rse_dn10_slot: &mut f64,
        var_rse_dn13_slot: &mut f64,
        var_rse_dn2_slot: &mut f64,
        var_rse_dn4_slot: &mut f64,
        var_rse_dn5_slot: &mut f64,
        var_rse_dn6_slot: &mut f64,
        var_rse_dn7_slot: &mut f64,
        var_rse_dn8_slot: &mut f64,
        var_rse_dn9_slot: &mut f64,
        var_rse_rv_slot: &mut f64,
        var_rsvde_slot: &mut f64,
        var_rsvde_dn0_slot: &mut f64,
        var_rsvde_dn10_slot: &mut f64,
        var_rsvde_dn13_slot: &mut f64,
        var_rsvde_dn2_slot: &mut f64,
        var_rsvde_dn4_slot: &mut f64,
        var_rsvde_dn5_slot: &mut f64,
        var_rsvde_dn6_slot: &mut f64,
        var_rsvde_dn7_slot: &mut f64,
        var_rsvde_dn8_slot: &mut f64,
        var_rsvde_dn9_slot: &mut f64,
        var_rsvde_rv_slot: &mut f64,
        var_sqrt_eg_slot: &mut f64,
        var_sqrt_eg_dn0_slot: &mut f64,
        var_sqrt_eg_dn10_slot: &mut f64,
        var_sqrt_eg_dn13_slot: &mut f64,
        var_sqrt_eg_dn2_slot: &mut f64,
        var_sqrt_eg_dn4_slot: &mut f64,
        var_sqrt_eg_dn5_slot: &mut f64,
        var_sqrt_eg_dn6_slot: &mut f64,
        var_sqrt_eg_dn7_slot: &mut f64,
        var_sqrt_eg_dn8_slot: &mut f64,
        var_sqrt_eg_dn9_slot: &mut f64,
        var_sqrt_eg_rv_slot: &mut f64,
        var_tratio_slot: &mut f64,
        var_tratio_dn0_slot: &mut f64,
        var_tratio_dn10_slot: &mut f64,
        var_tratio_dn13_slot: &mut f64,
        var_tratio_dn2_slot: &mut f64,
        var_tratio_dn4_slot: &mut f64,
        var_tratio_dn5_slot: &mut f64,
        var_tratio_dn6_slot: &mut f64,
        var_tratio_dn7_slot: &mut f64,
        var_tratio_dn8_slot: &mut f64,
        var_tratio_dn9_slot: &mut f64,
        var_tratio_rv_slot: &mut f64,
        var_uc_rdrbb_slot: &mut f64,
        var_uc_rdrbb_dn0_slot: &mut f64,
        var_uc_rdrbb_dn10_slot: &mut f64,
        var_uc_rdrbb_dn13_slot: &mut f64,
        var_uc_rdrbb_dn2_slot: &mut f64,
        var_uc_rdrbb_dn4_slot: &mut f64,
        var_uc_rdrbb_dn5_slot: &mut f64,
        var_uc_rdrbb_dn6_slot: &mut f64,
        var_uc_rdrbb_dn7_slot: &mut f64,
        var_uc_rdrbb_dn8_slot: &mut f64,
        var_uc_rdrbb_dn9_slot: &mut f64,
        var_uc_rdrbb_rv_slot: &mut f64,
        var_uc_rdrbb_s_slot: &mut f64,
        var_uc_rdrbb_s_dn0_slot: &mut f64,
        var_uc_rdrbb_s_dn10_slot: &mut f64,
        var_uc_rdrbb_s_dn13_slot: &mut f64,
        var_uc_rdrbb_s_dn2_slot: &mut f64,
        var_uc_rdrbb_s_dn4_slot: &mut f64,
        var_uc_rdrbb_s_dn5_slot: &mut f64,
        var_uc_rdrbb_s_dn6_slot: &mut f64,
        var_uc_rdrbb_s_dn7_slot: &mut f64,
        var_uc_rdrbb_s_dn8_slot: &mut f64,
        var_uc_rdrbb_s_dn9_slot: &mut f64,
        var_uc_rdrbb_s_rv_slot: &mut f64,
        var_uc_subtmp_slot: &mut f64,
        var_uc_subtmp_rv_slot: &mut f64,
        var_vbipn_slot: &mut f64,
        var_vbipn_dn0_slot: &mut f64,
        var_vbipn_dn10_slot: &mut f64,
        var_vbipn_dn13_slot: &mut f64,
        var_vbipn_dn2_slot: &mut f64,
        var_vbipn_dn4_slot: &mut f64,
        var_vbipn_dn5_slot: &mut f64,
        var_vbipn_dn6_slot: &mut f64,
        var_vbipn_dn7_slot: &mut f64,
        var_vbipn_dn8_slot: &mut f64,
        var_vbipn_dn9_slot: &mut f64,
        var_vbipn_rv_slot: &mut f64,
        var_vmaxeff_slot: &mut f64,
        var_vmaxeff_dn0_slot: &mut f64,
        var_vmaxeff_dn10_slot: &mut f64,
        var_vmaxeff_dn13_slot: &mut f64,
        var_vmaxeff_dn2_slot: &mut f64,
        var_vmaxeff_dn4_slot: &mut f64,
        var_vmaxeff_dn5_slot: &mut f64,
        var_vmaxeff_dn6_slot: &mut f64,
        var_vmaxeff_dn7_slot: &mut f64,
        var_vmaxeff_dn8_slot: &mut f64,
        var_vmaxeff_dn9_slot: &mut f64,
        var_vmaxeff_rv_slot: &mut f64,
        var_wdpl_slot: &mut f64,
        var_wdpl_dn0_slot: &mut f64,
        var_wdpl_dn10_slot: &mut f64,
        var_wdpl_dn13_slot: &mut f64,
        var_wdpl_dn2_slot: &mut f64,
        var_wdpl_dn4_slot: &mut f64,
        var_wdpl_dn5_slot: &mut f64,
        var_wdpl_dn6_slot: &mut f64,
        var_wdpl_dn7_slot: &mut f64,
        var_wdpl_dn8_slot: &mut f64,
        var_wdpl_dn9_slot: &mut f64,
        var_wdpl_rv_slot: &mut f64,
        var_wdplp_slot: &mut f64,
        var_wdplp_dn0_slot: &mut f64,
        var_wdplp_dn10_slot: &mut f64,
        var_wdplp_dn13_slot: &mut f64,
        var_wdplp_dn2_slot: &mut f64,
        var_wdplp_dn4_slot: &mut f64,
        var_wdplp_dn5_slot: &mut f64,
        var_wdplp_dn6_slot: &mut f64,
        var_wdplp_dn7_slot: &mut f64,
        var_wdplp_dn8_slot: &mut f64,
        var_wdplp_dn9_slot: &mut f64,
        var_wdplp_rv_slot: &mut f64,
    ) {
        let mut var_betatnom: f64 = *var_betatnom_slot;
        let mut var_betatnom_rv: f64 = *var_betatnom_rv_slot;
        let mut var_cgsb: f64 = *var_cgsb_slot;
        let mut var_cgsb_dn0: f64 = *var_cgsb_dn0_slot;
        let mut var_cgsb_dn10: f64 = *var_cgsb_dn10_slot;
        let mut var_cgsb_dn13: f64 = *var_cgsb_dn13_slot;
        let mut var_cgsb_dn2: f64 = *var_cgsb_dn2_slot;
        let mut var_cgsb_dn4: f64 = *var_cgsb_dn4_slot;
        let mut var_cgsb_dn5: f64 = *var_cgsb_dn5_slot;
        let mut var_cgsb_dn6: f64 = *var_cgsb_dn6_slot;
        let mut var_cgsb_dn7: f64 = *var_cgsb_dn7_slot;
        let mut var_cgsb_dn8: f64 = *var_cgsb_dn8_slot;
        let mut var_cgsb_dn9: f64 = *var_cgsb_dn9_slot;
        let mut var_cgsb_rv: f64 = *var_cgsb_rv_slot;
        let mut var_cnst0over: f64 = *var_cnst0over_slot;
        let mut var_cnst0over_dn0: f64 = *var_cnst0over_dn0_slot;
        let mut var_cnst0over_dn10: f64 = *var_cnst0over_dn10_slot;
        let mut var_cnst0over_dn13: f64 = *var_cnst0over_dn13_slot;
        let mut var_cnst0over_dn2: f64 = *var_cnst0over_dn2_slot;
        let mut var_cnst0over_dn4: f64 = *var_cnst0over_dn4_slot;
        let mut var_cnst0over_dn5: f64 = *var_cnst0over_dn5_slot;
        let mut var_cnst0over_dn6: f64 = *var_cnst0over_dn6_slot;
        let mut var_cnst0over_dn7: f64 = *var_cnst0over_dn7_slot;
        let mut var_cnst0over_dn8: f64 = *var_cnst0over_dn8_slot;
        let mut var_cnst0over_dn9: f64 = *var_cnst0over_dn9_slot;
        let mut var_cnst0over_rv: f64 = *var_cnst0over_rv_slot;
        let mut var_cnst0overs: f64 = *var_cnst0overs_slot;
        let mut var_cnst0overs_dn0: f64 = *var_cnst0overs_dn0_slot;
        let mut var_cnst0overs_dn10: f64 = *var_cnst0overs_dn10_slot;
        let mut var_cnst0overs_dn13: f64 = *var_cnst0overs_dn13_slot;
        let mut var_cnst0overs_dn2: f64 = *var_cnst0overs_dn2_slot;
        let mut var_cnst0overs_dn4: f64 = *var_cnst0overs_dn4_slot;
        let mut var_cnst0overs_dn5: f64 = *var_cnst0overs_dn5_slot;
        let mut var_cnst0overs_dn6: f64 = *var_cnst0overs_dn6_slot;
        let mut var_cnst0overs_dn7: f64 = *var_cnst0overs_dn7_slot;
        let mut var_cnst0overs_dn8: f64 = *var_cnst0overs_dn8_slot;
        let mut var_cnst0overs_dn9: f64 = *var_cnst0overs_dn9_slot;
        let mut var_cnst0overs_rv: f64 = *var_cnst0overs_rv_slot;
        let mut var_costi0_p2: f64 = *var_costi0_p2_slot;
        let mut var_costi0_p2_dn0: f64 = *var_costi0_p2_dn0_slot;
        let mut var_costi0_p2_dn10: f64 = *var_costi0_p2_dn10_slot;
        let mut var_costi0_p2_dn13: f64 = *var_costi0_p2_dn13_slot;
        let mut var_costi0_p2_dn2: f64 = *var_costi0_p2_dn2_slot;
        let mut var_costi0_p2_dn4: f64 = *var_costi0_p2_dn4_slot;
        let mut var_costi0_p2_dn5: f64 = *var_costi0_p2_dn5_slot;
        let mut var_costi0_p2_dn6: f64 = *var_costi0_p2_dn6_slot;
        let mut var_costi0_p2_dn7: f64 = *var_costi0_p2_dn7_slot;
        let mut var_costi0_p2_dn8: f64 = *var_costi0_p2_dn8_slot;
        let mut var_costi0_p2_dn9: f64 = *var_costi0_p2_dn9_slot;
        let mut var_costi0_p2_rv: f64 = *var_costi0_p2_rv_slot;
        let mut var_depmphn0: f64 = *var_depmphn0_slot;
        let mut var_depmphn0_dn0: f64 = *var_depmphn0_dn0_slot;
        let mut var_depmphn0_dn10: f64 = *var_depmphn0_dn10_slot;
        let mut var_depmphn0_dn13: f64 = *var_depmphn0_dn13_slot;
        let mut var_depmphn0_dn2: f64 = *var_depmphn0_dn2_slot;
        let mut var_depmphn0_dn4: f64 = *var_depmphn0_dn4_slot;
        let mut var_depmphn0_dn5: f64 = *var_depmphn0_dn5_slot;
        let mut var_depmphn0_dn6: f64 = *var_depmphn0_dn6_slot;
        let mut var_depmphn0_dn7: f64 = *var_depmphn0_dn7_slot;
        let mut var_depmphn0_dn8: f64 = *var_depmphn0_dn8_slot;
        let mut var_depmphn0_dn9: f64 = *var_depmphn0_dn9_slot;
        let mut var_depmphn0_rv: f64 = *var_depmphn0_rv_slot;
        let mut var_hbdceff: f64 = *var_hbdceff_slot;
        let mut var_hbdceff_dn0: f64 = *var_hbdceff_dn0_slot;
        let mut var_hbdceff_dn10: f64 = *var_hbdceff_dn10_slot;
        let mut var_hbdceff_dn13: f64 = *var_hbdceff_dn13_slot;
        let mut var_hbdceff_dn2: f64 = *var_hbdceff_dn2_slot;
        let mut var_hbdceff_dn4: f64 = *var_hbdceff_dn4_slot;
        let mut var_hbdceff_dn5: f64 = *var_hbdceff_dn5_slot;
        let mut var_hbdceff_dn6: f64 = *var_hbdceff_dn6_slot;
        let mut var_hbdceff_dn7: f64 = *var_hbdceff_dn7_slot;
        let mut var_hbdceff_dn8: f64 = *var_hbdceff_dn8_slot;
        let mut var_hbdceff_dn9: f64 = *var_hbdceff_dn9_slot;
        let mut var_hbdceff_rv: f64 = *var_hbdceff_rv_slot;
        let mut var_ids_acc: f64 = *var_ids_acc_slot;
        let mut var_ids_acc_dn0: f64 = *var_ids_acc_dn0_slot;
        let mut var_ids_acc_dn10: f64 = *var_ids_acc_dn10_slot;
        let mut var_ids_acc_dn13: f64 = *var_ids_acc_dn13_slot;
        let mut var_ids_acc_dn2: f64 = *var_ids_acc_dn2_slot;
        let mut var_ids_acc_dn4: f64 = *var_ids_acc_dn4_slot;
        let mut var_ids_acc_dn5: f64 = *var_ids_acc_dn5_slot;
        let mut var_ids_acc_dn6: f64 = *var_ids_acc_dn6_slot;
        let mut var_ids_acc_dn7: f64 = *var_ids_acc_dn7_slot;
        let mut var_ids_acc_dn8: f64 = *var_ids_acc_dn8_slot;
        let mut var_ids_acc_dn9: f64 = *var_ids_acc_dn9_slot;
        let mut var_ids_acc_rv: f64 = *var_ids_acc_rv_slot;
        let mut var_ids_res: f64 = *var_ids_res_slot;
        let mut var_ids_res_dn0: f64 = *var_ids_res_dn0_slot;
        let mut var_ids_res_dn10: f64 = *var_ids_res_dn10_slot;
        let mut var_ids_res_dn13: f64 = *var_ids_res_dn13_slot;
        let mut var_ids_res_dn2: f64 = *var_ids_res_dn2_slot;
        let mut var_ids_res_dn4: f64 = *var_ids_res_dn4_slot;
        let mut var_ids_res_dn5: f64 = *var_ids_res_dn5_slot;
        let mut var_ids_res_dn6: f64 = *var_ids_res_dn6_slot;
        let mut var_ids_res_dn7: f64 = *var_ids_res_dn7_slot;
        let mut var_ids_res_dn8: f64 = *var_ids_res_dn8_slot;
        let mut var_ids_res_dn9: f64 = *var_ids_res_dn9_slot;
        let mut var_ids_res_rv: f64 = *var_ids_res_rv_slot;
        let mut var_ires_leak: f64 = *var_ires_leak_slot;
        let mut var_ires_leak_dn0: f64 = *var_ires_leak_dn0_slot;
        let mut var_ires_leak_dn10: f64 = *var_ires_leak_dn10_slot;
        let mut var_ires_leak_dn13: f64 = *var_ires_leak_dn13_slot;
        let mut var_ires_leak_dn2: f64 = *var_ires_leak_dn2_slot;
        let mut var_ires_leak_dn4: f64 = *var_ires_leak_dn4_slot;
        let mut var_ires_leak_dn5: f64 = *var_ires_leak_dn5_slot;
        let mut var_ires_leak_dn6: f64 = *var_ires_leak_dn6_slot;
        let mut var_ires_leak_dn7: f64 = *var_ires_leak_dn7_slot;
        let mut var_ires_leak_dn8: f64 = *var_ires_leak_dn8_slot;
        let mut var_ires_leak_dn9: f64 = *var_ires_leak_dn9_slot;
        let mut var_ires_leak_rv: f64 = *var_ires_leak_rv_slot;
        let mut var_js: f64 = *var_js_slot;
        let mut var_js2: f64 = *var_js2_slot;
        let mut var_js2_dn0: f64 = *var_js2_dn0_slot;
        let mut var_js2_dn10: f64 = *var_js2_dn10_slot;
        let mut var_js2_dn13: f64 = *var_js2_dn13_slot;
        let mut var_js2_dn2: f64 = *var_js2_dn2_slot;
        let mut var_js2_dn4: f64 = *var_js2_dn4_slot;
        let mut var_js2_dn5: f64 = *var_js2_dn5_slot;
        let mut var_js2_dn6: f64 = *var_js2_dn6_slot;
        let mut var_js2_dn7: f64 = *var_js2_dn7_slot;
        let mut var_js2_dn8: f64 = *var_js2_dn8_slot;
        let mut var_js2_dn9: f64 = *var_js2_dn9_slot;
        let mut var_js2_rv: f64 = *var_js2_rv_slot;
        let mut var_js_dn0: f64 = *var_js_dn0_slot;
        let mut var_js_dn10: f64 = *var_js_dn10_slot;
        let mut var_js_dn13: f64 = *var_js_dn13_slot;
        let mut var_js_dn2: f64 = *var_js_dn2_slot;
        let mut var_js_dn4: f64 = *var_js_dn4_slot;
        let mut var_js_dn5: f64 = *var_js_dn5_slot;
        let mut var_js_dn6: f64 = *var_js_dn6_slot;
        let mut var_js_dn7: f64 = *var_js_dn7_slot;
        let mut var_js_dn8: f64 = *var_js_dn8_slot;
        let mut var_js_dn9: f64 = *var_js_dn9_slot;
        let mut var_js_rv: f64 = *var_js_rv_slot;
        let mut var_jssw: f64 = *var_jssw_slot;
        let mut var_jssw2: f64 = *var_jssw2_slot;
        let mut var_jssw2_dn0: f64 = *var_jssw2_dn0_slot;
        let mut var_jssw2_dn10: f64 = *var_jssw2_dn10_slot;
        let mut var_jssw2_dn13: f64 = *var_jssw2_dn13_slot;
        let mut var_jssw2_dn2: f64 = *var_jssw2_dn2_slot;
        let mut var_jssw2_dn4: f64 = *var_jssw2_dn4_slot;
        let mut var_jssw2_dn5: f64 = *var_jssw2_dn5_slot;
        let mut var_jssw2_dn6: f64 = *var_jssw2_dn6_slot;
        let mut var_jssw2_dn7: f64 = *var_jssw2_dn7_slot;
        let mut var_jssw2_dn8: f64 = *var_jssw2_dn8_slot;
        let mut var_jssw2_dn9: f64 = *var_jssw2_dn9_slot;
        let mut var_jssw2_rv: f64 = *var_jssw2_rv_slot;
        let mut var_jssw_dn0: f64 = *var_jssw_dn0_slot;
        let mut var_jssw_dn10: f64 = *var_jssw_dn10_slot;
        let mut var_jssw_dn13: f64 = *var_jssw_dn13_slot;
        let mut var_jssw_dn2: f64 = *var_jssw_dn2_slot;
        let mut var_jssw_dn4: f64 = *var_jssw_dn4_slot;
        let mut var_jssw_dn5: f64 = *var_jssw_dn5_slot;
        let mut var_jssw_dn6: f64 = *var_jssw_dn6_slot;
        let mut var_jssw_dn7: f64 = *var_jssw_dn7_slot;
        let mut var_jssw_dn8: f64 = *var_jssw_dn8_slot;
        let mut var_jssw_dn9: f64 = *var_jssw_dn9_slot;
        let mut var_jssw_rv: f64 = *var_jssw_rv_slot;
        let mut var_lp_s0_max: f64 = *var_lp_s0_max_slot;
        let mut var_lp_s0_max_rv: f64 = *var_lp_s0_max_rv_slot;
        let mut var_mphn0: f64 = *var_mphn0_slot;
        let mut var_mphn0_dn0: f64 = *var_mphn0_dn0_slot;
        let mut var_mphn0_dn10: f64 = *var_mphn0_dn10_slot;
        let mut var_mphn0_dn13: f64 = *var_mphn0_dn13_slot;
        let mut var_mphn0_dn2: f64 = *var_mphn0_dn2_slot;
        let mut var_mphn0_dn4: f64 = *var_mphn0_dn4_slot;
        let mut var_mphn0_dn5: f64 = *var_mphn0_dn5_slot;
        let mut var_mphn0_dn6: f64 = *var_mphn0_dn6_slot;
        let mut var_mphn0_dn7: f64 = *var_mphn0_dn7_slot;
        let mut var_mphn0_dn8: f64 = *var_mphn0_dn8_slot;
        let mut var_mphn0_dn9: f64 = *var_mphn0_dn9_slot;
        let mut var_mphn0_rv: f64 = *var_mphn0_rv_slot;
        let mut var_ninvde: f64 = *var_ninvde_slot;
        let mut var_ninvde_dn0: f64 = *var_ninvde_dn0_slot;
        let mut var_ninvde_dn10: f64 = *var_ninvde_dn10_slot;
        let mut var_ninvde_dn13: f64 = *var_ninvde_dn13_slot;
        let mut var_ninvde_dn2: f64 = *var_ninvde_dn2_slot;
        let mut var_ninvde_dn4: f64 = *var_ninvde_dn4_slot;
        let mut var_ninvde_dn5: f64 = *var_ninvde_dn5_slot;
        let mut var_ninvde_dn6: f64 = *var_ninvde_dn6_slot;
        let mut var_ninvde_dn7: f64 = *var_ninvde_dn7_slot;
        let mut var_ninvde_dn8: f64 = *var_ninvde_dn8_slot;
        let mut var_ninvde_dn9: f64 = *var_ninvde_dn9_slot;
        let mut var_ninvde_rv: f64 = *var_ninvde_rv_slot;
        let mut var_ninvdecres: f64 = *var_ninvdecres_slot;
        let mut var_ninvdecres_dn0: f64 = *var_ninvdecres_dn0_slot;
        let mut var_ninvdecres_dn10: f64 = *var_ninvdecres_dn10_slot;
        let mut var_ninvdecres_dn13: f64 = *var_ninvdecres_dn13_slot;
        let mut var_ninvdecres_dn2: f64 = *var_ninvdecres_dn2_slot;
        let mut var_ninvdecres_dn4: f64 = *var_ninvdecres_dn4_slot;
        let mut var_ninvdecres_dn5: f64 = *var_ninvdecres_dn5_slot;
        let mut var_ninvdecres_dn6: f64 = *var_ninvdecres_dn6_slot;
        let mut var_ninvdecres_dn7: f64 = *var_ninvdecres_dn7_slot;
        let mut var_ninvdecres_dn8: f64 = *var_ninvdecres_dn8_slot;
        let mut var_ninvdecres_dn9: f64 = *var_ninvdecres_dn9_slot;
        let mut var_ninvdecres_rv: f64 = *var_ninvdecres_rv_slot;
        let mut var_ninvdehres: f64 = *var_ninvdehres_slot;
        let mut var_ninvdehres_dn0: f64 = *var_ninvdehres_dn0_slot;
        let mut var_ninvdehres_dn10: f64 = *var_ninvdehres_dn10_slot;
        let mut var_ninvdehres_dn13: f64 = *var_ninvdehres_dn13_slot;
        let mut var_ninvdehres_dn2: f64 = *var_ninvdehres_dn2_slot;
        let mut var_ninvdehres_dn4: f64 = *var_ninvdehres_dn4_slot;
        let mut var_ninvdehres_dn5: f64 = *var_ninvdehres_dn5_slot;
        let mut var_ninvdehres_dn6: f64 = *var_ninvdehres_dn6_slot;
        let mut var_ninvdehres_dn7: f64 = *var_ninvdehres_dn7_slot;
        let mut var_ninvdehres_dn8: f64 = *var_ninvdehres_dn8_slot;
        let mut var_ninvdehres_dn9: f64 = *var_ninvdehres_dn9_slot;
        let mut var_ninvdehres_rv: f64 = *var_ninvdehres_rv_slot;
        let mut var_pb2n: f64 = *var_pb2n_slot;
        let mut var_pb2n_dn0: f64 = *var_pb2n_dn0_slot;
        let mut var_pb2n_dn10: f64 = *var_pb2n_dn10_slot;
        let mut var_pb2n_dn13: f64 = *var_pb2n_dn13_slot;
        let mut var_pb2n_dn2: f64 = *var_pb2n_dn2_slot;
        let mut var_pb2n_dn4: f64 = *var_pb2n_dn4_slot;
        let mut var_pb2n_dn5: f64 = *var_pb2n_dn5_slot;
        let mut var_pb2n_dn6: f64 = *var_pb2n_dn6_slot;
        let mut var_pb2n_dn7: f64 = *var_pb2n_dn7_slot;
        let mut var_pb2n_dn8: f64 = *var_pb2n_dn8_slot;
        let mut var_pb2n_dn9: f64 = *var_pb2n_dn9_slot;
        let mut var_pb2n_rv: f64 = *var_pb2n_rv_slot;
        let mut var_powratio: f64 = *var_powratio_slot;
        let mut var_powratio_dn0: f64 = *var_powratio_dn0_slot;
        let mut var_powratio_dn10: f64 = *var_powratio_dn10_slot;
        let mut var_powratio_dn13: f64 = *var_powratio_dn13_slot;
        let mut var_powratio_dn2: f64 = *var_powratio_dn2_slot;
        let mut var_powratio_dn4: f64 = *var_powratio_dn4_slot;
        let mut var_powratio_dn5: f64 = *var_powratio_dn5_slot;
        let mut var_powratio_dn6: f64 = *var_powratio_dn6_slot;
        let mut var_powratio_dn7: f64 = *var_powratio_dn7_slot;
        let mut var_powratio_dn8: f64 = *var_powratio_dn8_slot;
        let mut var_powratio_dn9: f64 = *var_powratio_dn9_slot;
        let mut var_powratio_rv: f64 = *var_powratio_rv_slot;
        let mut var_ptovr: f64 = *var_ptovr_slot;
        let mut var_ptovr_dn0: f64 = *var_ptovr_dn0_slot;
        let mut var_ptovr_dn10: f64 = *var_ptovr_dn10_slot;
        let mut var_ptovr_dn13: f64 = *var_ptovr_dn13_slot;
        let mut var_ptovr_dn2: f64 = *var_ptovr_dn2_slot;
        let mut var_ptovr_dn4: f64 = *var_ptovr_dn4_slot;
        let mut var_ptovr_dn5: f64 = *var_ptovr_dn5_slot;
        let mut var_ptovr_dn6: f64 = *var_ptovr_dn6_slot;
        let mut var_ptovr_dn7: f64 = *var_ptovr_dn7_slot;
        let mut var_ptovr_dn8: f64 = *var_ptovr_dn8_slot;
        let mut var_ptovr_dn9: f64 = *var_ptovr_dn9_slot;
        let mut var_ptovr_rv: f64 = *var_ptovr_rv_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn0: f64 = *var_qbs_dn0_slot;
        let mut var_qbs_dn10: f64 = *var_qbs_dn10_slot;
        let mut var_qbs_dn13: f64 = *var_qbs_dn13_slot;
        let mut var_qbs_dn2: f64 = *var_qbs_dn2_slot;
        let mut var_qbs_dn4: f64 = *var_qbs_dn4_slot;
        let mut var_qbs_dn5: f64 = *var_qbs_dn5_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_qbs_dn8: f64 = *var_qbs_dn8_slot;
        let mut var_qbs_dn9: f64 = *var_qbs_dn9_slot;
        let mut var_qbs_rv: f64 = *var_qbs_rv_slot;
        let mut var_qiu_noi: f64 = *var_qiu_noi_slot;
        let mut var_qiu_noi_dn0: f64 = *var_qiu_noi_dn0_slot;
        let mut var_qiu_noi_dn10: f64 = *var_qiu_noi_dn10_slot;
        let mut var_qiu_noi_dn13: f64 = *var_qiu_noi_dn13_slot;
        let mut var_qiu_noi_dn2: f64 = *var_qiu_noi_dn2_slot;
        let mut var_qiu_noi_dn4: f64 = *var_qiu_noi_dn4_slot;
        let mut var_qiu_noi_dn5: f64 = *var_qiu_noi_dn5_slot;
        let mut var_qiu_noi_dn6: f64 = *var_qiu_noi_dn6_slot;
        let mut var_qiu_noi_dn7: f64 = *var_qiu_noi_dn7_slot;
        let mut var_qiu_noi_dn8: f64 = *var_qiu_noi_dn8_slot;
        let mut var_qiu_noi_dn9: f64 = *var_qiu_noi_dn9_slot;
        let mut var_qiu_noi_rv: f64 = *var_qiu_noi_rv_slot;
        let mut var_rde: f64 = *var_rde_slot;
        let mut var_rde_dn0: f64 = *var_rde_dn0_slot;
        let mut var_rde_dn10: f64 = *var_rde_dn10_slot;
        let mut var_rde_dn13: f64 = *var_rde_dn13_slot;
        let mut var_rde_dn2: f64 = *var_rde_dn2_slot;
        let mut var_rde_dn4: f64 = *var_rde_dn4_slot;
        let mut var_rde_dn5: f64 = *var_rde_dn5_slot;
        let mut var_rde_dn6: f64 = *var_rde_dn6_slot;
        let mut var_rde_dn7: f64 = *var_rde_dn7_slot;
        let mut var_rde_dn8: f64 = *var_rde_dn8_slot;
        let mut var_rde_dn9: f64 = *var_rde_dn9_slot;
        let mut var_rde_rv: f64 = *var_rde_rv_slot;
        let mut var_rdvde: f64 = *var_rdvde_slot;
        let mut var_rdvde_dn0: f64 = *var_rdvde_dn0_slot;
        let mut var_rdvde_dn10: f64 = *var_rdvde_dn10_slot;
        let mut var_rdvde_dn13: f64 = *var_rdvde_dn13_slot;
        let mut var_rdvde_dn2: f64 = *var_rdvde_dn2_slot;
        let mut var_rdvde_dn4: f64 = *var_rdvde_dn4_slot;
        let mut var_rdvde_dn5: f64 = *var_rdvde_dn5_slot;
        let mut var_rdvde_dn6: f64 = *var_rdvde_dn6_slot;
        let mut var_rdvde_dn7: f64 = *var_rdvde_dn7_slot;
        let mut var_rdvde_dn8: f64 = *var_rdvde_dn8_slot;
        let mut var_rdvde_dn9: f64 = *var_rdvde_dn9_slot;
        let mut var_rdvde_rv: f64 = *var_rdvde_rv_slot;
        let mut var_rrdrmue: f64 = *var_rrdrmue_slot;
        let mut var_rrdrmue_dn0: f64 = *var_rrdrmue_dn0_slot;
        let mut var_rrdrmue_dn10: f64 = *var_rrdrmue_dn10_slot;
        let mut var_rrdrmue_dn13: f64 = *var_rrdrmue_dn13_slot;
        let mut var_rrdrmue_dn2: f64 = *var_rrdrmue_dn2_slot;
        let mut var_rrdrmue_dn4: f64 = *var_rrdrmue_dn4_slot;
        let mut var_rrdrmue_dn5: f64 = *var_rrdrmue_dn5_slot;
        let mut var_rrdrmue_dn6: f64 = *var_rrdrmue_dn6_slot;
        let mut var_rrdrmue_dn7: f64 = *var_rrdrmue_dn7_slot;
        let mut var_rrdrmue_dn8: f64 = *var_rrdrmue_dn8_slot;
        let mut var_rrdrmue_dn9: f64 = *var_rrdrmue_dn9_slot;
        let mut var_rrdrmue_rv: f64 = *var_rrdrmue_rv_slot;
        let mut var_rrdrmues: f64 = *var_rrdrmues_slot;
        let mut var_rrdrmues_dn0: f64 = *var_rrdrmues_dn0_slot;
        let mut var_rrdrmues_dn10: f64 = *var_rrdrmues_dn10_slot;
        let mut var_rrdrmues_dn13: f64 = *var_rrdrmues_dn13_slot;
        let mut var_rrdrmues_dn2: f64 = *var_rrdrmues_dn2_slot;
        let mut var_rrdrmues_dn4: f64 = *var_rrdrmues_dn4_slot;
        let mut var_rrdrmues_dn5: f64 = *var_rrdrmues_dn5_slot;
        let mut var_rrdrmues_dn6: f64 = *var_rrdrmues_dn6_slot;
        let mut var_rrdrmues_dn7: f64 = *var_rrdrmues_dn7_slot;
        let mut var_rrdrmues_dn8: f64 = *var_rrdrmues_dn8_slot;
        let mut var_rrdrmues_dn9: f64 = *var_rrdrmues_dn9_slot;
        let mut var_rrdrmues_rv: f64 = *var_rrdrmues_rv_slot;
        let mut var_rrdrvmax: f64 = *var_rrdrvmax_slot;
        let mut var_rrdrvmax_dn0: f64 = *var_rrdrvmax_dn0_slot;
        let mut var_rrdrvmax_dn10: f64 = *var_rrdrvmax_dn10_slot;
        let mut var_rrdrvmax_dn13: f64 = *var_rrdrvmax_dn13_slot;
        let mut var_rrdrvmax_dn2: f64 = *var_rrdrvmax_dn2_slot;
        let mut var_rrdrvmax_dn4: f64 = *var_rrdrvmax_dn4_slot;
        let mut var_rrdrvmax_dn5: f64 = *var_rrdrvmax_dn5_slot;
        let mut var_rrdrvmax_dn6: f64 = *var_rrdrvmax_dn6_slot;
        let mut var_rrdrvmax_dn7: f64 = *var_rrdrvmax_dn7_slot;
        let mut var_rrdrvmax_dn8: f64 = *var_rrdrvmax_dn8_slot;
        let mut var_rrdrvmax_dn9: f64 = *var_rrdrvmax_dn9_slot;
        let mut var_rrdrvmax_rv: f64 = *var_rrdrvmax_rv_slot;
        let mut var_rrdrvmaxs: f64 = *var_rrdrvmaxs_slot;
        let mut var_rrdrvmaxs_dn0: f64 = *var_rrdrvmaxs_dn0_slot;
        let mut var_rrdrvmaxs_dn10: f64 = *var_rrdrvmaxs_dn10_slot;
        let mut var_rrdrvmaxs_dn13: f64 = *var_rrdrvmaxs_dn13_slot;
        let mut var_rrdrvmaxs_dn2: f64 = *var_rrdrvmaxs_dn2_slot;
        let mut var_rrdrvmaxs_dn4: f64 = *var_rrdrvmaxs_dn4_slot;
        let mut var_rrdrvmaxs_dn5: f64 = *var_rrdrvmaxs_dn5_slot;
        let mut var_rrdrvmaxs_dn6: f64 = *var_rrdrvmaxs_dn6_slot;
        let mut var_rrdrvmaxs_dn7: f64 = *var_rrdrvmaxs_dn7_slot;
        let mut var_rrdrvmaxs_dn8: f64 = *var_rrdrvmaxs_dn8_slot;
        let mut var_rrdrvmaxs_dn9: f64 = *var_rrdrvmaxs_dn9_slot;
        let mut var_rrdrvmaxs_rv: f64 = *var_rrdrvmaxs_rv_slot;
        let mut var_rse: f64 = *var_rse_slot;
        let mut var_rse_dn0: f64 = *var_rse_dn0_slot;
        let mut var_rse_dn10: f64 = *var_rse_dn10_slot;
        let mut var_rse_dn13: f64 = *var_rse_dn13_slot;
        let mut var_rse_dn2: f64 = *var_rse_dn2_slot;
        let mut var_rse_dn4: f64 = *var_rse_dn4_slot;
        let mut var_rse_dn5: f64 = *var_rse_dn5_slot;
        let mut var_rse_dn6: f64 = *var_rse_dn6_slot;
        let mut var_rse_dn7: f64 = *var_rse_dn7_slot;
        let mut var_rse_dn8: f64 = *var_rse_dn8_slot;
        let mut var_rse_dn9: f64 = *var_rse_dn9_slot;
        let mut var_rse_rv: f64 = *var_rse_rv_slot;
        let mut var_rsvde: f64 = *var_rsvde_slot;
        let mut var_rsvde_dn0: f64 = *var_rsvde_dn0_slot;
        let mut var_rsvde_dn10: f64 = *var_rsvde_dn10_slot;
        let mut var_rsvde_dn13: f64 = *var_rsvde_dn13_slot;
        let mut var_rsvde_dn2: f64 = *var_rsvde_dn2_slot;
        let mut var_rsvde_dn4: f64 = *var_rsvde_dn4_slot;
        let mut var_rsvde_dn5: f64 = *var_rsvde_dn5_slot;
        let mut var_rsvde_dn6: f64 = *var_rsvde_dn6_slot;
        let mut var_rsvde_dn7: f64 = *var_rsvde_dn7_slot;
        let mut var_rsvde_dn8: f64 = *var_rsvde_dn8_slot;
        let mut var_rsvde_dn9: f64 = *var_rsvde_dn9_slot;
        let mut var_rsvde_rv: f64 = *var_rsvde_rv_slot;
        let mut var_sqrt_eg: f64 = *var_sqrt_eg_slot;
        let mut var_sqrt_eg_dn0: f64 = *var_sqrt_eg_dn0_slot;
        let mut var_sqrt_eg_dn10: f64 = *var_sqrt_eg_dn10_slot;
        let mut var_sqrt_eg_dn13: f64 = *var_sqrt_eg_dn13_slot;
        let mut var_sqrt_eg_dn2: f64 = *var_sqrt_eg_dn2_slot;
        let mut var_sqrt_eg_dn4: f64 = *var_sqrt_eg_dn4_slot;
        let mut var_sqrt_eg_dn5: f64 = *var_sqrt_eg_dn5_slot;
        let mut var_sqrt_eg_dn6: f64 = *var_sqrt_eg_dn6_slot;
        let mut var_sqrt_eg_dn7: f64 = *var_sqrt_eg_dn7_slot;
        let mut var_sqrt_eg_dn8: f64 = *var_sqrt_eg_dn8_slot;
        let mut var_sqrt_eg_dn9: f64 = *var_sqrt_eg_dn9_slot;
        let mut var_sqrt_eg_rv: f64 = *var_sqrt_eg_rv_slot;
        let mut var_tratio: f64 = *var_tratio_slot;
        let mut var_tratio_dn0: f64 = *var_tratio_dn0_slot;
        let mut var_tratio_dn10: f64 = *var_tratio_dn10_slot;
        let mut var_tratio_dn13: f64 = *var_tratio_dn13_slot;
        let mut var_tratio_dn2: f64 = *var_tratio_dn2_slot;
        let mut var_tratio_dn4: f64 = *var_tratio_dn4_slot;
        let mut var_tratio_dn5: f64 = *var_tratio_dn5_slot;
        let mut var_tratio_dn6: f64 = *var_tratio_dn6_slot;
        let mut var_tratio_dn7: f64 = *var_tratio_dn7_slot;
        let mut var_tratio_dn8: f64 = *var_tratio_dn8_slot;
        let mut var_tratio_dn9: f64 = *var_tratio_dn9_slot;
        let mut var_tratio_rv: f64 = *var_tratio_rv_slot;
        let mut var_uc_rdrbb: f64 = *var_uc_rdrbb_slot;
        let mut var_uc_rdrbb_dn0: f64 = *var_uc_rdrbb_dn0_slot;
        let mut var_uc_rdrbb_dn10: f64 = *var_uc_rdrbb_dn10_slot;
        let mut var_uc_rdrbb_dn13: f64 = *var_uc_rdrbb_dn13_slot;
        let mut var_uc_rdrbb_dn2: f64 = *var_uc_rdrbb_dn2_slot;
        let mut var_uc_rdrbb_dn4: f64 = *var_uc_rdrbb_dn4_slot;
        let mut var_uc_rdrbb_dn5: f64 = *var_uc_rdrbb_dn5_slot;
        let mut var_uc_rdrbb_dn6: f64 = *var_uc_rdrbb_dn6_slot;
        let mut var_uc_rdrbb_dn7: f64 = *var_uc_rdrbb_dn7_slot;
        let mut var_uc_rdrbb_dn8: f64 = *var_uc_rdrbb_dn8_slot;
        let mut var_uc_rdrbb_dn9: f64 = *var_uc_rdrbb_dn9_slot;
        let mut var_uc_rdrbb_rv: f64 = *var_uc_rdrbb_rv_slot;
        let mut var_uc_rdrbb_s: f64 = *var_uc_rdrbb_s_slot;
        let mut var_uc_rdrbb_s_dn0: f64 = *var_uc_rdrbb_s_dn0_slot;
        let mut var_uc_rdrbb_s_dn10: f64 = *var_uc_rdrbb_s_dn10_slot;
        let mut var_uc_rdrbb_s_dn13: f64 = *var_uc_rdrbb_s_dn13_slot;
        let mut var_uc_rdrbb_s_dn2: f64 = *var_uc_rdrbb_s_dn2_slot;
        let mut var_uc_rdrbb_s_dn4: f64 = *var_uc_rdrbb_s_dn4_slot;
        let mut var_uc_rdrbb_s_dn5: f64 = *var_uc_rdrbb_s_dn5_slot;
        let mut var_uc_rdrbb_s_dn6: f64 = *var_uc_rdrbb_s_dn6_slot;
        let mut var_uc_rdrbb_s_dn7: f64 = *var_uc_rdrbb_s_dn7_slot;
        let mut var_uc_rdrbb_s_dn8: f64 = *var_uc_rdrbb_s_dn8_slot;
        let mut var_uc_rdrbb_s_dn9: f64 = *var_uc_rdrbb_s_dn9_slot;
        let mut var_uc_rdrbb_s_rv: f64 = *var_uc_rdrbb_s_rv_slot;
        let mut var_uc_subtmp: f64 = *var_uc_subtmp_slot;
        let mut var_uc_subtmp_rv: f64 = *var_uc_subtmp_rv_slot;
        let mut var_vbipn: f64 = *var_vbipn_slot;
        let mut var_vbipn_dn0: f64 = *var_vbipn_dn0_slot;
        let mut var_vbipn_dn10: f64 = *var_vbipn_dn10_slot;
        let mut var_vbipn_dn13: f64 = *var_vbipn_dn13_slot;
        let mut var_vbipn_dn2: f64 = *var_vbipn_dn2_slot;
        let mut var_vbipn_dn4: f64 = *var_vbipn_dn4_slot;
        let mut var_vbipn_dn5: f64 = *var_vbipn_dn5_slot;
        let mut var_vbipn_dn6: f64 = *var_vbipn_dn6_slot;
        let mut var_vbipn_dn7: f64 = *var_vbipn_dn7_slot;
        let mut var_vbipn_dn8: f64 = *var_vbipn_dn8_slot;
        let mut var_vbipn_dn9: f64 = *var_vbipn_dn9_slot;
        let mut var_vbipn_rv: f64 = *var_vbipn_rv_slot;
        let mut var_vmaxeff: f64 = *var_vmaxeff_slot;
        let mut var_vmaxeff_dn0: f64 = *var_vmaxeff_dn0_slot;
        let mut var_vmaxeff_dn10: f64 = *var_vmaxeff_dn10_slot;
        let mut var_vmaxeff_dn13: f64 = *var_vmaxeff_dn13_slot;
        let mut var_vmaxeff_dn2: f64 = *var_vmaxeff_dn2_slot;
        let mut var_vmaxeff_dn4: f64 = *var_vmaxeff_dn4_slot;
        let mut var_vmaxeff_dn5: f64 = *var_vmaxeff_dn5_slot;
        let mut var_vmaxeff_dn6: f64 = *var_vmaxeff_dn6_slot;
        let mut var_vmaxeff_dn7: f64 = *var_vmaxeff_dn7_slot;
        let mut var_vmaxeff_dn8: f64 = *var_vmaxeff_dn8_slot;
        let mut var_vmaxeff_dn9: f64 = *var_vmaxeff_dn9_slot;
        let mut var_vmaxeff_rv: f64 = *var_vmaxeff_rv_slot;
        let mut var_wdpl: f64 = *var_wdpl_slot;
        let mut var_wdpl_dn0: f64 = *var_wdpl_dn0_slot;
        let mut var_wdpl_dn10: f64 = *var_wdpl_dn10_slot;
        let mut var_wdpl_dn13: f64 = *var_wdpl_dn13_slot;
        let mut var_wdpl_dn2: f64 = *var_wdpl_dn2_slot;
        let mut var_wdpl_dn4: f64 = *var_wdpl_dn4_slot;
        let mut var_wdpl_dn5: f64 = *var_wdpl_dn5_slot;
        let mut var_wdpl_dn6: f64 = *var_wdpl_dn6_slot;
        let mut var_wdpl_dn7: f64 = *var_wdpl_dn7_slot;
        let mut var_wdpl_dn8: f64 = *var_wdpl_dn8_slot;
        let mut var_wdpl_dn9: f64 = *var_wdpl_dn9_slot;
        let mut var_wdpl_rv: f64 = *var_wdpl_rv_slot;
        let mut var_wdplp: f64 = *var_wdplp_slot;
        let mut var_wdplp_dn0: f64 = *var_wdplp_dn0_slot;
        let mut var_wdplp_dn10: f64 = *var_wdplp_dn10_slot;
        let mut var_wdplp_dn13: f64 = *var_wdplp_dn13_slot;
        let mut var_wdplp_dn2: f64 = *var_wdplp_dn2_slot;
        let mut var_wdplp_dn4: f64 = *var_wdplp_dn4_slot;
        let mut var_wdplp_dn5: f64 = *var_wdplp_dn5_slot;
        let mut var_wdplp_dn6: f64 = *var_wdplp_dn6_slot;
        let mut var_wdplp_dn7: f64 = *var_wdplp_dn7_slot;
        let mut var_wdplp_dn8: f64 = *var_wdplp_dn8_slot;
        let mut var_wdplp_dn9: f64 = *var_wdplp_dn9_slot;
        let mut var_wdplp_rv: f64 = *var_wdplp_rv_slot;

        var_cgsb = 0.0;
        var_cgsb_dn0 = 0.0;
        var_cgsb_dn2 = 0.0;
        var_cgsb_dn4 = 0.0;
        var_cgsb_dn5 = 0.0;
        var_cgsb_dn6 = 0.0;
        var_cgsb_dn7 = 0.0;
        var_cgsb_dn8 = 0.0;
        var_cgsb_dn9 = 0.0;
        var_cgsb_dn10 = 0.0;
        var_cgsb_dn13 = 0.0;
        var_cgsb_rv = 0.0;

        var_ninvde = 0.0;
        var_ninvde_dn0 = 0.0;
        var_ninvde_dn2 = 0.0;
        var_ninvde_dn4 = 0.0;
        var_ninvde_dn5 = 0.0;
        var_ninvde_dn6 = 0.0;
        var_ninvde_dn7 = 0.0;
        var_ninvde_dn8 = 0.0;
        var_ninvde_dn9 = 0.0;
        var_ninvde_dn10 = 0.0;
        var_ninvde_dn13 = 0.0;
        var_ninvde_rv = 0.0;

        var_ninvdecres = 0.0;
        var_ninvdecres_dn0 = 0.0;
        var_ninvdecres_dn2 = 0.0;
        var_ninvdecres_dn4 = 0.0;
        var_ninvdecres_dn5 = 0.0;
        var_ninvdecres_dn6 = 0.0;
        var_ninvdecres_dn7 = 0.0;
        var_ninvdecres_dn8 = 0.0;
        var_ninvdecres_dn9 = 0.0;
        var_ninvdecres_dn10 = 0.0;
        var_ninvdecres_dn13 = 0.0;
        var_ninvdecres_rv = 0.0;

        var_ninvdehres = 0.0;
        var_ninvdehres_dn0 = 0.0;
        var_ninvdehres_dn2 = 0.0;
        var_ninvdehres_dn4 = 0.0;
        var_ninvdehres_dn5 = 0.0;
        var_ninvdehres_dn6 = 0.0;
        var_ninvdehres_dn7 = 0.0;
        var_ninvdehres_dn8 = 0.0;
        var_ninvdehres_dn9 = 0.0;
        var_ninvdehres_dn10 = 0.0;
        var_ninvdehres_dn13 = 0.0;
        var_ninvdehres_rv = 0.0;

        var_rrdrmue = 0.0;
        var_rrdrmue_dn0 = 0.0;
        var_rrdrmue_dn2 = 0.0;
        var_rrdrmue_dn4 = 0.0;
        var_rrdrmue_dn5 = 0.0;
        var_rrdrmue_dn6 = 0.0;
        var_rrdrmue_dn7 = 0.0;
        var_rrdrmue_dn8 = 0.0;
        var_rrdrmue_dn9 = 0.0;
        var_rrdrmue_dn10 = 0.0;
        var_rrdrmue_dn13 = 0.0;
        var_rrdrmue_rv = 0.0;

        var_rrdrmues = 0.0;
        var_rrdrmues_dn0 = 0.0;
        var_rrdrmues_dn2 = 0.0;
        var_rrdrmues_dn4 = 0.0;
        var_rrdrmues_dn5 = 0.0;
        var_rrdrmues_dn6 = 0.0;
        var_rrdrmues_dn7 = 0.0;
        var_rrdrmues_dn8 = 0.0;
        var_rrdrmues_dn9 = 0.0;
        var_rrdrmues_dn10 = 0.0;
        var_rrdrmues_dn13 = 0.0;
        var_rrdrmues_rv = 0.0;

        var_rrdrvmax = 0.0;
        var_rrdrvmax_dn0 = 0.0;
        var_rrdrvmax_dn2 = 0.0;
        var_rrdrvmax_dn4 = 0.0;
        var_rrdrvmax_dn5 = 0.0;
        var_rrdrvmax_dn6 = 0.0;
        var_rrdrvmax_dn7 = 0.0;
        var_rrdrvmax_dn8 = 0.0;
        var_rrdrvmax_dn9 = 0.0;
        var_rrdrvmax_dn10 = 0.0;
        var_rrdrvmax_dn13 = 0.0;
        var_rrdrvmax_rv = 0.0;

        var_rde = 0.0;
        var_rde_dn0 = 0.0;
        var_rde_dn2 = 0.0;
        var_rde_dn4 = 0.0;
        var_rde_dn5 = 0.0;
        var_rde_dn6 = 0.0;
        var_rde_dn7 = 0.0;
        var_rde_dn8 = 0.0;
        var_rde_dn9 = 0.0;
        var_rde_dn10 = 0.0;
        var_rde_dn13 = 0.0;
        var_rde_rv = 0.0;

        var_rdvde = 0.0;
        var_rdvde_dn0 = 0.0;
        var_rdvde_dn2 = 0.0;
        var_rdvde_dn4 = 0.0;
        var_rdvde_dn5 = 0.0;
        var_rdvde_dn6 = 0.0;
        var_rdvde_dn7 = 0.0;
        var_rdvde_dn8 = 0.0;
        var_rdvde_dn9 = 0.0;
        var_rdvde_dn10 = 0.0;
        var_rdvde_dn13 = 0.0;
        var_rdvde_rv = 0.0;

        var_rse = 0.0;
        var_rse_dn0 = 0.0;
        var_rse_dn2 = 0.0;
        var_rse_dn4 = 0.0;
        var_rse_dn5 = 0.0;
        var_rse_dn6 = 0.0;
        var_rse_dn7 = 0.0;
        var_rse_dn8 = 0.0;
        var_rse_dn9 = 0.0;
        var_rse_dn10 = 0.0;
        var_rse_dn13 = 0.0;
        var_rse_rv = 0.0;

        var_rsvde = 0.0;
        var_rsvde_dn0 = 0.0;
        var_rsvde_dn2 = 0.0;
        var_rsvde_dn4 = 0.0;
        var_rsvde_dn5 = 0.0;
        var_rsvde_dn6 = 0.0;
        var_rsvde_dn7 = 0.0;
        var_rsvde_dn8 = 0.0;
        var_rsvde_dn9 = 0.0;
        var_rsvde_dn10 = 0.0;
        var_rsvde_dn13 = 0.0;
        var_rsvde_rv = 0.0;

        var_rrdrvmaxs = 0.0;
        var_rrdrvmaxs_dn0 = 0.0;
        var_rrdrvmaxs_dn2 = 0.0;
        var_rrdrvmaxs_dn4 = 0.0;
        var_rrdrvmaxs_dn5 = 0.0;
        var_rrdrvmaxs_dn6 = 0.0;
        var_rrdrvmaxs_dn7 = 0.0;
        var_rrdrvmaxs_dn8 = 0.0;
        var_rrdrvmaxs_dn9 = 0.0;
        var_rrdrvmaxs_dn10 = 0.0;
        var_rrdrvmaxs_dn13 = 0.0;
        var_rrdrvmaxs_rv = 0.0;

        var_tratio = 0.0;
        var_tratio_dn0 = 0.0;
        var_tratio_dn2 = 0.0;
        var_tratio_dn4 = 0.0;
        var_tratio_dn5 = 0.0;
        var_tratio_dn6 = 0.0;
        var_tratio_dn7 = 0.0;
        var_tratio_dn8 = 0.0;
        var_tratio_dn9 = 0.0;
        var_tratio_dn10 = 0.0;
        var_tratio_dn13 = 0.0;
        var_tratio_rv = 0.0;

        var_vmaxeff = 0.0;
        var_vmaxeff_dn0 = 0.0;
        var_vmaxeff_dn2 = 0.0;
        var_vmaxeff_dn4 = 0.0;
        var_vmaxeff_dn5 = 0.0;
        var_vmaxeff_dn6 = 0.0;
        var_vmaxeff_dn7 = 0.0;
        var_vmaxeff_dn8 = 0.0;
        var_vmaxeff_dn9 = 0.0;
        var_vmaxeff_dn10 = 0.0;
        var_vmaxeff_dn13 = 0.0;
        var_vmaxeff_rv = 0.0;

        var_betatnom = 0.0;
        var_betatnom_rv = 0.0;

        var_cnst0over = 0.0;
        var_cnst0over_dn0 = 0.0;
        var_cnst0over_dn2 = 0.0;
        var_cnst0over_dn4 = 0.0;
        var_cnst0over_dn5 = 0.0;
        var_cnst0over_dn6 = 0.0;
        var_cnst0over_dn7 = 0.0;
        var_cnst0over_dn8 = 0.0;
        var_cnst0over_dn9 = 0.0;
        var_cnst0over_dn10 = 0.0;
        var_cnst0over_dn13 = 0.0;
        var_cnst0over_rv = 0.0;

        var_cnst0overs = 0.0;
        var_cnst0overs_dn0 = 0.0;
        var_cnst0overs_dn2 = 0.0;
        var_cnst0overs_dn4 = 0.0;
        var_cnst0overs_dn5 = 0.0;
        var_cnst0overs_dn6 = 0.0;
        var_cnst0overs_dn7 = 0.0;
        var_cnst0overs_dn8 = 0.0;
        var_cnst0overs_dn9 = 0.0;
        var_cnst0overs_dn10 = 0.0;
        var_cnst0overs_dn13 = 0.0;
        var_cnst0overs_rv = 0.0;

        var_costi0_p2 = 0.0;
        var_costi0_p2_dn0 = 0.0;
        var_costi0_p2_dn2 = 0.0;
        var_costi0_p2_dn4 = 0.0;
        var_costi0_p2_dn5 = 0.0;
        var_costi0_p2_dn6 = 0.0;
        var_costi0_p2_dn7 = 0.0;
        var_costi0_p2_dn8 = 0.0;
        var_costi0_p2_dn9 = 0.0;
        var_costi0_p2_dn10 = 0.0;
        var_costi0_p2_dn13 = 0.0;
        var_costi0_p2_rv = 0.0;

        var_mphn0 = 0.0;
        var_mphn0_dn0 = 0.0;
        var_mphn0_dn2 = 0.0;
        var_mphn0_dn4 = 0.0;
        var_mphn0_dn5 = 0.0;
        var_mphn0_dn6 = 0.0;
        var_mphn0_dn7 = 0.0;
        var_mphn0_dn8 = 0.0;
        var_mphn0_dn9 = 0.0;
        var_mphn0_dn10 = 0.0;
        var_mphn0_dn13 = 0.0;
        var_mphn0_rv = 0.0;

        var_powratio = 0.0;
        var_powratio_dn0 = 0.0;
        var_powratio_dn2 = 0.0;
        var_powratio_dn4 = 0.0;
        var_powratio_dn5 = 0.0;
        var_powratio_dn6 = 0.0;
        var_powratio_dn7 = 0.0;
        var_powratio_dn8 = 0.0;
        var_powratio_dn9 = 0.0;
        var_powratio_dn10 = 0.0;
        var_powratio_dn13 = 0.0;
        var_powratio_rv = 0.0;

        var_ptovr = 0.0;
        var_ptovr_dn0 = 0.0;
        var_ptovr_dn2 = 0.0;
        var_ptovr_dn4 = 0.0;
        var_ptovr_dn5 = 0.0;
        var_ptovr_dn6 = 0.0;
        var_ptovr_dn7 = 0.0;
        var_ptovr_dn8 = 0.0;
        var_ptovr_dn9 = 0.0;
        var_ptovr_dn10 = 0.0;
        var_ptovr_dn13 = 0.0;
        var_ptovr_rv = 0.0;

        var_sqrt_eg = 0.0;
        var_sqrt_eg_dn0 = 0.0;
        var_sqrt_eg_dn2 = 0.0;
        var_sqrt_eg_dn4 = 0.0;
        var_sqrt_eg_dn5 = 0.0;
        var_sqrt_eg_dn6 = 0.0;
        var_sqrt_eg_dn7 = 0.0;
        var_sqrt_eg_dn8 = 0.0;
        var_sqrt_eg_dn9 = 0.0;
        var_sqrt_eg_dn10 = 0.0;
        var_sqrt_eg_dn13 = 0.0;
        var_sqrt_eg_rv = 0.0;

        var_wdpl = 0.0;
        var_wdpl_dn0 = 0.0;
        var_wdpl_dn2 = 0.0;
        var_wdpl_dn4 = 0.0;
        var_wdpl_dn5 = 0.0;
        var_wdpl_dn6 = 0.0;
        var_wdpl_dn7 = 0.0;
        var_wdpl_dn8 = 0.0;
        var_wdpl_dn9 = 0.0;
        var_wdpl_dn10 = 0.0;
        var_wdpl_dn13 = 0.0;
        var_wdpl_rv = 0.0;

        var_wdplp = 0.0;
        var_wdplp_dn0 = 0.0;
        var_wdplp_dn2 = 0.0;
        var_wdplp_dn4 = 0.0;
        var_wdplp_dn5 = 0.0;
        var_wdplp_dn6 = 0.0;
        var_wdplp_dn7 = 0.0;
        var_wdplp_dn8 = 0.0;
        var_wdplp_dn9 = 0.0;
        var_wdplp_dn10 = 0.0;
        var_wdplp_dn13 = 0.0;
        var_wdplp_rv = 0.0;

        var_uc_rdrbb = p.p436;
        var_uc_rdrbb_dn0 = 0.0;
        var_uc_rdrbb_dn2 = 0.0;
        var_uc_rdrbb_dn4 = 0.0;
        var_uc_rdrbb_dn5 = 0.0;
        var_uc_rdrbb_dn6 = 0.0;
        var_uc_rdrbb_dn7 = 0.0;
        var_uc_rdrbb_dn8 = 0.0;
        var_uc_rdrbb_dn9 = 0.0;
        var_uc_rdrbb_dn10 = 0.0;
        var_uc_rdrbb_dn13 = 0.0;
        var_uc_rdrbb_rv = 0.0;

        var_uc_rdrbb_s = p.p437;
        var_uc_rdrbb_s_dn0 = 0.0;
        var_uc_rdrbb_s_dn2 = 0.0;
        var_uc_rdrbb_s_dn4 = 0.0;
        var_uc_rdrbb_s_dn5 = 0.0;
        var_uc_rdrbb_s_dn6 = 0.0;
        var_uc_rdrbb_s_dn7 = 0.0;
        var_uc_rdrbb_s_dn8 = 0.0;
        var_uc_rdrbb_s_dn9 = 0.0;
        var_uc_rdrbb_s_dn10 = 0.0;
        var_uc_rdrbb_s_dn13 = 0.0;
        var_uc_rdrbb_s_rv = 0.0;

        var_ids_acc = 0.0;
        var_ids_acc_dn0 = 0.0;
        var_ids_acc_dn2 = 0.0;
        var_ids_acc_dn4 = 0.0;
        var_ids_acc_dn5 = 0.0;
        var_ids_acc_dn6 = 0.0;
        var_ids_acc_dn7 = 0.0;
        var_ids_acc_dn8 = 0.0;
        var_ids_acc_dn9 = 0.0;
        var_ids_acc_dn10 = 0.0;
        var_ids_acc_dn13 = 0.0;
        var_ids_acc_rv = 0.0;

        var_ids_res = 0.0;
        var_ids_res_dn0 = 0.0;
        var_ids_res_dn2 = 0.0;
        var_ids_res_dn4 = 0.0;
        var_ids_res_dn5 = 0.0;
        var_ids_res_dn6 = 0.0;
        var_ids_res_dn7 = 0.0;
        var_ids_res_dn8 = 0.0;
        var_ids_res_dn9 = 0.0;
        var_ids_res_dn10 = 0.0;
        var_ids_res_dn13 = 0.0;
        var_ids_res_rv = 0.0;

        var_ires_leak = 0.0;
        var_ires_leak_dn0 = 0.0;
        var_ires_leak_dn2 = 0.0;
        var_ires_leak_dn4 = 0.0;
        var_ires_leak_dn5 = 0.0;
        var_ires_leak_dn6 = 0.0;
        var_ires_leak_dn7 = 0.0;
        var_ires_leak_dn8 = 0.0;
        var_ires_leak_dn9 = 0.0;
        var_ires_leak_dn10 = 0.0;
        var_ires_leak_dn13 = 0.0;
        var_ires_leak_rv = 0.0;

        var_pb2n = 0.0;
        var_pb2n_dn0 = 0.0;
        var_pb2n_dn2 = 0.0;
        var_pb2n_dn4 = 0.0;
        var_pb2n_dn5 = 0.0;
        var_pb2n_dn6 = 0.0;
        var_pb2n_dn7 = 0.0;
        var_pb2n_dn8 = 0.0;
        var_pb2n_dn9 = 0.0;
        var_pb2n_dn10 = 0.0;
        var_pb2n_dn13 = 0.0;
        var_pb2n_rv = 0.0;

        var_vbipn = 0.0;
        var_vbipn_dn0 = 0.0;
        var_vbipn_dn2 = 0.0;
        var_vbipn_dn4 = 0.0;
        var_vbipn_dn5 = 0.0;
        var_vbipn_dn6 = 0.0;
        var_vbipn_dn7 = 0.0;
        var_vbipn_dn8 = 0.0;
        var_vbipn_dn9 = 0.0;
        var_vbipn_dn10 = 0.0;
        var_vbipn_dn13 = 0.0;
        var_vbipn_rv = 0.0;

        var_hbdceff = p.p447;
        var_hbdceff_dn0 = 0.0;
        var_hbdceff_dn2 = 0.0;
        var_hbdceff_dn4 = 0.0;
        var_hbdceff_dn5 = 0.0;
        var_hbdceff_dn6 = 0.0;
        var_hbdceff_dn7 = 0.0;
        var_hbdceff_dn8 = 0.0;
        var_hbdceff_dn9 = 0.0;
        var_hbdceff_dn10 = 0.0;
        var_hbdceff_dn13 = 0.0;
        var_hbdceff_rv = 0.0;

        var_uc_subtmp = p.p193;
        var_uc_subtmp_rv = 0.0;

        var_depmphn0 = 0.0;
        var_depmphn0_dn0 = 0.0;
        var_depmphn0_dn2 = 0.0;
        var_depmphn0_dn4 = 0.0;
        var_depmphn0_dn5 = 0.0;
        var_depmphn0_dn6 = 0.0;
        var_depmphn0_dn7 = 0.0;
        var_depmphn0_dn8 = 0.0;
        var_depmphn0_dn9 = 0.0;
        var_depmphn0_dn10 = 0.0;
        var_depmphn0_dn13 = 0.0;
        var_depmphn0_rv = 0.0;

        var_qiu_noi = 0.0;
        var_qiu_noi_dn0 = 0.0;
        var_qiu_noi_dn2 = 0.0;
        var_qiu_noi_dn4 = 0.0;
        var_qiu_noi_dn5 = 0.0;
        var_qiu_noi_dn6 = 0.0;
        var_qiu_noi_dn7 = 0.0;
        var_qiu_noi_dn8 = 0.0;
        var_qiu_noi_dn9 = 0.0;
        var_qiu_noi_dn10 = 0.0;
        var_qiu_noi_dn13 = 0.0;
        var_qiu_noi_rv = 0.0;

        var_lp_s0_max = 40.0;
        var_lp_s0_max_rv = 0.0;

        var_js = 0.0;
        var_js_dn0 = 0.0;
        var_js_dn2 = 0.0;
        var_js_dn4 = 0.0;
        var_js_dn5 = 0.0;
        var_js_dn6 = 0.0;
        var_js_dn7 = 0.0;
        var_js_dn8 = 0.0;
        var_js_dn9 = 0.0;
        var_js_dn10 = 0.0;
        var_js_dn13 = 0.0;
        var_js_rv = 0.0;

        var_jssw = 0.0;
        var_jssw_dn0 = 0.0;
        var_jssw_dn2 = 0.0;
        var_jssw_dn4 = 0.0;
        var_jssw_dn5 = 0.0;
        var_jssw_dn6 = 0.0;
        var_jssw_dn7 = 0.0;
        var_jssw_dn8 = 0.0;
        var_jssw_dn9 = 0.0;
        var_jssw_dn10 = 0.0;
        var_jssw_dn13 = 0.0;
        var_jssw_rv = 0.0;

        var_js2 = 0.0;
        var_js2_dn0 = 0.0;
        var_js2_dn2 = 0.0;
        var_js2_dn4 = 0.0;
        var_js2_dn5 = 0.0;
        var_js2_dn6 = 0.0;
        var_js2_dn7 = 0.0;
        var_js2_dn8 = 0.0;
        var_js2_dn9 = 0.0;
        var_js2_dn10 = 0.0;
        var_js2_dn13 = 0.0;
        var_js2_rv = 0.0;

        var_jssw2 = 0.0;
        var_jssw2_dn0 = 0.0;
        var_jssw2_dn2 = 0.0;
        var_jssw2_dn4 = 0.0;
        var_jssw2_dn5 = 0.0;
        var_jssw2_dn6 = 0.0;
        var_jssw2_dn7 = 0.0;
        var_jssw2_dn8 = 0.0;
        var_jssw2_dn9 = 0.0;
        var_jssw2_dn10 = 0.0;
        var_jssw2_dn13 = 0.0;
        var_jssw2_rv = 0.0;

        var_qbs = 0.0;
        var_qbs_dn0 = 0.0;
        var_qbs_dn2 = 0.0;
        var_qbs_dn4 = 0.0;
        var_qbs_dn5 = 0.0;
        var_qbs_dn6 = 0.0;
        var_qbs_dn7 = 0.0;
        var_qbs_dn8 = 0.0;
        var_qbs_dn9 = 0.0;
        var_qbs_dn10 = 0.0;
        var_qbs_dn13 = 0.0;
        var_qbs_rv = 0.0;

        *var_betatnom_slot = var_betatnom;
        *var_betatnom_rv_slot = var_betatnom_rv;
        *var_cgsb_slot = var_cgsb;
        *var_cgsb_dn0_slot = var_cgsb_dn0;
        *var_cgsb_dn10_slot = var_cgsb_dn10;
        *var_cgsb_dn13_slot = var_cgsb_dn13;
        *var_cgsb_dn2_slot = var_cgsb_dn2;
        *var_cgsb_dn4_slot = var_cgsb_dn4;
        *var_cgsb_dn5_slot = var_cgsb_dn5;
        *var_cgsb_dn6_slot = var_cgsb_dn6;
        *var_cgsb_dn7_slot = var_cgsb_dn7;
        *var_cgsb_dn8_slot = var_cgsb_dn8;
        *var_cgsb_dn9_slot = var_cgsb_dn9;
        *var_cgsb_rv_slot = var_cgsb_rv;
        *var_cnst0over_slot = var_cnst0over;
        *var_cnst0over_dn0_slot = var_cnst0over_dn0;
        *var_cnst0over_dn10_slot = var_cnst0over_dn10;
        *var_cnst0over_dn13_slot = var_cnst0over_dn13;
        *var_cnst0over_dn2_slot = var_cnst0over_dn2;
        *var_cnst0over_dn4_slot = var_cnst0over_dn4;
        *var_cnst0over_dn5_slot = var_cnst0over_dn5;
        *var_cnst0over_dn6_slot = var_cnst0over_dn6;
        *var_cnst0over_dn7_slot = var_cnst0over_dn7;
        *var_cnst0over_dn8_slot = var_cnst0over_dn8;
        *var_cnst0over_dn9_slot = var_cnst0over_dn9;
        *var_cnst0over_rv_slot = var_cnst0over_rv;
        *var_cnst0overs_slot = var_cnst0overs;
        *var_cnst0overs_dn0_slot = var_cnst0overs_dn0;
        *var_cnst0overs_dn10_slot = var_cnst0overs_dn10;
        *var_cnst0overs_dn13_slot = var_cnst0overs_dn13;
        *var_cnst0overs_dn2_slot = var_cnst0overs_dn2;
        *var_cnst0overs_dn4_slot = var_cnst0overs_dn4;
        *var_cnst0overs_dn5_slot = var_cnst0overs_dn5;
        *var_cnst0overs_dn6_slot = var_cnst0overs_dn6;
        *var_cnst0overs_dn7_slot = var_cnst0overs_dn7;
        *var_cnst0overs_dn8_slot = var_cnst0overs_dn8;
        *var_cnst0overs_dn9_slot = var_cnst0overs_dn9;
        *var_cnst0overs_rv_slot = var_cnst0overs_rv;
        *var_costi0_p2_slot = var_costi0_p2;
        *var_costi0_p2_dn0_slot = var_costi0_p2_dn0;
        *var_costi0_p2_dn10_slot = var_costi0_p2_dn10;
        *var_costi0_p2_dn13_slot = var_costi0_p2_dn13;
        *var_costi0_p2_dn2_slot = var_costi0_p2_dn2;
        *var_costi0_p2_dn4_slot = var_costi0_p2_dn4;
        *var_costi0_p2_dn5_slot = var_costi0_p2_dn5;
        *var_costi0_p2_dn6_slot = var_costi0_p2_dn6;
        *var_costi0_p2_dn7_slot = var_costi0_p2_dn7;
        *var_costi0_p2_dn8_slot = var_costi0_p2_dn8;
        *var_costi0_p2_dn9_slot = var_costi0_p2_dn9;
        *var_costi0_p2_rv_slot = var_costi0_p2_rv;
        *var_depmphn0_slot = var_depmphn0;
        *var_depmphn0_dn0_slot = var_depmphn0_dn0;
        *var_depmphn0_dn10_slot = var_depmphn0_dn10;
        *var_depmphn0_dn13_slot = var_depmphn0_dn13;
        *var_depmphn0_dn2_slot = var_depmphn0_dn2;
        *var_depmphn0_dn4_slot = var_depmphn0_dn4;
        *var_depmphn0_dn5_slot = var_depmphn0_dn5;
        *var_depmphn0_dn6_slot = var_depmphn0_dn6;
        *var_depmphn0_dn7_slot = var_depmphn0_dn7;
        *var_depmphn0_dn8_slot = var_depmphn0_dn8;
        *var_depmphn0_dn9_slot = var_depmphn0_dn9;
        *var_depmphn0_rv_slot = var_depmphn0_rv;
        *var_hbdceff_slot = var_hbdceff;
        *var_hbdceff_dn0_slot = var_hbdceff_dn0;
        *var_hbdceff_dn10_slot = var_hbdceff_dn10;
        *var_hbdceff_dn13_slot = var_hbdceff_dn13;
        *var_hbdceff_dn2_slot = var_hbdceff_dn2;
        *var_hbdceff_dn4_slot = var_hbdceff_dn4;
        *var_hbdceff_dn5_slot = var_hbdceff_dn5;
        *var_hbdceff_dn6_slot = var_hbdceff_dn6;
        *var_hbdceff_dn7_slot = var_hbdceff_dn7;
        *var_hbdceff_dn8_slot = var_hbdceff_dn8;
        *var_hbdceff_dn9_slot = var_hbdceff_dn9;
        *var_hbdceff_rv_slot = var_hbdceff_rv;
        *var_ids_acc_slot = var_ids_acc;
        *var_ids_acc_dn0_slot = var_ids_acc_dn0;
        *var_ids_acc_dn10_slot = var_ids_acc_dn10;
        *var_ids_acc_dn13_slot = var_ids_acc_dn13;
        *var_ids_acc_dn2_slot = var_ids_acc_dn2;
        *var_ids_acc_dn4_slot = var_ids_acc_dn4;
        *var_ids_acc_dn5_slot = var_ids_acc_dn5;
        *var_ids_acc_dn6_slot = var_ids_acc_dn6;
        *var_ids_acc_dn7_slot = var_ids_acc_dn7;
        *var_ids_acc_dn8_slot = var_ids_acc_dn8;
        *var_ids_acc_dn9_slot = var_ids_acc_dn9;
        *var_ids_acc_rv_slot = var_ids_acc_rv;
        *var_ids_res_slot = var_ids_res;
        *var_ids_res_dn0_slot = var_ids_res_dn0;
        *var_ids_res_dn10_slot = var_ids_res_dn10;
        *var_ids_res_dn13_slot = var_ids_res_dn13;
        *var_ids_res_dn2_slot = var_ids_res_dn2;
        *var_ids_res_dn4_slot = var_ids_res_dn4;
        *var_ids_res_dn5_slot = var_ids_res_dn5;
        *var_ids_res_dn6_slot = var_ids_res_dn6;
        *var_ids_res_dn7_slot = var_ids_res_dn7;
        *var_ids_res_dn8_slot = var_ids_res_dn8;
        *var_ids_res_dn9_slot = var_ids_res_dn9;
        *var_ids_res_rv_slot = var_ids_res_rv;
        *var_ires_leak_slot = var_ires_leak;
        *var_ires_leak_dn0_slot = var_ires_leak_dn0;
        *var_ires_leak_dn10_slot = var_ires_leak_dn10;
        *var_ires_leak_dn13_slot = var_ires_leak_dn13;
        *var_ires_leak_dn2_slot = var_ires_leak_dn2;
        *var_ires_leak_dn4_slot = var_ires_leak_dn4;
        *var_ires_leak_dn5_slot = var_ires_leak_dn5;
        *var_ires_leak_dn6_slot = var_ires_leak_dn6;
        *var_ires_leak_dn7_slot = var_ires_leak_dn7;
        *var_ires_leak_dn8_slot = var_ires_leak_dn8;
        *var_ires_leak_dn9_slot = var_ires_leak_dn9;
        *var_ires_leak_rv_slot = var_ires_leak_rv;
        *var_js_slot = var_js;
        *var_js2_slot = var_js2;
        *var_js2_dn0_slot = var_js2_dn0;
        *var_js2_dn10_slot = var_js2_dn10;
        *var_js2_dn13_slot = var_js2_dn13;
        *var_js2_dn2_slot = var_js2_dn2;
        *var_js2_dn4_slot = var_js2_dn4;
        *var_js2_dn5_slot = var_js2_dn5;
        *var_js2_dn6_slot = var_js2_dn6;
        *var_js2_dn7_slot = var_js2_dn7;
        *var_js2_dn8_slot = var_js2_dn8;
        *var_js2_dn9_slot = var_js2_dn9;
        *var_js2_rv_slot = var_js2_rv;
        *var_js_dn0_slot = var_js_dn0;
        *var_js_dn10_slot = var_js_dn10;
        *var_js_dn13_slot = var_js_dn13;
        *var_js_dn2_slot = var_js_dn2;
        *var_js_dn4_slot = var_js_dn4;
        *var_js_dn5_slot = var_js_dn5;
        *var_js_dn6_slot = var_js_dn6;
        *var_js_dn7_slot = var_js_dn7;
        *var_js_dn8_slot = var_js_dn8;
        *var_js_dn9_slot = var_js_dn9;
        *var_js_rv_slot = var_js_rv;
        *var_jssw_slot = var_jssw;
        *var_jssw2_slot = var_jssw2;
        *var_jssw2_dn0_slot = var_jssw2_dn0;
        *var_jssw2_dn10_slot = var_jssw2_dn10;
        *var_jssw2_dn13_slot = var_jssw2_dn13;
        *var_jssw2_dn2_slot = var_jssw2_dn2;
        *var_jssw2_dn4_slot = var_jssw2_dn4;
        *var_jssw2_dn5_slot = var_jssw2_dn5;
        *var_jssw2_dn6_slot = var_jssw2_dn6;
        *var_jssw2_dn7_slot = var_jssw2_dn7;
        *var_jssw2_dn8_slot = var_jssw2_dn8;
        *var_jssw2_dn9_slot = var_jssw2_dn9;
        *var_jssw2_rv_slot = var_jssw2_rv;
        *var_jssw_dn0_slot = var_jssw_dn0;
        *var_jssw_dn10_slot = var_jssw_dn10;
        *var_jssw_dn13_slot = var_jssw_dn13;
        *var_jssw_dn2_slot = var_jssw_dn2;
        *var_jssw_dn4_slot = var_jssw_dn4;
        *var_jssw_dn5_slot = var_jssw_dn5;
        *var_jssw_dn6_slot = var_jssw_dn6;
        *var_jssw_dn7_slot = var_jssw_dn7;
        *var_jssw_dn8_slot = var_jssw_dn8;
        *var_jssw_dn9_slot = var_jssw_dn9;
        *var_jssw_rv_slot = var_jssw_rv;
        *var_lp_s0_max_slot = var_lp_s0_max;
        *var_lp_s0_max_rv_slot = var_lp_s0_max_rv;
        *var_mphn0_slot = var_mphn0;
        *var_mphn0_dn0_slot = var_mphn0_dn0;
        *var_mphn0_dn10_slot = var_mphn0_dn10;
        *var_mphn0_dn13_slot = var_mphn0_dn13;
        *var_mphn0_dn2_slot = var_mphn0_dn2;
        *var_mphn0_dn4_slot = var_mphn0_dn4;
        *var_mphn0_dn5_slot = var_mphn0_dn5;
        *var_mphn0_dn6_slot = var_mphn0_dn6;
        *var_mphn0_dn7_slot = var_mphn0_dn7;
        *var_mphn0_dn8_slot = var_mphn0_dn8;
        *var_mphn0_dn9_slot = var_mphn0_dn9;
        *var_mphn0_rv_slot = var_mphn0_rv;
        *var_ninvde_slot = var_ninvde;
        *var_ninvde_dn0_slot = var_ninvde_dn0;
        *var_ninvde_dn10_slot = var_ninvde_dn10;
        *var_ninvde_dn13_slot = var_ninvde_dn13;
        *var_ninvde_dn2_slot = var_ninvde_dn2;
        *var_ninvde_dn4_slot = var_ninvde_dn4;
        *var_ninvde_dn5_slot = var_ninvde_dn5;
        *var_ninvde_dn6_slot = var_ninvde_dn6;
        *var_ninvde_dn7_slot = var_ninvde_dn7;
        *var_ninvde_dn8_slot = var_ninvde_dn8;
        *var_ninvde_dn9_slot = var_ninvde_dn9;
        *var_ninvde_rv_slot = var_ninvde_rv;
        *var_ninvdecres_slot = var_ninvdecres;
        *var_ninvdecres_dn0_slot = var_ninvdecres_dn0;
        *var_ninvdecres_dn10_slot = var_ninvdecres_dn10;
        *var_ninvdecres_dn13_slot = var_ninvdecres_dn13;
        *var_ninvdecres_dn2_slot = var_ninvdecres_dn2;
        *var_ninvdecres_dn4_slot = var_ninvdecres_dn4;
        *var_ninvdecres_dn5_slot = var_ninvdecres_dn5;
        *var_ninvdecres_dn6_slot = var_ninvdecres_dn6;
        *var_ninvdecres_dn7_slot = var_ninvdecres_dn7;
        *var_ninvdecres_dn8_slot = var_ninvdecres_dn8;
        *var_ninvdecres_dn9_slot = var_ninvdecres_dn9;
        *var_ninvdecres_rv_slot = var_ninvdecres_rv;
        *var_ninvdehres_slot = var_ninvdehres;
        *var_ninvdehres_dn0_slot = var_ninvdehres_dn0;
        *var_ninvdehres_dn10_slot = var_ninvdehres_dn10;
        *var_ninvdehres_dn13_slot = var_ninvdehres_dn13;
        *var_ninvdehres_dn2_slot = var_ninvdehres_dn2;
        *var_ninvdehres_dn4_slot = var_ninvdehres_dn4;
        *var_ninvdehres_dn5_slot = var_ninvdehres_dn5;
        *var_ninvdehres_dn6_slot = var_ninvdehres_dn6;
        *var_ninvdehres_dn7_slot = var_ninvdehres_dn7;
        *var_ninvdehres_dn8_slot = var_ninvdehres_dn8;
        *var_ninvdehres_dn9_slot = var_ninvdehres_dn9;
        *var_ninvdehres_rv_slot = var_ninvdehres_rv;
        *var_pb2n_slot = var_pb2n;
        *var_pb2n_dn0_slot = var_pb2n_dn0;
        *var_pb2n_dn10_slot = var_pb2n_dn10;
        *var_pb2n_dn13_slot = var_pb2n_dn13;
        *var_pb2n_dn2_slot = var_pb2n_dn2;
        *var_pb2n_dn4_slot = var_pb2n_dn4;
        *var_pb2n_dn5_slot = var_pb2n_dn5;
        *var_pb2n_dn6_slot = var_pb2n_dn6;
        *var_pb2n_dn7_slot = var_pb2n_dn7;
        *var_pb2n_dn8_slot = var_pb2n_dn8;
        *var_pb2n_dn9_slot = var_pb2n_dn9;
        *var_pb2n_rv_slot = var_pb2n_rv;
        *var_powratio_slot = var_powratio;
        *var_powratio_dn0_slot = var_powratio_dn0;
        *var_powratio_dn10_slot = var_powratio_dn10;
        *var_powratio_dn13_slot = var_powratio_dn13;
        *var_powratio_dn2_slot = var_powratio_dn2;
        *var_powratio_dn4_slot = var_powratio_dn4;
        *var_powratio_dn5_slot = var_powratio_dn5;
        *var_powratio_dn6_slot = var_powratio_dn6;
        *var_powratio_dn7_slot = var_powratio_dn7;
        *var_powratio_dn8_slot = var_powratio_dn8;
        *var_powratio_dn9_slot = var_powratio_dn9;
        *var_powratio_rv_slot = var_powratio_rv;
        *var_ptovr_slot = var_ptovr;
        *var_ptovr_dn0_slot = var_ptovr_dn0;
        *var_ptovr_dn10_slot = var_ptovr_dn10;
        *var_ptovr_dn13_slot = var_ptovr_dn13;
        *var_ptovr_dn2_slot = var_ptovr_dn2;
        *var_ptovr_dn4_slot = var_ptovr_dn4;
        *var_ptovr_dn5_slot = var_ptovr_dn5;
        *var_ptovr_dn6_slot = var_ptovr_dn6;
        *var_ptovr_dn7_slot = var_ptovr_dn7;
        *var_ptovr_dn8_slot = var_ptovr_dn8;
        *var_ptovr_dn9_slot = var_ptovr_dn9;
        *var_ptovr_rv_slot = var_ptovr_rv;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn0_slot = var_qbs_dn0;
        *var_qbs_dn10_slot = var_qbs_dn10;
        *var_qbs_dn13_slot = var_qbs_dn13;
        *var_qbs_dn2_slot = var_qbs_dn2;
        *var_qbs_dn4_slot = var_qbs_dn4;
        *var_qbs_dn5_slot = var_qbs_dn5;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_qbs_dn8_slot = var_qbs_dn8;
        *var_qbs_dn9_slot = var_qbs_dn9;
        *var_qbs_rv_slot = var_qbs_rv;
        *var_qiu_noi_slot = var_qiu_noi;
        *var_qiu_noi_dn0_slot = var_qiu_noi_dn0;
        *var_qiu_noi_dn10_slot = var_qiu_noi_dn10;
        *var_qiu_noi_dn13_slot = var_qiu_noi_dn13;
        *var_qiu_noi_dn2_slot = var_qiu_noi_dn2;
        *var_qiu_noi_dn4_slot = var_qiu_noi_dn4;
        *var_qiu_noi_dn5_slot = var_qiu_noi_dn5;
        *var_qiu_noi_dn6_slot = var_qiu_noi_dn6;
        *var_qiu_noi_dn7_slot = var_qiu_noi_dn7;
        *var_qiu_noi_dn8_slot = var_qiu_noi_dn8;
        *var_qiu_noi_dn9_slot = var_qiu_noi_dn9;
        *var_qiu_noi_rv_slot = var_qiu_noi_rv;
        *var_rde_slot = var_rde;
        *var_rde_dn0_slot = var_rde_dn0;
        *var_rde_dn10_slot = var_rde_dn10;
        *var_rde_dn13_slot = var_rde_dn13;
        *var_rde_dn2_slot = var_rde_dn2;
        *var_rde_dn4_slot = var_rde_dn4;
        *var_rde_dn5_slot = var_rde_dn5;
        *var_rde_dn6_slot = var_rde_dn6;
        *var_rde_dn7_slot = var_rde_dn7;
        *var_rde_dn8_slot = var_rde_dn8;
        *var_rde_dn9_slot = var_rde_dn9;
        *var_rde_rv_slot = var_rde_rv;
        *var_rdvde_slot = var_rdvde;
        *var_rdvde_dn0_slot = var_rdvde_dn0;
        *var_rdvde_dn10_slot = var_rdvde_dn10;
        *var_rdvde_dn13_slot = var_rdvde_dn13;
        *var_rdvde_dn2_slot = var_rdvde_dn2;
        *var_rdvde_dn4_slot = var_rdvde_dn4;
        *var_rdvde_dn5_slot = var_rdvde_dn5;
        *var_rdvde_dn6_slot = var_rdvde_dn6;
        *var_rdvde_dn7_slot = var_rdvde_dn7;
        *var_rdvde_dn8_slot = var_rdvde_dn8;
        *var_rdvde_dn9_slot = var_rdvde_dn9;
        *var_rdvde_rv_slot = var_rdvde_rv;
        *var_rrdrmue_slot = var_rrdrmue;
        *var_rrdrmue_dn0_slot = var_rrdrmue_dn0;
        *var_rrdrmue_dn10_slot = var_rrdrmue_dn10;
        *var_rrdrmue_dn13_slot = var_rrdrmue_dn13;
        *var_rrdrmue_dn2_slot = var_rrdrmue_dn2;
        *var_rrdrmue_dn4_slot = var_rrdrmue_dn4;
        *var_rrdrmue_dn5_slot = var_rrdrmue_dn5;
        *var_rrdrmue_dn6_slot = var_rrdrmue_dn6;
        *var_rrdrmue_dn7_slot = var_rrdrmue_dn7;
        *var_rrdrmue_dn8_slot = var_rrdrmue_dn8;
        *var_rrdrmue_dn9_slot = var_rrdrmue_dn9;
        *var_rrdrmue_rv_slot = var_rrdrmue_rv;
        *var_rrdrmues_slot = var_rrdrmues;
        *var_rrdrmues_dn0_slot = var_rrdrmues_dn0;
        *var_rrdrmues_dn10_slot = var_rrdrmues_dn10;
        *var_rrdrmues_dn13_slot = var_rrdrmues_dn13;
        *var_rrdrmues_dn2_slot = var_rrdrmues_dn2;
        *var_rrdrmues_dn4_slot = var_rrdrmues_dn4;
        *var_rrdrmues_dn5_slot = var_rrdrmues_dn5;
        *var_rrdrmues_dn6_slot = var_rrdrmues_dn6;
        *var_rrdrmues_dn7_slot = var_rrdrmues_dn7;
        *var_rrdrmues_dn8_slot = var_rrdrmues_dn8;
        *var_rrdrmues_dn9_slot = var_rrdrmues_dn9;
        *var_rrdrmues_rv_slot = var_rrdrmues_rv;
        *var_rrdrvmax_slot = var_rrdrvmax;
        *var_rrdrvmax_dn0_slot = var_rrdrvmax_dn0;
        *var_rrdrvmax_dn10_slot = var_rrdrvmax_dn10;
        *var_rrdrvmax_dn13_slot = var_rrdrvmax_dn13;
        *var_rrdrvmax_dn2_slot = var_rrdrvmax_dn2;
        *var_rrdrvmax_dn4_slot = var_rrdrvmax_dn4;
        *var_rrdrvmax_dn5_slot = var_rrdrvmax_dn5;
        *var_rrdrvmax_dn6_slot = var_rrdrvmax_dn6;
        *var_rrdrvmax_dn7_slot = var_rrdrvmax_dn7;
        *var_rrdrvmax_dn8_slot = var_rrdrvmax_dn8;
        *var_rrdrvmax_dn9_slot = var_rrdrvmax_dn9;
        *var_rrdrvmax_rv_slot = var_rrdrvmax_rv;
        *var_rrdrvmaxs_slot = var_rrdrvmaxs;
        *var_rrdrvmaxs_dn0_slot = var_rrdrvmaxs_dn0;
        *var_rrdrvmaxs_dn10_slot = var_rrdrvmaxs_dn10;
        *var_rrdrvmaxs_dn13_slot = var_rrdrvmaxs_dn13;
        *var_rrdrvmaxs_dn2_slot = var_rrdrvmaxs_dn2;
        *var_rrdrvmaxs_dn4_slot = var_rrdrvmaxs_dn4;
        *var_rrdrvmaxs_dn5_slot = var_rrdrvmaxs_dn5;
        *var_rrdrvmaxs_dn6_slot = var_rrdrvmaxs_dn6;
        *var_rrdrvmaxs_dn7_slot = var_rrdrvmaxs_dn7;
        *var_rrdrvmaxs_dn8_slot = var_rrdrvmaxs_dn8;
        *var_rrdrvmaxs_dn9_slot = var_rrdrvmaxs_dn9;
        *var_rrdrvmaxs_rv_slot = var_rrdrvmaxs_rv;
        *var_rse_slot = var_rse;
        *var_rse_dn0_slot = var_rse_dn0;
        *var_rse_dn10_slot = var_rse_dn10;
        *var_rse_dn13_slot = var_rse_dn13;
        *var_rse_dn2_slot = var_rse_dn2;
        *var_rse_dn4_slot = var_rse_dn4;
        *var_rse_dn5_slot = var_rse_dn5;
        *var_rse_dn6_slot = var_rse_dn6;
        *var_rse_dn7_slot = var_rse_dn7;
        *var_rse_dn8_slot = var_rse_dn8;
        *var_rse_dn9_slot = var_rse_dn9;
        *var_rse_rv_slot = var_rse_rv;
        *var_rsvde_slot = var_rsvde;
        *var_rsvde_dn0_slot = var_rsvde_dn0;
        *var_rsvde_dn10_slot = var_rsvde_dn10;
        *var_rsvde_dn13_slot = var_rsvde_dn13;
        *var_rsvde_dn2_slot = var_rsvde_dn2;
        *var_rsvde_dn4_slot = var_rsvde_dn4;
        *var_rsvde_dn5_slot = var_rsvde_dn5;
        *var_rsvde_dn6_slot = var_rsvde_dn6;
        *var_rsvde_dn7_slot = var_rsvde_dn7;
        *var_rsvde_dn8_slot = var_rsvde_dn8;
        *var_rsvde_dn9_slot = var_rsvde_dn9;
        *var_rsvde_rv_slot = var_rsvde_rv;
        *var_sqrt_eg_slot = var_sqrt_eg;
        *var_sqrt_eg_dn0_slot = var_sqrt_eg_dn0;
        *var_sqrt_eg_dn10_slot = var_sqrt_eg_dn10;
        *var_sqrt_eg_dn13_slot = var_sqrt_eg_dn13;
        *var_sqrt_eg_dn2_slot = var_sqrt_eg_dn2;
        *var_sqrt_eg_dn4_slot = var_sqrt_eg_dn4;
        *var_sqrt_eg_dn5_slot = var_sqrt_eg_dn5;
        *var_sqrt_eg_dn6_slot = var_sqrt_eg_dn6;
        *var_sqrt_eg_dn7_slot = var_sqrt_eg_dn7;
        *var_sqrt_eg_dn8_slot = var_sqrt_eg_dn8;
        *var_sqrt_eg_dn9_slot = var_sqrt_eg_dn9;
        *var_sqrt_eg_rv_slot = var_sqrt_eg_rv;
        *var_tratio_slot = var_tratio;
        *var_tratio_dn0_slot = var_tratio_dn0;
        *var_tratio_dn10_slot = var_tratio_dn10;
        *var_tratio_dn13_slot = var_tratio_dn13;
        *var_tratio_dn2_slot = var_tratio_dn2;
        *var_tratio_dn4_slot = var_tratio_dn4;
        *var_tratio_dn5_slot = var_tratio_dn5;
        *var_tratio_dn6_slot = var_tratio_dn6;
        *var_tratio_dn7_slot = var_tratio_dn7;
        *var_tratio_dn8_slot = var_tratio_dn8;
        *var_tratio_dn9_slot = var_tratio_dn9;
        *var_tratio_rv_slot = var_tratio_rv;
        *var_uc_rdrbb_slot = var_uc_rdrbb;
        *var_uc_rdrbb_dn0_slot = var_uc_rdrbb_dn0;
        *var_uc_rdrbb_dn10_slot = var_uc_rdrbb_dn10;
        *var_uc_rdrbb_dn13_slot = var_uc_rdrbb_dn13;
        *var_uc_rdrbb_dn2_slot = var_uc_rdrbb_dn2;
        *var_uc_rdrbb_dn4_slot = var_uc_rdrbb_dn4;
        *var_uc_rdrbb_dn5_slot = var_uc_rdrbb_dn5;
        *var_uc_rdrbb_dn6_slot = var_uc_rdrbb_dn6;
        *var_uc_rdrbb_dn7_slot = var_uc_rdrbb_dn7;
        *var_uc_rdrbb_dn8_slot = var_uc_rdrbb_dn8;
        *var_uc_rdrbb_dn9_slot = var_uc_rdrbb_dn9;
        *var_uc_rdrbb_rv_slot = var_uc_rdrbb_rv;
        *var_uc_rdrbb_s_slot = var_uc_rdrbb_s;
        *var_uc_rdrbb_s_dn0_slot = var_uc_rdrbb_s_dn0;
        *var_uc_rdrbb_s_dn10_slot = var_uc_rdrbb_s_dn10;
        *var_uc_rdrbb_s_dn13_slot = var_uc_rdrbb_s_dn13;
        *var_uc_rdrbb_s_dn2_slot = var_uc_rdrbb_s_dn2;
        *var_uc_rdrbb_s_dn4_slot = var_uc_rdrbb_s_dn4;
        *var_uc_rdrbb_s_dn5_slot = var_uc_rdrbb_s_dn5;
        *var_uc_rdrbb_s_dn6_slot = var_uc_rdrbb_s_dn6;
        *var_uc_rdrbb_s_dn7_slot = var_uc_rdrbb_s_dn7;
        *var_uc_rdrbb_s_dn8_slot = var_uc_rdrbb_s_dn8;
        *var_uc_rdrbb_s_dn9_slot = var_uc_rdrbb_s_dn9;
        *var_uc_rdrbb_s_rv_slot = var_uc_rdrbb_s_rv;
        *var_uc_subtmp_slot = var_uc_subtmp;
        *var_uc_subtmp_rv_slot = var_uc_subtmp_rv;
        *var_vbipn_slot = var_vbipn;
        *var_vbipn_dn0_slot = var_vbipn_dn0;
        *var_vbipn_dn10_slot = var_vbipn_dn10;
        *var_vbipn_dn13_slot = var_vbipn_dn13;
        *var_vbipn_dn2_slot = var_vbipn_dn2;
        *var_vbipn_dn4_slot = var_vbipn_dn4;
        *var_vbipn_dn5_slot = var_vbipn_dn5;
        *var_vbipn_dn6_slot = var_vbipn_dn6;
        *var_vbipn_dn7_slot = var_vbipn_dn7;
        *var_vbipn_dn8_slot = var_vbipn_dn8;
        *var_vbipn_dn9_slot = var_vbipn_dn9;
        *var_vbipn_rv_slot = var_vbipn_rv;
        *var_vmaxeff_slot = var_vmaxeff;
        *var_vmaxeff_dn0_slot = var_vmaxeff_dn0;
        *var_vmaxeff_dn10_slot = var_vmaxeff_dn10;
        *var_vmaxeff_dn13_slot = var_vmaxeff_dn13;
        *var_vmaxeff_dn2_slot = var_vmaxeff_dn2;
        *var_vmaxeff_dn4_slot = var_vmaxeff_dn4;
        *var_vmaxeff_dn5_slot = var_vmaxeff_dn5;
        *var_vmaxeff_dn6_slot = var_vmaxeff_dn6;
        *var_vmaxeff_dn7_slot = var_vmaxeff_dn7;
        *var_vmaxeff_dn8_slot = var_vmaxeff_dn8;
        *var_vmaxeff_dn9_slot = var_vmaxeff_dn9;
        *var_vmaxeff_rv_slot = var_vmaxeff_rv;
        *var_wdpl_slot = var_wdpl;
        *var_wdpl_dn0_slot = var_wdpl_dn0;
        *var_wdpl_dn10_slot = var_wdpl_dn10;
        *var_wdpl_dn13_slot = var_wdpl_dn13;
        *var_wdpl_dn2_slot = var_wdpl_dn2;
        *var_wdpl_dn4_slot = var_wdpl_dn4;
        *var_wdpl_dn5_slot = var_wdpl_dn5;
        *var_wdpl_dn6_slot = var_wdpl_dn6;
        *var_wdpl_dn7_slot = var_wdpl_dn7;
        *var_wdpl_dn8_slot = var_wdpl_dn8;
        *var_wdpl_dn9_slot = var_wdpl_dn9;
        *var_wdpl_rv_slot = var_wdpl_rv;
        *var_wdplp_slot = var_wdplp;
        *var_wdplp_dn0_slot = var_wdplp_dn0;
        *var_wdplp_dn10_slot = var_wdplp_dn10;
        *var_wdplp_dn13_slot = var_wdplp_dn13;
        *var_wdplp_dn2_slot = var_wdplp_dn2;
        *var_wdplp_dn4_slot = var_wdplp_dn4;
        *var_wdplp_dn5_slot = var_wdplp_dn5;
        *var_wdplp_dn6_slot = var_wdplp_dn6;
        *var_wdplp_dn7_slot = var_wdplp_dn7;
        *var_wdplp_dn8_slot = var_wdplp_dn8;
        *var_wdplp_dn9_slot = var_wdplp_dn9;
        *var_wdplp_rv_slot = var_wdplp_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        var_czbd_slot: &mut f64,
        var_czbd_dn0_slot: &mut f64,
        var_czbd_dn10_slot: &mut f64,
        var_czbd_dn13_slot: &mut f64,
        var_czbd_dn2_slot: &mut f64,
        var_czbd_dn4_slot: &mut f64,
        var_czbd_dn5_slot: &mut f64,
        var_czbd_dn6_slot: &mut f64,
        var_czbd_dn7_slot: &mut f64,
        var_czbd_dn8_slot: &mut f64,
        var_czbd_dn9_slot: &mut f64,
        var_czbd_rv_slot: &mut f64,
        var_czbdsw_slot: &mut f64,
        var_czbdsw_dn0_slot: &mut f64,
        var_czbdsw_dn10_slot: &mut f64,
        var_czbdsw_dn13_slot: &mut f64,
        var_czbdsw_dn2_slot: &mut f64,
        var_czbdsw_dn4_slot: &mut f64,
        var_czbdsw_dn5_slot: &mut f64,
        var_czbdsw_dn6_slot: &mut f64,
        var_czbdsw_dn7_slot: &mut f64,
        var_czbdsw_dn8_slot: &mut f64,
        var_czbdsw_dn9_slot: &mut f64,
        var_czbdsw_rv_slot: &mut f64,
        var_czbdswg_slot: &mut f64,
        var_czbdswg_dn0_slot: &mut f64,
        var_czbdswg_dn10_slot: &mut f64,
        var_czbdswg_dn13_slot: &mut f64,
        var_czbdswg_dn2_slot: &mut f64,
        var_czbdswg_dn4_slot: &mut f64,
        var_czbdswg_dn5_slot: &mut f64,
        var_czbdswg_dn6_slot: &mut f64,
        var_czbdswg_dn7_slot: &mut f64,
        var_czbdswg_dn8_slot: &mut f64,
        var_czbdswg_dn9_slot: &mut f64,
        var_czbdswg_rv_slot: &mut f64,
        var_czbs_slot: &mut f64,
        var_czbs_dn0_slot: &mut f64,
        var_czbs_dn10_slot: &mut f64,
        var_czbs_dn13_slot: &mut f64,
        var_czbs_dn2_slot: &mut f64,
        var_czbs_dn4_slot: &mut f64,
        var_czbs_dn5_slot: &mut f64,
        var_czbs_dn6_slot: &mut f64,
        var_czbs_dn7_slot: &mut f64,
        var_czbs_dn8_slot: &mut f64,
        var_czbs_dn9_slot: &mut f64,
        var_czbs_rv_slot: &mut f64,
        var_czbssw_slot: &mut f64,
        var_czbssw_dn0_slot: &mut f64,
        var_czbssw_dn10_slot: &mut f64,
        var_czbssw_dn13_slot: &mut f64,
        var_czbssw_dn2_slot: &mut f64,
        var_czbssw_dn4_slot: &mut f64,
        var_czbssw_dn5_slot: &mut f64,
        var_czbssw_dn6_slot: &mut f64,
        var_czbssw_dn7_slot: &mut f64,
        var_czbssw_dn8_slot: &mut f64,
        var_czbssw_dn9_slot: &mut f64,
        var_czbssw_rv_slot: &mut f64,
        var_czbsswg_slot: &mut f64,
        var_czbsswg_dn0_slot: &mut f64,
        var_czbsswg_dn10_slot: &mut f64,
        var_czbsswg_dn13_slot: &mut f64,
        var_czbsswg_dn2_slot: &mut f64,
        var_czbsswg_dn4_slot: &mut f64,
        var_czbsswg_dn5_slot: &mut f64,
        var_czbsswg_dn6_slot: &mut f64,
        var_czbsswg_dn7_slot: &mut f64,
        var_czbsswg_dn8_slot: &mut f64,
        var_czbsswg_dn9_slot: &mut f64,
        var_czbsswg_rv_slot: &mut f64,
        var_end_of_part_1_slot: &mut f64,
        var_end_of_part_1_rv_slot: &mut f64,
        var_exptempd_slot: &mut f64,
        var_exptempd_dn0_slot: &mut f64,
        var_exptempd_dn10_slot: &mut f64,
        var_exptempd_dn13_slot: &mut f64,
        var_exptempd_dn2_slot: &mut f64,
        var_exptempd_dn4_slot: &mut f64,
        var_exptempd_dn5_slot: &mut f64,
        var_exptempd_dn6_slot: &mut f64,
        var_exptempd_dn7_slot: &mut f64,
        var_exptempd_dn8_slot: &mut f64,
        var_exptempd_dn9_slot: &mut f64,
        var_exptempd_rv_slot: &mut f64,
        var_exptemps_slot: &mut f64,
        var_exptemps_dn0_slot: &mut f64,
        var_exptemps_dn10_slot: &mut f64,
        var_exptemps_dn13_slot: &mut f64,
        var_exptemps_dn2_slot: &mut f64,
        var_exptemps_dn4_slot: &mut f64,
        var_exptemps_dn5_slot: &mut f64,
        var_exptemps_dn6_slot: &mut f64,
        var_exptemps_dn7_slot: &mut f64,
        var_exptemps_dn8_slot: &mut f64,
        var_exptemps_dn9_slot: &mut f64,
        var_exptemps_rv_slot: &mut f64,
        var_flg_brk1_slot: &mut f64,
        var_flg_brk1_rv_slot: &mut f64,
        var_flg_brk2_slot: &mut f64,
        var_flg_brk2_rv_slot: &mut f64,
        var_isbd_slot: &mut f64,
        var_isbd2_btm_slot: &mut f64,
        var_isbd2_btm_dn0_slot: &mut f64,
        var_isbd2_btm_dn10_slot: &mut f64,
        var_isbd2_btm_dn13_slot: &mut f64,
        var_isbd2_btm_dn2_slot: &mut f64,
        var_isbd2_btm_dn4_slot: &mut f64,
        var_isbd2_btm_dn5_slot: &mut f64,
        var_isbd2_btm_dn6_slot: &mut f64,
        var_isbd2_btm_dn7_slot: &mut f64,
        var_isbd2_btm_dn8_slot: &mut f64,
        var_isbd2_btm_dn9_slot: &mut f64,
        var_isbd2_btm_rv_slot: &mut f64,
        var_isbd2_swg_slot: &mut f64,
        var_isbd2_swg_dn0_slot: &mut f64,
        var_isbd2_swg_dn10_slot: &mut f64,
        var_isbd2_swg_dn13_slot: &mut f64,
        var_isbd2_swg_dn2_slot: &mut f64,
        var_isbd2_swg_dn4_slot: &mut f64,
        var_isbd2_swg_dn5_slot: &mut f64,
        var_isbd2_swg_dn6_slot: &mut f64,
        var_isbd2_swg_dn7_slot: &mut f64,
        var_isbd2_swg_dn8_slot: &mut f64,
        var_isbd2_swg_dn9_slot: &mut f64,
        var_isbd2_swg_rv_slot: &mut f64,
        var_isbd2_sws_slot: &mut f64,
        var_isbd2_sws_dn0_slot: &mut f64,
        var_isbd2_sws_dn10_slot: &mut f64,
        var_isbd2_sws_dn13_slot: &mut f64,
        var_isbd2_sws_dn2_slot: &mut f64,
        var_isbd2_sws_dn4_slot: &mut f64,
        var_isbd2_sws_dn5_slot: &mut f64,
        var_isbd2_sws_dn6_slot: &mut f64,
        var_isbd2_sws_dn7_slot: &mut f64,
        var_isbd2_sws_dn8_slot: &mut f64,
        var_isbd2_sws_dn9_slot: &mut f64,
        var_isbd2_sws_rv_slot: &mut f64,
        var_isbd_btm_slot: &mut f64,
        var_isbd_btm_dn0_slot: &mut f64,
        var_isbd_btm_dn10_slot: &mut f64,
        var_isbd_btm_dn13_slot: &mut f64,
        var_isbd_btm_dn2_slot: &mut f64,
        var_isbd_btm_dn4_slot: &mut f64,
        var_isbd_btm_dn5_slot: &mut f64,
        var_isbd_btm_dn6_slot: &mut f64,
        var_isbd_btm_dn7_slot: &mut f64,
        var_isbd_btm_dn8_slot: &mut f64,
        var_isbd_btm_dn9_slot: &mut f64,
        var_isbd_btm_rv_slot: &mut f64,
        var_isbd_dn0_slot: &mut f64,
        var_isbd_dn10_slot: &mut f64,
        var_isbd_dn13_slot: &mut f64,
        var_isbd_dn2_slot: &mut f64,
        var_isbd_dn4_slot: &mut f64,
        var_isbd_dn5_slot: &mut f64,
        var_isbd_dn6_slot: &mut f64,
        var_isbd_dn7_slot: &mut f64,
        var_isbd_dn8_slot: &mut f64,
        var_isbd_dn9_slot: &mut f64,
        var_isbd_rv_slot: &mut f64,
        var_isbd_swg_slot: &mut f64,
        var_isbd_swg_dn0_slot: &mut f64,
        var_isbd_swg_dn10_slot: &mut f64,
        var_isbd_swg_dn13_slot: &mut f64,
        var_isbd_swg_dn2_slot: &mut f64,
        var_isbd_swg_dn4_slot: &mut f64,
        var_isbd_swg_dn5_slot: &mut f64,
        var_isbd_swg_dn6_slot: &mut f64,
        var_isbd_swg_dn7_slot: &mut f64,
        var_isbd_swg_dn8_slot: &mut f64,
        var_isbd_swg_dn9_slot: &mut f64,
        var_isbd_swg_rv_slot: &mut f64,
        var_isbd_sws_slot: &mut f64,
        var_isbd_sws_dn0_slot: &mut f64,
        var_isbd_sws_dn10_slot: &mut f64,
        var_isbd_sws_dn13_slot: &mut f64,
        var_isbd_sws_dn2_slot: &mut f64,
        var_isbd_sws_dn4_slot: &mut f64,
        var_isbd_sws_dn5_slot: &mut f64,
        var_isbd_sws_dn6_slot: &mut f64,
        var_isbd_sws_dn7_slot: &mut f64,
        var_isbd_sws_dn8_slot: &mut f64,
        var_isbd_sws_dn9_slot: &mut f64,
        var_isbd_sws_rv_slot: &mut f64,
        var_isbs_slot: &mut f64,
        var_isbs_btm_slot: &mut f64,
        var_isbs_btm_dn0_slot: &mut f64,
        var_isbs_btm_dn10_slot: &mut f64,
        var_isbs_btm_dn13_slot: &mut f64,
        var_isbs_btm_dn2_slot: &mut f64,
        var_isbs_btm_dn4_slot: &mut f64,
        var_isbs_btm_dn5_slot: &mut f64,
        var_isbs_btm_dn6_slot: &mut f64,
        var_isbs_btm_dn7_slot: &mut f64,
        var_isbs_btm_dn8_slot: &mut f64,
        var_isbs_btm_dn9_slot: &mut f64,
        var_isbs_btm_rv_slot: &mut f64,
        var_isbs_dn0_slot: &mut f64,
        var_isbs_dn10_slot: &mut f64,
        var_isbs_dn13_slot: &mut f64,
        var_isbs_dn2_slot: &mut f64,
        var_isbs_dn4_slot: &mut f64,
        var_isbs_dn5_slot: &mut f64,
        var_isbs_dn6_slot: &mut f64,
        var_isbs_dn7_slot: &mut f64,
        var_isbs_dn8_slot: &mut f64,
        var_isbs_dn9_slot: &mut f64,
        var_isbs_rv_slot: &mut f64,
        var_jd_expcd_slot: &mut f64,
        var_jd_expcd_dn0_slot: &mut f64,
        var_jd_expcd_dn10_slot: &mut f64,
        var_jd_expcd_dn13_slot: &mut f64,
        var_jd_expcd_dn2_slot: &mut f64,
        var_jd_expcd_dn4_slot: &mut f64,
        var_jd_expcd_dn5_slot: &mut f64,
        var_jd_expcd_dn6_slot: &mut f64,
        var_jd_expcd_dn7_slot: &mut f64,
        var_jd_expcd_dn8_slot: &mut f64,
        var_jd_expcd_dn9_slot: &mut f64,
        var_jd_expcd_rv_slot: &mut f64,
        var_jd_expcs_slot: &mut f64,
        var_jd_expcs_dn0_slot: &mut f64,
        var_jd_expcs_dn10_slot: &mut f64,
        var_jd_expcs_dn13_slot: &mut f64,
        var_jd_expcs_dn2_slot: &mut f64,
        var_jd_expcs_dn4_slot: &mut f64,
        var_jd_expcs_dn5_slot: &mut f64,
        var_jd_expcs_dn6_slot: &mut f64,
        var_jd_expcs_dn7_slot: &mut f64,
        var_jd_expcs_dn8_slot: &mut f64,
        var_jd_expcs_dn9_slot: &mut f64,
        var_jd_expcs_rv_slot: &mut f64,
        var_jd_nvtm_invd_slot: &mut f64,
        var_jd_nvtm_invd_dn0_slot: &mut f64,
        var_jd_nvtm_invd_dn10_slot: &mut f64,
        var_jd_nvtm_invd_dn13_slot: &mut f64,
        var_jd_nvtm_invd_dn2_slot: &mut f64,
        var_jd_nvtm_invd_dn4_slot: &mut f64,
        var_jd_nvtm_invd_dn5_slot: &mut f64,
        var_jd_nvtm_invd_dn6_slot: &mut f64,
        var_jd_nvtm_invd_dn7_slot: &mut f64,
        var_jd_nvtm_invd_dn8_slot: &mut f64,
        var_jd_nvtm_invd_dn9_slot: &mut f64,
        var_jd_nvtm_invd_rv_slot: &mut f64,
        var_jd_nvtm_invs_slot: &mut f64,
        var_jd_nvtm_invs_dn0_slot: &mut f64,
        var_jd_nvtm_invs_dn10_slot: &mut f64,
        var_jd_nvtm_invs_dn13_slot: &mut f64,
        var_jd_nvtm_invs_dn2_slot: &mut f64,
        var_jd_nvtm_invs_dn4_slot: &mut f64,
        var_jd_nvtm_invs_dn5_slot: &mut f64,
        var_jd_nvtm_invs_dn6_slot: &mut f64,
        var_jd_nvtm_invs_dn7_slot: &mut f64,
        var_jd_nvtm_invs_dn8_slot: &mut f64,
        var_jd_nvtm_invs_dn9_slot: &mut f64,
        var_jd_nvtm_invs_rv_slot: &mut f64,
        var_pzbd_slot: &mut f64,
        var_pzbd_dn0_slot: &mut f64,
        var_pzbd_dn10_slot: &mut f64,
        var_pzbd_dn13_slot: &mut f64,
        var_pzbd_dn2_slot: &mut f64,
        var_pzbd_dn4_slot: &mut f64,
        var_pzbd_dn5_slot: &mut f64,
        var_pzbd_dn6_slot: &mut f64,
        var_pzbd_dn7_slot: &mut f64,
        var_pzbd_dn8_slot: &mut f64,
        var_pzbd_dn9_slot: &mut f64,
        var_pzbd_rv_slot: &mut f64,
        var_pzbdsw_slot: &mut f64,
        var_pzbdsw_dn0_slot: &mut f64,
        var_pzbdsw_dn10_slot: &mut f64,
        var_pzbdsw_dn13_slot: &mut f64,
        var_pzbdsw_dn2_slot: &mut f64,
        var_pzbdsw_dn4_slot: &mut f64,
        var_pzbdsw_dn5_slot: &mut f64,
        var_pzbdsw_dn6_slot: &mut f64,
        var_pzbdsw_dn7_slot: &mut f64,
        var_pzbdsw_dn8_slot: &mut f64,
        var_pzbdsw_dn9_slot: &mut f64,
        var_pzbdsw_rv_slot: &mut f64,
        var_pzbdswg_slot: &mut f64,
        var_pzbdswg_dn0_slot: &mut f64,
        var_pzbdswg_dn10_slot: &mut f64,
        var_pzbdswg_dn13_slot: &mut f64,
        var_pzbdswg_dn2_slot: &mut f64,
        var_pzbdswg_dn4_slot: &mut f64,
        var_pzbdswg_dn5_slot: &mut f64,
        var_pzbdswg_dn6_slot: &mut f64,
        var_pzbdswg_dn7_slot: &mut f64,
        var_pzbdswg_dn8_slot: &mut f64,
        var_pzbdswg_dn9_slot: &mut f64,
        var_pzbdswg_rv_slot: &mut f64,
        var_pzbs_slot: &mut f64,
        var_pzbs_dn0_slot: &mut f64,
        var_pzbs_dn10_slot: &mut f64,
        var_pzbs_dn13_slot: &mut f64,
        var_pzbs_dn2_slot: &mut f64,
        var_pzbs_dn4_slot: &mut f64,
        var_pzbs_dn5_slot: &mut f64,
        var_pzbs_dn6_slot: &mut f64,
        var_pzbs_dn7_slot: &mut f64,
        var_pzbs_dn8_slot: &mut f64,
        var_pzbs_dn9_slot: &mut f64,
        var_pzbs_rv_slot: &mut f64,
        var_pzbssw_slot: &mut f64,
        var_pzbssw_dn0_slot: &mut f64,
        var_pzbssw_dn10_slot: &mut f64,
        var_pzbssw_dn13_slot: &mut f64,
        var_pzbssw_dn2_slot: &mut f64,
        var_pzbssw_dn4_slot: &mut f64,
        var_pzbssw_dn5_slot: &mut f64,
        var_pzbssw_dn6_slot: &mut f64,
        var_pzbssw_dn7_slot: &mut f64,
        var_pzbssw_dn8_slot: &mut f64,
        var_pzbssw_dn9_slot: &mut f64,
        var_pzbssw_rv_slot: &mut f64,
        var_pzbsswg_slot: &mut f64,
        var_pzbsswg_dn0_slot: &mut f64,
        var_pzbsswg_dn10_slot: &mut f64,
        var_pzbsswg_dn13_slot: &mut f64,
        var_pzbsswg_dn2_slot: &mut f64,
        var_pzbsswg_dn4_slot: &mut f64,
        var_pzbsswg_dn5_slot: &mut f64,
        var_pzbsswg_dn6_slot: &mut f64,
        var_pzbsswg_dn7_slot: &mut f64,
        var_pzbsswg_dn8_slot: &mut f64,
        var_pzbsswg_dn9_slot: &mut f64,
        var_pzbsswg_rv_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn0_slot: &mut f64,
        var_qbd_dn10_slot: &mut f64,
        var_qbd_dn13_slot: &mut f64,
        var_qbd_dn15_slot: &mut f64,
        var_qbd_dn16_slot: &mut f64,
        var_qbd_dn17_slot: &mut f64,
        var_qbd_dn2_slot: &mut f64,
        var_qbd_dn4_slot: &mut f64,
        var_qbd_dn5_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbd_dn8_slot: &mut f64,
        var_qbd_dn9_slot: &mut f64,
        var_qbd_qs_slot: &mut f64,
        var_qbd_qs_dn0_slot: &mut f64,
        var_qbd_qs_dn10_slot: &mut f64,
        var_qbd_qs_dn13_slot: &mut f64,
        var_qbd_qs_dn2_slot: &mut f64,
        var_qbd_qs_dn4_slot: &mut f64,
        var_qbd_qs_dn5_slot: &mut f64,
        var_qbd_qs_dn6_slot: &mut f64,
        var_qbd_qs_dn7_slot: &mut f64,
        var_qbd_qs_dn8_slot: &mut f64,
        var_qbd_qs_dn9_slot: &mut f64,
        var_qbd_qs_rv_slot: &mut f64,
        var_qbd_rv_slot: &mut f64,
        var_qbdi_slot: &mut f64,
        var_qbdi_dn0_slot: &mut f64,
        var_qbdi_dn10_slot: &mut f64,
        var_qbdi_dn13_slot: &mut f64,
        var_qbdi_dn2_slot: &mut f64,
        var_qbdi_dn4_slot: &mut f64,
        var_qbdi_dn5_slot: &mut f64,
        var_qbdi_dn6_slot: &mut f64,
        var_qbdi_dn7_slot: &mut f64,
        var_qbdi_dn8_slot: &mut f64,
        var_qbdi_dn9_slot: &mut f64,
        var_qbdi_rv_slot: &mut f64,
        var_qbsi_slot: &mut f64,
        var_qbsi_dn0_slot: &mut f64,
        var_qbsi_dn10_slot: &mut f64,
        var_qbsi_dn13_slot: &mut f64,
        var_qbsi_dn2_slot: &mut f64,
        var_qbsi_dn4_slot: &mut f64,
        var_qbsi_dn5_slot: &mut f64,
        var_qbsi_dn6_slot: &mut f64,
        var_qbsi_dn7_slot: &mut f64,
        var_qbsi_dn8_slot: &mut f64,
        var_qbsi_dn9_slot: &mut f64,
        var_qbsi_rv_slot: &mut f64,
        var_sarg_slot: &mut f64,
        var_sarg_dn0_slot: &mut f64,
        var_sarg_dn10_slot: &mut f64,
        var_sarg_dn13_slot: &mut f64,
        var_sarg_dn2_slot: &mut f64,
        var_sarg_dn4_slot: &mut f64,
        var_sarg_dn5_slot: &mut f64,
        var_sarg_dn6_slot: &mut f64,
        var_sarg_dn7_slot: &mut f64,
        var_sarg_dn8_slot: &mut f64,
        var_sarg_dn9_slot: &mut f64,
        var_sarg_rv_slot: &mut f64,
        var_start_of_loopl_slot: &mut f64,
        var_start_of_loopl_rv_slot: &mut f64,
        var_start_of_mobility_slot: &mut f64,
        var_start_of_mobility_rv_slot: &mut f64,
        var_vbd_jct_slot: &mut f64,
        var_vbd_jct_dn0_slot: &mut f64,
        var_vbd_jct_dn9_slot: &mut f64,
        var_vbd_jct_rv_slot: &mut f64,
        var_vbdi_jct_slot: &mut f64,
        var_vbdi_jct_dn5_slot: &mut f64,
        var_vbdi_jct_dn8_slot: &mut f64,
        var_vbdi_jct_rv_slot: &mut f64,
        var_vbdt_slot: &mut f64,
        var_vbdt_dn0_slot: &mut f64,
        var_vbdt_dn10_slot: &mut f64,
        var_vbdt_dn13_slot: &mut f64,
        var_vbdt_dn2_slot: &mut f64,
        var_vbdt_dn4_slot: &mut f64,
        var_vbdt_dn5_slot: &mut f64,
        var_vbdt_dn6_slot: &mut f64,
        var_vbdt_dn7_slot: &mut f64,
        var_vbdt_dn8_slot: &mut f64,
        var_vbdt_dn9_slot: &mut f64,
        var_vbdt_rv_slot: &mut f64,
        var_vbpdp_slot: &mut f64,
        var_vbpdp_dn5_slot: &mut f64,
        var_vbpdp_dn8_slot: &mut f64,
        var_vbpdp_rv_slot: &mut f64,
        var_vbpsp_slot: &mut f64,
        var_vbpsp_dn7_slot: &mut f64,
        var_vbpsp_dn8_slot: &mut f64,
        var_vbpsp_rv_slot: &mut f64,
        var_vbs_jct_slot: &mut f64,
        var_vbs_jct_dn10_slot: &mut f64,
        var_vbs_jct_dn2_slot: &mut f64,
        var_vbs_jct_rv_slot: &mut f64,
        var_vbsi_jct_slot: &mut f64,
        var_vbsi_jct_dn7_slot: &mut f64,
        var_vbsi_jct_dn8_slot: &mut f64,
        var_vbsi_jct_rv_slot: &mut f64,
        var_vbst_slot: &mut f64,
        var_vbst_dn0_slot: &mut f64,
        var_vbst_dn10_slot: &mut f64,
        var_vbst_dn13_slot: &mut f64,
        var_vbst_dn2_slot: &mut f64,
        var_vbst_dn4_slot: &mut f64,
        var_vbst_dn5_slot: &mut f64,
        var_vbst_dn6_slot: &mut f64,
        var_vbst_dn7_slot: &mut f64,
        var_vbst_dn8_slot: &mut f64,
        var_vbst_dn9_slot: &mut f64,
        var_vbst_rv_slot: &mut f64,
        var_vdbd_slot: &mut f64,
        var_vdbd_dn0_slot: &mut f64,
        var_vdbd_dn9_slot: &mut f64,
        var_vdbd_rv_slot: &mut f64,
        var_vsbs_slot: &mut f64,
        var_vsbs_dn10_slot: &mut f64,
        var_vsbs_dn2_slot: &mut f64,
        var_vsbs_rv_slot: &mut f64,
    ) {
        let mut var_czbd: f64 = *var_czbd_slot;
        let mut var_czbd_dn0: f64 = *var_czbd_dn0_slot;
        let mut var_czbd_dn10: f64 = *var_czbd_dn10_slot;
        let mut var_czbd_dn13: f64 = *var_czbd_dn13_slot;
        let mut var_czbd_dn2: f64 = *var_czbd_dn2_slot;
        let mut var_czbd_dn4: f64 = *var_czbd_dn4_slot;
        let mut var_czbd_dn5: f64 = *var_czbd_dn5_slot;
        let mut var_czbd_dn6: f64 = *var_czbd_dn6_slot;
        let mut var_czbd_dn7: f64 = *var_czbd_dn7_slot;
        let mut var_czbd_dn8: f64 = *var_czbd_dn8_slot;
        let mut var_czbd_dn9: f64 = *var_czbd_dn9_slot;
        let mut var_czbd_rv: f64 = *var_czbd_rv_slot;
        let mut var_czbdsw: f64 = *var_czbdsw_slot;
        let mut var_czbdsw_dn0: f64 = *var_czbdsw_dn0_slot;
        let mut var_czbdsw_dn10: f64 = *var_czbdsw_dn10_slot;
        let mut var_czbdsw_dn13: f64 = *var_czbdsw_dn13_slot;
        let mut var_czbdsw_dn2: f64 = *var_czbdsw_dn2_slot;
        let mut var_czbdsw_dn4: f64 = *var_czbdsw_dn4_slot;
        let mut var_czbdsw_dn5: f64 = *var_czbdsw_dn5_slot;
        let mut var_czbdsw_dn6: f64 = *var_czbdsw_dn6_slot;
        let mut var_czbdsw_dn7: f64 = *var_czbdsw_dn7_slot;
        let mut var_czbdsw_dn8: f64 = *var_czbdsw_dn8_slot;
        let mut var_czbdsw_dn9: f64 = *var_czbdsw_dn9_slot;
        let mut var_czbdsw_rv: f64 = *var_czbdsw_rv_slot;
        let mut var_czbdswg: f64 = *var_czbdswg_slot;
        let mut var_czbdswg_dn0: f64 = *var_czbdswg_dn0_slot;
        let mut var_czbdswg_dn10: f64 = *var_czbdswg_dn10_slot;
        let mut var_czbdswg_dn13: f64 = *var_czbdswg_dn13_slot;
        let mut var_czbdswg_dn2: f64 = *var_czbdswg_dn2_slot;
        let mut var_czbdswg_dn4: f64 = *var_czbdswg_dn4_slot;
        let mut var_czbdswg_dn5: f64 = *var_czbdswg_dn5_slot;
        let mut var_czbdswg_dn6: f64 = *var_czbdswg_dn6_slot;
        let mut var_czbdswg_dn7: f64 = *var_czbdswg_dn7_slot;
        let mut var_czbdswg_dn8: f64 = *var_czbdswg_dn8_slot;
        let mut var_czbdswg_dn9: f64 = *var_czbdswg_dn9_slot;
        let mut var_czbdswg_rv: f64 = *var_czbdswg_rv_slot;
        let mut var_czbs: f64 = *var_czbs_slot;
        let mut var_czbs_dn0: f64 = *var_czbs_dn0_slot;
        let mut var_czbs_dn10: f64 = *var_czbs_dn10_slot;
        let mut var_czbs_dn13: f64 = *var_czbs_dn13_slot;
        let mut var_czbs_dn2: f64 = *var_czbs_dn2_slot;
        let mut var_czbs_dn4: f64 = *var_czbs_dn4_slot;
        let mut var_czbs_dn5: f64 = *var_czbs_dn5_slot;
        let mut var_czbs_dn6: f64 = *var_czbs_dn6_slot;
        let mut var_czbs_dn7: f64 = *var_czbs_dn7_slot;
        let mut var_czbs_dn8: f64 = *var_czbs_dn8_slot;
        let mut var_czbs_dn9: f64 = *var_czbs_dn9_slot;
        let mut var_czbs_rv: f64 = *var_czbs_rv_slot;
        let mut var_czbssw: f64 = *var_czbssw_slot;
        let mut var_czbssw_dn0: f64 = *var_czbssw_dn0_slot;
        let mut var_czbssw_dn10: f64 = *var_czbssw_dn10_slot;
        let mut var_czbssw_dn13: f64 = *var_czbssw_dn13_slot;
        let mut var_czbssw_dn2: f64 = *var_czbssw_dn2_slot;
        let mut var_czbssw_dn4: f64 = *var_czbssw_dn4_slot;
        let mut var_czbssw_dn5: f64 = *var_czbssw_dn5_slot;
        let mut var_czbssw_dn6: f64 = *var_czbssw_dn6_slot;
        let mut var_czbssw_dn7: f64 = *var_czbssw_dn7_slot;
        let mut var_czbssw_dn8: f64 = *var_czbssw_dn8_slot;
        let mut var_czbssw_dn9: f64 = *var_czbssw_dn9_slot;
        let mut var_czbssw_rv: f64 = *var_czbssw_rv_slot;
        let mut var_czbsswg: f64 = *var_czbsswg_slot;
        let mut var_czbsswg_dn0: f64 = *var_czbsswg_dn0_slot;
        let mut var_czbsswg_dn10: f64 = *var_czbsswg_dn10_slot;
        let mut var_czbsswg_dn13: f64 = *var_czbsswg_dn13_slot;
        let mut var_czbsswg_dn2: f64 = *var_czbsswg_dn2_slot;
        let mut var_czbsswg_dn4: f64 = *var_czbsswg_dn4_slot;
        let mut var_czbsswg_dn5: f64 = *var_czbsswg_dn5_slot;
        let mut var_czbsswg_dn6: f64 = *var_czbsswg_dn6_slot;
        let mut var_czbsswg_dn7: f64 = *var_czbsswg_dn7_slot;
        let mut var_czbsswg_dn8: f64 = *var_czbsswg_dn8_slot;
        let mut var_czbsswg_dn9: f64 = *var_czbsswg_dn9_slot;
        let mut var_czbsswg_rv: f64 = *var_czbsswg_rv_slot;
        let mut var_end_of_part_1: f64 = *var_end_of_part_1_slot;
        let mut var_end_of_part_1_rv: f64 = *var_end_of_part_1_rv_slot;
        let mut var_exptempd: f64 = *var_exptempd_slot;
        let mut var_exptempd_dn0: f64 = *var_exptempd_dn0_slot;
        let mut var_exptempd_dn10: f64 = *var_exptempd_dn10_slot;
        let mut var_exptempd_dn13: f64 = *var_exptempd_dn13_slot;
        let mut var_exptempd_dn2: f64 = *var_exptempd_dn2_slot;
        let mut var_exptempd_dn4: f64 = *var_exptempd_dn4_slot;
        let mut var_exptempd_dn5: f64 = *var_exptempd_dn5_slot;
        let mut var_exptempd_dn6: f64 = *var_exptempd_dn6_slot;
        let mut var_exptempd_dn7: f64 = *var_exptempd_dn7_slot;
        let mut var_exptempd_dn8: f64 = *var_exptempd_dn8_slot;
        let mut var_exptempd_dn9: f64 = *var_exptempd_dn9_slot;
        let mut var_exptempd_rv: f64 = *var_exptempd_rv_slot;
        let mut var_exptemps: f64 = *var_exptemps_slot;
        let mut var_exptemps_dn0: f64 = *var_exptemps_dn0_slot;
        let mut var_exptemps_dn10: f64 = *var_exptemps_dn10_slot;
        let mut var_exptemps_dn13: f64 = *var_exptemps_dn13_slot;
        let mut var_exptemps_dn2: f64 = *var_exptemps_dn2_slot;
        let mut var_exptemps_dn4: f64 = *var_exptemps_dn4_slot;
        let mut var_exptemps_dn5: f64 = *var_exptemps_dn5_slot;
        let mut var_exptemps_dn6: f64 = *var_exptemps_dn6_slot;
        let mut var_exptemps_dn7: f64 = *var_exptemps_dn7_slot;
        let mut var_exptemps_dn8: f64 = *var_exptemps_dn8_slot;
        let mut var_exptemps_dn9: f64 = *var_exptemps_dn9_slot;
        let mut var_exptemps_rv: f64 = *var_exptemps_rv_slot;
        let mut var_flg_brk1: f64 = *var_flg_brk1_slot;
        let mut var_flg_brk1_rv: f64 = *var_flg_brk1_rv_slot;
        let mut var_flg_brk2: f64 = *var_flg_brk2_slot;
        let mut var_flg_brk2_rv: f64 = *var_flg_brk2_rv_slot;
        let mut var_isbd: f64 = *var_isbd_slot;
        let mut var_isbd2_btm: f64 = *var_isbd2_btm_slot;
        let mut var_isbd2_btm_dn0: f64 = *var_isbd2_btm_dn0_slot;
        let mut var_isbd2_btm_dn10: f64 = *var_isbd2_btm_dn10_slot;
        let mut var_isbd2_btm_dn13: f64 = *var_isbd2_btm_dn13_slot;
        let mut var_isbd2_btm_dn2: f64 = *var_isbd2_btm_dn2_slot;
        let mut var_isbd2_btm_dn4: f64 = *var_isbd2_btm_dn4_slot;
        let mut var_isbd2_btm_dn5: f64 = *var_isbd2_btm_dn5_slot;
        let mut var_isbd2_btm_dn6: f64 = *var_isbd2_btm_dn6_slot;
        let mut var_isbd2_btm_dn7: f64 = *var_isbd2_btm_dn7_slot;
        let mut var_isbd2_btm_dn8: f64 = *var_isbd2_btm_dn8_slot;
        let mut var_isbd2_btm_dn9: f64 = *var_isbd2_btm_dn9_slot;
        let mut var_isbd2_btm_rv: f64 = *var_isbd2_btm_rv_slot;
        let mut var_isbd2_swg: f64 = *var_isbd2_swg_slot;
        let mut var_isbd2_swg_dn0: f64 = *var_isbd2_swg_dn0_slot;
        let mut var_isbd2_swg_dn10: f64 = *var_isbd2_swg_dn10_slot;
        let mut var_isbd2_swg_dn13: f64 = *var_isbd2_swg_dn13_slot;
        let mut var_isbd2_swg_dn2: f64 = *var_isbd2_swg_dn2_slot;
        let mut var_isbd2_swg_dn4: f64 = *var_isbd2_swg_dn4_slot;
        let mut var_isbd2_swg_dn5: f64 = *var_isbd2_swg_dn5_slot;
        let mut var_isbd2_swg_dn6: f64 = *var_isbd2_swg_dn6_slot;
        let mut var_isbd2_swg_dn7: f64 = *var_isbd2_swg_dn7_slot;
        let mut var_isbd2_swg_dn8: f64 = *var_isbd2_swg_dn8_slot;
        let mut var_isbd2_swg_dn9: f64 = *var_isbd2_swg_dn9_slot;
        let mut var_isbd2_swg_rv: f64 = *var_isbd2_swg_rv_slot;
        let mut var_isbd2_sws: f64 = *var_isbd2_sws_slot;
        let mut var_isbd2_sws_dn0: f64 = *var_isbd2_sws_dn0_slot;
        let mut var_isbd2_sws_dn10: f64 = *var_isbd2_sws_dn10_slot;
        let mut var_isbd2_sws_dn13: f64 = *var_isbd2_sws_dn13_slot;
        let mut var_isbd2_sws_dn2: f64 = *var_isbd2_sws_dn2_slot;
        let mut var_isbd2_sws_dn4: f64 = *var_isbd2_sws_dn4_slot;
        let mut var_isbd2_sws_dn5: f64 = *var_isbd2_sws_dn5_slot;
        let mut var_isbd2_sws_dn6: f64 = *var_isbd2_sws_dn6_slot;
        let mut var_isbd2_sws_dn7: f64 = *var_isbd2_sws_dn7_slot;
        let mut var_isbd2_sws_dn8: f64 = *var_isbd2_sws_dn8_slot;
        let mut var_isbd2_sws_dn9: f64 = *var_isbd2_sws_dn9_slot;
        let mut var_isbd2_sws_rv: f64 = *var_isbd2_sws_rv_slot;
        let mut var_isbd_btm: f64 = *var_isbd_btm_slot;
        let mut var_isbd_btm_dn0: f64 = *var_isbd_btm_dn0_slot;
        let mut var_isbd_btm_dn10: f64 = *var_isbd_btm_dn10_slot;
        let mut var_isbd_btm_dn13: f64 = *var_isbd_btm_dn13_slot;
        let mut var_isbd_btm_dn2: f64 = *var_isbd_btm_dn2_slot;
        let mut var_isbd_btm_dn4: f64 = *var_isbd_btm_dn4_slot;
        let mut var_isbd_btm_dn5: f64 = *var_isbd_btm_dn5_slot;
        let mut var_isbd_btm_dn6: f64 = *var_isbd_btm_dn6_slot;
        let mut var_isbd_btm_dn7: f64 = *var_isbd_btm_dn7_slot;
        let mut var_isbd_btm_dn8: f64 = *var_isbd_btm_dn8_slot;
        let mut var_isbd_btm_dn9: f64 = *var_isbd_btm_dn9_slot;
        let mut var_isbd_btm_rv: f64 = *var_isbd_btm_rv_slot;
        let mut var_isbd_dn0: f64 = *var_isbd_dn0_slot;
        let mut var_isbd_dn10: f64 = *var_isbd_dn10_slot;
        let mut var_isbd_dn13: f64 = *var_isbd_dn13_slot;
        let mut var_isbd_dn2: f64 = *var_isbd_dn2_slot;
        let mut var_isbd_dn4: f64 = *var_isbd_dn4_slot;
        let mut var_isbd_dn5: f64 = *var_isbd_dn5_slot;
        let mut var_isbd_dn6: f64 = *var_isbd_dn6_slot;
        let mut var_isbd_dn7: f64 = *var_isbd_dn7_slot;
        let mut var_isbd_dn8: f64 = *var_isbd_dn8_slot;
        let mut var_isbd_dn9: f64 = *var_isbd_dn9_slot;
        let mut var_isbd_rv: f64 = *var_isbd_rv_slot;
        let mut var_isbd_swg: f64 = *var_isbd_swg_slot;
        let mut var_isbd_swg_dn0: f64 = *var_isbd_swg_dn0_slot;
        let mut var_isbd_swg_dn10: f64 = *var_isbd_swg_dn10_slot;
        let mut var_isbd_swg_dn13: f64 = *var_isbd_swg_dn13_slot;
        let mut var_isbd_swg_dn2: f64 = *var_isbd_swg_dn2_slot;
        let mut var_isbd_swg_dn4: f64 = *var_isbd_swg_dn4_slot;
        let mut var_isbd_swg_dn5: f64 = *var_isbd_swg_dn5_slot;
        let mut var_isbd_swg_dn6: f64 = *var_isbd_swg_dn6_slot;
        let mut var_isbd_swg_dn7: f64 = *var_isbd_swg_dn7_slot;
        let mut var_isbd_swg_dn8: f64 = *var_isbd_swg_dn8_slot;
        let mut var_isbd_swg_dn9: f64 = *var_isbd_swg_dn9_slot;
        let mut var_isbd_swg_rv: f64 = *var_isbd_swg_rv_slot;
        let mut var_isbd_sws: f64 = *var_isbd_sws_slot;
        let mut var_isbd_sws_dn0: f64 = *var_isbd_sws_dn0_slot;
        let mut var_isbd_sws_dn10: f64 = *var_isbd_sws_dn10_slot;
        let mut var_isbd_sws_dn13: f64 = *var_isbd_sws_dn13_slot;
        let mut var_isbd_sws_dn2: f64 = *var_isbd_sws_dn2_slot;
        let mut var_isbd_sws_dn4: f64 = *var_isbd_sws_dn4_slot;
        let mut var_isbd_sws_dn5: f64 = *var_isbd_sws_dn5_slot;
        let mut var_isbd_sws_dn6: f64 = *var_isbd_sws_dn6_slot;
        let mut var_isbd_sws_dn7: f64 = *var_isbd_sws_dn7_slot;
        let mut var_isbd_sws_dn8: f64 = *var_isbd_sws_dn8_slot;
        let mut var_isbd_sws_dn9: f64 = *var_isbd_sws_dn9_slot;
        let mut var_isbd_sws_rv: f64 = *var_isbd_sws_rv_slot;
        let mut var_isbs: f64 = *var_isbs_slot;
        let mut var_isbs_btm: f64 = *var_isbs_btm_slot;
        let mut var_isbs_btm_dn0: f64 = *var_isbs_btm_dn0_slot;
        let mut var_isbs_btm_dn10: f64 = *var_isbs_btm_dn10_slot;
        let mut var_isbs_btm_dn13: f64 = *var_isbs_btm_dn13_slot;
        let mut var_isbs_btm_dn2: f64 = *var_isbs_btm_dn2_slot;
        let mut var_isbs_btm_dn4: f64 = *var_isbs_btm_dn4_slot;
        let mut var_isbs_btm_dn5: f64 = *var_isbs_btm_dn5_slot;
        let mut var_isbs_btm_dn6: f64 = *var_isbs_btm_dn6_slot;
        let mut var_isbs_btm_dn7: f64 = *var_isbs_btm_dn7_slot;
        let mut var_isbs_btm_dn8: f64 = *var_isbs_btm_dn8_slot;
        let mut var_isbs_btm_dn9: f64 = *var_isbs_btm_dn9_slot;
        let mut var_isbs_btm_rv: f64 = *var_isbs_btm_rv_slot;
        let mut var_isbs_dn0: f64 = *var_isbs_dn0_slot;
        let mut var_isbs_dn10: f64 = *var_isbs_dn10_slot;
        let mut var_isbs_dn13: f64 = *var_isbs_dn13_slot;
        let mut var_isbs_dn2: f64 = *var_isbs_dn2_slot;
        let mut var_isbs_dn4: f64 = *var_isbs_dn4_slot;
        let mut var_isbs_dn5: f64 = *var_isbs_dn5_slot;
        let mut var_isbs_dn6: f64 = *var_isbs_dn6_slot;
        let mut var_isbs_dn7: f64 = *var_isbs_dn7_slot;
        let mut var_isbs_dn8: f64 = *var_isbs_dn8_slot;
        let mut var_isbs_dn9: f64 = *var_isbs_dn9_slot;
        let mut var_isbs_rv: f64 = *var_isbs_rv_slot;
        let mut var_jd_expcd: f64 = *var_jd_expcd_slot;
        let mut var_jd_expcd_dn0: f64 = *var_jd_expcd_dn0_slot;
        let mut var_jd_expcd_dn10: f64 = *var_jd_expcd_dn10_slot;
        let mut var_jd_expcd_dn13: f64 = *var_jd_expcd_dn13_slot;
        let mut var_jd_expcd_dn2: f64 = *var_jd_expcd_dn2_slot;
        let mut var_jd_expcd_dn4: f64 = *var_jd_expcd_dn4_slot;
        let mut var_jd_expcd_dn5: f64 = *var_jd_expcd_dn5_slot;
        let mut var_jd_expcd_dn6: f64 = *var_jd_expcd_dn6_slot;
        let mut var_jd_expcd_dn7: f64 = *var_jd_expcd_dn7_slot;
        let mut var_jd_expcd_dn8: f64 = *var_jd_expcd_dn8_slot;
        let mut var_jd_expcd_dn9: f64 = *var_jd_expcd_dn9_slot;
        let mut var_jd_expcd_rv: f64 = *var_jd_expcd_rv_slot;
        let mut var_jd_expcs: f64 = *var_jd_expcs_slot;
        let mut var_jd_expcs_dn0: f64 = *var_jd_expcs_dn0_slot;
        let mut var_jd_expcs_dn10: f64 = *var_jd_expcs_dn10_slot;
        let mut var_jd_expcs_dn13: f64 = *var_jd_expcs_dn13_slot;
        let mut var_jd_expcs_dn2: f64 = *var_jd_expcs_dn2_slot;
        let mut var_jd_expcs_dn4: f64 = *var_jd_expcs_dn4_slot;
        let mut var_jd_expcs_dn5: f64 = *var_jd_expcs_dn5_slot;
        let mut var_jd_expcs_dn6: f64 = *var_jd_expcs_dn6_slot;
        let mut var_jd_expcs_dn7: f64 = *var_jd_expcs_dn7_slot;
        let mut var_jd_expcs_dn8: f64 = *var_jd_expcs_dn8_slot;
        let mut var_jd_expcs_dn9: f64 = *var_jd_expcs_dn9_slot;
        let mut var_jd_expcs_rv: f64 = *var_jd_expcs_rv_slot;
        let mut var_jd_nvtm_invd: f64 = *var_jd_nvtm_invd_slot;
        let mut var_jd_nvtm_invd_dn0: f64 = *var_jd_nvtm_invd_dn0_slot;
        let mut var_jd_nvtm_invd_dn10: f64 = *var_jd_nvtm_invd_dn10_slot;
        let mut var_jd_nvtm_invd_dn13: f64 = *var_jd_nvtm_invd_dn13_slot;
        let mut var_jd_nvtm_invd_dn2: f64 = *var_jd_nvtm_invd_dn2_slot;
        let mut var_jd_nvtm_invd_dn4: f64 = *var_jd_nvtm_invd_dn4_slot;
        let mut var_jd_nvtm_invd_dn5: f64 = *var_jd_nvtm_invd_dn5_slot;
        let mut var_jd_nvtm_invd_dn6: f64 = *var_jd_nvtm_invd_dn6_slot;
        let mut var_jd_nvtm_invd_dn7: f64 = *var_jd_nvtm_invd_dn7_slot;
        let mut var_jd_nvtm_invd_dn8: f64 = *var_jd_nvtm_invd_dn8_slot;
        let mut var_jd_nvtm_invd_dn9: f64 = *var_jd_nvtm_invd_dn9_slot;
        let mut var_jd_nvtm_invd_rv: f64 = *var_jd_nvtm_invd_rv_slot;
        let mut var_jd_nvtm_invs: f64 = *var_jd_nvtm_invs_slot;
        let mut var_jd_nvtm_invs_dn0: f64 = *var_jd_nvtm_invs_dn0_slot;
        let mut var_jd_nvtm_invs_dn10: f64 = *var_jd_nvtm_invs_dn10_slot;
        let mut var_jd_nvtm_invs_dn13: f64 = *var_jd_nvtm_invs_dn13_slot;
        let mut var_jd_nvtm_invs_dn2: f64 = *var_jd_nvtm_invs_dn2_slot;
        let mut var_jd_nvtm_invs_dn4: f64 = *var_jd_nvtm_invs_dn4_slot;
        let mut var_jd_nvtm_invs_dn5: f64 = *var_jd_nvtm_invs_dn5_slot;
        let mut var_jd_nvtm_invs_dn6: f64 = *var_jd_nvtm_invs_dn6_slot;
        let mut var_jd_nvtm_invs_dn7: f64 = *var_jd_nvtm_invs_dn7_slot;
        let mut var_jd_nvtm_invs_dn8: f64 = *var_jd_nvtm_invs_dn8_slot;
        let mut var_jd_nvtm_invs_dn9: f64 = *var_jd_nvtm_invs_dn9_slot;
        let mut var_jd_nvtm_invs_rv: f64 = *var_jd_nvtm_invs_rv_slot;
        let mut var_pzbd: f64 = *var_pzbd_slot;
        let mut var_pzbd_dn0: f64 = *var_pzbd_dn0_slot;
        let mut var_pzbd_dn10: f64 = *var_pzbd_dn10_slot;
        let mut var_pzbd_dn13: f64 = *var_pzbd_dn13_slot;
        let mut var_pzbd_dn2: f64 = *var_pzbd_dn2_slot;
        let mut var_pzbd_dn4: f64 = *var_pzbd_dn4_slot;
        let mut var_pzbd_dn5: f64 = *var_pzbd_dn5_slot;
        let mut var_pzbd_dn6: f64 = *var_pzbd_dn6_slot;
        let mut var_pzbd_dn7: f64 = *var_pzbd_dn7_slot;
        let mut var_pzbd_dn8: f64 = *var_pzbd_dn8_slot;
        let mut var_pzbd_dn9: f64 = *var_pzbd_dn9_slot;
        let mut var_pzbd_rv: f64 = *var_pzbd_rv_slot;
        let mut var_pzbdsw: f64 = *var_pzbdsw_slot;
        let mut var_pzbdsw_dn0: f64 = *var_pzbdsw_dn0_slot;
        let mut var_pzbdsw_dn10: f64 = *var_pzbdsw_dn10_slot;
        let mut var_pzbdsw_dn13: f64 = *var_pzbdsw_dn13_slot;
        let mut var_pzbdsw_dn2: f64 = *var_pzbdsw_dn2_slot;
        let mut var_pzbdsw_dn4: f64 = *var_pzbdsw_dn4_slot;
        let mut var_pzbdsw_dn5: f64 = *var_pzbdsw_dn5_slot;
        let mut var_pzbdsw_dn6: f64 = *var_pzbdsw_dn6_slot;
        let mut var_pzbdsw_dn7: f64 = *var_pzbdsw_dn7_slot;
        let mut var_pzbdsw_dn8: f64 = *var_pzbdsw_dn8_slot;
        let mut var_pzbdsw_dn9: f64 = *var_pzbdsw_dn9_slot;
        let mut var_pzbdsw_rv: f64 = *var_pzbdsw_rv_slot;
        let mut var_pzbdswg: f64 = *var_pzbdswg_slot;
        let mut var_pzbdswg_dn0: f64 = *var_pzbdswg_dn0_slot;
        let mut var_pzbdswg_dn10: f64 = *var_pzbdswg_dn10_slot;
        let mut var_pzbdswg_dn13: f64 = *var_pzbdswg_dn13_slot;
        let mut var_pzbdswg_dn2: f64 = *var_pzbdswg_dn2_slot;
        let mut var_pzbdswg_dn4: f64 = *var_pzbdswg_dn4_slot;
        let mut var_pzbdswg_dn5: f64 = *var_pzbdswg_dn5_slot;
        let mut var_pzbdswg_dn6: f64 = *var_pzbdswg_dn6_slot;
        let mut var_pzbdswg_dn7: f64 = *var_pzbdswg_dn7_slot;
        let mut var_pzbdswg_dn8: f64 = *var_pzbdswg_dn8_slot;
        let mut var_pzbdswg_dn9: f64 = *var_pzbdswg_dn9_slot;
        let mut var_pzbdswg_rv: f64 = *var_pzbdswg_rv_slot;
        let mut var_pzbs: f64 = *var_pzbs_slot;
        let mut var_pzbs_dn0: f64 = *var_pzbs_dn0_slot;
        let mut var_pzbs_dn10: f64 = *var_pzbs_dn10_slot;
        let mut var_pzbs_dn13: f64 = *var_pzbs_dn13_slot;
        let mut var_pzbs_dn2: f64 = *var_pzbs_dn2_slot;
        let mut var_pzbs_dn4: f64 = *var_pzbs_dn4_slot;
        let mut var_pzbs_dn5: f64 = *var_pzbs_dn5_slot;
        let mut var_pzbs_dn6: f64 = *var_pzbs_dn6_slot;
        let mut var_pzbs_dn7: f64 = *var_pzbs_dn7_slot;
        let mut var_pzbs_dn8: f64 = *var_pzbs_dn8_slot;
        let mut var_pzbs_dn9: f64 = *var_pzbs_dn9_slot;
        let mut var_pzbs_rv: f64 = *var_pzbs_rv_slot;
        let mut var_pzbssw: f64 = *var_pzbssw_slot;
        let mut var_pzbssw_dn0: f64 = *var_pzbssw_dn0_slot;
        let mut var_pzbssw_dn10: f64 = *var_pzbssw_dn10_slot;
        let mut var_pzbssw_dn13: f64 = *var_pzbssw_dn13_slot;
        let mut var_pzbssw_dn2: f64 = *var_pzbssw_dn2_slot;
        let mut var_pzbssw_dn4: f64 = *var_pzbssw_dn4_slot;
        let mut var_pzbssw_dn5: f64 = *var_pzbssw_dn5_slot;
        let mut var_pzbssw_dn6: f64 = *var_pzbssw_dn6_slot;
        let mut var_pzbssw_dn7: f64 = *var_pzbssw_dn7_slot;
        let mut var_pzbssw_dn8: f64 = *var_pzbssw_dn8_slot;
        let mut var_pzbssw_dn9: f64 = *var_pzbssw_dn9_slot;
        let mut var_pzbssw_rv: f64 = *var_pzbssw_rv_slot;
        let mut var_pzbsswg: f64 = *var_pzbsswg_slot;
        let mut var_pzbsswg_dn0: f64 = *var_pzbsswg_dn0_slot;
        let mut var_pzbsswg_dn10: f64 = *var_pzbsswg_dn10_slot;
        let mut var_pzbsswg_dn13: f64 = *var_pzbsswg_dn13_slot;
        let mut var_pzbsswg_dn2: f64 = *var_pzbsswg_dn2_slot;
        let mut var_pzbsswg_dn4: f64 = *var_pzbsswg_dn4_slot;
        let mut var_pzbsswg_dn5: f64 = *var_pzbsswg_dn5_slot;
        let mut var_pzbsswg_dn6: f64 = *var_pzbsswg_dn6_slot;
        let mut var_pzbsswg_dn7: f64 = *var_pzbsswg_dn7_slot;
        let mut var_pzbsswg_dn8: f64 = *var_pzbsswg_dn8_slot;
        let mut var_pzbsswg_dn9: f64 = *var_pzbsswg_dn9_slot;
        let mut var_pzbsswg_rv: f64 = *var_pzbsswg_rv_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn0: f64 = *var_qbd_dn0_slot;
        let mut var_qbd_dn10: f64 = *var_qbd_dn10_slot;
        let mut var_qbd_dn13: f64 = *var_qbd_dn13_slot;
        let mut var_qbd_dn15: f64 = *var_qbd_dn15_slot;
        let mut var_qbd_dn16: f64 = *var_qbd_dn16_slot;
        let mut var_qbd_dn17: f64 = *var_qbd_dn17_slot;
        let mut var_qbd_dn2: f64 = *var_qbd_dn2_slot;
        let mut var_qbd_dn4: f64 = *var_qbd_dn4_slot;
        let mut var_qbd_dn5: f64 = *var_qbd_dn5_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbd_dn8: f64 = *var_qbd_dn8_slot;
        let mut var_qbd_dn9: f64 = *var_qbd_dn9_slot;
        let mut var_qbd_qs: f64 = *var_qbd_qs_slot;
        let mut var_qbd_qs_dn0: f64 = *var_qbd_qs_dn0_slot;
        let mut var_qbd_qs_dn10: f64 = *var_qbd_qs_dn10_slot;
        let mut var_qbd_qs_dn13: f64 = *var_qbd_qs_dn13_slot;
        let mut var_qbd_qs_dn2: f64 = *var_qbd_qs_dn2_slot;
        let mut var_qbd_qs_dn4: f64 = *var_qbd_qs_dn4_slot;
        let mut var_qbd_qs_dn5: f64 = *var_qbd_qs_dn5_slot;
        let mut var_qbd_qs_dn6: f64 = *var_qbd_qs_dn6_slot;
        let mut var_qbd_qs_dn7: f64 = *var_qbd_qs_dn7_slot;
        let mut var_qbd_qs_dn8: f64 = *var_qbd_qs_dn8_slot;
        let mut var_qbd_qs_dn9: f64 = *var_qbd_qs_dn9_slot;
        let mut var_qbd_qs_rv: f64 = *var_qbd_qs_rv_slot;
        let mut var_qbd_rv: f64 = *var_qbd_rv_slot;
        let mut var_qbdi: f64 = *var_qbdi_slot;
        let mut var_qbdi_dn0: f64 = *var_qbdi_dn0_slot;
        let mut var_qbdi_dn10: f64 = *var_qbdi_dn10_slot;
        let mut var_qbdi_dn13: f64 = *var_qbdi_dn13_slot;
        let mut var_qbdi_dn2: f64 = *var_qbdi_dn2_slot;
        let mut var_qbdi_dn4: f64 = *var_qbdi_dn4_slot;
        let mut var_qbdi_dn5: f64 = *var_qbdi_dn5_slot;
        let mut var_qbdi_dn6: f64 = *var_qbdi_dn6_slot;
        let mut var_qbdi_dn7: f64 = *var_qbdi_dn7_slot;
        let mut var_qbdi_dn8: f64 = *var_qbdi_dn8_slot;
        let mut var_qbdi_dn9: f64 = *var_qbdi_dn9_slot;
        let mut var_qbdi_rv: f64 = *var_qbdi_rv_slot;
        let mut var_qbsi: f64 = *var_qbsi_slot;
        let mut var_qbsi_dn0: f64 = *var_qbsi_dn0_slot;
        let mut var_qbsi_dn10: f64 = *var_qbsi_dn10_slot;
        let mut var_qbsi_dn13: f64 = *var_qbsi_dn13_slot;
        let mut var_qbsi_dn2: f64 = *var_qbsi_dn2_slot;
        let mut var_qbsi_dn4: f64 = *var_qbsi_dn4_slot;
        let mut var_qbsi_dn5: f64 = *var_qbsi_dn5_slot;
        let mut var_qbsi_dn6: f64 = *var_qbsi_dn6_slot;
        let mut var_qbsi_dn7: f64 = *var_qbsi_dn7_slot;
        let mut var_qbsi_dn8: f64 = *var_qbsi_dn8_slot;
        let mut var_qbsi_dn9: f64 = *var_qbsi_dn9_slot;
        let mut var_qbsi_rv: f64 = *var_qbsi_rv_slot;
        let mut var_sarg: f64 = *var_sarg_slot;
        let mut var_sarg_dn0: f64 = *var_sarg_dn0_slot;
        let mut var_sarg_dn10: f64 = *var_sarg_dn10_slot;
        let mut var_sarg_dn13: f64 = *var_sarg_dn13_slot;
        let mut var_sarg_dn2: f64 = *var_sarg_dn2_slot;
        let mut var_sarg_dn4: f64 = *var_sarg_dn4_slot;
        let mut var_sarg_dn5: f64 = *var_sarg_dn5_slot;
        let mut var_sarg_dn6: f64 = *var_sarg_dn6_slot;
        let mut var_sarg_dn7: f64 = *var_sarg_dn7_slot;
        let mut var_sarg_dn8: f64 = *var_sarg_dn8_slot;
        let mut var_sarg_dn9: f64 = *var_sarg_dn9_slot;
        let mut var_sarg_rv: f64 = *var_sarg_rv_slot;
        let mut var_start_of_loopl: f64 = *var_start_of_loopl_slot;
        let mut var_start_of_loopl_rv: f64 = *var_start_of_loopl_rv_slot;
        let mut var_start_of_mobility: f64 = *var_start_of_mobility_slot;
        let mut var_start_of_mobility_rv: f64 = *var_start_of_mobility_rv_slot;
        let mut var_vbd_jct: f64 = *var_vbd_jct_slot;
        let mut var_vbd_jct_dn0: f64 = *var_vbd_jct_dn0_slot;
        let mut var_vbd_jct_dn9: f64 = *var_vbd_jct_dn9_slot;
        let mut var_vbd_jct_rv: f64 = *var_vbd_jct_rv_slot;
        let mut var_vbdi_jct: f64 = *var_vbdi_jct_slot;
        let mut var_vbdi_jct_dn5: f64 = *var_vbdi_jct_dn5_slot;
        let mut var_vbdi_jct_dn8: f64 = *var_vbdi_jct_dn8_slot;
        let mut var_vbdi_jct_rv: f64 = *var_vbdi_jct_rv_slot;
        let mut var_vbdt: f64 = *var_vbdt_slot;
        let mut var_vbdt_dn0: f64 = *var_vbdt_dn0_slot;
        let mut var_vbdt_dn10: f64 = *var_vbdt_dn10_slot;
        let mut var_vbdt_dn13: f64 = *var_vbdt_dn13_slot;
        let mut var_vbdt_dn2: f64 = *var_vbdt_dn2_slot;
        let mut var_vbdt_dn4: f64 = *var_vbdt_dn4_slot;
        let mut var_vbdt_dn5: f64 = *var_vbdt_dn5_slot;
        let mut var_vbdt_dn6: f64 = *var_vbdt_dn6_slot;
        let mut var_vbdt_dn7: f64 = *var_vbdt_dn7_slot;
        let mut var_vbdt_dn8: f64 = *var_vbdt_dn8_slot;
        let mut var_vbdt_dn9: f64 = *var_vbdt_dn9_slot;
        let mut var_vbdt_rv: f64 = *var_vbdt_rv_slot;
        let mut var_vbpdp: f64 = *var_vbpdp_slot;
        let mut var_vbpdp_dn5: f64 = *var_vbpdp_dn5_slot;
        let mut var_vbpdp_dn8: f64 = *var_vbpdp_dn8_slot;
        let mut var_vbpdp_rv: f64 = *var_vbpdp_rv_slot;
        let mut var_vbpsp: f64 = *var_vbpsp_slot;
        let mut var_vbpsp_dn7: f64 = *var_vbpsp_dn7_slot;
        let mut var_vbpsp_dn8: f64 = *var_vbpsp_dn8_slot;
        let mut var_vbpsp_rv: f64 = *var_vbpsp_rv_slot;
        let mut var_vbs_jct: f64 = *var_vbs_jct_slot;
        let mut var_vbs_jct_dn10: f64 = *var_vbs_jct_dn10_slot;
        let mut var_vbs_jct_dn2: f64 = *var_vbs_jct_dn2_slot;
        let mut var_vbs_jct_rv: f64 = *var_vbs_jct_rv_slot;
        let mut var_vbsi_jct: f64 = *var_vbsi_jct_slot;
        let mut var_vbsi_jct_dn7: f64 = *var_vbsi_jct_dn7_slot;
        let mut var_vbsi_jct_dn8: f64 = *var_vbsi_jct_dn8_slot;
        let mut var_vbsi_jct_rv: f64 = *var_vbsi_jct_rv_slot;
        let mut var_vbst: f64 = *var_vbst_slot;
        let mut var_vbst_dn0: f64 = *var_vbst_dn0_slot;
        let mut var_vbst_dn10: f64 = *var_vbst_dn10_slot;
        let mut var_vbst_dn13: f64 = *var_vbst_dn13_slot;
        let mut var_vbst_dn2: f64 = *var_vbst_dn2_slot;
        let mut var_vbst_dn4: f64 = *var_vbst_dn4_slot;
        let mut var_vbst_dn5: f64 = *var_vbst_dn5_slot;
        let mut var_vbst_dn6: f64 = *var_vbst_dn6_slot;
        let mut var_vbst_dn7: f64 = *var_vbst_dn7_slot;
        let mut var_vbst_dn8: f64 = *var_vbst_dn8_slot;
        let mut var_vbst_dn9: f64 = *var_vbst_dn9_slot;
        let mut var_vbst_rv: f64 = *var_vbst_rv_slot;
        let mut var_vdbd: f64 = *var_vdbd_slot;
        let mut var_vdbd_dn0: f64 = *var_vdbd_dn0_slot;
        let mut var_vdbd_dn9: f64 = *var_vdbd_dn9_slot;
        let mut var_vdbd_rv: f64 = *var_vdbd_rv_slot;
        let mut var_vsbs: f64 = *var_vsbs_slot;
        let mut var_vsbs_dn10: f64 = *var_vsbs_dn10_slot;
        let mut var_vsbs_dn2: f64 = *var_vsbs_dn2_slot;
        let mut var_vsbs_rv: f64 = *var_vsbs_rv_slot;

        var_qbd = 0.0;
        var_qbd_dn0 = 0.0;
        var_qbd_dn2 = 0.0;
        var_qbd_dn4 = 0.0;
        var_qbd_dn5 = 0.0;
        var_qbd_dn6 = 0.0;
        var_qbd_dn7 = 0.0;
        var_qbd_dn8 = 0.0;
        var_qbd_dn9 = 0.0;
        var_qbd_dn10 = 0.0;
        var_qbd_dn13 = 0.0;
        var_qbd_dn15 = 0.0;
        var_qbd_dn16 = 0.0;
        var_qbd_dn17 = 0.0;
        var_qbd_rv = 0.0;

        var_qbsi = 0.0;
        var_qbsi_dn0 = 0.0;
        var_qbsi_dn2 = 0.0;
        var_qbsi_dn4 = 0.0;
        var_qbsi_dn5 = 0.0;
        var_qbsi_dn6 = 0.0;
        var_qbsi_dn7 = 0.0;
        var_qbsi_dn8 = 0.0;
        var_qbsi_dn9 = 0.0;
        var_qbsi_dn10 = 0.0;
        var_qbsi_dn13 = 0.0;
        var_qbsi_rv = 0.0;

        var_qbdi = 0.0;
        var_qbdi_dn0 = 0.0;
        var_qbdi_dn2 = 0.0;
        var_qbdi_dn4 = 0.0;
        var_qbdi_dn5 = 0.0;
        var_qbdi_dn6 = 0.0;
        var_qbdi_dn7 = 0.0;
        var_qbdi_dn8 = 0.0;
        var_qbdi_dn9 = 0.0;
        var_qbdi_dn10 = 0.0;
        var_qbdi_dn13 = 0.0;
        var_qbdi_rv = 0.0;

        var_czbd = 0.0;
        var_czbd_dn0 = 0.0;
        var_czbd_dn2 = 0.0;
        var_czbd_dn4 = 0.0;
        var_czbd_dn5 = 0.0;
        var_czbd_dn6 = 0.0;
        var_czbd_dn7 = 0.0;
        var_czbd_dn8 = 0.0;
        var_czbd_dn9 = 0.0;
        var_czbd_dn10 = 0.0;
        var_czbd_dn13 = 0.0;
        var_czbd_rv = 0.0;

        var_czbdsw = 0.0;
        var_czbdsw_dn0 = 0.0;
        var_czbdsw_dn2 = 0.0;
        var_czbdsw_dn4 = 0.0;
        var_czbdsw_dn5 = 0.0;
        var_czbdsw_dn6 = 0.0;
        var_czbdsw_dn7 = 0.0;
        var_czbdsw_dn8 = 0.0;
        var_czbdsw_dn9 = 0.0;
        var_czbdsw_dn10 = 0.0;
        var_czbdsw_dn13 = 0.0;
        var_czbdsw_rv = 0.0;

        var_czbdswg = 0.0;
        var_czbdswg_dn0 = 0.0;
        var_czbdswg_dn2 = 0.0;
        var_czbdswg_dn4 = 0.0;
        var_czbdswg_dn5 = 0.0;
        var_czbdswg_dn6 = 0.0;
        var_czbdswg_dn7 = 0.0;
        var_czbdswg_dn8 = 0.0;
        var_czbdswg_dn9 = 0.0;
        var_czbdswg_dn10 = 0.0;
        var_czbdswg_dn13 = 0.0;
        var_czbdswg_rv = 0.0;

        var_czbs = 0.0;
        var_czbs_dn0 = 0.0;
        var_czbs_dn2 = 0.0;
        var_czbs_dn4 = 0.0;
        var_czbs_dn5 = 0.0;
        var_czbs_dn6 = 0.0;
        var_czbs_dn7 = 0.0;
        var_czbs_dn8 = 0.0;
        var_czbs_dn9 = 0.0;
        var_czbs_dn10 = 0.0;
        var_czbs_dn13 = 0.0;
        var_czbs_rv = 0.0;

        var_czbssw = 0.0;
        var_czbssw_dn0 = 0.0;
        var_czbssw_dn2 = 0.0;
        var_czbssw_dn4 = 0.0;
        var_czbssw_dn5 = 0.0;
        var_czbssw_dn6 = 0.0;
        var_czbssw_dn7 = 0.0;
        var_czbssw_dn8 = 0.0;
        var_czbssw_dn9 = 0.0;
        var_czbssw_dn10 = 0.0;
        var_czbssw_dn13 = 0.0;
        var_czbssw_rv = 0.0;

        var_czbsswg = 0.0;
        var_czbsswg_dn0 = 0.0;
        var_czbsswg_dn2 = 0.0;
        var_czbsswg_dn4 = 0.0;
        var_czbsswg_dn5 = 0.0;
        var_czbsswg_dn6 = 0.0;
        var_czbsswg_dn7 = 0.0;
        var_czbsswg_dn8 = 0.0;
        var_czbsswg_dn9 = 0.0;
        var_czbsswg_dn10 = 0.0;
        var_czbsswg_dn13 = 0.0;
        var_czbsswg_rv = 0.0;

        var_pzbd = 0.0;
        var_pzbd_dn0 = 0.0;
        var_pzbd_dn2 = 0.0;
        var_pzbd_dn4 = 0.0;
        var_pzbd_dn5 = 0.0;
        var_pzbd_dn6 = 0.0;
        var_pzbd_dn7 = 0.0;
        var_pzbd_dn8 = 0.0;
        var_pzbd_dn9 = 0.0;
        var_pzbd_dn10 = 0.0;
        var_pzbd_dn13 = 0.0;
        var_pzbd_rv = 0.0;

        var_pzbdsw = 0.0;
        var_pzbdsw_dn0 = 0.0;
        var_pzbdsw_dn2 = 0.0;
        var_pzbdsw_dn4 = 0.0;
        var_pzbdsw_dn5 = 0.0;
        var_pzbdsw_dn6 = 0.0;
        var_pzbdsw_dn7 = 0.0;
        var_pzbdsw_dn8 = 0.0;
        var_pzbdsw_dn9 = 0.0;
        var_pzbdsw_dn10 = 0.0;
        var_pzbdsw_dn13 = 0.0;
        var_pzbdsw_rv = 0.0;

        var_pzbdswg = 0.0;
        var_pzbdswg_dn0 = 0.0;
        var_pzbdswg_dn2 = 0.0;
        var_pzbdswg_dn4 = 0.0;
        var_pzbdswg_dn5 = 0.0;
        var_pzbdswg_dn6 = 0.0;
        var_pzbdswg_dn7 = 0.0;
        var_pzbdswg_dn8 = 0.0;
        var_pzbdswg_dn9 = 0.0;
        var_pzbdswg_dn10 = 0.0;
        var_pzbdswg_dn13 = 0.0;
        var_pzbdswg_rv = 0.0;

        var_pzbs = 0.0;
        var_pzbs_dn0 = 0.0;
        var_pzbs_dn2 = 0.0;
        var_pzbs_dn4 = 0.0;
        var_pzbs_dn5 = 0.0;
        var_pzbs_dn6 = 0.0;
        var_pzbs_dn7 = 0.0;
        var_pzbs_dn8 = 0.0;
        var_pzbs_dn9 = 0.0;
        var_pzbs_dn10 = 0.0;
        var_pzbs_dn13 = 0.0;
        var_pzbs_rv = 0.0;

        var_pzbssw = 0.0;
        var_pzbssw_dn0 = 0.0;
        var_pzbssw_dn2 = 0.0;
        var_pzbssw_dn4 = 0.0;
        var_pzbssw_dn5 = 0.0;
        var_pzbssw_dn6 = 0.0;
        var_pzbssw_dn7 = 0.0;
        var_pzbssw_dn8 = 0.0;
        var_pzbssw_dn9 = 0.0;
        var_pzbssw_dn10 = 0.0;
        var_pzbssw_dn13 = 0.0;
        var_pzbssw_rv = 0.0;

        var_pzbsswg = 0.0;
        var_pzbsswg_dn0 = 0.0;
        var_pzbsswg_dn2 = 0.0;
        var_pzbsswg_dn4 = 0.0;
        var_pzbsswg_dn5 = 0.0;
        var_pzbsswg_dn6 = 0.0;
        var_pzbsswg_dn7 = 0.0;
        var_pzbsswg_dn8 = 0.0;
        var_pzbsswg_dn9 = 0.0;
        var_pzbsswg_dn10 = 0.0;
        var_pzbsswg_dn13 = 0.0;
        var_pzbsswg_rv = 0.0;

        var_sarg = 0.0;
        var_sarg_dn0 = 0.0;
        var_sarg_dn2 = 0.0;
        var_sarg_dn4 = 0.0;
        var_sarg_dn5 = 0.0;
        var_sarg_dn6 = 0.0;
        var_sarg_dn7 = 0.0;
        var_sarg_dn8 = 0.0;
        var_sarg_dn9 = 0.0;
        var_sarg_dn10 = 0.0;
        var_sarg_dn13 = 0.0;
        var_sarg_rv = 0.0;

        var_vsbs = 0.0;
        var_vsbs_dn2 = 0.0;
        var_vsbs_dn10 = 0.0;
        var_vsbs_rv = 0.0;

        var_vdbd = 0.0;
        var_vdbd_dn0 = 0.0;
        var_vdbd_dn9 = 0.0;
        var_vdbd_rv = 0.0;

        var_vbs_jct = 0.0;
        var_vbs_jct_dn2 = 0.0;
        var_vbs_jct_dn10 = 0.0;
        var_vbs_jct_rv = 0.0;

        var_vbd_jct = 0.0;
        var_vbd_jct_dn0 = 0.0;
        var_vbd_jct_dn9 = 0.0;
        var_vbd_jct_rv = 0.0;

        var_vbpsp = 0.0;
        var_vbpsp_dn7 = 0.0;
        var_vbpsp_dn8 = 0.0;
        var_vbpsp_rv = 0.0;

        var_vbpdp = 0.0;
        var_vbpdp_dn5 = 0.0;
        var_vbpdp_dn8 = 0.0;
        var_vbpdp_rv = 0.0;

        var_vbsi_jct = 0.0;
        var_vbsi_jct_dn7 = 0.0;
        var_vbsi_jct_dn8 = 0.0;
        var_vbsi_jct_rv = 0.0;

        var_vbdi_jct = 0.0;
        var_vbdi_jct_dn5 = 0.0;
        var_vbdi_jct_dn8 = 0.0;
        var_vbdi_jct_rv = 0.0;

        var_exptempd = 0.0;
        var_exptempd_dn0 = 0.0;
        var_exptempd_dn2 = 0.0;
        var_exptempd_dn4 = 0.0;
        var_exptempd_dn5 = 0.0;
        var_exptempd_dn6 = 0.0;
        var_exptempd_dn7 = 0.0;
        var_exptempd_dn8 = 0.0;
        var_exptempd_dn9 = 0.0;
        var_exptempd_dn10 = 0.0;
        var_exptempd_dn13 = 0.0;
        var_exptempd_rv = 0.0;

        var_exptemps = 0.0;
        var_exptemps_dn0 = 0.0;
        var_exptemps_dn2 = 0.0;
        var_exptemps_dn4 = 0.0;
        var_exptemps_dn5 = 0.0;
        var_exptemps_dn6 = 0.0;
        var_exptemps_dn7 = 0.0;
        var_exptemps_dn8 = 0.0;
        var_exptemps_dn9 = 0.0;
        var_exptemps_dn10 = 0.0;
        var_exptemps_dn13 = 0.0;
        var_exptemps_rv = 0.0;

        var_isbd = 0.0;
        var_isbd_dn0 = 0.0;
        var_isbd_dn2 = 0.0;
        var_isbd_dn4 = 0.0;
        var_isbd_dn5 = 0.0;
        var_isbd_dn6 = 0.0;
        var_isbd_dn7 = 0.0;
        var_isbd_dn8 = 0.0;
        var_isbd_dn9 = 0.0;
        var_isbd_dn10 = 0.0;
        var_isbd_dn13 = 0.0;
        var_isbd_rv = 0.0;

        var_isbs = 0.0;
        var_isbs_dn0 = 0.0;
        var_isbs_dn2 = 0.0;
        var_isbs_dn4 = 0.0;
        var_isbs_dn5 = 0.0;
        var_isbs_dn6 = 0.0;
        var_isbs_dn7 = 0.0;
        var_isbs_dn8 = 0.0;
        var_isbs_dn9 = 0.0;
        var_isbs_dn10 = 0.0;
        var_isbs_dn13 = 0.0;
        var_isbs_rv = 0.0;

        var_jd_expcd = 0.0;
        var_jd_expcd_dn0 = 0.0;
        var_jd_expcd_dn2 = 0.0;
        var_jd_expcd_dn4 = 0.0;
        var_jd_expcd_dn5 = 0.0;
        var_jd_expcd_dn6 = 0.0;
        var_jd_expcd_dn7 = 0.0;
        var_jd_expcd_dn8 = 0.0;
        var_jd_expcd_dn9 = 0.0;
        var_jd_expcd_dn10 = 0.0;
        var_jd_expcd_dn13 = 0.0;
        var_jd_expcd_rv = 0.0;

        var_jd_expcs = 0.0;
        var_jd_expcs_dn0 = 0.0;
        var_jd_expcs_dn2 = 0.0;
        var_jd_expcs_dn4 = 0.0;
        var_jd_expcs_dn5 = 0.0;
        var_jd_expcs_dn6 = 0.0;
        var_jd_expcs_dn7 = 0.0;
        var_jd_expcs_dn8 = 0.0;
        var_jd_expcs_dn9 = 0.0;
        var_jd_expcs_dn10 = 0.0;
        var_jd_expcs_dn13 = 0.0;
        var_jd_expcs_rv = 0.0;

        var_vbdt = 0.0;
        var_vbdt_dn0 = 0.0;
        var_vbdt_dn2 = 0.0;
        var_vbdt_dn4 = 0.0;
        var_vbdt_dn5 = 0.0;
        var_vbdt_dn6 = 0.0;
        var_vbdt_dn7 = 0.0;
        var_vbdt_dn8 = 0.0;
        var_vbdt_dn9 = 0.0;
        var_vbdt_dn10 = 0.0;
        var_vbdt_dn13 = 0.0;
        var_vbdt_rv = 0.0;

        var_vbst = 0.0;
        var_vbst_dn0 = 0.0;
        var_vbst_dn2 = 0.0;
        var_vbst_dn4 = 0.0;
        var_vbst_dn5 = 0.0;
        var_vbst_dn6 = 0.0;
        var_vbst_dn7 = 0.0;
        var_vbst_dn8 = 0.0;
        var_vbst_dn9 = 0.0;
        var_vbst_dn10 = 0.0;
        var_vbst_dn13 = 0.0;
        var_vbst_rv = 0.0;

        var_jd_nvtm_invd = 0.0;
        var_jd_nvtm_invd_dn0 = 0.0;
        var_jd_nvtm_invd_dn2 = 0.0;
        var_jd_nvtm_invd_dn4 = 0.0;
        var_jd_nvtm_invd_dn5 = 0.0;
        var_jd_nvtm_invd_dn6 = 0.0;
        var_jd_nvtm_invd_dn7 = 0.0;
        var_jd_nvtm_invd_dn8 = 0.0;
        var_jd_nvtm_invd_dn9 = 0.0;
        var_jd_nvtm_invd_dn10 = 0.0;
        var_jd_nvtm_invd_dn13 = 0.0;
        var_jd_nvtm_invd_rv = 0.0;

        var_jd_nvtm_invs = 0.0;
        var_jd_nvtm_invs_dn0 = 0.0;
        var_jd_nvtm_invs_dn2 = 0.0;
        var_jd_nvtm_invs_dn4 = 0.0;
        var_jd_nvtm_invs_dn5 = 0.0;
        var_jd_nvtm_invs_dn6 = 0.0;
        var_jd_nvtm_invs_dn7 = 0.0;
        var_jd_nvtm_invs_dn8 = 0.0;
        var_jd_nvtm_invs_dn9 = 0.0;
        var_jd_nvtm_invs_dn10 = 0.0;
        var_jd_nvtm_invs_dn13 = 0.0;
        var_jd_nvtm_invs_rv = 0.0;

        var_end_of_part_1 = 0.0;
        var_end_of_part_1_rv = 0.0;

        var_flg_brk1 = 0.0;
        var_flg_brk1_rv = 0.0;

        var_start_of_loopl = 0.0;
        var_start_of_loopl_rv = 0.0;

        var_flg_brk2 = 0.0;
        var_flg_brk2_rv = 0.0;

        var_start_of_mobility = 0.0;
        var_start_of_mobility_rv = 0.0;

        var_qbd_qs = 0.0;
        var_qbd_qs_dn0 = 0.0;
        var_qbd_qs_dn2 = 0.0;
        var_qbd_qs_dn4 = 0.0;
        var_qbd_qs_dn5 = 0.0;
        var_qbd_qs_dn6 = 0.0;
        var_qbd_qs_dn7 = 0.0;
        var_qbd_qs_dn8 = 0.0;
        var_qbd_qs_dn9 = 0.0;
        var_qbd_qs_dn10 = 0.0;
        var_qbd_qs_dn13 = 0.0;
        var_qbd_qs_rv = 0.0;

        var_isbd_btm = 0.0;
        var_isbd_btm_dn0 = 0.0;
        var_isbd_btm_dn2 = 0.0;
        var_isbd_btm_dn4 = 0.0;
        var_isbd_btm_dn5 = 0.0;
        var_isbd_btm_dn6 = 0.0;
        var_isbd_btm_dn7 = 0.0;
        var_isbd_btm_dn8 = 0.0;
        var_isbd_btm_dn9 = 0.0;
        var_isbd_btm_dn10 = 0.0;
        var_isbd_btm_dn13 = 0.0;
        var_isbd_btm_rv = 0.0;

        var_isbd2_btm = 0.0;
        var_isbd2_btm_dn0 = 0.0;
        var_isbd2_btm_dn2 = 0.0;
        var_isbd2_btm_dn4 = 0.0;
        var_isbd2_btm_dn5 = 0.0;
        var_isbd2_btm_dn6 = 0.0;
        var_isbd2_btm_dn7 = 0.0;
        var_isbd2_btm_dn8 = 0.0;
        var_isbd2_btm_dn9 = 0.0;
        var_isbd2_btm_dn10 = 0.0;
        var_isbd2_btm_dn13 = 0.0;
        var_isbd2_btm_rv = 0.0;

        var_isbd_sws = 0.0;
        var_isbd_sws_dn0 = 0.0;
        var_isbd_sws_dn2 = 0.0;
        var_isbd_sws_dn4 = 0.0;
        var_isbd_sws_dn5 = 0.0;
        var_isbd_sws_dn6 = 0.0;
        var_isbd_sws_dn7 = 0.0;
        var_isbd_sws_dn8 = 0.0;
        var_isbd_sws_dn9 = 0.0;
        var_isbd_sws_dn10 = 0.0;
        var_isbd_sws_dn13 = 0.0;
        var_isbd_sws_rv = 0.0;

        var_isbd2_sws = 0.0;
        var_isbd2_sws_dn0 = 0.0;
        var_isbd2_sws_dn2 = 0.0;
        var_isbd2_sws_dn4 = 0.0;
        var_isbd2_sws_dn5 = 0.0;
        var_isbd2_sws_dn6 = 0.0;
        var_isbd2_sws_dn7 = 0.0;
        var_isbd2_sws_dn8 = 0.0;
        var_isbd2_sws_dn9 = 0.0;
        var_isbd2_sws_dn10 = 0.0;
        var_isbd2_sws_dn13 = 0.0;
        var_isbd2_sws_rv = 0.0;

        var_isbd_swg = 0.0;
        var_isbd_swg_dn0 = 0.0;
        var_isbd_swg_dn2 = 0.0;
        var_isbd_swg_dn4 = 0.0;
        var_isbd_swg_dn5 = 0.0;
        var_isbd_swg_dn6 = 0.0;
        var_isbd_swg_dn7 = 0.0;
        var_isbd_swg_dn8 = 0.0;
        var_isbd_swg_dn9 = 0.0;
        var_isbd_swg_dn10 = 0.0;
        var_isbd_swg_dn13 = 0.0;
        var_isbd_swg_rv = 0.0;

        var_isbd2_swg = 0.0;
        var_isbd2_swg_dn0 = 0.0;
        var_isbd2_swg_dn2 = 0.0;
        var_isbd2_swg_dn4 = 0.0;
        var_isbd2_swg_dn5 = 0.0;
        var_isbd2_swg_dn6 = 0.0;
        var_isbd2_swg_dn7 = 0.0;
        var_isbd2_swg_dn8 = 0.0;
        var_isbd2_swg_dn9 = 0.0;
        var_isbd2_swg_dn10 = 0.0;
        var_isbd2_swg_dn13 = 0.0;
        var_isbd2_swg_rv = 0.0;

        var_isbs_btm = 0.0;
        var_isbs_btm_dn0 = 0.0;
        var_isbs_btm_dn2 = 0.0;
        var_isbs_btm_dn4 = 0.0;
        var_isbs_btm_dn5 = 0.0;
        var_isbs_btm_dn6 = 0.0;
        var_isbs_btm_dn7 = 0.0;
        var_isbs_btm_dn8 = 0.0;
        var_isbs_btm_dn9 = 0.0;
        var_isbs_btm_dn10 = 0.0;
        var_isbs_btm_dn13 = 0.0;
        var_isbs_btm_rv = 0.0;

        *var_czbd_slot = var_czbd;
        *var_czbd_dn0_slot = var_czbd_dn0;
        *var_czbd_dn10_slot = var_czbd_dn10;
        *var_czbd_dn13_slot = var_czbd_dn13;
        *var_czbd_dn2_slot = var_czbd_dn2;
        *var_czbd_dn4_slot = var_czbd_dn4;
        *var_czbd_dn5_slot = var_czbd_dn5;
        *var_czbd_dn6_slot = var_czbd_dn6;
        *var_czbd_dn7_slot = var_czbd_dn7;
        *var_czbd_dn8_slot = var_czbd_dn8;
        *var_czbd_dn9_slot = var_czbd_dn9;
        *var_czbd_rv_slot = var_czbd_rv;
        *var_czbdsw_slot = var_czbdsw;
        *var_czbdsw_dn0_slot = var_czbdsw_dn0;
        *var_czbdsw_dn10_slot = var_czbdsw_dn10;
        *var_czbdsw_dn13_slot = var_czbdsw_dn13;
        *var_czbdsw_dn2_slot = var_czbdsw_dn2;
        *var_czbdsw_dn4_slot = var_czbdsw_dn4;
        *var_czbdsw_dn5_slot = var_czbdsw_dn5;
        *var_czbdsw_dn6_slot = var_czbdsw_dn6;
        *var_czbdsw_dn7_slot = var_czbdsw_dn7;
        *var_czbdsw_dn8_slot = var_czbdsw_dn8;
        *var_czbdsw_dn9_slot = var_czbdsw_dn9;
        *var_czbdsw_rv_slot = var_czbdsw_rv;
        *var_czbdswg_slot = var_czbdswg;
        *var_czbdswg_dn0_slot = var_czbdswg_dn0;
        *var_czbdswg_dn10_slot = var_czbdswg_dn10;
        *var_czbdswg_dn13_slot = var_czbdswg_dn13;
        *var_czbdswg_dn2_slot = var_czbdswg_dn2;
        *var_czbdswg_dn4_slot = var_czbdswg_dn4;
        *var_czbdswg_dn5_slot = var_czbdswg_dn5;
        *var_czbdswg_dn6_slot = var_czbdswg_dn6;
        *var_czbdswg_dn7_slot = var_czbdswg_dn7;
        *var_czbdswg_dn8_slot = var_czbdswg_dn8;
        *var_czbdswg_dn9_slot = var_czbdswg_dn9;
        *var_czbdswg_rv_slot = var_czbdswg_rv;
        *var_czbs_slot = var_czbs;
        *var_czbs_dn0_slot = var_czbs_dn0;
        *var_czbs_dn10_slot = var_czbs_dn10;
        *var_czbs_dn13_slot = var_czbs_dn13;
        *var_czbs_dn2_slot = var_czbs_dn2;
        *var_czbs_dn4_slot = var_czbs_dn4;
        *var_czbs_dn5_slot = var_czbs_dn5;
        *var_czbs_dn6_slot = var_czbs_dn6;
        *var_czbs_dn7_slot = var_czbs_dn7;
        *var_czbs_dn8_slot = var_czbs_dn8;
        *var_czbs_dn9_slot = var_czbs_dn9;
        *var_czbs_rv_slot = var_czbs_rv;
        *var_czbssw_slot = var_czbssw;
        *var_czbssw_dn0_slot = var_czbssw_dn0;
        *var_czbssw_dn10_slot = var_czbssw_dn10;
        *var_czbssw_dn13_slot = var_czbssw_dn13;
        *var_czbssw_dn2_slot = var_czbssw_dn2;
        *var_czbssw_dn4_slot = var_czbssw_dn4;
        *var_czbssw_dn5_slot = var_czbssw_dn5;
        *var_czbssw_dn6_slot = var_czbssw_dn6;
        *var_czbssw_dn7_slot = var_czbssw_dn7;
        *var_czbssw_dn8_slot = var_czbssw_dn8;
        *var_czbssw_dn9_slot = var_czbssw_dn9;
        *var_czbssw_rv_slot = var_czbssw_rv;
        *var_czbsswg_slot = var_czbsswg;
        *var_czbsswg_dn0_slot = var_czbsswg_dn0;
        *var_czbsswg_dn10_slot = var_czbsswg_dn10;
        *var_czbsswg_dn13_slot = var_czbsswg_dn13;
        *var_czbsswg_dn2_slot = var_czbsswg_dn2;
        *var_czbsswg_dn4_slot = var_czbsswg_dn4;
        *var_czbsswg_dn5_slot = var_czbsswg_dn5;
        *var_czbsswg_dn6_slot = var_czbsswg_dn6;
        *var_czbsswg_dn7_slot = var_czbsswg_dn7;
        *var_czbsswg_dn8_slot = var_czbsswg_dn8;
        *var_czbsswg_dn9_slot = var_czbsswg_dn9;
        *var_czbsswg_rv_slot = var_czbsswg_rv;
        *var_end_of_part_1_slot = var_end_of_part_1;
        *var_end_of_part_1_rv_slot = var_end_of_part_1_rv;
        *var_exptempd_slot = var_exptempd;
        *var_exptempd_dn0_slot = var_exptempd_dn0;
        *var_exptempd_dn10_slot = var_exptempd_dn10;
        *var_exptempd_dn13_slot = var_exptempd_dn13;
        *var_exptempd_dn2_slot = var_exptempd_dn2;
        *var_exptempd_dn4_slot = var_exptempd_dn4;
        *var_exptempd_dn5_slot = var_exptempd_dn5;
        *var_exptempd_dn6_slot = var_exptempd_dn6;
        *var_exptempd_dn7_slot = var_exptempd_dn7;
        *var_exptempd_dn8_slot = var_exptempd_dn8;
        *var_exptempd_dn9_slot = var_exptempd_dn9;
        *var_exptempd_rv_slot = var_exptempd_rv;
        *var_exptemps_slot = var_exptemps;
        *var_exptemps_dn0_slot = var_exptemps_dn0;
        *var_exptemps_dn10_slot = var_exptemps_dn10;
        *var_exptemps_dn13_slot = var_exptemps_dn13;
        *var_exptemps_dn2_slot = var_exptemps_dn2;
        *var_exptemps_dn4_slot = var_exptemps_dn4;
        *var_exptemps_dn5_slot = var_exptemps_dn5;
        *var_exptemps_dn6_slot = var_exptemps_dn6;
        *var_exptemps_dn7_slot = var_exptemps_dn7;
        *var_exptemps_dn8_slot = var_exptemps_dn8;
        *var_exptemps_dn9_slot = var_exptemps_dn9;
        *var_exptemps_rv_slot = var_exptemps_rv;
        *var_flg_brk1_slot = var_flg_brk1;
        *var_flg_brk1_rv_slot = var_flg_brk1_rv;
        *var_flg_brk2_slot = var_flg_brk2;
        *var_flg_brk2_rv_slot = var_flg_brk2_rv;
        *var_isbd_slot = var_isbd;
        *var_isbd2_btm_slot = var_isbd2_btm;
        *var_isbd2_btm_dn0_slot = var_isbd2_btm_dn0;
        *var_isbd2_btm_dn10_slot = var_isbd2_btm_dn10;
        *var_isbd2_btm_dn13_slot = var_isbd2_btm_dn13;
        *var_isbd2_btm_dn2_slot = var_isbd2_btm_dn2;
        *var_isbd2_btm_dn4_slot = var_isbd2_btm_dn4;
        *var_isbd2_btm_dn5_slot = var_isbd2_btm_dn5;
        *var_isbd2_btm_dn6_slot = var_isbd2_btm_dn6;
        *var_isbd2_btm_dn7_slot = var_isbd2_btm_dn7;
        *var_isbd2_btm_dn8_slot = var_isbd2_btm_dn8;
        *var_isbd2_btm_dn9_slot = var_isbd2_btm_dn9;
        *var_isbd2_btm_rv_slot = var_isbd2_btm_rv;
        *var_isbd2_swg_slot = var_isbd2_swg;
        *var_isbd2_swg_dn0_slot = var_isbd2_swg_dn0;
        *var_isbd2_swg_dn10_slot = var_isbd2_swg_dn10;
        *var_isbd2_swg_dn13_slot = var_isbd2_swg_dn13;
        *var_isbd2_swg_dn2_slot = var_isbd2_swg_dn2;
        *var_isbd2_swg_dn4_slot = var_isbd2_swg_dn4;
        *var_isbd2_swg_dn5_slot = var_isbd2_swg_dn5;
        *var_isbd2_swg_dn6_slot = var_isbd2_swg_dn6;
        *var_isbd2_swg_dn7_slot = var_isbd2_swg_dn7;
        *var_isbd2_swg_dn8_slot = var_isbd2_swg_dn8;
        *var_isbd2_swg_dn9_slot = var_isbd2_swg_dn9;
        *var_isbd2_swg_rv_slot = var_isbd2_swg_rv;
        *var_isbd2_sws_slot = var_isbd2_sws;
        *var_isbd2_sws_dn0_slot = var_isbd2_sws_dn0;
        *var_isbd2_sws_dn10_slot = var_isbd2_sws_dn10;
        *var_isbd2_sws_dn13_slot = var_isbd2_sws_dn13;
        *var_isbd2_sws_dn2_slot = var_isbd2_sws_dn2;
        *var_isbd2_sws_dn4_slot = var_isbd2_sws_dn4;
        *var_isbd2_sws_dn5_slot = var_isbd2_sws_dn5;
        *var_isbd2_sws_dn6_slot = var_isbd2_sws_dn6;
        *var_isbd2_sws_dn7_slot = var_isbd2_sws_dn7;
        *var_isbd2_sws_dn8_slot = var_isbd2_sws_dn8;
        *var_isbd2_sws_dn9_slot = var_isbd2_sws_dn9;
        *var_isbd2_sws_rv_slot = var_isbd2_sws_rv;
        *var_isbd_btm_slot = var_isbd_btm;
        *var_isbd_btm_dn0_slot = var_isbd_btm_dn0;
        *var_isbd_btm_dn10_slot = var_isbd_btm_dn10;
        *var_isbd_btm_dn13_slot = var_isbd_btm_dn13;
        *var_isbd_btm_dn2_slot = var_isbd_btm_dn2;
        *var_isbd_btm_dn4_slot = var_isbd_btm_dn4;
        *var_isbd_btm_dn5_slot = var_isbd_btm_dn5;
        *var_isbd_btm_dn6_slot = var_isbd_btm_dn6;
        *var_isbd_btm_dn7_slot = var_isbd_btm_dn7;
        *var_isbd_btm_dn8_slot = var_isbd_btm_dn8;
        *var_isbd_btm_dn9_slot = var_isbd_btm_dn9;
        *var_isbd_btm_rv_slot = var_isbd_btm_rv;
        *var_isbd_dn0_slot = var_isbd_dn0;
        *var_isbd_dn10_slot = var_isbd_dn10;
        *var_isbd_dn13_slot = var_isbd_dn13;
        *var_isbd_dn2_slot = var_isbd_dn2;
        *var_isbd_dn4_slot = var_isbd_dn4;
        *var_isbd_dn5_slot = var_isbd_dn5;
        *var_isbd_dn6_slot = var_isbd_dn6;
        *var_isbd_dn7_slot = var_isbd_dn7;
        *var_isbd_dn8_slot = var_isbd_dn8;
        *var_isbd_dn9_slot = var_isbd_dn9;
        *var_isbd_rv_slot = var_isbd_rv;
        *var_isbd_swg_slot = var_isbd_swg;
        *var_isbd_swg_dn0_slot = var_isbd_swg_dn0;
        *var_isbd_swg_dn10_slot = var_isbd_swg_dn10;
        *var_isbd_swg_dn13_slot = var_isbd_swg_dn13;
        *var_isbd_swg_dn2_slot = var_isbd_swg_dn2;
        *var_isbd_swg_dn4_slot = var_isbd_swg_dn4;
        *var_isbd_swg_dn5_slot = var_isbd_swg_dn5;
        *var_isbd_swg_dn6_slot = var_isbd_swg_dn6;
        *var_isbd_swg_dn7_slot = var_isbd_swg_dn7;
        *var_isbd_swg_dn8_slot = var_isbd_swg_dn8;
        *var_isbd_swg_dn9_slot = var_isbd_swg_dn9;
        *var_isbd_swg_rv_slot = var_isbd_swg_rv;
        *var_isbd_sws_slot = var_isbd_sws;
        *var_isbd_sws_dn0_slot = var_isbd_sws_dn0;
        *var_isbd_sws_dn10_slot = var_isbd_sws_dn10;
        *var_isbd_sws_dn13_slot = var_isbd_sws_dn13;
        *var_isbd_sws_dn2_slot = var_isbd_sws_dn2;
        *var_isbd_sws_dn4_slot = var_isbd_sws_dn4;
        *var_isbd_sws_dn5_slot = var_isbd_sws_dn5;
        *var_isbd_sws_dn6_slot = var_isbd_sws_dn6;
        *var_isbd_sws_dn7_slot = var_isbd_sws_dn7;
        *var_isbd_sws_dn8_slot = var_isbd_sws_dn8;
        *var_isbd_sws_dn9_slot = var_isbd_sws_dn9;
        *var_isbd_sws_rv_slot = var_isbd_sws_rv;
        *var_isbs_slot = var_isbs;
        *var_isbs_btm_slot = var_isbs_btm;
        *var_isbs_btm_dn0_slot = var_isbs_btm_dn0;
        *var_isbs_btm_dn10_slot = var_isbs_btm_dn10;
        *var_isbs_btm_dn13_slot = var_isbs_btm_dn13;
        *var_isbs_btm_dn2_slot = var_isbs_btm_dn2;
        *var_isbs_btm_dn4_slot = var_isbs_btm_dn4;
        *var_isbs_btm_dn5_slot = var_isbs_btm_dn5;
        *var_isbs_btm_dn6_slot = var_isbs_btm_dn6;
        *var_isbs_btm_dn7_slot = var_isbs_btm_dn7;
        *var_isbs_btm_dn8_slot = var_isbs_btm_dn8;
        *var_isbs_btm_dn9_slot = var_isbs_btm_dn9;
        *var_isbs_btm_rv_slot = var_isbs_btm_rv;
        *var_isbs_dn0_slot = var_isbs_dn0;
        *var_isbs_dn10_slot = var_isbs_dn10;
        *var_isbs_dn13_slot = var_isbs_dn13;
        *var_isbs_dn2_slot = var_isbs_dn2;
        *var_isbs_dn4_slot = var_isbs_dn4;
        *var_isbs_dn5_slot = var_isbs_dn5;
        *var_isbs_dn6_slot = var_isbs_dn6;
        *var_isbs_dn7_slot = var_isbs_dn7;
        *var_isbs_dn8_slot = var_isbs_dn8;
        *var_isbs_dn9_slot = var_isbs_dn9;
        *var_isbs_rv_slot = var_isbs_rv;
        *var_jd_expcd_slot = var_jd_expcd;
        *var_jd_expcd_dn0_slot = var_jd_expcd_dn0;
        *var_jd_expcd_dn10_slot = var_jd_expcd_dn10;
        *var_jd_expcd_dn13_slot = var_jd_expcd_dn13;
        *var_jd_expcd_dn2_slot = var_jd_expcd_dn2;
        *var_jd_expcd_dn4_slot = var_jd_expcd_dn4;
        *var_jd_expcd_dn5_slot = var_jd_expcd_dn5;
        *var_jd_expcd_dn6_slot = var_jd_expcd_dn6;
        *var_jd_expcd_dn7_slot = var_jd_expcd_dn7;
        *var_jd_expcd_dn8_slot = var_jd_expcd_dn8;
        *var_jd_expcd_dn9_slot = var_jd_expcd_dn9;
        *var_jd_expcd_rv_slot = var_jd_expcd_rv;
        *var_jd_expcs_slot = var_jd_expcs;
        *var_jd_expcs_dn0_slot = var_jd_expcs_dn0;
        *var_jd_expcs_dn10_slot = var_jd_expcs_dn10;
        *var_jd_expcs_dn13_slot = var_jd_expcs_dn13;
        *var_jd_expcs_dn2_slot = var_jd_expcs_dn2;
        *var_jd_expcs_dn4_slot = var_jd_expcs_dn4;
        *var_jd_expcs_dn5_slot = var_jd_expcs_dn5;
        *var_jd_expcs_dn6_slot = var_jd_expcs_dn6;
        *var_jd_expcs_dn7_slot = var_jd_expcs_dn7;
        *var_jd_expcs_dn8_slot = var_jd_expcs_dn8;
        *var_jd_expcs_dn9_slot = var_jd_expcs_dn9;
        *var_jd_expcs_rv_slot = var_jd_expcs_rv;
        *var_jd_nvtm_invd_slot = var_jd_nvtm_invd;
        *var_jd_nvtm_invd_dn0_slot = var_jd_nvtm_invd_dn0;
        *var_jd_nvtm_invd_dn10_slot = var_jd_nvtm_invd_dn10;
        *var_jd_nvtm_invd_dn13_slot = var_jd_nvtm_invd_dn13;
        *var_jd_nvtm_invd_dn2_slot = var_jd_nvtm_invd_dn2;
        *var_jd_nvtm_invd_dn4_slot = var_jd_nvtm_invd_dn4;
        *var_jd_nvtm_invd_dn5_slot = var_jd_nvtm_invd_dn5;
        *var_jd_nvtm_invd_dn6_slot = var_jd_nvtm_invd_dn6;
        *var_jd_nvtm_invd_dn7_slot = var_jd_nvtm_invd_dn7;
        *var_jd_nvtm_invd_dn8_slot = var_jd_nvtm_invd_dn8;
        *var_jd_nvtm_invd_dn9_slot = var_jd_nvtm_invd_dn9;
        *var_jd_nvtm_invd_rv_slot = var_jd_nvtm_invd_rv;
        *var_jd_nvtm_invs_slot = var_jd_nvtm_invs;
        *var_jd_nvtm_invs_dn0_slot = var_jd_nvtm_invs_dn0;
        *var_jd_nvtm_invs_dn10_slot = var_jd_nvtm_invs_dn10;
        *var_jd_nvtm_invs_dn13_slot = var_jd_nvtm_invs_dn13;
        *var_jd_nvtm_invs_dn2_slot = var_jd_nvtm_invs_dn2;
        *var_jd_nvtm_invs_dn4_slot = var_jd_nvtm_invs_dn4;
        *var_jd_nvtm_invs_dn5_slot = var_jd_nvtm_invs_dn5;
        *var_jd_nvtm_invs_dn6_slot = var_jd_nvtm_invs_dn6;
        *var_jd_nvtm_invs_dn7_slot = var_jd_nvtm_invs_dn7;
        *var_jd_nvtm_invs_dn8_slot = var_jd_nvtm_invs_dn8;
        *var_jd_nvtm_invs_dn9_slot = var_jd_nvtm_invs_dn9;
        *var_jd_nvtm_invs_rv_slot = var_jd_nvtm_invs_rv;
        *var_pzbd_slot = var_pzbd;
        *var_pzbd_dn0_slot = var_pzbd_dn0;
        *var_pzbd_dn10_slot = var_pzbd_dn10;
        *var_pzbd_dn13_slot = var_pzbd_dn13;
        *var_pzbd_dn2_slot = var_pzbd_dn2;
        *var_pzbd_dn4_slot = var_pzbd_dn4;
        *var_pzbd_dn5_slot = var_pzbd_dn5;
        *var_pzbd_dn6_slot = var_pzbd_dn6;
        *var_pzbd_dn7_slot = var_pzbd_dn7;
        *var_pzbd_dn8_slot = var_pzbd_dn8;
        *var_pzbd_dn9_slot = var_pzbd_dn9;
        *var_pzbd_rv_slot = var_pzbd_rv;
        *var_pzbdsw_slot = var_pzbdsw;
        *var_pzbdsw_dn0_slot = var_pzbdsw_dn0;
        *var_pzbdsw_dn10_slot = var_pzbdsw_dn10;
        *var_pzbdsw_dn13_slot = var_pzbdsw_dn13;
        *var_pzbdsw_dn2_slot = var_pzbdsw_dn2;
        *var_pzbdsw_dn4_slot = var_pzbdsw_dn4;
        *var_pzbdsw_dn5_slot = var_pzbdsw_dn5;
        *var_pzbdsw_dn6_slot = var_pzbdsw_dn6;
        *var_pzbdsw_dn7_slot = var_pzbdsw_dn7;
        *var_pzbdsw_dn8_slot = var_pzbdsw_dn8;
        *var_pzbdsw_dn9_slot = var_pzbdsw_dn9;
        *var_pzbdsw_rv_slot = var_pzbdsw_rv;
        *var_pzbdswg_slot = var_pzbdswg;
        *var_pzbdswg_dn0_slot = var_pzbdswg_dn0;
        *var_pzbdswg_dn10_slot = var_pzbdswg_dn10;
        *var_pzbdswg_dn13_slot = var_pzbdswg_dn13;
        *var_pzbdswg_dn2_slot = var_pzbdswg_dn2;
        *var_pzbdswg_dn4_slot = var_pzbdswg_dn4;
        *var_pzbdswg_dn5_slot = var_pzbdswg_dn5;
        *var_pzbdswg_dn6_slot = var_pzbdswg_dn6;
        *var_pzbdswg_dn7_slot = var_pzbdswg_dn7;
        *var_pzbdswg_dn8_slot = var_pzbdswg_dn8;
        *var_pzbdswg_dn9_slot = var_pzbdswg_dn9;
        *var_pzbdswg_rv_slot = var_pzbdswg_rv;
        *var_pzbs_slot = var_pzbs;
        *var_pzbs_dn0_slot = var_pzbs_dn0;
        *var_pzbs_dn10_slot = var_pzbs_dn10;
        *var_pzbs_dn13_slot = var_pzbs_dn13;
        *var_pzbs_dn2_slot = var_pzbs_dn2;
        *var_pzbs_dn4_slot = var_pzbs_dn4;
        *var_pzbs_dn5_slot = var_pzbs_dn5;
        *var_pzbs_dn6_slot = var_pzbs_dn6;
        *var_pzbs_dn7_slot = var_pzbs_dn7;
        *var_pzbs_dn8_slot = var_pzbs_dn8;
        *var_pzbs_dn9_slot = var_pzbs_dn9;
        *var_pzbs_rv_slot = var_pzbs_rv;
        *var_pzbssw_slot = var_pzbssw;
        *var_pzbssw_dn0_slot = var_pzbssw_dn0;
        *var_pzbssw_dn10_slot = var_pzbssw_dn10;
        *var_pzbssw_dn13_slot = var_pzbssw_dn13;
        *var_pzbssw_dn2_slot = var_pzbssw_dn2;
        *var_pzbssw_dn4_slot = var_pzbssw_dn4;
        *var_pzbssw_dn5_slot = var_pzbssw_dn5;
        *var_pzbssw_dn6_slot = var_pzbssw_dn6;
        *var_pzbssw_dn7_slot = var_pzbssw_dn7;
        *var_pzbssw_dn8_slot = var_pzbssw_dn8;
        *var_pzbssw_dn9_slot = var_pzbssw_dn9;
        *var_pzbssw_rv_slot = var_pzbssw_rv;
        *var_pzbsswg_slot = var_pzbsswg;
        *var_pzbsswg_dn0_slot = var_pzbsswg_dn0;
        *var_pzbsswg_dn10_slot = var_pzbsswg_dn10;
        *var_pzbsswg_dn13_slot = var_pzbsswg_dn13;
        *var_pzbsswg_dn2_slot = var_pzbsswg_dn2;
        *var_pzbsswg_dn4_slot = var_pzbsswg_dn4;
        *var_pzbsswg_dn5_slot = var_pzbsswg_dn5;
        *var_pzbsswg_dn6_slot = var_pzbsswg_dn6;
        *var_pzbsswg_dn7_slot = var_pzbsswg_dn7;
        *var_pzbsswg_dn8_slot = var_pzbsswg_dn8;
        *var_pzbsswg_dn9_slot = var_pzbsswg_dn9;
        *var_pzbsswg_rv_slot = var_pzbsswg_rv;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn0_slot = var_qbd_dn0;
        *var_qbd_dn10_slot = var_qbd_dn10;
        *var_qbd_dn13_slot = var_qbd_dn13;
        *var_qbd_dn15_slot = var_qbd_dn15;
        *var_qbd_dn16_slot = var_qbd_dn16;
        *var_qbd_dn17_slot = var_qbd_dn17;
        *var_qbd_dn2_slot = var_qbd_dn2;
        *var_qbd_dn4_slot = var_qbd_dn4;
        *var_qbd_dn5_slot = var_qbd_dn5;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbd_dn8_slot = var_qbd_dn8;
        *var_qbd_dn9_slot = var_qbd_dn9;
        *var_qbd_qs_slot = var_qbd_qs;
        *var_qbd_qs_dn0_slot = var_qbd_qs_dn0;
        *var_qbd_qs_dn10_slot = var_qbd_qs_dn10;
        *var_qbd_qs_dn13_slot = var_qbd_qs_dn13;
        *var_qbd_qs_dn2_slot = var_qbd_qs_dn2;
        *var_qbd_qs_dn4_slot = var_qbd_qs_dn4;
        *var_qbd_qs_dn5_slot = var_qbd_qs_dn5;
        *var_qbd_qs_dn6_slot = var_qbd_qs_dn6;
        *var_qbd_qs_dn7_slot = var_qbd_qs_dn7;
        *var_qbd_qs_dn8_slot = var_qbd_qs_dn8;
        *var_qbd_qs_dn9_slot = var_qbd_qs_dn9;
        *var_qbd_qs_rv_slot = var_qbd_qs_rv;
        *var_qbd_rv_slot = var_qbd_rv;
        *var_qbdi_slot = var_qbdi;
        *var_qbdi_dn0_slot = var_qbdi_dn0;
        *var_qbdi_dn10_slot = var_qbdi_dn10;
        *var_qbdi_dn13_slot = var_qbdi_dn13;
        *var_qbdi_dn2_slot = var_qbdi_dn2;
        *var_qbdi_dn4_slot = var_qbdi_dn4;
        *var_qbdi_dn5_slot = var_qbdi_dn5;
        *var_qbdi_dn6_slot = var_qbdi_dn6;
        *var_qbdi_dn7_slot = var_qbdi_dn7;
        *var_qbdi_dn8_slot = var_qbdi_dn8;
        *var_qbdi_dn9_slot = var_qbdi_dn9;
        *var_qbdi_rv_slot = var_qbdi_rv;
        *var_qbsi_slot = var_qbsi;
        *var_qbsi_dn0_slot = var_qbsi_dn0;
        *var_qbsi_dn10_slot = var_qbsi_dn10;
        *var_qbsi_dn13_slot = var_qbsi_dn13;
        *var_qbsi_dn2_slot = var_qbsi_dn2;
        *var_qbsi_dn4_slot = var_qbsi_dn4;
        *var_qbsi_dn5_slot = var_qbsi_dn5;
        *var_qbsi_dn6_slot = var_qbsi_dn6;
        *var_qbsi_dn7_slot = var_qbsi_dn7;
        *var_qbsi_dn8_slot = var_qbsi_dn8;
        *var_qbsi_dn9_slot = var_qbsi_dn9;
        *var_qbsi_rv_slot = var_qbsi_rv;
        *var_sarg_slot = var_sarg;
        *var_sarg_dn0_slot = var_sarg_dn0;
        *var_sarg_dn10_slot = var_sarg_dn10;
        *var_sarg_dn13_slot = var_sarg_dn13;
        *var_sarg_dn2_slot = var_sarg_dn2;
        *var_sarg_dn4_slot = var_sarg_dn4;
        *var_sarg_dn5_slot = var_sarg_dn5;
        *var_sarg_dn6_slot = var_sarg_dn6;
        *var_sarg_dn7_slot = var_sarg_dn7;
        *var_sarg_dn8_slot = var_sarg_dn8;
        *var_sarg_dn9_slot = var_sarg_dn9;
        *var_sarg_rv_slot = var_sarg_rv;
        *var_start_of_loopl_slot = var_start_of_loopl;
        *var_start_of_loopl_rv_slot = var_start_of_loopl_rv;
        *var_start_of_mobility_slot = var_start_of_mobility;
        *var_start_of_mobility_rv_slot = var_start_of_mobility_rv;
        *var_vbd_jct_slot = var_vbd_jct;
        *var_vbd_jct_dn0_slot = var_vbd_jct_dn0;
        *var_vbd_jct_dn9_slot = var_vbd_jct_dn9;
        *var_vbd_jct_rv_slot = var_vbd_jct_rv;
        *var_vbdi_jct_slot = var_vbdi_jct;
        *var_vbdi_jct_dn5_slot = var_vbdi_jct_dn5;
        *var_vbdi_jct_dn8_slot = var_vbdi_jct_dn8;
        *var_vbdi_jct_rv_slot = var_vbdi_jct_rv;
        *var_vbdt_slot = var_vbdt;
        *var_vbdt_dn0_slot = var_vbdt_dn0;
        *var_vbdt_dn10_slot = var_vbdt_dn10;
        *var_vbdt_dn13_slot = var_vbdt_dn13;
        *var_vbdt_dn2_slot = var_vbdt_dn2;
        *var_vbdt_dn4_slot = var_vbdt_dn4;
        *var_vbdt_dn5_slot = var_vbdt_dn5;
        *var_vbdt_dn6_slot = var_vbdt_dn6;
        *var_vbdt_dn7_slot = var_vbdt_dn7;
        *var_vbdt_dn8_slot = var_vbdt_dn8;
        *var_vbdt_dn9_slot = var_vbdt_dn9;
        *var_vbdt_rv_slot = var_vbdt_rv;
        *var_vbpdp_slot = var_vbpdp;
        *var_vbpdp_dn5_slot = var_vbpdp_dn5;
        *var_vbpdp_dn8_slot = var_vbpdp_dn8;
        *var_vbpdp_rv_slot = var_vbpdp_rv;
        *var_vbpsp_slot = var_vbpsp;
        *var_vbpsp_dn7_slot = var_vbpsp_dn7;
        *var_vbpsp_dn8_slot = var_vbpsp_dn8;
        *var_vbpsp_rv_slot = var_vbpsp_rv;
        *var_vbs_jct_slot = var_vbs_jct;
        *var_vbs_jct_dn10_slot = var_vbs_jct_dn10;
        *var_vbs_jct_dn2_slot = var_vbs_jct_dn2;
        *var_vbs_jct_rv_slot = var_vbs_jct_rv;
        *var_vbsi_jct_slot = var_vbsi_jct;
        *var_vbsi_jct_dn7_slot = var_vbsi_jct_dn7;
        *var_vbsi_jct_dn8_slot = var_vbsi_jct_dn8;
        *var_vbsi_jct_rv_slot = var_vbsi_jct_rv;
        *var_vbst_slot = var_vbst;
        *var_vbst_dn0_slot = var_vbst_dn0;
        *var_vbst_dn10_slot = var_vbst_dn10;
        *var_vbst_dn13_slot = var_vbst_dn13;
        *var_vbst_dn2_slot = var_vbst_dn2;
        *var_vbst_dn4_slot = var_vbst_dn4;
        *var_vbst_dn5_slot = var_vbst_dn5;
        *var_vbst_dn6_slot = var_vbst_dn6;
        *var_vbst_dn7_slot = var_vbst_dn7;
        *var_vbst_dn8_slot = var_vbst_dn8;
        *var_vbst_dn9_slot = var_vbst_dn9;
        *var_vbst_rv_slot = var_vbst_rv;
        *var_vdbd_slot = var_vdbd;
        *var_vdbd_dn0_slot = var_vdbd_dn0;
        *var_vdbd_dn9_slot = var_vdbd_dn9;
        *var_vdbd_rv_slot = var_vdbd_rv;
        *var_vsbs_slot = var_vsbs;
        *var_vsbs_dn10_slot = var_vsbs_dn10;
        *var_vsbs_dn2_slot = var_vsbs_dn2;
        *var_vsbs_rv_slot = var_vsbs_rv;
    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        var_cox0_func_slot: &mut f64,
        var_cox0_func_rv_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard11_rv_slot: &mut f64,
        var_guard14_slot: &mut f64,
        var_guard14_rv_slot: &mut f64,
        var_guard15_slot: &mut f64,
        var_guard15_rv_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard8_rv_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_guard9_rv_slot: &mut f64,
        var_idspt0_slot: &mut f64,
        var_idspt0_dn0_slot: &mut f64,
        var_idspt0_dn10_slot: &mut f64,
        var_idspt0_dn13_slot: &mut f64,
        var_idspt0_dn2_slot: &mut f64,
        var_idspt0_dn4_slot: &mut f64,
        var_idspt0_dn5_slot: &mut f64,
        var_idspt0_dn6_slot: &mut f64,
        var_idspt0_dn7_slot: &mut f64,
        var_idspt0_dn8_slot: &mut f64,
        var_idspt0_dn9_slot: &mut f64,
        var_idspt0_rv_slot: &mut f64,
        var_idspt1_slot: &mut f64,
        var_idspt1_dn0_slot: &mut f64,
        var_idspt1_dn10_slot: &mut f64,
        var_idspt1_dn13_slot: &mut f64,
        var_idspt1_dn2_slot: &mut f64,
        var_idspt1_dn4_slot: &mut f64,
        var_idspt1_dn5_slot: &mut f64,
        var_idspt1_dn6_slot: &mut f64,
        var_idspt1_dn7_slot: &mut f64,
        var_idspt1_dn8_slot: &mut f64,
        var_idspt1_dn9_slot: &mut f64,
        var_idspt1_rv_slot: &mut f64,
        var_inqs0_a_slot: &mut f64,
        var_inqs0_a_dn0_slot: &mut f64,
        var_inqs0_a_dn10_slot: &mut f64,
        var_inqs0_a_dn13_slot: &mut f64,
        var_inqs0_a_dn15_slot: &mut f64,
        var_inqs0_a_dn2_slot: &mut f64,
        var_inqs0_a_dn4_slot: &mut f64,
        var_inqs0_a_dn5_slot: &mut f64,
        var_inqs0_a_dn6_slot: &mut f64,
        var_inqs0_a_dn7_slot: &mut f64,
        var_inqs0_a_dn8_slot: &mut f64,
        var_inqs0_a_dn9_slot: &mut f64,
        var_inqs0_a_rv_slot: &mut f64,
        var_inqs0_k_slot: &mut f64,
        var_inqs0_k_dn0_slot: &mut f64,
        var_inqs0_k_dn10_slot: &mut f64,
        var_inqs0_k_dn13_slot: &mut f64,
        var_inqs0_k_dn16_slot: &mut f64,
        var_inqs0_k_dn2_slot: &mut f64,
        var_inqs0_k_dn4_slot: &mut f64,
        var_inqs0_k_dn5_slot: &mut f64,
        var_inqs0_k_dn6_slot: &mut f64,
        var_inqs0_k_dn7_slot: &mut f64,
        var_inqs0_k_dn8_slot: &mut f64,
        var_inqs0_k_dn9_slot: &mut f64,
        var_inqs0_k_rv_slot: &mut f64,
        var_isbs2_btm_slot: &mut f64,
        var_isbs2_btm_dn0_slot: &mut f64,
        var_isbs2_btm_dn10_slot: &mut f64,
        var_isbs2_btm_dn13_slot: &mut f64,
        var_isbs2_btm_dn2_slot: &mut f64,
        var_isbs2_btm_dn4_slot: &mut f64,
        var_isbs2_btm_dn5_slot: &mut f64,
        var_isbs2_btm_dn6_slot: &mut f64,
        var_isbs2_btm_dn7_slot: &mut f64,
        var_isbs2_btm_dn8_slot: &mut f64,
        var_isbs2_btm_dn9_slot: &mut f64,
        var_isbs2_btm_rv_slot: &mut f64,
        var_isbs2_swg_slot: &mut f64,
        var_isbs2_swg_dn0_slot: &mut f64,
        var_isbs2_swg_dn10_slot: &mut f64,
        var_isbs2_swg_dn13_slot: &mut f64,
        var_isbs2_swg_dn2_slot: &mut f64,
        var_isbs2_swg_dn4_slot: &mut f64,
        var_isbs2_swg_dn5_slot: &mut f64,
        var_isbs2_swg_dn6_slot: &mut f64,
        var_isbs2_swg_dn7_slot: &mut f64,
        var_isbs2_swg_dn8_slot: &mut f64,
        var_isbs2_swg_dn9_slot: &mut f64,
        var_isbs2_swg_rv_slot: &mut f64,
        var_isbs2_sws_slot: &mut f64,
        var_isbs2_sws_dn0_slot: &mut f64,
        var_isbs2_sws_dn10_slot: &mut f64,
        var_isbs2_sws_dn13_slot: &mut f64,
        var_isbs2_sws_dn2_slot: &mut f64,
        var_isbs2_sws_dn4_slot: &mut f64,
        var_isbs2_sws_dn5_slot: &mut f64,
        var_isbs2_sws_dn6_slot: &mut f64,
        var_isbs2_sws_dn7_slot: &mut f64,
        var_isbs2_sws_dn8_slot: &mut f64,
        var_isbs2_sws_dn9_slot: &mut f64,
        var_isbs2_sws_rv_slot: &mut f64,
        var_isbs_swg_slot: &mut f64,
        var_isbs_swg_dn0_slot: &mut f64,
        var_isbs_swg_dn10_slot: &mut f64,
        var_isbs_swg_dn13_slot: &mut f64,
        var_isbs_swg_dn2_slot: &mut f64,
        var_isbs_swg_dn4_slot: &mut f64,
        var_isbs_swg_dn5_slot: &mut f64,
        var_isbs_swg_dn6_slot: &mut f64,
        var_isbs_swg_dn7_slot: &mut f64,
        var_isbs_swg_dn8_slot: &mut f64,
        var_isbs_swg_dn9_slot: &mut f64,
        var_isbs_swg_rv_slot: &mut f64,
        var_isbs_sws_slot: &mut f64,
        var_isbs_sws_dn0_slot: &mut f64,
        var_isbs_sws_dn10_slot: &mut f64,
        var_isbs_sws_dn13_slot: &mut f64,
        var_isbs_sws_dn2_slot: &mut f64,
        var_isbs_sws_dn4_slot: &mut f64,
        var_isbs_sws_dn5_slot: &mut f64,
        var_isbs_sws_dn6_slot: &mut f64,
        var_isbs_sws_dn7_slot: &mut f64,
        var_isbs_sws_dn8_slot: &mut f64,
        var_isbs_sws_dn9_slot: &mut f64,
        var_isbs_sws_rv_slot: &mut f64,
        var_isubibpc_slot: &mut f64,
        var_isubibpc_dn0_slot: &mut f64,
        var_isubibpc_dn10_slot: &mut f64,
        var_isubibpc_dn13_slot: &mut f64,
        var_isubibpc_dn2_slot: &mut f64,
        var_isubibpc_dn4_slot: &mut f64,
        var_isubibpc_dn5_slot: &mut f64,
        var_isubibpc_dn6_slot: &mut f64,
        var_isubibpc_dn7_slot: &mut f64,
        var_isubibpc_dn8_slot: &mut f64,
        var_isubibpc_dn9_slot: &mut f64,
        var_isubibpc_rv_slot: &mut f64,
        var_iwnqs0_a_slot: &mut f64,
        var_iwnqs0_a_dn0_slot: &mut f64,
        var_iwnqs0_a_dn10_slot: &mut f64,
        var_iwnqs0_a_dn13_slot: &mut f64,
        var_iwnqs0_a_dn17_slot: &mut f64,
        var_iwnqs0_a_dn2_slot: &mut f64,
        var_iwnqs0_a_dn4_slot: &mut f64,
        var_iwnqs0_a_dn5_slot: &mut f64,
        var_iwnqs0_a_dn6_slot: &mut f64,
        var_iwnqs0_a_dn7_slot: &mut f64,
        var_iwnqs0_a_dn8_slot: &mut f64,
        var_iwnqs0_a_dn9_slot: &mut f64,
        var_iwnqs0_a_rv_slot: &mut f64,
        var_lover_func_slot: &mut f64,
        var_lover_func_dn0_slot: &mut f64,
        var_lover_func_dn10_slot: &mut f64,
        var_lover_func_dn13_slot: &mut f64,
        var_lover_func_dn2_slot: &mut f64,
        var_lover_func_dn4_slot: &mut f64,
        var_lover_func_dn5_slot: &mut f64,
        var_lover_func_dn6_slot: &mut f64,
        var_lover_func_dn7_slot: &mut f64,
        var_lover_func_dn8_slot: &mut f64,
        var_lover_func_dn9_slot: &mut f64,
        var_lover_func_rv_slot: &mut f64,
        var_mfactor_slot: &mut f64,
        var_mfactor_rv_slot: &mut f64,
        var_q_nqs_a_slot: &mut f64,
        var_q_nqs_a_dn15_slot: &mut f64,
        var_q_nqs_a_rv_slot: &mut f64,
        var_q_nqs_k_slot: &mut f64,
        var_q_nqs_k_dn16_slot: &mut f64,
        var_q_nqs_k_rv_slot: &mut f64,
        var_qbdld_add_slot: &mut f64,
        var_qbdld_add_dn0_slot: &mut f64,
        var_qbdld_add_dn10_slot: &mut f64,
        var_qbdld_add_dn13_slot: &mut f64,
        var_qbdld_add_dn2_slot: &mut f64,
        var_qbdld_add_dn4_slot: &mut f64,
        var_qbdld_add_dn5_slot: &mut f64,
        var_qbdld_add_dn6_slot: &mut f64,
        var_qbdld_add_dn7_slot: &mut f64,
        var_qbdld_add_dn8_slot: &mut f64,
        var_qbdld_add_dn9_slot: &mut f64,
        var_qbdld_add_rv_slot: &mut f64,
        var_qbsld_add_slot: &mut f64,
        var_qbsld_add_dn0_slot: &mut f64,
        var_qbsld_add_dn10_slot: &mut f64,
        var_qbsld_add_dn13_slot: &mut f64,
        var_qbsld_add_dn2_slot: &mut f64,
        var_qbsld_add_dn4_slot: &mut f64,
        var_qbsld_add_dn5_slot: &mut f64,
        var_qbsld_add_dn6_slot: &mut f64,
        var_qbsld_add_dn7_slot: &mut f64,
        var_qbsld_add_dn8_slot: &mut f64,
        var_qbsld_add_dn9_slot: &mut f64,
        var_qbsld_add_rv_slot: &mut f64,
        var_qovd_add_slot: &mut f64,
        var_qovd_add_dn0_slot: &mut f64,
        var_qovd_add_dn10_slot: &mut f64,
        var_qovd_add_dn13_slot: &mut f64,
        var_qovd_add_dn2_slot: &mut f64,
        var_qovd_add_dn4_slot: &mut f64,
        var_qovd_add_dn5_slot: &mut f64,
        var_qovd_add_dn6_slot: &mut f64,
        var_qovd_add_dn7_slot: &mut f64,
        var_qovd_add_dn8_slot: &mut f64,
        var_qovd_add_dn9_slot: &mut f64,
        var_qovd_add_rv_slot: &mut f64,
        var_qovs_add_slot: &mut f64,
        var_qovs_add_dn0_slot: &mut f64,
        var_qovs_add_dn10_slot: &mut f64,
        var_qovs_add_dn13_slot: &mut f64,
        var_qovs_add_dn2_slot: &mut f64,
        var_qovs_add_dn4_slot: &mut f64,
        var_qovs_add_dn5_slot: &mut f64,
        var_qovs_add_dn6_slot: &mut f64,
        var_qovs_add_dn7_slot: &mut f64,
        var_qovs_add_dn8_slot: &mut f64,
        var_qovs_add_dn9_slot: &mut f64,
        var_qovs_add_rv_slot: &mut f64,
        var_uc_codep_slot: &mut f64,
        var_uc_codep_rv_slot: &mut f64,
        var_uc_corsrd_slot: &mut f64,
        var_uc_corsrd_rv_slot: &mut f64,
        var_uc_depleak_slot: &mut f64,
        var_uc_depleak_dn0_slot: &mut f64,
        var_uc_depleak_dn10_slot: &mut f64,
        var_uc_depleak_dn13_slot: &mut f64,
        var_uc_depleak_dn2_slot: &mut f64,
        var_uc_depleak_dn4_slot: &mut f64,
        var_uc_depleak_dn5_slot: &mut f64,
        var_uc_depleak_dn6_slot: &mut f64,
        var_uc_depleak_dn7_slot: &mut f64,
        var_uc_depleak_dn8_slot: &mut f64,
        var_uc_depleak_dn9_slot: &mut f64,
        var_uc_depleak_rv_slot: &mut f64,
        var_uc_depmue0_slot: &mut f64,
        var_uc_depmue0_dn0_slot: &mut f64,
        var_uc_depmue0_dn10_slot: &mut f64,
        var_uc_depmue0_dn13_slot: &mut f64,
        var_uc_depmue0_dn2_slot: &mut f64,
        var_uc_depmue0_dn4_slot: &mut f64,
        var_uc_depmue0_dn5_slot: &mut f64,
        var_uc_depmue0_dn6_slot: &mut f64,
        var_uc_depmue0_dn7_slot: &mut f64,
        var_uc_depmue0_dn8_slot: &mut f64,
        var_uc_depmue0_dn9_slot: &mut f64,
        var_uc_depmue0_rv_slot: &mut f64,
        var_uc_depmue1_slot: &mut f64,
        var_uc_depmue1_dn0_slot: &mut f64,
        var_uc_depmue1_dn10_slot: &mut f64,
        var_uc_depmue1_dn13_slot: &mut f64,
        var_uc_depmue1_dn2_slot: &mut f64,
        var_uc_depmue1_dn4_slot: &mut f64,
        var_uc_depmue1_dn5_slot: &mut f64,
        var_uc_depmue1_dn6_slot: &mut f64,
        var_uc_depmue1_dn7_slot: &mut f64,
        var_uc_depmue1_dn8_slot: &mut f64,
        var_uc_depmue1_dn9_slot: &mut f64,
        var_uc_depmue1_rv_slot: &mut f64,
        var_uc_depmue2_slot: &mut f64,
        var_uc_depmue2_dn0_slot: &mut f64,
        var_uc_depmue2_dn10_slot: &mut f64,
        var_uc_depmue2_dn13_slot: &mut f64,
        var_uc_depmue2_dn2_slot: &mut f64,
        var_uc_depmue2_dn4_slot: &mut f64,
        var_uc_depmue2_dn5_slot: &mut f64,
        var_uc_depmue2_dn6_slot: &mut f64,
        var_uc_depmue2_dn7_slot: &mut f64,
        var_uc_depmue2_dn8_slot: &mut f64,
        var_uc_depmue2_dn9_slot: &mut f64,
        var_uc_depmue2_rv_slot: &mut f64,
        var_uc_depmueback0_slot: &mut f64,
        var_uc_depmueback0_dn0_slot: &mut f64,
        var_uc_depmueback0_dn10_slot: &mut f64,
        var_uc_depmueback0_dn13_slot: &mut f64,
        var_uc_depmueback0_dn2_slot: &mut f64,
        var_uc_depmueback0_dn4_slot: &mut f64,
        var_uc_depmueback0_dn5_slot: &mut f64,
        var_uc_depmueback0_dn6_slot: &mut f64,
        var_uc_depmueback0_dn7_slot: &mut f64,
        var_uc_depmueback0_dn8_slot: &mut f64,
        var_uc_depmueback0_dn9_slot: &mut f64,
        var_uc_depmueback0_rv_slot: &mut f64,
        var_uc_depmueback1_slot: &mut f64,
        var_uc_depmueback1_dn0_slot: &mut f64,
        var_uc_depmueback1_dn10_slot: &mut f64,
        var_uc_depmueback1_dn13_slot: &mut f64,
        var_uc_depmueback1_dn2_slot: &mut f64,
        var_uc_depmueback1_dn4_slot: &mut f64,
        var_uc_depmueback1_dn5_slot: &mut f64,
        var_uc_depmueback1_dn6_slot: &mut f64,
        var_uc_depmueback1_dn7_slot: &mut f64,
        var_uc_depmueback1_dn8_slot: &mut f64,
        var_uc_depmueback1_dn9_slot: &mut f64,
        var_uc_depmueback1_rv_slot: &mut f64,
        var_uc_depmueph1_slot: &mut f64,
        var_uc_depmueph1_rv_slot: &mut f64,
        var_uc_depthn_slot: &mut f64,
        var_uc_depthn_dn0_slot: &mut f64,
        var_uc_depthn_dn10_slot: &mut f64,
        var_uc_depthn_dn13_slot: &mut f64,
        var_uc_depthn_dn2_slot: &mut f64,
        var_uc_depthn_dn4_slot: &mut f64,
        var_uc_depthn_dn5_slot: &mut f64,
        var_uc_depthn_dn6_slot: &mut f64,
        var_uc_depthn_dn7_slot: &mut f64,
        var_uc_depthn_dn8_slot: &mut f64,
        var_uc_depthn_dn9_slot: &mut f64,
        var_uc_depthn_rv_slot: &mut f64,
        var_uc_depvmax_slot: &mut f64,
        var_uc_depvmax_dn0_slot: &mut f64,
        var_uc_depvmax_dn10_slot: &mut f64,
        var_uc_depvmax_dn13_slot: &mut f64,
        var_uc_depvmax_dn2_slot: &mut f64,
        var_uc_depvmax_dn4_slot: &mut f64,
        var_uc_depvmax_dn5_slot: &mut f64,
        var_uc_depvmax_dn6_slot: &mut f64,
        var_uc_depvmax_dn7_slot: &mut f64,
        var_uc_depvmax_dn8_slot: &mut f64,
        var_uc_depvmax_dn9_slot: &mut f64,
        var_uc_depvmax_rv_slot: &mut f64,
        var_uc_depwlp_slot: &mut f64,
        var_uc_depwlp_dn0_slot: &mut f64,
        var_uc_depwlp_dn10_slot: &mut f64,
        var_uc_depwlp_dn13_slot: &mut f64,
        var_uc_depwlp_dn2_slot: &mut f64,
        var_uc_depwlp_dn4_slot: &mut f64,
        var_uc_depwlp_dn5_slot: &mut f64,
        var_uc_depwlp_dn6_slot: &mut f64,
        var_uc_depwlp_dn7_slot: &mut f64,
        var_uc_depwlp_dn8_slot: &mut f64,
        var_uc_depwlp_dn9_slot: &mut f64,
        var_uc_depwlp_rv_slot: &mut f64,
        var_uc_ndepm_slot: &mut f64,
        var_uc_ndepm_dn0_slot: &mut f64,
        var_uc_ndepm_dn10_slot: &mut f64,
        var_uc_ndepm_dn13_slot: &mut f64,
        var_uc_ndepm_dn2_slot: &mut f64,
        var_uc_ndepm_dn4_slot: &mut f64,
        var_uc_ndepm_dn5_slot: &mut f64,
        var_uc_ndepm_dn6_slot: &mut f64,
        var_uc_ndepm_dn7_slot: &mut f64,
        var_uc_ndepm_dn8_slot: &mut f64,
        var_uc_ndepm_dn9_slot: &mut f64,
        var_uc_ndepm_rv_slot: &mut f64,
        var_uc_rdrcx_slot: &mut f64,
        var_uc_rdrcx_rv_slot: &mut f64,
        var_uc_scp22_slot: &mut f64,
        var_uc_scp22_rv_slot: &mut f64,
        var_uc_xldld_slot: &mut f64,
        var_uc_xldld_rv_slot: &mut f64,
        var_uc_xpdv_slot: &mut f64,
        var_uc_xpdv_rv_slot: &mut f64,
        var_w_nqs_a_slot: &mut f64,
        var_w_nqs_a_dn17_slot: &mut f64,
        var_w_nqs_a_rv_slot: &mut f64,
        var_w_res_slot: &mut f64,
        var_w_res_dn0_slot: &mut f64,
        var_w_res_dn10_slot: &mut f64,
        var_w_res_dn13_slot: &mut f64,
        var_w_res_dn2_slot: &mut f64,
        var_w_res_dn4_slot: &mut f64,
        var_w_res_dn5_slot: &mut f64,
        var_w_res_dn6_slot: &mut f64,
        var_w_res_dn7_slot: &mut f64,
        var_w_res_dn8_slot: &mut f64,
        var_w_res_dn9_slot: &mut f64,
        var_w_res_rv_slot: &mut f64,
        var_wdep_func_slot: &mut f64,
        var_wdep_func_dn0_slot: &mut f64,
        var_wdep_func_dn10_slot: &mut f64,
        var_wdep_func_dn13_slot: &mut f64,
        var_wdep_func_dn2_slot: &mut f64,
        var_wdep_func_dn4_slot: &mut f64,
        var_wdep_func_dn5_slot: &mut f64,
        var_wdep_func_dn6_slot: &mut f64,
        var_wdep_func_dn7_slot: &mut f64,
        var_wdep_func_dn8_slot: &mut f64,
        var_wdep_func_dn9_slot: &mut f64,
        var_wdep_func_rv_slot: &mut f64,
        var_wjuncld_slot: &mut f64,
        var_wjuncld_dn0_slot: &mut f64,
        var_wjuncld_dn10_slot: &mut f64,
        var_wjuncld_dn13_slot: &mut f64,
        var_wjuncld_dn2_slot: &mut f64,
        var_wjuncld_dn4_slot: &mut f64,
        var_wjuncld_dn5_slot: &mut f64,
        var_wjuncld_dn6_slot: &mut f64,
        var_wjuncld_dn7_slot: &mut f64,
        var_wjuncld_dn8_slot: &mut f64,
        var_wjuncld_dn9_slot: &mut f64,
        var_wjuncld_rv_slot: &mut f64,
        var_wk_ii_slot: &mut f64,
        var_wk_ii_dn0_slot: &mut f64,
        var_wk_ii_dn10_slot: &mut f64,
        var_wk_ii_dn13_slot: &mut f64,
        var_wk_ii_dn2_slot: &mut f64,
        var_wk_ii_dn4_slot: &mut f64,
        var_wk_ii_dn5_slot: &mut f64,
        var_wk_ii_dn6_slot: &mut f64,
        var_wk_ii_dn7_slot: &mut f64,
        var_wk_ii_dn8_slot: &mut f64,
        var_wk_ii_dn9_slot: &mut f64,
        var_wk_ii_rv_slot: &mut f64,
    ) {
        let mut var_cox0_func: f64 = *var_cox0_func_slot;
        let mut var_cox0_func_rv: f64 = *var_cox0_func_rv_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard11_rv: f64 = *var_guard11_rv_slot;
        let mut var_guard14: f64 = *var_guard14_slot;
        let mut var_guard14_rv: f64 = *var_guard14_rv_slot;
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_guard15_rv: f64 = *var_guard15_rv_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard8_rv: f64 = *var_guard8_rv_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_guard9_rv: f64 = *var_guard9_rv_slot;
        let mut var_idspt0: f64 = *var_idspt0_slot;
        let mut var_idspt0_dn0: f64 = *var_idspt0_dn0_slot;
        let mut var_idspt0_dn10: f64 = *var_idspt0_dn10_slot;
        let mut var_idspt0_dn13: f64 = *var_idspt0_dn13_slot;
        let mut var_idspt0_dn2: f64 = *var_idspt0_dn2_slot;
        let mut var_idspt0_dn4: f64 = *var_idspt0_dn4_slot;
        let mut var_idspt0_dn5: f64 = *var_idspt0_dn5_slot;
        let mut var_idspt0_dn6: f64 = *var_idspt0_dn6_slot;
        let mut var_idspt0_dn7: f64 = *var_idspt0_dn7_slot;
        let mut var_idspt0_dn8: f64 = *var_idspt0_dn8_slot;
        let mut var_idspt0_dn9: f64 = *var_idspt0_dn9_slot;
        let mut var_idspt0_rv: f64 = *var_idspt0_rv_slot;
        let mut var_idspt1: f64 = *var_idspt1_slot;
        let mut var_idspt1_dn0: f64 = *var_idspt1_dn0_slot;
        let mut var_idspt1_dn10: f64 = *var_idspt1_dn10_slot;
        let mut var_idspt1_dn13: f64 = *var_idspt1_dn13_slot;
        let mut var_idspt1_dn2: f64 = *var_idspt1_dn2_slot;
        let mut var_idspt1_dn4: f64 = *var_idspt1_dn4_slot;
        let mut var_idspt1_dn5: f64 = *var_idspt1_dn5_slot;
        let mut var_idspt1_dn6: f64 = *var_idspt1_dn6_slot;
        let mut var_idspt1_dn7: f64 = *var_idspt1_dn7_slot;
        let mut var_idspt1_dn8: f64 = *var_idspt1_dn8_slot;
        let mut var_idspt1_dn9: f64 = *var_idspt1_dn9_slot;
        let mut var_idspt1_rv: f64 = *var_idspt1_rv_slot;
        let mut var_inqs0_a: f64 = *var_inqs0_a_slot;
        let mut var_inqs0_a_dn0: f64 = *var_inqs0_a_dn0_slot;
        let mut var_inqs0_a_dn10: f64 = *var_inqs0_a_dn10_slot;
        let mut var_inqs0_a_dn13: f64 = *var_inqs0_a_dn13_slot;
        let mut var_inqs0_a_dn15: f64 = *var_inqs0_a_dn15_slot;
        let mut var_inqs0_a_dn2: f64 = *var_inqs0_a_dn2_slot;
        let mut var_inqs0_a_dn4: f64 = *var_inqs0_a_dn4_slot;
        let mut var_inqs0_a_dn5: f64 = *var_inqs0_a_dn5_slot;
        let mut var_inqs0_a_dn6: f64 = *var_inqs0_a_dn6_slot;
        let mut var_inqs0_a_dn7: f64 = *var_inqs0_a_dn7_slot;
        let mut var_inqs0_a_dn8: f64 = *var_inqs0_a_dn8_slot;
        let mut var_inqs0_a_dn9: f64 = *var_inqs0_a_dn9_slot;
        let mut var_inqs0_a_rv: f64 = *var_inqs0_a_rv_slot;
        let mut var_inqs0_k: f64 = *var_inqs0_k_slot;
        let mut var_inqs0_k_dn0: f64 = *var_inqs0_k_dn0_slot;
        let mut var_inqs0_k_dn10: f64 = *var_inqs0_k_dn10_slot;
        let mut var_inqs0_k_dn13: f64 = *var_inqs0_k_dn13_slot;
        let mut var_inqs0_k_dn16: f64 = *var_inqs0_k_dn16_slot;
        let mut var_inqs0_k_dn2: f64 = *var_inqs0_k_dn2_slot;
        let mut var_inqs0_k_dn4: f64 = *var_inqs0_k_dn4_slot;
        let mut var_inqs0_k_dn5: f64 = *var_inqs0_k_dn5_slot;
        let mut var_inqs0_k_dn6: f64 = *var_inqs0_k_dn6_slot;
        let mut var_inqs0_k_dn7: f64 = *var_inqs0_k_dn7_slot;
        let mut var_inqs0_k_dn8: f64 = *var_inqs0_k_dn8_slot;
        let mut var_inqs0_k_dn9: f64 = *var_inqs0_k_dn9_slot;
        let mut var_inqs0_k_rv: f64 = *var_inqs0_k_rv_slot;
        let mut var_isbs2_btm: f64 = *var_isbs2_btm_slot;
        let mut var_isbs2_btm_dn0: f64 = *var_isbs2_btm_dn0_slot;
        let mut var_isbs2_btm_dn10: f64 = *var_isbs2_btm_dn10_slot;
        let mut var_isbs2_btm_dn13: f64 = *var_isbs2_btm_dn13_slot;
        let mut var_isbs2_btm_dn2: f64 = *var_isbs2_btm_dn2_slot;
        let mut var_isbs2_btm_dn4: f64 = *var_isbs2_btm_dn4_slot;
        let mut var_isbs2_btm_dn5: f64 = *var_isbs2_btm_dn5_slot;
        let mut var_isbs2_btm_dn6: f64 = *var_isbs2_btm_dn6_slot;
        let mut var_isbs2_btm_dn7: f64 = *var_isbs2_btm_dn7_slot;
        let mut var_isbs2_btm_dn8: f64 = *var_isbs2_btm_dn8_slot;
        let mut var_isbs2_btm_dn9: f64 = *var_isbs2_btm_dn9_slot;
        let mut var_isbs2_btm_rv: f64 = *var_isbs2_btm_rv_slot;
        let mut var_isbs2_swg: f64 = *var_isbs2_swg_slot;
        let mut var_isbs2_swg_dn0: f64 = *var_isbs2_swg_dn0_slot;
        let mut var_isbs2_swg_dn10: f64 = *var_isbs2_swg_dn10_slot;
        let mut var_isbs2_swg_dn13: f64 = *var_isbs2_swg_dn13_slot;
        let mut var_isbs2_swg_dn2: f64 = *var_isbs2_swg_dn2_slot;
        let mut var_isbs2_swg_dn4: f64 = *var_isbs2_swg_dn4_slot;
        let mut var_isbs2_swg_dn5: f64 = *var_isbs2_swg_dn5_slot;
        let mut var_isbs2_swg_dn6: f64 = *var_isbs2_swg_dn6_slot;
        let mut var_isbs2_swg_dn7: f64 = *var_isbs2_swg_dn7_slot;
        let mut var_isbs2_swg_dn8: f64 = *var_isbs2_swg_dn8_slot;
        let mut var_isbs2_swg_dn9: f64 = *var_isbs2_swg_dn9_slot;
        let mut var_isbs2_swg_rv: f64 = *var_isbs2_swg_rv_slot;
        let mut var_isbs2_sws: f64 = *var_isbs2_sws_slot;
        let mut var_isbs2_sws_dn0: f64 = *var_isbs2_sws_dn0_slot;
        let mut var_isbs2_sws_dn10: f64 = *var_isbs2_sws_dn10_slot;
        let mut var_isbs2_sws_dn13: f64 = *var_isbs2_sws_dn13_slot;
        let mut var_isbs2_sws_dn2: f64 = *var_isbs2_sws_dn2_slot;
        let mut var_isbs2_sws_dn4: f64 = *var_isbs2_sws_dn4_slot;
        let mut var_isbs2_sws_dn5: f64 = *var_isbs2_sws_dn5_slot;
        let mut var_isbs2_sws_dn6: f64 = *var_isbs2_sws_dn6_slot;
        let mut var_isbs2_sws_dn7: f64 = *var_isbs2_sws_dn7_slot;
        let mut var_isbs2_sws_dn8: f64 = *var_isbs2_sws_dn8_slot;
        let mut var_isbs2_sws_dn9: f64 = *var_isbs2_sws_dn9_slot;
        let mut var_isbs2_sws_rv: f64 = *var_isbs2_sws_rv_slot;
        let mut var_isbs_swg: f64 = *var_isbs_swg_slot;
        let mut var_isbs_swg_dn0: f64 = *var_isbs_swg_dn0_slot;
        let mut var_isbs_swg_dn10: f64 = *var_isbs_swg_dn10_slot;
        let mut var_isbs_swg_dn13: f64 = *var_isbs_swg_dn13_slot;
        let mut var_isbs_swg_dn2: f64 = *var_isbs_swg_dn2_slot;
        let mut var_isbs_swg_dn4: f64 = *var_isbs_swg_dn4_slot;
        let mut var_isbs_swg_dn5: f64 = *var_isbs_swg_dn5_slot;
        let mut var_isbs_swg_dn6: f64 = *var_isbs_swg_dn6_slot;
        let mut var_isbs_swg_dn7: f64 = *var_isbs_swg_dn7_slot;
        let mut var_isbs_swg_dn8: f64 = *var_isbs_swg_dn8_slot;
        let mut var_isbs_swg_dn9: f64 = *var_isbs_swg_dn9_slot;
        let mut var_isbs_swg_rv: f64 = *var_isbs_swg_rv_slot;
        let mut var_isbs_sws: f64 = *var_isbs_sws_slot;
        let mut var_isbs_sws_dn0: f64 = *var_isbs_sws_dn0_slot;
        let mut var_isbs_sws_dn10: f64 = *var_isbs_sws_dn10_slot;
        let mut var_isbs_sws_dn13: f64 = *var_isbs_sws_dn13_slot;
        let mut var_isbs_sws_dn2: f64 = *var_isbs_sws_dn2_slot;
        let mut var_isbs_sws_dn4: f64 = *var_isbs_sws_dn4_slot;
        let mut var_isbs_sws_dn5: f64 = *var_isbs_sws_dn5_slot;
        let mut var_isbs_sws_dn6: f64 = *var_isbs_sws_dn6_slot;
        let mut var_isbs_sws_dn7: f64 = *var_isbs_sws_dn7_slot;
        let mut var_isbs_sws_dn8: f64 = *var_isbs_sws_dn8_slot;
        let mut var_isbs_sws_dn9: f64 = *var_isbs_sws_dn9_slot;
        let mut var_isbs_sws_rv: f64 = *var_isbs_sws_rv_slot;
        let mut var_isubibpc: f64 = *var_isubibpc_slot;
        let mut var_isubibpc_dn0: f64 = *var_isubibpc_dn0_slot;
        let mut var_isubibpc_dn10: f64 = *var_isubibpc_dn10_slot;
        let mut var_isubibpc_dn13: f64 = *var_isubibpc_dn13_slot;
        let mut var_isubibpc_dn2: f64 = *var_isubibpc_dn2_slot;
        let mut var_isubibpc_dn4: f64 = *var_isubibpc_dn4_slot;
        let mut var_isubibpc_dn5: f64 = *var_isubibpc_dn5_slot;
        let mut var_isubibpc_dn6: f64 = *var_isubibpc_dn6_slot;
        let mut var_isubibpc_dn7: f64 = *var_isubibpc_dn7_slot;
        let mut var_isubibpc_dn8: f64 = *var_isubibpc_dn8_slot;
        let mut var_isubibpc_dn9: f64 = *var_isubibpc_dn9_slot;
        let mut var_isubibpc_rv: f64 = *var_isubibpc_rv_slot;
        let mut var_iwnqs0_a: f64 = *var_iwnqs0_a_slot;
        let mut var_iwnqs0_a_dn0: f64 = *var_iwnqs0_a_dn0_slot;
        let mut var_iwnqs0_a_dn10: f64 = *var_iwnqs0_a_dn10_slot;
        let mut var_iwnqs0_a_dn13: f64 = *var_iwnqs0_a_dn13_slot;
        let mut var_iwnqs0_a_dn17: f64 = *var_iwnqs0_a_dn17_slot;
        let mut var_iwnqs0_a_dn2: f64 = *var_iwnqs0_a_dn2_slot;
        let mut var_iwnqs0_a_dn4: f64 = *var_iwnqs0_a_dn4_slot;
        let mut var_iwnqs0_a_dn5: f64 = *var_iwnqs0_a_dn5_slot;
        let mut var_iwnqs0_a_dn6: f64 = *var_iwnqs0_a_dn6_slot;
        let mut var_iwnqs0_a_dn7: f64 = *var_iwnqs0_a_dn7_slot;
        let mut var_iwnqs0_a_dn8: f64 = *var_iwnqs0_a_dn8_slot;
        let mut var_iwnqs0_a_dn9: f64 = *var_iwnqs0_a_dn9_slot;
        let mut var_iwnqs0_a_rv: f64 = *var_iwnqs0_a_rv_slot;
        let mut var_lover_func: f64 = *var_lover_func_slot;
        let mut var_lover_func_dn0: f64 = *var_lover_func_dn0_slot;
        let mut var_lover_func_dn10: f64 = *var_lover_func_dn10_slot;
        let mut var_lover_func_dn13: f64 = *var_lover_func_dn13_slot;
        let mut var_lover_func_dn2: f64 = *var_lover_func_dn2_slot;
        let mut var_lover_func_dn4: f64 = *var_lover_func_dn4_slot;
        let mut var_lover_func_dn5: f64 = *var_lover_func_dn5_slot;
        let mut var_lover_func_dn6: f64 = *var_lover_func_dn6_slot;
        let mut var_lover_func_dn7: f64 = *var_lover_func_dn7_slot;
        let mut var_lover_func_dn8: f64 = *var_lover_func_dn8_slot;
        let mut var_lover_func_dn9: f64 = *var_lover_func_dn9_slot;
        let mut var_lover_func_rv: f64 = *var_lover_func_rv_slot;
        let mut var_mfactor: f64 = *var_mfactor_slot;
        let mut var_mfactor_rv: f64 = *var_mfactor_rv_slot;
        let mut var_q_nqs_a: f64 = *var_q_nqs_a_slot;
        let mut var_q_nqs_a_dn15: f64 = *var_q_nqs_a_dn15_slot;
        let mut var_q_nqs_a_rv: f64 = *var_q_nqs_a_rv_slot;
        let mut var_q_nqs_k: f64 = *var_q_nqs_k_slot;
        let mut var_q_nqs_k_dn16: f64 = *var_q_nqs_k_dn16_slot;
        let mut var_q_nqs_k_rv: f64 = *var_q_nqs_k_rv_slot;
        let mut var_qbdld_add: f64 = *var_qbdld_add_slot;
        let mut var_qbdld_add_dn0: f64 = *var_qbdld_add_dn0_slot;
        let mut var_qbdld_add_dn10: f64 = *var_qbdld_add_dn10_slot;
        let mut var_qbdld_add_dn13: f64 = *var_qbdld_add_dn13_slot;
        let mut var_qbdld_add_dn2: f64 = *var_qbdld_add_dn2_slot;
        let mut var_qbdld_add_dn4: f64 = *var_qbdld_add_dn4_slot;
        let mut var_qbdld_add_dn5: f64 = *var_qbdld_add_dn5_slot;
        let mut var_qbdld_add_dn6: f64 = *var_qbdld_add_dn6_slot;
        let mut var_qbdld_add_dn7: f64 = *var_qbdld_add_dn7_slot;
        let mut var_qbdld_add_dn8: f64 = *var_qbdld_add_dn8_slot;
        let mut var_qbdld_add_dn9: f64 = *var_qbdld_add_dn9_slot;
        let mut var_qbdld_add_rv: f64 = *var_qbdld_add_rv_slot;
        let mut var_qbsld_add: f64 = *var_qbsld_add_slot;
        let mut var_qbsld_add_dn0: f64 = *var_qbsld_add_dn0_slot;
        let mut var_qbsld_add_dn10: f64 = *var_qbsld_add_dn10_slot;
        let mut var_qbsld_add_dn13: f64 = *var_qbsld_add_dn13_slot;
        let mut var_qbsld_add_dn2: f64 = *var_qbsld_add_dn2_slot;
        let mut var_qbsld_add_dn4: f64 = *var_qbsld_add_dn4_slot;
        let mut var_qbsld_add_dn5: f64 = *var_qbsld_add_dn5_slot;
        let mut var_qbsld_add_dn6: f64 = *var_qbsld_add_dn6_slot;
        let mut var_qbsld_add_dn7: f64 = *var_qbsld_add_dn7_slot;
        let mut var_qbsld_add_dn8: f64 = *var_qbsld_add_dn8_slot;
        let mut var_qbsld_add_dn9: f64 = *var_qbsld_add_dn9_slot;
        let mut var_qbsld_add_rv: f64 = *var_qbsld_add_rv_slot;
        let mut var_qovd_add: f64 = *var_qovd_add_slot;
        let mut var_qovd_add_dn0: f64 = *var_qovd_add_dn0_slot;
        let mut var_qovd_add_dn10: f64 = *var_qovd_add_dn10_slot;
        let mut var_qovd_add_dn13: f64 = *var_qovd_add_dn13_slot;
        let mut var_qovd_add_dn2: f64 = *var_qovd_add_dn2_slot;
        let mut var_qovd_add_dn4: f64 = *var_qovd_add_dn4_slot;
        let mut var_qovd_add_dn5: f64 = *var_qovd_add_dn5_slot;
        let mut var_qovd_add_dn6: f64 = *var_qovd_add_dn6_slot;
        let mut var_qovd_add_dn7: f64 = *var_qovd_add_dn7_slot;
        let mut var_qovd_add_dn8: f64 = *var_qovd_add_dn8_slot;
        let mut var_qovd_add_dn9: f64 = *var_qovd_add_dn9_slot;
        let mut var_qovd_add_rv: f64 = *var_qovd_add_rv_slot;
        let mut var_qovs_add: f64 = *var_qovs_add_slot;
        let mut var_qovs_add_dn0: f64 = *var_qovs_add_dn0_slot;
        let mut var_qovs_add_dn10: f64 = *var_qovs_add_dn10_slot;
        let mut var_qovs_add_dn13: f64 = *var_qovs_add_dn13_slot;
        let mut var_qovs_add_dn2: f64 = *var_qovs_add_dn2_slot;
        let mut var_qovs_add_dn4: f64 = *var_qovs_add_dn4_slot;
        let mut var_qovs_add_dn5: f64 = *var_qovs_add_dn5_slot;
        let mut var_qovs_add_dn6: f64 = *var_qovs_add_dn6_slot;
        let mut var_qovs_add_dn7: f64 = *var_qovs_add_dn7_slot;
        let mut var_qovs_add_dn8: f64 = *var_qovs_add_dn8_slot;
        let mut var_qovs_add_dn9: f64 = *var_qovs_add_dn9_slot;
        let mut var_qovs_add_rv: f64 = *var_qovs_add_rv_slot;
        let mut var_uc_codep: f64 = *var_uc_codep_slot;
        let mut var_uc_codep_rv: f64 = *var_uc_codep_rv_slot;
        let mut var_uc_corsrd: f64 = *var_uc_corsrd_slot;
        let mut var_uc_corsrd_rv: f64 = *var_uc_corsrd_rv_slot;
        let mut var_uc_depleak: f64 = *var_uc_depleak_slot;
        let mut var_uc_depleak_dn0: f64 = *var_uc_depleak_dn0_slot;
        let mut var_uc_depleak_dn10: f64 = *var_uc_depleak_dn10_slot;
        let mut var_uc_depleak_dn13: f64 = *var_uc_depleak_dn13_slot;
        let mut var_uc_depleak_dn2: f64 = *var_uc_depleak_dn2_slot;
        let mut var_uc_depleak_dn4: f64 = *var_uc_depleak_dn4_slot;
        let mut var_uc_depleak_dn5: f64 = *var_uc_depleak_dn5_slot;
        let mut var_uc_depleak_dn6: f64 = *var_uc_depleak_dn6_slot;
        let mut var_uc_depleak_dn7: f64 = *var_uc_depleak_dn7_slot;
        let mut var_uc_depleak_dn8: f64 = *var_uc_depleak_dn8_slot;
        let mut var_uc_depleak_dn9: f64 = *var_uc_depleak_dn9_slot;
        let mut var_uc_depleak_rv: f64 = *var_uc_depleak_rv_slot;
        let mut var_uc_depmue0: f64 = *var_uc_depmue0_slot;
        let mut var_uc_depmue0_dn0: f64 = *var_uc_depmue0_dn0_slot;
        let mut var_uc_depmue0_dn10: f64 = *var_uc_depmue0_dn10_slot;
        let mut var_uc_depmue0_dn13: f64 = *var_uc_depmue0_dn13_slot;
        let mut var_uc_depmue0_dn2: f64 = *var_uc_depmue0_dn2_slot;
        let mut var_uc_depmue0_dn4: f64 = *var_uc_depmue0_dn4_slot;
        let mut var_uc_depmue0_dn5: f64 = *var_uc_depmue0_dn5_slot;
        let mut var_uc_depmue0_dn6: f64 = *var_uc_depmue0_dn6_slot;
        let mut var_uc_depmue0_dn7: f64 = *var_uc_depmue0_dn7_slot;
        let mut var_uc_depmue0_dn8: f64 = *var_uc_depmue0_dn8_slot;
        let mut var_uc_depmue0_dn9: f64 = *var_uc_depmue0_dn9_slot;
        let mut var_uc_depmue0_rv: f64 = *var_uc_depmue0_rv_slot;
        let mut var_uc_depmue1: f64 = *var_uc_depmue1_slot;
        let mut var_uc_depmue1_dn0: f64 = *var_uc_depmue1_dn0_slot;
        let mut var_uc_depmue1_dn10: f64 = *var_uc_depmue1_dn10_slot;
        let mut var_uc_depmue1_dn13: f64 = *var_uc_depmue1_dn13_slot;
        let mut var_uc_depmue1_dn2: f64 = *var_uc_depmue1_dn2_slot;
        let mut var_uc_depmue1_dn4: f64 = *var_uc_depmue1_dn4_slot;
        let mut var_uc_depmue1_dn5: f64 = *var_uc_depmue1_dn5_slot;
        let mut var_uc_depmue1_dn6: f64 = *var_uc_depmue1_dn6_slot;
        let mut var_uc_depmue1_dn7: f64 = *var_uc_depmue1_dn7_slot;
        let mut var_uc_depmue1_dn8: f64 = *var_uc_depmue1_dn8_slot;
        let mut var_uc_depmue1_dn9: f64 = *var_uc_depmue1_dn9_slot;
        let mut var_uc_depmue1_rv: f64 = *var_uc_depmue1_rv_slot;
        let mut var_uc_depmue2: f64 = *var_uc_depmue2_slot;
        let mut var_uc_depmue2_dn0: f64 = *var_uc_depmue2_dn0_slot;
        let mut var_uc_depmue2_dn10: f64 = *var_uc_depmue2_dn10_slot;
        let mut var_uc_depmue2_dn13: f64 = *var_uc_depmue2_dn13_slot;
        let mut var_uc_depmue2_dn2: f64 = *var_uc_depmue2_dn2_slot;
        let mut var_uc_depmue2_dn4: f64 = *var_uc_depmue2_dn4_slot;
        let mut var_uc_depmue2_dn5: f64 = *var_uc_depmue2_dn5_slot;
        let mut var_uc_depmue2_dn6: f64 = *var_uc_depmue2_dn6_slot;
        let mut var_uc_depmue2_dn7: f64 = *var_uc_depmue2_dn7_slot;
        let mut var_uc_depmue2_dn8: f64 = *var_uc_depmue2_dn8_slot;
        let mut var_uc_depmue2_dn9: f64 = *var_uc_depmue2_dn9_slot;
        let mut var_uc_depmue2_rv: f64 = *var_uc_depmue2_rv_slot;
        let mut var_uc_depmueback0: f64 = *var_uc_depmueback0_slot;
        let mut var_uc_depmueback0_dn0: f64 = *var_uc_depmueback0_dn0_slot;
        let mut var_uc_depmueback0_dn10: f64 = *var_uc_depmueback0_dn10_slot;
        let mut var_uc_depmueback0_dn13: f64 = *var_uc_depmueback0_dn13_slot;
        let mut var_uc_depmueback0_dn2: f64 = *var_uc_depmueback0_dn2_slot;
        let mut var_uc_depmueback0_dn4: f64 = *var_uc_depmueback0_dn4_slot;
        let mut var_uc_depmueback0_dn5: f64 = *var_uc_depmueback0_dn5_slot;
        let mut var_uc_depmueback0_dn6: f64 = *var_uc_depmueback0_dn6_slot;
        let mut var_uc_depmueback0_dn7: f64 = *var_uc_depmueback0_dn7_slot;
        let mut var_uc_depmueback0_dn8: f64 = *var_uc_depmueback0_dn8_slot;
        let mut var_uc_depmueback0_dn9: f64 = *var_uc_depmueback0_dn9_slot;
        let mut var_uc_depmueback0_rv: f64 = *var_uc_depmueback0_rv_slot;
        let mut var_uc_depmueback1: f64 = *var_uc_depmueback1_slot;
        let mut var_uc_depmueback1_dn0: f64 = *var_uc_depmueback1_dn0_slot;
        let mut var_uc_depmueback1_dn10: f64 = *var_uc_depmueback1_dn10_slot;
        let mut var_uc_depmueback1_dn13: f64 = *var_uc_depmueback1_dn13_slot;
        let mut var_uc_depmueback1_dn2: f64 = *var_uc_depmueback1_dn2_slot;
        let mut var_uc_depmueback1_dn4: f64 = *var_uc_depmueback1_dn4_slot;
        let mut var_uc_depmueback1_dn5: f64 = *var_uc_depmueback1_dn5_slot;
        let mut var_uc_depmueback1_dn6: f64 = *var_uc_depmueback1_dn6_slot;
        let mut var_uc_depmueback1_dn7: f64 = *var_uc_depmueback1_dn7_slot;
        let mut var_uc_depmueback1_dn8: f64 = *var_uc_depmueback1_dn8_slot;
        let mut var_uc_depmueback1_dn9: f64 = *var_uc_depmueback1_dn9_slot;
        let mut var_uc_depmueback1_rv: f64 = *var_uc_depmueback1_rv_slot;
        let mut var_uc_depmueph1: f64 = *var_uc_depmueph1_slot;
        let mut var_uc_depmueph1_rv: f64 = *var_uc_depmueph1_rv_slot;
        let mut var_uc_depthn: f64 = *var_uc_depthn_slot;
        let mut var_uc_depthn_dn0: f64 = *var_uc_depthn_dn0_slot;
        let mut var_uc_depthn_dn10: f64 = *var_uc_depthn_dn10_slot;
        let mut var_uc_depthn_dn13: f64 = *var_uc_depthn_dn13_slot;
        let mut var_uc_depthn_dn2: f64 = *var_uc_depthn_dn2_slot;
        let mut var_uc_depthn_dn4: f64 = *var_uc_depthn_dn4_slot;
        let mut var_uc_depthn_dn5: f64 = *var_uc_depthn_dn5_slot;
        let mut var_uc_depthn_dn6: f64 = *var_uc_depthn_dn6_slot;
        let mut var_uc_depthn_dn7: f64 = *var_uc_depthn_dn7_slot;
        let mut var_uc_depthn_dn8: f64 = *var_uc_depthn_dn8_slot;
        let mut var_uc_depthn_dn9: f64 = *var_uc_depthn_dn9_slot;
        let mut var_uc_depthn_rv: f64 = *var_uc_depthn_rv_slot;
        let mut var_uc_depvmax: f64 = *var_uc_depvmax_slot;
        let mut var_uc_depvmax_dn0: f64 = *var_uc_depvmax_dn0_slot;
        let mut var_uc_depvmax_dn10: f64 = *var_uc_depvmax_dn10_slot;
        let mut var_uc_depvmax_dn13: f64 = *var_uc_depvmax_dn13_slot;
        let mut var_uc_depvmax_dn2: f64 = *var_uc_depvmax_dn2_slot;
        let mut var_uc_depvmax_dn4: f64 = *var_uc_depvmax_dn4_slot;
        let mut var_uc_depvmax_dn5: f64 = *var_uc_depvmax_dn5_slot;
        let mut var_uc_depvmax_dn6: f64 = *var_uc_depvmax_dn6_slot;
        let mut var_uc_depvmax_dn7: f64 = *var_uc_depvmax_dn7_slot;
        let mut var_uc_depvmax_dn8: f64 = *var_uc_depvmax_dn8_slot;
        let mut var_uc_depvmax_dn9: f64 = *var_uc_depvmax_dn9_slot;
        let mut var_uc_depvmax_rv: f64 = *var_uc_depvmax_rv_slot;
        let mut var_uc_depwlp: f64 = *var_uc_depwlp_slot;
        let mut var_uc_depwlp_dn0: f64 = *var_uc_depwlp_dn0_slot;
        let mut var_uc_depwlp_dn10: f64 = *var_uc_depwlp_dn10_slot;
        let mut var_uc_depwlp_dn13: f64 = *var_uc_depwlp_dn13_slot;
        let mut var_uc_depwlp_dn2: f64 = *var_uc_depwlp_dn2_slot;
        let mut var_uc_depwlp_dn4: f64 = *var_uc_depwlp_dn4_slot;
        let mut var_uc_depwlp_dn5: f64 = *var_uc_depwlp_dn5_slot;
        let mut var_uc_depwlp_dn6: f64 = *var_uc_depwlp_dn6_slot;
        let mut var_uc_depwlp_dn7: f64 = *var_uc_depwlp_dn7_slot;
        let mut var_uc_depwlp_dn8: f64 = *var_uc_depwlp_dn8_slot;
        let mut var_uc_depwlp_dn9: f64 = *var_uc_depwlp_dn9_slot;
        let mut var_uc_depwlp_rv: f64 = *var_uc_depwlp_rv_slot;
        let mut var_uc_ndepm: f64 = *var_uc_ndepm_slot;
        let mut var_uc_ndepm_dn0: f64 = *var_uc_ndepm_dn0_slot;
        let mut var_uc_ndepm_dn10: f64 = *var_uc_ndepm_dn10_slot;
        let mut var_uc_ndepm_dn13: f64 = *var_uc_ndepm_dn13_slot;
        let mut var_uc_ndepm_dn2: f64 = *var_uc_ndepm_dn2_slot;
        let mut var_uc_ndepm_dn4: f64 = *var_uc_ndepm_dn4_slot;
        let mut var_uc_ndepm_dn5: f64 = *var_uc_ndepm_dn5_slot;
        let mut var_uc_ndepm_dn6: f64 = *var_uc_ndepm_dn6_slot;
        let mut var_uc_ndepm_dn7: f64 = *var_uc_ndepm_dn7_slot;
        let mut var_uc_ndepm_dn8: f64 = *var_uc_ndepm_dn8_slot;
        let mut var_uc_ndepm_dn9: f64 = *var_uc_ndepm_dn9_slot;
        let mut var_uc_ndepm_rv: f64 = *var_uc_ndepm_rv_slot;
        let mut var_uc_rdrcx: f64 = *var_uc_rdrcx_slot;
        let mut var_uc_rdrcx_rv: f64 = *var_uc_rdrcx_rv_slot;
        let mut var_uc_scp22: f64 = *var_uc_scp22_slot;
        let mut var_uc_scp22_rv: f64 = *var_uc_scp22_rv_slot;
        let mut var_uc_xldld: f64 = *var_uc_xldld_slot;
        let mut var_uc_xldld_rv: f64 = *var_uc_xldld_rv_slot;
        let mut var_uc_xpdv: f64 = *var_uc_xpdv_slot;
        let mut var_uc_xpdv_rv: f64 = *var_uc_xpdv_rv_slot;
        let mut var_w_nqs_a: f64 = *var_w_nqs_a_slot;
        let mut var_w_nqs_a_dn17: f64 = *var_w_nqs_a_dn17_slot;
        let mut var_w_nqs_a_rv: f64 = *var_w_nqs_a_rv_slot;
        let mut var_w_res: f64 = *var_w_res_slot;
        let mut var_w_res_dn0: f64 = *var_w_res_dn0_slot;
        let mut var_w_res_dn10: f64 = *var_w_res_dn10_slot;
        let mut var_w_res_dn13: f64 = *var_w_res_dn13_slot;
        let mut var_w_res_dn2: f64 = *var_w_res_dn2_slot;
        let mut var_w_res_dn4: f64 = *var_w_res_dn4_slot;
        let mut var_w_res_dn5: f64 = *var_w_res_dn5_slot;
        let mut var_w_res_dn6: f64 = *var_w_res_dn6_slot;
        let mut var_w_res_dn7: f64 = *var_w_res_dn7_slot;
        let mut var_w_res_dn8: f64 = *var_w_res_dn8_slot;
        let mut var_w_res_dn9: f64 = *var_w_res_dn9_slot;
        let mut var_w_res_rv: f64 = *var_w_res_rv_slot;
        let mut var_wdep_func: f64 = *var_wdep_func_slot;
        let mut var_wdep_func_dn0: f64 = *var_wdep_func_dn0_slot;
        let mut var_wdep_func_dn10: f64 = *var_wdep_func_dn10_slot;
        let mut var_wdep_func_dn13: f64 = *var_wdep_func_dn13_slot;
        let mut var_wdep_func_dn2: f64 = *var_wdep_func_dn2_slot;
        let mut var_wdep_func_dn4: f64 = *var_wdep_func_dn4_slot;
        let mut var_wdep_func_dn5: f64 = *var_wdep_func_dn5_slot;
        let mut var_wdep_func_dn6: f64 = *var_wdep_func_dn6_slot;
        let mut var_wdep_func_dn7: f64 = *var_wdep_func_dn7_slot;
        let mut var_wdep_func_dn8: f64 = *var_wdep_func_dn8_slot;
        let mut var_wdep_func_dn9: f64 = *var_wdep_func_dn9_slot;
        let mut var_wdep_func_rv: f64 = *var_wdep_func_rv_slot;
        let mut var_wjuncld: f64 = *var_wjuncld_slot;
        let mut var_wjuncld_dn0: f64 = *var_wjuncld_dn0_slot;
        let mut var_wjuncld_dn10: f64 = *var_wjuncld_dn10_slot;
        let mut var_wjuncld_dn13: f64 = *var_wjuncld_dn13_slot;
        let mut var_wjuncld_dn2: f64 = *var_wjuncld_dn2_slot;
        let mut var_wjuncld_dn4: f64 = *var_wjuncld_dn4_slot;
        let mut var_wjuncld_dn5: f64 = *var_wjuncld_dn5_slot;
        let mut var_wjuncld_dn6: f64 = *var_wjuncld_dn6_slot;
        let mut var_wjuncld_dn7: f64 = *var_wjuncld_dn7_slot;
        let mut var_wjuncld_dn8: f64 = *var_wjuncld_dn8_slot;
        let mut var_wjuncld_dn9: f64 = *var_wjuncld_dn9_slot;
        let mut var_wjuncld_rv: f64 = *var_wjuncld_rv_slot;
        let mut var_wk_ii: f64 = *var_wk_ii_slot;
        let mut var_wk_ii_dn0: f64 = *var_wk_ii_dn0_slot;
        let mut var_wk_ii_dn10: f64 = *var_wk_ii_dn10_slot;
        let mut var_wk_ii_dn13: f64 = *var_wk_ii_dn13_slot;
        let mut var_wk_ii_dn2: f64 = *var_wk_ii_dn2_slot;
        let mut var_wk_ii_dn4: f64 = *var_wk_ii_dn4_slot;
        let mut var_wk_ii_dn5: f64 = *var_wk_ii_dn5_slot;
        let mut var_wk_ii_dn6: f64 = *var_wk_ii_dn6_slot;
        let mut var_wk_ii_dn7: f64 = *var_wk_ii_dn7_slot;
        let mut var_wk_ii_dn8: f64 = *var_wk_ii_dn8_slot;
        let mut var_wk_ii_dn9: f64 = *var_wk_ii_dn9_slot;
        let mut var_wk_ii_rv: f64 = *var_wk_ii_rv_slot;

        var_isbs2_btm = 0.0;
        var_isbs2_btm_dn0 = 0.0;
        var_isbs2_btm_dn2 = 0.0;
        var_isbs2_btm_dn4 = 0.0;
        var_isbs2_btm_dn5 = 0.0;
        var_isbs2_btm_dn6 = 0.0;
        var_isbs2_btm_dn7 = 0.0;
        var_isbs2_btm_dn8 = 0.0;
        var_isbs2_btm_dn9 = 0.0;
        var_isbs2_btm_dn10 = 0.0;
        var_isbs2_btm_dn13 = 0.0;
        var_isbs2_btm_rv = 0.0;

        var_isbs_sws = 0.0;
        var_isbs_sws_dn0 = 0.0;
        var_isbs_sws_dn2 = 0.0;
        var_isbs_sws_dn4 = 0.0;
        var_isbs_sws_dn5 = 0.0;
        var_isbs_sws_dn6 = 0.0;
        var_isbs_sws_dn7 = 0.0;
        var_isbs_sws_dn8 = 0.0;
        var_isbs_sws_dn9 = 0.0;
        var_isbs_sws_dn10 = 0.0;
        var_isbs_sws_dn13 = 0.0;
        var_isbs_sws_rv = 0.0;

        var_isbs2_sws = 0.0;
        var_isbs2_sws_dn0 = 0.0;
        var_isbs2_sws_dn2 = 0.0;
        var_isbs2_sws_dn4 = 0.0;
        var_isbs2_sws_dn5 = 0.0;
        var_isbs2_sws_dn6 = 0.0;
        var_isbs2_sws_dn7 = 0.0;
        var_isbs2_sws_dn8 = 0.0;
        var_isbs2_sws_dn9 = 0.0;
        var_isbs2_sws_dn10 = 0.0;
        var_isbs2_sws_dn13 = 0.0;
        var_isbs2_sws_rv = 0.0;

        var_isbs_swg = 0.0;
        var_isbs_swg_dn0 = 0.0;
        var_isbs_swg_dn2 = 0.0;
        var_isbs_swg_dn4 = 0.0;
        var_isbs_swg_dn5 = 0.0;
        var_isbs_swg_dn6 = 0.0;
        var_isbs_swg_dn7 = 0.0;
        var_isbs_swg_dn8 = 0.0;
        var_isbs_swg_dn9 = 0.0;
        var_isbs_swg_dn10 = 0.0;
        var_isbs_swg_dn13 = 0.0;
        var_isbs_swg_rv = 0.0;

        var_isbs2_swg = 0.0;
        var_isbs2_swg_dn0 = 0.0;
        var_isbs2_swg_dn2 = 0.0;
        var_isbs2_swg_dn4 = 0.0;
        var_isbs2_swg_dn5 = 0.0;
        var_isbs2_swg_dn6 = 0.0;
        var_isbs2_swg_dn7 = 0.0;
        var_isbs2_swg_dn8 = 0.0;
        var_isbs2_swg_dn9 = 0.0;
        var_isbs2_swg_dn10 = 0.0;
        var_isbs2_swg_dn13 = 0.0;
        var_isbs2_swg_rv = 0.0;

        var_qovd_add = 0.0;
        var_qovd_add_dn0 = 0.0;
        var_qovd_add_dn2 = 0.0;
        var_qovd_add_dn4 = 0.0;
        var_qovd_add_dn5 = 0.0;
        var_qovd_add_dn6 = 0.0;
        var_qovd_add_dn7 = 0.0;
        var_qovd_add_dn8 = 0.0;
        var_qovd_add_dn9 = 0.0;
        var_qovd_add_dn10 = 0.0;
        var_qovd_add_dn13 = 0.0;
        var_qovd_add_rv = 0.0;

        var_qovs_add = 0.0;
        var_qovs_add_dn0 = 0.0;
        var_qovs_add_dn2 = 0.0;
        var_qovs_add_dn4 = 0.0;
        var_qovs_add_dn5 = 0.0;
        var_qovs_add_dn6 = 0.0;
        var_qovs_add_dn7 = 0.0;
        var_qovs_add_dn8 = 0.0;
        var_qovs_add_dn9 = 0.0;
        var_qovs_add_dn10 = 0.0;
        var_qovs_add_dn13 = 0.0;
        var_qovs_add_rv = 0.0;

        var_qbdld_add = 0.0;
        var_qbdld_add_dn0 = 0.0;
        var_qbdld_add_dn2 = 0.0;
        var_qbdld_add_dn4 = 0.0;
        var_qbdld_add_dn5 = 0.0;
        var_qbdld_add_dn6 = 0.0;
        var_qbdld_add_dn7 = 0.0;
        var_qbdld_add_dn8 = 0.0;
        var_qbdld_add_dn9 = 0.0;
        var_qbdld_add_dn10 = 0.0;
        var_qbdld_add_dn13 = 0.0;
        var_qbdld_add_rv = 0.0;

        var_qbsld_add = 0.0;
        var_qbsld_add_dn0 = 0.0;
        var_qbsld_add_dn2 = 0.0;
        var_qbsld_add_dn4 = 0.0;
        var_qbsld_add_dn5 = 0.0;
        var_qbsld_add_dn6 = 0.0;
        var_qbsld_add_dn7 = 0.0;
        var_qbsld_add_dn8 = 0.0;
        var_qbsld_add_dn9 = 0.0;
        var_qbsld_add_dn10 = 0.0;
        var_qbsld_add_dn13 = 0.0;
        var_qbsld_add_rv = 0.0;

        var_wjuncld = 0.0;
        var_wjuncld_dn0 = 0.0;
        var_wjuncld_dn2 = 0.0;
        var_wjuncld_dn4 = 0.0;
        var_wjuncld_dn5 = 0.0;
        var_wjuncld_dn6 = 0.0;
        var_wjuncld_dn7 = 0.0;
        var_wjuncld_dn8 = 0.0;
        var_wjuncld_dn9 = 0.0;
        var_wjuncld_dn10 = 0.0;
        var_wjuncld_dn13 = 0.0;
        var_wjuncld_rv = 0.0;

        var_idspt0 = 0.0;
        var_idspt0_dn0 = 0.0;
        var_idspt0_dn2 = 0.0;
        var_idspt0_dn4 = 0.0;
        var_idspt0_dn5 = 0.0;
        var_idspt0_dn6 = 0.0;
        var_idspt0_dn7 = 0.0;
        var_idspt0_dn8 = 0.0;
        var_idspt0_dn9 = 0.0;
        var_idspt0_dn10 = 0.0;
        var_idspt0_dn13 = 0.0;
        var_idspt0_rv = 0.0;

        var_idspt1 = 0.0;
        var_idspt1_dn0 = 0.0;
        var_idspt1_dn2 = 0.0;
        var_idspt1_dn4 = 0.0;
        var_idspt1_dn5 = 0.0;
        var_idspt1_dn6 = 0.0;
        var_idspt1_dn7 = 0.0;
        var_idspt1_dn8 = 0.0;
        var_idspt1_dn9 = 0.0;
        var_idspt1_dn10 = 0.0;
        var_idspt1_dn13 = 0.0;
        var_idspt1_rv = 0.0;

        var_cox0_func = 0.0;
        var_cox0_func_rv = 0.0;

        var_iwnqs0_a = 0.0;
        var_iwnqs0_a_dn0 = 0.0;
        var_iwnqs0_a_dn2 = 0.0;
        var_iwnqs0_a_dn4 = 0.0;
        var_iwnqs0_a_dn5 = 0.0;
        var_iwnqs0_a_dn6 = 0.0;
        var_iwnqs0_a_dn7 = 0.0;
        var_iwnqs0_a_dn8 = 0.0;
        var_iwnqs0_a_dn9 = 0.0;
        var_iwnqs0_a_dn10 = 0.0;
        var_iwnqs0_a_dn13 = 0.0;
        var_iwnqs0_a_dn17 = 0.0;
        var_iwnqs0_a_rv = 0.0;

        var_inqs0_a = 0.0;
        var_inqs0_a_dn0 = 0.0;
        var_inqs0_a_dn2 = 0.0;
        var_inqs0_a_dn4 = 0.0;
        var_inqs0_a_dn5 = 0.0;
        var_inqs0_a_dn6 = 0.0;
        var_inqs0_a_dn7 = 0.0;
        var_inqs0_a_dn8 = 0.0;
        var_inqs0_a_dn9 = 0.0;
        var_inqs0_a_dn10 = 0.0;
        var_inqs0_a_dn13 = 0.0;
        var_inqs0_a_dn15 = 0.0;
        var_inqs0_a_rv = 0.0;

        var_inqs0_k = 0.0;
        var_inqs0_k_dn0 = 0.0;
        var_inqs0_k_dn2 = 0.0;
        var_inqs0_k_dn4 = 0.0;
        var_inqs0_k_dn5 = 0.0;
        var_inqs0_k_dn6 = 0.0;
        var_inqs0_k_dn7 = 0.0;
        var_inqs0_k_dn8 = 0.0;
        var_inqs0_k_dn9 = 0.0;
        var_inqs0_k_dn10 = 0.0;
        var_inqs0_k_dn13 = 0.0;
        var_inqs0_k_dn16 = 0.0;
        var_inqs0_k_rv = 0.0;

        var_isubibpc = 0.0;
        var_isubibpc_dn0 = 0.0;
        var_isubibpc_dn2 = 0.0;
        var_isubibpc_dn4 = 0.0;
        var_isubibpc_dn5 = 0.0;
        var_isubibpc_dn6 = 0.0;
        var_isubibpc_dn7 = 0.0;
        var_isubibpc_dn8 = 0.0;
        var_isubibpc_dn9 = 0.0;
        var_isubibpc_dn10 = 0.0;
        var_isubibpc_dn13 = 0.0;
        var_isubibpc_rv = 0.0;

        var_lover_func = 0.0;
        var_lover_func_dn0 = 0.0;
        var_lover_func_dn2 = 0.0;
        var_lover_func_dn4 = 0.0;
        var_lover_func_dn5 = 0.0;
        var_lover_func_dn6 = 0.0;
        var_lover_func_dn7 = 0.0;
        var_lover_func_dn8 = 0.0;
        var_lover_func_dn9 = 0.0;
        var_lover_func_dn10 = 0.0;
        var_lover_func_dn13 = 0.0;
        var_lover_func_rv = 0.0;

        var_q_nqs_a = 0.0;
        var_q_nqs_a_dn15 = 0.0;
        var_q_nqs_a_rv = 0.0;

        var_q_nqs_k = 0.0;
        var_q_nqs_k_dn16 = 0.0;
        var_q_nqs_k_rv = 0.0;

        var_w_nqs_a = 0.0;
        var_w_nqs_a_dn17 = 0.0;
        var_w_nqs_a_rv = 0.0;

        var_w_res = 0.0;
        var_w_res_dn0 = 0.0;
        var_w_res_dn2 = 0.0;
        var_w_res_dn4 = 0.0;
        var_w_res_dn5 = 0.0;
        var_w_res_dn6 = 0.0;
        var_w_res_dn7 = 0.0;
        var_w_res_dn8 = 0.0;
        var_w_res_dn9 = 0.0;
        var_w_res_dn10 = 0.0;
        var_w_res_dn13 = 0.0;
        var_w_res_rv = 0.0;

        var_wdep_func = 0.0;
        var_wdep_func_dn0 = 0.0;
        var_wdep_func_dn2 = 0.0;
        var_wdep_func_dn4 = 0.0;
        var_wdep_func_dn5 = 0.0;
        var_wdep_func_dn6 = 0.0;
        var_wdep_func_dn7 = 0.0;
        var_wdep_func_dn8 = 0.0;
        var_wdep_func_dn9 = 0.0;
        var_wdep_func_dn10 = 0.0;
        var_wdep_func_dn13 = 0.0;
        var_wdep_func_rv = 0.0;

        var_wk_ii = 0.0;
        var_wk_ii_dn0 = 0.0;
        var_wk_ii_dn2 = 0.0;
        var_wk_ii_dn4 = 0.0;
        var_wk_ii_dn5 = 0.0;
        var_wk_ii_dn6 = 0.0;
        var_wk_ii_dn7 = 0.0;
        var_wk_ii_dn8 = 0.0;
        var_wk_ii_dn9 = 0.0;
        var_wk_ii_dn10 = 0.0;
        var_wk_ii_dn13 = 0.0;
        var_wk_ii_rv = 0.0;

        let (assign5320_e1936,) = {
    if (p.p40 != 0.0) {
        (0.0,)
    } else {
        (p.p17,)
    }
};
        var_uc_corsrd = assign5320_e1936;
        var_uc_corsrd_rv = 0.0;

        var_uc_xpdv = p.p104;
        var_uc_xpdv_rv = 0.0;

        var_uc_xldld = p.p294;
        var_uc_xldld_rv = 0.0;

        var_uc_scp22 = p.p222;
        var_uc_scp22_rv = 0.0;

        var_uc_rdrcx = p.p420;
        var_uc_rdrcx_rv = 0.0;

        var_mfactor = 1.0;
        var_mfactor_rv = 0.0;

        let assign5480_e1979: f64 = if var_uc_scp22 < 0.0 { 1.0 } else { 0.0 };
        var_guard8 = assign5480_e1979;
        var_guard8_rv = 0.0;

        let (assign5490_e1983,) = {
    if (var_guard8 != 0.0) {
        (0.0,)
    } else {
        (var_uc_scp22,)
    }
};
        var_uc_scp22 = assign5490_e1983;
        var_uc_scp22_rv = 0.0;

        let assign5500_e1986: f64 = if var_uc_scp22 > 0.0 { 1.0 } else { 0.0 };
        var_guard9 = assign5500_e1986;
        var_guard9_rv = 0.0;

        let (assign5510_e1990,) = {
    if (var_guard9 != 0.0) {
        (0.0,)
    } else {
        (var_uc_scp22,)
    }
};
        var_uc_scp22 = assign5510_e1990;
        var_uc_scp22_rv = 0.0;

        let assign5530_e1998: f64 = if var_uc_xldld < 0.0 { 1.0 } else { 0.0 };
        var_guard11 = assign5530_e1998;
        var_guard11_rv = 0.0;

        let (assign5540_e2002,) = {
    if (var_guard11 != 0.0) {
        (0.0,)
    } else {
        (var_uc_xldld,)
    }
};
        var_uc_xldld = assign5540_e2002;
        var_uc_xldld_rv = 0.0;

        let assign5570_e2015: f64 = if var_uc_rdrcx < 0.0 { 1.0 } else { 0.0 };
        var_guard14 = assign5570_e2015;
        var_guard14_rv = 0.0;

        let (assign5580_e2019,) = {
    if (var_guard14 != 0.0) {
        (0.0,)
    } else {
        (var_uc_rdrcx,)
    }
};
        var_uc_rdrcx = assign5580_e2019;
        var_uc_rdrcx_rv = 0.0;

        let assign5590_e2022: f64 = if var_uc_rdrcx > 1.0 { 1.0 } else { 0.0 };
        var_guard15 = assign5590_e2022;
        var_guard15_rv = 0.0;

        let (assign5600_e2026,) = {
    if (var_guard15 != 0.0) {
        (1.0,)
    } else {
        (var_uc_rdrcx,)
    }
};
        var_uc_rdrcx = assign5600_e2026;
        var_uc_rdrcx_rv = 0.0;

        var_uc_ndepm = p.p340;
        var_uc_ndepm_dn0 = 0.0;
        var_uc_ndepm_dn2 = 0.0;
        var_uc_ndepm_dn4 = 0.0;
        var_uc_ndepm_dn5 = 0.0;
        var_uc_ndepm_dn6 = 0.0;
        var_uc_ndepm_dn7 = 0.0;
        var_uc_ndepm_dn8 = 0.0;
        var_uc_ndepm_dn9 = 0.0;
        var_uc_ndepm_dn10 = 0.0;
        var_uc_ndepm_dn13 = 0.0;
        var_uc_ndepm_rv = 0.0;

        var_uc_depthn = p.p343;
        var_uc_depthn_dn0 = 0.0;
        var_uc_depthn_dn2 = 0.0;
        var_uc_depthn_dn4 = 0.0;
        var_uc_depthn_dn5 = 0.0;
        var_uc_depthn_dn6 = 0.0;
        var_uc_depthn_dn7 = 0.0;
        var_uc_depthn_dn8 = 0.0;
        var_uc_depthn_dn9 = 0.0;
        var_uc_depthn_dn10 = 0.0;
        var_uc_depthn_dn13 = 0.0;
        var_uc_depthn_rv = 0.0;

        var_uc_codep = p.p42;
        var_uc_codep_rv = 0.0;

        var_uc_depmueback0 = p.p354;
        var_uc_depmueback0_dn0 = 0.0;
        var_uc_depmueback0_dn2 = 0.0;
        var_uc_depmueback0_dn4 = 0.0;
        var_uc_depmueback0_dn5 = 0.0;
        var_uc_depmueback0_dn6 = 0.0;
        var_uc_depmueback0_dn7 = 0.0;
        var_uc_depmueback0_dn8 = 0.0;
        var_uc_depmueback0_dn9 = 0.0;
        var_uc_depmueback0_dn10 = 0.0;
        var_uc_depmueback0_dn13 = 0.0;
        var_uc_depmueback0_rv = 0.0;

        var_uc_depmueback1 = p.p355;
        var_uc_depmueback1_dn0 = 0.0;
        var_uc_depmueback1_dn2 = 0.0;
        var_uc_depmueback1_dn4 = 0.0;
        var_uc_depmueback1_dn5 = 0.0;
        var_uc_depmueback1_dn6 = 0.0;
        var_uc_depmueback1_dn7 = 0.0;
        var_uc_depmueback1_dn8 = 0.0;
        var_uc_depmueback1_dn9 = 0.0;
        var_uc_depmueback1_dn10 = 0.0;
        var_uc_depmueback1_dn13 = 0.0;
        var_uc_depmueback1_rv = 0.0;

        var_uc_depmue0 = p.p346;
        var_uc_depmue0_dn0 = 0.0;
        var_uc_depmue0_dn2 = 0.0;
        var_uc_depmue0_dn4 = 0.0;
        var_uc_depmue0_dn5 = 0.0;
        var_uc_depmue0_dn6 = 0.0;
        var_uc_depmue0_dn7 = 0.0;
        var_uc_depmue0_dn8 = 0.0;
        var_uc_depmue0_dn9 = 0.0;
        var_uc_depmue0_dn10 = 0.0;
        var_uc_depmue0_dn13 = 0.0;
        var_uc_depmue0_rv = 0.0;

        var_uc_depmue1 = p.p349;
        var_uc_depmue1_dn0 = 0.0;
        var_uc_depmue1_dn2 = 0.0;
        var_uc_depmue1_dn4 = 0.0;
        var_uc_depmue1_dn5 = 0.0;
        var_uc_depmue1_dn6 = 0.0;
        var_uc_depmue1_dn7 = 0.0;
        var_uc_depmue1_dn8 = 0.0;
        var_uc_depmue1_dn9 = 0.0;
        var_uc_depmue1_dn10 = 0.0;
        var_uc_depmue1_dn13 = 0.0;
        var_uc_depmue1_rv = 0.0;

        var_uc_depmue2 = p.p352;
        var_uc_depmue2_dn0 = 0.0;
        var_uc_depmue2_dn2 = 0.0;
        var_uc_depmue2_dn4 = 0.0;
        var_uc_depmue2_dn5 = 0.0;
        var_uc_depmue2_dn6 = 0.0;
        var_uc_depmue2_dn7 = 0.0;
        var_uc_depmue2_dn8 = 0.0;
        var_uc_depmue2_dn9 = 0.0;
        var_uc_depmue2_dn10 = 0.0;
        var_uc_depmue2_dn13 = 0.0;
        var_uc_depmue2_rv = 0.0;

        var_uc_depleak = p.p360;
        var_uc_depleak_dn0 = 0.0;
        var_uc_depleak_dn2 = 0.0;
        var_uc_depleak_dn4 = 0.0;
        var_uc_depleak_dn5 = 0.0;
        var_uc_depleak_dn6 = 0.0;
        var_uc_depleak_dn7 = 0.0;
        var_uc_depleak_dn8 = 0.0;
        var_uc_depleak_dn9 = 0.0;
        var_uc_depleak_dn10 = 0.0;
        var_uc_depleak_dn13 = 0.0;
        var_uc_depleak_rv = 0.0;

        var_uc_depvmax = p.p367;
        var_uc_depvmax_dn0 = 0.0;
        var_uc_depvmax_dn2 = 0.0;
        var_uc_depvmax_dn4 = 0.0;
        var_uc_depvmax_dn5 = 0.0;
        var_uc_depvmax_dn6 = 0.0;
        var_uc_depvmax_dn7 = 0.0;
        var_uc_depvmax_dn8 = 0.0;
        var_uc_depvmax_dn9 = 0.0;
        var_uc_depvmax_dn10 = 0.0;
        var_uc_depvmax_dn13 = 0.0;
        var_uc_depvmax_rv = 0.0;

        var_uc_depwlp = p.p364;
        var_uc_depwlp_dn0 = 0.0;
        var_uc_depwlp_dn2 = 0.0;
        var_uc_depwlp_dn4 = 0.0;
        var_uc_depwlp_dn5 = 0.0;
        var_uc_depwlp_dn6 = 0.0;
        var_uc_depwlp_dn7 = 0.0;
        var_uc_depwlp_dn8 = 0.0;
        var_uc_depwlp_dn9 = 0.0;
        var_uc_depwlp_dn10 = 0.0;
        var_uc_depwlp_dn13 = 0.0;
        var_uc_depwlp_rv = 0.0;

        var_uc_depmueph1 = p.p377;
        var_uc_depmueph1_rv = 0.0;

        *var_cox0_func_slot = var_cox0_func;
        *var_cox0_func_rv_slot = var_cox0_func_rv;
        *var_guard11_slot = var_guard11;
        *var_guard11_rv_slot = var_guard11_rv;
        *var_guard14_slot = var_guard14;
        *var_guard14_rv_slot = var_guard14_rv;
        *var_guard15_slot = var_guard15;
        *var_guard15_rv_slot = var_guard15_rv;
        *var_guard8_slot = var_guard8;
        *var_guard8_rv_slot = var_guard8_rv;
        *var_guard9_slot = var_guard9;
        *var_guard9_rv_slot = var_guard9_rv;
        *var_idspt0_slot = var_idspt0;
        *var_idspt0_dn0_slot = var_idspt0_dn0;
        *var_idspt0_dn10_slot = var_idspt0_dn10;
        *var_idspt0_dn13_slot = var_idspt0_dn13;
        *var_idspt0_dn2_slot = var_idspt0_dn2;
        *var_idspt0_dn4_slot = var_idspt0_dn4;
        *var_idspt0_dn5_slot = var_idspt0_dn5;
        *var_idspt0_dn6_slot = var_idspt0_dn6;
        *var_idspt0_dn7_slot = var_idspt0_dn7;
        *var_idspt0_dn8_slot = var_idspt0_dn8;
        *var_idspt0_dn9_slot = var_idspt0_dn9;
        *var_idspt0_rv_slot = var_idspt0_rv;
        *var_idspt1_slot = var_idspt1;
        *var_idspt1_dn0_slot = var_idspt1_dn0;
        *var_idspt1_dn10_slot = var_idspt1_dn10;
        *var_idspt1_dn13_slot = var_idspt1_dn13;
        *var_idspt1_dn2_slot = var_idspt1_dn2;
        *var_idspt1_dn4_slot = var_idspt1_dn4;
        *var_idspt1_dn5_slot = var_idspt1_dn5;
        *var_idspt1_dn6_slot = var_idspt1_dn6;
        *var_idspt1_dn7_slot = var_idspt1_dn7;
        *var_idspt1_dn8_slot = var_idspt1_dn8;
        *var_idspt1_dn9_slot = var_idspt1_dn9;
        *var_idspt1_rv_slot = var_idspt1_rv;
        *var_inqs0_a_slot = var_inqs0_a;
        *var_inqs0_a_dn0_slot = var_inqs0_a_dn0;
        *var_inqs0_a_dn10_slot = var_inqs0_a_dn10;
        *var_inqs0_a_dn13_slot = var_inqs0_a_dn13;
        *var_inqs0_a_dn15_slot = var_inqs0_a_dn15;
        *var_inqs0_a_dn2_slot = var_inqs0_a_dn2;
        *var_inqs0_a_dn4_slot = var_inqs0_a_dn4;
        *var_inqs0_a_dn5_slot = var_inqs0_a_dn5;
        *var_inqs0_a_dn6_slot = var_inqs0_a_dn6;
        *var_inqs0_a_dn7_slot = var_inqs0_a_dn7;
        *var_inqs0_a_dn8_slot = var_inqs0_a_dn8;
        *var_inqs0_a_dn9_slot = var_inqs0_a_dn9;
        *var_inqs0_a_rv_slot = var_inqs0_a_rv;
        *var_inqs0_k_slot = var_inqs0_k;
        *var_inqs0_k_dn0_slot = var_inqs0_k_dn0;
        *var_inqs0_k_dn10_slot = var_inqs0_k_dn10;
        *var_inqs0_k_dn13_slot = var_inqs0_k_dn13;
        *var_inqs0_k_dn16_slot = var_inqs0_k_dn16;
        *var_inqs0_k_dn2_slot = var_inqs0_k_dn2;
        *var_inqs0_k_dn4_slot = var_inqs0_k_dn4;
        *var_inqs0_k_dn5_slot = var_inqs0_k_dn5;
        *var_inqs0_k_dn6_slot = var_inqs0_k_dn6;
        *var_inqs0_k_dn7_slot = var_inqs0_k_dn7;
        *var_inqs0_k_dn8_slot = var_inqs0_k_dn8;
        *var_inqs0_k_dn9_slot = var_inqs0_k_dn9;
        *var_inqs0_k_rv_slot = var_inqs0_k_rv;
        *var_isbs2_btm_slot = var_isbs2_btm;
        *var_isbs2_btm_dn0_slot = var_isbs2_btm_dn0;
        *var_isbs2_btm_dn10_slot = var_isbs2_btm_dn10;
        *var_isbs2_btm_dn13_slot = var_isbs2_btm_dn13;
        *var_isbs2_btm_dn2_slot = var_isbs2_btm_dn2;
        *var_isbs2_btm_dn4_slot = var_isbs2_btm_dn4;
        *var_isbs2_btm_dn5_slot = var_isbs2_btm_dn5;
        *var_isbs2_btm_dn6_slot = var_isbs2_btm_dn6;
        *var_isbs2_btm_dn7_slot = var_isbs2_btm_dn7;
        *var_isbs2_btm_dn8_slot = var_isbs2_btm_dn8;
        *var_isbs2_btm_dn9_slot = var_isbs2_btm_dn9;
        *var_isbs2_btm_rv_slot = var_isbs2_btm_rv;
        *var_isbs2_swg_slot = var_isbs2_swg;
        *var_isbs2_swg_dn0_slot = var_isbs2_swg_dn0;
        *var_isbs2_swg_dn10_slot = var_isbs2_swg_dn10;
        *var_isbs2_swg_dn13_slot = var_isbs2_swg_dn13;
        *var_isbs2_swg_dn2_slot = var_isbs2_swg_dn2;
        *var_isbs2_swg_dn4_slot = var_isbs2_swg_dn4;
        *var_isbs2_swg_dn5_slot = var_isbs2_swg_dn5;
        *var_isbs2_swg_dn6_slot = var_isbs2_swg_dn6;
        *var_isbs2_swg_dn7_slot = var_isbs2_swg_dn7;
        *var_isbs2_swg_dn8_slot = var_isbs2_swg_dn8;
        *var_isbs2_swg_dn9_slot = var_isbs2_swg_dn9;
        *var_isbs2_swg_rv_slot = var_isbs2_swg_rv;
        *var_isbs2_sws_slot = var_isbs2_sws;
        *var_isbs2_sws_dn0_slot = var_isbs2_sws_dn0;
        *var_isbs2_sws_dn10_slot = var_isbs2_sws_dn10;
        *var_isbs2_sws_dn13_slot = var_isbs2_sws_dn13;
        *var_isbs2_sws_dn2_slot = var_isbs2_sws_dn2;
        *var_isbs2_sws_dn4_slot = var_isbs2_sws_dn4;
        *var_isbs2_sws_dn5_slot = var_isbs2_sws_dn5;
        *var_isbs2_sws_dn6_slot = var_isbs2_sws_dn6;
        *var_isbs2_sws_dn7_slot = var_isbs2_sws_dn7;
        *var_isbs2_sws_dn8_slot = var_isbs2_sws_dn8;
        *var_isbs2_sws_dn9_slot = var_isbs2_sws_dn9;
        *var_isbs2_sws_rv_slot = var_isbs2_sws_rv;
        *var_isbs_swg_slot = var_isbs_swg;
        *var_isbs_swg_dn0_slot = var_isbs_swg_dn0;
        *var_isbs_swg_dn10_slot = var_isbs_swg_dn10;
        *var_isbs_swg_dn13_slot = var_isbs_swg_dn13;
        *var_isbs_swg_dn2_slot = var_isbs_swg_dn2;
        *var_isbs_swg_dn4_slot = var_isbs_swg_dn4;
        *var_isbs_swg_dn5_slot = var_isbs_swg_dn5;
        *var_isbs_swg_dn6_slot = var_isbs_swg_dn6;
        *var_isbs_swg_dn7_slot = var_isbs_swg_dn7;
        *var_isbs_swg_dn8_slot = var_isbs_swg_dn8;
        *var_isbs_swg_dn9_slot = var_isbs_swg_dn9;
        *var_isbs_swg_rv_slot = var_isbs_swg_rv;
        *var_isbs_sws_slot = var_isbs_sws;
        *var_isbs_sws_dn0_slot = var_isbs_sws_dn0;
        *var_isbs_sws_dn10_slot = var_isbs_sws_dn10;
        *var_isbs_sws_dn13_slot = var_isbs_sws_dn13;
        *var_isbs_sws_dn2_slot = var_isbs_sws_dn2;
        *var_isbs_sws_dn4_slot = var_isbs_sws_dn4;
        *var_isbs_sws_dn5_slot = var_isbs_sws_dn5;
        *var_isbs_sws_dn6_slot = var_isbs_sws_dn6;
        *var_isbs_sws_dn7_slot = var_isbs_sws_dn7;
        *var_isbs_sws_dn8_slot = var_isbs_sws_dn8;
        *var_isbs_sws_dn9_slot = var_isbs_sws_dn9;
        *var_isbs_sws_rv_slot = var_isbs_sws_rv;
        *var_isubibpc_slot = var_isubibpc;
        *var_isubibpc_dn0_slot = var_isubibpc_dn0;
        *var_isubibpc_dn10_slot = var_isubibpc_dn10;
        *var_isubibpc_dn13_slot = var_isubibpc_dn13;
        *var_isubibpc_dn2_slot = var_isubibpc_dn2;
        *var_isubibpc_dn4_slot = var_isubibpc_dn4;
        *var_isubibpc_dn5_slot = var_isubibpc_dn5;
        *var_isubibpc_dn6_slot = var_isubibpc_dn6;
        *var_isubibpc_dn7_slot = var_isubibpc_dn7;
        *var_isubibpc_dn8_slot = var_isubibpc_dn8;
        *var_isubibpc_dn9_slot = var_isubibpc_dn9;
        *var_isubibpc_rv_slot = var_isubibpc_rv;
        *var_iwnqs0_a_slot = var_iwnqs0_a;
        *var_iwnqs0_a_dn0_slot = var_iwnqs0_a_dn0;
        *var_iwnqs0_a_dn10_slot = var_iwnqs0_a_dn10;
        *var_iwnqs0_a_dn13_slot = var_iwnqs0_a_dn13;
        *var_iwnqs0_a_dn17_slot = var_iwnqs0_a_dn17;
        *var_iwnqs0_a_dn2_slot = var_iwnqs0_a_dn2;
        *var_iwnqs0_a_dn4_slot = var_iwnqs0_a_dn4;
        *var_iwnqs0_a_dn5_slot = var_iwnqs0_a_dn5;
        *var_iwnqs0_a_dn6_slot = var_iwnqs0_a_dn6;
        *var_iwnqs0_a_dn7_slot = var_iwnqs0_a_dn7;
        *var_iwnqs0_a_dn8_slot = var_iwnqs0_a_dn8;
        *var_iwnqs0_a_dn9_slot = var_iwnqs0_a_dn9;
        *var_iwnqs0_a_rv_slot = var_iwnqs0_a_rv;
        *var_lover_func_slot = var_lover_func;
        *var_lover_func_dn0_slot = var_lover_func_dn0;
        *var_lover_func_dn10_slot = var_lover_func_dn10;
        *var_lover_func_dn13_slot = var_lover_func_dn13;
        *var_lover_func_dn2_slot = var_lover_func_dn2;
        *var_lover_func_dn4_slot = var_lover_func_dn4;
        *var_lover_func_dn5_slot = var_lover_func_dn5;
        *var_lover_func_dn6_slot = var_lover_func_dn6;
        *var_lover_func_dn7_slot = var_lover_func_dn7;
        *var_lover_func_dn8_slot = var_lover_func_dn8;
        *var_lover_func_dn9_slot = var_lover_func_dn9;
        *var_lover_func_rv_slot = var_lover_func_rv;
        *var_mfactor_slot = var_mfactor;
        *var_mfactor_rv_slot = var_mfactor_rv;
        *var_q_nqs_a_slot = var_q_nqs_a;
        *var_q_nqs_a_dn15_slot = var_q_nqs_a_dn15;
        *var_q_nqs_a_rv_slot = var_q_nqs_a_rv;
        *var_q_nqs_k_slot = var_q_nqs_k;
        *var_q_nqs_k_dn16_slot = var_q_nqs_k_dn16;
        *var_q_nqs_k_rv_slot = var_q_nqs_k_rv;
        *var_qbdld_add_slot = var_qbdld_add;
        *var_qbdld_add_dn0_slot = var_qbdld_add_dn0;
        *var_qbdld_add_dn10_slot = var_qbdld_add_dn10;
        *var_qbdld_add_dn13_slot = var_qbdld_add_dn13;
        *var_qbdld_add_dn2_slot = var_qbdld_add_dn2;
        *var_qbdld_add_dn4_slot = var_qbdld_add_dn4;
        *var_qbdld_add_dn5_slot = var_qbdld_add_dn5;
        *var_qbdld_add_dn6_slot = var_qbdld_add_dn6;
        *var_qbdld_add_dn7_slot = var_qbdld_add_dn7;
        *var_qbdld_add_dn8_slot = var_qbdld_add_dn8;
        *var_qbdld_add_dn9_slot = var_qbdld_add_dn9;
        *var_qbdld_add_rv_slot = var_qbdld_add_rv;
        *var_qbsld_add_slot = var_qbsld_add;
        *var_qbsld_add_dn0_slot = var_qbsld_add_dn0;
        *var_qbsld_add_dn10_slot = var_qbsld_add_dn10;
        *var_qbsld_add_dn13_slot = var_qbsld_add_dn13;
        *var_qbsld_add_dn2_slot = var_qbsld_add_dn2;
        *var_qbsld_add_dn4_slot = var_qbsld_add_dn4;
        *var_qbsld_add_dn5_slot = var_qbsld_add_dn5;
        *var_qbsld_add_dn6_slot = var_qbsld_add_dn6;
        *var_qbsld_add_dn7_slot = var_qbsld_add_dn7;
        *var_qbsld_add_dn8_slot = var_qbsld_add_dn8;
        *var_qbsld_add_dn9_slot = var_qbsld_add_dn9;
        *var_qbsld_add_rv_slot = var_qbsld_add_rv;
        *var_qovd_add_slot = var_qovd_add;
        *var_qovd_add_dn0_slot = var_qovd_add_dn0;
        *var_qovd_add_dn10_slot = var_qovd_add_dn10;
        *var_qovd_add_dn13_slot = var_qovd_add_dn13;
        *var_qovd_add_dn2_slot = var_qovd_add_dn2;
        *var_qovd_add_dn4_slot = var_qovd_add_dn4;
        *var_qovd_add_dn5_slot = var_qovd_add_dn5;
        *var_qovd_add_dn6_slot = var_qovd_add_dn6;
        *var_qovd_add_dn7_slot = var_qovd_add_dn7;
        *var_qovd_add_dn8_slot = var_qovd_add_dn8;
        *var_qovd_add_dn9_slot = var_qovd_add_dn9;
        *var_qovd_add_rv_slot = var_qovd_add_rv;
        *var_qovs_add_slot = var_qovs_add;
        *var_qovs_add_dn0_slot = var_qovs_add_dn0;
        *var_qovs_add_dn10_slot = var_qovs_add_dn10;
        *var_qovs_add_dn13_slot = var_qovs_add_dn13;
        *var_qovs_add_dn2_slot = var_qovs_add_dn2;
        *var_qovs_add_dn4_slot = var_qovs_add_dn4;
        *var_qovs_add_dn5_slot = var_qovs_add_dn5;
        *var_qovs_add_dn6_slot = var_qovs_add_dn6;
        *var_qovs_add_dn7_slot = var_qovs_add_dn7;
        *var_qovs_add_dn8_slot = var_qovs_add_dn8;
        *var_qovs_add_dn9_slot = var_qovs_add_dn9;
        *var_qovs_add_rv_slot = var_qovs_add_rv;
        *var_uc_codep_slot = var_uc_codep;
        *var_uc_codep_rv_slot = var_uc_codep_rv;
        *var_uc_corsrd_slot = var_uc_corsrd;
        *var_uc_corsrd_rv_slot = var_uc_corsrd_rv;
        *var_uc_depleak_slot = var_uc_depleak;
        *var_uc_depleak_dn0_slot = var_uc_depleak_dn0;
        *var_uc_depleak_dn10_slot = var_uc_depleak_dn10;
        *var_uc_depleak_dn13_slot = var_uc_depleak_dn13;
        *var_uc_depleak_dn2_slot = var_uc_depleak_dn2;
        *var_uc_depleak_dn4_slot = var_uc_depleak_dn4;
        *var_uc_depleak_dn5_slot = var_uc_depleak_dn5;
        *var_uc_depleak_dn6_slot = var_uc_depleak_dn6;
        *var_uc_depleak_dn7_slot = var_uc_depleak_dn7;
        *var_uc_depleak_dn8_slot = var_uc_depleak_dn8;
        *var_uc_depleak_dn9_slot = var_uc_depleak_dn9;
        *var_uc_depleak_rv_slot = var_uc_depleak_rv;
        *var_uc_depmue0_slot = var_uc_depmue0;
        *var_uc_depmue0_dn0_slot = var_uc_depmue0_dn0;
        *var_uc_depmue0_dn10_slot = var_uc_depmue0_dn10;
        *var_uc_depmue0_dn13_slot = var_uc_depmue0_dn13;
        *var_uc_depmue0_dn2_slot = var_uc_depmue0_dn2;
        *var_uc_depmue0_dn4_slot = var_uc_depmue0_dn4;
        *var_uc_depmue0_dn5_slot = var_uc_depmue0_dn5;
        *var_uc_depmue0_dn6_slot = var_uc_depmue0_dn6;
        *var_uc_depmue0_dn7_slot = var_uc_depmue0_dn7;
        *var_uc_depmue0_dn8_slot = var_uc_depmue0_dn8;
        *var_uc_depmue0_dn9_slot = var_uc_depmue0_dn9;
        *var_uc_depmue0_rv_slot = var_uc_depmue0_rv;
        *var_uc_depmue1_slot = var_uc_depmue1;
        *var_uc_depmue1_dn0_slot = var_uc_depmue1_dn0;
        *var_uc_depmue1_dn10_slot = var_uc_depmue1_dn10;
        *var_uc_depmue1_dn13_slot = var_uc_depmue1_dn13;
        *var_uc_depmue1_dn2_slot = var_uc_depmue1_dn2;
        *var_uc_depmue1_dn4_slot = var_uc_depmue1_dn4;
        *var_uc_depmue1_dn5_slot = var_uc_depmue1_dn5;
        *var_uc_depmue1_dn6_slot = var_uc_depmue1_dn6;
        *var_uc_depmue1_dn7_slot = var_uc_depmue1_dn7;
        *var_uc_depmue1_dn8_slot = var_uc_depmue1_dn8;
        *var_uc_depmue1_dn9_slot = var_uc_depmue1_dn9;
        *var_uc_depmue1_rv_slot = var_uc_depmue1_rv;
        *var_uc_depmue2_slot = var_uc_depmue2;
        *var_uc_depmue2_dn0_slot = var_uc_depmue2_dn0;
        *var_uc_depmue2_dn10_slot = var_uc_depmue2_dn10;
        *var_uc_depmue2_dn13_slot = var_uc_depmue2_dn13;
        *var_uc_depmue2_dn2_slot = var_uc_depmue2_dn2;
        *var_uc_depmue2_dn4_slot = var_uc_depmue2_dn4;
        *var_uc_depmue2_dn5_slot = var_uc_depmue2_dn5;
        *var_uc_depmue2_dn6_slot = var_uc_depmue2_dn6;
        *var_uc_depmue2_dn7_slot = var_uc_depmue2_dn7;
        *var_uc_depmue2_dn8_slot = var_uc_depmue2_dn8;
        *var_uc_depmue2_dn9_slot = var_uc_depmue2_dn9;
        *var_uc_depmue2_rv_slot = var_uc_depmue2_rv;
        *var_uc_depmueback0_slot = var_uc_depmueback0;
        *var_uc_depmueback0_dn0_slot = var_uc_depmueback0_dn0;
        *var_uc_depmueback0_dn10_slot = var_uc_depmueback0_dn10;
        *var_uc_depmueback0_dn13_slot = var_uc_depmueback0_dn13;
        *var_uc_depmueback0_dn2_slot = var_uc_depmueback0_dn2;
        *var_uc_depmueback0_dn4_slot = var_uc_depmueback0_dn4;
        *var_uc_depmueback0_dn5_slot = var_uc_depmueback0_dn5;
        *var_uc_depmueback0_dn6_slot = var_uc_depmueback0_dn6;
        *var_uc_depmueback0_dn7_slot = var_uc_depmueback0_dn7;
        *var_uc_depmueback0_dn8_slot = var_uc_depmueback0_dn8;
        *var_uc_depmueback0_dn9_slot = var_uc_depmueback0_dn9;
        *var_uc_depmueback0_rv_slot = var_uc_depmueback0_rv;
        *var_uc_depmueback1_slot = var_uc_depmueback1;
        *var_uc_depmueback1_dn0_slot = var_uc_depmueback1_dn0;
        *var_uc_depmueback1_dn10_slot = var_uc_depmueback1_dn10;
        *var_uc_depmueback1_dn13_slot = var_uc_depmueback1_dn13;
        *var_uc_depmueback1_dn2_slot = var_uc_depmueback1_dn2;
        *var_uc_depmueback1_dn4_slot = var_uc_depmueback1_dn4;
        *var_uc_depmueback1_dn5_slot = var_uc_depmueback1_dn5;
        *var_uc_depmueback1_dn6_slot = var_uc_depmueback1_dn6;
        *var_uc_depmueback1_dn7_slot = var_uc_depmueback1_dn7;
        *var_uc_depmueback1_dn8_slot = var_uc_depmueback1_dn8;
        *var_uc_depmueback1_dn9_slot = var_uc_depmueback1_dn9;
        *var_uc_depmueback1_rv_slot = var_uc_depmueback1_rv;
        *var_uc_depmueph1_slot = var_uc_depmueph1;
        *var_uc_depmueph1_rv_slot = var_uc_depmueph1_rv;
        *var_uc_depthn_slot = var_uc_depthn;
        *var_uc_depthn_dn0_slot = var_uc_depthn_dn0;
        *var_uc_depthn_dn10_slot = var_uc_depthn_dn10;
        *var_uc_depthn_dn13_slot = var_uc_depthn_dn13;
        *var_uc_depthn_dn2_slot = var_uc_depthn_dn2;
        *var_uc_depthn_dn4_slot = var_uc_depthn_dn4;
        *var_uc_depthn_dn5_slot = var_uc_depthn_dn5;
        *var_uc_depthn_dn6_slot = var_uc_depthn_dn6;
        *var_uc_depthn_dn7_slot = var_uc_depthn_dn7;
        *var_uc_depthn_dn8_slot = var_uc_depthn_dn8;
        *var_uc_depthn_dn9_slot = var_uc_depthn_dn9;
        *var_uc_depthn_rv_slot = var_uc_depthn_rv;
        *var_uc_depvmax_slot = var_uc_depvmax;
        *var_uc_depvmax_dn0_slot = var_uc_depvmax_dn0;
        *var_uc_depvmax_dn10_slot = var_uc_depvmax_dn10;
        *var_uc_depvmax_dn13_slot = var_uc_depvmax_dn13;
        *var_uc_depvmax_dn2_slot = var_uc_depvmax_dn2;
        *var_uc_depvmax_dn4_slot = var_uc_depvmax_dn4;
        *var_uc_depvmax_dn5_slot = var_uc_depvmax_dn5;
        *var_uc_depvmax_dn6_slot = var_uc_depvmax_dn6;
        *var_uc_depvmax_dn7_slot = var_uc_depvmax_dn7;
        *var_uc_depvmax_dn8_slot = var_uc_depvmax_dn8;
        *var_uc_depvmax_dn9_slot = var_uc_depvmax_dn9;
        *var_uc_depvmax_rv_slot = var_uc_depvmax_rv;
        *var_uc_depwlp_slot = var_uc_depwlp;
        *var_uc_depwlp_dn0_slot = var_uc_depwlp_dn0;
        *var_uc_depwlp_dn10_slot = var_uc_depwlp_dn10;
        *var_uc_depwlp_dn13_slot = var_uc_depwlp_dn13;
        *var_uc_depwlp_dn2_slot = var_uc_depwlp_dn2;
        *var_uc_depwlp_dn4_slot = var_uc_depwlp_dn4;
        *var_uc_depwlp_dn5_slot = var_uc_depwlp_dn5;
        *var_uc_depwlp_dn6_slot = var_uc_depwlp_dn6;
        *var_uc_depwlp_dn7_slot = var_uc_depwlp_dn7;
        *var_uc_depwlp_dn8_slot = var_uc_depwlp_dn8;
        *var_uc_depwlp_dn9_slot = var_uc_depwlp_dn9;
        *var_uc_depwlp_rv_slot = var_uc_depwlp_rv;
        *var_uc_ndepm_slot = var_uc_ndepm;
        *var_uc_ndepm_dn0_slot = var_uc_ndepm_dn0;
        *var_uc_ndepm_dn10_slot = var_uc_ndepm_dn10;
        *var_uc_ndepm_dn13_slot = var_uc_ndepm_dn13;
        *var_uc_ndepm_dn2_slot = var_uc_ndepm_dn2;
        *var_uc_ndepm_dn4_slot = var_uc_ndepm_dn4;
        *var_uc_ndepm_dn5_slot = var_uc_ndepm_dn5;
        *var_uc_ndepm_dn6_slot = var_uc_ndepm_dn6;
        *var_uc_ndepm_dn7_slot = var_uc_ndepm_dn7;
        *var_uc_ndepm_dn8_slot = var_uc_ndepm_dn8;
        *var_uc_ndepm_dn9_slot = var_uc_ndepm_dn9;
        *var_uc_ndepm_rv_slot = var_uc_ndepm_rv;
        *var_uc_rdrcx_slot = var_uc_rdrcx;
        *var_uc_rdrcx_rv_slot = var_uc_rdrcx_rv;
        *var_uc_scp22_slot = var_uc_scp22;
        *var_uc_scp22_rv_slot = var_uc_scp22_rv;
        *var_uc_xldld_slot = var_uc_xldld;
        *var_uc_xldld_rv_slot = var_uc_xldld_rv;
        *var_uc_xpdv_slot = var_uc_xpdv;
        *var_uc_xpdv_rv_slot = var_uc_xpdv_rv;
        *var_w_nqs_a_slot = var_w_nqs_a;
        *var_w_nqs_a_dn17_slot = var_w_nqs_a_dn17;
        *var_w_nqs_a_rv_slot = var_w_nqs_a_rv;
        *var_w_res_slot = var_w_res;
        *var_w_res_dn0_slot = var_w_res_dn0;
        *var_w_res_dn10_slot = var_w_res_dn10;
        *var_w_res_dn13_slot = var_w_res_dn13;
        *var_w_res_dn2_slot = var_w_res_dn2;
        *var_w_res_dn4_slot = var_w_res_dn4;
        *var_w_res_dn5_slot = var_w_res_dn5;
        *var_w_res_dn6_slot = var_w_res_dn6;
        *var_w_res_dn7_slot = var_w_res_dn7;
        *var_w_res_dn8_slot = var_w_res_dn8;
        *var_w_res_dn9_slot = var_w_res_dn9;
        *var_w_res_rv_slot = var_w_res_rv;
        *var_wdep_func_slot = var_wdep_func;
        *var_wdep_func_dn0_slot = var_wdep_func_dn0;
        *var_wdep_func_dn10_slot = var_wdep_func_dn10;
        *var_wdep_func_dn13_slot = var_wdep_func_dn13;
        *var_wdep_func_dn2_slot = var_wdep_func_dn2;
        *var_wdep_func_dn4_slot = var_wdep_func_dn4;
        *var_wdep_func_dn5_slot = var_wdep_func_dn5;
        *var_wdep_func_dn6_slot = var_wdep_func_dn6;
        *var_wdep_func_dn7_slot = var_wdep_func_dn7;
        *var_wdep_func_dn8_slot = var_wdep_func_dn8;
        *var_wdep_func_dn9_slot = var_wdep_func_dn9;
        *var_wdep_func_rv_slot = var_wdep_func_rv;
        *var_wjuncld_slot = var_wjuncld;
        *var_wjuncld_dn0_slot = var_wjuncld_dn0;
        *var_wjuncld_dn10_slot = var_wjuncld_dn10;
        *var_wjuncld_dn13_slot = var_wjuncld_dn13;
        *var_wjuncld_dn2_slot = var_wjuncld_dn2;
        *var_wjuncld_dn4_slot = var_wjuncld_dn4;
        *var_wjuncld_dn5_slot = var_wjuncld_dn5;
        *var_wjuncld_dn6_slot = var_wjuncld_dn6;
        *var_wjuncld_dn7_slot = var_wjuncld_dn7;
        *var_wjuncld_dn8_slot = var_wjuncld_dn8;
        *var_wjuncld_dn9_slot = var_wjuncld_dn9;
        *var_wjuncld_rv_slot = var_wjuncld_rv;
        *var_wk_ii_slot = var_wk_ii;
        *var_wk_ii_dn0_slot = var_wk_ii_dn0;
        *var_wk_ii_dn10_slot = var_wk_ii_dn10;
        *var_wk_ii_dn13_slot = var_wk_ii_dn13;
        *var_wk_ii_dn2_slot = var_wk_ii_dn2;
        *var_wk_ii_dn4_slot = var_wk_ii_dn4;
        *var_wk_ii_dn5_slot = var_wk_ii_dn5;
        *var_wk_ii_dn6_slot = var_wk_ii_dn6;
        *var_wk_ii_dn7_slot = var_wk_ii_dn7;
        *var_wk_ii_dn8_slot = var_wk_ii_dn8;
        *var_wk_ii_dn9_slot = var_wk_ii_dn9;
        *var_wk_ii_rv_slot = var_wk_ii_rv;
    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        var_uc_codep: f64,
        var_guard110_slot: &mut f64,
        var_guard110_rv_slot: &mut f64,
        var_guard113_slot: &mut f64,
        var_guard113_rv_slot: &mut f64,
        var_guard114_slot: &mut f64,
        var_guard114_rv_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard117_rv_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_guard118_rv_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_guard121_rv_slot: &mut f64,
        var_guard122_slot: &mut f64,
        var_guard122_rv_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_guard125_rv_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard126_rv_slot: &mut f64,
        var_guard129_slot: &mut f64,
        var_guard129_rv_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard130_rv_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard133_rv_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_guard134_rv_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard137_rv_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard138_rv_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_guard139_rv_slot: &mut f64,
        var_guard142_slot: &mut f64,
        var_guard142_rv_slot: &mut f64,
        var_guard143_slot: &mut f64,
        var_guard143_rv_slot: &mut f64,
        var_guard146_slot: &mut f64,
        var_guard146_rv_slot: &mut f64,
        var_guard147_slot: &mut f64,
        var_guard147_rv_slot: &mut f64,
        var_guard150_slot: &mut f64,
        var_guard150_rv_slot: &mut f64,
        var_guard151_slot: &mut f64,
        var_guard151_rv_slot: &mut f64,
        var_guard154_slot: &mut f64,
        var_guard154_rv_slot: &mut f64,
        var_guard155_slot: &mut f64,
        var_guard155_rv_slot: &mut f64,
        var_uc_depleak_slot: &mut f64,
        var_uc_depleak_dn0_slot: &mut f64,
        var_uc_depleak_dn10_slot: &mut f64,
        var_uc_depleak_dn13_slot: &mut f64,
        var_uc_depleak_dn2_slot: &mut f64,
        var_uc_depleak_dn4_slot: &mut f64,
        var_uc_depleak_dn5_slot: &mut f64,
        var_uc_depleak_dn6_slot: &mut f64,
        var_uc_depleak_dn7_slot: &mut f64,
        var_uc_depleak_dn8_slot: &mut f64,
        var_uc_depleak_dn9_slot: &mut f64,
        var_uc_depleak_rv_slot: &mut f64,
        var_uc_depmue0_slot: &mut f64,
        var_uc_depmue0_dn0_slot: &mut f64,
        var_uc_depmue0_dn10_slot: &mut f64,
        var_uc_depmue0_dn13_slot: &mut f64,
        var_uc_depmue0_dn2_slot: &mut f64,
        var_uc_depmue0_dn4_slot: &mut f64,
        var_uc_depmue0_dn5_slot: &mut f64,
        var_uc_depmue0_dn6_slot: &mut f64,
        var_uc_depmue0_dn7_slot: &mut f64,
        var_uc_depmue0_dn8_slot: &mut f64,
        var_uc_depmue0_dn9_slot: &mut f64,
        var_uc_depmue0_rv_slot: &mut f64,
        var_uc_depmueback0_slot: &mut f64,
        var_uc_depmueback0_dn0_slot: &mut f64,
        var_uc_depmueback0_dn10_slot: &mut f64,
        var_uc_depmueback0_dn13_slot: &mut f64,
        var_uc_depmueback0_dn2_slot: &mut f64,
        var_uc_depmueback0_dn4_slot: &mut f64,
        var_uc_depmueback0_dn5_slot: &mut f64,
        var_uc_depmueback0_dn6_slot: &mut f64,
        var_uc_depmueback0_dn7_slot: &mut f64,
        var_uc_depmueback0_dn8_slot: &mut f64,
        var_uc_depmueback0_dn9_slot: &mut f64,
        var_uc_depmueback0_rv_slot: &mut f64,
        var_uc_depmueph1_slot: &mut f64,
        var_uc_depmueph1_rv_slot: &mut f64,
        var_uc_depthn_slot: &mut f64,
        var_uc_depthn_dn0_slot: &mut f64,
        var_uc_depthn_dn10_slot: &mut f64,
        var_uc_depthn_dn13_slot: &mut f64,
        var_uc_depthn_dn2_slot: &mut f64,
        var_uc_depthn_dn4_slot: &mut f64,
        var_uc_depthn_dn5_slot: &mut f64,
        var_uc_depthn_dn6_slot: &mut f64,
        var_uc_depthn_dn7_slot: &mut f64,
        var_uc_depthn_dn8_slot: &mut f64,
        var_uc_depthn_dn9_slot: &mut f64,
        var_uc_depthn_rv_slot: &mut f64,
        var_uc_depvdsef1_slot: &mut f64,
        var_uc_depvdsef1_dn0_slot: &mut f64,
        var_uc_depvdsef1_dn10_slot: &mut f64,
        var_uc_depvdsef1_dn13_slot: &mut f64,
        var_uc_depvdsef1_dn2_slot: &mut f64,
        var_uc_depvdsef1_dn4_slot: &mut f64,
        var_uc_depvdsef1_dn5_slot: &mut f64,
        var_uc_depvdsef1_dn6_slot: &mut f64,
        var_uc_depvdsef1_dn7_slot: &mut f64,
        var_uc_depvdsef1_dn8_slot: &mut f64,
        var_uc_depvdsef1_dn9_slot: &mut f64,
        var_uc_depvdsef1_rv_slot: &mut f64,
        var_uc_depvdsef2_slot: &mut f64,
        var_uc_depvdsef2_dn0_slot: &mut f64,
        var_uc_depvdsef2_dn10_slot: &mut f64,
        var_uc_depvdsef2_dn13_slot: &mut f64,
        var_uc_depvdsef2_dn2_slot: &mut f64,
        var_uc_depvdsef2_dn4_slot: &mut f64,
        var_uc_depvdsef2_dn5_slot: &mut f64,
        var_uc_depvdsef2_dn6_slot: &mut f64,
        var_uc_depvdsef2_dn7_slot: &mut f64,
        var_uc_depvdsef2_dn8_slot: &mut f64,
        var_uc_depvdsef2_dn9_slot: &mut f64,
        var_uc_depvdsef2_rv_slot: &mut f64,
        var_uc_ndepm_slot: &mut f64,
        var_uc_ndepm_dn0_slot: &mut f64,
        var_uc_ndepm_dn10_slot: &mut f64,
        var_uc_ndepm_dn13_slot: &mut f64,
        var_uc_ndepm_dn2_slot: &mut f64,
        var_uc_ndepm_dn4_slot: &mut f64,
        var_uc_ndepm_dn5_slot: &mut f64,
        var_uc_ndepm_dn6_slot: &mut f64,
        var_uc_ndepm_dn7_slot: &mut f64,
        var_uc_ndepm_dn8_slot: &mut f64,
        var_uc_ndepm_dn9_slot: &mut f64,
        var_uc_ndepm_rv_slot: &mut f64,
    ) {
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard110_rv: f64 = *var_guard110_rv_slot;
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_guard113_rv: f64 = *var_guard113_rv_slot;
        let mut var_guard114: f64 = *var_guard114_slot;
        let mut var_guard114_rv: f64 = *var_guard114_rv_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard117_rv: f64 = *var_guard117_rv_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard118_rv: f64 = *var_guard118_rv_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_guard121_rv: f64 = *var_guard121_rv_slot;
        let mut var_guard122: f64 = *var_guard122_slot;
        let mut var_guard122_rv: f64 = *var_guard122_rv_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_guard125_rv: f64 = *var_guard125_rv_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard126_rv: f64 = *var_guard126_rv_slot;
        let mut var_guard129: f64 = *var_guard129_slot;
        let mut var_guard129_rv: f64 = *var_guard129_rv_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard130_rv: f64 = *var_guard130_rv_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard133_rv: f64 = *var_guard133_rv_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard134_rv: f64 = *var_guard134_rv_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard137_rv: f64 = *var_guard137_rv_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard138_rv: f64 = *var_guard138_rv_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_guard139_rv: f64 = *var_guard139_rv_slot;
        let mut var_guard142: f64 = *var_guard142_slot;
        let mut var_guard142_rv: f64 = *var_guard142_rv_slot;
        let mut var_guard143: f64 = *var_guard143_slot;
        let mut var_guard143_rv: f64 = *var_guard143_rv_slot;
        let mut var_guard146: f64 = *var_guard146_slot;
        let mut var_guard146_rv: f64 = *var_guard146_rv_slot;
        let mut var_guard147: f64 = *var_guard147_slot;
        let mut var_guard147_rv: f64 = *var_guard147_rv_slot;
        let mut var_guard150: f64 = *var_guard150_slot;
        let mut var_guard150_rv: f64 = *var_guard150_rv_slot;
        let mut var_guard151: f64 = *var_guard151_slot;
        let mut var_guard151_rv: f64 = *var_guard151_rv_slot;
        let mut var_guard154: f64 = *var_guard154_slot;
        let mut var_guard154_rv: f64 = *var_guard154_rv_slot;
        let mut var_guard155: f64 = *var_guard155_slot;
        let mut var_guard155_rv: f64 = *var_guard155_rv_slot;
        let mut var_uc_depleak: f64 = *var_uc_depleak_slot;
        let mut var_uc_depleak_dn0: f64 = *var_uc_depleak_dn0_slot;
        let mut var_uc_depleak_dn10: f64 = *var_uc_depleak_dn10_slot;
        let mut var_uc_depleak_dn13: f64 = *var_uc_depleak_dn13_slot;
        let mut var_uc_depleak_dn2: f64 = *var_uc_depleak_dn2_slot;
        let mut var_uc_depleak_dn4: f64 = *var_uc_depleak_dn4_slot;
        let mut var_uc_depleak_dn5: f64 = *var_uc_depleak_dn5_slot;
        let mut var_uc_depleak_dn6: f64 = *var_uc_depleak_dn6_slot;
        let mut var_uc_depleak_dn7: f64 = *var_uc_depleak_dn7_slot;
        let mut var_uc_depleak_dn8: f64 = *var_uc_depleak_dn8_slot;
        let mut var_uc_depleak_dn9: f64 = *var_uc_depleak_dn9_slot;
        let mut var_uc_depleak_rv: f64 = *var_uc_depleak_rv_slot;
        let mut var_uc_depmue0: f64 = *var_uc_depmue0_slot;
        let mut var_uc_depmue0_dn0: f64 = *var_uc_depmue0_dn0_slot;
        let mut var_uc_depmue0_dn10: f64 = *var_uc_depmue0_dn10_slot;
        let mut var_uc_depmue0_dn13: f64 = *var_uc_depmue0_dn13_slot;
        let mut var_uc_depmue0_dn2: f64 = *var_uc_depmue0_dn2_slot;
        let mut var_uc_depmue0_dn4: f64 = *var_uc_depmue0_dn4_slot;
        let mut var_uc_depmue0_dn5: f64 = *var_uc_depmue0_dn5_slot;
        let mut var_uc_depmue0_dn6: f64 = *var_uc_depmue0_dn6_slot;
        let mut var_uc_depmue0_dn7: f64 = *var_uc_depmue0_dn7_slot;
        let mut var_uc_depmue0_dn8: f64 = *var_uc_depmue0_dn8_slot;
        let mut var_uc_depmue0_dn9: f64 = *var_uc_depmue0_dn9_slot;
        let mut var_uc_depmue0_rv: f64 = *var_uc_depmue0_rv_slot;
        let mut var_uc_depmueback0: f64 = *var_uc_depmueback0_slot;
        let mut var_uc_depmueback0_dn0: f64 = *var_uc_depmueback0_dn0_slot;
        let mut var_uc_depmueback0_dn10: f64 = *var_uc_depmueback0_dn10_slot;
        let mut var_uc_depmueback0_dn13: f64 = *var_uc_depmueback0_dn13_slot;
        let mut var_uc_depmueback0_dn2: f64 = *var_uc_depmueback0_dn2_slot;
        let mut var_uc_depmueback0_dn4: f64 = *var_uc_depmueback0_dn4_slot;
        let mut var_uc_depmueback0_dn5: f64 = *var_uc_depmueback0_dn5_slot;
        let mut var_uc_depmueback0_dn6: f64 = *var_uc_depmueback0_dn6_slot;
        let mut var_uc_depmueback0_dn7: f64 = *var_uc_depmueback0_dn7_slot;
        let mut var_uc_depmueback0_dn8: f64 = *var_uc_depmueback0_dn8_slot;
        let mut var_uc_depmueback0_dn9: f64 = *var_uc_depmueback0_dn9_slot;
        let mut var_uc_depmueback0_rv: f64 = *var_uc_depmueback0_rv_slot;
        let mut var_uc_depmueph1: f64 = *var_uc_depmueph1_slot;
        let mut var_uc_depmueph1_rv: f64 = *var_uc_depmueph1_rv_slot;
        let mut var_uc_depthn: f64 = *var_uc_depthn_slot;
        let mut var_uc_depthn_dn0: f64 = *var_uc_depthn_dn0_slot;
        let mut var_uc_depthn_dn10: f64 = *var_uc_depthn_dn10_slot;
        let mut var_uc_depthn_dn13: f64 = *var_uc_depthn_dn13_slot;
        let mut var_uc_depthn_dn2: f64 = *var_uc_depthn_dn2_slot;
        let mut var_uc_depthn_dn4: f64 = *var_uc_depthn_dn4_slot;
        let mut var_uc_depthn_dn5: f64 = *var_uc_depthn_dn5_slot;
        let mut var_uc_depthn_dn6: f64 = *var_uc_depthn_dn6_slot;
        let mut var_uc_depthn_dn7: f64 = *var_uc_depthn_dn7_slot;
        let mut var_uc_depthn_dn8: f64 = *var_uc_depthn_dn8_slot;
        let mut var_uc_depthn_dn9: f64 = *var_uc_depthn_dn9_slot;
        let mut var_uc_depthn_rv: f64 = *var_uc_depthn_rv_slot;
        let mut var_uc_depvdsef1: f64 = *var_uc_depvdsef1_slot;
        let mut var_uc_depvdsef1_dn0: f64 = *var_uc_depvdsef1_dn0_slot;
        let mut var_uc_depvdsef1_dn10: f64 = *var_uc_depvdsef1_dn10_slot;
        let mut var_uc_depvdsef1_dn13: f64 = *var_uc_depvdsef1_dn13_slot;
        let mut var_uc_depvdsef1_dn2: f64 = *var_uc_depvdsef1_dn2_slot;
        let mut var_uc_depvdsef1_dn4: f64 = *var_uc_depvdsef1_dn4_slot;
        let mut var_uc_depvdsef1_dn5: f64 = *var_uc_depvdsef1_dn5_slot;
        let mut var_uc_depvdsef1_dn6: f64 = *var_uc_depvdsef1_dn6_slot;
        let mut var_uc_depvdsef1_dn7: f64 = *var_uc_depvdsef1_dn7_slot;
        let mut var_uc_depvdsef1_dn8: f64 = *var_uc_depvdsef1_dn8_slot;
        let mut var_uc_depvdsef1_dn9: f64 = *var_uc_depvdsef1_dn9_slot;
        let mut var_uc_depvdsef1_rv: f64 = *var_uc_depvdsef1_rv_slot;
        let mut var_uc_depvdsef2: f64 = *var_uc_depvdsef2_slot;
        let mut var_uc_depvdsef2_dn0: f64 = *var_uc_depvdsef2_dn0_slot;
        let mut var_uc_depvdsef2_dn10: f64 = *var_uc_depvdsef2_dn10_slot;
        let mut var_uc_depvdsef2_dn13: f64 = *var_uc_depvdsef2_dn13_slot;
        let mut var_uc_depvdsef2_dn2: f64 = *var_uc_depvdsef2_dn2_slot;
        let mut var_uc_depvdsef2_dn4: f64 = *var_uc_depvdsef2_dn4_slot;
        let mut var_uc_depvdsef2_dn5: f64 = *var_uc_depvdsef2_dn5_slot;
        let mut var_uc_depvdsef2_dn6: f64 = *var_uc_depvdsef2_dn6_slot;
        let mut var_uc_depvdsef2_dn7: f64 = *var_uc_depvdsef2_dn7_slot;
        let mut var_uc_depvdsef2_dn8: f64 = *var_uc_depvdsef2_dn8_slot;
        let mut var_uc_depvdsef2_dn9: f64 = *var_uc_depvdsef2_dn9_slot;
        let mut var_uc_depvdsef2_rv: f64 = *var_uc_depvdsef2_rv_slot;
        let mut var_uc_ndepm: f64 = *var_uc_ndepm_slot;
        let mut var_uc_ndepm_dn0: f64 = *var_uc_ndepm_dn0_slot;
        let mut var_uc_ndepm_dn10: f64 = *var_uc_ndepm_dn10_slot;
        let mut var_uc_ndepm_dn13: f64 = *var_uc_ndepm_dn13_slot;
        let mut var_uc_ndepm_dn2: f64 = *var_uc_ndepm_dn2_slot;
        let mut var_uc_ndepm_dn4: f64 = *var_uc_ndepm_dn4_slot;
        let mut var_uc_ndepm_dn5: f64 = *var_uc_ndepm_dn5_slot;
        let mut var_uc_ndepm_dn6: f64 = *var_uc_ndepm_dn6_slot;
        let mut var_uc_ndepm_dn7: f64 = *var_uc_ndepm_dn7_slot;
        let mut var_uc_ndepm_dn8: f64 = *var_uc_ndepm_dn8_slot;
        let mut var_uc_ndepm_dn9: f64 = *var_uc_ndepm_dn9_slot;
        let mut var_uc_ndepm_rv: f64 = *var_uc_ndepm_rv_slot;

        var_uc_depvdsef1 = p.p370;
        var_uc_depvdsef1_dn0 = 0.0;
        var_uc_depvdsef1_dn2 = 0.0;
        var_uc_depvdsef1_dn4 = 0.0;
        var_uc_depvdsef1_dn5 = 0.0;
        var_uc_depvdsef1_dn6 = 0.0;
        var_uc_depvdsef1_dn7 = 0.0;
        var_uc_depvdsef1_dn8 = 0.0;
        var_uc_depvdsef1_dn9 = 0.0;
        var_uc_depvdsef1_dn10 = 0.0;
        var_uc_depvdsef1_dn13 = 0.0;
        var_uc_depvdsef1_rv = 0.0;

        var_uc_depvdsef2 = p.p371;
        var_uc_depvdsef2_dn0 = 0.0;
        var_uc_depvdsef2_dn2 = 0.0;
        var_uc_depvdsef2_dn4 = 0.0;
        var_uc_depvdsef2_dn5 = 0.0;
        var_uc_depvdsef2_dn6 = 0.0;
        var_uc_depvdsef2_dn7 = 0.0;
        var_uc_depvdsef2_dn8 = 0.0;
        var_uc_depvdsef2_dn9 = 0.0;
        var_uc_depvdsef2_dn10 = 0.0;
        var_uc_depvdsef2_dn13 = 0.0;
        var_uc_depvdsef2_rv = 0.0;

        let assign6690_e2699: f64 = if ((var_uc_codep < 3.0) && (var_uc_codep > 0.0)) { 1.0 } else { 0.0 };
        var_guard110 = assign6690_e2699;
        var_guard110_rv = 0.0;

        let assign6720_e2712: f64 = if var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        var_guard113 = assign6720_e2712;
        var_guard113_rv = 0.0;

        let (assign6730_e2718, assign6730_e2718_d_n0, assign6730_e2718_d_n2, assign6730_e2718_d_n4, assign6730_e2718_d_n5, assign6730_e2718_d_n6, assign6730_e2718_d_n7, assign6730_e2718_d_n8, assign6730_e2718_d_n9, assign6730_e2718_d_n10, assign6730_e2718_d_n13,) = {
    if ((var_guard110 != 0.0) && (var_guard113 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn13,)
    }
};
        var_uc_ndepm = assign6730_e2718;
        var_uc_ndepm_dn0 = assign6730_e2718_d_n0;
        var_uc_ndepm_dn2 = assign6730_e2718_d_n2;
        var_uc_ndepm_dn4 = assign6730_e2718_d_n4;
        var_uc_ndepm_dn5 = assign6730_e2718_d_n5;
        var_uc_ndepm_dn6 = assign6730_e2718_d_n6;
        var_uc_ndepm_dn7 = assign6730_e2718_d_n7;
        var_uc_ndepm_dn8 = assign6730_e2718_d_n8;
        var_uc_ndepm_dn9 = assign6730_e2718_d_n9;
        var_uc_ndepm_dn10 = assign6730_e2718_d_n10;
        var_uc_ndepm_dn13 = assign6730_e2718_d_n13;
        var_uc_ndepm_rv = 0.0;

        let assign6740_e2721: f64 = if var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        var_guard114 = assign6740_e2721;
        var_guard114_rv = 0.0;

        let (assign6750_e2727, assign6750_e2727_d_n0, assign6750_e2727_d_n2, assign6750_e2727_d_n4, assign6750_e2727_d_n5, assign6750_e2727_d_n6, assign6750_e2727_d_n7, assign6750_e2727_d_n8, assign6750_e2727_d_n9, assign6750_e2727_d_n10, assign6750_e2727_d_n13,) = {
    if ((var_guard110 != 0.0) && (var_guard114 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn13,)
    }
};
        var_uc_ndepm = assign6750_e2727;
        var_uc_ndepm_dn0 = assign6750_e2727_d_n0;
        var_uc_ndepm_dn2 = assign6750_e2727_d_n2;
        var_uc_ndepm_dn4 = assign6750_e2727_d_n4;
        var_uc_ndepm_dn5 = assign6750_e2727_d_n5;
        var_uc_ndepm_dn6 = assign6750_e2727_d_n6;
        var_uc_ndepm_dn7 = assign6750_e2727_d_n7;
        var_uc_ndepm_dn8 = assign6750_e2727_d_n8;
        var_uc_ndepm_dn9 = assign6750_e2727_d_n9;
        var_uc_ndepm_dn10 = assign6750_e2727_d_n10;
        var_uc_ndepm_dn13 = assign6750_e2727_d_n13;
        var_uc_ndepm_rv = 0.0;

        let assign6780_e2740: f64 = if var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        var_guard117 = assign6780_e2740;
        var_guard117_rv = 0.0;

        let (assign6790_e2746, assign6790_e2746_d_n0, assign6790_e2746_d_n2, assign6790_e2746_d_n4, assign6790_e2746_d_n5, assign6790_e2746_d_n6, assign6790_e2746_d_n7, assign6790_e2746_d_n8, assign6790_e2746_d_n9, assign6790_e2746_d_n10, assign6790_e2746_d_n13,) = {
    if ((var_guard110 != 0.0) && (var_guard117 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depthn, var_uc_depthn_dn0, var_uc_depthn_dn2, var_uc_depthn_dn4, var_uc_depthn_dn5, var_uc_depthn_dn6, var_uc_depthn_dn7, var_uc_depthn_dn8, var_uc_depthn_dn9, var_uc_depthn_dn10, var_uc_depthn_dn13,)
    }
};
        var_uc_depthn = assign6790_e2746;
        var_uc_depthn_dn0 = assign6790_e2746_d_n0;
        var_uc_depthn_dn2 = assign6790_e2746_d_n2;
        var_uc_depthn_dn4 = assign6790_e2746_d_n4;
        var_uc_depthn_dn5 = assign6790_e2746_d_n5;
        var_uc_depthn_dn6 = assign6790_e2746_d_n6;
        var_uc_depthn_dn7 = assign6790_e2746_d_n7;
        var_uc_depthn_dn8 = assign6790_e2746_d_n8;
        var_uc_depthn_dn9 = assign6790_e2746_d_n9;
        var_uc_depthn_dn10 = assign6790_e2746_d_n10;
        var_uc_depthn_dn13 = assign6790_e2746_d_n13;
        var_uc_depthn_rv = 0.0;

        let assign6800_e2749: f64 = if var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        var_guard118 = assign6800_e2749;
        var_guard118_rv = 0.0;

        let (assign6810_e2755, assign6810_e2755_d_n0, assign6810_e2755_d_n2, assign6810_e2755_d_n4, assign6810_e2755_d_n5, assign6810_e2755_d_n6, assign6810_e2755_d_n7, assign6810_e2755_d_n8, assign6810_e2755_d_n9, assign6810_e2755_d_n10, assign6810_e2755_d_n13,) = {
    if ((var_guard110 != 0.0) && (var_guard118 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depthn, var_uc_depthn_dn0, var_uc_depthn_dn2, var_uc_depthn_dn4, var_uc_depthn_dn5, var_uc_depthn_dn6, var_uc_depthn_dn7, var_uc_depthn_dn8, var_uc_depthn_dn9, var_uc_depthn_dn10, var_uc_depthn_dn13,)
    }
};
        var_uc_depthn = assign6810_e2755;
        var_uc_depthn_dn0 = assign6810_e2755_d_n0;
        var_uc_depthn_dn2 = assign6810_e2755_d_n2;
        var_uc_depthn_dn4 = assign6810_e2755_d_n4;
        var_uc_depthn_dn5 = assign6810_e2755_d_n5;
        var_uc_depthn_dn6 = assign6810_e2755_d_n6;
        var_uc_depthn_dn7 = assign6810_e2755_d_n7;
        var_uc_depthn_dn8 = assign6810_e2755_d_n8;
        var_uc_depthn_dn9 = assign6810_e2755_d_n9;
        var_uc_depthn_dn10 = assign6810_e2755_d_n10;
        var_uc_depthn_dn13 = assign6810_e2755_d_n13;
        var_uc_depthn_rv = 0.0;

        let assign6840_e2768: f64 = if var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        var_guard121 = assign6840_e2768;
        var_guard121_rv = 0.0;

        let (assign6850_e2774, assign6850_e2774_d_n0, assign6850_e2774_d_n2, assign6850_e2774_d_n4, assign6850_e2774_d_n5, assign6850_e2774_d_n6, assign6850_e2774_d_n7, assign6850_e2774_d_n8, assign6850_e2774_d_n9, assign6850_e2774_d_n10, assign6850_e2774_d_n13,) = {
    if ((var_guard110 != 0.0) && (var_guard121 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn13,)
    }
};
        var_uc_depmue0 = assign6850_e2774;
        var_uc_depmue0_dn0 = assign6850_e2774_d_n0;
        var_uc_depmue0_dn2 = assign6850_e2774_d_n2;
        var_uc_depmue0_dn4 = assign6850_e2774_d_n4;
        var_uc_depmue0_dn5 = assign6850_e2774_d_n5;
        var_uc_depmue0_dn6 = assign6850_e2774_d_n6;
        var_uc_depmue0_dn7 = assign6850_e2774_d_n7;
        var_uc_depmue0_dn8 = assign6850_e2774_d_n8;
        var_uc_depmue0_dn9 = assign6850_e2774_d_n9;
        var_uc_depmue0_dn10 = assign6850_e2774_d_n10;
        var_uc_depmue0_dn13 = assign6850_e2774_d_n13;
        var_uc_depmue0_rv = 0.0;

        let assign6860_e2777: f64 = if var_uc_depmue0 > 100000.0 { 1.0 } else { 0.0 };
        var_guard122 = assign6860_e2777;
        var_guard122_rv = 0.0;

        let (assign6870_e2783, assign6870_e2783_d_n0, assign6870_e2783_d_n2, assign6870_e2783_d_n4, assign6870_e2783_d_n5, assign6870_e2783_d_n6, assign6870_e2783_d_n7, assign6870_e2783_d_n8, assign6870_e2783_d_n9, assign6870_e2783_d_n10, assign6870_e2783_d_n13,) = {
    if ((var_guard110 != 0.0) && (var_guard122 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn13,)
    }
};
        var_uc_depmue0 = assign6870_e2783;
        var_uc_depmue0_dn0 = assign6870_e2783_d_n0;
        var_uc_depmue0_dn2 = assign6870_e2783_d_n2;
        var_uc_depmue0_dn4 = assign6870_e2783_d_n4;
        var_uc_depmue0_dn5 = assign6870_e2783_d_n5;
        var_uc_depmue0_dn6 = assign6870_e2783_d_n6;
        var_uc_depmue0_dn7 = assign6870_e2783_d_n7;
        var_uc_depmue0_dn8 = assign6870_e2783_d_n8;
        var_uc_depmue0_dn9 = assign6870_e2783_d_n9;
        var_uc_depmue0_dn10 = assign6870_e2783_d_n10;
        var_uc_depmue0_dn13 = assign6870_e2783_d_n13;
        var_uc_depmue0_rv = 0.0;

        let assign6900_e2796: f64 = if var_uc_depmueback0 < 1.0 { 1.0 } else { 0.0 };
        var_guard125 = assign6900_e2796;
        var_guard125_rv = 0.0;

        let (assign6910_e2802, assign6910_e2802_d_n0, assign6910_e2802_d_n2, assign6910_e2802_d_n4, assign6910_e2802_d_n5, assign6910_e2802_d_n6, assign6910_e2802_d_n7, assign6910_e2802_d_n8, assign6910_e2802_d_n9, assign6910_e2802_d_n10, assign6910_e2802_d_n13,) = {
    if ((var_guard110 != 0.0) && (var_guard125 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmueback0, var_uc_depmueback0_dn0, var_uc_depmueback0_dn2, var_uc_depmueback0_dn4, var_uc_depmueback0_dn5, var_uc_depmueback0_dn6, var_uc_depmueback0_dn7, var_uc_depmueback0_dn8, var_uc_depmueback0_dn9, var_uc_depmueback0_dn10, var_uc_depmueback0_dn13,)
    }
};
        var_uc_depmueback0 = assign6910_e2802;
        var_uc_depmueback0_dn0 = assign6910_e2802_d_n0;
        var_uc_depmueback0_dn2 = assign6910_e2802_d_n2;
        var_uc_depmueback0_dn4 = assign6910_e2802_d_n4;
        var_uc_depmueback0_dn5 = assign6910_e2802_d_n5;
        var_uc_depmueback0_dn6 = assign6910_e2802_d_n6;
        var_uc_depmueback0_dn7 = assign6910_e2802_d_n7;
        var_uc_depmueback0_dn8 = assign6910_e2802_d_n8;
        var_uc_depmueback0_dn9 = assign6910_e2802_d_n9;
        var_uc_depmueback0_dn10 = assign6910_e2802_d_n10;
        var_uc_depmueback0_dn13 = assign6910_e2802_d_n13;
        var_uc_depmueback0_rv = 0.0;

        let assign6920_e2805: f64 = if var_uc_depmueback0 > 100000.0 { 1.0 } else { 0.0 };
        var_guard126 = assign6920_e2805;
        var_guard126_rv = 0.0;

        let (assign6930_e2811, assign6930_e2811_d_n0, assign6930_e2811_d_n2, assign6930_e2811_d_n4, assign6930_e2811_d_n5, assign6930_e2811_d_n6, assign6930_e2811_d_n7, assign6930_e2811_d_n8, assign6930_e2811_d_n9, assign6930_e2811_d_n10, assign6930_e2811_d_n13,) = {
    if ((var_guard110 != 0.0) && (var_guard126 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmueback0, var_uc_depmueback0_dn0, var_uc_depmueback0_dn2, var_uc_depmueback0_dn4, var_uc_depmueback0_dn5, var_uc_depmueback0_dn6, var_uc_depmueback0_dn7, var_uc_depmueback0_dn8, var_uc_depmueback0_dn9, var_uc_depmueback0_dn10, var_uc_depmueback0_dn13,)
    }
};
        var_uc_depmueback0 = assign6930_e2811;
        var_uc_depmueback0_dn0 = assign6930_e2811_d_n0;
        var_uc_depmueback0_dn2 = assign6930_e2811_d_n2;
        var_uc_depmueback0_dn4 = assign6930_e2811_d_n4;
        var_uc_depmueback0_dn5 = assign6930_e2811_d_n5;
        var_uc_depmueback0_dn6 = assign6930_e2811_d_n6;
        var_uc_depmueback0_dn7 = assign6930_e2811_d_n7;
        var_uc_depmueback0_dn8 = assign6930_e2811_d_n8;
        var_uc_depmueback0_dn9 = assign6930_e2811_d_n9;
        var_uc_depmueback0_dn10 = assign6930_e2811_d_n10;
        var_uc_depmueback0_dn13 = assign6930_e2811_d_n13;
        var_uc_depmueback0_rv = 0.0;

        let assign6960_e2824: f64 = if var_uc_depmueph1 < 1.0 { 1.0 } else { 0.0 };
        var_guard129 = assign6960_e2824;
        var_guard129_rv = 0.0;

        let (assign6970_e2830,) = {
    if ((var_guard110 != 0.0) && (var_guard129 != 0.0)) {
        (1.0,)
    } else {
        (var_uc_depmueph1,)
    }
};
        var_uc_depmueph1 = assign6970_e2830;
        var_uc_depmueph1_rv = 0.0;

        let assign6980_e2833: f64 = if var_uc_depmueph1 > 100000.0 { 1.0 } else { 0.0 };
        var_guard130 = assign6980_e2833;
        var_guard130_rv = 0.0;

        let (assign6990_e2839,) = {
    if ((var_guard110 != 0.0) && (var_guard130 != 0.0)) {
        (100000.0,)
    } else {
        (var_uc_depmueph1,)
    }
};
        var_uc_depmueph1 = assign6990_e2839;
        var_uc_depmueph1_rv = 0.0;

        let assign7020_e2852: f64 = if var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        var_guard133 = assign7020_e2852;
        var_guard133_rv = 0.0;

        let (assign7030_e2858, assign7030_e2858_d_n0, assign7030_e2858_d_n2, assign7030_e2858_d_n4, assign7030_e2858_d_n5, assign7030_e2858_d_n6, assign7030_e2858_d_n7, assign7030_e2858_d_n8, assign7030_e2858_d_n9, assign7030_e2858_d_n10, assign7030_e2858_d_n13,) = {
    if ((var_guard110 != 0.0) && (var_guard133 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvdsef2, var_uc_depvdsef2_dn0, var_uc_depvdsef2_dn2, var_uc_depvdsef2_dn4, var_uc_depvdsef2_dn5, var_uc_depvdsef2_dn6, var_uc_depvdsef2_dn7, var_uc_depvdsef2_dn8, var_uc_depvdsef2_dn9, var_uc_depvdsef2_dn10, var_uc_depvdsef2_dn13,)
    }
};
        var_uc_depvdsef2 = assign7030_e2858;
        var_uc_depvdsef2_dn0 = assign7030_e2858_d_n0;
        var_uc_depvdsef2_dn2 = assign7030_e2858_d_n2;
        var_uc_depvdsef2_dn4 = assign7030_e2858_d_n4;
        var_uc_depvdsef2_dn5 = assign7030_e2858_d_n5;
        var_uc_depvdsef2_dn6 = assign7030_e2858_d_n6;
        var_uc_depvdsef2_dn7 = assign7030_e2858_d_n7;
        var_uc_depvdsef2_dn8 = assign7030_e2858_d_n8;
        var_uc_depvdsef2_dn9 = assign7030_e2858_d_n9;
        var_uc_depvdsef2_dn10 = assign7030_e2858_d_n10;
        var_uc_depvdsef2_dn13 = assign7030_e2858_d_n13;
        var_uc_depvdsef2_rv = 0.0;

        let assign7040_e2861: f64 = if var_uc_depvdsef2 > 4.0 { 1.0 } else { 0.0 };
        var_guard134 = assign7040_e2861;
        var_guard134_rv = 0.0;

        let (assign7050_e2867, assign7050_e2867_d_n0, assign7050_e2867_d_n2, assign7050_e2867_d_n4, assign7050_e2867_d_n5, assign7050_e2867_d_n6, assign7050_e2867_d_n7, assign7050_e2867_d_n8, assign7050_e2867_d_n9, assign7050_e2867_d_n10, assign7050_e2867_d_n13,) = {
    if ((var_guard110 != 0.0) && (var_guard134 != 0.0)) {
        (4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvdsef2, var_uc_depvdsef2_dn0, var_uc_depvdsef2_dn2, var_uc_depvdsef2_dn4, var_uc_depvdsef2_dn5, var_uc_depvdsef2_dn6, var_uc_depvdsef2_dn7, var_uc_depvdsef2_dn8, var_uc_depvdsef2_dn9, var_uc_depvdsef2_dn10, var_uc_depvdsef2_dn13,)
    }
};
        var_uc_depvdsef2 = assign7050_e2867;
        var_uc_depvdsef2_dn0 = assign7050_e2867_d_n0;
        var_uc_depvdsef2_dn2 = assign7050_e2867_d_n2;
        var_uc_depvdsef2_dn4 = assign7050_e2867_d_n4;
        var_uc_depvdsef2_dn5 = assign7050_e2867_d_n5;
        var_uc_depvdsef2_dn6 = assign7050_e2867_d_n6;
        var_uc_depvdsef2_dn7 = assign7050_e2867_d_n7;
        var_uc_depvdsef2_dn8 = assign7050_e2867_d_n8;
        var_uc_depvdsef2_dn9 = assign7050_e2867_d_n9;
        var_uc_depvdsef2_dn10 = assign7050_e2867_d_n10;
        var_uc_depvdsef2_dn13 = assign7050_e2867_d_n13;
        var_uc_depvdsef2_rv = 0.0;

        let assign7080_e2880: f64 = if var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        var_guard137 = assign7080_e2880;
        var_guard137_rv = 0.0;

        let (assign7090_e2886, assign7090_e2886_d_n0, assign7090_e2886_d_n2, assign7090_e2886_d_n4, assign7090_e2886_d_n5, assign7090_e2886_d_n6, assign7090_e2886_d_n7, assign7090_e2886_d_n8, assign7090_e2886_d_n9, assign7090_e2886_d_n10, assign7090_e2886_d_n13,) = {
    if ((var_guard110 != 0.0) && (var_guard137 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn13,)
    }
};
        var_uc_depleak = assign7090_e2886;
        var_uc_depleak_dn0 = assign7090_e2886_d_n0;
        var_uc_depleak_dn2 = assign7090_e2886_d_n2;
        var_uc_depleak_dn4 = assign7090_e2886_d_n4;
        var_uc_depleak_dn5 = assign7090_e2886_d_n5;
        var_uc_depleak_dn6 = assign7090_e2886_d_n6;
        var_uc_depleak_dn7 = assign7090_e2886_d_n7;
        var_uc_depleak_dn8 = assign7090_e2886_d_n8;
        var_uc_depleak_dn9 = assign7090_e2886_d_n9;
        var_uc_depleak_dn10 = assign7090_e2886_d_n10;
        var_uc_depleak_dn13 = assign7090_e2886_d_n13;
        var_uc_depleak_rv = 0.0;

        let assign7100_e2889: f64 = if var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        var_guard138 = assign7100_e2889;
        var_guard138_rv = 0.0;

        let (assign7110_e2895, assign7110_e2895_d_n0, assign7110_e2895_d_n2, assign7110_e2895_d_n4, assign7110_e2895_d_n5, assign7110_e2895_d_n6, assign7110_e2895_d_n7, assign7110_e2895_d_n8, assign7110_e2895_d_n9, assign7110_e2895_d_n10, assign7110_e2895_d_n13,) = {
    if ((var_guard110 != 0.0) && (var_guard138 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn13,)
    }
};
        var_uc_depleak = assign7110_e2895;
        var_uc_depleak_dn0 = assign7110_e2895_d_n0;
        var_uc_depleak_dn2 = assign7110_e2895_d_n2;
        var_uc_depleak_dn4 = assign7110_e2895_d_n4;
        var_uc_depleak_dn5 = assign7110_e2895_d_n5;
        var_uc_depleak_dn6 = assign7110_e2895_d_n6;
        var_uc_depleak_dn7 = assign7110_e2895_d_n7;
        var_uc_depleak_dn8 = assign7110_e2895_d_n8;
        var_uc_depleak_dn9 = assign7110_e2895_d_n9;
        var_uc_depleak_dn10 = assign7110_e2895_d_n10;
        var_uc_depleak_dn13 = assign7110_e2895_d_n13;
        var_uc_depleak_rv = 0.0;

        let assign7120_e2898: f64 = if var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        var_guard139 = assign7120_e2898;
        var_guard139_rv = 0.0;

        let assign7150_e2911: f64 = if var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        var_guard142 = assign7150_e2911;
        var_guard142_rv = 0.0;

        let (assign7160_e2920, assign7160_e2920_d_n0, assign7160_e2920_d_n2, assign7160_e2920_d_n4, assign7160_e2920_d_n5, assign7160_e2920_d_n6, assign7160_e2920_d_n7, assign7160_e2920_d_n8, assign7160_e2920_d_n9, assign7160_e2920_d_n10, assign7160_e2920_d_n13,) = {
    if (((var_guard110 == 0.0) && (var_guard139 != 0.0)) && (var_guard142 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn13,)
    }
};
        var_uc_ndepm = assign7160_e2920;
        var_uc_ndepm_dn0 = assign7160_e2920_d_n0;
        var_uc_ndepm_dn2 = assign7160_e2920_d_n2;
        var_uc_ndepm_dn4 = assign7160_e2920_d_n4;
        var_uc_ndepm_dn5 = assign7160_e2920_d_n5;
        var_uc_ndepm_dn6 = assign7160_e2920_d_n6;
        var_uc_ndepm_dn7 = assign7160_e2920_d_n7;
        var_uc_ndepm_dn8 = assign7160_e2920_d_n8;
        var_uc_ndepm_dn9 = assign7160_e2920_d_n9;
        var_uc_ndepm_dn10 = assign7160_e2920_d_n10;
        var_uc_ndepm_dn13 = assign7160_e2920_d_n13;
        var_uc_ndepm_rv = 0.0;

        let assign7170_e2923: f64 = if var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        var_guard143 = assign7170_e2923;
        var_guard143_rv = 0.0;

        let (assign7180_e2932, assign7180_e2932_d_n0, assign7180_e2932_d_n2, assign7180_e2932_d_n4, assign7180_e2932_d_n5, assign7180_e2932_d_n6, assign7180_e2932_d_n7, assign7180_e2932_d_n8, assign7180_e2932_d_n9, assign7180_e2932_d_n10, assign7180_e2932_d_n13,) = {
    if (((var_guard110 == 0.0) && (var_guard139 != 0.0)) && (var_guard143 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn13,)
    }
};
        var_uc_ndepm = assign7180_e2932;
        var_uc_ndepm_dn0 = assign7180_e2932_d_n0;
        var_uc_ndepm_dn2 = assign7180_e2932_d_n2;
        var_uc_ndepm_dn4 = assign7180_e2932_d_n4;
        var_uc_ndepm_dn5 = assign7180_e2932_d_n5;
        var_uc_ndepm_dn6 = assign7180_e2932_d_n6;
        var_uc_ndepm_dn7 = assign7180_e2932_d_n7;
        var_uc_ndepm_dn8 = assign7180_e2932_d_n8;
        var_uc_ndepm_dn9 = assign7180_e2932_d_n9;
        var_uc_ndepm_dn10 = assign7180_e2932_d_n10;
        var_uc_ndepm_dn13 = assign7180_e2932_d_n13;
        var_uc_ndepm_rv = 0.0;

        let assign7210_e2945: f64 = if var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        var_guard146 = assign7210_e2945;
        var_guard146_rv = 0.0;

        let (assign7220_e2954, assign7220_e2954_d_n0, assign7220_e2954_d_n2, assign7220_e2954_d_n4, assign7220_e2954_d_n5, assign7220_e2954_d_n6, assign7220_e2954_d_n7, assign7220_e2954_d_n8, assign7220_e2954_d_n9, assign7220_e2954_d_n10, assign7220_e2954_d_n13,) = {
    if (((var_guard110 == 0.0) && (var_guard139 != 0.0)) && (var_guard146 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depthn, var_uc_depthn_dn0, var_uc_depthn_dn2, var_uc_depthn_dn4, var_uc_depthn_dn5, var_uc_depthn_dn6, var_uc_depthn_dn7, var_uc_depthn_dn8, var_uc_depthn_dn9, var_uc_depthn_dn10, var_uc_depthn_dn13,)
    }
};
        var_uc_depthn = assign7220_e2954;
        var_uc_depthn_dn0 = assign7220_e2954_d_n0;
        var_uc_depthn_dn2 = assign7220_e2954_d_n2;
        var_uc_depthn_dn4 = assign7220_e2954_d_n4;
        var_uc_depthn_dn5 = assign7220_e2954_d_n5;
        var_uc_depthn_dn6 = assign7220_e2954_d_n6;
        var_uc_depthn_dn7 = assign7220_e2954_d_n7;
        var_uc_depthn_dn8 = assign7220_e2954_d_n8;
        var_uc_depthn_dn9 = assign7220_e2954_d_n9;
        var_uc_depthn_dn10 = assign7220_e2954_d_n10;
        var_uc_depthn_dn13 = assign7220_e2954_d_n13;
        var_uc_depthn_rv = 0.0;

        let assign7230_e2957: f64 = if var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        var_guard147 = assign7230_e2957;
        var_guard147_rv = 0.0;

        let (assign7240_e2966, assign7240_e2966_d_n0, assign7240_e2966_d_n2, assign7240_e2966_d_n4, assign7240_e2966_d_n5, assign7240_e2966_d_n6, assign7240_e2966_d_n7, assign7240_e2966_d_n8, assign7240_e2966_d_n9, assign7240_e2966_d_n10, assign7240_e2966_d_n13,) = {
    if (((var_guard110 == 0.0) && (var_guard139 != 0.0)) && (var_guard147 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depthn, var_uc_depthn_dn0, var_uc_depthn_dn2, var_uc_depthn_dn4, var_uc_depthn_dn5, var_uc_depthn_dn6, var_uc_depthn_dn7, var_uc_depthn_dn8, var_uc_depthn_dn9, var_uc_depthn_dn10, var_uc_depthn_dn13,)
    }
};
        var_uc_depthn = assign7240_e2966;
        var_uc_depthn_dn0 = assign7240_e2966_d_n0;
        var_uc_depthn_dn2 = assign7240_e2966_d_n2;
        var_uc_depthn_dn4 = assign7240_e2966_d_n4;
        var_uc_depthn_dn5 = assign7240_e2966_d_n5;
        var_uc_depthn_dn6 = assign7240_e2966_d_n6;
        var_uc_depthn_dn7 = assign7240_e2966_d_n7;
        var_uc_depthn_dn8 = assign7240_e2966_d_n8;
        var_uc_depthn_dn9 = assign7240_e2966_d_n9;
        var_uc_depthn_dn10 = assign7240_e2966_d_n10;
        var_uc_depthn_dn13 = assign7240_e2966_d_n13;
        var_uc_depthn_rv = 0.0;

        let assign7270_e2979: f64 = if var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        var_guard150 = assign7270_e2979;
        var_guard150_rv = 0.0;

        let (assign7280_e2988, assign7280_e2988_d_n0, assign7280_e2988_d_n2, assign7280_e2988_d_n4, assign7280_e2988_d_n5, assign7280_e2988_d_n6, assign7280_e2988_d_n7, assign7280_e2988_d_n8, assign7280_e2988_d_n9, assign7280_e2988_d_n10, assign7280_e2988_d_n13,) = {
    if (((var_guard110 == 0.0) && (var_guard139 != 0.0)) && (var_guard150 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn13,)
    }
};
        var_uc_depmue0 = assign7280_e2988;
        var_uc_depmue0_dn0 = assign7280_e2988_d_n0;
        var_uc_depmue0_dn2 = assign7280_e2988_d_n2;
        var_uc_depmue0_dn4 = assign7280_e2988_d_n4;
        var_uc_depmue0_dn5 = assign7280_e2988_d_n5;
        var_uc_depmue0_dn6 = assign7280_e2988_d_n6;
        var_uc_depmue0_dn7 = assign7280_e2988_d_n7;
        var_uc_depmue0_dn8 = assign7280_e2988_d_n8;
        var_uc_depmue0_dn9 = assign7280_e2988_d_n9;
        var_uc_depmue0_dn10 = assign7280_e2988_d_n10;
        var_uc_depmue0_dn13 = assign7280_e2988_d_n13;
        var_uc_depmue0_rv = 0.0;

        let assign7290_e2991: f64 = if var_uc_depmue0 > 10000000000.0 { 1.0 } else { 0.0 };
        var_guard151 = assign7290_e2991;
        var_guard151_rv = 0.0;

        let (assign7300_e3000, assign7300_e3000_d_n0, assign7300_e3000_d_n2, assign7300_e3000_d_n4, assign7300_e3000_d_n5, assign7300_e3000_d_n6, assign7300_e3000_d_n7, assign7300_e3000_d_n8, assign7300_e3000_d_n9, assign7300_e3000_d_n10, assign7300_e3000_d_n13,) = {
    if (((var_guard110 == 0.0) && (var_guard139 != 0.0)) && (var_guard151 != 0.0)) {
        (10000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn13,)
    }
};
        var_uc_depmue0 = assign7300_e3000;
        var_uc_depmue0_dn0 = assign7300_e3000_d_n0;
        var_uc_depmue0_dn2 = assign7300_e3000_d_n2;
        var_uc_depmue0_dn4 = assign7300_e3000_d_n4;
        var_uc_depmue0_dn5 = assign7300_e3000_d_n5;
        var_uc_depmue0_dn6 = assign7300_e3000_d_n6;
        var_uc_depmue0_dn7 = assign7300_e3000_d_n7;
        var_uc_depmue0_dn8 = assign7300_e3000_d_n8;
        var_uc_depmue0_dn9 = assign7300_e3000_d_n9;
        var_uc_depmue0_dn10 = assign7300_e3000_d_n10;
        var_uc_depmue0_dn13 = assign7300_e3000_d_n13;
        var_uc_depmue0_rv = 0.0;

        let assign7330_e3013: f64 = if var_uc_depmueph1 < 100.0 { 1.0 } else { 0.0 };
        var_guard154 = assign7330_e3013;
        var_guard154_rv = 0.0;

        let (assign7340_e3022,) = {
    if (((var_guard110 == 0.0) && (var_guard139 != 0.0)) && (var_guard154 != 0.0)) {
        (100.0,)
    } else {
        (var_uc_depmueph1,)
    }
};
        var_uc_depmueph1 = assign7340_e3022;
        var_uc_depmueph1_rv = 0.0;

        let assign7350_e3025: f64 = if var_uc_depmueph1 > 2000000000.0 { 1.0 } else { 0.0 };
        var_guard155 = assign7350_e3025;
        var_guard155_rv = 0.0;

        *var_guard110_slot = var_guard110;
        *var_guard110_rv_slot = var_guard110_rv;
        *var_guard113_slot = var_guard113;
        *var_guard113_rv_slot = var_guard113_rv;
        *var_guard114_slot = var_guard114;
        *var_guard114_rv_slot = var_guard114_rv;
        *var_guard117_slot = var_guard117;
        *var_guard117_rv_slot = var_guard117_rv;
        *var_guard118_slot = var_guard118;
        *var_guard118_rv_slot = var_guard118_rv;
        *var_guard121_slot = var_guard121;
        *var_guard121_rv_slot = var_guard121_rv;
        *var_guard122_slot = var_guard122;
        *var_guard122_rv_slot = var_guard122_rv;
        *var_guard125_slot = var_guard125;
        *var_guard125_rv_slot = var_guard125_rv;
        *var_guard126_slot = var_guard126;
        *var_guard126_rv_slot = var_guard126_rv;
        *var_guard129_slot = var_guard129;
        *var_guard129_rv_slot = var_guard129_rv;
        *var_guard130_slot = var_guard130;
        *var_guard130_rv_slot = var_guard130_rv;
        *var_guard133_slot = var_guard133;
        *var_guard133_rv_slot = var_guard133_rv;
        *var_guard134_slot = var_guard134;
        *var_guard134_rv_slot = var_guard134_rv;
        *var_guard137_slot = var_guard137;
        *var_guard137_rv_slot = var_guard137_rv;
        *var_guard138_slot = var_guard138;
        *var_guard138_rv_slot = var_guard138_rv;
        *var_guard139_slot = var_guard139;
        *var_guard139_rv_slot = var_guard139_rv;
        *var_guard142_slot = var_guard142;
        *var_guard142_rv_slot = var_guard142_rv;
        *var_guard143_slot = var_guard143;
        *var_guard143_rv_slot = var_guard143_rv;
        *var_guard146_slot = var_guard146;
        *var_guard146_rv_slot = var_guard146_rv;
        *var_guard147_slot = var_guard147;
        *var_guard147_rv_slot = var_guard147_rv;
        *var_guard150_slot = var_guard150;
        *var_guard150_rv_slot = var_guard150_rv;
        *var_guard151_slot = var_guard151;
        *var_guard151_rv_slot = var_guard151_rv;
        *var_guard154_slot = var_guard154;
        *var_guard154_rv_slot = var_guard154_rv;
        *var_guard155_slot = var_guard155;
        *var_guard155_rv_slot = var_guard155_rv;
        *var_uc_depleak_slot = var_uc_depleak;
        *var_uc_depleak_dn0_slot = var_uc_depleak_dn0;
        *var_uc_depleak_dn10_slot = var_uc_depleak_dn10;
        *var_uc_depleak_dn13_slot = var_uc_depleak_dn13;
        *var_uc_depleak_dn2_slot = var_uc_depleak_dn2;
        *var_uc_depleak_dn4_slot = var_uc_depleak_dn4;
        *var_uc_depleak_dn5_slot = var_uc_depleak_dn5;
        *var_uc_depleak_dn6_slot = var_uc_depleak_dn6;
        *var_uc_depleak_dn7_slot = var_uc_depleak_dn7;
        *var_uc_depleak_dn8_slot = var_uc_depleak_dn8;
        *var_uc_depleak_dn9_slot = var_uc_depleak_dn9;
        *var_uc_depleak_rv_slot = var_uc_depleak_rv;
        *var_uc_depmue0_slot = var_uc_depmue0;
        *var_uc_depmue0_dn0_slot = var_uc_depmue0_dn0;
        *var_uc_depmue0_dn10_slot = var_uc_depmue0_dn10;
        *var_uc_depmue0_dn13_slot = var_uc_depmue0_dn13;
        *var_uc_depmue0_dn2_slot = var_uc_depmue0_dn2;
        *var_uc_depmue0_dn4_slot = var_uc_depmue0_dn4;
        *var_uc_depmue0_dn5_slot = var_uc_depmue0_dn5;
        *var_uc_depmue0_dn6_slot = var_uc_depmue0_dn6;
        *var_uc_depmue0_dn7_slot = var_uc_depmue0_dn7;
        *var_uc_depmue0_dn8_slot = var_uc_depmue0_dn8;
        *var_uc_depmue0_dn9_slot = var_uc_depmue0_dn9;
        *var_uc_depmue0_rv_slot = var_uc_depmue0_rv;
        *var_uc_depmueback0_slot = var_uc_depmueback0;
        *var_uc_depmueback0_dn0_slot = var_uc_depmueback0_dn0;
        *var_uc_depmueback0_dn10_slot = var_uc_depmueback0_dn10;
        *var_uc_depmueback0_dn13_slot = var_uc_depmueback0_dn13;
        *var_uc_depmueback0_dn2_slot = var_uc_depmueback0_dn2;
        *var_uc_depmueback0_dn4_slot = var_uc_depmueback0_dn4;
        *var_uc_depmueback0_dn5_slot = var_uc_depmueback0_dn5;
        *var_uc_depmueback0_dn6_slot = var_uc_depmueback0_dn6;
        *var_uc_depmueback0_dn7_slot = var_uc_depmueback0_dn7;
        *var_uc_depmueback0_dn8_slot = var_uc_depmueback0_dn8;
        *var_uc_depmueback0_dn9_slot = var_uc_depmueback0_dn9;
        *var_uc_depmueback0_rv_slot = var_uc_depmueback0_rv;
        *var_uc_depmueph1_slot = var_uc_depmueph1;
        *var_uc_depmueph1_rv_slot = var_uc_depmueph1_rv;
        *var_uc_depthn_slot = var_uc_depthn;
        *var_uc_depthn_dn0_slot = var_uc_depthn_dn0;
        *var_uc_depthn_dn10_slot = var_uc_depthn_dn10;
        *var_uc_depthn_dn13_slot = var_uc_depthn_dn13;
        *var_uc_depthn_dn2_slot = var_uc_depthn_dn2;
        *var_uc_depthn_dn4_slot = var_uc_depthn_dn4;
        *var_uc_depthn_dn5_slot = var_uc_depthn_dn5;
        *var_uc_depthn_dn6_slot = var_uc_depthn_dn6;
        *var_uc_depthn_dn7_slot = var_uc_depthn_dn7;
        *var_uc_depthn_dn8_slot = var_uc_depthn_dn8;
        *var_uc_depthn_dn9_slot = var_uc_depthn_dn9;
        *var_uc_depthn_rv_slot = var_uc_depthn_rv;
        *var_uc_depvdsef1_slot = var_uc_depvdsef1;
        *var_uc_depvdsef1_dn0_slot = var_uc_depvdsef1_dn0;
        *var_uc_depvdsef1_dn10_slot = var_uc_depvdsef1_dn10;
        *var_uc_depvdsef1_dn13_slot = var_uc_depvdsef1_dn13;
        *var_uc_depvdsef1_dn2_slot = var_uc_depvdsef1_dn2;
        *var_uc_depvdsef1_dn4_slot = var_uc_depvdsef1_dn4;
        *var_uc_depvdsef1_dn5_slot = var_uc_depvdsef1_dn5;
        *var_uc_depvdsef1_dn6_slot = var_uc_depvdsef1_dn6;
        *var_uc_depvdsef1_dn7_slot = var_uc_depvdsef1_dn7;
        *var_uc_depvdsef1_dn8_slot = var_uc_depvdsef1_dn8;
        *var_uc_depvdsef1_dn9_slot = var_uc_depvdsef1_dn9;
        *var_uc_depvdsef1_rv_slot = var_uc_depvdsef1_rv;
        *var_uc_depvdsef2_slot = var_uc_depvdsef2;
        *var_uc_depvdsef2_dn0_slot = var_uc_depvdsef2_dn0;
        *var_uc_depvdsef2_dn10_slot = var_uc_depvdsef2_dn10;
        *var_uc_depvdsef2_dn13_slot = var_uc_depvdsef2_dn13;
        *var_uc_depvdsef2_dn2_slot = var_uc_depvdsef2_dn2;
        *var_uc_depvdsef2_dn4_slot = var_uc_depvdsef2_dn4;
        *var_uc_depvdsef2_dn5_slot = var_uc_depvdsef2_dn5;
        *var_uc_depvdsef2_dn6_slot = var_uc_depvdsef2_dn6;
        *var_uc_depvdsef2_dn7_slot = var_uc_depvdsef2_dn7;
        *var_uc_depvdsef2_dn8_slot = var_uc_depvdsef2_dn8;
        *var_uc_depvdsef2_dn9_slot = var_uc_depvdsef2_dn9;
        *var_uc_depvdsef2_rv_slot = var_uc_depvdsef2_rv;
        *var_uc_ndepm_slot = var_uc_ndepm;
        *var_uc_ndepm_dn0_slot = var_uc_ndepm_dn0;
        *var_uc_ndepm_dn10_slot = var_uc_ndepm_dn10;
        *var_uc_ndepm_dn13_slot = var_uc_ndepm_dn13;
        *var_uc_ndepm_dn2_slot = var_uc_ndepm_dn2;
        *var_uc_ndepm_dn4_slot = var_uc_ndepm_dn4;
        *var_uc_ndepm_dn5_slot = var_uc_ndepm_dn5;
        *var_uc_ndepm_dn6_slot = var_uc_ndepm_dn6;
        *var_uc_ndepm_dn7_slot = var_uc_ndepm_dn7;
        *var_uc_ndepm_dn8_slot = var_uc_ndepm_dn8;
        *var_uc_ndepm_dn9_slot = var_uc_ndepm_dn9;
        *var_uc_ndepm_rv_slot = var_uc_ndepm_rv;
    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        var_guard110: f64,
        var_guard139: f64,
        var_guard155: f64,
        var_guard158_slot: &mut f64,
        var_guard158_rv_slot: &mut f64,
        var_guard159_slot: &mut f64,
        var_guard159_rv_slot: &mut f64,
        var_guard168_slot: &mut f64,
        var_guard168_rv_slot: &mut f64,
        var_guard169_slot: &mut f64,
        var_guard169_rv_slot: &mut f64,
        var_ktnom_slot: &mut f64,
        var_ktnom_rv_slot: &mut f64,
        var_lbin_slot: &mut f64,
        var_lbin_rv_slot: &mut f64,
        var_lg_slot: &mut f64,
        var_lg_rv_slot: &mut f64,
        var_lgate_slot: &mut f64,
        var_lgate_rv_slot: &mut f64,
        var_lwbin_slot: &mut f64,
        var_lwbin_rv_slot: &mut f64,
        var_mks_cit_slot: &mut f64,
        var_mks_cit_rv_slot: &mut f64,
        var_mks_dly3_slot: &mut f64,
        var_mks_dly3_rv_slot: &mut f64,
        var_mks_dlyov_slot: &mut f64,
        var_mks_dlyov_dn0_slot: &mut f64,
        var_mks_dlyov_dn10_slot: &mut f64,
        var_mks_dlyov_dn13_slot: &mut f64,
        var_mks_dlyov_dn2_slot: &mut f64,
        var_mks_dlyov_dn4_slot: &mut f64,
        var_mks_dlyov_dn5_slot: &mut f64,
        var_mks_dlyov_dn6_slot: &mut f64,
        var_mks_dlyov_dn7_slot: &mut f64,
        var_mks_dlyov_dn8_slot: &mut f64,
        var_mks_dlyov_dn9_slot: &mut f64,
        var_mks_dlyov_rv_slot: &mut f64,
        var_mks_gleak4_slot: &mut f64,
        var_mks_gleak4_rv_slot: &mut f64,
        var_mks_gleak5_slot: &mut f64,
        var_mks_gleak5_rv_slot: &mut f64,
        var_mks_gleak7_slot: &mut f64,
        var_mks_gleak7_rv_slot: &mut f64,
        var_mks_glksd3_slot: &mut f64,
        var_mks_glksd3_rv_slot: &mut f64,
        var_mks_ll_slot: &mut f64,
        var_mks_ll_rv_slot: &mut f64,
        var_mks_nsubsub_slot: &mut f64,
        var_mks_nsubsub_rv_slot: &mut f64,
        var_mks_ovslp_slot: &mut f64,
        var_mks_ovslp_rv_slot: &mut f64,
        var_mks_rdrmue_slot: &mut f64,
        var_mks_rdrmue_rv_slot: &mut f64,
        var_mks_rdrmues_slot: &mut f64,
        var_mks_rdrmues_rv_slot: &mut f64,
        var_mks_rdrvmax_slot: &mut f64,
        var_mks_rdrvmax_rv_slot: &mut f64,
        var_mks_rdrvmaxs_slot: &mut f64,
        var_mks_rdrvmaxs_rv_slot: &mut f64,
        var_mks_rdtemp1_slot: &mut f64,
        var_mks_rdtemp1_rv_slot: &mut f64,
        var_mks_rdtemp2_slot: &mut f64,
        var_mks_rdtemp2_rv_slot: &mut f64,
        var_mks_rdvdtemp1_slot: &mut f64,
        var_mks_rdvdtemp1_rv_slot: &mut f64,
        var_mks_rdvdtemp2_slot: &mut f64,
        var_mks_rdvdtemp2_rv_slot: &mut f64,
        var_mks_slg_slot: &mut f64,
        var_mks_slg_rv_slot: &mut f64,
        var_mks_slgl_slot: &mut f64,
        var_mks_slgl_rv_slot: &mut f64,
        var_mks_sub1l_slot: &mut f64,
        var_mks_sub1l_rv_slot: &mut f64,
        var_mks_sub2l_slot: &mut f64,
        var_mks_sub2l_rv_slot: &mut f64,
        var_mks_subld2_slot: &mut f64,
        var_mks_subld2_rv_slot: &mut f64,
        var_mks_svbsl_slot: &mut f64,
        var_mks_svbsl_rv_slot: &mut f64,
        var_mks_svgsl_slot: &mut f64,
        var_mks_svgsl_rv_slot: &mut f64,
        var_mks_svgsw_slot: &mut f64,
        var_mks_svgsw_rv_slot: &mut f64,
        var_mks_wl_slot: &mut f64,
        var_mks_wl_rv_slot: &mut f64,
        var_uc_bgtmp1_slot: &mut f64,
        var_uc_bgtmp1_rv_slot: &mut f64,
        var_uc_bgtmp2_slot: &mut f64,
        var_uc_bgtmp2_rv_slot: &mut f64,
        var_uc_depleak_slot: &mut f64,
        var_uc_depleak_dn0_slot: &mut f64,
        var_uc_depleak_dn10_slot: &mut f64,
        var_uc_depleak_dn13_slot: &mut f64,
        var_uc_depleak_dn2_slot: &mut f64,
        var_uc_depleak_dn4_slot: &mut f64,
        var_uc_depleak_dn5_slot: &mut f64,
        var_uc_depleak_dn6_slot: &mut f64,
        var_uc_depleak_dn7_slot: &mut f64,
        var_uc_depleak_dn8_slot: &mut f64,
        var_uc_depleak_dn9_slot: &mut f64,
        var_uc_depleak_rv_slot: &mut f64,
        var_uc_depmueph1_slot: &mut f64,
        var_uc_depmueph1_rv_slot: &mut f64,
        var_uc_eg0_slot: &mut f64,
        var_uc_eg0_rv_slot: &mut f64,
        var_uc_muecb0_slot: &mut f64,
        var_uc_muecb0_rv_slot: &mut f64,
        var_uc_muecb1_slot: &mut f64,
        var_uc_muecb1_rv_slot: &mut f64,
        var_uc_mueph1_slot: &mut f64,
        var_uc_mueph1_rv_slot: &mut f64,
        var_uc_muesr1_slot: &mut f64,
        var_uc_muesr1_rv_slot: &mut f64,
        var_uc_muetmp_slot: &mut f64,
        var_uc_muetmp_rv_slot: &mut f64,
        var_uc_ndep_slot: &mut f64,
        var_uc_ndep_rv_slot: &mut f64,
        var_uc_ndepm_slot: &mut f64,
        var_uc_ndepm_dn0_slot: &mut f64,
        var_uc_ndepm_dn10_slot: &mut f64,
        var_uc_ndepm_dn13_slot: &mut f64,
        var_uc_ndepm_dn2_slot: &mut f64,
        var_uc_ndepm_dn4_slot: &mut f64,
        var_uc_ndepm_dn5_slot: &mut f64,
        var_uc_ndepm_dn6_slot: &mut f64,
        var_uc_ndepm_dn7_slot: &mut f64,
        var_uc_ndepm_dn8_slot: &mut f64,
        var_uc_ndepm_dn9_slot: &mut f64,
        var_uc_ndepm_rv_slot: &mut f64,
        var_uc_ninv_slot: &mut f64,
        var_uc_ninv_rv_slot: &mut f64,
        var_uc_njunc_slot: &mut f64,
        var_uc_njunc_rv_slot: &mut f64,
        var_uc_nover_slot: &mut f64,
        var_uc_nover_rv_slot: &mut f64,
        var_uc_novers_slot: &mut f64,
        var_uc_novers_rv_slot: &mut f64,
        var_uc_nsubc_slot: &mut f64,
        var_uc_nsubc_rv_slot: &mut f64,
        var_uc_nsubp_slot: &mut f64,
        var_uc_nsubp_rv_slot: &mut f64,
        var_uc_pgd1_slot: &mut f64,
        var_uc_pgd1_rv_slot: &mut f64,
        var_uc_sc1_slot: &mut f64,
        var_uc_sc1_rv_slot: &mut f64,
        var_uc_sc2_slot: &mut f64,
        var_uc_sc2_rv_slot: &mut f64,
        var_uc_sc3_slot: &mut f64,
        var_uc_sc3_rv_slot: &mut f64,
        var_uc_scp1_slot: &mut f64,
        var_uc_scp1_rv_slot: &mut f64,
        var_uc_scp2_slot: &mut f64,
        var_uc_scp2_rv_slot: &mut f64,
        var_uc_scp3_slot: &mut f64,
        var_uc_scp3_rv_slot: &mut f64,
        var_uc_toxb_slot: &mut f64,
        var_uc_toxb_rv_slot: &mut f64,
        var_uc_vfbc_slot: &mut f64,
        var_uc_vfbc_rv_slot: &mut f64,
        var_uc_vfbover_slot: &mut f64,
        var_uc_vfbover_rv_slot: &mut f64,
        var_uc_vmax_slot: &mut f64,
        var_uc_vmax_rv_slot: &mut f64,
        var_uc_vtmp_slot: &mut f64,
        var_uc_vtmp_rv_slot: &mut f64,
        var_uc_wl2_slot: &mut f64,
        var_uc_wl2_rv_slot: &mut f64,
        var_uc_wvth0_slot: &mut f64,
        var_uc_wvth0_rv_slot: &mut f64,
        var_wbin_slot: &mut f64,
        var_wbin_rv_slot: &mut f64,
        var_wg_slot: &mut f64,
        var_wg_rv_slot: &mut f64,
        var_wgate_slot: &mut f64,
        var_wgate_rv_slot: &mut f64,
    ) {
        let mut var_guard158: f64 = *var_guard158_slot;
        let mut var_guard158_rv: f64 = *var_guard158_rv_slot;
        let mut var_guard159: f64 = *var_guard159_slot;
        let mut var_guard159_rv: f64 = *var_guard159_rv_slot;
        let mut var_guard168: f64 = *var_guard168_slot;
        let mut var_guard168_rv: f64 = *var_guard168_rv_slot;
        let mut var_guard169: f64 = *var_guard169_slot;
        let mut var_guard169_rv: f64 = *var_guard169_rv_slot;
        let mut var_ktnom: f64 = *var_ktnom_slot;
        let mut var_ktnom_rv: f64 = *var_ktnom_rv_slot;
        let mut var_lbin: f64 = *var_lbin_slot;
        let mut var_lbin_rv: f64 = *var_lbin_rv_slot;
        let mut var_lg: f64 = *var_lg_slot;
        let mut var_lg_rv: f64 = *var_lg_rv_slot;
        let mut var_lgate: f64 = *var_lgate_slot;
        let mut var_lgate_rv: f64 = *var_lgate_rv_slot;
        let mut var_lwbin: f64 = *var_lwbin_slot;
        let mut var_lwbin_rv: f64 = *var_lwbin_rv_slot;
        let mut var_mks_cit: f64 = *var_mks_cit_slot;
        let mut var_mks_cit_rv: f64 = *var_mks_cit_rv_slot;
        let mut var_mks_dly3: f64 = *var_mks_dly3_slot;
        let mut var_mks_dly3_rv: f64 = *var_mks_dly3_rv_slot;
        let mut var_mks_dlyov: f64 = *var_mks_dlyov_slot;
        let mut var_mks_dlyov_dn0: f64 = *var_mks_dlyov_dn0_slot;
        let mut var_mks_dlyov_dn10: f64 = *var_mks_dlyov_dn10_slot;
        let mut var_mks_dlyov_dn13: f64 = *var_mks_dlyov_dn13_slot;
        let mut var_mks_dlyov_dn2: f64 = *var_mks_dlyov_dn2_slot;
        let mut var_mks_dlyov_dn4: f64 = *var_mks_dlyov_dn4_slot;
        let mut var_mks_dlyov_dn5: f64 = *var_mks_dlyov_dn5_slot;
        let mut var_mks_dlyov_dn6: f64 = *var_mks_dlyov_dn6_slot;
        let mut var_mks_dlyov_dn7: f64 = *var_mks_dlyov_dn7_slot;
        let mut var_mks_dlyov_dn8: f64 = *var_mks_dlyov_dn8_slot;
        let mut var_mks_dlyov_dn9: f64 = *var_mks_dlyov_dn9_slot;
        let mut var_mks_dlyov_rv: f64 = *var_mks_dlyov_rv_slot;
        let mut var_mks_gleak4: f64 = *var_mks_gleak4_slot;
        let mut var_mks_gleak4_rv: f64 = *var_mks_gleak4_rv_slot;
        let mut var_mks_gleak5: f64 = *var_mks_gleak5_slot;
        let mut var_mks_gleak5_rv: f64 = *var_mks_gleak5_rv_slot;
        let mut var_mks_gleak7: f64 = *var_mks_gleak7_slot;
        let mut var_mks_gleak7_rv: f64 = *var_mks_gleak7_rv_slot;
        let mut var_mks_glksd3: f64 = *var_mks_glksd3_slot;
        let mut var_mks_glksd3_rv: f64 = *var_mks_glksd3_rv_slot;
        let mut var_mks_ll: f64 = *var_mks_ll_slot;
        let mut var_mks_ll_rv: f64 = *var_mks_ll_rv_slot;
        let mut var_mks_nsubsub: f64 = *var_mks_nsubsub_slot;
        let mut var_mks_nsubsub_rv: f64 = *var_mks_nsubsub_rv_slot;
        let mut var_mks_ovslp: f64 = *var_mks_ovslp_slot;
        let mut var_mks_ovslp_rv: f64 = *var_mks_ovslp_rv_slot;
        let mut var_mks_rdrmue: f64 = *var_mks_rdrmue_slot;
        let mut var_mks_rdrmue_rv: f64 = *var_mks_rdrmue_rv_slot;
        let mut var_mks_rdrmues: f64 = *var_mks_rdrmues_slot;
        let mut var_mks_rdrmues_rv: f64 = *var_mks_rdrmues_rv_slot;
        let mut var_mks_rdrvmax: f64 = *var_mks_rdrvmax_slot;
        let mut var_mks_rdrvmax_rv: f64 = *var_mks_rdrvmax_rv_slot;
        let mut var_mks_rdrvmaxs: f64 = *var_mks_rdrvmaxs_slot;
        let mut var_mks_rdrvmaxs_rv: f64 = *var_mks_rdrvmaxs_rv_slot;
        let mut var_mks_rdtemp1: f64 = *var_mks_rdtemp1_slot;
        let mut var_mks_rdtemp1_rv: f64 = *var_mks_rdtemp1_rv_slot;
        let mut var_mks_rdtemp2: f64 = *var_mks_rdtemp2_slot;
        let mut var_mks_rdtemp2_rv: f64 = *var_mks_rdtemp2_rv_slot;
        let mut var_mks_rdvdtemp1: f64 = *var_mks_rdvdtemp1_slot;
        let mut var_mks_rdvdtemp1_rv: f64 = *var_mks_rdvdtemp1_rv_slot;
        let mut var_mks_rdvdtemp2: f64 = *var_mks_rdvdtemp2_slot;
        let mut var_mks_rdvdtemp2_rv: f64 = *var_mks_rdvdtemp2_rv_slot;
        let mut var_mks_slg: f64 = *var_mks_slg_slot;
        let mut var_mks_slg_rv: f64 = *var_mks_slg_rv_slot;
        let mut var_mks_slgl: f64 = *var_mks_slgl_slot;
        let mut var_mks_slgl_rv: f64 = *var_mks_slgl_rv_slot;
        let mut var_mks_sub1l: f64 = *var_mks_sub1l_slot;
        let mut var_mks_sub1l_rv: f64 = *var_mks_sub1l_rv_slot;
        let mut var_mks_sub2l: f64 = *var_mks_sub2l_slot;
        let mut var_mks_sub2l_rv: f64 = *var_mks_sub2l_rv_slot;
        let mut var_mks_subld2: f64 = *var_mks_subld2_slot;
        let mut var_mks_subld2_rv: f64 = *var_mks_subld2_rv_slot;
        let mut var_mks_svbsl: f64 = *var_mks_svbsl_slot;
        let mut var_mks_svbsl_rv: f64 = *var_mks_svbsl_rv_slot;
        let mut var_mks_svgsl: f64 = *var_mks_svgsl_slot;
        let mut var_mks_svgsl_rv: f64 = *var_mks_svgsl_rv_slot;
        let mut var_mks_svgsw: f64 = *var_mks_svgsw_slot;
        let mut var_mks_svgsw_rv: f64 = *var_mks_svgsw_rv_slot;
        let mut var_mks_wl: f64 = *var_mks_wl_slot;
        let mut var_mks_wl_rv: f64 = *var_mks_wl_rv_slot;
        let mut var_uc_bgtmp1: f64 = *var_uc_bgtmp1_slot;
        let mut var_uc_bgtmp1_rv: f64 = *var_uc_bgtmp1_rv_slot;
        let mut var_uc_bgtmp2: f64 = *var_uc_bgtmp2_slot;
        let mut var_uc_bgtmp2_rv: f64 = *var_uc_bgtmp2_rv_slot;
        let mut var_uc_depleak: f64 = *var_uc_depleak_slot;
        let mut var_uc_depleak_dn0: f64 = *var_uc_depleak_dn0_slot;
        let mut var_uc_depleak_dn10: f64 = *var_uc_depleak_dn10_slot;
        let mut var_uc_depleak_dn13: f64 = *var_uc_depleak_dn13_slot;
        let mut var_uc_depleak_dn2: f64 = *var_uc_depleak_dn2_slot;
        let mut var_uc_depleak_dn4: f64 = *var_uc_depleak_dn4_slot;
        let mut var_uc_depleak_dn5: f64 = *var_uc_depleak_dn5_slot;
        let mut var_uc_depleak_dn6: f64 = *var_uc_depleak_dn6_slot;
        let mut var_uc_depleak_dn7: f64 = *var_uc_depleak_dn7_slot;
        let mut var_uc_depleak_dn8: f64 = *var_uc_depleak_dn8_slot;
        let mut var_uc_depleak_dn9: f64 = *var_uc_depleak_dn9_slot;
        let mut var_uc_depleak_rv: f64 = *var_uc_depleak_rv_slot;
        let mut var_uc_depmueph1: f64 = *var_uc_depmueph1_slot;
        let mut var_uc_depmueph1_rv: f64 = *var_uc_depmueph1_rv_slot;
        let mut var_uc_eg0: f64 = *var_uc_eg0_slot;
        let mut var_uc_eg0_rv: f64 = *var_uc_eg0_rv_slot;
        let mut var_uc_muecb0: f64 = *var_uc_muecb0_slot;
        let mut var_uc_muecb0_rv: f64 = *var_uc_muecb0_rv_slot;
        let mut var_uc_muecb1: f64 = *var_uc_muecb1_slot;
        let mut var_uc_muecb1_rv: f64 = *var_uc_muecb1_rv_slot;
        let mut var_uc_mueph1: f64 = *var_uc_mueph1_slot;
        let mut var_uc_mueph1_rv: f64 = *var_uc_mueph1_rv_slot;
        let mut var_uc_muesr1: f64 = *var_uc_muesr1_slot;
        let mut var_uc_muesr1_rv: f64 = *var_uc_muesr1_rv_slot;
        let mut var_uc_muetmp: f64 = *var_uc_muetmp_slot;
        let mut var_uc_muetmp_rv: f64 = *var_uc_muetmp_rv_slot;
        let mut var_uc_ndep: f64 = *var_uc_ndep_slot;
        let mut var_uc_ndep_rv: f64 = *var_uc_ndep_rv_slot;
        let mut var_uc_ndepm: f64 = *var_uc_ndepm_slot;
        let mut var_uc_ndepm_dn0: f64 = *var_uc_ndepm_dn0_slot;
        let mut var_uc_ndepm_dn10: f64 = *var_uc_ndepm_dn10_slot;
        let mut var_uc_ndepm_dn13: f64 = *var_uc_ndepm_dn13_slot;
        let mut var_uc_ndepm_dn2: f64 = *var_uc_ndepm_dn2_slot;
        let mut var_uc_ndepm_dn4: f64 = *var_uc_ndepm_dn4_slot;
        let mut var_uc_ndepm_dn5: f64 = *var_uc_ndepm_dn5_slot;
        let mut var_uc_ndepm_dn6: f64 = *var_uc_ndepm_dn6_slot;
        let mut var_uc_ndepm_dn7: f64 = *var_uc_ndepm_dn7_slot;
        let mut var_uc_ndepm_dn8: f64 = *var_uc_ndepm_dn8_slot;
        let mut var_uc_ndepm_dn9: f64 = *var_uc_ndepm_dn9_slot;
        let mut var_uc_ndepm_rv: f64 = *var_uc_ndepm_rv_slot;
        let mut var_uc_ninv: f64 = *var_uc_ninv_slot;
        let mut var_uc_ninv_rv: f64 = *var_uc_ninv_rv_slot;
        let mut var_uc_njunc: f64 = *var_uc_njunc_slot;
        let mut var_uc_njunc_rv: f64 = *var_uc_njunc_rv_slot;
        let mut var_uc_nover: f64 = *var_uc_nover_slot;
        let mut var_uc_nover_rv: f64 = *var_uc_nover_rv_slot;
        let mut var_uc_novers: f64 = *var_uc_novers_slot;
        let mut var_uc_novers_rv: f64 = *var_uc_novers_rv_slot;
        let mut var_uc_nsubc: f64 = *var_uc_nsubc_slot;
        let mut var_uc_nsubc_rv: f64 = *var_uc_nsubc_rv_slot;
        let mut var_uc_nsubp: f64 = *var_uc_nsubp_slot;
        let mut var_uc_nsubp_rv: f64 = *var_uc_nsubp_rv_slot;
        let mut var_uc_pgd1: f64 = *var_uc_pgd1_slot;
        let mut var_uc_pgd1_rv: f64 = *var_uc_pgd1_rv_slot;
        let mut var_uc_sc1: f64 = *var_uc_sc1_slot;
        let mut var_uc_sc1_rv: f64 = *var_uc_sc1_rv_slot;
        let mut var_uc_sc2: f64 = *var_uc_sc2_slot;
        let mut var_uc_sc2_rv: f64 = *var_uc_sc2_rv_slot;
        let mut var_uc_sc3: f64 = *var_uc_sc3_slot;
        let mut var_uc_sc3_rv: f64 = *var_uc_sc3_rv_slot;
        let mut var_uc_scp1: f64 = *var_uc_scp1_slot;
        let mut var_uc_scp1_rv: f64 = *var_uc_scp1_rv_slot;
        let mut var_uc_scp2: f64 = *var_uc_scp2_slot;
        let mut var_uc_scp2_rv: f64 = *var_uc_scp2_rv_slot;
        let mut var_uc_scp3: f64 = *var_uc_scp3_slot;
        let mut var_uc_scp3_rv: f64 = *var_uc_scp3_rv_slot;
        let mut var_uc_toxb: f64 = *var_uc_toxb_slot;
        let mut var_uc_toxb_rv: f64 = *var_uc_toxb_rv_slot;
        let mut var_uc_vfbc: f64 = *var_uc_vfbc_slot;
        let mut var_uc_vfbc_rv: f64 = *var_uc_vfbc_rv_slot;
        let mut var_uc_vfbover: f64 = *var_uc_vfbover_slot;
        let mut var_uc_vfbover_rv: f64 = *var_uc_vfbover_rv_slot;
        let mut var_uc_vmax: f64 = *var_uc_vmax_slot;
        let mut var_uc_vmax_rv: f64 = *var_uc_vmax_rv_slot;
        let mut var_uc_vtmp: f64 = *var_uc_vtmp_slot;
        let mut var_uc_vtmp_rv: f64 = *var_uc_vtmp_rv_slot;
        let mut var_uc_wl2: f64 = *var_uc_wl2_slot;
        let mut var_uc_wl2_rv: f64 = *var_uc_wl2_rv_slot;
        let mut var_uc_wvth0: f64 = *var_uc_wvth0_slot;
        let mut var_uc_wvth0_rv: f64 = *var_uc_wvth0_rv_slot;
        let mut var_wbin: f64 = *var_wbin_slot;
        let mut var_wbin_rv: f64 = *var_wbin_rv_slot;
        let mut var_wg: f64 = *var_wg_slot;
        let mut var_wg_rv: f64 = *var_wg_rv_slot;
        let mut var_wgate: f64 = *var_wgate_slot;
        let mut var_wgate_rv: f64 = *var_wgate_rv_slot;

        let (assign7360_e3034,) = {
    if (((var_guard110 == 0.0) && (var_guard139 != 0.0)) && (var_guard155 != 0.0)) {
        (2000000000.0,)
    } else {
        (var_uc_depmueph1,)
    }
};
        var_uc_depmueph1 = assign7360_e3034;
        var_uc_depmueph1_rv = 0.0;

        let assign7390_e3047: f64 = if var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        var_guard158 = assign7390_e3047;
        var_guard158_rv = 0.0;

        let (assign7400_e3056, assign7400_e3056_d_n0, assign7400_e3056_d_n2, assign7400_e3056_d_n4, assign7400_e3056_d_n5, assign7400_e3056_d_n6, assign7400_e3056_d_n7, assign7400_e3056_d_n8, assign7400_e3056_d_n9, assign7400_e3056_d_n10, assign7400_e3056_d_n13,) = {
    if (((var_guard110 == 0.0) && (var_guard139 != 0.0)) && (var_guard158 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn13,)
    }
};
        var_uc_depleak = assign7400_e3056;
        var_uc_depleak_dn0 = assign7400_e3056_d_n0;
        var_uc_depleak_dn2 = assign7400_e3056_d_n2;
        var_uc_depleak_dn4 = assign7400_e3056_d_n4;
        var_uc_depleak_dn5 = assign7400_e3056_d_n5;
        var_uc_depleak_dn6 = assign7400_e3056_d_n6;
        var_uc_depleak_dn7 = assign7400_e3056_d_n7;
        var_uc_depleak_dn8 = assign7400_e3056_d_n8;
        var_uc_depleak_dn9 = assign7400_e3056_d_n9;
        var_uc_depleak_dn10 = assign7400_e3056_d_n10;
        var_uc_depleak_dn13 = assign7400_e3056_d_n13;
        var_uc_depleak_rv = 0.0;

        let assign7410_e3059: f64 = if var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        var_guard159 = assign7410_e3059;
        var_guard159_rv = 0.0;

        let (assign7420_e3068, assign7420_e3068_d_n0, assign7420_e3068_d_n2, assign7420_e3068_d_n4, assign7420_e3068_d_n5, assign7420_e3068_d_n6, assign7420_e3068_d_n7, assign7420_e3068_d_n8, assign7420_e3068_d_n9, assign7420_e3068_d_n10, assign7420_e3068_d_n13,) = {
    if (((var_guard110 == 0.0) && (var_guard139 != 0.0)) && (var_guard159 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn13,)
    }
};
        var_uc_depleak = assign7420_e3068;
        var_uc_depleak_dn0 = assign7420_e3068_d_n0;
        var_uc_depleak_dn2 = assign7420_e3068_d_n2;
        var_uc_depleak_dn4 = assign7420_e3068_d_n4;
        var_uc_depleak_dn5 = assign7420_e3068_d_n5;
        var_uc_depleak_dn6 = assign7420_e3068_d_n6;
        var_uc_depleak_dn7 = assign7420_e3068_d_n7;
        var_uc_depleak_dn8 = assign7420_e3068_d_n8;
        var_uc_depleak_dn9 = assign7420_e3068_d_n9;
        var_uc_depleak_dn10 = assign7420_e3068_d_n10;
        var_uc_depleak_dn13 = assign7420_e3068_d_n13;
        var_uc_depleak_rv = 0.0;

        var_uc_toxb = p.p96;
        var_uc_toxb_rv = 0.0;

        let assign7520_e3106: f64 = if var_uc_toxb < p.p95 { 1.0 } else { 0.0 };
        var_guard168 = assign7520_e3106;
        var_guard168_rv = 0.0;

        let (assign7530_e3110,) = {
    if (var_guard168 != 0.0) {
        (p.p95,)
    } else {
        (var_uc_toxb,)
    }
};
        var_uc_toxb = assign7530_e3110;
        var_uc_toxb_rv = 0.0;

        let assign7540_e3113: f64 = if var_uc_toxb > 5e-7 { 1.0 } else { 0.0 };
        var_guard169 = assign7540_e3113;
        var_guard169_rv = 0.0;

        let (assign7550_e3117,) = {
    if (var_guard169 != 0.0) {
        (5e-7,)
    } else {
        (var_uc_toxb,)
    }
};
        var_uc_toxb = assign7550_e3117;
        var_uc_toxb_rv = 0.0;

        let assign7560_e3121: f64 = (100.0_f64).powf(p.p122);
        let assign7560_e3122: f64 = (p.p120 / assign7560_e3121);
        var_mks_ll = assign7560_e3122;
        var_mks_ll_rv = 0.0;

        let assign7570_e3126: f64 = (100.0_f64).powf(p.p129);
        let assign7570_e3127: f64 = (p.p123 / assign7570_e3126);
        var_mks_wl = assign7570_e3127;
        var_mks_wl_rv = 0.0;

        let assign7580_e3131: f64 = (100.0_f64).powf(p.p199);
        let assign7580_e3132: f64 = (p.p198 / assign7580_e3131);
        var_mks_svgsl = assign7580_e3132;
        var_mks_svgsl_rv = 0.0;

        let assign7590_e3136: f64 = (100.0_f64).powf(p.p201);
        let assign7590_e3137: f64 = (p.p200 / assign7590_e3136);
        var_mks_svgsw = assign7590_e3137;
        var_mks_svgsw_rv = 0.0;

        let assign7600_e3141: f64 = (100.0_f64).powf(p.p184);
        let assign7600_e3142: f64 = (p.p183 / assign7600_e3141);
        var_mks_svbsl = assign7600_e3142;
        var_mks_svbsl_rv = 0.0;

        let assign7610_e3146: f64 = (100.0_f64).powf(p.p203);
        let assign7610_e3147: f64 = (p.p202 / assign7610_e3146);
        var_mks_slgl = assign7610_e3147;
        var_mks_slgl_rv = 0.0;

        let assign7620_e3151: f64 = (100.0_f64).powf(p.p191);
        let assign7620_e3152: f64 = (p.p190 / assign7620_e3151);
        var_mks_sub1l = assign7620_e3152;
        var_mks_sub1l_rv = 0.0;

        let assign7630_e3155: f64 = (p.p186 / 100.0);
        var_mks_slg = assign7630_e3155;
        var_mks_slg_rv = 0.0;

        let assign7640_e3158: f64 = (p.p192 / 100.0);
        var_mks_sub2l = assign7640_e3158;
        var_mks_sub2l_rv = 0.0;

        let assign7650_e3161: f64 = (p.p73 * 100.0);
        var_mks_subld2 = assign7650_e3161;
        var_mks_subld2_rv = 0.0;

        let assign7660_e3164: f64 = (p.p311 / 100.0);
        var_mks_rdtemp1 = assign7660_e3164;
        var_mks_rdtemp1_rv = 0.0;

        let assign7670_e3167: f64 = (p.p312 / 100.0);
        var_mks_rdtemp2 = assign7670_e3167;
        var_mks_rdtemp2_rv = 0.0;

        let assign7680_e3170: f64 = (p.p313 / 100.0);
        var_mks_rdvdtemp1 = assign7680_e3170;
        var_mks_rdvdtemp1_rv = 0.0;

        let assign7690_e3173: f64 = (p.p314 / 100.0);
        var_mks_rdvdtemp2 = assign7690_e3173;
        var_mks_rdvdtemp2_rv = 0.0;

        let assign7700_e3176: f64 = (p.p336 / 1e-6);
        var_mks_nsubsub = assign7700_e3176;
        var_mks_nsubsub_rv = 0.0;

        let assign7710_e3179: f64 = (p.p255 * 100.0);
        var_mks_glksd3 = assign7710_e3179;
        var_mks_glksd3_rv = 0.0;

        let assign7720_e3182: f64 = (p.p248 * 100.0);
        var_mks_gleak4 = assign7720_e3182;
        var_mks_gleak4_rv = 0.0;

        let assign7730_e3185: f64 = (p.p249 * 100.0);
        var_mks_gleak5 = assign7730_e3185;
        var_mks_gleak5_rv = 0.0;

        let assign7740_e3188: f64 = (p.p251 / 10000.0);
        var_mks_gleak7 = assign7740_e3188;
        var_mks_gleak7_rv = 0.0;

        let assign7750_e3191: f64 = (p.p266 * 10000.0);
        var_mks_cit = assign7750_e3191;
        var_mks_cit_rv = 0.0;

        let assign7760_e3194: f64 = (p.p275 / 100.0);
        var_mks_ovslp = assign7760_e3194;
        var_mks_ovslp_rv = 0.0;

        let assign7770_e3197: f64 = (p.p272 / 10000.0);
        var_mks_dly3 = assign7770_e3197;
        var_mks_dly3_rv = 0.0;

        let assign7780_e3200: f64 = (p.p273 / 10000.0);
        var_mks_dlyov = assign7780_e3200;
        var_mks_dlyov_dn0 = 0.0;
        var_mks_dlyov_dn2 = 0.0;
        var_mks_dlyov_dn4 = 0.0;
        var_mks_dlyov_dn5 = 0.0;
        var_mks_dlyov_dn6 = 0.0;
        var_mks_dlyov_dn7 = 0.0;
        var_mks_dlyov_dn8 = 0.0;
        var_mks_dlyov_dn9 = 0.0;
        var_mks_dlyov_dn10 = 0.0;
        var_mks_dlyov_dn13 = 0.0;
        var_mks_dlyov_rv = 0.0;

        let assign7800_e3206: f64 = (p.p409 / 10000.0);
        var_mks_rdrmue = assign7800_e3206;
        var_mks_rdrmue_rv = 0.0;

        let assign7810_e3209: f64 = (p.p412 / 100.0);
        var_mks_rdrvmax = assign7810_e3209;
        var_mks_rdrvmax_rv = 0.0;

        let assign7820_e3212: f64 = (p.p413 / 10000.0);
        var_mks_rdrmues = assign7820_e3212;
        var_mks_rdrmues_rv = 0.0;

        let assign7830_e3215: f64 = (p.p414 / 100.0);
        var_mks_rdrvmaxs = assign7830_e3215;
        var_mks_rdrvmaxs_rv = 0.0;

        let assign7840_e3218: f64 = (var_uc_ndepm / 1e-6);
        var_uc_ndepm = assign7840_e3218;
        var_uc_ndepm_dn0 = (var_uc_ndepm_dn0 / 1e-6);
        var_uc_ndepm_dn2 = (var_uc_ndepm_dn2 / 1e-6);
        var_uc_ndepm_dn4 = (var_uc_ndepm_dn4 / 1e-6);
        var_uc_ndepm_dn5 = (var_uc_ndepm_dn5 / 1e-6);
        var_uc_ndepm_dn6 = (var_uc_ndepm_dn6 / 1e-6);
        var_uc_ndepm_dn7 = (var_uc_ndepm_dn7 / 1e-6);
        var_uc_ndepm_dn8 = (var_uc_ndepm_dn8 / 1e-6);
        var_uc_ndepm_dn9 = (var_uc_ndepm_dn9 / 1e-6);
        var_uc_ndepm_dn10 = (var_uc_ndepm_dn10 / 1e-6);
        var_uc_ndepm_dn13 = (var_uc_ndepm_dn13 / 1e-6);
        var_uc_ndepm_rv = 0.0;

        let assign7850_e3221: f64 = (p.p453 / 1e-6);
        var_uc_njunc = assign7850_e3221;
        var_uc_njunc_rv = 0.0;

        let assign7860_e3224: f64 = (p.p274 + 273.15);
        var_ktnom = assign7860_e3224;
        var_ktnom_rv = 0.0;

        let assign7910_e3247: f64 = (p.p0 + p.p116);
        var_lgate = assign7910_e3247;
        var_lgate_rv = 0.0;

        let assign7920_e3250: f64 = (p.p1 / p.p7);
        let assign7920_e3252: f64 = (assign7920_e3250 + p.p117);
        var_wgate = assign7920_e3252;
        var_wgate_rv = 0.0;

        let assign8070_e3352: f64 = (var_lgate * 1000000.0);
        var_lg = assign8070_e3352;
        var_lg_rv = 0.0;

        let assign8080_e3355: f64 = (var_wgate * 1000000.0);
        var_wg = assign8080_e3355;
        var_wg_rv = 0.0;

        let assign8090_e3358: f64 = (var_lg).powf(p.p553);
        var_lbin = assign8090_e3358;
        var_lbin_rv = 0.0;

        let assign8100_e3361: f64 = (var_wg).powf(p.p554);
        var_wbin = assign8100_e3361;
        var_wbin_rv = 0.0;

        let assign8110_e3364: f64 = (var_lbin * var_wbin);
        var_lwbin = assign8110_e3364;
        var_lwbin_rv = 0.0;

        let assign8120_e3368: f64 = (p.p555 / var_lbin);
        let assign8120_e3369: f64 = (p.p89 + assign8120_e3368);
        let assign8120_e3372: f64 = (p.p643 / var_wbin);
        let assign8120_e3373: f64 = (assign8120_e3369 + assign8120_e3372);
        let assign8120_e3376: f64 = (p.p731 / var_lwbin);
        let assign8120_e3377: f64 = (assign8120_e3373 + assign8120_e3376);
        var_uc_vmax = assign8120_e3377;
        var_uc_vmax_rv = 0.0;

        let assign8130_e3381: f64 = (p.p556 / var_lbin);
        let assign8130_e3382: f64 = (p.p92 + assign8130_e3381);
        let assign8130_e3385: f64 = (p.p644 / var_wbin);
        let assign8130_e3386: f64 = (assign8130_e3382 + assign8130_e3385);
        let assign8130_e3389: f64 = (p.p732 / var_lwbin);
        let assign8130_e3390: f64 = (assign8130_e3386 + assign8130_e3389);
        var_uc_bgtmp1 = assign8130_e3390;
        var_uc_bgtmp1_rv = 0.0;

        let assign8140_e3394: f64 = (p.p557 / var_lbin);
        let assign8140_e3395: f64 = (p.p93 + assign8140_e3394);
        let assign8140_e3398: f64 = (p.p645 / var_wbin);
        let assign8140_e3399: f64 = (assign8140_e3395 + assign8140_e3398);
        let assign8140_e3402: f64 = (p.p733 / var_lwbin);
        let assign8140_e3403: f64 = (assign8140_e3399 + assign8140_e3402);
        var_uc_bgtmp2 = assign8140_e3403;
        var_uc_bgtmp2_rv = 0.0;

        let assign8150_e3407: f64 = (p.p558 / var_lbin);
        let assign8150_e3408: f64 = (p.p94 + assign8150_e3407);
        let assign8150_e3411: f64 = (p.p646 / var_wbin);
        let assign8150_e3412: f64 = (assign8150_e3408 + assign8150_e3411);
        let assign8150_e3415: f64 = (p.p734 / var_lwbin);
        let assign8150_e3416: f64 = (assign8150_e3412 + assign8150_e3415);
        var_uc_eg0 = assign8150_e3416;
        var_uc_eg0_rv = 0.0;

        let assign8160_e3420: f64 = (p.p559 / var_lbin);
        let assign8160_e3421: f64 = (p.p110 + assign8160_e3420);
        let assign8160_e3424: f64 = (p.p647 / var_wbin);
        let assign8160_e3425: f64 = (assign8160_e3421 + assign8160_e3424);
        let assign8160_e3428: f64 = (p.p735 / var_lwbin);
        let assign8160_e3429: f64 = (assign8160_e3425 + assign8160_e3428);
        var_uc_vfbover = assign8160_e3429;
        var_uc_vfbover_rv = 0.0;

        let assign8170_e3433: f64 = (p.p560 / var_lbin);
        let assign8170_e3434: f64 = (p.p111 + assign8170_e3433);
        let assign8170_e3437: f64 = (p.p648 / var_wbin);
        let assign8170_e3438: f64 = (assign8170_e3434 + assign8170_e3437);
        let assign8170_e3441: f64 = (p.p736 / var_lwbin);
        let assign8170_e3442: f64 = (assign8170_e3438 + assign8170_e3441);
        var_uc_nover = assign8170_e3442;
        var_uc_nover_rv = 0.0;

        let assign8180_e3446: f64 = (p.p561 / var_lbin);
        let assign8180_e3447: f64 = (p.p112 + assign8180_e3446);
        let assign8180_e3450: f64 = (p.p649 / var_wbin);
        let assign8180_e3451: f64 = (assign8180_e3447 + assign8180_e3450);
        let assign8180_e3454: f64 = (p.p737 / var_lwbin);
        let assign8180_e3455: f64 = (assign8180_e3451 + assign8180_e3454);
        var_uc_novers = assign8180_e3455;
        var_uc_novers_rv = 0.0;

        let assign8190_e3459: f64 = (p.p562 / var_lbin);
        let assign8190_e3460: f64 = (p.p126 + assign8190_e3459);
        let assign8190_e3463: f64 = (p.p650 / var_wbin);
        let assign8190_e3464: f64 = (assign8190_e3460 + assign8190_e3463);
        let assign8190_e3467: f64 = (p.p738 / var_lwbin);
        let assign8190_e3468: f64 = (assign8190_e3464 + assign8190_e3467);
        var_uc_wl2 = assign8190_e3468;
        var_uc_wl2_rv = 0.0;

        let assign8200_e3472: f64 = (p.p563 / var_lbin);
        let assign8200_e3473: f64 = (p.p136 + assign8200_e3472);
        let assign8200_e3476: f64 = (p.p651 / var_wbin);
        let assign8200_e3477: f64 = (assign8200_e3473 + assign8200_e3476);
        let assign8200_e3480: f64 = (p.p739 / var_lwbin);
        let assign8200_e3481: f64 = (assign8200_e3477 + assign8200_e3480);
        var_uc_vfbc = assign8200_e3481;
        var_uc_vfbc_rv = 0.0;

        let assign8210_e3485: f64 = (p.p564 / var_lbin);
        let assign8210_e3486: f64 = (p.p138 + assign8210_e3485);
        let assign8210_e3489: f64 = (p.p652 / var_wbin);
        let assign8210_e3490: f64 = (assign8210_e3486 + assign8210_e3489);
        let assign8210_e3493: f64 = (p.p740 / var_lwbin);
        let assign8210_e3494: f64 = (assign8210_e3490 + assign8210_e3493);
        var_uc_nsubc = assign8210_e3494;
        var_uc_nsubc_rv = 0.0;

        let assign8220_e3498: f64 = (p.p565 / var_lbin);
        let assign8220_e3499: f64 = (p.p141 + assign8220_e3498);
        let assign8220_e3502: f64 = (p.p653 / var_wbin);
        let assign8220_e3503: f64 = (assign8220_e3499 + assign8220_e3502);
        let assign8220_e3506: f64 = (p.p741 / var_lwbin);
        let assign8220_e3507: f64 = (assign8220_e3503 + assign8220_e3506);
        var_uc_nsubp = assign8220_e3507;
        var_uc_nsubp_rv = 0.0;

        let assign8230_e3511: f64 = (p.p566 / var_lbin);
        let assign8230_e3512: f64 = (p.p144 + assign8230_e3511);
        let assign8230_e3515: f64 = (p.p654 / var_wbin);
        let assign8230_e3516: f64 = (assign8230_e3512 + assign8230_e3515);
        let assign8230_e3519: f64 = (p.p742 / var_lwbin);
        let assign8230_e3520: f64 = (assign8230_e3516 + assign8230_e3519);
        var_uc_scp1 = assign8230_e3520;
        var_uc_scp1_rv = 0.0;

        let assign8240_e3524: f64 = (p.p567 / var_lbin);
        let assign8240_e3525: f64 = (p.p145 + assign8240_e3524);
        let assign8240_e3528: f64 = (p.p655 / var_wbin);
        let assign8240_e3529: f64 = (assign8240_e3525 + assign8240_e3528);
        let assign8240_e3532: f64 = (p.p743 / var_lwbin);
        let assign8240_e3533: f64 = (assign8240_e3529 + assign8240_e3532);
        var_uc_scp2 = assign8240_e3533;
        var_uc_scp2_rv = 0.0;

        let assign8250_e3537: f64 = (p.p568 / var_lbin);
        let assign8250_e3538: f64 = (p.p146 + assign8250_e3537);
        let assign8250_e3541: f64 = (p.p656 / var_wbin);
        let assign8250_e3542: f64 = (assign8250_e3538 + assign8250_e3541);
        let assign8250_e3545: f64 = (p.p744 / var_lwbin);
        let assign8250_e3546: f64 = (assign8250_e3542 + assign8250_e3545);
        var_uc_scp3 = assign8250_e3546;
        var_uc_scp3_rv = 0.0;

        let assign8260_e3550: f64 = (p.p569 / var_lbin);
        let assign8260_e3551: f64 = (p.p147 + assign8260_e3550);
        let assign8260_e3554: f64 = (p.p657 / var_wbin);
        let assign8260_e3555: f64 = (assign8260_e3551 + assign8260_e3554);
        let assign8260_e3558: f64 = (p.p745 / var_lwbin);
        let assign8260_e3559: f64 = (assign8260_e3555 + assign8260_e3558);
        var_uc_sc1 = assign8260_e3559;
        var_uc_sc1_rv = 0.0;

        let assign8270_e3563: f64 = (p.p570 / var_lbin);
        let assign8270_e3564: f64 = (p.p148 + assign8270_e3563);
        let assign8270_e3567: f64 = (p.p658 / var_wbin);
        let assign8270_e3568: f64 = (assign8270_e3564 + assign8270_e3567);
        let assign8270_e3571: f64 = (p.p746 / var_lwbin);
        let assign8270_e3572: f64 = (assign8270_e3568 + assign8270_e3571);
        var_uc_sc2 = assign8270_e3572;
        var_uc_sc2_rv = 0.0;

        let assign8280_e3576: f64 = (p.p571 / var_lbin);
        let assign8280_e3577: f64 = (p.p149 + assign8280_e3576);
        let assign8280_e3580: f64 = (p.p659 / var_wbin);
        let assign8280_e3581: f64 = (assign8280_e3577 + assign8280_e3580);
        let assign8280_e3584: f64 = (p.p747 / var_lwbin);
        let assign8280_e3585: f64 = (assign8280_e3581 + assign8280_e3584);
        var_uc_sc3 = assign8280_e3585;
        var_uc_sc3_rv = 0.0;

        let assign8290_e3589: f64 = (p.p572 / var_lbin);
        let assign8290_e3590: f64 = (p.p151 + assign8290_e3589);
        let assign8290_e3593: f64 = (p.p660 / var_wbin);
        let assign8290_e3594: f64 = (assign8290_e3590 + assign8290_e3593);
        let assign8290_e3597: f64 = (p.p748 / var_lwbin);
        let assign8290_e3598: f64 = (assign8290_e3594 + assign8290_e3597);
        var_uc_pgd1 = assign8290_e3598;
        var_uc_pgd1_rv = 0.0;

        let assign8300_e3602: f64 = (p.p573 / var_lbin);
        let assign8300_e3603: f64 = (p.p154 + assign8300_e3602);
        let assign8300_e3606: f64 = (p.p661 / var_wbin);
        let assign8300_e3607: f64 = (assign8300_e3603 + assign8300_e3606);
        let assign8300_e3610: f64 = (p.p749 / var_lwbin);
        let assign8300_e3611: f64 = (assign8300_e3607 + assign8300_e3610);
        var_uc_ndep = assign8300_e3611;
        var_uc_ndep_rv = 0.0;

        let assign8310_e3615: f64 = (p.p574 / var_lbin);
        let assign8310_e3616: f64 = (p.p157 + assign8310_e3615);
        let assign8310_e3619: f64 = (p.p662 / var_wbin);
        let assign8310_e3620: f64 = (assign8310_e3616 + assign8310_e3619);
        let assign8310_e3623: f64 = (p.p750 / var_lwbin);
        let assign8310_e3624: f64 = (assign8310_e3620 + assign8310_e3623);
        var_uc_ninv = assign8310_e3624;
        var_uc_ninv_rv = 0.0;

        let assign8320_e3628: f64 = (p.p575 / var_lbin);
        let assign8320_e3629: f64 = (p.p158 + assign8320_e3628);
        let assign8320_e3632: f64 = (p.p663 / var_wbin);
        let assign8320_e3633: f64 = (assign8320_e3629 + assign8320_e3632);
        let assign8320_e3636: f64 = (p.p751 / var_lwbin);
        let assign8320_e3637: f64 = (assign8320_e3633 + assign8320_e3636);
        var_uc_muecb0 = assign8320_e3637;
        var_uc_muecb0_rv = 0.0;

        let assign8330_e3641: f64 = (p.p576 / var_lbin);
        let assign8330_e3642: f64 = (p.p159 + assign8330_e3641);
        let assign8330_e3645: f64 = (p.p664 / var_wbin);
        let assign8330_e3646: f64 = (assign8330_e3642 + assign8330_e3645);
        let assign8330_e3649: f64 = (p.p752 / var_lwbin);
        let assign8330_e3650: f64 = (assign8330_e3646 + assign8330_e3649);
        var_uc_muecb1 = assign8330_e3650;
        var_uc_muecb1_rv = 0.0;

        let assign8340_e3654: f64 = (p.p577 / var_lbin);
        let assign8340_e3655: f64 = (p.p161 + assign8340_e3654);
        let assign8340_e3658: f64 = (p.p665 / var_wbin);
        let assign8340_e3659: f64 = (assign8340_e3655 + assign8340_e3658);
        let assign8340_e3662: f64 = (p.p753 / var_lwbin);
        let assign8340_e3663: f64 = (assign8340_e3659 + assign8340_e3662);
        var_uc_mueph1 = assign8340_e3663;
        var_uc_mueph1_rv = 0.0;

        let assign8350_e3667: f64 = (p.p578 / var_lbin);
        let assign8350_e3668: f64 = (p.p169 + assign8350_e3667);
        let assign8350_e3671: f64 = (p.p666 / var_wbin);
        let assign8350_e3672: f64 = (assign8350_e3668 + assign8350_e3671);
        let assign8350_e3675: f64 = (p.p754 / var_lwbin);
        let assign8350_e3676: f64 = (assign8350_e3672 + assign8350_e3675);
        var_uc_vtmp = assign8350_e3676;
        var_uc_vtmp_rv = 0.0;

        let assign8360_e3680: f64 = (p.p579 / var_lbin);
        let assign8360_e3681: f64 = (p.p170 + assign8360_e3680);
        let assign8360_e3684: f64 = (p.p667 / var_wbin);
        let assign8360_e3685: f64 = (assign8360_e3681 + assign8360_e3684);
        let assign8360_e3688: f64 = (p.p755 / var_lwbin);
        let assign8360_e3689: f64 = (assign8360_e3685 + assign8360_e3688);
        var_uc_wvth0 = assign8360_e3689;
        var_uc_wvth0_rv = 0.0;

        let assign8370_e3693: f64 = (p.p580 / var_lbin);
        let assign8370_e3694: f64 = (p.p172 + assign8370_e3693);
        let assign8370_e3697: f64 = (p.p668 / var_wbin);
        let assign8370_e3698: f64 = (assign8370_e3694 + assign8370_e3697);
        let assign8370_e3701: f64 = (p.p756 / var_lwbin);
        let assign8370_e3702: f64 = (assign8370_e3698 + assign8370_e3701);
        var_uc_muesr1 = assign8370_e3702;
        var_uc_muesr1_rv = 0.0;

        let assign8380_e3706: f64 = (p.p581 / var_lbin);
        let assign8380_e3707: f64 = (p.p177 + assign8380_e3706);
        let assign8380_e3710: f64 = (p.p669 / var_wbin);
        let assign8380_e3711: f64 = (assign8380_e3707 + assign8380_e3710);
        let assign8380_e3714: f64 = (p.p757 / var_lwbin);
        let assign8380_e3715: f64 = (assign8380_e3711 + assign8380_e3714);
        var_uc_muetmp = assign8380_e3715;
        var_uc_muetmp_rv = 0.0;

        *var_guard158_slot = var_guard158;
        *var_guard158_rv_slot = var_guard158_rv;
        *var_guard159_slot = var_guard159;
        *var_guard159_rv_slot = var_guard159_rv;
        *var_guard168_slot = var_guard168;
        *var_guard168_rv_slot = var_guard168_rv;
        *var_guard169_slot = var_guard169;
        *var_guard169_rv_slot = var_guard169_rv;
        *var_ktnom_slot = var_ktnom;
        *var_ktnom_rv_slot = var_ktnom_rv;
        *var_lbin_slot = var_lbin;
        *var_lbin_rv_slot = var_lbin_rv;
        *var_lg_slot = var_lg;
        *var_lg_rv_slot = var_lg_rv;
        *var_lgate_slot = var_lgate;
        *var_lgate_rv_slot = var_lgate_rv;
        *var_lwbin_slot = var_lwbin;
        *var_lwbin_rv_slot = var_lwbin_rv;
        *var_mks_cit_slot = var_mks_cit;
        *var_mks_cit_rv_slot = var_mks_cit_rv;
        *var_mks_dly3_slot = var_mks_dly3;
        *var_mks_dly3_rv_slot = var_mks_dly3_rv;
        *var_mks_dlyov_slot = var_mks_dlyov;
        *var_mks_dlyov_dn0_slot = var_mks_dlyov_dn0;
        *var_mks_dlyov_dn10_slot = var_mks_dlyov_dn10;
        *var_mks_dlyov_dn13_slot = var_mks_dlyov_dn13;
        *var_mks_dlyov_dn2_slot = var_mks_dlyov_dn2;
        *var_mks_dlyov_dn4_slot = var_mks_dlyov_dn4;
        *var_mks_dlyov_dn5_slot = var_mks_dlyov_dn5;
        *var_mks_dlyov_dn6_slot = var_mks_dlyov_dn6;
        *var_mks_dlyov_dn7_slot = var_mks_dlyov_dn7;
        *var_mks_dlyov_dn8_slot = var_mks_dlyov_dn8;
        *var_mks_dlyov_dn9_slot = var_mks_dlyov_dn9;
        *var_mks_dlyov_rv_slot = var_mks_dlyov_rv;
        *var_mks_gleak4_slot = var_mks_gleak4;
        *var_mks_gleak4_rv_slot = var_mks_gleak4_rv;
        *var_mks_gleak5_slot = var_mks_gleak5;
        *var_mks_gleak5_rv_slot = var_mks_gleak5_rv;
        *var_mks_gleak7_slot = var_mks_gleak7;
        *var_mks_gleak7_rv_slot = var_mks_gleak7_rv;
        *var_mks_glksd3_slot = var_mks_glksd3;
        *var_mks_glksd3_rv_slot = var_mks_glksd3_rv;
        *var_mks_ll_slot = var_mks_ll;
        *var_mks_ll_rv_slot = var_mks_ll_rv;
        *var_mks_nsubsub_slot = var_mks_nsubsub;
        *var_mks_nsubsub_rv_slot = var_mks_nsubsub_rv;
        *var_mks_ovslp_slot = var_mks_ovslp;
        *var_mks_ovslp_rv_slot = var_mks_ovslp_rv;
        *var_mks_rdrmue_slot = var_mks_rdrmue;
        *var_mks_rdrmue_rv_slot = var_mks_rdrmue_rv;
        *var_mks_rdrmues_slot = var_mks_rdrmues;
        *var_mks_rdrmues_rv_slot = var_mks_rdrmues_rv;
        *var_mks_rdrvmax_slot = var_mks_rdrvmax;
        *var_mks_rdrvmax_rv_slot = var_mks_rdrvmax_rv;
        *var_mks_rdrvmaxs_slot = var_mks_rdrvmaxs;
        *var_mks_rdrvmaxs_rv_slot = var_mks_rdrvmaxs_rv;
        *var_mks_rdtemp1_slot = var_mks_rdtemp1;
        *var_mks_rdtemp1_rv_slot = var_mks_rdtemp1_rv;
        *var_mks_rdtemp2_slot = var_mks_rdtemp2;
        *var_mks_rdtemp2_rv_slot = var_mks_rdtemp2_rv;
        *var_mks_rdvdtemp1_slot = var_mks_rdvdtemp1;
        *var_mks_rdvdtemp1_rv_slot = var_mks_rdvdtemp1_rv;
        *var_mks_rdvdtemp2_slot = var_mks_rdvdtemp2;
        *var_mks_rdvdtemp2_rv_slot = var_mks_rdvdtemp2_rv;
        *var_mks_slg_slot = var_mks_slg;
        *var_mks_slg_rv_slot = var_mks_slg_rv;
        *var_mks_slgl_slot = var_mks_slgl;
        *var_mks_slgl_rv_slot = var_mks_slgl_rv;
        *var_mks_sub1l_slot = var_mks_sub1l;
        *var_mks_sub1l_rv_slot = var_mks_sub1l_rv;
        *var_mks_sub2l_slot = var_mks_sub2l;
        *var_mks_sub2l_rv_slot = var_mks_sub2l_rv;
        *var_mks_subld2_slot = var_mks_subld2;
        *var_mks_subld2_rv_slot = var_mks_subld2_rv;
        *var_mks_svbsl_slot = var_mks_svbsl;
        *var_mks_svbsl_rv_slot = var_mks_svbsl_rv;
        *var_mks_svgsl_slot = var_mks_svgsl;
        *var_mks_svgsl_rv_slot = var_mks_svgsl_rv;
        *var_mks_svgsw_slot = var_mks_svgsw;
        *var_mks_svgsw_rv_slot = var_mks_svgsw_rv;
        *var_mks_wl_slot = var_mks_wl;
        *var_mks_wl_rv_slot = var_mks_wl_rv;
        *var_uc_bgtmp1_slot = var_uc_bgtmp1;
        *var_uc_bgtmp1_rv_slot = var_uc_bgtmp1_rv;
        *var_uc_bgtmp2_slot = var_uc_bgtmp2;
        *var_uc_bgtmp2_rv_slot = var_uc_bgtmp2_rv;
        *var_uc_depleak_slot = var_uc_depleak;
        *var_uc_depleak_dn0_slot = var_uc_depleak_dn0;
        *var_uc_depleak_dn10_slot = var_uc_depleak_dn10;
        *var_uc_depleak_dn13_slot = var_uc_depleak_dn13;
        *var_uc_depleak_dn2_slot = var_uc_depleak_dn2;
        *var_uc_depleak_dn4_slot = var_uc_depleak_dn4;
        *var_uc_depleak_dn5_slot = var_uc_depleak_dn5;
        *var_uc_depleak_dn6_slot = var_uc_depleak_dn6;
        *var_uc_depleak_dn7_slot = var_uc_depleak_dn7;
        *var_uc_depleak_dn8_slot = var_uc_depleak_dn8;
        *var_uc_depleak_dn9_slot = var_uc_depleak_dn9;
        *var_uc_depleak_rv_slot = var_uc_depleak_rv;
        *var_uc_depmueph1_slot = var_uc_depmueph1;
        *var_uc_depmueph1_rv_slot = var_uc_depmueph1_rv;
        *var_uc_eg0_slot = var_uc_eg0;
        *var_uc_eg0_rv_slot = var_uc_eg0_rv;
        *var_uc_muecb0_slot = var_uc_muecb0;
        *var_uc_muecb0_rv_slot = var_uc_muecb0_rv;
        *var_uc_muecb1_slot = var_uc_muecb1;
        *var_uc_muecb1_rv_slot = var_uc_muecb1_rv;
        *var_uc_mueph1_slot = var_uc_mueph1;
        *var_uc_mueph1_rv_slot = var_uc_mueph1_rv;
        *var_uc_muesr1_slot = var_uc_muesr1;
        *var_uc_muesr1_rv_slot = var_uc_muesr1_rv;
        *var_uc_muetmp_slot = var_uc_muetmp;
        *var_uc_muetmp_rv_slot = var_uc_muetmp_rv;
        *var_uc_ndep_slot = var_uc_ndep;
        *var_uc_ndep_rv_slot = var_uc_ndep_rv;
        *var_uc_ndepm_slot = var_uc_ndepm;
        *var_uc_ndepm_dn0_slot = var_uc_ndepm_dn0;
        *var_uc_ndepm_dn10_slot = var_uc_ndepm_dn10;
        *var_uc_ndepm_dn13_slot = var_uc_ndepm_dn13;
        *var_uc_ndepm_dn2_slot = var_uc_ndepm_dn2;
        *var_uc_ndepm_dn4_slot = var_uc_ndepm_dn4;
        *var_uc_ndepm_dn5_slot = var_uc_ndepm_dn5;
        *var_uc_ndepm_dn6_slot = var_uc_ndepm_dn6;
        *var_uc_ndepm_dn7_slot = var_uc_ndepm_dn7;
        *var_uc_ndepm_dn8_slot = var_uc_ndepm_dn8;
        *var_uc_ndepm_dn9_slot = var_uc_ndepm_dn9;
        *var_uc_ndepm_rv_slot = var_uc_ndepm_rv;
        *var_uc_ninv_slot = var_uc_ninv;
        *var_uc_ninv_rv_slot = var_uc_ninv_rv;
        *var_uc_njunc_slot = var_uc_njunc;
        *var_uc_njunc_rv_slot = var_uc_njunc_rv;
        *var_uc_nover_slot = var_uc_nover;
        *var_uc_nover_rv_slot = var_uc_nover_rv;
        *var_uc_novers_slot = var_uc_novers;
        *var_uc_novers_rv_slot = var_uc_novers_rv;
        *var_uc_nsubc_slot = var_uc_nsubc;
        *var_uc_nsubc_rv_slot = var_uc_nsubc_rv;
        *var_uc_nsubp_slot = var_uc_nsubp;
        *var_uc_nsubp_rv_slot = var_uc_nsubp_rv;
        *var_uc_pgd1_slot = var_uc_pgd1;
        *var_uc_pgd1_rv_slot = var_uc_pgd1_rv;
        *var_uc_sc1_slot = var_uc_sc1;
        *var_uc_sc1_rv_slot = var_uc_sc1_rv;
        *var_uc_sc2_slot = var_uc_sc2;
        *var_uc_sc2_rv_slot = var_uc_sc2_rv;
        *var_uc_sc3_slot = var_uc_sc3;
        *var_uc_sc3_rv_slot = var_uc_sc3_rv;
        *var_uc_scp1_slot = var_uc_scp1;
        *var_uc_scp1_rv_slot = var_uc_scp1_rv;
        *var_uc_scp2_slot = var_uc_scp2;
        *var_uc_scp2_rv_slot = var_uc_scp2_rv;
        *var_uc_scp3_slot = var_uc_scp3;
        *var_uc_scp3_rv_slot = var_uc_scp3_rv;
        *var_uc_toxb_slot = var_uc_toxb;
        *var_uc_toxb_rv_slot = var_uc_toxb_rv;
        *var_uc_vfbc_slot = var_uc_vfbc;
        *var_uc_vfbc_rv_slot = var_uc_vfbc_rv;
        *var_uc_vfbover_slot = var_uc_vfbover;
        *var_uc_vfbover_rv_slot = var_uc_vfbover_rv;
        *var_uc_vmax_slot = var_uc_vmax;
        *var_uc_vmax_rv_slot = var_uc_vmax_rv;
        *var_uc_vtmp_slot = var_uc_vtmp;
        *var_uc_vtmp_rv_slot = var_uc_vtmp_rv;
        *var_uc_wl2_slot = var_uc_wl2;
        *var_uc_wl2_rv_slot = var_uc_wl2_rv;
        *var_uc_wvth0_slot = var_uc_wvth0;
        *var_uc_wvth0_rv_slot = var_uc_wvth0_rv;
        *var_wbin_slot = var_wbin;
        *var_wbin_rv_slot = var_wbin_rv;
        *var_wg_slot = var_wg;
        *var_wg_rv_slot = var_wg_rv;
        *var_wgate_slot = var_wgate;
        *var_wgate_rv_slot = var_wgate_rv;
    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        var_lbin: f64,
        var_lwbin: f64,
        var_wbin: f64,
        var_uc_cgbo_slot: &mut f64,
        var_uc_cgbo_rv_slot: &mut f64,
        var_uc_cgdo_slot: &mut f64,
        var_uc_cgdo_rv_slot: &mut f64,
        var_uc_cgso_slot: &mut f64,
        var_uc_cgso_rv_slot: &mut f64,
        var_uc_clm1_slot: &mut f64,
        var_uc_clm1_rv_slot: &mut f64,
        var_uc_clm2_slot: &mut f64,
        var_uc_clm2_dn0_slot: &mut f64,
        var_uc_clm2_dn10_slot: &mut f64,
        var_uc_clm2_dn13_slot: &mut f64,
        var_uc_clm2_dn2_slot: &mut f64,
        var_uc_clm2_dn4_slot: &mut f64,
        var_uc_clm2_dn5_slot: &mut f64,
        var_uc_clm2_dn6_slot: &mut f64,
        var_uc_clm2_dn7_slot: &mut f64,
        var_uc_clm2_dn8_slot: &mut f64,
        var_uc_clm2_dn9_slot: &mut f64,
        var_uc_clm2_rv_slot: &mut f64,
        var_uc_clm3_slot: &mut f64,
        var_uc_clm3_rv_slot: &mut f64,
        var_uc_cvdsover_slot: &mut f64,
        var_uc_cvdsover_rv_slot: &mut f64,
        var_uc_fn1_slot: &mut f64,
        var_uc_fn1_rv_slot: &mut f64,
        var_uc_fn2_slot: &mut f64,
        var_uc_fn2_rv_slot: &mut f64,
        var_uc_fn3_slot: &mut f64,
        var_uc_fn3_rv_slot: &mut f64,
        var_uc_fvbs_slot: &mut f64,
        var_uc_fvbs_rv_slot: &mut f64,
        var_uc_gidl1_slot: &mut f64,
        var_uc_gidl1_rv_slot: &mut f64,
        var_uc_gidl2_slot: &mut f64,
        var_uc_gidl2_rv_slot: &mut f64,
        var_uc_gleak1_slot: &mut f64,
        var_uc_gleak1_rv_slot: &mut f64,
        var_uc_gleak2_slot: &mut f64,
        var_uc_gleak2_rv_slot: &mut f64,
        var_uc_gleak3_slot: &mut f64,
        var_uc_gleak3_rv_slot: &mut f64,
        var_uc_gleak6_slot: &mut f64,
        var_uc_gleak6_rv_slot: &mut f64,
        var_uc_glkb1_slot: &mut f64,
        var_uc_glkb1_rv_slot: &mut f64,
        var_uc_glkb2_slot: &mut f64,
        var_uc_glkb2_rv_slot: &mut f64,
        var_uc_glksd1_slot: &mut f64,
        var_uc_glksd1_rv_slot: &mut f64,
        var_uc_glksd2_slot: &mut f64,
        var_uc_glksd2_rv_slot: &mut f64,
        var_uc_ibpc1_slot: &mut f64,
        var_uc_ibpc1_rv_slot: &mut f64,
        var_uc_ibpc2_slot: &mut f64,
        var_uc_ibpc2_rv_slot: &mut f64,
        var_uc_muesti1_slot: &mut f64,
        var_uc_muesti1_rv_slot: &mut f64,
        var_uc_muesti2_slot: &mut f64,
        var_uc_muesti2_rv_slot: &mut f64,
        var_uc_muesti3_slot: &mut f64,
        var_uc_muesti3_rv_slot: &mut f64,
        var_uc_nfalp_slot: &mut f64,
        var_uc_nfalp_rv_slot: &mut f64,
        var_uc_npext_slot: &mut f64,
        var_uc_npext_rv_slot: &mut f64,
        var_uc_nsti_slot: &mut f64,
        var_uc_nsti_rv_slot: &mut f64,
        var_uc_nsubpsti1_slot: &mut f64,
        var_uc_nsubpsti1_rv_slot: &mut f64,
        var_uc_nsubpsti2_slot: &mut f64,
        var_uc_nsubpsti2_rv_slot: &mut f64,
        var_uc_nsubpsti3_slot: &mut f64,
        var_uc_nsubpsti3_rv_slot: &mut f64,
        var_uc_powrat_slot: &mut f64,
        var_uc_powrat_rv_slot: &mut f64,
        var_uc_rd_slot: &mut f64,
        var_uc_rd22_slot: &mut f64,
        var_uc_rd22_rv_slot: &mut f64,
        var_uc_rd23_slot: &mut f64,
        var_uc_rd23_rv_slot: &mut f64,
        var_uc_rd24_slot: &mut f64,
        var_uc_rd24_rv_slot: &mut f64,
        var_uc_rd_rv_slot: &mut f64,
        var_uc_rdict1_slot: &mut f64,
        var_uc_rdict1_rv_slot: &mut f64,
        var_uc_rdov13_slot: &mut f64,
        var_uc_rdov13_rv_slot: &mut f64,
        var_uc_rdslp1_slot: &mut f64,
        var_uc_rdslp1_rv_slot: &mut f64,
        var_uc_rdvb_slot: &mut f64,
        var_uc_rdvb_rv_slot: &mut f64,
        var_uc_scsti1_slot: &mut f64,
        var_uc_scsti1_rv_slot: &mut f64,
        var_uc_scsti2_slot: &mut f64,
        var_uc_scsti2_rv_slot: &mut f64,
        var_uc_sub1_slot: &mut f64,
        var_uc_sub1_rv_slot: &mut f64,
        var_uc_sub1snp_slot: &mut f64,
        var_uc_sub1snp_rv_slot: &mut f64,
        var_uc_sub2_slot: &mut f64,
        var_uc_sub2_rv_slot: &mut f64,
        var_uc_sub2snp_slot: &mut f64,
        var_uc_sub2snp_rv_slot: &mut f64,
        var_uc_svbs_slot: &mut f64,
        var_uc_svbs_rv_slot: &mut f64,
        var_uc_svds_slot: &mut f64,
        var_uc_svds_rv_slot: &mut f64,
        var_uc_svdssnp_slot: &mut f64,
        var_uc_svdssnp_rv_slot: &mut f64,
        var_uc_svgs_slot: &mut f64,
        var_uc_svgs_rv_slot: &mut f64,
        var_uc_vthsti_slot: &mut f64,
        var_uc_vthsti_rv_slot: &mut f64,
        var_uc_wfc_slot: &mut f64,
        var_uc_wfc_rv_slot: &mut f64,
        var_uc_wsti_slot: &mut f64,
        var_uc_wsti_dn0_slot: &mut f64,
        var_uc_wsti_dn10_slot: &mut f64,
        var_uc_wsti_dn13_slot: &mut f64,
        var_uc_wsti_dn2_slot: &mut f64,
        var_uc_wsti_dn4_slot: &mut f64,
        var_uc_wsti_dn5_slot: &mut f64,
        var_uc_wsti_dn6_slot: &mut f64,
        var_uc_wsti_dn7_slot: &mut f64,
        var_uc_wsti_dn8_slot: &mut f64,
        var_uc_wsti_dn9_slot: &mut f64,
        var_uc_wsti_rv_slot: &mut f64,
    ) {
        let mut var_uc_cgbo: f64 = *var_uc_cgbo_slot;
        let mut var_uc_cgbo_rv: f64 = *var_uc_cgbo_rv_slot;
        let mut var_uc_cgdo: f64 = *var_uc_cgdo_slot;
        let mut var_uc_cgdo_rv: f64 = *var_uc_cgdo_rv_slot;
        let mut var_uc_cgso: f64 = *var_uc_cgso_slot;
        let mut var_uc_cgso_rv: f64 = *var_uc_cgso_rv_slot;
        let mut var_uc_clm1: f64 = *var_uc_clm1_slot;
        let mut var_uc_clm1_rv: f64 = *var_uc_clm1_rv_slot;
        let mut var_uc_clm2: f64 = *var_uc_clm2_slot;
        let mut var_uc_clm2_dn0: f64 = *var_uc_clm2_dn0_slot;
        let mut var_uc_clm2_dn10: f64 = *var_uc_clm2_dn10_slot;
        let mut var_uc_clm2_dn13: f64 = *var_uc_clm2_dn13_slot;
        let mut var_uc_clm2_dn2: f64 = *var_uc_clm2_dn2_slot;
        let mut var_uc_clm2_dn4: f64 = *var_uc_clm2_dn4_slot;
        let mut var_uc_clm2_dn5: f64 = *var_uc_clm2_dn5_slot;
        let mut var_uc_clm2_dn6: f64 = *var_uc_clm2_dn6_slot;
        let mut var_uc_clm2_dn7: f64 = *var_uc_clm2_dn7_slot;
        let mut var_uc_clm2_dn8: f64 = *var_uc_clm2_dn8_slot;
        let mut var_uc_clm2_dn9: f64 = *var_uc_clm2_dn9_slot;
        let mut var_uc_clm2_rv: f64 = *var_uc_clm2_rv_slot;
        let mut var_uc_clm3: f64 = *var_uc_clm3_slot;
        let mut var_uc_clm3_rv: f64 = *var_uc_clm3_rv_slot;
        let mut var_uc_cvdsover: f64 = *var_uc_cvdsover_slot;
        let mut var_uc_cvdsover_rv: f64 = *var_uc_cvdsover_rv_slot;
        let mut var_uc_fn1: f64 = *var_uc_fn1_slot;
        let mut var_uc_fn1_rv: f64 = *var_uc_fn1_rv_slot;
        let mut var_uc_fn2: f64 = *var_uc_fn2_slot;
        let mut var_uc_fn2_rv: f64 = *var_uc_fn2_rv_slot;
        let mut var_uc_fn3: f64 = *var_uc_fn3_slot;
        let mut var_uc_fn3_rv: f64 = *var_uc_fn3_rv_slot;
        let mut var_uc_fvbs: f64 = *var_uc_fvbs_slot;
        let mut var_uc_fvbs_rv: f64 = *var_uc_fvbs_rv_slot;
        let mut var_uc_gidl1: f64 = *var_uc_gidl1_slot;
        let mut var_uc_gidl1_rv: f64 = *var_uc_gidl1_rv_slot;
        let mut var_uc_gidl2: f64 = *var_uc_gidl2_slot;
        let mut var_uc_gidl2_rv: f64 = *var_uc_gidl2_rv_slot;
        let mut var_uc_gleak1: f64 = *var_uc_gleak1_slot;
        let mut var_uc_gleak1_rv: f64 = *var_uc_gleak1_rv_slot;
        let mut var_uc_gleak2: f64 = *var_uc_gleak2_slot;
        let mut var_uc_gleak2_rv: f64 = *var_uc_gleak2_rv_slot;
        let mut var_uc_gleak3: f64 = *var_uc_gleak3_slot;
        let mut var_uc_gleak3_rv: f64 = *var_uc_gleak3_rv_slot;
        let mut var_uc_gleak6: f64 = *var_uc_gleak6_slot;
        let mut var_uc_gleak6_rv: f64 = *var_uc_gleak6_rv_slot;
        let mut var_uc_glkb1: f64 = *var_uc_glkb1_slot;
        let mut var_uc_glkb1_rv: f64 = *var_uc_glkb1_rv_slot;
        let mut var_uc_glkb2: f64 = *var_uc_glkb2_slot;
        let mut var_uc_glkb2_rv: f64 = *var_uc_glkb2_rv_slot;
        let mut var_uc_glksd1: f64 = *var_uc_glksd1_slot;
        let mut var_uc_glksd1_rv: f64 = *var_uc_glksd1_rv_slot;
        let mut var_uc_glksd2: f64 = *var_uc_glksd2_slot;
        let mut var_uc_glksd2_rv: f64 = *var_uc_glksd2_rv_slot;
        let mut var_uc_ibpc1: f64 = *var_uc_ibpc1_slot;
        let mut var_uc_ibpc1_rv: f64 = *var_uc_ibpc1_rv_slot;
        let mut var_uc_ibpc2: f64 = *var_uc_ibpc2_slot;
        let mut var_uc_ibpc2_rv: f64 = *var_uc_ibpc2_rv_slot;
        let mut var_uc_muesti1: f64 = *var_uc_muesti1_slot;
        let mut var_uc_muesti1_rv: f64 = *var_uc_muesti1_rv_slot;
        let mut var_uc_muesti2: f64 = *var_uc_muesti2_slot;
        let mut var_uc_muesti2_rv: f64 = *var_uc_muesti2_rv_slot;
        let mut var_uc_muesti3: f64 = *var_uc_muesti3_slot;
        let mut var_uc_muesti3_rv: f64 = *var_uc_muesti3_rv_slot;
        let mut var_uc_nfalp: f64 = *var_uc_nfalp_slot;
        let mut var_uc_nfalp_rv: f64 = *var_uc_nfalp_rv_slot;
        let mut var_uc_npext: f64 = *var_uc_npext_slot;
        let mut var_uc_npext_rv: f64 = *var_uc_npext_rv_slot;
        let mut var_uc_nsti: f64 = *var_uc_nsti_slot;
        let mut var_uc_nsti_rv: f64 = *var_uc_nsti_rv_slot;
        let mut var_uc_nsubpsti1: f64 = *var_uc_nsubpsti1_slot;
        let mut var_uc_nsubpsti1_rv: f64 = *var_uc_nsubpsti1_rv_slot;
        let mut var_uc_nsubpsti2: f64 = *var_uc_nsubpsti2_slot;
        let mut var_uc_nsubpsti2_rv: f64 = *var_uc_nsubpsti2_rv_slot;
        let mut var_uc_nsubpsti3: f64 = *var_uc_nsubpsti3_slot;
        let mut var_uc_nsubpsti3_rv: f64 = *var_uc_nsubpsti3_rv_slot;
        let mut var_uc_powrat: f64 = *var_uc_powrat_slot;
        let mut var_uc_powrat_rv: f64 = *var_uc_powrat_rv_slot;
        let mut var_uc_rd: f64 = *var_uc_rd_slot;
        let mut var_uc_rd22: f64 = *var_uc_rd22_slot;
        let mut var_uc_rd22_rv: f64 = *var_uc_rd22_rv_slot;
        let mut var_uc_rd23: f64 = *var_uc_rd23_slot;
        let mut var_uc_rd23_rv: f64 = *var_uc_rd23_rv_slot;
        let mut var_uc_rd24: f64 = *var_uc_rd24_slot;
        let mut var_uc_rd24_rv: f64 = *var_uc_rd24_rv_slot;
        let mut var_uc_rd_rv: f64 = *var_uc_rd_rv_slot;
        let mut var_uc_rdict1: f64 = *var_uc_rdict1_slot;
        let mut var_uc_rdict1_rv: f64 = *var_uc_rdict1_rv_slot;
        let mut var_uc_rdov13: f64 = *var_uc_rdov13_slot;
        let mut var_uc_rdov13_rv: f64 = *var_uc_rdov13_rv_slot;
        let mut var_uc_rdslp1: f64 = *var_uc_rdslp1_slot;
        let mut var_uc_rdslp1_rv: f64 = *var_uc_rdslp1_rv_slot;
        let mut var_uc_rdvb: f64 = *var_uc_rdvb_slot;
        let mut var_uc_rdvb_rv: f64 = *var_uc_rdvb_rv_slot;
        let mut var_uc_scsti1: f64 = *var_uc_scsti1_slot;
        let mut var_uc_scsti1_rv: f64 = *var_uc_scsti1_rv_slot;
        let mut var_uc_scsti2: f64 = *var_uc_scsti2_slot;
        let mut var_uc_scsti2_rv: f64 = *var_uc_scsti2_rv_slot;
        let mut var_uc_sub1: f64 = *var_uc_sub1_slot;
        let mut var_uc_sub1_rv: f64 = *var_uc_sub1_rv_slot;
        let mut var_uc_sub1snp: f64 = *var_uc_sub1snp_slot;
        let mut var_uc_sub1snp_rv: f64 = *var_uc_sub1snp_rv_slot;
        let mut var_uc_sub2: f64 = *var_uc_sub2_slot;
        let mut var_uc_sub2_rv: f64 = *var_uc_sub2_rv_slot;
        let mut var_uc_sub2snp: f64 = *var_uc_sub2snp_slot;
        let mut var_uc_sub2snp_rv: f64 = *var_uc_sub2snp_rv_slot;
        let mut var_uc_svbs: f64 = *var_uc_svbs_slot;
        let mut var_uc_svbs_rv: f64 = *var_uc_svbs_rv_slot;
        let mut var_uc_svds: f64 = *var_uc_svds_slot;
        let mut var_uc_svds_rv: f64 = *var_uc_svds_rv_slot;
        let mut var_uc_svdssnp: f64 = *var_uc_svdssnp_slot;
        let mut var_uc_svdssnp_rv: f64 = *var_uc_svdssnp_rv_slot;
        let mut var_uc_svgs: f64 = *var_uc_svgs_slot;
        let mut var_uc_svgs_rv: f64 = *var_uc_svgs_rv_slot;
        let mut var_uc_vthsti: f64 = *var_uc_vthsti_slot;
        let mut var_uc_vthsti_rv: f64 = *var_uc_vthsti_rv_slot;
        let mut var_uc_wfc: f64 = *var_uc_wfc_slot;
        let mut var_uc_wfc_rv: f64 = *var_uc_wfc_rv_slot;
        let mut var_uc_wsti: f64 = *var_uc_wsti_slot;
        let mut var_uc_wsti_dn0: f64 = *var_uc_wsti_dn0_slot;
        let mut var_uc_wsti_dn10: f64 = *var_uc_wsti_dn10_slot;
        let mut var_uc_wsti_dn13: f64 = *var_uc_wsti_dn13_slot;
        let mut var_uc_wsti_dn2: f64 = *var_uc_wsti_dn2_slot;
        let mut var_uc_wsti_dn4: f64 = *var_uc_wsti_dn4_slot;
        let mut var_uc_wsti_dn5: f64 = *var_uc_wsti_dn5_slot;
        let mut var_uc_wsti_dn6: f64 = *var_uc_wsti_dn6_slot;
        let mut var_uc_wsti_dn7: f64 = *var_uc_wsti_dn7_slot;
        let mut var_uc_wsti_dn8: f64 = *var_uc_wsti_dn8_slot;
        let mut var_uc_wsti_dn9: f64 = *var_uc_wsti_dn9_slot;
        let mut var_uc_wsti_rv: f64 = *var_uc_wsti_rv_slot;

        let assign8390_e3719: f64 = (p.p582 / var_lbin);
        let assign8390_e3720: f64 = (p.p179 + assign8390_e3719);
        let assign8390_e3723: f64 = (p.p670 / var_wbin);
        let assign8390_e3724: f64 = (assign8390_e3720 + assign8390_e3723);
        let assign8390_e3727: f64 = (p.p758 / var_lwbin);
        let assign8390_e3728: f64 = (assign8390_e3724 + assign8390_e3727);
        var_uc_sub1 = assign8390_e3728;
        var_uc_sub1_rv = 0.0;

        let assign8400_e3732: f64 = (p.p583 / var_lbin);
        let assign8400_e3733: f64 = (p.p180 + assign8400_e3732);
        let assign8400_e3736: f64 = (p.p671 / var_wbin);
        let assign8400_e3737: f64 = (assign8400_e3733 + assign8400_e3736);
        let assign8400_e3740: f64 = (p.p759 / var_lwbin);
        let assign8400_e3741: f64 = (assign8400_e3737 + assign8400_e3740);
        var_uc_sub2 = assign8400_e3741;
        var_uc_sub2_rv = 0.0;

        let assign8410_e3745: f64 = (p.p584 / var_lbin);
        let assign8410_e3746: f64 = (p.p185 + assign8410_e3745);
        let assign8410_e3749: f64 = (p.p672 / var_wbin);
        let assign8410_e3750: f64 = (assign8410_e3746 + assign8410_e3749);
        let assign8410_e3753: f64 = (p.p760 / var_lwbin);
        let assign8410_e3754: f64 = (assign8410_e3750 + assign8410_e3753);
        var_uc_svds = assign8410_e3754;
        var_uc_svds_rv = 0.0;

        let assign8420_e3758: f64 = (p.p585 / var_lbin);
        let assign8420_e3759: f64 = (p.p182 + assign8420_e3758);
        let assign8420_e3762: f64 = (p.p673 / var_wbin);
        let assign8420_e3763: f64 = (assign8420_e3759 + assign8420_e3762);
        let assign8420_e3766: f64 = (p.p761 / var_lwbin);
        let assign8420_e3767: f64 = (assign8420_e3763 + assign8420_e3766);
        var_uc_svbs = assign8420_e3767;
        var_uc_svbs_rv = 0.0;

        let assign8430_e3771: f64 = (p.p586 / var_lbin);
        let assign8430_e3772: f64 = (p.p181 + assign8430_e3771);
        let assign8430_e3775: f64 = (p.p674 / var_wbin);
        let assign8430_e3776: f64 = (assign8430_e3772 + assign8430_e3775);
        let assign8430_e3779: f64 = (p.p762 / var_lwbin);
        let assign8430_e3780: f64 = (assign8430_e3776 + assign8430_e3779);
        var_uc_svgs = assign8430_e3780;
        var_uc_svgs_rv = 0.0;

        let assign8440_e3784: f64 = (p.p587 / var_lbin);
        let assign8440_e3785: f64 = (p.p187 + assign8440_e3784);
        let assign8440_e3788: f64 = (p.p675 / var_wbin);
        let assign8440_e3789: f64 = (assign8440_e3785 + assign8440_e3788);
        let assign8440_e3792: f64 = (p.p763 / var_lwbin);
        let assign8440_e3793: f64 = (assign8440_e3789 + assign8440_e3792);
        var_uc_sub1snp = assign8440_e3793;
        var_uc_sub1snp_rv = 0.0;

        let assign8450_e3797: f64 = (p.p588 / var_lbin);
        let assign8450_e3798: f64 = (p.p188 + assign8450_e3797);
        let assign8450_e3801: f64 = (p.p676 / var_wbin);
        let assign8450_e3802: f64 = (assign8450_e3798 + assign8450_e3801);
        let assign8450_e3805: f64 = (p.p764 / var_lwbin);
        let assign8450_e3806: f64 = (assign8450_e3802 + assign8450_e3805);
        var_uc_sub2snp = assign8450_e3806;
        var_uc_sub2snp_rv = 0.0;

        let assign8460_e3810: f64 = (p.p589 / var_lbin);
        let assign8460_e3811: f64 = (p.p189 + assign8460_e3810);
        let assign8460_e3814: f64 = (p.p677 / var_wbin);
        let assign8460_e3815: f64 = (assign8460_e3811 + assign8460_e3814);
        let assign8460_e3818: f64 = (p.p765 / var_lwbin);
        let assign8460_e3819: f64 = (assign8460_e3815 + assign8460_e3818);
        var_uc_svdssnp = assign8460_e3819;
        var_uc_svdssnp_rv = 0.0;

        let assign8470_e3823: f64 = (p.p590 / var_lbin);
        let assign8470_e3824: f64 = (p.p194 + assign8470_e3823);
        let assign8470_e3827: f64 = (p.p678 / var_wbin);
        let assign8470_e3828: f64 = (assign8470_e3824 + assign8470_e3827);
        let assign8470_e3831: f64 = (p.p766 / var_lwbin);
        let assign8470_e3832: f64 = (assign8470_e3828 + assign8470_e3831);
        var_uc_fn1 = assign8470_e3832;
        var_uc_fn1_rv = 0.0;

        let assign8480_e3836: f64 = (p.p591 / var_lbin);
        let assign8480_e3837: f64 = (p.p195 + assign8480_e3836);
        let assign8480_e3840: f64 = (p.p679 / var_wbin);
        let assign8480_e3841: f64 = (assign8480_e3837 + assign8480_e3840);
        let assign8480_e3844: f64 = (p.p767 / var_lwbin);
        let assign8480_e3845: f64 = (assign8480_e3841 + assign8480_e3844);
        var_uc_fn2 = assign8480_e3845;
        var_uc_fn2_rv = 0.0;

        let assign8490_e3849: f64 = (p.p592 / var_lbin);
        let assign8490_e3850: f64 = (p.p196 + assign8490_e3849);
        let assign8490_e3853: f64 = (p.p680 / var_wbin);
        let assign8490_e3854: f64 = (assign8490_e3850 + assign8490_e3853);
        let assign8490_e3857: f64 = (p.p768 / var_lwbin);
        let assign8490_e3858: f64 = (assign8490_e3854 + assign8490_e3857);
        var_uc_fn3 = assign8490_e3858;
        var_uc_fn3_rv = 0.0;

        let assign8500_e3862: f64 = (p.p593 / var_lbin);
        let assign8500_e3863: f64 = (p.p197 + assign8500_e3862);
        let assign8500_e3866: f64 = (p.p681 / var_wbin);
        let assign8500_e3867: f64 = (assign8500_e3863 + assign8500_e3866);
        let assign8500_e3870: f64 = (p.p769 / var_lwbin);
        let assign8500_e3871: f64 = (assign8500_e3867 + assign8500_e3870);
        var_uc_fvbs = assign8500_e3871;
        var_uc_fvbs_rv = 0.0;

        let assign8510_e3875: f64 = (p.p594 / var_lbin);
        let assign8510_e3876: f64 = (p.p204 + assign8510_e3875);
        let assign8510_e3879: f64 = (p.p682 / var_wbin);
        let assign8510_e3880: f64 = (assign8510_e3876 + assign8510_e3879);
        let assign8510_e3883: f64 = (p.p770 / var_lwbin);
        let assign8510_e3884: f64 = (assign8510_e3880 + assign8510_e3883);
        var_uc_nsti = assign8510_e3884;
        var_uc_nsti_rv = 0.0;

        let assign8520_e3888: f64 = (p.p595 / var_lbin);
        let assign8520_e3889: f64 = (p.p205 + assign8520_e3888);
        let assign8520_e3892: f64 = (p.p683 / var_wbin);
        let assign8520_e3893: f64 = (assign8520_e3889 + assign8520_e3892);
        let assign8520_e3896: f64 = (p.p771 / var_lwbin);
        let assign8520_e3897: f64 = (assign8520_e3893 + assign8520_e3896);
        var_uc_wsti = assign8520_e3897;
        var_uc_wsti_dn0 = 0.0;
        var_uc_wsti_dn2 = 0.0;
        var_uc_wsti_dn4 = 0.0;
        var_uc_wsti_dn5 = 0.0;
        var_uc_wsti_dn6 = 0.0;
        var_uc_wsti_dn7 = 0.0;
        var_uc_wsti_dn8 = 0.0;
        var_uc_wsti_dn9 = 0.0;
        var_uc_wsti_dn10 = 0.0;
        var_uc_wsti_dn13 = 0.0;
        var_uc_wsti_rv = 0.0;

        let assign8530_e3901: f64 = (p.p596 / var_lbin);
        let assign8530_e3902: f64 = (p.p210 + assign8530_e3901);
        let assign8530_e3905: f64 = (p.p684 / var_wbin);
        let assign8530_e3906: f64 = (assign8530_e3902 + assign8530_e3905);
        let assign8530_e3909: f64 = (p.p772 / var_lwbin);
        let assign8530_e3910: f64 = (assign8530_e3906 + assign8530_e3909);
        var_uc_scsti1 = assign8530_e3910;
        var_uc_scsti1_rv = 0.0;

        let assign8540_e3914: f64 = (p.p597 / var_lbin);
        let assign8540_e3915: f64 = (p.p211 + assign8540_e3914);
        let assign8540_e3918: f64 = (p.p685 / var_wbin);
        let assign8540_e3919: f64 = (assign8540_e3915 + assign8540_e3918);
        let assign8540_e3922: f64 = (p.p773 / var_lwbin);
        let assign8540_e3923: f64 = (assign8540_e3919 + assign8540_e3922);
        var_uc_scsti2 = assign8540_e3923;
        var_uc_scsti2_rv = 0.0;

        let assign8550_e3927: f64 = (p.p598 / var_lbin);
        let assign8550_e3928: f64 = (p.p212 + assign8550_e3927);
        let assign8550_e3931: f64 = (p.p686 / var_wbin);
        let assign8550_e3932: f64 = (assign8550_e3928 + assign8550_e3931);
        let assign8550_e3935: f64 = (p.p774 / var_lwbin);
        let assign8550_e3936: f64 = (assign8550_e3932 + assign8550_e3935);
        var_uc_vthsti = assign8550_e3936;
        var_uc_vthsti_rv = 0.0;

        let assign8560_e3940: f64 = (p.p599 / var_lbin);
        let assign8560_e3941: f64 = (p.p214 + assign8560_e3940);
        let assign8560_e3944: f64 = (p.p687 / var_wbin);
        let assign8560_e3945: f64 = (assign8560_e3941 + assign8560_e3944);
        let assign8560_e3948: f64 = (p.p775 / var_lwbin);
        let assign8560_e3949: f64 = (assign8560_e3945 + assign8560_e3948);
        var_uc_muesti1 = assign8560_e3949;
        var_uc_muesti1_rv = 0.0;

        let assign8570_e3953: f64 = (p.p600 / var_lbin);
        let assign8570_e3954: f64 = (p.p215 + assign8570_e3953);
        let assign8570_e3957: f64 = (p.p688 / var_wbin);
        let assign8570_e3958: f64 = (assign8570_e3954 + assign8570_e3957);
        let assign8570_e3961: f64 = (p.p776 / var_lwbin);
        let assign8570_e3962: f64 = (assign8570_e3958 + assign8570_e3961);
        var_uc_muesti2 = assign8570_e3962;
        var_uc_muesti2_rv = 0.0;

        let assign8580_e3966: f64 = (p.p601 / var_lbin);
        let assign8580_e3967: f64 = (p.p216 + assign8580_e3966);
        let assign8580_e3970: f64 = (p.p689 / var_wbin);
        let assign8580_e3971: f64 = (assign8580_e3967 + assign8580_e3970);
        let assign8580_e3974: f64 = (p.p777 / var_lwbin);
        let assign8580_e3975: f64 = (assign8580_e3971 + assign8580_e3974);
        var_uc_muesti3 = assign8580_e3975;
        var_uc_muesti3_rv = 0.0;

        let assign8590_e3979: f64 = (p.p602 / var_lbin);
        let assign8590_e3980: f64 = (p.p217 + assign8590_e3979);
        let assign8590_e3983: f64 = (p.p690 / var_wbin);
        let assign8590_e3984: f64 = (assign8590_e3980 + assign8590_e3983);
        let assign8590_e3987: f64 = (p.p778 / var_lwbin);
        let assign8590_e3988: f64 = (assign8590_e3984 + assign8590_e3987);
        var_uc_nsubpsti1 = assign8590_e3988;
        var_uc_nsubpsti1_rv = 0.0;

        let assign8600_e3992: f64 = (p.p603 / var_lbin);
        let assign8600_e3993: f64 = (p.p218 + assign8600_e3992);
        let assign8600_e3996: f64 = (p.p691 / var_wbin);
        let assign8600_e3997: f64 = (assign8600_e3993 + assign8600_e3996);
        let assign8600_e4000: f64 = (p.p779 / var_lwbin);
        let assign8600_e4001: f64 = (assign8600_e3997 + assign8600_e4000);
        var_uc_nsubpsti2 = assign8600_e4001;
        var_uc_nsubpsti2_rv = 0.0;

        let assign8610_e4005: f64 = (p.p604 / var_lbin);
        let assign8610_e4006: f64 = (p.p219 + assign8610_e4005);
        let assign8610_e4009: f64 = (p.p692 / var_wbin);
        let assign8610_e4010: f64 = (assign8610_e4006 + assign8610_e4009);
        let assign8610_e4013: f64 = (p.p780 / var_lwbin);
        let assign8610_e4014: f64 = (assign8610_e4010 + assign8610_e4013);
        var_uc_nsubpsti3 = assign8610_e4014;
        var_uc_nsubpsti3_rv = 0.0;

        let assign8620_e4018: f64 = (p.p605 / var_lbin);
        let assign8620_e4019: f64 = (p.p269 + assign8620_e4018);
        let assign8620_e4022: f64 = (p.p693 / var_wbin);
        let assign8620_e4023: f64 = (assign8620_e4019 + assign8620_e4022);
        let assign8620_e4026: f64 = (p.p781 / var_lwbin);
        let assign8620_e4027: f64 = (assign8620_e4023 + assign8620_e4026);
        var_uc_cgso = assign8620_e4027;
        var_uc_cgso_rv = 0.0;

        let assign8630_e4031: f64 = (p.p606 / var_lbin);
        let assign8630_e4032: f64 = (p.p268 + assign8630_e4031);
        let assign8630_e4035: f64 = (p.p694 / var_wbin);
        let assign8630_e4036: f64 = (assign8630_e4032 + assign8630_e4035);
        let assign8630_e4039: f64 = (p.p782 / var_lwbin);
        let assign8630_e4040: f64 = (assign8630_e4036 + assign8630_e4039);
        var_uc_cgdo = assign8630_e4040;
        var_uc_cgdo_rv = 0.0;

        let assign8640_e4044: f64 = (p.p607 / var_lbin);
        let assign8640_e4045: f64 = (p.p226 + assign8640_e4044);
        let assign8640_e4048: f64 = (p.p695 / var_wbin);
        let assign8640_e4049: f64 = (assign8640_e4045 + assign8640_e4048);
        let assign8640_e4052: f64 = (p.p783 / var_lwbin);
        let assign8640_e4053: f64 = (assign8640_e4049 + assign8640_e4052);
        var_uc_clm1 = assign8640_e4053;
        var_uc_clm1_rv = 0.0;

        let assign8650_e4057: f64 = (p.p608 / var_lbin);
        let assign8650_e4058: f64 = (p.p227 + assign8650_e4057);
        let assign8650_e4061: f64 = (p.p696 / var_wbin);
        let assign8650_e4062: f64 = (assign8650_e4058 + assign8650_e4061);
        let assign8650_e4065: f64 = (p.p784 / var_lwbin);
        let assign8650_e4066: f64 = (assign8650_e4062 + assign8650_e4065);
        var_uc_clm2 = assign8650_e4066;
        var_uc_clm2_dn0 = 0.0;
        var_uc_clm2_dn2 = 0.0;
        var_uc_clm2_dn4 = 0.0;
        var_uc_clm2_dn5 = 0.0;
        var_uc_clm2_dn6 = 0.0;
        var_uc_clm2_dn7 = 0.0;
        var_uc_clm2_dn8 = 0.0;
        var_uc_clm2_dn9 = 0.0;
        var_uc_clm2_dn10 = 0.0;
        var_uc_clm2_dn13 = 0.0;
        var_uc_clm2_rv = 0.0;

        let assign8660_e4070: f64 = (p.p609 / var_lbin);
        let assign8660_e4071: f64 = (p.p228 + assign8660_e4070);
        let assign8660_e4074: f64 = (p.p697 / var_wbin);
        let assign8660_e4075: f64 = (assign8660_e4071 + assign8660_e4074);
        let assign8660_e4078: f64 = (p.p785 / var_lwbin);
        let assign8660_e4079: f64 = (assign8660_e4075 + assign8660_e4078);
        var_uc_clm3 = assign8660_e4079;
        var_uc_clm3_rv = 0.0;

        let assign8670_e4083: f64 = (p.p610 / var_lbin);
        let assign8670_e4084: f64 = (p.p232 + assign8670_e4083);
        let assign8670_e4087: f64 = (p.p698 / var_wbin);
        let assign8670_e4088: f64 = (assign8670_e4084 + assign8670_e4087);
        let assign8670_e4091: f64 = (p.p786 / var_lwbin);
        let assign8670_e4092: f64 = (assign8670_e4088 + assign8670_e4091);
        var_uc_wfc = assign8670_e4092;
        var_uc_wfc_rv = 0.0;

        let assign8680_e4096: f64 = (p.p611 / var_lbin);
        let assign8680_e4097: f64 = (p.p240 + assign8680_e4096);
        let assign8680_e4100: f64 = (p.p699 / var_wbin);
        let assign8680_e4101: f64 = (assign8680_e4097 + assign8680_e4100);
        let assign8680_e4104: f64 = (p.p787 / var_lwbin);
        let assign8680_e4105: f64 = (assign8680_e4101 + assign8680_e4104);
        var_uc_gidl1 = assign8680_e4105;
        var_uc_gidl1_rv = 0.0;

        let assign8690_e4109: f64 = (p.p612 / var_lbin);
        let assign8690_e4110: f64 = (p.p241 + assign8690_e4109);
        let assign8690_e4113: f64 = (p.p700 / var_wbin);
        let assign8690_e4114: f64 = (assign8690_e4110 + assign8690_e4113);
        let assign8690_e4117: f64 = (p.p788 / var_lwbin);
        let assign8690_e4118: f64 = (assign8690_e4114 + assign8690_e4117);
        var_uc_gidl2 = assign8690_e4118;
        var_uc_gidl2_rv = 0.0;

        let assign8700_e4122: f64 = (p.p613 / var_lbin);
        let assign8700_e4123: f64 = (p.p245 + assign8700_e4122);
        let assign8700_e4126: f64 = (p.p701 / var_wbin);
        let assign8700_e4127: f64 = (assign8700_e4123 + assign8700_e4126);
        let assign8700_e4130: f64 = (p.p789 / var_lwbin);
        let assign8700_e4131: f64 = (assign8700_e4127 + assign8700_e4130);
        var_uc_gleak1 = assign8700_e4131;
        var_uc_gleak1_rv = 0.0;

        let assign8710_e4135: f64 = (p.p614 / var_lbin);
        let assign8710_e4136: f64 = (p.p246 + assign8710_e4135);
        let assign8710_e4139: f64 = (p.p702 / var_wbin);
        let assign8710_e4140: f64 = (assign8710_e4136 + assign8710_e4139);
        let assign8710_e4143: f64 = (p.p790 / var_lwbin);
        let assign8710_e4144: f64 = (assign8710_e4140 + assign8710_e4143);
        var_uc_gleak2 = assign8710_e4144;
        var_uc_gleak2_rv = 0.0;

        let assign8720_e4148: f64 = (p.p615 / var_lbin);
        let assign8720_e4149: f64 = (p.p247 + assign8720_e4148);
        let assign8720_e4152: f64 = (p.p703 / var_wbin);
        let assign8720_e4153: f64 = (assign8720_e4149 + assign8720_e4152);
        let assign8720_e4156: f64 = (p.p791 / var_lwbin);
        let assign8720_e4157: f64 = (assign8720_e4153 + assign8720_e4156);
        var_uc_gleak3 = assign8720_e4157;
        var_uc_gleak3_rv = 0.0;

        let assign8730_e4161: f64 = (p.p616 / var_lbin);
        let assign8730_e4162: f64 = (p.p250 + assign8730_e4161);
        let assign8730_e4165: f64 = (p.p704 / var_wbin);
        let assign8730_e4166: f64 = (assign8730_e4162 + assign8730_e4165);
        let assign8730_e4169: f64 = (p.p792 / var_lwbin);
        let assign8730_e4170: f64 = (assign8730_e4166 + assign8730_e4169);
        var_uc_gleak6 = assign8730_e4170;
        var_uc_gleak6_rv = 0.0;

        let assign8740_e4174: f64 = (p.p617 / var_lbin);
        let assign8740_e4175: f64 = (p.p253 + assign8740_e4174);
        let assign8740_e4178: f64 = (p.p705 / var_wbin);
        let assign8740_e4179: f64 = (assign8740_e4175 + assign8740_e4178);
        let assign8740_e4182: f64 = (p.p793 / var_lwbin);
        let assign8740_e4183: f64 = (assign8740_e4179 + assign8740_e4182);
        var_uc_glksd1 = assign8740_e4183;
        var_uc_glksd1_rv = 0.0;

        let assign8750_e4187: f64 = (p.p618 / var_lbin);
        let assign8750_e4188: f64 = (p.p254 + assign8750_e4187);
        let assign8750_e4191: f64 = (p.p706 / var_wbin);
        let assign8750_e4192: f64 = (assign8750_e4188 + assign8750_e4191);
        let assign8750_e4195: f64 = (p.p794 / var_lwbin);
        let assign8750_e4196: f64 = (assign8750_e4192 + assign8750_e4195);
        var_uc_glksd2 = assign8750_e4196;
        var_uc_glksd2_rv = 0.0;

        let assign8760_e4200: f64 = (p.p619 / var_lbin);
        let assign8760_e4201: f64 = (p.p256 + assign8760_e4200);
        let assign8760_e4204: f64 = (p.p707 / var_wbin);
        let assign8760_e4205: f64 = (assign8760_e4201 + assign8760_e4204);
        let assign8760_e4208: f64 = (p.p795 / var_lwbin);
        let assign8760_e4209: f64 = (assign8760_e4205 + assign8760_e4208);
        var_uc_glkb1 = assign8760_e4209;
        var_uc_glkb1_rv = 0.0;

        let assign8770_e4213: f64 = (p.p620 / var_lbin);
        let assign8770_e4214: f64 = (p.p257 + assign8770_e4213);
        let assign8770_e4217: f64 = (p.p708 / var_wbin);
        let assign8770_e4218: f64 = (assign8770_e4214 + assign8770_e4217);
        let assign8770_e4221: f64 = (p.p796 / var_lwbin);
        let assign8770_e4222: f64 = (assign8770_e4218 + assign8770_e4221);
        var_uc_glkb2 = assign8770_e4222;
        var_uc_glkb2_rv = 0.0;

        let assign8790_e4239: f64 = (p.p622 / var_lbin);
        let assign8790_e4240: f64 = (p.p265 + assign8790_e4239);
        let assign8790_e4243: f64 = (p.p710 / var_wbin);
        let assign8790_e4244: f64 = (assign8790_e4240 + assign8790_e4243);
        let assign8790_e4247: f64 = (p.p798 / var_lwbin);
        let assign8790_e4248: f64 = (assign8790_e4244 + assign8790_e4247);
        var_uc_nfalp = assign8790_e4248;
        var_uc_nfalp_rv = 0.0;

        let assign8800_e4252: f64 = (p.p623 / var_lbin);
        let assign8800_e4253: f64 = (p.p278 + assign8800_e4252);
        let assign8800_e4256: f64 = (p.p711 / var_wbin);
        let assign8800_e4257: f64 = (assign8800_e4253 + assign8800_e4256);
        let assign8800_e4260: f64 = (p.p799 / var_lwbin);
        let assign8800_e4261: f64 = (assign8800_e4257 + assign8800_e4260);
        var_uc_ibpc1 = assign8800_e4261;
        var_uc_ibpc1_rv = 0.0;

        let assign8810_e4265: f64 = (p.p624 / var_lbin);
        let assign8810_e4266: f64 = (p.p281 + assign8810_e4265);
        let assign8810_e4269: f64 = (p.p712 / var_wbin);
        let assign8810_e4270: f64 = (assign8810_e4266 + assign8810_e4269);
        let assign8810_e4273: f64 = (p.p800 / var_lwbin);
        let assign8810_e4274: f64 = (assign8810_e4270 + assign8810_e4273);
        var_uc_ibpc2 = assign8810_e4274;
        var_uc_ibpc2_rv = 0.0;

        let assign8820_e4278: f64 = (p.p625 / var_lbin);
        let assign8820_e4279: f64 = (p.p79 + assign8820_e4278);
        let assign8820_e4282: f64 = (p.p713 / var_wbin);
        let assign8820_e4283: f64 = (assign8820_e4279 + assign8820_e4282);
        let assign8820_e4286: f64 = (p.p801 / var_lwbin);
        let assign8820_e4287: f64 = (assign8820_e4283 + assign8820_e4286);
        var_uc_cgbo = assign8820_e4287;
        var_uc_cgbo_rv = 0.0;

        let assign8830_e4291: f64 = (p.p626 / var_lbin);
        let assign8830_e4292: f64 = (p.p86 + assign8830_e4291);
        let assign8830_e4295: f64 = (p.p714 / var_wbin);
        let assign8830_e4296: f64 = (assign8830_e4292 + assign8830_e4295);
        let assign8830_e4299: f64 = (p.p802 / var_lwbin);
        let assign8830_e4300: f64 = (assign8830_e4296 + assign8830_e4299);
        var_uc_cvdsover = assign8830_e4300;
        var_uc_cvdsover_rv = 0.0;

        let assign8850_e4317: f64 = (p.p628 / var_lbin);
        let assign8850_e4318: f64 = (p.p76 + assign8850_e4317);
        let assign8850_e4321: f64 = (p.p716 / var_wbin);
        let assign8850_e4322: f64 = (assign8850_e4318 + assign8850_e4321);
        let assign8850_e4325: f64 = (p.p804 / var_lwbin);
        let assign8850_e4326: f64 = (assign8850_e4322 + assign8850_e4325);
        var_uc_npext = assign8850_e4326;
        var_uc_npext_rv = 0.0;

        let assign8860_e4330: f64 = (p.p629 / var_lbin);
        let assign8860_e4331: f64 = (p.p81 + assign8860_e4330);
        let assign8860_e4334: f64 = (p.p717 / var_wbin);
        let assign8860_e4335: f64 = (assign8860_e4331 + assign8860_e4334);
        let assign8860_e4338: f64 = (p.p805 / var_lwbin);
        let assign8860_e4339: f64 = (assign8860_e4335 + assign8860_e4338);
        var_uc_powrat = assign8860_e4339;
        var_uc_powrat_rv = 0.0;

        let assign8870_e4343: f64 = (p.p630 / var_lbin);
        let assign8870_e4344: f64 = (p.p74 + assign8870_e4343);
        let assign8870_e4347: f64 = (p.p718 / var_wbin);
        let assign8870_e4348: f64 = (assign8870_e4344 + assign8870_e4347);
        let assign8870_e4351: f64 = (p.p806 / var_lwbin);
        let assign8870_e4352: f64 = (assign8870_e4348 + assign8870_e4351);
        var_uc_rd = assign8870_e4352;
        var_uc_rd_rv = 0.0;

        let assign8880_e4356: f64 = (p.p631 / var_lbin);
        let assign8880_e4357: f64 = (p.p298 + assign8880_e4356);
        let assign8880_e4360: f64 = (p.p719 / var_wbin);
        let assign8880_e4361: f64 = (assign8880_e4357 + assign8880_e4360);
        let assign8880_e4364: f64 = (p.p807 / var_lwbin);
        let assign8880_e4365: f64 = (assign8880_e4361 + assign8880_e4364);
        var_uc_rd22 = assign8880_e4365;
        var_uc_rd22_rv = 0.0;

        let assign8890_e4369: f64 = (p.p632 / var_lbin);
        let assign8890_e4370: f64 = (p.p83 + assign8890_e4369);
        let assign8890_e4373: f64 = (p.p720 / var_wbin);
        let assign8890_e4374: f64 = (assign8890_e4370 + assign8890_e4373);
        let assign8890_e4377: f64 = (p.p808 / var_lwbin);
        let assign8890_e4378: f64 = (assign8890_e4374 + assign8890_e4377);
        var_uc_rd23 = assign8890_e4378;
        var_uc_rd23_rv = 0.0;

        let assign8900_e4382: f64 = (p.p633 / var_lbin);
        let assign8900_e4383: f64 = (p.p84 + assign8900_e4382);
        let assign8900_e4386: f64 = (p.p721 / var_wbin);
        let assign8900_e4387: f64 = (assign8900_e4383 + assign8900_e4386);
        let assign8900_e4390: f64 = (p.p809 / var_lwbin);
        let assign8900_e4391: f64 = (assign8900_e4387 + assign8900_e4390);
        var_uc_rd24 = assign8900_e4391;
        var_uc_rd24_rv = 0.0;

        let assign8910_e4395: f64 = (p.p634 / var_lbin);
        let assign8910_e4396: f64 = (p.p62 + assign8910_e4395);
        let assign8910_e4399: f64 = (p.p722 / var_wbin);
        let assign8910_e4400: f64 = (assign8910_e4396 + assign8910_e4399);
        let assign8910_e4403: f64 = (p.p810 / var_lwbin);
        let assign8910_e4404: f64 = (assign8910_e4400 + assign8910_e4403);
        var_uc_rdict1 = assign8910_e4404;
        var_uc_rdict1_rv = 0.0;

        let assign8920_e4408: f64 = (p.p635 / var_lbin);
        let assign8920_e4409: f64 = (p.p59 + assign8920_e4408);
        let assign8920_e4412: f64 = (p.p723 / var_wbin);
        let assign8920_e4413: f64 = (assign8920_e4409 + assign8920_e4412);
        let assign8920_e4416: f64 = (p.p811 / var_lwbin);
        let assign8920_e4417: f64 = (assign8920_e4413 + assign8920_e4416);
        var_uc_rdov13 = assign8920_e4417;
        var_uc_rdov13_rv = 0.0;

        let assign8930_e4421: f64 = (p.p636 / var_lbin);
        let assign8930_e4422: f64 = (p.p60 + assign8930_e4421);
        let assign8930_e4425: f64 = (p.p724 / var_wbin);
        let assign8930_e4426: f64 = (assign8930_e4422 + assign8930_e4425);
        let assign8930_e4429: f64 = (p.p812 / var_lwbin);
        let assign8930_e4430: f64 = (assign8930_e4426 + assign8930_e4429);
        var_uc_rdslp1 = assign8930_e4430;
        var_uc_rdslp1_rv = 0.0;

        let assign8940_e4434: f64 = (p.p637 / var_lbin);
        let assign8940_e4435: f64 = (p.p85 + assign8940_e4434);
        let assign8940_e4438: f64 = (p.p725 / var_wbin);
        let assign8940_e4439: f64 = (assign8940_e4435 + assign8940_e4438);
        let assign8940_e4442: f64 = (p.p813 / var_lwbin);
        let assign8940_e4443: f64 = (assign8940_e4439 + assign8940_e4442);
        var_uc_rdvb = assign8940_e4443;
        var_uc_rdvb_rv = 0.0;

        *var_uc_cgbo_slot = var_uc_cgbo;
        *var_uc_cgbo_rv_slot = var_uc_cgbo_rv;
        *var_uc_cgdo_slot = var_uc_cgdo;
        *var_uc_cgdo_rv_slot = var_uc_cgdo_rv;
        *var_uc_cgso_slot = var_uc_cgso;
        *var_uc_cgso_rv_slot = var_uc_cgso_rv;
        *var_uc_clm1_slot = var_uc_clm1;
        *var_uc_clm1_rv_slot = var_uc_clm1_rv;
        *var_uc_clm2_slot = var_uc_clm2;
        *var_uc_clm2_dn0_slot = var_uc_clm2_dn0;
        *var_uc_clm2_dn10_slot = var_uc_clm2_dn10;
        *var_uc_clm2_dn13_slot = var_uc_clm2_dn13;
        *var_uc_clm2_dn2_slot = var_uc_clm2_dn2;
        *var_uc_clm2_dn4_slot = var_uc_clm2_dn4;
        *var_uc_clm2_dn5_slot = var_uc_clm2_dn5;
        *var_uc_clm2_dn6_slot = var_uc_clm2_dn6;
        *var_uc_clm2_dn7_slot = var_uc_clm2_dn7;
        *var_uc_clm2_dn8_slot = var_uc_clm2_dn8;
        *var_uc_clm2_dn9_slot = var_uc_clm2_dn9;
        *var_uc_clm2_rv_slot = var_uc_clm2_rv;
        *var_uc_clm3_slot = var_uc_clm3;
        *var_uc_clm3_rv_slot = var_uc_clm3_rv;
        *var_uc_cvdsover_slot = var_uc_cvdsover;
        *var_uc_cvdsover_rv_slot = var_uc_cvdsover_rv;
        *var_uc_fn1_slot = var_uc_fn1;
        *var_uc_fn1_rv_slot = var_uc_fn1_rv;
        *var_uc_fn2_slot = var_uc_fn2;
        *var_uc_fn2_rv_slot = var_uc_fn2_rv;
        *var_uc_fn3_slot = var_uc_fn3;
        *var_uc_fn3_rv_slot = var_uc_fn3_rv;
        *var_uc_fvbs_slot = var_uc_fvbs;
        *var_uc_fvbs_rv_slot = var_uc_fvbs_rv;
        *var_uc_gidl1_slot = var_uc_gidl1;
        *var_uc_gidl1_rv_slot = var_uc_gidl1_rv;
        *var_uc_gidl2_slot = var_uc_gidl2;
        *var_uc_gidl2_rv_slot = var_uc_gidl2_rv;
        *var_uc_gleak1_slot = var_uc_gleak1;
        *var_uc_gleak1_rv_slot = var_uc_gleak1_rv;
        *var_uc_gleak2_slot = var_uc_gleak2;
        *var_uc_gleak2_rv_slot = var_uc_gleak2_rv;
        *var_uc_gleak3_slot = var_uc_gleak3;
        *var_uc_gleak3_rv_slot = var_uc_gleak3_rv;
        *var_uc_gleak6_slot = var_uc_gleak6;
        *var_uc_gleak6_rv_slot = var_uc_gleak6_rv;
        *var_uc_glkb1_slot = var_uc_glkb1;
        *var_uc_glkb1_rv_slot = var_uc_glkb1_rv;
        *var_uc_glkb2_slot = var_uc_glkb2;
        *var_uc_glkb2_rv_slot = var_uc_glkb2_rv;
        *var_uc_glksd1_slot = var_uc_glksd1;
        *var_uc_glksd1_rv_slot = var_uc_glksd1_rv;
        *var_uc_glksd2_slot = var_uc_glksd2;
        *var_uc_glksd2_rv_slot = var_uc_glksd2_rv;
        *var_uc_ibpc1_slot = var_uc_ibpc1;
        *var_uc_ibpc1_rv_slot = var_uc_ibpc1_rv;
        *var_uc_ibpc2_slot = var_uc_ibpc2;
        *var_uc_ibpc2_rv_slot = var_uc_ibpc2_rv;
        *var_uc_muesti1_slot = var_uc_muesti1;
        *var_uc_muesti1_rv_slot = var_uc_muesti1_rv;
        *var_uc_muesti2_slot = var_uc_muesti2;
        *var_uc_muesti2_rv_slot = var_uc_muesti2_rv;
        *var_uc_muesti3_slot = var_uc_muesti3;
        *var_uc_muesti3_rv_slot = var_uc_muesti3_rv;
        *var_uc_nfalp_slot = var_uc_nfalp;
        *var_uc_nfalp_rv_slot = var_uc_nfalp_rv;
        *var_uc_npext_slot = var_uc_npext;
        *var_uc_npext_rv_slot = var_uc_npext_rv;
        *var_uc_nsti_slot = var_uc_nsti;
        *var_uc_nsti_rv_slot = var_uc_nsti_rv;
        *var_uc_nsubpsti1_slot = var_uc_nsubpsti1;
        *var_uc_nsubpsti1_rv_slot = var_uc_nsubpsti1_rv;
        *var_uc_nsubpsti2_slot = var_uc_nsubpsti2;
        *var_uc_nsubpsti2_rv_slot = var_uc_nsubpsti2_rv;
        *var_uc_nsubpsti3_slot = var_uc_nsubpsti3;
        *var_uc_nsubpsti3_rv_slot = var_uc_nsubpsti3_rv;
        *var_uc_powrat_slot = var_uc_powrat;
        *var_uc_powrat_rv_slot = var_uc_powrat_rv;
        *var_uc_rd_slot = var_uc_rd;
        *var_uc_rd22_slot = var_uc_rd22;
        *var_uc_rd22_rv_slot = var_uc_rd22_rv;
        *var_uc_rd23_slot = var_uc_rd23;
        *var_uc_rd23_rv_slot = var_uc_rd23_rv;
        *var_uc_rd24_slot = var_uc_rd24;
        *var_uc_rd24_rv_slot = var_uc_rd24_rv;
        *var_uc_rd_rv_slot = var_uc_rd_rv;
        *var_uc_rdict1_slot = var_uc_rdict1;
        *var_uc_rdict1_rv_slot = var_uc_rdict1_rv;
        *var_uc_rdov13_slot = var_uc_rdov13;
        *var_uc_rdov13_rv_slot = var_uc_rdov13_rv;
        *var_uc_rdslp1_slot = var_uc_rdslp1;
        *var_uc_rdslp1_rv_slot = var_uc_rdslp1_rv;
        *var_uc_rdvb_slot = var_uc_rdvb;
        *var_uc_rdvb_rv_slot = var_uc_rdvb_rv;
        *var_uc_scsti1_slot = var_uc_scsti1;
        *var_uc_scsti1_rv_slot = var_uc_scsti1_rv;
        *var_uc_scsti2_slot = var_uc_scsti2;
        *var_uc_scsti2_rv_slot = var_uc_scsti2_rv;
        *var_uc_sub1_slot = var_uc_sub1;
        *var_uc_sub1_rv_slot = var_uc_sub1_rv;
        *var_uc_sub1snp_slot = var_uc_sub1snp;
        *var_uc_sub1snp_rv_slot = var_uc_sub1snp_rv;
        *var_uc_sub2_slot = var_uc_sub2;
        *var_uc_sub2_rv_slot = var_uc_sub2_rv;
        *var_uc_sub2snp_slot = var_uc_sub2snp;
        *var_uc_sub2snp_rv_slot = var_uc_sub2snp_rv;
        *var_uc_svbs_slot = var_uc_svbs;
        *var_uc_svbs_rv_slot = var_uc_svbs_rv;
        *var_uc_svds_slot = var_uc_svds;
        *var_uc_svds_rv_slot = var_uc_svds_rv;
        *var_uc_svdssnp_slot = var_uc_svdssnp;
        *var_uc_svdssnp_rv_slot = var_uc_svdssnp_rv;
        *var_uc_svgs_slot = var_uc_svgs;
        *var_uc_svgs_rv_slot = var_uc_svgs_rv;
        *var_uc_vthsti_slot = var_uc_vthsti;
        *var_uc_vthsti_rv_slot = var_uc_vthsti_rv;
        *var_uc_wfc_slot = var_uc_wfc;
        *var_uc_wfc_rv_slot = var_uc_wfc_rv;
        *var_uc_wsti_slot = var_uc_wsti;
        *var_uc_wsti_dn0_slot = var_uc_wsti_dn0;
        *var_uc_wsti_dn10_slot = var_uc_wsti_dn10;
        *var_uc_wsti_dn13_slot = var_uc_wsti_dn13;
        *var_uc_wsti_dn2_slot = var_uc_wsti_dn2;
        *var_uc_wsti_dn4_slot = var_uc_wsti_dn4;
        *var_uc_wsti_dn5_slot = var_uc_wsti_dn5;
        *var_uc_wsti_dn6_slot = var_uc_wsti_dn6;
        *var_uc_wsti_dn7_slot = var_uc_wsti_dn7;
        *var_uc_wsti_dn8_slot = var_uc_wsti_dn8;
        *var_uc_wsti_dn9_slot = var_uc_wsti_dn9;
        *var_uc_wsti_rv_slot = var_uc_wsti_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        var_lbin: f64,
        var_lg: f64,
        var_lwbin: f64,
        var_uc_codep: f64,
        var_wbin: f64,
        var_guard185_slot: &mut f64,
        var_guard185_rv_slot: &mut f64,
        var_guard186_slot: &mut f64,
        var_guard186_rv_slot: &mut f64,
        var_guard187_slot: &mut f64,
        var_guard187_rv_slot: &mut f64,
        var_guard188_slot: &mut f64,
        var_guard188_rv_slot: &mut f64,
        var_guard189_slot: &mut f64,
        var_guard189_rv_slot: &mut f64,
        var_guard190_slot: &mut f64,
        var_guard190_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_uc_depleak_slot: &mut f64,
        var_uc_depleak_dn0_slot: &mut f64,
        var_uc_depleak_dn10_slot: &mut f64,
        var_uc_depleak_dn13_slot: &mut f64,
        var_uc_depleak_dn2_slot: &mut f64,
        var_uc_depleak_dn4_slot: &mut f64,
        var_uc_depleak_dn5_slot: &mut f64,
        var_uc_depleak_dn6_slot: &mut f64,
        var_uc_depleak_dn7_slot: &mut f64,
        var_uc_depleak_dn8_slot: &mut f64,
        var_uc_depleak_dn9_slot: &mut f64,
        var_uc_depleak_rv_slot: &mut f64,
        var_uc_depmue0_slot: &mut f64,
        var_uc_depmue0_dn0_slot: &mut f64,
        var_uc_depmue0_dn10_slot: &mut f64,
        var_uc_depmue0_dn13_slot: &mut f64,
        var_uc_depmue0_dn2_slot: &mut f64,
        var_uc_depmue0_dn4_slot: &mut f64,
        var_uc_depmue0_dn5_slot: &mut f64,
        var_uc_depmue0_dn6_slot: &mut f64,
        var_uc_depmue0_dn7_slot: &mut f64,
        var_uc_depmue0_dn8_slot: &mut f64,
        var_uc_depmue0_dn9_slot: &mut f64,
        var_uc_depmue0_rv_slot: &mut f64,
        var_uc_depmue1_slot: &mut f64,
        var_uc_depmue1_dn0_slot: &mut f64,
        var_uc_depmue1_dn10_slot: &mut f64,
        var_uc_depmue1_dn13_slot: &mut f64,
        var_uc_depmue1_dn2_slot: &mut f64,
        var_uc_depmue1_dn4_slot: &mut f64,
        var_uc_depmue1_dn5_slot: &mut f64,
        var_uc_depmue1_dn6_slot: &mut f64,
        var_uc_depmue1_dn7_slot: &mut f64,
        var_uc_depmue1_dn8_slot: &mut f64,
        var_uc_depmue1_dn9_slot: &mut f64,
        var_uc_depmue1_rv_slot: &mut f64,
        var_uc_depmueback0_slot: &mut f64,
        var_uc_depmueback0_dn0_slot: &mut f64,
        var_uc_depmueback0_dn10_slot: &mut f64,
        var_uc_depmueback0_dn13_slot: &mut f64,
        var_uc_depmueback0_dn2_slot: &mut f64,
        var_uc_depmueback0_dn4_slot: &mut f64,
        var_uc_depmueback0_dn5_slot: &mut f64,
        var_uc_depmueback0_dn6_slot: &mut f64,
        var_uc_depmueback0_dn7_slot: &mut f64,
        var_uc_depmueback0_dn8_slot: &mut f64,
        var_uc_depmueback0_dn9_slot: &mut f64,
        var_uc_depmueback0_rv_slot: &mut f64,
        var_uc_depvmax_slot: &mut f64,
        var_uc_depvmax_dn0_slot: &mut f64,
        var_uc_depvmax_dn10_slot: &mut f64,
        var_uc_depvmax_dn13_slot: &mut f64,
        var_uc_depvmax_dn2_slot: &mut f64,
        var_uc_depvmax_dn4_slot: &mut f64,
        var_uc_depvmax_dn5_slot: &mut f64,
        var_uc_depvmax_dn6_slot: &mut f64,
        var_uc_depvmax_dn7_slot: &mut f64,
        var_uc_depvmax_dn8_slot: &mut f64,
        var_uc_depvmax_dn9_slot: &mut f64,
        var_uc_depvmax_rv_slot: &mut f64,
        var_uc_js0d_slot: &mut f64,
        var_uc_js0d_rv_slot: &mut f64,
        var_uc_js0s_slot: &mut f64,
        var_uc_js0s_rv_slot: &mut f64,
        var_uc_js0swd_slot: &mut f64,
        var_uc_js0swd_rv_slot: &mut f64,
        var_uc_js0sws_slot: &mut f64,
        var_uc_js0sws_rv_slot: &mut f64,
        var_uc_ndepm_slot: &mut f64,
        var_uc_ndepm_dn0_slot: &mut f64,
        var_uc_ndepm_dn10_slot: &mut f64,
        var_uc_ndepm_dn13_slot: &mut f64,
        var_uc_ndepm_dn2_slot: &mut f64,
        var_uc_ndepm_dn4_slot: &mut f64,
        var_uc_ndepm_dn5_slot: &mut f64,
        var_uc_ndepm_dn6_slot: &mut f64,
        var_uc_ndepm_dn7_slot: &mut f64,
        var_uc_ndepm_dn8_slot: &mut f64,
        var_uc_ndepm_dn9_slot: &mut f64,
        var_uc_ndepm_rv_slot: &mut f64,
        var_uc_njd_slot: &mut f64,
        var_uc_njd_rv_slot: &mut f64,
        var_uc_njs_slot: &mut f64,
        var_uc_njs_rv_slot: &mut f64,
        var_uc_rdvd_slot: &mut f64,
        var_uc_rdvd_rv_slot: &mut f64,
        var_uc_rdvg11_slot: &mut f64,
        var_uc_rdvg11_rv_slot: &mut f64,
        var_uc_rs_slot: &mut f64,
        var_uc_rs_rv_slot: &mut f64,
        var_uc_rth0_slot: &mut f64,
        var_uc_rth0_rv_slot: &mut f64,
        var_uc_vdiffjd_slot: &mut f64,
        var_uc_vdiffjd_rv_slot: &mut f64,
        var_uc_vdiffjs_slot: &mut f64,
        var_uc_vdiffjs_rv_slot: &mut f64,
        var_uc_vover_slot: &mut f64,
        var_uc_vover_rv_slot: &mut f64,
    ) {
        let mut var_guard185: f64 = *var_guard185_slot;
        let mut var_guard185_rv: f64 = *var_guard185_rv_slot;
        let mut var_guard186: f64 = *var_guard186_slot;
        let mut var_guard186_rv: f64 = *var_guard186_rv_slot;
        let mut var_guard187: f64 = *var_guard187_slot;
        let mut var_guard187_rv: f64 = *var_guard187_rv_slot;
        let mut var_guard188: f64 = *var_guard188_slot;
        let mut var_guard188_rv: f64 = *var_guard188_rv_slot;
        let mut var_guard189: f64 = *var_guard189_slot;
        let mut var_guard189_rv: f64 = *var_guard189_rv_slot;
        let mut var_guard190: f64 = *var_guard190_slot;
        let mut var_guard190_rv: f64 = *var_guard190_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_uc_depleak: f64 = *var_uc_depleak_slot;
        let mut var_uc_depleak_dn0: f64 = *var_uc_depleak_dn0_slot;
        let mut var_uc_depleak_dn10: f64 = *var_uc_depleak_dn10_slot;
        let mut var_uc_depleak_dn13: f64 = *var_uc_depleak_dn13_slot;
        let mut var_uc_depleak_dn2: f64 = *var_uc_depleak_dn2_slot;
        let mut var_uc_depleak_dn4: f64 = *var_uc_depleak_dn4_slot;
        let mut var_uc_depleak_dn5: f64 = *var_uc_depleak_dn5_slot;
        let mut var_uc_depleak_dn6: f64 = *var_uc_depleak_dn6_slot;
        let mut var_uc_depleak_dn7: f64 = *var_uc_depleak_dn7_slot;
        let mut var_uc_depleak_dn8: f64 = *var_uc_depleak_dn8_slot;
        let mut var_uc_depleak_dn9: f64 = *var_uc_depleak_dn9_slot;
        let mut var_uc_depleak_rv: f64 = *var_uc_depleak_rv_slot;
        let mut var_uc_depmue0: f64 = *var_uc_depmue0_slot;
        let mut var_uc_depmue0_dn0: f64 = *var_uc_depmue0_dn0_slot;
        let mut var_uc_depmue0_dn10: f64 = *var_uc_depmue0_dn10_slot;
        let mut var_uc_depmue0_dn13: f64 = *var_uc_depmue0_dn13_slot;
        let mut var_uc_depmue0_dn2: f64 = *var_uc_depmue0_dn2_slot;
        let mut var_uc_depmue0_dn4: f64 = *var_uc_depmue0_dn4_slot;
        let mut var_uc_depmue0_dn5: f64 = *var_uc_depmue0_dn5_slot;
        let mut var_uc_depmue0_dn6: f64 = *var_uc_depmue0_dn6_slot;
        let mut var_uc_depmue0_dn7: f64 = *var_uc_depmue0_dn7_slot;
        let mut var_uc_depmue0_dn8: f64 = *var_uc_depmue0_dn8_slot;
        let mut var_uc_depmue0_dn9: f64 = *var_uc_depmue0_dn9_slot;
        let mut var_uc_depmue0_rv: f64 = *var_uc_depmue0_rv_slot;
        let mut var_uc_depmue1: f64 = *var_uc_depmue1_slot;
        let mut var_uc_depmue1_dn0: f64 = *var_uc_depmue1_dn0_slot;
        let mut var_uc_depmue1_dn10: f64 = *var_uc_depmue1_dn10_slot;
        let mut var_uc_depmue1_dn13: f64 = *var_uc_depmue1_dn13_slot;
        let mut var_uc_depmue1_dn2: f64 = *var_uc_depmue1_dn2_slot;
        let mut var_uc_depmue1_dn4: f64 = *var_uc_depmue1_dn4_slot;
        let mut var_uc_depmue1_dn5: f64 = *var_uc_depmue1_dn5_slot;
        let mut var_uc_depmue1_dn6: f64 = *var_uc_depmue1_dn6_slot;
        let mut var_uc_depmue1_dn7: f64 = *var_uc_depmue1_dn7_slot;
        let mut var_uc_depmue1_dn8: f64 = *var_uc_depmue1_dn8_slot;
        let mut var_uc_depmue1_dn9: f64 = *var_uc_depmue1_dn9_slot;
        let mut var_uc_depmue1_rv: f64 = *var_uc_depmue1_rv_slot;
        let mut var_uc_depmueback0: f64 = *var_uc_depmueback0_slot;
        let mut var_uc_depmueback0_dn0: f64 = *var_uc_depmueback0_dn0_slot;
        let mut var_uc_depmueback0_dn10: f64 = *var_uc_depmueback0_dn10_slot;
        let mut var_uc_depmueback0_dn13: f64 = *var_uc_depmueback0_dn13_slot;
        let mut var_uc_depmueback0_dn2: f64 = *var_uc_depmueback0_dn2_slot;
        let mut var_uc_depmueback0_dn4: f64 = *var_uc_depmueback0_dn4_slot;
        let mut var_uc_depmueback0_dn5: f64 = *var_uc_depmueback0_dn5_slot;
        let mut var_uc_depmueback0_dn6: f64 = *var_uc_depmueback0_dn6_slot;
        let mut var_uc_depmueback0_dn7: f64 = *var_uc_depmueback0_dn7_slot;
        let mut var_uc_depmueback0_dn8: f64 = *var_uc_depmueback0_dn8_slot;
        let mut var_uc_depmueback0_dn9: f64 = *var_uc_depmueback0_dn9_slot;
        let mut var_uc_depmueback0_rv: f64 = *var_uc_depmueback0_rv_slot;
        let mut var_uc_depvmax: f64 = *var_uc_depvmax_slot;
        let mut var_uc_depvmax_dn0: f64 = *var_uc_depvmax_dn0_slot;
        let mut var_uc_depvmax_dn10: f64 = *var_uc_depvmax_dn10_slot;
        let mut var_uc_depvmax_dn13: f64 = *var_uc_depvmax_dn13_slot;
        let mut var_uc_depvmax_dn2: f64 = *var_uc_depvmax_dn2_slot;
        let mut var_uc_depvmax_dn4: f64 = *var_uc_depvmax_dn4_slot;
        let mut var_uc_depvmax_dn5: f64 = *var_uc_depvmax_dn5_slot;
        let mut var_uc_depvmax_dn6: f64 = *var_uc_depvmax_dn6_slot;
        let mut var_uc_depvmax_dn7: f64 = *var_uc_depvmax_dn7_slot;
        let mut var_uc_depvmax_dn8: f64 = *var_uc_depvmax_dn8_slot;
        let mut var_uc_depvmax_dn9: f64 = *var_uc_depvmax_dn9_slot;
        let mut var_uc_depvmax_rv: f64 = *var_uc_depvmax_rv_slot;
        let mut var_uc_js0d: f64 = *var_uc_js0d_slot;
        let mut var_uc_js0d_rv: f64 = *var_uc_js0d_rv_slot;
        let mut var_uc_js0s: f64 = *var_uc_js0s_slot;
        let mut var_uc_js0s_rv: f64 = *var_uc_js0s_rv_slot;
        let mut var_uc_js0swd: f64 = *var_uc_js0swd_slot;
        let mut var_uc_js0swd_rv: f64 = *var_uc_js0swd_rv_slot;
        let mut var_uc_js0sws: f64 = *var_uc_js0sws_slot;
        let mut var_uc_js0sws_rv: f64 = *var_uc_js0sws_rv_slot;
        let mut var_uc_ndepm: f64 = *var_uc_ndepm_slot;
        let mut var_uc_ndepm_dn0: f64 = *var_uc_ndepm_dn0_slot;
        let mut var_uc_ndepm_dn10: f64 = *var_uc_ndepm_dn10_slot;
        let mut var_uc_ndepm_dn13: f64 = *var_uc_ndepm_dn13_slot;
        let mut var_uc_ndepm_dn2: f64 = *var_uc_ndepm_dn2_slot;
        let mut var_uc_ndepm_dn4: f64 = *var_uc_ndepm_dn4_slot;
        let mut var_uc_ndepm_dn5: f64 = *var_uc_ndepm_dn5_slot;
        let mut var_uc_ndepm_dn6: f64 = *var_uc_ndepm_dn6_slot;
        let mut var_uc_ndepm_dn7: f64 = *var_uc_ndepm_dn7_slot;
        let mut var_uc_ndepm_dn8: f64 = *var_uc_ndepm_dn8_slot;
        let mut var_uc_ndepm_dn9: f64 = *var_uc_ndepm_dn9_slot;
        let mut var_uc_ndepm_rv: f64 = *var_uc_ndepm_rv_slot;
        let mut var_uc_njd: f64 = *var_uc_njd_slot;
        let mut var_uc_njd_rv: f64 = *var_uc_njd_rv_slot;
        let mut var_uc_njs: f64 = *var_uc_njs_slot;
        let mut var_uc_njs_rv: f64 = *var_uc_njs_rv_slot;
        let mut var_uc_rdvd: f64 = *var_uc_rdvd_slot;
        let mut var_uc_rdvd_rv: f64 = *var_uc_rdvd_rv_slot;
        let mut var_uc_rdvg11: f64 = *var_uc_rdvg11_slot;
        let mut var_uc_rdvg11_rv: f64 = *var_uc_rdvg11_rv_slot;
        let mut var_uc_rs: f64 = *var_uc_rs_slot;
        let mut var_uc_rs_rv: f64 = *var_uc_rs_rv_slot;
        let mut var_uc_rth0: f64 = *var_uc_rth0_slot;
        let mut var_uc_rth0_rv: f64 = *var_uc_rth0_rv_slot;
        let mut var_uc_vdiffjd: f64 = *var_uc_vdiffjd_slot;
        let mut var_uc_vdiffjd_rv: f64 = *var_uc_vdiffjd_rv_slot;
        let mut var_uc_vdiffjs: f64 = *var_uc_vdiffjs_slot;
        let mut var_uc_vdiffjs_rv: f64 = *var_uc_vdiffjs_rv_slot;
        let mut var_uc_vover: f64 = *var_uc_vover_slot;
        let mut var_uc_vover_rv: f64 = *var_uc_vover_rv_slot;

        let assign8950_e4447: f64 = (p.p638 / var_lbin);
        let assign8950_e4448: f64 = (p.p82 + assign8950_e4447);
        let assign8950_e4451: f64 = (p.p726 / var_wbin);
        let assign8950_e4452: f64 = (assign8950_e4448 + assign8950_e4451);
        let assign8950_e4455: f64 = (p.p814 / var_lwbin);
        let assign8950_e4456: f64 = (assign8950_e4452 + assign8950_e4455);
        var_uc_rdvd = assign8950_e4456;
        var_uc_rdvd_rv = 0.0;

        let assign8960_e4460: f64 = (p.p639 / var_lbin);
        let assign8960_e4461: f64 = (p.p61 + assign8960_e4460);
        let assign8960_e4464: f64 = (p.p727 / var_wbin);
        let assign8960_e4465: f64 = (assign8960_e4461 + assign8960_e4464);
        let assign8960_e4468: f64 = (p.p815 / var_lwbin);
        let assign8960_e4469: f64 = (assign8960_e4465 + assign8960_e4468);
        var_uc_rdvg11 = assign8960_e4469;
        var_uc_rdvg11_rv = 0.0;

        let assign8970_e4473: f64 = (p.p640 / var_lbin);
        let assign8970_e4474: f64 = (p.p75 + assign8970_e4473);
        let assign8970_e4477: f64 = (p.p728 / var_wbin);
        let assign8970_e4478: f64 = (assign8970_e4474 + assign8970_e4477);
        let assign8970_e4481: f64 = (p.p816 / var_lwbin);
        let assign8970_e4482: f64 = (assign8970_e4478 + assign8970_e4481);
        var_uc_rs = assign8970_e4482;
        var_uc_rs_rv = 0.0;

        let assign8980_e4486: f64 = (p.p641 / var_lbin);
        let assign8980_e4487: f64 = (p.p80 + assign8980_e4486);
        let assign8980_e4490: f64 = (p.p729 / var_wbin);
        let assign8980_e4491: f64 = (assign8980_e4487 + assign8980_e4490);
        let assign8980_e4494: f64 = (p.p817 / var_lwbin);
        let assign8980_e4495: f64 = (assign8980_e4491 + assign8980_e4494);
        var_uc_rth0 = assign8980_e4495;
        var_uc_rth0_rv = 0.0;

        let assign8990_e4499: f64 = (p.p642 / var_lbin);
        let assign8990_e4500: f64 = (p.p77 + assign8990_e4499);
        let assign8990_e4503: f64 = (p.p730 / var_wbin);
        let assign8990_e4504: f64 = (assign8990_e4500 + assign8990_e4503);
        let assign8990_e4507: f64 = (p.p818 / var_lwbin);
        let assign8990_e4508: f64 = (assign8990_e4504 + assign8990_e4507);
        var_uc_vover = assign8990_e4508;
        var_uc_vover_rv = 0.0;

        let assign9000_e4512: f64 = (p.p824 / var_lbin);
        let assign9000_e4513: f64 = (p.p493 + assign9000_e4512);
        let assign9000_e4516: f64 = (p.p839 / var_wbin);
        let assign9000_e4517: f64 = (assign9000_e4513 + assign9000_e4516);
        let assign9000_e4520: f64 = (p.p854 / var_lwbin);
        let assign9000_e4521: f64 = (assign9000_e4517 + assign9000_e4520);
        var_uc_js0d = assign9000_e4521;
        var_uc_js0d_rv = 0.0;

        let assign9010_e4525: f64 = (p.p825 / var_lbin);
        let assign9010_e4526: f64 = (p.p494 + assign9010_e4525);
        let assign9010_e4529: f64 = (p.p840 / var_wbin);
        let assign9010_e4530: f64 = (assign9010_e4526 + assign9010_e4529);
        let assign9010_e4533: f64 = (p.p855 / var_lwbin);
        let assign9010_e4534: f64 = (assign9010_e4530 + assign9010_e4533);
        var_uc_js0swd = assign9010_e4534;
        var_uc_js0swd_rv = 0.0;

        let assign9020_e4538: f64 = (p.p826 / var_lbin);
        let assign9020_e4539: f64 = (p.p496 + assign9020_e4538);
        let assign9020_e4542: f64 = (p.p841 / var_wbin);
        let assign9020_e4543: f64 = (assign9020_e4539 + assign9020_e4542);
        let assign9020_e4546: f64 = (p.p856 / var_lwbin);
        let assign9020_e4547: f64 = (assign9020_e4543 + assign9020_e4546);
        var_uc_njd = assign9020_e4547;
        var_uc_njd_rv = 0.0;

        let assign9040_e4564: f64 = (p.p828 / var_lbin);
        let assign9040_e4565: f64 = (p.p515 + assign9040_e4564);
        let assign9040_e4568: f64 = (p.p843 / var_wbin);
        let assign9040_e4569: f64 = (assign9040_e4565 + assign9040_e4568);
        let assign9040_e4572: f64 = (p.p858 / var_lwbin);
        let assign9040_e4573: f64 = (assign9040_e4569 + assign9040_e4572);
        var_uc_vdiffjd = assign9040_e4573;
        var_uc_vdiffjd_rv = 0.0;

        let assign9050_e4577: f64 = (p.p829 / var_lbin);
        let assign9050_e4578: f64 = (p.p516 + assign9050_e4577);
        let assign9050_e4581: f64 = (p.p844 / var_wbin);
        let assign9050_e4582: f64 = (assign9050_e4578 + assign9050_e4581);
        let assign9050_e4585: f64 = (p.p859 / var_lwbin);
        let assign9050_e4586: f64 = (assign9050_e4582 + assign9050_e4585);
        var_uc_js0s = assign9050_e4586;
        var_uc_js0s_rv = 0.0;

        let assign9060_e4590: f64 = (p.p830 / var_lbin);
        let assign9060_e4591: f64 = (p.p517 + assign9060_e4590);
        let assign9060_e4594: f64 = (p.p845 / var_wbin);
        let assign9060_e4595: f64 = (assign9060_e4591 + assign9060_e4594);
        let assign9060_e4598: f64 = (p.p860 / var_lwbin);
        let assign9060_e4599: f64 = (assign9060_e4595 + assign9060_e4598);
        var_uc_js0sws = assign9060_e4599;
        var_uc_js0sws_rv = 0.0;

        let assign9070_e4603: f64 = (p.p831 / var_lbin);
        let assign9070_e4604: f64 = (p.p519 + assign9070_e4603);
        let assign9070_e4607: f64 = (p.p846 / var_wbin);
        let assign9070_e4608: f64 = (assign9070_e4604 + assign9070_e4607);
        let assign9070_e4611: f64 = (p.p861 / var_lwbin);
        let assign9070_e4612: f64 = (assign9070_e4608 + assign9070_e4611);
        var_uc_njs = assign9070_e4612;
        var_uc_njs_rv = 0.0;

        let assign9090_e4629: f64 = (p.p833 / var_lbin);
        let assign9090_e4630: f64 = (p.p538 + assign9090_e4629);
        let assign9090_e4633: f64 = (p.p848 / var_wbin);
        let assign9090_e4634: f64 = (assign9090_e4630 + assign9090_e4633);
        let assign9090_e4637: f64 = (p.p863 / var_lwbin);
        let assign9090_e4638: f64 = (assign9090_e4634 + assign9090_e4637);
        var_uc_vdiffjs = assign9090_e4638;
        var_uc_vdiffjs_rv = 0.0;

        let assign9190_e4689: f64 = if var_uc_codep != 0.0 { 1.0 } else { 0.0 };
        var_guard185 = assign9190_e4689;
        var_guard185_rv = 0.0;

        let (assign9200_e4695, assign9200_e4695_d_n0, assign9200_e4695_d_n2, assign9200_e4695_d_n4, assign9200_e4695_d_n5, assign9200_e4695_d_n6, assign9200_e4695_d_n7, assign9200_e4695_d_n8, assign9200_e4695_d_n9, assign9200_e4695_d_n10, assign9200_e4695_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9200_e4693: f64 = (var_lg).powf(p.p342);
        (assign9200_e4693, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign9200_e4695;
        var_t3_dn0 = assign9200_e4695_d_n0;
        var_t3_dn2 = assign9200_e4695_d_n2;
        var_t3_dn4 = assign9200_e4695_d_n4;
        var_t3_dn5 = assign9200_e4695_d_n5;
        var_t3_dn6 = assign9200_e4695_d_n6;
        var_t3_dn7 = assign9200_e4695_d_n7;
        var_t3_dn8 = assign9200_e4695_d_n8;
        var_t3_dn9 = assign9200_e4695_d_n9;
        var_t3_dn10 = assign9200_e4695_d_n10;
        var_t3_dn13 = assign9200_e4695_d_n13;
        var_t3_rv = 0.0;

        let (assign9210_e4705, assign9210_e4705_d_n0, assign9210_e4705_d_n2, assign9210_e4705_d_n4, assign9210_e4705_d_n5, assign9210_e4705_d_n6, assign9210_e4705_d_n7, assign9210_e4705_d_n8, assign9210_e4705_d_n9, assign9210_e4705_d_n10, assign9210_e4705_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9210_e4701: f64 = (p.p341 / var_t3);
        let assign9210_e4702: f64 = (1.0 + assign9210_e4701);
        let assign9210_e4703: f64 = (var_uc_ndepm * assign9210_e4702);
        (assign9210_e4703, ((var_uc_ndepm_dn0 * assign9210_e4702) + (var_uc_ndepm * (-((p.p341 * var_t3_dn0) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn2 * assign9210_e4702) + (var_uc_ndepm * (-((p.p341 * var_t3_dn2) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn4 * assign9210_e4702) + (var_uc_ndepm * (-((p.p341 * var_t3_dn4) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn5 * assign9210_e4702) + (var_uc_ndepm * (-((p.p341 * var_t3_dn5) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn6 * assign9210_e4702) + (var_uc_ndepm * (-((p.p341 * var_t3_dn6) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn7 * assign9210_e4702) + (var_uc_ndepm * (-((p.p341 * var_t3_dn7) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn8 * assign9210_e4702) + (var_uc_ndepm * (-((p.p341 * var_t3_dn8) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn9 * assign9210_e4702) + (var_uc_ndepm * (-((p.p341 * var_t3_dn9) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn10 * assign9210_e4702) + (var_uc_ndepm * (-((p.p341 * var_t3_dn10) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn13 * assign9210_e4702) + (var_uc_ndepm * (-((p.p341 * var_t3_dn13) / (var_t3 * var_t3))))),)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn13,)
    }
};
        var_uc_ndepm = assign9210_e4705;
        var_uc_ndepm_dn0 = assign9210_e4705_d_n0;
        var_uc_ndepm_dn2 = assign9210_e4705_d_n2;
        var_uc_ndepm_dn4 = assign9210_e4705_d_n4;
        var_uc_ndepm_dn5 = assign9210_e4705_d_n5;
        var_uc_ndepm_dn6 = assign9210_e4705_d_n6;
        var_uc_ndepm_dn7 = assign9210_e4705_d_n7;
        var_uc_ndepm_dn8 = assign9210_e4705_d_n8;
        var_uc_ndepm_dn9 = assign9210_e4705_d_n9;
        var_uc_ndepm_dn10 = assign9210_e4705_d_n10;
        var_uc_ndepm_dn13 = assign9210_e4705_d_n13;
        var_uc_ndepm_rv = 0.0;

        let assign9220_e4708: f64 = if var_uc_ndepm < 1e21 { 1.0 } else { 0.0 };
        var_guard186 = assign9220_e4708;
        var_guard186_rv = 0.0;

        let (assign9230_e4714, assign9230_e4714_d_n0, assign9230_e4714_d_n2, assign9230_e4714_d_n4, assign9230_e4714_d_n5, assign9230_e4714_d_n6, assign9230_e4714_d_n7, assign9230_e4714_d_n8, assign9230_e4714_d_n9, assign9230_e4714_d_n10, assign9230_e4714_d_n13,) = {
    if ((var_guard185 != 0.0) && (var_guard186 != 0.0)) {
        (1e21, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn13,)
    }
};
        var_uc_ndepm = assign9230_e4714;
        var_uc_ndepm_dn0 = assign9230_e4714_d_n0;
        var_uc_ndepm_dn2 = assign9230_e4714_d_n2;
        var_uc_ndepm_dn4 = assign9230_e4714_d_n4;
        var_uc_ndepm_dn5 = assign9230_e4714_d_n5;
        var_uc_ndepm_dn6 = assign9230_e4714_d_n6;
        var_uc_ndepm_dn7 = assign9230_e4714_d_n7;
        var_uc_ndepm_dn8 = assign9230_e4714_d_n8;
        var_uc_ndepm_dn9 = assign9230_e4714_d_n9;
        var_uc_ndepm_dn10 = assign9230_e4714_d_n10;
        var_uc_ndepm_dn13 = assign9230_e4714_d_n13;
        var_uc_ndepm_rv = 0.0;

        let (assign9240_e4720, assign9240_e4720_d_n0, assign9240_e4720_d_n2, assign9240_e4720_d_n4, assign9240_e4720_d_n5, assign9240_e4720_d_n6, assign9240_e4720_d_n7, assign9240_e4720_d_n8, assign9240_e4720_d_n9, assign9240_e4720_d_n10, assign9240_e4720_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9240_e4718: f64 = (var_lg).powf(p.p369);
        (assign9240_e4718, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign9240_e4720;
        var_t3_dn0 = assign9240_e4720_d_n0;
        var_t3_dn2 = assign9240_e4720_d_n2;
        var_t3_dn4 = assign9240_e4720_d_n4;
        var_t3_dn5 = assign9240_e4720_d_n5;
        var_t3_dn6 = assign9240_e4720_d_n6;
        var_t3_dn7 = assign9240_e4720_d_n7;
        var_t3_dn8 = assign9240_e4720_d_n8;
        var_t3_dn9 = assign9240_e4720_d_n9;
        var_t3_dn10 = assign9240_e4720_d_n10;
        var_t3_dn13 = assign9240_e4720_d_n13;
        var_t3_rv = 0.0;

        let (assign9250_e4730, assign9250_e4730_d_n0, assign9250_e4730_d_n2, assign9250_e4730_d_n4, assign9250_e4730_d_n5, assign9250_e4730_d_n6, assign9250_e4730_d_n7, assign9250_e4730_d_n8, assign9250_e4730_d_n9, assign9250_e4730_d_n10, assign9250_e4730_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9250_e4726: f64 = (p.p368 / var_t3);
        let assign9250_e4727: f64 = (1.0 + assign9250_e4726);
        let assign9250_e4728: f64 = (var_uc_depvmax * assign9250_e4727);
        (assign9250_e4728, ((var_uc_depvmax_dn0 * assign9250_e4727) + (var_uc_depvmax * (-((p.p368 * var_t3_dn0) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn2 * assign9250_e4727) + (var_uc_depvmax * (-((p.p368 * var_t3_dn2) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn4 * assign9250_e4727) + (var_uc_depvmax * (-((p.p368 * var_t3_dn4) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn5 * assign9250_e4727) + (var_uc_depvmax * (-((p.p368 * var_t3_dn5) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn6 * assign9250_e4727) + (var_uc_depvmax * (-((p.p368 * var_t3_dn6) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn7 * assign9250_e4727) + (var_uc_depvmax * (-((p.p368 * var_t3_dn7) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn8 * assign9250_e4727) + (var_uc_depvmax * (-((p.p368 * var_t3_dn8) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn9 * assign9250_e4727) + (var_uc_depvmax * (-((p.p368 * var_t3_dn9) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn10 * assign9250_e4727) + (var_uc_depvmax * (-((p.p368 * var_t3_dn10) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn13 * assign9250_e4727) + (var_uc_depvmax * (-((p.p368 * var_t3_dn13) / (var_t3 * var_t3))))),)
    } else {
        (var_uc_depvmax, var_uc_depvmax_dn0, var_uc_depvmax_dn2, var_uc_depvmax_dn4, var_uc_depvmax_dn5, var_uc_depvmax_dn6, var_uc_depvmax_dn7, var_uc_depvmax_dn8, var_uc_depvmax_dn9, var_uc_depvmax_dn10, var_uc_depvmax_dn13,)
    }
};
        var_uc_depvmax = assign9250_e4730;
        var_uc_depvmax_dn0 = assign9250_e4730_d_n0;
        var_uc_depvmax_dn2 = assign9250_e4730_d_n2;
        var_uc_depvmax_dn4 = assign9250_e4730_d_n4;
        var_uc_depvmax_dn5 = assign9250_e4730_d_n5;
        var_uc_depvmax_dn6 = assign9250_e4730_d_n6;
        var_uc_depvmax_dn7 = assign9250_e4730_d_n7;
        var_uc_depvmax_dn8 = assign9250_e4730_d_n8;
        var_uc_depvmax_dn9 = assign9250_e4730_d_n9;
        var_uc_depvmax_dn10 = assign9250_e4730_d_n10;
        var_uc_depvmax_dn13 = assign9250_e4730_d_n13;
        var_uc_depvmax_rv = 0.0;

        let (assign9260_e4736, assign9260_e4736_d_n0, assign9260_e4736_d_n2, assign9260_e4736_d_n4, assign9260_e4736_d_n5, assign9260_e4736_d_n6, assign9260_e4736_d_n7, assign9260_e4736_d_n8, assign9260_e4736_d_n9, assign9260_e4736_d_n10, assign9260_e4736_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9260_e4734: f64 = (var_lg).powf(p.p362);
        (assign9260_e4734, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign9260_e4736;
        var_t3_dn0 = assign9260_e4736_d_n0;
        var_t3_dn2 = assign9260_e4736_d_n2;
        var_t3_dn4 = assign9260_e4736_d_n4;
        var_t3_dn5 = assign9260_e4736_d_n5;
        var_t3_dn6 = assign9260_e4736_d_n6;
        var_t3_dn7 = assign9260_e4736_d_n7;
        var_t3_dn8 = assign9260_e4736_d_n8;
        var_t3_dn9 = assign9260_e4736_d_n9;
        var_t3_dn10 = assign9260_e4736_d_n10;
        var_t3_dn13 = assign9260_e4736_d_n13;
        var_t3_rv = 0.0;

        let (assign9270_e4746, assign9270_e4746_d_n0, assign9270_e4746_d_n2, assign9270_e4746_d_n4, assign9270_e4746_d_n5, assign9270_e4746_d_n6, assign9270_e4746_d_n7, assign9270_e4746_d_n8, assign9270_e4746_d_n9, assign9270_e4746_d_n10, assign9270_e4746_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9270_e4742: f64 = (p.p361 / var_t3);
        let assign9270_e4743: f64 = (1.0 + assign9270_e4742);
        let assign9270_e4744: f64 = (p.p360 * assign9270_e4743);
        (assign9270_e4744, (p.p360 * (-((p.p361 * var_t3_dn0) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn2) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn4) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn5) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn6) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn7) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn8) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn9) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn10) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn13) / (var_t3 * var_t3)))),)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn13,)
    }
};
        var_uc_depleak = assign9270_e4746;
        var_uc_depleak_dn0 = assign9270_e4746_d_n0;
        var_uc_depleak_dn2 = assign9270_e4746_d_n2;
        var_uc_depleak_dn4 = assign9270_e4746_d_n4;
        var_uc_depleak_dn5 = assign9270_e4746_d_n5;
        var_uc_depleak_dn6 = assign9270_e4746_d_n6;
        var_uc_depleak_dn7 = assign9270_e4746_d_n7;
        var_uc_depleak_dn8 = assign9270_e4746_d_n8;
        var_uc_depleak_dn9 = assign9270_e4746_d_n9;
        var_uc_depleak_dn10 = assign9270_e4746_d_n10;
        var_uc_depleak_dn13 = assign9270_e4746_d_n13;
        var_uc_depleak_rv = 0.0;

        let assign9280_e4749: f64 = if var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        var_guard187 = assign9280_e4749;
        var_guard187_rv = 0.0;

        let (assign9290_e4755, assign9290_e4755_d_n0, assign9290_e4755_d_n2, assign9290_e4755_d_n4, assign9290_e4755_d_n5, assign9290_e4755_d_n6, assign9290_e4755_d_n7, assign9290_e4755_d_n8, assign9290_e4755_d_n9, assign9290_e4755_d_n10, assign9290_e4755_d_n13,) = {
    if ((var_guard185 != 0.0) && (var_guard187 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn13,)
    }
};
        var_uc_depleak = assign9290_e4755;
        var_uc_depleak_dn0 = assign9290_e4755_d_n0;
        var_uc_depleak_dn2 = assign9290_e4755_d_n2;
        var_uc_depleak_dn4 = assign9290_e4755_d_n4;
        var_uc_depleak_dn5 = assign9290_e4755_d_n5;
        var_uc_depleak_dn6 = assign9290_e4755_d_n6;
        var_uc_depleak_dn7 = assign9290_e4755_d_n7;
        var_uc_depleak_dn8 = assign9290_e4755_d_n8;
        var_uc_depleak_dn9 = assign9290_e4755_d_n9;
        var_uc_depleak_dn10 = assign9290_e4755_d_n10;
        var_uc_depleak_dn13 = assign9290_e4755_d_n13;
        var_uc_depleak_rv = 0.0;

        let (assign9300_e4761, assign9300_e4761_d_n0, assign9300_e4761_d_n2, assign9300_e4761_d_n4, assign9300_e4761_d_n5, assign9300_e4761_d_n6, assign9300_e4761_d_n7, assign9300_e4761_d_n8, assign9300_e4761_d_n9, assign9300_e4761_d_n10, assign9300_e4761_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9300_e4759: f64 = (var_lg).powf(p.p348);
        (assign9300_e4759, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign9300_e4761;
        var_t3_dn0 = assign9300_e4761_d_n0;
        var_t3_dn2 = assign9300_e4761_d_n2;
        var_t3_dn4 = assign9300_e4761_d_n4;
        var_t3_dn5 = assign9300_e4761_d_n5;
        var_t3_dn6 = assign9300_e4761_d_n6;
        var_t3_dn7 = assign9300_e4761_d_n7;
        var_t3_dn8 = assign9300_e4761_d_n8;
        var_t3_dn9 = assign9300_e4761_d_n9;
        var_t3_dn10 = assign9300_e4761_d_n10;
        var_t3_dn13 = assign9300_e4761_d_n13;
        var_t3_rv = 0.0;

        let (assign9310_e4771, assign9310_e4771_d_n0, assign9310_e4771_d_n2, assign9310_e4771_d_n4, assign9310_e4771_d_n5, assign9310_e4771_d_n6, assign9310_e4771_d_n7, assign9310_e4771_d_n8, assign9310_e4771_d_n9, assign9310_e4771_d_n10, assign9310_e4771_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9310_e4767: f64 = (p.p347 / var_t3);
        let assign9310_e4768: f64 = (1.0 + assign9310_e4767);
        let assign9310_e4769: f64 = (p.p346 * assign9310_e4768);
        (assign9310_e4769, (p.p346 * (-((p.p347 * var_t3_dn0) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn2) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn4) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn5) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn6) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn7) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn8) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn9) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn10) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn13) / (var_t3 * var_t3)))),)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn13,)
    }
};
        var_uc_depmue0 = assign9310_e4771;
        var_uc_depmue0_dn0 = assign9310_e4771_d_n0;
        var_uc_depmue0_dn2 = assign9310_e4771_d_n2;
        var_uc_depmue0_dn4 = assign9310_e4771_d_n4;
        var_uc_depmue0_dn5 = assign9310_e4771_d_n5;
        var_uc_depmue0_dn6 = assign9310_e4771_d_n6;
        var_uc_depmue0_dn7 = assign9310_e4771_d_n7;
        var_uc_depmue0_dn8 = assign9310_e4771_d_n8;
        var_uc_depmue0_dn9 = assign9310_e4771_d_n9;
        var_uc_depmue0_dn10 = assign9310_e4771_d_n10;
        var_uc_depmue0_dn13 = assign9310_e4771_d_n13;
        var_uc_depmue0_rv = 0.0;

        let assign9320_e4774: f64 = if var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        var_guard188 = assign9320_e4774;
        var_guard188_rv = 0.0;

        let (assign9330_e4780, assign9330_e4780_d_n0, assign9330_e4780_d_n2, assign9330_e4780_d_n4, assign9330_e4780_d_n5, assign9330_e4780_d_n6, assign9330_e4780_d_n7, assign9330_e4780_d_n8, assign9330_e4780_d_n9, assign9330_e4780_d_n10, assign9330_e4780_d_n13,) = {
    if ((var_guard185 != 0.0) && (var_guard188 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn13,)
    }
};
        var_uc_depmue0 = assign9330_e4780;
        var_uc_depmue0_dn0 = assign9330_e4780_d_n0;
        var_uc_depmue0_dn2 = assign9330_e4780_d_n2;
        var_uc_depmue0_dn4 = assign9330_e4780_d_n4;
        var_uc_depmue0_dn5 = assign9330_e4780_d_n5;
        var_uc_depmue0_dn6 = assign9330_e4780_d_n6;
        var_uc_depmue0_dn7 = assign9330_e4780_d_n7;
        var_uc_depmue0_dn8 = assign9330_e4780_d_n8;
        var_uc_depmue0_dn9 = assign9330_e4780_d_n9;
        var_uc_depmue0_dn10 = assign9330_e4780_d_n10;
        var_uc_depmue0_dn13 = assign9330_e4780_d_n13;
        var_uc_depmue0_rv = 0.0;

        let (assign9340_e4786, assign9340_e4786_d_n0, assign9340_e4786_d_n2, assign9340_e4786_d_n4, assign9340_e4786_d_n5, assign9340_e4786_d_n6, assign9340_e4786_d_n7, assign9340_e4786_d_n8, assign9340_e4786_d_n9, assign9340_e4786_d_n10, assign9340_e4786_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9340_e4784: f64 = (var_lg).powf(p.p351);
        (assign9340_e4784, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign9340_e4786;
        var_t3_dn0 = assign9340_e4786_d_n0;
        var_t3_dn2 = assign9340_e4786_d_n2;
        var_t3_dn4 = assign9340_e4786_d_n4;
        var_t3_dn5 = assign9340_e4786_d_n5;
        var_t3_dn6 = assign9340_e4786_d_n6;
        var_t3_dn7 = assign9340_e4786_d_n7;
        var_t3_dn8 = assign9340_e4786_d_n8;
        var_t3_dn9 = assign9340_e4786_d_n9;
        var_t3_dn10 = assign9340_e4786_d_n10;
        var_t3_dn13 = assign9340_e4786_d_n13;
        var_t3_rv = 0.0;

        let (assign9350_e4796, assign9350_e4796_d_n0, assign9350_e4796_d_n2, assign9350_e4796_d_n4, assign9350_e4796_d_n5, assign9350_e4796_d_n6, assign9350_e4796_d_n7, assign9350_e4796_d_n8, assign9350_e4796_d_n9, assign9350_e4796_d_n10, assign9350_e4796_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9350_e4792: f64 = (p.p350 / var_t3);
        let assign9350_e4793: f64 = (1.0 + assign9350_e4792);
        let assign9350_e4794: f64 = (p.p349 * assign9350_e4793);
        (assign9350_e4794, (p.p349 * (-((p.p350 * var_t3_dn0) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn2) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn4) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn5) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn6) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn7) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn8) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn9) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn10) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn13) / (var_t3 * var_t3)))),)
    } else {
        (var_uc_depmue1, var_uc_depmue1_dn0, var_uc_depmue1_dn2, var_uc_depmue1_dn4, var_uc_depmue1_dn5, var_uc_depmue1_dn6, var_uc_depmue1_dn7, var_uc_depmue1_dn8, var_uc_depmue1_dn9, var_uc_depmue1_dn10, var_uc_depmue1_dn13,)
    }
};
        var_uc_depmue1 = assign9350_e4796;
        var_uc_depmue1_dn0 = assign9350_e4796_d_n0;
        var_uc_depmue1_dn2 = assign9350_e4796_d_n2;
        var_uc_depmue1_dn4 = assign9350_e4796_d_n4;
        var_uc_depmue1_dn5 = assign9350_e4796_d_n5;
        var_uc_depmue1_dn6 = assign9350_e4796_d_n6;
        var_uc_depmue1_dn7 = assign9350_e4796_d_n7;
        var_uc_depmue1_dn8 = assign9350_e4796_d_n8;
        var_uc_depmue1_dn9 = assign9350_e4796_d_n9;
        var_uc_depmue1_dn10 = assign9350_e4796_d_n10;
        var_uc_depmue1_dn13 = assign9350_e4796_d_n13;
        var_uc_depmue1_rv = 0.0;

        let assign9360_e4799: f64 = if var_uc_depmue1 < 0.0 { 1.0 } else { 0.0 };
        var_guard189 = assign9360_e4799;
        var_guard189_rv = 0.0;

        let (assign9370_e4805, assign9370_e4805_d_n0, assign9370_e4805_d_n2, assign9370_e4805_d_n4, assign9370_e4805_d_n5, assign9370_e4805_d_n6, assign9370_e4805_d_n7, assign9370_e4805_d_n8, assign9370_e4805_d_n9, assign9370_e4805_d_n10, assign9370_e4805_d_n13,) = {
    if ((var_guard185 != 0.0) && (var_guard189 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue1, var_uc_depmue1_dn0, var_uc_depmue1_dn2, var_uc_depmue1_dn4, var_uc_depmue1_dn5, var_uc_depmue1_dn6, var_uc_depmue1_dn7, var_uc_depmue1_dn8, var_uc_depmue1_dn9, var_uc_depmue1_dn10, var_uc_depmue1_dn13,)
    }
};
        var_uc_depmue1 = assign9370_e4805;
        var_uc_depmue1_dn0 = assign9370_e4805_d_n0;
        var_uc_depmue1_dn2 = assign9370_e4805_d_n2;
        var_uc_depmue1_dn4 = assign9370_e4805_d_n4;
        var_uc_depmue1_dn5 = assign9370_e4805_d_n5;
        var_uc_depmue1_dn6 = assign9370_e4805_d_n6;
        var_uc_depmue1_dn7 = assign9370_e4805_d_n7;
        var_uc_depmue1_dn8 = assign9370_e4805_d_n8;
        var_uc_depmue1_dn9 = assign9370_e4805_d_n9;
        var_uc_depmue1_dn10 = assign9370_e4805_d_n10;
        var_uc_depmue1_dn13 = assign9370_e4805_d_n13;
        var_uc_depmue1_rv = 0.0;

        let (assign9380_e4811, assign9380_e4811_d_n0, assign9380_e4811_d_n2, assign9380_e4811_d_n4, assign9380_e4811_d_n5, assign9380_e4811_d_n6, assign9380_e4811_d_n7, assign9380_e4811_d_n8, assign9380_e4811_d_n9, assign9380_e4811_d_n10, assign9380_e4811_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9380_e4809: f64 = (var_lg).powf(p.p357);
        (assign9380_e4809, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign9380_e4811;
        var_t3_dn0 = assign9380_e4811_d_n0;
        var_t3_dn2 = assign9380_e4811_d_n2;
        var_t3_dn4 = assign9380_e4811_d_n4;
        var_t3_dn5 = assign9380_e4811_d_n5;
        var_t3_dn6 = assign9380_e4811_d_n6;
        var_t3_dn7 = assign9380_e4811_d_n7;
        var_t3_dn8 = assign9380_e4811_d_n8;
        var_t3_dn9 = assign9380_e4811_d_n9;
        var_t3_dn10 = assign9380_e4811_d_n10;
        var_t3_dn13 = assign9380_e4811_d_n13;
        var_t3_rv = 0.0;

        let (assign9390_e4821, assign9390_e4821_d_n0, assign9390_e4821_d_n2, assign9390_e4821_d_n4, assign9390_e4821_d_n5, assign9390_e4821_d_n6, assign9390_e4821_d_n7, assign9390_e4821_d_n8, assign9390_e4821_d_n9, assign9390_e4821_d_n10, assign9390_e4821_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9390_e4817: f64 = (p.p356 / var_t3);
        let assign9390_e4818: f64 = (1.0 + assign9390_e4817);
        let assign9390_e4819: f64 = (p.p354 * assign9390_e4818);
        (assign9390_e4819, (p.p354 * (-((p.p356 * var_t3_dn0) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn2) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn4) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn5) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn6) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn7) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn8) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn9) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn10) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn13) / (var_t3 * var_t3)))),)
    } else {
        (var_uc_depmueback0, var_uc_depmueback0_dn0, var_uc_depmueback0_dn2, var_uc_depmueback0_dn4, var_uc_depmueback0_dn5, var_uc_depmueback0_dn6, var_uc_depmueback0_dn7, var_uc_depmueback0_dn8, var_uc_depmueback0_dn9, var_uc_depmueback0_dn10, var_uc_depmueback0_dn13,)
    }
};
        var_uc_depmueback0 = assign9390_e4821;
        var_uc_depmueback0_dn0 = assign9390_e4821_d_n0;
        var_uc_depmueback0_dn2 = assign9390_e4821_d_n2;
        var_uc_depmueback0_dn4 = assign9390_e4821_d_n4;
        var_uc_depmueback0_dn5 = assign9390_e4821_d_n5;
        var_uc_depmueback0_dn6 = assign9390_e4821_d_n6;
        var_uc_depmueback0_dn7 = assign9390_e4821_d_n7;
        var_uc_depmueback0_dn8 = assign9390_e4821_d_n8;
        var_uc_depmueback0_dn9 = assign9390_e4821_d_n9;
        var_uc_depmueback0_dn10 = assign9390_e4821_d_n10;
        var_uc_depmueback0_dn13 = assign9390_e4821_d_n13;
        var_uc_depmueback0_rv = 0.0;

        let assign9400_e4824: f64 = if var_uc_depmueback0 < 0.0 { 1.0 } else { 0.0 };
        var_guard190 = assign9400_e4824;
        var_guard190_rv = 0.0;

        let (assign9410_e4830, assign9410_e4830_d_n0, assign9410_e4830_d_n2, assign9410_e4830_d_n4, assign9410_e4830_d_n5, assign9410_e4830_d_n6, assign9410_e4830_d_n7, assign9410_e4830_d_n8, assign9410_e4830_d_n9, assign9410_e4830_d_n10, assign9410_e4830_d_n13,) = {
    if ((var_guard185 != 0.0) && (var_guard190 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmueback0, var_uc_depmueback0_dn0, var_uc_depmueback0_dn2, var_uc_depmueback0_dn4, var_uc_depmueback0_dn5, var_uc_depmueback0_dn6, var_uc_depmueback0_dn7, var_uc_depmueback0_dn8, var_uc_depmueback0_dn9, var_uc_depmueback0_dn10, var_uc_depmueback0_dn13,)
    }
};
        var_uc_depmueback0 = assign9410_e4830;
        var_uc_depmueback0_dn0 = assign9410_e4830_d_n0;
        var_uc_depmueback0_dn2 = assign9410_e4830_d_n2;
        var_uc_depmueback0_dn4 = assign9410_e4830_d_n4;
        var_uc_depmueback0_dn5 = assign9410_e4830_d_n5;
        var_uc_depmueback0_dn6 = assign9410_e4830_d_n6;
        var_uc_depmueback0_dn7 = assign9410_e4830_d_n7;
        var_uc_depmueback0_dn8 = assign9410_e4830_d_n8;
        var_uc_depmueback0_dn9 = assign9410_e4830_d_n9;
        var_uc_depmueback0_dn10 = assign9410_e4830_d_n10;
        var_uc_depmueback0_dn13 = assign9410_e4830_d_n13;
        var_uc_depmueback0_rv = 0.0;

        *var_guard185_slot = var_guard185;
        *var_guard185_rv_slot = var_guard185_rv;
        *var_guard186_slot = var_guard186;
        *var_guard186_rv_slot = var_guard186_rv;
        *var_guard187_slot = var_guard187;
        *var_guard187_rv_slot = var_guard187_rv;
        *var_guard188_slot = var_guard188;
        *var_guard188_rv_slot = var_guard188_rv;
        *var_guard189_slot = var_guard189;
        *var_guard189_rv_slot = var_guard189_rv;
        *var_guard190_slot = var_guard190;
        *var_guard190_rv_slot = var_guard190_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_uc_depleak_slot = var_uc_depleak;
        *var_uc_depleak_dn0_slot = var_uc_depleak_dn0;
        *var_uc_depleak_dn10_slot = var_uc_depleak_dn10;
        *var_uc_depleak_dn13_slot = var_uc_depleak_dn13;
        *var_uc_depleak_dn2_slot = var_uc_depleak_dn2;
        *var_uc_depleak_dn4_slot = var_uc_depleak_dn4;
        *var_uc_depleak_dn5_slot = var_uc_depleak_dn5;
        *var_uc_depleak_dn6_slot = var_uc_depleak_dn6;
        *var_uc_depleak_dn7_slot = var_uc_depleak_dn7;
        *var_uc_depleak_dn8_slot = var_uc_depleak_dn8;
        *var_uc_depleak_dn9_slot = var_uc_depleak_dn9;
        *var_uc_depleak_rv_slot = var_uc_depleak_rv;
        *var_uc_depmue0_slot = var_uc_depmue0;
        *var_uc_depmue0_dn0_slot = var_uc_depmue0_dn0;
        *var_uc_depmue0_dn10_slot = var_uc_depmue0_dn10;
        *var_uc_depmue0_dn13_slot = var_uc_depmue0_dn13;
        *var_uc_depmue0_dn2_slot = var_uc_depmue0_dn2;
        *var_uc_depmue0_dn4_slot = var_uc_depmue0_dn4;
        *var_uc_depmue0_dn5_slot = var_uc_depmue0_dn5;
        *var_uc_depmue0_dn6_slot = var_uc_depmue0_dn6;
        *var_uc_depmue0_dn7_slot = var_uc_depmue0_dn7;
        *var_uc_depmue0_dn8_slot = var_uc_depmue0_dn8;
        *var_uc_depmue0_dn9_slot = var_uc_depmue0_dn9;
        *var_uc_depmue0_rv_slot = var_uc_depmue0_rv;
        *var_uc_depmue1_slot = var_uc_depmue1;
        *var_uc_depmue1_dn0_slot = var_uc_depmue1_dn0;
        *var_uc_depmue1_dn10_slot = var_uc_depmue1_dn10;
        *var_uc_depmue1_dn13_slot = var_uc_depmue1_dn13;
        *var_uc_depmue1_dn2_slot = var_uc_depmue1_dn2;
        *var_uc_depmue1_dn4_slot = var_uc_depmue1_dn4;
        *var_uc_depmue1_dn5_slot = var_uc_depmue1_dn5;
        *var_uc_depmue1_dn6_slot = var_uc_depmue1_dn6;
        *var_uc_depmue1_dn7_slot = var_uc_depmue1_dn7;
        *var_uc_depmue1_dn8_slot = var_uc_depmue1_dn8;
        *var_uc_depmue1_dn9_slot = var_uc_depmue1_dn9;
        *var_uc_depmue1_rv_slot = var_uc_depmue1_rv;
        *var_uc_depmueback0_slot = var_uc_depmueback0;
        *var_uc_depmueback0_dn0_slot = var_uc_depmueback0_dn0;
        *var_uc_depmueback0_dn10_slot = var_uc_depmueback0_dn10;
        *var_uc_depmueback0_dn13_slot = var_uc_depmueback0_dn13;
        *var_uc_depmueback0_dn2_slot = var_uc_depmueback0_dn2;
        *var_uc_depmueback0_dn4_slot = var_uc_depmueback0_dn4;
        *var_uc_depmueback0_dn5_slot = var_uc_depmueback0_dn5;
        *var_uc_depmueback0_dn6_slot = var_uc_depmueback0_dn6;
        *var_uc_depmueback0_dn7_slot = var_uc_depmueback0_dn7;
        *var_uc_depmueback0_dn8_slot = var_uc_depmueback0_dn8;
        *var_uc_depmueback0_dn9_slot = var_uc_depmueback0_dn9;
        *var_uc_depmueback0_rv_slot = var_uc_depmueback0_rv;
        *var_uc_depvmax_slot = var_uc_depvmax;
        *var_uc_depvmax_dn0_slot = var_uc_depvmax_dn0;
        *var_uc_depvmax_dn10_slot = var_uc_depvmax_dn10;
        *var_uc_depvmax_dn13_slot = var_uc_depvmax_dn13;
        *var_uc_depvmax_dn2_slot = var_uc_depvmax_dn2;
        *var_uc_depvmax_dn4_slot = var_uc_depvmax_dn4;
        *var_uc_depvmax_dn5_slot = var_uc_depvmax_dn5;
        *var_uc_depvmax_dn6_slot = var_uc_depvmax_dn6;
        *var_uc_depvmax_dn7_slot = var_uc_depvmax_dn7;
        *var_uc_depvmax_dn8_slot = var_uc_depvmax_dn8;
        *var_uc_depvmax_dn9_slot = var_uc_depvmax_dn9;
        *var_uc_depvmax_rv_slot = var_uc_depvmax_rv;
        *var_uc_js0d_slot = var_uc_js0d;
        *var_uc_js0d_rv_slot = var_uc_js0d_rv;
        *var_uc_js0s_slot = var_uc_js0s;
        *var_uc_js0s_rv_slot = var_uc_js0s_rv;
        *var_uc_js0swd_slot = var_uc_js0swd;
        *var_uc_js0swd_rv_slot = var_uc_js0swd_rv;
        *var_uc_js0sws_slot = var_uc_js0sws;
        *var_uc_js0sws_rv_slot = var_uc_js0sws_rv;
        *var_uc_ndepm_slot = var_uc_ndepm;
        *var_uc_ndepm_dn0_slot = var_uc_ndepm_dn0;
        *var_uc_ndepm_dn10_slot = var_uc_ndepm_dn10;
        *var_uc_ndepm_dn13_slot = var_uc_ndepm_dn13;
        *var_uc_ndepm_dn2_slot = var_uc_ndepm_dn2;
        *var_uc_ndepm_dn4_slot = var_uc_ndepm_dn4;
        *var_uc_ndepm_dn5_slot = var_uc_ndepm_dn5;
        *var_uc_ndepm_dn6_slot = var_uc_ndepm_dn6;
        *var_uc_ndepm_dn7_slot = var_uc_ndepm_dn7;
        *var_uc_ndepm_dn8_slot = var_uc_ndepm_dn8;
        *var_uc_ndepm_dn9_slot = var_uc_ndepm_dn9;
        *var_uc_ndepm_rv_slot = var_uc_ndepm_rv;
        *var_uc_njd_slot = var_uc_njd;
        *var_uc_njd_rv_slot = var_uc_njd_rv;
        *var_uc_njs_slot = var_uc_njs;
        *var_uc_njs_rv_slot = var_uc_njs_rv;
        *var_uc_rdvd_slot = var_uc_rdvd;
        *var_uc_rdvd_rv_slot = var_uc_rdvd_rv;
        *var_uc_rdvg11_slot = var_uc_rdvg11;
        *var_uc_rdvg11_rv_slot = var_uc_rdvg11_rv;
        *var_uc_rs_slot = var_uc_rs;
        *var_uc_rs_rv_slot = var_uc_rs_rv;
        *var_uc_rth0_slot = var_uc_rth0;
        *var_uc_rth0_rv_slot = var_uc_rth0_rv;
        *var_uc_vdiffjd_slot = var_uc_vdiffjd;
        *var_uc_vdiffjd_rv_slot = var_uc_vdiffjd_rv;
        *var_uc_vdiffjs_slot = var_uc_vdiffjs;
        *var_uc_vdiffjs_rv_slot = var_uc_vdiffjs_rv;
        *var_uc_vover_slot = var_uc_vover;
        *var_uc_vover_rv_slot = var_uc_vover_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        var_guard185: f64,
        var_lg: f64,
        var_uc_nover: f64,
        var_uc_novers: f64,
        var_uc_rd: f64,
        var_uc_rdict1: f64,
        var_uc_rdslp1: f64,
        var_uc_rdvd: f64,
        var_uc_xldld: f64,
        var_flg_rd_slot: &mut f64,
        var_flg_rd_rv_slot: &mut f64,
        var_flg_rs_slot: &mut f64,
        var_flg_rs_rv_slot: &mut f64,
        var_guard191_slot: &mut f64,
        var_guard191_rv_slot: &mut f64,
        var_guard192_slot: &mut f64,
        var_guard192_rv_slot: &mut f64,
        var_guard244_slot: &mut f64,
        var_guard244_rv_slot: &mut f64,
        var_guard246_slot: &mut f64,
        var_guard246_rv_slot: &mut f64,
        var_guard247_slot: &mut f64,
        var_guard247_rv_slot: &mut f64,
        var_guard248_slot: &mut f64,
        var_guard248_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_uc_cordrift_slot: &mut f64,
        var_uc_cordrift_rv_slot: &mut f64,
        var_uc_depleak_slot: &mut f64,
        var_uc_depleak_dn0_slot: &mut f64,
        var_uc_depleak_dn10_slot: &mut f64,
        var_uc_depleak_dn13_slot: &mut f64,
        var_uc_depleak_dn2_slot: &mut f64,
        var_uc_depleak_dn4_slot: &mut f64,
        var_uc_depleak_dn5_slot: &mut f64,
        var_uc_depleak_dn6_slot: &mut f64,
        var_uc_depleak_dn7_slot: &mut f64,
        var_uc_depleak_dn8_slot: &mut f64,
        var_uc_depleak_dn9_slot: &mut f64,
        var_uc_depleak_rv_slot: &mut f64,
        var_uc_depmue0_slot: &mut f64,
        var_uc_depmue0_dn0_slot: &mut f64,
        var_uc_depmue0_dn10_slot: &mut f64,
        var_uc_depmue0_dn13_slot: &mut f64,
        var_uc_depmue0_dn2_slot: &mut f64,
        var_uc_depmue0_dn4_slot: &mut f64,
        var_uc_depmue0_dn5_slot: &mut f64,
        var_uc_depmue0_dn6_slot: &mut f64,
        var_uc_depmue0_dn7_slot: &mut f64,
        var_uc_depmue0_dn8_slot: &mut f64,
        var_uc_depmue0_dn9_slot: &mut f64,
        var_uc_depmue0_rv_slot: &mut f64,
        var_uc_depmue1_slot: &mut f64,
        var_uc_depmue1_dn0_slot: &mut f64,
        var_uc_depmue1_dn10_slot: &mut f64,
        var_uc_depmue1_dn13_slot: &mut f64,
        var_uc_depmue1_dn2_slot: &mut f64,
        var_uc_depmue1_dn4_slot: &mut f64,
        var_uc_depmue1_dn5_slot: &mut f64,
        var_uc_depmue1_dn6_slot: &mut f64,
        var_uc_depmue1_dn7_slot: &mut f64,
        var_uc_depmue1_dn8_slot: &mut f64,
        var_uc_depmue1_dn9_slot: &mut f64,
        var_uc_depmue1_rv_slot: &mut f64,
        var_uc_depmueback0_slot: &mut f64,
        var_uc_depmueback0_dn0_slot: &mut f64,
        var_uc_depmueback0_dn10_slot: &mut f64,
        var_uc_depmueback0_dn13_slot: &mut f64,
        var_uc_depmueback0_dn2_slot: &mut f64,
        var_uc_depmueback0_dn4_slot: &mut f64,
        var_uc_depmueback0_dn5_slot: &mut f64,
        var_uc_depmueback0_dn6_slot: &mut f64,
        var_uc_depmueback0_dn7_slot: &mut f64,
        var_uc_depmueback0_dn8_slot: &mut f64,
        var_uc_depmueback0_dn9_slot: &mut f64,
        var_uc_depmueback0_rv_slot: &mut f64,
        var_uc_depmueback1_slot: &mut f64,
        var_uc_depmueback1_dn0_slot: &mut f64,
        var_uc_depmueback1_dn10_slot: &mut f64,
        var_uc_depmueback1_dn13_slot: &mut f64,
        var_uc_depmueback1_dn2_slot: &mut f64,
        var_uc_depmueback1_dn4_slot: &mut f64,
        var_uc_depmueback1_dn5_slot: &mut f64,
        var_uc_depmueback1_dn6_slot: &mut f64,
        var_uc_depmueback1_dn7_slot: &mut f64,
        var_uc_depmueback1_dn8_slot: &mut f64,
        var_uc_depmueback1_dn9_slot: &mut f64,
        var_uc_depmueback1_rv_slot: &mut f64,
        var_uc_depvdsef1_slot: &mut f64,
        var_uc_depvdsef1_dn0_slot: &mut f64,
        var_uc_depvdsef1_dn10_slot: &mut f64,
        var_uc_depvdsef1_dn13_slot: &mut f64,
        var_uc_depvdsef1_dn2_slot: &mut f64,
        var_uc_depvdsef1_dn4_slot: &mut f64,
        var_uc_depvdsef1_dn5_slot: &mut f64,
        var_uc_depvdsef1_dn6_slot: &mut f64,
        var_uc_depvdsef1_dn7_slot: &mut f64,
        var_uc_depvdsef1_dn8_slot: &mut f64,
        var_uc_depvdsef1_dn9_slot: &mut f64,
        var_uc_depvdsef1_rv_slot: &mut f64,
        var_uc_depvdsef2_slot: &mut f64,
        var_uc_depvdsef2_dn0_slot: &mut f64,
        var_uc_depvdsef2_dn10_slot: &mut f64,
        var_uc_depvdsef2_dn13_slot: &mut f64,
        var_uc_depvdsef2_dn2_slot: &mut f64,
        var_uc_depvdsef2_dn4_slot: &mut f64,
        var_uc_depvdsef2_dn5_slot: &mut f64,
        var_uc_depvdsef2_dn6_slot: &mut f64,
        var_uc_depvdsef2_dn7_slot: &mut f64,
        var_uc_depvdsef2_dn8_slot: &mut f64,
        var_uc_depvdsef2_dn9_slot: &mut f64,
        var_uc_depvdsef2_rv_slot: &mut f64,
        var_uc_depvmax_slot: &mut f64,
        var_uc_depvmax_dn0_slot: &mut f64,
        var_uc_depvmax_dn10_slot: &mut f64,
        var_uc_depvmax_dn13_slot: &mut f64,
        var_uc_depvmax_dn2_slot: &mut f64,
        var_uc_depvmax_dn4_slot: &mut f64,
        var_uc_depvmax_dn5_slot: &mut f64,
        var_uc_depvmax_dn6_slot: &mut f64,
        var_uc_depvmax_dn7_slot: &mut f64,
        var_uc_depvmax_dn8_slot: &mut f64,
        var_uc_depvmax_dn9_slot: &mut f64,
        var_uc_depvmax_rv_slot: &mut f64,
        var_uc_ndepm_slot: &mut f64,
        var_uc_ndepm_dn0_slot: &mut f64,
        var_uc_ndepm_dn10_slot: &mut f64,
        var_uc_ndepm_dn13_slot: &mut f64,
        var_uc_ndepm_dn2_slot: &mut f64,
        var_uc_ndepm_dn4_slot: &mut f64,
        var_uc_ndepm_dn5_slot: &mut f64,
        var_uc_ndepm_dn6_slot: &mut f64,
        var_uc_ndepm_dn7_slot: &mut f64,
        var_uc_ndepm_dn8_slot: &mut f64,
        var_uc_ndepm_dn9_slot: &mut f64,
        var_uc_ndepm_rv_slot: &mut f64,
        var_uc_xpdv_slot: &mut f64,
        var_uc_xpdv_rv_slot: &mut f64,
    ) {
        let mut var_flg_rd: f64 = *var_flg_rd_slot;
        let mut var_flg_rd_rv: f64 = *var_flg_rd_rv_slot;
        let mut var_flg_rs: f64 = *var_flg_rs_slot;
        let mut var_flg_rs_rv: f64 = *var_flg_rs_rv_slot;
        let mut var_guard191: f64 = *var_guard191_slot;
        let mut var_guard191_rv: f64 = *var_guard191_rv_slot;
        let mut var_guard192: f64 = *var_guard192_slot;
        let mut var_guard192_rv: f64 = *var_guard192_rv_slot;
        let mut var_guard244: f64 = *var_guard244_slot;
        let mut var_guard244_rv: f64 = *var_guard244_rv_slot;
        let mut var_guard246: f64 = *var_guard246_slot;
        let mut var_guard246_rv: f64 = *var_guard246_rv_slot;
        let mut var_guard247: f64 = *var_guard247_slot;
        let mut var_guard247_rv: f64 = *var_guard247_rv_slot;
        let mut var_guard248: f64 = *var_guard248_slot;
        let mut var_guard248_rv: f64 = *var_guard248_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_uc_cordrift: f64 = *var_uc_cordrift_slot;
        let mut var_uc_cordrift_rv: f64 = *var_uc_cordrift_rv_slot;
        let mut var_uc_depleak: f64 = *var_uc_depleak_slot;
        let mut var_uc_depleak_dn0: f64 = *var_uc_depleak_dn0_slot;
        let mut var_uc_depleak_dn10: f64 = *var_uc_depleak_dn10_slot;
        let mut var_uc_depleak_dn13: f64 = *var_uc_depleak_dn13_slot;
        let mut var_uc_depleak_dn2: f64 = *var_uc_depleak_dn2_slot;
        let mut var_uc_depleak_dn4: f64 = *var_uc_depleak_dn4_slot;
        let mut var_uc_depleak_dn5: f64 = *var_uc_depleak_dn5_slot;
        let mut var_uc_depleak_dn6: f64 = *var_uc_depleak_dn6_slot;
        let mut var_uc_depleak_dn7: f64 = *var_uc_depleak_dn7_slot;
        let mut var_uc_depleak_dn8: f64 = *var_uc_depleak_dn8_slot;
        let mut var_uc_depleak_dn9: f64 = *var_uc_depleak_dn9_slot;
        let mut var_uc_depleak_rv: f64 = *var_uc_depleak_rv_slot;
        let mut var_uc_depmue0: f64 = *var_uc_depmue0_slot;
        let mut var_uc_depmue0_dn0: f64 = *var_uc_depmue0_dn0_slot;
        let mut var_uc_depmue0_dn10: f64 = *var_uc_depmue0_dn10_slot;
        let mut var_uc_depmue0_dn13: f64 = *var_uc_depmue0_dn13_slot;
        let mut var_uc_depmue0_dn2: f64 = *var_uc_depmue0_dn2_slot;
        let mut var_uc_depmue0_dn4: f64 = *var_uc_depmue0_dn4_slot;
        let mut var_uc_depmue0_dn5: f64 = *var_uc_depmue0_dn5_slot;
        let mut var_uc_depmue0_dn6: f64 = *var_uc_depmue0_dn6_slot;
        let mut var_uc_depmue0_dn7: f64 = *var_uc_depmue0_dn7_slot;
        let mut var_uc_depmue0_dn8: f64 = *var_uc_depmue0_dn8_slot;
        let mut var_uc_depmue0_dn9: f64 = *var_uc_depmue0_dn9_slot;
        let mut var_uc_depmue0_rv: f64 = *var_uc_depmue0_rv_slot;
        let mut var_uc_depmue1: f64 = *var_uc_depmue1_slot;
        let mut var_uc_depmue1_dn0: f64 = *var_uc_depmue1_dn0_slot;
        let mut var_uc_depmue1_dn10: f64 = *var_uc_depmue1_dn10_slot;
        let mut var_uc_depmue1_dn13: f64 = *var_uc_depmue1_dn13_slot;
        let mut var_uc_depmue1_dn2: f64 = *var_uc_depmue1_dn2_slot;
        let mut var_uc_depmue1_dn4: f64 = *var_uc_depmue1_dn4_slot;
        let mut var_uc_depmue1_dn5: f64 = *var_uc_depmue1_dn5_slot;
        let mut var_uc_depmue1_dn6: f64 = *var_uc_depmue1_dn6_slot;
        let mut var_uc_depmue1_dn7: f64 = *var_uc_depmue1_dn7_slot;
        let mut var_uc_depmue1_dn8: f64 = *var_uc_depmue1_dn8_slot;
        let mut var_uc_depmue1_dn9: f64 = *var_uc_depmue1_dn9_slot;
        let mut var_uc_depmue1_rv: f64 = *var_uc_depmue1_rv_slot;
        let mut var_uc_depmueback0: f64 = *var_uc_depmueback0_slot;
        let mut var_uc_depmueback0_dn0: f64 = *var_uc_depmueback0_dn0_slot;
        let mut var_uc_depmueback0_dn10: f64 = *var_uc_depmueback0_dn10_slot;
        let mut var_uc_depmueback0_dn13: f64 = *var_uc_depmueback0_dn13_slot;
        let mut var_uc_depmueback0_dn2: f64 = *var_uc_depmueback0_dn2_slot;
        let mut var_uc_depmueback0_dn4: f64 = *var_uc_depmueback0_dn4_slot;
        let mut var_uc_depmueback0_dn5: f64 = *var_uc_depmueback0_dn5_slot;
        let mut var_uc_depmueback0_dn6: f64 = *var_uc_depmueback0_dn6_slot;
        let mut var_uc_depmueback0_dn7: f64 = *var_uc_depmueback0_dn7_slot;
        let mut var_uc_depmueback0_dn8: f64 = *var_uc_depmueback0_dn8_slot;
        let mut var_uc_depmueback0_dn9: f64 = *var_uc_depmueback0_dn9_slot;
        let mut var_uc_depmueback0_rv: f64 = *var_uc_depmueback0_rv_slot;
        let mut var_uc_depmueback1: f64 = *var_uc_depmueback1_slot;
        let mut var_uc_depmueback1_dn0: f64 = *var_uc_depmueback1_dn0_slot;
        let mut var_uc_depmueback1_dn10: f64 = *var_uc_depmueback1_dn10_slot;
        let mut var_uc_depmueback1_dn13: f64 = *var_uc_depmueback1_dn13_slot;
        let mut var_uc_depmueback1_dn2: f64 = *var_uc_depmueback1_dn2_slot;
        let mut var_uc_depmueback1_dn4: f64 = *var_uc_depmueback1_dn4_slot;
        let mut var_uc_depmueback1_dn5: f64 = *var_uc_depmueback1_dn5_slot;
        let mut var_uc_depmueback1_dn6: f64 = *var_uc_depmueback1_dn6_slot;
        let mut var_uc_depmueback1_dn7: f64 = *var_uc_depmueback1_dn7_slot;
        let mut var_uc_depmueback1_dn8: f64 = *var_uc_depmueback1_dn8_slot;
        let mut var_uc_depmueback1_dn9: f64 = *var_uc_depmueback1_dn9_slot;
        let mut var_uc_depmueback1_rv: f64 = *var_uc_depmueback1_rv_slot;
        let mut var_uc_depvdsef1: f64 = *var_uc_depvdsef1_slot;
        let mut var_uc_depvdsef1_dn0: f64 = *var_uc_depvdsef1_dn0_slot;
        let mut var_uc_depvdsef1_dn10: f64 = *var_uc_depvdsef1_dn10_slot;
        let mut var_uc_depvdsef1_dn13: f64 = *var_uc_depvdsef1_dn13_slot;
        let mut var_uc_depvdsef1_dn2: f64 = *var_uc_depvdsef1_dn2_slot;
        let mut var_uc_depvdsef1_dn4: f64 = *var_uc_depvdsef1_dn4_slot;
        let mut var_uc_depvdsef1_dn5: f64 = *var_uc_depvdsef1_dn5_slot;
        let mut var_uc_depvdsef1_dn6: f64 = *var_uc_depvdsef1_dn6_slot;
        let mut var_uc_depvdsef1_dn7: f64 = *var_uc_depvdsef1_dn7_slot;
        let mut var_uc_depvdsef1_dn8: f64 = *var_uc_depvdsef1_dn8_slot;
        let mut var_uc_depvdsef1_dn9: f64 = *var_uc_depvdsef1_dn9_slot;
        let mut var_uc_depvdsef1_rv: f64 = *var_uc_depvdsef1_rv_slot;
        let mut var_uc_depvdsef2: f64 = *var_uc_depvdsef2_slot;
        let mut var_uc_depvdsef2_dn0: f64 = *var_uc_depvdsef2_dn0_slot;
        let mut var_uc_depvdsef2_dn10: f64 = *var_uc_depvdsef2_dn10_slot;
        let mut var_uc_depvdsef2_dn13: f64 = *var_uc_depvdsef2_dn13_slot;
        let mut var_uc_depvdsef2_dn2: f64 = *var_uc_depvdsef2_dn2_slot;
        let mut var_uc_depvdsef2_dn4: f64 = *var_uc_depvdsef2_dn4_slot;
        let mut var_uc_depvdsef2_dn5: f64 = *var_uc_depvdsef2_dn5_slot;
        let mut var_uc_depvdsef2_dn6: f64 = *var_uc_depvdsef2_dn6_slot;
        let mut var_uc_depvdsef2_dn7: f64 = *var_uc_depvdsef2_dn7_slot;
        let mut var_uc_depvdsef2_dn8: f64 = *var_uc_depvdsef2_dn8_slot;
        let mut var_uc_depvdsef2_dn9: f64 = *var_uc_depvdsef2_dn9_slot;
        let mut var_uc_depvdsef2_rv: f64 = *var_uc_depvdsef2_rv_slot;
        let mut var_uc_depvmax: f64 = *var_uc_depvmax_slot;
        let mut var_uc_depvmax_dn0: f64 = *var_uc_depvmax_dn0_slot;
        let mut var_uc_depvmax_dn10: f64 = *var_uc_depvmax_dn10_slot;
        let mut var_uc_depvmax_dn13: f64 = *var_uc_depvmax_dn13_slot;
        let mut var_uc_depvmax_dn2: f64 = *var_uc_depvmax_dn2_slot;
        let mut var_uc_depvmax_dn4: f64 = *var_uc_depvmax_dn4_slot;
        let mut var_uc_depvmax_dn5: f64 = *var_uc_depvmax_dn5_slot;
        let mut var_uc_depvmax_dn6: f64 = *var_uc_depvmax_dn6_slot;
        let mut var_uc_depvmax_dn7: f64 = *var_uc_depvmax_dn7_slot;
        let mut var_uc_depvmax_dn8: f64 = *var_uc_depvmax_dn8_slot;
        let mut var_uc_depvmax_dn9: f64 = *var_uc_depvmax_dn9_slot;
        let mut var_uc_depvmax_rv: f64 = *var_uc_depvmax_rv_slot;
        let mut var_uc_ndepm: f64 = *var_uc_ndepm_slot;
        let mut var_uc_ndepm_dn0: f64 = *var_uc_ndepm_dn0_slot;
        let mut var_uc_ndepm_dn10: f64 = *var_uc_ndepm_dn10_slot;
        let mut var_uc_ndepm_dn13: f64 = *var_uc_ndepm_dn13_slot;
        let mut var_uc_ndepm_dn2: f64 = *var_uc_ndepm_dn2_slot;
        let mut var_uc_ndepm_dn4: f64 = *var_uc_ndepm_dn4_slot;
        let mut var_uc_ndepm_dn5: f64 = *var_uc_ndepm_dn5_slot;
        let mut var_uc_ndepm_dn6: f64 = *var_uc_ndepm_dn6_slot;
        let mut var_uc_ndepm_dn7: f64 = *var_uc_ndepm_dn7_slot;
        let mut var_uc_ndepm_dn8: f64 = *var_uc_ndepm_dn8_slot;
        let mut var_uc_ndepm_dn9: f64 = *var_uc_ndepm_dn9_slot;
        let mut var_uc_ndepm_rv: f64 = *var_uc_ndepm_rv_slot;
        let mut var_uc_xpdv: f64 = *var_uc_xpdv_slot;
        let mut var_uc_xpdv_rv: f64 = *var_uc_xpdv_rv_slot;

        let (assign9420_e4836, assign9420_e4836_d_n0, assign9420_e4836_d_n2, assign9420_e4836_d_n4, assign9420_e4836_d_n5, assign9420_e4836_d_n6, assign9420_e4836_d_n7, assign9420_e4836_d_n8, assign9420_e4836_d_n9, assign9420_e4836_d_n10, assign9420_e4836_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9420_e4834: f64 = (var_lg).powf(p.p359);
        (assign9420_e4834, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign9420_e4836;
        var_t3_dn0 = assign9420_e4836_d_n0;
        var_t3_dn2 = assign9420_e4836_d_n2;
        var_t3_dn4 = assign9420_e4836_d_n4;
        var_t3_dn5 = assign9420_e4836_d_n5;
        var_t3_dn6 = assign9420_e4836_d_n6;
        var_t3_dn7 = assign9420_e4836_d_n7;
        var_t3_dn8 = assign9420_e4836_d_n8;
        var_t3_dn9 = assign9420_e4836_d_n9;
        var_t3_dn10 = assign9420_e4836_d_n10;
        var_t3_dn13 = assign9420_e4836_d_n13;
        var_t3_rv = 0.0;

        let (assign9430_e4846, assign9430_e4846_d_n0, assign9430_e4846_d_n2, assign9430_e4846_d_n4, assign9430_e4846_d_n5, assign9430_e4846_d_n6, assign9430_e4846_d_n7, assign9430_e4846_d_n8, assign9430_e4846_d_n9, assign9430_e4846_d_n10, assign9430_e4846_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9430_e4842: f64 = (p.p358 / var_t3);
        let assign9430_e4843: f64 = (1.0 + assign9430_e4842);
        let assign9430_e4844: f64 = (p.p355 * assign9430_e4843);
        (assign9430_e4844, (p.p355 * (-((p.p358 * var_t3_dn0) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn2) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn4) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn5) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn6) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn7) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn8) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn9) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn10) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn13) / (var_t3 * var_t3)))),)
    } else {
        (var_uc_depmueback1, var_uc_depmueback1_dn0, var_uc_depmueback1_dn2, var_uc_depmueback1_dn4, var_uc_depmueback1_dn5, var_uc_depmueback1_dn6, var_uc_depmueback1_dn7, var_uc_depmueback1_dn8, var_uc_depmueback1_dn9, var_uc_depmueback1_dn10, var_uc_depmueback1_dn13,)
    }
};
        var_uc_depmueback1 = assign9430_e4846;
        var_uc_depmueback1_dn0 = assign9430_e4846_d_n0;
        var_uc_depmueback1_dn2 = assign9430_e4846_d_n2;
        var_uc_depmueback1_dn4 = assign9430_e4846_d_n4;
        var_uc_depmueback1_dn5 = assign9430_e4846_d_n5;
        var_uc_depmueback1_dn6 = assign9430_e4846_d_n6;
        var_uc_depmueback1_dn7 = assign9430_e4846_d_n7;
        var_uc_depmueback1_dn8 = assign9430_e4846_d_n8;
        var_uc_depmueback1_dn9 = assign9430_e4846_d_n9;
        var_uc_depmueback1_dn10 = assign9430_e4846_d_n10;
        var_uc_depmueback1_dn13 = assign9430_e4846_d_n13;
        var_uc_depmueback1_rv = 0.0;

        let assign9440_e4849: f64 = if var_uc_depmueback1 < 0.0 { 1.0 } else { 0.0 };
        var_guard191 = assign9440_e4849;
        var_guard191_rv = 0.0;

        let (assign9450_e4855, assign9450_e4855_d_n0, assign9450_e4855_d_n2, assign9450_e4855_d_n4, assign9450_e4855_d_n5, assign9450_e4855_d_n6, assign9450_e4855_d_n7, assign9450_e4855_d_n8, assign9450_e4855_d_n9, assign9450_e4855_d_n10, assign9450_e4855_d_n13,) = {
    if ((var_guard185 != 0.0) && (var_guard191 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmueback1, var_uc_depmueback1_dn0, var_uc_depmueback1_dn2, var_uc_depmueback1_dn4, var_uc_depmueback1_dn5, var_uc_depmueback1_dn6, var_uc_depmueback1_dn7, var_uc_depmueback1_dn8, var_uc_depmueback1_dn9, var_uc_depmueback1_dn10, var_uc_depmueback1_dn13,)
    }
};
        var_uc_depmueback1 = assign9450_e4855;
        var_uc_depmueback1_dn0 = assign9450_e4855_d_n0;
        var_uc_depmueback1_dn2 = assign9450_e4855_d_n2;
        var_uc_depmueback1_dn4 = assign9450_e4855_d_n4;
        var_uc_depmueback1_dn5 = assign9450_e4855_d_n5;
        var_uc_depmueback1_dn6 = assign9450_e4855_d_n6;
        var_uc_depmueback1_dn7 = assign9450_e4855_d_n7;
        var_uc_depmueback1_dn8 = assign9450_e4855_d_n8;
        var_uc_depmueback1_dn9 = assign9450_e4855_d_n9;
        var_uc_depmueback1_dn10 = assign9450_e4855_d_n10;
        var_uc_depmueback1_dn13 = assign9450_e4855_d_n13;
        var_uc_depmueback1_rv = 0.0;

        let (assign9460_e4861, assign9460_e4861_d_n0, assign9460_e4861_d_n2, assign9460_e4861_d_n4, assign9460_e4861_d_n5, assign9460_e4861_d_n6, assign9460_e4861_d_n7, assign9460_e4861_d_n8, assign9460_e4861_d_n9, assign9460_e4861_d_n10, assign9460_e4861_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9460_e4859: f64 = (var_lg).powf(p.p373);
        (assign9460_e4859, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign9460_e4861;
        var_t3_dn0 = assign9460_e4861_d_n0;
        var_t3_dn2 = assign9460_e4861_d_n2;
        var_t3_dn4 = assign9460_e4861_d_n4;
        var_t3_dn5 = assign9460_e4861_d_n5;
        var_t3_dn6 = assign9460_e4861_d_n6;
        var_t3_dn7 = assign9460_e4861_d_n7;
        var_t3_dn8 = assign9460_e4861_d_n8;
        var_t3_dn9 = assign9460_e4861_d_n9;
        var_t3_dn10 = assign9460_e4861_d_n10;
        var_t3_dn13 = assign9460_e4861_d_n13;
        var_t3_rv = 0.0;

        let (assign9470_e4871, assign9470_e4871_d_n0, assign9470_e4871_d_n2, assign9470_e4871_d_n4, assign9470_e4871_d_n5, assign9470_e4871_d_n6, assign9470_e4871_d_n7, assign9470_e4871_d_n8, assign9470_e4871_d_n9, assign9470_e4871_d_n10, assign9470_e4871_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9470_e4867: f64 = (p.p372 / var_t3);
        let assign9470_e4868: f64 = (1.0 + assign9470_e4867);
        let assign9470_e4869: f64 = (var_uc_depvdsef1 * assign9470_e4868);
        (assign9470_e4869, ((var_uc_depvdsef1_dn0 * assign9470_e4868) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn0) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn2 * assign9470_e4868) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn2) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn4 * assign9470_e4868) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn4) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn5 * assign9470_e4868) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn5) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn6 * assign9470_e4868) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn6) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn7 * assign9470_e4868) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn7) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn8 * assign9470_e4868) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn8) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn9 * assign9470_e4868) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn9) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn10 * assign9470_e4868) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn10) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn13 * assign9470_e4868) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn13) / (var_t3 * var_t3))))),)
    } else {
        (var_uc_depvdsef1, var_uc_depvdsef1_dn0, var_uc_depvdsef1_dn2, var_uc_depvdsef1_dn4, var_uc_depvdsef1_dn5, var_uc_depvdsef1_dn6, var_uc_depvdsef1_dn7, var_uc_depvdsef1_dn8, var_uc_depvdsef1_dn9, var_uc_depvdsef1_dn10, var_uc_depvdsef1_dn13,)
    }
};
        var_uc_depvdsef1 = assign9470_e4871;
        var_uc_depvdsef1_dn0 = assign9470_e4871_d_n0;
        var_uc_depvdsef1_dn2 = assign9470_e4871_d_n2;
        var_uc_depvdsef1_dn4 = assign9470_e4871_d_n4;
        var_uc_depvdsef1_dn5 = assign9470_e4871_d_n5;
        var_uc_depvdsef1_dn6 = assign9470_e4871_d_n6;
        var_uc_depvdsef1_dn7 = assign9470_e4871_d_n7;
        var_uc_depvdsef1_dn8 = assign9470_e4871_d_n8;
        var_uc_depvdsef1_dn9 = assign9470_e4871_d_n9;
        var_uc_depvdsef1_dn10 = assign9470_e4871_d_n10;
        var_uc_depvdsef1_dn13 = assign9470_e4871_d_n13;
        var_uc_depvdsef1_rv = 0.0;

        let (assign9480_e4877, assign9480_e4877_d_n0, assign9480_e4877_d_n2, assign9480_e4877_d_n4, assign9480_e4877_d_n5, assign9480_e4877_d_n6, assign9480_e4877_d_n7, assign9480_e4877_d_n8, assign9480_e4877_d_n9, assign9480_e4877_d_n10, assign9480_e4877_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9480_e4875: f64 = (var_lg).powf(p.p375);
        (assign9480_e4875, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign9480_e4877;
        var_t3_dn0 = assign9480_e4877_d_n0;
        var_t3_dn2 = assign9480_e4877_d_n2;
        var_t3_dn4 = assign9480_e4877_d_n4;
        var_t3_dn5 = assign9480_e4877_d_n5;
        var_t3_dn6 = assign9480_e4877_d_n6;
        var_t3_dn7 = assign9480_e4877_d_n7;
        var_t3_dn8 = assign9480_e4877_d_n8;
        var_t3_dn9 = assign9480_e4877_d_n9;
        var_t3_dn10 = assign9480_e4877_d_n10;
        var_t3_dn13 = assign9480_e4877_d_n13;
        var_t3_rv = 0.0;

        let (assign9490_e4887, assign9490_e4887_d_n0, assign9490_e4887_d_n2, assign9490_e4887_d_n4, assign9490_e4887_d_n5, assign9490_e4887_d_n6, assign9490_e4887_d_n7, assign9490_e4887_d_n8, assign9490_e4887_d_n9, assign9490_e4887_d_n10, assign9490_e4887_d_n13,) = {
    if (var_guard185 != 0.0) {
        let assign9490_e4883: f64 = (p.p374 / var_t3);
        let assign9490_e4884: f64 = (1.0 + assign9490_e4883);
        let assign9490_e4885: f64 = (var_uc_depvdsef2 * assign9490_e4884);
        (assign9490_e4885, ((var_uc_depvdsef2_dn0 * assign9490_e4884) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn0) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn2 * assign9490_e4884) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn2) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn4 * assign9490_e4884) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn4) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn5 * assign9490_e4884) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn5) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn6 * assign9490_e4884) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn6) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn7 * assign9490_e4884) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn7) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn8 * assign9490_e4884) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn8) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn9 * assign9490_e4884) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn9) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn10 * assign9490_e4884) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn10) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn13 * assign9490_e4884) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn13) / (var_t3 * var_t3))))),)
    } else {
        (var_uc_depvdsef2, var_uc_depvdsef2_dn0, var_uc_depvdsef2_dn2, var_uc_depvdsef2_dn4, var_uc_depvdsef2_dn5, var_uc_depvdsef2_dn6, var_uc_depvdsef2_dn7, var_uc_depvdsef2_dn8, var_uc_depvdsef2_dn9, var_uc_depvdsef2_dn10, var_uc_depvdsef2_dn13,)
    }
};
        var_uc_depvdsef2 = assign9490_e4887;
        var_uc_depvdsef2_dn0 = assign9490_e4887_d_n0;
        var_uc_depvdsef2_dn2 = assign9490_e4887_d_n2;
        var_uc_depvdsef2_dn4 = assign9490_e4887_d_n4;
        var_uc_depvdsef2_dn5 = assign9490_e4887_d_n5;
        var_uc_depvdsef2_dn6 = assign9490_e4887_d_n6;
        var_uc_depvdsef2_dn7 = assign9490_e4887_d_n7;
        var_uc_depvdsef2_dn8 = assign9490_e4887_d_n8;
        var_uc_depvdsef2_dn9 = assign9490_e4887_d_n9;
        var_uc_depvdsef2_dn10 = assign9490_e4887_d_n10;
        var_uc_depvdsef2_dn13 = assign9490_e4887_d_n13;
        var_uc_depvdsef2_rv = 0.0;

        let assign9500_e4890: f64 = if var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        var_guard192 = assign9500_e4890;
        var_guard192_rv = 0.0;

        let (assign9510_e4896, assign9510_e4896_d_n0, assign9510_e4896_d_n2, assign9510_e4896_d_n4, assign9510_e4896_d_n5, assign9510_e4896_d_n6, assign9510_e4896_d_n7, assign9510_e4896_d_n8, assign9510_e4896_d_n9, assign9510_e4896_d_n10, assign9510_e4896_d_n13,) = {
    if ((var_guard185 != 0.0) && (var_guard192 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvdsef2, var_uc_depvdsef2_dn0, var_uc_depvdsef2_dn2, var_uc_depvdsef2_dn4, var_uc_depvdsef2_dn5, var_uc_depvdsef2_dn6, var_uc_depvdsef2_dn7, var_uc_depvdsef2_dn8, var_uc_depvdsef2_dn9, var_uc_depvdsef2_dn10, var_uc_depvdsef2_dn13,)
    }
};
        var_uc_depvdsef2 = assign9510_e4896;
        var_uc_depvdsef2_dn0 = assign9510_e4896_d_n0;
        var_uc_depvdsef2_dn2 = assign9510_e4896_d_n2;
        var_uc_depvdsef2_dn4 = assign9510_e4896_d_n4;
        var_uc_depvdsef2_dn5 = assign9510_e4896_d_n5;
        var_uc_depvdsef2_dn6 = assign9510_e4896_d_n6;
        var_uc_depvdsef2_dn7 = assign9510_e4896_d_n7;
        var_uc_depvdsef2_dn8 = assign9510_e4896_d_n8;
        var_uc_depvdsef2_dn9 = assign9510_e4896_d_n9;
        var_uc_depvdsef2_dn10 = assign9510_e4896_d_n10;
        var_uc_depvdsef2_dn13 = assign9510_e4896_d_n13;
        var_uc_depvdsef2_rv = 0.0;

        let (assign9520_e4901, assign9520_e4901_d_n0, assign9520_e4901_d_n2, assign9520_e4901_d_n4, assign9520_e4901_d_n5, assign9520_e4901_d_n6, assign9520_e4901_d_n7, assign9520_e4901_d_n8, assign9520_e4901_d_n9, assign9520_e4901_d_n10, assign9520_e4901_d_n13,) = {
    if (var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn13,)
    }
};
        var_uc_ndepm = assign9520_e4901;
        var_uc_ndepm_dn0 = assign9520_e4901_d_n0;
        var_uc_ndepm_dn2 = assign9520_e4901_d_n2;
        var_uc_ndepm_dn4 = assign9520_e4901_d_n4;
        var_uc_ndepm_dn5 = assign9520_e4901_d_n5;
        var_uc_ndepm_dn6 = assign9520_e4901_d_n6;
        var_uc_ndepm_dn7 = assign9520_e4901_d_n7;
        var_uc_ndepm_dn8 = assign9520_e4901_d_n8;
        var_uc_ndepm_dn9 = assign9520_e4901_d_n9;
        var_uc_ndepm_dn10 = assign9520_e4901_d_n10;
        var_uc_ndepm_dn13 = assign9520_e4901_d_n13;
        var_uc_ndepm_rv = 0.0;

        let (assign9530_e4906, assign9530_e4906_d_n0, assign9530_e4906_d_n2, assign9530_e4906_d_n4, assign9530_e4906_d_n5, assign9530_e4906_d_n6, assign9530_e4906_d_n7, assign9530_e4906_d_n8, assign9530_e4906_d_n9, assign9530_e4906_d_n10, assign9530_e4906_d_n13,) = {
    if (var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvmax, var_uc_depvmax_dn0, var_uc_depvmax_dn2, var_uc_depvmax_dn4, var_uc_depvmax_dn5, var_uc_depvmax_dn6, var_uc_depvmax_dn7, var_uc_depvmax_dn8, var_uc_depvmax_dn9, var_uc_depvmax_dn10, var_uc_depvmax_dn13,)
    }
};
        var_uc_depvmax = assign9530_e4906;
        var_uc_depvmax_dn0 = assign9530_e4906_d_n0;
        var_uc_depvmax_dn2 = assign9530_e4906_d_n2;
        var_uc_depvmax_dn4 = assign9530_e4906_d_n4;
        var_uc_depvmax_dn5 = assign9530_e4906_d_n5;
        var_uc_depvmax_dn6 = assign9530_e4906_d_n6;
        var_uc_depvmax_dn7 = assign9530_e4906_d_n7;
        var_uc_depvmax_dn8 = assign9530_e4906_d_n8;
        var_uc_depvmax_dn9 = assign9530_e4906_d_n9;
        var_uc_depvmax_dn10 = assign9530_e4906_d_n10;
        var_uc_depvmax_dn13 = assign9530_e4906_d_n13;
        var_uc_depvmax_rv = 0.0;

        let (assign9540_e4911, assign9540_e4911_d_n0, assign9540_e4911_d_n2, assign9540_e4911_d_n4, assign9540_e4911_d_n5, assign9540_e4911_d_n6, assign9540_e4911_d_n7, assign9540_e4911_d_n8, assign9540_e4911_d_n9, assign9540_e4911_d_n10, assign9540_e4911_d_n13,) = {
    if (var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn13,)
    }
};
        var_uc_depleak = assign9540_e4911;
        var_uc_depleak_dn0 = assign9540_e4911_d_n0;
        var_uc_depleak_dn2 = assign9540_e4911_d_n2;
        var_uc_depleak_dn4 = assign9540_e4911_d_n4;
        var_uc_depleak_dn5 = assign9540_e4911_d_n5;
        var_uc_depleak_dn6 = assign9540_e4911_d_n6;
        var_uc_depleak_dn7 = assign9540_e4911_d_n7;
        var_uc_depleak_dn8 = assign9540_e4911_d_n8;
        var_uc_depleak_dn9 = assign9540_e4911_d_n9;
        var_uc_depleak_dn10 = assign9540_e4911_d_n10;
        var_uc_depleak_dn13 = assign9540_e4911_d_n13;
        var_uc_depleak_rv = 0.0;

        let (assign9550_e4916, assign9550_e4916_d_n0, assign9550_e4916_d_n2, assign9550_e4916_d_n4, assign9550_e4916_d_n5, assign9550_e4916_d_n6, assign9550_e4916_d_n7, assign9550_e4916_d_n8, assign9550_e4916_d_n9, assign9550_e4916_d_n10, assign9550_e4916_d_n13,) = {
    if (var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn13,)
    }
};
        var_uc_depmue0 = assign9550_e4916;
        var_uc_depmue0_dn0 = assign9550_e4916_d_n0;
        var_uc_depmue0_dn2 = assign9550_e4916_d_n2;
        var_uc_depmue0_dn4 = assign9550_e4916_d_n4;
        var_uc_depmue0_dn5 = assign9550_e4916_d_n5;
        var_uc_depmue0_dn6 = assign9550_e4916_d_n6;
        var_uc_depmue0_dn7 = assign9550_e4916_d_n7;
        var_uc_depmue0_dn8 = assign9550_e4916_d_n8;
        var_uc_depmue0_dn9 = assign9550_e4916_d_n9;
        var_uc_depmue0_dn10 = assign9550_e4916_d_n10;
        var_uc_depmue0_dn13 = assign9550_e4916_d_n13;
        var_uc_depmue0_rv = 0.0;

        let (assign9560_e4921, assign9560_e4921_d_n0, assign9560_e4921_d_n2, assign9560_e4921_d_n4, assign9560_e4921_d_n5, assign9560_e4921_d_n6, assign9560_e4921_d_n7, assign9560_e4921_d_n8, assign9560_e4921_d_n9, assign9560_e4921_d_n10, assign9560_e4921_d_n13,) = {
    if (var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue1, var_uc_depmue1_dn0, var_uc_depmue1_dn2, var_uc_depmue1_dn4, var_uc_depmue1_dn5, var_uc_depmue1_dn6, var_uc_depmue1_dn7, var_uc_depmue1_dn8, var_uc_depmue1_dn9, var_uc_depmue1_dn10, var_uc_depmue1_dn13,)
    }
};
        var_uc_depmue1 = assign9560_e4921;
        var_uc_depmue1_dn0 = assign9560_e4921_d_n0;
        var_uc_depmue1_dn2 = assign9560_e4921_d_n2;
        var_uc_depmue1_dn4 = assign9560_e4921_d_n4;
        var_uc_depmue1_dn5 = assign9560_e4921_d_n5;
        var_uc_depmue1_dn6 = assign9560_e4921_d_n6;
        var_uc_depmue1_dn7 = assign9560_e4921_d_n7;
        var_uc_depmue1_dn8 = assign9560_e4921_d_n8;
        var_uc_depmue1_dn9 = assign9560_e4921_d_n9;
        var_uc_depmue1_dn10 = assign9560_e4921_d_n10;
        var_uc_depmue1_dn13 = assign9560_e4921_d_n13;
        var_uc_depmue1_rv = 0.0;

        let (assign9570_e4926, assign9570_e4926_d_n0, assign9570_e4926_d_n2, assign9570_e4926_d_n4, assign9570_e4926_d_n5, assign9570_e4926_d_n6, assign9570_e4926_d_n7, assign9570_e4926_d_n8, assign9570_e4926_d_n9, assign9570_e4926_d_n10, assign9570_e4926_d_n13,) = {
    if (var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmueback0, var_uc_depmueback0_dn0, var_uc_depmueback0_dn2, var_uc_depmueback0_dn4, var_uc_depmueback0_dn5, var_uc_depmueback0_dn6, var_uc_depmueback0_dn7, var_uc_depmueback0_dn8, var_uc_depmueback0_dn9, var_uc_depmueback0_dn10, var_uc_depmueback0_dn13,)
    }
};
        var_uc_depmueback0 = assign9570_e4926;
        var_uc_depmueback0_dn0 = assign9570_e4926_d_n0;
        var_uc_depmueback0_dn2 = assign9570_e4926_d_n2;
        var_uc_depmueback0_dn4 = assign9570_e4926_d_n4;
        var_uc_depmueback0_dn5 = assign9570_e4926_d_n5;
        var_uc_depmueback0_dn6 = assign9570_e4926_d_n6;
        var_uc_depmueback0_dn7 = assign9570_e4926_d_n7;
        var_uc_depmueback0_dn8 = assign9570_e4926_d_n8;
        var_uc_depmueback0_dn9 = assign9570_e4926_d_n9;
        var_uc_depmueback0_dn10 = assign9570_e4926_d_n10;
        var_uc_depmueback0_dn13 = assign9570_e4926_d_n13;
        var_uc_depmueback0_rv = 0.0;

        let (assign9580_e4931, assign9580_e4931_d_n0, assign9580_e4931_d_n2, assign9580_e4931_d_n4, assign9580_e4931_d_n5, assign9580_e4931_d_n6, assign9580_e4931_d_n7, assign9580_e4931_d_n8, assign9580_e4931_d_n9, assign9580_e4931_d_n10, assign9580_e4931_d_n13,) = {
    if (var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmueback1, var_uc_depmueback1_dn0, var_uc_depmueback1_dn2, var_uc_depmueback1_dn4, var_uc_depmueback1_dn5, var_uc_depmueback1_dn6, var_uc_depmueback1_dn7, var_uc_depmueback1_dn8, var_uc_depmueback1_dn9, var_uc_depmueback1_dn10, var_uc_depmueback1_dn13,)
    }
};
        var_uc_depmueback1 = assign9580_e4931;
        var_uc_depmueback1_dn0 = assign9580_e4931_d_n0;
        var_uc_depmueback1_dn2 = assign9580_e4931_d_n2;
        var_uc_depmueback1_dn4 = assign9580_e4931_d_n4;
        var_uc_depmueback1_dn5 = assign9580_e4931_d_n5;
        var_uc_depmueback1_dn6 = assign9580_e4931_d_n6;
        var_uc_depmueback1_dn7 = assign9580_e4931_d_n7;
        var_uc_depmueback1_dn8 = assign9580_e4931_d_n8;
        var_uc_depmueback1_dn9 = assign9580_e4931_d_n9;
        var_uc_depmueback1_dn10 = assign9580_e4931_d_n10;
        var_uc_depmueback1_dn13 = assign9580_e4931_d_n13;
        var_uc_depmueback1_rv = 0.0;

        let (assign9590_e4936, assign9590_e4936_d_n0, assign9590_e4936_d_n2, assign9590_e4936_d_n4, assign9590_e4936_d_n5, assign9590_e4936_d_n6, assign9590_e4936_d_n7, assign9590_e4936_d_n8, assign9590_e4936_d_n9, assign9590_e4936_d_n10, assign9590_e4936_d_n13,) = {
    if (var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvdsef1, var_uc_depvdsef1_dn0, var_uc_depvdsef1_dn2, var_uc_depvdsef1_dn4, var_uc_depvdsef1_dn5, var_uc_depvdsef1_dn6, var_uc_depvdsef1_dn7, var_uc_depvdsef1_dn8, var_uc_depvdsef1_dn9, var_uc_depvdsef1_dn10, var_uc_depvdsef1_dn13,)
    }
};
        var_uc_depvdsef1 = assign9590_e4936;
        var_uc_depvdsef1_dn0 = assign9590_e4936_d_n0;
        var_uc_depvdsef1_dn2 = assign9590_e4936_d_n2;
        var_uc_depvdsef1_dn4 = assign9590_e4936_d_n4;
        var_uc_depvdsef1_dn5 = assign9590_e4936_d_n5;
        var_uc_depvdsef1_dn6 = assign9590_e4936_d_n6;
        var_uc_depvdsef1_dn7 = assign9590_e4936_d_n7;
        var_uc_depvdsef1_dn8 = assign9590_e4936_d_n8;
        var_uc_depvdsef1_dn9 = assign9590_e4936_d_n9;
        var_uc_depvdsef1_dn10 = assign9590_e4936_d_n10;
        var_uc_depvdsef1_dn13 = assign9590_e4936_d_n13;
        var_uc_depvdsef1_rv = 0.0;

        let (assign9600_e4941, assign9600_e4941_d_n0, assign9600_e4941_d_n2, assign9600_e4941_d_n4, assign9600_e4941_d_n5, assign9600_e4941_d_n6, assign9600_e4941_d_n7, assign9600_e4941_d_n8, assign9600_e4941_d_n9, assign9600_e4941_d_n10, assign9600_e4941_d_n13,) = {
    if (var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvdsef2, var_uc_depvdsef2_dn0, var_uc_depvdsef2_dn2, var_uc_depvdsef2_dn4, var_uc_depvdsef2_dn5, var_uc_depvdsef2_dn6, var_uc_depvdsef2_dn7, var_uc_depvdsef2_dn8, var_uc_depvdsef2_dn9, var_uc_depvdsef2_dn10, var_uc_depvdsef2_dn13,)
    }
};
        var_uc_depvdsef2 = assign9600_e4941;
        var_uc_depvdsef2_dn0 = assign9600_e4941_d_n0;
        var_uc_depvdsef2_dn2 = assign9600_e4941_d_n2;
        var_uc_depvdsef2_dn4 = assign9600_e4941_d_n4;
        var_uc_depvdsef2_dn5 = assign9600_e4941_d_n5;
        var_uc_depvdsef2_dn6 = assign9600_e4941_d_n6;
        var_uc_depvdsef2_dn7 = assign9600_e4941_d_n7;
        var_uc_depvdsef2_dn8 = assign9600_e4941_d_n8;
        var_uc_depvdsef2_dn9 = assign9600_e4941_d_n9;
        var_uc_depvdsef2_dn10 = assign9600_e4941_d_n10;
        var_uc_depvdsef2_dn13 = assign9600_e4941_d_n13;
        var_uc_depvdsef2_rv = 0.0;

        let assign10120_e5314: f64 = (var_uc_xpdv * var_uc_xldld);
        let assign10120_e5316: f64 = if assign10120_e5314 > 1.0 { 1.0 } else { 0.0 };
        var_guard244 = assign10120_e5316;
        var_guard244_rv = 0.0;

        let (assign10130_e5322,) = {
    if (var_guard244 != 0.0) {
        let assign10130_e5320: f64 = (1.0 / var_uc_xldld);
        (assign10130_e5320,)
    } else {
        (var_uc_xpdv,)
    }
};
        var_uc_xpdv = assign10130_e5322;
        var_uc_xpdv_rv = 0.0;

        let assign10150_e5350: f64 = if ((p.p40 == 1.0) && (((p.p19 > 0.0) && (var_uc_nover == 0.0)) || ((p.p18 > 0.0) && (var_uc_novers == 0.0)))) { 1.0 } else { 0.0 };
        var_guard246 = assign10150_e5350;
        var_guard246_rv = 0.0;

        let (assign10160_e5354,) = {
    if (var_guard246 != 0.0) {
        (0.0,)
    } else {
        (var_uc_cordrift,)
    }
};
        var_uc_cordrift = assign10160_e5354;
        var_uc_cordrift_rv = 0.0;

        let (assign10170_e5359,) = {
    if (var_guard246 == 0.0) {
        (p.p40,)
    } else {
        (var_uc_cordrift,)
    }
};
        var_uc_cordrift = assign10170_e5359;
        var_uc_cordrift_rv = 0.0;

        let assign10180_e5362: f64 = if var_uc_cordrift == 1.0 { 1.0 } else { 0.0 };
        var_guard247 = assign10180_e5362;
        var_guard247_rv = 0.0;

        let (assign10190_e5371,) = {
    if (var_guard247 != 0.0) {
        let (assign10190_e5369,) = {
            if (p.p19 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10190_e5369,)
    } else {
        (var_flg_rd,)
    }
};
        var_flg_rd = assign10190_e5371;
        var_flg_rd_rv = 0.0;

        let (assign10200_e5380,) = {
    if (var_guard247 != 0.0) {
        let (assign10200_e5378,) = {
            if (p.p18 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10200_e5378,)
    } else {
        (var_flg_rs,)
    }
};
        var_flg_rs = assign10200_e5380;
        var_flg_rs_rv = 0.0;

        let assign10210_e5387: f64 = if ((p.p17 == 0.0) || (p.p17 == 2.0)) { 1.0 } else { 0.0 };
        var_guard248 = assign10210_e5387;
        var_guard248_rv = 0.0;

        let (assign10220_e5394,) = {
    if ((var_guard247 == 0.0) && (var_guard248 != 0.0)) {
        (0.0,)
    } else {
        (var_flg_rd,)
    }
};
        var_flg_rd = assign10220_e5394;
        var_flg_rd_rv = 0.0;

        let (assign10230_e5401,) = {
    if ((var_guard247 == 0.0) && (var_guard248 != 0.0)) {
        (0.0,)
    } else {
        (var_flg_rs,)
    }
};
        var_flg_rs = assign10230_e5401;
        var_flg_rs_rv = 0.0;

        let (assign10240_e5433, assign10240_e5433_d_n0, assign10240_e5433_d_n2, assign10240_e5433_d_n4, assign10240_e5433_d_n5, assign10240_e5433_d_n6, assign10240_e5433_d_n7, assign10240_e5433_d_n8, assign10240_e5433_d_n9, assign10240_e5433_d_n10, assign10240_e5433_d_n13,) = {
    if ((var_guard247 == 0.0) && (var_guard248 == 0.0)) {
        let assign10240_e5409: f64 = (p.p130 * p.p2);
        let assign10240_e5411: f64 = (assign10240_e5409 * p.p7);
        let assign10240_e5414: f64 = (var_uc_rd + var_uc_rdvd);
        let assign10240_e5417: f64 = (p.p67 * var_uc_rdslp1);
        let assign10240_e5419: f64 = (assign10240_e5417 * 1000000.0);
        let assign10240_e5421: f64 = (assign10240_e5419 + var_uc_rdict1);
        let assign10240_e5422: f64 = (assign10240_e5414 * assign10240_e5421);
        let assign10240_e5425: f64 = (p.p68 * p.p100);
        let assign10240_e5427: f64 = (assign10240_e5425 * 1000000.0);
        let assign10240_e5429: f64 = (assign10240_e5427 + p.p101);
        let assign10240_e5430: f64 = (assign10240_e5422 * assign10240_e5429);
        let assign10240_e5431: f64 = (assign10240_e5411 + assign10240_e5430);
        (assign10240_e5431, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign10240_e5433;
        var_t1_dn0 = assign10240_e5433_d_n0;
        var_t1_dn2 = assign10240_e5433_d_n2;
        var_t1_dn4 = assign10240_e5433_d_n4;
        var_t1_dn5 = assign10240_e5433_d_n5;
        var_t1_dn6 = assign10240_e5433_d_n6;
        var_t1_dn7 = assign10240_e5433_d_n7;
        var_t1_dn8 = assign10240_e5433_d_n8;
        var_t1_dn9 = assign10240_e5433_d_n9;
        var_t1_dn10 = assign10240_e5433_d_n10;
        var_t1_dn13 = assign10240_e5433_d_n13;
        var_t1_rv = 0.0;

        let (assign10250_e5446,) = {
    if ((var_guard247 == 0.0) && (var_guard248 == 0.0)) {
        let (assign10250_e5444,) = {
            if (var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10250_e5444,)
    } else {
        (var_flg_rd,)
    }
};
        var_flg_rd = assign10250_e5446;
        var_flg_rd_rv = 0.0;

        *var_flg_rd_slot = var_flg_rd;
        *var_flg_rd_rv_slot = var_flg_rd_rv;
        *var_flg_rs_slot = var_flg_rs;
        *var_flg_rs_rv_slot = var_flg_rs_rv;
        *var_guard191_slot = var_guard191;
        *var_guard191_rv_slot = var_guard191_rv;
        *var_guard192_slot = var_guard192;
        *var_guard192_rv_slot = var_guard192_rv;
        *var_guard244_slot = var_guard244;
        *var_guard244_rv_slot = var_guard244_rv;
        *var_guard246_slot = var_guard246;
        *var_guard246_rv_slot = var_guard246_rv;
        *var_guard247_slot = var_guard247;
        *var_guard247_rv_slot = var_guard247_rv;
        *var_guard248_slot = var_guard248;
        *var_guard248_rv_slot = var_guard248_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_uc_cordrift_slot = var_uc_cordrift;
        *var_uc_cordrift_rv_slot = var_uc_cordrift_rv;
        *var_uc_depleak_slot = var_uc_depleak;
        *var_uc_depleak_dn0_slot = var_uc_depleak_dn0;
        *var_uc_depleak_dn10_slot = var_uc_depleak_dn10;
        *var_uc_depleak_dn13_slot = var_uc_depleak_dn13;
        *var_uc_depleak_dn2_slot = var_uc_depleak_dn2;
        *var_uc_depleak_dn4_slot = var_uc_depleak_dn4;
        *var_uc_depleak_dn5_slot = var_uc_depleak_dn5;
        *var_uc_depleak_dn6_slot = var_uc_depleak_dn6;
        *var_uc_depleak_dn7_slot = var_uc_depleak_dn7;
        *var_uc_depleak_dn8_slot = var_uc_depleak_dn8;
        *var_uc_depleak_dn9_slot = var_uc_depleak_dn9;
        *var_uc_depleak_rv_slot = var_uc_depleak_rv;
        *var_uc_depmue0_slot = var_uc_depmue0;
        *var_uc_depmue0_dn0_slot = var_uc_depmue0_dn0;
        *var_uc_depmue0_dn10_slot = var_uc_depmue0_dn10;
        *var_uc_depmue0_dn13_slot = var_uc_depmue0_dn13;
        *var_uc_depmue0_dn2_slot = var_uc_depmue0_dn2;
        *var_uc_depmue0_dn4_slot = var_uc_depmue0_dn4;
        *var_uc_depmue0_dn5_slot = var_uc_depmue0_dn5;
        *var_uc_depmue0_dn6_slot = var_uc_depmue0_dn6;
        *var_uc_depmue0_dn7_slot = var_uc_depmue0_dn7;
        *var_uc_depmue0_dn8_slot = var_uc_depmue0_dn8;
        *var_uc_depmue0_dn9_slot = var_uc_depmue0_dn9;
        *var_uc_depmue0_rv_slot = var_uc_depmue0_rv;
        *var_uc_depmue1_slot = var_uc_depmue1;
        *var_uc_depmue1_dn0_slot = var_uc_depmue1_dn0;
        *var_uc_depmue1_dn10_slot = var_uc_depmue1_dn10;
        *var_uc_depmue1_dn13_slot = var_uc_depmue1_dn13;
        *var_uc_depmue1_dn2_slot = var_uc_depmue1_dn2;
        *var_uc_depmue1_dn4_slot = var_uc_depmue1_dn4;
        *var_uc_depmue1_dn5_slot = var_uc_depmue1_dn5;
        *var_uc_depmue1_dn6_slot = var_uc_depmue1_dn6;
        *var_uc_depmue1_dn7_slot = var_uc_depmue1_dn7;
        *var_uc_depmue1_dn8_slot = var_uc_depmue1_dn8;
        *var_uc_depmue1_dn9_slot = var_uc_depmue1_dn9;
        *var_uc_depmue1_rv_slot = var_uc_depmue1_rv;
        *var_uc_depmueback0_slot = var_uc_depmueback0;
        *var_uc_depmueback0_dn0_slot = var_uc_depmueback0_dn0;
        *var_uc_depmueback0_dn10_slot = var_uc_depmueback0_dn10;
        *var_uc_depmueback0_dn13_slot = var_uc_depmueback0_dn13;
        *var_uc_depmueback0_dn2_slot = var_uc_depmueback0_dn2;
        *var_uc_depmueback0_dn4_slot = var_uc_depmueback0_dn4;
        *var_uc_depmueback0_dn5_slot = var_uc_depmueback0_dn5;
        *var_uc_depmueback0_dn6_slot = var_uc_depmueback0_dn6;
        *var_uc_depmueback0_dn7_slot = var_uc_depmueback0_dn7;
        *var_uc_depmueback0_dn8_slot = var_uc_depmueback0_dn8;
        *var_uc_depmueback0_dn9_slot = var_uc_depmueback0_dn9;
        *var_uc_depmueback0_rv_slot = var_uc_depmueback0_rv;
        *var_uc_depmueback1_slot = var_uc_depmueback1;
        *var_uc_depmueback1_dn0_slot = var_uc_depmueback1_dn0;
        *var_uc_depmueback1_dn10_slot = var_uc_depmueback1_dn10;
        *var_uc_depmueback1_dn13_slot = var_uc_depmueback1_dn13;
        *var_uc_depmueback1_dn2_slot = var_uc_depmueback1_dn2;
        *var_uc_depmueback1_dn4_slot = var_uc_depmueback1_dn4;
        *var_uc_depmueback1_dn5_slot = var_uc_depmueback1_dn5;
        *var_uc_depmueback1_dn6_slot = var_uc_depmueback1_dn6;
        *var_uc_depmueback1_dn7_slot = var_uc_depmueback1_dn7;
        *var_uc_depmueback1_dn8_slot = var_uc_depmueback1_dn8;
        *var_uc_depmueback1_dn9_slot = var_uc_depmueback1_dn9;
        *var_uc_depmueback1_rv_slot = var_uc_depmueback1_rv;
        *var_uc_depvdsef1_slot = var_uc_depvdsef1;
        *var_uc_depvdsef1_dn0_slot = var_uc_depvdsef1_dn0;
        *var_uc_depvdsef1_dn10_slot = var_uc_depvdsef1_dn10;
        *var_uc_depvdsef1_dn13_slot = var_uc_depvdsef1_dn13;
        *var_uc_depvdsef1_dn2_slot = var_uc_depvdsef1_dn2;
        *var_uc_depvdsef1_dn4_slot = var_uc_depvdsef1_dn4;
        *var_uc_depvdsef1_dn5_slot = var_uc_depvdsef1_dn5;
        *var_uc_depvdsef1_dn6_slot = var_uc_depvdsef1_dn6;
        *var_uc_depvdsef1_dn7_slot = var_uc_depvdsef1_dn7;
        *var_uc_depvdsef1_dn8_slot = var_uc_depvdsef1_dn8;
        *var_uc_depvdsef1_dn9_slot = var_uc_depvdsef1_dn9;
        *var_uc_depvdsef1_rv_slot = var_uc_depvdsef1_rv;
        *var_uc_depvdsef2_slot = var_uc_depvdsef2;
        *var_uc_depvdsef2_dn0_slot = var_uc_depvdsef2_dn0;
        *var_uc_depvdsef2_dn10_slot = var_uc_depvdsef2_dn10;
        *var_uc_depvdsef2_dn13_slot = var_uc_depvdsef2_dn13;
        *var_uc_depvdsef2_dn2_slot = var_uc_depvdsef2_dn2;
        *var_uc_depvdsef2_dn4_slot = var_uc_depvdsef2_dn4;
        *var_uc_depvdsef2_dn5_slot = var_uc_depvdsef2_dn5;
        *var_uc_depvdsef2_dn6_slot = var_uc_depvdsef2_dn6;
        *var_uc_depvdsef2_dn7_slot = var_uc_depvdsef2_dn7;
        *var_uc_depvdsef2_dn8_slot = var_uc_depvdsef2_dn8;
        *var_uc_depvdsef2_dn9_slot = var_uc_depvdsef2_dn9;
        *var_uc_depvdsef2_rv_slot = var_uc_depvdsef2_rv;
        *var_uc_depvmax_slot = var_uc_depvmax;
        *var_uc_depvmax_dn0_slot = var_uc_depvmax_dn0;
        *var_uc_depvmax_dn10_slot = var_uc_depvmax_dn10;
        *var_uc_depvmax_dn13_slot = var_uc_depvmax_dn13;
        *var_uc_depvmax_dn2_slot = var_uc_depvmax_dn2;
        *var_uc_depvmax_dn4_slot = var_uc_depvmax_dn4;
        *var_uc_depvmax_dn5_slot = var_uc_depvmax_dn5;
        *var_uc_depvmax_dn6_slot = var_uc_depvmax_dn6;
        *var_uc_depvmax_dn7_slot = var_uc_depvmax_dn7;
        *var_uc_depvmax_dn8_slot = var_uc_depvmax_dn8;
        *var_uc_depvmax_dn9_slot = var_uc_depvmax_dn9;
        *var_uc_depvmax_rv_slot = var_uc_depvmax_rv;
        *var_uc_ndepm_slot = var_uc_ndepm;
        *var_uc_ndepm_dn0_slot = var_uc_ndepm_dn0;
        *var_uc_ndepm_dn10_slot = var_uc_ndepm_dn10;
        *var_uc_ndepm_dn13_slot = var_uc_ndepm_dn13;
        *var_uc_ndepm_dn2_slot = var_uc_ndepm_dn2;
        *var_uc_ndepm_dn4_slot = var_uc_ndepm_dn4;
        *var_uc_ndepm_dn5_slot = var_uc_ndepm_dn5;
        *var_uc_ndepm_dn6_slot = var_uc_ndepm_dn6;
        *var_uc_ndepm_dn7_slot = var_uc_ndepm_dn7;
        *var_uc_ndepm_dn8_slot = var_uc_ndepm_dn8;
        *var_uc_ndepm_dn9_slot = var_uc_ndepm_dn9;
        *var_uc_ndepm_rv_slot = var_uc_ndepm_rv;
        *var_uc_xpdv_slot = var_uc_xpdv;
        *var_uc_xpdv_rv_slot = var_uc_xpdv_rv;
    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        var_guard247: f64,
        var_guard248: f64,
        var_ktnom: f64,
        var_lg: f64,
        var_nsubcdfm_given: f64,
        var_uc_eg0: f64,
        var_uc_mueph1: f64,
        var_uc_pgd1: f64,
        var_uc_rdict1: f64,
        var_uc_rdslp1: f64,
        var_uc_rs: f64,
        var_uc_scp22: f64,
        var_wg: f64,
        var_cecox_slot: &mut f64,
        var_cecox_rv_slot: &mut f64,
        var_clmmod_slot: &mut f64,
        var_clmmod_rv_slot: &mut f64,
        var_cnstpgd_slot: &mut f64,
        var_cnstpgd_rv_slot: &mut f64,
        var_ef_mueph1_slot: &mut f64,
        var_ef_mueph1_dn0_slot: &mut f64,
        var_ef_mueph1_dn10_slot: &mut f64,
        var_ef_mueph1_dn13_slot: &mut f64,
        var_ef_mueph1_dn2_slot: &mut f64,
        var_ef_mueph1_dn4_slot: &mut f64,
        var_ef_mueph1_dn5_slot: &mut f64,
        var_ef_mueph1_dn6_slot: &mut f64,
        var_ef_mueph1_dn7_slot: &mut f64,
        var_ef_mueph1_dn8_slot: &mut f64,
        var_ef_mueph1_dn9_slot: &mut f64,
        var_ef_mueph1_rv_slot: &mut f64,
        var_ef_nsubc_slot: &mut f64,
        var_ef_nsubc_dn0_slot: &mut f64,
        var_ef_nsubc_dn10_slot: &mut f64,
        var_ef_nsubc_dn13_slot: &mut f64,
        var_ef_nsubc_dn2_slot: &mut f64,
        var_ef_nsubc_dn4_slot: &mut f64,
        var_ef_nsubc_dn5_slot: &mut f64,
        var_ef_nsubc_dn6_slot: &mut f64,
        var_ef_nsubc_dn7_slot: &mut f64,
        var_ef_nsubc_dn8_slot: &mut f64,
        var_ef_nsubc_dn9_slot: &mut f64,
        var_ef_nsubc_rv_slot: &mut f64,
        var_ef_nsubp_slot: &mut f64,
        var_ef_nsubp_dn0_slot: &mut f64,
        var_ef_nsubp_dn10_slot: &mut f64,
        var_ef_nsubp_dn13_slot: &mut f64,
        var_ef_nsubp_dn2_slot: &mut f64,
        var_ef_nsubp_dn4_slot: &mut f64,
        var_ef_nsubp_dn5_slot: &mut f64,
        var_ef_nsubp_dn6_slot: &mut f64,
        var_ef_nsubp_dn7_slot: &mut f64,
        var_ef_nsubp_dn8_slot: &mut f64,
        var_ef_nsubp_dn9_slot: &mut f64,
        var_ef_nsubp_rv_slot: &mut f64,
        var_egtnom_slot: &mut f64,
        var_egtnom_rv_slot: &mut f64,
        var_flg_nqs_slot: &mut f64,
        var_flg_nqs_rv_slot: &mut f64,
        var_flg_pgd_slot: &mut f64,
        var_flg_pgd_rv_slot: &mut f64,
        var_flg_qmetemp_slot: &mut f64,
        var_flg_qmetemp_rv_slot: &mut f64,
        var_flg_qy_slot: &mut f64,
        var_flg_qy_rv_slot: &mut f64,
        var_flg_rs_slot: &mut f64,
        var_flg_rs_rv_slot: &mut f64,
        var_guard250_slot: &mut f64,
        var_guard250_rv_slot: &mut f64,
        var_guard251_slot: &mut f64,
        var_guard251_rv_slot: &mut f64,
        var_guard252_slot: &mut f64,
        var_guard252_rv_slot: &mut f64,
        var_guard253_slot: &mut f64,
        var_guard253_rv_slot: &mut f64,
        var_i_slot: &mut f64,
        var_i_rv_slot: &mut f64,
        var_lod_half_slot: &mut f64,
        var_lod_half_dn0_slot: &mut f64,
        var_lod_half_dn10_slot: &mut f64,
        var_lod_half_dn13_slot: &mut f64,
        var_lod_half_dn2_slot: &mut f64,
        var_lod_half_dn4_slot: &mut f64,
        var_lod_half_dn5_slot: &mut f64,
        var_lod_half_dn6_slot: &mut f64,
        var_lod_half_dn7_slot: &mut f64,
        var_lod_half_dn8_slot: &mut f64,
        var_lod_half_dn9_slot: &mut f64,
        var_lod_half_ref_slot: &mut f64,
        var_lod_half_ref_dn0_slot: &mut f64,
        var_lod_half_ref_dn10_slot: &mut f64,
        var_lod_half_ref_dn13_slot: &mut f64,
        var_lod_half_ref_dn2_slot: &mut f64,
        var_lod_half_ref_dn4_slot: &mut f64,
        var_lod_half_ref_dn5_slot: &mut f64,
        var_lod_half_ref_dn6_slot: &mut f64,
        var_lod_half_ref_dn7_slot: &mut f64,
        var_lod_half_ref_dn8_slot: &mut f64,
        var_lod_half_ref_dn9_slot: &mut f64,
        var_lod_half_ref_rv_slot: &mut f64,
        var_lod_half_rv_slot: &mut f64,
        var_mks_nsubcdfm_slot: &mut f64,
        var_mks_nsubcdfm_rv_slot: &mut f64,
        var_mks_subld2_slot: &mut f64,
        var_mks_subld2_rv_slot: &mut f64,
        var_msc_slot: &mut f64,
        var_msc_rv_slot: &mut f64,
        var_npexte_slot: &mut f64,
        var_npexte_dn0_slot: &mut f64,
        var_npexte_dn10_slot: &mut f64,
        var_npexte_dn13_slot: &mut f64,
        var_npexte_dn2_slot: &mut f64,
        var_npexte_dn4_slot: &mut f64,
        var_npexte_dn5_slot: &mut f64,
        var_npexte_dn6_slot: &mut f64,
        var_npexte_dn7_slot: &mut f64,
        var_npexte_dn8_slot: &mut f64,
        var_npexte_dn9_slot: &mut f64,
        var_npexte_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_uc_depvmax_slot: &mut f64,
        var_uc_depvmax_dn0_slot: &mut f64,
        var_uc_depvmax_dn10_slot: &mut f64,
        var_uc_depvmax_dn13_slot: &mut f64,
        var_uc_depvmax_dn2_slot: &mut f64,
        var_uc_depvmax_dn4_slot: &mut f64,
        var_uc_depvmax_dn5_slot: &mut f64,
        var_uc_depvmax_dn6_slot: &mut f64,
        var_uc_depvmax_dn7_slot: &mut f64,
        var_uc_depvmax_dn8_slot: &mut f64,
        var_uc_depvmax_dn9_slot: &mut f64,
        var_uc_depvmax_rv_slot: &mut f64,
        var_uc_fn2_slot: &mut f64,
        var_uc_fn2_rv_slot: &mut f64,
        var_uc_gdld_slot: &mut f64,
        var_uc_gdld_rv_slot: &mut f64,
        var_uc_gidl1_slot: &mut f64,
        var_uc_gidl1_rv_slot: &mut f64,
        var_uc_gidl2_slot: &mut f64,
        var_uc_gidl2_rv_slot: &mut f64,
        var_uc_gleak2_slot: &mut f64,
        var_uc_gleak2_rv_slot: &mut f64,
        var_uc_glkb2_slot: &mut f64,
        var_uc_glkb2_rv_slot: &mut f64,
        var_uc_glksd1_slot: &mut f64,
        var_uc_glksd1_rv_slot: &mut f64,
        var_uc_glksd2_slot: &mut f64,
        var_uc_glksd2_rv_slot: &mut f64,
        var_uc_muesti1_slot: &mut f64,
        var_uc_muesti1_rv_slot: &mut f64,
        var_uc_nfalp_slot: &mut f64,
        var_uc_nfalp_rv_slot: &mut f64,
        var_uc_nover_slot: &mut f64,
        var_uc_nover_rv_slot: &mut f64,
        var_uc_novers_slot: &mut f64,
        var_uc_novers_rv_slot: &mut f64,
        var_uc_npext_slot: &mut f64,
        var_uc_npext_rv_slot: &mut f64,
        var_uc_nsti_slot: &mut f64,
        var_uc_nsti_rv_slot: &mut f64,
        var_uc_nsubc_slot: &mut f64,
        var_uc_nsubc_rv_slot: &mut f64,
        var_uc_nsubp_slot: &mut f64,
        var_uc_nsubp_rv_slot: &mut f64,
        var_uc_nsubpsti1_slot: &mut f64,
        var_uc_nsubpsti1_rv_slot: &mut f64,
        var_uc_rd22_slot: &mut f64,
        var_uc_rd22_rv_slot: &mut f64,
        var_uc_rd23_slot: &mut f64,
        var_uc_rd23_rv_slot: &mut f64,
        var_uc_rd24_slot: &mut f64,
        var_uc_rd24_rv_slot: &mut f64,
        var_uc_rdvd_slot: &mut f64,
        var_uc_rdvd_rv_slot: &mut f64,
        var_uc_rth0_slot: &mut f64,
        var_uc_rth0_rv_slot: &mut f64,
        var_uc_vfbover_slot: &mut f64,
        var_uc_vfbover_rv_slot: &mut f64,
        var_uc_vmax_slot: &mut f64,
        var_uc_vmax_rv_slot: &mut f64,
        var_uc_wfc_slot: &mut f64,
        var_uc_wfc_rv_slot: &mut f64,
        var_wlg_slot: &mut f64,
        var_wlg_rv_slot: &mut f64,
    ) {
        let mut var_cecox: f64 = *var_cecox_slot;
        let mut var_cecox_rv: f64 = *var_cecox_rv_slot;
        let mut var_clmmod: f64 = *var_clmmod_slot;
        let mut var_clmmod_rv: f64 = *var_clmmod_rv_slot;
        let mut var_cnstpgd: f64 = *var_cnstpgd_slot;
        let mut var_cnstpgd_rv: f64 = *var_cnstpgd_rv_slot;
        let mut var_ef_mueph1: f64 = *var_ef_mueph1_slot;
        let mut var_ef_mueph1_dn0: f64 = *var_ef_mueph1_dn0_slot;
        let mut var_ef_mueph1_dn10: f64 = *var_ef_mueph1_dn10_slot;
        let mut var_ef_mueph1_dn13: f64 = *var_ef_mueph1_dn13_slot;
        let mut var_ef_mueph1_dn2: f64 = *var_ef_mueph1_dn2_slot;
        let mut var_ef_mueph1_dn4: f64 = *var_ef_mueph1_dn4_slot;
        let mut var_ef_mueph1_dn5: f64 = *var_ef_mueph1_dn5_slot;
        let mut var_ef_mueph1_dn6: f64 = *var_ef_mueph1_dn6_slot;
        let mut var_ef_mueph1_dn7: f64 = *var_ef_mueph1_dn7_slot;
        let mut var_ef_mueph1_dn8: f64 = *var_ef_mueph1_dn8_slot;
        let mut var_ef_mueph1_dn9: f64 = *var_ef_mueph1_dn9_slot;
        let mut var_ef_mueph1_rv: f64 = *var_ef_mueph1_rv_slot;
        let mut var_ef_nsubc: f64 = *var_ef_nsubc_slot;
        let mut var_ef_nsubc_dn0: f64 = *var_ef_nsubc_dn0_slot;
        let mut var_ef_nsubc_dn10: f64 = *var_ef_nsubc_dn10_slot;
        let mut var_ef_nsubc_dn13: f64 = *var_ef_nsubc_dn13_slot;
        let mut var_ef_nsubc_dn2: f64 = *var_ef_nsubc_dn2_slot;
        let mut var_ef_nsubc_dn4: f64 = *var_ef_nsubc_dn4_slot;
        let mut var_ef_nsubc_dn5: f64 = *var_ef_nsubc_dn5_slot;
        let mut var_ef_nsubc_dn6: f64 = *var_ef_nsubc_dn6_slot;
        let mut var_ef_nsubc_dn7: f64 = *var_ef_nsubc_dn7_slot;
        let mut var_ef_nsubc_dn8: f64 = *var_ef_nsubc_dn8_slot;
        let mut var_ef_nsubc_dn9: f64 = *var_ef_nsubc_dn9_slot;
        let mut var_ef_nsubc_rv: f64 = *var_ef_nsubc_rv_slot;
        let mut var_ef_nsubp: f64 = *var_ef_nsubp_slot;
        let mut var_ef_nsubp_dn0: f64 = *var_ef_nsubp_dn0_slot;
        let mut var_ef_nsubp_dn10: f64 = *var_ef_nsubp_dn10_slot;
        let mut var_ef_nsubp_dn13: f64 = *var_ef_nsubp_dn13_slot;
        let mut var_ef_nsubp_dn2: f64 = *var_ef_nsubp_dn2_slot;
        let mut var_ef_nsubp_dn4: f64 = *var_ef_nsubp_dn4_slot;
        let mut var_ef_nsubp_dn5: f64 = *var_ef_nsubp_dn5_slot;
        let mut var_ef_nsubp_dn6: f64 = *var_ef_nsubp_dn6_slot;
        let mut var_ef_nsubp_dn7: f64 = *var_ef_nsubp_dn7_slot;
        let mut var_ef_nsubp_dn8: f64 = *var_ef_nsubp_dn8_slot;
        let mut var_ef_nsubp_dn9: f64 = *var_ef_nsubp_dn9_slot;
        let mut var_ef_nsubp_rv: f64 = *var_ef_nsubp_rv_slot;
        let mut var_egtnom: f64 = *var_egtnom_slot;
        let mut var_egtnom_rv: f64 = *var_egtnom_rv_slot;
        let mut var_flg_nqs: f64 = *var_flg_nqs_slot;
        let mut var_flg_nqs_rv: f64 = *var_flg_nqs_rv_slot;
        let mut var_flg_pgd: f64 = *var_flg_pgd_slot;
        let mut var_flg_pgd_rv: f64 = *var_flg_pgd_rv_slot;
        let mut var_flg_qmetemp: f64 = *var_flg_qmetemp_slot;
        let mut var_flg_qmetemp_rv: f64 = *var_flg_qmetemp_rv_slot;
        let mut var_flg_qy: f64 = *var_flg_qy_slot;
        let mut var_flg_qy_rv: f64 = *var_flg_qy_rv_slot;
        let mut var_flg_rs: f64 = *var_flg_rs_slot;
        let mut var_flg_rs_rv: f64 = *var_flg_rs_rv_slot;
        let mut var_guard250: f64 = *var_guard250_slot;
        let mut var_guard250_rv: f64 = *var_guard250_rv_slot;
        let mut var_guard251: f64 = *var_guard251_slot;
        let mut var_guard251_rv: f64 = *var_guard251_rv_slot;
        let mut var_guard252: f64 = *var_guard252_slot;
        let mut var_guard252_rv: f64 = *var_guard252_rv_slot;
        let mut var_guard253: f64 = *var_guard253_slot;
        let mut var_guard253_rv: f64 = *var_guard253_rv_slot;
        let mut var_i: f64 = *var_i_slot;
        let mut var_i_rv: f64 = *var_i_rv_slot;
        let mut var_lod_half: f64 = *var_lod_half_slot;
        let mut var_lod_half_dn0: f64 = *var_lod_half_dn0_slot;
        let mut var_lod_half_dn10: f64 = *var_lod_half_dn10_slot;
        let mut var_lod_half_dn13: f64 = *var_lod_half_dn13_slot;
        let mut var_lod_half_dn2: f64 = *var_lod_half_dn2_slot;
        let mut var_lod_half_dn4: f64 = *var_lod_half_dn4_slot;
        let mut var_lod_half_dn5: f64 = *var_lod_half_dn5_slot;
        let mut var_lod_half_dn6: f64 = *var_lod_half_dn6_slot;
        let mut var_lod_half_dn7: f64 = *var_lod_half_dn7_slot;
        let mut var_lod_half_dn8: f64 = *var_lod_half_dn8_slot;
        let mut var_lod_half_dn9: f64 = *var_lod_half_dn9_slot;
        let mut var_lod_half_ref: f64 = *var_lod_half_ref_slot;
        let mut var_lod_half_ref_dn0: f64 = *var_lod_half_ref_dn0_slot;
        let mut var_lod_half_ref_dn10: f64 = *var_lod_half_ref_dn10_slot;
        let mut var_lod_half_ref_dn13: f64 = *var_lod_half_ref_dn13_slot;
        let mut var_lod_half_ref_dn2: f64 = *var_lod_half_ref_dn2_slot;
        let mut var_lod_half_ref_dn4: f64 = *var_lod_half_ref_dn4_slot;
        let mut var_lod_half_ref_dn5: f64 = *var_lod_half_ref_dn5_slot;
        let mut var_lod_half_ref_dn6: f64 = *var_lod_half_ref_dn6_slot;
        let mut var_lod_half_ref_dn7: f64 = *var_lod_half_ref_dn7_slot;
        let mut var_lod_half_ref_dn8: f64 = *var_lod_half_ref_dn8_slot;
        let mut var_lod_half_ref_dn9: f64 = *var_lod_half_ref_dn9_slot;
        let mut var_lod_half_ref_rv: f64 = *var_lod_half_ref_rv_slot;
        let mut var_lod_half_rv: f64 = *var_lod_half_rv_slot;
        let mut var_mks_nsubcdfm: f64 = *var_mks_nsubcdfm_slot;
        let mut var_mks_nsubcdfm_rv: f64 = *var_mks_nsubcdfm_rv_slot;
        let mut var_mks_subld2: f64 = *var_mks_subld2_slot;
        let mut var_mks_subld2_rv: f64 = *var_mks_subld2_rv_slot;
        let mut var_msc: f64 = *var_msc_slot;
        let mut var_msc_rv: f64 = *var_msc_rv_slot;
        let mut var_npexte: f64 = *var_npexte_slot;
        let mut var_npexte_dn0: f64 = *var_npexte_dn0_slot;
        let mut var_npexte_dn10: f64 = *var_npexte_dn10_slot;
        let mut var_npexte_dn13: f64 = *var_npexte_dn13_slot;
        let mut var_npexte_dn2: f64 = *var_npexte_dn2_slot;
        let mut var_npexte_dn4: f64 = *var_npexte_dn4_slot;
        let mut var_npexte_dn5: f64 = *var_npexte_dn5_slot;
        let mut var_npexte_dn6: f64 = *var_npexte_dn6_slot;
        let mut var_npexte_dn7: f64 = *var_npexte_dn7_slot;
        let mut var_npexte_dn8: f64 = *var_npexte_dn8_slot;
        let mut var_npexte_dn9: f64 = *var_npexte_dn9_slot;
        let mut var_npexte_rv: f64 = *var_npexte_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_uc_depvmax: f64 = *var_uc_depvmax_slot;
        let mut var_uc_depvmax_dn0: f64 = *var_uc_depvmax_dn0_slot;
        let mut var_uc_depvmax_dn10: f64 = *var_uc_depvmax_dn10_slot;
        let mut var_uc_depvmax_dn13: f64 = *var_uc_depvmax_dn13_slot;
        let mut var_uc_depvmax_dn2: f64 = *var_uc_depvmax_dn2_slot;
        let mut var_uc_depvmax_dn4: f64 = *var_uc_depvmax_dn4_slot;
        let mut var_uc_depvmax_dn5: f64 = *var_uc_depvmax_dn5_slot;
        let mut var_uc_depvmax_dn6: f64 = *var_uc_depvmax_dn6_slot;
        let mut var_uc_depvmax_dn7: f64 = *var_uc_depvmax_dn7_slot;
        let mut var_uc_depvmax_dn8: f64 = *var_uc_depvmax_dn8_slot;
        let mut var_uc_depvmax_dn9: f64 = *var_uc_depvmax_dn9_slot;
        let mut var_uc_depvmax_rv: f64 = *var_uc_depvmax_rv_slot;
        let mut var_uc_fn2: f64 = *var_uc_fn2_slot;
        let mut var_uc_fn2_rv: f64 = *var_uc_fn2_rv_slot;
        let mut var_uc_gdld: f64 = *var_uc_gdld_slot;
        let mut var_uc_gdld_rv: f64 = *var_uc_gdld_rv_slot;
        let mut var_uc_gidl1: f64 = *var_uc_gidl1_slot;
        let mut var_uc_gidl1_rv: f64 = *var_uc_gidl1_rv_slot;
        let mut var_uc_gidl2: f64 = *var_uc_gidl2_slot;
        let mut var_uc_gidl2_rv: f64 = *var_uc_gidl2_rv_slot;
        let mut var_uc_gleak2: f64 = *var_uc_gleak2_slot;
        let mut var_uc_gleak2_rv: f64 = *var_uc_gleak2_rv_slot;
        let mut var_uc_glkb2: f64 = *var_uc_glkb2_slot;
        let mut var_uc_glkb2_rv: f64 = *var_uc_glkb2_rv_slot;
        let mut var_uc_glksd1: f64 = *var_uc_glksd1_slot;
        let mut var_uc_glksd1_rv: f64 = *var_uc_glksd1_rv_slot;
        let mut var_uc_glksd2: f64 = *var_uc_glksd2_slot;
        let mut var_uc_glksd2_rv: f64 = *var_uc_glksd2_rv_slot;
        let mut var_uc_muesti1: f64 = *var_uc_muesti1_slot;
        let mut var_uc_muesti1_rv: f64 = *var_uc_muesti1_rv_slot;
        let mut var_uc_nfalp: f64 = *var_uc_nfalp_slot;
        let mut var_uc_nfalp_rv: f64 = *var_uc_nfalp_rv_slot;
        let mut var_uc_nover: f64 = *var_uc_nover_slot;
        let mut var_uc_nover_rv: f64 = *var_uc_nover_rv_slot;
        let mut var_uc_novers: f64 = *var_uc_novers_slot;
        let mut var_uc_novers_rv: f64 = *var_uc_novers_rv_slot;
        let mut var_uc_npext: f64 = *var_uc_npext_slot;
        let mut var_uc_npext_rv: f64 = *var_uc_npext_rv_slot;
        let mut var_uc_nsti: f64 = *var_uc_nsti_slot;
        let mut var_uc_nsti_rv: f64 = *var_uc_nsti_rv_slot;
        let mut var_uc_nsubc: f64 = *var_uc_nsubc_slot;
        let mut var_uc_nsubc_rv: f64 = *var_uc_nsubc_rv_slot;
        let mut var_uc_nsubp: f64 = *var_uc_nsubp_slot;
        let mut var_uc_nsubp_rv: f64 = *var_uc_nsubp_rv_slot;
        let mut var_uc_nsubpsti1: f64 = *var_uc_nsubpsti1_slot;
        let mut var_uc_nsubpsti1_rv: f64 = *var_uc_nsubpsti1_rv_slot;
        let mut var_uc_rd22: f64 = *var_uc_rd22_slot;
        let mut var_uc_rd22_rv: f64 = *var_uc_rd22_rv_slot;
        let mut var_uc_rd23: f64 = *var_uc_rd23_slot;
        let mut var_uc_rd23_rv: f64 = *var_uc_rd23_rv_slot;
        let mut var_uc_rd24: f64 = *var_uc_rd24_slot;
        let mut var_uc_rd24_rv: f64 = *var_uc_rd24_rv_slot;
        let mut var_uc_rdvd: f64 = *var_uc_rdvd_slot;
        let mut var_uc_rdvd_rv: f64 = *var_uc_rdvd_rv_slot;
        let mut var_uc_rth0: f64 = *var_uc_rth0_slot;
        let mut var_uc_rth0_rv: f64 = *var_uc_rth0_rv_slot;
        let mut var_uc_vfbover: f64 = *var_uc_vfbover_slot;
        let mut var_uc_vfbover_rv: f64 = *var_uc_vfbover_rv_slot;
        let mut var_uc_vmax: f64 = *var_uc_vmax_slot;
        let mut var_uc_vmax_rv: f64 = *var_uc_vmax_rv_slot;
        let mut var_uc_wfc: f64 = *var_uc_wfc_slot;
        let mut var_uc_wfc_rv: f64 = *var_uc_wfc_rv_slot;
        let mut var_wlg: f64 = *var_wlg_slot;
        let mut var_wlg_rv: f64 = *var_wlg_rv_slot;

        let (assign10260_e5476, assign10260_e5476_d_n0, assign10260_e5476_d_n2, assign10260_e5476_d_n4, assign10260_e5476_d_n5, assign10260_e5476_d_n6, assign10260_e5476_d_n7, assign10260_e5476_d_n8, assign10260_e5476_d_n9, assign10260_e5476_d_n10, assign10260_e5476_d_n13,) = {
    if ((var_guard247 == 0.0) && (var_guard248 == 0.0)) {
        let assign10260_e5454: f64 = (p.p131 * p.p3);
        let assign10260_e5456: f64 = (assign10260_e5454 * p.p7);
        let assign10260_e5460: f64 = (p.p69 * var_uc_rdslp1);
        let assign10260_e5462: f64 = (assign10260_e5460 * 1000000.0);
        let assign10260_e5464: f64 = (assign10260_e5462 + var_uc_rdict1);
        let assign10260_e5465: f64 = (var_uc_rs * assign10260_e5464);
        let assign10260_e5468: f64 = (p.p70 * p.p100);
        let assign10260_e5470: f64 = (assign10260_e5468 * 1000000.0);
        let assign10260_e5472: f64 = (assign10260_e5470 + p.p101);
        let assign10260_e5473: f64 = (assign10260_e5465 * assign10260_e5472);
        let assign10260_e5474: f64 = (assign10260_e5456 + assign10260_e5473);
        (assign10260_e5474, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign10260_e5476;
        var_t1_dn0 = assign10260_e5476_d_n0;
        var_t1_dn2 = assign10260_e5476_d_n2;
        var_t1_dn4 = assign10260_e5476_d_n4;
        var_t1_dn5 = assign10260_e5476_d_n5;
        var_t1_dn6 = assign10260_e5476_d_n6;
        var_t1_dn7 = assign10260_e5476_d_n7;
        var_t1_dn8 = assign10260_e5476_d_n8;
        var_t1_dn9 = assign10260_e5476_d_n9;
        var_t1_dn10 = assign10260_e5476_d_n10;
        var_t1_dn13 = assign10260_e5476_d_n13;
        var_t1_rv = 0.0;

        let (assign10270_e5489,) = {
    if ((var_guard247 == 0.0) && (var_guard248 == 0.0)) {
        let (assign10270_e5487,) = {
            if (var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10270_e5487,)
    } else {
        (var_flg_rs,)
    }
};
        var_flg_rs = assign10270_e5489;
        var_flg_rs_rv = 0.0;

        let assign10280_e5492: f64 = (p.p12 / 1e-6);
        var_mks_nsubcdfm = assign10280_e5492;
        var_mks_nsubcdfm_rv = 0.0;

        let assign10290_e5495: f64 = (p.p73 * 100.0);
        var_mks_subld2 = assign10290_e5495;
        var_mks_subld2_rv = 0.0;

        let assign10300_e5498: f64 = (var_uc_nsubc / 1e-6);
        var_uc_nsubc = assign10300_e5498;
        var_uc_nsubc_rv = 0.0;

        let assign10310_e5501: f64 = (var_uc_nsubp / 1e-6);
        var_uc_nsubp = assign10310_e5501;
        var_uc_nsubp_rv = 0.0;

        let assign10320_e5504: f64 = (var_uc_nsti / 1e-6);
        var_uc_nsti = assign10320_e5504;
        var_uc_nsti_rv = 0.0;

        let assign10330_e5507: f64 = (var_uc_nover / 1e-6);
        var_uc_nover = assign10330_e5507;
        var_uc_nover_rv = 0.0;

        let assign10340_e5510: f64 = (var_uc_novers / 1e-6);
        var_uc_novers = assign10340_e5510;
        var_uc_novers_rv = 0.0;

        let assign10350_e5513: f64 = (var_uc_nsubpsti1 / 100.0);
        var_uc_nsubpsti1 = assign10350_e5513;
        var_uc_nsubpsti1_rv = 0.0;

        let assign10360_e5516: f64 = (var_uc_muesti1 / 100.0);
        var_uc_muesti1 = assign10360_e5516;
        var_uc_muesti1_rv = 0.0;

        let assign10370_e5519: f64 = (var_uc_vmax / 100.0);
        var_uc_vmax = assign10370_e5519;
        var_uc_vmax_rv = 0.0;

        let assign10380_e5522: f64 = (var_uc_wfc * 10000.0);
        var_uc_wfc = assign10380_e5522;
        var_uc_wfc_rv = 0.0;

        let assign10390_e5525: f64 = (var_uc_glksd1 / 100.0);
        var_uc_glksd1 = assign10390_e5525;
        var_uc_glksd1_rv = 0.0;

        let assign10400_e5528: f64 = (var_uc_glksd2 * 100.0);
        var_uc_glksd2 = assign10400_e5528;
        var_uc_glksd2_rv = 0.0;

        let assign10410_e5531: f64 = (var_uc_gleak2 * 100.0);
        var_uc_gleak2 = assign10410_e5531;
        var_uc_gleak2_rv = 0.0;

        let assign10420_e5534: f64 = (var_uc_glkb2 * 100.0);
        var_uc_glkb2 = assign10420_e5534;
        var_uc_glkb2_rv = 0.0;

        let assign10430_e5537: f64 = (var_uc_fn2 * 100.0);
        var_uc_fn2 = assign10430_e5537;
        var_uc_fn2_rv = 0.0;

        let assign10440_e5540: f64 = (var_uc_gidl1 / 10.0);
        var_uc_gidl1 = assign10440_e5540;
        var_uc_gidl1_rv = 0.0;

        let assign10450_e5543: f64 = (var_uc_gidl2 * 100.0);
        var_uc_gidl2 = assign10450_e5543;
        var_uc_gidl2_rv = 0.0;

        let assign10460_e5546: f64 = (var_uc_nfalp / 100.0);
        var_uc_nfalp = assign10460_e5546;
        var_uc_nfalp_rv = 0.0;

        let assign10480_e5552: f64 = (var_uc_npext / 1e-6);
        var_uc_npext = assign10480_e5552;
        var_uc_npext_rv = 0.0;

        let assign10490_e5555: f64 = (var_uc_rd22 / 100.0);
        var_uc_rd22 = assign10490_e5555;
        var_uc_rd22_rv = 0.0;

        let assign10500_e5558: f64 = (var_uc_rd23 / 100.0);
        var_uc_rd23 = assign10500_e5558;
        var_uc_rd23_rv = 0.0;

        let assign10510_e5561: f64 = (var_uc_rd24 / 100.0);
        var_uc_rd24 = assign10510_e5561;
        var_uc_rd24_rv = 0.0;

        let assign10520_e5564: f64 = (var_uc_rdvd / 100.0);
        var_uc_rdvd = assign10520_e5564;
        var_uc_rdvd_rv = 0.0;

        let assign10530_e5567: f64 = (var_uc_rth0 / 100.0);
        var_uc_rth0 = assign10530_e5567;
        var_uc_rth0_rv = 0.0;

        let assign10540_e5569: f64 = (-var_uc_vfbover);
        var_uc_vfbover = assign10540_e5569;
        var_uc_vfbover_rv = 0.0;

        let assign10550_e5572: f64 = (var_uc_depvmax / 100.0);
        var_uc_depvmax = assign10550_e5572;
        var_uc_depvmax_dn0 = (var_uc_depvmax_dn0 / 100.0);
        var_uc_depvmax_dn2 = (var_uc_depvmax_dn2 / 100.0);
        var_uc_depvmax_dn4 = (var_uc_depvmax_dn4 / 100.0);
        var_uc_depvmax_dn5 = (var_uc_depvmax_dn5 / 100.0);
        var_uc_depvmax_dn6 = (var_uc_depvmax_dn6 / 100.0);
        var_uc_depvmax_dn7 = (var_uc_depvmax_dn7 / 100.0);
        var_uc_depvmax_dn8 = (var_uc_depvmax_dn8 / 100.0);
        var_uc_depvmax_dn9 = (var_uc_depvmax_dn9 / 100.0);
        var_uc_depvmax_dn10 = (var_uc_depvmax_dn10 / 100.0);
        var_uc_depvmax_dn13 = (var_uc_depvmax_dn13 / 100.0);
        var_uc_depvmax_rv = 0.0;

        var_flg_nqs = p.p28;
        var_flg_nqs_rv = 0.0;

        let (assign10570_e5583,) = {
    if ((p.p133 != 0.0) || (p.p134 != 0.0)) {
        (1.0,)
    } else {
        (0.0,)
    }
};
        var_flg_qy = assign10570_e5583;
        var_flg_qy_rv = 0.0;

        let assign10590_e5597: f64 = if (((p.p235 == 0.0) && (p.p237 == 0.0)) || (p.p236 == 0.0)) { 1.0 } else { 0.0 };
        var_guard250 = assign10590_e5597;
        var_guard250_rv = 0.0;

        let (assign10600_e5601,) = {
    if (var_guard250 != 0.0) {
        (0.0,)
    } else {
        (var_flg_qmetemp,)
    }
};
        var_flg_qmetemp = assign10600_e5601;
        var_flg_qmetemp_rv = 0.0;

        let (assign10610_e5606,) = {
    if (var_guard250 == 0.0) {
        (1.0,)
    } else {
        (var_flg_qmetemp,)
    }
};
        var_flg_qmetemp = assign10610_e5606;
        var_flg_qmetemp_rv = 0.0;

        let assign10620_e5609: f64 = (var_wg * var_lg);
        var_wlg = assign10620_e5609;
        var_wlg_rv = 0.0;

        let assign10630_e5612: f64 = (p.p289 * 1000000.0);
        var_uc_gdld = assign10630_e5612;
        var_uc_gdld_rv = 0.0;

        let assign10640_e5618: f64 = (var_ktnom * 1e-7);
        let assign10640_e5619: f64 = (9.025e-5 + assign10640_e5618);
        let assign10640_e5620: f64 = (var_ktnom * assign10640_e5619);
        let assign10640_e5621: f64 = (var_uc_eg0 - assign10640_e5620);
        var_egtnom = assign10640_e5621;
        var_egtnom_rv = 0.0;

        let assign10650_e5624: f64 = (8.8541878e-12 * p.p267);
        var_cecox = assign10650_e5624;
        var_cecox_rv = 0.0;

        var_msc = var_uc_scp22;
        var_msc_rv = 0.0;

        let assign10670_e5628: f64 = if var_uc_pgd1 == 0.0 { 1.0 } else { 0.0 };
        var_guard251 = assign10670_e5628;
        var_guard251_rv = 0.0;

        let (assign10680_e5632,) = {
    if (var_guard251 != 0.0) {
        (0.0,)
    } else {
        (var_flg_pgd,)
    }
};
        var_flg_pgd = assign10680_e5632;
        var_flg_pgd_rv = 0.0;

        let (assign10690_e5636,) = {
    if (var_guard251 != 0.0) {
        (0.0,)
    } else {
        (var_cnstpgd,)
    }
};
        var_cnstpgd = assign10690_e5636;
        var_cnstpgd_rv = 0.0;

        let (assign10700_e5641,) = {
    if (var_guard251 == 0.0) {
        (1.0,)
    } else {
        (var_flg_pgd,)
    }
};
        var_flg_pgd = assign10700_e5641;
        var_flg_pgd_rv = 0.0;

        let (assign10710_e5654,) = {
    if (var_guard251 == 0.0) {
        let assign10710_e5647: f64 = (1.0 / var_lg);
        let assign10710_e5648: f64 = (1.0 + assign10710_e5647);
        let assign10710_e5650: f64 = (assign10710_e5648).powf(p.p153);
        let assign10710_e5652: f64 = (assign10710_e5650 * var_uc_pgd1);
        (assign10710_e5652,)
    } else {
        (var_cnstpgd,)
    }
};
        var_cnstpgd = assign10710_e5654;
        var_cnstpgd_rv = 0.0;

        let assign10720_e5658: f64 = (var_lg).powf(p.p229);
        let assign10720_e5660: f64 = (assign10720_e5658 * p.p230);
        let assign10720_e5661: f64 = (1.0 + assign10720_e5660);
        var_clmmod = assign10720_e5661;
        var_clmmod_rv = 0.0;

        let assign10730_e5666: f64 = (0.5 * p.p0);
        let assign10730_e5667: f64 = (p.p118 + assign10730_e5666);
        let assign10730_e5668: f64 = (1.0 / assign10730_e5667);
        let assign10730_e5673: f64 = (0.5 * p.p0);
        let assign10730_e5674: f64 = (p.p119 + assign10730_e5673);
        let assign10730_e5675: f64 = (1.0 / assign10730_e5674);
        let assign10730_e5676: f64 = (assign10730_e5668 + assign10730_e5675);
        var_t1 = assign10730_e5676;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_rv = 0.0;

        let assign10740_e5679: f64 = (2.0 / var_t1);
        var_lod_half_ref = assign10740_e5679;
        var_lod_half_ref_dn0 = (-((2.0 * var_t1_dn0) / (var_t1 * var_t1)));
        var_lod_half_ref_dn2 = (-((2.0 * var_t1_dn2) / (var_t1 * var_t1)));
        var_lod_half_ref_dn4 = (-((2.0 * var_t1_dn4) / (var_t1 * var_t1)));
        var_lod_half_ref_dn5 = (-((2.0 * var_t1_dn5) / (var_t1 * var_t1)));
        var_lod_half_ref_dn6 = (-((2.0 * var_t1_dn6) / (var_t1 * var_t1)));
        var_lod_half_ref_dn7 = (-((2.0 * var_t1_dn7) / (var_t1 * var_t1)));
        var_lod_half_ref_dn8 = (-((2.0 * var_t1_dn8) / (var_t1 * var_t1)));
        var_lod_half_ref_dn9 = (-((2.0 * var_t1_dn9) / (var_t1 * var_t1)));
        var_lod_half_ref_dn10 = (-((2.0 * var_t1_dn10) / (var_t1 * var_t1)));
        var_lod_half_ref_dn13 = (-((2.0 * var_t1_dn13) / (var_t1 * var_t1)));
        var_lod_half_ref_rv = 0.0;

        let assign10750_e5698: f64 = if (((p.p8 > 0.0) && (p.p9 > 0.0)) && ((p.p7 == 1.0) || ((p.p7 > 1.0) && (p.p10 > 0.0)))) { 1.0 } else { 0.0 };
        var_guard252 = assign10750_e5698;
        var_guard252_rv = 0.0;

        let (assign10760_e5702, assign10760_e5702_d_n0, assign10760_e5702_d_n2, assign10760_e5702_d_n4, assign10760_e5702_d_n5, assign10760_e5702_d_n6, assign10760_e5702_d_n7, assign10760_e5702_d_n8, assign10760_e5702_d_n9, assign10760_e5702_d_n10, assign10760_e5702_d_n13,) = {
    if (var_guard252 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign10760_e5702;
        var_t1_dn0 = assign10760_e5702_d_n0;
        var_t1_dn2 = assign10760_e5702_d_n2;
        var_t1_dn4 = assign10760_e5702_d_n4;
        var_t1_dn5 = assign10760_e5702_d_n5;
        var_t1_dn6 = assign10760_e5702_d_n6;
        var_t1_dn7 = assign10760_e5702_d_n7;
        var_t1_dn8 = assign10760_e5702_d_n8;
        var_t1_dn9 = assign10760_e5702_d_n9;
        var_t1_dn10 = assign10760_e5702_d_n10;
        var_t1_dn13 = assign10760_e5702_d_n13;
        var_t1_rv = 0.0;

        let (assign10770_e5706,) = {
    if (var_guard252 != 0.0) {
        (0.0,)
    } else {
        (var_i,)
    }
};
        var_i = assign10770_e5706;
        var_i_rv = 0.0;

        let mut assign10780_loop_guard: usize = 0;
        while {
            let assign10780_cond_e5711: f64 = if ((var_guard252 != 0.0) && (var_i < p.p7)) { 1.0 } else { 0.0 };
            assign10780_cond_e5711 != 0.0
        } {
            assign10780_loop_guard += 1;
            assert!(assign10780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign10780_body0_e5743, assign10780_body0_e5743_d_n0, assign10780_body0_e5743_d_n2, assign10780_body0_e5743_d_n4, assign10780_body0_e5743_d_n5, assign10780_body0_e5743_d_n6, assign10780_body0_e5743_d_n7, assign10780_body0_e5743_d_n8, assign10780_body0_e5743_d_n9, assign10780_body0_e5743_d_n10, assign10780_body0_e5743_d_n13,) = {
    if (var_guard252 != 0.0) {
        let assign10780_body0_e5718: f64 = (0.5 * p.p0);
        let assign10780_body0_e5719: f64 = (p.p8 + assign10780_body0_e5718);
        let assign10780_body0_e5723: f64 = (p.p10 + p.p0);
        let assign10780_body0_e5724: f64 = (var_i * assign10780_body0_e5723);
        let assign10780_body0_e5725: f64 = (assign10780_body0_e5719 + assign10780_body0_e5724);
        let assign10780_body0_e5726: f64 = (1.0 / assign10780_body0_e5725);
        let assign10780_body0_e5727: f64 = (var_t1 + assign10780_body0_e5726);
        let assign10780_body0_e5732: f64 = (0.5 * p.p0);
        let assign10780_body0_e5733: f64 = (p.p9 + assign10780_body0_e5732);
        let assign10780_body0_e5737: f64 = (p.p10 + p.p0);
        let assign10780_body0_e5738: f64 = (var_i * assign10780_body0_e5737);
        let assign10780_body0_e5739: f64 = (assign10780_body0_e5733 + assign10780_body0_e5738);
        let assign10780_body0_e5740: f64 = (1.0 / assign10780_body0_e5739);
        let assign10780_body0_e5741: f64 = (assign10780_body0_e5727 + assign10780_body0_e5740);
        (assign10780_body0_e5741, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
            var_t1 = assign10780_body0_e5743;
            var_t1_dn0 = assign10780_body0_e5743_d_n0;
            var_t1_dn2 = assign10780_body0_e5743_d_n2;
            var_t1_dn4 = assign10780_body0_e5743_d_n4;
            var_t1_dn5 = assign10780_body0_e5743_d_n5;
            var_t1_dn6 = assign10780_body0_e5743_d_n6;
            var_t1_dn7 = assign10780_body0_e5743_d_n7;
            var_t1_dn8 = assign10780_body0_e5743_d_n8;
            var_t1_dn9 = assign10780_body0_e5743_d_n9;
            var_t1_dn10 = assign10780_body0_e5743_d_n10;
            var_t1_dn13 = assign10780_body0_e5743_d_n13;
            var_t1_rv = 0.0;
            let (assign10780_body1_e5749,) = {
    if (var_guard252 != 0.0) {
        let assign10780_body1_e5747: f64 = (var_i + 1.0);
        (assign10780_body1_e5747,)
    } else {
        (var_i,)
    }
};
            var_i = assign10780_body1_e5749;
            var_i_rv = 0.0;
        }

        let (assign10790_e5757, assign10790_e5757_d_n0, assign10790_e5757_d_n2, assign10790_e5757_d_n4, assign10790_e5757_d_n5, assign10790_e5757_d_n6, assign10790_e5757_d_n7, assign10790_e5757_d_n8, assign10790_e5757_d_n9, assign10790_e5757_d_n10, assign10790_e5757_d_n13,) = {
    if (var_guard252 != 0.0) {
        let assign10790_e5753: f64 = (2.0 * p.p7);
        let assign10790_e5755: f64 = (assign10790_e5753 / var_t1);
        (assign10790_e5755, (-((assign10790_e5753 * var_t1_dn0) / (var_t1 * var_t1))), (-((assign10790_e5753 * var_t1_dn2) / (var_t1 * var_t1))), (-((assign10790_e5753 * var_t1_dn4) / (var_t1 * var_t1))), (-((assign10790_e5753 * var_t1_dn5) / (var_t1 * var_t1))), (-((assign10790_e5753 * var_t1_dn6) / (var_t1 * var_t1))), (-((assign10790_e5753 * var_t1_dn7) / (var_t1 * var_t1))), (-((assign10790_e5753 * var_t1_dn8) / (var_t1 * var_t1))), (-((assign10790_e5753 * var_t1_dn9) / (var_t1 * var_t1))), (-((assign10790_e5753 * var_t1_dn10) / (var_t1 * var_t1))), (-((assign10790_e5753 * var_t1_dn13) / (var_t1 * var_t1))),)
    } else {
        (var_lod_half, var_lod_half_dn0, var_lod_half_dn2, var_lod_half_dn4, var_lod_half_dn5, var_lod_half_dn6, var_lod_half_dn7, var_lod_half_dn8, var_lod_half_dn9, var_lod_half_dn10, var_lod_half_dn13,)
    }
};
        var_lod_half = assign10790_e5757;
        var_lod_half_dn0 = assign10790_e5757_d_n0;
        var_lod_half_dn2 = assign10790_e5757_d_n2;
        var_lod_half_dn4 = assign10790_e5757_d_n4;
        var_lod_half_dn5 = assign10790_e5757_d_n5;
        var_lod_half_dn6 = assign10790_e5757_d_n6;
        var_lod_half_dn7 = assign10790_e5757_d_n7;
        var_lod_half_dn8 = assign10790_e5757_d_n8;
        var_lod_half_dn9 = assign10790_e5757_d_n9;
        var_lod_half_dn10 = assign10790_e5757_d_n10;
        var_lod_half_dn13 = assign10790_e5757_d_n13;
        var_lod_half_rv = 0.0;

        let (assign10800_e5762, assign10800_e5762_d_n0, assign10800_e5762_d_n2, assign10800_e5762_d_n4, assign10800_e5762_d_n5, assign10800_e5762_d_n6, assign10800_e5762_d_n7, assign10800_e5762_d_n8, assign10800_e5762_d_n9, assign10800_e5762_d_n10, assign10800_e5762_d_n13,) = {
    if (var_guard252 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_lod_half, var_lod_half_dn0, var_lod_half_dn2, var_lod_half_dn4, var_lod_half_dn5, var_lod_half_dn6, var_lod_half_dn7, var_lod_half_dn8, var_lod_half_dn9, var_lod_half_dn10, var_lod_half_dn13,)
    }
};
        var_lod_half = assign10800_e5762;
        var_lod_half_dn0 = assign10800_e5762_d_n0;
        var_lod_half_dn2 = assign10800_e5762_d_n2;
        var_lod_half_dn4 = assign10800_e5762_d_n4;
        var_lod_half_dn5 = assign10800_e5762_d_n5;
        var_lod_half_dn6 = assign10800_e5762_d_n6;
        var_lod_half_dn7 = assign10800_e5762_d_n7;
        var_lod_half_dn8 = assign10800_e5762_d_n8;
        var_lod_half_dn9 = assign10800_e5762_d_n9;
        var_lod_half_dn10 = assign10800_e5762_d_n10;
        var_lod_half_dn13 = assign10800_e5762_d_n13;
        var_lod_half_rv = 0.0;

        var_npexte = var_uc_npext;
        var_npexte_dn0 = 0.0;
        var_npexte_dn2 = 0.0;
        var_npexte_dn4 = 0.0;
        var_npexte_dn5 = 0.0;
        var_npexte_dn6 = 0.0;
        var_npexte_dn7 = 0.0;
        var_npexte_dn8 = 0.0;
        var_npexte_dn9 = 0.0;
        var_npexte_dn10 = 0.0;
        var_npexte_dn13 = 0.0;
        var_npexte_rv = 0.0;

        var_ef_mueph1 = var_uc_mueph1;
        var_ef_mueph1_dn0 = 0.0;
        var_ef_mueph1_dn2 = 0.0;
        var_ef_mueph1_dn4 = 0.0;
        var_ef_mueph1_dn5 = 0.0;
        var_ef_mueph1_dn6 = 0.0;
        var_ef_mueph1_dn7 = 0.0;
        var_ef_mueph1_dn8 = 0.0;
        var_ef_mueph1_dn9 = 0.0;
        var_ef_mueph1_dn10 = 0.0;
        var_ef_mueph1_dn13 = 0.0;
        var_ef_mueph1_rv = 0.0;

        var_ef_nsubp = var_uc_nsubp;
        var_ef_nsubp_dn0 = 0.0;
        var_ef_nsubp_dn2 = 0.0;
        var_ef_nsubp_dn4 = 0.0;
        var_ef_nsubp_dn5 = 0.0;
        var_ef_nsubp_dn6 = 0.0;
        var_ef_nsubp_dn7 = 0.0;
        var_ef_nsubp_dn8 = 0.0;
        var_ef_nsubp_dn9 = 0.0;
        var_ef_nsubp_dn10 = 0.0;
        var_ef_nsubp_dn13 = 0.0;
        var_ef_nsubp_rv = 0.0;

        var_ef_nsubc = var_uc_nsubc;
        var_ef_nsubc_dn0 = 0.0;
        var_ef_nsubc_dn2 = 0.0;
        var_ef_nsubc_dn4 = 0.0;
        var_ef_nsubc_dn5 = 0.0;
        var_ef_nsubc_dn6 = 0.0;
        var_ef_nsubc_dn7 = 0.0;
        var_ef_nsubc_dn8 = 0.0;
        var_ef_nsubc_dn9 = 0.0;
        var_ef_nsubc_dn10 = 0.0;
        var_ef_nsubc_dn13 = 0.0;
        var_ef_nsubc_rv = 0.0;

        let assign10850_e5771: f64 = if ((p.p32 == 1.0) && (var_nsubcdfm_given != 0.0)) { 1.0 } else { 0.0 };
        var_guard253 = assign10850_e5771;
        var_guard253_rv = 0.0;

        *var_cecox_slot = var_cecox;
        *var_cecox_rv_slot = var_cecox_rv;
        *var_clmmod_slot = var_clmmod;
        *var_clmmod_rv_slot = var_clmmod_rv;
        *var_cnstpgd_slot = var_cnstpgd;
        *var_cnstpgd_rv_slot = var_cnstpgd_rv;
        *var_ef_mueph1_slot = var_ef_mueph1;
        *var_ef_mueph1_dn0_slot = var_ef_mueph1_dn0;
        *var_ef_mueph1_dn10_slot = var_ef_mueph1_dn10;
        *var_ef_mueph1_dn13_slot = var_ef_mueph1_dn13;
        *var_ef_mueph1_dn2_slot = var_ef_mueph1_dn2;
        *var_ef_mueph1_dn4_slot = var_ef_mueph1_dn4;
        *var_ef_mueph1_dn5_slot = var_ef_mueph1_dn5;
        *var_ef_mueph1_dn6_slot = var_ef_mueph1_dn6;
        *var_ef_mueph1_dn7_slot = var_ef_mueph1_dn7;
        *var_ef_mueph1_dn8_slot = var_ef_mueph1_dn8;
        *var_ef_mueph1_dn9_slot = var_ef_mueph1_dn9;
        *var_ef_mueph1_rv_slot = var_ef_mueph1_rv;
        *var_ef_nsubc_slot = var_ef_nsubc;
        *var_ef_nsubc_dn0_slot = var_ef_nsubc_dn0;
        *var_ef_nsubc_dn10_slot = var_ef_nsubc_dn10;
        *var_ef_nsubc_dn13_slot = var_ef_nsubc_dn13;
        *var_ef_nsubc_dn2_slot = var_ef_nsubc_dn2;
        *var_ef_nsubc_dn4_slot = var_ef_nsubc_dn4;
        *var_ef_nsubc_dn5_slot = var_ef_nsubc_dn5;
        *var_ef_nsubc_dn6_slot = var_ef_nsubc_dn6;
        *var_ef_nsubc_dn7_slot = var_ef_nsubc_dn7;
        *var_ef_nsubc_dn8_slot = var_ef_nsubc_dn8;
        *var_ef_nsubc_dn9_slot = var_ef_nsubc_dn9;
        *var_ef_nsubc_rv_slot = var_ef_nsubc_rv;
        *var_ef_nsubp_slot = var_ef_nsubp;
        *var_ef_nsubp_dn0_slot = var_ef_nsubp_dn0;
        *var_ef_nsubp_dn10_slot = var_ef_nsubp_dn10;
        *var_ef_nsubp_dn13_slot = var_ef_nsubp_dn13;
        *var_ef_nsubp_dn2_slot = var_ef_nsubp_dn2;
        *var_ef_nsubp_dn4_slot = var_ef_nsubp_dn4;
        *var_ef_nsubp_dn5_slot = var_ef_nsubp_dn5;
        *var_ef_nsubp_dn6_slot = var_ef_nsubp_dn6;
        *var_ef_nsubp_dn7_slot = var_ef_nsubp_dn7;
        *var_ef_nsubp_dn8_slot = var_ef_nsubp_dn8;
        *var_ef_nsubp_dn9_slot = var_ef_nsubp_dn9;
        *var_ef_nsubp_rv_slot = var_ef_nsubp_rv;
        *var_egtnom_slot = var_egtnom;
        *var_egtnom_rv_slot = var_egtnom_rv;
        *var_flg_nqs_slot = var_flg_nqs;
        *var_flg_nqs_rv_slot = var_flg_nqs_rv;
        *var_flg_pgd_slot = var_flg_pgd;
        *var_flg_pgd_rv_slot = var_flg_pgd_rv;
        *var_flg_qmetemp_slot = var_flg_qmetemp;
        *var_flg_qmetemp_rv_slot = var_flg_qmetemp_rv;
        *var_flg_qy_slot = var_flg_qy;
        *var_flg_qy_rv_slot = var_flg_qy_rv;
        *var_flg_rs_slot = var_flg_rs;
        *var_flg_rs_rv_slot = var_flg_rs_rv;
        *var_guard250_slot = var_guard250;
        *var_guard250_rv_slot = var_guard250_rv;
        *var_guard251_slot = var_guard251;
        *var_guard251_rv_slot = var_guard251_rv;
        *var_guard252_slot = var_guard252;
        *var_guard252_rv_slot = var_guard252_rv;
        *var_guard253_slot = var_guard253;
        *var_guard253_rv_slot = var_guard253_rv;
        *var_i_slot = var_i;
        *var_i_rv_slot = var_i_rv;
        *var_lod_half_slot = var_lod_half;
        *var_lod_half_dn0_slot = var_lod_half_dn0;
        *var_lod_half_dn10_slot = var_lod_half_dn10;
        *var_lod_half_dn13_slot = var_lod_half_dn13;
        *var_lod_half_dn2_slot = var_lod_half_dn2;
        *var_lod_half_dn4_slot = var_lod_half_dn4;
        *var_lod_half_dn5_slot = var_lod_half_dn5;
        *var_lod_half_dn6_slot = var_lod_half_dn6;
        *var_lod_half_dn7_slot = var_lod_half_dn7;
        *var_lod_half_dn8_slot = var_lod_half_dn8;
        *var_lod_half_dn9_slot = var_lod_half_dn9;
        *var_lod_half_ref_slot = var_lod_half_ref;
        *var_lod_half_ref_dn0_slot = var_lod_half_ref_dn0;
        *var_lod_half_ref_dn10_slot = var_lod_half_ref_dn10;
        *var_lod_half_ref_dn13_slot = var_lod_half_ref_dn13;
        *var_lod_half_ref_dn2_slot = var_lod_half_ref_dn2;
        *var_lod_half_ref_dn4_slot = var_lod_half_ref_dn4;
        *var_lod_half_ref_dn5_slot = var_lod_half_ref_dn5;
        *var_lod_half_ref_dn6_slot = var_lod_half_ref_dn6;
        *var_lod_half_ref_dn7_slot = var_lod_half_ref_dn7;
        *var_lod_half_ref_dn8_slot = var_lod_half_ref_dn8;
        *var_lod_half_ref_dn9_slot = var_lod_half_ref_dn9;
        *var_lod_half_ref_rv_slot = var_lod_half_ref_rv;
        *var_lod_half_rv_slot = var_lod_half_rv;
        *var_mks_nsubcdfm_slot = var_mks_nsubcdfm;
        *var_mks_nsubcdfm_rv_slot = var_mks_nsubcdfm_rv;
        *var_mks_subld2_slot = var_mks_subld2;
        *var_mks_subld2_rv_slot = var_mks_subld2_rv;
        *var_msc_slot = var_msc;
        *var_msc_rv_slot = var_msc_rv;
        *var_npexte_slot = var_npexte;
        *var_npexte_dn0_slot = var_npexte_dn0;
        *var_npexte_dn10_slot = var_npexte_dn10;
        *var_npexte_dn13_slot = var_npexte_dn13;
        *var_npexte_dn2_slot = var_npexte_dn2;
        *var_npexte_dn4_slot = var_npexte_dn4;
        *var_npexte_dn5_slot = var_npexte_dn5;
        *var_npexte_dn6_slot = var_npexte_dn6;
        *var_npexte_dn7_slot = var_npexte_dn7;
        *var_npexte_dn8_slot = var_npexte_dn8;
        *var_npexte_dn9_slot = var_npexte_dn9;
        *var_npexte_rv_slot = var_npexte_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_uc_depvmax_slot = var_uc_depvmax;
        *var_uc_depvmax_dn0_slot = var_uc_depvmax_dn0;
        *var_uc_depvmax_dn10_slot = var_uc_depvmax_dn10;
        *var_uc_depvmax_dn13_slot = var_uc_depvmax_dn13;
        *var_uc_depvmax_dn2_slot = var_uc_depvmax_dn2;
        *var_uc_depvmax_dn4_slot = var_uc_depvmax_dn4;
        *var_uc_depvmax_dn5_slot = var_uc_depvmax_dn5;
        *var_uc_depvmax_dn6_slot = var_uc_depvmax_dn6;
        *var_uc_depvmax_dn7_slot = var_uc_depvmax_dn7;
        *var_uc_depvmax_dn8_slot = var_uc_depvmax_dn8;
        *var_uc_depvmax_dn9_slot = var_uc_depvmax_dn9;
        *var_uc_depvmax_rv_slot = var_uc_depvmax_rv;
        *var_uc_fn2_slot = var_uc_fn2;
        *var_uc_fn2_rv_slot = var_uc_fn2_rv;
        *var_uc_gdld_slot = var_uc_gdld;
        *var_uc_gdld_rv_slot = var_uc_gdld_rv;
        *var_uc_gidl1_slot = var_uc_gidl1;
        *var_uc_gidl1_rv_slot = var_uc_gidl1_rv;
        *var_uc_gidl2_slot = var_uc_gidl2;
        *var_uc_gidl2_rv_slot = var_uc_gidl2_rv;
        *var_uc_gleak2_slot = var_uc_gleak2;
        *var_uc_gleak2_rv_slot = var_uc_gleak2_rv;
        *var_uc_glkb2_slot = var_uc_glkb2;
        *var_uc_glkb2_rv_slot = var_uc_glkb2_rv;
        *var_uc_glksd1_slot = var_uc_glksd1;
        *var_uc_glksd1_rv_slot = var_uc_glksd1_rv;
        *var_uc_glksd2_slot = var_uc_glksd2;
        *var_uc_glksd2_rv_slot = var_uc_glksd2_rv;
        *var_uc_muesti1_slot = var_uc_muesti1;
        *var_uc_muesti1_rv_slot = var_uc_muesti1_rv;
        *var_uc_nfalp_slot = var_uc_nfalp;
        *var_uc_nfalp_rv_slot = var_uc_nfalp_rv;
        *var_uc_nover_slot = var_uc_nover;
        *var_uc_nover_rv_slot = var_uc_nover_rv;
        *var_uc_novers_slot = var_uc_novers;
        *var_uc_novers_rv_slot = var_uc_novers_rv;
        *var_uc_npext_slot = var_uc_npext;
        *var_uc_npext_rv_slot = var_uc_npext_rv;
        *var_uc_nsti_slot = var_uc_nsti;
        *var_uc_nsti_rv_slot = var_uc_nsti_rv;
        *var_uc_nsubc_slot = var_uc_nsubc;
        *var_uc_nsubc_rv_slot = var_uc_nsubc_rv;
        *var_uc_nsubp_slot = var_uc_nsubp;
        *var_uc_nsubp_rv_slot = var_uc_nsubp_rv;
        *var_uc_nsubpsti1_slot = var_uc_nsubpsti1;
        *var_uc_nsubpsti1_rv_slot = var_uc_nsubpsti1_rv;
        *var_uc_rd22_slot = var_uc_rd22;
        *var_uc_rd22_rv_slot = var_uc_rd22_rv;
        *var_uc_rd23_slot = var_uc_rd23;
        *var_uc_rd23_rv_slot = var_uc_rd23_rv;
        *var_uc_rd24_slot = var_uc_rd24;
        *var_uc_rd24_rv_slot = var_uc_rd24_rv;
        *var_uc_rdvd_slot = var_uc_rdvd;
        *var_uc_rdvd_rv_slot = var_uc_rdvd_rv;
        *var_uc_rth0_slot = var_uc_rth0;
        *var_uc_rth0_rv_slot = var_uc_rth0_rv;
        *var_uc_vfbover_slot = var_uc_vfbover;
        *var_uc_vfbover_rv_slot = var_uc_vfbover_rv;
        *var_uc_vmax_slot = var_uc_vmax;
        *var_uc_vmax_rv_slot = var_uc_vmax_rv;
        *var_uc_wfc_slot = var_uc_wfc;
        *var_uc_wfc_rv_slot = var_uc_wfc_rv;
        *var_wlg_slot = var_wlg;
        *var_wlg_rv_slot = var_wlg_rv;
    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        var_guard253: f64,
        var_lg: f64,
        var_lgate: f64,
        var_lod_half: f64,
        var_lod_half_dn0: f64,
        var_lod_half_dn10: f64,
        var_lod_half_dn13: f64,
        var_lod_half_dn2: f64,
        var_lod_half_dn4: f64,
        var_lod_half_dn5: f64,
        var_lod_half_dn6: f64,
        var_lod_half_dn7: f64,
        var_lod_half_dn8: f64,
        var_lod_half_dn9: f64,
        var_lod_half_ref: f64,
        var_lod_half_ref_dn0: f64,
        var_lod_half_ref_dn10: f64,
        var_lod_half_ref_dn13: f64,
        var_lod_half_ref_dn2: f64,
        var_lod_half_ref_dn4: f64,
        var_lod_half_ref_dn5: f64,
        var_lod_half_ref_dn6: f64,
        var_lod_half_ref_dn7: f64,
        var_lod_half_ref_dn8: f64,
        var_lod_half_ref_dn9: f64,
        var_mks_ll: f64,
        var_mks_nsubcdfm: f64,
        var_mks_wl: f64,
        var_uc_muesti1: f64,
        var_uc_muesti2: f64,
        var_uc_muesti3: f64,
        var_uc_ndep: f64,
        var_uc_ninv: f64,
        var_uc_wl2: f64,
        var_uc_xldld: f64,
        var_wg: f64,
        var_wgate: f64,
        var_wlg: f64,
        var_dl_slot: &mut f64,
        var_dl_rv_slot: &mut f64,
        var_dlld_slot: &mut f64,
        var_dlld_rv_slot: &mut f64,
        var_dvthsm_slot: &mut f64,
        var_dvthsm_rv_slot: &mut f64,
        var_dw_slot: &mut f64,
        var_dw_rv_slot: &mut f64,
        var_dwcv_slot: &mut f64,
        var_dwcv_rv_slot: &mut f64,
        var_dwld_slot: &mut f64,
        var_dwld_rv_slot: &mut f64,
        var_ef_mueph1_slot: &mut f64,
        var_ef_mueph1_dn0_slot: &mut f64,
        var_ef_mueph1_dn10_slot: &mut f64,
        var_ef_mueph1_dn13_slot: &mut f64,
        var_ef_mueph1_dn2_slot: &mut f64,
        var_ef_mueph1_dn4_slot: &mut f64,
        var_ef_mueph1_dn5_slot: &mut f64,
        var_ef_mueph1_dn6_slot: &mut f64,
        var_ef_mueph1_dn7_slot: &mut f64,
        var_ef_mueph1_dn8_slot: &mut f64,
        var_ef_mueph1_dn9_slot: &mut f64,
        var_ef_mueph1_rv_slot: &mut f64,
        var_ef_nsubc_slot: &mut f64,
        var_ef_nsubc_dn0_slot: &mut f64,
        var_ef_nsubc_dn10_slot: &mut f64,
        var_ef_nsubc_dn13_slot: &mut f64,
        var_ef_nsubc_dn2_slot: &mut f64,
        var_ef_nsubc_dn4_slot: &mut f64,
        var_ef_nsubc_dn5_slot: &mut f64,
        var_ef_nsubc_dn6_slot: &mut f64,
        var_ef_nsubc_dn7_slot: &mut f64,
        var_ef_nsubc_dn8_slot: &mut f64,
        var_ef_nsubc_dn9_slot: &mut f64,
        var_ef_nsubc_rv_slot: &mut f64,
        var_ef_nsubp_slot: &mut f64,
        var_ef_nsubp_dn0_slot: &mut f64,
        var_ef_nsubp_dn10_slot: &mut f64,
        var_ef_nsubp_dn13_slot: &mut f64,
        var_ef_nsubp_dn2_slot: &mut f64,
        var_ef_nsubp_dn4_slot: &mut f64,
        var_ef_nsubp_dn5_slot: &mut f64,
        var_ef_nsubp_dn6_slot: &mut f64,
        var_ef_nsubp_dn7_slot: &mut f64,
        var_ef_nsubp_dn8_slot: &mut f64,
        var_ef_nsubp_dn9_slot: &mut f64,
        var_ef_nsubp_rv_slot: &mut f64,
        var_guard255_slot: &mut f64,
        var_guard255_rv_slot: &mut f64,
        var_leff_slot: &mut f64,
        var_leff_rv_slot: &mut f64,
        var_lgatesm_slot: &mut f64,
        var_lgatesm_rv_slot: &mut f64,
        var_mueph_slot: &mut f64,
        var_mueph_dn0_slot: &mut f64,
        var_mueph_dn10_slot: &mut f64,
        var_mueph_dn13_slot: &mut f64,
        var_mueph_dn2_slot: &mut f64,
        var_mueph_dn4_slot: &mut f64,
        var_mueph_dn5_slot: &mut f64,
        var_mueph_dn6_slot: &mut f64,
        var_mueph_dn7_slot: &mut f64,
        var_mueph_dn8_slot: &mut f64,
        var_mueph_dn9_slot: &mut f64,
        var_mueph_rv_slot: &mut f64,
        var_muesr_slot: &mut f64,
        var_muesr_rv_slot: &mut f64,
        var_ndep_o_esi_slot: &mut f64,
        var_ndep_o_esi_dn0_slot: &mut f64,
        var_ndep_o_esi_dn10_slot: &mut f64,
        var_ndep_o_esi_dn13_slot: &mut f64,
        var_ndep_o_esi_dn2_slot: &mut f64,
        var_ndep_o_esi_dn4_slot: &mut f64,
        var_ndep_o_esi_dn5_slot: &mut f64,
        var_ndep_o_esi_dn6_slot: &mut f64,
        var_ndep_o_esi_dn7_slot: &mut f64,
        var_ndep_o_esi_dn8_slot: &mut f64,
        var_ndep_o_esi_dn9_slot: &mut f64,
        var_ndep_o_esi_rv_slot: &mut f64,
        var_ninv_o_esi_slot: &mut f64,
        var_ninv_o_esi_rv_slot: &mut f64,
        var_ninvd0_slot: &mut f64,
        var_ninvd0_rv_slot: &mut f64,
        var_ninvd0cres_slot: &mut f64,
        var_ninvd0cres_dn0_slot: &mut f64,
        var_ninvd0cres_dn10_slot: &mut f64,
        var_ninvd0cres_dn13_slot: &mut f64,
        var_ninvd0cres_dn2_slot: &mut f64,
        var_ninvd0cres_dn4_slot: &mut f64,
        var_ninvd0cres_dn5_slot: &mut f64,
        var_ninvd0cres_dn6_slot: &mut f64,
        var_ninvd0cres_dn7_slot: &mut f64,
        var_ninvd0cres_dn8_slot: &mut f64,
        var_ninvd0cres_dn9_slot: &mut f64,
        var_ninvd0cres_rv_slot: &mut f64,
        var_ninvd0hres_slot: &mut f64,
        var_ninvd0hres_dn0_slot: &mut f64,
        var_ninvd0hres_dn10_slot: &mut f64,
        var_ninvd0hres_dn13_slot: &mut f64,
        var_ninvd0hres_dn2_slot: &mut f64,
        var_ninvd0hres_dn4_slot: &mut f64,
        var_ninvd0hres_dn5_slot: &mut f64,
        var_ninvd0hres_dn6_slot: &mut f64,
        var_ninvd0hres_dn7_slot: &mut f64,
        var_ninvd0hres_dn8_slot: &mut f64,
        var_ninvd0hres_dn9_slot: &mut f64,
        var_ninvd0hres_rv_slot: &mut f64,
        var_npexte_slot: &mut f64,
        var_npexte_dn0_slot: &mut f64,
        var_npexte_dn10_slot: &mut f64,
        var_npexte_dn13_slot: &mut f64,
        var_npexte_dn2_slot: &mut f64,
        var_npexte_dn4_slot: &mut f64,
        var_npexte_dn5_slot: &mut f64,
        var_npexte_dn6_slot: &mut f64,
        var_npexte_dn7_slot: &mut f64,
        var_npexte_dn8_slot: &mut f64,
        var_npexte_dn9_slot: &mut f64,
        var_npexte_rv_slot: &mut f64,
        var_nsubpp_slot: &mut f64,
        var_nsubpp_dn0_slot: &mut f64,
        var_nsubpp_dn10_slot: &mut f64,
        var_nsubpp_dn13_slot: &mut f64,
        var_nsubpp_dn2_slot: &mut f64,
        var_nsubpp_dn4_slot: &mut f64,
        var_nsubpp_dn5_slot: &mut f64,
        var_nsubpp_dn6_slot: &mut f64,
        var_nsubpp_dn7_slot: &mut f64,
        var_nsubpp_dn8_slot: &mut f64,
        var_nsubpp_dn9_slot: &mut f64,
        var_nsubpp_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_uc_wsti_slot: &mut f64,
        var_uc_wsti_dn0_slot: &mut f64,
        var_uc_wsti_dn10_slot: &mut f64,
        var_uc_wsti_dn13_slot: &mut f64,
        var_uc_wsti_dn2_slot: &mut f64,
        var_uc_wsti_dn4_slot: &mut f64,
        var_uc_wsti_dn5_slot: &mut f64,
        var_uc_wsti_dn6_slot: &mut f64,
        var_uc_wsti_dn7_slot: &mut f64,
        var_uc_wsti_dn8_slot: &mut f64,
        var_uc_wsti_dn9_slot: &mut f64,
        var_uc_wsti_rv_slot: &mut f64,
        var_weff_slot: &mut f64,
        var_weff_cv_slot: &mut f64,
        var_weff_cv_rv_slot: &mut f64,
        var_weff_ld_slot: &mut f64,
        var_weff_ld_rv_slot: &mut f64,
        var_weff_nf_slot: &mut f64,
        var_weff_nf_rv_slot: &mut f64,
        var_weff_rv_slot: &mut f64,
        var_weffcv_nf_slot: &mut f64,
        var_weffcv_nf_rv_slot: &mut f64,
    ) {
        let mut var_dl: f64 = *var_dl_slot;
        let mut var_dl_rv: f64 = *var_dl_rv_slot;
        let mut var_dlld: f64 = *var_dlld_slot;
        let mut var_dlld_rv: f64 = *var_dlld_rv_slot;
        let mut var_dvthsm: f64 = *var_dvthsm_slot;
        let mut var_dvthsm_rv: f64 = *var_dvthsm_rv_slot;
        let mut var_dw: f64 = *var_dw_slot;
        let mut var_dw_rv: f64 = *var_dw_rv_slot;
        let mut var_dwcv: f64 = *var_dwcv_slot;
        let mut var_dwcv_rv: f64 = *var_dwcv_rv_slot;
        let mut var_dwld: f64 = *var_dwld_slot;
        let mut var_dwld_rv: f64 = *var_dwld_rv_slot;
        let mut var_ef_mueph1: f64 = *var_ef_mueph1_slot;
        let mut var_ef_mueph1_dn0: f64 = *var_ef_mueph1_dn0_slot;
        let mut var_ef_mueph1_dn10: f64 = *var_ef_mueph1_dn10_slot;
        let mut var_ef_mueph1_dn13: f64 = *var_ef_mueph1_dn13_slot;
        let mut var_ef_mueph1_dn2: f64 = *var_ef_mueph1_dn2_slot;
        let mut var_ef_mueph1_dn4: f64 = *var_ef_mueph1_dn4_slot;
        let mut var_ef_mueph1_dn5: f64 = *var_ef_mueph1_dn5_slot;
        let mut var_ef_mueph1_dn6: f64 = *var_ef_mueph1_dn6_slot;
        let mut var_ef_mueph1_dn7: f64 = *var_ef_mueph1_dn7_slot;
        let mut var_ef_mueph1_dn8: f64 = *var_ef_mueph1_dn8_slot;
        let mut var_ef_mueph1_dn9: f64 = *var_ef_mueph1_dn9_slot;
        let mut var_ef_mueph1_rv: f64 = *var_ef_mueph1_rv_slot;
        let mut var_ef_nsubc: f64 = *var_ef_nsubc_slot;
        let mut var_ef_nsubc_dn0: f64 = *var_ef_nsubc_dn0_slot;
        let mut var_ef_nsubc_dn10: f64 = *var_ef_nsubc_dn10_slot;
        let mut var_ef_nsubc_dn13: f64 = *var_ef_nsubc_dn13_slot;
        let mut var_ef_nsubc_dn2: f64 = *var_ef_nsubc_dn2_slot;
        let mut var_ef_nsubc_dn4: f64 = *var_ef_nsubc_dn4_slot;
        let mut var_ef_nsubc_dn5: f64 = *var_ef_nsubc_dn5_slot;
        let mut var_ef_nsubc_dn6: f64 = *var_ef_nsubc_dn6_slot;
        let mut var_ef_nsubc_dn7: f64 = *var_ef_nsubc_dn7_slot;
        let mut var_ef_nsubc_dn8: f64 = *var_ef_nsubc_dn8_slot;
        let mut var_ef_nsubc_dn9: f64 = *var_ef_nsubc_dn9_slot;
        let mut var_ef_nsubc_rv: f64 = *var_ef_nsubc_rv_slot;
        let mut var_ef_nsubp: f64 = *var_ef_nsubp_slot;
        let mut var_ef_nsubp_dn0: f64 = *var_ef_nsubp_dn0_slot;
        let mut var_ef_nsubp_dn10: f64 = *var_ef_nsubp_dn10_slot;
        let mut var_ef_nsubp_dn13: f64 = *var_ef_nsubp_dn13_slot;
        let mut var_ef_nsubp_dn2: f64 = *var_ef_nsubp_dn2_slot;
        let mut var_ef_nsubp_dn4: f64 = *var_ef_nsubp_dn4_slot;
        let mut var_ef_nsubp_dn5: f64 = *var_ef_nsubp_dn5_slot;
        let mut var_ef_nsubp_dn6: f64 = *var_ef_nsubp_dn6_slot;
        let mut var_ef_nsubp_dn7: f64 = *var_ef_nsubp_dn7_slot;
        let mut var_ef_nsubp_dn8: f64 = *var_ef_nsubp_dn8_slot;
        let mut var_ef_nsubp_dn9: f64 = *var_ef_nsubp_dn9_slot;
        let mut var_ef_nsubp_rv: f64 = *var_ef_nsubp_rv_slot;
        let mut var_guard255: f64 = *var_guard255_slot;
        let mut var_guard255_rv: f64 = *var_guard255_rv_slot;
        let mut var_leff: f64 = *var_leff_slot;
        let mut var_leff_rv: f64 = *var_leff_rv_slot;
        let mut var_lgatesm: f64 = *var_lgatesm_slot;
        let mut var_lgatesm_rv: f64 = *var_lgatesm_rv_slot;
        let mut var_mueph: f64 = *var_mueph_slot;
        let mut var_mueph_dn0: f64 = *var_mueph_dn0_slot;
        let mut var_mueph_dn10: f64 = *var_mueph_dn10_slot;
        let mut var_mueph_dn13: f64 = *var_mueph_dn13_slot;
        let mut var_mueph_dn2: f64 = *var_mueph_dn2_slot;
        let mut var_mueph_dn4: f64 = *var_mueph_dn4_slot;
        let mut var_mueph_dn5: f64 = *var_mueph_dn5_slot;
        let mut var_mueph_dn6: f64 = *var_mueph_dn6_slot;
        let mut var_mueph_dn7: f64 = *var_mueph_dn7_slot;
        let mut var_mueph_dn8: f64 = *var_mueph_dn8_slot;
        let mut var_mueph_dn9: f64 = *var_mueph_dn9_slot;
        let mut var_mueph_rv: f64 = *var_mueph_rv_slot;
        let mut var_muesr: f64 = *var_muesr_slot;
        let mut var_muesr_rv: f64 = *var_muesr_rv_slot;
        let mut var_ndep_o_esi: f64 = *var_ndep_o_esi_slot;
        let mut var_ndep_o_esi_dn0: f64 = *var_ndep_o_esi_dn0_slot;
        let mut var_ndep_o_esi_dn10: f64 = *var_ndep_o_esi_dn10_slot;
        let mut var_ndep_o_esi_dn13: f64 = *var_ndep_o_esi_dn13_slot;
        let mut var_ndep_o_esi_dn2: f64 = *var_ndep_o_esi_dn2_slot;
        let mut var_ndep_o_esi_dn4: f64 = *var_ndep_o_esi_dn4_slot;
        let mut var_ndep_o_esi_dn5: f64 = *var_ndep_o_esi_dn5_slot;
        let mut var_ndep_o_esi_dn6: f64 = *var_ndep_o_esi_dn6_slot;
        let mut var_ndep_o_esi_dn7: f64 = *var_ndep_o_esi_dn7_slot;
        let mut var_ndep_o_esi_dn8: f64 = *var_ndep_o_esi_dn8_slot;
        let mut var_ndep_o_esi_dn9: f64 = *var_ndep_o_esi_dn9_slot;
        let mut var_ndep_o_esi_rv: f64 = *var_ndep_o_esi_rv_slot;
        let mut var_ninv_o_esi: f64 = *var_ninv_o_esi_slot;
        let mut var_ninv_o_esi_rv: f64 = *var_ninv_o_esi_rv_slot;
        let mut var_ninvd0: f64 = *var_ninvd0_slot;
        let mut var_ninvd0_rv: f64 = *var_ninvd0_rv_slot;
        let mut var_ninvd0cres: f64 = *var_ninvd0cres_slot;
        let mut var_ninvd0cres_dn0: f64 = *var_ninvd0cres_dn0_slot;
        let mut var_ninvd0cres_dn10: f64 = *var_ninvd0cres_dn10_slot;
        let mut var_ninvd0cres_dn13: f64 = *var_ninvd0cres_dn13_slot;
        let mut var_ninvd0cres_dn2: f64 = *var_ninvd0cres_dn2_slot;
        let mut var_ninvd0cres_dn4: f64 = *var_ninvd0cres_dn4_slot;
        let mut var_ninvd0cres_dn5: f64 = *var_ninvd0cres_dn5_slot;
        let mut var_ninvd0cres_dn6: f64 = *var_ninvd0cres_dn6_slot;
        let mut var_ninvd0cres_dn7: f64 = *var_ninvd0cres_dn7_slot;
        let mut var_ninvd0cres_dn8: f64 = *var_ninvd0cres_dn8_slot;
        let mut var_ninvd0cres_dn9: f64 = *var_ninvd0cres_dn9_slot;
        let mut var_ninvd0cres_rv: f64 = *var_ninvd0cres_rv_slot;
        let mut var_ninvd0hres: f64 = *var_ninvd0hres_slot;
        let mut var_ninvd0hres_dn0: f64 = *var_ninvd0hres_dn0_slot;
        let mut var_ninvd0hres_dn10: f64 = *var_ninvd0hres_dn10_slot;
        let mut var_ninvd0hres_dn13: f64 = *var_ninvd0hres_dn13_slot;
        let mut var_ninvd0hres_dn2: f64 = *var_ninvd0hres_dn2_slot;
        let mut var_ninvd0hres_dn4: f64 = *var_ninvd0hres_dn4_slot;
        let mut var_ninvd0hres_dn5: f64 = *var_ninvd0hres_dn5_slot;
        let mut var_ninvd0hres_dn6: f64 = *var_ninvd0hres_dn6_slot;
        let mut var_ninvd0hres_dn7: f64 = *var_ninvd0hres_dn7_slot;
        let mut var_ninvd0hres_dn8: f64 = *var_ninvd0hres_dn8_slot;
        let mut var_ninvd0hres_dn9: f64 = *var_ninvd0hres_dn9_slot;
        let mut var_ninvd0hres_rv: f64 = *var_ninvd0hres_rv_slot;
        let mut var_npexte: f64 = *var_npexte_slot;
        let mut var_npexte_dn0: f64 = *var_npexte_dn0_slot;
        let mut var_npexte_dn10: f64 = *var_npexte_dn10_slot;
        let mut var_npexte_dn13: f64 = *var_npexte_dn13_slot;
        let mut var_npexte_dn2: f64 = *var_npexte_dn2_slot;
        let mut var_npexte_dn4: f64 = *var_npexte_dn4_slot;
        let mut var_npexte_dn5: f64 = *var_npexte_dn5_slot;
        let mut var_npexte_dn6: f64 = *var_npexte_dn6_slot;
        let mut var_npexte_dn7: f64 = *var_npexte_dn7_slot;
        let mut var_npexte_dn8: f64 = *var_npexte_dn8_slot;
        let mut var_npexte_dn9: f64 = *var_npexte_dn9_slot;
        let mut var_npexte_rv: f64 = *var_npexte_rv_slot;
        let mut var_nsubpp: f64 = *var_nsubpp_slot;
        let mut var_nsubpp_dn0: f64 = *var_nsubpp_dn0_slot;
        let mut var_nsubpp_dn10: f64 = *var_nsubpp_dn10_slot;
        let mut var_nsubpp_dn13: f64 = *var_nsubpp_dn13_slot;
        let mut var_nsubpp_dn2: f64 = *var_nsubpp_dn2_slot;
        let mut var_nsubpp_dn4: f64 = *var_nsubpp_dn4_slot;
        let mut var_nsubpp_dn5: f64 = *var_nsubpp_dn5_slot;
        let mut var_nsubpp_dn6: f64 = *var_nsubpp_dn6_slot;
        let mut var_nsubpp_dn7: f64 = *var_nsubpp_dn7_slot;
        let mut var_nsubpp_dn8: f64 = *var_nsubpp_dn8_slot;
        let mut var_nsubpp_dn9: f64 = *var_nsubpp_dn9_slot;
        let mut var_nsubpp_rv: f64 = *var_nsubpp_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_uc_wsti: f64 = *var_uc_wsti_slot;
        let mut var_uc_wsti_dn0: f64 = *var_uc_wsti_dn0_slot;
        let mut var_uc_wsti_dn10: f64 = *var_uc_wsti_dn10_slot;
        let mut var_uc_wsti_dn13: f64 = *var_uc_wsti_dn13_slot;
        let mut var_uc_wsti_dn2: f64 = *var_uc_wsti_dn2_slot;
        let mut var_uc_wsti_dn4: f64 = *var_uc_wsti_dn4_slot;
        let mut var_uc_wsti_dn5: f64 = *var_uc_wsti_dn5_slot;
        let mut var_uc_wsti_dn6: f64 = *var_uc_wsti_dn6_slot;
        let mut var_uc_wsti_dn7: f64 = *var_uc_wsti_dn7_slot;
        let mut var_uc_wsti_dn8: f64 = *var_uc_wsti_dn8_slot;
        let mut var_uc_wsti_dn9: f64 = *var_uc_wsti_dn9_slot;
        let mut var_uc_wsti_rv: f64 = *var_uc_wsti_rv_slot;
        let mut var_weff: f64 = *var_weff_slot;
        let mut var_weff_cv: f64 = *var_weff_cv_slot;
        let mut var_weff_cv_rv: f64 = *var_weff_cv_rv_slot;
        let mut var_weff_ld: f64 = *var_weff_ld_slot;
        let mut var_weff_ld_rv: f64 = *var_weff_ld_rv_slot;
        let mut var_weff_nf: f64 = *var_weff_nf_slot;
        let mut var_weff_nf_rv: f64 = *var_weff_nf_rv_slot;
        let mut var_weff_rv: f64 = *var_weff_rv_slot;
        let mut var_weffcv_nf: f64 = *var_weffcv_nf_slot;
        let mut var_weffcv_nf_rv: f64 = *var_weffcv_nf_rv_slot;

        let (assign10870_e5792, assign10870_e5792_d_n0, assign10870_e5792_d_n2, assign10870_e5792_d_n4, assign10870_e5792_d_n5, assign10870_e5792_d_n6, assign10870_e5792_d_n7, assign10870_e5792_d_n8, assign10870_e5792_d_n9, assign10870_e5792_d_n10, assign10870_e5792_d_n13,) = {
    if (var_guard253 != 0.0) {
        let assign10870_e5783: f64 = (var_mks_nsubcdfm).ln();
        let assign10870_e5785: f64 = (var_ef_nsubc).ln();
        let assign10870_e5786: f64 = (assign10870_e5783 - assign10870_e5785);
        let assign10870_e5787: f64 = (p.p282 * assign10870_e5786);
        let assign10870_e5789: f64 = (assign10870_e5787 + 1.0);
        let assign10870_e5790: f64 = (var_ef_mueph1 * assign10870_e5789);
        (assign10870_e5790, ((var_ef_mueph1_dn0 * assign10870_e5789) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn0 / var_ef_nsubc))))), ((var_ef_mueph1_dn2 * assign10870_e5789) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn2 / var_ef_nsubc))))), ((var_ef_mueph1_dn4 * assign10870_e5789) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn4 / var_ef_nsubc))))), ((var_ef_mueph1_dn5 * assign10870_e5789) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn5 / var_ef_nsubc))))), ((var_ef_mueph1_dn6 * assign10870_e5789) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn6 / var_ef_nsubc))))), ((var_ef_mueph1_dn7 * assign10870_e5789) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn7 / var_ef_nsubc))))), ((var_ef_mueph1_dn8 * assign10870_e5789) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn8 / var_ef_nsubc))))), ((var_ef_mueph1_dn9 * assign10870_e5789) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn9 / var_ef_nsubc))))), ((var_ef_mueph1_dn10 * assign10870_e5789) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn10 / var_ef_nsubc))))), ((var_ef_mueph1_dn13 * assign10870_e5789) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn13 / var_ef_nsubc))))),)
    } else {
        (var_ef_mueph1, var_ef_mueph1_dn0, var_ef_mueph1_dn2, var_ef_mueph1_dn4, var_ef_mueph1_dn5, var_ef_mueph1_dn6, var_ef_mueph1_dn7, var_ef_mueph1_dn8, var_ef_mueph1_dn9, var_ef_mueph1_dn10, var_ef_mueph1_dn13,)
    }
};
        var_ef_mueph1 = assign10870_e5792;
        var_ef_mueph1_dn0 = assign10870_e5792_d_n0;
        var_ef_mueph1_dn2 = assign10870_e5792_d_n2;
        var_ef_mueph1_dn4 = assign10870_e5792_d_n4;
        var_ef_mueph1_dn5 = assign10870_e5792_d_n5;
        var_ef_mueph1_dn6 = assign10870_e5792_d_n6;
        var_ef_mueph1_dn7 = assign10870_e5792_d_n7;
        var_ef_mueph1_dn8 = assign10870_e5792_d_n8;
        var_ef_mueph1_dn9 = assign10870_e5792_d_n9;
        var_ef_mueph1_dn10 = assign10870_e5792_d_n10;
        var_ef_mueph1_dn13 = assign10870_e5792_d_n13;
        var_ef_mueph1_rv = 0.0;

        let (assign10880_e5800, assign10880_e5800_d_n0, assign10880_e5800_d_n2, assign10880_e5800_d_n4, assign10880_e5800_d_n5, assign10880_e5800_d_n6, assign10880_e5800_d_n7, assign10880_e5800_d_n8, assign10880_e5800_d_n9, assign10880_e5800_d_n10, assign10880_e5800_d_n13,) = {
    if (var_guard253 != 0.0) {
        let assign10880_e5796: f64 = (var_ef_nsubp + var_mks_nsubcdfm);
        let assign10880_e5798: f64 = (assign10880_e5796 - var_ef_nsubc);
        (assign10880_e5798, (var_ef_nsubp_dn0 - var_ef_nsubc_dn0), (var_ef_nsubp_dn2 - var_ef_nsubc_dn2), (var_ef_nsubp_dn4 - var_ef_nsubc_dn4), (var_ef_nsubp_dn5 - var_ef_nsubc_dn5), (var_ef_nsubp_dn6 - var_ef_nsubc_dn6), (var_ef_nsubp_dn7 - var_ef_nsubc_dn7), (var_ef_nsubp_dn8 - var_ef_nsubc_dn8), (var_ef_nsubp_dn9 - var_ef_nsubc_dn9), (var_ef_nsubp_dn10 - var_ef_nsubc_dn10), (var_ef_nsubp_dn13 - var_ef_nsubc_dn13),)
    } else {
        (var_ef_nsubp, var_ef_nsubp_dn0, var_ef_nsubp_dn2, var_ef_nsubp_dn4, var_ef_nsubp_dn5, var_ef_nsubp_dn6, var_ef_nsubp_dn7, var_ef_nsubp_dn8, var_ef_nsubp_dn9, var_ef_nsubp_dn10, var_ef_nsubp_dn13,)
    }
};
        var_ef_nsubp = assign10880_e5800;
        var_ef_nsubp_dn0 = assign10880_e5800_d_n0;
        var_ef_nsubp_dn2 = assign10880_e5800_d_n2;
        var_ef_nsubp_dn4 = assign10880_e5800_d_n4;
        var_ef_nsubp_dn5 = assign10880_e5800_d_n5;
        var_ef_nsubp_dn6 = assign10880_e5800_d_n6;
        var_ef_nsubp_dn7 = assign10880_e5800_d_n7;
        var_ef_nsubp_dn8 = assign10880_e5800_d_n8;
        var_ef_nsubp_dn9 = assign10880_e5800_d_n9;
        var_ef_nsubp_dn10 = assign10880_e5800_d_n10;
        var_ef_nsubp_dn13 = assign10880_e5800_d_n13;
        var_ef_nsubp_rv = 0.0;

        let (assign10890_e5808, assign10890_e5808_d_n0, assign10890_e5808_d_n2, assign10890_e5808_d_n4, assign10890_e5808_d_n5, assign10890_e5808_d_n6, assign10890_e5808_d_n7, assign10890_e5808_d_n8, assign10890_e5808_d_n9, assign10890_e5808_d_n10, assign10890_e5808_d_n13,) = {
    if (var_guard253 != 0.0) {
        let assign10890_e5804: f64 = (var_npexte + var_mks_nsubcdfm);
        let assign10890_e5806: f64 = (assign10890_e5804 - var_ef_nsubc);
        (assign10890_e5806, (var_npexte_dn0 - var_ef_nsubc_dn0), (var_npexte_dn2 - var_ef_nsubc_dn2), (var_npexte_dn4 - var_ef_nsubc_dn4), (var_npexte_dn5 - var_ef_nsubc_dn5), (var_npexte_dn6 - var_ef_nsubc_dn6), (var_npexte_dn7 - var_ef_nsubc_dn7), (var_npexte_dn8 - var_ef_nsubc_dn8), (var_npexte_dn9 - var_ef_nsubc_dn9), (var_npexte_dn10 - var_ef_nsubc_dn10), (var_npexte_dn13 - var_ef_nsubc_dn13),)
    } else {
        (var_npexte, var_npexte_dn0, var_npexte_dn2, var_npexte_dn4, var_npexte_dn5, var_npexte_dn6, var_npexte_dn7, var_npexte_dn8, var_npexte_dn9, var_npexte_dn10, var_npexte_dn13,)
    }
};
        var_npexte = assign10890_e5808;
        var_npexte_dn0 = assign10890_e5808_d_n0;
        var_npexte_dn2 = assign10890_e5808_d_n2;
        var_npexte_dn4 = assign10890_e5808_d_n4;
        var_npexte_dn5 = assign10890_e5808_d_n5;
        var_npexte_dn6 = assign10890_e5808_d_n6;
        var_npexte_dn7 = assign10890_e5808_d_n7;
        var_npexte_dn8 = assign10890_e5808_d_n8;
        var_npexte_dn9 = assign10890_e5808_d_n9;
        var_npexte_dn10 = assign10890_e5808_d_n10;
        var_npexte_dn13 = assign10890_e5808_d_n13;
        var_npexte_rv = 0.0;

        let (assign10900_e5812, assign10900_e5812_d_n0, assign10900_e5812_d_n2, assign10900_e5812_d_n4, assign10900_e5812_d_n5, assign10900_e5812_d_n6, assign10900_e5812_d_n7, assign10900_e5812_d_n8, assign10900_e5812_d_n9, assign10900_e5812_d_n10, assign10900_e5812_d_n13,) = {
    if (var_guard253 != 0.0) {
        (var_mks_nsubcdfm, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ef_nsubc, var_ef_nsubc_dn0, var_ef_nsubc_dn2, var_ef_nsubc_dn4, var_ef_nsubc_dn5, var_ef_nsubc_dn6, var_ef_nsubc_dn7, var_ef_nsubc_dn8, var_ef_nsubc_dn9, var_ef_nsubc_dn10, var_ef_nsubc_dn13,)
    }
};
        var_ef_nsubc = assign10900_e5812;
        var_ef_nsubc_dn0 = assign10900_e5812_d_n0;
        var_ef_nsubc_dn2 = assign10900_e5812_d_n2;
        var_ef_nsubc_dn4 = assign10900_e5812_d_n4;
        var_ef_nsubc_dn5 = assign10900_e5812_d_n5;
        var_ef_nsubc_dn6 = assign10900_e5812_d_n6;
        var_ef_nsubc_dn7 = assign10900_e5812_d_n7;
        var_ef_nsubc_dn8 = assign10900_e5812_d_n8;
        var_ef_nsubc_dn9 = assign10900_e5812_d_n9;
        var_ef_nsubc_dn10 = assign10900_e5812_d_n10;
        var_ef_nsubc_dn13 = assign10900_e5812_d_n13;
        var_ef_nsubc_rv = 0.0;

        let assign10910_e5818: f64 = (var_wg).powf(p.p163);
        let assign10910_e5819: f64 = (p.p162 / assign10910_e5818);
        let assign10910_e5820: f64 = (1.0 + assign10910_e5819);
        let assign10910_e5821: f64 = (var_ef_mueph1 * assign10910_e5820);
        let assign10910_e5826: f64 = (var_lg).powf(p.p165);
        let assign10910_e5827: f64 = (p.p164 / assign10910_e5826);
        let assign10910_e5828: f64 = (1.0 + assign10910_e5827);
        let assign10910_e5829: f64 = (assign10910_e5821 * assign10910_e5828);
        let assign10910_e5834: f64 = (var_wlg).powf(p.p168);
        let assign10910_e5835: f64 = (p.p167 / assign10910_e5834);
        let assign10910_e5836: f64 = (1.0 + assign10910_e5835);
        let assign10910_e5837: f64 = (assign10910_e5829 * assign10910_e5836);
        var_mueph = assign10910_e5837;
        var_mueph_dn0 = (((var_ef_mueph1_dn0 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        var_mueph_dn2 = (((var_ef_mueph1_dn2 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        var_mueph_dn4 = (((var_ef_mueph1_dn4 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        var_mueph_dn5 = (((var_ef_mueph1_dn5 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        var_mueph_dn6 = (((var_ef_mueph1_dn6 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        var_mueph_dn7 = (((var_ef_mueph1_dn7 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        var_mueph_dn8 = (((var_ef_mueph1_dn8 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        var_mueph_dn9 = (((var_ef_mueph1_dn9 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        var_mueph_dn10 = (((var_ef_mueph1_dn10 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        var_mueph_dn13 = (((var_ef_mueph1_dn13 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        var_mueph_rv = 0.0;

        let assign10920_e5840: f64 = if var_lod_half > 0.0 { 1.0 } else { 0.0 };
        var_guard255 = assign10920_e5840;
        var_guard255_rv = 0.0;

        let (assign10930_e5848, assign10930_e5848_d_n0, assign10930_e5848_d_n2, assign10930_e5848_d_n4, assign10930_e5848_d_n5, assign10930_e5848_d_n6, assign10930_e5848_d_n7, assign10930_e5848_d_n8, assign10930_e5848_d_n9, assign10930_e5848_d_n10, assign10930_e5848_d_n13,) = {
    if (var_guard255 != 0.0) {
        let assign10930_e5845: f64 = (1.0 + var_uc_muesti2);
        let assign10930_e5846: f64 = (1.0 / assign10930_e5845);
        (assign10930_e5846, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign10930_e5848;
        var_t1_dn0 = assign10930_e5848_d_n0;
        var_t1_dn2 = assign10930_e5848_d_n2;
        var_t1_dn4 = assign10930_e5848_d_n4;
        var_t1_dn5 = assign10930_e5848_d_n5;
        var_t1_dn6 = assign10930_e5848_d_n6;
        var_t1_dn7 = assign10930_e5848_d_n7;
        var_t1_dn8 = assign10930_e5848_d_n8;
        var_t1_dn9 = assign10930_e5848_d_n9;
        var_t1_dn10 = assign10930_e5848_d_n10;
        var_t1_dn13 = assign10930_e5848_d_n13;
        var_t1_rv = 0.0;

        let (assign10940_e5856, assign10940_e5856_d_n0, assign10940_e5856_d_n2, assign10940_e5856_d_n4, assign10940_e5856_d_n5, assign10940_e5856_d_n6, assign10940_e5856_d_n7, assign10940_e5856_d_n8, assign10940_e5856_d_n9, assign10940_e5856_d_n10, assign10940_e5856_d_n13,) = {
    if (var_guard255 != 0.0) {
        let assign10940_e5852: f64 = (var_uc_muesti1 / var_lod_half);
        let assign10940_e5854: f64 = (assign10940_e5852).powf(var_uc_muesti3);
        (assign10940_e5854, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10940_e5852).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn0) / (var_lod_half * var_lod_half))))) } } else { (assign10940_e5854 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn0) / (var_lod_half * var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10940_e5852).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn2) / (var_lod_half * var_lod_half))))) } } else { (assign10940_e5854 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn2) / (var_lod_half * var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10940_e5852).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn4) / (var_lod_half * var_lod_half))))) } } else { (assign10940_e5854 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn4) / (var_lod_half * var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10940_e5852).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn5) / (var_lod_half * var_lod_half))))) } } else { (assign10940_e5854 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn5) / (var_lod_half * var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10940_e5852).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn6) / (var_lod_half * var_lod_half))))) } } else { (assign10940_e5854 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn6) / (var_lod_half * var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10940_e5852).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn7) / (var_lod_half * var_lod_half))))) } } else { (assign10940_e5854 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn7) / (var_lod_half * var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10940_e5852).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn8) / (var_lod_half * var_lod_half))))) } } else { (assign10940_e5854 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn8) / (var_lod_half * var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10940_e5852).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn9) / (var_lod_half * var_lod_half))))) } } else { (assign10940_e5854 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn9) / (var_lod_half * var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10940_e5852).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn10) / (var_lod_half * var_lod_half))))) } } else { (assign10940_e5854 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn10) / (var_lod_half * var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10940_e5852).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn13) / (var_lod_half * var_lod_half))))) } } else { (assign10940_e5854 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn13) / (var_lod_half * var_lod_half))) / assign10940_e5852))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign10940_e5856;
        var_t2_dn0 = assign10940_e5856_d_n0;
        var_t2_dn2 = assign10940_e5856_d_n2;
        var_t2_dn4 = assign10940_e5856_d_n4;
        var_t2_dn5 = assign10940_e5856_d_n5;
        var_t2_dn6 = assign10940_e5856_d_n6;
        var_t2_dn7 = assign10940_e5856_d_n7;
        var_t2_dn8 = assign10940_e5856_d_n8;
        var_t2_dn9 = assign10940_e5856_d_n9;
        var_t2_dn10 = assign10940_e5856_d_n10;
        var_t2_dn13 = assign10940_e5856_d_n13;
        var_t2_rv = 0.0;

        let (assign10950_e5864, assign10950_e5864_d_n0, assign10950_e5864_d_n2, assign10950_e5864_d_n4, assign10950_e5864_d_n5, assign10950_e5864_d_n6, assign10950_e5864_d_n7, assign10950_e5864_d_n8, assign10950_e5864_d_n9, assign10950_e5864_d_n10, assign10950_e5864_d_n13,) = {
    if (var_guard255 != 0.0) {
        let assign10950_e5860: f64 = (var_uc_muesti1 / var_lod_half_ref);
        let assign10950_e5862: f64 = (assign10950_e5860).powf(var_uc_muesti3);
        (assign10950_e5862, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10950_e5860).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn0) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10950_e5862 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn0) / (var_lod_half_ref * var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10950_e5860).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn2) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10950_e5862 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn2) / (var_lod_half_ref * var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10950_e5860).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn4) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10950_e5862 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn4) / (var_lod_half_ref * var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10950_e5860).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn5) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10950_e5862 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn5) / (var_lod_half_ref * var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10950_e5860).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn6) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10950_e5862 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn6) / (var_lod_half_ref * var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10950_e5860).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn7) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10950_e5862 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn7) / (var_lod_half_ref * var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10950_e5860).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn8) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10950_e5862 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn8) / (var_lod_half_ref * var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10950_e5860).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn9) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10950_e5862 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn9) / (var_lod_half_ref * var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10950_e5860).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn10) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10950_e5862 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn10) / (var_lod_half_ref * var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10950_e5860).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn13) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10950_e5862 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn13) / (var_lod_half_ref * var_lod_half_ref))) / assign10950_e5860))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign10950_e5864;
        var_t3_dn0 = assign10950_e5864_d_n0;
        var_t3_dn2 = assign10950_e5864_d_n2;
        var_t3_dn4 = assign10950_e5864_d_n4;
        var_t3_dn5 = assign10950_e5864_d_n5;
        var_t3_dn6 = assign10950_e5864_d_n6;
        var_t3_dn7 = assign10950_e5864_d_n7;
        var_t3_dn8 = assign10950_e5864_d_n8;
        var_t3_dn9 = assign10950_e5864_d_n9;
        var_t3_dn10 = assign10950_e5864_d_n10;
        var_t3_dn13 = assign10950_e5864_d_n13;
        var_t3_rv = 0.0;

        let (assign10960_e5880, assign10960_e5880_d_n0, assign10960_e5880_d_n2, assign10960_e5880_d_n4, assign10960_e5880_d_n5, assign10960_e5880_d_n6, assign10960_e5880_d_n7, assign10960_e5880_d_n8, assign10960_e5880_d_n9, assign10960_e5880_d_n10, assign10960_e5880_d_n13,) = {
    if (var_guard255 != 0.0) {
        let assign10960_e5870: f64 = (var_t1 * var_t2);
        let assign10960_e5871: f64 = (1.0 + assign10960_e5870);
        let assign10960_e5872: f64 = (var_mueph * assign10960_e5871);
        let assign10960_e5876: f64 = (var_t1 * var_t3);
        let assign10960_e5877: f64 = (1.0 + assign10960_e5876);
        let assign10960_e5878: f64 = (assign10960_e5872 / assign10960_e5877);
        (assign10960_e5878, (((((var_mueph_dn0 * assign10960_e5871) + (var_mueph * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0)))) * assign10960_e5877) - (assign10960_e5872 * ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)))) / (assign10960_e5877 * assign10960_e5877)), (((((var_mueph_dn2 * assign10960_e5871) + (var_mueph * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2)))) * assign10960_e5877) - (assign10960_e5872 * ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)))) / (assign10960_e5877 * assign10960_e5877)), (((((var_mueph_dn4 * assign10960_e5871) + (var_mueph * ((var_t1_dn4 * var_t2) + (var_t1 * var_t2_dn4)))) * assign10960_e5877) - (assign10960_e5872 * ((var_t1_dn4 * var_t3) + (var_t1 * var_t3_dn4)))) / (assign10960_e5877 * assign10960_e5877)), (((((var_mueph_dn5 * assign10960_e5871) + (var_mueph * ((var_t1_dn5 * var_t2) + (var_t1 * var_t2_dn5)))) * assign10960_e5877) - (assign10960_e5872 * ((var_t1_dn5 * var_t3) + (var_t1 * var_t3_dn5)))) / (assign10960_e5877 * assign10960_e5877)), (((((var_mueph_dn6 * assign10960_e5871) + (var_mueph * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * assign10960_e5877) - (assign10960_e5872 * ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)))) / (assign10960_e5877 * assign10960_e5877)), (((((var_mueph_dn7 * assign10960_e5871) + (var_mueph * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * assign10960_e5877) - (assign10960_e5872 * ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)))) / (assign10960_e5877 * assign10960_e5877)), (((((var_mueph_dn8 * assign10960_e5871) + (var_mueph * ((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8)))) * assign10960_e5877) - (assign10960_e5872 * ((var_t1_dn8 * var_t3) + (var_t1 * var_t3_dn8)))) / (assign10960_e5877 * assign10960_e5877)), (((((var_mueph_dn9 * assign10960_e5871) + (var_mueph * ((var_t1_dn9 * var_t2) + (var_t1 * var_t2_dn9)))) * assign10960_e5877) - (assign10960_e5872 * ((var_t1_dn9 * var_t3) + (var_t1 * var_t3_dn9)))) / (assign10960_e5877 * assign10960_e5877)), (((((var_mueph_dn10 * assign10960_e5871) + (var_mueph * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10)))) * assign10960_e5877) - (assign10960_e5872 * ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)))) / (assign10960_e5877 * assign10960_e5877)), (((((var_mueph_dn13 * assign10960_e5871) + (var_mueph * ((var_t1_dn13 * var_t2) + (var_t1 * var_t2_dn13)))) * assign10960_e5877) - (assign10960_e5872 * ((var_t1_dn13 * var_t3) + (var_t1 * var_t3_dn13)))) / (assign10960_e5877 * assign10960_e5877)),)
    } else {
        (var_mueph, var_mueph_dn0, var_mueph_dn2, var_mueph_dn4, var_mueph_dn5, var_mueph_dn6, var_mueph_dn7, var_mueph_dn8, var_mueph_dn9, var_mueph_dn10, var_mueph_dn13,)
    }
};
        var_mueph = assign10960_e5880;
        var_mueph_dn0 = assign10960_e5880_d_n0;
        var_mueph_dn2 = assign10960_e5880_d_n2;
        var_mueph_dn4 = assign10960_e5880_d_n4;
        var_mueph_dn5 = assign10960_e5880_d_n5;
        var_mueph_dn6 = assign10960_e5880_d_n6;
        var_mueph_dn7 = assign10960_e5880_d_n7;
        var_mueph_dn8 = assign10960_e5880_d_n8;
        var_mueph_dn9 = assign10960_e5880_d_n9;
        var_mueph_dn10 = assign10960_e5880_d_n10;
        var_mueph_dn13 = assign10960_e5880_d_n13;
        var_mueph_rv = 0.0;

        let assign10970_e5886: f64 = (var_lg).powf(p.p176);
        let assign10970_e5887: f64 = (p.p173 / assign10970_e5886);
        let assign10970_e5888: f64 = (1.0 + assign10970_e5887);
        let assign10970_e5889: f64 = (p.p171 * assign10970_e5888);
        let assign10970_e5894: f64 = (var_wg).powf(p.p175);
        let assign10970_e5895: f64 = (p.p174 / assign10970_e5894);
        let assign10970_e5896: f64 = (1.0 + assign10970_e5895);
        let assign10970_e5897: f64 = (assign10970_e5889 * assign10970_e5896);
        var_muesr = assign10970_e5897;
        var_muesr_rv = 0.0;

        let (assign11000_e5921, assign11000_e5921_d_n0, assign11000_e5921_d_n2, assign11000_e5921_d_n4, assign11000_e5921_d_n5, assign11000_e5921_d_n6, assign11000_e5921_d_n7, assign11000_e5921_d_n8, assign11000_e5921_d_n9, assign11000_e5921_d_n10, assign11000_e5921_d_n13,) = {
    if (var_mueph < 1e-25) {
        (1e-25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mueph, var_mueph_dn0, var_mueph_dn2, var_mueph_dn4, var_mueph_dn5, var_mueph_dn6, var_mueph_dn7, var_mueph_dn8, var_mueph_dn9, var_mueph_dn10, var_mueph_dn13,)
    }
};
        var_mueph = assign11000_e5921;
        var_mueph_dn0 = assign11000_e5921_d_n0;
        var_mueph_dn2 = assign11000_e5921_d_n2;
        var_mueph_dn4 = assign11000_e5921_d_n4;
        var_mueph_dn5 = assign11000_e5921_d_n5;
        var_mueph_dn6 = assign11000_e5921_d_n6;
        var_mueph_dn7 = assign11000_e5921_d_n7;
        var_mueph_dn8 = assign11000_e5921_d_n8;
        var_mueph_dn9 = assign11000_e5921_d_n9;
        var_mueph_dn10 = assign11000_e5921_d_n10;
        var_mueph_dn13 = assign11000_e5921_d_n13;
        var_mueph_rv = 0.0;

        let (assign11010_e5927,) = {
    if (var_muesr < 1e-25) {
        (1e-25,)
    } else {
        (var_muesr,)
    }
};
        var_muesr = assign11010_e5927;
        var_muesr_rv = 0.0;

        let assign11020_e5930: f64 = (var_lg).powf(p.p156);
        var_t1 = assign11020_e5930;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_rv = 0.0;

        let assign11030_e5933: f64 = (var_uc_ndep * var_t1);
        let assign11030_e5936: f64 = (var_t1 + p.p155);
        let assign11030_e5937: f64 = (assign11030_e5933 / assign11030_e5936);
        let assign11030_e5939: f64 = (assign11030_e5937 / 1.034943e-10);
        var_ndep_o_esi = assign11030_e5939;
        var_ndep_o_esi_dn0 = (((((var_uc_ndep * var_t1_dn0) * assign11030_e5936) - (assign11030_e5933 * var_t1_dn0)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        var_ndep_o_esi_dn2 = (((((var_uc_ndep * var_t1_dn2) * assign11030_e5936) - (assign11030_e5933 * var_t1_dn2)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        var_ndep_o_esi_dn4 = (((((var_uc_ndep * var_t1_dn4) * assign11030_e5936) - (assign11030_e5933 * var_t1_dn4)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        var_ndep_o_esi_dn5 = (((((var_uc_ndep * var_t1_dn5) * assign11030_e5936) - (assign11030_e5933 * var_t1_dn5)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        var_ndep_o_esi_dn6 = (((((var_uc_ndep * var_t1_dn6) * assign11030_e5936) - (assign11030_e5933 * var_t1_dn6)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        var_ndep_o_esi_dn7 = (((((var_uc_ndep * var_t1_dn7) * assign11030_e5936) - (assign11030_e5933 * var_t1_dn7)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        var_ndep_o_esi_dn8 = (((((var_uc_ndep * var_t1_dn8) * assign11030_e5936) - (assign11030_e5933 * var_t1_dn8)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        var_ndep_o_esi_dn9 = (((((var_uc_ndep * var_t1_dn9) * assign11030_e5936) - (assign11030_e5933 * var_t1_dn9)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        var_ndep_o_esi_dn10 = (((((var_uc_ndep * var_t1_dn10) * assign11030_e5936) - (assign11030_e5933 * var_t1_dn10)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        var_ndep_o_esi_dn13 = (((((var_uc_ndep * var_t1_dn13) * assign11030_e5936) - (assign11030_e5933 * var_t1_dn13)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        var_ndep_o_esi_rv = 0.0;

        let assign11040_e5942: f64 = (var_uc_ninv / 1.034943e-10);
        var_ninv_o_esi = assign11040_e5942;
        var_ninv_o_esi_rv = 0.0;

        let assign11050_e5948: f64 = (var_lg).powf(p.p321);
        let assign11050_e5949: f64 = (p.p320 / assign11050_e5948);
        let assign11050_e5950: f64 = (1.0 + assign11050_e5949);
        let assign11050_e5951: f64 = (p.p319 * assign11050_e5950);
        let assign11050_e5956: f64 = (var_wg).powf(p.p323);
        let assign11050_e5957: f64 = (p.p322 / assign11050_e5956);
        let assign11050_e5958: f64 = (1.0 + assign11050_e5957);
        let assign11050_e5959: f64 = (assign11050_e5951 * assign11050_e5958);
        var_ninvd0 = assign11050_e5959;
        var_ninvd0_rv = 0.0;

        let assign11060_e5964: f64 = (var_lg).powf(p.p387);
        let assign11060_e5965: f64 = (p.p386 / assign11060_e5964);
        let assign11060_e5966: f64 = (1.0 + assign11060_e5965);
        let assign11060_e5971: f64 = (var_wg).powf(p.p389);
        let assign11060_e5972: f64 = (p.p388 / assign11060_e5971);
        let assign11060_e5973: f64 = (1.0 + assign11060_e5972);
        let assign11060_e5974: f64 = (assign11060_e5966 * assign11060_e5973);
        var_t1 = assign11060_e5974;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_rv = 0.0;

        let assign11070_e5977: f64 = (p.p384 * var_t1);
        var_ninvd0cres = assign11070_e5977;
        var_ninvd0cres_dn0 = (p.p384 * var_t1_dn0);
        var_ninvd0cres_dn2 = (p.p384 * var_t1_dn2);
        var_ninvd0cres_dn4 = (p.p384 * var_t1_dn4);
        var_ninvd0cres_dn5 = (p.p384 * var_t1_dn5);
        var_ninvd0cres_dn6 = (p.p384 * var_t1_dn6);
        var_ninvd0cres_dn7 = (p.p384 * var_t1_dn7);
        var_ninvd0cres_dn8 = (p.p384 * var_t1_dn8);
        var_ninvd0cres_dn9 = (p.p384 * var_t1_dn9);
        var_ninvd0cres_dn10 = (p.p384 * var_t1_dn10);
        var_ninvd0cres_dn13 = (p.p384 * var_t1_dn13);
        var_ninvd0cres_rv = 0.0;

        let assign11080_e5980: f64 = (p.p385 * var_t1);
        var_ninvd0hres = assign11080_e5980;
        var_ninvd0hres_dn0 = (p.p385 * var_t1_dn0);
        var_ninvd0hres_dn2 = (p.p385 * var_t1_dn2);
        var_ninvd0hres_dn4 = (p.p385 * var_t1_dn4);
        var_ninvd0hres_dn5 = (p.p385 * var_t1_dn5);
        var_ninvd0hres_dn6 = (p.p385 * var_t1_dn6);
        var_ninvd0hres_dn7 = (p.p385 * var_t1_dn7);
        var_ninvd0hres_dn8 = (p.p385 * var_t1_dn8);
        var_ninvd0hres_dn9 = (p.p385 * var_t1_dn9);
        var_ninvd0hres_dn10 = (p.p385 * var_t1_dn10);
        var_ninvd0hres_dn13 = (p.p385 * var_t1_dn13);
        var_ninvd0hres_rv = 0.0;

        let assign11090_e5985: f64 = (var_lgate + p.p121);
        let assign11090_e5987: f64 = (assign11090_e5985).powf(p.p122);
        let assign11090_e5988: f64 = (var_mks_ll / assign11090_e5987);
        let assign11090_e5989: f64 = (p.p97 + assign11090_e5988);
        var_dl = assign11090_e5989;
        var_dl_rv = 0.0;

        let assign11100_e5994: f64 = (var_lgate + p.p121);
        let assign11100_e5996: f64 = (assign11100_e5994).powf(p.p122);
        let assign11100_e5997: f64 = (var_mks_ll / assign11100_e5996);
        let assign11100_e5998: f64 = (var_uc_xldld + assign11100_e5997);
        var_dlld = assign11100_e5998;
        var_dlld_rv = 0.0;

        let assign11110_e6003: f64 = (var_wgate + p.p128);
        let assign11110_e6005: f64 = (assign11110_e6003).powf(p.p129);
        let assign11110_e6006: f64 = (var_mks_wl / assign11110_e6005);
        let assign11110_e6007: f64 = (p.p114 + assign11110_e6006);
        var_dw = assign11110_e6007;
        var_dw_rv = 0.0;

        let assign11120_e6012: f64 = (var_wgate + p.p128);
        let assign11120_e6014: f64 = (assign11120_e6012).powf(p.p129);
        let assign11120_e6015: f64 = (var_mks_wl / assign11120_e6014);
        let assign11120_e6016: f64 = (p.p295 + assign11120_e6015);
        var_dwld = assign11120_e6016;
        var_dwld_rv = 0.0;

        let assign11130_e6021: f64 = (var_wgate + p.p128);
        let assign11130_e6023: f64 = (assign11130_e6021).powf(p.p129);
        let assign11130_e6024: f64 = (var_mks_wl / assign11130_e6023);
        let assign11130_e6025: f64 = (p.p115 + assign11130_e6024);
        var_dwcv = assign11130_e6025;
        var_dwcv_rv = 0.0;

        let assign11140_e6029: f64 = (var_dl + var_dlld);
        let assign11140_e6030: f64 = (var_lgate - assign11140_e6029);
        var_leff = assign11140_e6030;
        var_leff_rv = 0.0;

        let assign11170_e6042: f64 = (var_wlg).powf(p.p125);
        let assign11170_e6043: f64 = (p.p124 / assign11170_e6042);
        let assign11170_e6044: f64 = (var_lgate + assign11170_e6043);
        var_lgatesm = assign11170_e6044;
        var_lgatesm_rv = 0.0;

        let assign11180_e6048: f64 = (var_wlg).powf(p.p127);
        let assign11180_e6049: f64 = (var_uc_wl2 / assign11180_e6048);
        var_dvthsm = assign11180_e6049;
        var_dvthsm_rv = 0.0;

        let assign11190_e6054: f64 = (var_lgatesm * 1000000.0);
        let assign11190_e6056: f64 = (assign11190_e6054).powf(p.p207);
        let assign11190_e6057: f64 = (p.p206 / assign11190_e6056);
        let assign11190_e6058: f64 = (1.0 + assign11190_e6057);
        var_t1 = assign11190_e6058;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_rv = 0.0;

        let assign11200_e6063: f64 = (var_wg).powf(p.p209);
        let assign11200_e6064: f64 = (p.p208 / assign11200_e6063);
        let assign11200_e6065: f64 = (1.0 + assign11200_e6064);
        var_t2 = assign11200_e6065;
        var_t2_dn0 = 0.0;
        var_t2_dn2 = 0.0;
        var_t2_dn4 = 0.0;
        var_t2_dn5 = 0.0;
        var_t2_dn6 = 0.0;
        var_t2_dn7 = 0.0;
        var_t2_dn8 = 0.0;
        var_t2_dn9 = 0.0;
        var_t2_dn10 = 0.0;
        var_t2_dn13 = 0.0;
        var_t2_rv = 0.0;

        let assign11210_e6068: f64 = (var_uc_wsti * var_t1);
        let assign11210_e6070: f64 = (assign11210_e6068 * var_t2);
        var_uc_wsti = assign11210_e6070;
        var_uc_wsti_dn0 = ((((var_uc_wsti_dn0 * var_t1) + (var_uc_wsti * var_t1_dn0)) * var_t2) + (assign11210_e6068 * var_t2_dn0));
        var_uc_wsti_dn2 = ((((var_uc_wsti_dn2 * var_t1) + (var_uc_wsti * var_t1_dn2)) * var_t2) + (assign11210_e6068 * var_t2_dn2));
        var_uc_wsti_dn4 = ((((var_uc_wsti_dn4 * var_t1) + (var_uc_wsti * var_t1_dn4)) * var_t2) + (assign11210_e6068 * var_t2_dn4));
        var_uc_wsti_dn5 = ((((var_uc_wsti_dn5 * var_t1) + (var_uc_wsti * var_t1_dn5)) * var_t2) + (assign11210_e6068 * var_t2_dn5));
        var_uc_wsti_dn6 = ((((var_uc_wsti_dn6 * var_t1) + (var_uc_wsti * var_t1_dn6)) * var_t2) + (assign11210_e6068 * var_t2_dn6));
        var_uc_wsti_dn7 = ((((var_uc_wsti_dn7 * var_t1) + (var_uc_wsti * var_t1_dn7)) * var_t2) + (assign11210_e6068 * var_t2_dn7));
        var_uc_wsti_dn8 = ((((var_uc_wsti_dn8 * var_t1) + (var_uc_wsti * var_t1_dn8)) * var_t2) + (assign11210_e6068 * var_t2_dn8));
        var_uc_wsti_dn9 = ((((var_uc_wsti_dn9 * var_t1) + (var_uc_wsti * var_t1_dn9)) * var_t2) + (assign11210_e6068 * var_t2_dn9));
        var_uc_wsti_dn10 = ((((var_uc_wsti_dn10 * var_t1) + (var_uc_wsti * var_t1_dn10)) * var_t2) + (assign11210_e6068 * var_t2_dn10));
        var_uc_wsti_dn13 = ((((var_uc_wsti_dn13 * var_t1) + (var_uc_wsti * var_t1_dn13)) * var_t2) + (assign11210_e6068 * var_t2_dn13));
        var_uc_wsti_rv = 0.0;

        let assign11220_e6074: f64 = (2.0 * var_dw);
        let assign11220_e6075: f64 = (var_wgate - assign11220_e6074);
        var_weff = assign11220_e6075;
        var_weff_rv = 0.0;

        let assign11230_e6079: f64 = (2.0 * var_dwld);
        let assign11230_e6080: f64 = (var_wgate - assign11230_e6079);
        var_weff_ld = assign11230_e6080;
        var_weff_ld_rv = 0.0;

        let assign11240_e6084: f64 = (2.0 * var_dwcv);
        let assign11240_e6085: f64 = (var_wgate - assign11240_e6084);
        var_weff_cv = assign11240_e6085;
        var_weff_cv_rv = 0.0;

        let assign11310_e6109: f64 = (var_weff * p.p7);
        var_weff_nf = assign11310_e6109;
        var_weff_nf_rv = 0.0;

        let assign11320_e6112: f64 = (var_weff_cv * p.p7);
        var_weffcv_nf = assign11320_e6112;
        var_weffcv_nf_rv = 0.0;

        let assign11330_e6118: f64 = (var_wg).powf(p.p143);
        let assign11330_e6119: f64 = (p.p142 / assign11330_e6118);
        let assign11330_e6120: f64 = (1.0 + assign11330_e6119);
        let assign11330_e6121: f64 = (var_ef_nsubp * assign11330_e6120);
        var_nsubpp = assign11330_e6121;
        var_nsubpp_dn0 = (var_ef_nsubp_dn0 * assign11330_e6120);
        var_nsubpp_dn2 = (var_ef_nsubp_dn2 * assign11330_e6120);
        var_nsubpp_dn4 = (var_ef_nsubp_dn4 * assign11330_e6120);
        var_nsubpp_dn5 = (var_ef_nsubp_dn5 * assign11330_e6120);
        var_nsubpp_dn6 = (var_ef_nsubp_dn6 * assign11330_e6120);
        var_nsubpp_dn7 = (var_ef_nsubp_dn7 * assign11330_e6120);
        var_nsubpp_dn8 = (var_ef_nsubp_dn8 * assign11330_e6120);
        var_nsubpp_dn9 = (var_ef_nsubp_dn9 * assign11330_e6120);
        var_nsubpp_dn10 = (var_ef_nsubp_dn10 * assign11330_e6120);
        var_nsubpp_dn13 = (var_ef_nsubp_dn13 * assign11330_e6120);
        var_nsubpp_rv = 0.0;

        let assign11340_e6127: f64 = (var_wg).powf(p.p234);
        let assign11340_e6128: f64 = (p.p233 / assign11340_e6127);
        let assign11340_e6129: f64 = (1.0 + assign11340_e6128);
        let assign11340_e6130: f64 = (var_ef_nsubc * assign11340_e6129);
        var_ef_nsubc = assign11340_e6130;
        var_ef_nsubc_dn0 = (var_ef_nsubc_dn0 * assign11340_e6129);
        var_ef_nsubc_dn2 = (var_ef_nsubc_dn2 * assign11340_e6129);
        var_ef_nsubc_dn4 = (var_ef_nsubc_dn4 * assign11340_e6129);
        var_ef_nsubc_dn5 = (var_ef_nsubc_dn5 * assign11340_e6129);
        var_ef_nsubc_dn6 = (var_ef_nsubc_dn6 * assign11340_e6129);
        var_ef_nsubc_dn7 = (var_ef_nsubc_dn7 * assign11340_e6129);
        var_ef_nsubc_dn8 = (var_ef_nsubc_dn8 * assign11340_e6129);
        var_ef_nsubc_dn9 = (var_ef_nsubc_dn9 * assign11340_e6129);
        var_ef_nsubc_dn10 = (var_ef_nsubc_dn10 * assign11340_e6129);
        var_ef_nsubc_dn13 = (var_ef_nsubc_dn13 * assign11340_e6129);
        var_ef_nsubc_rv = 0.0;

        *var_dl_slot = var_dl;
        *var_dl_rv_slot = var_dl_rv;
        *var_dlld_slot = var_dlld;
        *var_dlld_rv_slot = var_dlld_rv;
        *var_dvthsm_slot = var_dvthsm;
        *var_dvthsm_rv_slot = var_dvthsm_rv;
        *var_dw_slot = var_dw;
        *var_dw_rv_slot = var_dw_rv;
        *var_dwcv_slot = var_dwcv;
        *var_dwcv_rv_slot = var_dwcv_rv;
        *var_dwld_slot = var_dwld;
        *var_dwld_rv_slot = var_dwld_rv;
        *var_ef_mueph1_slot = var_ef_mueph1;
        *var_ef_mueph1_dn0_slot = var_ef_mueph1_dn0;
        *var_ef_mueph1_dn10_slot = var_ef_mueph1_dn10;
        *var_ef_mueph1_dn13_slot = var_ef_mueph1_dn13;
        *var_ef_mueph1_dn2_slot = var_ef_mueph1_dn2;
        *var_ef_mueph1_dn4_slot = var_ef_mueph1_dn4;
        *var_ef_mueph1_dn5_slot = var_ef_mueph1_dn5;
        *var_ef_mueph1_dn6_slot = var_ef_mueph1_dn6;
        *var_ef_mueph1_dn7_slot = var_ef_mueph1_dn7;
        *var_ef_mueph1_dn8_slot = var_ef_mueph1_dn8;
        *var_ef_mueph1_dn9_slot = var_ef_mueph1_dn9;
        *var_ef_mueph1_rv_slot = var_ef_mueph1_rv;
        *var_ef_nsubc_slot = var_ef_nsubc;
        *var_ef_nsubc_dn0_slot = var_ef_nsubc_dn0;
        *var_ef_nsubc_dn10_slot = var_ef_nsubc_dn10;
        *var_ef_nsubc_dn13_slot = var_ef_nsubc_dn13;
        *var_ef_nsubc_dn2_slot = var_ef_nsubc_dn2;
        *var_ef_nsubc_dn4_slot = var_ef_nsubc_dn4;
        *var_ef_nsubc_dn5_slot = var_ef_nsubc_dn5;
        *var_ef_nsubc_dn6_slot = var_ef_nsubc_dn6;
        *var_ef_nsubc_dn7_slot = var_ef_nsubc_dn7;
        *var_ef_nsubc_dn8_slot = var_ef_nsubc_dn8;
        *var_ef_nsubc_dn9_slot = var_ef_nsubc_dn9;
        *var_ef_nsubc_rv_slot = var_ef_nsubc_rv;
        *var_ef_nsubp_slot = var_ef_nsubp;
        *var_ef_nsubp_dn0_slot = var_ef_nsubp_dn0;
        *var_ef_nsubp_dn10_slot = var_ef_nsubp_dn10;
        *var_ef_nsubp_dn13_slot = var_ef_nsubp_dn13;
        *var_ef_nsubp_dn2_slot = var_ef_nsubp_dn2;
        *var_ef_nsubp_dn4_slot = var_ef_nsubp_dn4;
        *var_ef_nsubp_dn5_slot = var_ef_nsubp_dn5;
        *var_ef_nsubp_dn6_slot = var_ef_nsubp_dn6;
        *var_ef_nsubp_dn7_slot = var_ef_nsubp_dn7;
        *var_ef_nsubp_dn8_slot = var_ef_nsubp_dn8;
        *var_ef_nsubp_dn9_slot = var_ef_nsubp_dn9;
        *var_ef_nsubp_rv_slot = var_ef_nsubp_rv;
        *var_guard255_slot = var_guard255;
        *var_guard255_rv_slot = var_guard255_rv;
        *var_leff_slot = var_leff;
        *var_leff_rv_slot = var_leff_rv;
        *var_lgatesm_slot = var_lgatesm;
        *var_lgatesm_rv_slot = var_lgatesm_rv;
        *var_mueph_slot = var_mueph;
        *var_mueph_dn0_slot = var_mueph_dn0;
        *var_mueph_dn10_slot = var_mueph_dn10;
        *var_mueph_dn13_slot = var_mueph_dn13;
        *var_mueph_dn2_slot = var_mueph_dn2;
        *var_mueph_dn4_slot = var_mueph_dn4;
        *var_mueph_dn5_slot = var_mueph_dn5;
        *var_mueph_dn6_slot = var_mueph_dn6;
        *var_mueph_dn7_slot = var_mueph_dn7;
        *var_mueph_dn8_slot = var_mueph_dn8;
        *var_mueph_dn9_slot = var_mueph_dn9;
        *var_mueph_rv_slot = var_mueph_rv;
        *var_muesr_slot = var_muesr;
        *var_muesr_rv_slot = var_muesr_rv;
        *var_ndep_o_esi_slot = var_ndep_o_esi;
        *var_ndep_o_esi_dn0_slot = var_ndep_o_esi_dn0;
        *var_ndep_o_esi_dn10_slot = var_ndep_o_esi_dn10;
        *var_ndep_o_esi_dn13_slot = var_ndep_o_esi_dn13;
        *var_ndep_o_esi_dn2_slot = var_ndep_o_esi_dn2;
        *var_ndep_o_esi_dn4_slot = var_ndep_o_esi_dn4;
        *var_ndep_o_esi_dn5_slot = var_ndep_o_esi_dn5;
        *var_ndep_o_esi_dn6_slot = var_ndep_o_esi_dn6;
        *var_ndep_o_esi_dn7_slot = var_ndep_o_esi_dn7;
        *var_ndep_o_esi_dn8_slot = var_ndep_o_esi_dn8;
        *var_ndep_o_esi_dn9_slot = var_ndep_o_esi_dn9;
        *var_ndep_o_esi_rv_slot = var_ndep_o_esi_rv;
        *var_ninv_o_esi_slot = var_ninv_o_esi;
        *var_ninv_o_esi_rv_slot = var_ninv_o_esi_rv;
        *var_ninvd0_slot = var_ninvd0;
        *var_ninvd0_rv_slot = var_ninvd0_rv;
        *var_ninvd0cres_slot = var_ninvd0cres;
        *var_ninvd0cres_dn0_slot = var_ninvd0cres_dn0;
        *var_ninvd0cres_dn10_slot = var_ninvd0cres_dn10;
        *var_ninvd0cres_dn13_slot = var_ninvd0cres_dn13;
        *var_ninvd0cres_dn2_slot = var_ninvd0cres_dn2;
        *var_ninvd0cres_dn4_slot = var_ninvd0cres_dn4;
        *var_ninvd0cres_dn5_slot = var_ninvd0cres_dn5;
        *var_ninvd0cres_dn6_slot = var_ninvd0cres_dn6;
        *var_ninvd0cres_dn7_slot = var_ninvd0cres_dn7;
        *var_ninvd0cres_dn8_slot = var_ninvd0cres_dn8;
        *var_ninvd0cres_dn9_slot = var_ninvd0cres_dn9;
        *var_ninvd0cres_rv_slot = var_ninvd0cres_rv;
        *var_ninvd0hres_slot = var_ninvd0hres;
        *var_ninvd0hres_dn0_slot = var_ninvd0hres_dn0;
        *var_ninvd0hres_dn10_slot = var_ninvd0hres_dn10;
        *var_ninvd0hres_dn13_slot = var_ninvd0hres_dn13;
        *var_ninvd0hres_dn2_slot = var_ninvd0hres_dn2;
        *var_ninvd0hres_dn4_slot = var_ninvd0hres_dn4;
        *var_ninvd0hres_dn5_slot = var_ninvd0hres_dn5;
        *var_ninvd0hres_dn6_slot = var_ninvd0hres_dn6;
        *var_ninvd0hres_dn7_slot = var_ninvd0hres_dn7;
        *var_ninvd0hres_dn8_slot = var_ninvd0hres_dn8;
        *var_ninvd0hres_dn9_slot = var_ninvd0hres_dn9;
        *var_ninvd0hres_rv_slot = var_ninvd0hres_rv;
        *var_npexte_slot = var_npexte;
        *var_npexte_dn0_slot = var_npexte_dn0;
        *var_npexte_dn10_slot = var_npexte_dn10;
        *var_npexte_dn13_slot = var_npexte_dn13;
        *var_npexte_dn2_slot = var_npexte_dn2;
        *var_npexte_dn4_slot = var_npexte_dn4;
        *var_npexte_dn5_slot = var_npexte_dn5;
        *var_npexte_dn6_slot = var_npexte_dn6;
        *var_npexte_dn7_slot = var_npexte_dn7;
        *var_npexte_dn8_slot = var_npexte_dn8;
        *var_npexte_dn9_slot = var_npexte_dn9;
        *var_npexte_rv_slot = var_npexte_rv;
        *var_nsubpp_slot = var_nsubpp;
        *var_nsubpp_dn0_slot = var_nsubpp_dn0;
        *var_nsubpp_dn10_slot = var_nsubpp_dn10;
        *var_nsubpp_dn13_slot = var_nsubpp_dn13;
        *var_nsubpp_dn2_slot = var_nsubpp_dn2;
        *var_nsubpp_dn4_slot = var_nsubpp_dn4;
        *var_nsubpp_dn5_slot = var_nsubpp_dn5;
        *var_nsubpp_dn6_slot = var_nsubpp_dn6;
        *var_nsubpp_dn7_slot = var_nsubpp_dn7;
        *var_nsubpp_dn8_slot = var_nsubpp_dn8;
        *var_nsubpp_dn9_slot = var_nsubpp_dn9;
        *var_nsubpp_rv_slot = var_nsubpp_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_uc_wsti_slot = var_uc_wsti;
        *var_uc_wsti_dn0_slot = var_uc_wsti_dn0;
        *var_uc_wsti_dn10_slot = var_uc_wsti_dn10;
        *var_uc_wsti_dn13_slot = var_uc_wsti_dn13;
        *var_uc_wsti_dn2_slot = var_uc_wsti_dn2;
        *var_uc_wsti_dn4_slot = var_uc_wsti_dn4;
        *var_uc_wsti_dn5_slot = var_uc_wsti_dn5;
        *var_uc_wsti_dn6_slot = var_uc_wsti_dn6;
        *var_uc_wsti_dn7_slot = var_uc_wsti_dn7;
        *var_uc_wsti_dn8_slot = var_uc_wsti_dn8;
        *var_uc_wsti_dn9_slot = var_uc_wsti_dn9;
        *var_uc_wsti_rv_slot = var_uc_wsti_rv;
        *var_weff_slot = var_weff;
        *var_weff_cv_slot = var_weff_cv;
        *var_weff_cv_rv_slot = var_weff_cv_rv;
        *var_weff_ld_slot = var_weff_ld;
        *var_weff_ld_rv_slot = var_weff_ld_rv;
        *var_weff_nf_slot = var_weff_nf;
        *var_weff_nf_rv_slot = var_weff_nf_rv;
        *var_weff_rv_slot = var_weff_rv;
        *var_weffcv_nf_slot = var_weffcv_nf;
        *var_weffcv_nf_rv_slot = var_weffcv_nf_rv;
    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        var_lgate: f64,
        var_lod_half: f64,
        var_lod_half_dn0: f64,
        var_lod_half_dn10: f64,
        var_lod_half_dn13: f64,
        var_lod_half_dn2: f64,
        var_lod_half_dn4: f64,
        var_lod_half_dn5: f64,
        var_lod_half_dn6: f64,
        var_lod_half_dn7: f64,
        var_lod_half_dn8: f64,
        var_lod_half_dn9: f64,
        var_lod_half_ref: f64,
        var_lod_half_ref_dn0: f64,
        var_lod_half_ref_dn10: f64,
        var_lod_half_ref_dn13: f64,
        var_lod_half_ref_dn2: f64,
        var_lod_half_ref_dn4: f64,
        var_lod_half_ref_dn5: f64,
        var_lod_half_ref_dn6: f64,
        var_lod_half_ref_dn7: f64,
        var_lod_half_ref_dn8: f64,
        var_lod_half_ref_dn9: f64,
        var_npexte: f64,
        var_npexte_dn0: f64,
        var_npexte_dn10: f64,
        var_npexte_dn13: f64,
        var_npexte_dn2: f64,
        var_npexte_dn4: f64,
        var_npexte_dn5: f64,
        var_npexte_dn6: f64,
        var_npexte_dn7: f64,
        var_npexte_dn8: f64,
        var_npexte_dn9: f64,
        var_uc_nsubpsti1: f64,
        var_uc_nsubpsti2: f64,
        var_uc_nsubpsti3: f64,
        var_ef_nsubc_slot: &mut f64,
        var_ef_nsubc_dn0_slot: &mut f64,
        var_ef_nsubc_dn10_slot: &mut f64,
        var_ef_nsubc_dn13_slot: &mut f64,
        var_ef_nsubc_dn2_slot: &mut f64,
        var_ef_nsubc_dn4_slot: &mut f64,
        var_ef_nsubc_dn5_slot: &mut f64,
        var_ef_nsubc_dn6_slot: &mut f64,
        var_ef_nsubc_dn7_slot: &mut f64,
        var_ef_nsubc_dn8_slot: &mut f64,
        var_ef_nsubc_dn9_slot: &mut f64,
        var_ef_nsubc_rv_slot: &mut f64,
        var_guard263_slot: &mut f64,
        var_guard263_rv_slot: &mut f64,
        var_guard265_slot: &mut f64,
        var_guard265_rv_slot: &mut f64,
        var_guard266_slot: &mut f64,
        var_guard266_rv_slot: &mut f64,
        var_guard267_slot: &mut f64,
        var_guard267_rv_slot: &mut f64,
        var_guard268_slot: &mut f64,
        var_guard268_rv_slot: &mut f64,
        var_nsub_slot: &mut f64,
        var_nsub_dn0_slot: &mut f64,
        var_nsub_dn10_slot: &mut f64,
        var_nsub_dn13_slot: &mut f64,
        var_nsub_dn2_slot: &mut f64,
        var_nsub_dn4_slot: &mut f64,
        var_nsub_dn5_slot: &mut f64,
        var_nsub_dn6_slot: &mut f64,
        var_nsub_dn7_slot: &mut f64,
        var_nsub_dn8_slot: &mut f64,
        var_nsub_dn9_slot: &mut f64,
        var_nsub_rv_slot: &mut f64,
        var_nsubpp_slot: &mut f64,
        var_nsubpp_dn0_slot: &mut f64,
        var_nsubpp_dn10_slot: &mut f64,
        var_nsubpp_dn13_slot: &mut f64,
        var_nsubpp_dn2_slot: &mut f64,
        var_nsubpp_dn4_slot: &mut f64,
        var_nsubpp_dn5_slot: &mut f64,
        var_nsubpp_dn6_slot: &mut f64,
        var_nsubpp_dn7_slot: &mut f64,
        var_nsubpp_dn8_slot: &mut f64,
        var_nsubpp_dn9_slot: &mut f64,
        var_nsubpp_rv_slot: &mut f64,
        var_nsubps_slot: &mut f64,
        var_nsubps_dn0_slot: &mut f64,
        var_nsubps_dn10_slot: &mut f64,
        var_nsubps_dn13_slot: &mut f64,
        var_nsubps_dn2_slot: &mut f64,
        var_nsubps_dn4_slot: &mut f64,
        var_nsubps_dn5_slot: &mut f64,
        var_nsubps_dn6_slot: &mut f64,
        var_nsubps_dn7_slot: &mut f64,
        var_nsubps_dn8_slot: &mut f64,
        var_nsubps_dn9_slot: &mut f64,
        var_nsubps_rv_slot: &mut f64,
        var_q_nsub_slot: &mut f64,
        var_q_nsub_dn0_slot: &mut f64,
        var_q_nsub_dn10_slot: &mut f64,
        var_q_nsub_dn13_slot: &mut f64,
        var_q_nsub_dn2_slot: &mut f64,
        var_q_nsub_dn4_slot: &mut f64,
        var_q_nsub_dn5_slot: &mut f64,
        var_q_nsub_dn6_slot: &mut f64,
        var_q_nsub_dn7_slot: &mut f64,
        var_q_nsub_dn8_slot: &mut f64,
        var_q_nsub_dn9_slot: &mut f64,
        var_q_nsub_rv_slot: &mut f64,
        var_qnsub_esi_slot: &mut f64,
        var_qnsub_esi2_slot: &mut f64,
        var_qnsub_esi2_dn0_slot: &mut f64,
        var_qnsub_esi2_dn10_slot: &mut f64,
        var_qnsub_esi2_dn13_slot: &mut f64,
        var_qnsub_esi2_dn2_slot: &mut f64,
        var_qnsub_esi2_dn4_slot: &mut f64,
        var_qnsub_esi2_dn5_slot: &mut f64,
        var_qnsub_esi2_dn6_slot: &mut f64,
        var_qnsub_esi2_dn7_slot: &mut f64,
        var_qnsub_esi2_dn8_slot: &mut f64,
        var_qnsub_esi2_dn9_slot: &mut f64,
        var_qnsub_esi2_rv_slot: &mut f64,
        var_qnsub_esi_dn0_slot: &mut f64,
        var_qnsub_esi_dn10_slot: &mut f64,
        var_qnsub_esi_dn13_slot: &mut f64,
        var_qnsub_esi_dn2_slot: &mut f64,
        var_qnsub_esi_dn4_slot: &mut f64,
        var_qnsub_esi_dn5_slot: &mut f64,
        var_qnsub_esi_dn6_slot: &mut f64,
        var_qnsub_esi_dn7_slot: &mut f64,
        var_qnsub_esi_dn8_slot: &mut f64,
        var_qnsub_esi_dn9_slot: &mut f64,
        var_qnsub_esi_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn13_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_dn9_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn13_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_ef_nsubc: f64 = *var_ef_nsubc_slot;
        let mut var_ef_nsubc_dn0: f64 = *var_ef_nsubc_dn0_slot;
        let mut var_ef_nsubc_dn10: f64 = *var_ef_nsubc_dn10_slot;
        let mut var_ef_nsubc_dn13: f64 = *var_ef_nsubc_dn13_slot;
        let mut var_ef_nsubc_dn2: f64 = *var_ef_nsubc_dn2_slot;
        let mut var_ef_nsubc_dn4: f64 = *var_ef_nsubc_dn4_slot;
        let mut var_ef_nsubc_dn5: f64 = *var_ef_nsubc_dn5_slot;
        let mut var_ef_nsubc_dn6: f64 = *var_ef_nsubc_dn6_slot;
        let mut var_ef_nsubc_dn7: f64 = *var_ef_nsubc_dn7_slot;
        let mut var_ef_nsubc_dn8: f64 = *var_ef_nsubc_dn8_slot;
        let mut var_ef_nsubc_dn9: f64 = *var_ef_nsubc_dn9_slot;
        let mut var_ef_nsubc_rv: f64 = *var_ef_nsubc_rv_slot;
        let mut var_guard263: f64 = *var_guard263_slot;
        let mut var_guard263_rv: f64 = *var_guard263_rv_slot;
        let mut var_guard265: f64 = *var_guard265_slot;
        let mut var_guard265_rv: f64 = *var_guard265_rv_slot;
        let mut var_guard266: f64 = *var_guard266_slot;
        let mut var_guard266_rv: f64 = *var_guard266_rv_slot;
        let mut var_guard267: f64 = *var_guard267_slot;
        let mut var_guard267_rv: f64 = *var_guard267_rv_slot;
        let mut var_guard268: f64 = *var_guard268_slot;
        let mut var_guard268_rv: f64 = *var_guard268_rv_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
        let mut var_nsub_dn0: f64 = *var_nsub_dn0_slot;
        let mut var_nsub_dn10: f64 = *var_nsub_dn10_slot;
        let mut var_nsub_dn13: f64 = *var_nsub_dn13_slot;
        let mut var_nsub_dn2: f64 = *var_nsub_dn2_slot;
        let mut var_nsub_dn4: f64 = *var_nsub_dn4_slot;
        let mut var_nsub_dn5: f64 = *var_nsub_dn5_slot;
        let mut var_nsub_dn6: f64 = *var_nsub_dn6_slot;
        let mut var_nsub_dn7: f64 = *var_nsub_dn7_slot;
        let mut var_nsub_dn8: f64 = *var_nsub_dn8_slot;
        let mut var_nsub_dn9: f64 = *var_nsub_dn9_slot;
        let mut var_nsub_rv: f64 = *var_nsub_rv_slot;
        let mut var_nsubpp: f64 = *var_nsubpp_slot;
        let mut var_nsubpp_dn0: f64 = *var_nsubpp_dn0_slot;
        let mut var_nsubpp_dn10: f64 = *var_nsubpp_dn10_slot;
        let mut var_nsubpp_dn13: f64 = *var_nsubpp_dn13_slot;
        let mut var_nsubpp_dn2: f64 = *var_nsubpp_dn2_slot;
        let mut var_nsubpp_dn4: f64 = *var_nsubpp_dn4_slot;
        let mut var_nsubpp_dn5: f64 = *var_nsubpp_dn5_slot;
        let mut var_nsubpp_dn6: f64 = *var_nsubpp_dn6_slot;
        let mut var_nsubpp_dn7: f64 = *var_nsubpp_dn7_slot;
        let mut var_nsubpp_dn8: f64 = *var_nsubpp_dn8_slot;
        let mut var_nsubpp_dn9: f64 = *var_nsubpp_dn9_slot;
        let mut var_nsubpp_rv: f64 = *var_nsubpp_rv_slot;
        let mut var_nsubps: f64 = *var_nsubps_slot;
        let mut var_nsubps_dn0: f64 = *var_nsubps_dn0_slot;
        let mut var_nsubps_dn10: f64 = *var_nsubps_dn10_slot;
        let mut var_nsubps_dn13: f64 = *var_nsubps_dn13_slot;
        let mut var_nsubps_dn2: f64 = *var_nsubps_dn2_slot;
        let mut var_nsubps_dn4: f64 = *var_nsubps_dn4_slot;
        let mut var_nsubps_dn5: f64 = *var_nsubps_dn5_slot;
        let mut var_nsubps_dn6: f64 = *var_nsubps_dn6_slot;
        let mut var_nsubps_dn7: f64 = *var_nsubps_dn7_slot;
        let mut var_nsubps_dn8: f64 = *var_nsubps_dn8_slot;
        let mut var_nsubps_dn9: f64 = *var_nsubps_dn9_slot;
        let mut var_nsubps_rv: f64 = *var_nsubps_rv_slot;
        let mut var_q_nsub: f64 = *var_q_nsub_slot;
        let mut var_q_nsub_dn0: f64 = *var_q_nsub_dn0_slot;
        let mut var_q_nsub_dn10: f64 = *var_q_nsub_dn10_slot;
        let mut var_q_nsub_dn13: f64 = *var_q_nsub_dn13_slot;
        let mut var_q_nsub_dn2: f64 = *var_q_nsub_dn2_slot;
        let mut var_q_nsub_dn4: f64 = *var_q_nsub_dn4_slot;
        let mut var_q_nsub_dn5: f64 = *var_q_nsub_dn5_slot;
        let mut var_q_nsub_dn6: f64 = *var_q_nsub_dn6_slot;
        let mut var_q_nsub_dn7: f64 = *var_q_nsub_dn7_slot;
        let mut var_q_nsub_dn8: f64 = *var_q_nsub_dn8_slot;
        let mut var_q_nsub_dn9: f64 = *var_q_nsub_dn9_slot;
        let mut var_q_nsub_rv: f64 = *var_q_nsub_rv_slot;
        let mut var_qnsub_esi: f64 = *var_qnsub_esi_slot;
        let mut var_qnsub_esi2: f64 = *var_qnsub_esi2_slot;
        let mut var_qnsub_esi2_dn0: f64 = *var_qnsub_esi2_dn0_slot;
        let mut var_qnsub_esi2_dn10: f64 = *var_qnsub_esi2_dn10_slot;
        let mut var_qnsub_esi2_dn13: f64 = *var_qnsub_esi2_dn13_slot;
        let mut var_qnsub_esi2_dn2: f64 = *var_qnsub_esi2_dn2_slot;
        let mut var_qnsub_esi2_dn4: f64 = *var_qnsub_esi2_dn4_slot;
        let mut var_qnsub_esi2_dn5: f64 = *var_qnsub_esi2_dn5_slot;
        let mut var_qnsub_esi2_dn6: f64 = *var_qnsub_esi2_dn6_slot;
        let mut var_qnsub_esi2_dn7: f64 = *var_qnsub_esi2_dn7_slot;
        let mut var_qnsub_esi2_dn8: f64 = *var_qnsub_esi2_dn8_slot;
        let mut var_qnsub_esi2_dn9: f64 = *var_qnsub_esi2_dn9_slot;
        let mut var_qnsub_esi2_rv: f64 = *var_qnsub_esi2_rv_slot;
        let mut var_qnsub_esi_dn0: f64 = *var_qnsub_esi_dn0_slot;
        let mut var_qnsub_esi_dn10: f64 = *var_qnsub_esi_dn10_slot;
        let mut var_qnsub_esi_dn13: f64 = *var_qnsub_esi_dn13_slot;
        let mut var_qnsub_esi_dn2: f64 = *var_qnsub_esi_dn2_slot;
        let mut var_qnsub_esi_dn4: f64 = *var_qnsub_esi_dn4_slot;
        let mut var_qnsub_esi_dn5: f64 = *var_qnsub_esi_dn5_slot;
        let mut var_qnsub_esi_dn6: f64 = *var_qnsub_esi_dn6_slot;
        let mut var_qnsub_esi_dn7: f64 = *var_qnsub_esi_dn7_slot;
        let mut var_qnsub_esi_dn8: f64 = *var_qnsub_esi_dn8_slot;
        let mut var_qnsub_esi_dn9: f64 = *var_qnsub_esi_dn9_slot;
        let mut var_qnsub_esi_rv: f64 = *var_qnsub_esi_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn13: f64 = *var_tmf1_dn13_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_dn9: f64 = *var_tmf1_dn9_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn13: f64 = *var_tmf2_dn13_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let assign11350_e6133: f64 = (var_ef_nsubc * 1e-6);
        var_t1 = assign11350_e6133;
        var_t1_dn0 = (var_ef_nsubc_dn0 * 1e-6);
        var_t1_dn2 = (var_ef_nsubc_dn2 * 1e-6);
        var_t1_dn4 = (var_ef_nsubc_dn4 * 1e-6);
        var_t1_dn5 = (var_ef_nsubc_dn5 * 1e-6);
        var_t1_dn6 = (var_ef_nsubc_dn6 * 1e-6);
        var_t1_dn7 = (var_ef_nsubc_dn7 * 1e-6);
        var_t1_dn8 = (var_ef_nsubc_dn8 * 1e-6);
        var_t1_dn9 = (var_ef_nsubc_dn9 * 1e-6);
        var_t1_dn10 = (var_ef_nsubc_dn10 * 1e-6);
        var_t1_dn13 = (var_ef_nsubc_dn13 * 1e-6);
        var_t1_rv = 0.0;

        let assign11360_e6136: f64 = (var_nsubpp * 1e-6);
        var_t2 = assign11360_e6136;
        var_t2_dn0 = (var_nsubpp_dn0 * 1e-6);
        var_t2_dn2 = (var_nsubpp_dn2 * 1e-6);
        var_t2_dn4 = (var_nsubpp_dn4 * 1e-6);
        var_t2_dn5 = (var_nsubpp_dn5 * 1e-6);
        var_t2_dn6 = (var_nsubpp_dn6 * 1e-6);
        var_t2_dn7 = (var_nsubpp_dn7 * 1e-6);
        var_t2_dn8 = (var_nsubpp_dn8 * 1e-6);
        var_t2_dn9 = (var_nsubpp_dn9 * 1e-6);
        var_t2_dn10 = (var_nsubpp_dn10 * 1e-6);
        var_t2_dn13 = (var_nsubpp_dn13 * 1e-6);
        var_t2_rv = 0.0;

        let assign11380_e6144: f64 = if var_t1 < 1000000000000000.0 { 1.0 } else { 0.0 };
        var_guard263 = assign11380_e6144;
        var_guard263_rv = 0.0;

        let (assign11390_e6148, assign11390_e6148_d_n0, assign11390_e6148_d_n2, assign11390_e6148_d_n4, assign11390_e6148_d_n5, assign11390_e6148_d_n6, assign11390_e6148_d_n7, assign11390_e6148_d_n8, assign11390_e6148_d_n9, assign11390_e6148_d_n10, assign11390_e6148_d_n13,) = {
    if (var_guard263 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign11390_e6148;
        var_t1_dn0 = assign11390_e6148_d_n0;
        var_t1_dn2 = assign11390_e6148_d_n2;
        var_t1_dn4 = assign11390_e6148_d_n4;
        var_t1_dn5 = assign11390_e6148_d_n5;
        var_t1_dn6 = assign11390_e6148_d_n6;
        var_t1_dn7 = assign11390_e6148_d_n7;
        var_t1_dn8 = assign11390_e6148_d_n8;
        var_t1_dn9 = assign11390_e6148_d_n9;
        var_t1_dn10 = assign11390_e6148_d_n10;
        var_t1_dn13 = assign11390_e6148_d_n13;
        var_t1_rv = 0.0;

        let assign11400_e6151: f64 = (var_t1 / 1e-6);
        var_ef_nsubc = assign11400_e6151;
        var_ef_nsubc_dn0 = (var_t1_dn0 / 1e-6);
        var_ef_nsubc_dn2 = (var_t1_dn2 / 1e-6);
        var_ef_nsubc_dn4 = (var_t1_dn4 / 1e-6);
        var_ef_nsubc_dn5 = (var_t1_dn5 / 1e-6);
        var_ef_nsubc_dn6 = (var_t1_dn6 / 1e-6);
        var_ef_nsubc_dn7 = (var_t1_dn7 / 1e-6);
        var_ef_nsubc_dn8 = (var_t1_dn8 / 1e-6);
        var_ef_nsubc_dn9 = (var_t1_dn9 / 1e-6);
        var_ef_nsubc_dn10 = (var_t1_dn10 / 1e-6);
        var_ef_nsubc_dn13 = (var_t1_dn13 / 1e-6);
        var_ef_nsubc_rv = 0.0;

        let assign11420_e6159: f64 = if var_t2 < 1000000000000000.0 { 1.0 } else { 0.0 };
        var_guard265 = assign11420_e6159;
        var_guard265_rv = 0.0;

        let (assign11430_e6163, assign11430_e6163_d_n0, assign11430_e6163_d_n2, assign11430_e6163_d_n4, assign11430_e6163_d_n5, assign11430_e6163_d_n6, assign11430_e6163_d_n7, assign11430_e6163_d_n8, assign11430_e6163_d_n9, assign11430_e6163_d_n10, assign11430_e6163_d_n13,) = {
    if (var_guard265 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign11430_e6163;
        var_t2_dn0 = assign11430_e6163_d_n0;
        var_t2_dn2 = assign11430_e6163_d_n2;
        var_t2_dn4 = assign11430_e6163_d_n4;
        var_t2_dn5 = assign11430_e6163_d_n5;
        var_t2_dn6 = assign11430_e6163_d_n6;
        var_t2_dn7 = assign11430_e6163_d_n7;
        var_t2_dn8 = assign11430_e6163_d_n8;
        var_t2_dn9 = assign11430_e6163_d_n9;
        var_t2_dn10 = assign11430_e6163_d_n10;
        var_t2_dn13 = assign11430_e6163_d_n13;
        var_t2_rv = 0.0;

        let assign11440_e6166: f64 = (var_t2 / 1e-6);
        var_nsubpp = assign11440_e6166;
        var_nsubpp_dn0 = (var_t2_dn0 / 1e-6);
        var_nsubpp_dn2 = (var_t2_dn2 / 1e-6);
        var_nsubpp_dn4 = (var_t2_dn4 / 1e-6);
        var_nsubpp_dn5 = (var_t2_dn5 / 1e-6);
        var_nsubpp_dn6 = (var_t2_dn6 / 1e-6);
        var_nsubpp_dn7 = (var_t2_dn7 / 1e-6);
        var_nsubpp_dn8 = (var_t2_dn8 / 1e-6);
        var_nsubpp_dn9 = (var_t2_dn9 / 1e-6);
        var_nsubpp_dn10 = (var_t2_dn10 / 1e-6);
        var_nsubpp_dn13 = (var_t2_dn13 / 1e-6);
        var_nsubpp_rv = 0.0;

        let assign11450_e6169: f64 = if var_lod_half > 0.0 { 1.0 } else { 0.0 };
        var_guard266 = assign11450_e6169;
        var_guard266_rv = 0.0;

        let (assign11460_e6177, assign11460_e6177_d_n0, assign11460_e6177_d_n2, assign11460_e6177_d_n4, assign11460_e6177_d_n5, assign11460_e6177_d_n6, assign11460_e6177_d_n7, assign11460_e6177_d_n8, assign11460_e6177_d_n9, assign11460_e6177_d_n10, assign11460_e6177_d_n13,) = {
    if (var_guard266 != 0.0) {
        let assign11460_e6174: f64 = (1.0 + var_uc_nsubpsti2);
        let assign11460_e6175: f64 = (1.0 / assign11460_e6174);
        (assign11460_e6175, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign11460_e6177;
        var_t1_dn0 = assign11460_e6177_d_n0;
        var_t1_dn2 = assign11460_e6177_d_n2;
        var_t1_dn4 = assign11460_e6177_d_n4;
        var_t1_dn5 = assign11460_e6177_d_n5;
        var_t1_dn6 = assign11460_e6177_d_n6;
        var_t1_dn7 = assign11460_e6177_d_n7;
        var_t1_dn8 = assign11460_e6177_d_n8;
        var_t1_dn9 = assign11460_e6177_d_n9;
        var_t1_dn10 = assign11460_e6177_d_n10;
        var_t1_dn13 = assign11460_e6177_d_n13;
        var_t1_rv = 0.0;

        let (assign11470_e6185, assign11470_e6185_d_n0, assign11470_e6185_d_n2, assign11470_e6185_d_n4, assign11470_e6185_d_n5, assign11470_e6185_d_n6, assign11470_e6185_d_n7, assign11470_e6185_d_n8, assign11470_e6185_d_n9, assign11470_e6185_d_n10, assign11470_e6185_d_n13,) = {
    if (var_guard266 != 0.0) {
        let assign11470_e6181: f64 = (var_uc_nsubpsti1 / var_lod_half);
        let assign11470_e6183: f64 = (assign11470_e6181).powf(var_uc_nsubpsti3);
        (assign11470_e6183, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11470_e6181).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn0) / (var_lod_half * var_lod_half))))) } } else { (assign11470_e6183 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn0) / (var_lod_half * var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11470_e6181).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn2) / (var_lod_half * var_lod_half))))) } } else { (assign11470_e6183 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn2) / (var_lod_half * var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11470_e6181).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn4) / (var_lod_half * var_lod_half))))) } } else { (assign11470_e6183 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn4) / (var_lod_half * var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11470_e6181).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn5) / (var_lod_half * var_lod_half))))) } } else { (assign11470_e6183 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn5) / (var_lod_half * var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11470_e6181).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn6) / (var_lod_half * var_lod_half))))) } } else { (assign11470_e6183 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn6) / (var_lod_half * var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11470_e6181).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn7) / (var_lod_half * var_lod_half))))) } } else { (assign11470_e6183 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn7) / (var_lod_half * var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11470_e6181).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn8) / (var_lod_half * var_lod_half))))) } } else { (assign11470_e6183 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn8) / (var_lod_half * var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11470_e6181).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn9) / (var_lod_half * var_lod_half))))) } } else { (assign11470_e6183 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn9) / (var_lod_half * var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11470_e6181).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn10) / (var_lod_half * var_lod_half))))) } } else { (assign11470_e6183 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn10) / (var_lod_half * var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11470_e6181).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn13) / (var_lod_half * var_lod_half))))) } } else { (assign11470_e6183 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn13) / (var_lod_half * var_lod_half))) / assign11470_e6181))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign11470_e6185;
        var_t2_dn0 = assign11470_e6185_d_n0;
        var_t2_dn2 = assign11470_e6185_d_n2;
        var_t2_dn4 = assign11470_e6185_d_n4;
        var_t2_dn5 = assign11470_e6185_d_n5;
        var_t2_dn6 = assign11470_e6185_d_n6;
        var_t2_dn7 = assign11470_e6185_d_n7;
        var_t2_dn8 = assign11470_e6185_d_n8;
        var_t2_dn9 = assign11470_e6185_d_n9;
        var_t2_dn10 = assign11470_e6185_d_n10;
        var_t2_dn13 = assign11470_e6185_d_n13;
        var_t2_rv = 0.0;

        let (assign11480_e6193, assign11480_e6193_d_n0, assign11480_e6193_d_n2, assign11480_e6193_d_n4, assign11480_e6193_d_n5, assign11480_e6193_d_n6, assign11480_e6193_d_n7, assign11480_e6193_d_n8, assign11480_e6193_d_n9, assign11480_e6193_d_n10, assign11480_e6193_d_n13,) = {
    if (var_guard266 != 0.0) {
        let assign11480_e6189: f64 = (var_uc_nsubpsti1 / var_lod_half_ref);
        let assign11480_e6191: f64 = (assign11480_e6189).powf(var_uc_nsubpsti3);
        (assign11480_e6191, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11480_e6189).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn0) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11480_e6191 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn0) / (var_lod_half_ref * var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11480_e6189).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn2) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11480_e6191 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn2) / (var_lod_half_ref * var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11480_e6189).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn4) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11480_e6191 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn4) / (var_lod_half_ref * var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11480_e6189).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn5) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11480_e6191 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn5) / (var_lod_half_ref * var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11480_e6189).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn6) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11480_e6191 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn6) / (var_lod_half_ref * var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11480_e6189).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn7) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11480_e6191 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn7) / (var_lod_half_ref * var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11480_e6189).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn8) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11480_e6191 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn8) / (var_lod_half_ref * var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11480_e6189).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn9) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11480_e6191 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn9) / (var_lod_half_ref * var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11480_e6189).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn10) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11480_e6191 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn10) / (var_lod_half_ref * var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11480_e6189).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn13) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11480_e6191 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn13) / (var_lod_half_ref * var_lod_half_ref))) / assign11480_e6189))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign11480_e6193;
        var_t3_dn0 = assign11480_e6193_d_n0;
        var_t3_dn2 = assign11480_e6193_d_n2;
        var_t3_dn4 = assign11480_e6193_d_n4;
        var_t3_dn5 = assign11480_e6193_d_n5;
        var_t3_dn6 = assign11480_e6193_d_n6;
        var_t3_dn7 = assign11480_e6193_d_n7;
        var_t3_dn8 = assign11480_e6193_d_n8;
        var_t3_dn9 = assign11480_e6193_d_n9;
        var_t3_dn10 = assign11480_e6193_d_n10;
        var_t3_dn13 = assign11480_e6193_d_n13;
        var_t3_rv = 0.0;

        let (assign11490_e6209, assign11490_e6209_d_n0, assign11490_e6209_d_n2, assign11490_e6209_d_n4, assign11490_e6209_d_n5, assign11490_e6209_d_n6, assign11490_e6209_d_n7, assign11490_e6209_d_n8, assign11490_e6209_d_n9, assign11490_e6209_d_n10, assign11490_e6209_d_n13,) = {
    if (var_guard266 != 0.0) {
        let assign11490_e6199: f64 = (var_t1 * var_t2);
        let assign11490_e6200: f64 = (1.0 + assign11490_e6199);
        let assign11490_e6201: f64 = (var_nsubpp * assign11490_e6200);
        let assign11490_e6205: f64 = (var_t1 * var_t3);
        let assign11490_e6206: f64 = (1.0 + assign11490_e6205);
        let assign11490_e6207: f64 = (assign11490_e6201 / assign11490_e6206);
        (assign11490_e6207, (((((var_nsubpp_dn0 * assign11490_e6200) + (var_nsubpp * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0)))) * assign11490_e6206) - (assign11490_e6201 * ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)))) / (assign11490_e6206 * assign11490_e6206)), (((((var_nsubpp_dn2 * assign11490_e6200) + (var_nsubpp * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2)))) * assign11490_e6206) - (assign11490_e6201 * ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)))) / (assign11490_e6206 * assign11490_e6206)), (((((var_nsubpp_dn4 * assign11490_e6200) + (var_nsubpp * ((var_t1_dn4 * var_t2) + (var_t1 * var_t2_dn4)))) * assign11490_e6206) - (assign11490_e6201 * ((var_t1_dn4 * var_t3) + (var_t1 * var_t3_dn4)))) / (assign11490_e6206 * assign11490_e6206)), (((((var_nsubpp_dn5 * assign11490_e6200) + (var_nsubpp * ((var_t1_dn5 * var_t2) + (var_t1 * var_t2_dn5)))) * assign11490_e6206) - (assign11490_e6201 * ((var_t1_dn5 * var_t3) + (var_t1 * var_t3_dn5)))) / (assign11490_e6206 * assign11490_e6206)), (((((var_nsubpp_dn6 * assign11490_e6200) + (var_nsubpp * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * assign11490_e6206) - (assign11490_e6201 * ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)))) / (assign11490_e6206 * assign11490_e6206)), (((((var_nsubpp_dn7 * assign11490_e6200) + (var_nsubpp * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * assign11490_e6206) - (assign11490_e6201 * ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)))) / (assign11490_e6206 * assign11490_e6206)), (((((var_nsubpp_dn8 * assign11490_e6200) + (var_nsubpp * ((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8)))) * assign11490_e6206) - (assign11490_e6201 * ((var_t1_dn8 * var_t3) + (var_t1 * var_t3_dn8)))) / (assign11490_e6206 * assign11490_e6206)), (((((var_nsubpp_dn9 * assign11490_e6200) + (var_nsubpp * ((var_t1_dn9 * var_t2) + (var_t1 * var_t2_dn9)))) * assign11490_e6206) - (assign11490_e6201 * ((var_t1_dn9 * var_t3) + (var_t1 * var_t3_dn9)))) / (assign11490_e6206 * assign11490_e6206)), (((((var_nsubpp_dn10 * assign11490_e6200) + (var_nsubpp * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10)))) * assign11490_e6206) - (assign11490_e6201 * ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)))) / (assign11490_e6206 * assign11490_e6206)), (((((var_nsubpp_dn13 * assign11490_e6200) + (var_nsubpp * ((var_t1_dn13 * var_t2) + (var_t1 * var_t2_dn13)))) * assign11490_e6206) - (assign11490_e6201 * ((var_t1_dn13 * var_t3) + (var_t1 * var_t3_dn13)))) / (assign11490_e6206 * assign11490_e6206)),)
    } else {
        (var_nsubps, var_nsubps_dn0, var_nsubps_dn2, var_nsubps_dn4, var_nsubps_dn5, var_nsubps_dn6, var_nsubps_dn7, var_nsubps_dn8, var_nsubps_dn9, var_nsubps_dn10, var_nsubps_dn13,)
    }
};
        var_nsubps = assign11490_e6209;
        var_nsubps_dn0 = assign11490_e6209_d_n0;
        var_nsubps_dn2 = assign11490_e6209_d_n2;
        var_nsubps_dn4 = assign11490_e6209_d_n4;
        var_nsubps_dn5 = assign11490_e6209_d_n5;
        var_nsubps_dn6 = assign11490_e6209_d_n6;
        var_nsubps_dn7 = assign11490_e6209_d_n7;
        var_nsubps_dn8 = assign11490_e6209_d_n8;
        var_nsubps_dn9 = assign11490_e6209_d_n9;
        var_nsubps_dn10 = assign11490_e6209_d_n10;
        var_nsubps_dn13 = assign11490_e6209_d_n13;
        var_nsubps_rv = 0.0;

        let (assign11500_e6214, assign11500_e6214_d_n0, assign11500_e6214_d_n2, assign11500_e6214_d_n4, assign11500_e6214_d_n5, assign11500_e6214_d_n6, assign11500_e6214_d_n7, assign11500_e6214_d_n8, assign11500_e6214_d_n9, assign11500_e6214_d_n10, assign11500_e6214_d_n13,) = {
    if (var_guard266 == 0.0) {
        (var_nsubpp, var_nsubpp_dn0, var_nsubpp_dn2, var_nsubpp_dn4, var_nsubpp_dn5, var_nsubpp_dn6, var_nsubpp_dn7, var_nsubpp_dn8, var_nsubpp_dn9, var_nsubpp_dn10, var_nsubpp_dn13,)
    } else {
        (var_nsubps, var_nsubps_dn0, var_nsubps_dn2, var_nsubps_dn4, var_nsubps_dn5, var_nsubps_dn6, var_nsubps_dn7, var_nsubps_dn8, var_nsubps_dn9, var_nsubps_dn10, var_nsubps_dn13,)
    }
};
        var_nsubps = assign11500_e6214;
        var_nsubps_dn0 = assign11500_e6214_d_n0;
        var_nsubps_dn2 = assign11500_e6214_d_n2;
        var_nsubps_dn4 = assign11500_e6214_d_n4;
        var_nsubps_dn5 = assign11500_e6214_d_n5;
        var_nsubps_dn6 = assign11500_e6214_d_n6;
        var_nsubps_dn7 = assign11500_e6214_d_n7;
        var_nsubps_dn8 = assign11500_e6214_d_n8;
        var_nsubps_dn9 = assign11500_e6214_d_n9;
        var_nsubps_dn10 = assign11500_e6214_d_n10;
        var_nsubps_dn13 = assign11500_e6214_d_n13;
        var_nsubps_rv = 0.0;

        let assign11510_e6221: f64 = if ((var_lgate > p.p140) || (p.p140 <= 0.0)) { 1.0 } else { 0.0 };
        var_guard267 = assign11510_e6221;
        var_guard267_rv = 0.0;

        let (assign11520_e6235, assign11520_e6235_d_n0, assign11520_e6235_d_n2, assign11520_e6235_d_n4, assign11520_e6235_d_n5, assign11520_e6235_d_n6, assign11520_e6235_d_n7, assign11520_e6235_d_n8, assign11520_e6235_d_n9, assign11520_e6235_d_n10, assign11520_e6235_d_n13,) = {
    if (var_guard267 != 0.0) {
        let assign11520_e6226: f64 = (var_lgate - p.p140);
        let assign11520_e6227: f64 = (var_ef_nsubc * assign11520_e6226);
        let assign11520_e6230: f64 = (var_nsubps * p.p140);
        let assign11520_e6231: f64 = (assign11520_e6227 + assign11520_e6230);
        let assign11520_e6233: f64 = (assign11520_e6231 / var_lgate);
        (assign11520_e6233, (((var_ef_nsubc_dn0 * assign11520_e6226) + (var_nsubps_dn0 * p.p140)) / var_lgate), (((var_ef_nsubc_dn2 * assign11520_e6226) + (var_nsubps_dn2 * p.p140)) / var_lgate), (((var_ef_nsubc_dn4 * assign11520_e6226) + (var_nsubps_dn4 * p.p140)) / var_lgate), (((var_ef_nsubc_dn5 * assign11520_e6226) + (var_nsubps_dn5 * p.p140)) / var_lgate), (((var_ef_nsubc_dn6 * assign11520_e6226) + (var_nsubps_dn6 * p.p140)) / var_lgate), (((var_ef_nsubc_dn7 * assign11520_e6226) + (var_nsubps_dn7 * p.p140)) / var_lgate), (((var_ef_nsubc_dn8 * assign11520_e6226) + (var_nsubps_dn8 * p.p140)) / var_lgate), (((var_ef_nsubc_dn9 * assign11520_e6226) + (var_nsubps_dn9 * p.p140)) / var_lgate), (((var_ef_nsubc_dn10 * assign11520_e6226) + (var_nsubps_dn10 * p.p140)) / var_lgate), (((var_ef_nsubc_dn13 * assign11520_e6226) + (var_nsubps_dn13 * p.p140)) / var_lgate),)
    } else {
        (var_nsub, var_nsub_dn0, var_nsub_dn2, var_nsub_dn4, var_nsub_dn5, var_nsub_dn6, var_nsub_dn7, var_nsub_dn8, var_nsub_dn9, var_nsub_dn10, var_nsub_dn13,)
    }
};
        var_nsub = assign11520_e6235;
        var_nsub_dn0 = assign11520_e6235_d_n0;
        var_nsub_dn2 = assign11520_e6235_d_n2;
        var_nsub_dn4 = assign11520_e6235_d_n4;
        var_nsub_dn5 = assign11520_e6235_d_n5;
        var_nsub_dn6 = assign11520_e6235_d_n6;
        var_nsub_dn7 = assign11520_e6235_d_n7;
        var_nsub_dn8 = assign11520_e6235_d_n8;
        var_nsub_dn9 = assign11520_e6235_d_n9;
        var_nsub_dn10 = assign11520_e6235_d_n10;
        var_nsub_dn13 = assign11520_e6235_d_n13;
        var_nsub_rv = 0.0;

        let (assign11530_e6250, assign11530_e6250_d_n0, assign11530_e6250_d_n2, assign11530_e6250_d_n4, assign11530_e6250_d_n5, assign11530_e6250_d_n6, assign11530_e6250_d_n7, assign11530_e6250_d_n8, assign11530_e6250_d_n9, assign11530_e6250_d_n10, assign11530_e6250_d_n13,) = {
    if (var_guard267 == 0.0) {
        let assign11530_e6241: f64 = (var_nsubps - var_ef_nsubc);
        let assign11530_e6244: f64 = (p.p140 - var_lgate);
        let assign11530_e6245: f64 = (assign11530_e6241 * assign11530_e6244);
        let assign11530_e6247: f64 = (assign11530_e6245 / p.p140);
        let assign11530_e6248: f64 = (var_nsubps + assign11530_e6247);
        (assign11530_e6248, (var_nsubps_dn0 + (((var_nsubps_dn0 - var_ef_nsubc_dn0) * assign11530_e6244) / p.p140)), (var_nsubps_dn2 + (((var_nsubps_dn2 - var_ef_nsubc_dn2) * assign11530_e6244) / p.p140)), (var_nsubps_dn4 + (((var_nsubps_dn4 - var_ef_nsubc_dn4) * assign11530_e6244) / p.p140)), (var_nsubps_dn5 + (((var_nsubps_dn5 - var_ef_nsubc_dn5) * assign11530_e6244) / p.p140)), (var_nsubps_dn6 + (((var_nsubps_dn6 - var_ef_nsubc_dn6) * assign11530_e6244) / p.p140)), (var_nsubps_dn7 + (((var_nsubps_dn7 - var_ef_nsubc_dn7) * assign11530_e6244) / p.p140)), (var_nsubps_dn8 + (((var_nsubps_dn8 - var_ef_nsubc_dn8) * assign11530_e6244) / p.p140)), (var_nsubps_dn9 + (((var_nsubps_dn9 - var_ef_nsubc_dn9) * assign11530_e6244) / p.p140)), (var_nsubps_dn10 + (((var_nsubps_dn10 - var_ef_nsubc_dn10) * assign11530_e6244) / p.p140)), (var_nsubps_dn13 + (((var_nsubps_dn13 - var_ef_nsubc_dn13) * assign11530_e6244) / p.p140)),)
    } else {
        (var_nsub, var_nsub_dn0, var_nsub_dn2, var_nsub_dn4, var_nsub_dn5, var_nsub_dn6, var_nsub_dn7, var_nsub_dn8, var_nsub_dn9, var_nsub_dn10, var_nsub_dn13,)
    }
};
        var_nsub = assign11530_e6250;
        var_nsub_dn0 = assign11530_e6250_d_n0;
        var_nsub_dn2 = assign11530_e6250_d_n2;
        var_nsub_dn4 = assign11530_e6250_d_n4;
        var_nsub_dn5 = assign11530_e6250_d_n5;
        var_nsub_dn6 = assign11530_e6250_d_n6;
        var_nsub_dn7 = assign11530_e6250_d_n7;
        var_nsub_dn8 = assign11530_e6250_d_n8;
        var_nsub_dn9 = assign11530_e6250_d_n9;
        var_nsub_dn10 = assign11530_e6250_d_n10;
        var_nsub_dn13 = assign11530_e6250_d_n13;
        var_nsub_rv = 0.0;

        let assign11540_e6253: f64 = (0.5 * var_lgate);
        let assign11540_e6255: f64 = (assign11540_e6253 - p.p140);
        var_t3 = assign11540_e6255;
        var_t3_dn0 = 0.0;
        var_t3_dn2 = 0.0;
        var_t3_dn4 = 0.0;
        var_t3_dn5 = 0.0;
        var_t3_dn6 = 0.0;
        var_t3_dn7 = 0.0;
        var_t3_dn8 = 0.0;
        var_t3_dn9 = 0.0;
        var_t3_dn10 = 0.0;
        var_t3_dn13 = 0.0;
        var_t3_rv = 0.0;

        let assign11550_e6258: f64 = (var_t3 - 1e-9);
        let assign11550_e6260: f64 = (assign11550_e6258 - 1e-10);
        var_tmf1 = assign11550_e6260;
        var_tmf1_dn0 = var_t3_dn0;
        var_tmf1_dn2 = var_t3_dn2;
        var_tmf1_dn4 = var_t3_dn4;
        var_tmf1_dn5 = var_t3_dn5;
        var_tmf1_dn6 = var_t3_dn6;
        var_tmf1_dn7 = var_t3_dn7;
        var_tmf1_dn8 = var_t3_dn8;
        var_tmf1_dn9 = var_t3_dn9;
        var_tmf1_dn10 = var_t3_dn10;
        var_tmf1_dn13 = var_t3_dn13;
        var_tmf1_rv = 0.0;

        let assign11560_e6263: f64 = (4.0 * 1e-9);
        let assign11560_e6265: f64 = (assign11560_e6263 * 1e-10);
        var_tmf2 = assign11560_e6265;
        var_tmf2_dn0 = 0.0;
        var_tmf2_dn2 = 0.0;
        var_tmf2_dn4 = 0.0;
        var_tmf2_dn5 = 0.0;
        var_tmf2_dn6 = 0.0;
        var_tmf2_dn7 = 0.0;
        var_tmf2_dn8 = 0.0;
        var_tmf2_dn9 = 0.0;
        var_tmf2_dn10 = 0.0;
        var_tmf2_dn13 = 0.0;
        var_tmf2_rv = 0.0;

        let (assign11570_e6272, assign11570_e6272_d_n0, assign11570_e6272_d_n2, assign11570_e6272_d_n4, assign11570_e6272_d_n5, assign11570_e6272_d_n6, assign11570_e6272_d_n7, assign11570_e6272_d_n8, assign11570_e6272_d_n9, assign11570_e6272_d_n10, assign11570_e6272_d_n13,) = {
    if (var_tmf2 > 0.0) {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    } else {
        let assign11570_e6271: f64 = (-var_tmf2);
        (assign11570_e6271, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn13),)
    }
};
        var_tmf2 = assign11570_e6272;
        var_tmf2_dn0 = assign11570_e6272_d_n0;
        var_tmf2_dn2 = assign11570_e6272_d_n2;
        var_tmf2_dn4 = assign11570_e6272_d_n4;
        var_tmf2_dn5 = assign11570_e6272_d_n5;
        var_tmf2_dn6 = assign11570_e6272_d_n6;
        var_tmf2_dn7 = assign11570_e6272_d_n7;
        var_tmf2_dn8 = assign11570_e6272_d_n8;
        var_tmf2_dn9 = assign11570_e6272_d_n9;
        var_tmf2_dn10 = assign11570_e6272_d_n10;
        var_tmf2_dn13 = assign11570_e6272_d_n13;
        var_tmf2_rv = 0.0;

        let assign11580_e6275: f64 = (var_tmf1 * var_tmf1);
        let assign11580_e6277: f64 = (assign11580_e6275 + var_tmf2);
        let assign11580_e6278: f64 = (assign11580_e6277).sqrt();
        var_tmf2 = assign11580_e6278;
        var_tmf2_dn0 = ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign11580_e6278));
        var_tmf2_dn2 = ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign11580_e6278));
        var_tmf2_dn4 = ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign11580_e6278));
        var_tmf2_dn5 = ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign11580_e6278));
        var_tmf2_dn6 = ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign11580_e6278));
        var_tmf2_dn7 = ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign11580_e6278));
        var_tmf2_dn8 = ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign11580_e6278));
        var_tmf2_dn9 = ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign11580_e6278));
        var_tmf2_dn10 = ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign11580_e6278));
        var_tmf2_dn13 = ((((var_tmf1_dn13 * var_tmf1) + (var_tmf1 * var_tmf1_dn13)) + var_tmf2_dn13) / (2.0 * assign11580_e6278));
        var_tmf2_rv = 0.0;

        let assign11590_e6283: f64 = (var_tmf1 / var_tmf2);
        let assign11590_e6284: f64 = (1.0 + assign11590_e6283);
        let assign11590_e6285: f64 = (0.5 * assign11590_e6284);
        var_t0 = assign11590_e6285;
        var_t0_dn0 = (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2)));
        var_t0_dn2 = (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2)));
        var_t0_dn4 = (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2)));
        var_t0_dn5 = (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2)));
        var_t0_dn6 = (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2)));
        var_t0_dn7 = (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2)));
        var_t0_dn8 = (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2)));
        var_t0_dn9 = (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2)));
        var_t0_dn10 = (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2)));
        var_t0_dn13 = (0.5 * (((var_tmf1_dn13 * var_tmf2) - (var_tmf1 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2)));
        var_t0_rv = 0.0;

        let assign11600_e6290: f64 = (var_tmf1 + var_tmf2);
        let assign11600_e6291: f64 = (0.5 * assign11600_e6290);
        let assign11600_e6292: f64 = (1e-9 + assign11600_e6291);
        var_t3 = assign11600_e6292;
        var_t3_dn0 = (0.5 * (var_tmf1_dn0 + var_tmf2_dn0));
        var_t3_dn2 = (0.5 * (var_tmf1_dn2 + var_tmf2_dn2));
        var_t3_dn4 = (0.5 * (var_tmf1_dn4 + var_tmf2_dn4));
        var_t3_dn5 = (0.5 * (var_tmf1_dn5 + var_tmf2_dn5));
        var_t3_dn6 = (0.5 * (var_tmf1_dn6 + var_tmf2_dn6));
        var_t3_dn7 = (0.5 * (var_tmf1_dn7 + var_tmf2_dn7));
        var_t3_dn8 = (0.5 * (var_tmf1_dn8 + var_tmf2_dn8));
        var_t3_dn9 = (0.5 * (var_tmf1_dn9 + var_tmf2_dn9));
        var_t3_dn10 = (0.5 * (var_tmf1_dn10 + var_tmf2_dn10));
        var_t3_dn13 = (0.5 * (var_tmf1_dn13 + var_tmf2_dn13));
        var_t3_rv = 0.0;

        let assign11610_e6296: f64 = (1.0 / var_t3);
        let assign11610_e6299: f64 = (1.0 / p.p220);
        let assign11610_e6300: f64 = (assign11610_e6296 + assign11610_e6299);
        let assign11610_e6301: f64 = (1.0 / assign11610_e6300);
        var_t1 = assign11610_e6301;
        var_t1_dn0 = (-((-(var_t3_dn0 / (var_t3 * var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        var_t1_dn2 = (-((-(var_t3_dn2 / (var_t3 * var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        var_t1_dn4 = (-((-(var_t3_dn4 / (var_t3 * var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        var_t1_dn5 = (-((-(var_t3_dn5 / (var_t3 * var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        var_t1_dn6 = (-((-(var_t3_dn6 / (var_t3 * var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        var_t1_dn7 = (-((-(var_t3_dn7 / (var_t3 * var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        var_t1_dn8 = (-((-(var_t3_dn8 / (var_t3 * var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        var_t1_dn9 = (-((-(var_t3_dn9 / (var_t3 * var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        var_t1_dn10 = (-((-(var_t3_dn10 / (var_t3 * var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        var_t1_dn13 = (-((-(var_t3_dn13 / (var_t3 * var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        var_t1_rv = 0.0;

        let (assign11620_e6307, assign11620_e6307_d_n0, assign11620_e6307_d_n2, assign11620_e6307_d_n4, assign11620_e6307_d_n5, assign11620_e6307_d_n6, assign11620_e6307_d_n7, assign11620_e6307_d_n8, assign11620_e6307_d_n9, assign11620_e6307_d_n10, assign11620_e6307_d_n13,) = {
    if (0.0 >= var_t1) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t2 = assign11620_e6307;
        var_t2_dn0 = assign11620_e6307_d_n0;
        var_t2_dn2 = assign11620_e6307_d_n2;
        var_t2_dn4 = assign11620_e6307_d_n4;
        var_t2_dn5 = assign11620_e6307_d_n5;
        var_t2_dn6 = assign11620_e6307_d_n6;
        var_t2_dn7 = assign11620_e6307_d_n7;
        var_t2_dn8 = assign11620_e6307_d_n8;
        var_t2_dn9 = assign11620_e6307_d_n9;
        var_t2_dn10 = assign11620_e6307_d_n10;
        var_t2_dn13 = assign11620_e6307_d_n13;
        var_t2_rv = 0.0;

        let assign11630_e6312: f64 = (var_npexte - var_ef_nsubc);
        let assign11630_e6313: f64 = (var_t2 * assign11630_e6312);
        let assign11630_e6315: f64 = (assign11630_e6313 / var_lgate);
        let assign11630_e6316: f64 = (var_nsub + assign11630_e6315);
        var_nsub = assign11630_e6316;
        var_nsub_dn0 = (var_nsub_dn0 + (((var_t2_dn0 * assign11630_e6312) + (var_t2 * (var_npexte_dn0 - var_ef_nsubc_dn0))) / var_lgate));
        var_nsub_dn2 = (var_nsub_dn2 + (((var_t2_dn2 * assign11630_e6312) + (var_t2 * (var_npexte_dn2 - var_ef_nsubc_dn2))) / var_lgate));
        var_nsub_dn4 = (var_nsub_dn4 + (((var_t2_dn4 * assign11630_e6312) + (var_t2 * (var_npexte_dn4 - var_ef_nsubc_dn4))) / var_lgate));
        var_nsub_dn5 = (var_nsub_dn5 + (((var_t2_dn5 * assign11630_e6312) + (var_t2 * (var_npexte_dn5 - var_ef_nsubc_dn5))) / var_lgate));
        var_nsub_dn6 = (var_nsub_dn6 + (((var_t2_dn6 * assign11630_e6312) + (var_t2 * (var_npexte_dn6 - var_ef_nsubc_dn6))) / var_lgate));
        var_nsub_dn7 = (var_nsub_dn7 + (((var_t2_dn7 * assign11630_e6312) + (var_t2 * (var_npexte_dn7 - var_ef_nsubc_dn7))) / var_lgate));
        var_nsub_dn8 = (var_nsub_dn8 + (((var_t2_dn8 * assign11630_e6312) + (var_t2 * (var_npexte_dn8 - var_ef_nsubc_dn8))) / var_lgate));
        var_nsub_dn9 = (var_nsub_dn9 + (((var_t2_dn9 * assign11630_e6312) + (var_t2 * (var_npexte_dn9 - var_ef_nsubc_dn9))) / var_lgate));
        var_nsub_dn10 = (var_nsub_dn10 + (((var_t2_dn10 * assign11630_e6312) + (var_t2 * (var_npexte_dn10 - var_ef_nsubc_dn10))) / var_lgate));
        var_nsub_dn13 = (var_nsub_dn13 + (((var_t2_dn13 * assign11630_e6312) + (var_t2 * (var_npexte_dn13 - var_ef_nsubc_dn13))) / var_lgate));
        var_nsub_rv = 0.0;

        let assign11640_e6319: f64 = (1.6021918e-19 * var_nsub);
        var_q_nsub = assign11640_e6319;
        var_q_nsub_dn0 = (1.6021918e-19 * var_nsub_dn0);
        var_q_nsub_dn2 = (1.6021918e-19 * var_nsub_dn2);
        var_q_nsub_dn4 = (1.6021918e-19 * var_nsub_dn4);
        var_q_nsub_dn5 = (1.6021918e-19 * var_nsub_dn5);
        var_q_nsub_dn6 = (1.6021918e-19 * var_nsub_dn6);
        var_q_nsub_dn7 = (1.6021918e-19 * var_nsub_dn7);
        var_q_nsub_dn8 = (1.6021918e-19 * var_nsub_dn8);
        var_q_nsub_dn9 = (1.6021918e-19 * var_nsub_dn9);
        var_q_nsub_dn10 = (1.6021918e-19 * var_nsub_dn10);
        var_q_nsub_dn13 = (1.6021918e-19 * var_nsub_dn13);
        var_q_nsub_rv = 0.0;

        let assign11650_e6322: f64 = (var_q_nsub * 1.034943e-10);
        var_qnsub_esi = assign11650_e6322;
        var_qnsub_esi_dn0 = (var_q_nsub_dn0 * 1.034943e-10);
        var_qnsub_esi_dn2 = (var_q_nsub_dn2 * 1.034943e-10);
        var_qnsub_esi_dn4 = (var_q_nsub_dn4 * 1.034943e-10);
        var_qnsub_esi_dn5 = (var_q_nsub_dn5 * 1.034943e-10);
        var_qnsub_esi_dn6 = (var_q_nsub_dn6 * 1.034943e-10);
        var_qnsub_esi_dn7 = (var_q_nsub_dn7 * 1.034943e-10);
        var_qnsub_esi_dn8 = (var_q_nsub_dn8 * 1.034943e-10);
        var_qnsub_esi_dn9 = (var_q_nsub_dn9 * 1.034943e-10);
        var_qnsub_esi_dn10 = (var_q_nsub_dn10 * 1.034943e-10);
        var_qnsub_esi_dn13 = (var_q_nsub_dn13 * 1.034943e-10);
        var_qnsub_esi_rv = 0.0;

        let assign11660_e6325: f64 = (2.0 * var_qnsub_esi);
        var_qnsub_esi2 = assign11660_e6325;
        var_qnsub_esi2_dn0 = (2.0 * var_qnsub_esi_dn0);
        var_qnsub_esi2_dn2 = (2.0 * var_qnsub_esi_dn2);
        var_qnsub_esi2_dn4 = (2.0 * var_qnsub_esi_dn4);
        var_qnsub_esi2_dn5 = (2.0 * var_qnsub_esi_dn5);
        var_qnsub_esi2_dn6 = (2.0 * var_qnsub_esi_dn6);
        var_qnsub_esi2_dn7 = (2.0 * var_qnsub_esi_dn7);
        var_qnsub_esi2_dn8 = (2.0 * var_qnsub_esi_dn8);
        var_qnsub_esi2_dn9 = (2.0 * var_qnsub_esi_dn9);
        var_qnsub_esi2_dn10 = (2.0 * var_qnsub_esi_dn10);
        var_qnsub_esi2_dn13 = (2.0 * var_qnsub_esi_dn13);
        var_qnsub_esi2_rv = 0.0;

        let assign11670_e6329: f64 = (2.0 * p.p140);
        let assign11670_e6334: f64 = if ((var_lgate <= assign11670_e6329) && (p.p140 > 0.0)) { 1.0 } else { 0.0 };
        var_guard268 = assign11670_e6334;
        var_guard268_rv = 0.0;

        *var_ef_nsubc_slot = var_ef_nsubc;
        *var_ef_nsubc_dn0_slot = var_ef_nsubc_dn0;
        *var_ef_nsubc_dn10_slot = var_ef_nsubc_dn10;
        *var_ef_nsubc_dn13_slot = var_ef_nsubc_dn13;
        *var_ef_nsubc_dn2_slot = var_ef_nsubc_dn2;
        *var_ef_nsubc_dn4_slot = var_ef_nsubc_dn4;
        *var_ef_nsubc_dn5_slot = var_ef_nsubc_dn5;
        *var_ef_nsubc_dn6_slot = var_ef_nsubc_dn6;
        *var_ef_nsubc_dn7_slot = var_ef_nsubc_dn7;
        *var_ef_nsubc_dn8_slot = var_ef_nsubc_dn8;
        *var_ef_nsubc_dn9_slot = var_ef_nsubc_dn9;
        *var_ef_nsubc_rv_slot = var_ef_nsubc_rv;
        *var_guard263_slot = var_guard263;
        *var_guard263_rv_slot = var_guard263_rv;
        *var_guard265_slot = var_guard265;
        *var_guard265_rv_slot = var_guard265_rv;
        *var_guard266_slot = var_guard266;
        *var_guard266_rv_slot = var_guard266_rv;
        *var_guard267_slot = var_guard267;
        *var_guard267_rv_slot = var_guard267_rv;
        *var_guard268_slot = var_guard268;
        *var_guard268_rv_slot = var_guard268_rv;
        *var_nsub_slot = var_nsub;
        *var_nsub_dn0_slot = var_nsub_dn0;
        *var_nsub_dn10_slot = var_nsub_dn10;
        *var_nsub_dn13_slot = var_nsub_dn13;
        *var_nsub_dn2_slot = var_nsub_dn2;
        *var_nsub_dn4_slot = var_nsub_dn4;
        *var_nsub_dn5_slot = var_nsub_dn5;
        *var_nsub_dn6_slot = var_nsub_dn6;
        *var_nsub_dn7_slot = var_nsub_dn7;
        *var_nsub_dn8_slot = var_nsub_dn8;
        *var_nsub_dn9_slot = var_nsub_dn9;
        *var_nsub_rv_slot = var_nsub_rv;
        *var_nsubpp_slot = var_nsubpp;
        *var_nsubpp_dn0_slot = var_nsubpp_dn0;
        *var_nsubpp_dn10_slot = var_nsubpp_dn10;
        *var_nsubpp_dn13_slot = var_nsubpp_dn13;
        *var_nsubpp_dn2_slot = var_nsubpp_dn2;
        *var_nsubpp_dn4_slot = var_nsubpp_dn4;
        *var_nsubpp_dn5_slot = var_nsubpp_dn5;
        *var_nsubpp_dn6_slot = var_nsubpp_dn6;
        *var_nsubpp_dn7_slot = var_nsubpp_dn7;
        *var_nsubpp_dn8_slot = var_nsubpp_dn8;
        *var_nsubpp_dn9_slot = var_nsubpp_dn9;
        *var_nsubpp_rv_slot = var_nsubpp_rv;
        *var_nsubps_slot = var_nsubps;
        *var_nsubps_dn0_slot = var_nsubps_dn0;
        *var_nsubps_dn10_slot = var_nsubps_dn10;
        *var_nsubps_dn13_slot = var_nsubps_dn13;
        *var_nsubps_dn2_slot = var_nsubps_dn2;
        *var_nsubps_dn4_slot = var_nsubps_dn4;
        *var_nsubps_dn5_slot = var_nsubps_dn5;
        *var_nsubps_dn6_slot = var_nsubps_dn6;
        *var_nsubps_dn7_slot = var_nsubps_dn7;
        *var_nsubps_dn8_slot = var_nsubps_dn8;
        *var_nsubps_dn9_slot = var_nsubps_dn9;
        *var_nsubps_rv_slot = var_nsubps_rv;
        *var_q_nsub_slot = var_q_nsub;
        *var_q_nsub_dn0_slot = var_q_nsub_dn0;
        *var_q_nsub_dn10_slot = var_q_nsub_dn10;
        *var_q_nsub_dn13_slot = var_q_nsub_dn13;
        *var_q_nsub_dn2_slot = var_q_nsub_dn2;
        *var_q_nsub_dn4_slot = var_q_nsub_dn4;
        *var_q_nsub_dn5_slot = var_q_nsub_dn5;
        *var_q_nsub_dn6_slot = var_q_nsub_dn6;
        *var_q_nsub_dn7_slot = var_q_nsub_dn7;
        *var_q_nsub_dn8_slot = var_q_nsub_dn8;
        *var_q_nsub_dn9_slot = var_q_nsub_dn9;
        *var_q_nsub_rv_slot = var_q_nsub_rv;
        *var_qnsub_esi_slot = var_qnsub_esi;
        *var_qnsub_esi2_slot = var_qnsub_esi2;
        *var_qnsub_esi2_dn0_slot = var_qnsub_esi2_dn0;
        *var_qnsub_esi2_dn10_slot = var_qnsub_esi2_dn10;
        *var_qnsub_esi2_dn13_slot = var_qnsub_esi2_dn13;
        *var_qnsub_esi2_dn2_slot = var_qnsub_esi2_dn2;
        *var_qnsub_esi2_dn4_slot = var_qnsub_esi2_dn4;
        *var_qnsub_esi2_dn5_slot = var_qnsub_esi2_dn5;
        *var_qnsub_esi2_dn6_slot = var_qnsub_esi2_dn6;
        *var_qnsub_esi2_dn7_slot = var_qnsub_esi2_dn7;
        *var_qnsub_esi2_dn8_slot = var_qnsub_esi2_dn8;
        *var_qnsub_esi2_dn9_slot = var_qnsub_esi2_dn9;
        *var_qnsub_esi2_rv_slot = var_qnsub_esi2_rv;
        *var_qnsub_esi_dn0_slot = var_qnsub_esi_dn0;
        *var_qnsub_esi_dn10_slot = var_qnsub_esi_dn10;
        *var_qnsub_esi_dn13_slot = var_qnsub_esi_dn13;
        *var_qnsub_esi_dn2_slot = var_qnsub_esi_dn2;
        *var_qnsub_esi_dn4_slot = var_qnsub_esi_dn4;
        *var_qnsub_esi_dn5_slot = var_qnsub_esi_dn5;
        *var_qnsub_esi_dn6_slot = var_qnsub_esi_dn6;
        *var_qnsub_esi_dn7_slot = var_qnsub_esi_dn7;
        *var_qnsub_esi_dn8_slot = var_qnsub_esi_dn8;
        *var_qnsub_esi_dn9_slot = var_qnsub_esi_dn9;
        *var_qnsub_esi_rv_slot = var_qnsub_esi_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn13_slot = var_tmf1_dn13;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_dn9_slot = var_tmf1_dn9;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn13_slot = var_tmf2_dn13;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        var_ef_nsubc: f64,
        var_ef_nsubc_dn0: f64,
        var_ef_nsubc_dn10: f64,
        var_ef_nsubc_dn13: f64,
        var_ef_nsubc_dn2: f64,
        var_ef_nsubc_dn4: f64,
        var_ef_nsubc_dn5: f64,
        var_ef_nsubc_dn6: f64,
        var_ef_nsubc_dn7: f64,
        var_ef_nsubc_dn8: f64,
        var_ef_nsubc_dn9: f64,
        var_guard268: f64,
        var_lg: f64,
        var_lgate: f64,
        var_nsub: f64,
        var_nsub_dn0: f64,
        var_nsub_dn10: f64,
        var_nsub_dn13: f64,
        var_nsub_dn2: f64,
        var_nsub_dn4: f64,
        var_nsub_dn5: f64,
        var_nsub_dn6: f64,
        var_nsub_dn7: f64,
        var_nsub_dn8: f64,
        var_nsub_dn9: f64,
        var_nsubps: f64,
        var_nsubps_dn0: f64,
        var_nsubps_dn10: f64,
        var_nsubps_dn13: f64,
        var_nsubps_dn2: f64,
        var_nsubps_dn4: f64,
        var_nsubps_dn5: f64,
        var_nsubps_dn6: f64,
        var_nsubps_dn7: f64,
        var_nsubps_dn8: f64,
        var_nsubps_dn9: f64,
        var_uc_cordrift: f64,
        var_uc_nsti: f64,
        var_uc_rd: f64,
        var_uc_rd23: f64,
        var_uc_rdvd: f64,
        var_uc_rs: f64,
        var_uc_vover: f64,
        var_weff: f64,
        var_wlg: f64,
        var_costi00_slot: &mut f64,
        var_costi00_rv_slot: &mut f64,
        var_guard269_slot: &mut f64,
        var_guard269_rv_slot: &mut f64,
        var_guard271_slot: &mut f64,
        var_guard271_rv_slot: &mut f64,
        var_guard272_slot: &mut f64,
        var_guard272_rv_slot: &mut f64,
        var_guard273_slot: &mut f64,
        var_guard273_rv_slot: &mut f64,
        var_guard274_slot: &mut f64,
        var_guard274_rv_slot: &mut f64,
        var_guard275_slot: &mut f64,
        var_guard275_rv_slot: &mut f64,
        var_guard276_slot: &mut f64,
        var_guard276_rv_slot: &mut f64,
        var_guard277_slot: &mut f64,
        var_guard277_rv_slot: &mut f64,
        var_guard278_slot: &mut f64,
        var_guard278_rv_slot: &mut f64,
        var_nsti_p2_slot: &mut f64,
        var_nsti_p2_rv_slot: &mut f64,
        var_nsubb_slot: &mut f64,
        var_nsubb_dn0_slot: &mut f64,
        var_nsubb_dn10_slot: &mut f64,
        var_nsubb_dn13_slot: &mut f64,
        var_nsubb_dn2_slot: &mut f64,
        var_nsubb_dn4_slot: &mut f64,
        var_nsubb_dn5_slot: &mut f64,
        var_nsubb_dn6_slot: &mut f64,
        var_nsubb_dn7_slot: &mut f64,
        var_nsubb_dn8_slot: &mut f64,
        var_nsubb_dn9_slot: &mut f64,
        var_nsubb_rv_slot: &mut f64,
        var_pb20_slot: &mut f64,
        var_pb20_dn0_slot: &mut f64,
        var_pb20_dn10_slot: &mut f64,
        var_pb20_dn13_slot: &mut f64,
        var_pb20_dn2_slot: &mut f64,
        var_pb20_dn4_slot: &mut f64,
        var_pb20_dn5_slot: &mut f64,
        var_pb20_dn6_slot: &mut f64,
        var_pb20_dn7_slot: &mut f64,
        var_pb20_dn8_slot: &mut f64,
        var_pb20_dn9_slot: &mut f64,
        var_pb20_rv_slot: &mut f64,
        var_pb2c_slot: &mut f64,
        var_pb2c_dn0_slot: &mut f64,
        var_pb2c_dn10_slot: &mut f64,
        var_pb2c_dn13_slot: &mut f64,
        var_pb2c_dn2_slot: &mut f64,
        var_pb2c_dn4_slot: &mut f64,
        var_pb2c_dn5_slot: &mut f64,
        var_pb2c_dn6_slot: &mut f64,
        var_pb2c_dn7_slot: &mut f64,
        var_pb2c_dn8_slot: &mut f64,
        var_pb2c_dn9_slot: &mut f64,
        var_pb2c_rv_slot: &mut f64,
        var_ptovr0_slot: &mut f64,
        var_ptovr0_dn0_slot: &mut f64,
        var_ptovr0_dn10_slot: &mut f64,
        var_ptovr0_dn13_slot: &mut f64,
        var_ptovr0_dn2_slot: &mut f64,
        var_ptovr0_dn4_slot: &mut f64,
        var_ptovr0_dn5_slot: &mut f64,
        var_ptovr0_dn6_slot: &mut f64,
        var_ptovr0_dn7_slot: &mut f64,
        var_ptovr0_dn8_slot: &mut f64,
        var_ptovr0_dn9_slot: &mut f64,
        var_ptovr0_rv_slot: &mut f64,
        var_rd0_slot: &mut f64,
        var_rd0_rv_slot: &mut f64,
        var_rdtemp0_slot: &mut f64,
        var_rdtemp0_rv_slot: &mut f64,
        var_rdvdtemp0_slot: &mut f64,
        var_rdvdtemp0_dn0_slot: &mut f64,
        var_rdvdtemp0_dn10_slot: &mut f64,
        var_rdvdtemp0_dn13_slot: &mut f64,
        var_rdvdtemp0_dn2_slot: &mut f64,
        var_rdvdtemp0_dn4_slot: &mut f64,
        var_rdvdtemp0_dn5_slot: &mut f64,
        var_rdvdtemp0_dn6_slot: &mut f64,
        var_rdvdtemp0_dn7_slot: &mut f64,
        var_rdvdtemp0_dn8_slot: &mut f64,
        var_rdvdtemp0_dn9_slot: &mut f64,
        var_rdvdtemp0_rv_slot: &mut f64,
        var_rs0_slot: &mut f64,
        var_rs0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn13_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn13_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_vmax0_slot: &mut f64,
        var_vmax0_rv_slot: &mut f64,
    ) {
        let mut var_costi00: f64 = *var_costi00_slot;
        let mut var_costi00_rv: f64 = *var_costi00_rv_slot;
        let mut var_guard269: f64 = *var_guard269_slot;
        let mut var_guard269_rv: f64 = *var_guard269_rv_slot;
        let mut var_guard271: f64 = *var_guard271_slot;
        let mut var_guard271_rv: f64 = *var_guard271_rv_slot;
        let mut var_guard272: f64 = *var_guard272_slot;
        let mut var_guard272_rv: f64 = *var_guard272_rv_slot;
        let mut var_guard273: f64 = *var_guard273_slot;
        let mut var_guard273_rv: f64 = *var_guard273_rv_slot;
        let mut var_guard274: f64 = *var_guard274_slot;
        let mut var_guard274_rv: f64 = *var_guard274_rv_slot;
        let mut var_guard275: f64 = *var_guard275_slot;
        let mut var_guard275_rv: f64 = *var_guard275_rv_slot;
        let mut var_guard276: f64 = *var_guard276_slot;
        let mut var_guard276_rv: f64 = *var_guard276_rv_slot;
        let mut var_guard277: f64 = *var_guard277_slot;
        let mut var_guard277_rv: f64 = *var_guard277_rv_slot;
        let mut var_guard278: f64 = *var_guard278_slot;
        let mut var_guard278_rv: f64 = *var_guard278_rv_slot;
        let mut var_nsti_p2: f64 = *var_nsti_p2_slot;
        let mut var_nsti_p2_rv: f64 = *var_nsti_p2_rv_slot;
        let mut var_nsubb: f64 = *var_nsubb_slot;
        let mut var_nsubb_dn0: f64 = *var_nsubb_dn0_slot;
        let mut var_nsubb_dn10: f64 = *var_nsubb_dn10_slot;
        let mut var_nsubb_dn13: f64 = *var_nsubb_dn13_slot;
        let mut var_nsubb_dn2: f64 = *var_nsubb_dn2_slot;
        let mut var_nsubb_dn4: f64 = *var_nsubb_dn4_slot;
        let mut var_nsubb_dn5: f64 = *var_nsubb_dn5_slot;
        let mut var_nsubb_dn6: f64 = *var_nsubb_dn6_slot;
        let mut var_nsubb_dn7: f64 = *var_nsubb_dn7_slot;
        let mut var_nsubb_dn8: f64 = *var_nsubb_dn8_slot;
        let mut var_nsubb_dn9: f64 = *var_nsubb_dn9_slot;
        let mut var_nsubb_rv: f64 = *var_nsubb_rv_slot;
        let mut var_pb20: f64 = *var_pb20_slot;
        let mut var_pb20_dn0: f64 = *var_pb20_dn0_slot;
        let mut var_pb20_dn10: f64 = *var_pb20_dn10_slot;
        let mut var_pb20_dn13: f64 = *var_pb20_dn13_slot;
        let mut var_pb20_dn2: f64 = *var_pb20_dn2_slot;
        let mut var_pb20_dn4: f64 = *var_pb20_dn4_slot;
        let mut var_pb20_dn5: f64 = *var_pb20_dn5_slot;
        let mut var_pb20_dn6: f64 = *var_pb20_dn6_slot;
        let mut var_pb20_dn7: f64 = *var_pb20_dn7_slot;
        let mut var_pb20_dn8: f64 = *var_pb20_dn8_slot;
        let mut var_pb20_dn9: f64 = *var_pb20_dn9_slot;
        let mut var_pb20_rv: f64 = *var_pb20_rv_slot;
        let mut var_pb2c: f64 = *var_pb2c_slot;
        let mut var_pb2c_dn0: f64 = *var_pb2c_dn0_slot;
        let mut var_pb2c_dn10: f64 = *var_pb2c_dn10_slot;
        let mut var_pb2c_dn13: f64 = *var_pb2c_dn13_slot;
        let mut var_pb2c_dn2: f64 = *var_pb2c_dn2_slot;
        let mut var_pb2c_dn4: f64 = *var_pb2c_dn4_slot;
        let mut var_pb2c_dn5: f64 = *var_pb2c_dn5_slot;
        let mut var_pb2c_dn6: f64 = *var_pb2c_dn6_slot;
        let mut var_pb2c_dn7: f64 = *var_pb2c_dn7_slot;
        let mut var_pb2c_dn8: f64 = *var_pb2c_dn8_slot;
        let mut var_pb2c_dn9: f64 = *var_pb2c_dn9_slot;
        let mut var_pb2c_rv: f64 = *var_pb2c_rv_slot;
        let mut var_ptovr0: f64 = *var_ptovr0_slot;
        let mut var_ptovr0_dn0: f64 = *var_ptovr0_dn0_slot;
        let mut var_ptovr0_dn10: f64 = *var_ptovr0_dn10_slot;
        let mut var_ptovr0_dn13: f64 = *var_ptovr0_dn13_slot;
        let mut var_ptovr0_dn2: f64 = *var_ptovr0_dn2_slot;
        let mut var_ptovr0_dn4: f64 = *var_ptovr0_dn4_slot;
        let mut var_ptovr0_dn5: f64 = *var_ptovr0_dn5_slot;
        let mut var_ptovr0_dn6: f64 = *var_ptovr0_dn6_slot;
        let mut var_ptovr0_dn7: f64 = *var_ptovr0_dn7_slot;
        let mut var_ptovr0_dn8: f64 = *var_ptovr0_dn8_slot;
        let mut var_ptovr0_dn9: f64 = *var_ptovr0_dn9_slot;
        let mut var_ptovr0_rv: f64 = *var_ptovr0_rv_slot;
        let mut var_rd0: f64 = *var_rd0_slot;
        let mut var_rd0_rv: f64 = *var_rd0_rv_slot;
        let mut var_rdtemp0: f64 = *var_rdtemp0_slot;
        let mut var_rdtemp0_rv: f64 = *var_rdtemp0_rv_slot;
        let mut var_rdvdtemp0: f64 = *var_rdvdtemp0_slot;
        let mut var_rdvdtemp0_dn0: f64 = *var_rdvdtemp0_dn0_slot;
        let mut var_rdvdtemp0_dn10: f64 = *var_rdvdtemp0_dn10_slot;
        let mut var_rdvdtemp0_dn13: f64 = *var_rdvdtemp0_dn13_slot;
        let mut var_rdvdtemp0_dn2: f64 = *var_rdvdtemp0_dn2_slot;
        let mut var_rdvdtemp0_dn4: f64 = *var_rdvdtemp0_dn4_slot;
        let mut var_rdvdtemp0_dn5: f64 = *var_rdvdtemp0_dn5_slot;
        let mut var_rdvdtemp0_dn6: f64 = *var_rdvdtemp0_dn6_slot;
        let mut var_rdvdtemp0_dn7: f64 = *var_rdvdtemp0_dn7_slot;
        let mut var_rdvdtemp0_dn8: f64 = *var_rdvdtemp0_dn8_slot;
        let mut var_rdvdtemp0_dn9: f64 = *var_rdvdtemp0_dn9_slot;
        let mut var_rdvdtemp0_rv: f64 = *var_rdvdtemp0_rv_slot;
        let mut var_rs0: f64 = *var_rs0_slot;
        let mut var_rs0_rv: f64 = *var_rs0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn13: f64 = *var_t7_dn13_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_vmax0: f64 = *var_vmax0_slot;
        let mut var_vmax0_rv: f64 = *var_vmax0_rv_slot;

        let (assign11680_e6350, assign11680_e6350_d_n0, assign11680_e6350_d_n2, assign11680_e6350_d_n4, assign11680_e6350_d_n5, assign11680_e6350_d_n6, assign11680_e6350_d_n7, assign11680_e6350_d_n8, assign11680_e6350_d_n9, assign11680_e6350_d_n10, assign11680_e6350_d_n13,) = {
    if (var_guard268 != 0.0) {
        let assign11680_e6338: f64 = (2.0 * var_nsubps);
        let assign11680_e6341: f64 = (var_nsubps - var_ef_nsubc);
        let assign11680_e6343: f64 = (assign11680_e6341 * var_lgate);
        let assign11680_e6345: f64 = (assign11680_e6343 / p.p140);
        let assign11680_e6346: f64 = (assign11680_e6338 - assign11680_e6345);
        let assign11680_e6348: f64 = (assign11680_e6346 - var_ef_nsubc);
        (assign11680_e6348, (((2.0 * var_nsubps_dn0) - (((var_nsubps_dn0 - var_ef_nsubc_dn0) * var_lgate) / p.p140)) - var_ef_nsubc_dn0), (((2.0 * var_nsubps_dn2) - (((var_nsubps_dn2 - var_ef_nsubc_dn2) * var_lgate) / p.p140)) - var_ef_nsubc_dn2), (((2.0 * var_nsubps_dn4) - (((var_nsubps_dn4 - var_ef_nsubc_dn4) * var_lgate) / p.p140)) - var_ef_nsubc_dn4), (((2.0 * var_nsubps_dn5) - (((var_nsubps_dn5 - var_ef_nsubc_dn5) * var_lgate) / p.p140)) - var_ef_nsubc_dn5), (((2.0 * var_nsubps_dn6) - (((var_nsubps_dn6 - var_ef_nsubc_dn6) * var_lgate) / p.p140)) - var_ef_nsubc_dn6), (((2.0 * var_nsubps_dn7) - (((var_nsubps_dn7 - var_ef_nsubc_dn7) * var_lgate) / p.p140)) - var_ef_nsubc_dn7), (((2.0 * var_nsubps_dn8) - (((var_nsubps_dn8 - var_ef_nsubc_dn8) * var_lgate) / p.p140)) - var_ef_nsubc_dn8), (((2.0 * var_nsubps_dn9) - (((var_nsubps_dn9 - var_ef_nsubc_dn9) * var_lgate) / p.p140)) - var_ef_nsubc_dn9), (((2.0 * var_nsubps_dn10) - (((var_nsubps_dn10 - var_ef_nsubc_dn10) * var_lgate) / p.p140)) - var_ef_nsubc_dn10), (((2.0 * var_nsubps_dn13) - (((var_nsubps_dn13 - var_ef_nsubc_dn13) * var_lgate) / p.p140)) - var_ef_nsubc_dn13),)
    } else {
        (var_nsubb, var_nsubb_dn0, var_nsubb_dn2, var_nsubb_dn4, var_nsubb_dn5, var_nsubb_dn6, var_nsubb_dn7, var_nsubb_dn8, var_nsubb_dn9, var_nsubb_dn10, var_nsubb_dn13,)
    }
};
        var_nsubb = assign11680_e6350;
        var_nsubb_dn0 = assign11680_e6350_d_n0;
        var_nsubb_dn2 = assign11680_e6350_d_n2;
        var_nsubb_dn4 = assign11680_e6350_d_n4;
        var_nsubb_dn5 = assign11680_e6350_d_n5;
        var_nsubb_dn6 = assign11680_e6350_d_n6;
        var_nsubb_dn7 = assign11680_e6350_d_n7;
        var_nsubb_dn8 = assign11680_e6350_d_n8;
        var_nsubb_dn9 = assign11680_e6350_d_n9;
        var_nsubb_dn10 = assign11680_e6350_d_n10;
        var_nsubb_dn13 = assign11680_e6350_d_n13;
        var_nsubb_rv = 0.0;

        let (assign11690_e6357, assign11690_e6357_d_n0, assign11690_e6357_d_n2, assign11690_e6357_d_n4, assign11690_e6357_d_n5, assign11690_e6357_d_n6, assign11690_e6357_d_n7, assign11690_e6357_d_n8, assign11690_e6357_d_n9, assign11690_e6357_d_n10, assign11690_e6357_d_n13,) = {
    if (var_guard268 != 0.0) {
        let assign11690_e6354: f64 = (var_nsubb / var_ef_nsubc);
        let assign11690_e6355: f64 = (assign11690_e6354).ln();
        (assign11690_e6355, ((((var_nsubb_dn0 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn0)) / (var_ef_nsubc * var_ef_nsubc)) / assign11690_e6354), ((((var_nsubb_dn2 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn2)) / (var_ef_nsubc * var_ef_nsubc)) / assign11690_e6354), ((((var_nsubb_dn4 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn4)) / (var_ef_nsubc * var_ef_nsubc)) / assign11690_e6354), ((((var_nsubb_dn5 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn5)) / (var_ef_nsubc * var_ef_nsubc)) / assign11690_e6354), ((((var_nsubb_dn6 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn6)) / (var_ef_nsubc * var_ef_nsubc)) / assign11690_e6354), ((((var_nsubb_dn7 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn7)) / (var_ef_nsubc * var_ef_nsubc)) / assign11690_e6354), ((((var_nsubb_dn8 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn8)) / (var_ef_nsubc * var_ef_nsubc)) / assign11690_e6354), ((((var_nsubb_dn9 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn9)) / (var_ef_nsubc * var_ef_nsubc)) / assign11690_e6354), ((((var_nsubb_dn10 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn10)) / (var_ef_nsubc * var_ef_nsubc)) / assign11690_e6354), ((((var_nsubb_dn13 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn13)) / (var_ef_nsubc * var_ef_nsubc)) / assign11690_e6354),)
    } else {
        (var_ptovr0, var_ptovr0_dn0, var_ptovr0_dn2, var_ptovr0_dn4, var_ptovr0_dn5, var_ptovr0_dn6, var_ptovr0_dn7, var_ptovr0_dn8, var_ptovr0_dn9, var_ptovr0_dn10, var_ptovr0_dn13,)
    }
};
        var_ptovr0 = assign11690_e6357;
        var_ptovr0_dn0 = assign11690_e6357_d_n0;
        var_ptovr0_dn2 = assign11690_e6357_d_n2;
        var_ptovr0_dn4 = assign11690_e6357_d_n4;
        var_ptovr0_dn5 = assign11690_e6357_d_n5;
        var_ptovr0_dn6 = assign11690_e6357_d_n6;
        var_ptovr0_dn7 = assign11690_e6357_d_n7;
        var_ptovr0_dn8 = assign11690_e6357_d_n8;
        var_ptovr0_dn9 = assign11690_e6357_d_n9;
        var_ptovr0_dn10 = assign11690_e6357_d_n10;
        var_ptovr0_dn13 = assign11690_e6357_d_n13;
        var_ptovr0_rv = 0.0;

        let (assign11700_e6362, assign11700_e6362_d_n0, assign11700_e6362_d_n2, assign11700_e6362_d_n4, assign11700_e6362_d_n5, assign11700_e6362_d_n6, assign11700_e6362_d_n7, assign11700_e6362_d_n8, assign11700_e6362_d_n9, assign11700_e6362_d_n10, assign11700_e6362_d_n13,) = {
    if (var_guard268 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ptovr0, var_ptovr0_dn0, var_ptovr0_dn2, var_ptovr0_dn4, var_ptovr0_dn5, var_ptovr0_dn6, var_ptovr0_dn7, var_ptovr0_dn8, var_ptovr0_dn9, var_ptovr0_dn10, var_ptovr0_dn13,)
    }
};
        var_ptovr0 = assign11700_e6362;
        var_ptovr0_dn0 = assign11700_e6362_d_n0;
        var_ptovr0_dn2 = assign11700_e6362_d_n2;
        var_ptovr0_dn4 = assign11700_e6362_d_n4;
        var_ptovr0_dn5 = assign11700_e6362_d_n5;
        var_ptovr0_dn6 = assign11700_e6362_d_n6;
        var_ptovr0_dn7 = assign11700_e6362_d_n7;
        var_ptovr0_dn8 = assign11700_e6362_d_n8;
        var_ptovr0_dn9 = assign11700_e6362_d_n9;
        var_ptovr0_dn10 = assign11700_e6362_d_n10;
        var_ptovr0_dn13 = assign11700_e6362_d_n13;
        var_ptovr0_rv = 0.0;

        let assign11710_e6365: f64 = (2.0 * 1.6021918e-19);
        let assign11710_e6367: f64 = (assign11710_e6365 * var_uc_nsti);
        let assign11710_e6369: f64 = (assign11710_e6367 * 1.034943e-10);
        let assign11710_e6370: f64 = (assign11710_e6369).sqrt();
        var_costi00 = assign11710_e6370;
        var_costi00_rv = 0.0;

        let assign11720_e6374: f64 = (var_uc_nsti * var_uc_nsti);
        let assign11720_e6375: f64 = (1.0 / assign11720_e6374);
        var_nsti_p2 = assign11720_e6375;
        var_nsti_p2_rv = 0.0;

        let assign11730_e6380: f64 = (var_lg).powf(p.p231);
        let assign11730_e6381: f64 = (var_uc_vover / assign11730_e6380);
        let assign11730_e6382: f64 = (1.0 + assign11730_e6381);
        let assign11730_e6387: f64 = (var_wlg).powf(p.p239);
        let assign11730_e6388: f64 = (p.p238 / assign11730_e6387);
        let assign11730_e6389: f64 = (1.0 + assign11730_e6388);
        let assign11730_e6390: f64 = (assign11730_e6382 * assign11730_e6389);
        var_vmax0 = assign11730_e6390;
        var_vmax0_rv = 0.0;

        let assign11740_e6393: f64 = (2.0 / 38.68283);
        let assign11740_e6396: f64 = (var_nsub / 1.04e16);
        let assign11740_e6397: f64 = (assign11740_e6396).ln();
        let assign11740_e6398: f64 = (assign11740_e6393 * assign11740_e6397);
        var_pb20 = assign11740_e6398;
        var_pb20_dn0 = (assign11740_e6393 * ((var_nsub_dn0 / 1.04e16) / assign11740_e6396));
        var_pb20_dn2 = (assign11740_e6393 * ((var_nsub_dn2 / 1.04e16) / assign11740_e6396));
        var_pb20_dn4 = (assign11740_e6393 * ((var_nsub_dn4 / 1.04e16) / assign11740_e6396));
        var_pb20_dn5 = (assign11740_e6393 * ((var_nsub_dn5 / 1.04e16) / assign11740_e6396));
        var_pb20_dn6 = (assign11740_e6393 * ((var_nsub_dn6 / 1.04e16) / assign11740_e6396));
        var_pb20_dn7 = (assign11740_e6393 * ((var_nsub_dn7 / 1.04e16) / assign11740_e6396));
        var_pb20_dn8 = (assign11740_e6393 * ((var_nsub_dn8 / 1.04e16) / assign11740_e6396));
        var_pb20_dn9 = (assign11740_e6393 * ((var_nsub_dn9 / 1.04e16) / assign11740_e6396));
        var_pb20_dn10 = (assign11740_e6393 * ((var_nsub_dn10 / 1.04e16) / assign11740_e6396));
        var_pb20_dn13 = (assign11740_e6393 * ((var_nsub_dn13 / 1.04e16) / assign11740_e6396));
        var_pb20_rv = 0.0;

        let assign11750_e6401: f64 = (2.0 / 38.68283);
        let assign11750_e6404: f64 = (var_ef_nsubc / 1.04e16);
        let assign11750_e6405: f64 = (assign11750_e6404).ln();
        let assign11750_e6406: f64 = (assign11750_e6401 * assign11750_e6405);
        var_pb2c = assign11750_e6406;
        var_pb2c_dn0 = (assign11750_e6401 * ((var_ef_nsubc_dn0 / 1.04e16) / assign11750_e6404));
        var_pb2c_dn2 = (assign11750_e6401 * ((var_ef_nsubc_dn2 / 1.04e16) / assign11750_e6404));
        var_pb2c_dn4 = (assign11750_e6401 * ((var_ef_nsubc_dn4 / 1.04e16) / assign11750_e6404));
        var_pb2c_dn5 = (assign11750_e6401 * ((var_ef_nsubc_dn5 / 1.04e16) / assign11750_e6404));
        var_pb2c_dn6 = (assign11750_e6401 * ((var_ef_nsubc_dn6 / 1.04e16) / assign11750_e6404));
        var_pb2c_dn7 = (assign11750_e6401 * ((var_ef_nsubc_dn7 / 1.04e16) / assign11750_e6404));
        var_pb2c_dn8 = (assign11750_e6401 * ((var_ef_nsubc_dn8 / 1.04e16) / assign11750_e6404));
        var_pb2c_dn9 = (assign11750_e6401 * ((var_ef_nsubc_dn9 / 1.04e16) / assign11750_e6404));
        var_pb2c_dn10 = (assign11750_e6401 * ((var_ef_nsubc_dn10 / 1.04e16) / assign11750_e6404));
        var_pb2c_dn13 = (assign11750_e6401 * ((var_ef_nsubc_dn13 / 1.04e16) / assign11750_e6404));
        var_pb2c_rv = 0.0;

        let assign11760_e6409: f64 = if p.p51 == 1.0 { 1.0 } else { 0.0 };
        var_guard269 = assign11760_e6409;
        var_guard269_rv = 0.0;

        let (assign11770_e6419, assign11770_e6419_d_n0, assign11770_e6419_d_n2, assign11770_e6419_d_n4, assign11770_e6419_d_n5, assign11770_e6419_d_n6, assign11770_e6419_d_n7, assign11770_e6419_d_n8, assign11770_e6419_d_n9, assign11770_e6419_d_n10, assign11770_e6419_d_n13,) = {
    if (var_guard269 != 0.0) {
        let assign11770_e6415: f64 = (3.0 * p.p4);
        let assign11770_e6416: f64 = (var_weff / assign11770_e6415);
        let assign11770_e6417: f64 = (p.p5 + assign11770_e6416);
        (assign11770_e6417, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign11770_e6419;
        var_t1_dn0 = assign11770_e6419_d_n0;
        var_t1_dn2 = assign11770_e6419_d_n2;
        var_t1_dn4 = assign11770_e6419_d_n4;
        var_t1_dn5 = assign11770_e6419_d_n5;
        var_t1_dn6 = assign11770_e6419_d_n6;
        var_t1_dn7 = assign11770_e6419_d_n7;
        var_t1_dn8 = assign11770_e6419_d_n8;
        var_t1_dn9 = assign11770_e6419_d_n9;
        var_t1_dn10 = assign11770_e6419_d_n10;
        var_t1_dn13 = assign11770_e6419_d_n13;
        var_t1_rv = 0.0;

        let (assign11780_e6425, assign11780_e6425_d_n0, assign11780_e6425_d_n2, assign11780_e6425_d_n4, assign11780_e6425_d_n5, assign11780_e6425_d_n6, assign11780_e6425_d_n7, assign11780_e6425_d_n8, assign11780_e6425_d_n9, assign11780_e6425_d_n10, assign11780_e6425_d_n13,) = {
    if (var_guard269 != 0.0) {
        let assign11780_e6423: f64 = (var_lgate - p.p6);
        (assign11780_e6423, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign11780_e6425;
        var_t2_dn0 = assign11780_e6425_d_n0;
        var_t2_dn2 = assign11780_e6425_d_n2;
        var_t2_dn4 = assign11780_e6425_d_n4;
        var_t2_dn5 = assign11780_e6425_d_n5;
        var_t2_dn6 = assign11780_e6425_d_n6;
        var_t2_dn7 = assign11780_e6425_d_n7;
        var_t2_dn8 = assign11780_e6425_d_n8;
        var_t2_dn9 = assign11780_e6425_d_n9;
        var_t2_dn10 = assign11780_e6425_d_n10;
        var_t2_dn13 = assign11780_e6425_d_n13;
        var_t2_rv = 0.0;

        let assign11840_e6467: f64 = if p.p130 > 0.0 { 1.0 } else { 0.0 };
        var_guard271 = assign11840_e6467;
        var_guard271_rv = 0.0;

        let (assign11850_e6473,) = {
    if (var_guard271 != 0.0) {
        let assign11850_e6471: f64 = (p.p130 * p.p2);
        (assign11850_e6471,)
    } else {
        (var_rd0,)
    }
};
        var_rd0 = assign11850_e6473;
        var_rd0_rv = 0.0;

        let (assign11860_e6479,) = {
    if (var_guard271 != 0.0) {
        let assign11860_e6477: f64 = (p.p130 * p.p3);
        (assign11860_e6477,)
    } else {
        (var_rs0,)
    }
};
        var_rs0 = assign11860_e6479;
        var_rs0_rv = 0.0;

        let (assign11870_e6484,) = {
    if (var_guard271 == 0.0) {
        (0.0,)
    } else {
        (var_rd0,)
    }
};
        var_rd0 = assign11870_e6484;
        var_rd0_rv = 0.0;

        let (assign11880_e6489,) = {
    if (var_guard271 == 0.0) {
        (0.0,)
    } else {
        (var_rs0,)
    }
};
        var_rs0 = assign11880_e6489;
        var_rs0_rv = 0.0;

        let assign11890_e6492: f64 = if p.p131 > 0.0 { 1.0 } else { 0.0 };
        var_guard272 = assign11890_e6492;
        var_guard272_rv = 0.0;

        let (assign11900_e6498,) = {
    if (var_guard272 != 0.0) {
        let assign11900_e6496: f64 = (p.p131 * p.p3);
        (assign11900_e6496,)
    } else {
        (var_rs0,)
    }
};
        var_rs0 = assign11900_e6498;
        var_rs0_rv = 0.0;

        let (assign11910_e6503,) = {
    if (var_guard272 == 0.0) {
        (0.0,)
    } else {
        (var_rs0,)
    }
};
        var_rs0 = assign11910_e6503;
        var_rs0_rv = 0.0;

        let assign11920_e6506: f64 = if var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        var_guard273 = assign11920_e6506;
        var_guard273_rv = 0.0;

        let assign11930_e6513: f64 = if ((var_uc_rd > 0.0) || (var_uc_rs > 0.0)) { 1.0 } else { 0.0 };
        var_guard274 = assign11930_e6513;
        var_guard274_rv = 0.0;

        let (assign11940_e6525,) = {
    if ((var_guard273 != 0.0) && (var_guard274 != 0.0)) {
        let assign11940_e6521: f64 = (var_wlg).powf(p.p310);
        let assign11940_e6522: f64 = (p.p309 / assign11940_e6521);
        let assign11940_e6523: f64 = (1.0 + assign11940_e6522);
        (assign11940_e6523,)
    } else {
        (var_rdtemp0,)
    }
};
        var_rdtemp0 = assign11940_e6525;
        var_rdtemp0_rv = 0.0;

        let assign11950_e6528: f64 = if var_uc_rdvd != 0.0 { 1.0 } else { 0.0 };
        var_guard275 = assign11950_e6528;
        var_guard275_rv = 0.0;

        let (assign11960_e6542, assign11960_e6542_d_n0, assign11960_e6542_d_n2, assign11960_e6542_d_n4, assign11960_e6542_d_n5, assign11960_e6542_d_n6, assign11960_e6542_d_n7, assign11960_e6542_d_n8, assign11960_e6542_d_n9, assign11960_e6542_d_n10, assign11960_e6542_d_n13,) = {
    if (((var_guard273 != 0.0) && (var_guard274 != 0.0)) && (var_guard275 != 0.0)) {
        let assign11960_e6538: f64 = (var_wlg).powf(p.p304);
        let assign11960_e6539: f64 = (p.p303 / assign11960_e6538);
        let assign11960_e6540: f64 = (1.0 + assign11960_e6539);
        (assign11960_e6540, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn13,)
    }
};
        var_t7 = assign11960_e6542;
        var_t7_dn0 = assign11960_e6542_d_n0;
        var_t7_dn2 = assign11960_e6542_d_n2;
        var_t7_dn4 = assign11960_e6542_d_n4;
        var_t7_dn5 = assign11960_e6542_d_n5;
        var_t7_dn6 = assign11960_e6542_d_n6;
        var_t7_dn7 = assign11960_e6542_d_n7;
        var_t7_dn8 = assign11960_e6542_d_n8;
        var_t7_dn9 = assign11960_e6542_d_n9;
        var_t7_dn10 = assign11960_e6542_d_n10;
        var_t7_dn13 = assign11960_e6542_d_n13;
        var_t7_rv = 0.0;

        let (assign11970_e6555, assign11970_e6555_d_n0, assign11970_e6555_d_n2, assign11970_e6555_d_n4, assign11970_e6555_d_n5, assign11970_e6555_d_n6, assign11970_e6555_d_n7, assign11970_e6555_d_n8, assign11970_e6555_d_n9, assign11970_e6555_d_n10, assign11970_e6555_d_n13,) = {
    if (((var_guard273 != 0.0) && (var_guard274 != 0.0)) && (var_guard275 != 0.0)) {
        let assign11970_e6549: f64 = (-p.p301);
        let assign11970_e6552: f64 = (var_lg).powf(p.p302);
        let assign11970_e6553: f64 = (assign11970_e6549 * assign11970_e6552);
        (assign11970_e6553, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    }
};
        var_t6 = assign11970_e6555;
        var_t6_dn0 = assign11970_e6555_d_n0;
        var_t6_dn2 = assign11970_e6555_d_n2;
        var_t6_dn4 = assign11970_e6555_d_n4;
        var_t6_dn5 = assign11970_e6555_d_n5;
        var_t6_dn6 = assign11970_e6555_d_n6;
        var_t6_dn7 = assign11970_e6555_d_n7;
        var_t6_dn8 = assign11970_e6555_d_n8;
        var_t6_dn9 = assign11970_e6555_d_n9;
        var_t6_dn10 = assign11970_e6555_d_n10;
        var_t6_dn13 = assign11970_e6555_d_n13;
        var_t6_rv = 0.0;

        let assign11980_e6558: f64 = if var_t6 > 60.0 { 1.0 } else { 0.0 };
        var_guard276 = assign11980_e6558;
        var_guard276_rv = 0.0;

        let (assign11990_e6568, assign11990_e6568_d_n0, assign11990_e6568_d_n2, assign11990_e6568_d_n4, assign11990_e6568_d_n5, assign11990_e6568_d_n6, assign11990_e6568_d_n7, assign11990_e6568_d_n8, assign11990_e6568_d_n9, assign11990_e6568_d_n10, assign11990_e6568_d_n13,) = {
    if ((((var_guard273 != 0.0) && (var_guard274 != 0.0)) && (var_guard275 != 0.0)) && (var_guard276 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    }
};
        var_t6 = assign11990_e6568;
        var_t6_dn0 = assign11990_e6568_d_n0;
        var_t6_dn2 = assign11990_e6568_d_n2;
        var_t6_dn4 = assign11990_e6568_d_n4;
        var_t6_dn5 = assign11990_e6568_d_n5;
        var_t6_dn6 = assign11990_e6568_d_n6;
        var_t6_dn7 = assign11990_e6568_d_n7;
        var_t6_dn8 = assign11990_e6568_d_n8;
        var_t6_dn9 = assign11990_e6568_d_n9;
        var_t6_dn10 = assign11990_e6568_d_n10;
        var_t6_dn13 = assign11990_e6568_d_n13;
        var_t6_rv = 0.0;

        let (assign12000_e6577, assign12000_e6577_d_n0, assign12000_e6577_d_n2, assign12000_e6577_d_n4, assign12000_e6577_d_n5, assign12000_e6577_d_n6, assign12000_e6577_d_n7, assign12000_e6577_d_n8, assign12000_e6577_d_n9, assign12000_e6577_d_n10, assign12000_e6577_d_n13,) = {
    if (((var_guard273 != 0.0) && (var_guard274 != 0.0)) && (var_guard275 != 0.0)) {
        let assign12000_e6575: f64 = (var_t6).exp();
        (assign12000_e6575, (assign12000_e6575 * var_t6_dn0), (assign12000_e6575 * var_t6_dn2), (assign12000_e6575 * var_t6_dn4), (assign12000_e6575 * var_t6_dn5), (assign12000_e6575 * var_t6_dn6), (assign12000_e6575 * var_t6_dn7), (assign12000_e6575 * var_t6_dn8), (assign12000_e6575 * var_t6_dn9), (assign12000_e6575 * var_t6_dn10), (assign12000_e6575 * var_t6_dn13),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    }
};
        var_t6 = assign12000_e6577;
        var_t6_dn0 = assign12000_e6577_d_n0;
        var_t6_dn2 = assign12000_e6577_d_n2;
        var_t6_dn4 = assign12000_e6577_d_n4;
        var_t6_dn5 = assign12000_e6577_d_n5;
        var_t6_dn6 = assign12000_e6577_d_n6;
        var_t6_dn7 = assign12000_e6577_d_n7;
        var_t6_dn8 = assign12000_e6577_d_n8;
        var_t6_dn9 = assign12000_e6577_d_n9;
        var_t6_dn10 = assign12000_e6577_d_n10;
        var_t6_dn13 = assign12000_e6577_d_n13;
        var_t6_rv = 0.0;

        let (assign12010_e6587, assign12010_e6587_d_n0, assign12010_e6587_d_n2, assign12010_e6587_d_n4, assign12010_e6587_d_n5, assign12010_e6587_d_n6, assign12010_e6587_d_n7, assign12010_e6587_d_n8, assign12010_e6587_d_n9, assign12010_e6587_d_n10, assign12010_e6587_d_n13,) = {
    if (((var_guard273 != 0.0) && (var_guard274 != 0.0)) && (var_guard275 != 0.0)) {
        let assign12010_e6585: f64 = (var_t6 * var_t7);
        (assign12010_e6585, ((var_t6_dn0 * var_t7) + (var_t6 * var_t7_dn0)), ((var_t6_dn2 * var_t7) + (var_t6 * var_t7_dn2)), ((var_t6_dn4 * var_t7) + (var_t6 * var_t7_dn4)), ((var_t6_dn5 * var_t7) + (var_t6 * var_t7_dn5)), ((var_t6_dn6 * var_t7) + (var_t6 * var_t7_dn6)), ((var_t6_dn7 * var_t7) + (var_t6 * var_t7_dn7)), ((var_t6_dn8 * var_t7) + (var_t6 * var_t7_dn8)), ((var_t6_dn9 * var_t7) + (var_t6 * var_t7_dn9)), ((var_t6_dn10 * var_t7) + (var_t6 * var_t7_dn10)), ((var_t6_dn13 * var_t7) + (var_t6 * var_t7_dn13)),)
    } else {
        (var_rdvdtemp0, var_rdvdtemp0_dn0, var_rdvdtemp0_dn2, var_rdvdtemp0_dn4, var_rdvdtemp0_dn5, var_rdvdtemp0_dn6, var_rdvdtemp0_dn7, var_rdvdtemp0_dn8, var_rdvdtemp0_dn9, var_rdvdtemp0_dn10, var_rdvdtemp0_dn13,)
    }
};
        var_rdvdtemp0 = assign12010_e6587;
        var_rdvdtemp0_dn0 = assign12010_e6587_d_n0;
        var_rdvdtemp0_dn2 = assign12010_e6587_d_n2;
        var_rdvdtemp0_dn4 = assign12010_e6587_d_n4;
        var_rdvdtemp0_dn5 = assign12010_e6587_d_n5;
        var_rdvdtemp0_dn6 = assign12010_e6587_d_n6;
        var_rdvdtemp0_dn7 = assign12010_e6587_d_n7;
        var_rdvdtemp0_dn8 = assign12010_e6587_d_n8;
        var_rdvdtemp0_dn9 = assign12010_e6587_d_n9;
        var_rdvdtemp0_dn10 = assign12010_e6587_d_n10;
        var_rdvdtemp0_dn13 = assign12010_e6587_d_n13;
        var_rdvdtemp0_rv = 0.0;

        let (assign12020_e6596, assign12020_e6596_d_n0, assign12020_e6596_d_n2, assign12020_e6596_d_n4, assign12020_e6596_d_n5, assign12020_e6596_d_n6, assign12020_e6596_d_n7, assign12020_e6596_d_n8, assign12020_e6596_d_n9, assign12020_e6596_d_n10, assign12020_e6596_d_n13,) = {
    if (((var_guard273 != 0.0) && (var_guard274 != 0.0)) && (var_guard275 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdvdtemp0, var_rdvdtemp0_dn0, var_rdvdtemp0_dn2, var_rdvdtemp0_dn4, var_rdvdtemp0_dn5, var_rdvdtemp0_dn6, var_rdvdtemp0_dn7, var_rdvdtemp0_dn8, var_rdvdtemp0_dn9, var_rdvdtemp0_dn10, var_rdvdtemp0_dn13,)
    }
};
        var_rdvdtemp0 = assign12020_e6596;
        var_rdvdtemp0_dn0 = assign12020_e6596_d_n0;
        var_rdvdtemp0_dn2 = assign12020_e6596_d_n2;
        var_rdvdtemp0_dn4 = assign12020_e6596_d_n4;
        var_rdvdtemp0_dn5 = assign12020_e6596_d_n5;
        var_rdvdtemp0_dn6 = assign12020_e6596_d_n6;
        var_rdvdtemp0_dn7 = assign12020_e6596_d_n7;
        var_rdvdtemp0_dn8 = assign12020_e6596_d_n8;
        var_rdvdtemp0_dn9 = assign12020_e6596_d_n9;
        var_rdvdtemp0_dn10 = assign12020_e6596_d_n10;
        var_rdvdtemp0_dn13 = assign12020_e6596_d_n13;
        var_rdvdtemp0_rv = 0.0;

        let (assign12030_e6603,) = {
    if ((var_guard273 != 0.0) && (var_guard274 == 0.0)) {
        (0.0,)
    } else {
        (var_rdtemp0,)
    }
};
        var_rdtemp0 = assign12030_e6603;
        var_rdtemp0_rv = 0.0;

        let (assign12040_e6610, assign12040_e6610_d_n0, assign12040_e6610_d_n2, assign12040_e6610_d_n4, assign12040_e6610_d_n5, assign12040_e6610_d_n6, assign12040_e6610_d_n7, assign12040_e6610_d_n8, assign12040_e6610_d_n9, assign12040_e6610_d_n10, assign12040_e6610_d_n13,) = {
    if ((var_guard273 != 0.0) && (var_guard274 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdvdtemp0, var_rdvdtemp0_dn0, var_rdvdtemp0_dn2, var_rdvdtemp0_dn4, var_rdvdtemp0_dn5, var_rdvdtemp0_dn6, var_rdvdtemp0_dn7, var_rdvdtemp0_dn8, var_rdvdtemp0_dn9, var_rdvdtemp0_dn10, var_rdvdtemp0_dn13,)
    }
};
        var_rdvdtemp0 = assign12040_e6610;
        var_rdvdtemp0_dn0 = assign12040_e6610_d_n0;
        var_rdvdtemp0_dn2 = assign12040_e6610_d_n2;
        var_rdvdtemp0_dn4 = assign12040_e6610_d_n4;
        var_rdvdtemp0_dn5 = assign12040_e6610_d_n5;
        var_rdvdtemp0_dn6 = assign12040_e6610_d_n6;
        var_rdvdtemp0_dn7 = assign12040_e6610_d_n7;
        var_rdvdtemp0_dn8 = assign12040_e6610_d_n8;
        var_rdvdtemp0_dn9 = assign12040_e6610_d_n9;
        var_rdvdtemp0_dn10 = assign12040_e6610_d_n10;
        var_rdvdtemp0_dn13 = assign12040_e6610_d_n13;
        var_rdvdtemp0_rv = 0.0;

        let assign12050_e6613: f64 = if var_uc_rd23 != 0.0 { 1.0 } else { 0.0 };
        var_guard277 = assign12050_e6613;
        var_guard277_rv = 0.0;

        let (assign12060_e6625, assign12060_e6625_d_n0, assign12060_e6625_d_n2, assign12060_e6625_d_n4, assign12060_e6625_d_n5, assign12060_e6625_d_n6, assign12060_e6625_d_n7, assign12060_e6625_d_n8, assign12060_e6625_d_n9, assign12060_e6625_d_n10, assign12060_e6625_d_n13,) = {
    if ((var_guard273 != 0.0) && (var_guard277 != 0.0)) {
        let assign12060_e6621: f64 = (var_wlg).powf(p.p308);
        let assign12060_e6622: f64 = (p.p307 / assign12060_e6621);
        let assign12060_e6623: f64 = (1.0 + assign12060_e6622);
        (assign12060_e6623, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign12060_e6625;
        var_t2_dn0 = assign12060_e6625_d_n0;
        var_t2_dn2 = assign12060_e6625_d_n2;
        var_t2_dn4 = assign12060_e6625_d_n4;
        var_t2_dn5 = assign12060_e6625_d_n5;
        var_t2_dn6 = assign12060_e6625_d_n6;
        var_t2_dn7 = assign12060_e6625_d_n7;
        var_t2_dn8 = assign12060_e6625_d_n8;
        var_t2_dn9 = assign12060_e6625_d_n9;
        var_t2_dn10 = assign12060_e6625_d_n10;
        var_t2_dn13 = assign12060_e6625_d_n13;
        var_t2_rv = 0.0;

        let (assign12070_e6636, assign12070_e6636_d_n0, assign12070_e6636_d_n2, assign12070_e6636_d_n4, assign12070_e6636_d_n5, assign12070_e6636_d_n6, assign12070_e6636_d_n7, assign12070_e6636_d_n8, assign12070_e6636_d_n9, assign12070_e6636_d_n10, assign12070_e6636_d_n13,) = {
    if ((var_guard273 != 0.0) && (var_guard277 != 0.0)) {
        let assign12070_e6630: f64 = (-p.p305);
        let assign12070_e6633: f64 = (var_lg).powf(p.p306);
        let assign12070_e6634: f64 = (assign12070_e6630 * assign12070_e6633);
        (assign12070_e6634, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign12070_e6636;
        var_t1_dn0 = assign12070_e6636_d_n0;
        var_t1_dn2 = assign12070_e6636_d_n2;
        var_t1_dn4 = assign12070_e6636_d_n4;
        var_t1_dn5 = assign12070_e6636_d_n5;
        var_t1_dn6 = assign12070_e6636_d_n6;
        var_t1_dn7 = assign12070_e6636_d_n7;
        var_t1_dn8 = assign12070_e6636_d_n8;
        var_t1_dn9 = assign12070_e6636_d_n9;
        var_t1_dn10 = assign12070_e6636_d_n10;
        var_t1_dn13 = assign12070_e6636_d_n13;
        var_t1_rv = 0.0;

        let assign12080_e6639: f64 = if var_t1 > 60.0 { 1.0 } else { 0.0 };
        var_guard278 = assign12080_e6639;
        var_guard278_rv = 0.0;

        let (assign12090_e6647, assign12090_e6647_d_n0, assign12090_e6647_d_n2, assign12090_e6647_d_n4, assign12090_e6647_d_n5, assign12090_e6647_d_n6, assign12090_e6647_d_n7, assign12090_e6647_d_n8, assign12090_e6647_d_n9, assign12090_e6647_d_n10, assign12090_e6647_d_n13,) = {
    if (((var_guard273 != 0.0) && (var_guard277 != 0.0)) && (var_guard278 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign12090_e6647;
        var_t1_dn0 = assign12090_e6647_d_n0;
        var_t1_dn2 = assign12090_e6647_d_n2;
        var_t1_dn4 = assign12090_e6647_d_n4;
        var_t1_dn5 = assign12090_e6647_d_n5;
        var_t1_dn6 = assign12090_e6647_d_n6;
        var_t1_dn7 = assign12090_e6647_d_n7;
        var_t1_dn8 = assign12090_e6647_d_n8;
        var_t1_dn9 = assign12090_e6647_d_n9;
        var_t1_dn10 = assign12090_e6647_d_n10;
        var_t1_dn13 = assign12090_e6647_d_n13;
        var_t1_rv = 0.0;

        *var_costi00_slot = var_costi00;
        *var_costi00_rv_slot = var_costi00_rv;
        *var_guard269_slot = var_guard269;
        *var_guard269_rv_slot = var_guard269_rv;
        *var_guard271_slot = var_guard271;
        *var_guard271_rv_slot = var_guard271_rv;
        *var_guard272_slot = var_guard272;
        *var_guard272_rv_slot = var_guard272_rv;
        *var_guard273_slot = var_guard273;
        *var_guard273_rv_slot = var_guard273_rv;
        *var_guard274_slot = var_guard274;
        *var_guard274_rv_slot = var_guard274_rv;
        *var_guard275_slot = var_guard275;
        *var_guard275_rv_slot = var_guard275_rv;
        *var_guard276_slot = var_guard276;
        *var_guard276_rv_slot = var_guard276_rv;
        *var_guard277_slot = var_guard277;
        *var_guard277_rv_slot = var_guard277_rv;
        *var_guard278_slot = var_guard278;
        *var_guard278_rv_slot = var_guard278_rv;
        *var_nsti_p2_slot = var_nsti_p2;
        *var_nsti_p2_rv_slot = var_nsti_p2_rv;
        *var_nsubb_slot = var_nsubb;
        *var_nsubb_dn0_slot = var_nsubb_dn0;
        *var_nsubb_dn10_slot = var_nsubb_dn10;
        *var_nsubb_dn13_slot = var_nsubb_dn13;
        *var_nsubb_dn2_slot = var_nsubb_dn2;
        *var_nsubb_dn4_slot = var_nsubb_dn4;
        *var_nsubb_dn5_slot = var_nsubb_dn5;
        *var_nsubb_dn6_slot = var_nsubb_dn6;
        *var_nsubb_dn7_slot = var_nsubb_dn7;
        *var_nsubb_dn8_slot = var_nsubb_dn8;
        *var_nsubb_dn9_slot = var_nsubb_dn9;
        *var_nsubb_rv_slot = var_nsubb_rv;
        *var_pb20_slot = var_pb20;
        *var_pb20_dn0_slot = var_pb20_dn0;
        *var_pb20_dn10_slot = var_pb20_dn10;
        *var_pb20_dn13_slot = var_pb20_dn13;
        *var_pb20_dn2_slot = var_pb20_dn2;
        *var_pb20_dn4_slot = var_pb20_dn4;
        *var_pb20_dn5_slot = var_pb20_dn5;
        *var_pb20_dn6_slot = var_pb20_dn6;
        *var_pb20_dn7_slot = var_pb20_dn7;
        *var_pb20_dn8_slot = var_pb20_dn8;
        *var_pb20_dn9_slot = var_pb20_dn9;
        *var_pb20_rv_slot = var_pb20_rv;
        *var_pb2c_slot = var_pb2c;
        *var_pb2c_dn0_slot = var_pb2c_dn0;
        *var_pb2c_dn10_slot = var_pb2c_dn10;
        *var_pb2c_dn13_slot = var_pb2c_dn13;
        *var_pb2c_dn2_slot = var_pb2c_dn2;
        *var_pb2c_dn4_slot = var_pb2c_dn4;
        *var_pb2c_dn5_slot = var_pb2c_dn5;
        *var_pb2c_dn6_slot = var_pb2c_dn6;
        *var_pb2c_dn7_slot = var_pb2c_dn7;
        *var_pb2c_dn8_slot = var_pb2c_dn8;
        *var_pb2c_dn9_slot = var_pb2c_dn9;
        *var_pb2c_rv_slot = var_pb2c_rv;
        *var_ptovr0_slot = var_ptovr0;
        *var_ptovr0_dn0_slot = var_ptovr0_dn0;
        *var_ptovr0_dn10_slot = var_ptovr0_dn10;
        *var_ptovr0_dn13_slot = var_ptovr0_dn13;
        *var_ptovr0_dn2_slot = var_ptovr0_dn2;
        *var_ptovr0_dn4_slot = var_ptovr0_dn4;
        *var_ptovr0_dn5_slot = var_ptovr0_dn5;
        *var_ptovr0_dn6_slot = var_ptovr0_dn6;
        *var_ptovr0_dn7_slot = var_ptovr0_dn7;
        *var_ptovr0_dn8_slot = var_ptovr0_dn8;
        *var_ptovr0_dn9_slot = var_ptovr0_dn9;
        *var_ptovr0_rv_slot = var_ptovr0_rv;
        *var_rd0_slot = var_rd0;
        *var_rd0_rv_slot = var_rd0_rv;
        *var_rdtemp0_slot = var_rdtemp0;
        *var_rdtemp0_rv_slot = var_rdtemp0_rv;
        *var_rdvdtemp0_slot = var_rdvdtemp0;
        *var_rdvdtemp0_dn0_slot = var_rdvdtemp0_dn0;
        *var_rdvdtemp0_dn10_slot = var_rdvdtemp0_dn10;
        *var_rdvdtemp0_dn13_slot = var_rdvdtemp0_dn13;
        *var_rdvdtemp0_dn2_slot = var_rdvdtemp0_dn2;
        *var_rdvdtemp0_dn4_slot = var_rdvdtemp0_dn4;
        *var_rdvdtemp0_dn5_slot = var_rdvdtemp0_dn5;
        *var_rdvdtemp0_dn6_slot = var_rdvdtemp0_dn6;
        *var_rdvdtemp0_dn7_slot = var_rdvdtemp0_dn7;
        *var_rdvdtemp0_dn8_slot = var_rdvdtemp0_dn8;
        *var_rdvdtemp0_dn9_slot = var_rdvdtemp0_dn9;
        *var_rdvdtemp0_rv_slot = var_rdvdtemp0_rv;
        *var_rs0_slot = var_rs0;
        *var_rs0_rv_slot = var_rs0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn13_slot = var_t6_dn13;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t6_rv_slot = var_t6_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn13_slot = var_t7_dn13;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_t7_rv_slot = var_t7_rv;
        *var_vmax0_slot = var_vmax0;
        *var_vmax0_rv_slot = var_vmax0_rv;
    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        var_ef_nsubc: f64,
        var_ef_nsubc_dn0: f64,
        var_ef_nsubc_dn10: f64,
        var_ef_nsubc_dn13: f64,
        var_ef_nsubc_dn2: f64,
        var_ef_nsubc_dn4: f64,
        var_ef_nsubc_dn5: f64,
        var_ef_nsubc_dn6: f64,
        var_ef_nsubc_dn7: f64,
        var_ef_nsubc_dn8: f64,
        var_ef_nsubc_dn9: f64,
        var_guard273: f64,
        var_guard277: f64,
        var_lg: f64,
        var_t2: f64,
        var_t2_dn0: f64,
        var_t2_dn10: f64,
        var_t2_dn13: f64,
        var_t2_dn2: f64,
        var_t2_dn4: f64,
        var_t2_dn5: f64,
        var_t2_dn6: f64,
        var_t2_dn7: f64,
        var_t2_dn8: f64,
        var_t2_dn9: f64,
        var_uc_nover: f64,
        var_uc_rd23: f64,
        var_uc_xldld: f64,
        var_wg: f64,
        var_ddlte_slot: &mut f64,
        var_ddlte_dn0_slot: &mut f64,
        var_ddlte_dn10_slot: &mut f64,
        var_ddlte_dn13_slot: &mut f64,
        var_ddlte_dn2_slot: &mut f64,
        var_ddlte_dn4_slot: &mut f64,
        var_ddlte_dn5_slot: &mut f64,
        var_ddlte_dn6_slot: &mut f64,
        var_ddlte_dn7_slot: &mut f64,
        var_ddlte_dn8_slot: &mut f64,
        var_ddlte_dn9_slot: &mut f64,
        var_ddlte_rv_slot: &mut f64,
        var_guard279_slot: &mut f64,
        var_guard279_rv_slot: &mut f64,
        var_guard284_slot: &mut f64,
        var_guard284_rv_slot: &mut f64,
        var_guard285_slot: &mut f64,
        var_guard285_rv_slot: &mut f64,
        var_guard286_slot: &mut f64,
        var_guard286_rv_slot: &mut f64,
        var_kdep_slot: &mut f64,
        var_kdep_rv_slot: &mut f64,
        var_kjunc_slot: &mut f64,
        var_kjunc_dn0_slot: &mut f64,
        var_kjunc_dn10_slot: &mut f64,
        var_kjunc_dn13_slot: &mut f64,
        var_kjunc_dn2_slot: &mut f64,
        var_kjunc_dn4_slot: &mut f64,
        var_kjunc_dn5_slot: &mut f64,
        var_kjunc_dn6_slot: &mut f64,
        var_kjunc_dn7_slot: &mut f64,
        var_kjunc_dn8_slot: &mut f64,
        var_kjunc_dn9_slot: &mut f64,
        var_kjunc_rv_slot: &mut f64,
        var_rd23e_slot: &mut f64,
        var_rd23e_dn0_slot: &mut f64,
        var_rd23e_dn10_slot: &mut f64,
        var_rd23e_dn13_slot: &mut f64,
        var_rd23e_dn2_slot: &mut f64,
        var_rd23e_dn4_slot: &mut f64,
        var_rd23e_dn5_slot: &mut f64,
        var_rd23e_dn6_slot: &mut f64,
        var_rd23e_dn7_slot: &mut f64,
        var_rd23e_dn8_slot: &mut f64,
        var_rd23e_dn9_slot: &mut f64,
        var_rd23e_rv_slot: &mut f64,
        var_rdrmuele_slot: &mut f64,
        var_rdrmuele_rv_slot: &mut f64,
        var_rdrmuevbs_slot: &mut f64,
        var_rdrmuevbs_dn0_slot: &mut f64,
        var_rdrmuevbs_dn10_slot: &mut f64,
        var_rdrmuevbs_dn13_slot: &mut f64,
        var_rdrmuevbs_dn2_slot: &mut f64,
        var_rdrmuevbs_dn4_slot: &mut f64,
        var_rdrmuevbs_dn5_slot: &mut f64,
        var_rdrmuevbs_dn6_slot: &mut f64,
        var_rdrmuevbs_dn7_slot: &mut f64,
        var_rdrmuevbs_dn8_slot: &mut f64,
        var_rdrmuevbs_dn9_slot: &mut f64,
        var_rdrmuevbs_rv_slot: &mut f64,
        var_rdrvmaxle_slot: &mut f64,
        var_rdrvmaxle_rv_slot: &mut f64,
        var_rdrvmaxwe_slot: &mut f64,
        var_rdrvmaxwe_rv_slot: &mut f64,
        var_rdtemp0_slot: &mut f64,
        var_rdtemp0_rv_slot: &mut f64,
        var_rdvdtemp0_slot: &mut f64,
        var_rdvdtemp0_dn0_slot: &mut f64,
        var_rdvdtemp0_dn10_slot: &mut f64,
        var_rdvdtemp0_dn13_slot: &mut f64,
        var_rdvdtemp0_dn2_slot: &mut f64,
        var_rdvdtemp0_dn4_slot: &mut f64,
        var_rdvdtemp0_dn5_slot: &mut f64,
        var_rdvdtemp0_dn6_slot: &mut f64,
        var_rdvdtemp0_dn7_slot: &mut f64,
        var_rdvdtemp0_dn8_slot: &mut f64,
        var_rdvdtemp0_dn9_slot: &mut f64,
        var_rdvdtemp0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_xmax_slot: &mut f64,
        var_xmax_rv_slot: &mut f64,
        var_xmax_s_slot: &mut f64,
        var_xmax_s_rv_slot: &mut f64,
    ) {
        let mut var_ddlte: f64 = *var_ddlte_slot;
        let mut var_ddlte_dn0: f64 = *var_ddlte_dn0_slot;
        let mut var_ddlte_dn10: f64 = *var_ddlte_dn10_slot;
        let mut var_ddlte_dn13: f64 = *var_ddlte_dn13_slot;
        let mut var_ddlte_dn2: f64 = *var_ddlte_dn2_slot;
        let mut var_ddlte_dn4: f64 = *var_ddlte_dn4_slot;
        let mut var_ddlte_dn5: f64 = *var_ddlte_dn5_slot;
        let mut var_ddlte_dn6: f64 = *var_ddlte_dn6_slot;
        let mut var_ddlte_dn7: f64 = *var_ddlte_dn7_slot;
        let mut var_ddlte_dn8: f64 = *var_ddlte_dn8_slot;
        let mut var_ddlte_dn9: f64 = *var_ddlte_dn9_slot;
        let mut var_ddlte_rv: f64 = *var_ddlte_rv_slot;
        let mut var_guard279: f64 = *var_guard279_slot;
        let mut var_guard279_rv: f64 = *var_guard279_rv_slot;
        let mut var_guard284: f64 = *var_guard284_slot;
        let mut var_guard284_rv: f64 = *var_guard284_rv_slot;
        let mut var_guard285: f64 = *var_guard285_slot;
        let mut var_guard285_rv: f64 = *var_guard285_rv_slot;
        let mut var_guard286: f64 = *var_guard286_slot;
        let mut var_guard286_rv: f64 = *var_guard286_rv_slot;
        let mut var_kdep: f64 = *var_kdep_slot;
        let mut var_kdep_rv: f64 = *var_kdep_rv_slot;
        let mut var_kjunc: f64 = *var_kjunc_slot;
        let mut var_kjunc_dn0: f64 = *var_kjunc_dn0_slot;
        let mut var_kjunc_dn10: f64 = *var_kjunc_dn10_slot;
        let mut var_kjunc_dn13: f64 = *var_kjunc_dn13_slot;
        let mut var_kjunc_dn2: f64 = *var_kjunc_dn2_slot;
        let mut var_kjunc_dn4: f64 = *var_kjunc_dn4_slot;
        let mut var_kjunc_dn5: f64 = *var_kjunc_dn5_slot;
        let mut var_kjunc_dn6: f64 = *var_kjunc_dn6_slot;
        let mut var_kjunc_dn7: f64 = *var_kjunc_dn7_slot;
        let mut var_kjunc_dn8: f64 = *var_kjunc_dn8_slot;
        let mut var_kjunc_dn9: f64 = *var_kjunc_dn9_slot;
        let mut var_kjunc_rv: f64 = *var_kjunc_rv_slot;
        let mut var_rd23e: f64 = *var_rd23e_slot;
        let mut var_rd23e_dn0: f64 = *var_rd23e_dn0_slot;
        let mut var_rd23e_dn10: f64 = *var_rd23e_dn10_slot;
        let mut var_rd23e_dn13: f64 = *var_rd23e_dn13_slot;
        let mut var_rd23e_dn2: f64 = *var_rd23e_dn2_slot;
        let mut var_rd23e_dn4: f64 = *var_rd23e_dn4_slot;
        let mut var_rd23e_dn5: f64 = *var_rd23e_dn5_slot;
        let mut var_rd23e_dn6: f64 = *var_rd23e_dn6_slot;
        let mut var_rd23e_dn7: f64 = *var_rd23e_dn7_slot;
        let mut var_rd23e_dn8: f64 = *var_rd23e_dn8_slot;
        let mut var_rd23e_dn9: f64 = *var_rd23e_dn9_slot;
        let mut var_rd23e_rv: f64 = *var_rd23e_rv_slot;
        let mut var_rdrmuele: f64 = *var_rdrmuele_slot;
        let mut var_rdrmuele_rv: f64 = *var_rdrmuele_rv_slot;
        let mut var_rdrmuevbs: f64 = *var_rdrmuevbs_slot;
        let mut var_rdrmuevbs_dn0: f64 = *var_rdrmuevbs_dn0_slot;
        let mut var_rdrmuevbs_dn10: f64 = *var_rdrmuevbs_dn10_slot;
        let mut var_rdrmuevbs_dn13: f64 = *var_rdrmuevbs_dn13_slot;
        let mut var_rdrmuevbs_dn2: f64 = *var_rdrmuevbs_dn2_slot;
        let mut var_rdrmuevbs_dn4: f64 = *var_rdrmuevbs_dn4_slot;
        let mut var_rdrmuevbs_dn5: f64 = *var_rdrmuevbs_dn5_slot;
        let mut var_rdrmuevbs_dn6: f64 = *var_rdrmuevbs_dn6_slot;
        let mut var_rdrmuevbs_dn7: f64 = *var_rdrmuevbs_dn7_slot;
        let mut var_rdrmuevbs_dn8: f64 = *var_rdrmuevbs_dn8_slot;
        let mut var_rdrmuevbs_dn9: f64 = *var_rdrmuevbs_dn9_slot;
        let mut var_rdrmuevbs_rv: f64 = *var_rdrmuevbs_rv_slot;
        let mut var_rdrvmaxle: f64 = *var_rdrvmaxle_slot;
        let mut var_rdrvmaxle_rv: f64 = *var_rdrvmaxle_rv_slot;
        let mut var_rdrvmaxwe: f64 = *var_rdrvmaxwe_slot;
        let mut var_rdrvmaxwe_rv: f64 = *var_rdrvmaxwe_rv_slot;
        let mut var_rdtemp0: f64 = *var_rdtemp0_slot;
        let mut var_rdtemp0_rv: f64 = *var_rdtemp0_rv_slot;
        let mut var_rdvdtemp0: f64 = *var_rdvdtemp0_slot;
        let mut var_rdvdtemp0_dn0: f64 = *var_rdvdtemp0_dn0_slot;
        let mut var_rdvdtemp0_dn10: f64 = *var_rdvdtemp0_dn10_slot;
        let mut var_rdvdtemp0_dn13: f64 = *var_rdvdtemp0_dn13_slot;
        let mut var_rdvdtemp0_dn2: f64 = *var_rdvdtemp0_dn2_slot;
        let mut var_rdvdtemp0_dn4: f64 = *var_rdvdtemp0_dn4_slot;
        let mut var_rdvdtemp0_dn5: f64 = *var_rdvdtemp0_dn5_slot;
        let mut var_rdvdtemp0_dn6: f64 = *var_rdvdtemp0_dn6_slot;
        let mut var_rdvdtemp0_dn7: f64 = *var_rdvdtemp0_dn7_slot;
        let mut var_rdvdtemp0_dn8: f64 = *var_rdvdtemp0_dn8_slot;
        let mut var_rdvdtemp0_dn9: f64 = *var_rdvdtemp0_dn9_slot;
        let mut var_rdvdtemp0_rv: f64 = *var_rdvdtemp0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_xmax: f64 = *var_xmax_slot;
        let mut var_xmax_rv: f64 = *var_xmax_rv_slot;
        let mut var_xmax_s: f64 = *var_xmax_s_slot;
        let mut var_xmax_s_rv: f64 = *var_xmax_s_rv_slot;

        let (assign12100_e6654, assign12100_e6654_d_n0, assign12100_e6654_d_n2, assign12100_e6654_d_n4, assign12100_e6654_d_n5, assign12100_e6654_d_n6, assign12100_e6654_d_n7, assign12100_e6654_d_n8, assign12100_e6654_d_n9, assign12100_e6654_d_n10, assign12100_e6654_d_n13,) = {
    if ((var_guard273 != 0.0) && (var_guard277 != 0.0)) {
        let assign12100_e6652: f64 = (var_t1).exp();
        (assign12100_e6652, (assign12100_e6652 * var_t1_dn0), (assign12100_e6652 * var_t1_dn2), (assign12100_e6652 * var_t1_dn4), (assign12100_e6652 * var_t1_dn5), (assign12100_e6652 * var_t1_dn6), (assign12100_e6652 * var_t1_dn7), (assign12100_e6652 * var_t1_dn8), (assign12100_e6652 * var_t1_dn9), (assign12100_e6652 * var_t1_dn10), (assign12100_e6652 * var_t1_dn13),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign12100_e6654;
        var_t1_dn0 = assign12100_e6654_d_n0;
        var_t1_dn2 = assign12100_e6654_d_n2;
        var_t1_dn4 = assign12100_e6654_d_n4;
        var_t1_dn5 = assign12100_e6654_d_n5;
        var_t1_dn6 = assign12100_e6654_d_n6;
        var_t1_dn7 = assign12100_e6654_d_n7;
        var_t1_dn8 = assign12100_e6654_d_n8;
        var_t1_dn9 = assign12100_e6654_d_n9;
        var_t1_dn10 = assign12100_e6654_d_n10;
        var_t1_dn13 = assign12100_e6654_d_n13;
        var_t1_rv = 0.0;

        let (assign12110_e6664, assign12110_e6664_d_n0, assign12110_e6664_d_n2, assign12110_e6664_d_n4, assign12110_e6664_d_n5, assign12110_e6664_d_n6, assign12110_e6664_d_n7, assign12110_e6664_d_n8, assign12110_e6664_d_n9, assign12110_e6664_d_n10, assign12110_e6664_d_n13,) = {
    if ((var_guard273 != 0.0) && (var_guard277 != 0.0)) {
        let assign12110_e6660: f64 = (var_uc_rd23 * var_t2);
        let assign12110_e6662: f64 = (assign12110_e6660 * var_t1);
        (assign12110_e6662, (((var_uc_rd23 * var_t2_dn0) * var_t1) + (assign12110_e6660 * var_t1_dn0)), (((var_uc_rd23 * var_t2_dn2) * var_t1) + (assign12110_e6660 * var_t1_dn2)), (((var_uc_rd23 * var_t2_dn4) * var_t1) + (assign12110_e6660 * var_t1_dn4)), (((var_uc_rd23 * var_t2_dn5) * var_t1) + (assign12110_e6660 * var_t1_dn5)), (((var_uc_rd23 * var_t2_dn6) * var_t1) + (assign12110_e6660 * var_t1_dn6)), (((var_uc_rd23 * var_t2_dn7) * var_t1) + (assign12110_e6660 * var_t1_dn7)), (((var_uc_rd23 * var_t2_dn8) * var_t1) + (assign12110_e6660 * var_t1_dn8)), (((var_uc_rd23 * var_t2_dn9) * var_t1) + (assign12110_e6660 * var_t1_dn9)), (((var_uc_rd23 * var_t2_dn10) * var_t1) + (assign12110_e6660 * var_t1_dn10)), (((var_uc_rd23 * var_t2_dn13) * var_t1) + (assign12110_e6660 * var_t1_dn13)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign12110_e6664;
        var_t3_dn0 = assign12110_e6664_d_n0;
        var_t3_dn2 = assign12110_e6664_d_n2;
        var_t3_dn4 = assign12110_e6664_d_n4;
        var_t3_dn5 = assign12110_e6664_d_n5;
        var_t3_dn6 = assign12110_e6664_d_n6;
        var_t3_dn7 = assign12110_e6664_d_n7;
        var_t3_dn8 = assign12110_e6664_d_n8;
        var_t3_dn9 = assign12110_e6664_d_n9;
        var_t3_dn10 = assign12110_e6664_d_n10;
        var_t3_dn13 = assign12110_e6664_d_n13;
        var_t3_rv = 0.0;

        let (assign12120_e6687, assign12120_e6687_d_n0, assign12120_e6687_d_n2, assign12120_e6687_d_n4, assign12120_e6687_d_n5, assign12120_e6687_d_n6, assign12120_e6687_d_n7, assign12120_e6687_d_n8, assign12120_e6687_d_n9, assign12120_e6687_d_n10, assign12120_e6687_d_n13,) = {
    if ((var_guard273 != 0.0) && (var_guard277 != 0.0)) {
        let assign12120_e6672: f64 = (var_t3 * var_t3);
        let assign12120_e6675: f64 = (4.0 * 1e-6);
        let assign12120_e6677: f64 = (assign12120_e6675 / 100.0);
        let assign12120_e6679: f64 = (assign12120_e6677 * 1e-6);
        let assign12120_e6681: f64 = (assign12120_e6679 / 100.0);
        let assign12120_e6682: f64 = (assign12120_e6672 + assign12120_e6681);
        let assign12120_e6683: f64 = (assign12120_e6682).sqrt();
        let assign12120_e6684: f64 = (var_t3 + assign12120_e6683);
        let assign12120_e6685: f64 = (0.5 * assign12120_e6684);
        (assign12120_e6685, (0.5 * (var_t3_dn0 + (((var_t3_dn0 * var_t3) + (var_t3 * var_t3_dn0)) / (2.0 * assign12120_e6683)))), (0.5 * (var_t3_dn2 + (((var_t3_dn2 * var_t3) + (var_t3 * var_t3_dn2)) / (2.0 * assign12120_e6683)))), (0.5 * (var_t3_dn4 + (((var_t3_dn4 * var_t3) + (var_t3 * var_t3_dn4)) / (2.0 * assign12120_e6683)))), (0.5 * (var_t3_dn5 + (((var_t3_dn5 * var_t3) + (var_t3 * var_t3_dn5)) / (2.0 * assign12120_e6683)))), (0.5 * (var_t3_dn6 + (((var_t3_dn6 * var_t3) + (var_t3 * var_t3_dn6)) / (2.0 * assign12120_e6683)))), (0.5 * (var_t3_dn7 + (((var_t3_dn7 * var_t3) + (var_t3 * var_t3_dn7)) / (2.0 * assign12120_e6683)))), (0.5 * (var_t3_dn8 + (((var_t3_dn8 * var_t3) + (var_t3 * var_t3_dn8)) / (2.0 * assign12120_e6683)))), (0.5 * (var_t3_dn9 + (((var_t3_dn9 * var_t3) + (var_t3 * var_t3_dn9)) / (2.0 * assign12120_e6683)))), (0.5 * (var_t3_dn10 + (((var_t3_dn10 * var_t3) + (var_t3 * var_t3_dn10)) / (2.0 * assign12120_e6683)))), (0.5 * (var_t3_dn13 + (((var_t3_dn13 * var_t3) + (var_t3 * var_t3_dn13)) / (2.0 * assign12120_e6683)))),)
    } else {
        (var_rd23e, var_rd23e_dn0, var_rd23e_dn2, var_rd23e_dn4, var_rd23e_dn5, var_rd23e_dn6, var_rd23e_dn7, var_rd23e_dn8, var_rd23e_dn9, var_rd23e_dn10, var_rd23e_dn13,)
    }
};
        var_rd23e = assign12120_e6687;
        var_rd23e_dn0 = assign12120_e6687_d_n0;
        var_rd23e_dn2 = assign12120_e6687_d_n2;
        var_rd23e_dn4 = assign12120_e6687_d_n4;
        var_rd23e_dn5 = assign12120_e6687_d_n5;
        var_rd23e_dn6 = assign12120_e6687_d_n6;
        var_rd23e_dn7 = assign12120_e6687_d_n7;
        var_rd23e_dn8 = assign12120_e6687_d_n8;
        var_rd23e_dn9 = assign12120_e6687_d_n9;
        var_rd23e_dn10 = assign12120_e6687_d_n10;
        var_rd23e_dn13 = assign12120_e6687_d_n13;
        var_rd23e_rv = 0.0;

        let (assign12130_e6694, assign12130_e6694_d_n0, assign12130_e6694_d_n2, assign12130_e6694_d_n4, assign12130_e6694_d_n5, assign12130_e6694_d_n6, assign12130_e6694_d_n7, assign12130_e6694_d_n8, assign12130_e6694_d_n9, assign12130_e6694_d_n10, assign12130_e6694_d_n13,) = {
    if ((var_guard273 != 0.0) && (var_guard277 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rd23e, var_rd23e_dn0, var_rd23e_dn2, var_rd23e_dn4, var_rd23e_dn5, var_rd23e_dn6, var_rd23e_dn7, var_rd23e_dn8, var_rd23e_dn9, var_rd23e_dn10, var_rd23e_dn13,)
    }
};
        var_rd23e = assign12130_e6694;
        var_rd23e_dn0 = assign12130_e6694_d_n0;
        var_rd23e_dn2 = assign12130_e6694_d_n2;
        var_rd23e_dn4 = assign12130_e6694_d_n4;
        var_rd23e_dn5 = assign12130_e6694_d_n5;
        var_rd23e_dn6 = assign12130_e6694_d_n6;
        var_rd23e_dn7 = assign12130_e6694_d_n7;
        var_rd23e_dn8 = assign12130_e6694_d_n8;
        var_rd23e_dn9 = assign12130_e6694_d_n9;
        var_rd23e_dn10 = assign12130_e6694_d_n10;
        var_rd23e_dn13 = assign12130_e6694_d_n13;
        var_rd23e_rv = 0.0;

        let (assign12140_e6698,) = {
    if (var_guard273 != 0.0) {
        (0.0,)
    } else {
        (var_xmax,)
    }
};
        var_xmax = assign12140_e6698;
        var_xmax_rv = 0.0;

        let (assign12150_e6702,) = {
    if (var_guard273 != 0.0) {
        (0.0,)
    } else {
        (var_xmax_s,)
    }
};
        var_xmax_s = assign12150_e6702;
        var_xmax_s_rv = 0.0;

        let (assign12160_e6706,) = {
    if (var_guard273 != 0.0) {
        (0.0,)
    } else {
        (var_rdrvmaxwe,)
    }
};
        var_rdrvmaxwe = assign12160_e6706;
        var_rdrvmaxwe_rv = 0.0;

        let (assign12170_e6710,) = {
    if (var_guard273 != 0.0) {
        (0.0,)
    } else {
        (var_rdrvmaxle,)
    }
};
        var_rdrvmaxle = assign12170_e6710;
        var_rdrvmaxle_rv = 0.0;

        let (assign12180_e6714,) = {
    if (var_guard273 != 0.0) {
        (0.0,)
    } else {
        (var_rdrmuele,)
    }
};
        var_rdrmuele = assign12180_e6714;
        var_rdrmuele_rv = 0.0;

        let (assign12190_e6718, assign12190_e6718_d_n0, assign12190_e6718_d_n2, assign12190_e6718_d_n4, assign12190_e6718_d_n5, assign12190_e6718_d_n6, assign12190_e6718_d_n7, assign12190_e6718_d_n8, assign12190_e6718_d_n9, assign12190_e6718_d_n10, assign12190_e6718_d_n13,) = {
    if (var_guard273 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdrmuevbs, var_rdrmuevbs_dn0, var_rdrmuevbs_dn2, var_rdrmuevbs_dn4, var_rdrmuevbs_dn5, var_rdrmuevbs_dn6, var_rdrmuevbs_dn7, var_rdrmuevbs_dn8, var_rdrmuevbs_dn9, var_rdrmuevbs_dn10, var_rdrmuevbs_dn13,)
    }
};
        var_rdrmuevbs = assign12190_e6718;
        var_rdrmuevbs_dn0 = assign12190_e6718_d_n0;
        var_rdrmuevbs_dn2 = assign12190_e6718_d_n2;
        var_rdrmuevbs_dn4 = assign12190_e6718_d_n4;
        var_rdrmuevbs_dn5 = assign12190_e6718_d_n5;
        var_rdrmuevbs_dn6 = assign12190_e6718_d_n6;
        var_rdrmuevbs_dn7 = assign12190_e6718_d_n7;
        var_rdrmuevbs_dn8 = assign12190_e6718_d_n8;
        var_rdrmuevbs_dn9 = assign12190_e6718_d_n9;
        var_rdrmuevbs_dn10 = assign12190_e6718_d_n10;
        var_rdrmuevbs_dn13 = assign12190_e6718_d_n13;
        var_rdrmuevbs_rv = 0.0;

        let (assign12200_e6730,) = {
    if (var_guard273 == 0.0) {
        let assign12200_e6723: f64 = (p.p419 * p.p419);
        let assign12200_e6726: f64 = (var_uc_xldld * var_uc_xldld);
        let assign12200_e6727: f64 = (assign12200_e6723 + assign12200_e6726);
        let assign12200_e6728: f64 = (assign12200_e6727).sqrt();
        (assign12200_e6728,)
    } else {
        (var_xmax,)
    }
};
        var_xmax = assign12200_e6730;
        var_xmax_rv = 0.0;

        let (assign12210_e6742,) = {
    if (var_guard273 == 0.0) {
        let assign12210_e6735: f64 = (p.p419 * p.p419);
        let assign12210_e6738: f64 = (p.p97 * p.p97);
        let assign12210_e6739: f64 = (assign12210_e6735 + assign12210_e6738);
        let assign12210_e6740: f64 = (assign12210_e6739).sqrt();
        (assign12210_e6740,)
    } else {
        (var_xmax_s,)
    }
};
        var_xmax_s = assign12210_e6742;
        var_xmax_s_rv = 0.0;

        let (assign12220_e6753,) = {
    if (var_guard273 == 0.0) {
        let assign12220_e6749: f64 = (var_wg).powf(p.p425);
        let assign12220_e6750: f64 = (p.p424 / assign12220_e6749);
        let assign12220_e6751: f64 = (1.0 + assign12220_e6750);
        (assign12220_e6751,)
    } else {
        (var_rdrvmaxwe,)
    }
};
        var_rdrvmaxwe = assign12220_e6753;
        var_rdrvmaxwe_rv = 0.0;

        let (assign12230_e6764,) = {
    if (var_guard273 == 0.0) {
        let assign12230_e6760: f64 = (var_lg).powf(p.p427);
        let assign12230_e6761: f64 = (p.p426 / assign12230_e6760);
        let assign12230_e6762: f64 = (1.0 + assign12230_e6761);
        (assign12230_e6762,)
    } else {
        (var_rdrvmaxle,)
    }
};
        var_rdrvmaxle = assign12230_e6764;
        var_rdrvmaxle_rv = 0.0;

        let (assign12240_e6775,) = {
    if (var_guard273 == 0.0) {
        let assign12240_e6771: f64 = (var_lg).powf(p.p429);
        let assign12240_e6772: f64 = (p.p428 / assign12240_e6771);
        let assign12240_e6773: f64 = (1.0 + assign12240_e6772);
        (assign12240_e6773,)
    } else {
        (var_rdrmuele,)
    }
};
        var_rdrmuele = assign12240_e6775;
        var_rdrmuele_rv = 0.0;

        let (assign12250_e6780, assign12250_e6780_d_n0, assign12250_e6780_d_n2, assign12250_e6780_d_n4, assign12250_e6780_d_n5, assign12250_e6780_d_n6, assign12250_e6780_d_n7, assign12250_e6780_d_n8, assign12250_e6780_d_n9, assign12250_e6780_d_n10, assign12250_e6780_d_n13,) = {
    if (var_guard273 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdrmuevbs, var_rdrmuevbs_dn0, var_rdrmuevbs_dn2, var_rdrmuevbs_dn4, var_rdrmuevbs_dn5, var_rdrmuevbs_dn6, var_rdrmuevbs_dn7, var_rdrmuevbs_dn8, var_rdrmuevbs_dn9, var_rdrmuevbs_dn10, var_rdrmuevbs_dn13,)
    }
};
        var_rdrmuevbs = assign12250_e6780;
        var_rdrmuevbs_dn0 = assign12250_e6780_d_n0;
        var_rdrmuevbs_dn2 = assign12250_e6780_d_n2;
        var_rdrmuevbs_dn4 = assign12250_e6780_d_n4;
        var_rdrmuevbs_dn5 = assign12250_e6780_d_n5;
        var_rdrmuevbs_dn6 = assign12250_e6780_d_n6;
        var_rdrmuevbs_dn7 = assign12250_e6780_d_n7;
        var_rdrmuevbs_dn8 = assign12250_e6780_d_n8;
        var_rdrmuevbs_dn9 = assign12250_e6780_d_n9;
        var_rdrmuevbs_dn10 = assign12250_e6780_d_n10;
        var_rdrmuevbs_dn13 = assign12250_e6780_d_n13;
        var_rdrmuevbs_rv = 0.0;

        let (assign12260_e6785,) = {
    if (var_guard273 == 0.0) {
        (0.0,)
    } else {
        (var_rdtemp0,)
    }
};
        var_rdtemp0 = assign12260_e6785;
        var_rdtemp0_rv = 0.0;

        let (assign12270_e6790, assign12270_e6790_d_n0, assign12270_e6790_d_n2, assign12270_e6790_d_n4, assign12270_e6790_d_n5, assign12270_e6790_d_n6, assign12270_e6790_d_n7, assign12270_e6790_d_n8, assign12270_e6790_d_n9, assign12270_e6790_d_n10, assign12270_e6790_d_n13,) = {
    if (var_guard273 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdvdtemp0, var_rdvdtemp0_dn0, var_rdvdtemp0_dn2, var_rdvdtemp0_dn4, var_rdvdtemp0_dn5, var_rdvdtemp0_dn6, var_rdvdtemp0_dn7, var_rdvdtemp0_dn8, var_rdvdtemp0_dn9, var_rdvdtemp0_dn10, var_rdvdtemp0_dn13,)
    }
};
        var_rdvdtemp0 = assign12270_e6790;
        var_rdvdtemp0_dn0 = assign12270_e6790_d_n0;
        var_rdvdtemp0_dn2 = assign12270_e6790_d_n2;
        var_rdvdtemp0_dn4 = assign12270_e6790_d_n4;
        var_rdvdtemp0_dn5 = assign12270_e6790_d_n5;
        var_rdvdtemp0_dn6 = assign12270_e6790_d_n6;
        var_rdvdtemp0_dn7 = assign12270_e6790_d_n7;
        var_rdvdtemp0_dn8 = assign12270_e6790_d_n8;
        var_rdvdtemp0_dn9 = assign12270_e6790_d_n9;
        var_rdvdtemp0_dn10 = assign12270_e6790_d_n10;
        var_rdvdtemp0_dn13 = assign12270_e6790_d_n13;
        var_rdvdtemp0_rv = 0.0;

        let (assign12280_e6795, assign12280_e6795_d_n0, assign12280_e6795_d_n2, assign12280_e6795_d_n4, assign12280_e6795_d_n5, assign12280_e6795_d_n6, assign12280_e6795_d_n7, assign12280_e6795_d_n8, assign12280_e6795_d_n9, assign12280_e6795_d_n10, assign12280_e6795_d_n13,) = {
    if (var_guard273 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rd23e, var_rd23e_dn0, var_rd23e_dn2, var_rd23e_dn4, var_rd23e_dn5, var_rd23e_dn6, var_rd23e_dn7, var_rd23e_dn8, var_rd23e_dn9, var_rd23e_dn10, var_rd23e_dn13,)
    }
};
        var_rd23e = assign12280_e6795;
        var_rd23e_dn0 = assign12280_e6795_d_n0;
        var_rd23e_dn2 = assign12280_e6795_d_n2;
        var_rd23e_dn4 = assign12280_e6795_d_n4;
        var_rd23e_dn5 = assign12280_e6795_d_n5;
        var_rd23e_dn6 = assign12280_e6795_d_n6;
        var_rd23e_dn7 = assign12280_e6795_d_n7;
        var_rd23e_dn8 = assign12280_e6795_d_n8;
        var_rd23e_dn9 = assign12280_e6795_d_n9;
        var_rd23e_dn10 = assign12280_e6795_d_n10;
        var_rd23e_dn13 = assign12280_e6795_d_n13;
        var_rd23e_rv = 0.0;

        let assign12290_e6798: f64 = if var_uc_nover > 0.0 { 1.0 } else { 0.0 };
        var_guard279 = assign12290_e6798;
        var_guard279_rv = 0.0;

        let (assign12300_e6808,) = {
    if (var_guard279 != 0.0) {
        let assign12300_e6802: f64 = (2.0 * 1.034943e-10);
        let assign12300_e6805: f64 = (1.6021918e-19 * var_uc_nover);
        let assign12300_e6806: f64 = (assign12300_e6802 / assign12300_e6805);
        (assign12300_e6806,)
    } else {
        (var_kdep,)
    }
};
        var_kdep = assign12300_e6808;
        var_kdep_rv = 0.0;

        let (assign12310_e6824, assign12310_e6824_d_n0, assign12310_e6824_d_n2, assign12310_e6824_d_n4, assign12310_e6824_d_n5, assign12310_e6824_d_n6, assign12310_e6824_d_n7, assign12310_e6824_d_n8, assign12310_e6824_d_n9, assign12310_e6824_d_n10, assign12310_e6824_d_n13,) = {
    if (var_guard279 != 0.0) {
        let assign12310_e6812: f64 = (2.0 * 1.034943e-10);
        let assign12310_e6814: f64 = (assign12310_e6812 / 1.6021918e-19);
        let assign12310_e6816: f64 = (assign12310_e6814 * var_ef_nsubc);
        let assign12310_e6819: f64 = (var_uc_nover + var_ef_nsubc);
        let assign12310_e6820: f64 = (assign12310_e6816 / assign12310_e6819);
        let assign12310_e6822: f64 = (assign12310_e6820 / var_uc_nover);
        (assign12310_e6822, (((((assign12310_e6814 * var_ef_nsubc_dn0) * assign12310_e6819) - (assign12310_e6816 * var_ef_nsubc_dn0)) / (assign12310_e6819 * assign12310_e6819)) / var_uc_nover), (((((assign12310_e6814 * var_ef_nsubc_dn2) * assign12310_e6819) - (assign12310_e6816 * var_ef_nsubc_dn2)) / (assign12310_e6819 * assign12310_e6819)) / var_uc_nover), (((((assign12310_e6814 * var_ef_nsubc_dn4) * assign12310_e6819) - (assign12310_e6816 * var_ef_nsubc_dn4)) / (assign12310_e6819 * assign12310_e6819)) / var_uc_nover), (((((assign12310_e6814 * var_ef_nsubc_dn5) * assign12310_e6819) - (assign12310_e6816 * var_ef_nsubc_dn5)) / (assign12310_e6819 * assign12310_e6819)) / var_uc_nover), (((((assign12310_e6814 * var_ef_nsubc_dn6) * assign12310_e6819) - (assign12310_e6816 * var_ef_nsubc_dn6)) / (assign12310_e6819 * assign12310_e6819)) / var_uc_nover), (((((assign12310_e6814 * var_ef_nsubc_dn7) * assign12310_e6819) - (assign12310_e6816 * var_ef_nsubc_dn7)) / (assign12310_e6819 * assign12310_e6819)) / var_uc_nover), (((((assign12310_e6814 * var_ef_nsubc_dn8) * assign12310_e6819) - (assign12310_e6816 * var_ef_nsubc_dn8)) / (assign12310_e6819 * assign12310_e6819)) / var_uc_nover), (((((assign12310_e6814 * var_ef_nsubc_dn9) * assign12310_e6819) - (assign12310_e6816 * var_ef_nsubc_dn9)) / (assign12310_e6819 * assign12310_e6819)) / var_uc_nover), (((((assign12310_e6814 * var_ef_nsubc_dn10) * assign12310_e6819) - (assign12310_e6816 * var_ef_nsubc_dn10)) / (assign12310_e6819 * assign12310_e6819)) / var_uc_nover), (((((assign12310_e6814 * var_ef_nsubc_dn13) * assign12310_e6819) - (assign12310_e6816 * var_ef_nsubc_dn13)) / (assign12310_e6819 * assign12310_e6819)) / var_uc_nover),)
    } else {
        (var_kjunc, var_kjunc_dn0, var_kjunc_dn2, var_kjunc_dn4, var_kjunc_dn5, var_kjunc_dn6, var_kjunc_dn7, var_kjunc_dn8, var_kjunc_dn9, var_kjunc_dn10, var_kjunc_dn13,)
    }
};
        var_kjunc = assign12310_e6824;
        var_kjunc_dn0 = assign12310_e6824_d_n0;
        var_kjunc_dn2 = assign12310_e6824_d_n2;
        var_kjunc_dn4 = assign12310_e6824_d_n4;
        var_kjunc_dn5 = assign12310_e6824_d_n5;
        var_kjunc_dn6 = assign12310_e6824_d_n6;
        var_kjunc_dn7 = assign12310_e6824_d_n7;
        var_kjunc_dn8 = assign12310_e6824_d_n8;
        var_kjunc_dn9 = assign12310_e6824_d_n9;
        var_kjunc_dn10 = assign12310_e6824_d_n10;
        var_kjunc_dn13 = assign12310_e6824_d_n13;
        var_kjunc_rv = 0.0;

        let (assign12320_e6829,) = {
    if (var_guard279 == 0.0) {
        (0.0,)
    } else {
        (var_kdep,)
    }
};
        var_kdep = assign12320_e6829;
        var_kdep_rv = 0.0;

        let (assign12330_e6834, assign12330_e6834_d_n0, assign12330_e6834_d_n2, assign12330_e6834_d_n4, assign12330_e6834_d_n5, assign12330_e6834_d_n6, assign12330_e6834_d_n7, assign12330_e6834_d_n8, assign12330_e6834_d_n9, assign12330_e6834_d_n10, assign12330_e6834_d_n13,) = {
    if (var_guard279 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_kjunc, var_kjunc_dn0, var_kjunc_dn2, var_kjunc_dn4, var_kjunc_dn5, var_kjunc_dn6, var_kjunc_dn7, var_kjunc_dn8, var_kjunc_dn9, var_kjunc_dn10, var_kjunc_dn13,)
    }
};
        var_kjunc = assign12330_e6834;
        var_kjunc_dn0 = assign12330_e6834_d_n0;
        var_kjunc_dn2 = assign12330_e6834_d_n2;
        var_kjunc_dn4 = assign12330_e6834_d_n4;
        var_kjunc_dn5 = assign12330_e6834_d_n5;
        var_kjunc_dn6 = assign12330_e6834_d_n6;
        var_kjunc_dn7 = assign12330_e6834_d_n7;
        var_kjunc_dn8 = assign12330_e6834_d_n8;
        var_kjunc_dn9 = assign12330_e6834_d_n9;
        var_kjunc_dn10 = assign12330_e6834_d_n10;
        var_kjunc_dn13 = assign12330_e6834_d_n13;
        var_kjunc_rv = 0.0;

        let assign12470_e6929: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard284 = assign12470_e6929;
        var_guard284_rv = 0.0;

        let (assign12480_e6937, assign12480_e6937_d_n0, assign12480_e6937_d_n2, assign12480_e6937_d_n4, assign12480_e6937_d_n5, assign12480_e6937_d_n6, assign12480_e6937_d_n7, assign12480_e6937_d_n8, assign12480_e6937_d_n9, assign12480_e6937_d_n10, assign12480_e6937_d_n13,) = {
    if (var_guard284 != 0.0) {
        let assign12480_e6933: f64 = (p.p108 * var_lg);
        let assign12480_e6935: f64 = (assign12480_e6933 + p.p109);
        (assign12480_e6935, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign12480_e6937;
        var_t1_dn0 = assign12480_e6937_d_n0;
        var_t1_dn2 = assign12480_e6937_d_n2;
        var_t1_dn4 = assign12480_e6937_d_n4;
        var_t1_dn5 = assign12480_e6937_d_n5;
        var_t1_dn6 = assign12480_e6937_d_n6;
        var_t1_dn7 = assign12480_e6937_d_n7;
        var_t1_dn8 = assign12480_e6937_d_n8;
        var_t1_dn9 = assign12480_e6937_d_n9;
        var_t1_dn10 = assign12480_e6937_d_n10;
        var_t1_dn13 = assign12480_e6937_d_n13;
        var_t1_rv = 0.0;

        let assign12490_e6940: f64 = if var_t1 < 0.0 { 1.0 } else { 0.0 };
        var_guard285 = assign12490_e6940;
        var_guard285_rv = 0.0;

        let (assign12500_e6946, assign12500_e6946_d_n0, assign12500_e6946_d_n2, assign12500_e6946_d_n4, assign12500_e6946_d_n5, assign12500_e6946_d_n6, assign12500_e6946_d_n7, assign12500_e6946_d_n8, assign12500_e6946_d_n9, assign12500_e6946_d_n10, assign12500_e6946_d_n13,) = {
    if ((var_guard284 != 0.0) && (var_guard285 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign12500_e6946;
        var_t1_dn0 = assign12500_e6946_d_n0;
        var_t1_dn2 = assign12500_e6946_d_n2;
        var_t1_dn4 = assign12500_e6946_d_n4;
        var_t1_dn5 = assign12500_e6946_d_n5;
        var_t1_dn6 = assign12500_e6946_d_n6;
        var_t1_dn7 = assign12500_e6946_d_n7;
        var_t1_dn8 = assign12500_e6946_d_n8;
        var_t1_dn9 = assign12500_e6946_d_n9;
        var_t1_dn10 = assign12500_e6946_d_n10;
        var_t1_dn13 = assign12500_e6946_d_n13;
        var_t1_rv = 0.0;

        let (assign12510_e6958, assign12510_e6958_d_n0, assign12510_e6958_d_n2, assign12510_e6958_d_n4, assign12510_e6958_d_n5, assign12510_e6958_d_n6, assign12510_e6958_d_n7, assign12510_e6958_d_n8, assign12510_e6958_d_n9, assign12510_e6958_d_n10, assign12510_e6958_d_n13,) = {
    if (var_guard284 != 0.0) {
        let assign12510_e6950: f64 = (var_t1 * p.p107);
        let assign12510_e6953: f64 = (var_t1 + p.p107);
        let assign12510_e6954: f64 = (assign12510_e6950 / assign12510_e6953);
        let assign12510_e6956: f64 = (assign12510_e6954 + 1.0);
        (assign12510_e6956, ((((var_t1_dn0 * p.p107) * assign12510_e6953) - (assign12510_e6950 * var_t1_dn0)) / (assign12510_e6953 * assign12510_e6953)), ((((var_t1_dn2 * p.p107) * assign12510_e6953) - (assign12510_e6950 * var_t1_dn2)) / (assign12510_e6953 * assign12510_e6953)), ((((var_t1_dn4 * p.p107) * assign12510_e6953) - (assign12510_e6950 * var_t1_dn4)) / (assign12510_e6953 * assign12510_e6953)), ((((var_t1_dn5 * p.p107) * assign12510_e6953) - (assign12510_e6950 * var_t1_dn5)) / (assign12510_e6953 * assign12510_e6953)), ((((var_t1_dn6 * p.p107) * assign12510_e6953) - (assign12510_e6950 * var_t1_dn6)) / (assign12510_e6953 * assign12510_e6953)), ((((var_t1_dn7 * p.p107) * assign12510_e6953) - (assign12510_e6950 * var_t1_dn7)) / (assign12510_e6953 * assign12510_e6953)), ((((var_t1_dn8 * p.p107) * assign12510_e6953) - (assign12510_e6950 * var_t1_dn8)) / (assign12510_e6953 * assign12510_e6953)), ((((var_t1_dn9 * p.p107) * assign12510_e6953) - (assign12510_e6950 * var_t1_dn9)) / (assign12510_e6953 * assign12510_e6953)), ((((var_t1_dn10 * p.p107) * assign12510_e6953) - (assign12510_e6950 * var_t1_dn10)) / (assign12510_e6953 * assign12510_e6953)), ((((var_t1_dn13 * p.p107) * assign12510_e6953) - (assign12510_e6950 * var_t1_dn13)) / (assign12510_e6953 * assign12510_e6953)),)
    } else {
        (var_ddlte, var_ddlte_dn0, var_ddlte_dn2, var_ddlte_dn4, var_ddlte_dn5, var_ddlte_dn6, var_ddlte_dn7, var_ddlte_dn8, var_ddlte_dn9, var_ddlte_dn10, var_ddlte_dn13,)
    }
};
        var_ddlte = assign12510_e6958;
        var_ddlte_dn0 = assign12510_e6958_d_n0;
        var_ddlte_dn2 = assign12510_e6958_d_n2;
        var_ddlte_dn4 = assign12510_e6958_d_n4;
        var_ddlte_dn5 = assign12510_e6958_d_n5;
        var_ddlte_dn6 = assign12510_e6958_d_n6;
        var_ddlte_dn7 = assign12510_e6958_d_n7;
        var_ddlte_dn8 = assign12510_e6958_d_n8;
        var_ddlte_dn9 = assign12510_e6958_d_n9;
        var_ddlte_dn10 = assign12510_e6958_d_n10;
        var_ddlte_dn13 = assign12510_e6958_d_n13;
        var_ddlte_rv = 0.0;

        let (assign12520_e6965, assign12520_e6965_d_n0, assign12520_e6965_d_n2, assign12520_e6965_d_n4, assign12520_e6965_d_n5, assign12520_e6965_d_n6, assign12520_e6965_d_n7, assign12520_e6965_d_n8, assign12520_e6965_d_n9, assign12520_e6965_d_n10, assign12520_e6965_d_n13,) = {
    if (var_guard284 == 0.0) {
        let assign12520_e6963: f64 = (p.p108 * var_lg);
        (assign12520_e6963, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign12520_e6965;
        var_t1_dn0 = assign12520_e6965_d_n0;
        var_t1_dn2 = assign12520_e6965_d_n2;
        var_t1_dn4 = assign12520_e6965_d_n4;
        var_t1_dn5 = assign12520_e6965_d_n5;
        var_t1_dn6 = assign12520_e6965_d_n6;
        var_t1_dn7 = assign12520_e6965_d_n7;
        var_t1_dn8 = assign12520_e6965_d_n8;
        var_t1_dn9 = assign12520_e6965_d_n9;
        var_t1_dn10 = assign12520_e6965_d_n10;
        var_t1_dn13 = assign12520_e6965_d_n13;
        var_t1_rv = 0.0;

        let assign12530_e6968: f64 = if var_t1 < 0.0 { 1.0 } else { 0.0 };
        var_guard286 = assign12530_e6968;
        var_guard286_rv = 0.0;

        let (assign12540_e6975, assign12540_e6975_d_n0, assign12540_e6975_d_n2, assign12540_e6975_d_n4, assign12540_e6975_d_n5, assign12540_e6975_d_n6, assign12540_e6975_d_n7, assign12540_e6975_d_n8, assign12540_e6975_d_n9, assign12540_e6975_d_n10, assign12540_e6975_d_n13,) = {
    if ((var_guard284 == 0.0) && (var_guard286 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign12540_e6975;
        var_t1_dn0 = assign12540_e6975_d_n0;
        var_t1_dn2 = assign12540_e6975_d_n2;
        var_t1_dn4 = assign12540_e6975_d_n4;
        var_t1_dn5 = assign12540_e6975_d_n5;
        var_t1_dn6 = assign12540_e6975_d_n6;
        var_t1_dn7 = assign12540_e6975_d_n7;
        var_t1_dn8 = assign12540_e6975_d_n8;
        var_t1_dn9 = assign12540_e6975_d_n9;
        var_t1_dn10 = assign12540_e6975_d_n10;
        var_t1_dn13 = assign12540_e6975_d_n13;
        var_t1_rv = 0.0;

        *var_ddlte_slot = var_ddlte;
        *var_ddlte_dn0_slot = var_ddlte_dn0;
        *var_ddlte_dn10_slot = var_ddlte_dn10;
        *var_ddlte_dn13_slot = var_ddlte_dn13;
        *var_ddlte_dn2_slot = var_ddlte_dn2;
        *var_ddlte_dn4_slot = var_ddlte_dn4;
        *var_ddlte_dn5_slot = var_ddlte_dn5;
        *var_ddlte_dn6_slot = var_ddlte_dn6;
        *var_ddlte_dn7_slot = var_ddlte_dn7;
        *var_ddlte_dn8_slot = var_ddlte_dn8;
        *var_ddlte_dn9_slot = var_ddlte_dn9;
        *var_ddlte_rv_slot = var_ddlte_rv;
        *var_guard279_slot = var_guard279;
        *var_guard279_rv_slot = var_guard279_rv;
        *var_guard284_slot = var_guard284;
        *var_guard284_rv_slot = var_guard284_rv;
        *var_guard285_slot = var_guard285;
        *var_guard285_rv_slot = var_guard285_rv;
        *var_guard286_slot = var_guard286;
        *var_guard286_rv_slot = var_guard286_rv;
        *var_kdep_slot = var_kdep;
        *var_kdep_rv_slot = var_kdep_rv;
        *var_kjunc_slot = var_kjunc;
        *var_kjunc_dn0_slot = var_kjunc_dn0;
        *var_kjunc_dn10_slot = var_kjunc_dn10;
        *var_kjunc_dn13_slot = var_kjunc_dn13;
        *var_kjunc_dn2_slot = var_kjunc_dn2;
        *var_kjunc_dn4_slot = var_kjunc_dn4;
        *var_kjunc_dn5_slot = var_kjunc_dn5;
        *var_kjunc_dn6_slot = var_kjunc_dn6;
        *var_kjunc_dn7_slot = var_kjunc_dn7;
        *var_kjunc_dn8_slot = var_kjunc_dn8;
        *var_kjunc_dn9_slot = var_kjunc_dn9;
        *var_kjunc_rv_slot = var_kjunc_rv;
        *var_rd23e_slot = var_rd23e;
        *var_rd23e_dn0_slot = var_rd23e_dn0;
        *var_rd23e_dn10_slot = var_rd23e_dn10;
        *var_rd23e_dn13_slot = var_rd23e_dn13;
        *var_rd23e_dn2_slot = var_rd23e_dn2;
        *var_rd23e_dn4_slot = var_rd23e_dn4;
        *var_rd23e_dn5_slot = var_rd23e_dn5;
        *var_rd23e_dn6_slot = var_rd23e_dn6;
        *var_rd23e_dn7_slot = var_rd23e_dn7;
        *var_rd23e_dn8_slot = var_rd23e_dn8;
        *var_rd23e_dn9_slot = var_rd23e_dn9;
        *var_rd23e_rv_slot = var_rd23e_rv;
        *var_rdrmuele_slot = var_rdrmuele;
        *var_rdrmuele_rv_slot = var_rdrmuele_rv;
        *var_rdrmuevbs_slot = var_rdrmuevbs;
        *var_rdrmuevbs_dn0_slot = var_rdrmuevbs_dn0;
        *var_rdrmuevbs_dn10_slot = var_rdrmuevbs_dn10;
        *var_rdrmuevbs_dn13_slot = var_rdrmuevbs_dn13;
        *var_rdrmuevbs_dn2_slot = var_rdrmuevbs_dn2;
        *var_rdrmuevbs_dn4_slot = var_rdrmuevbs_dn4;
        *var_rdrmuevbs_dn5_slot = var_rdrmuevbs_dn5;
        *var_rdrmuevbs_dn6_slot = var_rdrmuevbs_dn6;
        *var_rdrmuevbs_dn7_slot = var_rdrmuevbs_dn7;
        *var_rdrmuevbs_dn8_slot = var_rdrmuevbs_dn8;
        *var_rdrmuevbs_dn9_slot = var_rdrmuevbs_dn9;
        *var_rdrmuevbs_rv_slot = var_rdrmuevbs_rv;
        *var_rdrvmaxle_slot = var_rdrvmaxle;
        *var_rdrvmaxle_rv_slot = var_rdrvmaxle_rv;
        *var_rdrvmaxwe_slot = var_rdrvmaxwe;
        *var_rdrvmaxwe_rv_slot = var_rdrvmaxwe_rv;
        *var_rdtemp0_slot = var_rdtemp0;
        *var_rdtemp0_rv_slot = var_rdtemp0_rv;
        *var_rdvdtemp0_slot = var_rdvdtemp0;
        *var_rdvdtemp0_dn0_slot = var_rdvdtemp0_dn0;
        *var_rdvdtemp0_dn10_slot = var_rdvdtemp0_dn10;
        *var_rdvdtemp0_dn13_slot = var_rdvdtemp0_dn13;
        *var_rdvdtemp0_dn2_slot = var_rdvdtemp0_dn2;
        *var_rdvdtemp0_dn4_slot = var_rdvdtemp0_dn4;
        *var_rdvdtemp0_dn5_slot = var_rdvdtemp0_dn5;
        *var_rdvdtemp0_dn6_slot = var_rdvdtemp0_dn6;
        *var_rdvdtemp0_dn7_slot = var_rdvdtemp0_dn7;
        *var_rdvdtemp0_dn8_slot = var_rdvdtemp0_dn8;
        *var_rdvdtemp0_dn9_slot = var_rdvdtemp0_dn9;
        *var_rdvdtemp0_rv_slot = var_rdvdtemp0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_xmax_slot = var_xmax;
        *var_xmax_rv_slot = var_xmax_rv;
        *var_xmax_s_slot = var_xmax_s;
        *var_xmax_s_rv_slot = var_xmax_s_rv;
    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        var_guard284: f64,
        var_lg: f64,
        var_lgate: f64,
        var_mfactor: f64,
        var_mks_slg: f64,
        var_mks_slgl: f64,
        var_mks_sub1l: f64,
        var_mks_sub2l: f64,
        var_mks_svbsl: f64,
        var_mks_svgsl: f64,
        var_mks_svgsw: f64,
        var_t1: f64,
        var_t1_dn0: f64,
        var_t1_dn10: f64,
        var_t1_dn13: f64,
        var_t1_dn2: f64,
        var_t1_dn4: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_uc_gdld: f64,
        var_uc_rth0: f64,
        var_uc_sub1: f64,
        var_uc_sub1snp: f64,
        var_uc_sub2: f64,
        var_uc_sub2snp: f64,
        var_uc_svbs: f64,
        var_uc_svgs: f64,
        var_weff: f64,
        var_weff_nf: f64,
        var_weffcv_nf: f64,
        var_wg: f64,
        var_cfrng_slot: &mut f64,
        var_cfrng_rv_slot: &mut f64,
        var_cqyb0_slot: &mut f64,
        var_cqyb0_rv_slot: &mut f64,
        var_ddlte_slot: &mut f64,
        var_ddlte_dn0_slot: &mut f64,
        var_ddlte_dn10_slot: &mut f64,
        var_ddlte_dn13_slot: &mut f64,
        var_ddlte_dn2_slot: &mut f64,
        var_ddlte_dn4_slot: &mut f64,
        var_ddlte_dn5_slot: &mut f64,
        var_ddlte_dn6_slot: &mut f64,
        var_ddlte_dn7_slot: &mut f64,
        var_ddlte_dn8_slot: &mut f64,
        var_ddlte_dn9_slot: &mut f64,
        var_ddlte_rv_slot: &mut f64,
        var_gdl0_slot: &mut f64,
        var_gdl0_rv_slot: &mut f64,
        var_guard288_slot: &mut f64,
        var_guard288_rv_slot: &mut f64,
        var_guard289_slot: &mut f64,
        var_guard289_rv_slot: &mut f64,
        var_pt40_slot: &mut f64,
        var_pt40_rv_slot: &mut f64,
        var_ptl0_slot: &mut f64,
        var_ptl0_rv_slot: &mut f64,
        var_rth_slot: &mut f64,
        var_rth_dn0_slot: &mut f64,
        var_rth_dn10_slot: &mut f64,
        var_rth_dn13_slot: &mut f64,
        var_rth_dn2_slot: &mut f64,
        var_rth_dn4_slot: &mut f64,
        var_rth_dn5_slot: &mut f64,
        var_rth_dn6_slot: &mut f64,
        var_rth_dn7_slot: &mut f64,
        var_rth_dn8_slot: &mut f64,
        var_rth_dn9_slot: &mut f64,
        var_rth_rv_slot: &mut f64,
        var_rthtemp0_slot: &mut f64,
        var_rthtemp0_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_uc_ibpc1_slot: &mut f64,
        var_uc_ibpc1_rv_slot: &mut f64,
        var_uc_subld1_slot: &mut f64,
        var_uc_subld1_rv_slot: &mut f64,
        var_vg2const_slot: &mut f64,
        var_vg2const_1_slot: &mut f64,
        var_vg2const_1_dn0_slot: &mut f64,
        var_vg2const_1_dn10_slot: &mut f64,
        var_vg2const_1_dn13_slot: &mut f64,
        var_vg2const_1_dn2_slot: &mut f64,
        var_vg2const_1_dn4_slot: &mut f64,
        var_vg2const_1_dn5_slot: &mut f64,
        var_vg2const_1_dn6_slot: &mut f64,
        var_vg2const_1_dn7_slot: &mut f64,
        var_vg2const_1_dn8_slot: &mut f64,
        var_vg2const_1_dn9_slot: &mut f64,
        var_vg2const_1_rv_slot: &mut f64,
        var_vg2const_dn0_slot: &mut f64,
        var_vg2const_dn10_slot: &mut f64,
        var_vg2const_dn13_slot: &mut f64,
        var_vg2const_dn2_slot: &mut f64,
        var_vg2const_dn4_slot: &mut f64,
        var_vg2const_dn5_slot: &mut f64,
        var_vg2const_dn6_slot: &mut f64,
        var_vg2const_dn7_slot: &mut f64,
        var_vg2const_dn8_slot: &mut f64,
        var_vg2const_dn9_slot: &mut f64,
        var_vg2const_rv_slot: &mut f64,
        var_xgate_slot: &mut f64,
        var_xgate_1_slot: &mut f64,
        var_xgate_1_rv_slot: &mut f64,
        var_xgate_rv_slot: &mut f64,
        var_xsub1_slot: &mut f64,
        var_xsub1_1_slot: &mut f64,
        var_xsub1_1_rv_slot: &mut f64,
        var_xsub1_rv_slot: &mut f64,
        var_xsub2_slot: &mut f64,
        var_xsub2_1_slot: &mut f64,
        var_xsub2_1_rv_slot: &mut f64,
        var_xsub2_rv_slot: &mut f64,
        var_xvbs_slot: &mut f64,
        var_xvbs_1_slot: &mut f64,
        var_xvbs_1_rv_slot: &mut f64,
        var_xvbs_rv_slot: &mut f64,
    ) {
        let mut var_cfrng: f64 = *var_cfrng_slot;
        let mut var_cfrng_rv: f64 = *var_cfrng_rv_slot;
        let mut var_cqyb0: f64 = *var_cqyb0_slot;
        let mut var_cqyb0_rv: f64 = *var_cqyb0_rv_slot;
        let mut var_ddlte: f64 = *var_ddlte_slot;
        let mut var_ddlte_dn0: f64 = *var_ddlte_dn0_slot;
        let mut var_ddlte_dn10: f64 = *var_ddlte_dn10_slot;
        let mut var_ddlte_dn13: f64 = *var_ddlte_dn13_slot;
        let mut var_ddlte_dn2: f64 = *var_ddlte_dn2_slot;
        let mut var_ddlte_dn4: f64 = *var_ddlte_dn4_slot;
        let mut var_ddlte_dn5: f64 = *var_ddlte_dn5_slot;
        let mut var_ddlte_dn6: f64 = *var_ddlte_dn6_slot;
        let mut var_ddlte_dn7: f64 = *var_ddlte_dn7_slot;
        let mut var_ddlte_dn8: f64 = *var_ddlte_dn8_slot;
        let mut var_ddlte_dn9: f64 = *var_ddlte_dn9_slot;
        let mut var_ddlte_rv: f64 = *var_ddlte_rv_slot;
        let mut var_gdl0: f64 = *var_gdl0_slot;
        let mut var_gdl0_rv: f64 = *var_gdl0_rv_slot;
        let mut var_guard288: f64 = *var_guard288_slot;
        let mut var_guard288_rv: f64 = *var_guard288_rv_slot;
        let mut var_guard289: f64 = *var_guard289_slot;
        let mut var_guard289_rv: f64 = *var_guard289_rv_slot;
        let mut var_pt40: f64 = *var_pt40_slot;
        let mut var_pt40_rv: f64 = *var_pt40_rv_slot;
        let mut var_ptl0: f64 = *var_ptl0_slot;
        let mut var_ptl0_rv: f64 = *var_ptl0_rv_slot;
        let mut var_rth: f64 = *var_rth_slot;
        let mut var_rth_dn0: f64 = *var_rth_dn0_slot;
        let mut var_rth_dn10: f64 = *var_rth_dn10_slot;
        let mut var_rth_dn13: f64 = *var_rth_dn13_slot;
        let mut var_rth_dn2: f64 = *var_rth_dn2_slot;
        let mut var_rth_dn4: f64 = *var_rth_dn4_slot;
        let mut var_rth_dn5: f64 = *var_rth_dn5_slot;
        let mut var_rth_dn6: f64 = *var_rth_dn6_slot;
        let mut var_rth_dn7: f64 = *var_rth_dn7_slot;
        let mut var_rth_dn8: f64 = *var_rth_dn8_slot;
        let mut var_rth_dn9: f64 = *var_rth_dn9_slot;
        let mut var_rth_rv: f64 = *var_rth_rv_slot;
        let mut var_rthtemp0: f64 = *var_rthtemp0_slot;
        let mut var_rthtemp0_rv: f64 = *var_rthtemp0_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_uc_ibpc1: f64 = *var_uc_ibpc1_slot;
        let mut var_uc_ibpc1_rv: f64 = *var_uc_ibpc1_rv_slot;
        let mut var_uc_subld1: f64 = *var_uc_subld1_slot;
        let mut var_uc_subld1_rv: f64 = *var_uc_subld1_rv_slot;
        let mut var_vg2const: f64 = *var_vg2const_slot;
        let mut var_vg2const_1: f64 = *var_vg2const_1_slot;
        let mut var_vg2const_1_dn0: f64 = *var_vg2const_1_dn0_slot;
        let mut var_vg2const_1_dn10: f64 = *var_vg2const_1_dn10_slot;
        let mut var_vg2const_1_dn13: f64 = *var_vg2const_1_dn13_slot;
        let mut var_vg2const_1_dn2: f64 = *var_vg2const_1_dn2_slot;
        let mut var_vg2const_1_dn4: f64 = *var_vg2const_1_dn4_slot;
        let mut var_vg2const_1_dn5: f64 = *var_vg2const_1_dn5_slot;
        let mut var_vg2const_1_dn6: f64 = *var_vg2const_1_dn6_slot;
        let mut var_vg2const_1_dn7: f64 = *var_vg2const_1_dn7_slot;
        let mut var_vg2const_1_dn8: f64 = *var_vg2const_1_dn8_slot;
        let mut var_vg2const_1_dn9: f64 = *var_vg2const_1_dn9_slot;
        let mut var_vg2const_1_rv: f64 = *var_vg2const_1_rv_slot;
        let mut var_vg2const_dn0: f64 = *var_vg2const_dn0_slot;
        let mut var_vg2const_dn10: f64 = *var_vg2const_dn10_slot;
        let mut var_vg2const_dn13: f64 = *var_vg2const_dn13_slot;
        let mut var_vg2const_dn2: f64 = *var_vg2const_dn2_slot;
        let mut var_vg2const_dn4: f64 = *var_vg2const_dn4_slot;
        let mut var_vg2const_dn5: f64 = *var_vg2const_dn5_slot;
        let mut var_vg2const_dn6: f64 = *var_vg2const_dn6_slot;
        let mut var_vg2const_dn7: f64 = *var_vg2const_dn7_slot;
        let mut var_vg2const_dn8: f64 = *var_vg2const_dn8_slot;
        let mut var_vg2const_dn9: f64 = *var_vg2const_dn9_slot;
        let mut var_vg2const_rv: f64 = *var_vg2const_rv_slot;
        let mut var_xgate: f64 = *var_xgate_slot;
        let mut var_xgate_1: f64 = *var_xgate_1_slot;
        let mut var_xgate_1_rv: f64 = *var_xgate_1_rv_slot;
        let mut var_xgate_rv: f64 = *var_xgate_rv_slot;
        let mut var_xsub1: f64 = *var_xsub1_slot;
        let mut var_xsub1_1: f64 = *var_xsub1_1_slot;
        let mut var_xsub1_1_rv: f64 = *var_xsub1_1_rv_slot;
        let mut var_xsub1_rv: f64 = *var_xsub1_rv_slot;
        let mut var_xsub2: f64 = *var_xsub2_slot;
        let mut var_xsub2_1: f64 = *var_xsub2_1_slot;
        let mut var_xsub2_1_rv: f64 = *var_xsub2_1_rv_slot;
        let mut var_xsub2_rv: f64 = *var_xsub2_rv_slot;
        let mut var_xvbs: f64 = *var_xvbs_slot;
        let mut var_xvbs_1: f64 = *var_xvbs_1_slot;
        let mut var_xvbs_1_rv: f64 = *var_xvbs_1_rv_slot;
        let mut var_xvbs_rv: f64 = *var_xvbs_rv_slot;

        let (assign12550_e6990, assign12550_e6990_d_n0, assign12550_e6990_d_n2, assign12550_e6990_d_n4, assign12550_e6990_d_n5, assign12550_e6990_d_n6, assign12550_e6990_d_n7, assign12550_e6990_d_n8, assign12550_e6990_d_n9, assign12550_e6990_d_n10, assign12550_e6990_d_n13,) = {
    if (var_guard284 == 0.0) {
        let assign12550_e6980: f64 = (var_t1 * p.p107);
        let assign12550_e6983: f64 = (var_t1 + p.p107);
        let assign12550_e6984: f64 = (assign12550_e6980 / assign12550_e6983);
        let assign12550_e6986: f64 = (assign12550_e6984 + p.p109);
        let assign12550_e6988: f64 = (assign12550_e6986 + 1e-25);
        (assign12550_e6988, ((((var_t1_dn0 * p.p107) * assign12550_e6983) - (assign12550_e6980 * var_t1_dn0)) / (assign12550_e6983 * assign12550_e6983)), ((((var_t1_dn2 * p.p107) * assign12550_e6983) - (assign12550_e6980 * var_t1_dn2)) / (assign12550_e6983 * assign12550_e6983)), ((((var_t1_dn4 * p.p107) * assign12550_e6983) - (assign12550_e6980 * var_t1_dn4)) / (assign12550_e6983 * assign12550_e6983)), ((((var_t1_dn5 * p.p107) * assign12550_e6983) - (assign12550_e6980 * var_t1_dn5)) / (assign12550_e6983 * assign12550_e6983)), ((((var_t1_dn6 * p.p107) * assign12550_e6983) - (assign12550_e6980 * var_t1_dn6)) / (assign12550_e6983 * assign12550_e6983)), ((((var_t1_dn7 * p.p107) * assign12550_e6983) - (assign12550_e6980 * var_t1_dn7)) / (assign12550_e6983 * assign12550_e6983)), ((((var_t1_dn8 * p.p107) * assign12550_e6983) - (assign12550_e6980 * var_t1_dn8)) / (assign12550_e6983 * assign12550_e6983)), ((((var_t1_dn9 * p.p107) * assign12550_e6983) - (assign12550_e6980 * var_t1_dn9)) / (assign12550_e6983 * assign12550_e6983)), ((((var_t1_dn10 * p.p107) * assign12550_e6983) - (assign12550_e6980 * var_t1_dn10)) / (assign12550_e6983 * assign12550_e6983)), ((((var_t1_dn13 * p.p107) * assign12550_e6983) - (assign12550_e6980 * var_t1_dn13)) / (assign12550_e6983 * assign12550_e6983)),)
    } else {
        (var_ddlte, var_ddlte_dn0, var_ddlte_dn2, var_ddlte_dn4, var_ddlte_dn5, var_ddlte_dn6, var_ddlte_dn7, var_ddlte_dn8, var_ddlte_dn9, var_ddlte_dn10, var_ddlte_dn13,)
    }
};
        var_ddlte = assign12550_e6990;
        var_ddlte_dn0 = assign12550_e6990_d_n0;
        var_ddlte_dn2 = assign12550_e6990_d_n2;
        var_ddlte_dn4 = assign12550_e6990_d_n4;
        var_ddlte_dn5 = assign12550_e6990_d_n5;
        var_ddlte_dn6 = assign12550_e6990_d_n6;
        var_ddlte_dn7 = assign12550_e6990_d_n7;
        var_ddlte_dn8 = assign12550_e6990_d_n8;
        var_ddlte_dn9 = assign12550_e6990_d_n9;
        var_ddlte_dn10 = assign12550_e6990_d_n10;
        var_ddlte_dn13 = assign12550_e6990_d_n13;
        var_ddlte_rv = 0.0;

        let assign12570_e6998: f64 = if var_ddlte < 0.1 { 1.0 } else { 0.0 };
        var_guard288 = assign12570_e6998;
        var_guard288_rv = 0.0;

        let (assign12580_e7002, assign12580_e7002_d_n0, assign12580_e7002_d_n2, assign12580_e7002_d_n4, assign12580_e7002_d_n5, assign12580_e7002_d_n6, assign12580_e7002_d_n7, assign12580_e7002_d_n8, assign12580_e7002_d_n9, assign12580_e7002_d_n10, assign12580_e7002_d_n13,) = {
    if (var_guard288 != 0.0) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ddlte, var_ddlte_dn0, var_ddlte_dn2, var_ddlte_dn4, var_ddlte_dn5, var_ddlte_dn6, var_ddlte_dn7, var_ddlte_dn8, var_ddlte_dn9, var_ddlte_dn10, var_ddlte_dn13,)
    }
};
        var_ddlte = assign12580_e7002;
        var_ddlte_dn0 = assign12580_e7002_d_n0;
        var_ddlte_dn2 = assign12580_e7002_d_n2;
        var_ddlte_dn4 = assign12580_e7002_d_n4;
        var_ddlte_dn5 = assign12580_e7002_d_n5;
        var_ddlte_dn6 = assign12580_e7002_d_n6;
        var_ddlte_dn7 = assign12580_e7002_d_n7;
        var_ddlte_dn8 = assign12580_e7002_d_n8;
        var_ddlte_dn9 = assign12580_e7002_d_n9;
        var_ddlte_dn10 = assign12580_e7002_d_n10;
        var_ddlte_dn13 = assign12580_e7002_d_n13;
        var_ddlte_rv = 0.0;

        let (assign12590_e7008, assign12590_e7008_d_n0, assign12590_e7008_d_n2, assign12590_e7008_d_n4, assign12590_e7008_d_n5, assign12590_e7008_d_n6, assign12590_e7008_d_n7, assign12590_e7008_d_n8, assign12590_e7008_d_n9, assign12590_e7008_d_n10, assign12590_e7008_d_n13,) = {
    if (p.p23 != 0.0) {
        let assign12590_e7006: f64 = (var_weff).powf(p.p201);
        (assign12590_e7006, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign12590_e7008;
        var_t2_dn0 = assign12590_e7008_d_n0;
        var_t2_dn2 = assign12590_e7008_d_n2;
        var_t2_dn4 = assign12590_e7008_d_n4;
        var_t2_dn5 = assign12590_e7008_d_n5;
        var_t2_dn6 = assign12590_e7008_d_n6;
        var_t2_dn7 = assign12590_e7008_d_n7;
        var_t2_dn8 = assign12590_e7008_d_n8;
        var_t2_dn9 = assign12590_e7008_d_n9;
        var_t2_dn10 = assign12590_e7008_d_n10;
        var_t2_dn13 = assign12590_e7008_d_n13;
        var_t2_rv = 0.0;

        let (assign12600_e7026, assign12600_e7026_d_n0, assign12600_e7026_d_n2, assign12600_e7026_d_n4, assign12600_e7026_d_n5, assign12600_e7026_d_n6, assign12600_e7026_d_n7, assign12600_e7026_d_n8, assign12600_e7026_d_n9, assign12600_e7026_d_n10, assign12600_e7026_d_n13,) = {
    if (p.p23 != 0.0) {
        let assign12600_e7015: f64 = (var_lgate).powf(p.p199);
        let assign12600_e7016: f64 = (var_mks_svgsl / assign12600_e7015);
        let assign12600_e7017: f64 = (1.0 + assign12600_e7016);
        let assign12600_e7018: f64 = (var_uc_svgs * assign12600_e7017);
        let assign12600_e7022: f64 = (var_t2 + var_mks_svgsw);
        let assign12600_e7023: f64 = (var_t2 / assign12600_e7022);
        let assign12600_e7024: f64 = (assign12600_e7018 * assign12600_e7023);
        (assign12600_e7024, (assign12600_e7018 * (((var_t2_dn0 * assign12600_e7022) - (var_t2 * var_t2_dn0)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((var_t2_dn2 * assign12600_e7022) - (var_t2 * var_t2_dn2)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((var_t2_dn4 * assign12600_e7022) - (var_t2 * var_t2_dn4)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((var_t2_dn5 * assign12600_e7022) - (var_t2 * var_t2_dn5)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((var_t2_dn6 * assign12600_e7022) - (var_t2 * var_t2_dn6)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((var_t2_dn7 * assign12600_e7022) - (var_t2 * var_t2_dn7)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((var_t2_dn8 * assign12600_e7022) - (var_t2 * var_t2_dn8)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((var_t2_dn9 * assign12600_e7022) - (var_t2 * var_t2_dn9)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((var_t2_dn10 * assign12600_e7022) - (var_t2 * var_t2_dn10)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((var_t2_dn13 * assign12600_e7022) - (var_t2 * var_t2_dn13)) / (assign12600_e7022 * assign12600_e7022))),)
    } else {
        (var_vg2const, var_vg2const_dn0, var_vg2const_dn2, var_vg2const_dn4, var_vg2const_dn5, var_vg2const_dn6, var_vg2const_dn7, var_vg2const_dn8, var_vg2const_dn9, var_vg2const_dn10, var_vg2const_dn13,)
    }
};
        var_vg2const = assign12600_e7026;
        var_vg2const_dn0 = assign12600_e7026_d_n0;
        var_vg2const_dn2 = assign12600_e7026_d_n2;
        var_vg2const_dn4 = assign12600_e7026_d_n4;
        var_vg2const_dn5 = assign12600_e7026_d_n5;
        var_vg2const_dn6 = assign12600_e7026_d_n6;
        var_vg2const_dn7 = assign12600_e7026_d_n7;
        var_vg2const_dn8 = assign12600_e7026_d_n8;
        var_vg2const_dn9 = assign12600_e7026_d_n9;
        var_vg2const_dn10 = assign12600_e7026_d_n10;
        var_vg2const_dn13 = assign12600_e7026_d_n13;
        var_vg2const_rv = 0.0;

        let (assign12610_e7038,) = {
    if (p.p23 != 0.0) {
        let assign12610_e7033: f64 = (var_lgate).powf(p.p184);
        let assign12610_e7034: f64 = (var_mks_svbsl / assign12610_e7033);
        let assign12610_e7035: f64 = (1.0 + assign12610_e7034);
        let assign12610_e7036: f64 = (var_uc_svbs * assign12610_e7035);
        (assign12610_e7036,)
    } else {
        (var_xvbs,)
    }
};
        var_xvbs = assign12610_e7038;
        var_xvbs_rv = 0.0;

        let (assign12620_e7050,) = {
    if (p.p23 != 0.0) {
        let assign12620_e7045: f64 = (var_lgate).powf(p.p203);
        let assign12620_e7046: f64 = (var_mks_slgl / assign12620_e7045);
        let assign12620_e7047: f64 = (1.0 + assign12620_e7046);
        let assign12620_e7048: f64 = (var_mks_slg * assign12620_e7047);
        (assign12620_e7048,)
    } else {
        (var_xgate,)
    }
};
        var_xgate = assign12620_e7050;
        var_xgate_rv = 0.0;

        let (assign12630_e7062,) = {
    if (p.p23 != 0.0) {
        let assign12630_e7057: f64 = (var_lgate).powf(p.p191);
        let assign12630_e7058: f64 = (var_mks_sub1l / assign12630_e7057);
        let assign12630_e7059: f64 = (1.0 + assign12630_e7058);
        let assign12630_e7060: f64 = (var_uc_sub1 * assign12630_e7059);
        (assign12630_e7060,)
    } else {
        (var_xsub1,)
    }
};
        var_xsub1 = assign12630_e7062;
        var_xsub1_rv = 0.0;

        let (assign12640_e7072,) = {
    if (p.p23 != 0.0) {
        let assign12640_e7068: f64 = (var_mks_sub2l / var_lgate);
        let assign12640_e7069: f64 = (1.0 + assign12640_e7068);
        let assign12640_e7070: f64 = (var_uc_sub2 * assign12640_e7069);
        (assign12640_e7070,)
    } else {
        (var_xsub2,)
    }
};
        var_xsub2 = assign12640_e7072;
        var_xsub2_rv = 0.0;

        let (assign12650_e7076,) = {
    if (p.p23 != 0.0) {
        (var_xsub1,)
    } else {
        (var_xsub1_1,)
    }
};
        var_xsub1_1 = assign12650_e7076;
        var_xsub1_1_rv = 0.0;

        let (assign12660_e7080,) = {
    if (p.p23 != 0.0) {
        (var_xsub2,)
    } else {
        (var_xsub2_1,)
    }
};
        var_xsub2_1 = assign12660_e7080;
        var_xsub2_1_rv = 0.0;

        let (assign12670_e7084, assign12670_e7084_d_n0, assign12670_e7084_d_n2, assign12670_e7084_d_n4, assign12670_e7084_d_n5, assign12670_e7084_d_n6, assign12670_e7084_d_n7, assign12670_e7084_d_n8, assign12670_e7084_d_n9, assign12670_e7084_d_n10, assign12670_e7084_d_n13,) = {
    if (p.p23 != 0.0) {
        (var_vg2const, var_vg2const_dn0, var_vg2const_dn2, var_vg2const_dn4, var_vg2const_dn5, var_vg2const_dn6, var_vg2const_dn7, var_vg2const_dn8, var_vg2const_dn9, var_vg2const_dn10, var_vg2const_dn13,)
    } else {
        (var_vg2const_1, var_vg2const_1_dn0, var_vg2const_1_dn2, var_vg2const_1_dn4, var_vg2const_1_dn5, var_vg2const_1_dn6, var_vg2const_1_dn7, var_vg2const_1_dn8, var_vg2const_1_dn9, var_vg2const_1_dn10, var_vg2const_1_dn13,)
    }
};
        var_vg2const_1 = assign12670_e7084;
        var_vg2const_1_dn0 = assign12670_e7084_d_n0;
        var_vg2const_1_dn2 = assign12670_e7084_d_n2;
        var_vg2const_1_dn4 = assign12670_e7084_d_n4;
        var_vg2const_1_dn5 = assign12670_e7084_d_n5;
        var_vg2const_1_dn6 = assign12670_e7084_d_n6;
        var_vg2const_1_dn7 = assign12670_e7084_d_n7;
        var_vg2const_1_dn8 = assign12670_e7084_d_n8;
        var_vg2const_1_dn9 = assign12670_e7084_d_n9;
        var_vg2const_1_dn10 = assign12670_e7084_d_n10;
        var_vg2const_1_dn13 = assign12670_e7084_d_n13;
        var_vg2const_1_rv = 0.0;

        let (assign12680_e7088,) = {
    if (p.p23 != 0.0) {
        (var_xvbs,)
    } else {
        (var_xvbs_1,)
    }
};
        var_xvbs_1 = assign12680_e7088;
        var_xvbs_1_rv = 0.0;

        let (assign12690_e7092,) = {
    if (p.p23 != 0.0) {
        (var_xgate,)
    } else {
        (var_xgate_1,)
    }
};
        var_xgate_1 = assign12690_e7092;
        var_xgate_1_rv = 0.0;

        let (assign12700_e7106,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12700_e7101: f64 = (var_lgate).powf(p.p191);
        let assign12700_e7102: f64 = (var_mks_sub1l / assign12700_e7101);
        let assign12700_e7103: f64 = (1.0 + assign12700_e7102);
        let assign12700_e7104: f64 = (var_uc_sub1snp * assign12700_e7103);
        (assign12700_e7104,)
    } else {
        (var_xsub1_1,)
    }
};
        var_xsub1_1 = assign12700_e7106;
        var_xsub1_1_rv = 0.0;

        let (assign12710_e7118,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12710_e7114: f64 = (var_mks_sub2l / var_lgate);
        let assign12710_e7115: f64 = (1.0 + assign12710_e7114);
        let assign12710_e7116: f64 = (var_uc_sub2snp * assign12710_e7115);
        (assign12710_e7116,)
    } else {
        (var_xsub2_1,)
    }
};
        var_xsub2_1 = assign12710_e7118;
        var_xsub2_1_rv = 0.0;

        let (assign12720_e7130,) = {
    if (p.p23 != 0.0) {
        let assign12720_e7125: f64 = (var_lg).powf(p.p103);
        let assign12720_e7126: f64 = (p.p102 / assign12720_e7125);
        let assign12720_e7127: f64 = (1.0 + assign12720_e7126);
        let assign12720_e7128: f64 = (p.p72 * assign12720_e7127);
        (assign12720_e7128,)
    } else {
        (var_uc_subld1,)
    }
};
        var_uc_subld1 = assign12720_e7130;
        var_uc_subld1_rv = 0.0;

        let (assign12730_e7135, assign12730_e7135_d_n0, assign12730_e7135_d_n2, assign12730_e7135_d_n4, assign12730_e7135_d_n5, assign12730_e7135_d_n6, assign12730_e7135_d_n7, assign12730_e7135_d_n8, assign12730_e7135_d_n9, assign12730_e7135_d_n10, assign12730_e7135_d_n13,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vg2const, var_vg2const_dn0, var_vg2const_dn2, var_vg2const_dn4, var_vg2const_dn5, var_vg2const_dn6, var_vg2const_dn7, var_vg2const_dn8, var_vg2const_dn9, var_vg2const_dn10, var_vg2const_dn13,)
    }
};
        var_vg2const = assign12730_e7135;
        var_vg2const_dn0 = assign12730_e7135_d_n0;
        var_vg2const_dn2 = assign12730_e7135_d_n2;
        var_vg2const_dn4 = assign12730_e7135_d_n4;
        var_vg2const_dn5 = assign12730_e7135_d_n5;
        var_vg2const_dn6 = assign12730_e7135_d_n6;
        var_vg2const_dn7 = assign12730_e7135_d_n7;
        var_vg2const_dn8 = assign12730_e7135_d_n8;
        var_vg2const_dn9 = assign12730_e7135_d_n9;
        var_vg2const_dn10 = assign12730_e7135_d_n10;
        var_vg2const_dn13 = assign12730_e7135_d_n13;
        var_vg2const_rv = 0.0;

        let (assign12740_e7140,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xvbs,)
    }
};
        var_xvbs = assign12740_e7140;
        var_xvbs_rv = 0.0;

        let (assign12750_e7145,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xgate,)
    }
};
        var_xgate = assign12750_e7145;
        var_xgate_rv = 0.0;

        let (assign12760_e7150,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xsub1,)
    }
};
        var_xsub1 = assign12760_e7150;
        var_xsub1_rv = 0.0;

        let (assign12770_e7155,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xsub2,)
    }
};
        var_xsub2 = assign12770_e7155;
        var_xsub2_rv = 0.0;

        let (assign12780_e7160,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_uc_subld1,)
    }
};
        var_uc_subld1 = assign12780_e7160;
        var_uc_subld1_rv = 0.0;

        let (assign12790_e7165, assign12790_e7165_d_n0, assign12790_e7165_d_n2, assign12790_e7165_d_n4, assign12790_e7165_d_n5, assign12790_e7165_d_n6, assign12790_e7165_d_n7, assign12790_e7165_d_n8, assign12790_e7165_d_n9, assign12790_e7165_d_n10, assign12790_e7165_d_n13,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vg2const_1, var_vg2const_1_dn0, var_vg2const_1_dn2, var_vg2const_1_dn4, var_vg2const_1_dn5, var_vg2const_1_dn6, var_vg2const_1_dn7, var_vg2const_1_dn8, var_vg2const_1_dn9, var_vg2const_1_dn10, var_vg2const_1_dn13,)
    }
};
        var_vg2const_1 = assign12790_e7165;
        var_vg2const_1_dn0 = assign12790_e7165_d_n0;
        var_vg2const_1_dn2 = assign12790_e7165_d_n2;
        var_vg2const_1_dn4 = assign12790_e7165_d_n4;
        var_vg2const_1_dn5 = assign12790_e7165_d_n5;
        var_vg2const_1_dn6 = assign12790_e7165_d_n6;
        var_vg2const_1_dn7 = assign12790_e7165_d_n7;
        var_vg2const_1_dn8 = assign12790_e7165_d_n8;
        var_vg2const_1_dn9 = assign12790_e7165_d_n9;
        var_vg2const_1_dn10 = assign12790_e7165_d_n10;
        var_vg2const_1_dn13 = assign12790_e7165_d_n13;
        var_vg2const_1_rv = 0.0;

        let (assign12800_e7170,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xvbs_1,)
    }
};
        var_xvbs_1 = assign12800_e7170;
        var_xvbs_1_rv = 0.0;

        let (assign12810_e7175,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xgate_1,)
    }
};
        var_xgate_1 = assign12810_e7175;
        var_xgate_1_rv = 0.0;

        let (assign12820_e7180,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xsub1_1,)
    }
};
        var_xsub1_1 = assign12820_e7180;
        var_xsub1_1_rv = 0.0;

        let (assign12830_e7185,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xsub2_1,)
    }
};
        var_xsub2_1 = assign12830_e7185;
        var_xsub2_1_rv = 0.0;

        let (assign12840_e7199,) = {
    if (var_uc_ibpc1 != 0.0) {
        let assign12840_e7194: f64 = (var_lg).powf(p.p280);
        let assign12840_e7195: f64 = (p.p279 / assign12840_e7194);
        let assign12840_e7196: f64 = (1.0 + assign12840_e7195);
        let assign12840_e7197: f64 = (var_uc_ibpc1 * assign12840_e7196);
        (assign12840_e7197,)
    } else {
        (0.0,)
    }
};
        var_uc_ibpc1 = assign12840_e7199;
        var_uc_ibpc1_rv = 0.0;

        let assign12850_e7203: f64 = (3.141592653589793 / 2.0);
        let assign12850_e7204: f64 = (3.453133e-11 / assign12850_e7203);
        let assign12850_e7206: f64 = (assign12850_e7204 * var_weffcv_nf);
        let assign12850_e7210: f64 = (p.p225 / p.p95);
        let assign12850_e7211: f64 = (1.0 + assign12850_e7210);
        let assign12850_e7212: f64 = (assign12850_e7211).ln();
        let assign12850_e7213: f64 = (assign12850_e7206 * assign12850_e7212);
        var_cfrng = assign12850_e7213;
        var_cfrng_rv = 0.0;

        let (assign12860_e7227,) = {
    if (p.p134 != 0.0) {
        let assign12860_e7219: f64 = (1000000.0 * var_weffcv_nf);
        let assign12860_e7221: f64 = (assign12860_e7219 * p.p134);
        let assign12860_e7224: f64 = (var_lg).powf(p.p135);
        let assign12860_e7225: f64 = (assign12860_e7221 / assign12860_e7224);
        (assign12860_e7225,)
    } else {
        (0.0,)
    }
};
        var_cqyb0 = assign12860_e7227;
        var_cqyb0_rv = 0.0;

        let assign12870_e7231: f64 = (-p.p286);
        let assign12870_e7232: f64 = (var_lg).powf(assign12870_e7231);
        let assign12870_e7233: f64 = (p.p283 * assign12870_e7232);
        var_ptl0 = assign12870_e7233;
        var_ptl0_rv = 0.0;

        let assign12880_e7237: f64 = (-p.p291);
        let assign12880_e7238: f64 = (var_lg).powf(assign12880_e7237);
        let assign12880_e7239: f64 = (p.p290 * assign12880_e7238);
        var_pt40 = assign12880_e7239;
        var_pt40_rv = 0.0;

        let assign12890_e7243: f64 = (var_lg + var_uc_gdld);
        let assign12890_e7245: f64 = (-p.p288);
        let assign12890_e7246: f64 = (assign12890_e7243).powf(assign12890_e7245);
        let assign12890_e7247: f64 = (p.p287 * assign12890_e7246);
        var_gdl0 = assign12890_e7247;
        var_gdl0_rv = 0.0;

        let assign12900_e7251: f64 = (var_mfactor * var_weff_nf);
        let assign12900_e7252: f64 = (var_uc_rth0 / assign12900_e7251);
        let assign12900_e7257: f64 = (var_lg).powf(p.p318);
        let assign12900_e7258: f64 = (p.p317 / assign12900_e7257);
        let assign12900_e7259: f64 = (1.0 + assign12900_e7258);
        let assign12900_e7260: f64 = (assign12900_e7252 * assign12900_e7259);
        let assign12900_e7265: f64 = (var_wg).powf(p.p316);
        let assign12900_e7266: f64 = (p.p315 / assign12900_e7265);
        let assign12900_e7267: f64 = (1.0 + assign12900_e7266);
        let assign12900_e7268: f64 = (assign12900_e7260 * assign12900_e7267);
        var_rth = assign12900_e7268;
        var_rth_dn0 = 0.0;
        var_rth_dn2 = 0.0;
        var_rth_dn4 = 0.0;
        var_rth_dn5 = 0.0;
        var_rth_dn6 = 0.0;
        var_rth_dn7 = 0.0;
        var_rth_dn8 = 0.0;
        var_rth_dn9 = 0.0;
        var_rth_dn10 = 0.0;
        var_rth_dn13 = 0.0;
        var_rth_rv = 0.0;

        let assign12920_e7278: f64 = (p.p7).powf(p.p327);
        let assign12920_e7279: f64 = (1.0 / assign12920_e7278);
        let assign12920_e7280: f64 = (var_rth * assign12920_e7279);
        var_rth = assign12920_e7280;
        var_rth_dn0 = (var_rth_dn0 * assign12920_e7279);
        var_rth_dn2 = (var_rth_dn2 * assign12920_e7279);
        var_rth_dn4 = (var_rth_dn4 * assign12920_e7279);
        var_rth_dn5 = (var_rth_dn5 * assign12920_e7279);
        var_rth_dn6 = (var_rth_dn6 * assign12920_e7279);
        var_rth_dn7 = (var_rth_dn7 * assign12920_e7279);
        var_rth_dn8 = (var_rth_dn8 * assign12920_e7279);
        var_rth_dn9 = (var_rth_dn9 * assign12920_e7279);
        var_rth_dn10 = (var_rth_dn10 * assign12920_e7279);
        var_rth_dn13 = (var_rth_dn13 * assign12920_e7279);
        var_rth_rv = 0.0;

        let assign12930_e7284: f64 = (p.p7).powf(p.p327);
        let assign12930_e7285: f64 = (1.0 / assign12930_e7284);
        let assign12930_e7288: f64 = (var_mfactor * var_weff_nf);
        let assign12930_e7289: f64 = (assign12930_e7285 / assign12930_e7288);
        let assign12930_e7294: f64 = (var_lg).powf(p.p318);
        let assign12930_e7295: f64 = (p.p317 / assign12930_e7294);
        let assign12930_e7296: f64 = (1.0 + assign12930_e7295);
        let assign12930_e7297: f64 = (assign12930_e7289 * assign12930_e7296);
        let assign12930_e7302: f64 = (var_wg).powf(p.p316);
        let assign12930_e7303: f64 = (p.p315 / assign12930_e7302);
        let assign12930_e7304: f64 = (1.0 + assign12930_e7303);
        let assign12930_e7305: f64 = (assign12930_e7297 * assign12930_e7304);
        var_rthtemp0 = assign12930_e7305;
        var_rthtemp0_rv = 0.0;

        let assign12940_e7312: f64 = if ((p.p53 == 0.0) || (var_uc_rth0 == 0.0)) { 1.0 } else { 0.0 };
        var_guard289 = assign12940_e7312;
        var_guard289_rv = 0.0;

        *var_cfrng_slot = var_cfrng;
        *var_cfrng_rv_slot = var_cfrng_rv;
        *var_cqyb0_slot = var_cqyb0;
        *var_cqyb0_rv_slot = var_cqyb0_rv;
        *var_ddlte_slot = var_ddlte;
        *var_ddlte_dn0_slot = var_ddlte_dn0;
        *var_ddlte_dn10_slot = var_ddlte_dn10;
        *var_ddlte_dn13_slot = var_ddlte_dn13;
        *var_ddlte_dn2_slot = var_ddlte_dn2;
        *var_ddlte_dn4_slot = var_ddlte_dn4;
        *var_ddlte_dn5_slot = var_ddlte_dn5;
        *var_ddlte_dn6_slot = var_ddlte_dn6;
        *var_ddlte_dn7_slot = var_ddlte_dn7;
        *var_ddlte_dn8_slot = var_ddlte_dn8;
        *var_ddlte_dn9_slot = var_ddlte_dn9;
        *var_ddlte_rv_slot = var_ddlte_rv;
        *var_gdl0_slot = var_gdl0;
        *var_gdl0_rv_slot = var_gdl0_rv;
        *var_guard288_slot = var_guard288;
        *var_guard288_rv_slot = var_guard288_rv;
        *var_guard289_slot = var_guard289;
        *var_guard289_rv_slot = var_guard289_rv;
        *var_pt40_slot = var_pt40;
        *var_pt40_rv_slot = var_pt40_rv;
        *var_ptl0_slot = var_ptl0;
        *var_ptl0_rv_slot = var_ptl0_rv;
        *var_rth_slot = var_rth;
        *var_rth_dn0_slot = var_rth_dn0;
        *var_rth_dn10_slot = var_rth_dn10;
        *var_rth_dn13_slot = var_rth_dn13;
        *var_rth_dn2_slot = var_rth_dn2;
        *var_rth_dn4_slot = var_rth_dn4;
        *var_rth_dn5_slot = var_rth_dn5;
        *var_rth_dn6_slot = var_rth_dn6;
        *var_rth_dn7_slot = var_rth_dn7;
        *var_rth_dn8_slot = var_rth_dn8;
        *var_rth_dn9_slot = var_rth_dn9;
        *var_rth_rv_slot = var_rth_rv;
        *var_rthtemp0_slot = var_rthtemp0;
        *var_rthtemp0_rv_slot = var_rthtemp0_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_uc_ibpc1_slot = var_uc_ibpc1;
        *var_uc_ibpc1_rv_slot = var_uc_ibpc1_rv;
        *var_uc_subld1_slot = var_uc_subld1;
        *var_uc_subld1_rv_slot = var_uc_subld1_rv;
        *var_vg2const_slot = var_vg2const;
        *var_vg2const_1_slot = var_vg2const_1;
        *var_vg2const_1_dn0_slot = var_vg2const_1_dn0;
        *var_vg2const_1_dn10_slot = var_vg2const_1_dn10;
        *var_vg2const_1_dn13_slot = var_vg2const_1_dn13;
        *var_vg2const_1_dn2_slot = var_vg2const_1_dn2;
        *var_vg2const_1_dn4_slot = var_vg2const_1_dn4;
        *var_vg2const_1_dn5_slot = var_vg2const_1_dn5;
        *var_vg2const_1_dn6_slot = var_vg2const_1_dn6;
        *var_vg2const_1_dn7_slot = var_vg2const_1_dn7;
        *var_vg2const_1_dn8_slot = var_vg2const_1_dn8;
        *var_vg2const_1_dn9_slot = var_vg2const_1_dn9;
        *var_vg2const_1_rv_slot = var_vg2const_1_rv;
        *var_vg2const_dn0_slot = var_vg2const_dn0;
        *var_vg2const_dn10_slot = var_vg2const_dn10;
        *var_vg2const_dn13_slot = var_vg2const_dn13;
        *var_vg2const_dn2_slot = var_vg2const_dn2;
        *var_vg2const_dn4_slot = var_vg2const_dn4;
        *var_vg2const_dn5_slot = var_vg2const_dn5;
        *var_vg2const_dn6_slot = var_vg2const_dn6;
        *var_vg2const_dn7_slot = var_vg2const_dn7;
        *var_vg2const_dn8_slot = var_vg2const_dn8;
        *var_vg2const_dn9_slot = var_vg2const_dn9;
        *var_vg2const_rv_slot = var_vg2const_rv;
        *var_xgate_slot = var_xgate;
        *var_xgate_1_slot = var_xgate_1;
        *var_xgate_1_rv_slot = var_xgate_1_rv;
        *var_xgate_rv_slot = var_xgate_rv;
        *var_xsub1_slot = var_xsub1;
        *var_xsub1_1_slot = var_xsub1_1;
        *var_xsub1_1_rv_slot = var_xsub1_1_rv;
        *var_xsub1_rv_slot = var_xsub1_rv;
        *var_xsub2_slot = var_xsub2;
        *var_xsub2_1_slot = var_xsub2_1;
        *var_xsub2_1_rv_slot = var_xsub2_1_rv;
        *var_xsub2_rv_slot = var_xsub2_rv;
        *var_xvbs_slot = var_xvbs;
        *var_xvbs_1_slot = var_xvbs_1;
        *var_xvbs_1_rv_slot = var_xvbs_1_rv;
        *var_xvbs_rv_slot = var_xvbs_rv;
    }
}
