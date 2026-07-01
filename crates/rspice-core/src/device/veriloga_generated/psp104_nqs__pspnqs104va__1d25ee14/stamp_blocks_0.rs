#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        p: &Parameters,
        var_alphaav_slot: &mut f64,
        var_berfc_slot: &mut f64,
        var_cerfc_slot: &mut f64,
        var_chnl_type_slot: &mut f64,
        var_cjorbotd_i_slot: &mut f64,
        var_cjorgat2nd_slot: &mut f64,
        var_deltaphigr_slot: &mut f64,
        var_epssi_slot: &mut f64,
        var_fstopbot_slot: &mut f64,
        var_fstopgat_slot: &mut f64,
        var_fstopsti_slot: &mut f64,
        var_guard1_slot: &mut f64,
        var_guard10_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_kbol_over_qele_slot: &mut f64,
        var_one_minus_pbot_slot: &mut f64,
        var_one_minus_pgat_slot: &mut f64,
        var_one_minus_pgat2nd_slot: &mut f64,
        var_one_minus_psti_slot: &mut f64,
        var_one_over_one_minus_pbot_slot: &mut f64,
        var_one_over_one_minus_pgat_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_slot: &mut f64,
        var_one_over_one_minus_psti_slot: &mut f64,
        var_perfc_slot: &mut f64,
        var_pgat2nd_slot: &mut f64,
        var_phiggat2nd_slot: &mut f64,
        var_phigrbot_slot: &mut f64,
        var_phigrgat_slot: &mut f64,
        var_phigrgat2nd_slot: &mut f64,
        var_phigrsti_slot: &mut f64,
        var_phitr_slot: &mut f64,
        var_phitrinv_slot: &mut f64,
        var_slopebot_slot: &mut f64,
        var_slopegat_slot: &mut f64,
        var_slopegat_db0_slot: &mut f64,
        var_slopegat_db1_slot: &mut f64,
        var_slopegat_db10_slot: &mut f64,
        var_slopegat_db11_slot: &mut f64,
        var_slopegat_db12_slot: &mut f64,
        var_slopegat_db13_slot: &mut f64,
        var_slopegat_db14_slot: &mut f64,
        var_slopegat_db15_slot: &mut f64,
        var_slopegat_db16_slot: &mut f64,
        var_slopegat_db17_slot: &mut f64,
        var_slopegat_db18_slot: &mut f64,
        var_slopegat_db19_slot: &mut f64,
        var_slopegat_db2_slot: &mut f64,
        var_slopegat_db20_slot: &mut f64,
        var_slopegat_db21_slot: &mut f64,
        var_slopegat_db22_slot: &mut f64,
        var_slopegat_db23_slot: &mut f64,
        var_slopegat_db24_slot: &mut f64,
        var_slopegat_db3_slot: &mut f64,
        var_slopegat_db4_slot: &mut f64,
        var_slopegat_db5_slot: &mut f64,
        var_slopegat_db6_slot: &mut f64,
        var_slopegat_db7_slot: &mut f64,
        var_slopegat_db8_slot: &mut f64,
        var_slopegat_db9_slot: &mut f64,
        var_slopegat_dn0_slot: &mut f64,
        var_slopegat_dn1_slot: &mut f64,
        var_slopegat_dn10_slot: &mut f64,
        var_slopegat_dn11_slot: &mut f64,
        var_slopegat_dn12_slot: &mut f64,
        var_slopegat_dn13_slot: &mut f64,
        var_slopegat_dn14_slot: &mut f64,
        var_slopegat_dn15_slot: &mut f64,
        var_slopegat_dn16_slot: &mut f64,
        var_slopegat_dn17_slot: &mut f64,
        var_slopegat_dn18_slot: &mut f64,
        var_slopegat_dn19_slot: &mut f64,
        var_slopegat_dn2_slot: &mut f64,
        var_slopegat_dn20_slot: &mut f64,
        var_slopegat_dn3_slot: &mut f64,
        var_slopegat_dn4_slot: &mut f64,
        var_slopegat_dn5_slot: &mut f64,
        var_slopegat_dn6_slot: &mut f64,
        var_slopegat_dn7_slot: &mut f64,
        var_slopegat_dn8_slot: &mut f64,
        var_slopegat_dn9_slot: &mut f64,
        var_slopesti_slot: &mut f64,
        var_swgat2nd_slot: &mut f64,
        var_swjunexp_i_slot: &mut f64,
        var_tkr_slot: &mut f64,
        var_tkr_1_slot: &mut f64,
        var_vbirbotinv_slot: &mut f64,
        var_vbirgat2nd_slot: &mut f64,
        var_vbirgatinv_slot: &mut f64,
        var_vbirstiinv_slot: &mut f64,
        var_vbrinvbot_slot: &mut f64,
        var_vbrinvgat_slot: &mut f64,
        var_vbrinvgat_db0_slot: &mut f64,
        var_vbrinvgat_db1_slot: &mut f64,
        var_vbrinvgat_db10_slot: &mut f64,
        var_vbrinvgat_db11_slot: &mut f64,
        var_vbrinvgat_db12_slot: &mut f64,
        var_vbrinvgat_db13_slot: &mut f64,
        var_vbrinvgat_db14_slot: &mut f64,
        var_vbrinvgat_db15_slot: &mut f64,
        var_vbrinvgat_db16_slot: &mut f64,
        var_vbrinvgat_db17_slot: &mut f64,
        var_vbrinvgat_db18_slot: &mut f64,
        var_vbrinvgat_db19_slot: &mut f64,
        var_vbrinvgat_db2_slot: &mut f64,
        var_vbrinvgat_db20_slot: &mut f64,
        var_vbrinvgat_db21_slot: &mut f64,
        var_vbrinvgat_db22_slot: &mut f64,
        var_vbrinvgat_db23_slot: &mut f64,
        var_vbrinvgat_db24_slot: &mut f64,
        var_vbrinvgat_db3_slot: &mut f64,
        var_vbrinvgat_db4_slot: &mut f64,
        var_vbrinvgat_db5_slot: &mut f64,
        var_vbrinvgat_db6_slot: &mut f64,
        var_vbrinvgat_db7_slot: &mut f64,
        var_vbrinvgat_db8_slot: &mut f64,
        var_vbrinvgat_db9_slot: &mut f64,
        var_vbrinvgat_dn0_slot: &mut f64,
        var_vbrinvgat_dn1_slot: &mut f64,
        var_vbrinvgat_dn10_slot: &mut f64,
        var_vbrinvgat_dn11_slot: &mut f64,
        var_vbrinvgat_dn12_slot: &mut f64,
        var_vbrinvgat_dn13_slot: &mut f64,
        var_vbrinvgat_dn14_slot: &mut f64,
        var_vbrinvgat_dn15_slot: &mut f64,
        var_vbrinvgat_dn16_slot: &mut f64,
        var_vbrinvgat_dn17_slot: &mut f64,
        var_vbrinvgat_dn18_slot: &mut f64,
        var_vbrinvgat_dn19_slot: &mut f64,
        var_vbrinvgat_dn2_slot: &mut f64,
        var_vbrinvgat_dn20_slot: &mut f64,
        var_vbrinvgat_dn3_slot: &mut f64,
        var_vbrinvgat_dn4_slot: &mut f64,
        var_vbrinvgat_dn5_slot: &mut f64,
        var_vbrinvgat_dn6_slot: &mut f64,
        var_vbrinvgat_dn7_slot: &mut f64,
        var_vbrinvgat_dn8_slot: &mut f64,
        var_vbrinvgat_dn9_slot: &mut f64,
        var_vbrinvsti_slot: &mut f64,
        var_wdepnulrbot_slot: &mut f64,
        var_wdepnulrgat_slot: &mut f64,
        var_wdepnulrinvbot_slot: &mut f64,
        var_wdepnulrinvgat_slot: &mut f64,
        var_wdepnulrinvsti_slot: &mut f64,
        var_wdepnulrsti_slot: &mut f64,
    ) {
        let mut var_alphaav: f64 = *var_alphaav_slot;
        let mut var_berfc: f64 = *var_berfc_slot;
        let mut var_cerfc: f64 = *var_cerfc_slot;
        let mut var_chnl_type: f64 = *var_chnl_type_slot;
        let mut var_cjorbotd_i: f64 = *var_cjorbotd_i_slot;
        let mut var_cjorgat2nd: f64 = *var_cjorgat2nd_slot;
        let mut var_deltaphigr: f64 = *var_deltaphigr_slot;
        let mut var_epssi: f64 = *var_epssi_slot;
        let mut var_fstopbot: f64 = *var_fstopbot_slot;
        let mut var_fstopgat: f64 = *var_fstopgat_slot;
        let mut var_fstopsti: f64 = *var_fstopsti_slot;
        let mut var_guard1: f64 = *var_guard1_slot;
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_kbol_over_qele: f64 = *var_kbol_over_qele_slot;
        let mut var_one_minus_pbot: f64 = *var_one_minus_pbot_slot;
        let mut var_one_minus_pgat: f64 = *var_one_minus_pgat_slot;
        let mut var_one_minus_pgat2nd: f64 = *var_one_minus_pgat2nd_slot;
        let mut var_one_minus_psti: f64 = *var_one_minus_psti_slot;
        let mut var_one_over_one_minus_pbot: f64 = *var_one_over_one_minus_pbot_slot;
        let mut var_one_over_one_minus_pgat: f64 = *var_one_over_one_minus_pgat_slot;
        let mut var_one_over_one_minus_pgat2nd: f64 = *var_one_over_one_minus_pgat2nd_slot;
        let mut var_one_over_one_minus_psti: f64 = *var_one_over_one_minus_psti_slot;
        let mut var_perfc: f64 = *var_perfc_slot;
        let mut var_pgat2nd: f64 = *var_pgat2nd_slot;
        let mut var_phiggat2nd: f64 = *var_phiggat2nd_slot;
        let mut var_phigrbot: f64 = *var_phigrbot_slot;
        let mut var_phigrgat: f64 = *var_phigrgat_slot;
        let mut var_phigrgat2nd: f64 = *var_phigrgat2nd_slot;
        let mut var_phigrsti: f64 = *var_phigrsti_slot;
        let mut var_phitr: f64 = *var_phitr_slot;
        let mut var_phitrinv: f64 = *var_phitrinv_slot;
        let mut var_slopebot: f64 = *var_slopebot_slot;
        let mut var_slopegat: f64 = *var_slopegat_slot;
        let mut var_slopegat_db0: f64 = *var_slopegat_db0_slot;
        let mut var_slopegat_db1: f64 = *var_slopegat_db1_slot;
        let mut var_slopegat_db10: f64 = *var_slopegat_db10_slot;
        let mut var_slopegat_db11: f64 = *var_slopegat_db11_slot;
        let mut var_slopegat_db12: f64 = *var_slopegat_db12_slot;
        let mut var_slopegat_db13: f64 = *var_slopegat_db13_slot;
        let mut var_slopegat_db14: f64 = *var_slopegat_db14_slot;
        let mut var_slopegat_db15: f64 = *var_slopegat_db15_slot;
        let mut var_slopegat_db16: f64 = *var_slopegat_db16_slot;
        let mut var_slopegat_db17: f64 = *var_slopegat_db17_slot;
        let mut var_slopegat_db18: f64 = *var_slopegat_db18_slot;
        let mut var_slopegat_db19: f64 = *var_slopegat_db19_slot;
        let mut var_slopegat_db2: f64 = *var_slopegat_db2_slot;
        let mut var_slopegat_db20: f64 = *var_slopegat_db20_slot;
        let mut var_slopegat_db21: f64 = *var_slopegat_db21_slot;
        let mut var_slopegat_db22: f64 = *var_slopegat_db22_slot;
        let mut var_slopegat_db23: f64 = *var_slopegat_db23_slot;
        let mut var_slopegat_db24: f64 = *var_slopegat_db24_slot;
        let mut var_slopegat_db3: f64 = *var_slopegat_db3_slot;
        let mut var_slopegat_db4: f64 = *var_slopegat_db4_slot;
        let mut var_slopegat_db5: f64 = *var_slopegat_db5_slot;
        let mut var_slopegat_db6: f64 = *var_slopegat_db6_slot;
        let mut var_slopegat_db7: f64 = *var_slopegat_db7_slot;
        let mut var_slopegat_db8: f64 = *var_slopegat_db8_slot;
        let mut var_slopegat_db9: f64 = *var_slopegat_db9_slot;
        let mut var_slopegat_dn0: f64 = *var_slopegat_dn0_slot;
        let mut var_slopegat_dn1: f64 = *var_slopegat_dn1_slot;
        let mut var_slopegat_dn10: f64 = *var_slopegat_dn10_slot;
        let mut var_slopegat_dn11: f64 = *var_slopegat_dn11_slot;
        let mut var_slopegat_dn12: f64 = *var_slopegat_dn12_slot;
        let mut var_slopegat_dn13: f64 = *var_slopegat_dn13_slot;
        let mut var_slopegat_dn14: f64 = *var_slopegat_dn14_slot;
        let mut var_slopegat_dn15: f64 = *var_slopegat_dn15_slot;
        let mut var_slopegat_dn16: f64 = *var_slopegat_dn16_slot;
        let mut var_slopegat_dn17: f64 = *var_slopegat_dn17_slot;
        let mut var_slopegat_dn18: f64 = *var_slopegat_dn18_slot;
        let mut var_slopegat_dn19: f64 = *var_slopegat_dn19_slot;
        let mut var_slopegat_dn2: f64 = *var_slopegat_dn2_slot;
        let mut var_slopegat_dn20: f64 = *var_slopegat_dn20_slot;
        let mut var_slopegat_dn3: f64 = *var_slopegat_dn3_slot;
        let mut var_slopegat_dn4: f64 = *var_slopegat_dn4_slot;
        let mut var_slopegat_dn5: f64 = *var_slopegat_dn5_slot;
        let mut var_slopegat_dn6: f64 = *var_slopegat_dn6_slot;
        let mut var_slopegat_dn7: f64 = *var_slopegat_dn7_slot;
        let mut var_slopegat_dn8: f64 = *var_slopegat_dn8_slot;
        let mut var_slopegat_dn9: f64 = *var_slopegat_dn9_slot;
        let mut var_slopesti: f64 = *var_slopesti_slot;
        let mut var_swgat2nd: f64 = *var_swgat2nd_slot;
        let mut var_swjunexp_i: f64 = *var_swjunexp_i_slot;
        let mut var_tkr: f64 = *var_tkr_slot;
        let mut var_tkr_1: f64 = *var_tkr_1_slot;
        let mut var_vbirbotinv: f64 = *var_vbirbotinv_slot;
        let mut var_vbirgat2nd: f64 = *var_vbirgat2nd_slot;
        let mut var_vbirgatinv: f64 = *var_vbirgatinv_slot;
        let mut var_vbirstiinv: f64 = *var_vbirstiinv_slot;
        let mut var_vbrinvbot: f64 = *var_vbrinvbot_slot;
        let mut var_vbrinvgat: f64 = *var_vbrinvgat_slot;
        let mut var_vbrinvgat_db0: f64 = *var_vbrinvgat_db0_slot;
        let mut var_vbrinvgat_db1: f64 = *var_vbrinvgat_db1_slot;
        let mut var_vbrinvgat_db10: f64 = *var_vbrinvgat_db10_slot;
        let mut var_vbrinvgat_db11: f64 = *var_vbrinvgat_db11_slot;
        let mut var_vbrinvgat_db12: f64 = *var_vbrinvgat_db12_slot;
        let mut var_vbrinvgat_db13: f64 = *var_vbrinvgat_db13_slot;
        let mut var_vbrinvgat_db14: f64 = *var_vbrinvgat_db14_slot;
        let mut var_vbrinvgat_db15: f64 = *var_vbrinvgat_db15_slot;
        let mut var_vbrinvgat_db16: f64 = *var_vbrinvgat_db16_slot;
        let mut var_vbrinvgat_db17: f64 = *var_vbrinvgat_db17_slot;
        let mut var_vbrinvgat_db18: f64 = *var_vbrinvgat_db18_slot;
        let mut var_vbrinvgat_db19: f64 = *var_vbrinvgat_db19_slot;
        let mut var_vbrinvgat_db2: f64 = *var_vbrinvgat_db2_slot;
        let mut var_vbrinvgat_db20: f64 = *var_vbrinvgat_db20_slot;
        let mut var_vbrinvgat_db21: f64 = *var_vbrinvgat_db21_slot;
        let mut var_vbrinvgat_db22: f64 = *var_vbrinvgat_db22_slot;
        let mut var_vbrinvgat_db23: f64 = *var_vbrinvgat_db23_slot;
        let mut var_vbrinvgat_db24: f64 = *var_vbrinvgat_db24_slot;
        let mut var_vbrinvgat_db3: f64 = *var_vbrinvgat_db3_slot;
        let mut var_vbrinvgat_db4: f64 = *var_vbrinvgat_db4_slot;
        let mut var_vbrinvgat_db5: f64 = *var_vbrinvgat_db5_slot;
        let mut var_vbrinvgat_db6: f64 = *var_vbrinvgat_db6_slot;
        let mut var_vbrinvgat_db7: f64 = *var_vbrinvgat_db7_slot;
        let mut var_vbrinvgat_db8: f64 = *var_vbrinvgat_db8_slot;
        let mut var_vbrinvgat_db9: f64 = *var_vbrinvgat_db9_slot;
        let mut var_vbrinvgat_dn0: f64 = *var_vbrinvgat_dn0_slot;
        let mut var_vbrinvgat_dn1: f64 = *var_vbrinvgat_dn1_slot;
        let mut var_vbrinvgat_dn10: f64 = *var_vbrinvgat_dn10_slot;
        let mut var_vbrinvgat_dn11: f64 = *var_vbrinvgat_dn11_slot;
        let mut var_vbrinvgat_dn12: f64 = *var_vbrinvgat_dn12_slot;
        let mut var_vbrinvgat_dn13: f64 = *var_vbrinvgat_dn13_slot;
        let mut var_vbrinvgat_dn14: f64 = *var_vbrinvgat_dn14_slot;
        let mut var_vbrinvgat_dn15: f64 = *var_vbrinvgat_dn15_slot;
        let mut var_vbrinvgat_dn16: f64 = *var_vbrinvgat_dn16_slot;
        let mut var_vbrinvgat_dn17: f64 = *var_vbrinvgat_dn17_slot;
        let mut var_vbrinvgat_dn18: f64 = *var_vbrinvgat_dn18_slot;
        let mut var_vbrinvgat_dn19: f64 = *var_vbrinvgat_dn19_slot;
        let mut var_vbrinvgat_dn2: f64 = *var_vbrinvgat_dn2_slot;
        let mut var_vbrinvgat_dn20: f64 = *var_vbrinvgat_dn20_slot;
        let mut var_vbrinvgat_dn3: f64 = *var_vbrinvgat_dn3_slot;
        let mut var_vbrinvgat_dn4: f64 = *var_vbrinvgat_dn4_slot;
        let mut var_vbrinvgat_dn5: f64 = *var_vbrinvgat_dn5_slot;
        let mut var_vbrinvgat_dn6: f64 = *var_vbrinvgat_dn6_slot;
        let mut var_vbrinvgat_dn7: f64 = *var_vbrinvgat_dn7_slot;
        let mut var_vbrinvgat_dn8: f64 = *var_vbrinvgat_dn8_slot;
        let mut var_vbrinvgat_dn9: f64 = *var_vbrinvgat_dn9_slot;
        let mut var_vbrinvsti: f64 = *var_vbrinvsti_slot;
        let mut var_wdepnulrbot: f64 = *var_wdepnulrbot_slot;
        let mut var_wdepnulrgat: f64 = *var_wdepnulrgat_slot;
        let mut var_wdepnulrinvbot: f64 = *var_wdepnulrinvbot_slot;
        let mut var_wdepnulrinvgat: f64 = *var_wdepnulrinvgat_slot;
        let mut var_wdepnulrinvsti: f64 = *var_wdepnulrinvsti_slot;
        let mut var_wdepnulrsti: f64 = *var_wdepnulrsti_slot;

        let assign00_e1569: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        var_guard1 = assign00_e1569;

        let (assign10_e1574,) = {
    if (var_guard1 != 0.0) {
        let assign10_e1572: f64 = 1.0;
        (assign10_e1572,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign10_e1574;

        let (assign20_e1580,) = {
    if (var_guard1 == 0.0) {
        let assign20_e1578: f64 = (-1.0);
        (assign20_e1578,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign20_e1580;

        let assign30_e1583: f64 = (8.8541878176e-12 * 11.8);
        var_epssi = assign30_e1583;

        s.b[991] = (p.p51 < 0.5);
        s.store_scalar(991, if s.b[991] { 1.0 } else { 0.0 });

        let (assign50_e1590,) = {
    if s.b[991] {
        (0.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, assign50_e1590);

        s.b[992] = (p.p51 < 1.5);
        s.store_scalar(992, if s.b[992] { 1.0 } else { 0.0 });

        let (assign70_e1600,) = {
    if ((!s.b[991]) && s.b[992]) {
        (1.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, assign70_e1600);

        s.b[993] = (p.p51 < 2.5);
        s.store_scalar(993, if s.b[993] { 1.0 } else { 0.0 });

        let (assign90_e1613,) = {
    if (((!s.b[991]) && (!s.b[992])) && s.b[993]) {
        (2.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, assign90_e1613);

        s.b[994] = (p.p51 < 4.0);
        s.store_scalar(994, if s.b[994] { 1.0 } else { 0.0 });

        let (assign110_e1629,) = {
    if ((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && s.b[994]) {
        (3.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, assign110_e1629);

        s.b[995] = (p.p51 < 7.0);
        s.store_scalar(995, if s.b[995] { 1.0 } else { 0.0 });

        let (assign130_e1648,) = {
    if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && s.b[995]) {
        (5.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, assign130_e1648);

        let (assign140_e1665,) = {
    if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && (!s.b[995])) {
        (9.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, assign140_e1665);

        s.store_scalar(3, 10.0);

        s.store_scalar(4, (1.0 / s.v[3]));

        let assign180_e1673: f64 = (273.15 + p.p38);
        var_tkr = assign180_e1673;

        var_swjunexp_i = 0.0;

        let assign200_e1677: f64 = if p.p927 > 0.5 { 1.0 } else { 0.0 };
        var_guard7 = assign200_e1677;

        let (assign210_e1681,) = {
    if (var_guard7 != 0.0) {
        (1.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign210_e1681;

        let (assign220_e1686,) = {
    if (var_guard7 == 0.0) {
        (0.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign220_e1686;

        let assign230_e1689: f64 = (273.15 + p.p823);
        var_tkr_1 = assign230_e1689;

        let assign240_e1692: f64 = (1.3806505e-23 / 1.6021918e-19);
        var_kbol_over_qele = assign240_e1692;

        let assign250_e1695: f64 = (var_kbol_over_qele * var_tkr_1);
        var_phitr = assign250_e1695;

        let assign260_e1698: f64 = (1.0 / var_phitr);
        var_phitrinv = assign260_e1698;

        let assign270_e1701: f64 = (0.000702 * var_tkr_1);
        let assign270_e1703: f64 = (assign270_e1701 * var_tkr_1);
        let assign270_e1704: f64 = (-assign270_e1703);
        let assign270_e1707: f64 = (1108.0 + var_tkr_1);
        let assign270_e1708: f64 = (assign270_e1704 / assign270_e1707);
        var_deltaphigr = assign270_e1708;

        let assign280_e1711: f64 = (p.p834 + var_deltaphigr);
        var_phigrbot = assign280_e1711;

        let assign290_e1714: f64 = (p.p835 + var_deltaphigr);
        var_phigrsti = assign290_e1714;

        let assign300_e1717: f64 = (p.p836 + var_deltaphigr);
        var_phigrgat = assign300_e1717;

        let assign310_e1720: f64 = (1.0 - p.p831);
        var_one_minus_pbot = assign310_e1720;

        let assign320_e1723: f64 = (1.0 - p.p832);
        var_one_minus_psti = assign320_e1723;

        let assign330_e1726: f64 = (1.0 - p.p833);
        var_one_minus_pgat = assign330_e1726;

        let assign340_e1729: f64 = (1.0 / var_one_minus_pbot);
        var_one_over_one_minus_pbot = assign340_e1729;

        let assign350_e1732: f64 = (1.0 / var_one_minus_psti);
        var_one_over_one_minus_psti = assign350_e1732;

        let assign360_e1735: f64 = (1.0 / var_one_minus_pgat);
        var_one_over_one_minus_pgat = assign360_e1735;

        let assign370_e1738: f64 = (var_epssi / p.p825);
        var_wdepnulrbot = assign370_e1738;

        let assign380_e1741: f64 = (p.p843 * var_epssi);
        let assign380_e1743: f64 = (assign380_e1741 / p.p826);
        var_wdepnulrsti = assign380_e1743;

        let assign390_e1746: f64 = (p.p844 * var_epssi);
        let assign390_e1748: f64 = (assign390_e1746 / p.p827);
        var_wdepnulrgat = assign390_e1748;

        let assign400_e1751: f64 = (1.0 / var_wdepnulrbot);
        var_wdepnulrinvbot = assign400_e1751;

        let assign410_e1754: f64 = (1.0 / var_wdepnulrsti);
        var_wdepnulrinvsti = assign410_e1754;

        let assign420_e1757: f64 = (1.0 / var_wdepnulrgat);
        var_wdepnulrinvgat = assign420_e1757;

        let assign430_e1760: f64 = (1.0 / p.p828);
        var_vbirbotinv = assign430_e1760;

        let assign440_e1763: f64 = (1.0 / p.p829);
        var_vbirstiinv = assign440_e1763;

        let assign450_e1766: f64 = (1.0 / p.p830);
        var_vbirgatinv = assign450_e1766;

        let assign460_e1769: f64 = (1.772453850905516 * 0.29214664);
        var_perfc = assign460_e1769;

        let assign470_e1771: f64 = (-5.0);
        let assign470_e1773: f64 = (assign470_e1771 * 0.29214664);
        let assign470_e1775: f64 = (assign470_e1773 + 6.0);
        let assign470_e1778: f64 = (-2.0);
        let assign470_e1779: f64 = (var_perfc).powf(assign470_e1778);
        let assign470_e1780: f64 = (assign470_e1775 - assign470_e1779);
        let assign470_e1782: f64 = (assign470_e1780 / 3.0);
        var_berfc = assign470_e1782;

        let assign480_e1785: f64 = (1.0 - 0.29214664);
        let assign480_e1787: f64 = (assign480_e1785 - var_berfc);
        var_cerfc = assign480_e1787;

        let assign490_e1791: f64 = (1.0 / p.p824);
        let assign490_e1792: f64 = (1.0 - assign490_e1791);
        var_alphaav = assign490_e1792;

        let assign500_e1797: f64 = (var_alphaav).powf(p.p863);
        let assign500_e1798: f64 = (1.0 - assign500_e1797);
        let assign500_e1799: f64 = (1.0 / assign500_e1798);
        var_fstopbot = assign500_e1799;

        let assign510_e1804: f64 = (var_alphaav).powf(p.p864);
        let assign510_e1805: f64 = (1.0 - assign510_e1804);
        let assign510_e1806: f64 = (1.0 / assign510_e1805);
        var_fstopsti = assign510_e1806;

        let assign520_e1811: f64 = (var_alphaav).powf(p.p865);
        let assign520_e1812: f64 = (1.0 - assign520_e1811);
        let assign520_e1813: f64 = (1.0 / assign520_e1812);
        var_fstopgat = assign520_e1813;

        let assign530_e1816: f64 = (1.0 / p.p860);
        var_vbrinvbot = assign530_e1816;

        let assign540_e1819: f64 = (1.0 / p.p861);
        var_vbrinvsti = assign540_e1819;

        let assign550_e1822: f64 = (1.0 / p.p862);
        var_vbrinvgat = assign550_e1822;
        var_vbrinvgat_dn0 = 0.0;
        var_vbrinvgat_dn1 = 0.0;
        var_vbrinvgat_dn2 = 0.0;
        var_vbrinvgat_dn3 = 0.0;
        var_vbrinvgat_dn4 = 0.0;
        var_vbrinvgat_dn5 = 0.0;
        var_vbrinvgat_dn6 = 0.0;
        var_vbrinvgat_dn7 = 0.0;
        var_vbrinvgat_dn8 = 0.0;
        var_vbrinvgat_dn9 = 0.0;
        var_vbrinvgat_dn10 = 0.0;
        var_vbrinvgat_dn11 = 0.0;
        var_vbrinvgat_dn12 = 0.0;
        var_vbrinvgat_dn13 = 0.0;
        var_vbrinvgat_dn14 = 0.0;
        var_vbrinvgat_dn15 = 0.0;
        var_vbrinvgat_dn16 = 0.0;
        var_vbrinvgat_dn17 = 0.0;
        var_vbrinvgat_dn18 = 0.0;
        var_vbrinvgat_dn19 = 0.0;
        var_vbrinvgat_dn20 = 0.0;
        var_vbrinvgat_db0 = 0.0;
        var_vbrinvgat_db1 = 0.0;
        var_vbrinvgat_db2 = 0.0;
        var_vbrinvgat_db3 = 0.0;
        var_vbrinvgat_db4 = 0.0;
        var_vbrinvgat_db5 = 0.0;
        var_vbrinvgat_db6 = 0.0;
        var_vbrinvgat_db7 = 0.0;
        var_vbrinvgat_db8 = 0.0;
        var_vbrinvgat_db9 = 0.0;
        var_vbrinvgat_db10 = 0.0;
        var_vbrinvgat_db11 = 0.0;
        var_vbrinvgat_db12 = 0.0;
        var_vbrinvgat_db13 = 0.0;
        var_vbrinvgat_db14 = 0.0;
        var_vbrinvgat_db15 = 0.0;
        var_vbrinvgat_db16 = 0.0;
        var_vbrinvgat_db17 = 0.0;
        var_vbrinvgat_db18 = 0.0;
        var_vbrinvgat_db19 = 0.0;
        var_vbrinvgat_db20 = 0.0;
        var_vbrinvgat_db21 = 0.0;
        var_vbrinvgat_db22 = 0.0;
        var_vbrinvgat_db23 = 0.0;
        var_vbrinvgat_db24 = 0.0;

        let assign560_e1825: f64 = (var_fstopbot * var_fstopbot);
        let assign560_e1829: f64 = (p.p863 - 1.0);
        let assign560_e1830: f64 = (var_alphaav).powf(assign560_e1829);
        let assign560_e1831: f64 = (assign560_e1825 * assign560_e1830);
        let assign560_e1832: f64 = (-assign560_e1831);
        let assign560_e1834: f64 = (assign560_e1832 * p.p863);
        let assign560_e1836: f64 = (assign560_e1834 * var_vbrinvbot);
        var_slopebot = assign560_e1836;

        let assign570_e1839: f64 = (var_fstopsti * var_fstopsti);
        let assign570_e1843: f64 = (p.p864 - 1.0);
        let assign570_e1844: f64 = (var_alphaav).powf(assign570_e1843);
        let assign570_e1845: f64 = (assign570_e1839 * assign570_e1844);
        let assign570_e1846: f64 = (-assign570_e1845);
        let assign570_e1848: f64 = (assign570_e1846 * p.p864);
        let assign570_e1850: f64 = (assign570_e1848 * var_vbrinvsti);
        var_slopesti = assign570_e1850;

        let assign580_e1853: f64 = (var_fstopgat * var_fstopgat);
        let assign580_e1857: f64 = (p.p865 - 1.0);
        let assign580_e1858: f64 = (var_alphaav).powf(assign580_e1857);
        let assign580_e1859: f64 = (assign580_e1853 * assign580_e1858);
        let assign580_e1860: f64 = (-assign580_e1859);
        let assign580_e1862: f64 = (assign580_e1860 * p.p865);
        let assign580_e1864: f64 = (assign580_e1862 * var_vbrinvgat);
        var_slopegat = assign580_e1864;
        var_slopegat_dn0 = (assign580_e1862 * var_vbrinvgat_dn0);
        var_slopegat_dn1 = (assign580_e1862 * var_vbrinvgat_dn1);
        var_slopegat_dn2 = (assign580_e1862 * var_vbrinvgat_dn2);
        var_slopegat_dn3 = (assign580_e1862 * var_vbrinvgat_dn3);
        var_slopegat_dn4 = (assign580_e1862 * var_vbrinvgat_dn4);
        var_slopegat_dn5 = (assign580_e1862 * var_vbrinvgat_dn5);
        var_slopegat_dn6 = (assign580_e1862 * var_vbrinvgat_dn6);
        var_slopegat_dn7 = (assign580_e1862 * var_vbrinvgat_dn7);
        var_slopegat_dn8 = (assign580_e1862 * var_vbrinvgat_dn8);
        var_slopegat_dn9 = (assign580_e1862 * var_vbrinvgat_dn9);
        var_slopegat_dn10 = (assign580_e1862 * var_vbrinvgat_dn10);
        var_slopegat_dn11 = (assign580_e1862 * var_vbrinvgat_dn11);
        var_slopegat_dn12 = (assign580_e1862 * var_vbrinvgat_dn12);
        var_slopegat_dn13 = (assign580_e1862 * var_vbrinvgat_dn13);
        var_slopegat_dn14 = (assign580_e1862 * var_vbrinvgat_dn14);
        var_slopegat_dn15 = (assign580_e1862 * var_vbrinvgat_dn15);
        var_slopegat_dn16 = (assign580_e1862 * var_vbrinvgat_dn16);
        var_slopegat_dn17 = (assign580_e1862 * var_vbrinvgat_dn17);
        var_slopegat_dn18 = (assign580_e1862 * var_vbrinvgat_dn18);
        var_slopegat_dn19 = (assign580_e1862 * var_vbrinvgat_dn19);
        var_slopegat_dn20 = (assign580_e1862 * var_vbrinvgat_dn20);
        var_slopegat_db0 = (assign580_e1862 * var_vbrinvgat_db0);
        var_slopegat_db1 = (assign580_e1862 * var_vbrinvgat_db1);
        var_slopegat_db2 = (assign580_e1862 * var_vbrinvgat_db2);
        var_slopegat_db3 = (assign580_e1862 * var_vbrinvgat_db3);
        var_slopegat_db4 = (assign580_e1862 * var_vbrinvgat_db4);
        var_slopegat_db5 = (assign580_e1862 * var_vbrinvgat_db5);
        var_slopegat_db6 = (assign580_e1862 * var_vbrinvgat_db6);
        var_slopegat_db7 = (assign580_e1862 * var_vbrinvgat_db7);
        var_slopegat_db8 = (assign580_e1862 * var_vbrinvgat_db8);
        var_slopegat_db9 = (assign580_e1862 * var_vbrinvgat_db9);
        var_slopegat_db10 = (assign580_e1862 * var_vbrinvgat_db10);
        var_slopegat_db11 = (assign580_e1862 * var_vbrinvgat_db11);
        var_slopegat_db12 = (assign580_e1862 * var_vbrinvgat_db12);
        var_slopegat_db13 = (assign580_e1862 * var_vbrinvgat_db13);
        var_slopegat_db14 = (assign580_e1862 * var_vbrinvgat_db14);
        var_slopegat_db15 = (assign580_e1862 * var_vbrinvgat_db15);
        var_slopegat_db16 = (assign580_e1862 * var_vbrinvgat_db16);
        var_slopegat_db17 = (assign580_e1862 * var_vbrinvgat_db17);
        var_slopegat_db18 = (assign580_e1862 * var_vbrinvgat_db18);
        var_slopegat_db19 = (assign580_e1862 * var_vbrinvgat_db19);
        var_slopegat_db20 = (assign580_e1862 * var_vbrinvgat_db20);
        var_slopegat_db21 = (assign580_e1862 * var_vbrinvgat_db21);
        var_slopegat_db22 = (assign580_e1862 * var_vbrinvgat_db22);
        var_slopegat_db23 = (assign580_e1862 * var_vbrinvgat_db23);
        var_slopegat_db24 = (assign580_e1862 * var_vbrinvgat_db24);

        let assign590_e1879: f64 = if ((((p.p866 != 1.0) || (p.p867 != 1.0)) || (p.p868 != 1.0)) || (p.p869 != 1.0)) { 1.0 } else { 0.0 };
        var_guard8 = assign590_e1879;

        let (assign600_e1883,) = {
    if (var_guard8 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign600_e1883;

        let (assign610_e1888,) = {
    if (var_guard8 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign610_e1888;

        let assign620_e1891: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard9 = assign620_e1891;

        let (assign630_e1904,) = {
    if (var_guard9 != 0.0) {
        let assign630_e1895: f64 = (p.p827 * p.p866);
        let (assign630_e1902,) = {
            if (assign630_e1895 > 1e-18) {
                let assign630_e1900: f64 = (p.p827 * p.p866);
                (assign630_e1900,)
            } else {
                (1e-18,)
            }
        };
        (assign630_e1902,)
    } else {
        (var_cjorgat2nd,)
    }
};
        var_cjorgat2nd = assign630_e1904;

        let (assign640_e1917,) = {
    if (var_guard9 != 0.0) {
        let assign640_e1908: f64 = (p.p830 * p.p867);
        let (assign640_e1915,) = {
            if (assign640_e1908 > 0.05) {
                let assign640_e1913: f64 = (p.p830 * p.p867);
                (assign640_e1913,)
            } else {
                (0.05,)
            }
        };
        (assign640_e1915,)
    } else {
        (var_vbirgat2nd,)
    }
};
        var_vbirgat2nd = assign640_e1917;

        let (assign650_e1944,) = {
    if (var_guard9 != 0.0) {
        let assign650_e1921: f64 = (p.p833 * p.p868);
        let (assign650_e1928,) = {
            if (assign650_e1921 > 0.05) {
                let assign650_e1926: f64 = (p.p833 * p.p868);
                (assign650_e1926,)
            } else {
                (0.05,)
            }
        };
        let (assign650_e1942,) = {
            if (assign650_e1928 < 0.95) {
                let assign650_e1933: f64 = (p.p833 * p.p868);
                let (assign650_e1940,) = {
                    if (assign650_e1933 > 0.05) {
                        let assign650_e1938: f64 = (p.p833 * p.p868);
                        (assign650_e1938,)
                    } else {
                        (0.05,)
                    }
                };
                (assign650_e1940,)
            } else {
                (0.95,)
            }
        };
        (assign650_e1942,)
    } else {
        (var_pgat2nd,)
    }
};
        var_pgat2nd = assign650_e1944;

        let (assign660_e1950,) = {
    if (var_guard9 != 0.0) {
        let assign660_e1948: f64 = (p.p836 * p.p869);
        (assign660_e1948,)
    } else {
        (var_phiggat2nd,)
    }
};
        var_phiggat2nd = assign660_e1950;

        let (assign670_e1956,) = {
    if (var_guard9 != 0.0) {
        let assign670_e1954: f64 = (var_phiggat2nd + var_deltaphigr);
        (assign670_e1954,)
    } else {
        (var_phigrgat2nd,)
    }
};
        var_phigrgat2nd = assign670_e1956;

        let (assign680_e1962,) = {
    if (var_guard9 != 0.0) {
        let assign680_e1960: f64 = (1.0 - var_pgat2nd);
        (assign680_e1960,)
    } else {
        (var_one_minus_pgat2nd,)
    }
};
        var_one_minus_pgat2nd = assign680_e1962;

        let (assign690_e1968,) = {
    if (var_guard9 != 0.0) {
        let assign690_e1966: f64 = (1.0 / var_one_minus_pgat2nd);
        (assign690_e1966,)
    } else {
        (var_one_over_one_minus_pgat2nd,)
    }
};
        var_one_over_one_minus_pgat2nd = assign690_e1968;

        let assign700_e1971: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard10 = assign700_e1971;

        let (assign710_e1975,) = {
    if (var_guard10 != 0.0) {
        (p.p825,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign710_e1975;

        *var_alphaav_slot = var_alphaav;
        *var_berfc_slot = var_berfc;
        *var_cerfc_slot = var_cerfc;
        *var_chnl_type_slot = var_chnl_type;
        *var_cjorbotd_i_slot = var_cjorbotd_i;
        *var_cjorgat2nd_slot = var_cjorgat2nd;
        *var_deltaphigr_slot = var_deltaphigr;
        *var_epssi_slot = var_epssi;
        *var_fstopbot_slot = var_fstopbot;
        *var_fstopgat_slot = var_fstopgat;
        *var_fstopsti_slot = var_fstopsti;
        *var_guard1_slot = var_guard1;
        *var_guard10_slot = var_guard10;
        *var_guard7_slot = var_guard7;
        *var_guard8_slot = var_guard8;
        *var_guard9_slot = var_guard9;
        *var_kbol_over_qele_slot = var_kbol_over_qele;
        *var_one_minus_pbot_slot = var_one_minus_pbot;
        *var_one_minus_pgat_slot = var_one_minus_pgat;
        *var_one_minus_pgat2nd_slot = var_one_minus_pgat2nd;
        *var_one_minus_psti_slot = var_one_minus_psti;
        *var_one_over_one_minus_pbot_slot = var_one_over_one_minus_pbot;
        *var_one_over_one_minus_pgat_slot = var_one_over_one_minus_pgat;
        *var_one_over_one_minus_pgat2nd_slot = var_one_over_one_minus_pgat2nd;
        *var_one_over_one_minus_psti_slot = var_one_over_one_minus_psti;
        *var_perfc_slot = var_perfc;
        *var_pgat2nd_slot = var_pgat2nd;
        *var_phiggat2nd_slot = var_phiggat2nd;
        *var_phigrbot_slot = var_phigrbot;
        *var_phigrgat_slot = var_phigrgat;
        *var_phigrgat2nd_slot = var_phigrgat2nd;
        *var_phigrsti_slot = var_phigrsti;
        *var_phitr_slot = var_phitr;
        *var_phitrinv_slot = var_phitrinv;
        *var_slopebot_slot = var_slopebot;
        *var_slopegat_slot = var_slopegat;
        *var_slopegat_db0_slot = var_slopegat_db0;
        *var_slopegat_db1_slot = var_slopegat_db1;
        *var_slopegat_db10_slot = var_slopegat_db10;
        *var_slopegat_db11_slot = var_slopegat_db11;
        *var_slopegat_db12_slot = var_slopegat_db12;
        *var_slopegat_db13_slot = var_slopegat_db13;
        *var_slopegat_db14_slot = var_slopegat_db14;
        *var_slopegat_db15_slot = var_slopegat_db15;
        *var_slopegat_db16_slot = var_slopegat_db16;
        *var_slopegat_db17_slot = var_slopegat_db17;
        *var_slopegat_db18_slot = var_slopegat_db18;
        *var_slopegat_db19_slot = var_slopegat_db19;
        *var_slopegat_db2_slot = var_slopegat_db2;
        *var_slopegat_db20_slot = var_slopegat_db20;
        *var_slopegat_db21_slot = var_slopegat_db21;
        *var_slopegat_db22_slot = var_slopegat_db22;
        *var_slopegat_db23_slot = var_slopegat_db23;
        *var_slopegat_db24_slot = var_slopegat_db24;
        *var_slopegat_db3_slot = var_slopegat_db3;
        *var_slopegat_db4_slot = var_slopegat_db4;
        *var_slopegat_db5_slot = var_slopegat_db5;
        *var_slopegat_db6_slot = var_slopegat_db6;
        *var_slopegat_db7_slot = var_slopegat_db7;
        *var_slopegat_db8_slot = var_slopegat_db8;
        *var_slopegat_db9_slot = var_slopegat_db9;
        *var_slopegat_dn0_slot = var_slopegat_dn0;
        *var_slopegat_dn1_slot = var_slopegat_dn1;
        *var_slopegat_dn10_slot = var_slopegat_dn10;
        *var_slopegat_dn11_slot = var_slopegat_dn11;
        *var_slopegat_dn12_slot = var_slopegat_dn12;
        *var_slopegat_dn13_slot = var_slopegat_dn13;
        *var_slopegat_dn14_slot = var_slopegat_dn14;
        *var_slopegat_dn15_slot = var_slopegat_dn15;
        *var_slopegat_dn16_slot = var_slopegat_dn16;
        *var_slopegat_dn17_slot = var_slopegat_dn17;
        *var_slopegat_dn18_slot = var_slopegat_dn18;
        *var_slopegat_dn19_slot = var_slopegat_dn19;
        *var_slopegat_dn2_slot = var_slopegat_dn2;
        *var_slopegat_dn20_slot = var_slopegat_dn20;
        *var_slopegat_dn3_slot = var_slopegat_dn3;
        *var_slopegat_dn4_slot = var_slopegat_dn4;
        *var_slopegat_dn5_slot = var_slopegat_dn5;
        *var_slopegat_dn6_slot = var_slopegat_dn6;
        *var_slopegat_dn7_slot = var_slopegat_dn7;
        *var_slopegat_dn8_slot = var_slopegat_dn8;
        *var_slopegat_dn9_slot = var_slopegat_dn9;
        *var_slopesti_slot = var_slopesti;
        *var_swgat2nd_slot = var_swgat2nd;
        *var_swjunexp_i_slot = var_swjunexp_i;
        *var_tkr_slot = var_tkr;
        *var_tkr_1_slot = var_tkr_1;
        *var_vbirbotinv_slot = var_vbirbotinv;
        *var_vbirgat2nd_slot = var_vbirgat2nd;
        *var_vbirgatinv_slot = var_vbirgatinv;
        *var_vbirstiinv_slot = var_vbirstiinv;
        *var_vbrinvbot_slot = var_vbrinvbot;
        *var_vbrinvgat_slot = var_vbrinvgat;
        *var_vbrinvgat_db0_slot = var_vbrinvgat_db0;
        *var_vbrinvgat_db1_slot = var_vbrinvgat_db1;
        *var_vbrinvgat_db10_slot = var_vbrinvgat_db10;
        *var_vbrinvgat_db11_slot = var_vbrinvgat_db11;
        *var_vbrinvgat_db12_slot = var_vbrinvgat_db12;
        *var_vbrinvgat_db13_slot = var_vbrinvgat_db13;
        *var_vbrinvgat_db14_slot = var_vbrinvgat_db14;
        *var_vbrinvgat_db15_slot = var_vbrinvgat_db15;
        *var_vbrinvgat_db16_slot = var_vbrinvgat_db16;
        *var_vbrinvgat_db17_slot = var_vbrinvgat_db17;
        *var_vbrinvgat_db18_slot = var_vbrinvgat_db18;
        *var_vbrinvgat_db19_slot = var_vbrinvgat_db19;
        *var_vbrinvgat_db2_slot = var_vbrinvgat_db2;
        *var_vbrinvgat_db20_slot = var_vbrinvgat_db20;
        *var_vbrinvgat_db21_slot = var_vbrinvgat_db21;
        *var_vbrinvgat_db22_slot = var_vbrinvgat_db22;
        *var_vbrinvgat_db23_slot = var_vbrinvgat_db23;
        *var_vbrinvgat_db24_slot = var_vbrinvgat_db24;
        *var_vbrinvgat_db3_slot = var_vbrinvgat_db3;
        *var_vbrinvgat_db4_slot = var_vbrinvgat_db4;
        *var_vbrinvgat_db5_slot = var_vbrinvgat_db5;
        *var_vbrinvgat_db6_slot = var_vbrinvgat_db6;
        *var_vbrinvgat_db7_slot = var_vbrinvgat_db7;
        *var_vbrinvgat_db8_slot = var_vbrinvgat_db8;
        *var_vbrinvgat_db9_slot = var_vbrinvgat_db9;
        *var_vbrinvgat_dn0_slot = var_vbrinvgat_dn0;
        *var_vbrinvgat_dn1_slot = var_vbrinvgat_dn1;
        *var_vbrinvgat_dn10_slot = var_vbrinvgat_dn10;
        *var_vbrinvgat_dn11_slot = var_vbrinvgat_dn11;
        *var_vbrinvgat_dn12_slot = var_vbrinvgat_dn12;
        *var_vbrinvgat_dn13_slot = var_vbrinvgat_dn13;
        *var_vbrinvgat_dn14_slot = var_vbrinvgat_dn14;
        *var_vbrinvgat_dn15_slot = var_vbrinvgat_dn15;
        *var_vbrinvgat_dn16_slot = var_vbrinvgat_dn16;
        *var_vbrinvgat_dn17_slot = var_vbrinvgat_dn17;
        *var_vbrinvgat_dn18_slot = var_vbrinvgat_dn18;
        *var_vbrinvgat_dn19_slot = var_vbrinvgat_dn19;
        *var_vbrinvgat_dn2_slot = var_vbrinvgat_dn2;
        *var_vbrinvgat_dn20_slot = var_vbrinvgat_dn20;
        *var_vbrinvgat_dn3_slot = var_vbrinvgat_dn3;
        *var_vbrinvgat_dn4_slot = var_vbrinvgat_dn4;
        *var_vbrinvgat_dn5_slot = var_vbrinvgat_dn5;
        *var_vbrinvgat_dn6_slot = var_vbrinvgat_dn6;
        *var_vbrinvgat_dn7_slot = var_vbrinvgat_dn7;
        *var_vbrinvgat_dn8_slot = var_vbrinvgat_dn8;
        *var_vbrinvgat_dn9_slot = var_vbrinvgat_dn9;
        *var_vbrinvsti_slot = var_vbrinvsti;
        *var_wdepnulrbot_slot = var_wdepnulrbot;
        *var_wdepnulrgat_slot = var_wdepnulrgat;
        *var_wdepnulrinvbot_slot = var_wdepnulrinvbot;
        *var_wdepnulrinvgat_slot = var_wdepnulrinvgat;
        *var_wdepnulrinvsti_slot = var_wdepnulrinvsti;
        *var_wdepnulrsti_slot = var_wdepnulrsti;
    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        var_guard10: f64,
        var_adbbtgatd_i_slot: &mut f64,
        var_advbrgatd_i_slot: &mut f64,
        var_anugatd_i_slot: &mut f64,
        var_bdbbtgatd_i_slot: &mut f64,
        var_bdvbrgatd_i_slot: &mut f64,
        var_cbbtbotd_i_slot: &mut f64,
        var_cbbtgatd_i_slot: &mut f64,
        var_cbbtstid_i_slot: &mut f64,
        var_cjorbotd_i_slot: &mut f64,
        var_cjorgatd_i_slot: &mut f64,
        var_cjorstid_i_slot: &mut f64,
        var_csrhbotd_i_slot: &mut f64,
        var_csrhgatd_i_slot: &mut f64,
        var_csrhstid_i_slot: &mut f64,
        var_ctatbotd_i_slot: &mut f64,
        var_ctatgatd_i_slot: &mut f64,
        var_ctatstid_i_slot: &mut f64,
        var_fbbtrbotd_i_slot: &mut f64,
        var_fbbtrgatd_i_slot: &mut f64,
        var_fbbtrstid_i_slot: &mut f64,
        var_fcjorgat2d_i_slot: &mut f64,
        var_fjunqd_i_slot: &mut f64,
        var_fpgat2d_i_slot: &mut f64,
        var_fphiggat2d_i_slot: &mut f64,
        var_fvbirgat2d_i_slot: &mut f64,
        var_idsatrbotd_i_slot: &mut f64,
        var_idsatrgatd_i_slot: &mut f64,
        var_idsatrstid_i_slot: &mut f64,
        var_mefftatbotd_i_slot: &mut f64,
        var_mefftatgatd_i_slot: &mut f64,
        var_mefftatstid_i_slot: &mut f64,
        var_pbotd_i_slot: &mut f64,
        var_pbrbotd_i_slot: &mut f64,
        var_pbrgatd_i_slot: &mut f64,
        var_pbrstid_i_slot: &mut f64,
        var_pgatd_i_slot: &mut f64,
        var_phigbotd_i_slot: &mut f64,
        var_phiggatd_i_slot: &mut f64,
        var_phigstid_i_slot: &mut f64,
        var_pstid_i_slot: &mut f64,
        var_stfbbtbotd_i_slot: &mut f64,
        var_stfbbtgatd_i_slot: &mut f64,
        var_stfbbtstid_i_slot: &mut f64,
        var_vbirbotd_i_slot: &mut f64,
        var_vbirgatd_i_slot: &mut f64,
        var_vbirstid_i_slot: &mut f64,
        var_vbrbotd_i_slot: &mut f64,
        var_vbrgatd_i_slot: &mut f64,
        var_vbrstid_i_slot: &mut f64,
        var_vjunrefd_i_slot: &mut f64,
        var_vtrgatd_i_slot: &mut f64,
        var_xjungatd_i_slot: &mut f64,
        var_xjunstid_i_slot: &mut f64,
    ) {
        let mut var_adbbtgatd_i: f64 = *var_adbbtgatd_i_slot;
        let mut var_advbrgatd_i: f64 = *var_advbrgatd_i_slot;
        let mut var_anugatd_i: f64 = *var_anugatd_i_slot;
        let mut var_bdbbtgatd_i: f64 = *var_bdbbtgatd_i_slot;
        let mut var_bdvbrgatd_i: f64 = *var_bdvbrgatd_i_slot;
        let mut var_cbbtbotd_i: f64 = *var_cbbtbotd_i_slot;
        let mut var_cbbtgatd_i: f64 = *var_cbbtgatd_i_slot;
        let mut var_cbbtstid_i: f64 = *var_cbbtstid_i_slot;
        let mut var_cjorbotd_i: f64 = *var_cjorbotd_i_slot;
        let mut var_cjorgatd_i: f64 = *var_cjorgatd_i_slot;
        let mut var_cjorstid_i: f64 = *var_cjorstid_i_slot;
        let mut var_csrhbotd_i: f64 = *var_csrhbotd_i_slot;
        let mut var_csrhgatd_i: f64 = *var_csrhgatd_i_slot;
        let mut var_csrhstid_i: f64 = *var_csrhstid_i_slot;
        let mut var_ctatbotd_i: f64 = *var_ctatbotd_i_slot;
        let mut var_ctatgatd_i: f64 = *var_ctatgatd_i_slot;
        let mut var_ctatstid_i: f64 = *var_ctatstid_i_slot;
        let mut var_fbbtrbotd_i: f64 = *var_fbbtrbotd_i_slot;
        let mut var_fbbtrgatd_i: f64 = *var_fbbtrgatd_i_slot;
        let mut var_fbbtrstid_i: f64 = *var_fbbtrstid_i_slot;
        let mut var_fcjorgat2d_i: f64 = *var_fcjorgat2d_i_slot;
        let mut var_fjunqd_i: f64 = *var_fjunqd_i_slot;
        let mut var_fpgat2d_i: f64 = *var_fpgat2d_i_slot;
        let mut var_fphiggat2d_i: f64 = *var_fphiggat2d_i_slot;
        let mut var_fvbirgat2d_i: f64 = *var_fvbirgat2d_i_slot;
        let mut var_idsatrbotd_i: f64 = *var_idsatrbotd_i_slot;
        let mut var_idsatrgatd_i: f64 = *var_idsatrgatd_i_slot;
        let mut var_idsatrstid_i: f64 = *var_idsatrstid_i_slot;
        let mut var_mefftatbotd_i: f64 = *var_mefftatbotd_i_slot;
        let mut var_mefftatgatd_i: f64 = *var_mefftatgatd_i_slot;
        let mut var_mefftatstid_i: f64 = *var_mefftatstid_i_slot;
        let mut var_pbotd_i: f64 = *var_pbotd_i_slot;
        let mut var_pbrbotd_i: f64 = *var_pbrbotd_i_slot;
        let mut var_pbrgatd_i: f64 = *var_pbrgatd_i_slot;
        let mut var_pbrstid_i: f64 = *var_pbrstid_i_slot;
        let mut var_pgatd_i: f64 = *var_pgatd_i_slot;
        let mut var_phigbotd_i: f64 = *var_phigbotd_i_slot;
        let mut var_phiggatd_i: f64 = *var_phiggatd_i_slot;
        let mut var_phigstid_i: f64 = *var_phigstid_i_slot;
        let mut var_pstid_i: f64 = *var_pstid_i_slot;
        let mut var_stfbbtbotd_i: f64 = *var_stfbbtbotd_i_slot;
        let mut var_stfbbtgatd_i: f64 = *var_stfbbtgatd_i_slot;
        let mut var_stfbbtstid_i: f64 = *var_stfbbtstid_i_slot;
        let mut var_vbirbotd_i: f64 = *var_vbirbotd_i_slot;
        let mut var_vbirgatd_i: f64 = *var_vbirgatd_i_slot;
        let mut var_vbirstid_i: f64 = *var_vbirstid_i_slot;
        let mut var_vbrbotd_i: f64 = *var_vbrbotd_i_slot;
        let mut var_vbrgatd_i: f64 = *var_vbrgatd_i_slot;
        let mut var_vbrstid_i: f64 = *var_vbrstid_i_slot;
        let mut var_vjunrefd_i: f64 = *var_vjunrefd_i_slot;
        let mut var_vtrgatd_i: f64 = *var_vtrgatd_i_slot;
        let mut var_xjungatd_i: f64 = *var_xjungatd_i_slot;
        let mut var_xjunstid_i: f64 = *var_xjunstid_i_slot;

        let (assign720_e1979,) = {
    if (var_guard10 != 0.0) {
        (p.p826,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign720_e1979;

        let (assign730_e1983,) = {
    if (var_guard10 != 0.0) {
        (p.p827,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign730_e1983;

        let (assign740_e1987,) = {
    if (var_guard10 != 0.0) {
        (p.p828,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign740_e1987;

        let (assign750_e1991,) = {
    if (var_guard10 != 0.0) {
        (p.p829,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign750_e1991;

        let (assign760_e1995,) = {
    if (var_guard10 != 0.0) {
        (p.p830,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign760_e1995;

        let (assign770_e1999,) = {
    if (var_guard10 != 0.0) {
        (p.p831,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign770_e1999;

        let (assign780_e2003,) = {
    if (var_guard10 != 0.0) {
        (p.p832,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign780_e2003;

        let (assign790_e2007,) = {
    if (var_guard10 != 0.0) {
        (p.p833,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign790_e2007;

        let (assign800_e2011,) = {
    if (var_guard10 != 0.0) {
        (p.p834,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign800_e2011;

        let (assign810_e2015,) = {
    if (var_guard10 != 0.0) {
        (p.p835,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign810_e2015;

        let (assign820_e2019,) = {
    if (var_guard10 != 0.0) {
        (p.p836,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign820_e2019;

        let (assign830_e2023,) = {
    if (var_guard10 != 0.0) {
        (p.p837,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign830_e2023;

        let (assign840_e2027,) = {
    if (var_guard10 != 0.0) {
        (p.p838,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign840_e2027;

        let (assign850_e2031,) = {
    if (var_guard10 != 0.0) {
        (p.p839,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign850_e2031;

        let (assign860_e2035,) = {
    if (var_guard10 != 0.0) {
        (p.p840,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign860_e2035;

        let (assign870_e2039,) = {
    if (var_guard10 != 0.0) {
        (p.p841,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign870_e2039;

        let (assign880_e2043,) = {
    if (var_guard10 != 0.0) {
        (p.p842,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign880_e2043;

        let (assign890_e2047,) = {
    if (var_guard10 != 0.0) {
        (p.p843,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign890_e2047;

        let (assign900_e2051,) = {
    if (var_guard10 != 0.0) {
        (p.p844,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign900_e2051;

        let (assign910_e2055,) = {
    if (var_guard10 != 0.0) {
        (p.p845,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign910_e2055;

        let (assign920_e2059,) = {
    if (var_guard10 != 0.0) {
        (p.p846,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign920_e2059;

        let (assign930_e2063,) = {
    if (var_guard10 != 0.0) {
        (p.p847,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign930_e2063;

        let (assign940_e2067,) = {
    if (var_guard10 != 0.0) {
        (p.p848,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign940_e2067;

        let (assign950_e2071,) = {
    if (var_guard10 != 0.0) {
        (p.p849,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign950_e2071;

        let (assign960_e2075,) = {
    if (var_guard10 != 0.0) {
        (p.p850,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign960_e2075;

        let (assign970_e2079,) = {
    if (var_guard10 != 0.0) {
        (p.p851,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign970_e2079;

        let (assign980_e2083,) = {
    if (var_guard10 != 0.0) {
        (p.p852,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign980_e2083;

        let (assign990_e2087,) = {
    if (var_guard10 != 0.0) {
        (p.p853,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign990_e2087;

        let (assign1000_e2091,) = {
    if (var_guard10 != 0.0) {
        (p.p854,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign1000_e2091;

        let (assign1010_e2095,) = {
    if (var_guard10 != 0.0) {
        (p.p855,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign1010_e2095;

        let (assign1020_e2099,) = {
    if (var_guard10 != 0.0) {
        (p.p856,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign1020_e2099;

        let (assign1030_e2103,) = {
    if (var_guard10 != 0.0) {
        (p.p857,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign1030_e2103;

        let (assign1040_e2107,) = {
    if (var_guard10 != 0.0) {
        (p.p858,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign1040_e2107;

        let (assign1050_e2111,) = {
    if (var_guard10 != 0.0) {
        (p.p859,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign1050_e2111;

        let (assign1060_e2115,) = {
    if (var_guard10 != 0.0) {
        (p.p860,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign1060_e2115;

        let (assign1070_e2119,) = {
    if (var_guard10 != 0.0) {
        (p.p861,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign1070_e2119;

        let (assign1080_e2123,) = {
    if (var_guard10 != 0.0) {
        (p.p862,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign1080_e2123;

        let (assign1090_e2127,) = {
    if (var_guard10 != 0.0) {
        (p.p863,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign1090_e2127;

        let (assign1100_e2131,) = {
    if (var_guard10 != 0.0) {
        (p.p864,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign1100_e2131;

        let (assign1110_e2135,) = {
    if (var_guard10 != 0.0) {
        (p.p865,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign1110_e2135;

        let (assign1120_e2139,) = {
    if (var_guard10 != 0.0) {
        (p.p928,)
    } else {
        (var_vjunrefd_i,)
    }
};
        var_vjunrefd_i = assign1120_e2139;

        let (assign1130_e2143,) = {
    if (var_guard10 != 0.0) {
        (p.p929,)
    } else {
        (var_fjunqd_i,)
    }
};
        var_fjunqd_i = assign1130_e2143;

        let (assign1140_e2147,) = {
    if (var_guard10 != 0.0) {
        (p.p872,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1140_e2147;

        let (assign1150_e2151,) = {
    if (var_guard10 != 0.0) {
        (p.p873,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1150_e2151;

        let (assign1160_e2155,) = {
    if (var_guard10 != 0.0) {
        (p.p874,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1160_e2155;

        let (assign1170_e2159,) = {
    if (var_guard10 != 0.0) {
        (p.p875,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1170_e2159;

        let (assign1180_e2163,) = {
    if (var_guard10 != 0.0) {
        (p.p866,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1180_e2163;

        let (assign1190_e2167,) = {
    if (var_guard10 != 0.0) {
        (p.p867,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1190_e2167;

        let (assign1200_e2171,) = {
    if (var_guard10 != 0.0) {
        (p.p868,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1200_e2171;

        let (assign1210_e2175,) = {
    if (var_guard10 != 0.0) {
        (p.p869,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1210_e2175;

        let (assign1220_e2179,) = {
    if (var_guard10 != 0.0) {
        (p.p870,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1220_e2179;

        let (assign1230_e2183,) = {
    if (var_guard10 != 0.0) {
        (p.p871,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1230_e2183;

        let (assign1240_e2188,) = {
    if (var_guard10 == 0.0) {
        (p.p876,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign1240_e2188;

        let (assign1250_e2193,) = {
    if (var_guard10 == 0.0) {
        (p.p877,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign1250_e2193;

        let (assign1260_e2198,) = {
    if (var_guard10 == 0.0) {
        (p.p878,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign1260_e2198;

        let (assign1270_e2203,) = {
    if (var_guard10 == 0.0) {
        (p.p879,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign1270_e2203;

        *var_adbbtgatd_i_slot = var_adbbtgatd_i;
        *var_advbrgatd_i_slot = var_advbrgatd_i;
        *var_anugatd_i_slot = var_anugatd_i;
        *var_bdbbtgatd_i_slot = var_bdbbtgatd_i;
        *var_bdvbrgatd_i_slot = var_bdvbrgatd_i;
        *var_cbbtbotd_i_slot = var_cbbtbotd_i;
        *var_cbbtgatd_i_slot = var_cbbtgatd_i;
        *var_cbbtstid_i_slot = var_cbbtstid_i;
        *var_cjorbotd_i_slot = var_cjorbotd_i;
        *var_cjorgatd_i_slot = var_cjorgatd_i;
        *var_cjorstid_i_slot = var_cjorstid_i;
        *var_csrhbotd_i_slot = var_csrhbotd_i;
        *var_csrhgatd_i_slot = var_csrhgatd_i;
        *var_csrhstid_i_slot = var_csrhstid_i;
        *var_ctatbotd_i_slot = var_ctatbotd_i;
        *var_ctatgatd_i_slot = var_ctatgatd_i;
        *var_ctatstid_i_slot = var_ctatstid_i;
        *var_fbbtrbotd_i_slot = var_fbbtrbotd_i;
        *var_fbbtrgatd_i_slot = var_fbbtrgatd_i;
        *var_fbbtrstid_i_slot = var_fbbtrstid_i;
        *var_fcjorgat2d_i_slot = var_fcjorgat2d_i;
        *var_fjunqd_i_slot = var_fjunqd_i;
        *var_fpgat2d_i_slot = var_fpgat2d_i;
        *var_fphiggat2d_i_slot = var_fphiggat2d_i;
        *var_fvbirgat2d_i_slot = var_fvbirgat2d_i;
        *var_idsatrbotd_i_slot = var_idsatrbotd_i;
        *var_idsatrgatd_i_slot = var_idsatrgatd_i;
        *var_idsatrstid_i_slot = var_idsatrstid_i;
        *var_mefftatbotd_i_slot = var_mefftatbotd_i;
        *var_mefftatgatd_i_slot = var_mefftatgatd_i;
        *var_mefftatstid_i_slot = var_mefftatstid_i;
        *var_pbotd_i_slot = var_pbotd_i;
        *var_pbrbotd_i_slot = var_pbrbotd_i;
        *var_pbrgatd_i_slot = var_pbrgatd_i;
        *var_pbrstid_i_slot = var_pbrstid_i;
        *var_pgatd_i_slot = var_pgatd_i;
        *var_phigbotd_i_slot = var_phigbotd_i;
        *var_phiggatd_i_slot = var_phiggatd_i;
        *var_phigstid_i_slot = var_phigstid_i;
        *var_pstid_i_slot = var_pstid_i;
        *var_stfbbtbotd_i_slot = var_stfbbtbotd_i;
        *var_stfbbtgatd_i_slot = var_stfbbtgatd_i;
        *var_stfbbtstid_i_slot = var_stfbbtstid_i;
        *var_vbirbotd_i_slot = var_vbirbotd_i;
        *var_vbirgatd_i_slot = var_vbirgatd_i;
        *var_vbirstid_i_slot = var_vbirstid_i;
        *var_vbrbotd_i_slot = var_vbrbotd_i;
        *var_vbrgatd_i_slot = var_vbrgatd_i;
        *var_vbrstid_i_slot = var_vbrstid_i;
        *var_vjunrefd_i_slot = var_vjunrefd_i;
        *var_vtrgatd_i_slot = var_vtrgatd_i;
        *var_xjungatd_i_slot = var_xjungatd_i;
        *var_xjunstid_i_slot = var_xjunstid_i;
    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        var_alphaav: f64,
        var_cjorbotd_i: f64,
        var_cjorgatd_i: f64,
        var_cjorstid_i: f64,
        var_deltaphigr: f64,
        var_epssi: f64,
        var_guard10: f64,
        var_vbirbotd_i: f64,
        var_adbbtgatd_i_slot: &mut f64,
        var_advbrgatd_i_slot: &mut f64,
        var_anugatd_i_slot: &mut f64,
        var_bdbbtgatd_i_slot: &mut f64,
        var_bdvbrgatd_i_slot: &mut f64,
        var_cbbtbotd_i_slot: &mut f64,
        var_cbbtgatd_i_slot: &mut f64,
        var_cbbtstid_i_slot: &mut f64,
        var_csrhbotd_i_slot: &mut f64,
        var_csrhgatd_i_slot: &mut f64,
        var_csrhstid_i_slot: &mut f64,
        var_ctatbotd_i_slot: &mut f64,
        var_ctatgatd_i_slot: &mut f64,
        var_ctatstid_i_slot: &mut f64,
        var_fbbtrbotd_i_slot: &mut f64,
        var_fbbtrgatd_i_slot: &mut f64,
        var_fbbtrstid_i_slot: &mut f64,
        var_fcjorgat2d_i_slot: &mut f64,
        var_fjunqd_i_slot: &mut f64,
        var_fpgat2d_i_slot: &mut f64,
        var_fphiggat2d_i_slot: &mut f64,
        var_fstopbot_d_slot: &mut f64,
        var_fstopgat_d_slot: &mut f64,
        var_fstopsti_d_slot: &mut f64,
        var_fvbirgat2d_i_slot: &mut f64,
        var_idsatrbotd_i_slot: &mut f64,
        var_idsatrgatd_i_slot: &mut f64,
        var_idsatrstid_i_slot: &mut f64,
        var_mefftatbotd_i_slot: &mut f64,
        var_mefftatgatd_i_slot: &mut f64,
        var_mefftatstid_i_slot: &mut f64,
        var_one_minus_pbot_d_slot: &mut f64,
        var_one_minus_pgat_d_slot: &mut f64,
        var_one_minus_psti_d_slot: &mut f64,
        var_one_over_one_minus_pbot_d_slot: &mut f64,
        var_one_over_one_minus_pgat_d_slot: &mut f64,
        var_one_over_one_minus_psti_d_slot: &mut f64,
        var_pbotd_i_slot: &mut f64,
        var_pbrbotd_i_slot: &mut f64,
        var_pbrgatd_i_slot: &mut f64,
        var_pbrstid_i_slot: &mut f64,
        var_pgatd_i_slot: &mut f64,
        var_phigbotd_i_slot: &mut f64,
        var_phiggatd_i_slot: &mut f64,
        var_phigrbot_d_slot: &mut f64,
        var_phigrgat_d_slot: &mut f64,
        var_phigrsti_d_slot: &mut f64,
        var_phigstid_i_slot: &mut f64,
        var_pstid_i_slot: &mut f64,
        var_stfbbtbotd_i_slot: &mut f64,
        var_stfbbtgatd_i_slot: &mut f64,
        var_stfbbtstid_i_slot: &mut f64,
        var_vbirbotinv_d_slot: &mut f64,
        var_vbirgatd_i_slot: &mut f64,
        var_vbirgatinv_d_slot: &mut f64,
        var_vbirstid_i_slot: &mut f64,
        var_vbirstiinv_d_slot: &mut f64,
        var_vbrbotd_i_slot: &mut f64,
        var_vbrgatd_i_slot: &mut f64,
        var_vbrstid_i_slot: &mut f64,
        var_vjunrefd_i_slot: &mut f64,
        var_vtrgatd_i_slot: &mut f64,
        var_wdepnulrbot_d_slot: &mut f64,
        var_wdepnulrgat_d_slot: &mut f64,
        var_wdepnulrinvbot_d_slot: &mut f64,
        var_wdepnulrinvgat_d_slot: &mut f64,
        var_wdepnulrinvsti_d_slot: &mut f64,
        var_wdepnulrsti_d_slot: &mut f64,
        var_xjungatd_i_slot: &mut f64,
        var_xjunstid_i_slot: &mut f64,
    ) {
        let mut var_adbbtgatd_i: f64 = *var_adbbtgatd_i_slot;
        let mut var_advbrgatd_i: f64 = *var_advbrgatd_i_slot;
        let mut var_anugatd_i: f64 = *var_anugatd_i_slot;
        let mut var_bdbbtgatd_i: f64 = *var_bdbbtgatd_i_slot;
        let mut var_bdvbrgatd_i: f64 = *var_bdvbrgatd_i_slot;
        let mut var_cbbtbotd_i: f64 = *var_cbbtbotd_i_slot;
        let mut var_cbbtgatd_i: f64 = *var_cbbtgatd_i_slot;
        let mut var_cbbtstid_i: f64 = *var_cbbtstid_i_slot;
        let mut var_csrhbotd_i: f64 = *var_csrhbotd_i_slot;
        let mut var_csrhgatd_i: f64 = *var_csrhgatd_i_slot;
        let mut var_csrhstid_i: f64 = *var_csrhstid_i_slot;
        let mut var_ctatbotd_i: f64 = *var_ctatbotd_i_slot;
        let mut var_ctatgatd_i: f64 = *var_ctatgatd_i_slot;
        let mut var_ctatstid_i: f64 = *var_ctatstid_i_slot;
        let mut var_fbbtrbotd_i: f64 = *var_fbbtrbotd_i_slot;
        let mut var_fbbtrgatd_i: f64 = *var_fbbtrgatd_i_slot;
        let mut var_fbbtrstid_i: f64 = *var_fbbtrstid_i_slot;
        let mut var_fcjorgat2d_i: f64 = *var_fcjorgat2d_i_slot;
        let mut var_fjunqd_i: f64 = *var_fjunqd_i_slot;
        let mut var_fpgat2d_i: f64 = *var_fpgat2d_i_slot;
        let mut var_fphiggat2d_i: f64 = *var_fphiggat2d_i_slot;
        let mut var_fstopbot_d: f64 = *var_fstopbot_d_slot;
        let mut var_fstopgat_d: f64 = *var_fstopgat_d_slot;
        let mut var_fstopsti_d: f64 = *var_fstopsti_d_slot;
        let mut var_fvbirgat2d_i: f64 = *var_fvbirgat2d_i_slot;
        let mut var_idsatrbotd_i: f64 = *var_idsatrbotd_i_slot;
        let mut var_idsatrgatd_i: f64 = *var_idsatrgatd_i_slot;
        let mut var_idsatrstid_i: f64 = *var_idsatrstid_i_slot;
        let mut var_mefftatbotd_i: f64 = *var_mefftatbotd_i_slot;
        let mut var_mefftatgatd_i: f64 = *var_mefftatgatd_i_slot;
        let mut var_mefftatstid_i: f64 = *var_mefftatstid_i_slot;
        let mut var_one_minus_pbot_d: f64 = *var_one_minus_pbot_d_slot;
        let mut var_one_minus_pgat_d: f64 = *var_one_minus_pgat_d_slot;
        let mut var_one_minus_psti_d: f64 = *var_one_minus_psti_d_slot;
        let mut var_one_over_one_minus_pbot_d: f64 = *var_one_over_one_minus_pbot_d_slot;
        let mut var_one_over_one_minus_pgat_d: f64 = *var_one_over_one_minus_pgat_d_slot;
        let mut var_one_over_one_minus_psti_d: f64 = *var_one_over_one_minus_psti_d_slot;
        let mut var_pbotd_i: f64 = *var_pbotd_i_slot;
        let mut var_pbrbotd_i: f64 = *var_pbrbotd_i_slot;
        let mut var_pbrgatd_i: f64 = *var_pbrgatd_i_slot;
        let mut var_pbrstid_i: f64 = *var_pbrstid_i_slot;
        let mut var_pgatd_i: f64 = *var_pgatd_i_slot;
        let mut var_phigbotd_i: f64 = *var_phigbotd_i_slot;
        let mut var_phiggatd_i: f64 = *var_phiggatd_i_slot;
        let mut var_phigrbot_d: f64 = *var_phigrbot_d_slot;
        let mut var_phigrgat_d: f64 = *var_phigrgat_d_slot;
        let mut var_phigrsti_d: f64 = *var_phigrsti_d_slot;
        let mut var_phigstid_i: f64 = *var_phigstid_i_slot;
        let mut var_pstid_i: f64 = *var_pstid_i_slot;
        let mut var_stfbbtbotd_i: f64 = *var_stfbbtbotd_i_slot;
        let mut var_stfbbtgatd_i: f64 = *var_stfbbtgatd_i_slot;
        let mut var_stfbbtstid_i: f64 = *var_stfbbtstid_i_slot;
        let mut var_vbirbotinv_d: f64 = *var_vbirbotinv_d_slot;
        let mut var_vbirgatd_i: f64 = *var_vbirgatd_i_slot;
        let mut var_vbirgatinv_d: f64 = *var_vbirgatinv_d_slot;
        let mut var_vbirstid_i: f64 = *var_vbirstid_i_slot;
        let mut var_vbirstiinv_d: f64 = *var_vbirstiinv_d_slot;
        let mut var_vbrbotd_i: f64 = *var_vbrbotd_i_slot;
        let mut var_vbrgatd_i: f64 = *var_vbrgatd_i_slot;
        let mut var_vbrstid_i: f64 = *var_vbrstid_i_slot;
        let mut var_vjunrefd_i: f64 = *var_vjunrefd_i_slot;
        let mut var_vtrgatd_i: f64 = *var_vtrgatd_i_slot;
        let mut var_wdepnulrbot_d: f64 = *var_wdepnulrbot_d_slot;
        let mut var_wdepnulrgat_d: f64 = *var_wdepnulrgat_d_slot;
        let mut var_wdepnulrinvbot_d: f64 = *var_wdepnulrinvbot_d_slot;
        let mut var_wdepnulrinvgat_d: f64 = *var_wdepnulrinvgat_d_slot;
        let mut var_wdepnulrinvsti_d: f64 = *var_wdepnulrinvsti_d_slot;
        let mut var_wdepnulrsti_d: f64 = *var_wdepnulrsti_d_slot;
        let mut var_xjungatd_i: f64 = *var_xjungatd_i_slot;
        let mut var_xjunstid_i: f64 = *var_xjunstid_i_slot;

        let (assign1280_e2208,) = {
    if (var_guard10 == 0.0) {
        (p.p880,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign1280_e2208;

        let (assign1290_e2213,) = {
    if (var_guard10 == 0.0) {
        (p.p881,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign1290_e2213;

        let (assign1300_e2218,) = {
    if (var_guard10 == 0.0) {
        (p.p882,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign1300_e2218;

        let (assign1310_e2223,) = {
    if (var_guard10 == 0.0) {
        (p.p883,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign1310_e2223;

        let (assign1320_e2228,) = {
    if (var_guard10 == 0.0) {
        (p.p884,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign1320_e2228;

        let (assign1330_e2233,) = {
    if (var_guard10 == 0.0) {
        (p.p885,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign1330_e2233;

        let (assign1340_e2238,) = {
    if (var_guard10 == 0.0) {
        (p.p886,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign1340_e2238;

        let (assign1350_e2243,) = {
    if (var_guard10 == 0.0) {
        (p.p887,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign1350_e2243;

        let (assign1360_e2248,) = {
    if (var_guard10 == 0.0) {
        (p.p888,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign1360_e2248;

        let (assign1370_e2253,) = {
    if (var_guard10 == 0.0) {
        (p.p889,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign1370_e2253;

        let (assign1380_e2258,) = {
    if (var_guard10 == 0.0) {
        (p.p890,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign1380_e2258;

        let (assign1390_e2263,) = {
    if (var_guard10 == 0.0) {
        (p.p891,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign1390_e2263;

        let (assign1400_e2268,) = {
    if (var_guard10 == 0.0) {
        (p.p892,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign1400_e2268;

        let (assign1410_e2273,) = {
    if (var_guard10 == 0.0) {
        (p.p893,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign1410_e2273;

        let (assign1420_e2278,) = {
    if (var_guard10 == 0.0) {
        (p.p894,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign1420_e2278;

        let (assign1430_e2283,) = {
    if (var_guard10 == 0.0) {
        (p.p895,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign1430_e2283;

        let (assign1440_e2288,) = {
    if (var_guard10 == 0.0) {
        (p.p896,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign1440_e2288;

        let (assign1450_e2293,) = {
    if (var_guard10 == 0.0) {
        (p.p897,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign1450_e2293;

        let (assign1460_e2298,) = {
    if (var_guard10 == 0.0) {
        (p.p898,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign1460_e2298;

        let (assign1470_e2303,) = {
    if (var_guard10 == 0.0) {
        (p.p899,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign1470_e2303;

        let (assign1480_e2308,) = {
    if (var_guard10 == 0.0) {
        (p.p900,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign1480_e2308;

        let (assign1490_e2313,) = {
    if (var_guard10 == 0.0) {
        (p.p901,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign1490_e2313;

        let (assign1500_e2318,) = {
    if (var_guard10 == 0.0) {
        (p.p902,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign1500_e2318;

        let (assign1510_e2323,) = {
    if (var_guard10 == 0.0) {
        (p.p903,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign1510_e2323;

        let (assign1520_e2328,) = {
    if (var_guard10 == 0.0) {
        (p.p904,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign1520_e2328;

        let (assign1530_e2333,) = {
    if (var_guard10 == 0.0) {
        (p.p905,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign1530_e2333;

        let (assign1540_e2338,) = {
    if (var_guard10 == 0.0) {
        (p.p906,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign1540_e2338;

        let (assign1550_e2343,) = {
    if (var_guard10 == 0.0) {
        (p.p907,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign1550_e2343;

        let (assign1560_e2348,) = {
    if (var_guard10 == 0.0) {
        (p.p908,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign1560_e2348;

        let (assign1570_e2353,) = {
    if (var_guard10 == 0.0) {
        (p.p909,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign1570_e2353;

        let (assign1580_e2358,) = {
    if (var_guard10 == 0.0) {
        (p.p910,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign1580_e2358;

        let (assign1590_e2363,) = {
    if (var_guard10 == 0.0) {
        (p.p911,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign1590_e2363;

        let (assign1600_e2368,) = {
    if (var_guard10 == 0.0) {
        (p.p912,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign1600_e2368;

        let (assign1610_e2373,) = {
    if (var_guard10 == 0.0) {
        (p.p913,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign1610_e2373;

        let (assign1620_e2378,) = {
    if (var_guard10 == 0.0) {
        (p.p914,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign1620_e2378;

        let (assign1630_e2383,) = {
    if (var_guard10 == 0.0) {
        (p.p915,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign1630_e2383;

        let (assign1640_e2388,) = {
    if (var_guard10 == 0.0) {
        (p.p916,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign1640_e2388;

        let (assign1650_e2393,) = {
    if (var_guard10 == 0.0) {
        (p.p930,)
    } else {
        (var_vjunrefd_i,)
    }
};
        var_vjunrefd_i = assign1650_e2393;

        let (assign1660_e2398,) = {
    if (var_guard10 == 0.0) {
        (p.p931,)
    } else {
        (var_fjunqd_i,)
    }
};
        var_fjunqd_i = assign1660_e2398;

        let (assign1670_e2403,) = {
    if (var_guard10 == 0.0) {
        (p.p923,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1670_e2403;

        let (assign1680_e2408,) = {
    if (var_guard10 == 0.0) {
        (p.p924,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1680_e2408;

        let (assign1690_e2413,) = {
    if (var_guard10 == 0.0) {
        (p.p925,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1690_e2413;

        let (assign1700_e2418,) = {
    if (var_guard10 == 0.0) {
        (p.p926,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1700_e2418;

        let (assign1710_e2423,) = {
    if (var_guard10 == 0.0) {
        (p.p917,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1710_e2423;

        let (assign1720_e2428,) = {
    if (var_guard10 == 0.0) {
        (p.p918,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1720_e2428;

        let (assign1730_e2433,) = {
    if (var_guard10 == 0.0) {
        (p.p919,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1730_e2433;

        let (assign1740_e2438,) = {
    if (var_guard10 == 0.0) {
        (p.p920,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1740_e2438;

        let (assign1750_e2443,) = {
    if (var_guard10 == 0.0) {
        (p.p921,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1750_e2443;

        let (assign1760_e2448,) = {
    if (var_guard10 == 0.0) {
        (p.p922,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1760_e2448;

        let assign1770_e2451: f64 = (var_phigbotd_i + var_deltaphigr);
        var_phigrbot_d = assign1770_e2451;

        let assign1780_e2454: f64 = (var_phigstid_i + var_deltaphigr);
        var_phigrsti_d = assign1780_e2454;

        let assign1790_e2457: f64 = (var_phiggatd_i + var_deltaphigr);
        var_phigrgat_d = assign1790_e2457;

        let assign1800_e2460: f64 = (1.0 - var_pbotd_i);
        var_one_minus_pbot_d = assign1800_e2460;

        let assign1810_e2463: f64 = (1.0 - var_pstid_i);
        var_one_minus_psti_d = assign1810_e2463;

        let assign1820_e2466: f64 = (1.0 - var_pgatd_i);
        var_one_minus_pgat_d = assign1820_e2466;

        let assign1830_e2469: f64 = (1.0 / var_one_minus_pbot_d);
        var_one_over_one_minus_pbot_d = assign1830_e2469;

        let assign1840_e2472: f64 = (1.0 / var_one_minus_psti_d);
        var_one_over_one_minus_psti_d = assign1840_e2472;

        let assign1850_e2475: f64 = (1.0 / var_one_minus_pgat_d);
        var_one_over_one_minus_pgat_d = assign1850_e2475;

        let assign1860_e2478: f64 = (var_epssi / var_cjorbotd_i);
        var_wdepnulrbot_d = assign1860_e2478;

        let assign1870_e2481: f64 = (var_xjunstid_i * var_epssi);
        let assign1870_e2483: f64 = (assign1870_e2481 / var_cjorstid_i);
        var_wdepnulrsti_d = assign1870_e2483;

        let assign1880_e2486: f64 = (var_xjungatd_i * var_epssi);
        let assign1880_e2488: f64 = (assign1880_e2486 / var_cjorgatd_i);
        var_wdepnulrgat_d = assign1880_e2488;

        let assign1890_e2491: f64 = (1.0 / var_wdepnulrbot_d);
        var_wdepnulrinvbot_d = assign1890_e2491;

        let assign1900_e2494: f64 = (1.0 / var_wdepnulrsti_d);
        var_wdepnulrinvsti_d = assign1900_e2494;

        let assign1910_e2497: f64 = (1.0 / var_wdepnulrgat_d);
        var_wdepnulrinvgat_d = assign1910_e2497;

        let assign1920_e2500: f64 = (1.0 / var_vbirbotd_i);
        var_vbirbotinv_d = assign1920_e2500;

        let assign1930_e2503: f64 = (1.0 / var_vbirstid_i);
        var_vbirstiinv_d = assign1930_e2503;

        let assign1940_e2506: f64 = (1.0 / var_vbirgatd_i);
        var_vbirgatinv_d = assign1940_e2506;

        let assign1950_e2511: f64 = (var_alphaav).powf(var_pbrbotd_i);
        let assign1950_e2512: f64 = (1.0 - assign1950_e2511);
        let assign1950_e2513: f64 = (1.0 / assign1950_e2512);
        var_fstopbot_d = assign1950_e2513;

        let assign1960_e2518: f64 = (var_alphaav).powf(var_pbrstid_i);
        let assign1960_e2519: f64 = (1.0 - assign1960_e2518);
        let assign1960_e2520: f64 = (1.0 / assign1960_e2519);
        var_fstopsti_d = assign1960_e2520;

        let assign1970_e2525: f64 = (var_alphaav).powf(var_pbrgatd_i);
        let assign1970_e2526: f64 = (1.0 - assign1970_e2525);
        let assign1970_e2527: f64 = (1.0 / assign1970_e2526);
        var_fstopgat_d = assign1970_e2527;

        *var_adbbtgatd_i_slot = var_adbbtgatd_i;
        *var_advbrgatd_i_slot = var_advbrgatd_i;
        *var_anugatd_i_slot = var_anugatd_i;
        *var_bdbbtgatd_i_slot = var_bdbbtgatd_i;
        *var_bdvbrgatd_i_slot = var_bdvbrgatd_i;
        *var_cbbtbotd_i_slot = var_cbbtbotd_i;
        *var_cbbtgatd_i_slot = var_cbbtgatd_i;
        *var_cbbtstid_i_slot = var_cbbtstid_i;
        *var_csrhbotd_i_slot = var_csrhbotd_i;
        *var_csrhgatd_i_slot = var_csrhgatd_i;
        *var_csrhstid_i_slot = var_csrhstid_i;
        *var_ctatbotd_i_slot = var_ctatbotd_i;
        *var_ctatgatd_i_slot = var_ctatgatd_i;
        *var_ctatstid_i_slot = var_ctatstid_i;
        *var_fbbtrbotd_i_slot = var_fbbtrbotd_i;
        *var_fbbtrgatd_i_slot = var_fbbtrgatd_i;
        *var_fbbtrstid_i_slot = var_fbbtrstid_i;
        *var_fcjorgat2d_i_slot = var_fcjorgat2d_i;
        *var_fjunqd_i_slot = var_fjunqd_i;
        *var_fpgat2d_i_slot = var_fpgat2d_i;
        *var_fphiggat2d_i_slot = var_fphiggat2d_i;
        *var_fstopbot_d_slot = var_fstopbot_d;
        *var_fstopgat_d_slot = var_fstopgat_d;
        *var_fstopsti_d_slot = var_fstopsti_d;
        *var_fvbirgat2d_i_slot = var_fvbirgat2d_i;
        *var_idsatrbotd_i_slot = var_idsatrbotd_i;
        *var_idsatrgatd_i_slot = var_idsatrgatd_i;
        *var_idsatrstid_i_slot = var_idsatrstid_i;
        *var_mefftatbotd_i_slot = var_mefftatbotd_i;
        *var_mefftatgatd_i_slot = var_mefftatgatd_i;
        *var_mefftatstid_i_slot = var_mefftatstid_i;
        *var_one_minus_pbot_d_slot = var_one_minus_pbot_d;
        *var_one_minus_pgat_d_slot = var_one_minus_pgat_d;
        *var_one_minus_psti_d_slot = var_one_minus_psti_d;
        *var_one_over_one_minus_pbot_d_slot = var_one_over_one_minus_pbot_d;
        *var_one_over_one_minus_pgat_d_slot = var_one_over_one_minus_pgat_d;
        *var_one_over_one_minus_psti_d_slot = var_one_over_one_minus_psti_d;
        *var_pbotd_i_slot = var_pbotd_i;
        *var_pbrbotd_i_slot = var_pbrbotd_i;
        *var_pbrgatd_i_slot = var_pbrgatd_i;
        *var_pbrstid_i_slot = var_pbrstid_i;
        *var_pgatd_i_slot = var_pgatd_i;
        *var_phigbotd_i_slot = var_phigbotd_i;
        *var_phiggatd_i_slot = var_phiggatd_i;
        *var_phigrbot_d_slot = var_phigrbot_d;
        *var_phigrgat_d_slot = var_phigrgat_d;
        *var_phigrsti_d_slot = var_phigrsti_d;
        *var_phigstid_i_slot = var_phigstid_i;
        *var_pstid_i_slot = var_pstid_i;
        *var_stfbbtbotd_i_slot = var_stfbbtbotd_i;
        *var_stfbbtgatd_i_slot = var_stfbbtgatd_i;
        *var_stfbbtstid_i_slot = var_stfbbtstid_i;
        *var_vbirbotinv_d_slot = var_vbirbotinv_d;
        *var_vbirgatd_i_slot = var_vbirgatd_i;
        *var_vbirgatinv_d_slot = var_vbirgatinv_d;
        *var_vbirstid_i_slot = var_vbirstid_i;
        *var_vbirstiinv_d_slot = var_vbirstiinv_d;
        *var_vbrbotd_i_slot = var_vbrbotd_i;
        *var_vbrgatd_i_slot = var_vbrgatd_i;
        *var_vbrstid_i_slot = var_vbrstid_i;
        *var_vjunrefd_i_slot = var_vjunrefd_i;
        *var_vtrgatd_i_slot = var_vtrgatd_i;
        *var_wdepnulrbot_d_slot = var_wdepnulrbot_d;
        *var_wdepnulrgat_d_slot = var_wdepnulrgat_d;
        *var_wdepnulrinvbot_d_slot = var_wdepnulrinvbot_d;
        *var_wdepnulrinvgat_d_slot = var_wdepnulrinvgat_d;
        *var_wdepnulrinvsti_d_slot = var_wdepnulrinvsti_d;
        *var_wdepnulrsti_d_slot = var_wdepnulrsti_d;
        *var_xjungatd_i_slot = var_xjungatd_i;
        *var_xjunstid_i_slot = var_xjunstid_i;
    }

    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        var_alphaav: f64,
        var_cjorgatd_i: f64,
        var_deltaphigr: f64,
        var_fcjorgat2d_i: f64,
        var_fpgat2d_i: f64,
        var_fphiggat2d_i: f64,
        var_fstopbot_d: f64,
        var_fstopgat_d: f64,
        var_fstopsti_d: f64,
        var_fvbirgat2d_i: f64,
        var_kbol_over_qele: f64,
        var_one_over_one_minus_pbot: f64,
        var_one_over_one_minus_pgat: f64,
        var_one_over_one_minus_psti: f64,
        var_pbrbotd_i: f64,
        var_pbrgatd_i: f64,
        var_pbrstid_i: f64,
        var_pgatd_i: f64,
        var_phiggatd_i: f64,
        var_phigrbot: f64,
        var_phigrgat: f64,
        var_phigrsti: f64,
        var_phitrinv: f64,
        var_tkr: f64,
        var_tkr_1: f64,
        var_vbirgatd_i: f64,
        var_vbrbotd_i: f64,
        var_vbrgatd_i: f64,
        var_vbrstid_i: f64,
        var_atatbot_slot: &mut f64,
        var_atatgat_slot: &mut f64,
        var_atatsti_slot: &mut f64,
        var_auxt_slot: &mut f64,
        var_btatpartbot_slot: &mut f64,
        var_btatpartsti_slot: &mut f64,
        var_cjobot_slot: &mut f64,
        var_cjogat_slot: &mut f64,
        var_cjorgat2nd_d_slot: &mut f64,
        var_cjosti_slot: &mut f64,
        var_deltaebot_slot: &mut f64,
        var_deltaegat_slot: &mut f64,
        var_deltaesti_slot: &mut f64,
        var_deltaphigd_slot: &mut f64,
        var_ftdbot_slot: &mut f64,
        var_ftdgat_slot: &mut f64,
        var_ftdsti_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_idsatbot_slot: &mut f64,
        var_idsatgat_slot: &mut f64,
        var_idsatsti_slot: &mut f64,
        var_inv_phita_slot: &mut f64,
        var_one_minus_pgat2nd_d_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_d_slot: &mut f64,
        var_pgat2nd_d_slot: &mut f64,
        var_phigdbot_slot: &mut f64,
        var_phigdgat_slot: &mut f64,
        var_phigdsti_slot: &mut f64,
        var_phiggat2nd_d_slot: &mut f64,
        var_phigrgat2nd_d_slot: &mut f64,
        var_phita_slot: &mut f64,
        var_phitd_slot: &mut f64,
        var_phitdinv_slot: &mut f64,
        var_qpref2bot_slot: &mut f64,
        var_qpref2gat_slot: &mut f64,
        var_qpref2sti_slot: &mut f64,
        var_qprefbot_slot: &mut f64,
        var_qprefgat_slot: &mut f64,
        var_qprefsti_slot: &mut f64,
        var_rta_slot: &mut f64,
        var_slopebot_d_slot: &mut f64,
        var_slopegat_d_slot: &mut f64,
        var_slopegat_d_db0_slot: &mut f64,
        var_slopegat_d_db1_slot: &mut f64,
        var_slopegat_d_db10_slot: &mut f64,
        var_slopegat_d_db11_slot: &mut f64,
        var_slopegat_d_db12_slot: &mut f64,
        var_slopegat_d_db13_slot: &mut f64,
        var_slopegat_d_db14_slot: &mut f64,
        var_slopegat_d_db15_slot: &mut f64,
        var_slopegat_d_db16_slot: &mut f64,
        var_slopegat_d_db17_slot: &mut f64,
        var_slopegat_d_db18_slot: &mut f64,
        var_slopegat_d_db19_slot: &mut f64,
        var_slopegat_d_db2_slot: &mut f64,
        var_slopegat_d_db20_slot: &mut f64,
        var_slopegat_d_db21_slot: &mut f64,
        var_slopegat_d_db22_slot: &mut f64,
        var_slopegat_d_db23_slot: &mut f64,
        var_slopegat_d_db24_slot: &mut f64,
        var_slopegat_d_db3_slot: &mut f64,
        var_slopegat_d_db4_slot: &mut f64,
        var_slopegat_d_db5_slot: &mut f64,
        var_slopegat_d_db6_slot: &mut f64,
        var_slopegat_d_db7_slot: &mut f64,
        var_slopegat_d_db8_slot: &mut f64,
        var_slopegat_d_db9_slot: &mut f64,
        var_slopegat_d_dn0_slot: &mut f64,
        var_slopegat_d_dn1_slot: &mut f64,
        var_slopegat_d_dn10_slot: &mut f64,
        var_slopegat_d_dn11_slot: &mut f64,
        var_slopegat_d_dn12_slot: &mut f64,
        var_slopegat_d_dn13_slot: &mut f64,
        var_slopegat_d_dn14_slot: &mut f64,
        var_slopegat_d_dn15_slot: &mut f64,
        var_slopegat_d_dn16_slot: &mut f64,
        var_slopegat_d_dn17_slot: &mut f64,
        var_slopegat_d_dn18_slot: &mut f64,
        var_slopegat_d_dn19_slot: &mut f64,
        var_slopegat_d_dn2_slot: &mut f64,
        var_slopegat_d_dn20_slot: &mut f64,
        var_slopegat_d_dn3_slot: &mut f64,
        var_slopegat_d_dn4_slot: &mut f64,
        var_slopegat_d_dn5_slot: &mut f64,
        var_slopegat_d_dn6_slot: &mut f64,
        var_slopegat_d_dn7_slot: &mut f64,
        var_slopegat_d_dn8_slot: &mut f64,
        var_slopegat_d_dn9_slot: &mut f64,
        var_slopesti_d_slot: &mut f64,
        var_swgat2nd_d_slot: &mut f64,
        var_tka_slot: &mut f64,
        var_tkd_1_slot: &mut f64,
        var_ubibot_slot: &mut f64,
        var_ubigat_slot: &mut f64,
        var_ubisti_slot: &mut f64,
        var_vbibot_slot: &mut f64,
        var_vbigat_slot: &mut f64,
        var_vbiinvbot_slot: &mut f64,
        var_vbiinvgat_slot: &mut f64,
        var_vbiinvsti_slot: &mut f64,
        var_vbirgat2nd_d_slot: &mut f64,
        var_vbisti_slot: &mut f64,
        var_vbrinvbot_d_slot: &mut f64,
        var_vbrinvgat_d_slot: &mut f64,
        var_vbrinvgat_d_db0_slot: &mut f64,
        var_vbrinvgat_d_db1_slot: &mut f64,
        var_vbrinvgat_d_db10_slot: &mut f64,
        var_vbrinvgat_d_db11_slot: &mut f64,
        var_vbrinvgat_d_db12_slot: &mut f64,
        var_vbrinvgat_d_db13_slot: &mut f64,
        var_vbrinvgat_d_db14_slot: &mut f64,
        var_vbrinvgat_d_db15_slot: &mut f64,
        var_vbrinvgat_d_db16_slot: &mut f64,
        var_vbrinvgat_d_db17_slot: &mut f64,
        var_vbrinvgat_d_db18_slot: &mut f64,
        var_vbrinvgat_d_db19_slot: &mut f64,
        var_vbrinvgat_d_db2_slot: &mut f64,
        var_vbrinvgat_d_db20_slot: &mut f64,
        var_vbrinvgat_d_db21_slot: &mut f64,
        var_vbrinvgat_d_db22_slot: &mut f64,
        var_vbrinvgat_d_db23_slot: &mut f64,
        var_vbrinvgat_d_db24_slot: &mut f64,
        var_vbrinvgat_d_db3_slot: &mut f64,
        var_vbrinvgat_d_db4_slot: &mut f64,
        var_vbrinvgat_d_db5_slot: &mut f64,
        var_vbrinvgat_d_db6_slot: &mut f64,
        var_vbrinvgat_d_db7_slot: &mut f64,
        var_vbrinvgat_d_db8_slot: &mut f64,
        var_vbrinvgat_d_db9_slot: &mut f64,
        var_vbrinvgat_d_dn0_slot: &mut f64,
        var_vbrinvgat_d_dn1_slot: &mut f64,
        var_vbrinvgat_d_dn10_slot: &mut f64,
        var_vbrinvgat_d_dn11_slot: &mut f64,
        var_vbrinvgat_d_dn12_slot: &mut f64,
        var_vbrinvgat_d_dn13_slot: &mut f64,
        var_vbrinvgat_d_dn14_slot: &mut f64,
        var_vbrinvgat_d_dn15_slot: &mut f64,
        var_vbrinvgat_d_dn16_slot: &mut f64,
        var_vbrinvgat_d_dn17_slot: &mut f64,
        var_vbrinvgat_d_dn18_slot: &mut f64,
        var_vbrinvgat_d_dn19_slot: &mut f64,
        var_vbrinvgat_d_dn2_slot: &mut f64,
        var_vbrinvgat_d_dn20_slot: &mut f64,
        var_vbrinvgat_d_dn3_slot: &mut f64,
        var_vbrinvgat_d_dn4_slot: &mut f64,
        var_vbrinvgat_d_dn5_slot: &mut f64,
        var_vbrinvgat_d_dn6_slot: &mut f64,
        var_vbrinvgat_d_dn7_slot: &mut f64,
        var_vbrinvgat_d_dn8_slot: &mut f64,
        var_vbrinvgat_d_dn9_slot: &mut f64,
        var_vbrinvsti_d_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_atatbot: f64 = *var_atatbot_slot;
        let mut var_atatgat: f64 = *var_atatgat_slot;
        let mut var_atatsti: f64 = *var_atatsti_slot;
        let mut var_auxt: f64 = *var_auxt_slot;
        let mut var_btatpartbot: f64 = *var_btatpartbot_slot;
        let mut var_btatpartsti: f64 = *var_btatpartsti_slot;
        let mut var_cjobot: f64 = *var_cjobot_slot;
        let mut var_cjogat: f64 = *var_cjogat_slot;
        let mut var_cjorgat2nd_d: f64 = *var_cjorgat2nd_d_slot;
        let mut var_cjosti: f64 = *var_cjosti_slot;
        let mut var_deltaebot: f64 = *var_deltaebot_slot;
        let mut var_deltaegat: f64 = *var_deltaegat_slot;
        let mut var_deltaesti: f64 = *var_deltaesti_slot;
        let mut var_deltaphigd: f64 = *var_deltaphigd_slot;
        let mut var_ftdbot: f64 = *var_ftdbot_slot;
        let mut var_ftdgat: f64 = *var_ftdgat_slot;
        let mut var_ftdsti: f64 = *var_ftdsti_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_idsatbot: f64 = *var_idsatbot_slot;
        let mut var_idsatgat: f64 = *var_idsatgat_slot;
        let mut var_idsatsti: f64 = *var_idsatsti_slot;
        let mut var_inv_phita: f64 = *var_inv_phita_slot;
        let mut var_one_minus_pgat2nd_d: f64 = *var_one_minus_pgat2nd_d_slot;
        let mut var_one_over_one_minus_pgat2nd_d: f64 = *var_one_over_one_minus_pgat2nd_d_slot;
        let mut var_pgat2nd_d: f64 = *var_pgat2nd_d_slot;
        let mut var_phigdbot: f64 = *var_phigdbot_slot;
        let mut var_phigdgat: f64 = *var_phigdgat_slot;
        let mut var_phigdsti: f64 = *var_phigdsti_slot;
        let mut var_phiggat2nd_d: f64 = *var_phiggat2nd_d_slot;
        let mut var_phigrgat2nd_d: f64 = *var_phigrgat2nd_d_slot;
        let mut var_phita: f64 = *var_phita_slot;
        let mut var_phitd: f64 = *var_phitd_slot;
        let mut var_phitdinv: f64 = *var_phitdinv_slot;
        let mut var_qpref2bot: f64 = *var_qpref2bot_slot;
        let mut var_qpref2gat: f64 = *var_qpref2gat_slot;
        let mut var_qpref2sti: f64 = *var_qpref2sti_slot;
        let mut var_qprefbot: f64 = *var_qprefbot_slot;
        let mut var_qprefgat: f64 = *var_qprefgat_slot;
        let mut var_qprefsti: f64 = *var_qprefsti_slot;
        let mut var_rta: f64 = *var_rta_slot;
        let mut var_slopebot_d: f64 = *var_slopebot_d_slot;
        let mut var_slopegat_d: f64 = *var_slopegat_d_slot;
        let mut var_slopegat_d_db0: f64 = *var_slopegat_d_db0_slot;
        let mut var_slopegat_d_db1: f64 = *var_slopegat_d_db1_slot;
        let mut var_slopegat_d_db10: f64 = *var_slopegat_d_db10_slot;
        let mut var_slopegat_d_db11: f64 = *var_slopegat_d_db11_slot;
        let mut var_slopegat_d_db12: f64 = *var_slopegat_d_db12_slot;
        let mut var_slopegat_d_db13: f64 = *var_slopegat_d_db13_slot;
        let mut var_slopegat_d_db14: f64 = *var_slopegat_d_db14_slot;
        let mut var_slopegat_d_db15: f64 = *var_slopegat_d_db15_slot;
        let mut var_slopegat_d_db16: f64 = *var_slopegat_d_db16_slot;
        let mut var_slopegat_d_db17: f64 = *var_slopegat_d_db17_slot;
        let mut var_slopegat_d_db18: f64 = *var_slopegat_d_db18_slot;
        let mut var_slopegat_d_db19: f64 = *var_slopegat_d_db19_slot;
        let mut var_slopegat_d_db2: f64 = *var_slopegat_d_db2_slot;
        let mut var_slopegat_d_db20: f64 = *var_slopegat_d_db20_slot;
        let mut var_slopegat_d_db21: f64 = *var_slopegat_d_db21_slot;
        let mut var_slopegat_d_db22: f64 = *var_slopegat_d_db22_slot;
        let mut var_slopegat_d_db23: f64 = *var_slopegat_d_db23_slot;
        let mut var_slopegat_d_db24: f64 = *var_slopegat_d_db24_slot;
        let mut var_slopegat_d_db3: f64 = *var_slopegat_d_db3_slot;
        let mut var_slopegat_d_db4: f64 = *var_slopegat_d_db4_slot;
        let mut var_slopegat_d_db5: f64 = *var_slopegat_d_db5_slot;
        let mut var_slopegat_d_db6: f64 = *var_slopegat_d_db6_slot;
        let mut var_slopegat_d_db7: f64 = *var_slopegat_d_db7_slot;
        let mut var_slopegat_d_db8: f64 = *var_slopegat_d_db8_slot;
        let mut var_slopegat_d_db9: f64 = *var_slopegat_d_db9_slot;
        let mut var_slopegat_d_dn0: f64 = *var_slopegat_d_dn0_slot;
        let mut var_slopegat_d_dn1: f64 = *var_slopegat_d_dn1_slot;
        let mut var_slopegat_d_dn10: f64 = *var_slopegat_d_dn10_slot;
        let mut var_slopegat_d_dn11: f64 = *var_slopegat_d_dn11_slot;
        let mut var_slopegat_d_dn12: f64 = *var_slopegat_d_dn12_slot;
        let mut var_slopegat_d_dn13: f64 = *var_slopegat_d_dn13_slot;
        let mut var_slopegat_d_dn14: f64 = *var_slopegat_d_dn14_slot;
        let mut var_slopegat_d_dn15: f64 = *var_slopegat_d_dn15_slot;
        let mut var_slopegat_d_dn16: f64 = *var_slopegat_d_dn16_slot;
        let mut var_slopegat_d_dn17: f64 = *var_slopegat_d_dn17_slot;
        let mut var_slopegat_d_dn18: f64 = *var_slopegat_d_dn18_slot;
        let mut var_slopegat_d_dn19: f64 = *var_slopegat_d_dn19_slot;
        let mut var_slopegat_d_dn2: f64 = *var_slopegat_d_dn2_slot;
        let mut var_slopegat_d_dn20: f64 = *var_slopegat_d_dn20_slot;
        let mut var_slopegat_d_dn3: f64 = *var_slopegat_d_dn3_slot;
        let mut var_slopegat_d_dn4: f64 = *var_slopegat_d_dn4_slot;
        let mut var_slopegat_d_dn5: f64 = *var_slopegat_d_dn5_slot;
        let mut var_slopegat_d_dn6: f64 = *var_slopegat_d_dn6_slot;
        let mut var_slopegat_d_dn7: f64 = *var_slopegat_d_dn7_slot;
        let mut var_slopegat_d_dn8: f64 = *var_slopegat_d_dn8_slot;
        let mut var_slopegat_d_dn9: f64 = *var_slopegat_d_dn9_slot;
        let mut var_slopesti_d: f64 = *var_slopesti_d_slot;
        let mut var_swgat2nd_d: f64 = *var_swgat2nd_d_slot;
        let mut var_tka: f64 = *var_tka_slot;
        let mut var_tkd_1: f64 = *var_tkd_1_slot;
        let mut var_ubibot: f64 = *var_ubibot_slot;
        let mut var_ubigat: f64 = *var_ubigat_slot;
        let mut var_ubisti: f64 = *var_ubisti_slot;
        let mut var_vbibot: f64 = *var_vbibot_slot;
        let mut var_vbigat: f64 = *var_vbigat_slot;
        let mut var_vbiinvbot: f64 = *var_vbiinvbot_slot;
        let mut var_vbiinvgat: f64 = *var_vbiinvgat_slot;
        let mut var_vbiinvsti: f64 = *var_vbiinvsti_slot;
        let mut var_vbirgat2nd_d: f64 = *var_vbirgat2nd_d_slot;
        let mut var_vbisti: f64 = *var_vbisti_slot;
        let mut var_vbrinvbot_d: f64 = *var_vbrinvbot_d_slot;
        let mut var_vbrinvgat_d: f64 = *var_vbrinvgat_d_slot;
        let mut var_vbrinvgat_d_db0: f64 = *var_vbrinvgat_d_db0_slot;
        let mut var_vbrinvgat_d_db1: f64 = *var_vbrinvgat_d_db1_slot;
        let mut var_vbrinvgat_d_db10: f64 = *var_vbrinvgat_d_db10_slot;
        let mut var_vbrinvgat_d_db11: f64 = *var_vbrinvgat_d_db11_slot;
        let mut var_vbrinvgat_d_db12: f64 = *var_vbrinvgat_d_db12_slot;
        let mut var_vbrinvgat_d_db13: f64 = *var_vbrinvgat_d_db13_slot;
        let mut var_vbrinvgat_d_db14: f64 = *var_vbrinvgat_d_db14_slot;
        let mut var_vbrinvgat_d_db15: f64 = *var_vbrinvgat_d_db15_slot;
        let mut var_vbrinvgat_d_db16: f64 = *var_vbrinvgat_d_db16_slot;
        let mut var_vbrinvgat_d_db17: f64 = *var_vbrinvgat_d_db17_slot;
        let mut var_vbrinvgat_d_db18: f64 = *var_vbrinvgat_d_db18_slot;
        let mut var_vbrinvgat_d_db19: f64 = *var_vbrinvgat_d_db19_slot;
        let mut var_vbrinvgat_d_db2: f64 = *var_vbrinvgat_d_db2_slot;
        let mut var_vbrinvgat_d_db20: f64 = *var_vbrinvgat_d_db20_slot;
        let mut var_vbrinvgat_d_db21: f64 = *var_vbrinvgat_d_db21_slot;
        let mut var_vbrinvgat_d_db22: f64 = *var_vbrinvgat_d_db22_slot;
        let mut var_vbrinvgat_d_db23: f64 = *var_vbrinvgat_d_db23_slot;
        let mut var_vbrinvgat_d_db24: f64 = *var_vbrinvgat_d_db24_slot;
        let mut var_vbrinvgat_d_db3: f64 = *var_vbrinvgat_d_db3_slot;
        let mut var_vbrinvgat_d_db4: f64 = *var_vbrinvgat_d_db4_slot;
        let mut var_vbrinvgat_d_db5: f64 = *var_vbrinvgat_d_db5_slot;
        let mut var_vbrinvgat_d_db6: f64 = *var_vbrinvgat_d_db6_slot;
        let mut var_vbrinvgat_d_db7: f64 = *var_vbrinvgat_d_db7_slot;
        let mut var_vbrinvgat_d_db8: f64 = *var_vbrinvgat_d_db8_slot;
        let mut var_vbrinvgat_d_db9: f64 = *var_vbrinvgat_d_db9_slot;
        let mut var_vbrinvgat_d_dn0: f64 = *var_vbrinvgat_d_dn0_slot;
        let mut var_vbrinvgat_d_dn1: f64 = *var_vbrinvgat_d_dn1_slot;
        let mut var_vbrinvgat_d_dn10: f64 = *var_vbrinvgat_d_dn10_slot;
        let mut var_vbrinvgat_d_dn11: f64 = *var_vbrinvgat_d_dn11_slot;
        let mut var_vbrinvgat_d_dn12: f64 = *var_vbrinvgat_d_dn12_slot;
        let mut var_vbrinvgat_d_dn13: f64 = *var_vbrinvgat_d_dn13_slot;
        let mut var_vbrinvgat_d_dn14: f64 = *var_vbrinvgat_d_dn14_slot;
        let mut var_vbrinvgat_d_dn15: f64 = *var_vbrinvgat_d_dn15_slot;
        let mut var_vbrinvgat_d_dn16: f64 = *var_vbrinvgat_d_dn16_slot;
        let mut var_vbrinvgat_d_dn17: f64 = *var_vbrinvgat_d_dn17_slot;
        let mut var_vbrinvgat_d_dn18: f64 = *var_vbrinvgat_d_dn18_slot;
        let mut var_vbrinvgat_d_dn19: f64 = *var_vbrinvgat_d_dn19_slot;
        let mut var_vbrinvgat_d_dn2: f64 = *var_vbrinvgat_d_dn2_slot;
        let mut var_vbrinvgat_d_dn20: f64 = *var_vbrinvgat_d_dn20_slot;
        let mut var_vbrinvgat_d_dn3: f64 = *var_vbrinvgat_d_dn3_slot;
        let mut var_vbrinvgat_d_dn4: f64 = *var_vbrinvgat_d_dn4_slot;
        let mut var_vbrinvgat_d_dn5: f64 = *var_vbrinvgat_d_dn5_slot;
        let mut var_vbrinvgat_d_dn6: f64 = *var_vbrinvgat_d_dn6_slot;
        let mut var_vbrinvgat_d_dn7: f64 = *var_vbrinvgat_d_dn7_slot;
        let mut var_vbrinvgat_d_dn8: f64 = *var_vbrinvgat_d_dn8_slot;
        let mut var_vbrinvgat_d_dn9: f64 = *var_vbrinvgat_d_dn9_slot;
        let mut var_vbrinvsti_d: f64 = *var_vbrinvsti_d_slot;

        let assign1980_e2530: f64 = (1.0 / var_vbrbotd_i);
        var_vbrinvbot_d = assign1980_e2530;

        let assign1990_e2533: f64 = (1.0 / var_vbrstid_i);
        var_vbrinvsti_d = assign1990_e2533;

        let assign2000_e2536: f64 = (1.0 / var_vbrgatd_i);
        var_vbrinvgat_d = assign2000_e2536;
        var_vbrinvgat_d_dn0 = 0.0;
        var_vbrinvgat_d_dn1 = 0.0;
        var_vbrinvgat_d_dn2 = 0.0;
        var_vbrinvgat_d_dn3 = 0.0;
        var_vbrinvgat_d_dn4 = 0.0;
        var_vbrinvgat_d_dn5 = 0.0;
        var_vbrinvgat_d_dn6 = 0.0;
        var_vbrinvgat_d_dn7 = 0.0;
        var_vbrinvgat_d_dn8 = 0.0;
        var_vbrinvgat_d_dn9 = 0.0;
        var_vbrinvgat_d_dn10 = 0.0;
        var_vbrinvgat_d_dn11 = 0.0;
        var_vbrinvgat_d_dn12 = 0.0;
        var_vbrinvgat_d_dn13 = 0.0;
        var_vbrinvgat_d_dn14 = 0.0;
        var_vbrinvgat_d_dn15 = 0.0;
        var_vbrinvgat_d_dn16 = 0.0;
        var_vbrinvgat_d_dn17 = 0.0;
        var_vbrinvgat_d_dn18 = 0.0;
        var_vbrinvgat_d_dn19 = 0.0;
        var_vbrinvgat_d_dn20 = 0.0;
        var_vbrinvgat_d_db0 = 0.0;
        var_vbrinvgat_d_db1 = 0.0;
        var_vbrinvgat_d_db2 = 0.0;
        var_vbrinvgat_d_db3 = 0.0;
        var_vbrinvgat_d_db4 = 0.0;
        var_vbrinvgat_d_db5 = 0.0;
        var_vbrinvgat_d_db6 = 0.0;
        var_vbrinvgat_d_db7 = 0.0;
        var_vbrinvgat_d_db8 = 0.0;
        var_vbrinvgat_d_db9 = 0.0;
        var_vbrinvgat_d_db10 = 0.0;
        var_vbrinvgat_d_db11 = 0.0;
        var_vbrinvgat_d_db12 = 0.0;
        var_vbrinvgat_d_db13 = 0.0;
        var_vbrinvgat_d_db14 = 0.0;
        var_vbrinvgat_d_db15 = 0.0;
        var_vbrinvgat_d_db16 = 0.0;
        var_vbrinvgat_d_db17 = 0.0;
        var_vbrinvgat_d_db18 = 0.0;
        var_vbrinvgat_d_db19 = 0.0;
        var_vbrinvgat_d_db20 = 0.0;
        var_vbrinvgat_d_db21 = 0.0;
        var_vbrinvgat_d_db22 = 0.0;
        var_vbrinvgat_d_db23 = 0.0;
        var_vbrinvgat_d_db24 = 0.0;

        let assign2010_e2539: f64 = (var_fstopbot_d * var_fstopbot_d);
        let assign2010_e2543: f64 = (var_pbrbotd_i - 1.0);
        let assign2010_e2544: f64 = (var_alphaav).powf(assign2010_e2543);
        let assign2010_e2545: f64 = (assign2010_e2539 * assign2010_e2544);
        let assign2010_e2546: f64 = (-assign2010_e2545);
        let assign2010_e2548: f64 = (assign2010_e2546 * var_pbrbotd_i);
        let assign2010_e2550: f64 = (assign2010_e2548 * var_vbrinvbot_d);
        var_slopebot_d = assign2010_e2550;

        let assign2020_e2553: f64 = (var_fstopsti_d * var_fstopsti_d);
        let assign2020_e2557: f64 = (var_pbrstid_i - 1.0);
        let assign2020_e2558: f64 = (var_alphaav).powf(assign2020_e2557);
        let assign2020_e2559: f64 = (assign2020_e2553 * assign2020_e2558);
        let assign2020_e2560: f64 = (-assign2020_e2559);
        let assign2020_e2562: f64 = (assign2020_e2560 * var_pbrstid_i);
        let assign2020_e2564: f64 = (assign2020_e2562 * var_vbrinvsti_d);
        var_slopesti_d = assign2020_e2564;

        let assign2030_e2567: f64 = (var_fstopgat_d * var_fstopgat_d);
        let assign2030_e2571: f64 = (var_pbrgatd_i - 1.0);
        let assign2030_e2572: f64 = (var_alphaav).powf(assign2030_e2571);
        let assign2030_e2573: f64 = (assign2030_e2567 * assign2030_e2572);
        let assign2030_e2574: f64 = (-assign2030_e2573);
        let assign2030_e2576: f64 = (assign2030_e2574 * var_pbrgatd_i);
        let assign2030_e2578: f64 = (assign2030_e2576 * var_vbrinvgat_d);
        var_slopegat_d = assign2030_e2578;
        var_slopegat_d_dn0 = (assign2030_e2576 * var_vbrinvgat_d_dn0);
        var_slopegat_d_dn1 = (assign2030_e2576 * var_vbrinvgat_d_dn1);
        var_slopegat_d_dn2 = (assign2030_e2576 * var_vbrinvgat_d_dn2);
        var_slopegat_d_dn3 = (assign2030_e2576 * var_vbrinvgat_d_dn3);
        var_slopegat_d_dn4 = (assign2030_e2576 * var_vbrinvgat_d_dn4);
        var_slopegat_d_dn5 = (assign2030_e2576 * var_vbrinvgat_d_dn5);
        var_slopegat_d_dn6 = (assign2030_e2576 * var_vbrinvgat_d_dn6);
        var_slopegat_d_dn7 = (assign2030_e2576 * var_vbrinvgat_d_dn7);
        var_slopegat_d_dn8 = (assign2030_e2576 * var_vbrinvgat_d_dn8);
        var_slopegat_d_dn9 = (assign2030_e2576 * var_vbrinvgat_d_dn9);
        var_slopegat_d_dn10 = (assign2030_e2576 * var_vbrinvgat_d_dn10);
        var_slopegat_d_dn11 = (assign2030_e2576 * var_vbrinvgat_d_dn11);
        var_slopegat_d_dn12 = (assign2030_e2576 * var_vbrinvgat_d_dn12);
        var_slopegat_d_dn13 = (assign2030_e2576 * var_vbrinvgat_d_dn13);
        var_slopegat_d_dn14 = (assign2030_e2576 * var_vbrinvgat_d_dn14);
        var_slopegat_d_dn15 = (assign2030_e2576 * var_vbrinvgat_d_dn15);
        var_slopegat_d_dn16 = (assign2030_e2576 * var_vbrinvgat_d_dn16);
        var_slopegat_d_dn17 = (assign2030_e2576 * var_vbrinvgat_d_dn17);
        var_slopegat_d_dn18 = (assign2030_e2576 * var_vbrinvgat_d_dn18);
        var_slopegat_d_dn19 = (assign2030_e2576 * var_vbrinvgat_d_dn19);
        var_slopegat_d_dn20 = (assign2030_e2576 * var_vbrinvgat_d_dn20);
        var_slopegat_d_db0 = (assign2030_e2576 * var_vbrinvgat_d_db0);
        var_slopegat_d_db1 = (assign2030_e2576 * var_vbrinvgat_d_db1);
        var_slopegat_d_db2 = (assign2030_e2576 * var_vbrinvgat_d_db2);
        var_slopegat_d_db3 = (assign2030_e2576 * var_vbrinvgat_d_db3);
        var_slopegat_d_db4 = (assign2030_e2576 * var_vbrinvgat_d_db4);
        var_slopegat_d_db5 = (assign2030_e2576 * var_vbrinvgat_d_db5);
        var_slopegat_d_db6 = (assign2030_e2576 * var_vbrinvgat_d_db6);
        var_slopegat_d_db7 = (assign2030_e2576 * var_vbrinvgat_d_db7);
        var_slopegat_d_db8 = (assign2030_e2576 * var_vbrinvgat_d_db8);
        var_slopegat_d_db9 = (assign2030_e2576 * var_vbrinvgat_d_db9);
        var_slopegat_d_db10 = (assign2030_e2576 * var_vbrinvgat_d_db10);
        var_slopegat_d_db11 = (assign2030_e2576 * var_vbrinvgat_d_db11);
        var_slopegat_d_db12 = (assign2030_e2576 * var_vbrinvgat_d_db12);
        var_slopegat_d_db13 = (assign2030_e2576 * var_vbrinvgat_d_db13);
        var_slopegat_d_db14 = (assign2030_e2576 * var_vbrinvgat_d_db14);
        var_slopegat_d_db15 = (assign2030_e2576 * var_vbrinvgat_d_db15);
        var_slopegat_d_db16 = (assign2030_e2576 * var_vbrinvgat_d_db16);
        var_slopegat_d_db17 = (assign2030_e2576 * var_vbrinvgat_d_db17);
        var_slopegat_d_db18 = (assign2030_e2576 * var_vbrinvgat_d_db18);
        var_slopegat_d_db19 = (assign2030_e2576 * var_vbrinvgat_d_db19);
        var_slopegat_d_db20 = (assign2030_e2576 * var_vbrinvgat_d_db20);
        var_slopegat_d_db21 = (assign2030_e2576 * var_vbrinvgat_d_db21);
        var_slopegat_d_db22 = (assign2030_e2576 * var_vbrinvgat_d_db22);
        var_slopegat_d_db23 = (assign2030_e2576 * var_vbrinvgat_d_db23);
        var_slopegat_d_db24 = (assign2030_e2576 * var_vbrinvgat_d_db24);

        let assign2040_e2593: f64 = if ((((var_fcjorgat2d_i != 1.0) || (var_fvbirgat2d_i != 1.0)) || (var_fpgat2d_i != 1.0)) || (var_fphiggat2d_i != 1.0)) { 1.0 } else { 0.0 };
        var_guard11 = assign2040_e2593;

        let (assign2050_e2597,) = {
    if (var_guard11 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign2050_e2597;

        let (assign2060_e2602,) = {
    if (var_guard11 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign2060_e2602;

        let assign2070_e2605: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard12 = assign2070_e2605;

        let (assign2080_e2618,) = {
    if (var_guard12 != 0.0) {
        let assign2080_e2609: f64 = (var_cjorgatd_i * var_fcjorgat2d_i);
        let (assign2080_e2616,) = {
            if (assign2080_e2609 > 1e-18) {
                let assign2080_e2614: f64 = (var_cjorgatd_i * var_fcjorgat2d_i);
                (assign2080_e2614,)
            } else {
                (1e-18,)
            }
        };
        (assign2080_e2616,)
    } else {
        (var_cjorgat2nd_d,)
    }
};
        var_cjorgat2nd_d = assign2080_e2618;

        let (assign2090_e2631,) = {
    if (var_guard12 != 0.0) {
        let assign2090_e2622: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
        let (assign2090_e2629,) = {
            if (assign2090_e2622 > 0.05) {
                let assign2090_e2627: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
                (assign2090_e2627,)
            } else {
                (0.05,)
            }
        };
        (assign2090_e2629,)
    } else {
        (var_vbirgat2nd_d,)
    }
};
        var_vbirgat2nd_d = assign2090_e2631;

        let (assign2100_e2658,) = {
    if (var_guard12 != 0.0) {
        let assign2100_e2635: f64 = (var_pgatd_i * var_fpgat2d_i);
        let (assign2100_e2642,) = {
            if (assign2100_e2635 > 0.05) {
                let assign2100_e2640: f64 = (var_pgatd_i * var_fpgat2d_i);
                (assign2100_e2640,)
            } else {
                (0.05,)
            }
        };
        let (assign2100_e2656,) = {
            if (assign2100_e2642 < 0.95) {
                let assign2100_e2647: f64 = (var_pgatd_i * var_fpgat2d_i);
                let (assign2100_e2654,) = {
                    if (assign2100_e2647 > 0.05) {
                        let assign2100_e2652: f64 = (var_pgatd_i * var_fpgat2d_i);
                        (assign2100_e2652,)
                    } else {
                        (0.05,)
                    }
                };
                (assign2100_e2654,)
            } else {
                (0.95,)
            }
        };
        (assign2100_e2656,)
    } else {
        (var_pgat2nd_d,)
    }
};
        var_pgat2nd_d = assign2100_e2658;

        let (assign2110_e2664,) = {
    if (var_guard12 != 0.0) {
        let assign2110_e2662: f64 = (var_phiggatd_i * var_fphiggat2d_i);
        (assign2110_e2662,)
    } else {
        (var_phiggat2nd_d,)
    }
};
        var_phiggat2nd_d = assign2110_e2664;

        let (assign2120_e2670,) = {
    if (var_guard12 != 0.0) {
        let assign2120_e2668: f64 = (var_phiggat2nd_d + var_deltaphigr);
        (assign2120_e2668,)
    } else {
        (var_phigrgat2nd_d,)
    }
};
        var_phigrgat2nd_d = assign2120_e2670;

        let (assign2130_e2676,) = {
    if (var_guard12 != 0.0) {
        let assign2130_e2674: f64 = (1.0 - var_pgat2nd_d);
        (assign2130_e2674,)
    } else {
        (var_one_minus_pgat2nd_d,)
    }
};
        var_one_minus_pgat2nd_d = assign2130_e2676;

        let (assign2140_e2682,) = {
    if (var_guard12 != 0.0) {
        let assign2140_e2680: f64 = (1.0 / var_one_minus_pgat2nd_d);
        (assign2140_e2680,)
    } else {
        (var_one_over_one_minus_pgat2nd_d,)
    }
};
        var_one_over_one_minus_pgat2nd_d = assign2140_e2682;

        let assign2190_e2704: f64 = ctx_temp;
        let assign2190_e2706: f64 = (assign2190_e2704 + p.p56);
        let assign2190_e2708: f64 = (assign2190_e2706 + p.p35);
        var_tka = assign2190_e2708;

        let assign2200_e2711: f64 = (var_tka / var_tkr);
        var_rta = assign2200_e2711;

        s.store_scalar(353, (var_tka - var_tkr));

        let assign2220_e2717: f64 = (var_tka * 1.3806505e-23);
        let assign2220_e2719: f64 = (assign2220_e2717 / 1.6021918e-19);
        var_phita = assign2220_e2719;

        let assign2230_e2722: f64 = (1.0 / var_phita);
        var_inv_phita = assign2230_e2722;

        s.store_scalar(356, var_tka);

        s.store_scalar(357, (s.v[356] * s.v[356]));

        s.store_scalar(358, (s.v[356] - var_tkr));

        s.store_scalar(359, (var_tkr / s.v[356]));

        s.store_scalar(360, ((s.v[359]) as f64).ln());

        s.store_scalar(715, ((s.v[356] * 1.3806505e-23) / 1.6021918e-19));

        s.store_scalar(361, (1.0 / s.v[715]));

        s.store_scalar(362, ((1.179 - (9.025e-5 * s.v[356])) - (3.05e-7 * s.v[357])));

        s.store_scalar(363, ((((1.045 + (0.00045 * s.v[356])) * ((0.523 + (0.0014 * s.v[356])) - (1.48e-6 * s.v[357]))) * s.v[357]) / 90000.0));

        if (!(s.v[363] > 0.001)) {
            s.store_scalar(363, 0.001);
        }

        s.store_scalar(718, ((4.0 * 1.3806505e-23) * s.v[356]));

        let assign2350_e2782: f64 = ctx_temp;
        let assign2350_e2784: f64 = (assign2350_e2782 + p.p56);
        let assign2350_e2786: f64 = (assign2350_e2784 + p.p35);
        let assign2350_e2789: f64 = (-250.0);
        let assign2350_e2790: f64 = (273.15 + assign2350_e2789);
        let assign2350_e2791: f64 = (assign2350_e2786).max(assign2350_e2790);
        var_tkd_1 = assign2350_e2791;

        let assign2360_e2794: f64 = (var_tkd_1 / var_tkr_1);
        var_auxt = assign2360_e2794;

        let assign2370_e2797: f64 = (var_kbol_over_qele * var_tkd_1);
        var_phitd = assign2370_e2797;

        let assign2380_e2800: f64 = (1.0 / var_phitd);
        var_phitdinv = assign2380_e2800;

        let assign2390_e2803: f64 = (0.000702 * var_tkd_1);
        let assign2390_e2805: f64 = (assign2390_e2803 * var_tkd_1);
        let assign2390_e2806: f64 = (-assign2390_e2805);
        let assign2390_e2809: f64 = (1108.0 + var_tkd_1);
        let assign2390_e2810: f64 = (assign2390_e2806 / assign2390_e2809);
        var_deltaphigd = assign2390_e2810;

        let assign2400_e2813: f64 = (p.p834 + var_deltaphigd);
        var_phigdbot = assign2400_e2813;

        let assign2410_e2816: f64 = (p.p835 + var_deltaphigd);
        var_phigdsti = assign2410_e2816;

        let assign2420_e2819: f64 = (p.p836 + var_deltaphigd);
        var_phigdgat = assign2420_e2819;

        let assign2430_e2822: f64 = (var_auxt).powf(1.5);
        let assign2430_e2826: f64 = (var_phigrbot * var_phitrinv);
        let assign2430_e2829: f64 = (var_phigdbot * var_phitdinv);
        let assign2430_e2830: f64 = (assign2430_e2826 - assign2430_e2829);
        let assign2430_e2831: f64 = (0.5 * assign2430_e2830);
        let assign2430_e2832: f64 = (assign2430_e2831).exp();
        let assign2430_e2833: f64 = (assign2430_e2822 * assign2430_e2832);
        var_ftdbot = assign2430_e2833;

        let assign2440_e2836: f64 = (var_auxt).powf(1.5);
        let assign2440_e2840: f64 = (var_phigrsti * var_phitrinv);
        let assign2440_e2843: f64 = (var_phigdsti * var_phitdinv);
        let assign2440_e2844: f64 = (assign2440_e2840 - assign2440_e2843);
        let assign2440_e2845: f64 = (0.5 * assign2440_e2844);
        let assign2440_e2846: f64 = (assign2440_e2845).exp();
        let assign2440_e2847: f64 = (assign2440_e2836 * assign2440_e2846);
        var_ftdsti = assign2440_e2847;

        let assign2450_e2850: f64 = (var_auxt).powf(1.5);
        let assign2450_e2854: f64 = (var_phigrgat * var_phitrinv);
        let assign2450_e2857: f64 = (var_phigdgat * var_phitdinv);
        let assign2450_e2858: f64 = (assign2450_e2854 - assign2450_e2857);
        let assign2450_e2859: f64 = (0.5 * assign2450_e2858);
        let assign2450_e2860: f64 = (assign2450_e2859).exp();
        let assign2450_e2861: f64 = (assign2450_e2850 * assign2450_e2860);
        var_ftdgat = assign2450_e2861;

        let assign2460_e2864: f64 = (p.p837 * var_ftdbot);
        let assign2460_e2866: f64 = (assign2460_e2864 * var_ftdbot);
        var_idsatbot = assign2460_e2866;

        let assign2470_e2869: f64 = (p.p838 * var_ftdsti);
        let assign2470_e2871: f64 = (assign2470_e2869 * var_ftdsti);
        var_idsatsti = assign2470_e2871;

        let assign2480_e2874: f64 = (p.p839 * var_ftdgat);
        let assign2480_e2876: f64 = (assign2480_e2874 * var_ftdgat);
        var_idsatgat = assign2480_e2876;

        let assign2490_e2879: f64 = (p.p828 * var_auxt);
        let assign2490_e2882: f64 = (2.0 * var_phitd);
        let assign2490_e2884: f64 = (var_ftdbot).ln();
        let assign2490_e2885: f64 = (assign2490_e2882 * assign2490_e2884);
        let assign2490_e2886: f64 = (assign2490_e2879 - assign2490_e2885);
        var_ubibot = assign2490_e2886;

        let assign2500_e2889: f64 = (p.p829 * var_auxt);
        let assign2500_e2892: f64 = (2.0 * var_phitd);
        let assign2500_e2894: f64 = (var_ftdsti).ln();
        let assign2500_e2895: f64 = (assign2500_e2892 * assign2500_e2894);
        let assign2500_e2896: f64 = (assign2500_e2889 - assign2500_e2895);
        var_ubisti = assign2500_e2896;

        let assign2510_e2899: f64 = (p.p830 * var_auxt);
        let assign2510_e2902: f64 = (2.0 * var_phitd);
        let assign2510_e2904: f64 = (var_ftdgat).ln();
        let assign2510_e2905: f64 = (assign2510_e2902 * assign2510_e2904);
        let assign2510_e2906: f64 = (assign2510_e2899 - assign2510_e2905);
        var_ubigat = assign2510_e2906;

        let assign2520_e2912: f64 = (0.05 - var_ubibot);
        let assign2520_e2914: f64 = (assign2520_e2912 * var_phitdinv);
        let assign2520_e2915: f64 = (assign2520_e2914).exp();
        let assign2520_e2916: f64 = (1.0 + assign2520_e2915);
        let assign2520_e2917: f64 = (assign2520_e2916).ln();
        let assign2520_e2918: f64 = (var_phitd * assign2520_e2917);
        let assign2520_e2919: f64 = (var_ubibot + assign2520_e2918);
        var_vbibot = assign2520_e2919;

        let assign2530_e2925: f64 = (0.05 - var_ubisti);
        let assign2530_e2927: f64 = (assign2530_e2925 * var_phitdinv);
        let assign2530_e2928: f64 = (assign2530_e2927).exp();
        let assign2530_e2929: f64 = (1.0 + assign2530_e2928);
        let assign2530_e2930: f64 = (assign2530_e2929).ln();
        let assign2530_e2931: f64 = (var_phitd * assign2530_e2930);
        let assign2530_e2932: f64 = (var_ubisti + assign2530_e2931);
        var_vbisti = assign2530_e2932;

        let assign2540_e2938: f64 = (0.05 - var_ubigat);
        let assign2540_e2940: f64 = (assign2540_e2938 * var_phitdinv);
        let assign2540_e2941: f64 = (assign2540_e2940).exp();
        let assign2540_e2942: f64 = (1.0 + assign2540_e2941);
        let assign2540_e2943: f64 = (assign2540_e2942).ln();
        let assign2540_e2944: f64 = (var_phitd * assign2540_e2943);
        let assign2540_e2945: f64 = (var_ubigat + assign2540_e2944);
        var_vbigat = assign2540_e2945;

        let assign2550_e2948: f64 = (1.0 / var_vbibot);
        var_vbiinvbot = assign2550_e2948;

        let assign2560_e2951: f64 = (1.0 / var_vbisti);
        var_vbiinvsti = assign2560_e2951;

        let assign2570_e2954: f64 = (1.0 / var_vbigat);
        var_vbiinvgat = assign2570_e2954;

        let assign2580_e2958: f64 = (p.p828 * var_vbiinvbot);
        let assign2580_e2960: f64 = (assign2580_e2958).powf(p.p831);
        let assign2580_e2961: f64 = (p.p825 * assign2580_e2960);
        var_cjobot = assign2580_e2961;

        let assign2590_e2965: f64 = (p.p829 * var_vbiinvsti);
        let assign2590_e2967: f64 = (assign2590_e2965).powf(p.p832);
        let assign2590_e2968: f64 = (p.p826 * assign2590_e2967);
        var_cjosti = assign2590_e2968;

        let assign2600_e2972: f64 = (p.p830 * var_vbiinvgat);
        let assign2600_e2974: f64 = (assign2600_e2972).powf(p.p833);
        let assign2600_e2975: f64 = (p.p827 * assign2600_e2974);
        var_cjogat = assign2600_e2975;

        let assign2610_e2978: f64 = (var_cjobot * var_vbibot);
        let assign2610_e2980: f64 = (assign2610_e2978 * var_one_over_one_minus_pbot);
        var_qprefbot = assign2610_e2980;

        let assign2620_e2983: f64 = (var_cjosti * var_vbisti);
        let assign2620_e2985: f64 = (assign2620_e2983 * var_one_over_one_minus_psti);
        var_qprefsti = assign2620_e2985;

        let assign2630_e2988: f64 = (var_cjogat * var_vbigat);
        let assign2630_e2990: f64 = (assign2630_e2988 * var_one_over_one_minus_pgat);
        var_qprefgat = assign2630_e2990;

        let assign2640_e2993: f64 = (2.0 * var_cjobot);
        var_qpref2bot = assign2640_e2993;

        let assign2650_e2996: f64 = (2.0 * var_cjosti);
        var_qpref2sti = assign2650_e2996;

        let assign2660_e2999: f64 = (2.0 * var_cjogat);
        var_qpref2gat = assign2660_e2999;

        let assign2670_e3002: f64 = (0.5 * var_phigdbot);
        let assign2670_e3004: f64 = (assign2670_e3002).max(var_phitd);
        var_deltaebot = assign2670_e3004;

        let assign2680_e3007: f64 = (0.5 * var_phigdsti);
        let assign2680_e3009: f64 = (assign2680_e3007).max(var_phitd);
        var_deltaesti = assign2680_e3009;

        let assign2690_e3012: f64 = (0.5 * var_phigdgat);
        let assign2690_e3014: f64 = (assign2690_e3012).max(var_phitd);
        var_deltaegat = assign2690_e3014;

        let assign2700_e3017: f64 = (var_deltaebot * var_phitdinv);
        var_atatbot = assign2700_e3017;

        let assign2710_e3020: f64 = (var_deltaesti * var_phitdinv);
        var_atatsti = assign2710_e3020;

        let assign2720_e3023: f64 = (var_deltaegat * var_phitdinv);
        var_atatgat = assign2720_e3023;

        let assign2730_e3026: f64 = (32.0 * p.p848);
        let assign2730_e3028: f64 = (assign2730_e3026 * 9.1093826e-31);
        let assign2730_e3030: f64 = (assign2730_e3028 * 1.6021918e-19);
        let assign2730_e3033: f64 = (var_deltaebot * var_deltaebot);
        let assign2730_e3035: f64 = (assign2730_e3033 * var_deltaebot);
        let assign2730_e3036: f64 = (assign2730_e3030 * assign2730_e3035);
        let assign2730_e3037: f64 = (assign2730_e3036).sqrt();
        let assign2730_e3040: f64 = (3.0 * 1.05457168e-34);
        let assign2730_e3041: f64 = (assign2730_e3037 / assign2730_e3040);
        var_btatpartbot = assign2730_e3041;

        let assign2740_e3044: f64 = (32.0 * p.p849);
        let assign2740_e3046: f64 = (assign2740_e3044 * 9.1093826e-31);
        let assign2740_e3048: f64 = (assign2740_e3046 * 1.6021918e-19);
        let assign2740_e3051: f64 = (var_deltaesti * var_deltaesti);
        let assign2740_e3053: f64 = (assign2740_e3051 * var_deltaesti);
        let assign2740_e3054: f64 = (assign2740_e3048 * assign2740_e3053);
        let assign2740_e3055: f64 = (assign2740_e3054).sqrt();
        let assign2740_e3058: f64 = (3.0 * 1.05457168e-34);
        let assign2740_e3059: f64 = (assign2740_e3055 / assign2740_e3058);
        var_btatpartsti = assign2740_e3059;

        *var_atatbot_slot = var_atatbot;
        *var_atatgat_slot = var_atatgat;
        *var_atatsti_slot = var_atatsti;
        *var_auxt_slot = var_auxt;
        *var_btatpartbot_slot = var_btatpartbot;
        *var_btatpartsti_slot = var_btatpartsti;
        *var_cjobot_slot = var_cjobot;
        *var_cjogat_slot = var_cjogat;
        *var_cjorgat2nd_d_slot = var_cjorgat2nd_d;
        *var_cjosti_slot = var_cjosti;
        *var_deltaebot_slot = var_deltaebot;
        *var_deltaegat_slot = var_deltaegat;
        *var_deltaesti_slot = var_deltaesti;
        *var_deltaphigd_slot = var_deltaphigd;
        *var_ftdbot_slot = var_ftdbot;
        *var_ftdgat_slot = var_ftdgat;
        *var_ftdsti_slot = var_ftdsti;
        *var_guard11_slot = var_guard11;
        *var_guard12_slot = var_guard12;
        *var_idsatbot_slot = var_idsatbot;
        *var_idsatgat_slot = var_idsatgat;
        *var_idsatsti_slot = var_idsatsti;
        *var_inv_phita_slot = var_inv_phita;
        *var_one_minus_pgat2nd_d_slot = var_one_minus_pgat2nd_d;
        *var_one_over_one_minus_pgat2nd_d_slot = var_one_over_one_minus_pgat2nd_d;
        *var_pgat2nd_d_slot = var_pgat2nd_d;
        *var_phigdbot_slot = var_phigdbot;
        *var_phigdgat_slot = var_phigdgat;
        *var_phigdsti_slot = var_phigdsti;
        *var_phiggat2nd_d_slot = var_phiggat2nd_d;
        *var_phigrgat2nd_d_slot = var_phigrgat2nd_d;
        *var_phita_slot = var_phita;
        *var_phitd_slot = var_phitd;
        *var_phitdinv_slot = var_phitdinv;
        *var_qpref2bot_slot = var_qpref2bot;
        *var_qpref2gat_slot = var_qpref2gat;
        *var_qpref2sti_slot = var_qpref2sti;
        *var_qprefbot_slot = var_qprefbot;
        *var_qprefgat_slot = var_qprefgat;
        *var_qprefsti_slot = var_qprefsti;
        *var_rta_slot = var_rta;
        *var_slopebot_d_slot = var_slopebot_d;
        *var_slopegat_d_slot = var_slopegat_d;
        *var_slopegat_d_db0_slot = var_slopegat_d_db0;
        *var_slopegat_d_db1_slot = var_slopegat_d_db1;
        *var_slopegat_d_db10_slot = var_slopegat_d_db10;
        *var_slopegat_d_db11_slot = var_slopegat_d_db11;
        *var_slopegat_d_db12_slot = var_slopegat_d_db12;
        *var_slopegat_d_db13_slot = var_slopegat_d_db13;
        *var_slopegat_d_db14_slot = var_slopegat_d_db14;
        *var_slopegat_d_db15_slot = var_slopegat_d_db15;
        *var_slopegat_d_db16_slot = var_slopegat_d_db16;
        *var_slopegat_d_db17_slot = var_slopegat_d_db17;
        *var_slopegat_d_db18_slot = var_slopegat_d_db18;
        *var_slopegat_d_db19_slot = var_slopegat_d_db19;
        *var_slopegat_d_db2_slot = var_slopegat_d_db2;
        *var_slopegat_d_db20_slot = var_slopegat_d_db20;
        *var_slopegat_d_db21_slot = var_slopegat_d_db21;
        *var_slopegat_d_db22_slot = var_slopegat_d_db22;
        *var_slopegat_d_db23_slot = var_slopegat_d_db23;
        *var_slopegat_d_db24_slot = var_slopegat_d_db24;
        *var_slopegat_d_db3_slot = var_slopegat_d_db3;
        *var_slopegat_d_db4_slot = var_slopegat_d_db4;
        *var_slopegat_d_db5_slot = var_slopegat_d_db5;
        *var_slopegat_d_db6_slot = var_slopegat_d_db6;
        *var_slopegat_d_db7_slot = var_slopegat_d_db7;
        *var_slopegat_d_db8_slot = var_slopegat_d_db8;
        *var_slopegat_d_db9_slot = var_slopegat_d_db9;
        *var_slopegat_d_dn0_slot = var_slopegat_d_dn0;
        *var_slopegat_d_dn1_slot = var_slopegat_d_dn1;
        *var_slopegat_d_dn10_slot = var_slopegat_d_dn10;
        *var_slopegat_d_dn11_slot = var_slopegat_d_dn11;
        *var_slopegat_d_dn12_slot = var_slopegat_d_dn12;
        *var_slopegat_d_dn13_slot = var_slopegat_d_dn13;
        *var_slopegat_d_dn14_slot = var_slopegat_d_dn14;
        *var_slopegat_d_dn15_slot = var_slopegat_d_dn15;
        *var_slopegat_d_dn16_slot = var_slopegat_d_dn16;
        *var_slopegat_d_dn17_slot = var_slopegat_d_dn17;
        *var_slopegat_d_dn18_slot = var_slopegat_d_dn18;
        *var_slopegat_d_dn19_slot = var_slopegat_d_dn19;
        *var_slopegat_d_dn2_slot = var_slopegat_d_dn2;
        *var_slopegat_d_dn20_slot = var_slopegat_d_dn20;
        *var_slopegat_d_dn3_slot = var_slopegat_d_dn3;
        *var_slopegat_d_dn4_slot = var_slopegat_d_dn4;
        *var_slopegat_d_dn5_slot = var_slopegat_d_dn5;
        *var_slopegat_d_dn6_slot = var_slopegat_d_dn6;
        *var_slopegat_d_dn7_slot = var_slopegat_d_dn7;
        *var_slopegat_d_dn8_slot = var_slopegat_d_dn8;
        *var_slopegat_d_dn9_slot = var_slopegat_d_dn9;
        *var_slopesti_d_slot = var_slopesti_d;
        *var_swgat2nd_d_slot = var_swgat2nd_d;
        *var_tka_slot = var_tka;
        *var_tkd_1_slot = var_tkd_1;
        *var_ubibot_slot = var_ubibot;
        *var_ubigat_slot = var_ubigat;
        *var_ubisti_slot = var_ubisti;
        *var_vbibot_slot = var_vbibot;
        *var_vbigat_slot = var_vbigat;
        *var_vbiinvbot_slot = var_vbiinvbot;
        *var_vbiinvgat_slot = var_vbiinvgat;
        *var_vbiinvsti_slot = var_vbiinvsti;
        *var_vbirgat2nd_d_slot = var_vbirgat2nd_d;
        *var_vbisti_slot = var_vbisti;
        *var_vbrinvbot_d_slot = var_vbrinvbot_d;
        *var_vbrinvgat_d_slot = var_vbrinvgat_d;
        *var_vbrinvgat_d_db0_slot = var_vbrinvgat_d_db0;
        *var_vbrinvgat_d_db1_slot = var_vbrinvgat_d_db1;
        *var_vbrinvgat_d_db10_slot = var_vbrinvgat_d_db10;
        *var_vbrinvgat_d_db11_slot = var_vbrinvgat_d_db11;
        *var_vbrinvgat_d_db12_slot = var_vbrinvgat_d_db12;
        *var_vbrinvgat_d_db13_slot = var_vbrinvgat_d_db13;
        *var_vbrinvgat_d_db14_slot = var_vbrinvgat_d_db14;
        *var_vbrinvgat_d_db15_slot = var_vbrinvgat_d_db15;
        *var_vbrinvgat_d_db16_slot = var_vbrinvgat_d_db16;
        *var_vbrinvgat_d_db17_slot = var_vbrinvgat_d_db17;
        *var_vbrinvgat_d_db18_slot = var_vbrinvgat_d_db18;
        *var_vbrinvgat_d_db19_slot = var_vbrinvgat_d_db19;
        *var_vbrinvgat_d_db2_slot = var_vbrinvgat_d_db2;
        *var_vbrinvgat_d_db20_slot = var_vbrinvgat_d_db20;
        *var_vbrinvgat_d_db21_slot = var_vbrinvgat_d_db21;
        *var_vbrinvgat_d_db22_slot = var_vbrinvgat_d_db22;
        *var_vbrinvgat_d_db23_slot = var_vbrinvgat_d_db23;
        *var_vbrinvgat_d_db24_slot = var_vbrinvgat_d_db24;
        *var_vbrinvgat_d_db3_slot = var_vbrinvgat_d_db3;
        *var_vbrinvgat_d_db4_slot = var_vbrinvgat_d_db4;
        *var_vbrinvgat_d_db5_slot = var_vbrinvgat_d_db5;
        *var_vbrinvgat_d_db6_slot = var_vbrinvgat_d_db6;
        *var_vbrinvgat_d_db7_slot = var_vbrinvgat_d_db7;
        *var_vbrinvgat_d_db8_slot = var_vbrinvgat_d_db8;
        *var_vbrinvgat_d_db9_slot = var_vbrinvgat_d_db9;
        *var_vbrinvgat_d_dn0_slot = var_vbrinvgat_d_dn0;
        *var_vbrinvgat_d_dn1_slot = var_vbrinvgat_d_dn1;
        *var_vbrinvgat_d_dn10_slot = var_vbrinvgat_d_dn10;
        *var_vbrinvgat_d_dn11_slot = var_vbrinvgat_d_dn11;
        *var_vbrinvgat_d_dn12_slot = var_vbrinvgat_d_dn12;
        *var_vbrinvgat_d_dn13_slot = var_vbrinvgat_d_dn13;
        *var_vbrinvgat_d_dn14_slot = var_vbrinvgat_d_dn14;
        *var_vbrinvgat_d_dn15_slot = var_vbrinvgat_d_dn15;
        *var_vbrinvgat_d_dn16_slot = var_vbrinvgat_d_dn16;
        *var_vbrinvgat_d_dn17_slot = var_vbrinvgat_d_dn17;
        *var_vbrinvgat_d_dn18_slot = var_vbrinvgat_d_dn18;
        *var_vbrinvgat_d_dn19_slot = var_vbrinvgat_d_dn19;
        *var_vbrinvgat_d_dn2_slot = var_vbrinvgat_d_dn2;
        *var_vbrinvgat_d_dn20_slot = var_vbrinvgat_d_dn20;
        *var_vbrinvgat_d_dn3_slot = var_vbrinvgat_d_dn3;
        *var_vbrinvgat_d_dn4_slot = var_vbrinvgat_d_dn4;
        *var_vbrinvgat_d_dn5_slot = var_vbrinvgat_d_dn5;
        *var_vbrinvgat_d_dn6_slot = var_vbrinvgat_d_dn6;
        *var_vbrinvgat_d_dn7_slot = var_vbrinvgat_d_dn7;
        *var_vbrinvgat_d_dn8_slot = var_vbrinvgat_d_dn8;
        *var_vbrinvgat_d_dn9_slot = var_vbrinvgat_d_dn9;
        *var_vbrinvsti_d_slot = var_vbrinvsti_d;
    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        var_auxt: f64,
        var_cjorbotd_i: f64,
        var_cjorgat2nd: f64,
        var_cjorgatd_i: f64,
        var_cjorstid_i: f64,
        var_deltaegat: f64,
        var_deltaphigd: f64,
        var_fbbtrbotd_i: f64,
        var_fbbtrgatd_i: f64,
        var_fbbtrstid_i: f64,
        var_idsatrbotd_i: f64,
        var_idsatrgatd_i: f64,
        var_idsatrstid_i: f64,
        var_mefftatbotd_i: f64,
        var_mefftatgatd_i: f64,
        var_mefftatstid_i: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_one_over_one_minus_pgat2nd: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbotd_i: f64,
        var_pgat2nd: f64,
        var_pgatd_i: f64,
        var_phigbotd_i: f64,
        var_phiggat2nd: f64,
        var_phiggatd_i: f64,
        var_phigrbot_d: f64,
        var_phigrgat2nd: f64,
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
        var_swgat2nd: f64,
        var_tkd_1: f64,
        var_tkr_1: f64,
        var_vbirbotd_i: f64,
        var_vbirgat2nd: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_atatbot_d_slot: &mut f64,
        var_atatgat_d_slot: &mut f64,
        var_atatsti_d_slot: &mut f64,
        var_btatpartbot_d_slot: &mut f64,
        var_btatpartgat_slot: &mut f64,
        var_btatpartgat_d_slot: &mut f64,
        var_btatpartsti_d_slot: &mut f64,
        var_cjobot_d_slot: &mut f64,
        var_cjogat2nd_slot: &mut f64,
        var_cjogat_d_slot: &mut f64,
        var_cjosti_d_slot: &mut f64,
        var_deltaebot_d_slot: &mut f64,
        var_deltaegat_d_slot: &mut f64,
        var_deltaesti_d_slot: &mut f64,
        var_fbbtbot_slot: &mut f64,
        var_fbbtbot_d_slot: &mut f64,
        var_fbbtgat_slot: &mut f64,
        var_fbbtgat_d_slot: &mut f64,
        var_fbbtgat_d_db0_slot: &mut f64,
        var_fbbtgat_d_db1_slot: &mut f64,
        var_fbbtgat_d_db10_slot: &mut f64,
        var_fbbtgat_d_db11_slot: &mut f64,
        var_fbbtgat_d_db12_slot: &mut f64,
        var_fbbtgat_d_db13_slot: &mut f64,
        var_fbbtgat_d_db14_slot: &mut f64,
        var_fbbtgat_d_db15_slot: &mut f64,
        var_fbbtgat_d_db16_slot: &mut f64,
        var_fbbtgat_d_db17_slot: &mut f64,
        var_fbbtgat_d_db18_slot: &mut f64,
        var_fbbtgat_d_db19_slot: &mut f64,
        var_fbbtgat_d_db2_slot: &mut f64,
        var_fbbtgat_d_db20_slot: &mut f64,
        var_fbbtgat_d_db21_slot: &mut f64,
        var_fbbtgat_d_db22_slot: &mut f64,
        var_fbbtgat_d_db23_slot: &mut f64,
        var_fbbtgat_d_db24_slot: &mut f64,
        var_fbbtgat_d_db3_slot: &mut f64,
        var_fbbtgat_d_db4_slot: &mut f64,
        var_fbbtgat_d_db5_slot: &mut f64,
        var_fbbtgat_d_db6_slot: &mut f64,
        var_fbbtgat_d_db7_slot: &mut f64,
        var_fbbtgat_d_db8_slot: &mut f64,
        var_fbbtgat_d_db9_slot: &mut f64,
        var_fbbtgat_d_dn0_slot: &mut f64,
        var_fbbtgat_d_dn1_slot: &mut f64,
        var_fbbtgat_d_dn10_slot: &mut f64,
        var_fbbtgat_d_dn11_slot: &mut f64,
        var_fbbtgat_d_dn12_slot: &mut f64,
        var_fbbtgat_d_dn13_slot: &mut f64,
        var_fbbtgat_d_dn14_slot: &mut f64,
        var_fbbtgat_d_dn15_slot: &mut f64,
        var_fbbtgat_d_dn16_slot: &mut f64,
        var_fbbtgat_d_dn17_slot: &mut f64,
        var_fbbtgat_d_dn18_slot: &mut f64,
        var_fbbtgat_d_dn19_slot: &mut f64,
        var_fbbtgat_d_dn2_slot: &mut f64,
        var_fbbtgat_d_dn20_slot: &mut f64,
        var_fbbtgat_d_dn3_slot: &mut f64,
        var_fbbtgat_d_dn4_slot: &mut f64,
        var_fbbtgat_d_dn5_slot: &mut f64,
        var_fbbtgat_d_dn6_slot: &mut f64,
        var_fbbtgat_d_dn7_slot: &mut f64,
        var_fbbtgat_d_dn8_slot: &mut f64,
        var_fbbtgat_d_dn9_slot: &mut f64,
        var_fbbtgat_db0_slot: &mut f64,
        var_fbbtgat_db1_slot: &mut f64,
        var_fbbtgat_db10_slot: &mut f64,
        var_fbbtgat_db11_slot: &mut f64,
        var_fbbtgat_db12_slot: &mut f64,
        var_fbbtgat_db13_slot: &mut f64,
        var_fbbtgat_db14_slot: &mut f64,
        var_fbbtgat_db15_slot: &mut f64,
        var_fbbtgat_db16_slot: &mut f64,
        var_fbbtgat_db17_slot: &mut f64,
        var_fbbtgat_db18_slot: &mut f64,
        var_fbbtgat_db19_slot: &mut f64,
        var_fbbtgat_db2_slot: &mut f64,
        var_fbbtgat_db20_slot: &mut f64,
        var_fbbtgat_db21_slot: &mut f64,
        var_fbbtgat_db22_slot: &mut f64,
        var_fbbtgat_db23_slot: &mut f64,
        var_fbbtgat_db24_slot: &mut f64,
        var_fbbtgat_db3_slot: &mut f64,
        var_fbbtgat_db4_slot: &mut f64,
        var_fbbtgat_db5_slot: &mut f64,
        var_fbbtgat_db6_slot: &mut f64,
        var_fbbtgat_db7_slot: &mut f64,
        var_fbbtgat_db8_slot: &mut f64,
        var_fbbtgat_db9_slot: &mut f64,
        var_fbbtgat_dn0_slot: &mut f64,
        var_fbbtgat_dn1_slot: &mut f64,
        var_fbbtgat_dn10_slot: &mut f64,
        var_fbbtgat_dn11_slot: &mut f64,
        var_fbbtgat_dn12_slot: &mut f64,
        var_fbbtgat_dn13_slot: &mut f64,
        var_fbbtgat_dn14_slot: &mut f64,
        var_fbbtgat_dn15_slot: &mut f64,
        var_fbbtgat_dn16_slot: &mut f64,
        var_fbbtgat_dn17_slot: &mut f64,
        var_fbbtgat_dn18_slot: &mut f64,
        var_fbbtgat_dn19_slot: &mut f64,
        var_fbbtgat_dn2_slot: &mut f64,
        var_fbbtgat_dn20_slot: &mut f64,
        var_fbbtgat_dn3_slot: &mut f64,
        var_fbbtgat_dn4_slot: &mut f64,
        var_fbbtgat_dn5_slot: &mut f64,
        var_fbbtgat_dn6_slot: &mut f64,
        var_fbbtgat_dn7_slot: &mut f64,
        var_fbbtgat_dn8_slot: &mut f64,
        var_fbbtgat_dn9_slot: &mut f64,
        var_fbbtsti_slot: &mut f64,
        var_fbbtsti_d_slot: &mut f64,
        var_ftdbot_d_slot: &mut f64,
        var_ftdgat2nd_slot: &mut f64,
        var_ftdgat_d_slot: &mut f64,
        var_ftdsti_d_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_idsatbot_d_slot: &mut f64,
        var_idsatgat_d_slot: &mut f64,
        var_idsatsti_d_slot: &mut f64,
        var_phigdbot_d_slot: &mut f64,
        var_phigdgat2nd_slot: &mut f64,
        var_phigdgat_d_slot: &mut f64,
        var_phigdsti_d_slot: &mut f64,
        var_qpref2bot_d_slot: &mut f64,
        var_qpref2gat2nd_slot: &mut f64,
        var_qpref2gat_d_slot: &mut f64,
        var_qpref2sti_d_slot: &mut f64,
        var_qprefbot_d_slot: &mut f64,
        var_qprefgat2nd_slot: &mut f64,
        var_qprefgat_d_slot: &mut f64,
        var_qprefsti_d_slot: &mut f64,
        var_ubibot_d_slot: &mut f64,
        var_ubigat2nd_slot: &mut f64,
        var_ubigat_d_slot: &mut f64,
        var_ubisti_d_slot: &mut f64,
        var_vbibot_d_slot: &mut f64,
        var_vbigat2nd_slot: &mut f64,
        var_vbigat_d_slot: &mut f64,
        var_vbiinvbot_d_slot: &mut f64,
        var_vbiinvgat2nd_slot: &mut f64,
        var_vbiinvgat_d_slot: &mut f64,
        var_vbiinvsti_d_slot: &mut f64,
        var_vbisti_d_slot: &mut f64,
    ) {
        let mut var_atatbot_d: f64 = *var_atatbot_d_slot;
        let mut var_atatgat_d: f64 = *var_atatgat_d_slot;
        let mut var_atatsti_d: f64 = *var_atatsti_d_slot;
        let mut var_btatpartbot_d: f64 = *var_btatpartbot_d_slot;
        let mut var_btatpartgat: f64 = *var_btatpartgat_slot;
        let mut var_btatpartgat_d: f64 = *var_btatpartgat_d_slot;
        let mut var_btatpartsti_d: f64 = *var_btatpartsti_d_slot;
        let mut var_cjobot_d: f64 = *var_cjobot_d_slot;
        let mut var_cjogat2nd: f64 = *var_cjogat2nd_slot;
        let mut var_cjogat_d: f64 = *var_cjogat_d_slot;
        let mut var_cjosti_d: f64 = *var_cjosti_d_slot;
        let mut var_deltaebot_d: f64 = *var_deltaebot_d_slot;
        let mut var_deltaegat_d: f64 = *var_deltaegat_d_slot;
        let mut var_deltaesti_d: f64 = *var_deltaesti_d_slot;
        let mut var_fbbtbot: f64 = *var_fbbtbot_slot;
        let mut var_fbbtbot_d: f64 = *var_fbbtbot_d_slot;
        let mut var_fbbtgat: f64 = *var_fbbtgat_slot;
        let mut var_fbbtgat_d: f64 = *var_fbbtgat_d_slot;
        let mut var_fbbtgat_d_db0: f64 = *var_fbbtgat_d_db0_slot;
        let mut var_fbbtgat_d_db1: f64 = *var_fbbtgat_d_db1_slot;
        let mut var_fbbtgat_d_db10: f64 = *var_fbbtgat_d_db10_slot;
        let mut var_fbbtgat_d_db11: f64 = *var_fbbtgat_d_db11_slot;
        let mut var_fbbtgat_d_db12: f64 = *var_fbbtgat_d_db12_slot;
        let mut var_fbbtgat_d_db13: f64 = *var_fbbtgat_d_db13_slot;
        let mut var_fbbtgat_d_db14: f64 = *var_fbbtgat_d_db14_slot;
        let mut var_fbbtgat_d_db15: f64 = *var_fbbtgat_d_db15_slot;
        let mut var_fbbtgat_d_db16: f64 = *var_fbbtgat_d_db16_slot;
        let mut var_fbbtgat_d_db17: f64 = *var_fbbtgat_d_db17_slot;
        let mut var_fbbtgat_d_db18: f64 = *var_fbbtgat_d_db18_slot;
        let mut var_fbbtgat_d_db19: f64 = *var_fbbtgat_d_db19_slot;
        let mut var_fbbtgat_d_db2: f64 = *var_fbbtgat_d_db2_slot;
        let mut var_fbbtgat_d_db20: f64 = *var_fbbtgat_d_db20_slot;
        let mut var_fbbtgat_d_db21: f64 = *var_fbbtgat_d_db21_slot;
        let mut var_fbbtgat_d_db22: f64 = *var_fbbtgat_d_db22_slot;
        let mut var_fbbtgat_d_db23: f64 = *var_fbbtgat_d_db23_slot;
        let mut var_fbbtgat_d_db24: f64 = *var_fbbtgat_d_db24_slot;
        let mut var_fbbtgat_d_db3: f64 = *var_fbbtgat_d_db3_slot;
        let mut var_fbbtgat_d_db4: f64 = *var_fbbtgat_d_db4_slot;
        let mut var_fbbtgat_d_db5: f64 = *var_fbbtgat_d_db5_slot;
        let mut var_fbbtgat_d_db6: f64 = *var_fbbtgat_d_db6_slot;
        let mut var_fbbtgat_d_db7: f64 = *var_fbbtgat_d_db7_slot;
        let mut var_fbbtgat_d_db8: f64 = *var_fbbtgat_d_db8_slot;
        let mut var_fbbtgat_d_db9: f64 = *var_fbbtgat_d_db9_slot;
        let mut var_fbbtgat_d_dn0: f64 = *var_fbbtgat_d_dn0_slot;
        let mut var_fbbtgat_d_dn1: f64 = *var_fbbtgat_d_dn1_slot;
        let mut var_fbbtgat_d_dn10: f64 = *var_fbbtgat_d_dn10_slot;
        let mut var_fbbtgat_d_dn11: f64 = *var_fbbtgat_d_dn11_slot;
        let mut var_fbbtgat_d_dn12: f64 = *var_fbbtgat_d_dn12_slot;
        let mut var_fbbtgat_d_dn13: f64 = *var_fbbtgat_d_dn13_slot;
        let mut var_fbbtgat_d_dn14: f64 = *var_fbbtgat_d_dn14_slot;
        let mut var_fbbtgat_d_dn15: f64 = *var_fbbtgat_d_dn15_slot;
        let mut var_fbbtgat_d_dn16: f64 = *var_fbbtgat_d_dn16_slot;
        let mut var_fbbtgat_d_dn17: f64 = *var_fbbtgat_d_dn17_slot;
        let mut var_fbbtgat_d_dn18: f64 = *var_fbbtgat_d_dn18_slot;
        let mut var_fbbtgat_d_dn19: f64 = *var_fbbtgat_d_dn19_slot;
        let mut var_fbbtgat_d_dn2: f64 = *var_fbbtgat_d_dn2_slot;
        let mut var_fbbtgat_d_dn20: f64 = *var_fbbtgat_d_dn20_slot;
        let mut var_fbbtgat_d_dn3: f64 = *var_fbbtgat_d_dn3_slot;
        let mut var_fbbtgat_d_dn4: f64 = *var_fbbtgat_d_dn4_slot;
        let mut var_fbbtgat_d_dn5: f64 = *var_fbbtgat_d_dn5_slot;
        let mut var_fbbtgat_d_dn6: f64 = *var_fbbtgat_d_dn6_slot;
        let mut var_fbbtgat_d_dn7: f64 = *var_fbbtgat_d_dn7_slot;
        let mut var_fbbtgat_d_dn8: f64 = *var_fbbtgat_d_dn8_slot;
        let mut var_fbbtgat_d_dn9: f64 = *var_fbbtgat_d_dn9_slot;
        let mut var_fbbtgat_db0: f64 = *var_fbbtgat_db0_slot;
        let mut var_fbbtgat_db1: f64 = *var_fbbtgat_db1_slot;
        let mut var_fbbtgat_db10: f64 = *var_fbbtgat_db10_slot;
        let mut var_fbbtgat_db11: f64 = *var_fbbtgat_db11_slot;
        let mut var_fbbtgat_db12: f64 = *var_fbbtgat_db12_slot;
        let mut var_fbbtgat_db13: f64 = *var_fbbtgat_db13_slot;
        let mut var_fbbtgat_db14: f64 = *var_fbbtgat_db14_slot;
        let mut var_fbbtgat_db15: f64 = *var_fbbtgat_db15_slot;
        let mut var_fbbtgat_db16: f64 = *var_fbbtgat_db16_slot;
        let mut var_fbbtgat_db17: f64 = *var_fbbtgat_db17_slot;
        let mut var_fbbtgat_db18: f64 = *var_fbbtgat_db18_slot;
        let mut var_fbbtgat_db19: f64 = *var_fbbtgat_db19_slot;
        let mut var_fbbtgat_db2: f64 = *var_fbbtgat_db2_slot;
        let mut var_fbbtgat_db20: f64 = *var_fbbtgat_db20_slot;
        let mut var_fbbtgat_db21: f64 = *var_fbbtgat_db21_slot;
        let mut var_fbbtgat_db22: f64 = *var_fbbtgat_db22_slot;
        let mut var_fbbtgat_db23: f64 = *var_fbbtgat_db23_slot;
        let mut var_fbbtgat_db24: f64 = *var_fbbtgat_db24_slot;
        let mut var_fbbtgat_db3: f64 = *var_fbbtgat_db3_slot;
        let mut var_fbbtgat_db4: f64 = *var_fbbtgat_db4_slot;
        let mut var_fbbtgat_db5: f64 = *var_fbbtgat_db5_slot;
        let mut var_fbbtgat_db6: f64 = *var_fbbtgat_db6_slot;
        let mut var_fbbtgat_db7: f64 = *var_fbbtgat_db7_slot;
        let mut var_fbbtgat_db8: f64 = *var_fbbtgat_db8_slot;
        let mut var_fbbtgat_db9: f64 = *var_fbbtgat_db9_slot;
        let mut var_fbbtgat_dn0: f64 = *var_fbbtgat_dn0_slot;
        let mut var_fbbtgat_dn1: f64 = *var_fbbtgat_dn1_slot;
        let mut var_fbbtgat_dn10: f64 = *var_fbbtgat_dn10_slot;
        let mut var_fbbtgat_dn11: f64 = *var_fbbtgat_dn11_slot;
        let mut var_fbbtgat_dn12: f64 = *var_fbbtgat_dn12_slot;
        let mut var_fbbtgat_dn13: f64 = *var_fbbtgat_dn13_slot;
        let mut var_fbbtgat_dn14: f64 = *var_fbbtgat_dn14_slot;
        let mut var_fbbtgat_dn15: f64 = *var_fbbtgat_dn15_slot;
        let mut var_fbbtgat_dn16: f64 = *var_fbbtgat_dn16_slot;
        let mut var_fbbtgat_dn17: f64 = *var_fbbtgat_dn17_slot;
        let mut var_fbbtgat_dn18: f64 = *var_fbbtgat_dn18_slot;
        let mut var_fbbtgat_dn19: f64 = *var_fbbtgat_dn19_slot;
        let mut var_fbbtgat_dn2: f64 = *var_fbbtgat_dn2_slot;
        let mut var_fbbtgat_dn20: f64 = *var_fbbtgat_dn20_slot;
        let mut var_fbbtgat_dn3: f64 = *var_fbbtgat_dn3_slot;
        let mut var_fbbtgat_dn4: f64 = *var_fbbtgat_dn4_slot;
        let mut var_fbbtgat_dn5: f64 = *var_fbbtgat_dn5_slot;
        let mut var_fbbtgat_dn6: f64 = *var_fbbtgat_dn6_slot;
        let mut var_fbbtgat_dn7: f64 = *var_fbbtgat_dn7_slot;
        let mut var_fbbtgat_dn8: f64 = *var_fbbtgat_dn8_slot;
        let mut var_fbbtgat_dn9: f64 = *var_fbbtgat_dn9_slot;
        let mut var_fbbtsti: f64 = *var_fbbtsti_slot;
        let mut var_fbbtsti_d: f64 = *var_fbbtsti_d_slot;
        let mut var_ftdbot_d: f64 = *var_ftdbot_d_slot;
        let mut var_ftdgat2nd: f64 = *var_ftdgat2nd_slot;
        let mut var_ftdgat_d: f64 = *var_ftdgat_d_slot;
        let mut var_ftdsti_d: f64 = *var_ftdsti_d_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_idsatbot_d: f64 = *var_idsatbot_d_slot;
        let mut var_idsatgat_d: f64 = *var_idsatgat_d_slot;
        let mut var_idsatsti_d: f64 = *var_idsatsti_d_slot;
        let mut var_phigdbot_d: f64 = *var_phigdbot_d_slot;
        let mut var_phigdgat2nd: f64 = *var_phigdgat2nd_slot;
        let mut var_phigdgat_d: f64 = *var_phigdgat_d_slot;
        let mut var_phigdsti_d: f64 = *var_phigdsti_d_slot;
        let mut var_qpref2bot_d: f64 = *var_qpref2bot_d_slot;
        let mut var_qpref2gat2nd: f64 = *var_qpref2gat2nd_slot;
        let mut var_qpref2gat_d: f64 = *var_qpref2gat_d_slot;
        let mut var_qpref2sti_d: f64 = *var_qpref2sti_d_slot;
        let mut var_qprefbot_d: f64 = *var_qprefbot_d_slot;
        let mut var_qprefgat2nd: f64 = *var_qprefgat2nd_slot;
        let mut var_qprefgat_d: f64 = *var_qprefgat_d_slot;
        let mut var_qprefsti_d: f64 = *var_qprefsti_d_slot;
        let mut var_ubibot_d: f64 = *var_ubibot_d_slot;
        let mut var_ubigat2nd: f64 = *var_ubigat2nd_slot;
        let mut var_ubigat_d: f64 = *var_ubigat_d_slot;
        let mut var_ubisti_d: f64 = *var_ubisti_d_slot;
        let mut var_vbibot_d: f64 = *var_vbibot_d_slot;
        let mut var_vbigat2nd: f64 = *var_vbigat2nd_slot;
        let mut var_vbigat_d: f64 = *var_vbigat_d_slot;
        let mut var_vbiinvbot_d: f64 = *var_vbiinvbot_d_slot;
        let mut var_vbiinvgat2nd: f64 = *var_vbiinvgat2nd_slot;
        let mut var_vbiinvgat_d: f64 = *var_vbiinvgat_d_slot;
        let mut var_vbiinvsti_d: f64 = *var_vbiinvsti_d_slot;
        let mut var_vbisti_d: f64 = *var_vbisti_d_slot;

        let assign2750_e3062: f64 = (32.0 * p.p850);
        let assign2750_e3064: f64 = (assign2750_e3062 * 9.1093826e-31);
        let assign2750_e3066: f64 = (assign2750_e3064 * 1.6021918e-19);
        let assign2750_e3069: f64 = (var_deltaegat * var_deltaegat);
        let assign2750_e3071: f64 = (assign2750_e3069 * var_deltaegat);
        let assign2750_e3072: f64 = (assign2750_e3066 * assign2750_e3071);
        let assign2750_e3073: f64 = (assign2750_e3072).sqrt();
        let assign2750_e3076: f64 = (3.0 * 1.05457168e-34);
        let assign2750_e3077: f64 = (assign2750_e3073 / assign2750_e3076);
        var_btatpartgat = assign2750_e3077;

        let assign2760_e3083: f64 = (var_tkd_1 - var_tkr_1);
        let assign2760_e3084: f64 = (p.p857 * assign2760_e3083);
        let assign2760_e3085: f64 = (1.0 + assign2760_e3084);
        let assign2760_e3086: f64 = (p.p854 * assign2760_e3085);
        var_fbbtbot = assign2760_e3086;

        let assign2770_e3092: f64 = (var_tkd_1 - var_tkr_1);
        let assign2770_e3093: f64 = (p.p858 * assign2770_e3092);
        let assign2770_e3094: f64 = (1.0 + assign2770_e3093);
        let assign2770_e3095: f64 = (p.p855 * assign2770_e3094);
        var_fbbtsti = assign2770_e3095;

        let assign2780_e3101: f64 = (var_tkd_1 - var_tkr_1);
        let assign2780_e3102: f64 = (p.p859 * assign2780_e3101);
        let assign2780_e3103: f64 = (1.0 + assign2780_e3102);
        let assign2780_e3104: f64 = (p.p856 * assign2780_e3103);
        var_fbbtgat = assign2780_e3104;
        var_fbbtgat_dn0 = 0.0;
        var_fbbtgat_dn1 = 0.0;
        var_fbbtgat_dn2 = 0.0;
        var_fbbtgat_dn3 = 0.0;
        var_fbbtgat_dn4 = 0.0;
        var_fbbtgat_dn5 = 0.0;
        var_fbbtgat_dn6 = 0.0;
        var_fbbtgat_dn7 = 0.0;
        var_fbbtgat_dn8 = 0.0;
        var_fbbtgat_dn9 = 0.0;
        var_fbbtgat_dn10 = 0.0;
        var_fbbtgat_dn11 = 0.0;
        var_fbbtgat_dn12 = 0.0;
        var_fbbtgat_dn13 = 0.0;
        var_fbbtgat_dn14 = 0.0;
        var_fbbtgat_dn15 = 0.0;
        var_fbbtgat_dn16 = 0.0;
        var_fbbtgat_dn17 = 0.0;
        var_fbbtgat_dn18 = 0.0;
        var_fbbtgat_dn19 = 0.0;
        var_fbbtgat_dn20 = 0.0;
        var_fbbtgat_db0 = 0.0;
        var_fbbtgat_db1 = 0.0;
        var_fbbtgat_db2 = 0.0;
        var_fbbtgat_db3 = 0.0;
        var_fbbtgat_db4 = 0.0;
        var_fbbtgat_db5 = 0.0;
        var_fbbtgat_db6 = 0.0;
        var_fbbtgat_db7 = 0.0;
        var_fbbtgat_db8 = 0.0;
        var_fbbtgat_db9 = 0.0;
        var_fbbtgat_db10 = 0.0;
        var_fbbtgat_db11 = 0.0;
        var_fbbtgat_db12 = 0.0;
        var_fbbtgat_db13 = 0.0;
        var_fbbtgat_db14 = 0.0;
        var_fbbtgat_db15 = 0.0;
        var_fbbtgat_db16 = 0.0;
        var_fbbtgat_db17 = 0.0;
        var_fbbtgat_db18 = 0.0;
        var_fbbtgat_db19 = 0.0;
        var_fbbtgat_db20 = 0.0;
        var_fbbtgat_db21 = 0.0;
        var_fbbtgat_db22 = 0.0;
        var_fbbtgat_db23 = 0.0;
        var_fbbtgat_db24 = 0.0;

        let (assign2790_e3110,) = {
    if (var_fbbtbot > 0.0) {
        (var_fbbtbot,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot = assign2790_e3110;

        let (assign2800_e3116,) = {
    if (var_fbbtsti > 0.0) {
        (var_fbbtsti,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti = assign2800_e3116;

        let (assign2810_e3122, assign2810_e3122_d_n0, assign2810_e3122_d_n1, assign2810_e3122_d_n2, assign2810_e3122_d_n3, assign2810_e3122_d_n4, assign2810_e3122_d_n5, assign2810_e3122_d_n6, assign2810_e3122_d_n7, assign2810_e3122_d_n8, assign2810_e3122_d_n9, assign2810_e3122_d_n10, assign2810_e3122_d_n11, assign2810_e3122_d_n12, assign2810_e3122_d_n13, assign2810_e3122_d_n14, assign2810_e3122_d_n15, assign2810_e3122_d_n16, assign2810_e3122_d_n17, assign2810_e3122_d_n18, assign2810_e3122_d_n19, assign2810_e3122_d_n20, assign2810_e3122_d_b0, assign2810_e3122_d_b1, assign2810_e3122_d_b2, assign2810_e3122_d_b3, assign2810_e3122_d_b4, assign2810_e3122_d_b5, assign2810_e3122_d_b6, assign2810_e3122_d_b7, assign2810_e3122_d_b8, assign2810_e3122_d_b9, assign2810_e3122_d_b10, assign2810_e3122_d_b11, assign2810_e3122_d_b12, assign2810_e3122_d_b13, assign2810_e3122_d_b14, assign2810_e3122_d_b15, assign2810_e3122_d_b16, assign2810_e3122_d_b17, assign2810_e3122_d_b18, assign2810_e3122_d_b19, assign2810_e3122_d_b20, assign2810_e3122_d_b21, assign2810_e3122_d_b22, assign2810_e3122_d_b23, assign2810_e3122_d_b24,) = {
    if (var_fbbtgat > 0.0) {
        (var_fbbtgat, var_fbbtgat_dn0, var_fbbtgat_dn1, var_fbbtgat_dn2, var_fbbtgat_dn3, var_fbbtgat_dn4, var_fbbtgat_dn5, var_fbbtgat_dn6, var_fbbtgat_dn7, var_fbbtgat_dn8, var_fbbtgat_dn9, var_fbbtgat_dn10, var_fbbtgat_dn11, var_fbbtgat_dn12, var_fbbtgat_dn13, var_fbbtgat_dn14, var_fbbtgat_dn15, var_fbbtgat_dn16, var_fbbtgat_dn17, var_fbbtgat_dn18, var_fbbtgat_dn19, var_fbbtgat_dn20, var_fbbtgat_db0, var_fbbtgat_db1, var_fbbtgat_db2, var_fbbtgat_db3, var_fbbtgat_db4, var_fbbtgat_db5, var_fbbtgat_db6, var_fbbtgat_db7, var_fbbtgat_db8, var_fbbtgat_db9, var_fbbtgat_db10, var_fbbtgat_db11, var_fbbtgat_db12, var_fbbtgat_db13, var_fbbtgat_db14, var_fbbtgat_db15, var_fbbtgat_db16, var_fbbtgat_db17, var_fbbtgat_db18, var_fbbtgat_db19, var_fbbtgat_db20, var_fbbtgat_db21, var_fbbtgat_db22, var_fbbtgat_db23, var_fbbtgat_db24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat = assign2810_e3122;
        var_fbbtgat_dn0 = assign2810_e3122_d_n0;
        var_fbbtgat_dn1 = assign2810_e3122_d_n1;
        var_fbbtgat_dn2 = assign2810_e3122_d_n2;
        var_fbbtgat_dn3 = assign2810_e3122_d_n3;
        var_fbbtgat_dn4 = assign2810_e3122_d_n4;
        var_fbbtgat_dn5 = assign2810_e3122_d_n5;
        var_fbbtgat_dn6 = assign2810_e3122_d_n6;
        var_fbbtgat_dn7 = assign2810_e3122_d_n7;
        var_fbbtgat_dn8 = assign2810_e3122_d_n8;
        var_fbbtgat_dn9 = assign2810_e3122_d_n9;
        var_fbbtgat_dn10 = assign2810_e3122_d_n10;
        var_fbbtgat_dn11 = assign2810_e3122_d_n11;
        var_fbbtgat_dn12 = assign2810_e3122_d_n12;
        var_fbbtgat_dn13 = assign2810_e3122_d_n13;
        var_fbbtgat_dn14 = assign2810_e3122_d_n14;
        var_fbbtgat_dn15 = assign2810_e3122_d_n15;
        var_fbbtgat_dn16 = assign2810_e3122_d_n16;
        var_fbbtgat_dn17 = assign2810_e3122_d_n17;
        var_fbbtgat_dn18 = assign2810_e3122_d_n18;
        var_fbbtgat_dn19 = assign2810_e3122_d_n19;
        var_fbbtgat_dn20 = assign2810_e3122_d_n20;
        var_fbbtgat_db0 = assign2810_e3122_d_b0;
        var_fbbtgat_db1 = assign2810_e3122_d_b1;
        var_fbbtgat_db2 = assign2810_e3122_d_b2;
        var_fbbtgat_db3 = assign2810_e3122_d_b3;
        var_fbbtgat_db4 = assign2810_e3122_d_b4;
        var_fbbtgat_db5 = assign2810_e3122_d_b5;
        var_fbbtgat_db6 = assign2810_e3122_d_b6;
        var_fbbtgat_db7 = assign2810_e3122_d_b7;
        var_fbbtgat_db8 = assign2810_e3122_d_b8;
        var_fbbtgat_db9 = assign2810_e3122_d_b9;
        var_fbbtgat_db10 = assign2810_e3122_d_b10;
        var_fbbtgat_db11 = assign2810_e3122_d_b11;
        var_fbbtgat_db12 = assign2810_e3122_d_b12;
        var_fbbtgat_db13 = assign2810_e3122_d_b13;
        var_fbbtgat_db14 = assign2810_e3122_d_b14;
        var_fbbtgat_db15 = assign2810_e3122_d_b15;
        var_fbbtgat_db16 = assign2810_e3122_d_b16;
        var_fbbtgat_db17 = assign2810_e3122_d_b17;
        var_fbbtgat_db18 = assign2810_e3122_d_b18;
        var_fbbtgat_db19 = assign2810_e3122_d_b19;
        var_fbbtgat_db20 = assign2810_e3122_d_b20;
        var_fbbtgat_db21 = assign2810_e3122_d_b21;
        var_fbbtgat_db22 = assign2810_e3122_d_b22;
        var_fbbtgat_db23 = assign2810_e3122_d_b23;
        var_fbbtgat_db24 = assign2810_e3122_d_b24;

        let assign2820_e3125: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard32 = assign2820_e3125;

        let (assign2830_e3131,) = {
    if (var_guard32 != 0.0) {
        let assign2830_e3129: f64 = (var_phiggat2nd + var_deltaphigd);
        (assign2830_e3129,)
    } else {
        (var_phigdgat2nd,)
    }
};
        var_phigdgat2nd = assign2830_e3131;

        let (assign2840_e3148,) = {
    if (var_guard32 != 0.0) {
        let assign2840_e3135: f64 = (var_auxt).powf(1.5);
        let assign2840_e3139: f64 = (var_phigrgat2nd * var_phitrinv);
        let assign2840_e3142: f64 = (var_phigdgat2nd * var_phitdinv);
        let assign2840_e3143: f64 = (assign2840_e3139 - assign2840_e3142);
        let assign2840_e3144: f64 = (0.5 * assign2840_e3143);
        let assign2840_e3145: f64 = (assign2840_e3144).exp();
        let assign2840_e3146: f64 = (assign2840_e3135 * assign2840_e3145);
        (assign2840_e3146,)
    } else {
        (var_ftdgat2nd,)
    }
};
        var_ftdgat2nd = assign2840_e3148;

        let (assign2850_e3161,) = {
    if (var_guard32 != 0.0) {
        let assign2850_e3152: f64 = (var_vbirgat2nd * var_auxt);
        let assign2850_e3155: f64 = (2.0 * var_phitd);
        let assign2850_e3157: f64 = (var_ftdgat2nd).ln();
        let assign2850_e3158: f64 = (assign2850_e3155 * assign2850_e3157);
        let assign2850_e3159: f64 = (assign2850_e3152 - assign2850_e3158);
        (assign2850_e3159,)
    } else {
        (var_ubigat2nd,)
    }
};
        var_ubigat2nd = assign2850_e3161;

        let (assign2860_e3177,) = {
    if (var_guard32 != 0.0) {
        let assign2860_e3168: f64 = (0.05 - var_ubigat2nd);
        let assign2860_e3170: f64 = (assign2860_e3168 * var_phitdinv);
        let assign2860_e3171: f64 = (assign2860_e3170).exp();
        let assign2860_e3172: f64 = (1.0 + assign2860_e3171);
        let assign2860_e3173: f64 = (assign2860_e3172).ln();
        let assign2860_e3174: f64 = (var_phitd * assign2860_e3173);
        let assign2860_e3175: f64 = (var_ubigat2nd + assign2860_e3174);
        (assign2860_e3175,)
    } else {
        (var_vbigat2nd,)
    }
};
        var_vbigat2nd = assign2860_e3177;

        let (assign2870_e3183,) = {
    if (var_guard32 != 0.0) {
        let assign2870_e3181: f64 = (1.0 / var_vbigat2nd);
        (assign2870_e3181,)
    } else {
        (var_vbiinvgat2nd,)
    }
};
        var_vbiinvgat2nd = assign2870_e3183;

        let (assign2880_e3193,) = {
    if (var_guard32 != 0.0) {
        let assign2880_e3188: f64 = (var_vbirgat2nd * var_vbiinvgat2nd);
        let assign2880_e3190: f64 = (assign2880_e3188).powf(var_pgat2nd);
        let assign2880_e3191: f64 = (var_cjorgat2nd * assign2880_e3190);
        (assign2880_e3191,)
    } else {
        (var_cjogat2nd,)
    }
};
        var_cjogat2nd = assign2880_e3193;

        let (assign2890_e3201,) = {
    if (var_guard32 != 0.0) {
        let assign2890_e3197: f64 = (var_cjogat2nd * var_vbigat2nd);
        let assign2890_e3199: f64 = (assign2890_e3197 * var_one_over_one_minus_pgat2nd);
        (assign2890_e3199,)
    } else {
        (var_qprefgat2nd,)
    }
};
        var_qprefgat2nd = assign2890_e3201;

        let (assign2900_e3207,) = {
    if (var_guard32 != 0.0) {
        let assign2900_e3205: f64 = (2.0 * var_cjogat2nd);
        (assign2900_e3205,)
    } else {
        (var_qpref2gat2nd,)
    }
};
        var_qpref2gat2nd = assign2900_e3207;

        let assign2910_e3210: f64 = (var_phigbotd_i + var_deltaphigd);
        var_phigdbot_d = assign2910_e3210;

        let assign2920_e3213: f64 = (var_phigstid_i + var_deltaphigd);
        var_phigdsti_d = assign2920_e3213;

        let assign2930_e3216: f64 = (var_phiggatd_i + var_deltaphigd);
        var_phigdgat_d = assign2930_e3216;

        let assign2940_e3219: f64 = (var_auxt).powf(1.5);
        let assign2940_e3223: f64 = (var_phigrbot_d * var_phitrinv);
        let assign2940_e3226: f64 = (var_phigdbot_d * var_phitdinv);
        let assign2940_e3227: f64 = (assign2940_e3223 - assign2940_e3226);
        let assign2940_e3228: f64 = (0.5 * assign2940_e3227);
        let assign2940_e3229: f64 = (assign2940_e3228).exp();
        let assign2940_e3230: f64 = (assign2940_e3219 * assign2940_e3229);
        var_ftdbot_d = assign2940_e3230;

        let assign2950_e3233: f64 = (var_auxt).powf(1.5);
        let assign2950_e3237: f64 = (var_phigrsti_d * var_phitrinv);
        let assign2950_e3240: f64 = (var_phigdsti_d * var_phitdinv);
        let assign2950_e3241: f64 = (assign2950_e3237 - assign2950_e3240);
        let assign2950_e3242: f64 = (0.5 * assign2950_e3241);
        let assign2950_e3243: f64 = (assign2950_e3242).exp();
        let assign2950_e3244: f64 = (assign2950_e3233 * assign2950_e3243);
        var_ftdsti_d = assign2950_e3244;

        let assign2960_e3247: f64 = (var_auxt).powf(1.5);
        let assign2960_e3251: f64 = (var_phigrgat_d * var_phitrinv);
        let assign2960_e3254: f64 = (var_phigdgat_d * var_phitdinv);
        let assign2960_e3255: f64 = (assign2960_e3251 - assign2960_e3254);
        let assign2960_e3256: f64 = (0.5 * assign2960_e3255);
        let assign2960_e3257: f64 = (assign2960_e3256).exp();
        let assign2960_e3258: f64 = (assign2960_e3247 * assign2960_e3257);
        var_ftdgat_d = assign2960_e3258;

        let assign2970_e3261: f64 = (var_idsatrbotd_i * var_ftdbot_d);
        let assign2970_e3263: f64 = (assign2970_e3261 * var_ftdbot_d);
        var_idsatbot_d = assign2970_e3263;

        let assign2980_e3266: f64 = (var_idsatrstid_i * var_ftdsti_d);
        let assign2980_e3268: f64 = (assign2980_e3266 * var_ftdsti_d);
        var_idsatsti_d = assign2980_e3268;

        let assign2990_e3271: f64 = (var_idsatrgatd_i * var_ftdgat_d);
        let assign2990_e3273: f64 = (assign2990_e3271 * var_ftdgat_d);
        var_idsatgat_d = assign2990_e3273;

        let assign3000_e3276: f64 = (var_vbirbotd_i * var_auxt);
        let assign3000_e3279: f64 = (2.0 * var_phitd);
        let assign3000_e3281: f64 = (var_ftdbot_d).ln();
        let assign3000_e3282: f64 = (assign3000_e3279 * assign3000_e3281);
        let assign3000_e3283: f64 = (assign3000_e3276 - assign3000_e3282);
        var_ubibot_d = assign3000_e3283;

        let assign3010_e3286: f64 = (var_vbirstid_i * var_auxt);
        let assign3010_e3289: f64 = (2.0 * var_phitd);
        let assign3010_e3291: f64 = (var_ftdsti_d).ln();
        let assign3010_e3292: f64 = (assign3010_e3289 * assign3010_e3291);
        let assign3010_e3293: f64 = (assign3010_e3286 - assign3010_e3292);
        var_ubisti_d = assign3010_e3293;

        let assign3020_e3296: f64 = (var_vbirgatd_i * var_auxt);
        let assign3020_e3299: f64 = (2.0 * var_phitd);
        let assign3020_e3301: f64 = (var_ftdgat_d).ln();
        let assign3020_e3302: f64 = (assign3020_e3299 * assign3020_e3301);
        let assign3020_e3303: f64 = (assign3020_e3296 - assign3020_e3302);
        var_ubigat_d = assign3020_e3303;

        let assign3030_e3309: f64 = (0.05 - var_ubibot_d);
        let assign3030_e3311: f64 = (assign3030_e3309 * var_phitdinv);
        let assign3030_e3312: f64 = (assign3030_e3311).exp();
        let assign3030_e3313: f64 = (1.0 + assign3030_e3312);
        let assign3030_e3314: f64 = (assign3030_e3313).ln();
        let assign3030_e3315: f64 = (var_phitd * assign3030_e3314);
        let assign3030_e3316: f64 = (var_ubibot_d + assign3030_e3315);
        var_vbibot_d = assign3030_e3316;

        let assign3040_e3322: f64 = (0.05 - var_ubisti_d);
        let assign3040_e3324: f64 = (assign3040_e3322 * var_phitdinv);
        let assign3040_e3325: f64 = (assign3040_e3324).exp();
        let assign3040_e3326: f64 = (1.0 + assign3040_e3325);
        let assign3040_e3327: f64 = (assign3040_e3326).ln();
        let assign3040_e3328: f64 = (var_phitd * assign3040_e3327);
        let assign3040_e3329: f64 = (var_ubisti_d + assign3040_e3328);
        var_vbisti_d = assign3040_e3329;

        let assign3050_e3335: f64 = (0.05 - var_ubigat_d);
        let assign3050_e3337: f64 = (assign3050_e3335 * var_phitdinv);
        let assign3050_e3338: f64 = (assign3050_e3337).exp();
        let assign3050_e3339: f64 = (1.0 + assign3050_e3338);
        let assign3050_e3340: f64 = (assign3050_e3339).ln();
        let assign3050_e3341: f64 = (var_phitd * assign3050_e3340);
        let assign3050_e3342: f64 = (var_ubigat_d + assign3050_e3341);
        var_vbigat_d = assign3050_e3342;

        let assign3060_e3345: f64 = (1.0 / var_vbibot_d);
        var_vbiinvbot_d = assign3060_e3345;

        let assign3070_e3348: f64 = (1.0 / var_vbisti_d);
        var_vbiinvsti_d = assign3070_e3348;

        let assign3080_e3351: f64 = (1.0 / var_vbigat_d);
        var_vbiinvgat_d = assign3080_e3351;

        let assign3090_e3355: f64 = (var_vbirbotd_i * var_vbiinvbot_d);
        let assign3090_e3357: f64 = (assign3090_e3355).powf(var_pbotd_i);
        let assign3090_e3358: f64 = (var_cjorbotd_i * assign3090_e3357);
        var_cjobot_d = assign3090_e3358;

        let assign3100_e3362: f64 = (var_vbirstid_i * var_vbiinvsti_d);
        let assign3100_e3364: f64 = (assign3100_e3362).powf(var_pstid_i);
        let assign3100_e3365: f64 = (var_cjorstid_i * assign3100_e3364);
        var_cjosti_d = assign3100_e3365;

        let assign3110_e3369: f64 = (var_vbirgatd_i * var_vbiinvgat_d);
        let assign3110_e3371: f64 = (assign3110_e3369).powf(var_pgatd_i);
        let assign3110_e3372: f64 = (var_cjorgatd_i * assign3110_e3371);
        var_cjogat_d = assign3110_e3372;

        let assign3120_e3375: f64 = (var_cjobot_d * var_vbibot_d);
        let assign3120_e3377: f64 = (assign3120_e3375 * var_one_over_one_minus_pbot_d);
        var_qprefbot_d = assign3120_e3377;

        let assign3130_e3380: f64 = (var_cjosti_d * var_vbisti_d);
        let assign3130_e3382: f64 = (assign3130_e3380 * var_one_over_one_minus_psti_d);
        var_qprefsti_d = assign3130_e3382;

        let assign3140_e3385: f64 = (var_cjogat_d * var_vbigat_d);
        let assign3140_e3387: f64 = (assign3140_e3385 * var_one_over_one_minus_pgat_d);
        var_qprefgat_d = assign3140_e3387;

        let assign3150_e3390: f64 = (2.0 * var_cjobot_d);
        var_qpref2bot_d = assign3150_e3390;

        let assign3160_e3393: f64 = (2.0 * var_cjosti_d);
        var_qpref2sti_d = assign3160_e3393;

        let assign3170_e3396: f64 = (2.0 * var_cjogat_d);
        var_qpref2gat_d = assign3170_e3396;

        let assign3180_e3399: f64 = (0.5 * var_phigdbot_d);
        let assign3180_e3401: f64 = (assign3180_e3399).max(var_phitd);
        var_deltaebot_d = assign3180_e3401;

        let assign3190_e3404: f64 = (0.5 * var_phigdsti_d);
        let assign3190_e3406: f64 = (assign3190_e3404).max(var_phitd);
        var_deltaesti_d = assign3190_e3406;

        let assign3200_e3409: f64 = (0.5 * var_phigdgat_d);
        let assign3200_e3411: f64 = (assign3200_e3409).max(var_phitd);
        var_deltaegat_d = assign3200_e3411;

        let assign3210_e3414: f64 = (var_deltaebot_d * var_phitdinv);
        var_atatbot_d = assign3210_e3414;

        let assign3220_e3417: f64 = (var_deltaesti_d * var_phitdinv);
        var_atatsti_d = assign3220_e3417;

        let assign3230_e3420: f64 = (var_deltaegat_d * var_phitdinv);
        var_atatgat_d = assign3230_e3420;

        let assign3240_e3423: f64 = (32.0 * var_mefftatbotd_i);
        let assign3240_e3425: f64 = (assign3240_e3423 * 9.1093826e-31);
        let assign3240_e3427: f64 = (assign3240_e3425 * 1.6021918e-19);
        let assign3240_e3430: f64 = (var_deltaebot_d * var_deltaebot_d);
        let assign3240_e3432: f64 = (assign3240_e3430 * var_deltaebot_d);
        let assign3240_e3433: f64 = (assign3240_e3427 * assign3240_e3432);
        let assign3240_e3434: f64 = (assign3240_e3433).sqrt();
        let assign3240_e3437: f64 = (3.0 * 1.05457168e-34);
        let assign3240_e3438: f64 = (assign3240_e3434 / assign3240_e3437);
        var_btatpartbot_d = assign3240_e3438;

        let assign3250_e3441: f64 = (32.0 * var_mefftatstid_i);
        let assign3250_e3443: f64 = (assign3250_e3441 * 9.1093826e-31);
        let assign3250_e3445: f64 = (assign3250_e3443 * 1.6021918e-19);
        let assign3250_e3448: f64 = (var_deltaesti_d * var_deltaesti_d);
        let assign3250_e3450: f64 = (assign3250_e3448 * var_deltaesti_d);
        let assign3250_e3451: f64 = (assign3250_e3445 * assign3250_e3450);
        let assign3250_e3452: f64 = (assign3250_e3451).sqrt();
        let assign3250_e3455: f64 = (3.0 * 1.05457168e-34);
        let assign3250_e3456: f64 = (assign3250_e3452 / assign3250_e3455);
        var_btatpartsti_d = assign3250_e3456;

        let assign3260_e3459: f64 = (32.0 * var_mefftatgatd_i);
        let assign3260_e3461: f64 = (assign3260_e3459 * 9.1093826e-31);
        let assign3260_e3463: f64 = (assign3260_e3461 * 1.6021918e-19);
        let assign3260_e3466: f64 = (var_deltaegat_d * var_deltaegat_d);
        let assign3260_e3468: f64 = (assign3260_e3466 * var_deltaegat_d);
        let assign3260_e3469: f64 = (assign3260_e3463 * assign3260_e3468);
        let assign3260_e3470: f64 = (assign3260_e3469).sqrt();
        let assign3260_e3473: f64 = (3.0 * 1.05457168e-34);
        let assign3260_e3474: f64 = (assign3260_e3470 / assign3260_e3473);
        var_btatpartgat_d = assign3260_e3474;

        let assign3270_e3480: f64 = (var_tkd_1 - var_tkr_1);
        let assign3270_e3481: f64 = (var_stfbbtbotd_i * assign3270_e3480);
        let assign3270_e3482: f64 = (1.0 + assign3270_e3481);
        let assign3270_e3483: f64 = (var_fbbtrbotd_i * assign3270_e3482);
        var_fbbtbot_d = assign3270_e3483;

        let assign3280_e3489: f64 = (var_tkd_1 - var_tkr_1);
        let assign3280_e3490: f64 = (var_stfbbtstid_i * assign3280_e3489);
        let assign3280_e3491: f64 = (1.0 + assign3280_e3490);
        let assign3280_e3492: f64 = (var_fbbtrstid_i * assign3280_e3491);
        var_fbbtsti_d = assign3280_e3492;

        let assign3290_e3498: f64 = (var_tkd_1 - var_tkr_1);
        let assign3290_e3499: f64 = (var_stfbbtgatd_i * assign3290_e3498);
        let assign3290_e3500: f64 = (1.0 + assign3290_e3499);
        let assign3290_e3501: f64 = (var_fbbtrgatd_i * assign3290_e3500);
        var_fbbtgat_d = assign3290_e3501;
        var_fbbtgat_d_dn0 = 0.0;
        var_fbbtgat_d_dn1 = 0.0;
        var_fbbtgat_d_dn2 = 0.0;
        var_fbbtgat_d_dn3 = 0.0;
        var_fbbtgat_d_dn4 = 0.0;
        var_fbbtgat_d_dn5 = 0.0;
        var_fbbtgat_d_dn6 = 0.0;
        var_fbbtgat_d_dn7 = 0.0;
        var_fbbtgat_d_dn8 = 0.0;
        var_fbbtgat_d_dn9 = 0.0;
        var_fbbtgat_d_dn10 = 0.0;
        var_fbbtgat_d_dn11 = 0.0;
        var_fbbtgat_d_dn12 = 0.0;
        var_fbbtgat_d_dn13 = 0.0;
        var_fbbtgat_d_dn14 = 0.0;
        var_fbbtgat_d_dn15 = 0.0;
        var_fbbtgat_d_dn16 = 0.0;
        var_fbbtgat_d_dn17 = 0.0;
        var_fbbtgat_d_dn18 = 0.0;
        var_fbbtgat_d_dn19 = 0.0;
        var_fbbtgat_d_dn20 = 0.0;
        var_fbbtgat_d_db0 = 0.0;
        var_fbbtgat_d_db1 = 0.0;
        var_fbbtgat_d_db2 = 0.0;
        var_fbbtgat_d_db3 = 0.0;
        var_fbbtgat_d_db4 = 0.0;
        var_fbbtgat_d_db5 = 0.0;
        var_fbbtgat_d_db6 = 0.0;
        var_fbbtgat_d_db7 = 0.0;
        var_fbbtgat_d_db8 = 0.0;
        var_fbbtgat_d_db9 = 0.0;
        var_fbbtgat_d_db10 = 0.0;
        var_fbbtgat_d_db11 = 0.0;
        var_fbbtgat_d_db12 = 0.0;
        var_fbbtgat_d_db13 = 0.0;
        var_fbbtgat_d_db14 = 0.0;
        var_fbbtgat_d_db15 = 0.0;
        var_fbbtgat_d_db16 = 0.0;
        var_fbbtgat_d_db17 = 0.0;
        var_fbbtgat_d_db18 = 0.0;
        var_fbbtgat_d_db19 = 0.0;
        var_fbbtgat_d_db20 = 0.0;
        var_fbbtgat_d_db21 = 0.0;
        var_fbbtgat_d_db22 = 0.0;
        var_fbbtgat_d_db23 = 0.0;
        var_fbbtgat_d_db24 = 0.0;

        *var_atatbot_d_slot = var_atatbot_d;
        *var_atatgat_d_slot = var_atatgat_d;
        *var_atatsti_d_slot = var_atatsti_d;
        *var_btatpartbot_d_slot = var_btatpartbot_d;
        *var_btatpartgat_slot = var_btatpartgat;
        *var_btatpartgat_d_slot = var_btatpartgat_d;
        *var_btatpartsti_d_slot = var_btatpartsti_d;
        *var_cjobot_d_slot = var_cjobot_d;
        *var_cjogat2nd_slot = var_cjogat2nd;
        *var_cjogat_d_slot = var_cjogat_d;
        *var_cjosti_d_slot = var_cjosti_d;
        *var_deltaebot_d_slot = var_deltaebot_d;
        *var_deltaegat_d_slot = var_deltaegat_d;
        *var_deltaesti_d_slot = var_deltaesti_d;
        *var_fbbtbot_slot = var_fbbtbot;
        *var_fbbtbot_d_slot = var_fbbtbot_d;
        *var_fbbtgat_slot = var_fbbtgat;
        *var_fbbtgat_d_slot = var_fbbtgat_d;
        *var_fbbtgat_d_db0_slot = var_fbbtgat_d_db0;
        *var_fbbtgat_d_db1_slot = var_fbbtgat_d_db1;
        *var_fbbtgat_d_db10_slot = var_fbbtgat_d_db10;
        *var_fbbtgat_d_db11_slot = var_fbbtgat_d_db11;
        *var_fbbtgat_d_db12_slot = var_fbbtgat_d_db12;
        *var_fbbtgat_d_db13_slot = var_fbbtgat_d_db13;
        *var_fbbtgat_d_db14_slot = var_fbbtgat_d_db14;
        *var_fbbtgat_d_db15_slot = var_fbbtgat_d_db15;
        *var_fbbtgat_d_db16_slot = var_fbbtgat_d_db16;
        *var_fbbtgat_d_db17_slot = var_fbbtgat_d_db17;
        *var_fbbtgat_d_db18_slot = var_fbbtgat_d_db18;
        *var_fbbtgat_d_db19_slot = var_fbbtgat_d_db19;
        *var_fbbtgat_d_db2_slot = var_fbbtgat_d_db2;
        *var_fbbtgat_d_db20_slot = var_fbbtgat_d_db20;
        *var_fbbtgat_d_db21_slot = var_fbbtgat_d_db21;
        *var_fbbtgat_d_db22_slot = var_fbbtgat_d_db22;
        *var_fbbtgat_d_db23_slot = var_fbbtgat_d_db23;
        *var_fbbtgat_d_db24_slot = var_fbbtgat_d_db24;
        *var_fbbtgat_d_db3_slot = var_fbbtgat_d_db3;
        *var_fbbtgat_d_db4_slot = var_fbbtgat_d_db4;
        *var_fbbtgat_d_db5_slot = var_fbbtgat_d_db5;
        *var_fbbtgat_d_db6_slot = var_fbbtgat_d_db6;
        *var_fbbtgat_d_db7_slot = var_fbbtgat_d_db7;
        *var_fbbtgat_d_db8_slot = var_fbbtgat_d_db8;
        *var_fbbtgat_d_db9_slot = var_fbbtgat_d_db9;
        *var_fbbtgat_d_dn0_slot = var_fbbtgat_d_dn0;
        *var_fbbtgat_d_dn1_slot = var_fbbtgat_d_dn1;
        *var_fbbtgat_d_dn10_slot = var_fbbtgat_d_dn10;
        *var_fbbtgat_d_dn11_slot = var_fbbtgat_d_dn11;
        *var_fbbtgat_d_dn12_slot = var_fbbtgat_d_dn12;
        *var_fbbtgat_d_dn13_slot = var_fbbtgat_d_dn13;
        *var_fbbtgat_d_dn14_slot = var_fbbtgat_d_dn14;
        *var_fbbtgat_d_dn15_slot = var_fbbtgat_d_dn15;
        *var_fbbtgat_d_dn16_slot = var_fbbtgat_d_dn16;
        *var_fbbtgat_d_dn17_slot = var_fbbtgat_d_dn17;
        *var_fbbtgat_d_dn18_slot = var_fbbtgat_d_dn18;
        *var_fbbtgat_d_dn19_slot = var_fbbtgat_d_dn19;
        *var_fbbtgat_d_dn2_slot = var_fbbtgat_d_dn2;
        *var_fbbtgat_d_dn20_slot = var_fbbtgat_d_dn20;
        *var_fbbtgat_d_dn3_slot = var_fbbtgat_d_dn3;
        *var_fbbtgat_d_dn4_slot = var_fbbtgat_d_dn4;
        *var_fbbtgat_d_dn5_slot = var_fbbtgat_d_dn5;
        *var_fbbtgat_d_dn6_slot = var_fbbtgat_d_dn6;
        *var_fbbtgat_d_dn7_slot = var_fbbtgat_d_dn7;
        *var_fbbtgat_d_dn8_slot = var_fbbtgat_d_dn8;
        *var_fbbtgat_d_dn9_slot = var_fbbtgat_d_dn9;
        *var_fbbtgat_db0_slot = var_fbbtgat_db0;
        *var_fbbtgat_db1_slot = var_fbbtgat_db1;
        *var_fbbtgat_db10_slot = var_fbbtgat_db10;
        *var_fbbtgat_db11_slot = var_fbbtgat_db11;
        *var_fbbtgat_db12_slot = var_fbbtgat_db12;
        *var_fbbtgat_db13_slot = var_fbbtgat_db13;
        *var_fbbtgat_db14_slot = var_fbbtgat_db14;
        *var_fbbtgat_db15_slot = var_fbbtgat_db15;
        *var_fbbtgat_db16_slot = var_fbbtgat_db16;
        *var_fbbtgat_db17_slot = var_fbbtgat_db17;
        *var_fbbtgat_db18_slot = var_fbbtgat_db18;
        *var_fbbtgat_db19_slot = var_fbbtgat_db19;
        *var_fbbtgat_db2_slot = var_fbbtgat_db2;
        *var_fbbtgat_db20_slot = var_fbbtgat_db20;
        *var_fbbtgat_db21_slot = var_fbbtgat_db21;
        *var_fbbtgat_db22_slot = var_fbbtgat_db22;
        *var_fbbtgat_db23_slot = var_fbbtgat_db23;
        *var_fbbtgat_db24_slot = var_fbbtgat_db24;
        *var_fbbtgat_db3_slot = var_fbbtgat_db3;
        *var_fbbtgat_db4_slot = var_fbbtgat_db4;
        *var_fbbtgat_db5_slot = var_fbbtgat_db5;
        *var_fbbtgat_db6_slot = var_fbbtgat_db6;
        *var_fbbtgat_db7_slot = var_fbbtgat_db7;
        *var_fbbtgat_db8_slot = var_fbbtgat_db8;
        *var_fbbtgat_db9_slot = var_fbbtgat_db9;
        *var_fbbtgat_dn0_slot = var_fbbtgat_dn0;
        *var_fbbtgat_dn1_slot = var_fbbtgat_dn1;
        *var_fbbtgat_dn10_slot = var_fbbtgat_dn10;
        *var_fbbtgat_dn11_slot = var_fbbtgat_dn11;
        *var_fbbtgat_dn12_slot = var_fbbtgat_dn12;
        *var_fbbtgat_dn13_slot = var_fbbtgat_dn13;
        *var_fbbtgat_dn14_slot = var_fbbtgat_dn14;
        *var_fbbtgat_dn15_slot = var_fbbtgat_dn15;
        *var_fbbtgat_dn16_slot = var_fbbtgat_dn16;
        *var_fbbtgat_dn17_slot = var_fbbtgat_dn17;
        *var_fbbtgat_dn18_slot = var_fbbtgat_dn18;
        *var_fbbtgat_dn19_slot = var_fbbtgat_dn19;
        *var_fbbtgat_dn2_slot = var_fbbtgat_dn2;
        *var_fbbtgat_dn20_slot = var_fbbtgat_dn20;
        *var_fbbtgat_dn3_slot = var_fbbtgat_dn3;
        *var_fbbtgat_dn4_slot = var_fbbtgat_dn4;
        *var_fbbtgat_dn5_slot = var_fbbtgat_dn5;
        *var_fbbtgat_dn6_slot = var_fbbtgat_dn6;
        *var_fbbtgat_dn7_slot = var_fbbtgat_dn7;
        *var_fbbtgat_dn8_slot = var_fbbtgat_dn8;
        *var_fbbtgat_dn9_slot = var_fbbtgat_dn9;
        *var_fbbtsti_slot = var_fbbtsti;
        *var_fbbtsti_d_slot = var_fbbtsti_d;
        *var_ftdbot_d_slot = var_ftdbot_d;
        *var_ftdgat2nd_slot = var_ftdgat2nd;
        *var_ftdgat_d_slot = var_ftdgat_d;
        *var_ftdsti_d_slot = var_ftdsti_d;
        *var_guard32_slot = var_guard32;
        *var_idsatbot_d_slot = var_idsatbot_d;
        *var_idsatgat_d_slot = var_idsatgat_d;
        *var_idsatsti_d_slot = var_idsatsti_d;
        *var_phigdbot_d_slot = var_phigdbot_d;
        *var_phigdgat2nd_slot = var_phigdgat2nd;
        *var_phigdgat_d_slot = var_phigdgat_d;
        *var_phigdsti_d_slot = var_phigdsti_d;
        *var_qpref2bot_d_slot = var_qpref2bot_d;
        *var_qpref2gat2nd_slot = var_qpref2gat2nd;
        *var_qpref2gat_d_slot = var_qpref2gat_d;
        *var_qpref2sti_d_slot = var_qpref2sti_d;
        *var_qprefbot_d_slot = var_qprefbot_d;
        *var_qprefgat2nd_slot = var_qprefgat2nd;
        *var_qprefgat_d_slot = var_qprefgat_d;
        *var_qprefsti_d_slot = var_qprefsti_d;
        *var_ubibot_d_slot = var_ubibot_d;
        *var_ubigat2nd_slot = var_ubigat2nd;
        *var_ubigat_d_slot = var_ubigat_d;
        *var_ubisti_d_slot = var_ubisti_d;
        *var_vbibot_d_slot = var_vbibot_d;
        *var_vbigat2nd_slot = var_vbigat2nd;
        *var_vbigat_d_slot = var_vbigat_d;
        *var_vbiinvbot_d_slot = var_vbiinvbot_d;
        *var_vbiinvgat2nd_slot = var_vbiinvgat2nd;
        *var_vbiinvgat_d_slot = var_vbiinvgat_d;
        *var_vbiinvsti_d_slot = var_vbiinvsti_d;
        *var_vbisti_d_slot = var_vbisti_d;
    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
        var_auxt: f64,
        var_cjorgat2nd_d: f64,
        var_deltaphigd: f64,
        var_one_over_one_minus_pgat2nd_d: f64,
        var_pgat2nd_d: f64,
        var_phiggat2nd_d: f64,
        var_phigrgat2nd_d: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitrinv: f64,
        var_swgat2nd_d: f64,
        var_vbirgat2nd_d: f64,
        var_abdrain_i_slot: &mut f64,
        var_absource_i_slot: &mut f64,
        var_ad_i_slot: &mut f64,
        var_as_i_slot: &mut f64,
        var_cjogat2nd_d_slot: &mut f64,
        var_dellps_slot: &mut f64,
        var_delwod_slot: &mut f64,
        var_epsrox_p_slot: &mut f64,
        var_fbbtbot_d_slot: &mut f64,
        var_fbbtgat_d_slot: &mut f64,
        var_fbbtgat_d_db0_slot: &mut f64,
        var_fbbtgat_d_db1_slot: &mut f64,
        var_fbbtgat_d_db10_slot: &mut f64,
        var_fbbtgat_d_db11_slot: &mut f64,
        var_fbbtgat_d_db12_slot: &mut f64,
        var_fbbtgat_d_db13_slot: &mut f64,
        var_fbbtgat_d_db14_slot: &mut f64,
        var_fbbtgat_d_db15_slot: &mut f64,
        var_fbbtgat_d_db16_slot: &mut f64,
        var_fbbtgat_d_db17_slot: &mut f64,
        var_fbbtgat_d_db18_slot: &mut f64,
        var_fbbtgat_d_db19_slot: &mut f64,
        var_fbbtgat_d_db2_slot: &mut f64,
        var_fbbtgat_d_db20_slot: &mut f64,
        var_fbbtgat_d_db21_slot: &mut f64,
        var_fbbtgat_d_db22_slot: &mut f64,
        var_fbbtgat_d_db23_slot: &mut f64,
        var_fbbtgat_d_db24_slot: &mut f64,
        var_fbbtgat_d_db3_slot: &mut f64,
        var_fbbtgat_d_db4_slot: &mut f64,
        var_fbbtgat_d_db5_slot: &mut f64,
        var_fbbtgat_d_db6_slot: &mut f64,
        var_fbbtgat_d_db7_slot: &mut f64,
        var_fbbtgat_d_db8_slot: &mut f64,
        var_fbbtgat_d_db9_slot: &mut f64,
        var_fbbtgat_d_dn0_slot: &mut f64,
        var_fbbtgat_d_dn1_slot: &mut f64,
        var_fbbtgat_d_dn10_slot: &mut f64,
        var_fbbtgat_d_dn11_slot: &mut f64,
        var_fbbtgat_d_dn12_slot: &mut f64,
        var_fbbtgat_d_dn13_slot: &mut f64,
        var_fbbtgat_d_dn14_slot: &mut f64,
        var_fbbtgat_d_dn15_slot: &mut f64,
        var_fbbtgat_d_dn16_slot: &mut f64,
        var_fbbtgat_d_dn17_slot: &mut f64,
        var_fbbtgat_d_dn18_slot: &mut f64,
        var_fbbtgat_d_dn19_slot: &mut f64,
        var_fbbtgat_d_dn2_slot: &mut f64,
        var_fbbtgat_d_dn20_slot: &mut f64,
        var_fbbtgat_d_dn3_slot: &mut f64,
        var_fbbtgat_d_dn4_slot: &mut f64,
        var_fbbtgat_d_dn5_slot: &mut f64,
        var_fbbtgat_d_dn6_slot: &mut f64,
        var_fbbtgat_d_dn7_slot: &mut f64,
        var_fbbtgat_d_dn8_slot: &mut f64,
        var_fbbtgat_d_dn9_slot: &mut f64,
        var_fbbtsti_d_slot: &mut f64,
        var_ftdgat2nd_d_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard34_slot: &mut f64,
        var_iae_slot: &mut f64,
        var_iiwcv_slot: &mut f64,
        var_iiwe_slot: &mut f64,
        var_iiwecv_slot: &mut f64,
        var_il_slot: &mut f64,
        var_ile_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_iw_slot: &mut f64,
        var_iwe_slot: &mut f64,
        var_jw_i_slot: &mut f64,
        var_l_f_slot: &mut f64,
        var_l_i_slot: &mut f64,
        var_l_slif_slot: &mut f64,
        var_le_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_nf_i_slot: &mut f64,
        var_ngcon_i_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_pd_i_slot: &mut f64,
        var_phigdgat2nd_d_slot: &mut f64,
        var_ps_i_slot: &mut f64,
        var_qpref2gat2nd_d_slot: &mut f64,
        var_qprefgat2nd_d_slot: &mut f64,
        var_toxov_p_slot: &mut f64,
        var_toxovd_p_slot: &mut f64,
        var_ubigat2nd_d_slot: &mut f64,
        var_vbigat2nd_d_slot: &mut f64,
        var_vbiinvgat2nd_d_slot: &mut f64,
        var_w_f_slot: &mut f64,
        var_w_i_slot: &mut f64,
        var_wcv_slot: &mut f64,
        var_we_slot: &mut f64,
        var_wecv_slot: &mut f64,
        var_xgw_i_slot: &mut f64,
        var_xgwe_slot: &mut f64,
    ) {
        let mut var_abdrain_i: f64 = *var_abdrain_i_slot;
        let mut var_absource_i: f64 = *var_absource_i_slot;
        let mut var_ad_i: f64 = *var_ad_i_slot;
        let mut var_as_i: f64 = *var_as_i_slot;
        let mut var_cjogat2nd_d: f64 = *var_cjogat2nd_d_slot;
        let mut var_dellps: f64 = *var_dellps_slot;
        let mut var_delwod: f64 = *var_delwod_slot;
        let mut var_epsrox_p: f64 = *var_epsrox_p_slot;
        let mut var_fbbtbot_d: f64 = *var_fbbtbot_d_slot;
        let mut var_fbbtgat_d: f64 = *var_fbbtgat_d_slot;
        let mut var_fbbtgat_d_db0: f64 = *var_fbbtgat_d_db0_slot;
        let mut var_fbbtgat_d_db1: f64 = *var_fbbtgat_d_db1_slot;
        let mut var_fbbtgat_d_db10: f64 = *var_fbbtgat_d_db10_slot;
        let mut var_fbbtgat_d_db11: f64 = *var_fbbtgat_d_db11_slot;
        let mut var_fbbtgat_d_db12: f64 = *var_fbbtgat_d_db12_slot;
        let mut var_fbbtgat_d_db13: f64 = *var_fbbtgat_d_db13_slot;
        let mut var_fbbtgat_d_db14: f64 = *var_fbbtgat_d_db14_slot;
        let mut var_fbbtgat_d_db15: f64 = *var_fbbtgat_d_db15_slot;
        let mut var_fbbtgat_d_db16: f64 = *var_fbbtgat_d_db16_slot;
        let mut var_fbbtgat_d_db17: f64 = *var_fbbtgat_d_db17_slot;
        let mut var_fbbtgat_d_db18: f64 = *var_fbbtgat_d_db18_slot;
        let mut var_fbbtgat_d_db19: f64 = *var_fbbtgat_d_db19_slot;
        let mut var_fbbtgat_d_db2: f64 = *var_fbbtgat_d_db2_slot;
        let mut var_fbbtgat_d_db20: f64 = *var_fbbtgat_d_db20_slot;
        let mut var_fbbtgat_d_db21: f64 = *var_fbbtgat_d_db21_slot;
        let mut var_fbbtgat_d_db22: f64 = *var_fbbtgat_d_db22_slot;
        let mut var_fbbtgat_d_db23: f64 = *var_fbbtgat_d_db23_slot;
        let mut var_fbbtgat_d_db24: f64 = *var_fbbtgat_d_db24_slot;
        let mut var_fbbtgat_d_db3: f64 = *var_fbbtgat_d_db3_slot;
        let mut var_fbbtgat_d_db4: f64 = *var_fbbtgat_d_db4_slot;
        let mut var_fbbtgat_d_db5: f64 = *var_fbbtgat_d_db5_slot;
        let mut var_fbbtgat_d_db6: f64 = *var_fbbtgat_d_db6_slot;
        let mut var_fbbtgat_d_db7: f64 = *var_fbbtgat_d_db7_slot;
        let mut var_fbbtgat_d_db8: f64 = *var_fbbtgat_d_db8_slot;
        let mut var_fbbtgat_d_db9: f64 = *var_fbbtgat_d_db9_slot;
        let mut var_fbbtgat_d_dn0: f64 = *var_fbbtgat_d_dn0_slot;
        let mut var_fbbtgat_d_dn1: f64 = *var_fbbtgat_d_dn1_slot;
        let mut var_fbbtgat_d_dn10: f64 = *var_fbbtgat_d_dn10_slot;
        let mut var_fbbtgat_d_dn11: f64 = *var_fbbtgat_d_dn11_slot;
        let mut var_fbbtgat_d_dn12: f64 = *var_fbbtgat_d_dn12_slot;
        let mut var_fbbtgat_d_dn13: f64 = *var_fbbtgat_d_dn13_slot;
        let mut var_fbbtgat_d_dn14: f64 = *var_fbbtgat_d_dn14_slot;
        let mut var_fbbtgat_d_dn15: f64 = *var_fbbtgat_d_dn15_slot;
        let mut var_fbbtgat_d_dn16: f64 = *var_fbbtgat_d_dn16_slot;
        let mut var_fbbtgat_d_dn17: f64 = *var_fbbtgat_d_dn17_slot;
        let mut var_fbbtgat_d_dn18: f64 = *var_fbbtgat_d_dn18_slot;
        let mut var_fbbtgat_d_dn19: f64 = *var_fbbtgat_d_dn19_slot;
        let mut var_fbbtgat_d_dn2: f64 = *var_fbbtgat_d_dn2_slot;
        let mut var_fbbtgat_d_dn20: f64 = *var_fbbtgat_d_dn20_slot;
        let mut var_fbbtgat_d_dn3: f64 = *var_fbbtgat_d_dn3_slot;
        let mut var_fbbtgat_d_dn4: f64 = *var_fbbtgat_d_dn4_slot;
        let mut var_fbbtgat_d_dn5: f64 = *var_fbbtgat_d_dn5_slot;
        let mut var_fbbtgat_d_dn6: f64 = *var_fbbtgat_d_dn6_slot;
        let mut var_fbbtgat_d_dn7: f64 = *var_fbbtgat_d_dn7_slot;
        let mut var_fbbtgat_d_dn8: f64 = *var_fbbtgat_d_dn8_slot;
        let mut var_fbbtgat_d_dn9: f64 = *var_fbbtgat_d_dn9_slot;
        let mut var_fbbtsti_d: f64 = *var_fbbtsti_d_slot;
        let mut var_ftdgat2nd_d: f64 = *var_ftdgat2nd_d_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard34: f64 = *var_guard34_slot;
        let mut var_iae: f64 = *var_iae_slot;
        let mut var_iiwcv: f64 = *var_iiwcv_slot;
        let mut var_iiwe: f64 = *var_iiwe_slot;
        let mut var_iiwecv: f64 = *var_iiwecv_slot;
        let mut var_il: f64 = *var_il_slot;
        let mut var_ile: f64 = *var_ile_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_iw: f64 = *var_iw_slot;
        let mut var_iwe: f64 = *var_iwe_slot;
        let mut var_jw_i: f64 = *var_jw_i_slot;
        let mut var_l_f: f64 = *var_l_f_slot;
        let mut var_l_i: f64 = *var_l_i_slot;
        let mut var_l_slif: f64 = *var_l_slif_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_nf_i: f64 = *var_nf_i_slot;
        let mut var_ngcon_i: f64 = *var_ngcon_i_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_pd_i: f64 = *var_pd_i_slot;
        let mut var_phigdgat2nd_d: f64 = *var_phigdgat2nd_d_slot;
        let mut var_ps_i: f64 = *var_ps_i_slot;
        let mut var_qpref2gat2nd_d: f64 = *var_qpref2gat2nd_d_slot;
        let mut var_qprefgat2nd_d: f64 = *var_qprefgat2nd_d_slot;
        let mut var_toxov_p: f64 = *var_toxov_p_slot;
        let mut var_toxovd_p: f64 = *var_toxovd_p_slot;
        let mut var_ubigat2nd_d: f64 = *var_ubigat2nd_d_slot;
        let mut var_vbigat2nd_d: f64 = *var_vbigat2nd_d_slot;
        let mut var_vbiinvgat2nd_d: f64 = *var_vbiinvgat2nd_d_slot;
        let mut var_w_f: f64 = *var_w_f_slot;
        let mut var_w_i: f64 = *var_w_i_slot;
        let mut var_wcv: f64 = *var_wcv_slot;
        let mut var_we: f64 = *var_we_slot;
        let mut var_wecv: f64 = *var_wecv_slot;
        let mut var_xgw_i: f64 = *var_xgw_i_slot;
        let mut var_xgwe: f64 = *var_xgwe_slot;

        let (assign3300_e3507,) = {
    if (var_fbbtbot_d > 0.0) {
        (var_fbbtbot_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot_d = assign3300_e3507;

        let (assign3310_e3513,) = {
    if (var_fbbtsti_d > 0.0) {
        (var_fbbtsti_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti_d = assign3310_e3513;

        let (assign3320_e3519, assign3320_e3519_d_n0, assign3320_e3519_d_n1, assign3320_e3519_d_n2, assign3320_e3519_d_n3, assign3320_e3519_d_n4, assign3320_e3519_d_n5, assign3320_e3519_d_n6, assign3320_e3519_d_n7, assign3320_e3519_d_n8, assign3320_e3519_d_n9, assign3320_e3519_d_n10, assign3320_e3519_d_n11, assign3320_e3519_d_n12, assign3320_e3519_d_n13, assign3320_e3519_d_n14, assign3320_e3519_d_n15, assign3320_e3519_d_n16, assign3320_e3519_d_n17, assign3320_e3519_d_n18, assign3320_e3519_d_n19, assign3320_e3519_d_n20, assign3320_e3519_d_b0, assign3320_e3519_d_b1, assign3320_e3519_d_b2, assign3320_e3519_d_b3, assign3320_e3519_d_b4, assign3320_e3519_d_b5, assign3320_e3519_d_b6, assign3320_e3519_d_b7, assign3320_e3519_d_b8, assign3320_e3519_d_b9, assign3320_e3519_d_b10, assign3320_e3519_d_b11, assign3320_e3519_d_b12, assign3320_e3519_d_b13, assign3320_e3519_d_b14, assign3320_e3519_d_b15, assign3320_e3519_d_b16, assign3320_e3519_d_b17, assign3320_e3519_d_b18, assign3320_e3519_d_b19, assign3320_e3519_d_b20, assign3320_e3519_d_b21, assign3320_e3519_d_b22, assign3320_e3519_d_b23, assign3320_e3519_d_b24,) = {
    if (var_fbbtgat_d > 0.0) {
        (var_fbbtgat_d, var_fbbtgat_d_dn0, var_fbbtgat_d_dn1, var_fbbtgat_d_dn2, var_fbbtgat_d_dn3, var_fbbtgat_d_dn4, var_fbbtgat_d_dn5, var_fbbtgat_d_dn6, var_fbbtgat_d_dn7, var_fbbtgat_d_dn8, var_fbbtgat_d_dn9, var_fbbtgat_d_dn10, var_fbbtgat_d_dn11, var_fbbtgat_d_dn12, var_fbbtgat_d_dn13, var_fbbtgat_d_dn14, var_fbbtgat_d_dn15, var_fbbtgat_d_dn16, var_fbbtgat_d_dn17, var_fbbtgat_d_dn18, var_fbbtgat_d_dn19, var_fbbtgat_d_dn20, var_fbbtgat_d_db0, var_fbbtgat_d_db1, var_fbbtgat_d_db2, var_fbbtgat_d_db3, var_fbbtgat_d_db4, var_fbbtgat_d_db5, var_fbbtgat_d_db6, var_fbbtgat_d_db7, var_fbbtgat_d_db8, var_fbbtgat_d_db9, var_fbbtgat_d_db10, var_fbbtgat_d_db11, var_fbbtgat_d_db12, var_fbbtgat_d_db13, var_fbbtgat_d_db14, var_fbbtgat_d_db15, var_fbbtgat_d_db16, var_fbbtgat_d_db17, var_fbbtgat_d_db18, var_fbbtgat_d_db19, var_fbbtgat_d_db20, var_fbbtgat_d_db21, var_fbbtgat_d_db22, var_fbbtgat_d_db23, var_fbbtgat_d_db24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat_d = assign3320_e3519;
        var_fbbtgat_d_dn0 = assign3320_e3519_d_n0;
        var_fbbtgat_d_dn1 = assign3320_e3519_d_n1;
        var_fbbtgat_d_dn2 = assign3320_e3519_d_n2;
        var_fbbtgat_d_dn3 = assign3320_e3519_d_n3;
        var_fbbtgat_d_dn4 = assign3320_e3519_d_n4;
        var_fbbtgat_d_dn5 = assign3320_e3519_d_n5;
        var_fbbtgat_d_dn6 = assign3320_e3519_d_n6;
        var_fbbtgat_d_dn7 = assign3320_e3519_d_n7;
        var_fbbtgat_d_dn8 = assign3320_e3519_d_n8;
        var_fbbtgat_d_dn9 = assign3320_e3519_d_n9;
        var_fbbtgat_d_dn10 = assign3320_e3519_d_n10;
        var_fbbtgat_d_dn11 = assign3320_e3519_d_n11;
        var_fbbtgat_d_dn12 = assign3320_e3519_d_n12;
        var_fbbtgat_d_dn13 = assign3320_e3519_d_n13;
        var_fbbtgat_d_dn14 = assign3320_e3519_d_n14;
        var_fbbtgat_d_dn15 = assign3320_e3519_d_n15;
        var_fbbtgat_d_dn16 = assign3320_e3519_d_n16;
        var_fbbtgat_d_dn17 = assign3320_e3519_d_n17;
        var_fbbtgat_d_dn18 = assign3320_e3519_d_n18;
        var_fbbtgat_d_dn19 = assign3320_e3519_d_n19;
        var_fbbtgat_d_dn20 = assign3320_e3519_d_n20;
        var_fbbtgat_d_db0 = assign3320_e3519_d_b0;
        var_fbbtgat_d_db1 = assign3320_e3519_d_b1;
        var_fbbtgat_d_db2 = assign3320_e3519_d_b2;
        var_fbbtgat_d_db3 = assign3320_e3519_d_b3;
        var_fbbtgat_d_db4 = assign3320_e3519_d_b4;
        var_fbbtgat_d_db5 = assign3320_e3519_d_b5;
        var_fbbtgat_d_db6 = assign3320_e3519_d_b6;
        var_fbbtgat_d_db7 = assign3320_e3519_d_b7;
        var_fbbtgat_d_db8 = assign3320_e3519_d_b8;
        var_fbbtgat_d_db9 = assign3320_e3519_d_b9;
        var_fbbtgat_d_db10 = assign3320_e3519_d_b10;
        var_fbbtgat_d_db11 = assign3320_e3519_d_b11;
        var_fbbtgat_d_db12 = assign3320_e3519_d_b12;
        var_fbbtgat_d_db13 = assign3320_e3519_d_b13;
        var_fbbtgat_d_db14 = assign3320_e3519_d_b14;
        var_fbbtgat_d_db15 = assign3320_e3519_d_b15;
        var_fbbtgat_d_db16 = assign3320_e3519_d_b16;
        var_fbbtgat_d_db17 = assign3320_e3519_d_b17;
        var_fbbtgat_d_db18 = assign3320_e3519_d_b18;
        var_fbbtgat_d_db19 = assign3320_e3519_d_b19;
        var_fbbtgat_d_db20 = assign3320_e3519_d_b20;
        var_fbbtgat_d_db21 = assign3320_e3519_d_b21;
        var_fbbtgat_d_db22 = assign3320_e3519_d_b22;
        var_fbbtgat_d_db23 = assign3320_e3519_d_b23;
        var_fbbtgat_d_db24 = assign3320_e3519_d_b24;

        let assign3330_e3522: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard33 = assign3330_e3522;

        let (assign3340_e3528,) = {
    if (var_guard33 != 0.0) {
        let assign3340_e3526: f64 = (var_phiggat2nd_d + var_deltaphigd);
        (assign3340_e3526,)
    } else {
        (var_phigdgat2nd_d,)
    }
};
        var_phigdgat2nd_d = assign3340_e3528;

        let (assign3350_e3545,) = {
    if (var_guard33 != 0.0) {
        let assign3350_e3532: f64 = (var_auxt).powf(1.5);
        let assign3350_e3536: f64 = (var_phigrgat2nd_d * var_phitrinv);
        let assign3350_e3539: f64 = (var_phigdgat2nd_d * var_phitdinv);
        let assign3350_e3540: f64 = (assign3350_e3536 - assign3350_e3539);
        let assign3350_e3541: f64 = (0.5 * assign3350_e3540);
        let assign3350_e3542: f64 = (assign3350_e3541).exp();
        let assign3350_e3543: f64 = (assign3350_e3532 * assign3350_e3542);
        (assign3350_e3543,)
    } else {
        (var_ftdgat2nd_d,)
    }
};
        var_ftdgat2nd_d = assign3350_e3545;

        let (assign3360_e3558,) = {
    if (var_guard33 != 0.0) {
        let assign3360_e3549: f64 = (var_vbirgat2nd_d * var_auxt);
        let assign3360_e3552: f64 = (2.0 * var_phitd);
        let assign3360_e3554: f64 = (var_ftdgat2nd_d).ln();
        let assign3360_e3555: f64 = (assign3360_e3552 * assign3360_e3554);
        let assign3360_e3556: f64 = (assign3360_e3549 - assign3360_e3555);
        (assign3360_e3556,)
    } else {
        (var_ubigat2nd_d,)
    }
};
        var_ubigat2nd_d = assign3360_e3558;

        let (assign3370_e3574,) = {
    if (var_guard33 != 0.0) {
        let assign3370_e3565: f64 = (0.05 - var_ubigat2nd_d);
        let assign3370_e3567: f64 = (assign3370_e3565 * var_phitdinv);
        let assign3370_e3568: f64 = (assign3370_e3567).exp();
        let assign3370_e3569: f64 = (1.0 + assign3370_e3568);
        let assign3370_e3570: f64 = (assign3370_e3569).ln();
        let assign3370_e3571: f64 = (var_phitd * assign3370_e3570);
        let assign3370_e3572: f64 = (var_ubigat2nd_d + assign3370_e3571);
        (assign3370_e3572,)
    } else {
        (var_vbigat2nd_d,)
    }
};
        var_vbigat2nd_d = assign3370_e3574;

        let (assign3380_e3580,) = {
    if (var_guard33 != 0.0) {
        let assign3380_e3578: f64 = (1.0 / var_vbigat2nd_d);
        (assign3380_e3578,)
    } else {
        (var_vbiinvgat2nd_d,)
    }
};
        var_vbiinvgat2nd_d = assign3380_e3580;

        let (assign3390_e3590,) = {
    if (var_guard33 != 0.0) {
        let assign3390_e3585: f64 = (var_vbirgat2nd_d * var_vbiinvgat2nd_d);
        let assign3390_e3587: f64 = (assign3390_e3585).powf(var_pgat2nd_d);
        let assign3390_e3588: f64 = (var_cjorgat2nd_d * assign3390_e3587);
        (assign3390_e3588,)
    } else {
        (var_cjogat2nd_d,)
    }
};
        var_cjogat2nd_d = assign3390_e3590;

        let (assign3400_e3598,) = {
    if (var_guard33 != 0.0) {
        let assign3400_e3594: f64 = (var_cjogat2nd_d * var_vbigat2nd_d);
        let assign3400_e3596: f64 = (assign3400_e3594 * var_one_over_one_minus_pgat2nd_d);
        (assign3400_e3596,)
    } else {
        (var_qprefgat2nd_d,)
    }
};
        var_qprefgat2nd_d = assign3400_e3598;

        let (assign3410_e3604,) = {
    if (var_guard33 != 0.0) {
        let assign3410_e3602: f64 = (2.0 * var_cjogat2nd_d);
        (assign3410_e3602,)
    } else {
        (var_qpref2gat2nd_d,)
    }
};
        var_qpref2gat2nd_d = assign3410_e3604;

        var_nf_i = 1.0;

        var_invnf = 1.0;

        var_le = 0.0;

        var_we = 0.0;

        var_l_i = p.p0;

        var_w_i = p.p1;

        s.store_scalar(9, p.p2);

        s.store_scalar(10, p.p3);

        s.store_scalar(11, p.p4);

        s.store_scalar(12, p.p8);

        var_xgw_i = p.p11;

        var_absource_i = p.p19;

        var_lssource_i = p.p20;

        var_lgsource_i = p.p21;

        var_abdrain_i = p.p22;

        var_lsdrain_i = p.p23;

        var_lgdrain_i = p.p24;

        var_as_i = p.p25;

        var_ps_i = p.p26;

        var_ad_i = p.p27;

        var_pd_i = p.p28;

        var_jw_i = p.p14;

        let assign3640_e3629: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard34 = assign3640_e3629;

        let (assign3650_e3638,) = {
    if (var_guard34 != 0.0) {
        let (assign3650_e3636,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3650_e3636,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3650_e3638;

        let (assign3660_e3645,) = {
    if (var_guard34 != 0.0) {
        let assign3660_e3642: f64 = (var_nf_i + 0.5);
        let assign3660_e3643: f64 = (assign3660_e3642).floor();
        (assign3660_e3643,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3660_e3645;

        let (assign3670_e3651,) = {
    if (var_guard34 != 0.0) {
        let assign3670_e3649: f64 = (1.0 / var_nf_i);
        (assign3670_e3649,)
    } else {
        (var_invnf,)
    }
};
        var_invnf = assign3670_e3651;

        let assign3680_e3654: f64 = (var_w_i * var_invnf);
        let (assign3680_e3661,) = {
    if (assign3680_e3654 > 1e-9) {
        let assign3680_e3659: f64 = (var_w_i * var_invnf);
        (assign3680_e3659,)
    } else {
        (1e-9,)
    }
};
        var_w_i = assign3680_e3661;

        s.store_scalar(15, p.p5);

        s.store_scalar(16, p.p6);

        s.store_scalar(17, p.p7);

        let (assign3720_e3670,) = {
    if (p.p10 < 1.5) {
        (1.0,)
    } else {
        (2.0,)
    }
};
        var_ngcon_i = assign3720_e3670;

        let assign3730_e3673: f64 = (1e-6 / var_l_i);
        var_il = assign3730_e3673;

        let assign3740_e3676: f64 = (1e-6 / var_w_i);
        var_iw = assign3740_e3676;

        let assign3750_e3681: f64 = (p.p189 * var_il);
        let assign3750_e3682: f64 = (1.0 + assign3750_e3681);
        let assign3750_e3683: f64 = (p.p188 * assign3750_e3682);
        let assign3750_e3687: f64 = (p.p190 * var_iw);
        let assign3750_e3688: f64 = (1.0 + assign3750_e3687);
        let assign3750_e3689: f64 = (assign3750_e3683 * assign3750_e3688);
        var_dellps = assign3750_e3689;

        let assign3760_e3694: f64 = (p.p193 * var_il);
        let assign3760_e3695: f64 = (1.0 + assign3760_e3694);
        let assign3760_e3696: f64 = (p.p192 * assign3760_e3695);
        let assign3760_e3700: f64 = (p.p194 * var_iw);
        let assign3760_e3701: f64 = (1.0 + assign3760_e3700);
        let assign3760_e3702: f64 = (assign3760_e3696 * assign3760_e3701);
        var_delwod = assign3760_e3702;

        let assign3770_e3705: f64 = (var_l_i + var_dellps);
        let assign3770_e3708: f64 = (2.0 * p.p191);
        let assign3770_e3709: f64 = (assign3770_e3705 - assign3770_e3708);
        let (assign3770_e3720,) = {
    if (assign3770_e3709 > 1e-9) {
        let assign3770_e3714: f64 = (var_l_i + var_dellps);
        let assign3770_e3717: f64 = (2.0 * p.p191);
        let assign3770_e3718: f64 = (assign3770_e3714 - assign3770_e3717);
        (assign3770_e3718,)
    } else {
        (1e-9,)
    }
};
        var_le = assign3770_e3720;

        let assign3780_e3723: f64 = (var_w_i + var_delwod);
        let assign3780_e3726: f64 = (2.0 * p.p195);
        let assign3780_e3727: f64 = (assign3780_e3723 - assign3780_e3726);
        let (assign3780_e3738,) = {
    if (assign3780_e3727 > 1e-9) {
        let assign3780_e3732: f64 = (var_w_i + var_delwod);
        let assign3780_e3735: f64 = (2.0 * p.p195);
        let assign3780_e3736: f64 = (assign3780_e3732 - assign3780_e3735);
        (assign3780_e3736,)
    } else {
        (1e-9,)
    }
};
        var_we = assign3780_e3738;

        let assign3790_e3741: f64 = (1e-6 / var_le);
        var_ile = assign3790_e3741;

        s.store_scalar(315, (var_ile * var_ile));

        let assign3810_e3747: f64 = (1e-6 / var_we);
        var_iwe = assign3810_e3747;

        let assign3820_e3750: f64 = (1.0 / var_iwe);
        var_iiwe = assign3820_e3750;

        let assign3830_e3753: f64 = (var_ile * var_iwe);
        var_iae = assign3830_e3753;

        s.store_scalar(319, (1.0 / var_iae));

        s.store_scalar(320, (if ((((s.v[7] + s.v[310]) - (2.0 * p.p191)) + p.p196) > 1e-9) { (((var_l_i + var_dellps) - (2.0 * p.p191)) + p.p196) } else { 1e-9 }));

        let assign3860_e3781: f64 = (var_w_i + var_delwod);
        let assign3860_e3784: f64 = (2.0 * p.p195);
        let assign3860_e3785: f64 = (assign3860_e3781 - assign3860_e3784);
        let assign3860_e3787: f64 = (assign3860_e3785 + p.p197);
        let (assign3860_e3800,) = {
    if (assign3860_e3787 > 1e-9) {
        let assign3860_e3792: f64 = (var_w_i + var_delwod);
        let assign3860_e3795: f64 = (2.0 * p.p195);
        let assign3860_e3796: f64 = (assign3860_e3792 - assign3860_e3795);
        let assign3860_e3798: f64 = (assign3860_e3796 + p.p197);
        (assign3860_e3798,)
    } else {
        (1e-9,)
    }
};
        var_wecv = assign3860_e3800;

        let assign3870_e3803: f64 = (var_wecv / 1e-6);
        var_iiwecv = assign3870_e3803;

        s.store_scalar(323, (if (((s.v[7] + s.v[310]) + p.p196) > 1e-9) { ((var_l_i + var_dellps) + p.p196) } else { 1e-9 }));

        let assign3890_e3820: f64 = (var_w_i + var_delwod);
        let assign3890_e3822: f64 = (assign3890_e3820 + p.p197);
        let (assign3890_e3831,) = {
    if (assign3890_e3822 > 1e-9) {
        let assign3890_e3827: f64 = (var_w_i + var_delwod);
        let assign3890_e3829: f64 = (assign3890_e3827 + p.p197);
        (assign3890_e3829,)
    } else {
        (1e-9,)
    }
};
        var_wcv = assign3890_e3831;

        s.store_scalar(325, (s.v[323] / 1e-6));

        let assign3910_e3837: f64 = (var_wcv / 1e-6);
        var_iiwcv = assign3910_e3837;

        let assign3920_e3840: f64 = (var_l_i + var_dellps);
        let (assign3920_e3847,) = {
    if (assign3920_e3840 > 1e-9) {
        let assign3920_e3845: f64 = (var_l_i + var_dellps);
        (assign3920_e3845,)
    } else {
        (1e-9,)
    }
};
        var_l_f = assign3920_e3847;

        let assign3930_e3850: f64 = (var_l_f + p.p443);
        let (assign3930_e3857,) = {
    if (assign3930_e3850 > 1e-9) {
        let assign3930_e3855: f64 = (var_l_f + p.p443);
        (assign3930_e3855,)
    } else {
        (1e-9,)
    }
};
        var_l_slif = assign3930_e3857;

        let assign3940_e3860: f64 = (var_w_i + var_delwod);
        let (assign3940_e3867,) = {
    if (assign3940_e3860 > 1e-9) {
        let assign3940_e3865: f64 = (var_w_i + var_delwod);
        (assign3940_e3865,)
    } else {
        (1e-9,)
    }
};
        var_w_f = assign3940_e3867;

        let assign3950_e3871: f64 = (0.5 * var_delwod);
        let assign3950_e3872: f64 = (var_xgw_i - assign3950_e3871);
        let (assign3950_e3881,) = {
    if (assign3950_e3872 > 1e-9) {
        let assign3950_e3878: f64 = (0.5 * var_delwod);
        let assign3950_e3879: f64 = (var_xgw_i - assign3950_e3878);
        (assign3950_e3879,)
    } else {
        (1e-9,)
    }
};
        var_xgwe = assign3950_e3881;

        s.store_scalar(44, p.p57);

        s.store_scalar(45, p.p58);

        s.store_scalar(46, p.p59);

        s.store_scalar(47, p.p60);

        var_epsrox_p = p.p61;

        s.store_scalar(49, p.p62);

        s.store_scalar(50, p.p63);

        s.store_scalar(51, p.p64);

        s.store_scalar(52, p.p65);

        s.store_scalar(53, p.p66);

        s.store_scalar(54, p.p67);

        var_toxov_p = p.p68;

        var_toxovd_p = p.p69;

        var_nov_p = p.p70;

        var_novd_p = p.p71;

        s.store_scalar(55, p.p72);

        s.store_scalar(56, p.p74);

        s.store_scalar(57, p.p73);

        s.store_scalar(58, p.p75);

        s.store_scalar(63, p.p79);

        s.store_scalar(64, p.p81);

        s.store_scalar(65, p.p80);

        s.store_scalar(66, p.p76);

        s.store_scalar(67, p.p78);

        s.store_scalar(68, p.p77);

        s.store_scalar(69, p.p82);

        s.store_scalar(70, p.p83);

        s.store_scalar(71, p.p84);

        s.store_scalar(72, p.p85);

        s.store_scalar(73, p.p86);

        s.store_scalar(74, p.p87);

        s.store_scalar(75, p.p88);

        s.store_scalar(76, p.p89);

        s.store_scalar(77, p.p90);

        s.store_scalar(78, p.p91);

        *var_abdrain_i_slot = var_abdrain_i;
        *var_absource_i_slot = var_absource_i;
        *var_ad_i_slot = var_ad_i;
        *var_as_i_slot = var_as_i;
        *var_cjogat2nd_d_slot = var_cjogat2nd_d;
        *var_dellps_slot = var_dellps;
        *var_delwod_slot = var_delwod;
        *var_epsrox_p_slot = var_epsrox_p;
        *var_fbbtbot_d_slot = var_fbbtbot_d;
        *var_fbbtgat_d_slot = var_fbbtgat_d;
        *var_fbbtgat_d_db0_slot = var_fbbtgat_d_db0;
        *var_fbbtgat_d_db1_slot = var_fbbtgat_d_db1;
        *var_fbbtgat_d_db10_slot = var_fbbtgat_d_db10;
        *var_fbbtgat_d_db11_slot = var_fbbtgat_d_db11;
        *var_fbbtgat_d_db12_slot = var_fbbtgat_d_db12;
        *var_fbbtgat_d_db13_slot = var_fbbtgat_d_db13;
        *var_fbbtgat_d_db14_slot = var_fbbtgat_d_db14;
        *var_fbbtgat_d_db15_slot = var_fbbtgat_d_db15;
        *var_fbbtgat_d_db16_slot = var_fbbtgat_d_db16;
        *var_fbbtgat_d_db17_slot = var_fbbtgat_d_db17;
        *var_fbbtgat_d_db18_slot = var_fbbtgat_d_db18;
        *var_fbbtgat_d_db19_slot = var_fbbtgat_d_db19;
        *var_fbbtgat_d_db2_slot = var_fbbtgat_d_db2;
        *var_fbbtgat_d_db20_slot = var_fbbtgat_d_db20;
        *var_fbbtgat_d_db21_slot = var_fbbtgat_d_db21;
        *var_fbbtgat_d_db22_slot = var_fbbtgat_d_db22;
        *var_fbbtgat_d_db23_slot = var_fbbtgat_d_db23;
        *var_fbbtgat_d_db24_slot = var_fbbtgat_d_db24;
        *var_fbbtgat_d_db3_slot = var_fbbtgat_d_db3;
        *var_fbbtgat_d_db4_slot = var_fbbtgat_d_db4;
        *var_fbbtgat_d_db5_slot = var_fbbtgat_d_db5;
        *var_fbbtgat_d_db6_slot = var_fbbtgat_d_db6;
        *var_fbbtgat_d_db7_slot = var_fbbtgat_d_db7;
        *var_fbbtgat_d_db8_slot = var_fbbtgat_d_db8;
        *var_fbbtgat_d_db9_slot = var_fbbtgat_d_db9;
        *var_fbbtgat_d_dn0_slot = var_fbbtgat_d_dn0;
        *var_fbbtgat_d_dn1_slot = var_fbbtgat_d_dn1;
        *var_fbbtgat_d_dn10_slot = var_fbbtgat_d_dn10;
        *var_fbbtgat_d_dn11_slot = var_fbbtgat_d_dn11;
        *var_fbbtgat_d_dn12_slot = var_fbbtgat_d_dn12;
        *var_fbbtgat_d_dn13_slot = var_fbbtgat_d_dn13;
        *var_fbbtgat_d_dn14_slot = var_fbbtgat_d_dn14;
        *var_fbbtgat_d_dn15_slot = var_fbbtgat_d_dn15;
        *var_fbbtgat_d_dn16_slot = var_fbbtgat_d_dn16;
        *var_fbbtgat_d_dn17_slot = var_fbbtgat_d_dn17;
        *var_fbbtgat_d_dn18_slot = var_fbbtgat_d_dn18;
        *var_fbbtgat_d_dn19_slot = var_fbbtgat_d_dn19;
        *var_fbbtgat_d_dn2_slot = var_fbbtgat_d_dn2;
        *var_fbbtgat_d_dn20_slot = var_fbbtgat_d_dn20;
        *var_fbbtgat_d_dn3_slot = var_fbbtgat_d_dn3;
        *var_fbbtgat_d_dn4_slot = var_fbbtgat_d_dn4;
        *var_fbbtgat_d_dn5_slot = var_fbbtgat_d_dn5;
        *var_fbbtgat_d_dn6_slot = var_fbbtgat_d_dn6;
        *var_fbbtgat_d_dn7_slot = var_fbbtgat_d_dn7;
        *var_fbbtgat_d_dn8_slot = var_fbbtgat_d_dn8;
        *var_fbbtgat_d_dn9_slot = var_fbbtgat_d_dn9;
        *var_fbbtsti_d_slot = var_fbbtsti_d;
        *var_ftdgat2nd_d_slot = var_ftdgat2nd_d;
        *var_guard33_slot = var_guard33;
        *var_guard34_slot = var_guard34;
        *var_iae_slot = var_iae;
        *var_iiwcv_slot = var_iiwcv;
        *var_iiwe_slot = var_iiwe;
        *var_iiwecv_slot = var_iiwecv;
        *var_il_slot = var_il;
        *var_ile_slot = var_ile;
        *var_invnf_slot = var_invnf;
        *var_iw_slot = var_iw;
        *var_iwe_slot = var_iwe;
        *var_jw_i_slot = var_jw_i;
        *var_l_f_slot = var_l_f;
        *var_l_i_slot = var_l_i;
        *var_l_slif_slot = var_l_slif;
        *var_le_slot = var_le;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lssource_i_slot = var_lssource_i;
        *var_nf_i_slot = var_nf_i;
        *var_ngcon_i_slot = var_ngcon_i;
        *var_nov_p_slot = var_nov_p;
        *var_novd_p_slot = var_novd_p;
        *var_pd_i_slot = var_pd_i;
        *var_phigdgat2nd_d_slot = var_phigdgat2nd_d;
        *var_ps_i_slot = var_ps_i;
        *var_qpref2gat2nd_d_slot = var_qpref2gat2nd_d;
        *var_qprefgat2nd_d_slot = var_qprefgat2nd_d;
        *var_toxov_p_slot = var_toxov_p;
        *var_toxovd_p_slot = var_toxovd_p;
        *var_ubigat2nd_d_slot = var_ubigat2nd_d;
        *var_vbigat2nd_d_slot = var_vbigat2nd_d;
        *var_vbiinvgat2nd_d_slot = var_vbiinvgat2nd_d;
        *var_w_f_slot = var_w_f;
        *var_w_i_slot = var_w_i;
        *var_wcv_slot = var_wcv;
        *var_we_slot = var_we;
        *var_wecv_slot = var_wecv;
        *var_xgw_i_slot = var_xgw_i;
        *var_xgwe_slot = var_xgwe;
    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_agidl_p_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_epsrox_p_slot: &mut f64,
        var_guard41_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_rbulk_p_slot: &mut f64,
        var_rde_p_slot: &mut f64,
        var_rg_p_slot: &mut f64,
        var_rjund_p_slot: &mut f64,
        var_rjuns_p_slot: &mut f64,
        var_rse_p_slot: &mut f64,
        var_rwell_p_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_toxov_p_slot: &mut f64,
        var_toxovd_p_slot: &mut f64,
    ) {
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_epsrox_p: f64 = *var_epsrox_p_slot;
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_rbulk_p: f64 = *var_rbulk_p_slot;
        let mut var_rde_p: f64 = *var_rde_p_slot;
        let mut var_rg_p: f64 = *var_rg_p_slot;
        let mut var_rjund_p: f64 = *var_rjund_p_slot;
        let mut var_rjuns_p: f64 = *var_rjuns_p_slot;
        let mut var_rse_p: f64 = *var_rse_p_slot;
        let mut var_rwell_p: f64 = *var_rwell_p_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_toxov_p: f64 = *var_toxov_p_slot;
        let mut var_toxovd_p: f64 = *var_toxovd_p_slot;

        s.store_scalar(79, p.p92);

        s.store_scalar(80, p.p93);

        s.store_scalar(81, p.p94);

        s.store_scalar(82, p.p95);

        s.store_scalar(83, p.p96);

        s.store_scalar(84, p.p97);

        s.store_scalar(85, p.p98);

        s.store_scalar(86, p.p99);

        s.store_scalar(87, p.p100);

        s.store_scalar(88, p.p101);

        s.store_scalar(89, p.p102);

        s.store_scalar(90, p.p103);

        s.store_scalar(91, p.p104);

        s.store_scalar(92, p.p105);

        s.store_scalar(93, p.p106);

        s.store_scalar(94, p.p107);

        s.store_scalar(95, p.p108);

        s.store_scalar(96, p.p109);

        s.store_scalar(97, p.p110);

        s.store_scalar(98, p.p111);

        s.store_scalar(99, p.p112);

        s.store_scalar(100, p.p113);

        s.store_scalar(101, p.p114);

        s.store_scalar(102, p.p115);

        s.store_scalar(103, p.p116);

        var_igov_p = p.p117;

        var_igovd_p = p.p118;

        var_stig_p = p.p119;

        s.store_scalar(107, p.p120);

        s.store_scalar(108, p.p121);

        s.store_scalar(109, p.p120);

        s.b[1024] = param_given[122];
        s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });

        if s.b[1024] {
            s.store_scalar(109, p.p122);
        }

        s.store_scalar(110, p.p121);

        s.b[1025] = param_given[123];
        s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });

        if s.b[1025] {
            s.store_scalar(110, p.p123);
        }

        s.copy_ad(111, 109);

        s.b[1026] = param_given[124];
        s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });

        if s.b[1026] {
            s.store_scalar(111, p.p124);
        }

        s.copy_ad(112, 110);

        s.b[1027] = param_given[125];
        s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });

        if s.b[1027] {
            s.store_scalar(112, p.p125);
        }

        s.store_scalar(113, p.p126);

        var_agidl_p = p.p127;

        var_agidld_p = p.p128;

        s.store_scalar(116, p.p129);

        s.store_scalar(117, p.p130);

        s.store_scalar(118, p.p131);

        s.store_scalar(119, p.p132);

        s.store_scalar(120, p.p133);

        s.store_scalar(121, p.p134);

        s.store_scalar(122, p.p135);

        s.store_scalar(123, p.p136);

        s.store_scalar(124, p.p137);

        s.store_scalar(125, p.p99);

        s.b[1028] = param_given[138];
        s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });

        if s.b[1028] {
            s.store_scalar(125, p.p138);
        }

        s.store_scalar(126, p.p104);

        s.b[1029] = param_given[139];
        s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });

        if s.b[1029] {
            s.store_scalar(126, p.p139);
        }

        s.store_scalar(127, p.p140);

        s.store_scalar(128, p.p141);

        var_cgov_p = p.p142;

        var_cgovd_p = p.p143;

        s.store_scalar(131, p.p144);

        s.store_scalar(132, p.p145);

        s.store_scalar(133, p.p146);

        s.store_scalar(134, p.p147);

        s.store_scalar(135, p.p148);

        s.store_scalar(136, p.p149);

        s.store_scalar(137, p.p150);

        s.store_scalar(138, p.p151);

        s.store_scalar(139, p.p152);

        s.store_scalar(140, p.p153);

        var_cfr_p = p.p154;

        var_cfrd_p = p.p155;

        s.store_scalar(143, p.p156);

        s.store_scalar(144, p.p157);

        s.store_scalar(149, p.p162);

        s.store_scalar(150, p.p163);

        s.store_scalar(151, p.p164);

        s.store_scalar(152, p.p165);

        s.store_scalar(153, p.p166);

        s.store_scalar(154, p.p167);

        s.store_scalar(155, p.p168);

        s.store_scalar(156, p.p169);

        s.store_scalar(157, p.p170);

        s.store_scalar(158, p.p171);

        s.store_scalar(159, p.p172);

        s.store_scalar(160, p.p174);

        s.store_scalar(161, p.p173);

        var_rg_p = p.p180;

        var_rse_p = p.p181;

        var_rde_p = p.p182;

        var_rwell_p = p.p184;

        var_rbulk_p = p.p183;

        var_rjuns_p = p.p185;

        var_rjund_p = p.p186;

        s.store_scalar(176, p.p187);

        let assign5390_e4063: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard41 = assign5390_e4063;

        if (s.v[1030] != 0.0) {
            s.store_scalar(44, (((p.p198 + (p.p199 * ((var_ile) as f64).powf(p.p200))) + (p.p201 * var_iwe)) + (p.p202 * var_iae)));
            s.store_scalar(45, (((p.p203 + (p.p204 * var_ile)) + (p.p205 * var_iwe)) + (p.p206 * var_iae)));
            s.store_scalar(46, p.p207);
            s.store_scalar(47, p.p208);
        }

        let (assign5440_e4109,) = {
    if (var_guard41 != 0.0) {
        (p.p209,)
    } else {
        (var_epsrox_p,)
    }
};
        var_epsrox_p = assign5440_e4109;

        if (s.v[1030] != 0.0) {
            s.store_scalar(331, (p.p210 * (if ((1.0 + ((p.p211 * s.v[316]) * (((1.0 + (s.v[313] / p.p212))) as f64).ln())) > 0.001) { (1.0 + ((p.p211 * var_iwe) * (((1.0 + (var_we / p.p212))) as f64).ln())) } else { 0.001 })));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(332, (p.p213 * (if ((1.0 + ((p.p214 * s.v[316]) * (((1.0 + (s.v[313] / p.p215))) as f64).ln())) > 0.001) { (1.0 + ((p.p214 * var_iwe) * (((1.0 + (var_we / p.p215))) as f64).ln())) } else { 0.001 })));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(333, (p.p216 * (if ((1.0 + ((p.p217 * s.v[316]) * (((1.0 + (s.v[313] / p.p215))) as f64).ln())) > 0.001) { (1.0 + ((p.p217 * var_iwe) * (((1.0 + (var_we / p.p215))) as f64).ln())) } else { 0.001 })));
        }

        s.b[1031] = (s.v[312] > (2.0 * s.v[333]));
        s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1031]) {
            s.store_scalar(334, 75000000000.0);
            s.store_sub_ad(335, A::sqrt(A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(332), 0.5)), A::sqrt(s.ad_value(331)));
            s.store_add_scaled_product_mixed_aia(336, A::sqrt(s.ad_value(331)), 1.0, 334, A::ln(A::offset(A::mul_scaled_lhs(s.ad_value(333), (2.0 * 1.0 / (var_le)), A::offset(A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0))), 1.0)), 1.0);
            s.store_square(336, 336);
        }

        s.b[1032] = (s.v[312] >= s.v[333]);
        s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });

        if (((s.v[1030] != 0.0) && (!s.b[1031])) && s.b[1032]) {
            s.store_add_scaled_product_indices(336, 331, 1.0, 332, 333, 1.0 / (var_le));
        }

        if (((s.v[1030] != 0.0) && (!s.b[1031])) && (!s.b[1032])) {
            s.store_add_ad_rhs(336, 331, A::mul_sub_from_scalar_rhs(s.ad_value(332), 2.0, A::div_from_scalar(var_le, s.ad_value(333))));
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(49, 336, ((1.0 - (p.p218 * var_ile)) - (p.p219 * s.v[315])));
            s.store_scalar(50, (((p.p220 + (p.p221 * ((var_ile) as f64).powf(p.p222))) + (p.p223 * var_iwe)) + (p.p224 * var_iae)));
            s.store_scalar(51, p.p225);
            s.store_scalar(52, p.p226);
            s.store_scalar(53, (((p.p227 + (p.p228 * ((var_ile) as f64).powf(p.p229))) + (p.p230 * var_iwe)) + (p.p231 * var_iae)));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(54, (p.p232 * (if (1e-6 > (1.0 + (p.p233 * s.v[314]))) { 1e-6 } else { (1.0 + (p.p233 * var_ile)) })));
        }

        let (assign5620_e4383,) = {
    if (var_guard41 != 0.0) {
        (p.p234,)
    } else {
        (var_toxov_p,)
    }
};
        var_toxov_p = assign5620_e4383;

        let (assign5630_e4387,) = {
    if (var_guard41 != 0.0) {
        (p.p235,)
    } else {
        (var_toxovd_p,)
    }
};
        var_toxovd_p = assign5630_e4387;

        let (assign5640_e4391,) = {
    if (var_guard41 != 0.0) {
        (p.p238,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign5640_e4391;

        let (assign5650_e4395,) = {
    if (var_guard41 != 0.0) {
        (p.p239,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign5650_e4395;

        if (s.v[1030] != 0.0) {
            s.store_scalar(55, (((p.p240 + (p.p241 * ((var_ile) as f64).powf(p.p242))) * (1.0 + (p.p243 * var_iwe))) * (1.0 + (p.p244 * var_iae))));
            s.store_scalar(56, p.p246);
            s.store_scalar(57, p.p245);
            s.store_scalar(58, p.p247);
            s.store_scalar(66, ((p.p248 * ((var_ile) as f64).powf(p.p249)) * (1.0 + (p.p250 * var_iwe))));
            s.store_scalar(67, p.p252);
            s.store_scalar(68, p.p251);
            s.store_scalar(63, ((p.p253 * ((var_ile) as f64).powf(p.p254)) * (1.0 + (p.p255 * var_iwe))));
            s.store_scalar(64, p.p257);
            s.store_scalar(65, p.p256);
            s.store_scalar(337, (p.p259 * (1.0 + (p.p260 * var_iwe))));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(338, (p.p261 * (if ((1.0 + (p.p262 * s.v[316])) > 0.001) { (1.0 + (p.p262 * var_iwe)) } else { 0.001 })));
        }

        if (s.v[1030] != 0.0) {
            s.store_offset_product3(339, s.ad_value(337), s.ad_value(338), A::sub_from_scalar(1.0, A::exp(A::div_from_scalar((-var_le), s.ad_value(338)))), 1.0 / (var_le), ((1.0) + ((((p.p263 * p.p264) / var_le) * (1.0 - ((((-var_le) / p.p264)) as f64).exp())))));
        }

        if (s.v[1030] != 0.0) {
            if (s.v[339] > 1e-15) {
            } else {
                s.store_scalar(339, 1e-15);
            }
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(340, ((1.0 + (p.p265 * var_iwe)) + ((p.p266 * var_iwe) * (((1.0 + (var_we / p.p267))) as f64).ln())));
            s.store_mul_div_from_scalar_ad_lhs(69, (p.p258 * var_we), A::scale(s.ad_value(339), var_le), 340);
            s.store_scalar(70, (((p.p268 + (p.p269 * var_ile)) + (p.p270 * var_iwe)) + (p.p271 * var_iae)));
            s.store_scalar(71, (p.p272 * (1.0 + (p.p273 * var_iwe))));
            s.store_scalar(72, p.p274);
            s.store_scalar(73, p.p275);
            s.store_scalar(74, p.p276);
            s.store_scalar(75, (((p.p277 + (p.p278 * ((var_ile) as f64).powf(p.p279))) * (1.0 + (p.p280 * var_iwe))) * (1.0 + (p.p281 * var_iae))));
            s.store_scalar(76, p.p282);
            s.store_scalar(77, p.p283);
            s.store_scalar(78, p.p284);
            s.store_scalar(79, (((p.p285 * (1.0 + (p.p286 * var_ile))) * (1.0 + (p.p287 * var_iwe))) * (1.0 + (p.p288 * var_iae))));
            s.store_scalar(80, p.p289);
            s.store_scalar(81, p.p290);
            s.store_scalar(82, ((p.p291 * var_iwe) * (1.0 + (p.p292 * var_iwe))));
            s.store_scalar(83, p.p293);
            s.store_scalar(84, p.p294);
            s.store_scalar(85, p.p295);
            s.store_scaled_offset_ad(86, A::div_scaled_inputs(s.ad_value(340), (p.p297 * ((var_ile) as f64).powf(p.p298)), s.ad_value(339), 1.0), p.p296, ((1.0 + (p.p299 * var_iwe)) * (1.0 + (p.p300 * var_iae))));
            s.store_scalar(87, (((p.p301 + (p.p302 * var_ile)) + (p.p303 * var_iwe)) + (p.p304 * var_iae)));
            s.store_scalar(88, p.p305);
            s.store_scalar(89, p.p306);
            s.store_scalar(90, p.p307);
            s.store_scalar(91, (p.p308 / (1.0 + (p.p309 * var_ile))));
            s.store_scalar(92, ((p.p310 * ((var_ile) as f64).powf(p.p311)) * (1.0 + (p.p312 * var_iwe))));
            s.store_scalar(341, ((var_ile) as f64).powf(p.p314));
        }

        *var_agidl_p_slot = var_agidl_p;
        *var_agidld_p_slot = var_agidld_p;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_epsrox_p_slot = var_epsrox_p;
        *var_guard41_slot = var_guard41;
        *var_igov_p_slot = var_igov_p;
        *var_igovd_p_slot = var_igovd_p;
        *var_nov_p_slot = var_nov_p;
        *var_novd_p_slot = var_novd_p;
        *var_rbulk_p_slot = var_rbulk_p;
        *var_rde_p_slot = var_rde_p;
        *var_rg_p_slot = var_rg_p;
        *var_rjund_p_slot = var_rjund_p;
        *var_rjuns_p_slot = var_rjuns_p;
        *var_rse_p_slot = var_rse_p;
        *var_rwell_p_slot = var_rwell_p;
        *var_stig_p_slot = var_stig_p;
        *var_toxov_p_slot = var_toxov_p;
        *var_toxovd_p_slot = var_toxovd_p;
    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard41: f64,
        var_iae: f64,
        var_iiwcv: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_iwe: f64,
        var_l_f: f64,
        var_l_slif: f64,
        var_le: f64,
        var_nf_i: f64,
        var_ngcon_i: f64,
        var_w_f: f64,
        var_we: f64,
        var_wecv: f64,
        var_xgwe: f64,
        var_agidl_p_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_rg_p_slot: &mut f64,
        var_rsh_i_slot: &mut f64,
        var_stig_p_slot: &mut f64,
    ) {
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_rg_p: f64 = *var_rg_p_slot;
        let mut var_rsh_i: f64 = *var_rsh_i_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;

        if (s.v[1030] != 0.0) {
            s.store_div_scaled_inputs_mixed_ia(93, 341, (p.p313 * (1.0 + (p.p316 * var_iwe))), A::scale_offset(s.ad_value(341), (p.p315 * var_ile), 1.0), 1.0);
            s.store_scalar(341, ((var_ile) as f64).powf(p.p318));
            s.store_div_scaled_inputs_mixed_ia(94, 341, (p.p317 * (1.0 + (p.p320 * var_iwe))), A::scale_offset(s.ad_value(341), (p.p319 * var_ile), 1.0), 1.0);
            s.store_scalar(95, p.p321);
            s.store_scalar(96, ((p.p322 * (1.0 + (p.p323 * var_ile))) * (1.0 + (p.p324 * var_iwe))));
            s.store_scalar(97, p.p325);
            s.store_scalar(98, p.p326);
            s.store_scalar(99, ((p.p327 * (1.0 + (p.p328 * var_ile))) * (1.0 + (p.p329 * var_iwe))));
            s.store_scalar(100, ((p.p330 * (1.0 + (p.p331 * var_ile))) * (1.0 + (p.p332 * var_iwe))));
            s.store_scalar(101, p.p333);
            s.store_scalar(102, p.p334);
            s.store_scalar(103, (p.p335 / var_iae));
        }

        let (assign6180_e4914,) = {
    if (var_guard41 != 0.0) {
        let assign6180_e4908: f64 = (p.p336 * p.p236);
        let assign6180_e4911: f64 = (1e-6 * var_iwe);
        let assign6180_e4912: f64 = (assign6180_e4908 / assign6180_e4911);
        (assign6180_e4912,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign6180_e4914;

        let (assign6190_e4924,) = {
    if (var_guard41 != 0.0) {
        let assign6190_e4918: f64 = (p.p337 * p.p237);
        let assign6190_e4921: f64 = (1e-6 * var_iwe);
        let assign6190_e4922: f64 = (assign6190_e4918 / assign6190_e4921);
        (assign6190_e4922,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign6190_e4924;

        let (assign6200_e4928,) = {
    if (var_guard41 != 0.0) {
        (p.p338,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign6200_e4928;

        if (s.v[1030] != 0.0) {
            s.store_scalar(107, p.p339);
            s.store_scalar(108, p.p340);
            s.store_scalar(109, p.p339);
        }

        s.b[1033] = param_given[341];
        s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1033]) {
            s.store_scalar(109, p.p341);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(110, p.p340);
        }

        s.b[1034] = param_given[342];
        s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1034]) {
            s.store_scalar(110, p.p342);
        }

        if (s.v[1030] != 0.0) {
            s.copy_ad(111, 109);
        }

        s.b[1035] = param_given[343];
        s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1035]) {
            s.store_scalar(111, p.p343);
        }

        if (s.v[1030] != 0.0) {
            s.copy_ad(112, 110);
        }

        s.b[1036] = param_given[344];
        s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1036]) {
            s.store_scalar(112, p.p344);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(113, p.p345);
        }

        let (assign6360_e5006,) = {
    if (var_guard41 != 0.0) {
        let assign6360_e5000: f64 = (p.p346 * p.p236);
        let assign6360_e5003: f64 = (1e-6 * var_iwe);
        let assign6360_e5004: f64 = (assign6360_e5000 / assign6360_e5003);
        (assign6360_e5004,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign6360_e5006;

        let (assign6370_e5016,) = {
    if (var_guard41 != 0.0) {
        let assign6370_e5010: f64 = (p.p347 * p.p237);
        let assign6370_e5013: f64 = (1e-6 * var_iwe);
        let assign6370_e5014: f64 = (assign6370_e5010 / assign6370_e5013);
        (assign6370_e5014,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign6370_e5016;

        if (s.v[1030] != 0.0) {
            s.store_scalar(116, p.p348);
            s.store_scalar(117, p.p349);
            s.store_scalar(118, p.p350);
            s.store_scalar(119, p.p351);
            s.store_scalar(120, p.p352);
            s.store_scalar(121, p.p353);
            s.store_scalar(122, ((((8.8541878176e-12 * p.p209) * var_wecv) * s.v[320]) / p.p208));
        }

        let (assign6450_e5064,) = {
    if (var_guard41 != 0.0) {
        let assign6450_e5056: f64 = (8.8541878176e-12 * p.p209);
        let assign6450_e5058: f64 = (assign6450_e5056 * var_wecv);
        let assign6450_e5060: f64 = (assign6450_e5058 * p.p236);
        let assign6450_e5062: f64 = (assign6450_e5060 / p.p234);
        (assign6450_e5062,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign6450_e5064;

        let (assign6460_e5076,) = {
    if (var_guard41 != 0.0) {
        let assign6460_e5068: f64 = (8.8541878176e-12 * p.p209);
        let assign6460_e5070: f64 = (assign6460_e5068 * var_wecv);
        let assign6460_e5072: f64 = (assign6460_e5070 * p.p237);
        let assign6460_e5074: f64 = (assign6460_e5072 / p.p235);
        (assign6460_e5074,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign6460_e5076;

        if (s.v[1030] != 0.0) {
            s.store_scalar(123, (((p.p354 + (p.p355 * ((var_ile) as f64).powf(p.p356))) + (p.p357 * var_iwe)) + (p.p358 * var_iae)));
            s.store_scalar(124, (((p.p359 + (p.p360 * var_ile)) + (p.p361 * var_iwe)) + (p.p362 * var_iae)));
            s.store_scalar(36, p.p296);
        }

        s.b[1037] = param_given[363];
        s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1037]) {
            s.store_scalar(36, p.p363);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(37, p.p297);
        }

        s.b[1038] = param_given[364];
        s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1038]) {
            s.store_scalar(37, p.p364);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(38, p.p298);
        }

        s.b[1039] = param_given[365];
        s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1039]) {
            s.store_scalar(38, p.p365);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(39, p.p299);
        }

        s.b[1040] = param_given[366];
        s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1040]) {
            s.store_scalar(39, p.p366);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(40, p.p300);
        }

        s.b[1041] = param_given[367];
        s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1041]) {
            s.store_scalar(40, p.p367);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul3_ad(125, A::add_scaled_product(s.ad_value(36), 1.0, A::div_scaled_product(s.ad_value(37), s.ad_value(340), 1.0, s.ad_value(339), 1.0), A::pow_from_scalar(var_ile, s.ad_value(38)), 1.0), A::scale_offset(s.ad_value(39), var_iwe, 1.0), A::scale_offset(s.ad_value(40), var_iae, 1.0));
            s.store_scalar(41, p.p308);
        }

        s.b[1042] = param_given[368];
        s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1042]) {
            s.store_scalar(41, p.p368);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(42, p.p309);
        }

        s.b[1043] = param_given[369];
        s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1043]) {
            s.store_scalar(42, p.p369);
        }

        if (s.v[1030] != 0.0) {
            s.store_div_ad_rhs(126, 41, A::scale_offset(s.ad_value(42), var_ile, 1.0));
            s.store_scalar(127, ((p.p370 * ((var_ile) as f64).powf(p.p371)) * (1.0 + (p.p372 * var_iwe))));
            s.store_scalar(341, ((var_ile) as f64).powf(p.p374));
            s.store_div_scaled_inputs_mixed_ia(128, 341, (p.p373 * (1.0 + (p.p376 * var_iwe))), A::scale_offset(s.ad_value(341), (p.p375 * var_ile), 1.0), 1.0);
            s.store_scalar(131, p.p377);
            s.store_scalar(132, p.p378);
            s.store_scalar(133, p.p379);
            s.store_scalar(134, (p.p380 * s.v[325]));
            s.store_scalar(135, (p.p381 * var_iiwecv));
            s.store_scalar(136, (p.p382 * var_iiwecv));
            s.store_scalar(137, p.p383);
            s.store_scalar(138, p.p384);
            s.store_scalar(139, p.p385);
            s.store_scalar(140, p.p386);
        }

        let (assign6850_e5336,) = {
    if (var_guard41 != 0.0) {
        let assign6850_e5334: f64 = (p.p387 * var_iiwcv);
        (assign6850_e5334,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign6850_e5336;

        let (assign6860_e5342,) = {
    if (var_guard41 != 0.0) {
        let assign6860_e5340: f64 = (p.p388 * var_iiwcv);
        (assign6860_e5340,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign6860_e5342;

        if (s.v[1030] != 0.0) {
            s.store_scalar(1012, (1.0 - ((2.0 * p.p395) / var_le)));
            s.store_scalar(143, p.p389);
            s.store_scaled_mul(144, 69, 69, (p.p390 * (var_iwe * var_iwe)));
            s.store_scalar(344, ((2.0 * p.p397) + (p.p398 * var_we)));
            s.store_scalar(149, p.p399);
            s.store_scalar(150, (((p.p400 + (p.p401 * var_ile)) + (p.p402 * var_iwe)) + (p.p403 * var_iae)));
            s.store_scalar(151, (((p.p404 + (p.p405 * ((var_ile) as f64).powf(p.p406))) + (p.p407 * var_iwe)) + (p.p408 * var_iae)));
            s.store_scalar(152, (((p.p409 * (1.0 + (p.p410 * ((var_ile) as f64).powf(p.p411)))) * (1.0 + (p.p412 * var_iwe))) * (1.0 + (p.p413 * var_iae))));
            s.store_scalar(153, (p.p414 + (p.p415 * ((var_ile) as f64).powf(p.p416))));
            s.store_scalar(347, (1.0 + (((p.p417 * p.p418) / var_le) * (1.0 - ((((-var_le) / p.p418)) as f64).exp()))));
        }

        if (s.v[1030] != 0.0) {
            if (s.v[347] > 1e-15) {
            } else {
                s.store_scalar(347, 1e-15);
            }
        }

        if (s.v[1030] != 0.0) {
            s.store_div_scaled_inputs_indices(154, 344, (p.p258 * (1.0 + (p.p419 * var_iwe))), 347, var_le);
            s.store_scalar(155, (((p.p420 + (p.p421 * var_ile)) + (p.p422 * var_iwe)) + (p.p423 * var_iae)));
            s.store_scalar(156, ((p.p424 * ((var_ile) as f64).powf(p.p425)) * (1.0 + (p.p426 * var_iwe))));
            s.store_scalar(157, p.p427);
            s.store_scalar(158, p.p428);
            s.store_scalar(159, ((p.p429 * ((var_ile) as f64).powf(p.p430)) * (1.0 + (p.p431 * var_iwe))));
            s.store_scalar(160, p.p433);
            s.store_scalar(161, p.p432);
            s.store_scalar(348, (((p.p814 + (p.p815 * var_ile)) + (p.p816 * var_iwe)) + (p.p817 * var_iae)));
            s.store_scalar(349, (((p.p818 + (p.p819 * var_ile)) + (p.p820 * var_iwe)) + (p.p821 * var_iae)));
        }

        let (assign7210_e5696,) = {
    if (var_guard41 != 0.0) {
        let assign7210_e5673: f64 = (0.3333333333333333 * var_w_f);
        let assign7210_e5675: f64 = (assign7210_e5673 / var_ngcon_i);
        let assign7210_e5677: f64 = (assign7210_e5675 + var_xgwe);
        let assign7210_e5678: f64 = (p.p442 * assign7210_e5677);
        let assign7210_e5681: f64 = (var_ngcon_i * var_l_slif);
        let assign7210_e5682: f64 = (assign7210_e5678 / assign7210_e5681);
        let assign7210_e5685: f64 = (p.p440 + p.p441);
        let assign7210_e5688: f64 = (var_w_f * var_l_f);
        let assign7210_e5689: f64 = (assign7210_e5685 / assign7210_e5688);
        let assign7210_e5690: f64 = (assign7210_e5682 + assign7210_e5689);
        let assign7210_e5693: f64 = (var_nf_i * p.p439);
        let assign7210_e5694: f64 = (assign7210_e5690 + assign7210_e5693);
        (assign7210_e5694,)
    } else {
        (var_rg_p,)
    }
};
        var_rg_p = assign7210_e5696;

        let (assign7220_e5705,) = {
    if (var_guard41 != 0.0) {
        let (assign7220_e5703,) = {
            if (p.p444 > 0.0) {
                (p.p444,)
            } else {
                (0.0,)
            }
        };
        (assign7220_e5703,)
    } else {
        (var_rsh_i,)
    }
};
        var_rsh_i = assign7220_e5705;

        *var_agidl_p_slot = var_agidl_p;
        *var_agidld_p_slot = var_agidld_p;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_igov_p_slot = var_igov_p;
        *var_igovd_p_slot = var_igovd_p;
        *var_rg_p_slot = var_rg_p;
        *var_rsh_i_slot = var_rsh_i;
        *var_stig_p_slot = var_stig_p;
    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard41: f64,
        var_iae: f64,
        var_iiwe: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_nf_i: f64,
        var_rsh_i: f64,
        var_we: f64,
        var_agidl_p_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard55_slot: &mut f64,
        var_guard63_slot: &mut f64,
        var_guard64_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_rbulk_p_slot: &mut f64,
        var_rde_p_slot: &mut f64,
        var_rjund_p_slot: &mut f64,
        var_rjuns_p_slot: &mut f64,
        var_rse_p_slot: &mut f64,
        var_rshd_i_slot: &mut f64,
        var_rwell_p_slot: &mut f64,
        var_stig_p_slot: &mut f64,
    ) {
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_guard63: f64 = *var_guard63_slot;
        let mut var_guard64: f64 = *var_guard64_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_rbulk_p: f64 = *var_rbulk_p_slot;
        let mut var_rde_p: f64 = *var_rde_p_slot;
        let mut var_rjund_p: f64 = *var_rjund_p_slot;
        let mut var_rjuns_p: f64 = *var_rjuns_p_slot;
        let mut var_rse_p: f64 = *var_rse_p_slot;
        let mut var_rshd_i: f64 = *var_rshd_i_slot;
        let mut var_rwell_p: f64 = *var_rwell_p_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;

        let (assign7230_e5714,) = {
    if (var_guard41 != 0.0) {
        let (assign7230_e5712,) = {
            if (p.p445 > 0.0) {
                (p.p445,)
            } else {
                (0.0,)
            }
        };
        (assign7230_e5712,)
    } else {
        (var_rshd_i,)
    }
};
        var_rshd_i = assign7230_e5714;

        let assign7240_e5717: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard55 = assign7240_e5717;

        let (assign7250_e5723,) = {
    if ((var_guard41 != 0.0) && (var_guard55 != 0.0)) {
        (var_rsh_i,)
    } else {
        (var_rshd_i,)
    }
};
        var_rshd_i = assign7250_e5723;

        let (assign7260_e5731,) = {
    if (var_guard41 != 0.0) {
        let assign7260_e5727: f64 = (var_nf_i * p.p12);
        let assign7260_e5729: f64 = (assign7260_e5727 * var_rsh_i);
        (assign7260_e5729,)
    } else {
        (var_rse_p,)
    }
};
        var_rse_p = assign7260_e5731;

        let (assign7270_e5739,) = {
    if (var_guard41 != 0.0) {
        let assign7270_e5735: f64 = (var_nf_i * p.p13);
        let assign7270_e5737: f64 = (assign7270_e5735 * var_rshd_i);
        (assign7270_e5737,)
    } else {
        (var_rde_p,)
    }
};
        var_rde_p = assign7270_e5739;

        let (assign7280_e5745,) = {
    if (var_guard41 != 0.0) {
        let assign7280_e5743: f64 = (var_nf_i * p.p447);
        (assign7280_e5743,)
    } else {
        (var_rwell_p,)
    }
};
        var_rwell_p = assign7280_e5745;

        let (assign7290_e5751,) = {
    if (var_guard41 != 0.0) {
        let assign7290_e5749: f64 = (var_nf_i * p.p446);
        (assign7290_e5749,)
    } else {
        (var_rbulk_p,)
    }
};
        var_rbulk_p = assign7290_e5751;

        let (assign7300_e5757,) = {
    if (var_guard41 != 0.0) {
        let assign7300_e5755: f64 = (var_nf_i * p.p448);
        (assign7300_e5755,)
    } else {
        (var_rjuns_p,)
    }
};
        var_rjuns_p = assign7300_e5757;

        let (assign7310_e5763,) = {
    if (var_guard41 != 0.0) {
        let assign7310_e5761: f64 = (var_nf_i * p.p449);
        (assign7310_e5761,)
    } else {
        (var_rjund_p,)
    }
};
        var_rjund_p = assign7310_e5763;

        if (s.v[1030] != 0.0) {
            s.store_scalar(176, p.p450);
        }

        s.b[1045] = (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]);
        s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1045]) {
            s.store_scalar(44, (((p.p451 + (p.p452 * var_ile)) + (p.p453 * var_iwe)) + (p.p454 * var_iae)));
        }

        s.b[1046] = (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]);
        s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1046]) {
            s.store_scalar(45, (((p.p455 + (p.p456 * var_ile)) + (p.p457 * var_iwe)) + (p.p458 * var_iae)));
        }

        s.b[1047] = (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]);
        s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1047]) {
            s.store_scalar(49, (((p.p459 + (p.p460 * var_ile)) + (p.p461 * var_iwe)) + (p.p462 * var_iae)));
        }

        s.b[1048] = (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]);
        s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1048]) {
            s.store_scalar(50, (((p.p463 + (p.p464 * var_ile)) + (p.p465 * var_iwe)) + (p.p466 * var_iae)));
        }

        s.b[1049] = (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]);
        s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1049]) {
            s.store_scalar(51, (((p.p467 + (p.p468 * var_ile)) + (p.p469 * var_iwe)) + (p.p470 * var_iae)));
        }

        s.b[1050] = (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]);
        s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1050]) {
            s.store_scalar(53, (((p.p471 + (p.p472 * var_ile)) + (p.p473 * var_iwe)) + (p.p474 * var_iae)));
        }

        s.b[1051] = (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]);
        s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1051]) {
            s.store_scalar(54, (((p.p475 + (p.p476 * var_ile)) + (p.p477 * var_iwe)) + (p.p478 * var_iae)));
        }

        let assign7470_e6045: f64 = if (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]) { 1.0 } else { 0.0 };
        var_guard63 = assign7470_e6045;

        let (assign7480_e6063,) = {
    if ((var_guard41 != 0.0) && (var_guard63 != 0.0)) {
        let assign7480_e6052: f64 = (p.p480 * var_ile);
        let assign7480_e6053: f64 = (p.p479 + assign7480_e6052);
        let assign7480_e6056: f64 = (p.p481 * var_iwe);
        let assign7480_e6057: f64 = (assign7480_e6053 + assign7480_e6056);
        let assign7480_e6060: f64 = (p.p482 * var_iae);
        let assign7480_e6061: f64 = (assign7480_e6057 + assign7480_e6060);
        (assign7480_e6061,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign7480_e6063;

        let assign7490_e6082: f64 = if (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]) { 1.0 } else { 0.0 };
        var_guard64 = assign7490_e6082;

        let (assign7500_e6100,) = {
    if ((var_guard41 != 0.0) && (var_guard64 != 0.0)) {
        let assign7500_e6089: f64 = (p.p484 * var_ile);
        let assign7500_e6090: f64 = (p.p483 + assign7500_e6089);
        let assign7500_e6093: f64 = (p.p485 * var_iwe);
        let assign7500_e6094: f64 = (assign7500_e6090 + assign7500_e6093);
        let assign7500_e6097: f64 = (p.p486 * var_iae);
        let assign7500_e6098: f64 = (assign7500_e6094 + assign7500_e6097);
        (assign7500_e6098,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign7500_e6100;

        s.b[1054] = (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]);
        s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1054]) {
            s.store_scalar(55, (((p.p487 + (p.p488 * var_ile)) + (p.p489 * var_iwe)) + (p.p490 * var_iae)));
        }

        s.b[1055] = (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]);
        s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1055]) {
            s.store_scalar(56, (((p.p495 + (p.p496 * var_ile)) + (p.p497 * var_iwe)) + (p.p498 * var_iae)));
        }

        s.b[1056] = (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]);
        s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1056]) {
            s.store_scalar(57, (((p.p491 + (p.p492 * var_ile)) + (p.p493 * var_iwe)) + (p.p494 * var_iae)));
        }

        s.b[1057] = (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]);
        s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1057]) {
            s.store_scalar(58, (((p.p499 + (p.p500 * var_ile)) + (p.p501 * var_iwe)) + (p.p502 * var_iae)));
        }

        s.b[1058] = (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]);
        s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1058]) {
            s.store_scalar(66, (s.v[315] * (((p.p503 + (p.p504 * var_ile)) + (p.p505 * var_iwe)) + (p.p506 * var_iae))));
        }

        s.b[1059] = (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]);
        s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1059]) {
            s.store_scalar(67, (((p.p511 + (p.p512 * var_ile)) + (p.p513 * var_iwe)) + (p.p514 * var_iae)));
        }

        s.b[1060] = (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]);
        s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1060]) {
            s.store_scalar(68, (((p.p507 + (p.p508 * var_ile)) + (p.p509 * var_iwe)) + (p.p510 * var_iae)));
        }

        s.b[1061] = (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]);
        s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1061]) {
            s.store_scalar(63, (s.v[315] * (((p.p515 + (p.p516 * var_ile)) + (p.p517 * var_iwe)) + (p.p518 * var_iae))));
        }

        s.b[1062] = (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]);
        s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1062]) {
            s.store_scalar(64, (((p.p523 + (p.p524 * var_ile)) + (p.p525 * var_iwe)) + (p.p526 * var_iae)));
        }

        s.b[1063] = (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]);
        s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1063]) {
            s.store_scalar(65, (((p.p519 + (p.p520 * var_ile)) + (p.p521 * var_iwe)) + (p.p522 * var_iae)));
        }

        s.b[1064] = (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]);
        s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1064]) {
            s.store_scalar(69, ((var_we / var_le) * (((p.p527 + (p.p528 * var_ile)) + (p.p529 * var_iwe)) + (p.p530 * var_iae))));
        }

        s.b[1065] = (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]);
        s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1065]) {
            s.store_scalar(70, (((p.p531 + (p.p532 * var_ile)) + (p.p533 * var_iwe)) + (p.p534 * var_iae)));
        }

        s.b[1066] = (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]);
        s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1066]) {
            s.store_scalar(71, (((p.p535 + (p.p536 * var_ile)) + (p.p537 * var_iwe)) + (p.p538 * var_iae)));
        }

        s.b[1067] = (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]);
        s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1067]) {
            s.store_scalar(73, (((p.p539 + (p.p540 * var_ile)) + (p.p541 * var_iwe)) + (p.p542 * var_iae)));
        }

        s.b[1068] = (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]);
        s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1068]) {
            s.store_scalar(75, (((p.p543 + (p.p544 * var_ile)) + (p.p545 * var_iwe)) + (p.p546 * var_iae)));
        }

        s.b[1069] = (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]);
        s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1069]) {
            s.store_scalar(77, (((p.p547 + (p.p548 * var_ile)) + (p.p549 * var_iwe)) + (p.p550 * var_iae)));
        }

        s.b[1070] = (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]);
        s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1070]) {
            s.store_scalar(79, (((p.p551 + (p.p552 * var_ile)) + (p.p553 * var_iwe)) + (p.p554 * var_iae)));
        }

        s.b[1071] = (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]);
        s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1071]) {
            s.store_scalar(82, (var_iwe * (((p.p555 + (p.p556 * var_ile)) + (p.p557 * var_iwe)) + (p.p558 * var_iae))));
        }

        s.b[1072] = (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]);
        s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1072]) {
            s.store_scalar(83, (((p.p559 + (p.p560 * var_ile)) + (p.p561 * var_iwe)) + (p.p562 * var_iae)));
        }

        s.b[1073] = (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]);
        s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1073]) {
            s.store_scalar(84, (((p.p563 + (p.p564 * var_ile)) + (p.p565 * var_iwe)) + (p.p566 * var_iae)));
        }

        s.b[1074] = (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]);
        s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1074]) {
            s.store_scalar(85, (((p.p567 + (p.p568 * var_ile)) + (p.p569 * var_iwe)) + (p.p570 * var_iae)));
        }

        s.b[1075] = (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]);
        s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1075]) {
            s.store_scalar(86, (var_ile * (((p.p571 + (p.p572 * var_ile)) + (p.p573 * var_iwe)) + (p.p574 * var_iae))));
        }

        s.b[1076] = (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]);
        s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1076]) {
            s.store_scalar(87, (((p.p575 + (p.p576 * var_ile)) + (p.p577 * var_iwe)) + (p.p578 * var_iae)));
        }

        s.b[1077] = (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]);
        s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1077]) {
            s.store_scalar(88, (((p.p579 + (p.p580 * var_ile)) + (p.p581 * var_iwe)) + (p.p582 * var_iae)));
        }

        s.b[1078] = (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]);
        s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1078]) {
            s.store_scalar(89, (((p.p583 + (p.p584 * var_ile)) + (p.p585 * var_iwe)) + (p.p586 * var_iae)));
        }

        s.b[1079] = (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]);
        s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1079]) {
            s.store_scalar(91, (((p.p587 + (p.p588 * var_ile)) + (p.p589 * var_iwe)) + (p.p590 * var_iae)));
        }

        s.b[1080] = (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]);
        s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1080]) {
            s.store_scalar(92, (var_ile * (((p.p591 + (p.p592 * var_ile)) + (p.p593 * var_iwe)) + (p.p594 * var_iae))));
        }

        s.b[1081] = (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]);
        s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1081]) {
            s.store_scalar(93, (((p.p595 + (p.p596 * var_ile)) + (p.p597 * var_iwe)) + (p.p598 * var_iae)));
        }

        s.b[1082] = (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]);
        s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1082]) {
            s.store_scalar(94, (((p.p599 + (p.p600 * var_ile)) + (p.p601 * var_iwe)) + (p.p602 * var_iae)));
        }

        s.b[1083] = (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]);
        s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1083]) {
            s.store_scalar(96, (((p.p603 + (p.p604 * var_ile)) + (p.p605 * var_iwe)) + (p.p606 * var_iae)));
        }

        s.b[1084] = (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]);
        s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1084]) {
            s.store_scalar(98, (((p.p607 + (p.p608 * var_ile)) + (p.p609 * var_iwe)) + (p.p610 * var_iae)));
        }

        s.b[1085] = (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]);
        s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1085]) {
            s.store_scalar(99, (((p.p611 + (p.p612 * var_ile)) + (p.p613 * var_iwe)) + (p.p614 * var_iae)));
        }

        s.b[1086] = (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]);
        s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1086]) {
            s.store_scalar(100, (((p.p615 + (p.p616 * var_ile)) + (p.p617 * var_iwe)) + (p.p618 * var_iae)));
        }

        s.b[1087] = (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]);
        s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1087]) {
            s.store_scalar(103, (s.v[319] * (((p.p619 + (p.p620 * var_ile)) + (p.p621 * var_iwe)) + (p.p622 * var_iae))));
        }

        let assign8190_e7393: f64 = if (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]) { 1.0 } else { 0.0 };
        var_guard99 = assign8190_e7393;

        let (assign8200_e7413,) = {
    if ((var_guard41 != 0.0) && (var_guard99 != 0.0)) {
        let assign8200_e7401: f64 = (p.p624 * var_ile);
        let assign8200_e7402: f64 = (p.p623 + assign8200_e7401);
        let assign8200_e7405: f64 = (p.p625 * var_iwe);
        let assign8200_e7406: f64 = (assign8200_e7402 + assign8200_e7405);
        let assign8200_e7409: f64 = (p.p626 * var_iae);
        let assign8200_e7410: f64 = (assign8200_e7406 + assign8200_e7409);
        let assign8200_e7411: f64 = (var_iiwe * assign8200_e7410);
        (assign8200_e7411,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign8200_e7413;

        let assign8210_e7432: f64 = if (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]) { 1.0 } else { 0.0 };
        var_guard100 = assign8210_e7432;

        let (assign8220_e7452,) = {
    if ((var_guard41 != 0.0) && (var_guard100 != 0.0)) {
        let assign8220_e7440: f64 = (p.p628 * var_ile);
        let assign8220_e7441: f64 = (p.p627 + assign8220_e7440);
        let assign8220_e7444: f64 = (p.p629 * var_iwe);
        let assign8220_e7445: f64 = (assign8220_e7441 + assign8220_e7444);
        let assign8220_e7448: f64 = (p.p630 * var_iae);
        let assign8220_e7449: f64 = (assign8220_e7445 + assign8220_e7448);
        let assign8220_e7450: f64 = (var_iiwe * assign8220_e7449);
        (assign8220_e7450,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign8220_e7452;

        let assign8230_e7471: f64 = if (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]) { 1.0 } else { 0.0 };
        var_guard101 = assign8230_e7471;

        let (assign8240_e7489,) = {
    if ((var_guard41 != 0.0) && (var_guard101 != 0.0)) {
        let assign8240_e7478: f64 = (p.p632 * var_ile);
        let assign8240_e7479: f64 = (p.p631 + assign8240_e7478);
        let assign8240_e7482: f64 = (p.p633 * var_iwe);
        let assign8240_e7483: f64 = (assign8240_e7479 + assign8240_e7482);
        let assign8240_e7486: f64 = (p.p634 * var_iae);
        let assign8240_e7487: f64 = (assign8240_e7483 + assign8240_e7486);
        (assign8240_e7487,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign8240_e7489;

        let assign8250_e7508: f64 = if (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]) { 1.0 } else { 0.0 };
        var_guard102 = assign8250_e7508;

        let (assign8260_e7528,) = {
    if ((var_guard41 != 0.0) && (var_guard102 != 0.0)) {
        let assign8260_e7516: f64 = (p.p636 * var_ile);
        let assign8260_e7517: f64 = (p.p635 + assign8260_e7516);
        let assign8260_e7520: f64 = (p.p637 * var_iwe);
        let assign8260_e7521: f64 = (assign8260_e7517 + assign8260_e7520);
        let assign8260_e7524: f64 = (p.p638 * var_iae);
        let assign8260_e7525: f64 = (assign8260_e7521 + assign8260_e7524);
        let assign8260_e7526: f64 = (var_iiwe * assign8260_e7525);
        (assign8260_e7526,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign8260_e7528;

        let assign8270_e7547: f64 = if (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]) { 1.0 } else { 0.0 };
        var_guard103 = assign8270_e7547;

        let (assign8280_e7567,) = {
    if ((var_guard41 != 0.0) && (var_guard103 != 0.0)) {
        let assign8280_e7555: f64 = (p.p640 * var_ile);
        let assign8280_e7556: f64 = (p.p639 + assign8280_e7555);
        let assign8280_e7559: f64 = (p.p641 * var_iwe);
        let assign8280_e7560: f64 = (assign8280_e7556 + assign8280_e7559);
        let assign8280_e7563: f64 = (p.p642 * var_iae);
        let assign8280_e7564: f64 = (assign8280_e7560 + assign8280_e7563);
        let assign8280_e7565: f64 = (var_iiwe * assign8280_e7564);
        (assign8280_e7565,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign8280_e7567;

        *var_agidl_p_slot = var_agidl_p;
        *var_agidld_p_slot = var_agidld_p;
        *var_guard100_slot = var_guard100;
        *var_guard101_slot = var_guard101;
        *var_guard102_slot = var_guard102;
        *var_guard103_slot = var_guard103;
        *var_guard55_slot = var_guard55;
        *var_guard63_slot = var_guard63;
        *var_guard64_slot = var_guard64;
        *var_guard99_slot = var_guard99;
        *var_igov_p_slot = var_igov_p;
        *var_igovd_p_slot = var_igovd_p;
        *var_nov_p_slot = var_nov_p;
        *var_novd_p_slot = var_novd_p;
        *var_rbulk_p_slot = var_rbulk_p;
        *var_rde_p_slot = var_rde_p;
        *var_rjund_p_slot = var_rjund_p;
        *var_rjuns_p_slot = var_rjuns_p;
        *var_rse_p_slot = var_rse_p;
        *var_rshd_i_slot = var_rshd_i;
        *var_rwell_p_slot = var_rwell_p;
        *var_stig_p_slot = var_stig_p;
    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_dellps: f64,
        var_delwod: f64,
        var_guard41: f64,
        var_iae: f64,
        var_iiwcv: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_invnf: f64,
        var_iwe: f64,
        var_l_i: f64,
        var_le: f64,
        var_nf_i: f64,
        var_rta: f64,
        var_w_i: f64,
        var_cfr_p_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_guard122_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard127_slot: &mut f64,
    ) {
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_guard122: f64 = *var_guard122_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard127: f64 = *var_guard127_slot;

        s.b[1093] = (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]);
        s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1093]) {
            s.store_scalar(118, (((p.p643 + (p.p644 * var_ile)) + (p.p645 * var_iwe)) + (p.p646 * var_iae)));
        }

        s.b[1094] = (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]);
        s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1094]) {
            s.store_scalar(119, (((p.p647 + (p.p648 * var_ile)) + (p.p649 * var_iwe)) + (p.p650 * var_iae)));
        }

        s.b[1095] = (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]);
        s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1095]) {
            s.store_scalar(122, (((var_iiwecv * s.v[320]) / 1e-6) * (((p.p651 + (p.p652 * var_ile)) + (p.p653 * var_iwe)) + (p.p654 * var_iae))));
        }

        s.b[1096] = (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]);
        s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1096]) {
            s.store_scalar(123, (((p.p655 + (p.p656 * var_ile)) + (p.p657 * var_iwe)) + (p.p658 * var_iae)));
        }

        s.b[1097] = (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]);
        s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1097]) {
            s.store_scalar(124, (((p.p659 + (p.p660 * var_ile)) + (p.p661 * var_iwe)) + (p.p662 * var_iae)));
        }

        s.b[1098] = (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]);
        s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1098]) {
            s.store_scalar(32, p.p571);
        }

        s.b[1099] = param_given[663];
        s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });

        if (((s.v[1030] != 0.0) && s.b[1098]) && s.b[1099]) {
            s.store_scalar(32, p.p663);
        }

        if ((s.v[1030] != 0.0) && s.b[1098]) {
            s.store_scalar(33, p.p572);
        }

        s.b[1100] = param_given[664];
        s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });

        if (((s.v[1030] != 0.0) && s.b[1098]) && s.b[1100]) {
            s.store_scalar(33, p.p664);
        }

        if ((s.v[1030] != 0.0) && s.b[1098]) {
            s.store_scalar(34, p.p573);
        }

        s.b[1101] = param_given[665];
        s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });

        if (((s.v[1030] != 0.0) && s.b[1098]) && s.b[1101]) {
            s.store_scalar(34, p.p665);
        }

        if ((s.v[1030] != 0.0) && s.b[1098]) {
            s.store_scalar(35, p.p574);
        }

        s.b[1102] = param_given[666];
        s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });

        if (((s.v[1030] != 0.0) && s.b[1098]) && s.b[1102]) {
            s.store_scalar(35, p.p666);
        }

        if ((s.v[1030] != 0.0) && s.b[1098]) {
            s.store_add_scaled_inputs4_indices(125, 32, var_ile, 33, (var_ile * var_ile), 34, (var_iwe * var_ile), 35, (var_iae * var_ile));
        }

        s.b[1103] = (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]);
        s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1103]) {
            s.store_scalar(32, p.p587);
        }

        s.b[1104] = param_given[667];
        s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });

        if (((s.v[1030] != 0.0) && s.b[1103]) && s.b[1104]) {
            s.store_scalar(32, p.p667);
        }

        if ((s.v[1030] != 0.0) && s.b[1103]) {
            s.store_scalar(33, p.p588);
        }

        s.b[1105] = param_given[668];
        s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });

        if (((s.v[1030] != 0.0) && s.b[1103]) && s.b[1105]) {
            s.store_scalar(33, p.p668);
        }

        if ((s.v[1030] != 0.0) && s.b[1103]) {
            s.store_scalar(34, p.p589);
        }

        s.b[1106] = param_given[669];
        s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });

        if (((s.v[1030] != 0.0) && s.b[1103]) && s.b[1106]) {
            s.store_scalar(34, p.p669);
        }

        if ((s.v[1030] != 0.0) && s.b[1103]) {
            s.store_scalar(35, p.p590);
        }

        s.b[1107] = param_given[670];
        s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });

        if (((s.v[1030] != 0.0) && s.b[1103]) && s.b[1107]) {
            s.store_scalar(35, p.p670);
        }

        if ((s.v[1030] != 0.0) && s.b[1103]) {
            s.store_add_scaled_inputs4_indices(126, 32, 1.0, 33, var_ile, 34, var_iwe, 35, var_iae);
        }

        s.b[1108] = (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]);
        s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1108]) {
            s.store_scalar(127, (var_ile * (((p.p671 + (p.p672 * var_ile)) + (p.p673 * var_iwe)) + (p.p674 * var_iae))));
        }

        s.b[1109] = (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]);
        s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1109]) {
            s.store_scalar(128, (var_ile * (((p.p675 + (p.p676 * var_ile)) + (p.p677 * var_iwe)) + (p.p678 * var_iae))));
        }

        let assign8710_e8117: f64 = if (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]) { 1.0 } else { 0.0 };
        var_guard121 = assign8710_e8117;

        let (assign8720_e8137,) = {
    if ((var_guard41 != 0.0) && (var_guard121 != 0.0)) {
        let assign8720_e8125: f64 = (p.p680 * var_ile);
        let assign8720_e8126: f64 = (p.p679 + assign8720_e8125);
        let assign8720_e8129: f64 = (p.p681 * var_iwe);
        let assign8720_e8130: f64 = (assign8720_e8126 + assign8720_e8129);
        let assign8720_e8133: f64 = (p.p682 * var_iae);
        let assign8720_e8134: f64 = (assign8720_e8130 + assign8720_e8133);
        let assign8720_e8135: f64 = (var_iiwecv * assign8720_e8134);
        (assign8720_e8135,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign8720_e8137;

        let assign8730_e8156: f64 = if (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]) { 1.0 } else { 0.0 };
        var_guard122 = assign8730_e8156;

        let (assign8740_e8176,) = {
    if ((var_guard41 != 0.0) && (var_guard122 != 0.0)) {
        let assign8740_e8164: f64 = (p.p684 * var_ile);
        let assign8740_e8165: f64 = (p.p683 + assign8740_e8164);
        let assign8740_e8168: f64 = (p.p685 * var_iwe);
        let assign8740_e8169: f64 = (assign8740_e8165 + assign8740_e8168);
        let assign8740_e8172: f64 = (p.p686 * var_iae);
        let assign8740_e8173: f64 = (assign8740_e8169 + assign8740_e8172);
        let assign8740_e8174: f64 = (var_iiwecv * assign8740_e8173);
        (assign8740_e8174,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign8740_e8176;

        s.b[1112] = (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]);
        s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1112]) {
            s.store_scalar(134, (s.v[325] * (((p.p687 + (p.p688 * var_ile)) + (p.p689 * var_iwe)) + (p.p690 * var_iae))));
        }

        s.b[1113] = (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]);
        s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1113]) {
            s.store_scalar(135, (var_iiwecv * (((p.p691 + (p.p692 * var_ile)) + (p.p693 * var_iwe)) + (p.p694 * var_iae))));
        }

        s.b[1114] = (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]);
        s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1114]) {
            s.store_scalar(136, (var_iiwecv * (((p.p695 + (p.p696 * var_ile)) + (p.p697 * var_iwe)) + (p.p698 * var_iae))));
        }

        let assign8810_e8312: f64 = if (((param_given[699] || param_given[700]) || param_given[701]) || param_given[702]) { 1.0 } else { 0.0 };
        var_guard126 = assign8810_e8312;

        let (assign8820_e8332,) = {
    if ((var_guard41 != 0.0) && (var_guard126 != 0.0)) {
        let assign8820_e8320: f64 = (p.p700 * var_ile);
        let assign8820_e8321: f64 = (p.p699 + assign8820_e8320);
        let assign8820_e8324: f64 = (p.p701 * var_iwe);
        let assign8820_e8325: f64 = (assign8820_e8321 + assign8820_e8324);
        let assign8820_e8328: f64 = (p.p702 * var_iae);
        let assign8820_e8329: f64 = (assign8820_e8325 + assign8820_e8328);
        let assign8820_e8330: f64 = (var_iiwcv * assign8820_e8329);
        (assign8820_e8330,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign8820_e8332;

        let assign8830_e8351: f64 = if (((param_given[703] || param_given[704]) || param_given[705]) || param_given[706]) { 1.0 } else { 0.0 };
        var_guard127 = assign8830_e8351;

        let (assign8840_e8371,) = {
    if ((var_guard41 != 0.0) && (var_guard127 != 0.0)) {
        let assign8840_e8359: f64 = (p.p704 * var_ile);
        let assign8840_e8360: f64 = (p.p703 + assign8840_e8359);
        let assign8840_e8363: f64 = (p.p705 * var_iwe);
        let assign8840_e8364: f64 = (assign8840_e8360 + assign8840_e8363);
        let assign8840_e8367: f64 = (p.p706 * var_iae);
        let assign8840_e8368: f64 = (assign8840_e8364 + assign8840_e8367);
        let assign8840_e8369: f64 = (var_iiwcv * assign8840_e8368);
        (assign8840_e8369,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign8840_e8371;

        s.b[1117] = (((param_given[707] || param_given[708]) || param_given[709]) || param_given[710]);
        s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1117]) {
            s.store_scalar(144, (s.v[315] * (((p.p707 + (p.p708 * var_ile)) + (p.p709 * var_iwe)) + (p.p710 * var_iae))));
        }

        s.b[1121] = (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]);
        s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1121]) {
            s.store_scalar(149, (((p.p723 + (p.p724 * var_ile)) + (p.p725 * var_iwe)) + (p.p726 * var_iae)));
        }

        s.b[1122] = (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]);
        s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1122]) {
            s.store_scalar(150, (((p.p727 + (p.p728 * var_ile)) + (p.p729 * var_iwe)) + (p.p730 * var_iae)));
        }

        s.b[1123] = (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]);
        s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1123]) {
            s.store_scalar(151, (((p.p731 + (p.p732 * var_ile)) + (p.p733 * var_iwe)) + (p.p734 * var_iae)));
        }

        s.b[1124] = (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]);
        s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1124]) {
            s.store_scalar(152, (((p.p735 + (p.p736 * var_ile)) + (p.p737 * var_iwe)) + (p.p738 * var_iae)));
        }

        s.b[1125] = (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]);
        s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1125]) {
            s.store_scalar(153, (((p.p739 + (p.p740 * var_ile)) + (p.p741 * var_iwe)) + (p.p742 * var_iae)));
        }

        s.b[1126] = (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]);
        s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1126]) {
            s.store_scale(154, 344, (1.0 / (var_le) * (((p.p743 + (p.p744 * var_ile)) + (p.p745 * var_iwe)) + (p.p746 * var_iae))));
        }

        s.b[1127] = (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]);
        s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1127]) {
            s.store_scalar(155, (((p.p747 + (p.p748 * var_ile)) + (p.p749 * var_iwe)) + (p.p750 * var_iae)));
        }

        s.b[1128] = (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]);
        s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1128]) {
            s.store_scalar(156, (s.v[315] * (((p.p751 + (p.p752 * var_ile)) + (p.p753 * var_iwe)) + (p.p754 * var_iae))));
        }

        s.b[1129] = (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]);
        s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1129]) {
            s.store_scalar(157, (((p.p755 + (p.p756 * var_ile)) + (p.p757 * var_iwe)) + (p.p758 * var_iae)));
        }

        s.b[1130] = (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]);
        s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1130]) {
            s.store_scalar(158, (((p.p759 + (p.p760 * var_ile)) + (p.p761 * var_iwe)) + (p.p762 * var_iae)));
        }

        s.b[1131] = (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]);
        s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1131]) {
            s.store_scalar(159, (s.v[315] * (((p.p763 + (p.p764 * var_ile)) + (p.p765 * var_iwe)) + (p.p766 * var_iae))));
        }

        s.b[1132] = (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]);
        s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1132]) {
            s.store_scalar(160, (((p.p771 + (p.p772 * var_ile)) + (p.p773 * var_iwe)) + (p.p774 * var_iae)));
        }

        s.b[1133] = (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]);
        s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1133]) {
            s.store_scalar(161, (((p.p767 + (p.p768 * var_ile)) + (p.p769 * var_iwe)) + (p.p770 * var_iae)));
        }

        s.b[1137] = (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]);
        s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1137]) {
            s.store_scalar(176, (((p.p787 + (p.p788 * var_ile)) + (p.p789 * var_iwe)) + (p.p790 * var_iae)));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(1019, 0.0);
            s.store_scalar(1020, 0.0);
            s.store_scalar(1018, 0.0);
            s.store_scalar(43, p.p795);
        }

        s.b[1138] = param_given[796];
        s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });

        if ((s.v[1030] != 0.0) && s.b[1138]) {
            s.store_scalar(43, p.p796);
        }

        s.b[1139] = (((s.v[9] > 0.0) && (s.v[10] > 0.0)) && ((s.v[5] == 1.0) || ((s.v[5] > 1.0) && (s.v[11] > 0.0))));
        s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });

        let mut assign9340_loop_guard: usize = 0;
        while {
            let assign9340_cond_e9222: f64 = (var_nf_i - 0.5);
            let assign9340_cond_e9224: f64 = if (((var_guard41 != 0.0) && s.b[1139]) && (s.v[1018] < assign9340_cond_e9222)) { 1.0 } else { 0.0 };
            assign9340_cond_e9224 != 0.0
        } {
            assign9340_loop_guard += 1;
            assert!(assign9340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[1030] != 0.0) && s.b[1139]) {
                s.store_add_ad_rhs(1019, 1019, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + var_l_i), (s.v[9] + (0.5 * var_l_i)))));
                s.store_add_ad_rhs(1020, 1020, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + var_l_i), (s.v[10] + (0.5 * var_l_i)))));
                s.store_offset(1018, 1018, 1.0);
            }
        }

        if ((s.v[1030] != 0.0) && s.b[1139]) {
            s.store_scale(1003, 1019, var_invnf);
            s.store_scale(1004, 1020, var_invnf);
            s.store_scalar(1005, (1.0 / (p.p791 + (0.5 * var_l_i))));
            s.store_scalar(1006, (1.0 / (p.p792 + (0.5 * var_l_i))));
        }

        if ((s.v[1030] != 0.0) && s.b[1139]) {
            s.store_scalar(1016, (if ((s.v[7] + s.v[310]) > 1e-9) { (var_l_i + var_dellps) } else { 1e-9 }));
        }

        if ((s.v[1030] != 0.0) && s.b[1139]) {
            s.store_scalar(1017, (if (((s.v[8] + s.v[311]) + p.p793) > 1e-9) { ((var_w_i + var_delwod) + p.p793) } else { 1e-9 }));
        }

        if ((s.v[1030] != 0.0) && s.b[1139]) {
            s.store_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p.p801);
            s.store_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p.p802);
            s.store_add_scaled_inputs_product_first_ad(1007, A::scale_offset(s.ad_value(1014), p.p798, 1.0), (1.0 + (p.p797 * (var_rta - 1.0))), 1015, (p.p799 * (1.0 + (p.p797 * (var_rta - 1.0)))), 1014, 1015, (p.p800 * (1.0 + (p.p797 * (var_rta - 1.0)))));
            s.store_div_scaled_inputs2_indices(1008, 1003, p.p794, 1004, p.p794, 1007, 1.0);
            s.store_div_scaled_inputs2_indices(1009, 1005, p.p794, 1006, p.p794, 1007, 1.0);
            s.store_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p.p807);
            s.store_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p.p808);
            s.store_add_scaled_inputs_product_first_ad(1010, A::scale_offset(s.ad_value(1014), p.p804, 1.0), 1.0, 1015, p.p805, 1014, 1015, p.p806);
            s.store_add_scaled_inputs4_indices(1012, 1003, 1.0, 1004, 1.0, 1005, -1.0, 1006, -1.0);
            s.store_div_scaled_offset_numerator(1013, s.ad_value(1008), 1.0, 1.0, A::offset(s.ad_value(1009), 1.0), 1.0);
            s.store_mul(69, 69, 1013);
            s.store_div_scaled_product3_mixed_iiaa(86, 86, 1013, A::scale_offset(s.ad_value(1009), p.p795, 1.0), 1.0, A::scale_offset(s.ad_value(1008), p.p795, 1.0), 1.0);
            s.store_div_scaled_product3_mixed_iiaa(125, 125, 1013, A::offset(A::mul(s.ad_value(43), s.ad_value(1009)), 1.0), 1.0, A::offset(A::mul(s.ad_value(43), s.ad_value(1008)), 1.0), 1.0);
            s.store_mul(154, 154, 1013);
            s.store_div_scaled_inputs_indices(1013, 1012, p.p803, 1010, 1.0);
            s.store_add(44, 44, 1013);
            s.store_add(149, 149, 1013);
            s.store_div_scaled_inputs_mixed_ia(1013, 1012, p.p809, A::powf(s.ad_value(1010), p.p810), 1.0);
            s.store_add(66, 66, 1013);
            s.store_add(159, 159, 1013);
        }

        s.b[1140] = ((((s.v[15] > 0.0) || (s.v[16] > 0.0)) || (s.v[17] > 0.0)) || (s.v[12] > 0.0));
        s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });

        s.b[1141] = (((s.v[15] == 0.0) && (s.v[16] == 0.0)) && (s.v[17] == 0.0));
        s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });

        if (((s.v[1030] != 0.0) && s.b[1140]) && s.b[1141]) {
            s.store_scalar(1012, (s.v[12] + var_w_i));
        }

        *var_cfr_p_slot = var_cfr_p;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_guard121_slot = var_guard121;
        *var_guard122_slot = var_guard122;
        *var_guard126_slot = var_guard126;
        *var_guard127_slot = var_guard127;
    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
        var_agidl_p: f64,
        var_agidld_p: f64,
        var_cgov_p: f64,
        var_cgovd_p: f64,
        var_epsrox_p: f64,
        var_igov_p: f64,
        var_igovd_p: f64,
        var_nov_p: f64,
        var_novd_p: f64,
        var_stig_p: f64,
        var_toxov_p: f64,
        var_toxovd_p: f64,
        var_w_i: f64,
        var_agidl_i_slot: &mut f64,
        var_agidld_i_slot: &mut f64,
        var_cgov_i_slot: &mut f64,
        var_cgovd_i_slot: &mut f64,
        var_epsrox_i_slot: &mut f64,
        var_igov_i_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_nov_i_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_stig_i_slot: &mut f64,
        var_toxov_i_slot: &mut f64,
        var_toxovd_i_slot: &mut f64,
    ) {
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_cgov_i: f64 = *var_cgov_i_slot;
        let mut var_cgovd_i: f64 = *var_cgovd_i_slot;
        let mut var_epsrox_i: f64 = *var_epsrox_i_slot;
        let mut var_igov_i: f64 = *var_igov_i_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_nov_i: f64 = *var_nov_i_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_stig_i: f64 = *var_stig_i_slot;
        let mut var_toxov_i: f64 = *var_toxov_i_slot;
        let mut var_toxovd_i: f64 = *var_toxovd_i_slot;

        if (((s.v[1030] != 0.0) && s.b[1140]) && s.b[1141]) {
            s.store_scalar(1013, (1.0 / p.p811));
            s.store_div_from_scalar_scaled_input(15, (p.p811 * p.p811), 1012, s.v[12]);
            s.store_add_scaled_product(16, A::exp_scaled_input(s.ad_value(1013), ((-10.0) * s.v[12])), (((0.1 * s.v[12]) + (0.01 * p.p811)) * 1.0 / (var_w_i)), A::scale_offset(s.ad_value(1012), 0.1, (0.01 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-10.0), s.ad_value(1013))), ((-1.0) * 1.0 / (var_w_i)));
            s.store_add_scaled_product(17, A::exp_scaled_input(s.ad_value(1013), ((-20.0) * s.v[12])), (((0.05 * s.v[12]) + (0.0025 * p.p811)) * 1.0 / (var_w_i)), A::scale_offset(s.ad_value(1012), 0.05, (0.0025 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-20.0), s.ad_value(1013))), ((-1.0) * 1.0 / (var_w_i)));
        }

        if ((s.v[1030] != 0.0) && s.b[1140]) {
            s.store_add_scaled_inputs3_indices(1012, 15, 1.0, 16, p.p812, 17, p.p813);
            s.store_add_scaled_product_indices(44, 44, 1.0, 348, 1012, 1.0);
            s.store_mul_offset_ad_rhs(69, 69, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0);
            s.store_add_scaled_product_indices(149, 149, 1.0, 348, 1012, 1.0);
            s.store_mul_offset_ad_rhs(154, 154, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0);
        }

        s.copy_ad(177, 44);

        s.copy_ad(178, 45);

        s.copy_ad(179, 46);

        s.copy_ad(181, 47);

        var_epsrox_i = var_epsrox_p;

        if (s.v[49] > 1e20) {
            if (s.v[49] < 1e26) {
                s.copy_ad(183, 49);
            } else {
                s.store_scalar(183, 1e26);
            }
        } else {
            s.store_scalar(183, 1e20);
        }

        if (s.v[50] > 0.01) {
            s.copy_ad(184, 50);
        } else {
            s.store_scalar(184, 0.01);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(185, 51);
        } else {
            s.store_scalar(185, 0.0);
        }

        s.copy_ad(186, 52);

        s.copy_ad(187, 53);

        if (s.v[54] > 0.0) {
            s.copy_ad(188, 54);
        } else {
            s.store_scalar(188, 0.0);
        }

        var_toxov_i = var_toxov_p;

        var_toxovd_i = var_toxovd_p;

        let (assign9860_e9839,) = {
    if (var_nov_p > 1e23) {
        let (assign9860_e9837,) = {
            if (var_nov_p < 1e27) {
                (var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9860_e9837,)
    } else {
        (1e23,)
    }
};
        var_nov_i = assign9860_e9839;

        let (assign9870_e9850,) = {
    if (var_novd_p > 1e23) {
        let (assign9870_e9848,) = {
            if (var_novd_p < 1e27) {
                (var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9870_e9848,)
    } else {
        (1e23,)
    }
};
        var_novd_i = assign9870_e9850;

        if (s.v[55] > 0.0) {
            s.copy_ad(189, 55);
        } else {
            s.store_scalar(189, 0.0);
        }

        if (s.v[57] > 0.0) {
            if (s.v[57] < 0.5) {
                s.copy_ad(191, 57);
            } else {
                s.store_scalar(191, 0.5);
            }
        } else {
            s.store_scalar(191, 0.0);
        }

        if (s.v[56] > 0.0) {
            if (s.v[56] < 1.0) {
                s.copy_ad(190, 56);
            } else {
                s.store_scalar(190, 1.0);
            }
        } else {
            s.store_scalar(190, 0.0);
        }

        s.copy_ad(180, 58);

        if (s.v[66] > 0.0) {
            s.copy_ad(196, 66);
        } else {
            s.store_scalar(196, 0.0);
        }

        if (s.v[68] > 0.0) {
            if (s.v[68] < 1.0) {
                s.copy_ad(198, 68);
            } else {
                s.store_scalar(198, 1.0);
            }
        } else {
            s.store_scalar(198, 0.0);
        }

        if (s.v[67] > 0.0) {
            s.copy_ad(197, 67);
        } else {
            s.store_scalar(197, 0.0);
        }

        if (s.v[63] > 0.0) {
            s.copy_ad(199, 63);
        } else {
            s.store_scalar(199, 0.0);
        }

        if (s.v[65] > 0.0) {
            if (s.v[65] < 1.0) {
                s.copy_ad(200, 65);
            } else {
                s.store_scalar(200, 1.0);
            }
        } else {
            s.store_scalar(200, 0.0);
        }

        if (s.v[64] > 0.0) {
            s.copy_ad(201, 64);
        } else {
            s.store_scalar(201, 0.0);
        }

        if (s.v[69] > 0.0) {
            s.copy_ad(202, 69);
        } else {
            s.store_scalar(202, 0.0);
        }

        s.copy_ad(203, 70);

        if (s.v[71] > 0.0) {
            s.copy_ad(204, 71);
        } else {
            s.store_scalar(204, 0.0);
        }

        s.copy_ad(205, 72);

        if (s.v[73] > 0.0) {
            s.copy_ad(206, 73);
        } else {
            s.store_scalar(206, 0.0);
        }

        s.copy_ad(207, 74);

        if (s.v[75] > 0.0) {
            s.copy_ad(208, 75);
        } else {
            s.store_scalar(208, 0.0);
        }

        s.copy_ad(209, 76);

        if (s.v[77] > 0.0) {
            s.copy_ad(210, 77);
        } else {
            s.store_scalar(210, 0.0);
        }

        s.copy_ad(211, 78);

        if (s.v[79] > 0.0) {
            s.copy_ad(212, 79);
        } else {
            s.store_scalar(212, 0.0);
        }

        s.copy_ad(213, 80);

        s.copy_ad(214, 81);

        if (s.v[82] > 0.0) {
            s.copy_ad(215, 82);
        } else {
            s.store_scalar(215, 0.0);
        }

        s.copy_ad(216, 83);

        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(217, 84);
            } else {
                s.store_scalar(217, 1.0);
            }
        } else {
            s.store_scalar(217, (-0.5));
        }

        if (s.v[85] > (-0.5)) {
            s.copy_ad(218, 85);
        } else {
            s.store_scalar(218, (-0.5));
        }

        if (s.v[86] > 0.0) {
            s.copy_ad(219, 86);
        } else {
            s.store_scalar(219, 0.0);
        }

        s.copy_ad(220, 87);

        if (s.v[88] > (-0.5)) {
            if (s.v[88] < 1.0) {
                s.copy_ad(221, 88);
            } else {
                s.store_scalar(221, 1.0);
            }
        } else {
            s.store_scalar(221, (-0.5));
        }

        if (s.v[89] > (-0.5)) {
            s.copy_ad(222, 89);
        } else {
            s.store_scalar(222, (-0.5));
        }

        if (s.v[90] > 0.01) {
            s.copy_ad(223, 90);
        } else {
            s.store_scalar(223, 0.01);
        }

        if (s.v[91] > 2.0) {
            s.copy_ad(224, 91);
        } else {
            s.store_scalar(224, 2.0);
        }

        if (s.v[92] > 0.0) {
            s.copy_ad(225, 92);
        } else {
            s.store_scalar(225, 0.0);
        }

        if (s.v[93] > 0.0) {
            s.copy_ad(226, 93);
        } else {
            s.store_scalar(226, 0.0);
        }

        if (s.v[94] > 0.0) {
            s.copy_ad(227, 94);
        } else {
            s.store_scalar(227, 0.0);
        }

        s.copy_ad(228, 95);

        if (s.v[96] > 0.0) {
            s.copy_ad(229, 96);
        } else {
            s.store_scalar(229, 0.0);
        }

        s.copy_ad(230, 97);

        s.copy_ad(231, 98);

        if (s.v[99] > 0.0) {
            s.copy_ad(232, 99);
        } else {
            s.store_scalar(232, 0.0);
        }

        if (s.v[100] > 0.0) {
            s.copy_ad(233, 100);
        } else {
            s.store_scalar(233, 0.0);
        }

        if (s.v[101] > 1e-12) {
            s.copy_ad(234, 101);
        } else {
            s.store_scalar(234, 1e-12);
        }

        s.copy_ad(235, 102);

        if (s.v[103] > 0.0) {
            s.copy_ad(236, 103);
        } else {
            s.store_scalar(236, 0.0);
        }

        let (assign10330_e10094,) = {
    if (var_igov_p > 0.0) {
        (var_igov_p,)
    } else {
        (0.0,)
    }
};
        var_igov_i = assign10330_e10094;

        let (assign10340_e10100,) = {
    if (var_igovd_p > 0.0) {
        (var_igovd_p,)
    } else {
        (0.0,)
    }
};
        var_igovd_i = assign10340_e10100;

        var_stig_i = var_stig_p;

        s.copy_ad(240, 107);

        s.copy_ad(241, 108);

        s.copy_ad(242, 109);

        s.copy_ad(243, 110);

        s.copy_ad(244, 111);

        s.copy_ad(245, 112);

        s.copy_ad(246, 113);

        let (assign10430_e10114,) = {
    if (var_agidl_p > 0.0) {
        (var_agidl_p,)
    } else {
        (0.0,)
    }
};
        var_agidl_i = assign10430_e10114;

        let (assign10440_e10120,) = {
    if (var_agidld_p > 0.0) {
        (var_agidld_p,)
    } else {
        (0.0,)
    }
};
        var_agidld_i = assign10440_e10120;

        s.copy_ad(249, 116);

        s.copy_ad(250, 117);

        s.copy_ad(251, 118);

        s.copy_ad(252, 119);

        s.copy_ad(253, 120);

        s.copy_ad(254, 121);

        if (s.v[122] > 0.0) {
            s.copy_ad(255, 122);
        } else {
            s.store_scalar(255, 0.0);
        }

        s.copy_ad(256, 123);

        if (s.v[124] > 0.0) {
            s.copy_ad(257, 124);
        } else {
            s.store_scalar(257, 0.0);
        }

        if (s.v[125] > 0.0) {
            s.copy_ad(258, 125);
        } else {
            s.store_scalar(258, 0.0);
        }

        if (s.v[126] > 2.0) {
            s.copy_ad(259, 126);
        } else {
            s.store_scalar(259, 2.0);
        }

        s.copy_ad(260, 127);

        if (s.v[128] > 0.0) {
            s.copy_ad(261, 128);
        } else {
            s.store_scalar(261, 0.0);
        }

        let (assign10580_e10164,) = {
    if (var_cgov_p > 0.0) {
        (var_cgov_p,)
    } else {
        (0.0,)
    }
};
        var_cgov_i = assign10580_e10164;

        let (assign10590_e10170,) = {
    if (var_cgovd_p > 0.0) {
        (var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        var_cgovd_i = assign10590_e10170;

        s.copy_ad(264, 131);

        s.copy_ad(265, 132);

        s.copy_ad(266, 133);

        if (s.v[134] > 0.0) {
            s.copy_ad(267, 134);
        } else {
            s.store_scalar(267, 0.0);
        }

        if (s.v[135] > 0.0) {
            s.copy_ad(268, 135);
        } else {
            s.store_scalar(268, 0.0);
        }

        if (s.v[136] > 0.0) {
            s.copy_ad(269, 136);
        } else {
            s.store_scalar(269, 0.0);
        }

        s.copy_ad(270, 137);

        s.copy_ad(271, 138);

        s.copy_ad(272, 139);

        *var_agidl_i_slot = var_agidl_i;
        *var_agidld_i_slot = var_agidld_i;
        *var_cgov_i_slot = var_cgov_i;
        *var_cgovd_i_slot = var_cgovd_i;
        *var_epsrox_i_slot = var_epsrox_i;
        *var_igov_i_slot = var_igov_i;
        *var_igovd_i_slot = var_igovd_i;
        *var_nov_i_slot = var_nov_i;
        *var_novd_i_slot = var_novd_i;
        *var_stig_i_slot = var_stig_i;
        *var_toxov_i_slot = var_toxov_i;
        *var_toxovd_i_slot = var_toxovd_i;
    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
        var_agidl_i: f64,
        var_cfr_p: f64,
        var_cfrd_p: f64,
        var_cgov_i: f64,
        var_epsrox_i: f64,
        var_epssi: f64,
        var_igov_i: f64,
        var_inv_phita: f64,
        var_nf_i: f64,
        var_nov_i: f64,
        var_rbulk_p: f64,
        var_rde_p: f64,
        var_rg_p: f64,
        var_rjund_p: f64,
        var_rjuns_p: f64,
        var_rse_p: f64,
        var_rwell_p: f64,
        var_toxov_i: f64,
        var_agidld_i_slot: &mut f64,
        var_cfr_i_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cgovd_i_slot: &mut f64,
        var_coxovprime_slot: &mut f64,
        var_coxovprime_d_slot: &mut f64,
        var_epsox_slot: &mut f64,
        var_gov2_d_slot: &mut f64,
        var_gov2_s_slot: &mut f64,
        var_gov_d_slot: &mut f64,
        var_gov_s_slot: &mut f64,
        var_guard153_slot: &mut f64,
        var_guard157_slot: &mut f64,
        var_guard158_slot: &mut f64,
        var_guard159_slot: &mut f64,
        var_guard160_slot: &mut f64,
        var_guard161_slot: &mut f64,
        var_guard162_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_inv_gov_slot: &mut f64,
        var_mult_inst_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_rbulk_i_slot: &mut f64,
        var_rde_i_slot: &mut f64,
        var_rg_i_slot: &mut f64,
        var_rjund_i_slot: &mut f64,
        var_rjuns_i_slot: &mut f64,
        var_rse_i_slot: &mut f64,
        var_rwell_i_slot: &mut f64,
        var_sp_ov_a_d_slot: &mut f64,
        var_sp_ov_a_s_slot: &mut f64,
        var_sp_ov_delta_slot: &mut f64,
        var_sp_ov_delta1_d_slot: &mut f64,
        var_sp_ov_delta1_s_slot: &mut f64,
        var_sp_ov_eps_slot: &mut f64,
        var_sp_ov_eps2_d_slot: &mut f64,
        var_sp_ov_eps2_s_slot: &mut f64,
        var_toxovd_i_slot: &mut f64,
    ) {
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_cfr_i: f64 = *var_cfr_i_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cgovd_i: f64 = *var_cgovd_i_slot;
        let mut var_coxovprime: f64 = *var_coxovprime_slot;
        let mut var_coxovprime_d: f64 = *var_coxovprime_d_slot;
        let mut var_epsox: f64 = *var_epsox_slot;
        let mut var_gov2_d: f64 = *var_gov2_d_slot;
        let mut var_gov2_s: f64 = *var_gov2_s_slot;
        let mut var_gov_d: f64 = *var_gov_d_slot;
        let mut var_gov_s: f64 = *var_gov_s_slot;
        let mut var_guard153: f64 = *var_guard153_slot;
        let mut var_guard157: f64 = *var_guard157_slot;
        let mut var_guard158: f64 = *var_guard158_slot;
        let mut var_guard159: f64 = *var_guard159_slot;
        let mut var_guard160: f64 = *var_guard160_slot;
        let mut var_guard161: f64 = *var_guard161_slot;
        let mut var_guard162: f64 = *var_guard162_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_inv_gov: f64 = *var_inv_gov_slot;
        let mut var_mult_inst: f64 = *var_mult_inst_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_rbulk_i: f64 = *var_rbulk_i_slot;
        let mut var_rde_i: f64 = *var_rde_i_slot;
        let mut var_rg_i: f64 = *var_rg_i_slot;
        let mut var_rjund_i: f64 = *var_rjund_i_slot;
        let mut var_rjuns_i: f64 = *var_rjuns_i_slot;
        let mut var_rse_i: f64 = *var_rse_i_slot;
        let mut var_rwell_i: f64 = *var_rwell_i_slot;
        let mut var_sp_ov_a_d: f64 = *var_sp_ov_a_d_slot;
        let mut var_sp_ov_a_s: f64 = *var_sp_ov_a_s_slot;
        let mut var_sp_ov_delta: f64 = *var_sp_ov_delta_slot;
        let mut var_sp_ov_delta1_d: f64 = *var_sp_ov_delta1_d_slot;
        let mut var_sp_ov_delta1_s: f64 = *var_sp_ov_delta1_s_slot;
        let mut var_sp_ov_eps: f64 = *var_sp_ov_eps_slot;
        let mut var_sp_ov_eps2_d: f64 = *var_sp_ov_eps2_d_slot;
        let mut var_sp_ov_eps2_s: f64 = *var_sp_ov_eps2_s_slot;
        let mut var_toxovd_i: f64 = *var_toxovd_i_slot;

        s.copy_ad(273, 140);

        let (assign10700_e10201,) = {
    if (var_cfr_p > 0.0) {
        (var_cfr_p,)
    } else {
        (0.0,)
    }
};
        var_cfr_i = assign10700_e10201;

        let (assign10710_e10207,) = {
    if (var_cfrd_p > 0.0) {
        (var_cfrd_p,)
    } else {
        (0.0,)
    }
};
        var_cfrd_i = assign10710_e10207;

        s.copy_ad(276, 143);

        if (s.v[144] > 0.0) {
            s.copy_ad(277, 144);
        } else {
            s.store_scalar(277, 0.0);
        }

        s.copy_ad(282, 149);

        s.copy_ad(283, 150);

        s.copy_ad(284, 151);

        if (s.v[152] > 1e20) {
            if (s.v[152] < 1e26) {
                s.copy_ad(285, 152);
            } else {
                s.store_scalar(285, 1e26);
            }
        } else {
            s.store_scalar(285, 1e20);
        }

        if (s.v[153] > 0.0) {
            s.copy_ad(286, 153);
        } else {
            s.store_scalar(286, 0.0);
        }

        if (s.v[154] > 0.0) {
            s.copy_ad(287, 154);
        } else {
            s.store_scalar(287, 0.0);
        }

        s.copy_ad(288, 155);

        if (s.v[156] > 0.0) {
            s.copy_ad(289, 156);
        } else {
            s.store_scalar(289, 0.0);
        }

        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(290, 157);
            } else {
                s.store_scalar(290, 1.0);
            }
        } else {
            s.store_scalar(290, 0.0);
        }

        if (s.v[158] > 0.0) {
            s.copy_ad(291, 158);
        } else {
            s.store_scalar(291, 0.0);
        }

        if (s.v[159] > 0.0) {
            s.copy_ad(292, 159);
        } else {
            s.store_scalar(292, 0.0);
        }

        if (s.v[161] > 0.0) {
            if (s.v[161] < 1.0) {
                s.copy_ad(294, 161);
            } else {
                s.store_scalar(294, 1.0);
            }
        } else {
            s.store_scalar(294, 0.0);
        }

        if (s.v[160] > 0.0) {
            s.copy_ad(293, 160);
        } else {
            s.store_scalar(293, 0.0);
        }

        let (assign10960_e10332,) = {
    if (var_rg_p > 0.0) {
        (var_rg_p,)
    } else {
        (0.0,)
    }
};
        var_rg_i = assign10960_e10332;

        var_rse_i = var_rse_p;

        var_rde_i = var_rde_p;

        var_rbulk_i = var_rbulk_p;

        var_rjuns_i = var_rjuns_p;

        var_rjund_i = var_rjund_p;

        var_rwell_i = var_rwell_p;

        let assign11030_e10341: f64 = (p.p31 * var_nf_i);
        let (assign11030_e10348,) = {
    if (assign11030_e10341 > 0.0) {
        let assign11030_e10346: f64 = (p.p31 * var_nf_i);
        (assign11030_e10346,)
    } else {
        (0.0,)
    }
};
        var_mult_inst = assign11030_e10348;

        s.store_scalar(20, p.p16);

        s.store_scalar(21, p.p15);

        s.store_scalar(22, p.p18);

        s.store_scalar(23, p.p17);

        if (s.v[176] > 0.0) {
            s.copy_ad(307, 176);
        } else {
            s.store_scalar(307, 0.0);
        }

        let assign11090_e10361: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard153 = assign11090_e10361;

        let (assign11100_e10365,) = {
    if (var_guard153 != 0.0) {
        (var_toxov_i,)
    } else {
        (var_toxovd_i,)
    }
};
        var_toxovd_i = assign11100_e10365;

        let (assign11110_e10369,) = {
    if (var_guard153 != 0.0) {
        (var_nov_i,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign11110_e10369;

        let (assign11120_e10373,) = {
    if (var_guard153 != 0.0) {
        (var_agidl_i,)
    } else {
        (var_agidld_i,)
    }
};
        var_agidld_i = assign11120_e10373;

        if (s.v[1142] != 0.0) {
            s.copy_ad(250, 249);
            s.copy_ad(252, 251);
            s.copy_ad(254, 253);
        }

        let (assign11160_e10389,) = {
    if (var_guard153 != 0.0) {
        (var_igov_i,)
    } else {
        (var_igovd_i,)
    }
};
        var_igovd_i = assign11160_e10389;

        if (s.v[1142] != 0.0) {
            s.copy_ad(244, 242);
            s.copy_ad(245, 243);
        }

        let (assign11190_e10401,) = {
    if (var_guard153 != 0.0) {
        (var_cgov_i,)
    } else {
        (var_cgovd_i,)
    }
};
        var_cgovd_i = assign11190_e10401;

        if (s.v[1142] != 0.0) {
            s.copy_ad(265, 264);
            s.copy_ad(269, 268);
        }

        let (assign11220_e10413,) = {
    if (var_guard153 != 0.0) {
        (var_cfr_i,)
    } else {
        (var_cfrd_i,)
    }
};
        var_cfrd_i = assign11220_e10413;

        let assign11230_e10416: f64 = (8.8541878176e-12 * var_epsrox_i);
        var_epsox = assign11230_e10416;

        s.store_div_from_scalar(769, var_epsox, 181);

        s.store_square(770, 181);

        s.store_scale(771, 769, 6.241449993689894e18);

        s.store_mul(772, 257, 183);

        if (s.v[772] > 1e20) {
            if (s.v[772] < 1e26) {
            } else {
                s.store_scalar(772, 1e26);
            }
        } else {
            s.store_scalar(772, 1e20);
        }

        s.store_scalar(773, 0.0);

        s.b[1143] = (p.p52 > 0.0);
        s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });

        if s.b[1143] {
            s.store_scale_ad(773, A::powf(s.ad_value(769), 0.6666666666666666), ((0.4 * 5.951993) * p.p52));
        }

        s.b[1144] = (s.v[0] == (-1.0));
        s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });

        if (s.b[1143] && s.b[1144]) {
            s.store_scale(773, 773, (7.448711 / 5.951993));
        }

        s.store_scale(774, 769, (1e-8 * 1.0 / (var_epssi)));

        s.store_scale(775, 214, 0.5);

        s.store_scalar(776, 0.5);

        s.b[1145] = (s.v[0] == (-1.0));
        s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });

        if s.b[1145] {
            s.store_scale(775, 214, 0.3333333333333333);
            s.store_scalar(776, 0.3333333333333333);
        }

        s.store_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(224)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(777, s.ad_value(1011), (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(259)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(778, s.ad_value(1011), (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_div_from_scalar(779, 1.0, 228);

        let assign11450_e10554: f64 = (var_epsox / var_toxov_i);
        var_coxovprime = assign11450_e10554;

        let assign11460_e10557: f64 = (var_epsox / var_toxovd_i);
        var_coxovprime_d = assign11460_e10557;

        let assign11470_e10560: f64 = (2.0 * 1.6021918e-19);
        let assign11470_e10562: f64 = (assign11470_e10560 * var_nov_i);
        let assign11470_e10564: f64 = (assign11470_e10562 * var_epssi);
        let assign11470_e10566: f64 = (assign11470_e10564 * var_inv_phita);
        let assign11470_e10567: f64 = (assign11470_e10566).sqrt();
        let assign11470_e10569: f64 = (assign11470_e10567 / var_coxovprime);
        var_gov_s = assign11470_e10569;

        let assign11480_e10572: f64 = (2.0 * 1.6021918e-19);
        let assign11480_e10574: f64 = (assign11480_e10572 * var_novd_i);
        let assign11480_e10576: f64 = (assign11480_e10574 * var_epssi);
        let assign11480_e10578: f64 = (assign11480_e10576 * var_inv_phita);
        let assign11480_e10579: f64 = (assign11480_e10578).sqrt();
        let assign11480_e10581: f64 = (assign11480_e10579 / var_coxovprime_d);
        var_gov_d = assign11480_e10581;

        let assign11490_e10584: f64 = (var_gov_s * var_gov_s);
        var_gov2_s = assign11490_e10584;

        let assign11500_e10587: f64 = (var_gov_d * var_gov_d);
        var_gov2_d = assign11500_e10587;

        s.store_offset_div_ad(786, A::ln(A::offset(A::exp_scaled_input(s.ad_value(266), (0.005 * var_inv_phita)), (-1.0))), s.ad_value(266), (-((((((0.005 * var_inv_phita)) as f64).exp() - 1.0)) as f64).ln()));

        s.store_offset(787, 786, (((0.5 * var_gov_s)) as f64).ln());

        s.store_offset(788, 786, (((0.5 * var_gov_d)) as f64).ln());

        let assign11540_e10621: f64 = (1.0 / var_gov_s);
        var_inv_gov = assign11540_e10621;

        let assign11550_e10624: f64 = (3.1 * var_gov_s);
        let assign11550_e10626: f64 = (assign11550_e10624 + 8.5);
        var_sp_ov_eps = assign11550_e10626;

        let assign11560_e10629: f64 = (var_sp_ov_eps * var_sp_ov_eps);
        var_sp_ov_eps2_s = assign11560_e10629;

        let assign11570_e10632: f64 = (0.5 * var_sp_ov_eps);
        var_sp_ov_delta = assign11570_e10632;

        let assign11580_e10635: f64 = if var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        var_guard157 = assign11580_e10635;

        let (assign11590_e10641,) = {
    if (var_guard157 != 0.0) {
        let assign11590_e10639: f64 = (64.0 * var_inv_gov);
        (assign11590_e10639,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11590_e10641;

        let assign11600_e10644: f64 = if var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        var_guard158 = assign11600_e10644;

        let (assign11610_e10655,) = {
    if ((var_guard157 == 0.0) && (var_guard158 != 0.0)) {
        let assign11610_e10651: f64 = (22.0 * var_inv_gov);
        let assign11610_e10653: f64 = (assign11610_e10651 + 3.0);
        (assign11610_e10653,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11610_e10655;

        let assign11620_e10658: f64 = if var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        var_guard159 = assign11620_e10658;

        let (assign11630_e10673,) = {
    if (((var_guard157 == 0.0) && (var_guard158 == 0.0)) && (var_guard159 != 0.0)) {
        let assign11630_e10667: f64 = (-7.2);
        let assign11630_e10669: f64 = (assign11630_e10667 * var_inv_gov);
        let assign11630_e10671: f64 = (assign11630_e10669 + 15.5);
        (assign11630_e10671,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11630_e10673;

        let (assign11640_e10684,) = {
    if (((var_guard157 == 0.0) && (var_guard158 == 0.0)) && (var_guard159 == 0.0)) {
        (var_gov_s,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11640_e10684;

        let assign11650_e10688: f64 = (var_gov2_s * 0.5);
        let assign11650_e10689: f64 = (var_sp_ov_delta + assign11650_e10688);
        let assign11650_e10694: f64 = (var_gov2_s * 0.25);
        let assign11650_e10695: f64 = (var_sp_ov_delta + assign11650_e10694);
        let assign11650_e10697: f64 = (assign11650_e10695 + var_sp_ov_a_s);
        let assign11650_e10698: f64 = (assign11650_e10697).sqrt();
        let assign11650_e10699: f64 = (var_gov_s * assign11650_e10698);
        let assign11650_e10700: f64 = (assign11650_e10689 - assign11650_e10699);
        var_sp_ov_delta1_s = assign11650_e10700;

        let assign11660_e10703: f64 = (1.0 / var_gov_d);
        var_inv_gov = assign11660_e10703;

        let assign11670_e10706: f64 = (3.1 * var_gov_d);
        let assign11670_e10708: f64 = (assign11670_e10706 + 8.5);
        var_sp_ov_eps = assign11670_e10708;

        let assign11680_e10711: f64 = (var_sp_ov_eps * var_sp_ov_eps);
        var_sp_ov_eps2_d = assign11680_e10711;

        let assign11690_e10714: f64 = (0.5 * var_sp_ov_eps);
        var_sp_ov_delta = assign11690_e10714;

        let assign11700_e10717: f64 = if var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        var_guard160 = assign11700_e10717;

        let (assign11710_e10723,) = {
    if (var_guard160 != 0.0) {
        let assign11710_e10721: f64 = (64.0 * var_inv_gov);
        (assign11710_e10721,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11710_e10723;

        let assign11720_e10726: f64 = if var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        var_guard161 = assign11720_e10726;

        let (assign11730_e10737,) = {
    if ((var_guard160 == 0.0) && (var_guard161 != 0.0)) {
        let assign11730_e10733: f64 = (22.0 * var_inv_gov);
        let assign11730_e10735: f64 = (assign11730_e10733 + 3.0);
        (assign11730_e10735,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11730_e10737;

        let assign11740_e10740: f64 = if var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        var_guard162 = assign11740_e10740;

        let (assign11750_e10755,) = {
    if (((var_guard160 == 0.0) && (var_guard161 == 0.0)) && (var_guard162 != 0.0)) {
        let assign11750_e10749: f64 = (-7.2);
        let assign11750_e10751: f64 = (assign11750_e10749 * var_inv_gov);
        let assign11750_e10753: f64 = (assign11750_e10751 + 15.5);
        (assign11750_e10753,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11750_e10755;

        let (assign11760_e10766,) = {
    if (((var_guard160 == 0.0) && (var_guard161 == 0.0)) && (var_guard162 == 0.0)) {
        (var_gov_d,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11760_e10766;

        let assign11770_e10770: f64 = (var_gov2_d * 0.5);
        let assign11770_e10771: f64 = (var_sp_ov_delta + assign11770_e10770);
        let assign11770_e10776: f64 = (var_gov2_d * 0.25);
        let assign11770_e10777: f64 = (var_sp_ov_delta + assign11770_e10776);
        let assign11770_e10779: f64 = (assign11770_e10777 + var_sp_ov_a_d);
        let assign11770_e10780: f64 = (assign11770_e10779).sqrt();
        let assign11770_e10781: f64 = (var_gov_d * assign11770_e10780);
        let assign11770_e10782: f64 = (assign11770_e10771 - assign11770_e10781);
        var_sp_ov_delta1_d = assign11770_e10782;

        s.store_add_scaled_inputs_ad(728, A::offset(s.ad_value(187), s.v[362]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(183), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]));

        if (!(s.v[728] > 0.05)) {
            s.store_scalar(728, 0.05);
        }

        s.store_div_ad_lhs(729, A::sqrt_scaled_input(s.ad_value(183), (((2.0 * 1.6021918e-19) * var_epssi) * s.v[361])), 769);

        s.store_scalar(730, 0.0);

        s.store_scalar(731, 0.0);

        *var_agidld_i_slot = var_agidld_i;
        *var_cfr_i_slot = var_cfr_i;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cgovd_i_slot = var_cgovd_i;
        *var_coxovprime_slot = var_coxovprime;
        *var_coxovprime_d_slot = var_coxovprime_d;
        *var_epsox_slot = var_epsox;
        *var_gov2_d_slot = var_gov2_d;
        *var_gov2_s_slot = var_gov2_s;
        *var_gov_d_slot = var_gov_d;
        *var_gov_s_slot = var_gov_s;
        *var_guard153_slot = var_guard153;
        *var_guard157_slot = var_guard157;
        *var_guard158_slot = var_guard158;
        *var_guard159_slot = var_guard159;
        *var_guard160_slot = var_guard160;
        *var_guard161_slot = var_guard161;
        *var_guard162_slot = var_guard162;
        *var_igovd_i_slot = var_igovd_i;
        *var_inv_gov_slot = var_inv_gov;
        *var_mult_inst_slot = var_mult_inst;
        *var_novd_i_slot = var_novd_i;
        *var_rbulk_i_slot = var_rbulk_i;
        *var_rde_i_slot = var_rde_i;
        *var_rg_i_slot = var_rg_i;
        *var_rjund_i_slot = var_rjund_i;
        *var_rjuns_i_slot = var_rjuns_i;
        *var_rse_i_slot = var_rse_i;
        *var_rwell_i_slot = var_rwell_i;
        *var_sp_ov_a_d_slot = var_sp_ov_a_d;
        *var_sp_ov_a_s_slot = var_sp_ov_a_s;
        *var_sp_ov_delta_slot = var_sp_ov_delta;
        *var_sp_ov_delta1_d_slot = var_sp_ov_delta1_d;
        *var_sp_ov_delta1_s_slot = var_sp_ov_delta1_s;
        *var_sp_ov_eps_slot = var_sp_ov_eps;
        *var_sp_ov_eps2_d_slot = var_sp_ov_eps2_d;
        *var_sp_ov_eps2_s_slot = var_sp_ov_eps2_s;
        *var_toxovd_i_slot = var_toxovd_i;
    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
        var_agidl_i: f64,
        var_agidld_i: f64,
        var_epssi: f64,
        var_rbulk_i: f64,
        var_rde_i: f64,
        var_rg_i: f64,
        var_rjund_i: f64,
        var_rjuns_i: f64,
        var_rse_i: f64,
        var_rta: f64,
        var_rwell_i: f64,
        var_stig_i: f64,
        var_toxov_i: f64,
        var_toxovd_i: f64,
        var_gbulk_slot: &mut f64,
        var_gdrain_slot: &mut f64,
        var_ggate_slot: &mut f64,
        var_gjund_slot: &mut f64,
        var_gjuns_slot: &mut f64,
        var_gsource_slot: &mut f64,
        var_guard171_slot: &mut f64,
        var_guard172_slot: &mut f64,
        var_guard173_slot: &mut f64,
        var_guard174_slot: &mut f64,
        var_guard175_slot: &mut f64,
        var_guard176_slot: &mut f64,
        var_guard177_slot: &mut f64,
        var_gwell_slot: &mut f64,
        var_igov_i_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_tf_ig_slot: &mut f64,
    ) {
        let mut var_gbulk: f64 = *var_gbulk_slot;
        let mut var_gdrain: f64 = *var_gdrain_slot;
        let mut var_ggate: f64 = *var_ggate_slot;
        let mut var_gjund: f64 = *var_gjund_slot;
        let mut var_gjuns: f64 = *var_gjuns_slot;
        let mut var_gsource: f64 = *var_gsource_slot;
        let mut var_guard171: f64 = *var_guard171_slot;
        let mut var_guard172: f64 = *var_guard172_slot;
        let mut var_guard173: f64 = *var_guard173_slot;
        let mut var_guard174: f64 = *var_guard174_slot;
        let mut var_guard175: f64 = *var_guard175_slot;
        let mut var_guard176: f64 = *var_guard176_slot;
        let mut var_guard177: f64 = *var_guard177_slot;
        let mut var_gwell: f64 = *var_gwell_slot;
        let mut var_igov_i: f64 = *var_igov_i_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_tf_ig: f64 = *var_tf_ig_slot;

        s.b[1152] = (s.v[188] > 0.0);
        s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });

        if s.b[1152] {
            s.store_div_from_scalar(732, 80000000.0, 770);
        }

        if s.b[1152] {
            if (s.v[188] > s.v[732]) {
                s.copy_ad(731, 188);
            } else {
                s.copy_ad(731, 732);
            }
        }

        if s.b[1152] {
            if (5e24 > s.v[731]) {
                s.store_scalar(731, 5e24);
            } else {
            }
        }

        if s.b[1152] {
            s.store_div_scaled_product_indices(730, 769, 769, (2.0 * s.v[715]), 731, (1.6021918e-19 * var_epssi));
        }

        s.store_scalar(733, ((100.0 * s.v[715]) * s.v[715]));

        s.b[1153] = (p.p52 > 0.0);
        s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });

        if s.b[1153] {
            s.store_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(729), s.ad_value(729), s.ad_value(728), s.v[715]));
            s.store_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);
            s.store_add(728, 728, 735);
            s.store_mul_offset_ad_rhs(729, 729, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0);
        }

        s.store_sqrt(736, 728);

        s.store_scale(737, 728, 0.95);

        s.store_scaled_mul(738, 728, 728, 0.0025);

        s.copy_ad(739, 738);

        s.store_scaled_sqrt(740, 739, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(741, 737, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(737), s.ad_value(740))), s.ad_value(738)), (-0.5));

        s.store_scaled_offset(742, 728, s.v[362], 0.5);

        s.store_sub_ad_lhs(743, A::sqrt(A::add(s.ad_value(185), s.ad_value(728))), 736);

        s.store_add_scaled_inputs3_sqrt_first_mixed_aii(744, A::add_scaled_inputs3(s.ad_value(185), 1.0, s.ad_value(186), 1.0, s.ad_value(728), 1.0), 1.0, 736, (-1.0), 743, -1.0);

        s.store_add_scaled_inputs3_offset_mixed_iia(745, 187, 1.0, 256, 1.0, A::ln_scaled_input(A::mul(s.ad_value(772), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]), s.v[362]);

        if (!(s.v[745] > 0.05)) {
            s.store_scalar(745, 0.05);
        }

        s.store_div_ad_lhs(746, A::sqrt_scaled_input(s.ad_value(772), (((2.0 * 1.6021918e-19) * var_epssi) * s.v[361])), 769);

        s.b[1154] = (p.p52 > 0.0);
        s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });

        if s.b[1154] {
            s.store_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(746), s.ad_value(746), s.ad_value(745), s.v[715]));
            s.store_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);
            s.store_add(745, 745, 735);
            s.store_mul_offset_ad_rhs(746, 746, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0);
        }

        s.store_scale(747, 745, 0.95);

        s.store_scaled_mul(748, 745, 745, 0.0025);

        s.copy_ad(749, 748);

        s.store_scaled_sqrt(740, 749, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(750, 747, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(747), s.ad_value(740))), s.ad_value(748)), (-0.5));

        s.store_offset_add_scaled_product(700, s.ad_value(177), 1.0, s.ad_value(178), A::scale_offset(s.ad_value(179), s.v[358], 1.0), s.v[358], s.v[21]);

        s.store_exp_scaled_input(751, 180, s.v[360]);

        s.store_mul(701, 189, 751);

        s.store_scale(702, 190, 1.0 / (s.v[359]));

        s.store_exp_scaled_input(752, 203, s.v[360]);

        s.store_mul(703, 202, 752);

        s.store_scaled_mul(716, 703, 769, s.v[20]);

        s.store_mul_ad_rhs(705, 206, A::exp_scaled_input(s.ad_value(207), s.v[360]));

        s.store_exp_scaled_input(753, 205, s.v[360]);

        s.store_mul(704, 204, 753);

        s.store_mul_ad_rhs(707, 210, A::exp_scaled_input(s.ad_value(211), s.v[360]));

        s.store_exp_scaled_input(754, 209, s.v[360]);

        s.store_mul(706, 208, 754);

        s.store_exp_scaled_input(755, 213, s.v[360]);

        s.store_mul(708, 212, 755);

        s.store_exp_scaled_input(756, 216, s.v[360]);

        s.store_mul(709, 215, 756);

        s.store_scaled_mul(757, 716, 709, 2.0);

        s.store_exp_scaled_input(758, 220, s.v[360]);

        s.store_mul(720, 219, 758);

        s.store_mul(721, 258, 758);

        s.store_mul_ad_rhs(712, 230, A::exp_scaled_input(s.ad_value(231), (-s.v[360])));

        s.store_scale(719, 276, (4.0 * (1.3806505e-23 * s.v[356])));

        s.b[1155] = ((p.p46 != 0.0) && (s.v[287] > 0.0));
        s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });

        if s.b[1155] {
            s.store_offset_add_scaled_inputs_indices(713, 282, 1.0, 283, s.v[358], s.v[23]);
            s.store_exp_scaled_input(759, 288, s.v[360]);
            s.store_mul(714, 287, 759);
            s.store_scaled_mul(717, 714, 769, s.v[22]);
            s.store_offset_scaled(723, 286, ((s.v[359]) * (s.v[715])), s.v[715]);
            s.store_add_scaled_product_mixed_aia(760, A::offset(s.ad_value(284), s.v[362]), 1.0, 723, A::ln_scaled_input(A::mul(s.ad_value(285), A::powf(s.ad_value(363), (-0.75))), 4e-26), 2.0);
        }

        if s.b[1155] {
            if (s.v[760] > 0.05) {
            } else {
                s.store_scalar(760, 0.05);
            }
        }

        if s.b[1155] {
            s.store_div_ad_lhs(761, A::sqrt_scaled_input(s.ad_value(285), (((2.0 * 1.6021918e-19) * var_epssi) * s.v[361])), 769);
            s.store_square(724, 761);
            s.store_ln(725, 724);
            s.store_scale(762, 760, 0.95);
            s.store_scaled_mul(763, 760, 760, 0.0025);
            s.copy_ad(764, 763);
            s.store_scaled_sqrt(765, 764, 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(766, 762, 0.5, 765, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(762), s.ad_value(765))), s.ad_value(763)), (-0.5));
        }

        if (!s.b[1155]) {
            s.store_scalar(713, 0.0);
            s.store_scalar(759, 1.0);
            s.store_scalar(714, 0.0);
            s.store_scalar(717, 0.0);
            s.store_scalar(723, s.v[715]);
            s.store_scalar(760, 0.0);
            s.store_scalar(761, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 0.0);
            s.store_scalar(762, 0.0);
            s.store_scalar(763, 0.0);
            s.store_scalar(764, 0.0);
            s.store_scalar(765, 0.0);
            s.store_scalar(766, 0.0);
        }

        s.store_div_from_scalar(795, 1.0, 246);

        s.store_scaled_sqrt_scaled_input(796, 246, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(797, 796, 181);

        s.store_scale(798, 796, var_toxov_i);

        s.store_scale(799, 796, var_toxovd_i);

        s.store_scalar(800, 0.0);

        s.b[1156] = (s.v[241] < 0.0);
        s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });

        if s.b[1156] {
            s.store_div_scaled_inputs_indices(800, 240, (-0.495), 241, 1.0);
        }

        s.store_scalar(801, 0.0);

        s.b[1157] = (s.v[243] < 0.0);
        s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });

        if s.b[1157] {
            s.store_div_scaled_inputs_indices(801, 242, (-0.495), 243, 1.0);
        }

        s.b[1158] = (s.v[245] < 0.0);
        s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });

        if s.b[1158] {
            s.store_div_scaled_inputs_indices(802, 244, (-0.495), 245, 1.0);
        }

        let assign12870_e11516: f64 = (var_rta).powf(var_stig_i);
        var_tf_ig = assign12870_e11516;

        s.store_scale(236, 236, var_tf_ig);

        let assign12890_e11522: f64 = (var_igov_i * var_tf_ig);
        var_igov_i = assign12890_e11522;

        let assign12900_e11525: f64 = (var_igovd_i * var_tf_ig);
        var_igovd_i = assign12900_e11525;

        s.store_scalar(804, ((var_agidl_i * 4e-18) / (var_toxov_i * var_toxov_i)));

        s.store_scalar(805, ((var_agidld_i * 4e-18) / (var_toxovd_i * var_toxovd_i)));

        if ((1.0 + (s.v[251] * s.v[353])) > 0.0) {
            s.store_offset_scaled(796, 251, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }

        s.store_mul(710, 249, 796);

        s.store_scale(806, 710, (var_toxov_i * 500000000.0));

        if ((1.0 + (s.v[252] * s.v[353])) > 0.0) {
            s.store_offset_scaled(796, 252, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }

        s.store_mul(711, 250, 796);

        s.store_scale(807, 711, (var_toxovd_i * 500000000.0));

        s.store_scalar(808, 0.0);

        s.b[1159] = (s.v[272] > 1e-10);
        s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });

        if s.b[1159] {
            s.store_div_from_scalar(808, 0.75, 272);
        }

        s.store_square(809, 273);

        s.store_scale(810, 277, (9.1093826e-31 * 1000000000.0));

        let assign13040_e11604: f64 = if var_rg_i > 0.0 { 1.0 } else { 0.0 };
        var_guard171 = assign13040_e11604;

        let (assign13050_e11610,) = {
    if (var_guard171 != 0.0) {
        let assign13050_e11608: f64 = (1.0 / var_rg_i);
        (assign13050_e11608,)
    } else {
        (var_ggate,)
    }
};
        var_ggate = assign13050_e11610;

        let (assign13060_e11615,) = {
    if (var_guard171 == 0.0) {
        (0.0,)
    } else {
        (var_ggate,)
    }
};
        var_ggate = assign13060_e11615;

        let assign13070_e11618: f64 = if var_rse_i > 0.0 { 1.0 } else { 0.0 };
        var_guard172 = assign13070_e11618;

        let (assign13080_e11624,) = {
    if (var_guard172 != 0.0) {
        let assign13080_e11622: f64 = (1.0 / var_rse_i);
        (assign13080_e11622,)
    } else {
        (var_gsource,)
    }
};
        var_gsource = assign13080_e11624;

        let (assign13090_e11629,) = {
    if (var_guard172 == 0.0) {
        (0.0,)
    } else {
        (var_gsource,)
    }
};
        var_gsource = assign13090_e11629;

        let assign13100_e11632: f64 = if var_rde_i > 0.0 { 1.0 } else { 0.0 };
        var_guard173 = assign13100_e11632;

        let (assign13110_e11638,) = {
    if (var_guard173 != 0.0) {
        let assign13110_e11636: f64 = (1.0 / var_rde_i);
        (assign13110_e11636,)
    } else {
        (var_gdrain,)
    }
};
        var_gdrain = assign13110_e11638;

        let (assign13120_e11643,) = {
    if (var_guard173 == 0.0) {
        (0.0,)
    } else {
        (var_gdrain,)
    }
};
        var_gdrain = assign13120_e11643;

        let assign13130_e11646: f64 = if var_rbulk_i > 0.0 { 1.0 } else { 0.0 };
        var_guard174 = assign13130_e11646;

        let (assign13140_e11652,) = {
    if (var_guard174 != 0.0) {
        let assign13140_e11650: f64 = (1.0 / var_rbulk_i);
        (assign13140_e11650,)
    } else {
        (var_gbulk,)
    }
};
        var_gbulk = assign13140_e11652;

        let (assign13150_e11657,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_gbulk,)
    }
};
        var_gbulk = assign13150_e11657;

        let assign13160_e11660: f64 = if var_rjuns_i > 0.0 { 1.0 } else { 0.0 };
        var_guard175 = assign13160_e11660;

        let (assign13170_e11666,) = {
    if (var_guard175 != 0.0) {
        let assign13170_e11664: f64 = (1.0 / var_rjuns_i);
        (assign13170_e11664,)
    } else {
        (var_gjuns,)
    }
};
        var_gjuns = assign13170_e11666;

        let (assign13180_e11671,) = {
    if (var_guard175 == 0.0) {
        (0.0,)
    } else {
        (var_gjuns,)
    }
};
        var_gjuns = assign13180_e11671;

        let assign13190_e11674: f64 = if var_rjund_i > 0.0 { 1.0 } else { 0.0 };
        var_guard176 = assign13190_e11674;

        let (assign13200_e11680,) = {
    if (var_guard176 != 0.0) {
        let assign13200_e11678: f64 = (1.0 / var_rjund_i);
        (assign13200_e11678,)
    } else {
        (var_gjund,)
    }
};
        var_gjund = assign13200_e11680;

        let (assign13210_e11685,) = {
    if (var_guard176 == 0.0) {
        (0.0,)
    } else {
        (var_gjund,)
    }
};
        var_gjund = assign13210_e11685;

        let assign13220_e11688: f64 = if var_rwell_i > 0.0 { 1.0 } else { 0.0 };
        var_guard177 = assign13220_e11688;

        let (assign13230_e11694,) = {
    if (var_guard177 != 0.0) {
        let assign13230_e11692: f64 = (1.0 / var_rwell_i);
        (assign13230_e11692,)
    } else {
        (var_gwell,)
    }
};
        var_gwell = assign13230_e11694;

        *var_gbulk_slot = var_gbulk;
        *var_gdrain_slot = var_gdrain;
        *var_ggate_slot = var_ggate;
        *var_gjund_slot = var_gjund;
        *var_gjuns_slot = var_gjuns;
        *var_gsource_slot = var_gsource;
        *var_guard171_slot = var_guard171;
        *var_guard172_slot = var_guard172;
        *var_guard173_slot = var_guard173;
        *var_guard174_slot = var_guard174;
        *var_guard175_slot = var_guard175;
        *var_guard176_slot = var_guard176;
        *var_guard177_slot = var_guard177;
        *var_gwell_slot = var_gwell;
        *var_igov_i_slot = var_igov_i;
        *var_igovd_i_slot = var_igovd_i;
        *var_tf_ig_slot = var_tf_ig;
    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        var_ad_i: f64,
        var_as_i: f64,
        var_guard177: f64,
        var_invnf: f64,
        var_jw_i: f64,
        var_pd_i: f64,
        var_ps_i: f64,
        var_we: f64,
        var_abd_i_slot: &mut f64,
        var_abdrain_i_slot: &mut f64,
        var_abs_i_slot: &mut f64,
        var_absource_i_slot: &mut f64,
        var_exp_vmax_over_phitd_d_slot: &mut f64,
        var_exp_vmax_over_phitd_s_slot: &mut f64,
        var_guard178_slot: &mut f64,
        var_guard179_slot: &mut f64,
        var_guard180_slot: &mut f64,
        var_guard181_slot: &mut f64,
        var_gwell_slot: &mut f64,
        var_isatfor1_d_slot: &mut f64,
        var_isatfor1_s_slot: &mut f64,
        var_isatfor2_d_slot: &mut f64,
        var_isatfor2_d_db0_slot: &mut f64,
        var_isatfor2_d_db1_slot: &mut f64,
        var_isatfor2_d_db10_slot: &mut f64,
        var_isatfor2_d_db11_slot: &mut f64,
        var_isatfor2_d_db12_slot: &mut f64,
        var_isatfor2_d_db13_slot: &mut f64,
        var_isatfor2_d_db14_slot: &mut f64,
        var_isatfor2_d_db15_slot: &mut f64,
        var_isatfor2_d_db16_slot: &mut f64,
        var_isatfor2_d_db17_slot: &mut f64,
        var_isatfor2_d_db18_slot: &mut f64,
        var_isatfor2_d_db19_slot: &mut f64,
        var_isatfor2_d_db2_slot: &mut f64,
        var_isatfor2_d_db20_slot: &mut f64,
        var_isatfor2_d_db21_slot: &mut f64,
        var_isatfor2_d_db22_slot: &mut f64,
        var_isatfor2_d_db23_slot: &mut f64,
        var_isatfor2_d_db24_slot: &mut f64,
        var_isatfor2_d_db3_slot: &mut f64,
        var_isatfor2_d_db4_slot: &mut f64,
        var_isatfor2_d_db5_slot: &mut f64,
        var_isatfor2_d_db6_slot: &mut f64,
        var_isatfor2_d_db7_slot: &mut f64,
        var_isatfor2_d_db8_slot: &mut f64,
        var_isatfor2_d_db9_slot: &mut f64,
        var_isatfor2_d_dn0_slot: &mut f64,
        var_isatfor2_d_dn1_slot: &mut f64,
        var_isatfor2_d_dn10_slot: &mut f64,
        var_isatfor2_d_dn11_slot: &mut f64,
        var_isatfor2_d_dn12_slot: &mut f64,
        var_isatfor2_d_dn13_slot: &mut f64,
        var_isatfor2_d_dn14_slot: &mut f64,
        var_isatfor2_d_dn15_slot: &mut f64,
        var_isatfor2_d_dn16_slot: &mut f64,
        var_isatfor2_d_dn17_slot: &mut f64,
        var_isatfor2_d_dn18_slot: &mut f64,
        var_isatfor2_d_dn19_slot: &mut f64,
        var_isatfor2_d_dn2_slot: &mut f64,
        var_isatfor2_d_dn20_slot: &mut f64,
        var_isatfor2_d_dn3_slot: &mut f64,
        var_isatfor2_d_dn4_slot: &mut f64,
        var_isatfor2_d_dn5_slot: &mut f64,
        var_isatfor2_d_dn6_slot: &mut f64,
        var_isatfor2_d_dn7_slot: &mut f64,
        var_isatfor2_d_dn8_slot: &mut f64,
        var_isatfor2_d_dn9_slot: &mut f64,
        var_isatfor2_s_slot: &mut f64,
        var_isatfor2_s_db0_slot: &mut f64,
        var_isatfor2_s_db1_slot: &mut f64,
        var_isatfor2_s_db10_slot: &mut f64,
        var_isatfor2_s_db11_slot: &mut f64,
        var_isatfor2_s_db12_slot: &mut f64,
        var_isatfor2_s_db13_slot: &mut f64,
        var_isatfor2_s_db14_slot: &mut f64,
        var_isatfor2_s_db15_slot: &mut f64,
        var_isatfor2_s_db16_slot: &mut f64,
        var_isatfor2_s_db17_slot: &mut f64,
        var_isatfor2_s_db18_slot: &mut f64,
        var_isatfor2_s_db19_slot: &mut f64,
        var_isatfor2_s_db2_slot: &mut f64,
        var_isatfor2_s_db20_slot: &mut f64,
        var_isatfor2_s_db21_slot: &mut f64,
        var_isatfor2_s_db22_slot: &mut f64,
        var_isatfor2_s_db23_slot: &mut f64,
        var_isatfor2_s_db24_slot: &mut f64,
        var_isatfor2_s_db3_slot: &mut f64,
        var_isatfor2_s_db4_slot: &mut f64,
        var_isatfor2_s_db5_slot: &mut f64,
        var_isatfor2_s_db6_slot: &mut f64,
        var_isatfor2_s_db7_slot: &mut f64,
        var_isatfor2_s_db8_slot: &mut f64,
        var_isatfor2_s_db9_slot: &mut f64,
        var_isatfor2_s_dn0_slot: &mut f64,
        var_isatfor2_s_dn1_slot: &mut f64,
        var_isatfor2_s_dn10_slot: &mut f64,
        var_isatfor2_s_dn11_slot: &mut f64,
        var_isatfor2_s_dn12_slot: &mut f64,
        var_isatfor2_s_dn13_slot: &mut f64,
        var_isatfor2_s_dn14_slot: &mut f64,
        var_isatfor2_s_dn15_slot: &mut f64,
        var_isatfor2_s_dn16_slot: &mut f64,
        var_isatfor2_s_dn17_slot: &mut f64,
        var_isatfor2_s_dn18_slot: &mut f64,
        var_isatfor2_s_dn19_slot: &mut f64,
        var_isatfor2_s_dn2_slot: &mut f64,
        var_isatfor2_s_dn20_slot: &mut f64,
        var_isatfor2_s_dn3_slot: &mut f64,
        var_isatfor2_s_dn4_slot: &mut f64,
        var_isatfor2_s_dn5_slot: &mut f64,
        var_isatfor2_s_dn6_slot: &mut f64,
        var_isatfor2_s_dn7_slot: &mut f64,
        var_isatfor2_s_dn8_slot: &mut f64,
        var_isatfor2_s_dn9_slot: &mut f64,
        var_jwcorr_slot: &mut f64,
        var_jww_slot: &mut f64,
        var_lgd_i_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgs_i_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lsd_i_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lss_i_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_mfor1_d_slot: &mut f64,
        var_mfor1_s_slot: &mut f64,
        var_mfor2_d_slot: &mut f64,
        var_mfor2_d_db0_slot: &mut f64,
        var_mfor2_d_db1_slot: &mut f64,
        var_mfor2_d_db10_slot: &mut f64,
        var_mfor2_d_db11_slot: &mut f64,
        var_mfor2_d_db12_slot: &mut f64,
        var_mfor2_d_db13_slot: &mut f64,
        var_mfor2_d_db14_slot: &mut f64,
        var_mfor2_d_db15_slot: &mut f64,
        var_mfor2_d_db16_slot: &mut f64,
        var_mfor2_d_db17_slot: &mut f64,
        var_mfor2_d_db18_slot: &mut f64,
        var_mfor2_d_db19_slot: &mut f64,
        var_mfor2_d_db2_slot: &mut f64,
        var_mfor2_d_db20_slot: &mut f64,
        var_mfor2_d_db21_slot: &mut f64,
        var_mfor2_d_db22_slot: &mut f64,
        var_mfor2_d_db23_slot: &mut f64,
        var_mfor2_d_db24_slot: &mut f64,
        var_mfor2_d_db3_slot: &mut f64,
        var_mfor2_d_db4_slot: &mut f64,
        var_mfor2_d_db5_slot: &mut f64,
        var_mfor2_d_db6_slot: &mut f64,
        var_mfor2_d_db7_slot: &mut f64,
        var_mfor2_d_db8_slot: &mut f64,
        var_mfor2_d_db9_slot: &mut f64,
        var_mfor2_d_dn0_slot: &mut f64,
        var_mfor2_d_dn1_slot: &mut f64,
        var_mfor2_d_dn10_slot: &mut f64,
        var_mfor2_d_dn11_slot: &mut f64,
        var_mfor2_d_dn12_slot: &mut f64,
        var_mfor2_d_dn13_slot: &mut f64,
        var_mfor2_d_dn14_slot: &mut f64,
        var_mfor2_d_dn15_slot: &mut f64,
        var_mfor2_d_dn16_slot: &mut f64,
        var_mfor2_d_dn17_slot: &mut f64,
        var_mfor2_d_dn18_slot: &mut f64,
        var_mfor2_d_dn19_slot: &mut f64,
        var_mfor2_d_dn2_slot: &mut f64,
        var_mfor2_d_dn20_slot: &mut f64,
        var_mfor2_d_dn3_slot: &mut f64,
        var_mfor2_d_dn4_slot: &mut f64,
        var_mfor2_d_dn5_slot: &mut f64,
        var_mfor2_d_dn6_slot: &mut f64,
        var_mfor2_d_dn7_slot: &mut f64,
        var_mfor2_d_dn8_slot: &mut f64,
        var_mfor2_d_dn9_slot: &mut f64,
        var_mfor2_s_slot: &mut f64,
        var_mfor2_s_db0_slot: &mut f64,
        var_mfor2_s_db1_slot: &mut f64,
        var_mfor2_s_db10_slot: &mut f64,
        var_mfor2_s_db11_slot: &mut f64,
        var_mfor2_s_db12_slot: &mut f64,
        var_mfor2_s_db13_slot: &mut f64,
        var_mfor2_s_db14_slot: &mut f64,
        var_mfor2_s_db15_slot: &mut f64,
        var_mfor2_s_db16_slot: &mut f64,
        var_mfor2_s_db17_slot: &mut f64,
        var_mfor2_s_db18_slot: &mut f64,
        var_mfor2_s_db19_slot: &mut f64,
        var_mfor2_s_db2_slot: &mut f64,
        var_mfor2_s_db20_slot: &mut f64,
        var_mfor2_s_db21_slot: &mut f64,
        var_mfor2_s_db22_slot: &mut f64,
        var_mfor2_s_db23_slot: &mut f64,
        var_mfor2_s_db24_slot: &mut f64,
        var_mfor2_s_db3_slot: &mut f64,
        var_mfor2_s_db4_slot: &mut f64,
        var_mfor2_s_db5_slot: &mut f64,
        var_mfor2_s_db6_slot: &mut f64,
        var_mfor2_s_db7_slot: &mut f64,
        var_mfor2_s_db8_slot: &mut f64,
        var_mfor2_s_db9_slot: &mut f64,
        var_mfor2_s_dn0_slot: &mut f64,
        var_mfor2_s_dn1_slot: &mut f64,
        var_mfor2_s_dn10_slot: &mut f64,
        var_mfor2_s_dn11_slot: &mut f64,
        var_mfor2_s_dn12_slot: &mut f64,
        var_mfor2_s_dn13_slot: &mut f64,
        var_mfor2_s_dn14_slot: &mut f64,
        var_mfor2_s_dn15_slot: &mut f64,
        var_mfor2_s_dn16_slot: &mut f64,
        var_mfor2_s_dn17_slot: &mut f64,
        var_mfor2_s_dn18_slot: &mut f64,
        var_mfor2_s_dn19_slot: &mut f64,
        var_mfor2_s_dn2_slot: &mut f64,
        var_mfor2_s_dn20_slot: &mut f64,
        var_mfor2_s_dn3_slot: &mut f64,
        var_mfor2_s_dn4_slot: &mut f64,
        var_mfor2_s_dn5_slot: &mut f64,
        var_mfor2_s_dn6_slot: &mut f64,
        var_mfor2_s_dn7_slot: &mut f64,
        var_mfor2_s_dn8_slot: &mut f64,
        var_mfor2_s_dn9_slot: &mut f64,
        var_vbbtlim_d_slot: &mut f64,
        var_vbbtlim_s_slot: &mut f64,
        var_vbimin_d_slot: &mut f64,
        var_vbimin_s_slot: &mut f64,
        var_vch_d_slot: &mut f64,
        var_vch_s_slot: &mut f64,
        var_vfmin_d_slot: &mut f64,
        var_vfmin_s_slot: &mut f64,
        var_vmax_d_slot: &mut f64,
        var_vmax_s_slot: &mut f64,
    ) {
        let mut var_abd_i: f64 = *var_abd_i_slot;
        let mut var_abdrain_i: f64 = *var_abdrain_i_slot;
        let mut var_abs_i: f64 = *var_abs_i_slot;
        let mut var_absource_i: f64 = *var_absource_i_slot;
        let mut var_exp_vmax_over_phitd_d: f64 = *var_exp_vmax_over_phitd_d_slot;
        let mut var_exp_vmax_over_phitd_s: f64 = *var_exp_vmax_over_phitd_s_slot;
        let mut var_guard178: f64 = *var_guard178_slot;
        let mut var_guard179: f64 = *var_guard179_slot;
        let mut var_guard180: f64 = *var_guard180_slot;
        let mut var_guard181: f64 = *var_guard181_slot;
        let mut var_gwell: f64 = *var_gwell_slot;
        let mut var_isatfor1_d: f64 = *var_isatfor1_d_slot;
        let mut var_isatfor1_s: f64 = *var_isatfor1_s_slot;
        let mut var_isatfor2_d: f64 = *var_isatfor2_d_slot;
        let mut var_isatfor2_d_db0: f64 = *var_isatfor2_d_db0_slot;
        let mut var_isatfor2_d_db1: f64 = *var_isatfor2_d_db1_slot;
        let mut var_isatfor2_d_db10: f64 = *var_isatfor2_d_db10_slot;
        let mut var_isatfor2_d_db11: f64 = *var_isatfor2_d_db11_slot;
        let mut var_isatfor2_d_db12: f64 = *var_isatfor2_d_db12_slot;
        let mut var_isatfor2_d_db13: f64 = *var_isatfor2_d_db13_slot;
        let mut var_isatfor2_d_db14: f64 = *var_isatfor2_d_db14_slot;
        let mut var_isatfor2_d_db15: f64 = *var_isatfor2_d_db15_slot;
        let mut var_isatfor2_d_db16: f64 = *var_isatfor2_d_db16_slot;
        let mut var_isatfor2_d_db17: f64 = *var_isatfor2_d_db17_slot;
        let mut var_isatfor2_d_db18: f64 = *var_isatfor2_d_db18_slot;
        let mut var_isatfor2_d_db19: f64 = *var_isatfor2_d_db19_slot;
        let mut var_isatfor2_d_db2: f64 = *var_isatfor2_d_db2_slot;
        let mut var_isatfor2_d_db20: f64 = *var_isatfor2_d_db20_slot;
        let mut var_isatfor2_d_db21: f64 = *var_isatfor2_d_db21_slot;
        let mut var_isatfor2_d_db22: f64 = *var_isatfor2_d_db22_slot;
        let mut var_isatfor2_d_db23: f64 = *var_isatfor2_d_db23_slot;
        let mut var_isatfor2_d_db24: f64 = *var_isatfor2_d_db24_slot;
        let mut var_isatfor2_d_db3: f64 = *var_isatfor2_d_db3_slot;
        let mut var_isatfor2_d_db4: f64 = *var_isatfor2_d_db4_slot;
        let mut var_isatfor2_d_db5: f64 = *var_isatfor2_d_db5_slot;
        let mut var_isatfor2_d_db6: f64 = *var_isatfor2_d_db6_slot;
        let mut var_isatfor2_d_db7: f64 = *var_isatfor2_d_db7_slot;
        let mut var_isatfor2_d_db8: f64 = *var_isatfor2_d_db8_slot;
        let mut var_isatfor2_d_db9: f64 = *var_isatfor2_d_db9_slot;
        let mut var_isatfor2_d_dn0: f64 = *var_isatfor2_d_dn0_slot;
        let mut var_isatfor2_d_dn1: f64 = *var_isatfor2_d_dn1_slot;
        let mut var_isatfor2_d_dn10: f64 = *var_isatfor2_d_dn10_slot;
        let mut var_isatfor2_d_dn11: f64 = *var_isatfor2_d_dn11_slot;
        let mut var_isatfor2_d_dn12: f64 = *var_isatfor2_d_dn12_slot;
        let mut var_isatfor2_d_dn13: f64 = *var_isatfor2_d_dn13_slot;
        let mut var_isatfor2_d_dn14: f64 = *var_isatfor2_d_dn14_slot;
        let mut var_isatfor2_d_dn15: f64 = *var_isatfor2_d_dn15_slot;
        let mut var_isatfor2_d_dn16: f64 = *var_isatfor2_d_dn16_slot;
        let mut var_isatfor2_d_dn17: f64 = *var_isatfor2_d_dn17_slot;
        let mut var_isatfor2_d_dn18: f64 = *var_isatfor2_d_dn18_slot;
        let mut var_isatfor2_d_dn19: f64 = *var_isatfor2_d_dn19_slot;
        let mut var_isatfor2_d_dn2: f64 = *var_isatfor2_d_dn2_slot;
        let mut var_isatfor2_d_dn20: f64 = *var_isatfor2_d_dn20_slot;
        let mut var_isatfor2_d_dn3: f64 = *var_isatfor2_d_dn3_slot;
        let mut var_isatfor2_d_dn4: f64 = *var_isatfor2_d_dn4_slot;
        let mut var_isatfor2_d_dn5: f64 = *var_isatfor2_d_dn5_slot;
        let mut var_isatfor2_d_dn6: f64 = *var_isatfor2_d_dn6_slot;
        let mut var_isatfor2_d_dn7: f64 = *var_isatfor2_d_dn7_slot;
        let mut var_isatfor2_d_dn8: f64 = *var_isatfor2_d_dn8_slot;
        let mut var_isatfor2_d_dn9: f64 = *var_isatfor2_d_dn9_slot;
        let mut var_isatfor2_s: f64 = *var_isatfor2_s_slot;
        let mut var_isatfor2_s_db0: f64 = *var_isatfor2_s_db0_slot;
        let mut var_isatfor2_s_db1: f64 = *var_isatfor2_s_db1_slot;
        let mut var_isatfor2_s_db10: f64 = *var_isatfor2_s_db10_slot;
        let mut var_isatfor2_s_db11: f64 = *var_isatfor2_s_db11_slot;
        let mut var_isatfor2_s_db12: f64 = *var_isatfor2_s_db12_slot;
        let mut var_isatfor2_s_db13: f64 = *var_isatfor2_s_db13_slot;
        let mut var_isatfor2_s_db14: f64 = *var_isatfor2_s_db14_slot;
        let mut var_isatfor2_s_db15: f64 = *var_isatfor2_s_db15_slot;
        let mut var_isatfor2_s_db16: f64 = *var_isatfor2_s_db16_slot;
        let mut var_isatfor2_s_db17: f64 = *var_isatfor2_s_db17_slot;
        let mut var_isatfor2_s_db18: f64 = *var_isatfor2_s_db18_slot;
        let mut var_isatfor2_s_db19: f64 = *var_isatfor2_s_db19_slot;
        let mut var_isatfor2_s_db2: f64 = *var_isatfor2_s_db2_slot;
        let mut var_isatfor2_s_db20: f64 = *var_isatfor2_s_db20_slot;
        let mut var_isatfor2_s_db21: f64 = *var_isatfor2_s_db21_slot;
        let mut var_isatfor2_s_db22: f64 = *var_isatfor2_s_db22_slot;
        let mut var_isatfor2_s_db23: f64 = *var_isatfor2_s_db23_slot;
        let mut var_isatfor2_s_db24: f64 = *var_isatfor2_s_db24_slot;
        let mut var_isatfor2_s_db3: f64 = *var_isatfor2_s_db3_slot;
        let mut var_isatfor2_s_db4: f64 = *var_isatfor2_s_db4_slot;
        let mut var_isatfor2_s_db5: f64 = *var_isatfor2_s_db5_slot;
        let mut var_isatfor2_s_db6: f64 = *var_isatfor2_s_db6_slot;
        let mut var_isatfor2_s_db7: f64 = *var_isatfor2_s_db7_slot;
        let mut var_isatfor2_s_db8: f64 = *var_isatfor2_s_db8_slot;
        let mut var_isatfor2_s_db9: f64 = *var_isatfor2_s_db9_slot;
        let mut var_isatfor2_s_dn0: f64 = *var_isatfor2_s_dn0_slot;
        let mut var_isatfor2_s_dn1: f64 = *var_isatfor2_s_dn1_slot;
        let mut var_isatfor2_s_dn10: f64 = *var_isatfor2_s_dn10_slot;
        let mut var_isatfor2_s_dn11: f64 = *var_isatfor2_s_dn11_slot;
        let mut var_isatfor2_s_dn12: f64 = *var_isatfor2_s_dn12_slot;
        let mut var_isatfor2_s_dn13: f64 = *var_isatfor2_s_dn13_slot;
        let mut var_isatfor2_s_dn14: f64 = *var_isatfor2_s_dn14_slot;
        let mut var_isatfor2_s_dn15: f64 = *var_isatfor2_s_dn15_slot;
        let mut var_isatfor2_s_dn16: f64 = *var_isatfor2_s_dn16_slot;
        let mut var_isatfor2_s_dn17: f64 = *var_isatfor2_s_dn17_slot;
        let mut var_isatfor2_s_dn18: f64 = *var_isatfor2_s_dn18_slot;
        let mut var_isatfor2_s_dn19: f64 = *var_isatfor2_s_dn19_slot;
        let mut var_isatfor2_s_dn2: f64 = *var_isatfor2_s_dn2_slot;
        let mut var_isatfor2_s_dn20: f64 = *var_isatfor2_s_dn20_slot;
        let mut var_isatfor2_s_dn3: f64 = *var_isatfor2_s_dn3_slot;
        let mut var_isatfor2_s_dn4: f64 = *var_isatfor2_s_dn4_slot;
        let mut var_isatfor2_s_dn5: f64 = *var_isatfor2_s_dn5_slot;
        let mut var_isatfor2_s_dn6: f64 = *var_isatfor2_s_dn6_slot;
        let mut var_isatfor2_s_dn7: f64 = *var_isatfor2_s_dn7_slot;
        let mut var_isatfor2_s_dn8: f64 = *var_isatfor2_s_dn8_slot;
        let mut var_isatfor2_s_dn9: f64 = *var_isatfor2_s_dn9_slot;
        let mut var_jwcorr: f64 = *var_jwcorr_slot;
        let mut var_jww: f64 = *var_jww_slot;
        let mut var_lgd_i: f64 = *var_lgd_i_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgs_i: f64 = *var_lgs_i_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lsd_i: f64 = *var_lsd_i_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lss_i: f64 = *var_lss_i_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_mfor1_d: f64 = *var_mfor1_d_slot;
        let mut var_mfor1_s: f64 = *var_mfor1_s_slot;
        let mut var_mfor2_d: f64 = *var_mfor2_d_slot;
        let mut var_mfor2_d_db0: f64 = *var_mfor2_d_db0_slot;
        let mut var_mfor2_d_db1: f64 = *var_mfor2_d_db1_slot;
        let mut var_mfor2_d_db10: f64 = *var_mfor2_d_db10_slot;
        let mut var_mfor2_d_db11: f64 = *var_mfor2_d_db11_slot;
        let mut var_mfor2_d_db12: f64 = *var_mfor2_d_db12_slot;
        let mut var_mfor2_d_db13: f64 = *var_mfor2_d_db13_slot;
        let mut var_mfor2_d_db14: f64 = *var_mfor2_d_db14_slot;
        let mut var_mfor2_d_db15: f64 = *var_mfor2_d_db15_slot;
        let mut var_mfor2_d_db16: f64 = *var_mfor2_d_db16_slot;
        let mut var_mfor2_d_db17: f64 = *var_mfor2_d_db17_slot;
        let mut var_mfor2_d_db18: f64 = *var_mfor2_d_db18_slot;
        let mut var_mfor2_d_db19: f64 = *var_mfor2_d_db19_slot;
        let mut var_mfor2_d_db2: f64 = *var_mfor2_d_db2_slot;
        let mut var_mfor2_d_db20: f64 = *var_mfor2_d_db20_slot;
        let mut var_mfor2_d_db21: f64 = *var_mfor2_d_db21_slot;
        let mut var_mfor2_d_db22: f64 = *var_mfor2_d_db22_slot;
        let mut var_mfor2_d_db23: f64 = *var_mfor2_d_db23_slot;
        let mut var_mfor2_d_db24: f64 = *var_mfor2_d_db24_slot;
        let mut var_mfor2_d_db3: f64 = *var_mfor2_d_db3_slot;
        let mut var_mfor2_d_db4: f64 = *var_mfor2_d_db4_slot;
        let mut var_mfor2_d_db5: f64 = *var_mfor2_d_db5_slot;
        let mut var_mfor2_d_db6: f64 = *var_mfor2_d_db6_slot;
        let mut var_mfor2_d_db7: f64 = *var_mfor2_d_db7_slot;
        let mut var_mfor2_d_db8: f64 = *var_mfor2_d_db8_slot;
        let mut var_mfor2_d_db9: f64 = *var_mfor2_d_db9_slot;
        let mut var_mfor2_d_dn0: f64 = *var_mfor2_d_dn0_slot;
        let mut var_mfor2_d_dn1: f64 = *var_mfor2_d_dn1_slot;
        let mut var_mfor2_d_dn10: f64 = *var_mfor2_d_dn10_slot;
        let mut var_mfor2_d_dn11: f64 = *var_mfor2_d_dn11_slot;
        let mut var_mfor2_d_dn12: f64 = *var_mfor2_d_dn12_slot;
        let mut var_mfor2_d_dn13: f64 = *var_mfor2_d_dn13_slot;
        let mut var_mfor2_d_dn14: f64 = *var_mfor2_d_dn14_slot;
        let mut var_mfor2_d_dn15: f64 = *var_mfor2_d_dn15_slot;
        let mut var_mfor2_d_dn16: f64 = *var_mfor2_d_dn16_slot;
        let mut var_mfor2_d_dn17: f64 = *var_mfor2_d_dn17_slot;
        let mut var_mfor2_d_dn18: f64 = *var_mfor2_d_dn18_slot;
        let mut var_mfor2_d_dn19: f64 = *var_mfor2_d_dn19_slot;
        let mut var_mfor2_d_dn2: f64 = *var_mfor2_d_dn2_slot;
        let mut var_mfor2_d_dn20: f64 = *var_mfor2_d_dn20_slot;
        let mut var_mfor2_d_dn3: f64 = *var_mfor2_d_dn3_slot;
        let mut var_mfor2_d_dn4: f64 = *var_mfor2_d_dn4_slot;
        let mut var_mfor2_d_dn5: f64 = *var_mfor2_d_dn5_slot;
        let mut var_mfor2_d_dn6: f64 = *var_mfor2_d_dn6_slot;
        let mut var_mfor2_d_dn7: f64 = *var_mfor2_d_dn7_slot;
        let mut var_mfor2_d_dn8: f64 = *var_mfor2_d_dn8_slot;
        let mut var_mfor2_d_dn9: f64 = *var_mfor2_d_dn9_slot;
        let mut var_mfor2_s: f64 = *var_mfor2_s_slot;
        let mut var_mfor2_s_db0: f64 = *var_mfor2_s_db0_slot;
        let mut var_mfor2_s_db1: f64 = *var_mfor2_s_db1_slot;
        let mut var_mfor2_s_db10: f64 = *var_mfor2_s_db10_slot;
        let mut var_mfor2_s_db11: f64 = *var_mfor2_s_db11_slot;
        let mut var_mfor2_s_db12: f64 = *var_mfor2_s_db12_slot;
        let mut var_mfor2_s_db13: f64 = *var_mfor2_s_db13_slot;
        let mut var_mfor2_s_db14: f64 = *var_mfor2_s_db14_slot;
        let mut var_mfor2_s_db15: f64 = *var_mfor2_s_db15_slot;
        let mut var_mfor2_s_db16: f64 = *var_mfor2_s_db16_slot;
        let mut var_mfor2_s_db17: f64 = *var_mfor2_s_db17_slot;
        let mut var_mfor2_s_db18: f64 = *var_mfor2_s_db18_slot;
        let mut var_mfor2_s_db19: f64 = *var_mfor2_s_db19_slot;
        let mut var_mfor2_s_db2: f64 = *var_mfor2_s_db2_slot;
        let mut var_mfor2_s_db20: f64 = *var_mfor2_s_db20_slot;
        let mut var_mfor2_s_db21: f64 = *var_mfor2_s_db21_slot;
        let mut var_mfor2_s_db22: f64 = *var_mfor2_s_db22_slot;
        let mut var_mfor2_s_db23: f64 = *var_mfor2_s_db23_slot;
        let mut var_mfor2_s_db24: f64 = *var_mfor2_s_db24_slot;
        let mut var_mfor2_s_db3: f64 = *var_mfor2_s_db3_slot;
        let mut var_mfor2_s_db4: f64 = *var_mfor2_s_db4_slot;
        let mut var_mfor2_s_db5: f64 = *var_mfor2_s_db5_slot;
        let mut var_mfor2_s_db6: f64 = *var_mfor2_s_db6_slot;
        let mut var_mfor2_s_db7: f64 = *var_mfor2_s_db7_slot;
        let mut var_mfor2_s_db8: f64 = *var_mfor2_s_db8_slot;
        let mut var_mfor2_s_db9: f64 = *var_mfor2_s_db9_slot;
        let mut var_mfor2_s_dn0: f64 = *var_mfor2_s_dn0_slot;
        let mut var_mfor2_s_dn1: f64 = *var_mfor2_s_dn1_slot;
        let mut var_mfor2_s_dn10: f64 = *var_mfor2_s_dn10_slot;
        let mut var_mfor2_s_dn11: f64 = *var_mfor2_s_dn11_slot;
        let mut var_mfor2_s_dn12: f64 = *var_mfor2_s_dn12_slot;
        let mut var_mfor2_s_dn13: f64 = *var_mfor2_s_dn13_slot;
        let mut var_mfor2_s_dn14: f64 = *var_mfor2_s_dn14_slot;
        let mut var_mfor2_s_dn15: f64 = *var_mfor2_s_dn15_slot;
        let mut var_mfor2_s_dn16: f64 = *var_mfor2_s_dn16_slot;
        let mut var_mfor2_s_dn17: f64 = *var_mfor2_s_dn17_slot;
        let mut var_mfor2_s_dn18: f64 = *var_mfor2_s_dn18_slot;
        let mut var_mfor2_s_dn19: f64 = *var_mfor2_s_dn19_slot;
        let mut var_mfor2_s_dn2: f64 = *var_mfor2_s_dn2_slot;
        let mut var_mfor2_s_dn20: f64 = *var_mfor2_s_dn20_slot;
        let mut var_mfor2_s_dn3: f64 = *var_mfor2_s_dn3_slot;
        let mut var_mfor2_s_dn4: f64 = *var_mfor2_s_dn4_slot;
        let mut var_mfor2_s_dn5: f64 = *var_mfor2_s_dn5_slot;
        let mut var_mfor2_s_dn6: f64 = *var_mfor2_s_dn6_slot;
        let mut var_mfor2_s_dn7: f64 = *var_mfor2_s_dn7_slot;
        let mut var_mfor2_s_dn8: f64 = *var_mfor2_s_dn8_slot;
        let mut var_mfor2_s_dn9: f64 = *var_mfor2_s_dn9_slot;
        let mut var_vbbtlim_d: f64 = *var_vbbtlim_d_slot;
        let mut var_vbbtlim_s: f64 = *var_vbbtlim_s_slot;
        let mut var_vbimin_d: f64 = *var_vbimin_d_slot;
        let mut var_vbimin_s: f64 = *var_vbimin_s_slot;
        let mut var_vch_d: f64 = *var_vch_d_slot;
        let mut var_vch_s: f64 = *var_vch_s_slot;
        let mut var_vfmin_d: f64 = *var_vfmin_d_slot;
        let mut var_vfmin_s: f64 = *var_vfmin_s_slot;
        let mut var_vmax_d: f64 = *var_vmax_d_slot;
        let mut var_vmax_s: f64 = *var_vmax_s_slot;

        let (assign13240_e11699,) = {
    if (var_guard177 == 0.0) {
        (0.0,)
    } else {
        (var_gwell,)
    }
};
        var_gwell = assign13240_e11699;

        let assign13250_e11702: f64 = (var_absource_i * var_invnf);
        var_abs_i = assign13250_e11702;

        let assign13260_e11705: f64 = (var_lssource_i * var_invnf);
        var_lss_i = assign13260_e11705;

        let assign13270_e11708: f64 = (var_lgsource_i * var_invnf);
        var_lgs_i = assign13270_e11708;

        let assign13280_e11711: f64 = (var_abdrain_i * var_invnf);
        var_abd_i = assign13280_e11711;

        let assign13290_e11714: f64 = (var_lsdrain_i * var_invnf);
        var_lsd_i = assign13290_e11714;

        let assign13300_e11717: f64 = (var_lgdrain_i * var_invnf);
        var_lgd_i = assign13300_e11717;

        var_jwcorr = 0.0;

        let assign13320_e11721: f64 = if p.p43 == 3.0 { 1.0 } else { 0.0 };
        var_guard178 = assign13320_e11721;

        let (assign13330_e11725,) = {
    if (var_guard178 != 0.0) {
        (1.0,)
    } else {
        (var_jwcorr,)
    }
};
        var_jwcorr = assign13330_e11725;

        var_jww = var_we;

        let assign13350_e11729: f64 = if p.p39 == 0.0 { 1.0 } else { 0.0 };
        var_guard179 = assign13350_e11729;

        let (assign13360_e11738,) = {
    if (var_guard179 != 0.0) {
        let (assign13360_e11736,) = {
            if (var_jw_i > 0.0) {
                (var_jw_i,)
            } else {
                (0.0,)
            }
        };
        (assign13360_e11736,)
    } else {
        (var_jww,)
    }
};
        var_jww = assign13360_e11738;

        let assign13370_e11745: f64 = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        var_guard180 = assign13370_e11745;

        let (assign13380_e11751,) = {
    if (var_guard180 != 0.0) {
        let assign13380_e11749: f64 = (var_as_i * var_invnf);
        (assign13380_e11749,)
    } else {
        (var_abs_i,)
    }
};
        var_abs_i = assign13380_e11751;

        let (assign13390_e11761,) = {
    if (var_guard180 != 0.0) {
        let assign13390_e11755: f64 = (var_ps_i * var_invnf);
        let assign13390_e11758: f64 = (var_jwcorr * var_jww);
        let assign13390_e11759: f64 = (assign13390_e11755 - assign13390_e11758);
        (assign13390_e11759,)
    } else {
        (var_lss_i,)
    }
};
        var_lss_i = assign13390_e11761;

        let (assign13400_e11765,) = {
    if (var_guard180 != 0.0) {
        (var_jww,)
    } else {
        (var_lgs_i,)
    }
};
        var_lgs_i = assign13400_e11765;

        let (assign13410_e11771,) = {
    if (var_guard180 != 0.0) {
        let assign13410_e11769: f64 = (var_ad_i * var_invnf);
        (assign13410_e11769,)
    } else {
        (var_abd_i,)
    }
};
        var_abd_i = assign13410_e11771;

        let (assign13420_e11781,) = {
    if (var_guard180 != 0.0) {
        let assign13420_e11775: f64 = (var_pd_i * var_invnf);
        let assign13420_e11778: f64 = (var_jwcorr * var_jww);
        let assign13420_e11779: f64 = (assign13420_e11775 - assign13420_e11778);
        (assign13420_e11779,)
    } else {
        (var_lsd_i,)
    }
};
        var_lsd_i = assign13420_e11781;

        let (assign13430_e11785,) = {
    if (var_guard180 != 0.0) {
        (var_jww,)
    } else {
        (var_lgd_i,)
    }
};
        var_lgd_i = assign13430_e11785;

        let assign13440_e11796: f64 = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        var_guard181 = assign13440_e11796;

        let (assign13450_e11805,) = {
    if (var_guard181 != 0.0) {
        let (assign13450_e11803,) = {
            if (var_abs_i > 0.0) {
                (var_abs_i,)
            } else {
                (0.0,)
            }
        };
        (assign13450_e11803,)
    } else {
        (var_absource_i,)
    }
};
        var_absource_i = assign13450_e11805;

        let (assign13460_e11814,) = {
    if (var_guard181 != 0.0) {
        let (assign13460_e11812,) = {
            if (var_lss_i > 0.0) {
                (var_lss_i,)
            } else {
                (0.0,)
            }
        };
        (assign13460_e11812,)
    } else {
        (var_lssource_i,)
    }
};
        var_lssource_i = assign13460_e11814;

        let (assign13470_e11823,) = {
    if (var_guard181 != 0.0) {
        let (assign13470_e11821,) = {
            if (var_lgs_i > 0.0) {
                (var_lgs_i,)
            } else {
                (0.0,)
            }
        };
        (assign13470_e11821,)
    } else {
        (var_lgsource_i,)
    }
};
        var_lgsource_i = assign13470_e11823;

        let (assign13480_e11832,) = {
    if (var_guard181 != 0.0) {
        let (assign13480_e11830,) = {
            if (var_abd_i > 0.0) {
                (var_abd_i,)
            } else {
                (0.0,)
            }
        };
        (assign13480_e11830,)
    } else {
        (var_abdrain_i,)
    }
};
        var_abdrain_i = assign13480_e11832;

        let (assign13490_e11841,) = {
    if (var_guard181 != 0.0) {
        let (assign13490_e11839,) = {
            if (var_lsd_i > 0.0) {
                (var_lsd_i,)
            } else {
                (0.0,)
            }
        };
        (assign13490_e11839,)
    } else {
        (var_lsdrain_i,)
    }
};
        var_lsdrain_i = assign13490_e11841;

        let (assign13500_e11850,) = {
    if (var_guard181 != 0.0) {
        let (assign13500_e11848,) = {
            if (var_lgd_i > 0.0) {
                (var_lgd_i,)
            } else {
                (0.0,)
            }
        };
        (assign13500_e11848,)
    } else {
        (var_lgdrain_i,)
    }
};
        var_lgdrain_i = assign13500_e11850;

        let (assign13510_e11855,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_absource_i,)
    }
};
        var_absource_i = assign13510_e11855;

        let (assign13520_e11860,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_lssource_i,)
    }
};
        var_lssource_i = assign13520_e11860;

        let (assign13530_e11865,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_lgsource_i,)
    }
};
        var_lgsource_i = assign13530_e11865;

        let (assign13540_e11870,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_abdrain_i,)
    }
};
        var_abdrain_i = assign13540_e11870;

        let (assign13550_e11875,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_lsdrain_i,)
    }
};
        var_lsdrain_i = assign13550_e11875;

        let (assign13560_e11880,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_lgdrain_i,)
    }
};
        var_lgdrain_i = assign13560_e11880;

        var_vbimin_s = 0.0;

        var_vbimin_d = 0.0;

        var_vfmin_s = 0.0;

        var_vfmin_d = 0.0;

        var_vch_s = 0.0;

        var_vch_d = 0.0;

        var_vbbtlim_s = 0.0;

        var_vbbtlim_d = 0.0;

        var_vmax_s = 0.0;

        var_vmax_d = 0.0;

        var_exp_vmax_over_phitd_s = 0.0;

        var_exp_vmax_over_phitd_d = 0.0;

        var_isatfor1_s = 0.0;

        var_isatfor1_d = 0.0;

        var_mfor1_s = 1.0;

        var_mfor1_d = 1.0;

        var_isatfor2_s = 0.0;
        var_isatfor2_s_dn0 = 0.0;
        var_isatfor2_s_dn1 = 0.0;
        var_isatfor2_s_dn2 = 0.0;
        var_isatfor2_s_dn3 = 0.0;
        var_isatfor2_s_dn4 = 0.0;
        var_isatfor2_s_dn5 = 0.0;
        var_isatfor2_s_dn6 = 0.0;
        var_isatfor2_s_dn7 = 0.0;
        var_isatfor2_s_dn8 = 0.0;
        var_isatfor2_s_dn9 = 0.0;
        var_isatfor2_s_dn10 = 0.0;
        var_isatfor2_s_dn11 = 0.0;
        var_isatfor2_s_dn12 = 0.0;
        var_isatfor2_s_dn13 = 0.0;
        var_isatfor2_s_dn14 = 0.0;
        var_isatfor2_s_dn15 = 0.0;
        var_isatfor2_s_dn16 = 0.0;
        var_isatfor2_s_dn17 = 0.0;
        var_isatfor2_s_dn18 = 0.0;
        var_isatfor2_s_dn19 = 0.0;
        var_isatfor2_s_dn20 = 0.0;
        var_isatfor2_s_db0 = 0.0;
        var_isatfor2_s_db1 = 0.0;
        var_isatfor2_s_db2 = 0.0;
        var_isatfor2_s_db3 = 0.0;
        var_isatfor2_s_db4 = 0.0;
        var_isatfor2_s_db5 = 0.0;
        var_isatfor2_s_db6 = 0.0;
        var_isatfor2_s_db7 = 0.0;
        var_isatfor2_s_db8 = 0.0;
        var_isatfor2_s_db9 = 0.0;
        var_isatfor2_s_db10 = 0.0;
        var_isatfor2_s_db11 = 0.0;
        var_isatfor2_s_db12 = 0.0;
        var_isatfor2_s_db13 = 0.0;
        var_isatfor2_s_db14 = 0.0;
        var_isatfor2_s_db15 = 0.0;
        var_isatfor2_s_db16 = 0.0;
        var_isatfor2_s_db17 = 0.0;
        var_isatfor2_s_db18 = 0.0;
        var_isatfor2_s_db19 = 0.0;
        var_isatfor2_s_db20 = 0.0;
        var_isatfor2_s_db21 = 0.0;
        var_isatfor2_s_db22 = 0.0;
        var_isatfor2_s_db23 = 0.0;
        var_isatfor2_s_db24 = 0.0;

        var_isatfor2_d = 0.0;
        var_isatfor2_d_dn0 = 0.0;
        var_isatfor2_d_dn1 = 0.0;
        var_isatfor2_d_dn2 = 0.0;
        var_isatfor2_d_dn3 = 0.0;
        var_isatfor2_d_dn4 = 0.0;
        var_isatfor2_d_dn5 = 0.0;
        var_isatfor2_d_dn6 = 0.0;
        var_isatfor2_d_dn7 = 0.0;
        var_isatfor2_d_dn8 = 0.0;
        var_isatfor2_d_dn9 = 0.0;
        var_isatfor2_d_dn10 = 0.0;
        var_isatfor2_d_dn11 = 0.0;
        var_isatfor2_d_dn12 = 0.0;
        var_isatfor2_d_dn13 = 0.0;
        var_isatfor2_d_dn14 = 0.0;
        var_isatfor2_d_dn15 = 0.0;
        var_isatfor2_d_dn16 = 0.0;
        var_isatfor2_d_dn17 = 0.0;
        var_isatfor2_d_dn18 = 0.0;
        var_isatfor2_d_dn19 = 0.0;
        var_isatfor2_d_dn20 = 0.0;
        var_isatfor2_d_db0 = 0.0;
        var_isatfor2_d_db1 = 0.0;
        var_isatfor2_d_db2 = 0.0;
        var_isatfor2_d_db3 = 0.0;
        var_isatfor2_d_db4 = 0.0;
        var_isatfor2_d_db5 = 0.0;
        var_isatfor2_d_db6 = 0.0;
        var_isatfor2_d_db7 = 0.0;
        var_isatfor2_d_db8 = 0.0;
        var_isatfor2_d_db9 = 0.0;
        var_isatfor2_d_db10 = 0.0;
        var_isatfor2_d_db11 = 0.0;
        var_isatfor2_d_db12 = 0.0;
        var_isatfor2_d_db13 = 0.0;
        var_isatfor2_d_db14 = 0.0;
        var_isatfor2_d_db15 = 0.0;
        var_isatfor2_d_db16 = 0.0;
        var_isatfor2_d_db17 = 0.0;
        var_isatfor2_d_db18 = 0.0;
        var_isatfor2_d_db19 = 0.0;
        var_isatfor2_d_db20 = 0.0;
        var_isatfor2_d_db21 = 0.0;
        var_isatfor2_d_db22 = 0.0;
        var_isatfor2_d_db23 = 0.0;
        var_isatfor2_d_db24 = 0.0;

        var_mfor2_s = 1.0;
        var_mfor2_s_dn0 = 0.0;
        var_mfor2_s_dn1 = 0.0;
        var_mfor2_s_dn2 = 0.0;
        var_mfor2_s_dn3 = 0.0;
        var_mfor2_s_dn4 = 0.0;
        var_mfor2_s_dn5 = 0.0;
        var_mfor2_s_dn6 = 0.0;
        var_mfor2_s_dn7 = 0.0;
        var_mfor2_s_dn8 = 0.0;
        var_mfor2_s_dn9 = 0.0;
        var_mfor2_s_dn10 = 0.0;
        var_mfor2_s_dn11 = 0.0;
        var_mfor2_s_dn12 = 0.0;
        var_mfor2_s_dn13 = 0.0;
        var_mfor2_s_dn14 = 0.0;
        var_mfor2_s_dn15 = 0.0;
        var_mfor2_s_dn16 = 0.0;
        var_mfor2_s_dn17 = 0.0;
        var_mfor2_s_dn18 = 0.0;
        var_mfor2_s_dn19 = 0.0;
        var_mfor2_s_dn20 = 0.0;
        var_mfor2_s_db0 = 0.0;
        var_mfor2_s_db1 = 0.0;
        var_mfor2_s_db2 = 0.0;
        var_mfor2_s_db3 = 0.0;
        var_mfor2_s_db4 = 0.0;
        var_mfor2_s_db5 = 0.0;
        var_mfor2_s_db6 = 0.0;
        var_mfor2_s_db7 = 0.0;
        var_mfor2_s_db8 = 0.0;
        var_mfor2_s_db9 = 0.0;
        var_mfor2_s_db10 = 0.0;
        var_mfor2_s_db11 = 0.0;
        var_mfor2_s_db12 = 0.0;
        var_mfor2_s_db13 = 0.0;
        var_mfor2_s_db14 = 0.0;
        var_mfor2_s_db15 = 0.0;
        var_mfor2_s_db16 = 0.0;
        var_mfor2_s_db17 = 0.0;
        var_mfor2_s_db18 = 0.0;
        var_mfor2_s_db19 = 0.0;
        var_mfor2_s_db20 = 0.0;
        var_mfor2_s_db21 = 0.0;
        var_mfor2_s_db22 = 0.0;
        var_mfor2_s_db23 = 0.0;
        var_mfor2_s_db24 = 0.0;

        var_mfor2_d = 1.0;
        var_mfor2_d_dn0 = 0.0;
        var_mfor2_d_dn1 = 0.0;
        var_mfor2_d_dn2 = 0.0;
        var_mfor2_d_dn3 = 0.0;
        var_mfor2_d_dn4 = 0.0;
        var_mfor2_d_dn5 = 0.0;
        var_mfor2_d_dn6 = 0.0;
        var_mfor2_d_dn7 = 0.0;
        var_mfor2_d_dn8 = 0.0;
        var_mfor2_d_dn9 = 0.0;
        var_mfor2_d_dn10 = 0.0;
        var_mfor2_d_dn11 = 0.0;
        var_mfor2_d_dn12 = 0.0;
        var_mfor2_d_dn13 = 0.0;
        var_mfor2_d_dn14 = 0.0;
        var_mfor2_d_dn15 = 0.0;
        var_mfor2_d_dn16 = 0.0;
        var_mfor2_d_dn17 = 0.0;
        var_mfor2_d_dn18 = 0.0;
        var_mfor2_d_dn19 = 0.0;
        var_mfor2_d_dn20 = 0.0;
        var_mfor2_d_db0 = 0.0;
        var_mfor2_d_db1 = 0.0;
        var_mfor2_d_db2 = 0.0;
        var_mfor2_d_db3 = 0.0;
        var_mfor2_d_db4 = 0.0;
        var_mfor2_d_db5 = 0.0;
        var_mfor2_d_db6 = 0.0;
        var_mfor2_d_db7 = 0.0;
        var_mfor2_d_db8 = 0.0;
        var_mfor2_d_db9 = 0.0;
        var_mfor2_d_db10 = 0.0;
        var_mfor2_d_db11 = 0.0;
        var_mfor2_d_db12 = 0.0;
        var_mfor2_d_db13 = 0.0;
        var_mfor2_d_db14 = 0.0;
        var_mfor2_d_db15 = 0.0;
        var_mfor2_d_db16 = 0.0;
        var_mfor2_d_db17 = 0.0;
        var_mfor2_d_db18 = 0.0;
        var_mfor2_d_db19 = 0.0;
        var_mfor2_d_db20 = 0.0;
        var_mfor2_d_db21 = 0.0;
        var_mfor2_d_db22 = 0.0;
        var_mfor2_d_db23 = 0.0;
        var_mfor2_d_db24 = 0.0;

        *var_abd_i_slot = var_abd_i;
        *var_abdrain_i_slot = var_abdrain_i;
        *var_abs_i_slot = var_abs_i;
        *var_absource_i_slot = var_absource_i;
        *var_exp_vmax_over_phitd_d_slot = var_exp_vmax_over_phitd_d;
        *var_exp_vmax_over_phitd_s_slot = var_exp_vmax_over_phitd_s;
        *var_guard178_slot = var_guard178;
        *var_guard179_slot = var_guard179;
        *var_guard180_slot = var_guard180;
        *var_guard181_slot = var_guard181;
        *var_gwell_slot = var_gwell;
        *var_isatfor1_d_slot = var_isatfor1_d;
        *var_isatfor1_s_slot = var_isatfor1_s;
        *var_isatfor2_d_slot = var_isatfor2_d;
        *var_isatfor2_d_db0_slot = var_isatfor2_d_db0;
        *var_isatfor2_d_db1_slot = var_isatfor2_d_db1;
        *var_isatfor2_d_db10_slot = var_isatfor2_d_db10;
        *var_isatfor2_d_db11_slot = var_isatfor2_d_db11;
        *var_isatfor2_d_db12_slot = var_isatfor2_d_db12;
        *var_isatfor2_d_db13_slot = var_isatfor2_d_db13;
        *var_isatfor2_d_db14_slot = var_isatfor2_d_db14;
        *var_isatfor2_d_db15_slot = var_isatfor2_d_db15;
        *var_isatfor2_d_db16_slot = var_isatfor2_d_db16;
        *var_isatfor2_d_db17_slot = var_isatfor2_d_db17;
        *var_isatfor2_d_db18_slot = var_isatfor2_d_db18;
        *var_isatfor2_d_db19_slot = var_isatfor2_d_db19;
        *var_isatfor2_d_db2_slot = var_isatfor2_d_db2;
        *var_isatfor2_d_db20_slot = var_isatfor2_d_db20;
        *var_isatfor2_d_db21_slot = var_isatfor2_d_db21;
        *var_isatfor2_d_db22_slot = var_isatfor2_d_db22;
        *var_isatfor2_d_db23_slot = var_isatfor2_d_db23;
        *var_isatfor2_d_db24_slot = var_isatfor2_d_db24;
        *var_isatfor2_d_db3_slot = var_isatfor2_d_db3;
        *var_isatfor2_d_db4_slot = var_isatfor2_d_db4;
        *var_isatfor2_d_db5_slot = var_isatfor2_d_db5;
        *var_isatfor2_d_db6_slot = var_isatfor2_d_db6;
        *var_isatfor2_d_db7_slot = var_isatfor2_d_db7;
        *var_isatfor2_d_db8_slot = var_isatfor2_d_db8;
        *var_isatfor2_d_db9_slot = var_isatfor2_d_db9;
        *var_isatfor2_d_dn0_slot = var_isatfor2_d_dn0;
        *var_isatfor2_d_dn1_slot = var_isatfor2_d_dn1;
        *var_isatfor2_d_dn10_slot = var_isatfor2_d_dn10;
        *var_isatfor2_d_dn11_slot = var_isatfor2_d_dn11;
        *var_isatfor2_d_dn12_slot = var_isatfor2_d_dn12;
        *var_isatfor2_d_dn13_slot = var_isatfor2_d_dn13;
        *var_isatfor2_d_dn14_slot = var_isatfor2_d_dn14;
        *var_isatfor2_d_dn15_slot = var_isatfor2_d_dn15;
        *var_isatfor2_d_dn16_slot = var_isatfor2_d_dn16;
        *var_isatfor2_d_dn17_slot = var_isatfor2_d_dn17;
        *var_isatfor2_d_dn18_slot = var_isatfor2_d_dn18;
        *var_isatfor2_d_dn19_slot = var_isatfor2_d_dn19;
        *var_isatfor2_d_dn2_slot = var_isatfor2_d_dn2;
        *var_isatfor2_d_dn20_slot = var_isatfor2_d_dn20;
        *var_isatfor2_d_dn3_slot = var_isatfor2_d_dn3;
        *var_isatfor2_d_dn4_slot = var_isatfor2_d_dn4;
        *var_isatfor2_d_dn5_slot = var_isatfor2_d_dn5;
        *var_isatfor2_d_dn6_slot = var_isatfor2_d_dn6;
        *var_isatfor2_d_dn7_slot = var_isatfor2_d_dn7;
        *var_isatfor2_d_dn8_slot = var_isatfor2_d_dn8;
        *var_isatfor2_d_dn9_slot = var_isatfor2_d_dn9;
        *var_isatfor2_s_slot = var_isatfor2_s;
        *var_isatfor2_s_db0_slot = var_isatfor2_s_db0;
        *var_isatfor2_s_db1_slot = var_isatfor2_s_db1;
        *var_isatfor2_s_db10_slot = var_isatfor2_s_db10;
        *var_isatfor2_s_db11_slot = var_isatfor2_s_db11;
        *var_isatfor2_s_db12_slot = var_isatfor2_s_db12;
        *var_isatfor2_s_db13_slot = var_isatfor2_s_db13;
        *var_isatfor2_s_db14_slot = var_isatfor2_s_db14;
        *var_isatfor2_s_db15_slot = var_isatfor2_s_db15;
        *var_isatfor2_s_db16_slot = var_isatfor2_s_db16;
        *var_isatfor2_s_db17_slot = var_isatfor2_s_db17;
        *var_isatfor2_s_db18_slot = var_isatfor2_s_db18;
        *var_isatfor2_s_db19_slot = var_isatfor2_s_db19;
        *var_isatfor2_s_db2_slot = var_isatfor2_s_db2;
        *var_isatfor2_s_db20_slot = var_isatfor2_s_db20;
        *var_isatfor2_s_db21_slot = var_isatfor2_s_db21;
        *var_isatfor2_s_db22_slot = var_isatfor2_s_db22;
        *var_isatfor2_s_db23_slot = var_isatfor2_s_db23;
        *var_isatfor2_s_db24_slot = var_isatfor2_s_db24;
        *var_isatfor2_s_db3_slot = var_isatfor2_s_db3;
        *var_isatfor2_s_db4_slot = var_isatfor2_s_db4;
        *var_isatfor2_s_db5_slot = var_isatfor2_s_db5;
        *var_isatfor2_s_db6_slot = var_isatfor2_s_db6;
        *var_isatfor2_s_db7_slot = var_isatfor2_s_db7;
        *var_isatfor2_s_db8_slot = var_isatfor2_s_db8;
        *var_isatfor2_s_db9_slot = var_isatfor2_s_db9;
        *var_isatfor2_s_dn0_slot = var_isatfor2_s_dn0;
        *var_isatfor2_s_dn1_slot = var_isatfor2_s_dn1;
        *var_isatfor2_s_dn10_slot = var_isatfor2_s_dn10;
        *var_isatfor2_s_dn11_slot = var_isatfor2_s_dn11;
        *var_isatfor2_s_dn12_slot = var_isatfor2_s_dn12;
        *var_isatfor2_s_dn13_slot = var_isatfor2_s_dn13;
        *var_isatfor2_s_dn14_slot = var_isatfor2_s_dn14;
        *var_isatfor2_s_dn15_slot = var_isatfor2_s_dn15;
        *var_isatfor2_s_dn16_slot = var_isatfor2_s_dn16;
        *var_isatfor2_s_dn17_slot = var_isatfor2_s_dn17;
        *var_isatfor2_s_dn18_slot = var_isatfor2_s_dn18;
        *var_isatfor2_s_dn19_slot = var_isatfor2_s_dn19;
        *var_isatfor2_s_dn2_slot = var_isatfor2_s_dn2;
        *var_isatfor2_s_dn20_slot = var_isatfor2_s_dn20;
        *var_isatfor2_s_dn3_slot = var_isatfor2_s_dn3;
        *var_isatfor2_s_dn4_slot = var_isatfor2_s_dn4;
        *var_isatfor2_s_dn5_slot = var_isatfor2_s_dn5;
        *var_isatfor2_s_dn6_slot = var_isatfor2_s_dn6;
        *var_isatfor2_s_dn7_slot = var_isatfor2_s_dn7;
        *var_isatfor2_s_dn8_slot = var_isatfor2_s_dn8;
        *var_isatfor2_s_dn9_slot = var_isatfor2_s_dn9;
        *var_jwcorr_slot = var_jwcorr;
        *var_jww_slot = var_jww;
        *var_lgd_i_slot = var_lgd_i;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgs_i_slot = var_lgs_i;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lsd_i_slot = var_lsd_i;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lss_i_slot = var_lss_i;
        *var_lssource_i_slot = var_lssource_i;
        *var_mfor1_d_slot = var_mfor1_d;
        *var_mfor1_s_slot = var_mfor1_s;
        *var_mfor2_d_slot = var_mfor2_d;
        *var_mfor2_d_db0_slot = var_mfor2_d_db0;
        *var_mfor2_d_db1_slot = var_mfor2_d_db1;
        *var_mfor2_d_db10_slot = var_mfor2_d_db10;
        *var_mfor2_d_db11_slot = var_mfor2_d_db11;
        *var_mfor2_d_db12_slot = var_mfor2_d_db12;
        *var_mfor2_d_db13_slot = var_mfor2_d_db13;
        *var_mfor2_d_db14_slot = var_mfor2_d_db14;
        *var_mfor2_d_db15_slot = var_mfor2_d_db15;
        *var_mfor2_d_db16_slot = var_mfor2_d_db16;
        *var_mfor2_d_db17_slot = var_mfor2_d_db17;
        *var_mfor2_d_db18_slot = var_mfor2_d_db18;
        *var_mfor2_d_db19_slot = var_mfor2_d_db19;
        *var_mfor2_d_db2_slot = var_mfor2_d_db2;
        *var_mfor2_d_db20_slot = var_mfor2_d_db20;
        *var_mfor2_d_db21_slot = var_mfor2_d_db21;
        *var_mfor2_d_db22_slot = var_mfor2_d_db22;
        *var_mfor2_d_db23_slot = var_mfor2_d_db23;
        *var_mfor2_d_db24_slot = var_mfor2_d_db24;
        *var_mfor2_d_db3_slot = var_mfor2_d_db3;
        *var_mfor2_d_db4_slot = var_mfor2_d_db4;
        *var_mfor2_d_db5_slot = var_mfor2_d_db5;
        *var_mfor2_d_db6_slot = var_mfor2_d_db6;
        *var_mfor2_d_db7_slot = var_mfor2_d_db7;
        *var_mfor2_d_db8_slot = var_mfor2_d_db8;
        *var_mfor2_d_db9_slot = var_mfor2_d_db9;
        *var_mfor2_d_dn0_slot = var_mfor2_d_dn0;
        *var_mfor2_d_dn1_slot = var_mfor2_d_dn1;
        *var_mfor2_d_dn10_slot = var_mfor2_d_dn10;
        *var_mfor2_d_dn11_slot = var_mfor2_d_dn11;
        *var_mfor2_d_dn12_slot = var_mfor2_d_dn12;
        *var_mfor2_d_dn13_slot = var_mfor2_d_dn13;
        *var_mfor2_d_dn14_slot = var_mfor2_d_dn14;
        *var_mfor2_d_dn15_slot = var_mfor2_d_dn15;
        *var_mfor2_d_dn16_slot = var_mfor2_d_dn16;
        *var_mfor2_d_dn17_slot = var_mfor2_d_dn17;
        *var_mfor2_d_dn18_slot = var_mfor2_d_dn18;
        *var_mfor2_d_dn19_slot = var_mfor2_d_dn19;
        *var_mfor2_d_dn2_slot = var_mfor2_d_dn2;
        *var_mfor2_d_dn20_slot = var_mfor2_d_dn20;
        *var_mfor2_d_dn3_slot = var_mfor2_d_dn3;
        *var_mfor2_d_dn4_slot = var_mfor2_d_dn4;
        *var_mfor2_d_dn5_slot = var_mfor2_d_dn5;
        *var_mfor2_d_dn6_slot = var_mfor2_d_dn6;
        *var_mfor2_d_dn7_slot = var_mfor2_d_dn7;
        *var_mfor2_d_dn8_slot = var_mfor2_d_dn8;
        *var_mfor2_d_dn9_slot = var_mfor2_d_dn9;
        *var_mfor2_s_slot = var_mfor2_s;
        *var_mfor2_s_db0_slot = var_mfor2_s_db0;
        *var_mfor2_s_db1_slot = var_mfor2_s_db1;
        *var_mfor2_s_db10_slot = var_mfor2_s_db10;
        *var_mfor2_s_db11_slot = var_mfor2_s_db11;
        *var_mfor2_s_db12_slot = var_mfor2_s_db12;
        *var_mfor2_s_db13_slot = var_mfor2_s_db13;
        *var_mfor2_s_db14_slot = var_mfor2_s_db14;
        *var_mfor2_s_db15_slot = var_mfor2_s_db15;
        *var_mfor2_s_db16_slot = var_mfor2_s_db16;
        *var_mfor2_s_db17_slot = var_mfor2_s_db17;
        *var_mfor2_s_db18_slot = var_mfor2_s_db18;
        *var_mfor2_s_db19_slot = var_mfor2_s_db19;
        *var_mfor2_s_db2_slot = var_mfor2_s_db2;
        *var_mfor2_s_db20_slot = var_mfor2_s_db20;
        *var_mfor2_s_db21_slot = var_mfor2_s_db21;
        *var_mfor2_s_db22_slot = var_mfor2_s_db22;
        *var_mfor2_s_db23_slot = var_mfor2_s_db23;
        *var_mfor2_s_db24_slot = var_mfor2_s_db24;
        *var_mfor2_s_db3_slot = var_mfor2_s_db3;
        *var_mfor2_s_db4_slot = var_mfor2_s_db4;
        *var_mfor2_s_db5_slot = var_mfor2_s_db5;
        *var_mfor2_s_db6_slot = var_mfor2_s_db6;
        *var_mfor2_s_db7_slot = var_mfor2_s_db7;
        *var_mfor2_s_db8_slot = var_mfor2_s_db8;
        *var_mfor2_s_db9_slot = var_mfor2_s_db9;
        *var_mfor2_s_dn0_slot = var_mfor2_s_dn0;
        *var_mfor2_s_dn1_slot = var_mfor2_s_dn1;
        *var_mfor2_s_dn10_slot = var_mfor2_s_dn10;
        *var_mfor2_s_dn11_slot = var_mfor2_s_dn11;
        *var_mfor2_s_dn12_slot = var_mfor2_s_dn12;
        *var_mfor2_s_dn13_slot = var_mfor2_s_dn13;
        *var_mfor2_s_dn14_slot = var_mfor2_s_dn14;
        *var_mfor2_s_dn15_slot = var_mfor2_s_dn15;
        *var_mfor2_s_dn16_slot = var_mfor2_s_dn16;
        *var_mfor2_s_dn17_slot = var_mfor2_s_dn17;
        *var_mfor2_s_dn18_slot = var_mfor2_s_dn18;
        *var_mfor2_s_dn19_slot = var_mfor2_s_dn19;
        *var_mfor2_s_dn2_slot = var_mfor2_s_dn2;
        *var_mfor2_s_dn20_slot = var_mfor2_s_dn20;
        *var_mfor2_s_dn3_slot = var_mfor2_s_dn3;
        *var_mfor2_s_dn4_slot = var_mfor2_s_dn4;
        *var_mfor2_s_dn5_slot = var_mfor2_s_dn5;
        *var_mfor2_s_dn6_slot = var_mfor2_s_dn6;
        *var_mfor2_s_dn7_slot = var_mfor2_s_dn7;
        *var_mfor2_s_dn8_slot = var_mfor2_s_dn8;
        *var_mfor2_s_dn9_slot = var_mfor2_s_dn9;
        *var_vbbtlim_d_slot = var_vbbtlim_d;
        *var_vbbtlim_s_slot = var_vbbtlim_s;
        *var_vbimin_d_slot = var_vbimin_d;
        *var_vbimin_s_slot = var_vbimin_s;
        *var_vch_d_slot = var_vch_d;
        *var_vch_s_slot = var_vch_s;
        *var_vfmin_d_slot = var_vfmin_d;
        *var_vfmin_s_slot = var_vfmin_s;
        *var_vmax_d_slot = var_vmax_d;
        *var_vmax_s_slot = var_vmax_s;
    }

    pub(super) fn stamp_transient_block_14(
        var_expxhf1_d_slot: &mut f64,
        var_expxhf1_s_slot: &mut f64,
        var_expxhf2_d_slot: &mut f64,
        var_expxhf2_d_db0_slot: &mut f64,
        var_expxhf2_d_db1_slot: &mut f64,
        var_expxhf2_d_db10_slot: &mut f64,
        var_expxhf2_d_db11_slot: &mut f64,
        var_expxhf2_d_db12_slot: &mut f64,
        var_expxhf2_d_db13_slot: &mut f64,
        var_expxhf2_d_db14_slot: &mut f64,
        var_expxhf2_d_db15_slot: &mut f64,
        var_expxhf2_d_db16_slot: &mut f64,
        var_expxhf2_d_db17_slot: &mut f64,
        var_expxhf2_d_db18_slot: &mut f64,
        var_expxhf2_d_db19_slot: &mut f64,
        var_expxhf2_d_db2_slot: &mut f64,
        var_expxhf2_d_db20_slot: &mut f64,
        var_expxhf2_d_db21_slot: &mut f64,
        var_expxhf2_d_db22_slot: &mut f64,
        var_expxhf2_d_db23_slot: &mut f64,
        var_expxhf2_d_db24_slot: &mut f64,
        var_expxhf2_d_db3_slot: &mut f64,
        var_expxhf2_d_db4_slot: &mut f64,
        var_expxhf2_d_db5_slot: &mut f64,
        var_expxhf2_d_db6_slot: &mut f64,
        var_expxhf2_d_db7_slot: &mut f64,
        var_expxhf2_d_db8_slot: &mut f64,
        var_expxhf2_d_db9_slot: &mut f64,
        var_expxhf2_d_dn0_slot: &mut f64,
        var_expxhf2_d_dn1_slot: &mut f64,
        var_expxhf2_d_dn10_slot: &mut f64,
        var_expxhf2_d_dn11_slot: &mut f64,
        var_expxhf2_d_dn12_slot: &mut f64,
        var_expxhf2_d_dn13_slot: &mut f64,
        var_expxhf2_d_dn14_slot: &mut f64,
        var_expxhf2_d_dn15_slot: &mut f64,
        var_expxhf2_d_dn16_slot: &mut f64,
        var_expxhf2_d_dn17_slot: &mut f64,
        var_expxhf2_d_dn18_slot: &mut f64,
        var_expxhf2_d_dn19_slot: &mut f64,
        var_expxhf2_d_dn2_slot: &mut f64,
        var_expxhf2_d_dn20_slot: &mut f64,
        var_expxhf2_d_dn3_slot: &mut f64,
        var_expxhf2_d_dn4_slot: &mut f64,
        var_expxhf2_d_dn5_slot: &mut f64,
        var_expxhf2_d_dn6_slot: &mut f64,
        var_expxhf2_d_dn7_slot: &mut f64,
        var_expxhf2_d_dn8_slot: &mut f64,
        var_expxhf2_d_dn9_slot: &mut f64,
        var_expxhf2_s_slot: &mut f64,
        var_expxhf2_s_db0_slot: &mut f64,
        var_expxhf2_s_db1_slot: &mut f64,
        var_expxhf2_s_db10_slot: &mut f64,
        var_expxhf2_s_db11_slot: &mut f64,
        var_expxhf2_s_db12_slot: &mut f64,
        var_expxhf2_s_db13_slot: &mut f64,
        var_expxhf2_s_db14_slot: &mut f64,
        var_expxhf2_s_db15_slot: &mut f64,
        var_expxhf2_s_db16_slot: &mut f64,
        var_expxhf2_s_db17_slot: &mut f64,
        var_expxhf2_s_db18_slot: &mut f64,
        var_expxhf2_s_db19_slot: &mut f64,
        var_expxhf2_s_db2_slot: &mut f64,
        var_expxhf2_s_db20_slot: &mut f64,
        var_expxhf2_s_db21_slot: &mut f64,
        var_expxhf2_s_db22_slot: &mut f64,
        var_expxhf2_s_db23_slot: &mut f64,
        var_expxhf2_s_db24_slot: &mut f64,
        var_expxhf2_s_db3_slot: &mut f64,
        var_expxhf2_s_db4_slot: &mut f64,
        var_expxhf2_s_db5_slot: &mut f64,
        var_expxhf2_s_db6_slot: &mut f64,
        var_expxhf2_s_db7_slot: &mut f64,
        var_expxhf2_s_db8_slot: &mut f64,
        var_expxhf2_s_db9_slot: &mut f64,
        var_expxhf2_s_dn0_slot: &mut f64,
        var_expxhf2_s_dn1_slot: &mut f64,
        var_expxhf2_s_dn10_slot: &mut f64,
        var_expxhf2_s_dn11_slot: &mut f64,
        var_expxhf2_s_dn12_slot: &mut f64,
        var_expxhf2_s_dn13_slot: &mut f64,
        var_expxhf2_s_dn14_slot: &mut f64,
        var_expxhf2_s_dn15_slot: &mut f64,
        var_expxhf2_s_dn16_slot: &mut f64,
        var_expxhf2_s_dn17_slot: &mut f64,
        var_expxhf2_s_dn18_slot: &mut f64,
        var_expxhf2_s_dn19_slot: &mut f64,
        var_expxhf2_s_dn2_slot: &mut f64,
        var_expxhf2_s_dn20_slot: &mut f64,
        var_expxhf2_s_dn3_slot: &mut f64,
        var_expxhf2_s_dn4_slot: &mut f64,
        var_expxhf2_s_dn5_slot: &mut f64,
        var_expxhf2_s_dn6_slot: &mut f64,
        var_expxhf2_s_dn7_slot: &mut f64,
        var_expxhf2_s_dn8_slot: &mut f64,
        var_expxhf2_s_dn9_slot: &mut f64,
        var_isatrev_d_slot: &mut f64,
        var_isatrev_d_db0_slot: &mut f64,
        var_isatrev_d_db1_slot: &mut f64,
        var_isatrev_d_db10_slot: &mut f64,
        var_isatrev_d_db11_slot: &mut f64,
        var_isatrev_d_db12_slot: &mut f64,
        var_isatrev_d_db13_slot: &mut f64,
        var_isatrev_d_db14_slot: &mut f64,
        var_isatrev_d_db15_slot: &mut f64,
        var_isatrev_d_db16_slot: &mut f64,
        var_isatrev_d_db17_slot: &mut f64,
        var_isatrev_d_db18_slot: &mut f64,
        var_isatrev_d_db19_slot: &mut f64,
        var_isatrev_d_db2_slot: &mut f64,
        var_isatrev_d_db20_slot: &mut f64,
        var_isatrev_d_db21_slot: &mut f64,
        var_isatrev_d_db22_slot: &mut f64,
        var_isatrev_d_db23_slot: &mut f64,
        var_isatrev_d_db24_slot: &mut f64,
        var_isatrev_d_db3_slot: &mut f64,
        var_isatrev_d_db4_slot: &mut f64,
        var_isatrev_d_db5_slot: &mut f64,
        var_isatrev_d_db6_slot: &mut f64,
        var_isatrev_d_db7_slot: &mut f64,
        var_isatrev_d_db8_slot: &mut f64,
        var_isatrev_d_db9_slot: &mut f64,
        var_isatrev_d_dn0_slot: &mut f64,
        var_isatrev_d_dn1_slot: &mut f64,
        var_isatrev_d_dn10_slot: &mut f64,
        var_isatrev_d_dn11_slot: &mut f64,
        var_isatrev_d_dn12_slot: &mut f64,
        var_isatrev_d_dn13_slot: &mut f64,
        var_isatrev_d_dn14_slot: &mut f64,
        var_isatrev_d_dn15_slot: &mut f64,
        var_isatrev_d_dn16_slot: &mut f64,
        var_isatrev_d_dn17_slot: &mut f64,
        var_isatrev_d_dn18_slot: &mut f64,
        var_isatrev_d_dn19_slot: &mut f64,
        var_isatrev_d_dn2_slot: &mut f64,
        var_isatrev_d_dn20_slot: &mut f64,
        var_isatrev_d_dn3_slot: &mut f64,
        var_isatrev_d_dn4_slot: &mut f64,
        var_isatrev_d_dn5_slot: &mut f64,
        var_isatrev_d_dn6_slot: &mut f64,
        var_isatrev_d_dn7_slot: &mut f64,
        var_isatrev_d_dn8_slot: &mut f64,
        var_isatrev_d_dn9_slot: &mut f64,
        var_isatrev_s_slot: &mut f64,
        var_isatrev_s_db0_slot: &mut f64,
        var_isatrev_s_db1_slot: &mut f64,
        var_isatrev_s_db10_slot: &mut f64,
        var_isatrev_s_db11_slot: &mut f64,
        var_isatrev_s_db12_slot: &mut f64,
        var_isatrev_s_db13_slot: &mut f64,
        var_isatrev_s_db14_slot: &mut f64,
        var_isatrev_s_db15_slot: &mut f64,
        var_isatrev_s_db16_slot: &mut f64,
        var_isatrev_s_db17_slot: &mut f64,
        var_isatrev_s_db18_slot: &mut f64,
        var_isatrev_s_db19_slot: &mut f64,
        var_isatrev_s_db2_slot: &mut f64,
        var_isatrev_s_db20_slot: &mut f64,
        var_isatrev_s_db21_slot: &mut f64,
        var_isatrev_s_db22_slot: &mut f64,
        var_isatrev_s_db23_slot: &mut f64,
        var_isatrev_s_db24_slot: &mut f64,
        var_isatrev_s_db3_slot: &mut f64,
        var_isatrev_s_db4_slot: &mut f64,
        var_isatrev_s_db5_slot: &mut f64,
        var_isatrev_s_db6_slot: &mut f64,
        var_isatrev_s_db7_slot: &mut f64,
        var_isatrev_s_db8_slot: &mut f64,
        var_isatrev_s_db9_slot: &mut f64,
        var_isatrev_s_dn0_slot: &mut f64,
        var_isatrev_s_dn1_slot: &mut f64,
        var_isatrev_s_dn10_slot: &mut f64,
        var_isatrev_s_dn11_slot: &mut f64,
        var_isatrev_s_dn12_slot: &mut f64,
        var_isatrev_s_dn13_slot: &mut f64,
        var_isatrev_s_dn14_slot: &mut f64,
        var_isatrev_s_dn15_slot: &mut f64,
        var_isatrev_s_dn16_slot: &mut f64,
        var_isatrev_s_dn17_slot: &mut f64,
        var_isatrev_s_dn18_slot: &mut f64,
        var_isatrev_s_dn19_slot: &mut f64,
        var_isatrev_s_dn2_slot: &mut f64,
        var_isatrev_s_dn20_slot: &mut f64,
        var_isatrev_s_dn3_slot: &mut f64,
        var_isatrev_s_dn4_slot: &mut f64,
        var_isatrev_s_dn5_slot: &mut f64,
        var_isatrev_s_dn6_slot: &mut f64,
        var_isatrev_s_dn7_slot: &mut f64,
        var_isatrev_s_dn8_slot: &mut f64,
        var_isatrev_s_dn9_slot: &mut f64,
        var_m0flag_d_slot: &mut f64,
        var_m0flag_s_slot: &mut f64,
        var_mrev_d_slot: &mut f64,
        var_mrev_d_db0_slot: &mut f64,
        var_mrev_d_db1_slot: &mut f64,
        var_mrev_d_db10_slot: &mut f64,
        var_mrev_d_db11_slot: &mut f64,
        var_mrev_d_db12_slot: &mut f64,
        var_mrev_d_db13_slot: &mut f64,
        var_mrev_d_db14_slot: &mut f64,
        var_mrev_d_db15_slot: &mut f64,
        var_mrev_d_db16_slot: &mut f64,
        var_mrev_d_db17_slot: &mut f64,
        var_mrev_d_db18_slot: &mut f64,
        var_mrev_d_db19_slot: &mut f64,
        var_mrev_d_db2_slot: &mut f64,
        var_mrev_d_db20_slot: &mut f64,
        var_mrev_d_db21_slot: &mut f64,
        var_mrev_d_db22_slot: &mut f64,
        var_mrev_d_db23_slot: &mut f64,
        var_mrev_d_db24_slot: &mut f64,
        var_mrev_d_db3_slot: &mut f64,
        var_mrev_d_db4_slot: &mut f64,
        var_mrev_d_db5_slot: &mut f64,
        var_mrev_d_db6_slot: &mut f64,
        var_mrev_d_db7_slot: &mut f64,
        var_mrev_d_db8_slot: &mut f64,
        var_mrev_d_db9_slot: &mut f64,
        var_mrev_d_dn0_slot: &mut f64,
        var_mrev_d_dn1_slot: &mut f64,
        var_mrev_d_dn10_slot: &mut f64,
        var_mrev_d_dn11_slot: &mut f64,
        var_mrev_d_dn12_slot: &mut f64,
        var_mrev_d_dn13_slot: &mut f64,
        var_mrev_d_dn14_slot: &mut f64,
        var_mrev_d_dn15_slot: &mut f64,
        var_mrev_d_dn16_slot: &mut f64,
        var_mrev_d_dn17_slot: &mut f64,
        var_mrev_d_dn18_slot: &mut f64,
        var_mrev_d_dn19_slot: &mut f64,
        var_mrev_d_dn2_slot: &mut f64,
        var_mrev_d_dn20_slot: &mut f64,
        var_mrev_d_dn3_slot: &mut f64,
        var_mrev_d_dn4_slot: &mut f64,
        var_mrev_d_dn5_slot: &mut f64,
        var_mrev_d_dn6_slot: &mut f64,
        var_mrev_d_dn7_slot: &mut f64,
        var_mrev_d_dn8_slot: &mut f64,
        var_mrev_d_dn9_slot: &mut f64,
        var_mrev_s_slot: &mut f64,
        var_mrev_s_db0_slot: &mut f64,
        var_mrev_s_db1_slot: &mut f64,
        var_mrev_s_db10_slot: &mut f64,
        var_mrev_s_db11_slot: &mut f64,
        var_mrev_s_db12_slot: &mut f64,
        var_mrev_s_db13_slot: &mut f64,
        var_mrev_s_db14_slot: &mut f64,
        var_mrev_s_db15_slot: &mut f64,
        var_mrev_s_db16_slot: &mut f64,
        var_mrev_s_db17_slot: &mut f64,
        var_mrev_s_db18_slot: &mut f64,
        var_mrev_s_db19_slot: &mut f64,
        var_mrev_s_db2_slot: &mut f64,
        var_mrev_s_db20_slot: &mut f64,
        var_mrev_s_db21_slot: &mut f64,
        var_mrev_s_db22_slot: &mut f64,
        var_mrev_s_db23_slot: &mut f64,
        var_mrev_s_db24_slot: &mut f64,
        var_mrev_s_db3_slot: &mut f64,
        var_mrev_s_db4_slot: &mut f64,
        var_mrev_s_db5_slot: &mut f64,
        var_mrev_s_db6_slot: &mut f64,
        var_mrev_s_db7_slot: &mut f64,
        var_mrev_s_db8_slot: &mut f64,
        var_mrev_s_db9_slot: &mut f64,
        var_mrev_s_dn0_slot: &mut f64,
        var_mrev_s_dn1_slot: &mut f64,
        var_mrev_s_dn10_slot: &mut f64,
        var_mrev_s_dn11_slot: &mut f64,
        var_mrev_s_dn12_slot: &mut f64,
        var_mrev_s_dn13_slot: &mut f64,
        var_mrev_s_dn14_slot: &mut f64,
        var_mrev_s_dn15_slot: &mut f64,
        var_mrev_s_dn16_slot: &mut f64,
        var_mrev_s_dn17_slot: &mut f64,
        var_mrev_s_dn18_slot: &mut f64,
        var_mrev_s_dn19_slot: &mut f64,
        var_mrev_s_dn2_slot: &mut f64,
        var_mrev_s_dn20_slot: &mut f64,
        var_mrev_s_dn3_slot: &mut f64,
        var_mrev_s_dn4_slot: &mut f64,
        var_mrev_s_dn5_slot: &mut f64,
        var_mrev_s_dn6_slot: &mut f64,
        var_mrev_s_dn7_slot: &mut f64,
        var_mrev_s_dn8_slot: &mut f64,
        var_mrev_s_dn9_slot: &mut f64,
        var_xhighf1_d_slot: &mut f64,
        var_xhighf1_s_slot: &mut f64,
        var_xhighf2_d_slot: &mut f64,
        var_xhighf2_d_db0_slot: &mut f64,
        var_xhighf2_d_db1_slot: &mut f64,
        var_xhighf2_d_db10_slot: &mut f64,
        var_xhighf2_d_db11_slot: &mut f64,
        var_xhighf2_d_db12_slot: &mut f64,
        var_xhighf2_d_db13_slot: &mut f64,
        var_xhighf2_d_db14_slot: &mut f64,
        var_xhighf2_d_db15_slot: &mut f64,
        var_xhighf2_d_db16_slot: &mut f64,
        var_xhighf2_d_db17_slot: &mut f64,
        var_xhighf2_d_db18_slot: &mut f64,
        var_xhighf2_d_db19_slot: &mut f64,
        var_xhighf2_d_db2_slot: &mut f64,
        var_xhighf2_d_db20_slot: &mut f64,
        var_xhighf2_d_db21_slot: &mut f64,
        var_xhighf2_d_db22_slot: &mut f64,
        var_xhighf2_d_db23_slot: &mut f64,
        var_xhighf2_d_db24_slot: &mut f64,
        var_xhighf2_d_db3_slot: &mut f64,
        var_xhighf2_d_db4_slot: &mut f64,
        var_xhighf2_d_db5_slot: &mut f64,
        var_xhighf2_d_db6_slot: &mut f64,
        var_xhighf2_d_db7_slot: &mut f64,
        var_xhighf2_d_db8_slot: &mut f64,
        var_xhighf2_d_db9_slot: &mut f64,
        var_xhighf2_d_dn0_slot: &mut f64,
        var_xhighf2_d_dn1_slot: &mut f64,
        var_xhighf2_d_dn10_slot: &mut f64,
        var_xhighf2_d_dn11_slot: &mut f64,
        var_xhighf2_d_dn12_slot: &mut f64,
        var_xhighf2_d_dn13_slot: &mut f64,
        var_xhighf2_d_dn14_slot: &mut f64,
        var_xhighf2_d_dn15_slot: &mut f64,
        var_xhighf2_d_dn16_slot: &mut f64,
        var_xhighf2_d_dn17_slot: &mut f64,
        var_xhighf2_d_dn18_slot: &mut f64,
        var_xhighf2_d_dn19_slot: &mut f64,
        var_xhighf2_d_dn2_slot: &mut f64,
        var_xhighf2_d_dn20_slot: &mut f64,
        var_xhighf2_d_dn3_slot: &mut f64,
        var_xhighf2_d_dn4_slot: &mut f64,
        var_xhighf2_d_dn5_slot: &mut f64,
        var_xhighf2_d_dn6_slot: &mut f64,
        var_xhighf2_d_dn7_slot: &mut f64,
        var_xhighf2_d_dn8_slot: &mut f64,
        var_xhighf2_d_dn9_slot: &mut f64,
        var_xhighf2_s_slot: &mut f64,
        var_xhighf2_s_db0_slot: &mut f64,
        var_xhighf2_s_db1_slot: &mut f64,
        var_xhighf2_s_db10_slot: &mut f64,
        var_xhighf2_s_db11_slot: &mut f64,
        var_xhighf2_s_db12_slot: &mut f64,
        var_xhighf2_s_db13_slot: &mut f64,
        var_xhighf2_s_db14_slot: &mut f64,
        var_xhighf2_s_db15_slot: &mut f64,
        var_xhighf2_s_db16_slot: &mut f64,
        var_xhighf2_s_db17_slot: &mut f64,
        var_xhighf2_s_db18_slot: &mut f64,
        var_xhighf2_s_db19_slot: &mut f64,
        var_xhighf2_s_db2_slot: &mut f64,
        var_xhighf2_s_db20_slot: &mut f64,
        var_xhighf2_s_db21_slot: &mut f64,
        var_xhighf2_s_db22_slot: &mut f64,
        var_xhighf2_s_db23_slot: &mut f64,
        var_xhighf2_s_db24_slot: &mut f64,
        var_xhighf2_s_db3_slot: &mut f64,
        var_xhighf2_s_db4_slot: &mut f64,
        var_xhighf2_s_db5_slot: &mut f64,
        var_xhighf2_s_db6_slot: &mut f64,
        var_xhighf2_s_db7_slot: &mut f64,
        var_xhighf2_s_db8_slot: &mut f64,
        var_xhighf2_s_db9_slot: &mut f64,
        var_xhighf2_s_dn0_slot: &mut f64,
        var_xhighf2_s_dn1_slot: &mut f64,
        var_xhighf2_s_dn10_slot: &mut f64,
        var_xhighf2_s_dn11_slot: &mut f64,
        var_xhighf2_s_dn12_slot: &mut f64,
        var_xhighf2_s_dn13_slot: &mut f64,
        var_xhighf2_s_dn14_slot: &mut f64,
        var_xhighf2_s_dn15_slot: &mut f64,
        var_xhighf2_s_dn16_slot: &mut f64,
        var_xhighf2_s_dn17_slot: &mut f64,
        var_xhighf2_s_dn18_slot: &mut f64,
        var_xhighf2_s_dn19_slot: &mut f64,
        var_xhighf2_s_dn2_slot: &mut f64,
        var_xhighf2_s_dn20_slot: &mut f64,
        var_xhighf2_s_dn3_slot: &mut f64,
        var_xhighf2_s_dn4_slot: &mut f64,
        var_xhighf2_s_dn5_slot: &mut f64,
        var_xhighf2_s_dn6_slot: &mut f64,
        var_xhighf2_s_dn7_slot: &mut f64,
        var_xhighf2_s_dn8_slot: &mut f64,
        var_xhighf2_s_dn9_slot: &mut f64,
        var_xhighr_d_slot: &mut f64,
        var_xhighr_d_db0_slot: &mut f64,
        var_xhighr_d_db1_slot: &mut f64,
        var_xhighr_d_db10_slot: &mut f64,
        var_xhighr_d_db11_slot: &mut f64,
        var_xhighr_d_db12_slot: &mut f64,
        var_xhighr_d_db13_slot: &mut f64,
        var_xhighr_d_db14_slot: &mut f64,
        var_xhighr_d_db15_slot: &mut f64,
        var_xhighr_d_db16_slot: &mut f64,
        var_xhighr_d_db17_slot: &mut f64,
        var_xhighr_d_db18_slot: &mut f64,
        var_xhighr_d_db19_slot: &mut f64,
        var_xhighr_d_db2_slot: &mut f64,
        var_xhighr_d_db20_slot: &mut f64,
        var_xhighr_d_db21_slot: &mut f64,
        var_xhighr_d_db22_slot: &mut f64,
        var_xhighr_d_db23_slot: &mut f64,
        var_xhighr_d_db24_slot: &mut f64,
        var_xhighr_d_db3_slot: &mut f64,
        var_xhighr_d_db4_slot: &mut f64,
        var_xhighr_d_db5_slot: &mut f64,
        var_xhighr_d_db6_slot: &mut f64,
        var_xhighr_d_db7_slot: &mut f64,
        var_xhighr_d_db8_slot: &mut f64,
        var_xhighr_d_db9_slot: &mut f64,
        var_xhighr_d_dn0_slot: &mut f64,
        var_xhighr_d_dn1_slot: &mut f64,
        var_xhighr_d_dn10_slot: &mut f64,
        var_xhighr_d_dn11_slot: &mut f64,
        var_xhighr_d_dn12_slot: &mut f64,
        var_xhighr_d_dn13_slot: &mut f64,
        var_xhighr_d_dn14_slot: &mut f64,
        var_xhighr_d_dn15_slot: &mut f64,
        var_xhighr_d_dn16_slot: &mut f64,
        var_xhighr_d_dn17_slot: &mut f64,
        var_xhighr_d_dn18_slot: &mut f64,
        var_xhighr_d_dn19_slot: &mut f64,
        var_xhighr_d_dn2_slot: &mut f64,
        var_xhighr_d_dn20_slot: &mut f64,
        var_xhighr_d_dn3_slot: &mut f64,
        var_xhighr_d_dn4_slot: &mut f64,
        var_xhighr_d_dn5_slot: &mut f64,
        var_xhighr_d_dn6_slot: &mut f64,
        var_xhighr_d_dn7_slot: &mut f64,
        var_xhighr_d_dn8_slot: &mut f64,
        var_xhighr_d_dn9_slot: &mut f64,
        var_xhighr_s_slot: &mut f64,
        var_xhighr_s_db0_slot: &mut f64,
        var_xhighr_s_db1_slot: &mut f64,
        var_xhighr_s_db10_slot: &mut f64,
        var_xhighr_s_db11_slot: &mut f64,
        var_xhighr_s_db12_slot: &mut f64,
        var_xhighr_s_db13_slot: &mut f64,
        var_xhighr_s_db14_slot: &mut f64,
        var_xhighr_s_db15_slot: &mut f64,
        var_xhighr_s_db16_slot: &mut f64,
        var_xhighr_s_db17_slot: &mut f64,
        var_xhighr_s_db18_slot: &mut f64,
        var_xhighr_s_db19_slot: &mut f64,
        var_xhighr_s_db2_slot: &mut f64,
        var_xhighr_s_db20_slot: &mut f64,
        var_xhighr_s_db21_slot: &mut f64,
        var_xhighr_s_db22_slot: &mut f64,
        var_xhighr_s_db23_slot: &mut f64,
        var_xhighr_s_db24_slot: &mut f64,
        var_xhighr_s_db3_slot: &mut f64,
        var_xhighr_s_db4_slot: &mut f64,
        var_xhighr_s_db5_slot: &mut f64,
        var_xhighr_s_db6_slot: &mut f64,
        var_xhighr_s_db7_slot: &mut f64,
        var_xhighr_s_db8_slot: &mut f64,
        var_xhighr_s_db9_slot: &mut f64,
        var_xhighr_s_dn0_slot: &mut f64,
        var_xhighr_s_dn1_slot: &mut f64,
        var_xhighr_s_dn10_slot: &mut f64,
        var_xhighr_s_dn11_slot: &mut f64,
        var_xhighr_s_dn12_slot: &mut f64,
        var_xhighr_s_dn13_slot: &mut f64,
        var_xhighr_s_dn14_slot: &mut f64,
        var_xhighr_s_dn15_slot: &mut f64,
        var_xhighr_s_dn16_slot: &mut f64,
        var_xhighr_s_dn17_slot: &mut f64,
        var_xhighr_s_dn18_slot: &mut f64,
        var_xhighr_s_dn19_slot: &mut f64,
        var_xhighr_s_dn2_slot: &mut f64,
        var_xhighr_s_dn20_slot: &mut f64,
        var_xhighr_s_dn3_slot: &mut f64,
        var_xhighr_s_dn4_slot: &mut f64,
        var_xhighr_s_dn5_slot: &mut f64,
        var_xhighr_s_dn6_slot: &mut f64,
        var_xhighr_s_dn7_slot: &mut f64,
        var_xhighr_s_dn8_slot: &mut f64,
        var_xhighr_s_dn9_slot: &mut f64,
    ) {
        let mut var_expxhf1_d: f64 = *var_expxhf1_d_slot;
        let mut var_expxhf1_s: f64 = *var_expxhf1_s_slot;
        let mut var_expxhf2_d: f64 = *var_expxhf2_d_slot;
        let mut var_expxhf2_d_db0: f64 = *var_expxhf2_d_db0_slot;
        let mut var_expxhf2_d_db1: f64 = *var_expxhf2_d_db1_slot;
        let mut var_expxhf2_d_db10: f64 = *var_expxhf2_d_db10_slot;
        let mut var_expxhf2_d_db11: f64 = *var_expxhf2_d_db11_slot;
        let mut var_expxhf2_d_db12: f64 = *var_expxhf2_d_db12_slot;
        let mut var_expxhf2_d_db13: f64 = *var_expxhf2_d_db13_slot;
        let mut var_expxhf2_d_db14: f64 = *var_expxhf2_d_db14_slot;
        let mut var_expxhf2_d_db15: f64 = *var_expxhf2_d_db15_slot;
        let mut var_expxhf2_d_db16: f64 = *var_expxhf2_d_db16_slot;
        let mut var_expxhf2_d_db17: f64 = *var_expxhf2_d_db17_slot;
        let mut var_expxhf2_d_db18: f64 = *var_expxhf2_d_db18_slot;
        let mut var_expxhf2_d_db19: f64 = *var_expxhf2_d_db19_slot;
        let mut var_expxhf2_d_db2: f64 = *var_expxhf2_d_db2_slot;
        let mut var_expxhf2_d_db20: f64 = *var_expxhf2_d_db20_slot;
        let mut var_expxhf2_d_db21: f64 = *var_expxhf2_d_db21_slot;
        let mut var_expxhf2_d_db22: f64 = *var_expxhf2_d_db22_slot;
        let mut var_expxhf2_d_db23: f64 = *var_expxhf2_d_db23_slot;
        let mut var_expxhf2_d_db24: f64 = *var_expxhf2_d_db24_slot;
        let mut var_expxhf2_d_db3: f64 = *var_expxhf2_d_db3_slot;
        let mut var_expxhf2_d_db4: f64 = *var_expxhf2_d_db4_slot;
        let mut var_expxhf2_d_db5: f64 = *var_expxhf2_d_db5_slot;
        let mut var_expxhf2_d_db6: f64 = *var_expxhf2_d_db6_slot;
        let mut var_expxhf2_d_db7: f64 = *var_expxhf2_d_db7_slot;
        let mut var_expxhf2_d_db8: f64 = *var_expxhf2_d_db8_slot;
        let mut var_expxhf2_d_db9: f64 = *var_expxhf2_d_db9_slot;
        let mut var_expxhf2_d_dn0: f64 = *var_expxhf2_d_dn0_slot;
        let mut var_expxhf2_d_dn1: f64 = *var_expxhf2_d_dn1_slot;
        let mut var_expxhf2_d_dn10: f64 = *var_expxhf2_d_dn10_slot;
        let mut var_expxhf2_d_dn11: f64 = *var_expxhf2_d_dn11_slot;
        let mut var_expxhf2_d_dn12: f64 = *var_expxhf2_d_dn12_slot;
        let mut var_expxhf2_d_dn13: f64 = *var_expxhf2_d_dn13_slot;
        let mut var_expxhf2_d_dn14: f64 = *var_expxhf2_d_dn14_slot;
        let mut var_expxhf2_d_dn15: f64 = *var_expxhf2_d_dn15_slot;
        let mut var_expxhf2_d_dn16: f64 = *var_expxhf2_d_dn16_slot;
        let mut var_expxhf2_d_dn17: f64 = *var_expxhf2_d_dn17_slot;
        let mut var_expxhf2_d_dn18: f64 = *var_expxhf2_d_dn18_slot;
        let mut var_expxhf2_d_dn19: f64 = *var_expxhf2_d_dn19_slot;
        let mut var_expxhf2_d_dn2: f64 = *var_expxhf2_d_dn2_slot;
        let mut var_expxhf2_d_dn20: f64 = *var_expxhf2_d_dn20_slot;
        let mut var_expxhf2_d_dn3: f64 = *var_expxhf2_d_dn3_slot;
        let mut var_expxhf2_d_dn4: f64 = *var_expxhf2_d_dn4_slot;
        let mut var_expxhf2_d_dn5: f64 = *var_expxhf2_d_dn5_slot;
        let mut var_expxhf2_d_dn6: f64 = *var_expxhf2_d_dn6_slot;
        let mut var_expxhf2_d_dn7: f64 = *var_expxhf2_d_dn7_slot;
        let mut var_expxhf2_d_dn8: f64 = *var_expxhf2_d_dn8_slot;
        let mut var_expxhf2_d_dn9: f64 = *var_expxhf2_d_dn9_slot;
        let mut var_expxhf2_s: f64 = *var_expxhf2_s_slot;
        let mut var_expxhf2_s_db0: f64 = *var_expxhf2_s_db0_slot;
        let mut var_expxhf2_s_db1: f64 = *var_expxhf2_s_db1_slot;
        let mut var_expxhf2_s_db10: f64 = *var_expxhf2_s_db10_slot;
        let mut var_expxhf2_s_db11: f64 = *var_expxhf2_s_db11_slot;
        let mut var_expxhf2_s_db12: f64 = *var_expxhf2_s_db12_slot;
        let mut var_expxhf2_s_db13: f64 = *var_expxhf2_s_db13_slot;
        let mut var_expxhf2_s_db14: f64 = *var_expxhf2_s_db14_slot;
        let mut var_expxhf2_s_db15: f64 = *var_expxhf2_s_db15_slot;
        let mut var_expxhf2_s_db16: f64 = *var_expxhf2_s_db16_slot;
        let mut var_expxhf2_s_db17: f64 = *var_expxhf2_s_db17_slot;
        let mut var_expxhf2_s_db18: f64 = *var_expxhf2_s_db18_slot;
        let mut var_expxhf2_s_db19: f64 = *var_expxhf2_s_db19_slot;
        let mut var_expxhf2_s_db2: f64 = *var_expxhf2_s_db2_slot;
        let mut var_expxhf2_s_db20: f64 = *var_expxhf2_s_db20_slot;
        let mut var_expxhf2_s_db21: f64 = *var_expxhf2_s_db21_slot;
        let mut var_expxhf2_s_db22: f64 = *var_expxhf2_s_db22_slot;
        let mut var_expxhf2_s_db23: f64 = *var_expxhf2_s_db23_slot;
        let mut var_expxhf2_s_db24: f64 = *var_expxhf2_s_db24_slot;
        let mut var_expxhf2_s_db3: f64 = *var_expxhf2_s_db3_slot;
        let mut var_expxhf2_s_db4: f64 = *var_expxhf2_s_db4_slot;
        let mut var_expxhf2_s_db5: f64 = *var_expxhf2_s_db5_slot;
        let mut var_expxhf2_s_db6: f64 = *var_expxhf2_s_db6_slot;
        let mut var_expxhf2_s_db7: f64 = *var_expxhf2_s_db7_slot;
        let mut var_expxhf2_s_db8: f64 = *var_expxhf2_s_db8_slot;
        let mut var_expxhf2_s_db9: f64 = *var_expxhf2_s_db9_slot;
        let mut var_expxhf2_s_dn0: f64 = *var_expxhf2_s_dn0_slot;
        let mut var_expxhf2_s_dn1: f64 = *var_expxhf2_s_dn1_slot;
        let mut var_expxhf2_s_dn10: f64 = *var_expxhf2_s_dn10_slot;
        let mut var_expxhf2_s_dn11: f64 = *var_expxhf2_s_dn11_slot;
        let mut var_expxhf2_s_dn12: f64 = *var_expxhf2_s_dn12_slot;
        let mut var_expxhf2_s_dn13: f64 = *var_expxhf2_s_dn13_slot;
        let mut var_expxhf2_s_dn14: f64 = *var_expxhf2_s_dn14_slot;
        let mut var_expxhf2_s_dn15: f64 = *var_expxhf2_s_dn15_slot;
        let mut var_expxhf2_s_dn16: f64 = *var_expxhf2_s_dn16_slot;
        let mut var_expxhf2_s_dn17: f64 = *var_expxhf2_s_dn17_slot;
        let mut var_expxhf2_s_dn18: f64 = *var_expxhf2_s_dn18_slot;
        let mut var_expxhf2_s_dn19: f64 = *var_expxhf2_s_dn19_slot;
        let mut var_expxhf2_s_dn2: f64 = *var_expxhf2_s_dn2_slot;
        let mut var_expxhf2_s_dn20: f64 = *var_expxhf2_s_dn20_slot;
        let mut var_expxhf2_s_dn3: f64 = *var_expxhf2_s_dn3_slot;
        let mut var_expxhf2_s_dn4: f64 = *var_expxhf2_s_dn4_slot;
        let mut var_expxhf2_s_dn5: f64 = *var_expxhf2_s_dn5_slot;
        let mut var_expxhf2_s_dn6: f64 = *var_expxhf2_s_dn6_slot;
        let mut var_expxhf2_s_dn7: f64 = *var_expxhf2_s_dn7_slot;
        let mut var_expxhf2_s_dn8: f64 = *var_expxhf2_s_dn8_slot;
        let mut var_expxhf2_s_dn9: f64 = *var_expxhf2_s_dn9_slot;
        let mut var_isatrev_d: f64 = *var_isatrev_d_slot;
        let mut var_isatrev_d_db0: f64 = *var_isatrev_d_db0_slot;
        let mut var_isatrev_d_db1: f64 = *var_isatrev_d_db1_slot;
        let mut var_isatrev_d_db10: f64 = *var_isatrev_d_db10_slot;
        let mut var_isatrev_d_db11: f64 = *var_isatrev_d_db11_slot;
        let mut var_isatrev_d_db12: f64 = *var_isatrev_d_db12_slot;
        let mut var_isatrev_d_db13: f64 = *var_isatrev_d_db13_slot;
        let mut var_isatrev_d_db14: f64 = *var_isatrev_d_db14_slot;
        let mut var_isatrev_d_db15: f64 = *var_isatrev_d_db15_slot;
        let mut var_isatrev_d_db16: f64 = *var_isatrev_d_db16_slot;
        let mut var_isatrev_d_db17: f64 = *var_isatrev_d_db17_slot;
        let mut var_isatrev_d_db18: f64 = *var_isatrev_d_db18_slot;
        let mut var_isatrev_d_db19: f64 = *var_isatrev_d_db19_slot;
        let mut var_isatrev_d_db2: f64 = *var_isatrev_d_db2_slot;
        let mut var_isatrev_d_db20: f64 = *var_isatrev_d_db20_slot;
        let mut var_isatrev_d_db21: f64 = *var_isatrev_d_db21_slot;
        let mut var_isatrev_d_db22: f64 = *var_isatrev_d_db22_slot;
        let mut var_isatrev_d_db23: f64 = *var_isatrev_d_db23_slot;
        let mut var_isatrev_d_db24: f64 = *var_isatrev_d_db24_slot;
        let mut var_isatrev_d_db3: f64 = *var_isatrev_d_db3_slot;
        let mut var_isatrev_d_db4: f64 = *var_isatrev_d_db4_slot;
        let mut var_isatrev_d_db5: f64 = *var_isatrev_d_db5_slot;
        let mut var_isatrev_d_db6: f64 = *var_isatrev_d_db6_slot;
        let mut var_isatrev_d_db7: f64 = *var_isatrev_d_db7_slot;
        let mut var_isatrev_d_db8: f64 = *var_isatrev_d_db8_slot;
        let mut var_isatrev_d_db9: f64 = *var_isatrev_d_db9_slot;
        let mut var_isatrev_d_dn0: f64 = *var_isatrev_d_dn0_slot;
        let mut var_isatrev_d_dn1: f64 = *var_isatrev_d_dn1_slot;
        let mut var_isatrev_d_dn10: f64 = *var_isatrev_d_dn10_slot;
        let mut var_isatrev_d_dn11: f64 = *var_isatrev_d_dn11_slot;
        let mut var_isatrev_d_dn12: f64 = *var_isatrev_d_dn12_slot;
        let mut var_isatrev_d_dn13: f64 = *var_isatrev_d_dn13_slot;
        let mut var_isatrev_d_dn14: f64 = *var_isatrev_d_dn14_slot;
        let mut var_isatrev_d_dn15: f64 = *var_isatrev_d_dn15_slot;
        let mut var_isatrev_d_dn16: f64 = *var_isatrev_d_dn16_slot;
        let mut var_isatrev_d_dn17: f64 = *var_isatrev_d_dn17_slot;
        let mut var_isatrev_d_dn18: f64 = *var_isatrev_d_dn18_slot;
        let mut var_isatrev_d_dn19: f64 = *var_isatrev_d_dn19_slot;
        let mut var_isatrev_d_dn2: f64 = *var_isatrev_d_dn2_slot;
        let mut var_isatrev_d_dn20: f64 = *var_isatrev_d_dn20_slot;
        let mut var_isatrev_d_dn3: f64 = *var_isatrev_d_dn3_slot;
        let mut var_isatrev_d_dn4: f64 = *var_isatrev_d_dn4_slot;
        let mut var_isatrev_d_dn5: f64 = *var_isatrev_d_dn5_slot;
        let mut var_isatrev_d_dn6: f64 = *var_isatrev_d_dn6_slot;
        let mut var_isatrev_d_dn7: f64 = *var_isatrev_d_dn7_slot;
        let mut var_isatrev_d_dn8: f64 = *var_isatrev_d_dn8_slot;
        let mut var_isatrev_d_dn9: f64 = *var_isatrev_d_dn9_slot;
        let mut var_isatrev_s: f64 = *var_isatrev_s_slot;
        let mut var_isatrev_s_db0: f64 = *var_isatrev_s_db0_slot;
        let mut var_isatrev_s_db1: f64 = *var_isatrev_s_db1_slot;
        let mut var_isatrev_s_db10: f64 = *var_isatrev_s_db10_slot;
        let mut var_isatrev_s_db11: f64 = *var_isatrev_s_db11_slot;
        let mut var_isatrev_s_db12: f64 = *var_isatrev_s_db12_slot;
        let mut var_isatrev_s_db13: f64 = *var_isatrev_s_db13_slot;
        let mut var_isatrev_s_db14: f64 = *var_isatrev_s_db14_slot;
        let mut var_isatrev_s_db15: f64 = *var_isatrev_s_db15_slot;
        let mut var_isatrev_s_db16: f64 = *var_isatrev_s_db16_slot;
        let mut var_isatrev_s_db17: f64 = *var_isatrev_s_db17_slot;
        let mut var_isatrev_s_db18: f64 = *var_isatrev_s_db18_slot;
        let mut var_isatrev_s_db19: f64 = *var_isatrev_s_db19_slot;
        let mut var_isatrev_s_db2: f64 = *var_isatrev_s_db2_slot;
        let mut var_isatrev_s_db20: f64 = *var_isatrev_s_db20_slot;
        let mut var_isatrev_s_db21: f64 = *var_isatrev_s_db21_slot;
        let mut var_isatrev_s_db22: f64 = *var_isatrev_s_db22_slot;
        let mut var_isatrev_s_db23: f64 = *var_isatrev_s_db23_slot;
        let mut var_isatrev_s_db24: f64 = *var_isatrev_s_db24_slot;
        let mut var_isatrev_s_db3: f64 = *var_isatrev_s_db3_slot;
        let mut var_isatrev_s_db4: f64 = *var_isatrev_s_db4_slot;
        let mut var_isatrev_s_db5: f64 = *var_isatrev_s_db5_slot;
        let mut var_isatrev_s_db6: f64 = *var_isatrev_s_db6_slot;
        let mut var_isatrev_s_db7: f64 = *var_isatrev_s_db7_slot;
        let mut var_isatrev_s_db8: f64 = *var_isatrev_s_db8_slot;
        let mut var_isatrev_s_db9: f64 = *var_isatrev_s_db9_slot;
        let mut var_isatrev_s_dn0: f64 = *var_isatrev_s_dn0_slot;
        let mut var_isatrev_s_dn1: f64 = *var_isatrev_s_dn1_slot;
        let mut var_isatrev_s_dn10: f64 = *var_isatrev_s_dn10_slot;
        let mut var_isatrev_s_dn11: f64 = *var_isatrev_s_dn11_slot;
        let mut var_isatrev_s_dn12: f64 = *var_isatrev_s_dn12_slot;
        let mut var_isatrev_s_dn13: f64 = *var_isatrev_s_dn13_slot;
        let mut var_isatrev_s_dn14: f64 = *var_isatrev_s_dn14_slot;
        let mut var_isatrev_s_dn15: f64 = *var_isatrev_s_dn15_slot;
        let mut var_isatrev_s_dn16: f64 = *var_isatrev_s_dn16_slot;
        let mut var_isatrev_s_dn17: f64 = *var_isatrev_s_dn17_slot;
        let mut var_isatrev_s_dn18: f64 = *var_isatrev_s_dn18_slot;
        let mut var_isatrev_s_dn19: f64 = *var_isatrev_s_dn19_slot;
        let mut var_isatrev_s_dn2: f64 = *var_isatrev_s_dn2_slot;
        let mut var_isatrev_s_dn20: f64 = *var_isatrev_s_dn20_slot;
        let mut var_isatrev_s_dn3: f64 = *var_isatrev_s_dn3_slot;
        let mut var_isatrev_s_dn4: f64 = *var_isatrev_s_dn4_slot;
        let mut var_isatrev_s_dn5: f64 = *var_isatrev_s_dn5_slot;
        let mut var_isatrev_s_dn6: f64 = *var_isatrev_s_dn6_slot;
        let mut var_isatrev_s_dn7: f64 = *var_isatrev_s_dn7_slot;
        let mut var_isatrev_s_dn8: f64 = *var_isatrev_s_dn8_slot;
        let mut var_isatrev_s_dn9: f64 = *var_isatrev_s_dn9_slot;
        let mut var_m0flag_d: f64 = *var_m0flag_d_slot;
        let mut var_m0flag_s: f64 = *var_m0flag_s_slot;
        let mut var_mrev_d: f64 = *var_mrev_d_slot;
        let mut var_mrev_d_db0: f64 = *var_mrev_d_db0_slot;
        let mut var_mrev_d_db1: f64 = *var_mrev_d_db1_slot;
        let mut var_mrev_d_db10: f64 = *var_mrev_d_db10_slot;
        let mut var_mrev_d_db11: f64 = *var_mrev_d_db11_slot;
        let mut var_mrev_d_db12: f64 = *var_mrev_d_db12_slot;
        let mut var_mrev_d_db13: f64 = *var_mrev_d_db13_slot;
        let mut var_mrev_d_db14: f64 = *var_mrev_d_db14_slot;
        let mut var_mrev_d_db15: f64 = *var_mrev_d_db15_slot;
        let mut var_mrev_d_db16: f64 = *var_mrev_d_db16_slot;
        let mut var_mrev_d_db17: f64 = *var_mrev_d_db17_slot;
        let mut var_mrev_d_db18: f64 = *var_mrev_d_db18_slot;
        let mut var_mrev_d_db19: f64 = *var_mrev_d_db19_slot;
        let mut var_mrev_d_db2: f64 = *var_mrev_d_db2_slot;
        let mut var_mrev_d_db20: f64 = *var_mrev_d_db20_slot;
        let mut var_mrev_d_db21: f64 = *var_mrev_d_db21_slot;
        let mut var_mrev_d_db22: f64 = *var_mrev_d_db22_slot;
        let mut var_mrev_d_db23: f64 = *var_mrev_d_db23_slot;
        let mut var_mrev_d_db24: f64 = *var_mrev_d_db24_slot;
        let mut var_mrev_d_db3: f64 = *var_mrev_d_db3_slot;
        let mut var_mrev_d_db4: f64 = *var_mrev_d_db4_slot;
        let mut var_mrev_d_db5: f64 = *var_mrev_d_db5_slot;
        let mut var_mrev_d_db6: f64 = *var_mrev_d_db6_slot;
        let mut var_mrev_d_db7: f64 = *var_mrev_d_db7_slot;
        let mut var_mrev_d_db8: f64 = *var_mrev_d_db8_slot;
        let mut var_mrev_d_db9: f64 = *var_mrev_d_db9_slot;
        let mut var_mrev_d_dn0: f64 = *var_mrev_d_dn0_slot;
        let mut var_mrev_d_dn1: f64 = *var_mrev_d_dn1_slot;
        let mut var_mrev_d_dn10: f64 = *var_mrev_d_dn10_slot;
        let mut var_mrev_d_dn11: f64 = *var_mrev_d_dn11_slot;
        let mut var_mrev_d_dn12: f64 = *var_mrev_d_dn12_slot;
        let mut var_mrev_d_dn13: f64 = *var_mrev_d_dn13_slot;
        let mut var_mrev_d_dn14: f64 = *var_mrev_d_dn14_slot;
        let mut var_mrev_d_dn15: f64 = *var_mrev_d_dn15_slot;
        let mut var_mrev_d_dn16: f64 = *var_mrev_d_dn16_slot;
        let mut var_mrev_d_dn17: f64 = *var_mrev_d_dn17_slot;
        let mut var_mrev_d_dn18: f64 = *var_mrev_d_dn18_slot;
        let mut var_mrev_d_dn19: f64 = *var_mrev_d_dn19_slot;
        let mut var_mrev_d_dn2: f64 = *var_mrev_d_dn2_slot;
        let mut var_mrev_d_dn20: f64 = *var_mrev_d_dn20_slot;
        let mut var_mrev_d_dn3: f64 = *var_mrev_d_dn3_slot;
        let mut var_mrev_d_dn4: f64 = *var_mrev_d_dn4_slot;
        let mut var_mrev_d_dn5: f64 = *var_mrev_d_dn5_slot;
        let mut var_mrev_d_dn6: f64 = *var_mrev_d_dn6_slot;
        let mut var_mrev_d_dn7: f64 = *var_mrev_d_dn7_slot;
        let mut var_mrev_d_dn8: f64 = *var_mrev_d_dn8_slot;
        let mut var_mrev_d_dn9: f64 = *var_mrev_d_dn9_slot;
        let mut var_mrev_s: f64 = *var_mrev_s_slot;
        let mut var_mrev_s_db0: f64 = *var_mrev_s_db0_slot;
        let mut var_mrev_s_db1: f64 = *var_mrev_s_db1_slot;
        let mut var_mrev_s_db10: f64 = *var_mrev_s_db10_slot;
        let mut var_mrev_s_db11: f64 = *var_mrev_s_db11_slot;
        let mut var_mrev_s_db12: f64 = *var_mrev_s_db12_slot;
        let mut var_mrev_s_db13: f64 = *var_mrev_s_db13_slot;
        let mut var_mrev_s_db14: f64 = *var_mrev_s_db14_slot;
        let mut var_mrev_s_db15: f64 = *var_mrev_s_db15_slot;
        let mut var_mrev_s_db16: f64 = *var_mrev_s_db16_slot;
        let mut var_mrev_s_db17: f64 = *var_mrev_s_db17_slot;
        let mut var_mrev_s_db18: f64 = *var_mrev_s_db18_slot;
        let mut var_mrev_s_db19: f64 = *var_mrev_s_db19_slot;
        let mut var_mrev_s_db2: f64 = *var_mrev_s_db2_slot;
        let mut var_mrev_s_db20: f64 = *var_mrev_s_db20_slot;
        let mut var_mrev_s_db21: f64 = *var_mrev_s_db21_slot;
        let mut var_mrev_s_db22: f64 = *var_mrev_s_db22_slot;
        let mut var_mrev_s_db23: f64 = *var_mrev_s_db23_slot;
        let mut var_mrev_s_db24: f64 = *var_mrev_s_db24_slot;
        let mut var_mrev_s_db3: f64 = *var_mrev_s_db3_slot;
        let mut var_mrev_s_db4: f64 = *var_mrev_s_db4_slot;
        let mut var_mrev_s_db5: f64 = *var_mrev_s_db5_slot;
        let mut var_mrev_s_db6: f64 = *var_mrev_s_db6_slot;
        let mut var_mrev_s_db7: f64 = *var_mrev_s_db7_slot;
        let mut var_mrev_s_db8: f64 = *var_mrev_s_db8_slot;
        let mut var_mrev_s_db9: f64 = *var_mrev_s_db9_slot;
        let mut var_mrev_s_dn0: f64 = *var_mrev_s_dn0_slot;
        let mut var_mrev_s_dn1: f64 = *var_mrev_s_dn1_slot;
        let mut var_mrev_s_dn10: f64 = *var_mrev_s_dn10_slot;
        let mut var_mrev_s_dn11: f64 = *var_mrev_s_dn11_slot;
        let mut var_mrev_s_dn12: f64 = *var_mrev_s_dn12_slot;
        let mut var_mrev_s_dn13: f64 = *var_mrev_s_dn13_slot;
        let mut var_mrev_s_dn14: f64 = *var_mrev_s_dn14_slot;
        let mut var_mrev_s_dn15: f64 = *var_mrev_s_dn15_slot;
        let mut var_mrev_s_dn16: f64 = *var_mrev_s_dn16_slot;
        let mut var_mrev_s_dn17: f64 = *var_mrev_s_dn17_slot;
        let mut var_mrev_s_dn18: f64 = *var_mrev_s_dn18_slot;
        let mut var_mrev_s_dn19: f64 = *var_mrev_s_dn19_slot;
        let mut var_mrev_s_dn2: f64 = *var_mrev_s_dn2_slot;
        let mut var_mrev_s_dn20: f64 = *var_mrev_s_dn20_slot;
        let mut var_mrev_s_dn3: f64 = *var_mrev_s_dn3_slot;
        let mut var_mrev_s_dn4: f64 = *var_mrev_s_dn4_slot;
        let mut var_mrev_s_dn5: f64 = *var_mrev_s_dn5_slot;
        let mut var_mrev_s_dn6: f64 = *var_mrev_s_dn6_slot;
        let mut var_mrev_s_dn7: f64 = *var_mrev_s_dn7_slot;
        let mut var_mrev_s_dn8: f64 = *var_mrev_s_dn8_slot;
        let mut var_mrev_s_dn9: f64 = *var_mrev_s_dn9_slot;
        let mut var_xhighf1_d: f64 = *var_xhighf1_d_slot;
        let mut var_xhighf1_s: f64 = *var_xhighf1_s_slot;
        let mut var_xhighf2_d: f64 = *var_xhighf2_d_slot;
        let mut var_xhighf2_d_db0: f64 = *var_xhighf2_d_db0_slot;
        let mut var_xhighf2_d_db1: f64 = *var_xhighf2_d_db1_slot;
        let mut var_xhighf2_d_db10: f64 = *var_xhighf2_d_db10_slot;
        let mut var_xhighf2_d_db11: f64 = *var_xhighf2_d_db11_slot;
        let mut var_xhighf2_d_db12: f64 = *var_xhighf2_d_db12_slot;
        let mut var_xhighf2_d_db13: f64 = *var_xhighf2_d_db13_slot;
        let mut var_xhighf2_d_db14: f64 = *var_xhighf2_d_db14_slot;
        let mut var_xhighf2_d_db15: f64 = *var_xhighf2_d_db15_slot;
        let mut var_xhighf2_d_db16: f64 = *var_xhighf2_d_db16_slot;
        let mut var_xhighf2_d_db17: f64 = *var_xhighf2_d_db17_slot;
        let mut var_xhighf2_d_db18: f64 = *var_xhighf2_d_db18_slot;
        let mut var_xhighf2_d_db19: f64 = *var_xhighf2_d_db19_slot;
        let mut var_xhighf2_d_db2: f64 = *var_xhighf2_d_db2_slot;
        let mut var_xhighf2_d_db20: f64 = *var_xhighf2_d_db20_slot;
        let mut var_xhighf2_d_db21: f64 = *var_xhighf2_d_db21_slot;
        let mut var_xhighf2_d_db22: f64 = *var_xhighf2_d_db22_slot;
        let mut var_xhighf2_d_db23: f64 = *var_xhighf2_d_db23_slot;
        let mut var_xhighf2_d_db24: f64 = *var_xhighf2_d_db24_slot;
        let mut var_xhighf2_d_db3: f64 = *var_xhighf2_d_db3_slot;
        let mut var_xhighf2_d_db4: f64 = *var_xhighf2_d_db4_slot;
        let mut var_xhighf2_d_db5: f64 = *var_xhighf2_d_db5_slot;
        let mut var_xhighf2_d_db6: f64 = *var_xhighf2_d_db6_slot;
        let mut var_xhighf2_d_db7: f64 = *var_xhighf2_d_db7_slot;
        let mut var_xhighf2_d_db8: f64 = *var_xhighf2_d_db8_slot;
        let mut var_xhighf2_d_db9: f64 = *var_xhighf2_d_db9_slot;
        let mut var_xhighf2_d_dn0: f64 = *var_xhighf2_d_dn0_slot;
        let mut var_xhighf2_d_dn1: f64 = *var_xhighf2_d_dn1_slot;
        let mut var_xhighf2_d_dn10: f64 = *var_xhighf2_d_dn10_slot;
        let mut var_xhighf2_d_dn11: f64 = *var_xhighf2_d_dn11_slot;
        let mut var_xhighf2_d_dn12: f64 = *var_xhighf2_d_dn12_slot;
        let mut var_xhighf2_d_dn13: f64 = *var_xhighf2_d_dn13_slot;
        let mut var_xhighf2_d_dn14: f64 = *var_xhighf2_d_dn14_slot;
        let mut var_xhighf2_d_dn15: f64 = *var_xhighf2_d_dn15_slot;
        let mut var_xhighf2_d_dn16: f64 = *var_xhighf2_d_dn16_slot;
        let mut var_xhighf2_d_dn17: f64 = *var_xhighf2_d_dn17_slot;
        let mut var_xhighf2_d_dn18: f64 = *var_xhighf2_d_dn18_slot;
        let mut var_xhighf2_d_dn19: f64 = *var_xhighf2_d_dn19_slot;
        let mut var_xhighf2_d_dn2: f64 = *var_xhighf2_d_dn2_slot;
        let mut var_xhighf2_d_dn20: f64 = *var_xhighf2_d_dn20_slot;
        let mut var_xhighf2_d_dn3: f64 = *var_xhighf2_d_dn3_slot;
        let mut var_xhighf2_d_dn4: f64 = *var_xhighf2_d_dn4_slot;
        let mut var_xhighf2_d_dn5: f64 = *var_xhighf2_d_dn5_slot;
        let mut var_xhighf2_d_dn6: f64 = *var_xhighf2_d_dn6_slot;
        let mut var_xhighf2_d_dn7: f64 = *var_xhighf2_d_dn7_slot;
        let mut var_xhighf2_d_dn8: f64 = *var_xhighf2_d_dn8_slot;
        let mut var_xhighf2_d_dn9: f64 = *var_xhighf2_d_dn9_slot;
        let mut var_xhighf2_s: f64 = *var_xhighf2_s_slot;
        let mut var_xhighf2_s_db0: f64 = *var_xhighf2_s_db0_slot;
        let mut var_xhighf2_s_db1: f64 = *var_xhighf2_s_db1_slot;
        let mut var_xhighf2_s_db10: f64 = *var_xhighf2_s_db10_slot;
        let mut var_xhighf2_s_db11: f64 = *var_xhighf2_s_db11_slot;
        let mut var_xhighf2_s_db12: f64 = *var_xhighf2_s_db12_slot;
        let mut var_xhighf2_s_db13: f64 = *var_xhighf2_s_db13_slot;
        let mut var_xhighf2_s_db14: f64 = *var_xhighf2_s_db14_slot;
        let mut var_xhighf2_s_db15: f64 = *var_xhighf2_s_db15_slot;
        let mut var_xhighf2_s_db16: f64 = *var_xhighf2_s_db16_slot;
        let mut var_xhighf2_s_db17: f64 = *var_xhighf2_s_db17_slot;
        let mut var_xhighf2_s_db18: f64 = *var_xhighf2_s_db18_slot;
        let mut var_xhighf2_s_db19: f64 = *var_xhighf2_s_db19_slot;
        let mut var_xhighf2_s_db2: f64 = *var_xhighf2_s_db2_slot;
        let mut var_xhighf2_s_db20: f64 = *var_xhighf2_s_db20_slot;
        let mut var_xhighf2_s_db21: f64 = *var_xhighf2_s_db21_slot;
        let mut var_xhighf2_s_db22: f64 = *var_xhighf2_s_db22_slot;
        let mut var_xhighf2_s_db23: f64 = *var_xhighf2_s_db23_slot;
        let mut var_xhighf2_s_db24: f64 = *var_xhighf2_s_db24_slot;
        let mut var_xhighf2_s_db3: f64 = *var_xhighf2_s_db3_slot;
        let mut var_xhighf2_s_db4: f64 = *var_xhighf2_s_db4_slot;
        let mut var_xhighf2_s_db5: f64 = *var_xhighf2_s_db5_slot;
        let mut var_xhighf2_s_db6: f64 = *var_xhighf2_s_db6_slot;
        let mut var_xhighf2_s_db7: f64 = *var_xhighf2_s_db7_slot;
        let mut var_xhighf2_s_db8: f64 = *var_xhighf2_s_db8_slot;
        let mut var_xhighf2_s_db9: f64 = *var_xhighf2_s_db9_slot;
        let mut var_xhighf2_s_dn0: f64 = *var_xhighf2_s_dn0_slot;
        let mut var_xhighf2_s_dn1: f64 = *var_xhighf2_s_dn1_slot;
        let mut var_xhighf2_s_dn10: f64 = *var_xhighf2_s_dn10_slot;
        let mut var_xhighf2_s_dn11: f64 = *var_xhighf2_s_dn11_slot;
        let mut var_xhighf2_s_dn12: f64 = *var_xhighf2_s_dn12_slot;
        let mut var_xhighf2_s_dn13: f64 = *var_xhighf2_s_dn13_slot;
        let mut var_xhighf2_s_dn14: f64 = *var_xhighf2_s_dn14_slot;
        let mut var_xhighf2_s_dn15: f64 = *var_xhighf2_s_dn15_slot;
        let mut var_xhighf2_s_dn16: f64 = *var_xhighf2_s_dn16_slot;
        let mut var_xhighf2_s_dn17: f64 = *var_xhighf2_s_dn17_slot;
        let mut var_xhighf2_s_dn18: f64 = *var_xhighf2_s_dn18_slot;
        let mut var_xhighf2_s_dn19: f64 = *var_xhighf2_s_dn19_slot;
        let mut var_xhighf2_s_dn2: f64 = *var_xhighf2_s_dn2_slot;
        let mut var_xhighf2_s_dn20: f64 = *var_xhighf2_s_dn20_slot;
        let mut var_xhighf2_s_dn3: f64 = *var_xhighf2_s_dn3_slot;
        let mut var_xhighf2_s_dn4: f64 = *var_xhighf2_s_dn4_slot;
        let mut var_xhighf2_s_dn5: f64 = *var_xhighf2_s_dn5_slot;
        let mut var_xhighf2_s_dn6: f64 = *var_xhighf2_s_dn6_slot;
        let mut var_xhighf2_s_dn7: f64 = *var_xhighf2_s_dn7_slot;
        let mut var_xhighf2_s_dn8: f64 = *var_xhighf2_s_dn8_slot;
        let mut var_xhighf2_s_dn9: f64 = *var_xhighf2_s_dn9_slot;
        let mut var_xhighr_d: f64 = *var_xhighr_d_slot;
        let mut var_xhighr_d_db0: f64 = *var_xhighr_d_db0_slot;
        let mut var_xhighr_d_db1: f64 = *var_xhighr_d_db1_slot;
        let mut var_xhighr_d_db10: f64 = *var_xhighr_d_db10_slot;
        let mut var_xhighr_d_db11: f64 = *var_xhighr_d_db11_slot;
        let mut var_xhighr_d_db12: f64 = *var_xhighr_d_db12_slot;
        let mut var_xhighr_d_db13: f64 = *var_xhighr_d_db13_slot;
        let mut var_xhighr_d_db14: f64 = *var_xhighr_d_db14_slot;
        let mut var_xhighr_d_db15: f64 = *var_xhighr_d_db15_slot;
        let mut var_xhighr_d_db16: f64 = *var_xhighr_d_db16_slot;
        let mut var_xhighr_d_db17: f64 = *var_xhighr_d_db17_slot;
        let mut var_xhighr_d_db18: f64 = *var_xhighr_d_db18_slot;
        let mut var_xhighr_d_db19: f64 = *var_xhighr_d_db19_slot;
        let mut var_xhighr_d_db2: f64 = *var_xhighr_d_db2_slot;
        let mut var_xhighr_d_db20: f64 = *var_xhighr_d_db20_slot;
        let mut var_xhighr_d_db21: f64 = *var_xhighr_d_db21_slot;
        let mut var_xhighr_d_db22: f64 = *var_xhighr_d_db22_slot;
        let mut var_xhighr_d_db23: f64 = *var_xhighr_d_db23_slot;
        let mut var_xhighr_d_db24: f64 = *var_xhighr_d_db24_slot;
        let mut var_xhighr_d_db3: f64 = *var_xhighr_d_db3_slot;
        let mut var_xhighr_d_db4: f64 = *var_xhighr_d_db4_slot;
        let mut var_xhighr_d_db5: f64 = *var_xhighr_d_db5_slot;
        let mut var_xhighr_d_db6: f64 = *var_xhighr_d_db6_slot;
        let mut var_xhighr_d_db7: f64 = *var_xhighr_d_db7_slot;
        let mut var_xhighr_d_db8: f64 = *var_xhighr_d_db8_slot;
        let mut var_xhighr_d_db9: f64 = *var_xhighr_d_db9_slot;
        let mut var_xhighr_d_dn0: f64 = *var_xhighr_d_dn0_slot;
        let mut var_xhighr_d_dn1: f64 = *var_xhighr_d_dn1_slot;
        let mut var_xhighr_d_dn10: f64 = *var_xhighr_d_dn10_slot;
        let mut var_xhighr_d_dn11: f64 = *var_xhighr_d_dn11_slot;
        let mut var_xhighr_d_dn12: f64 = *var_xhighr_d_dn12_slot;
        let mut var_xhighr_d_dn13: f64 = *var_xhighr_d_dn13_slot;
        let mut var_xhighr_d_dn14: f64 = *var_xhighr_d_dn14_slot;
        let mut var_xhighr_d_dn15: f64 = *var_xhighr_d_dn15_slot;
        let mut var_xhighr_d_dn16: f64 = *var_xhighr_d_dn16_slot;
        let mut var_xhighr_d_dn17: f64 = *var_xhighr_d_dn17_slot;
        let mut var_xhighr_d_dn18: f64 = *var_xhighr_d_dn18_slot;
        let mut var_xhighr_d_dn19: f64 = *var_xhighr_d_dn19_slot;
        let mut var_xhighr_d_dn2: f64 = *var_xhighr_d_dn2_slot;
        let mut var_xhighr_d_dn20: f64 = *var_xhighr_d_dn20_slot;
        let mut var_xhighr_d_dn3: f64 = *var_xhighr_d_dn3_slot;
        let mut var_xhighr_d_dn4: f64 = *var_xhighr_d_dn4_slot;
        let mut var_xhighr_d_dn5: f64 = *var_xhighr_d_dn5_slot;
        let mut var_xhighr_d_dn6: f64 = *var_xhighr_d_dn6_slot;
        let mut var_xhighr_d_dn7: f64 = *var_xhighr_d_dn7_slot;
        let mut var_xhighr_d_dn8: f64 = *var_xhighr_d_dn8_slot;
        let mut var_xhighr_d_dn9: f64 = *var_xhighr_d_dn9_slot;
        let mut var_xhighr_s: f64 = *var_xhighr_s_slot;
        let mut var_xhighr_s_db0: f64 = *var_xhighr_s_db0_slot;
        let mut var_xhighr_s_db1: f64 = *var_xhighr_s_db1_slot;
        let mut var_xhighr_s_db10: f64 = *var_xhighr_s_db10_slot;
        let mut var_xhighr_s_db11: f64 = *var_xhighr_s_db11_slot;
        let mut var_xhighr_s_db12: f64 = *var_xhighr_s_db12_slot;
        let mut var_xhighr_s_db13: f64 = *var_xhighr_s_db13_slot;
        let mut var_xhighr_s_db14: f64 = *var_xhighr_s_db14_slot;
        let mut var_xhighr_s_db15: f64 = *var_xhighr_s_db15_slot;
        let mut var_xhighr_s_db16: f64 = *var_xhighr_s_db16_slot;
        let mut var_xhighr_s_db17: f64 = *var_xhighr_s_db17_slot;
        let mut var_xhighr_s_db18: f64 = *var_xhighr_s_db18_slot;
        let mut var_xhighr_s_db19: f64 = *var_xhighr_s_db19_slot;
        let mut var_xhighr_s_db2: f64 = *var_xhighr_s_db2_slot;
        let mut var_xhighr_s_db20: f64 = *var_xhighr_s_db20_slot;
        let mut var_xhighr_s_db21: f64 = *var_xhighr_s_db21_slot;
        let mut var_xhighr_s_db22: f64 = *var_xhighr_s_db22_slot;
        let mut var_xhighr_s_db23: f64 = *var_xhighr_s_db23_slot;
        let mut var_xhighr_s_db24: f64 = *var_xhighr_s_db24_slot;
        let mut var_xhighr_s_db3: f64 = *var_xhighr_s_db3_slot;
        let mut var_xhighr_s_db4: f64 = *var_xhighr_s_db4_slot;
        let mut var_xhighr_s_db5: f64 = *var_xhighr_s_db5_slot;
        let mut var_xhighr_s_db6: f64 = *var_xhighr_s_db6_slot;
        let mut var_xhighr_s_db7: f64 = *var_xhighr_s_db7_slot;
        let mut var_xhighr_s_db8: f64 = *var_xhighr_s_db8_slot;
        let mut var_xhighr_s_db9: f64 = *var_xhighr_s_db9_slot;
        let mut var_xhighr_s_dn0: f64 = *var_xhighr_s_dn0_slot;
        let mut var_xhighr_s_dn1: f64 = *var_xhighr_s_dn1_slot;
        let mut var_xhighr_s_dn10: f64 = *var_xhighr_s_dn10_slot;
        let mut var_xhighr_s_dn11: f64 = *var_xhighr_s_dn11_slot;
        let mut var_xhighr_s_dn12: f64 = *var_xhighr_s_dn12_slot;
        let mut var_xhighr_s_dn13: f64 = *var_xhighr_s_dn13_slot;
        let mut var_xhighr_s_dn14: f64 = *var_xhighr_s_dn14_slot;
        let mut var_xhighr_s_dn15: f64 = *var_xhighr_s_dn15_slot;
        let mut var_xhighr_s_dn16: f64 = *var_xhighr_s_dn16_slot;
        let mut var_xhighr_s_dn17: f64 = *var_xhighr_s_dn17_slot;
        let mut var_xhighr_s_dn18: f64 = *var_xhighr_s_dn18_slot;
        let mut var_xhighr_s_dn19: f64 = *var_xhighr_s_dn19_slot;
        let mut var_xhighr_s_dn2: f64 = *var_xhighr_s_dn2_slot;
        let mut var_xhighr_s_dn20: f64 = *var_xhighr_s_dn20_slot;
        let mut var_xhighr_s_dn3: f64 = *var_xhighr_s_dn3_slot;
        let mut var_xhighr_s_dn4: f64 = *var_xhighr_s_dn4_slot;
        let mut var_xhighr_s_dn5: f64 = *var_xhighr_s_dn5_slot;
        let mut var_xhighr_s_dn6: f64 = *var_xhighr_s_dn6_slot;
        let mut var_xhighr_s_dn7: f64 = *var_xhighr_s_dn7_slot;
        let mut var_xhighr_s_dn8: f64 = *var_xhighr_s_dn8_slot;
        let mut var_xhighr_s_dn9: f64 = *var_xhighr_s_dn9_slot;

        var_isatrev_s = 0.0;
        var_isatrev_s_dn0 = 0.0;
        var_isatrev_s_dn1 = 0.0;
        var_isatrev_s_dn2 = 0.0;
        var_isatrev_s_dn3 = 0.0;
        var_isatrev_s_dn4 = 0.0;
        var_isatrev_s_dn5 = 0.0;
        var_isatrev_s_dn6 = 0.0;
        var_isatrev_s_dn7 = 0.0;
        var_isatrev_s_dn8 = 0.0;
        var_isatrev_s_dn9 = 0.0;
        var_isatrev_s_dn10 = 0.0;
        var_isatrev_s_dn11 = 0.0;
        var_isatrev_s_dn12 = 0.0;
        var_isatrev_s_dn13 = 0.0;
        var_isatrev_s_dn14 = 0.0;
        var_isatrev_s_dn15 = 0.0;
        var_isatrev_s_dn16 = 0.0;
        var_isatrev_s_dn17 = 0.0;
        var_isatrev_s_dn18 = 0.0;
        var_isatrev_s_dn19 = 0.0;
        var_isatrev_s_dn20 = 0.0;
        var_isatrev_s_db0 = 0.0;
        var_isatrev_s_db1 = 0.0;
        var_isatrev_s_db2 = 0.0;
        var_isatrev_s_db3 = 0.0;
        var_isatrev_s_db4 = 0.0;
        var_isatrev_s_db5 = 0.0;
        var_isatrev_s_db6 = 0.0;
        var_isatrev_s_db7 = 0.0;
        var_isatrev_s_db8 = 0.0;
        var_isatrev_s_db9 = 0.0;
        var_isatrev_s_db10 = 0.0;
        var_isatrev_s_db11 = 0.0;
        var_isatrev_s_db12 = 0.0;
        var_isatrev_s_db13 = 0.0;
        var_isatrev_s_db14 = 0.0;
        var_isatrev_s_db15 = 0.0;
        var_isatrev_s_db16 = 0.0;
        var_isatrev_s_db17 = 0.0;
        var_isatrev_s_db18 = 0.0;
        var_isatrev_s_db19 = 0.0;
        var_isatrev_s_db20 = 0.0;
        var_isatrev_s_db21 = 0.0;
        var_isatrev_s_db22 = 0.0;
        var_isatrev_s_db23 = 0.0;
        var_isatrev_s_db24 = 0.0;

        var_isatrev_d = 0.0;
        var_isatrev_d_dn0 = 0.0;
        var_isatrev_d_dn1 = 0.0;
        var_isatrev_d_dn2 = 0.0;
        var_isatrev_d_dn3 = 0.0;
        var_isatrev_d_dn4 = 0.0;
        var_isatrev_d_dn5 = 0.0;
        var_isatrev_d_dn6 = 0.0;
        var_isatrev_d_dn7 = 0.0;
        var_isatrev_d_dn8 = 0.0;
        var_isatrev_d_dn9 = 0.0;
        var_isatrev_d_dn10 = 0.0;
        var_isatrev_d_dn11 = 0.0;
        var_isatrev_d_dn12 = 0.0;
        var_isatrev_d_dn13 = 0.0;
        var_isatrev_d_dn14 = 0.0;
        var_isatrev_d_dn15 = 0.0;
        var_isatrev_d_dn16 = 0.0;
        var_isatrev_d_dn17 = 0.0;
        var_isatrev_d_dn18 = 0.0;
        var_isatrev_d_dn19 = 0.0;
        var_isatrev_d_dn20 = 0.0;
        var_isatrev_d_db0 = 0.0;
        var_isatrev_d_db1 = 0.0;
        var_isatrev_d_db2 = 0.0;
        var_isatrev_d_db3 = 0.0;
        var_isatrev_d_db4 = 0.0;
        var_isatrev_d_db5 = 0.0;
        var_isatrev_d_db6 = 0.0;
        var_isatrev_d_db7 = 0.0;
        var_isatrev_d_db8 = 0.0;
        var_isatrev_d_db9 = 0.0;
        var_isatrev_d_db10 = 0.0;
        var_isatrev_d_db11 = 0.0;
        var_isatrev_d_db12 = 0.0;
        var_isatrev_d_db13 = 0.0;
        var_isatrev_d_db14 = 0.0;
        var_isatrev_d_db15 = 0.0;
        var_isatrev_d_db16 = 0.0;
        var_isatrev_d_db17 = 0.0;
        var_isatrev_d_db18 = 0.0;
        var_isatrev_d_db19 = 0.0;
        var_isatrev_d_db20 = 0.0;
        var_isatrev_d_db21 = 0.0;
        var_isatrev_d_db22 = 0.0;
        var_isatrev_d_db23 = 0.0;
        var_isatrev_d_db24 = 0.0;

        var_mrev_s = 1.0;
        var_mrev_s_dn0 = 0.0;
        var_mrev_s_dn1 = 0.0;
        var_mrev_s_dn2 = 0.0;
        var_mrev_s_dn3 = 0.0;
        var_mrev_s_dn4 = 0.0;
        var_mrev_s_dn5 = 0.0;
        var_mrev_s_dn6 = 0.0;
        var_mrev_s_dn7 = 0.0;
        var_mrev_s_dn8 = 0.0;
        var_mrev_s_dn9 = 0.0;
        var_mrev_s_dn10 = 0.0;
        var_mrev_s_dn11 = 0.0;
        var_mrev_s_dn12 = 0.0;
        var_mrev_s_dn13 = 0.0;
        var_mrev_s_dn14 = 0.0;
        var_mrev_s_dn15 = 0.0;
        var_mrev_s_dn16 = 0.0;
        var_mrev_s_dn17 = 0.0;
        var_mrev_s_dn18 = 0.0;
        var_mrev_s_dn19 = 0.0;
        var_mrev_s_dn20 = 0.0;
        var_mrev_s_db0 = 0.0;
        var_mrev_s_db1 = 0.0;
        var_mrev_s_db2 = 0.0;
        var_mrev_s_db3 = 0.0;
        var_mrev_s_db4 = 0.0;
        var_mrev_s_db5 = 0.0;
        var_mrev_s_db6 = 0.0;
        var_mrev_s_db7 = 0.0;
        var_mrev_s_db8 = 0.0;
        var_mrev_s_db9 = 0.0;
        var_mrev_s_db10 = 0.0;
        var_mrev_s_db11 = 0.0;
        var_mrev_s_db12 = 0.0;
        var_mrev_s_db13 = 0.0;
        var_mrev_s_db14 = 0.0;
        var_mrev_s_db15 = 0.0;
        var_mrev_s_db16 = 0.0;
        var_mrev_s_db17 = 0.0;
        var_mrev_s_db18 = 0.0;
        var_mrev_s_db19 = 0.0;
        var_mrev_s_db20 = 0.0;
        var_mrev_s_db21 = 0.0;
        var_mrev_s_db22 = 0.0;
        var_mrev_s_db23 = 0.0;
        var_mrev_s_db24 = 0.0;

        var_mrev_d = 1.0;
        var_mrev_d_dn0 = 0.0;
        var_mrev_d_dn1 = 0.0;
        var_mrev_d_dn2 = 0.0;
        var_mrev_d_dn3 = 0.0;
        var_mrev_d_dn4 = 0.0;
        var_mrev_d_dn5 = 0.0;
        var_mrev_d_dn6 = 0.0;
        var_mrev_d_dn7 = 0.0;
        var_mrev_d_dn8 = 0.0;
        var_mrev_d_dn9 = 0.0;
        var_mrev_d_dn10 = 0.0;
        var_mrev_d_dn11 = 0.0;
        var_mrev_d_dn12 = 0.0;
        var_mrev_d_dn13 = 0.0;
        var_mrev_d_dn14 = 0.0;
        var_mrev_d_dn15 = 0.0;
        var_mrev_d_dn16 = 0.0;
        var_mrev_d_dn17 = 0.0;
        var_mrev_d_dn18 = 0.0;
        var_mrev_d_dn19 = 0.0;
        var_mrev_d_dn20 = 0.0;
        var_mrev_d_db0 = 0.0;
        var_mrev_d_db1 = 0.0;
        var_mrev_d_db2 = 0.0;
        var_mrev_d_db3 = 0.0;
        var_mrev_d_db4 = 0.0;
        var_mrev_d_db5 = 0.0;
        var_mrev_d_db6 = 0.0;
        var_mrev_d_db7 = 0.0;
        var_mrev_d_db8 = 0.0;
        var_mrev_d_db9 = 0.0;
        var_mrev_d_db10 = 0.0;
        var_mrev_d_db11 = 0.0;
        var_mrev_d_db12 = 0.0;
        var_mrev_d_db13 = 0.0;
        var_mrev_d_db14 = 0.0;
        var_mrev_d_db15 = 0.0;
        var_mrev_d_db16 = 0.0;
        var_mrev_d_db17 = 0.0;
        var_mrev_d_db18 = 0.0;
        var_mrev_d_db19 = 0.0;
        var_mrev_d_db20 = 0.0;
        var_mrev_d_db21 = 0.0;
        var_mrev_d_db22 = 0.0;
        var_mrev_d_db23 = 0.0;
        var_mrev_d_db24 = 0.0;

        var_m0flag_s = 0.0;

        var_m0flag_d = 0.0;

        var_xhighf1_s = 0.0;

        var_xhighf1_d = 0.0;

        var_expxhf1_s = 0.0;

        var_expxhf1_d = 0.0;

        var_xhighf2_s = 0.0;
        var_xhighf2_s_dn0 = 0.0;
        var_xhighf2_s_dn1 = 0.0;
        var_xhighf2_s_dn2 = 0.0;
        var_xhighf2_s_dn3 = 0.0;
        var_xhighf2_s_dn4 = 0.0;
        var_xhighf2_s_dn5 = 0.0;
        var_xhighf2_s_dn6 = 0.0;
        var_xhighf2_s_dn7 = 0.0;
        var_xhighf2_s_dn8 = 0.0;
        var_xhighf2_s_dn9 = 0.0;
        var_xhighf2_s_dn10 = 0.0;
        var_xhighf2_s_dn11 = 0.0;
        var_xhighf2_s_dn12 = 0.0;
        var_xhighf2_s_dn13 = 0.0;
        var_xhighf2_s_dn14 = 0.0;
        var_xhighf2_s_dn15 = 0.0;
        var_xhighf2_s_dn16 = 0.0;
        var_xhighf2_s_dn17 = 0.0;
        var_xhighf2_s_dn18 = 0.0;
        var_xhighf2_s_dn19 = 0.0;
        var_xhighf2_s_dn20 = 0.0;
        var_xhighf2_s_db0 = 0.0;
        var_xhighf2_s_db1 = 0.0;
        var_xhighf2_s_db2 = 0.0;
        var_xhighf2_s_db3 = 0.0;
        var_xhighf2_s_db4 = 0.0;
        var_xhighf2_s_db5 = 0.0;
        var_xhighf2_s_db6 = 0.0;
        var_xhighf2_s_db7 = 0.0;
        var_xhighf2_s_db8 = 0.0;
        var_xhighf2_s_db9 = 0.0;
        var_xhighf2_s_db10 = 0.0;
        var_xhighf2_s_db11 = 0.0;
        var_xhighf2_s_db12 = 0.0;
        var_xhighf2_s_db13 = 0.0;
        var_xhighf2_s_db14 = 0.0;
        var_xhighf2_s_db15 = 0.0;
        var_xhighf2_s_db16 = 0.0;
        var_xhighf2_s_db17 = 0.0;
        var_xhighf2_s_db18 = 0.0;
        var_xhighf2_s_db19 = 0.0;
        var_xhighf2_s_db20 = 0.0;
        var_xhighf2_s_db21 = 0.0;
        var_xhighf2_s_db22 = 0.0;
        var_xhighf2_s_db23 = 0.0;
        var_xhighf2_s_db24 = 0.0;

        var_xhighf2_d = 0.0;
        var_xhighf2_d_dn0 = 0.0;
        var_xhighf2_d_dn1 = 0.0;
        var_xhighf2_d_dn2 = 0.0;
        var_xhighf2_d_dn3 = 0.0;
        var_xhighf2_d_dn4 = 0.0;
        var_xhighf2_d_dn5 = 0.0;
        var_xhighf2_d_dn6 = 0.0;
        var_xhighf2_d_dn7 = 0.0;
        var_xhighf2_d_dn8 = 0.0;
        var_xhighf2_d_dn9 = 0.0;
        var_xhighf2_d_dn10 = 0.0;
        var_xhighf2_d_dn11 = 0.0;
        var_xhighf2_d_dn12 = 0.0;
        var_xhighf2_d_dn13 = 0.0;
        var_xhighf2_d_dn14 = 0.0;
        var_xhighf2_d_dn15 = 0.0;
        var_xhighf2_d_dn16 = 0.0;
        var_xhighf2_d_dn17 = 0.0;
        var_xhighf2_d_dn18 = 0.0;
        var_xhighf2_d_dn19 = 0.0;
        var_xhighf2_d_dn20 = 0.0;
        var_xhighf2_d_db0 = 0.0;
        var_xhighf2_d_db1 = 0.0;
        var_xhighf2_d_db2 = 0.0;
        var_xhighf2_d_db3 = 0.0;
        var_xhighf2_d_db4 = 0.0;
        var_xhighf2_d_db5 = 0.0;
        var_xhighf2_d_db6 = 0.0;
        var_xhighf2_d_db7 = 0.0;
        var_xhighf2_d_db8 = 0.0;
        var_xhighf2_d_db9 = 0.0;
        var_xhighf2_d_db10 = 0.0;
        var_xhighf2_d_db11 = 0.0;
        var_xhighf2_d_db12 = 0.0;
        var_xhighf2_d_db13 = 0.0;
        var_xhighf2_d_db14 = 0.0;
        var_xhighf2_d_db15 = 0.0;
        var_xhighf2_d_db16 = 0.0;
        var_xhighf2_d_db17 = 0.0;
        var_xhighf2_d_db18 = 0.0;
        var_xhighf2_d_db19 = 0.0;
        var_xhighf2_d_db20 = 0.0;
        var_xhighf2_d_db21 = 0.0;
        var_xhighf2_d_db22 = 0.0;
        var_xhighf2_d_db23 = 0.0;
        var_xhighf2_d_db24 = 0.0;

        var_expxhf2_s = 0.0;
        var_expxhf2_s_dn0 = 0.0;
        var_expxhf2_s_dn1 = 0.0;
        var_expxhf2_s_dn2 = 0.0;
        var_expxhf2_s_dn3 = 0.0;
        var_expxhf2_s_dn4 = 0.0;
        var_expxhf2_s_dn5 = 0.0;
        var_expxhf2_s_dn6 = 0.0;
        var_expxhf2_s_dn7 = 0.0;
        var_expxhf2_s_dn8 = 0.0;
        var_expxhf2_s_dn9 = 0.0;
        var_expxhf2_s_dn10 = 0.0;
        var_expxhf2_s_dn11 = 0.0;
        var_expxhf2_s_dn12 = 0.0;
        var_expxhf2_s_dn13 = 0.0;
        var_expxhf2_s_dn14 = 0.0;
        var_expxhf2_s_dn15 = 0.0;
        var_expxhf2_s_dn16 = 0.0;
        var_expxhf2_s_dn17 = 0.0;
        var_expxhf2_s_dn18 = 0.0;
        var_expxhf2_s_dn19 = 0.0;
        var_expxhf2_s_dn20 = 0.0;
        var_expxhf2_s_db0 = 0.0;
        var_expxhf2_s_db1 = 0.0;
        var_expxhf2_s_db2 = 0.0;
        var_expxhf2_s_db3 = 0.0;
        var_expxhf2_s_db4 = 0.0;
        var_expxhf2_s_db5 = 0.0;
        var_expxhf2_s_db6 = 0.0;
        var_expxhf2_s_db7 = 0.0;
        var_expxhf2_s_db8 = 0.0;
        var_expxhf2_s_db9 = 0.0;
        var_expxhf2_s_db10 = 0.0;
        var_expxhf2_s_db11 = 0.0;
        var_expxhf2_s_db12 = 0.0;
        var_expxhf2_s_db13 = 0.0;
        var_expxhf2_s_db14 = 0.0;
        var_expxhf2_s_db15 = 0.0;
        var_expxhf2_s_db16 = 0.0;
        var_expxhf2_s_db17 = 0.0;
        var_expxhf2_s_db18 = 0.0;
        var_expxhf2_s_db19 = 0.0;
        var_expxhf2_s_db20 = 0.0;
        var_expxhf2_s_db21 = 0.0;
        var_expxhf2_s_db22 = 0.0;
        var_expxhf2_s_db23 = 0.0;
        var_expxhf2_s_db24 = 0.0;

        var_expxhf2_d = 0.0;
        var_expxhf2_d_dn0 = 0.0;
        var_expxhf2_d_dn1 = 0.0;
        var_expxhf2_d_dn2 = 0.0;
        var_expxhf2_d_dn3 = 0.0;
        var_expxhf2_d_dn4 = 0.0;
        var_expxhf2_d_dn5 = 0.0;
        var_expxhf2_d_dn6 = 0.0;
        var_expxhf2_d_dn7 = 0.0;
        var_expxhf2_d_dn8 = 0.0;
        var_expxhf2_d_dn9 = 0.0;
        var_expxhf2_d_dn10 = 0.0;
        var_expxhf2_d_dn11 = 0.0;
        var_expxhf2_d_dn12 = 0.0;
        var_expxhf2_d_dn13 = 0.0;
        var_expxhf2_d_dn14 = 0.0;
        var_expxhf2_d_dn15 = 0.0;
        var_expxhf2_d_dn16 = 0.0;
        var_expxhf2_d_dn17 = 0.0;
        var_expxhf2_d_dn18 = 0.0;
        var_expxhf2_d_dn19 = 0.0;
        var_expxhf2_d_dn20 = 0.0;
        var_expxhf2_d_db0 = 0.0;
        var_expxhf2_d_db1 = 0.0;
        var_expxhf2_d_db2 = 0.0;
        var_expxhf2_d_db3 = 0.0;
        var_expxhf2_d_db4 = 0.0;
        var_expxhf2_d_db5 = 0.0;
        var_expxhf2_d_db6 = 0.0;
        var_expxhf2_d_db7 = 0.0;
        var_expxhf2_d_db8 = 0.0;
        var_expxhf2_d_db9 = 0.0;
        var_expxhf2_d_db10 = 0.0;
        var_expxhf2_d_db11 = 0.0;
        var_expxhf2_d_db12 = 0.0;
        var_expxhf2_d_db13 = 0.0;
        var_expxhf2_d_db14 = 0.0;
        var_expxhf2_d_db15 = 0.0;
        var_expxhf2_d_db16 = 0.0;
        var_expxhf2_d_db17 = 0.0;
        var_expxhf2_d_db18 = 0.0;
        var_expxhf2_d_db19 = 0.0;
        var_expxhf2_d_db20 = 0.0;
        var_expxhf2_d_db21 = 0.0;
        var_expxhf2_d_db22 = 0.0;
        var_expxhf2_d_db23 = 0.0;
        var_expxhf2_d_db24 = 0.0;

        var_xhighr_s = 0.0;
        var_xhighr_s_dn0 = 0.0;
        var_xhighr_s_dn1 = 0.0;
        var_xhighr_s_dn2 = 0.0;
        var_xhighr_s_dn3 = 0.0;
        var_xhighr_s_dn4 = 0.0;
        var_xhighr_s_dn5 = 0.0;
        var_xhighr_s_dn6 = 0.0;
        var_xhighr_s_dn7 = 0.0;
        var_xhighr_s_dn8 = 0.0;
        var_xhighr_s_dn9 = 0.0;
        var_xhighr_s_dn10 = 0.0;
        var_xhighr_s_dn11 = 0.0;
        var_xhighr_s_dn12 = 0.0;
        var_xhighr_s_dn13 = 0.0;
        var_xhighr_s_dn14 = 0.0;
        var_xhighr_s_dn15 = 0.0;
        var_xhighr_s_dn16 = 0.0;
        var_xhighr_s_dn17 = 0.0;
        var_xhighr_s_dn18 = 0.0;
        var_xhighr_s_dn19 = 0.0;
        var_xhighr_s_dn20 = 0.0;
        var_xhighr_s_db0 = 0.0;
        var_xhighr_s_db1 = 0.0;
        var_xhighr_s_db2 = 0.0;
        var_xhighr_s_db3 = 0.0;
        var_xhighr_s_db4 = 0.0;
        var_xhighr_s_db5 = 0.0;
        var_xhighr_s_db6 = 0.0;
        var_xhighr_s_db7 = 0.0;
        var_xhighr_s_db8 = 0.0;
        var_xhighr_s_db9 = 0.0;
        var_xhighr_s_db10 = 0.0;
        var_xhighr_s_db11 = 0.0;
        var_xhighr_s_db12 = 0.0;
        var_xhighr_s_db13 = 0.0;
        var_xhighr_s_db14 = 0.0;
        var_xhighr_s_db15 = 0.0;
        var_xhighr_s_db16 = 0.0;
        var_xhighr_s_db17 = 0.0;
        var_xhighr_s_db18 = 0.0;
        var_xhighr_s_db19 = 0.0;
        var_xhighr_s_db20 = 0.0;
        var_xhighr_s_db21 = 0.0;
        var_xhighr_s_db22 = 0.0;
        var_xhighr_s_db23 = 0.0;
        var_xhighr_s_db24 = 0.0;

        var_xhighr_d = 0.0;
        var_xhighr_d_dn0 = 0.0;
        var_xhighr_d_dn1 = 0.0;
        var_xhighr_d_dn2 = 0.0;
        var_xhighr_d_dn3 = 0.0;
        var_xhighr_d_dn4 = 0.0;
        var_xhighr_d_dn5 = 0.0;
        var_xhighr_d_dn6 = 0.0;
        var_xhighr_d_dn7 = 0.0;
        var_xhighr_d_dn8 = 0.0;
        var_xhighr_d_dn9 = 0.0;
        var_xhighr_d_dn10 = 0.0;
        var_xhighr_d_dn11 = 0.0;
        var_xhighr_d_dn12 = 0.0;
        var_xhighr_d_dn13 = 0.0;
        var_xhighr_d_dn14 = 0.0;
        var_xhighr_d_dn15 = 0.0;
        var_xhighr_d_dn16 = 0.0;
        var_xhighr_d_dn17 = 0.0;
        var_xhighr_d_dn18 = 0.0;
        var_xhighr_d_dn19 = 0.0;
        var_xhighr_d_dn20 = 0.0;
        var_xhighr_d_db0 = 0.0;
        var_xhighr_d_db1 = 0.0;
        var_xhighr_d_db2 = 0.0;
        var_xhighr_d_db3 = 0.0;
        var_xhighr_d_db4 = 0.0;
        var_xhighr_d_db5 = 0.0;
        var_xhighr_d_db6 = 0.0;
        var_xhighr_d_db7 = 0.0;
        var_xhighr_d_db8 = 0.0;
        var_xhighr_d_db9 = 0.0;
        var_xhighr_d_db10 = 0.0;
        var_xhighr_d_db11 = 0.0;
        var_xhighr_d_db12 = 0.0;
        var_xhighr_d_db13 = 0.0;
        var_xhighr_d_db14 = 0.0;
        var_xhighr_d_db15 = 0.0;
        var_xhighr_d_db16 = 0.0;
        var_xhighr_d_db17 = 0.0;
        var_xhighr_d_db18 = 0.0;
        var_xhighr_d_db19 = 0.0;
        var_xhighr_d_db20 = 0.0;
        var_xhighr_d_db21 = 0.0;
        var_xhighr_d_db22 = 0.0;
        var_xhighr_d_db23 = 0.0;
        var_xhighr_d_db24 = 0.0;

        *var_expxhf1_d_slot = var_expxhf1_d;
        *var_expxhf1_s_slot = var_expxhf1_s;
        *var_expxhf2_d_slot = var_expxhf2_d;
        *var_expxhf2_d_db0_slot = var_expxhf2_d_db0;
        *var_expxhf2_d_db1_slot = var_expxhf2_d_db1;
        *var_expxhf2_d_db10_slot = var_expxhf2_d_db10;
        *var_expxhf2_d_db11_slot = var_expxhf2_d_db11;
        *var_expxhf2_d_db12_slot = var_expxhf2_d_db12;
        *var_expxhf2_d_db13_slot = var_expxhf2_d_db13;
        *var_expxhf2_d_db14_slot = var_expxhf2_d_db14;
        *var_expxhf2_d_db15_slot = var_expxhf2_d_db15;
        *var_expxhf2_d_db16_slot = var_expxhf2_d_db16;
        *var_expxhf2_d_db17_slot = var_expxhf2_d_db17;
        *var_expxhf2_d_db18_slot = var_expxhf2_d_db18;
        *var_expxhf2_d_db19_slot = var_expxhf2_d_db19;
        *var_expxhf2_d_db2_slot = var_expxhf2_d_db2;
        *var_expxhf2_d_db20_slot = var_expxhf2_d_db20;
        *var_expxhf2_d_db21_slot = var_expxhf2_d_db21;
        *var_expxhf2_d_db22_slot = var_expxhf2_d_db22;
        *var_expxhf2_d_db23_slot = var_expxhf2_d_db23;
        *var_expxhf2_d_db24_slot = var_expxhf2_d_db24;
        *var_expxhf2_d_db3_slot = var_expxhf2_d_db3;
        *var_expxhf2_d_db4_slot = var_expxhf2_d_db4;
        *var_expxhf2_d_db5_slot = var_expxhf2_d_db5;
        *var_expxhf2_d_db6_slot = var_expxhf2_d_db6;
        *var_expxhf2_d_db7_slot = var_expxhf2_d_db7;
        *var_expxhf2_d_db8_slot = var_expxhf2_d_db8;
        *var_expxhf2_d_db9_slot = var_expxhf2_d_db9;
        *var_expxhf2_d_dn0_slot = var_expxhf2_d_dn0;
        *var_expxhf2_d_dn1_slot = var_expxhf2_d_dn1;
        *var_expxhf2_d_dn10_slot = var_expxhf2_d_dn10;
        *var_expxhf2_d_dn11_slot = var_expxhf2_d_dn11;
        *var_expxhf2_d_dn12_slot = var_expxhf2_d_dn12;
        *var_expxhf2_d_dn13_slot = var_expxhf2_d_dn13;
        *var_expxhf2_d_dn14_slot = var_expxhf2_d_dn14;
        *var_expxhf2_d_dn15_slot = var_expxhf2_d_dn15;
        *var_expxhf2_d_dn16_slot = var_expxhf2_d_dn16;
        *var_expxhf2_d_dn17_slot = var_expxhf2_d_dn17;
        *var_expxhf2_d_dn18_slot = var_expxhf2_d_dn18;
        *var_expxhf2_d_dn19_slot = var_expxhf2_d_dn19;
        *var_expxhf2_d_dn2_slot = var_expxhf2_d_dn2;
        *var_expxhf2_d_dn20_slot = var_expxhf2_d_dn20;
        *var_expxhf2_d_dn3_slot = var_expxhf2_d_dn3;
        *var_expxhf2_d_dn4_slot = var_expxhf2_d_dn4;
        *var_expxhf2_d_dn5_slot = var_expxhf2_d_dn5;
        *var_expxhf2_d_dn6_slot = var_expxhf2_d_dn6;
        *var_expxhf2_d_dn7_slot = var_expxhf2_d_dn7;
        *var_expxhf2_d_dn8_slot = var_expxhf2_d_dn8;
        *var_expxhf2_d_dn9_slot = var_expxhf2_d_dn9;
        *var_expxhf2_s_slot = var_expxhf2_s;
        *var_expxhf2_s_db0_slot = var_expxhf2_s_db0;
        *var_expxhf2_s_db1_slot = var_expxhf2_s_db1;
        *var_expxhf2_s_db10_slot = var_expxhf2_s_db10;
        *var_expxhf2_s_db11_slot = var_expxhf2_s_db11;
        *var_expxhf2_s_db12_slot = var_expxhf2_s_db12;
        *var_expxhf2_s_db13_slot = var_expxhf2_s_db13;
        *var_expxhf2_s_db14_slot = var_expxhf2_s_db14;
        *var_expxhf2_s_db15_slot = var_expxhf2_s_db15;
        *var_expxhf2_s_db16_slot = var_expxhf2_s_db16;
        *var_expxhf2_s_db17_slot = var_expxhf2_s_db17;
        *var_expxhf2_s_db18_slot = var_expxhf2_s_db18;
        *var_expxhf2_s_db19_slot = var_expxhf2_s_db19;
        *var_expxhf2_s_db2_slot = var_expxhf2_s_db2;
        *var_expxhf2_s_db20_slot = var_expxhf2_s_db20;
        *var_expxhf2_s_db21_slot = var_expxhf2_s_db21;
        *var_expxhf2_s_db22_slot = var_expxhf2_s_db22;
        *var_expxhf2_s_db23_slot = var_expxhf2_s_db23;
        *var_expxhf2_s_db24_slot = var_expxhf2_s_db24;
        *var_expxhf2_s_db3_slot = var_expxhf2_s_db3;
        *var_expxhf2_s_db4_slot = var_expxhf2_s_db4;
        *var_expxhf2_s_db5_slot = var_expxhf2_s_db5;
        *var_expxhf2_s_db6_slot = var_expxhf2_s_db6;
        *var_expxhf2_s_db7_slot = var_expxhf2_s_db7;
        *var_expxhf2_s_db8_slot = var_expxhf2_s_db8;
        *var_expxhf2_s_db9_slot = var_expxhf2_s_db9;
        *var_expxhf2_s_dn0_slot = var_expxhf2_s_dn0;
        *var_expxhf2_s_dn1_slot = var_expxhf2_s_dn1;
        *var_expxhf2_s_dn10_slot = var_expxhf2_s_dn10;
        *var_expxhf2_s_dn11_slot = var_expxhf2_s_dn11;
        *var_expxhf2_s_dn12_slot = var_expxhf2_s_dn12;
        *var_expxhf2_s_dn13_slot = var_expxhf2_s_dn13;
        *var_expxhf2_s_dn14_slot = var_expxhf2_s_dn14;
        *var_expxhf2_s_dn15_slot = var_expxhf2_s_dn15;
        *var_expxhf2_s_dn16_slot = var_expxhf2_s_dn16;
        *var_expxhf2_s_dn17_slot = var_expxhf2_s_dn17;
        *var_expxhf2_s_dn18_slot = var_expxhf2_s_dn18;
        *var_expxhf2_s_dn19_slot = var_expxhf2_s_dn19;
        *var_expxhf2_s_dn2_slot = var_expxhf2_s_dn2;
        *var_expxhf2_s_dn20_slot = var_expxhf2_s_dn20;
        *var_expxhf2_s_dn3_slot = var_expxhf2_s_dn3;
        *var_expxhf2_s_dn4_slot = var_expxhf2_s_dn4;
        *var_expxhf2_s_dn5_slot = var_expxhf2_s_dn5;
        *var_expxhf2_s_dn6_slot = var_expxhf2_s_dn6;
        *var_expxhf2_s_dn7_slot = var_expxhf2_s_dn7;
        *var_expxhf2_s_dn8_slot = var_expxhf2_s_dn8;
        *var_expxhf2_s_dn9_slot = var_expxhf2_s_dn9;
        *var_isatrev_d_slot = var_isatrev_d;
        *var_isatrev_d_db0_slot = var_isatrev_d_db0;
        *var_isatrev_d_db1_slot = var_isatrev_d_db1;
        *var_isatrev_d_db10_slot = var_isatrev_d_db10;
        *var_isatrev_d_db11_slot = var_isatrev_d_db11;
        *var_isatrev_d_db12_slot = var_isatrev_d_db12;
        *var_isatrev_d_db13_slot = var_isatrev_d_db13;
        *var_isatrev_d_db14_slot = var_isatrev_d_db14;
        *var_isatrev_d_db15_slot = var_isatrev_d_db15;
        *var_isatrev_d_db16_slot = var_isatrev_d_db16;
        *var_isatrev_d_db17_slot = var_isatrev_d_db17;
        *var_isatrev_d_db18_slot = var_isatrev_d_db18;
        *var_isatrev_d_db19_slot = var_isatrev_d_db19;
        *var_isatrev_d_db2_slot = var_isatrev_d_db2;
        *var_isatrev_d_db20_slot = var_isatrev_d_db20;
        *var_isatrev_d_db21_slot = var_isatrev_d_db21;
        *var_isatrev_d_db22_slot = var_isatrev_d_db22;
        *var_isatrev_d_db23_slot = var_isatrev_d_db23;
        *var_isatrev_d_db24_slot = var_isatrev_d_db24;
        *var_isatrev_d_db3_slot = var_isatrev_d_db3;
        *var_isatrev_d_db4_slot = var_isatrev_d_db4;
        *var_isatrev_d_db5_slot = var_isatrev_d_db5;
        *var_isatrev_d_db6_slot = var_isatrev_d_db6;
        *var_isatrev_d_db7_slot = var_isatrev_d_db7;
        *var_isatrev_d_db8_slot = var_isatrev_d_db8;
        *var_isatrev_d_db9_slot = var_isatrev_d_db9;
        *var_isatrev_d_dn0_slot = var_isatrev_d_dn0;
        *var_isatrev_d_dn1_slot = var_isatrev_d_dn1;
        *var_isatrev_d_dn10_slot = var_isatrev_d_dn10;
        *var_isatrev_d_dn11_slot = var_isatrev_d_dn11;
        *var_isatrev_d_dn12_slot = var_isatrev_d_dn12;
        *var_isatrev_d_dn13_slot = var_isatrev_d_dn13;
        *var_isatrev_d_dn14_slot = var_isatrev_d_dn14;
        *var_isatrev_d_dn15_slot = var_isatrev_d_dn15;
        *var_isatrev_d_dn16_slot = var_isatrev_d_dn16;
        *var_isatrev_d_dn17_slot = var_isatrev_d_dn17;
        *var_isatrev_d_dn18_slot = var_isatrev_d_dn18;
        *var_isatrev_d_dn19_slot = var_isatrev_d_dn19;
        *var_isatrev_d_dn2_slot = var_isatrev_d_dn2;
        *var_isatrev_d_dn20_slot = var_isatrev_d_dn20;
        *var_isatrev_d_dn3_slot = var_isatrev_d_dn3;
        *var_isatrev_d_dn4_slot = var_isatrev_d_dn4;
        *var_isatrev_d_dn5_slot = var_isatrev_d_dn5;
        *var_isatrev_d_dn6_slot = var_isatrev_d_dn6;
        *var_isatrev_d_dn7_slot = var_isatrev_d_dn7;
        *var_isatrev_d_dn8_slot = var_isatrev_d_dn8;
        *var_isatrev_d_dn9_slot = var_isatrev_d_dn9;
        *var_isatrev_s_slot = var_isatrev_s;
        *var_isatrev_s_db0_slot = var_isatrev_s_db0;
        *var_isatrev_s_db1_slot = var_isatrev_s_db1;
        *var_isatrev_s_db10_slot = var_isatrev_s_db10;
        *var_isatrev_s_db11_slot = var_isatrev_s_db11;
        *var_isatrev_s_db12_slot = var_isatrev_s_db12;
        *var_isatrev_s_db13_slot = var_isatrev_s_db13;
        *var_isatrev_s_db14_slot = var_isatrev_s_db14;
        *var_isatrev_s_db15_slot = var_isatrev_s_db15;
        *var_isatrev_s_db16_slot = var_isatrev_s_db16;
        *var_isatrev_s_db17_slot = var_isatrev_s_db17;
        *var_isatrev_s_db18_slot = var_isatrev_s_db18;
        *var_isatrev_s_db19_slot = var_isatrev_s_db19;
        *var_isatrev_s_db2_slot = var_isatrev_s_db2;
        *var_isatrev_s_db20_slot = var_isatrev_s_db20;
        *var_isatrev_s_db21_slot = var_isatrev_s_db21;
        *var_isatrev_s_db22_slot = var_isatrev_s_db22;
        *var_isatrev_s_db23_slot = var_isatrev_s_db23;
        *var_isatrev_s_db24_slot = var_isatrev_s_db24;
        *var_isatrev_s_db3_slot = var_isatrev_s_db3;
        *var_isatrev_s_db4_slot = var_isatrev_s_db4;
        *var_isatrev_s_db5_slot = var_isatrev_s_db5;
        *var_isatrev_s_db6_slot = var_isatrev_s_db6;
        *var_isatrev_s_db7_slot = var_isatrev_s_db7;
        *var_isatrev_s_db8_slot = var_isatrev_s_db8;
        *var_isatrev_s_db9_slot = var_isatrev_s_db9;
        *var_isatrev_s_dn0_slot = var_isatrev_s_dn0;
        *var_isatrev_s_dn1_slot = var_isatrev_s_dn1;
        *var_isatrev_s_dn10_slot = var_isatrev_s_dn10;
        *var_isatrev_s_dn11_slot = var_isatrev_s_dn11;
        *var_isatrev_s_dn12_slot = var_isatrev_s_dn12;
        *var_isatrev_s_dn13_slot = var_isatrev_s_dn13;
        *var_isatrev_s_dn14_slot = var_isatrev_s_dn14;
        *var_isatrev_s_dn15_slot = var_isatrev_s_dn15;
        *var_isatrev_s_dn16_slot = var_isatrev_s_dn16;
        *var_isatrev_s_dn17_slot = var_isatrev_s_dn17;
        *var_isatrev_s_dn18_slot = var_isatrev_s_dn18;
        *var_isatrev_s_dn19_slot = var_isatrev_s_dn19;
        *var_isatrev_s_dn2_slot = var_isatrev_s_dn2;
        *var_isatrev_s_dn20_slot = var_isatrev_s_dn20;
        *var_isatrev_s_dn3_slot = var_isatrev_s_dn3;
        *var_isatrev_s_dn4_slot = var_isatrev_s_dn4;
        *var_isatrev_s_dn5_slot = var_isatrev_s_dn5;
        *var_isatrev_s_dn6_slot = var_isatrev_s_dn6;
        *var_isatrev_s_dn7_slot = var_isatrev_s_dn7;
        *var_isatrev_s_dn8_slot = var_isatrev_s_dn8;
        *var_isatrev_s_dn9_slot = var_isatrev_s_dn9;
        *var_m0flag_d_slot = var_m0flag_d;
        *var_m0flag_s_slot = var_m0flag_s;
        *var_mrev_d_slot = var_mrev_d;
        *var_mrev_d_db0_slot = var_mrev_d_db0;
        *var_mrev_d_db1_slot = var_mrev_d_db1;
        *var_mrev_d_db10_slot = var_mrev_d_db10;
        *var_mrev_d_db11_slot = var_mrev_d_db11;
        *var_mrev_d_db12_slot = var_mrev_d_db12;
        *var_mrev_d_db13_slot = var_mrev_d_db13;
        *var_mrev_d_db14_slot = var_mrev_d_db14;
        *var_mrev_d_db15_slot = var_mrev_d_db15;
        *var_mrev_d_db16_slot = var_mrev_d_db16;
        *var_mrev_d_db17_slot = var_mrev_d_db17;
        *var_mrev_d_db18_slot = var_mrev_d_db18;
        *var_mrev_d_db19_slot = var_mrev_d_db19;
        *var_mrev_d_db2_slot = var_mrev_d_db2;
        *var_mrev_d_db20_slot = var_mrev_d_db20;
        *var_mrev_d_db21_slot = var_mrev_d_db21;
        *var_mrev_d_db22_slot = var_mrev_d_db22;
        *var_mrev_d_db23_slot = var_mrev_d_db23;
        *var_mrev_d_db24_slot = var_mrev_d_db24;
        *var_mrev_d_db3_slot = var_mrev_d_db3;
        *var_mrev_d_db4_slot = var_mrev_d_db4;
        *var_mrev_d_db5_slot = var_mrev_d_db5;
        *var_mrev_d_db6_slot = var_mrev_d_db6;
        *var_mrev_d_db7_slot = var_mrev_d_db7;
        *var_mrev_d_db8_slot = var_mrev_d_db8;
        *var_mrev_d_db9_slot = var_mrev_d_db9;
        *var_mrev_d_dn0_slot = var_mrev_d_dn0;
        *var_mrev_d_dn1_slot = var_mrev_d_dn1;
        *var_mrev_d_dn10_slot = var_mrev_d_dn10;
        *var_mrev_d_dn11_slot = var_mrev_d_dn11;
        *var_mrev_d_dn12_slot = var_mrev_d_dn12;
        *var_mrev_d_dn13_slot = var_mrev_d_dn13;
        *var_mrev_d_dn14_slot = var_mrev_d_dn14;
        *var_mrev_d_dn15_slot = var_mrev_d_dn15;
        *var_mrev_d_dn16_slot = var_mrev_d_dn16;
        *var_mrev_d_dn17_slot = var_mrev_d_dn17;
        *var_mrev_d_dn18_slot = var_mrev_d_dn18;
        *var_mrev_d_dn19_slot = var_mrev_d_dn19;
        *var_mrev_d_dn2_slot = var_mrev_d_dn2;
        *var_mrev_d_dn20_slot = var_mrev_d_dn20;
        *var_mrev_d_dn3_slot = var_mrev_d_dn3;
        *var_mrev_d_dn4_slot = var_mrev_d_dn4;
        *var_mrev_d_dn5_slot = var_mrev_d_dn5;
        *var_mrev_d_dn6_slot = var_mrev_d_dn6;
        *var_mrev_d_dn7_slot = var_mrev_d_dn7;
        *var_mrev_d_dn8_slot = var_mrev_d_dn8;
        *var_mrev_d_dn9_slot = var_mrev_d_dn9;
        *var_mrev_s_slot = var_mrev_s;
        *var_mrev_s_db0_slot = var_mrev_s_db0;
        *var_mrev_s_db1_slot = var_mrev_s_db1;
        *var_mrev_s_db10_slot = var_mrev_s_db10;
        *var_mrev_s_db11_slot = var_mrev_s_db11;
        *var_mrev_s_db12_slot = var_mrev_s_db12;
        *var_mrev_s_db13_slot = var_mrev_s_db13;
        *var_mrev_s_db14_slot = var_mrev_s_db14;
        *var_mrev_s_db15_slot = var_mrev_s_db15;
        *var_mrev_s_db16_slot = var_mrev_s_db16;
        *var_mrev_s_db17_slot = var_mrev_s_db17;
        *var_mrev_s_db18_slot = var_mrev_s_db18;
        *var_mrev_s_db19_slot = var_mrev_s_db19;
        *var_mrev_s_db2_slot = var_mrev_s_db2;
        *var_mrev_s_db20_slot = var_mrev_s_db20;
        *var_mrev_s_db21_slot = var_mrev_s_db21;
        *var_mrev_s_db22_slot = var_mrev_s_db22;
        *var_mrev_s_db23_slot = var_mrev_s_db23;
        *var_mrev_s_db24_slot = var_mrev_s_db24;
        *var_mrev_s_db3_slot = var_mrev_s_db3;
        *var_mrev_s_db4_slot = var_mrev_s_db4;
        *var_mrev_s_db5_slot = var_mrev_s_db5;
        *var_mrev_s_db6_slot = var_mrev_s_db6;
        *var_mrev_s_db7_slot = var_mrev_s_db7;
        *var_mrev_s_db8_slot = var_mrev_s_db8;
        *var_mrev_s_db9_slot = var_mrev_s_db9;
        *var_mrev_s_dn0_slot = var_mrev_s_dn0;
        *var_mrev_s_dn1_slot = var_mrev_s_dn1;
        *var_mrev_s_dn10_slot = var_mrev_s_dn10;
        *var_mrev_s_dn11_slot = var_mrev_s_dn11;
        *var_mrev_s_dn12_slot = var_mrev_s_dn12;
        *var_mrev_s_dn13_slot = var_mrev_s_dn13;
        *var_mrev_s_dn14_slot = var_mrev_s_dn14;
        *var_mrev_s_dn15_slot = var_mrev_s_dn15;
        *var_mrev_s_dn16_slot = var_mrev_s_dn16;
        *var_mrev_s_dn17_slot = var_mrev_s_dn17;
        *var_mrev_s_dn18_slot = var_mrev_s_dn18;
        *var_mrev_s_dn19_slot = var_mrev_s_dn19;
        *var_mrev_s_dn2_slot = var_mrev_s_dn2;
        *var_mrev_s_dn20_slot = var_mrev_s_dn20;
        *var_mrev_s_dn3_slot = var_mrev_s_dn3;
        *var_mrev_s_dn4_slot = var_mrev_s_dn4;
        *var_mrev_s_dn5_slot = var_mrev_s_dn5;
        *var_mrev_s_dn6_slot = var_mrev_s_dn6;
        *var_mrev_s_dn7_slot = var_mrev_s_dn7;
        *var_mrev_s_dn8_slot = var_mrev_s_dn8;
        *var_mrev_s_dn9_slot = var_mrev_s_dn9;
        *var_xhighf1_d_slot = var_xhighf1_d;
        *var_xhighf1_s_slot = var_xhighf1_s;
        *var_xhighf2_d_slot = var_xhighf2_d;
        *var_xhighf2_d_db0_slot = var_xhighf2_d_db0;
        *var_xhighf2_d_db1_slot = var_xhighf2_d_db1;
        *var_xhighf2_d_db10_slot = var_xhighf2_d_db10;
        *var_xhighf2_d_db11_slot = var_xhighf2_d_db11;
        *var_xhighf2_d_db12_slot = var_xhighf2_d_db12;
        *var_xhighf2_d_db13_slot = var_xhighf2_d_db13;
        *var_xhighf2_d_db14_slot = var_xhighf2_d_db14;
        *var_xhighf2_d_db15_slot = var_xhighf2_d_db15;
        *var_xhighf2_d_db16_slot = var_xhighf2_d_db16;
        *var_xhighf2_d_db17_slot = var_xhighf2_d_db17;
        *var_xhighf2_d_db18_slot = var_xhighf2_d_db18;
        *var_xhighf2_d_db19_slot = var_xhighf2_d_db19;
        *var_xhighf2_d_db2_slot = var_xhighf2_d_db2;
        *var_xhighf2_d_db20_slot = var_xhighf2_d_db20;
        *var_xhighf2_d_db21_slot = var_xhighf2_d_db21;
        *var_xhighf2_d_db22_slot = var_xhighf2_d_db22;
        *var_xhighf2_d_db23_slot = var_xhighf2_d_db23;
        *var_xhighf2_d_db24_slot = var_xhighf2_d_db24;
        *var_xhighf2_d_db3_slot = var_xhighf2_d_db3;
        *var_xhighf2_d_db4_slot = var_xhighf2_d_db4;
        *var_xhighf2_d_db5_slot = var_xhighf2_d_db5;
        *var_xhighf2_d_db6_slot = var_xhighf2_d_db6;
        *var_xhighf2_d_db7_slot = var_xhighf2_d_db7;
        *var_xhighf2_d_db8_slot = var_xhighf2_d_db8;
        *var_xhighf2_d_db9_slot = var_xhighf2_d_db9;
        *var_xhighf2_d_dn0_slot = var_xhighf2_d_dn0;
        *var_xhighf2_d_dn1_slot = var_xhighf2_d_dn1;
        *var_xhighf2_d_dn10_slot = var_xhighf2_d_dn10;
        *var_xhighf2_d_dn11_slot = var_xhighf2_d_dn11;
        *var_xhighf2_d_dn12_slot = var_xhighf2_d_dn12;
        *var_xhighf2_d_dn13_slot = var_xhighf2_d_dn13;
        *var_xhighf2_d_dn14_slot = var_xhighf2_d_dn14;
        *var_xhighf2_d_dn15_slot = var_xhighf2_d_dn15;
        *var_xhighf2_d_dn16_slot = var_xhighf2_d_dn16;
        *var_xhighf2_d_dn17_slot = var_xhighf2_d_dn17;
        *var_xhighf2_d_dn18_slot = var_xhighf2_d_dn18;
        *var_xhighf2_d_dn19_slot = var_xhighf2_d_dn19;
        *var_xhighf2_d_dn2_slot = var_xhighf2_d_dn2;
        *var_xhighf2_d_dn20_slot = var_xhighf2_d_dn20;
        *var_xhighf2_d_dn3_slot = var_xhighf2_d_dn3;
        *var_xhighf2_d_dn4_slot = var_xhighf2_d_dn4;
        *var_xhighf2_d_dn5_slot = var_xhighf2_d_dn5;
        *var_xhighf2_d_dn6_slot = var_xhighf2_d_dn6;
        *var_xhighf2_d_dn7_slot = var_xhighf2_d_dn7;
        *var_xhighf2_d_dn8_slot = var_xhighf2_d_dn8;
        *var_xhighf2_d_dn9_slot = var_xhighf2_d_dn9;
        *var_xhighf2_s_slot = var_xhighf2_s;
        *var_xhighf2_s_db0_slot = var_xhighf2_s_db0;
        *var_xhighf2_s_db1_slot = var_xhighf2_s_db1;
        *var_xhighf2_s_db10_slot = var_xhighf2_s_db10;
        *var_xhighf2_s_db11_slot = var_xhighf2_s_db11;
        *var_xhighf2_s_db12_slot = var_xhighf2_s_db12;
        *var_xhighf2_s_db13_slot = var_xhighf2_s_db13;
        *var_xhighf2_s_db14_slot = var_xhighf2_s_db14;
        *var_xhighf2_s_db15_slot = var_xhighf2_s_db15;
        *var_xhighf2_s_db16_slot = var_xhighf2_s_db16;
        *var_xhighf2_s_db17_slot = var_xhighf2_s_db17;
        *var_xhighf2_s_db18_slot = var_xhighf2_s_db18;
        *var_xhighf2_s_db19_slot = var_xhighf2_s_db19;
        *var_xhighf2_s_db2_slot = var_xhighf2_s_db2;
        *var_xhighf2_s_db20_slot = var_xhighf2_s_db20;
        *var_xhighf2_s_db21_slot = var_xhighf2_s_db21;
        *var_xhighf2_s_db22_slot = var_xhighf2_s_db22;
        *var_xhighf2_s_db23_slot = var_xhighf2_s_db23;
        *var_xhighf2_s_db24_slot = var_xhighf2_s_db24;
        *var_xhighf2_s_db3_slot = var_xhighf2_s_db3;
        *var_xhighf2_s_db4_slot = var_xhighf2_s_db4;
        *var_xhighf2_s_db5_slot = var_xhighf2_s_db5;
        *var_xhighf2_s_db6_slot = var_xhighf2_s_db6;
        *var_xhighf2_s_db7_slot = var_xhighf2_s_db7;
        *var_xhighf2_s_db8_slot = var_xhighf2_s_db8;
        *var_xhighf2_s_db9_slot = var_xhighf2_s_db9;
        *var_xhighf2_s_dn0_slot = var_xhighf2_s_dn0;
        *var_xhighf2_s_dn1_slot = var_xhighf2_s_dn1;
        *var_xhighf2_s_dn10_slot = var_xhighf2_s_dn10;
        *var_xhighf2_s_dn11_slot = var_xhighf2_s_dn11;
        *var_xhighf2_s_dn12_slot = var_xhighf2_s_dn12;
        *var_xhighf2_s_dn13_slot = var_xhighf2_s_dn13;
        *var_xhighf2_s_dn14_slot = var_xhighf2_s_dn14;
        *var_xhighf2_s_dn15_slot = var_xhighf2_s_dn15;
        *var_xhighf2_s_dn16_slot = var_xhighf2_s_dn16;
        *var_xhighf2_s_dn17_slot = var_xhighf2_s_dn17;
        *var_xhighf2_s_dn18_slot = var_xhighf2_s_dn18;
        *var_xhighf2_s_dn19_slot = var_xhighf2_s_dn19;
        *var_xhighf2_s_dn2_slot = var_xhighf2_s_dn2;
        *var_xhighf2_s_dn20_slot = var_xhighf2_s_dn20;
        *var_xhighf2_s_dn3_slot = var_xhighf2_s_dn3;
        *var_xhighf2_s_dn4_slot = var_xhighf2_s_dn4;
        *var_xhighf2_s_dn5_slot = var_xhighf2_s_dn5;
        *var_xhighf2_s_dn6_slot = var_xhighf2_s_dn6;
        *var_xhighf2_s_dn7_slot = var_xhighf2_s_dn7;
        *var_xhighf2_s_dn8_slot = var_xhighf2_s_dn8;
        *var_xhighf2_s_dn9_slot = var_xhighf2_s_dn9;
        *var_xhighr_d_slot = var_xhighr_d;
        *var_xhighr_d_db0_slot = var_xhighr_d_db0;
        *var_xhighr_d_db1_slot = var_xhighr_d_db1;
        *var_xhighr_d_db10_slot = var_xhighr_d_db10;
        *var_xhighr_d_db11_slot = var_xhighr_d_db11;
        *var_xhighr_d_db12_slot = var_xhighr_d_db12;
        *var_xhighr_d_db13_slot = var_xhighr_d_db13;
        *var_xhighr_d_db14_slot = var_xhighr_d_db14;
        *var_xhighr_d_db15_slot = var_xhighr_d_db15;
        *var_xhighr_d_db16_slot = var_xhighr_d_db16;
        *var_xhighr_d_db17_slot = var_xhighr_d_db17;
        *var_xhighr_d_db18_slot = var_xhighr_d_db18;
        *var_xhighr_d_db19_slot = var_xhighr_d_db19;
        *var_xhighr_d_db2_slot = var_xhighr_d_db2;
        *var_xhighr_d_db20_slot = var_xhighr_d_db20;
        *var_xhighr_d_db21_slot = var_xhighr_d_db21;
        *var_xhighr_d_db22_slot = var_xhighr_d_db22;
        *var_xhighr_d_db23_slot = var_xhighr_d_db23;
        *var_xhighr_d_db24_slot = var_xhighr_d_db24;
        *var_xhighr_d_db3_slot = var_xhighr_d_db3;
        *var_xhighr_d_db4_slot = var_xhighr_d_db4;
        *var_xhighr_d_db5_slot = var_xhighr_d_db5;
        *var_xhighr_d_db6_slot = var_xhighr_d_db6;
        *var_xhighr_d_db7_slot = var_xhighr_d_db7;
        *var_xhighr_d_db8_slot = var_xhighr_d_db8;
        *var_xhighr_d_db9_slot = var_xhighr_d_db9;
        *var_xhighr_d_dn0_slot = var_xhighr_d_dn0;
        *var_xhighr_d_dn1_slot = var_xhighr_d_dn1;
        *var_xhighr_d_dn10_slot = var_xhighr_d_dn10;
        *var_xhighr_d_dn11_slot = var_xhighr_d_dn11;
        *var_xhighr_d_dn12_slot = var_xhighr_d_dn12;
        *var_xhighr_d_dn13_slot = var_xhighr_d_dn13;
        *var_xhighr_d_dn14_slot = var_xhighr_d_dn14;
        *var_xhighr_d_dn15_slot = var_xhighr_d_dn15;
        *var_xhighr_d_dn16_slot = var_xhighr_d_dn16;
        *var_xhighr_d_dn17_slot = var_xhighr_d_dn17;
        *var_xhighr_d_dn18_slot = var_xhighr_d_dn18;
        *var_xhighr_d_dn19_slot = var_xhighr_d_dn19;
        *var_xhighr_d_dn2_slot = var_xhighr_d_dn2;
        *var_xhighr_d_dn20_slot = var_xhighr_d_dn20;
        *var_xhighr_d_dn3_slot = var_xhighr_d_dn3;
        *var_xhighr_d_dn4_slot = var_xhighr_d_dn4;
        *var_xhighr_d_dn5_slot = var_xhighr_d_dn5;
        *var_xhighr_d_dn6_slot = var_xhighr_d_dn6;
        *var_xhighr_d_dn7_slot = var_xhighr_d_dn7;
        *var_xhighr_d_dn8_slot = var_xhighr_d_dn8;
        *var_xhighr_d_dn9_slot = var_xhighr_d_dn9;
        *var_xhighr_s_slot = var_xhighr_s;
        *var_xhighr_s_db0_slot = var_xhighr_s_db0;
        *var_xhighr_s_db1_slot = var_xhighr_s_db1;
        *var_xhighr_s_db10_slot = var_xhighr_s_db10;
        *var_xhighr_s_db11_slot = var_xhighr_s_db11;
        *var_xhighr_s_db12_slot = var_xhighr_s_db12;
        *var_xhighr_s_db13_slot = var_xhighr_s_db13;
        *var_xhighr_s_db14_slot = var_xhighr_s_db14;
        *var_xhighr_s_db15_slot = var_xhighr_s_db15;
        *var_xhighr_s_db16_slot = var_xhighr_s_db16;
        *var_xhighr_s_db17_slot = var_xhighr_s_db17;
        *var_xhighr_s_db18_slot = var_xhighr_s_db18;
        *var_xhighr_s_db19_slot = var_xhighr_s_db19;
        *var_xhighr_s_db2_slot = var_xhighr_s_db2;
        *var_xhighr_s_db20_slot = var_xhighr_s_db20;
        *var_xhighr_s_db21_slot = var_xhighr_s_db21;
        *var_xhighr_s_db22_slot = var_xhighr_s_db22;
        *var_xhighr_s_db23_slot = var_xhighr_s_db23;
        *var_xhighr_s_db24_slot = var_xhighr_s_db24;
        *var_xhighr_s_db3_slot = var_xhighr_s_db3;
        *var_xhighr_s_db4_slot = var_xhighr_s_db4;
        *var_xhighr_s_db5_slot = var_xhighr_s_db5;
        *var_xhighr_s_db6_slot = var_xhighr_s_db6;
        *var_xhighr_s_db7_slot = var_xhighr_s_db7;
        *var_xhighr_s_db8_slot = var_xhighr_s_db8;
        *var_xhighr_s_db9_slot = var_xhighr_s_db9;
        *var_xhighr_s_dn0_slot = var_xhighr_s_dn0;
        *var_xhighr_s_dn1_slot = var_xhighr_s_dn1;
        *var_xhighr_s_dn10_slot = var_xhighr_s_dn10;
        *var_xhighr_s_dn11_slot = var_xhighr_s_dn11;
        *var_xhighr_s_dn12_slot = var_xhighr_s_dn12;
        *var_xhighr_s_dn13_slot = var_xhighr_s_dn13;
        *var_xhighr_s_dn14_slot = var_xhighr_s_dn14;
        *var_xhighr_s_dn15_slot = var_xhighr_s_dn15;
        *var_xhighr_s_dn16_slot = var_xhighr_s_dn16;
        *var_xhighr_s_dn17_slot = var_xhighr_s_dn17;
        *var_xhighr_s_dn18_slot = var_xhighr_s_dn18;
        *var_xhighr_s_dn19_slot = var_xhighr_s_dn19;
        *var_xhighr_s_dn2_slot = var_xhighr_s_dn2;
        *var_xhighr_s_dn20_slot = var_xhighr_s_dn20;
        *var_xhighr_s_dn3_slot = var_xhighr_s_dn3;
        *var_xhighr_s_dn4_slot = var_xhighr_s_dn4;
        *var_xhighr_s_dn5_slot = var_xhighr_s_dn5;
        *var_xhighr_s_dn6_slot = var_xhighr_s_dn6;
        *var_xhighr_s_dn7_slot = var_xhighr_s_dn7;
        *var_xhighr_s_dn8_slot = var_xhighr_s_dn8;
        *var_xhighr_s_dn9_slot = var_xhighr_s_dn9;
    }

    pub(super) fn stamp_transient_block_15(
        var_expxhr_d_slot: &mut f64,
        var_expxhr_d_db0_slot: &mut f64,
        var_expxhr_d_db1_slot: &mut f64,
        var_expxhr_d_db10_slot: &mut f64,
        var_expxhr_d_db11_slot: &mut f64,
        var_expxhr_d_db12_slot: &mut f64,
        var_expxhr_d_db13_slot: &mut f64,
        var_expxhr_d_db14_slot: &mut f64,
        var_expxhr_d_db15_slot: &mut f64,
        var_expxhr_d_db16_slot: &mut f64,
        var_expxhr_d_db17_slot: &mut f64,
        var_expxhr_d_db18_slot: &mut f64,
        var_expxhr_d_db19_slot: &mut f64,
        var_expxhr_d_db2_slot: &mut f64,
        var_expxhr_d_db20_slot: &mut f64,
        var_expxhr_d_db21_slot: &mut f64,
        var_expxhr_d_db22_slot: &mut f64,
        var_expxhr_d_db23_slot: &mut f64,
        var_expxhr_d_db24_slot: &mut f64,
        var_expxhr_d_db3_slot: &mut f64,
        var_expxhr_d_db4_slot: &mut f64,
        var_expxhr_d_db5_slot: &mut f64,
        var_expxhr_d_db6_slot: &mut f64,
        var_expxhr_d_db7_slot: &mut f64,
        var_expxhr_d_db8_slot: &mut f64,
        var_expxhr_d_db9_slot: &mut f64,
        var_expxhr_d_dn0_slot: &mut f64,
        var_expxhr_d_dn1_slot: &mut f64,
        var_expxhr_d_dn10_slot: &mut f64,
        var_expxhr_d_dn11_slot: &mut f64,
        var_expxhr_d_dn12_slot: &mut f64,
        var_expxhr_d_dn13_slot: &mut f64,
        var_expxhr_d_dn14_slot: &mut f64,
        var_expxhr_d_dn15_slot: &mut f64,
        var_expxhr_d_dn16_slot: &mut f64,
        var_expxhr_d_dn17_slot: &mut f64,
        var_expxhr_d_dn18_slot: &mut f64,
        var_expxhr_d_dn19_slot: &mut f64,
        var_expxhr_d_dn2_slot: &mut f64,
        var_expxhr_d_dn20_slot: &mut f64,
        var_expxhr_d_dn3_slot: &mut f64,
        var_expxhr_d_dn4_slot: &mut f64,
        var_expxhr_d_dn5_slot: &mut f64,
        var_expxhr_d_dn6_slot: &mut f64,
        var_expxhr_d_dn7_slot: &mut f64,
        var_expxhr_d_dn8_slot: &mut f64,
        var_expxhr_d_dn9_slot: &mut f64,
        var_expxhr_s_slot: &mut f64,
        var_expxhr_s_db0_slot: &mut f64,
        var_expxhr_s_db1_slot: &mut f64,
        var_expxhr_s_db10_slot: &mut f64,
        var_expxhr_s_db11_slot: &mut f64,
        var_expxhr_s_db12_slot: &mut f64,
        var_expxhr_s_db13_slot: &mut f64,
        var_expxhr_s_db14_slot: &mut f64,
        var_expxhr_s_db15_slot: &mut f64,
        var_expxhr_s_db16_slot: &mut f64,
        var_expxhr_s_db17_slot: &mut f64,
        var_expxhr_s_db18_slot: &mut f64,
        var_expxhr_s_db19_slot: &mut f64,
        var_expxhr_s_db2_slot: &mut f64,
        var_expxhr_s_db20_slot: &mut f64,
        var_expxhr_s_db21_slot: &mut f64,
        var_expxhr_s_db22_slot: &mut f64,
        var_expxhr_s_db23_slot: &mut f64,
        var_expxhr_s_db24_slot: &mut f64,
        var_expxhr_s_db3_slot: &mut f64,
        var_expxhr_s_db4_slot: &mut f64,
        var_expxhr_s_db5_slot: &mut f64,
        var_expxhr_s_db6_slot: &mut f64,
        var_expxhr_s_db7_slot: &mut f64,
        var_expxhr_s_db8_slot: &mut f64,
        var_expxhr_s_db9_slot: &mut f64,
        var_expxhr_s_dn0_slot: &mut f64,
        var_expxhr_s_dn1_slot: &mut f64,
        var_expxhr_s_dn10_slot: &mut f64,
        var_expxhr_s_dn11_slot: &mut f64,
        var_expxhr_s_dn12_slot: &mut f64,
        var_expxhr_s_dn13_slot: &mut f64,
        var_expxhr_s_dn14_slot: &mut f64,
        var_expxhr_s_dn15_slot: &mut f64,
        var_expxhr_s_dn16_slot: &mut f64,
        var_expxhr_s_dn17_slot: &mut f64,
        var_expxhr_s_dn18_slot: &mut f64,
        var_expxhr_s_dn19_slot: &mut f64,
        var_expxhr_s_dn2_slot: &mut f64,
        var_expxhr_s_dn20_slot: &mut f64,
        var_expxhr_s_dn3_slot: &mut f64,
        var_expxhr_s_dn4_slot: &mut f64,
        var_expxhr_s_dn5_slot: &mut f64,
        var_expxhr_s_dn6_slot: &mut f64,
        var_expxhr_s_dn7_slot: &mut f64,
        var_expxhr_s_dn8_slot: &mut f64,
        var_expxhr_s_dn9_slot: &mut f64,
        var_i1_cor_slot: &mut f64,
        var_i1_cor_db0_slot: &mut f64,
        var_i1_cor_db1_slot: &mut f64,
        var_i1_cor_db10_slot: &mut f64,
        var_i1_cor_db11_slot: &mut f64,
        var_i1_cor_db12_slot: &mut f64,
        var_i1_cor_db13_slot: &mut f64,
        var_i1_cor_db14_slot: &mut f64,
        var_i1_cor_db15_slot: &mut f64,
        var_i1_cor_db16_slot: &mut f64,
        var_i1_cor_db17_slot: &mut f64,
        var_i1_cor_db18_slot: &mut f64,
        var_i1_cor_db19_slot: &mut f64,
        var_i1_cor_db2_slot: &mut f64,
        var_i1_cor_db20_slot: &mut f64,
        var_i1_cor_db21_slot: &mut f64,
        var_i1_cor_db22_slot: &mut f64,
        var_i1_cor_db23_slot: &mut f64,
        var_i1_cor_db24_slot: &mut f64,
        var_i1_cor_db3_slot: &mut f64,
        var_i1_cor_db4_slot: &mut f64,
        var_i1_cor_db5_slot: &mut f64,
        var_i1_cor_db6_slot: &mut f64,
        var_i1_cor_db7_slot: &mut f64,
        var_i1_cor_db8_slot: &mut f64,
        var_i1_cor_db9_slot: &mut f64,
        var_i1_cor_dn0_slot: &mut f64,
        var_i1_cor_dn1_slot: &mut f64,
        var_i1_cor_dn10_slot: &mut f64,
        var_i1_cor_dn11_slot: &mut f64,
        var_i1_cor_dn12_slot: &mut f64,
        var_i1_cor_dn13_slot: &mut f64,
        var_i1_cor_dn14_slot: &mut f64,
        var_i1_cor_dn15_slot: &mut f64,
        var_i1_cor_dn16_slot: &mut f64,
        var_i1_cor_dn17_slot: &mut f64,
        var_i1_cor_dn18_slot: &mut f64,
        var_i1_cor_dn19_slot: &mut f64,
        var_i1_cor_dn2_slot: &mut f64,
        var_i1_cor_dn20_slot: &mut f64,
        var_i1_cor_dn3_slot: &mut f64,
        var_i1_cor_dn4_slot: &mut f64,
        var_i1_cor_dn5_slot: &mut f64,
        var_i1_cor_dn6_slot: &mut f64,
        var_i1_cor_dn7_slot: &mut f64,
        var_i1_cor_dn8_slot: &mut f64,
        var_i1_cor_dn9_slot: &mut f64,
        var_i2_cor_slot: &mut f64,
        var_i2_cor_db0_slot: &mut f64,
        var_i2_cor_db1_slot: &mut f64,
        var_i2_cor_db10_slot: &mut f64,
        var_i2_cor_db11_slot: &mut f64,
        var_i2_cor_db12_slot: &mut f64,
        var_i2_cor_db13_slot: &mut f64,
        var_i2_cor_db14_slot: &mut f64,
        var_i2_cor_db15_slot: &mut f64,
        var_i2_cor_db16_slot: &mut f64,
        var_i2_cor_db17_slot: &mut f64,
        var_i2_cor_db18_slot: &mut f64,
        var_i2_cor_db19_slot: &mut f64,
        var_i2_cor_db2_slot: &mut f64,
        var_i2_cor_db20_slot: &mut f64,
        var_i2_cor_db21_slot: &mut f64,
        var_i2_cor_db22_slot: &mut f64,
        var_i2_cor_db23_slot: &mut f64,
        var_i2_cor_db24_slot: &mut f64,
        var_i2_cor_db3_slot: &mut f64,
        var_i2_cor_db4_slot: &mut f64,
        var_i2_cor_db5_slot: &mut f64,
        var_i2_cor_db6_slot: &mut f64,
        var_i2_cor_db7_slot: &mut f64,
        var_i2_cor_db8_slot: &mut f64,
        var_i2_cor_db9_slot: &mut f64,
        var_i2_cor_dn0_slot: &mut f64,
        var_i2_cor_dn1_slot: &mut f64,
        var_i2_cor_dn10_slot: &mut f64,
        var_i2_cor_dn11_slot: &mut f64,
        var_i2_cor_dn12_slot: &mut f64,
        var_i2_cor_dn13_slot: &mut f64,
        var_i2_cor_dn14_slot: &mut f64,
        var_i2_cor_dn15_slot: &mut f64,
        var_i2_cor_dn16_slot: &mut f64,
        var_i2_cor_dn17_slot: &mut f64,
        var_i2_cor_dn18_slot: &mut f64,
        var_i2_cor_dn19_slot: &mut f64,
        var_i2_cor_dn2_slot: &mut f64,
        var_i2_cor_dn20_slot: &mut f64,
        var_i2_cor_dn3_slot: &mut f64,
        var_i2_cor_dn4_slot: &mut f64,
        var_i2_cor_dn5_slot: &mut f64,
        var_i2_cor_dn6_slot: &mut f64,
        var_i2_cor_dn7_slot: &mut f64,
        var_i2_cor_dn8_slot: &mut f64,
        var_i2_cor_dn9_slot: &mut f64,
        var_i3_cor_slot: &mut f64,
        var_i3_cor_db0_slot: &mut f64,
        var_i3_cor_db1_slot: &mut f64,
        var_i3_cor_db10_slot: &mut f64,
        var_i3_cor_db11_slot: &mut f64,
        var_i3_cor_db12_slot: &mut f64,
        var_i3_cor_db13_slot: &mut f64,
        var_i3_cor_db14_slot: &mut f64,
        var_i3_cor_db15_slot: &mut f64,
        var_i3_cor_db16_slot: &mut f64,
        var_i3_cor_db17_slot: &mut f64,
        var_i3_cor_db18_slot: &mut f64,
        var_i3_cor_db19_slot: &mut f64,
        var_i3_cor_db2_slot: &mut f64,
        var_i3_cor_db20_slot: &mut f64,
        var_i3_cor_db21_slot: &mut f64,
        var_i3_cor_db22_slot: &mut f64,
        var_i3_cor_db23_slot: &mut f64,
        var_i3_cor_db24_slot: &mut f64,
        var_i3_cor_db3_slot: &mut f64,
        var_i3_cor_db4_slot: &mut f64,
        var_i3_cor_db5_slot: &mut f64,
        var_i3_cor_db6_slot: &mut f64,
        var_i3_cor_db7_slot: &mut f64,
        var_i3_cor_db8_slot: &mut f64,
        var_i3_cor_db9_slot: &mut f64,
        var_i3_cor_dn0_slot: &mut f64,
        var_i3_cor_dn1_slot: &mut f64,
        var_i3_cor_dn10_slot: &mut f64,
        var_i3_cor_dn11_slot: &mut f64,
        var_i3_cor_dn12_slot: &mut f64,
        var_i3_cor_dn13_slot: &mut f64,
        var_i3_cor_dn14_slot: &mut f64,
        var_i3_cor_dn15_slot: &mut f64,
        var_i3_cor_dn16_slot: &mut f64,
        var_i3_cor_dn17_slot: &mut f64,
        var_i3_cor_dn18_slot: &mut f64,
        var_i3_cor_dn19_slot: &mut f64,
        var_i3_cor_dn2_slot: &mut f64,
        var_i3_cor_dn20_slot: &mut f64,
        var_i3_cor_dn3_slot: &mut f64,
        var_i3_cor_dn4_slot: &mut f64,
        var_i3_cor_dn5_slot: &mut f64,
        var_i3_cor_dn6_slot: &mut f64,
        var_i3_cor_dn7_slot: &mut f64,
        var_i3_cor_dn8_slot: &mut f64,
        var_i3_cor_dn9_slot: &mut f64,
        var_i4_cor_slot: &mut f64,
        var_i4_cor_db0_slot: &mut f64,
        var_i4_cor_db1_slot: &mut f64,
        var_i4_cor_db10_slot: &mut f64,
        var_i4_cor_db11_slot: &mut f64,
        var_i4_cor_db12_slot: &mut f64,
        var_i4_cor_db13_slot: &mut f64,
        var_i4_cor_db14_slot: &mut f64,
        var_i4_cor_db15_slot: &mut f64,
        var_i4_cor_db16_slot: &mut f64,
        var_i4_cor_db17_slot: &mut f64,
        var_i4_cor_db18_slot: &mut f64,
        var_i4_cor_db19_slot: &mut f64,
        var_i4_cor_db2_slot: &mut f64,
        var_i4_cor_db20_slot: &mut f64,
        var_i4_cor_db21_slot: &mut f64,
        var_i4_cor_db22_slot: &mut f64,
        var_i4_cor_db23_slot: &mut f64,
        var_i4_cor_db24_slot: &mut f64,
        var_i4_cor_db3_slot: &mut f64,
        var_i4_cor_db4_slot: &mut f64,
        var_i4_cor_db5_slot: &mut f64,
        var_i4_cor_db6_slot: &mut f64,
        var_i4_cor_db7_slot: &mut f64,
        var_i4_cor_db8_slot: &mut f64,
        var_i4_cor_db9_slot: &mut f64,
        var_i4_cor_dn0_slot: &mut f64,
        var_i4_cor_dn1_slot: &mut f64,
        var_i4_cor_dn10_slot: &mut f64,
        var_i4_cor_dn11_slot: &mut f64,
        var_i4_cor_dn12_slot: &mut f64,
        var_i4_cor_dn13_slot: &mut f64,
        var_i4_cor_dn14_slot: &mut f64,
        var_i4_cor_dn15_slot: &mut f64,
        var_i4_cor_dn16_slot: &mut f64,
        var_i4_cor_dn17_slot: &mut f64,
        var_i4_cor_dn18_slot: &mut f64,
        var_i4_cor_dn19_slot: &mut f64,
        var_i4_cor_dn2_slot: &mut f64,
        var_i4_cor_dn20_slot: &mut f64,
        var_i4_cor_dn3_slot: &mut f64,
        var_i4_cor_dn4_slot: &mut f64,
        var_i4_cor_dn5_slot: &mut f64,
        var_i4_cor_dn6_slot: &mut f64,
        var_i4_cor_dn7_slot: &mut f64,
        var_i4_cor_dn8_slot: &mut f64,
        var_i4_cor_dn9_slot: &mut f64,
        var_i5_cor_slot: &mut f64,
        var_i5_cor_db0_slot: &mut f64,
        var_i5_cor_db1_slot: &mut f64,
        var_i5_cor_db10_slot: &mut f64,
        var_i5_cor_db11_slot: &mut f64,
        var_i5_cor_db12_slot: &mut f64,
        var_i5_cor_db13_slot: &mut f64,
        var_i5_cor_db14_slot: &mut f64,
        var_i5_cor_db15_slot: &mut f64,
        var_i5_cor_db16_slot: &mut f64,
        var_i5_cor_db17_slot: &mut f64,
        var_i5_cor_db18_slot: &mut f64,
        var_i5_cor_db19_slot: &mut f64,
        var_i5_cor_db2_slot: &mut f64,
        var_i5_cor_db20_slot: &mut f64,
        var_i5_cor_db21_slot: &mut f64,
        var_i5_cor_db22_slot: &mut f64,
        var_i5_cor_db23_slot: &mut f64,
        var_i5_cor_db24_slot: &mut f64,
        var_i5_cor_db3_slot: &mut f64,
        var_i5_cor_db4_slot: &mut f64,
        var_i5_cor_db5_slot: &mut f64,
        var_i5_cor_db6_slot: &mut f64,
        var_i5_cor_db7_slot: &mut f64,
        var_i5_cor_db8_slot: &mut f64,
        var_i5_cor_db9_slot: &mut f64,
        var_i5_cor_dn0_slot: &mut f64,
        var_i5_cor_dn1_slot: &mut f64,
        var_i5_cor_dn10_slot: &mut f64,
        var_i5_cor_dn11_slot: &mut f64,
        var_i5_cor_dn12_slot: &mut f64,
        var_i5_cor_dn13_slot: &mut f64,
        var_i5_cor_dn14_slot: &mut f64,
        var_i5_cor_dn15_slot: &mut f64,
        var_i5_cor_dn16_slot: &mut f64,
        var_i5_cor_dn17_slot: &mut f64,
        var_i5_cor_dn18_slot: &mut f64,
        var_i5_cor_dn19_slot: &mut f64,
        var_i5_cor_dn2_slot: &mut f64,
        var_i5_cor_dn20_slot: &mut f64,
        var_i5_cor_dn3_slot: &mut f64,
        var_i5_cor_dn4_slot: &mut f64,
        var_i5_cor_dn5_slot: &mut f64,
        var_i5_cor_dn6_slot: &mut f64,
        var_i5_cor_dn7_slot: &mut f64,
        var_i5_cor_dn8_slot: &mut f64,
        var_i5_cor_dn9_slot: &mut f64,
        var_m0_rev_slot: &mut f64,
        var_m0_rev_db0_slot: &mut f64,
        var_m0_rev_db1_slot: &mut f64,
        var_m0_rev_db10_slot: &mut f64,
        var_m0_rev_db11_slot: &mut f64,
        var_m0_rev_db12_slot: &mut f64,
        var_m0_rev_db13_slot: &mut f64,
        var_m0_rev_db14_slot: &mut f64,
        var_m0_rev_db15_slot: &mut f64,
        var_m0_rev_db16_slot: &mut f64,
        var_m0_rev_db17_slot: &mut f64,
        var_m0_rev_db18_slot: &mut f64,
        var_m0_rev_db19_slot: &mut f64,
        var_m0_rev_db2_slot: &mut f64,
        var_m0_rev_db20_slot: &mut f64,
        var_m0_rev_db21_slot: &mut f64,
        var_m0_rev_db22_slot: &mut f64,
        var_m0_rev_db23_slot: &mut f64,
        var_m0_rev_db24_slot: &mut f64,
        var_m0_rev_db3_slot: &mut f64,
        var_m0_rev_db4_slot: &mut f64,
        var_m0_rev_db5_slot: &mut f64,
        var_m0_rev_db6_slot: &mut f64,
        var_m0_rev_db7_slot: &mut f64,
        var_m0_rev_db8_slot: &mut f64,
        var_m0_rev_db9_slot: &mut f64,
        var_m0_rev_dn0_slot: &mut f64,
        var_m0_rev_dn1_slot: &mut f64,
        var_m0_rev_dn10_slot: &mut f64,
        var_m0_rev_dn11_slot: &mut f64,
        var_m0_rev_dn12_slot: &mut f64,
        var_m0_rev_dn13_slot: &mut f64,
        var_m0_rev_dn14_slot: &mut f64,
        var_m0_rev_dn15_slot: &mut f64,
        var_m0_rev_dn16_slot: &mut f64,
        var_m0_rev_dn17_slot: &mut f64,
        var_m0_rev_dn18_slot: &mut f64,
        var_m0_rev_dn19_slot: &mut f64,
        var_m0_rev_dn2_slot: &mut f64,
        var_m0_rev_dn20_slot: &mut f64,
        var_m0_rev_dn3_slot: &mut f64,
        var_m0_rev_dn4_slot: &mut f64,
        var_m0_rev_dn5_slot: &mut f64,
        var_m0_rev_dn6_slot: &mut f64,
        var_m0_rev_dn7_slot: &mut f64,
        var_m0_rev_dn8_slot: &mut f64,
        var_m0_rev_dn9_slot: &mut f64,
        var_mcor_rev_slot: &mut f64,
        var_mcor_rev_db0_slot: &mut f64,
        var_mcor_rev_db1_slot: &mut f64,
        var_mcor_rev_db10_slot: &mut f64,
        var_mcor_rev_db11_slot: &mut f64,
        var_mcor_rev_db12_slot: &mut f64,
        var_mcor_rev_db13_slot: &mut f64,
        var_mcor_rev_db14_slot: &mut f64,
        var_mcor_rev_db15_slot: &mut f64,
        var_mcor_rev_db16_slot: &mut f64,
        var_mcor_rev_db17_slot: &mut f64,
        var_mcor_rev_db18_slot: &mut f64,
        var_mcor_rev_db19_slot: &mut f64,
        var_mcor_rev_db2_slot: &mut f64,
        var_mcor_rev_db20_slot: &mut f64,
        var_mcor_rev_db21_slot: &mut f64,
        var_mcor_rev_db22_slot: &mut f64,
        var_mcor_rev_db23_slot: &mut f64,
        var_mcor_rev_db24_slot: &mut f64,
        var_mcor_rev_db3_slot: &mut f64,
        var_mcor_rev_db4_slot: &mut f64,
        var_mcor_rev_db5_slot: &mut f64,
        var_mcor_rev_db6_slot: &mut f64,
        var_mcor_rev_db7_slot: &mut f64,
        var_mcor_rev_db8_slot: &mut f64,
        var_mcor_rev_db9_slot: &mut f64,
        var_mcor_rev_dn0_slot: &mut f64,
        var_mcor_rev_dn1_slot: &mut f64,
        var_mcor_rev_dn10_slot: &mut f64,
        var_mcor_rev_dn11_slot: &mut f64,
        var_mcor_rev_dn12_slot: &mut f64,
        var_mcor_rev_dn13_slot: &mut f64,
        var_mcor_rev_dn14_slot: &mut f64,
        var_mcor_rev_dn15_slot: &mut f64,
        var_mcor_rev_dn16_slot: &mut f64,
        var_mcor_rev_dn17_slot: &mut f64,
        var_mcor_rev_dn18_slot: &mut f64,
        var_mcor_rev_dn19_slot: &mut f64,
        var_mcor_rev_dn2_slot: &mut f64,
        var_mcor_rev_dn20_slot: &mut f64,
        var_mcor_rev_dn3_slot: &mut f64,
        var_mcor_rev_dn4_slot: &mut f64,
        var_mcor_rev_dn5_slot: &mut f64,
        var_mcor_rev_dn6_slot: &mut f64,
        var_mcor_rev_dn7_slot: &mut f64,
        var_mcor_rev_dn8_slot: &mut f64,
        var_mcor_rev_dn9_slot: &mut f64,
        var_tt0_slot: &mut f64,
        var_tt1_slot: &mut f64,
        var_tt1_db0_slot: &mut f64,
        var_tt1_db1_slot: &mut f64,
        var_tt1_db10_slot: &mut f64,
        var_tt1_db11_slot: &mut f64,
        var_tt1_db12_slot: &mut f64,
        var_tt1_db13_slot: &mut f64,
        var_tt1_db14_slot: &mut f64,
        var_tt1_db15_slot: &mut f64,
        var_tt1_db16_slot: &mut f64,
        var_tt1_db17_slot: &mut f64,
        var_tt1_db18_slot: &mut f64,
        var_tt1_db19_slot: &mut f64,
        var_tt1_db2_slot: &mut f64,
        var_tt1_db20_slot: &mut f64,
        var_tt1_db21_slot: &mut f64,
        var_tt1_db22_slot: &mut f64,
        var_tt1_db23_slot: &mut f64,
        var_tt1_db24_slot: &mut f64,
        var_tt1_db3_slot: &mut f64,
        var_tt1_db4_slot: &mut f64,
        var_tt1_db5_slot: &mut f64,
        var_tt1_db6_slot: &mut f64,
        var_tt1_db7_slot: &mut f64,
        var_tt1_db8_slot: &mut f64,
        var_tt1_db9_slot: &mut f64,
        var_tt1_dn0_slot: &mut f64,
        var_tt1_dn1_slot: &mut f64,
        var_tt1_dn10_slot: &mut f64,
        var_tt1_dn11_slot: &mut f64,
        var_tt1_dn12_slot: &mut f64,
        var_tt1_dn13_slot: &mut f64,
        var_tt1_dn14_slot: &mut f64,
        var_tt1_dn15_slot: &mut f64,
        var_tt1_dn16_slot: &mut f64,
        var_tt1_dn17_slot: &mut f64,
        var_tt1_dn18_slot: &mut f64,
        var_tt1_dn19_slot: &mut f64,
        var_tt1_dn2_slot: &mut f64,
        var_tt1_dn20_slot: &mut f64,
        var_tt1_dn3_slot: &mut f64,
        var_tt1_dn4_slot: &mut f64,
        var_tt1_dn5_slot: &mut f64,
        var_tt1_dn6_slot: &mut f64,
        var_tt1_dn7_slot: &mut f64,
        var_tt1_dn8_slot: &mut f64,
        var_tt1_dn9_slot: &mut f64,
        var_zflagbot_d_slot: &mut f64,
        var_zflagbot_s_slot: &mut f64,
        var_zflaggat_d_slot: &mut f64,
        var_zflaggat_s_slot: &mut f64,
        var_zflagsti_d_slot: &mut f64,
        var_zflagsti_s_slot: &mut f64,
    ) {
        let mut var_expxhr_d: f64 = *var_expxhr_d_slot;
        let mut var_expxhr_d_db0: f64 = *var_expxhr_d_db0_slot;
        let mut var_expxhr_d_db1: f64 = *var_expxhr_d_db1_slot;
        let mut var_expxhr_d_db10: f64 = *var_expxhr_d_db10_slot;
        let mut var_expxhr_d_db11: f64 = *var_expxhr_d_db11_slot;
        let mut var_expxhr_d_db12: f64 = *var_expxhr_d_db12_slot;
        let mut var_expxhr_d_db13: f64 = *var_expxhr_d_db13_slot;
        let mut var_expxhr_d_db14: f64 = *var_expxhr_d_db14_slot;
        let mut var_expxhr_d_db15: f64 = *var_expxhr_d_db15_slot;
        let mut var_expxhr_d_db16: f64 = *var_expxhr_d_db16_slot;
        let mut var_expxhr_d_db17: f64 = *var_expxhr_d_db17_slot;
        let mut var_expxhr_d_db18: f64 = *var_expxhr_d_db18_slot;
        let mut var_expxhr_d_db19: f64 = *var_expxhr_d_db19_slot;
        let mut var_expxhr_d_db2: f64 = *var_expxhr_d_db2_slot;
        let mut var_expxhr_d_db20: f64 = *var_expxhr_d_db20_slot;
        let mut var_expxhr_d_db21: f64 = *var_expxhr_d_db21_slot;
        let mut var_expxhr_d_db22: f64 = *var_expxhr_d_db22_slot;
        let mut var_expxhr_d_db23: f64 = *var_expxhr_d_db23_slot;
        let mut var_expxhr_d_db24: f64 = *var_expxhr_d_db24_slot;
        let mut var_expxhr_d_db3: f64 = *var_expxhr_d_db3_slot;
        let mut var_expxhr_d_db4: f64 = *var_expxhr_d_db4_slot;
        let mut var_expxhr_d_db5: f64 = *var_expxhr_d_db5_slot;
        let mut var_expxhr_d_db6: f64 = *var_expxhr_d_db6_slot;
        let mut var_expxhr_d_db7: f64 = *var_expxhr_d_db7_slot;
        let mut var_expxhr_d_db8: f64 = *var_expxhr_d_db8_slot;
        let mut var_expxhr_d_db9: f64 = *var_expxhr_d_db9_slot;
        let mut var_expxhr_d_dn0: f64 = *var_expxhr_d_dn0_slot;
        let mut var_expxhr_d_dn1: f64 = *var_expxhr_d_dn1_slot;
        let mut var_expxhr_d_dn10: f64 = *var_expxhr_d_dn10_slot;
        let mut var_expxhr_d_dn11: f64 = *var_expxhr_d_dn11_slot;
        let mut var_expxhr_d_dn12: f64 = *var_expxhr_d_dn12_slot;
        let mut var_expxhr_d_dn13: f64 = *var_expxhr_d_dn13_slot;
        let mut var_expxhr_d_dn14: f64 = *var_expxhr_d_dn14_slot;
        let mut var_expxhr_d_dn15: f64 = *var_expxhr_d_dn15_slot;
        let mut var_expxhr_d_dn16: f64 = *var_expxhr_d_dn16_slot;
        let mut var_expxhr_d_dn17: f64 = *var_expxhr_d_dn17_slot;
        let mut var_expxhr_d_dn18: f64 = *var_expxhr_d_dn18_slot;
        let mut var_expxhr_d_dn19: f64 = *var_expxhr_d_dn19_slot;
        let mut var_expxhr_d_dn2: f64 = *var_expxhr_d_dn2_slot;
        let mut var_expxhr_d_dn20: f64 = *var_expxhr_d_dn20_slot;
        let mut var_expxhr_d_dn3: f64 = *var_expxhr_d_dn3_slot;
        let mut var_expxhr_d_dn4: f64 = *var_expxhr_d_dn4_slot;
        let mut var_expxhr_d_dn5: f64 = *var_expxhr_d_dn5_slot;
        let mut var_expxhr_d_dn6: f64 = *var_expxhr_d_dn6_slot;
        let mut var_expxhr_d_dn7: f64 = *var_expxhr_d_dn7_slot;
        let mut var_expxhr_d_dn8: f64 = *var_expxhr_d_dn8_slot;
        let mut var_expxhr_d_dn9: f64 = *var_expxhr_d_dn9_slot;
        let mut var_expxhr_s: f64 = *var_expxhr_s_slot;
        let mut var_expxhr_s_db0: f64 = *var_expxhr_s_db0_slot;
        let mut var_expxhr_s_db1: f64 = *var_expxhr_s_db1_slot;
        let mut var_expxhr_s_db10: f64 = *var_expxhr_s_db10_slot;
        let mut var_expxhr_s_db11: f64 = *var_expxhr_s_db11_slot;
        let mut var_expxhr_s_db12: f64 = *var_expxhr_s_db12_slot;
        let mut var_expxhr_s_db13: f64 = *var_expxhr_s_db13_slot;
        let mut var_expxhr_s_db14: f64 = *var_expxhr_s_db14_slot;
        let mut var_expxhr_s_db15: f64 = *var_expxhr_s_db15_slot;
        let mut var_expxhr_s_db16: f64 = *var_expxhr_s_db16_slot;
        let mut var_expxhr_s_db17: f64 = *var_expxhr_s_db17_slot;
        let mut var_expxhr_s_db18: f64 = *var_expxhr_s_db18_slot;
        let mut var_expxhr_s_db19: f64 = *var_expxhr_s_db19_slot;
        let mut var_expxhr_s_db2: f64 = *var_expxhr_s_db2_slot;
        let mut var_expxhr_s_db20: f64 = *var_expxhr_s_db20_slot;
        let mut var_expxhr_s_db21: f64 = *var_expxhr_s_db21_slot;
        let mut var_expxhr_s_db22: f64 = *var_expxhr_s_db22_slot;
        let mut var_expxhr_s_db23: f64 = *var_expxhr_s_db23_slot;
        let mut var_expxhr_s_db24: f64 = *var_expxhr_s_db24_slot;
        let mut var_expxhr_s_db3: f64 = *var_expxhr_s_db3_slot;
        let mut var_expxhr_s_db4: f64 = *var_expxhr_s_db4_slot;
        let mut var_expxhr_s_db5: f64 = *var_expxhr_s_db5_slot;
        let mut var_expxhr_s_db6: f64 = *var_expxhr_s_db6_slot;
        let mut var_expxhr_s_db7: f64 = *var_expxhr_s_db7_slot;
        let mut var_expxhr_s_db8: f64 = *var_expxhr_s_db8_slot;
        let mut var_expxhr_s_db9: f64 = *var_expxhr_s_db9_slot;
        let mut var_expxhr_s_dn0: f64 = *var_expxhr_s_dn0_slot;
        let mut var_expxhr_s_dn1: f64 = *var_expxhr_s_dn1_slot;
        let mut var_expxhr_s_dn10: f64 = *var_expxhr_s_dn10_slot;
        let mut var_expxhr_s_dn11: f64 = *var_expxhr_s_dn11_slot;
        let mut var_expxhr_s_dn12: f64 = *var_expxhr_s_dn12_slot;
        let mut var_expxhr_s_dn13: f64 = *var_expxhr_s_dn13_slot;
        let mut var_expxhr_s_dn14: f64 = *var_expxhr_s_dn14_slot;
        let mut var_expxhr_s_dn15: f64 = *var_expxhr_s_dn15_slot;
        let mut var_expxhr_s_dn16: f64 = *var_expxhr_s_dn16_slot;
        let mut var_expxhr_s_dn17: f64 = *var_expxhr_s_dn17_slot;
        let mut var_expxhr_s_dn18: f64 = *var_expxhr_s_dn18_slot;
        let mut var_expxhr_s_dn19: f64 = *var_expxhr_s_dn19_slot;
        let mut var_expxhr_s_dn2: f64 = *var_expxhr_s_dn2_slot;
        let mut var_expxhr_s_dn20: f64 = *var_expxhr_s_dn20_slot;
        let mut var_expxhr_s_dn3: f64 = *var_expxhr_s_dn3_slot;
        let mut var_expxhr_s_dn4: f64 = *var_expxhr_s_dn4_slot;
        let mut var_expxhr_s_dn5: f64 = *var_expxhr_s_dn5_slot;
        let mut var_expxhr_s_dn6: f64 = *var_expxhr_s_dn6_slot;
        let mut var_expxhr_s_dn7: f64 = *var_expxhr_s_dn7_slot;
        let mut var_expxhr_s_dn8: f64 = *var_expxhr_s_dn8_slot;
        let mut var_expxhr_s_dn9: f64 = *var_expxhr_s_dn9_slot;
        let mut var_i1_cor: f64 = *var_i1_cor_slot;
        let mut var_i1_cor_db0: f64 = *var_i1_cor_db0_slot;
        let mut var_i1_cor_db1: f64 = *var_i1_cor_db1_slot;
        let mut var_i1_cor_db10: f64 = *var_i1_cor_db10_slot;
        let mut var_i1_cor_db11: f64 = *var_i1_cor_db11_slot;
        let mut var_i1_cor_db12: f64 = *var_i1_cor_db12_slot;
        let mut var_i1_cor_db13: f64 = *var_i1_cor_db13_slot;
        let mut var_i1_cor_db14: f64 = *var_i1_cor_db14_slot;
        let mut var_i1_cor_db15: f64 = *var_i1_cor_db15_slot;
        let mut var_i1_cor_db16: f64 = *var_i1_cor_db16_slot;
        let mut var_i1_cor_db17: f64 = *var_i1_cor_db17_slot;
        let mut var_i1_cor_db18: f64 = *var_i1_cor_db18_slot;
        let mut var_i1_cor_db19: f64 = *var_i1_cor_db19_slot;
        let mut var_i1_cor_db2: f64 = *var_i1_cor_db2_slot;
        let mut var_i1_cor_db20: f64 = *var_i1_cor_db20_slot;
        let mut var_i1_cor_db21: f64 = *var_i1_cor_db21_slot;
        let mut var_i1_cor_db22: f64 = *var_i1_cor_db22_slot;
        let mut var_i1_cor_db23: f64 = *var_i1_cor_db23_slot;
        let mut var_i1_cor_db24: f64 = *var_i1_cor_db24_slot;
        let mut var_i1_cor_db3: f64 = *var_i1_cor_db3_slot;
        let mut var_i1_cor_db4: f64 = *var_i1_cor_db4_slot;
        let mut var_i1_cor_db5: f64 = *var_i1_cor_db5_slot;
        let mut var_i1_cor_db6: f64 = *var_i1_cor_db6_slot;
        let mut var_i1_cor_db7: f64 = *var_i1_cor_db7_slot;
        let mut var_i1_cor_db8: f64 = *var_i1_cor_db8_slot;
        let mut var_i1_cor_db9: f64 = *var_i1_cor_db9_slot;
        let mut var_i1_cor_dn0: f64 = *var_i1_cor_dn0_slot;
        let mut var_i1_cor_dn1: f64 = *var_i1_cor_dn1_slot;
        let mut var_i1_cor_dn10: f64 = *var_i1_cor_dn10_slot;
        let mut var_i1_cor_dn11: f64 = *var_i1_cor_dn11_slot;
        let mut var_i1_cor_dn12: f64 = *var_i1_cor_dn12_slot;
        let mut var_i1_cor_dn13: f64 = *var_i1_cor_dn13_slot;
        let mut var_i1_cor_dn14: f64 = *var_i1_cor_dn14_slot;
        let mut var_i1_cor_dn15: f64 = *var_i1_cor_dn15_slot;
        let mut var_i1_cor_dn16: f64 = *var_i1_cor_dn16_slot;
        let mut var_i1_cor_dn17: f64 = *var_i1_cor_dn17_slot;
        let mut var_i1_cor_dn18: f64 = *var_i1_cor_dn18_slot;
        let mut var_i1_cor_dn19: f64 = *var_i1_cor_dn19_slot;
        let mut var_i1_cor_dn2: f64 = *var_i1_cor_dn2_slot;
        let mut var_i1_cor_dn20: f64 = *var_i1_cor_dn20_slot;
        let mut var_i1_cor_dn3: f64 = *var_i1_cor_dn3_slot;
        let mut var_i1_cor_dn4: f64 = *var_i1_cor_dn4_slot;
        let mut var_i1_cor_dn5: f64 = *var_i1_cor_dn5_slot;
        let mut var_i1_cor_dn6: f64 = *var_i1_cor_dn6_slot;
        let mut var_i1_cor_dn7: f64 = *var_i1_cor_dn7_slot;
        let mut var_i1_cor_dn8: f64 = *var_i1_cor_dn8_slot;
        let mut var_i1_cor_dn9: f64 = *var_i1_cor_dn9_slot;
        let mut var_i2_cor: f64 = *var_i2_cor_slot;
        let mut var_i2_cor_db0: f64 = *var_i2_cor_db0_slot;
        let mut var_i2_cor_db1: f64 = *var_i2_cor_db1_slot;
        let mut var_i2_cor_db10: f64 = *var_i2_cor_db10_slot;
        let mut var_i2_cor_db11: f64 = *var_i2_cor_db11_slot;
        let mut var_i2_cor_db12: f64 = *var_i2_cor_db12_slot;
        let mut var_i2_cor_db13: f64 = *var_i2_cor_db13_slot;
        let mut var_i2_cor_db14: f64 = *var_i2_cor_db14_slot;
        let mut var_i2_cor_db15: f64 = *var_i2_cor_db15_slot;
        let mut var_i2_cor_db16: f64 = *var_i2_cor_db16_slot;
        let mut var_i2_cor_db17: f64 = *var_i2_cor_db17_slot;
        let mut var_i2_cor_db18: f64 = *var_i2_cor_db18_slot;
        let mut var_i2_cor_db19: f64 = *var_i2_cor_db19_slot;
        let mut var_i2_cor_db2: f64 = *var_i2_cor_db2_slot;
        let mut var_i2_cor_db20: f64 = *var_i2_cor_db20_slot;
        let mut var_i2_cor_db21: f64 = *var_i2_cor_db21_slot;
        let mut var_i2_cor_db22: f64 = *var_i2_cor_db22_slot;
        let mut var_i2_cor_db23: f64 = *var_i2_cor_db23_slot;
        let mut var_i2_cor_db24: f64 = *var_i2_cor_db24_slot;
        let mut var_i2_cor_db3: f64 = *var_i2_cor_db3_slot;
        let mut var_i2_cor_db4: f64 = *var_i2_cor_db4_slot;
        let mut var_i2_cor_db5: f64 = *var_i2_cor_db5_slot;
        let mut var_i2_cor_db6: f64 = *var_i2_cor_db6_slot;
        let mut var_i2_cor_db7: f64 = *var_i2_cor_db7_slot;
        let mut var_i2_cor_db8: f64 = *var_i2_cor_db8_slot;
        let mut var_i2_cor_db9: f64 = *var_i2_cor_db9_slot;
        let mut var_i2_cor_dn0: f64 = *var_i2_cor_dn0_slot;
        let mut var_i2_cor_dn1: f64 = *var_i2_cor_dn1_slot;
        let mut var_i2_cor_dn10: f64 = *var_i2_cor_dn10_slot;
        let mut var_i2_cor_dn11: f64 = *var_i2_cor_dn11_slot;
        let mut var_i2_cor_dn12: f64 = *var_i2_cor_dn12_slot;
        let mut var_i2_cor_dn13: f64 = *var_i2_cor_dn13_slot;
        let mut var_i2_cor_dn14: f64 = *var_i2_cor_dn14_slot;
        let mut var_i2_cor_dn15: f64 = *var_i2_cor_dn15_slot;
        let mut var_i2_cor_dn16: f64 = *var_i2_cor_dn16_slot;
        let mut var_i2_cor_dn17: f64 = *var_i2_cor_dn17_slot;
        let mut var_i2_cor_dn18: f64 = *var_i2_cor_dn18_slot;
        let mut var_i2_cor_dn19: f64 = *var_i2_cor_dn19_slot;
        let mut var_i2_cor_dn2: f64 = *var_i2_cor_dn2_slot;
        let mut var_i2_cor_dn20: f64 = *var_i2_cor_dn20_slot;
        let mut var_i2_cor_dn3: f64 = *var_i2_cor_dn3_slot;
        let mut var_i2_cor_dn4: f64 = *var_i2_cor_dn4_slot;
        let mut var_i2_cor_dn5: f64 = *var_i2_cor_dn5_slot;
        let mut var_i2_cor_dn6: f64 = *var_i2_cor_dn6_slot;
        let mut var_i2_cor_dn7: f64 = *var_i2_cor_dn7_slot;
        let mut var_i2_cor_dn8: f64 = *var_i2_cor_dn8_slot;
        let mut var_i2_cor_dn9: f64 = *var_i2_cor_dn9_slot;
        let mut var_i3_cor: f64 = *var_i3_cor_slot;
        let mut var_i3_cor_db0: f64 = *var_i3_cor_db0_slot;
        let mut var_i3_cor_db1: f64 = *var_i3_cor_db1_slot;
        let mut var_i3_cor_db10: f64 = *var_i3_cor_db10_slot;
        let mut var_i3_cor_db11: f64 = *var_i3_cor_db11_slot;
        let mut var_i3_cor_db12: f64 = *var_i3_cor_db12_slot;
        let mut var_i3_cor_db13: f64 = *var_i3_cor_db13_slot;
        let mut var_i3_cor_db14: f64 = *var_i3_cor_db14_slot;
        let mut var_i3_cor_db15: f64 = *var_i3_cor_db15_slot;
        let mut var_i3_cor_db16: f64 = *var_i3_cor_db16_slot;
        let mut var_i3_cor_db17: f64 = *var_i3_cor_db17_slot;
        let mut var_i3_cor_db18: f64 = *var_i3_cor_db18_slot;
        let mut var_i3_cor_db19: f64 = *var_i3_cor_db19_slot;
        let mut var_i3_cor_db2: f64 = *var_i3_cor_db2_slot;
        let mut var_i3_cor_db20: f64 = *var_i3_cor_db20_slot;
        let mut var_i3_cor_db21: f64 = *var_i3_cor_db21_slot;
        let mut var_i3_cor_db22: f64 = *var_i3_cor_db22_slot;
        let mut var_i3_cor_db23: f64 = *var_i3_cor_db23_slot;
        let mut var_i3_cor_db24: f64 = *var_i3_cor_db24_slot;
        let mut var_i3_cor_db3: f64 = *var_i3_cor_db3_slot;
        let mut var_i3_cor_db4: f64 = *var_i3_cor_db4_slot;
        let mut var_i3_cor_db5: f64 = *var_i3_cor_db5_slot;
        let mut var_i3_cor_db6: f64 = *var_i3_cor_db6_slot;
        let mut var_i3_cor_db7: f64 = *var_i3_cor_db7_slot;
        let mut var_i3_cor_db8: f64 = *var_i3_cor_db8_slot;
        let mut var_i3_cor_db9: f64 = *var_i3_cor_db9_slot;
        let mut var_i3_cor_dn0: f64 = *var_i3_cor_dn0_slot;
        let mut var_i3_cor_dn1: f64 = *var_i3_cor_dn1_slot;
        let mut var_i3_cor_dn10: f64 = *var_i3_cor_dn10_slot;
        let mut var_i3_cor_dn11: f64 = *var_i3_cor_dn11_slot;
        let mut var_i3_cor_dn12: f64 = *var_i3_cor_dn12_slot;
        let mut var_i3_cor_dn13: f64 = *var_i3_cor_dn13_slot;
        let mut var_i3_cor_dn14: f64 = *var_i3_cor_dn14_slot;
        let mut var_i3_cor_dn15: f64 = *var_i3_cor_dn15_slot;
        let mut var_i3_cor_dn16: f64 = *var_i3_cor_dn16_slot;
        let mut var_i3_cor_dn17: f64 = *var_i3_cor_dn17_slot;
        let mut var_i3_cor_dn18: f64 = *var_i3_cor_dn18_slot;
        let mut var_i3_cor_dn19: f64 = *var_i3_cor_dn19_slot;
        let mut var_i3_cor_dn2: f64 = *var_i3_cor_dn2_slot;
        let mut var_i3_cor_dn20: f64 = *var_i3_cor_dn20_slot;
        let mut var_i3_cor_dn3: f64 = *var_i3_cor_dn3_slot;
        let mut var_i3_cor_dn4: f64 = *var_i3_cor_dn4_slot;
        let mut var_i3_cor_dn5: f64 = *var_i3_cor_dn5_slot;
        let mut var_i3_cor_dn6: f64 = *var_i3_cor_dn6_slot;
        let mut var_i3_cor_dn7: f64 = *var_i3_cor_dn7_slot;
        let mut var_i3_cor_dn8: f64 = *var_i3_cor_dn8_slot;
        let mut var_i3_cor_dn9: f64 = *var_i3_cor_dn9_slot;
        let mut var_i4_cor: f64 = *var_i4_cor_slot;
        let mut var_i4_cor_db0: f64 = *var_i4_cor_db0_slot;
        let mut var_i4_cor_db1: f64 = *var_i4_cor_db1_slot;
        let mut var_i4_cor_db10: f64 = *var_i4_cor_db10_slot;
        let mut var_i4_cor_db11: f64 = *var_i4_cor_db11_slot;
        let mut var_i4_cor_db12: f64 = *var_i4_cor_db12_slot;
        let mut var_i4_cor_db13: f64 = *var_i4_cor_db13_slot;
        let mut var_i4_cor_db14: f64 = *var_i4_cor_db14_slot;
        let mut var_i4_cor_db15: f64 = *var_i4_cor_db15_slot;
        let mut var_i4_cor_db16: f64 = *var_i4_cor_db16_slot;
        let mut var_i4_cor_db17: f64 = *var_i4_cor_db17_slot;
        let mut var_i4_cor_db18: f64 = *var_i4_cor_db18_slot;
        let mut var_i4_cor_db19: f64 = *var_i4_cor_db19_slot;
        let mut var_i4_cor_db2: f64 = *var_i4_cor_db2_slot;
        let mut var_i4_cor_db20: f64 = *var_i4_cor_db20_slot;
        let mut var_i4_cor_db21: f64 = *var_i4_cor_db21_slot;
        let mut var_i4_cor_db22: f64 = *var_i4_cor_db22_slot;
        let mut var_i4_cor_db23: f64 = *var_i4_cor_db23_slot;
        let mut var_i4_cor_db24: f64 = *var_i4_cor_db24_slot;
        let mut var_i4_cor_db3: f64 = *var_i4_cor_db3_slot;
        let mut var_i4_cor_db4: f64 = *var_i4_cor_db4_slot;
        let mut var_i4_cor_db5: f64 = *var_i4_cor_db5_slot;
        let mut var_i4_cor_db6: f64 = *var_i4_cor_db6_slot;
        let mut var_i4_cor_db7: f64 = *var_i4_cor_db7_slot;
        let mut var_i4_cor_db8: f64 = *var_i4_cor_db8_slot;
        let mut var_i4_cor_db9: f64 = *var_i4_cor_db9_slot;
        let mut var_i4_cor_dn0: f64 = *var_i4_cor_dn0_slot;
        let mut var_i4_cor_dn1: f64 = *var_i4_cor_dn1_slot;
        let mut var_i4_cor_dn10: f64 = *var_i4_cor_dn10_slot;
        let mut var_i4_cor_dn11: f64 = *var_i4_cor_dn11_slot;
        let mut var_i4_cor_dn12: f64 = *var_i4_cor_dn12_slot;
        let mut var_i4_cor_dn13: f64 = *var_i4_cor_dn13_slot;
        let mut var_i4_cor_dn14: f64 = *var_i4_cor_dn14_slot;
        let mut var_i4_cor_dn15: f64 = *var_i4_cor_dn15_slot;
        let mut var_i4_cor_dn16: f64 = *var_i4_cor_dn16_slot;
        let mut var_i4_cor_dn17: f64 = *var_i4_cor_dn17_slot;
        let mut var_i4_cor_dn18: f64 = *var_i4_cor_dn18_slot;
        let mut var_i4_cor_dn19: f64 = *var_i4_cor_dn19_slot;
        let mut var_i4_cor_dn2: f64 = *var_i4_cor_dn2_slot;
        let mut var_i4_cor_dn20: f64 = *var_i4_cor_dn20_slot;
        let mut var_i4_cor_dn3: f64 = *var_i4_cor_dn3_slot;
        let mut var_i4_cor_dn4: f64 = *var_i4_cor_dn4_slot;
        let mut var_i4_cor_dn5: f64 = *var_i4_cor_dn5_slot;
        let mut var_i4_cor_dn6: f64 = *var_i4_cor_dn6_slot;
        let mut var_i4_cor_dn7: f64 = *var_i4_cor_dn7_slot;
        let mut var_i4_cor_dn8: f64 = *var_i4_cor_dn8_slot;
        let mut var_i4_cor_dn9: f64 = *var_i4_cor_dn9_slot;
        let mut var_i5_cor: f64 = *var_i5_cor_slot;
        let mut var_i5_cor_db0: f64 = *var_i5_cor_db0_slot;
        let mut var_i5_cor_db1: f64 = *var_i5_cor_db1_slot;
        let mut var_i5_cor_db10: f64 = *var_i5_cor_db10_slot;
        let mut var_i5_cor_db11: f64 = *var_i5_cor_db11_slot;
        let mut var_i5_cor_db12: f64 = *var_i5_cor_db12_slot;
        let mut var_i5_cor_db13: f64 = *var_i5_cor_db13_slot;
        let mut var_i5_cor_db14: f64 = *var_i5_cor_db14_slot;
        let mut var_i5_cor_db15: f64 = *var_i5_cor_db15_slot;
        let mut var_i5_cor_db16: f64 = *var_i5_cor_db16_slot;
        let mut var_i5_cor_db17: f64 = *var_i5_cor_db17_slot;
        let mut var_i5_cor_db18: f64 = *var_i5_cor_db18_slot;
        let mut var_i5_cor_db19: f64 = *var_i5_cor_db19_slot;
        let mut var_i5_cor_db2: f64 = *var_i5_cor_db2_slot;
        let mut var_i5_cor_db20: f64 = *var_i5_cor_db20_slot;
        let mut var_i5_cor_db21: f64 = *var_i5_cor_db21_slot;
        let mut var_i5_cor_db22: f64 = *var_i5_cor_db22_slot;
        let mut var_i5_cor_db23: f64 = *var_i5_cor_db23_slot;
        let mut var_i5_cor_db24: f64 = *var_i5_cor_db24_slot;
        let mut var_i5_cor_db3: f64 = *var_i5_cor_db3_slot;
        let mut var_i5_cor_db4: f64 = *var_i5_cor_db4_slot;
        let mut var_i5_cor_db5: f64 = *var_i5_cor_db5_slot;
        let mut var_i5_cor_db6: f64 = *var_i5_cor_db6_slot;
        let mut var_i5_cor_db7: f64 = *var_i5_cor_db7_slot;
        let mut var_i5_cor_db8: f64 = *var_i5_cor_db8_slot;
        let mut var_i5_cor_db9: f64 = *var_i5_cor_db9_slot;
        let mut var_i5_cor_dn0: f64 = *var_i5_cor_dn0_slot;
        let mut var_i5_cor_dn1: f64 = *var_i5_cor_dn1_slot;
        let mut var_i5_cor_dn10: f64 = *var_i5_cor_dn10_slot;
        let mut var_i5_cor_dn11: f64 = *var_i5_cor_dn11_slot;
        let mut var_i5_cor_dn12: f64 = *var_i5_cor_dn12_slot;
        let mut var_i5_cor_dn13: f64 = *var_i5_cor_dn13_slot;
        let mut var_i5_cor_dn14: f64 = *var_i5_cor_dn14_slot;
        let mut var_i5_cor_dn15: f64 = *var_i5_cor_dn15_slot;
        let mut var_i5_cor_dn16: f64 = *var_i5_cor_dn16_slot;
        let mut var_i5_cor_dn17: f64 = *var_i5_cor_dn17_slot;
        let mut var_i5_cor_dn18: f64 = *var_i5_cor_dn18_slot;
        let mut var_i5_cor_dn19: f64 = *var_i5_cor_dn19_slot;
        let mut var_i5_cor_dn2: f64 = *var_i5_cor_dn2_slot;
        let mut var_i5_cor_dn20: f64 = *var_i5_cor_dn20_slot;
        let mut var_i5_cor_dn3: f64 = *var_i5_cor_dn3_slot;
        let mut var_i5_cor_dn4: f64 = *var_i5_cor_dn4_slot;
        let mut var_i5_cor_dn5: f64 = *var_i5_cor_dn5_slot;
        let mut var_i5_cor_dn6: f64 = *var_i5_cor_dn6_slot;
        let mut var_i5_cor_dn7: f64 = *var_i5_cor_dn7_slot;
        let mut var_i5_cor_dn8: f64 = *var_i5_cor_dn8_slot;
        let mut var_i5_cor_dn9: f64 = *var_i5_cor_dn9_slot;
        let mut var_m0_rev: f64 = *var_m0_rev_slot;
        let mut var_m0_rev_db0: f64 = *var_m0_rev_db0_slot;
        let mut var_m0_rev_db1: f64 = *var_m0_rev_db1_slot;
        let mut var_m0_rev_db10: f64 = *var_m0_rev_db10_slot;
        let mut var_m0_rev_db11: f64 = *var_m0_rev_db11_slot;
        let mut var_m0_rev_db12: f64 = *var_m0_rev_db12_slot;
        let mut var_m0_rev_db13: f64 = *var_m0_rev_db13_slot;
        let mut var_m0_rev_db14: f64 = *var_m0_rev_db14_slot;
        let mut var_m0_rev_db15: f64 = *var_m0_rev_db15_slot;
        let mut var_m0_rev_db16: f64 = *var_m0_rev_db16_slot;
        let mut var_m0_rev_db17: f64 = *var_m0_rev_db17_slot;
        let mut var_m0_rev_db18: f64 = *var_m0_rev_db18_slot;
        let mut var_m0_rev_db19: f64 = *var_m0_rev_db19_slot;
        let mut var_m0_rev_db2: f64 = *var_m0_rev_db2_slot;
        let mut var_m0_rev_db20: f64 = *var_m0_rev_db20_slot;
        let mut var_m0_rev_db21: f64 = *var_m0_rev_db21_slot;
        let mut var_m0_rev_db22: f64 = *var_m0_rev_db22_slot;
        let mut var_m0_rev_db23: f64 = *var_m0_rev_db23_slot;
        let mut var_m0_rev_db24: f64 = *var_m0_rev_db24_slot;
        let mut var_m0_rev_db3: f64 = *var_m0_rev_db3_slot;
        let mut var_m0_rev_db4: f64 = *var_m0_rev_db4_slot;
        let mut var_m0_rev_db5: f64 = *var_m0_rev_db5_slot;
        let mut var_m0_rev_db6: f64 = *var_m0_rev_db6_slot;
        let mut var_m0_rev_db7: f64 = *var_m0_rev_db7_slot;
        let mut var_m0_rev_db8: f64 = *var_m0_rev_db8_slot;
        let mut var_m0_rev_db9: f64 = *var_m0_rev_db9_slot;
        let mut var_m0_rev_dn0: f64 = *var_m0_rev_dn0_slot;
        let mut var_m0_rev_dn1: f64 = *var_m0_rev_dn1_slot;
        let mut var_m0_rev_dn10: f64 = *var_m0_rev_dn10_slot;
        let mut var_m0_rev_dn11: f64 = *var_m0_rev_dn11_slot;
        let mut var_m0_rev_dn12: f64 = *var_m0_rev_dn12_slot;
        let mut var_m0_rev_dn13: f64 = *var_m0_rev_dn13_slot;
        let mut var_m0_rev_dn14: f64 = *var_m0_rev_dn14_slot;
        let mut var_m0_rev_dn15: f64 = *var_m0_rev_dn15_slot;
        let mut var_m0_rev_dn16: f64 = *var_m0_rev_dn16_slot;
        let mut var_m0_rev_dn17: f64 = *var_m0_rev_dn17_slot;
        let mut var_m0_rev_dn18: f64 = *var_m0_rev_dn18_slot;
        let mut var_m0_rev_dn19: f64 = *var_m0_rev_dn19_slot;
        let mut var_m0_rev_dn2: f64 = *var_m0_rev_dn2_slot;
        let mut var_m0_rev_dn20: f64 = *var_m0_rev_dn20_slot;
        let mut var_m0_rev_dn3: f64 = *var_m0_rev_dn3_slot;
        let mut var_m0_rev_dn4: f64 = *var_m0_rev_dn4_slot;
        let mut var_m0_rev_dn5: f64 = *var_m0_rev_dn5_slot;
        let mut var_m0_rev_dn6: f64 = *var_m0_rev_dn6_slot;
        let mut var_m0_rev_dn7: f64 = *var_m0_rev_dn7_slot;
        let mut var_m0_rev_dn8: f64 = *var_m0_rev_dn8_slot;
        let mut var_m0_rev_dn9: f64 = *var_m0_rev_dn9_slot;
        let mut var_mcor_rev: f64 = *var_mcor_rev_slot;
        let mut var_mcor_rev_db0: f64 = *var_mcor_rev_db0_slot;
        let mut var_mcor_rev_db1: f64 = *var_mcor_rev_db1_slot;
        let mut var_mcor_rev_db10: f64 = *var_mcor_rev_db10_slot;
        let mut var_mcor_rev_db11: f64 = *var_mcor_rev_db11_slot;
        let mut var_mcor_rev_db12: f64 = *var_mcor_rev_db12_slot;
        let mut var_mcor_rev_db13: f64 = *var_mcor_rev_db13_slot;
        let mut var_mcor_rev_db14: f64 = *var_mcor_rev_db14_slot;
        let mut var_mcor_rev_db15: f64 = *var_mcor_rev_db15_slot;
        let mut var_mcor_rev_db16: f64 = *var_mcor_rev_db16_slot;
        let mut var_mcor_rev_db17: f64 = *var_mcor_rev_db17_slot;
        let mut var_mcor_rev_db18: f64 = *var_mcor_rev_db18_slot;
        let mut var_mcor_rev_db19: f64 = *var_mcor_rev_db19_slot;
        let mut var_mcor_rev_db2: f64 = *var_mcor_rev_db2_slot;
        let mut var_mcor_rev_db20: f64 = *var_mcor_rev_db20_slot;
        let mut var_mcor_rev_db21: f64 = *var_mcor_rev_db21_slot;
        let mut var_mcor_rev_db22: f64 = *var_mcor_rev_db22_slot;
        let mut var_mcor_rev_db23: f64 = *var_mcor_rev_db23_slot;
        let mut var_mcor_rev_db24: f64 = *var_mcor_rev_db24_slot;
        let mut var_mcor_rev_db3: f64 = *var_mcor_rev_db3_slot;
        let mut var_mcor_rev_db4: f64 = *var_mcor_rev_db4_slot;
        let mut var_mcor_rev_db5: f64 = *var_mcor_rev_db5_slot;
        let mut var_mcor_rev_db6: f64 = *var_mcor_rev_db6_slot;
        let mut var_mcor_rev_db7: f64 = *var_mcor_rev_db7_slot;
        let mut var_mcor_rev_db8: f64 = *var_mcor_rev_db8_slot;
        let mut var_mcor_rev_db9: f64 = *var_mcor_rev_db9_slot;
        let mut var_mcor_rev_dn0: f64 = *var_mcor_rev_dn0_slot;
        let mut var_mcor_rev_dn1: f64 = *var_mcor_rev_dn1_slot;
        let mut var_mcor_rev_dn10: f64 = *var_mcor_rev_dn10_slot;
        let mut var_mcor_rev_dn11: f64 = *var_mcor_rev_dn11_slot;
        let mut var_mcor_rev_dn12: f64 = *var_mcor_rev_dn12_slot;
        let mut var_mcor_rev_dn13: f64 = *var_mcor_rev_dn13_slot;
        let mut var_mcor_rev_dn14: f64 = *var_mcor_rev_dn14_slot;
        let mut var_mcor_rev_dn15: f64 = *var_mcor_rev_dn15_slot;
        let mut var_mcor_rev_dn16: f64 = *var_mcor_rev_dn16_slot;
        let mut var_mcor_rev_dn17: f64 = *var_mcor_rev_dn17_slot;
        let mut var_mcor_rev_dn18: f64 = *var_mcor_rev_dn18_slot;
        let mut var_mcor_rev_dn19: f64 = *var_mcor_rev_dn19_slot;
        let mut var_mcor_rev_dn2: f64 = *var_mcor_rev_dn2_slot;
        let mut var_mcor_rev_dn20: f64 = *var_mcor_rev_dn20_slot;
        let mut var_mcor_rev_dn3: f64 = *var_mcor_rev_dn3_slot;
        let mut var_mcor_rev_dn4: f64 = *var_mcor_rev_dn4_slot;
        let mut var_mcor_rev_dn5: f64 = *var_mcor_rev_dn5_slot;
        let mut var_mcor_rev_dn6: f64 = *var_mcor_rev_dn6_slot;
        let mut var_mcor_rev_dn7: f64 = *var_mcor_rev_dn7_slot;
        let mut var_mcor_rev_dn8: f64 = *var_mcor_rev_dn8_slot;
        let mut var_mcor_rev_dn9: f64 = *var_mcor_rev_dn9_slot;
        let mut var_tt0: f64 = *var_tt0_slot;
        let mut var_tt1: f64 = *var_tt1_slot;
        let mut var_tt1_db0: f64 = *var_tt1_db0_slot;
        let mut var_tt1_db1: f64 = *var_tt1_db1_slot;
        let mut var_tt1_db10: f64 = *var_tt1_db10_slot;
        let mut var_tt1_db11: f64 = *var_tt1_db11_slot;
        let mut var_tt1_db12: f64 = *var_tt1_db12_slot;
        let mut var_tt1_db13: f64 = *var_tt1_db13_slot;
        let mut var_tt1_db14: f64 = *var_tt1_db14_slot;
        let mut var_tt1_db15: f64 = *var_tt1_db15_slot;
        let mut var_tt1_db16: f64 = *var_tt1_db16_slot;
        let mut var_tt1_db17: f64 = *var_tt1_db17_slot;
        let mut var_tt1_db18: f64 = *var_tt1_db18_slot;
        let mut var_tt1_db19: f64 = *var_tt1_db19_slot;
        let mut var_tt1_db2: f64 = *var_tt1_db2_slot;
        let mut var_tt1_db20: f64 = *var_tt1_db20_slot;
        let mut var_tt1_db21: f64 = *var_tt1_db21_slot;
        let mut var_tt1_db22: f64 = *var_tt1_db22_slot;
        let mut var_tt1_db23: f64 = *var_tt1_db23_slot;
        let mut var_tt1_db24: f64 = *var_tt1_db24_slot;
        let mut var_tt1_db3: f64 = *var_tt1_db3_slot;
        let mut var_tt1_db4: f64 = *var_tt1_db4_slot;
        let mut var_tt1_db5: f64 = *var_tt1_db5_slot;
        let mut var_tt1_db6: f64 = *var_tt1_db6_slot;
        let mut var_tt1_db7: f64 = *var_tt1_db7_slot;
        let mut var_tt1_db8: f64 = *var_tt1_db8_slot;
        let mut var_tt1_db9: f64 = *var_tt1_db9_slot;
        let mut var_tt1_dn0: f64 = *var_tt1_dn0_slot;
        let mut var_tt1_dn1: f64 = *var_tt1_dn1_slot;
        let mut var_tt1_dn10: f64 = *var_tt1_dn10_slot;
        let mut var_tt1_dn11: f64 = *var_tt1_dn11_slot;
        let mut var_tt1_dn12: f64 = *var_tt1_dn12_slot;
        let mut var_tt1_dn13: f64 = *var_tt1_dn13_slot;
        let mut var_tt1_dn14: f64 = *var_tt1_dn14_slot;
        let mut var_tt1_dn15: f64 = *var_tt1_dn15_slot;
        let mut var_tt1_dn16: f64 = *var_tt1_dn16_slot;
        let mut var_tt1_dn17: f64 = *var_tt1_dn17_slot;
        let mut var_tt1_dn18: f64 = *var_tt1_dn18_slot;
        let mut var_tt1_dn19: f64 = *var_tt1_dn19_slot;
        let mut var_tt1_dn2: f64 = *var_tt1_dn2_slot;
        let mut var_tt1_dn20: f64 = *var_tt1_dn20_slot;
        let mut var_tt1_dn3: f64 = *var_tt1_dn3_slot;
        let mut var_tt1_dn4: f64 = *var_tt1_dn4_slot;
        let mut var_tt1_dn5: f64 = *var_tt1_dn5_slot;
        let mut var_tt1_dn6: f64 = *var_tt1_dn6_slot;
        let mut var_tt1_dn7: f64 = *var_tt1_dn7_slot;
        let mut var_tt1_dn8: f64 = *var_tt1_dn8_slot;
        let mut var_tt1_dn9: f64 = *var_tt1_dn9_slot;
        let mut var_zflagbot_d: f64 = *var_zflagbot_d_slot;
        let mut var_zflagbot_s: f64 = *var_zflagbot_s_slot;
        let mut var_zflaggat_d: f64 = *var_zflaggat_d_slot;
        let mut var_zflaggat_s: f64 = *var_zflaggat_s_slot;
        let mut var_zflagsti_d: f64 = *var_zflagsti_d_slot;
        let mut var_zflagsti_s: f64 = *var_zflagsti_s_slot;

        var_expxhr_s = 0.0;
        var_expxhr_s_dn0 = 0.0;
        var_expxhr_s_dn1 = 0.0;
        var_expxhr_s_dn2 = 0.0;
        var_expxhr_s_dn3 = 0.0;
        var_expxhr_s_dn4 = 0.0;
        var_expxhr_s_dn5 = 0.0;
        var_expxhr_s_dn6 = 0.0;
        var_expxhr_s_dn7 = 0.0;
        var_expxhr_s_dn8 = 0.0;
        var_expxhr_s_dn9 = 0.0;
        var_expxhr_s_dn10 = 0.0;
        var_expxhr_s_dn11 = 0.0;
        var_expxhr_s_dn12 = 0.0;
        var_expxhr_s_dn13 = 0.0;
        var_expxhr_s_dn14 = 0.0;
        var_expxhr_s_dn15 = 0.0;
        var_expxhr_s_dn16 = 0.0;
        var_expxhr_s_dn17 = 0.0;
        var_expxhr_s_dn18 = 0.0;
        var_expxhr_s_dn19 = 0.0;
        var_expxhr_s_dn20 = 0.0;
        var_expxhr_s_db0 = 0.0;
        var_expxhr_s_db1 = 0.0;
        var_expxhr_s_db2 = 0.0;
        var_expxhr_s_db3 = 0.0;
        var_expxhr_s_db4 = 0.0;
        var_expxhr_s_db5 = 0.0;
        var_expxhr_s_db6 = 0.0;
        var_expxhr_s_db7 = 0.0;
        var_expxhr_s_db8 = 0.0;
        var_expxhr_s_db9 = 0.0;
        var_expxhr_s_db10 = 0.0;
        var_expxhr_s_db11 = 0.0;
        var_expxhr_s_db12 = 0.0;
        var_expxhr_s_db13 = 0.0;
        var_expxhr_s_db14 = 0.0;
        var_expxhr_s_db15 = 0.0;
        var_expxhr_s_db16 = 0.0;
        var_expxhr_s_db17 = 0.0;
        var_expxhr_s_db18 = 0.0;
        var_expxhr_s_db19 = 0.0;
        var_expxhr_s_db20 = 0.0;
        var_expxhr_s_db21 = 0.0;
        var_expxhr_s_db22 = 0.0;
        var_expxhr_s_db23 = 0.0;
        var_expxhr_s_db24 = 0.0;

        var_expxhr_d = 0.0;
        var_expxhr_d_dn0 = 0.0;
        var_expxhr_d_dn1 = 0.0;
        var_expxhr_d_dn2 = 0.0;
        var_expxhr_d_dn3 = 0.0;
        var_expxhr_d_dn4 = 0.0;
        var_expxhr_d_dn5 = 0.0;
        var_expxhr_d_dn6 = 0.0;
        var_expxhr_d_dn7 = 0.0;
        var_expxhr_d_dn8 = 0.0;
        var_expxhr_d_dn9 = 0.0;
        var_expxhr_d_dn10 = 0.0;
        var_expxhr_d_dn11 = 0.0;
        var_expxhr_d_dn12 = 0.0;
        var_expxhr_d_dn13 = 0.0;
        var_expxhr_d_dn14 = 0.0;
        var_expxhr_d_dn15 = 0.0;
        var_expxhr_d_dn16 = 0.0;
        var_expxhr_d_dn17 = 0.0;
        var_expxhr_d_dn18 = 0.0;
        var_expxhr_d_dn19 = 0.0;
        var_expxhr_d_dn20 = 0.0;
        var_expxhr_d_db0 = 0.0;
        var_expxhr_d_db1 = 0.0;
        var_expxhr_d_db2 = 0.0;
        var_expxhr_d_db3 = 0.0;
        var_expxhr_d_db4 = 0.0;
        var_expxhr_d_db5 = 0.0;
        var_expxhr_d_db6 = 0.0;
        var_expxhr_d_db7 = 0.0;
        var_expxhr_d_db8 = 0.0;
        var_expxhr_d_db9 = 0.0;
        var_expxhr_d_db10 = 0.0;
        var_expxhr_d_db11 = 0.0;
        var_expxhr_d_db12 = 0.0;
        var_expxhr_d_db13 = 0.0;
        var_expxhr_d_db14 = 0.0;
        var_expxhr_d_db15 = 0.0;
        var_expxhr_d_db16 = 0.0;
        var_expxhr_d_db17 = 0.0;
        var_expxhr_d_db18 = 0.0;
        var_expxhr_d_db19 = 0.0;
        var_expxhr_d_db20 = 0.0;
        var_expxhr_d_db21 = 0.0;
        var_expxhr_d_db22 = 0.0;
        var_expxhr_d_db23 = 0.0;
        var_expxhr_d_db24 = 0.0;

        var_zflagbot_s = 1.0;

        var_zflagbot_d = 1.0;

        var_zflagsti_s = 1.0;

        var_zflagsti_d = 1.0;

        var_zflaggat_s = 1.0;

        var_zflaggat_d = 1.0;

        var_m0_rev = 0.0;
        var_m0_rev_dn0 = 0.0;
        var_m0_rev_dn1 = 0.0;
        var_m0_rev_dn2 = 0.0;
        var_m0_rev_dn3 = 0.0;
        var_m0_rev_dn4 = 0.0;
        var_m0_rev_dn5 = 0.0;
        var_m0_rev_dn6 = 0.0;
        var_m0_rev_dn7 = 0.0;
        var_m0_rev_dn8 = 0.0;
        var_m0_rev_dn9 = 0.0;
        var_m0_rev_dn10 = 0.0;
        var_m0_rev_dn11 = 0.0;
        var_m0_rev_dn12 = 0.0;
        var_m0_rev_dn13 = 0.0;
        var_m0_rev_dn14 = 0.0;
        var_m0_rev_dn15 = 0.0;
        var_m0_rev_dn16 = 0.0;
        var_m0_rev_dn17 = 0.0;
        var_m0_rev_dn18 = 0.0;
        var_m0_rev_dn19 = 0.0;
        var_m0_rev_dn20 = 0.0;
        var_m0_rev_db0 = 0.0;
        var_m0_rev_db1 = 0.0;
        var_m0_rev_db2 = 0.0;
        var_m0_rev_db3 = 0.0;
        var_m0_rev_db4 = 0.0;
        var_m0_rev_db5 = 0.0;
        var_m0_rev_db6 = 0.0;
        var_m0_rev_db7 = 0.0;
        var_m0_rev_db8 = 0.0;
        var_m0_rev_db9 = 0.0;
        var_m0_rev_db10 = 0.0;
        var_m0_rev_db11 = 0.0;
        var_m0_rev_db12 = 0.0;
        var_m0_rev_db13 = 0.0;
        var_m0_rev_db14 = 0.0;
        var_m0_rev_db15 = 0.0;
        var_m0_rev_db16 = 0.0;
        var_m0_rev_db17 = 0.0;
        var_m0_rev_db18 = 0.0;
        var_m0_rev_db19 = 0.0;
        var_m0_rev_db20 = 0.0;
        var_m0_rev_db21 = 0.0;
        var_m0_rev_db22 = 0.0;
        var_m0_rev_db23 = 0.0;
        var_m0_rev_db24 = 0.0;

        var_mcor_rev = 0.0;
        var_mcor_rev_dn0 = 0.0;
        var_mcor_rev_dn1 = 0.0;
        var_mcor_rev_dn2 = 0.0;
        var_mcor_rev_dn3 = 0.0;
        var_mcor_rev_dn4 = 0.0;
        var_mcor_rev_dn5 = 0.0;
        var_mcor_rev_dn6 = 0.0;
        var_mcor_rev_dn7 = 0.0;
        var_mcor_rev_dn8 = 0.0;
        var_mcor_rev_dn9 = 0.0;
        var_mcor_rev_dn10 = 0.0;
        var_mcor_rev_dn11 = 0.0;
        var_mcor_rev_dn12 = 0.0;
        var_mcor_rev_dn13 = 0.0;
        var_mcor_rev_dn14 = 0.0;
        var_mcor_rev_dn15 = 0.0;
        var_mcor_rev_dn16 = 0.0;
        var_mcor_rev_dn17 = 0.0;
        var_mcor_rev_dn18 = 0.0;
        var_mcor_rev_dn19 = 0.0;
        var_mcor_rev_dn20 = 0.0;
        var_mcor_rev_db0 = 0.0;
        var_mcor_rev_db1 = 0.0;
        var_mcor_rev_db2 = 0.0;
        var_mcor_rev_db3 = 0.0;
        var_mcor_rev_db4 = 0.0;
        var_mcor_rev_db5 = 0.0;
        var_mcor_rev_db6 = 0.0;
        var_mcor_rev_db7 = 0.0;
        var_mcor_rev_db8 = 0.0;
        var_mcor_rev_db9 = 0.0;
        var_mcor_rev_db10 = 0.0;
        var_mcor_rev_db11 = 0.0;
        var_mcor_rev_db12 = 0.0;
        var_mcor_rev_db13 = 0.0;
        var_mcor_rev_db14 = 0.0;
        var_mcor_rev_db15 = 0.0;
        var_mcor_rev_db16 = 0.0;
        var_mcor_rev_db17 = 0.0;
        var_mcor_rev_db18 = 0.0;
        var_mcor_rev_db19 = 0.0;
        var_mcor_rev_db20 = 0.0;
        var_mcor_rev_db21 = 0.0;
        var_mcor_rev_db22 = 0.0;
        var_mcor_rev_db23 = 0.0;
        var_mcor_rev_db24 = 0.0;

        var_i1_cor = 0.0;
        var_i1_cor_dn0 = 0.0;
        var_i1_cor_dn1 = 0.0;
        var_i1_cor_dn2 = 0.0;
        var_i1_cor_dn3 = 0.0;
        var_i1_cor_dn4 = 0.0;
        var_i1_cor_dn5 = 0.0;
        var_i1_cor_dn6 = 0.0;
        var_i1_cor_dn7 = 0.0;
        var_i1_cor_dn8 = 0.0;
        var_i1_cor_dn9 = 0.0;
        var_i1_cor_dn10 = 0.0;
        var_i1_cor_dn11 = 0.0;
        var_i1_cor_dn12 = 0.0;
        var_i1_cor_dn13 = 0.0;
        var_i1_cor_dn14 = 0.0;
        var_i1_cor_dn15 = 0.0;
        var_i1_cor_dn16 = 0.0;
        var_i1_cor_dn17 = 0.0;
        var_i1_cor_dn18 = 0.0;
        var_i1_cor_dn19 = 0.0;
        var_i1_cor_dn20 = 0.0;
        var_i1_cor_db0 = 0.0;
        var_i1_cor_db1 = 0.0;
        var_i1_cor_db2 = 0.0;
        var_i1_cor_db3 = 0.0;
        var_i1_cor_db4 = 0.0;
        var_i1_cor_db5 = 0.0;
        var_i1_cor_db6 = 0.0;
        var_i1_cor_db7 = 0.0;
        var_i1_cor_db8 = 0.0;
        var_i1_cor_db9 = 0.0;
        var_i1_cor_db10 = 0.0;
        var_i1_cor_db11 = 0.0;
        var_i1_cor_db12 = 0.0;
        var_i1_cor_db13 = 0.0;
        var_i1_cor_db14 = 0.0;
        var_i1_cor_db15 = 0.0;
        var_i1_cor_db16 = 0.0;
        var_i1_cor_db17 = 0.0;
        var_i1_cor_db18 = 0.0;
        var_i1_cor_db19 = 0.0;
        var_i1_cor_db20 = 0.0;
        var_i1_cor_db21 = 0.0;
        var_i1_cor_db22 = 0.0;
        var_i1_cor_db23 = 0.0;
        var_i1_cor_db24 = 0.0;

        var_i2_cor = 0.0;
        var_i2_cor_dn0 = 0.0;
        var_i2_cor_dn1 = 0.0;
        var_i2_cor_dn2 = 0.0;
        var_i2_cor_dn3 = 0.0;
        var_i2_cor_dn4 = 0.0;
        var_i2_cor_dn5 = 0.0;
        var_i2_cor_dn6 = 0.0;
        var_i2_cor_dn7 = 0.0;
        var_i2_cor_dn8 = 0.0;
        var_i2_cor_dn9 = 0.0;
        var_i2_cor_dn10 = 0.0;
        var_i2_cor_dn11 = 0.0;
        var_i2_cor_dn12 = 0.0;
        var_i2_cor_dn13 = 0.0;
        var_i2_cor_dn14 = 0.0;
        var_i2_cor_dn15 = 0.0;
        var_i2_cor_dn16 = 0.0;
        var_i2_cor_dn17 = 0.0;
        var_i2_cor_dn18 = 0.0;
        var_i2_cor_dn19 = 0.0;
        var_i2_cor_dn20 = 0.0;
        var_i2_cor_db0 = 0.0;
        var_i2_cor_db1 = 0.0;
        var_i2_cor_db2 = 0.0;
        var_i2_cor_db3 = 0.0;
        var_i2_cor_db4 = 0.0;
        var_i2_cor_db5 = 0.0;
        var_i2_cor_db6 = 0.0;
        var_i2_cor_db7 = 0.0;
        var_i2_cor_db8 = 0.0;
        var_i2_cor_db9 = 0.0;
        var_i2_cor_db10 = 0.0;
        var_i2_cor_db11 = 0.0;
        var_i2_cor_db12 = 0.0;
        var_i2_cor_db13 = 0.0;
        var_i2_cor_db14 = 0.0;
        var_i2_cor_db15 = 0.0;
        var_i2_cor_db16 = 0.0;
        var_i2_cor_db17 = 0.0;
        var_i2_cor_db18 = 0.0;
        var_i2_cor_db19 = 0.0;
        var_i2_cor_db20 = 0.0;
        var_i2_cor_db21 = 0.0;
        var_i2_cor_db22 = 0.0;
        var_i2_cor_db23 = 0.0;
        var_i2_cor_db24 = 0.0;

        var_i3_cor = 0.0;
        var_i3_cor_dn0 = 0.0;
        var_i3_cor_dn1 = 0.0;
        var_i3_cor_dn2 = 0.0;
        var_i3_cor_dn3 = 0.0;
        var_i3_cor_dn4 = 0.0;
        var_i3_cor_dn5 = 0.0;
        var_i3_cor_dn6 = 0.0;
        var_i3_cor_dn7 = 0.0;
        var_i3_cor_dn8 = 0.0;
        var_i3_cor_dn9 = 0.0;
        var_i3_cor_dn10 = 0.0;
        var_i3_cor_dn11 = 0.0;
        var_i3_cor_dn12 = 0.0;
        var_i3_cor_dn13 = 0.0;
        var_i3_cor_dn14 = 0.0;
        var_i3_cor_dn15 = 0.0;
        var_i3_cor_dn16 = 0.0;
        var_i3_cor_dn17 = 0.0;
        var_i3_cor_dn18 = 0.0;
        var_i3_cor_dn19 = 0.0;
        var_i3_cor_dn20 = 0.0;
        var_i3_cor_db0 = 0.0;
        var_i3_cor_db1 = 0.0;
        var_i3_cor_db2 = 0.0;
        var_i3_cor_db3 = 0.0;
        var_i3_cor_db4 = 0.0;
        var_i3_cor_db5 = 0.0;
        var_i3_cor_db6 = 0.0;
        var_i3_cor_db7 = 0.0;
        var_i3_cor_db8 = 0.0;
        var_i3_cor_db9 = 0.0;
        var_i3_cor_db10 = 0.0;
        var_i3_cor_db11 = 0.0;
        var_i3_cor_db12 = 0.0;
        var_i3_cor_db13 = 0.0;
        var_i3_cor_db14 = 0.0;
        var_i3_cor_db15 = 0.0;
        var_i3_cor_db16 = 0.0;
        var_i3_cor_db17 = 0.0;
        var_i3_cor_db18 = 0.0;
        var_i3_cor_db19 = 0.0;
        var_i3_cor_db20 = 0.0;
        var_i3_cor_db21 = 0.0;
        var_i3_cor_db22 = 0.0;
        var_i3_cor_db23 = 0.0;
        var_i3_cor_db24 = 0.0;

        var_i4_cor = 0.0;
        var_i4_cor_dn0 = 0.0;
        var_i4_cor_dn1 = 0.0;
        var_i4_cor_dn2 = 0.0;
        var_i4_cor_dn3 = 0.0;
        var_i4_cor_dn4 = 0.0;
        var_i4_cor_dn5 = 0.0;
        var_i4_cor_dn6 = 0.0;
        var_i4_cor_dn7 = 0.0;
        var_i4_cor_dn8 = 0.0;
        var_i4_cor_dn9 = 0.0;
        var_i4_cor_dn10 = 0.0;
        var_i4_cor_dn11 = 0.0;
        var_i4_cor_dn12 = 0.0;
        var_i4_cor_dn13 = 0.0;
        var_i4_cor_dn14 = 0.0;
        var_i4_cor_dn15 = 0.0;
        var_i4_cor_dn16 = 0.0;
        var_i4_cor_dn17 = 0.0;
        var_i4_cor_dn18 = 0.0;
        var_i4_cor_dn19 = 0.0;
        var_i4_cor_dn20 = 0.0;
        var_i4_cor_db0 = 0.0;
        var_i4_cor_db1 = 0.0;
        var_i4_cor_db2 = 0.0;
        var_i4_cor_db3 = 0.0;
        var_i4_cor_db4 = 0.0;
        var_i4_cor_db5 = 0.0;
        var_i4_cor_db6 = 0.0;
        var_i4_cor_db7 = 0.0;
        var_i4_cor_db8 = 0.0;
        var_i4_cor_db9 = 0.0;
        var_i4_cor_db10 = 0.0;
        var_i4_cor_db11 = 0.0;
        var_i4_cor_db12 = 0.0;
        var_i4_cor_db13 = 0.0;
        var_i4_cor_db14 = 0.0;
        var_i4_cor_db15 = 0.0;
        var_i4_cor_db16 = 0.0;
        var_i4_cor_db17 = 0.0;
        var_i4_cor_db18 = 0.0;
        var_i4_cor_db19 = 0.0;
        var_i4_cor_db20 = 0.0;
        var_i4_cor_db21 = 0.0;
        var_i4_cor_db22 = 0.0;
        var_i4_cor_db23 = 0.0;
        var_i4_cor_db24 = 0.0;

        var_i5_cor = 0.0;
        var_i5_cor_dn0 = 0.0;
        var_i5_cor_dn1 = 0.0;
        var_i5_cor_dn2 = 0.0;
        var_i5_cor_dn3 = 0.0;
        var_i5_cor_dn4 = 0.0;
        var_i5_cor_dn5 = 0.0;
        var_i5_cor_dn6 = 0.0;
        var_i5_cor_dn7 = 0.0;
        var_i5_cor_dn8 = 0.0;
        var_i5_cor_dn9 = 0.0;
        var_i5_cor_dn10 = 0.0;
        var_i5_cor_dn11 = 0.0;
        var_i5_cor_dn12 = 0.0;
        var_i5_cor_dn13 = 0.0;
        var_i5_cor_dn14 = 0.0;
        var_i5_cor_dn15 = 0.0;
        var_i5_cor_dn16 = 0.0;
        var_i5_cor_dn17 = 0.0;
        var_i5_cor_dn18 = 0.0;
        var_i5_cor_dn19 = 0.0;
        var_i5_cor_dn20 = 0.0;
        var_i5_cor_db0 = 0.0;
        var_i5_cor_db1 = 0.0;
        var_i5_cor_db2 = 0.0;
        var_i5_cor_db3 = 0.0;
        var_i5_cor_db4 = 0.0;
        var_i5_cor_db5 = 0.0;
        var_i5_cor_db6 = 0.0;
        var_i5_cor_db7 = 0.0;
        var_i5_cor_db8 = 0.0;
        var_i5_cor_db9 = 0.0;
        var_i5_cor_db10 = 0.0;
        var_i5_cor_db11 = 0.0;
        var_i5_cor_db12 = 0.0;
        var_i5_cor_db13 = 0.0;
        var_i5_cor_db14 = 0.0;
        var_i5_cor_db15 = 0.0;
        var_i5_cor_db16 = 0.0;
        var_i5_cor_db17 = 0.0;
        var_i5_cor_db18 = 0.0;
        var_i5_cor_db19 = 0.0;
        var_i5_cor_db20 = 0.0;
        var_i5_cor_db21 = 0.0;
        var_i5_cor_db22 = 0.0;
        var_i5_cor_db23 = 0.0;
        var_i5_cor_db24 = 0.0;

        var_tt0 = 0.0;

        var_tt1 = 0.0;
        var_tt1_dn0 = 0.0;
        var_tt1_dn1 = 0.0;
        var_tt1_dn2 = 0.0;
        var_tt1_dn3 = 0.0;
        var_tt1_dn4 = 0.0;
        var_tt1_dn5 = 0.0;
        var_tt1_dn6 = 0.0;
        var_tt1_dn7 = 0.0;
        var_tt1_dn8 = 0.0;
        var_tt1_dn9 = 0.0;
        var_tt1_dn10 = 0.0;
        var_tt1_dn11 = 0.0;
        var_tt1_dn12 = 0.0;
        var_tt1_dn13 = 0.0;
        var_tt1_dn14 = 0.0;
        var_tt1_dn15 = 0.0;
        var_tt1_dn16 = 0.0;
        var_tt1_dn17 = 0.0;
        var_tt1_dn18 = 0.0;
        var_tt1_dn19 = 0.0;
        var_tt1_dn20 = 0.0;
        var_tt1_db0 = 0.0;
        var_tt1_db1 = 0.0;
        var_tt1_db2 = 0.0;
        var_tt1_db3 = 0.0;
        var_tt1_db4 = 0.0;
        var_tt1_db5 = 0.0;
        var_tt1_db6 = 0.0;
        var_tt1_db7 = 0.0;
        var_tt1_db8 = 0.0;
        var_tt1_db9 = 0.0;
        var_tt1_db10 = 0.0;
        var_tt1_db11 = 0.0;
        var_tt1_db12 = 0.0;
        var_tt1_db13 = 0.0;
        var_tt1_db14 = 0.0;
        var_tt1_db15 = 0.0;
        var_tt1_db16 = 0.0;
        var_tt1_db17 = 0.0;
        var_tt1_db18 = 0.0;
        var_tt1_db19 = 0.0;
        var_tt1_db20 = 0.0;
        var_tt1_db21 = 0.0;
        var_tt1_db22 = 0.0;
        var_tt1_db23 = 0.0;
        var_tt1_db24 = 0.0;

        *var_expxhr_d_slot = var_expxhr_d;
        *var_expxhr_d_db0_slot = var_expxhr_d_db0;
        *var_expxhr_d_db1_slot = var_expxhr_d_db1;
        *var_expxhr_d_db10_slot = var_expxhr_d_db10;
        *var_expxhr_d_db11_slot = var_expxhr_d_db11;
        *var_expxhr_d_db12_slot = var_expxhr_d_db12;
        *var_expxhr_d_db13_slot = var_expxhr_d_db13;
        *var_expxhr_d_db14_slot = var_expxhr_d_db14;
        *var_expxhr_d_db15_slot = var_expxhr_d_db15;
        *var_expxhr_d_db16_slot = var_expxhr_d_db16;
        *var_expxhr_d_db17_slot = var_expxhr_d_db17;
        *var_expxhr_d_db18_slot = var_expxhr_d_db18;
        *var_expxhr_d_db19_slot = var_expxhr_d_db19;
        *var_expxhr_d_db2_slot = var_expxhr_d_db2;
        *var_expxhr_d_db20_slot = var_expxhr_d_db20;
        *var_expxhr_d_db21_slot = var_expxhr_d_db21;
        *var_expxhr_d_db22_slot = var_expxhr_d_db22;
        *var_expxhr_d_db23_slot = var_expxhr_d_db23;
        *var_expxhr_d_db24_slot = var_expxhr_d_db24;
        *var_expxhr_d_db3_slot = var_expxhr_d_db3;
        *var_expxhr_d_db4_slot = var_expxhr_d_db4;
        *var_expxhr_d_db5_slot = var_expxhr_d_db5;
        *var_expxhr_d_db6_slot = var_expxhr_d_db6;
        *var_expxhr_d_db7_slot = var_expxhr_d_db7;
        *var_expxhr_d_db8_slot = var_expxhr_d_db8;
        *var_expxhr_d_db9_slot = var_expxhr_d_db9;
        *var_expxhr_d_dn0_slot = var_expxhr_d_dn0;
        *var_expxhr_d_dn1_slot = var_expxhr_d_dn1;
        *var_expxhr_d_dn10_slot = var_expxhr_d_dn10;
        *var_expxhr_d_dn11_slot = var_expxhr_d_dn11;
        *var_expxhr_d_dn12_slot = var_expxhr_d_dn12;
        *var_expxhr_d_dn13_slot = var_expxhr_d_dn13;
        *var_expxhr_d_dn14_slot = var_expxhr_d_dn14;
        *var_expxhr_d_dn15_slot = var_expxhr_d_dn15;
        *var_expxhr_d_dn16_slot = var_expxhr_d_dn16;
        *var_expxhr_d_dn17_slot = var_expxhr_d_dn17;
        *var_expxhr_d_dn18_slot = var_expxhr_d_dn18;
        *var_expxhr_d_dn19_slot = var_expxhr_d_dn19;
        *var_expxhr_d_dn2_slot = var_expxhr_d_dn2;
        *var_expxhr_d_dn20_slot = var_expxhr_d_dn20;
        *var_expxhr_d_dn3_slot = var_expxhr_d_dn3;
        *var_expxhr_d_dn4_slot = var_expxhr_d_dn4;
        *var_expxhr_d_dn5_slot = var_expxhr_d_dn5;
        *var_expxhr_d_dn6_slot = var_expxhr_d_dn6;
        *var_expxhr_d_dn7_slot = var_expxhr_d_dn7;
        *var_expxhr_d_dn8_slot = var_expxhr_d_dn8;
        *var_expxhr_d_dn9_slot = var_expxhr_d_dn9;
        *var_expxhr_s_slot = var_expxhr_s;
        *var_expxhr_s_db0_slot = var_expxhr_s_db0;
        *var_expxhr_s_db1_slot = var_expxhr_s_db1;
        *var_expxhr_s_db10_slot = var_expxhr_s_db10;
        *var_expxhr_s_db11_slot = var_expxhr_s_db11;
        *var_expxhr_s_db12_slot = var_expxhr_s_db12;
        *var_expxhr_s_db13_slot = var_expxhr_s_db13;
        *var_expxhr_s_db14_slot = var_expxhr_s_db14;
        *var_expxhr_s_db15_slot = var_expxhr_s_db15;
        *var_expxhr_s_db16_slot = var_expxhr_s_db16;
        *var_expxhr_s_db17_slot = var_expxhr_s_db17;
        *var_expxhr_s_db18_slot = var_expxhr_s_db18;
        *var_expxhr_s_db19_slot = var_expxhr_s_db19;
        *var_expxhr_s_db2_slot = var_expxhr_s_db2;
        *var_expxhr_s_db20_slot = var_expxhr_s_db20;
        *var_expxhr_s_db21_slot = var_expxhr_s_db21;
        *var_expxhr_s_db22_slot = var_expxhr_s_db22;
        *var_expxhr_s_db23_slot = var_expxhr_s_db23;
        *var_expxhr_s_db24_slot = var_expxhr_s_db24;
        *var_expxhr_s_db3_slot = var_expxhr_s_db3;
        *var_expxhr_s_db4_slot = var_expxhr_s_db4;
        *var_expxhr_s_db5_slot = var_expxhr_s_db5;
        *var_expxhr_s_db6_slot = var_expxhr_s_db6;
        *var_expxhr_s_db7_slot = var_expxhr_s_db7;
        *var_expxhr_s_db8_slot = var_expxhr_s_db8;
        *var_expxhr_s_db9_slot = var_expxhr_s_db9;
        *var_expxhr_s_dn0_slot = var_expxhr_s_dn0;
        *var_expxhr_s_dn1_slot = var_expxhr_s_dn1;
        *var_expxhr_s_dn10_slot = var_expxhr_s_dn10;
        *var_expxhr_s_dn11_slot = var_expxhr_s_dn11;
        *var_expxhr_s_dn12_slot = var_expxhr_s_dn12;
        *var_expxhr_s_dn13_slot = var_expxhr_s_dn13;
        *var_expxhr_s_dn14_slot = var_expxhr_s_dn14;
        *var_expxhr_s_dn15_slot = var_expxhr_s_dn15;
        *var_expxhr_s_dn16_slot = var_expxhr_s_dn16;
        *var_expxhr_s_dn17_slot = var_expxhr_s_dn17;
        *var_expxhr_s_dn18_slot = var_expxhr_s_dn18;
        *var_expxhr_s_dn19_slot = var_expxhr_s_dn19;
        *var_expxhr_s_dn2_slot = var_expxhr_s_dn2;
        *var_expxhr_s_dn20_slot = var_expxhr_s_dn20;
        *var_expxhr_s_dn3_slot = var_expxhr_s_dn3;
        *var_expxhr_s_dn4_slot = var_expxhr_s_dn4;
        *var_expxhr_s_dn5_slot = var_expxhr_s_dn5;
        *var_expxhr_s_dn6_slot = var_expxhr_s_dn6;
        *var_expxhr_s_dn7_slot = var_expxhr_s_dn7;
        *var_expxhr_s_dn8_slot = var_expxhr_s_dn8;
        *var_expxhr_s_dn9_slot = var_expxhr_s_dn9;
        *var_i1_cor_slot = var_i1_cor;
        *var_i1_cor_db0_slot = var_i1_cor_db0;
        *var_i1_cor_db1_slot = var_i1_cor_db1;
        *var_i1_cor_db10_slot = var_i1_cor_db10;
        *var_i1_cor_db11_slot = var_i1_cor_db11;
        *var_i1_cor_db12_slot = var_i1_cor_db12;
        *var_i1_cor_db13_slot = var_i1_cor_db13;
        *var_i1_cor_db14_slot = var_i1_cor_db14;
        *var_i1_cor_db15_slot = var_i1_cor_db15;
        *var_i1_cor_db16_slot = var_i1_cor_db16;
        *var_i1_cor_db17_slot = var_i1_cor_db17;
        *var_i1_cor_db18_slot = var_i1_cor_db18;
        *var_i1_cor_db19_slot = var_i1_cor_db19;
        *var_i1_cor_db2_slot = var_i1_cor_db2;
        *var_i1_cor_db20_slot = var_i1_cor_db20;
        *var_i1_cor_db21_slot = var_i1_cor_db21;
        *var_i1_cor_db22_slot = var_i1_cor_db22;
        *var_i1_cor_db23_slot = var_i1_cor_db23;
        *var_i1_cor_db24_slot = var_i1_cor_db24;
        *var_i1_cor_db3_slot = var_i1_cor_db3;
        *var_i1_cor_db4_slot = var_i1_cor_db4;
        *var_i1_cor_db5_slot = var_i1_cor_db5;
        *var_i1_cor_db6_slot = var_i1_cor_db6;
        *var_i1_cor_db7_slot = var_i1_cor_db7;
        *var_i1_cor_db8_slot = var_i1_cor_db8;
        *var_i1_cor_db9_slot = var_i1_cor_db9;
        *var_i1_cor_dn0_slot = var_i1_cor_dn0;
        *var_i1_cor_dn1_slot = var_i1_cor_dn1;
        *var_i1_cor_dn10_slot = var_i1_cor_dn10;
        *var_i1_cor_dn11_slot = var_i1_cor_dn11;
        *var_i1_cor_dn12_slot = var_i1_cor_dn12;
        *var_i1_cor_dn13_slot = var_i1_cor_dn13;
        *var_i1_cor_dn14_slot = var_i1_cor_dn14;
        *var_i1_cor_dn15_slot = var_i1_cor_dn15;
        *var_i1_cor_dn16_slot = var_i1_cor_dn16;
        *var_i1_cor_dn17_slot = var_i1_cor_dn17;
        *var_i1_cor_dn18_slot = var_i1_cor_dn18;
        *var_i1_cor_dn19_slot = var_i1_cor_dn19;
        *var_i1_cor_dn2_slot = var_i1_cor_dn2;
        *var_i1_cor_dn20_slot = var_i1_cor_dn20;
        *var_i1_cor_dn3_slot = var_i1_cor_dn3;
        *var_i1_cor_dn4_slot = var_i1_cor_dn4;
        *var_i1_cor_dn5_slot = var_i1_cor_dn5;
        *var_i1_cor_dn6_slot = var_i1_cor_dn6;
        *var_i1_cor_dn7_slot = var_i1_cor_dn7;
        *var_i1_cor_dn8_slot = var_i1_cor_dn8;
        *var_i1_cor_dn9_slot = var_i1_cor_dn9;
        *var_i2_cor_slot = var_i2_cor;
        *var_i2_cor_db0_slot = var_i2_cor_db0;
        *var_i2_cor_db1_slot = var_i2_cor_db1;
        *var_i2_cor_db10_slot = var_i2_cor_db10;
        *var_i2_cor_db11_slot = var_i2_cor_db11;
        *var_i2_cor_db12_slot = var_i2_cor_db12;
        *var_i2_cor_db13_slot = var_i2_cor_db13;
        *var_i2_cor_db14_slot = var_i2_cor_db14;
        *var_i2_cor_db15_slot = var_i2_cor_db15;
        *var_i2_cor_db16_slot = var_i2_cor_db16;
        *var_i2_cor_db17_slot = var_i2_cor_db17;
        *var_i2_cor_db18_slot = var_i2_cor_db18;
        *var_i2_cor_db19_slot = var_i2_cor_db19;
        *var_i2_cor_db2_slot = var_i2_cor_db2;
        *var_i2_cor_db20_slot = var_i2_cor_db20;
        *var_i2_cor_db21_slot = var_i2_cor_db21;
        *var_i2_cor_db22_slot = var_i2_cor_db22;
        *var_i2_cor_db23_slot = var_i2_cor_db23;
        *var_i2_cor_db24_slot = var_i2_cor_db24;
        *var_i2_cor_db3_slot = var_i2_cor_db3;
        *var_i2_cor_db4_slot = var_i2_cor_db4;
        *var_i2_cor_db5_slot = var_i2_cor_db5;
        *var_i2_cor_db6_slot = var_i2_cor_db6;
        *var_i2_cor_db7_slot = var_i2_cor_db7;
        *var_i2_cor_db8_slot = var_i2_cor_db8;
        *var_i2_cor_db9_slot = var_i2_cor_db9;
        *var_i2_cor_dn0_slot = var_i2_cor_dn0;
        *var_i2_cor_dn1_slot = var_i2_cor_dn1;
        *var_i2_cor_dn10_slot = var_i2_cor_dn10;
        *var_i2_cor_dn11_slot = var_i2_cor_dn11;
        *var_i2_cor_dn12_slot = var_i2_cor_dn12;
        *var_i2_cor_dn13_slot = var_i2_cor_dn13;
        *var_i2_cor_dn14_slot = var_i2_cor_dn14;
        *var_i2_cor_dn15_slot = var_i2_cor_dn15;
        *var_i2_cor_dn16_slot = var_i2_cor_dn16;
        *var_i2_cor_dn17_slot = var_i2_cor_dn17;
        *var_i2_cor_dn18_slot = var_i2_cor_dn18;
        *var_i2_cor_dn19_slot = var_i2_cor_dn19;
        *var_i2_cor_dn2_slot = var_i2_cor_dn2;
        *var_i2_cor_dn20_slot = var_i2_cor_dn20;
        *var_i2_cor_dn3_slot = var_i2_cor_dn3;
        *var_i2_cor_dn4_slot = var_i2_cor_dn4;
        *var_i2_cor_dn5_slot = var_i2_cor_dn5;
        *var_i2_cor_dn6_slot = var_i2_cor_dn6;
        *var_i2_cor_dn7_slot = var_i2_cor_dn7;
        *var_i2_cor_dn8_slot = var_i2_cor_dn8;
        *var_i2_cor_dn9_slot = var_i2_cor_dn9;
        *var_i3_cor_slot = var_i3_cor;
        *var_i3_cor_db0_slot = var_i3_cor_db0;
        *var_i3_cor_db1_slot = var_i3_cor_db1;
        *var_i3_cor_db10_slot = var_i3_cor_db10;
        *var_i3_cor_db11_slot = var_i3_cor_db11;
        *var_i3_cor_db12_slot = var_i3_cor_db12;
        *var_i3_cor_db13_slot = var_i3_cor_db13;
        *var_i3_cor_db14_slot = var_i3_cor_db14;
        *var_i3_cor_db15_slot = var_i3_cor_db15;
        *var_i3_cor_db16_slot = var_i3_cor_db16;
        *var_i3_cor_db17_slot = var_i3_cor_db17;
        *var_i3_cor_db18_slot = var_i3_cor_db18;
        *var_i3_cor_db19_slot = var_i3_cor_db19;
        *var_i3_cor_db2_slot = var_i3_cor_db2;
        *var_i3_cor_db20_slot = var_i3_cor_db20;
        *var_i3_cor_db21_slot = var_i3_cor_db21;
        *var_i3_cor_db22_slot = var_i3_cor_db22;
        *var_i3_cor_db23_slot = var_i3_cor_db23;
        *var_i3_cor_db24_slot = var_i3_cor_db24;
        *var_i3_cor_db3_slot = var_i3_cor_db3;
        *var_i3_cor_db4_slot = var_i3_cor_db4;
        *var_i3_cor_db5_slot = var_i3_cor_db5;
        *var_i3_cor_db6_slot = var_i3_cor_db6;
        *var_i3_cor_db7_slot = var_i3_cor_db7;
        *var_i3_cor_db8_slot = var_i3_cor_db8;
        *var_i3_cor_db9_slot = var_i3_cor_db9;
        *var_i3_cor_dn0_slot = var_i3_cor_dn0;
        *var_i3_cor_dn1_slot = var_i3_cor_dn1;
        *var_i3_cor_dn10_slot = var_i3_cor_dn10;
        *var_i3_cor_dn11_slot = var_i3_cor_dn11;
        *var_i3_cor_dn12_slot = var_i3_cor_dn12;
        *var_i3_cor_dn13_slot = var_i3_cor_dn13;
        *var_i3_cor_dn14_slot = var_i3_cor_dn14;
        *var_i3_cor_dn15_slot = var_i3_cor_dn15;
        *var_i3_cor_dn16_slot = var_i3_cor_dn16;
        *var_i3_cor_dn17_slot = var_i3_cor_dn17;
        *var_i3_cor_dn18_slot = var_i3_cor_dn18;
        *var_i3_cor_dn19_slot = var_i3_cor_dn19;
        *var_i3_cor_dn2_slot = var_i3_cor_dn2;
        *var_i3_cor_dn20_slot = var_i3_cor_dn20;
        *var_i3_cor_dn3_slot = var_i3_cor_dn3;
        *var_i3_cor_dn4_slot = var_i3_cor_dn4;
        *var_i3_cor_dn5_slot = var_i3_cor_dn5;
        *var_i3_cor_dn6_slot = var_i3_cor_dn6;
        *var_i3_cor_dn7_slot = var_i3_cor_dn7;
        *var_i3_cor_dn8_slot = var_i3_cor_dn8;
        *var_i3_cor_dn9_slot = var_i3_cor_dn9;
        *var_i4_cor_slot = var_i4_cor;
        *var_i4_cor_db0_slot = var_i4_cor_db0;
        *var_i4_cor_db1_slot = var_i4_cor_db1;
        *var_i4_cor_db10_slot = var_i4_cor_db10;
        *var_i4_cor_db11_slot = var_i4_cor_db11;
        *var_i4_cor_db12_slot = var_i4_cor_db12;
        *var_i4_cor_db13_slot = var_i4_cor_db13;
        *var_i4_cor_db14_slot = var_i4_cor_db14;
        *var_i4_cor_db15_slot = var_i4_cor_db15;
        *var_i4_cor_db16_slot = var_i4_cor_db16;
        *var_i4_cor_db17_slot = var_i4_cor_db17;
        *var_i4_cor_db18_slot = var_i4_cor_db18;
        *var_i4_cor_db19_slot = var_i4_cor_db19;
        *var_i4_cor_db2_slot = var_i4_cor_db2;
        *var_i4_cor_db20_slot = var_i4_cor_db20;
        *var_i4_cor_db21_slot = var_i4_cor_db21;
        *var_i4_cor_db22_slot = var_i4_cor_db22;
        *var_i4_cor_db23_slot = var_i4_cor_db23;
        *var_i4_cor_db24_slot = var_i4_cor_db24;
        *var_i4_cor_db3_slot = var_i4_cor_db3;
        *var_i4_cor_db4_slot = var_i4_cor_db4;
        *var_i4_cor_db5_slot = var_i4_cor_db5;
        *var_i4_cor_db6_slot = var_i4_cor_db6;
        *var_i4_cor_db7_slot = var_i4_cor_db7;
        *var_i4_cor_db8_slot = var_i4_cor_db8;
        *var_i4_cor_db9_slot = var_i4_cor_db9;
        *var_i4_cor_dn0_slot = var_i4_cor_dn0;
        *var_i4_cor_dn1_slot = var_i4_cor_dn1;
        *var_i4_cor_dn10_slot = var_i4_cor_dn10;
        *var_i4_cor_dn11_slot = var_i4_cor_dn11;
        *var_i4_cor_dn12_slot = var_i4_cor_dn12;
        *var_i4_cor_dn13_slot = var_i4_cor_dn13;
        *var_i4_cor_dn14_slot = var_i4_cor_dn14;
        *var_i4_cor_dn15_slot = var_i4_cor_dn15;
        *var_i4_cor_dn16_slot = var_i4_cor_dn16;
        *var_i4_cor_dn17_slot = var_i4_cor_dn17;
        *var_i4_cor_dn18_slot = var_i4_cor_dn18;
        *var_i4_cor_dn19_slot = var_i4_cor_dn19;
        *var_i4_cor_dn2_slot = var_i4_cor_dn2;
        *var_i4_cor_dn20_slot = var_i4_cor_dn20;
        *var_i4_cor_dn3_slot = var_i4_cor_dn3;
        *var_i4_cor_dn4_slot = var_i4_cor_dn4;
        *var_i4_cor_dn5_slot = var_i4_cor_dn5;
        *var_i4_cor_dn6_slot = var_i4_cor_dn6;
        *var_i4_cor_dn7_slot = var_i4_cor_dn7;
        *var_i4_cor_dn8_slot = var_i4_cor_dn8;
        *var_i4_cor_dn9_slot = var_i4_cor_dn9;
        *var_i5_cor_slot = var_i5_cor;
        *var_i5_cor_db0_slot = var_i5_cor_db0;
        *var_i5_cor_db1_slot = var_i5_cor_db1;
        *var_i5_cor_db10_slot = var_i5_cor_db10;
        *var_i5_cor_db11_slot = var_i5_cor_db11;
        *var_i5_cor_db12_slot = var_i5_cor_db12;
        *var_i5_cor_db13_slot = var_i5_cor_db13;
        *var_i5_cor_db14_slot = var_i5_cor_db14;
        *var_i5_cor_db15_slot = var_i5_cor_db15;
        *var_i5_cor_db16_slot = var_i5_cor_db16;
        *var_i5_cor_db17_slot = var_i5_cor_db17;
        *var_i5_cor_db18_slot = var_i5_cor_db18;
        *var_i5_cor_db19_slot = var_i5_cor_db19;
        *var_i5_cor_db2_slot = var_i5_cor_db2;
        *var_i5_cor_db20_slot = var_i5_cor_db20;
        *var_i5_cor_db21_slot = var_i5_cor_db21;
        *var_i5_cor_db22_slot = var_i5_cor_db22;
        *var_i5_cor_db23_slot = var_i5_cor_db23;
        *var_i5_cor_db24_slot = var_i5_cor_db24;
        *var_i5_cor_db3_slot = var_i5_cor_db3;
        *var_i5_cor_db4_slot = var_i5_cor_db4;
        *var_i5_cor_db5_slot = var_i5_cor_db5;
        *var_i5_cor_db6_slot = var_i5_cor_db6;
        *var_i5_cor_db7_slot = var_i5_cor_db7;
        *var_i5_cor_db8_slot = var_i5_cor_db8;
        *var_i5_cor_db9_slot = var_i5_cor_db9;
        *var_i5_cor_dn0_slot = var_i5_cor_dn0;
        *var_i5_cor_dn1_slot = var_i5_cor_dn1;
        *var_i5_cor_dn10_slot = var_i5_cor_dn10;
        *var_i5_cor_dn11_slot = var_i5_cor_dn11;
        *var_i5_cor_dn12_slot = var_i5_cor_dn12;
        *var_i5_cor_dn13_slot = var_i5_cor_dn13;
        *var_i5_cor_dn14_slot = var_i5_cor_dn14;
        *var_i5_cor_dn15_slot = var_i5_cor_dn15;
        *var_i5_cor_dn16_slot = var_i5_cor_dn16;
        *var_i5_cor_dn17_slot = var_i5_cor_dn17;
        *var_i5_cor_dn18_slot = var_i5_cor_dn18;
        *var_i5_cor_dn19_slot = var_i5_cor_dn19;
        *var_i5_cor_dn2_slot = var_i5_cor_dn2;
        *var_i5_cor_dn20_slot = var_i5_cor_dn20;
        *var_i5_cor_dn3_slot = var_i5_cor_dn3;
        *var_i5_cor_dn4_slot = var_i5_cor_dn4;
        *var_i5_cor_dn5_slot = var_i5_cor_dn5;
        *var_i5_cor_dn6_slot = var_i5_cor_dn6;
        *var_i5_cor_dn7_slot = var_i5_cor_dn7;
        *var_i5_cor_dn8_slot = var_i5_cor_dn8;
        *var_i5_cor_dn9_slot = var_i5_cor_dn9;
        *var_m0_rev_slot = var_m0_rev;
        *var_m0_rev_db0_slot = var_m0_rev_db0;
        *var_m0_rev_db1_slot = var_m0_rev_db1;
        *var_m0_rev_db10_slot = var_m0_rev_db10;
        *var_m0_rev_db11_slot = var_m0_rev_db11;
        *var_m0_rev_db12_slot = var_m0_rev_db12;
        *var_m0_rev_db13_slot = var_m0_rev_db13;
        *var_m0_rev_db14_slot = var_m0_rev_db14;
        *var_m0_rev_db15_slot = var_m0_rev_db15;
        *var_m0_rev_db16_slot = var_m0_rev_db16;
        *var_m0_rev_db17_slot = var_m0_rev_db17;
        *var_m0_rev_db18_slot = var_m0_rev_db18;
        *var_m0_rev_db19_slot = var_m0_rev_db19;
        *var_m0_rev_db2_slot = var_m0_rev_db2;
        *var_m0_rev_db20_slot = var_m0_rev_db20;
        *var_m0_rev_db21_slot = var_m0_rev_db21;
        *var_m0_rev_db22_slot = var_m0_rev_db22;
        *var_m0_rev_db23_slot = var_m0_rev_db23;
        *var_m0_rev_db24_slot = var_m0_rev_db24;
        *var_m0_rev_db3_slot = var_m0_rev_db3;
        *var_m0_rev_db4_slot = var_m0_rev_db4;
        *var_m0_rev_db5_slot = var_m0_rev_db5;
        *var_m0_rev_db6_slot = var_m0_rev_db6;
        *var_m0_rev_db7_slot = var_m0_rev_db7;
        *var_m0_rev_db8_slot = var_m0_rev_db8;
        *var_m0_rev_db9_slot = var_m0_rev_db9;
        *var_m0_rev_dn0_slot = var_m0_rev_dn0;
        *var_m0_rev_dn1_slot = var_m0_rev_dn1;
        *var_m0_rev_dn10_slot = var_m0_rev_dn10;
        *var_m0_rev_dn11_slot = var_m0_rev_dn11;
        *var_m0_rev_dn12_slot = var_m0_rev_dn12;
        *var_m0_rev_dn13_slot = var_m0_rev_dn13;
        *var_m0_rev_dn14_slot = var_m0_rev_dn14;
        *var_m0_rev_dn15_slot = var_m0_rev_dn15;
        *var_m0_rev_dn16_slot = var_m0_rev_dn16;
        *var_m0_rev_dn17_slot = var_m0_rev_dn17;
        *var_m0_rev_dn18_slot = var_m0_rev_dn18;
        *var_m0_rev_dn19_slot = var_m0_rev_dn19;
        *var_m0_rev_dn2_slot = var_m0_rev_dn2;
        *var_m0_rev_dn20_slot = var_m0_rev_dn20;
        *var_m0_rev_dn3_slot = var_m0_rev_dn3;
        *var_m0_rev_dn4_slot = var_m0_rev_dn4;
        *var_m0_rev_dn5_slot = var_m0_rev_dn5;
        *var_m0_rev_dn6_slot = var_m0_rev_dn6;
        *var_m0_rev_dn7_slot = var_m0_rev_dn7;
        *var_m0_rev_dn8_slot = var_m0_rev_dn8;
        *var_m0_rev_dn9_slot = var_m0_rev_dn9;
        *var_mcor_rev_slot = var_mcor_rev;
        *var_mcor_rev_db0_slot = var_mcor_rev_db0;
        *var_mcor_rev_db1_slot = var_mcor_rev_db1;
        *var_mcor_rev_db10_slot = var_mcor_rev_db10;
        *var_mcor_rev_db11_slot = var_mcor_rev_db11;
        *var_mcor_rev_db12_slot = var_mcor_rev_db12;
        *var_mcor_rev_db13_slot = var_mcor_rev_db13;
        *var_mcor_rev_db14_slot = var_mcor_rev_db14;
        *var_mcor_rev_db15_slot = var_mcor_rev_db15;
        *var_mcor_rev_db16_slot = var_mcor_rev_db16;
        *var_mcor_rev_db17_slot = var_mcor_rev_db17;
        *var_mcor_rev_db18_slot = var_mcor_rev_db18;
        *var_mcor_rev_db19_slot = var_mcor_rev_db19;
        *var_mcor_rev_db2_slot = var_mcor_rev_db2;
        *var_mcor_rev_db20_slot = var_mcor_rev_db20;
        *var_mcor_rev_db21_slot = var_mcor_rev_db21;
        *var_mcor_rev_db22_slot = var_mcor_rev_db22;
        *var_mcor_rev_db23_slot = var_mcor_rev_db23;
        *var_mcor_rev_db24_slot = var_mcor_rev_db24;
        *var_mcor_rev_db3_slot = var_mcor_rev_db3;
        *var_mcor_rev_db4_slot = var_mcor_rev_db4;
        *var_mcor_rev_db5_slot = var_mcor_rev_db5;
        *var_mcor_rev_db6_slot = var_mcor_rev_db6;
        *var_mcor_rev_db7_slot = var_mcor_rev_db7;
        *var_mcor_rev_db8_slot = var_mcor_rev_db8;
        *var_mcor_rev_db9_slot = var_mcor_rev_db9;
        *var_mcor_rev_dn0_slot = var_mcor_rev_dn0;
        *var_mcor_rev_dn1_slot = var_mcor_rev_dn1;
        *var_mcor_rev_dn10_slot = var_mcor_rev_dn10;
        *var_mcor_rev_dn11_slot = var_mcor_rev_dn11;
        *var_mcor_rev_dn12_slot = var_mcor_rev_dn12;
        *var_mcor_rev_dn13_slot = var_mcor_rev_dn13;
        *var_mcor_rev_dn14_slot = var_mcor_rev_dn14;
        *var_mcor_rev_dn15_slot = var_mcor_rev_dn15;
        *var_mcor_rev_dn16_slot = var_mcor_rev_dn16;
        *var_mcor_rev_dn17_slot = var_mcor_rev_dn17;
        *var_mcor_rev_dn18_slot = var_mcor_rev_dn18;
        *var_mcor_rev_dn19_slot = var_mcor_rev_dn19;
        *var_mcor_rev_dn2_slot = var_mcor_rev_dn2;
        *var_mcor_rev_dn20_slot = var_mcor_rev_dn20;
        *var_mcor_rev_dn3_slot = var_mcor_rev_dn3;
        *var_mcor_rev_dn4_slot = var_mcor_rev_dn4;
        *var_mcor_rev_dn5_slot = var_mcor_rev_dn5;
        *var_mcor_rev_dn6_slot = var_mcor_rev_dn6;
        *var_mcor_rev_dn7_slot = var_mcor_rev_dn7;
        *var_mcor_rev_dn8_slot = var_mcor_rev_dn8;
        *var_mcor_rev_dn9_slot = var_mcor_rev_dn9;
        *var_tt0_slot = var_tt0;
        *var_tt1_slot = var_tt1;
        *var_tt1_db0_slot = var_tt1_db0;
        *var_tt1_db1_slot = var_tt1_db1;
        *var_tt1_db10_slot = var_tt1_db10;
        *var_tt1_db11_slot = var_tt1_db11;
        *var_tt1_db12_slot = var_tt1_db12;
        *var_tt1_db13_slot = var_tt1_db13;
        *var_tt1_db14_slot = var_tt1_db14;
        *var_tt1_db15_slot = var_tt1_db15;
        *var_tt1_db16_slot = var_tt1_db16;
        *var_tt1_db17_slot = var_tt1_db17;
        *var_tt1_db18_slot = var_tt1_db18;
        *var_tt1_db19_slot = var_tt1_db19;
        *var_tt1_db2_slot = var_tt1_db2;
        *var_tt1_db20_slot = var_tt1_db20;
        *var_tt1_db21_slot = var_tt1_db21;
        *var_tt1_db22_slot = var_tt1_db22;
        *var_tt1_db23_slot = var_tt1_db23;
        *var_tt1_db24_slot = var_tt1_db24;
        *var_tt1_db3_slot = var_tt1_db3;
        *var_tt1_db4_slot = var_tt1_db4;
        *var_tt1_db5_slot = var_tt1_db5;
        *var_tt1_db6_slot = var_tt1_db6;
        *var_tt1_db7_slot = var_tt1_db7;
        *var_tt1_db8_slot = var_tt1_db8;
        *var_tt1_db9_slot = var_tt1_db9;
        *var_tt1_dn0_slot = var_tt1_dn0;
        *var_tt1_dn1_slot = var_tt1_dn1;
        *var_tt1_dn10_slot = var_tt1_dn10;
        *var_tt1_dn11_slot = var_tt1_dn11;
        *var_tt1_dn12_slot = var_tt1_dn12;
        *var_tt1_dn13_slot = var_tt1_dn13;
        *var_tt1_dn14_slot = var_tt1_dn14;
        *var_tt1_dn15_slot = var_tt1_dn15;
        *var_tt1_dn16_slot = var_tt1_dn16;
        *var_tt1_dn17_slot = var_tt1_dn17;
        *var_tt1_dn18_slot = var_tt1_dn18;
        *var_tt1_dn19_slot = var_tt1_dn19;
        *var_tt1_dn2_slot = var_tt1_dn2;
        *var_tt1_dn20_slot = var_tt1_dn20;
        *var_tt1_dn3_slot = var_tt1_dn3;
        *var_tt1_dn4_slot = var_tt1_dn4;
        *var_tt1_dn5_slot = var_tt1_dn5;
        *var_tt1_dn6_slot = var_tt1_dn6;
        *var_tt1_dn7_slot = var_tt1_dn7;
        *var_tt1_dn8_slot = var_tt1_dn8;
        *var_tt1_dn9_slot = var_tt1_dn9;
        *var_zflagbot_d_slot = var_zflagbot_d;
        *var_zflagbot_s_slot = var_zflagbot_s;
        *var_zflaggat_d_slot = var_zflaggat_d;
        *var_zflaggat_s_slot = var_zflaggat_s;
        *var_zflagsti_d_slot = var_zflagsti_d;
        *var_zflagsti_s_slot = var_zflagsti_s;
    }
}
