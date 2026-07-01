#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_128(
        p: &Parameters,
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
        var_mode: f64,
        var_noicross: f64,
        var_noicross_dn0: f64,
        var_noicross_dn10: f64,
        var_noicross_dn11: f64,
        var_noicross_dn12: f64,
        var_noicross_dn17: f64,
        var_noicross_dn2: f64,
        var_noicross_dn6: f64,
        var_noicross_dn7: f64,
        var_noiigate: f64,
        var_noiigate_dn0: f64,
        var_noiigate_dn10: f64,
        var_noiigate_dn11: f64,
        var_noiigate_dn12: f64,
        var_noiigate_dn13: f64,
        var_noiigate_dn15: f64,
        var_noiigate_dn16: f64,
        var_noiigate_dn17: f64,
        var_noiigate_dn18: f64,
        var_noiigate_dn2: f64,
        var_noiigate_dn6: f64,
        var_noiigate_dn7: f64,
        var_noithrml: f64,
        var_noithrml_dn0: f64,
        var_noithrml_dn10: f64,
        var_noithrml_dn11: f64,
        var_noithrml_dn12: f64,
        var_noithrml_dn17: f64,
        var_noithrml_dn2: f64,
        var_noithrml_dn6: f64,
        var_noithrml_dn7: f64,
        var_qdrat_noi: f64,
        var_qdrat_noi_dn0: f64,
        var_qdrat_noi_dn10: f64,
        var_qdrat_noi_dn11: f64,
        var_qdrat_noi_dn12: f64,
        var_qdrat_noi_dn17: f64,
        var_qdrat_noi_dn2: f64,
        var_qdrat_noi_dn6: f64,
        var_qdrat_noi_dn7: f64,
        var_qg_dn7: f64,
        var_rpower: f64,
        var_rpower_dn0: f64,
        var_rpower_dn10: f64,
        var_rpower_dn11: f64,
        var_rpower_dn12: f64,
        var_rpower_dn17: f64,
        var_rpower_dn2: f64,
        var_rpower_dn6: f64,
        var_rpower_dn7: f64,
        var_ttemp: f64,
        var_ttemp_dn10: f64,
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
        var_ci_slot: &mut f64,
        var_ci_dn0_slot: &mut f64,
        var_ci_dn10_slot: &mut f64,
        var_ci_dn11_slot: &mut f64,
        var_ci_dn12_slot: &mut f64,
        var_ci_dn17_slot: &mut f64,
        var_ci_dn2_slot: &mut f64,
        var_ci_dn6_slot: &mut f64,
        var_ci_dn7_slot: &mut f64,
        var_guard1218_slot: &mut f64,
        var_guard1224_slot: &mut f64,
        var_guard1226_slot: &mut f64,
        var_guard1227_slot: &mut f64,
        var_guard1228_slot: &mut f64,
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
        var_itemp_slot: &mut f64,
        var_itemp_dn0_slot: &mut f64,
        var_itemp_dn10_slot: &mut f64,
        var_itemp_dn11_slot: &mut f64,
        var_itemp_dn12_slot: &mut f64,
        var_itemp_dn17_slot: &mut f64,
        var_itemp_dn2_slot: &mut f64,
        var_itemp_dn6_slot: &mut f64,
        var_itemp_dn7_slot: &mut f64,
        var_qdrat_slot: &mut f64,
        var_qdrat_dn0_slot: &mut f64,
        var_qdrat_dn10_slot: &mut f64,
        var_qdrat_dn11_slot: &mut f64,
        var_qdrat_dn12_slot: &mut f64,
        var_qdrat_dn17_slot: &mut f64,
        var_qdrat_dn2_slot: &mut f64,
        var_qdrat_dn6_slot: &mut f64,
        var_qdrat_dn7_slot: &mut f64,
        var_sid_slot: &mut f64,
        var_sid_dn0_slot: &mut f64,
        var_sid_dn10_slot: &mut f64,
        var_sid_dn11_slot: &mut f64,
        var_sid_dn12_slot: &mut f64,
        var_sid_dn17_slot: &mut f64,
        var_sid_dn2_slot: &mut f64,
        var_sid_dn6_slot: &mut f64,
        var_sid_dn7_slot: &mut f64,
        var_sigrat_slot: &mut f64,
        var_sigrat_d_slot: &mut f64,
        var_sigrat_d_dn0_slot: &mut f64,
        var_sigrat_d_dn10_slot: &mut f64,
        var_sigrat_d_dn11_slot: &mut f64,
        var_sigrat_d_dn12_slot: &mut f64,
        var_sigrat_d_dn13_slot: &mut f64,
        var_sigrat_d_dn15_slot: &mut f64,
        var_sigrat_d_dn16_slot: &mut f64,
        var_sigrat_d_dn17_slot: &mut f64,
        var_sigrat_d_dn18_slot: &mut f64,
        var_sigrat_d_dn2_slot: &mut f64,
        var_sigrat_d_dn6_slot: &mut f64,
        var_sigrat_d_dn7_slot: &mut f64,
        var_sigrat_dn0_slot: &mut f64,
        var_sigrat_dn10_slot: &mut f64,
        var_sigrat_dn11_slot: &mut f64,
        var_sigrat_dn12_slot: &mut f64,
        var_sigrat_dn13_slot: &mut f64,
        var_sigrat_dn15_slot: &mut f64,
        var_sigrat_dn16_slot: &mut f64,
        var_sigrat_dn17_slot: &mut f64,
        var_sigrat_dn18_slot: &mut f64,
        var_sigrat_dn2_slot: &mut f64,
        var_sigrat_dn6_slot: &mut f64,
        var_sigrat_dn7_slot: &mut f64,
        var_sigrat_s_slot: &mut f64,
        var_sigrat_s_dn0_slot: &mut f64,
        var_sigrat_s_dn10_slot: &mut f64,
        var_sigrat_s_dn11_slot: &mut f64,
        var_sigrat_s_dn12_slot: &mut f64,
        var_sigrat_s_dn13_slot: &mut f64,
        var_sigrat_s_dn15_slot: &mut f64,
        var_sigrat_s_dn16_slot: &mut f64,
        var_sigrat_s_dn17_slot: &mut f64,
        var_sigrat_s_dn18_slot: &mut f64,
        var_sigrat_s_dn2_slot: &mut f64,
        var_sigrat_s_dn6_slot: &mut f64,
        var_sigrat_s_dn7_slot: &mut f64,
        var_whi_noise_slot: &mut f64,
        var_whi_noise_dn10_slot: &mut f64,
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
        let mut var_ci: f64 = *var_ci_slot;
        let mut var_ci_dn0: f64 = *var_ci_dn0_slot;
        let mut var_ci_dn10: f64 = *var_ci_dn10_slot;
        let mut var_ci_dn11: f64 = *var_ci_dn11_slot;
        let mut var_ci_dn12: f64 = *var_ci_dn12_slot;
        let mut var_ci_dn17: f64 = *var_ci_dn17_slot;
        let mut var_ci_dn2: f64 = *var_ci_dn2_slot;
        let mut var_ci_dn6: f64 = *var_ci_dn6_slot;
        let mut var_ci_dn7: f64 = *var_ci_dn7_slot;
        let mut var_guard1218: f64 = *var_guard1218_slot;
        let mut var_guard1224: f64 = *var_guard1224_slot;
        let mut var_guard1226: f64 = *var_guard1226_slot;
        let mut var_guard1227: f64 = *var_guard1227_slot;
        let mut var_guard1228: f64 = *var_guard1228_slot;
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
        let mut var_itemp: f64 = *var_itemp_slot;
        let mut var_itemp_dn0: f64 = *var_itemp_dn0_slot;
        let mut var_itemp_dn10: f64 = *var_itemp_dn10_slot;
        let mut var_itemp_dn11: f64 = *var_itemp_dn11_slot;
        let mut var_itemp_dn12: f64 = *var_itemp_dn12_slot;
        let mut var_itemp_dn17: f64 = *var_itemp_dn17_slot;
        let mut var_itemp_dn2: f64 = *var_itemp_dn2_slot;
        let mut var_itemp_dn6: f64 = *var_itemp_dn6_slot;
        let mut var_itemp_dn7: f64 = *var_itemp_dn7_slot;
        let mut var_qdrat: f64 = *var_qdrat_slot;
        let mut var_qdrat_dn0: f64 = *var_qdrat_dn0_slot;
        let mut var_qdrat_dn10: f64 = *var_qdrat_dn10_slot;
        let mut var_qdrat_dn11: f64 = *var_qdrat_dn11_slot;
        let mut var_qdrat_dn12: f64 = *var_qdrat_dn12_slot;
        let mut var_qdrat_dn17: f64 = *var_qdrat_dn17_slot;
        let mut var_qdrat_dn2: f64 = *var_qdrat_dn2_slot;
        let mut var_qdrat_dn6: f64 = *var_qdrat_dn6_slot;
        let mut var_qdrat_dn7: f64 = *var_qdrat_dn7_slot;
        let mut var_sid: f64 = *var_sid_slot;
        let mut var_sid_dn0: f64 = *var_sid_dn0_slot;
        let mut var_sid_dn10: f64 = *var_sid_dn10_slot;
        let mut var_sid_dn11: f64 = *var_sid_dn11_slot;
        let mut var_sid_dn12: f64 = *var_sid_dn12_slot;
        let mut var_sid_dn17: f64 = *var_sid_dn17_slot;
        let mut var_sid_dn2: f64 = *var_sid_dn2_slot;
        let mut var_sid_dn6: f64 = *var_sid_dn6_slot;
        let mut var_sid_dn7: f64 = *var_sid_dn7_slot;
        let mut var_sigrat: f64 = *var_sigrat_slot;
        let mut var_sigrat_d: f64 = *var_sigrat_d_slot;
        let mut var_sigrat_d_dn0: f64 = *var_sigrat_d_dn0_slot;
        let mut var_sigrat_d_dn10: f64 = *var_sigrat_d_dn10_slot;
        let mut var_sigrat_d_dn11: f64 = *var_sigrat_d_dn11_slot;
        let mut var_sigrat_d_dn12: f64 = *var_sigrat_d_dn12_slot;
        let mut var_sigrat_d_dn13: f64 = *var_sigrat_d_dn13_slot;
        let mut var_sigrat_d_dn15: f64 = *var_sigrat_d_dn15_slot;
        let mut var_sigrat_d_dn16: f64 = *var_sigrat_d_dn16_slot;
        let mut var_sigrat_d_dn17: f64 = *var_sigrat_d_dn17_slot;
        let mut var_sigrat_d_dn18: f64 = *var_sigrat_d_dn18_slot;
        let mut var_sigrat_d_dn2: f64 = *var_sigrat_d_dn2_slot;
        let mut var_sigrat_d_dn6: f64 = *var_sigrat_d_dn6_slot;
        let mut var_sigrat_d_dn7: f64 = *var_sigrat_d_dn7_slot;
        let mut var_sigrat_dn0: f64 = *var_sigrat_dn0_slot;
        let mut var_sigrat_dn10: f64 = *var_sigrat_dn10_slot;
        let mut var_sigrat_dn11: f64 = *var_sigrat_dn11_slot;
        let mut var_sigrat_dn12: f64 = *var_sigrat_dn12_slot;
        let mut var_sigrat_dn13: f64 = *var_sigrat_dn13_slot;
        let mut var_sigrat_dn15: f64 = *var_sigrat_dn15_slot;
        let mut var_sigrat_dn16: f64 = *var_sigrat_dn16_slot;
        let mut var_sigrat_dn17: f64 = *var_sigrat_dn17_slot;
        let mut var_sigrat_dn18: f64 = *var_sigrat_dn18_slot;
        let mut var_sigrat_dn2: f64 = *var_sigrat_dn2_slot;
        let mut var_sigrat_dn6: f64 = *var_sigrat_dn6_slot;
        let mut var_sigrat_dn7: f64 = *var_sigrat_dn7_slot;
        let mut var_sigrat_s: f64 = *var_sigrat_s_slot;
        let mut var_sigrat_s_dn0: f64 = *var_sigrat_s_dn0_slot;
        let mut var_sigrat_s_dn10: f64 = *var_sigrat_s_dn10_slot;
        let mut var_sigrat_s_dn11: f64 = *var_sigrat_s_dn11_slot;
        let mut var_sigrat_s_dn12: f64 = *var_sigrat_s_dn12_slot;
        let mut var_sigrat_s_dn13: f64 = *var_sigrat_s_dn13_slot;
        let mut var_sigrat_s_dn15: f64 = *var_sigrat_s_dn15_slot;
        let mut var_sigrat_s_dn16: f64 = *var_sigrat_s_dn16_slot;
        let mut var_sigrat_s_dn17: f64 = *var_sigrat_s_dn17_slot;
        let mut var_sigrat_s_dn18: f64 = *var_sigrat_s_dn18_slot;
        let mut var_sigrat_s_dn2: f64 = *var_sigrat_s_dn2_slot;
        let mut var_sigrat_s_dn6: f64 = *var_sigrat_s_dn6_slot;
        let mut var_sigrat_s_dn7: f64 = *var_sigrat_s_dn7_slot;
        let mut var_whi_noise: f64 = *var_whi_noise_slot;
        let mut var_whi_noise_dn10: f64 = *var_whi_noise_dn10_slot;

        let assign37200_e51549: f64 = (p.p50 * var_cgdbd);
        var_cgdbd = assign37200_e51549;
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

        let assign37210_e51552: f64 = var_qg_dn7;
        var_cgsbd = assign37210_e51552;
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

        let assign37220_e51555: f64 = (p.p50 * var_cgsbd);
        var_cgsbd = assign37220_e51555;
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

        let assign37490_e51636: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1218 = assign37490_e51636;

        let (assign37500_e51642, assign37500_e51642_d_n0, assign37500_e51642_d_n2, assign37500_e51642_d_n6, assign37500_e51642_d_n7, assign37500_e51642_d_n10, assign37500_e51642_d_n11, assign37500_e51642_d_n12, assign37500_e51642_d_n17,) = {
    if (var_guard1218 != 0.0) {
        let assign37500_e51640: f64 = (p.p50 * var_ibd);
        (assign37500_e51640, (p.p50 * var_ibd_dn0), (p.p50 * var_ibd_dn2), (p.p50 * var_ibd_dn6), (p.p50 * var_ibd_dn7), (p.p50 * var_ibd_dn10), (p.p50 * var_ibd_dn11), (p.p50 * var_ibd_dn12), (p.p50 * var_ibd_dn17),)
    } else {
        (var_ibdb, var_ibdb_dn0, var_ibdb_dn2, var_ibdb_dn6, var_ibdb_dn7, var_ibdb_dn10, var_ibdb_dn11, var_ibdb_dn12, var_ibdb_dn17,)
    }
};
        var_ibdb = assign37500_e51642;
        var_ibdb_dn0 = assign37500_e51642_d_n0;
        var_ibdb_dn2 = assign37500_e51642_d_n2;
        var_ibdb_dn6 = assign37500_e51642_d_n6;
        var_ibdb_dn7 = assign37500_e51642_d_n7;
        var_ibdb_dn10 = assign37500_e51642_d_n10;
        var_ibdb_dn11 = assign37500_e51642_d_n11;
        var_ibdb_dn12 = assign37500_e51642_d_n12;
        var_ibdb_dn17 = assign37500_e51642_d_n17;

        let (assign37510_e51648, assign37510_e51648_d_n0, assign37510_e51648_d_n2, assign37510_e51648_d_n6, assign37510_e51648_d_n7, assign37510_e51648_d_n10, assign37510_e51648_d_n11, assign37510_e51648_d_n12, assign37510_e51648_d_n17,) = {
    if (var_guard1218 != 0.0) {
        let assign37510_e51646: f64 = (p.p50 * var_ibs);
        (assign37510_e51646, (p.p50 * var_ibs_dn0), (p.p50 * var_ibs_dn2), (p.p50 * var_ibs_dn6), (p.p50 * var_ibs_dn7), (p.p50 * var_ibs_dn10), (p.p50 * var_ibs_dn11), (p.p50 * var_ibs_dn12), (p.p50 * var_ibs_dn17),)
    } else {
        (var_ibsb, var_ibsb_dn0, var_ibsb_dn2, var_ibsb_dn6, var_ibsb_dn7, var_ibsb_dn10, var_ibsb_dn11, var_ibsb_dn12, var_ibsb_dn17,)
    }
};
        var_ibsb = assign37510_e51648;
        var_ibsb_dn0 = assign37510_e51648_d_n0;
        var_ibsb_dn2 = assign37510_e51648_d_n2;
        var_ibsb_dn6 = assign37510_e51648_d_n6;
        var_ibsb_dn7 = assign37510_e51648_d_n7;
        var_ibsb_dn10 = assign37510_e51648_d_n10;
        var_ibsb_dn11 = assign37510_e51648_d_n11;
        var_ibsb_dn12 = assign37510_e51648_d_n12;
        var_ibsb_dn17 = assign37510_e51648_d_n17;

        let assign37630_e51700: f64 = (4.0 * 1.3806226e-23);
        let assign37630_e51702: f64 = (assign37630_e51700 * var_ttemp);
        let assign37630_e51704: f64 = assign37630_e51702;
        var_whi_noise = assign37630_e51704;
        var_whi_noise_dn10 = (assign37630_e51700 * var_ttemp_dn10);

        let assign37640_e51707: f64 = if p.p27 == 1.0 { 1.0 } else { 0.0 };
        var_guard1224 = assign37640_e51707;

        var_qdrat = var_qdrat_noi;
        var_qdrat_dn0 = var_qdrat_noi_dn0;
        var_qdrat_dn2 = var_qdrat_noi_dn2;
        var_qdrat_dn6 = var_qdrat_noi_dn6;
        var_qdrat_dn7 = var_qdrat_noi_dn7;
        var_qdrat_dn10 = var_qdrat_noi_dn10;
        var_qdrat_dn11 = var_qdrat_noi_dn11;
        var_qdrat_dn12 = var_qdrat_noi_dn12;
        var_qdrat_dn17 = var_qdrat_noi_dn17;

        let assign37660_e51711: f64 = (var_whi_noise * var_noithrml);
        var_sid = assign37660_e51711;
        var_sid_dn0 = (var_whi_noise * var_noithrml_dn0);
        var_sid_dn2 = (var_whi_noise * var_noithrml_dn2);
        var_sid_dn6 = (var_whi_noise * var_noithrml_dn6);
        var_sid_dn7 = (var_whi_noise * var_noithrml_dn7);
        var_sid_dn10 = ((var_whi_noise_dn10 * var_noithrml) + (var_whi_noise * var_noithrml_dn10));
        var_sid_dn11 = (var_whi_noise * var_noithrml_dn11);
        var_sid_dn12 = (var_whi_noise * var_noithrml_dn12);
        var_sid_dn17 = (var_whi_noise * var_noithrml_dn17);

        var_ci = var_noicross;
        var_ci_dn0 = var_noicross_dn0;
        var_ci_dn2 = var_noicross_dn2;
        var_ci_dn6 = var_noicross_dn6;
        var_ci_dn7 = var_noicross_dn7;
        var_ci_dn10 = var_noicross_dn10;
        var_ci_dn11 = var_noicross_dn11;
        var_ci_dn12 = var_noicross_dn12;
        var_ci_dn17 = var_noicross_dn17;

        let (assign37680_e51725, assign37680_e51725_d_n0, assign37680_e51725_d_n2, assign37680_e51725_d_n6, assign37680_e51725_d_n7, assign37680_e51725_d_n10, assign37680_e51725_d_n11, assign37680_e51725_d_n12, assign37680_e51725_d_n13, assign37680_e51725_d_n15, assign37680_e51725_d_n16, assign37680_e51725_d_n17, assign37680_e51725_d_n18,) = {
    if ((var_sid > 0.0) && (var_noiigate > 0.0)) {
        let assign37680_e51722: f64 = (var_noiigate / var_sid);
        let assign37680_e51723: f64 = (assign37680_e51722).sqrt();
        (assign37680_e51723, ((((var_noiigate_dn0 * var_sid) - (var_noiigate * var_sid_dn0)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn2 * var_sid) - (var_noiigate * var_sid_dn2)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn6 * var_sid) - (var_noiigate * var_sid_dn6)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn7 * var_sid) - (var_noiigate * var_sid_dn7)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn10 * var_sid) - (var_noiigate * var_sid_dn10)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn11 * var_sid) - (var_noiigate * var_sid_dn11)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn12 * var_sid) - (var_noiigate * var_sid_dn12)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((var_noiigate_dn13 / var_sid) / (2.0 * assign37680_e51723)), ((var_noiigate_dn15 / var_sid) / (2.0 * assign37680_e51723)), ((var_noiigate_dn16 / var_sid) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn17 * var_sid) - (var_noiigate * var_sid_dn17)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((var_noiigate_dn18 / var_sid) / (2.0 * assign37680_e51723)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_sigrat = assign37680_e51725;
        var_sigrat_dn0 = assign37680_e51725_d_n0;
        var_sigrat_dn2 = assign37680_e51725_d_n2;
        var_sigrat_dn6 = assign37680_e51725_d_n6;
        var_sigrat_dn7 = assign37680_e51725_d_n7;
        var_sigrat_dn10 = assign37680_e51725_d_n10;
        var_sigrat_dn11 = assign37680_e51725_d_n11;
        var_sigrat_dn12 = assign37680_e51725_d_n12;
        var_sigrat_dn13 = assign37680_e51725_d_n13;
        var_sigrat_dn15 = assign37680_e51725_d_n15;
        var_sigrat_dn16 = assign37680_e51725_d_n16;
        var_sigrat_dn17 = assign37680_e51725_d_n17;
        var_sigrat_dn18 = assign37680_e51725_d_n18;

        let (assign37690_e51737, assign37690_e51737_d_n0, assign37690_e51737_d_n2, assign37690_e51737_d_n6, assign37690_e51737_d_n7, assign37690_e51737_d_n10, assign37690_e51737_d_n11, assign37690_e51737_d_n12, assign37690_e51737_d_n13, assign37690_e51737_d_n15, assign37690_e51737_d_n16, assign37690_e51737_d_n17, assign37690_e51737_d_n18,) = {
    if (var_mode > 0.0) {
        let assign37690_e51732: f64 = (1.0 - var_qdrat);
        let assign37690_e51733: f64 = (var_sigrat * assign37690_e51732);
        (assign37690_e51733, ((var_sigrat_dn0 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn0))), ((var_sigrat_dn2 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn2))), ((var_sigrat_dn6 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn6))), ((var_sigrat_dn7 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn7))), ((var_sigrat_dn10 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn10))), ((var_sigrat_dn11 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn11))), ((var_sigrat_dn12 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn12))), (var_sigrat_dn13 * assign37690_e51732), (var_sigrat_dn15 * assign37690_e51732), (var_sigrat_dn16 * assign37690_e51732), ((var_sigrat_dn17 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn17))), (var_sigrat_dn18 * assign37690_e51732),)
    } else {
        let assign37690_e51736: f64 = (var_sigrat * var_qdrat);
        (assign37690_e51736, ((var_sigrat_dn0 * var_qdrat) + (var_sigrat * var_qdrat_dn0)), ((var_sigrat_dn2 * var_qdrat) + (var_sigrat * var_qdrat_dn2)), ((var_sigrat_dn6 * var_qdrat) + (var_sigrat * var_qdrat_dn6)), ((var_sigrat_dn7 * var_qdrat) + (var_sigrat * var_qdrat_dn7)), ((var_sigrat_dn10 * var_qdrat) + (var_sigrat * var_qdrat_dn10)), ((var_sigrat_dn11 * var_qdrat) + (var_sigrat * var_qdrat_dn11)), ((var_sigrat_dn12 * var_qdrat) + (var_sigrat * var_qdrat_dn12)), (var_sigrat_dn13 * var_qdrat), (var_sigrat_dn15 * var_qdrat), (var_sigrat_dn16 * var_qdrat), ((var_sigrat_dn17 * var_qdrat) + (var_sigrat * var_qdrat_dn17)), (var_sigrat_dn18 * var_qdrat),)
    }
};
        var_sigrat_s = assign37690_e51737;
        var_sigrat_s_dn0 = assign37690_e51737_d_n0;
        var_sigrat_s_dn2 = assign37690_e51737_d_n2;
        var_sigrat_s_dn6 = assign37690_e51737_d_n6;
        var_sigrat_s_dn7 = assign37690_e51737_d_n7;
        var_sigrat_s_dn10 = assign37690_e51737_d_n10;
        var_sigrat_s_dn11 = assign37690_e51737_d_n11;
        var_sigrat_s_dn12 = assign37690_e51737_d_n12;
        var_sigrat_s_dn13 = assign37690_e51737_d_n13;
        var_sigrat_s_dn15 = assign37690_e51737_d_n15;
        var_sigrat_s_dn16 = assign37690_e51737_d_n16;
        var_sigrat_s_dn17 = assign37690_e51737_d_n17;
        var_sigrat_s_dn18 = assign37690_e51737_d_n18;

        let (assign37700_e51749, assign37700_e51749_d_n0, assign37700_e51749_d_n2, assign37700_e51749_d_n6, assign37700_e51749_d_n7, assign37700_e51749_d_n10, assign37700_e51749_d_n11, assign37700_e51749_d_n12, assign37700_e51749_d_n13, assign37700_e51749_d_n15, assign37700_e51749_d_n16, assign37700_e51749_d_n17, assign37700_e51749_d_n18,) = {
    if (var_mode > 0.0) {
        let assign37700_e51743: f64 = (var_sigrat * var_qdrat);
        (assign37700_e51743, ((var_sigrat_dn0 * var_qdrat) + (var_sigrat * var_qdrat_dn0)), ((var_sigrat_dn2 * var_qdrat) + (var_sigrat * var_qdrat_dn2)), ((var_sigrat_dn6 * var_qdrat) + (var_sigrat * var_qdrat_dn6)), ((var_sigrat_dn7 * var_qdrat) + (var_sigrat * var_qdrat_dn7)), ((var_sigrat_dn10 * var_qdrat) + (var_sigrat * var_qdrat_dn10)), ((var_sigrat_dn11 * var_qdrat) + (var_sigrat * var_qdrat_dn11)), ((var_sigrat_dn12 * var_qdrat) + (var_sigrat * var_qdrat_dn12)), (var_sigrat_dn13 * var_qdrat), (var_sigrat_dn15 * var_qdrat), (var_sigrat_dn16 * var_qdrat), ((var_sigrat_dn17 * var_qdrat) + (var_sigrat * var_qdrat_dn17)), (var_sigrat_dn18 * var_qdrat),)
    } else {
        let assign37700_e51747: f64 = (1.0 - var_qdrat);
        let assign37700_e51748: f64 = (var_sigrat * assign37700_e51747);
        (assign37700_e51748, ((var_sigrat_dn0 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn0))), ((var_sigrat_dn2 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn2))), ((var_sigrat_dn6 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn6))), ((var_sigrat_dn7 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn7))), ((var_sigrat_dn10 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn10))), ((var_sigrat_dn11 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn11))), ((var_sigrat_dn12 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn12))), (var_sigrat_dn13 * assign37700_e51747), (var_sigrat_dn15 * assign37700_e51747), (var_sigrat_dn16 * assign37700_e51747), ((var_sigrat_dn17 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn17))), (var_sigrat_dn18 * assign37700_e51747),)
    }
};
        var_sigrat_d = assign37700_e51749;
        var_sigrat_d_dn0 = assign37700_e51749_d_n0;
        var_sigrat_d_dn2 = assign37700_e51749_d_n2;
        var_sigrat_d_dn6 = assign37700_e51749_d_n6;
        var_sigrat_d_dn7 = assign37700_e51749_d_n7;
        var_sigrat_d_dn10 = assign37700_e51749_d_n10;
        var_sigrat_d_dn11 = assign37700_e51749_d_n11;
        var_sigrat_d_dn12 = assign37700_e51749_d_n12;
        var_sigrat_d_dn13 = assign37700_e51749_d_n13;
        var_sigrat_d_dn15 = assign37700_e51749_d_n15;
        var_sigrat_d_dn16 = assign37700_e51749_d_n16;
        var_sigrat_d_dn17 = assign37700_e51749_d_n17;
        var_sigrat_d_dn18 = assign37700_e51749_d_n18;

        let assign37720_e51759: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1226 = assign37720_e51759;

        let (assign37730_e51763, assign37730_e51763_d_n0, assign37730_e51763_d_n2, assign37730_e51763_d_n6, assign37730_e51763_d_n7, assign37730_e51763_d_n10, assign37730_e51763_d_n11, assign37730_e51763_d_n12, assign37730_e51763_d_n17,) = {
    if (var_guard1226 != 0.0) {
        (var_rpower, var_rpower_dn0, var_rpower_dn2, var_rpower_dn6, var_rpower_dn7, var_rpower_dn10, var_rpower_dn11, var_rpower_dn12, var_rpower_dn17,)
    } else {
        (var_itemp, var_itemp_dn0, var_itemp_dn2, var_itemp_dn6, var_itemp_dn7, var_itemp_dn10, var_itemp_dn11, var_itemp_dn12, var_itemp_dn17,)
    }
};
        var_itemp = assign37730_e51763;
        var_itemp_dn0 = assign37730_e51763_d_n0;
        var_itemp_dn2 = assign37730_e51763_d_n2;
        var_itemp_dn6 = assign37730_e51763_d_n6;
        var_itemp_dn7 = assign37730_e51763_d_n7;
        var_itemp_dn10 = assign37730_e51763_d_n10;
        var_itemp_dn11 = assign37730_e51763_d_n11;
        var_itemp_dn12 = assign37730_e51763_d_n12;
        var_itemp_dn17 = assign37730_e51763_d_n17;

        let assign37740_e51766: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1227 = assign37740_e51766;

        let assign37750_e51775: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        var_guard1228 = assign37750_e51775;

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
        *var_ci_slot = var_ci;
        *var_ci_dn0_slot = var_ci_dn0;
        *var_ci_dn10_slot = var_ci_dn10;
        *var_ci_dn11_slot = var_ci_dn11;
        *var_ci_dn12_slot = var_ci_dn12;
        *var_ci_dn17_slot = var_ci_dn17;
        *var_ci_dn2_slot = var_ci_dn2;
        *var_ci_dn6_slot = var_ci_dn6;
        *var_ci_dn7_slot = var_ci_dn7;
        *var_guard1218_slot = var_guard1218;
        *var_guard1224_slot = var_guard1224;
        *var_guard1226_slot = var_guard1226;
        *var_guard1227_slot = var_guard1227;
        *var_guard1228_slot = var_guard1228;
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
        *var_itemp_slot = var_itemp;
        *var_itemp_dn0_slot = var_itemp_dn0;
        *var_itemp_dn10_slot = var_itemp_dn10;
        *var_itemp_dn11_slot = var_itemp_dn11;
        *var_itemp_dn12_slot = var_itemp_dn12;
        *var_itemp_dn17_slot = var_itemp_dn17;
        *var_itemp_dn2_slot = var_itemp_dn2;
        *var_itemp_dn6_slot = var_itemp_dn6;
        *var_itemp_dn7_slot = var_itemp_dn7;
        *var_qdrat_slot = var_qdrat;
        *var_qdrat_dn0_slot = var_qdrat_dn0;
        *var_qdrat_dn10_slot = var_qdrat_dn10;
        *var_qdrat_dn11_slot = var_qdrat_dn11;
        *var_qdrat_dn12_slot = var_qdrat_dn12;
        *var_qdrat_dn17_slot = var_qdrat_dn17;
        *var_qdrat_dn2_slot = var_qdrat_dn2;
        *var_qdrat_dn6_slot = var_qdrat_dn6;
        *var_qdrat_dn7_slot = var_qdrat_dn7;
        *var_sid_slot = var_sid;
        *var_sid_dn0_slot = var_sid_dn0;
        *var_sid_dn10_slot = var_sid_dn10;
        *var_sid_dn11_slot = var_sid_dn11;
        *var_sid_dn12_slot = var_sid_dn12;
        *var_sid_dn17_slot = var_sid_dn17;
        *var_sid_dn2_slot = var_sid_dn2;
        *var_sid_dn6_slot = var_sid_dn6;
        *var_sid_dn7_slot = var_sid_dn7;
        *var_sigrat_slot = var_sigrat;
        *var_sigrat_d_slot = var_sigrat_d;
        *var_sigrat_d_dn0_slot = var_sigrat_d_dn0;
        *var_sigrat_d_dn10_slot = var_sigrat_d_dn10;
        *var_sigrat_d_dn11_slot = var_sigrat_d_dn11;
        *var_sigrat_d_dn12_slot = var_sigrat_d_dn12;
        *var_sigrat_d_dn13_slot = var_sigrat_d_dn13;
        *var_sigrat_d_dn15_slot = var_sigrat_d_dn15;
        *var_sigrat_d_dn16_slot = var_sigrat_d_dn16;
        *var_sigrat_d_dn17_slot = var_sigrat_d_dn17;
        *var_sigrat_d_dn18_slot = var_sigrat_d_dn18;
        *var_sigrat_d_dn2_slot = var_sigrat_d_dn2;
        *var_sigrat_d_dn6_slot = var_sigrat_d_dn6;
        *var_sigrat_d_dn7_slot = var_sigrat_d_dn7;
        *var_sigrat_dn0_slot = var_sigrat_dn0;
        *var_sigrat_dn10_slot = var_sigrat_dn10;
        *var_sigrat_dn11_slot = var_sigrat_dn11;
        *var_sigrat_dn12_slot = var_sigrat_dn12;
        *var_sigrat_dn13_slot = var_sigrat_dn13;
        *var_sigrat_dn15_slot = var_sigrat_dn15;
        *var_sigrat_dn16_slot = var_sigrat_dn16;
        *var_sigrat_dn17_slot = var_sigrat_dn17;
        *var_sigrat_dn18_slot = var_sigrat_dn18;
        *var_sigrat_dn2_slot = var_sigrat_dn2;
        *var_sigrat_dn6_slot = var_sigrat_dn6;
        *var_sigrat_dn7_slot = var_sigrat_dn7;
        *var_sigrat_s_slot = var_sigrat_s;
        *var_sigrat_s_dn0_slot = var_sigrat_s_dn0;
        *var_sigrat_s_dn10_slot = var_sigrat_s_dn10;
        *var_sigrat_s_dn11_slot = var_sigrat_s_dn11;
        *var_sigrat_s_dn12_slot = var_sigrat_s_dn12;
        *var_sigrat_s_dn13_slot = var_sigrat_s_dn13;
        *var_sigrat_s_dn15_slot = var_sigrat_s_dn15;
        *var_sigrat_s_dn16_slot = var_sigrat_s_dn16;
        *var_sigrat_s_dn17_slot = var_sigrat_s_dn17;
        *var_sigrat_s_dn18_slot = var_sigrat_s_dn18;
        *var_sigrat_s_dn2_slot = var_sigrat_s_dn2;
        *var_sigrat_s_dn6_slot = var_sigrat_s_dn6;
        *var_sigrat_s_dn7_slot = var_sigrat_s_dn7;
        *var_whi_noise_slot = var_whi_noise;
        *var_whi_noise_dn10_slot = var_whi_noise_dn10;
    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(246, 0.0);

        s.store_scalar(300, 1e-12);

        s.store_scalar(25, 0.0);

        s.store_scalar(146, 0.0);

        s.store_scalar(612, 0.0);

        s.store_scalar(556, 0.0);

        s.store_scalar(145, 0.0);

        s.store_scalar(338, 0.0);

        s.store_scalar(162, 0.0);

        s.store_scalar(163, 0.0);

        s.store_scalar(164, 0.0);

        s.store_scalar(165, 0.0);

        s.store_scalar(176, 1.0);

        s.store_scalar(190, 0.0);

        s.store_scalar(192, 0.0);

        s.store_scalar(196, 0.0);

        s.store_scalar(197, 0.0);

        s.store_scalar(198, 0.0);

        s.store_scalar(199, 0.0);

        s.store_scalar(242, 0.0);

        s.store_scalar(244, 0.0);

        s.store_scalar(250, 0.0);

        s.store_scalar(251, 0.0);

        s.store_scalar(252, 0.0);

        s.store_scalar(263, 0.0);

        s.store_scalar(264, 1.0);

        s.store_scalar(265, 0.0);

        s.store_scalar(267, 0.0);

        s.store_scalar(268, 0.0);

        s.store_scalar(272, 0.0);

        s.store_scalar(454, 0.0);

        s.store_scalar(455, 0.0);

        s.store_scalar(456, 0.0);

        s.store_scalar(457, 0.0);

        s.store_scalar(282, 0.0);

        s.store_scalar(281, 0.0);

        s.store_scalar(284, 0.0);

        s.store_scalar(283, 0.0);

        s.store_scalar(478, 0.0);

        s.store_scalar(479, 0.0);

        s.store_scalar(402, p.p237);

        s.store_scalar(463, 0.0);

        s.store_scalar(464, 0.0);

        s.store_scalar(466, 0.0);

        s.store_scalar(465, 0.0);

        s.store_scalar(467, 0.0);

        s.store_scalar(468, 0.0);

        s.store_scalar(470, 0.0);

        s.store_scalar(469, 0.0);

        s.store_scalar(522, 0.0);

        s.store_scalar(523, 0.0);

        s.store_scalar(471, 0.0);

        s.store_scalar(473, 0.0);

        s.store_scalar(293, 0.0);

        s.store_scalar(294, 0.0);

        s.store_scalar(296, 0.0);

        s.store_scalar(297, 0.0);

        s.store_scalar(298, 0.0);

        s.store_scalar(299, 0.0);

        s.store_scalar(314, 0.0);

        s.store_scalar(315, 0.0);

        s.store_scalar(316, 0.0);

        s.store_scalar(339, 0.0);

        s.store_scalar(346, 0.0);

        s.store_scalar(347, 0.0);

        s.store_scalar(348, 0.0);

        s.store_scalar(349, 0.0);

        s.store_scalar(350, 0.0);

        s.store_scalar(351, 0.0);

        s.store_scalar(352, 0.0);

        s.store_scalar(353, 0.0);

        s.store_scalar(354, 0.0);

        s.store_scalar(370, 0.0);

        s.store_scalar(355, 0.0);

        s.store_scalar(363, 0.0);

        s.store_scalar(366, 0.0);

        s.store_scalar(356, 0.0);

        s.store_scalar(357, 0.0);

        s.store_scalar(358, 0.0);

        s.store_scalar(359, 0.0);

        s.store_scalar(360, 0.0);

        s.store_scalar(383, 0.0);

        s.store_scalar(386, 0.0);

        s.store_scalar(580, 0.0);

        s.store_scalar(584, 0.0);

        s.store_scalar(585, 0.0);

        s.store_scalar(390, 0.0);

        s.store_scalar(392, 0.0);

        s.store_scalar(393, 0.0);

        s.store_scalar(401, 0.0);

        s.store_scalar(376, 0.0);

        s.store_scalar(436, 0.0);

        s.store_scalar(437, 0.0);

        s.store_scalar(438, 0.5);

        s.store_scalar(439, 0.5);

        s.store_scalar(476, 0.0);

        s.store_scalar(477, 0.0);

        s.store_scalar(488, 0.0);

        s.store_scalar(490, 0.0);

        s.store_scalar(497, 0.0);

        s.store_scalar(499, 0.0);

        s.store_scalar(56, ((p.p51 * 10.0) % 10.0));

        s.store_scalar(57, 200.0);

        s.store_scalar(58, 200.0);

        s.store_scalar(86, 0.0);

        s.store_scalar(475, 0.0);

        s.store_scalar(378, 0.0);

        s.store_scalar(369, 0.0);

        s.store_scalar(203, 0.0);

        s.store_scalar(161, 0.0);

        s.store_scalar(515, 0.0);

        s.store_scalar(73, (p.p52 * 0.01));

        s.store_scalar(59, (p.p73 / 1e-6));

        s.store_scalar(60, (p.p104 * 0.01));

        s.store_scalar(61, (p.p201 / 1e-6));

        s.store_scalar(65, (p.p240 / 1e-6));

        s.store_scalar(66, (p.p241 / 1e-6));

        s.store_scalar(67, (p.p242 * 0.01));

        s.store_scalar(68, (p.p243 / 0.01));

        s.store_scalar(69, (p.p59 / 1e-6));

        s.store_scalar(70, (p.p284 / 1e-6));

        s.store_scalar(71, (p.p148 / 1e-6));

        s.store_scalar(72, (p.p198 / 0.0001));

        s.store_scalar(74, (p.p70 * 0.01));

        s.store_scalar(75, (if (p.p83 == 0.0) { 0.0 } else { p.p84 }));

        s.store_scalar(76, (if (p.p83 == 0.0) { 0.0 } else { p.p85 }));

        s.store_scalar(77, (if (p.p80 == 0.0) { 0.0 } else { p.p81 }));

        s.store_scalar(78, (if (p.p83 == 0.0) { 0.0 } else { p.p82 }));

        s.store_scalar(79, (p.p250 * 1000000.0));

        s.store_scalar(81, (p.p232 + 273.15));

        s.store_scalar(82, p.p58);

        s.store_scalar(84, p.p46);

        s.store_scalar(85, p.p34);

        s.store_scalar(80, (if param_given[190] { p.p190 } else { (5000000000.0 / (p.p237 * p.p240)) }));

        s.b[630] = ((s.v[80] < (2.0 + 0.1)) && (0.1 >= 0.0));
        s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });

        if s.b[630] {
            s.store_scalar(44, ((2.0 + 0.1) - s.v[80]));
            s.store_square(49, 44);
            s.store_scalar(50, (0.1 * 0.1));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[631] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });

        s.b[632] = (2.0 == 1.0);
        s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });

        if ((s.b[630] && s.b[631]) && s.b[632]) {
            s.store_scalar(55, 1.0);
        }

        s.b[633] = (2.0 == 2.0);
        s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });

        if (((s.b[630] && s.b[631]) && (!s.b[632])) && s.b[633]) {
            s.store_scalar(55, 2.0);
        }

        s.b[634] = (2.0 == 4.0);
        s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });

        if ((((s.b[630] && s.b[631]) && (!s.b[632])) && (!s.b[633])) && s.b[634]) {
            s.store_scalar(55, 3.0);
        }

        s.b[635] = (2.0 == 8.0);
        s.store_scalar(635, if s.b[635] { 1.0 } else { 0.0 });

        if (((((s.b[630] && s.b[631]) && (!s.b[632])) && (!s.b[633])) && (!s.b[634])) && s.b[635]) {
            s.store_scalar(55, 4.0);
        }

        if (s.b[630] && s.b[631]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign1880_loop_guard: usize = 0;
        while {
            let assign1880_cond_e1275: f64 = if ((s.b[630] && s.b[631]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign1880_cond_e1275 != 0.0
        } {
            assign1880_loop_guard += 1;
            assert!(assign1880_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[630] && s.b[631]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (s.b[630] && (!s.b[631])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if s.b[630] {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 0.1);
            s.store_sub_from_scalar(80, (2.0 + 0.1), 43);
        }

        if (!s.b[630]) {
        }

        s.store_scalar(87, (p.p55 - (s.v[81] * (9.025e-5 + (s.v[81] * 1e-7)))));

        s.store_scalar(88, p.p236);

        s.store_scalar(89, (1.034943e-10 / p.p237));

        s.store_scalar(90, (1.0 / s.v[89]));

        s.store_scalar(91, (3.453133e-11 / s.v[88]));

        s.store_scalar(92, (s.v[88] / 3.453133e-11));

        s.store_scalar(93, (3.453133e-11 / p.p239));

        s.store_scalar(94, (p.p239 / 3.453133e-11));

        s.store_scalar(95, (s.v[94] + s.v[90]));

        s.store_scalar(96, p.p0);

        s.store_scalar(97, (s.v[96] - (2.0 * p.p56)));

        s.store_scalar(98, (s.v[96] - (2.0 * p.p57)));

        s.store_scalar(99, (if (p.p40 == 0.0) { s.v[96] } else { s.v[97] }));

        s.store_scalar(100, (s.v[99] * 1000000.0));

        s.store_scalar(101, (p.p1 / p.p9));

        s.store_scalar(102, p.p60);

        s.store_scalar(103, (if (s.v[56] < 1.0) { 0.0 } else { p.p295 }));

        s.store_scalar(104, (if (s.v[56] < 1.0) { p.p60 } else { p.p61 }));

        s.b[636] = (p.p43 == 0.0);
        s.store_scalar(636, if s.b[636] { 1.0 } else { 0.0 });

        if s.b[636] {
            s.store_scalar(105, (s.v[101] - (2.0 * s.v[102])));
            s.store_scalar(106, (s.v[101] - (2.0 * s.v[104])));
        }

        if (!s.b[636]) {
            s.store_scalar(105, ((s.v[101] - (p.p18 * s.v[103])) - ((2.0 - p.p18) * s.v[102])));
            s.store_scalar(106, ((s.v[101] - (p.p18 * s.v[103])) - ((2.0 - p.p18) * s.v[104])));
        }

        s.store_scale(107, 105, p.p9);

        s.store_scale(108, 106, p.p9);

        s.store_scalar(109, (s.v[101] * 1000000.0));

        s.store_scalar(110, (s.v[109] * s.v[100]));

        s.store_scalar(111, ((p.p107 * (1.0 + (p.p108 / ((s.v[100]) as f64).powf(p.p111)))) * (1.0 + (p.p109 / ((s.v[109]) as f64).powf(p.p110)))));

        s.b[637] = (((s.v[56] > 3.0) && (s.v[59] < s.v[65])) && (p.p72 > 0.0));
        s.store_scalar(637, if s.b[637] { 1.0 } else { 0.0 });

        if s.b[637] {
            s.store_scalar(59, s.v[65]);
        }

        s.store_scale(112, 59, (1.0 + (p.p74 / ((s.v[109]) as f64).powf(p.p75))));

        s.store_scalar(113, (2.0 / ((1.0 / (p.p62 + (0.5 * s.v[96]))) + (1.0 / (p.p63 + (0.5 * s.v[96]))))));

        s.store_scalar(114, (1.6021918e-19 / (1.3806226e-23 * s.v[81])));

        s.store_scalar(115, ((1.6021918e-19 * s.v[66]) * 1.034943e-10));

        s.store_scalar(116, (p.p244 * ((s.v[100]) as f64).powf((-p.p247))));

        s.store_scalar(117, (p.p251 * ((s.v[100]) as f64).powf((-p.p252))));

        s.store_scalar(118, (p.p248 * (((s.v[100] + s.v[79])) as f64).powf((-p.p249))));

        s.store_scalar(119, (((((2.0 * 1.6021918e-19) * s.v[71]) * 1.034943e-10)) as f64).sqrt());

        s.store_scalar(120, (1.0 / (s.v[71] * s.v[71])));

        s.store_scalar(121, ((((1.0 + (1.0 / s.v[100]))) as f64).powf(p.p91) * p.p89));

        s.store_scalar(122, s.v[115]);

        s.store_scalar(123, p.p68);

        s.store_scalar(124, (s.v[99] + (p.p76 / ((s.v[110]) as f64).powf(p.p77))));

        s.store_scalar(125, (p.p78 / ((s.v[110]) as f64).powf(p.p79)));

        s.store_scalar(126, ((p.p149 * (1.0 + (p.p150 / (((s.v[124] * 1000000.0)) as f64).powf(p.p151)))) + (p.p152 / ((s.v[109]) as f64).powf(p.p153))));

        s.store_scalar(127, (1.0 + (((s.v[100]) as f64).powf(p.p192) * p.p193)));

        s.b[638] = (p.p44 <= 0.0);
        s.store_scalar(638, if s.b[638] { 1.0 } else { 0.0 });

        if s.b[638] {
            s.store_scalar(129, (1.0 + (p.p130 / ((s.v[109]) as f64).powf(p.p131))));
        }

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        let nv10 = ctx.node_voltage(nodes[10]);
        if s.b[638] {
            s.store_scalar(130, (p.p124 * (1.0 + (p.p125 / ((s.v[100]) as f64).powf(p.p126)))));
            s.store_scalar(131, (s.v[100] / (s.v[100] + p.p123)));
            s.store_scalar(132, (p.p117 * (1.0 + (p.p119 / ((s.v[100]) as f64).powf(p.p120)))));
            s.store_scalar(133, (p.p118 * (1.0 + (p.p121 / s.v[100]))));
        }

        if (!s.b[638]) {
            s.store_scalar(329, ((s.v[109]) as f64).powf(p.p131));
            s.store_div_scaled_value_offset_denominator(134, s.ad_value(329), (p.p127 * (1.0 + (p.p128 / ((s.v[100]) as f64).powf(p.p129)))), s.ad_value(329), p.p130, 1.0);
            s.store_scalar(130, (p.p124 * (1.0 + (p.p125 / ((s.v[100]) as f64).powf(p.p126)))));
            s.store_scalar(131, (p.p123 * (1.0 + (p.p132 / ((s.v[100]) as f64).powf(p.p133)))));
            s.store_scalar(132, (p.p117 * (1.0 + (p.p119 / ((s.v[100]) as f64).powf(p.p120)))));
            s.store_scalar(133, (p.p118 * (1.0 + (p.p121 / s.v[100]))));
        }

        s.store_scale(135, 108, (1000000.0 * (p.p65 * 1.0 / (((s.v[100]) as f64).powf(p.p66)))));

        s.store_scalar(136, (p.p134 * (1.0 + (p.p135 / ((s.v[100]) as f64).powf(p.p136)))));

        s.b[639] = (p.p44 <= 0.0);
        s.store_scalar(639, if s.b[639] { 1.0 } else { 0.0 });

        if s.b[639] {
            s.store_scalar(137, (p.p127 * (1.0 + (p.p128 / ((s.v[100]) as f64).powf(p.p129)))));
        }

        s.store_scalar(138, (((((p.p115 * s.v[100]) * p.p114) / ((p.p115 * s.v[100]) + p.p114)) + p.p116) + 1e-50));

        s.b[640] = (s.v[138] < 3.0);
        s.store_scalar(640, if s.b[640] { 1.0 } else { 0.0 });

        if s.b[640] {
            s.store_scalar(138, 3.0);
        }

        s.store_scalar(139, (p.p50 * p.p253));

        s.b[564] = param_given[168];
        s.store_scalar(564, if s.b[564] { 1.0 } else { 0.0 });

        s.b[565] = param_given[169];
        s.store_scalar(565, if s.b[565] { 1.0 } else { 0.0 });

        s.b[566] = param_given[170];
        s.store_scalar(566, if s.b[566] { 1.0 } else { 0.0 });

        s.b[525] = param_given[294];
        s.store_scalar(525, if s.b[525] { 1.0 } else { 0.0 });

        s.b[524] = param_given[293];
        s.store_scalar(524, if s.b[524] { 1.0 } else { 0.0 });

        s.b[529] = param_given[13];
        s.store_scalar(529, if s.b[529] { 1.0 } else { 0.0 });

        s.b[530] = param_given[14];
        s.store_scalar(530, if s.b[530] { 1.0 } else { 0.0 });

        s.b[527] = param_given[23];
        s.store_scalar(527, if s.b[527] { 1.0 } else { 0.0 });

        s.b[526] = param_given[22];
        s.store_scalar(526, if s.b[526] { 1.0 } else { 0.0 });

        s.b[539] = param_given[16];
        s.store_scalar(539, if s.b[539] { 1.0 } else { 0.0 });

        s.b[540] = (p.p17 != 0.0);
        s.store_scalar(540, if s.b[540] { 1.0 } else { 0.0 });

        s.store_scalar(451, 1.0);

        s.store_scalar(142, 0.0);

        s.store_scalar(518, p.p13);

        s.store_scalar(519, p.p14);

        s.store_scalar(520, (p.p16 + 273.15));

        s.store_scale(542, 108, (s.v[451] * s.v[68]));

        s.b[641] = (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p9 == 1.0) || ((p.p9 > 1.0) && (p.p12 > 0.0))));
        s.store_scalar(641, if s.b[641] { 1.0 } else { 0.0 });

        if s.b[641] {
            s.store_scalar(328, 0.0);
            s.store_scalar(562, 0.0);
        }

        let mut assign2820_loop_guard: usize = 0;
        while {
            let assign2820_cond_e1891: f64 = if (s.b[641] && (s.v[562] < p.p9)) { 1.0 } else { 0.0 };
            assign2820_cond_e1891 != 0.0
        } {
            assign2820_loop_guard += 1;
            assert!(assign2820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[641] {
                s.store_add_scaled_inputs3_mixed_iaa(328, 328, 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(562), (p.p12 + s.v[96]), (p.p10 + (0.5 * s.v[96])))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(562), (p.p12 + s.v[96]), (p.p11 + (0.5 * s.v[96])))), 1.0);
                s.store_offset(562, 562, 1.0);
            }
        }

        if s.b[641] {
            s.store_div_from_scalar(537, (2.0 * p.p9), 328);
        }

        if (!s.b[641]) {
            s.store_scalar(537, 0.0);
        }

        s.b[642] = (s.v[537] > 0.0);
        s.store_scalar(642, if s.b[642] { 1.0 } else { 0.0 });

        if s.b[642] {
            s.store_scalar(328, (1.0 / (1.0 + p.p162)));
            s.store_powf_ad(329, A::div_from_scalar(p.p161, s.ad_value(537)), p.p163);
            s.store_scalar(330, (((p.p161 / s.v[113])) as f64).powf(p.p163));
            s.store_div_scaled_product_offset_denominator(538, s.ad_value(112), A::offset(A::mul(s.ad_value(328), s.ad_value(329)), 1.0), 1.0, A::mul(s.ad_value(328), s.ad_value(330)), 1.0, 1.0);
        }

        if (!s.b[642]) {
            s.copy_ad(538, 112);
        }

        s.store_scalar(329, ((1.0 + (p.p199 / ((s.v[109]) as f64).powf(p.p200))) * (1.0 + (p.p202 / ((s.v[100]) as f64).powf(p.p203)))));

        s.store_scalar(330, (s.v[61] / s.v[65]));

        s.store_scalar(44, ((s.v[330] - s.v[329]) - 0.01));

        s.store_scalar(45, ((4.0 * s.v[330]) * 0.01));

        if (!(s.v[45] > 0.0)) {
            s.store_scalar(45, (-s.v[45]));
        }

        s.store_sqrt_offset_input(45, 45, (s.v[44] * s.v[44]));

        s.store_sub_from_scalar_ad(328, s.v[330], A::scaled_offset(s.ad_value(45), s.v[44], 0.5));

        s.store_scale(544, 328, s.v[65]);

        s.b[643] = (s.v[537] > 0.0);
        s.store_scalar(643, if s.b[643] { 1.0 } else { 0.0 });

        if s.b[643] {
            s.store_scalar(328, (1.0 / (1.0 + p.p165)));
            s.store_powf_ad(329, A::div_from_scalar(p.p164, s.ad_value(537)), p.p166);
            s.store_scalar(330, (((p.p164 / s.v[113])) as f64).powf(p.p166));
            s.store_div_scaled_product_offset_denominator(544, s.ad_value(544), A::offset(A::mul(s.ad_value(328), s.ad_value(329)), 1.0), 1.0, A::mul(s.ad_value(328), s.ad_value(330)), 1.0, 1.0);
        }

        s.b[644] = ((s.v[99] > p.p72) || (p.p72 <= 0.0));
        s.store_scalar(644, if s.b[644] { 1.0 } else { 0.0 });

        if s.b[644] {
            s.store_add_scaled_inputs(536, 544, ((s.v[99] - p.p72) * 1.0 / (s.v[99])), 538, (p.p72 * 1.0 / (s.v[99])));
        }

        if (!s.b[644]) {
            s.store_add_scaled_inputs3_indices(536, 538, 1.0, 538, ((p.p72 - s.v[99]) * 1.0 / (p.p72)), 544, (-((p.p72 - s.v[99]) * 1.0 / (p.p72))));
        }

        s.store_scale(229, 536, 1.6021918e-19);

        s.store_scale(545, 229, 1.034943e-10);

        s.store_scale(546, 545, 2.0);

        s.b[645] = ((s.v[99] <= (2.0 * p.p72)) && (p.p72 > 0.0));
        s.store_scalar(645, if s.b[645] { 1.0 } else { 0.0 });

        if s.b[645] {
            s.store_add_scaled_inputs4_indices(593, 538, 2.0, 538, (-(s.v[99] * 1.0 / (p.p72))), 544, (-(-(s.v[99] * 1.0 / (p.p72)))), 544, -1.0);
            s.store_ln_div(548, 593, 544);
        }

        if (!s.b[645]) {
            s.store_scalar(548, 0.0);
        }

        s.store_scaled_ln_scaled_input(232, 536, 1.0 / ((10400000000.0 / 1e-6)), (2.0 / 38.68283));

        s.store_scaled_ln_scaled_input(236, 544, 1.0 / ((10400000000.0 / 1e-6)), (2.0 / 38.68283));

        s.store_sqrt_div_from_scalar_ad(549, ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(536));

        s.store_scalar(328, ((1.0 + (p.p194 / ((s.v[100]) as f64).powf(p.p195))) * (1.0 + (p.p196 / ((s.v[110]) as f64).powf(p.p197)))));

        s.store_scalar(44, ((((s.v[328] * s.v[328]) + ((4.0 * 0.001) * 0.001))) as f64).sqrt());

        s.store_scalar(550, ((0.5 * (s.v[328] + s.v[44])) + (1e-10 * 0.001)));

        s.b[646] = (s.v[550] < 0.0);
        s.store_scalar(646, if s.b[646] { 1.0 } else { 0.0 });

        if s.b[646] {
            s.store_scalar(550, 0.0);
        }

        s.b[649] = (p.p261 == 1.0);
        s.store_scalar(649, if s.b[649] { 1.0 } else { 0.0 });

        if s.b[649] {
            s.store_offset_scaled(327, 107, p.p289, p.p288);
        }

        s.b[654] = (p.p43 == 1.0);
        s.store_scalar(654, if s.b[654] { 1.0 } else { 0.0 });

        if (s.b[654] && (p.p24 != 0.0)) {
            s.store_scalar(533, (if s.b[527] { p.p23 } else { ((p.p20 * p.p9) * p.p19) }));
        }

        if (s.b[654] && (p.p24 != 0.0)) {
            s.store_scalar(534, (if s.b[526] { p.p22 } else { ((p.p21 * p.p9) * p.p19) }));
        }

        if (s.b[654] && (p.p24 != 0.0)) {
            s.store_scalar(531, 0.0);
            s.store_scalar(532, 0.0);
        }

        s.b[655] = ((s.v[533] > 0.0) && s.b[525]);
        s.store_scalar(655, if s.b[655] { 1.0 } else { 0.0 });

        if ((s.b[654] && (p.p24 != 0.0)) && s.b[655]) {
            s.store_scale(531, 533, (-p.p294));
        }

        if ((s.b[654] && (p.p24 != 0.0)) && (!s.b[655])) {
            s.store_scalar(531, 0.0);
        }

        s.b[656] = ((s.v[534] > 0.0) && s.b[524]);
        s.store_scalar(656, if s.b[656] { 1.0 } else { 0.0 });

        if ((s.b[654] && (p.p24 != 0.0)) && s.b[656]) {
            s.store_scale(532, 534, (-p.p293));
            s.store_scalar(534, 0.0);
        }

        if (s.b[654] && (p.p24 == 0.0)) {
            s.store_scalar(534, 0.0);
            s.store_scalar(532, 0.0);
            s.store_scalar(533, 0.0);
            s.store_scalar(531, 0.0);
        }

        if s.b[654] {
            s.store_scalar(535, (if (p.p19 > s.v[96]) { (0.5 * (p.p19 - s.v[96])) } else { 0.0 }));
        }

        s.b[657] = (!s.b[529]);
        s.store_scalar(657, if s.b[657] { 1.0 } else { 0.0 });

        if (s.b[654] && s.b[657]) {
            s.copy_ad(518, 535);
        }

        s.b[658] = (!s.b[530]);
        s.store_scalar(658, if s.b[658] { 1.0 } else { 0.0 });

        if (s.b[654] && s.b[658]) {
            s.copy_ad(519, 535);
        }

        if s.b[654] {
            s.store_add_scaled_inputs(286, 107, 1.0, 518, p.p9);
            s.store_add_scaled_inputs(285, 107, 1.0, 519, p.p9);
            s.store_add_scaled_inputs(288, 108, 1.0, 518, p.p9);
            s.store_add_scaled_inputs(287, 108, 1.0, 519, p.p9);
        }

        if (!s.b[654]) {
            s.store_scalar(534, 0.0);
            s.store_scalar(532, 0.0);
            s.store_scalar(533, 0.0);
            s.store_scalar(531, 0.0);
            s.store_scalar(286, 0.0);
            s.store_scalar(285, 0.0);
            s.store_scalar(288, 0.0);
            s.store_scalar(287, 0.0);
        }

        s.store_scaled_voltage(571, ctx, nodes, Some(6), Some(7), p.p50);

        s.store_scaled_voltage(572, ctx, nodes, Some(11), Some(7), p.p50);

        s.store_scaled_voltage(570, ctx, nodes, Some(12), Some(7), p.p50);

        s.b[659] = (p.p43 == 1.0);
        s.store_scalar(659, if s.b[659] { 1.0 } else { 0.0 });

        if s.b[659] {
            s.store_scaled_voltage(590, ctx, nodes, Some(12), Some(6), p.p50);
            s.store_scaled_voltage(591, ctx, nodes, Some(12), Some(7), p.p50);
        }

        if (s.b[659] && (s.v[85] != 0.0)) {
            s.store_scaled_voltage(580, ctx, nodes, Some(18), None, (1e-9 / 0.0001));
            s.store_scaled_voltage(581, ctx, nodes, Some(13), None, (1e-9 / 0.0001));
        }

        if (s.b[659] && (s.v[85] == 0.0)) {
            s.store_scalar(580, 0.0);
            s.store_scalar(581, 0.0);
        }

        if (!s.b[659]) {
            s.store_scalar(590, 0.0);
            s.store_scalar(591, 0.0);
        }

        if ((!s.b[659]) && (s.v[85] != 0.0)) {
            s.store_scaled_voltage(584, ctx, nodes, Some(15), None, (1e-9 / 0.0001));
            s.store_scaled_voltage(585, ctx, nodes, Some(16), None, (1e-9 / 0.0001));
            s.store_scaled_voltage(581, ctx, nodes, Some(13), None, (1e-9 / 0.0001));
        }

        if ((!s.b[659]) && (s.v[85] == 0.0)) {
            s.store_scalar(584, 0.0);
            s.store_scalar(585, 0.0);
            s.store_scalar(581, 0.0);
        }

        if ((p.p38 > 0.0) && (s.v[67] > 0.0)) {
            if (nv10 > 0.0) {
                s.store_voltage(20, ctx, nodes, Some(10), None);
            } else {
                s.store_scalar(20, 0.0);
            }
        } else {
            s.store_scalar(20, 0.0);
        }

        s.b[660] = (s.v[571] >= 0.0);
        s.store_scalar(660, if s.b[660] { 1.0 } else { 0.0 });

        if s.b[660] {
            s.store_scalar(613, 1.0);
            s.store_scalar(461, 1.0);
            s.store_scalar(462, 0.0);
            s.copy_ad(157, 571);
            s.copy_ad(158, 572);
            s.copy_ad(156, 570);
        }

        if (!s.b[660]) {
            s.store_scalar(613, (-1.0));
            s.store_scalar(461, 0.0);
            s.store_scalar(462, 1.0);
            s.store_neg(157, 571);
            s.store_sub(158, 572, 571);
            s.store_sub(156, 570, 571);
        }

        s.store_scalar(429, ctx_temp);

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[539] {
            s.store_scalar(429, s.v[520]);
        }

        if s.b[540] {
            s.store_offset(429, 429, p.p17);
        }

        s.store_add(429, 429, 20);

        s.store_offset(328, 429, (-s.v[81]));

        s.store_mul_offset_rhs(329, 328, 429, s.v[81]);

        s.store_sub_scaled_ad_lhs(237, A::sub_from_scalar(s.v[87], A::scale(s.ad_value(328), p.p53)), 329, p.p54);

        s.store_div_from_scalar_scaled_input(225, 1.6021918e-19, 429, 1.3806226e-23);

        s.store_square(226, 225);

        s.store_div_from_scalar(227, 1.0, 225);

        s.store_scalar(663, (((p.p254 * (1.0 + (p.p98 / ((s.v[109]) as f64).powf(p.p99)))) * (1.0 + (p.p100 / ((s.v[100]) as f64).powf(p.p101)))) * (1.0 + (p.p102 / ((s.v[110]) as f64).powf(p.p103)))));

        s.store_scalar(666, (1.0 / (1.0 + p.p159)));

        s.store_scalar(667, 0.0);

        s.store_scalar(664, (s.v[663] * (1.0 + (s.v[666] * s.v[667]))));

        s.store_powf_scaled_input(665, 429, 1.0 / (s.v[81]), p.p112);

        s.store_scale(543, 665, 1.0 / (s.v[664]));

        s.store_mul(433, 548, 227);

        s.store_scale(328, 429, 1.0 / (s.v[81]));

        s.store_div_scaled_inputs_mixed_ia(253, 550, s.v[73], A::sub(A::add_scaled_product(A::scale_offset(s.ad_value(328), 0.4, 1.8), 1.0, s.ad_value(328), s.ad_value(328), 0.1), A::scale_offset(s.ad_value(328), (-s.v[60]), s.v[60])), 1.0);

        s.store_sqrt(302, 237);

        s.store_mul(303, 237, 302);

        s.store_scaled_mul_ad(230, A::powf(A::scale(s.ad_value(429), 1.0 / (s.v[81])), 1.5), A::exp(A::offset(A::mul_scaled_lhs(s.ad_value(237), (-1.0 / (2.0)), s.ad_value(225)), ((s.v[87] / 2.0) * s.v[114]))), (10400000000.0 / 1e-6));

        s.store_scaled_sqrt(208, 227, s.v[119]);

        s.store_square(205, 208);

        s.store_scaled_square(209, 230, s.v[120]);

        s.store_scalar(441, (s.v[96] - (2.0 * p.p56)));

        s.b[668] = (s.v[56] > 3.0);
        s.store_scalar(668, if s.b[668] { 1.0 } else { 0.0 });

        if s.b[668] {
            s.store_mul_scaled_ln_ad_rhs(231, 227, 2.0, A::div(s.ad_value(536), s.ad_value(230)));
        }

        if (!s.b[668]) {
            s.store_mul_scaled_ln_ad_rhs(231, 227, 2.0, A::div(s.ad_value(544), s.ad_value(230)));
        }

        s.store_sqrt_mul_ad(228, A::div_from_scalar(1.034943e-10, s.ad_value(229)), s.ad_value(227));

        s.store_scaled_mul(238, 229, 228, 1.414213562373095);

        s.b[669] = (p.p43 == 1.0);
        s.store_scalar(669, if s.b[669] { 1.0 } else { 0.0 });

        if s.b[669] {
            s.store_scalar(474, 0.0);
            s.store_scalar(239, 0.0);
            s.store_div(328, 230, 536);
        }

        if (!s.b[669]) {
            s.store_sqrt_scaled_input(474, 227, (2.0 * s.v[122]));
            s.store_scale(328, 230, 1.0 / (s.v[66]));
            s.store_square(239, 328);
            s.store_div(328, 230, 544);
        }

        s.store_square(379, 328);

        s.store_sqrt_scaled_input_ad(444, A::div_scalar_by_product(1.034943e-10, s.ad_value(229), s.ad_value(225), 1.0), 2.0);

        s.store_div_from_scalar(547, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544);

        s.store_sqrt_div_scaled_inputs(416, 231, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);

        s.b[674] = (p.p43 == 1.0);
        s.store_scalar(674, if s.b[674] { 1.0 } else { 0.0 });

        if s.b[674] {
            s.store_scalar(141, 0.4);
            s.store_scalar(140, 0.8);
        }

        if (!s.b[674]) {
            s.store_scalar(141, 0.8);
            s.store_scalar(140, 1.2);
        }

        s.b[675] = (s.v[141] > (s.v[140] * 0.5));
        s.store_scalar(675, if s.b[675] { 1.0 } else { 0.0 });

        if s.b[675] {
            s.store_scale(141, 140, 0.5);
        }

        s.b[676] = (s.v[156] > s.v[141]);
        s.store_scalar(676, if s.b[676] { 1.0 } else { 0.0 });

        if s.b[676] {
            s.store_sub(329, 156, 141);
            s.store_sub(330, 140, 141);
            s.store_square(49, 329);
            s.store_square(50, 330);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[677] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.store_scalar(677, if s.b[677] { 1.0 } else { 0.0 });

        s.b[678] = (4.0 == 1.0);
        s.store_scalar(678, if s.b[678] { 1.0 } else { 0.0 });

        if ((s.b[676] && s.b[677]) && s.b[678]) {
            s.store_scalar(55, 1.0);
        }

        s.b[679] = (4.0 == 2.0);
        s.store_scalar(679, if s.b[679] { 1.0 } else { 0.0 });

        if (((s.b[676] && s.b[677]) && (!s.b[678])) && s.b[679]) {
            s.store_scalar(55, 2.0);
        }

        s.b[680] = (4.0 == 4.0);
        s.store_scalar(680, if s.b[680] { 1.0 } else { 0.0 });

        if ((((s.b[676] && s.b[677]) && (!s.b[678])) && (!s.b[679])) && s.b[680]) {
            s.store_scalar(55, 3.0);
        }

        s.b[681] = (4.0 == 8.0);
        s.store_scalar(681, if s.b[681] { 1.0 } else { 0.0 });

        if (((((s.b[676] && s.b[677]) && (!s.b[678])) && (!s.b[679])) && (!s.b[680])) && s.b[681]) {
            s.store_scalar(55, 4.0);
        }

        if (s.b[676] && s.b[677]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign5050_loop_guard: usize = 0;
        while {
            let assign5050_cond_e3346: f64 = if ((s.b[676] && s.b[677]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign5050_cond_e3346 != 0.0
        } {
            assign5050_loop_guard += 1;
            assert!(assign5050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[676] && s.b[677]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (s.b[676] && (!s.b[677])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if s.b[676] {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(331, 329, 330, 53);
            s.store_div_scaled_product3_indices(335, 330, 52, 53, 1.0, 48, 1.0);
            s.store_add(154, 141, 331);
            s.copy_ad(155, 335);
        }

        if (!s.b[676]) {
            s.copy_ad(154, 156);
            s.store_scalar(155, 1.0);
        }

        if (s.v[157] > 20.0) {
            s.store_scalar(152, 20.0);
        } else {
            s.copy_ad(152, 157);
        }

        if (s.v[158] > 20.0) {
            s.store_scalar(153, 20.0);
        } else {
            s.copy_ad(153, 158);
        }

        if (s.v[158] < (-20.0)) {
            s.store_scalar(153, (-20.0));
        }

        if (s.v[154] < (-20.0)) {
            s.store_scalar(154, (-20.0));
        }

        s.copy_ad(157, 152);

        s.copy_ad(158, 153);

        s.copy_ad(156, 154);

        s.store_scalar(144, 0.0);

        s.store_scalar(619, 0.0);

        s.store_scalar(620, 0.0);

        s.store_scalar(621, 0.0);

        s.store_scalar(622, 0.0);

        s.store_scalar(623, 0.0);

        s.store_scalar(624, 0.0);

        s.store_scalar(425, 0.0);

        s.store_scalar(426, 0.0);

        s.store_scalar(427, 0.0);

        s.store_scalar(428, 0.0);

        s.store_scalar(167, 0.0);

        s.store_scalar(168, 0.0);

        s.store_scaled_mul(682, 155, 157, 0.5);

        s.store_scale(44, 682, (2.0 * 1.0 / (p.p226)));

        s.store_offset_mul_offset_rhs_ad_rhs(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_div_from_scalar(175, p.p226, 45);

        s.b[683] = (s.v[175] < 5e-12);
        s.store_scalar(683, if s.b[683] { 1.0 } else { 0.0 });

        if s.b[683] {
            s.store_scalar(175, 5e-12);
        }

        s.store_add(172, 156, 175);

        s.store_add_scaled_inputs(173, 157, 1.0, 175, 2.0);

        s.store_add(174, 158, 175);

        s.b[684] = (p.p43 == 1.0);
        s.store_scalar(684, if s.b[684] { 1.0 } else { 0.0 });

        if s.b[684] {
            s.copy_ad(513, 156);
            s.copy_ad(514, 172);
        }

        if (!s.b[684]) {
            if (s.v[56] < 3.0) {
                s.copy_ad(513, 156);
            } else {
                s.store_scalar(513, 0.0);
            }
        }

        if (!s.b[684]) {
            if (s.v[56] < 3.0) {
                s.copy_ad(514, 172);
            } else {
                s.store_scalar(514, 0.0);
            }
        }

        s.store_scale(685, 229, (2.0 * (1.034943e-10 * (s.v[92] * s.v[92]))));

        s.store_offset(686, 158, (-s.v[123]));

        s.store_offset_mul_ad(687, A::div_from_scalar(2.0, s.ad_value(685)), A::add_scaled_inputs3(s.ad_value(686), 1.0, s.ad_value(227), (-1.0), s.ad_value(513), -1.0), 1.0);

        s.store_sqrt_square_offset(44, 687, ((4.0 * 0.001) * 0.001));

        s.store_offset_add_scaled_inputs_indices(331, 687, 0.5, 44, 0.5, (1e-10 * 0.001));

        s.b[689] = (s.v[331] < 0.0);
        s.store_scalar(689, if s.b[689] { 1.0 } else { 0.0 });

        if s.b[689] {
            s.store_scalar(331, 0.0);
        }

        s.store_sqrt_offset_input(688, 331, 1e-50);

        s.store_add_mul_sub_from_scalar_rhs_indices(193, 686, 685, 1.0, 688);

        s.store_sub(194, 193, 231);

        s.store_offset(44, 194, (((-0.1)) + ((-0.05))));

        s.store_scalar(45, ((4.0 * 0.1) * 0.05));

        if (!(s.v[45] > 0.0)) {
            s.store_scalar(45, (-s.v[45]));
        }

        s.store_sqrt_square_add(45, 44, 45);

        s.store_offset_add_scaled_inputs_indices(194, 44, 0.5, 45, 0.5, 0.1);

        s.store_div(685, 157, 194);

        s.copy_ad(44, 685);

        s.store_square(45, 44);

        s.store_mul(46, 45, 44);

        s.store_square(47, 45);

        s.store_div_from_scalar_ad(688, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));

        s.store_mul_ad_affine_product_lhs(327, A::add_scaled_inputs3_offset(s.ad_value(44), 2.0, s.ad_value(45), 3.0, s.ad_value(46), 4.0, 1.0), s.ad_value(688), -1.0, 0.0, 688);

        s.store_sub_from_scalar(688, 1.0, 688);

        s.store_neg(327, 327);

        s.store_square(326, 688);

        s.b[696] = (((p.p204 == 0.0) && (p.p206 == 0.0)) || (p.p205 == 0.0));
        s.store_scalar(696, if s.b[696] { 1.0 } else { 0.0 });

        if s.b[696] {
            s.store_scalar(148, 0.0);
        }

        if (!s.b[696]) {
            s.store_scalar(148, 1.0);
        }

        s.store_sqrt_mul_scaled_lhs(690, 229, (2.0 * 1.034943e-10), 232);

        s.store_add_scaled_ad_lhs(325, A::offset(s.ad_value(232), s.v[123]), 690, 1.0 / (s.v[91]));

        s.b[697] = (s.v[148] == 0.0);
        s.store_scalar(697, if s.b[697] { 1.0 } else { 0.0 });

        if s.b[697] {
            s.store_scalar(321, s.v[88]);
            s.store_scalar(323, s.v[91]);
            s.store_scalar(324, s.v[92]);
            s.store_scaled_mul(434, 238, 238, (s.v[92] * s.v[92]));
        }

        if (!s.b[697]) {
            s.store_add_scaled_inputs3_offset_indices(694, 158, 1.0, 513, (-1.0), 325, -1.0, p.p205);
            s.store_sqrt_square_offset(44, 694, ((4.0 * 0.0001) * 0.0001));
            s.store_offset_add_scaled_inputs_indices(690, 694, 0.5, 44, 0.5, (1e-10 * 0.0001));
        }

        s.b[698] = (s.v[690] < 0.0);
        s.store_scalar(698, if s.b[698] { 1.0 } else { 0.0 });

        if ((!s.b[697]) && s.b[698]) {
            s.store_scalar(690, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[697]) {
            s.store_div_from_scalar(691, 1.0, 690);
            s.store_scaled_abs(693, 325, 2.0);
            s.store_offset_sub_from_scalar_ad(695, s.v[123], s.ad_value(325), p.p205);
        }

        if (!s.b[697]) {
            if (s.v[695] > s.v[693]) {
                s.copy_ad(692, 695);
            } else {
                s.copy_ad(692, 693);
            }
        }

        if (!s.b[697]) {
            s.store_offset_sub_ad(44, A::div_from_scalar(1.0, s.ad_value(692)), s.ad_value(691), (-0.0001));
            s.store_scale_ad(45, A::div_from_scalar(1.0, s.ad_value(692)), (4.0 * 0.0001));
        }

        if (!s.b[697]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (!s.b[697]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_mixed_aii(690, A::div_from_scalar(1.0, s.ad_value(692)), 1.0, 44, (-0.5), 45, (-0.5));
            s.store_offset_scaled(322, 690, p.p204, p.p206);
        }

        s.b[699] = ((s.v[322] * 1000000000000.0) < s.v[88]);
        s.store_scalar(699, if s.b[699] { 1.0 } else { 0.0 });

        if ((!s.b[697]) && s.b[699]) {
            s.store_scalar(322, 0.0);
            s.store_scalar(148, 0.0);
        }

        if (!s.b[697]) {
            s.store_offset(321, 322, s.v[88]);
            s.store_div_from_scalar(323, 3.453133e-11, 321);
            s.store_scale(324, 321, 28959208927.08158);
            s.store_mul_ad_product_lhs_mixed_ai(434, A::square(s.ad_value(238)), 324, 324);
        }

        s.b[700] = ((p.p43 == 1.0) || (s.v[56] < 3.0));
        s.store_scalar(700, if s.b[700] { 1.0 } else { 0.0 });

        if s.b[700] {
            s.store_offset_sub_from_scalar_ad(44, 0.5, s.ad_value(514), (-0.001));
            s.store_scalar(45, ((4.0 * 0.5) * 0.001));
        }

        if s.b[700] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[700] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_add_scaled_inputs_indices(435, 44, (-0.5), 45, (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(440, 229, (((-p.p237) * p.p237) * 1.0 / ((2.0 * 1.034943e-10))), 231, 1.0, 227, -1.0);
            s.store_offset_sub(44, 435, 440, (-0.001));
            s.store_scale(45, 440, (4.0 * 0.001));
        }

        if s.b[700] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if s.b[700] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(435, 440, 1.0, 44, 0.5, 45, 0.5);
        }

        s.b[701] = (s.v[56] > 2.0);
        s.store_scalar(701, if s.b[701] { 1.0 } else { 0.0 });

        if (s.b[700] && s.b[701]) {
            s.store_offset_sub(44, 232, 435, (-0.001));
            s.store_scale(45, 232, (4.0 * 0.001));
        }

        if (s.b[700] && s.b[701]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[700] && s.b[701]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(435, 232, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if (!s.b[700]) {
            s.store_scalar(435, 0.0);
        }

        s.b[702] = (s.v[56] < 3.0);
        s.store_scalar(702, if s.b[702] { 1.0 } else { 0.0 });

        if s.b[702] {
            s.store_scalar(184, p.p237);
        }

        if (!s.b[702]) {
            s.store_div_from_scalar(328, (2.0 * 1.034943e-10), 229);
            s.store_sqrt_mul_sub_rhs(184, 328, 232, 435);
        }

        if (s.v[56] < 3.0) {
            s.store_sqrt_mul(245, 546, 232);
        } else {
            s.store_sqrt_mul_sub_rhs(245, 546, 232, 435);
        }

        s.store_add_ad_lhs(318, A::add_scaled_product(A::offset(s.ad_value(232), s.v[123]), 1.0, s.ad_value(245), s.ad_value(324), 1.0), 433);

        s.copy_ad(233, 232);

        s.store_scalar(704, 0.95);

        s.store_offset_sub_scaled_inputs_indices(703, 233, s.v[704], 435, 1.0, (-0.001));

        s.store_sqrt_add_scaled_square_input(705, 703, 1.0, 233, ((4.0 * s.v[704]) * 0.001));

        s.store_add_scaled_inputs3_indices(706, 233, s.v[704], 703, (-0.5), 705, (-0.5));

        s.store_sub(234, 233, 706);

        s.store_sqrt(235, 234);

        s.b[714] = (p.p72 != 0.0);
        s.store_scalar(714, if s.b[714] { 1.0 } else { 0.0 });

        if s.b[714] {
            s.store_scale(708, 544, ((2.0 * 1.6021918e-19) * 1.034943e-10));
        }

        if s.b[714] {
            if (s.v[56] < 3.0) {
                s.store_sqrt_mul(709, 708, 236);
            } else {
                s.store_sqrt_mul_sub_rhs(709, 708, 236, 435);
            }
        }

        if s.b[714] {
            s.store_add_scaled_product_value_ad(183, A::offset(s.ad_value(236), s.v[123]), 1.0, 709, 324, 1.0);
            s.store_scale(708, 324, 1.034943e-10);
            s.store_scalar(711, (1.0 / (p.p72 * p.p72)));
            s.store_scaled_mul(710, 184, 711, 2.0);
            s.store_mul_ad_product_rhs_mixed_ia(712, 708, 710, A::sub_from_scalar(p.p69, s.ad_value(233)));
            s.copy_ad(713, 712);
            s.store_sub(708, 318, 183);
            s.store_scalar(707, (s.v[78] / p.p72));
            s.store_offset_mul(709, 707, 234, p.p80);
            s.store_scalar(712, s.v[77]);
            s.store_add_scaled_product_indices(710, 709, 1.0, 712, 173, 1.0);
            s.store_mul3_lhs(319, 708, 713, 710);
        }

        if (!s.b[714]) {
            s.store_scalar(319, 0.0);
        }

        s.store_scale(715, 184, (1.034943e-10 * 2.0));

        s.store_mul(716, 324, 715);

        s.store_sub_from_scalar(717, p.p69, 233);

        s.store_scalar(718, (s.v[99] - p.p71));

        s.store_scalar(719, (1.0 / (s.v[718] * s.v[718])));

        s.store_scaled_mul(721, 716, 717, s.v[719]);

        s.store_scalar(716, (s.v[76] / s.v[99]));

        s.store_offset_scaled(719, 234, s.v[716], p.p83);

        s.store_add_scaled_inputs(720, 719, 1.0, 173, s.v[75]);

        s.store_mul(187, 721, 720);

        s.b[725] = (p.p86 > 0.0);
        s.store_scalar(725, if s.b[725] { 1.0 } else { 0.0 });

        if s.b[725] {
            s.store_add_scaled_inputs3_offset_indices(722, 237, 1.0, 231, 1.0, 173, p.p87, (-(2.0 * p.p88)));
            s.store_scalar(723, ((s.v[99] * 0.5) + s.v[74]));
            s.store_div_from_scalar(724, (p.p86 * p.p237), 723);
            s.store_mul(188, 722, 724);
        }

        if (!s.b[725]) {
            s.store_scalar(188, 0.0);
        }

        s.copy_ad(726, 324);

        s.store_div_from_scalar_add_ad(727, 1.0, s.ad_value(323), A::div_from_scalar(s.v[72], s.ad_value(105)));

        s.store_sub(728, 726, 727);

        s.store_offset_mul(189, 245, 728, (p.p105 / s.v[109]));

        s.store_add_scaled_inputs4_offset_indices(185, 187, 1.0, 319, 1.0, 189, 1.0, 188, 1.0, s.v[125]);

        s.store_sub(182, 318, 185);

        s.b[732] = (p.p89 == 0.0);
        s.store_scalar(732, if s.b[732] { 1.0 } else { 0.0 });

        if s.b[732] {
            s.store_scalar(147, 0.0);
        }

        if (!s.b[732]) {
            s.store_scalar(147, 1.0);
        }

        s.b[733] = (s.v[147] == 0.0);
        s.store_scalar(733, if s.b[733] { 1.0 } else { 0.0 });

        if s.b[733] {
            s.store_scalar(320, 0.0);
        }

        if (!s.b[733]) {
            s.copy_ad(729, 174);
            s.store_scalar(730, s.v[121]);
            s.store_offset(731, 729, (-p.p90));
        }

        s.b[734] = (s.v[731] < (-3.0));
        s.store_scalar(734, if s.b[734] { 1.0 } else { 0.0 });

        if ((!s.b[733]) && s.b[734]) {
            s.store_scalar(320, 0.0);
        }

        s.b[735] = (s.v[731] < 0.0);
        s.store_scalar(735, if s.b[735] { 1.0 } else { 0.0 });

        if (((!s.b[733]) && (!s.b[734])) && s.b[735]) {
            s.store_offset_mul_offset_rhs_ad_rhs(320, 731, A::mul(s.ad_value(731), A::scale_offset(s.ad_value(731), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);
        }

        if (((!s.b[733]) && (!s.b[734])) && (!s.b[735])) {
            s.store_offset_mul_offset_rhs_ad_rhs(320, 731, A::mul_offset_rhs(s.ad_value(731), A::mul(s.ad_value(731), A::scale_offset(s.ad_value(731), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);
        }

        if (!s.b[733]) {
            s.store_sqrt_offset_square_offset(44, 320, (-1.0), ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_mixed_ai(320, A::offset(s.ad_value(320), (-1.0)), 0.5, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[736] = (s.v[320] < 0.0);
        s.store_scalar(736, if s.b[736] { 1.0 } else { 0.0 });

        if ((!s.b[733]) && s.b[736]) {
            s.store_scalar(320, 0.0);
        }

        if (!s.b[733]) {
            s.store_mul(320, 320, 730);
            s.store_offset_sub_from_scalar_ad(44, 1.0, s.ad_value(320), (-0.05));
            s.store_scalar(45, (4.0 * 0.05));
        }

        if (!s.b[733]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (!s.b[733]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_add_scaled_inputs_indices(320, 44, (-0.5), 45, (-0.5), 1.0);
        }

        s.store_add_scaled_inputs3_offset_indices(159, 158, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]));

        s.copy_ad(178, 159);

        s.store_ln_scaled_input(328, 544, 1.0 / (s.v[66]));

        s.store_mul(342, 227, 328);

        s.store_add_ad_lhs(160, A::sub_from_scalar(s.v[123], s.ad_value(185)), 320);

        s.store_mul(240, 238, 324);

        s.store_square(241, 240);

        s.b[737] = (p.p43 == 0.0);
        s.store_scalar(737, if s.b[737] { 1.0 } else { 0.0 });

        if s.b[737] {
            s.store_scalar(742, 7.0);
            s.store_offset(399, 231, 1.0);
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::offset(s.ad_value(399), (-s.v[383])), A::offset(s.ad_value(399), (-s.v[383])));
            s.store_add_ad_rhs(330, 225, A::div_scalar_offset_denominator(2.0, s.ad_value(399), (-s.v[383]), 1.0));
            s.store_div_ln_lhs(180, 329, 330);
            s.store_sqrt_mul(403, 547, 180);
        }

        if s.b[737] {
            if (s.v[403] > p.p237) {
                s.store_scalar(403, p.p237);
            } else {
            }
        }

        if s.b[737] {
            s.store_scaled_mul(406, 544, 403, (-1.6021918e-19));
            s.store_scalar(740, p.p237);
            s.store_scaled_mul(341, 544, 740, (-1.6021918e-19));
            s.store_scalar(741, 1.5);
            s.store_div_from_scalar(738, 1.034943e-10, 740);
            s.store_div_from_scalar(739, 1.0, 738);
            s.store_scale(743, 341, (-0.001));
            s.store_scale(744, 341, (-1e-5));
        }

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[737] && (p.p39 != 0.0)) {
            s.store_add(475, 172, 342);
        }

        if (s.b[737] && (p.p39 == 0.0)) {
            s.store_add(475, 156, 342);
        }

        if s.b[737] {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(382, 2.0, 225, A::ln(A::div_from_scalar(s.v[66], s.ad_value(230))));
            s.store_scaled_square(745, 474, (s.v[95] * s.v[95]));
            s.store_neg(746, 475);
            s.store_add_scaled_inputs3_mixed_aai(747, A::square(A::add_scaled_product(s.ad_value(746), 2.0, s.ad_value(745), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(746)), (-4.0), 745, (-4.0));
        }

        if s.b[737] {
            if (s.v[747] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(747, (10.0 * 2.220446049250313e-16));
            }
        }

        if s.b[737] {
            s.store_sqrt(747, 747);
            s.store_add_scaled_product_indices(748, 746, 2.0, 745, 225, 1.0);
            s.store_scaled_sub(749, 748, 747, 0.5);
            s.store_div_ad(750, A::ln(A::div_scaled_product_by_product(s.ad_value(746), s.ad_value(746), 1.0, s.ad_value(745), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(746))));
        }

        s.b[751] = (s.v[749] < s.v[382]);
        s.store_scalar(751, if s.b[751] { 1.0 } else { 0.0 });

        if (s.b[737] && s.b[751]) {
            s.copy_ad(387, 749);
        }

        if (s.b[737] && (!s.b[751])) {
            s.store_offset_sub(44, 750, 749, (-0.0008));
            s.store_scale(45, 750, (4.0 * 0.0008));
        }

        if (s.b[737] && (!s.b[751])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[737] && (!s.b[751])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(387, 750, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if s.b[737] {
            s.store_scalar(167, 0.0);
        }

        let mut assign7410_loop_guard: usize = 0;
        while {
            let assign7410_cond_e5011: f64 = if (s.b[737] && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            assign7410_cond_e5011 != 0.0
        } {
            assign7410_loop_guard += 1;
            assert!(assign7410_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[737] {
                s.copy_ad(752, 474);
                s.store_mul(753, 225, 387);
                s.store_exp_neg_input(754, 753);
            }
            s.b[760] = (s.v[387] > 1e-9);
            s.store_scalar(760, if s.b[760] { 1.0 } else { 0.0 });
            if (s.b[737] && s.b[760]) {
                s.store_exp_mul(755, 225, 387);
                s.store_mul_scaled_sqrt_ad_rhs(756, 752, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(754), s.ad_value(753)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(755), (-1.0), 1.0));
                s.store_mul_div_from_scalar_lhs_ad_mixed_ia(757, s.v[122], 756, A::add_scaled_sub_value_product(1.0, s.ad_value(754), 1.0, s.ad_value(239), s.ad_value(755), 1.0));
            }
            s.b[761] = (s.v[387] < (-1e-9));
            s.store_scalar(761, if s.b[761] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[760])) && s.b[761]) {
                s.store_mul_sqrt_ad_rhs(756, 752, A::offset(A::add(s.ad_value(754), s.ad_value(753)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(757, A::div_from_scalar(s.v[122], s.ad_value(756)), 1.0, 754);
            }
            if ((s.b[737] && (!s.b[760])) && (!s.b[761])) {
                s.store_mul_ad_affine_product_lhs(756, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 387);
                s.store_scaled_sqrt_scaled_input(757, 225, s.v[122], -1.0);
            }
            if s.b[737] {
                s.store_sqrt_add_scaled_square_product(45, 756, 1.0, 743, 743, 4.0);
                s.store_offset_scaled_div(759, 756, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(758, 756, 0.5, 45, 0.5, 743, 1e-10);
            }
            s.b[762] = (s.v[758] < 0.0);
            s.store_scalar(762, if s.b[762] { 1.0 } else { 0.0 });
            if (s.b[737] && s.b[762]) {
                s.store_scalar(758, 0.0);
                s.store_scalar(759, 0.0);
            }
            if s.b[737] {
                s.store_add_scaled_inputs3_indices(44, 341, -1.0, 758, (-1.0), 744, -1.0);
                s.store_scaled_mul(45, 341, 744, (-4.0));
            }
            if s.b[737] {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if s.b[737] {
                s.store_sqrt_square_add(45, 44, 45);
                s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(758, 341, -1.0, 44, (-0.5), 45, (-0.5));
                s.store_mul3_lhs(759, 759, 757, 335);
                s.store_div_scaled_inputs_mixed_ai(390, A::square(s.ad_value(758)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);
                s.store_div_scaled_product_indices(391, 390, 759, 2.0, 758, 1.0);
                s.store_sub_ad_rhs(758, 387, A::div_scaled_inputs4(s.ad_value(756), 1.0 / (s.v[93]), s.ad_value(387), (-1.0), s.ad_value(475), -1.0, s.ad_value(390), 1.0, A::add(A::scale_offset(s.ad_value(757), 1.0 / (s.v[93]), (-1.0)), s.ad_value(391)), 1.0));
            }
            s.b[763] = ((((s.v[758] - s.v[387])) as f64).abs() < 5e-12);
            s.store_scalar(763, if s.b[763] { 1.0 } else { 0.0 });
            if (s.b[737] && s.b[763]) {
                s.store_scalar(167, s.v[57]);
            }
            if s.b[737] {
                s.copy_ad(387, 758);
                s.copy_ad(386, 756);
                s.store_offset(167, 167, 1.0);
            }
        }

        if s.b[737] {
            s.copy_ad(388, 390);
            s.store_sqrt_div_scaled_inputs(765, 388, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);
        }

        s.b[770] = (s.v[765] > (0.99 * s.v[740]));
        s.store_scalar(770, if s.b[770] { 1.0 } else { 0.0 });

        if (s.b[737] && s.b[770]) {
            s.store_div_from_scalar(764, 1.0, 323);
            s.store_scale(765, 740, 9662367879.197212);
            s.store_scalar(766, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(767, 1.0, A::add_scaled_inputs3(s.ad_value(764), 1.0, s.ad_value(765), 1.0, s.ad_value(766), 1.0));
            s.store_sub_from_scalar_scaled_mul(768, 1.0, 767, 764, 1.0);
            s.store_mul_ad_product_rhs_mixed_ia(769, 764, 767, A::sub(A::mul_scaled_rhs(A::add_scaled_inputs(s.ad_value(766), 1.0, s.ad_value(765), 0.5), s.ad_value(341), -1.0), s.ad_value(475)));
            s.store_div(383, 769, 768);
            s.store_add(160, 160, 383);
        }

        if s.b[737] {
            s.store_scaled_mul(771, 155, 157, 0.5);
            s.store_scale(44, 771, (2.0 * 10.0));
            s.store_offset_mul_offset_rhs_ad_rhs(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_div_from_scalar(772, 0.1, 45);
        }

        s.b[773] = (s.v[772] < 5e-12);
        s.store_scalar(773, if s.b[773] { 1.0 } else { 0.0 });

        if (s.b[737] && s.b[773]) {
            s.store_scalar(772, 5e-12);
        }

        if s.b[737] {
            s.copy_ad(330, 772);
            s.store_add_scaled_inputs4_offset_indices(179, 158, 1.0, 330, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]));
            s.store_mul_div_ad_lhs(404, s.ad_value(403), A::mul(s.ad_value(741), s.ad_value(231)), 179);
        }

        s.b[774] = ((s.v[404] < (s.v[740] * 7.0)) && ((s.v[740] * 7.0) >= 0.0));
        s.store_scalar(774, if s.b[774] { 1.0 } else { 0.0 });

        if (s.b[737] && s.b[774]) {
            s.store_sub_scaled_inputs(44, 740, 7.0, 404, 1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 740, 740, (7.0 * 7.0));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[775] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(775, if s.b[775] { 1.0 } else { 0.0 });

        s.b[776] = (2.0 == 1.0);
        s.store_scalar(776, if s.b[776] { 1.0 } else { 0.0 });

        if (((s.b[737] && s.b[774]) && s.b[775]) && s.b[776]) {
            s.store_scalar(55, 1.0);
        }

        s.b[777] = (2.0 == 2.0);
        s.store_scalar(777, if s.b[777] { 1.0 } else { 0.0 });

        if ((((s.b[737] && s.b[774]) && s.b[775]) && (!s.b[776])) && s.b[777]) {
            s.store_scalar(55, 2.0);
        }

        s.b[778] = (2.0 == 4.0);
        s.store_scalar(778, if s.b[778] { 1.0 } else { 0.0 });

        if (((((s.b[737] && s.b[774]) && s.b[775]) && (!s.b[776])) && (!s.b[777])) && s.b[778]) {
            s.store_scalar(55, 3.0);
        }

        s.b[779] = (2.0 == 8.0);
        s.store_scalar(779, if s.b[779] { 1.0 } else { 0.0 });

        if ((((((s.b[737] && s.b[774]) && s.b[775]) && (!s.b[776])) && (!s.b[777])) && (!s.b[778])) && s.b[779]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[737] && s.b[774]) && s.b[775]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign7880_loop_guard: usize = 0;
        while {
            let assign7880_cond_e5764: f64 = if (((s.b[737] && s.b[774]) && s.b[775]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign7880_cond_e5764 != 0.0
        } {
            assign7880_loop_guard += 1;
            assert!(assign7880_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[737] && s.b[774]) && s.b[775]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[737] && s.b[774]) && (!s.b[775])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (s.b[737] && s.b[774]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 740, 7.0, 0.0, 53);
            s.store_sub_scaled_inputs(405, 740, 7.0, 43, 1.0);
        }

        if (s.b[737] && (!s.b[774])) {
            s.copy_ad(405, 404);
        }

        s.b[780] = ((s.v[405] > (s.v[403] - s.v[740])) && (s.v[740] >= 0.0));
        s.store_scalar(780, if s.b[780] { 1.0 } else { 0.0 });

        if (s.b[737] && s.b[780]) {
            s.store_add_scaled_inputs3_indices(44, 405, 1.0, 403, (-1.0), 740, 1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 740, 740, 1.0);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[781] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(781, if s.b[781] { 1.0 } else { 0.0 });

        s.b[782] = (2.0 == 1.0);
        s.store_scalar(782, if s.b[782] { 1.0 } else { 0.0 });

        if (((s.b[737] && s.b[780]) && s.b[781]) && s.b[782]) {
            s.store_scalar(55, 1.0);
        }

        s.b[783] = (2.0 == 2.0);
        s.store_scalar(783, if s.b[783] { 1.0 } else { 0.0 });

        if ((((s.b[737] && s.b[780]) && s.b[781]) && (!s.b[782])) && s.b[783]) {
            s.store_scalar(55, 2.0);
        }

        s.b[784] = (2.0 == 4.0);
        s.store_scalar(784, if s.b[784] { 1.0 } else { 0.0 });

        if (((((s.b[737] && s.b[780]) && s.b[781]) && (!s.b[782])) && (!s.b[783])) && s.b[784]) {
            s.store_scalar(55, 3.0);
        }

        s.b[785] = (2.0 == 8.0);
        s.store_scalar(785, if s.b[785] { 1.0 } else { 0.0 });

        if ((((((s.b[737] && s.b[780]) && s.b[781]) && (!s.b[782])) && (!s.b[783])) && (!s.b[784])) && s.b[785]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[737] && s.b[780]) && s.b[781]) {
            s.store_scalar(54, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
    ) {
        let mut assign8200_loop_guard: usize = 0;
        while {
            let assign8200_cond_e6066: f64 = if (((s.b[737] && s.b[780]) && s.b[781]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign8200_cond_e6066 != 0.0
        } {
            assign8200_loop_guard += 1;
            assert!(assign8200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[737] && s.b[780]) && s.b[781]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[737] && s.b[780]) && (!s.b[781])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (s.b[737] && s.b[780]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 740, 53);
            s.store_add_scaled_inputs3_indices(405, 403, 1.0, 740, (-1.0), 43, 1.0);
        }

        if (s.b[737] && (!s.b[780])) {
        }

        if s.b[737] {
            s.store_mul_neg_lhs(369, 405, 229);
            s.store_add_scaled_product_indices(384, 227, 1.0, 341, 740, ((-0.5) * 9662367879.197212));
            s.store_add_scaled_product_indices(385, 384, 1.0, 386, 740, (-9662367879.197212));
        }

        s.b[786] = (s.v[144] >= 1.0);
        s.store_scalar(786, if s.b[786] { 1.0 } else { 0.0 });

        if (s.b[737] && s.b[786]) {
            s.store_scalar(349, s.v[619]);
            s.store_scalar(350, s.v[620]);
            s.store_scalar(351, s.v[621]);
        }

        if (s.b[737] && s.b[786]) {
            s.store_scalar(339, (if (s.v[349] < s.v[385]) { 1.0 } else { 2.0 }));
        }

        if (s.b[737] && (!s.b[786])) {
            s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);
        }

        if (s.b[737] && (!s.b[786])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if (s.b[737] && (!s.b[786])) {
            s.store_add_product3_rhs_mixed_iia(376, 178, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);
            s.store_mul(181, 225, 376);
        }

        s.b[787] = (s.v[181] < 3.0);
        s.store_scalar(787, if s.b[787] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[786])) && s.b[787]) {
            s.store_mul_sub_rhs(337, 225, 178, 156);
            s.store_div_from_scalar_scaled_mul(328, 1.0, 225, 240, (1.414213562373095 / 108.0));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(376, 156, 1.0, 336, 227, 1.0);
            s.copy_ad(378, 376);
        }

        s.b[788] = ((s.v[158] - s.v[383]) <= s.v[182]);
        s.store_scalar(788, if s.b[788] { 1.0 } else { 0.0 });

        if (((s.b[737] && (!s.b[786])) && (!s.b[787])) && s.b[788]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 740, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 178, 331, 323);
            s.copy_ad(378, 376);
        }

        if (((s.b[737] && (!s.b[786])) && (!s.b[787])) && (!s.b[788])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));
            s.store_div_ln_lhs(377, 329, 330);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((s.b[737] && (!s.b[786])) && (!s.b[787])) && (!s.b[788])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[737] && (!s.b[786])) && (!s.b[787])) && (!s.b[788])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if (s.b[737] && (!s.b[786])) {
            if (s.v[378] > 0.0) {
                s.store_sqrt_div_scaled_inputs(401, 378, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);
            } else {
                s.store_scalar(401, 0.0);
            }
        }

        s.b[789] = (s.v[401] < s.v[740]);
        s.store_scalar(789, if s.b[789] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[786])) && s.b[789]) {
            s.store_scalar(339, 1.0);
        }

        if ((s.b[737] && (!s.b[786])) && (!s.b[789])) {
            s.store_scalar(339, 2.0);
        }

        s.b[790] = ((s.v[158] - s.v[383]) <= s.v[182]);
        s.store_scalar(790, if s.b[790] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[786])) && s.b[790]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 740, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 178, 331, 323);
            s.copy_ad(378, 376);
        }

        if ((s.b[737] && (!s.b[786])) && (!s.b[790])) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 740, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));
            s.store_mul_ad_rhs(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));
            s.store_sub_div_rhs_indices(376, 178, 331, 323);
            s.copy_ad(378, 376);
        }

        s.b[791] = ((s.v[178] - s.v[383]) > 0.0);
        s.store_scalar(791, if s.b[791] { 1.0 } else { 0.0 });

        if (((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));
            s.store_div_ln_lhs(377, 329, 330);
        }

        s.b[792] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));
        s.store_scalar(792, if s.b[792] { 1.0 } else { 0.0 });

        if ((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) {
            s.store_offset_sub_scaled_inputs_indices(44, 376, 1.0, 377, 0.98, 0.4);
            s.store_square(49, 44);
            s.store_scalar(50, (0.4 * 0.4));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[793] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(793, if s.b[793] { 1.0 } else { 0.0 });

        s.b[794] = (2.0 == 1.0);
        s.store_scalar(794, if s.b[794] { 1.0 } else { 0.0 });

        if ((((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) && s.b[794]) {
            s.store_scalar(55, 1.0);
        }

        s.b[795] = (2.0 == 2.0);
        s.store_scalar(795, if s.b[795] { 1.0 } else { 0.0 });

        if (((((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) && (!s.b[794])) && s.b[795]) {
            s.store_scalar(55, 2.0);
        }

        s.b[796] = (2.0 == 4.0);
        s.store_scalar(796, if s.b[796] { 1.0 } else { 0.0 });

        if ((((((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) && (!s.b[794])) && (!s.b[795])) && s.b[796]) {
            s.store_scalar(55, 3.0);
        }

        s.b[797] = (2.0 == 8.0);
        s.store_scalar(797, if s.b[797] { 1.0 } else { 0.0 });

        if (((((((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) && (!s.b[794])) && (!s.b[795])) && (!s.b[796])) && s.b[797]) {
            s.store_scalar(55, 4.0);
        }

        if (((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e7415: f64 = if ((((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign9160_cond_e7415 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && s.b[793]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) && (!s.b[793])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && s.b[792]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 0.4);
            s.store_add_ad_lhs(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);
        }

        if ((((s.b[737] && (!s.b[786])) && (!s.b[790])) && s.b[791]) && (!s.b[792])) {
            s.copy_ad(378, 376);
        }

        if (s.b[737] && (!s.b[786])) {
            s.copy_ad(349, 378);
            s.copy_ad(163, 376);
            s.store_sub_ad_lhs(328, A::add_scaled_product(s.ad_value(349), 1.0, s.ad_value(341), s.ad_value(739), 0.5), 475);
        }

        s.b[798] = (s.v[328] < 0.0);
        s.store_scalar(798, if s.b[798] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[786])) && s.b[798]) {
            s.store_mul_offset_rhs(329, 474, 739, s.v[94]);
            s.store_square(329, 329);
            s.store_offset_scaled(332, 328, (-1.6), 0.6);
            s.store_scalar(331, 0.5);
            s.store_add_scaled_inputs3_indices(44, 332, 1.0, 331, (-1.0), 332, (-0.001));
            s.store_scaled_mul(45, 332, 332, (4.0 * 0.001));
        }

        if ((s.b[737] && (!s.b[786])) && s.b[798]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if ((s.b[737] && (!s.b[786])) && s.b[798]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(331, 332, 1.0, 44, (-0.5), 45, (-0.5));
            s.store_mul3_lhs(330, 329, 331, 226);
            s.store_div_ad(351, A::mul_sub_from_scalar_rhs(s.ad_value(328), 1.0, A::sqrt(s.ad_value(330))), A::sub_from_scalar(1.0, s.ad_value(330)));
        }

        if ((s.b[737] && (!s.b[786])) && (!s.b[798])) {
            s.store_scaled_square(327, 474, (s.v[95] * s.v[95]));
            s.store_neg_ad(328, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(349), (-1.0), s.ad_value(341), s.ad_value(740), (-(1.0 / (2.0) * 9662367879.197212))));
            s.store_add_scaled_inputs3_mixed_aai(329, A::square(A::add_scaled_product(s.ad_value(328), 2.0, s.ad_value(327), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(328)), (-4.0), 327, (-4.0));
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[737] && (!s.b[786])) && (!s.b[798])) {
            if (s.v[329] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(329, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((s.b[737] && (!s.b[786])) && (!s.b[798])) {
            s.store_sqrt(329, 329);
            s.store_add_scaled_product_indices(330, 328, 2.0, 327, 225, 1.0);
            s.store_scaled_sub(380, 330, 329, 0.5);
            s.store_div_ad(381, A::ln(A::div_scaled_product_by_product(s.ad_value(328), s.ad_value(328), 1.0, s.ad_value(327), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(328))));
        }

        s.b[799] = (s.v[380] < s.v[382]);
        s.store_scalar(799, if s.b[799] { 1.0 } else { 0.0 });

        if (((s.b[737] && (!s.b[786])) && (!s.b[798])) && s.b[799]) {
            s.copy_ad(351, 380);
        }

        if (((s.b[737] && (!s.b[786])) && (!s.b[798])) && (!s.b[799])) {
            s.store_offset_sub(44, 381, 380, (-0.0008));
            s.store_scale(45, 381, (4.0 * 0.0008));
        }

        if (((s.b[737] && (!s.b[786])) && (!s.b[798])) && (!s.b[799])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[737] && (!s.b[786])) && (!s.b[798])) && (!s.b[799])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(351, 381, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if (s.b[737] && (!s.b[786])) {
            s.store_scalar(167, 0.0);
        }

        let mut assign9530_loop_guard: usize = 0;
        while {
            let assign9530_cond_e7998: f64 = if ((s.b[737] && (!s.b[786])) && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            assign9530_cond_e7998 != 0.0
        } {
            assign9530_loop_guard += 1;
            assert!(assign9530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[737] && (!s.b[786])) {
                s.copy_ad(328, 474);
                s.store_mul(329, 225, 351);
                s.store_exp_neg_input(330, 329);
            }
            s.b[800] = (s.v[351] > 1e-9);
            s.store_scalar(800, if s.b[800] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[786])) && s.b[800]) {
                s.store_exp_mul(327, 225, 351);
                s.store_mul_scaled_sqrt_ad_rhs(331, 328, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(327), (-1.0), 1.0));
                s.store_mul_div_from_scalar_lhs_ad_mixed_ia(332, s.v[122], 331, A::add_scaled_sub_value_product(1.0, s.ad_value(330), 1.0, s.ad_value(239), s.ad_value(327), 1.0));
            }
            s.b[801] = (s.v[351] < (-1e-9));
            s.store_scalar(801, if s.b[801] { 1.0 } else { 0.0 });
            if (((s.b[737] && (!s.b[786])) && (!s.b[800])) && s.b[801]) {
                s.store_mul_sqrt_ad_rhs(331, 328, A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(332, A::div_from_scalar(s.v[122], s.ad_value(331)), 1.0, 330);
            }
            if (((s.b[737] && (!s.b[786])) && (!s.b[800])) && (!s.b[801])) {
                s.store_mul_ad_affine_product_lhs(331, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 351);
                s.store_scaled_sqrt_scaled_input(332, 225, s.v[122], -1.0);
            }
            if (s.b[737] && (!s.b[786])) {
                s.store_sqrt_add_scaled_square_product(45, 331, 1.0, 743, 743, 4.0);
                s.store_offset_scaled_div(334, 331, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(333, 331, 0.5, 45, 0.5, 743, 1e-10);
            }
            s.b[802] = (s.v[333] < 0.0);
            s.store_scalar(802, if s.b[802] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[786])) && s.b[802]) {
                s.store_scalar(333, 0.0);
                s.store_scalar(334, 0.0);
            }
            if (s.b[737] && (!s.b[786])) {
                s.store_add_scaled_inputs3_indices(44, 341, -1.0, 333, (-1.0), 744, -1.0);
                s.store_scaled_mul(45, 341, 744, (-4.0));
            }
            if (s.b[737] && (!s.b[786])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if (s.b[737] && (!s.b[786])) {
                s.store_sqrt_square_add(45, 44, 45);
                s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(333, 341, -1.0, 44, (-0.5), 45, (-0.5));
                s.store_mul3_lhs(334, 334, 332, 335);
                s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(333)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);
                s.store_div_scaled_product_indices(389, 388, 334, 2.0, 333, 1.0);
                s.store_sub_ad_rhs(333, 351, A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(349), 1.0, s.ad_value(351), (-1.0), s.ad_value(331), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(341), 0.5), s.ad_value(740), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(332), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(332), s.ad_value(740), 9662367879.197212), s.ad_value(389)), 1.0));
                s.copy_ad(334, 167);
            }
            s.b[803] = ((((s.v[333] - s.v[351])) as f64).abs() < 0.001);
            s.store_scalar(803, if s.b[803] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[786])) && s.b[803]) {
                s.store_scalar(167, s.v[57]);
            }
            if (s.b[737] && (!s.b[786])) {
                s.copy_ad(351, 333);
                s.copy_ad(357, 331);
                s.store_offset(167, 167, 1.0);
            }
        }

        if (s.b[737] && (!s.b[786])) {
            s.store_add(351, 475, 351);
            s.store_add_scaled_product_right_ad(350, 349, 1.0, 739, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);
        }

        s.b[804] = ((p.p25 == 1.0) && (s.v[158] > (s.v[160] + 0.2)));
        s.store_scalar(804, if s.b[804] { 1.0 } else { 0.0 });

        if (s.b[737] && s.b[804]) {
            s.store_scalar(446, s.v[136]);
            s.store_add_scaled_inputs4_indices(445, 174, 1.0, 446, (-1.0), 185, 1.0, 320, -1.0);
            s.store_scalar(143, p.p137);
            s.copy_ad(207, 445);
            s.store_sqrt_div_scaled_inputs(208, 544, ((2.0 * 1.6021918e-19) * 1.034943e-10), 225, 1.0);
            s.store_div_scaled_product_by_product(209, s.ad_value(230), s.ad_value(230), 1.0, s.ad_value(544), s.ad_value(544), 1.0);
            s.store_div_scaled_product_by_product(210, s.ad_value(208), s.ad_value(208), 1.0, s.ad_value(323), s.ad_value(323), 1.0);
            s.store_scaled_mul(211, 210, 225, 0.5);
            s.store_scaled_mul(212, 211, 225, 2.0);
            s.store_sqrt_offset_ad(213, A::div_scaled_offset_numerator(A::mul(s.ad_value(225), s.ad_value(207)), 4.0, ((-1.0) * 4.0), s.ad_value(212), 1.0), 1.0);
            s.store_add_mul_sub_from_scalar_rhs_indices(215, 207, 211, 1.0, 213);
            s.store_div_scalar_by_product(223, 1.0, s.ad_value(209), s.ad_value(210), 1.0);
            s.store_div_ad(216, A::ln(A::mul(s.ad_value(223), A::square(s.ad_value(207)))), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(207))));
            s.store_add_scaled_inputs3_indices(217, 216, 1.0, 215, (-1.0), 143, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(218, 216, 1.0, 217, (-0.5), A::add_scaled_square_product(s.ad_value(217), 1.0, s.ad_value(143), s.ad_value(216), 4.0), (-0.5));
            s.store_exp_mul(224, 225, 218);
            s.store_add_scaled_product_value_ad(219, A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), 1.0, 209, 224, 1.0);
            s.store_offset_mul(220, 225, 218, (-1.0));
        }

        s.b[805] = ((s.v[219] > 0.0) && (s.v[220] > 0.0));
        s.store_scalar(805, if s.b[805] { 1.0 } else { 0.0 });

        if ((s.b[737] && s.b[804]) && s.b[805]) {
            s.store_sqrt_ad(219, A::add_scaled_product(A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), 1.0, s.ad_value(209), s.ad_value(224), 1.0));
            s.store_sqrt_offset_ad(220, A::mul(s.ad_value(225), s.ad_value(218)), (-1.0));
            s.store_mul_sub_rhs(221, 208, 219, 220);
            s.store_div_scaled_inputs_indices(214, 105, 2.0, 225, 1.0);
            s.store_scalar(250, (300.0 * 0.0001));
            s.store_scalar(316, 0.0);
            s.store_scalar(328, 0.0);
            s.store_div_from_scalar_sub_from_scalar_ad(329, 1.0, s.v[97], s.ad_value(316));
            s.store_mul_ad_product_lhs_mixed_ai(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 328, 329);
            s.copy_ad(394, 222);
            s.copy_ad(395, 218);
            s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);
        }

        s.b[806] = (s.v[336] < (10.0 * 2.220446049250313e-16));
        s.store_scalar(806, if s.b[806] { 1.0 } else { 0.0 });

        if (((s.b[737] && s.b[804]) && s.b[805]) && s.b[806]) {
            s.store_scalar(336, (10.0 * 2.220446049250313e-16));
        }

        if ((s.b[737] && s.b[804]) && s.b[805]) {
            s.store_add_product3_rhs_mixed_iia(376, 178, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);
            s.copy_ad(163, 376);
            s.store_sub(166, 376, 395);
        }

        s.b[807] = (s.v[166] < 0.0);
        s.store_scalar(807, if s.b[807] { 1.0 } else { 0.0 });

        if (((s.b[737] && s.b[804]) && s.b[805]) && s.b[807]) {
            s.store_scalar(166, 0.0);
        }

        if ((s.b[737] && s.b[804]) && s.b[805]) {
            s.store_scale(332, 166, (1.0 + 0.3));
            s.store_offset_sub(333, 332, 173, (-0.03));
            s.store_sqrt_add_scaled_square_input(334, 333, 1.0, 332, (4.0 * 0.03));
            s.store_add_scaled_inputs3_indices(165, 332, 1.0, 333, (-0.5), 334, (-0.5));
        }

        s.b[808] = (s.v[165] > s.v[166]);
        s.store_scalar(808, if s.b[808] { 1.0 } else { 0.0 });

        if (((s.b[737] && s.b[804]) && s.b[805]) && s.b[808]) {
            s.copy_ad(165, 166);
        }

        if ((s.b[737] && s.b[804]) && s.b[805]) {
            s.copy_ad(449, 165);
            s.store_scalar(826, (s.v[88] * 100.0));
            s.store_scale(827, 107, 100.0);
            s.store_scalar(828, (s.v[97] * 100.0));
        }

        s.b[829] = (p.p36 == 0.0);
        s.store_scalar(829, if s.b[829] { 1.0 } else { 0.0 });

        if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
            s.store_scalar(448, 4.12);
            s.store_scaled_mul(809, 827, 828, (p.p142 * 1.6021918e-19));
            s.store_div(810, 809, 302);
            s.store_div_scaled_inputs_mixed_ai(811, A::offset(A::add_scaled_inputs4(s.ad_value(514), p.p145, s.ad_value(187), 1.0, s.ad_value(319), 1.0, s.ad_value(237), 1.0), p.p144), -1.0, 826, 1.0);
            s.store_scalar(562, 0.0);
        }

        let mut assign10120_loop_guard: usize = 0;
        while {
            let assign10120_cond_e9100: f64 = (100.0 - 1.0);
            let assign10120_cond_e9102: f64 = if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && (s.v[562] <= assign10120_cond_e9100)) { 1.0 } else { 0.0 };
            assign10120_cond_e9102 != 0.0
        } {
            assign10120_loop_guard += 1;
            assert!(assign10120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
                s.copy_ad(812, 562);
                s.store_scalar(813, 100.0);
                s.store_div(814, 812, 813);
                s.store_add_scaled_inputs3_mixed_iia(815, 159, 1.0, 175, 1.0, A::add_scaled_product(s.ad_value(395), 1.0, s.ad_value(449), s.ad_value(814), 1.0), -1.0);
                s.store_sub_from_scalar_div_indices(816, 1.0, 815, 448);
                s.store_add_div_rhs_indices(819, 811, 815, 826);
                s.store_square(817, 819);
                s.store_sqrt_square_offset(44, 816, ((4.0 * 0.001) * 0.001));
                s.store_offset_add_scaled_inputs_indices(816, 816, 0.5, 44, 0.5, (1e-10 * 0.001));
            }
            s.b[830] = (s.v[816] < 0.0);
            s.store_scalar(830, if s.b[830] { 1.0 } else { 0.0 });
            if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && s.b[830]) {
                s.store_scalar(816, 0.0);
            }
            if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
                s.store_offset_scaled_ad(818, A::mul(A::sqrt(s.ad_value(816)), s.ad_value(816)), (-p.p143), p.p143);
                s.store_div_scaled_inputs_indices(820, 818, -1.0, 819, 1.0);
            }
            s.b[831] = (s.v[820] < (-34.0));
            s.store_scalar(831, if s.b[831] { 1.0 } else { 0.0 });
            if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && s.b[831]) {
                s.store_scalar(822, 0.0);
            }
            if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && (!s.b[831])) {
                s.store_exp(822, 820);
            }
            if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
                s.copy_ad(823, 810);
                s.store_mul3_affine_lhs(824, 823, 818, (0.25 * 7.38905609893065), 0.0, 818);
            }
            s.b[832] = (((2.0 * s.v[819]) + s.v[818]) < 0.0);
            s.store_scalar(832, if s.b[832] { 1.0 } else { 0.0 });
            if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && s.b[832]) {
                s.copy_ad(450, 824);
            }
            if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && (!s.b[832])) {
                s.copy_ad(821, 809);
                s.store_mul3_lhs(825, 821, 817, 822);
            }
            s.b[833] = ((s.v[825] < s.v[824]) || (s.v[819] < 0.0));
            s.store_scalar(833, if s.b[833] { 1.0 } else { 0.0 });
            if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && (!s.b[832])) && s.b[833]) {
                s.copy_ad(450, 824);
            }
            if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && (!s.b[832])) && (!s.b[833])) {
                s.copy_ad(450, 825);
            }
            s.b[834] = (s.v[450] < 1e-9);
            s.store_scalar(834, if s.b[834] { 1.0 } else { 0.0 });
            if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) && s.b[834]) {
                s.store_scalar(562, 100.0);
                s.store_scalar(167, s.v[57]);
            }
            if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[829])) {
                s.store_offset(562, 562, 1.0);
            }
        }

        s.b[847] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));
        s.store_scalar(847, if s.b[847] { 1.0 } else { 0.0 });

        if (((s.b[737] && s.b[804]) && s.b[805]) && s.b[847]) {
            s.store_scalar(263, 0.0);
        }

        s.b[848] = (p.p44 <= 0.0);
        s.store_scalar(848, if s.b[848] { 1.0 } else { 0.0 });

        if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && s.b[848]) {
            s.copy_ad(835, 445);
            s.store_square(842, 323);
            s.copy_ad(843, 545);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && s.b[848]) {
            s.store_div(837, 843, 842);
            s.store_div_from_scalar(844, 2.0, 843);
            s.store_mul(838, 844, 842);
            s.store_add_scaled_inputs_product_indices(839, 835, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_offset_mul(841, 838, 839, 1.0);
            s.store_sqrt_square_offset(44, 841, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(840, 841, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[849] = (s.v[840] < 0.0);
        s.store_scalar(849, if s.b[849] { 1.0 } else { 0.0 });

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && s.b[848]) && s.b[849]) {
            s.store_scalar(840, 0.0);
        }

        if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && s.b[848]) {
            s.store_offset(840, 840, 1e-50);
            s.store_sqrt(840, 840);
            s.store_add_scaled_product_value_ad(845, A::mul_sub_from_scalar_rhs(s.ad_value(837), 1.0, s.ad_value(840)), 1.0, 835, 137, 1.0);
            s.store_add_scaled_inputs3_mixed_iia(846, 173, p.p122, 395, 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(845)), -1.0);
            s.store_sqrt_square_offset(44, 846, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(846, 846, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[850] = (s.v[846] < 0.0);
        s.store_scalar(850, if s.b[850] { 1.0 } else { 0.0 });

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && s.b[848]) && s.b[850]) {
            s.store_scalar(846, 0.0);
        }

        if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) {
            s.store_mul(835, 134, 445);
            s.store_div_square_rhs(837, 545, 323);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(838, 2.0, 545, A::square(s.ad_value(323)));
            s.store_add_scaled_inputs_product_indices(839, 835, 1.0, 227, (-1.0), 130, 514, (-1.0));
            s.store_offset_mul(840, 838, 839, 1.0);
            s.store_scaled_offset(842, 838, 1.0, 2.0);
        }

        s.b[851] = ((s.v[840] < (1e-50 + s.v[842])) && (s.v[842] >= 0.0));
        s.store_scalar(851, if s.b[851] { 1.0 } else { 0.0 });

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) {
            s.store_sub_offset_lhs(44, 842, 1e-50, 840);
            s.store_square(49, 44);
            s.store_square(50, 842);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[852] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.store_scalar(852, if s.b[852] { 1.0 } else { 0.0 });

        s.b[853] = (4.0 == 1.0);
        s.store_scalar(853, if s.b[853] { 1.0 } else { 0.0 });

        if (((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) && s.b[853]) {
            s.store_scalar(55, 1.0);
        }

        s.b[854] = (4.0 == 2.0);
        s.store_scalar(854, if s.b[854] { 1.0 } else { 0.0 });

        if ((((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) && (!s.b[853])) && s.b[854]) {
            s.store_scalar(55, 2.0);
        }

        s.b[855] = (4.0 == 4.0);
        s.store_scalar(855, if s.b[855] { 1.0 } else { 0.0 });

        if (((((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) && (!s.b[853])) && (!s.b[854])) && s.b[855]) {
            s.store_scalar(55, 3.0);
        }

        s.b[856] = (4.0 == 8.0);
        s.store_scalar(856, if s.b[856] { 1.0 } else { 0.0 });

        if ((((((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) && (!s.b[853])) && (!s.b[854])) && (!s.b[855])) && s.b[856]) {
            s.store_scalar(55, 4.0);
        }

        if ((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign10720_loop_guard: usize = 0;
        while {
            let assign10720_cond_e10443: f64 = if (((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign10720_cond_e10443 != 0.0
        } {
            assign10720_loop_guard += 1;
            assert!(assign10720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && s.b[852]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) && (!s.b[852])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[851]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 842, 53);
            s.store_sub_offset_lhs(840, 842, 1e-50, 43);
        }

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && (!s.b[851])) {
        }

        if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) {
            if (s.v[840] <= 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_sqrt(840, 840);
            }
        }

        if ((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) {
            s.store_add_mul_sub_from_scalar_rhs_indices(845, 835, 837, 1.0, 840);
            s.store_div_from_scalar_offset_input(836, s.v[100], 131, s.v[100]);
            s.store_add_scaled_product_value_ad(846, A::scale_offset(s.ad_value(173), p.p122, s.v[176]), 1.0, 836, 845, (-1.0));
            s.store_sqrt_square_offset(44, 846, ((4.0 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs_indices(846, 846, 0.5, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[857] = (s.v[846] < 0.0);
        s.store_scalar(857, if s.b[857] { 1.0 } else { 0.0 });

        if (((((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) && (!s.b[848])) && s.b[857]) {
            s.store_scalar(846, 0.0);
        }

        if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[847])) {
            s.store_offset(846, 846, 1e-50);
            s.store_ad_value(836, A::exp_div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(846), 1.0));
            s.store_mul_product3_indices(263, 836, 132, 846, 394, 1.0);
        }

        s.b[865] = (p.p26 == 1.0);
        s.store_scalar(865, if s.b[865] { 1.0 } else { 0.0 });

        if (((s.b[737] && s.b[804]) && s.b[805]) && s.b[865]) {
            s.store_scale(861, 227, 0.0);
            s.store_sqrt_mul_scaled_lhs(862, 544, ((2.0 * 1.034943e-10) * 1.6021918e-19), 227);
            s.store_sqrt_mul_sub_rhs(863, 225, 395, 861);
            s.store_sqrt_mul(864, 225, 395);
            s.store_mul_sub_scaled_inputs_rhs(393, 862, s.ad_value(863), -1.0, s.ad_value(864), -1.0);
        }

        if ((((s.b[737] && s.b[804]) && s.b[805]) && s.b[865]) && (p.p37 != 0.0)) {
            s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
            s.copy_ad(393, 596);
        }

        if (((s.b[737] && s.b[804]) && s.b[805]) && (!s.b[865])) {
            s.store_scalar(393, 0.0);
        }

        if ((s.b[737] && s.b[804]) && (!s.b[805])) {
            s.store_scalar(263, 0.0);
            s.store_scalar(393, 0.0);
        }

        if (s.b[737] && (!s.b[804])) {
            s.store_scalar(263, 0.0);
            s.store_scalar(393, 0.0);
        }

        if s.b[737] {
            s.copy_ad(343, 349);
            s.copy_ad(344, 350);
            s.copy_ad(345, 351);
            s.store_scalar(430, 0.0);
            s.store_scalar(611, 0.0);
            s.store_scalar(167, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
    ) {
        let mut assign11160_loop_guard: usize = 0;
        while {
            let assign11160_cond_e11104: f64 = if (s.b[737] && (s.v[167] <= s.v[57])) { 1.0 } else { 0.0 };
            assign11160_cond_e11104 != 0.0
        } {
            assign11160_loop_guard += 1;
            assert!(assign11160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[737] {
                s.store_sub(867, 351, 475);
                s.store_mul(866, 225, 867);
                s.store_exp_neg_input(327, 866);
            }
            s.b[901] = (s.v[867] < (-1e-9));
            s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });
            if (s.b[737] && s.b[901]) {
                s.store_mul_sqrt_ad_rhs(357, 474, A::offset(A::add(s.ad_value(327), s.ad_value(866)), (-1.0)));
                s.store_div_scaled_offset_numerator(873, s.ad_value(327), (-s.v[122]), s.v[122], s.ad_value(357), 1.0);
            }
            s.b[902] = (s.v[867] > 1e-9);
            s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[901])) && s.b[902]) {
                s.store_exp(868, 866);
                s.store_mul_scaled_sqrt_ad_rhs(357, 474, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(327), s.ad_value(866)), (-1.0)), 1.0, s.ad_value(239), A::add(s.ad_value(868), s.ad_value(866)), (-1.0), 1.0));
                s.store_div_ad_lhs(873, A::add_scaled_sub_value_product(1.0, s.ad_value(327), s.v[122], s.ad_value(239), A::offset(s.ad_value(868), 1.0), s.v[122]), 357);
            }
            if ((s.b[737] && (!s.b[901])) && (!s.b[902])) {
                s.store_mul_neg_lhs(357, 474, 866);
                s.store_mul_neg_lhs(873, 474, 225);
            }
            if s.b[737] {
                s.copy_ad(361, 369);
                s.store_mul(866, 225, 349);
                s.store_exp_mul(871, 225, 349);
                s.store_scalar(869, 1.0);
                s.store_sqrt_ad(870, A::add_scaled_product(A::div_scaled_product(s.ad_value(361), s.ad_value(361), 1.0, A::square(s.ad_value(238)), 1.0), 1.0, s.ad_value(379), A::add_scaled_inputs3(s.ad_value(871), 1.0, s.ad_value(866), 1.0, s.ad_value(869), -1.0), 2.0));
                s.store_div_scaled_product3_mixed_iiai(900, 225, 379, A::offset(s.ad_value(871), 1.0), 2.0, 870, 2.0);
                s.store_add_scaled_product_indices(355, 361, (-1.0), 238, 870, -1.0);
                s.store_mul_neg_lhs(872, 238, 900);
                s.store_div_scaled_inputs2_indices(867, 350, 1.0, 349, (-1.0), 742, 1.0);
                s.store_mul(866, 225, 867);
            }
            s.b[903] = ((-s.v[866]) >= 500.0);
            s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });
            if (s.b[737] && s.b[903]) {
                s.store_scaled_offset_ad(327, A::sub_from_scalar(1.0, s.ad_value(866)), (-500.0), 1.403592217853e217);
                s.store_scalar(333, 1.403592217853e217);
            }
            if (s.b[737] && (!s.b[903])) {
                s.store_neg(44, 866);
                s.store_scalar(327, 1.0);
            }
            let mut assign11160_body27_loop_guard: usize = 0;
            while {
                let assign11160_body27_cond_e11372: f64 = if ((s.b[737] && (!s.b[903])) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                assign11160_body27_cond_e11372 != 0.0
            } {
                assign11160_body27_loop_guard += 1;
                assert!(assign11160_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (s.b[737] && (!s.b[903])) {
                    s.store_scale(327, 327, 1.14200738981568e26);
                    s.store_offset(44, 44, (-60.0));
                }
            }
            if (s.b[737] && (!s.b[903])) {
                s.store_mul_exp_rhs(327, 327, 44);
                s.copy_ad(333, 327);
            }
            if s.b[737] {
                s.store_exp_neg_input(327, 866);
                s.store_sqrt_offset_ad(868, A::add(s.ad_value(327), s.ad_value(866)), (-1.0));
            }
            s.b[904] = (s.v[867] < (-1e-9));
            s.store_scalar(904, if s.b[904] { 1.0 } else { 0.0 });
            if (s.b[737] && s.b[904]) {
                s.store_mul(363, 238, 868);
                s.store_div_scaled_product3_by_product(364, s.ad_value(238), s.ad_value(225), A::sub_from_scalar(1.0, s.ad_value(333)), 1.0, s.ad_value(868), s.ad_value(742), 2.0);
                s.store_neg(365, 364);
            }
            s.b[905] = (s.v[867] > 1e-9);
            s.store_scalar(905, if s.b[905] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[904])) && s.b[905]) {
                s.store_mul_neg_lhs(363, 238, 868);
                s.store_div_scaled_product3_by_product(364, s.ad_value(238), s.ad_value(225), A::sub_from_scalar(1.0, s.ad_value(333)), -1.0, s.ad_value(868), s.ad_value(742), 2.0);
                s.store_neg(365, 364);
            }
            if ((s.b[737] && (!s.b[904])) && (!s.b[905])) {
                s.store_scaled_mul(363, 238, 866, (-0.7071067811865476));
                s.store_scaled_mul(364, 238, 225, (-0.7071067811865476));
                s.store_neg(365, 364);
            }
            s.b[906] = ((s.v[363] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0));
            s.store_scalar(906, if s.b[906] { 1.0 } else { 0.0 });
            if (s.b[737] && s.b[906]) {
                s.store_add_scaled_inputs(44, 363, 1.0, 406, -1.0);
                s.store_square(49, 44);
                s.store_scaled_mul(50, 406, 406, 1.0);
                s.store_scalar(51, 1.0);
                s.store_scalar(52, 1.0);
                s.store_scalar(54, 0.0);
                s.store_scalar(55, 0.0);
                s.store_scalar(48, 0.0);
                s.store_scalar(53, 0.0);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_add(48, 51, 52);
                s.copy_ad(53, 48);
            }
            s.b[907] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(907, if s.b[907] { 1.0 } else { 0.0 });
            s.b[908] = (2.0 == 1.0);
            s.store_scalar(908, if s.b[908] { 1.0 } else { 0.0 });
            if (((s.b[737] && s.b[906]) && s.b[907]) && s.b[908]) {
                s.store_scalar(55, 1.0);
            }
            s.b[909] = (2.0 == 2.0);
            s.store_scalar(909, if s.b[909] { 1.0 } else { 0.0 });
            if ((((s.b[737] && s.b[906]) && s.b[907]) && (!s.b[908])) && s.b[909]) {
                s.store_scalar(55, 2.0);
            }
            s.b[910] = (2.0 == 4.0);
            s.store_scalar(910, if s.b[910] { 1.0 } else { 0.0 });
            if (((((s.b[737] && s.b[906]) && s.b[907]) && (!s.b[908])) && (!s.b[909])) && s.b[910]) {
                s.store_scalar(55, 3.0);
            }
            s.b[911] = (2.0 == 8.0);
            s.store_scalar(911, if s.b[911] { 1.0 } else { 0.0 });
            if ((((((s.b[737] && s.b[906]) && s.b[907]) && (!s.b[908])) && (!s.b[909])) && (!s.b[910])) && s.b[911]) {
                s.store_scalar(55, 4.0);
            }
            if ((s.b[737] && s.b[906]) && s.b[907]) {
                s.store_scalar(54, 0.0);
            }
            let mut assign11160_body69_loop_guard: usize = 0;
            while {
                let assign11160_body69_cond_e11783: f64 = if (((s.b[737] && s.b[906]) && s.b[907]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign11160_body69_cond_e11783 != 0.0
            } {
                assign11160_body69_loop_guard += 1;
                assert!(assign11160_body69_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[737] && s.b[906]) && s.b[907]) {
                    s.store_sqrt(53, 53);
                    s.store_offset(54, 54, 1.0);
                }
            }
            if ((s.b[737] && s.b[906]) && (!s.b[907])) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if (s.b[737] && s.b[906]) {
                s.store_div_from_scalar(53, 1.0, 53);
                s.store_mul3_affine_lhs(899, 44, 406, -1.0, 0.0, 53);
                s.store_div_scaled_product3_indices(327, 406, 52, 53, -1.0, 48, 1.0);
                s.store_add_scaled_inputs_ad_lhs(363, A::neg(s.ad_value(406)), -1.0, 899, 1.0);
            }
            if (s.b[737] && s.b[906]) {
            }
            if (s.b[737] && (!s.b[906])) {
            }
            if (s.b[737] && (!s.b[906])) {
                s.store_scalar(327, 1.0);
            }
            if s.b[737] {
                s.store_mul(364, 364, 327);
                s.store_mul(365, 365, 327);
            }
            s.b[912] = ((s.v[363] < ((s.v[341] - s.v[361]) + (-(s.v[341] - s.v[361])))) && ((-(s.v[341] - s.v[361])) >= 0.0));
            s.store_scalar(912, if s.b[912] { 1.0 } else { 0.0 });
            if (s.b[737] && s.b[912]) {
                s.store_sub_add_scaled_inputs4_lhs_indices(44, 341, 1.0, 361, (-1.0), 341, -1.0, 361, 1.0, 363);
                s.store_square(49, 44);
                s.store_scaled_mul_ad(50, A::sub(s.ad_value(341), s.ad_value(361)), A::sub(s.ad_value(341), s.ad_value(361)), 1.0);
                s.store_scalar(51, 1.0);
                s.store_scalar(52, 1.0);
                s.store_scalar(54, 0.0);
                s.store_scalar(55, 0.0);
                s.store_scalar(48, 0.0);
                s.store_scalar(53, 0.0);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_add(48, 51, 52);
                s.copy_ad(53, 48);
            }
            s.b[913] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(913, if s.b[913] { 1.0 } else { 0.0 });
            s.b[914] = (2.0 == 1.0);
            s.store_scalar(914, if s.b[914] { 1.0 } else { 0.0 });
            if (((s.b[737] && s.b[912]) && s.b[913]) && s.b[914]) {
                s.store_scalar(55, 1.0);
            }
            s.b[915] = (2.0 == 2.0);
            s.store_scalar(915, if s.b[915] { 1.0 } else { 0.0 });
            if ((((s.b[737] && s.b[912]) && s.b[913]) && (!s.b[914])) && s.b[915]) {
                s.store_scalar(55, 2.0);
            }
            s.b[916] = (2.0 == 4.0);
            s.store_scalar(916, if s.b[916] { 1.0 } else { 0.0 });
            if (((((s.b[737] && s.b[912]) && s.b[913]) && (!s.b[914])) && (!s.b[915])) && s.b[916]) {
                s.store_scalar(55, 3.0);
            }
            s.b[917] = (2.0 == 8.0);
            s.store_scalar(917, if s.b[917] { 1.0 } else { 0.0 });
            if ((((((s.b[737] && s.b[912]) && s.b[913]) && (!s.b[914])) && (!s.b[915])) && (!s.b[916])) && s.b[917]) {
                s.store_scalar(55, 4.0);
            }
            if ((s.b[737] && s.b[912]) && s.b[913]) {
                s.store_scalar(54, 0.0);
            }
            let mut assign11160_body106_loop_guard: usize = 0;
            while {
                let assign11160_body106_cond_e12146: f64 = if (((s.b[737] && s.b[912]) && s.b[913]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign11160_body106_cond_e12146 != 0.0
            } {
                assign11160_body106_loop_guard += 1;
                assert!(assign11160_body106_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[737] && s.b[912]) && s.b[913]) {
                    s.store_sqrt(53, 53);
                    s.store_offset(54, 54, 1.0);
                }
            }
            if ((s.b[737] && s.b[912]) && (!s.b[913])) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if (s.b[737] && s.b[912]) {
                s.store_div_from_scalar(53, 1.0, 53);
                s.store_mul_ad_affine_product_lhs(899, s.ad_value(44), A::sub(s.ad_value(341), s.ad_value(361)), -1.0, 0.0, 53);
                s.store_div_scaled_product3_mixed_aiii(327, A::sub(s.ad_value(341), s.ad_value(361)), 52, 53, -1.0, 48, 1.0);
                s.store_sub_add_scaled_inputs4_lhs_indices(363, 341, 1.0, 361, (-1.0), 341, -1.0, 361, 1.0, 899);
            }
            if (s.b[737] && s.b[912]) {
            }
            if (s.b[737] && (!s.b[912])) {
            }
            if (s.b[737] && (!s.b[912])) {
                s.store_scalar(327, 1.0);
            }
            if s.b[737] {
                s.store_mul(365, 365, 327);
                s.store_mul(364, 364, 327);
                s.store_add(356, 361, 363);
            }
            s.b[918] = (s.v[430] == 1.0);
            s.store_scalar(918, if s.b[918] { 1.0 } else { 0.0 });
            if (s.b[737] && s.b[918]) {
                s.copy_ad(611, 167);
                s.store_scalar(167, s.v[57]);
            }
            if (s.b[737] && (!s.b[918])) {
                s.store_add_scaled_inputs_product_right_ad(877, 349, 1.0, 178, (-1.0), 324, A::add(A::add_scaled_inputs4(s.ad_value(357), 1.0, s.ad_value(361), 1.0, s.ad_value(355), 1.0, s.ad_value(363), 1.0), s.ad_value(393)), (-1.0));
                s.store_sub_from_scalar_scaled_mul_ad_rhs(878, 1.0, 324, A::add(s.ad_value(872), s.ad_value(365)), 1.0);
                s.store_mul_neg_lhs(879, 324, 364);
                s.store_mul_neg_lhs(880, 324, 873);
                s.store_add_scaled_product_right_ad(867, 349, 1.0, 739, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);
                s.store_mul(869, 739, 873);
                s.store_sub(881, 350, 867);
                s.store_scalar(882, (-1.0));
                s.store_scalar(883, 1.0);
                s.store_neg(884, 869);
                s.store_add_scaled_inputs3_indices(885, 351, 1.0, 350, (-1.0), 357, (-s.v[94]));
                s.store_scalar(886, (-1.0));
                s.store_sub_from_scalar_scaled_input(887, 1.0, 873, s.v[94]);
                s.store_add_scaled_inputs4(888, A::mul3(s.ad_value(878), s.ad_value(883), s.ad_value(887)), 1.0, A::mul3(s.ad_value(878), s.ad_value(884), s.ad_value(886)), (-1.0), A::mul3(s.ad_value(879), s.ad_value(882), s.ad_value(887)), -1.0, A::mul3(s.ad_value(880), s.ad_value(882), s.ad_value(886)), 1.0);
                s.store_div_from_scalar_offset_input(889, 1.0, 888, 1e-50);
                s.store_add_scaled_products_indices(890, 883, 887, 1.0, 884, 886, (-1.0));
                s.store_add_scaled_products_indices(891, 880, 886, 1.0, 879, 887, (-1.0));
                s.store_add_scaled_products_indices(892, 879, 884, 1.0, 880, 883, (-1.0));
                s.store_mul_neg_lhs(893, 882, 887);
                s.store_mul(894, 878, 887);
                s.store_add_scaled_products_indices(895, 880, 882, 1.0, 878, 884, (-1.0));
                s.store_mul(896, 882, 886);
                s.store_mul_neg_lhs(897, 878, 886);
                s.store_add_scaled_products_indices(898, 878, 883, 1.0, 879, 882, (-1.0));
                s.store_mul_add_scaled_products3_indices_rhs(874, 889, 890, 877, -1.0, 891, 881, -1.0, 892, 885, -1.0);
                s.store_mul_add_scaled_products3_indices_rhs(875, 889, 893, 877, -1.0, 894, 881, -1.0, 895, 885, -1.0);
                s.store_mul_add_scaled_products3_indices_rhs(876, 889, 896, 877, -1.0, 897, 881, -1.0, 898, 885, -1.0);
                s.store_abs(867, 874);
            }
            s.b[919] = (s.v[867] < ((s.v[875]) as f64).abs());
            s.store_scalar(919, if s.b[919] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[918])) && s.b[919]) {
                s.store_abs(867, 875);
            }
            s.b[920] = (s.v[867] < ((s.v[876]) as f64).abs());
            s.store_scalar(920, if s.b[920] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[918])) && s.b[920]) {
                s.store_abs(867, 876);
            }
            if (s.b[737] && (!s.b[918])) {
                s.store_scalar(407, 1.0);
            }
            s.b[921] = (s.v[167] > 80.0);
            s.store_scalar(921, if s.b[921] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[918])) && s.b[921]) {
                s.store_scalar(407, 125.0);
            }
            s.b[922] = (s.v[167] > 40.0);
            s.store_scalar(922, if s.b[922] { 1.0 } else { 0.0 });
            if (((s.b[737] && (!s.b[918])) && (!s.b[921])) && s.b[922]) {
                s.store_scalar(407, 125.0);
            }
            s.b[923] = (s.v[167] > 20.0);
            s.store_scalar(923, if s.b[923] { 1.0 } else { 0.0 });
            if ((((s.b[737] && (!s.b[918])) && (!s.b[921])) && (!s.b[922])) && s.b[923]) {
                s.store_scalar(407, 25.0);
            }
            s.b[924] = (s.v[167] > 10.0);
            s.store_scalar(924, if s.b[924] { 1.0 } else { 0.0 });
            if (((((s.b[737] && (!s.b[918])) && (!s.b[921])) && (!s.b[922])) && (!s.b[923])) && s.b[924]) {
                s.store_scalar(407, 5.0);
            }
            s.b[925] = (s.v[867] > (0.1 / s.v[407]));
            s.store_scalar(925, if s.b[925] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[918])) && s.b[925]) {
                s.store_mul_ad_rhs(874, 874, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(867), 1.0));
                s.store_mul_ad_rhs(875, 875, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(867), 1.0));
                s.store_mul_ad_rhs(876, 876, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(867), 1.0));
            }
            if (s.b[737] && (!s.b[918])) {
                s.store_add(349, 349, 874);
                s.store_add(350, 350, 875);
                s.store_add(351, 351, 876);
                s.store_scale(408, 407, 5e-12);
            }
            s.b[926] = (s.v[867] < s.v[408]);
            s.store_scalar(926, if s.b[926] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[918])) && s.b[926]) {
                s.store_scalar(430, 1.0);
            }
            if s.b[737] {
                s.store_offset(167, 167, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
    ) {
        if s.b[737] {
            if (s.v[611] > 0.0) {
                s.copy_ad(167, 611);
            } else {
            }
        }

        s.b[927] = (s.v[430] == 0.0);
        s.store_scalar(927, if s.b[927] { 1.0 } else { 0.0 });

        if (s.b[737] && s.b[927]) {
            s.copy_ad(349, 343);
            s.copy_ad(350, 344);
            s.copy_ad(351, 345);
        }

        if s.b[737] {
            s.copy_ad(161, 349);
            s.store_neg(244, 355);
        }

        s.b[928] = (s.v[244] <= 1e-50);
        s.store_scalar(928, if s.b[928] { 1.0 } else { 0.0 });

        if (s.b[737] && s.b[928]) {
            s.store_scalar(244, 1e-50);
        }

        if s.b[737] {
            s.store_mul(192, 244, 324);
        }

        s.b[929] = ((s.v[349] <= 0.0) && (s.v[86] != 0.0));
        s.store_scalar(929, if s.b[929] { 1.0 } else { 0.0 });

        if (s.b[737] && s.b[929]) {
            s.store_scale(327, 108, (-s.v[98]));
            s.copy_ad(362, 369);
            s.copy_ad(366, 363);
            s.store_add(359, 362, 366);
            s.store_scaled_add(437, 359, 356, (-0.5));
            s.store_mul(196, 327, 437);
            s.store_scale(477, 196, 0.5);
            s.store_scale(476, 196, (1.0 - 0.5));
            s.store_scalar(197, 0.0);
            s.store_scaled_mul(392, 357, 108, s.v[98]);
            s.store_scalar(198, 0.0);
            s.store_scalar(199, 0.0);
            s.store_scalar(192, 0.0);
            s.store_scalar(145, 1.0);
            s.copy_ad(352, 349);
            s.copy_ad(353, 350);
            s.copy_ad(354, 351);
            s.copy_ad(360, 357);
            s.copy_ad(162, 161);
            s.copy_ad(314, 162);
        }

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(453, 157);
            s.store_scalar(936, 1e-50);
            s.store_div_square_rhs(931, 545, 323);
            s.store_offset_mul_ad(933, A::div_from_scalar(2.0, s.ad_value(931)), A::sub(s.ad_value(159), s.ad_value(936)), 1.0);
            s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(931), 1.0);
        }

        s.b[937] = ((s.v[933] < s.v[332]) && (s.v[332] >= 0.0));
        s.store_scalar(937, if s.b[937] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[929])) && s.b[937]) {
            s.store_sub(44, 332, 933);
            s.store_square(49, 44);
            s.store_square(50, 332);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[938] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.store_scalar(938, if s.b[938] { 1.0 } else { 0.0 });

        s.b[939] = (4.0 == 1.0);
        s.store_scalar(939, if s.b[939] { 1.0 } else { 0.0 });

        if ((((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) && s.b[939]) {
            s.store_scalar(55, 1.0);
        }

        s.b[940] = (4.0 == 2.0);
        s.store_scalar(940, if s.b[940] { 1.0 } else { 0.0 });

        if (((((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) && (!s.b[939])) && s.b[940]) {
            s.store_scalar(55, 2.0);
        }

        s.b[941] = (4.0 == 4.0);
        s.store_scalar(941, if s.b[941] { 1.0 } else { 0.0 });

        if ((((((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) && (!s.b[939])) && (!s.b[940])) && s.b[941]) {
            s.store_scalar(55, 3.0);
        }

        s.b[942] = (4.0 == 8.0);
        s.store_scalar(942, if s.b[942] { 1.0 } else { 0.0 });

        if (((((((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) && (!s.b[939])) && (!s.b[940])) && (!s.b[941])) && s.b[942]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign11860_loop_guard: usize = 0;
        while {
            let assign11860_cond_e13445: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign11860_cond_e13445 != 0.0
        } {
            assign11860_loop_guard += 1;
            assert!(assign11860_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && (!s.b[929])) && s.b[937]) && s.b[938]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[737] && (!s.b[929])) && s.b[937]) && (!s.b[938])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((s.b[737] && (!s.b[929])) && s.b[937]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 332, 53);
            s.store_sub(933, 332, 43);
        }

        if ((s.b[737] && (!s.b[929])) && (!s.b[937])) {
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_sqrt(932, 933);
            s.store_add_mul_sub_from_scalar_rhs_indices(936, 159, 931, 1.0, 932);
            s.store_sqrt_square_offset(44, 936, ((4.0 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs_indices(936, 936, 0.5, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[943] = (s.v[936] < 0.0);
        s.store_scalar(943, if s.b[943] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[929])) && s.b[943]) {
            s.store_scalar(936, 0.0);
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_div(930, 157, 936);
            s.store_pow_offset_rhs(931, 930, 138, (-1.0));
            s.store_mul(935, 931, 930);
            s.store_offset(932, 935, 1.0);
            s.store_pow_ad(933, s.ad_value(932), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));
            s.store_mul(934, 933, 932);
            s.store_div(452, 157, 934);
            s.copy_ad(157, 452);
        }

        s.b[944] = (s.v[157] < 0.0);
        s.store_scalar(944, if s.b[944] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[929])) && s.b[944]) {
            s.copy_ad(162, 161);
            s.store_sub(164, 162, 161);
            s.copy_ad(352, 162);
            s.copy_ad(353, 350);
            s.copy_ad(354, 351);
            s.store_scalar(430, 1.0);
        }

        s.b[945] = (s.v[144] >= 1.0);
        s.store_scalar(945, if s.b[945] { 1.0 } else { 0.0 });

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && s.b[945]) {
            s.store_scalar(352, s.v[622]);
            s.store_scalar(353, s.v[623]);
            s.store_scalar(354, s.v[624]);
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            if ((s.v[163] - s.v[349]) >= 0.0) {
                s.store_sub(166, 163, 349);
            } else {
                s.store_scalar(166, 0.0);
            }
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            s.store_offset_sub_scaled_inputs_indices(44, 166, (1.0 + 0.3), 157, 1.0, (-0.03));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }

        s.b[946] = (s.v[165] < 0.0);
        s.store_scalar(946, if s.b[946] { 1.0 } else { 0.0 });

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[946]) {
            s.store_scalar(165, 0.0);
        }

        s.b[947] = (s.v[165] > s.v[157]);
        s.store_scalar(947, if s.b[947] { 1.0 } else { 0.0 });

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[946])) && s.b[947]) {
            s.copy_ad(165, 157);
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            s.copy_ad(164, 165);
            s.store_add(162, 349, 164);
            s.copy_ad(352, 162);
            s.copy_ad(388, 390);
            s.store_scaled_square(948, 474, (s.v[95] * s.v[95]));
        }

        s.b[954] = (s.v[352] < s.v[385]);
        s.store_scalar(954, if s.b[954] { 1.0 } else { 0.0 });

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) {
            s.store_neg(949, 475);
            s.store_add_scaled_inputs3_mixed_aai(950, A::square(A::add_scaled_product(s.ad_value(949), 2.0, s.ad_value(948), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(949)), (-4.0), 948, (-4.0));
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) {
            if (s.v[950] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(950, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) {
            s.store_sqrt(950, 950);
            s.store_add_scaled_product_indices(951, 949, 2.0, 948, 225, 1.0);
            s.store_scaled_sub(952, 951, 950, 0.5);
            s.store_div_ad(953, A::ln(A::div_scaled_product_by_product(s.ad_value(949), s.ad_value(949), 1.0, s.ad_value(948), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(949))));
        }

        s.b[955] = (s.v[952] < s.v[382]);
        s.store_scalar(955, if s.b[955] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
    ) {
        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) && s.b[955]) {
            s.copy_ad(354, 952);
        }

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) && (!s.b[955])) {
            s.store_offset_sub(44, 953, 952, (-0.0008));
            s.store_scale(45, 953, (4.0 * 0.0008));
        }

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) && (!s.b[955])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[954]) && (!s.b[955])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(354, 953, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) {
            s.store_neg_ad(949, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(352), (-1.0), s.ad_value(341), s.ad_value(740), (-(1.0 / (2.0) * 9662367879.197212))));
            s.store_add_scaled_inputs3_mixed_aai(950, A::square(A::add_scaled_product(s.ad_value(949), 2.0, s.ad_value(948), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(949)), (-4.0), 948, (-4.0));
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) {
            if (s.v[950] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(950, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) {
            s.store_sqrt(950, 950);
            s.store_add_scaled_product_indices(951, 949, 2.0, 948, 225, 1.0);
            s.store_scaled_sub(952, 951, 950, 0.5);
            s.store_div_ad(953, A::ln(A::div_scaled_product_by_product(s.ad_value(949), s.ad_value(949), 1.0, s.ad_value(948), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(949))));
        }

        s.b[956] = (s.v[952] < s.v[382]);
        s.store_scalar(956, if s.b[956] { 1.0 } else { 0.0 });

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) && s.b[956]) {
            s.copy_ad(354, 952);
        }

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) && (!s.b[956])) {
            s.store_offset_sub(44, 953, 952, (-0.0008));
            s.store_scale(45, 953, (4.0 * 0.0008));
        }

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) && (!s.b[956])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[954])) && (!s.b[956])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(354, 953, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if (((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) {
            s.store_div_scaled_inputs_indices(957, 352, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);
        }

        s.b[965] = (s.v[957] > 0.0);
        s.store_scalar(965, if s.b[965] { 1.0 } else { 0.0 });

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[965]) {
            s.store_sqrt_div_scaled_inputs(401, 352, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[965])) {
            s.store_scalar(401, 0.0);
        }

        s.b[966] = ((s.v[352] < s.v[385]) && (0.0 != 0.0));
        s.store_scalar(966, if s.b[966] { 1.0 } else { 0.0 });

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
            s.store_scalar(168, 0.0);
        }

        let mut assign12720_loop_guard: usize = 0;
        while {
            let assign12720_cond_e14793: f64 = if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            assign12720_cond_e14793 != 0.0
        } {
            assign12720_loop_guard += 1;
            assert!(assign12720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
                s.copy_ad(958, 474);
                s.store_mul(959, 225, 354);
                s.store_exp_neg_input(960, 959);
            }
            s.b[967] = (s.v[354] > 1e-9);
            s.store_scalar(967, if s.b[967] { 1.0 } else { 0.0 });
            if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) && s.b[967]) {
                s.store_exp_mul(957, 225, 354);
                s.store_mul_scaled_sqrt_ad_rhs(961, 958, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(960), s.ad_value(959)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(957), (-1.0), 1.0));
                s.store_mul_div_from_scalar_lhs_ad_mixed_ia(962, s.v[122], 961, A::add_scaled_sub_value_product(1.0, s.ad_value(960), 1.0, s.ad_value(239), s.ad_value(957), 1.0));
            }
            s.b[968] = (s.v[354] < (-1e-9));
            s.store_scalar(968, if s.b[968] { 1.0 } else { 0.0 });
            if ((((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) && (!s.b[967])) && s.b[968]) {
                s.store_mul_sqrt_ad_rhs(961, 958, A::offset(A::add(s.ad_value(960), s.ad_value(959)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(962, A::div_from_scalar(s.v[122], s.ad_value(961)), 1.0, 960);
            }
            if ((((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) && (!s.b[967])) && (!s.b[968])) {
                s.store_mul_ad_affine_product_lhs(961, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 354);
                s.store_scaled_sqrt_scaled_input(962, 225, s.v[122], -1.0);
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
                s.store_sqrt_add_scaled_square_product(45, 961, 1.0, 743, 743, 4.0);
                s.store_offset_scaled_div(964, 961, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(963, 961, 0.5, 45, 0.5, 743, 1e-10);
            }
            s.b[969] = (s.v[963] < 0.0);
            s.store_scalar(969, if s.b[969] { 1.0 } else { 0.0 });
            if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) && s.b[969]) {
                s.store_scalar(963, 0.0);
                s.store_scalar(964, 0.0);
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
                s.store_add_scaled_inputs3_indices(44, 341, -1.0, 963, (-1.0), 744, -1.0);
                s.store_scaled_mul(45, 341, 744, (-4.0));
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
                s.store_sqrt_square_add(45, 44, 45);
                s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(963, 341, -1.0, 44, (-0.5), 45, (-0.5));
                s.store_mul3_lhs(964, 964, 962, 335);
                s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(963)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);
                s.store_div_scaled_product_indices(389, 388, 964, 2.0, 963, 1.0);
                s.store_sub_ad_rhs(963, 354, A::div_scaled_inputs4(s.ad_value(961), 1.0 / (s.v[93]), s.ad_value(354), (-1.0), s.ad_value(475), -1.0, s.ad_value(388), 1.0, A::add(A::scale_offset(s.ad_value(962), 1.0 / (s.v[93]), (-1.0)), s.ad_value(389)), 1.0));
            }
            s.b[970] = ((((s.v[963] - s.v[354])) as f64).abs() < 5e-12);
            s.store_scalar(970, if s.b[970] { 1.0 } else { 0.0 });
            if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) && s.b[970]) {
                s.store_scalar(168, s.v[58]);
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
                s.copy_ad(354, 963);
                s.copy_ad(360, 961);
                s.store_offset(168, 168, 1.0);
            }
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[966]) {
            s.store_add(354, 475, 354);
            s.store_sub_scaled_inputs(353, 354, 1.0, 360, 1.0 / (s.v[93]));
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
            s.store_scalar(168, 0.0);
        }

        let mut assign12770_loop_guard: usize = 0;
        while {
            let assign12770_cond_e15520: f64 = if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            assign12770_cond_e15520 != 0.0
        } {
            assign12770_loop_guard += 1;
            assert!(assign12770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
                s.copy_ad(958, 474);
                s.store_mul(959, 225, 354);
                s.store_exp_neg_input(960, 959);
            }
            s.b[971] = (s.v[354] > 1e-9);
            s.store_scalar(971, if s.b[971] { 1.0 } else { 0.0 });
            if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) && s.b[971]) {
                s.store_exp_mul(957, 225, 354);
                s.store_mul_scaled_sqrt_ad_rhs(961, 958, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(960), s.ad_value(959)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(957), (-1.0), 1.0));
                s.store_mul_div_from_scalar_lhs_ad_mixed_ia(962, s.v[122], 961, A::add_scaled_sub_value_product(1.0, s.ad_value(960), 1.0, s.ad_value(239), s.ad_value(957), 1.0));
            }
            s.b[972] = (s.v[354] < (-1e-9));
            s.store_scalar(972, if s.b[972] { 1.0 } else { 0.0 });
            if ((((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) && (!s.b[971])) && s.b[972]) {
                s.store_mul_sqrt_ad_rhs(961, 958, A::offset(A::add(s.ad_value(960), s.ad_value(959)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(962, A::div_from_scalar(s.v[122], s.ad_value(961)), 1.0, 960);
            }
            if ((((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) && (!s.b[971])) && (!s.b[972])) {
                s.store_mul_ad_affine_product_lhs(961, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 354);
                s.store_scaled_sqrt_scaled_input(962, 225, s.v[122], -1.0);
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
                s.store_sqrt_add_scaled_square_product(45, 961, 1.0, 743, 743, 4.0);
                s.store_offset_scaled_div(964, 961, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(963, 961, 0.5, 45, 0.5, 743, 1e-10);
            }
            s.b[973] = (s.v[963] < 0.0);
            s.store_scalar(973, if s.b[973] { 1.0 } else { 0.0 });
            if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) && s.b[973]) {
                s.store_scalar(963, 0.0);
                s.store_scalar(964, 0.0);
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
                s.store_add_scaled_inputs3_indices(44, 341, -1.0, 963, (-1.0), 744, -1.0);
                s.store_scaled_mul(45, 341, 744, (-4.0));
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
                s.store_sqrt_square_add(45, 44, 45);
                s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(963, 341, -1.0, 44, (-0.5), 45, (-0.5));
                s.store_mul3_lhs(964, 964, 962, 335);
                s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(963)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);
                s.store_div_scaled_product_indices(389, 388, 964, 2.0, 963, 1.0);
                s.store_sub_ad_rhs(963, 354, A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(352), 1.0, s.ad_value(354), (-1.0), s.ad_value(961), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(961), 1.0, s.ad_value(341), 0.5), s.ad_value(740), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(962), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(962), s.ad_value(740), 9662367879.197212), s.ad_value(389)), 1.0));
            }
            s.b[974] = ((((s.v[963] - s.v[354])) as f64).abs() < 5e-12);
            s.store_scalar(974, if s.b[974] { 1.0 } else { 0.0 });
            if (((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) && s.b[974]) {
                s.store_scalar(168, s.v[58]);
            }
            if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
                s.copy_ad(354, 963);
                s.copy_ad(360, 961);
                s.store_offset(168, 168, 1.0);
            }
        }

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && (!s.b[966])) {
            s.store_add(354, 475, 354);
            s.store_sub_scaled_inputs(353, 354, 1.0, 360, 1.0 / (s.v[93]));
        }

        s.b[975] = (s.v[353] < 0.0);
        s.store_scalar(975, if s.b[975] { 1.0 } else { 0.0 });

        if ((((s.b[737] && (!s.b[929])) && (!s.b[944])) && (!s.b[945])) && s.b[975]) {
            s.store_scalar(353, 0.0);
        }

        s.b[1011] = (s.v[349] < 0.0);
        s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[929])) && s.b[1011]) {
            s.copy_ad(352, 349);
        }

        s.b[1012] = (s.v[353] < 0.01);
        s.store_scalar(1012, if s.b[1012] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[929])) && s.b[1012]) {
            s.store_add_scaled_product_right_ad(353, 352, 1.0, 739, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);
        }

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(346, 352);
            s.copy_ad(347, 353);
            s.copy_ad(348, 354);
            s.store_scalar(430, 0.0);
            s.store_scalar(611, 0.0);
            s.store_scalar(168, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
    ) {
        let mut assign12920_loop_guard: usize = 0;
        while {
            let assign12920_cond_e16346: f64 = if ((s.b[737] && (!s.b[929])) && (s.v[168] <= s.v[58])) { 1.0 } else { 0.0 };
            assign12920_cond_e16346 != 0.0
        } {
            assign12920_loop_guard += 1;
            assert!(assign12920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[737] && (!s.b[929])) {
                s.store_sub(977, 354, 475);
                s.store_mul(976, 225, 977);
                s.store_exp_neg_input(327, 976);
            }
            s.b[1013] = (s.v[977] < (-1e-9));
            s.store_scalar(1013, if s.b[1013] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[929])) && s.b[1013]) {
                s.store_mul_sqrt_ad_rhs(360, 474, A::offset(A::add(s.ad_value(327), s.ad_value(976)), (-1.0)));
                s.store_div_scaled_offset_numerator(983, s.ad_value(327), (-s.v[122]), s.v[122], s.ad_value(360), 1.0);
            }
            s.b[1014] = (s.v[977] > 1e-9);
            s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });
            if (((s.b[737] && (!s.b[929])) && (!s.b[1013])) && s.b[1014]) {
                s.store_exp(978, 976);
                s.store_mul_scaled_sqrt_ad_rhs(360, 474, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(327), s.ad_value(976)), (-1.0)), 1.0, s.ad_value(239), A::add(s.ad_value(978), s.ad_value(976)), (-1.0), 1.0));
                s.store_div_ad_lhs(983, A::add_scaled_sub_value_product(1.0, s.ad_value(327), s.v[122], s.ad_value(239), A::offset(s.ad_value(978), 1.0), s.v[122]), 360);
            }
            if (((s.b[737] && (!s.b[929])) && (!s.b[1013])) && (!s.b[1014])) {
                s.store_mul_neg_lhs(360, 474, 976);
                s.store_mul_neg_lhs(983, 474, 225);
            }
            if (s.b[737] && (!s.b[929])) {
                s.copy_ad(362, 369);
                s.store_exp_ad(981, A::mul(s.ad_value(225), A::sub(s.ad_value(352), s.ad_value(157))));
                s.store_scalar(979, 1.0);
                s.store_sqrt_ad(980, A::add_scaled_product(A::div_scaled_product(s.ad_value(362), s.ad_value(362), 1.0, A::square(s.ad_value(238)), 1.0), 1.0, s.ad_value(379), A::add_scaled_inputs3(s.ad_value(981), 1.0, s.ad_value(976), 1.0, s.ad_value(979), -1.0), 2.0));
                s.store_div_scaled_product3_mixed_iiai(1010, 225, 379, A::offset(s.ad_value(981), 1.0), 2.0, 980, 2.0);
                s.store_add_scaled_product_indices(358, 362, (-1.0), 238, 980, -1.0);
                s.store_mul_neg_lhs(982, 238, 1010);
                s.store_div_scaled_inputs2_indices(977, 353, 1.0, 352, (-1.0), 742, 1.0);
                s.store_mul(976, 225, 977);
            }
            s.b[1015] = ((-s.v[976]) >= 500.0);
            s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[929])) && s.b[1015]) {
                s.store_scaled_offset_ad(327, A::sub_from_scalar(1.0, s.ad_value(976)), (-500.0), 1.403592217853e217);
                s.store_scalar(333, 1.403592217853e217);
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1015])) {
                s.store_neg(44, 976);
                s.store_scalar(327, 1.0);
            }
            let mut assign12920_body26_loop_guard: usize = 0;
            while {
                let assign12920_body26_cond_e16682: f64 = if (((s.b[737] && (!s.b[929])) && (!s.b[1015])) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                assign12920_body26_cond_e16682 != 0.0
            } {
                assign12920_body26_loop_guard += 1;
                assert!(assign12920_body26_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[737] && (!s.b[929])) && (!s.b[1015])) {
                    s.store_scale(327, 327, 1.14200738981568e26);
                    s.store_offset(44, 44, (-60.0));
                }
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1015])) {
                s.store_mul_exp_rhs(327, 327, 44);
                s.copy_ad(333, 327);
            }
            if (s.b[737] && (!s.b[929])) {
                s.store_sqrt_offset_ad(978, A::add(s.ad_value(327), s.ad_value(976)), (-1.0));
            }
            s.b[1016] = (s.v[977] < (-1e-9));
            s.store_scalar(1016, if s.b[1016] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[929])) && s.b[1016]) {
                s.store_mul(366, 238, 978);
                s.store_div_scaled_product3_by_product(367, s.ad_value(238), s.ad_value(225), A::sub_from_scalar(1.0, s.ad_value(333)), 1.0, s.ad_value(978), s.ad_value(742), 2.0);
                s.store_neg(368, 367);
            }
            s.b[1017] = (s.v[977] > 1e-9);
            s.store_scalar(1017, if s.b[1017] { 1.0 } else { 0.0 });
            if (((s.b[737] && (!s.b[929])) && (!s.b[1016])) && s.b[1017]) {
                s.store_mul_neg_lhs(366, 238, 978);
                s.store_div_scaled_product3_by_product(367, s.ad_value(238), s.ad_value(225), A::sub_from_scalar(1.0, s.ad_value(333)), -1.0, s.ad_value(978), s.ad_value(742), 2.0);
                s.store_neg(368, 367);
            }
            if (((s.b[737] && (!s.b[929])) && (!s.b[1016])) && (!s.b[1017])) {
                s.store_scaled_mul(366, 238, 976, (-0.7071067811865476));
                s.store_scaled_mul(367, 238, 225, (-0.7071067811865476));
                s.store_neg(368, 367);
            }
            s.b[1018] = ((s.v[366] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0));
            s.store_scalar(1018, if s.b[1018] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[929])) && s.b[1018]) {
                s.store_add_scaled_inputs(44, 366, 1.0, 406, -1.0);
                s.store_square(49, 44);
                s.store_scaled_mul(50, 406, 406, 1.0);
                s.store_scalar(51, 1.0);
                s.store_scalar(52, 1.0);
                s.store_scalar(54, 0.0);
                s.store_scalar(55, 0.0);
                s.store_scalar(48, 0.0);
                s.store_scalar(53, 0.0);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_add(48, 51, 52);
                s.copy_ad(53, 48);
            }
            s.b[1019] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1019, if s.b[1019] { 1.0 } else { 0.0 });
            s.b[1020] = (2.0 == 1.0);
            s.store_scalar(1020, if s.b[1020] { 1.0 } else { 0.0 });
            if ((((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) && s.b[1020]) {
                s.store_scalar(55, 1.0);
            }
            s.b[1021] = (2.0 == 2.0);
            s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });
            if (((((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) && (!s.b[1020])) && s.b[1021]) {
                s.store_scalar(55, 2.0);
            }
            s.b[1022] = (2.0 == 4.0);
            s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });
            if ((((((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) && (!s.b[1020])) && (!s.b[1021])) && s.b[1022]) {
                s.store_scalar(55, 3.0);
            }
            s.b[1023] = (2.0 == 8.0);
            s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });
            if (((((((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) && (!s.b[1020])) && (!s.b[1021])) && (!s.b[1022])) && s.b[1023]) {
                s.store_scalar(55, 4.0);
            }
            if (((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) {
                s.store_scalar(54, 0.0);
            }
            let mut assign12920_body67_loop_guard: usize = 0;
            while {
                let assign12920_body67_cond_e17192: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign12920_body67_cond_e17192 != 0.0
            } {
                assign12920_body67_loop_guard += 1;
                assert!(assign12920_body67_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[737] && (!s.b[929])) && s.b[1018]) && s.b[1019]) {
                    s.store_sqrt(53, 53);
                    s.store_offset(54, 54, 1.0);
                }
            }
            if (((s.b[737] && (!s.b[929])) && s.b[1018]) && (!s.b[1019])) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if ((s.b[737] && (!s.b[929])) && s.b[1018]) {
                s.store_div_from_scalar(53, 1.0, 53);
                s.store_mul3_affine_lhs(1009, 44, 406, -1.0, 0.0, 53);
                s.store_div_scaled_product3_indices(327, 406, 52, 53, -1.0, 48, 1.0);
                s.store_add_scaled_inputs_ad_lhs(366, A::neg(s.ad_value(406)), -1.0, 1009, 1.0);
            }
            if ((s.b[737] && (!s.b[929])) && s.b[1018]) {
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1018])) {
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1018])) {
                s.store_scalar(327, 1.0);
            }
            if (s.b[737] && (!s.b[929])) {
                s.store_mul(367, 367, 327);
                s.store_mul(368, 368, 327);
            }
            s.b[1024] = ((s.v[366] < ((s.v[341] - s.v[362]) + (-(s.v[341] - s.v[362])))) && ((-(s.v[341] - s.v[362])) >= 0.0));
            s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[929])) && s.b[1024]) {
                s.store_sub_add_scaled_inputs4_lhs_indices(44, 341, 1.0, 362, (-1.0), 341, -1.0, 362, 1.0, 366);
                s.store_square(49, 44);
                s.store_scaled_mul_ad(50, A::sub(s.ad_value(341), s.ad_value(362)), A::sub(s.ad_value(341), s.ad_value(362)), 1.0);
                s.store_scalar(51, 1.0);
                s.store_scalar(52, 1.0);
                s.store_scalar(54, 0.0);
                s.store_scalar(55, 0.0);
                s.store_scalar(48, 0.0);
                s.store_scalar(53, 0.0);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_mul(51, 51, 49);
                s.store_mul(52, 52, 50);
                s.store_add(48, 51, 52);
                s.copy_ad(53, 48);
            }
            s.b[1025] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });
            s.b[1026] = (2.0 == 1.0);
            s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });
            if ((((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) && s.b[1026]) {
                s.store_scalar(55, 1.0);
            }
            s.b[1027] = (2.0 == 2.0);
            s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });
            if (((((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) && (!s.b[1026])) && s.b[1027]) {
                s.store_scalar(55, 2.0);
            }
            s.b[1028] = (2.0 == 4.0);
            s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });
            if ((((((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) && (!s.b[1026])) && (!s.b[1027])) && s.b[1028]) {
                s.store_scalar(55, 3.0);
            }
            s.b[1029] = (2.0 == 8.0);
            s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });
            if (((((((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) && (!s.b[1026])) && (!s.b[1027])) && (!s.b[1028])) && s.b[1029]) {
                s.store_scalar(55, 4.0);
            }
            if (((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) {
                s.store_scalar(54, 0.0);
            }
            let mut assign12920_body104_loop_guard: usize = 0;
            while {
                let assign12920_body104_cond_e17654: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                assign12920_body104_cond_e17654 != 0.0
            } {
                assign12920_body104_loop_guard += 1;
                assert!(assign12920_body104_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[737] && (!s.b[929])) && s.b[1024]) && s.b[1025]) {
                    s.store_sqrt(53, 53);
                    s.store_offset(54, 54, 1.0);
                }
            }
            if (((s.b[737] && (!s.b[929])) && s.b[1024]) && (!s.b[1025])) {
                s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
            }
            if ((s.b[737] && (!s.b[929])) && s.b[1024]) {
                s.store_div_from_scalar(53, 1.0, 53);
                s.store_mul_ad_affine_product_lhs(1009, s.ad_value(44), A::sub(s.ad_value(341), s.ad_value(362)), -1.0, 0.0, 53);
                s.store_div_scaled_product3_mixed_aiii(327, A::sub(s.ad_value(341), s.ad_value(362)), 52, 53, -1.0, 48, 1.0);
                s.store_sub_add_scaled_inputs4_lhs_indices(366, 341, 1.0, 362, (-1.0), 341, -1.0, 362, 1.0, 1009);
            }
            if ((s.b[737] && (!s.b[929])) && s.b[1024]) {
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1024])) {
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1024])) {
                s.store_scalar(327, 1.0);
            }
            if (s.b[737] && (!s.b[929])) {
                s.store_mul(368, 368, 327);
                s.store_mul(367, 367, 327);
                s.store_add(359, 362, 366);
            }
            s.b[1030] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));
            s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });
            if ((s.b[737] && (!s.b[929])) && s.b[1030]) {
                s.copy_ad(611, 168);
                s.store_scalar(168, s.v[58]);
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1030])) {
                s.store_add_scaled_inputs_product_right_ad(987, 352, 1.0, 178, (-1.0), 324, A::add(A::add_scaled_inputs4(s.ad_value(360), 1.0, s.ad_value(362), 1.0, s.ad_value(358), 1.0, s.ad_value(366), 1.0), s.ad_value(393)), (-1.0));
                s.store_sub_from_scalar_scaled_mul_ad_rhs(988, 1.0, 324, A::add(s.ad_value(982), s.ad_value(368)), 1.0);
                s.store_mul_neg_lhs(989, 324, 367);
                s.store_mul_neg_lhs(990, 324, 983);
                s.store_add_scaled_product_right_ad(977, 352, 1.0, 739, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(360), 1.0), 1.0);
                s.store_mul(979, 739, 983);
                s.store_sub(991, 353, 977);
                s.store_scalar(992, (-1.0));
                s.store_scalar(993, 1.0);
                s.store_neg(994, 979);
                s.store_add_scaled_inputs3_indices(995, 354, 1.0, 353, (-1.0), 360, (-s.v[94]));
                s.store_scalar(996, (-1.0));
                s.store_sub_from_scalar_scaled_input(997, 1.0, 983, s.v[94]);
                s.store_add_scaled_inputs4(998, A::mul3(s.ad_value(988), s.ad_value(993), s.ad_value(997)), 1.0, A::mul3(s.ad_value(988), s.ad_value(994), s.ad_value(996)), (-1.0), A::mul3(s.ad_value(989), s.ad_value(992), s.ad_value(997)), -1.0, A::mul3(s.ad_value(990), s.ad_value(992), s.ad_value(996)), 1.0);
                s.store_div_from_scalar_offset_input(999, 1.0, 998, 1e-50);
                s.store_add_scaled_products_indices(1000, 993, 997, 1.0, 994, 996, (-1.0));
                s.store_add_scaled_products_indices(1001, 990, 996, 1.0, 989, 997, (-1.0));
                s.store_add_scaled_products_indices(1002, 989, 994, 1.0, 990, 993, (-1.0));
                s.store_mul_neg_lhs(1003, 992, 997);
                s.store_mul(1004, 988, 997);
                s.store_add_scaled_products_indices(1005, 990, 992, 1.0, 988, 994, (-1.0));
                s.store_mul(1006, 992, 996);
                s.store_mul_neg_lhs(1007, 988, 996);
                s.store_add_scaled_products_indices(1008, 988, 993, 1.0, 989, 992, (-1.0));
                s.store_mul_add_scaled_products3_indices_rhs(984, 999, 1000, 987, -1.0, 1001, 991, -1.0, 1002, 995, -1.0);
                s.store_mul_add_scaled_products3_indices_rhs(985, 999, 1003, 987, -1.0, 1004, 991, -1.0, 1005, 995, -1.0);
                s.store_mul_add_scaled_products3_indices_rhs(986, 999, 1006, 987, -1.0, 1007, 991, -1.0, 1008, 995, -1.0);
                s.store_abs(977, 984);
            }
            s.b[1031] = (s.v[977] < ((s.v[985]) as f64).abs());
            s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });
            if (((s.b[737] && (!s.b[929])) && (!s.b[1030])) && s.b[1031]) {
                s.store_abs(977, 985);
            }
            s.b[1032] = (s.v[977] < ((s.v[986]) as f64).abs());
            s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });
            if (((s.b[737] && (!s.b[929])) && (!s.b[1030])) && s.b[1032]) {
                s.store_abs(977, 986);
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1030])) {
                s.store_scalar(407, 1.0);
            }
            s.b[1033] = (s.v[168] > 80.0);
            s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });
            if (((s.b[737] && (!s.b[929])) && (!s.b[1030])) && s.b[1033]) {
                s.store_scalar(407, 125.0);
            }
            s.b[1034] = (s.v[168] > 40.0);
            s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });
            if ((((s.b[737] && (!s.b[929])) && (!s.b[1030])) && (!s.b[1033])) && s.b[1034]) {
                s.store_scalar(407, 125.0);
            }
            s.b[1035] = (s.v[168] > 20.0);
            s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });
            if (((((s.b[737] && (!s.b[929])) && (!s.b[1030])) && (!s.b[1033])) && (!s.b[1034])) && s.b[1035]) {
                s.store_scalar(407, 25.0);
            }
            s.b[1036] = (s.v[168] > 10.0);
            s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });
            if ((((((s.b[737] && (!s.b[929])) && (!s.b[1030])) && (!s.b[1033])) && (!s.b[1034])) && (!s.b[1035])) && s.b[1036]) {
                s.store_scalar(407, 5.0);
            }
            s.b[1037] = (s.v[977] > (0.1 / s.v[407]));
            s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });
            if (((s.b[737] && (!s.b[929])) && (!s.b[1030])) && s.b[1037]) {
                s.store_mul_ad_rhs(984, 984, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(977), 1.0));
                s.store_mul_ad_rhs(985, 985, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(977), 1.0));
                s.store_mul_ad_rhs(986, 986, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(977), 1.0));
            }
            if ((s.b[737] && (!s.b[929])) && (!s.b[1030])) {
                s.store_add(352, 352, 984);
                s.store_add(353, 353, 985);
                s.store_add(354, 354, 986);
                s.store_scale(408, 407, 5e-12);
            }
            s.b[1038] = (s.v[977] < s.v[408]);
            s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });
            if (((s.b[737] && (!s.b[929])) && (!s.b[1030])) && s.b[1038]) {
                s.store_scalar(430, 1.0);
            }
            if (s.b[737] && (!s.b[929])) {
                s.store_offset(168, 168, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[737] && (!s.b[929])) {
            if (s.v[611] > 0.0) {
                s.copy_ad(168, 611);
            } else {
            }
        }

        s.b[1039] = (s.v[430] == 0.0);
        s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[929])) && s.b[1039]) {
            s.copy_ad(352, 346);
            s.copy_ad(353, 347);
            s.copy_ad(354, 348);
        }

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(162, 352);
            s.copy_ad(157, 453);
        }

        s.b[1040] = (s.v[349] < 0.0);
        s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[929])) && s.b[1040]) {
            s.store_scalar(145, 1.0);
        }

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(374, 349);
            s.copy_ad(375, 352);
            s.store_sub(164, 375, 374);
            s.copy_ad(373, 351);
            s.store_scale(400, 401, 9662367879.197212);
            s.store_add_scaled_inputs3_mixed_iia(246, 358, 1.0, 355, (-1.0), A::mul3_scaled_output(s.ad_value(225), A::add(s.ad_value(358), s.ad_value(355)), A::sub(s.ad_value(375), s.ad_value(374)), 0.5), -1.0);
        }

        s.b[1041] = ((s.v[246] < 0.0) || (s.v[157] == 0.0));
        s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[929])) && s.b[1041]) {
            s.store_scalar(246, 0.0);
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_scaled_add(437, 359, 356, (-0.5));
            s.store_sub(411, 352, 349);
            s.store_offset(411, 411, 5e-12);
            s.store_div_from_scalar_offset_scaled_input(410, s.v[93], 400, s.v[93], 1.0);
            s.store_div_scaled_inputs2_mixed_aai(409, A::square(s.ad_value(360)), 1.0, A::square(s.ad_value(357)), (-1.0), 410, 1.0);
        }

        s.b[1042] = (((-s.v[409]) < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));
        s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[929])) && s.b[1042]) {
            s.store_sub_scaled_inputs(44, 341, 1e-5, 409, -1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1043] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });

        s.b[1044] = (2.0 == 1.0);
        s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });

        if ((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && s.b[1044]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1045] = (2.0 == 2.0);
        s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });

        if (((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && (!s.b[1044])) && s.b[1045]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1046] = (2.0 == 4.0);
        s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });

        if ((((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && (!s.b[1044])) && (!s.b[1045])) && s.b[1046]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1047] = (2.0 == 8.0);
        s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });

        if (((((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && (!s.b[1044])) && (!s.b[1045])) && (!s.b[1046])) && s.b[1047]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign13420_loop_guard: usize = 0;
        while {
            let assign13420_cond_e19042: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13420_cond_e19042 != 0.0
        } {
            assign13420_loop_guard += 1;
            assert!(assign13420_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && (!s.b[929])) && s.b[1042]) && s.b[1043]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1042]) && (!s.b[1043])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[737] && (!s.b[929])) && s.b[1042]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);
            s.store_sub_scaled_inputs(328, 341, 1e-5, 43, 1.0);
        }

        if ((s.b[737] && (!s.b[929])) && (!s.b[1042])) {
            s.store_neg(328, 409);
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_neg(409, 328);
        }

        s.b[1048] = (((s.v[225] * s.v[373]) - 1.0) > 0.0);
        s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[929])) && s.b[1048]) {
            s.store_sqrt_offset_ad(328, A::mul(s.ad_value(225), s.ad_value(373)), (-1.0));
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_sub(414, 355, 358);
        }

        s.b[1049] = ((s.v[414] < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));
        s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[929])) && s.b[1049]) {
            s.store_sub_scaled_inputs(44, 341, 1e-5, 414, 1.0);
            s.store_square(49, 44);
            s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1050] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });

        s.b[1051] = (2.0 == 1.0);
        s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });

        if ((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && s.b[1051]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1052] = (2.0 == 2.0);
        s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });

        if (((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && (!s.b[1051])) && s.b[1052]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1053] = (2.0 == 4.0);
        s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });

        if ((((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && (!s.b[1051])) && (!s.b[1052])) && s.b[1053]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1054] = (2.0 == 8.0);
        s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });

        if (((((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && (!s.b[1051])) && (!s.b[1052])) && (!s.b[1053])) && s.b[1054]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign13780_loop_guard: usize = 0;
        while {
            let assign13780_cond_e19468: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign13780_cond_e19468 != 0.0
        } {
            assign13780_loop_guard += 1;
            assert!(assign13780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && (!s.b[929])) && s.b[1049]) && s.b[1050]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1049]) && (!s.b[1050])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[737] && (!s.b[929])) && s.b[1049]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);
            s.store_sub_scaled_inputs(414, 341, 1e-5, 43, 1.0);
        }

        if ((s.b[737] && (!s.b[929])) && (!s.b[1049])) {
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_offset_div_scaled_inputs_mixed_ia(412, 414, (-2.0), A::mul(A::mul3(s.ad_value(225), s.ad_value(323), s.ad_value(411)), s.ad_value(411)), 1.0, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(328, A::square(s.ad_value(411)), 411, 411);
            s.store_mul(415, 412, 411);
            s.store_sub_from_scalar_div_indices(413, 1.0, 415, 192);
        }

        s.b[1055] = ((s.v[413] < 1e-5) && (1e-5 >= 0.0));
        s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });

        if ((s.b[737] && (!s.b[929])) && s.b[1055]) {
            s.store_sub_from_scalar(44, 1e-5, 413);
            s.store_square(49, 44);
            s.store_scalar(50, (1e-5 * 1e-5));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1056] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });

        s.b[1057] = (2.0 == 1.0);
        s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });

        if ((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && s.b[1057]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1058] = (2.0 == 2.0);
        s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });

        if (((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && (!s.b[1057])) && s.b[1058]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1059] = (2.0 == 4.0);
        s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });

        if ((((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && (!s.b[1057])) && (!s.b[1058])) && s.b[1059]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1060] = (2.0 == 8.0);
        s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });

        if (((((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && (!s.b[1057])) && (!s.b[1058])) && (!s.b[1059])) && s.b[1060]) {
            s.store_scalar(55, 4.0);
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign14140_loop_guard: usize = 0;
        while {
            let assign14140_cond_e19897: f64 = if ((((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign14140_cond_e19897 != 0.0
        } {
            assign14140_loop_guard += 1;
            assert!(assign14140_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[737] && (!s.b[929])) && s.b[1055]) && s.b[1056]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((s.b[737] && (!s.b[929])) && s.b[1055]) && (!s.b[1056])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if ((s.b[737] && (!s.b[929])) && s.b[1055]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 1e-5);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[737] && (!s.b[929])) && s.b[1055]) {
            s.store_sub_from_scalar(413, 1e-5, 43);
        }

        if ((s.b[737] && (!s.b[929])) && (!s.b[1055])) {
        }

        if (s.b[737] && (!s.b[929])) {
            s.copy_ad(190, 413);
            s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);
        }

        if (s.b[737] && (!s.b[929])) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }

        if (s.b[737] && (!s.b[929])) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        if (!s.b[737]) {
            s.copy_ad(515, 154);
        }

        s.b[1067] = (s.v[416] < p.p237);
        s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });

        if ((!s.b[737]) && s.b[1067]) {
            s.store_scalar(339, 1.0);
        }

        if ((!s.b[737]) && (!s.b[1067])) {
            s.store_scalar(339, 2.0);
        }

        if (!s.b[737]) {
            s.store_add_scaled_inputs3_offset_indices(160, 185, (-1.0), 320, 1.0, 515, 1.0, s.v[123]);
        }

        s.b[1068] = (s.v[158] < s.v[160]);
        s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });

        if ((!s.b[737]) && s.b[1068]) {
            s.store_scalar(338, (-1.0));
            s.store_mul_scaled_ln_ad_rhs(254, 227, 2.0, A::div_from_scalar((-s.v[139]), s.ad_value(240)));
            s.store_mul_sub_rhs(336, 225, 159, 515);
            s.store_div_from_scalar_mul_ad(328, 1.0, s.ad_value(225), s.ad_value(238));
            s.store_mul(337, 328, 323);
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);
            s.store_offset(331, 336, (-2.0));
            s.store_scaled_mul(332, 337, 331, 9.0);
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
            s.store_square(259, 261);
        }

        s.b[1069] = (s.v[260] < (s.v[259] * 1e-8));
        s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });

        if (((!s.b[737]) && s.b[1068]) && s.b[1069]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(257, 261, 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, 332, 1.0, ((-7.0) * 1.414213562373095));
        }

        if (((!s.b[737]) && s.b[1068]) && (!s.b[1069])) {
            s.store_sqrt_add(258, 260, 259);
            s.store_add_offset_lhs(257, 258, ((-7.0) * 1.414213562373095), 332);
        }

        if ((!s.b[737]) && s.b[1068]) {
            s.store_powf(256, 257, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);
            s.store_div_from_scalar(328, 1.0, 256);
            s.store_mul(181, 255, 328);
            s.store_add_scaled_product_indices(313, 515, 1.0, 181, 227, 1.0);
            s.store_sub(328, 313, 515);
            s.store_div(329, 328, 254);
            s.store_sqrt_square_offset(330, 329, 1.0);
            s.store_add_div_lhs_indices(161, 328, 330, 515);
        }

        s.b[1070] = (s.v[144] >= 1.0);
        s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });

        if (((!s.b[737]) && (!s.b[1068])) && s.b[1070]) {
            s.store_scalar(349, s.v[619]);
            s.store_scalar(378, s.v[619]);
        }

        if (((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) {
            s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(159), s.ad_value(515))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);
        }

        if (((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) {
            s.store_add_product3_rhs_mixed_iia(376, 159, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);
            s.store_mul_sub_rhs(181, 225, 376, 515);
        }

        s.b[1071] = (s.v[181] < 3.0);
        s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });

        if ((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && s.b[1071]) {
            s.store_mul_sub_rhs(337, 225, 159, 515);
            s.store_div_from_scalar_scaled_mul(328, 1.0, 225, 240, (1.414213562373095 / 108.0));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(376, 515, 1.0, 336, 227, 1.0);
            s.copy_ad(378, 376);
        }

        s.b[1072] = (s.v[158] <= s.v[182]);
        s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });

        if (((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && (!s.b[1071])) && s.b[1072]) {
            s.copy_ad(378, 376);
        }

        if (((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && (!s.b[1071])) && (!s.b[1072])) {
            s.store_div_scalar_by_product(328, 1.0, s.ad_value(379), s.ad_value(434), 1.0);
            s.store_mul3_lhs(329, 328, 159, 159);
            s.store_add_div_from_scalar_rhs(330, 225, 2.0, 159);
            s.store_div_ln_lhs(377, 329, 330);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && (!s.b[1071])) && (!s.b[1072])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && (!s.b[1071])) && (!s.b[1072])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));
        }

        if (((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) {
            s.store_offset(336, 515, (5e-12 / 2.0));
        }

        s.b[1073] = (s.v[378] < s.v[336]);
        s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });

        if ((((!s.b[737]) && (!s.b[1068])) && (!s.b[1070])) && s.b[1073]) {
            s.copy_ad(378, 336);
        }

        if ((!s.b[737]) && (!s.b[1068])) {
            s.copy_ad(161, 378);
            s.copy_ad(163, 376);
        }

        s.b[1074] = ((p.p25 == 1.0) && (p.p26 == 2.0));
        s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });

        if ((!s.b[737]) && s.b[1074]) {
            s.store_scaled_voltage(393, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
        }

        if ((!s.b[737]) && (!s.b[1074])) {
            s.store_scalar(393, 0.0);
        }

        if (!s.b[737]) {
            s.store_exp_mul(486, 225, 515);
            s.store_mul(487, 379, 486);
            s.store_scalar(430, 0.0);
            s.copy_ad(349, 161);
            s.store_scale(419, 229, ((p.p237 * (p.p237 * 0.5)) * 9662367879.197212));
            s.store_sqrt_mul_scaled_lhs(327, 225, 2.0, 419);
            s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);
            s.store_div_ln_lhs(420, 328, 419);
            s.store_scalar(167, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign15070_loop_guard: usize = 0;
        while {
            let assign15070_cond_e21065: f64 = (s.v[57] + 1.0);
            let assign15070_cond_e21067: f64 = if ((!s.b[737]) && (s.v[167] <= assign15070_cond_e21065)) { 1.0 } else { 0.0 };
            assign15070_cond_e21067 != 0.0
        } {
            assign15070_loop_guard += 1;
            assert!(assign15070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!s.b[737]) {
                s.store_sub(417, 349, 515);
                s.store_mul(181, 225, 417);
                s.store_mul_sub_rhs(337, 420, 417, 419);
            }
            s.b[1075] = (s.v[337] < 80.0);
            s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1075]) {
                s.store_exp(328, 337);
                s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);
                s.store_sub(329, 328, 327);
                s.store_div_ln_offset_lhs(422, 329, 1.0, 420);
                s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);
            }
            if ((!s.b[737]) && (!s.b[1075])) {
                s.store_sub(422, 417, 419);
                s.store_scalar(423, 1.0);
            }
            if (!s.b[737]) {
                s.store_mul(421, 225, 422);
            }
            s.b[1076] = (((s.v[181]) as f64).abs() < 1e-16);
            s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1076]) {
                s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1077] = (s.v[181] < 0.0);
            s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && s.b[1076]) && s.b[1077]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1078] = (((s.v[181]) as f64).abs() < 0.005);
            s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && (!s.b[1076])) && s.b[1078]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_scaled_product_right_ad(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);
            }
            if (((!s.b[737]) && (!s.b[1076])) && (!s.b[1078])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));
                s.store_div_scaled_product_right_ad(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);
            }
            s.b[1079] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));
            s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1079]) {
                s.store_scalar(338, (-1.0));
            }
            s.b[1080] = (s.v[338] == (-1.0));
            s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1080]) {
                s.store_scalar(401, 0.0);
            }
            if ((!s.b[737]) && (!s.b[1080])) {
                s.store_mul(401, 444, 242);
            }
            s.b[1081] = (s.v[401] < (p.p237 * 1.01));
            s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1081]) {
                s.store_scalar(339, 1.0);
            }
            if ((!s.b[737]) && (!s.b[1081])) {
                s.store_scalar(339, 2.0);
            }
            if (!s.b[737]) {
                s.store_mul(370, 229, 401);
            }
            s.b[1082] = (s.v[181] < 0.0);
            s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1082]) {
                s.store_neg(490, 242);
                s.store_neg(491, 443);
            }
            s.b[1083] = (s.v[181] < 1e-7);
            s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && (!s.b[1082])) && s.b[1083]) {
                s.copy_ad(490, 242);
                s.copy_ad(491, 443);
            }
            s.b[1084] = (s.v[181] < 80.0);
            s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
            if ((((!s.b[737]) && (!s.b[1082])) && (!s.b[1083])) && s.b[1084]) {
                s.store_exp(243, 181);
                s.store_mul_sub_ad_rhs(488, 487, s.ad_value(243), A::offset(s.ad_value(181), 1.0));
                s.store_mul_ad_product_rhs_mixed_ia(489, 487, 225, A::offset(s.ad_value(243), (-1.0)));
            }
            if ((((!s.b[737]) && (!s.b[1082])) && (!s.b[1083])) && (!s.b[1084])) {
                s.store_exp_mul(485, 225, 349);
                s.store_mul_ad_rhs(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(485), 1.0, s.ad_value(486), s.ad_value(181), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(489, 379, 225, A::sub(s.ad_value(485), s.ad_value(486)));
            }
            if (((!s.b[737]) && (!s.b[1082])) && (!s.b[1083])) {
                s.store_sqrt_square_add(490, 242, 488);
                s.store_div_scaled_add_product(491, s.ad_value(489), 0.5, s.ad_value(443), s.ad_value(242), (2.0 * 0.5), s.ad_value(490), 1.0);
            }
            if (!s.b[737]) {
                s.store_add_scaled_inputs_products_indices(492, 349, 1.0, 159, (-1.0), 240, 490, 1.0, 324, 393, (-1.0));
                s.store_offset_mul(493, 240, 491, 1.0);
            }
            s.b[1085] = (s.v[430] == 1.0);
            s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
            if ((!s.b[737]) && s.b[1085]) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if ((!s.b[737]) && (!s.b[1085])) {
                s.store_div_scaled_inputs_indices(494, 492, -1.0, 493, 1.0);
            }
            if ((!s.b[737]) && (!s.b[1085])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[349]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(349))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1086] = (((s.v[494]) as f64).abs() > s.v[496]);
            s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && (!s.b[1085])) && s.b[1086]) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((!s.b[737]) && (!s.b[1085])) {
                s.store_add(349, 349, 494);
            }
            s.b[1087] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));
            s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
            if (((!s.b[737]) && (!s.b[1085])) && s.b[1087]) {
                s.store_scalar(430, 1.0);
            }
            if (!s.b[737]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if (!s.b[737]) {
            s.store_offset(167, 167, (-1.0));
            s.copy_ad(371, 370);
            s.copy_ad(356, 371);
            s.copy_ad(161, 349);
            s.store_div(568, 371, 238);
            s.store_offset_square(169, 568, (10.0 * 2.220446049250313e-16));
            s.store_scale(328, 568, 2.0);
            s.store_offset(170, 568, (10.0 * 2.220446049250313e-16));
            s.store_mul(245, 238, 170);
            s.store_div_from_scalar_add_ad(328, 1.0, s.ad_value(490), s.ad_value(170));
            s.store_mul3_lhs(244, 238, 488, 328);
            s.store_neg(355, 244);
            s.store_mul(192, 244, 324);
        }

        s.b[1088] = ((s.v[338] == (-1.0)) || (s.v[192] <= 1e-12));
        s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });

        if ((!s.b[737]) && s.b[1088]) {
            s.store_scalar(338, 4.0);
            s.store_scalar(145, 1.0);
            s.store_sub(329, 159, 161);
            s.store_mul(437, 323, 329);
            s.store_scale(327, 108, (-s.v[98]));
            s.store_mul(196, 327, 437);
            s.store_scalar(197, 0.0);
            s.store_scalar(198, 0.0);
            s.store_mul_neg_lhs(329, 534, 437);
            s.store_scale(468, 329, s.v[438]);
            s.store_sub(467, 329, 468);
            s.store_scalar(470, 0.0);
            s.store_scalar(469, 0.0);
            s.store_scalar(199, 0.0);
            s.store_scalar(192, 0.0);
            s.store_scalar(145, 1.0);
            s.copy_ad(352, 349);
            s.copy_ad(162, 161);
            s.copy_ad(314, 162);
            s.store_scalar(612, 1.0);
        }

        s.b[1089] = (s.v[612] == 0.0);
        s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });

        if ((!s.b[737]) && s.b[1089]) {
            s.copy_ad(453, 157);
            s.store_scalar(1096, 1e-50);
            s.store_div_square_rhs(1091, 545, 323);
            s.store_offset_mul_ad(1093, A::div_from_scalar(2.0, s.ad_value(1091)), A::sub(s.ad_value(159), s.ad_value(1096)), 1.0);
            s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(1091), 1.0);
        }

        s.b[1097] = ((s.v[1093] < s.v[332]) && (s.v[332] >= 0.0));
        s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });

        if (((!s.b[737]) && s.b[1089]) && s.b[1097]) {
            s.store_sub(44, 332, 1093);
            s.store_square(49, 44);
            s.store_square(50, 332);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1098] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });

        s.b[1099] = (4.0 == 1.0);
        s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });

        if (((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && s.b[1099]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1100] = (4.0 == 2.0);
        s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });

        if ((((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && (!s.b[1099])) && s.b[1100]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1101] = (4.0 == 4.0);
        s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });

        if (((((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && (!s.b[1099])) && (!s.b[1100])) && s.b[1101]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1102] = (4.0 == 8.0);
        s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });

        if ((((((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && (!s.b[1099])) && (!s.b[1100])) && (!s.b[1101])) && s.b[1102]) {
            s.store_scalar(55, 4.0);
        }

        if ((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign15790_loop_guard: usize = 0;
        while {
            let assign15790_cond_e22480: f64 = if (((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign15790_cond_e22480 != 0.0
        } {
            assign15790_loop_guard += 1;
            assert!(assign15790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[737]) && s.b[1089]) && s.b[1097]) && s.b[1098]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((!s.b[737]) && s.b[1089]) && s.b[1097]) && (!s.b[1098])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (((!s.b[737]) && s.b[1089]) && s.b[1097]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 332, 53);
            s.store_sub(1093, 332, 43);
        }

        if (((!s.b[737]) && s.b[1089]) && (!s.b[1097])) {
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.store_sqrt(1092, 1093);
        }

    }
}
