#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
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
        p: &Parameters,
        var_alpha_slot: &mut f64,
        var_alpha_dn0_slot: &mut f64,
        var_alpha_dn10_slot: &mut f64,
        var_alpha_dn11_slot: &mut f64,
        var_alpha_dn12_slot: &mut f64,
        var_alpha_dn17_slot: &mut f64,
        var_alpha_dn2_slot: &mut f64,
        var_alpha_dn6_slot: &mut f64,
        var_alpha_dn7_slot: &mut f64,
        var_alpha_rv_slot: &mut f64,
        var_betawl_slot: &mut f64,
        var_betawl_dn0_slot: &mut f64,
        var_betawl_dn10_slot: &mut f64,
        var_betawl_dn11_slot: &mut f64,
        var_betawl_dn12_slot: &mut f64,
        var_betawl_dn17_slot: &mut f64,
        var_betawl_dn2_slot: &mut f64,
        var_betawl_dn6_slot: &mut f64,
        var_betawl_dn7_slot: &mut f64,
        var_betawl_rv_slot: &mut f64,
        var_end_of_part_1_slot: &mut f64,
        var_end_of_part_1_rv_slot: &mut f64,
        var_ey_slot: &mut f64,
        var_ey_dn0_slot: &mut f64,
        var_ey_dn10_slot: &mut f64,
        var_ey_dn11_slot: &mut f64,
        var_ey_dn12_slot: &mut f64,
        var_ey_dn17_slot: &mut f64,
        var_ey_dn2_slot: &mut f64,
        var_ey_dn6_slot: &mut f64,
        var_ey_dn7_slot: &mut f64,
        var_ey_rv_slot: &mut f64,
        var_fb_slot: &mut f64,
        var_fb_dn0_slot: &mut f64,
        var_fb_dn10_slot: &mut f64,
        var_fb_dn11_slot: &mut f64,
        var_fb_dn12_slot: &mut f64,
        var_fb_dn17_slot: &mut f64,
        var_fb_dn2_slot: &mut f64,
        var_fb_dn6_slot: &mut f64,
        var_fb_dn7_slot: &mut f64,
        var_fb_rv_slot: &mut f64,
        var_flg_ign_slot: &mut f64,
        var_flg_ign_rv_slot: &mut f64,
        var_flg_noqi_slot: &mut f64,
        var_flg_noqi_rv_slot: &mut f64,
        var_flg_zone_slot: &mut f64,
        var_flg_zone_rv_slot: &mut f64,
        var_gds0_ign_slot: &mut f64,
        var_gds0_ign_dn0_slot: &mut f64,
        var_gds0_ign_dn10_slot: &mut f64,
        var_gds0_ign_dn11_slot: &mut f64,
        var_gds0_ign_dn12_slot: &mut f64,
        var_gds0_ign_dn17_slot: &mut f64,
        var_gds0_ign_dn2_slot: &mut f64,
        var_gds0_ign_dn6_slot: &mut f64,
        var_gds0_ign_dn7_slot: &mut f64,
        var_gds0_ign_rv_slot: &mut f64,
        var_ibd_slot: &mut f64,
        var_ibd_dn0_slot: &mut f64,
        var_ibd_dn10_slot: &mut f64,
        var_ibd_dn11_slot: &mut f64,
        var_ibd_dn12_slot: &mut f64,
        var_ibd_dn17_slot: &mut f64,
        var_ibd_dn2_slot: &mut f64,
        var_ibd_dn6_slot: &mut f64,
        var_ibd_dn7_slot: &mut f64,
        var_ibd_rv_slot: &mut f64,
        var_ibs_slot: &mut f64,
        var_ibs_dn0_slot: &mut f64,
        var_ibs_dn10_slot: &mut f64,
        var_ibs_dn11_slot: &mut f64,
        var_ibs_dn12_slot: &mut f64,
        var_ibs_dn17_slot: &mut f64,
        var_ibs_dn2_slot: &mut f64,
        var_ibs_dn6_slot: &mut f64,
        var_ibs_dn7_slot: &mut f64,
        var_ibs_rv_slot: &mut f64,
        var_idd_slot: &mut f64,
        var_idd_dn0_slot: &mut f64,
        var_idd_dn10_slot: &mut f64,
        var_idd_dn11_slot: &mut f64,
        var_idd_dn12_slot: &mut f64,
        var_idd_dn17_slot: &mut f64,
        var_idd_dn2_slot: &mut f64,
        var_idd_dn6_slot: &mut f64,
        var_idd_dn7_slot: &mut f64,
        var_idd_rv_slot: &mut f64,
        var_ids_slot: &mut f64,
        var_ids_dn0_slot: &mut f64,
        var_ids_dn10_slot: &mut f64,
        var_ids_dn11_slot: &mut f64,
        var_ids_dn12_slot: &mut f64,
        var_ids_dn17_slot: &mut f64,
        var_ids_dn2_slot: &mut f64,
        var_ids_dn6_slot: &mut f64,
        var_ids_dn7_slot: &mut f64,
        var_ids_rv_slot: &mut f64,
        var_idsibpc_slot: &mut f64,
        var_idsibpc_dn0_slot: &mut f64,
        var_idsibpc_dn10_slot: &mut f64,
        var_idsibpc_dn11_slot: &mut f64,
        var_idsibpc_dn12_slot: &mut f64,
        var_idsibpc_dn17_slot: &mut f64,
        var_idsibpc_dn2_slot: &mut f64,
        var_idsibpc_dn6_slot: &mut f64,
        var_idsibpc_dn7_slot: &mut f64,
        var_idsibpc_rv_slot: &mut f64,
        var_isub_slot: &mut f64,
        var_isub_dn0_slot: &mut f64,
        var_isub_dn10_slot: &mut f64,
        var_isub_dn11_slot: &mut f64,
        var_isub_dn12_slot: &mut f64,
        var_isub_dn17_slot: &mut f64,
        var_isub_dn2_slot: &mut f64,
        var_isub_dn6_slot: &mut f64,
        var_isub_dn7_slot: &mut f64,
        var_isub_rv_slot: &mut f64,
        var_mu_slot: &mut f64,
        var_mu_dn0_slot: &mut f64,
        var_mu_dn10_slot: &mut f64,
        var_mu_dn11_slot: &mut f64,
        var_mu_dn12_slot: &mut f64,
        var_mu_dn17_slot: &mut f64,
        var_mu_dn2_slot: &mut f64,
        var_mu_dn6_slot: &mut f64,
        var_mu_dn7_slot: &mut f64,
        var_mu_rv_slot: &mut f64,
        var_muun_slot: &mut f64,
        var_muun_dn0_slot: &mut f64,
        var_muun_dn10_slot: &mut f64,
        var_muun_dn11_slot: &mut f64,
        var_muun_dn12_slot: &mut f64,
        var_muun_dn17_slot: &mut f64,
        var_muun_dn2_slot: &mut f64,
        var_muun_dn6_slot: &mut f64,
        var_muun_dn7_slot: &mut f64,
        var_muun_rv_slot: &mut f64,
        var_pds_slot: &mut f64,
        var_pds_dn0_slot: &mut f64,
        var_pds_dn10_slot: &mut f64,
        var_pds_dn11_slot: &mut f64,
        var_pds_dn12_slot: &mut f64,
        var_pds_dn17_slot: &mut f64,
        var_pds_dn2_slot: &mut f64,
        var_pds_dn6_slot: &mut f64,
        var_pds_dn7_slot: &mut f64,
        var_pds_ini_slot: &mut f64,
        var_pds_ini_dn0_slot: &mut f64,
        var_pds_ini_dn10_slot: &mut f64,
        var_pds_ini_dn11_slot: &mut f64,
        var_pds_ini_dn12_slot: &mut f64,
        var_pds_ini_dn17_slot: &mut f64,
        var_pds_ini_dn2_slot: &mut f64,
        var_pds_ini_dn6_slot: &mut f64,
        var_pds_ini_dn7_slot: &mut f64,
        var_pds_ini_rv_slot: &mut f64,
        var_pds_rv_slot: &mut f64,
        var_ps0z_slot: &mut f64,
        var_ps0z_dn0_slot: &mut f64,
        var_ps0z_dn10_slot: &mut f64,
        var_ps0z_dn11_slot: &mut f64,
        var_ps0z_dn12_slot: &mut f64,
        var_ps0z_dn17_slot: &mut f64,
        var_ps0z_dn2_slot: &mut f64,
        var_ps0z_dn6_slot: &mut f64,
        var_ps0z_dn7_slot: &mut f64,
        var_ps0z_rv_slot: &mut f64,
        var_psl_slot: &mut f64,
        var_psl_dn0_slot: &mut f64,
        var_psl_dn10_slot: &mut f64,
        var_psl_dn11_slot: &mut f64,
        var_psl_dn12_slot: &mut f64,
        var_psl_dn17_slot: &mut f64,
        var_psl_dn2_slot: &mut f64,
        var_psl_dn6_slot: &mut f64,
        var_psl_dn7_slot: &mut f64,
        var_psl_lim_slot: &mut f64,
        var_psl_lim_dn0_slot: &mut f64,
        var_psl_lim_dn10_slot: &mut f64,
        var_psl_lim_dn11_slot: &mut f64,
        var_psl_lim_dn12_slot: &mut f64,
        var_psl_lim_dn17_slot: &mut f64,
        var_psl_lim_dn2_slot: &mut f64,
        var_psl_lim_dn6_slot: &mut f64,
        var_psl_lim_dn7_slot: &mut f64,
        var_psl_lim_rv_slot: &mut f64,
        var_psl_rv_slot: &mut f64,
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
        var_qb_rv_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn0_slot: &mut f64,
        var_qbd_dn10_slot: &mut f64,
        var_qbd_dn11_slot: &mut f64,
        var_qbd_dn12_slot: &mut f64,
        var_qbd_dn17_slot: &mut f64,
        var_qbd_dn2_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbd_rv_slot: &mut f64,
        var_qbdld_slot: &mut f64,
        var_qbdld_dn0_slot: &mut f64,
        var_qbdld_dn10_slot: &mut f64,
        var_qbdld_dn11_slot: &mut f64,
        var_qbdld_dn12_slot: &mut f64,
        var_qbdld_dn17_slot: &mut f64,
        var_qbdld_dn2_slot: &mut f64,
        var_qbdld_dn6_slot: &mut f64,
        var_qbdld_dn7_slot: &mut f64,
        var_qbdld_rv_slot: &mut f64,
        var_qbody_bt_n_iud_slot: &mut f64,
        var_qbody_bt_n_iud_dn0_slot: &mut f64,
        var_qbody_bt_n_iud_dn10_slot: &mut f64,
        var_qbody_bt_n_iud_dn11_slot: &mut f64,
        var_qbody_bt_n_iud_dn12_slot: &mut f64,
        var_qbody_bt_n_iud_dn17_slot: &mut f64,
        var_qbody_bt_n_iud_dn2_slot: &mut f64,
        var_qbody_bt_n_iud_dn6_slot: &mut f64,
        var_qbody_bt_n_iud_dn7_slot: &mut f64,
        var_qbody_bt_n_iud_rv_slot: &mut f64,
        var_qbody_bt_n_ius_slot: &mut f64,
        var_qbody_bt_n_ius_dn0_slot: &mut f64,
        var_qbody_bt_n_ius_dn10_slot: &mut f64,
        var_qbody_bt_n_ius_dn11_slot: &mut f64,
        var_qbody_bt_n_ius_dn12_slot: &mut f64,
        var_qbody_bt_n_ius_dn17_slot: &mut f64,
        var_qbody_bt_n_ius_dn2_slot: &mut f64,
        var_qbody_bt_n_ius_dn6_slot: &mut f64,
        var_qbody_bt_n_ius_dn7_slot: &mut f64,
        var_qbody_bt_n_ius_rv_slot: &mut f64,
        var_qbody_bt_n_sud_slot: &mut f64,
        var_qbody_bt_n_sud_dn0_slot: &mut f64,
        var_qbody_bt_n_sud_dn10_slot: &mut f64,
        var_qbody_bt_n_sud_dn11_slot: &mut f64,
        var_qbody_bt_n_sud_dn12_slot: &mut f64,
        var_qbody_bt_n_sud_dn17_slot: &mut f64,
        var_qbody_bt_n_sud_dn2_slot: &mut f64,
        var_qbody_bt_n_sud_dn6_slot: &mut f64,
        var_qbody_bt_n_sud_dn7_slot: &mut f64,
        var_qbody_bt_n_sud_rv_slot: &mut f64,
        var_qbody_bt_n_sus_slot: &mut f64,
        var_qbody_bt_n_sus_dn0_slot: &mut f64,
        var_qbody_bt_n_sus_dn10_slot: &mut f64,
        var_qbody_bt_n_sus_dn11_slot: &mut f64,
        var_qbody_bt_n_sus_dn12_slot: &mut f64,
        var_qbody_bt_n_sus_dn17_slot: &mut f64,
        var_qbody_bt_n_sus_dn2_slot: &mut f64,
        var_qbody_bt_n_sus_dn6_slot: &mut f64,
        var_qbody_bt_n_sus_dn7_slot: &mut f64,
        var_qbody_bt_n_sus_rv_slot: &mut f64,
        var_qbody_bt_p_iud_slot: &mut f64,
        var_qbody_bt_p_iud_dn0_slot: &mut f64,
        var_qbody_bt_p_iud_dn10_slot: &mut f64,
        var_qbody_bt_p_iud_dn11_slot: &mut f64,
        var_qbody_bt_p_iud_dn12_slot: &mut f64,
        var_qbody_bt_p_iud_dn17_slot: &mut f64,
        var_qbody_bt_p_iud_dn2_slot: &mut f64,
        var_qbody_bt_p_iud_dn6_slot: &mut f64,
        var_qbody_bt_p_iud_dn7_slot: &mut f64,
        var_qbody_bt_p_iud_rv_slot: &mut f64,
        var_qbody_bt_p_ius_slot: &mut f64,
        var_qbody_bt_p_ius_dn0_slot: &mut f64,
        var_qbody_bt_p_ius_dn10_slot: &mut f64,
        var_qbody_bt_p_ius_dn11_slot: &mut f64,
        var_qbody_bt_p_ius_dn12_slot: &mut f64,
        var_qbody_bt_p_ius_dn17_slot: &mut f64,
        var_qbody_bt_p_ius_dn2_slot: &mut f64,
        var_qbody_bt_p_ius_dn6_slot: &mut f64,
        var_qbody_bt_p_ius_dn7_slot: &mut f64,
        var_qbody_bt_p_ius_rv_slot: &mut f64,
        var_qbody_bt_p_sud_slot: &mut f64,
        var_qbody_bt_p_sud_dn0_slot: &mut f64,
        var_qbody_bt_p_sud_dn10_slot: &mut f64,
        var_qbody_bt_p_sud_dn11_slot: &mut f64,
        var_qbody_bt_p_sud_dn12_slot: &mut f64,
        var_qbody_bt_p_sud_dn17_slot: &mut f64,
        var_qbody_bt_p_sud_dn2_slot: &mut f64,
        var_qbody_bt_p_sud_dn6_slot: &mut f64,
        var_qbody_bt_p_sud_dn7_slot: &mut f64,
        var_qbody_bt_p_sud_rv_slot: &mut f64,
        var_qbody_bt_p_sus_slot: &mut f64,
        var_qbody_bt_p_sus_dn0_slot: &mut f64,
        var_qbody_bt_p_sus_dn10_slot: &mut f64,
        var_qbody_bt_p_sus_dn11_slot: &mut f64,
        var_qbody_bt_p_sus_dn12_slot: &mut f64,
        var_qbody_bt_p_sus_dn17_slot: &mut f64,
        var_qbody_bt_p_sus_dn2_slot: &mut f64,
        var_qbody_bt_p_sus_dn6_slot: &mut f64,
        var_qbody_bt_p_sus_dn7_slot: &mut f64,
        var_qbody_bt_p_sus_rv_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn0_slot: &mut f64,
        var_qbs_dn10_slot: &mut f64,
        var_qbs_dn11_slot: &mut f64,
        var_qbs_dn12_slot: &mut f64,
        var_qbs_dn17_slot: &mut f64,
        var_qbs_dn2_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_qbs_rv_slot: &mut f64,
        var_qbsld_slot: &mut f64,
        var_qbsld_dn0_slot: &mut f64,
        var_qbsld_dn10_slot: &mut f64,
        var_qbsld_dn11_slot: &mut f64,
        var_qbsld_dn12_slot: &mut f64,
        var_qbsld_dn17_slot: &mut f64,
        var_qbsld_dn2_slot: &mut f64,
        var_qbsld_dn6_slot: &mut f64,
        var_qbsld_dn7_slot: &mut f64,
        var_qbsld_rv_slot: &mut f64,
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
        var_qd_rv_slot: &mut f64,
        var_qgob_slot: &mut f64,
        var_qgob_dn0_slot: &mut f64,
        var_qgob_dn10_slot: &mut f64,
        var_qgob_dn11_slot: &mut f64,
        var_qgob_dn12_slot: &mut f64,
        var_qgob_dn17_slot: &mut f64,
        var_qgob_dn2_slot: &mut f64,
        var_qgob_dn6_slot: &mut f64,
        var_qgob_dn7_slot: &mut f64,
        var_qgob_rv_slot: &mut f64,
        var_qgod_slot: &mut f64,
        var_qgod_dn0_slot: &mut f64,
        var_qgod_dn10_slot: &mut f64,
        var_qgod_dn11_slot: &mut f64,
        var_qgod_dn12_slot: &mut f64,
        var_qgod_dn17_slot: &mut f64,
        var_qgod_dn2_slot: &mut f64,
        var_qgod_dn6_slot: &mut f64,
        var_qgod_dn7_slot: &mut f64,
        var_qgod_rv_slot: &mut f64,
        var_qgos_slot: &mut f64,
        var_qgos_dn0_slot: &mut f64,
        var_qgos_dn10_slot: &mut f64,
        var_qgos_dn11_slot: &mut f64,
        var_qgos_dn12_slot: &mut f64,
        var_qgos_dn17_slot: &mut f64,
        var_qgos_dn2_slot: &mut f64,
        var_qgos_dn6_slot: &mut f64,
        var_qgos_dn7_slot: &mut f64,
        var_qgos_rv_slot: &mut f64,
        var_qi_slot: &mut f64,
        var_qi_dn0_slot: &mut f64,
        var_qi_dn10_slot: &mut f64,
        var_qi_dn11_slot: &mut f64,
        var_qi_dn12_slot: &mut f64,
        var_qi_dn17_slot: &mut f64,
        var_qi_dn2_slot: &mut f64,
        var_qi_dn6_slot: &mut f64,
        var_qi_dn7_slot: &mut f64,
        var_qi_rv_slot: &mut f64,
        var_qidn_slot: &mut f64,
        var_qidn_dn0_slot: &mut f64,
        var_qidn_dn10_slot: &mut f64,
        var_qidn_dn11_slot: &mut f64,
        var_qidn_dn12_slot: &mut f64,
        var_qidn_dn17_slot: &mut f64,
        var_qidn_dn2_slot: &mut f64,
        var_qidn_dn6_slot: &mut f64,
        var_qidn_dn7_slot: &mut f64,
        var_qidn_rv_slot: &mut f64,
        var_qinm_slot: &mut f64,
        var_qinm_dn0_slot: &mut f64,
        var_qinm_dn10_slot: &mut f64,
        var_qinm_dn11_slot: &mut f64,
        var_qinm_dn12_slot: &mut f64,
        var_qinm_dn17_slot: &mut f64,
        var_qinm_dn2_slot: &mut f64,
        var_qinm_dn6_slot: &mut f64,
        var_qinm_dn7_slot: &mut f64,
        var_qinm_rv_slot: &mut f64,
        var_qn0_slot: &mut f64,
        var_qn0_dn0_slot: &mut f64,
        var_qn0_dn10_slot: &mut f64,
        var_qn0_dn11_slot: &mut f64,
        var_qn0_dn12_slot: &mut f64,
        var_qn0_dn17_slot: &mut f64,
        var_qn0_dn2_slot: &mut f64,
        var_qn0_dn6_slot: &mut f64,
        var_qn0_dn7_slot: &mut f64,
        var_qn0_rv_slot: &mut f64,
        var_qovd_slot: &mut f64,
        var_qovd_dn0_slot: &mut f64,
        var_qovd_dn10_slot: &mut f64,
        var_qovd_dn11_slot: &mut f64,
        var_qovd_dn12_slot: &mut f64,
        var_qovd_dn17_slot: &mut f64,
        var_qovd_dn2_slot: &mut f64,
        var_qovd_dn6_slot: &mut f64,
        var_qovd_dn7_slot: &mut f64,
        var_qovd_rv_slot: &mut f64,
        var_qovs_slot: &mut f64,
        var_qovs_dn0_slot: &mut f64,
        var_qovs_dn10_slot: &mut f64,
        var_qovs_dn11_slot: &mut f64,
        var_qovs_dn12_slot: &mut f64,
        var_qovs_dn17_slot: &mut f64,
        var_qovs_dn2_slot: &mut f64,
        var_qovs_dn6_slot: &mut f64,
        var_qovs_dn7_slot: &mut f64,
        var_qovs_rv_slot: &mut f64,
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
        var_qse_rv_slot: &mut f64,
        var_vgvt_slot: &mut f64,
        var_vgvt_dn0_slot: &mut f64,
        var_vgvt_dn10_slot: &mut f64,
        var_vgvt_dn11_slot: &mut f64,
        var_vgvt_dn12_slot: &mut f64,
        var_vgvt_dn17_slot: &mut f64,
        var_vgvt_dn2_slot: &mut f64,
        var_vgvt_dn6_slot: &mut f64,
        var_vgvt_dn7_slot: &mut f64,
        var_vgvt_rv_slot: &mut f64,
        var_wdsoi_0_slot: &mut f64,
        var_wdsoi_0_rv_slot: &mut f64,
        var_xd_slot: &mut f64,
        var_xd_dn0_slot: &mut f64,
        var_xd_dn10_slot: &mut f64,
        var_xd_dn11_slot: &mut f64,
        var_xd_dn12_slot: &mut f64,
        var_xd_dn17_slot: &mut f64,
        var_xd_dn2_slot: &mut f64,
        var_xd_dn6_slot: &mut f64,
        var_xd_dn7_slot: &mut f64,
        var_xd_rv_slot: &mut f64,
    ) {
        let mut var_alpha: f64 = *var_alpha_slot;
        let mut var_alpha_dn0: f64 = *var_alpha_dn0_slot;
        let mut var_alpha_dn10: f64 = *var_alpha_dn10_slot;
        let mut var_alpha_dn11: f64 = *var_alpha_dn11_slot;
        let mut var_alpha_dn12: f64 = *var_alpha_dn12_slot;
        let mut var_alpha_dn17: f64 = *var_alpha_dn17_slot;
        let mut var_alpha_dn2: f64 = *var_alpha_dn2_slot;
        let mut var_alpha_dn6: f64 = *var_alpha_dn6_slot;
        let mut var_alpha_dn7: f64 = *var_alpha_dn7_slot;
        let mut var_alpha_rv: f64 = *var_alpha_rv_slot;
        let mut var_betawl: f64 = *var_betawl_slot;
        let mut var_betawl_dn0: f64 = *var_betawl_dn0_slot;
        let mut var_betawl_dn10: f64 = *var_betawl_dn10_slot;
        let mut var_betawl_dn11: f64 = *var_betawl_dn11_slot;
        let mut var_betawl_dn12: f64 = *var_betawl_dn12_slot;
        let mut var_betawl_dn17: f64 = *var_betawl_dn17_slot;
        let mut var_betawl_dn2: f64 = *var_betawl_dn2_slot;
        let mut var_betawl_dn6: f64 = *var_betawl_dn6_slot;
        let mut var_betawl_dn7: f64 = *var_betawl_dn7_slot;
        let mut var_betawl_rv: f64 = *var_betawl_rv_slot;
        let mut var_end_of_part_1: f64 = *var_end_of_part_1_slot;
        let mut var_end_of_part_1_rv: f64 = *var_end_of_part_1_rv_slot;
        let mut var_ey: f64 = *var_ey_slot;
        let mut var_ey_dn0: f64 = *var_ey_dn0_slot;
        let mut var_ey_dn10: f64 = *var_ey_dn10_slot;
        let mut var_ey_dn11: f64 = *var_ey_dn11_slot;
        let mut var_ey_dn12: f64 = *var_ey_dn12_slot;
        let mut var_ey_dn17: f64 = *var_ey_dn17_slot;
        let mut var_ey_dn2: f64 = *var_ey_dn2_slot;
        let mut var_ey_dn6: f64 = *var_ey_dn6_slot;
        let mut var_ey_dn7: f64 = *var_ey_dn7_slot;
        let mut var_ey_rv: f64 = *var_ey_rv_slot;
        let mut var_fb: f64 = *var_fb_slot;
        let mut var_fb_dn0: f64 = *var_fb_dn0_slot;
        let mut var_fb_dn10: f64 = *var_fb_dn10_slot;
        let mut var_fb_dn11: f64 = *var_fb_dn11_slot;
        let mut var_fb_dn12: f64 = *var_fb_dn12_slot;
        let mut var_fb_dn17: f64 = *var_fb_dn17_slot;
        let mut var_fb_dn2: f64 = *var_fb_dn2_slot;
        let mut var_fb_dn6: f64 = *var_fb_dn6_slot;
        let mut var_fb_dn7: f64 = *var_fb_dn7_slot;
        let mut var_fb_rv: f64 = *var_fb_rv_slot;
        let mut var_flg_ign: f64 = *var_flg_ign_slot;
        let mut var_flg_ign_rv: f64 = *var_flg_ign_rv_slot;
        let mut var_flg_noqi: f64 = *var_flg_noqi_slot;
        let mut var_flg_noqi_rv: f64 = *var_flg_noqi_rv_slot;
        let mut var_flg_zone: f64 = *var_flg_zone_slot;
        let mut var_flg_zone_rv: f64 = *var_flg_zone_rv_slot;
        let mut var_gds0_ign: f64 = *var_gds0_ign_slot;
        let mut var_gds0_ign_dn0: f64 = *var_gds0_ign_dn0_slot;
        let mut var_gds0_ign_dn10: f64 = *var_gds0_ign_dn10_slot;
        let mut var_gds0_ign_dn11: f64 = *var_gds0_ign_dn11_slot;
        let mut var_gds0_ign_dn12: f64 = *var_gds0_ign_dn12_slot;
        let mut var_gds0_ign_dn17: f64 = *var_gds0_ign_dn17_slot;
        let mut var_gds0_ign_dn2: f64 = *var_gds0_ign_dn2_slot;
        let mut var_gds0_ign_dn6: f64 = *var_gds0_ign_dn6_slot;
        let mut var_gds0_ign_dn7: f64 = *var_gds0_ign_dn7_slot;
        let mut var_gds0_ign_rv: f64 = *var_gds0_ign_rv_slot;
        let mut var_ibd: f64 = *var_ibd_slot;
        let mut var_ibd_dn0: f64 = *var_ibd_dn0_slot;
        let mut var_ibd_dn10: f64 = *var_ibd_dn10_slot;
        let mut var_ibd_dn11: f64 = *var_ibd_dn11_slot;
        let mut var_ibd_dn12: f64 = *var_ibd_dn12_slot;
        let mut var_ibd_dn17: f64 = *var_ibd_dn17_slot;
        let mut var_ibd_dn2: f64 = *var_ibd_dn2_slot;
        let mut var_ibd_dn6: f64 = *var_ibd_dn6_slot;
        let mut var_ibd_dn7: f64 = *var_ibd_dn7_slot;
        let mut var_ibd_rv: f64 = *var_ibd_rv_slot;
        let mut var_ibs: f64 = *var_ibs_slot;
        let mut var_ibs_dn0: f64 = *var_ibs_dn0_slot;
        let mut var_ibs_dn10: f64 = *var_ibs_dn10_slot;
        let mut var_ibs_dn11: f64 = *var_ibs_dn11_slot;
        let mut var_ibs_dn12: f64 = *var_ibs_dn12_slot;
        let mut var_ibs_dn17: f64 = *var_ibs_dn17_slot;
        let mut var_ibs_dn2: f64 = *var_ibs_dn2_slot;
        let mut var_ibs_dn6: f64 = *var_ibs_dn6_slot;
        let mut var_ibs_dn7: f64 = *var_ibs_dn7_slot;
        let mut var_ibs_rv: f64 = *var_ibs_rv_slot;
        let mut var_idd: f64 = *var_idd_slot;
        let mut var_idd_dn0: f64 = *var_idd_dn0_slot;
        let mut var_idd_dn10: f64 = *var_idd_dn10_slot;
        let mut var_idd_dn11: f64 = *var_idd_dn11_slot;
        let mut var_idd_dn12: f64 = *var_idd_dn12_slot;
        let mut var_idd_dn17: f64 = *var_idd_dn17_slot;
        let mut var_idd_dn2: f64 = *var_idd_dn2_slot;
        let mut var_idd_dn6: f64 = *var_idd_dn6_slot;
        let mut var_idd_dn7: f64 = *var_idd_dn7_slot;
        let mut var_idd_rv: f64 = *var_idd_rv_slot;
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn0: f64 = *var_ids_dn0_slot;
        let mut var_ids_dn10: f64 = *var_ids_dn10_slot;
        let mut var_ids_dn11: f64 = *var_ids_dn11_slot;
        let mut var_ids_dn12: f64 = *var_ids_dn12_slot;
        let mut var_ids_dn17: f64 = *var_ids_dn17_slot;
        let mut var_ids_dn2: f64 = *var_ids_dn2_slot;
        let mut var_ids_dn6: f64 = *var_ids_dn6_slot;
        let mut var_ids_dn7: f64 = *var_ids_dn7_slot;
        let mut var_ids_rv: f64 = *var_ids_rv_slot;
        let mut var_idsibpc: f64 = *var_idsibpc_slot;
        let mut var_idsibpc_dn0: f64 = *var_idsibpc_dn0_slot;
        let mut var_idsibpc_dn10: f64 = *var_idsibpc_dn10_slot;
        let mut var_idsibpc_dn11: f64 = *var_idsibpc_dn11_slot;
        let mut var_idsibpc_dn12: f64 = *var_idsibpc_dn12_slot;
        let mut var_idsibpc_dn17: f64 = *var_idsibpc_dn17_slot;
        let mut var_idsibpc_dn2: f64 = *var_idsibpc_dn2_slot;
        let mut var_idsibpc_dn6: f64 = *var_idsibpc_dn6_slot;
        let mut var_idsibpc_dn7: f64 = *var_idsibpc_dn7_slot;
        let mut var_idsibpc_rv: f64 = *var_idsibpc_rv_slot;
        let mut var_isub: f64 = *var_isub_slot;
        let mut var_isub_dn0: f64 = *var_isub_dn0_slot;
        let mut var_isub_dn10: f64 = *var_isub_dn10_slot;
        let mut var_isub_dn11: f64 = *var_isub_dn11_slot;
        let mut var_isub_dn12: f64 = *var_isub_dn12_slot;
        let mut var_isub_dn17: f64 = *var_isub_dn17_slot;
        let mut var_isub_dn2: f64 = *var_isub_dn2_slot;
        let mut var_isub_dn6: f64 = *var_isub_dn6_slot;
        let mut var_isub_dn7: f64 = *var_isub_dn7_slot;
        let mut var_isub_rv: f64 = *var_isub_rv_slot;
        let mut var_mu: f64 = *var_mu_slot;
        let mut var_mu_dn0: f64 = *var_mu_dn0_slot;
        let mut var_mu_dn10: f64 = *var_mu_dn10_slot;
        let mut var_mu_dn11: f64 = *var_mu_dn11_slot;
        let mut var_mu_dn12: f64 = *var_mu_dn12_slot;
        let mut var_mu_dn17: f64 = *var_mu_dn17_slot;
        let mut var_mu_dn2: f64 = *var_mu_dn2_slot;
        let mut var_mu_dn6: f64 = *var_mu_dn6_slot;
        let mut var_mu_dn7: f64 = *var_mu_dn7_slot;
        let mut var_mu_rv: f64 = *var_mu_rv_slot;
        let mut var_muun: f64 = *var_muun_slot;
        let mut var_muun_dn0: f64 = *var_muun_dn0_slot;
        let mut var_muun_dn10: f64 = *var_muun_dn10_slot;
        let mut var_muun_dn11: f64 = *var_muun_dn11_slot;
        let mut var_muun_dn12: f64 = *var_muun_dn12_slot;
        let mut var_muun_dn17: f64 = *var_muun_dn17_slot;
        let mut var_muun_dn2: f64 = *var_muun_dn2_slot;
        let mut var_muun_dn6: f64 = *var_muun_dn6_slot;
        let mut var_muun_dn7: f64 = *var_muun_dn7_slot;
        let mut var_muun_rv: f64 = *var_muun_rv_slot;
        let mut var_pds: f64 = *var_pds_slot;
        let mut var_pds_dn0: f64 = *var_pds_dn0_slot;
        let mut var_pds_dn10: f64 = *var_pds_dn10_slot;
        let mut var_pds_dn11: f64 = *var_pds_dn11_slot;
        let mut var_pds_dn12: f64 = *var_pds_dn12_slot;
        let mut var_pds_dn17: f64 = *var_pds_dn17_slot;
        let mut var_pds_dn2: f64 = *var_pds_dn2_slot;
        let mut var_pds_dn6: f64 = *var_pds_dn6_slot;
        let mut var_pds_dn7: f64 = *var_pds_dn7_slot;
        let mut var_pds_ini: f64 = *var_pds_ini_slot;
        let mut var_pds_ini_dn0: f64 = *var_pds_ini_dn0_slot;
        let mut var_pds_ini_dn10: f64 = *var_pds_ini_dn10_slot;
        let mut var_pds_ini_dn11: f64 = *var_pds_ini_dn11_slot;
        let mut var_pds_ini_dn12: f64 = *var_pds_ini_dn12_slot;
        let mut var_pds_ini_dn17: f64 = *var_pds_ini_dn17_slot;
        let mut var_pds_ini_dn2: f64 = *var_pds_ini_dn2_slot;
        let mut var_pds_ini_dn6: f64 = *var_pds_ini_dn6_slot;
        let mut var_pds_ini_dn7: f64 = *var_pds_ini_dn7_slot;
        let mut var_pds_ini_rv: f64 = *var_pds_ini_rv_slot;
        let mut var_pds_rv: f64 = *var_pds_rv_slot;
        let mut var_ps0z: f64 = *var_ps0z_slot;
        let mut var_ps0z_dn0: f64 = *var_ps0z_dn0_slot;
        let mut var_ps0z_dn10: f64 = *var_ps0z_dn10_slot;
        let mut var_ps0z_dn11: f64 = *var_ps0z_dn11_slot;
        let mut var_ps0z_dn12: f64 = *var_ps0z_dn12_slot;
        let mut var_ps0z_dn17: f64 = *var_ps0z_dn17_slot;
        let mut var_ps0z_dn2: f64 = *var_ps0z_dn2_slot;
        let mut var_ps0z_dn6: f64 = *var_ps0z_dn6_slot;
        let mut var_ps0z_dn7: f64 = *var_ps0z_dn7_slot;
        let mut var_ps0z_rv: f64 = *var_ps0z_rv_slot;
        let mut var_psl: f64 = *var_psl_slot;
        let mut var_psl_dn0: f64 = *var_psl_dn0_slot;
        let mut var_psl_dn10: f64 = *var_psl_dn10_slot;
        let mut var_psl_dn11: f64 = *var_psl_dn11_slot;
        let mut var_psl_dn12: f64 = *var_psl_dn12_slot;
        let mut var_psl_dn17: f64 = *var_psl_dn17_slot;
        let mut var_psl_dn2: f64 = *var_psl_dn2_slot;
        let mut var_psl_dn6: f64 = *var_psl_dn6_slot;
        let mut var_psl_dn7: f64 = *var_psl_dn7_slot;
        let mut var_psl_lim: f64 = *var_psl_lim_slot;
        let mut var_psl_lim_dn0: f64 = *var_psl_lim_dn0_slot;
        let mut var_psl_lim_dn10: f64 = *var_psl_lim_dn10_slot;
        let mut var_psl_lim_dn11: f64 = *var_psl_lim_dn11_slot;
        let mut var_psl_lim_dn12: f64 = *var_psl_lim_dn12_slot;
        let mut var_psl_lim_dn17: f64 = *var_psl_lim_dn17_slot;
        let mut var_psl_lim_dn2: f64 = *var_psl_lim_dn2_slot;
        let mut var_psl_lim_dn6: f64 = *var_psl_lim_dn6_slot;
        let mut var_psl_lim_dn7: f64 = *var_psl_lim_dn7_slot;
        let mut var_psl_lim_rv: f64 = *var_psl_lim_rv_slot;
        let mut var_psl_rv: f64 = *var_psl_rv_slot;
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
        let mut var_qb_rv: f64 = *var_qb_rv_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn0: f64 = *var_qbd_dn0_slot;
        let mut var_qbd_dn10: f64 = *var_qbd_dn10_slot;
        let mut var_qbd_dn11: f64 = *var_qbd_dn11_slot;
        let mut var_qbd_dn12: f64 = *var_qbd_dn12_slot;
        let mut var_qbd_dn17: f64 = *var_qbd_dn17_slot;
        let mut var_qbd_dn2: f64 = *var_qbd_dn2_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbd_rv: f64 = *var_qbd_rv_slot;
        let mut var_qbdld: f64 = *var_qbdld_slot;
        let mut var_qbdld_dn0: f64 = *var_qbdld_dn0_slot;
        let mut var_qbdld_dn10: f64 = *var_qbdld_dn10_slot;
        let mut var_qbdld_dn11: f64 = *var_qbdld_dn11_slot;
        let mut var_qbdld_dn12: f64 = *var_qbdld_dn12_slot;
        let mut var_qbdld_dn17: f64 = *var_qbdld_dn17_slot;
        let mut var_qbdld_dn2: f64 = *var_qbdld_dn2_slot;
        let mut var_qbdld_dn6: f64 = *var_qbdld_dn6_slot;
        let mut var_qbdld_dn7: f64 = *var_qbdld_dn7_slot;
        let mut var_qbdld_rv: f64 = *var_qbdld_rv_slot;
        let mut var_qbody_bt_n_iud: f64 = *var_qbody_bt_n_iud_slot;
        let mut var_qbody_bt_n_iud_dn0: f64 = *var_qbody_bt_n_iud_dn0_slot;
        let mut var_qbody_bt_n_iud_dn10: f64 = *var_qbody_bt_n_iud_dn10_slot;
        let mut var_qbody_bt_n_iud_dn11: f64 = *var_qbody_bt_n_iud_dn11_slot;
        let mut var_qbody_bt_n_iud_dn12: f64 = *var_qbody_bt_n_iud_dn12_slot;
        let mut var_qbody_bt_n_iud_dn17: f64 = *var_qbody_bt_n_iud_dn17_slot;
        let mut var_qbody_bt_n_iud_dn2: f64 = *var_qbody_bt_n_iud_dn2_slot;
        let mut var_qbody_bt_n_iud_dn6: f64 = *var_qbody_bt_n_iud_dn6_slot;
        let mut var_qbody_bt_n_iud_dn7: f64 = *var_qbody_bt_n_iud_dn7_slot;
        let mut var_qbody_bt_n_iud_rv: f64 = *var_qbody_bt_n_iud_rv_slot;
        let mut var_qbody_bt_n_ius: f64 = *var_qbody_bt_n_ius_slot;
        let mut var_qbody_bt_n_ius_dn0: f64 = *var_qbody_bt_n_ius_dn0_slot;
        let mut var_qbody_bt_n_ius_dn10: f64 = *var_qbody_bt_n_ius_dn10_slot;
        let mut var_qbody_bt_n_ius_dn11: f64 = *var_qbody_bt_n_ius_dn11_slot;
        let mut var_qbody_bt_n_ius_dn12: f64 = *var_qbody_bt_n_ius_dn12_slot;
        let mut var_qbody_bt_n_ius_dn17: f64 = *var_qbody_bt_n_ius_dn17_slot;
        let mut var_qbody_bt_n_ius_dn2: f64 = *var_qbody_bt_n_ius_dn2_slot;
        let mut var_qbody_bt_n_ius_dn6: f64 = *var_qbody_bt_n_ius_dn6_slot;
        let mut var_qbody_bt_n_ius_dn7: f64 = *var_qbody_bt_n_ius_dn7_slot;
        let mut var_qbody_bt_n_ius_rv: f64 = *var_qbody_bt_n_ius_rv_slot;
        let mut var_qbody_bt_n_sud: f64 = *var_qbody_bt_n_sud_slot;
        let mut var_qbody_bt_n_sud_dn0: f64 = *var_qbody_bt_n_sud_dn0_slot;
        let mut var_qbody_bt_n_sud_dn10: f64 = *var_qbody_bt_n_sud_dn10_slot;
        let mut var_qbody_bt_n_sud_dn11: f64 = *var_qbody_bt_n_sud_dn11_slot;
        let mut var_qbody_bt_n_sud_dn12: f64 = *var_qbody_bt_n_sud_dn12_slot;
        let mut var_qbody_bt_n_sud_dn17: f64 = *var_qbody_bt_n_sud_dn17_slot;
        let mut var_qbody_bt_n_sud_dn2: f64 = *var_qbody_bt_n_sud_dn2_slot;
        let mut var_qbody_bt_n_sud_dn6: f64 = *var_qbody_bt_n_sud_dn6_slot;
        let mut var_qbody_bt_n_sud_dn7: f64 = *var_qbody_bt_n_sud_dn7_slot;
        let mut var_qbody_bt_n_sud_rv: f64 = *var_qbody_bt_n_sud_rv_slot;
        let mut var_qbody_bt_n_sus: f64 = *var_qbody_bt_n_sus_slot;
        let mut var_qbody_bt_n_sus_dn0: f64 = *var_qbody_bt_n_sus_dn0_slot;
        let mut var_qbody_bt_n_sus_dn10: f64 = *var_qbody_bt_n_sus_dn10_slot;
        let mut var_qbody_bt_n_sus_dn11: f64 = *var_qbody_bt_n_sus_dn11_slot;
        let mut var_qbody_bt_n_sus_dn12: f64 = *var_qbody_bt_n_sus_dn12_slot;
        let mut var_qbody_bt_n_sus_dn17: f64 = *var_qbody_bt_n_sus_dn17_slot;
        let mut var_qbody_bt_n_sus_dn2: f64 = *var_qbody_bt_n_sus_dn2_slot;
        let mut var_qbody_bt_n_sus_dn6: f64 = *var_qbody_bt_n_sus_dn6_slot;
        let mut var_qbody_bt_n_sus_dn7: f64 = *var_qbody_bt_n_sus_dn7_slot;
        let mut var_qbody_bt_n_sus_rv: f64 = *var_qbody_bt_n_sus_rv_slot;
        let mut var_qbody_bt_p_iud: f64 = *var_qbody_bt_p_iud_slot;
        let mut var_qbody_bt_p_iud_dn0: f64 = *var_qbody_bt_p_iud_dn0_slot;
        let mut var_qbody_bt_p_iud_dn10: f64 = *var_qbody_bt_p_iud_dn10_slot;
        let mut var_qbody_bt_p_iud_dn11: f64 = *var_qbody_bt_p_iud_dn11_slot;
        let mut var_qbody_bt_p_iud_dn12: f64 = *var_qbody_bt_p_iud_dn12_slot;
        let mut var_qbody_bt_p_iud_dn17: f64 = *var_qbody_bt_p_iud_dn17_slot;
        let mut var_qbody_bt_p_iud_dn2: f64 = *var_qbody_bt_p_iud_dn2_slot;
        let mut var_qbody_bt_p_iud_dn6: f64 = *var_qbody_bt_p_iud_dn6_slot;
        let mut var_qbody_bt_p_iud_dn7: f64 = *var_qbody_bt_p_iud_dn7_slot;
        let mut var_qbody_bt_p_iud_rv: f64 = *var_qbody_bt_p_iud_rv_slot;
        let mut var_qbody_bt_p_ius: f64 = *var_qbody_bt_p_ius_slot;
        let mut var_qbody_bt_p_ius_dn0: f64 = *var_qbody_bt_p_ius_dn0_slot;
        let mut var_qbody_bt_p_ius_dn10: f64 = *var_qbody_bt_p_ius_dn10_slot;
        let mut var_qbody_bt_p_ius_dn11: f64 = *var_qbody_bt_p_ius_dn11_slot;
        let mut var_qbody_bt_p_ius_dn12: f64 = *var_qbody_bt_p_ius_dn12_slot;
        let mut var_qbody_bt_p_ius_dn17: f64 = *var_qbody_bt_p_ius_dn17_slot;
        let mut var_qbody_bt_p_ius_dn2: f64 = *var_qbody_bt_p_ius_dn2_slot;
        let mut var_qbody_bt_p_ius_dn6: f64 = *var_qbody_bt_p_ius_dn6_slot;
        let mut var_qbody_bt_p_ius_dn7: f64 = *var_qbody_bt_p_ius_dn7_slot;
        let mut var_qbody_bt_p_ius_rv: f64 = *var_qbody_bt_p_ius_rv_slot;
        let mut var_qbody_bt_p_sud: f64 = *var_qbody_bt_p_sud_slot;
        let mut var_qbody_bt_p_sud_dn0: f64 = *var_qbody_bt_p_sud_dn0_slot;
        let mut var_qbody_bt_p_sud_dn10: f64 = *var_qbody_bt_p_sud_dn10_slot;
        let mut var_qbody_bt_p_sud_dn11: f64 = *var_qbody_bt_p_sud_dn11_slot;
        let mut var_qbody_bt_p_sud_dn12: f64 = *var_qbody_bt_p_sud_dn12_slot;
        let mut var_qbody_bt_p_sud_dn17: f64 = *var_qbody_bt_p_sud_dn17_slot;
        let mut var_qbody_bt_p_sud_dn2: f64 = *var_qbody_bt_p_sud_dn2_slot;
        let mut var_qbody_bt_p_sud_dn6: f64 = *var_qbody_bt_p_sud_dn6_slot;
        let mut var_qbody_bt_p_sud_dn7: f64 = *var_qbody_bt_p_sud_dn7_slot;
        let mut var_qbody_bt_p_sud_rv: f64 = *var_qbody_bt_p_sud_rv_slot;
        let mut var_qbody_bt_p_sus: f64 = *var_qbody_bt_p_sus_slot;
        let mut var_qbody_bt_p_sus_dn0: f64 = *var_qbody_bt_p_sus_dn0_slot;
        let mut var_qbody_bt_p_sus_dn10: f64 = *var_qbody_bt_p_sus_dn10_slot;
        let mut var_qbody_bt_p_sus_dn11: f64 = *var_qbody_bt_p_sus_dn11_slot;
        let mut var_qbody_bt_p_sus_dn12: f64 = *var_qbody_bt_p_sus_dn12_slot;
        let mut var_qbody_bt_p_sus_dn17: f64 = *var_qbody_bt_p_sus_dn17_slot;
        let mut var_qbody_bt_p_sus_dn2: f64 = *var_qbody_bt_p_sus_dn2_slot;
        let mut var_qbody_bt_p_sus_dn6: f64 = *var_qbody_bt_p_sus_dn6_slot;
        let mut var_qbody_bt_p_sus_dn7: f64 = *var_qbody_bt_p_sus_dn7_slot;
        let mut var_qbody_bt_p_sus_rv: f64 = *var_qbody_bt_p_sus_rv_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn0: f64 = *var_qbs_dn0_slot;
        let mut var_qbs_dn10: f64 = *var_qbs_dn10_slot;
        let mut var_qbs_dn11: f64 = *var_qbs_dn11_slot;
        let mut var_qbs_dn12: f64 = *var_qbs_dn12_slot;
        let mut var_qbs_dn17: f64 = *var_qbs_dn17_slot;
        let mut var_qbs_dn2: f64 = *var_qbs_dn2_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_qbs_rv: f64 = *var_qbs_rv_slot;
        let mut var_qbsld: f64 = *var_qbsld_slot;
        let mut var_qbsld_dn0: f64 = *var_qbsld_dn0_slot;
        let mut var_qbsld_dn10: f64 = *var_qbsld_dn10_slot;
        let mut var_qbsld_dn11: f64 = *var_qbsld_dn11_slot;
        let mut var_qbsld_dn12: f64 = *var_qbsld_dn12_slot;
        let mut var_qbsld_dn17: f64 = *var_qbsld_dn17_slot;
        let mut var_qbsld_dn2: f64 = *var_qbsld_dn2_slot;
        let mut var_qbsld_dn6: f64 = *var_qbsld_dn6_slot;
        let mut var_qbsld_dn7: f64 = *var_qbsld_dn7_slot;
        let mut var_qbsld_rv: f64 = *var_qbsld_rv_slot;
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
        let mut var_qd_rv: f64 = *var_qd_rv_slot;
        let mut var_qgob: f64 = *var_qgob_slot;
        let mut var_qgob_dn0: f64 = *var_qgob_dn0_slot;
        let mut var_qgob_dn10: f64 = *var_qgob_dn10_slot;
        let mut var_qgob_dn11: f64 = *var_qgob_dn11_slot;
        let mut var_qgob_dn12: f64 = *var_qgob_dn12_slot;
        let mut var_qgob_dn17: f64 = *var_qgob_dn17_slot;
        let mut var_qgob_dn2: f64 = *var_qgob_dn2_slot;
        let mut var_qgob_dn6: f64 = *var_qgob_dn6_slot;
        let mut var_qgob_dn7: f64 = *var_qgob_dn7_slot;
        let mut var_qgob_rv: f64 = *var_qgob_rv_slot;
        let mut var_qgod: f64 = *var_qgod_slot;
        let mut var_qgod_dn0: f64 = *var_qgod_dn0_slot;
        let mut var_qgod_dn10: f64 = *var_qgod_dn10_slot;
        let mut var_qgod_dn11: f64 = *var_qgod_dn11_slot;
        let mut var_qgod_dn12: f64 = *var_qgod_dn12_slot;
        let mut var_qgod_dn17: f64 = *var_qgod_dn17_slot;
        let mut var_qgod_dn2: f64 = *var_qgod_dn2_slot;
        let mut var_qgod_dn6: f64 = *var_qgod_dn6_slot;
        let mut var_qgod_dn7: f64 = *var_qgod_dn7_slot;
        let mut var_qgod_rv: f64 = *var_qgod_rv_slot;
        let mut var_qgos: f64 = *var_qgos_slot;
        let mut var_qgos_dn0: f64 = *var_qgos_dn0_slot;
        let mut var_qgos_dn10: f64 = *var_qgos_dn10_slot;
        let mut var_qgos_dn11: f64 = *var_qgos_dn11_slot;
        let mut var_qgos_dn12: f64 = *var_qgos_dn12_slot;
        let mut var_qgos_dn17: f64 = *var_qgos_dn17_slot;
        let mut var_qgos_dn2: f64 = *var_qgos_dn2_slot;
        let mut var_qgos_dn6: f64 = *var_qgos_dn6_slot;
        let mut var_qgos_dn7: f64 = *var_qgos_dn7_slot;
        let mut var_qgos_rv: f64 = *var_qgos_rv_slot;
        let mut var_qi: f64 = *var_qi_slot;
        let mut var_qi_dn0: f64 = *var_qi_dn0_slot;
        let mut var_qi_dn10: f64 = *var_qi_dn10_slot;
        let mut var_qi_dn11: f64 = *var_qi_dn11_slot;
        let mut var_qi_dn12: f64 = *var_qi_dn12_slot;
        let mut var_qi_dn17: f64 = *var_qi_dn17_slot;
        let mut var_qi_dn2: f64 = *var_qi_dn2_slot;
        let mut var_qi_dn6: f64 = *var_qi_dn6_slot;
        let mut var_qi_dn7: f64 = *var_qi_dn7_slot;
        let mut var_qi_rv: f64 = *var_qi_rv_slot;
        let mut var_qidn: f64 = *var_qidn_slot;
        let mut var_qidn_dn0: f64 = *var_qidn_dn0_slot;
        let mut var_qidn_dn10: f64 = *var_qidn_dn10_slot;
        let mut var_qidn_dn11: f64 = *var_qidn_dn11_slot;
        let mut var_qidn_dn12: f64 = *var_qidn_dn12_slot;
        let mut var_qidn_dn17: f64 = *var_qidn_dn17_slot;
        let mut var_qidn_dn2: f64 = *var_qidn_dn2_slot;
        let mut var_qidn_dn6: f64 = *var_qidn_dn6_slot;
        let mut var_qidn_dn7: f64 = *var_qidn_dn7_slot;
        let mut var_qidn_rv: f64 = *var_qidn_rv_slot;
        let mut var_qinm: f64 = *var_qinm_slot;
        let mut var_qinm_dn0: f64 = *var_qinm_dn0_slot;
        let mut var_qinm_dn10: f64 = *var_qinm_dn10_slot;
        let mut var_qinm_dn11: f64 = *var_qinm_dn11_slot;
        let mut var_qinm_dn12: f64 = *var_qinm_dn12_slot;
        let mut var_qinm_dn17: f64 = *var_qinm_dn17_slot;
        let mut var_qinm_dn2: f64 = *var_qinm_dn2_slot;
        let mut var_qinm_dn6: f64 = *var_qinm_dn6_slot;
        let mut var_qinm_dn7: f64 = *var_qinm_dn7_slot;
        let mut var_qinm_rv: f64 = *var_qinm_rv_slot;
        let mut var_qn0: f64 = *var_qn0_slot;
        let mut var_qn0_dn0: f64 = *var_qn0_dn0_slot;
        let mut var_qn0_dn10: f64 = *var_qn0_dn10_slot;
        let mut var_qn0_dn11: f64 = *var_qn0_dn11_slot;
        let mut var_qn0_dn12: f64 = *var_qn0_dn12_slot;
        let mut var_qn0_dn17: f64 = *var_qn0_dn17_slot;
        let mut var_qn0_dn2: f64 = *var_qn0_dn2_slot;
        let mut var_qn0_dn6: f64 = *var_qn0_dn6_slot;
        let mut var_qn0_dn7: f64 = *var_qn0_dn7_slot;
        let mut var_qn0_rv: f64 = *var_qn0_rv_slot;
        let mut var_qovd: f64 = *var_qovd_slot;
        let mut var_qovd_dn0: f64 = *var_qovd_dn0_slot;
        let mut var_qovd_dn10: f64 = *var_qovd_dn10_slot;
        let mut var_qovd_dn11: f64 = *var_qovd_dn11_slot;
        let mut var_qovd_dn12: f64 = *var_qovd_dn12_slot;
        let mut var_qovd_dn17: f64 = *var_qovd_dn17_slot;
        let mut var_qovd_dn2: f64 = *var_qovd_dn2_slot;
        let mut var_qovd_dn6: f64 = *var_qovd_dn6_slot;
        let mut var_qovd_dn7: f64 = *var_qovd_dn7_slot;
        let mut var_qovd_rv: f64 = *var_qovd_rv_slot;
        let mut var_qovs: f64 = *var_qovs_slot;
        let mut var_qovs_dn0: f64 = *var_qovs_dn0_slot;
        let mut var_qovs_dn10: f64 = *var_qovs_dn10_slot;
        let mut var_qovs_dn11: f64 = *var_qovs_dn11_slot;
        let mut var_qovs_dn12: f64 = *var_qovs_dn12_slot;
        let mut var_qovs_dn17: f64 = *var_qovs_dn17_slot;
        let mut var_qovs_dn2: f64 = *var_qovs_dn2_slot;
        let mut var_qovs_dn6: f64 = *var_qovs_dn6_slot;
        let mut var_qovs_dn7: f64 = *var_qovs_dn7_slot;
        let mut var_qovs_rv: f64 = *var_qovs_rv_slot;
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
        let mut var_qse_rv: f64 = *var_qse_rv_slot;
        let mut var_vgvt: f64 = *var_vgvt_slot;
        let mut var_vgvt_dn0: f64 = *var_vgvt_dn0_slot;
        let mut var_vgvt_dn10: f64 = *var_vgvt_dn10_slot;
        let mut var_vgvt_dn11: f64 = *var_vgvt_dn11_slot;
        let mut var_vgvt_dn12: f64 = *var_vgvt_dn12_slot;
        let mut var_vgvt_dn17: f64 = *var_vgvt_dn17_slot;
        let mut var_vgvt_dn2: f64 = *var_vgvt_dn2_slot;
        let mut var_vgvt_dn6: f64 = *var_vgvt_dn6_slot;
        let mut var_vgvt_dn7: f64 = *var_vgvt_dn7_slot;
        let mut var_vgvt_rv: f64 = *var_vgvt_rv_slot;
        let mut var_wdsoi_0: f64 = *var_wdsoi_0_slot;
        let mut var_wdsoi_0_rv: f64 = *var_wdsoi_0_rv_slot;
        let mut var_xd: f64 = *var_xd_slot;
        let mut var_xd_dn0: f64 = *var_xd_dn0_slot;
        let mut var_xd_dn10: f64 = *var_xd_dn10_slot;
        let mut var_xd_dn11: f64 = *var_xd_dn11_slot;
        let mut var_xd_dn12: f64 = *var_xd_dn12_slot;
        let mut var_xd_dn17: f64 = *var_xd_dn17_slot;
        let mut var_xd_dn2: f64 = *var_xd_dn2_slot;
        let mut var_xd_dn6: f64 = *var_xd_dn6_slot;
        let mut var_xd_dn7: f64 = *var_xd_dn7_slot;
        let mut var_xd_rv: f64 = *var_xd_rv_slot;

        var_idd = 0.0;
        var_idd_dn0 = 0.0;
        var_idd_dn2 = 0.0;
        var_idd_dn6 = 0.0;
        var_idd_dn7 = 0.0;
        var_idd_dn10 = 0.0;
        var_idd_dn11 = 0.0;
        var_idd_dn12 = 0.0;
        var_idd_dn17 = 0.0;
        var_idd_rv = 0.0;

        var_gds0_ign = 1e-12;
        var_gds0_ign_dn0 = 0.0;
        var_gds0_ign_dn2 = 0.0;
        var_gds0_ign_dn6 = 0.0;
        var_gds0_ign_dn7 = 0.0;
        var_gds0_ign_dn10 = 0.0;
        var_gds0_ign_dn11 = 0.0;
        var_gds0_ign_dn12 = 0.0;
        var_gds0_ign_dn17 = 0.0;
        var_gds0_ign_rv = 0.0;

        var_qse = 0.0;
        var_qse_dn0 = 0.0;
        var_qse_dn2 = 0.0;
        var_qse_dn6 = 0.0;
        var_qse_dn7 = 0.0;
        var_qse_dn10 = 0.0;
        var_qse_dn11 = 0.0;
        var_qse_dn12 = 0.0;
        var_qse_dn13 = 0.0;
        var_qse_dn15 = 0.0;
        var_qse_dn16 = 0.0;
        var_qse_dn17 = 0.0;
        var_qse_dn18 = 0.0;
        var_qse_rv = 0.0;

        var_flg_ign = 0.0;
        var_flg_ign_rv = 0.0;

        var_end_of_part_1 = 0.0;
        var_end_of_part_1_rv = 0.0;

        var_xd = 0.0;
        var_xd_dn0 = 0.0;
        var_xd_dn2 = 0.0;
        var_xd_dn6 = 0.0;
        var_xd_dn7 = 0.0;
        var_xd_dn10 = 0.0;
        var_xd_dn11 = 0.0;
        var_xd_dn12 = 0.0;
        var_xd_dn17 = 0.0;
        var_xd_rv = 0.0;

        var_flg_noqi = 0.0;
        var_flg_noqi_rv = 0.0;

        var_flg_zone = 0.0;
        var_flg_zone_rv = 0.0;

        var_psl = 0.0;
        var_psl_dn0 = 0.0;
        var_psl_dn2 = 0.0;
        var_psl_dn6 = 0.0;
        var_psl_dn7 = 0.0;
        var_psl_dn10 = 0.0;
        var_psl_dn11 = 0.0;
        var_psl_dn12 = 0.0;
        var_psl_dn17 = 0.0;
        var_psl_rv = 0.0;

        var_psl_lim = 0.0;
        var_psl_lim_dn0 = 0.0;
        var_psl_lim_dn2 = 0.0;
        var_psl_lim_dn6 = 0.0;
        var_psl_lim_dn7 = 0.0;
        var_psl_lim_dn10 = 0.0;
        var_psl_lim_dn11 = 0.0;
        var_psl_lim_dn12 = 0.0;
        var_psl_lim_dn17 = 0.0;
        var_psl_lim_rv = 0.0;

        var_pds = 0.0;
        var_pds_dn0 = 0.0;
        var_pds_dn2 = 0.0;
        var_pds_dn6 = 0.0;
        var_pds_dn7 = 0.0;
        var_pds_dn10 = 0.0;
        var_pds_dn11 = 0.0;
        var_pds_dn12 = 0.0;
        var_pds_dn17 = 0.0;
        var_pds_rv = 0.0;

        var_pds_ini = 0.0;
        var_pds_ini_dn0 = 0.0;
        var_pds_ini_dn2 = 0.0;
        var_pds_ini_dn6 = 0.0;
        var_pds_ini_dn7 = 0.0;
        var_pds_ini_dn10 = 0.0;
        var_pds_ini_dn11 = 0.0;
        var_pds_ini_dn12 = 0.0;
        var_pds_ini_dn17 = 0.0;
        var_pds_ini_rv = 0.0;

        var_ps0z = 1.0;
        var_ps0z_dn0 = 0.0;
        var_ps0z_dn2 = 0.0;
        var_ps0z_dn6 = 0.0;
        var_ps0z_dn7 = 0.0;
        var_ps0z_dn10 = 0.0;
        var_ps0z_dn11 = 0.0;
        var_ps0z_dn12 = 0.0;
        var_ps0z_dn17 = 0.0;
        var_ps0z_rv = 0.0;

        var_alpha = 0.0;
        var_alpha_dn0 = 0.0;
        var_alpha_dn2 = 0.0;
        var_alpha_dn6 = 0.0;
        var_alpha_dn7 = 0.0;
        var_alpha_dn10 = 0.0;
        var_alpha_dn11 = 0.0;
        var_alpha_dn12 = 0.0;
        var_alpha_dn17 = 0.0;
        var_alpha_rv = 0.0;

        var_vgvt = 0.0;
        var_vgvt_dn0 = 0.0;
        var_vgvt_dn2 = 0.0;
        var_vgvt_dn6 = 0.0;
        var_vgvt_dn7 = 0.0;
        var_vgvt_dn10 = 0.0;
        var_vgvt_dn11 = 0.0;
        var_vgvt_dn12 = 0.0;
        var_vgvt_dn17 = 0.0;
        var_vgvt_rv = 0.0;

        var_qb = 0.0;
        var_qb_dn0 = 0.0;
        var_qb_dn2 = 0.0;
        var_qb_dn6 = 0.0;
        var_qb_dn7 = 0.0;
        var_qb_dn10 = 0.0;
        var_qb_dn11 = 0.0;
        var_qb_dn12 = 0.0;
        var_qb_dn13 = 0.0;
        var_qb_dn15 = 0.0;
        var_qb_dn16 = 0.0;
        var_qb_dn17 = 0.0;
        var_qb_dn18 = 0.0;
        var_qb_rv = 0.0;

        var_qi = 0.0;
        var_qi_dn0 = 0.0;
        var_qi_dn2 = 0.0;
        var_qi_dn6 = 0.0;
        var_qi_dn7 = 0.0;
        var_qi_dn10 = 0.0;
        var_qi_dn11 = 0.0;
        var_qi_dn12 = 0.0;
        var_qi_dn17 = 0.0;
        var_qi_rv = 0.0;

        var_qd = 0.0;
        var_qd_dn0 = 0.0;
        var_qd_dn2 = 0.0;
        var_qd_dn6 = 0.0;
        var_qd_dn7 = 0.0;
        var_qd_dn10 = 0.0;
        var_qd_dn11 = 0.0;
        var_qd_dn12 = 0.0;
        var_qd_dn13 = 0.0;
        var_qd_dn15 = 0.0;
        var_qd_dn16 = 0.0;
        var_qd_dn17 = 0.0;
        var_qd_dn18 = 0.0;
        var_qd_rv = 0.0;

        var_ids = 0.0;
        var_ids_dn0 = 0.0;
        var_ids_dn2 = 0.0;
        var_ids_dn6 = 0.0;
        var_ids_dn7 = 0.0;
        var_ids_dn10 = 0.0;
        var_ids_dn11 = 0.0;
        var_ids_dn12 = 0.0;
        var_ids_dn17 = 0.0;
        var_ids_rv = 0.0;

        var_fb = 0.0;
        var_fb_dn0 = 0.0;
        var_fb_dn2 = 0.0;
        var_fb_dn6 = 0.0;
        var_fb_dn7 = 0.0;
        var_fb_dn10 = 0.0;
        var_fb_dn11 = 0.0;
        var_fb_dn12 = 0.0;
        var_fb_dn17 = 0.0;
        var_fb_rv = 0.0;

        var_qn0 = 0.0;
        var_qn0_dn0 = 0.0;
        var_qn0_dn2 = 0.0;
        var_qn0_dn6 = 0.0;
        var_qn0_dn7 = 0.0;
        var_qn0_dn10 = 0.0;
        var_qn0_dn11 = 0.0;
        var_qn0_dn12 = 0.0;
        var_qn0_dn17 = 0.0;
        var_qn0_rv = 0.0;

        var_mu = 0.0;
        var_mu_dn0 = 0.0;
        var_mu_dn2 = 0.0;
        var_mu_dn6 = 0.0;
        var_mu_dn7 = 0.0;
        var_mu_dn10 = 0.0;
        var_mu_dn11 = 0.0;
        var_mu_dn12 = 0.0;
        var_mu_dn17 = 0.0;
        var_mu_rv = 0.0;

        var_muun = 0.0;
        var_muun_dn0 = 0.0;
        var_muun_dn2 = 0.0;
        var_muun_dn6 = 0.0;
        var_muun_dn7 = 0.0;
        var_muun_dn10 = 0.0;
        var_muun_dn11 = 0.0;
        var_muun_dn12 = 0.0;
        var_muun_dn17 = 0.0;
        var_muun_rv = 0.0;

        var_ey = 0.0;
        var_ey_dn0 = 0.0;
        var_ey_dn2 = 0.0;
        var_ey_dn6 = 0.0;
        var_ey_dn7 = 0.0;
        var_ey_dn10 = 0.0;
        var_ey_dn11 = 0.0;
        var_ey_dn12 = 0.0;
        var_ey_dn17 = 0.0;
        var_ey_rv = 0.0;

        var_isub = 0.0;
        var_isub_dn0 = 0.0;
        var_isub_dn2 = 0.0;
        var_isub_dn6 = 0.0;
        var_isub_dn7 = 0.0;
        var_isub_dn10 = 0.0;
        var_isub_dn11 = 0.0;
        var_isub_dn12 = 0.0;
        var_isub_dn17 = 0.0;
        var_isub_rv = 0.0;

        var_betawl = 1.0;
        var_betawl_dn0 = 0.0;
        var_betawl_dn2 = 0.0;
        var_betawl_dn6 = 0.0;
        var_betawl_dn7 = 0.0;
        var_betawl_dn10 = 0.0;
        var_betawl_dn11 = 0.0;
        var_betawl_dn12 = 0.0;
        var_betawl_dn17 = 0.0;
        var_betawl_rv = 0.0;

        var_idsibpc = 0.0;
        var_idsibpc_dn0 = 0.0;
        var_idsibpc_dn2 = 0.0;
        var_idsibpc_dn6 = 0.0;
        var_idsibpc_dn7 = 0.0;
        var_idsibpc_dn10 = 0.0;
        var_idsibpc_dn11 = 0.0;
        var_idsibpc_dn12 = 0.0;
        var_idsibpc_dn17 = 0.0;
        var_idsibpc_rv = 0.0;

        var_qgos = 0.0;
        var_qgos_dn0 = 0.0;
        var_qgos_dn2 = 0.0;
        var_qgos_dn6 = 0.0;
        var_qgos_dn7 = 0.0;
        var_qgos_dn10 = 0.0;
        var_qgos_dn11 = 0.0;
        var_qgos_dn12 = 0.0;
        var_qgos_dn17 = 0.0;
        var_qgos_rv = 0.0;

        var_qgod = 0.0;
        var_qgod_dn0 = 0.0;
        var_qgod_dn2 = 0.0;
        var_qgod_dn6 = 0.0;
        var_qgod_dn7 = 0.0;
        var_qgod_dn10 = 0.0;
        var_qgod_dn11 = 0.0;
        var_qgod_dn12 = 0.0;
        var_qgod_dn17 = 0.0;
        var_qgod_rv = 0.0;

        var_qgob = 0.0;
        var_qgob_dn0 = 0.0;
        var_qgob_dn2 = 0.0;
        var_qgob_dn6 = 0.0;
        var_qgob_dn7 = 0.0;
        var_qgob_dn10 = 0.0;
        var_qgob_dn11 = 0.0;
        var_qgob_dn12 = 0.0;
        var_qgob_dn17 = 0.0;
        var_qgob_rv = 0.0;

        var_qovd = 0.0;
        var_qovd_dn0 = 0.0;
        var_qovd_dn2 = 0.0;
        var_qovd_dn6 = 0.0;
        var_qovd_dn7 = 0.0;
        var_qovd_dn10 = 0.0;
        var_qovd_dn11 = 0.0;
        var_qovd_dn12 = 0.0;
        var_qovd_dn17 = 0.0;
        var_qovd_rv = 0.0;

        var_qovs = 0.0;
        var_qovs_dn0 = 0.0;
        var_qovs_dn2 = 0.0;
        var_qovs_dn6 = 0.0;
        var_qovs_dn7 = 0.0;
        var_qovs_dn10 = 0.0;
        var_qovs_dn11 = 0.0;
        var_qovs_dn12 = 0.0;
        var_qovs_dn17 = 0.0;
        var_qovs_rv = 0.0;

        var_qbdld = 0.0;
        var_qbdld_dn0 = 0.0;
        var_qbdld_dn2 = 0.0;
        var_qbdld_dn6 = 0.0;
        var_qbdld_dn7 = 0.0;
        var_qbdld_dn10 = 0.0;
        var_qbdld_dn11 = 0.0;
        var_qbdld_dn12 = 0.0;
        var_qbdld_dn17 = 0.0;
        var_qbdld_rv = 0.0;

        var_qbsld = 0.0;
        var_qbsld_dn0 = 0.0;
        var_qbsld_dn2 = 0.0;
        var_qbsld_dn6 = 0.0;
        var_qbsld_dn7 = 0.0;
        var_qbsld_dn10 = 0.0;
        var_qbsld_dn11 = 0.0;
        var_qbsld_dn12 = 0.0;
        var_qbsld_dn17 = 0.0;
        var_qbsld_rv = 0.0;

        var_ibd = 0.0;
        var_ibd_dn0 = 0.0;
        var_ibd_dn2 = 0.0;
        var_ibd_dn6 = 0.0;
        var_ibd_dn7 = 0.0;
        var_ibd_dn10 = 0.0;
        var_ibd_dn11 = 0.0;
        var_ibd_dn12 = 0.0;
        var_ibd_dn17 = 0.0;
        var_ibd_rv = 0.0;

        var_ibs = 0.0;
        var_ibs_dn0 = 0.0;
        var_ibs_dn2 = 0.0;
        var_ibs_dn6 = 0.0;
        var_ibs_dn7 = 0.0;
        var_ibs_dn10 = 0.0;
        var_ibs_dn11 = 0.0;
        var_ibs_dn12 = 0.0;
        var_ibs_dn17 = 0.0;
        var_ibs_rv = 0.0;

        var_qbd = 0.0;
        var_qbd_dn0 = 0.0;
        var_qbd_dn2 = 0.0;
        var_qbd_dn6 = 0.0;
        var_qbd_dn7 = 0.0;
        var_qbd_dn10 = 0.0;
        var_qbd_dn11 = 0.0;
        var_qbd_dn12 = 0.0;
        var_qbd_dn17 = 0.0;
        var_qbd_rv = 0.0;

        var_qbs = 0.0;
        var_qbs_dn0 = 0.0;
        var_qbs_dn2 = 0.0;
        var_qbs_dn6 = 0.0;
        var_qbs_dn7 = 0.0;
        var_qbs_dn10 = 0.0;
        var_qbs_dn11 = 0.0;
        var_qbs_dn12 = 0.0;
        var_qbs_dn17 = 0.0;
        var_qbs_rv = 0.0;

        var_qinm = 0.0;
        var_qinm_dn0 = 0.0;
        var_qinm_dn2 = 0.0;
        var_qinm_dn6 = 0.0;
        var_qinm_dn7 = 0.0;
        var_qinm_dn10 = 0.0;
        var_qinm_dn11 = 0.0;
        var_qinm_dn12 = 0.0;
        var_qinm_dn17 = 0.0;
        var_qinm_rv = 0.0;

        var_qidn = 0.0;
        var_qidn_dn0 = 0.0;
        var_qidn_dn2 = 0.0;
        var_qidn_dn6 = 0.0;
        var_qidn_dn7 = 0.0;
        var_qidn_dn10 = 0.0;
        var_qidn_dn11 = 0.0;
        var_qidn_dn12 = 0.0;
        var_qidn_dn17 = 0.0;
        var_qidn_rv = 0.0;

        var_wdsoi_0 = p.p237;
        var_wdsoi_0_rv = 0.0;

        var_qbody_bt_p_sus = 0.0;
        var_qbody_bt_p_sus_dn0 = 0.0;
        var_qbody_bt_p_sus_dn2 = 0.0;
        var_qbody_bt_p_sus_dn6 = 0.0;
        var_qbody_bt_p_sus_dn7 = 0.0;
        var_qbody_bt_p_sus_dn10 = 0.0;
        var_qbody_bt_p_sus_dn11 = 0.0;
        var_qbody_bt_p_sus_dn12 = 0.0;
        var_qbody_bt_p_sus_dn17 = 0.0;
        var_qbody_bt_p_sus_rv = 0.0;

        var_qbody_bt_p_sud = 0.0;
        var_qbody_bt_p_sud_dn0 = 0.0;
        var_qbody_bt_p_sud_dn2 = 0.0;
        var_qbody_bt_p_sud_dn6 = 0.0;
        var_qbody_bt_p_sud_dn7 = 0.0;
        var_qbody_bt_p_sud_dn10 = 0.0;
        var_qbody_bt_p_sud_dn11 = 0.0;
        var_qbody_bt_p_sud_dn12 = 0.0;
        var_qbody_bt_p_sud_dn17 = 0.0;
        var_qbody_bt_p_sud_rv = 0.0;

        var_qbody_bt_p_iud = 0.0;
        var_qbody_bt_p_iud_dn0 = 0.0;
        var_qbody_bt_p_iud_dn2 = 0.0;
        var_qbody_bt_p_iud_dn6 = 0.0;
        var_qbody_bt_p_iud_dn7 = 0.0;
        var_qbody_bt_p_iud_dn10 = 0.0;
        var_qbody_bt_p_iud_dn11 = 0.0;
        var_qbody_bt_p_iud_dn12 = 0.0;
        var_qbody_bt_p_iud_dn17 = 0.0;
        var_qbody_bt_p_iud_rv = 0.0;

        var_qbody_bt_p_ius = 0.0;
        var_qbody_bt_p_ius_dn0 = 0.0;
        var_qbody_bt_p_ius_dn2 = 0.0;
        var_qbody_bt_p_ius_dn6 = 0.0;
        var_qbody_bt_p_ius_dn7 = 0.0;
        var_qbody_bt_p_ius_dn10 = 0.0;
        var_qbody_bt_p_ius_dn11 = 0.0;
        var_qbody_bt_p_ius_dn12 = 0.0;
        var_qbody_bt_p_ius_dn17 = 0.0;
        var_qbody_bt_p_ius_rv = 0.0;

        var_qbody_bt_n_sus = 0.0;
        var_qbody_bt_n_sus_dn0 = 0.0;
        var_qbody_bt_n_sus_dn2 = 0.0;
        var_qbody_bt_n_sus_dn6 = 0.0;
        var_qbody_bt_n_sus_dn7 = 0.0;
        var_qbody_bt_n_sus_dn10 = 0.0;
        var_qbody_bt_n_sus_dn11 = 0.0;
        var_qbody_bt_n_sus_dn12 = 0.0;
        var_qbody_bt_n_sus_dn17 = 0.0;
        var_qbody_bt_n_sus_rv = 0.0;

        var_qbody_bt_n_sud = 0.0;
        var_qbody_bt_n_sud_dn0 = 0.0;
        var_qbody_bt_n_sud_dn2 = 0.0;
        var_qbody_bt_n_sud_dn6 = 0.0;
        var_qbody_bt_n_sud_dn7 = 0.0;
        var_qbody_bt_n_sud_dn10 = 0.0;
        var_qbody_bt_n_sud_dn11 = 0.0;
        var_qbody_bt_n_sud_dn12 = 0.0;
        var_qbody_bt_n_sud_dn17 = 0.0;
        var_qbody_bt_n_sud_rv = 0.0;

        var_qbody_bt_n_iud = 0.0;
        var_qbody_bt_n_iud_dn0 = 0.0;
        var_qbody_bt_n_iud_dn2 = 0.0;
        var_qbody_bt_n_iud_dn6 = 0.0;
        var_qbody_bt_n_iud_dn7 = 0.0;
        var_qbody_bt_n_iud_dn10 = 0.0;
        var_qbody_bt_n_iud_dn11 = 0.0;
        var_qbody_bt_n_iud_dn12 = 0.0;
        var_qbody_bt_n_iud_dn17 = 0.0;
        var_qbody_bt_n_iud_rv = 0.0;

        var_qbody_bt_n_ius = 0.0;
        var_qbody_bt_n_ius_dn0 = 0.0;
        var_qbody_bt_n_ius_dn2 = 0.0;
        var_qbody_bt_n_ius_dn6 = 0.0;
        var_qbody_bt_n_ius_dn7 = 0.0;
        var_qbody_bt_n_ius_dn10 = 0.0;
        var_qbody_bt_n_ius_dn11 = 0.0;
        var_qbody_bt_n_ius_dn12 = 0.0;
        var_qbody_bt_n_ius_dn17 = 0.0;
        var_qbody_bt_n_ius_rv = 0.0;

        *var_alpha_slot = var_alpha;
        *var_alpha_dn0_slot = var_alpha_dn0;
        *var_alpha_dn10_slot = var_alpha_dn10;
        *var_alpha_dn11_slot = var_alpha_dn11;
        *var_alpha_dn12_slot = var_alpha_dn12;
        *var_alpha_dn17_slot = var_alpha_dn17;
        *var_alpha_dn2_slot = var_alpha_dn2;
        *var_alpha_dn6_slot = var_alpha_dn6;
        *var_alpha_dn7_slot = var_alpha_dn7;
        *var_alpha_rv_slot = var_alpha_rv;
        *var_betawl_slot = var_betawl;
        *var_betawl_dn0_slot = var_betawl_dn0;
        *var_betawl_dn10_slot = var_betawl_dn10;
        *var_betawl_dn11_slot = var_betawl_dn11;
        *var_betawl_dn12_slot = var_betawl_dn12;
        *var_betawl_dn17_slot = var_betawl_dn17;
        *var_betawl_dn2_slot = var_betawl_dn2;
        *var_betawl_dn6_slot = var_betawl_dn6;
        *var_betawl_dn7_slot = var_betawl_dn7;
        *var_betawl_rv_slot = var_betawl_rv;
        *var_end_of_part_1_slot = var_end_of_part_1;
        *var_end_of_part_1_rv_slot = var_end_of_part_1_rv;
        *var_ey_slot = var_ey;
        *var_ey_dn0_slot = var_ey_dn0;
        *var_ey_dn10_slot = var_ey_dn10;
        *var_ey_dn11_slot = var_ey_dn11;
        *var_ey_dn12_slot = var_ey_dn12;
        *var_ey_dn17_slot = var_ey_dn17;
        *var_ey_dn2_slot = var_ey_dn2;
        *var_ey_dn6_slot = var_ey_dn6;
        *var_ey_dn7_slot = var_ey_dn7;
        *var_ey_rv_slot = var_ey_rv;
        *var_fb_slot = var_fb;
        *var_fb_dn0_slot = var_fb_dn0;
        *var_fb_dn10_slot = var_fb_dn10;
        *var_fb_dn11_slot = var_fb_dn11;
        *var_fb_dn12_slot = var_fb_dn12;
        *var_fb_dn17_slot = var_fb_dn17;
        *var_fb_dn2_slot = var_fb_dn2;
        *var_fb_dn6_slot = var_fb_dn6;
        *var_fb_dn7_slot = var_fb_dn7;
        *var_fb_rv_slot = var_fb_rv;
        *var_flg_ign_slot = var_flg_ign;
        *var_flg_ign_rv_slot = var_flg_ign_rv;
        *var_flg_noqi_slot = var_flg_noqi;
        *var_flg_noqi_rv_slot = var_flg_noqi_rv;
        *var_flg_zone_slot = var_flg_zone;
        *var_flg_zone_rv_slot = var_flg_zone_rv;
        *var_gds0_ign_slot = var_gds0_ign;
        *var_gds0_ign_dn0_slot = var_gds0_ign_dn0;
        *var_gds0_ign_dn10_slot = var_gds0_ign_dn10;
        *var_gds0_ign_dn11_slot = var_gds0_ign_dn11;
        *var_gds0_ign_dn12_slot = var_gds0_ign_dn12;
        *var_gds0_ign_dn17_slot = var_gds0_ign_dn17;
        *var_gds0_ign_dn2_slot = var_gds0_ign_dn2;
        *var_gds0_ign_dn6_slot = var_gds0_ign_dn6;
        *var_gds0_ign_dn7_slot = var_gds0_ign_dn7;
        *var_gds0_ign_rv_slot = var_gds0_ign_rv;
        *var_ibd_slot = var_ibd;
        *var_ibd_dn0_slot = var_ibd_dn0;
        *var_ibd_dn10_slot = var_ibd_dn10;
        *var_ibd_dn11_slot = var_ibd_dn11;
        *var_ibd_dn12_slot = var_ibd_dn12;
        *var_ibd_dn17_slot = var_ibd_dn17;
        *var_ibd_dn2_slot = var_ibd_dn2;
        *var_ibd_dn6_slot = var_ibd_dn6;
        *var_ibd_dn7_slot = var_ibd_dn7;
        *var_ibd_rv_slot = var_ibd_rv;
        *var_ibs_slot = var_ibs;
        *var_ibs_dn0_slot = var_ibs_dn0;
        *var_ibs_dn10_slot = var_ibs_dn10;
        *var_ibs_dn11_slot = var_ibs_dn11;
        *var_ibs_dn12_slot = var_ibs_dn12;
        *var_ibs_dn17_slot = var_ibs_dn17;
        *var_ibs_dn2_slot = var_ibs_dn2;
        *var_ibs_dn6_slot = var_ibs_dn6;
        *var_ibs_dn7_slot = var_ibs_dn7;
        *var_ibs_rv_slot = var_ibs_rv;
        *var_idd_slot = var_idd;
        *var_idd_dn0_slot = var_idd_dn0;
        *var_idd_dn10_slot = var_idd_dn10;
        *var_idd_dn11_slot = var_idd_dn11;
        *var_idd_dn12_slot = var_idd_dn12;
        *var_idd_dn17_slot = var_idd_dn17;
        *var_idd_dn2_slot = var_idd_dn2;
        *var_idd_dn6_slot = var_idd_dn6;
        *var_idd_dn7_slot = var_idd_dn7;
        *var_idd_rv_slot = var_idd_rv;
        *var_ids_slot = var_ids;
        *var_ids_dn0_slot = var_ids_dn0;
        *var_ids_dn10_slot = var_ids_dn10;
        *var_ids_dn11_slot = var_ids_dn11;
        *var_ids_dn12_slot = var_ids_dn12;
        *var_ids_dn17_slot = var_ids_dn17;
        *var_ids_dn2_slot = var_ids_dn2;
        *var_ids_dn6_slot = var_ids_dn6;
        *var_ids_dn7_slot = var_ids_dn7;
        *var_ids_rv_slot = var_ids_rv;
        *var_idsibpc_slot = var_idsibpc;
        *var_idsibpc_dn0_slot = var_idsibpc_dn0;
        *var_idsibpc_dn10_slot = var_idsibpc_dn10;
        *var_idsibpc_dn11_slot = var_idsibpc_dn11;
        *var_idsibpc_dn12_slot = var_idsibpc_dn12;
        *var_idsibpc_dn17_slot = var_idsibpc_dn17;
        *var_idsibpc_dn2_slot = var_idsibpc_dn2;
        *var_idsibpc_dn6_slot = var_idsibpc_dn6;
        *var_idsibpc_dn7_slot = var_idsibpc_dn7;
        *var_idsibpc_rv_slot = var_idsibpc_rv;
        *var_isub_slot = var_isub;
        *var_isub_dn0_slot = var_isub_dn0;
        *var_isub_dn10_slot = var_isub_dn10;
        *var_isub_dn11_slot = var_isub_dn11;
        *var_isub_dn12_slot = var_isub_dn12;
        *var_isub_dn17_slot = var_isub_dn17;
        *var_isub_dn2_slot = var_isub_dn2;
        *var_isub_dn6_slot = var_isub_dn6;
        *var_isub_dn7_slot = var_isub_dn7;
        *var_isub_rv_slot = var_isub_rv;
        *var_mu_slot = var_mu;
        *var_mu_dn0_slot = var_mu_dn0;
        *var_mu_dn10_slot = var_mu_dn10;
        *var_mu_dn11_slot = var_mu_dn11;
        *var_mu_dn12_slot = var_mu_dn12;
        *var_mu_dn17_slot = var_mu_dn17;
        *var_mu_dn2_slot = var_mu_dn2;
        *var_mu_dn6_slot = var_mu_dn6;
        *var_mu_dn7_slot = var_mu_dn7;
        *var_mu_rv_slot = var_mu_rv;
        *var_muun_slot = var_muun;
        *var_muun_dn0_slot = var_muun_dn0;
        *var_muun_dn10_slot = var_muun_dn10;
        *var_muun_dn11_slot = var_muun_dn11;
        *var_muun_dn12_slot = var_muun_dn12;
        *var_muun_dn17_slot = var_muun_dn17;
        *var_muun_dn2_slot = var_muun_dn2;
        *var_muun_dn6_slot = var_muun_dn6;
        *var_muun_dn7_slot = var_muun_dn7;
        *var_muun_rv_slot = var_muun_rv;
        *var_pds_slot = var_pds;
        *var_pds_dn0_slot = var_pds_dn0;
        *var_pds_dn10_slot = var_pds_dn10;
        *var_pds_dn11_slot = var_pds_dn11;
        *var_pds_dn12_slot = var_pds_dn12;
        *var_pds_dn17_slot = var_pds_dn17;
        *var_pds_dn2_slot = var_pds_dn2;
        *var_pds_dn6_slot = var_pds_dn6;
        *var_pds_dn7_slot = var_pds_dn7;
        *var_pds_ini_slot = var_pds_ini;
        *var_pds_ini_dn0_slot = var_pds_ini_dn0;
        *var_pds_ini_dn10_slot = var_pds_ini_dn10;
        *var_pds_ini_dn11_slot = var_pds_ini_dn11;
        *var_pds_ini_dn12_slot = var_pds_ini_dn12;
        *var_pds_ini_dn17_slot = var_pds_ini_dn17;
        *var_pds_ini_dn2_slot = var_pds_ini_dn2;
        *var_pds_ini_dn6_slot = var_pds_ini_dn6;
        *var_pds_ini_dn7_slot = var_pds_ini_dn7;
        *var_pds_ini_rv_slot = var_pds_ini_rv;
        *var_pds_rv_slot = var_pds_rv;
        *var_ps0z_slot = var_ps0z;
        *var_ps0z_dn0_slot = var_ps0z_dn0;
        *var_ps0z_dn10_slot = var_ps0z_dn10;
        *var_ps0z_dn11_slot = var_ps0z_dn11;
        *var_ps0z_dn12_slot = var_ps0z_dn12;
        *var_ps0z_dn17_slot = var_ps0z_dn17;
        *var_ps0z_dn2_slot = var_ps0z_dn2;
        *var_ps0z_dn6_slot = var_ps0z_dn6;
        *var_ps0z_dn7_slot = var_ps0z_dn7;
        *var_ps0z_rv_slot = var_ps0z_rv;
        *var_psl_slot = var_psl;
        *var_psl_dn0_slot = var_psl_dn0;
        *var_psl_dn10_slot = var_psl_dn10;
        *var_psl_dn11_slot = var_psl_dn11;
        *var_psl_dn12_slot = var_psl_dn12;
        *var_psl_dn17_slot = var_psl_dn17;
        *var_psl_dn2_slot = var_psl_dn2;
        *var_psl_dn6_slot = var_psl_dn6;
        *var_psl_dn7_slot = var_psl_dn7;
        *var_psl_lim_slot = var_psl_lim;
        *var_psl_lim_dn0_slot = var_psl_lim_dn0;
        *var_psl_lim_dn10_slot = var_psl_lim_dn10;
        *var_psl_lim_dn11_slot = var_psl_lim_dn11;
        *var_psl_lim_dn12_slot = var_psl_lim_dn12;
        *var_psl_lim_dn17_slot = var_psl_lim_dn17;
        *var_psl_lim_dn2_slot = var_psl_lim_dn2;
        *var_psl_lim_dn6_slot = var_psl_lim_dn6;
        *var_psl_lim_dn7_slot = var_psl_lim_dn7;
        *var_psl_lim_rv_slot = var_psl_lim_rv;
        *var_psl_rv_slot = var_psl_rv;
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
        *var_qb_rv_slot = var_qb_rv;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn0_slot = var_qbd_dn0;
        *var_qbd_dn10_slot = var_qbd_dn10;
        *var_qbd_dn11_slot = var_qbd_dn11;
        *var_qbd_dn12_slot = var_qbd_dn12;
        *var_qbd_dn17_slot = var_qbd_dn17;
        *var_qbd_dn2_slot = var_qbd_dn2;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbd_rv_slot = var_qbd_rv;
        *var_qbdld_slot = var_qbdld;
        *var_qbdld_dn0_slot = var_qbdld_dn0;
        *var_qbdld_dn10_slot = var_qbdld_dn10;
        *var_qbdld_dn11_slot = var_qbdld_dn11;
        *var_qbdld_dn12_slot = var_qbdld_dn12;
        *var_qbdld_dn17_slot = var_qbdld_dn17;
        *var_qbdld_dn2_slot = var_qbdld_dn2;
        *var_qbdld_dn6_slot = var_qbdld_dn6;
        *var_qbdld_dn7_slot = var_qbdld_dn7;
        *var_qbdld_rv_slot = var_qbdld_rv;
        *var_qbody_bt_n_iud_slot = var_qbody_bt_n_iud;
        *var_qbody_bt_n_iud_dn0_slot = var_qbody_bt_n_iud_dn0;
        *var_qbody_bt_n_iud_dn10_slot = var_qbody_bt_n_iud_dn10;
        *var_qbody_bt_n_iud_dn11_slot = var_qbody_bt_n_iud_dn11;
        *var_qbody_bt_n_iud_dn12_slot = var_qbody_bt_n_iud_dn12;
        *var_qbody_bt_n_iud_dn17_slot = var_qbody_bt_n_iud_dn17;
        *var_qbody_bt_n_iud_dn2_slot = var_qbody_bt_n_iud_dn2;
        *var_qbody_bt_n_iud_dn6_slot = var_qbody_bt_n_iud_dn6;
        *var_qbody_bt_n_iud_dn7_slot = var_qbody_bt_n_iud_dn7;
        *var_qbody_bt_n_iud_rv_slot = var_qbody_bt_n_iud_rv;
        *var_qbody_bt_n_ius_slot = var_qbody_bt_n_ius;
        *var_qbody_bt_n_ius_dn0_slot = var_qbody_bt_n_ius_dn0;
        *var_qbody_bt_n_ius_dn10_slot = var_qbody_bt_n_ius_dn10;
        *var_qbody_bt_n_ius_dn11_slot = var_qbody_bt_n_ius_dn11;
        *var_qbody_bt_n_ius_dn12_slot = var_qbody_bt_n_ius_dn12;
        *var_qbody_bt_n_ius_dn17_slot = var_qbody_bt_n_ius_dn17;
        *var_qbody_bt_n_ius_dn2_slot = var_qbody_bt_n_ius_dn2;
        *var_qbody_bt_n_ius_dn6_slot = var_qbody_bt_n_ius_dn6;
        *var_qbody_bt_n_ius_dn7_slot = var_qbody_bt_n_ius_dn7;
        *var_qbody_bt_n_ius_rv_slot = var_qbody_bt_n_ius_rv;
        *var_qbody_bt_n_sud_slot = var_qbody_bt_n_sud;
        *var_qbody_bt_n_sud_dn0_slot = var_qbody_bt_n_sud_dn0;
        *var_qbody_bt_n_sud_dn10_slot = var_qbody_bt_n_sud_dn10;
        *var_qbody_bt_n_sud_dn11_slot = var_qbody_bt_n_sud_dn11;
        *var_qbody_bt_n_sud_dn12_slot = var_qbody_bt_n_sud_dn12;
        *var_qbody_bt_n_sud_dn17_slot = var_qbody_bt_n_sud_dn17;
        *var_qbody_bt_n_sud_dn2_slot = var_qbody_bt_n_sud_dn2;
        *var_qbody_bt_n_sud_dn6_slot = var_qbody_bt_n_sud_dn6;
        *var_qbody_bt_n_sud_dn7_slot = var_qbody_bt_n_sud_dn7;
        *var_qbody_bt_n_sud_rv_slot = var_qbody_bt_n_sud_rv;
        *var_qbody_bt_n_sus_slot = var_qbody_bt_n_sus;
        *var_qbody_bt_n_sus_dn0_slot = var_qbody_bt_n_sus_dn0;
        *var_qbody_bt_n_sus_dn10_slot = var_qbody_bt_n_sus_dn10;
        *var_qbody_bt_n_sus_dn11_slot = var_qbody_bt_n_sus_dn11;
        *var_qbody_bt_n_sus_dn12_slot = var_qbody_bt_n_sus_dn12;
        *var_qbody_bt_n_sus_dn17_slot = var_qbody_bt_n_sus_dn17;
        *var_qbody_bt_n_sus_dn2_slot = var_qbody_bt_n_sus_dn2;
        *var_qbody_bt_n_sus_dn6_slot = var_qbody_bt_n_sus_dn6;
        *var_qbody_bt_n_sus_dn7_slot = var_qbody_bt_n_sus_dn7;
        *var_qbody_bt_n_sus_rv_slot = var_qbody_bt_n_sus_rv;
        *var_qbody_bt_p_iud_slot = var_qbody_bt_p_iud;
        *var_qbody_bt_p_iud_dn0_slot = var_qbody_bt_p_iud_dn0;
        *var_qbody_bt_p_iud_dn10_slot = var_qbody_bt_p_iud_dn10;
        *var_qbody_bt_p_iud_dn11_slot = var_qbody_bt_p_iud_dn11;
        *var_qbody_bt_p_iud_dn12_slot = var_qbody_bt_p_iud_dn12;
        *var_qbody_bt_p_iud_dn17_slot = var_qbody_bt_p_iud_dn17;
        *var_qbody_bt_p_iud_dn2_slot = var_qbody_bt_p_iud_dn2;
        *var_qbody_bt_p_iud_dn6_slot = var_qbody_bt_p_iud_dn6;
        *var_qbody_bt_p_iud_dn7_slot = var_qbody_bt_p_iud_dn7;
        *var_qbody_bt_p_iud_rv_slot = var_qbody_bt_p_iud_rv;
        *var_qbody_bt_p_ius_slot = var_qbody_bt_p_ius;
        *var_qbody_bt_p_ius_dn0_slot = var_qbody_bt_p_ius_dn0;
        *var_qbody_bt_p_ius_dn10_slot = var_qbody_bt_p_ius_dn10;
        *var_qbody_bt_p_ius_dn11_slot = var_qbody_bt_p_ius_dn11;
        *var_qbody_bt_p_ius_dn12_slot = var_qbody_bt_p_ius_dn12;
        *var_qbody_bt_p_ius_dn17_slot = var_qbody_bt_p_ius_dn17;
        *var_qbody_bt_p_ius_dn2_slot = var_qbody_bt_p_ius_dn2;
        *var_qbody_bt_p_ius_dn6_slot = var_qbody_bt_p_ius_dn6;
        *var_qbody_bt_p_ius_dn7_slot = var_qbody_bt_p_ius_dn7;
        *var_qbody_bt_p_ius_rv_slot = var_qbody_bt_p_ius_rv;
        *var_qbody_bt_p_sud_slot = var_qbody_bt_p_sud;
        *var_qbody_bt_p_sud_dn0_slot = var_qbody_bt_p_sud_dn0;
        *var_qbody_bt_p_sud_dn10_slot = var_qbody_bt_p_sud_dn10;
        *var_qbody_bt_p_sud_dn11_slot = var_qbody_bt_p_sud_dn11;
        *var_qbody_bt_p_sud_dn12_slot = var_qbody_bt_p_sud_dn12;
        *var_qbody_bt_p_sud_dn17_slot = var_qbody_bt_p_sud_dn17;
        *var_qbody_bt_p_sud_dn2_slot = var_qbody_bt_p_sud_dn2;
        *var_qbody_bt_p_sud_dn6_slot = var_qbody_bt_p_sud_dn6;
        *var_qbody_bt_p_sud_dn7_slot = var_qbody_bt_p_sud_dn7;
        *var_qbody_bt_p_sud_rv_slot = var_qbody_bt_p_sud_rv;
        *var_qbody_bt_p_sus_slot = var_qbody_bt_p_sus;
        *var_qbody_bt_p_sus_dn0_slot = var_qbody_bt_p_sus_dn0;
        *var_qbody_bt_p_sus_dn10_slot = var_qbody_bt_p_sus_dn10;
        *var_qbody_bt_p_sus_dn11_slot = var_qbody_bt_p_sus_dn11;
        *var_qbody_bt_p_sus_dn12_slot = var_qbody_bt_p_sus_dn12;
        *var_qbody_bt_p_sus_dn17_slot = var_qbody_bt_p_sus_dn17;
        *var_qbody_bt_p_sus_dn2_slot = var_qbody_bt_p_sus_dn2;
        *var_qbody_bt_p_sus_dn6_slot = var_qbody_bt_p_sus_dn6;
        *var_qbody_bt_p_sus_dn7_slot = var_qbody_bt_p_sus_dn7;
        *var_qbody_bt_p_sus_rv_slot = var_qbody_bt_p_sus_rv;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn0_slot = var_qbs_dn0;
        *var_qbs_dn10_slot = var_qbs_dn10;
        *var_qbs_dn11_slot = var_qbs_dn11;
        *var_qbs_dn12_slot = var_qbs_dn12;
        *var_qbs_dn17_slot = var_qbs_dn17;
        *var_qbs_dn2_slot = var_qbs_dn2;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_qbs_rv_slot = var_qbs_rv;
        *var_qbsld_slot = var_qbsld;
        *var_qbsld_dn0_slot = var_qbsld_dn0;
        *var_qbsld_dn10_slot = var_qbsld_dn10;
        *var_qbsld_dn11_slot = var_qbsld_dn11;
        *var_qbsld_dn12_slot = var_qbsld_dn12;
        *var_qbsld_dn17_slot = var_qbsld_dn17;
        *var_qbsld_dn2_slot = var_qbsld_dn2;
        *var_qbsld_dn6_slot = var_qbsld_dn6;
        *var_qbsld_dn7_slot = var_qbsld_dn7;
        *var_qbsld_rv_slot = var_qbsld_rv;
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
        *var_qd_rv_slot = var_qd_rv;
        *var_qgob_slot = var_qgob;
        *var_qgob_dn0_slot = var_qgob_dn0;
        *var_qgob_dn10_slot = var_qgob_dn10;
        *var_qgob_dn11_slot = var_qgob_dn11;
        *var_qgob_dn12_slot = var_qgob_dn12;
        *var_qgob_dn17_slot = var_qgob_dn17;
        *var_qgob_dn2_slot = var_qgob_dn2;
        *var_qgob_dn6_slot = var_qgob_dn6;
        *var_qgob_dn7_slot = var_qgob_dn7;
        *var_qgob_rv_slot = var_qgob_rv;
        *var_qgod_slot = var_qgod;
        *var_qgod_dn0_slot = var_qgod_dn0;
        *var_qgod_dn10_slot = var_qgod_dn10;
        *var_qgod_dn11_slot = var_qgod_dn11;
        *var_qgod_dn12_slot = var_qgod_dn12;
        *var_qgod_dn17_slot = var_qgod_dn17;
        *var_qgod_dn2_slot = var_qgod_dn2;
        *var_qgod_dn6_slot = var_qgod_dn6;
        *var_qgod_dn7_slot = var_qgod_dn7;
        *var_qgod_rv_slot = var_qgod_rv;
        *var_qgos_slot = var_qgos;
        *var_qgos_dn0_slot = var_qgos_dn0;
        *var_qgos_dn10_slot = var_qgos_dn10;
        *var_qgos_dn11_slot = var_qgos_dn11;
        *var_qgos_dn12_slot = var_qgos_dn12;
        *var_qgos_dn17_slot = var_qgos_dn17;
        *var_qgos_dn2_slot = var_qgos_dn2;
        *var_qgos_dn6_slot = var_qgos_dn6;
        *var_qgos_dn7_slot = var_qgos_dn7;
        *var_qgos_rv_slot = var_qgos_rv;
        *var_qi_slot = var_qi;
        *var_qi_dn0_slot = var_qi_dn0;
        *var_qi_dn10_slot = var_qi_dn10;
        *var_qi_dn11_slot = var_qi_dn11;
        *var_qi_dn12_slot = var_qi_dn12;
        *var_qi_dn17_slot = var_qi_dn17;
        *var_qi_dn2_slot = var_qi_dn2;
        *var_qi_dn6_slot = var_qi_dn6;
        *var_qi_dn7_slot = var_qi_dn7;
        *var_qi_rv_slot = var_qi_rv;
        *var_qidn_slot = var_qidn;
        *var_qidn_dn0_slot = var_qidn_dn0;
        *var_qidn_dn10_slot = var_qidn_dn10;
        *var_qidn_dn11_slot = var_qidn_dn11;
        *var_qidn_dn12_slot = var_qidn_dn12;
        *var_qidn_dn17_slot = var_qidn_dn17;
        *var_qidn_dn2_slot = var_qidn_dn2;
        *var_qidn_dn6_slot = var_qidn_dn6;
        *var_qidn_dn7_slot = var_qidn_dn7;
        *var_qidn_rv_slot = var_qidn_rv;
        *var_qinm_slot = var_qinm;
        *var_qinm_dn0_slot = var_qinm_dn0;
        *var_qinm_dn10_slot = var_qinm_dn10;
        *var_qinm_dn11_slot = var_qinm_dn11;
        *var_qinm_dn12_slot = var_qinm_dn12;
        *var_qinm_dn17_slot = var_qinm_dn17;
        *var_qinm_dn2_slot = var_qinm_dn2;
        *var_qinm_dn6_slot = var_qinm_dn6;
        *var_qinm_dn7_slot = var_qinm_dn7;
        *var_qinm_rv_slot = var_qinm_rv;
        *var_qn0_slot = var_qn0;
        *var_qn0_dn0_slot = var_qn0_dn0;
        *var_qn0_dn10_slot = var_qn0_dn10;
        *var_qn0_dn11_slot = var_qn0_dn11;
        *var_qn0_dn12_slot = var_qn0_dn12;
        *var_qn0_dn17_slot = var_qn0_dn17;
        *var_qn0_dn2_slot = var_qn0_dn2;
        *var_qn0_dn6_slot = var_qn0_dn6;
        *var_qn0_dn7_slot = var_qn0_dn7;
        *var_qn0_rv_slot = var_qn0_rv;
        *var_qovd_slot = var_qovd;
        *var_qovd_dn0_slot = var_qovd_dn0;
        *var_qovd_dn10_slot = var_qovd_dn10;
        *var_qovd_dn11_slot = var_qovd_dn11;
        *var_qovd_dn12_slot = var_qovd_dn12;
        *var_qovd_dn17_slot = var_qovd_dn17;
        *var_qovd_dn2_slot = var_qovd_dn2;
        *var_qovd_dn6_slot = var_qovd_dn6;
        *var_qovd_dn7_slot = var_qovd_dn7;
        *var_qovd_rv_slot = var_qovd_rv;
        *var_qovs_slot = var_qovs;
        *var_qovs_dn0_slot = var_qovs_dn0;
        *var_qovs_dn10_slot = var_qovs_dn10;
        *var_qovs_dn11_slot = var_qovs_dn11;
        *var_qovs_dn12_slot = var_qovs_dn12;
        *var_qovs_dn17_slot = var_qovs_dn17;
        *var_qovs_dn2_slot = var_qovs_dn2;
        *var_qovs_dn6_slot = var_qovs_dn6;
        *var_qovs_dn7_slot = var_qovs_dn7;
        *var_qovs_rv_slot = var_qovs_rv;
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
        *var_qse_rv_slot = var_qse_rv;
        *var_vgvt_slot = var_vgvt;
        *var_vgvt_dn0_slot = var_vgvt_dn0;
        *var_vgvt_dn10_slot = var_vgvt_dn10;
        *var_vgvt_dn11_slot = var_vgvt_dn11;
        *var_vgvt_dn12_slot = var_vgvt_dn12;
        *var_vgvt_dn17_slot = var_vgvt_dn17;
        *var_vgvt_dn2_slot = var_vgvt_dn2;
        *var_vgvt_dn6_slot = var_vgvt_dn6;
        *var_vgvt_dn7_slot = var_vgvt_dn7;
        *var_vgvt_rv_slot = var_vgvt_rv;
        *var_wdsoi_0_slot = var_wdsoi_0;
        *var_wdsoi_0_rv_slot = var_wdsoi_0_rv;
        *var_xd_slot = var_xd;
        *var_xd_dn0_slot = var_xd_dn0;
        *var_xd_dn10_slot = var_xd_dn10;
        *var_xd_dn11_slot = var_xd_dn11;
        *var_xd_dn12_slot = var_xd_dn12;
        *var_xd_dn17_slot = var_xd_dn17;
        *var_xd_dn2_slot = var_xd_dn2;
        *var_xd_dn6_slot = var_xd_dn6;
        *var_xd_dn7_slot = var_xd_dn7;
        *var_xd_rv_slot = var_xd_rv;
    }

    pub(super) fn stamp_reactive_block_1(
        var_ec_slot: &mut f64,
        var_ec_dn0_slot: &mut f64,
        var_ec_dn10_slot: &mut f64,
        var_ec_dn11_slot: &mut f64,
        var_ec_dn12_slot: &mut f64,
        var_ec_dn17_slot: &mut f64,
        var_ec_dn2_slot: &mut f64,
        var_ec_dn6_slot: &mut f64,
        var_ec_dn7_slot: &mut f64,
        var_ec_rv_slot: &mut f64,
        var_flg_depmode_slot: &mut f64,
        var_flg_depmode_rv_slot: &mut f64,
        var_kusai00_slot: &mut f64,
        var_kusai00_dn0_slot: &mut f64,
        var_kusai00_dn10_slot: &mut f64,
        var_kusai00_dn11_slot: &mut f64,
        var_kusai00_dn12_slot: &mut f64,
        var_kusai00_dn17_slot: &mut f64,
        var_kusai00_dn2_slot: &mut f64,
        var_kusai00_dn6_slot: &mut f64,
        var_kusai00_dn7_slot: &mut f64,
        var_kusai00_rv_slot: &mut f64,
        var_kusai00l_slot: &mut f64,
        var_kusai00l_dn0_slot: &mut f64,
        var_kusai00l_dn10_slot: &mut f64,
        var_kusai00l_dn11_slot: &mut f64,
        var_kusai00l_dn12_slot: &mut f64,
        var_kusai00l_dn17_slot: &mut f64,
        var_kusai00l_dn2_slot: &mut f64,
        var_kusai00l_dn6_slot: &mut f64,
        var_kusai00l_dn7_slot: &mut f64,
        var_kusai00l_rv_slot: &mut f64,
        var_kusai_ig_slot: &mut f64,
        var_kusai_ig_dn0_slot: &mut f64,
        var_kusai_ig_dn10_slot: &mut f64,
        var_kusai_ig_dn11_slot: &mut f64,
        var_kusai_ig_dn12_slot: &mut f64,
        var_kusai_ig_dn17_slot: &mut f64,
        var_kusai_ig_dn2_slot: &mut f64,
        var_kusai_ig_dn6_slot: &mut f64,
        var_kusai_ig_dn7_slot: &mut f64,
        var_kusai_ig_rv_slot: &mut f64,
        var_kusail_slot: &mut f64,
        var_kusail_dn0_slot: &mut f64,
        var_kusail_dn10_slot: &mut f64,
        var_kusail_dn11_slot: &mut f64,
        var_kusail_dn12_slot: &mut f64,
        var_kusail_dn17_slot: &mut f64,
        var_kusail_dn2_slot: &mut f64,
        var_kusail_dn6_slot: &mut f64,
        var_kusail_dn7_slot: &mut f64,
        var_kusail_rv_slot: &mut f64,
        var_lred_slot: &mut f64,
        var_lred_dn0_slot: &mut f64,
        var_lred_dn10_slot: &mut f64,
        var_lred_dn11_slot: &mut f64,
        var_lred_dn12_slot: &mut f64,
        var_lred_dn17_slot: &mut f64,
        var_lred_dn2_slot: &mut f64,
        var_lred_dn6_slot: &mut f64,
        var_lred_dn7_slot: &mut f64,
        var_lred_rv_slot: &mut f64,
        var_mud_hoso_slot: &mut f64,
        var_mud_hoso_dn0_slot: &mut f64,
        var_mud_hoso_dn10_slot: &mut f64,
        var_mud_hoso_dn11_slot: &mut f64,
        var_mud_hoso_dn12_slot: &mut f64,
        var_mud_hoso_dn17_slot: &mut f64,
        var_mud_hoso_dn2_slot: &mut f64,
        var_mud_hoso_dn6_slot: &mut f64,
        var_mud_hoso_dn7_slot: &mut f64,
        var_mud_hoso_rv_slot: &mut f64,
        var_phi_b0_soi_slot: &mut f64,
        var_phi_b0_soi_dn0_slot: &mut f64,
        var_phi_b0_soi_dn10_slot: &mut f64,
        var_phi_b0_soi_dn11_slot: &mut f64,
        var_phi_b0_soi_dn12_slot: &mut f64,
        var_phi_b0_soi_dn17_slot: &mut f64,
        var_phi_b0_soi_dn2_slot: &mut f64,
        var_phi_b0_soi_dn6_slot: &mut f64,
        var_phi_b0_soi_dn7_slot: &mut f64,
        var_phi_b0_soi_rv_slot: &mut f64,
        var_phi_b_dep0_slot: &mut f64,
        var_phi_b_dep0_dn0_slot: &mut f64,
        var_phi_b_dep0_dn10_slot: &mut f64,
        var_phi_b_dep0_dn11_slot: &mut f64,
        var_phi_b_dep0_dn12_slot: &mut f64,
        var_phi_b_dep0_dn17_slot: &mut f64,
        var_phi_b_dep0_dn2_slot: &mut f64,
        var_phi_b_dep0_dn6_slot: &mut f64,
        var_phi_b_dep0_dn7_slot: &mut f64,
        var_phi_b_dep0_rv_slot: &mut f64,
        var_phi_bl_soi_slot: &mut f64,
        var_phi_bl_soi_dn0_slot: &mut f64,
        var_phi_bl_soi_dn10_slot: &mut f64,
        var_phi_bl_soi_dn11_slot: &mut f64,
        var_phi_bl_soi_dn12_slot: &mut f64,
        var_phi_bl_soi_dn17_slot: &mut f64,
        var_phi_bl_soi_dn2_slot: &mut f64,
        var_phi_bl_soi_dn6_slot: &mut f64,
        var_phi_bl_soi_dn7_slot: &mut f64,
        var_phi_bl_soi_ini_slot: &mut f64,
        var_phi_bl_soi_ini_dn0_slot: &mut f64,
        var_phi_bl_soi_ini_dn10_slot: &mut f64,
        var_phi_bl_soi_ini_dn11_slot: &mut f64,
        var_phi_bl_soi_ini_dn12_slot: &mut f64,
        var_phi_bl_soi_ini_dn17_slot: &mut f64,
        var_phi_bl_soi_ini_dn2_slot: &mut f64,
        var_phi_bl_soi_ini_dn6_slot: &mut f64,
        var_phi_bl_soi_ini_dn7_slot: &mut f64,
        var_phi_bl_soi_ini_rv_slot: &mut f64,
        var_phi_bl_soi_rv_slot: &mut f64,
        var_phi_s0_bulk_slot: &mut f64,
        var_phi_s0_bulk_dn0_slot: &mut f64,
        var_phi_s0_bulk_dn10_slot: &mut f64,
        var_phi_s0_bulk_dn11_slot: &mut f64,
        var_phi_s0_bulk_dn12_slot: &mut f64,
        var_phi_s0_bulk_dn17_slot: &mut f64,
        var_phi_s0_bulk_dn2_slot: &mut f64,
        var_phi_s0_bulk_dn6_slot: &mut f64,
        var_phi_s0_bulk_dn7_slot: &mut f64,
        var_phi_s0_bulk_rv_slot: &mut f64,
        var_phi_s0_soi_slot: &mut f64,
        var_phi_s0_soi_dn0_slot: &mut f64,
        var_phi_s0_soi_dn10_slot: &mut f64,
        var_phi_s0_soi_dn11_slot: &mut f64,
        var_phi_s0_soi_dn12_slot: &mut f64,
        var_phi_s0_soi_dn17_slot: &mut f64,
        var_phi_s0_soi_dn2_slot: &mut f64,
        var_phi_s0_soi_dn6_slot: &mut f64,
        var_phi_s0_soi_dn7_slot: &mut f64,
        var_phi_s0_soi_rv_slot: &mut f64,
        var_phi_sl_bulk_slot: &mut f64,
        var_phi_sl_bulk_dn0_slot: &mut f64,
        var_phi_sl_bulk_dn10_slot: &mut f64,
        var_phi_sl_bulk_dn11_slot: &mut f64,
        var_phi_sl_bulk_dn12_slot: &mut f64,
        var_phi_sl_bulk_dn17_slot: &mut f64,
        var_phi_sl_bulk_dn2_slot: &mut f64,
        var_phi_sl_bulk_dn6_slot: &mut f64,
        var_phi_sl_bulk_dn7_slot: &mut f64,
        var_phi_sl_bulk_ini_slot: &mut f64,
        var_phi_sl_bulk_ini_dn0_slot: &mut f64,
        var_phi_sl_bulk_ini_dn10_slot: &mut f64,
        var_phi_sl_bulk_ini_dn11_slot: &mut f64,
        var_phi_sl_bulk_ini_dn12_slot: &mut f64,
        var_phi_sl_bulk_ini_dn17_slot: &mut f64,
        var_phi_sl_bulk_ini_dn2_slot: &mut f64,
        var_phi_sl_bulk_ini_dn6_slot: &mut f64,
        var_phi_sl_bulk_ini_dn7_slot: &mut f64,
        var_phi_sl_bulk_ini_rv_slot: &mut f64,
        var_phi_sl_bulk_rv_slot: &mut f64,
        var_phi_sl_soi_slot: &mut f64,
        var_phi_sl_soi_dn0_slot: &mut f64,
        var_phi_sl_soi_dn10_slot: &mut f64,
        var_phi_sl_soi_dn11_slot: &mut f64,
        var_phi_sl_soi_dn12_slot: &mut f64,
        var_phi_sl_soi_dn17_slot: &mut f64,
        var_phi_sl_soi_dn2_slot: &mut f64,
        var_phi_sl_soi_dn6_slot: &mut f64,
        var_phi_sl_soi_dn7_slot: &mut f64,
        var_phi_sl_soi_ini_slot: &mut f64,
        var_phi_sl_soi_ini_dn0_slot: &mut f64,
        var_phi_sl_soi_ini_dn10_slot: &mut f64,
        var_phi_sl_soi_ini_dn11_slot: &mut f64,
        var_phi_sl_soi_ini_dn12_slot: &mut f64,
        var_phi_sl_soi_ini_dn17_slot: &mut f64,
        var_phi_sl_soi_ini_dn2_slot: &mut f64,
        var_phi_sl_soi_ini_dn6_slot: &mut f64,
        var_phi_sl_soi_ini_dn7_slot: &mut f64,
        var_phi_sl_soi_ini_rv_slot: &mut f64,
        var_phi_sl_soi_rv_slot: &mut f64,
        var_ps0_inia_slot: &mut f64,
        var_ps0_inia_dn0_slot: &mut f64,
        var_ps0_inia_dn10_slot: &mut f64,
        var_ps0_inia_dn11_slot: &mut f64,
        var_ps0_inia_dn12_slot: &mut f64,
        var_ps0_inia_dn17_slot: &mut f64,
        var_ps0_inia_dn2_slot: &mut f64,
        var_ps0_inia_dn6_slot: &mut f64,
        var_ps0_inia_dn7_slot: &mut f64,
        var_ps0_inia_rv_slot: &mut f64,
        var_psdl_slot: &mut f64,
        var_psdl_dn0_slot: &mut f64,
        var_psdl_dn10_slot: &mut f64,
        var_psdl_dn11_slot: &mut f64,
        var_psdl_dn12_slot: &mut f64,
        var_psdl_dn17_slot: &mut f64,
        var_psdl_dn2_slot: &mut f64,
        var_psdl_dn6_slot: &mut f64,
        var_psdl_dn7_slot: &mut f64,
        var_psdl_rv_slot: &mut f64,
        var_q_b0_dep_slot: &mut f64,
        var_q_b0_dep_dn0_slot: &mut f64,
        var_q_b0_dep_dn10_slot: &mut f64,
        var_q_b0_dep_dn11_slot: &mut f64,
        var_q_b0_dep_dn12_slot: &mut f64,
        var_q_b0_dep_dn17_slot: &mut f64,
        var_q_b0_dep_dn2_slot: &mut f64,
        var_q_b0_dep_dn6_slot: &mut f64,
        var_q_b0_dep_dn7_slot: &mut f64,
        var_q_b0_dep_rv_slot: &mut f64,
        var_q_bl_dep_slot: &mut f64,
        var_q_bl_dep_dn0_slot: &mut f64,
        var_q_bl_dep_dn10_slot: &mut f64,
        var_q_bl_dep_dn11_slot: &mut f64,
        var_q_bl_dep_dn12_slot: &mut f64,
        var_q_bl_dep_dn17_slot: &mut f64,
        var_q_bl_dep_dn2_slot: &mut f64,
        var_q_bl_dep_dn6_slot: &mut f64,
        var_q_bl_dep_dn7_slot: &mut f64,
        var_q_bl_dep_rv_slot: &mut f64,
        var_q_bt_ge_slot: &mut f64,
        var_q_bt_ge_dn0_slot: &mut f64,
        var_q_bt_ge_dn10_slot: &mut f64,
        var_q_bt_ge_dn11_slot: &mut f64,
        var_q_bt_ge_dn12_slot: &mut f64,
        var_q_bt_ge_dn17_slot: &mut f64,
        var_q_bt_ge_dn2_slot: &mut f64,
        var_q_bt_ge_dn6_slot: &mut f64,
        var_q_bt_ge_dn7_slot: &mut f64,
        var_q_bt_ge_rv_slot: &mut f64,
        var_q_bt_se_slot: &mut f64,
        var_q_bt_se_dn0_slot: &mut f64,
        var_q_bt_se_dn10_slot: &mut f64,
        var_q_bt_se_dn11_slot: &mut f64,
        var_q_bt_se_dn12_slot: &mut f64,
        var_q_bt_se_dn17_slot: &mut f64,
        var_q_bt_se_dn2_slot: &mut f64,
        var_q_bt_se_dn6_slot: &mut f64,
        var_q_bt_se_dn7_slot: &mut f64,
        var_q_bt_se_rv_slot: &mut f64,
        var_q_dep0_slot: &mut f64,
        var_q_dep0_dn0_slot: &mut f64,
        var_q_dep0_dn10_slot: &mut f64,
        var_q_dep0_dn11_slot: &mut f64,
        var_q_dep0_dn12_slot: &mut f64,
        var_q_dep0_dn17_slot: &mut f64,
        var_q_dep0_dn2_slot: &mut f64,
        var_q_dep0_dn6_slot: &mut f64,
        var_q_dep0_dn7_slot: &mut f64,
        var_q_dep0_rv_slot: &mut f64,
        var_q_dep_soi_slot: &mut f64,
        var_q_dep_soi_dn0_slot: &mut f64,
        var_q_dep_soi_dn10_slot: &mut f64,
        var_q_dep_soi_dn11_slot: &mut f64,
        var_q_dep_soi_dn12_slot: &mut f64,
        var_q_dep_soi_dn17_slot: &mut f64,
        var_q_dep_soi_dn2_slot: &mut f64,
        var_q_dep_soi_dn6_slot: &mut f64,
        var_q_dep_soi_dn7_slot: &mut f64,
        var_q_dep_soi_rv_slot: &mut f64,
        var_q_depl_slot: &mut f64,
        var_q_depl_dn0_slot: &mut f64,
        var_q_depl_dn10_slot: &mut f64,
        var_q_depl_dn11_slot: &mut f64,
        var_q_depl_dn12_slot: &mut f64,
        var_q_depl_dn17_slot: &mut f64,
        var_q_depl_dn2_slot: &mut f64,
        var_q_depl_dn6_slot: &mut f64,
        var_q_depl_dn7_slot: &mut f64,
        var_q_depl_rv_slot: &mut f64,
        var_q_n0_slot: &mut f64,
        var_q_n0_dn0_slot: &mut f64,
        var_q_n0_dn10_slot: &mut f64,
        var_q_n0_dn11_slot: &mut f64,
        var_q_n0_dn12_slot: &mut f64,
        var_q_n0_dn17_slot: &mut f64,
        var_q_n0_dn2_slot: &mut f64,
        var_q_n0_dn6_slot: &mut f64,
        var_q_n0_dn7_slot: &mut f64,
        var_q_n0_rv_slot: &mut f64,
        var_q_nl_slot: &mut f64,
        var_q_nl_dn0_slot: &mut f64,
        var_q_nl_dn10_slot: &mut f64,
        var_q_nl_dn11_slot: &mut f64,
        var_q_nl_dn12_slot: &mut f64,
        var_q_nl_dn17_slot: &mut f64,
        var_q_nl_dn2_slot: &mut f64,
        var_q_nl_dn6_slot: &mut f64,
        var_q_nl_dn7_slot: &mut f64,
        var_q_nl_rv_slot: &mut f64,
        var_q_s0_bulk_slot: &mut f64,
        var_q_s0_bulk_0_slot: &mut f64,
        var_q_s0_bulk_0_dn0_slot: &mut f64,
        var_q_s0_bulk_0_dn10_slot: &mut f64,
        var_q_s0_bulk_0_dn11_slot: &mut f64,
        var_q_s0_bulk_0_dn12_slot: &mut f64,
        var_q_s0_bulk_0_dn17_slot: &mut f64,
        var_q_s0_bulk_0_dn2_slot: &mut f64,
        var_q_s0_bulk_0_dn6_slot: &mut f64,
        var_q_s0_bulk_0_dn7_slot: &mut f64,
        var_q_s0_bulk_0_rv_slot: &mut f64,
        var_q_s0_bulk_dn0_slot: &mut f64,
        var_q_s0_bulk_dn10_slot: &mut f64,
        var_q_s0_bulk_dn11_slot: &mut f64,
        var_q_s0_bulk_dn12_slot: &mut f64,
        var_q_s0_bulk_dn17_slot: &mut f64,
        var_q_s0_bulk_dn2_slot: &mut f64,
        var_q_s0_bulk_dn6_slot: &mut f64,
        var_q_s0_bulk_dn7_slot: &mut f64,
        var_q_s0_bulk_rv_slot: &mut f64,
        var_q_sl_bulk_slot: &mut f64,
        var_q_sl_bulk_dn0_slot: &mut f64,
        var_q_sl_bulk_dn10_slot: &mut f64,
        var_q_sl_bulk_dn11_slot: &mut f64,
        var_q_sl_bulk_dn12_slot: &mut f64,
        var_q_sl_bulk_dn17_slot: &mut f64,
        var_q_sl_bulk_dn2_slot: &mut f64,
        var_q_sl_bulk_dn6_slot: &mut f64,
        var_q_sl_bulk_dn7_slot: &mut f64,
        var_q_sl_bulk_rv_slot: &mut f64,
        var_qbu_slot: &mut f64,
        var_qbu_dn0_slot: &mut f64,
        var_qbu_dn10_slot: &mut f64,
        var_qbu_dn11_slot: &mut f64,
        var_qbu_dn12_slot: &mut f64,
        var_qbu_dn17_slot: &mut f64,
        var_qbu_dn2_slot: &mut f64,
        var_qbu_dn6_slot: &mut f64,
        var_qbu_dn7_slot: &mut f64,
        var_qbu_rv_slot: &mut f64,
        var_qd_fb_slot: &mut f64,
        var_qd_fb_dn0_slot: &mut f64,
        var_qd_fb_dn10_slot: &mut f64,
        var_qd_fb_dn11_slot: &mut f64,
        var_qd_fb_dn12_slot: &mut f64,
        var_qd_fb_dn13_slot: &mut f64,
        var_qd_fb_dn15_slot: &mut f64,
        var_qd_fb_dn16_slot: &mut f64,
        var_qd_fb_dn17_slot: &mut f64,
        var_qd_fb_dn18_slot: &mut f64,
        var_qd_fb_dn2_slot: &mut f64,
        var_qd_fb_dn6_slot: &mut f64,
        var_qd_fb_dn7_slot: &mut f64,
        var_qd_fb_rv_slot: &mut f64,
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
        var_qd_nqs_rv_slot: &mut f64,
        var_qdrat_slot: &mut f64,
        var_qdrat_dn0_slot: &mut f64,
        var_qdrat_dn10_slot: &mut f64,
        var_qdrat_dn11_slot: &mut f64,
        var_qdrat_dn12_slot: &mut f64,
        var_qdrat_dn17_slot: &mut f64,
        var_qdrat_dn2_slot: &mut f64,
        var_qdrat_dn6_slot: &mut f64,
        var_qdrat_dn7_slot: &mut f64,
        var_qdrat_noi_slot: &mut f64,
        var_qdrat_noi_dn0_slot: &mut f64,
        var_qdrat_noi_dn10_slot: &mut f64,
        var_qdrat_noi_dn11_slot: &mut f64,
        var_qdrat_noi_dn12_slot: &mut f64,
        var_qdrat_noi_dn17_slot: &mut f64,
        var_qdrat_noi_dn2_slot: &mut f64,
        var_qdrat_noi_dn6_slot: &mut f64,
        var_qdrat_noi_dn7_slot: &mut f64,
        var_qdrat_noi_rv_slot: &mut f64,
        var_qdrat_rv_slot: &mut f64,
        var_qhs_slot: &mut f64,
        var_qhs_dn0_slot: &mut f64,
        var_qhs_dn10_slot: &mut f64,
        var_qhs_dn11_slot: &mut f64,
        var_qhs_dn12_slot: &mut f64,
        var_qhs_dn17_slot: &mut f64,
        var_qhs_dn2_slot: &mut f64,
        var_qhs_dn6_slot: &mut f64,
        var_qhs_dn7_slot: &mut f64,
        var_qhs_rv_slot: &mut f64,
        var_qi_nqs_slot: &mut f64,
        var_qi_nqs_dn18_slot: &mut f64,
        var_qi_nqs_rv_slot: &mut f64,
        var_qiu_slot: &mut f64,
        var_qiu_dn0_slot: &mut f64,
        var_qiu_dn10_slot: &mut f64,
        var_qiu_dn11_slot: &mut f64,
        var_qiu_dn12_slot: &mut f64,
        var_qiu_dn17_slot: &mut f64,
        var_qiu_dn2_slot: &mut f64,
        var_qiu_dn6_slot: &mut f64,
        var_qiu_dn7_slot: &mut f64,
        var_qiu_rv_slot: &mut f64,
        var_qs_fb_slot: &mut f64,
        var_qs_fb_dn0_slot: &mut f64,
        var_qs_fb_dn10_slot: &mut f64,
        var_qs_fb_dn11_slot: &mut f64,
        var_qs_fb_dn12_slot: &mut f64,
        var_qs_fb_dn13_slot: &mut f64,
        var_qs_fb_dn15_slot: &mut f64,
        var_qs_fb_dn16_slot: &mut f64,
        var_qs_fb_dn17_slot: &mut f64,
        var_qs_fb_dn18_slot: &mut f64,
        var_qs_fb_dn2_slot: &mut f64,
        var_qs_fb_dn6_slot: &mut f64,
        var_qs_fb_dn7_slot: &mut f64,
        var_qs_fb_rv_slot: &mut f64,
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
        var_qs_nqs_rv_slot: &mut f64,
        var_qsub_slot: &mut f64,
        var_qsub_dn0_slot: &mut f64,
        var_qsub_dn10_slot: &mut f64,
        var_qsub_dn11_slot: &mut f64,
        var_qsub_dn12_slot: &mut f64,
        var_qsub_dn17_slot: &mut f64,
        var_qsub_dn2_slot: &mut f64,
        var_qsub_dn6_slot: &mut f64,
        var_qsub_dn7_slot: &mut f64,
        var_qsub_rv_slot: &mut f64,
        var_shift_slot: &mut f64,
        var_shift_dn0_slot: &mut f64,
        var_shift_dn10_slot: &mut f64,
        var_shift_dn11_slot: &mut f64,
        var_shift_dn12_slot: &mut f64,
        var_shift_dn17_slot: &mut f64,
        var_shift_dn2_slot: &mut f64,
        var_shift_dn6_slot: &mut f64,
        var_shift_dn7_slot: &mut f64,
        var_shift_rv_slot: &mut f64,
        var_sqrtkusail_slot: &mut f64,
        var_sqrtkusail_dn0_slot: &mut f64,
        var_sqrtkusail_dn10_slot: &mut f64,
        var_sqrtkusail_dn11_slot: &mut f64,
        var_sqrtkusail_dn12_slot: &mut f64,
        var_sqrtkusail_dn17_slot: &mut f64,
        var_sqrtkusail_dn2_slot: &mut f64,
        var_sqrtkusail_dn6_slot: &mut f64,
        var_sqrtkusail_dn7_slot: &mut f64,
        var_sqrtkusail_rv_slot: &mut f64,
        var_uc_areabt_slot: &mut f64,
        var_uc_areabt_rv_slot: &mut f64,
        var_uc_vfbbt_slot: &mut f64,
        var_uc_vfbbt_rv_slot: &mut f64,
        var_wdsoi_slot: &mut f64,
        var_wdsoi_dn0_slot: &mut f64,
        var_wdsoi_dn10_slot: &mut f64,
        var_wdsoi_dn11_slot: &mut f64,
        var_wdsoi_dn12_slot: &mut f64,
        var_wdsoi_dn17_slot: &mut f64,
        var_wdsoi_dn2_slot: &mut f64,
        var_wdsoi_dn6_slot: &mut f64,
        var_wdsoi_dn7_slot: &mut f64,
        var_wdsoi_rv_slot: &mut f64,
    ) {
        let mut var_ec: f64 = *var_ec_slot;
        let mut var_ec_dn0: f64 = *var_ec_dn0_slot;
        let mut var_ec_dn10: f64 = *var_ec_dn10_slot;
        let mut var_ec_dn11: f64 = *var_ec_dn11_slot;
        let mut var_ec_dn12: f64 = *var_ec_dn12_slot;
        let mut var_ec_dn17: f64 = *var_ec_dn17_slot;
        let mut var_ec_dn2: f64 = *var_ec_dn2_slot;
        let mut var_ec_dn6: f64 = *var_ec_dn6_slot;
        let mut var_ec_dn7: f64 = *var_ec_dn7_slot;
        let mut var_ec_rv: f64 = *var_ec_rv_slot;
        let mut var_flg_depmode: f64 = *var_flg_depmode_slot;
        let mut var_flg_depmode_rv: f64 = *var_flg_depmode_rv_slot;
        let mut var_kusai00: f64 = *var_kusai00_slot;
        let mut var_kusai00_dn0: f64 = *var_kusai00_dn0_slot;
        let mut var_kusai00_dn10: f64 = *var_kusai00_dn10_slot;
        let mut var_kusai00_dn11: f64 = *var_kusai00_dn11_slot;
        let mut var_kusai00_dn12: f64 = *var_kusai00_dn12_slot;
        let mut var_kusai00_dn17: f64 = *var_kusai00_dn17_slot;
        let mut var_kusai00_dn2: f64 = *var_kusai00_dn2_slot;
        let mut var_kusai00_dn6: f64 = *var_kusai00_dn6_slot;
        let mut var_kusai00_dn7: f64 = *var_kusai00_dn7_slot;
        let mut var_kusai00_rv: f64 = *var_kusai00_rv_slot;
        let mut var_kusai00l: f64 = *var_kusai00l_slot;
        let mut var_kusai00l_dn0: f64 = *var_kusai00l_dn0_slot;
        let mut var_kusai00l_dn10: f64 = *var_kusai00l_dn10_slot;
        let mut var_kusai00l_dn11: f64 = *var_kusai00l_dn11_slot;
        let mut var_kusai00l_dn12: f64 = *var_kusai00l_dn12_slot;
        let mut var_kusai00l_dn17: f64 = *var_kusai00l_dn17_slot;
        let mut var_kusai00l_dn2: f64 = *var_kusai00l_dn2_slot;
        let mut var_kusai00l_dn6: f64 = *var_kusai00l_dn6_slot;
        let mut var_kusai00l_dn7: f64 = *var_kusai00l_dn7_slot;
        let mut var_kusai00l_rv: f64 = *var_kusai00l_rv_slot;
        let mut var_kusai_ig: f64 = *var_kusai_ig_slot;
        let mut var_kusai_ig_dn0: f64 = *var_kusai_ig_dn0_slot;
        let mut var_kusai_ig_dn10: f64 = *var_kusai_ig_dn10_slot;
        let mut var_kusai_ig_dn11: f64 = *var_kusai_ig_dn11_slot;
        let mut var_kusai_ig_dn12: f64 = *var_kusai_ig_dn12_slot;
        let mut var_kusai_ig_dn17: f64 = *var_kusai_ig_dn17_slot;
        let mut var_kusai_ig_dn2: f64 = *var_kusai_ig_dn2_slot;
        let mut var_kusai_ig_dn6: f64 = *var_kusai_ig_dn6_slot;
        let mut var_kusai_ig_dn7: f64 = *var_kusai_ig_dn7_slot;
        let mut var_kusai_ig_rv: f64 = *var_kusai_ig_rv_slot;
        let mut var_kusail: f64 = *var_kusail_slot;
        let mut var_kusail_dn0: f64 = *var_kusail_dn0_slot;
        let mut var_kusail_dn10: f64 = *var_kusail_dn10_slot;
        let mut var_kusail_dn11: f64 = *var_kusail_dn11_slot;
        let mut var_kusail_dn12: f64 = *var_kusail_dn12_slot;
        let mut var_kusail_dn17: f64 = *var_kusail_dn17_slot;
        let mut var_kusail_dn2: f64 = *var_kusail_dn2_slot;
        let mut var_kusail_dn6: f64 = *var_kusail_dn6_slot;
        let mut var_kusail_dn7: f64 = *var_kusail_dn7_slot;
        let mut var_kusail_rv: f64 = *var_kusail_rv_slot;
        let mut var_lred: f64 = *var_lred_slot;
        let mut var_lred_dn0: f64 = *var_lred_dn0_slot;
        let mut var_lred_dn10: f64 = *var_lred_dn10_slot;
        let mut var_lred_dn11: f64 = *var_lred_dn11_slot;
        let mut var_lred_dn12: f64 = *var_lred_dn12_slot;
        let mut var_lred_dn17: f64 = *var_lred_dn17_slot;
        let mut var_lred_dn2: f64 = *var_lred_dn2_slot;
        let mut var_lred_dn6: f64 = *var_lred_dn6_slot;
        let mut var_lred_dn7: f64 = *var_lred_dn7_slot;
        let mut var_lred_rv: f64 = *var_lred_rv_slot;
        let mut var_mud_hoso: f64 = *var_mud_hoso_slot;
        let mut var_mud_hoso_dn0: f64 = *var_mud_hoso_dn0_slot;
        let mut var_mud_hoso_dn10: f64 = *var_mud_hoso_dn10_slot;
        let mut var_mud_hoso_dn11: f64 = *var_mud_hoso_dn11_slot;
        let mut var_mud_hoso_dn12: f64 = *var_mud_hoso_dn12_slot;
        let mut var_mud_hoso_dn17: f64 = *var_mud_hoso_dn17_slot;
        let mut var_mud_hoso_dn2: f64 = *var_mud_hoso_dn2_slot;
        let mut var_mud_hoso_dn6: f64 = *var_mud_hoso_dn6_slot;
        let mut var_mud_hoso_dn7: f64 = *var_mud_hoso_dn7_slot;
        let mut var_mud_hoso_rv: f64 = *var_mud_hoso_rv_slot;
        let mut var_phi_b0_soi: f64 = *var_phi_b0_soi_slot;
        let mut var_phi_b0_soi_dn0: f64 = *var_phi_b0_soi_dn0_slot;
        let mut var_phi_b0_soi_dn10: f64 = *var_phi_b0_soi_dn10_slot;
        let mut var_phi_b0_soi_dn11: f64 = *var_phi_b0_soi_dn11_slot;
        let mut var_phi_b0_soi_dn12: f64 = *var_phi_b0_soi_dn12_slot;
        let mut var_phi_b0_soi_dn17: f64 = *var_phi_b0_soi_dn17_slot;
        let mut var_phi_b0_soi_dn2: f64 = *var_phi_b0_soi_dn2_slot;
        let mut var_phi_b0_soi_dn6: f64 = *var_phi_b0_soi_dn6_slot;
        let mut var_phi_b0_soi_dn7: f64 = *var_phi_b0_soi_dn7_slot;
        let mut var_phi_b0_soi_rv: f64 = *var_phi_b0_soi_rv_slot;
        let mut var_phi_b_dep0: f64 = *var_phi_b_dep0_slot;
        let mut var_phi_b_dep0_dn0: f64 = *var_phi_b_dep0_dn0_slot;
        let mut var_phi_b_dep0_dn10: f64 = *var_phi_b_dep0_dn10_slot;
        let mut var_phi_b_dep0_dn11: f64 = *var_phi_b_dep0_dn11_slot;
        let mut var_phi_b_dep0_dn12: f64 = *var_phi_b_dep0_dn12_slot;
        let mut var_phi_b_dep0_dn17: f64 = *var_phi_b_dep0_dn17_slot;
        let mut var_phi_b_dep0_dn2: f64 = *var_phi_b_dep0_dn2_slot;
        let mut var_phi_b_dep0_dn6: f64 = *var_phi_b_dep0_dn6_slot;
        let mut var_phi_b_dep0_dn7: f64 = *var_phi_b_dep0_dn7_slot;
        let mut var_phi_b_dep0_rv: f64 = *var_phi_b_dep0_rv_slot;
        let mut var_phi_bl_soi: f64 = *var_phi_bl_soi_slot;
        let mut var_phi_bl_soi_dn0: f64 = *var_phi_bl_soi_dn0_slot;
        let mut var_phi_bl_soi_dn10: f64 = *var_phi_bl_soi_dn10_slot;
        let mut var_phi_bl_soi_dn11: f64 = *var_phi_bl_soi_dn11_slot;
        let mut var_phi_bl_soi_dn12: f64 = *var_phi_bl_soi_dn12_slot;
        let mut var_phi_bl_soi_dn17: f64 = *var_phi_bl_soi_dn17_slot;
        let mut var_phi_bl_soi_dn2: f64 = *var_phi_bl_soi_dn2_slot;
        let mut var_phi_bl_soi_dn6: f64 = *var_phi_bl_soi_dn6_slot;
        let mut var_phi_bl_soi_dn7: f64 = *var_phi_bl_soi_dn7_slot;
        let mut var_phi_bl_soi_ini: f64 = *var_phi_bl_soi_ini_slot;
        let mut var_phi_bl_soi_ini_dn0: f64 = *var_phi_bl_soi_ini_dn0_slot;
        let mut var_phi_bl_soi_ini_dn10: f64 = *var_phi_bl_soi_ini_dn10_slot;
        let mut var_phi_bl_soi_ini_dn11: f64 = *var_phi_bl_soi_ini_dn11_slot;
        let mut var_phi_bl_soi_ini_dn12: f64 = *var_phi_bl_soi_ini_dn12_slot;
        let mut var_phi_bl_soi_ini_dn17: f64 = *var_phi_bl_soi_ini_dn17_slot;
        let mut var_phi_bl_soi_ini_dn2: f64 = *var_phi_bl_soi_ini_dn2_slot;
        let mut var_phi_bl_soi_ini_dn6: f64 = *var_phi_bl_soi_ini_dn6_slot;
        let mut var_phi_bl_soi_ini_dn7: f64 = *var_phi_bl_soi_ini_dn7_slot;
        let mut var_phi_bl_soi_ini_rv: f64 = *var_phi_bl_soi_ini_rv_slot;
        let mut var_phi_bl_soi_rv: f64 = *var_phi_bl_soi_rv_slot;
        let mut var_phi_s0_bulk: f64 = *var_phi_s0_bulk_slot;
        let mut var_phi_s0_bulk_dn0: f64 = *var_phi_s0_bulk_dn0_slot;
        let mut var_phi_s0_bulk_dn10: f64 = *var_phi_s0_bulk_dn10_slot;
        let mut var_phi_s0_bulk_dn11: f64 = *var_phi_s0_bulk_dn11_slot;
        let mut var_phi_s0_bulk_dn12: f64 = *var_phi_s0_bulk_dn12_slot;
        let mut var_phi_s0_bulk_dn17: f64 = *var_phi_s0_bulk_dn17_slot;
        let mut var_phi_s0_bulk_dn2: f64 = *var_phi_s0_bulk_dn2_slot;
        let mut var_phi_s0_bulk_dn6: f64 = *var_phi_s0_bulk_dn6_slot;
        let mut var_phi_s0_bulk_dn7: f64 = *var_phi_s0_bulk_dn7_slot;
        let mut var_phi_s0_bulk_rv: f64 = *var_phi_s0_bulk_rv_slot;
        let mut var_phi_s0_soi: f64 = *var_phi_s0_soi_slot;
        let mut var_phi_s0_soi_dn0: f64 = *var_phi_s0_soi_dn0_slot;
        let mut var_phi_s0_soi_dn10: f64 = *var_phi_s0_soi_dn10_slot;
        let mut var_phi_s0_soi_dn11: f64 = *var_phi_s0_soi_dn11_slot;
        let mut var_phi_s0_soi_dn12: f64 = *var_phi_s0_soi_dn12_slot;
        let mut var_phi_s0_soi_dn17: f64 = *var_phi_s0_soi_dn17_slot;
        let mut var_phi_s0_soi_dn2: f64 = *var_phi_s0_soi_dn2_slot;
        let mut var_phi_s0_soi_dn6: f64 = *var_phi_s0_soi_dn6_slot;
        let mut var_phi_s0_soi_dn7: f64 = *var_phi_s0_soi_dn7_slot;
        let mut var_phi_s0_soi_rv: f64 = *var_phi_s0_soi_rv_slot;
        let mut var_phi_sl_bulk: f64 = *var_phi_sl_bulk_slot;
        let mut var_phi_sl_bulk_dn0: f64 = *var_phi_sl_bulk_dn0_slot;
        let mut var_phi_sl_bulk_dn10: f64 = *var_phi_sl_bulk_dn10_slot;
        let mut var_phi_sl_bulk_dn11: f64 = *var_phi_sl_bulk_dn11_slot;
        let mut var_phi_sl_bulk_dn12: f64 = *var_phi_sl_bulk_dn12_slot;
        let mut var_phi_sl_bulk_dn17: f64 = *var_phi_sl_bulk_dn17_slot;
        let mut var_phi_sl_bulk_dn2: f64 = *var_phi_sl_bulk_dn2_slot;
        let mut var_phi_sl_bulk_dn6: f64 = *var_phi_sl_bulk_dn6_slot;
        let mut var_phi_sl_bulk_dn7: f64 = *var_phi_sl_bulk_dn7_slot;
        let mut var_phi_sl_bulk_ini: f64 = *var_phi_sl_bulk_ini_slot;
        let mut var_phi_sl_bulk_ini_dn0: f64 = *var_phi_sl_bulk_ini_dn0_slot;
        let mut var_phi_sl_bulk_ini_dn10: f64 = *var_phi_sl_bulk_ini_dn10_slot;
        let mut var_phi_sl_bulk_ini_dn11: f64 = *var_phi_sl_bulk_ini_dn11_slot;
        let mut var_phi_sl_bulk_ini_dn12: f64 = *var_phi_sl_bulk_ini_dn12_slot;
        let mut var_phi_sl_bulk_ini_dn17: f64 = *var_phi_sl_bulk_ini_dn17_slot;
        let mut var_phi_sl_bulk_ini_dn2: f64 = *var_phi_sl_bulk_ini_dn2_slot;
        let mut var_phi_sl_bulk_ini_dn6: f64 = *var_phi_sl_bulk_ini_dn6_slot;
        let mut var_phi_sl_bulk_ini_dn7: f64 = *var_phi_sl_bulk_ini_dn7_slot;
        let mut var_phi_sl_bulk_ini_rv: f64 = *var_phi_sl_bulk_ini_rv_slot;
        let mut var_phi_sl_bulk_rv: f64 = *var_phi_sl_bulk_rv_slot;
        let mut var_phi_sl_soi: f64 = *var_phi_sl_soi_slot;
        let mut var_phi_sl_soi_dn0: f64 = *var_phi_sl_soi_dn0_slot;
        let mut var_phi_sl_soi_dn10: f64 = *var_phi_sl_soi_dn10_slot;
        let mut var_phi_sl_soi_dn11: f64 = *var_phi_sl_soi_dn11_slot;
        let mut var_phi_sl_soi_dn12: f64 = *var_phi_sl_soi_dn12_slot;
        let mut var_phi_sl_soi_dn17: f64 = *var_phi_sl_soi_dn17_slot;
        let mut var_phi_sl_soi_dn2: f64 = *var_phi_sl_soi_dn2_slot;
        let mut var_phi_sl_soi_dn6: f64 = *var_phi_sl_soi_dn6_slot;
        let mut var_phi_sl_soi_dn7: f64 = *var_phi_sl_soi_dn7_slot;
        let mut var_phi_sl_soi_ini: f64 = *var_phi_sl_soi_ini_slot;
        let mut var_phi_sl_soi_ini_dn0: f64 = *var_phi_sl_soi_ini_dn0_slot;
        let mut var_phi_sl_soi_ini_dn10: f64 = *var_phi_sl_soi_ini_dn10_slot;
        let mut var_phi_sl_soi_ini_dn11: f64 = *var_phi_sl_soi_ini_dn11_slot;
        let mut var_phi_sl_soi_ini_dn12: f64 = *var_phi_sl_soi_ini_dn12_slot;
        let mut var_phi_sl_soi_ini_dn17: f64 = *var_phi_sl_soi_ini_dn17_slot;
        let mut var_phi_sl_soi_ini_dn2: f64 = *var_phi_sl_soi_ini_dn2_slot;
        let mut var_phi_sl_soi_ini_dn6: f64 = *var_phi_sl_soi_ini_dn6_slot;
        let mut var_phi_sl_soi_ini_dn7: f64 = *var_phi_sl_soi_ini_dn7_slot;
        let mut var_phi_sl_soi_ini_rv: f64 = *var_phi_sl_soi_ini_rv_slot;
        let mut var_phi_sl_soi_rv: f64 = *var_phi_sl_soi_rv_slot;
        let mut var_ps0_inia: f64 = *var_ps0_inia_slot;
        let mut var_ps0_inia_dn0: f64 = *var_ps0_inia_dn0_slot;
        let mut var_ps0_inia_dn10: f64 = *var_ps0_inia_dn10_slot;
        let mut var_ps0_inia_dn11: f64 = *var_ps0_inia_dn11_slot;
        let mut var_ps0_inia_dn12: f64 = *var_ps0_inia_dn12_slot;
        let mut var_ps0_inia_dn17: f64 = *var_ps0_inia_dn17_slot;
        let mut var_ps0_inia_dn2: f64 = *var_ps0_inia_dn2_slot;
        let mut var_ps0_inia_dn6: f64 = *var_ps0_inia_dn6_slot;
        let mut var_ps0_inia_dn7: f64 = *var_ps0_inia_dn7_slot;
        let mut var_ps0_inia_rv: f64 = *var_ps0_inia_rv_slot;
        let mut var_psdl: f64 = *var_psdl_slot;
        let mut var_psdl_dn0: f64 = *var_psdl_dn0_slot;
        let mut var_psdl_dn10: f64 = *var_psdl_dn10_slot;
        let mut var_psdl_dn11: f64 = *var_psdl_dn11_slot;
        let mut var_psdl_dn12: f64 = *var_psdl_dn12_slot;
        let mut var_psdl_dn17: f64 = *var_psdl_dn17_slot;
        let mut var_psdl_dn2: f64 = *var_psdl_dn2_slot;
        let mut var_psdl_dn6: f64 = *var_psdl_dn6_slot;
        let mut var_psdl_dn7: f64 = *var_psdl_dn7_slot;
        let mut var_psdl_rv: f64 = *var_psdl_rv_slot;
        let mut var_q_b0_dep: f64 = *var_q_b0_dep_slot;
        let mut var_q_b0_dep_dn0: f64 = *var_q_b0_dep_dn0_slot;
        let mut var_q_b0_dep_dn10: f64 = *var_q_b0_dep_dn10_slot;
        let mut var_q_b0_dep_dn11: f64 = *var_q_b0_dep_dn11_slot;
        let mut var_q_b0_dep_dn12: f64 = *var_q_b0_dep_dn12_slot;
        let mut var_q_b0_dep_dn17: f64 = *var_q_b0_dep_dn17_slot;
        let mut var_q_b0_dep_dn2: f64 = *var_q_b0_dep_dn2_slot;
        let mut var_q_b0_dep_dn6: f64 = *var_q_b0_dep_dn6_slot;
        let mut var_q_b0_dep_dn7: f64 = *var_q_b0_dep_dn7_slot;
        let mut var_q_b0_dep_rv: f64 = *var_q_b0_dep_rv_slot;
        let mut var_q_bl_dep: f64 = *var_q_bl_dep_slot;
        let mut var_q_bl_dep_dn0: f64 = *var_q_bl_dep_dn0_slot;
        let mut var_q_bl_dep_dn10: f64 = *var_q_bl_dep_dn10_slot;
        let mut var_q_bl_dep_dn11: f64 = *var_q_bl_dep_dn11_slot;
        let mut var_q_bl_dep_dn12: f64 = *var_q_bl_dep_dn12_slot;
        let mut var_q_bl_dep_dn17: f64 = *var_q_bl_dep_dn17_slot;
        let mut var_q_bl_dep_dn2: f64 = *var_q_bl_dep_dn2_slot;
        let mut var_q_bl_dep_dn6: f64 = *var_q_bl_dep_dn6_slot;
        let mut var_q_bl_dep_dn7: f64 = *var_q_bl_dep_dn7_slot;
        let mut var_q_bl_dep_rv: f64 = *var_q_bl_dep_rv_slot;
        let mut var_q_bt_ge: f64 = *var_q_bt_ge_slot;
        let mut var_q_bt_ge_dn0: f64 = *var_q_bt_ge_dn0_slot;
        let mut var_q_bt_ge_dn10: f64 = *var_q_bt_ge_dn10_slot;
        let mut var_q_bt_ge_dn11: f64 = *var_q_bt_ge_dn11_slot;
        let mut var_q_bt_ge_dn12: f64 = *var_q_bt_ge_dn12_slot;
        let mut var_q_bt_ge_dn17: f64 = *var_q_bt_ge_dn17_slot;
        let mut var_q_bt_ge_dn2: f64 = *var_q_bt_ge_dn2_slot;
        let mut var_q_bt_ge_dn6: f64 = *var_q_bt_ge_dn6_slot;
        let mut var_q_bt_ge_dn7: f64 = *var_q_bt_ge_dn7_slot;
        let mut var_q_bt_ge_rv: f64 = *var_q_bt_ge_rv_slot;
        let mut var_q_bt_se: f64 = *var_q_bt_se_slot;
        let mut var_q_bt_se_dn0: f64 = *var_q_bt_se_dn0_slot;
        let mut var_q_bt_se_dn10: f64 = *var_q_bt_se_dn10_slot;
        let mut var_q_bt_se_dn11: f64 = *var_q_bt_se_dn11_slot;
        let mut var_q_bt_se_dn12: f64 = *var_q_bt_se_dn12_slot;
        let mut var_q_bt_se_dn17: f64 = *var_q_bt_se_dn17_slot;
        let mut var_q_bt_se_dn2: f64 = *var_q_bt_se_dn2_slot;
        let mut var_q_bt_se_dn6: f64 = *var_q_bt_se_dn6_slot;
        let mut var_q_bt_se_dn7: f64 = *var_q_bt_se_dn7_slot;
        let mut var_q_bt_se_rv: f64 = *var_q_bt_se_rv_slot;
        let mut var_q_dep0: f64 = *var_q_dep0_slot;
        let mut var_q_dep0_dn0: f64 = *var_q_dep0_dn0_slot;
        let mut var_q_dep0_dn10: f64 = *var_q_dep0_dn10_slot;
        let mut var_q_dep0_dn11: f64 = *var_q_dep0_dn11_slot;
        let mut var_q_dep0_dn12: f64 = *var_q_dep0_dn12_slot;
        let mut var_q_dep0_dn17: f64 = *var_q_dep0_dn17_slot;
        let mut var_q_dep0_dn2: f64 = *var_q_dep0_dn2_slot;
        let mut var_q_dep0_dn6: f64 = *var_q_dep0_dn6_slot;
        let mut var_q_dep0_dn7: f64 = *var_q_dep0_dn7_slot;
        let mut var_q_dep0_rv: f64 = *var_q_dep0_rv_slot;
        let mut var_q_dep_soi: f64 = *var_q_dep_soi_slot;
        let mut var_q_dep_soi_dn0: f64 = *var_q_dep_soi_dn0_slot;
        let mut var_q_dep_soi_dn10: f64 = *var_q_dep_soi_dn10_slot;
        let mut var_q_dep_soi_dn11: f64 = *var_q_dep_soi_dn11_slot;
        let mut var_q_dep_soi_dn12: f64 = *var_q_dep_soi_dn12_slot;
        let mut var_q_dep_soi_dn17: f64 = *var_q_dep_soi_dn17_slot;
        let mut var_q_dep_soi_dn2: f64 = *var_q_dep_soi_dn2_slot;
        let mut var_q_dep_soi_dn6: f64 = *var_q_dep_soi_dn6_slot;
        let mut var_q_dep_soi_dn7: f64 = *var_q_dep_soi_dn7_slot;
        let mut var_q_dep_soi_rv: f64 = *var_q_dep_soi_rv_slot;
        let mut var_q_depl: f64 = *var_q_depl_slot;
        let mut var_q_depl_dn0: f64 = *var_q_depl_dn0_slot;
        let mut var_q_depl_dn10: f64 = *var_q_depl_dn10_slot;
        let mut var_q_depl_dn11: f64 = *var_q_depl_dn11_slot;
        let mut var_q_depl_dn12: f64 = *var_q_depl_dn12_slot;
        let mut var_q_depl_dn17: f64 = *var_q_depl_dn17_slot;
        let mut var_q_depl_dn2: f64 = *var_q_depl_dn2_slot;
        let mut var_q_depl_dn6: f64 = *var_q_depl_dn6_slot;
        let mut var_q_depl_dn7: f64 = *var_q_depl_dn7_slot;
        let mut var_q_depl_rv: f64 = *var_q_depl_rv_slot;
        let mut var_q_n0: f64 = *var_q_n0_slot;
        let mut var_q_n0_dn0: f64 = *var_q_n0_dn0_slot;
        let mut var_q_n0_dn10: f64 = *var_q_n0_dn10_slot;
        let mut var_q_n0_dn11: f64 = *var_q_n0_dn11_slot;
        let mut var_q_n0_dn12: f64 = *var_q_n0_dn12_slot;
        let mut var_q_n0_dn17: f64 = *var_q_n0_dn17_slot;
        let mut var_q_n0_dn2: f64 = *var_q_n0_dn2_slot;
        let mut var_q_n0_dn6: f64 = *var_q_n0_dn6_slot;
        let mut var_q_n0_dn7: f64 = *var_q_n0_dn7_slot;
        let mut var_q_n0_rv: f64 = *var_q_n0_rv_slot;
        let mut var_q_nl: f64 = *var_q_nl_slot;
        let mut var_q_nl_dn0: f64 = *var_q_nl_dn0_slot;
        let mut var_q_nl_dn10: f64 = *var_q_nl_dn10_slot;
        let mut var_q_nl_dn11: f64 = *var_q_nl_dn11_slot;
        let mut var_q_nl_dn12: f64 = *var_q_nl_dn12_slot;
        let mut var_q_nl_dn17: f64 = *var_q_nl_dn17_slot;
        let mut var_q_nl_dn2: f64 = *var_q_nl_dn2_slot;
        let mut var_q_nl_dn6: f64 = *var_q_nl_dn6_slot;
        let mut var_q_nl_dn7: f64 = *var_q_nl_dn7_slot;
        let mut var_q_nl_rv: f64 = *var_q_nl_rv_slot;
        let mut var_q_s0_bulk: f64 = *var_q_s0_bulk_slot;
        let mut var_q_s0_bulk_0: f64 = *var_q_s0_bulk_0_slot;
        let mut var_q_s0_bulk_0_dn0: f64 = *var_q_s0_bulk_0_dn0_slot;
        let mut var_q_s0_bulk_0_dn10: f64 = *var_q_s0_bulk_0_dn10_slot;
        let mut var_q_s0_bulk_0_dn11: f64 = *var_q_s0_bulk_0_dn11_slot;
        let mut var_q_s0_bulk_0_dn12: f64 = *var_q_s0_bulk_0_dn12_slot;
        let mut var_q_s0_bulk_0_dn17: f64 = *var_q_s0_bulk_0_dn17_slot;
        let mut var_q_s0_bulk_0_dn2: f64 = *var_q_s0_bulk_0_dn2_slot;
        let mut var_q_s0_bulk_0_dn6: f64 = *var_q_s0_bulk_0_dn6_slot;
        let mut var_q_s0_bulk_0_dn7: f64 = *var_q_s0_bulk_0_dn7_slot;
        let mut var_q_s0_bulk_0_rv: f64 = *var_q_s0_bulk_0_rv_slot;
        let mut var_q_s0_bulk_dn0: f64 = *var_q_s0_bulk_dn0_slot;
        let mut var_q_s0_bulk_dn10: f64 = *var_q_s0_bulk_dn10_slot;
        let mut var_q_s0_bulk_dn11: f64 = *var_q_s0_bulk_dn11_slot;
        let mut var_q_s0_bulk_dn12: f64 = *var_q_s0_bulk_dn12_slot;
        let mut var_q_s0_bulk_dn17: f64 = *var_q_s0_bulk_dn17_slot;
        let mut var_q_s0_bulk_dn2: f64 = *var_q_s0_bulk_dn2_slot;
        let mut var_q_s0_bulk_dn6: f64 = *var_q_s0_bulk_dn6_slot;
        let mut var_q_s0_bulk_dn7: f64 = *var_q_s0_bulk_dn7_slot;
        let mut var_q_s0_bulk_rv: f64 = *var_q_s0_bulk_rv_slot;
        let mut var_q_sl_bulk: f64 = *var_q_sl_bulk_slot;
        let mut var_q_sl_bulk_dn0: f64 = *var_q_sl_bulk_dn0_slot;
        let mut var_q_sl_bulk_dn10: f64 = *var_q_sl_bulk_dn10_slot;
        let mut var_q_sl_bulk_dn11: f64 = *var_q_sl_bulk_dn11_slot;
        let mut var_q_sl_bulk_dn12: f64 = *var_q_sl_bulk_dn12_slot;
        let mut var_q_sl_bulk_dn17: f64 = *var_q_sl_bulk_dn17_slot;
        let mut var_q_sl_bulk_dn2: f64 = *var_q_sl_bulk_dn2_slot;
        let mut var_q_sl_bulk_dn6: f64 = *var_q_sl_bulk_dn6_slot;
        let mut var_q_sl_bulk_dn7: f64 = *var_q_sl_bulk_dn7_slot;
        let mut var_q_sl_bulk_rv: f64 = *var_q_sl_bulk_rv_slot;
        let mut var_qbu: f64 = *var_qbu_slot;
        let mut var_qbu_dn0: f64 = *var_qbu_dn0_slot;
        let mut var_qbu_dn10: f64 = *var_qbu_dn10_slot;
        let mut var_qbu_dn11: f64 = *var_qbu_dn11_slot;
        let mut var_qbu_dn12: f64 = *var_qbu_dn12_slot;
        let mut var_qbu_dn17: f64 = *var_qbu_dn17_slot;
        let mut var_qbu_dn2: f64 = *var_qbu_dn2_slot;
        let mut var_qbu_dn6: f64 = *var_qbu_dn6_slot;
        let mut var_qbu_dn7: f64 = *var_qbu_dn7_slot;
        let mut var_qbu_rv: f64 = *var_qbu_rv_slot;
        let mut var_qd_fb: f64 = *var_qd_fb_slot;
        let mut var_qd_fb_dn0: f64 = *var_qd_fb_dn0_slot;
        let mut var_qd_fb_dn10: f64 = *var_qd_fb_dn10_slot;
        let mut var_qd_fb_dn11: f64 = *var_qd_fb_dn11_slot;
        let mut var_qd_fb_dn12: f64 = *var_qd_fb_dn12_slot;
        let mut var_qd_fb_dn13: f64 = *var_qd_fb_dn13_slot;
        let mut var_qd_fb_dn15: f64 = *var_qd_fb_dn15_slot;
        let mut var_qd_fb_dn16: f64 = *var_qd_fb_dn16_slot;
        let mut var_qd_fb_dn17: f64 = *var_qd_fb_dn17_slot;
        let mut var_qd_fb_dn18: f64 = *var_qd_fb_dn18_slot;
        let mut var_qd_fb_dn2: f64 = *var_qd_fb_dn2_slot;
        let mut var_qd_fb_dn6: f64 = *var_qd_fb_dn6_slot;
        let mut var_qd_fb_dn7: f64 = *var_qd_fb_dn7_slot;
        let mut var_qd_fb_rv: f64 = *var_qd_fb_rv_slot;
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
        let mut var_qd_nqs_rv: f64 = *var_qd_nqs_rv_slot;
        let mut var_qdrat: f64 = *var_qdrat_slot;
        let mut var_qdrat_dn0: f64 = *var_qdrat_dn0_slot;
        let mut var_qdrat_dn10: f64 = *var_qdrat_dn10_slot;
        let mut var_qdrat_dn11: f64 = *var_qdrat_dn11_slot;
        let mut var_qdrat_dn12: f64 = *var_qdrat_dn12_slot;
        let mut var_qdrat_dn17: f64 = *var_qdrat_dn17_slot;
        let mut var_qdrat_dn2: f64 = *var_qdrat_dn2_slot;
        let mut var_qdrat_dn6: f64 = *var_qdrat_dn6_slot;
        let mut var_qdrat_dn7: f64 = *var_qdrat_dn7_slot;
        let mut var_qdrat_noi: f64 = *var_qdrat_noi_slot;
        let mut var_qdrat_noi_dn0: f64 = *var_qdrat_noi_dn0_slot;
        let mut var_qdrat_noi_dn10: f64 = *var_qdrat_noi_dn10_slot;
        let mut var_qdrat_noi_dn11: f64 = *var_qdrat_noi_dn11_slot;
        let mut var_qdrat_noi_dn12: f64 = *var_qdrat_noi_dn12_slot;
        let mut var_qdrat_noi_dn17: f64 = *var_qdrat_noi_dn17_slot;
        let mut var_qdrat_noi_dn2: f64 = *var_qdrat_noi_dn2_slot;
        let mut var_qdrat_noi_dn6: f64 = *var_qdrat_noi_dn6_slot;
        let mut var_qdrat_noi_dn7: f64 = *var_qdrat_noi_dn7_slot;
        let mut var_qdrat_noi_rv: f64 = *var_qdrat_noi_rv_slot;
        let mut var_qdrat_rv: f64 = *var_qdrat_rv_slot;
        let mut var_qhs: f64 = *var_qhs_slot;
        let mut var_qhs_dn0: f64 = *var_qhs_dn0_slot;
        let mut var_qhs_dn10: f64 = *var_qhs_dn10_slot;
        let mut var_qhs_dn11: f64 = *var_qhs_dn11_slot;
        let mut var_qhs_dn12: f64 = *var_qhs_dn12_slot;
        let mut var_qhs_dn17: f64 = *var_qhs_dn17_slot;
        let mut var_qhs_dn2: f64 = *var_qhs_dn2_slot;
        let mut var_qhs_dn6: f64 = *var_qhs_dn6_slot;
        let mut var_qhs_dn7: f64 = *var_qhs_dn7_slot;
        let mut var_qhs_rv: f64 = *var_qhs_rv_slot;
        let mut var_qi_nqs: f64 = *var_qi_nqs_slot;
        let mut var_qi_nqs_dn18: f64 = *var_qi_nqs_dn18_slot;
        let mut var_qi_nqs_rv: f64 = *var_qi_nqs_rv_slot;
        let mut var_qiu: f64 = *var_qiu_slot;
        let mut var_qiu_dn0: f64 = *var_qiu_dn0_slot;
        let mut var_qiu_dn10: f64 = *var_qiu_dn10_slot;
        let mut var_qiu_dn11: f64 = *var_qiu_dn11_slot;
        let mut var_qiu_dn12: f64 = *var_qiu_dn12_slot;
        let mut var_qiu_dn17: f64 = *var_qiu_dn17_slot;
        let mut var_qiu_dn2: f64 = *var_qiu_dn2_slot;
        let mut var_qiu_dn6: f64 = *var_qiu_dn6_slot;
        let mut var_qiu_dn7: f64 = *var_qiu_dn7_slot;
        let mut var_qiu_rv: f64 = *var_qiu_rv_slot;
        let mut var_qs_fb: f64 = *var_qs_fb_slot;
        let mut var_qs_fb_dn0: f64 = *var_qs_fb_dn0_slot;
        let mut var_qs_fb_dn10: f64 = *var_qs_fb_dn10_slot;
        let mut var_qs_fb_dn11: f64 = *var_qs_fb_dn11_slot;
        let mut var_qs_fb_dn12: f64 = *var_qs_fb_dn12_slot;
        let mut var_qs_fb_dn13: f64 = *var_qs_fb_dn13_slot;
        let mut var_qs_fb_dn15: f64 = *var_qs_fb_dn15_slot;
        let mut var_qs_fb_dn16: f64 = *var_qs_fb_dn16_slot;
        let mut var_qs_fb_dn17: f64 = *var_qs_fb_dn17_slot;
        let mut var_qs_fb_dn18: f64 = *var_qs_fb_dn18_slot;
        let mut var_qs_fb_dn2: f64 = *var_qs_fb_dn2_slot;
        let mut var_qs_fb_dn6: f64 = *var_qs_fb_dn6_slot;
        let mut var_qs_fb_dn7: f64 = *var_qs_fb_dn7_slot;
        let mut var_qs_fb_rv: f64 = *var_qs_fb_rv_slot;
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
        let mut var_qs_nqs_rv: f64 = *var_qs_nqs_rv_slot;
        let mut var_qsub: f64 = *var_qsub_slot;
        let mut var_qsub_dn0: f64 = *var_qsub_dn0_slot;
        let mut var_qsub_dn10: f64 = *var_qsub_dn10_slot;
        let mut var_qsub_dn11: f64 = *var_qsub_dn11_slot;
        let mut var_qsub_dn12: f64 = *var_qsub_dn12_slot;
        let mut var_qsub_dn17: f64 = *var_qsub_dn17_slot;
        let mut var_qsub_dn2: f64 = *var_qsub_dn2_slot;
        let mut var_qsub_dn6: f64 = *var_qsub_dn6_slot;
        let mut var_qsub_dn7: f64 = *var_qsub_dn7_slot;
        let mut var_qsub_rv: f64 = *var_qsub_rv_slot;
        let mut var_shift: f64 = *var_shift_slot;
        let mut var_shift_dn0: f64 = *var_shift_dn0_slot;
        let mut var_shift_dn10: f64 = *var_shift_dn10_slot;
        let mut var_shift_dn11: f64 = *var_shift_dn11_slot;
        let mut var_shift_dn12: f64 = *var_shift_dn12_slot;
        let mut var_shift_dn17: f64 = *var_shift_dn17_slot;
        let mut var_shift_dn2: f64 = *var_shift_dn2_slot;
        let mut var_shift_dn6: f64 = *var_shift_dn6_slot;
        let mut var_shift_dn7: f64 = *var_shift_dn7_slot;
        let mut var_shift_rv: f64 = *var_shift_rv_slot;
        let mut var_sqrtkusail: f64 = *var_sqrtkusail_slot;
        let mut var_sqrtkusail_dn0: f64 = *var_sqrtkusail_dn0_slot;
        let mut var_sqrtkusail_dn10: f64 = *var_sqrtkusail_dn10_slot;
        let mut var_sqrtkusail_dn11: f64 = *var_sqrtkusail_dn11_slot;
        let mut var_sqrtkusail_dn12: f64 = *var_sqrtkusail_dn12_slot;
        let mut var_sqrtkusail_dn17: f64 = *var_sqrtkusail_dn17_slot;
        let mut var_sqrtkusail_dn2: f64 = *var_sqrtkusail_dn2_slot;
        let mut var_sqrtkusail_dn6: f64 = *var_sqrtkusail_dn6_slot;
        let mut var_sqrtkusail_dn7: f64 = *var_sqrtkusail_dn7_slot;
        let mut var_sqrtkusail_rv: f64 = *var_sqrtkusail_rv_slot;
        let mut var_uc_areabt: f64 = *var_uc_areabt_slot;
        let mut var_uc_areabt_rv: f64 = *var_uc_areabt_rv_slot;
        let mut var_uc_vfbbt: f64 = *var_uc_vfbbt_slot;
        let mut var_uc_vfbbt_rv: f64 = *var_uc_vfbbt_rv_slot;
        let mut var_wdsoi: f64 = *var_wdsoi_slot;
        let mut var_wdsoi_dn0: f64 = *var_wdsoi_dn0_slot;
        let mut var_wdsoi_dn10: f64 = *var_wdsoi_dn10_slot;
        let mut var_wdsoi_dn11: f64 = *var_wdsoi_dn11_slot;
        let mut var_wdsoi_dn12: f64 = *var_wdsoi_dn12_slot;
        let mut var_wdsoi_dn17: f64 = *var_wdsoi_dn17_slot;
        let mut var_wdsoi_dn2: f64 = *var_wdsoi_dn2_slot;
        let mut var_wdsoi_dn6: f64 = *var_wdsoi_dn6_slot;
        let mut var_wdsoi_dn7: f64 = *var_wdsoi_dn7_slot;
        let mut var_wdsoi_rv: f64 = *var_wdsoi_rv_slot;

        var_uc_areabt = 0.0;
        var_uc_areabt_rv = 0.0;

        var_uc_vfbbt = 0.0;
        var_uc_vfbbt_rv = 0.0;

        var_q_bt_ge = 0.0;
        var_q_bt_ge_dn0 = 0.0;
        var_q_bt_ge_dn2 = 0.0;
        var_q_bt_ge_dn6 = 0.0;
        var_q_bt_ge_dn7 = 0.0;
        var_q_bt_ge_dn10 = 0.0;
        var_q_bt_ge_dn11 = 0.0;
        var_q_bt_ge_dn12 = 0.0;
        var_q_bt_ge_dn17 = 0.0;
        var_q_bt_ge_rv = 0.0;

        var_q_bt_se = 0.0;
        var_q_bt_se_dn0 = 0.0;
        var_q_bt_se_dn2 = 0.0;
        var_q_bt_se_dn6 = 0.0;
        var_q_bt_se_dn7 = 0.0;
        var_q_bt_se_dn10 = 0.0;
        var_q_bt_se_dn11 = 0.0;
        var_q_bt_se_dn12 = 0.0;
        var_q_bt_se_dn17 = 0.0;
        var_q_bt_se_rv = 0.0;

        var_mud_hoso = 0.0;
        var_mud_hoso_dn0 = 0.0;
        var_mud_hoso_dn2 = 0.0;
        var_mud_hoso_dn6 = 0.0;
        var_mud_hoso_dn7 = 0.0;
        var_mud_hoso_dn10 = 0.0;
        var_mud_hoso_dn11 = 0.0;
        var_mud_hoso_dn12 = 0.0;
        var_mud_hoso_dn17 = 0.0;
        var_mud_hoso_rv = 0.0;

        var_kusai00 = 0.0;
        var_kusai00_dn0 = 0.0;
        var_kusai00_dn2 = 0.0;
        var_kusai00_dn6 = 0.0;
        var_kusai00_dn7 = 0.0;
        var_kusai00_dn10 = 0.0;
        var_kusai00_dn11 = 0.0;
        var_kusai00_dn12 = 0.0;
        var_kusai00_dn17 = 0.0;
        var_kusai00_rv = 0.0;

        var_kusail = 0.0;
        var_kusail_dn0 = 0.0;
        var_kusail_dn2 = 0.0;
        var_kusail_dn6 = 0.0;
        var_kusail_dn7 = 0.0;
        var_kusail_dn10 = 0.0;
        var_kusail_dn11 = 0.0;
        var_kusail_dn12 = 0.0;
        var_kusail_dn17 = 0.0;
        var_kusail_rv = 0.0;

        var_kusai00l = 0.0;
        var_kusai00l_dn0 = 0.0;
        var_kusai00l_dn2 = 0.0;
        var_kusai00l_dn6 = 0.0;
        var_kusai00l_dn7 = 0.0;
        var_kusai00l_dn10 = 0.0;
        var_kusai00l_dn11 = 0.0;
        var_kusai00l_dn12 = 0.0;
        var_kusai00l_dn17 = 0.0;
        var_kusai00l_rv = 0.0;

        var_sqrtkusail = 0.0;
        var_sqrtkusail_dn0 = 0.0;
        var_sqrtkusail_dn2 = 0.0;
        var_sqrtkusail_dn6 = 0.0;
        var_sqrtkusail_dn7 = 0.0;
        var_sqrtkusail_dn10 = 0.0;
        var_sqrtkusail_dn11 = 0.0;
        var_sqrtkusail_dn12 = 0.0;
        var_sqrtkusail_dn17 = 0.0;
        var_sqrtkusail_rv = 0.0;

        var_kusai_ig = 0.0;
        var_kusai_ig_dn0 = 0.0;
        var_kusai_ig_dn2 = 0.0;
        var_kusai_ig_dn6 = 0.0;
        var_kusai_ig_dn7 = 0.0;
        var_kusai_ig_dn10 = 0.0;
        var_kusai_ig_dn11 = 0.0;
        var_kusai_ig_dn12 = 0.0;
        var_kusai_ig_dn17 = 0.0;
        var_kusai_ig_rv = 0.0;

        var_psdl = 0.0;
        var_psdl_dn0 = 0.0;
        var_psdl_dn2 = 0.0;
        var_psdl_dn6 = 0.0;
        var_psdl_dn7 = 0.0;
        var_psdl_dn10 = 0.0;
        var_psdl_dn11 = 0.0;
        var_psdl_dn12 = 0.0;
        var_psdl_dn17 = 0.0;
        var_psdl_rv = 0.0;

        var_ec = 0.0;
        var_ec_dn0 = 0.0;
        var_ec_dn2 = 0.0;
        var_ec_dn6 = 0.0;
        var_ec_dn7 = 0.0;
        var_ec_dn10 = 0.0;
        var_ec_dn11 = 0.0;
        var_ec_dn12 = 0.0;
        var_ec_dn17 = 0.0;
        var_ec_rv = 0.0;

        var_lred = 0.0;
        var_lred_dn0 = 0.0;
        var_lred_dn2 = 0.0;
        var_lred_dn6 = 0.0;
        var_lred_dn7 = 0.0;
        var_lred_dn10 = 0.0;
        var_lred_dn11 = 0.0;
        var_lred_dn12 = 0.0;
        var_lred_dn17 = 0.0;
        var_lred_rv = 0.0;

        var_flg_depmode = 0.0;
        var_flg_depmode_rv = 0.0;

        var_phi_sl_soi_ini = 0.0;
        var_phi_sl_soi_ini_dn0 = 0.0;
        var_phi_sl_soi_ini_dn2 = 0.0;
        var_phi_sl_soi_ini_dn6 = 0.0;
        var_phi_sl_soi_ini_dn7 = 0.0;
        var_phi_sl_soi_ini_dn10 = 0.0;
        var_phi_sl_soi_ini_dn11 = 0.0;
        var_phi_sl_soi_ini_dn12 = 0.0;
        var_phi_sl_soi_ini_dn17 = 0.0;
        var_phi_sl_soi_ini_rv = 0.0;

        var_phi_bl_soi_ini = 0.0;
        var_phi_bl_soi_ini_dn0 = 0.0;
        var_phi_bl_soi_ini_dn2 = 0.0;
        var_phi_bl_soi_ini_dn6 = 0.0;
        var_phi_bl_soi_ini_dn7 = 0.0;
        var_phi_bl_soi_ini_dn10 = 0.0;
        var_phi_bl_soi_ini_dn11 = 0.0;
        var_phi_bl_soi_ini_dn12 = 0.0;
        var_phi_bl_soi_ini_dn17 = 0.0;
        var_phi_bl_soi_ini_rv = 0.0;

        var_phi_sl_bulk_ini = 0.0;
        var_phi_sl_bulk_ini_dn0 = 0.0;
        var_phi_sl_bulk_ini_dn2 = 0.0;
        var_phi_sl_bulk_ini_dn6 = 0.0;
        var_phi_sl_bulk_ini_dn7 = 0.0;
        var_phi_sl_bulk_ini_dn10 = 0.0;
        var_phi_sl_bulk_ini_dn11 = 0.0;
        var_phi_sl_bulk_ini_dn12 = 0.0;
        var_phi_sl_bulk_ini_dn17 = 0.0;
        var_phi_sl_bulk_ini_rv = 0.0;

        var_phi_s0_soi = 0.0;
        var_phi_s0_soi_dn0 = 0.0;
        var_phi_s0_soi_dn2 = 0.0;
        var_phi_s0_soi_dn6 = 0.0;
        var_phi_s0_soi_dn7 = 0.0;
        var_phi_s0_soi_dn10 = 0.0;
        var_phi_s0_soi_dn11 = 0.0;
        var_phi_s0_soi_dn12 = 0.0;
        var_phi_s0_soi_dn17 = 0.0;
        var_phi_s0_soi_rv = 0.0;

        var_phi_b0_soi = 0.0;
        var_phi_b0_soi_dn0 = 0.0;
        var_phi_b0_soi_dn2 = 0.0;
        var_phi_b0_soi_dn6 = 0.0;
        var_phi_b0_soi_dn7 = 0.0;
        var_phi_b0_soi_dn10 = 0.0;
        var_phi_b0_soi_dn11 = 0.0;
        var_phi_b0_soi_dn12 = 0.0;
        var_phi_b0_soi_dn17 = 0.0;
        var_phi_b0_soi_rv = 0.0;

        var_phi_s0_bulk = 0.0;
        var_phi_s0_bulk_dn0 = 0.0;
        var_phi_s0_bulk_dn2 = 0.0;
        var_phi_s0_bulk_dn6 = 0.0;
        var_phi_s0_bulk_dn7 = 0.0;
        var_phi_s0_bulk_dn10 = 0.0;
        var_phi_s0_bulk_dn11 = 0.0;
        var_phi_s0_bulk_dn12 = 0.0;
        var_phi_s0_bulk_dn17 = 0.0;
        var_phi_s0_bulk_rv = 0.0;

        var_phi_sl_soi = 0.0;
        var_phi_sl_soi_dn0 = 0.0;
        var_phi_sl_soi_dn2 = 0.0;
        var_phi_sl_soi_dn6 = 0.0;
        var_phi_sl_soi_dn7 = 0.0;
        var_phi_sl_soi_dn10 = 0.0;
        var_phi_sl_soi_dn11 = 0.0;
        var_phi_sl_soi_dn12 = 0.0;
        var_phi_sl_soi_dn17 = 0.0;
        var_phi_sl_soi_rv = 0.0;

        var_phi_bl_soi = 0.0;
        var_phi_bl_soi_dn0 = 0.0;
        var_phi_bl_soi_dn2 = 0.0;
        var_phi_bl_soi_dn6 = 0.0;
        var_phi_bl_soi_dn7 = 0.0;
        var_phi_bl_soi_dn10 = 0.0;
        var_phi_bl_soi_dn11 = 0.0;
        var_phi_bl_soi_dn12 = 0.0;
        var_phi_bl_soi_dn17 = 0.0;
        var_phi_bl_soi_rv = 0.0;

        var_phi_sl_bulk = 0.0;
        var_phi_sl_bulk_dn0 = 0.0;
        var_phi_sl_bulk_dn2 = 0.0;
        var_phi_sl_bulk_dn6 = 0.0;
        var_phi_sl_bulk_dn7 = 0.0;
        var_phi_sl_bulk_dn10 = 0.0;
        var_phi_sl_bulk_dn11 = 0.0;
        var_phi_sl_bulk_dn12 = 0.0;
        var_phi_sl_bulk_dn17 = 0.0;
        var_phi_sl_bulk_rv = 0.0;

        var_q_dep_soi = 0.0;
        var_q_dep_soi_dn0 = 0.0;
        var_q_dep_soi_dn2 = 0.0;
        var_q_dep_soi_dn6 = 0.0;
        var_q_dep_soi_dn7 = 0.0;
        var_q_dep_soi_dn10 = 0.0;
        var_q_dep_soi_dn11 = 0.0;
        var_q_dep_soi_dn12 = 0.0;
        var_q_dep_soi_dn17 = 0.0;
        var_q_dep_soi_rv = 0.0;

        var_q_n0 = 0.0;
        var_q_n0_dn0 = 0.0;
        var_q_n0_dn2 = 0.0;
        var_q_n0_dn6 = 0.0;
        var_q_n0_dn7 = 0.0;
        var_q_n0_dn10 = 0.0;
        var_q_n0_dn11 = 0.0;
        var_q_n0_dn12 = 0.0;
        var_q_n0_dn17 = 0.0;
        var_q_n0_rv = 0.0;

        var_q_b0_dep = 0.0;
        var_q_b0_dep_dn0 = 0.0;
        var_q_b0_dep_dn2 = 0.0;
        var_q_b0_dep_dn6 = 0.0;
        var_q_b0_dep_dn7 = 0.0;
        var_q_b0_dep_dn10 = 0.0;
        var_q_b0_dep_dn11 = 0.0;
        var_q_b0_dep_dn12 = 0.0;
        var_q_b0_dep_dn17 = 0.0;
        var_q_b0_dep_rv = 0.0;

        var_q_bl_dep = 0.0;
        var_q_bl_dep_dn0 = 0.0;
        var_q_bl_dep_dn2 = 0.0;
        var_q_bl_dep_dn6 = 0.0;
        var_q_bl_dep_dn7 = 0.0;
        var_q_bl_dep_dn10 = 0.0;
        var_q_bl_dep_dn11 = 0.0;
        var_q_bl_dep_dn12 = 0.0;
        var_q_bl_dep_dn17 = 0.0;
        var_q_bl_dep_rv = 0.0;

        var_q_dep0 = 0.0;
        var_q_dep0_dn0 = 0.0;
        var_q_dep0_dn2 = 0.0;
        var_q_dep0_dn6 = 0.0;
        var_q_dep0_dn7 = 0.0;
        var_q_dep0_dn10 = 0.0;
        var_q_dep0_dn11 = 0.0;
        var_q_dep0_dn12 = 0.0;
        var_q_dep0_dn17 = 0.0;
        var_q_dep0_rv = 0.0;

        var_q_s0_bulk = 0.0;
        var_q_s0_bulk_dn0 = 0.0;
        var_q_s0_bulk_dn2 = 0.0;
        var_q_s0_bulk_dn6 = 0.0;
        var_q_s0_bulk_dn7 = 0.0;
        var_q_s0_bulk_dn10 = 0.0;
        var_q_s0_bulk_dn11 = 0.0;
        var_q_s0_bulk_dn12 = 0.0;
        var_q_s0_bulk_dn17 = 0.0;
        var_q_s0_bulk_rv = 0.0;

        var_q_nl = 0.0;
        var_q_nl_dn0 = 0.0;
        var_q_nl_dn2 = 0.0;
        var_q_nl_dn6 = 0.0;
        var_q_nl_dn7 = 0.0;
        var_q_nl_dn10 = 0.0;
        var_q_nl_dn11 = 0.0;
        var_q_nl_dn12 = 0.0;
        var_q_nl_dn17 = 0.0;
        var_q_nl_rv = 0.0;

        var_q_depl = 0.0;
        var_q_depl_dn0 = 0.0;
        var_q_depl_dn2 = 0.0;
        var_q_depl_dn6 = 0.0;
        var_q_depl_dn7 = 0.0;
        var_q_depl_dn10 = 0.0;
        var_q_depl_dn11 = 0.0;
        var_q_depl_dn12 = 0.0;
        var_q_depl_dn17 = 0.0;
        var_q_depl_rv = 0.0;

        var_q_sl_bulk = 0.0;
        var_q_sl_bulk_dn0 = 0.0;
        var_q_sl_bulk_dn2 = 0.0;
        var_q_sl_bulk_dn6 = 0.0;
        var_q_sl_bulk_dn7 = 0.0;
        var_q_sl_bulk_dn10 = 0.0;
        var_q_sl_bulk_dn11 = 0.0;
        var_q_sl_bulk_dn12 = 0.0;
        var_q_sl_bulk_dn17 = 0.0;
        var_q_sl_bulk_rv = 0.0;

        var_shift = 0.0;
        var_shift_dn0 = 0.0;
        var_shift_dn2 = 0.0;
        var_shift_dn6 = 0.0;
        var_shift_dn7 = 0.0;
        var_shift_dn10 = 0.0;
        var_shift_dn11 = 0.0;
        var_shift_dn12 = 0.0;
        var_shift_dn17 = 0.0;
        var_shift_rv = 0.0;

        var_q_s0_bulk_0 = 0.0;
        var_q_s0_bulk_0_dn0 = 0.0;
        var_q_s0_bulk_0_dn2 = 0.0;
        var_q_s0_bulk_0_dn6 = 0.0;
        var_q_s0_bulk_0_dn7 = 0.0;
        var_q_s0_bulk_0_dn10 = 0.0;
        var_q_s0_bulk_0_dn11 = 0.0;
        var_q_s0_bulk_0_dn12 = 0.0;
        var_q_s0_bulk_0_dn17 = 0.0;
        var_q_s0_bulk_0_rv = 0.0;

        var_qi_nqs = 0.0;
        var_qi_nqs_dn18 = 0.0;
        var_qi_nqs_rv = 0.0;

        var_qd_nqs = 0.0;
        var_qd_nqs_dn0 = 0.0;
        var_qd_nqs_dn2 = 0.0;
        var_qd_nqs_dn6 = 0.0;
        var_qd_nqs_dn7 = 0.0;
        var_qd_nqs_dn10 = 0.0;
        var_qd_nqs_dn11 = 0.0;
        var_qd_nqs_dn12 = 0.0;
        var_qd_nqs_dn15 = 0.0;
        var_qd_nqs_dn17 = 0.0;
        var_qd_nqs_dn18 = 0.0;
        var_qd_nqs_rv = 0.0;

        var_qs_nqs = 0.0;
        var_qs_nqs_dn0 = 0.0;
        var_qs_nqs_dn2 = 0.0;
        var_qs_nqs_dn6 = 0.0;
        var_qs_nqs_dn7 = 0.0;
        var_qs_nqs_dn10 = 0.0;
        var_qs_nqs_dn11 = 0.0;
        var_qs_nqs_dn12 = 0.0;
        var_qs_nqs_dn16 = 0.0;
        var_qs_nqs_dn17 = 0.0;
        var_qs_nqs_dn18 = 0.0;
        var_qs_nqs_rv = 0.0;

        var_phi_b_dep0 = 0.0;
        var_phi_b_dep0_dn0 = 0.0;
        var_phi_b_dep0_dn2 = 0.0;
        var_phi_b_dep0_dn6 = 0.0;
        var_phi_b_dep0_dn7 = 0.0;
        var_phi_b_dep0_dn10 = 0.0;
        var_phi_b_dep0_dn11 = 0.0;
        var_phi_b_dep0_dn12 = 0.0;
        var_phi_b_dep0_dn17 = 0.0;
        var_phi_b_dep0_rv = 0.0;

        var_qsub = 0.0;
        var_qsub_dn0 = 0.0;
        var_qsub_dn2 = 0.0;
        var_qsub_dn6 = 0.0;
        var_qsub_dn7 = 0.0;
        var_qsub_dn10 = 0.0;
        var_qsub_dn11 = 0.0;
        var_qsub_dn12 = 0.0;
        var_qsub_dn17 = 0.0;
        var_qsub_rv = 0.0;

        var_qhs = 0.0;
        var_qhs_dn0 = 0.0;
        var_qhs_dn2 = 0.0;
        var_qhs_dn6 = 0.0;
        var_qhs_dn7 = 0.0;
        var_qhs_dn10 = 0.0;
        var_qhs_dn11 = 0.0;
        var_qhs_dn12 = 0.0;
        var_qhs_dn17 = 0.0;
        var_qhs_rv = 0.0;

        var_wdsoi = 0.0;
        var_wdsoi_dn0 = 0.0;
        var_wdsoi_dn2 = 0.0;
        var_wdsoi_dn6 = 0.0;
        var_wdsoi_dn7 = 0.0;
        var_wdsoi_dn10 = 0.0;
        var_wdsoi_dn11 = 0.0;
        var_wdsoi_dn12 = 0.0;
        var_wdsoi_dn17 = 0.0;
        var_wdsoi_rv = 0.0;

        var_ps0_inia = 0.0;
        var_ps0_inia_dn0 = 0.0;
        var_ps0_inia_dn2 = 0.0;
        var_ps0_inia_dn6 = 0.0;
        var_ps0_inia_dn7 = 0.0;
        var_ps0_inia_dn10 = 0.0;
        var_ps0_inia_dn11 = 0.0;
        var_ps0_inia_dn12 = 0.0;
        var_ps0_inia_dn17 = 0.0;
        var_ps0_inia_rv = 0.0;

        var_qiu = 0.0;
        var_qiu_dn0 = 0.0;
        var_qiu_dn2 = 0.0;
        var_qiu_dn6 = 0.0;
        var_qiu_dn7 = 0.0;
        var_qiu_dn10 = 0.0;
        var_qiu_dn11 = 0.0;
        var_qiu_dn12 = 0.0;
        var_qiu_dn17 = 0.0;
        var_qiu_rv = 0.0;

        var_qbu = 0.0;
        var_qbu_dn0 = 0.0;
        var_qbu_dn2 = 0.0;
        var_qbu_dn6 = 0.0;
        var_qbu_dn7 = 0.0;
        var_qbu_dn10 = 0.0;
        var_qbu_dn11 = 0.0;
        var_qbu_dn12 = 0.0;
        var_qbu_dn17 = 0.0;
        var_qbu_rv = 0.0;

        var_qdrat = 0.5;
        var_qdrat_dn0 = 0.0;
        var_qdrat_dn2 = 0.0;
        var_qdrat_dn6 = 0.0;
        var_qdrat_dn7 = 0.0;
        var_qdrat_dn10 = 0.0;
        var_qdrat_dn11 = 0.0;
        var_qdrat_dn12 = 0.0;
        var_qdrat_dn17 = 0.0;
        var_qdrat_rv = 0.0;

        var_qdrat_noi = 0.5;
        var_qdrat_noi_dn0 = 0.0;
        var_qdrat_noi_dn2 = 0.0;
        var_qdrat_noi_dn6 = 0.0;
        var_qdrat_noi_dn7 = 0.0;
        var_qdrat_noi_dn10 = 0.0;
        var_qdrat_noi_dn11 = 0.0;
        var_qdrat_noi_dn12 = 0.0;
        var_qdrat_noi_dn17 = 0.0;
        var_qdrat_noi_rv = 0.0;

        var_qs_fb = 0.0;
        var_qs_fb_dn0 = 0.0;
        var_qs_fb_dn2 = 0.0;
        var_qs_fb_dn6 = 0.0;
        var_qs_fb_dn7 = 0.0;
        var_qs_fb_dn10 = 0.0;
        var_qs_fb_dn11 = 0.0;
        var_qs_fb_dn12 = 0.0;
        var_qs_fb_dn13 = 0.0;
        var_qs_fb_dn15 = 0.0;
        var_qs_fb_dn16 = 0.0;
        var_qs_fb_dn17 = 0.0;
        var_qs_fb_dn18 = 0.0;
        var_qs_fb_rv = 0.0;

        var_qd_fb = 0.0;
        var_qd_fb_dn0 = 0.0;
        var_qd_fb_dn2 = 0.0;
        var_qd_fb_dn6 = 0.0;
        var_qd_fb_dn7 = 0.0;
        var_qd_fb_dn10 = 0.0;
        var_qd_fb_dn11 = 0.0;
        var_qd_fb_dn12 = 0.0;
        var_qd_fb_dn13 = 0.0;
        var_qd_fb_dn15 = 0.0;
        var_qd_fb_dn16 = 0.0;
        var_qd_fb_dn17 = 0.0;
        var_qd_fb_dn18 = 0.0;
        var_qd_fb_rv = 0.0;

        *var_ec_slot = var_ec;
        *var_ec_dn0_slot = var_ec_dn0;
        *var_ec_dn10_slot = var_ec_dn10;
        *var_ec_dn11_slot = var_ec_dn11;
        *var_ec_dn12_slot = var_ec_dn12;
        *var_ec_dn17_slot = var_ec_dn17;
        *var_ec_dn2_slot = var_ec_dn2;
        *var_ec_dn6_slot = var_ec_dn6;
        *var_ec_dn7_slot = var_ec_dn7;
        *var_ec_rv_slot = var_ec_rv;
        *var_flg_depmode_slot = var_flg_depmode;
        *var_flg_depmode_rv_slot = var_flg_depmode_rv;
        *var_kusai00_slot = var_kusai00;
        *var_kusai00_dn0_slot = var_kusai00_dn0;
        *var_kusai00_dn10_slot = var_kusai00_dn10;
        *var_kusai00_dn11_slot = var_kusai00_dn11;
        *var_kusai00_dn12_slot = var_kusai00_dn12;
        *var_kusai00_dn17_slot = var_kusai00_dn17;
        *var_kusai00_dn2_slot = var_kusai00_dn2;
        *var_kusai00_dn6_slot = var_kusai00_dn6;
        *var_kusai00_dn7_slot = var_kusai00_dn7;
        *var_kusai00_rv_slot = var_kusai00_rv;
        *var_kusai00l_slot = var_kusai00l;
        *var_kusai00l_dn0_slot = var_kusai00l_dn0;
        *var_kusai00l_dn10_slot = var_kusai00l_dn10;
        *var_kusai00l_dn11_slot = var_kusai00l_dn11;
        *var_kusai00l_dn12_slot = var_kusai00l_dn12;
        *var_kusai00l_dn17_slot = var_kusai00l_dn17;
        *var_kusai00l_dn2_slot = var_kusai00l_dn2;
        *var_kusai00l_dn6_slot = var_kusai00l_dn6;
        *var_kusai00l_dn7_slot = var_kusai00l_dn7;
        *var_kusai00l_rv_slot = var_kusai00l_rv;
        *var_kusai_ig_slot = var_kusai_ig;
        *var_kusai_ig_dn0_slot = var_kusai_ig_dn0;
        *var_kusai_ig_dn10_slot = var_kusai_ig_dn10;
        *var_kusai_ig_dn11_slot = var_kusai_ig_dn11;
        *var_kusai_ig_dn12_slot = var_kusai_ig_dn12;
        *var_kusai_ig_dn17_slot = var_kusai_ig_dn17;
        *var_kusai_ig_dn2_slot = var_kusai_ig_dn2;
        *var_kusai_ig_dn6_slot = var_kusai_ig_dn6;
        *var_kusai_ig_dn7_slot = var_kusai_ig_dn7;
        *var_kusai_ig_rv_slot = var_kusai_ig_rv;
        *var_kusail_slot = var_kusail;
        *var_kusail_dn0_slot = var_kusail_dn0;
        *var_kusail_dn10_slot = var_kusail_dn10;
        *var_kusail_dn11_slot = var_kusail_dn11;
        *var_kusail_dn12_slot = var_kusail_dn12;
        *var_kusail_dn17_slot = var_kusail_dn17;
        *var_kusail_dn2_slot = var_kusail_dn2;
        *var_kusail_dn6_slot = var_kusail_dn6;
        *var_kusail_dn7_slot = var_kusail_dn7;
        *var_kusail_rv_slot = var_kusail_rv;
        *var_lred_slot = var_lred;
        *var_lred_dn0_slot = var_lred_dn0;
        *var_lred_dn10_slot = var_lred_dn10;
        *var_lred_dn11_slot = var_lred_dn11;
        *var_lred_dn12_slot = var_lred_dn12;
        *var_lred_dn17_slot = var_lred_dn17;
        *var_lred_dn2_slot = var_lred_dn2;
        *var_lred_dn6_slot = var_lred_dn6;
        *var_lred_dn7_slot = var_lred_dn7;
        *var_lred_rv_slot = var_lred_rv;
        *var_mud_hoso_slot = var_mud_hoso;
        *var_mud_hoso_dn0_slot = var_mud_hoso_dn0;
        *var_mud_hoso_dn10_slot = var_mud_hoso_dn10;
        *var_mud_hoso_dn11_slot = var_mud_hoso_dn11;
        *var_mud_hoso_dn12_slot = var_mud_hoso_dn12;
        *var_mud_hoso_dn17_slot = var_mud_hoso_dn17;
        *var_mud_hoso_dn2_slot = var_mud_hoso_dn2;
        *var_mud_hoso_dn6_slot = var_mud_hoso_dn6;
        *var_mud_hoso_dn7_slot = var_mud_hoso_dn7;
        *var_mud_hoso_rv_slot = var_mud_hoso_rv;
        *var_phi_b0_soi_slot = var_phi_b0_soi;
        *var_phi_b0_soi_dn0_slot = var_phi_b0_soi_dn0;
        *var_phi_b0_soi_dn10_slot = var_phi_b0_soi_dn10;
        *var_phi_b0_soi_dn11_slot = var_phi_b0_soi_dn11;
        *var_phi_b0_soi_dn12_slot = var_phi_b0_soi_dn12;
        *var_phi_b0_soi_dn17_slot = var_phi_b0_soi_dn17;
        *var_phi_b0_soi_dn2_slot = var_phi_b0_soi_dn2;
        *var_phi_b0_soi_dn6_slot = var_phi_b0_soi_dn6;
        *var_phi_b0_soi_dn7_slot = var_phi_b0_soi_dn7;
        *var_phi_b0_soi_rv_slot = var_phi_b0_soi_rv;
        *var_phi_b_dep0_slot = var_phi_b_dep0;
        *var_phi_b_dep0_dn0_slot = var_phi_b_dep0_dn0;
        *var_phi_b_dep0_dn10_slot = var_phi_b_dep0_dn10;
        *var_phi_b_dep0_dn11_slot = var_phi_b_dep0_dn11;
        *var_phi_b_dep0_dn12_slot = var_phi_b_dep0_dn12;
        *var_phi_b_dep0_dn17_slot = var_phi_b_dep0_dn17;
        *var_phi_b_dep0_dn2_slot = var_phi_b_dep0_dn2;
        *var_phi_b_dep0_dn6_slot = var_phi_b_dep0_dn6;
        *var_phi_b_dep0_dn7_slot = var_phi_b_dep0_dn7;
        *var_phi_b_dep0_rv_slot = var_phi_b_dep0_rv;
        *var_phi_bl_soi_slot = var_phi_bl_soi;
        *var_phi_bl_soi_dn0_slot = var_phi_bl_soi_dn0;
        *var_phi_bl_soi_dn10_slot = var_phi_bl_soi_dn10;
        *var_phi_bl_soi_dn11_slot = var_phi_bl_soi_dn11;
        *var_phi_bl_soi_dn12_slot = var_phi_bl_soi_dn12;
        *var_phi_bl_soi_dn17_slot = var_phi_bl_soi_dn17;
        *var_phi_bl_soi_dn2_slot = var_phi_bl_soi_dn2;
        *var_phi_bl_soi_dn6_slot = var_phi_bl_soi_dn6;
        *var_phi_bl_soi_dn7_slot = var_phi_bl_soi_dn7;
        *var_phi_bl_soi_ini_slot = var_phi_bl_soi_ini;
        *var_phi_bl_soi_ini_dn0_slot = var_phi_bl_soi_ini_dn0;
        *var_phi_bl_soi_ini_dn10_slot = var_phi_bl_soi_ini_dn10;
        *var_phi_bl_soi_ini_dn11_slot = var_phi_bl_soi_ini_dn11;
        *var_phi_bl_soi_ini_dn12_slot = var_phi_bl_soi_ini_dn12;
        *var_phi_bl_soi_ini_dn17_slot = var_phi_bl_soi_ini_dn17;
        *var_phi_bl_soi_ini_dn2_slot = var_phi_bl_soi_ini_dn2;
        *var_phi_bl_soi_ini_dn6_slot = var_phi_bl_soi_ini_dn6;
        *var_phi_bl_soi_ini_dn7_slot = var_phi_bl_soi_ini_dn7;
        *var_phi_bl_soi_ini_rv_slot = var_phi_bl_soi_ini_rv;
        *var_phi_bl_soi_rv_slot = var_phi_bl_soi_rv;
        *var_phi_s0_bulk_slot = var_phi_s0_bulk;
        *var_phi_s0_bulk_dn0_slot = var_phi_s0_bulk_dn0;
        *var_phi_s0_bulk_dn10_slot = var_phi_s0_bulk_dn10;
        *var_phi_s0_bulk_dn11_slot = var_phi_s0_bulk_dn11;
        *var_phi_s0_bulk_dn12_slot = var_phi_s0_bulk_dn12;
        *var_phi_s0_bulk_dn17_slot = var_phi_s0_bulk_dn17;
        *var_phi_s0_bulk_dn2_slot = var_phi_s0_bulk_dn2;
        *var_phi_s0_bulk_dn6_slot = var_phi_s0_bulk_dn6;
        *var_phi_s0_bulk_dn7_slot = var_phi_s0_bulk_dn7;
        *var_phi_s0_bulk_rv_slot = var_phi_s0_bulk_rv;
        *var_phi_s0_soi_slot = var_phi_s0_soi;
        *var_phi_s0_soi_dn0_slot = var_phi_s0_soi_dn0;
        *var_phi_s0_soi_dn10_slot = var_phi_s0_soi_dn10;
        *var_phi_s0_soi_dn11_slot = var_phi_s0_soi_dn11;
        *var_phi_s0_soi_dn12_slot = var_phi_s0_soi_dn12;
        *var_phi_s0_soi_dn17_slot = var_phi_s0_soi_dn17;
        *var_phi_s0_soi_dn2_slot = var_phi_s0_soi_dn2;
        *var_phi_s0_soi_dn6_slot = var_phi_s0_soi_dn6;
        *var_phi_s0_soi_dn7_slot = var_phi_s0_soi_dn7;
        *var_phi_s0_soi_rv_slot = var_phi_s0_soi_rv;
        *var_phi_sl_bulk_slot = var_phi_sl_bulk;
        *var_phi_sl_bulk_dn0_slot = var_phi_sl_bulk_dn0;
        *var_phi_sl_bulk_dn10_slot = var_phi_sl_bulk_dn10;
        *var_phi_sl_bulk_dn11_slot = var_phi_sl_bulk_dn11;
        *var_phi_sl_bulk_dn12_slot = var_phi_sl_bulk_dn12;
        *var_phi_sl_bulk_dn17_slot = var_phi_sl_bulk_dn17;
        *var_phi_sl_bulk_dn2_slot = var_phi_sl_bulk_dn2;
        *var_phi_sl_bulk_dn6_slot = var_phi_sl_bulk_dn6;
        *var_phi_sl_bulk_dn7_slot = var_phi_sl_bulk_dn7;
        *var_phi_sl_bulk_ini_slot = var_phi_sl_bulk_ini;
        *var_phi_sl_bulk_ini_dn0_slot = var_phi_sl_bulk_ini_dn0;
        *var_phi_sl_bulk_ini_dn10_slot = var_phi_sl_bulk_ini_dn10;
        *var_phi_sl_bulk_ini_dn11_slot = var_phi_sl_bulk_ini_dn11;
        *var_phi_sl_bulk_ini_dn12_slot = var_phi_sl_bulk_ini_dn12;
        *var_phi_sl_bulk_ini_dn17_slot = var_phi_sl_bulk_ini_dn17;
        *var_phi_sl_bulk_ini_dn2_slot = var_phi_sl_bulk_ini_dn2;
        *var_phi_sl_bulk_ini_dn6_slot = var_phi_sl_bulk_ini_dn6;
        *var_phi_sl_bulk_ini_dn7_slot = var_phi_sl_bulk_ini_dn7;
        *var_phi_sl_bulk_ini_rv_slot = var_phi_sl_bulk_ini_rv;
        *var_phi_sl_bulk_rv_slot = var_phi_sl_bulk_rv;
        *var_phi_sl_soi_slot = var_phi_sl_soi;
        *var_phi_sl_soi_dn0_slot = var_phi_sl_soi_dn0;
        *var_phi_sl_soi_dn10_slot = var_phi_sl_soi_dn10;
        *var_phi_sl_soi_dn11_slot = var_phi_sl_soi_dn11;
        *var_phi_sl_soi_dn12_slot = var_phi_sl_soi_dn12;
        *var_phi_sl_soi_dn17_slot = var_phi_sl_soi_dn17;
        *var_phi_sl_soi_dn2_slot = var_phi_sl_soi_dn2;
        *var_phi_sl_soi_dn6_slot = var_phi_sl_soi_dn6;
        *var_phi_sl_soi_dn7_slot = var_phi_sl_soi_dn7;
        *var_phi_sl_soi_ini_slot = var_phi_sl_soi_ini;
        *var_phi_sl_soi_ini_dn0_slot = var_phi_sl_soi_ini_dn0;
        *var_phi_sl_soi_ini_dn10_slot = var_phi_sl_soi_ini_dn10;
        *var_phi_sl_soi_ini_dn11_slot = var_phi_sl_soi_ini_dn11;
        *var_phi_sl_soi_ini_dn12_slot = var_phi_sl_soi_ini_dn12;
        *var_phi_sl_soi_ini_dn17_slot = var_phi_sl_soi_ini_dn17;
        *var_phi_sl_soi_ini_dn2_slot = var_phi_sl_soi_ini_dn2;
        *var_phi_sl_soi_ini_dn6_slot = var_phi_sl_soi_ini_dn6;
        *var_phi_sl_soi_ini_dn7_slot = var_phi_sl_soi_ini_dn7;
        *var_phi_sl_soi_ini_rv_slot = var_phi_sl_soi_ini_rv;
        *var_phi_sl_soi_rv_slot = var_phi_sl_soi_rv;
        *var_ps0_inia_slot = var_ps0_inia;
        *var_ps0_inia_dn0_slot = var_ps0_inia_dn0;
        *var_ps0_inia_dn10_slot = var_ps0_inia_dn10;
        *var_ps0_inia_dn11_slot = var_ps0_inia_dn11;
        *var_ps0_inia_dn12_slot = var_ps0_inia_dn12;
        *var_ps0_inia_dn17_slot = var_ps0_inia_dn17;
        *var_ps0_inia_dn2_slot = var_ps0_inia_dn2;
        *var_ps0_inia_dn6_slot = var_ps0_inia_dn6;
        *var_ps0_inia_dn7_slot = var_ps0_inia_dn7;
        *var_ps0_inia_rv_slot = var_ps0_inia_rv;
        *var_psdl_slot = var_psdl;
        *var_psdl_dn0_slot = var_psdl_dn0;
        *var_psdl_dn10_slot = var_psdl_dn10;
        *var_psdl_dn11_slot = var_psdl_dn11;
        *var_psdl_dn12_slot = var_psdl_dn12;
        *var_psdl_dn17_slot = var_psdl_dn17;
        *var_psdl_dn2_slot = var_psdl_dn2;
        *var_psdl_dn6_slot = var_psdl_dn6;
        *var_psdl_dn7_slot = var_psdl_dn7;
        *var_psdl_rv_slot = var_psdl_rv;
        *var_q_b0_dep_slot = var_q_b0_dep;
        *var_q_b0_dep_dn0_slot = var_q_b0_dep_dn0;
        *var_q_b0_dep_dn10_slot = var_q_b0_dep_dn10;
        *var_q_b0_dep_dn11_slot = var_q_b0_dep_dn11;
        *var_q_b0_dep_dn12_slot = var_q_b0_dep_dn12;
        *var_q_b0_dep_dn17_slot = var_q_b0_dep_dn17;
        *var_q_b0_dep_dn2_slot = var_q_b0_dep_dn2;
        *var_q_b0_dep_dn6_slot = var_q_b0_dep_dn6;
        *var_q_b0_dep_dn7_slot = var_q_b0_dep_dn7;
        *var_q_b0_dep_rv_slot = var_q_b0_dep_rv;
        *var_q_bl_dep_slot = var_q_bl_dep;
        *var_q_bl_dep_dn0_slot = var_q_bl_dep_dn0;
        *var_q_bl_dep_dn10_slot = var_q_bl_dep_dn10;
        *var_q_bl_dep_dn11_slot = var_q_bl_dep_dn11;
        *var_q_bl_dep_dn12_slot = var_q_bl_dep_dn12;
        *var_q_bl_dep_dn17_slot = var_q_bl_dep_dn17;
        *var_q_bl_dep_dn2_slot = var_q_bl_dep_dn2;
        *var_q_bl_dep_dn6_slot = var_q_bl_dep_dn6;
        *var_q_bl_dep_dn7_slot = var_q_bl_dep_dn7;
        *var_q_bl_dep_rv_slot = var_q_bl_dep_rv;
        *var_q_bt_ge_slot = var_q_bt_ge;
        *var_q_bt_ge_dn0_slot = var_q_bt_ge_dn0;
        *var_q_bt_ge_dn10_slot = var_q_bt_ge_dn10;
        *var_q_bt_ge_dn11_slot = var_q_bt_ge_dn11;
        *var_q_bt_ge_dn12_slot = var_q_bt_ge_dn12;
        *var_q_bt_ge_dn17_slot = var_q_bt_ge_dn17;
        *var_q_bt_ge_dn2_slot = var_q_bt_ge_dn2;
        *var_q_bt_ge_dn6_slot = var_q_bt_ge_dn6;
        *var_q_bt_ge_dn7_slot = var_q_bt_ge_dn7;
        *var_q_bt_ge_rv_slot = var_q_bt_ge_rv;
        *var_q_bt_se_slot = var_q_bt_se;
        *var_q_bt_se_dn0_slot = var_q_bt_se_dn0;
        *var_q_bt_se_dn10_slot = var_q_bt_se_dn10;
        *var_q_bt_se_dn11_slot = var_q_bt_se_dn11;
        *var_q_bt_se_dn12_slot = var_q_bt_se_dn12;
        *var_q_bt_se_dn17_slot = var_q_bt_se_dn17;
        *var_q_bt_se_dn2_slot = var_q_bt_se_dn2;
        *var_q_bt_se_dn6_slot = var_q_bt_se_dn6;
        *var_q_bt_se_dn7_slot = var_q_bt_se_dn7;
        *var_q_bt_se_rv_slot = var_q_bt_se_rv;
        *var_q_dep0_slot = var_q_dep0;
        *var_q_dep0_dn0_slot = var_q_dep0_dn0;
        *var_q_dep0_dn10_slot = var_q_dep0_dn10;
        *var_q_dep0_dn11_slot = var_q_dep0_dn11;
        *var_q_dep0_dn12_slot = var_q_dep0_dn12;
        *var_q_dep0_dn17_slot = var_q_dep0_dn17;
        *var_q_dep0_dn2_slot = var_q_dep0_dn2;
        *var_q_dep0_dn6_slot = var_q_dep0_dn6;
        *var_q_dep0_dn7_slot = var_q_dep0_dn7;
        *var_q_dep0_rv_slot = var_q_dep0_rv;
        *var_q_dep_soi_slot = var_q_dep_soi;
        *var_q_dep_soi_dn0_slot = var_q_dep_soi_dn0;
        *var_q_dep_soi_dn10_slot = var_q_dep_soi_dn10;
        *var_q_dep_soi_dn11_slot = var_q_dep_soi_dn11;
        *var_q_dep_soi_dn12_slot = var_q_dep_soi_dn12;
        *var_q_dep_soi_dn17_slot = var_q_dep_soi_dn17;
        *var_q_dep_soi_dn2_slot = var_q_dep_soi_dn2;
        *var_q_dep_soi_dn6_slot = var_q_dep_soi_dn6;
        *var_q_dep_soi_dn7_slot = var_q_dep_soi_dn7;
        *var_q_dep_soi_rv_slot = var_q_dep_soi_rv;
        *var_q_depl_slot = var_q_depl;
        *var_q_depl_dn0_slot = var_q_depl_dn0;
        *var_q_depl_dn10_slot = var_q_depl_dn10;
        *var_q_depl_dn11_slot = var_q_depl_dn11;
        *var_q_depl_dn12_slot = var_q_depl_dn12;
        *var_q_depl_dn17_slot = var_q_depl_dn17;
        *var_q_depl_dn2_slot = var_q_depl_dn2;
        *var_q_depl_dn6_slot = var_q_depl_dn6;
        *var_q_depl_dn7_slot = var_q_depl_dn7;
        *var_q_depl_rv_slot = var_q_depl_rv;
        *var_q_n0_slot = var_q_n0;
        *var_q_n0_dn0_slot = var_q_n0_dn0;
        *var_q_n0_dn10_slot = var_q_n0_dn10;
        *var_q_n0_dn11_slot = var_q_n0_dn11;
        *var_q_n0_dn12_slot = var_q_n0_dn12;
        *var_q_n0_dn17_slot = var_q_n0_dn17;
        *var_q_n0_dn2_slot = var_q_n0_dn2;
        *var_q_n0_dn6_slot = var_q_n0_dn6;
        *var_q_n0_dn7_slot = var_q_n0_dn7;
        *var_q_n0_rv_slot = var_q_n0_rv;
        *var_q_nl_slot = var_q_nl;
        *var_q_nl_dn0_slot = var_q_nl_dn0;
        *var_q_nl_dn10_slot = var_q_nl_dn10;
        *var_q_nl_dn11_slot = var_q_nl_dn11;
        *var_q_nl_dn12_slot = var_q_nl_dn12;
        *var_q_nl_dn17_slot = var_q_nl_dn17;
        *var_q_nl_dn2_slot = var_q_nl_dn2;
        *var_q_nl_dn6_slot = var_q_nl_dn6;
        *var_q_nl_dn7_slot = var_q_nl_dn7;
        *var_q_nl_rv_slot = var_q_nl_rv;
        *var_q_s0_bulk_slot = var_q_s0_bulk;
        *var_q_s0_bulk_0_slot = var_q_s0_bulk_0;
        *var_q_s0_bulk_0_dn0_slot = var_q_s0_bulk_0_dn0;
        *var_q_s0_bulk_0_dn10_slot = var_q_s0_bulk_0_dn10;
        *var_q_s0_bulk_0_dn11_slot = var_q_s0_bulk_0_dn11;
        *var_q_s0_bulk_0_dn12_slot = var_q_s0_bulk_0_dn12;
        *var_q_s0_bulk_0_dn17_slot = var_q_s0_bulk_0_dn17;
        *var_q_s0_bulk_0_dn2_slot = var_q_s0_bulk_0_dn2;
        *var_q_s0_bulk_0_dn6_slot = var_q_s0_bulk_0_dn6;
        *var_q_s0_bulk_0_dn7_slot = var_q_s0_bulk_0_dn7;
        *var_q_s0_bulk_0_rv_slot = var_q_s0_bulk_0_rv;
        *var_q_s0_bulk_dn0_slot = var_q_s0_bulk_dn0;
        *var_q_s0_bulk_dn10_slot = var_q_s0_bulk_dn10;
        *var_q_s0_bulk_dn11_slot = var_q_s0_bulk_dn11;
        *var_q_s0_bulk_dn12_slot = var_q_s0_bulk_dn12;
        *var_q_s0_bulk_dn17_slot = var_q_s0_bulk_dn17;
        *var_q_s0_bulk_dn2_slot = var_q_s0_bulk_dn2;
        *var_q_s0_bulk_dn6_slot = var_q_s0_bulk_dn6;
        *var_q_s0_bulk_dn7_slot = var_q_s0_bulk_dn7;
        *var_q_s0_bulk_rv_slot = var_q_s0_bulk_rv;
        *var_q_sl_bulk_slot = var_q_sl_bulk;
        *var_q_sl_bulk_dn0_slot = var_q_sl_bulk_dn0;
        *var_q_sl_bulk_dn10_slot = var_q_sl_bulk_dn10;
        *var_q_sl_bulk_dn11_slot = var_q_sl_bulk_dn11;
        *var_q_sl_bulk_dn12_slot = var_q_sl_bulk_dn12;
        *var_q_sl_bulk_dn17_slot = var_q_sl_bulk_dn17;
        *var_q_sl_bulk_dn2_slot = var_q_sl_bulk_dn2;
        *var_q_sl_bulk_dn6_slot = var_q_sl_bulk_dn6;
        *var_q_sl_bulk_dn7_slot = var_q_sl_bulk_dn7;
        *var_q_sl_bulk_rv_slot = var_q_sl_bulk_rv;
        *var_qbu_slot = var_qbu;
        *var_qbu_dn0_slot = var_qbu_dn0;
        *var_qbu_dn10_slot = var_qbu_dn10;
        *var_qbu_dn11_slot = var_qbu_dn11;
        *var_qbu_dn12_slot = var_qbu_dn12;
        *var_qbu_dn17_slot = var_qbu_dn17;
        *var_qbu_dn2_slot = var_qbu_dn2;
        *var_qbu_dn6_slot = var_qbu_dn6;
        *var_qbu_dn7_slot = var_qbu_dn7;
        *var_qbu_rv_slot = var_qbu_rv;
        *var_qd_fb_slot = var_qd_fb;
        *var_qd_fb_dn0_slot = var_qd_fb_dn0;
        *var_qd_fb_dn10_slot = var_qd_fb_dn10;
        *var_qd_fb_dn11_slot = var_qd_fb_dn11;
        *var_qd_fb_dn12_slot = var_qd_fb_dn12;
        *var_qd_fb_dn13_slot = var_qd_fb_dn13;
        *var_qd_fb_dn15_slot = var_qd_fb_dn15;
        *var_qd_fb_dn16_slot = var_qd_fb_dn16;
        *var_qd_fb_dn17_slot = var_qd_fb_dn17;
        *var_qd_fb_dn18_slot = var_qd_fb_dn18;
        *var_qd_fb_dn2_slot = var_qd_fb_dn2;
        *var_qd_fb_dn6_slot = var_qd_fb_dn6;
        *var_qd_fb_dn7_slot = var_qd_fb_dn7;
        *var_qd_fb_rv_slot = var_qd_fb_rv;
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
        *var_qd_nqs_rv_slot = var_qd_nqs_rv;
        *var_qdrat_slot = var_qdrat;
        *var_qdrat_dn0_slot = var_qdrat_dn0;
        *var_qdrat_dn10_slot = var_qdrat_dn10;
        *var_qdrat_dn11_slot = var_qdrat_dn11;
        *var_qdrat_dn12_slot = var_qdrat_dn12;
        *var_qdrat_dn17_slot = var_qdrat_dn17;
        *var_qdrat_dn2_slot = var_qdrat_dn2;
        *var_qdrat_dn6_slot = var_qdrat_dn6;
        *var_qdrat_dn7_slot = var_qdrat_dn7;
        *var_qdrat_noi_slot = var_qdrat_noi;
        *var_qdrat_noi_dn0_slot = var_qdrat_noi_dn0;
        *var_qdrat_noi_dn10_slot = var_qdrat_noi_dn10;
        *var_qdrat_noi_dn11_slot = var_qdrat_noi_dn11;
        *var_qdrat_noi_dn12_slot = var_qdrat_noi_dn12;
        *var_qdrat_noi_dn17_slot = var_qdrat_noi_dn17;
        *var_qdrat_noi_dn2_slot = var_qdrat_noi_dn2;
        *var_qdrat_noi_dn6_slot = var_qdrat_noi_dn6;
        *var_qdrat_noi_dn7_slot = var_qdrat_noi_dn7;
        *var_qdrat_noi_rv_slot = var_qdrat_noi_rv;
        *var_qdrat_rv_slot = var_qdrat_rv;
        *var_qhs_slot = var_qhs;
        *var_qhs_dn0_slot = var_qhs_dn0;
        *var_qhs_dn10_slot = var_qhs_dn10;
        *var_qhs_dn11_slot = var_qhs_dn11;
        *var_qhs_dn12_slot = var_qhs_dn12;
        *var_qhs_dn17_slot = var_qhs_dn17;
        *var_qhs_dn2_slot = var_qhs_dn2;
        *var_qhs_dn6_slot = var_qhs_dn6;
        *var_qhs_dn7_slot = var_qhs_dn7;
        *var_qhs_rv_slot = var_qhs_rv;
        *var_qi_nqs_slot = var_qi_nqs;
        *var_qi_nqs_dn18_slot = var_qi_nqs_dn18;
        *var_qi_nqs_rv_slot = var_qi_nqs_rv;
        *var_qiu_slot = var_qiu;
        *var_qiu_dn0_slot = var_qiu_dn0;
        *var_qiu_dn10_slot = var_qiu_dn10;
        *var_qiu_dn11_slot = var_qiu_dn11;
        *var_qiu_dn12_slot = var_qiu_dn12;
        *var_qiu_dn17_slot = var_qiu_dn17;
        *var_qiu_dn2_slot = var_qiu_dn2;
        *var_qiu_dn6_slot = var_qiu_dn6;
        *var_qiu_dn7_slot = var_qiu_dn7;
        *var_qiu_rv_slot = var_qiu_rv;
        *var_qs_fb_slot = var_qs_fb;
        *var_qs_fb_dn0_slot = var_qs_fb_dn0;
        *var_qs_fb_dn10_slot = var_qs_fb_dn10;
        *var_qs_fb_dn11_slot = var_qs_fb_dn11;
        *var_qs_fb_dn12_slot = var_qs_fb_dn12;
        *var_qs_fb_dn13_slot = var_qs_fb_dn13;
        *var_qs_fb_dn15_slot = var_qs_fb_dn15;
        *var_qs_fb_dn16_slot = var_qs_fb_dn16;
        *var_qs_fb_dn17_slot = var_qs_fb_dn17;
        *var_qs_fb_dn18_slot = var_qs_fb_dn18;
        *var_qs_fb_dn2_slot = var_qs_fb_dn2;
        *var_qs_fb_dn6_slot = var_qs_fb_dn6;
        *var_qs_fb_dn7_slot = var_qs_fb_dn7;
        *var_qs_fb_rv_slot = var_qs_fb_rv;
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
        *var_qs_nqs_rv_slot = var_qs_nqs_rv;
        *var_qsub_slot = var_qsub;
        *var_qsub_dn0_slot = var_qsub_dn0;
        *var_qsub_dn10_slot = var_qsub_dn10;
        *var_qsub_dn11_slot = var_qsub_dn11;
        *var_qsub_dn12_slot = var_qsub_dn12;
        *var_qsub_dn17_slot = var_qsub_dn17;
        *var_qsub_dn2_slot = var_qsub_dn2;
        *var_qsub_dn6_slot = var_qsub_dn6;
        *var_qsub_dn7_slot = var_qsub_dn7;
        *var_qsub_rv_slot = var_qsub_rv;
        *var_shift_slot = var_shift;
        *var_shift_dn0_slot = var_shift_dn0;
        *var_shift_dn10_slot = var_shift_dn10;
        *var_shift_dn11_slot = var_shift_dn11;
        *var_shift_dn12_slot = var_shift_dn12;
        *var_shift_dn17_slot = var_shift_dn17;
        *var_shift_dn2_slot = var_shift_dn2;
        *var_shift_dn6_slot = var_shift_dn6;
        *var_shift_dn7_slot = var_shift_dn7;
        *var_shift_rv_slot = var_shift_rv;
        *var_sqrtkusail_slot = var_sqrtkusail;
        *var_sqrtkusail_dn0_slot = var_sqrtkusail_dn0;
        *var_sqrtkusail_dn10_slot = var_sqrtkusail_dn10;
        *var_sqrtkusail_dn11_slot = var_sqrtkusail_dn11;
        *var_sqrtkusail_dn12_slot = var_sqrtkusail_dn12;
        *var_sqrtkusail_dn17_slot = var_sqrtkusail_dn17;
        *var_sqrtkusail_dn2_slot = var_sqrtkusail_dn2;
        *var_sqrtkusail_dn6_slot = var_sqrtkusail_dn6;
        *var_sqrtkusail_dn7_slot = var_sqrtkusail_dn7;
        *var_sqrtkusail_rv_slot = var_sqrtkusail_rv;
        *var_uc_areabt_slot = var_uc_areabt;
        *var_uc_areabt_rv_slot = var_uc_areabt_rv;
        *var_uc_vfbbt_slot = var_uc_vfbbt;
        *var_uc_vfbbt_rv_slot = var_uc_vfbbt_rv;
        *var_wdsoi_slot = var_wdsoi;
        *var_wdsoi_dn0_slot = var_wdsoi_dn0;
        *var_wdsoi_dn10_slot = var_wdsoi_dn10;
        *var_wdsoi_dn11_slot = var_wdsoi_dn11;
        *var_wdsoi_dn12_slot = var_wdsoi_dn12;
        *var_wdsoi_dn17_slot = var_wdsoi_dn17;
        *var_wdsoi_dn2_slot = var_wdsoi_dn2;
        *var_wdsoi_dn6_slot = var_wdsoi_dn6;
        *var_wdsoi_dn7_slot = var_wdsoi_dn7;
        *var_wdsoi_rv_slot = var_wdsoi_rv;
    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn11_slot: &mut f64,
        var_arg_dn12_slot: &mut f64,
        var_arg_dn17_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn11_slot: &mut f64,
        var_dnm_dn12_slot: &mut f64,
        var_dnm_dn17_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_rv_slot: &mut f64,
        var_flg_info_slot: &mut f64,
        var_flg_info_rv_slot: &mut f64,
        var_flg_nqs_slot: &mut f64,
        var_flg_nqs_rv_slot: &mut f64,
        var_flg_skipacc_slot: &mut f64,
        var_flg_skipacc_rv_slot: &mut f64,
        var_fs01_slot: &mut f64,
        var_fs01_dn0_slot: &mut f64,
        var_fs01_dn10_slot: &mut f64,
        var_fs01_dn11_slot: &mut f64,
        var_fs01_dn12_slot: &mut f64,
        var_fs01_dn17_slot: &mut f64,
        var_fs01_dn2_slot: &mut f64,
        var_fs01_dn6_slot: &mut f64,
        var_fs01_dn7_slot: &mut f64,
        var_fs01_rv_slot: &mut f64,
        var_fs02_slot: &mut f64,
        var_fs02_dn0_slot: &mut f64,
        var_fs02_dn10_slot: &mut f64,
        var_fs02_dn11_slot: &mut f64,
        var_fs02_dn12_slot: &mut f64,
        var_fs02_dn17_slot: &mut f64,
        var_fs02_dn2_slot: &mut f64,
        var_fs02_dn6_slot: &mut f64,
        var_fs02_dn7_slot: &mut f64,
        var_fs02_rv_slot: &mut f64,
        var_fsl1_slot: &mut f64,
        var_fsl1_dn0_slot: &mut f64,
        var_fsl1_dn10_slot: &mut f64,
        var_fsl1_dn11_slot: &mut f64,
        var_fsl1_dn12_slot: &mut f64,
        var_fsl1_dn17_slot: &mut f64,
        var_fsl1_dn2_slot: &mut f64,
        var_fsl1_dn6_slot: &mut f64,
        var_fsl1_dn7_slot: &mut f64,
        var_fsl1_rv_slot: &mut f64,
        var_fsl2_slot: &mut f64,
        var_fsl2_dn0_slot: &mut f64,
        var_fsl2_dn10_slot: &mut f64,
        var_fsl2_dn11_slot: &mut f64,
        var_fsl2_dn12_slot: &mut f64,
        var_fsl2_dn17_slot: &mut f64,
        var_fsl2_dn2_slot: &mut f64,
        var_fsl2_dn6_slot: &mut f64,
        var_fsl2_dn7_slot: &mut f64,
        var_fsl2_rv_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard6_rv_slot: &mut f64,
        var_idspt0_slot: &mut f64,
        var_idspt0_dn0_slot: &mut f64,
        var_idspt0_dn10_slot: &mut f64,
        var_idspt0_dn11_slot: &mut f64,
        var_idspt0_dn12_slot: &mut f64,
        var_idspt0_dn17_slot: &mut f64,
        var_idspt0_dn2_slot: &mut f64,
        var_idspt0_dn6_slot: &mut f64,
        var_idspt0_dn7_slot: &mut f64,
        var_idspt0_rv_slot: &mut f64,
        var_lp_s0_max_slot: &mut f64,
        var_lp_s0_max_rv_slot: &mut f64,
        var_lp_sl_max_slot: &mut f64,
        var_lp_sl_max_rv_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_m0_rv_slot: &mut f64,
        var_mks_cth0_slot: &mut f64,
        var_mks_cth0_rv_slot: &mut f64,
        var_mks_njunc_slot: &mut f64,
        var_mks_njunc_rv_slot: &mut f64,
        var_mks_nover_slot: &mut f64,
        var_mks_nover_rv_slot: &mut f64,
        var_mks_nsti_slot: &mut f64,
        var_mks_nsti_rv_slot: &mut f64,
        var_mks_nsubb_slot: &mut f64,
        var_mks_nsubb_rv_slot: &mut f64,
        var_mks_nsubcmax_slot: &mut f64,
        var_mks_nsubcmax_rv_slot: &mut f64,
        var_mks_nsubp_slot: &mut f64,
        var_mks_nsubp_rv_slot: &mut f64,
        var_mks_nsubs_slot: &mut f64,
        var_mks_nsubs_rv_slot: &mut f64,
        var_mks_parl1_slot: &mut f64,
        var_mks_parl1_rv_slot: &mut f64,
        var_mks_rth0_slot: &mut f64,
        var_mks_rth0_rv_slot: &mut f64,
        var_mks_vmax_slot: &mut f64,
        var_mks_vmax_rv_slot: &mut f64,
        var_mks_vtmp_slot: &mut f64,
        var_mks_vtmp_rv_slot: &mut f64,
        var_mks_wfc_slot: &mut f64,
        var_mks_wfc_rv_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_mm_rv_slot: &mut f64,
        var_ps0_slot: &mut f64,
        var_ps0_dn0_slot: &mut f64,
        var_ps0_dn10_slot: &mut f64,
        var_ps0_dn11_slot: &mut f64,
        var_ps0_dn12_slot: &mut f64,
        var_ps0_dn17_slot: &mut f64,
        var_ps0_dn2_slot: &mut f64,
        var_ps0_dn6_slot: &mut f64,
        var_ps0_dn7_slot: &mut f64,
        var_ps0_ini_slot: &mut f64,
        var_ps0_ini_dn0_slot: &mut f64,
        var_ps0_ini_dn10_slot: &mut f64,
        var_ps0_ini_dn11_slot: &mut f64,
        var_ps0_ini_dn12_slot: &mut f64,
        var_ps0_ini_dn17_slot: &mut f64,
        var_ps0_ini_dn2_slot: &mut f64,
        var_ps0_ini_dn6_slot: &mut f64,
        var_ps0_ini_dn7_slot: &mut f64,
        var_ps0_ini_rv_slot: &mut f64,
        var_ps0_rv_slot: &mut f64,
        var_q_s0_dep_ini_slot: &mut f64,
        var_q_s0_dep_ini_dn0_slot: &mut f64,
        var_q_s0_dep_ini_dn10_slot: &mut f64,
        var_q_s0_dep_ini_dn11_slot: &mut f64,
        var_q_s0_dep_ini_dn12_slot: &mut f64,
        var_q_s0_dep_ini_dn17_slot: &mut f64,
        var_q_s0_dep_ini_dn2_slot: &mut f64,
        var_q_s0_dep_ini_dn6_slot: &mut f64,
        var_q_s0_dep_ini_dn7_slot: &mut f64,
        var_q_s0_dep_ini_rv_slot: &mut f64,
        var_subversion_slot: &mut f64,
        var_subversion_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_uc_clm2_slot: &mut f64,
        var_uc_clm2_dn0_slot: &mut f64,
        var_uc_clm2_dn10_slot: &mut f64,
        var_uc_clm2_dn11_slot: &mut f64,
        var_uc_clm2_dn12_slot: &mut f64,
        var_uc_clm2_dn17_slot: &mut f64,
        var_uc_clm2_dn2_slot: &mut f64,
        var_uc_clm2_dn6_slot: &mut f64,
        var_uc_clm2_dn7_slot: &mut f64,
        var_uc_clm2_rv_slot: &mut f64,
        var_uc_gdld_slot: &mut f64,
        var_uc_gdld_rv_slot: &mut f64,
        var_uc_sc2_slot: &mut f64,
        var_uc_sc2_rv_slot: &mut f64,
        var_uc_sc3_slot: &mut f64,
        var_uc_sc3_rv_slot: &mut f64,
        var_uc_scp2_slot: &mut f64,
        var_uc_scp2_rv_slot: &mut f64,
        var_uc_scp3_slot: &mut f64,
        var_uc_scp3_rv_slot: &mut f64,
        var_uc_tnom_slot: &mut f64,
        var_uc_tnom_rv_slot: &mut f64,
        var_uc_vfbover_slot: &mut f64,
        var_uc_vfbover_rv_slot: &mut f64,
        var_vbcs_cl_slot: &mut f64,
        var_vbcs_cl_dn0_slot: &mut f64,
        var_vbcs_cl_dn10_slot: &mut f64,
        var_vbcs_cl_dn11_slot: &mut f64,
        var_vbcs_cl_dn12_slot: &mut f64,
        var_vbcs_cl_dn17_slot: &mut f64,
        var_vbcs_cl_dn2_slot: &mut f64,
        var_vbcs_cl_dn6_slot: &mut f64,
        var_vbcs_cl_dn7_slot: &mut f64,
        var_vbcs_cl_rv_slot: &mut f64,
        var_vbsbiz_slot: &mut f64,
        var_vbsbiz_dn0_slot: &mut f64,
        var_vbsbiz_dn10_slot: &mut f64,
        var_vbsbiz_dn11_slot: &mut f64,
        var_vbsbiz_dn12_slot: &mut f64,
        var_vbsbiz_dn17_slot: &mut f64,
        var_vbsbiz_dn2_slot: &mut f64,
        var_vbsbiz_dn6_slot: &mut f64,
        var_vbsbiz_dn7_slot: &mut f64,
        var_vbsbiz_rv_slot: &mut f64,
        var_x2_slot: &mut f64,
        var_x2_dn0_slot: &mut f64,
        var_x2_dn10_slot: &mut f64,
        var_x2_dn11_slot: &mut f64,
        var_x2_dn12_slot: &mut f64,
        var_x2_dn17_slot: &mut f64,
        var_x2_dn2_slot: &mut f64,
        var_x2_dn6_slot: &mut f64,
        var_x2_dn7_slot: &mut f64,
        var_x2_rv_slot: &mut f64,
        var_xmax2_slot: &mut f64,
        var_xmax2_dn0_slot: &mut f64,
        var_xmax2_dn10_slot: &mut f64,
        var_xmax2_dn11_slot: &mut f64,
        var_xmax2_dn12_slot: &mut f64,
        var_xmax2_dn17_slot: &mut f64,
        var_xmax2_dn2_slot: &mut f64,
        var_xmax2_dn6_slot: &mut f64,
        var_xmax2_dn7_slot: &mut f64,
        var_xmax2_rv_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn11_slot: &mut f64,
        var_xmp_dn12_slot: &mut f64,
        var_xmp_dn17_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn7_slot: &mut f64,
        var_xmp_rv_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn11_slot: &mut f64,
        var_xp_dn12_slot: &mut f64,
        var_xp_dn17_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
        var_xp_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn11: f64 = *var_arg_dn11_slot;
        let mut var_arg_dn12: f64 = *var_arg_dn12_slot;
        let mut var_arg_dn17: f64 = *var_arg_dn17_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn11: f64 = *var_dnm_dn11_slot;
        let mut var_dnm_dn12: f64 = *var_dnm_dn12_slot;
        let mut var_dnm_dn17: f64 = *var_dnm_dn17_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_rv: f64 = *var_dnm_rv_slot;
        let mut var_flg_info: f64 = *var_flg_info_slot;
        let mut var_flg_info_rv: f64 = *var_flg_info_rv_slot;
        let mut var_flg_nqs: f64 = *var_flg_nqs_slot;
        let mut var_flg_nqs_rv: f64 = *var_flg_nqs_rv_slot;
        let mut var_flg_skipacc: f64 = *var_flg_skipacc_slot;
        let mut var_flg_skipacc_rv: f64 = *var_flg_skipacc_rv_slot;
        let mut var_fs01: f64 = *var_fs01_slot;
        let mut var_fs01_dn0: f64 = *var_fs01_dn0_slot;
        let mut var_fs01_dn10: f64 = *var_fs01_dn10_slot;
        let mut var_fs01_dn11: f64 = *var_fs01_dn11_slot;
        let mut var_fs01_dn12: f64 = *var_fs01_dn12_slot;
        let mut var_fs01_dn17: f64 = *var_fs01_dn17_slot;
        let mut var_fs01_dn2: f64 = *var_fs01_dn2_slot;
        let mut var_fs01_dn6: f64 = *var_fs01_dn6_slot;
        let mut var_fs01_dn7: f64 = *var_fs01_dn7_slot;
        let mut var_fs01_rv: f64 = *var_fs01_rv_slot;
        let mut var_fs02: f64 = *var_fs02_slot;
        let mut var_fs02_dn0: f64 = *var_fs02_dn0_slot;
        let mut var_fs02_dn10: f64 = *var_fs02_dn10_slot;
        let mut var_fs02_dn11: f64 = *var_fs02_dn11_slot;
        let mut var_fs02_dn12: f64 = *var_fs02_dn12_slot;
        let mut var_fs02_dn17: f64 = *var_fs02_dn17_slot;
        let mut var_fs02_dn2: f64 = *var_fs02_dn2_slot;
        let mut var_fs02_dn6: f64 = *var_fs02_dn6_slot;
        let mut var_fs02_dn7: f64 = *var_fs02_dn7_slot;
        let mut var_fs02_rv: f64 = *var_fs02_rv_slot;
        let mut var_fsl1: f64 = *var_fsl1_slot;
        let mut var_fsl1_dn0: f64 = *var_fsl1_dn0_slot;
        let mut var_fsl1_dn10: f64 = *var_fsl1_dn10_slot;
        let mut var_fsl1_dn11: f64 = *var_fsl1_dn11_slot;
        let mut var_fsl1_dn12: f64 = *var_fsl1_dn12_slot;
        let mut var_fsl1_dn17: f64 = *var_fsl1_dn17_slot;
        let mut var_fsl1_dn2: f64 = *var_fsl1_dn2_slot;
        let mut var_fsl1_dn6: f64 = *var_fsl1_dn6_slot;
        let mut var_fsl1_dn7: f64 = *var_fsl1_dn7_slot;
        let mut var_fsl1_rv: f64 = *var_fsl1_rv_slot;
        let mut var_fsl2: f64 = *var_fsl2_slot;
        let mut var_fsl2_dn0: f64 = *var_fsl2_dn0_slot;
        let mut var_fsl2_dn10: f64 = *var_fsl2_dn10_slot;
        let mut var_fsl2_dn11: f64 = *var_fsl2_dn11_slot;
        let mut var_fsl2_dn12: f64 = *var_fsl2_dn12_slot;
        let mut var_fsl2_dn17: f64 = *var_fsl2_dn17_slot;
        let mut var_fsl2_dn2: f64 = *var_fsl2_dn2_slot;
        let mut var_fsl2_dn6: f64 = *var_fsl2_dn6_slot;
        let mut var_fsl2_dn7: f64 = *var_fsl2_dn7_slot;
        let mut var_fsl2_rv: f64 = *var_fsl2_rv_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard6_rv: f64 = *var_guard6_rv_slot;
        let mut var_idspt0: f64 = *var_idspt0_slot;
        let mut var_idspt0_dn0: f64 = *var_idspt0_dn0_slot;
        let mut var_idspt0_dn10: f64 = *var_idspt0_dn10_slot;
        let mut var_idspt0_dn11: f64 = *var_idspt0_dn11_slot;
        let mut var_idspt0_dn12: f64 = *var_idspt0_dn12_slot;
        let mut var_idspt0_dn17: f64 = *var_idspt0_dn17_slot;
        let mut var_idspt0_dn2: f64 = *var_idspt0_dn2_slot;
        let mut var_idspt0_dn6: f64 = *var_idspt0_dn6_slot;
        let mut var_idspt0_dn7: f64 = *var_idspt0_dn7_slot;
        let mut var_idspt0_rv: f64 = *var_idspt0_rv_slot;
        let mut var_lp_s0_max: f64 = *var_lp_s0_max_slot;
        let mut var_lp_s0_max_rv: f64 = *var_lp_s0_max_rv_slot;
        let mut var_lp_sl_max: f64 = *var_lp_sl_max_slot;
        let mut var_lp_sl_max_rv: f64 = *var_lp_sl_max_rv_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_m0_rv: f64 = *var_m0_rv_slot;
        let mut var_mks_cth0: f64 = *var_mks_cth0_slot;
        let mut var_mks_cth0_rv: f64 = *var_mks_cth0_rv_slot;
        let mut var_mks_njunc: f64 = *var_mks_njunc_slot;
        let mut var_mks_njunc_rv: f64 = *var_mks_njunc_rv_slot;
        let mut var_mks_nover: f64 = *var_mks_nover_slot;
        let mut var_mks_nover_rv: f64 = *var_mks_nover_rv_slot;
        let mut var_mks_nsti: f64 = *var_mks_nsti_slot;
        let mut var_mks_nsti_rv: f64 = *var_mks_nsti_rv_slot;
        let mut var_mks_nsubb: f64 = *var_mks_nsubb_slot;
        let mut var_mks_nsubb_rv: f64 = *var_mks_nsubb_rv_slot;
        let mut var_mks_nsubcmax: f64 = *var_mks_nsubcmax_slot;
        let mut var_mks_nsubcmax_rv: f64 = *var_mks_nsubcmax_rv_slot;
        let mut var_mks_nsubp: f64 = *var_mks_nsubp_slot;
        let mut var_mks_nsubp_rv: f64 = *var_mks_nsubp_rv_slot;
        let mut var_mks_nsubs: f64 = *var_mks_nsubs_slot;
        let mut var_mks_nsubs_rv: f64 = *var_mks_nsubs_rv_slot;
        let mut var_mks_parl1: f64 = *var_mks_parl1_slot;
        let mut var_mks_parl1_rv: f64 = *var_mks_parl1_rv_slot;
        let mut var_mks_rth0: f64 = *var_mks_rth0_slot;
        let mut var_mks_rth0_rv: f64 = *var_mks_rth0_rv_slot;
        let mut var_mks_vmax: f64 = *var_mks_vmax_slot;
        let mut var_mks_vmax_rv: f64 = *var_mks_vmax_rv_slot;
        let mut var_mks_vtmp: f64 = *var_mks_vtmp_slot;
        let mut var_mks_vtmp_rv: f64 = *var_mks_vtmp_rv_slot;
        let mut var_mks_wfc: f64 = *var_mks_wfc_slot;
        let mut var_mks_wfc_rv: f64 = *var_mks_wfc_rv_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_mm_rv: f64 = *var_mm_rv_slot;
        let mut var_ps0: f64 = *var_ps0_slot;
        let mut var_ps0_dn0: f64 = *var_ps0_dn0_slot;
        let mut var_ps0_dn10: f64 = *var_ps0_dn10_slot;
        let mut var_ps0_dn11: f64 = *var_ps0_dn11_slot;
        let mut var_ps0_dn12: f64 = *var_ps0_dn12_slot;
        let mut var_ps0_dn17: f64 = *var_ps0_dn17_slot;
        let mut var_ps0_dn2: f64 = *var_ps0_dn2_slot;
        let mut var_ps0_dn6: f64 = *var_ps0_dn6_slot;
        let mut var_ps0_dn7: f64 = *var_ps0_dn7_slot;
        let mut var_ps0_ini: f64 = *var_ps0_ini_slot;
        let mut var_ps0_ini_dn0: f64 = *var_ps0_ini_dn0_slot;
        let mut var_ps0_ini_dn10: f64 = *var_ps0_ini_dn10_slot;
        let mut var_ps0_ini_dn11: f64 = *var_ps0_ini_dn11_slot;
        let mut var_ps0_ini_dn12: f64 = *var_ps0_ini_dn12_slot;
        let mut var_ps0_ini_dn17: f64 = *var_ps0_ini_dn17_slot;
        let mut var_ps0_ini_dn2: f64 = *var_ps0_ini_dn2_slot;
        let mut var_ps0_ini_dn6: f64 = *var_ps0_ini_dn6_slot;
        let mut var_ps0_ini_dn7: f64 = *var_ps0_ini_dn7_slot;
        let mut var_ps0_ini_rv: f64 = *var_ps0_ini_rv_slot;
        let mut var_ps0_rv: f64 = *var_ps0_rv_slot;
        let mut var_q_s0_dep_ini: f64 = *var_q_s0_dep_ini_slot;
        let mut var_q_s0_dep_ini_dn0: f64 = *var_q_s0_dep_ini_dn0_slot;
        let mut var_q_s0_dep_ini_dn10: f64 = *var_q_s0_dep_ini_dn10_slot;
        let mut var_q_s0_dep_ini_dn11: f64 = *var_q_s0_dep_ini_dn11_slot;
        let mut var_q_s0_dep_ini_dn12: f64 = *var_q_s0_dep_ini_dn12_slot;
        let mut var_q_s0_dep_ini_dn17: f64 = *var_q_s0_dep_ini_dn17_slot;
        let mut var_q_s0_dep_ini_dn2: f64 = *var_q_s0_dep_ini_dn2_slot;
        let mut var_q_s0_dep_ini_dn6: f64 = *var_q_s0_dep_ini_dn6_slot;
        let mut var_q_s0_dep_ini_dn7: f64 = *var_q_s0_dep_ini_dn7_slot;
        let mut var_q_s0_dep_ini_rv: f64 = *var_q_s0_dep_ini_rv_slot;
        let mut var_subversion: f64 = *var_subversion_slot;
        let mut var_subversion_rv: f64 = *var_subversion_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_uc_clm2: f64 = *var_uc_clm2_slot;
        let mut var_uc_clm2_dn0: f64 = *var_uc_clm2_dn0_slot;
        let mut var_uc_clm2_dn10: f64 = *var_uc_clm2_dn10_slot;
        let mut var_uc_clm2_dn11: f64 = *var_uc_clm2_dn11_slot;
        let mut var_uc_clm2_dn12: f64 = *var_uc_clm2_dn12_slot;
        let mut var_uc_clm2_dn17: f64 = *var_uc_clm2_dn17_slot;
        let mut var_uc_clm2_dn2: f64 = *var_uc_clm2_dn2_slot;
        let mut var_uc_clm2_dn6: f64 = *var_uc_clm2_dn6_slot;
        let mut var_uc_clm2_dn7: f64 = *var_uc_clm2_dn7_slot;
        let mut var_uc_clm2_rv: f64 = *var_uc_clm2_rv_slot;
        let mut var_uc_gdld: f64 = *var_uc_gdld_slot;
        let mut var_uc_gdld_rv: f64 = *var_uc_gdld_rv_slot;
        let mut var_uc_sc2: f64 = *var_uc_sc2_slot;
        let mut var_uc_sc2_rv: f64 = *var_uc_sc2_rv_slot;
        let mut var_uc_sc3: f64 = *var_uc_sc3_slot;
        let mut var_uc_sc3_rv: f64 = *var_uc_sc3_rv_slot;
        let mut var_uc_scp2: f64 = *var_uc_scp2_slot;
        let mut var_uc_scp2_rv: f64 = *var_uc_scp2_rv_slot;
        let mut var_uc_scp3: f64 = *var_uc_scp3_slot;
        let mut var_uc_scp3_rv: f64 = *var_uc_scp3_rv_slot;
        let mut var_uc_tnom: f64 = *var_uc_tnom_slot;
        let mut var_uc_tnom_rv: f64 = *var_uc_tnom_rv_slot;
        let mut var_uc_vfbover: f64 = *var_uc_vfbover_slot;
        let mut var_uc_vfbover_rv: f64 = *var_uc_vfbover_rv_slot;
        let mut var_vbcs_cl: f64 = *var_vbcs_cl_slot;
        let mut var_vbcs_cl_dn0: f64 = *var_vbcs_cl_dn0_slot;
        let mut var_vbcs_cl_dn10: f64 = *var_vbcs_cl_dn10_slot;
        let mut var_vbcs_cl_dn11: f64 = *var_vbcs_cl_dn11_slot;
        let mut var_vbcs_cl_dn12: f64 = *var_vbcs_cl_dn12_slot;
        let mut var_vbcs_cl_dn17: f64 = *var_vbcs_cl_dn17_slot;
        let mut var_vbcs_cl_dn2: f64 = *var_vbcs_cl_dn2_slot;
        let mut var_vbcs_cl_dn6: f64 = *var_vbcs_cl_dn6_slot;
        let mut var_vbcs_cl_dn7: f64 = *var_vbcs_cl_dn7_slot;
        let mut var_vbcs_cl_rv: f64 = *var_vbcs_cl_rv_slot;
        let mut var_vbsbiz: f64 = *var_vbsbiz_slot;
        let mut var_vbsbiz_dn0: f64 = *var_vbsbiz_dn0_slot;
        let mut var_vbsbiz_dn10: f64 = *var_vbsbiz_dn10_slot;
        let mut var_vbsbiz_dn11: f64 = *var_vbsbiz_dn11_slot;
        let mut var_vbsbiz_dn12: f64 = *var_vbsbiz_dn12_slot;
        let mut var_vbsbiz_dn17: f64 = *var_vbsbiz_dn17_slot;
        let mut var_vbsbiz_dn2: f64 = *var_vbsbiz_dn2_slot;
        let mut var_vbsbiz_dn6: f64 = *var_vbsbiz_dn6_slot;
        let mut var_vbsbiz_dn7: f64 = *var_vbsbiz_dn7_slot;
        let mut var_vbsbiz_rv: f64 = *var_vbsbiz_rv_slot;
        let mut var_x2: f64 = *var_x2_slot;
        let mut var_x2_dn0: f64 = *var_x2_dn0_slot;
        let mut var_x2_dn10: f64 = *var_x2_dn10_slot;
        let mut var_x2_dn11: f64 = *var_x2_dn11_slot;
        let mut var_x2_dn12: f64 = *var_x2_dn12_slot;
        let mut var_x2_dn17: f64 = *var_x2_dn17_slot;
        let mut var_x2_dn2: f64 = *var_x2_dn2_slot;
        let mut var_x2_dn6: f64 = *var_x2_dn6_slot;
        let mut var_x2_dn7: f64 = *var_x2_dn7_slot;
        let mut var_x2_rv: f64 = *var_x2_rv_slot;
        let mut var_xmax2: f64 = *var_xmax2_slot;
        let mut var_xmax2_dn0: f64 = *var_xmax2_dn0_slot;
        let mut var_xmax2_dn10: f64 = *var_xmax2_dn10_slot;
        let mut var_xmax2_dn11: f64 = *var_xmax2_dn11_slot;
        let mut var_xmax2_dn12: f64 = *var_xmax2_dn12_slot;
        let mut var_xmax2_dn17: f64 = *var_xmax2_dn17_slot;
        let mut var_xmax2_dn2: f64 = *var_xmax2_dn2_slot;
        let mut var_xmax2_dn6: f64 = *var_xmax2_dn6_slot;
        let mut var_xmax2_dn7: f64 = *var_xmax2_dn7_slot;
        let mut var_xmax2_rv: f64 = *var_xmax2_rv_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn11: f64 = *var_xmp_dn11_slot;
        let mut var_xmp_dn12: f64 = *var_xmp_dn12_slot;
        let mut var_xmp_dn17: f64 = *var_xmp_dn17_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn7: f64 = *var_xmp_dn7_slot;
        let mut var_xmp_rv: f64 = *var_xmp_rv_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn11: f64 = *var_xp_dn11_slot;
        let mut var_xp_dn12: f64 = *var_xp_dn12_slot;
        let mut var_xp_dn17: f64 = *var_xp_dn17_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_rv: f64 = *var_xp_rv_slot;

        var_fs01 = 0.0;
        var_fs01_dn0 = 0.0;
        var_fs01_dn2 = 0.0;
        var_fs01_dn6 = 0.0;
        var_fs01_dn7 = 0.0;
        var_fs01_dn10 = 0.0;
        var_fs01_dn11 = 0.0;
        var_fs01_dn12 = 0.0;
        var_fs01_dn17 = 0.0;
        var_fs01_rv = 0.0;

        var_fs02 = 0.0;
        var_fs02_dn0 = 0.0;
        var_fs02_dn2 = 0.0;
        var_fs02_dn6 = 0.0;
        var_fs02_dn7 = 0.0;
        var_fs02_dn10 = 0.0;
        var_fs02_dn11 = 0.0;
        var_fs02_dn12 = 0.0;
        var_fs02_dn17 = 0.0;
        var_fs02_rv = 0.0;

        var_fsl1 = 0.0;
        var_fsl1_dn0 = 0.0;
        var_fsl1_dn2 = 0.0;
        var_fsl1_dn6 = 0.0;
        var_fsl1_dn7 = 0.0;
        var_fsl1_dn10 = 0.0;
        var_fsl1_dn11 = 0.0;
        var_fsl1_dn12 = 0.0;
        var_fsl1_dn17 = 0.0;
        var_fsl1_rv = 0.0;

        var_fsl2 = 0.0;
        var_fsl2_dn0 = 0.0;
        var_fsl2_dn2 = 0.0;
        var_fsl2_dn6 = 0.0;
        var_fsl2_dn7 = 0.0;
        var_fsl2_dn10 = 0.0;
        var_fsl2_dn11 = 0.0;
        var_fsl2_dn12 = 0.0;
        var_fsl2_dn17 = 0.0;
        var_fsl2_rv = 0.0;

        let assign1240_e993: f64 = (p.p51 * 10.0);
        let assign1240_e995: f64 = (assign1240_e993 % 10.0);
        var_subversion = assign1240_e995;
        var_subversion_rv = 0.0;

        var_lp_s0_max = 200.0;
        var_lp_s0_max_rv = 0.0;

        var_lp_sl_max = 200.0;
        var_lp_sl_max_rv = 0.0;

        var_flg_skipacc = 0.0;
        var_flg_skipacc_rv = 0.0;

        var_vbsbiz = 0.0;
        var_vbsbiz_dn0 = 0.0;
        var_vbsbiz_dn2 = 0.0;
        var_vbsbiz_dn6 = 0.0;
        var_vbsbiz_dn7 = 0.0;
        var_vbsbiz_dn10 = 0.0;
        var_vbsbiz_dn11 = 0.0;
        var_vbsbiz_dn12 = 0.0;
        var_vbsbiz_dn17 = 0.0;
        var_vbsbiz_rv = 0.0;

        var_ps0_ini = 0.0;
        var_ps0_ini_dn0 = 0.0;
        var_ps0_ini_dn2 = 0.0;
        var_ps0_ini_dn6 = 0.0;
        var_ps0_ini_dn7 = 0.0;
        var_ps0_ini_dn10 = 0.0;
        var_ps0_ini_dn11 = 0.0;
        var_ps0_ini_dn12 = 0.0;
        var_ps0_ini_dn17 = 0.0;
        var_ps0_ini_rv = 0.0;

        var_q_s0_dep_ini = 0.0;
        var_q_s0_dep_ini_dn0 = 0.0;
        var_q_s0_dep_ini_dn2 = 0.0;
        var_q_s0_dep_ini_dn6 = 0.0;
        var_q_s0_dep_ini_dn7 = 0.0;
        var_q_s0_dep_ini_dn10 = 0.0;
        var_q_s0_dep_ini_dn11 = 0.0;
        var_q_s0_dep_ini_dn12 = 0.0;
        var_q_s0_dep_ini_dn17 = 0.0;
        var_q_s0_dep_ini_rv = 0.0;

        var_idspt0 = 0.0;
        var_idspt0_dn0 = 0.0;
        var_idspt0_dn2 = 0.0;
        var_idspt0_dn6 = 0.0;
        var_idspt0_dn7 = 0.0;
        var_idspt0_dn10 = 0.0;
        var_idspt0_dn11 = 0.0;
        var_idspt0_dn12 = 0.0;
        var_idspt0_dn17 = 0.0;
        var_idspt0_rv = 0.0;

        var_ps0 = 0.0;
        var_ps0_dn0 = 0.0;
        var_ps0_dn2 = 0.0;
        var_ps0_dn6 = 0.0;
        var_ps0_dn7 = 0.0;
        var_ps0_dn10 = 0.0;
        var_ps0_dn11 = 0.0;
        var_ps0_dn12 = 0.0;
        var_ps0_dn17 = 0.0;
        var_ps0_rv = 0.0;

        var_vbcs_cl = 0.0;
        var_vbcs_cl_dn0 = 0.0;
        var_vbcs_cl_dn2 = 0.0;
        var_vbcs_cl_dn6 = 0.0;
        var_vbcs_cl_dn7 = 0.0;
        var_vbcs_cl_dn10 = 0.0;
        var_vbcs_cl_dn11 = 0.0;
        var_vbcs_cl_dn12 = 0.0;
        var_vbcs_cl_dn17 = 0.0;
        var_vbcs_cl_rv = 0.0;

        let assign1350_e1008: f64 = (p.p52 * 0.01);
        var_mks_vmax = assign1350_e1008;
        var_mks_vmax_rv = 0.0;

        let assign1360_e1011: f64 = (p.p73 / 1e-6);
        var_mks_nsubp = assign1360_e1011;
        var_mks_nsubp_rv = 0.0;

        let assign1370_e1014: f64 = (p.p104 * 0.01);
        var_mks_vtmp = assign1370_e1014;
        var_mks_vtmp_rv = 0.0;

        let assign1380_e1017: f64 = (p.p201 / 1e-6);
        var_mks_nsubcmax = assign1380_e1017;
        var_mks_nsubcmax_rv = 0.0;

        let assign1420_e1029: f64 = (p.p240 / 1e-6);
        var_mks_nsubs = assign1420_e1029;
        var_mks_nsubs_rv = 0.0;

        let assign1430_e1032: f64 = (p.p241 / 1e-6);
        var_mks_nsubb = assign1430_e1032;
        var_mks_nsubb_rv = 0.0;

        let assign1440_e1035: f64 = (p.p242 * 0.01);
        var_mks_rth0 = assign1440_e1035;
        var_mks_rth0_rv = 0.0;

        let assign1450_e1038: f64 = (p.p243 / 0.01);
        var_mks_cth0 = assign1450_e1038;
        var_mks_cth0_rv = 0.0;

        let assign1460_e1041: f64 = (p.p59 / 1e-6);
        var_mks_nover = assign1460_e1041;
        var_mks_nover_rv = 0.0;

        let assign1470_e1044: f64 = (p.p284 / 1e-6);
        var_mks_njunc = assign1470_e1044;
        var_mks_njunc_rv = 0.0;

        let assign1480_e1047: f64 = (p.p148 / 1e-6);
        var_mks_nsti = assign1480_e1047;
        var_mks_nsti_rv = 0.0;

        let assign1490_e1050: f64 = (p.p198 / 0.0001);
        var_mks_wfc = assign1490_e1050;
        var_mks_wfc_rv = 0.0;

        let assign1500_e1053: f64 = (p.p70 * 0.01);
        var_mks_parl1 = assign1500_e1053;
        var_mks_parl1_rv = 0.0;

        let (assign1510_e1059,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p84,)
    }
};
        var_uc_sc2 = assign1510_e1059;
        var_uc_sc2_rv = 0.0;

        let (assign1520_e1065,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p85,)
    }
};
        var_uc_sc3 = assign1520_e1065;
        var_uc_sc3_rv = 0.0;

        let (assign1530_e1071,) = {
    if (p.p80 == 0.0) {
        (0.0,)
    } else {
        (p.p81,)
    }
};
        var_uc_scp2 = assign1530_e1071;
        var_uc_scp2_rv = 0.0;

        let (assign1540_e1077,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p82,)
    }
};
        var_uc_scp3 = assign1540_e1077;
        var_uc_scp3_rv = 0.0;

        let assign1550_e1080: f64 = (p.p250 * 1000000.0);
        var_uc_gdld = assign1550_e1080;
        var_uc_gdld_rv = 0.0;

        let assign1560_e1083: f64 = (p.p232 + 273.15);
        var_uc_tnom = assign1560_e1083;
        var_uc_tnom_rv = 0.0;

        var_uc_vfbover = p.p58;
        var_uc_vfbover_rv = 0.0;

        var_flg_info = p.p46;
        var_flg_info_rv = 0.0;

        var_flg_nqs = p.p34;
        var_flg_nqs_rv = 0.0;

        let (assign1610_e1098,) = {
    if param_given[190] {
        (p.p190,)
    } else {
        let assign1610_e1096: f64 = (p.p237 * p.p240);
        let assign1610_e1097: f64 = (5000000000.0 / assign1610_e1096);
        (assign1610_e1097,)
    }
};
        var_uc_clm2 = assign1610_e1098;
        var_uc_clm2_dn0 = 0.0;
        var_uc_clm2_dn2 = 0.0;
        var_uc_clm2_dn6 = 0.0;
        var_uc_clm2_dn7 = 0.0;
        var_uc_clm2_dn10 = 0.0;
        var_uc_clm2_dn11 = 0.0;
        var_uc_clm2_dn12 = 0.0;
        var_uc_clm2_dn17 = 0.0;
        var_uc_clm2_rv = 0.0;

        let assign1620_e1102: f64 = (2.0 + 0.1);
        let assign1620_e1107: f64 = if ((var_uc_clm2 < assign1620_e1102) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard6 = assign1620_e1107;
        var_guard6_rv = 0.0;

        let (assign1630_e1115, assign1630_e1115_d_n0, assign1630_e1115_d_n2, assign1630_e1115_d_n6, assign1630_e1115_d_n7, assign1630_e1115_d_n10, assign1630_e1115_d_n11, assign1630_e1115_d_n12, assign1630_e1115_d_n17,) = {
    if (var_guard6 != 0.0) {
        let assign1630_e1111: f64 = (2.0 + 0.1);
        let assign1630_e1113: f64 = (assign1630_e1111 - var_uc_clm2);
        (assign1630_e1113, (-var_uc_clm2_dn0), (-var_uc_clm2_dn2), (-var_uc_clm2_dn6), (-var_uc_clm2_dn7), (-var_uc_clm2_dn10), (-var_uc_clm2_dn11), (-var_uc_clm2_dn12), (-var_uc_clm2_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign1630_e1115;
        var_tmf1_dn0 = assign1630_e1115_d_n0;
        var_tmf1_dn2 = assign1630_e1115_d_n2;
        var_tmf1_dn6 = assign1630_e1115_d_n6;
        var_tmf1_dn7 = assign1630_e1115_d_n7;
        var_tmf1_dn10 = assign1630_e1115_d_n10;
        var_tmf1_dn11 = assign1630_e1115_d_n11;
        var_tmf1_dn12 = assign1630_e1115_d_n12;
        var_tmf1_dn17 = assign1630_e1115_d_n17;
        var_tmf1_rv = 0.0;

        let (assign1640_e1121, assign1640_e1121_d_n0, assign1640_e1121_d_n2, assign1640_e1121_d_n6, assign1640_e1121_d_n7, assign1640_e1121_d_n10, assign1640_e1121_d_n11, assign1640_e1121_d_n12, assign1640_e1121_d_n17,) = {
    if (var_guard6 != 0.0) {
        let assign1640_e1119: f64 = (var_tmf1 * var_tmf1);
        (assign1640_e1119, ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)), ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)), ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)), ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)), ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)), ((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)), ((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)), ((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn6, var_x2_dn7, var_x2_dn10, var_x2_dn11, var_x2_dn12, var_x2_dn17,)
    }
};
        var_x2 = assign1640_e1121;
        var_x2_dn0 = assign1640_e1121_d_n0;
        var_x2_dn2 = assign1640_e1121_d_n2;
        var_x2_dn6 = assign1640_e1121_d_n6;
        var_x2_dn7 = assign1640_e1121_d_n7;
        var_x2_dn10 = assign1640_e1121_d_n10;
        var_x2_dn11 = assign1640_e1121_d_n11;
        var_x2_dn12 = assign1640_e1121_d_n12;
        var_x2_dn17 = assign1640_e1121_d_n17;
        var_x2_rv = 0.0;

        let (assign1650_e1127, assign1650_e1127_d_n0, assign1650_e1127_d_n2, assign1650_e1127_d_n6, assign1650_e1127_d_n7, assign1650_e1127_d_n10, assign1650_e1127_d_n11, assign1650_e1127_d_n12, assign1650_e1127_d_n17,) = {
    if (var_guard6 != 0.0) {
        let assign1650_e1125: f64 = (0.1 * 0.1);
        (assign1650_e1125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn10, var_xmax2_dn11, var_xmax2_dn12, var_xmax2_dn17,)
    }
};
        var_xmax2 = assign1650_e1127;
        var_xmax2_dn0 = assign1650_e1127_d_n0;
        var_xmax2_dn2 = assign1650_e1127_d_n2;
        var_xmax2_dn6 = assign1650_e1127_d_n6;
        var_xmax2_dn7 = assign1650_e1127_d_n7;
        var_xmax2_dn10 = assign1650_e1127_d_n10;
        var_xmax2_dn11 = assign1650_e1127_d_n11;
        var_xmax2_dn12 = assign1650_e1127_d_n12;
        var_xmax2_dn17 = assign1650_e1127_d_n17;
        var_xmax2_rv = 0.0;

        let (assign1660_e1131, assign1660_e1131_d_n0, assign1660_e1131_d_n2, assign1660_e1131_d_n6, assign1660_e1131_d_n7, assign1660_e1131_d_n10, assign1660_e1131_d_n11, assign1660_e1131_d_n12, assign1660_e1131_d_n17,) = {
    if (var_guard6 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign1660_e1131;
        var_xp_dn0 = assign1660_e1131_d_n0;
        var_xp_dn2 = assign1660_e1131_d_n2;
        var_xp_dn6 = assign1660_e1131_d_n6;
        var_xp_dn7 = assign1660_e1131_d_n7;
        var_xp_dn10 = assign1660_e1131_d_n10;
        var_xp_dn11 = assign1660_e1131_d_n11;
        var_xp_dn12 = assign1660_e1131_d_n12;
        var_xp_dn17 = assign1660_e1131_d_n17;
        var_xp_rv = 0.0;

        let (assign1670_e1135, assign1670_e1135_d_n0, assign1670_e1135_d_n2, assign1670_e1135_d_n6, assign1670_e1135_d_n7, assign1670_e1135_d_n10, assign1670_e1135_d_n11, assign1670_e1135_d_n12, assign1670_e1135_d_n17,) = {
    if (var_guard6 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign1670_e1135;
        var_xmp_dn0 = assign1670_e1135_d_n0;
        var_xmp_dn2 = assign1670_e1135_d_n2;
        var_xmp_dn6 = assign1670_e1135_d_n6;
        var_xmp_dn7 = assign1670_e1135_d_n7;
        var_xmp_dn10 = assign1670_e1135_d_n10;
        var_xmp_dn11 = assign1670_e1135_d_n11;
        var_xmp_dn12 = assign1670_e1135_d_n12;
        var_xmp_dn17 = assign1670_e1135_d_n17;
        var_xmp_rv = 0.0;

        let (assign1680_e1139,) = {
    if (var_guard6 != 0.0) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign1680_e1139;
        var_m0_rv = 0.0;

        let (assign1690_e1143,) = {
    if (var_guard6 != 0.0) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1690_e1143;
        var_mm_rv = 0.0;

        let (assign1700_e1147, assign1700_e1147_d_n0, assign1700_e1147_d_n2, assign1700_e1147_d_n6, assign1700_e1147_d_n7, assign1700_e1147_d_n10, assign1700_e1147_d_n11, assign1700_e1147_d_n12, assign1700_e1147_d_n17,) = {
    if (var_guard6 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    }
};
        var_arg = assign1700_e1147;
        var_arg_dn0 = assign1700_e1147_d_n0;
        var_arg_dn2 = assign1700_e1147_d_n2;
        var_arg_dn6 = assign1700_e1147_d_n6;
        var_arg_dn7 = assign1700_e1147_d_n7;
        var_arg_dn10 = assign1700_e1147_d_n10;
        var_arg_dn11 = assign1700_e1147_d_n11;
        var_arg_dn12 = assign1700_e1147_d_n12;
        var_arg_dn17 = assign1700_e1147_d_n17;
        var_arg_rv = 0.0;

        let (assign1710_e1151, assign1710_e1151_d_n0, assign1710_e1151_d_n2, assign1710_e1151_d_n6, assign1710_e1151_d_n7, assign1710_e1151_d_n10, assign1710_e1151_d_n11, assign1710_e1151_d_n12, assign1710_e1151_d_n17,) = {
    if (var_guard6 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign1710_e1151;
        var_dnm_dn0 = assign1710_e1151_d_n0;
        var_dnm_dn2 = assign1710_e1151_d_n2;
        var_dnm_dn6 = assign1710_e1151_d_n6;
        var_dnm_dn7 = assign1710_e1151_d_n7;
        var_dnm_dn10 = assign1710_e1151_d_n10;
        var_dnm_dn11 = assign1710_e1151_d_n11;
        var_dnm_dn12 = assign1710_e1151_d_n12;
        var_dnm_dn17 = assign1710_e1151_d_n17;
        var_dnm_rv = 0.0;

        let (assign1720_e1157, assign1720_e1157_d_n0, assign1720_e1157_d_n2, assign1720_e1157_d_n6, assign1720_e1157_d_n7, assign1720_e1157_d_n10, assign1720_e1157_d_n11, assign1720_e1157_d_n12, assign1720_e1157_d_n17,) = {
    if (var_guard6 != 0.0) {
        let assign1720_e1155: f64 = (var_xp * var_x2);
        (assign1720_e1155, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign1720_e1157;
        var_xp_dn0 = assign1720_e1157_d_n0;
        var_xp_dn2 = assign1720_e1157_d_n2;
        var_xp_dn6 = assign1720_e1157_d_n6;
        var_xp_dn7 = assign1720_e1157_d_n7;
        var_xp_dn10 = assign1720_e1157_d_n10;
        var_xp_dn11 = assign1720_e1157_d_n11;
        var_xp_dn12 = assign1720_e1157_d_n12;
        var_xp_dn17 = assign1720_e1157_d_n17;
        var_xp_rv = 0.0;

        let (assign1730_e1163, assign1730_e1163_d_n0, assign1730_e1163_d_n2, assign1730_e1163_d_n6, assign1730_e1163_d_n7, assign1730_e1163_d_n10, assign1730_e1163_d_n11, assign1730_e1163_d_n12, assign1730_e1163_d_n17,) = {
    if (var_guard6 != 0.0) {
        let assign1730_e1161: f64 = (var_xmp * var_xmax2);
        (assign1730_e1161, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign1730_e1163;
        var_xmp_dn0 = assign1730_e1163_d_n0;
        var_xmp_dn2 = assign1730_e1163_d_n2;
        var_xmp_dn6 = assign1730_e1163_d_n6;
        var_xmp_dn7 = assign1730_e1163_d_n7;
        var_xmp_dn10 = assign1730_e1163_d_n10;
        var_xmp_dn11 = assign1730_e1163_d_n11;
        var_xmp_dn12 = assign1730_e1163_d_n12;
        var_xmp_dn17 = assign1730_e1163_d_n17;
        var_xmp_rv = 0.0;

        let (assign1740_e1169, assign1740_e1169_d_n0, assign1740_e1169_d_n2, assign1740_e1169_d_n6, assign1740_e1169_d_n7, assign1740_e1169_d_n10, assign1740_e1169_d_n11, assign1740_e1169_d_n12, assign1740_e1169_d_n17,) = {
    if (var_guard6 != 0.0) {
        let assign1740_e1167: f64 = (var_xp * var_x2);
        (assign1740_e1167, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign1740_e1169;
        var_xp_dn0 = assign1740_e1169_d_n0;
        var_xp_dn2 = assign1740_e1169_d_n2;
        var_xp_dn6 = assign1740_e1169_d_n6;
        var_xp_dn7 = assign1740_e1169_d_n7;
        var_xp_dn10 = assign1740_e1169_d_n10;
        var_xp_dn11 = assign1740_e1169_d_n11;
        var_xp_dn12 = assign1740_e1169_d_n12;
        var_xp_dn17 = assign1740_e1169_d_n17;
        var_xp_rv = 0.0;

        let (assign1750_e1175, assign1750_e1175_d_n0, assign1750_e1175_d_n2, assign1750_e1175_d_n6, assign1750_e1175_d_n7, assign1750_e1175_d_n10, assign1750_e1175_d_n11, assign1750_e1175_d_n12, assign1750_e1175_d_n17,) = {
    if (var_guard6 != 0.0) {
        let assign1750_e1173: f64 = (var_xmp * var_xmax2);
        (assign1750_e1173, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign1750_e1175;
        var_xmp_dn0 = assign1750_e1175_d_n0;
        var_xmp_dn2 = assign1750_e1175_d_n2;
        var_xmp_dn6 = assign1750_e1175_d_n6;
        var_xmp_dn7 = assign1750_e1175_d_n7;
        var_xmp_dn10 = assign1750_e1175_d_n10;
        var_xmp_dn11 = assign1750_e1175_d_n11;
        var_xmp_dn12 = assign1750_e1175_d_n12;
        var_xmp_dn17 = assign1750_e1175_d_n17;
        var_xmp_rv = 0.0;

        let (assign1760_e1181, assign1760_e1181_d_n0, assign1760_e1181_d_n2, assign1760_e1181_d_n6, assign1760_e1181_d_n7, assign1760_e1181_d_n10, assign1760_e1181_d_n11, assign1760_e1181_d_n12, assign1760_e1181_d_n17,) = {
    if (var_guard6 != 0.0) {
        let assign1760_e1179: f64 = (var_xp + var_xmp);
        (assign1760_e1179, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn11 + var_xmp_dn11), (var_xp_dn12 + var_xmp_dn12), (var_xp_dn17 + var_xmp_dn17),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    }
};
        var_arg = assign1760_e1181;
        var_arg_dn0 = assign1760_e1181_d_n0;
        var_arg_dn2 = assign1760_e1181_d_n2;
        var_arg_dn6 = assign1760_e1181_d_n6;
        var_arg_dn7 = assign1760_e1181_d_n7;
        var_arg_dn10 = assign1760_e1181_d_n10;
        var_arg_dn11 = assign1760_e1181_d_n11;
        var_arg_dn12 = assign1760_e1181_d_n12;
        var_arg_dn17 = assign1760_e1181_d_n17;
        var_arg_rv = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn11_slot = var_arg_dn11;
        *var_arg_dn12_slot = var_arg_dn12;
        *var_arg_dn17_slot = var_arg_dn17;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_rv_slot = var_arg_rv;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn11_slot = var_dnm_dn11;
        *var_dnm_dn12_slot = var_dnm_dn12;
        *var_dnm_dn17_slot = var_dnm_dn17;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_rv_slot = var_dnm_rv;
        *var_flg_info_slot = var_flg_info;
        *var_flg_info_rv_slot = var_flg_info_rv;
        *var_flg_nqs_slot = var_flg_nqs;
        *var_flg_nqs_rv_slot = var_flg_nqs_rv;
        *var_flg_skipacc_slot = var_flg_skipacc;
        *var_flg_skipacc_rv_slot = var_flg_skipacc_rv;
        *var_fs01_slot = var_fs01;
        *var_fs01_dn0_slot = var_fs01_dn0;
        *var_fs01_dn10_slot = var_fs01_dn10;
        *var_fs01_dn11_slot = var_fs01_dn11;
        *var_fs01_dn12_slot = var_fs01_dn12;
        *var_fs01_dn17_slot = var_fs01_dn17;
        *var_fs01_dn2_slot = var_fs01_dn2;
        *var_fs01_dn6_slot = var_fs01_dn6;
        *var_fs01_dn7_slot = var_fs01_dn7;
        *var_fs01_rv_slot = var_fs01_rv;
        *var_fs02_slot = var_fs02;
        *var_fs02_dn0_slot = var_fs02_dn0;
        *var_fs02_dn10_slot = var_fs02_dn10;
        *var_fs02_dn11_slot = var_fs02_dn11;
        *var_fs02_dn12_slot = var_fs02_dn12;
        *var_fs02_dn17_slot = var_fs02_dn17;
        *var_fs02_dn2_slot = var_fs02_dn2;
        *var_fs02_dn6_slot = var_fs02_dn6;
        *var_fs02_dn7_slot = var_fs02_dn7;
        *var_fs02_rv_slot = var_fs02_rv;
        *var_fsl1_slot = var_fsl1;
        *var_fsl1_dn0_slot = var_fsl1_dn0;
        *var_fsl1_dn10_slot = var_fsl1_dn10;
        *var_fsl1_dn11_slot = var_fsl1_dn11;
        *var_fsl1_dn12_slot = var_fsl1_dn12;
        *var_fsl1_dn17_slot = var_fsl1_dn17;
        *var_fsl1_dn2_slot = var_fsl1_dn2;
        *var_fsl1_dn6_slot = var_fsl1_dn6;
        *var_fsl1_dn7_slot = var_fsl1_dn7;
        *var_fsl1_rv_slot = var_fsl1_rv;
        *var_fsl2_slot = var_fsl2;
        *var_fsl2_dn0_slot = var_fsl2_dn0;
        *var_fsl2_dn10_slot = var_fsl2_dn10;
        *var_fsl2_dn11_slot = var_fsl2_dn11;
        *var_fsl2_dn12_slot = var_fsl2_dn12;
        *var_fsl2_dn17_slot = var_fsl2_dn17;
        *var_fsl2_dn2_slot = var_fsl2_dn2;
        *var_fsl2_dn6_slot = var_fsl2_dn6;
        *var_fsl2_dn7_slot = var_fsl2_dn7;
        *var_fsl2_rv_slot = var_fsl2_rv;
        *var_guard6_slot = var_guard6;
        *var_guard6_rv_slot = var_guard6_rv;
        *var_idspt0_slot = var_idspt0;
        *var_idspt0_dn0_slot = var_idspt0_dn0;
        *var_idspt0_dn10_slot = var_idspt0_dn10;
        *var_idspt0_dn11_slot = var_idspt0_dn11;
        *var_idspt0_dn12_slot = var_idspt0_dn12;
        *var_idspt0_dn17_slot = var_idspt0_dn17;
        *var_idspt0_dn2_slot = var_idspt0_dn2;
        *var_idspt0_dn6_slot = var_idspt0_dn6;
        *var_idspt0_dn7_slot = var_idspt0_dn7;
        *var_idspt0_rv_slot = var_idspt0_rv;
        *var_lp_s0_max_slot = var_lp_s0_max;
        *var_lp_s0_max_rv_slot = var_lp_s0_max_rv;
        *var_lp_sl_max_slot = var_lp_sl_max;
        *var_lp_sl_max_rv_slot = var_lp_sl_max_rv;
        *var_m0_slot = var_m0;
        *var_m0_rv_slot = var_m0_rv;
        *var_mks_cth0_slot = var_mks_cth0;
        *var_mks_cth0_rv_slot = var_mks_cth0_rv;
        *var_mks_njunc_slot = var_mks_njunc;
        *var_mks_njunc_rv_slot = var_mks_njunc_rv;
        *var_mks_nover_slot = var_mks_nover;
        *var_mks_nover_rv_slot = var_mks_nover_rv;
        *var_mks_nsti_slot = var_mks_nsti;
        *var_mks_nsti_rv_slot = var_mks_nsti_rv;
        *var_mks_nsubb_slot = var_mks_nsubb;
        *var_mks_nsubb_rv_slot = var_mks_nsubb_rv;
        *var_mks_nsubcmax_slot = var_mks_nsubcmax;
        *var_mks_nsubcmax_rv_slot = var_mks_nsubcmax_rv;
        *var_mks_nsubp_slot = var_mks_nsubp;
        *var_mks_nsubp_rv_slot = var_mks_nsubp_rv;
        *var_mks_nsubs_slot = var_mks_nsubs;
        *var_mks_nsubs_rv_slot = var_mks_nsubs_rv;
        *var_mks_parl1_slot = var_mks_parl1;
        *var_mks_parl1_rv_slot = var_mks_parl1_rv;
        *var_mks_rth0_slot = var_mks_rth0;
        *var_mks_rth0_rv_slot = var_mks_rth0_rv;
        *var_mks_vmax_slot = var_mks_vmax;
        *var_mks_vmax_rv_slot = var_mks_vmax_rv;
        *var_mks_vtmp_slot = var_mks_vtmp;
        *var_mks_vtmp_rv_slot = var_mks_vtmp_rv;
        *var_mks_wfc_slot = var_mks_wfc;
        *var_mks_wfc_rv_slot = var_mks_wfc_rv;
        *var_mm_slot = var_mm;
        *var_mm_rv_slot = var_mm_rv;
        *var_ps0_slot = var_ps0;
        *var_ps0_dn0_slot = var_ps0_dn0;
        *var_ps0_dn10_slot = var_ps0_dn10;
        *var_ps0_dn11_slot = var_ps0_dn11;
        *var_ps0_dn12_slot = var_ps0_dn12;
        *var_ps0_dn17_slot = var_ps0_dn17;
        *var_ps0_dn2_slot = var_ps0_dn2;
        *var_ps0_dn6_slot = var_ps0_dn6;
        *var_ps0_dn7_slot = var_ps0_dn7;
        *var_ps0_ini_slot = var_ps0_ini;
        *var_ps0_ini_dn0_slot = var_ps0_ini_dn0;
        *var_ps0_ini_dn10_slot = var_ps0_ini_dn10;
        *var_ps0_ini_dn11_slot = var_ps0_ini_dn11;
        *var_ps0_ini_dn12_slot = var_ps0_ini_dn12;
        *var_ps0_ini_dn17_slot = var_ps0_ini_dn17;
        *var_ps0_ini_dn2_slot = var_ps0_ini_dn2;
        *var_ps0_ini_dn6_slot = var_ps0_ini_dn6;
        *var_ps0_ini_dn7_slot = var_ps0_ini_dn7;
        *var_ps0_ini_rv_slot = var_ps0_ini_rv;
        *var_ps0_rv_slot = var_ps0_rv;
        *var_q_s0_dep_ini_slot = var_q_s0_dep_ini;
        *var_q_s0_dep_ini_dn0_slot = var_q_s0_dep_ini_dn0;
        *var_q_s0_dep_ini_dn10_slot = var_q_s0_dep_ini_dn10;
        *var_q_s0_dep_ini_dn11_slot = var_q_s0_dep_ini_dn11;
        *var_q_s0_dep_ini_dn12_slot = var_q_s0_dep_ini_dn12;
        *var_q_s0_dep_ini_dn17_slot = var_q_s0_dep_ini_dn17;
        *var_q_s0_dep_ini_dn2_slot = var_q_s0_dep_ini_dn2;
        *var_q_s0_dep_ini_dn6_slot = var_q_s0_dep_ini_dn6;
        *var_q_s0_dep_ini_dn7_slot = var_q_s0_dep_ini_dn7;
        *var_q_s0_dep_ini_rv_slot = var_q_s0_dep_ini_rv;
        *var_subversion_slot = var_subversion;
        *var_subversion_rv_slot = var_subversion_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_uc_clm2_slot = var_uc_clm2;
        *var_uc_clm2_dn0_slot = var_uc_clm2_dn0;
        *var_uc_clm2_dn10_slot = var_uc_clm2_dn10;
        *var_uc_clm2_dn11_slot = var_uc_clm2_dn11;
        *var_uc_clm2_dn12_slot = var_uc_clm2_dn12;
        *var_uc_clm2_dn17_slot = var_uc_clm2_dn17;
        *var_uc_clm2_dn2_slot = var_uc_clm2_dn2;
        *var_uc_clm2_dn6_slot = var_uc_clm2_dn6;
        *var_uc_clm2_dn7_slot = var_uc_clm2_dn7;
        *var_uc_clm2_rv_slot = var_uc_clm2_rv;
        *var_uc_gdld_slot = var_uc_gdld;
        *var_uc_gdld_rv_slot = var_uc_gdld_rv;
        *var_uc_sc2_slot = var_uc_sc2;
        *var_uc_sc2_rv_slot = var_uc_sc2_rv;
        *var_uc_sc3_slot = var_uc_sc3;
        *var_uc_sc3_rv_slot = var_uc_sc3_rv;
        *var_uc_scp2_slot = var_uc_scp2;
        *var_uc_scp2_rv_slot = var_uc_scp2_rv;
        *var_uc_scp3_slot = var_uc_scp3;
        *var_uc_scp3_rv_slot = var_uc_scp3_rv;
        *var_uc_tnom_slot = var_uc_tnom;
        *var_uc_tnom_rv_slot = var_uc_tnom_rv;
        *var_uc_vfbover_slot = var_uc_vfbover;
        *var_uc_vfbover_rv_slot = var_uc_vfbover_rv;
        *var_vbcs_cl_slot = var_vbcs_cl;
        *var_vbcs_cl_dn0_slot = var_vbcs_cl_dn0;
        *var_vbcs_cl_dn10_slot = var_vbcs_cl_dn10;
        *var_vbcs_cl_dn11_slot = var_vbcs_cl_dn11;
        *var_vbcs_cl_dn12_slot = var_vbcs_cl_dn12;
        *var_vbcs_cl_dn17_slot = var_vbcs_cl_dn17;
        *var_vbcs_cl_dn2_slot = var_vbcs_cl_dn2;
        *var_vbcs_cl_dn6_slot = var_vbcs_cl_dn6;
        *var_vbcs_cl_dn7_slot = var_vbcs_cl_dn7;
        *var_vbcs_cl_rv_slot = var_vbcs_cl_rv;
        *var_vbsbiz_slot = var_vbsbiz;
        *var_vbsbiz_dn0_slot = var_vbsbiz_dn0;
        *var_vbsbiz_dn10_slot = var_vbsbiz_dn10;
        *var_vbsbiz_dn11_slot = var_vbsbiz_dn11;
        *var_vbsbiz_dn12_slot = var_vbsbiz_dn12;
        *var_vbsbiz_dn17_slot = var_vbsbiz_dn17;
        *var_vbsbiz_dn2_slot = var_vbsbiz_dn2;
        *var_vbsbiz_dn6_slot = var_vbsbiz_dn6;
        *var_vbsbiz_dn7_slot = var_vbsbiz_dn7;
        *var_vbsbiz_rv_slot = var_vbsbiz_rv;
        *var_x2_slot = var_x2;
        *var_x2_dn0_slot = var_x2_dn0;
        *var_x2_dn10_slot = var_x2_dn10;
        *var_x2_dn11_slot = var_x2_dn11;
        *var_x2_dn12_slot = var_x2_dn12;
        *var_x2_dn17_slot = var_x2_dn17;
        *var_x2_dn2_slot = var_x2_dn2;
        *var_x2_dn6_slot = var_x2_dn6;
        *var_x2_dn7_slot = var_x2_dn7;
        *var_x2_rv_slot = var_x2_rv;
        *var_xmax2_slot = var_xmax2;
        *var_xmax2_dn0_slot = var_xmax2_dn0;
        *var_xmax2_dn10_slot = var_xmax2_dn10;
        *var_xmax2_dn11_slot = var_xmax2_dn11;
        *var_xmax2_dn12_slot = var_xmax2_dn12;
        *var_xmax2_dn17_slot = var_xmax2_dn17;
        *var_xmax2_dn2_slot = var_xmax2_dn2;
        *var_xmax2_dn6_slot = var_xmax2_dn6;
        *var_xmax2_dn7_slot = var_xmax2_dn7;
        *var_xmax2_rv_slot = var_xmax2_rv;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn11_slot = var_xmp_dn11;
        *var_xmp_dn12_slot = var_xmp_dn12;
        *var_xmp_dn17_slot = var_xmp_dn17;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn7_slot = var_xmp_dn7;
        *var_xmp_rv_slot = var_xmp_rv;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn11_slot = var_xp_dn11;
        *var_xp_dn12_slot = var_xp_dn12;
        *var_xp_dn17_slot = var_xp_dn17;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_rv_slot = var_xp_rv;
    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        var_arg: f64,
        var_arg_dn0: f64,
        var_arg_dn10: f64,
        var_arg_dn11: f64,
        var_arg_dn12: f64,
        var_arg_dn17: f64,
        var_arg_dn2: f64,
        var_arg_dn6: f64,
        var_arg_dn7: f64,
        var_guard6: f64,
        var_mks_nsti: f64,
        var_mks_nsubb: f64,
        var_mks_nsubs: f64,
        var_subversion: f64,
        var_tmf1: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn10: f64,
        var_tmf1_dn11: f64,
        var_tmf1_dn12: f64,
        var_tmf1_dn17: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn6: f64,
        var_tmf1_dn7: f64,
        var_uc_gdld: f64,
        var_uc_tnom: f64,
        var_betatnom_slot: &mut f64,
        var_betatnom_rv_slot: &mut f64,
        var_c0bulk_slot: &mut f64,
        var_c0bulk_rv_slot: &mut f64,
        var_c_box_slot: &mut f64,
        var_c_box_fd_inv_slot: &mut f64,
        var_c_box_fd_inv_rv_slot: &mut f64,
        var_c_box_inv_slot: &mut f64,
        var_c_box_inv_rv_slot: &mut f64,
        var_c_box_rv_slot: &mut f64,
        var_c_fox0_slot: &mut f64,
        var_c_fox0_inv_slot: &mut f64,
        var_c_fox0_inv_rv_slot: &mut f64,
        var_c_fox0_rv_slot: &mut f64,
        var_c_soi_slot: &mut f64,
        var_c_soi_inv_slot: &mut f64,
        var_c_soi_inv_rv_slot: &mut f64,
        var_c_soi_rv_slot: &mut f64,
        var_cnstpgd_slot: &mut f64,
        var_cnstpgd_rv_slot: &mut f64,
        var_costi00_slot: &mut f64,
        var_costi00_rv_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn11_slot: &mut f64,
        var_dnm_dn12_slot: &mut f64,
        var_dnm_dn17_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_rv_slot: &mut f64,
        var_dvthsm_slot: &mut f64,
        var_dvthsm_rv_slot: &mut f64,
        var_dw_slot: &mut f64,
        var_dw_rv_slot: &mut f64,
        var_dwbt_slot: &mut f64,
        var_dwbt_rv_slot: &mut f64,
        var_dwcv_slot: &mut f64,
        var_dwcv_rv_slot: &mut f64,
        var_egtnom_slot: &mut f64,
        var_egtnom_rv_slot: &mut f64,
        var_gdl0_slot: &mut f64,
        var_gdl0_rv_slot: &mut f64,
        var_guard10_slot: &mut f64,
        var_guard10_rv_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard11_rv_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_guard12_rv_slot: &mut f64,
        var_guard13_slot: &mut f64,
        var_guard13_rv_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard7_rv_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard8_rv_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_guard9_rv_slot: &mut f64,
        var_leff_slot: &mut f64,
        var_leff_cv_slot: &mut f64,
        var_leff_cv_rv_slot: &mut f64,
        var_leff_rv_slot: &mut f64,
        var_lgate_slot: &mut f64,
        var_lgate_rv_slot: &mut f64,
        var_lgatesm_slot: &mut f64,
        var_lgatesm_rv_slot: &mut f64,
        var_lgle_slot: &mut f64,
        var_lgle_rv_slot: &mut f64,
        var_lgleff_slot: &mut f64,
        var_lgleff_rv_slot: &mut f64,
        var_lod_half_ref_slot: &mut f64,
        var_lod_half_ref_rv_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_m0_rv_slot: &mut f64,
        var_mks_nsubp_slot: &mut f64,
        var_mks_nsubp_rv_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_mm_rv_slot: &mut f64,
        var_muesr_slot: &mut f64,
        var_muesr_rv_slot: &mut f64,
        var_nsti_p2_slot: &mut f64,
        var_nsti_p2_rv_slot: &mut f64,
        var_nsubpp_slot: &mut f64,
        var_nsubpp_rv_slot: &mut f64,
        var_pt40_slot: &mut f64,
        var_pt40_rv_slot: &mut f64,
        var_ptl0_slot: &mut f64,
        var_ptl0_rv_slot: &mut f64,
        var_qnbulk_esi_slot: &mut f64,
        var_qnbulk_esi_rv_slot: &mut f64,
        var_tfox0_slot: &mut f64,
        var_tfox0_rv_slot: &mut f64,
        var_tmf0_slot: &mut f64,
        var_tmf0_dn0_slot: &mut f64,
        var_tmf0_dn10_slot: &mut f64,
        var_tmf0_dn11_slot: &mut f64,
        var_tmf0_dn12_slot: &mut f64,
        var_tmf0_dn17_slot: &mut f64,
        var_tmf0_dn2_slot: &mut f64,
        var_tmf0_dn6_slot: &mut f64,
        var_tmf0_dn7_slot: &mut f64,
        var_tmf0_rv_slot: &mut f64,
        var_uc_clm2_slot: &mut f64,
        var_uc_clm2_dn0_slot: &mut f64,
        var_uc_clm2_dn10_slot: &mut f64,
        var_uc_clm2_dn11_slot: &mut f64,
        var_uc_clm2_dn12_slot: &mut f64,
        var_uc_clm2_dn17_slot: &mut f64,
        var_uc_clm2_dn2_slot: &mut f64,
        var_uc_clm2_dn6_slot: &mut f64,
        var_uc_clm2_dn7_slot: &mut f64,
        var_uc_clm2_rv_slot: &mut f64,
        var_uc_wsti_slot: &mut f64,
        var_uc_wsti_rv_slot: &mut f64,
        var_vfb_slot: &mut f64,
        var_vfb_rv_slot: &mut f64,
        var_weff_slot: &mut f64,
        var_weff_cv_slot: &mut f64,
        var_weff_cv_rv_slot: &mut f64,
        var_weff_nf_slot: &mut f64,
        var_weff_nf_rv_slot: &mut f64,
        var_weff_rv_slot: &mut f64,
        var_weffcv_nf_slot: &mut f64,
        var_weffcv_nf_rv_slot: &mut f64,
        var_wg_slot: &mut f64,
        var_wg_rv_slot: &mut f64,
        var_wgate_slot: &mut f64,
        var_wgate_rv_slot: &mut f64,
        var_wl_slot: &mut f64,
        var_wl_rv_slot: &mut f64,
    ) {
        let mut var_betatnom: f64 = *var_betatnom_slot;
        let mut var_betatnom_rv: f64 = *var_betatnom_rv_slot;
        let mut var_c0bulk: f64 = *var_c0bulk_slot;
        let mut var_c0bulk_rv: f64 = *var_c0bulk_rv_slot;
        let mut var_c_box: f64 = *var_c_box_slot;
        let mut var_c_box_fd_inv: f64 = *var_c_box_fd_inv_slot;
        let mut var_c_box_fd_inv_rv: f64 = *var_c_box_fd_inv_rv_slot;
        let mut var_c_box_inv: f64 = *var_c_box_inv_slot;
        let mut var_c_box_inv_rv: f64 = *var_c_box_inv_rv_slot;
        let mut var_c_box_rv: f64 = *var_c_box_rv_slot;
        let mut var_c_fox0: f64 = *var_c_fox0_slot;
        let mut var_c_fox0_inv: f64 = *var_c_fox0_inv_slot;
        let mut var_c_fox0_inv_rv: f64 = *var_c_fox0_inv_rv_slot;
        let mut var_c_fox0_rv: f64 = *var_c_fox0_rv_slot;
        let mut var_c_soi: f64 = *var_c_soi_slot;
        let mut var_c_soi_inv: f64 = *var_c_soi_inv_slot;
        let mut var_c_soi_inv_rv: f64 = *var_c_soi_inv_rv_slot;
        let mut var_c_soi_rv: f64 = *var_c_soi_rv_slot;
        let mut var_cnstpgd: f64 = *var_cnstpgd_slot;
        let mut var_cnstpgd_rv: f64 = *var_cnstpgd_rv_slot;
        let mut var_costi00: f64 = *var_costi00_slot;
        let mut var_costi00_rv: f64 = *var_costi00_rv_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn11: f64 = *var_dnm_dn11_slot;
        let mut var_dnm_dn12: f64 = *var_dnm_dn12_slot;
        let mut var_dnm_dn17: f64 = *var_dnm_dn17_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_rv: f64 = *var_dnm_rv_slot;
        let mut var_dvthsm: f64 = *var_dvthsm_slot;
        let mut var_dvthsm_rv: f64 = *var_dvthsm_rv_slot;
        let mut var_dw: f64 = *var_dw_slot;
        let mut var_dw_rv: f64 = *var_dw_rv_slot;
        let mut var_dwbt: f64 = *var_dwbt_slot;
        let mut var_dwbt_rv: f64 = *var_dwbt_rv_slot;
        let mut var_dwcv: f64 = *var_dwcv_slot;
        let mut var_dwcv_rv: f64 = *var_dwcv_rv_slot;
        let mut var_egtnom: f64 = *var_egtnom_slot;
        let mut var_egtnom_rv: f64 = *var_egtnom_rv_slot;
        let mut var_gdl0: f64 = *var_gdl0_slot;
        let mut var_gdl0_rv: f64 = *var_gdl0_rv_slot;
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard10_rv: f64 = *var_guard10_rv_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard11_rv: f64 = *var_guard11_rv_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_guard12_rv: f64 = *var_guard12_rv_slot;
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard13_rv: f64 = *var_guard13_rv_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard7_rv: f64 = *var_guard7_rv_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard8_rv: f64 = *var_guard8_rv_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_guard9_rv: f64 = *var_guard9_rv_slot;
        let mut var_leff: f64 = *var_leff_slot;
        let mut var_leff_cv: f64 = *var_leff_cv_slot;
        let mut var_leff_cv_rv: f64 = *var_leff_cv_rv_slot;
        let mut var_leff_rv: f64 = *var_leff_rv_slot;
        let mut var_lgate: f64 = *var_lgate_slot;
        let mut var_lgate_rv: f64 = *var_lgate_rv_slot;
        let mut var_lgatesm: f64 = *var_lgatesm_slot;
        let mut var_lgatesm_rv: f64 = *var_lgatesm_rv_slot;
        let mut var_lgle: f64 = *var_lgle_slot;
        let mut var_lgle_rv: f64 = *var_lgle_rv_slot;
        let mut var_lgleff: f64 = *var_lgleff_slot;
        let mut var_lgleff_rv: f64 = *var_lgleff_rv_slot;
        let mut var_lod_half_ref: f64 = *var_lod_half_ref_slot;
        let mut var_lod_half_ref_rv: f64 = *var_lod_half_ref_rv_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_m0_rv: f64 = *var_m0_rv_slot;
        let mut var_mks_nsubp: f64 = *var_mks_nsubp_slot;
        let mut var_mks_nsubp_rv: f64 = *var_mks_nsubp_rv_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_mm_rv: f64 = *var_mm_rv_slot;
        let mut var_muesr: f64 = *var_muesr_slot;
        let mut var_muesr_rv: f64 = *var_muesr_rv_slot;
        let mut var_nsti_p2: f64 = *var_nsti_p2_slot;
        let mut var_nsti_p2_rv: f64 = *var_nsti_p2_rv_slot;
        let mut var_nsubpp: f64 = *var_nsubpp_slot;
        let mut var_nsubpp_rv: f64 = *var_nsubpp_rv_slot;
        let mut var_pt40: f64 = *var_pt40_slot;
        let mut var_pt40_rv: f64 = *var_pt40_rv_slot;
        let mut var_ptl0: f64 = *var_ptl0_slot;
        let mut var_ptl0_rv: f64 = *var_ptl0_rv_slot;
        let mut var_qnbulk_esi: f64 = *var_qnbulk_esi_slot;
        let mut var_qnbulk_esi_rv: f64 = *var_qnbulk_esi_rv_slot;
        let mut var_tfox0: f64 = *var_tfox0_slot;
        let mut var_tfox0_rv: f64 = *var_tfox0_rv_slot;
        let mut var_tmf0: f64 = *var_tmf0_slot;
        let mut var_tmf0_dn0: f64 = *var_tmf0_dn0_slot;
        let mut var_tmf0_dn10: f64 = *var_tmf0_dn10_slot;
        let mut var_tmf0_dn11: f64 = *var_tmf0_dn11_slot;
        let mut var_tmf0_dn12: f64 = *var_tmf0_dn12_slot;
        let mut var_tmf0_dn17: f64 = *var_tmf0_dn17_slot;
        let mut var_tmf0_dn2: f64 = *var_tmf0_dn2_slot;
        let mut var_tmf0_dn6: f64 = *var_tmf0_dn6_slot;
        let mut var_tmf0_dn7: f64 = *var_tmf0_dn7_slot;
        let mut var_tmf0_rv: f64 = *var_tmf0_rv_slot;
        let mut var_uc_clm2: f64 = *var_uc_clm2_slot;
        let mut var_uc_clm2_dn0: f64 = *var_uc_clm2_dn0_slot;
        let mut var_uc_clm2_dn10: f64 = *var_uc_clm2_dn10_slot;
        let mut var_uc_clm2_dn11: f64 = *var_uc_clm2_dn11_slot;
        let mut var_uc_clm2_dn12: f64 = *var_uc_clm2_dn12_slot;
        let mut var_uc_clm2_dn17: f64 = *var_uc_clm2_dn17_slot;
        let mut var_uc_clm2_dn2: f64 = *var_uc_clm2_dn2_slot;
        let mut var_uc_clm2_dn6: f64 = *var_uc_clm2_dn6_slot;
        let mut var_uc_clm2_dn7: f64 = *var_uc_clm2_dn7_slot;
        let mut var_uc_clm2_rv: f64 = *var_uc_clm2_rv_slot;
        let mut var_uc_wsti: f64 = *var_uc_wsti_slot;
        let mut var_uc_wsti_rv: f64 = *var_uc_wsti_rv_slot;
        let mut var_vfb: f64 = *var_vfb_slot;
        let mut var_vfb_rv: f64 = *var_vfb_rv_slot;
        let mut var_weff: f64 = *var_weff_slot;
        let mut var_weff_cv: f64 = *var_weff_cv_slot;
        let mut var_weff_cv_rv: f64 = *var_weff_cv_rv_slot;
        let mut var_weff_nf: f64 = *var_weff_nf_slot;
        let mut var_weff_nf_rv: f64 = *var_weff_nf_rv_slot;
        let mut var_weff_rv: f64 = *var_weff_rv_slot;
        let mut var_weffcv_nf: f64 = *var_weffcv_nf_slot;
        let mut var_weffcv_nf_rv: f64 = *var_weffcv_nf_rv_slot;
        let mut var_wg: f64 = *var_wg_slot;
        let mut var_wg_rv: f64 = *var_wg_rv_slot;
        let mut var_wgate: f64 = *var_wgate_slot;
        let mut var_wgate_rv: f64 = *var_wgate_rv_slot;
        let mut var_wl: f64 = *var_wl_slot;
        let mut var_wl_rv: f64 = *var_wl_rv_slot;

        let (assign1770_e1185, assign1770_e1185_d_n0, assign1770_e1185_d_n2, assign1770_e1185_d_n6, assign1770_e1185_d_n7, assign1770_e1185_d_n10, assign1770_e1185_d_n11, assign1770_e1185_d_n12, assign1770_e1185_d_n17,) = {
    if (var_guard6 != 0.0) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign1770_e1185;
        var_dnm_dn0 = assign1770_e1185_d_n0;
        var_dnm_dn2 = assign1770_e1185_d_n2;
        var_dnm_dn6 = assign1770_e1185_d_n6;
        var_dnm_dn7 = assign1770_e1185_d_n7;
        var_dnm_dn10 = assign1770_e1185_d_n10;
        var_dnm_dn11 = assign1770_e1185_d_n11;
        var_dnm_dn12 = assign1770_e1185_d_n12;
        var_dnm_dn17 = assign1770_e1185_d_n17;
        var_dnm_rv = 0.0;

        let assign1780_e1200: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard7 = assign1780_e1200;
        var_guard7_rv = 0.0;

        let assign1790_e1203: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard8 = assign1790_e1203;
        var_guard8_rv = 0.0;

        let (assign1800_e1211,) = {
    if (((var_guard6 != 0.0) && (var_guard7 != 0.0)) && (var_guard8 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1800_e1211;
        var_mm_rv = 0.0;

        let assign1810_e1214: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard9 = assign1810_e1214;
        var_guard9_rv = 0.0;

        let (assign1820_e1225,) = {
    if ((((var_guard6 != 0.0) && (var_guard7 != 0.0)) && (var_guard8 == 0.0)) && (var_guard9 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1820_e1225;
        var_mm_rv = 0.0;

        let assign1830_e1228: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard10 = assign1830_e1228;
        var_guard10_rv = 0.0;

        let (assign1840_e1242,) = {
    if (((((var_guard6 != 0.0) && (var_guard7 != 0.0)) && (var_guard8 == 0.0)) && (var_guard9 == 0.0)) && (var_guard10 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1840_e1242;
        var_mm_rv = 0.0;

        let assign1850_e1245: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard11 = assign1850_e1245;
        var_guard11_rv = 0.0;

        let (assign1860_e1262,) = {
    if ((((((var_guard6 != 0.0) && (var_guard7 != 0.0)) && (var_guard8 == 0.0)) && (var_guard9 == 0.0)) && (var_guard10 == 0.0)) && (var_guard11 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1860_e1262;
        var_mm_rv = 0.0;

        let (assign1870_e1268,) = {
    if ((var_guard6 != 0.0) && (var_guard7 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign1870_e1268;
        var_m0_rv = 0.0;

        let mut assign1880_loop_guard: usize = 0;
        while {
            let assign1880_cond_e1275: f64 = if (((var_guard6 != 0.0) && (var_guard7 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign1880_cond_e1275 != 0.0
        } {
            assign1880_loop_guard += 1;
            assert!(assign1880_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign1880_body0_e1282, assign1880_body0_e1282_d_n0, assign1880_body0_e1282_d_n2, assign1880_body0_e1282_d_n6, assign1880_body0_e1282_d_n7, assign1880_body0_e1282_d_n10, assign1880_body0_e1282_d_n11, assign1880_body0_e1282_d_n12, assign1880_body0_e1282_d_n17,) = {
    if ((var_guard6 != 0.0) && (var_guard7 != 0.0)) {
        let assign1880_body0_e1280: f64 = (var_dnm).sqrt();
        (assign1880_body0_e1280, (var_dnm_dn0 / (2.0 * assign1880_body0_e1280)), (var_dnm_dn2 / (2.0 * assign1880_body0_e1280)), (var_dnm_dn6 / (2.0 * assign1880_body0_e1280)), (var_dnm_dn7 / (2.0 * assign1880_body0_e1280)), (var_dnm_dn10 / (2.0 * assign1880_body0_e1280)), (var_dnm_dn11 / (2.0 * assign1880_body0_e1280)), (var_dnm_dn12 / (2.0 * assign1880_body0_e1280)), (var_dnm_dn17 / (2.0 * assign1880_body0_e1280)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
            var_dnm = assign1880_body0_e1282;
            var_dnm_dn0 = assign1880_body0_e1282_d_n0;
            var_dnm_dn2 = assign1880_body0_e1282_d_n2;
            var_dnm_dn6 = assign1880_body0_e1282_d_n6;
            var_dnm_dn7 = assign1880_body0_e1282_d_n7;
            var_dnm_dn10 = assign1880_body0_e1282_d_n10;
            var_dnm_dn11 = assign1880_body0_e1282_d_n11;
            var_dnm_dn12 = assign1880_body0_e1282_d_n12;
            var_dnm_dn17 = assign1880_body0_e1282_d_n17;
            var_dnm_rv = 0.0;
            let (assign1880_body1_e1290,) = {
    if ((var_guard6 != 0.0) && (var_guard7 != 0.0)) {
        let assign1880_body1_e1288: f64 = (var_m0 + 1.0);
        (assign1880_body1_e1288,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign1880_body1_e1290;
            var_m0_rv = 0.0;
        }

        let (assign1890_e1303, assign1890_e1303_d_n0, assign1890_e1303_d_n2, assign1890_e1303_d_n6, assign1890_e1303_d_n7, assign1890_e1303_d_n10, assign1890_e1303_d_n11, assign1890_e1303_d_n12, assign1890_e1303_d_n17,) = {
    if ((var_guard6 != 0.0) && (var_guard7 == 0.0)) {
        let assign1890_e1299: f64 = (2.0 * 2.0);
        let assign1890_e1300: f64 = (1.0 / assign1890_e1299);
        let assign1890_e1301: f64 = (var_dnm).powf(assign1890_e1300);
        (assign1890_e1301, if 0.0 == 0.0 && ((assign1890_e1300) as f64).is_finite() && ((assign1890_e1300) as f64).fract() == 0.0 { if assign1890_e1300 == 0.0 { 0.0 } else { (assign1890_e1300 * ((var_dnm).powf(assign1890_e1300 - 1.0) * var_dnm_dn0)) } } else { (assign1890_e1301 * (assign1890_e1300 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign1890_e1300) as f64).is_finite() && ((assign1890_e1300) as f64).fract() == 0.0 { if assign1890_e1300 == 0.0 { 0.0 } else { (assign1890_e1300 * ((var_dnm).powf(assign1890_e1300 - 1.0) * var_dnm_dn2)) } } else { (assign1890_e1301 * (assign1890_e1300 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign1890_e1300) as f64).is_finite() && ((assign1890_e1300) as f64).fract() == 0.0 { if assign1890_e1300 == 0.0 { 0.0 } else { (assign1890_e1300 * ((var_dnm).powf(assign1890_e1300 - 1.0) * var_dnm_dn6)) } } else { (assign1890_e1301 * (assign1890_e1300 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign1890_e1300) as f64).is_finite() && ((assign1890_e1300) as f64).fract() == 0.0 { if assign1890_e1300 == 0.0 { 0.0 } else { (assign1890_e1300 * ((var_dnm).powf(assign1890_e1300 - 1.0) * var_dnm_dn7)) } } else { (assign1890_e1301 * (assign1890_e1300 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign1890_e1300) as f64).is_finite() && ((assign1890_e1300) as f64).fract() == 0.0 { if assign1890_e1300 == 0.0 { 0.0 } else { (assign1890_e1300 * ((var_dnm).powf(assign1890_e1300 - 1.0) * var_dnm_dn10)) } } else { (assign1890_e1301 * (assign1890_e1300 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign1890_e1300) as f64).is_finite() && ((assign1890_e1300) as f64).fract() == 0.0 { if assign1890_e1300 == 0.0 { 0.0 } else { (assign1890_e1300 * ((var_dnm).powf(assign1890_e1300 - 1.0) * var_dnm_dn11)) } } else { (assign1890_e1301 * (assign1890_e1300 * (var_dnm_dn11 / var_dnm))) }, if 0.0 == 0.0 && ((assign1890_e1300) as f64).is_finite() && ((assign1890_e1300) as f64).fract() == 0.0 { if assign1890_e1300 == 0.0 { 0.0 } else { (assign1890_e1300 * ((var_dnm).powf(assign1890_e1300 - 1.0) * var_dnm_dn12)) } } else { (assign1890_e1301 * (assign1890_e1300 * (var_dnm_dn12 / var_dnm))) }, if 0.0 == 0.0 && ((assign1890_e1300) as f64).is_finite() && ((assign1890_e1300) as f64).fract() == 0.0 { if assign1890_e1300 == 0.0 { 0.0 } else { (assign1890_e1300 * ((var_dnm).powf(assign1890_e1300 - 1.0) * var_dnm_dn17)) } } else { (assign1890_e1301 * (assign1890_e1300 * (var_dnm_dn17 / var_dnm))) },)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign1890_e1303;
        var_dnm_dn0 = assign1890_e1303_d_n0;
        var_dnm_dn2 = assign1890_e1303_d_n2;
        var_dnm_dn6 = assign1890_e1303_d_n6;
        var_dnm_dn7 = assign1890_e1303_d_n7;
        var_dnm_dn10 = assign1890_e1303_d_n10;
        var_dnm_dn11 = assign1890_e1303_d_n11;
        var_dnm_dn12 = assign1890_e1303_d_n12;
        var_dnm_dn17 = assign1890_e1303_d_n17;
        var_dnm_rv = 0.0;

        let (assign1900_e1309, assign1900_e1309_d_n0, assign1900_e1309_d_n2, assign1900_e1309_d_n6, assign1900_e1309_d_n7, assign1900_e1309_d_n10, assign1900_e1309_d_n11, assign1900_e1309_d_n12, assign1900_e1309_d_n17,) = {
    if (var_guard6 != 0.0) {
        let assign1900_e1307: f64 = (1.0 / var_dnm);
        (assign1900_e1307, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn11 / (var_dnm * var_dnm))), (-(var_dnm_dn12 / (var_dnm * var_dnm))), (-(var_dnm_dn17 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign1900_e1309;
        var_dnm_dn0 = assign1900_e1309_d_n0;
        var_dnm_dn2 = assign1900_e1309_d_n2;
        var_dnm_dn6 = assign1900_e1309_d_n6;
        var_dnm_dn7 = assign1900_e1309_d_n7;
        var_dnm_dn10 = assign1900_e1309_d_n10;
        var_dnm_dn11 = assign1900_e1309_d_n11;
        var_dnm_dn12 = assign1900_e1309_d_n12;
        var_dnm_dn17 = assign1900_e1309_d_n17;
        var_dnm_rv = 0.0;

        let (assign1910_e1317, assign1910_e1317_d_n0, assign1910_e1317_d_n2, assign1910_e1317_d_n6, assign1910_e1317_d_n7, assign1910_e1317_d_n10, assign1910_e1317_d_n11, assign1910_e1317_d_n12, assign1910_e1317_d_n17,) = {
    if (var_guard6 != 0.0) {
        let assign1910_e1313: f64 = (var_tmf1 * 0.1);
        let assign1910_e1315: f64 = (assign1910_e1313 * var_dnm);
        (assign1910_e1315, (((var_tmf1_dn0 * 0.1) * var_dnm) + (assign1910_e1313 * var_dnm_dn0)), (((var_tmf1_dn2 * 0.1) * var_dnm) + (assign1910_e1313 * var_dnm_dn2)), (((var_tmf1_dn6 * 0.1) * var_dnm) + (assign1910_e1313 * var_dnm_dn6)), (((var_tmf1_dn7 * 0.1) * var_dnm) + (assign1910_e1313 * var_dnm_dn7)), (((var_tmf1_dn10 * 0.1) * var_dnm) + (assign1910_e1313 * var_dnm_dn10)), (((var_tmf1_dn11 * 0.1) * var_dnm) + (assign1910_e1313 * var_dnm_dn11)), (((var_tmf1_dn12 * 0.1) * var_dnm) + (assign1910_e1313 * var_dnm_dn12)), (((var_tmf1_dn17 * 0.1) * var_dnm) + (assign1910_e1313 * var_dnm_dn17)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn10, var_tmf0_dn11, var_tmf0_dn12, var_tmf0_dn17,)
    }
};
        var_tmf0 = assign1910_e1317;
        var_tmf0_dn0 = assign1910_e1317_d_n0;
        var_tmf0_dn2 = assign1910_e1317_d_n2;
        var_tmf0_dn6 = assign1910_e1317_d_n6;
        var_tmf0_dn7 = assign1910_e1317_d_n7;
        var_tmf0_dn10 = assign1910_e1317_d_n10;
        var_tmf0_dn11 = assign1910_e1317_d_n11;
        var_tmf0_dn12 = assign1910_e1317_d_n12;
        var_tmf0_dn17 = assign1910_e1317_d_n17;
        var_tmf0_rv = 0.0;

        let (assign1920_e1325, assign1920_e1325_d_n0, assign1920_e1325_d_n2, assign1920_e1325_d_n6, assign1920_e1325_d_n7, assign1920_e1325_d_n10, assign1920_e1325_d_n11, assign1920_e1325_d_n12, assign1920_e1325_d_n17,) = {
    if (var_guard6 != 0.0) {
        let assign1920_e1321: f64 = (2.0 + 0.1);
        let assign1920_e1323: f64 = (assign1920_e1321 - var_tmf0);
        (assign1920_e1323, (-var_tmf0_dn0), (-var_tmf0_dn2), (-var_tmf0_dn6), (-var_tmf0_dn7), (-var_tmf0_dn10), (-var_tmf0_dn11), (-var_tmf0_dn12), (-var_tmf0_dn17),)
    } else {
        (var_uc_clm2, var_uc_clm2_dn0, var_uc_clm2_dn2, var_uc_clm2_dn6, var_uc_clm2_dn7, var_uc_clm2_dn10, var_uc_clm2_dn11, var_uc_clm2_dn12, var_uc_clm2_dn17,)
    }
};
        var_uc_clm2 = assign1920_e1325;
        var_uc_clm2_dn0 = assign1920_e1325_d_n0;
        var_uc_clm2_dn2 = assign1920_e1325_d_n2;
        var_uc_clm2_dn6 = assign1920_e1325_d_n6;
        var_uc_clm2_dn7 = assign1920_e1325_d_n7;
        var_uc_clm2_dn10 = assign1920_e1325_d_n10;
        var_uc_clm2_dn11 = assign1920_e1325_d_n11;
        var_uc_clm2_dn12 = assign1920_e1325_d_n12;
        var_uc_clm2_dn17 = assign1920_e1325_d_n17;
        var_uc_clm2_rv = 0.0;

        let (assign1930_e1330, assign1930_e1330_d_n0, assign1930_e1330_d_n2, assign1930_e1330_d_n6, assign1930_e1330_d_n7, assign1930_e1330_d_n10, assign1930_e1330_d_n11, assign1930_e1330_d_n12, assign1930_e1330_d_n17,) = {
    if (var_guard6 == 0.0) {
        (var_uc_clm2, var_uc_clm2_dn0, var_uc_clm2_dn2, var_uc_clm2_dn6, var_uc_clm2_dn7, var_uc_clm2_dn10, var_uc_clm2_dn11, var_uc_clm2_dn12, var_uc_clm2_dn17,)
    } else {
        (var_uc_clm2, var_uc_clm2_dn0, var_uc_clm2_dn2, var_uc_clm2_dn6, var_uc_clm2_dn7, var_uc_clm2_dn10, var_uc_clm2_dn11, var_uc_clm2_dn12, var_uc_clm2_dn17,)
    }
};
        var_uc_clm2 = assign1930_e1330;
        var_uc_clm2_dn0 = assign1930_e1330_d_n0;
        var_uc_clm2_dn2 = assign1930_e1330_d_n2;
        var_uc_clm2_dn6 = assign1930_e1330_d_n6;
        var_uc_clm2_dn7 = assign1930_e1330_d_n7;
        var_uc_clm2_dn10 = assign1930_e1330_d_n10;
        var_uc_clm2_dn11 = assign1930_e1330_d_n11;
        var_uc_clm2_dn12 = assign1930_e1330_d_n12;
        var_uc_clm2_dn17 = assign1930_e1330_d_n17;
        var_uc_clm2_rv = 0.0;

        let assign1940_e1336: f64 = (var_uc_tnom * 1e-7);
        let assign1940_e1337: f64 = (9.025e-5 + assign1940_e1336);
        let assign1940_e1338: f64 = (var_uc_tnom * assign1940_e1337);
        let assign1940_e1339: f64 = (p.p55 - assign1940_e1338);
        var_egtnom = assign1940_e1339;
        var_egtnom_rv = 0.0;

        var_tfox0 = p.p236;
        var_tfox0_rv = 0.0;

        let assign1960_e1343: f64 = (1.034943e-10 / p.p237);
        var_c_soi = assign1960_e1343;
        var_c_soi_rv = 0.0;

        let assign1970_e1346: f64 = (1.0 / var_c_soi);
        var_c_soi_inv = assign1970_e1346;
        var_c_soi_inv_rv = 0.0;

        let assign1980_e1349: f64 = (3.453133e-11 / var_tfox0);
        var_c_fox0 = assign1980_e1349;
        var_c_fox0_rv = 0.0;

        let assign1990_e1352: f64 = (var_tfox0 / 3.453133e-11);
        var_c_fox0_inv = assign1990_e1352;
        var_c_fox0_inv_rv = 0.0;

        let assign2000_e1355: f64 = (3.453133e-11 / p.p239);
        var_c_box = assign2000_e1355;
        var_c_box_rv = 0.0;

        let assign2010_e1358: f64 = (p.p239 / 3.453133e-11);
        var_c_box_inv = assign2010_e1358;
        var_c_box_inv_rv = 0.0;

        let assign2020_e1361: f64 = (var_c_box_inv + var_c_soi_inv);
        var_c_box_fd_inv = assign2020_e1361;
        var_c_box_fd_inv_rv = 0.0;

        var_lgate = p.p0;
        var_lgate_rv = 0.0;

        let assign2040_e1366: f64 = (2.0 * p.p56);
        let assign2040_e1367: f64 = (var_lgate - assign2040_e1366);
        var_leff = assign2040_e1367;
        var_leff_rv = 0.0;

        let assign2050_e1371: f64 = (2.0 * p.p57);
        let assign2050_e1372: f64 = (var_lgate - assign2050_e1371);
        var_leff_cv = assign2050_e1372;
        var_leff_cv_rv = 0.0;

        let (assign2060_e1378,) = {
    if (p.p40 == 0.0) {
        (var_lgate,)
    } else {
        (var_leff,)
    }
};
        var_lgleff = assign2060_e1378;
        var_lgleff_rv = 0.0;

        let assign2070_e1381: f64 = (var_lgleff * 1000000.0);
        var_lgle = assign2070_e1381;
        var_lgle_rv = 0.0;

        let assign2080_e1384: f64 = (p.p1 / p.p9);
        var_wgate = assign2080_e1384;
        var_wgate_rv = 0.0;

        var_dw = p.p60;
        var_dw_rv = 0.0;

        let (assign2100_e1391,) = {
    if (var_subversion < 1.0) {
        (0.0,)
    } else {
        (p.p295,)
    }
};
        var_dwbt = assign2100_e1391;
        var_dwbt_rv = 0.0;

        let (assign2110_e1397,) = {
    if (var_subversion < 1.0) {
        (p.p60,)
    } else {
        (p.p61,)
    }
};
        var_dwcv = assign2110_e1397;
        var_dwcv_rv = 0.0;

        let assign2120_e1400: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        var_guard12 = assign2120_e1400;
        var_guard12_rv = 0.0;

        let (assign2130_e1408,) = {
    if (var_guard12 != 0.0) {
        let assign2130_e1405: f64 = (2.0 * var_dw);
        let assign2130_e1406: f64 = (var_wgate - assign2130_e1405);
        (assign2130_e1406,)
    } else {
        (var_weff,)
    }
};
        var_weff = assign2130_e1408;
        var_weff_rv = 0.0;

        let (assign2140_e1416,) = {
    if (var_guard12 != 0.0) {
        let assign2140_e1413: f64 = (2.0 * var_dwcv);
        let assign2140_e1414: f64 = (var_wgate - assign2140_e1413);
        (assign2140_e1414,)
    } else {
        (var_weff_cv,)
    }
};
        var_weff_cv = assign2140_e1416;
        var_weff_cv_rv = 0.0;

        let (assign2150_e1431,) = {
    if (var_guard12 == 0.0) {
        let assign2150_e1422: f64 = (p.p18 * var_dwbt);
        let assign2150_e1423: f64 = (var_wgate - assign2150_e1422);
        let assign2150_e1426: f64 = (2.0 - p.p18);
        let assign2150_e1428: f64 = (assign2150_e1426 * var_dw);
        let assign2150_e1429: f64 = (assign2150_e1423 - assign2150_e1428);
        (assign2150_e1429,)
    } else {
        (var_weff,)
    }
};
        var_weff = assign2150_e1431;
        var_weff_rv = 0.0;

        let (assign2160_e1446,) = {
    if (var_guard12 == 0.0) {
        let assign2160_e1437: f64 = (p.p18 * var_dwbt);
        let assign2160_e1438: f64 = (var_wgate - assign2160_e1437);
        let assign2160_e1441: f64 = (2.0 - p.p18);
        let assign2160_e1443: f64 = (assign2160_e1441 * var_dwcv);
        let assign2160_e1444: f64 = (assign2160_e1438 - assign2160_e1443);
        (assign2160_e1444,)
    } else {
        (var_weff_cv,)
    }
};
        var_weff_cv = assign2160_e1446;
        var_weff_cv_rv = 0.0;

        let assign2170_e1449: f64 = (var_weff * p.p9);
        var_weff_nf = assign2170_e1449;
        var_weff_nf_rv = 0.0;

        let assign2180_e1452: f64 = (var_weff_cv * p.p9);
        var_weffcv_nf = assign2180_e1452;
        var_weffcv_nf_rv = 0.0;

        let assign2190_e1455: f64 = (var_wgate * 1000000.0);
        var_wg = assign2190_e1455;
        var_wg_rv = 0.0;

        let assign2200_e1458: f64 = (var_wg * var_lgle);
        var_wl = assign2200_e1458;
        var_wl_rv = 0.0;

        let assign2210_e1464: f64 = (var_lgle).powf(p.p111);
        let assign2210_e1465: f64 = (p.p108 / assign2210_e1464);
        let assign2210_e1466: f64 = (1.0 + assign2210_e1465);
        let assign2210_e1467: f64 = (p.p107 * assign2210_e1466);
        let assign2210_e1472: f64 = (var_wg).powf(p.p110);
        let assign2210_e1473: f64 = (p.p109 / assign2210_e1472);
        let assign2210_e1474: f64 = (1.0 + assign2210_e1473);
        let assign2210_e1475: f64 = (assign2210_e1467 * assign2210_e1474);
        var_muesr = assign2210_e1475;
        var_muesr_rv = 0.0;

        let assign2220_e1486: f64 = if (((var_subversion > 3.0) && (var_mks_nsubp < var_mks_nsubs)) && (p.p72 > 0.0)) { 1.0 } else { 0.0 };
        var_guard13 = assign2220_e1486;
        var_guard13_rv = 0.0;

        let (assign2230_e1490,) = {
    if (var_guard13 != 0.0) {
        (var_mks_nsubs,)
    } else {
        (var_mks_nsubp,)
    }
};
        var_mks_nsubp = assign2230_e1490;
        var_mks_nsubp_rv = 0.0;

        let assign2240_e1496: f64 = (var_wg).powf(p.p75);
        let assign2240_e1497: f64 = (p.p74 / assign2240_e1496);
        let assign2240_e1498: f64 = (1.0 + assign2240_e1497);
        let assign2240_e1499: f64 = (var_mks_nsubp * assign2240_e1498);
        var_nsubpp = assign2240_e1499;
        var_nsubpp_rv = 0.0;

        let assign2250_e1505: f64 = (0.5 * var_lgate);
        let assign2250_e1506: f64 = (p.p62 + assign2250_e1505);
        let assign2250_e1507: f64 = (1.0 / assign2250_e1506);
        let assign2250_e1512: f64 = (0.5 * var_lgate);
        let assign2250_e1513: f64 = (p.p63 + assign2250_e1512);
        let assign2250_e1514: f64 = (1.0 / assign2250_e1513);
        let assign2250_e1515: f64 = (assign2250_e1507 + assign2250_e1514);
        let assign2250_e1516: f64 = (2.0 / assign2250_e1515);
        var_lod_half_ref = assign2250_e1516;
        var_lod_half_ref_rv = 0.0;

        let assign2260_e1520: f64 = (1.3806226e-23 * var_uc_tnom);
        let assign2260_e1521: f64 = (1.6021918e-19 / assign2260_e1520);
        var_betatnom = assign2260_e1521;
        var_betatnom_rv = 0.0;

        let assign2270_e1524: f64 = (1.6021918e-19 * var_mks_nsubb);
        let assign2270_e1526: f64 = (assign2270_e1524 * 1.034943e-10);
        var_qnbulk_esi = assign2270_e1526;
        var_qnbulk_esi_rv = 0.0;

        let assign2280_e1530: f64 = (-p.p247);
        let assign2280_e1531: f64 = (var_lgle).powf(assign2280_e1530);
        let assign2280_e1532: f64 = (p.p244 * assign2280_e1531);
        var_ptl0 = assign2280_e1532;
        var_ptl0_rv = 0.0;

        let assign2290_e1536: f64 = (-p.p252);
        let assign2290_e1537: f64 = (var_lgle).powf(assign2290_e1536);
        let assign2290_e1538: f64 = (p.p251 * assign2290_e1537);
        var_pt40 = assign2290_e1538;
        var_pt40_rv = 0.0;

        let assign2300_e1542: f64 = (var_lgle + var_uc_gdld);
        let assign2300_e1544: f64 = (-p.p249);
        let assign2300_e1545: f64 = (assign2300_e1542).powf(assign2300_e1544);
        let assign2300_e1546: f64 = (p.p248 * assign2300_e1545);
        var_gdl0 = assign2300_e1546;
        var_gdl0_rv = 0.0;

        let assign2310_e1549: f64 = (2.0 * 1.6021918e-19);
        let assign2310_e1551: f64 = (assign2310_e1549 * var_mks_nsti);
        let assign2310_e1553: f64 = (assign2310_e1551 * 1.034943e-10);
        let assign2310_e1554: f64 = (assign2310_e1553).sqrt();
        var_costi00 = assign2310_e1554;
        var_costi00_rv = 0.0;

        let assign2320_e1558: f64 = (var_mks_nsti * var_mks_nsti);
        let assign2320_e1559: f64 = (1.0 / assign2320_e1558);
        var_nsti_p2 = assign2320_e1559;
        var_nsti_p2_rv = 0.0;

        let assign2330_e1563: f64 = (1.0 / var_lgle);
        let assign2330_e1564: f64 = (1.0 + assign2330_e1563);
        let assign2330_e1566: f64 = (assign2330_e1564).powf(p.p91);
        let assign2330_e1568: f64 = (assign2330_e1566 * p.p89);
        var_cnstpgd = assign2330_e1568;
        var_cnstpgd_rv = 0.0;

        var_c0bulk = var_qnbulk_esi;
        var_c0bulk_rv = 0.0;

        var_vfb = p.p68;
        var_vfb_rv = 0.0;

        let assign2360_e1575: f64 = (var_wl).powf(p.p77);
        let assign2360_e1576: f64 = (p.p76 / assign2360_e1575);
        let assign2360_e1577: f64 = (var_lgleff + assign2360_e1576);
        var_lgatesm = assign2360_e1577;
        var_lgatesm_rv = 0.0;

        let assign2370_e1581: f64 = (var_wl).powf(p.p79);
        let assign2370_e1582: f64 = (p.p78 / assign2370_e1581);
        var_dvthsm = assign2370_e1582;
        var_dvthsm_rv = 0.0;

        let assign2380_e1588: f64 = (var_lgatesm * 1000000.0);
        let assign2380_e1590: f64 = (assign2380_e1588).powf(p.p151);
        let assign2380_e1591: f64 = (p.p150 / assign2380_e1590);
        let assign2380_e1592: f64 = (1.0 + assign2380_e1591);
        let assign2380_e1593: f64 = (p.p149 * assign2380_e1592);
        let assign2380_e1595: f64 = assign2380_e1593;
        let assign2380_e1599: f64 = (var_wg).powf(p.p153);
        let assign2380_e1600: f64 = (p.p152 / assign2380_e1599);
        let assign2380_e1601: f64 = (assign2380_e1595 + assign2380_e1600);
        var_uc_wsti = assign2380_e1601;
        var_uc_wsti_rv = 0.0;

        *var_betatnom_slot = var_betatnom;
        *var_betatnom_rv_slot = var_betatnom_rv;
        *var_c0bulk_slot = var_c0bulk;
        *var_c0bulk_rv_slot = var_c0bulk_rv;
        *var_c_box_slot = var_c_box;
        *var_c_box_fd_inv_slot = var_c_box_fd_inv;
        *var_c_box_fd_inv_rv_slot = var_c_box_fd_inv_rv;
        *var_c_box_inv_slot = var_c_box_inv;
        *var_c_box_inv_rv_slot = var_c_box_inv_rv;
        *var_c_box_rv_slot = var_c_box_rv;
        *var_c_fox0_slot = var_c_fox0;
        *var_c_fox0_inv_slot = var_c_fox0_inv;
        *var_c_fox0_inv_rv_slot = var_c_fox0_inv_rv;
        *var_c_fox0_rv_slot = var_c_fox0_rv;
        *var_c_soi_slot = var_c_soi;
        *var_c_soi_inv_slot = var_c_soi_inv;
        *var_c_soi_inv_rv_slot = var_c_soi_inv_rv;
        *var_c_soi_rv_slot = var_c_soi_rv;
        *var_cnstpgd_slot = var_cnstpgd;
        *var_cnstpgd_rv_slot = var_cnstpgd_rv;
        *var_costi00_slot = var_costi00;
        *var_costi00_rv_slot = var_costi00_rv;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn11_slot = var_dnm_dn11;
        *var_dnm_dn12_slot = var_dnm_dn12;
        *var_dnm_dn17_slot = var_dnm_dn17;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_rv_slot = var_dnm_rv;
        *var_dvthsm_slot = var_dvthsm;
        *var_dvthsm_rv_slot = var_dvthsm_rv;
        *var_dw_slot = var_dw;
        *var_dw_rv_slot = var_dw_rv;
        *var_dwbt_slot = var_dwbt;
        *var_dwbt_rv_slot = var_dwbt_rv;
        *var_dwcv_slot = var_dwcv;
        *var_dwcv_rv_slot = var_dwcv_rv;
        *var_egtnom_slot = var_egtnom;
        *var_egtnom_rv_slot = var_egtnom_rv;
        *var_gdl0_slot = var_gdl0;
        *var_gdl0_rv_slot = var_gdl0_rv;
        *var_guard10_slot = var_guard10;
        *var_guard10_rv_slot = var_guard10_rv;
        *var_guard11_slot = var_guard11;
        *var_guard11_rv_slot = var_guard11_rv;
        *var_guard12_slot = var_guard12;
        *var_guard12_rv_slot = var_guard12_rv;
        *var_guard13_slot = var_guard13;
        *var_guard13_rv_slot = var_guard13_rv;
        *var_guard7_slot = var_guard7;
        *var_guard7_rv_slot = var_guard7_rv;
        *var_guard8_slot = var_guard8;
        *var_guard8_rv_slot = var_guard8_rv;
        *var_guard9_slot = var_guard9;
        *var_guard9_rv_slot = var_guard9_rv;
        *var_leff_slot = var_leff;
        *var_leff_cv_slot = var_leff_cv;
        *var_leff_cv_rv_slot = var_leff_cv_rv;
        *var_leff_rv_slot = var_leff_rv;
        *var_lgate_slot = var_lgate;
        *var_lgate_rv_slot = var_lgate_rv;
        *var_lgatesm_slot = var_lgatesm;
        *var_lgatesm_rv_slot = var_lgatesm_rv;
        *var_lgle_slot = var_lgle;
        *var_lgle_rv_slot = var_lgle_rv;
        *var_lgleff_slot = var_lgleff;
        *var_lgleff_rv_slot = var_lgleff_rv;
        *var_lod_half_ref_slot = var_lod_half_ref;
        *var_lod_half_ref_rv_slot = var_lod_half_ref_rv;
        *var_m0_slot = var_m0;
        *var_m0_rv_slot = var_m0_rv;
        *var_mks_nsubp_slot = var_mks_nsubp;
        *var_mks_nsubp_rv_slot = var_mks_nsubp_rv;
        *var_mm_slot = var_mm;
        *var_mm_rv_slot = var_mm_rv;
        *var_muesr_slot = var_muesr;
        *var_muesr_rv_slot = var_muesr_rv;
        *var_nsti_p2_slot = var_nsti_p2;
        *var_nsti_p2_rv_slot = var_nsti_p2_rv;
        *var_nsubpp_slot = var_nsubpp;
        *var_nsubpp_rv_slot = var_nsubpp_rv;
        *var_pt40_slot = var_pt40;
        *var_pt40_rv_slot = var_pt40_rv;
        *var_ptl0_slot = var_ptl0;
        *var_ptl0_rv_slot = var_ptl0_rv;
        *var_qnbulk_esi_slot = var_qnbulk_esi;
        *var_qnbulk_esi_rv_slot = var_qnbulk_esi_rv;
        *var_tfox0_slot = var_tfox0;
        *var_tfox0_rv_slot = var_tfox0_rv;
        *var_tmf0_slot = var_tmf0;
        *var_tmf0_dn0_slot = var_tmf0_dn0;
        *var_tmf0_dn10_slot = var_tmf0_dn10;
        *var_tmf0_dn11_slot = var_tmf0_dn11;
        *var_tmf0_dn12_slot = var_tmf0_dn12;
        *var_tmf0_dn17_slot = var_tmf0_dn17;
        *var_tmf0_dn2_slot = var_tmf0_dn2;
        *var_tmf0_dn6_slot = var_tmf0_dn6;
        *var_tmf0_dn7_slot = var_tmf0_dn7;
        *var_tmf0_rv_slot = var_tmf0_rv;
        *var_uc_clm2_slot = var_uc_clm2;
        *var_uc_clm2_dn0_slot = var_uc_clm2_dn0;
        *var_uc_clm2_dn10_slot = var_uc_clm2_dn10;
        *var_uc_clm2_dn11_slot = var_uc_clm2_dn11;
        *var_uc_clm2_dn12_slot = var_uc_clm2_dn12;
        *var_uc_clm2_dn17_slot = var_uc_clm2_dn17;
        *var_uc_clm2_dn2_slot = var_uc_clm2_dn2;
        *var_uc_clm2_dn6_slot = var_uc_clm2_dn6;
        *var_uc_clm2_dn7_slot = var_uc_clm2_dn7;
        *var_uc_clm2_rv_slot = var_uc_clm2_rv;
        *var_uc_wsti_slot = var_uc_wsti;
        *var_uc_wsti_rv_slot = var_uc_wsti_rv;
        *var_vfb_slot = var_vfb;
        *var_vfb_rv_slot = var_vfb_rv;
        *var_weff_slot = var_weff;
        *var_weff_cv_slot = var_weff_cv;
        *var_weff_cv_rv_slot = var_weff_cv_rv;
        *var_weff_nf_slot = var_weff_nf;
        *var_weff_nf_rv_slot = var_weff_nf_rv;
        *var_weff_rv_slot = var_weff_rv;
        *var_weffcv_nf_slot = var_weffcv_nf;
        *var_weffcv_nf_rv_slot = var_weffcv_nf_rv;
        *var_wg_slot = var_wg;
        *var_wg_rv_slot = var_wg_rv;
        *var_wgate_slot = var_wgate;
        *var_wgate_rv_slot = var_wgate_rv;
        *var_wl_slot = var_wl;
        *var_wl_rv_slot = var_wl_rv;
    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_lgate: f64,
        var_lgle: f64,
        var_lod_half_ref: f64,
        var_mks_cth0: f64,
        var_weffcv_nf: f64,
        var_wg: f64,
        var_abtn_given_slot: &mut f64,
        var_abtn_given_rv_slot: &mut f64,
        var_abtp_given_slot: &mut f64,
        var_abtp_given_rv_slot: &mut f64,
        var_cbtbn_given_slot: &mut f64,
        var_cbtbn_given_rv_slot: &mut f64,
        var_cbtbp_given_slot: &mut f64,
        var_cbtbp_given_rv_slot: &mut f64,
        var_cgbo_given_slot: &mut f64,
        var_cgbo_given_rv_slot: &mut f64,
        var_cgdo_given_slot: &mut f64,
        var_cgdo_given_rv_slot: &mut f64,
        var_cgso_given_slot: &mut f64,
        var_cgso_given_rv_slot: &mut f64,
        var_clmmod_slot: &mut f64,
        var_clmmod_rv_slot: &mut f64,
        var_cqyb0_slot: &mut f64,
        var_cqyb0_rv_slot: &mut f64,
        var_cth_slot: &mut f64,
        var_cth_rv_slot: &mut f64,
        var_ddlte_slot: &mut f64,
        var_ddlte_rv_slot: &mut f64,
        var_dtemp_given_slot: &mut f64,
        var_dtemp_given_rv_slot: &mut f64,
        var_gjmin_slot: &mut f64,
        var_gjmin_rv_slot: &mut f64,
        var_guard14_slot: &mut f64,
        var_guard14_rv_slot: &mut f64,
        var_guard15_slot: &mut f64,
        var_guard15_rv_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_guard16_rv_slot: &mut f64,
        var_guard17_slot: &mut f64,
        var_guard17_rv_slot: &mut f64,
        var_guard18_slot: &mut f64,
        var_guard18_rv_slot: &mut f64,
        var_i_slot: &mut f64,
        var_i_rv_slot: &mut f64,
        var_lod_half_slot: &mut f64,
        var_lod_half_dn0_slot: &mut f64,
        var_lod_half_dn10_slot: &mut f64,
        var_lod_half_dn11_slot: &mut f64,
        var_lod_half_dn12_slot: &mut f64,
        var_lod_half_dn17_slot: &mut f64,
        var_lod_half_dn2_slot: &mut f64,
        var_lod_half_dn6_slot: &mut f64,
        var_lod_half_dn7_slot: &mut f64,
        var_lod_half_rv_slot: &mut f64,
        var_mfactor_slot: &mut f64,
        var_mfactor_rv_slot: &mut f64,
        var_pdbcp_given_slot: &mut f64,
        var_pdbcp_given_rv_slot: &mut f64,
        var_psbcp_given_slot: &mut f64,
        var_psbcp_given_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_temp_given_slot: &mut f64,
        var_temp_given_rv_slot: &mut f64,
        var_uc_pdbcp_slot: &mut f64,
        var_uc_pdbcp_rv_slot: &mut f64,
        var_uc_psbcp_slot: &mut f64,
        var_uc_psbcp_rv_slot: &mut f64,
        var_uc_svgs_slot: &mut f64,
        var_uc_svgs_rv_slot: &mut f64,
        var_uc_temp_slot: &mut f64,
        var_uc_temp_rv_slot: &mut f64,
        var_vfbsub0_slot: &mut f64,
        var_vfbsub0_rv_slot: &mut f64,
        var_vg2const_slot: &mut f64,
        var_vg2const_dn0_slot: &mut f64,
        var_vg2const_dn10_slot: &mut f64,
        var_vg2const_dn11_slot: &mut f64,
        var_vg2const_dn12_slot: &mut f64,
        var_vg2const_dn17_slot: &mut f64,
        var_vg2const_dn2_slot: &mut f64,
        var_vg2const_dn6_slot: &mut f64,
        var_vg2const_dn7_slot: &mut f64,
        var_vg2const_rv_slot: &mut f64,
        var_vgs_min_slot: &mut f64,
        var_vgs_min_rv_slot: &mut f64,
        var_xgate_slot: &mut f64,
        var_xgate_rv_slot: &mut f64,
        var_xsub1_slot: &mut f64,
        var_xsub1_rv_slot: &mut f64,
        var_xsub2_slot: &mut f64,
        var_xsub2_rv_slot: &mut f64,
        var_xvbs_slot: &mut f64,
        var_xvbs_rv_slot: &mut f64,
        var_zvgs_slot: &mut f64,
        var_zvgs_rv_slot: &mut f64,
    ) {
        let mut var_abtn_given: f64 = *var_abtn_given_slot;
        let mut var_abtn_given_rv: f64 = *var_abtn_given_rv_slot;
        let mut var_abtp_given: f64 = *var_abtp_given_slot;
        let mut var_abtp_given_rv: f64 = *var_abtp_given_rv_slot;
        let mut var_cbtbn_given: f64 = *var_cbtbn_given_slot;
        let mut var_cbtbn_given_rv: f64 = *var_cbtbn_given_rv_slot;
        let mut var_cbtbp_given: f64 = *var_cbtbp_given_slot;
        let mut var_cbtbp_given_rv: f64 = *var_cbtbp_given_rv_slot;
        let mut var_cgbo_given: f64 = *var_cgbo_given_slot;
        let mut var_cgbo_given_rv: f64 = *var_cgbo_given_rv_slot;
        let mut var_cgdo_given: f64 = *var_cgdo_given_slot;
        let mut var_cgdo_given_rv: f64 = *var_cgdo_given_rv_slot;
        let mut var_cgso_given: f64 = *var_cgso_given_slot;
        let mut var_cgso_given_rv: f64 = *var_cgso_given_rv_slot;
        let mut var_clmmod: f64 = *var_clmmod_slot;
        let mut var_clmmod_rv: f64 = *var_clmmod_rv_slot;
        let mut var_cqyb0: f64 = *var_cqyb0_slot;
        let mut var_cqyb0_rv: f64 = *var_cqyb0_rv_slot;
        let mut var_cth: f64 = *var_cth_slot;
        let mut var_cth_rv: f64 = *var_cth_rv_slot;
        let mut var_ddlte: f64 = *var_ddlte_slot;
        let mut var_ddlte_rv: f64 = *var_ddlte_rv_slot;
        let mut var_dtemp_given: f64 = *var_dtemp_given_slot;
        let mut var_dtemp_given_rv: f64 = *var_dtemp_given_rv_slot;
        let mut var_gjmin: f64 = *var_gjmin_slot;
        let mut var_gjmin_rv: f64 = *var_gjmin_rv_slot;
        let mut var_guard14: f64 = *var_guard14_slot;
        let mut var_guard14_rv: f64 = *var_guard14_rv_slot;
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_guard15_rv: f64 = *var_guard15_rv_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard16_rv: f64 = *var_guard16_rv_slot;
        let mut var_guard17: f64 = *var_guard17_slot;
        let mut var_guard17_rv: f64 = *var_guard17_rv_slot;
        let mut var_guard18: f64 = *var_guard18_slot;
        let mut var_guard18_rv: f64 = *var_guard18_rv_slot;
        let mut var_i: f64 = *var_i_slot;
        let mut var_i_rv: f64 = *var_i_rv_slot;
        let mut var_lod_half: f64 = *var_lod_half_slot;
        let mut var_lod_half_dn0: f64 = *var_lod_half_dn0_slot;
        let mut var_lod_half_dn10: f64 = *var_lod_half_dn10_slot;
        let mut var_lod_half_dn11: f64 = *var_lod_half_dn11_slot;
        let mut var_lod_half_dn12: f64 = *var_lod_half_dn12_slot;
        let mut var_lod_half_dn17: f64 = *var_lod_half_dn17_slot;
        let mut var_lod_half_dn2: f64 = *var_lod_half_dn2_slot;
        let mut var_lod_half_dn6: f64 = *var_lod_half_dn6_slot;
        let mut var_lod_half_dn7: f64 = *var_lod_half_dn7_slot;
        let mut var_lod_half_rv: f64 = *var_lod_half_rv_slot;
        let mut var_mfactor: f64 = *var_mfactor_slot;
        let mut var_mfactor_rv: f64 = *var_mfactor_rv_slot;
        let mut var_pdbcp_given: f64 = *var_pdbcp_given_slot;
        let mut var_pdbcp_given_rv: f64 = *var_pdbcp_given_rv_slot;
        let mut var_psbcp_given: f64 = *var_psbcp_given_slot;
        let mut var_psbcp_given_rv: f64 = *var_psbcp_given_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_temp_given: f64 = *var_temp_given_slot;
        let mut var_temp_given_rv: f64 = *var_temp_given_rv_slot;
        let mut var_uc_pdbcp: f64 = *var_uc_pdbcp_slot;
        let mut var_uc_pdbcp_rv: f64 = *var_uc_pdbcp_rv_slot;
        let mut var_uc_psbcp: f64 = *var_uc_psbcp_slot;
        let mut var_uc_psbcp_rv: f64 = *var_uc_psbcp_rv_slot;
        let mut var_uc_svgs: f64 = *var_uc_svgs_slot;
        let mut var_uc_svgs_rv: f64 = *var_uc_svgs_rv_slot;
        let mut var_uc_temp: f64 = *var_uc_temp_slot;
        let mut var_uc_temp_rv: f64 = *var_uc_temp_rv_slot;
        let mut var_vfbsub0: f64 = *var_vfbsub0_slot;
        let mut var_vfbsub0_rv: f64 = *var_vfbsub0_rv_slot;
        let mut var_vg2const: f64 = *var_vg2const_slot;
        let mut var_vg2const_dn0: f64 = *var_vg2const_dn0_slot;
        let mut var_vg2const_dn10: f64 = *var_vg2const_dn10_slot;
        let mut var_vg2const_dn11: f64 = *var_vg2const_dn11_slot;
        let mut var_vg2const_dn12: f64 = *var_vg2const_dn12_slot;
        let mut var_vg2const_dn17: f64 = *var_vg2const_dn17_slot;
        let mut var_vg2const_dn2: f64 = *var_vg2const_dn2_slot;
        let mut var_vg2const_dn6: f64 = *var_vg2const_dn6_slot;
        let mut var_vg2const_dn7: f64 = *var_vg2const_dn7_slot;
        let mut var_vg2const_rv: f64 = *var_vg2const_rv_slot;
        let mut var_vgs_min: f64 = *var_vgs_min_slot;
        let mut var_vgs_min_rv: f64 = *var_vgs_min_rv_slot;
        let mut var_xgate: f64 = *var_xgate_slot;
        let mut var_xgate_rv: f64 = *var_xgate_rv_slot;
        let mut var_xsub1: f64 = *var_xsub1_slot;
        let mut var_xsub1_rv: f64 = *var_xsub1_rv_slot;
        let mut var_xsub2: f64 = *var_xsub2_slot;
        let mut var_xsub2_rv: f64 = *var_xsub2_rv_slot;
        let mut var_xvbs: f64 = *var_xvbs_slot;
        let mut var_xvbs_rv: f64 = *var_xvbs_rv_slot;
        let mut var_zvgs: f64 = *var_zvgs_slot;
        let mut var_zvgs_rv: f64 = *var_zvgs_rv_slot;

        let assign2390_e1605: f64 = (var_lgle).powf(p.p192);
        let assign2390_e1607: f64 = (assign2390_e1605 * p.p193);
        let assign2390_e1608: f64 = (1.0 + assign2390_e1607);
        var_clmmod = assign2390_e1608;
        var_clmmod_rv = 0.0;

        let assign2410_e1628: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        var_guard14 = assign2410_e1628;
        var_guard14_rv = 0.0;

        let (assign2420_e1638,) = {
    if (var_guard14 != 0.0) {
        let assign2420_e1634: f64 = (var_wg).powf(p.p131);
        let assign2420_e1635: f64 = (p.p130 / assign2420_e1634);
        let assign2420_e1636: f64 = (1.0 + assign2420_e1635);
        (assign2420_e1636,)
    } else {
        (var_zvgs,)
    }
};
        var_zvgs = assign2420_e1638;
        var_zvgs_rv = 0.0;

        let (assign2430_e1650,) = {
    if (var_guard14 != 0.0) {
        let assign2430_e1645: f64 = (var_lgle).powf(p.p126);
        let assign2430_e1646: f64 = (p.p125 / assign2430_e1645);
        let assign2430_e1647: f64 = (1.0 + assign2430_e1646);
        let assign2430_e1648: f64 = (p.p124 * assign2430_e1647);
        (assign2430_e1648,)
    } else {
        (var_xvbs,)
    }
};
        var_xvbs = assign2430_e1650;
        var_xvbs_rv = 0.0;

        let (assign2440_e1658,) = {
    if (var_guard14 != 0.0) {
        let assign2440_e1655: f64 = (var_lgle + p.p123);
        let assign2440_e1656: f64 = (var_lgle / assign2440_e1655);
        (assign2440_e1656,)
    } else {
        (var_xgate,)
    }
};
        var_xgate = assign2440_e1658;
        var_xgate_rv = 0.0;

        let (assign2450_e1670,) = {
    if (var_guard14 != 0.0) {
        let assign2450_e1665: f64 = (var_lgle).powf(p.p120);
        let assign2450_e1666: f64 = (p.p119 / assign2450_e1665);
        let assign2450_e1667: f64 = (1.0 + assign2450_e1666);
        let assign2450_e1668: f64 = (p.p117 * assign2450_e1667);
        (assign2450_e1668,)
    } else {
        (var_xsub1,)
    }
};
        var_xsub1 = assign2450_e1670;
        var_xsub1_rv = 0.0;

        let (assign2460_e1680,) = {
    if (var_guard14 != 0.0) {
        let assign2460_e1676: f64 = (p.p121 / var_lgle);
        let assign2460_e1677: f64 = (1.0 + assign2460_e1676);
        let assign2460_e1678: f64 = (p.p118 * assign2460_e1677);
        (assign2460_e1678,)
    } else {
        (var_xsub2,)
    }
};
        var_xsub2 = assign2460_e1680;
        var_xsub2_rv = 0.0;

        let (assign2470_e1687, assign2470_e1687_d_n0, assign2470_e1687_d_n2, assign2470_e1687_d_n6, assign2470_e1687_d_n7, assign2470_e1687_d_n10, assign2470_e1687_d_n11, assign2470_e1687_d_n12, assign2470_e1687_d_n17,) = {
    if (var_guard14 == 0.0) {
        let assign2470_e1685: f64 = (var_wg).powf(p.p131);
        (assign2470_e1685, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign2470_e1687;
        var_t2_dn0 = assign2470_e1687_d_n0;
        var_t2_dn2 = assign2470_e1687_d_n2;
        var_t2_dn6 = assign2470_e1687_d_n6;
        var_t2_dn7 = assign2470_e1687_d_n7;
        var_t2_dn10 = assign2470_e1687_d_n10;
        var_t2_dn11 = assign2470_e1687_d_n11;
        var_t2_dn12 = assign2470_e1687_d_n12;
        var_t2_dn17 = assign2470_e1687_d_n17;
        var_t2_rv = 0.0;

        let (assign2480_e1706, assign2480_e1706_d_n0, assign2480_e1706_d_n2, assign2480_e1706_d_n6, assign2480_e1706_d_n7, assign2480_e1706_d_n10, assign2480_e1706_d_n11, assign2480_e1706_d_n12, assign2480_e1706_d_n17,) = {
    if (var_guard14 == 0.0) {
        let assign2480_e1695: f64 = (var_lgle).powf(p.p129);
        let assign2480_e1696: f64 = (p.p128 / assign2480_e1695);
        let assign2480_e1697: f64 = (1.0 + assign2480_e1696);
        let assign2480_e1698: f64 = (p.p127 * assign2480_e1697);
        let assign2480_e1702: f64 = (var_t2 + p.p130);
        let assign2480_e1703: f64 = (var_t2 / assign2480_e1702);
        let assign2480_e1704: f64 = (assign2480_e1698 * assign2480_e1703);
        (assign2480_e1704, (assign2480_e1698 * (((var_t2_dn0 * assign2480_e1702) - (var_t2 * var_t2_dn0)) / (assign2480_e1702 * assign2480_e1702))), (assign2480_e1698 * (((var_t2_dn2 * assign2480_e1702) - (var_t2 * var_t2_dn2)) / (assign2480_e1702 * assign2480_e1702))), (assign2480_e1698 * (((var_t2_dn6 * assign2480_e1702) - (var_t2 * var_t2_dn6)) / (assign2480_e1702 * assign2480_e1702))), (assign2480_e1698 * (((var_t2_dn7 * assign2480_e1702) - (var_t2 * var_t2_dn7)) / (assign2480_e1702 * assign2480_e1702))), (assign2480_e1698 * (((var_t2_dn10 * assign2480_e1702) - (var_t2 * var_t2_dn10)) / (assign2480_e1702 * assign2480_e1702))), (assign2480_e1698 * (((var_t2_dn11 * assign2480_e1702) - (var_t2 * var_t2_dn11)) / (assign2480_e1702 * assign2480_e1702))), (assign2480_e1698 * (((var_t2_dn12 * assign2480_e1702) - (var_t2 * var_t2_dn12)) / (assign2480_e1702 * assign2480_e1702))), (assign2480_e1698 * (((var_t2_dn17 * assign2480_e1702) - (var_t2 * var_t2_dn17)) / (assign2480_e1702 * assign2480_e1702))),)
    } else {
        (var_vg2const, var_vg2const_dn0, var_vg2const_dn2, var_vg2const_dn6, var_vg2const_dn7, var_vg2const_dn10, var_vg2const_dn11, var_vg2const_dn12, var_vg2const_dn17,)
    }
};
        var_vg2const = assign2480_e1706;
        var_vg2const_dn0 = assign2480_e1706_d_n0;
        var_vg2const_dn2 = assign2480_e1706_d_n2;
        var_vg2const_dn6 = assign2480_e1706_d_n6;
        var_vg2const_dn7 = assign2480_e1706_d_n7;
        var_vg2const_dn10 = assign2480_e1706_d_n10;
        var_vg2const_dn11 = assign2480_e1706_d_n11;
        var_vg2const_dn12 = assign2480_e1706_d_n12;
        var_vg2const_dn17 = assign2480_e1706_d_n17;
        var_vg2const_rv = 0.0;

        let (assign2490_e1719,) = {
    if (var_guard14 == 0.0) {
        let assign2490_e1714: f64 = (var_lgle).powf(p.p126);
        let assign2490_e1715: f64 = (p.p125 / assign2490_e1714);
        let assign2490_e1716: f64 = (1.0 + assign2490_e1715);
        let assign2490_e1717: f64 = (p.p124 * assign2490_e1716);
        (assign2490_e1717,)
    } else {
        (var_xvbs,)
    }
};
        var_xvbs = assign2490_e1719;
        var_xvbs_rv = 0.0;

        let (assign2500_e1732,) = {
    if (var_guard14 == 0.0) {
        let assign2500_e1727: f64 = (var_lgle).powf(p.p133);
        let assign2500_e1728: f64 = (p.p132 / assign2500_e1727);
        let assign2500_e1729: f64 = (1.0 + assign2500_e1728);
        let assign2500_e1730: f64 = (p.p123 * assign2500_e1729);
        (assign2500_e1730,)
    } else {
        (var_xgate,)
    }
};
        var_xgate = assign2500_e1732;
        var_xgate_rv = 0.0;

        let (assign2510_e1745,) = {
    if (var_guard14 == 0.0) {
        let assign2510_e1740: f64 = (var_lgle).powf(p.p120);
        let assign2510_e1741: f64 = (p.p119 / assign2510_e1740);
        let assign2510_e1742: f64 = (1.0 + assign2510_e1741);
        let assign2510_e1743: f64 = (p.p117 * assign2510_e1742);
        (assign2510_e1743,)
    } else {
        (var_xsub1,)
    }
};
        var_xsub1 = assign2510_e1745;
        var_xsub1_rv = 0.0;

        let (assign2520_e1756,) = {
    if (var_guard14 == 0.0) {
        let assign2520_e1752: f64 = (p.p121 / var_lgle);
        let assign2520_e1753: f64 = (1.0 + assign2520_e1752);
        let assign2520_e1754: f64 = (p.p118 * assign2520_e1753);
        (assign2520_e1754,)
    } else {
        (var_xsub2,)
    }
};
        var_xsub2 = assign2520_e1756;
        var_xsub2_rv = 0.0;

        let assign2530_e1759: f64 = (1000000.0 * var_weffcv_nf);
        let assign2530_e1761: f64 = (assign2530_e1759 * p.p65);
        let assign2530_e1764: f64 = (var_lgle).powf(p.p66);
        let assign2530_e1765: f64 = (assign2530_e1761 / assign2530_e1764);
        var_cqyb0 = assign2530_e1765;
        var_cqyb0_rv = 0.0;

        let assign2540_e1771: f64 = (var_lgle).powf(p.p136);
        let assign2540_e1772: f64 = (p.p135 / assign2540_e1771);
        let assign2540_e1773: f64 = (1.0 + assign2540_e1772);
        let assign2540_e1774: f64 = (p.p134 * assign2540_e1773);
        var_vfbsub0 = assign2540_e1774;
        var_vfbsub0_rv = 0.0;

        let assign2550_e1777: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        var_guard15 = assign2550_e1777;
        var_guard15_rv = 0.0;

        let (assign2560_e1789,) = {
    if (var_guard15 != 0.0) {
        let assign2560_e1784: f64 = (var_lgle).powf(p.p129);
        let assign2560_e1785: f64 = (p.p128 / assign2560_e1784);
        let assign2560_e1786: f64 = (1.0 + assign2560_e1785);
        let assign2560_e1787: f64 = (p.p127 * assign2560_e1786);
        (assign2560_e1787,)
    } else {
        (var_uc_svgs,)
    }
};
        var_uc_svgs = assign2560_e1789;
        var_uc_svgs_rv = 0.0;

        let assign2570_e1792: f64 = (p.p115 * var_lgle);
        let assign2570_e1794: f64 = (assign2570_e1792 * p.p114);
        let assign2570_e1797: f64 = (p.p115 * var_lgle);
        let assign2570_e1799: f64 = (assign2570_e1797 + p.p114);
        let assign2570_e1800: f64 = (assign2570_e1794 / assign2570_e1799);
        let assign2570_e1802: f64 = (assign2570_e1800 + p.p116);
        let assign2570_e1804: f64 = (assign2570_e1802 + 1e-50);
        var_ddlte = assign2570_e1804;
        var_ddlte_rv = 0.0;

        let assign2580_e1807: f64 = if var_ddlte < 3.0 { 1.0 } else { 0.0 };
        var_guard16 = assign2580_e1807;
        var_guard16_rv = 0.0;

        let (assign2590_e1811,) = {
    if (var_guard16 != 0.0) {
        (3.0,)
    } else {
        (var_ddlte,)
    }
};
        var_ddlte = assign2590_e1811;
        var_ddlte_rv = 0.0;

        let assign2600_e1814: f64 = (p.p50 * p.p253);
        var_vgs_min = assign2600_e1814;
        var_vgs_min_rv = 0.0;

        let assign2610_e1816: f64 = if param_given[168] { 1.0 } else { 0.0 };
        var_cgbo_given = assign2610_e1816;
        var_cgbo_given_rv = 0.0;

        let assign2620_e1818: f64 = if param_given[169] { 1.0 } else { 0.0 };
        var_cgdo_given = assign2620_e1818;
        var_cgdo_given_rv = 0.0;

        let assign2630_e1820: f64 = if param_given[170] { 1.0 } else { 0.0 };
        var_cgso_given = assign2630_e1820;
        var_cgso_given_rv = 0.0;

        let assign2640_e1822: f64 = if param_given[294] { 1.0 } else { 0.0 };
        var_cbtbp_given = assign2640_e1822;
        var_cbtbp_given_rv = 0.0;

        let assign2650_e1824: f64 = if param_given[293] { 1.0 } else { 0.0 };
        var_cbtbn_given = assign2650_e1824;
        var_cbtbn_given_rv = 0.0;

        let assign2660_e1826: f64 = if param_given[13] { 1.0 } else { 0.0 };
        var_pdbcp_given = assign2660_e1826;
        var_pdbcp_given_rv = 0.0;

        let assign2670_e1828: f64 = if param_given[14] { 1.0 } else { 0.0 };
        var_psbcp_given = assign2670_e1828;
        var_psbcp_given_rv = 0.0;

        let assign2680_e1830: f64 = if param_given[23] { 1.0 } else { 0.0 };
        var_abtp_given = assign2680_e1830;
        var_abtp_given_rv = 0.0;

        let assign2690_e1832: f64 = if param_given[22] { 1.0 } else { 0.0 };
        var_abtn_given = assign2690_e1832;
        var_abtn_given_rv = 0.0;

        let assign2700_e1834: f64 = if param_given[16] { 1.0 } else { 0.0 };
        var_temp_given = assign2700_e1834;
        var_temp_given_rv = 0.0;

        let (assign2710_e1840,) = {
    if (p.p17 == 0.0) {
        (0.0,)
    } else {
        (1.0,)
    }
};
        var_dtemp_given = assign2710_e1840;
        var_dtemp_given_rv = 0.0;

        var_mfactor = 1.0;
        var_mfactor_rv = 0.0;

        let assign2730_e1844: f64 = 0.0;
        var_gjmin = assign2730_e1844;
        var_gjmin_rv = 0.0;

        var_uc_pdbcp = p.p13;
        var_uc_pdbcp_rv = 0.0;

        var_uc_psbcp = p.p14;
        var_uc_psbcp_rv = 0.0;

        let assign2760_e1849: f64 = (p.p16 + 273.15);
        var_uc_temp = assign2760_e1849;
        var_uc_temp_rv = 0.0;

        let assign2780_e1858: f64 = (var_mfactor * var_weffcv_nf);
        let assign2780_e1859: f64 = (var_mks_cth0 * assign2780_e1858);
        var_cth = assign2780_e1859;
        var_cth_rv = 0.0;

        let assign2790_e1878: f64 = if (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p9 == 1.0) || ((p.p9 > 1.0) && (p.p12 > 0.0)))) { 1.0 } else { 0.0 };
        var_guard17 = assign2790_e1878;
        var_guard17_rv = 0.0;

        let (assign2800_e1882, assign2800_e1882_d_n0, assign2800_e1882_d_n2, assign2800_e1882_d_n6, assign2800_e1882_d_n7, assign2800_e1882_d_n10, assign2800_e1882_d_n11, assign2800_e1882_d_n12, assign2800_e1882_d_n17,) = {
    if (var_guard17 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign2800_e1882;
        var_t1_dn0 = assign2800_e1882_d_n0;
        var_t1_dn2 = assign2800_e1882_d_n2;
        var_t1_dn6 = assign2800_e1882_d_n6;
        var_t1_dn7 = assign2800_e1882_d_n7;
        var_t1_dn10 = assign2800_e1882_d_n10;
        var_t1_dn11 = assign2800_e1882_d_n11;
        var_t1_dn12 = assign2800_e1882_d_n12;
        var_t1_dn17 = assign2800_e1882_d_n17;
        var_t1_rv = 0.0;

        let (assign2810_e1886,) = {
    if (var_guard17 != 0.0) {
        (0.0,)
    } else {
        (var_i,)
    }
};
        var_i = assign2810_e1886;
        var_i_rv = 0.0;

        let mut assign2820_loop_guard: usize = 0;
        while {
            let assign2820_cond_e1891: f64 = if ((var_guard17 != 0.0) && (var_i < p.p9)) { 1.0 } else { 0.0 };
            assign2820_cond_e1891 != 0.0
        } {
            assign2820_loop_guard += 1;
            assert!(assign2820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign2820_body0_e1923, assign2820_body0_e1923_d_n0, assign2820_body0_e1923_d_n2, assign2820_body0_e1923_d_n6, assign2820_body0_e1923_d_n7, assign2820_body0_e1923_d_n10, assign2820_body0_e1923_d_n11, assign2820_body0_e1923_d_n12, assign2820_body0_e1923_d_n17,) = {
    if (var_guard17 != 0.0) {
        let assign2820_body0_e1898: f64 = (0.5 * var_lgate);
        let assign2820_body0_e1899: f64 = (p.p10 + assign2820_body0_e1898);
        let assign2820_body0_e1903: f64 = (p.p12 + var_lgate);
        let assign2820_body0_e1904: f64 = (var_i * assign2820_body0_e1903);
        let assign2820_body0_e1905: f64 = (assign2820_body0_e1899 + assign2820_body0_e1904);
        let assign2820_body0_e1906: f64 = (1.0 / assign2820_body0_e1905);
        let assign2820_body0_e1907: f64 = (var_t1 + assign2820_body0_e1906);
        let assign2820_body0_e1912: f64 = (0.5 * var_lgate);
        let assign2820_body0_e1913: f64 = (p.p11 + assign2820_body0_e1912);
        let assign2820_body0_e1917: f64 = (p.p12 + var_lgate);
        let assign2820_body0_e1918: f64 = (var_i * assign2820_body0_e1917);
        let assign2820_body0_e1919: f64 = (assign2820_body0_e1913 + assign2820_body0_e1918);
        let assign2820_body0_e1920: f64 = (1.0 / assign2820_body0_e1919);
        let assign2820_body0_e1921: f64 = (assign2820_body0_e1907 + assign2820_body0_e1920);
        (assign2820_body0_e1921, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
            var_t1 = assign2820_body0_e1923;
            var_t1_dn0 = assign2820_body0_e1923_d_n0;
            var_t1_dn2 = assign2820_body0_e1923_d_n2;
            var_t1_dn6 = assign2820_body0_e1923_d_n6;
            var_t1_dn7 = assign2820_body0_e1923_d_n7;
            var_t1_dn10 = assign2820_body0_e1923_d_n10;
            var_t1_dn11 = assign2820_body0_e1923_d_n11;
            var_t1_dn12 = assign2820_body0_e1923_d_n12;
            var_t1_dn17 = assign2820_body0_e1923_d_n17;
            var_t1_rv = 0.0;
            let (assign2820_body1_e1929,) = {
    if (var_guard17 != 0.0) {
        let assign2820_body1_e1927: f64 = (var_i + 1.0);
        (assign2820_body1_e1927,)
    } else {
        (var_i,)
    }
};
            var_i = assign2820_body1_e1929;
            var_i_rv = 0.0;
        }

        let (assign2830_e1937, assign2830_e1937_d_n0, assign2830_e1937_d_n2, assign2830_e1937_d_n6, assign2830_e1937_d_n7, assign2830_e1937_d_n10, assign2830_e1937_d_n11, assign2830_e1937_d_n12, assign2830_e1937_d_n17,) = {
    if (var_guard17 != 0.0) {
        let assign2830_e1933: f64 = (2.0 * p.p9);
        let assign2830_e1935: f64 = (assign2830_e1933 / var_t1);
        (assign2830_e1935, (-((assign2830_e1933 * var_t1_dn0) / (var_t1 * var_t1))), (-((assign2830_e1933 * var_t1_dn2) / (var_t1 * var_t1))), (-((assign2830_e1933 * var_t1_dn6) / (var_t1 * var_t1))), (-((assign2830_e1933 * var_t1_dn7) / (var_t1 * var_t1))), (-((assign2830_e1933 * var_t1_dn10) / (var_t1 * var_t1))), (-((assign2830_e1933 * var_t1_dn11) / (var_t1 * var_t1))), (-((assign2830_e1933 * var_t1_dn12) / (var_t1 * var_t1))), (-((assign2830_e1933 * var_t1_dn17) / (var_t1 * var_t1))),)
    } else {
        (var_lod_half, var_lod_half_dn0, var_lod_half_dn2, var_lod_half_dn6, var_lod_half_dn7, var_lod_half_dn10, var_lod_half_dn11, var_lod_half_dn12, var_lod_half_dn17,)
    }
};
        var_lod_half = assign2830_e1937;
        var_lod_half_dn0 = assign2830_e1937_d_n0;
        var_lod_half_dn2 = assign2830_e1937_d_n2;
        var_lod_half_dn6 = assign2830_e1937_d_n6;
        var_lod_half_dn7 = assign2830_e1937_d_n7;
        var_lod_half_dn10 = assign2830_e1937_d_n10;
        var_lod_half_dn11 = assign2830_e1937_d_n11;
        var_lod_half_dn12 = assign2830_e1937_d_n12;
        var_lod_half_dn17 = assign2830_e1937_d_n17;
        var_lod_half_rv = 0.0;

        let (assign2840_e1942, assign2840_e1942_d_n0, assign2840_e1942_d_n2, assign2840_e1942_d_n6, assign2840_e1942_d_n7, assign2840_e1942_d_n10, assign2840_e1942_d_n11, assign2840_e1942_d_n12, assign2840_e1942_d_n17,) = {
    if (var_guard17 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_lod_half, var_lod_half_dn0, var_lod_half_dn2, var_lod_half_dn6, var_lod_half_dn7, var_lod_half_dn10, var_lod_half_dn11, var_lod_half_dn12, var_lod_half_dn17,)
    }
};
        var_lod_half = assign2840_e1942;
        var_lod_half_dn0 = assign2840_e1942_d_n0;
        var_lod_half_dn2 = assign2840_e1942_d_n2;
        var_lod_half_dn6 = assign2840_e1942_d_n6;
        var_lod_half_dn7 = assign2840_e1942_d_n7;
        var_lod_half_dn10 = assign2840_e1942_d_n10;
        var_lod_half_dn11 = assign2840_e1942_d_n11;
        var_lod_half_dn12 = assign2840_e1942_d_n12;
        var_lod_half_dn17 = assign2840_e1942_d_n17;
        var_lod_half_rv = 0.0;

        let assign2850_e1945: f64 = if var_lod_half > 0.0 { 1.0 } else { 0.0 };
        var_guard18 = assign2850_e1945;
        var_guard18_rv = 0.0;

        let (assign2860_e1953, assign2860_e1953_d_n0, assign2860_e1953_d_n2, assign2860_e1953_d_n6, assign2860_e1953_d_n7, assign2860_e1953_d_n10, assign2860_e1953_d_n11, assign2860_e1953_d_n12, assign2860_e1953_d_n17,) = {
    if (var_guard18 != 0.0) {
        let assign2860_e1950: f64 = (1.0 + p.p162);
        let assign2860_e1951: f64 = (1.0 / assign2860_e1950);
        (assign2860_e1951, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign2860_e1953;
        var_t1_dn0 = assign2860_e1953_d_n0;
        var_t1_dn2 = assign2860_e1953_d_n2;
        var_t1_dn6 = assign2860_e1953_d_n6;
        var_t1_dn7 = assign2860_e1953_d_n7;
        var_t1_dn10 = assign2860_e1953_d_n10;
        var_t1_dn11 = assign2860_e1953_d_n11;
        var_t1_dn12 = assign2860_e1953_d_n12;
        var_t1_dn17 = assign2860_e1953_d_n17;
        var_t1_rv = 0.0;

        let (assign2870_e1961, assign2870_e1961_d_n0, assign2870_e1961_d_n2, assign2870_e1961_d_n6, assign2870_e1961_d_n7, assign2870_e1961_d_n10, assign2870_e1961_d_n11, assign2870_e1961_d_n12, assign2870_e1961_d_n17,) = {
    if (var_guard18 != 0.0) {
        let assign2870_e1957: f64 = (p.p161 / var_lod_half);
        let assign2870_e1959: f64 = (assign2870_e1957).powf(p.p163);
        (assign2870_e1959, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2870_e1957).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn0) / (var_lod_half * var_lod_half))))) } } else { (assign2870_e1959 * (p.p163 * ((-((p.p161 * var_lod_half_dn0) / (var_lod_half * var_lod_half))) / assign2870_e1957))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2870_e1957).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn2) / (var_lod_half * var_lod_half))))) } } else { (assign2870_e1959 * (p.p163 * ((-((p.p161 * var_lod_half_dn2) / (var_lod_half * var_lod_half))) / assign2870_e1957))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2870_e1957).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn6) / (var_lod_half * var_lod_half))))) } } else { (assign2870_e1959 * (p.p163 * ((-((p.p161 * var_lod_half_dn6) / (var_lod_half * var_lod_half))) / assign2870_e1957))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2870_e1957).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn7) / (var_lod_half * var_lod_half))))) } } else { (assign2870_e1959 * (p.p163 * ((-((p.p161 * var_lod_half_dn7) / (var_lod_half * var_lod_half))) / assign2870_e1957))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2870_e1957).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn10) / (var_lod_half * var_lod_half))))) } } else { (assign2870_e1959 * (p.p163 * ((-((p.p161 * var_lod_half_dn10) / (var_lod_half * var_lod_half))) / assign2870_e1957))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2870_e1957).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn11) / (var_lod_half * var_lod_half))))) } } else { (assign2870_e1959 * (p.p163 * ((-((p.p161 * var_lod_half_dn11) / (var_lod_half * var_lod_half))) / assign2870_e1957))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2870_e1957).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn12) / (var_lod_half * var_lod_half))))) } } else { (assign2870_e1959 * (p.p163 * ((-((p.p161 * var_lod_half_dn12) / (var_lod_half * var_lod_half))) / assign2870_e1957))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2870_e1957).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn17) / (var_lod_half * var_lod_half))))) } } else { (assign2870_e1959 * (p.p163 * ((-((p.p161 * var_lod_half_dn17) / (var_lod_half * var_lod_half))) / assign2870_e1957))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign2870_e1961;
        var_t2_dn0 = assign2870_e1961_d_n0;
        var_t2_dn2 = assign2870_e1961_d_n2;
        var_t2_dn6 = assign2870_e1961_d_n6;
        var_t2_dn7 = assign2870_e1961_d_n7;
        var_t2_dn10 = assign2870_e1961_d_n10;
        var_t2_dn11 = assign2870_e1961_d_n11;
        var_t2_dn12 = assign2870_e1961_d_n12;
        var_t2_dn17 = assign2870_e1961_d_n17;
        var_t2_rv = 0.0;

        let (assign2880_e1969, assign2880_e1969_d_n0, assign2880_e1969_d_n2, assign2880_e1969_d_n6, assign2880_e1969_d_n7, assign2880_e1969_d_n10, assign2880_e1969_d_n11, assign2880_e1969_d_n12, assign2880_e1969_d_n17,) = {
    if (var_guard18 != 0.0) {
        let assign2880_e1965: f64 = (p.p161 / var_lod_half_ref);
        let assign2880_e1967: f64 = (assign2880_e1965).powf(p.p163);
        (assign2880_e1967, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign2880_e1969;
        var_t3_dn0 = assign2880_e1969_d_n0;
        var_t3_dn2 = assign2880_e1969_d_n2;
        var_t3_dn6 = assign2880_e1969_d_n6;
        var_t3_dn7 = assign2880_e1969_d_n7;
        var_t3_dn10 = assign2880_e1969_d_n10;
        var_t3_dn11 = assign2880_e1969_d_n11;
        var_t3_dn12 = assign2880_e1969_d_n12;
        var_t3_dn17 = assign2880_e1969_d_n17;
        var_t3_rv = 0.0;

        *var_abtn_given_slot = var_abtn_given;
        *var_abtn_given_rv_slot = var_abtn_given_rv;
        *var_abtp_given_slot = var_abtp_given;
        *var_abtp_given_rv_slot = var_abtp_given_rv;
        *var_cbtbn_given_slot = var_cbtbn_given;
        *var_cbtbn_given_rv_slot = var_cbtbn_given_rv;
        *var_cbtbp_given_slot = var_cbtbp_given;
        *var_cbtbp_given_rv_slot = var_cbtbp_given_rv;
        *var_cgbo_given_slot = var_cgbo_given;
        *var_cgbo_given_rv_slot = var_cgbo_given_rv;
        *var_cgdo_given_slot = var_cgdo_given;
        *var_cgdo_given_rv_slot = var_cgdo_given_rv;
        *var_cgso_given_slot = var_cgso_given;
        *var_cgso_given_rv_slot = var_cgso_given_rv;
        *var_clmmod_slot = var_clmmod;
        *var_clmmod_rv_slot = var_clmmod_rv;
        *var_cqyb0_slot = var_cqyb0;
        *var_cqyb0_rv_slot = var_cqyb0_rv;
        *var_cth_slot = var_cth;
        *var_cth_rv_slot = var_cth_rv;
        *var_ddlte_slot = var_ddlte;
        *var_ddlte_rv_slot = var_ddlte_rv;
        *var_dtemp_given_slot = var_dtemp_given;
        *var_dtemp_given_rv_slot = var_dtemp_given_rv;
        *var_gjmin_slot = var_gjmin;
        *var_gjmin_rv_slot = var_gjmin_rv;
        *var_guard14_slot = var_guard14;
        *var_guard14_rv_slot = var_guard14_rv;
        *var_guard15_slot = var_guard15;
        *var_guard15_rv_slot = var_guard15_rv;
        *var_guard16_slot = var_guard16;
        *var_guard16_rv_slot = var_guard16_rv;
        *var_guard17_slot = var_guard17;
        *var_guard17_rv_slot = var_guard17_rv;
        *var_guard18_slot = var_guard18;
        *var_guard18_rv_slot = var_guard18_rv;
        *var_i_slot = var_i;
        *var_i_rv_slot = var_i_rv;
        *var_lod_half_slot = var_lod_half;
        *var_lod_half_dn0_slot = var_lod_half_dn0;
        *var_lod_half_dn10_slot = var_lod_half_dn10;
        *var_lod_half_dn11_slot = var_lod_half_dn11;
        *var_lod_half_dn12_slot = var_lod_half_dn12;
        *var_lod_half_dn17_slot = var_lod_half_dn17;
        *var_lod_half_dn2_slot = var_lod_half_dn2;
        *var_lod_half_dn6_slot = var_lod_half_dn6;
        *var_lod_half_dn7_slot = var_lod_half_dn7;
        *var_lod_half_rv_slot = var_lod_half_rv;
        *var_mfactor_slot = var_mfactor;
        *var_mfactor_rv_slot = var_mfactor_rv;
        *var_pdbcp_given_slot = var_pdbcp_given;
        *var_pdbcp_given_rv_slot = var_pdbcp_given_rv;
        *var_psbcp_given_slot = var_psbcp_given;
        *var_psbcp_given_rv_slot = var_psbcp_given_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_rv_slot = var_t3_rv;
        *var_temp_given_slot = var_temp_given;
        *var_temp_given_rv_slot = var_temp_given_rv;
        *var_uc_pdbcp_slot = var_uc_pdbcp;
        *var_uc_pdbcp_rv_slot = var_uc_pdbcp_rv;
        *var_uc_psbcp_slot = var_uc_psbcp;
        *var_uc_psbcp_rv_slot = var_uc_psbcp_rv;
        *var_uc_svgs_slot = var_uc_svgs;
        *var_uc_svgs_rv_slot = var_uc_svgs_rv;
        *var_uc_temp_slot = var_uc_temp;
        *var_uc_temp_rv_slot = var_uc_temp_rv;
        *var_vfbsub0_slot = var_vfbsub0;
        *var_vfbsub0_rv_slot = var_vfbsub0_rv;
        *var_vg2const_slot = var_vg2const;
        *var_vg2const_dn0_slot = var_vg2const_dn0;
        *var_vg2const_dn10_slot = var_vg2const_dn10;
        *var_vg2const_dn11_slot = var_vg2const_dn11;
        *var_vg2const_dn12_slot = var_vg2const_dn12;
        *var_vg2const_dn17_slot = var_vg2const_dn17;
        *var_vg2const_dn2_slot = var_vg2const_dn2;
        *var_vg2const_dn6_slot = var_vg2const_dn6;
        *var_vg2const_dn7_slot = var_vg2const_dn7;
        *var_vg2const_rv_slot = var_vg2const_rv;
        *var_vgs_min_slot = var_vgs_min;
        *var_vgs_min_rv_slot = var_vgs_min_rv;
        *var_xgate_slot = var_xgate;
        *var_xgate_rv_slot = var_xgate_rv;
        *var_xsub1_slot = var_xsub1;
        *var_xsub1_rv_slot = var_xsub1_rv;
        *var_xsub2_slot = var_xsub2;
        *var_xsub2_rv_slot = var_xsub2_rv;
        *var_xvbs_slot = var_xvbs;
        *var_xvbs_rv_slot = var_xvbs_rv;
        *var_zvgs_slot = var_zvgs;
        *var_zvgs_rv_slot = var_zvgs_rv;
    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        var_guard18: f64,
        var_lgle: f64,
        var_lgleff: f64,
        var_lod_half: f64,
        var_lod_half_dn0: f64,
        var_lod_half_dn10: f64,
        var_lod_half_dn11: f64,
        var_lod_half_dn12: f64,
        var_lod_half_dn17: f64,
        var_lod_half_dn2: f64,
        var_lod_half_dn6: f64,
        var_lod_half_dn7: f64,
        var_lod_half_ref: f64,
        var_mks_nsubcmax: f64,
        var_mks_nsubs: f64,
        var_nsubpp: f64,
        var_wg: f64,
        var_wl: f64,
        var_guard19_slot: &mut f64,
        var_guard19_rv_slot: &mut f64,
        var_guard20_slot: &mut f64,
        var_guard20_rv_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard21_rv_slot: &mut f64,
        var_guard22_slot: &mut f64,
        var_guard22_rv_slot: &mut f64,
        var_nsub_slot: &mut f64,
        var_nsub_dn0_slot: &mut f64,
        var_nsub_dn10_slot: &mut f64,
        var_nsub_dn11_slot: &mut f64,
        var_nsub_dn12_slot: &mut f64,
        var_nsub_dn17_slot: &mut f64,
        var_nsub_dn2_slot: &mut f64,
        var_nsub_dn6_slot: &mut f64,
        var_nsub_dn7_slot: &mut f64,
        var_nsub_rv_slot: &mut f64,
        var_nsubb0_slot: &mut f64,
        var_nsubb0_dn0_slot: &mut f64,
        var_nsubb0_dn10_slot: &mut f64,
        var_nsubb0_dn11_slot: &mut f64,
        var_nsubb0_dn12_slot: &mut f64,
        var_nsubb0_dn17_slot: &mut f64,
        var_nsubb0_dn2_slot: &mut f64,
        var_nsubb0_dn6_slot: &mut f64,
        var_nsubb0_dn7_slot: &mut f64,
        var_nsubb0_rv_slot: &mut f64,
        var_nsubps_slot: &mut f64,
        var_nsubps_dn0_slot: &mut f64,
        var_nsubps_dn10_slot: &mut f64,
        var_nsubps_dn11_slot: &mut f64,
        var_nsubps_dn12_slot: &mut f64,
        var_nsubps_dn17_slot: &mut f64,
        var_nsubps_dn2_slot: &mut f64,
        var_nsubps_dn6_slot: &mut f64,
        var_nsubps_dn7_slot: &mut f64,
        var_nsubps_rv_slot: &mut f64,
        var_pb20_slot: &mut f64,
        var_pb20_dn0_slot: &mut f64,
        var_pb20_dn10_slot: &mut f64,
        var_pb20_dn11_slot: &mut f64,
        var_pb20_dn12_slot: &mut f64,
        var_pb20_dn17_slot: &mut f64,
        var_pb20_dn2_slot: &mut f64,
        var_pb20_dn6_slot: &mut f64,
        var_pb20_dn7_slot: &mut f64,
        var_pb20_rv_slot: &mut f64,
        var_pb2c_slot: &mut f64,
        var_pb2c_dn0_slot: &mut f64,
        var_pb2c_dn10_slot: &mut f64,
        var_pb2c_dn11_slot: &mut f64,
        var_pb2c_dn12_slot: &mut f64,
        var_pb2c_dn17_slot: &mut f64,
        var_pb2c_dn2_slot: &mut f64,
        var_pb2c_dn6_slot: &mut f64,
        var_pb2c_dn7_slot: &mut f64,
        var_pb2c_rv_slot: &mut f64,
        var_ptovr0_slot: &mut f64,
        var_ptovr0_dn0_slot: &mut f64,
        var_ptovr0_dn10_slot: &mut f64,
        var_ptovr0_dn11_slot: &mut f64,
        var_ptovr0_dn12_slot: &mut f64,
        var_ptovr0_dn17_slot: &mut f64,
        var_ptovr0_dn2_slot: &mut f64,
        var_ptovr0_dn6_slot: &mut f64,
        var_ptovr0_dn7_slot: &mut f64,
        var_ptovr0_rv_slot: &mut f64,
        var_q_nsub_slot: &mut f64,
        var_q_nsub_dn0_slot: &mut f64,
        var_q_nsub_dn10_slot: &mut f64,
        var_q_nsub_dn11_slot: &mut f64,
        var_q_nsub_dn12_slot: &mut f64,
        var_q_nsub_dn17_slot: &mut f64,
        var_q_nsub_dn2_slot: &mut f64,
        var_q_nsub_dn6_slot: &mut f64,
        var_q_nsub_dn7_slot: &mut f64,
        var_q_nsub_rv_slot: &mut f64,
        var_qnsub_esi_slot: &mut f64,
        var_qnsub_esi2_slot: &mut f64,
        var_qnsub_esi2_dn0_slot: &mut f64,
        var_qnsub_esi2_dn10_slot: &mut f64,
        var_qnsub_esi2_dn11_slot: &mut f64,
        var_qnsub_esi2_dn12_slot: &mut f64,
        var_qnsub_esi2_dn17_slot: &mut f64,
        var_qnsub_esi2_dn2_slot: &mut f64,
        var_qnsub_esi2_dn6_slot: &mut f64,
        var_qnsub_esi2_dn7_slot: &mut f64,
        var_qnsub_esi2_rv_slot: &mut f64,
        var_qnsub_esi_dn0_slot: &mut f64,
        var_qnsub_esi_dn10_slot: &mut f64,
        var_qnsub_esi_dn11_slot: &mut f64,
        var_qnsub_esi_dn12_slot: &mut f64,
        var_qnsub_esi_dn17_slot: &mut f64,
        var_qnsub_esi_dn2_slot: &mut f64,
        var_qnsub_esi_dn6_slot: &mut f64,
        var_qnsub_esi_dn7_slot: &mut f64,
        var_qnsub_esi_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_uc_nsubs_slot: &mut f64,
        var_uc_nsubs_dn0_slot: &mut f64,
        var_uc_nsubs_dn10_slot: &mut f64,
        var_uc_nsubs_dn11_slot: &mut f64,
        var_uc_nsubs_dn12_slot: &mut f64,
        var_uc_nsubs_dn17_slot: &mut f64,
        var_uc_nsubs_dn2_slot: &mut f64,
        var_uc_nsubs_dn6_slot: &mut f64,
        var_uc_nsubs_dn7_slot: &mut f64,
        var_uc_nsubs_rv_slot: &mut f64,
        var_vmax0_slot: &mut f64,
        var_vmax0_dn0_slot: &mut f64,
        var_vmax0_dn10_slot: &mut f64,
        var_vmax0_dn11_slot: &mut f64,
        var_vmax0_dn12_slot: &mut f64,
        var_vmax0_dn17_slot: &mut f64,
        var_vmax0_dn2_slot: &mut f64,
        var_vmax0_dn6_slot: &mut f64,
        var_vmax0_dn7_slot: &mut f64,
        var_vmax0_rv_slot: &mut f64,
        var_wdpl_slot: &mut f64,
        var_wdpl_dn0_slot: &mut f64,
        var_wdpl_dn10_slot: &mut f64,
        var_wdpl_dn11_slot: &mut f64,
        var_wdpl_dn12_slot: &mut f64,
        var_wdpl_dn17_slot: &mut f64,
        var_wdpl_dn2_slot: &mut f64,
        var_wdpl_dn6_slot: &mut f64,
        var_wdpl_dn7_slot: &mut f64,
        var_wdpl_rv_slot: &mut f64,
    ) {
        let mut var_guard19: f64 = *var_guard19_slot;
        let mut var_guard19_rv: f64 = *var_guard19_rv_slot;
        let mut var_guard20: f64 = *var_guard20_slot;
        let mut var_guard20_rv: f64 = *var_guard20_rv_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard21_rv: f64 = *var_guard21_rv_slot;
        let mut var_guard22: f64 = *var_guard22_slot;
        let mut var_guard22_rv: f64 = *var_guard22_rv_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
        let mut var_nsub_dn0: f64 = *var_nsub_dn0_slot;
        let mut var_nsub_dn10: f64 = *var_nsub_dn10_slot;
        let mut var_nsub_dn11: f64 = *var_nsub_dn11_slot;
        let mut var_nsub_dn12: f64 = *var_nsub_dn12_slot;
        let mut var_nsub_dn17: f64 = *var_nsub_dn17_slot;
        let mut var_nsub_dn2: f64 = *var_nsub_dn2_slot;
        let mut var_nsub_dn6: f64 = *var_nsub_dn6_slot;
        let mut var_nsub_dn7: f64 = *var_nsub_dn7_slot;
        let mut var_nsub_rv: f64 = *var_nsub_rv_slot;
        let mut var_nsubb0: f64 = *var_nsubb0_slot;
        let mut var_nsubb0_dn0: f64 = *var_nsubb0_dn0_slot;
        let mut var_nsubb0_dn10: f64 = *var_nsubb0_dn10_slot;
        let mut var_nsubb0_dn11: f64 = *var_nsubb0_dn11_slot;
        let mut var_nsubb0_dn12: f64 = *var_nsubb0_dn12_slot;
        let mut var_nsubb0_dn17: f64 = *var_nsubb0_dn17_slot;
        let mut var_nsubb0_dn2: f64 = *var_nsubb0_dn2_slot;
        let mut var_nsubb0_dn6: f64 = *var_nsubb0_dn6_slot;
        let mut var_nsubb0_dn7: f64 = *var_nsubb0_dn7_slot;
        let mut var_nsubb0_rv: f64 = *var_nsubb0_rv_slot;
        let mut var_nsubps: f64 = *var_nsubps_slot;
        let mut var_nsubps_dn0: f64 = *var_nsubps_dn0_slot;
        let mut var_nsubps_dn10: f64 = *var_nsubps_dn10_slot;
        let mut var_nsubps_dn11: f64 = *var_nsubps_dn11_slot;
        let mut var_nsubps_dn12: f64 = *var_nsubps_dn12_slot;
        let mut var_nsubps_dn17: f64 = *var_nsubps_dn17_slot;
        let mut var_nsubps_dn2: f64 = *var_nsubps_dn2_slot;
        let mut var_nsubps_dn6: f64 = *var_nsubps_dn6_slot;
        let mut var_nsubps_dn7: f64 = *var_nsubps_dn7_slot;
        let mut var_nsubps_rv: f64 = *var_nsubps_rv_slot;
        let mut var_pb20: f64 = *var_pb20_slot;
        let mut var_pb20_dn0: f64 = *var_pb20_dn0_slot;
        let mut var_pb20_dn10: f64 = *var_pb20_dn10_slot;
        let mut var_pb20_dn11: f64 = *var_pb20_dn11_slot;
        let mut var_pb20_dn12: f64 = *var_pb20_dn12_slot;
        let mut var_pb20_dn17: f64 = *var_pb20_dn17_slot;
        let mut var_pb20_dn2: f64 = *var_pb20_dn2_slot;
        let mut var_pb20_dn6: f64 = *var_pb20_dn6_slot;
        let mut var_pb20_dn7: f64 = *var_pb20_dn7_slot;
        let mut var_pb20_rv: f64 = *var_pb20_rv_slot;
        let mut var_pb2c: f64 = *var_pb2c_slot;
        let mut var_pb2c_dn0: f64 = *var_pb2c_dn0_slot;
        let mut var_pb2c_dn10: f64 = *var_pb2c_dn10_slot;
        let mut var_pb2c_dn11: f64 = *var_pb2c_dn11_slot;
        let mut var_pb2c_dn12: f64 = *var_pb2c_dn12_slot;
        let mut var_pb2c_dn17: f64 = *var_pb2c_dn17_slot;
        let mut var_pb2c_dn2: f64 = *var_pb2c_dn2_slot;
        let mut var_pb2c_dn6: f64 = *var_pb2c_dn6_slot;
        let mut var_pb2c_dn7: f64 = *var_pb2c_dn7_slot;
        let mut var_pb2c_rv: f64 = *var_pb2c_rv_slot;
        let mut var_ptovr0: f64 = *var_ptovr0_slot;
        let mut var_ptovr0_dn0: f64 = *var_ptovr0_dn0_slot;
        let mut var_ptovr0_dn10: f64 = *var_ptovr0_dn10_slot;
        let mut var_ptovr0_dn11: f64 = *var_ptovr0_dn11_slot;
        let mut var_ptovr0_dn12: f64 = *var_ptovr0_dn12_slot;
        let mut var_ptovr0_dn17: f64 = *var_ptovr0_dn17_slot;
        let mut var_ptovr0_dn2: f64 = *var_ptovr0_dn2_slot;
        let mut var_ptovr0_dn6: f64 = *var_ptovr0_dn6_slot;
        let mut var_ptovr0_dn7: f64 = *var_ptovr0_dn7_slot;
        let mut var_ptovr0_rv: f64 = *var_ptovr0_rv_slot;
        let mut var_q_nsub: f64 = *var_q_nsub_slot;
        let mut var_q_nsub_dn0: f64 = *var_q_nsub_dn0_slot;
        let mut var_q_nsub_dn10: f64 = *var_q_nsub_dn10_slot;
        let mut var_q_nsub_dn11: f64 = *var_q_nsub_dn11_slot;
        let mut var_q_nsub_dn12: f64 = *var_q_nsub_dn12_slot;
        let mut var_q_nsub_dn17: f64 = *var_q_nsub_dn17_slot;
        let mut var_q_nsub_dn2: f64 = *var_q_nsub_dn2_slot;
        let mut var_q_nsub_dn6: f64 = *var_q_nsub_dn6_slot;
        let mut var_q_nsub_dn7: f64 = *var_q_nsub_dn7_slot;
        let mut var_q_nsub_rv: f64 = *var_q_nsub_rv_slot;
        let mut var_qnsub_esi: f64 = *var_qnsub_esi_slot;
        let mut var_qnsub_esi2: f64 = *var_qnsub_esi2_slot;
        let mut var_qnsub_esi2_dn0: f64 = *var_qnsub_esi2_dn0_slot;
        let mut var_qnsub_esi2_dn10: f64 = *var_qnsub_esi2_dn10_slot;
        let mut var_qnsub_esi2_dn11: f64 = *var_qnsub_esi2_dn11_slot;
        let mut var_qnsub_esi2_dn12: f64 = *var_qnsub_esi2_dn12_slot;
        let mut var_qnsub_esi2_dn17: f64 = *var_qnsub_esi2_dn17_slot;
        let mut var_qnsub_esi2_dn2: f64 = *var_qnsub_esi2_dn2_slot;
        let mut var_qnsub_esi2_dn6: f64 = *var_qnsub_esi2_dn6_slot;
        let mut var_qnsub_esi2_dn7: f64 = *var_qnsub_esi2_dn7_slot;
        let mut var_qnsub_esi2_rv: f64 = *var_qnsub_esi2_rv_slot;
        let mut var_qnsub_esi_dn0: f64 = *var_qnsub_esi_dn0_slot;
        let mut var_qnsub_esi_dn10: f64 = *var_qnsub_esi_dn10_slot;
        let mut var_qnsub_esi_dn11: f64 = *var_qnsub_esi_dn11_slot;
        let mut var_qnsub_esi_dn12: f64 = *var_qnsub_esi_dn12_slot;
        let mut var_qnsub_esi_dn17: f64 = *var_qnsub_esi_dn17_slot;
        let mut var_qnsub_esi_dn2: f64 = *var_qnsub_esi_dn2_slot;
        let mut var_qnsub_esi_dn6: f64 = *var_qnsub_esi_dn6_slot;
        let mut var_qnsub_esi_dn7: f64 = *var_qnsub_esi_dn7_slot;
        let mut var_qnsub_esi_rv: f64 = *var_qnsub_esi_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_uc_nsubs: f64 = *var_uc_nsubs_slot;
        let mut var_uc_nsubs_dn0: f64 = *var_uc_nsubs_dn0_slot;
        let mut var_uc_nsubs_dn10: f64 = *var_uc_nsubs_dn10_slot;
        let mut var_uc_nsubs_dn11: f64 = *var_uc_nsubs_dn11_slot;
        let mut var_uc_nsubs_dn12: f64 = *var_uc_nsubs_dn12_slot;
        let mut var_uc_nsubs_dn17: f64 = *var_uc_nsubs_dn17_slot;
        let mut var_uc_nsubs_dn2: f64 = *var_uc_nsubs_dn2_slot;
        let mut var_uc_nsubs_dn6: f64 = *var_uc_nsubs_dn6_slot;
        let mut var_uc_nsubs_dn7: f64 = *var_uc_nsubs_dn7_slot;
        let mut var_uc_nsubs_rv: f64 = *var_uc_nsubs_rv_slot;
        let mut var_vmax0: f64 = *var_vmax0_slot;
        let mut var_vmax0_dn0: f64 = *var_vmax0_dn0_slot;
        let mut var_vmax0_dn10: f64 = *var_vmax0_dn10_slot;
        let mut var_vmax0_dn11: f64 = *var_vmax0_dn11_slot;
        let mut var_vmax0_dn12: f64 = *var_vmax0_dn12_slot;
        let mut var_vmax0_dn17: f64 = *var_vmax0_dn17_slot;
        let mut var_vmax0_dn2: f64 = *var_vmax0_dn2_slot;
        let mut var_vmax0_dn6: f64 = *var_vmax0_dn6_slot;
        let mut var_vmax0_dn7: f64 = *var_vmax0_dn7_slot;
        let mut var_vmax0_rv: f64 = *var_vmax0_rv_slot;
        let mut var_wdpl: f64 = *var_wdpl_slot;
        let mut var_wdpl_dn0: f64 = *var_wdpl_dn0_slot;
        let mut var_wdpl_dn10: f64 = *var_wdpl_dn10_slot;
        let mut var_wdpl_dn11: f64 = *var_wdpl_dn11_slot;
        let mut var_wdpl_dn12: f64 = *var_wdpl_dn12_slot;
        let mut var_wdpl_dn17: f64 = *var_wdpl_dn17_slot;
        let mut var_wdpl_dn2: f64 = *var_wdpl_dn2_slot;
        let mut var_wdpl_dn6: f64 = *var_wdpl_dn6_slot;
        let mut var_wdpl_dn7: f64 = *var_wdpl_dn7_slot;
        let mut var_wdpl_rv: f64 = *var_wdpl_rv_slot;

        let (assign2890_e1985, assign2890_e1985_d_n0, assign2890_e1985_d_n2, assign2890_e1985_d_n6, assign2890_e1985_d_n7, assign2890_e1985_d_n10, assign2890_e1985_d_n11, assign2890_e1985_d_n12, assign2890_e1985_d_n17,) = {
    if (var_guard18 != 0.0) {
        let assign2890_e1975: f64 = (var_t1 * var_t2);
        let assign2890_e1976: f64 = (1.0 + assign2890_e1975);
        let assign2890_e1977: f64 = (var_nsubpp * assign2890_e1976);
        let assign2890_e1981: f64 = (var_t1 * var_t3);
        let assign2890_e1982: f64 = (1.0 + assign2890_e1981);
        let assign2890_e1983: f64 = (assign2890_e1977 / assign2890_e1982);
        (assign2890_e1983, ((((var_nsubpp * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0))) * assign2890_e1982) - (assign2890_e1977 * ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)))) / (assign2890_e1982 * assign2890_e1982)), ((((var_nsubpp * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2))) * assign2890_e1982) - (assign2890_e1977 * ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)))) / (assign2890_e1982 * assign2890_e1982)), ((((var_nsubpp * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6))) * assign2890_e1982) - (assign2890_e1977 * ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)))) / (assign2890_e1982 * assign2890_e1982)), ((((var_nsubpp * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7))) * assign2890_e1982) - (assign2890_e1977 * ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)))) / (assign2890_e1982 * assign2890_e1982)), ((((var_nsubpp * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10))) * assign2890_e1982) - (assign2890_e1977 * ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)))) / (assign2890_e1982 * assign2890_e1982)), ((((var_nsubpp * ((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11))) * assign2890_e1982) - (assign2890_e1977 * ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)))) / (assign2890_e1982 * assign2890_e1982)), ((((var_nsubpp * ((var_t1_dn12 * var_t2) + (var_t1 * var_t2_dn12))) * assign2890_e1982) - (assign2890_e1977 * ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)))) / (assign2890_e1982 * assign2890_e1982)), ((((var_nsubpp * ((var_t1_dn17 * var_t2) + (var_t1 * var_t2_dn17))) * assign2890_e1982) - (assign2890_e1977 * ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)))) / (assign2890_e1982 * assign2890_e1982)),)
    } else {
        (var_nsubps, var_nsubps_dn0, var_nsubps_dn2, var_nsubps_dn6, var_nsubps_dn7, var_nsubps_dn10, var_nsubps_dn11, var_nsubps_dn12, var_nsubps_dn17,)
    }
};
        var_nsubps = assign2890_e1985;
        var_nsubps_dn0 = assign2890_e1985_d_n0;
        var_nsubps_dn2 = assign2890_e1985_d_n2;
        var_nsubps_dn6 = assign2890_e1985_d_n6;
        var_nsubps_dn7 = assign2890_e1985_d_n7;
        var_nsubps_dn10 = assign2890_e1985_d_n10;
        var_nsubps_dn11 = assign2890_e1985_d_n11;
        var_nsubps_dn12 = assign2890_e1985_d_n12;
        var_nsubps_dn17 = assign2890_e1985_d_n17;
        var_nsubps_rv = 0.0;

        let (assign2900_e1990, assign2900_e1990_d_n0, assign2900_e1990_d_n2, assign2900_e1990_d_n6, assign2900_e1990_d_n7, assign2900_e1990_d_n10, assign2900_e1990_d_n11, assign2900_e1990_d_n12, assign2900_e1990_d_n17,) = {
    if (var_guard18 == 0.0) {
        (var_nsubpp, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nsubps, var_nsubps_dn0, var_nsubps_dn2, var_nsubps_dn6, var_nsubps_dn7, var_nsubps_dn10, var_nsubps_dn11, var_nsubps_dn12, var_nsubps_dn17,)
    }
};
        var_nsubps = assign2900_e1990;
        var_nsubps_dn0 = assign2900_e1990_d_n0;
        var_nsubps_dn2 = assign2900_e1990_d_n2;
        var_nsubps_dn6 = assign2900_e1990_d_n6;
        var_nsubps_dn7 = assign2900_e1990_d_n7;
        var_nsubps_dn10 = assign2900_e1990_d_n10;
        var_nsubps_dn11 = assign2900_e1990_d_n11;
        var_nsubps_dn12 = assign2900_e1990_d_n12;
        var_nsubps_dn17 = assign2900_e1990_d_n17;
        var_nsubps_rv = 0.0;

        let assign2910_e1995: f64 = (var_wg).powf(p.p200);
        let assign2910_e1996: f64 = (p.p199 / assign2910_e1995);
        let assign2910_e1997: f64 = (1.0 + assign2910_e1996);
        let assign2910_e2002: f64 = (var_lgle).powf(p.p203);
        let assign2910_e2003: f64 = (p.p202 / assign2910_e2002);
        let assign2910_e2004: f64 = (1.0 + assign2910_e2003);
        let assign2910_e2005: f64 = (assign2910_e1997 * assign2910_e2004);
        var_t2 = assign2910_e2005;
        var_t2_dn0 = 0.0;
        var_t2_dn2 = 0.0;
        var_t2_dn6 = 0.0;
        var_t2_dn7 = 0.0;
        var_t2_dn10 = 0.0;
        var_t2_dn11 = 0.0;
        var_t2_dn12 = 0.0;
        var_t2_dn17 = 0.0;
        var_t2_rv = 0.0;

        let assign2920_e2008: f64 = (var_mks_nsubcmax / var_mks_nsubs);
        var_t3 = assign2920_e2008;
        var_t3_dn0 = 0.0;
        var_t3_dn2 = 0.0;
        var_t3_dn6 = 0.0;
        var_t3_dn7 = 0.0;
        var_t3_dn10 = 0.0;
        var_t3_dn11 = 0.0;
        var_t3_dn12 = 0.0;
        var_t3_dn17 = 0.0;
        var_t3_rv = 0.0;

        let assign2930_e2011: f64 = (var_t3 - var_t2);
        let assign2930_e2013: f64 = (assign2930_e2011 - 0.01);
        var_tmf1 = assign2930_e2013;
        var_tmf1_dn0 = (var_t3_dn0 - var_t2_dn0);
        var_tmf1_dn2 = (var_t3_dn2 - var_t2_dn2);
        var_tmf1_dn6 = (var_t3_dn6 - var_t2_dn6);
        var_tmf1_dn7 = (var_t3_dn7 - var_t2_dn7);
        var_tmf1_dn10 = (var_t3_dn10 - var_t2_dn10);
        var_tmf1_dn11 = (var_t3_dn11 - var_t2_dn11);
        var_tmf1_dn12 = (var_t3_dn12 - var_t2_dn12);
        var_tmf1_dn17 = (var_t3_dn17 - var_t2_dn17);
        var_tmf1_rv = 0.0;

        let assign2940_e2016: f64 = (4.0 * var_t3);
        let assign2940_e2018: f64 = (assign2940_e2016 * 0.01);
        var_tmf2 = assign2940_e2018;
        var_tmf2_dn0 = ((4.0 * var_t3_dn0) * 0.01);
        var_tmf2_dn2 = ((4.0 * var_t3_dn2) * 0.01);
        var_tmf2_dn6 = ((4.0 * var_t3_dn6) * 0.01);
        var_tmf2_dn7 = ((4.0 * var_t3_dn7) * 0.01);
        var_tmf2_dn10 = ((4.0 * var_t3_dn10) * 0.01);
        var_tmf2_dn11 = ((4.0 * var_t3_dn11) * 0.01);
        var_tmf2_dn12 = ((4.0 * var_t3_dn12) * 0.01);
        var_tmf2_dn17 = ((4.0 * var_t3_dn17) * 0.01);
        var_tmf2_rv = 0.0;

        let (assign2950_e2025, assign2950_e2025_d_n0, assign2950_e2025_d_n2, assign2950_e2025_d_n6, assign2950_e2025_d_n7, assign2950_e2025_d_n10, assign2950_e2025_d_n11, assign2950_e2025_d_n12, assign2950_e2025_d_n17,) = {
    if (var_tmf2 > 0.0) {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    } else {
        let assign2950_e2024: f64 = (-var_tmf2);
        (assign2950_e2024, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
    }
};
        var_tmf2 = assign2950_e2025;
        var_tmf2_dn0 = assign2950_e2025_d_n0;
        var_tmf2_dn2 = assign2950_e2025_d_n2;
        var_tmf2_dn6 = assign2950_e2025_d_n6;
        var_tmf2_dn7 = assign2950_e2025_d_n7;
        var_tmf2_dn10 = assign2950_e2025_d_n10;
        var_tmf2_dn11 = assign2950_e2025_d_n11;
        var_tmf2_dn12 = assign2950_e2025_d_n12;
        var_tmf2_dn17 = assign2950_e2025_d_n17;
        var_tmf2_rv = 0.0;

        let assign2960_e2028: f64 = (var_tmf1 * var_tmf1);
        let assign2960_e2030: f64 = (assign2960_e2028 + var_tmf2);
        let assign2960_e2031: f64 = (assign2960_e2030).sqrt();
        var_tmf2 = assign2960_e2031;
        var_tmf2_dn0 = ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign2960_e2031));
        var_tmf2_dn2 = ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign2960_e2031));
        var_tmf2_dn6 = ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign2960_e2031));
        var_tmf2_dn7 = ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign2960_e2031));
        var_tmf2_dn10 = ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign2960_e2031));
        var_tmf2_dn11 = ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign2960_e2031));
        var_tmf2_dn12 = ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign2960_e2031));
        var_tmf2_dn17 = ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign2960_e2031));
        var_tmf2_rv = 0.0;

        let assign2970_e2036: f64 = (var_tmf1 + var_tmf2);
        let assign2970_e2037: f64 = (0.5 * assign2970_e2036);
        let assign2970_e2038: f64 = (var_t3 - assign2970_e2037);
        var_t1 = assign2970_e2038;
        var_t1_dn0 = (var_t3_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)));
        var_t1_dn2 = (var_t3_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)));
        var_t1_dn6 = (var_t3_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)));
        var_t1_dn7 = (var_t3_dn7 - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)));
        var_t1_dn10 = (var_t3_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)));
        var_t1_dn11 = (var_t3_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)));
        var_t1_dn12 = (var_t3_dn12 - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12)));
        var_t1_dn17 = (var_t3_dn17 - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17)));
        var_t1_rv = 0.0;

        let assign2980_e2041: f64 = (var_mks_nsubs * var_t1);
        var_uc_nsubs = assign2980_e2041;
        var_uc_nsubs_dn0 = (var_mks_nsubs * var_t1_dn0);
        var_uc_nsubs_dn2 = (var_mks_nsubs * var_t1_dn2);
        var_uc_nsubs_dn6 = (var_mks_nsubs * var_t1_dn6);
        var_uc_nsubs_dn7 = (var_mks_nsubs * var_t1_dn7);
        var_uc_nsubs_dn10 = (var_mks_nsubs * var_t1_dn10);
        var_uc_nsubs_dn11 = (var_mks_nsubs * var_t1_dn11);
        var_uc_nsubs_dn12 = (var_mks_nsubs * var_t1_dn12);
        var_uc_nsubs_dn17 = (var_mks_nsubs * var_t1_dn17);
        var_uc_nsubs_rv = 0.0;

        let assign2990_e2044: f64 = if var_lod_half > 0.0 { 1.0 } else { 0.0 };
        var_guard19 = assign2990_e2044;
        var_guard19_rv = 0.0;

        let (assign3000_e2052, assign3000_e2052_d_n0, assign3000_e2052_d_n2, assign3000_e2052_d_n6, assign3000_e2052_d_n7, assign3000_e2052_d_n10, assign3000_e2052_d_n11, assign3000_e2052_d_n12, assign3000_e2052_d_n17,) = {
    if (var_guard19 != 0.0) {
        let assign3000_e2049: f64 = (1.0 + p.p165);
        let assign3000_e2050: f64 = (1.0 / assign3000_e2049);
        (assign3000_e2050, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign3000_e2052;
        var_t1_dn0 = assign3000_e2052_d_n0;
        var_t1_dn2 = assign3000_e2052_d_n2;
        var_t1_dn6 = assign3000_e2052_d_n6;
        var_t1_dn7 = assign3000_e2052_d_n7;
        var_t1_dn10 = assign3000_e2052_d_n10;
        var_t1_dn11 = assign3000_e2052_d_n11;
        var_t1_dn12 = assign3000_e2052_d_n12;
        var_t1_dn17 = assign3000_e2052_d_n17;
        var_t1_rv = 0.0;

        let (assign3010_e2060, assign3010_e2060_d_n0, assign3010_e2060_d_n2, assign3010_e2060_d_n6, assign3010_e2060_d_n7, assign3010_e2060_d_n10, assign3010_e2060_d_n11, assign3010_e2060_d_n12, assign3010_e2060_d_n17,) = {
    if (var_guard19 != 0.0) {
        let assign3010_e2056: f64 = (p.p164 / var_lod_half);
        let assign3010_e2058: f64 = (assign3010_e2056).powf(p.p166);
        (assign3010_e2058, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign3010_e2056).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn0) / (var_lod_half * var_lod_half))))) } } else { (assign3010_e2058 * (p.p166 * ((-((p.p164 * var_lod_half_dn0) / (var_lod_half * var_lod_half))) / assign3010_e2056))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign3010_e2056).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn2) / (var_lod_half * var_lod_half))))) } } else { (assign3010_e2058 * (p.p166 * ((-((p.p164 * var_lod_half_dn2) / (var_lod_half * var_lod_half))) / assign3010_e2056))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign3010_e2056).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn6) / (var_lod_half * var_lod_half))))) } } else { (assign3010_e2058 * (p.p166 * ((-((p.p164 * var_lod_half_dn6) / (var_lod_half * var_lod_half))) / assign3010_e2056))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign3010_e2056).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn7) / (var_lod_half * var_lod_half))))) } } else { (assign3010_e2058 * (p.p166 * ((-((p.p164 * var_lod_half_dn7) / (var_lod_half * var_lod_half))) / assign3010_e2056))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign3010_e2056).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn10) / (var_lod_half * var_lod_half))))) } } else { (assign3010_e2058 * (p.p166 * ((-((p.p164 * var_lod_half_dn10) / (var_lod_half * var_lod_half))) / assign3010_e2056))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign3010_e2056).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn11) / (var_lod_half * var_lod_half))))) } } else { (assign3010_e2058 * (p.p166 * ((-((p.p164 * var_lod_half_dn11) / (var_lod_half * var_lod_half))) / assign3010_e2056))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign3010_e2056).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn12) / (var_lod_half * var_lod_half))))) } } else { (assign3010_e2058 * (p.p166 * ((-((p.p164 * var_lod_half_dn12) / (var_lod_half * var_lod_half))) / assign3010_e2056))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign3010_e2056).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn17) / (var_lod_half * var_lod_half))))) } } else { (assign3010_e2058 * (p.p166 * ((-((p.p164 * var_lod_half_dn17) / (var_lod_half * var_lod_half))) / assign3010_e2056))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign3010_e2060;
        var_t2_dn0 = assign3010_e2060_d_n0;
        var_t2_dn2 = assign3010_e2060_d_n2;
        var_t2_dn6 = assign3010_e2060_d_n6;
        var_t2_dn7 = assign3010_e2060_d_n7;
        var_t2_dn10 = assign3010_e2060_d_n10;
        var_t2_dn11 = assign3010_e2060_d_n11;
        var_t2_dn12 = assign3010_e2060_d_n12;
        var_t2_dn17 = assign3010_e2060_d_n17;
        var_t2_rv = 0.0;

        let (assign3020_e2068, assign3020_e2068_d_n0, assign3020_e2068_d_n2, assign3020_e2068_d_n6, assign3020_e2068_d_n7, assign3020_e2068_d_n10, assign3020_e2068_d_n11, assign3020_e2068_d_n12, assign3020_e2068_d_n17,) = {
    if (var_guard19 != 0.0) {
        let assign3020_e2064: f64 = (p.p164 / var_lod_half_ref);
        let assign3020_e2066: f64 = (assign3020_e2064).powf(p.p166);
        (assign3020_e2066, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign3020_e2068;
        var_t3_dn0 = assign3020_e2068_d_n0;
        var_t3_dn2 = assign3020_e2068_d_n2;
        var_t3_dn6 = assign3020_e2068_d_n6;
        var_t3_dn7 = assign3020_e2068_d_n7;
        var_t3_dn10 = assign3020_e2068_d_n10;
        var_t3_dn11 = assign3020_e2068_d_n11;
        var_t3_dn12 = assign3020_e2068_d_n12;
        var_t3_dn17 = assign3020_e2068_d_n17;
        var_t3_rv = 0.0;

        let (assign3030_e2084, assign3030_e2084_d_n0, assign3030_e2084_d_n2, assign3030_e2084_d_n6, assign3030_e2084_d_n7, assign3030_e2084_d_n10, assign3030_e2084_d_n11, assign3030_e2084_d_n12, assign3030_e2084_d_n17,) = {
    if (var_guard19 != 0.0) {
        let assign3030_e2074: f64 = (var_t1 * var_t2);
        let assign3030_e2075: f64 = (1.0 + assign3030_e2074);
        let assign3030_e2076: f64 = (var_uc_nsubs * assign3030_e2075);
        let assign3030_e2080: f64 = (var_t1 * var_t3);
        let assign3030_e2081: f64 = (1.0 + assign3030_e2080);
        let assign3030_e2082: f64 = (assign3030_e2076 / assign3030_e2081);
        (assign3030_e2082, (((((var_uc_nsubs_dn0 * assign3030_e2075) + (var_uc_nsubs * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0)))) * assign3030_e2081) - (assign3030_e2076 * ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)))) / (assign3030_e2081 * assign3030_e2081)), (((((var_uc_nsubs_dn2 * assign3030_e2075) + (var_uc_nsubs * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2)))) * assign3030_e2081) - (assign3030_e2076 * ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)))) / (assign3030_e2081 * assign3030_e2081)), (((((var_uc_nsubs_dn6 * assign3030_e2075) + (var_uc_nsubs * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * assign3030_e2081) - (assign3030_e2076 * ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)))) / (assign3030_e2081 * assign3030_e2081)), (((((var_uc_nsubs_dn7 * assign3030_e2075) + (var_uc_nsubs * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * assign3030_e2081) - (assign3030_e2076 * ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)))) / (assign3030_e2081 * assign3030_e2081)), (((((var_uc_nsubs_dn10 * assign3030_e2075) + (var_uc_nsubs * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10)))) * assign3030_e2081) - (assign3030_e2076 * ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)))) / (assign3030_e2081 * assign3030_e2081)), (((((var_uc_nsubs_dn11 * assign3030_e2075) + (var_uc_nsubs * ((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11)))) * assign3030_e2081) - (assign3030_e2076 * ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)))) / (assign3030_e2081 * assign3030_e2081)), (((((var_uc_nsubs_dn12 * assign3030_e2075) + (var_uc_nsubs * ((var_t1_dn12 * var_t2) + (var_t1 * var_t2_dn12)))) * assign3030_e2081) - (assign3030_e2076 * ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)))) / (assign3030_e2081 * assign3030_e2081)), (((((var_uc_nsubs_dn17 * assign3030_e2075) + (var_uc_nsubs * ((var_t1_dn17 * var_t2) + (var_t1 * var_t2_dn17)))) * assign3030_e2081) - (assign3030_e2076 * ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)))) / (assign3030_e2081 * assign3030_e2081)),)
    } else {
        (var_uc_nsubs, var_uc_nsubs_dn0, var_uc_nsubs_dn2, var_uc_nsubs_dn6, var_uc_nsubs_dn7, var_uc_nsubs_dn10, var_uc_nsubs_dn11, var_uc_nsubs_dn12, var_uc_nsubs_dn17,)
    }
};
        var_uc_nsubs = assign3030_e2084;
        var_uc_nsubs_dn0 = assign3030_e2084_d_n0;
        var_uc_nsubs_dn2 = assign3030_e2084_d_n2;
        var_uc_nsubs_dn6 = assign3030_e2084_d_n6;
        var_uc_nsubs_dn7 = assign3030_e2084_d_n7;
        var_uc_nsubs_dn10 = assign3030_e2084_d_n10;
        var_uc_nsubs_dn11 = assign3030_e2084_d_n11;
        var_uc_nsubs_dn12 = assign3030_e2084_d_n12;
        var_uc_nsubs_dn17 = assign3030_e2084_d_n17;
        var_uc_nsubs_rv = 0.0;

        let assign3040_e2091: f64 = if ((var_lgleff > p.p72) || (p.p72 <= 0.0)) { 1.0 } else { 0.0 };
        var_guard20 = assign3040_e2091;
        var_guard20_rv = 0.0;

        let (assign3050_e2105, assign3050_e2105_d_n0, assign3050_e2105_d_n2, assign3050_e2105_d_n6, assign3050_e2105_d_n7, assign3050_e2105_d_n10, assign3050_e2105_d_n11, assign3050_e2105_d_n12, assign3050_e2105_d_n17,) = {
    if (var_guard20 != 0.0) {
        let assign3050_e2096: f64 = (var_lgleff - p.p72);
        let assign3050_e2097: f64 = (var_uc_nsubs * assign3050_e2096);
        let assign3050_e2100: f64 = (var_nsubps * p.p72);
        let assign3050_e2101: f64 = (assign3050_e2097 + assign3050_e2100);
        let assign3050_e2103: f64 = (assign3050_e2101 / var_lgleff);
        (assign3050_e2103, (((var_uc_nsubs_dn0 * assign3050_e2096) + (var_nsubps_dn0 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn2 * assign3050_e2096) + (var_nsubps_dn2 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn6 * assign3050_e2096) + (var_nsubps_dn6 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn7 * assign3050_e2096) + (var_nsubps_dn7 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn10 * assign3050_e2096) + (var_nsubps_dn10 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn11 * assign3050_e2096) + (var_nsubps_dn11 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn12 * assign3050_e2096) + (var_nsubps_dn12 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn17 * assign3050_e2096) + (var_nsubps_dn17 * p.p72)) / var_lgleff),)
    } else {
        (var_nsub, var_nsub_dn0, var_nsub_dn2, var_nsub_dn6, var_nsub_dn7, var_nsub_dn10, var_nsub_dn11, var_nsub_dn12, var_nsub_dn17,)
    }
};
        var_nsub = assign3050_e2105;
        var_nsub_dn0 = assign3050_e2105_d_n0;
        var_nsub_dn2 = assign3050_e2105_d_n2;
        var_nsub_dn6 = assign3050_e2105_d_n6;
        var_nsub_dn7 = assign3050_e2105_d_n7;
        var_nsub_dn10 = assign3050_e2105_d_n10;
        var_nsub_dn11 = assign3050_e2105_d_n11;
        var_nsub_dn12 = assign3050_e2105_d_n12;
        var_nsub_dn17 = assign3050_e2105_d_n17;
        var_nsub_rv = 0.0;

        let (assign3060_e2120, assign3060_e2120_d_n0, assign3060_e2120_d_n2, assign3060_e2120_d_n6, assign3060_e2120_d_n7, assign3060_e2120_d_n10, assign3060_e2120_d_n11, assign3060_e2120_d_n12, assign3060_e2120_d_n17,) = {
    if (var_guard20 == 0.0) {
        let assign3060_e2111: f64 = (var_nsubps - var_uc_nsubs);
        let assign3060_e2114: f64 = (p.p72 - var_lgleff);
        let assign3060_e2115: f64 = (assign3060_e2111 * assign3060_e2114);
        let assign3060_e2117: f64 = (assign3060_e2115 / p.p72);
        let assign3060_e2118: f64 = (var_nsubps + assign3060_e2117);
        (assign3060_e2118, (var_nsubps_dn0 + (((var_nsubps_dn0 - var_uc_nsubs_dn0) * assign3060_e2114) / p.p72)), (var_nsubps_dn2 + (((var_nsubps_dn2 - var_uc_nsubs_dn2) * assign3060_e2114) / p.p72)), (var_nsubps_dn6 + (((var_nsubps_dn6 - var_uc_nsubs_dn6) * assign3060_e2114) / p.p72)), (var_nsubps_dn7 + (((var_nsubps_dn7 - var_uc_nsubs_dn7) * assign3060_e2114) / p.p72)), (var_nsubps_dn10 + (((var_nsubps_dn10 - var_uc_nsubs_dn10) * assign3060_e2114) / p.p72)), (var_nsubps_dn11 + (((var_nsubps_dn11 - var_uc_nsubs_dn11) * assign3060_e2114) / p.p72)), (var_nsubps_dn12 + (((var_nsubps_dn12 - var_uc_nsubs_dn12) * assign3060_e2114) / p.p72)), (var_nsubps_dn17 + (((var_nsubps_dn17 - var_uc_nsubs_dn17) * assign3060_e2114) / p.p72)),)
    } else {
        (var_nsub, var_nsub_dn0, var_nsub_dn2, var_nsub_dn6, var_nsub_dn7, var_nsub_dn10, var_nsub_dn11, var_nsub_dn12, var_nsub_dn17,)
    }
};
        var_nsub = assign3060_e2120;
        var_nsub_dn0 = assign3060_e2120_d_n0;
        var_nsub_dn2 = assign3060_e2120_d_n2;
        var_nsub_dn6 = assign3060_e2120_d_n6;
        var_nsub_dn7 = assign3060_e2120_d_n7;
        var_nsub_dn10 = assign3060_e2120_d_n10;
        var_nsub_dn11 = assign3060_e2120_d_n11;
        var_nsub_dn12 = assign3060_e2120_d_n12;
        var_nsub_dn17 = assign3060_e2120_d_n17;
        var_nsub_rv = 0.0;

        let assign3070_e2123: f64 = (1.6021918e-19 * var_nsub);
        var_q_nsub = assign3070_e2123;
        var_q_nsub_dn0 = (1.6021918e-19 * var_nsub_dn0);
        var_q_nsub_dn2 = (1.6021918e-19 * var_nsub_dn2);
        var_q_nsub_dn6 = (1.6021918e-19 * var_nsub_dn6);
        var_q_nsub_dn7 = (1.6021918e-19 * var_nsub_dn7);
        var_q_nsub_dn10 = (1.6021918e-19 * var_nsub_dn10);
        var_q_nsub_dn11 = (1.6021918e-19 * var_nsub_dn11);
        var_q_nsub_dn12 = (1.6021918e-19 * var_nsub_dn12);
        var_q_nsub_dn17 = (1.6021918e-19 * var_nsub_dn17);
        var_q_nsub_rv = 0.0;

        let assign3080_e2126: f64 = (var_q_nsub * 1.034943e-10);
        var_qnsub_esi = assign3080_e2126;
        var_qnsub_esi_dn0 = (var_q_nsub_dn0 * 1.034943e-10);
        var_qnsub_esi_dn2 = (var_q_nsub_dn2 * 1.034943e-10);
        var_qnsub_esi_dn6 = (var_q_nsub_dn6 * 1.034943e-10);
        var_qnsub_esi_dn7 = (var_q_nsub_dn7 * 1.034943e-10);
        var_qnsub_esi_dn10 = (var_q_nsub_dn10 * 1.034943e-10);
        var_qnsub_esi_dn11 = (var_q_nsub_dn11 * 1.034943e-10);
        var_qnsub_esi_dn12 = (var_q_nsub_dn12 * 1.034943e-10);
        var_qnsub_esi_dn17 = (var_q_nsub_dn17 * 1.034943e-10);
        var_qnsub_esi_rv = 0.0;

        let assign3090_e2129: f64 = (2.0 * var_qnsub_esi);
        var_qnsub_esi2 = assign3090_e2129;
        var_qnsub_esi2_dn0 = (2.0 * var_qnsub_esi_dn0);
        var_qnsub_esi2_dn2 = (2.0 * var_qnsub_esi_dn2);
        var_qnsub_esi2_dn6 = (2.0 * var_qnsub_esi_dn6);
        var_qnsub_esi2_dn7 = (2.0 * var_qnsub_esi_dn7);
        var_qnsub_esi2_dn10 = (2.0 * var_qnsub_esi_dn10);
        var_qnsub_esi2_dn11 = (2.0 * var_qnsub_esi_dn11);
        var_qnsub_esi2_dn12 = (2.0 * var_qnsub_esi_dn12);
        var_qnsub_esi2_dn17 = (2.0 * var_qnsub_esi_dn17);
        var_qnsub_esi2_rv = 0.0;

        let assign3100_e2133: f64 = (2.0 * p.p72);
        let assign3100_e2138: f64 = if ((var_lgleff <= assign3100_e2133) && (p.p72 > 0.0)) { 1.0 } else { 0.0 };
        var_guard21 = assign3100_e2138;
        var_guard21_rv = 0.0;

        let (assign3110_e2154, assign3110_e2154_d_n0, assign3110_e2154_d_n2, assign3110_e2154_d_n6, assign3110_e2154_d_n7, assign3110_e2154_d_n10, assign3110_e2154_d_n11, assign3110_e2154_d_n12, assign3110_e2154_d_n17,) = {
    if (var_guard21 != 0.0) {
        let assign3110_e2142: f64 = (2.0 * var_nsubps);
        let assign3110_e2145: f64 = (var_nsubps - var_uc_nsubs);
        let assign3110_e2147: f64 = (assign3110_e2145 * var_lgleff);
        let assign3110_e2149: f64 = (assign3110_e2147 / p.p72);
        let assign3110_e2150: f64 = (assign3110_e2142 - assign3110_e2149);
        let assign3110_e2152: f64 = (assign3110_e2150 - var_uc_nsubs);
        (assign3110_e2152, (((2.0 * var_nsubps_dn0) - (((var_nsubps_dn0 - var_uc_nsubs_dn0) * var_lgleff) / p.p72)) - var_uc_nsubs_dn0), (((2.0 * var_nsubps_dn2) - (((var_nsubps_dn2 - var_uc_nsubs_dn2) * var_lgleff) / p.p72)) - var_uc_nsubs_dn2), (((2.0 * var_nsubps_dn6) - (((var_nsubps_dn6 - var_uc_nsubs_dn6) * var_lgleff) / p.p72)) - var_uc_nsubs_dn6), (((2.0 * var_nsubps_dn7) - (((var_nsubps_dn7 - var_uc_nsubs_dn7) * var_lgleff) / p.p72)) - var_uc_nsubs_dn7), (((2.0 * var_nsubps_dn10) - (((var_nsubps_dn10 - var_uc_nsubs_dn10) * var_lgleff) / p.p72)) - var_uc_nsubs_dn10), (((2.0 * var_nsubps_dn11) - (((var_nsubps_dn11 - var_uc_nsubs_dn11) * var_lgleff) / p.p72)) - var_uc_nsubs_dn11), (((2.0 * var_nsubps_dn12) - (((var_nsubps_dn12 - var_uc_nsubs_dn12) * var_lgleff) / p.p72)) - var_uc_nsubs_dn12), (((2.0 * var_nsubps_dn17) - (((var_nsubps_dn17 - var_uc_nsubs_dn17) * var_lgleff) / p.p72)) - var_uc_nsubs_dn17),)
    } else {
        (var_nsubb0, var_nsubb0_dn0, var_nsubb0_dn2, var_nsubb0_dn6, var_nsubb0_dn7, var_nsubb0_dn10, var_nsubb0_dn11, var_nsubb0_dn12, var_nsubb0_dn17,)
    }
};
        var_nsubb0 = assign3110_e2154;
        var_nsubb0_dn0 = assign3110_e2154_d_n0;
        var_nsubb0_dn2 = assign3110_e2154_d_n2;
        var_nsubb0_dn6 = assign3110_e2154_d_n6;
        var_nsubb0_dn7 = assign3110_e2154_d_n7;
        var_nsubb0_dn10 = assign3110_e2154_d_n10;
        var_nsubb0_dn11 = assign3110_e2154_d_n11;
        var_nsubb0_dn12 = assign3110_e2154_d_n12;
        var_nsubb0_dn17 = assign3110_e2154_d_n17;
        var_nsubb0_rv = 0.0;

        let (assign3120_e2161, assign3120_e2161_d_n0, assign3120_e2161_d_n2, assign3120_e2161_d_n6, assign3120_e2161_d_n7, assign3120_e2161_d_n10, assign3120_e2161_d_n11, assign3120_e2161_d_n12, assign3120_e2161_d_n17,) = {
    if (var_guard21 != 0.0) {
        let assign3120_e2158: f64 = (var_nsubb0 / var_uc_nsubs);
        let assign3120_e2159: f64 = (assign3120_e2158).ln();
        (assign3120_e2159, ((((var_nsubb0_dn0 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)) / assign3120_e2158), ((((var_nsubb0_dn2 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)) / assign3120_e2158), ((((var_nsubb0_dn6 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)) / assign3120_e2158), ((((var_nsubb0_dn7 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn7)) / (var_uc_nsubs * var_uc_nsubs)) / assign3120_e2158), ((((var_nsubb0_dn10 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)) / assign3120_e2158), ((((var_nsubb0_dn11 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)) / assign3120_e2158), ((((var_nsubb0_dn12 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)) / assign3120_e2158), ((((var_nsubb0_dn17 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn17)) / (var_uc_nsubs * var_uc_nsubs)) / assign3120_e2158),)
    } else {
        (var_ptovr0, var_ptovr0_dn0, var_ptovr0_dn2, var_ptovr0_dn6, var_ptovr0_dn7, var_ptovr0_dn10, var_ptovr0_dn11, var_ptovr0_dn12, var_ptovr0_dn17,)
    }
};
        var_ptovr0 = assign3120_e2161;
        var_ptovr0_dn0 = assign3120_e2161_d_n0;
        var_ptovr0_dn2 = assign3120_e2161_d_n2;
        var_ptovr0_dn6 = assign3120_e2161_d_n6;
        var_ptovr0_dn7 = assign3120_e2161_d_n7;
        var_ptovr0_dn10 = assign3120_e2161_d_n10;
        var_ptovr0_dn11 = assign3120_e2161_d_n11;
        var_ptovr0_dn12 = assign3120_e2161_d_n12;
        var_ptovr0_dn17 = assign3120_e2161_d_n17;
        var_ptovr0_rv = 0.0;

        let (assign3130_e2166, assign3130_e2166_d_n0, assign3130_e2166_d_n2, assign3130_e2166_d_n6, assign3130_e2166_d_n7, assign3130_e2166_d_n10, assign3130_e2166_d_n11, assign3130_e2166_d_n12, assign3130_e2166_d_n17,) = {
    if (var_guard21 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ptovr0, var_ptovr0_dn0, var_ptovr0_dn2, var_ptovr0_dn6, var_ptovr0_dn7, var_ptovr0_dn10, var_ptovr0_dn11, var_ptovr0_dn12, var_ptovr0_dn17,)
    }
};
        var_ptovr0 = assign3130_e2166;
        var_ptovr0_dn0 = assign3130_e2166_d_n0;
        var_ptovr0_dn2 = assign3130_e2166_d_n2;
        var_ptovr0_dn6 = assign3130_e2166_d_n6;
        var_ptovr0_dn7 = assign3130_e2166_d_n7;
        var_ptovr0_dn10 = assign3130_e2166_d_n10;
        var_ptovr0_dn11 = assign3130_e2166_d_n11;
        var_ptovr0_dn12 = assign3130_e2166_d_n12;
        var_ptovr0_dn17 = assign3130_e2166_d_n17;
        var_ptovr0_rv = 0.0;

        let assign3140_e2169: f64 = (2.0 / 38.68283);
        let assign3140_e2173: f64 = (10400000000.0 / 1e-6);
        let assign3140_e2174: f64 = (var_nsub / assign3140_e2173);
        let assign3140_e2175: f64 = (assign3140_e2174).ln();
        let assign3140_e2176: f64 = (assign3140_e2169 * assign3140_e2175);
        var_pb20 = assign3140_e2176;
        var_pb20_dn0 = (assign3140_e2169 * ((var_nsub_dn0 / assign3140_e2173) / assign3140_e2174));
        var_pb20_dn2 = (assign3140_e2169 * ((var_nsub_dn2 / assign3140_e2173) / assign3140_e2174));
        var_pb20_dn6 = (assign3140_e2169 * ((var_nsub_dn6 / assign3140_e2173) / assign3140_e2174));
        var_pb20_dn7 = (assign3140_e2169 * ((var_nsub_dn7 / assign3140_e2173) / assign3140_e2174));
        var_pb20_dn10 = (assign3140_e2169 * ((var_nsub_dn10 / assign3140_e2173) / assign3140_e2174));
        var_pb20_dn11 = (assign3140_e2169 * ((var_nsub_dn11 / assign3140_e2173) / assign3140_e2174));
        var_pb20_dn12 = (assign3140_e2169 * ((var_nsub_dn12 / assign3140_e2173) / assign3140_e2174));
        var_pb20_dn17 = (assign3140_e2169 * ((var_nsub_dn17 / assign3140_e2173) / assign3140_e2174));
        var_pb20_rv = 0.0;

        let assign3150_e2179: f64 = (2.0 / 38.68283);
        let assign3150_e2183: f64 = (10400000000.0 / 1e-6);
        let assign3150_e2184: f64 = (var_uc_nsubs / assign3150_e2183);
        let assign3150_e2185: f64 = (assign3150_e2184).ln();
        let assign3150_e2186: f64 = (assign3150_e2179 * assign3150_e2185);
        var_pb2c = assign3150_e2186;
        var_pb2c_dn0 = (assign3150_e2179 * ((var_uc_nsubs_dn0 / assign3150_e2183) / assign3150_e2184));
        var_pb2c_dn2 = (assign3150_e2179 * ((var_uc_nsubs_dn2 / assign3150_e2183) / assign3150_e2184));
        var_pb2c_dn6 = (assign3150_e2179 * ((var_uc_nsubs_dn6 / assign3150_e2183) / assign3150_e2184));
        var_pb2c_dn7 = (assign3150_e2179 * ((var_uc_nsubs_dn7 / assign3150_e2183) / assign3150_e2184));
        var_pb2c_dn10 = (assign3150_e2179 * ((var_uc_nsubs_dn10 / assign3150_e2183) / assign3150_e2184));
        var_pb2c_dn11 = (assign3150_e2179 * ((var_uc_nsubs_dn11 / assign3150_e2183) / assign3150_e2184));
        var_pb2c_dn12 = (assign3150_e2179 * ((var_uc_nsubs_dn12 / assign3150_e2183) / assign3150_e2184));
        var_pb2c_dn17 = (assign3150_e2179 * ((var_uc_nsubs_dn17 / assign3150_e2183) / assign3150_e2184));
        var_pb2c_rv = 0.0;

        let assign3160_e2189: f64 = (2.0 * 1.034943e-10);
        let assign3160_e2191: f64 = (assign3160_e2189 / 1.6021918e-19);
        let assign3160_e2193: f64 = (assign3160_e2191 / var_nsub);
        let assign3160_e2194: f64 = (assign3160_e2193).sqrt();
        var_wdpl = assign3160_e2194;
        var_wdpl_dn0 = ((-((assign3160_e2191 * var_nsub_dn0) / (var_nsub * var_nsub))) / (2.0 * assign3160_e2194));
        var_wdpl_dn2 = ((-((assign3160_e2191 * var_nsub_dn2) / (var_nsub * var_nsub))) / (2.0 * assign3160_e2194));
        var_wdpl_dn6 = ((-((assign3160_e2191 * var_nsub_dn6) / (var_nsub * var_nsub))) / (2.0 * assign3160_e2194));
        var_wdpl_dn7 = ((-((assign3160_e2191 * var_nsub_dn7) / (var_nsub * var_nsub))) / (2.0 * assign3160_e2194));
        var_wdpl_dn10 = ((-((assign3160_e2191 * var_nsub_dn10) / (var_nsub * var_nsub))) / (2.0 * assign3160_e2194));
        var_wdpl_dn11 = ((-((assign3160_e2191 * var_nsub_dn11) / (var_nsub * var_nsub))) / (2.0 * assign3160_e2194));
        var_wdpl_dn12 = ((-((assign3160_e2191 * var_nsub_dn12) / (var_nsub * var_nsub))) / (2.0 * assign3160_e2194));
        var_wdpl_dn17 = ((-((assign3160_e2191 * var_nsub_dn17) / (var_nsub * var_nsub))) / (2.0 * assign3160_e2194));
        var_wdpl_rv = 0.0;

        let assign3170_e2199: f64 = (var_lgle).powf(p.p195);
        let assign3170_e2200: f64 = (p.p194 / assign3170_e2199);
        let assign3170_e2201: f64 = (1.0 + assign3170_e2200);
        let assign3170_e2206: f64 = (var_wl).powf(p.p197);
        let assign3170_e2207: f64 = (p.p196 / assign3170_e2206);
        let assign3170_e2208: f64 = (1.0 + assign3170_e2207);
        let assign3170_e2209: f64 = (assign3170_e2201 * assign3170_e2208);
        var_t1 = assign3170_e2209;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn17 = 0.0;
        var_t1_rv = 0.0;

        let assign3180_e2212: f64 = (var_t1 * var_t1);
        let assign3180_e2215: f64 = (4.0 * 0.001);
        let assign3180_e2217: f64 = (assign3180_e2215 * 0.001);
        let assign3180_e2218: f64 = (assign3180_e2212 + assign3180_e2217);
        let assign3180_e2219: f64 = (assign3180_e2218).sqrt();
        var_tmf1 = assign3180_e2219;
        var_tmf1_dn0 = (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) / (2.0 * assign3180_e2219));
        var_tmf1_dn2 = (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) / (2.0 * assign3180_e2219));
        var_tmf1_dn6 = (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign3180_e2219));
        var_tmf1_dn7 = (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign3180_e2219));
        var_tmf1_dn10 = (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) / (2.0 * assign3180_e2219));
        var_tmf1_dn11 = (((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) / (2.0 * assign3180_e2219));
        var_tmf1_dn12 = (((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) / (2.0 * assign3180_e2219));
        var_tmf1_dn17 = (((var_t1_dn17 * var_t1) + (var_t1 * var_t1_dn17)) / (2.0 * assign3180_e2219));
        var_tmf1_rv = 0.0;

        let assign3190_e2223: f64 = (var_t1 + var_tmf1);
        let assign3190_e2224: f64 = (0.5 * assign3190_e2223);
        let assign3190_e2227: f64 = (1e-10 * 0.001);
        let assign3190_e2228: f64 = (assign3190_e2224 + assign3190_e2227);
        var_vmax0 = assign3190_e2228;
        var_vmax0_dn0 = (0.5 * (var_t1_dn0 + var_tmf1_dn0));
        var_vmax0_dn2 = (0.5 * (var_t1_dn2 + var_tmf1_dn2));
        var_vmax0_dn6 = (0.5 * (var_t1_dn6 + var_tmf1_dn6));
        var_vmax0_dn7 = (0.5 * (var_t1_dn7 + var_tmf1_dn7));
        var_vmax0_dn10 = (0.5 * (var_t1_dn10 + var_tmf1_dn10));
        var_vmax0_dn11 = (0.5 * (var_t1_dn11 + var_tmf1_dn11));
        var_vmax0_dn12 = (0.5 * (var_t1_dn12 + var_tmf1_dn12));
        var_vmax0_dn17 = (0.5 * (var_t1_dn17 + var_tmf1_dn17));
        var_vmax0_rv = 0.0;

        let assign3200_e2231: f64 = if var_vmax0 < 0.0 { 1.0 } else { 0.0 };
        var_guard22 = assign3200_e2231;
        var_guard22_rv = 0.0;

        *var_guard19_slot = var_guard19;
        *var_guard19_rv_slot = var_guard19_rv;
        *var_guard20_slot = var_guard20;
        *var_guard20_rv_slot = var_guard20_rv;
        *var_guard21_slot = var_guard21;
        *var_guard21_rv_slot = var_guard21_rv;
        *var_guard22_slot = var_guard22;
        *var_guard22_rv_slot = var_guard22_rv;
        *var_nsub_slot = var_nsub;
        *var_nsub_dn0_slot = var_nsub_dn0;
        *var_nsub_dn10_slot = var_nsub_dn10;
        *var_nsub_dn11_slot = var_nsub_dn11;
        *var_nsub_dn12_slot = var_nsub_dn12;
        *var_nsub_dn17_slot = var_nsub_dn17;
        *var_nsub_dn2_slot = var_nsub_dn2;
        *var_nsub_dn6_slot = var_nsub_dn6;
        *var_nsub_dn7_slot = var_nsub_dn7;
        *var_nsub_rv_slot = var_nsub_rv;
        *var_nsubb0_slot = var_nsubb0;
        *var_nsubb0_dn0_slot = var_nsubb0_dn0;
        *var_nsubb0_dn10_slot = var_nsubb0_dn10;
        *var_nsubb0_dn11_slot = var_nsubb0_dn11;
        *var_nsubb0_dn12_slot = var_nsubb0_dn12;
        *var_nsubb0_dn17_slot = var_nsubb0_dn17;
        *var_nsubb0_dn2_slot = var_nsubb0_dn2;
        *var_nsubb0_dn6_slot = var_nsubb0_dn6;
        *var_nsubb0_dn7_slot = var_nsubb0_dn7;
        *var_nsubb0_rv_slot = var_nsubb0_rv;
        *var_nsubps_slot = var_nsubps;
        *var_nsubps_dn0_slot = var_nsubps_dn0;
        *var_nsubps_dn10_slot = var_nsubps_dn10;
        *var_nsubps_dn11_slot = var_nsubps_dn11;
        *var_nsubps_dn12_slot = var_nsubps_dn12;
        *var_nsubps_dn17_slot = var_nsubps_dn17;
        *var_nsubps_dn2_slot = var_nsubps_dn2;
        *var_nsubps_dn6_slot = var_nsubps_dn6;
        *var_nsubps_dn7_slot = var_nsubps_dn7;
        *var_nsubps_rv_slot = var_nsubps_rv;
        *var_pb20_slot = var_pb20;
        *var_pb20_dn0_slot = var_pb20_dn0;
        *var_pb20_dn10_slot = var_pb20_dn10;
        *var_pb20_dn11_slot = var_pb20_dn11;
        *var_pb20_dn12_slot = var_pb20_dn12;
        *var_pb20_dn17_slot = var_pb20_dn17;
        *var_pb20_dn2_slot = var_pb20_dn2;
        *var_pb20_dn6_slot = var_pb20_dn6;
        *var_pb20_dn7_slot = var_pb20_dn7;
        *var_pb20_rv_slot = var_pb20_rv;
        *var_pb2c_slot = var_pb2c;
        *var_pb2c_dn0_slot = var_pb2c_dn0;
        *var_pb2c_dn10_slot = var_pb2c_dn10;
        *var_pb2c_dn11_slot = var_pb2c_dn11;
        *var_pb2c_dn12_slot = var_pb2c_dn12;
        *var_pb2c_dn17_slot = var_pb2c_dn17;
        *var_pb2c_dn2_slot = var_pb2c_dn2;
        *var_pb2c_dn6_slot = var_pb2c_dn6;
        *var_pb2c_dn7_slot = var_pb2c_dn7;
        *var_pb2c_rv_slot = var_pb2c_rv;
        *var_ptovr0_slot = var_ptovr0;
        *var_ptovr0_dn0_slot = var_ptovr0_dn0;
        *var_ptovr0_dn10_slot = var_ptovr0_dn10;
        *var_ptovr0_dn11_slot = var_ptovr0_dn11;
        *var_ptovr0_dn12_slot = var_ptovr0_dn12;
        *var_ptovr0_dn17_slot = var_ptovr0_dn17;
        *var_ptovr0_dn2_slot = var_ptovr0_dn2;
        *var_ptovr0_dn6_slot = var_ptovr0_dn6;
        *var_ptovr0_dn7_slot = var_ptovr0_dn7;
        *var_ptovr0_rv_slot = var_ptovr0_rv;
        *var_q_nsub_slot = var_q_nsub;
        *var_q_nsub_dn0_slot = var_q_nsub_dn0;
        *var_q_nsub_dn10_slot = var_q_nsub_dn10;
        *var_q_nsub_dn11_slot = var_q_nsub_dn11;
        *var_q_nsub_dn12_slot = var_q_nsub_dn12;
        *var_q_nsub_dn17_slot = var_q_nsub_dn17;
        *var_q_nsub_dn2_slot = var_q_nsub_dn2;
        *var_q_nsub_dn6_slot = var_q_nsub_dn6;
        *var_q_nsub_dn7_slot = var_q_nsub_dn7;
        *var_q_nsub_rv_slot = var_q_nsub_rv;
        *var_qnsub_esi_slot = var_qnsub_esi;
        *var_qnsub_esi2_slot = var_qnsub_esi2;
        *var_qnsub_esi2_dn0_slot = var_qnsub_esi2_dn0;
        *var_qnsub_esi2_dn10_slot = var_qnsub_esi2_dn10;
        *var_qnsub_esi2_dn11_slot = var_qnsub_esi2_dn11;
        *var_qnsub_esi2_dn12_slot = var_qnsub_esi2_dn12;
        *var_qnsub_esi2_dn17_slot = var_qnsub_esi2_dn17;
        *var_qnsub_esi2_dn2_slot = var_qnsub_esi2_dn2;
        *var_qnsub_esi2_dn6_slot = var_qnsub_esi2_dn6;
        *var_qnsub_esi2_dn7_slot = var_qnsub_esi2_dn7;
        *var_qnsub_esi2_rv_slot = var_qnsub_esi2_rv;
        *var_qnsub_esi_dn0_slot = var_qnsub_esi_dn0;
        *var_qnsub_esi_dn10_slot = var_qnsub_esi_dn10;
        *var_qnsub_esi_dn11_slot = var_qnsub_esi_dn11;
        *var_qnsub_esi_dn12_slot = var_qnsub_esi_dn12;
        *var_qnsub_esi_dn17_slot = var_qnsub_esi_dn17;
        *var_qnsub_esi_dn2_slot = var_qnsub_esi_dn2;
        *var_qnsub_esi_dn6_slot = var_qnsub_esi_dn6;
        *var_qnsub_esi_dn7_slot = var_qnsub_esi_dn7;
        *var_qnsub_esi_rv_slot = var_qnsub_esi_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_rv_slot = var_t3_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_uc_nsubs_slot = var_uc_nsubs;
        *var_uc_nsubs_dn0_slot = var_uc_nsubs_dn0;
        *var_uc_nsubs_dn10_slot = var_uc_nsubs_dn10;
        *var_uc_nsubs_dn11_slot = var_uc_nsubs_dn11;
        *var_uc_nsubs_dn12_slot = var_uc_nsubs_dn12;
        *var_uc_nsubs_dn17_slot = var_uc_nsubs_dn17;
        *var_uc_nsubs_dn2_slot = var_uc_nsubs_dn2;
        *var_uc_nsubs_dn6_slot = var_uc_nsubs_dn6;
        *var_uc_nsubs_dn7_slot = var_uc_nsubs_dn7;
        *var_uc_nsubs_rv_slot = var_uc_nsubs_rv;
        *var_vmax0_slot = var_vmax0;
        *var_vmax0_dn0_slot = var_vmax0_dn0;
        *var_vmax0_dn10_slot = var_vmax0_dn10;
        *var_vmax0_dn11_slot = var_vmax0_dn11;
        *var_vmax0_dn12_slot = var_vmax0_dn12;
        *var_vmax0_dn17_slot = var_vmax0_dn17;
        *var_vmax0_dn2_slot = var_vmax0_dn2;
        *var_vmax0_dn6_slot = var_vmax0_dn6;
        *var_vmax0_dn7_slot = var_vmax0_dn7;
        *var_vmax0_rv_slot = var_vmax0_rv;
        *var_wdpl_slot = var_wdpl;
        *var_wdpl_dn0_slot = var_wdpl_dn0;
        *var_wdpl_dn10_slot = var_wdpl_dn10;
        *var_wdpl_dn11_slot = var_wdpl_dn11;
        *var_wdpl_dn12_slot = var_wdpl_dn12;
        *var_wdpl_dn17_slot = var_wdpl_dn17;
        *var_wdpl_dn2_slot = var_wdpl_dn2;
        *var_wdpl_dn6_slot = var_wdpl_dn6;
        *var_wdpl_dn7_slot = var_wdpl_dn7;
        *var_wdpl_rv_slot = var_wdpl_rv;
    }

    pub(super) fn stamp_reactive_block_6(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_abtn_given: f64,
        var_abtp_given: f64,
        var_cbtbn_given: f64,
        var_cbtbp_given: f64,
        var_flg_nqs: f64,
        var_guard22: f64,
        var_lgate: f64,
        var_pdbcp_given: f64,
        var_psbcp_given: f64,
        var_weff_nf: f64,
        var_weffcv_nf: f64,
        var_area_bt_n_slot: &mut f64,
        var_area_bt_n_rv_slot: &mut f64,
        var_area_bt_p_slot: &mut f64,
        var_area_bt_p_rv_slot: &mut f64,
        var_cbtn_slot: &mut f64,
        var_cbtn_rv_slot: &mut f64,
        var_cbtp_slot: &mut f64,
        var_cbtp_rv_slot: &mut f64,
        var_guard25_slot: &mut f64,
        var_guard25_rv_slot: &mut f64,
        var_guard30_slot: &mut f64,
        var_guard30_rv_slot: &mut f64,
        var_guard31_slot: &mut f64,
        var_guard31_rv_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_guard32_rv_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard33_rv_slot: &mut f64,
        var_guard34_slot: &mut f64,
        var_guard34_rv_slot: &mut f64,
        var_guard35_slot: &mut f64,
        var_guard35_rv_slot: &mut f64,
        var_peri_hhi_slot: &mut f64,
        var_peri_hhi_rv_slot: &mut f64,
        var_qb_nqs_slot: &mut f64,
        var_qb_nqs_dn13_slot: &mut f64,
        var_qb_nqs_rv_slot: &mut f64,
        var_qi_nqs_slot: &mut f64,
        var_qi_nqs_dn18_slot: &mut f64,
        var_qi_nqs_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_uc_pdbcp_slot: &mut f64,
        var_uc_pdbcp_rv_slot: &mut f64,
        var_uc_psbcp_slot: &mut f64,
        var_uc_psbcp_rv_slot: &mut f64,
        var_vbcd_slot: &mut f64,
        var_vbcd_dn12_slot: &mut f64,
        var_vbcd_dn6_slot: &mut f64,
        var_vbcd_rv_slot: &mut f64,
        var_vbcs_slot: &mut f64,
        var_vbcs_dn12_slot: &mut f64,
        var_vbcs_dn7_slot: &mut f64,
        var_vbcs_rv_slot: &mut f64,
        var_vbsi_slot: &mut f64,
        var_vbsi_dn12_slot: &mut f64,
        var_vbsi_dn7_slot: &mut f64,
        var_vbsi_rv_slot: &mut f64,
        var_vdsi_slot: &mut f64,
        var_vdsi_dn6_slot: &mut f64,
        var_vdsi_dn7_slot: &mut f64,
        var_vdsi_rv_slot: &mut f64,
        var_vgsi_slot: &mut f64,
        var_vgsi_dn11_slot: &mut f64,
        var_vgsi_dn7_slot: &mut f64,
        var_vgsi_rv_slot: &mut f64,
        var_vmax0_slot: &mut f64,
        var_vmax0_dn0_slot: &mut f64,
        var_vmax0_dn10_slot: &mut f64,
        var_vmax0_dn11_slot: &mut f64,
        var_vmax0_dn12_slot: &mut f64,
        var_vmax0_dn17_slot: &mut f64,
        var_vmax0_dn2_slot: &mut f64,
        var_vmax0_dn6_slot: &mut f64,
        var_vmax0_dn7_slot: &mut f64,
        var_vmax0_rv_slot: &mut f64,
        var_w_diod_slot: &mut f64,
        var_w_diod_rv_slot: &mut f64,
        var_w_diodcv_slot: &mut f64,
        var_w_diodcv_rv_slot: &mut f64,
        var_w_dios_slot: &mut f64,
        var_w_dios_rv_slot: &mut f64,
        var_w_dioscv_slot: &mut f64,
        var_w_dioscv_rv_slot: &mut f64,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let mut var_area_bt_n: f64 = *var_area_bt_n_slot;
        let mut var_area_bt_n_rv: f64 = *var_area_bt_n_rv_slot;
        let mut var_area_bt_p: f64 = *var_area_bt_p_slot;
        let mut var_area_bt_p_rv: f64 = *var_area_bt_p_rv_slot;
        let mut var_cbtn: f64 = *var_cbtn_slot;
        let mut var_cbtn_rv: f64 = *var_cbtn_rv_slot;
        let mut var_cbtp: f64 = *var_cbtp_slot;
        let mut var_cbtp_rv: f64 = *var_cbtp_rv_slot;
        let mut var_guard25: f64 = *var_guard25_slot;
        let mut var_guard25_rv: f64 = *var_guard25_rv_slot;
        let mut var_guard30: f64 = *var_guard30_slot;
        let mut var_guard30_rv: f64 = *var_guard30_rv_slot;
        let mut var_guard31: f64 = *var_guard31_slot;
        let mut var_guard31_rv: f64 = *var_guard31_rv_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_guard32_rv: f64 = *var_guard32_rv_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard33_rv: f64 = *var_guard33_rv_slot;
        let mut var_guard34: f64 = *var_guard34_slot;
        let mut var_guard34_rv: f64 = *var_guard34_rv_slot;
        let mut var_guard35: f64 = *var_guard35_slot;
        let mut var_guard35_rv: f64 = *var_guard35_rv_slot;
        let mut var_peri_hhi: f64 = *var_peri_hhi_slot;
        let mut var_peri_hhi_rv: f64 = *var_peri_hhi_rv_slot;
        let mut var_qb_nqs: f64 = *var_qb_nqs_slot;
        let mut var_qb_nqs_dn13: f64 = *var_qb_nqs_dn13_slot;
        let mut var_qb_nqs_rv: f64 = *var_qb_nqs_rv_slot;
        let mut var_qi_nqs: f64 = *var_qi_nqs_slot;
        let mut var_qi_nqs_dn18: f64 = *var_qi_nqs_dn18_slot;
        let mut var_qi_nqs_rv: f64 = *var_qi_nqs_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_uc_pdbcp: f64 = *var_uc_pdbcp_slot;
        let mut var_uc_pdbcp_rv: f64 = *var_uc_pdbcp_rv_slot;
        let mut var_uc_psbcp: f64 = *var_uc_psbcp_slot;
        let mut var_uc_psbcp_rv: f64 = *var_uc_psbcp_rv_slot;
        let mut var_vbcd: f64 = *var_vbcd_slot;
        let mut var_vbcd_dn12: f64 = *var_vbcd_dn12_slot;
        let mut var_vbcd_dn6: f64 = *var_vbcd_dn6_slot;
        let mut var_vbcd_rv: f64 = *var_vbcd_rv_slot;
        let mut var_vbcs: f64 = *var_vbcs_slot;
        let mut var_vbcs_dn12: f64 = *var_vbcs_dn12_slot;
        let mut var_vbcs_dn7: f64 = *var_vbcs_dn7_slot;
        let mut var_vbcs_rv: f64 = *var_vbcs_rv_slot;
        let mut var_vbsi: f64 = *var_vbsi_slot;
        let mut var_vbsi_dn12: f64 = *var_vbsi_dn12_slot;
        let mut var_vbsi_dn7: f64 = *var_vbsi_dn7_slot;
        let mut var_vbsi_rv: f64 = *var_vbsi_rv_slot;
        let mut var_vdsi: f64 = *var_vdsi_slot;
        let mut var_vdsi_dn6: f64 = *var_vdsi_dn6_slot;
        let mut var_vdsi_dn7: f64 = *var_vdsi_dn7_slot;
        let mut var_vdsi_rv: f64 = *var_vdsi_rv_slot;
        let mut var_vgsi: f64 = *var_vgsi_slot;
        let mut var_vgsi_dn11: f64 = *var_vgsi_dn11_slot;
        let mut var_vgsi_dn7: f64 = *var_vgsi_dn7_slot;
        let mut var_vgsi_rv: f64 = *var_vgsi_rv_slot;
        let mut var_vmax0: f64 = *var_vmax0_slot;
        let mut var_vmax0_dn0: f64 = *var_vmax0_dn0_slot;
        let mut var_vmax0_dn10: f64 = *var_vmax0_dn10_slot;
        let mut var_vmax0_dn11: f64 = *var_vmax0_dn11_slot;
        let mut var_vmax0_dn12: f64 = *var_vmax0_dn12_slot;
        let mut var_vmax0_dn17: f64 = *var_vmax0_dn17_slot;
        let mut var_vmax0_dn2: f64 = *var_vmax0_dn2_slot;
        let mut var_vmax0_dn6: f64 = *var_vmax0_dn6_slot;
        let mut var_vmax0_dn7: f64 = *var_vmax0_dn7_slot;
        let mut var_vmax0_rv: f64 = *var_vmax0_rv_slot;
        let mut var_w_diod: f64 = *var_w_diod_slot;
        let mut var_w_diod_rv: f64 = *var_w_diod_rv_slot;
        let mut var_w_diodcv: f64 = *var_w_diodcv_slot;
        let mut var_w_diodcv_rv: f64 = *var_w_diodcv_rv_slot;
        let mut var_w_dios: f64 = *var_w_dios_slot;
        let mut var_w_dios_rv: f64 = *var_w_dios_rv_slot;
        let mut var_w_dioscv: f64 = *var_w_dioscv_slot;
        let mut var_w_dioscv_rv: f64 = *var_w_dioscv_rv_slot;

        let (assign3210_e2235, assign3210_e2235_d_n0, assign3210_e2235_d_n2, assign3210_e2235_d_n6, assign3210_e2235_d_n7, assign3210_e2235_d_n10, assign3210_e2235_d_n11, assign3210_e2235_d_n12, assign3210_e2235_d_n17,) = {
    if (var_guard22 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vmax0, var_vmax0_dn0, var_vmax0_dn2, var_vmax0_dn6, var_vmax0_dn7, var_vmax0_dn10, var_vmax0_dn11, var_vmax0_dn12, var_vmax0_dn17,)
    }
};
        var_vmax0 = assign3210_e2235;
        var_vmax0_dn0 = assign3210_e2235_d_n0;
        var_vmax0_dn2 = assign3210_e2235_d_n2;
        var_vmax0_dn6 = assign3210_e2235_d_n6;
        var_vmax0_dn7 = assign3210_e2235_d_n7;
        var_vmax0_dn10 = assign3210_e2235_d_n10;
        var_vmax0_dn11 = assign3210_e2235_d_n11;
        var_vmax0_dn12 = assign3210_e2235_d_n12;
        var_vmax0_dn17 = assign3210_e2235_d_n17;
        var_vmax0_rv = 0.0;

        let assign3270_e2268: f64 = if p.p261 == 1.0 { 1.0 } else { 0.0 };
        var_guard25 = assign3270_e2268;
        var_guard25_rv = 0.0;

        let (assign3280_e2276, assign3280_e2276_d_n0, assign3280_e2276_d_n2, assign3280_e2276_d_n6, assign3280_e2276_d_n7, assign3280_e2276_d_n10, assign3280_e2276_d_n11, assign3280_e2276_d_n12, assign3280_e2276_d_n17,) = {
    if (var_guard25 != 0.0) {
        let assign3280_e2272: f64 = (p.p289 * var_weff_nf);
        let assign3280_e2274: f64 = (assign3280_e2272 + p.p288);
        (assign3280_e2274, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn6, var_t0_dn7, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn17,)
    }
};
        var_t0 = assign3280_e2276;
        var_t0_dn0 = assign3280_e2276_d_n0;
        var_t0_dn2 = assign3280_e2276_d_n2;
        var_t0_dn6 = assign3280_e2276_d_n6;
        var_t0_dn7 = assign3280_e2276_d_n7;
        var_t0_dn10 = assign3280_e2276_d_n10;
        var_t0_dn11 = assign3280_e2276_d_n11;
        var_t0_dn12 = assign3280_e2276_d_n12;
        var_t0_dn17 = assign3280_e2276_d_n17;
        var_t0_rv = 0.0;

        let assign3420_e2352: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard30 = assign3420_e2352;
        var_guard30_rv = 0.0;

        let (assign3430_e2365,) = {
    if ((var_guard30 != 0.0) && (p.p24 != 0.0)) {
        let (assign3430_e2363,) = {
            if (var_abtp_given != 0.0) {
                (p.p23,)
            } else {
                let assign3430_e2360: f64 = (p.p20 * p.p9);
                let assign3430_e2362: f64 = (assign3430_e2360 * p.p19);
                (assign3430_e2362,)
            }
        };
        (assign3430_e2363,)
    } else {
        (var_area_bt_p,)
    }
};
        var_area_bt_p = assign3430_e2365;
        var_area_bt_p_rv = 0.0;

        let (assign3440_e2378,) = {
    if ((var_guard30 != 0.0) && (p.p24 != 0.0)) {
        let (assign3440_e2376,) = {
            if (var_abtn_given != 0.0) {
                (p.p22,)
            } else {
                let assign3440_e2373: f64 = (p.p21 * p.p9);
                let assign3440_e2375: f64 = (assign3440_e2373 * p.p19);
                (assign3440_e2375,)
            }
        };
        (assign3440_e2376,)
    } else {
        (var_area_bt_n,)
    }
};
        var_area_bt_n = assign3440_e2378;
        var_area_bt_n_rv = 0.0;

        let (assign3450_e2384,) = {
    if ((var_guard30 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3450_e2384;
        var_cbtp_rv = 0.0;

        let (assign3460_e2390,) = {
    if ((var_guard30 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (var_cbtn,)
    }
};
        var_cbtn = assign3460_e2390;
        var_cbtn_rv = 0.0;

        let assign3470_e2395: f64 = if ((var_area_bt_p > 0.0) && (var_cbtbp_given != 0.0)) { 1.0 } else { 0.0 };
        var_guard31 = assign3470_e2395;
        var_guard31_rv = 0.0;

        let (assign3480_e2406,) = {
    if (((var_guard30 != 0.0) && (p.p24 != 0.0)) && (var_guard31 != 0.0)) {
        let assign3480_e2402: f64 = (-var_area_bt_p);
        let assign3480_e2404: f64 = (assign3480_e2402 * p.p294);
        (assign3480_e2404,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3480_e2406;
        var_cbtp_rv = 0.0;

        let (assign3490_e2415,) = {
    if (((var_guard30 != 0.0) && (p.p24 != 0.0)) && (var_guard31 == 0.0)) {
        (0.0,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3490_e2415;
        var_cbtp_rv = 0.0;

        let assign3500_e2420: f64 = if ((var_area_bt_n > 0.0) && (var_cbtbn_given != 0.0)) { 1.0 } else { 0.0 };
        var_guard32 = assign3500_e2420;
        var_guard32_rv = 0.0;

        let (assign3510_e2431,) = {
    if (((var_guard30 != 0.0) && (p.p24 != 0.0)) && (var_guard32 != 0.0)) {
        let assign3510_e2427: f64 = (-var_area_bt_n);
        let assign3510_e2429: f64 = (assign3510_e2427 * p.p293);
        (assign3510_e2429,)
    } else {
        (var_cbtn,)
    }
};
        var_cbtn = assign3510_e2431;
        var_cbtn_rv = 0.0;

        let (assign3520_e2439,) = {
    if (((var_guard30 != 0.0) && (p.p24 != 0.0)) && (var_guard32 != 0.0)) {
        (0.0,)
    } else {
        (var_area_bt_n,)
    }
};
        var_area_bt_n = assign3520_e2439;
        var_area_bt_n_rv = 0.0;

        let (assign3530_e2446,) = {
    if ((var_guard30 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (var_area_bt_n,)
    }
};
        var_area_bt_n = assign3530_e2446;
        var_area_bt_n_rv = 0.0;

        let (assign3540_e2453,) = {
    if ((var_guard30 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (var_cbtn,)
    }
};
        var_cbtn = assign3540_e2453;
        var_cbtn_rv = 0.0;

        let (assign3550_e2460,) = {
    if ((var_guard30 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (var_area_bt_p,)
    }
};
        var_area_bt_p = assign3550_e2460;
        var_area_bt_p_rv = 0.0;

        let (assign3560_e2467,) = {
    if ((var_guard30 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3560_e2467;
        var_cbtp_rv = 0.0;

        let (assign3570_e2480,) = {
    if (var_guard30 != 0.0) {
        let (assign3570_e2478,) = {
            if (p.p19 > var_lgate) {
                let assign3570_e2475: f64 = (p.p19 - var_lgate);
                let assign3570_e2476: f64 = (0.5 * assign3570_e2475);
                (assign3570_e2476,)
            } else {
                (0.0,)
            }
        };
        (assign3570_e2478,)
    } else {
        (var_peri_hhi,)
    }
};
        var_peri_hhi = assign3570_e2480;
        var_peri_hhi_rv = 0.0;

        let assign3580_e2483: f64 = if var_pdbcp_given == 0.0 { 1.0 } else { 0.0 };
        var_guard33 = assign3580_e2483;
        var_guard33_rv = 0.0;

        let (assign3590_e2489,) = {
    if ((var_guard30 != 0.0) && (var_guard33 != 0.0)) {
        (var_peri_hhi,)
    } else {
        (var_uc_pdbcp,)
    }
};
        var_uc_pdbcp = assign3590_e2489;
        var_uc_pdbcp_rv = 0.0;

        let assign3600_e2492: f64 = if var_psbcp_given == 0.0 { 1.0 } else { 0.0 };
        var_guard34 = assign3600_e2492;
        var_guard34_rv = 0.0;

        let (assign3610_e2498,) = {
    if ((var_guard30 != 0.0) && (var_guard34 != 0.0)) {
        (var_peri_hhi,)
    } else {
        (var_uc_psbcp,)
    }
};
        var_uc_psbcp = assign3610_e2498;
        var_uc_psbcp_rv = 0.0;

        let (assign3620_e2506,) = {
    if (var_guard30 != 0.0) {
        let assign3620_e2503: f64 = (p.p9 * var_uc_pdbcp);
        let assign3620_e2504: f64 = (var_weff_nf + assign3620_e2503);
        (assign3620_e2504,)
    } else {
        (var_w_diod,)
    }
};
        var_w_diod = assign3620_e2506;
        var_w_diod_rv = 0.0;

        let (assign3630_e2514,) = {
    if (var_guard30 != 0.0) {
        let assign3630_e2511: f64 = (p.p9 * var_uc_psbcp);
        let assign3630_e2512: f64 = (var_weff_nf + assign3630_e2511);
        (assign3630_e2512,)
    } else {
        (var_w_dios,)
    }
};
        var_w_dios = assign3630_e2514;
        var_w_dios_rv = 0.0;

        let (assign3640_e2522,) = {
    if (var_guard30 != 0.0) {
        let assign3640_e2519: f64 = (p.p9 * var_uc_pdbcp);
        let assign3640_e2520: f64 = (var_weffcv_nf + assign3640_e2519);
        (assign3640_e2520,)
    } else {
        (var_w_diodcv,)
    }
};
        var_w_diodcv = assign3640_e2522;
        var_w_diodcv_rv = 0.0;

        let (assign3650_e2530,) = {
    if (var_guard30 != 0.0) {
        let assign3650_e2527: f64 = (p.p9 * var_uc_psbcp);
        let assign3650_e2528: f64 = (var_weffcv_nf + assign3650_e2527);
        (assign3650_e2528,)
    } else {
        (var_w_dioscv,)
    }
};
        var_w_dioscv = assign3650_e2530;
        var_w_dioscv_rv = 0.0;

        let (assign3660_e2535,) = {
    if (var_guard30 == 0.0) {
        (0.0,)
    } else {
        (var_area_bt_n,)
    }
};
        var_area_bt_n = assign3660_e2535;
        var_area_bt_n_rv = 0.0;

        let (assign3670_e2540,) = {
    if (var_guard30 == 0.0) {
        (0.0,)
    } else {
        (var_cbtn,)
    }
};
        var_cbtn = assign3670_e2540;
        var_cbtn_rv = 0.0;

        let (assign3680_e2545,) = {
    if (var_guard30 == 0.0) {
        (0.0,)
    } else {
        (var_area_bt_p,)
    }
};
        var_area_bt_p = assign3680_e2545;
        var_area_bt_p_rv = 0.0;

        let (assign3690_e2550,) = {
    if (var_guard30 == 0.0) {
        (0.0,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3690_e2550;
        var_cbtp_rv = 0.0;

        let (assign3700_e2555,) = {
    if (var_guard30 == 0.0) {
        (0.0,)
    } else {
        (var_w_diod,)
    }
};
        var_w_diod = assign3700_e2555;
        var_w_diod_rv = 0.0;

        let (assign3710_e2560,) = {
    if (var_guard30 == 0.0) {
        (0.0,)
    } else {
        (var_w_dios,)
    }
};
        var_w_dios = assign3710_e2560;
        var_w_dios_rv = 0.0;

        let (assign3720_e2565,) = {
    if (var_guard30 == 0.0) {
        (0.0,)
    } else {
        (var_w_diodcv,)
    }
};
        var_w_diodcv = assign3720_e2565;
        var_w_diodcv_rv = 0.0;

        let (assign3730_e2570,) = {
    if (var_guard30 == 0.0) {
        (0.0,)
    } else {
        (var_w_dioscv,)
    }
};
        var_w_dioscv = assign3730_e2570;
        var_w_dioscv_rv = 0.0;

        let assign3740_e2573: f64 = (p.p50 * (nv6 - nv7));
        var_vdsi = assign3740_e2573;
        var_vdsi_dn6 = p.p50;
        var_vdsi_dn7 = (-p.p50);
        var_vdsi_rv = 0.0;

        let assign3750_e2576: f64 = (p.p50 * (nv11 - nv7));
        var_vgsi = assign3750_e2576;
        var_vgsi_dn7 = (-p.p50);
        var_vgsi_dn11 = p.p50;
        var_vgsi_rv = 0.0;

        let assign3760_e2579: f64 = (p.p50 * (nv12 - nv7));
        var_vbsi = assign3760_e2579;
        var_vbsi_dn7 = (-p.p50);
        var_vbsi_dn12 = p.p50;
        var_vbsi_rv = 0.0;

        let assign3800_e2591: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard35 = assign3800_e2591;
        var_guard35_rv = 0.0;

        let (assign3810_e2597, assign3810_e2597_d_n6, assign3810_e2597_d_n12,) = {
    if (var_guard35 != 0.0) {
        let assign3810_e2595: f64 = (p.p50 * (nv12 - nv6));
        (assign3810_e2595, (-p.p50), p.p50,)
    } else {
        (var_vbcd, var_vbcd_dn6, var_vbcd_dn12,)
    }
};
        var_vbcd = assign3810_e2597;
        var_vbcd_dn6 = assign3810_e2597_d_n6;
        var_vbcd_dn12 = assign3810_e2597_d_n12;
        var_vbcd_rv = 0.0;

        let (assign3820_e2603, assign3820_e2603_d_n7, assign3820_e2603_d_n12,) = {
    if (var_guard35 != 0.0) {
        let assign3820_e2601: f64 = (p.p50 * (nv12 - nv7));
        (assign3820_e2601, (-p.p50), p.p50,)
    } else {
        (var_vbcs, var_vbcs_dn7, var_vbcs_dn12,)
    }
};
        var_vbcs = assign3820_e2603;
        var_vbcs_dn7 = assign3820_e2603_d_n7;
        var_vbcs_dn12 = assign3820_e2603_d_n12;
        var_vbcs_rv = 0.0;

        let (assign3830_e2613, assign3830_e2613_d_n18,) = {
    if ((var_guard35 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign3830_e2609: f64 = (1e-9 / 0.0001);
        let assign3830_e2611: f64 = (assign3830_e2609 * (nv18 - 0.0));
        (assign3830_e2611, assign3830_e2609,)
    } else {
        (var_qi_nqs, var_qi_nqs_dn18,)
    }
};
        var_qi_nqs = assign3830_e2613;
        var_qi_nqs_dn18 = assign3830_e2613_d_n18;
        var_qi_nqs_rv = 0.0;

        let (assign3840_e2623, assign3840_e2623_d_n13,) = {
    if ((var_guard35 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign3840_e2619: f64 = (1e-9 / 0.0001);
        let assign3840_e2621: f64 = (assign3840_e2619 * (nv13 - 0.0));
        (assign3840_e2621, assign3840_e2619,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign3840_e2623;
        var_qb_nqs_dn13 = assign3840_e2623_d_n13;
        var_qb_nqs_rv = 0.0;

        let (assign3850_e2630, assign3850_e2630_d_n18,) = {
    if ((var_guard35 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qi_nqs, var_qi_nqs_dn18,)
    }
};
        var_qi_nqs = assign3850_e2630;
        var_qi_nqs_dn18 = assign3850_e2630_d_n18;
        var_qi_nqs_rv = 0.0;

        let (assign3860_e2637, assign3860_e2637_d_n13,) = {
    if ((var_guard35 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign3860_e2637;
        var_qb_nqs_dn13 = assign3860_e2637_d_n13;
        var_qb_nqs_rv = 0.0;

        let (assign3870_e2642, assign3870_e2642_d_n6, assign3870_e2642_d_n12,) = {
    if (var_guard35 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_vbcd, var_vbcd_dn6, var_vbcd_dn12,)
    }
};
        var_vbcd = assign3870_e2642;
        var_vbcd_dn6 = assign3870_e2642_d_n6;
        var_vbcd_dn12 = assign3870_e2642_d_n12;
        var_vbcd_rv = 0.0;

        let (assign3880_e2647, assign3880_e2647_d_n7, assign3880_e2647_d_n12,) = {
    if (var_guard35 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_vbcs, var_vbcs_dn7, var_vbcs_dn12,)
    }
};
        var_vbcs = assign3880_e2647;
        var_vbcs_dn7 = assign3880_e2647_d_n7;
        var_vbcs_dn12 = assign3880_e2647_d_n12;
        var_vbcs_rv = 0.0;

        *var_area_bt_n_slot = var_area_bt_n;
        *var_area_bt_n_rv_slot = var_area_bt_n_rv;
        *var_area_bt_p_slot = var_area_bt_p;
        *var_area_bt_p_rv_slot = var_area_bt_p_rv;
        *var_cbtn_slot = var_cbtn;
        *var_cbtn_rv_slot = var_cbtn_rv;
        *var_cbtp_slot = var_cbtp;
        *var_cbtp_rv_slot = var_cbtp_rv;
        *var_guard25_slot = var_guard25;
        *var_guard25_rv_slot = var_guard25_rv;
        *var_guard30_slot = var_guard30;
        *var_guard30_rv_slot = var_guard30_rv;
        *var_guard31_slot = var_guard31;
        *var_guard31_rv_slot = var_guard31_rv;
        *var_guard32_slot = var_guard32;
        *var_guard32_rv_slot = var_guard32_rv;
        *var_guard33_slot = var_guard33;
        *var_guard33_rv_slot = var_guard33_rv;
        *var_guard34_slot = var_guard34;
        *var_guard34_rv_slot = var_guard34_rv;
        *var_guard35_slot = var_guard35;
        *var_guard35_rv_slot = var_guard35_rv;
        *var_peri_hhi_slot = var_peri_hhi;
        *var_peri_hhi_rv_slot = var_peri_hhi_rv;
        *var_qb_nqs_slot = var_qb_nqs;
        *var_qb_nqs_dn13_slot = var_qb_nqs_dn13;
        *var_qb_nqs_rv_slot = var_qb_nqs_rv;
        *var_qi_nqs_slot = var_qi_nqs;
        *var_qi_nqs_dn18_slot = var_qi_nqs_dn18;
        *var_qi_nqs_rv_slot = var_qi_nqs_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_rv_slot = var_t0_rv;
        *var_uc_pdbcp_slot = var_uc_pdbcp;
        *var_uc_pdbcp_rv_slot = var_uc_pdbcp_rv;
        *var_uc_psbcp_slot = var_uc_psbcp;
        *var_uc_psbcp_rv_slot = var_uc_psbcp_rv;
        *var_vbcd_slot = var_vbcd;
        *var_vbcd_dn12_slot = var_vbcd_dn12;
        *var_vbcd_dn6_slot = var_vbcd_dn6;
        *var_vbcd_rv_slot = var_vbcd_rv;
        *var_vbcs_slot = var_vbcs;
        *var_vbcs_dn12_slot = var_vbcs_dn12;
        *var_vbcs_dn7_slot = var_vbcs_dn7;
        *var_vbcs_rv_slot = var_vbcs_rv;
        *var_vbsi_slot = var_vbsi;
        *var_vbsi_dn12_slot = var_vbsi_dn12;
        *var_vbsi_dn7_slot = var_vbsi_dn7;
        *var_vbsi_rv_slot = var_vbsi_rv;
        *var_vdsi_slot = var_vdsi;
        *var_vdsi_dn6_slot = var_vdsi_dn6;
        *var_vdsi_dn7_slot = var_vdsi_dn7;
        *var_vdsi_rv_slot = var_vdsi_rv;
        *var_vgsi_slot = var_vgsi;
        *var_vgsi_dn11_slot = var_vgsi_dn11;
        *var_vgsi_dn7_slot = var_vgsi_dn7;
        *var_vgsi_rv_slot = var_vgsi_rv;
        *var_vmax0_slot = var_vmax0;
        *var_vmax0_dn0_slot = var_vmax0_dn0;
        *var_vmax0_dn10_slot = var_vmax0_dn10;
        *var_vmax0_dn11_slot = var_vmax0_dn11;
        *var_vmax0_dn12_slot = var_vmax0_dn12;
        *var_vmax0_dn17_slot = var_vmax0_dn17;
        *var_vmax0_dn2_slot = var_vmax0_dn2;
        *var_vmax0_dn6_slot = var_vmax0_dn6;
        *var_vmax0_dn7_slot = var_vmax0_dn7;
        *var_vmax0_rv_slot = var_vmax0_rv;
        *var_w_diod_slot = var_w_diod;
        *var_w_diod_rv_slot = var_w_diod_rv;
        *var_w_diodcv_slot = var_w_diodcv;
        *var_w_diodcv_rv_slot = var_w_diodcv_rv;
        *var_w_dios_slot = var_w_dios;
        *var_w_dios_rv_slot = var_w_dios_rv;
        *var_w_dioscv_slot = var_w_dioscv;
        *var_w_dioscv_rv_slot = var_w_dioscv_rv;
    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_dtemp_given: f64,
        var_egtnom: f64,
        var_flg_nqs: f64,
        var_guard35: f64,
        var_lgle: f64,
        var_mks_rth0: f64,
        var_mks_vmax: f64,
        var_mks_vtmp: f64,
        var_ptovr0: f64,
        var_ptovr0_dn0: f64,
        var_ptovr0_dn10: f64,
        var_ptovr0_dn11: f64,
        var_ptovr0_dn12: f64,
        var_ptovr0_dn17: f64,
        var_ptovr0_dn2: f64,
        var_ptovr0_dn6: f64,
        var_ptovr0_dn7: f64,
        var_temp_given: f64,
        var_uc_temp: f64,
        var_uc_tnom: f64,
        var_vbsi: f64,
        var_vbsi_dn12: f64,
        var_vbsi_dn7: f64,
        var_vdsi: f64,
        var_vdsi_dn6: f64,
        var_vdsi_dn7: f64,
        var_vgsi: f64,
        var_vgsi_dn11: f64,
        var_vgsi_dn7: f64,
        var_vmax0: f64,
        var_vmax0_dn0: f64,
        var_vmax0_dn10: f64,
        var_vmax0_dn11: f64,
        var_vmax0_dn12: f64,
        var_vmax0_dn17: f64,
        var_vmax0_dn2: f64,
        var_vmax0_dn6: f64,
        var_vmax0_dn7: f64,
        var_wg: f64,
        var_wl: f64,
        var_beta_slot: &mut f64,
        var_beta2_slot: &mut f64,
        var_beta2_dn10_slot: &mut f64,
        var_beta2_rv_slot: &mut f64,
        var_beta_dn10_slot: &mut f64,
        var_beta_inv_slot: &mut f64,
        var_beta_inv_dn10_slot: &mut f64,
        var_beta_inv_rv_slot: &mut f64,
        var_beta_rv_slot: &mut f64,
        var_cgs_mphn0_slot: &mut f64,
        var_cgs_mphn0_dn10_slot: &mut f64,
        var_cgs_mphn0_rv_slot: &mut f64,
        var_cgs_mueph_slot: &mut f64,
        var_cgs_mueph_rv_slot: &mut f64,
        var_cgs_wmueph_slot: &mut f64,
        var_cgs_wmueph_rv_slot: &mut f64,
        var_deltemp_slot: &mut f64,
        var_deltemp_dn10_slot: &mut f64,
        var_deltemp_rv_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_dn0_slot: &mut f64,
        var_eg_dn10_slot: &mut f64,
        var_eg_dn11_slot: &mut f64,
        var_eg_dn12_slot: &mut f64,
        var_eg_dn17_slot: &mut f64,
        var_eg_dn2_slot: &mut f64,
        var_eg_dn6_slot: &mut f64,
        var_eg_dn7_slot: &mut f64,
        var_eg_rv_slot: &mut f64,
        var_egp12_slot: &mut f64,
        var_egp12_dn0_slot: &mut f64,
        var_egp12_dn10_slot: &mut f64,
        var_egp12_dn11_slot: &mut f64,
        var_egp12_dn12_slot: &mut f64,
        var_egp12_dn17_slot: &mut f64,
        var_egp12_dn2_slot: &mut f64,
        var_egp12_dn6_slot: &mut f64,
        var_egp12_dn7_slot: &mut f64,
        var_egp12_rv_slot: &mut f64,
        var_egp32_slot: &mut f64,
        var_egp32_dn0_slot: &mut f64,
        var_egp32_dn10_slot: &mut f64,
        var_egp32_dn11_slot: &mut f64,
        var_egp32_dn12_slot: &mut f64,
        var_egp32_dn17_slot: &mut f64,
        var_egp32_dn2_slot: &mut f64,
        var_egp32_dn6_slot: &mut f64,
        var_egp32_dn7_slot: &mut f64,
        var_egp32_rv_slot: &mut f64,
        var_guard36_slot: &mut f64,
        var_guard36_rv_slot: &mut f64,
        var_mode_slot: &mut f64,
        var_mode_rv_slot: &mut f64,
        var_modenml_slot: &mut f64,
        var_modenml_rv_slot: &mut f64,
        var_modervs_slot: &mut f64,
        var_modervs_rv_slot: &mut f64,
        var_ptovr_slot: &mut f64,
        var_ptovr_dn0_slot: &mut f64,
        var_ptovr_dn10_slot: &mut f64,
        var_ptovr_dn11_slot: &mut f64,
        var_ptovr_dn12_slot: &mut f64,
        var_ptovr_dn17_slot: &mut f64,
        var_ptovr_dn2_slot: &mut f64,
        var_ptovr_dn6_slot: &mut f64,
        var_ptovr_dn7_slot: &mut f64,
        var_ptovr_rv_slot: &mut f64,
        var_qb_nqs_slot: &mut f64,
        var_qb_nqs_dn13_slot: &mut f64,
        var_qb_nqs_rv_slot: &mut f64,
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
        var_qd_nqs_rv_slot: &mut f64,
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
        var_qs_nqs_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1__blk41_slot: &mut f64,
        var_t1__blk41_dn10_slot: &mut f64,
        var_t1__blk41_rv_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2__blk42_slot: &mut f64,
        var_t2__blk42_rv_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3__blk43_slot: &mut f64,
        var_t3__blk43_rv_slot: &mut f64,
        var_ttemp_slot: &mut f64,
        var_ttemp_dn10_slot: &mut f64,
        var_ttemp_rv_slot: &mut f64,
        var_vbs_slot: &mut f64,
        var_vbs_dn0_slot: &mut f64,
        var_vbs_dn10_slot: &mut f64,
        var_vbs_dn11_slot: &mut f64,
        var_vbs_dn12_slot: &mut f64,
        var_vbs_dn17_slot: &mut f64,
        var_vbs_dn2_slot: &mut f64,
        var_vbs_dn6_slot: &mut f64,
        var_vbs_dn7_slot: &mut f64,
        var_vbs_rv_slot: &mut f64,
        var_vds_slot: &mut f64,
        var_vds_dn0_slot: &mut f64,
        var_vds_dn10_slot: &mut f64,
        var_vds_dn11_slot: &mut f64,
        var_vds_dn12_slot: &mut f64,
        var_vds_dn17_slot: &mut f64,
        var_vds_dn2_slot: &mut f64,
        var_vds_dn6_slot: &mut f64,
        var_vds_dn7_slot: &mut f64,
        var_vds_rv_slot: &mut f64,
        var_vgs_slot: &mut f64,
        var_vgs_dn11_slot: &mut f64,
        var_vgs_dn6_slot: &mut f64,
        var_vgs_dn7_slot: &mut f64,
        var_vgs_rv_slot: &mut f64,
        var_vmaxe_slot: &mut f64,
        var_vmaxe_dn0_slot: &mut f64,
        var_vmaxe_dn10_slot: &mut f64,
        var_vmaxe_dn11_slot: &mut f64,
        var_vmaxe_dn12_slot: &mut f64,
        var_vmaxe_dn17_slot: &mut f64,
        var_vmaxe_dn2_slot: &mut f64,
        var_vmaxe_dn6_slot: &mut f64,
        var_vmaxe_dn7_slot: &mut f64,
        var_vmaxe_rv_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let mut var_beta: f64 = *var_beta_slot;
        let mut var_beta2: f64 = *var_beta2_slot;
        let mut var_beta2_dn10: f64 = *var_beta2_dn10_slot;
        let mut var_beta2_rv: f64 = *var_beta2_rv_slot;
        let mut var_beta_dn10: f64 = *var_beta_dn10_slot;
        let mut var_beta_inv: f64 = *var_beta_inv_slot;
        let mut var_beta_inv_dn10: f64 = *var_beta_inv_dn10_slot;
        let mut var_beta_inv_rv: f64 = *var_beta_inv_rv_slot;
        let mut var_beta_rv: f64 = *var_beta_rv_slot;
        let mut var_cgs_mphn0: f64 = *var_cgs_mphn0_slot;
        let mut var_cgs_mphn0_dn10: f64 = *var_cgs_mphn0_dn10_slot;
        let mut var_cgs_mphn0_rv: f64 = *var_cgs_mphn0_rv_slot;
        let mut var_cgs_mueph: f64 = *var_cgs_mueph_slot;
        let mut var_cgs_mueph_rv: f64 = *var_cgs_mueph_rv_slot;
        let mut var_cgs_wmueph: f64 = *var_cgs_wmueph_slot;
        let mut var_cgs_wmueph_rv: f64 = *var_cgs_wmueph_rv_slot;
        let mut var_deltemp: f64 = *var_deltemp_slot;
        let mut var_deltemp_dn10: f64 = *var_deltemp_dn10_slot;
        let mut var_deltemp_rv: f64 = *var_deltemp_rv_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_dn0: f64 = *var_eg_dn0_slot;
        let mut var_eg_dn10: f64 = *var_eg_dn10_slot;
        let mut var_eg_dn11: f64 = *var_eg_dn11_slot;
        let mut var_eg_dn12: f64 = *var_eg_dn12_slot;
        let mut var_eg_dn17: f64 = *var_eg_dn17_slot;
        let mut var_eg_dn2: f64 = *var_eg_dn2_slot;
        let mut var_eg_dn6: f64 = *var_eg_dn6_slot;
        let mut var_eg_dn7: f64 = *var_eg_dn7_slot;
        let mut var_eg_rv: f64 = *var_eg_rv_slot;
        let mut var_egp12: f64 = *var_egp12_slot;
        let mut var_egp12_dn0: f64 = *var_egp12_dn0_slot;
        let mut var_egp12_dn10: f64 = *var_egp12_dn10_slot;
        let mut var_egp12_dn11: f64 = *var_egp12_dn11_slot;
        let mut var_egp12_dn12: f64 = *var_egp12_dn12_slot;
        let mut var_egp12_dn17: f64 = *var_egp12_dn17_slot;
        let mut var_egp12_dn2: f64 = *var_egp12_dn2_slot;
        let mut var_egp12_dn6: f64 = *var_egp12_dn6_slot;
        let mut var_egp12_dn7: f64 = *var_egp12_dn7_slot;
        let mut var_egp12_rv: f64 = *var_egp12_rv_slot;
        let mut var_egp32: f64 = *var_egp32_slot;
        let mut var_egp32_dn0: f64 = *var_egp32_dn0_slot;
        let mut var_egp32_dn10: f64 = *var_egp32_dn10_slot;
        let mut var_egp32_dn11: f64 = *var_egp32_dn11_slot;
        let mut var_egp32_dn12: f64 = *var_egp32_dn12_slot;
        let mut var_egp32_dn17: f64 = *var_egp32_dn17_slot;
        let mut var_egp32_dn2: f64 = *var_egp32_dn2_slot;
        let mut var_egp32_dn6: f64 = *var_egp32_dn6_slot;
        let mut var_egp32_dn7: f64 = *var_egp32_dn7_slot;
        let mut var_egp32_rv: f64 = *var_egp32_rv_slot;
        let mut var_guard36: f64 = *var_guard36_slot;
        let mut var_guard36_rv: f64 = *var_guard36_rv_slot;
        let mut var_mode: f64 = *var_mode_slot;
        let mut var_mode_rv: f64 = *var_mode_rv_slot;
        let mut var_modenml: f64 = *var_modenml_slot;
        let mut var_modenml_rv: f64 = *var_modenml_rv_slot;
        let mut var_modervs: f64 = *var_modervs_slot;
        let mut var_modervs_rv: f64 = *var_modervs_rv_slot;
        let mut var_ptovr: f64 = *var_ptovr_slot;
        let mut var_ptovr_dn0: f64 = *var_ptovr_dn0_slot;
        let mut var_ptovr_dn10: f64 = *var_ptovr_dn10_slot;
        let mut var_ptovr_dn11: f64 = *var_ptovr_dn11_slot;
        let mut var_ptovr_dn12: f64 = *var_ptovr_dn12_slot;
        let mut var_ptovr_dn17: f64 = *var_ptovr_dn17_slot;
        let mut var_ptovr_dn2: f64 = *var_ptovr_dn2_slot;
        let mut var_ptovr_dn6: f64 = *var_ptovr_dn6_slot;
        let mut var_ptovr_dn7: f64 = *var_ptovr_dn7_slot;
        let mut var_ptovr_rv: f64 = *var_ptovr_rv_slot;
        let mut var_qb_nqs: f64 = *var_qb_nqs_slot;
        let mut var_qb_nqs_dn13: f64 = *var_qb_nqs_dn13_slot;
        let mut var_qb_nqs_rv: f64 = *var_qb_nqs_rv_slot;
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
        let mut var_qd_nqs_rv: f64 = *var_qd_nqs_rv_slot;
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
        let mut var_qs_nqs_rv: f64 = *var_qs_nqs_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1__blk41: f64 = *var_t1__blk41_slot;
        let mut var_t1__blk41_dn10: f64 = *var_t1__blk41_dn10_slot;
        let mut var_t1__blk41_rv: f64 = *var_t1__blk41_rv_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2__blk42: f64 = *var_t2__blk42_slot;
        let mut var_t2__blk42_rv: f64 = *var_t2__blk42_rv_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3__blk43: f64 = *var_t3__blk43_slot;
        let mut var_t3__blk43_rv: f64 = *var_t3__blk43_rv_slot;
        let mut var_ttemp: f64 = *var_ttemp_slot;
        let mut var_ttemp_dn10: f64 = *var_ttemp_dn10_slot;
        let mut var_ttemp_rv: f64 = *var_ttemp_rv_slot;
        let mut var_vbs: f64 = *var_vbs_slot;
        let mut var_vbs_dn0: f64 = *var_vbs_dn0_slot;
        let mut var_vbs_dn10: f64 = *var_vbs_dn10_slot;
        let mut var_vbs_dn11: f64 = *var_vbs_dn11_slot;
        let mut var_vbs_dn12: f64 = *var_vbs_dn12_slot;
        let mut var_vbs_dn17: f64 = *var_vbs_dn17_slot;
        let mut var_vbs_dn2: f64 = *var_vbs_dn2_slot;
        let mut var_vbs_dn6: f64 = *var_vbs_dn6_slot;
        let mut var_vbs_dn7: f64 = *var_vbs_dn7_slot;
        let mut var_vbs_rv: f64 = *var_vbs_rv_slot;
        let mut var_vds: f64 = *var_vds_slot;
        let mut var_vds_dn0: f64 = *var_vds_dn0_slot;
        let mut var_vds_dn10: f64 = *var_vds_dn10_slot;
        let mut var_vds_dn11: f64 = *var_vds_dn11_slot;
        let mut var_vds_dn12: f64 = *var_vds_dn12_slot;
        let mut var_vds_dn17: f64 = *var_vds_dn17_slot;
        let mut var_vds_dn2: f64 = *var_vds_dn2_slot;
        let mut var_vds_dn6: f64 = *var_vds_dn6_slot;
        let mut var_vds_dn7: f64 = *var_vds_dn7_slot;
        let mut var_vds_rv: f64 = *var_vds_rv_slot;
        let mut var_vgs: f64 = *var_vgs_slot;
        let mut var_vgs_dn11: f64 = *var_vgs_dn11_slot;
        let mut var_vgs_dn6: f64 = *var_vgs_dn6_slot;
        let mut var_vgs_dn7: f64 = *var_vgs_dn7_slot;
        let mut var_vgs_rv: f64 = *var_vgs_rv_slot;
        let mut var_vmaxe: f64 = *var_vmaxe_slot;
        let mut var_vmaxe_dn0: f64 = *var_vmaxe_dn0_slot;
        let mut var_vmaxe_dn10: f64 = *var_vmaxe_dn10_slot;
        let mut var_vmaxe_dn11: f64 = *var_vmaxe_dn11_slot;
        let mut var_vmaxe_dn12: f64 = *var_vmaxe_dn12_slot;
        let mut var_vmaxe_dn17: f64 = *var_vmaxe_dn17_slot;
        let mut var_vmaxe_dn2: f64 = *var_vmaxe_dn2_slot;
        let mut var_vmaxe_dn6: f64 = *var_vmaxe_dn6_slot;
        let mut var_vmaxe_dn7: f64 = *var_vmaxe_dn7_slot;
        let mut var_vmaxe_rv: f64 = *var_vmaxe_rv_slot;

        let (assign3890_e2658, assign3890_e2658_d_n0, assign3890_e2658_d_n2, assign3890_e2658_d_n6, assign3890_e2658_d_n7, assign3890_e2658_d_n10, assign3890_e2658_d_n11, assign3890_e2658_d_n12, assign3890_e2658_d_n15, assign3890_e2658_d_n17, assign3890_e2658_d_n18,) = {
    if ((var_guard35 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign3890_e2654: f64 = (1e-9 / 0.0001);
        let assign3890_e2656: f64 = (assign3890_e2654 * (nv15 - 0.0));
        (assign3890_e2656, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign3890_e2654, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign3890_e2658;
        var_qd_nqs_dn0 = assign3890_e2658_d_n0;
        var_qd_nqs_dn2 = assign3890_e2658_d_n2;
        var_qd_nqs_dn6 = assign3890_e2658_d_n6;
        var_qd_nqs_dn7 = assign3890_e2658_d_n7;
        var_qd_nqs_dn10 = assign3890_e2658_d_n10;
        var_qd_nqs_dn11 = assign3890_e2658_d_n11;
        var_qd_nqs_dn12 = assign3890_e2658_d_n12;
        var_qd_nqs_dn15 = assign3890_e2658_d_n15;
        var_qd_nqs_dn17 = assign3890_e2658_d_n17;
        var_qd_nqs_dn18 = assign3890_e2658_d_n18;
        var_qd_nqs_rv = 0.0;

        let (assign3900_e2669, assign3900_e2669_d_n0, assign3900_e2669_d_n2, assign3900_e2669_d_n6, assign3900_e2669_d_n7, assign3900_e2669_d_n10, assign3900_e2669_d_n11, assign3900_e2669_d_n12, assign3900_e2669_d_n16, assign3900_e2669_d_n17, assign3900_e2669_d_n18,) = {
    if ((var_guard35 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign3900_e2665: f64 = (1e-9 / 0.0001);
        let assign3900_e2667: f64 = (assign3900_e2665 * (nv16 - 0.0));
        (assign3900_e2667, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign3900_e2665, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign3900_e2669;
        var_qs_nqs_dn0 = assign3900_e2669_d_n0;
        var_qs_nqs_dn2 = assign3900_e2669_d_n2;
        var_qs_nqs_dn6 = assign3900_e2669_d_n6;
        var_qs_nqs_dn7 = assign3900_e2669_d_n7;
        var_qs_nqs_dn10 = assign3900_e2669_d_n10;
        var_qs_nqs_dn11 = assign3900_e2669_d_n11;
        var_qs_nqs_dn12 = assign3900_e2669_d_n12;
        var_qs_nqs_dn16 = assign3900_e2669_d_n16;
        var_qs_nqs_dn17 = assign3900_e2669_d_n17;
        var_qs_nqs_dn18 = assign3900_e2669_d_n18;
        var_qs_nqs_rv = 0.0;

        let (assign3910_e2680, assign3910_e2680_d_n13,) = {
    if ((var_guard35 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign3910_e2676: f64 = (1e-9 / 0.0001);
        let assign3910_e2678: f64 = (assign3910_e2676 * (nv13 - 0.0));
        (assign3910_e2678, assign3910_e2676,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign3910_e2680;
        var_qb_nqs_dn13 = assign3910_e2680_d_n13;
        var_qb_nqs_rv = 0.0;

        let (assign3920_e2688, assign3920_e2688_d_n0, assign3920_e2688_d_n2, assign3920_e2688_d_n6, assign3920_e2688_d_n7, assign3920_e2688_d_n10, assign3920_e2688_d_n11, assign3920_e2688_d_n12, assign3920_e2688_d_n15, assign3920_e2688_d_n17, assign3920_e2688_d_n18,) = {
    if ((var_guard35 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign3920_e2688;
        var_qd_nqs_dn0 = assign3920_e2688_d_n0;
        var_qd_nqs_dn2 = assign3920_e2688_d_n2;
        var_qd_nqs_dn6 = assign3920_e2688_d_n6;
        var_qd_nqs_dn7 = assign3920_e2688_d_n7;
        var_qd_nqs_dn10 = assign3920_e2688_d_n10;
        var_qd_nqs_dn11 = assign3920_e2688_d_n11;
        var_qd_nqs_dn12 = assign3920_e2688_d_n12;
        var_qd_nqs_dn15 = assign3920_e2688_d_n15;
        var_qd_nqs_dn17 = assign3920_e2688_d_n17;
        var_qd_nqs_dn18 = assign3920_e2688_d_n18;
        var_qd_nqs_rv = 0.0;

        let (assign3930_e2696, assign3930_e2696_d_n0, assign3930_e2696_d_n2, assign3930_e2696_d_n6, assign3930_e2696_d_n7, assign3930_e2696_d_n10, assign3930_e2696_d_n11, assign3930_e2696_d_n12, assign3930_e2696_d_n16, assign3930_e2696_d_n17, assign3930_e2696_d_n18,) = {
    if ((var_guard35 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign3930_e2696;
        var_qs_nqs_dn0 = assign3930_e2696_d_n0;
        var_qs_nqs_dn2 = assign3930_e2696_d_n2;
        var_qs_nqs_dn6 = assign3930_e2696_d_n6;
        var_qs_nqs_dn7 = assign3930_e2696_d_n7;
        var_qs_nqs_dn10 = assign3930_e2696_d_n10;
        var_qs_nqs_dn11 = assign3930_e2696_d_n11;
        var_qs_nqs_dn12 = assign3930_e2696_d_n12;
        var_qs_nqs_dn16 = assign3930_e2696_d_n16;
        var_qs_nqs_dn17 = assign3930_e2696_d_n17;
        var_qs_nqs_dn18 = assign3930_e2696_d_n18;
        var_qs_nqs_rv = 0.0;

        let (assign3940_e2704, assign3940_e2704_d_n13,) = {
    if ((var_guard35 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign3940_e2704;
        var_qb_nqs_dn13 = assign3940_e2704_d_n13;
        var_qb_nqs_rv = 0.0;

        let (assign3950_e2719, assign3950_e2719_d_n10,) = {
    if ((p.p38 > 0.0) && (var_mks_rth0 > 0.0)) {
        let (assign3950_e2717, assign3950_e2717_d_n10,) = {
            if ((nv10 - 0.0) > 0.0) {
                ((nv10 - 0.0), 1.0,)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign3950_e2717, assign3950_e2717_d_n10,)
    } else {
        (0.0, 0.0,)
    }
};
        var_deltemp = assign3950_e2719;
        var_deltemp_dn10 = assign3950_e2719_d_n10;
        var_deltemp_rv = 0.0;

        let assign3960_e2722: f64 = if var_vdsi >= 0.0 { 1.0 } else { 0.0 };
        var_guard36 = assign3960_e2722;
        var_guard36_rv = 0.0;

        let (assign3970_e2726,) = {
    if (var_guard36 != 0.0) {
        (1.0,)
    } else {
        (var_mode,)
    }
};
        var_mode = assign3970_e2726;
        var_mode_rv = 0.0;

        let (assign3980_e2730,) = {
    if (var_guard36 != 0.0) {
        (1.0,)
    } else {
        (var_modenml,)
    }
};
        var_modenml = assign3980_e2730;
        var_modenml_rv = 0.0;

        let (assign3990_e2734,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_modervs,)
    }
};
        var_modervs = assign3990_e2734;
        var_modervs_rv = 0.0;

        let (assign4000_e2738, assign4000_e2738_d_n0, assign4000_e2738_d_n2, assign4000_e2738_d_n6, assign4000_e2738_d_n7, assign4000_e2738_d_n10, assign4000_e2738_d_n11, assign4000_e2738_d_n12, assign4000_e2738_d_n17,) = {
    if (var_guard36 != 0.0) {
        (var_vdsi, 0.0, 0.0, var_vdsi_dn6, var_vdsi_dn7, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vds, var_vds_dn0, var_vds_dn2, var_vds_dn6, var_vds_dn7, var_vds_dn10, var_vds_dn11, var_vds_dn12, var_vds_dn17,)
    }
};
        var_vds = assign4000_e2738;
        var_vds_dn0 = assign4000_e2738_d_n0;
        var_vds_dn2 = assign4000_e2738_d_n2;
        var_vds_dn6 = assign4000_e2738_d_n6;
        var_vds_dn7 = assign4000_e2738_d_n7;
        var_vds_dn10 = assign4000_e2738_d_n10;
        var_vds_dn11 = assign4000_e2738_d_n11;
        var_vds_dn12 = assign4000_e2738_d_n12;
        var_vds_dn17 = assign4000_e2738_d_n17;
        var_vds_rv = 0.0;

        let (assign4010_e2742, assign4010_e2742_d_n6, assign4010_e2742_d_n7, assign4010_e2742_d_n11,) = {
    if (var_guard36 != 0.0) {
        (var_vgsi, 0.0, var_vgsi_dn7, var_vgsi_dn11,)
    } else {
        (var_vgs, var_vgs_dn6, var_vgs_dn7, var_vgs_dn11,)
    }
};
        var_vgs = assign4010_e2742;
        var_vgs_dn6 = assign4010_e2742_d_n6;
        var_vgs_dn7 = assign4010_e2742_d_n7;
        var_vgs_dn11 = assign4010_e2742_d_n11;
        var_vgs_rv = 0.0;

        let (assign4020_e2746, assign4020_e2746_d_n0, assign4020_e2746_d_n2, assign4020_e2746_d_n6, assign4020_e2746_d_n7, assign4020_e2746_d_n10, assign4020_e2746_d_n11, assign4020_e2746_d_n12, assign4020_e2746_d_n17,) = {
    if (var_guard36 != 0.0) {
        (var_vbsi, 0.0, 0.0, 0.0, var_vbsi_dn7, 0.0, 0.0, var_vbsi_dn12, 0.0,)
    } else {
        (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    }
};
        var_vbs = assign4020_e2746;
        var_vbs_dn0 = assign4020_e2746_d_n0;
        var_vbs_dn2 = assign4020_e2746_d_n2;
        var_vbs_dn6 = assign4020_e2746_d_n6;
        var_vbs_dn7 = assign4020_e2746_d_n7;
        var_vbs_dn10 = assign4020_e2746_d_n10;
        var_vbs_dn11 = assign4020_e2746_d_n11;
        var_vbs_dn12 = assign4020_e2746_d_n12;
        var_vbs_dn17 = assign4020_e2746_d_n17;
        var_vbs_rv = 0.0;

        let (assign4060_e2764,) = {
    if (var_guard36 == 0.0) {
        let assign4060_e2762: f64 = (-1.0);
        (assign4060_e2762,)
    } else {
        (var_mode,)
    }
};
        var_mode = assign4060_e2764;
        var_mode_rv = 0.0;

        let (assign4070_e2769,) = {
    if (var_guard36 == 0.0) {
        (0.0,)
    } else {
        (var_modenml,)
    }
};
        var_modenml = assign4070_e2769;
        var_modenml_rv = 0.0;

        let (assign4080_e2774,) = {
    if (var_guard36 == 0.0) {
        (1.0,)
    } else {
        (var_modervs,)
    }
};
        var_modervs = assign4080_e2774;
        var_modervs_rv = 0.0;

        let (assign4090_e2780, assign4090_e2780_d_n0, assign4090_e2780_d_n2, assign4090_e2780_d_n6, assign4090_e2780_d_n7, assign4090_e2780_d_n10, assign4090_e2780_d_n11, assign4090_e2780_d_n12, assign4090_e2780_d_n17,) = {
    if (var_guard36 == 0.0) {
        let assign4090_e2778: f64 = (-var_vdsi);
        (assign4090_e2778, 0.0, 0.0, (-var_vdsi_dn6), (-var_vdsi_dn7), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vds, var_vds_dn0, var_vds_dn2, var_vds_dn6, var_vds_dn7, var_vds_dn10, var_vds_dn11, var_vds_dn12, var_vds_dn17,)
    }
};
        var_vds = assign4090_e2780;
        var_vds_dn0 = assign4090_e2780_d_n0;
        var_vds_dn2 = assign4090_e2780_d_n2;
        var_vds_dn6 = assign4090_e2780_d_n6;
        var_vds_dn7 = assign4090_e2780_d_n7;
        var_vds_dn10 = assign4090_e2780_d_n10;
        var_vds_dn11 = assign4090_e2780_d_n11;
        var_vds_dn12 = assign4090_e2780_d_n12;
        var_vds_dn17 = assign4090_e2780_d_n17;
        var_vds_rv = 0.0;

        let (assign4100_e2787, assign4100_e2787_d_n6, assign4100_e2787_d_n7, assign4100_e2787_d_n11,) = {
    if (var_guard36 == 0.0) {
        let assign4100_e2785: f64 = (var_vgsi - var_vdsi);
        (assign4100_e2785, (-var_vdsi_dn6), (var_vgsi_dn7 - var_vdsi_dn7), var_vgsi_dn11,)
    } else {
        (var_vgs, var_vgs_dn6, var_vgs_dn7, var_vgs_dn11,)
    }
};
        var_vgs = assign4100_e2787;
        var_vgs_dn6 = assign4100_e2787_d_n6;
        var_vgs_dn7 = assign4100_e2787_d_n7;
        var_vgs_dn11 = assign4100_e2787_d_n11;
        var_vgs_rv = 0.0;

        let (assign4110_e2794, assign4110_e2794_d_n0, assign4110_e2794_d_n2, assign4110_e2794_d_n6, assign4110_e2794_d_n7, assign4110_e2794_d_n10, assign4110_e2794_d_n11, assign4110_e2794_d_n12, assign4110_e2794_d_n17,) = {
    if (var_guard36 == 0.0) {
        let assign4110_e2792: f64 = (var_vbsi - var_vdsi);
        (assign4110_e2792, 0.0, 0.0, (-var_vdsi_dn6), (var_vbsi_dn7 - var_vdsi_dn7), 0.0, 0.0, var_vbsi_dn12, 0.0,)
    } else {
        (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    }
};
        var_vbs = assign4110_e2794;
        var_vbs_dn0 = assign4110_e2794_d_n0;
        var_vbs_dn2 = assign4110_e2794_d_n2;
        var_vbs_dn6 = assign4110_e2794_d_n6;
        var_vbs_dn7 = assign4110_e2794_d_n7;
        var_vbs_dn10 = assign4110_e2794_d_n10;
        var_vbs_dn11 = assign4110_e2794_d_n11;
        var_vbs_dn12 = assign4110_e2794_d_n12;
        var_vbs_dn17 = assign4110_e2794_d_n17;
        var_vbs_rv = 0.0;

        let assign4170_e2821: f64 = ctx_temp;
        var_ttemp = assign4170_e2821;
        var_ttemp_dn10 = 0.0;
        var_ttemp_rv = 0.0;

        let (assign4180_e2825, assign4180_e2825_d_n10,) = {
    if (var_temp_given != 0.0) {
        (var_uc_temp, 0.0,)
    } else {
        (var_ttemp, var_ttemp_dn10,)
    }
};
        var_ttemp = assign4180_e2825;
        var_ttemp_dn10 = assign4180_e2825_d_n10;
        var_ttemp_rv = 0.0;

        let (assign4190_e2831, assign4190_e2831_d_n10,) = {
    if (var_dtemp_given != 0.0) {
        let assign4190_e2829: f64 = (var_ttemp + p.p17);
        (assign4190_e2829, var_ttemp_dn10,)
    } else {
        (var_ttemp, var_ttemp_dn10,)
    }
};
        var_ttemp = assign4190_e2831;
        var_ttemp_dn10 = assign4190_e2831_d_n10;
        var_ttemp_rv = 0.0;

        let assign4200_e2834: f64 = (var_ttemp + var_deltemp);
        var_ttemp = assign4200_e2834;
        var_ttemp_dn10 = (var_ttemp_dn10 + var_deltemp_dn10);
        var_ttemp_rv = 0.0;

        let assign4210_e2837: f64 = (var_ttemp - var_uc_tnom);
        var_t1 = assign4210_e2837;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn10 = var_ttemp_dn10;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn17 = 0.0;
        var_t1_rv = 0.0;

        let assign4220_e2841: f64 = (var_ttemp + var_uc_tnom);
        let assign4220_e2842: f64 = (var_t1 * assign4220_e2841);
        var_t2 = assign4220_e2842;
        var_t2_dn0 = (var_t1_dn0 * assign4220_e2841);
        var_t2_dn2 = (var_t1_dn2 * assign4220_e2841);
        var_t2_dn6 = (var_t1_dn6 * assign4220_e2841);
        var_t2_dn7 = (var_t1_dn7 * assign4220_e2841);
        var_t2_dn10 = ((var_t1_dn10 * assign4220_e2841) + (var_t1 * var_ttemp_dn10));
        var_t2_dn11 = (var_t1_dn11 * assign4220_e2841);
        var_t2_dn12 = (var_t1_dn12 * assign4220_e2841);
        var_t2_dn17 = (var_t1_dn17 * assign4220_e2841);
        var_t2_rv = 0.0;

        let assign4230_e2846: f64 = (p.p53 * var_t1);
        let assign4230_e2847: f64 = (var_egtnom - assign4230_e2846);
        let assign4230_e2850: f64 = (p.p54 * var_t2);
        let assign4230_e2851: f64 = (assign4230_e2847 - assign4230_e2850);
        var_eg = assign4230_e2851;
        var_eg_dn0 = ((-(p.p53 * var_t1_dn0)) - (p.p54 * var_t2_dn0));
        var_eg_dn2 = ((-(p.p53 * var_t1_dn2)) - (p.p54 * var_t2_dn2));
        var_eg_dn6 = ((-(p.p53 * var_t1_dn6)) - (p.p54 * var_t2_dn6));
        var_eg_dn7 = ((-(p.p53 * var_t1_dn7)) - (p.p54 * var_t2_dn7));
        var_eg_dn10 = ((-(p.p53 * var_t1_dn10)) - (p.p54 * var_t2_dn10));
        var_eg_dn11 = ((-(p.p53 * var_t1_dn11)) - (p.p54 * var_t2_dn11));
        var_eg_dn12 = ((-(p.p53 * var_t1_dn12)) - (p.p54 * var_t2_dn12));
        var_eg_dn17 = ((-(p.p53 * var_t1_dn17)) - (p.p54 * var_t2_dn17));
        var_eg_rv = 0.0;

        let assign4240_e2855: f64 = (1.3806226e-23 * var_ttemp);
        let assign4240_e2856: f64 = (1.6021918e-19 / assign4240_e2855);
        var_beta = assign4240_e2856;
        var_beta_dn10 = (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn10)) / (assign4240_e2855 * assign4240_e2855)));
        var_beta_rv = 0.0;

        let assign4250_e2859: f64 = (var_beta * var_beta);
        var_beta2 = assign4250_e2859;
        var_beta2_dn10 = ((var_beta_dn10 * var_beta) + (var_beta * var_beta_dn10));
        var_beta2_rv = 0.0;

        let assign4260_e2862: f64 = (1.0 / var_beta);
        var_beta_inv = assign4260_e2862;
        var_beta_inv_dn10 = (-(var_beta_dn10 / (var_beta * var_beta)));
        var_beta_inv_rv = 0.0;

        let assign4270_e2868: f64 = (var_wg).powf(p.p99);
        let assign4270_e2869: f64 = (p.p98 / assign4270_e2868);
        let assign4270_e2870: f64 = (1.0 + assign4270_e2869);
        let assign4270_e2871: f64 = (p.p254 * assign4270_e2870);
        let assign4270_e2876: f64 = (var_lgle).powf(p.p101);
        let assign4270_e2877: f64 = (p.p100 / assign4270_e2876);
        let assign4270_e2878: f64 = (1.0 + assign4270_e2877);
        let assign4270_e2879: f64 = (assign4270_e2871 * assign4270_e2878);
        let assign4270_e2884: f64 = (var_wl).powf(p.p103);
        let assign4270_e2885: f64 = (p.p102 / assign4270_e2884);
        let assign4270_e2886: f64 = (1.0 + assign4270_e2885);
        let assign4270_e2887: f64 = (assign4270_e2879 * assign4270_e2886);
        var_cgs_mueph = assign4270_e2887;
        var_cgs_mueph_rv = 0.0;

        let assign4280_e2891: f64 = (1.0 + p.p159);
        let assign4280_e2892: f64 = (1.0 / assign4280_e2891);
        var_t2__blk42 = assign4280_e2892;
        var_t2__blk42_rv = 0.0;

        var_t3__blk43 = 0.0;
        var_t3__blk43_rv = 0.0;

        let assign4300_e2898: f64 = (var_t2__blk42 * var_t3__blk43);
        let assign4300_e2899: f64 = (1.0 + assign4300_e2898);
        let assign4300_e2900: f64 = (var_cgs_mueph * assign4300_e2899);
        var_cgs_wmueph = assign4300_e2900;
        var_cgs_wmueph_rv = 0.0;

        let assign4310_e2903: f64 = (var_ttemp / var_uc_tnom);
        let assign4310_e2905: f64 = (assign4310_e2903).powf(p.p112);
        var_t1__blk41 = assign4310_e2905;
        var_t1__blk41_dn10 = if 0.0 == 0.0 && ((p.p112) as f64).is_finite() && ((p.p112) as f64).fract() == 0.0 { if p.p112 == 0.0 { 0.0 } else { (p.p112 * ((assign4310_e2903).powf(p.p112 - 1.0) * (var_ttemp_dn10 / var_uc_tnom))) } } else { (assign4310_e2905 * (p.p112 * ((var_ttemp_dn10 / var_uc_tnom) / assign4310_e2903))) };
        var_t1__blk41_rv = 0.0;

        let assign4320_e2908: f64 = (var_t1__blk41 / var_cgs_wmueph);
        var_cgs_mphn0 = assign4320_e2908;
        var_cgs_mphn0_dn10 = (var_t1__blk41_dn10 / var_cgs_wmueph);
        var_cgs_mphn0_rv = 0.0;

        let assign4330_e2911: f64 = (var_ptovr0 * var_beta_inv);
        var_ptovr = assign4330_e2911;
        var_ptovr_dn0 = (var_ptovr0_dn0 * var_beta_inv);
        var_ptovr_dn2 = (var_ptovr0_dn2 * var_beta_inv);
        var_ptovr_dn6 = (var_ptovr0_dn6 * var_beta_inv);
        var_ptovr_dn7 = (var_ptovr0_dn7 * var_beta_inv);
        var_ptovr_dn10 = ((var_ptovr0_dn10 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn10));
        var_ptovr_dn11 = (var_ptovr0_dn11 * var_beta_inv);
        var_ptovr_dn12 = (var_ptovr0_dn12 * var_beta_inv);
        var_ptovr_dn17 = (var_ptovr0_dn17 * var_beta_inv);
        var_ptovr_rv = 0.0;

        let assign4340_e2914: f64 = (var_ttemp / var_uc_tnom);
        var_t1 = assign4340_e2914;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn10 = (var_ttemp_dn10 / var_uc_tnom);
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn17 = 0.0;
        var_t1_rv = 0.0;

        let assign4350_e2917: f64 = (var_vmax0 * var_mks_vmax);
        let assign4350_e2921: f64 = (0.4 * var_t1);
        let assign4350_e2922: f64 = (1.8 + assign4350_e2921);
        let assign4350_e2925: f64 = (0.1 * var_t1);
        let assign4350_e2927: f64 = (assign4350_e2925 * var_t1);
        let assign4350_e2928: f64 = (assign4350_e2922 + assign4350_e2927);
        let assign4350_e2932: f64 = (1.0 - var_t1);
        let assign4350_e2933: f64 = (var_mks_vtmp * assign4350_e2932);
        let assign4350_e2934: f64 = (assign4350_e2928 - assign4350_e2933);
        let assign4350_e2935: f64 = (assign4350_e2917 / assign4350_e2934);
        var_vmaxe = assign4350_e2935;
        var_vmaxe_dn0 = ((((var_vmax0_dn0 * var_mks_vmax) * assign4350_e2934) - (assign4350_e2917 * (((0.4 * var_t1_dn0) + (((0.1 * var_t1_dn0) * var_t1) + (assign4350_e2925 * var_t1_dn0))) - (var_mks_vtmp * (-var_t1_dn0))))) / (assign4350_e2934 * assign4350_e2934));
        var_vmaxe_dn2 = ((((var_vmax0_dn2 * var_mks_vmax) * assign4350_e2934) - (assign4350_e2917 * (((0.4 * var_t1_dn2) + (((0.1 * var_t1_dn2) * var_t1) + (assign4350_e2925 * var_t1_dn2))) - (var_mks_vtmp * (-var_t1_dn2))))) / (assign4350_e2934 * assign4350_e2934));
        var_vmaxe_dn6 = ((((var_vmax0_dn6 * var_mks_vmax) * assign4350_e2934) - (assign4350_e2917 * (((0.4 * var_t1_dn6) + (((0.1 * var_t1_dn6) * var_t1) + (assign4350_e2925 * var_t1_dn6))) - (var_mks_vtmp * (-var_t1_dn6))))) / (assign4350_e2934 * assign4350_e2934));
        var_vmaxe_dn7 = ((((var_vmax0_dn7 * var_mks_vmax) * assign4350_e2934) - (assign4350_e2917 * (((0.4 * var_t1_dn7) + (((0.1 * var_t1_dn7) * var_t1) + (assign4350_e2925 * var_t1_dn7))) - (var_mks_vtmp * (-var_t1_dn7))))) / (assign4350_e2934 * assign4350_e2934));
        var_vmaxe_dn10 = ((((var_vmax0_dn10 * var_mks_vmax) * assign4350_e2934) - (assign4350_e2917 * (((0.4 * var_t1_dn10) + (((0.1 * var_t1_dn10) * var_t1) + (assign4350_e2925 * var_t1_dn10))) - (var_mks_vtmp * (-var_t1_dn10))))) / (assign4350_e2934 * assign4350_e2934));
        var_vmaxe_dn11 = ((((var_vmax0_dn11 * var_mks_vmax) * assign4350_e2934) - (assign4350_e2917 * (((0.4 * var_t1_dn11) + (((0.1 * var_t1_dn11) * var_t1) + (assign4350_e2925 * var_t1_dn11))) - (var_mks_vtmp * (-var_t1_dn11))))) / (assign4350_e2934 * assign4350_e2934));
        var_vmaxe_dn12 = ((((var_vmax0_dn12 * var_mks_vmax) * assign4350_e2934) - (assign4350_e2917 * (((0.4 * var_t1_dn12) + (((0.1 * var_t1_dn12) * var_t1) + (assign4350_e2925 * var_t1_dn12))) - (var_mks_vtmp * (-var_t1_dn12))))) / (assign4350_e2934 * assign4350_e2934));
        var_vmaxe_dn17 = ((((var_vmax0_dn17 * var_mks_vmax) * assign4350_e2934) - (assign4350_e2917 * (((0.4 * var_t1_dn17) + (((0.1 * var_t1_dn17) * var_t1) + (assign4350_e2925 * var_t1_dn17))) - (var_mks_vtmp * (-var_t1_dn17))))) / (assign4350_e2934 * assign4350_e2934));
        var_vmaxe_rv = 0.0;

        let assign4360_e2937: f64 = (var_eg).sqrt();
        var_egp12 = assign4360_e2937;
        var_egp12_dn0 = (var_eg_dn0 / (2.0 * assign4360_e2937));
        var_egp12_dn2 = (var_eg_dn2 / (2.0 * assign4360_e2937));
        var_egp12_dn6 = (var_eg_dn6 / (2.0 * assign4360_e2937));
        var_egp12_dn7 = (var_eg_dn7 / (2.0 * assign4360_e2937));
        var_egp12_dn10 = (var_eg_dn10 / (2.0 * assign4360_e2937));
        var_egp12_dn11 = (var_eg_dn11 / (2.0 * assign4360_e2937));
        var_egp12_dn12 = (var_eg_dn12 / (2.0 * assign4360_e2937));
        var_egp12_dn17 = (var_eg_dn17 / (2.0 * assign4360_e2937));
        var_egp12_rv = 0.0;

        let assign4370_e2940: f64 = (var_eg * var_egp12);
        var_egp32 = assign4370_e2940;
        var_egp32_dn0 = ((var_eg_dn0 * var_egp12) + (var_eg * var_egp12_dn0));
        var_egp32_dn2 = ((var_eg_dn2 * var_egp12) + (var_eg * var_egp12_dn2));
        var_egp32_dn6 = ((var_eg_dn6 * var_egp12) + (var_eg * var_egp12_dn6));
        var_egp32_dn7 = ((var_eg_dn7 * var_egp12) + (var_eg * var_egp12_dn7));
        var_egp32_dn10 = ((var_eg_dn10 * var_egp12) + (var_eg * var_egp12_dn10));
        var_egp32_dn11 = ((var_eg_dn11 * var_egp12) + (var_eg * var_egp12_dn11));
        var_egp32_dn12 = ((var_eg_dn12 * var_egp12) + (var_eg * var_egp12_dn12));
        var_egp32_dn17 = ((var_eg_dn17 * var_egp12) + (var_eg * var_egp12_dn17));
        var_egp32_rv = 0.0;

        *var_beta_slot = var_beta;
        *var_beta2_slot = var_beta2;
        *var_beta2_dn10_slot = var_beta2_dn10;
        *var_beta2_rv_slot = var_beta2_rv;
        *var_beta_dn10_slot = var_beta_dn10;
        *var_beta_inv_slot = var_beta_inv;
        *var_beta_inv_dn10_slot = var_beta_inv_dn10;
        *var_beta_inv_rv_slot = var_beta_inv_rv;
        *var_beta_rv_slot = var_beta_rv;
        *var_cgs_mphn0_slot = var_cgs_mphn0;
        *var_cgs_mphn0_dn10_slot = var_cgs_mphn0_dn10;
        *var_cgs_mphn0_rv_slot = var_cgs_mphn0_rv;
        *var_cgs_mueph_slot = var_cgs_mueph;
        *var_cgs_mueph_rv_slot = var_cgs_mueph_rv;
        *var_cgs_wmueph_slot = var_cgs_wmueph;
        *var_cgs_wmueph_rv_slot = var_cgs_wmueph_rv;
        *var_deltemp_slot = var_deltemp;
        *var_deltemp_dn10_slot = var_deltemp_dn10;
        *var_deltemp_rv_slot = var_deltemp_rv;
        *var_eg_slot = var_eg;
        *var_eg_dn0_slot = var_eg_dn0;
        *var_eg_dn10_slot = var_eg_dn10;
        *var_eg_dn11_slot = var_eg_dn11;
        *var_eg_dn12_slot = var_eg_dn12;
        *var_eg_dn17_slot = var_eg_dn17;
        *var_eg_dn2_slot = var_eg_dn2;
        *var_eg_dn6_slot = var_eg_dn6;
        *var_eg_dn7_slot = var_eg_dn7;
        *var_eg_rv_slot = var_eg_rv;
        *var_egp12_slot = var_egp12;
        *var_egp12_dn0_slot = var_egp12_dn0;
        *var_egp12_dn10_slot = var_egp12_dn10;
        *var_egp12_dn11_slot = var_egp12_dn11;
        *var_egp12_dn12_slot = var_egp12_dn12;
        *var_egp12_dn17_slot = var_egp12_dn17;
        *var_egp12_dn2_slot = var_egp12_dn2;
        *var_egp12_dn6_slot = var_egp12_dn6;
        *var_egp12_dn7_slot = var_egp12_dn7;
        *var_egp12_rv_slot = var_egp12_rv;
        *var_egp32_slot = var_egp32;
        *var_egp32_dn0_slot = var_egp32_dn0;
        *var_egp32_dn10_slot = var_egp32_dn10;
        *var_egp32_dn11_slot = var_egp32_dn11;
        *var_egp32_dn12_slot = var_egp32_dn12;
        *var_egp32_dn17_slot = var_egp32_dn17;
        *var_egp32_dn2_slot = var_egp32_dn2;
        *var_egp32_dn6_slot = var_egp32_dn6;
        *var_egp32_dn7_slot = var_egp32_dn7;
        *var_egp32_rv_slot = var_egp32_rv;
        *var_guard36_slot = var_guard36;
        *var_guard36_rv_slot = var_guard36_rv;
        *var_mode_slot = var_mode;
        *var_mode_rv_slot = var_mode_rv;
        *var_modenml_slot = var_modenml;
        *var_modenml_rv_slot = var_modenml_rv;
        *var_modervs_slot = var_modervs;
        *var_modervs_rv_slot = var_modervs_rv;
        *var_ptovr_slot = var_ptovr;
        *var_ptovr_dn0_slot = var_ptovr_dn0;
        *var_ptovr_dn10_slot = var_ptovr_dn10;
        *var_ptovr_dn11_slot = var_ptovr_dn11;
        *var_ptovr_dn12_slot = var_ptovr_dn12;
        *var_ptovr_dn17_slot = var_ptovr_dn17;
        *var_ptovr_dn2_slot = var_ptovr_dn2;
        *var_ptovr_dn6_slot = var_ptovr_dn6;
        *var_ptovr_dn7_slot = var_ptovr_dn7;
        *var_ptovr_rv_slot = var_ptovr_rv;
        *var_qb_nqs_slot = var_qb_nqs;
        *var_qb_nqs_dn13_slot = var_qb_nqs_dn13;
        *var_qb_nqs_rv_slot = var_qb_nqs_rv;
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
        *var_qd_nqs_rv_slot = var_qd_nqs_rv;
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
        *var_qs_nqs_rv_slot = var_qs_nqs_rv;
        *var_t1_slot = var_t1;
        *var_t1__blk41_slot = var_t1__blk41;
        *var_t1__blk41_dn10_slot = var_t1__blk41_dn10;
        *var_t1__blk41_rv_slot = var_t1__blk41_rv;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2__blk42_slot = var_t2__blk42;
        *var_t2__blk42_rv_slot = var_t2__blk42_rv;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3__blk43_slot = var_t3__blk43;
        *var_t3__blk43_rv_slot = var_t3__blk43_rv;
        *var_ttemp_slot = var_ttemp;
        *var_ttemp_dn10_slot = var_ttemp_dn10;
        *var_ttemp_rv_slot = var_ttemp_rv;
        *var_vbs_slot = var_vbs;
        *var_vbs_dn0_slot = var_vbs_dn0;
        *var_vbs_dn10_slot = var_vbs_dn10;
        *var_vbs_dn11_slot = var_vbs_dn11;
        *var_vbs_dn12_slot = var_vbs_dn12;
        *var_vbs_dn17_slot = var_vbs_dn17;
        *var_vbs_dn2_slot = var_vbs_dn2;
        *var_vbs_dn6_slot = var_vbs_dn6;
        *var_vbs_dn7_slot = var_vbs_dn7;
        *var_vbs_rv_slot = var_vbs_rv;
        *var_vds_slot = var_vds;
        *var_vds_dn0_slot = var_vds_dn0;
        *var_vds_dn10_slot = var_vds_dn10;
        *var_vds_dn11_slot = var_vds_dn11;
        *var_vds_dn12_slot = var_vds_dn12;
        *var_vds_dn17_slot = var_vds_dn17;
        *var_vds_dn2_slot = var_vds_dn2;
        *var_vds_dn6_slot = var_vds_dn6;
        *var_vds_dn7_slot = var_vds_dn7;
        *var_vds_rv_slot = var_vds_rv;
        *var_vgs_slot = var_vgs;
        *var_vgs_dn11_slot = var_vgs_dn11;
        *var_vgs_dn6_slot = var_vgs_dn6;
        *var_vgs_dn7_slot = var_vgs_dn7;
        *var_vgs_rv_slot = var_vgs_rv;
        *var_vmaxe_slot = var_vmaxe;
        *var_vmaxe_dn0_slot = var_vmaxe_dn0;
        *var_vmaxe_dn10_slot = var_vmaxe_dn10;
        *var_vmaxe_dn11_slot = var_vmaxe_dn11;
        *var_vmaxe_dn12_slot = var_vmaxe_dn12;
        *var_vmaxe_dn17_slot = var_vmaxe_dn17;
        *var_vmaxe_dn2_slot = var_vmaxe_dn2;
        *var_vmaxe_dn6_slot = var_vmaxe_dn6;
        *var_vmaxe_dn7_slot = var_vmaxe_dn7;
        *var_vmaxe_rv_slot = var_vmaxe_rv;
    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn10: f64,
        var_beta_inv: f64,
        var_beta_inv_dn10: f64,
        var_betatnom: f64,
        var_c0bulk: f64,
        var_costi00: f64,
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
        var_lgate: f64,
        var_mks_nsubb: f64,
        var_nsti_p2: f64,
        var_nsub: f64,
        var_nsub_dn0: f64,
        var_nsub_dn10: f64,
        var_nsub_dn11: f64,
        var_nsub_dn12: f64,
        var_nsub_dn17: f64,
        var_nsub_dn2: f64,
        var_nsub_dn6: f64,
        var_nsub_dn7: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn11: f64,
        var_q_nsub_dn12: f64,
        var_q_nsub_dn17: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn7: f64,
        var_subversion: f64,
        var_ttemp: f64,
        var_ttemp_dn10: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn17: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn7: f64,
        var_uc_tnom: f64,
        var_vbs: f64,
        var_vbs_dn0: f64,
        var_vbs_dn10: f64,
        var_vbs_dn11: f64,
        var_vbs_dn12: f64,
        var_vbs_dn17: f64,
        var_vbs_dn2: f64,
        var_vbs_dn6: f64,
        var_vbs_dn7: f64,
        var_c_w_soi_slot: &mut f64,
        var_c_w_soi_dn0_slot: &mut f64,
        var_c_w_soi_dn10_slot: &mut f64,
        var_c_w_soi_dn11_slot: &mut f64,
        var_c_w_soi_dn12_slot: &mut f64,
        var_c_w_soi_dn17_slot: &mut f64,
        var_c_w_soi_dn2_slot: &mut f64,
        var_c_w_soi_dn6_slot: &mut f64,
        var_c_w_soi_dn7_slot: &mut f64,
        var_c_w_soi_rv_slot: &mut f64,
        var_cnst0bulk_slot: &mut f64,
        var_cnst0bulk_dn10_slot: &mut f64,
        var_cnst0bulk_rv_slot: &mut f64,
        var_cnst0soi_slot: &mut f64,
        var_cnst0soi_dn0_slot: &mut f64,
        var_cnst0soi_dn10_slot: &mut f64,
        var_cnst0soi_dn11_slot: &mut f64,
        var_cnst0soi_dn12_slot: &mut f64,
        var_cnst0soi_dn17_slot: &mut f64,
        var_cnst0soi_dn2_slot: &mut f64,
        var_cnst0soi_dn6_slot: &mut f64,
        var_cnst0soi_dn7_slot: &mut f64,
        var_cnst0soi_rv_slot: &mut f64,
        var_cnst1bulk_slot: &mut f64,
        var_cnst1bulk_dn0_slot: &mut f64,
        var_cnst1bulk_dn10_slot: &mut f64,
        var_cnst1bulk_dn11_slot: &mut f64,
        var_cnst1bulk_dn12_slot: &mut f64,
        var_cnst1bulk_dn17_slot: &mut f64,
        var_cnst1bulk_dn2_slot: &mut f64,
        var_cnst1bulk_dn6_slot: &mut f64,
        var_cnst1bulk_dn7_slot: &mut f64,
        var_cnst1bulk_rv_slot: &mut f64,
        var_cnst1soi_slot: &mut f64,
        var_cnst1soi_dn0_slot: &mut f64,
        var_cnst1soi_dn10_slot: &mut f64,
        var_cnst1soi_dn11_slot: &mut f64,
        var_cnst1soi_dn12_slot: &mut f64,
        var_cnst1soi_dn17_slot: &mut f64,
        var_cnst1soi_dn2_slot: &mut f64,
        var_cnst1soi_dn6_slot: &mut f64,
        var_cnst1soi_dn7_slot: &mut f64,
        var_cnst1soi_rv_slot: &mut f64,
        var_cnst_2esi_q_nsubs_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn0_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn10_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn11_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn12_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn17_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn2_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn6_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn7_slot: &mut f64,
        var_cnst_2esi_q_nsubs_rv_slot: &mut f64,
        var_costi0_slot: &mut f64,
        var_costi0_dn0_slot: &mut f64,
        var_costi0_dn10_slot: &mut f64,
        var_costi0_dn11_slot: &mut f64,
        var_costi0_dn12_slot: &mut f64,
        var_costi0_dn17_slot: &mut f64,
        var_costi0_dn2_slot: &mut f64,
        var_costi0_dn6_slot: &mut f64,
        var_costi0_dn7_slot: &mut f64,
        var_costi0_p2_slot: &mut f64,
        var_costi0_p2_dn0_slot: &mut f64,
        var_costi0_p2_dn10_slot: &mut f64,
        var_costi0_p2_dn11_slot: &mut f64,
        var_costi0_p2_dn12_slot: &mut f64,
        var_costi0_p2_dn17_slot: &mut f64,
        var_costi0_p2_dn2_slot: &mut f64,
        var_costi0_p2_dn6_slot: &mut f64,
        var_costi0_p2_dn7_slot: &mut f64,
        var_costi0_p2_rv_slot: &mut f64,
        var_costi0_rv_slot: &mut f64,
        var_costi1_slot: &mut f64,
        var_costi1_dn0_slot: &mut f64,
        var_costi1_dn10_slot: &mut f64,
        var_costi1_dn11_slot: &mut f64,
        var_costi1_dn12_slot: &mut f64,
        var_costi1_dn17_slot: &mut f64,
        var_costi1_dn2_slot: &mut f64,
        var_costi1_dn6_slot: &mut f64,
        var_costi1_dn7_slot: &mut f64,
        var_costi1_rv_slot: &mut f64,
        var_guard44_slot: &mut f64,
        var_guard44_rv_slot: &mut f64,
        var_guard45_slot: &mut f64,
        var_guard45_rv_slot: &mut f64,
        var_guard50_slot: &mut f64,
        var_guard50_rv_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard51_rv_slot: &mut f64,
        var_guard52_slot: &mut f64,
        var_guard52_rv_slot: &mut f64,
        var_lch_slot: &mut f64,
        var_lch_dn0_slot: &mut f64,
        var_lch_dn10_slot: &mut f64,
        var_lch_dn11_slot: &mut f64,
        var_lch_dn12_slot: &mut f64,
        var_lch_dn17_slot: &mut f64,
        var_lch_dn2_slot: &mut f64,
        var_lch_dn6_slot: &mut f64,
        var_lch_dn7_slot: &mut f64,
        var_lch_rv_slot: &mut f64,
        var_ldby_slot: &mut f64,
        var_ldby_dn0_slot: &mut f64,
        var_ldby_dn10_slot: &mut f64,
        var_ldby_dn11_slot: &mut f64,
        var_ldby_dn12_slot: &mut f64,
        var_ldby_dn17_slot: &mut f64,
        var_ldby_dn2_slot: &mut f64,
        var_ldby_dn6_slot: &mut f64,
        var_ldby_dn7_slot: &mut f64,
        var_ldby_rv_slot: &mut f64,
        var_nin_slot: &mut f64,
        var_nin_dn0_slot: &mut f64,
        var_nin_dn10_slot: &mut f64,
        var_nin_dn11_slot: &mut f64,
        var_nin_dn12_slot: &mut f64,
        var_nin_dn17_slot: &mut f64,
        var_nin_dn2_slot: &mut f64,
        var_nin_dn6_slot: &mut f64,
        var_nin_dn7_slot: &mut f64,
        var_nin_rv_slot: &mut f64,
        var_pb2_slot: &mut f64,
        var_pb2_dn0_slot: &mut f64,
        var_pb2_dn10_slot: &mut f64,
        var_pb2_dn11_slot: &mut f64,
        var_pb2_dn12_slot: &mut f64,
        var_pb2_dn17_slot: &mut f64,
        var_pb2_dn2_slot: &mut f64,
        var_pb2_dn6_slot: &mut f64,
        var_pb2_dn7_slot: &mut f64,
        var_pb2_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_vbs_bnd_slot: &mut f64,
        var_vbs_bnd_rv_slot: &mut f64,
        var_vbs_max_slot: &mut f64,
        var_vbs_max_rv_slot: &mut f64,
        var_wdsoi_ini_slot: &mut f64,
        var_wdsoi_ini_dn0_slot: &mut f64,
        var_wdsoi_ini_dn10_slot: &mut f64,
        var_wdsoi_ini_dn11_slot: &mut f64,
        var_wdsoi_ini_dn12_slot: &mut f64,
        var_wdsoi_ini_dn17_slot: &mut f64,
        var_wdsoi_ini_dn2_slot: &mut f64,
        var_wdsoi_ini_dn6_slot: &mut f64,
        var_wdsoi_ini_dn7_slot: &mut f64,
        var_wdsoi_ini_rv_slot: &mut f64,
        var_x2_slot: &mut f64,
        var_x2_dn0_slot: &mut f64,
        var_x2_dn10_slot: &mut f64,
        var_x2_dn11_slot: &mut f64,
        var_x2_dn12_slot: &mut f64,
        var_x2_dn17_slot: &mut f64,
        var_x2_dn2_slot: &mut f64,
        var_x2_dn6_slot: &mut f64,
        var_x2_dn7_slot: &mut f64,
        var_x2_rv_slot: &mut f64,
        var_xmax2_slot: &mut f64,
        var_xmax2_dn0_slot: &mut f64,
        var_xmax2_dn10_slot: &mut f64,
        var_xmax2_dn11_slot: &mut f64,
        var_xmax2_dn12_slot: &mut f64,
        var_xmax2_dn17_slot: &mut f64,
        var_xmax2_dn2_slot: &mut f64,
        var_xmax2_dn6_slot: &mut f64,
        var_xmax2_dn7_slot: &mut f64,
        var_xmax2_rv_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn11_slot: &mut f64,
        var_xmp_dn12_slot: &mut f64,
        var_xmp_dn17_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn7_slot: &mut f64,
        var_xmp_rv_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn11_slot: &mut f64,
        var_xp_dn12_slot: &mut f64,
        var_xp_dn17_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
        var_xp_rv_slot: &mut f64,
    ) {
        let mut var_c_w_soi: f64 = *var_c_w_soi_slot;
        let mut var_c_w_soi_dn0: f64 = *var_c_w_soi_dn0_slot;
        let mut var_c_w_soi_dn10: f64 = *var_c_w_soi_dn10_slot;
        let mut var_c_w_soi_dn11: f64 = *var_c_w_soi_dn11_slot;
        let mut var_c_w_soi_dn12: f64 = *var_c_w_soi_dn12_slot;
        let mut var_c_w_soi_dn17: f64 = *var_c_w_soi_dn17_slot;
        let mut var_c_w_soi_dn2: f64 = *var_c_w_soi_dn2_slot;
        let mut var_c_w_soi_dn6: f64 = *var_c_w_soi_dn6_slot;
        let mut var_c_w_soi_dn7: f64 = *var_c_w_soi_dn7_slot;
        let mut var_c_w_soi_rv: f64 = *var_c_w_soi_rv_slot;
        let mut var_cnst0bulk: f64 = *var_cnst0bulk_slot;
        let mut var_cnst0bulk_dn10: f64 = *var_cnst0bulk_dn10_slot;
        let mut var_cnst0bulk_rv: f64 = *var_cnst0bulk_rv_slot;
        let mut var_cnst0soi: f64 = *var_cnst0soi_slot;
        let mut var_cnst0soi_dn0: f64 = *var_cnst0soi_dn0_slot;
        let mut var_cnst0soi_dn10: f64 = *var_cnst0soi_dn10_slot;
        let mut var_cnst0soi_dn11: f64 = *var_cnst0soi_dn11_slot;
        let mut var_cnst0soi_dn12: f64 = *var_cnst0soi_dn12_slot;
        let mut var_cnst0soi_dn17: f64 = *var_cnst0soi_dn17_slot;
        let mut var_cnst0soi_dn2: f64 = *var_cnst0soi_dn2_slot;
        let mut var_cnst0soi_dn6: f64 = *var_cnst0soi_dn6_slot;
        let mut var_cnst0soi_dn7: f64 = *var_cnst0soi_dn7_slot;
        let mut var_cnst0soi_rv: f64 = *var_cnst0soi_rv_slot;
        let mut var_cnst1bulk: f64 = *var_cnst1bulk_slot;
        let mut var_cnst1bulk_dn0: f64 = *var_cnst1bulk_dn0_slot;
        let mut var_cnst1bulk_dn10: f64 = *var_cnst1bulk_dn10_slot;
        let mut var_cnst1bulk_dn11: f64 = *var_cnst1bulk_dn11_slot;
        let mut var_cnst1bulk_dn12: f64 = *var_cnst1bulk_dn12_slot;
        let mut var_cnst1bulk_dn17: f64 = *var_cnst1bulk_dn17_slot;
        let mut var_cnst1bulk_dn2: f64 = *var_cnst1bulk_dn2_slot;
        let mut var_cnst1bulk_dn6: f64 = *var_cnst1bulk_dn6_slot;
        let mut var_cnst1bulk_dn7: f64 = *var_cnst1bulk_dn7_slot;
        let mut var_cnst1bulk_rv: f64 = *var_cnst1bulk_rv_slot;
        let mut var_cnst1soi: f64 = *var_cnst1soi_slot;
        let mut var_cnst1soi_dn0: f64 = *var_cnst1soi_dn0_slot;
        let mut var_cnst1soi_dn10: f64 = *var_cnst1soi_dn10_slot;
        let mut var_cnst1soi_dn11: f64 = *var_cnst1soi_dn11_slot;
        let mut var_cnst1soi_dn12: f64 = *var_cnst1soi_dn12_slot;
        let mut var_cnst1soi_dn17: f64 = *var_cnst1soi_dn17_slot;
        let mut var_cnst1soi_dn2: f64 = *var_cnst1soi_dn2_slot;
        let mut var_cnst1soi_dn6: f64 = *var_cnst1soi_dn6_slot;
        let mut var_cnst1soi_dn7: f64 = *var_cnst1soi_dn7_slot;
        let mut var_cnst1soi_rv: f64 = *var_cnst1soi_rv_slot;
        let mut var_cnst_2esi_q_nsubs: f64 = *var_cnst_2esi_q_nsubs_slot;
        let mut var_cnst_2esi_q_nsubs_dn0: f64 = *var_cnst_2esi_q_nsubs_dn0_slot;
        let mut var_cnst_2esi_q_nsubs_dn10: f64 = *var_cnst_2esi_q_nsubs_dn10_slot;
        let mut var_cnst_2esi_q_nsubs_dn11: f64 = *var_cnst_2esi_q_nsubs_dn11_slot;
        let mut var_cnst_2esi_q_nsubs_dn12: f64 = *var_cnst_2esi_q_nsubs_dn12_slot;
        let mut var_cnst_2esi_q_nsubs_dn17: f64 = *var_cnst_2esi_q_nsubs_dn17_slot;
        let mut var_cnst_2esi_q_nsubs_dn2: f64 = *var_cnst_2esi_q_nsubs_dn2_slot;
        let mut var_cnst_2esi_q_nsubs_dn6: f64 = *var_cnst_2esi_q_nsubs_dn6_slot;
        let mut var_cnst_2esi_q_nsubs_dn7: f64 = *var_cnst_2esi_q_nsubs_dn7_slot;
        let mut var_cnst_2esi_q_nsubs_rv: f64 = *var_cnst_2esi_q_nsubs_rv_slot;
        let mut var_costi0: f64 = *var_costi0_slot;
        let mut var_costi0_dn0: f64 = *var_costi0_dn0_slot;
        let mut var_costi0_dn10: f64 = *var_costi0_dn10_slot;
        let mut var_costi0_dn11: f64 = *var_costi0_dn11_slot;
        let mut var_costi0_dn12: f64 = *var_costi0_dn12_slot;
        let mut var_costi0_dn17: f64 = *var_costi0_dn17_slot;
        let mut var_costi0_dn2: f64 = *var_costi0_dn2_slot;
        let mut var_costi0_dn6: f64 = *var_costi0_dn6_slot;
        let mut var_costi0_dn7: f64 = *var_costi0_dn7_slot;
        let mut var_costi0_p2: f64 = *var_costi0_p2_slot;
        let mut var_costi0_p2_dn0: f64 = *var_costi0_p2_dn0_slot;
        let mut var_costi0_p2_dn10: f64 = *var_costi0_p2_dn10_slot;
        let mut var_costi0_p2_dn11: f64 = *var_costi0_p2_dn11_slot;
        let mut var_costi0_p2_dn12: f64 = *var_costi0_p2_dn12_slot;
        let mut var_costi0_p2_dn17: f64 = *var_costi0_p2_dn17_slot;
        let mut var_costi0_p2_dn2: f64 = *var_costi0_p2_dn2_slot;
        let mut var_costi0_p2_dn6: f64 = *var_costi0_p2_dn6_slot;
        let mut var_costi0_p2_dn7: f64 = *var_costi0_p2_dn7_slot;
        let mut var_costi0_p2_rv: f64 = *var_costi0_p2_rv_slot;
        let mut var_costi0_rv: f64 = *var_costi0_rv_slot;
        let mut var_costi1: f64 = *var_costi1_slot;
        let mut var_costi1_dn0: f64 = *var_costi1_dn0_slot;
        let mut var_costi1_dn10: f64 = *var_costi1_dn10_slot;
        let mut var_costi1_dn11: f64 = *var_costi1_dn11_slot;
        let mut var_costi1_dn12: f64 = *var_costi1_dn12_slot;
        let mut var_costi1_dn17: f64 = *var_costi1_dn17_slot;
        let mut var_costi1_dn2: f64 = *var_costi1_dn2_slot;
        let mut var_costi1_dn6: f64 = *var_costi1_dn6_slot;
        let mut var_costi1_dn7: f64 = *var_costi1_dn7_slot;
        let mut var_costi1_rv: f64 = *var_costi1_rv_slot;
        let mut var_guard44: f64 = *var_guard44_slot;
        let mut var_guard44_rv: f64 = *var_guard44_rv_slot;
        let mut var_guard45: f64 = *var_guard45_slot;
        let mut var_guard45_rv: f64 = *var_guard45_rv_slot;
        let mut var_guard50: f64 = *var_guard50_slot;
        let mut var_guard50_rv: f64 = *var_guard50_rv_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard51_rv: f64 = *var_guard51_rv_slot;
        let mut var_guard52: f64 = *var_guard52_slot;
        let mut var_guard52_rv: f64 = *var_guard52_rv_slot;
        let mut var_lch: f64 = *var_lch_slot;
        let mut var_lch_dn0: f64 = *var_lch_dn0_slot;
        let mut var_lch_dn10: f64 = *var_lch_dn10_slot;
        let mut var_lch_dn11: f64 = *var_lch_dn11_slot;
        let mut var_lch_dn12: f64 = *var_lch_dn12_slot;
        let mut var_lch_dn17: f64 = *var_lch_dn17_slot;
        let mut var_lch_dn2: f64 = *var_lch_dn2_slot;
        let mut var_lch_dn6: f64 = *var_lch_dn6_slot;
        let mut var_lch_dn7: f64 = *var_lch_dn7_slot;
        let mut var_lch_rv: f64 = *var_lch_rv_slot;
        let mut var_ldby: f64 = *var_ldby_slot;
        let mut var_ldby_dn0: f64 = *var_ldby_dn0_slot;
        let mut var_ldby_dn10: f64 = *var_ldby_dn10_slot;
        let mut var_ldby_dn11: f64 = *var_ldby_dn11_slot;
        let mut var_ldby_dn12: f64 = *var_ldby_dn12_slot;
        let mut var_ldby_dn17: f64 = *var_ldby_dn17_slot;
        let mut var_ldby_dn2: f64 = *var_ldby_dn2_slot;
        let mut var_ldby_dn6: f64 = *var_ldby_dn6_slot;
        let mut var_ldby_dn7: f64 = *var_ldby_dn7_slot;
        let mut var_ldby_rv: f64 = *var_ldby_rv_slot;
        let mut var_nin: f64 = *var_nin_slot;
        let mut var_nin_dn0: f64 = *var_nin_dn0_slot;
        let mut var_nin_dn10: f64 = *var_nin_dn10_slot;
        let mut var_nin_dn11: f64 = *var_nin_dn11_slot;
        let mut var_nin_dn12: f64 = *var_nin_dn12_slot;
        let mut var_nin_dn17: f64 = *var_nin_dn17_slot;
        let mut var_nin_dn2: f64 = *var_nin_dn2_slot;
        let mut var_nin_dn6: f64 = *var_nin_dn6_slot;
        let mut var_nin_dn7: f64 = *var_nin_dn7_slot;
        let mut var_nin_rv: f64 = *var_nin_rv_slot;
        let mut var_pb2: f64 = *var_pb2_slot;
        let mut var_pb2_dn0: f64 = *var_pb2_dn0_slot;
        let mut var_pb2_dn10: f64 = *var_pb2_dn10_slot;
        let mut var_pb2_dn11: f64 = *var_pb2_dn11_slot;
        let mut var_pb2_dn12: f64 = *var_pb2_dn12_slot;
        let mut var_pb2_dn17: f64 = *var_pb2_dn17_slot;
        let mut var_pb2_dn2: f64 = *var_pb2_dn2_slot;
        let mut var_pb2_dn6: f64 = *var_pb2_dn6_slot;
        let mut var_pb2_dn7: f64 = *var_pb2_dn7_slot;
        let mut var_pb2_rv: f64 = *var_pb2_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_vbs_bnd: f64 = *var_vbs_bnd_slot;
        let mut var_vbs_bnd_rv: f64 = *var_vbs_bnd_rv_slot;
        let mut var_vbs_max: f64 = *var_vbs_max_slot;
        let mut var_vbs_max_rv: f64 = *var_vbs_max_rv_slot;
        let mut var_wdsoi_ini: f64 = *var_wdsoi_ini_slot;
        let mut var_wdsoi_ini_dn0: f64 = *var_wdsoi_ini_dn0_slot;
        let mut var_wdsoi_ini_dn10: f64 = *var_wdsoi_ini_dn10_slot;
        let mut var_wdsoi_ini_dn11: f64 = *var_wdsoi_ini_dn11_slot;
        let mut var_wdsoi_ini_dn12: f64 = *var_wdsoi_ini_dn12_slot;
        let mut var_wdsoi_ini_dn17: f64 = *var_wdsoi_ini_dn17_slot;
        let mut var_wdsoi_ini_dn2: f64 = *var_wdsoi_ini_dn2_slot;
        let mut var_wdsoi_ini_dn6: f64 = *var_wdsoi_ini_dn6_slot;
        let mut var_wdsoi_ini_dn7: f64 = *var_wdsoi_ini_dn7_slot;
        let mut var_wdsoi_ini_rv: f64 = *var_wdsoi_ini_rv_slot;
        let mut var_x2: f64 = *var_x2_slot;
        let mut var_x2_dn0: f64 = *var_x2_dn0_slot;
        let mut var_x2_dn10: f64 = *var_x2_dn10_slot;
        let mut var_x2_dn11: f64 = *var_x2_dn11_slot;
        let mut var_x2_dn12: f64 = *var_x2_dn12_slot;
        let mut var_x2_dn17: f64 = *var_x2_dn17_slot;
        let mut var_x2_dn2: f64 = *var_x2_dn2_slot;
        let mut var_x2_dn6: f64 = *var_x2_dn6_slot;
        let mut var_x2_dn7: f64 = *var_x2_dn7_slot;
        let mut var_x2_rv: f64 = *var_x2_rv_slot;
        let mut var_xmax2: f64 = *var_xmax2_slot;
        let mut var_xmax2_dn0: f64 = *var_xmax2_dn0_slot;
        let mut var_xmax2_dn10: f64 = *var_xmax2_dn10_slot;
        let mut var_xmax2_dn11: f64 = *var_xmax2_dn11_slot;
        let mut var_xmax2_dn12: f64 = *var_xmax2_dn12_slot;
        let mut var_xmax2_dn17: f64 = *var_xmax2_dn17_slot;
        let mut var_xmax2_dn2: f64 = *var_xmax2_dn2_slot;
        let mut var_xmax2_dn6: f64 = *var_xmax2_dn6_slot;
        let mut var_xmax2_dn7: f64 = *var_xmax2_dn7_slot;
        let mut var_xmax2_rv: f64 = *var_xmax2_rv_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn11: f64 = *var_xmp_dn11_slot;
        let mut var_xmp_dn12: f64 = *var_xmp_dn12_slot;
        let mut var_xmp_dn17: f64 = *var_xmp_dn17_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn7: f64 = *var_xmp_dn7_slot;
        let mut var_xmp_rv: f64 = *var_xmp_rv_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn11: f64 = *var_xp_dn11_slot;
        let mut var_xp_dn12: f64 = *var_xp_dn12_slot;
        let mut var_xp_dn17: f64 = *var_xp_dn17_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_rv: f64 = *var_xp_rv_slot;

        let assign4380_e2943: f64 = (10400000000.0 / 1e-6);
        let assign4380_e2946: f64 = (var_ttemp / var_uc_tnom);
        let assign4380_e2948: f64 = (assign4380_e2946).powf(1.5);
        let assign4380_e2949: f64 = (assign4380_e2943 * assign4380_e2948);
        let assign4380_e2951: f64 = (-var_eg);
        let assign4380_e2953: f64 = (assign4380_e2951 / 2.0);
        let assign4380_e2955: f64 = (assign4380_e2953 * var_beta);
        let assign4380_e2958: f64 = (var_egtnom / 2.0);
        let assign4380_e2960: f64 = (assign4380_e2958 * var_betatnom);
        let assign4380_e2961: f64 = (assign4380_e2955 + assign4380_e2960);
        let assign4380_e2962: f64 = (assign4380_e2961).exp();
        let assign4380_e2963: f64 = (assign4380_e2949 * assign4380_e2962);
        var_nin = assign4380_e2963;
        var_nin_dn0 = (assign4380_e2949 * (assign4380_e2962 * (((-var_eg_dn0) / 2.0) * var_beta)));
        var_nin_dn2 = (assign4380_e2949 * (assign4380_e2962 * (((-var_eg_dn2) / 2.0) * var_beta)));
        var_nin_dn6 = (assign4380_e2949 * (assign4380_e2962 * (((-var_eg_dn6) / 2.0) * var_beta)));
        var_nin_dn7 = (assign4380_e2949 * (assign4380_e2962 * (((-var_eg_dn7) / 2.0) * var_beta)));
        var_nin_dn10 = (((assign4380_e2943 * if 0.0 == 0.0 && ((1.5) as f64).is_finite() && ((1.5) as f64).fract() == 0.0 { if 1.5 == 0.0 { 0.0 } else { (1.5 * ((assign4380_e2946).powf(1.5 - 1.0) * (var_ttemp_dn10 / var_uc_tnom))) } } else { (assign4380_e2948 * (1.5 * ((var_ttemp_dn10 / var_uc_tnom) / assign4380_e2946))) }) * assign4380_e2962) + (assign4380_e2949 * (assign4380_e2962 * ((((-var_eg_dn10) / 2.0) * var_beta) + (assign4380_e2953 * var_beta_dn10)))));
        var_nin_dn11 = (assign4380_e2949 * (assign4380_e2962 * (((-var_eg_dn11) / 2.0) * var_beta)));
        var_nin_dn12 = (assign4380_e2949 * (assign4380_e2962 * (((-var_eg_dn12) / 2.0) * var_beta)));
        var_nin_dn17 = (assign4380_e2949 * (assign4380_e2962 * (((-var_eg_dn17) / 2.0) * var_beta)));
        var_nin_rv = 0.0;

        let assign4390_e2966: f64 = (var_beta_inv).sqrt();
        let assign4390_e2967: f64 = (var_costi00 * assign4390_e2966);
        var_costi0 = assign4390_e2967;
        var_costi0_dn0 = 0.0;
        var_costi0_dn2 = 0.0;
        var_costi0_dn6 = 0.0;
        var_costi0_dn7 = 0.0;
        var_costi0_dn10 = (var_costi00 * (var_beta_inv_dn10 / (2.0 * assign4390_e2966)));
        var_costi0_dn11 = 0.0;
        var_costi0_dn12 = 0.0;
        var_costi0_dn17 = 0.0;
        var_costi0_rv = 0.0;

        let assign4400_e2970: f64 = (var_costi0 * var_costi0);
        var_costi0_p2 = assign4400_e2970;
        var_costi0_p2_dn0 = ((var_costi0_dn0 * var_costi0) + (var_costi0 * var_costi0_dn0));
        var_costi0_p2_dn2 = ((var_costi0_dn2 * var_costi0) + (var_costi0 * var_costi0_dn2));
        var_costi0_p2_dn6 = ((var_costi0_dn6 * var_costi0) + (var_costi0 * var_costi0_dn6));
        var_costi0_p2_dn7 = ((var_costi0_dn7 * var_costi0) + (var_costi0 * var_costi0_dn7));
        var_costi0_p2_dn10 = ((var_costi0_dn10 * var_costi0) + (var_costi0 * var_costi0_dn10));
        var_costi0_p2_dn11 = ((var_costi0_dn11 * var_costi0) + (var_costi0 * var_costi0_dn11));
        var_costi0_p2_dn12 = ((var_costi0_dn12 * var_costi0) + (var_costi0 * var_costi0_dn12));
        var_costi0_p2_dn17 = ((var_costi0_dn17 * var_costi0) + (var_costi0 * var_costi0_dn17));
        var_costi0_p2_rv = 0.0;

        let assign4410_e2973: f64 = (var_nin * var_nin);
        let assign4410_e2975: f64 = (assign4410_e2973 * var_nsti_p2);
        var_costi1 = assign4410_e2975;
        var_costi1_dn0 = (((var_nin_dn0 * var_nin) + (var_nin * var_nin_dn0)) * var_nsti_p2);
        var_costi1_dn2 = (((var_nin_dn2 * var_nin) + (var_nin * var_nin_dn2)) * var_nsti_p2);
        var_costi1_dn6 = (((var_nin_dn6 * var_nin) + (var_nin * var_nin_dn6)) * var_nsti_p2);
        var_costi1_dn7 = (((var_nin_dn7 * var_nin) + (var_nin * var_nin_dn7)) * var_nsti_p2);
        var_costi1_dn10 = (((var_nin_dn10 * var_nin) + (var_nin * var_nin_dn10)) * var_nsti_p2);
        var_costi1_dn11 = (((var_nin_dn11 * var_nin) + (var_nin * var_nin_dn11)) * var_nsti_p2);
        var_costi1_dn12 = (((var_nin_dn12 * var_nin) + (var_nin * var_nin_dn12)) * var_nsti_p2);
        var_costi1_dn17 = (((var_nin_dn17 * var_nin) + (var_nin * var_nin_dn17)) * var_nsti_p2);
        var_costi1_rv = 0.0;

        let assign4420_e2979: f64 = (2.0 * p.p56);
        let assign4420_e2980: f64 = (var_lgate - assign4420_e2979);
        var_lch = assign4420_e2980;
        var_lch_dn0 = 0.0;
        var_lch_dn2 = 0.0;
        var_lch_dn6 = 0.0;
        var_lch_dn7 = 0.0;
        var_lch_dn10 = 0.0;
        var_lch_dn11 = 0.0;
        var_lch_dn12 = 0.0;
        var_lch_dn17 = 0.0;
        var_lch_rv = 0.0;

        let assign4430_e2983: f64 = if var_subversion > 3.0 { 1.0 } else { 0.0 };
        var_guard44 = assign4430_e2983;
        var_guard44_rv = 0.0;

        let (assign4440_e2994, assign4440_e2994_d_n0, assign4440_e2994_d_n2, assign4440_e2994_d_n6, assign4440_e2994_d_n7, assign4440_e2994_d_n10, assign4440_e2994_d_n11, assign4440_e2994_d_n12, assign4440_e2994_d_n17,) = {
    if (var_guard44 != 0.0) {
        let assign4440_e2987: f64 = (2.0 * var_beta_inv);
        let assign4440_e2990: f64 = (var_nsub / var_nin);
        let assign4440_e2991: f64 = (assign4440_e2990).ln();
        let assign4440_e2992: f64 = (assign4440_e2987 * assign4440_e2991);
        (assign4440_e2992, (assign4440_e2987 * ((((var_nsub_dn0 * var_nin) - (var_nsub * var_nin_dn0)) / (var_nin * var_nin)) / assign4440_e2990)), (assign4440_e2987 * ((((var_nsub_dn2 * var_nin) - (var_nsub * var_nin_dn2)) / (var_nin * var_nin)) / assign4440_e2990)), (assign4440_e2987 * ((((var_nsub_dn6 * var_nin) - (var_nsub * var_nin_dn6)) / (var_nin * var_nin)) / assign4440_e2990)), (assign4440_e2987 * ((((var_nsub_dn7 * var_nin) - (var_nsub * var_nin_dn7)) / (var_nin * var_nin)) / assign4440_e2990)), (((2.0 * var_beta_inv_dn10) * assign4440_e2991) + (assign4440_e2987 * ((((var_nsub_dn10 * var_nin) - (var_nsub * var_nin_dn10)) / (var_nin * var_nin)) / assign4440_e2990))), (assign4440_e2987 * ((((var_nsub_dn11 * var_nin) - (var_nsub * var_nin_dn11)) / (var_nin * var_nin)) / assign4440_e2990)), (assign4440_e2987 * ((((var_nsub_dn12 * var_nin) - (var_nsub * var_nin_dn12)) / (var_nin * var_nin)) / assign4440_e2990)), (assign4440_e2987 * ((((var_nsub_dn17 * var_nin) - (var_nsub * var_nin_dn17)) / (var_nin * var_nin)) / assign4440_e2990)),)
    } else {
        (var_pb2, var_pb2_dn0, var_pb2_dn2, var_pb2_dn6, var_pb2_dn7, var_pb2_dn10, var_pb2_dn11, var_pb2_dn12, var_pb2_dn17,)
    }
};
        var_pb2 = assign4440_e2994;
        var_pb2_dn0 = assign4440_e2994_d_n0;
        var_pb2_dn2 = assign4440_e2994_d_n2;
        var_pb2_dn6 = assign4440_e2994_d_n6;
        var_pb2_dn7 = assign4440_e2994_d_n7;
        var_pb2_dn10 = assign4440_e2994_d_n10;
        var_pb2_dn11 = assign4440_e2994_d_n11;
        var_pb2_dn12 = assign4440_e2994_d_n12;
        var_pb2_dn17 = assign4440_e2994_d_n17;
        var_pb2_rv = 0.0;

        let (assign4450_e3006, assign4450_e3006_d_n0, assign4450_e3006_d_n2, assign4450_e3006_d_n6, assign4450_e3006_d_n7, assign4450_e3006_d_n10, assign4450_e3006_d_n11, assign4450_e3006_d_n12, assign4450_e3006_d_n17,) = {
    if (var_guard44 == 0.0) {
        let assign4450_e2999: f64 = (2.0 * var_beta_inv);
        let assign4450_e3002: f64 = (var_uc_nsubs / var_nin);
        let assign4450_e3003: f64 = (assign4450_e3002).ln();
        let assign4450_e3004: f64 = (assign4450_e2999 * assign4450_e3003);
        (assign4450_e3004, (assign4450_e2999 * ((((var_uc_nsubs_dn0 * var_nin) - (var_uc_nsubs * var_nin_dn0)) / (var_nin * var_nin)) / assign4450_e3002)), (assign4450_e2999 * ((((var_uc_nsubs_dn2 * var_nin) - (var_uc_nsubs * var_nin_dn2)) / (var_nin * var_nin)) / assign4450_e3002)), (assign4450_e2999 * ((((var_uc_nsubs_dn6 * var_nin) - (var_uc_nsubs * var_nin_dn6)) / (var_nin * var_nin)) / assign4450_e3002)), (assign4450_e2999 * ((((var_uc_nsubs_dn7 * var_nin) - (var_uc_nsubs * var_nin_dn7)) / (var_nin * var_nin)) / assign4450_e3002)), (((2.0 * var_beta_inv_dn10) * assign4450_e3003) + (assign4450_e2999 * ((((var_uc_nsubs_dn10 * var_nin) - (var_uc_nsubs * var_nin_dn10)) / (var_nin * var_nin)) / assign4450_e3002))), (assign4450_e2999 * ((((var_uc_nsubs_dn11 * var_nin) - (var_uc_nsubs * var_nin_dn11)) / (var_nin * var_nin)) / assign4450_e3002)), (assign4450_e2999 * ((((var_uc_nsubs_dn12 * var_nin) - (var_uc_nsubs * var_nin_dn12)) / (var_nin * var_nin)) / assign4450_e3002)), (assign4450_e2999 * ((((var_uc_nsubs_dn17 * var_nin) - (var_uc_nsubs * var_nin_dn17)) / (var_nin * var_nin)) / assign4450_e3002)),)
    } else {
        (var_pb2, var_pb2_dn0, var_pb2_dn2, var_pb2_dn6, var_pb2_dn7, var_pb2_dn10, var_pb2_dn11, var_pb2_dn12, var_pb2_dn17,)
    }
};
        var_pb2 = assign4450_e3006;
        var_pb2_dn0 = assign4450_e3006_d_n0;
        var_pb2_dn2 = assign4450_e3006_d_n2;
        var_pb2_dn6 = assign4450_e3006_d_n6;
        var_pb2_dn7 = assign4450_e3006_d_n7;
        var_pb2_dn10 = assign4450_e3006_d_n10;
        var_pb2_dn11 = assign4450_e3006_d_n11;
        var_pb2_dn12 = assign4450_e3006_d_n12;
        var_pb2_dn17 = assign4450_e3006_d_n17;
        var_pb2_rv = 0.0;

        let assign4460_e3009: f64 = (1.034943e-10 / var_q_nsub);
        let assign4460_e3011: f64 = (assign4460_e3009 * var_beta_inv);
        let assign4460_e3012: f64 = (assign4460_e3011).sqrt();
        var_ldby = assign4460_e3012;
        var_ldby_dn0 = (((-((1.034943e-10 * var_q_nsub_dn0) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4460_e3012));
        var_ldby_dn2 = (((-((1.034943e-10 * var_q_nsub_dn2) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4460_e3012));
        var_ldby_dn6 = (((-((1.034943e-10 * var_q_nsub_dn6) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4460_e3012));
        var_ldby_dn7 = (((-((1.034943e-10 * var_q_nsub_dn7) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4460_e3012));
        var_ldby_dn10 = ((((-((1.034943e-10 * var_q_nsub_dn10) / (var_q_nsub * var_q_nsub))) * var_beta_inv) + (assign4460_e3009 * var_beta_inv_dn10)) / (2.0 * assign4460_e3012));
        var_ldby_dn11 = (((-((1.034943e-10 * var_q_nsub_dn11) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4460_e3012));
        var_ldby_dn12 = (((-((1.034943e-10 * var_q_nsub_dn12) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4460_e3012));
        var_ldby_dn17 = (((-((1.034943e-10 * var_q_nsub_dn17) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4460_e3012));
        var_ldby_rv = 0.0;

        let assign4470_e3015: f64 = (var_q_nsub * 1.414213562373095);
        let assign4470_e3017: f64 = (assign4470_e3015 * var_ldby);
        var_cnst0soi = assign4470_e3017;
        var_cnst0soi_dn0 = (((var_q_nsub_dn0 * 1.414213562373095) * var_ldby) + (assign4470_e3015 * var_ldby_dn0));
        var_cnst0soi_dn2 = (((var_q_nsub_dn2 * 1.414213562373095) * var_ldby) + (assign4470_e3015 * var_ldby_dn2));
        var_cnst0soi_dn6 = (((var_q_nsub_dn6 * 1.414213562373095) * var_ldby) + (assign4470_e3015 * var_ldby_dn6));
        var_cnst0soi_dn7 = (((var_q_nsub_dn7 * 1.414213562373095) * var_ldby) + (assign4470_e3015 * var_ldby_dn7));
        var_cnst0soi_dn10 = (((var_q_nsub_dn10 * 1.414213562373095) * var_ldby) + (assign4470_e3015 * var_ldby_dn10));
        var_cnst0soi_dn11 = (((var_q_nsub_dn11 * 1.414213562373095) * var_ldby) + (assign4470_e3015 * var_ldby_dn11));
        var_cnst0soi_dn12 = (((var_q_nsub_dn12 * 1.414213562373095) * var_ldby) + (assign4470_e3015 * var_ldby_dn12));
        var_cnst0soi_dn17 = (((var_q_nsub_dn17 * 1.414213562373095) * var_ldby) + (assign4470_e3015 * var_ldby_dn17));
        var_cnst0soi_rv = 0.0;

        let assign4480_e3020: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard45 = assign4480_e3020;
        var_guard45_rv = 0.0;

        let (assign4490_e3024, assign4490_e3024_d_n10,) = {
    if (var_guard45 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_cnst0bulk, var_cnst0bulk_dn10,)
    }
};
        var_cnst0bulk = assign4490_e3024;
        var_cnst0bulk_dn10 = assign4490_e3024_d_n10;
        var_cnst0bulk_rv = 0.0;

        let (assign4500_e3028, assign4500_e3028_d_n0, assign4500_e3028_d_n2, assign4500_e3028_d_n6, assign4500_e3028_d_n7, assign4500_e3028_d_n10, assign4500_e3028_d_n11, assign4500_e3028_d_n12, assign4500_e3028_d_n17,) = {
    if (var_guard45 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cnst1bulk, var_cnst1bulk_dn0, var_cnst1bulk_dn2, var_cnst1bulk_dn6, var_cnst1bulk_dn7, var_cnst1bulk_dn10, var_cnst1bulk_dn11, var_cnst1bulk_dn12, var_cnst1bulk_dn17,)
    }
};
        var_cnst1bulk = assign4500_e3028;
        var_cnst1bulk_dn0 = assign4500_e3028_d_n0;
        var_cnst1bulk_dn2 = assign4500_e3028_d_n2;
        var_cnst1bulk_dn6 = assign4500_e3028_d_n6;
        var_cnst1bulk_dn7 = assign4500_e3028_d_n7;
        var_cnst1bulk_dn10 = assign4500_e3028_d_n10;
        var_cnst1bulk_dn11 = assign4500_e3028_d_n11;
        var_cnst1bulk_dn12 = assign4500_e3028_d_n12;
        var_cnst1bulk_dn17 = assign4500_e3028_d_n17;
        var_cnst1bulk_rv = 0.0;

        let (assign4510_e3034, assign4510_e3034_d_n0, assign4510_e3034_d_n2, assign4510_e3034_d_n6, assign4510_e3034_d_n7, assign4510_e3034_d_n10, assign4510_e3034_d_n11, assign4510_e3034_d_n12, assign4510_e3034_d_n17,) = {
    if (var_guard45 != 0.0) {
        let assign4510_e3032: f64 = (var_nin / var_nsub);
        (assign4510_e3032, (((var_nin_dn0 * var_nsub) - (var_nin * var_nsub_dn0)) / (var_nsub * var_nsub)), (((var_nin_dn2 * var_nsub) - (var_nin * var_nsub_dn2)) / (var_nsub * var_nsub)), (((var_nin_dn6 * var_nsub) - (var_nin * var_nsub_dn6)) / (var_nsub * var_nsub)), (((var_nin_dn7 * var_nsub) - (var_nin * var_nsub_dn7)) / (var_nsub * var_nsub)), (((var_nin_dn10 * var_nsub) - (var_nin * var_nsub_dn10)) / (var_nsub * var_nsub)), (((var_nin_dn11 * var_nsub) - (var_nin * var_nsub_dn11)) / (var_nsub * var_nsub)), (((var_nin_dn12 * var_nsub) - (var_nin * var_nsub_dn12)) / (var_nsub * var_nsub)), (((var_nin_dn17 * var_nsub) - (var_nin * var_nsub_dn17)) / (var_nsub * var_nsub)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign4510_e3034;
        var_t1_dn0 = assign4510_e3034_d_n0;
        var_t1_dn2 = assign4510_e3034_d_n2;
        var_t1_dn6 = assign4510_e3034_d_n6;
        var_t1_dn7 = assign4510_e3034_d_n7;
        var_t1_dn10 = assign4510_e3034_d_n10;
        var_t1_dn11 = assign4510_e3034_d_n11;
        var_t1_dn12 = assign4510_e3034_d_n12;
        var_t1_dn17 = assign4510_e3034_d_n17;
        var_t1_rv = 0.0;

        let (assign4520_e3044, assign4520_e3044_d_n10,) = {
    if (var_guard45 == 0.0) {
        let assign4520_e3039: f64 = (2.0 * var_c0bulk);
        let assign4520_e3041: f64 = (assign4520_e3039 * var_beta_inv);
        let assign4520_e3042: f64 = (assign4520_e3041).sqrt();
        (assign4520_e3042, ((assign4520_e3039 * var_beta_inv_dn10) / (2.0 * assign4520_e3042)),)
    } else {
        (var_cnst0bulk, var_cnst0bulk_dn10,)
    }
};
        var_cnst0bulk = assign4520_e3044;
        var_cnst0bulk_dn10 = assign4520_e3044_d_n10;
        var_cnst0bulk_rv = 0.0;

        let (assign4530_e3051, assign4530_e3051_d_n0, assign4530_e3051_d_n2, assign4530_e3051_d_n6, assign4530_e3051_d_n7, assign4530_e3051_d_n10, assign4530_e3051_d_n11, assign4530_e3051_d_n12, assign4530_e3051_d_n17,) = {
    if (var_guard45 == 0.0) {
        let assign4530_e3049: f64 = (var_nin / var_mks_nsubb);
        (assign4530_e3049, (var_nin_dn0 / var_mks_nsubb), (var_nin_dn2 / var_mks_nsubb), (var_nin_dn6 / var_mks_nsubb), (var_nin_dn7 / var_mks_nsubb), (var_nin_dn10 / var_mks_nsubb), (var_nin_dn11 / var_mks_nsubb), (var_nin_dn12 / var_mks_nsubb), (var_nin_dn17 / var_mks_nsubb),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign4530_e3051;
        var_t1_dn0 = assign4530_e3051_d_n0;
        var_t1_dn2 = assign4530_e3051_d_n2;
        var_t1_dn6 = assign4530_e3051_d_n6;
        var_t1_dn7 = assign4530_e3051_d_n7;
        var_t1_dn10 = assign4530_e3051_d_n10;
        var_t1_dn11 = assign4530_e3051_d_n11;
        var_t1_dn12 = assign4530_e3051_d_n12;
        var_t1_dn17 = assign4530_e3051_d_n17;
        var_t1_rv = 0.0;

        let (assign4540_e3058, assign4540_e3058_d_n0, assign4540_e3058_d_n2, assign4540_e3058_d_n6, assign4540_e3058_d_n7, assign4540_e3058_d_n10, assign4540_e3058_d_n11, assign4540_e3058_d_n12, assign4540_e3058_d_n17,) = {
    if (var_guard45 == 0.0) {
        let assign4540_e3056: f64 = (var_t1 * var_t1);
        (assign4540_e3056, ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)), ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)), ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)), ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)), ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)), ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)), ((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)), ((var_t1_dn17 * var_t1) + (var_t1 * var_t1_dn17)),)
    } else {
        (var_cnst1bulk, var_cnst1bulk_dn0, var_cnst1bulk_dn2, var_cnst1bulk_dn6, var_cnst1bulk_dn7, var_cnst1bulk_dn10, var_cnst1bulk_dn11, var_cnst1bulk_dn12, var_cnst1bulk_dn17,)
    }
};
        var_cnst1bulk = assign4540_e3058;
        var_cnst1bulk_dn0 = assign4540_e3058_d_n0;
        var_cnst1bulk_dn2 = assign4540_e3058_d_n2;
        var_cnst1bulk_dn6 = assign4540_e3058_d_n6;
        var_cnst1bulk_dn7 = assign4540_e3058_d_n7;
        var_cnst1bulk_dn10 = assign4540_e3058_d_n10;
        var_cnst1bulk_dn11 = assign4540_e3058_d_n11;
        var_cnst1bulk_dn12 = assign4540_e3058_d_n12;
        var_cnst1bulk_dn17 = assign4540_e3058_d_n17;
        var_cnst1bulk_rv = 0.0;

        let (assign4550_e3065, assign4550_e3065_d_n0, assign4550_e3065_d_n2, assign4550_e3065_d_n6, assign4550_e3065_d_n7, assign4550_e3065_d_n10, assign4550_e3065_d_n11, assign4550_e3065_d_n12, assign4550_e3065_d_n17,) = {
    if (var_guard45 == 0.0) {
        let assign4550_e3063: f64 = (var_nin / var_uc_nsubs);
        (assign4550_e3063, (((var_nin_dn0 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn2 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn6 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn7 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn7)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn10 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn11 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn12 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn17 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn17)) / (var_uc_nsubs * var_uc_nsubs)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign4550_e3065;
        var_t1_dn0 = assign4550_e3065_d_n0;
        var_t1_dn2 = assign4550_e3065_d_n2;
        var_t1_dn6 = assign4550_e3065_d_n6;
        var_t1_dn7 = assign4550_e3065_d_n7;
        var_t1_dn10 = assign4550_e3065_d_n10;
        var_t1_dn11 = assign4550_e3065_d_n11;
        var_t1_dn12 = assign4550_e3065_d_n12;
        var_t1_dn17 = assign4550_e3065_d_n17;
        var_t1_rv = 0.0;

        let assign4560_e3068: f64 = (var_t1 * var_t1);
        var_cnst1soi = assign4560_e3068;
        var_cnst1soi_dn0 = ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0));
        var_cnst1soi_dn2 = ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2));
        var_cnst1soi_dn6 = ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6));
        var_cnst1soi_dn7 = ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7));
        var_cnst1soi_dn10 = ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10));
        var_cnst1soi_dn11 = ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11));
        var_cnst1soi_dn12 = ((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12));
        var_cnst1soi_dn17 = ((var_t1_dn17 * var_t1) + (var_t1 * var_t1_dn17));
        var_cnst1soi_rv = 0.0;

        let assign4570_e3072: f64 = (1.034943e-10 / var_q_nsub);
        let assign4570_e3074: f64 = (assign4570_e3072 / var_beta);
        let assign4570_e3075: f64 = (2.0 * assign4570_e3074);
        let assign4570_e3076: f64 = (assign4570_e3075).sqrt();
        var_c_w_soi = assign4570_e3076;
        var_c_w_soi_dn0 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn0) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4570_e3076));
        var_c_w_soi_dn2 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn2) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4570_e3076));
        var_c_w_soi_dn6 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn6) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4570_e3076));
        var_c_w_soi_dn7 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn7) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4570_e3076));
        var_c_w_soi_dn10 = ((2.0 * ((((-((1.034943e-10 * var_q_nsub_dn10) / (var_q_nsub * var_q_nsub))) * var_beta) - (assign4570_e3072 * var_beta_dn10)) / (var_beta * var_beta))) / (2.0 * assign4570_e3076));
        var_c_w_soi_dn11 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn11) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4570_e3076));
        var_c_w_soi_dn12 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn12) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4570_e3076));
        var_c_w_soi_dn17 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn17) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4570_e3076));
        var_c_w_soi_rv = 0.0;

        let assign4580_e3079: f64 = (2.0 * 1.034943e-10);
        let assign4580_e3081: f64 = (assign4580_e3079 / 1.6021918e-19);
        let assign4580_e3083: f64 = (assign4580_e3081 / var_uc_nsubs);
        var_cnst_2esi_q_nsubs = assign4580_e3083;
        var_cnst_2esi_q_nsubs_dn0 = (-((assign4580_e3081 * var_uc_nsubs_dn0) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn2 = (-((assign4580_e3081 * var_uc_nsubs_dn2) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn6 = (-((assign4580_e3081 * var_uc_nsubs_dn6) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn7 = (-((assign4580_e3081 * var_uc_nsubs_dn7) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn10 = (-((assign4580_e3081 * var_uc_nsubs_dn10) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn11 = (-((assign4580_e3081 * var_uc_nsubs_dn11) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn12 = (-((assign4580_e3081 * var_uc_nsubs_dn12) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn17 = (-((assign4580_e3081 * var_uc_nsubs_dn17) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_rv = 0.0;

        let assign4590_e3086: f64 = (2.0 * 1.034943e-10);
        let assign4590_e3088: f64 = (assign4590_e3086 / 1.6021918e-19);
        let assign4590_e3090: f64 = (assign4590_e3088 * var_pb2);
        let assign4590_e3092: f64 = (assign4590_e3090 / var_uc_nsubs);
        let assign4590_e3093: f64 = (assign4590_e3092).sqrt();
        var_wdsoi_ini = assign4590_e3093;
        var_wdsoi_ini_dn0 = (((((assign4590_e3088 * var_pb2_dn0) * var_uc_nsubs) - (assign4590_e3090 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4590_e3093));
        var_wdsoi_ini_dn2 = (((((assign4590_e3088 * var_pb2_dn2) * var_uc_nsubs) - (assign4590_e3090 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4590_e3093));
        var_wdsoi_ini_dn6 = (((((assign4590_e3088 * var_pb2_dn6) * var_uc_nsubs) - (assign4590_e3090 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4590_e3093));
        var_wdsoi_ini_dn7 = (((((assign4590_e3088 * var_pb2_dn7) * var_uc_nsubs) - (assign4590_e3090 * var_uc_nsubs_dn7)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4590_e3093));
        var_wdsoi_ini_dn10 = (((((assign4590_e3088 * var_pb2_dn10) * var_uc_nsubs) - (assign4590_e3090 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4590_e3093));
        var_wdsoi_ini_dn11 = (((((assign4590_e3088 * var_pb2_dn11) * var_uc_nsubs) - (assign4590_e3090 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4590_e3093));
        var_wdsoi_ini_dn12 = (((((assign4590_e3088 * var_pb2_dn12) * var_uc_nsubs) - (assign4590_e3090 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4590_e3093));
        var_wdsoi_ini_dn17 = (((((assign4590_e3088 * var_pb2_dn17) * var_uc_nsubs) - (assign4590_e3090 * var_uc_nsubs_dn17)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4590_e3093));
        var_wdsoi_ini_rv = 0.0;

        let assign4670_e3118: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard50 = assign4670_e3118;
        var_guard50_rv = 0.0;

        let (assign4680_e3122,) = {
    if (var_guard50 != 0.0) {
        (0.4,)
    } else {
        (var_vbs_bnd,)
    }
};
        var_vbs_bnd = assign4680_e3122;
        var_vbs_bnd_rv = 0.0;

        let (assign4690_e3126,) = {
    if (var_guard50 != 0.0) {
        (0.8,)
    } else {
        (var_vbs_max,)
    }
};
        var_vbs_max = assign4690_e3126;
        var_vbs_max_rv = 0.0;

        let (assign4700_e3131,) = {
    if (var_guard50 == 0.0) {
        (0.8,)
    } else {
        (var_vbs_bnd,)
    }
};
        var_vbs_bnd = assign4700_e3131;
        var_vbs_bnd_rv = 0.0;

        let (assign4710_e3136,) = {
    if (var_guard50 == 0.0) {
        (1.2,)
    } else {
        (var_vbs_max,)
    }
};
        var_vbs_max = assign4710_e3136;
        var_vbs_max_rv = 0.0;

        let assign4720_e3140: f64 = (var_vbs_max * 0.5);
        let assign4720_e3141: f64 = if var_vbs_bnd > assign4720_e3140 { 1.0 } else { 0.0 };
        var_guard51 = assign4720_e3141;
        var_guard51_rv = 0.0;

        let (assign4730_e3147,) = {
    if (var_guard51 != 0.0) {
        let assign4730_e3145: f64 = (0.5 * var_vbs_max);
        (assign4730_e3145,)
    } else {
        (var_vbs_bnd,)
    }
};
        var_vbs_bnd = assign4730_e3147;
        var_vbs_bnd_rv = 0.0;

        let assign4740_e3150: f64 = if var_vbs > var_vbs_bnd { 1.0 } else { 0.0 };
        var_guard52 = assign4740_e3150;
        var_guard52_rv = 0.0;

        let (assign4750_e3156, assign4750_e3156_d_n0, assign4750_e3156_d_n2, assign4750_e3156_d_n6, assign4750_e3156_d_n7, assign4750_e3156_d_n10, assign4750_e3156_d_n11, assign4750_e3156_d_n12, assign4750_e3156_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4750_e3154: f64 = (var_vbs - var_vbs_bnd);
        (assign4750_e3154, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign4750_e3156;
        var_t2_dn0 = assign4750_e3156_d_n0;
        var_t2_dn2 = assign4750_e3156_d_n2;
        var_t2_dn6 = assign4750_e3156_d_n6;
        var_t2_dn7 = assign4750_e3156_d_n7;
        var_t2_dn10 = assign4750_e3156_d_n10;
        var_t2_dn11 = assign4750_e3156_d_n11;
        var_t2_dn12 = assign4750_e3156_d_n12;
        var_t2_dn17 = assign4750_e3156_d_n17;
        var_t2_rv = 0.0;

        let (assign4760_e3162, assign4760_e3162_d_n0, assign4760_e3162_d_n2, assign4760_e3162_d_n6, assign4760_e3162_d_n7, assign4760_e3162_d_n10, assign4760_e3162_d_n11, assign4760_e3162_d_n12, assign4760_e3162_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4760_e3160: f64 = (var_vbs_max - var_vbs_bnd);
        (assign4760_e3160, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign4760_e3162;
        var_t3_dn0 = assign4760_e3162_d_n0;
        var_t3_dn2 = assign4760_e3162_d_n2;
        var_t3_dn6 = assign4760_e3162_d_n6;
        var_t3_dn7 = assign4760_e3162_d_n7;
        var_t3_dn10 = assign4760_e3162_d_n10;
        var_t3_dn11 = assign4760_e3162_d_n11;
        var_t3_dn12 = assign4760_e3162_d_n12;
        var_t3_dn17 = assign4760_e3162_d_n17;
        var_t3_rv = 0.0;

        let (assign4770_e3168, assign4770_e3168_d_n0, assign4770_e3168_d_n2, assign4770_e3168_d_n6, assign4770_e3168_d_n7, assign4770_e3168_d_n10, assign4770_e3168_d_n11, assign4770_e3168_d_n12, assign4770_e3168_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4770_e3166: f64 = (var_t2 * var_t2);
        (assign4770_e3166, ((var_t2_dn0 * var_t2) + (var_t2 * var_t2_dn0)), ((var_t2_dn2 * var_t2) + (var_t2 * var_t2_dn2)), ((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)), ((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7)), ((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10)), ((var_t2_dn11 * var_t2) + (var_t2 * var_t2_dn11)), ((var_t2_dn12 * var_t2) + (var_t2 * var_t2_dn12)), ((var_t2_dn17 * var_t2) + (var_t2 * var_t2_dn17)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn6, var_x2_dn7, var_x2_dn10, var_x2_dn11, var_x2_dn12, var_x2_dn17,)
    }
};
        var_x2 = assign4770_e3168;
        var_x2_dn0 = assign4770_e3168_d_n0;
        var_x2_dn2 = assign4770_e3168_d_n2;
        var_x2_dn6 = assign4770_e3168_d_n6;
        var_x2_dn7 = assign4770_e3168_d_n7;
        var_x2_dn10 = assign4770_e3168_d_n10;
        var_x2_dn11 = assign4770_e3168_d_n11;
        var_x2_dn12 = assign4770_e3168_d_n12;
        var_x2_dn17 = assign4770_e3168_d_n17;
        var_x2_rv = 0.0;

        let (assign4780_e3174, assign4780_e3174_d_n0, assign4780_e3174_d_n2, assign4780_e3174_d_n6, assign4780_e3174_d_n7, assign4780_e3174_d_n10, assign4780_e3174_d_n11, assign4780_e3174_d_n12, assign4780_e3174_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4780_e3172: f64 = (var_t3 * var_t3);
        (assign4780_e3172, ((var_t3_dn0 * var_t3) + (var_t3 * var_t3_dn0)), ((var_t3_dn2 * var_t3) + (var_t3 * var_t3_dn2)), ((var_t3_dn6 * var_t3) + (var_t3 * var_t3_dn6)), ((var_t3_dn7 * var_t3) + (var_t3 * var_t3_dn7)), ((var_t3_dn10 * var_t3) + (var_t3 * var_t3_dn10)), ((var_t3_dn11 * var_t3) + (var_t3 * var_t3_dn11)), ((var_t3_dn12 * var_t3) + (var_t3 * var_t3_dn12)), ((var_t3_dn17 * var_t3) + (var_t3 * var_t3_dn17)),)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn10, var_xmax2_dn11, var_xmax2_dn12, var_xmax2_dn17,)
    }
};
        var_xmax2 = assign4780_e3174;
        var_xmax2_dn0 = assign4780_e3174_d_n0;
        var_xmax2_dn2 = assign4780_e3174_d_n2;
        var_xmax2_dn6 = assign4780_e3174_d_n6;
        var_xmax2_dn7 = assign4780_e3174_d_n7;
        var_xmax2_dn10 = assign4780_e3174_d_n10;
        var_xmax2_dn11 = assign4780_e3174_d_n11;
        var_xmax2_dn12 = assign4780_e3174_d_n12;
        var_xmax2_dn17 = assign4780_e3174_d_n17;
        var_xmax2_rv = 0.0;

        let (assign4790_e3178, assign4790_e3178_d_n0, assign4790_e3178_d_n2, assign4790_e3178_d_n6, assign4790_e3178_d_n7, assign4790_e3178_d_n10, assign4790_e3178_d_n11, assign4790_e3178_d_n12, assign4790_e3178_d_n17,) = {
    if (var_guard52 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4790_e3178;
        var_xp_dn0 = assign4790_e3178_d_n0;
        var_xp_dn2 = assign4790_e3178_d_n2;
        var_xp_dn6 = assign4790_e3178_d_n6;
        var_xp_dn7 = assign4790_e3178_d_n7;
        var_xp_dn10 = assign4790_e3178_d_n10;
        var_xp_dn11 = assign4790_e3178_d_n11;
        var_xp_dn12 = assign4790_e3178_d_n12;
        var_xp_dn17 = assign4790_e3178_d_n17;
        var_xp_rv = 0.0;

        let (assign4800_e3182, assign4800_e3182_d_n0, assign4800_e3182_d_n2, assign4800_e3182_d_n6, assign4800_e3182_d_n7, assign4800_e3182_d_n10, assign4800_e3182_d_n11, assign4800_e3182_d_n12, assign4800_e3182_d_n17,) = {
    if (var_guard52 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4800_e3182;
        var_xmp_dn0 = assign4800_e3182_d_n0;
        var_xmp_dn2 = assign4800_e3182_d_n2;
        var_xmp_dn6 = assign4800_e3182_d_n6;
        var_xmp_dn7 = assign4800_e3182_d_n7;
        var_xmp_dn10 = assign4800_e3182_d_n10;
        var_xmp_dn11 = assign4800_e3182_d_n11;
        var_xmp_dn12 = assign4800_e3182_d_n12;
        var_xmp_dn17 = assign4800_e3182_d_n17;
        var_xmp_rv = 0.0;

        *var_c_w_soi_slot = var_c_w_soi;
        *var_c_w_soi_dn0_slot = var_c_w_soi_dn0;
        *var_c_w_soi_dn10_slot = var_c_w_soi_dn10;
        *var_c_w_soi_dn11_slot = var_c_w_soi_dn11;
        *var_c_w_soi_dn12_slot = var_c_w_soi_dn12;
        *var_c_w_soi_dn17_slot = var_c_w_soi_dn17;
        *var_c_w_soi_dn2_slot = var_c_w_soi_dn2;
        *var_c_w_soi_dn6_slot = var_c_w_soi_dn6;
        *var_c_w_soi_dn7_slot = var_c_w_soi_dn7;
        *var_c_w_soi_rv_slot = var_c_w_soi_rv;
        *var_cnst0bulk_slot = var_cnst0bulk;
        *var_cnst0bulk_dn10_slot = var_cnst0bulk_dn10;
        *var_cnst0bulk_rv_slot = var_cnst0bulk_rv;
        *var_cnst0soi_slot = var_cnst0soi;
        *var_cnst0soi_dn0_slot = var_cnst0soi_dn0;
        *var_cnst0soi_dn10_slot = var_cnst0soi_dn10;
        *var_cnst0soi_dn11_slot = var_cnst0soi_dn11;
        *var_cnst0soi_dn12_slot = var_cnst0soi_dn12;
        *var_cnst0soi_dn17_slot = var_cnst0soi_dn17;
        *var_cnst0soi_dn2_slot = var_cnst0soi_dn2;
        *var_cnst0soi_dn6_slot = var_cnst0soi_dn6;
        *var_cnst0soi_dn7_slot = var_cnst0soi_dn7;
        *var_cnst0soi_rv_slot = var_cnst0soi_rv;
        *var_cnst1bulk_slot = var_cnst1bulk;
        *var_cnst1bulk_dn0_slot = var_cnst1bulk_dn0;
        *var_cnst1bulk_dn10_slot = var_cnst1bulk_dn10;
        *var_cnst1bulk_dn11_slot = var_cnst1bulk_dn11;
        *var_cnst1bulk_dn12_slot = var_cnst1bulk_dn12;
        *var_cnst1bulk_dn17_slot = var_cnst1bulk_dn17;
        *var_cnst1bulk_dn2_slot = var_cnst1bulk_dn2;
        *var_cnst1bulk_dn6_slot = var_cnst1bulk_dn6;
        *var_cnst1bulk_dn7_slot = var_cnst1bulk_dn7;
        *var_cnst1bulk_rv_slot = var_cnst1bulk_rv;
        *var_cnst1soi_slot = var_cnst1soi;
        *var_cnst1soi_dn0_slot = var_cnst1soi_dn0;
        *var_cnst1soi_dn10_slot = var_cnst1soi_dn10;
        *var_cnst1soi_dn11_slot = var_cnst1soi_dn11;
        *var_cnst1soi_dn12_slot = var_cnst1soi_dn12;
        *var_cnst1soi_dn17_slot = var_cnst1soi_dn17;
        *var_cnst1soi_dn2_slot = var_cnst1soi_dn2;
        *var_cnst1soi_dn6_slot = var_cnst1soi_dn6;
        *var_cnst1soi_dn7_slot = var_cnst1soi_dn7;
        *var_cnst1soi_rv_slot = var_cnst1soi_rv;
        *var_cnst_2esi_q_nsubs_slot = var_cnst_2esi_q_nsubs;
        *var_cnst_2esi_q_nsubs_dn0_slot = var_cnst_2esi_q_nsubs_dn0;
        *var_cnst_2esi_q_nsubs_dn10_slot = var_cnst_2esi_q_nsubs_dn10;
        *var_cnst_2esi_q_nsubs_dn11_slot = var_cnst_2esi_q_nsubs_dn11;
        *var_cnst_2esi_q_nsubs_dn12_slot = var_cnst_2esi_q_nsubs_dn12;
        *var_cnst_2esi_q_nsubs_dn17_slot = var_cnst_2esi_q_nsubs_dn17;
        *var_cnst_2esi_q_nsubs_dn2_slot = var_cnst_2esi_q_nsubs_dn2;
        *var_cnst_2esi_q_nsubs_dn6_slot = var_cnst_2esi_q_nsubs_dn6;
        *var_cnst_2esi_q_nsubs_dn7_slot = var_cnst_2esi_q_nsubs_dn7;
        *var_cnst_2esi_q_nsubs_rv_slot = var_cnst_2esi_q_nsubs_rv;
        *var_costi0_slot = var_costi0;
        *var_costi0_dn0_slot = var_costi0_dn0;
        *var_costi0_dn10_slot = var_costi0_dn10;
        *var_costi0_dn11_slot = var_costi0_dn11;
        *var_costi0_dn12_slot = var_costi0_dn12;
        *var_costi0_dn17_slot = var_costi0_dn17;
        *var_costi0_dn2_slot = var_costi0_dn2;
        *var_costi0_dn6_slot = var_costi0_dn6;
        *var_costi0_dn7_slot = var_costi0_dn7;
        *var_costi0_p2_slot = var_costi0_p2;
        *var_costi0_p2_dn0_slot = var_costi0_p2_dn0;
        *var_costi0_p2_dn10_slot = var_costi0_p2_dn10;
        *var_costi0_p2_dn11_slot = var_costi0_p2_dn11;
        *var_costi0_p2_dn12_slot = var_costi0_p2_dn12;
        *var_costi0_p2_dn17_slot = var_costi0_p2_dn17;
        *var_costi0_p2_dn2_slot = var_costi0_p2_dn2;
        *var_costi0_p2_dn6_slot = var_costi0_p2_dn6;
        *var_costi0_p2_dn7_slot = var_costi0_p2_dn7;
        *var_costi0_p2_rv_slot = var_costi0_p2_rv;
        *var_costi0_rv_slot = var_costi0_rv;
        *var_costi1_slot = var_costi1;
        *var_costi1_dn0_slot = var_costi1_dn0;
        *var_costi1_dn10_slot = var_costi1_dn10;
        *var_costi1_dn11_slot = var_costi1_dn11;
        *var_costi1_dn12_slot = var_costi1_dn12;
        *var_costi1_dn17_slot = var_costi1_dn17;
        *var_costi1_dn2_slot = var_costi1_dn2;
        *var_costi1_dn6_slot = var_costi1_dn6;
        *var_costi1_dn7_slot = var_costi1_dn7;
        *var_costi1_rv_slot = var_costi1_rv;
        *var_guard44_slot = var_guard44;
        *var_guard44_rv_slot = var_guard44_rv;
        *var_guard45_slot = var_guard45;
        *var_guard45_rv_slot = var_guard45_rv;
        *var_guard50_slot = var_guard50;
        *var_guard50_rv_slot = var_guard50_rv;
        *var_guard51_slot = var_guard51;
        *var_guard51_rv_slot = var_guard51_rv;
        *var_guard52_slot = var_guard52;
        *var_guard52_rv_slot = var_guard52_rv;
        *var_lch_slot = var_lch;
        *var_lch_dn0_slot = var_lch_dn0;
        *var_lch_dn10_slot = var_lch_dn10;
        *var_lch_dn11_slot = var_lch_dn11;
        *var_lch_dn12_slot = var_lch_dn12;
        *var_lch_dn17_slot = var_lch_dn17;
        *var_lch_dn2_slot = var_lch_dn2;
        *var_lch_dn6_slot = var_lch_dn6;
        *var_lch_dn7_slot = var_lch_dn7;
        *var_lch_rv_slot = var_lch_rv;
        *var_ldby_slot = var_ldby;
        *var_ldby_dn0_slot = var_ldby_dn0;
        *var_ldby_dn10_slot = var_ldby_dn10;
        *var_ldby_dn11_slot = var_ldby_dn11;
        *var_ldby_dn12_slot = var_ldby_dn12;
        *var_ldby_dn17_slot = var_ldby_dn17;
        *var_ldby_dn2_slot = var_ldby_dn2;
        *var_ldby_dn6_slot = var_ldby_dn6;
        *var_ldby_dn7_slot = var_ldby_dn7;
        *var_ldby_rv_slot = var_ldby_rv;
        *var_nin_slot = var_nin;
        *var_nin_dn0_slot = var_nin_dn0;
        *var_nin_dn10_slot = var_nin_dn10;
        *var_nin_dn11_slot = var_nin_dn11;
        *var_nin_dn12_slot = var_nin_dn12;
        *var_nin_dn17_slot = var_nin_dn17;
        *var_nin_dn2_slot = var_nin_dn2;
        *var_nin_dn6_slot = var_nin_dn6;
        *var_nin_dn7_slot = var_nin_dn7;
        *var_nin_rv_slot = var_nin_rv;
        *var_pb2_slot = var_pb2;
        *var_pb2_dn0_slot = var_pb2_dn0;
        *var_pb2_dn10_slot = var_pb2_dn10;
        *var_pb2_dn11_slot = var_pb2_dn11;
        *var_pb2_dn12_slot = var_pb2_dn12;
        *var_pb2_dn17_slot = var_pb2_dn17;
        *var_pb2_dn2_slot = var_pb2_dn2;
        *var_pb2_dn6_slot = var_pb2_dn6;
        *var_pb2_dn7_slot = var_pb2_dn7;
        *var_pb2_rv_slot = var_pb2_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_rv_slot = var_t3_rv;
        *var_vbs_bnd_slot = var_vbs_bnd;
        *var_vbs_bnd_rv_slot = var_vbs_bnd_rv;
        *var_vbs_max_slot = var_vbs_max;
        *var_vbs_max_rv_slot = var_vbs_max_rv;
        *var_wdsoi_ini_slot = var_wdsoi_ini;
        *var_wdsoi_ini_dn0_slot = var_wdsoi_ini_dn0;
        *var_wdsoi_ini_dn10_slot = var_wdsoi_ini_dn10;
        *var_wdsoi_ini_dn11_slot = var_wdsoi_ini_dn11;
        *var_wdsoi_ini_dn12_slot = var_wdsoi_ini_dn12;
        *var_wdsoi_ini_dn17_slot = var_wdsoi_ini_dn17;
        *var_wdsoi_ini_dn2_slot = var_wdsoi_ini_dn2;
        *var_wdsoi_ini_dn6_slot = var_wdsoi_ini_dn6;
        *var_wdsoi_ini_dn7_slot = var_wdsoi_ini_dn7;
        *var_wdsoi_ini_rv_slot = var_wdsoi_ini_rv;
        *var_x2_slot = var_x2;
        *var_x2_dn0_slot = var_x2_dn0;
        *var_x2_dn10_slot = var_x2_dn10;
        *var_x2_dn11_slot = var_x2_dn11;
        *var_x2_dn12_slot = var_x2_dn12;
        *var_x2_dn17_slot = var_x2_dn17;
        *var_x2_dn2_slot = var_x2_dn2;
        *var_x2_dn6_slot = var_x2_dn6;
        *var_x2_dn7_slot = var_x2_dn7;
        *var_x2_rv_slot = var_x2_rv;
        *var_xmax2_slot = var_xmax2;
        *var_xmax2_dn0_slot = var_xmax2_dn0;
        *var_xmax2_dn10_slot = var_xmax2_dn10;
        *var_xmax2_dn11_slot = var_xmax2_dn11;
        *var_xmax2_dn12_slot = var_xmax2_dn12;
        *var_xmax2_dn17_slot = var_xmax2_dn17;
        *var_xmax2_dn2_slot = var_xmax2_dn2;
        *var_xmax2_dn6_slot = var_xmax2_dn6;
        *var_xmax2_dn7_slot = var_xmax2_dn7;
        *var_xmax2_rv_slot = var_xmax2_rv;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn11_slot = var_xmp_dn11;
        *var_xmp_dn12_slot = var_xmp_dn12;
        *var_xmp_dn17_slot = var_xmp_dn17;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn7_slot = var_xmp_dn7;
        *var_xmp_rv_slot = var_xmp_rv;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn11_slot = var_xp_dn11;
        *var_xp_dn12_slot = var_xp_dn12;
        *var_xp_dn17_slot = var_xp_dn17;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_rv_slot = var_xp_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        var_guard52: f64,
        var_t2: f64,
        var_t2_dn0: f64,
        var_t2_dn10: f64,
        var_t2_dn11: f64,
        var_t2_dn12: f64,
        var_t2_dn17: f64,
        var_t2_dn2: f64,
        var_t2_dn6: f64,
        var_t2_dn7: f64,
        var_t3: f64,
        var_t3_dn0: f64,
        var_t3_dn10: f64,
        var_t3_dn11: f64,
        var_t3_dn12: f64,
        var_t3_dn17: f64,
        var_t3_dn2: f64,
        var_t3_dn6: f64,
        var_t3_dn7: f64,
        var_vbs: f64,
        var_vbs_bnd: f64,
        var_vbs_dn0: f64,
        var_vbs_dn10: f64,
        var_vbs_dn11: f64,
        var_vbs_dn12: f64,
        var_vbs_dn17: f64,
        var_vbs_dn2: f64,
        var_vbs_dn6: f64,
        var_vbs_dn7: f64,
        var_x2: f64,
        var_x2_dn0: f64,
        var_x2_dn10: f64,
        var_x2_dn11: f64,
        var_x2_dn12: f64,
        var_x2_dn17: f64,
        var_x2_dn2: f64,
        var_x2_dn6: f64,
        var_x2_dn7: f64,
        var_xmax2: f64,
        var_xmax2_dn0: f64,
        var_xmax2_dn10: f64,
        var_xmax2_dn11: f64,
        var_xmax2_dn12: f64,
        var_xmax2_dn17: f64,
        var_xmax2_dn2: f64,
        var_xmax2_dn6: f64,
        var_xmax2_dn7: f64,
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn11_slot: &mut f64,
        var_arg_dn12_slot: &mut f64,
        var_arg_dn17_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn11_slot: &mut f64,
        var_dnm_dn12_slot: &mut f64,
        var_dnm_dn17_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_rv_slot: &mut f64,
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
        var_m0_slot: &mut f64,
        var_m0_rv_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_mm_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn17_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn11_slot: &mut f64,
        var_t8_dn12_slot: &mut f64,
        var_t8_dn17_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
        var_vbsc_slot: &mut f64,
        var_vbsc_dn0_slot: &mut f64,
        var_vbsc_dn10_slot: &mut f64,
        var_vbsc_dn11_slot: &mut f64,
        var_vbsc_dn12_slot: &mut f64,
        var_vbsc_dn17_slot: &mut f64,
        var_vbsc_dn2_slot: &mut f64,
        var_vbsc_dn6_slot: &mut f64,
        var_vbsc_dn7_slot: &mut f64,
        var_vbsc_dvbse_slot: &mut f64,
        var_vbsc_dvbse_dn0_slot: &mut f64,
        var_vbsc_dvbse_dn10_slot: &mut f64,
        var_vbsc_dvbse_dn11_slot: &mut f64,
        var_vbsc_dvbse_dn12_slot: &mut f64,
        var_vbsc_dvbse_dn17_slot: &mut f64,
        var_vbsc_dvbse_dn2_slot: &mut f64,
        var_vbsc_dvbse_dn6_slot: &mut f64,
        var_vbsc_dvbse_dn7_slot: &mut f64,
        var_vbsc_dvbse_rv_slot: &mut f64,
        var_vbsc_rv_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn11_slot: &mut f64,
        var_xmp_dn12_slot: &mut f64,
        var_xmp_dn17_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn7_slot: &mut f64,
        var_xmp_rv_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn11_slot: &mut f64,
        var_xp_dn12_slot: &mut f64,
        var_xp_dn17_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
        var_xp_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn11: f64 = *var_arg_dn11_slot;
        let mut var_arg_dn12: f64 = *var_arg_dn12_slot;
        let mut var_arg_dn17: f64 = *var_arg_dn17_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn11: f64 = *var_dnm_dn11_slot;
        let mut var_dnm_dn12: f64 = *var_dnm_dn12_slot;
        let mut var_dnm_dn17: f64 = *var_dnm_dn17_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_rv: f64 = *var_dnm_rv_slot;
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
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_m0_rv: f64 = *var_m0_rv_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_mm_rv: f64 = *var_mm_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn17: f64 = *var_t4_dn17_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn11: f64 = *var_t8_dn11_slot;
        let mut var_t8_dn12: f64 = *var_t8_dn12_slot;
        let mut var_t8_dn17: f64 = *var_t8_dn17_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;
        let mut var_vbsc: f64 = *var_vbsc_slot;
        let mut var_vbsc_dn0: f64 = *var_vbsc_dn0_slot;
        let mut var_vbsc_dn10: f64 = *var_vbsc_dn10_slot;
        let mut var_vbsc_dn11: f64 = *var_vbsc_dn11_slot;
        let mut var_vbsc_dn12: f64 = *var_vbsc_dn12_slot;
        let mut var_vbsc_dn17: f64 = *var_vbsc_dn17_slot;
        let mut var_vbsc_dn2: f64 = *var_vbsc_dn2_slot;
        let mut var_vbsc_dn6: f64 = *var_vbsc_dn6_slot;
        let mut var_vbsc_dn7: f64 = *var_vbsc_dn7_slot;
        let mut var_vbsc_dvbse: f64 = *var_vbsc_dvbse_slot;
        let mut var_vbsc_dvbse_dn0: f64 = *var_vbsc_dvbse_dn0_slot;
        let mut var_vbsc_dvbse_dn10: f64 = *var_vbsc_dvbse_dn10_slot;
        let mut var_vbsc_dvbse_dn11: f64 = *var_vbsc_dvbse_dn11_slot;
        let mut var_vbsc_dvbse_dn12: f64 = *var_vbsc_dvbse_dn12_slot;
        let mut var_vbsc_dvbse_dn17: f64 = *var_vbsc_dvbse_dn17_slot;
        let mut var_vbsc_dvbse_dn2: f64 = *var_vbsc_dvbse_dn2_slot;
        let mut var_vbsc_dvbse_dn6: f64 = *var_vbsc_dvbse_dn6_slot;
        let mut var_vbsc_dvbse_dn7: f64 = *var_vbsc_dvbse_dn7_slot;
        let mut var_vbsc_dvbse_rv: f64 = *var_vbsc_dvbse_rv_slot;
        let mut var_vbsc_rv: f64 = *var_vbsc_rv_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn11: f64 = *var_xmp_dn11_slot;
        let mut var_xmp_dn12: f64 = *var_xmp_dn12_slot;
        let mut var_xmp_dn17: f64 = *var_xmp_dn17_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn7: f64 = *var_xmp_dn7_slot;
        let mut var_xmp_rv: f64 = *var_xmp_rv_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn11: f64 = *var_xp_dn11_slot;
        let mut var_xp_dn12: f64 = *var_xp_dn12_slot;
        let mut var_xp_dn17: f64 = *var_xp_dn17_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_rv: f64 = *var_xp_rv_slot;

        let (assign4810_e3186,) = {
    if (var_guard52 != 0.0) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign4810_e3186;
        var_m0_rv = 0.0;

        let (assign4820_e3190,) = {
    if (var_guard52 != 0.0) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign4820_e3190;
        var_mm_rv = 0.0;

        let (assign4830_e3194, assign4830_e3194_d_n0, assign4830_e3194_d_n2, assign4830_e3194_d_n6, assign4830_e3194_d_n7, assign4830_e3194_d_n10, assign4830_e3194_d_n11, assign4830_e3194_d_n12, assign4830_e3194_d_n17,) = {
    if (var_guard52 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    }
};
        var_arg = assign4830_e3194;
        var_arg_dn0 = assign4830_e3194_d_n0;
        var_arg_dn2 = assign4830_e3194_d_n2;
        var_arg_dn6 = assign4830_e3194_d_n6;
        var_arg_dn7 = assign4830_e3194_d_n7;
        var_arg_dn10 = assign4830_e3194_d_n10;
        var_arg_dn11 = assign4830_e3194_d_n11;
        var_arg_dn12 = assign4830_e3194_d_n12;
        var_arg_dn17 = assign4830_e3194_d_n17;
        var_arg_rv = 0.0;

        let (assign4840_e3198, assign4840_e3198_d_n0, assign4840_e3198_d_n2, assign4840_e3198_d_n6, assign4840_e3198_d_n7, assign4840_e3198_d_n10, assign4840_e3198_d_n11, assign4840_e3198_d_n12, assign4840_e3198_d_n17,) = {
    if (var_guard52 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign4840_e3198;
        var_dnm_dn0 = assign4840_e3198_d_n0;
        var_dnm_dn2 = assign4840_e3198_d_n2;
        var_dnm_dn6 = assign4840_e3198_d_n6;
        var_dnm_dn7 = assign4840_e3198_d_n7;
        var_dnm_dn10 = assign4840_e3198_d_n10;
        var_dnm_dn11 = assign4840_e3198_d_n11;
        var_dnm_dn12 = assign4840_e3198_d_n12;
        var_dnm_dn17 = assign4840_e3198_d_n17;
        var_dnm_rv = 0.0;

        let (assign4850_e3204, assign4850_e3204_d_n0, assign4850_e3204_d_n2, assign4850_e3204_d_n6, assign4850_e3204_d_n7, assign4850_e3204_d_n10, assign4850_e3204_d_n11, assign4850_e3204_d_n12, assign4850_e3204_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4850_e3202: f64 = (var_xp * var_x2);
        (assign4850_e3202, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4850_e3204;
        var_xp_dn0 = assign4850_e3204_d_n0;
        var_xp_dn2 = assign4850_e3204_d_n2;
        var_xp_dn6 = assign4850_e3204_d_n6;
        var_xp_dn7 = assign4850_e3204_d_n7;
        var_xp_dn10 = assign4850_e3204_d_n10;
        var_xp_dn11 = assign4850_e3204_d_n11;
        var_xp_dn12 = assign4850_e3204_d_n12;
        var_xp_dn17 = assign4850_e3204_d_n17;
        var_xp_rv = 0.0;

        let (assign4860_e3210, assign4860_e3210_d_n0, assign4860_e3210_d_n2, assign4860_e3210_d_n6, assign4860_e3210_d_n7, assign4860_e3210_d_n10, assign4860_e3210_d_n11, assign4860_e3210_d_n12, assign4860_e3210_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4860_e3208: f64 = (var_xmp * var_xmax2);
        (assign4860_e3208, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4860_e3210;
        var_xmp_dn0 = assign4860_e3210_d_n0;
        var_xmp_dn2 = assign4860_e3210_d_n2;
        var_xmp_dn6 = assign4860_e3210_d_n6;
        var_xmp_dn7 = assign4860_e3210_d_n7;
        var_xmp_dn10 = assign4860_e3210_d_n10;
        var_xmp_dn11 = assign4860_e3210_d_n11;
        var_xmp_dn12 = assign4860_e3210_d_n12;
        var_xmp_dn17 = assign4860_e3210_d_n17;
        var_xmp_rv = 0.0;

        let (assign4870_e3216, assign4870_e3216_d_n0, assign4870_e3216_d_n2, assign4870_e3216_d_n6, assign4870_e3216_d_n7, assign4870_e3216_d_n10, assign4870_e3216_d_n11, assign4870_e3216_d_n12, assign4870_e3216_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4870_e3214: f64 = (var_xp * var_x2);
        (assign4870_e3214, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4870_e3216;
        var_xp_dn0 = assign4870_e3216_d_n0;
        var_xp_dn2 = assign4870_e3216_d_n2;
        var_xp_dn6 = assign4870_e3216_d_n6;
        var_xp_dn7 = assign4870_e3216_d_n7;
        var_xp_dn10 = assign4870_e3216_d_n10;
        var_xp_dn11 = assign4870_e3216_d_n11;
        var_xp_dn12 = assign4870_e3216_d_n12;
        var_xp_dn17 = assign4870_e3216_d_n17;
        var_xp_rv = 0.0;

        let (assign4880_e3222, assign4880_e3222_d_n0, assign4880_e3222_d_n2, assign4880_e3222_d_n6, assign4880_e3222_d_n7, assign4880_e3222_d_n10, assign4880_e3222_d_n11, assign4880_e3222_d_n12, assign4880_e3222_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4880_e3220: f64 = (var_xmp * var_xmax2);
        (assign4880_e3220, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4880_e3222;
        var_xmp_dn0 = assign4880_e3222_d_n0;
        var_xmp_dn2 = assign4880_e3222_d_n2;
        var_xmp_dn6 = assign4880_e3222_d_n6;
        var_xmp_dn7 = assign4880_e3222_d_n7;
        var_xmp_dn10 = assign4880_e3222_d_n10;
        var_xmp_dn11 = assign4880_e3222_d_n11;
        var_xmp_dn12 = assign4880_e3222_d_n12;
        var_xmp_dn17 = assign4880_e3222_d_n17;
        var_xmp_rv = 0.0;

        let (assign4890_e3228, assign4890_e3228_d_n0, assign4890_e3228_d_n2, assign4890_e3228_d_n6, assign4890_e3228_d_n7, assign4890_e3228_d_n10, assign4890_e3228_d_n11, assign4890_e3228_d_n12, assign4890_e3228_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4890_e3226: f64 = (var_xp * var_x2);
        (assign4890_e3226, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4890_e3228;
        var_xp_dn0 = assign4890_e3228_d_n0;
        var_xp_dn2 = assign4890_e3228_d_n2;
        var_xp_dn6 = assign4890_e3228_d_n6;
        var_xp_dn7 = assign4890_e3228_d_n7;
        var_xp_dn10 = assign4890_e3228_d_n10;
        var_xp_dn11 = assign4890_e3228_d_n11;
        var_xp_dn12 = assign4890_e3228_d_n12;
        var_xp_dn17 = assign4890_e3228_d_n17;
        var_xp_rv = 0.0;

        let (assign4900_e3234, assign4900_e3234_d_n0, assign4900_e3234_d_n2, assign4900_e3234_d_n6, assign4900_e3234_d_n7, assign4900_e3234_d_n10, assign4900_e3234_d_n11, assign4900_e3234_d_n12, assign4900_e3234_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4900_e3232: f64 = (var_xmp * var_xmax2);
        (assign4900_e3232, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4900_e3234;
        var_xmp_dn0 = assign4900_e3234_d_n0;
        var_xmp_dn2 = assign4900_e3234_d_n2;
        var_xmp_dn6 = assign4900_e3234_d_n6;
        var_xmp_dn7 = assign4900_e3234_d_n7;
        var_xmp_dn10 = assign4900_e3234_d_n10;
        var_xmp_dn11 = assign4900_e3234_d_n11;
        var_xmp_dn12 = assign4900_e3234_d_n12;
        var_xmp_dn17 = assign4900_e3234_d_n17;
        var_xmp_rv = 0.0;

        let (assign4910_e3240, assign4910_e3240_d_n0, assign4910_e3240_d_n2, assign4910_e3240_d_n6, assign4910_e3240_d_n7, assign4910_e3240_d_n10, assign4910_e3240_d_n11, assign4910_e3240_d_n12, assign4910_e3240_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4910_e3238: f64 = (var_xp * var_x2);
        (assign4910_e3238, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4910_e3240;
        var_xp_dn0 = assign4910_e3240_d_n0;
        var_xp_dn2 = assign4910_e3240_d_n2;
        var_xp_dn6 = assign4910_e3240_d_n6;
        var_xp_dn7 = assign4910_e3240_d_n7;
        var_xp_dn10 = assign4910_e3240_d_n10;
        var_xp_dn11 = assign4910_e3240_d_n11;
        var_xp_dn12 = assign4910_e3240_d_n12;
        var_xp_dn17 = assign4910_e3240_d_n17;
        var_xp_rv = 0.0;

        let (assign4920_e3246, assign4920_e3246_d_n0, assign4920_e3246_d_n2, assign4920_e3246_d_n6, assign4920_e3246_d_n7, assign4920_e3246_d_n10, assign4920_e3246_d_n11, assign4920_e3246_d_n12, assign4920_e3246_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4920_e3244: f64 = (var_xmp * var_xmax2);
        (assign4920_e3244, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4920_e3246;
        var_xmp_dn0 = assign4920_e3246_d_n0;
        var_xmp_dn2 = assign4920_e3246_d_n2;
        var_xmp_dn6 = assign4920_e3246_d_n6;
        var_xmp_dn7 = assign4920_e3246_d_n7;
        var_xmp_dn10 = assign4920_e3246_d_n10;
        var_xmp_dn11 = assign4920_e3246_d_n11;
        var_xmp_dn12 = assign4920_e3246_d_n12;
        var_xmp_dn17 = assign4920_e3246_d_n17;
        var_xmp_rv = 0.0;

        let (assign4930_e3252, assign4930_e3252_d_n0, assign4930_e3252_d_n2, assign4930_e3252_d_n6, assign4930_e3252_d_n7, assign4930_e3252_d_n10, assign4930_e3252_d_n11, assign4930_e3252_d_n12, assign4930_e3252_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign4930_e3250: f64 = (var_xp + var_xmp);
        (assign4930_e3250, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn11 + var_xmp_dn11), (var_xp_dn12 + var_xmp_dn12), (var_xp_dn17 + var_xmp_dn17),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    }
};
        var_arg = assign4930_e3252;
        var_arg_dn0 = assign4930_e3252_d_n0;
        var_arg_dn2 = assign4930_e3252_d_n2;
        var_arg_dn6 = assign4930_e3252_d_n6;
        var_arg_dn7 = assign4930_e3252_d_n7;
        var_arg_dn10 = assign4930_e3252_d_n10;
        var_arg_dn11 = assign4930_e3252_d_n11;
        var_arg_dn12 = assign4930_e3252_d_n12;
        var_arg_dn17 = assign4930_e3252_d_n17;
        var_arg_rv = 0.0;

        let (assign4940_e3256, assign4940_e3256_d_n0, assign4940_e3256_d_n2, assign4940_e3256_d_n6, assign4940_e3256_d_n7, assign4940_e3256_d_n10, assign4940_e3256_d_n11, assign4940_e3256_d_n12, assign4940_e3256_d_n17,) = {
    if (var_guard52 != 0.0) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign4940_e3256;
        var_dnm_dn0 = assign4940_e3256_d_n0;
        var_dnm_dn2 = assign4940_e3256_d_n2;
        var_dnm_dn6 = assign4940_e3256_d_n6;
        var_dnm_dn7 = assign4940_e3256_d_n7;
        var_dnm_dn10 = assign4940_e3256_d_n10;
        var_dnm_dn11 = assign4940_e3256_d_n11;
        var_dnm_dn12 = assign4940_e3256_d_n12;
        var_dnm_dn17 = assign4940_e3256_d_n17;
        var_dnm_rv = 0.0;

        let assign4950_e3271: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard53 = assign4950_e3271;
        var_guard53_rv = 0.0;

        let assign4960_e3274: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard54 = assign4960_e3274;
        var_guard54_rv = 0.0;

        let (assign4970_e3282,) = {
    if (((var_guard52 != 0.0) && (var_guard53 != 0.0)) && (var_guard54 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign4970_e3282;
        var_mm_rv = 0.0;

        let assign4980_e3285: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard55 = assign4980_e3285;
        var_guard55_rv = 0.0;

        let (assign4990_e3296,) = {
    if ((((var_guard52 != 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign4990_e3296;
        var_mm_rv = 0.0;

        let assign5000_e3299: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard56 = assign5000_e3299;
        var_guard56_rv = 0.0;

        let (assign5010_e3313,) = {
    if (((((var_guard52 != 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard56 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign5010_e3313;
        var_mm_rv = 0.0;

        let assign5020_e3316: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard57 = assign5020_e3316;
        var_guard57_rv = 0.0;

        let (assign5030_e3333,) = {
    if ((((((var_guard52 != 0.0) && (var_guard53 != 0.0)) && (var_guard54 == 0.0)) && (var_guard55 == 0.0)) && (var_guard56 == 0.0)) && (var_guard57 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign5030_e3333;
        var_mm_rv = 0.0;

        let (assign5040_e3339,) = {
    if ((var_guard52 != 0.0) && (var_guard53 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign5040_e3339;
        var_m0_rv = 0.0;

        let mut assign5050_loop_guard: usize = 0;
        while {
            let assign5050_cond_e3346: f64 = if (((var_guard52 != 0.0) && (var_guard53 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign5050_cond_e3346 != 0.0
        } {
            assign5050_loop_guard += 1;
            assert!(assign5050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign5050_body0_e3353, assign5050_body0_e3353_d_n0, assign5050_body0_e3353_d_n2, assign5050_body0_e3353_d_n6, assign5050_body0_e3353_d_n7, assign5050_body0_e3353_d_n10, assign5050_body0_e3353_d_n11, assign5050_body0_e3353_d_n12, assign5050_body0_e3353_d_n17,) = {
    if ((var_guard52 != 0.0) && (var_guard53 != 0.0)) {
        let assign5050_body0_e3351: f64 = (var_dnm).sqrt();
        (assign5050_body0_e3351, (var_dnm_dn0 / (2.0 * assign5050_body0_e3351)), (var_dnm_dn2 / (2.0 * assign5050_body0_e3351)), (var_dnm_dn6 / (2.0 * assign5050_body0_e3351)), (var_dnm_dn7 / (2.0 * assign5050_body0_e3351)), (var_dnm_dn10 / (2.0 * assign5050_body0_e3351)), (var_dnm_dn11 / (2.0 * assign5050_body0_e3351)), (var_dnm_dn12 / (2.0 * assign5050_body0_e3351)), (var_dnm_dn17 / (2.0 * assign5050_body0_e3351)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
            var_dnm = assign5050_body0_e3353;
            var_dnm_dn0 = assign5050_body0_e3353_d_n0;
            var_dnm_dn2 = assign5050_body0_e3353_d_n2;
            var_dnm_dn6 = assign5050_body0_e3353_d_n6;
            var_dnm_dn7 = assign5050_body0_e3353_d_n7;
            var_dnm_dn10 = assign5050_body0_e3353_d_n10;
            var_dnm_dn11 = assign5050_body0_e3353_d_n11;
            var_dnm_dn12 = assign5050_body0_e3353_d_n12;
            var_dnm_dn17 = assign5050_body0_e3353_d_n17;
            var_dnm_rv = 0.0;
            let (assign5050_body1_e3361,) = {
    if ((var_guard52 != 0.0) && (var_guard53 != 0.0)) {
        let assign5050_body1_e3359: f64 = (var_m0 + 1.0);
        (assign5050_body1_e3359,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign5050_body1_e3361;
            var_m0_rv = 0.0;
        }

        let (assign5060_e3374, assign5060_e3374_d_n0, assign5060_e3374_d_n2, assign5060_e3374_d_n6, assign5060_e3374_d_n7, assign5060_e3374_d_n10, assign5060_e3374_d_n11, assign5060_e3374_d_n12, assign5060_e3374_d_n17,) = {
    if ((var_guard52 != 0.0) && (var_guard53 == 0.0)) {
        let assign5060_e3370: f64 = (2.0 * 4.0);
        let assign5060_e3371: f64 = (1.0 / assign5060_e3370);
        let assign5060_e3372: f64 = (var_dnm).powf(assign5060_e3371);
        (assign5060_e3372, if 0.0 == 0.0 && ((assign5060_e3371) as f64).is_finite() && ((assign5060_e3371) as f64).fract() == 0.0 { if assign5060_e3371 == 0.0 { 0.0 } else { (assign5060_e3371 * ((var_dnm).powf(assign5060_e3371 - 1.0) * var_dnm_dn0)) } } else { (assign5060_e3372 * (assign5060_e3371 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign5060_e3371) as f64).is_finite() && ((assign5060_e3371) as f64).fract() == 0.0 { if assign5060_e3371 == 0.0 { 0.0 } else { (assign5060_e3371 * ((var_dnm).powf(assign5060_e3371 - 1.0) * var_dnm_dn2)) } } else { (assign5060_e3372 * (assign5060_e3371 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign5060_e3371) as f64).is_finite() && ((assign5060_e3371) as f64).fract() == 0.0 { if assign5060_e3371 == 0.0 { 0.0 } else { (assign5060_e3371 * ((var_dnm).powf(assign5060_e3371 - 1.0) * var_dnm_dn6)) } } else { (assign5060_e3372 * (assign5060_e3371 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign5060_e3371) as f64).is_finite() && ((assign5060_e3371) as f64).fract() == 0.0 { if assign5060_e3371 == 0.0 { 0.0 } else { (assign5060_e3371 * ((var_dnm).powf(assign5060_e3371 - 1.0) * var_dnm_dn7)) } } else { (assign5060_e3372 * (assign5060_e3371 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign5060_e3371) as f64).is_finite() && ((assign5060_e3371) as f64).fract() == 0.0 { if assign5060_e3371 == 0.0 { 0.0 } else { (assign5060_e3371 * ((var_dnm).powf(assign5060_e3371 - 1.0) * var_dnm_dn10)) } } else { (assign5060_e3372 * (assign5060_e3371 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign5060_e3371) as f64).is_finite() && ((assign5060_e3371) as f64).fract() == 0.0 { if assign5060_e3371 == 0.0 { 0.0 } else { (assign5060_e3371 * ((var_dnm).powf(assign5060_e3371 - 1.0) * var_dnm_dn11)) } } else { (assign5060_e3372 * (assign5060_e3371 * (var_dnm_dn11 / var_dnm))) }, if 0.0 == 0.0 && ((assign5060_e3371) as f64).is_finite() && ((assign5060_e3371) as f64).fract() == 0.0 { if assign5060_e3371 == 0.0 { 0.0 } else { (assign5060_e3371 * ((var_dnm).powf(assign5060_e3371 - 1.0) * var_dnm_dn12)) } } else { (assign5060_e3372 * (assign5060_e3371 * (var_dnm_dn12 / var_dnm))) }, if 0.0 == 0.0 && ((assign5060_e3371) as f64).is_finite() && ((assign5060_e3371) as f64).fract() == 0.0 { if assign5060_e3371 == 0.0 { 0.0 } else { (assign5060_e3371 * ((var_dnm).powf(assign5060_e3371 - 1.0) * var_dnm_dn17)) } } else { (assign5060_e3372 * (assign5060_e3371 * (var_dnm_dn17 / var_dnm))) },)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign5060_e3374;
        var_dnm_dn0 = assign5060_e3374_d_n0;
        var_dnm_dn2 = assign5060_e3374_d_n2;
        var_dnm_dn6 = assign5060_e3374_d_n6;
        var_dnm_dn7 = assign5060_e3374_d_n7;
        var_dnm_dn10 = assign5060_e3374_d_n10;
        var_dnm_dn11 = assign5060_e3374_d_n11;
        var_dnm_dn12 = assign5060_e3374_d_n12;
        var_dnm_dn17 = assign5060_e3374_d_n17;
        var_dnm_rv = 0.0;

        let (assign5070_e3380, assign5070_e3380_d_n0, assign5070_e3380_d_n2, assign5070_e3380_d_n6, assign5070_e3380_d_n7, assign5070_e3380_d_n10, assign5070_e3380_d_n11, assign5070_e3380_d_n12, assign5070_e3380_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign5070_e3378: f64 = (1.0 / var_dnm);
        (assign5070_e3378, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn11 / (var_dnm * var_dnm))), (-(var_dnm_dn12 / (var_dnm * var_dnm))), (-(var_dnm_dn17 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign5070_e3380;
        var_dnm_dn0 = assign5070_e3380_d_n0;
        var_dnm_dn2 = assign5070_e3380_d_n2;
        var_dnm_dn6 = assign5070_e3380_d_n6;
        var_dnm_dn7 = assign5070_e3380_d_n7;
        var_dnm_dn10 = assign5070_e3380_d_n10;
        var_dnm_dn11 = assign5070_e3380_d_n11;
        var_dnm_dn12 = assign5070_e3380_d_n12;
        var_dnm_dn17 = assign5070_e3380_d_n17;
        var_dnm_rv = 0.0;

        let (assign5080_e3388, assign5080_e3388_d_n0, assign5080_e3388_d_n2, assign5080_e3388_d_n6, assign5080_e3388_d_n7, assign5080_e3388_d_n10, assign5080_e3388_d_n11, assign5080_e3388_d_n12, assign5080_e3388_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign5080_e3384: f64 = (var_t2 * var_t3);
        let assign5080_e3386: f64 = (assign5080_e3384 * var_dnm);
        (assign5080_e3386, ((((var_t2_dn0 * var_t3) + (var_t2 * var_t3_dn0)) * var_dnm) + (assign5080_e3384 * var_dnm_dn0)), ((((var_t2_dn2 * var_t3) + (var_t2 * var_t3_dn2)) * var_dnm) + (assign5080_e3384 * var_dnm_dn2)), ((((var_t2_dn6 * var_t3) + (var_t2 * var_t3_dn6)) * var_dnm) + (assign5080_e3384 * var_dnm_dn6)), ((((var_t2_dn7 * var_t3) + (var_t2 * var_t3_dn7)) * var_dnm) + (assign5080_e3384 * var_dnm_dn7)), ((((var_t2_dn10 * var_t3) + (var_t2 * var_t3_dn10)) * var_dnm) + (assign5080_e3384 * var_dnm_dn10)), ((((var_t2_dn11 * var_t3) + (var_t2 * var_t3_dn11)) * var_dnm) + (assign5080_e3384 * var_dnm_dn11)), ((((var_t2_dn12 * var_t3) + (var_t2 * var_t3_dn12)) * var_dnm) + (assign5080_e3384 * var_dnm_dn12)), ((((var_t2_dn17 * var_t3) + (var_t2 * var_t3_dn17)) * var_dnm) + (assign5080_e3384 * var_dnm_dn17)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign5080_e3388;
        var_t4_dn0 = assign5080_e3388_d_n0;
        var_t4_dn2 = assign5080_e3388_d_n2;
        var_t4_dn6 = assign5080_e3388_d_n6;
        var_t4_dn7 = assign5080_e3388_d_n7;
        var_t4_dn10 = assign5080_e3388_d_n10;
        var_t4_dn11 = assign5080_e3388_d_n11;
        var_t4_dn12 = assign5080_e3388_d_n12;
        var_t4_dn17 = assign5080_e3388_d_n17;
        var_t4_rv = 0.0;

        let (assign5090_e3398, assign5090_e3398_d_n0, assign5090_e3398_d_n2, assign5090_e3398_d_n6, assign5090_e3398_d_n7, assign5090_e3398_d_n10, assign5090_e3398_d_n11, assign5090_e3398_d_n12, assign5090_e3398_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign5090_e3392: f64 = (var_t3 * var_xmp);
        let assign5090_e3394: f64 = (assign5090_e3392 * var_dnm);
        let assign5090_e3396: f64 = (assign5090_e3394 / var_arg);
        (assign5090_e3396, (((((((var_t3_dn0 * var_xmp) + (var_t3 * var_xmp_dn0)) * var_dnm) + (assign5090_e3392 * var_dnm_dn0)) * var_arg) - (assign5090_e3394 * var_arg_dn0)) / (var_arg * var_arg)), (((((((var_t3_dn2 * var_xmp) + (var_t3 * var_xmp_dn2)) * var_dnm) + (assign5090_e3392 * var_dnm_dn2)) * var_arg) - (assign5090_e3394 * var_arg_dn2)) / (var_arg * var_arg)), (((((((var_t3_dn6 * var_xmp) + (var_t3 * var_xmp_dn6)) * var_dnm) + (assign5090_e3392 * var_dnm_dn6)) * var_arg) - (assign5090_e3394 * var_arg_dn6)) / (var_arg * var_arg)), (((((((var_t3_dn7 * var_xmp) + (var_t3 * var_xmp_dn7)) * var_dnm) + (assign5090_e3392 * var_dnm_dn7)) * var_arg) - (assign5090_e3394 * var_arg_dn7)) / (var_arg * var_arg)), (((((((var_t3_dn10 * var_xmp) + (var_t3 * var_xmp_dn10)) * var_dnm) + (assign5090_e3392 * var_dnm_dn10)) * var_arg) - (assign5090_e3394 * var_arg_dn10)) / (var_arg * var_arg)), (((((((var_t3_dn11 * var_xmp) + (var_t3 * var_xmp_dn11)) * var_dnm) + (assign5090_e3392 * var_dnm_dn11)) * var_arg) - (assign5090_e3394 * var_arg_dn11)) / (var_arg * var_arg)), (((((((var_t3_dn12 * var_xmp) + (var_t3 * var_xmp_dn12)) * var_dnm) + (assign5090_e3392 * var_dnm_dn12)) * var_arg) - (assign5090_e3394 * var_arg_dn12)) / (var_arg * var_arg)), (((((((var_t3_dn17 * var_xmp) + (var_t3 * var_xmp_dn17)) * var_dnm) + (assign5090_e3392 * var_dnm_dn17)) * var_arg) - (assign5090_e3394 * var_arg_dn17)) / (var_arg * var_arg)),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn6, var_t8_dn7, var_t8_dn10, var_t8_dn11, var_t8_dn12, var_t8_dn17,)
    }
};
        var_t8 = assign5090_e3398;
        var_t8_dn0 = assign5090_e3398_d_n0;
        var_t8_dn2 = assign5090_e3398_d_n2;
        var_t8_dn6 = assign5090_e3398_d_n6;
        var_t8_dn7 = assign5090_e3398_d_n7;
        var_t8_dn10 = assign5090_e3398_d_n10;
        var_t8_dn11 = assign5090_e3398_d_n11;
        var_t8_dn12 = assign5090_e3398_d_n12;
        var_t8_dn17 = assign5090_e3398_d_n17;
        var_t8_rv = 0.0;

        let (assign5100_e3404, assign5100_e3404_d_n0, assign5100_e3404_d_n2, assign5100_e3404_d_n6, assign5100_e3404_d_n7, assign5100_e3404_d_n10, assign5100_e3404_d_n11, assign5100_e3404_d_n12, assign5100_e3404_d_n17,) = {
    if (var_guard52 != 0.0) {
        let assign5100_e3402: f64 = (var_vbs_bnd + var_t4);
        (assign5100_e3402, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    } else {
        (var_vbsc, var_vbsc_dn0, var_vbsc_dn2, var_vbsc_dn6, var_vbsc_dn7, var_vbsc_dn10, var_vbsc_dn11, var_vbsc_dn12, var_vbsc_dn17,)
    }
};
        var_vbsc = assign5100_e3404;
        var_vbsc_dn0 = assign5100_e3404_d_n0;
        var_vbsc_dn2 = assign5100_e3404_d_n2;
        var_vbsc_dn6 = assign5100_e3404_d_n6;
        var_vbsc_dn7 = assign5100_e3404_d_n7;
        var_vbsc_dn10 = assign5100_e3404_d_n10;
        var_vbsc_dn11 = assign5100_e3404_d_n11;
        var_vbsc_dn12 = assign5100_e3404_d_n12;
        var_vbsc_dn17 = assign5100_e3404_d_n17;
        var_vbsc_rv = 0.0;

        let (assign5110_e3408, assign5110_e3408_d_n0, assign5110_e3408_d_n2, assign5110_e3408_d_n6, assign5110_e3408_d_n7, assign5110_e3408_d_n10, assign5110_e3408_d_n11, assign5110_e3408_d_n12, assign5110_e3408_d_n17,) = {
    if (var_guard52 != 0.0) {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn6, var_t8_dn7, var_t8_dn10, var_t8_dn11, var_t8_dn12, var_t8_dn17,)
    } else {
        (var_vbsc_dvbse, var_vbsc_dvbse_dn0, var_vbsc_dvbse_dn2, var_vbsc_dvbse_dn6, var_vbsc_dvbse_dn7, var_vbsc_dvbse_dn10, var_vbsc_dvbse_dn11, var_vbsc_dvbse_dn12, var_vbsc_dvbse_dn17,)
    }
};
        var_vbsc_dvbse = assign5110_e3408;
        var_vbsc_dvbse_dn0 = assign5110_e3408_d_n0;
        var_vbsc_dvbse_dn2 = assign5110_e3408_d_n2;
        var_vbsc_dvbse_dn6 = assign5110_e3408_d_n6;
        var_vbsc_dvbse_dn7 = assign5110_e3408_d_n7;
        var_vbsc_dvbse_dn10 = assign5110_e3408_d_n10;
        var_vbsc_dvbse_dn11 = assign5110_e3408_d_n11;
        var_vbsc_dvbse_dn12 = assign5110_e3408_d_n12;
        var_vbsc_dvbse_dn17 = assign5110_e3408_d_n17;
        var_vbsc_dvbse_rv = 0.0;

        let (assign5120_e3413, assign5120_e3413_d_n0, assign5120_e3413_d_n2, assign5120_e3413_d_n6, assign5120_e3413_d_n7, assign5120_e3413_d_n10, assign5120_e3413_d_n11, assign5120_e3413_d_n12, assign5120_e3413_d_n17,) = {
    if (var_guard52 == 0.0) {
        (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    } else {
        (var_vbsc, var_vbsc_dn0, var_vbsc_dn2, var_vbsc_dn6, var_vbsc_dn7, var_vbsc_dn10, var_vbsc_dn11, var_vbsc_dn12, var_vbsc_dn17,)
    }
};
        var_vbsc = assign5120_e3413;
        var_vbsc_dn0 = assign5120_e3413_d_n0;
        var_vbsc_dn2 = assign5120_e3413_d_n2;
        var_vbsc_dn6 = assign5120_e3413_d_n6;
        var_vbsc_dn7 = assign5120_e3413_d_n7;
        var_vbsc_dn10 = assign5120_e3413_d_n10;
        var_vbsc_dn11 = assign5120_e3413_d_n11;
        var_vbsc_dn12 = assign5120_e3413_d_n12;
        var_vbsc_dn17 = assign5120_e3413_d_n17;
        var_vbsc_rv = 0.0;

        let (assign5130_e3418, assign5130_e3418_d_n0, assign5130_e3418_d_n2, assign5130_e3418_d_n6, assign5130_e3418_d_n7, assign5130_e3418_d_n10, assign5130_e3418_d_n11, assign5130_e3418_d_n12, assign5130_e3418_d_n17,) = {
    if (var_guard52 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vbsc_dvbse, var_vbsc_dvbse_dn0, var_vbsc_dvbse_dn2, var_vbsc_dvbse_dn6, var_vbsc_dvbse_dn7, var_vbsc_dvbse_dn10, var_vbsc_dvbse_dn11, var_vbsc_dvbse_dn12, var_vbsc_dvbse_dn17,)
    }
};
        var_vbsc_dvbse = assign5130_e3418;
        var_vbsc_dvbse_dn0 = assign5130_e3418_d_n0;
        var_vbsc_dvbse_dn2 = assign5130_e3418_d_n2;
        var_vbsc_dvbse_dn6 = assign5130_e3418_d_n6;
        var_vbsc_dvbse_dn7 = assign5130_e3418_d_n7;
        var_vbsc_dvbse_dn10 = assign5130_e3418_d_n10;
        var_vbsc_dvbse_dn11 = assign5130_e3418_d_n11;
        var_vbsc_dvbse_dn12 = assign5130_e3418_d_n12;
        var_vbsc_dvbse_dn17 = assign5130_e3418_d_n17;
        var_vbsc_dvbse_rv = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn11_slot = var_arg_dn11;
        *var_arg_dn12_slot = var_arg_dn12;
        *var_arg_dn17_slot = var_arg_dn17;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_rv_slot = var_arg_rv;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn11_slot = var_dnm_dn11;
        *var_dnm_dn12_slot = var_dnm_dn12;
        *var_dnm_dn17_slot = var_dnm_dn17;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_rv_slot = var_dnm_rv;
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
        *var_m0_slot = var_m0;
        *var_m0_rv_slot = var_m0_rv;
        *var_mm_slot = var_mm;
        *var_mm_rv_slot = var_mm_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn17_slot = var_t4_dn17;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_rv_slot = var_t4_rv;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn11_slot = var_t8_dn11;
        *var_t8_dn12_slot = var_t8_dn12;
        *var_t8_dn17_slot = var_t8_dn17;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_rv_slot = var_t8_rv;
        *var_vbsc_slot = var_vbsc;
        *var_vbsc_dn0_slot = var_vbsc_dn0;
        *var_vbsc_dn10_slot = var_vbsc_dn10;
        *var_vbsc_dn11_slot = var_vbsc_dn11;
        *var_vbsc_dn12_slot = var_vbsc_dn12;
        *var_vbsc_dn17_slot = var_vbsc_dn17;
        *var_vbsc_dn2_slot = var_vbsc_dn2;
        *var_vbsc_dn6_slot = var_vbsc_dn6;
        *var_vbsc_dn7_slot = var_vbsc_dn7;
        *var_vbsc_dvbse_slot = var_vbsc_dvbse;
        *var_vbsc_dvbse_dn0_slot = var_vbsc_dvbse_dn0;
        *var_vbsc_dvbse_dn10_slot = var_vbsc_dvbse_dn10;
        *var_vbsc_dvbse_dn11_slot = var_vbsc_dvbse_dn11;
        *var_vbsc_dvbse_dn12_slot = var_vbsc_dvbse_dn12;
        *var_vbsc_dvbse_dn17_slot = var_vbsc_dvbse_dn17;
        *var_vbsc_dvbse_dn2_slot = var_vbsc_dvbse_dn2;
        *var_vbsc_dvbse_dn6_slot = var_vbsc_dvbse_dn6;
        *var_vbsc_dvbse_dn7_slot = var_vbsc_dvbse_dn7;
        *var_vbsc_dvbse_rv_slot = var_vbsc_dvbse_rv;
        *var_vbsc_rv_slot = var_vbsc_rv;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn11_slot = var_xmp_dn11;
        *var_xmp_dn12_slot = var_xmp_dn12;
        *var_xmp_dn17_slot = var_xmp_dn17;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn7_slot = var_xmp_dn7;
        *var_xmp_rv_slot = var_xmp_rv;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn11_slot = var_xp_dn11;
        *var_xp_dn12_slot = var_xp_dn12;
        *var_xp_dn17_slot = var_xp_dn17;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_rv_slot = var_xp_rv;
    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        var_beta_inv: f64,
        var_beta_inv_dn10: f64,
        var_c_fox0_inv: f64,
        var_pb2: f64,
        var_pb2_dn0: f64,
        var_pb2_dn10: f64,
        var_pb2_dn11: f64,
        var_pb2_dn12: f64,
        var_pb2_dn17: f64,
        var_pb2_dn2: f64,
        var_pb2_dn6: f64,
        var_pb2_dn7: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn11: f64,
        var_q_nsub_dn12: f64,
        var_q_nsub_dn17: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn7: f64,
        var_subversion: f64,
        var_vbsc_dvbse: f64,
        var_vbsc_dvbse_dn0: f64,
        var_vbsc_dvbse_dn10: f64,
        var_vbsc_dvbse_dn11: f64,
        var_vbsc_dvbse_dn12: f64,
        var_vbsc_dvbse_dn17: f64,
        var_vbsc_dvbse_dn2: f64,
        var_vbsc_dvbse_dn6: f64,
        var_vbsc_dvbse_dn7: f64,
        var_vfb: f64,
        var_ai_slot: &mut f64,
        var_ai_dn0_slot: &mut f64,
        var_ai_dn10_slot: &mut f64,
        var_ai_dn11_slot: &mut f64,
        var_ai_dn12_slot: &mut f64,
        var_ai_dn17_slot: &mut f64,
        var_ai_dn2_slot: &mut f64,
        var_ai_dn6_slot: &mut f64,
        var_ai_dn7_slot: &mut f64,
        var_ai_rv_slot: &mut f64,
        var_c2_slot: &mut f64,
        var_c2_dn0_slot: &mut f64,
        var_c2_dn10_slot: &mut f64,
        var_c2_dn11_slot: &mut f64,
        var_c2_dn12_slot: &mut f64,
        var_c2_dn17_slot: &mut f64,
        var_c2_dn2_slot: &mut f64,
        var_c2_dn6_slot: &mut f64,
        var_c2_dn7_slot: &mut f64,
        var_c2_rv_slot: &mut f64,
        var_db_slot: &mut f64,
        var_db_dn0_slot: &mut f64,
        var_db_dn10_slot: &mut f64,
        var_db_dn11_slot: &mut f64,
        var_db_dn12_slot: &mut f64,
        var_db_dn17_slot: &mut f64,
        var_db_dn2_slot: &mut f64,
        var_db_dn6_slot: &mut f64,
        var_db_dn7_slot: &mut f64,
        var_db_rv_slot: &mut f64,
        var_di_slot: &mut f64,
        var_di_dn0_slot: &mut f64,
        var_di_dn10_slot: &mut f64,
        var_di_dn11_slot: &mut f64,
        var_di_dn12_slot: &mut f64,
        var_di_dn17_slot: &mut f64,
        var_di_dn2_slot: &mut f64,
        var_di_dn6_slot: &mut f64,
        var_di_dn7_slot: &mut f64,
        var_di_rv_slot: &mut f64,
        var_flg_pprv_slot: &mut f64,
        var_flg_pprv_rv_slot: &mut f64,
        var_guard59_slot: &mut f64,
        var_guard59_rv_slot: &mut f64,
        var_guard60_slot: &mut f64,
        var_guard60_rv_slot: &mut f64,
        var_guard65_slot: &mut f64,
        var_guard65_rv_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_lp_s0_rv_slot: &mut f64,
        var_lp_sl_slot: &mut f64,
        var_lp_sl_rv_slot: &mut f64,
        var_pbs0_ini_slot: &mut f64,
        var_pbs0_ini_rv_slot: &mut f64,
        var_pbsl_ini_slot: &mut f64,
        var_pbsl_ini_rv_slot: &mut f64,
        var_psb0_ini_slot: &mut f64,
        var_psb0_ini_rv_slot: &mut f64,
        var_psbl_ini_slot: &mut f64,
        var_psbl_ini_rv_slot: &mut f64,
        var_pslsat_slot: &mut f64,
        var_pslsat_dn0_slot: &mut f64,
        var_pslsat_dn10_slot: &mut f64,
        var_pslsat_dn11_slot: &mut f64,
        var_pslsat_dn12_slot: &mut f64,
        var_pslsat_dn17_slot: &mut f64,
        var_pslsat_dn2_slot: &mut f64,
        var_pslsat_dn6_slot: &mut f64,
        var_pslsat_dn7_slot: &mut f64,
        var_pslsat_rv_slot: &mut f64,
        var_pss0_ini_slot: &mut f64,
        var_pss0_ini_rv_slot: &mut f64,
        var_pssl_ini_slot: &mut f64,
        var_pssl_ini_rv_slot: &mut f64,
        var_t1__blk58_slot: &mut f64,
        var_t1__blk58_dn0_slot: &mut f64,
        var_t1__blk58_dn10_slot: &mut f64,
        var_t1__blk58_dn11_slot: &mut f64,
        var_t1__blk58_dn12_slot: &mut f64,
        var_t1__blk58_dn17_slot: &mut f64,
        var_t1__blk58_dn2_slot: &mut f64,
        var_t1__blk58_dn6_slot: &mut f64,
        var_t1__blk58_dn7_slot: &mut f64,
        var_t1__blk58_rv_slot: &mut f64,
        var_t1__blk61_slot: &mut f64,
        var_t1__blk61_dn0_slot: &mut f64,
        var_t1__blk61_dn10_slot: &mut f64,
        var_t1__blk61_dn11_slot: &mut f64,
        var_t1__blk61_dn12_slot: &mut f64,
        var_t1__blk61_dn17_slot: &mut f64,
        var_t1__blk61_dn2_slot: &mut f64,
        var_t1__blk61_dn6_slot: &mut f64,
        var_t1__blk61_dn7_slot: &mut f64,
        var_t1__blk61_rv_slot: &mut f64,
        var_t2__blk62_slot: &mut f64,
        var_t2__blk62_dn11_slot: &mut f64,
        var_t2__blk62_dn6_slot: &mut f64,
        var_t2__blk62_dn7_slot: &mut f64,
        var_t2__blk62_rv_slot: &mut f64,
        var_t3__blk63_slot: &mut f64,
        var_t3__blk63_dn0_slot: &mut f64,
        var_t3__blk63_dn10_slot: &mut f64,
        var_t3__blk63_dn11_slot: &mut f64,
        var_t3__blk63_dn12_slot: &mut f64,
        var_t3__blk63_dn17_slot: &mut f64,
        var_t3__blk63_dn2_slot: &mut f64,
        var_t3__blk63_dn6_slot: &mut f64,
        var_t3__blk63_dn7_slot: &mut f64,
        var_t3__blk63_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn17_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_tx__blk64_slot: &mut f64,
        var_tx__blk64_dn0_slot: &mut f64,
        var_tx__blk64_dn10_slot: &mut f64,
        var_tx__blk64_dn11_slot: &mut f64,
        var_tx__blk64_dn12_slot: &mut f64,
        var_tx__blk64_dn17_slot: &mut f64,
        var_tx__blk64_dn2_slot: &mut f64,
        var_tx__blk64_dn6_slot: &mut f64,
        var_tx__blk64_dn7_slot: &mut f64,
        var_tx__blk64_rv_slot: &mut f64,
        var_vbs_slot: &mut f64,
        var_vbs_dn0_slot: &mut f64,
        var_vbs_dn10_slot: &mut f64,
        var_vbs_dn11_slot: &mut f64,
        var_vbs_dn12_slot: &mut f64,
        var_vbs_dn17_slot: &mut f64,
        var_vbs_dn2_slot: &mut f64,
        var_vbs_dn6_slot: &mut f64,
        var_vbs_dn7_slot: &mut f64,
        var_vbs_rv_slot: &mut f64,
        var_vbsc_slot: &mut f64,
        var_vbsc_dn0_slot: &mut f64,
        var_vbsc_dn10_slot: &mut f64,
        var_vbsc_dn11_slot: &mut f64,
        var_vbsc_dn12_slot: &mut f64,
        var_vbsc_dn17_slot: &mut f64,
        var_vbsc_dn2_slot: &mut f64,
        var_vbsc_dn6_slot: &mut f64,
        var_vbsc_dn7_slot: &mut f64,
        var_vbsc_rv_slot: &mut f64,
        var_vbsp_slot: &mut f64,
        var_vbsp_dn0_slot: &mut f64,
        var_vbsp_dn10_slot: &mut f64,
        var_vbsp_dn11_slot: &mut f64,
        var_vbsp_dn12_slot: &mut f64,
        var_vbsp_dn17_slot: &mut f64,
        var_vbsp_dn2_slot: &mut f64,
        var_vbsp_dn6_slot: &mut f64,
        var_vbsp_dn7_slot: &mut f64,
        var_vbsp_rv_slot: &mut f64,
        var_vbspz_slot: &mut f64,
        var_vbspz_dn0_slot: &mut f64,
        var_vbspz_dn10_slot: &mut f64,
        var_vbspz_dn11_slot: &mut f64,
        var_vbspz_dn12_slot: &mut f64,
        var_vbspz_dn17_slot: &mut f64,
        var_vbspz_dn2_slot: &mut f64,
        var_vbspz_dn6_slot: &mut f64,
        var_vbspz_dn7_slot: &mut f64,
        var_vbspz_rv_slot: &mut f64,
        var_vbsz_slot: &mut f64,
        var_vbsz_dn0_slot: &mut f64,
        var_vbsz_dn10_slot: &mut f64,
        var_vbsz_dn11_slot: &mut f64,
        var_vbsz_dn12_slot: &mut f64,
        var_vbsz_dn17_slot: &mut f64,
        var_vbsz_dn2_slot: &mut f64,
        var_vbsz_dn6_slot: &mut f64,
        var_vbsz_dn7_slot: &mut f64,
        var_vbsz_rv_slot: &mut f64,
        var_vds_slot: &mut f64,
        var_vds_dn0_slot: &mut f64,
        var_vds_dn10_slot: &mut f64,
        var_vds_dn11_slot: &mut f64,
        var_vds_dn12_slot: &mut f64,
        var_vds_dn17_slot: &mut f64,
        var_vds_dn2_slot: &mut f64,
        var_vds_dn6_slot: &mut f64,
        var_vds_dn7_slot: &mut f64,
        var_vds_rv_slot: &mut f64,
        var_vdsats_slot: &mut f64,
        var_vdsats_dn0_slot: &mut f64,
        var_vdsats_dn10_slot: &mut f64,
        var_vdsats_dn11_slot: &mut f64,
        var_vdsats_dn12_slot: &mut f64,
        var_vdsats_dn17_slot: &mut f64,
        var_vdsats_dn2_slot: &mut f64,
        var_vdsats_dn6_slot: &mut f64,
        var_vdsats_dn7_slot: &mut f64,
        var_vdsats_rv_slot: &mut f64,
        var_vdsc_slot: &mut f64,
        var_vdsc_dn0_slot: &mut f64,
        var_vdsc_dn10_slot: &mut f64,
        var_vdsc_dn11_slot: &mut f64,
        var_vdsc_dn12_slot: &mut f64,
        var_vdsc_dn17_slot: &mut f64,
        var_vdsc_dn2_slot: &mut f64,
        var_vdsc_dn6_slot: &mut f64,
        var_vdsc_dn7_slot: &mut f64,
        var_vdsc_rv_slot: &mut f64,
        var_vdsz_slot: &mut f64,
        var_vdsz_dn0_slot: &mut f64,
        var_vdsz_dn10_slot: &mut f64,
        var_vdsz_dn11_slot: &mut f64,
        var_vdsz_dn12_slot: &mut f64,
        var_vdsz_dn17_slot: &mut f64,
        var_vdsz_dn2_slot: &mut f64,
        var_vdsz_dn6_slot: &mut f64,
        var_vdsz_dn7_slot: &mut f64,
        var_vdsz_rv_slot: &mut f64,
        var_vgs_slot: &mut f64,
        var_vgs_dn11_slot: &mut f64,
        var_vgs_dn6_slot: &mut f64,
        var_vgs_dn7_slot: &mut f64,
        var_vgs_rv_slot: &mut f64,
        var_vgsc_slot: &mut f64,
        var_vgsc_dn11_slot: &mut f64,
        var_vgsc_dn6_slot: &mut f64,
        var_vgsc_dn7_slot: &mut f64,
        var_vgsc_rv_slot: &mut f64,
        var_vgsz_slot: &mut f64,
        var_vgsz_dn0_slot: &mut f64,
        var_vgsz_dn10_slot: &mut f64,
        var_vgsz_dn11_slot: &mut f64,
        var_vgsz_dn12_slot: &mut f64,
        var_vgsz_dn17_slot: &mut f64,
        var_vgsz_dn2_slot: &mut f64,
        var_vgsz_dn6_slot: &mut f64,
        var_vgsz_dn7_slot: &mut f64,
        var_vgsz_rv_slot: &mut f64,
        var_vzadd_slot: &mut f64,
        var_vzadd_dn0_slot: &mut f64,
        var_vzadd_dn10_slot: &mut f64,
        var_vzadd_dn11_slot: &mut f64,
        var_vzadd_dn12_slot: &mut f64,
        var_vzadd_dn17_slot: &mut f64,
        var_vzadd_dn2_slot: &mut f64,
        var_vzadd_dn6_slot: &mut f64,
        var_vzadd_dn7_slot: &mut f64,
        var_vzadd_rv_slot: &mut f64,
    ) {
        let mut var_ai: f64 = *var_ai_slot;
        let mut var_ai_dn0: f64 = *var_ai_dn0_slot;
        let mut var_ai_dn10: f64 = *var_ai_dn10_slot;
        let mut var_ai_dn11: f64 = *var_ai_dn11_slot;
        let mut var_ai_dn12: f64 = *var_ai_dn12_slot;
        let mut var_ai_dn17: f64 = *var_ai_dn17_slot;
        let mut var_ai_dn2: f64 = *var_ai_dn2_slot;
        let mut var_ai_dn6: f64 = *var_ai_dn6_slot;
        let mut var_ai_dn7: f64 = *var_ai_dn7_slot;
        let mut var_ai_rv: f64 = *var_ai_rv_slot;
        let mut var_c2: f64 = *var_c2_slot;
        let mut var_c2_dn0: f64 = *var_c2_dn0_slot;
        let mut var_c2_dn10: f64 = *var_c2_dn10_slot;
        let mut var_c2_dn11: f64 = *var_c2_dn11_slot;
        let mut var_c2_dn12: f64 = *var_c2_dn12_slot;
        let mut var_c2_dn17: f64 = *var_c2_dn17_slot;
        let mut var_c2_dn2: f64 = *var_c2_dn2_slot;
        let mut var_c2_dn6: f64 = *var_c2_dn6_slot;
        let mut var_c2_dn7: f64 = *var_c2_dn7_slot;
        let mut var_c2_rv: f64 = *var_c2_rv_slot;
        let mut var_db: f64 = *var_db_slot;
        let mut var_db_dn0: f64 = *var_db_dn0_slot;
        let mut var_db_dn10: f64 = *var_db_dn10_slot;
        let mut var_db_dn11: f64 = *var_db_dn11_slot;
        let mut var_db_dn12: f64 = *var_db_dn12_slot;
        let mut var_db_dn17: f64 = *var_db_dn17_slot;
        let mut var_db_dn2: f64 = *var_db_dn2_slot;
        let mut var_db_dn6: f64 = *var_db_dn6_slot;
        let mut var_db_dn7: f64 = *var_db_dn7_slot;
        let mut var_db_rv: f64 = *var_db_rv_slot;
        let mut var_di: f64 = *var_di_slot;
        let mut var_di_dn0: f64 = *var_di_dn0_slot;
        let mut var_di_dn10: f64 = *var_di_dn10_slot;
        let mut var_di_dn11: f64 = *var_di_dn11_slot;
        let mut var_di_dn12: f64 = *var_di_dn12_slot;
        let mut var_di_dn17: f64 = *var_di_dn17_slot;
        let mut var_di_dn2: f64 = *var_di_dn2_slot;
        let mut var_di_dn6: f64 = *var_di_dn6_slot;
        let mut var_di_dn7: f64 = *var_di_dn7_slot;
        let mut var_di_rv: f64 = *var_di_rv_slot;
        let mut var_flg_pprv: f64 = *var_flg_pprv_slot;
        let mut var_flg_pprv_rv: f64 = *var_flg_pprv_rv_slot;
        let mut var_guard59: f64 = *var_guard59_slot;
        let mut var_guard59_rv: f64 = *var_guard59_rv_slot;
        let mut var_guard60: f64 = *var_guard60_slot;
        let mut var_guard60_rv: f64 = *var_guard60_rv_slot;
        let mut var_guard65: f64 = *var_guard65_slot;
        let mut var_guard65_rv: f64 = *var_guard65_rv_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_lp_s0_rv: f64 = *var_lp_s0_rv_slot;
        let mut var_lp_sl: f64 = *var_lp_sl_slot;
        let mut var_lp_sl_rv: f64 = *var_lp_sl_rv_slot;
        let mut var_pbs0_ini: f64 = *var_pbs0_ini_slot;
        let mut var_pbs0_ini_rv: f64 = *var_pbs0_ini_rv_slot;
        let mut var_pbsl_ini: f64 = *var_pbsl_ini_slot;
        let mut var_pbsl_ini_rv: f64 = *var_pbsl_ini_rv_slot;
        let mut var_psb0_ini: f64 = *var_psb0_ini_slot;
        let mut var_psb0_ini_rv: f64 = *var_psb0_ini_rv_slot;
        let mut var_psbl_ini: f64 = *var_psbl_ini_slot;
        let mut var_psbl_ini_rv: f64 = *var_psbl_ini_rv_slot;
        let mut var_pslsat: f64 = *var_pslsat_slot;
        let mut var_pslsat_dn0: f64 = *var_pslsat_dn0_slot;
        let mut var_pslsat_dn10: f64 = *var_pslsat_dn10_slot;
        let mut var_pslsat_dn11: f64 = *var_pslsat_dn11_slot;
        let mut var_pslsat_dn12: f64 = *var_pslsat_dn12_slot;
        let mut var_pslsat_dn17: f64 = *var_pslsat_dn17_slot;
        let mut var_pslsat_dn2: f64 = *var_pslsat_dn2_slot;
        let mut var_pslsat_dn6: f64 = *var_pslsat_dn6_slot;
        let mut var_pslsat_dn7: f64 = *var_pslsat_dn7_slot;
        let mut var_pslsat_rv: f64 = *var_pslsat_rv_slot;
        let mut var_pss0_ini: f64 = *var_pss0_ini_slot;
        let mut var_pss0_ini_rv: f64 = *var_pss0_ini_rv_slot;
        let mut var_pssl_ini: f64 = *var_pssl_ini_slot;
        let mut var_pssl_ini_rv: f64 = *var_pssl_ini_rv_slot;
        let mut var_t1__blk58: f64 = *var_t1__blk58_slot;
        let mut var_t1__blk58_dn0: f64 = *var_t1__blk58_dn0_slot;
        let mut var_t1__blk58_dn10: f64 = *var_t1__blk58_dn10_slot;
        let mut var_t1__blk58_dn11: f64 = *var_t1__blk58_dn11_slot;
        let mut var_t1__blk58_dn12: f64 = *var_t1__blk58_dn12_slot;
        let mut var_t1__blk58_dn17: f64 = *var_t1__blk58_dn17_slot;
        let mut var_t1__blk58_dn2: f64 = *var_t1__blk58_dn2_slot;
        let mut var_t1__blk58_dn6: f64 = *var_t1__blk58_dn6_slot;
        let mut var_t1__blk58_dn7: f64 = *var_t1__blk58_dn7_slot;
        let mut var_t1__blk58_rv: f64 = *var_t1__blk58_rv_slot;
        let mut var_t1__blk61: f64 = *var_t1__blk61_slot;
        let mut var_t1__blk61_dn0: f64 = *var_t1__blk61_dn0_slot;
        let mut var_t1__blk61_dn10: f64 = *var_t1__blk61_dn10_slot;
        let mut var_t1__blk61_dn11: f64 = *var_t1__blk61_dn11_slot;
        let mut var_t1__blk61_dn12: f64 = *var_t1__blk61_dn12_slot;
        let mut var_t1__blk61_dn17: f64 = *var_t1__blk61_dn17_slot;
        let mut var_t1__blk61_dn2: f64 = *var_t1__blk61_dn2_slot;
        let mut var_t1__blk61_dn6: f64 = *var_t1__blk61_dn6_slot;
        let mut var_t1__blk61_dn7: f64 = *var_t1__blk61_dn7_slot;
        let mut var_t1__blk61_rv: f64 = *var_t1__blk61_rv_slot;
        let mut var_t2__blk62: f64 = *var_t2__blk62_slot;
        let mut var_t2__blk62_dn11: f64 = *var_t2__blk62_dn11_slot;
        let mut var_t2__blk62_dn6: f64 = *var_t2__blk62_dn6_slot;
        let mut var_t2__blk62_dn7: f64 = *var_t2__blk62_dn7_slot;
        let mut var_t2__blk62_rv: f64 = *var_t2__blk62_rv_slot;
        let mut var_t3__blk63: f64 = *var_t3__blk63_slot;
        let mut var_t3__blk63_dn0: f64 = *var_t3__blk63_dn0_slot;
        let mut var_t3__blk63_dn10: f64 = *var_t3__blk63_dn10_slot;
        let mut var_t3__blk63_dn11: f64 = *var_t3__blk63_dn11_slot;
        let mut var_t3__blk63_dn12: f64 = *var_t3__blk63_dn12_slot;
        let mut var_t3__blk63_dn17: f64 = *var_t3__blk63_dn17_slot;
        let mut var_t3__blk63_dn2: f64 = *var_t3__blk63_dn2_slot;
        let mut var_t3__blk63_dn6: f64 = *var_t3__blk63_dn6_slot;
        let mut var_t3__blk63_dn7: f64 = *var_t3__blk63_dn7_slot;
        let mut var_t3__blk63_rv: f64 = *var_t3__blk63_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn17: f64 = *var_t4_dn17_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_tx__blk64: f64 = *var_tx__blk64_slot;
        let mut var_tx__blk64_dn0: f64 = *var_tx__blk64_dn0_slot;
        let mut var_tx__blk64_dn10: f64 = *var_tx__blk64_dn10_slot;
        let mut var_tx__blk64_dn11: f64 = *var_tx__blk64_dn11_slot;
        let mut var_tx__blk64_dn12: f64 = *var_tx__blk64_dn12_slot;
        let mut var_tx__blk64_dn17: f64 = *var_tx__blk64_dn17_slot;
        let mut var_tx__blk64_dn2: f64 = *var_tx__blk64_dn2_slot;
        let mut var_tx__blk64_dn6: f64 = *var_tx__blk64_dn6_slot;
        let mut var_tx__blk64_dn7: f64 = *var_tx__blk64_dn7_slot;
        let mut var_tx__blk64_rv: f64 = *var_tx__blk64_rv_slot;
        let mut var_vbs: f64 = *var_vbs_slot;
        let mut var_vbs_dn0: f64 = *var_vbs_dn0_slot;
        let mut var_vbs_dn10: f64 = *var_vbs_dn10_slot;
        let mut var_vbs_dn11: f64 = *var_vbs_dn11_slot;
        let mut var_vbs_dn12: f64 = *var_vbs_dn12_slot;
        let mut var_vbs_dn17: f64 = *var_vbs_dn17_slot;
        let mut var_vbs_dn2: f64 = *var_vbs_dn2_slot;
        let mut var_vbs_dn6: f64 = *var_vbs_dn6_slot;
        let mut var_vbs_dn7: f64 = *var_vbs_dn7_slot;
        let mut var_vbs_rv: f64 = *var_vbs_rv_slot;
        let mut var_vbsc: f64 = *var_vbsc_slot;
        let mut var_vbsc_dn0: f64 = *var_vbsc_dn0_slot;
        let mut var_vbsc_dn10: f64 = *var_vbsc_dn10_slot;
        let mut var_vbsc_dn11: f64 = *var_vbsc_dn11_slot;
        let mut var_vbsc_dn12: f64 = *var_vbsc_dn12_slot;
        let mut var_vbsc_dn17: f64 = *var_vbsc_dn17_slot;
        let mut var_vbsc_dn2: f64 = *var_vbsc_dn2_slot;
        let mut var_vbsc_dn6: f64 = *var_vbsc_dn6_slot;
        let mut var_vbsc_dn7: f64 = *var_vbsc_dn7_slot;
        let mut var_vbsc_rv: f64 = *var_vbsc_rv_slot;
        let mut var_vbsp: f64 = *var_vbsp_slot;
        let mut var_vbsp_dn0: f64 = *var_vbsp_dn0_slot;
        let mut var_vbsp_dn10: f64 = *var_vbsp_dn10_slot;
        let mut var_vbsp_dn11: f64 = *var_vbsp_dn11_slot;
        let mut var_vbsp_dn12: f64 = *var_vbsp_dn12_slot;
        let mut var_vbsp_dn17: f64 = *var_vbsp_dn17_slot;
        let mut var_vbsp_dn2: f64 = *var_vbsp_dn2_slot;
        let mut var_vbsp_dn6: f64 = *var_vbsp_dn6_slot;
        let mut var_vbsp_dn7: f64 = *var_vbsp_dn7_slot;
        let mut var_vbsp_rv: f64 = *var_vbsp_rv_slot;
        let mut var_vbspz: f64 = *var_vbspz_slot;
        let mut var_vbspz_dn0: f64 = *var_vbspz_dn0_slot;
        let mut var_vbspz_dn10: f64 = *var_vbspz_dn10_slot;
        let mut var_vbspz_dn11: f64 = *var_vbspz_dn11_slot;
        let mut var_vbspz_dn12: f64 = *var_vbspz_dn12_slot;
        let mut var_vbspz_dn17: f64 = *var_vbspz_dn17_slot;
        let mut var_vbspz_dn2: f64 = *var_vbspz_dn2_slot;
        let mut var_vbspz_dn6: f64 = *var_vbspz_dn6_slot;
        let mut var_vbspz_dn7: f64 = *var_vbspz_dn7_slot;
        let mut var_vbspz_rv: f64 = *var_vbspz_rv_slot;
        let mut var_vbsz: f64 = *var_vbsz_slot;
        let mut var_vbsz_dn0: f64 = *var_vbsz_dn0_slot;
        let mut var_vbsz_dn10: f64 = *var_vbsz_dn10_slot;
        let mut var_vbsz_dn11: f64 = *var_vbsz_dn11_slot;
        let mut var_vbsz_dn12: f64 = *var_vbsz_dn12_slot;
        let mut var_vbsz_dn17: f64 = *var_vbsz_dn17_slot;
        let mut var_vbsz_dn2: f64 = *var_vbsz_dn2_slot;
        let mut var_vbsz_dn6: f64 = *var_vbsz_dn6_slot;
        let mut var_vbsz_dn7: f64 = *var_vbsz_dn7_slot;
        let mut var_vbsz_rv: f64 = *var_vbsz_rv_slot;
        let mut var_vds: f64 = *var_vds_slot;
        let mut var_vds_dn0: f64 = *var_vds_dn0_slot;
        let mut var_vds_dn10: f64 = *var_vds_dn10_slot;
        let mut var_vds_dn11: f64 = *var_vds_dn11_slot;
        let mut var_vds_dn12: f64 = *var_vds_dn12_slot;
        let mut var_vds_dn17: f64 = *var_vds_dn17_slot;
        let mut var_vds_dn2: f64 = *var_vds_dn2_slot;
        let mut var_vds_dn6: f64 = *var_vds_dn6_slot;
        let mut var_vds_dn7: f64 = *var_vds_dn7_slot;
        let mut var_vds_rv: f64 = *var_vds_rv_slot;
        let mut var_vdsats: f64 = *var_vdsats_slot;
        let mut var_vdsats_dn0: f64 = *var_vdsats_dn0_slot;
        let mut var_vdsats_dn10: f64 = *var_vdsats_dn10_slot;
        let mut var_vdsats_dn11: f64 = *var_vdsats_dn11_slot;
        let mut var_vdsats_dn12: f64 = *var_vdsats_dn12_slot;
        let mut var_vdsats_dn17: f64 = *var_vdsats_dn17_slot;
        let mut var_vdsats_dn2: f64 = *var_vdsats_dn2_slot;
        let mut var_vdsats_dn6: f64 = *var_vdsats_dn6_slot;
        let mut var_vdsats_dn7: f64 = *var_vdsats_dn7_slot;
        let mut var_vdsats_rv: f64 = *var_vdsats_rv_slot;
        let mut var_vdsc: f64 = *var_vdsc_slot;
        let mut var_vdsc_dn0: f64 = *var_vdsc_dn0_slot;
        let mut var_vdsc_dn10: f64 = *var_vdsc_dn10_slot;
        let mut var_vdsc_dn11: f64 = *var_vdsc_dn11_slot;
        let mut var_vdsc_dn12: f64 = *var_vdsc_dn12_slot;
        let mut var_vdsc_dn17: f64 = *var_vdsc_dn17_slot;
        let mut var_vdsc_dn2: f64 = *var_vdsc_dn2_slot;
        let mut var_vdsc_dn6: f64 = *var_vdsc_dn6_slot;
        let mut var_vdsc_dn7: f64 = *var_vdsc_dn7_slot;
        let mut var_vdsc_rv: f64 = *var_vdsc_rv_slot;
        let mut var_vdsz: f64 = *var_vdsz_slot;
        let mut var_vdsz_dn0: f64 = *var_vdsz_dn0_slot;
        let mut var_vdsz_dn10: f64 = *var_vdsz_dn10_slot;
        let mut var_vdsz_dn11: f64 = *var_vdsz_dn11_slot;
        let mut var_vdsz_dn12: f64 = *var_vdsz_dn12_slot;
        let mut var_vdsz_dn17: f64 = *var_vdsz_dn17_slot;
        let mut var_vdsz_dn2: f64 = *var_vdsz_dn2_slot;
        let mut var_vdsz_dn6: f64 = *var_vdsz_dn6_slot;
        let mut var_vdsz_dn7: f64 = *var_vdsz_dn7_slot;
        let mut var_vdsz_rv: f64 = *var_vdsz_rv_slot;
        let mut var_vgs: f64 = *var_vgs_slot;
        let mut var_vgs_dn11: f64 = *var_vgs_dn11_slot;
        let mut var_vgs_dn6: f64 = *var_vgs_dn6_slot;
        let mut var_vgs_dn7: f64 = *var_vgs_dn7_slot;
        let mut var_vgs_rv: f64 = *var_vgs_rv_slot;
        let mut var_vgsc: f64 = *var_vgsc_slot;
        let mut var_vgsc_dn11: f64 = *var_vgsc_dn11_slot;
        let mut var_vgsc_dn6: f64 = *var_vgsc_dn6_slot;
        let mut var_vgsc_dn7: f64 = *var_vgsc_dn7_slot;
        let mut var_vgsc_rv: f64 = *var_vgsc_rv_slot;
        let mut var_vgsz: f64 = *var_vgsz_slot;
        let mut var_vgsz_dn0: f64 = *var_vgsz_dn0_slot;
        let mut var_vgsz_dn10: f64 = *var_vgsz_dn10_slot;
        let mut var_vgsz_dn11: f64 = *var_vgsz_dn11_slot;
        let mut var_vgsz_dn12: f64 = *var_vgsz_dn12_slot;
        let mut var_vgsz_dn17: f64 = *var_vgsz_dn17_slot;
        let mut var_vgsz_dn2: f64 = *var_vgsz_dn2_slot;
        let mut var_vgsz_dn6: f64 = *var_vgsz_dn6_slot;
        let mut var_vgsz_dn7: f64 = *var_vgsz_dn7_slot;
        let mut var_vgsz_rv: f64 = *var_vgsz_rv_slot;
        let mut var_vzadd: f64 = *var_vzadd_slot;
        let mut var_vzadd_dn0: f64 = *var_vzadd_dn0_slot;
        let mut var_vzadd_dn10: f64 = *var_vzadd_dn10_slot;
        let mut var_vzadd_dn11: f64 = *var_vzadd_dn11_slot;
        let mut var_vzadd_dn12: f64 = *var_vzadd_dn12_slot;
        let mut var_vzadd_dn17: f64 = *var_vzadd_dn17_slot;
        let mut var_vzadd_dn2: f64 = *var_vzadd_dn2_slot;
        let mut var_vzadd_dn6: f64 = *var_vzadd_dn6_slot;
        let mut var_vzadd_dn7: f64 = *var_vzadd_dn7_slot;
        let mut var_vzadd_rv: f64 = *var_vzadd_rv_slot;

        let (assign5140_e3424, assign5140_e3424_d_n0, assign5140_e3424_d_n2, assign5140_e3424_d_n6, assign5140_e3424_d_n7, assign5140_e3424_d_n10, assign5140_e3424_d_n11, assign5140_e3424_d_n12, assign5140_e3424_d_n17,) = {
    if (var_vds > 20.0) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vds, var_vds_dn0, var_vds_dn2, var_vds_dn6, var_vds_dn7, var_vds_dn10, var_vds_dn11, var_vds_dn12, var_vds_dn17,)
    }
};
        var_vdsc = assign5140_e3424;
        var_vdsc_dn0 = assign5140_e3424_d_n0;
        var_vdsc_dn2 = assign5140_e3424_d_n2;
        var_vdsc_dn6 = assign5140_e3424_d_n6;
        var_vdsc_dn7 = assign5140_e3424_d_n7;
        var_vdsc_dn10 = assign5140_e3424_d_n10;
        var_vdsc_dn11 = assign5140_e3424_d_n11;
        var_vdsc_dn12 = assign5140_e3424_d_n12;
        var_vdsc_dn17 = assign5140_e3424_d_n17;
        var_vdsc_rv = 0.0;

        let (assign5150_e3430, assign5150_e3430_d_n6, assign5150_e3430_d_n7, assign5150_e3430_d_n11,) = {
    if (var_vgs > 20.0) {
        (20.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vgs, var_vgs_dn6, var_vgs_dn7, var_vgs_dn11,)
    }
};
        var_vgsc = assign5150_e3430;
        var_vgsc_dn6 = assign5150_e3430_d_n6;
        var_vgsc_dn7 = assign5150_e3430_d_n7;
        var_vgsc_dn11 = assign5150_e3430_d_n11;
        var_vgsc_rv = 0.0;

        let assign5160_e3433: f64 = (-20.0);
        let (assign5160_e3438, assign5160_e3438_d_n6, assign5160_e3438_d_n7, assign5160_e3438_d_n11,) = {
    if (var_vgs < assign5160_e3433) {
        let assign5160_e3436: f64 = (-20.0);
        (assign5160_e3436, 0.0, 0.0, 0.0,)
    } else {
        (var_vgsc, var_vgsc_dn6, var_vgsc_dn7, var_vgsc_dn11,)
    }
};
        var_vgsc = assign5160_e3438;
        var_vgsc_dn6 = assign5160_e3438_d_n6;
        var_vgsc_dn7 = assign5160_e3438_d_n7;
        var_vgsc_dn11 = assign5160_e3438_d_n11;
        var_vgsc_rv = 0.0;

        let assign5170_e3441: f64 = (-20.0);
        let (assign5170_e3446, assign5170_e3446_d_n0, assign5170_e3446_d_n2, assign5170_e3446_d_n6, assign5170_e3446_d_n7, assign5170_e3446_d_n10, assign5170_e3446_d_n11, assign5170_e3446_d_n12, assign5170_e3446_d_n17,) = {
    if (var_vbsc < assign5170_e3441) {
        let assign5170_e3444: f64 = (-20.0);
        (assign5170_e3444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vbsc, var_vbsc_dn0, var_vbsc_dn2, var_vbsc_dn6, var_vbsc_dn7, var_vbsc_dn10, var_vbsc_dn11, var_vbsc_dn12, var_vbsc_dn17,)
    }
};
        var_vbsc = assign5170_e3446;
        var_vbsc_dn0 = assign5170_e3446_d_n0;
        var_vbsc_dn2 = assign5170_e3446_d_n2;
        var_vbsc_dn6 = assign5170_e3446_d_n6;
        var_vbsc_dn7 = assign5170_e3446_d_n7;
        var_vbsc_dn10 = assign5170_e3446_d_n10;
        var_vbsc_dn11 = assign5170_e3446_d_n11;
        var_vbsc_dn12 = assign5170_e3446_d_n12;
        var_vbsc_dn17 = assign5170_e3446_d_n17;
        var_vbsc_rv = 0.0;

        var_vds = var_vdsc;
        var_vds_dn0 = var_vdsc_dn0;
        var_vds_dn2 = var_vdsc_dn2;
        var_vds_dn6 = var_vdsc_dn6;
        var_vds_dn7 = var_vdsc_dn7;
        var_vds_dn10 = var_vdsc_dn10;
        var_vds_dn11 = var_vdsc_dn11;
        var_vds_dn12 = var_vdsc_dn12;
        var_vds_dn17 = var_vdsc_dn17;
        var_vds_rv = 0.0;

        var_vgs = var_vgsc;
        var_vgs_dn6 = var_vgsc_dn6;
        var_vgs_dn7 = var_vgsc_dn7;
        var_vgs_dn11 = var_vgsc_dn11;
        var_vgs_rv = 0.0;

        var_vbs = var_vbsc;
        var_vbs_dn0 = var_vbsc_dn0;
        var_vbs_dn2 = var_vbsc_dn2;
        var_vbs_dn6 = var_vbsc_dn6;
        var_vbs_dn7 = var_vbsc_dn7;
        var_vbs_dn10 = var_vbsc_dn10;
        var_vbs_dn11 = var_vbsc_dn11;
        var_vbs_dn12 = var_vbsc_dn12;
        var_vbs_dn17 = var_vbsc_dn17;
        var_vbs_rv = 0.0;

        var_flg_pprv = 0.0;
        var_flg_pprv_rv = 0.0;

        var_pss0_ini = 0.0;
        var_pss0_ini_rv = 0.0;

        var_pbs0_ini = 0.0;
        var_pbs0_ini_rv = 0.0;

        var_psb0_ini = 0.0;
        var_psb0_ini_rv = 0.0;

        var_pssl_ini = 0.0;
        var_pssl_ini_rv = 0.0;

        var_pbsl_ini = 0.0;
        var_pbsl_ini_rv = 0.0;

        var_psbl_ini = 0.0;
        var_psbl_ini_rv = 0.0;

        var_ai = 0.0;
        var_ai_dn0 = 0.0;
        var_ai_dn2 = 0.0;
        var_ai_dn6 = 0.0;
        var_ai_dn7 = 0.0;
        var_ai_dn10 = 0.0;
        var_ai_dn11 = 0.0;
        var_ai_dn12 = 0.0;
        var_ai_dn17 = 0.0;
        var_ai_rv = 0.0;

        var_db = 0.0;
        var_db_dn0 = 0.0;
        var_db_dn2 = 0.0;
        var_db_dn6 = 0.0;
        var_db_dn7 = 0.0;
        var_db_dn10 = 0.0;
        var_db_dn11 = 0.0;
        var_db_dn12 = 0.0;
        var_db_dn17 = 0.0;
        var_db_rv = 0.0;

        var_di = 0.0;
        var_di_dn0 = 0.0;
        var_di_dn2 = 0.0;
        var_di_dn6 = 0.0;
        var_di_dn7 = 0.0;
        var_di_dn10 = 0.0;
        var_di_dn11 = 0.0;
        var_di_dn12 = 0.0;
        var_di_dn17 = 0.0;
        var_di_rv = 0.0;

        var_c2 = 0.0;
        var_c2_dn0 = 0.0;
        var_c2_dn2 = 0.0;
        var_c2_dn6 = 0.0;
        var_c2_dn7 = 0.0;
        var_c2_dn10 = 0.0;
        var_c2_dn11 = 0.0;
        var_c2_dn12 = 0.0;
        var_c2_dn17 = 0.0;
        var_c2_rv = 0.0;

        var_lp_s0 = 0.0;
        var_lp_s0_rv = 0.0;

        var_lp_sl = 0.0;
        var_lp_sl_rv = 0.0;

        let assign5340_e3465: f64 = (var_vbsc_dvbse * var_vds);
        let assign5340_e3467: f64 = (assign5340_e3465 / 2.0);
        var_t1__blk58 = assign5340_e3467;
        var_t1__blk58_dn0 = (((var_vbsc_dvbse_dn0 * var_vds) + (var_vbsc_dvbse * var_vds_dn0)) / 2.0);
        var_t1__blk58_dn2 = (((var_vbsc_dvbse_dn2 * var_vds) + (var_vbsc_dvbse * var_vds_dn2)) / 2.0);
        var_t1__blk58_dn6 = (((var_vbsc_dvbse_dn6 * var_vds) + (var_vbsc_dvbse * var_vds_dn6)) / 2.0);
        var_t1__blk58_dn7 = (((var_vbsc_dvbse_dn7 * var_vds) + (var_vbsc_dvbse * var_vds_dn7)) / 2.0);
        var_t1__blk58_dn10 = (((var_vbsc_dvbse_dn10 * var_vds) + (var_vbsc_dvbse * var_vds_dn10)) / 2.0);
        var_t1__blk58_dn11 = (((var_vbsc_dvbse_dn11 * var_vds) + (var_vbsc_dvbse * var_vds_dn11)) / 2.0);
        var_t1__blk58_dn12 = (((var_vbsc_dvbse_dn12 * var_vds) + (var_vbsc_dvbse * var_vds_dn12)) / 2.0);
        var_t1__blk58_dn17 = (((var_vbsc_dvbse_dn17 * var_vds) + (var_vbsc_dvbse * var_vds_dn17)) / 2.0);
        var_t1__blk58_rv = 0.0;

        let assign5350_e3470: f64 = (2.0 * var_t1__blk58);
        let assign5350_e3472: f64 = (assign5350_e3470 / p.p226);
        var_tmf1 = assign5350_e3472;
        var_tmf1_dn0 = ((2.0 * var_t1__blk58_dn0) / p.p226);
        var_tmf1_dn2 = ((2.0 * var_t1__blk58_dn2) / p.p226);
        var_tmf1_dn6 = ((2.0 * var_t1__blk58_dn6) / p.p226);
        var_tmf1_dn7 = ((2.0 * var_t1__blk58_dn7) / p.p226);
        var_tmf1_dn10 = ((2.0 * var_t1__blk58_dn10) / p.p226);
        var_tmf1_dn11 = ((2.0 * var_t1__blk58_dn11) / p.p226);
        var_tmf1_dn12 = ((2.0 * var_t1__blk58_dn12) / p.p226);
        var_tmf1_dn17 = ((2.0 * var_t1__blk58_dn17) / p.p226);
        var_tmf1_rv = 0.0;

        let assign5360_e3477: f64 = (1.0 / 2.0);
        let assign5360_e3481: f64 = (1.0 / 6.0);
        let assign5360_e3485: f64 = (1.0 / 24.0);
        let assign5360_e3489: f64 = (1.0 / 120.0);
        let assign5360_e3493: f64 = (1.0 / 720.0);
        let assign5360_e3497: f64 = (1.0 / 5040.0);
        let assign5360_e3498: f64 = (var_tmf1 * assign5360_e3497);
        let assign5360_e3499: f64 = (assign5360_e3493 + assign5360_e3498);
        let assign5360_e3500: f64 = (var_tmf1 * assign5360_e3499);
        let assign5360_e3501: f64 = (assign5360_e3489 + assign5360_e3500);
        let assign5360_e3502: f64 = (var_tmf1 * assign5360_e3501);
        let assign5360_e3503: f64 = (assign5360_e3485 + assign5360_e3502);
        let assign5360_e3504: f64 = (var_tmf1 * assign5360_e3503);
        let assign5360_e3505: f64 = (assign5360_e3481 + assign5360_e3504);
        let assign5360_e3506: f64 = (var_tmf1 * assign5360_e3505);
        let assign5360_e3507: f64 = (assign5360_e3477 + assign5360_e3506);
        let assign5360_e3508: f64 = (var_tmf1 * assign5360_e3507);
        let assign5360_e3509: f64 = (1.0 + assign5360_e3508);
        var_tmf2 = assign5360_e3509;
        var_tmf2_dn0 = ((var_tmf1_dn0 * assign5360_e3507) + (var_tmf1 * ((var_tmf1_dn0 * assign5360_e3505) + (var_tmf1 * ((var_tmf1_dn0 * assign5360_e3503) + (var_tmf1 * ((var_tmf1_dn0 * assign5360_e3501) + (var_tmf1 * ((var_tmf1_dn0 * assign5360_e3499) + (var_tmf1 * (var_tmf1_dn0 * assign5360_e3497)))))))))));
        var_tmf2_dn2 = ((var_tmf1_dn2 * assign5360_e3507) + (var_tmf1 * ((var_tmf1_dn2 * assign5360_e3505) + (var_tmf1 * ((var_tmf1_dn2 * assign5360_e3503) + (var_tmf1 * ((var_tmf1_dn2 * assign5360_e3501) + (var_tmf1 * ((var_tmf1_dn2 * assign5360_e3499) + (var_tmf1 * (var_tmf1_dn2 * assign5360_e3497)))))))))));
        var_tmf2_dn6 = ((var_tmf1_dn6 * assign5360_e3507) + (var_tmf1 * ((var_tmf1_dn6 * assign5360_e3505) + (var_tmf1 * ((var_tmf1_dn6 * assign5360_e3503) + (var_tmf1 * ((var_tmf1_dn6 * assign5360_e3501) + (var_tmf1 * ((var_tmf1_dn6 * assign5360_e3499) + (var_tmf1 * (var_tmf1_dn6 * assign5360_e3497)))))))))));
        var_tmf2_dn7 = ((var_tmf1_dn7 * assign5360_e3507) + (var_tmf1 * ((var_tmf1_dn7 * assign5360_e3505) + (var_tmf1 * ((var_tmf1_dn7 * assign5360_e3503) + (var_tmf1 * ((var_tmf1_dn7 * assign5360_e3501) + (var_tmf1 * ((var_tmf1_dn7 * assign5360_e3499) + (var_tmf1 * (var_tmf1_dn7 * assign5360_e3497)))))))))));
        var_tmf2_dn10 = ((var_tmf1_dn10 * assign5360_e3507) + (var_tmf1 * ((var_tmf1_dn10 * assign5360_e3505) + (var_tmf1 * ((var_tmf1_dn10 * assign5360_e3503) + (var_tmf1 * ((var_tmf1_dn10 * assign5360_e3501) + (var_tmf1 * ((var_tmf1_dn10 * assign5360_e3499) + (var_tmf1 * (var_tmf1_dn10 * assign5360_e3497)))))))))));
        var_tmf2_dn11 = ((var_tmf1_dn11 * assign5360_e3507) + (var_tmf1 * ((var_tmf1_dn11 * assign5360_e3505) + (var_tmf1 * ((var_tmf1_dn11 * assign5360_e3503) + (var_tmf1 * ((var_tmf1_dn11 * assign5360_e3501) + (var_tmf1 * ((var_tmf1_dn11 * assign5360_e3499) + (var_tmf1 * (var_tmf1_dn11 * assign5360_e3497)))))))))));
        var_tmf2_dn12 = ((var_tmf1_dn12 * assign5360_e3507) + (var_tmf1 * ((var_tmf1_dn12 * assign5360_e3505) + (var_tmf1 * ((var_tmf1_dn12 * assign5360_e3503) + (var_tmf1 * ((var_tmf1_dn12 * assign5360_e3501) + (var_tmf1 * ((var_tmf1_dn12 * assign5360_e3499) + (var_tmf1 * (var_tmf1_dn12 * assign5360_e3497)))))))))));
        var_tmf2_dn17 = ((var_tmf1_dn17 * assign5360_e3507) + (var_tmf1 * ((var_tmf1_dn17 * assign5360_e3505) + (var_tmf1 * ((var_tmf1_dn17 * assign5360_e3503) + (var_tmf1 * ((var_tmf1_dn17 * assign5360_e3501) + (var_tmf1 * ((var_tmf1_dn17 * assign5360_e3499) + (var_tmf1 * (var_tmf1_dn17 * assign5360_e3497)))))))))));
        var_tmf2_rv = 0.0;

        let assign5370_e3512: f64 = (p.p226 / var_tmf2);
        var_vzadd = assign5370_e3512;
        var_vzadd_dn0 = (-((p.p226 * var_tmf2_dn0) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn2 = (-((p.p226 * var_tmf2_dn2) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn6 = (-((p.p226 * var_tmf2_dn6) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn7 = (-((p.p226 * var_tmf2_dn7) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn10 = (-((p.p226 * var_tmf2_dn10) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn11 = (-((p.p226 * var_tmf2_dn11) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn12 = (-((p.p226 * var_tmf2_dn12) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn17 = (-((p.p226 * var_tmf2_dn17) / (var_tmf2 * var_tmf2)));
        var_vzadd_rv = 0.0;

        let assign5380_e3515: f64 = if var_vzadd < 5e-12 { 1.0 } else { 0.0 };
        var_guard59 = assign5380_e3515;
        var_guard59_rv = 0.0;

        let (assign5390_e3519, assign5390_e3519_d_n0, assign5390_e3519_d_n2, assign5390_e3519_d_n6, assign5390_e3519_d_n7, assign5390_e3519_d_n10, assign5390_e3519_d_n11, assign5390_e3519_d_n12, assign5390_e3519_d_n17,) = {
    if (var_guard59 != 0.0) {
        (5e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vzadd, var_vzadd_dn0, var_vzadd_dn2, var_vzadd_dn6, var_vzadd_dn7, var_vzadd_dn10, var_vzadd_dn11, var_vzadd_dn12, var_vzadd_dn17,)
    }
};
        var_vzadd = assign5390_e3519;
        var_vzadd_dn0 = assign5390_e3519_d_n0;
        var_vzadd_dn2 = assign5390_e3519_d_n2;
        var_vzadd_dn6 = assign5390_e3519_d_n6;
        var_vzadd_dn7 = assign5390_e3519_d_n7;
        var_vzadd_dn10 = assign5390_e3519_d_n10;
        var_vzadd_dn11 = assign5390_e3519_d_n11;
        var_vzadd_dn12 = assign5390_e3519_d_n12;
        var_vzadd_dn17 = assign5390_e3519_d_n17;
        var_vzadd_rv = 0.0;

        let assign5400_e3522: f64 = (var_vbs + var_vzadd);
        var_vbsz = assign5400_e3522;
        var_vbsz_dn0 = (var_vbs_dn0 + var_vzadd_dn0);
        var_vbsz_dn2 = (var_vbs_dn2 + var_vzadd_dn2);
        var_vbsz_dn6 = (var_vbs_dn6 + var_vzadd_dn6);
        var_vbsz_dn7 = (var_vbs_dn7 + var_vzadd_dn7);
        var_vbsz_dn10 = (var_vbs_dn10 + var_vzadd_dn10);
        var_vbsz_dn11 = (var_vbs_dn11 + var_vzadd_dn11);
        var_vbsz_dn12 = (var_vbs_dn12 + var_vzadd_dn12);
        var_vbsz_dn17 = (var_vbs_dn17 + var_vzadd_dn17);
        var_vbsz_rv = 0.0;

        let assign5410_e3526: f64 = (2.0 * var_vzadd);
        let assign5410_e3527: f64 = (var_vds + assign5410_e3526);
        var_vdsz = assign5410_e3527;
        var_vdsz_dn0 = (var_vds_dn0 + (2.0 * var_vzadd_dn0));
        var_vdsz_dn2 = (var_vds_dn2 + (2.0 * var_vzadd_dn2));
        var_vdsz_dn6 = (var_vds_dn6 + (2.0 * var_vzadd_dn6));
        var_vdsz_dn7 = (var_vds_dn7 + (2.0 * var_vzadd_dn7));
        var_vdsz_dn10 = (var_vds_dn10 + (2.0 * var_vzadd_dn10));
        var_vdsz_dn11 = (var_vds_dn11 + (2.0 * var_vzadd_dn11));
        var_vdsz_dn12 = (var_vds_dn12 + (2.0 * var_vzadd_dn12));
        var_vdsz_dn17 = (var_vds_dn17 + (2.0 * var_vzadd_dn17));
        var_vdsz_rv = 0.0;

        let assign5420_e3530: f64 = (var_vgs + var_vzadd);
        var_vgsz = assign5420_e3530;
        var_vgsz_dn0 = var_vzadd_dn0;
        var_vgsz_dn2 = var_vzadd_dn2;
        var_vgsz_dn6 = (var_vgs_dn6 + var_vzadd_dn6);
        var_vgsz_dn7 = (var_vgs_dn7 + var_vzadd_dn7);
        var_vgsz_dn10 = var_vzadd_dn10;
        var_vgsz_dn11 = (var_vgs_dn11 + var_vzadd_dn11);
        var_vgsz_dn12 = var_vzadd_dn12;
        var_vgsz_dn17 = var_vzadd_dn17;
        var_vgsz_rv = 0.0;

        let assign5430_e3533: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard60 = assign5430_e3533;
        var_guard60_rv = 0.0;

        let (assign5440_e3537, assign5440_e3537_d_n0, assign5440_e3537_d_n2, assign5440_e3537_d_n6, assign5440_e3537_d_n7, assign5440_e3537_d_n10, assign5440_e3537_d_n11, assign5440_e3537_d_n12, assign5440_e3537_d_n17,) = {
    if (var_guard60 != 0.0) {
        (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    } else {
        (var_vbsp, var_vbsp_dn0, var_vbsp_dn2, var_vbsp_dn6, var_vbsp_dn7, var_vbsp_dn10, var_vbsp_dn11, var_vbsp_dn12, var_vbsp_dn17,)
    }
};
        var_vbsp = assign5440_e3537;
        var_vbsp_dn0 = assign5440_e3537_d_n0;
        var_vbsp_dn2 = assign5440_e3537_d_n2;
        var_vbsp_dn6 = assign5440_e3537_d_n6;
        var_vbsp_dn7 = assign5440_e3537_d_n7;
        var_vbsp_dn10 = assign5440_e3537_d_n10;
        var_vbsp_dn11 = assign5440_e3537_d_n11;
        var_vbsp_dn12 = assign5440_e3537_d_n12;
        var_vbsp_dn17 = assign5440_e3537_d_n17;
        var_vbsp_rv = 0.0;

        let (assign5450_e3541, assign5450_e3541_d_n0, assign5450_e3541_d_n2, assign5450_e3541_d_n6, assign5450_e3541_d_n7, assign5450_e3541_d_n10, assign5450_e3541_d_n11, assign5450_e3541_d_n12, assign5450_e3541_d_n17,) = {
    if (var_guard60 != 0.0) {
        (var_vbsz, var_vbsz_dn0, var_vbsz_dn2, var_vbsz_dn6, var_vbsz_dn7, var_vbsz_dn10, var_vbsz_dn11, var_vbsz_dn12, var_vbsz_dn17,)
    } else {
        (var_vbspz, var_vbspz_dn0, var_vbspz_dn2, var_vbspz_dn6, var_vbspz_dn7, var_vbspz_dn10, var_vbspz_dn11, var_vbspz_dn12, var_vbspz_dn17,)
    }
};
        var_vbspz = assign5450_e3541;
        var_vbspz_dn0 = assign5450_e3541_d_n0;
        var_vbspz_dn2 = assign5450_e3541_d_n2;
        var_vbspz_dn6 = assign5450_e3541_d_n6;
        var_vbspz_dn7 = assign5450_e3541_d_n7;
        var_vbspz_dn10 = assign5450_e3541_d_n10;
        var_vbspz_dn11 = assign5450_e3541_d_n11;
        var_vbspz_dn12 = assign5450_e3541_d_n12;
        var_vbspz_dn17 = assign5450_e3541_d_n17;
        var_vbspz_rv = 0.0;

        let (assign5460_e3551, assign5460_e3551_d_n0, assign5460_e3551_d_n2, assign5460_e3551_d_n6, assign5460_e3551_d_n7, assign5460_e3551_d_n10, assign5460_e3551_d_n11, assign5460_e3551_d_n12, assign5460_e3551_d_n17,) = {
    if (var_guard60 == 0.0) {
        let (assign5460_e3549, assign5460_e3549_d_n0, assign5460_e3549_d_n2, assign5460_e3549_d_n6, assign5460_e3549_d_n7, assign5460_e3549_d_n10, assign5460_e3549_d_n11, assign5460_e3549_d_n12, assign5460_e3549_d_n17,) = {
            if (var_subversion < 3.0) {
                (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign5460_e3549, assign5460_e3549_d_n0, assign5460_e3549_d_n2, assign5460_e3549_d_n6, assign5460_e3549_d_n7, assign5460_e3549_d_n10, assign5460_e3549_d_n11, assign5460_e3549_d_n12, assign5460_e3549_d_n17,)
    } else {
        (var_vbsp, var_vbsp_dn0, var_vbsp_dn2, var_vbsp_dn6, var_vbsp_dn7, var_vbsp_dn10, var_vbsp_dn11, var_vbsp_dn12, var_vbsp_dn17,)
    }
};
        var_vbsp = assign5460_e3551;
        var_vbsp_dn0 = assign5460_e3551_d_n0;
        var_vbsp_dn2 = assign5460_e3551_d_n2;
        var_vbsp_dn6 = assign5460_e3551_d_n6;
        var_vbsp_dn7 = assign5460_e3551_d_n7;
        var_vbsp_dn10 = assign5460_e3551_d_n10;
        var_vbsp_dn11 = assign5460_e3551_d_n11;
        var_vbsp_dn12 = assign5460_e3551_d_n12;
        var_vbsp_dn17 = assign5460_e3551_d_n17;
        var_vbsp_rv = 0.0;

        let (assign5470_e3561, assign5470_e3561_d_n0, assign5470_e3561_d_n2, assign5470_e3561_d_n6, assign5470_e3561_d_n7, assign5470_e3561_d_n10, assign5470_e3561_d_n11, assign5470_e3561_d_n12, assign5470_e3561_d_n17,) = {
    if (var_guard60 == 0.0) {
        let (assign5470_e3559, assign5470_e3559_d_n0, assign5470_e3559_d_n2, assign5470_e3559_d_n6, assign5470_e3559_d_n7, assign5470_e3559_d_n10, assign5470_e3559_d_n11, assign5470_e3559_d_n12, assign5470_e3559_d_n17,) = {
            if (var_subversion < 3.0) {
                (var_vbsz, var_vbsz_dn0, var_vbsz_dn2, var_vbsz_dn6, var_vbsz_dn7, var_vbsz_dn10, var_vbsz_dn11, var_vbsz_dn12, var_vbsz_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign5470_e3559, assign5470_e3559_d_n0, assign5470_e3559_d_n2, assign5470_e3559_d_n6, assign5470_e3559_d_n7, assign5470_e3559_d_n10, assign5470_e3559_d_n11, assign5470_e3559_d_n12, assign5470_e3559_d_n17,)
    } else {
        (var_vbspz, var_vbspz_dn0, var_vbspz_dn2, var_vbspz_dn6, var_vbspz_dn7, var_vbspz_dn10, var_vbspz_dn11, var_vbspz_dn12, var_vbspz_dn17,)
    }
};
        var_vbspz = assign5470_e3561;
        var_vbspz_dn0 = assign5470_e3561_d_n0;
        var_vbspz_dn2 = assign5470_e3561_d_n2;
        var_vbspz_dn6 = assign5470_e3561_d_n6;
        var_vbspz_dn7 = assign5470_e3561_d_n7;
        var_vbspz_dn10 = assign5470_e3561_d_n10;
        var_vbspz_dn11 = assign5470_e3561_d_n11;
        var_vbspz_dn12 = assign5470_e3561_d_n12;
        var_vbspz_dn17 = assign5470_e3561_d_n17;
        var_vbspz_rv = 0.0;

        let assign5480_e3564: f64 = (2.0 * var_q_nsub);
        let assign5480_e3566: f64 = (assign5480_e3564 * 1.034943e-10);
        let assign5480_e3568: f64 = (assign5480_e3566 * var_c_fox0_inv);
        let assign5480_e3570: f64 = (assign5480_e3568 * var_c_fox0_inv);
        var_t1__blk61 = assign5480_e3570;
        var_t1__blk61_dn0 = ((((2.0 * var_q_nsub_dn0) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk61_dn2 = ((((2.0 * var_q_nsub_dn2) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk61_dn6 = ((((2.0 * var_q_nsub_dn6) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk61_dn7 = ((((2.0 * var_q_nsub_dn7) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk61_dn10 = ((((2.0 * var_q_nsub_dn10) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk61_dn11 = ((((2.0 * var_q_nsub_dn11) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk61_dn12 = ((((2.0 * var_q_nsub_dn12) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk61_dn17 = ((((2.0 * var_q_nsub_dn17) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk61_rv = 0.0;

        let assign5490_e3573: f64 = (var_vgs - var_vfb);
        var_t2__blk62 = assign5490_e3573;
        var_t2__blk62_dn6 = var_vgs_dn6;
        var_t2__blk62_dn7 = var_vgs_dn7;
        var_t2__blk62_dn11 = var_vgs_dn11;
        var_t2__blk62_rv = 0.0;

        let assign5500_e3577: f64 = (2.0 / var_t1__blk61);
        let assign5500_e3580: f64 = (var_t2__blk62 - var_beta_inv);
        let assign5500_e3582: f64 = (assign5500_e3580 - var_vbsp);
        let assign5500_e3583: f64 = (assign5500_e3577 * assign5500_e3582);
        let assign5500_e3584: f64 = (1.0 + assign5500_e3583);
        var_t3__blk63 = assign5500_e3584;
        var_t3__blk63_dn0 = (((-((2.0 * var_t1__blk61_dn0) / (var_t1__blk61 * var_t1__blk61))) * assign5500_e3582) + (assign5500_e3577 * (-var_vbsp_dn0)));
        var_t3__blk63_dn2 = (((-((2.0 * var_t1__blk61_dn2) / (var_t1__blk61 * var_t1__blk61))) * assign5500_e3582) + (assign5500_e3577 * (-var_vbsp_dn2)));
        var_t3__blk63_dn6 = (((-((2.0 * var_t1__blk61_dn6) / (var_t1__blk61 * var_t1__blk61))) * assign5500_e3582) + (assign5500_e3577 * (var_t2__blk62_dn6 - var_vbsp_dn6)));
        var_t3__blk63_dn7 = (((-((2.0 * var_t1__blk61_dn7) / (var_t1__blk61 * var_t1__blk61))) * assign5500_e3582) + (assign5500_e3577 * (var_t2__blk62_dn7 - var_vbsp_dn7)));
        var_t3__blk63_dn10 = (((-((2.0 * var_t1__blk61_dn10) / (var_t1__blk61 * var_t1__blk61))) * assign5500_e3582) + (assign5500_e3577 * ((-var_beta_inv_dn10) - var_vbsp_dn10)));
        var_t3__blk63_dn11 = (((-((2.0 * var_t1__blk61_dn11) / (var_t1__blk61 * var_t1__blk61))) * assign5500_e3582) + (assign5500_e3577 * (var_t2__blk62_dn11 - var_vbsp_dn11)));
        var_t3__blk63_dn12 = (((-((2.0 * var_t1__blk61_dn12) / (var_t1__blk61 * var_t1__blk61))) * assign5500_e3582) + (assign5500_e3577 * (-var_vbsp_dn12)));
        var_t3__blk63_dn17 = (((-((2.0 * var_t1__blk61_dn17) / (var_t1__blk61 * var_t1__blk61))) * assign5500_e3582) + (assign5500_e3577 * (-var_vbsp_dn17)));
        var_t3__blk63_rv = 0.0;

        let assign5510_e3587: f64 = (var_t3__blk63 * var_t3__blk63);
        let assign5510_e3590: f64 = (4.0 * 0.001);
        let assign5510_e3592: f64 = (assign5510_e3590 * 0.001);
        let assign5510_e3593: f64 = (assign5510_e3587 + assign5510_e3592);
        let assign5510_e3594: f64 = (assign5510_e3593).sqrt();
        var_tmf1 = assign5510_e3594;
        var_tmf1_dn0 = (((var_t3__blk63_dn0 * var_t3__blk63) + (var_t3__blk63 * var_t3__blk63_dn0)) / (2.0 * assign5510_e3594));
        var_tmf1_dn2 = (((var_t3__blk63_dn2 * var_t3__blk63) + (var_t3__blk63 * var_t3__blk63_dn2)) / (2.0 * assign5510_e3594));
        var_tmf1_dn6 = (((var_t3__blk63_dn6 * var_t3__blk63) + (var_t3__blk63 * var_t3__blk63_dn6)) / (2.0 * assign5510_e3594));
        var_tmf1_dn7 = (((var_t3__blk63_dn7 * var_t3__blk63) + (var_t3__blk63 * var_t3__blk63_dn7)) / (2.0 * assign5510_e3594));
        var_tmf1_dn10 = (((var_t3__blk63_dn10 * var_t3__blk63) + (var_t3__blk63 * var_t3__blk63_dn10)) / (2.0 * assign5510_e3594));
        var_tmf1_dn11 = (((var_t3__blk63_dn11 * var_t3__blk63) + (var_t3__blk63 * var_t3__blk63_dn11)) / (2.0 * assign5510_e3594));
        var_tmf1_dn12 = (((var_t3__blk63_dn12 * var_t3__blk63) + (var_t3__blk63 * var_t3__blk63_dn12)) / (2.0 * assign5510_e3594));
        var_tmf1_dn17 = (((var_t3__blk63_dn17 * var_t3__blk63) + (var_t3__blk63 * var_t3__blk63_dn17)) / (2.0 * assign5510_e3594));
        var_tmf1_rv = 0.0;

        let assign5520_e3598: f64 = (var_t3__blk63 + var_tmf1);
        let assign5520_e3599: f64 = (0.5 * assign5520_e3598);
        let assign5520_e3602: f64 = (1e-10 * 0.001);
        let assign5520_e3603: f64 = (assign5520_e3599 + assign5520_e3602);
        var_t4 = assign5520_e3603;
        var_t4_dn0 = (0.5 * (var_t3__blk63_dn0 + var_tmf1_dn0));
        var_t4_dn2 = (0.5 * (var_t3__blk63_dn2 + var_tmf1_dn2));
        var_t4_dn6 = (0.5 * (var_t3__blk63_dn6 + var_tmf1_dn6));
        var_t4_dn7 = (0.5 * (var_t3__blk63_dn7 + var_tmf1_dn7));
        var_t4_dn10 = (0.5 * (var_t3__blk63_dn10 + var_tmf1_dn10));
        var_t4_dn11 = (0.5 * (var_t3__blk63_dn11 + var_tmf1_dn11));
        var_t4_dn12 = (0.5 * (var_t3__blk63_dn12 + var_tmf1_dn12));
        var_t4_dn17 = (0.5 * (var_t3__blk63_dn17 + var_tmf1_dn17));
        var_t4_rv = 0.0;

        let assign5530_e3606: f64 = if var_t4 < 0.0 { 1.0 } else { 0.0 };
        var_guard65 = assign5530_e3606;
        var_guard65_rv = 0.0;

        let (assign5540_e3610, assign5540_e3610_d_n0, assign5540_e3610_d_n2, assign5540_e3610_d_n6, assign5540_e3610_d_n7, assign5540_e3610_d_n10, assign5540_e3610_d_n11, assign5540_e3610_d_n12, assign5540_e3610_d_n17,) = {
    if (var_guard65 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign5540_e3610;
        var_t4_dn0 = assign5540_e3610_d_n0;
        var_t4_dn2 = assign5540_e3610_d_n2;
        var_t4_dn6 = assign5540_e3610_d_n6;
        var_t4_dn7 = assign5540_e3610_d_n7;
        var_t4_dn10 = assign5540_e3610_d_n10;
        var_t4_dn11 = assign5540_e3610_d_n11;
        var_t4_dn12 = assign5540_e3610_d_n12;
        var_t4_dn17 = assign5540_e3610_d_n17;
        var_t4_rv = 0.0;

        let assign5550_e3613: f64 = (var_t4 + 1e-50);
        let assign5550_e3614: f64 = (assign5550_e3613).sqrt();
        var_tx__blk64 = assign5550_e3614;
        var_tx__blk64_dn0 = (var_t4_dn0 / (2.0 * assign5550_e3614));
        var_tx__blk64_dn2 = (var_t4_dn2 / (2.0 * assign5550_e3614));
        var_tx__blk64_dn6 = (var_t4_dn6 / (2.0 * assign5550_e3614));
        var_tx__blk64_dn7 = (var_t4_dn7 / (2.0 * assign5550_e3614));
        var_tx__blk64_dn10 = (var_t4_dn10 / (2.0 * assign5550_e3614));
        var_tx__blk64_dn11 = (var_t4_dn11 / (2.0 * assign5550_e3614));
        var_tx__blk64_dn12 = (var_t4_dn12 / (2.0 * assign5550_e3614));
        var_tx__blk64_dn17 = (var_t4_dn17 / (2.0 * assign5550_e3614));
        var_tx__blk64_rv = 0.0;

        let assign5560_e3619: f64 = (1.0 - var_tx__blk64);
        let assign5560_e3620: f64 = (var_t1__blk61 * assign5560_e3619);
        let assign5560_e3621: f64 = (var_t2__blk62 + assign5560_e3620);
        var_pslsat = assign5560_e3621;
        var_pslsat_dn0 = ((var_t1__blk61_dn0 * assign5560_e3619) + (var_t1__blk61 * (-var_tx__blk64_dn0)));
        var_pslsat_dn2 = ((var_t1__blk61_dn2 * assign5560_e3619) + (var_t1__blk61 * (-var_tx__blk64_dn2)));
        var_pslsat_dn6 = (var_t2__blk62_dn6 + ((var_t1__blk61_dn6 * assign5560_e3619) + (var_t1__blk61 * (-var_tx__blk64_dn6))));
        var_pslsat_dn7 = (var_t2__blk62_dn7 + ((var_t1__blk61_dn7 * assign5560_e3619) + (var_t1__blk61 * (-var_tx__blk64_dn7))));
        var_pslsat_dn10 = ((var_t1__blk61_dn10 * assign5560_e3619) + (var_t1__blk61 * (-var_tx__blk64_dn10)));
        var_pslsat_dn11 = (var_t2__blk62_dn11 + ((var_t1__blk61_dn11 * assign5560_e3619) + (var_t1__blk61 * (-var_tx__blk64_dn11))));
        var_pslsat_dn12 = ((var_t1__blk61_dn12 * assign5560_e3619) + (var_t1__blk61 * (-var_tx__blk64_dn12)));
        var_pslsat_dn17 = ((var_t1__blk61_dn17 * assign5560_e3619) + (var_t1__blk61 * (-var_tx__blk64_dn17)));
        var_pslsat_rv = 0.0;

        let assign5570_e3624: f64 = (var_pslsat - var_pb2);
        var_vdsats = assign5570_e3624;
        var_vdsats_dn0 = (var_pslsat_dn0 - var_pb2_dn0);
        var_vdsats_dn2 = (var_pslsat_dn2 - var_pb2_dn2);
        var_vdsats_dn6 = (var_pslsat_dn6 - var_pb2_dn6);
        var_vdsats_dn7 = (var_pslsat_dn7 - var_pb2_dn7);
        var_vdsats_dn10 = (var_pslsat_dn10 - var_pb2_dn10);
        var_vdsats_dn11 = (var_pslsat_dn11 - var_pb2_dn11);
        var_vdsats_dn12 = (var_pslsat_dn12 - var_pb2_dn12);
        var_vdsats_dn17 = (var_pslsat_dn17 - var_pb2_dn17);
        var_vdsats_rv = 0.0;

        *var_ai_slot = var_ai;
        *var_ai_dn0_slot = var_ai_dn0;
        *var_ai_dn10_slot = var_ai_dn10;
        *var_ai_dn11_slot = var_ai_dn11;
        *var_ai_dn12_slot = var_ai_dn12;
        *var_ai_dn17_slot = var_ai_dn17;
        *var_ai_dn2_slot = var_ai_dn2;
        *var_ai_dn6_slot = var_ai_dn6;
        *var_ai_dn7_slot = var_ai_dn7;
        *var_ai_rv_slot = var_ai_rv;
        *var_c2_slot = var_c2;
        *var_c2_dn0_slot = var_c2_dn0;
        *var_c2_dn10_slot = var_c2_dn10;
        *var_c2_dn11_slot = var_c2_dn11;
        *var_c2_dn12_slot = var_c2_dn12;
        *var_c2_dn17_slot = var_c2_dn17;
        *var_c2_dn2_slot = var_c2_dn2;
        *var_c2_dn6_slot = var_c2_dn6;
        *var_c2_dn7_slot = var_c2_dn7;
        *var_c2_rv_slot = var_c2_rv;
        *var_db_slot = var_db;
        *var_db_dn0_slot = var_db_dn0;
        *var_db_dn10_slot = var_db_dn10;
        *var_db_dn11_slot = var_db_dn11;
        *var_db_dn12_slot = var_db_dn12;
        *var_db_dn17_slot = var_db_dn17;
        *var_db_dn2_slot = var_db_dn2;
        *var_db_dn6_slot = var_db_dn6;
        *var_db_dn7_slot = var_db_dn7;
        *var_db_rv_slot = var_db_rv;
        *var_di_slot = var_di;
        *var_di_dn0_slot = var_di_dn0;
        *var_di_dn10_slot = var_di_dn10;
        *var_di_dn11_slot = var_di_dn11;
        *var_di_dn12_slot = var_di_dn12;
        *var_di_dn17_slot = var_di_dn17;
        *var_di_dn2_slot = var_di_dn2;
        *var_di_dn6_slot = var_di_dn6;
        *var_di_dn7_slot = var_di_dn7;
        *var_di_rv_slot = var_di_rv;
        *var_flg_pprv_slot = var_flg_pprv;
        *var_flg_pprv_rv_slot = var_flg_pprv_rv;
        *var_guard59_slot = var_guard59;
        *var_guard59_rv_slot = var_guard59_rv;
        *var_guard60_slot = var_guard60;
        *var_guard60_rv_slot = var_guard60_rv;
        *var_guard65_slot = var_guard65;
        *var_guard65_rv_slot = var_guard65_rv;
        *var_lp_s0_slot = var_lp_s0;
        *var_lp_s0_rv_slot = var_lp_s0_rv;
        *var_lp_sl_slot = var_lp_sl;
        *var_lp_sl_rv_slot = var_lp_sl_rv;
        *var_pbs0_ini_slot = var_pbs0_ini;
        *var_pbs0_ini_rv_slot = var_pbs0_ini_rv;
        *var_pbsl_ini_slot = var_pbsl_ini;
        *var_pbsl_ini_rv_slot = var_pbsl_ini_rv;
        *var_psb0_ini_slot = var_psb0_ini;
        *var_psb0_ini_rv_slot = var_psb0_ini_rv;
        *var_psbl_ini_slot = var_psbl_ini;
        *var_psbl_ini_rv_slot = var_psbl_ini_rv;
        *var_pslsat_slot = var_pslsat;
        *var_pslsat_dn0_slot = var_pslsat_dn0;
        *var_pslsat_dn10_slot = var_pslsat_dn10;
        *var_pslsat_dn11_slot = var_pslsat_dn11;
        *var_pslsat_dn12_slot = var_pslsat_dn12;
        *var_pslsat_dn17_slot = var_pslsat_dn17;
        *var_pslsat_dn2_slot = var_pslsat_dn2;
        *var_pslsat_dn6_slot = var_pslsat_dn6;
        *var_pslsat_dn7_slot = var_pslsat_dn7;
        *var_pslsat_rv_slot = var_pslsat_rv;
        *var_pss0_ini_slot = var_pss0_ini;
        *var_pss0_ini_rv_slot = var_pss0_ini_rv;
        *var_pssl_ini_slot = var_pssl_ini;
        *var_pssl_ini_rv_slot = var_pssl_ini_rv;
        *var_t1__blk58_slot = var_t1__blk58;
        *var_t1__blk58_dn0_slot = var_t1__blk58_dn0;
        *var_t1__blk58_dn10_slot = var_t1__blk58_dn10;
        *var_t1__blk58_dn11_slot = var_t1__blk58_dn11;
        *var_t1__blk58_dn12_slot = var_t1__blk58_dn12;
        *var_t1__blk58_dn17_slot = var_t1__blk58_dn17;
        *var_t1__blk58_dn2_slot = var_t1__blk58_dn2;
        *var_t1__blk58_dn6_slot = var_t1__blk58_dn6;
        *var_t1__blk58_dn7_slot = var_t1__blk58_dn7;
        *var_t1__blk58_rv_slot = var_t1__blk58_rv;
        *var_t1__blk61_slot = var_t1__blk61;
        *var_t1__blk61_dn0_slot = var_t1__blk61_dn0;
        *var_t1__blk61_dn10_slot = var_t1__blk61_dn10;
        *var_t1__blk61_dn11_slot = var_t1__blk61_dn11;
        *var_t1__blk61_dn12_slot = var_t1__blk61_dn12;
        *var_t1__blk61_dn17_slot = var_t1__blk61_dn17;
        *var_t1__blk61_dn2_slot = var_t1__blk61_dn2;
        *var_t1__blk61_dn6_slot = var_t1__blk61_dn6;
        *var_t1__blk61_dn7_slot = var_t1__blk61_dn7;
        *var_t1__blk61_rv_slot = var_t1__blk61_rv;
        *var_t2__blk62_slot = var_t2__blk62;
        *var_t2__blk62_dn11_slot = var_t2__blk62_dn11;
        *var_t2__blk62_dn6_slot = var_t2__blk62_dn6;
        *var_t2__blk62_dn7_slot = var_t2__blk62_dn7;
        *var_t2__blk62_rv_slot = var_t2__blk62_rv;
        *var_t3__blk63_slot = var_t3__blk63;
        *var_t3__blk63_dn0_slot = var_t3__blk63_dn0;
        *var_t3__blk63_dn10_slot = var_t3__blk63_dn10;
        *var_t3__blk63_dn11_slot = var_t3__blk63_dn11;
        *var_t3__blk63_dn12_slot = var_t3__blk63_dn12;
        *var_t3__blk63_dn17_slot = var_t3__blk63_dn17;
        *var_t3__blk63_dn2_slot = var_t3__blk63_dn2;
        *var_t3__blk63_dn6_slot = var_t3__blk63_dn6;
        *var_t3__blk63_dn7_slot = var_t3__blk63_dn7;
        *var_t3__blk63_rv_slot = var_t3__blk63_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn17_slot = var_t4_dn17;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_rv_slot = var_t4_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_tx__blk64_slot = var_tx__blk64;
        *var_tx__blk64_dn0_slot = var_tx__blk64_dn0;
        *var_tx__blk64_dn10_slot = var_tx__blk64_dn10;
        *var_tx__blk64_dn11_slot = var_tx__blk64_dn11;
        *var_tx__blk64_dn12_slot = var_tx__blk64_dn12;
        *var_tx__blk64_dn17_slot = var_tx__blk64_dn17;
        *var_tx__blk64_dn2_slot = var_tx__blk64_dn2;
        *var_tx__blk64_dn6_slot = var_tx__blk64_dn6;
        *var_tx__blk64_dn7_slot = var_tx__blk64_dn7;
        *var_tx__blk64_rv_slot = var_tx__blk64_rv;
        *var_vbs_slot = var_vbs;
        *var_vbs_dn0_slot = var_vbs_dn0;
        *var_vbs_dn10_slot = var_vbs_dn10;
        *var_vbs_dn11_slot = var_vbs_dn11;
        *var_vbs_dn12_slot = var_vbs_dn12;
        *var_vbs_dn17_slot = var_vbs_dn17;
        *var_vbs_dn2_slot = var_vbs_dn2;
        *var_vbs_dn6_slot = var_vbs_dn6;
        *var_vbs_dn7_slot = var_vbs_dn7;
        *var_vbs_rv_slot = var_vbs_rv;
        *var_vbsc_slot = var_vbsc;
        *var_vbsc_dn0_slot = var_vbsc_dn0;
        *var_vbsc_dn10_slot = var_vbsc_dn10;
        *var_vbsc_dn11_slot = var_vbsc_dn11;
        *var_vbsc_dn12_slot = var_vbsc_dn12;
        *var_vbsc_dn17_slot = var_vbsc_dn17;
        *var_vbsc_dn2_slot = var_vbsc_dn2;
        *var_vbsc_dn6_slot = var_vbsc_dn6;
        *var_vbsc_dn7_slot = var_vbsc_dn7;
        *var_vbsc_rv_slot = var_vbsc_rv;
        *var_vbsp_slot = var_vbsp;
        *var_vbsp_dn0_slot = var_vbsp_dn0;
        *var_vbsp_dn10_slot = var_vbsp_dn10;
        *var_vbsp_dn11_slot = var_vbsp_dn11;
        *var_vbsp_dn12_slot = var_vbsp_dn12;
        *var_vbsp_dn17_slot = var_vbsp_dn17;
        *var_vbsp_dn2_slot = var_vbsp_dn2;
        *var_vbsp_dn6_slot = var_vbsp_dn6;
        *var_vbsp_dn7_slot = var_vbsp_dn7;
        *var_vbsp_rv_slot = var_vbsp_rv;
        *var_vbspz_slot = var_vbspz;
        *var_vbspz_dn0_slot = var_vbspz_dn0;
        *var_vbspz_dn10_slot = var_vbspz_dn10;
        *var_vbspz_dn11_slot = var_vbspz_dn11;
        *var_vbspz_dn12_slot = var_vbspz_dn12;
        *var_vbspz_dn17_slot = var_vbspz_dn17;
        *var_vbspz_dn2_slot = var_vbspz_dn2;
        *var_vbspz_dn6_slot = var_vbspz_dn6;
        *var_vbspz_dn7_slot = var_vbspz_dn7;
        *var_vbspz_rv_slot = var_vbspz_rv;
        *var_vbsz_slot = var_vbsz;
        *var_vbsz_dn0_slot = var_vbsz_dn0;
        *var_vbsz_dn10_slot = var_vbsz_dn10;
        *var_vbsz_dn11_slot = var_vbsz_dn11;
        *var_vbsz_dn12_slot = var_vbsz_dn12;
        *var_vbsz_dn17_slot = var_vbsz_dn17;
        *var_vbsz_dn2_slot = var_vbsz_dn2;
        *var_vbsz_dn6_slot = var_vbsz_dn6;
        *var_vbsz_dn7_slot = var_vbsz_dn7;
        *var_vbsz_rv_slot = var_vbsz_rv;
        *var_vds_slot = var_vds;
        *var_vds_dn0_slot = var_vds_dn0;
        *var_vds_dn10_slot = var_vds_dn10;
        *var_vds_dn11_slot = var_vds_dn11;
        *var_vds_dn12_slot = var_vds_dn12;
        *var_vds_dn17_slot = var_vds_dn17;
        *var_vds_dn2_slot = var_vds_dn2;
        *var_vds_dn6_slot = var_vds_dn6;
        *var_vds_dn7_slot = var_vds_dn7;
        *var_vds_rv_slot = var_vds_rv;
        *var_vdsats_slot = var_vdsats;
        *var_vdsats_dn0_slot = var_vdsats_dn0;
        *var_vdsats_dn10_slot = var_vdsats_dn10;
        *var_vdsats_dn11_slot = var_vdsats_dn11;
        *var_vdsats_dn12_slot = var_vdsats_dn12;
        *var_vdsats_dn17_slot = var_vdsats_dn17;
        *var_vdsats_dn2_slot = var_vdsats_dn2;
        *var_vdsats_dn6_slot = var_vdsats_dn6;
        *var_vdsats_dn7_slot = var_vdsats_dn7;
        *var_vdsats_rv_slot = var_vdsats_rv;
        *var_vdsc_slot = var_vdsc;
        *var_vdsc_dn0_slot = var_vdsc_dn0;
        *var_vdsc_dn10_slot = var_vdsc_dn10;
        *var_vdsc_dn11_slot = var_vdsc_dn11;
        *var_vdsc_dn12_slot = var_vdsc_dn12;
        *var_vdsc_dn17_slot = var_vdsc_dn17;
        *var_vdsc_dn2_slot = var_vdsc_dn2;
        *var_vdsc_dn6_slot = var_vdsc_dn6;
        *var_vdsc_dn7_slot = var_vdsc_dn7;
        *var_vdsc_rv_slot = var_vdsc_rv;
        *var_vdsz_slot = var_vdsz;
        *var_vdsz_dn0_slot = var_vdsz_dn0;
        *var_vdsz_dn10_slot = var_vdsz_dn10;
        *var_vdsz_dn11_slot = var_vdsz_dn11;
        *var_vdsz_dn12_slot = var_vdsz_dn12;
        *var_vdsz_dn17_slot = var_vdsz_dn17;
        *var_vdsz_dn2_slot = var_vdsz_dn2;
        *var_vdsz_dn6_slot = var_vdsz_dn6;
        *var_vdsz_dn7_slot = var_vdsz_dn7;
        *var_vdsz_rv_slot = var_vdsz_rv;
        *var_vgs_slot = var_vgs;
        *var_vgs_dn11_slot = var_vgs_dn11;
        *var_vgs_dn6_slot = var_vgs_dn6;
        *var_vgs_dn7_slot = var_vgs_dn7;
        *var_vgs_rv_slot = var_vgs_rv;
        *var_vgsc_slot = var_vgsc;
        *var_vgsc_dn11_slot = var_vgsc_dn11;
        *var_vgsc_dn6_slot = var_vgsc_dn6;
        *var_vgsc_dn7_slot = var_vgsc_dn7;
        *var_vgsc_rv_slot = var_vgsc_rv;
        *var_vgsz_slot = var_vgsz;
        *var_vgsz_dn0_slot = var_vgsz_dn0;
        *var_vgsz_dn10_slot = var_vgsz_dn10;
        *var_vgsz_dn11_slot = var_vgsz_dn11;
        *var_vgsz_dn12_slot = var_vgsz_dn12;
        *var_vgsz_dn17_slot = var_vgsz_dn17;
        *var_vgsz_dn2_slot = var_vgsz_dn2;
        *var_vgsz_dn6_slot = var_vgsz_dn6;
        *var_vgsz_dn7_slot = var_vgsz_dn7;
        *var_vgsz_rv_slot = var_vgsz_rv;
        *var_vzadd_slot = var_vzadd;
        *var_vzadd_dn0_slot = var_vzadd_dn0;
        *var_vzadd_dn10_slot = var_vzadd_dn10;
        *var_vzadd_dn11_slot = var_vzadd_dn11;
        *var_vzadd_dn12_slot = var_vzadd_dn12;
        *var_vzadd_dn17_slot = var_vzadd_dn17;
        *var_vzadd_dn2_slot = var_vzadd_dn2;
        *var_vzadd_dn6_slot = var_vzadd_dn6;
        *var_vzadd_dn7_slot = var_vzadd_dn7;
        *var_vzadd_rv_slot = var_vzadd_rv;
    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        var_c_fox0: f64,
        var_c_fox0_inv: f64,
        var_cnst0soi: f64,
        var_cnst0soi_dn0: f64,
        var_cnst0soi_dn10: f64,
        var_cnst0soi_dn11: f64,
        var_cnst0soi_dn12: f64,
        var_cnst0soi_dn17: f64,
        var_cnst0soi_dn2: f64,
        var_cnst0soi_dn6: f64,
        var_cnst0soi_dn7: f64,
        var_pb20: f64,
        var_pb20_dn0: f64,
        var_pb20_dn10: f64,
        var_pb20_dn11: f64,
        var_pb20_dn12: f64,
        var_pb20_dn17: f64,
        var_pb20_dn2: f64,
        var_pb20_dn6: f64,
        var_pb20_dn7: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn11: f64,
        var_q_nsub_dn12: f64,
        var_q_nsub_dn17: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn7: f64,
        var_tfox0: f64,
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
        var_vfb: f64,
        var_vgs: f64,
        var_vgs_dn11: f64,
        var_vgs_dn6: f64,
        var_vgs_dn7: f64,
        var_c_fox_slot: &mut f64,
        var_c_fox_dn0_slot: &mut f64,
        var_c_fox_dn10_slot: &mut f64,
        var_c_fox_dn11_slot: &mut f64,
        var_c_fox_dn12_slot: &mut f64,
        var_c_fox_dn17_slot: &mut f64,
        var_c_fox_dn2_slot: &mut f64,
        var_c_fox_dn6_slot: &mut f64,
        var_c_fox_dn7_slot: &mut f64,
        var_c_fox_inv_slot: &mut f64,
        var_c_fox_inv_dn0_slot: &mut f64,
        var_c_fox_inv_dn10_slot: &mut f64,
        var_c_fox_inv_dn11_slot: &mut f64,
        var_c_fox_inv_dn12_slot: &mut f64,
        var_c_fox_inv_dn17_slot: &mut f64,
        var_c_fox_inv_dn2_slot: &mut f64,
        var_c_fox_inv_dn6_slot: &mut f64,
        var_c_fox_inv_dn7_slot: &mut f64,
        var_c_fox_inv_rv_slot: &mut f64,
        var_c_fox_rv_slot: &mut f64,
        var_cnstc_foxi_slot: &mut f64,
        var_cnstc_foxi_dn0_slot: &mut f64,
        var_cnstc_foxi_dn10_slot: &mut f64,
        var_cnstc_foxi_dn11_slot: &mut f64,
        var_cnstc_foxi_dn12_slot: &mut f64,
        var_cnstc_foxi_dn17_slot: &mut f64,
        var_cnstc_foxi_dn2_slot: &mut f64,
        var_cnstc_foxi_dn6_slot: &mut f64,
        var_cnstc_foxi_dn7_slot: &mut f64,
        var_cnstc_foxi_rv_slot: &mut f64,
        var_flg_qme_slot: &mut f64,
        var_flg_qme_rv_slot: &mut f64,
        var_fmdvds_slot: &mut f64,
        var_fmdvds_dn0_slot: &mut f64,
        var_fmdvds_dn10_slot: &mut f64,
        var_fmdvds_dn11_slot: &mut f64,
        var_fmdvds_dn12_slot: &mut f64,
        var_fmdvds_dn17_slot: &mut f64,
        var_fmdvds_dn2_slot: &mut f64,
        var_fmdvds_dn6_slot: &mut f64,
        var_fmdvds_dn7_slot: &mut f64,
        var_fmdvds_rv_slot: &mut f64,
        var_guard72_slot: &mut f64,
        var_guard72_rv_slot: &mut f64,
        var_guard73_slot: &mut f64,
        var_guard73_rv_slot: &mut f64,
        var_guard74_slot: &mut f64,
        var_guard74_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1__blk61_slot: &mut f64,
        var_t1__blk61_dn0_slot: &mut f64,
        var_t1__blk61_dn10_slot: &mut f64,
        var_t1__blk61_dn11_slot: &mut f64,
        var_t1__blk61_dn12_slot: &mut f64,
        var_t1__blk61_dn17_slot: &mut f64,
        var_t1__blk61_dn2_slot: &mut f64,
        var_t1__blk61_dn6_slot: &mut f64,
        var_t1__blk61_dn7_slot: &mut f64,
        var_t1__blk61_rv_slot: &mut f64,
        var_t2__blk66_slot: &mut f64,
        var_t2__blk66_dn0_slot: &mut f64,
        var_t2__blk66_dn10_slot: &mut f64,
        var_t2__blk66_dn11_slot: &mut f64,
        var_t2__blk66_dn12_slot: &mut f64,
        var_t2__blk66_dn17_slot: &mut f64,
        var_t2__blk66_dn2_slot: &mut f64,
        var_t2__blk66_dn6_slot: &mut f64,
        var_t2__blk66_dn7_slot: &mut f64,
        var_t2__blk66_rv_slot: &mut f64,
        var_t3__blk67_slot: &mut f64,
        var_t3__blk67_dn0_slot: &mut f64,
        var_t3__blk67_dn10_slot: &mut f64,
        var_t3__blk67_dn11_slot: &mut f64,
        var_t3__blk67_dn12_slot: &mut f64,
        var_t3__blk67_dn17_slot: &mut f64,
        var_t3__blk67_dn2_slot: &mut f64,
        var_t3__blk67_dn6_slot: &mut f64,
        var_t3__blk67_dn7_slot: &mut f64,
        var_t3__blk67_rv_slot: &mut f64,
        var_t4__blk68_slot: &mut f64,
        var_t4__blk68_dn0_slot: &mut f64,
        var_t4__blk68_dn10_slot: &mut f64,
        var_t4__blk68_dn11_slot: &mut f64,
        var_t4__blk68_dn12_slot: &mut f64,
        var_t4__blk68_dn17_slot: &mut f64,
        var_t4__blk68_dn2_slot: &mut f64,
        var_t4__blk68_dn6_slot: &mut f64,
        var_t4__blk68_dn7_slot: &mut f64,
        var_t4__blk68_rv_slot: &mut f64,
        var_t4w_slot: &mut f64,
        var_t4w_dn0_slot: &mut f64,
        var_t4w_dn10_slot: &mut f64,
        var_t4w_dn11_slot: &mut f64,
        var_t4w_dn12_slot: &mut f64,
        var_t4w_dn17_slot: &mut f64,
        var_t4w_dn2_slot: &mut f64,
        var_t4w_dn6_slot: &mut f64,
        var_t4w_dn7_slot: &mut f64,
        var_t4w_rv_slot: &mut f64,
        var_t5__blk70_slot: &mut f64,
        var_t5__blk70_dn0_slot: &mut f64,
        var_t5__blk70_dn10_slot: &mut f64,
        var_t5__blk70_dn11_slot: &mut f64,
        var_t5__blk70_dn12_slot: &mut f64,
        var_t5__blk70_dn17_slot: &mut f64,
        var_t5__blk70_dn2_slot: &mut f64,
        var_t5__blk70_dn6_slot: &mut f64,
        var_t5__blk70_dn7_slot: &mut f64,
        var_t5__blk70_rv_slot: &mut f64,
        var_t6__blk71_slot: &mut f64,
        var_t6__blk71_dn0_slot: &mut f64,
        var_t6__blk71_dn10_slot: &mut f64,
        var_t6__blk71_dn11_slot: &mut f64,
        var_t6__blk71_dn12_slot: &mut f64,
        var_t6__blk71_dn17_slot: &mut f64,
        var_t6__blk71_dn2_slot: &mut f64,
        var_t6__blk71_dn6_slot: &mut f64,
        var_t6__blk71_dn7_slot: &mut f64,
        var_t6__blk71_rv_slot: &mut f64,
        var_tfoxe_slot: &mut f64,
        var_tfoxe_dn0_slot: &mut f64,
        var_tfoxe_dn10_slot: &mut f64,
        var_tfoxe_dn11_slot: &mut f64,
        var_tfoxe_dn12_slot: &mut f64,
        var_tfoxe_dn17_slot: &mut f64,
        var_tfoxe_dn2_slot: &mut f64,
        var_tfoxe_dn6_slot: &mut f64,
        var_tfoxe_dn7_slot: &mut f64,
        var_tfoxe_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_tmf3_slot: &mut f64,
        var_tmf3_dn0_slot: &mut f64,
        var_tmf3_dn10_slot: &mut f64,
        var_tmf3_dn11_slot: &mut f64,
        var_tmf3_dn12_slot: &mut f64,
        var_tmf3_dn17_slot: &mut f64,
        var_tmf3_dn2_slot: &mut f64,
        var_tmf3_dn6_slot: &mut f64,
        var_tmf3_dn7_slot: &mut f64,
        var_tmf3_rv_slot: &mut f64,
        var_tmf4_slot: &mut f64,
        var_tmf4_dn0_slot: &mut f64,
        var_tmf4_dn10_slot: &mut f64,
        var_tmf4_dn11_slot: &mut f64,
        var_tmf4_dn12_slot: &mut f64,
        var_tmf4_dn17_slot: &mut f64,
        var_tmf4_dn2_slot: &mut f64,
        var_tmf4_dn6_slot: &mut f64,
        var_tmf4_dn7_slot: &mut f64,
        var_tmf4_rv_slot: &mut f64,
        var_tx__blk64_slot: &mut f64,
        var_tx__blk64_dn0_slot: &mut f64,
        var_tx__blk64_dn10_slot: &mut f64,
        var_tx__blk64_dn11_slot: &mut f64,
        var_tx__blk64_dn12_slot: &mut f64,
        var_tx__blk64_dn17_slot: &mut f64,
        var_tx__blk64_dn2_slot: &mut f64,
        var_tx__blk64_dn6_slot: &mut f64,
        var_tx__blk64_dn7_slot: &mut f64,
        var_tx__blk64_rv_slot: &mut f64,
        var_vdsats_slot: &mut f64,
        var_vdsats_dn0_slot: &mut f64,
        var_vdsats_dn10_slot: &mut f64,
        var_vdsats_dn11_slot: &mut f64,
        var_vdsats_dn12_slot: &mut f64,
        var_vdsats_dn17_slot: &mut f64,
        var_vdsats_dn2_slot: &mut f64,
        var_vdsats_dn6_slot: &mut f64,
        var_vdsats_dn7_slot: &mut f64,
        var_vdsats_rv_slot: &mut f64,
        var_vthq_slot: &mut f64,
        var_vthq_dn0_slot: &mut f64,
        var_vthq_dn10_slot: &mut f64,
        var_vthq_dn11_slot: &mut f64,
        var_vthq_dn12_slot: &mut f64,
        var_vthq_dn17_slot: &mut f64,
        var_vthq_dn2_slot: &mut f64,
        var_vthq_dn6_slot: &mut f64,
        var_vthq_dn7_slot: &mut f64,
        var_vthq_rv_slot: &mut f64,
    ) {
        let mut var_c_fox: f64 = *var_c_fox_slot;
        let mut var_c_fox_dn0: f64 = *var_c_fox_dn0_slot;
        let mut var_c_fox_dn10: f64 = *var_c_fox_dn10_slot;
        let mut var_c_fox_dn11: f64 = *var_c_fox_dn11_slot;
        let mut var_c_fox_dn12: f64 = *var_c_fox_dn12_slot;
        let mut var_c_fox_dn17: f64 = *var_c_fox_dn17_slot;
        let mut var_c_fox_dn2: f64 = *var_c_fox_dn2_slot;
        let mut var_c_fox_dn6: f64 = *var_c_fox_dn6_slot;
        let mut var_c_fox_dn7: f64 = *var_c_fox_dn7_slot;
        let mut var_c_fox_inv: f64 = *var_c_fox_inv_slot;
        let mut var_c_fox_inv_dn0: f64 = *var_c_fox_inv_dn0_slot;
        let mut var_c_fox_inv_dn10: f64 = *var_c_fox_inv_dn10_slot;
        let mut var_c_fox_inv_dn11: f64 = *var_c_fox_inv_dn11_slot;
        let mut var_c_fox_inv_dn12: f64 = *var_c_fox_inv_dn12_slot;
        let mut var_c_fox_inv_dn17: f64 = *var_c_fox_inv_dn17_slot;
        let mut var_c_fox_inv_dn2: f64 = *var_c_fox_inv_dn2_slot;
        let mut var_c_fox_inv_dn6: f64 = *var_c_fox_inv_dn6_slot;
        let mut var_c_fox_inv_dn7: f64 = *var_c_fox_inv_dn7_slot;
        let mut var_c_fox_inv_rv: f64 = *var_c_fox_inv_rv_slot;
        let mut var_c_fox_rv: f64 = *var_c_fox_rv_slot;
        let mut var_cnstc_foxi: f64 = *var_cnstc_foxi_slot;
        let mut var_cnstc_foxi_dn0: f64 = *var_cnstc_foxi_dn0_slot;
        let mut var_cnstc_foxi_dn10: f64 = *var_cnstc_foxi_dn10_slot;
        let mut var_cnstc_foxi_dn11: f64 = *var_cnstc_foxi_dn11_slot;
        let mut var_cnstc_foxi_dn12: f64 = *var_cnstc_foxi_dn12_slot;
        let mut var_cnstc_foxi_dn17: f64 = *var_cnstc_foxi_dn17_slot;
        let mut var_cnstc_foxi_dn2: f64 = *var_cnstc_foxi_dn2_slot;
        let mut var_cnstc_foxi_dn6: f64 = *var_cnstc_foxi_dn6_slot;
        let mut var_cnstc_foxi_dn7: f64 = *var_cnstc_foxi_dn7_slot;
        let mut var_cnstc_foxi_rv: f64 = *var_cnstc_foxi_rv_slot;
        let mut var_flg_qme: f64 = *var_flg_qme_slot;
        let mut var_flg_qme_rv: f64 = *var_flg_qme_rv_slot;
        let mut var_fmdvds: f64 = *var_fmdvds_slot;
        let mut var_fmdvds_dn0: f64 = *var_fmdvds_dn0_slot;
        let mut var_fmdvds_dn10: f64 = *var_fmdvds_dn10_slot;
        let mut var_fmdvds_dn11: f64 = *var_fmdvds_dn11_slot;
        let mut var_fmdvds_dn12: f64 = *var_fmdvds_dn12_slot;
        let mut var_fmdvds_dn17: f64 = *var_fmdvds_dn17_slot;
        let mut var_fmdvds_dn2: f64 = *var_fmdvds_dn2_slot;
        let mut var_fmdvds_dn6: f64 = *var_fmdvds_dn6_slot;
        let mut var_fmdvds_dn7: f64 = *var_fmdvds_dn7_slot;
        let mut var_fmdvds_rv: f64 = *var_fmdvds_rv_slot;
        let mut var_guard72: f64 = *var_guard72_slot;
        let mut var_guard72_rv: f64 = *var_guard72_rv_slot;
        let mut var_guard73: f64 = *var_guard73_slot;
        let mut var_guard73_rv: f64 = *var_guard73_rv_slot;
        let mut var_guard74: f64 = *var_guard74_slot;
        let mut var_guard74_rv: f64 = *var_guard74_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1__blk61: f64 = *var_t1__blk61_slot;
        let mut var_t1__blk61_dn0: f64 = *var_t1__blk61_dn0_slot;
        let mut var_t1__blk61_dn10: f64 = *var_t1__blk61_dn10_slot;
        let mut var_t1__blk61_dn11: f64 = *var_t1__blk61_dn11_slot;
        let mut var_t1__blk61_dn12: f64 = *var_t1__blk61_dn12_slot;
        let mut var_t1__blk61_dn17: f64 = *var_t1__blk61_dn17_slot;
        let mut var_t1__blk61_dn2: f64 = *var_t1__blk61_dn2_slot;
        let mut var_t1__blk61_dn6: f64 = *var_t1__blk61_dn6_slot;
        let mut var_t1__blk61_dn7: f64 = *var_t1__blk61_dn7_slot;
        let mut var_t1__blk61_rv: f64 = *var_t1__blk61_rv_slot;
        let mut var_t2__blk66: f64 = *var_t2__blk66_slot;
        let mut var_t2__blk66_dn0: f64 = *var_t2__blk66_dn0_slot;
        let mut var_t2__blk66_dn10: f64 = *var_t2__blk66_dn10_slot;
        let mut var_t2__blk66_dn11: f64 = *var_t2__blk66_dn11_slot;
        let mut var_t2__blk66_dn12: f64 = *var_t2__blk66_dn12_slot;
        let mut var_t2__blk66_dn17: f64 = *var_t2__blk66_dn17_slot;
        let mut var_t2__blk66_dn2: f64 = *var_t2__blk66_dn2_slot;
        let mut var_t2__blk66_dn6: f64 = *var_t2__blk66_dn6_slot;
        let mut var_t2__blk66_dn7: f64 = *var_t2__blk66_dn7_slot;
        let mut var_t2__blk66_rv: f64 = *var_t2__blk66_rv_slot;
        let mut var_t3__blk67: f64 = *var_t3__blk67_slot;
        let mut var_t3__blk67_dn0: f64 = *var_t3__blk67_dn0_slot;
        let mut var_t3__blk67_dn10: f64 = *var_t3__blk67_dn10_slot;
        let mut var_t3__blk67_dn11: f64 = *var_t3__blk67_dn11_slot;
        let mut var_t3__blk67_dn12: f64 = *var_t3__blk67_dn12_slot;
        let mut var_t3__blk67_dn17: f64 = *var_t3__blk67_dn17_slot;
        let mut var_t3__blk67_dn2: f64 = *var_t3__blk67_dn2_slot;
        let mut var_t3__blk67_dn6: f64 = *var_t3__blk67_dn6_slot;
        let mut var_t3__blk67_dn7: f64 = *var_t3__blk67_dn7_slot;
        let mut var_t3__blk67_rv: f64 = *var_t3__blk67_rv_slot;
        let mut var_t4__blk68: f64 = *var_t4__blk68_slot;
        let mut var_t4__blk68_dn0: f64 = *var_t4__blk68_dn0_slot;
        let mut var_t4__blk68_dn10: f64 = *var_t4__blk68_dn10_slot;
        let mut var_t4__blk68_dn11: f64 = *var_t4__blk68_dn11_slot;
        let mut var_t4__blk68_dn12: f64 = *var_t4__blk68_dn12_slot;
        let mut var_t4__blk68_dn17: f64 = *var_t4__blk68_dn17_slot;
        let mut var_t4__blk68_dn2: f64 = *var_t4__blk68_dn2_slot;
        let mut var_t4__blk68_dn6: f64 = *var_t4__blk68_dn6_slot;
        let mut var_t4__blk68_dn7: f64 = *var_t4__blk68_dn7_slot;
        let mut var_t4__blk68_rv: f64 = *var_t4__blk68_rv_slot;
        let mut var_t4w: f64 = *var_t4w_slot;
        let mut var_t4w_dn0: f64 = *var_t4w_dn0_slot;
        let mut var_t4w_dn10: f64 = *var_t4w_dn10_slot;
        let mut var_t4w_dn11: f64 = *var_t4w_dn11_slot;
        let mut var_t4w_dn12: f64 = *var_t4w_dn12_slot;
        let mut var_t4w_dn17: f64 = *var_t4w_dn17_slot;
        let mut var_t4w_dn2: f64 = *var_t4w_dn2_slot;
        let mut var_t4w_dn6: f64 = *var_t4w_dn6_slot;
        let mut var_t4w_dn7: f64 = *var_t4w_dn7_slot;
        let mut var_t4w_rv: f64 = *var_t4w_rv_slot;
        let mut var_t5__blk70: f64 = *var_t5__blk70_slot;
        let mut var_t5__blk70_dn0: f64 = *var_t5__blk70_dn0_slot;
        let mut var_t5__blk70_dn10: f64 = *var_t5__blk70_dn10_slot;
        let mut var_t5__blk70_dn11: f64 = *var_t5__blk70_dn11_slot;
        let mut var_t5__blk70_dn12: f64 = *var_t5__blk70_dn12_slot;
        let mut var_t5__blk70_dn17: f64 = *var_t5__blk70_dn17_slot;
        let mut var_t5__blk70_dn2: f64 = *var_t5__blk70_dn2_slot;
        let mut var_t5__blk70_dn6: f64 = *var_t5__blk70_dn6_slot;
        let mut var_t5__blk70_dn7: f64 = *var_t5__blk70_dn7_slot;
        let mut var_t5__blk70_rv: f64 = *var_t5__blk70_rv_slot;
        let mut var_t6__blk71: f64 = *var_t6__blk71_slot;
        let mut var_t6__blk71_dn0: f64 = *var_t6__blk71_dn0_slot;
        let mut var_t6__blk71_dn10: f64 = *var_t6__blk71_dn10_slot;
        let mut var_t6__blk71_dn11: f64 = *var_t6__blk71_dn11_slot;
        let mut var_t6__blk71_dn12: f64 = *var_t6__blk71_dn12_slot;
        let mut var_t6__blk71_dn17: f64 = *var_t6__blk71_dn17_slot;
        let mut var_t6__blk71_dn2: f64 = *var_t6__blk71_dn2_slot;
        let mut var_t6__blk71_dn6: f64 = *var_t6__blk71_dn6_slot;
        let mut var_t6__blk71_dn7: f64 = *var_t6__blk71_dn7_slot;
        let mut var_t6__blk71_rv: f64 = *var_t6__blk71_rv_slot;
        let mut var_tfoxe: f64 = *var_tfoxe_slot;
        let mut var_tfoxe_dn0: f64 = *var_tfoxe_dn0_slot;
        let mut var_tfoxe_dn10: f64 = *var_tfoxe_dn10_slot;
        let mut var_tfoxe_dn11: f64 = *var_tfoxe_dn11_slot;
        let mut var_tfoxe_dn12: f64 = *var_tfoxe_dn12_slot;
        let mut var_tfoxe_dn17: f64 = *var_tfoxe_dn17_slot;
        let mut var_tfoxe_dn2: f64 = *var_tfoxe_dn2_slot;
        let mut var_tfoxe_dn6: f64 = *var_tfoxe_dn6_slot;
        let mut var_tfoxe_dn7: f64 = *var_tfoxe_dn7_slot;
        let mut var_tfoxe_rv: f64 = *var_tfoxe_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_tmf3: f64 = *var_tmf3_slot;
        let mut var_tmf3_dn0: f64 = *var_tmf3_dn0_slot;
        let mut var_tmf3_dn10: f64 = *var_tmf3_dn10_slot;
        let mut var_tmf3_dn11: f64 = *var_tmf3_dn11_slot;
        let mut var_tmf3_dn12: f64 = *var_tmf3_dn12_slot;
        let mut var_tmf3_dn17: f64 = *var_tmf3_dn17_slot;
        let mut var_tmf3_dn2: f64 = *var_tmf3_dn2_slot;
        let mut var_tmf3_dn6: f64 = *var_tmf3_dn6_slot;
        let mut var_tmf3_dn7: f64 = *var_tmf3_dn7_slot;
        let mut var_tmf3_rv: f64 = *var_tmf3_rv_slot;
        let mut var_tmf4: f64 = *var_tmf4_slot;
        let mut var_tmf4_dn0: f64 = *var_tmf4_dn0_slot;
        let mut var_tmf4_dn10: f64 = *var_tmf4_dn10_slot;
        let mut var_tmf4_dn11: f64 = *var_tmf4_dn11_slot;
        let mut var_tmf4_dn12: f64 = *var_tmf4_dn12_slot;
        let mut var_tmf4_dn17: f64 = *var_tmf4_dn17_slot;
        let mut var_tmf4_dn2: f64 = *var_tmf4_dn2_slot;
        let mut var_tmf4_dn6: f64 = *var_tmf4_dn6_slot;
        let mut var_tmf4_dn7: f64 = *var_tmf4_dn7_slot;
        let mut var_tmf4_rv: f64 = *var_tmf4_rv_slot;
        let mut var_tx__blk64: f64 = *var_tx__blk64_slot;
        let mut var_tx__blk64_dn0: f64 = *var_tx__blk64_dn0_slot;
        let mut var_tx__blk64_dn10: f64 = *var_tx__blk64_dn10_slot;
        let mut var_tx__blk64_dn11: f64 = *var_tx__blk64_dn11_slot;
        let mut var_tx__blk64_dn12: f64 = *var_tx__blk64_dn12_slot;
        let mut var_tx__blk64_dn17: f64 = *var_tx__blk64_dn17_slot;
        let mut var_tx__blk64_dn2: f64 = *var_tx__blk64_dn2_slot;
        let mut var_tx__blk64_dn6: f64 = *var_tx__blk64_dn6_slot;
        let mut var_tx__blk64_dn7: f64 = *var_tx__blk64_dn7_slot;
        let mut var_tx__blk64_rv: f64 = *var_tx__blk64_rv_slot;
        let mut var_vdsats: f64 = *var_vdsats_slot;
        let mut var_vdsats_dn0: f64 = *var_vdsats_dn0_slot;
        let mut var_vdsats_dn10: f64 = *var_vdsats_dn10_slot;
        let mut var_vdsats_dn11: f64 = *var_vdsats_dn11_slot;
        let mut var_vdsats_dn12: f64 = *var_vdsats_dn12_slot;
        let mut var_vdsats_dn17: f64 = *var_vdsats_dn17_slot;
        let mut var_vdsats_dn2: f64 = *var_vdsats_dn2_slot;
        let mut var_vdsats_dn6: f64 = *var_vdsats_dn6_slot;
        let mut var_vdsats_dn7: f64 = *var_vdsats_dn7_slot;
        let mut var_vdsats_rv: f64 = *var_vdsats_rv_slot;
        let mut var_vthq: f64 = *var_vthq_slot;
        let mut var_vthq_dn0: f64 = *var_vthq_dn0_slot;
        let mut var_vthq_dn10: f64 = *var_vthq_dn10_slot;
        let mut var_vthq_dn11: f64 = *var_vthq_dn11_slot;
        let mut var_vthq_dn12: f64 = *var_vthq_dn12_slot;
        let mut var_vthq_dn17: f64 = *var_vthq_dn17_slot;
        let mut var_vthq_dn2: f64 = *var_vthq_dn2_slot;
        let mut var_vthq_dn6: f64 = *var_vthq_dn6_slot;
        let mut var_vthq_dn7: f64 = *var_vthq_dn7_slot;
        let mut var_vthq_rv: f64 = *var_vthq_rv_slot;

        let assign5580_e3627: f64 = (var_vdsats - 0.1);
        let assign5580_e3629: f64 = (assign5580_e3627 - 0.05);
        var_tmf1 = assign5580_e3629;
        var_tmf1_dn0 = var_vdsats_dn0;
        var_tmf1_dn2 = var_vdsats_dn2;
        var_tmf1_dn6 = var_vdsats_dn6;
        var_tmf1_dn7 = var_vdsats_dn7;
        var_tmf1_dn10 = var_vdsats_dn10;
        var_tmf1_dn11 = var_vdsats_dn11;
        var_tmf1_dn12 = var_vdsats_dn12;
        var_tmf1_dn17 = var_vdsats_dn17;
        var_tmf1_rv = 0.0;

        let assign5590_e3632: f64 = (4.0 * 0.1);
        let assign5590_e3634: f64 = (assign5590_e3632 * 0.05);
        var_tmf2 = assign5590_e3634;
        var_tmf2_dn0 = 0.0;
        var_tmf2_dn2 = 0.0;
        var_tmf2_dn6 = 0.0;
        var_tmf2_dn7 = 0.0;
        var_tmf2_dn10 = 0.0;
        var_tmf2_dn11 = 0.0;
        var_tmf2_dn12 = 0.0;
        var_tmf2_dn17 = 0.0;
        var_tmf2_rv = 0.0;

        let (assign5600_e3641, assign5600_e3641_d_n0, assign5600_e3641_d_n2, assign5600_e3641_d_n6, assign5600_e3641_d_n7, assign5600_e3641_d_n10, assign5600_e3641_d_n11, assign5600_e3641_d_n12, assign5600_e3641_d_n17,) = {
    if (var_tmf2 > 0.0) {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    } else {
        let assign5600_e3640: f64 = (-var_tmf2);
        (assign5600_e3640, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
    }
};
        var_tmf2 = assign5600_e3641;
        var_tmf2_dn0 = assign5600_e3641_d_n0;
        var_tmf2_dn2 = assign5600_e3641_d_n2;
        var_tmf2_dn6 = assign5600_e3641_d_n6;
        var_tmf2_dn7 = assign5600_e3641_d_n7;
        var_tmf2_dn10 = assign5600_e3641_d_n10;
        var_tmf2_dn11 = assign5600_e3641_d_n11;
        var_tmf2_dn12 = assign5600_e3641_d_n12;
        var_tmf2_dn17 = assign5600_e3641_d_n17;
        var_tmf2_rv = 0.0;

        let assign5610_e3644: f64 = (var_tmf1 * var_tmf1);
        let assign5610_e3646: f64 = (assign5610_e3644 + var_tmf2);
        let assign5610_e3647: f64 = (assign5610_e3646).sqrt();
        var_tmf2 = assign5610_e3647;
        var_tmf2_dn0 = ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5610_e3647));
        var_tmf2_dn2 = ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5610_e3647));
        var_tmf2_dn6 = ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign5610_e3647));
        var_tmf2_dn7 = ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign5610_e3647));
        var_tmf2_dn10 = ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign5610_e3647));
        var_tmf2_dn11 = ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign5610_e3647));
        var_tmf2_dn12 = ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign5610_e3647));
        var_tmf2_dn17 = ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign5610_e3647));
        var_tmf2_rv = 0.0;

        let assign5620_e3652: f64 = (var_tmf1 + var_tmf2);
        let assign5620_e3653: f64 = (0.5 * assign5620_e3652);
        let assign5620_e3654: f64 = (0.1 + assign5620_e3653);
        var_vdsats = assign5620_e3654;
        var_vdsats_dn0 = (0.5 * (var_tmf1_dn0 + var_tmf2_dn0));
        var_vdsats_dn2 = (0.5 * (var_tmf1_dn2 + var_tmf2_dn2));
        var_vdsats_dn6 = (0.5 * (var_tmf1_dn6 + var_tmf2_dn6));
        var_vdsats_dn7 = (0.5 * (var_tmf1_dn7 + var_tmf2_dn7));
        var_vdsats_dn10 = (0.5 * (var_tmf1_dn10 + var_tmf2_dn10));
        var_vdsats_dn11 = (0.5 * (var_tmf1_dn11 + var_tmf2_dn11));
        var_vdsats_dn12 = (0.5 * (var_tmf1_dn12 + var_tmf2_dn12));
        var_vdsats_dn17 = (0.5 * (var_tmf1_dn17 + var_tmf2_dn17));
        var_vdsats_rv = 0.0;

        let assign5630_e3657: f64 = (var_vds / var_vdsats);
        var_t1__blk61 = assign5630_e3657;
        var_t1__blk61_dn0 = (((var_vds_dn0 * var_vdsats) - (var_vds * var_vdsats_dn0)) / (var_vdsats * var_vdsats));
        var_t1__blk61_dn2 = (((var_vds_dn2 * var_vdsats) - (var_vds * var_vdsats_dn2)) / (var_vdsats * var_vdsats));
        var_t1__blk61_dn6 = (((var_vds_dn6 * var_vdsats) - (var_vds * var_vdsats_dn6)) / (var_vdsats * var_vdsats));
        var_t1__blk61_dn7 = (((var_vds_dn7 * var_vdsats) - (var_vds * var_vdsats_dn7)) / (var_vdsats * var_vdsats));
        var_t1__blk61_dn10 = (((var_vds_dn10 * var_vdsats) - (var_vds * var_vdsats_dn10)) / (var_vdsats * var_vdsats));
        var_t1__blk61_dn11 = (((var_vds_dn11 * var_vdsats) - (var_vds * var_vdsats_dn11)) / (var_vdsats * var_vdsats));
        var_t1__blk61_dn12 = (((var_vds_dn12 * var_vdsats) - (var_vds * var_vdsats_dn12)) / (var_vdsats * var_vdsats));
        var_t1__blk61_dn17 = (((var_vds_dn17 * var_vdsats) - (var_vds * var_vdsats_dn17)) / (var_vdsats * var_vdsats));
        var_t1__blk61_rv = 0.0;

        let assign5640_e3660: f64 = var_t1__blk61;
        var_tmf1 = assign5640_e3660;
        var_tmf1_dn0 = var_t1__blk61_dn0;
        var_tmf1_dn2 = var_t1__blk61_dn2;
        var_tmf1_dn6 = var_t1__blk61_dn6;
        var_tmf1_dn7 = var_t1__blk61_dn7;
        var_tmf1_dn10 = var_t1__blk61_dn10;
        var_tmf1_dn11 = var_t1__blk61_dn11;
        var_tmf1_dn12 = var_t1__blk61_dn12;
        var_tmf1_dn17 = var_t1__blk61_dn17;
        var_tmf1_rv = 0.0;

        let assign5650_e3663: f64 = (var_tmf1 * var_tmf1);
        var_tmf2 = assign5650_e3663;
        var_tmf2_dn0 = ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0));
        var_tmf2_dn2 = ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2));
        var_tmf2_dn6 = ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6));
        var_tmf2_dn7 = ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7));
        var_tmf2_dn10 = ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10));
        var_tmf2_dn11 = ((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11));
        var_tmf2_dn12 = ((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12));
        var_tmf2_dn17 = ((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17));
        var_tmf2_rv = 0.0;

        let assign5660_e3666: f64 = (var_tmf2 * var_tmf1);
        var_tmf3 = assign5660_e3666;
        var_tmf3_dn0 = ((var_tmf2_dn0 * var_tmf1) + (var_tmf2 * var_tmf1_dn0));
        var_tmf3_dn2 = ((var_tmf2_dn2 * var_tmf1) + (var_tmf2 * var_tmf1_dn2));
        var_tmf3_dn6 = ((var_tmf2_dn6 * var_tmf1) + (var_tmf2 * var_tmf1_dn6));
        var_tmf3_dn7 = ((var_tmf2_dn7 * var_tmf1) + (var_tmf2 * var_tmf1_dn7));
        var_tmf3_dn10 = ((var_tmf2_dn10 * var_tmf1) + (var_tmf2 * var_tmf1_dn10));
        var_tmf3_dn11 = ((var_tmf2_dn11 * var_tmf1) + (var_tmf2 * var_tmf1_dn11));
        var_tmf3_dn12 = ((var_tmf2_dn12 * var_tmf1) + (var_tmf2 * var_tmf1_dn12));
        var_tmf3_dn17 = ((var_tmf2_dn17 * var_tmf1) + (var_tmf2 * var_tmf1_dn17));
        var_tmf3_rv = 0.0;

        let assign5670_e3669: f64 = (var_tmf2 * var_tmf2);
        var_tmf4 = assign5670_e3669;
        var_tmf4_dn0 = ((var_tmf2_dn0 * var_tmf2) + (var_tmf2 * var_tmf2_dn0));
        var_tmf4_dn2 = ((var_tmf2_dn2 * var_tmf2) + (var_tmf2 * var_tmf2_dn2));
        var_tmf4_dn6 = ((var_tmf2_dn6 * var_tmf2) + (var_tmf2 * var_tmf2_dn6));
        var_tmf4_dn7 = ((var_tmf2_dn7 * var_tmf2) + (var_tmf2 * var_tmf2_dn7));
        var_tmf4_dn10 = ((var_tmf2_dn10 * var_tmf2) + (var_tmf2 * var_tmf2_dn10));
        var_tmf4_dn11 = ((var_tmf2_dn11 * var_tmf2) + (var_tmf2 * var_tmf2_dn11));
        var_tmf4_dn12 = ((var_tmf2_dn12 * var_tmf2) + (var_tmf2 * var_tmf2_dn12));
        var_tmf4_dn17 = ((var_tmf2_dn17 * var_tmf2) + (var_tmf2 * var_tmf2_dn17));
        var_tmf4_rv = 0.0;

        let assign5680_e3673: f64 = (1.0 + var_tmf1);
        let assign5680_e3675: f64 = (assign5680_e3673 + var_tmf2);
        let assign5680_e3677: f64 = (assign5680_e3675 + var_tmf3);
        let assign5680_e3679: f64 = (assign5680_e3677 + var_tmf4);
        let assign5680_e3680: f64 = (1.0 / assign5680_e3679);
        var_tx__blk64 = assign5680_e3680;
        var_tx__blk64_dn0 = (-((((var_tmf1_dn0 + var_tmf2_dn0) + var_tmf3_dn0) + var_tmf4_dn0) / (assign5680_e3679 * assign5680_e3679)));
        var_tx__blk64_dn2 = (-((((var_tmf1_dn2 + var_tmf2_dn2) + var_tmf3_dn2) + var_tmf4_dn2) / (assign5680_e3679 * assign5680_e3679)));
        var_tx__blk64_dn6 = (-((((var_tmf1_dn6 + var_tmf2_dn6) + var_tmf3_dn6) + var_tmf4_dn6) / (assign5680_e3679 * assign5680_e3679)));
        var_tx__blk64_dn7 = (-((((var_tmf1_dn7 + var_tmf2_dn7) + var_tmf3_dn7) + var_tmf4_dn7) / (assign5680_e3679 * assign5680_e3679)));
        var_tx__blk64_dn10 = (-((((var_tmf1_dn10 + var_tmf2_dn10) + var_tmf3_dn10) + var_tmf4_dn10) / (assign5680_e3679 * assign5680_e3679)));
        var_tx__blk64_dn11 = (-((((var_tmf1_dn11 + var_tmf2_dn11) + var_tmf3_dn11) + var_tmf4_dn11) / (assign5680_e3679 * assign5680_e3679)));
        var_tx__blk64_dn12 = (-((((var_tmf1_dn12 + var_tmf2_dn12) + var_tmf3_dn12) + var_tmf4_dn12) / (assign5680_e3679 * assign5680_e3679)));
        var_tx__blk64_dn17 = (-((((var_tmf1_dn17 + var_tmf2_dn17) + var_tmf3_dn17) + var_tmf4_dn17) / (assign5680_e3679 * assign5680_e3679)));
        var_tx__blk64_rv = 0.0;

        let assign5690_e3684: f64 = (2.0 * var_tmf1);
        let assign5690_e3685: f64 = (1.0 + assign5690_e3684);
        let assign5690_e3688: f64 = (3.0 * var_tmf2);
        let assign5690_e3689: f64 = (assign5690_e3685 + assign5690_e3688);
        let assign5690_e3692: f64 = (4.0 * var_tmf3);
        let assign5690_e3693: f64 = (assign5690_e3689 + assign5690_e3692);
        let assign5690_e3694: f64 = (-assign5690_e3693);
        let assign5690_e3696: f64 = (assign5690_e3694 * var_tx__blk64);
        let assign5690_e3698: f64 = (assign5690_e3696 * var_tx__blk64);
        var_t0 = assign5690_e3698;
        var_t0_dn0 = (((((-(((2.0 * var_tmf1_dn0) + (3.0 * var_tmf2_dn0)) + (4.0 * var_tmf3_dn0))) * var_tx__blk64) + (assign5690_e3694 * var_tx__blk64_dn0)) * var_tx__blk64) + (assign5690_e3696 * var_tx__blk64_dn0));
        var_t0_dn2 = (((((-(((2.0 * var_tmf1_dn2) + (3.0 * var_tmf2_dn2)) + (4.0 * var_tmf3_dn2))) * var_tx__blk64) + (assign5690_e3694 * var_tx__blk64_dn2)) * var_tx__blk64) + (assign5690_e3696 * var_tx__blk64_dn2));
        var_t0_dn6 = (((((-(((2.0 * var_tmf1_dn6) + (3.0 * var_tmf2_dn6)) + (4.0 * var_tmf3_dn6))) * var_tx__blk64) + (assign5690_e3694 * var_tx__blk64_dn6)) * var_tx__blk64) + (assign5690_e3696 * var_tx__blk64_dn6));
        var_t0_dn7 = (((((-(((2.0 * var_tmf1_dn7) + (3.0 * var_tmf2_dn7)) + (4.0 * var_tmf3_dn7))) * var_tx__blk64) + (assign5690_e3694 * var_tx__blk64_dn7)) * var_tx__blk64) + (assign5690_e3696 * var_tx__blk64_dn7));
        var_t0_dn10 = (((((-(((2.0 * var_tmf1_dn10) + (3.0 * var_tmf2_dn10)) + (4.0 * var_tmf3_dn10))) * var_tx__blk64) + (assign5690_e3694 * var_tx__blk64_dn10)) * var_tx__blk64) + (assign5690_e3696 * var_tx__blk64_dn10));
        var_t0_dn11 = (((((-(((2.0 * var_tmf1_dn11) + (3.0 * var_tmf2_dn11)) + (4.0 * var_tmf3_dn11))) * var_tx__blk64) + (assign5690_e3694 * var_tx__blk64_dn11)) * var_tx__blk64) + (assign5690_e3696 * var_tx__blk64_dn11));
        var_t0_dn12 = (((((-(((2.0 * var_tmf1_dn12) + (3.0 * var_tmf2_dn12)) + (4.0 * var_tmf3_dn12))) * var_tx__blk64) + (assign5690_e3694 * var_tx__blk64_dn12)) * var_tx__blk64) + (assign5690_e3696 * var_tx__blk64_dn12));
        var_t0_dn17 = (((((-(((2.0 * var_tmf1_dn17) + (3.0 * var_tmf2_dn17)) + (4.0 * var_tmf3_dn17))) * var_tx__blk64) + (assign5690_e3694 * var_tx__blk64_dn17)) * var_tx__blk64) + (assign5690_e3696 * var_tx__blk64_dn17));
        var_t0_rv = 0.0;

        let assign5700_e3702: f64 = (1.0 - var_tx__blk64);
        let assign5700_e3703: f64 = assign5700_e3702;
        var_tx__blk64 = assign5700_e3703;
        var_tx__blk64_dn0 = (-var_tx__blk64_dn0);
        var_tx__blk64_dn2 = (-var_tx__blk64_dn2);
        var_tx__blk64_dn6 = (-var_tx__blk64_dn6);
        var_tx__blk64_dn7 = (-var_tx__blk64_dn7);
        var_tx__blk64_dn10 = (-var_tx__blk64_dn10);
        var_tx__blk64_dn11 = (-var_tx__blk64_dn11);
        var_tx__blk64_dn12 = (-var_tx__blk64_dn12);
        var_tx__blk64_dn17 = (-var_tx__blk64_dn17);
        var_tx__blk64_rv = 0.0;

        let assign5710_e3705: f64 = (-var_t0);
        var_t0 = assign5710_e3705;
        var_t0_dn0 = (-var_t0_dn0);
        var_t0_dn2 = (-var_t0_dn2);
        var_t0_dn6 = (-var_t0_dn6);
        var_t0_dn7 = (-var_t0_dn7);
        var_t0_dn10 = (-var_t0_dn10);
        var_t0_dn11 = (-var_t0_dn11);
        var_t0_dn12 = (-var_t0_dn12);
        var_t0_dn17 = (-var_t0_dn17);
        var_t0_rv = 0.0;

        let assign5720_e3708: f64 = (var_tx__blk64 * var_tx__blk64);
        var_fmdvds = assign5720_e3708;
        var_fmdvds_dn0 = ((var_tx__blk64_dn0 * var_tx__blk64) + (var_tx__blk64 * var_tx__blk64_dn0));
        var_fmdvds_dn2 = ((var_tx__blk64_dn2 * var_tx__blk64) + (var_tx__blk64 * var_tx__blk64_dn2));
        var_fmdvds_dn6 = ((var_tx__blk64_dn6 * var_tx__blk64) + (var_tx__blk64 * var_tx__blk64_dn6));
        var_fmdvds_dn7 = ((var_tx__blk64_dn7 * var_tx__blk64) + (var_tx__blk64 * var_tx__blk64_dn7));
        var_fmdvds_dn10 = ((var_tx__blk64_dn10 * var_tx__blk64) + (var_tx__blk64 * var_tx__blk64_dn10));
        var_fmdvds_dn11 = ((var_tx__blk64_dn11 * var_tx__blk64) + (var_tx__blk64 * var_tx__blk64_dn11));
        var_fmdvds_dn12 = ((var_tx__blk64_dn12 * var_tx__blk64) + (var_tx__blk64 * var_tx__blk64_dn12));
        var_fmdvds_dn17 = ((var_tx__blk64_dn17 * var_tx__blk64) + (var_tx__blk64 * var_tx__blk64_dn17));
        var_fmdvds_rv = 0.0;

        let assign5730_e3719: f64 = if (((p.p204 == 0.0) && (p.p206 == 0.0)) || (p.p205 == 0.0)) { 1.0 } else { 0.0 };
        var_guard72 = assign5730_e3719;
        var_guard72_rv = 0.0;

        let (assign5740_e3723,) = {
    if (var_guard72 != 0.0) {
        (0.0,)
    } else {
        (var_flg_qme,)
    }
};
        var_flg_qme = assign5740_e3723;
        var_flg_qme_rv = 0.0;

        let (assign5750_e3728,) = {
    if (var_guard72 == 0.0) {
        (1.0,)
    } else {
        (var_flg_qme,)
    }
};
        var_flg_qme = assign5750_e3728;
        var_flg_qme_rv = 0.0;

        let assign5760_e3731: f64 = (2.0 * var_q_nsub);
        let assign5760_e3733: f64 = (assign5760_e3731 * 1.034943e-10);
        let assign5760_e3735: f64 = (assign5760_e3733 * var_pb20);
        let assign5760_e3736: f64 = (assign5760_e3735).sqrt();
        var_t2__blk66 = assign5760_e3736;
        var_t2__blk66_dn0 = (((((2.0 * var_q_nsub_dn0) * 1.034943e-10) * var_pb20) + (assign5760_e3733 * var_pb20_dn0)) / (2.0 * assign5760_e3736));
        var_t2__blk66_dn2 = (((((2.0 * var_q_nsub_dn2) * 1.034943e-10) * var_pb20) + (assign5760_e3733 * var_pb20_dn2)) / (2.0 * assign5760_e3736));
        var_t2__blk66_dn6 = (((((2.0 * var_q_nsub_dn6) * 1.034943e-10) * var_pb20) + (assign5760_e3733 * var_pb20_dn6)) / (2.0 * assign5760_e3736));
        var_t2__blk66_dn7 = (((((2.0 * var_q_nsub_dn7) * 1.034943e-10) * var_pb20) + (assign5760_e3733 * var_pb20_dn7)) / (2.0 * assign5760_e3736));
        var_t2__blk66_dn10 = (((((2.0 * var_q_nsub_dn10) * 1.034943e-10) * var_pb20) + (assign5760_e3733 * var_pb20_dn10)) / (2.0 * assign5760_e3736));
        var_t2__blk66_dn11 = (((((2.0 * var_q_nsub_dn11) * 1.034943e-10) * var_pb20) + (assign5760_e3733 * var_pb20_dn11)) / (2.0 * assign5760_e3736));
        var_t2__blk66_dn12 = (((((2.0 * var_q_nsub_dn12) * 1.034943e-10) * var_pb20) + (assign5760_e3733 * var_pb20_dn12)) / (2.0 * assign5760_e3736));
        var_t2__blk66_dn17 = (((((2.0 * var_q_nsub_dn17) * 1.034943e-10) * var_pb20) + (assign5760_e3733 * var_pb20_dn17)) / (2.0 * assign5760_e3736));
        var_t2__blk66_rv = 0.0;

        let assign5770_e3739: f64 = (var_pb20 + var_vfb);
        let assign5770_e3742: f64 = (var_t2__blk66 / var_c_fox0);
        let assign5770_e3743: f64 = (assign5770_e3739 + assign5770_e3742);
        var_vthq = assign5770_e3743;
        var_vthq_dn0 = (var_pb20_dn0 + (var_t2__blk66_dn0 / var_c_fox0));
        var_vthq_dn2 = (var_pb20_dn2 + (var_t2__blk66_dn2 / var_c_fox0));
        var_vthq_dn6 = (var_pb20_dn6 + (var_t2__blk66_dn6 / var_c_fox0));
        var_vthq_dn7 = (var_pb20_dn7 + (var_t2__blk66_dn7 / var_c_fox0));
        var_vthq_dn10 = (var_pb20_dn10 + (var_t2__blk66_dn10 / var_c_fox0));
        var_vthq_dn11 = (var_pb20_dn11 + (var_t2__blk66_dn11 / var_c_fox0));
        var_vthq_dn12 = (var_pb20_dn12 + (var_t2__blk66_dn12 / var_c_fox0));
        var_vthq_dn17 = (var_pb20_dn17 + (var_t2__blk66_dn17 / var_c_fox0));
        var_vthq_rv = 0.0;

        let assign5780_e3746: f64 = if var_flg_qme == 0.0 { 1.0 } else { 0.0 };
        var_guard73 = assign5780_e3746;
        var_guard73_rv = 0.0;

        let (assign5790_e3750, assign5790_e3750_d_n0, assign5790_e3750_d_n2, assign5790_e3750_d_n6, assign5790_e3750_d_n7, assign5790_e3750_d_n10, assign5790_e3750_d_n11, assign5790_e3750_d_n12, assign5790_e3750_d_n17,) = {
    if (var_guard73 != 0.0) {
        (var_tfox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tfoxe, var_tfoxe_dn0, var_tfoxe_dn2, var_tfoxe_dn6, var_tfoxe_dn7, var_tfoxe_dn10, var_tfoxe_dn11, var_tfoxe_dn12, var_tfoxe_dn17,)
    }
};
        var_tfoxe = assign5790_e3750;
        var_tfoxe_dn0 = assign5790_e3750_d_n0;
        var_tfoxe_dn2 = assign5790_e3750_d_n2;
        var_tfoxe_dn6 = assign5790_e3750_d_n6;
        var_tfoxe_dn7 = assign5790_e3750_d_n7;
        var_tfoxe_dn10 = assign5790_e3750_d_n10;
        var_tfoxe_dn11 = assign5790_e3750_d_n11;
        var_tfoxe_dn12 = assign5790_e3750_d_n12;
        var_tfoxe_dn17 = assign5790_e3750_d_n17;
        var_tfoxe_rv = 0.0;

        let (assign5800_e3754, assign5800_e3754_d_n0, assign5800_e3754_d_n2, assign5800_e3754_d_n6, assign5800_e3754_d_n7, assign5800_e3754_d_n10, assign5800_e3754_d_n11, assign5800_e3754_d_n12, assign5800_e3754_d_n17,) = {
    if (var_guard73 != 0.0) {
        (var_c_fox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_c_fox, var_c_fox_dn0, var_c_fox_dn2, var_c_fox_dn6, var_c_fox_dn7, var_c_fox_dn10, var_c_fox_dn11, var_c_fox_dn12, var_c_fox_dn17,)
    }
};
        var_c_fox = assign5800_e3754;
        var_c_fox_dn0 = assign5800_e3754_d_n0;
        var_c_fox_dn2 = assign5800_e3754_d_n2;
        var_c_fox_dn6 = assign5800_e3754_d_n6;
        var_c_fox_dn7 = assign5800_e3754_d_n7;
        var_c_fox_dn10 = assign5800_e3754_d_n10;
        var_c_fox_dn11 = assign5800_e3754_d_n11;
        var_c_fox_dn12 = assign5800_e3754_d_n12;
        var_c_fox_dn17 = assign5800_e3754_d_n17;
        var_c_fox_rv = 0.0;

        let (assign5810_e3758, assign5810_e3758_d_n0, assign5810_e3758_d_n2, assign5810_e3758_d_n6, assign5810_e3758_d_n7, assign5810_e3758_d_n10, assign5810_e3758_d_n11, assign5810_e3758_d_n12, assign5810_e3758_d_n17,) = {
    if (var_guard73 != 0.0) {
        (var_c_fox0_inv, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_c_fox_inv, var_c_fox_inv_dn0, var_c_fox_inv_dn2, var_c_fox_inv_dn6, var_c_fox_inv_dn7, var_c_fox_inv_dn10, var_c_fox_inv_dn11, var_c_fox_inv_dn12, var_c_fox_inv_dn17,)
    }
};
        var_c_fox_inv = assign5810_e3758;
        var_c_fox_inv_dn0 = assign5810_e3758_d_n0;
        var_c_fox_inv_dn2 = assign5810_e3758_d_n2;
        var_c_fox_inv_dn6 = assign5810_e3758_d_n6;
        var_c_fox_inv_dn7 = assign5810_e3758_d_n7;
        var_c_fox_inv_dn10 = assign5810_e3758_d_n10;
        var_c_fox_inv_dn11 = assign5810_e3758_d_n11;
        var_c_fox_inv_dn12 = assign5810_e3758_d_n12;
        var_c_fox_inv_dn17 = assign5810_e3758_d_n17;
        var_c_fox_inv_rv = 0.0;

        let (assign5820_e3768, assign5820_e3768_d_n0, assign5820_e3768_d_n2, assign5820_e3768_d_n6, assign5820_e3768_d_n7, assign5820_e3768_d_n10, assign5820_e3768_d_n11, assign5820_e3768_d_n12, assign5820_e3768_d_n17,) = {
    if (var_guard73 != 0.0) {
        let assign5820_e3762: f64 = (var_cnst0soi * var_c_fox0_inv);
        let assign5820_e3764: f64 = (assign5820_e3762 * var_c_fox0_inv);
        let assign5820_e3766: f64 = (assign5820_e3764 * var_cnst0soi);
        (assign5820_e3766, ((((var_cnst0soi_dn0 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5820_e3764 * var_cnst0soi_dn0)), ((((var_cnst0soi_dn2 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5820_e3764 * var_cnst0soi_dn2)), ((((var_cnst0soi_dn6 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5820_e3764 * var_cnst0soi_dn6)), ((((var_cnst0soi_dn7 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5820_e3764 * var_cnst0soi_dn7)), ((((var_cnst0soi_dn10 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5820_e3764 * var_cnst0soi_dn10)), ((((var_cnst0soi_dn11 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5820_e3764 * var_cnst0soi_dn11)), ((((var_cnst0soi_dn12 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5820_e3764 * var_cnst0soi_dn12)), ((((var_cnst0soi_dn17 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5820_e3764 * var_cnst0soi_dn17)),)
    } else {
        (var_cnstc_foxi, var_cnstc_foxi_dn0, var_cnstc_foxi_dn2, var_cnstc_foxi_dn6, var_cnstc_foxi_dn7, var_cnstc_foxi_dn10, var_cnstc_foxi_dn11, var_cnstc_foxi_dn12, var_cnstc_foxi_dn17,)
    }
};
        var_cnstc_foxi = assign5820_e3768;
        var_cnstc_foxi_dn0 = assign5820_e3768_d_n0;
        var_cnstc_foxi_dn2 = assign5820_e3768_d_n2;
        var_cnstc_foxi_dn6 = assign5820_e3768_d_n6;
        var_cnstc_foxi_dn7 = assign5820_e3768_d_n7;
        var_cnstc_foxi_dn10 = assign5820_e3768_d_n10;
        var_cnstc_foxi_dn11 = assign5820_e3768_d_n11;
        var_cnstc_foxi_dn12 = assign5820_e3768_d_n12;
        var_cnstc_foxi_dn17 = assign5820_e3768_d_n17;
        var_cnstc_foxi_rv = 0.0;

        let (assign5830_e3779, assign5830_e3779_d_n0, assign5830_e3779_d_n2, assign5830_e3779_d_n6, assign5830_e3779_d_n7, assign5830_e3779_d_n10, assign5830_e3779_d_n11, assign5830_e3779_d_n12, assign5830_e3779_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign5830_e3773: f64 = (var_vgs - var_vbsp);
        let assign5830_e3775: f64 = (assign5830_e3773 - var_vthq);
        let assign5830_e3777: f64 = (assign5830_e3775 + p.p205);
        (assign5830_e3777, ((-var_vbsp_dn0) - var_vthq_dn0), ((-var_vbsp_dn2) - var_vthq_dn2), ((var_vgs_dn6 - var_vbsp_dn6) - var_vthq_dn6), ((var_vgs_dn7 - var_vbsp_dn7) - var_vthq_dn7), ((-var_vbsp_dn10) - var_vthq_dn10), ((var_vgs_dn11 - var_vbsp_dn11) - var_vthq_dn11), ((-var_vbsp_dn12) - var_vthq_dn12), ((-var_vbsp_dn17) - var_vthq_dn17),)
    } else {
        (var_t5__blk70, var_t5__blk70_dn0, var_t5__blk70_dn2, var_t5__blk70_dn6, var_t5__blk70_dn7, var_t5__blk70_dn10, var_t5__blk70_dn11, var_t5__blk70_dn12, var_t5__blk70_dn17,)
    }
};
        var_t5__blk70 = assign5830_e3779;
        var_t5__blk70_dn0 = assign5830_e3779_d_n0;
        var_t5__blk70_dn2 = assign5830_e3779_d_n2;
        var_t5__blk70_dn6 = assign5830_e3779_d_n6;
        var_t5__blk70_dn7 = assign5830_e3779_d_n7;
        var_t5__blk70_dn10 = assign5830_e3779_d_n10;
        var_t5__blk70_dn11 = assign5830_e3779_d_n11;
        var_t5__blk70_dn12 = assign5830_e3779_d_n12;
        var_t5__blk70_dn17 = assign5830_e3779_d_n17;
        var_t5__blk70_rv = 0.0;

        let (assign5840_e3793, assign5840_e3793_d_n0, assign5840_e3793_d_n2, assign5840_e3793_d_n6, assign5840_e3793_d_n7, assign5840_e3793_d_n10, assign5840_e3793_d_n11, assign5840_e3793_d_n12, assign5840_e3793_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign5840_e3784: f64 = (var_t5__blk70 * var_t5__blk70);
        let assign5840_e3787: f64 = (4.0 * 0.0001);
        let assign5840_e3789: f64 = (assign5840_e3787 * 0.0001);
        let assign5840_e3790: f64 = (assign5840_e3784 + assign5840_e3789);
        let assign5840_e3791: f64 = (assign5840_e3790).sqrt();
        (assign5840_e3791, (((var_t5__blk70_dn0 * var_t5__blk70) + (var_t5__blk70 * var_t5__blk70_dn0)) / (2.0 * assign5840_e3791)), (((var_t5__blk70_dn2 * var_t5__blk70) + (var_t5__blk70 * var_t5__blk70_dn2)) / (2.0 * assign5840_e3791)), (((var_t5__blk70_dn6 * var_t5__blk70) + (var_t5__blk70 * var_t5__blk70_dn6)) / (2.0 * assign5840_e3791)), (((var_t5__blk70_dn7 * var_t5__blk70) + (var_t5__blk70 * var_t5__blk70_dn7)) / (2.0 * assign5840_e3791)), (((var_t5__blk70_dn10 * var_t5__blk70) + (var_t5__blk70 * var_t5__blk70_dn10)) / (2.0 * assign5840_e3791)), (((var_t5__blk70_dn11 * var_t5__blk70) + (var_t5__blk70 * var_t5__blk70_dn11)) / (2.0 * assign5840_e3791)), (((var_t5__blk70_dn12 * var_t5__blk70) + (var_t5__blk70 * var_t5__blk70_dn12)) / (2.0 * assign5840_e3791)), (((var_t5__blk70_dn17 * var_t5__blk70) + (var_t5__blk70 * var_t5__blk70_dn17)) / (2.0 * assign5840_e3791)),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign5840_e3793;
        var_tmf1_dn0 = assign5840_e3793_d_n0;
        var_tmf1_dn2 = assign5840_e3793_d_n2;
        var_tmf1_dn6 = assign5840_e3793_d_n6;
        var_tmf1_dn7 = assign5840_e3793_d_n7;
        var_tmf1_dn10 = assign5840_e3793_d_n10;
        var_tmf1_dn11 = assign5840_e3793_d_n11;
        var_tmf1_dn12 = assign5840_e3793_d_n12;
        var_tmf1_dn17 = assign5840_e3793_d_n17;
        var_tmf1_rv = 0.0;

        let (assign5850_e3806, assign5850_e3806_d_n0, assign5850_e3806_d_n2, assign5850_e3806_d_n6, assign5850_e3806_d_n7, assign5850_e3806_d_n10, assign5850_e3806_d_n11, assign5850_e3806_d_n12, assign5850_e3806_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign5850_e3799: f64 = (var_t5__blk70 + var_tmf1);
        let assign5850_e3800: f64 = (0.5 * assign5850_e3799);
        let assign5850_e3803: f64 = (1e-10 * 0.0001);
        let assign5850_e3804: f64 = (assign5850_e3800 + assign5850_e3803);
        (assign5850_e3804, (0.5 * (var_t5__blk70_dn0 + var_tmf1_dn0)), (0.5 * (var_t5__blk70_dn2 + var_tmf1_dn2)), (0.5 * (var_t5__blk70_dn6 + var_tmf1_dn6)), (0.5 * (var_t5__blk70_dn7 + var_tmf1_dn7)), (0.5 * (var_t5__blk70_dn10 + var_tmf1_dn10)), (0.5 * (var_t5__blk70_dn11 + var_tmf1_dn11)), (0.5 * (var_t5__blk70_dn12 + var_tmf1_dn12)), (0.5 * (var_t5__blk70_dn17 + var_tmf1_dn17)),)
    } else {
        (var_t2__blk66, var_t2__blk66_dn0, var_t2__blk66_dn2, var_t2__blk66_dn6, var_t2__blk66_dn7, var_t2__blk66_dn10, var_t2__blk66_dn11, var_t2__blk66_dn12, var_t2__blk66_dn17,)
    }
};
        var_t2__blk66 = assign5850_e3806;
        var_t2__blk66_dn0 = assign5850_e3806_d_n0;
        var_t2__blk66_dn2 = assign5850_e3806_d_n2;
        var_t2__blk66_dn6 = assign5850_e3806_d_n6;
        var_t2__blk66_dn7 = assign5850_e3806_d_n7;
        var_t2__blk66_dn10 = assign5850_e3806_d_n10;
        var_t2__blk66_dn11 = assign5850_e3806_d_n11;
        var_t2__blk66_dn12 = assign5850_e3806_d_n12;
        var_t2__blk66_dn17 = assign5850_e3806_d_n17;
        var_t2__blk66_rv = 0.0;

        let assign5860_e3809: f64 = if var_t2__blk66 < 0.0 { 1.0 } else { 0.0 };
        var_guard74 = assign5860_e3809;
        var_guard74_rv = 0.0;

        let (assign5870_e3816, assign5870_e3816_d_n0, assign5870_e3816_d_n2, assign5870_e3816_d_n6, assign5870_e3816_d_n7, assign5870_e3816_d_n10, assign5870_e3816_d_n11, assign5870_e3816_d_n12, assign5870_e3816_d_n17,) = {
    if ((var_guard73 == 0.0) && (var_guard74 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2__blk66, var_t2__blk66_dn0, var_t2__blk66_dn2, var_t2__blk66_dn6, var_t2__blk66_dn7, var_t2__blk66_dn10, var_t2__blk66_dn11, var_t2__blk66_dn12, var_t2__blk66_dn17,)
    }
};
        var_t2__blk66 = assign5870_e3816;
        var_t2__blk66_dn0 = assign5870_e3816_d_n0;
        var_t2__blk66_dn2 = assign5870_e3816_d_n2;
        var_t2__blk66_dn6 = assign5870_e3816_d_n6;
        var_t2__blk66_dn7 = assign5870_e3816_d_n7;
        var_t2__blk66_dn10 = assign5870_e3816_d_n10;
        var_t2__blk66_dn11 = assign5870_e3816_d_n11;
        var_t2__blk66_dn12 = assign5870_e3816_d_n12;
        var_t2__blk66_dn17 = assign5870_e3816_d_n17;
        var_t2__blk66_rv = 0.0;

        let (assign5880_e3823, assign5880_e3823_d_n0, assign5880_e3823_d_n2, assign5880_e3823_d_n6, assign5880_e3823_d_n7, assign5880_e3823_d_n10, assign5880_e3823_d_n11, assign5880_e3823_d_n12, assign5880_e3823_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign5880_e3821: f64 = (1.0 / var_t2__blk66);
        (assign5880_e3821, (-(var_t2__blk66_dn0 / (var_t2__blk66 * var_t2__blk66))), (-(var_t2__blk66_dn2 / (var_t2__blk66 * var_t2__blk66))), (-(var_t2__blk66_dn6 / (var_t2__blk66 * var_t2__blk66))), (-(var_t2__blk66_dn7 / (var_t2__blk66 * var_t2__blk66))), (-(var_t2__blk66_dn10 / (var_t2__blk66 * var_t2__blk66))), (-(var_t2__blk66_dn11 / (var_t2__blk66 * var_t2__blk66))), (-(var_t2__blk66_dn12 / (var_t2__blk66 * var_t2__blk66))), (-(var_t2__blk66_dn17 / (var_t2__blk66 * var_t2__blk66))),)
    } else {
        (var_t3__blk67, var_t3__blk67_dn0, var_t3__blk67_dn2, var_t3__blk67_dn6, var_t3__blk67_dn7, var_t3__blk67_dn10, var_t3__blk67_dn11, var_t3__blk67_dn12, var_t3__blk67_dn17,)
    }
};
        var_t3__blk67 = assign5880_e3823;
        var_t3__blk67_dn0 = assign5880_e3823_d_n0;
        var_t3__blk67_dn2 = assign5880_e3823_d_n2;
        var_t3__blk67_dn6 = assign5880_e3823_d_n6;
        var_t3__blk67_dn7 = assign5880_e3823_d_n7;
        var_t3__blk67_dn10 = assign5880_e3823_d_n10;
        var_t3__blk67_dn11 = assign5880_e3823_d_n11;
        var_t3__blk67_dn12 = assign5880_e3823_d_n12;
        var_t3__blk67_dn17 = assign5880_e3823_d_n17;
        var_t3__blk67_rv = 0.0;

        let (assign5890_e3831, assign5890_e3831_d_n0, assign5890_e3831_d_n2, assign5890_e3831_d_n6, assign5890_e3831_d_n7, assign5890_e3831_d_n10, assign5890_e3831_d_n11, assign5890_e3831_d_n12, assign5890_e3831_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign5890_e3828: f64 = (var_vthq).abs();
        let assign5890_e3829: f64 = (2.0 * assign5890_e3828);
        (assign5890_e3829, (2.0 * if var_vthq >= 0.0 { var_vthq_dn0 } else { (-var_vthq_dn0) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn2 } else { (-var_vthq_dn2) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn6 } else { (-var_vthq_dn6) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn7 } else { (-var_vthq_dn7) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn10 } else { (-var_vthq_dn10) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn11 } else { (-var_vthq_dn11) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn12 } else { (-var_vthq_dn12) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn17 } else { (-var_vthq_dn17) }),)
    } else {
        (var_t4w, var_t4w_dn0, var_t4w_dn2, var_t4w_dn6, var_t4w_dn7, var_t4w_dn10, var_t4w_dn11, var_t4w_dn12, var_t4w_dn17,)
    }
};
        var_t4w = assign5890_e3831;
        var_t4w_dn0 = assign5890_e3831_d_n0;
        var_t4w_dn2 = assign5890_e3831_d_n2;
        var_t4w_dn6 = assign5890_e3831_d_n6;
        var_t4w_dn7 = assign5890_e3831_d_n7;
        var_t4w_dn10 = assign5890_e3831_d_n10;
        var_t4w_dn11 = assign5890_e3831_d_n11;
        var_t4w_dn12 = assign5890_e3831_d_n12;
        var_t4w_dn17 = assign5890_e3831_d_n17;
        var_t4w_rv = 0.0;

        let (assign5900_e3840, assign5900_e3840_d_n0, assign5900_e3840_d_n2, assign5900_e3840_d_n6, assign5900_e3840_d_n7, assign5900_e3840_d_n10, assign5900_e3840_d_n11, assign5900_e3840_d_n12, assign5900_e3840_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign5900_e3836: f64 = (var_vfb - var_vthq);
        let assign5900_e3838: f64 = (assign5900_e3836 + p.p205);
        (assign5900_e3838, (-var_vthq_dn0), (-var_vthq_dn2), (-var_vthq_dn6), (-var_vthq_dn7), (-var_vthq_dn10), (-var_vthq_dn11), (-var_vthq_dn12), (-var_vthq_dn17),)
    } else {
        (var_t6__blk71, var_t6__blk71_dn0, var_t6__blk71_dn2, var_t6__blk71_dn6, var_t6__blk71_dn7, var_t6__blk71_dn10, var_t6__blk71_dn11, var_t6__blk71_dn12, var_t6__blk71_dn17,)
    }
};
        var_t6__blk71 = assign5900_e3840;
        var_t6__blk71_dn0 = assign5900_e3840_d_n0;
        var_t6__blk71_dn2 = assign5900_e3840_d_n2;
        var_t6__blk71_dn6 = assign5900_e3840_d_n6;
        var_t6__blk71_dn7 = assign5900_e3840_d_n7;
        var_t6__blk71_dn10 = assign5900_e3840_d_n10;
        var_t6__blk71_dn11 = assign5900_e3840_d_n11;
        var_t6__blk71_dn12 = assign5900_e3840_d_n12;
        var_t6__blk71_dn17 = assign5900_e3840_d_n17;
        var_t6__blk71_rv = 0.0;

        let (assign5910_e3850, assign5910_e3850_d_n0, assign5910_e3850_d_n2, assign5910_e3850_d_n6, assign5910_e3850_d_n7, assign5910_e3850_d_n10, assign5910_e3850_d_n11, assign5910_e3850_d_n12, assign5910_e3850_d_n17,) = {
    if (var_guard73 == 0.0) {
        let (assign5910_e3848, assign5910_e3848_d_n0, assign5910_e3848_d_n2, assign5910_e3848_d_n6, assign5910_e3848_d_n7, assign5910_e3848_d_n10, assign5910_e3848_d_n11, assign5910_e3848_d_n12, assign5910_e3848_d_n17,) = {
            if (var_t6__blk71 > var_t4w) {
                (var_t6__blk71, var_t6__blk71_dn0, var_t6__blk71_dn2, var_t6__blk71_dn6, var_t6__blk71_dn7, var_t6__blk71_dn10, var_t6__blk71_dn11, var_t6__blk71_dn12, var_t6__blk71_dn17,)
            } else {
                (var_t4w, var_t4w_dn0, var_t4w_dn2, var_t4w_dn6, var_t4w_dn7, var_t4w_dn10, var_t4w_dn11, var_t4w_dn12, var_t4w_dn17,)
            }
        };
        (assign5910_e3848, assign5910_e3848_d_n0, assign5910_e3848_d_n2, assign5910_e3848_d_n6, assign5910_e3848_d_n7, assign5910_e3848_d_n10, assign5910_e3848_d_n11, assign5910_e3848_d_n12, assign5910_e3848_d_n17,)
    } else {
        (var_t4__blk68, var_t4__blk68_dn0, var_t4__blk68_dn2, var_t4__blk68_dn6, var_t4__blk68_dn7, var_t4__blk68_dn10, var_t4__blk68_dn11, var_t4__blk68_dn12, var_t4__blk68_dn17,)
    }
};
        var_t4__blk68 = assign5910_e3850;
        var_t4__blk68_dn0 = assign5910_e3850_d_n0;
        var_t4__blk68_dn2 = assign5910_e3850_d_n2;
        var_t4__blk68_dn6 = assign5910_e3850_d_n6;
        var_t4__blk68_dn7 = assign5910_e3850_d_n7;
        var_t4__blk68_dn10 = assign5910_e3850_d_n10;
        var_t4__blk68_dn11 = assign5910_e3850_d_n11;
        var_t4__blk68_dn12 = assign5910_e3850_d_n12;
        var_t4__blk68_dn17 = assign5910_e3850_d_n17;
        var_t4__blk68_rv = 0.0;

        *var_c_fox_slot = var_c_fox;
        *var_c_fox_dn0_slot = var_c_fox_dn0;
        *var_c_fox_dn10_slot = var_c_fox_dn10;
        *var_c_fox_dn11_slot = var_c_fox_dn11;
        *var_c_fox_dn12_slot = var_c_fox_dn12;
        *var_c_fox_dn17_slot = var_c_fox_dn17;
        *var_c_fox_dn2_slot = var_c_fox_dn2;
        *var_c_fox_dn6_slot = var_c_fox_dn6;
        *var_c_fox_dn7_slot = var_c_fox_dn7;
        *var_c_fox_inv_slot = var_c_fox_inv;
        *var_c_fox_inv_dn0_slot = var_c_fox_inv_dn0;
        *var_c_fox_inv_dn10_slot = var_c_fox_inv_dn10;
        *var_c_fox_inv_dn11_slot = var_c_fox_inv_dn11;
        *var_c_fox_inv_dn12_slot = var_c_fox_inv_dn12;
        *var_c_fox_inv_dn17_slot = var_c_fox_inv_dn17;
        *var_c_fox_inv_dn2_slot = var_c_fox_inv_dn2;
        *var_c_fox_inv_dn6_slot = var_c_fox_inv_dn6;
        *var_c_fox_inv_dn7_slot = var_c_fox_inv_dn7;
        *var_c_fox_inv_rv_slot = var_c_fox_inv_rv;
        *var_c_fox_rv_slot = var_c_fox_rv;
        *var_cnstc_foxi_slot = var_cnstc_foxi;
        *var_cnstc_foxi_dn0_slot = var_cnstc_foxi_dn0;
        *var_cnstc_foxi_dn10_slot = var_cnstc_foxi_dn10;
        *var_cnstc_foxi_dn11_slot = var_cnstc_foxi_dn11;
        *var_cnstc_foxi_dn12_slot = var_cnstc_foxi_dn12;
        *var_cnstc_foxi_dn17_slot = var_cnstc_foxi_dn17;
        *var_cnstc_foxi_dn2_slot = var_cnstc_foxi_dn2;
        *var_cnstc_foxi_dn6_slot = var_cnstc_foxi_dn6;
        *var_cnstc_foxi_dn7_slot = var_cnstc_foxi_dn7;
        *var_cnstc_foxi_rv_slot = var_cnstc_foxi_rv;
        *var_flg_qme_slot = var_flg_qme;
        *var_flg_qme_rv_slot = var_flg_qme_rv;
        *var_fmdvds_slot = var_fmdvds;
        *var_fmdvds_dn0_slot = var_fmdvds_dn0;
        *var_fmdvds_dn10_slot = var_fmdvds_dn10;
        *var_fmdvds_dn11_slot = var_fmdvds_dn11;
        *var_fmdvds_dn12_slot = var_fmdvds_dn12;
        *var_fmdvds_dn17_slot = var_fmdvds_dn17;
        *var_fmdvds_dn2_slot = var_fmdvds_dn2;
        *var_fmdvds_dn6_slot = var_fmdvds_dn6;
        *var_fmdvds_dn7_slot = var_fmdvds_dn7;
        *var_fmdvds_rv_slot = var_fmdvds_rv;
        *var_guard72_slot = var_guard72;
        *var_guard72_rv_slot = var_guard72_rv;
        *var_guard73_slot = var_guard73;
        *var_guard73_rv_slot = var_guard73_rv;
        *var_guard74_slot = var_guard74;
        *var_guard74_rv_slot = var_guard74_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1__blk61_slot = var_t1__blk61;
        *var_t1__blk61_dn0_slot = var_t1__blk61_dn0;
        *var_t1__blk61_dn10_slot = var_t1__blk61_dn10;
        *var_t1__blk61_dn11_slot = var_t1__blk61_dn11;
        *var_t1__blk61_dn12_slot = var_t1__blk61_dn12;
        *var_t1__blk61_dn17_slot = var_t1__blk61_dn17;
        *var_t1__blk61_dn2_slot = var_t1__blk61_dn2;
        *var_t1__blk61_dn6_slot = var_t1__blk61_dn6;
        *var_t1__blk61_dn7_slot = var_t1__blk61_dn7;
        *var_t1__blk61_rv_slot = var_t1__blk61_rv;
        *var_t2__blk66_slot = var_t2__blk66;
        *var_t2__blk66_dn0_slot = var_t2__blk66_dn0;
        *var_t2__blk66_dn10_slot = var_t2__blk66_dn10;
        *var_t2__blk66_dn11_slot = var_t2__blk66_dn11;
        *var_t2__blk66_dn12_slot = var_t2__blk66_dn12;
        *var_t2__blk66_dn17_slot = var_t2__blk66_dn17;
        *var_t2__blk66_dn2_slot = var_t2__blk66_dn2;
        *var_t2__blk66_dn6_slot = var_t2__blk66_dn6;
        *var_t2__blk66_dn7_slot = var_t2__blk66_dn7;
        *var_t2__blk66_rv_slot = var_t2__blk66_rv;
        *var_t3__blk67_slot = var_t3__blk67;
        *var_t3__blk67_dn0_slot = var_t3__blk67_dn0;
        *var_t3__blk67_dn10_slot = var_t3__blk67_dn10;
        *var_t3__blk67_dn11_slot = var_t3__blk67_dn11;
        *var_t3__blk67_dn12_slot = var_t3__blk67_dn12;
        *var_t3__blk67_dn17_slot = var_t3__blk67_dn17;
        *var_t3__blk67_dn2_slot = var_t3__blk67_dn2;
        *var_t3__blk67_dn6_slot = var_t3__blk67_dn6;
        *var_t3__blk67_dn7_slot = var_t3__blk67_dn7;
        *var_t3__blk67_rv_slot = var_t3__blk67_rv;
        *var_t4__blk68_slot = var_t4__blk68;
        *var_t4__blk68_dn0_slot = var_t4__blk68_dn0;
        *var_t4__blk68_dn10_slot = var_t4__blk68_dn10;
        *var_t4__blk68_dn11_slot = var_t4__blk68_dn11;
        *var_t4__blk68_dn12_slot = var_t4__blk68_dn12;
        *var_t4__blk68_dn17_slot = var_t4__blk68_dn17;
        *var_t4__blk68_dn2_slot = var_t4__blk68_dn2;
        *var_t4__blk68_dn6_slot = var_t4__blk68_dn6;
        *var_t4__blk68_dn7_slot = var_t4__blk68_dn7;
        *var_t4__blk68_rv_slot = var_t4__blk68_rv;
        *var_t4w_slot = var_t4w;
        *var_t4w_dn0_slot = var_t4w_dn0;
        *var_t4w_dn10_slot = var_t4w_dn10;
        *var_t4w_dn11_slot = var_t4w_dn11;
        *var_t4w_dn12_slot = var_t4w_dn12;
        *var_t4w_dn17_slot = var_t4w_dn17;
        *var_t4w_dn2_slot = var_t4w_dn2;
        *var_t4w_dn6_slot = var_t4w_dn6;
        *var_t4w_dn7_slot = var_t4w_dn7;
        *var_t4w_rv_slot = var_t4w_rv;
        *var_t5__blk70_slot = var_t5__blk70;
        *var_t5__blk70_dn0_slot = var_t5__blk70_dn0;
        *var_t5__blk70_dn10_slot = var_t5__blk70_dn10;
        *var_t5__blk70_dn11_slot = var_t5__blk70_dn11;
        *var_t5__blk70_dn12_slot = var_t5__blk70_dn12;
        *var_t5__blk70_dn17_slot = var_t5__blk70_dn17;
        *var_t5__blk70_dn2_slot = var_t5__blk70_dn2;
        *var_t5__blk70_dn6_slot = var_t5__blk70_dn6;
        *var_t5__blk70_dn7_slot = var_t5__blk70_dn7;
        *var_t5__blk70_rv_slot = var_t5__blk70_rv;
        *var_t6__blk71_slot = var_t6__blk71;
        *var_t6__blk71_dn0_slot = var_t6__blk71_dn0;
        *var_t6__blk71_dn10_slot = var_t6__blk71_dn10;
        *var_t6__blk71_dn11_slot = var_t6__blk71_dn11;
        *var_t6__blk71_dn12_slot = var_t6__blk71_dn12;
        *var_t6__blk71_dn17_slot = var_t6__blk71_dn17;
        *var_t6__blk71_dn2_slot = var_t6__blk71_dn2;
        *var_t6__blk71_dn6_slot = var_t6__blk71_dn6;
        *var_t6__blk71_dn7_slot = var_t6__blk71_dn7;
        *var_t6__blk71_rv_slot = var_t6__blk71_rv;
        *var_tfoxe_slot = var_tfoxe;
        *var_tfoxe_dn0_slot = var_tfoxe_dn0;
        *var_tfoxe_dn10_slot = var_tfoxe_dn10;
        *var_tfoxe_dn11_slot = var_tfoxe_dn11;
        *var_tfoxe_dn12_slot = var_tfoxe_dn12;
        *var_tfoxe_dn17_slot = var_tfoxe_dn17;
        *var_tfoxe_dn2_slot = var_tfoxe_dn2;
        *var_tfoxe_dn6_slot = var_tfoxe_dn6;
        *var_tfoxe_dn7_slot = var_tfoxe_dn7;
        *var_tfoxe_rv_slot = var_tfoxe_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_tmf3_slot = var_tmf3;
        *var_tmf3_dn0_slot = var_tmf3_dn0;
        *var_tmf3_dn10_slot = var_tmf3_dn10;
        *var_tmf3_dn11_slot = var_tmf3_dn11;
        *var_tmf3_dn12_slot = var_tmf3_dn12;
        *var_tmf3_dn17_slot = var_tmf3_dn17;
        *var_tmf3_dn2_slot = var_tmf3_dn2;
        *var_tmf3_dn6_slot = var_tmf3_dn6;
        *var_tmf3_dn7_slot = var_tmf3_dn7;
        *var_tmf3_rv_slot = var_tmf3_rv;
        *var_tmf4_slot = var_tmf4;
        *var_tmf4_dn0_slot = var_tmf4_dn0;
        *var_tmf4_dn10_slot = var_tmf4_dn10;
        *var_tmf4_dn11_slot = var_tmf4_dn11;
        *var_tmf4_dn12_slot = var_tmf4_dn12;
        *var_tmf4_dn17_slot = var_tmf4_dn17;
        *var_tmf4_dn2_slot = var_tmf4_dn2;
        *var_tmf4_dn6_slot = var_tmf4_dn6;
        *var_tmf4_dn7_slot = var_tmf4_dn7;
        *var_tmf4_rv_slot = var_tmf4_rv;
        *var_tx__blk64_slot = var_tx__blk64;
        *var_tx__blk64_dn0_slot = var_tx__blk64_dn0;
        *var_tx__blk64_dn10_slot = var_tx__blk64_dn10;
        *var_tx__blk64_dn11_slot = var_tx__blk64_dn11;
        *var_tx__blk64_dn12_slot = var_tx__blk64_dn12;
        *var_tx__blk64_dn17_slot = var_tx__blk64_dn17;
        *var_tx__blk64_dn2_slot = var_tx__blk64_dn2;
        *var_tx__blk64_dn6_slot = var_tx__blk64_dn6;
        *var_tx__blk64_dn7_slot = var_tx__blk64_dn7;
        *var_tx__blk64_rv_slot = var_tx__blk64_rv;
        *var_vdsats_slot = var_vdsats;
        *var_vdsats_dn0_slot = var_vdsats_dn0;
        *var_vdsats_dn10_slot = var_vdsats_dn10;
        *var_vdsats_dn11_slot = var_vdsats_dn11;
        *var_vdsats_dn12_slot = var_vdsats_dn12;
        *var_vdsats_dn17_slot = var_vdsats_dn17;
        *var_vdsats_dn2_slot = var_vdsats_dn2;
        *var_vdsats_dn6_slot = var_vdsats_dn6;
        *var_vdsats_dn7_slot = var_vdsats_dn7;
        *var_vdsats_rv_slot = var_vdsats_rv;
        *var_vthq_slot = var_vthq;
        *var_vthq_dn0_slot = var_vthq_dn0;
        *var_vthq_dn10_slot = var_vthq_dn10;
        *var_vthq_dn11_slot = var_vthq_dn11;
        *var_vthq_dn12_slot = var_vthq_dn12;
        *var_vthq_dn17_slot = var_vthq_dn17;
        *var_vthq_dn2_slot = var_vthq_dn2;
        *var_vthq_dn6_slot = var_vthq_dn6;
        *var_vthq_dn7_slot = var_vthq_dn7;
        *var_vthq_rv_slot = var_vthq_rv;
    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        var_beta_inv: f64,
        var_beta_inv_dn10: f64,
        var_cnst0soi: f64,
        var_cnst0soi_dn0: f64,
        var_cnst0soi_dn10: f64,
        var_cnst0soi_dn11: f64,
        var_cnst0soi_dn12: f64,
        var_cnst0soi_dn17: f64,
        var_cnst0soi_dn2: f64,
        var_cnst0soi_dn6: f64,
        var_cnst0soi_dn7: f64,
        var_guard73: f64,
        var_pb2: f64,
        var_pb20: f64,
        var_pb20_dn0: f64,
        var_pb20_dn10: f64,
        var_pb20_dn11: f64,
        var_pb20_dn12: f64,
        var_pb20_dn17: f64,
        var_pb20_dn2: f64,
        var_pb20_dn6: f64,
        var_pb20_dn7: f64,
        var_pb2_dn0: f64,
        var_pb2_dn10: f64,
        var_pb2_dn11: f64,
        var_pb2_dn12: f64,
        var_pb2_dn17: f64,
        var_pb2_dn2: f64,
        var_pb2_dn6: f64,
        var_pb2_dn7: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn11: f64,
        var_q_nsub_dn12: f64,
        var_q_nsub_dn17: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn7: f64,
        var_subversion: f64,
        var_t3__blk67: f64,
        var_t3__blk67_dn0: f64,
        var_t3__blk67_dn10: f64,
        var_t3__blk67_dn11: f64,
        var_t3__blk67_dn12: f64,
        var_t3__blk67_dn17: f64,
        var_t3__blk67_dn2: f64,
        var_t3__blk67_dn6: f64,
        var_t3__blk67_dn7: f64,
        var_t4__blk68: f64,
        var_t4__blk68_dn0: f64,
        var_t4__blk68_dn10: f64,
        var_t4__blk68_dn11: f64,
        var_t4__blk68_dn12: f64,
        var_t4__blk68_dn17: f64,
        var_t4__blk68_dn2: f64,
        var_t4__blk68_dn6: f64,
        var_t4__blk68_dn7: f64,
        var_tfox0: f64,
        var_vbspz: f64,
        var_vbspz_dn0: f64,
        var_vbspz_dn10: f64,
        var_vbspz_dn11: f64,
        var_vbspz_dn12: f64,
        var_vbspz_dn17: f64,
        var_vbspz_dn2: f64,
        var_vbspz_dn6: f64,
        var_vbspz_dn7: f64,
        var_c_fox_slot: &mut f64,
        var_c_fox_dn0_slot: &mut f64,
        var_c_fox_dn10_slot: &mut f64,
        var_c_fox_dn11_slot: &mut f64,
        var_c_fox_dn12_slot: &mut f64,
        var_c_fox_dn17_slot: &mut f64,
        var_c_fox_dn2_slot: &mut f64,
        var_c_fox_dn6_slot: &mut f64,
        var_c_fox_dn7_slot: &mut f64,
        var_c_fox_inv_slot: &mut f64,
        var_c_fox_inv_dn0_slot: &mut f64,
        var_c_fox_inv_dn10_slot: &mut f64,
        var_c_fox_inv_dn11_slot: &mut f64,
        var_c_fox_inv_dn12_slot: &mut f64,
        var_c_fox_inv_dn17_slot: &mut f64,
        var_c_fox_inv_dn2_slot: &mut f64,
        var_c_fox_inv_dn6_slot: &mut f64,
        var_c_fox_inv_dn7_slot: &mut f64,
        var_c_fox_inv_rv_slot: &mut f64,
        var_c_fox_rv_slot: &mut f64,
        var_cnstc_foxi_slot: &mut f64,
        var_cnstc_foxi_dn0_slot: &mut f64,
        var_cnstc_foxi_dn10_slot: &mut f64,
        var_cnstc_foxi_dn11_slot: &mut f64,
        var_cnstc_foxi_dn12_slot: &mut f64,
        var_cnstc_foxi_dn17_slot: &mut f64,
        var_cnstc_foxi_dn2_slot: &mut f64,
        var_cnstc_foxi_dn6_slot: &mut f64,
        var_cnstc_foxi_dn7_slot: &mut f64,
        var_cnstc_foxi_rv_slot: &mut f64,
        var_dtfox_slot: &mut f64,
        var_dtfox_dn0_slot: &mut f64,
        var_dtfox_dn10_slot: &mut f64,
        var_dtfox_dn11_slot: &mut f64,
        var_dtfox_dn12_slot: &mut f64,
        var_dtfox_dn17_slot: &mut f64,
        var_dtfox_dn2_slot: &mut f64,
        var_dtfox_dn6_slot: &mut f64,
        var_dtfox_dn7_slot: &mut f64,
        var_dtfox_rv_slot: &mut f64,
        var_flg_qme_slot: &mut f64,
        var_flg_qme_rv_slot: &mut f64,
        var_guard75_slot: &mut f64,
        var_guard75_rv_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard76_rv_slot: &mut f64,
        var_guard77_slot: &mut f64,
        var_guard77_rv_slot: &mut f64,
        var_t2__blk66_slot: &mut f64,
        var_t2__blk66_dn0_slot: &mut f64,
        var_t2__blk66_dn10_slot: &mut f64,
        var_t2__blk66_dn11_slot: &mut f64,
        var_t2__blk66_dn12_slot: &mut f64,
        var_t2__blk66_dn17_slot: &mut f64,
        var_t2__blk66_dn2_slot: &mut f64,
        var_t2__blk66_dn6_slot: &mut f64,
        var_t2__blk66_dn7_slot: &mut f64,
        var_t2__blk66_rv_slot: &mut f64,
        var_tfoxe_slot: &mut f64,
        var_tfoxe_dn0_slot: &mut f64,
        var_tfoxe_dn10_slot: &mut f64,
        var_tfoxe_dn11_slot: &mut f64,
        var_tfoxe_dn12_slot: &mut f64,
        var_tfoxe_dn17_slot: &mut f64,
        var_tfoxe_dn2_slot: &mut f64,
        var_tfoxe_dn6_slot: &mut f64,
        var_tfoxe_dn7_slot: &mut f64,
        var_tfoxe_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vbslim_slot: &mut f64,
        var_vbslim_dn0_slot: &mut f64,
        var_vbslim_dn10_slot: &mut f64,
        var_vbslim_dn11_slot: &mut f64,
        var_vbslim_dn12_slot: &mut f64,
        var_vbslim_dn17_slot: &mut f64,
        var_vbslim_dn2_slot: &mut f64,
        var_vbslim_dn6_slot: &mut f64,
        var_vbslim_dn7_slot: &mut f64,
        var_vbslim_rv_slot: &mut f64,
        var_vbsz2_slot: &mut f64,
        var_vbsz2_dn0_slot: &mut f64,
        var_vbsz2_dn10_slot: &mut f64,
        var_vbsz2_dn11_slot: &mut f64,
        var_vbsz2_dn12_slot: &mut f64,
        var_vbsz2_dn17_slot: &mut f64,
        var_vbsz2_dn2_slot: &mut f64,
        var_vbsz2_dn6_slot: &mut f64,
        var_vbsz2_dn7_slot: &mut f64,
        var_vbsz2_rv_slot: &mut f64,
    ) {
        let mut var_c_fox: f64 = *var_c_fox_slot;
        let mut var_c_fox_dn0: f64 = *var_c_fox_dn0_slot;
        let mut var_c_fox_dn10: f64 = *var_c_fox_dn10_slot;
        let mut var_c_fox_dn11: f64 = *var_c_fox_dn11_slot;
        let mut var_c_fox_dn12: f64 = *var_c_fox_dn12_slot;
        let mut var_c_fox_dn17: f64 = *var_c_fox_dn17_slot;
        let mut var_c_fox_dn2: f64 = *var_c_fox_dn2_slot;
        let mut var_c_fox_dn6: f64 = *var_c_fox_dn6_slot;
        let mut var_c_fox_dn7: f64 = *var_c_fox_dn7_slot;
        let mut var_c_fox_inv: f64 = *var_c_fox_inv_slot;
        let mut var_c_fox_inv_dn0: f64 = *var_c_fox_inv_dn0_slot;
        let mut var_c_fox_inv_dn10: f64 = *var_c_fox_inv_dn10_slot;
        let mut var_c_fox_inv_dn11: f64 = *var_c_fox_inv_dn11_slot;
        let mut var_c_fox_inv_dn12: f64 = *var_c_fox_inv_dn12_slot;
        let mut var_c_fox_inv_dn17: f64 = *var_c_fox_inv_dn17_slot;
        let mut var_c_fox_inv_dn2: f64 = *var_c_fox_inv_dn2_slot;
        let mut var_c_fox_inv_dn6: f64 = *var_c_fox_inv_dn6_slot;
        let mut var_c_fox_inv_dn7: f64 = *var_c_fox_inv_dn7_slot;
        let mut var_c_fox_inv_rv: f64 = *var_c_fox_inv_rv_slot;
        let mut var_c_fox_rv: f64 = *var_c_fox_rv_slot;
        let mut var_cnstc_foxi: f64 = *var_cnstc_foxi_slot;
        let mut var_cnstc_foxi_dn0: f64 = *var_cnstc_foxi_dn0_slot;
        let mut var_cnstc_foxi_dn10: f64 = *var_cnstc_foxi_dn10_slot;
        let mut var_cnstc_foxi_dn11: f64 = *var_cnstc_foxi_dn11_slot;
        let mut var_cnstc_foxi_dn12: f64 = *var_cnstc_foxi_dn12_slot;
        let mut var_cnstc_foxi_dn17: f64 = *var_cnstc_foxi_dn17_slot;
        let mut var_cnstc_foxi_dn2: f64 = *var_cnstc_foxi_dn2_slot;
        let mut var_cnstc_foxi_dn6: f64 = *var_cnstc_foxi_dn6_slot;
        let mut var_cnstc_foxi_dn7: f64 = *var_cnstc_foxi_dn7_slot;
        let mut var_cnstc_foxi_rv: f64 = *var_cnstc_foxi_rv_slot;
        let mut var_dtfox: f64 = *var_dtfox_slot;
        let mut var_dtfox_dn0: f64 = *var_dtfox_dn0_slot;
        let mut var_dtfox_dn10: f64 = *var_dtfox_dn10_slot;
        let mut var_dtfox_dn11: f64 = *var_dtfox_dn11_slot;
        let mut var_dtfox_dn12: f64 = *var_dtfox_dn12_slot;
        let mut var_dtfox_dn17: f64 = *var_dtfox_dn17_slot;
        let mut var_dtfox_dn2: f64 = *var_dtfox_dn2_slot;
        let mut var_dtfox_dn6: f64 = *var_dtfox_dn6_slot;
        let mut var_dtfox_dn7: f64 = *var_dtfox_dn7_slot;
        let mut var_dtfox_rv: f64 = *var_dtfox_rv_slot;
        let mut var_flg_qme: f64 = *var_flg_qme_slot;
        let mut var_flg_qme_rv: f64 = *var_flg_qme_rv_slot;
        let mut var_guard75: f64 = *var_guard75_slot;
        let mut var_guard75_rv: f64 = *var_guard75_rv_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard76_rv: f64 = *var_guard76_rv_slot;
        let mut var_guard77: f64 = *var_guard77_slot;
        let mut var_guard77_rv: f64 = *var_guard77_rv_slot;
        let mut var_t2__blk66: f64 = *var_t2__blk66_slot;
        let mut var_t2__blk66_dn0: f64 = *var_t2__blk66_dn0_slot;
        let mut var_t2__blk66_dn10: f64 = *var_t2__blk66_dn10_slot;
        let mut var_t2__blk66_dn11: f64 = *var_t2__blk66_dn11_slot;
        let mut var_t2__blk66_dn12: f64 = *var_t2__blk66_dn12_slot;
        let mut var_t2__blk66_dn17: f64 = *var_t2__blk66_dn17_slot;
        let mut var_t2__blk66_dn2: f64 = *var_t2__blk66_dn2_slot;
        let mut var_t2__blk66_dn6: f64 = *var_t2__blk66_dn6_slot;
        let mut var_t2__blk66_dn7: f64 = *var_t2__blk66_dn7_slot;
        let mut var_t2__blk66_rv: f64 = *var_t2__blk66_rv_slot;
        let mut var_tfoxe: f64 = *var_tfoxe_slot;
        let mut var_tfoxe_dn0: f64 = *var_tfoxe_dn0_slot;
        let mut var_tfoxe_dn10: f64 = *var_tfoxe_dn10_slot;
        let mut var_tfoxe_dn11: f64 = *var_tfoxe_dn11_slot;
        let mut var_tfoxe_dn12: f64 = *var_tfoxe_dn12_slot;
        let mut var_tfoxe_dn17: f64 = *var_tfoxe_dn17_slot;
        let mut var_tfoxe_dn2: f64 = *var_tfoxe_dn2_slot;
        let mut var_tfoxe_dn6: f64 = *var_tfoxe_dn6_slot;
        let mut var_tfoxe_dn7: f64 = *var_tfoxe_dn7_slot;
        let mut var_tfoxe_rv: f64 = *var_tfoxe_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vbslim: f64 = *var_vbslim_slot;
        let mut var_vbslim_dn0: f64 = *var_vbslim_dn0_slot;
        let mut var_vbslim_dn10: f64 = *var_vbslim_dn10_slot;
        let mut var_vbslim_dn11: f64 = *var_vbslim_dn11_slot;
        let mut var_vbslim_dn12: f64 = *var_vbslim_dn12_slot;
        let mut var_vbslim_dn17: f64 = *var_vbslim_dn17_slot;
        let mut var_vbslim_dn2: f64 = *var_vbslim_dn2_slot;
        let mut var_vbslim_dn6: f64 = *var_vbslim_dn6_slot;
        let mut var_vbslim_dn7: f64 = *var_vbslim_dn7_slot;
        let mut var_vbslim_rv: f64 = *var_vbslim_rv_slot;
        let mut var_vbsz2: f64 = *var_vbsz2_slot;
        let mut var_vbsz2_dn0: f64 = *var_vbsz2_dn0_slot;
        let mut var_vbsz2_dn10: f64 = *var_vbsz2_dn10_slot;
        let mut var_vbsz2_dn11: f64 = *var_vbsz2_dn11_slot;
        let mut var_vbsz2_dn12: f64 = *var_vbsz2_dn12_slot;
        let mut var_vbsz2_dn17: f64 = *var_vbsz2_dn17_slot;
        let mut var_vbsz2_dn2: f64 = *var_vbsz2_dn2_slot;
        let mut var_vbsz2_dn6: f64 = *var_vbsz2_dn6_slot;
        let mut var_vbsz2_dn7: f64 = *var_vbsz2_dn7_slot;
        let mut var_vbsz2_rv: f64 = *var_vbsz2_rv_slot;

        let (assign5920_e3861, assign5920_e3861_d_n0, assign5920_e3861_d_n2, assign5920_e3861_d_n6, assign5920_e3861_d_n7, assign5920_e3861_d_n10, assign5920_e3861_d_n11, assign5920_e3861_d_n12, assign5920_e3861_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign5920_e3855: f64 = (1.0 / var_t4__blk68);
        let assign5920_e3857: f64 = (assign5920_e3855 - var_t3__blk67);
        let assign5920_e3859: f64 = (assign5920_e3857 - 0.0001);
        (assign5920_e3859, ((-(var_t4__blk68_dn0 / (var_t4__blk68 * var_t4__blk68))) - var_t3__blk67_dn0), ((-(var_t4__blk68_dn2 / (var_t4__blk68 * var_t4__blk68))) - var_t3__blk67_dn2), ((-(var_t4__blk68_dn6 / (var_t4__blk68 * var_t4__blk68))) - var_t3__blk67_dn6), ((-(var_t4__blk68_dn7 / (var_t4__blk68 * var_t4__blk68))) - var_t3__blk67_dn7), ((-(var_t4__blk68_dn10 / (var_t4__blk68 * var_t4__blk68))) - var_t3__blk67_dn10), ((-(var_t4__blk68_dn11 / (var_t4__blk68 * var_t4__blk68))) - var_t3__blk67_dn11), ((-(var_t4__blk68_dn12 / (var_t4__blk68 * var_t4__blk68))) - var_t3__blk67_dn12), ((-(var_t4__blk68_dn17 / (var_t4__blk68 * var_t4__blk68))) - var_t3__blk67_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign5920_e3861;
        var_tmf1_dn0 = assign5920_e3861_d_n0;
        var_tmf1_dn2 = assign5920_e3861_d_n2;
        var_tmf1_dn6 = assign5920_e3861_d_n6;
        var_tmf1_dn7 = assign5920_e3861_d_n7;
        var_tmf1_dn10 = assign5920_e3861_d_n10;
        var_tmf1_dn11 = assign5920_e3861_d_n11;
        var_tmf1_dn12 = assign5920_e3861_d_n12;
        var_tmf1_dn17 = assign5920_e3861_d_n17;
        var_tmf1_rv = 0.0;

        let (assign5930_e3872, assign5930_e3872_d_n0, assign5930_e3872_d_n2, assign5930_e3872_d_n6, assign5930_e3872_d_n7, assign5930_e3872_d_n10, assign5930_e3872_d_n11, assign5930_e3872_d_n12, assign5930_e3872_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign5930_e3867: f64 = (1.0 / var_t4__blk68);
        let assign5930_e3868: f64 = (4.0 * assign5930_e3867);
        let assign5930_e3870: f64 = (assign5930_e3868 * 0.0001);
        (assign5930_e3870, ((4.0 * (-(var_t4__blk68_dn0 / (var_t4__blk68 * var_t4__blk68)))) * 0.0001), ((4.0 * (-(var_t4__blk68_dn2 / (var_t4__blk68 * var_t4__blk68)))) * 0.0001), ((4.0 * (-(var_t4__blk68_dn6 / (var_t4__blk68 * var_t4__blk68)))) * 0.0001), ((4.0 * (-(var_t4__blk68_dn7 / (var_t4__blk68 * var_t4__blk68)))) * 0.0001), ((4.0 * (-(var_t4__blk68_dn10 / (var_t4__blk68 * var_t4__blk68)))) * 0.0001), ((4.0 * (-(var_t4__blk68_dn11 / (var_t4__blk68 * var_t4__blk68)))) * 0.0001), ((4.0 * (-(var_t4__blk68_dn12 / (var_t4__blk68 * var_t4__blk68)))) * 0.0001), ((4.0 * (-(var_t4__blk68_dn17 / (var_t4__blk68 * var_t4__blk68)))) * 0.0001),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign5930_e3872;
        var_tmf2_dn0 = assign5930_e3872_d_n0;
        var_tmf2_dn2 = assign5930_e3872_d_n2;
        var_tmf2_dn6 = assign5930_e3872_d_n6;
        var_tmf2_dn7 = assign5930_e3872_d_n7;
        var_tmf2_dn10 = assign5930_e3872_d_n10;
        var_tmf2_dn11 = assign5930_e3872_d_n11;
        var_tmf2_dn12 = assign5930_e3872_d_n12;
        var_tmf2_dn17 = assign5930_e3872_d_n17;
        var_tmf2_rv = 0.0;

        let (assign5940_e3883, assign5940_e3883_d_n0, assign5940_e3883_d_n2, assign5940_e3883_d_n6, assign5940_e3883_d_n7, assign5940_e3883_d_n10, assign5940_e3883_d_n11, assign5940_e3883_d_n12, assign5940_e3883_d_n17,) = {
    if (var_guard73 == 0.0) {
        let (assign5940_e3881, assign5940_e3881_d_n0, assign5940_e3881_d_n2, assign5940_e3881_d_n6, assign5940_e3881_d_n7, assign5940_e3881_d_n10, assign5940_e3881_d_n11, assign5940_e3881_d_n12, assign5940_e3881_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign5940_e3880: f64 = (-var_tmf2);
                (assign5940_e3880, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign5940_e3881, assign5940_e3881_d_n0, assign5940_e3881_d_n2, assign5940_e3881_d_n6, assign5940_e3881_d_n7, assign5940_e3881_d_n10, assign5940_e3881_d_n11, assign5940_e3881_d_n12, assign5940_e3881_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign5940_e3883;
        var_tmf2_dn0 = assign5940_e3883_d_n0;
        var_tmf2_dn2 = assign5940_e3883_d_n2;
        var_tmf2_dn6 = assign5940_e3883_d_n6;
        var_tmf2_dn7 = assign5940_e3883_d_n7;
        var_tmf2_dn10 = assign5940_e3883_d_n10;
        var_tmf2_dn11 = assign5940_e3883_d_n11;
        var_tmf2_dn12 = assign5940_e3883_d_n12;
        var_tmf2_dn17 = assign5940_e3883_d_n17;
        var_tmf2_rv = 0.0;

        let (assign5950_e3893, assign5950_e3893_d_n0, assign5950_e3893_d_n2, assign5950_e3893_d_n6, assign5950_e3893_d_n7, assign5950_e3893_d_n10, assign5950_e3893_d_n11, assign5950_e3893_d_n12, assign5950_e3893_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign5950_e3888: f64 = (var_tmf1 * var_tmf1);
        let assign5950_e3890: f64 = (assign5950_e3888 + var_tmf2);
        let assign5950_e3891: f64 = (assign5950_e3890).sqrt();
        (assign5950_e3891, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5950_e3891)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5950_e3891)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign5950_e3891)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign5950_e3891)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign5950_e3891)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign5950_e3891)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign5950_e3891)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign5950_e3891)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign5950_e3893;
        var_tmf2_dn0 = assign5950_e3893_d_n0;
        var_tmf2_dn2 = assign5950_e3893_d_n2;
        var_tmf2_dn6 = assign5950_e3893_d_n6;
        var_tmf2_dn7 = assign5950_e3893_d_n7;
        var_tmf2_dn10 = assign5950_e3893_d_n10;
        var_tmf2_dn11 = assign5950_e3893_d_n11;
        var_tmf2_dn12 = assign5950_e3893_d_n12;
        var_tmf2_dn17 = assign5950_e3893_d_n17;
        var_tmf2_rv = 0.0;

        let (assign5960_e3906, assign5960_e3906_d_n0, assign5960_e3906_d_n2, assign5960_e3906_d_n6, assign5960_e3906_d_n7, assign5960_e3906_d_n10, assign5960_e3906_d_n11, assign5960_e3906_d_n12, assign5960_e3906_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign5960_e3898: f64 = (1.0 / var_t4__blk68);
        let assign5960_e3902: f64 = (var_tmf1 + var_tmf2);
        let assign5960_e3903: f64 = (0.5 * assign5960_e3902);
        let assign5960_e3904: f64 = (assign5960_e3898 - assign5960_e3903);
        (assign5960_e3904, ((-(var_t4__blk68_dn0 / (var_t4__blk68 * var_t4__blk68))) - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), ((-(var_t4__blk68_dn2 / (var_t4__blk68 * var_t4__blk68))) - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), ((-(var_t4__blk68_dn6 / (var_t4__blk68 * var_t4__blk68))) - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), ((-(var_t4__blk68_dn7 / (var_t4__blk68 * var_t4__blk68))) - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), ((-(var_t4__blk68_dn10 / (var_t4__blk68 * var_t4__blk68))) - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), ((-(var_t4__blk68_dn11 / (var_t4__blk68 * var_t4__blk68))) - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), ((-(var_t4__blk68_dn12 / (var_t4__blk68 * var_t4__blk68))) - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), ((-(var_t4__blk68_dn17 / (var_t4__blk68 * var_t4__blk68))) - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_t2__blk66, var_t2__blk66_dn0, var_t2__blk66_dn2, var_t2__blk66_dn6, var_t2__blk66_dn7, var_t2__blk66_dn10, var_t2__blk66_dn11, var_t2__blk66_dn12, var_t2__blk66_dn17,)
    }
};
        var_t2__blk66 = assign5960_e3906;
        var_t2__blk66_dn0 = assign5960_e3906_d_n0;
        var_t2__blk66_dn2 = assign5960_e3906_d_n2;
        var_t2__blk66_dn6 = assign5960_e3906_d_n6;
        var_t2__blk66_dn7 = assign5960_e3906_d_n7;
        var_t2__blk66_dn10 = assign5960_e3906_d_n10;
        var_t2__blk66_dn11 = assign5960_e3906_d_n11;
        var_t2__blk66_dn12 = assign5960_e3906_d_n12;
        var_t2__blk66_dn17 = assign5960_e3906_d_n17;
        var_t2__blk66_rv = 0.0;

        let (assign5970_e3915, assign5970_e3915_d_n0, assign5970_e3915_d_n2, assign5970_e3915_d_n6, assign5970_e3915_d_n7, assign5970_e3915_d_n10, assign5970_e3915_d_n11, assign5970_e3915_d_n12, assign5970_e3915_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign5970_e3911: f64 = (p.p204 * var_t2__blk66);
        let assign5970_e3913: f64 = (assign5970_e3911 + p.p206);
        (assign5970_e3913, (p.p204 * var_t2__blk66_dn0), (p.p204 * var_t2__blk66_dn2), (p.p204 * var_t2__blk66_dn6), (p.p204 * var_t2__blk66_dn7), (p.p204 * var_t2__blk66_dn10), (p.p204 * var_t2__blk66_dn11), (p.p204 * var_t2__blk66_dn12), (p.p204 * var_t2__blk66_dn17),)
    } else {
        (var_dtfox, var_dtfox_dn0, var_dtfox_dn2, var_dtfox_dn6, var_dtfox_dn7, var_dtfox_dn10, var_dtfox_dn11, var_dtfox_dn12, var_dtfox_dn17,)
    }
};
        var_dtfox = assign5970_e3915;
        var_dtfox_dn0 = assign5970_e3915_d_n0;
        var_dtfox_dn2 = assign5970_e3915_d_n2;
        var_dtfox_dn6 = assign5970_e3915_d_n6;
        var_dtfox_dn7 = assign5970_e3915_d_n7;
        var_dtfox_dn10 = assign5970_e3915_d_n10;
        var_dtfox_dn11 = assign5970_e3915_d_n11;
        var_dtfox_dn12 = assign5970_e3915_d_n12;
        var_dtfox_dn17 = assign5970_e3915_d_n17;
        var_dtfox_rv = 0.0;

        let assign5980_e3918: f64 = (var_dtfox * 1000000000000.0);
        let assign5980_e3920: f64 = if assign5980_e3918 < var_tfox0 { 1.0 } else { 0.0 };
        var_guard75 = assign5980_e3920;
        var_guard75_rv = 0.0;

        let (assign5990_e3927, assign5990_e3927_d_n0, assign5990_e3927_d_n2, assign5990_e3927_d_n6, assign5990_e3927_d_n7, assign5990_e3927_d_n10, assign5990_e3927_d_n11, assign5990_e3927_d_n12, assign5990_e3927_d_n17,) = {
    if ((var_guard73 == 0.0) && (var_guard75 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dtfox, var_dtfox_dn0, var_dtfox_dn2, var_dtfox_dn6, var_dtfox_dn7, var_dtfox_dn10, var_dtfox_dn11, var_dtfox_dn12, var_dtfox_dn17,)
    }
};
        var_dtfox = assign5990_e3927;
        var_dtfox_dn0 = assign5990_e3927_d_n0;
        var_dtfox_dn2 = assign5990_e3927_d_n2;
        var_dtfox_dn6 = assign5990_e3927_d_n6;
        var_dtfox_dn7 = assign5990_e3927_d_n7;
        var_dtfox_dn10 = assign5990_e3927_d_n10;
        var_dtfox_dn11 = assign5990_e3927_d_n11;
        var_dtfox_dn12 = assign5990_e3927_d_n12;
        var_dtfox_dn17 = assign5990_e3927_d_n17;
        var_dtfox_rv = 0.0;

        let (assign6000_e3934,) = {
    if ((var_guard73 == 0.0) && (var_guard75 != 0.0)) {
        (0.0,)
    } else {
        (var_flg_qme,)
    }
};
        var_flg_qme = assign6000_e3934;
        var_flg_qme_rv = 0.0;

        let (assign6010_e3941, assign6010_e3941_d_n0, assign6010_e3941_d_n2, assign6010_e3941_d_n6, assign6010_e3941_d_n7, assign6010_e3941_d_n10, assign6010_e3941_d_n11, assign6010_e3941_d_n12, assign6010_e3941_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign6010_e3939: f64 = (var_tfox0 + var_dtfox);
        (assign6010_e3939, var_dtfox_dn0, var_dtfox_dn2, var_dtfox_dn6, var_dtfox_dn7, var_dtfox_dn10, var_dtfox_dn11, var_dtfox_dn12, var_dtfox_dn17,)
    } else {
        (var_tfoxe, var_tfoxe_dn0, var_tfoxe_dn2, var_tfoxe_dn6, var_tfoxe_dn7, var_tfoxe_dn10, var_tfoxe_dn11, var_tfoxe_dn12, var_tfoxe_dn17,)
    }
};
        var_tfoxe = assign6010_e3941;
        var_tfoxe_dn0 = assign6010_e3941_d_n0;
        var_tfoxe_dn2 = assign6010_e3941_d_n2;
        var_tfoxe_dn6 = assign6010_e3941_d_n6;
        var_tfoxe_dn7 = assign6010_e3941_d_n7;
        var_tfoxe_dn10 = assign6010_e3941_d_n10;
        var_tfoxe_dn11 = assign6010_e3941_d_n11;
        var_tfoxe_dn12 = assign6010_e3941_d_n12;
        var_tfoxe_dn17 = assign6010_e3941_d_n17;
        var_tfoxe_rv = 0.0;

        let (assign6020_e3948, assign6020_e3948_d_n0, assign6020_e3948_d_n2, assign6020_e3948_d_n6, assign6020_e3948_d_n7, assign6020_e3948_d_n10, assign6020_e3948_d_n11, assign6020_e3948_d_n12, assign6020_e3948_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign6020_e3946: f64 = (3.453133e-11 / var_tfoxe);
        (assign6020_e3946, (-((3.453133e-11 * var_tfoxe_dn0) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn2) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn6) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn7) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn10) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn11) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn12) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn17) / (var_tfoxe * var_tfoxe))),)
    } else {
        (var_c_fox, var_c_fox_dn0, var_c_fox_dn2, var_c_fox_dn6, var_c_fox_dn7, var_c_fox_dn10, var_c_fox_dn11, var_c_fox_dn12, var_c_fox_dn17,)
    }
};
        var_c_fox = assign6020_e3948;
        var_c_fox_dn0 = assign6020_e3948_d_n0;
        var_c_fox_dn2 = assign6020_e3948_d_n2;
        var_c_fox_dn6 = assign6020_e3948_d_n6;
        var_c_fox_dn7 = assign6020_e3948_d_n7;
        var_c_fox_dn10 = assign6020_e3948_d_n10;
        var_c_fox_dn11 = assign6020_e3948_d_n11;
        var_c_fox_dn12 = assign6020_e3948_d_n12;
        var_c_fox_dn17 = assign6020_e3948_d_n17;
        var_c_fox_rv = 0.0;

        let (assign6030_e3955, assign6030_e3955_d_n0, assign6030_e3955_d_n2, assign6030_e3955_d_n6, assign6030_e3955_d_n7, assign6030_e3955_d_n10, assign6030_e3955_d_n11, assign6030_e3955_d_n12, assign6030_e3955_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign6030_e3953: f64 = (var_tfoxe / 3.453133e-11);
        (assign6030_e3953, (var_tfoxe_dn0 / 3.453133e-11), (var_tfoxe_dn2 / 3.453133e-11), (var_tfoxe_dn6 / 3.453133e-11), (var_tfoxe_dn7 / 3.453133e-11), (var_tfoxe_dn10 / 3.453133e-11), (var_tfoxe_dn11 / 3.453133e-11), (var_tfoxe_dn12 / 3.453133e-11), (var_tfoxe_dn17 / 3.453133e-11),)
    } else {
        (var_c_fox_inv, var_c_fox_inv_dn0, var_c_fox_inv_dn2, var_c_fox_inv_dn6, var_c_fox_inv_dn7, var_c_fox_inv_dn10, var_c_fox_inv_dn11, var_c_fox_inv_dn12, var_c_fox_inv_dn17,)
    }
};
        var_c_fox_inv = assign6030_e3955;
        var_c_fox_inv_dn0 = assign6030_e3955_d_n0;
        var_c_fox_inv_dn2 = assign6030_e3955_d_n2;
        var_c_fox_inv_dn6 = assign6030_e3955_d_n6;
        var_c_fox_inv_dn7 = assign6030_e3955_d_n7;
        var_c_fox_inv_dn10 = assign6030_e3955_d_n10;
        var_c_fox_inv_dn11 = assign6030_e3955_d_n11;
        var_c_fox_inv_dn12 = assign6030_e3955_d_n12;
        var_c_fox_inv_dn17 = assign6030_e3955_d_n17;
        var_c_fox_inv_rv = 0.0;

        let (assign6040_e3966, assign6040_e3966_d_n0, assign6040_e3966_d_n2, assign6040_e3966_d_n6, assign6040_e3966_d_n7, assign6040_e3966_d_n10, assign6040_e3966_d_n11, assign6040_e3966_d_n12, assign6040_e3966_d_n17,) = {
    if (var_guard73 == 0.0) {
        let assign6040_e3960: f64 = (var_cnst0soi * var_cnst0soi);
        let assign6040_e3962: f64 = (assign6040_e3960 * var_c_fox_inv);
        let assign6040_e3964: f64 = (assign6040_e3962 * var_c_fox_inv);
        (assign6040_e3964, ((((((var_cnst0soi_dn0 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn0)) * var_c_fox_inv) + (assign6040_e3960 * var_c_fox_inv_dn0)) * var_c_fox_inv) + (assign6040_e3962 * var_c_fox_inv_dn0)), ((((((var_cnst0soi_dn2 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn2)) * var_c_fox_inv) + (assign6040_e3960 * var_c_fox_inv_dn2)) * var_c_fox_inv) + (assign6040_e3962 * var_c_fox_inv_dn2)), ((((((var_cnst0soi_dn6 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn6)) * var_c_fox_inv) + (assign6040_e3960 * var_c_fox_inv_dn6)) * var_c_fox_inv) + (assign6040_e3962 * var_c_fox_inv_dn6)), ((((((var_cnst0soi_dn7 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn7)) * var_c_fox_inv) + (assign6040_e3960 * var_c_fox_inv_dn7)) * var_c_fox_inv) + (assign6040_e3962 * var_c_fox_inv_dn7)), ((((((var_cnst0soi_dn10 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn10)) * var_c_fox_inv) + (assign6040_e3960 * var_c_fox_inv_dn10)) * var_c_fox_inv) + (assign6040_e3962 * var_c_fox_inv_dn10)), ((((((var_cnst0soi_dn11 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn11)) * var_c_fox_inv) + (assign6040_e3960 * var_c_fox_inv_dn11)) * var_c_fox_inv) + (assign6040_e3962 * var_c_fox_inv_dn11)), ((((((var_cnst0soi_dn12 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn12)) * var_c_fox_inv) + (assign6040_e3960 * var_c_fox_inv_dn12)) * var_c_fox_inv) + (assign6040_e3962 * var_c_fox_inv_dn12)), ((((((var_cnst0soi_dn17 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn17)) * var_c_fox_inv) + (assign6040_e3960 * var_c_fox_inv_dn17)) * var_c_fox_inv) + (assign6040_e3962 * var_c_fox_inv_dn17)),)
    } else {
        (var_cnstc_foxi, var_cnstc_foxi_dn0, var_cnstc_foxi_dn2, var_cnstc_foxi_dn6, var_cnstc_foxi_dn7, var_cnstc_foxi_dn10, var_cnstc_foxi_dn11, var_cnstc_foxi_dn12, var_cnstc_foxi_dn17,)
    }
};
        var_cnstc_foxi = assign6040_e3966;
        var_cnstc_foxi_dn0 = assign6040_e3966_d_n0;
        var_cnstc_foxi_dn2 = assign6040_e3966_d_n2;
        var_cnstc_foxi_dn6 = assign6040_e3966_d_n6;
        var_cnstc_foxi_dn7 = assign6040_e3966_d_n7;
        var_cnstc_foxi_dn10 = assign6040_e3966_d_n10;
        var_cnstc_foxi_dn11 = assign6040_e3966_d_n11;
        var_cnstc_foxi_dn12 = assign6040_e3966_d_n12;
        var_cnstc_foxi_dn17 = assign6040_e3966_d_n17;
        var_cnstc_foxi_rv = 0.0;

        let assign6050_e3973: f64 = if ((p.p43 == 1.0) || (var_subversion < 3.0)) { 1.0 } else { 0.0 };
        var_guard76 = assign6050_e3973;
        var_guard76_rv = 0.0;

        let (assign6060_e3981, assign6060_e3981_d_n0, assign6060_e3981_d_n2, assign6060_e3981_d_n6, assign6060_e3981_d_n7, assign6060_e3981_d_n10, assign6060_e3981_d_n11, assign6060_e3981_d_n12, assign6060_e3981_d_n17,) = {
    if (var_guard76 != 0.0) {
        let assign6060_e3977: f64 = (0.5 - var_vbspz);
        let assign6060_e3979: f64 = (assign6060_e3977 - 0.001);
        (assign6060_e3979, (-var_vbspz_dn0), (-var_vbspz_dn2), (-var_vbspz_dn6), (-var_vbspz_dn7), (-var_vbspz_dn10), (-var_vbspz_dn11), (-var_vbspz_dn12), (-var_vbspz_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign6060_e3981;
        var_tmf1_dn0 = assign6060_e3981_d_n0;
        var_tmf1_dn2 = assign6060_e3981_d_n2;
        var_tmf1_dn6 = assign6060_e3981_d_n6;
        var_tmf1_dn7 = assign6060_e3981_d_n7;
        var_tmf1_dn10 = assign6060_e3981_d_n10;
        var_tmf1_dn11 = assign6060_e3981_d_n11;
        var_tmf1_dn12 = assign6060_e3981_d_n12;
        var_tmf1_dn17 = assign6060_e3981_d_n17;
        var_tmf1_rv = 0.0;

        let (assign6070_e3989, assign6070_e3989_d_n0, assign6070_e3989_d_n2, assign6070_e3989_d_n6, assign6070_e3989_d_n7, assign6070_e3989_d_n10, assign6070_e3989_d_n11, assign6070_e3989_d_n12, assign6070_e3989_d_n17,) = {
    if (var_guard76 != 0.0) {
        let assign6070_e3985: f64 = (4.0 * 0.5);
        let assign6070_e3987: f64 = (assign6070_e3985 * 0.001);
        (assign6070_e3987, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6070_e3989;
        var_tmf2_dn0 = assign6070_e3989_d_n0;
        var_tmf2_dn2 = assign6070_e3989_d_n2;
        var_tmf2_dn6 = assign6070_e3989_d_n6;
        var_tmf2_dn7 = assign6070_e3989_d_n7;
        var_tmf2_dn10 = assign6070_e3989_d_n10;
        var_tmf2_dn11 = assign6070_e3989_d_n11;
        var_tmf2_dn12 = assign6070_e3989_d_n12;
        var_tmf2_dn17 = assign6070_e3989_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6080_e3999, assign6080_e3999_d_n0, assign6080_e3999_d_n2, assign6080_e3999_d_n6, assign6080_e3999_d_n7, assign6080_e3999_d_n10, assign6080_e3999_d_n11, assign6080_e3999_d_n12, assign6080_e3999_d_n17,) = {
    if (var_guard76 != 0.0) {
        let (assign6080_e3997, assign6080_e3997_d_n0, assign6080_e3997_d_n2, assign6080_e3997_d_n6, assign6080_e3997_d_n7, assign6080_e3997_d_n10, assign6080_e3997_d_n11, assign6080_e3997_d_n12, assign6080_e3997_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign6080_e3996: f64 = (-var_tmf2);
                (assign6080_e3996, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign6080_e3997, assign6080_e3997_d_n0, assign6080_e3997_d_n2, assign6080_e3997_d_n6, assign6080_e3997_d_n7, assign6080_e3997_d_n10, assign6080_e3997_d_n11, assign6080_e3997_d_n12, assign6080_e3997_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6080_e3999;
        var_tmf2_dn0 = assign6080_e3999_d_n0;
        var_tmf2_dn2 = assign6080_e3999_d_n2;
        var_tmf2_dn6 = assign6080_e3999_d_n6;
        var_tmf2_dn7 = assign6080_e3999_d_n7;
        var_tmf2_dn10 = assign6080_e3999_d_n10;
        var_tmf2_dn11 = assign6080_e3999_d_n11;
        var_tmf2_dn12 = assign6080_e3999_d_n12;
        var_tmf2_dn17 = assign6080_e3999_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6090_e4008, assign6090_e4008_d_n0, assign6090_e4008_d_n2, assign6090_e4008_d_n6, assign6090_e4008_d_n7, assign6090_e4008_d_n10, assign6090_e4008_d_n11, assign6090_e4008_d_n12, assign6090_e4008_d_n17,) = {
    if (var_guard76 != 0.0) {
        let assign6090_e4003: f64 = (var_tmf1 * var_tmf1);
        let assign6090_e4005: f64 = (assign6090_e4003 + var_tmf2);
        let assign6090_e4006: f64 = (assign6090_e4005).sqrt();
        (assign6090_e4006, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign6090_e4006)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign6090_e4006)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign6090_e4006)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign6090_e4006)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign6090_e4006)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign6090_e4006)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign6090_e4006)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign6090_e4006)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6090_e4008;
        var_tmf2_dn0 = assign6090_e4008_d_n0;
        var_tmf2_dn2 = assign6090_e4008_d_n2;
        var_tmf2_dn6 = assign6090_e4008_d_n6;
        var_tmf2_dn7 = assign6090_e4008_d_n7;
        var_tmf2_dn10 = assign6090_e4008_d_n10;
        var_tmf2_dn11 = assign6090_e4008_d_n11;
        var_tmf2_dn12 = assign6090_e4008_d_n12;
        var_tmf2_dn17 = assign6090_e4008_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6100_e4018, assign6100_e4018_d_n0, assign6100_e4018_d_n2, assign6100_e4018_d_n6, assign6100_e4018_d_n7, assign6100_e4018_d_n10, assign6100_e4018_d_n11, assign6100_e4018_d_n12, assign6100_e4018_d_n17,) = {
    if (var_guard76 != 0.0) {
        let assign6100_e4014: f64 = (var_tmf1 + var_tmf2);
        let assign6100_e4015: f64 = (0.5 * assign6100_e4014);
        let assign6100_e4016: f64 = (0.5 - assign6100_e4015);
        (assign6100_e4016, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (-(0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (-(0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (-(0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (-(0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), (-(0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_vbsz2, var_vbsz2_dn0, var_vbsz2_dn2, var_vbsz2_dn6, var_vbsz2_dn7, var_vbsz2_dn10, var_vbsz2_dn11, var_vbsz2_dn12, var_vbsz2_dn17,)
    }
};
        var_vbsz2 = assign6100_e4018;
        var_vbsz2_dn0 = assign6100_e4018_d_n0;
        var_vbsz2_dn2 = assign6100_e4018_d_n2;
        var_vbsz2_dn6 = assign6100_e4018_d_n6;
        var_vbsz2_dn7 = assign6100_e4018_d_n7;
        var_vbsz2_dn10 = assign6100_e4018_d_n10;
        var_vbsz2_dn11 = assign6100_e4018_d_n11;
        var_vbsz2_dn12 = assign6100_e4018_d_n12;
        var_vbsz2_dn17 = assign6100_e4018_d_n17;
        var_vbsz2_rv = 0.0;

        let (assign6110_e4035, assign6110_e4035_d_n0, assign6110_e4035_d_n2, assign6110_e4035_d_n6, assign6110_e4035_d_n7, assign6110_e4035_d_n10, assign6110_e4035_d_n11, assign6110_e4035_d_n12, assign6110_e4035_d_n17,) = {
    if (var_guard76 != 0.0) {
        let assign6110_e4021: f64 = (-p.p237);
        let assign6110_e4023: f64 = (assign6110_e4021 * p.p237);
        let assign6110_e4025: f64 = (assign6110_e4023 * var_q_nsub);
        let assign6110_e4028: f64 = (2.0 * 1.034943e-10);
        let assign6110_e4029: f64 = (assign6110_e4025 / assign6110_e4028);
        let assign6110_e4031: f64 = (assign6110_e4029 + var_pb2);
        let assign6110_e4033: f64 = (assign6110_e4031 - var_beta_inv);
        (assign6110_e4033, (((assign6110_e4023 * var_q_nsub_dn0) / assign6110_e4028) + var_pb2_dn0), (((assign6110_e4023 * var_q_nsub_dn2) / assign6110_e4028) + var_pb2_dn2), (((assign6110_e4023 * var_q_nsub_dn6) / assign6110_e4028) + var_pb2_dn6), (((assign6110_e4023 * var_q_nsub_dn7) / assign6110_e4028) + var_pb2_dn7), ((((assign6110_e4023 * var_q_nsub_dn10) / assign6110_e4028) + var_pb2_dn10) - var_beta_inv_dn10), (((assign6110_e4023 * var_q_nsub_dn11) / assign6110_e4028) + var_pb2_dn11), (((assign6110_e4023 * var_q_nsub_dn12) / assign6110_e4028) + var_pb2_dn12), (((assign6110_e4023 * var_q_nsub_dn17) / assign6110_e4028) + var_pb2_dn17),)
    } else {
        (var_vbslim, var_vbslim_dn0, var_vbslim_dn2, var_vbslim_dn6, var_vbslim_dn7, var_vbslim_dn10, var_vbslim_dn11, var_vbslim_dn12, var_vbslim_dn17,)
    }
};
        var_vbslim = assign6110_e4035;
        var_vbslim_dn0 = assign6110_e4035_d_n0;
        var_vbslim_dn2 = assign6110_e4035_d_n2;
        var_vbslim_dn6 = assign6110_e4035_d_n6;
        var_vbslim_dn7 = assign6110_e4035_d_n7;
        var_vbslim_dn10 = assign6110_e4035_d_n10;
        var_vbslim_dn11 = assign6110_e4035_d_n11;
        var_vbslim_dn12 = assign6110_e4035_d_n12;
        var_vbslim_dn17 = assign6110_e4035_d_n17;
        var_vbslim_rv = 0.0;

        let (assign6120_e4043, assign6120_e4043_d_n0, assign6120_e4043_d_n2, assign6120_e4043_d_n6, assign6120_e4043_d_n7, assign6120_e4043_d_n10, assign6120_e4043_d_n11, assign6120_e4043_d_n12, assign6120_e4043_d_n17,) = {
    if (var_guard76 != 0.0) {
        let assign6120_e4039: f64 = (var_vbsz2 - var_vbslim);
        let assign6120_e4041: f64 = (assign6120_e4039 - 0.001);
        (assign6120_e4041, (var_vbsz2_dn0 - var_vbslim_dn0), (var_vbsz2_dn2 - var_vbslim_dn2), (var_vbsz2_dn6 - var_vbslim_dn6), (var_vbsz2_dn7 - var_vbslim_dn7), (var_vbsz2_dn10 - var_vbslim_dn10), (var_vbsz2_dn11 - var_vbslim_dn11), (var_vbsz2_dn12 - var_vbslim_dn12), (var_vbsz2_dn17 - var_vbslim_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign6120_e4043;
        var_tmf1_dn0 = assign6120_e4043_d_n0;
        var_tmf1_dn2 = assign6120_e4043_d_n2;
        var_tmf1_dn6 = assign6120_e4043_d_n6;
        var_tmf1_dn7 = assign6120_e4043_d_n7;
        var_tmf1_dn10 = assign6120_e4043_d_n10;
        var_tmf1_dn11 = assign6120_e4043_d_n11;
        var_tmf1_dn12 = assign6120_e4043_d_n12;
        var_tmf1_dn17 = assign6120_e4043_d_n17;
        var_tmf1_rv = 0.0;

        let (assign6130_e4051, assign6130_e4051_d_n0, assign6130_e4051_d_n2, assign6130_e4051_d_n6, assign6130_e4051_d_n7, assign6130_e4051_d_n10, assign6130_e4051_d_n11, assign6130_e4051_d_n12, assign6130_e4051_d_n17,) = {
    if (var_guard76 != 0.0) {
        let assign6130_e4047: f64 = (4.0 * var_vbslim);
        let assign6130_e4049: f64 = (assign6130_e4047 * 0.001);
        (assign6130_e4049, ((4.0 * var_vbslim_dn0) * 0.001), ((4.0 * var_vbslim_dn2) * 0.001), ((4.0 * var_vbslim_dn6) * 0.001), ((4.0 * var_vbslim_dn7) * 0.001), ((4.0 * var_vbslim_dn10) * 0.001), ((4.0 * var_vbslim_dn11) * 0.001), ((4.0 * var_vbslim_dn12) * 0.001), ((4.0 * var_vbslim_dn17) * 0.001),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6130_e4051;
        var_tmf2_dn0 = assign6130_e4051_d_n0;
        var_tmf2_dn2 = assign6130_e4051_d_n2;
        var_tmf2_dn6 = assign6130_e4051_d_n6;
        var_tmf2_dn7 = assign6130_e4051_d_n7;
        var_tmf2_dn10 = assign6130_e4051_d_n10;
        var_tmf2_dn11 = assign6130_e4051_d_n11;
        var_tmf2_dn12 = assign6130_e4051_d_n12;
        var_tmf2_dn17 = assign6130_e4051_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6140_e4061, assign6140_e4061_d_n0, assign6140_e4061_d_n2, assign6140_e4061_d_n6, assign6140_e4061_d_n7, assign6140_e4061_d_n10, assign6140_e4061_d_n11, assign6140_e4061_d_n12, assign6140_e4061_d_n17,) = {
    if (var_guard76 != 0.0) {
        let (assign6140_e4059, assign6140_e4059_d_n0, assign6140_e4059_d_n2, assign6140_e4059_d_n6, assign6140_e4059_d_n7, assign6140_e4059_d_n10, assign6140_e4059_d_n11, assign6140_e4059_d_n12, assign6140_e4059_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign6140_e4058: f64 = (-var_tmf2);
                (assign6140_e4058, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign6140_e4059, assign6140_e4059_d_n0, assign6140_e4059_d_n2, assign6140_e4059_d_n6, assign6140_e4059_d_n7, assign6140_e4059_d_n10, assign6140_e4059_d_n11, assign6140_e4059_d_n12, assign6140_e4059_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6140_e4061;
        var_tmf2_dn0 = assign6140_e4061_d_n0;
        var_tmf2_dn2 = assign6140_e4061_d_n2;
        var_tmf2_dn6 = assign6140_e4061_d_n6;
        var_tmf2_dn7 = assign6140_e4061_d_n7;
        var_tmf2_dn10 = assign6140_e4061_d_n10;
        var_tmf2_dn11 = assign6140_e4061_d_n11;
        var_tmf2_dn12 = assign6140_e4061_d_n12;
        var_tmf2_dn17 = assign6140_e4061_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6150_e4070, assign6150_e4070_d_n0, assign6150_e4070_d_n2, assign6150_e4070_d_n6, assign6150_e4070_d_n7, assign6150_e4070_d_n10, assign6150_e4070_d_n11, assign6150_e4070_d_n12, assign6150_e4070_d_n17,) = {
    if (var_guard76 != 0.0) {
        let assign6150_e4065: f64 = (var_tmf1 * var_tmf1);
        let assign6150_e4067: f64 = (assign6150_e4065 + var_tmf2);
        let assign6150_e4068: f64 = (assign6150_e4067).sqrt();
        (assign6150_e4068, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign6150_e4068)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign6150_e4068)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign6150_e4068)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign6150_e4068)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign6150_e4068)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign6150_e4068)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign6150_e4068)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign6150_e4068)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6150_e4070;
        var_tmf2_dn0 = assign6150_e4070_d_n0;
        var_tmf2_dn2 = assign6150_e4070_d_n2;
        var_tmf2_dn6 = assign6150_e4070_d_n6;
        var_tmf2_dn7 = assign6150_e4070_d_n7;
        var_tmf2_dn10 = assign6150_e4070_d_n10;
        var_tmf2_dn11 = assign6150_e4070_d_n11;
        var_tmf2_dn12 = assign6150_e4070_d_n12;
        var_tmf2_dn17 = assign6150_e4070_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6160_e4080, assign6160_e4080_d_n0, assign6160_e4080_d_n2, assign6160_e4080_d_n6, assign6160_e4080_d_n7, assign6160_e4080_d_n10, assign6160_e4080_d_n11, assign6160_e4080_d_n12, assign6160_e4080_d_n17,) = {
    if (var_guard76 != 0.0) {
        let assign6160_e4076: f64 = (var_tmf1 + var_tmf2);
        let assign6160_e4077: f64 = (0.5 * assign6160_e4076);
        let assign6160_e4078: f64 = (var_vbslim + assign6160_e4077);
        (assign6160_e4078, (var_vbslim_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_vbslim_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_vbslim_dn6 + (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_vbslim_dn7 + (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_vbslim_dn10 + (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_vbslim_dn11 + (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_vbslim_dn12 + (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), (var_vbslim_dn17 + (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_vbsz2, var_vbsz2_dn0, var_vbsz2_dn2, var_vbsz2_dn6, var_vbsz2_dn7, var_vbsz2_dn10, var_vbsz2_dn11, var_vbsz2_dn12, var_vbsz2_dn17,)
    }
};
        var_vbsz2 = assign6160_e4080;
        var_vbsz2_dn0 = assign6160_e4080_d_n0;
        var_vbsz2_dn2 = assign6160_e4080_d_n2;
        var_vbsz2_dn6 = assign6160_e4080_d_n6;
        var_vbsz2_dn7 = assign6160_e4080_d_n7;
        var_vbsz2_dn10 = assign6160_e4080_d_n10;
        var_vbsz2_dn11 = assign6160_e4080_d_n11;
        var_vbsz2_dn12 = assign6160_e4080_d_n12;
        var_vbsz2_dn17 = assign6160_e4080_d_n17;
        var_vbsz2_rv = 0.0;

        let assign6170_e4083: f64 = if var_subversion > 2.0 { 1.0 } else { 0.0 };
        var_guard77 = assign6170_e4083;
        var_guard77_rv = 0.0;

        let (assign6180_e4093, assign6180_e4093_d_n0, assign6180_e4093_d_n2, assign6180_e4093_d_n6, assign6180_e4093_d_n7, assign6180_e4093_d_n10, assign6180_e4093_d_n11, assign6180_e4093_d_n12, assign6180_e4093_d_n17,) = {
    if ((var_guard76 != 0.0) && (var_guard77 != 0.0)) {
        let assign6180_e4089: f64 = (var_pb20 - var_vbsz2);
        let assign6180_e4091: f64 = (assign6180_e4089 - 0.001);
        (assign6180_e4091, (var_pb20_dn0 - var_vbsz2_dn0), (var_pb20_dn2 - var_vbsz2_dn2), (var_pb20_dn6 - var_vbsz2_dn6), (var_pb20_dn7 - var_vbsz2_dn7), (var_pb20_dn10 - var_vbsz2_dn10), (var_pb20_dn11 - var_vbsz2_dn11), (var_pb20_dn12 - var_vbsz2_dn12), (var_pb20_dn17 - var_vbsz2_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign6180_e4093;
        var_tmf1_dn0 = assign6180_e4093_d_n0;
        var_tmf1_dn2 = assign6180_e4093_d_n2;
        var_tmf1_dn6 = assign6180_e4093_d_n6;
        var_tmf1_dn7 = assign6180_e4093_d_n7;
        var_tmf1_dn10 = assign6180_e4093_d_n10;
        var_tmf1_dn11 = assign6180_e4093_d_n11;
        var_tmf1_dn12 = assign6180_e4093_d_n12;
        var_tmf1_dn17 = assign6180_e4093_d_n17;
        var_tmf1_rv = 0.0;

        *var_c_fox_slot = var_c_fox;
        *var_c_fox_dn0_slot = var_c_fox_dn0;
        *var_c_fox_dn10_slot = var_c_fox_dn10;
        *var_c_fox_dn11_slot = var_c_fox_dn11;
        *var_c_fox_dn12_slot = var_c_fox_dn12;
        *var_c_fox_dn17_slot = var_c_fox_dn17;
        *var_c_fox_dn2_slot = var_c_fox_dn2;
        *var_c_fox_dn6_slot = var_c_fox_dn6;
        *var_c_fox_dn7_slot = var_c_fox_dn7;
        *var_c_fox_inv_slot = var_c_fox_inv;
        *var_c_fox_inv_dn0_slot = var_c_fox_inv_dn0;
        *var_c_fox_inv_dn10_slot = var_c_fox_inv_dn10;
        *var_c_fox_inv_dn11_slot = var_c_fox_inv_dn11;
        *var_c_fox_inv_dn12_slot = var_c_fox_inv_dn12;
        *var_c_fox_inv_dn17_slot = var_c_fox_inv_dn17;
        *var_c_fox_inv_dn2_slot = var_c_fox_inv_dn2;
        *var_c_fox_inv_dn6_slot = var_c_fox_inv_dn6;
        *var_c_fox_inv_dn7_slot = var_c_fox_inv_dn7;
        *var_c_fox_inv_rv_slot = var_c_fox_inv_rv;
        *var_c_fox_rv_slot = var_c_fox_rv;
        *var_cnstc_foxi_slot = var_cnstc_foxi;
        *var_cnstc_foxi_dn0_slot = var_cnstc_foxi_dn0;
        *var_cnstc_foxi_dn10_slot = var_cnstc_foxi_dn10;
        *var_cnstc_foxi_dn11_slot = var_cnstc_foxi_dn11;
        *var_cnstc_foxi_dn12_slot = var_cnstc_foxi_dn12;
        *var_cnstc_foxi_dn17_slot = var_cnstc_foxi_dn17;
        *var_cnstc_foxi_dn2_slot = var_cnstc_foxi_dn2;
        *var_cnstc_foxi_dn6_slot = var_cnstc_foxi_dn6;
        *var_cnstc_foxi_dn7_slot = var_cnstc_foxi_dn7;
        *var_cnstc_foxi_rv_slot = var_cnstc_foxi_rv;
        *var_dtfox_slot = var_dtfox;
        *var_dtfox_dn0_slot = var_dtfox_dn0;
        *var_dtfox_dn10_slot = var_dtfox_dn10;
        *var_dtfox_dn11_slot = var_dtfox_dn11;
        *var_dtfox_dn12_slot = var_dtfox_dn12;
        *var_dtfox_dn17_slot = var_dtfox_dn17;
        *var_dtfox_dn2_slot = var_dtfox_dn2;
        *var_dtfox_dn6_slot = var_dtfox_dn6;
        *var_dtfox_dn7_slot = var_dtfox_dn7;
        *var_dtfox_rv_slot = var_dtfox_rv;
        *var_flg_qme_slot = var_flg_qme;
        *var_flg_qme_rv_slot = var_flg_qme_rv;
        *var_guard75_slot = var_guard75;
        *var_guard75_rv_slot = var_guard75_rv;
        *var_guard76_slot = var_guard76;
        *var_guard76_rv_slot = var_guard76_rv;
        *var_guard77_slot = var_guard77;
        *var_guard77_rv_slot = var_guard77_rv;
        *var_t2__blk66_slot = var_t2__blk66;
        *var_t2__blk66_dn0_slot = var_t2__blk66_dn0;
        *var_t2__blk66_dn10_slot = var_t2__blk66_dn10;
        *var_t2__blk66_dn11_slot = var_t2__blk66_dn11;
        *var_t2__blk66_dn12_slot = var_t2__blk66_dn12;
        *var_t2__blk66_dn17_slot = var_t2__blk66_dn17;
        *var_t2__blk66_dn2_slot = var_t2__blk66_dn2;
        *var_t2__blk66_dn6_slot = var_t2__blk66_dn6;
        *var_t2__blk66_dn7_slot = var_t2__blk66_dn7;
        *var_t2__blk66_rv_slot = var_t2__blk66_rv;
        *var_tfoxe_slot = var_tfoxe;
        *var_tfoxe_dn0_slot = var_tfoxe_dn0;
        *var_tfoxe_dn10_slot = var_tfoxe_dn10;
        *var_tfoxe_dn11_slot = var_tfoxe_dn11;
        *var_tfoxe_dn12_slot = var_tfoxe_dn12;
        *var_tfoxe_dn17_slot = var_tfoxe_dn17;
        *var_tfoxe_dn2_slot = var_tfoxe_dn2;
        *var_tfoxe_dn6_slot = var_tfoxe_dn6;
        *var_tfoxe_dn7_slot = var_tfoxe_dn7;
        *var_tfoxe_rv_slot = var_tfoxe_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vbslim_slot = var_vbslim;
        *var_vbslim_dn0_slot = var_vbslim_dn0;
        *var_vbslim_dn10_slot = var_vbslim_dn10;
        *var_vbslim_dn11_slot = var_vbslim_dn11;
        *var_vbslim_dn12_slot = var_vbslim_dn12;
        *var_vbslim_dn17_slot = var_vbslim_dn17;
        *var_vbslim_dn2_slot = var_vbslim_dn2;
        *var_vbslim_dn6_slot = var_vbslim_dn6;
        *var_vbslim_dn7_slot = var_vbslim_dn7;
        *var_vbslim_rv_slot = var_vbslim_rv;
        *var_vbsz2_slot = var_vbsz2;
        *var_vbsz2_dn0_slot = var_vbsz2_dn0;
        *var_vbsz2_dn10_slot = var_vbsz2_dn10;
        *var_vbsz2_dn11_slot = var_vbsz2_dn11;
        *var_vbsz2_dn12_slot = var_vbsz2_dn12;
        *var_vbsz2_dn17_slot = var_vbsz2_dn17;
        *var_vbsz2_dn2_slot = var_vbsz2_dn2;
        *var_vbsz2_dn6_slot = var_vbsz2_dn6;
        *var_vbsz2_dn7_slot = var_vbsz2_dn7;
        *var_vbsz2_rv_slot = var_vbsz2_rv;
    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        var_c_fox_inv: f64,
        var_c_fox_inv_dn0: f64,
        var_c_fox_inv_dn10: f64,
        var_c_fox_inv_dn11: f64,
        var_c_fox_inv_dn12: f64,
        var_c_fox_inv_dn17: f64,
        var_c_fox_inv_dn2: f64,
        var_c_fox_inv_dn6: f64,
        var_c_fox_inv_dn7: f64,
        var_guard76: f64,
        var_guard77: f64,
        var_pb20: f64,
        var_pb20_dn0: f64,
        var_pb20_dn10: f64,
        var_pb20_dn11: f64,
        var_pb20_dn12: f64,
        var_pb20_dn17: f64,
        var_pb20_dn2: f64,
        var_pb20_dn6: f64,
        var_pb20_dn7: f64,
        var_pb2c: f64,
        var_pb2c_dn0: f64,
        var_pb2c_dn10: f64,
        var_pb2c_dn11: f64,
        var_pb2c_dn12: f64,
        var_pb2c_dn17: f64,
        var_pb2c_dn2: f64,
        var_pb2c_dn6: f64,
        var_pb2c_dn7: f64,
        var_ptovr: f64,
        var_ptovr_dn0: f64,
        var_ptovr_dn10: f64,
        var_ptovr_dn11: f64,
        var_ptovr_dn12: f64,
        var_ptovr_dn17: f64,
        var_ptovr_dn2: f64,
        var_ptovr_dn6: f64,
        var_ptovr_dn7: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn11: f64,
        var_q_nsub_dn12: f64,
        var_q_nsub_dn17: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn7: f64,
        var_qnsub_esi2: f64,
        var_qnsub_esi2_dn0: f64,
        var_qnsub_esi2_dn10: f64,
        var_qnsub_esi2_dn11: f64,
        var_qnsub_esi2_dn12: f64,
        var_qnsub_esi2_dn17: f64,
        var_qnsub_esi2_dn2: f64,
        var_qnsub_esi2_dn6: f64,
        var_qnsub_esi2_dn7: f64,
        var_subversion: f64,
        var_tmf1: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn10: f64,
        var_tmf1_dn11: f64,
        var_tmf1_dn12: f64,
        var_tmf1_dn17: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn6: f64,
        var_tmf1_dn7: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn17: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn7: f64,
        var_uc_scp3: f64,
        var_vfb: f64,
        var_dvth0__blk89_slot: &mut f64,
        var_dvth0__blk89_dn0_slot: &mut f64,
        var_dvth0__blk89_dn10_slot: &mut f64,
        var_dvth0__blk89_dn11_slot: &mut f64,
        var_dvth0__blk89_dn12_slot: &mut f64,
        var_dvth0__blk89_dn17_slot: &mut f64,
        var_dvth0__blk89_dn2_slot: &mut f64,
        var_dvth0__blk89_dn6_slot: &mut f64,
        var_dvth0__blk89_dn7_slot: &mut f64,
        var_dvth0__blk89_rv_slot: &mut f64,
        var_guard78_slot: &mut f64,
        var_guard78_rv_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard90_rv_slot: &mut f64,
        var_pb20b_slot: &mut f64,
        var_pb20b_dn0_slot: &mut f64,
        var_pb20b_dn10_slot: &mut f64,
        var_pb20b_dn11_slot: &mut f64,
        var_pb20b_dn12_slot: &mut f64,
        var_pb20b_dn17_slot: &mut f64,
        var_pb20b_dn2_slot: &mut f64,
        var_pb20b_dn6_slot: &mut f64,
        var_pb20b_dn7_slot: &mut f64,
        var_pb20b_rv_slot: &mut f64,
        var_pbsum_slot: &mut f64,
        var_pbsum_dn0_slot: &mut f64,
        var_pbsum_dn10_slot: &mut f64,
        var_pbsum_dn11_slot: &mut f64,
        var_pbsum_dn12_slot: &mut f64,
        var_pbsum_dn17_slot: &mut f64,
        var_pbsum_dn2_slot: &mut f64,
        var_pbsum_dn6_slot: &mut f64,
        var_pbsum_dn7_slot: &mut f64,
        var_pbsum_rv_slot: &mut f64,
        var_qb0_slot: &mut f64,
        var_qb0_dn0_slot: &mut f64,
        var_qb0_dn10_slot: &mut f64,
        var_qb0_dn11_slot: &mut f64,
        var_qb0_dn12_slot: &mut f64,
        var_qb0_dn17_slot: &mut f64,
        var_qb0_dn2_slot: &mut f64,
        var_qb0_dn6_slot: &mut f64,
        var_qb0_dn7_slot: &mut f64,
        var_qb0_rv_slot: &mut f64,
        var_sqrt_pbsum_slot: &mut f64,
        var_sqrt_pbsum_dn0_slot: &mut f64,
        var_sqrt_pbsum_dn10_slot: &mut f64,
        var_sqrt_pbsum_dn11_slot: &mut f64,
        var_sqrt_pbsum_dn12_slot: &mut f64,
        var_sqrt_pbsum_dn17_slot: &mut f64,
        var_sqrt_pbsum_dn2_slot: &mut f64,
        var_sqrt_pbsum_dn6_slot: &mut f64,
        var_sqrt_pbsum_dn7_slot: &mut f64,
        var_sqrt_pbsum_rv_slot: &mut f64,
        var_t0__blk80_slot: &mut f64,
        var_t0__blk80_rv_slot: &mut f64,
        var_t0__blk83_slot: &mut f64,
        var_t0__blk83_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1__blk79_slot: &mut f64,
        var_t1__blk79_dn0_slot: &mut f64,
        var_t1__blk79_dn10_slot: &mut f64,
        var_t1__blk79_dn11_slot: &mut f64,
        var_t1__blk79_dn12_slot: &mut f64,
        var_t1__blk79_dn17_slot: &mut f64,
        var_t1__blk79_dn2_slot: &mut f64,
        var_t1__blk79_dn6_slot: &mut f64,
        var_t1__blk79_dn7_slot: &mut f64,
        var_t1__blk79_rv_slot: &mut f64,
        var_t1__blk84_slot: &mut f64,
        var_t1__blk84_dn0_slot: &mut f64,
        var_t1__blk84_dn10_slot: &mut f64,
        var_t1__blk84_dn11_slot: &mut f64,
        var_t1__blk84_dn12_slot: &mut f64,
        var_t1__blk84_dn17_slot: &mut f64,
        var_t1__blk84_dn2_slot: &mut f64,
        var_t1__blk84_dn6_slot: &mut f64,
        var_t1__blk84_dn7_slot: &mut f64,
        var_t1__blk84_rv_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2__blk81_slot: &mut f64,
        var_t2__blk81_dn0_slot: &mut f64,
        var_t2__blk81_dn10_slot: &mut f64,
        var_t2__blk81_dn11_slot: &mut f64,
        var_t2__blk81_dn12_slot: &mut f64,
        var_t2__blk81_dn17_slot: &mut f64,
        var_t2__blk81_dn2_slot: &mut f64,
        var_t2__blk81_dn6_slot: &mut f64,
        var_t2__blk81_dn7_slot: &mut f64,
        var_t2__blk81_rv_slot: &mut f64,
        var_t2__blk85_slot: &mut f64,
        var_t2__blk85_dn0_slot: &mut f64,
        var_t2__blk85_dn10_slot: &mut f64,
        var_t2__blk85_dn11_slot: &mut f64,
        var_t2__blk85_dn12_slot: &mut f64,
        var_t2__blk85_dn17_slot: &mut f64,
        var_t2__blk85_dn2_slot: &mut f64,
        var_t2__blk85_dn6_slot: &mut f64,
        var_t2__blk85_dn7_slot: &mut f64,
        var_t2__blk85_rv_slot: &mut f64,
        var_t3__blk82_slot: &mut f64,
        var_t3__blk82_dn0_slot: &mut f64,
        var_t3__blk82_dn10_slot: &mut f64,
        var_t3__blk82_dn11_slot: &mut f64,
        var_t3__blk82_dn12_slot: &mut f64,
        var_t3__blk82_dn17_slot: &mut f64,
        var_t3__blk82_dn2_slot: &mut f64,
        var_t3__blk82_dn6_slot: &mut f64,
        var_t3__blk82_dn7_slot: &mut f64,
        var_t3__blk82_rv_slot: &mut f64,
        var_t3__blk86_slot: &mut f64,
        var_t3__blk86_dn0_slot: &mut f64,
        var_t3__blk86_dn10_slot: &mut f64,
        var_t3__blk86_dn11_slot: &mut f64,
        var_t3__blk86_dn12_slot: &mut f64,
        var_t3__blk86_dn17_slot: &mut f64,
        var_t3__blk86_dn2_slot: &mut f64,
        var_t3__blk86_dn6_slot: &mut f64,
        var_t3__blk86_dn7_slot: &mut f64,
        var_t3__blk86_rv_slot: &mut f64,
        var_t4__blk87_slot: &mut f64,
        var_t4__blk87_rv_slot: &mut f64,
        var_t5__blk88_slot: &mut f64,
        var_t5__blk88_dn0_slot: &mut f64,
        var_t5__blk88_dn10_slot: &mut f64,
        var_t5__blk88_dn11_slot: &mut f64,
        var_t5__blk88_dn12_slot: &mut f64,
        var_t5__blk88_dn17_slot: &mut f64,
        var_t5__blk88_dn2_slot: &mut f64,
        var_t5__blk88_dn6_slot: &mut f64,
        var_t5__blk88_dn7_slot: &mut f64,
        var_t5__blk88_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vbsz2_slot: &mut f64,
        var_vbsz2_dn0_slot: &mut f64,
        var_vbsz2_dn10_slot: &mut f64,
        var_vbsz2_dn11_slot: &mut f64,
        var_vbsz2_dn12_slot: &mut f64,
        var_vbsz2_dn17_slot: &mut f64,
        var_vbsz2_dn2_slot: &mut f64,
        var_vbsz2_dn6_slot: &mut f64,
        var_vbsz2_dn7_slot: &mut f64,
        var_vbsz2_rv_slot: &mut f64,
        var_vth0_slot: &mut f64,
        var_vth0_dn0_slot: &mut f64,
        var_vth0_dn10_slot: &mut f64,
        var_vth0_dn11_slot: &mut f64,
        var_vth0_dn12_slot: &mut f64,
        var_vth0_dn17_slot: &mut f64,
        var_vth0_dn2_slot: &mut f64,
        var_vth0_dn6_slot: &mut f64,
        var_vth0_dn7_slot: &mut f64,
        var_vth0_rv_slot: &mut f64,
        var_vthp_slot: &mut f64,
        var_vthp_dn0_slot: &mut f64,
        var_vthp_dn10_slot: &mut f64,
        var_vthp_dn11_slot: &mut f64,
        var_vthp_dn12_slot: &mut f64,
        var_vthp_dn17_slot: &mut f64,
        var_vthp_dn2_slot: &mut f64,
        var_vthp_dn6_slot: &mut f64,
        var_vthp_dn7_slot: &mut f64,
        var_vthp_rv_slot: &mut f64,
        var_wd0_slot: &mut f64,
        var_wd0_dn0_slot: &mut f64,
        var_wd0_dn10_slot: &mut f64,
        var_wd0_dn11_slot: &mut f64,
        var_wd0_dn12_slot: &mut f64,
        var_wd0_dn17_slot: &mut f64,
        var_wd0_dn2_slot: &mut f64,
        var_wd0_dn6_slot: &mut f64,
        var_wd0_dn7_slot: &mut f64,
        var_wd0_rv_slot: &mut f64,
    ) {
        let mut var_dvth0__blk89: f64 = *var_dvth0__blk89_slot;
        let mut var_dvth0__blk89_dn0: f64 = *var_dvth0__blk89_dn0_slot;
        let mut var_dvth0__blk89_dn10: f64 = *var_dvth0__blk89_dn10_slot;
        let mut var_dvth0__blk89_dn11: f64 = *var_dvth0__blk89_dn11_slot;
        let mut var_dvth0__blk89_dn12: f64 = *var_dvth0__blk89_dn12_slot;
        let mut var_dvth0__blk89_dn17: f64 = *var_dvth0__blk89_dn17_slot;
        let mut var_dvth0__blk89_dn2: f64 = *var_dvth0__blk89_dn2_slot;
        let mut var_dvth0__blk89_dn6: f64 = *var_dvth0__blk89_dn6_slot;
        let mut var_dvth0__blk89_dn7: f64 = *var_dvth0__blk89_dn7_slot;
        let mut var_dvth0__blk89_rv: f64 = *var_dvth0__blk89_rv_slot;
        let mut var_guard78: f64 = *var_guard78_slot;
        let mut var_guard78_rv: f64 = *var_guard78_rv_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard90_rv: f64 = *var_guard90_rv_slot;
        let mut var_pb20b: f64 = *var_pb20b_slot;
        let mut var_pb20b_dn0: f64 = *var_pb20b_dn0_slot;
        let mut var_pb20b_dn10: f64 = *var_pb20b_dn10_slot;
        let mut var_pb20b_dn11: f64 = *var_pb20b_dn11_slot;
        let mut var_pb20b_dn12: f64 = *var_pb20b_dn12_slot;
        let mut var_pb20b_dn17: f64 = *var_pb20b_dn17_slot;
        let mut var_pb20b_dn2: f64 = *var_pb20b_dn2_slot;
        let mut var_pb20b_dn6: f64 = *var_pb20b_dn6_slot;
        let mut var_pb20b_dn7: f64 = *var_pb20b_dn7_slot;
        let mut var_pb20b_rv: f64 = *var_pb20b_rv_slot;
        let mut var_pbsum: f64 = *var_pbsum_slot;
        let mut var_pbsum_dn0: f64 = *var_pbsum_dn0_slot;
        let mut var_pbsum_dn10: f64 = *var_pbsum_dn10_slot;
        let mut var_pbsum_dn11: f64 = *var_pbsum_dn11_slot;
        let mut var_pbsum_dn12: f64 = *var_pbsum_dn12_slot;
        let mut var_pbsum_dn17: f64 = *var_pbsum_dn17_slot;
        let mut var_pbsum_dn2: f64 = *var_pbsum_dn2_slot;
        let mut var_pbsum_dn6: f64 = *var_pbsum_dn6_slot;
        let mut var_pbsum_dn7: f64 = *var_pbsum_dn7_slot;
        let mut var_pbsum_rv: f64 = *var_pbsum_rv_slot;
        let mut var_qb0: f64 = *var_qb0_slot;
        let mut var_qb0_dn0: f64 = *var_qb0_dn0_slot;
        let mut var_qb0_dn10: f64 = *var_qb0_dn10_slot;
        let mut var_qb0_dn11: f64 = *var_qb0_dn11_slot;
        let mut var_qb0_dn12: f64 = *var_qb0_dn12_slot;
        let mut var_qb0_dn17: f64 = *var_qb0_dn17_slot;
        let mut var_qb0_dn2: f64 = *var_qb0_dn2_slot;
        let mut var_qb0_dn6: f64 = *var_qb0_dn6_slot;
        let mut var_qb0_dn7: f64 = *var_qb0_dn7_slot;
        let mut var_qb0_rv: f64 = *var_qb0_rv_slot;
        let mut var_sqrt_pbsum: f64 = *var_sqrt_pbsum_slot;
        let mut var_sqrt_pbsum_dn0: f64 = *var_sqrt_pbsum_dn0_slot;
        let mut var_sqrt_pbsum_dn10: f64 = *var_sqrt_pbsum_dn10_slot;
        let mut var_sqrt_pbsum_dn11: f64 = *var_sqrt_pbsum_dn11_slot;
        let mut var_sqrt_pbsum_dn12: f64 = *var_sqrt_pbsum_dn12_slot;
        let mut var_sqrt_pbsum_dn17: f64 = *var_sqrt_pbsum_dn17_slot;
        let mut var_sqrt_pbsum_dn2: f64 = *var_sqrt_pbsum_dn2_slot;
        let mut var_sqrt_pbsum_dn6: f64 = *var_sqrt_pbsum_dn6_slot;
        let mut var_sqrt_pbsum_dn7: f64 = *var_sqrt_pbsum_dn7_slot;
        let mut var_sqrt_pbsum_rv: f64 = *var_sqrt_pbsum_rv_slot;
        let mut var_t0__blk80: f64 = *var_t0__blk80_slot;
        let mut var_t0__blk80_rv: f64 = *var_t0__blk80_rv_slot;
        let mut var_t0__blk83: f64 = *var_t0__blk83_slot;
        let mut var_t0__blk83_rv: f64 = *var_t0__blk83_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1__blk79: f64 = *var_t1__blk79_slot;
        let mut var_t1__blk79_dn0: f64 = *var_t1__blk79_dn0_slot;
        let mut var_t1__blk79_dn10: f64 = *var_t1__blk79_dn10_slot;
        let mut var_t1__blk79_dn11: f64 = *var_t1__blk79_dn11_slot;
        let mut var_t1__blk79_dn12: f64 = *var_t1__blk79_dn12_slot;
        let mut var_t1__blk79_dn17: f64 = *var_t1__blk79_dn17_slot;
        let mut var_t1__blk79_dn2: f64 = *var_t1__blk79_dn2_slot;
        let mut var_t1__blk79_dn6: f64 = *var_t1__blk79_dn6_slot;
        let mut var_t1__blk79_dn7: f64 = *var_t1__blk79_dn7_slot;
        let mut var_t1__blk79_rv: f64 = *var_t1__blk79_rv_slot;
        let mut var_t1__blk84: f64 = *var_t1__blk84_slot;
        let mut var_t1__blk84_dn0: f64 = *var_t1__blk84_dn0_slot;
        let mut var_t1__blk84_dn10: f64 = *var_t1__blk84_dn10_slot;
        let mut var_t1__blk84_dn11: f64 = *var_t1__blk84_dn11_slot;
        let mut var_t1__blk84_dn12: f64 = *var_t1__blk84_dn12_slot;
        let mut var_t1__blk84_dn17: f64 = *var_t1__blk84_dn17_slot;
        let mut var_t1__blk84_dn2: f64 = *var_t1__blk84_dn2_slot;
        let mut var_t1__blk84_dn6: f64 = *var_t1__blk84_dn6_slot;
        let mut var_t1__blk84_dn7: f64 = *var_t1__blk84_dn7_slot;
        let mut var_t1__blk84_rv: f64 = *var_t1__blk84_rv_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2__blk81: f64 = *var_t2__blk81_slot;
        let mut var_t2__blk81_dn0: f64 = *var_t2__blk81_dn0_slot;
        let mut var_t2__blk81_dn10: f64 = *var_t2__blk81_dn10_slot;
        let mut var_t2__blk81_dn11: f64 = *var_t2__blk81_dn11_slot;
        let mut var_t2__blk81_dn12: f64 = *var_t2__blk81_dn12_slot;
        let mut var_t2__blk81_dn17: f64 = *var_t2__blk81_dn17_slot;
        let mut var_t2__blk81_dn2: f64 = *var_t2__blk81_dn2_slot;
        let mut var_t2__blk81_dn6: f64 = *var_t2__blk81_dn6_slot;
        let mut var_t2__blk81_dn7: f64 = *var_t2__blk81_dn7_slot;
        let mut var_t2__blk81_rv: f64 = *var_t2__blk81_rv_slot;
        let mut var_t2__blk85: f64 = *var_t2__blk85_slot;
        let mut var_t2__blk85_dn0: f64 = *var_t2__blk85_dn0_slot;
        let mut var_t2__blk85_dn10: f64 = *var_t2__blk85_dn10_slot;
        let mut var_t2__blk85_dn11: f64 = *var_t2__blk85_dn11_slot;
        let mut var_t2__blk85_dn12: f64 = *var_t2__blk85_dn12_slot;
        let mut var_t2__blk85_dn17: f64 = *var_t2__blk85_dn17_slot;
        let mut var_t2__blk85_dn2: f64 = *var_t2__blk85_dn2_slot;
        let mut var_t2__blk85_dn6: f64 = *var_t2__blk85_dn6_slot;
        let mut var_t2__blk85_dn7: f64 = *var_t2__blk85_dn7_slot;
        let mut var_t2__blk85_rv: f64 = *var_t2__blk85_rv_slot;
        let mut var_t3__blk82: f64 = *var_t3__blk82_slot;
        let mut var_t3__blk82_dn0: f64 = *var_t3__blk82_dn0_slot;
        let mut var_t3__blk82_dn10: f64 = *var_t3__blk82_dn10_slot;
        let mut var_t3__blk82_dn11: f64 = *var_t3__blk82_dn11_slot;
        let mut var_t3__blk82_dn12: f64 = *var_t3__blk82_dn12_slot;
        let mut var_t3__blk82_dn17: f64 = *var_t3__blk82_dn17_slot;
        let mut var_t3__blk82_dn2: f64 = *var_t3__blk82_dn2_slot;
        let mut var_t3__blk82_dn6: f64 = *var_t3__blk82_dn6_slot;
        let mut var_t3__blk82_dn7: f64 = *var_t3__blk82_dn7_slot;
        let mut var_t3__blk82_rv: f64 = *var_t3__blk82_rv_slot;
        let mut var_t3__blk86: f64 = *var_t3__blk86_slot;
        let mut var_t3__blk86_dn0: f64 = *var_t3__blk86_dn0_slot;
        let mut var_t3__blk86_dn10: f64 = *var_t3__blk86_dn10_slot;
        let mut var_t3__blk86_dn11: f64 = *var_t3__blk86_dn11_slot;
        let mut var_t3__blk86_dn12: f64 = *var_t3__blk86_dn12_slot;
        let mut var_t3__blk86_dn17: f64 = *var_t3__blk86_dn17_slot;
        let mut var_t3__blk86_dn2: f64 = *var_t3__blk86_dn2_slot;
        let mut var_t3__blk86_dn6: f64 = *var_t3__blk86_dn6_slot;
        let mut var_t3__blk86_dn7: f64 = *var_t3__blk86_dn7_slot;
        let mut var_t3__blk86_rv: f64 = *var_t3__blk86_rv_slot;
        let mut var_t4__blk87: f64 = *var_t4__blk87_slot;
        let mut var_t4__blk87_rv: f64 = *var_t4__blk87_rv_slot;
        let mut var_t5__blk88: f64 = *var_t5__blk88_slot;
        let mut var_t5__blk88_dn0: f64 = *var_t5__blk88_dn0_slot;
        let mut var_t5__blk88_dn10: f64 = *var_t5__blk88_dn10_slot;
        let mut var_t5__blk88_dn11: f64 = *var_t5__blk88_dn11_slot;
        let mut var_t5__blk88_dn12: f64 = *var_t5__blk88_dn12_slot;
        let mut var_t5__blk88_dn17: f64 = *var_t5__blk88_dn17_slot;
        let mut var_t5__blk88_dn2: f64 = *var_t5__blk88_dn2_slot;
        let mut var_t5__blk88_dn6: f64 = *var_t5__blk88_dn6_slot;
        let mut var_t5__blk88_dn7: f64 = *var_t5__blk88_dn7_slot;
        let mut var_t5__blk88_rv: f64 = *var_t5__blk88_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vbsz2: f64 = *var_vbsz2_slot;
        let mut var_vbsz2_dn0: f64 = *var_vbsz2_dn0_slot;
        let mut var_vbsz2_dn10: f64 = *var_vbsz2_dn10_slot;
        let mut var_vbsz2_dn11: f64 = *var_vbsz2_dn11_slot;
        let mut var_vbsz2_dn12: f64 = *var_vbsz2_dn12_slot;
        let mut var_vbsz2_dn17: f64 = *var_vbsz2_dn17_slot;
        let mut var_vbsz2_dn2: f64 = *var_vbsz2_dn2_slot;
        let mut var_vbsz2_dn6: f64 = *var_vbsz2_dn6_slot;
        let mut var_vbsz2_dn7: f64 = *var_vbsz2_dn7_slot;
        let mut var_vbsz2_rv: f64 = *var_vbsz2_rv_slot;
        let mut var_vth0: f64 = *var_vth0_slot;
        let mut var_vth0_dn0: f64 = *var_vth0_dn0_slot;
        let mut var_vth0_dn10: f64 = *var_vth0_dn10_slot;
        let mut var_vth0_dn11: f64 = *var_vth0_dn11_slot;
        let mut var_vth0_dn12: f64 = *var_vth0_dn12_slot;
        let mut var_vth0_dn17: f64 = *var_vth0_dn17_slot;
        let mut var_vth0_dn2: f64 = *var_vth0_dn2_slot;
        let mut var_vth0_dn6: f64 = *var_vth0_dn6_slot;
        let mut var_vth0_dn7: f64 = *var_vth0_dn7_slot;
        let mut var_vth0_rv: f64 = *var_vth0_rv_slot;
        let mut var_vthp: f64 = *var_vthp_slot;
        let mut var_vthp_dn0: f64 = *var_vthp_dn0_slot;
        let mut var_vthp_dn10: f64 = *var_vthp_dn10_slot;
        let mut var_vthp_dn11: f64 = *var_vthp_dn11_slot;
        let mut var_vthp_dn12: f64 = *var_vthp_dn12_slot;
        let mut var_vthp_dn17: f64 = *var_vthp_dn17_slot;
        let mut var_vthp_dn2: f64 = *var_vthp_dn2_slot;
        let mut var_vthp_dn6: f64 = *var_vthp_dn6_slot;
        let mut var_vthp_dn7: f64 = *var_vthp_dn7_slot;
        let mut var_vthp_rv: f64 = *var_vthp_rv_slot;
        let mut var_wd0: f64 = *var_wd0_slot;
        let mut var_wd0_dn0: f64 = *var_wd0_dn0_slot;
        let mut var_wd0_dn10: f64 = *var_wd0_dn10_slot;
        let mut var_wd0_dn11: f64 = *var_wd0_dn11_slot;
        let mut var_wd0_dn12: f64 = *var_wd0_dn12_slot;
        let mut var_wd0_dn17: f64 = *var_wd0_dn17_slot;
        let mut var_wd0_dn2: f64 = *var_wd0_dn2_slot;
        let mut var_wd0_dn6: f64 = *var_wd0_dn6_slot;
        let mut var_wd0_dn7: f64 = *var_wd0_dn7_slot;
        let mut var_wd0_rv: f64 = *var_wd0_rv_slot;

        let (assign6190_e4103, assign6190_e4103_d_n0, assign6190_e4103_d_n2, assign6190_e4103_d_n6, assign6190_e4103_d_n7, assign6190_e4103_d_n10, assign6190_e4103_d_n11, assign6190_e4103_d_n12, assign6190_e4103_d_n17,) = {
    if ((var_guard76 != 0.0) && (var_guard77 != 0.0)) {
        let assign6190_e4099: f64 = (4.0 * var_pb20);
        let assign6190_e4101: f64 = (assign6190_e4099 * 0.001);
        (assign6190_e4101, ((4.0 * var_pb20_dn0) * 0.001), ((4.0 * var_pb20_dn2) * 0.001), ((4.0 * var_pb20_dn6) * 0.001), ((4.0 * var_pb20_dn7) * 0.001), ((4.0 * var_pb20_dn10) * 0.001), ((4.0 * var_pb20_dn11) * 0.001), ((4.0 * var_pb20_dn12) * 0.001), ((4.0 * var_pb20_dn17) * 0.001),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6190_e4103;
        var_tmf2_dn0 = assign6190_e4103_d_n0;
        var_tmf2_dn2 = assign6190_e4103_d_n2;
        var_tmf2_dn6 = assign6190_e4103_d_n6;
        var_tmf2_dn7 = assign6190_e4103_d_n7;
        var_tmf2_dn10 = assign6190_e4103_d_n10;
        var_tmf2_dn11 = assign6190_e4103_d_n11;
        var_tmf2_dn12 = assign6190_e4103_d_n12;
        var_tmf2_dn17 = assign6190_e4103_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6200_e4115, assign6200_e4115_d_n0, assign6200_e4115_d_n2, assign6200_e4115_d_n6, assign6200_e4115_d_n7, assign6200_e4115_d_n10, assign6200_e4115_d_n11, assign6200_e4115_d_n12, assign6200_e4115_d_n17,) = {
    if ((var_guard76 != 0.0) && (var_guard77 != 0.0)) {
        let (assign6200_e4113, assign6200_e4113_d_n0, assign6200_e4113_d_n2, assign6200_e4113_d_n6, assign6200_e4113_d_n7, assign6200_e4113_d_n10, assign6200_e4113_d_n11, assign6200_e4113_d_n12, assign6200_e4113_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign6200_e4112: f64 = (-var_tmf2);
                (assign6200_e4112, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign6200_e4113, assign6200_e4113_d_n0, assign6200_e4113_d_n2, assign6200_e4113_d_n6, assign6200_e4113_d_n7, assign6200_e4113_d_n10, assign6200_e4113_d_n11, assign6200_e4113_d_n12, assign6200_e4113_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6200_e4115;
        var_tmf2_dn0 = assign6200_e4115_d_n0;
        var_tmf2_dn2 = assign6200_e4115_d_n2;
        var_tmf2_dn6 = assign6200_e4115_d_n6;
        var_tmf2_dn7 = assign6200_e4115_d_n7;
        var_tmf2_dn10 = assign6200_e4115_d_n10;
        var_tmf2_dn11 = assign6200_e4115_d_n11;
        var_tmf2_dn12 = assign6200_e4115_d_n12;
        var_tmf2_dn17 = assign6200_e4115_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6210_e4126, assign6210_e4126_d_n0, assign6210_e4126_d_n2, assign6210_e4126_d_n6, assign6210_e4126_d_n7, assign6210_e4126_d_n10, assign6210_e4126_d_n11, assign6210_e4126_d_n12, assign6210_e4126_d_n17,) = {
    if ((var_guard76 != 0.0) && (var_guard77 != 0.0)) {
        let assign6210_e4121: f64 = (var_tmf1 * var_tmf1);
        let assign6210_e4123: f64 = (assign6210_e4121 + var_tmf2);
        let assign6210_e4124: f64 = (assign6210_e4123).sqrt();
        (assign6210_e4124, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign6210_e4124)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign6210_e4124)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign6210_e4124)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign6210_e4124)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign6210_e4124)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign6210_e4124)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign6210_e4124)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign6210_e4124)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6210_e4126;
        var_tmf2_dn0 = assign6210_e4126_d_n0;
        var_tmf2_dn2 = assign6210_e4126_d_n2;
        var_tmf2_dn6 = assign6210_e4126_d_n6;
        var_tmf2_dn7 = assign6210_e4126_d_n7;
        var_tmf2_dn10 = assign6210_e4126_d_n10;
        var_tmf2_dn11 = assign6210_e4126_d_n11;
        var_tmf2_dn12 = assign6210_e4126_d_n12;
        var_tmf2_dn17 = assign6210_e4126_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6220_e4138, assign6220_e4138_d_n0, assign6220_e4138_d_n2, assign6220_e4138_d_n6, assign6220_e4138_d_n7, assign6220_e4138_d_n10, assign6220_e4138_d_n11, assign6220_e4138_d_n12, assign6220_e4138_d_n17,) = {
    if ((var_guard76 != 0.0) && (var_guard77 != 0.0)) {
        let assign6220_e4134: f64 = (var_tmf1 + var_tmf2);
        let assign6220_e4135: f64 = (0.5 * assign6220_e4134);
        let assign6220_e4136: f64 = (var_pb20 - assign6220_e4135);
        (assign6220_e4136, (var_pb20_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_pb20_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_pb20_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_pb20_dn7 - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_pb20_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_pb20_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_pb20_dn12 - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), (var_pb20_dn17 - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_vbsz2, var_vbsz2_dn0, var_vbsz2_dn2, var_vbsz2_dn6, var_vbsz2_dn7, var_vbsz2_dn10, var_vbsz2_dn11, var_vbsz2_dn12, var_vbsz2_dn17,)
    }
};
        var_vbsz2 = assign6220_e4138;
        var_vbsz2_dn0 = assign6220_e4138_d_n0;
        var_vbsz2_dn2 = assign6220_e4138_d_n2;
        var_vbsz2_dn6 = assign6220_e4138_d_n6;
        var_vbsz2_dn7 = assign6220_e4138_d_n7;
        var_vbsz2_dn10 = assign6220_e4138_d_n10;
        var_vbsz2_dn11 = assign6220_e4138_d_n11;
        var_vbsz2_dn12 = assign6220_e4138_d_n12;
        var_vbsz2_dn17 = assign6220_e4138_d_n17;
        var_vbsz2_rv = 0.0;

        let (assign6230_e4143, assign6230_e4143_d_n0, assign6230_e4143_d_n2, assign6230_e4143_d_n6, assign6230_e4143_d_n7, assign6230_e4143_d_n10, assign6230_e4143_d_n11, assign6230_e4143_d_n12, assign6230_e4143_d_n17,) = {
    if (var_guard76 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vbsz2, var_vbsz2_dn0, var_vbsz2_dn2, var_vbsz2_dn6, var_vbsz2_dn7, var_vbsz2_dn10, var_vbsz2_dn11, var_vbsz2_dn12, var_vbsz2_dn17,)
    }
};
        var_vbsz2 = assign6230_e4143;
        var_vbsz2_dn0 = assign6230_e4143_d_n0;
        var_vbsz2_dn2 = assign6230_e4143_d_n2;
        var_vbsz2_dn6 = assign6230_e4143_d_n6;
        var_vbsz2_dn7 = assign6230_e4143_d_n7;
        var_vbsz2_dn10 = assign6230_e4143_d_n10;
        var_vbsz2_dn11 = assign6230_e4143_d_n11;
        var_vbsz2_dn12 = assign6230_e4143_d_n12;
        var_vbsz2_dn17 = assign6230_e4143_d_n17;
        var_vbsz2_rv = 0.0;

        let assign6240_e4146: f64 = if var_subversion < 3.0 { 1.0 } else { 0.0 };
        var_guard78 = assign6240_e4146;
        var_guard78_rv = 0.0;

        let (assign6250_e4150, assign6250_e4150_d_n0, assign6250_e4150_d_n2, assign6250_e4150_d_n6, assign6250_e4150_d_n7, assign6250_e4150_d_n10, assign6250_e4150_d_n11, assign6250_e4150_d_n12, assign6250_e4150_d_n17,) = {
    if (var_guard78 != 0.0) {
        (p.p237, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_wd0, var_wd0_dn0, var_wd0_dn2, var_wd0_dn6, var_wd0_dn7, var_wd0_dn10, var_wd0_dn11, var_wd0_dn12, var_wd0_dn17,)
    }
};
        var_wd0 = assign6250_e4150;
        var_wd0_dn0 = assign6250_e4150_d_n0;
        var_wd0_dn2 = assign6250_e4150_d_n2;
        var_wd0_dn6 = assign6250_e4150_d_n6;
        var_wd0_dn7 = assign6250_e4150_d_n7;
        var_wd0_dn10 = assign6250_e4150_d_n10;
        var_wd0_dn11 = assign6250_e4150_d_n11;
        var_wd0_dn12 = assign6250_e4150_d_n12;
        var_wd0_dn17 = assign6250_e4150_d_n17;
        var_wd0_rv = 0.0;

        let (assign6260_e4159, assign6260_e4159_d_n0, assign6260_e4159_d_n2, assign6260_e4159_d_n6, assign6260_e4159_d_n7, assign6260_e4159_d_n10, assign6260_e4159_d_n11, assign6260_e4159_d_n12, assign6260_e4159_d_n17,) = {
    if (var_guard78 == 0.0) {
        let assign6260_e4155: f64 = (2.0 * 1.034943e-10);
        let assign6260_e4157: f64 = (assign6260_e4155 / var_q_nsub);
        (assign6260_e4157, (-((assign6260_e4155 * var_q_nsub_dn0) / (var_q_nsub * var_q_nsub))), (-((assign6260_e4155 * var_q_nsub_dn2) / (var_q_nsub * var_q_nsub))), (-((assign6260_e4155 * var_q_nsub_dn6) / (var_q_nsub * var_q_nsub))), (-((assign6260_e4155 * var_q_nsub_dn7) / (var_q_nsub * var_q_nsub))), (-((assign6260_e4155 * var_q_nsub_dn10) / (var_q_nsub * var_q_nsub))), (-((assign6260_e4155 * var_q_nsub_dn11) / (var_q_nsub * var_q_nsub))), (-((assign6260_e4155 * var_q_nsub_dn12) / (var_q_nsub * var_q_nsub))), (-((assign6260_e4155 * var_q_nsub_dn17) / (var_q_nsub * var_q_nsub))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign6260_e4159;
        var_t1_dn0 = assign6260_e4159_d_n0;
        var_t1_dn2 = assign6260_e4159_d_n2;
        var_t1_dn6 = assign6260_e4159_d_n6;
        var_t1_dn7 = assign6260_e4159_d_n7;
        var_t1_dn10 = assign6260_e4159_d_n10;
        var_t1_dn11 = assign6260_e4159_d_n11;
        var_t1_dn12 = assign6260_e4159_d_n12;
        var_t1_dn17 = assign6260_e4159_d_n17;
        var_t1_rv = 0.0;

        let (assign6270_e4169, assign6270_e4169_d_n0, assign6270_e4169_d_n2, assign6270_e4169_d_n6, assign6270_e4169_d_n7, assign6270_e4169_d_n10, assign6270_e4169_d_n11, assign6270_e4169_d_n12, assign6270_e4169_d_n17,) = {
    if (var_guard78 == 0.0) {
        let assign6270_e4165: f64 = (var_pb20 - var_vbsz2);
        let assign6270_e4166: f64 = (var_t1 * assign6270_e4165);
        let assign6270_e4167: f64 = (assign6270_e4166).sqrt();
        (assign6270_e4167, (((var_t1_dn0 * assign6270_e4165) + (var_t1 * (var_pb20_dn0 - var_vbsz2_dn0))) / (2.0 * assign6270_e4167)), (((var_t1_dn2 * assign6270_e4165) + (var_t1 * (var_pb20_dn2 - var_vbsz2_dn2))) / (2.0 * assign6270_e4167)), (((var_t1_dn6 * assign6270_e4165) + (var_t1 * (var_pb20_dn6 - var_vbsz2_dn6))) / (2.0 * assign6270_e4167)), (((var_t1_dn7 * assign6270_e4165) + (var_t1 * (var_pb20_dn7 - var_vbsz2_dn7))) / (2.0 * assign6270_e4167)), (((var_t1_dn10 * assign6270_e4165) + (var_t1 * (var_pb20_dn10 - var_vbsz2_dn10))) / (2.0 * assign6270_e4167)), (((var_t1_dn11 * assign6270_e4165) + (var_t1 * (var_pb20_dn11 - var_vbsz2_dn11))) / (2.0 * assign6270_e4167)), (((var_t1_dn12 * assign6270_e4165) + (var_t1 * (var_pb20_dn12 - var_vbsz2_dn12))) / (2.0 * assign6270_e4167)), (((var_t1_dn17 * assign6270_e4165) + (var_t1 * (var_pb20_dn17 - var_vbsz2_dn17))) / (2.0 * assign6270_e4167)),)
    } else {
        (var_wd0, var_wd0_dn0, var_wd0_dn2, var_wd0_dn6, var_wd0_dn7, var_wd0_dn10, var_wd0_dn11, var_wd0_dn12, var_wd0_dn17,)
    }
};
        var_wd0 = assign6270_e4169;
        var_wd0_dn0 = assign6270_e4169_d_n0;
        var_wd0_dn2 = assign6270_e4169_d_n2;
        var_wd0_dn6 = assign6270_e4169_d_n6;
        var_wd0_dn7 = assign6270_e4169_d_n7;
        var_wd0_dn10 = assign6270_e4169_d_n10;
        var_wd0_dn11 = assign6270_e4169_d_n11;
        var_wd0_dn12 = assign6270_e4169_d_n12;
        var_wd0_dn17 = assign6270_e4169_d_n17;
        var_wd0_rv = 0.0;

        let (assign6280_e4183, assign6280_e4183_d_n0, assign6280_e4183_d_n2, assign6280_e4183_d_n6, assign6280_e4183_d_n7, assign6280_e4183_d_n10, assign6280_e4183_d_n11, assign6280_e4183_d_n12, assign6280_e4183_d_n17,) = {
    if (var_subversion < 3.0) {
        let assign6280_e4175: f64 = (var_qnsub_esi2 * var_pb20);
        let assign6280_e4176: f64 = (assign6280_e4175).sqrt();
        (assign6280_e4176, (((var_qnsub_esi2_dn0 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn0)) / (2.0 * assign6280_e4176)), (((var_qnsub_esi2_dn2 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn2)) / (2.0 * assign6280_e4176)), (((var_qnsub_esi2_dn6 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn6)) / (2.0 * assign6280_e4176)), (((var_qnsub_esi2_dn7 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn7)) / (2.0 * assign6280_e4176)), (((var_qnsub_esi2_dn10 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn10)) / (2.0 * assign6280_e4176)), (((var_qnsub_esi2_dn11 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn11)) / (2.0 * assign6280_e4176)), (((var_qnsub_esi2_dn12 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn12)) / (2.0 * assign6280_e4176)), (((var_qnsub_esi2_dn17 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn17)) / (2.0 * assign6280_e4176)),)
    } else {
        let assign6280_e4180: f64 = (var_pb20 - var_vbsz2);
        let assign6280_e4181: f64 = (var_qnsub_esi2 * assign6280_e4180);
        let assign6280_e4182: f64 = (assign6280_e4181).sqrt();
        (assign6280_e4182, (((var_qnsub_esi2_dn0 * assign6280_e4180) + (var_qnsub_esi2 * (var_pb20_dn0 - var_vbsz2_dn0))) / (2.0 * assign6280_e4182)), (((var_qnsub_esi2_dn2 * assign6280_e4180) + (var_qnsub_esi2 * (var_pb20_dn2 - var_vbsz2_dn2))) / (2.0 * assign6280_e4182)), (((var_qnsub_esi2_dn6 * assign6280_e4180) + (var_qnsub_esi2 * (var_pb20_dn6 - var_vbsz2_dn6))) / (2.0 * assign6280_e4182)), (((var_qnsub_esi2_dn7 * assign6280_e4180) + (var_qnsub_esi2 * (var_pb20_dn7 - var_vbsz2_dn7))) / (2.0 * assign6280_e4182)), (((var_qnsub_esi2_dn10 * assign6280_e4180) + (var_qnsub_esi2 * (var_pb20_dn10 - var_vbsz2_dn10))) / (2.0 * assign6280_e4182)), (((var_qnsub_esi2_dn11 * assign6280_e4180) + (var_qnsub_esi2 * (var_pb20_dn11 - var_vbsz2_dn11))) / (2.0 * assign6280_e4182)), (((var_qnsub_esi2_dn12 * assign6280_e4180) + (var_qnsub_esi2 * (var_pb20_dn12 - var_vbsz2_dn12))) / (2.0 * assign6280_e4182)), (((var_qnsub_esi2_dn17 * assign6280_e4180) + (var_qnsub_esi2 * (var_pb20_dn17 - var_vbsz2_dn17))) / (2.0 * assign6280_e4182)),)
    }
};
        var_qb0 = assign6280_e4183;
        var_qb0_dn0 = assign6280_e4183_d_n0;
        var_qb0_dn2 = assign6280_e4183_d_n2;
        var_qb0_dn6 = assign6280_e4183_d_n6;
        var_qb0_dn7 = assign6280_e4183_d_n7;
        var_qb0_dn10 = assign6280_e4183_d_n10;
        var_qb0_dn11 = assign6280_e4183_d_n11;
        var_qb0_dn12 = assign6280_e4183_d_n12;
        var_qb0_dn17 = assign6280_e4183_d_n17;
        var_qb0_rv = 0.0;

        let assign6290_e4186: f64 = (var_pb20 + var_vfb);
        let assign6290_e4189: f64 = (var_qb0 * var_c_fox_inv);
        let assign6290_e4190: f64 = (assign6290_e4186 + assign6290_e4189);
        let assign6290_e4192: f64 = (assign6290_e4190 + var_ptovr);
        var_vthp = assign6290_e4192;
        var_vthp_dn0 = ((var_pb20_dn0 + ((var_qb0_dn0 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn0))) + var_ptovr_dn0);
        var_vthp_dn2 = ((var_pb20_dn2 + ((var_qb0_dn2 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn2))) + var_ptovr_dn2);
        var_vthp_dn6 = ((var_pb20_dn6 + ((var_qb0_dn6 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn6))) + var_ptovr_dn6);
        var_vthp_dn7 = ((var_pb20_dn7 + ((var_qb0_dn7 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn7))) + var_ptovr_dn7);
        var_vthp_dn10 = ((var_pb20_dn10 + ((var_qb0_dn10 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn10))) + var_ptovr_dn10);
        var_vthp_dn11 = ((var_pb20_dn11 + ((var_qb0_dn11 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn11))) + var_ptovr_dn11);
        var_vthp_dn12 = ((var_pb20_dn12 + ((var_qb0_dn12 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn12))) + var_ptovr_dn12);
        var_vthp_dn17 = ((var_pb20_dn17 + ((var_qb0_dn17 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn17))) + var_ptovr_dn17);
        var_vthp_rv = 0.0;

        var_pb20b = var_pb20;
        var_pb20b_dn0 = var_pb20_dn0;
        var_pb20b_dn2 = var_pb20_dn2;
        var_pb20b_dn6 = var_pb20_dn6;
        var_pb20b_dn7 = var_pb20_dn7;
        var_pb20b_dn10 = var_pb20_dn10;
        var_pb20b_dn11 = var_pb20_dn11;
        var_pb20b_dn12 = var_pb20_dn12;
        var_pb20b_dn17 = var_pb20_dn17;
        var_pb20b_rv = 0.0;

        var_t0__blk80 = 0.95;
        var_t0__blk80_rv = 0.0;

        let assign6320_e4197: f64 = (var_t0__blk80 * var_pb20b);
        let assign6320_e4199: f64 = (assign6320_e4197 - var_vbsz2);
        let assign6320_e4201: f64 = (assign6320_e4199 - 0.001);
        var_t1__blk79 = assign6320_e4201;
        var_t1__blk79_dn0 = ((var_t0__blk80 * var_pb20b_dn0) - var_vbsz2_dn0);
        var_t1__blk79_dn2 = ((var_t0__blk80 * var_pb20b_dn2) - var_vbsz2_dn2);
        var_t1__blk79_dn6 = ((var_t0__blk80 * var_pb20b_dn6) - var_vbsz2_dn6);
        var_t1__blk79_dn7 = ((var_t0__blk80 * var_pb20b_dn7) - var_vbsz2_dn7);
        var_t1__blk79_dn10 = ((var_t0__blk80 * var_pb20b_dn10) - var_vbsz2_dn10);
        var_t1__blk79_dn11 = ((var_t0__blk80 * var_pb20b_dn11) - var_vbsz2_dn11);
        var_t1__blk79_dn12 = ((var_t0__blk80 * var_pb20b_dn12) - var_vbsz2_dn12);
        var_t1__blk79_dn17 = ((var_t0__blk80 * var_pb20b_dn17) - var_vbsz2_dn17);
        var_t1__blk79_rv = 0.0;

        let assign6330_e4204: f64 = (var_t1__blk79 * var_t1__blk79);
        let assign6330_e4207: f64 = (4.0 * var_t0__blk80);
        let assign6330_e4209: f64 = (assign6330_e4207 * var_pb20b);
        let assign6330_e4211: f64 = (assign6330_e4209 * 0.001);
        let assign6330_e4212: f64 = (assign6330_e4204 + assign6330_e4211);
        let assign6330_e4213: f64 = (assign6330_e4212).sqrt();
        var_t2__blk81 = assign6330_e4213;
        var_t2__blk81_dn0 = ((((var_t1__blk79_dn0 * var_t1__blk79) + (var_t1__blk79 * var_t1__blk79_dn0)) + ((assign6330_e4207 * var_pb20b_dn0) * 0.001)) / (2.0 * assign6330_e4213));
        var_t2__blk81_dn2 = ((((var_t1__blk79_dn2 * var_t1__blk79) + (var_t1__blk79 * var_t1__blk79_dn2)) + ((assign6330_e4207 * var_pb20b_dn2) * 0.001)) / (2.0 * assign6330_e4213));
        var_t2__blk81_dn6 = ((((var_t1__blk79_dn6 * var_t1__blk79) + (var_t1__blk79 * var_t1__blk79_dn6)) + ((assign6330_e4207 * var_pb20b_dn6) * 0.001)) / (2.0 * assign6330_e4213));
        var_t2__blk81_dn7 = ((((var_t1__blk79_dn7 * var_t1__blk79) + (var_t1__blk79 * var_t1__blk79_dn7)) + ((assign6330_e4207 * var_pb20b_dn7) * 0.001)) / (2.0 * assign6330_e4213));
        var_t2__blk81_dn10 = ((((var_t1__blk79_dn10 * var_t1__blk79) + (var_t1__blk79 * var_t1__blk79_dn10)) + ((assign6330_e4207 * var_pb20b_dn10) * 0.001)) / (2.0 * assign6330_e4213));
        var_t2__blk81_dn11 = ((((var_t1__blk79_dn11 * var_t1__blk79) + (var_t1__blk79 * var_t1__blk79_dn11)) + ((assign6330_e4207 * var_pb20b_dn11) * 0.001)) / (2.0 * assign6330_e4213));
        var_t2__blk81_dn12 = ((((var_t1__blk79_dn12 * var_t1__blk79) + (var_t1__blk79 * var_t1__blk79_dn12)) + ((assign6330_e4207 * var_pb20b_dn12) * 0.001)) / (2.0 * assign6330_e4213));
        var_t2__blk81_dn17 = ((((var_t1__blk79_dn17 * var_t1__blk79) + (var_t1__blk79 * var_t1__blk79_dn17)) + ((assign6330_e4207 * var_pb20b_dn17) * 0.001)) / (2.0 * assign6330_e4213));
        var_t2__blk81_rv = 0.0;

        let assign6340_e4216: f64 = (var_t0__blk80 * var_pb20b);
        let assign6340_e4220: f64 = (var_t1__blk79 + var_t2__blk81);
        let assign6340_e4221: f64 = (0.5 * assign6340_e4220);
        let assign6340_e4222: f64 = (assign6340_e4216 - assign6340_e4221);
        var_t3__blk82 = assign6340_e4222;
        var_t3__blk82_dn0 = ((var_t0__blk80 * var_pb20b_dn0) - (0.5 * (var_t1__blk79_dn0 + var_t2__blk81_dn0)));
        var_t3__blk82_dn2 = ((var_t0__blk80 * var_pb20b_dn2) - (0.5 * (var_t1__blk79_dn2 + var_t2__blk81_dn2)));
        var_t3__blk82_dn6 = ((var_t0__blk80 * var_pb20b_dn6) - (0.5 * (var_t1__blk79_dn6 + var_t2__blk81_dn6)));
        var_t3__blk82_dn7 = ((var_t0__blk80 * var_pb20b_dn7) - (0.5 * (var_t1__blk79_dn7 + var_t2__blk81_dn7)));
        var_t3__blk82_dn10 = ((var_t0__blk80 * var_pb20b_dn10) - (0.5 * (var_t1__blk79_dn10 + var_t2__blk81_dn10)));
        var_t3__blk82_dn11 = ((var_t0__blk80 * var_pb20b_dn11) - (0.5 * (var_t1__blk79_dn11 + var_t2__blk81_dn11)));
        var_t3__blk82_dn12 = ((var_t0__blk80 * var_pb20b_dn12) - (0.5 * (var_t1__blk79_dn12 + var_t2__blk81_dn12)));
        var_t3__blk82_dn17 = ((var_t0__blk80 * var_pb20b_dn17) - (0.5 * (var_t1__blk79_dn17 + var_t2__blk81_dn17)));
        var_t3__blk82_rv = 0.0;

        let assign6350_e4225: f64 = (var_pb20b - var_t3__blk82);
        var_pbsum = assign6350_e4225;
        var_pbsum_dn0 = (var_pb20b_dn0 - var_t3__blk82_dn0);
        var_pbsum_dn2 = (var_pb20b_dn2 - var_t3__blk82_dn2);
        var_pbsum_dn6 = (var_pb20b_dn6 - var_t3__blk82_dn6);
        var_pbsum_dn7 = (var_pb20b_dn7 - var_t3__blk82_dn7);
        var_pbsum_dn10 = (var_pb20b_dn10 - var_t3__blk82_dn10);
        var_pbsum_dn11 = (var_pb20b_dn11 - var_t3__blk82_dn11);
        var_pbsum_dn12 = (var_pb20b_dn12 - var_t3__blk82_dn12);
        var_pbsum_dn17 = (var_pb20b_dn17 - var_t3__blk82_dn17);
        var_pbsum_rv = 0.0;

        let assign6360_e4227: f64 = (var_pbsum).sqrt();
        var_sqrt_pbsum = assign6360_e4227;
        var_sqrt_pbsum_dn0 = (var_pbsum_dn0 / (2.0 * assign6360_e4227));
        var_sqrt_pbsum_dn2 = (var_pbsum_dn2 / (2.0 * assign6360_e4227));
        var_sqrt_pbsum_dn6 = (var_pbsum_dn6 / (2.0 * assign6360_e4227));
        var_sqrt_pbsum_dn7 = (var_pbsum_dn7 / (2.0 * assign6360_e4227));
        var_sqrt_pbsum_dn10 = (var_pbsum_dn10 / (2.0 * assign6360_e4227));
        var_sqrt_pbsum_dn11 = (var_pbsum_dn11 / (2.0 * assign6360_e4227));
        var_sqrt_pbsum_dn12 = (var_pbsum_dn12 / (2.0 * assign6360_e4227));
        var_sqrt_pbsum_dn17 = (var_pbsum_dn17 / (2.0 * assign6360_e4227));
        var_sqrt_pbsum_rv = 0.0;

        let assign6370_e4230: f64 = if p.p72 != 0.0 { 1.0 } else { 0.0 };
        var_guard90 = assign6370_e4230;
        var_guard90_rv = 0.0;

        let (assign6380_e4240, assign6380_e4240_d_n0, assign6380_e4240_d_n2, assign6380_e4240_d_n6, assign6380_e4240_d_n7, assign6380_e4240_d_n10, assign6380_e4240_d_n11, assign6380_e4240_d_n12, assign6380_e4240_d_n17,) = {
    if (var_guard90 != 0.0) {
        let assign6380_e4234: f64 = (2.0 * 1.6021918e-19);
        let assign6380_e4236: f64 = (assign6380_e4234 * var_uc_nsubs);
        let assign6380_e4238: f64 = (assign6380_e4236 * 1.034943e-10);
        (assign6380_e4238, ((assign6380_e4234 * var_uc_nsubs_dn0) * 1.034943e-10), ((assign6380_e4234 * var_uc_nsubs_dn2) * 1.034943e-10), ((assign6380_e4234 * var_uc_nsubs_dn6) * 1.034943e-10), ((assign6380_e4234 * var_uc_nsubs_dn7) * 1.034943e-10), ((assign6380_e4234 * var_uc_nsubs_dn10) * 1.034943e-10), ((assign6380_e4234 * var_uc_nsubs_dn11) * 1.034943e-10), ((assign6380_e4234 * var_uc_nsubs_dn12) * 1.034943e-10), ((assign6380_e4234 * var_uc_nsubs_dn17) * 1.034943e-10),)
    } else {
        (var_t1__blk84, var_t1__blk84_dn0, var_t1__blk84_dn2, var_t1__blk84_dn6, var_t1__blk84_dn7, var_t1__blk84_dn10, var_t1__blk84_dn11, var_t1__blk84_dn12, var_t1__blk84_dn17,)
    }
};
        var_t1__blk84 = assign6380_e4240;
        var_t1__blk84_dn0 = assign6380_e4240_d_n0;
        var_t1__blk84_dn2 = assign6380_e4240_d_n2;
        var_t1__blk84_dn6 = assign6380_e4240_d_n6;
        var_t1__blk84_dn7 = assign6380_e4240_d_n7;
        var_t1__blk84_dn10 = assign6380_e4240_d_n10;
        var_t1__blk84_dn11 = assign6380_e4240_d_n11;
        var_t1__blk84_dn12 = assign6380_e4240_d_n12;
        var_t1__blk84_dn17 = assign6380_e4240_d_n17;
        var_t1__blk84_rv = 0.0;

        let (assign6390_e4257, assign6390_e4257_d_n0, assign6390_e4257_d_n2, assign6390_e4257_d_n6, assign6390_e4257_d_n7, assign6390_e4257_d_n10, assign6390_e4257_d_n11, assign6390_e4257_d_n12, assign6390_e4257_d_n17,) = {
    if (var_guard90 != 0.0) {
        let (assign6390_e4255, assign6390_e4255_d_n0, assign6390_e4255_d_n2, assign6390_e4255_d_n6, assign6390_e4255_d_n7, assign6390_e4255_d_n10, assign6390_e4255_d_n11, assign6390_e4255_d_n12, assign6390_e4255_d_n17,) = {
            if (var_subversion < 3.0) {
                let assign6390_e4247: f64 = (var_t1__blk84 * var_pb2c);
                let assign6390_e4248: f64 = (assign6390_e4247).sqrt();
                (assign6390_e4248, (((var_t1__blk84_dn0 * var_pb2c) + (var_t1__blk84 * var_pb2c_dn0)) / (2.0 * assign6390_e4248)), (((var_t1__blk84_dn2 * var_pb2c) + (var_t1__blk84 * var_pb2c_dn2)) / (2.0 * assign6390_e4248)), (((var_t1__blk84_dn6 * var_pb2c) + (var_t1__blk84 * var_pb2c_dn6)) / (2.0 * assign6390_e4248)), (((var_t1__blk84_dn7 * var_pb2c) + (var_t1__blk84 * var_pb2c_dn7)) / (2.0 * assign6390_e4248)), (((var_t1__blk84_dn10 * var_pb2c) + (var_t1__blk84 * var_pb2c_dn10)) / (2.0 * assign6390_e4248)), (((var_t1__blk84_dn11 * var_pb2c) + (var_t1__blk84 * var_pb2c_dn11)) / (2.0 * assign6390_e4248)), (((var_t1__blk84_dn12 * var_pb2c) + (var_t1__blk84 * var_pb2c_dn12)) / (2.0 * assign6390_e4248)), (((var_t1__blk84_dn17 * var_pb2c) + (var_t1__blk84 * var_pb2c_dn17)) / (2.0 * assign6390_e4248)),)
            } else {
                let assign6390_e4252: f64 = (var_pb2c - var_vbsz2);
                let assign6390_e4253: f64 = (var_t1__blk84 * assign6390_e4252);
                let assign6390_e4254: f64 = (assign6390_e4253).sqrt();
                (assign6390_e4254, (((var_t1__blk84_dn0 * assign6390_e4252) + (var_t1__blk84 * (var_pb2c_dn0 - var_vbsz2_dn0))) / (2.0 * assign6390_e4254)), (((var_t1__blk84_dn2 * assign6390_e4252) + (var_t1__blk84 * (var_pb2c_dn2 - var_vbsz2_dn2))) / (2.0 * assign6390_e4254)), (((var_t1__blk84_dn6 * assign6390_e4252) + (var_t1__blk84 * (var_pb2c_dn6 - var_vbsz2_dn6))) / (2.0 * assign6390_e4254)), (((var_t1__blk84_dn7 * assign6390_e4252) + (var_t1__blk84 * (var_pb2c_dn7 - var_vbsz2_dn7))) / (2.0 * assign6390_e4254)), (((var_t1__blk84_dn10 * assign6390_e4252) + (var_t1__blk84 * (var_pb2c_dn10 - var_vbsz2_dn10))) / (2.0 * assign6390_e4254)), (((var_t1__blk84_dn11 * assign6390_e4252) + (var_t1__blk84 * (var_pb2c_dn11 - var_vbsz2_dn11))) / (2.0 * assign6390_e4254)), (((var_t1__blk84_dn12 * assign6390_e4252) + (var_t1__blk84 * (var_pb2c_dn12 - var_vbsz2_dn12))) / (2.0 * assign6390_e4254)), (((var_t1__blk84_dn17 * assign6390_e4252) + (var_t1__blk84 * (var_pb2c_dn17 - var_vbsz2_dn17))) / (2.0 * assign6390_e4254)),)
            }
        };
        (assign6390_e4255, assign6390_e4255_d_n0, assign6390_e4255_d_n2, assign6390_e4255_d_n6, assign6390_e4255_d_n7, assign6390_e4255_d_n10, assign6390_e4255_d_n11, assign6390_e4255_d_n12, assign6390_e4255_d_n17,)
    } else {
        (var_t2__blk85, var_t2__blk85_dn0, var_t2__blk85_dn2, var_t2__blk85_dn6, var_t2__blk85_dn7, var_t2__blk85_dn10, var_t2__blk85_dn11, var_t2__blk85_dn12, var_t2__blk85_dn17,)
    }
};
        var_t2__blk85 = assign6390_e4257;
        var_t2__blk85_dn0 = assign6390_e4257_d_n0;
        var_t2__blk85_dn2 = assign6390_e4257_d_n2;
        var_t2__blk85_dn6 = assign6390_e4257_d_n6;
        var_t2__blk85_dn7 = assign6390_e4257_d_n7;
        var_t2__blk85_dn10 = assign6390_e4257_d_n10;
        var_t2__blk85_dn11 = assign6390_e4257_d_n11;
        var_t2__blk85_dn12 = assign6390_e4257_d_n12;
        var_t2__blk85_dn17 = assign6390_e4257_d_n17;
        var_t2__blk85_rv = 0.0;

        let (assign6400_e4267, assign6400_e4267_d_n0, assign6400_e4267_d_n2, assign6400_e4267_d_n6, assign6400_e4267_d_n7, assign6400_e4267_d_n10, assign6400_e4267_d_n11, assign6400_e4267_d_n12, assign6400_e4267_d_n17,) = {
    if (var_guard90 != 0.0) {
        let assign6400_e4261: f64 = (var_pb2c + var_vfb);
        let assign6400_e4264: f64 = (var_t2__blk85 * var_c_fox_inv);
        let assign6400_e4265: f64 = (assign6400_e4261 + assign6400_e4264);
        (assign6400_e4265, (var_pb2c_dn0 + ((var_t2__blk85_dn0 * var_c_fox_inv) + (var_t2__blk85 * var_c_fox_inv_dn0))), (var_pb2c_dn2 + ((var_t2__blk85_dn2 * var_c_fox_inv) + (var_t2__blk85 * var_c_fox_inv_dn2))), (var_pb2c_dn6 + ((var_t2__blk85_dn6 * var_c_fox_inv) + (var_t2__blk85 * var_c_fox_inv_dn6))), (var_pb2c_dn7 + ((var_t2__blk85_dn7 * var_c_fox_inv) + (var_t2__blk85 * var_c_fox_inv_dn7))), (var_pb2c_dn10 + ((var_t2__blk85_dn10 * var_c_fox_inv) + (var_t2__blk85 * var_c_fox_inv_dn10))), (var_pb2c_dn11 + ((var_t2__blk85_dn11 * var_c_fox_inv) + (var_t2__blk85 * var_c_fox_inv_dn11))), (var_pb2c_dn12 + ((var_t2__blk85_dn12 * var_c_fox_inv) + (var_t2__blk85 * var_c_fox_inv_dn12))), (var_pb2c_dn17 + ((var_t2__blk85_dn17 * var_c_fox_inv) + (var_t2__blk85 * var_c_fox_inv_dn17))),)
    } else {
        (var_vth0, var_vth0_dn0, var_vth0_dn2, var_vth0_dn6, var_vth0_dn7, var_vth0_dn10, var_vth0_dn11, var_vth0_dn12, var_vth0_dn17,)
    }
};
        var_vth0 = assign6400_e4267;
        var_vth0_dn0 = assign6400_e4267_d_n0;
        var_vth0_dn2 = assign6400_e4267_d_n2;
        var_vth0_dn6 = assign6400_e4267_d_n6;
        var_vth0_dn7 = assign6400_e4267_d_n7;
        var_vth0_dn10 = assign6400_e4267_d_n10;
        var_vth0_dn11 = assign6400_e4267_d_n11;
        var_vth0_dn12 = assign6400_e4267_d_n12;
        var_vth0_dn17 = assign6400_e4267_d_n17;
        var_vth0_rv = 0.0;

        let (assign6410_e4273, assign6410_e4273_d_n0, assign6410_e4273_d_n2, assign6410_e4273_d_n6, assign6410_e4273_d_n7, assign6410_e4273_d_n10, assign6410_e4273_d_n11, assign6410_e4273_d_n12, assign6410_e4273_d_n17,) = {
    if (var_guard90 != 0.0) {
        let assign6410_e4271: f64 = (1.034943e-10 * var_c_fox_inv);
        (assign6410_e4271, (1.034943e-10 * var_c_fox_inv_dn0), (1.034943e-10 * var_c_fox_inv_dn2), (1.034943e-10 * var_c_fox_inv_dn6), (1.034943e-10 * var_c_fox_inv_dn7), (1.034943e-10 * var_c_fox_inv_dn10), (1.034943e-10 * var_c_fox_inv_dn11), (1.034943e-10 * var_c_fox_inv_dn12), (1.034943e-10 * var_c_fox_inv_dn17),)
    } else {
        (var_t1__blk84, var_t1__blk84_dn0, var_t1__blk84_dn2, var_t1__blk84_dn6, var_t1__blk84_dn7, var_t1__blk84_dn10, var_t1__blk84_dn11, var_t1__blk84_dn12, var_t1__blk84_dn17,)
    }
};
        var_t1__blk84 = assign6410_e4273;
        var_t1__blk84_dn0 = assign6410_e4273_d_n0;
        var_t1__blk84_dn2 = assign6410_e4273_d_n2;
        var_t1__blk84_dn6 = assign6410_e4273_d_n6;
        var_t1__blk84_dn7 = assign6410_e4273_d_n7;
        var_t1__blk84_dn10 = assign6410_e4273_d_n10;
        var_t1__blk84_dn11 = assign6410_e4273_d_n11;
        var_t1__blk84_dn12 = assign6410_e4273_d_n12;
        var_t1__blk84_dn17 = assign6410_e4273_d_n17;
        var_t1__blk84_rv = 0.0;

        let (assign6420_e4281,) = {
    if (var_guard90 != 0.0) {
        let assign6420_e4278: f64 = (p.p72 * p.p72);
        let assign6420_e4279: f64 = (1.0 / assign6420_e4278);
        (assign6420_e4279,)
    } else {
        (var_t4__blk87,)
    }
};
        var_t4__blk87 = assign6420_e4281;
        var_t4__blk87_rv = 0.0;

        let (assign6430_e4289, assign6430_e4289_d_n0, assign6430_e4289_d_n2, assign6430_e4289_d_n6, assign6430_e4289_d_n7, assign6430_e4289_d_n10, assign6430_e4289_d_n11, assign6430_e4289_d_n12, assign6430_e4289_d_n17,) = {
    if (var_guard90 != 0.0) {
        let assign6430_e4285: f64 = (2.0 * var_wd0);
        let assign6430_e4287: f64 = (assign6430_e4285 * var_t4__blk87);
        (assign6430_e4287, ((2.0 * var_wd0_dn0) * var_t4__blk87), ((2.0 * var_wd0_dn2) * var_t4__blk87), ((2.0 * var_wd0_dn6) * var_t4__blk87), ((2.0 * var_wd0_dn7) * var_t4__blk87), ((2.0 * var_wd0_dn10) * var_t4__blk87), ((2.0 * var_wd0_dn11) * var_t4__blk87), ((2.0 * var_wd0_dn12) * var_t4__blk87), ((2.0 * var_wd0_dn17) * var_t4__blk87),)
    } else {
        (var_t3__blk86, var_t3__blk86_dn0, var_t3__blk86_dn2, var_t3__blk86_dn6, var_t3__blk86_dn7, var_t3__blk86_dn10, var_t3__blk86_dn11, var_t3__blk86_dn12, var_t3__blk86_dn17,)
    }
};
        var_t3__blk86 = assign6430_e4289;
        var_t3__blk86_dn0 = assign6430_e4289_d_n0;
        var_t3__blk86_dn2 = assign6430_e4289_d_n2;
        var_t3__blk86_dn6 = assign6430_e4289_d_n6;
        var_t3__blk86_dn7 = assign6430_e4289_d_n7;
        var_t3__blk86_dn10 = assign6430_e4289_d_n10;
        var_t3__blk86_dn11 = assign6430_e4289_d_n11;
        var_t3__blk86_dn12 = assign6430_e4289_d_n12;
        var_t3__blk86_dn17 = assign6430_e4289_d_n17;
        var_t3__blk86_rv = 0.0;

        let (assign6440_e4299, assign6440_e4299_d_n0, assign6440_e4299_d_n2, assign6440_e4299_d_n6, assign6440_e4299_d_n7, assign6440_e4299_d_n10, assign6440_e4299_d_n11, assign6440_e4299_d_n12, assign6440_e4299_d_n17,) = {
    if (var_guard90 != 0.0) {
        let assign6440_e4293: f64 = (var_t1__blk84 * var_t3__blk86);
        let assign6440_e4296: f64 = (p.p69 - var_pb20b);
        let assign6440_e4297: f64 = (assign6440_e4293 * assign6440_e4296);
        (assign6440_e4297, ((((var_t1__blk84_dn0 * var_t3__blk86) + (var_t1__blk84 * var_t3__blk86_dn0)) * assign6440_e4296) + (assign6440_e4293 * (-var_pb20b_dn0))), ((((var_t1__blk84_dn2 * var_t3__blk86) + (var_t1__blk84 * var_t3__blk86_dn2)) * assign6440_e4296) + (assign6440_e4293 * (-var_pb20b_dn2))), ((((var_t1__blk84_dn6 * var_t3__blk86) + (var_t1__blk84 * var_t3__blk86_dn6)) * assign6440_e4296) + (assign6440_e4293 * (-var_pb20b_dn6))), ((((var_t1__blk84_dn7 * var_t3__blk86) + (var_t1__blk84 * var_t3__blk86_dn7)) * assign6440_e4296) + (assign6440_e4293 * (-var_pb20b_dn7))), ((((var_t1__blk84_dn10 * var_t3__blk86) + (var_t1__blk84 * var_t3__blk86_dn10)) * assign6440_e4296) + (assign6440_e4293 * (-var_pb20b_dn10))), ((((var_t1__blk84_dn11 * var_t3__blk86) + (var_t1__blk84 * var_t3__blk86_dn11)) * assign6440_e4296) + (assign6440_e4293 * (-var_pb20b_dn11))), ((((var_t1__blk84_dn12 * var_t3__blk86) + (var_t1__blk84 * var_t3__blk86_dn12)) * assign6440_e4296) + (assign6440_e4293 * (-var_pb20b_dn12))), ((((var_t1__blk84_dn17 * var_t3__blk86) + (var_t1__blk84 * var_t3__blk86_dn17)) * assign6440_e4296) + (assign6440_e4293 * (-var_pb20b_dn17))),)
    } else {
        (var_t5__blk88, var_t5__blk88_dn0, var_t5__blk88_dn2, var_t5__blk88_dn6, var_t5__blk88_dn7, var_t5__blk88_dn10, var_t5__blk88_dn11, var_t5__blk88_dn12, var_t5__blk88_dn17,)
    }
};
        var_t5__blk88 = assign6440_e4299;
        var_t5__blk88_dn0 = assign6440_e4299_d_n0;
        var_t5__blk88_dn2 = assign6440_e4299_d_n2;
        var_t5__blk88_dn6 = assign6440_e4299_d_n6;
        var_t5__blk88_dn7 = assign6440_e4299_d_n7;
        var_t5__blk88_dn10 = assign6440_e4299_d_n10;
        var_t5__blk88_dn11 = assign6440_e4299_d_n11;
        var_t5__blk88_dn12 = assign6440_e4299_d_n12;
        var_t5__blk88_dn17 = assign6440_e4299_d_n17;
        var_t5__blk88_rv = 0.0;

        let (assign6450_e4303, assign6450_e4303_d_n0, assign6450_e4303_d_n2, assign6450_e4303_d_n6, assign6450_e4303_d_n7, assign6450_e4303_d_n10, assign6450_e4303_d_n11, assign6450_e4303_d_n12, assign6450_e4303_d_n17,) = {
    if (var_guard90 != 0.0) {
        (var_t5__blk88, var_t5__blk88_dn0, var_t5__blk88_dn2, var_t5__blk88_dn6, var_t5__blk88_dn7, var_t5__blk88_dn10, var_t5__blk88_dn11, var_t5__blk88_dn12, var_t5__blk88_dn17,)
    } else {
        (var_dvth0__blk89, var_dvth0__blk89_dn0, var_dvth0__blk89_dn2, var_dvth0__blk89_dn6, var_dvth0__blk89_dn7, var_dvth0__blk89_dn10, var_dvth0__blk89_dn11, var_dvth0__blk89_dn12, var_dvth0__blk89_dn17,)
    }
};
        var_dvth0__blk89 = assign6450_e4303;
        var_dvth0__blk89_dn0 = assign6450_e4303_d_n0;
        var_dvth0__blk89_dn2 = assign6450_e4303_d_n2;
        var_dvth0__blk89_dn6 = assign6450_e4303_d_n6;
        var_dvth0__blk89_dn7 = assign6450_e4303_d_n7;
        var_dvth0__blk89_dn10 = assign6450_e4303_d_n10;
        var_dvth0__blk89_dn11 = assign6450_e4303_d_n11;
        var_dvth0__blk89_dn12 = assign6450_e4303_d_n12;
        var_dvth0__blk89_dn17 = assign6450_e4303_d_n17;
        var_dvth0__blk89_rv = 0.0;

        let (assign6460_e4309, assign6460_e4309_d_n0, assign6460_e4309_d_n2, assign6460_e4309_d_n6, assign6460_e4309_d_n7, assign6460_e4309_d_n10, assign6460_e4309_d_n11, assign6460_e4309_d_n12, assign6460_e4309_d_n17,) = {
    if (var_guard90 != 0.0) {
        let assign6460_e4307: f64 = (var_vthp - var_vth0);
        (assign6460_e4307, (var_vthp_dn0 - var_vth0_dn0), (var_vthp_dn2 - var_vth0_dn2), (var_vthp_dn6 - var_vth0_dn6), (var_vthp_dn7 - var_vth0_dn7), (var_vthp_dn10 - var_vth0_dn10), (var_vthp_dn11 - var_vth0_dn11), (var_vthp_dn12 - var_vth0_dn12), (var_vthp_dn17 - var_vth0_dn17),)
    } else {
        (var_t1__blk84, var_t1__blk84_dn0, var_t1__blk84_dn2, var_t1__blk84_dn6, var_t1__blk84_dn7, var_t1__blk84_dn10, var_t1__blk84_dn11, var_t1__blk84_dn12, var_t1__blk84_dn17,)
    }
};
        var_t1__blk84 = assign6460_e4309;
        var_t1__blk84_dn0 = assign6460_e4309_d_n0;
        var_t1__blk84_dn2 = assign6460_e4309_d_n2;
        var_t1__blk84_dn6 = assign6460_e4309_d_n6;
        var_t1__blk84_dn7 = assign6460_e4309_d_n7;
        var_t1__blk84_dn10 = assign6460_e4309_d_n10;
        var_t1__blk84_dn11 = assign6460_e4309_d_n11;
        var_t1__blk84_dn12 = assign6460_e4309_d_n12;
        var_t1__blk84_dn17 = assign6460_e4309_d_n17;
        var_t1__blk84_rv = 0.0;

        let (assign6470_e4315,) = {
    if (var_guard90 != 0.0) {
        let assign6470_e4313: f64 = (var_uc_scp3 / p.p72);
        (assign6470_e4313,)
    } else {
        (var_t0__blk83,)
    }
};
        var_t0__blk83 = assign6470_e4315;
        var_t0__blk83_rv = 0.0;

        let (assign6480_e4323, assign6480_e4323_d_n0, assign6480_e4323_d_n2, assign6480_e4323_d_n6, assign6480_e4323_d_n7, assign6480_e4323_d_n10, assign6480_e4323_d_n11, assign6480_e4323_d_n12, assign6480_e4323_d_n17,) = {
    if (var_guard90 != 0.0) {
        let assign6480_e4320: f64 = (var_t0__blk83 * var_pbsum);
        let assign6480_e4321: f64 = (p.p80 + assign6480_e4320);
        (assign6480_e4321, (var_t0__blk83 * var_pbsum_dn0), (var_t0__blk83 * var_pbsum_dn2), (var_t0__blk83 * var_pbsum_dn6), (var_t0__blk83 * var_pbsum_dn7), (var_t0__blk83 * var_pbsum_dn10), (var_t0__blk83 * var_pbsum_dn11), (var_t0__blk83 * var_pbsum_dn12), (var_t0__blk83 * var_pbsum_dn17),)
    } else {
        (var_t2__blk85, var_t2__blk85_dn0, var_t2__blk85_dn2, var_t2__blk85_dn6, var_t2__blk85_dn7, var_t2__blk85_dn10, var_t2__blk85_dn11, var_t2__blk85_dn12, var_t2__blk85_dn17,)
    }
};
        var_t2__blk85 = assign6480_e4323;
        var_t2__blk85_dn0 = assign6480_e4323_d_n0;
        var_t2__blk85_dn2 = assign6480_e4323_d_n2;
        var_t2__blk85_dn6 = assign6480_e4323_d_n6;
        var_t2__blk85_dn7 = assign6480_e4323_d_n7;
        var_t2__blk85_dn10 = assign6480_e4323_d_n10;
        var_t2__blk85_dn11 = assign6480_e4323_d_n11;
        var_t2__blk85_dn12 = assign6480_e4323_d_n12;
        var_t2__blk85_dn17 = assign6480_e4323_d_n17;
        var_t2__blk85_rv = 0.0;

        *var_dvth0__blk89_slot = var_dvth0__blk89;
        *var_dvth0__blk89_dn0_slot = var_dvth0__blk89_dn0;
        *var_dvth0__blk89_dn10_slot = var_dvth0__blk89_dn10;
        *var_dvth0__blk89_dn11_slot = var_dvth0__blk89_dn11;
        *var_dvth0__blk89_dn12_slot = var_dvth0__blk89_dn12;
        *var_dvth0__blk89_dn17_slot = var_dvth0__blk89_dn17;
        *var_dvth0__blk89_dn2_slot = var_dvth0__blk89_dn2;
        *var_dvth0__blk89_dn6_slot = var_dvth0__blk89_dn6;
        *var_dvth0__blk89_dn7_slot = var_dvth0__blk89_dn7;
        *var_dvth0__blk89_rv_slot = var_dvth0__blk89_rv;
        *var_guard78_slot = var_guard78;
        *var_guard78_rv_slot = var_guard78_rv;
        *var_guard90_slot = var_guard90;
        *var_guard90_rv_slot = var_guard90_rv;
        *var_pb20b_slot = var_pb20b;
        *var_pb20b_dn0_slot = var_pb20b_dn0;
        *var_pb20b_dn10_slot = var_pb20b_dn10;
        *var_pb20b_dn11_slot = var_pb20b_dn11;
        *var_pb20b_dn12_slot = var_pb20b_dn12;
        *var_pb20b_dn17_slot = var_pb20b_dn17;
        *var_pb20b_dn2_slot = var_pb20b_dn2;
        *var_pb20b_dn6_slot = var_pb20b_dn6;
        *var_pb20b_dn7_slot = var_pb20b_dn7;
        *var_pb20b_rv_slot = var_pb20b_rv;
        *var_pbsum_slot = var_pbsum;
        *var_pbsum_dn0_slot = var_pbsum_dn0;
        *var_pbsum_dn10_slot = var_pbsum_dn10;
        *var_pbsum_dn11_slot = var_pbsum_dn11;
        *var_pbsum_dn12_slot = var_pbsum_dn12;
        *var_pbsum_dn17_slot = var_pbsum_dn17;
        *var_pbsum_dn2_slot = var_pbsum_dn2;
        *var_pbsum_dn6_slot = var_pbsum_dn6;
        *var_pbsum_dn7_slot = var_pbsum_dn7;
        *var_pbsum_rv_slot = var_pbsum_rv;
        *var_qb0_slot = var_qb0;
        *var_qb0_dn0_slot = var_qb0_dn0;
        *var_qb0_dn10_slot = var_qb0_dn10;
        *var_qb0_dn11_slot = var_qb0_dn11;
        *var_qb0_dn12_slot = var_qb0_dn12;
        *var_qb0_dn17_slot = var_qb0_dn17;
        *var_qb0_dn2_slot = var_qb0_dn2;
        *var_qb0_dn6_slot = var_qb0_dn6;
        *var_qb0_dn7_slot = var_qb0_dn7;
        *var_qb0_rv_slot = var_qb0_rv;
        *var_sqrt_pbsum_slot = var_sqrt_pbsum;
        *var_sqrt_pbsum_dn0_slot = var_sqrt_pbsum_dn0;
        *var_sqrt_pbsum_dn10_slot = var_sqrt_pbsum_dn10;
        *var_sqrt_pbsum_dn11_slot = var_sqrt_pbsum_dn11;
        *var_sqrt_pbsum_dn12_slot = var_sqrt_pbsum_dn12;
        *var_sqrt_pbsum_dn17_slot = var_sqrt_pbsum_dn17;
        *var_sqrt_pbsum_dn2_slot = var_sqrt_pbsum_dn2;
        *var_sqrt_pbsum_dn6_slot = var_sqrt_pbsum_dn6;
        *var_sqrt_pbsum_dn7_slot = var_sqrt_pbsum_dn7;
        *var_sqrt_pbsum_rv_slot = var_sqrt_pbsum_rv;
        *var_t0__blk80_slot = var_t0__blk80;
        *var_t0__blk80_rv_slot = var_t0__blk80_rv;
        *var_t0__blk83_slot = var_t0__blk83;
        *var_t0__blk83_rv_slot = var_t0__blk83_rv;
        *var_t1_slot = var_t1;
        *var_t1__blk79_slot = var_t1__blk79;
        *var_t1__blk79_dn0_slot = var_t1__blk79_dn0;
        *var_t1__blk79_dn10_slot = var_t1__blk79_dn10;
        *var_t1__blk79_dn11_slot = var_t1__blk79_dn11;
        *var_t1__blk79_dn12_slot = var_t1__blk79_dn12;
        *var_t1__blk79_dn17_slot = var_t1__blk79_dn17;
        *var_t1__blk79_dn2_slot = var_t1__blk79_dn2;
        *var_t1__blk79_dn6_slot = var_t1__blk79_dn6;
        *var_t1__blk79_dn7_slot = var_t1__blk79_dn7;
        *var_t1__blk79_rv_slot = var_t1__blk79_rv;
        *var_t1__blk84_slot = var_t1__blk84;
        *var_t1__blk84_dn0_slot = var_t1__blk84_dn0;
        *var_t1__blk84_dn10_slot = var_t1__blk84_dn10;
        *var_t1__blk84_dn11_slot = var_t1__blk84_dn11;
        *var_t1__blk84_dn12_slot = var_t1__blk84_dn12;
        *var_t1__blk84_dn17_slot = var_t1__blk84_dn17;
        *var_t1__blk84_dn2_slot = var_t1__blk84_dn2;
        *var_t1__blk84_dn6_slot = var_t1__blk84_dn6;
        *var_t1__blk84_dn7_slot = var_t1__blk84_dn7;
        *var_t1__blk84_rv_slot = var_t1__blk84_rv;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2__blk81_slot = var_t2__blk81;
        *var_t2__blk81_dn0_slot = var_t2__blk81_dn0;
        *var_t2__blk81_dn10_slot = var_t2__blk81_dn10;
        *var_t2__blk81_dn11_slot = var_t2__blk81_dn11;
        *var_t2__blk81_dn12_slot = var_t2__blk81_dn12;
        *var_t2__blk81_dn17_slot = var_t2__blk81_dn17;
        *var_t2__blk81_dn2_slot = var_t2__blk81_dn2;
        *var_t2__blk81_dn6_slot = var_t2__blk81_dn6;
        *var_t2__blk81_dn7_slot = var_t2__blk81_dn7;
        *var_t2__blk81_rv_slot = var_t2__blk81_rv;
        *var_t2__blk85_slot = var_t2__blk85;
        *var_t2__blk85_dn0_slot = var_t2__blk85_dn0;
        *var_t2__blk85_dn10_slot = var_t2__blk85_dn10;
        *var_t2__blk85_dn11_slot = var_t2__blk85_dn11;
        *var_t2__blk85_dn12_slot = var_t2__blk85_dn12;
        *var_t2__blk85_dn17_slot = var_t2__blk85_dn17;
        *var_t2__blk85_dn2_slot = var_t2__blk85_dn2;
        *var_t2__blk85_dn6_slot = var_t2__blk85_dn6;
        *var_t2__blk85_dn7_slot = var_t2__blk85_dn7;
        *var_t2__blk85_rv_slot = var_t2__blk85_rv;
        *var_t3__blk82_slot = var_t3__blk82;
        *var_t3__blk82_dn0_slot = var_t3__blk82_dn0;
        *var_t3__blk82_dn10_slot = var_t3__blk82_dn10;
        *var_t3__blk82_dn11_slot = var_t3__blk82_dn11;
        *var_t3__blk82_dn12_slot = var_t3__blk82_dn12;
        *var_t3__blk82_dn17_slot = var_t3__blk82_dn17;
        *var_t3__blk82_dn2_slot = var_t3__blk82_dn2;
        *var_t3__blk82_dn6_slot = var_t3__blk82_dn6;
        *var_t3__blk82_dn7_slot = var_t3__blk82_dn7;
        *var_t3__blk82_rv_slot = var_t3__blk82_rv;
        *var_t3__blk86_slot = var_t3__blk86;
        *var_t3__blk86_dn0_slot = var_t3__blk86_dn0;
        *var_t3__blk86_dn10_slot = var_t3__blk86_dn10;
        *var_t3__blk86_dn11_slot = var_t3__blk86_dn11;
        *var_t3__blk86_dn12_slot = var_t3__blk86_dn12;
        *var_t3__blk86_dn17_slot = var_t3__blk86_dn17;
        *var_t3__blk86_dn2_slot = var_t3__blk86_dn2;
        *var_t3__blk86_dn6_slot = var_t3__blk86_dn6;
        *var_t3__blk86_dn7_slot = var_t3__blk86_dn7;
        *var_t3__blk86_rv_slot = var_t3__blk86_rv;
        *var_t4__blk87_slot = var_t4__blk87;
        *var_t4__blk87_rv_slot = var_t4__blk87_rv;
        *var_t5__blk88_slot = var_t5__blk88;
        *var_t5__blk88_dn0_slot = var_t5__blk88_dn0;
        *var_t5__blk88_dn10_slot = var_t5__blk88_dn10;
        *var_t5__blk88_dn11_slot = var_t5__blk88_dn11;
        *var_t5__blk88_dn12_slot = var_t5__blk88_dn12;
        *var_t5__blk88_dn17_slot = var_t5__blk88_dn17;
        *var_t5__blk88_dn2_slot = var_t5__blk88_dn2;
        *var_t5__blk88_dn6_slot = var_t5__blk88_dn6;
        *var_t5__blk88_dn7_slot = var_t5__blk88_dn7;
        *var_t5__blk88_rv_slot = var_t5__blk88_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vbsz2_slot = var_vbsz2;
        *var_vbsz2_dn0_slot = var_vbsz2_dn0;
        *var_vbsz2_dn10_slot = var_vbsz2_dn10;
        *var_vbsz2_dn11_slot = var_vbsz2_dn11;
        *var_vbsz2_dn12_slot = var_vbsz2_dn12;
        *var_vbsz2_dn17_slot = var_vbsz2_dn17;
        *var_vbsz2_dn2_slot = var_vbsz2_dn2;
        *var_vbsz2_dn6_slot = var_vbsz2_dn6;
        *var_vbsz2_dn7_slot = var_vbsz2_dn7;
        *var_vbsz2_rv_slot = var_vbsz2_rv;
        *var_vth0_slot = var_vth0;
        *var_vth0_dn0_slot = var_vth0_dn0;
        *var_vth0_dn10_slot = var_vth0_dn10;
        *var_vth0_dn11_slot = var_vth0_dn11;
        *var_vth0_dn12_slot = var_vth0_dn12;
        *var_vth0_dn17_slot = var_vth0_dn17;
        *var_vth0_dn2_slot = var_vth0_dn2;
        *var_vth0_dn6_slot = var_vth0_dn6;
        *var_vth0_dn7_slot = var_vth0_dn7;
        *var_vth0_rv_slot = var_vth0_rv;
        *var_vthp_slot = var_vthp;
        *var_vthp_dn0_slot = var_vthp_dn0;
        *var_vthp_dn10_slot = var_vthp_dn10;
        *var_vthp_dn11_slot = var_vthp_dn11;
        *var_vthp_dn12_slot = var_vthp_dn12;
        *var_vthp_dn17_slot = var_vthp_dn17;
        *var_vthp_dn2_slot = var_vthp_dn2;
        *var_vthp_dn6_slot = var_vthp_dn6;
        *var_vthp_dn7_slot = var_vthp_dn7;
        *var_vthp_rv_slot = var_vthp_rv;
        *var_wd0_slot = var_wd0;
        *var_wd0_dn0_slot = var_wd0_dn0;
        *var_wd0_dn10_slot = var_wd0_dn10;
        *var_wd0_dn11_slot = var_wd0_dn11;
        *var_wd0_dn12_slot = var_wd0_dn12;
        *var_wd0_dn17_slot = var_wd0_dn17;
        *var_wd0_dn2_slot = var_wd0_dn2;
        *var_wd0_dn6_slot = var_wd0_dn6;
        *var_wd0_dn7_slot = var_wd0_dn7;
        *var_wd0_rv_slot = var_wd0_rv;
    }

    pub(super) fn stamp_reactive_block_14(
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
        var_c_fox_inv: f64,
        var_c_fox_inv_dn0: f64,
        var_c_fox_inv_dn10: f64,
        var_c_fox_inv_dn11: f64,
        var_c_fox_inv_dn12: f64,
        var_c_fox_inv_dn17: f64,
        var_c_fox_inv_dn2: f64,
        var_c_fox_inv_dn6: f64,
        var_c_fox_inv_dn7: f64,
        var_cnstpgd: f64,
        var_dvth0__blk89: f64,
        var_dvth0__blk89_dn0: f64,
        var_dvth0__blk89_dn10: f64,
        var_dvth0__blk89_dn11: f64,
        var_dvth0__blk89_dn12: f64,
        var_dvth0__blk89_dn17: f64,
        var_dvth0__blk89_dn2: f64,
        var_dvth0__blk89_dn6: f64,
        var_dvth0__blk89_dn7: f64,
        var_dvthsm: f64,
        var_eg: f64,
        var_eg_dn0: f64,
        var_eg_dn10: f64,
        var_eg_dn11: f64,
        var_eg_dn12: f64,
        var_eg_dn17: f64,
        var_eg_dn2: f64,
        var_eg_dn6: f64,
        var_eg_dn7: f64,
        var_guard90: f64,
        var_lgleff: f64,
        var_mks_parl1: f64,
        var_mks_wfc: f64,
        var_pb2: f64,
        var_pb20b: f64,
        var_pb20b_dn0: f64,
        var_pb20b_dn10: f64,
        var_pb20b_dn11: f64,
        var_pb20b_dn12: f64,
        var_pb20b_dn17: f64,
        var_pb20b_dn2: f64,
        var_pb20b_dn6: f64,
        var_pb20b_dn7: f64,
        var_pb2_dn0: f64,
        var_pb2_dn10: f64,
        var_pb2_dn11: f64,
        var_pb2_dn12: f64,
        var_pb2_dn17: f64,
        var_pb2_dn2: f64,
        var_pb2_dn6: f64,
        var_pb2_dn7: f64,
        var_pbsum: f64,
        var_pbsum_dn0: f64,
        var_pbsum_dn10: f64,
        var_pbsum_dn11: f64,
        var_pbsum_dn12: f64,
        var_pbsum_dn17: f64,
        var_pbsum_dn2: f64,
        var_pbsum_dn6: f64,
        var_pbsum_dn7: f64,
        var_qb0: f64,
        var_qb0_dn0: f64,
        var_qb0_dn10: f64,
        var_qb0_dn11: f64,
        var_qb0_dn12: f64,
        var_qb0_dn17: f64,
        var_qb0_dn2: f64,
        var_qb0_dn6: f64,
        var_qb0_dn7: f64,
        var_t1__blk84: f64,
        var_t1__blk84_dn0: f64,
        var_t1__blk84_dn10: f64,
        var_t1__blk84_dn11: f64,
        var_t1__blk84_dn12: f64,
        var_t1__blk84_dn17: f64,
        var_t1__blk84_dn2: f64,
        var_t1__blk84_dn6: f64,
        var_t1__blk84_dn7: f64,
        var_t2__blk85: f64,
        var_t2__blk85_dn0: f64,
        var_t2__blk85_dn10: f64,
        var_t2__blk85_dn11: f64,
        var_t2__blk85_dn12: f64,
        var_t2__blk85_dn17: f64,
        var_t2__blk85_dn2: f64,
        var_t2__blk85_dn6: f64,
        var_t2__blk85_dn7: f64,
        var_uc_sc2: f64,
        var_uc_sc3: f64,
        var_uc_scp2: f64,
        var_vdsz: f64,
        var_vdsz_dn0: f64,
        var_vdsz_dn10: f64,
        var_vdsz_dn11: f64,
        var_vdsz_dn12: f64,
        var_vdsz_dn17: f64,
        var_vdsz_dn2: f64,
        var_vdsz_dn6: f64,
        var_vdsz_dn7: f64,
        var_vgsz: f64,
        var_vgsz_dn0: f64,
        var_vgsz_dn10: f64,
        var_vgsz_dn11: f64,
        var_vgsz_dn12: f64,
        var_vgsz_dn17: f64,
        var_vgsz_dn2: f64,
        var_vgsz_dn6: f64,
        var_vgsz_dn7: f64,
        var_vthp: f64,
        var_vthp_dn0: f64,
        var_vthp_dn10: f64,
        var_vthp_dn11: f64,
        var_vthp_dn12: f64,
        var_vthp_dn17: f64,
        var_vthp_dn2: f64,
        var_vthp_dn6: f64,
        var_vthp_dn7: f64,
        var_wd0: f64,
        var_wd0_dn0: f64,
        var_wd0_dn10: f64,
        var_wd0_dn11: f64,
        var_wd0_dn12: f64,
        var_wd0_dn17: f64,
        var_wd0_dn2: f64,
        var_wd0_dn6: f64,
        var_wd0_dn7: f64,
        var_weff: f64,
        var_wg: f64,
        var_dppg_slot: &mut f64,
        var_dppg_dn0_slot: &mut f64,
        var_dppg_dn10_slot: &mut f64,
        var_dppg_dn11_slot: &mut f64,
        var_dppg_dn12_slot: &mut f64,
        var_dppg_dn17_slot: &mut f64,
        var_dppg_dn2_slot: &mut f64,
        var_dppg_dn6_slot: &mut f64,
        var_dppg_dn7_slot: &mut f64,
        var_dppg_rv_slot: &mut f64,
        var_dvth_slot: &mut f64,
        var_dvth0__blk97_slot: &mut f64,
        var_dvth0__blk97_dn0_slot: &mut f64,
        var_dvth0__blk97_dn10_slot: &mut f64,
        var_dvth0__blk97_dn11_slot: &mut f64,
        var_dvth0__blk97_dn12_slot: &mut f64,
        var_dvth0__blk97_dn17_slot: &mut f64,
        var_dvth0__blk97_dn2_slot: &mut f64,
        var_dvth0__blk97_dn6_slot: &mut f64,
        var_dvth0__blk97_dn7_slot: &mut f64,
        var_dvth0__blk97_rv_slot: &mut f64,
        var_dvth_dn0_slot: &mut f64,
        var_dvth_dn10_slot: &mut f64,
        var_dvth_dn11_slot: &mut f64,
        var_dvth_dn12_slot: &mut f64,
        var_dvth_dn17_slot: &mut f64,
        var_dvth_dn2_slot: &mut f64,
        var_dvth_dn6_slot: &mut f64,
        var_dvth_dn7_slot: &mut f64,
        var_dvth_rv_slot: &mut f64,
        var_dvthlp_slot: &mut f64,
        var_dvthlp_dn0_slot: &mut f64,
        var_dvthlp_dn10_slot: &mut f64,
        var_dvthlp_dn11_slot: &mut f64,
        var_dvthlp_dn12_slot: &mut f64,
        var_dvthlp_dn17_slot: &mut f64,
        var_dvthlp_dn2_slot: &mut f64,
        var_dvthlp_dn6_slot: &mut f64,
        var_dvthlp_dn7_slot: &mut f64,
        var_dvthlp_rv_slot: &mut f64,
        var_dvthsc_slot: &mut f64,
        var_dvthsc_dn0_slot: &mut f64,
        var_dvthsc_dn10_slot: &mut f64,
        var_dvthsc_dn11_slot: &mut f64,
        var_dvthsc_dn12_slot: &mut f64,
        var_dvthsc_dn17_slot: &mut f64,
        var_dvthsc_dn2_slot: &mut f64,
        var_dvthsc_dn6_slot: &mut f64,
        var_dvthsc_dn7_slot: &mut f64,
        var_dvthsc_rv_slot: &mut f64,
        var_dvthscr_slot: &mut f64,
        var_dvthscr_dn0_slot: &mut f64,
        var_dvthscr_dn10_slot: &mut f64,
        var_dvthscr_dn11_slot: &mut f64,
        var_dvthscr_dn12_slot: &mut f64,
        var_dvthscr_dn17_slot: &mut f64,
        var_dvthscr_dn2_slot: &mut f64,
        var_dvthscr_dn6_slot: &mut f64,
        var_dvthscr_dn7_slot: &mut f64,
        var_dvthscr_rv_slot: &mut f64,
        var_dvthw_slot: &mut f64,
        var_dvthw_dn0_slot: &mut f64,
        var_dvthw_dn10_slot: &mut f64,
        var_dvthw_dn11_slot: &mut f64,
        var_dvthw_dn12_slot: &mut f64,
        var_dvthw_dn17_slot: &mut f64,
        var_dvthw_dn2_slot: &mut f64,
        var_dvthw_dn6_slot: &mut f64,
        var_dvthw_dn7_slot: &mut f64,
        var_dvthw_rv_slot: &mut f64,
        var_flg_dppg_slot: &mut f64,
        var_flg_dppg_rv_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard101_rv_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard108_rv_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard109_rv_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard110_rv_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard111_rv_slot: &mut f64,
        var_t0__blk106_slot: &mut f64,
        var_t0__blk106_rv_slot: &mut f64,
        var_t0__blk91_slot: &mut f64,
        var_t0__blk91_dn0_slot: &mut f64,
        var_t0__blk91_dn10_slot: &mut f64,
        var_t0__blk91_dn11_slot: &mut f64,
        var_t0__blk91_dn12_slot: &mut f64,
        var_t0__blk91_dn17_slot: &mut f64,
        var_t0__blk91_dn2_slot: &mut f64,
        var_t0__blk91_dn6_slot: &mut f64,
        var_t0__blk91_dn7_slot: &mut f64,
        var_t0__blk91_rv_slot: &mut f64,
        var_t1__blk102_slot: &mut f64,
        var_t1__blk102_dn0_slot: &mut f64,
        var_t1__blk102_dn10_slot: &mut f64,
        var_t1__blk102_dn11_slot: &mut f64,
        var_t1__blk102_dn12_slot: &mut f64,
        var_t1__blk102_dn17_slot: &mut f64,
        var_t1__blk102_dn2_slot: &mut f64,
        var_t1__blk102_dn6_slot: &mut f64,
        var_t1__blk102_dn7_slot: &mut f64,
        var_t1__blk102_rv_slot: &mut f64,
        var_t1__blk92_slot: &mut f64,
        var_t1__blk92_dn0_slot: &mut f64,
        var_t1__blk92_dn10_slot: &mut f64,
        var_t1__blk92_dn11_slot: &mut f64,
        var_t1__blk92_dn12_slot: &mut f64,
        var_t1__blk92_dn17_slot: &mut f64,
        var_t1__blk92_dn2_slot: &mut f64,
        var_t1__blk92_dn6_slot: &mut f64,
        var_t1__blk92_dn7_slot: &mut f64,
        var_t1__blk92_rv_slot: &mut f64,
        var_t1__blk98_slot: &mut f64,
        var_t1__blk98_dn0_slot: &mut f64,
        var_t1__blk98_dn10_slot: &mut f64,
        var_t1__blk98_dn11_slot: &mut f64,
        var_t1__blk98_dn12_slot: &mut f64,
        var_t1__blk98_dn17_slot: &mut f64,
        var_t1__blk98_dn2_slot: &mut f64,
        var_t1__blk98_dn6_slot: &mut f64,
        var_t1__blk98_dn7_slot: &mut f64,
        var_t1__blk98_rv_slot: &mut f64,
        var_t2__blk93_slot: &mut f64,
        var_t2__blk93_dn0_slot: &mut f64,
        var_t2__blk93_dn10_slot: &mut f64,
        var_t2__blk93_dn11_slot: &mut f64,
        var_t2__blk93_dn12_slot: &mut f64,
        var_t2__blk93_dn17_slot: &mut f64,
        var_t2__blk93_dn2_slot: &mut f64,
        var_t2__blk93_dn6_slot: &mut f64,
        var_t2__blk93_dn7_slot: &mut f64,
        var_t2__blk93_rv_slot: &mut f64,
        var_t2__blk99_slot: &mut f64,
        var_t2__blk99_rv_slot: &mut f64,
        var_t3__blk100_slot: &mut f64,
        var_t3__blk100_rv_slot: &mut f64,
        var_t3__blk103_slot: &mut f64,
        var_t3__blk103_dn0_slot: &mut f64,
        var_t3__blk103_dn10_slot: &mut f64,
        var_t3__blk103_dn11_slot: &mut f64,
        var_t3__blk103_dn12_slot: &mut f64,
        var_t3__blk103_dn17_slot: &mut f64,
        var_t3__blk103_dn2_slot: &mut f64,
        var_t3__blk103_dn6_slot: &mut f64,
        var_t3__blk103_dn7_slot: &mut f64,
        var_t3__blk103_rv_slot: &mut f64,
        var_t3__blk107_slot: &mut f64,
        var_t3__blk107_dn0_slot: &mut f64,
        var_t3__blk107_dn10_slot: &mut f64,
        var_t3__blk107_dn11_slot: &mut f64,
        var_t3__blk107_dn12_slot: &mut f64,
        var_t3__blk107_dn17_slot: &mut f64,
        var_t3__blk107_dn2_slot: &mut f64,
        var_t3__blk107_dn6_slot: &mut f64,
        var_t3__blk107_dn7_slot: &mut f64,
        var_t3__blk107_rv_slot: &mut f64,
        var_t3__blk86_slot: &mut f64,
        var_t3__blk86_dn0_slot: &mut f64,
        var_t3__blk86_dn10_slot: &mut f64,
        var_t3__blk86_dn11_slot: &mut f64,
        var_t3__blk86_dn12_slot: &mut f64,
        var_t3__blk86_dn17_slot: &mut f64,
        var_t3__blk86_dn2_slot: &mut f64,
        var_t3__blk86_dn6_slot: &mut f64,
        var_t3__blk86_dn7_slot: &mut f64,
        var_t3__blk86_rv_slot: &mut f64,
        var_t3__blk94_slot: &mut f64,
        var_t3__blk94_rv_slot: &mut f64,
        var_t4__blk95_slot: &mut f64,
        var_t4__blk95_dn0_slot: &mut f64,
        var_t4__blk95_dn10_slot: &mut f64,
        var_t4__blk95_dn11_slot: &mut f64,
        var_t4__blk95_dn12_slot: &mut f64,
        var_t4__blk95_dn17_slot: &mut f64,
        var_t4__blk95_dn2_slot: &mut f64,
        var_t4__blk95_dn6_slot: &mut f64,
        var_t4__blk95_dn7_slot: &mut f64,
        var_t4__blk95_rv_slot: &mut f64,
        var_t5__blk104_slot: &mut f64,
        var_t5__blk104_dn0_slot: &mut f64,
        var_t5__blk104_dn10_slot: &mut f64,
        var_t5__blk104_dn11_slot: &mut f64,
        var_t5__blk104_dn12_slot: &mut f64,
        var_t5__blk104_dn17_slot: &mut f64,
        var_t5__blk104_dn2_slot: &mut f64,
        var_t5__blk104_dn6_slot: &mut f64,
        var_t5__blk104_dn7_slot: &mut f64,
        var_t5__blk104_rv_slot: &mut f64,
        var_t5__blk88_slot: &mut f64,
        var_t5__blk88_dn0_slot: &mut f64,
        var_t5__blk88_dn10_slot: &mut f64,
        var_t5__blk88_dn11_slot: &mut f64,
        var_t5__blk88_dn12_slot: &mut f64,
        var_t5__blk88_dn17_slot: &mut f64,
        var_t5__blk88_dn2_slot: &mut f64,
        var_t5__blk88_dn6_slot: &mut f64,
        var_t5__blk88_dn7_slot: &mut f64,
        var_t5__blk88_rv_slot: &mut f64,
        var_t5__blk96_slot: &mut f64,
        var_t5__blk96_dn0_slot: &mut f64,
        var_t5__blk96_dn10_slot: &mut f64,
        var_t5__blk96_dn11_slot: &mut f64,
        var_t5__blk96_dn12_slot: &mut f64,
        var_t5__blk96_dn17_slot: &mut f64,
        var_t5__blk96_dn2_slot: &mut f64,
        var_t5__blk96_dn6_slot: &mut f64,
        var_t5__blk96_dn7_slot: &mut f64,
        var_t5__blk96_rv_slot: &mut f64,
        var_t7__blk105_slot: &mut f64,
        var_t7__blk105_dn0_slot: &mut f64,
        var_t7__blk105_dn10_slot: &mut f64,
        var_t7__blk105_dn11_slot: &mut f64,
        var_t7__blk105_dn12_slot: &mut f64,
        var_t7__blk105_dn17_slot: &mut f64,
        var_t7__blk105_dn2_slot: &mut f64,
        var_t7__blk105_dn6_slot: &mut f64,
        var_t7__blk105_dn7_slot: &mut f64,
        var_t7__blk105_rv_slot: &mut f64,
        var_vth_slot: &mut f64,
        var_vth_dn0_slot: &mut f64,
        var_vth_dn10_slot: &mut f64,
        var_vth_dn11_slot: &mut f64,
        var_vth_dn12_slot: &mut f64,
        var_vth_dn17_slot: &mut f64,
        var_vth_dn2_slot: &mut f64,
        var_vth_dn6_slot: &mut f64,
        var_vth_dn7_slot: &mut f64,
        var_vth_rv_slot: &mut f64,
    ) {
        let mut var_dppg: f64 = *var_dppg_slot;
        let mut var_dppg_dn0: f64 = *var_dppg_dn0_slot;
        let mut var_dppg_dn10: f64 = *var_dppg_dn10_slot;
        let mut var_dppg_dn11: f64 = *var_dppg_dn11_slot;
        let mut var_dppg_dn12: f64 = *var_dppg_dn12_slot;
        let mut var_dppg_dn17: f64 = *var_dppg_dn17_slot;
        let mut var_dppg_dn2: f64 = *var_dppg_dn2_slot;
        let mut var_dppg_dn6: f64 = *var_dppg_dn6_slot;
        let mut var_dppg_dn7: f64 = *var_dppg_dn7_slot;
        let mut var_dppg_rv: f64 = *var_dppg_rv_slot;
        let mut var_dvth: f64 = *var_dvth_slot;
        let mut var_dvth0__blk97: f64 = *var_dvth0__blk97_slot;
        let mut var_dvth0__blk97_dn0: f64 = *var_dvth0__blk97_dn0_slot;
        let mut var_dvth0__blk97_dn10: f64 = *var_dvth0__blk97_dn10_slot;
        let mut var_dvth0__blk97_dn11: f64 = *var_dvth0__blk97_dn11_slot;
        let mut var_dvth0__blk97_dn12: f64 = *var_dvth0__blk97_dn12_slot;
        let mut var_dvth0__blk97_dn17: f64 = *var_dvth0__blk97_dn17_slot;
        let mut var_dvth0__blk97_dn2: f64 = *var_dvth0__blk97_dn2_slot;
        let mut var_dvth0__blk97_dn6: f64 = *var_dvth0__blk97_dn6_slot;
        let mut var_dvth0__blk97_dn7: f64 = *var_dvth0__blk97_dn7_slot;
        let mut var_dvth0__blk97_rv: f64 = *var_dvth0__blk97_rv_slot;
        let mut var_dvth_dn0: f64 = *var_dvth_dn0_slot;
        let mut var_dvth_dn10: f64 = *var_dvth_dn10_slot;
        let mut var_dvth_dn11: f64 = *var_dvth_dn11_slot;
        let mut var_dvth_dn12: f64 = *var_dvth_dn12_slot;
        let mut var_dvth_dn17: f64 = *var_dvth_dn17_slot;
        let mut var_dvth_dn2: f64 = *var_dvth_dn2_slot;
        let mut var_dvth_dn6: f64 = *var_dvth_dn6_slot;
        let mut var_dvth_dn7: f64 = *var_dvth_dn7_slot;
        let mut var_dvth_rv: f64 = *var_dvth_rv_slot;
        let mut var_dvthlp: f64 = *var_dvthlp_slot;
        let mut var_dvthlp_dn0: f64 = *var_dvthlp_dn0_slot;
        let mut var_dvthlp_dn10: f64 = *var_dvthlp_dn10_slot;
        let mut var_dvthlp_dn11: f64 = *var_dvthlp_dn11_slot;
        let mut var_dvthlp_dn12: f64 = *var_dvthlp_dn12_slot;
        let mut var_dvthlp_dn17: f64 = *var_dvthlp_dn17_slot;
        let mut var_dvthlp_dn2: f64 = *var_dvthlp_dn2_slot;
        let mut var_dvthlp_dn6: f64 = *var_dvthlp_dn6_slot;
        let mut var_dvthlp_dn7: f64 = *var_dvthlp_dn7_slot;
        let mut var_dvthlp_rv: f64 = *var_dvthlp_rv_slot;
        let mut var_dvthsc: f64 = *var_dvthsc_slot;
        let mut var_dvthsc_dn0: f64 = *var_dvthsc_dn0_slot;
        let mut var_dvthsc_dn10: f64 = *var_dvthsc_dn10_slot;
        let mut var_dvthsc_dn11: f64 = *var_dvthsc_dn11_slot;
        let mut var_dvthsc_dn12: f64 = *var_dvthsc_dn12_slot;
        let mut var_dvthsc_dn17: f64 = *var_dvthsc_dn17_slot;
        let mut var_dvthsc_dn2: f64 = *var_dvthsc_dn2_slot;
        let mut var_dvthsc_dn6: f64 = *var_dvthsc_dn6_slot;
        let mut var_dvthsc_dn7: f64 = *var_dvthsc_dn7_slot;
        let mut var_dvthsc_rv: f64 = *var_dvthsc_rv_slot;
        let mut var_dvthscr: f64 = *var_dvthscr_slot;
        let mut var_dvthscr_dn0: f64 = *var_dvthscr_dn0_slot;
        let mut var_dvthscr_dn10: f64 = *var_dvthscr_dn10_slot;
        let mut var_dvthscr_dn11: f64 = *var_dvthscr_dn11_slot;
        let mut var_dvthscr_dn12: f64 = *var_dvthscr_dn12_slot;
        let mut var_dvthscr_dn17: f64 = *var_dvthscr_dn17_slot;
        let mut var_dvthscr_dn2: f64 = *var_dvthscr_dn2_slot;
        let mut var_dvthscr_dn6: f64 = *var_dvthscr_dn6_slot;
        let mut var_dvthscr_dn7: f64 = *var_dvthscr_dn7_slot;
        let mut var_dvthscr_rv: f64 = *var_dvthscr_rv_slot;
        let mut var_dvthw: f64 = *var_dvthw_slot;
        let mut var_dvthw_dn0: f64 = *var_dvthw_dn0_slot;
        let mut var_dvthw_dn10: f64 = *var_dvthw_dn10_slot;
        let mut var_dvthw_dn11: f64 = *var_dvthw_dn11_slot;
        let mut var_dvthw_dn12: f64 = *var_dvthw_dn12_slot;
        let mut var_dvthw_dn17: f64 = *var_dvthw_dn17_slot;
        let mut var_dvthw_dn2: f64 = *var_dvthw_dn2_slot;
        let mut var_dvthw_dn6: f64 = *var_dvthw_dn6_slot;
        let mut var_dvthw_dn7: f64 = *var_dvthw_dn7_slot;
        let mut var_dvthw_rv: f64 = *var_dvthw_rv_slot;
        let mut var_flg_dppg: f64 = *var_flg_dppg_slot;
        let mut var_flg_dppg_rv: f64 = *var_flg_dppg_rv_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard101_rv: f64 = *var_guard101_rv_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard108_rv: f64 = *var_guard108_rv_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard109_rv: f64 = *var_guard109_rv_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard110_rv: f64 = *var_guard110_rv_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard111_rv: f64 = *var_guard111_rv_slot;
        let mut var_t0__blk106: f64 = *var_t0__blk106_slot;
        let mut var_t0__blk106_rv: f64 = *var_t0__blk106_rv_slot;
        let mut var_t0__blk91: f64 = *var_t0__blk91_slot;
        let mut var_t0__blk91_dn0: f64 = *var_t0__blk91_dn0_slot;
        let mut var_t0__blk91_dn10: f64 = *var_t0__blk91_dn10_slot;
        let mut var_t0__blk91_dn11: f64 = *var_t0__blk91_dn11_slot;
        let mut var_t0__blk91_dn12: f64 = *var_t0__blk91_dn12_slot;
        let mut var_t0__blk91_dn17: f64 = *var_t0__blk91_dn17_slot;
        let mut var_t0__blk91_dn2: f64 = *var_t0__blk91_dn2_slot;
        let mut var_t0__blk91_dn6: f64 = *var_t0__blk91_dn6_slot;
        let mut var_t0__blk91_dn7: f64 = *var_t0__blk91_dn7_slot;
        let mut var_t0__blk91_rv: f64 = *var_t0__blk91_rv_slot;
        let mut var_t1__blk102: f64 = *var_t1__blk102_slot;
        let mut var_t1__blk102_dn0: f64 = *var_t1__blk102_dn0_slot;
        let mut var_t1__blk102_dn10: f64 = *var_t1__blk102_dn10_slot;
        let mut var_t1__blk102_dn11: f64 = *var_t1__blk102_dn11_slot;
        let mut var_t1__blk102_dn12: f64 = *var_t1__blk102_dn12_slot;
        let mut var_t1__blk102_dn17: f64 = *var_t1__blk102_dn17_slot;
        let mut var_t1__blk102_dn2: f64 = *var_t1__blk102_dn2_slot;
        let mut var_t1__blk102_dn6: f64 = *var_t1__blk102_dn6_slot;
        let mut var_t1__blk102_dn7: f64 = *var_t1__blk102_dn7_slot;
        let mut var_t1__blk102_rv: f64 = *var_t1__blk102_rv_slot;
        let mut var_t1__blk92: f64 = *var_t1__blk92_slot;
        let mut var_t1__blk92_dn0: f64 = *var_t1__blk92_dn0_slot;
        let mut var_t1__blk92_dn10: f64 = *var_t1__blk92_dn10_slot;
        let mut var_t1__blk92_dn11: f64 = *var_t1__blk92_dn11_slot;
        let mut var_t1__blk92_dn12: f64 = *var_t1__blk92_dn12_slot;
        let mut var_t1__blk92_dn17: f64 = *var_t1__blk92_dn17_slot;
        let mut var_t1__blk92_dn2: f64 = *var_t1__blk92_dn2_slot;
        let mut var_t1__blk92_dn6: f64 = *var_t1__blk92_dn6_slot;
        let mut var_t1__blk92_dn7: f64 = *var_t1__blk92_dn7_slot;
        let mut var_t1__blk92_rv: f64 = *var_t1__blk92_rv_slot;
        let mut var_t1__blk98: f64 = *var_t1__blk98_slot;
        let mut var_t1__blk98_dn0: f64 = *var_t1__blk98_dn0_slot;
        let mut var_t1__blk98_dn10: f64 = *var_t1__blk98_dn10_slot;
        let mut var_t1__blk98_dn11: f64 = *var_t1__blk98_dn11_slot;
        let mut var_t1__blk98_dn12: f64 = *var_t1__blk98_dn12_slot;
        let mut var_t1__blk98_dn17: f64 = *var_t1__blk98_dn17_slot;
        let mut var_t1__blk98_dn2: f64 = *var_t1__blk98_dn2_slot;
        let mut var_t1__blk98_dn6: f64 = *var_t1__blk98_dn6_slot;
        let mut var_t1__blk98_dn7: f64 = *var_t1__blk98_dn7_slot;
        let mut var_t1__blk98_rv: f64 = *var_t1__blk98_rv_slot;
        let mut var_t2__blk93: f64 = *var_t2__blk93_slot;
        let mut var_t2__blk93_dn0: f64 = *var_t2__blk93_dn0_slot;
        let mut var_t2__blk93_dn10: f64 = *var_t2__blk93_dn10_slot;
        let mut var_t2__blk93_dn11: f64 = *var_t2__blk93_dn11_slot;
        let mut var_t2__blk93_dn12: f64 = *var_t2__blk93_dn12_slot;
        let mut var_t2__blk93_dn17: f64 = *var_t2__blk93_dn17_slot;
        let mut var_t2__blk93_dn2: f64 = *var_t2__blk93_dn2_slot;
        let mut var_t2__blk93_dn6: f64 = *var_t2__blk93_dn6_slot;
        let mut var_t2__blk93_dn7: f64 = *var_t2__blk93_dn7_slot;
        let mut var_t2__blk93_rv: f64 = *var_t2__blk93_rv_slot;
        let mut var_t2__blk99: f64 = *var_t2__blk99_slot;
        let mut var_t2__blk99_rv: f64 = *var_t2__blk99_rv_slot;
        let mut var_t3__blk100: f64 = *var_t3__blk100_slot;
        let mut var_t3__blk100_rv: f64 = *var_t3__blk100_rv_slot;
        let mut var_t3__blk103: f64 = *var_t3__blk103_slot;
        let mut var_t3__blk103_dn0: f64 = *var_t3__blk103_dn0_slot;
        let mut var_t3__blk103_dn10: f64 = *var_t3__blk103_dn10_slot;
        let mut var_t3__blk103_dn11: f64 = *var_t3__blk103_dn11_slot;
        let mut var_t3__blk103_dn12: f64 = *var_t3__blk103_dn12_slot;
        let mut var_t3__blk103_dn17: f64 = *var_t3__blk103_dn17_slot;
        let mut var_t3__blk103_dn2: f64 = *var_t3__blk103_dn2_slot;
        let mut var_t3__blk103_dn6: f64 = *var_t3__blk103_dn6_slot;
        let mut var_t3__blk103_dn7: f64 = *var_t3__blk103_dn7_slot;
        let mut var_t3__blk103_rv: f64 = *var_t3__blk103_rv_slot;
        let mut var_t3__blk107: f64 = *var_t3__blk107_slot;
        let mut var_t3__blk107_dn0: f64 = *var_t3__blk107_dn0_slot;
        let mut var_t3__blk107_dn10: f64 = *var_t3__blk107_dn10_slot;
        let mut var_t3__blk107_dn11: f64 = *var_t3__blk107_dn11_slot;
        let mut var_t3__blk107_dn12: f64 = *var_t3__blk107_dn12_slot;
        let mut var_t3__blk107_dn17: f64 = *var_t3__blk107_dn17_slot;
        let mut var_t3__blk107_dn2: f64 = *var_t3__blk107_dn2_slot;
        let mut var_t3__blk107_dn6: f64 = *var_t3__blk107_dn6_slot;
        let mut var_t3__blk107_dn7: f64 = *var_t3__blk107_dn7_slot;
        let mut var_t3__blk107_rv: f64 = *var_t3__blk107_rv_slot;
        let mut var_t3__blk86: f64 = *var_t3__blk86_slot;
        let mut var_t3__blk86_dn0: f64 = *var_t3__blk86_dn0_slot;
        let mut var_t3__blk86_dn10: f64 = *var_t3__blk86_dn10_slot;
        let mut var_t3__blk86_dn11: f64 = *var_t3__blk86_dn11_slot;
        let mut var_t3__blk86_dn12: f64 = *var_t3__blk86_dn12_slot;
        let mut var_t3__blk86_dn17: f64 = *var_t3__blk86_dn17_slot;
        let mut var_t3__blk86_dn2: f64 = *var_t3__blk86_dn2_slot;
        let mut var_t3__blk86_dn6: f64 = *var_t3__blk86_dn6_slot;
        let mut var_t3__blk86_dn7: f64 = *var_t3__blk86_dn7_slot;
        let mut var_t3__blk86_rv: f64 = *var_t3__blk86_rv_slot;
        let mut var_t3__blk94: f64 = *var_t3__blk94_slot;
        let mut var_t3__blk94_rv: f64 = *var_t3__blk94_rv_slot;
        let mut var_t4__blk95: f64 = *var_t4__blk95_slot;
        let mut var_t4__blk95_dn0: f64 = *var_t4__blk95_dn0_slot;
        let mut var_t4__blk95_dn10: f64 = *var_t4__blk95_dn10_slot;
        let mut var_t4__blk95_dn11: f64 = *var_t4__blk95_dn11_slot;
        let mut var_t4__blk95_dn12: f64 = *var_t4__blk95_dn12_slot;
        let mut var_t4__blk95_dn17: f64 = *var_t4__blk95_dn17_slot;
        let mut var_t4__blk95_dn2: f64 = *var_t4__blk95_dn2_slot;
        let mut var_t4__blk95_dn6: f64 = *var_t4__blk95_dn6_slot;
        let mut var_t4__blk95_dn7: f64 = *var_t4__blk95_dn7_slot;
        let mut var_t4__blk95_rv: f64 = *var_t4__blk95_rv_slot;
        let mut var_t5__blk104: f64 = *var_t5__blk104_slot;
        let mut var_t5__blk104_dn0: f64 = *var_t5__blk104_dn0_slot;
        let mut var_t5__blk104_dn10: f64 = *var_t5__blk104_dn10_slot;
        let mut var_t5__blk104_dn11: f64 = *var_t5__blk104_dn11_slot;
        let mut var_t5__blk104_dn12: f64 = *var_t5__blk104_dn12_slot;
        let mut var_t5__blk104_dn17: f64 = *var_t5__blk104_dn17_slot;
        let mut var_t5__blk104_dn2: f64 = *var_t5__blk104_dn2_slot;
        let mut var_t5__blk104_dn6: f64 = *var_t5__blk104_dn6_slot;
        let mut var_t5__blk104_dn7: f64 = *var_t5__blk104_dn7_slot;
        let mut var_t5__blk104_rv: f64 = *var_t5__blk104_rv_slot;
        let mut var_t5__blk88: f64 = *var_t5__blk88_slot;
        let mut var_t5__blk88_dn0: f64 = *var_t5__blk88_dn0_slot;
        let mut var_t5__blk88_dn10: f64 = *var_t5__blk88_dn10_slot;
        let mut var_t5__blk88_dn11: f64 = *var_t5__blk88_dn11_slot;
        let mut var_t5__blk88_dn12: f64 = *var_t5__blk88_dn12_slot;
        let mut var_t5__blk88_dn17: f64 = *var_t5__blk88_dn17_slot;
        let mut var_t5__blk88_dn2: f64 = *var_t5__blk88_dn2_slot;
        let mut var_t5__blk88_dn6: f64 = *var_t5__blk88_dn6_slot;
        let mut var_t5__blk88_dn7: f64 = *var_t5__blk88_dn7_slot;
        let mut var_t5__blk88_rv: f64 = *var_t5__blk88_rv_slot;
        let mut var_t5__blk96: f64 = *var_t5__blk96_slot;
        let mut var_t5__blk96_dn0: f64 = *var_t5__blk96_dn0_slot;
        let mut var_t5__blk96_dn10: f64 = *var_t5__blk96_dn10_slot;
        let mut var_t5__blk96_dn11: f64 = *var_t5__blk96_dn11_slot;
        let mut var_t5__blk96_dn12: f64 = *var_t5__blk96_dn12_slot;
        let mut var_t5__blk96_dn17: f64 = *var_t5__blk96_dn17_slot;
        let mut var_t5__blk96_dn2: f64 = *var_t5__blk96_dn2_slot;
        let mut var_t5__blk96_dn6: f64 = *var_t5__blk96_dn6_slot;
        let mut var_t5__blk96_dn7: f64 = *var_t5__blk96_dn7_slot;
        let mut var_t5__blk96_rv: f64 = *var_t5__blk96_rv_slot;
        let mut var_t7__blk105: f64 = *var_t7__blk105_slot;
        let mut var_t7__blk105_dn0: f64 = *var_t7__blk105_dn0_slot;
        let mut var_t7__blk105_dn10: f64 = *var_t7__blk105_dn10_slot;
        let mut var_t7__blk105_dn11: f64 = *var_t7__blk105_dn11_slot;
        let mut var_t7__blk105_dn12: f64 = *var_t7__blk105_dn12_slot;
        let mut var_t7__blk105_dn17: f64 = *var_t7__blk105_dn17_slot;
        let mut var_t7__blk105_dn2: f64 = *var_t7__blk105_dn2_slot;
        let mut var_t7__blk105_dn6: f64 = *var_t7__blk105_dn6_slot;
        let mut var_t7__blk105_dn7: f64 = *var_t7__blk105_dn7_slot;
        let mut var_t7__blk105_rv: f64 = *var_t7__blk105_rv_slot;
        let mut var_vth: f64 = *var_vth_slot;
        let mut var_vth_dn0: f64 = *var_vth_dn0_slot;
        let mut var_vth_dn10: f64 = *var_vth_dn10_slot;
        let mut var_vth_dn11: f64 = *var_vth_dn11_slot;
        let mut var_vth_dn12: f64 = *var_vth_dn12_slot;
        let mut var_vth_dn17: f64 = *var_vth_dn17_slot;
        let mut var_vth_dn2: f64 = *var_vth_dn2_slot;
        let mut var_vth_dn6: f64 = *var_vth_dn6_slot;
        let mut var_vth_dn7: f64 = *var_vth_dn7_slot;
        let mut var_vth_rv: f64 = *var_vth_rv_slot;

        let (assign6490_e4327, assign6490_e4327_d_n0, assign6490_e4327_d_n2, assign6490_e4327_d_n6, assign6490_e4327_d_n7, assign6490_e4327_d_n10, assign6490_e4327_d_n11, assign6490_e4327_d_n12, assign6490_e4327_d_n17,) = {
    if (var_guard90 != 0.0) {
        (var_uc_scp2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t5__blk88, var_t5__blk88_dn0, var_t5__blk88_dn2, var_t5__blk88_dn6, var_t5__blk88_dn7, var_t5__blk88_dn10, var_t5__blk88_dn11, var_t5__blk88_dn12, var_t5__blk88_dn17,)
    }
};
        var_t5__blk88 = assign6490_e4327;
        var_t5__blk88_dn0 = assign6490_e4327_d_n0;
        var_t5__blk88_dn2 = assign6490_e4327_d_n2;
        var_t5__blk88_dn6 = assign6490_e4327_d_n6;
        var_t5__blk88_dn7 = assign6490_e4327_d_n7;
        var_t5__blk88_dn10 = assign6490_e4327_d_n10;
        var_t5__blk88_dn11 = assign6490_e4327_d_n11;
        var_t5__blk88_dn12 = assign6490_e4327_d_n12;
        var_t5__blk88_dn17 = assign6490_e4327_d_n17;
        var_t5__blk88_rv = 0.0;

        let (assign6500_e4335, assign6500_e4335_d_n0, assign6500_e4335_d_n2, assign6500_e4335_d_n6, assign6500_e4335_d_n7, assign6500_e4335_d_n10, assign6500_e4335_d_n11, assign6500_e4335_d_n12, assign6500_e4335_d_n17,) = {
    if (var_guard90 != 0.0) {
        let assign6500_e4332: f64 = (var_t5__blk88 * var_vdsz);
        let assign6500_e4333: f64 = (var_t2__blk85 + assign6500_e4332);
        (assign6500_e4333, (var_t2__blk85_dn0 + ((var_t5__blk88_dn0 * var_vdsz) + (var_t5__blk88 * var_vdsz_dn0))), (var_t2__blk85_dn2 + ((var_t5__blk88_dn2 * var_vdsz) + (var_t5__blk88 * var_vdsz_dn2))), (var_t2__blk85_dn6 + ((var_t5__blk88_dn6 * var_vdsz) + (var_t5__blk88 * var_vdsz_dn6))), (var_t2__blk85_dn7 + ((var_t5__blk88_dn7 * var_vdsz) + (var_t5__blk88 * var_vdsz_dn7))), (var_t2__blk85_dn10 + ((var_t5__blk88_dn10 * var_vdsz) + (var_t5__blk88 * var_vdsz_dn10))), (var_t2__blk85_dn11 + ((var_t5__blk88_dn11 * var_vdsz) + (var_t5__blk88 * var_vdsz_dn11))), (var_t2__blk85_dn12 + ((var_t5__blk88_dn12 * var_vdsz) + (var_t5__blk88 * var_vdsz_dn12))), (var_t2__blk85_dn17 + ((var_t5__blk88_dn17 * var_vdsz) + (var_t5__blk88 * var_vdsz_dn17))),)
    } else {
        (var_t3__blk86, var_t3__blk86_dn0, var_t3__blk86_dn2, var_t3__blk86_dn6, var_t3__blk86_dn7, var_t3__blk86_dn10, var_t3__blk86_dn11, var_t3__blk86_dn12, var_t3__blk86_dn17,)
    }
};
        var_t3__blk86 = assign6500_e4335;
        var_t3__blk86_dn0 = assign6500_e4335_d_n0;
        var_t3__blk86_dn2 = assign6500_e4335_d_n2;
        var_t3__blk86_dn6 = assign6500_e4335_d_n6;
        var_t3__blk86_dn7 = assign6500_e4335_d_n7;
        var_t3__blk86_dn10 = assign6500_e4335_d_n10;
        var_t3__blk86_dn11 = assign6500_e4335_d_n11;
        var_t3__blk86_dn12 = assign6500_e4335_d_n12;
        var_t3__blk86_dn17 = assign6500_e4335_d_n17;
        var_t3__blk86_rv = 0.0;

        let (assign6510_e4343, assign6510_e4343_d_n0, assign6510_e4343_d_n2, assign6510_e4343_d_n6, assign6510_e4343_d_n7, assign6510_e4343_d_n10, assign6510_e4343_d_n11, assign6510_e4343_d_n12, assign6510_e4343_d_n17,) = {
    if (var_guard90 != 0.0) {
        let assign6510_e4339: f64 = (var_t1__blk84 * var_dvth0__blk89);
        let assign6510_e4341: f64 = (assign6510_e4339 * var_t3__blk86);
        (assign6510_e4341, ((((var_t1__blk84_dn0 * var_dvth0__blk89) + (var_t1__blk84 * var_dvth0__blk89_dn0)) * var_t3__blk86) + (assign6510_e4339 * var_t3__blk86_dn0)), ((((var_t1__blk84_dn2 * var_dvth0__blk89) + (var_t1__blk84 * var_dvth0__blk89_dn2)) * var_t3__blk86) + (assign6510_e4339 * var_t3__blk86_dn2)), ((((var_t1__blk84_dn6 * var_dvth0__blk89) + (var_t1__blk84 * var_dvth0__blk89_dn6)) * var_t3__blk86) + (assign6510_e4339 * var_t3__blk86_dn6)), ((((var_t1__blk84_dn7 * var_dvth0__blk89) + (var_t1__blk84 * var_dvth0__blk89_dn7)) * var_t3__blk86) + (assign6510_e4339 * var_t3__blk86_dn7)), ((((var_t1__blk84_dn10 * var_dvth0__blk89) + (var_t1__blk84 * var_dvth0__blk89_dn10)) * var_t3__blk86) + (assign6510_e4339 * var_t3__blk86_dn10)), ((((var_t1__blk84_dn11 * var_dvth0__blk89) + (var_t1__blk84 * var_dvth0__blk89_dn11)) * var_t3__blk86) + (assign6510_e4339 * var_t3__blk86_dn11)), ((((var_t1__blk84_dn12 * var_dvth0__blk89) + (var_t1__blk84 * var_dvth0__blk89_dn12)) * var_t3__blk86) + (assign6510_e4339 * var_t3__blk86_dn12)), ((((var_t1__blk84_dn17 * var_dvth0__blk89) + (var_t1__blk84 * var_dvth0__blk89_dn17)) * var_t3__blk86) + (assign6510_e4339 * var_t3__blk86_dn17)),)
    } else {
        (var_dvthlp, var_dvthlp_dn0, var_dvthlp_dn2, var_dvthlp_dn6, var_dvthlp_dn7, var_dvthlp_dn10, var_dvthlp_dn11, var_dvthlp_dn12, var_dvthlp_dn17,)
    }
};
        var_dvthlp = assign6510_e4343;
        var_dvthlp_dn0 = assign6510_e4343_d_n0;
        var_dvthlp_dn2 = assign6510_e4343_d_n2;
        var_dvthlp_dn6 = assign6510_e4343_d_n6;
        var_dvthlp_dn7 = assign6510_e4343_d_n7;
        var_dvthlp_dn10 = assign6510_e4343_d_n10;
        var_dvthlp_dn11 = assign6510_e4343_d_n11;
        var_dvthlp_dn12 = assign6510_e4343_d_n12;
        var_dvthlp_dn17 = assign6510_e4343_d_n17;
        var_dvthlp_rv = 0.0;

        let (assign6520_e4348, assign6520_e4348_d_n0, assign6520_e4348_d_n2, assign6520_e4348_d_n6, assign6520_e4348_d_n7, assign6520_e4348_d_n10, assign6520_e4348_d_n11, assign6520_e4348_d_n12, assign6520_e4348_d_n17,) = {
    if (var_guard90 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dvthlp, var_dvthlp_dn0, var_dvthlp_dn2, var_dvthlp_dn6, var_dvthlp_dn7, var_dvthlp_dn10, var_dvthlp_dn11, var_dvthlp_dn12, var_dvthlp_dn17,)
    }
};
        var_dvthlp = assign6520_e4348;
        var_dvthlp_dn0 = assign6520_e4348_d_n0;
        var_dvthlp_dn2 = assign6520_e4348_d_n2;
        var_dvthlp_dn6 = assign6520_e4348_d_n6;
        var_dvthlp_dn7 = assign6520_e4348_d_n7;
        var_dvthlp_dn10 = assign6520_e4348_d_n10;
        var_dvthlp_dn11 = assign6520_e4348_d_n11;
        var_dvthlp_dn12 = assign6520_e4348_d_n12;
        var_dvthlp_dn17 = assign6520_e4348_d_n17;
        var_dvthlp_rv = 0.0;

        let assign6530_e4351: f64 = (1.034943e-10 * var_wd0);
        let assign6530_e4353: f64 = (assign6530_e4351 * 2.0);
        var_t0__blk91 = assign6530_e4353;
        var_t0__blk91_dn0 = ((1.034943e-10 * var_wd0_dn0) * 2.0);
        var_t0__blk91_dn2 = ((1.034943e-10 * var_wd0_dn2) * 2.0);
        var_t0__blk91_dn6 = ((1.034943e-10 * var_wd0_dn6) * 2.0);
        var_t0__blk91_dn7 = ((1.034943e-10 * var_wd0_dn7) * 2.0);
        var_t0__blk91_dn10 = ((1.034943e-10 * var_wd0_dn10) * 2.0);
        var_t0__blk91_dn11 = ((1.034943e-10 * var_wd0_dn11) * 2.0);
        var_t0__blk91_dn12 = ((1.034943e-10 * var_wd0_dn12) * 2.0);
        var_t0__blk91_dn17 = ((1.034943e-10 * var_wd0_dn17) * 2.0);
        var_t0__blk91_rv = 0.0;

        let assign6540_e4356: f64 = (var_c_fox_inv * var_t0__blk91);
        var_t1__blk92 = assign6540_e4356;
        var_t1__blk92_dn0 = ((var_c_fox_inv_dn0 * var_t0__blk91) + (var_c_fox_inv * var_t0__blk91_dn0));
        var_t1__blk92_dn2 = ((var_c_fox_inv_dn2 * var_t0__blk91) + (var_c_fox_inv * var_t0__blk91_dn2));
        var_t1__blk92_dn6 = ((var_c_fox_inv_dn6 * var_t0__blk91) + (var_c_fox_inv * var_t0__blk91_dn6));
        var_t1__blk92_dn7 = ((var_c_fox_inv_dn7 * var_t0__blk91) + (var_c_fox_inv * var_t0__blk91_dn7));
        var_t1__blk92_dn10 = ((var_c_fox_inv_dn10 * var_t0__blk91) + (var_c_fox_inv * var_t0__blk91_dn10));
        var_t1__blk92_dn11 = ((var_c_fox_inv_dn11 * var_t0__blk91) + (var_c_fox_inv * var_t0__blk91_dn11));
        var_t1__blk92_dn12 = ((var_c_fox_inv_dn12 * var_t0__blk91) + (var_c_fox_inv * var_t0__blk91_dn12));
        var_t1__blk92_dn17 = ((var_c_fox_inv_dn17 * var_t0__blk91) + (var_c_fox_inv * var_t0__blk91_dn17));
        var_t1__blk92_rv = 0.0;

        let assign6550_e4359: f64 = (p.p69 - var_pb20b);
        var_t2__blk93 = assign6550_e4359;
        var_t2__blk93_dn0 = (-var_pb20b_dn0);
        var_t2__blk93_dn2 = (-var_pb20b_dn2);
        var_t2__blk93_dn6 = (-var_pb20b_dn6);
        var_t2__blk93_dn7 = (-var_pb20b_dn7);
        var_t2__blk93_dn10 = (-var_pb20b_dn10);
        var_t2__blk93_dn11 = (-var_pb20b_dn11);
        var_t2__blk93_dn12 = (-var_pb20b_dn12);
        var_t2__blk93_dn17 = (-var_pb20b_dn17);
        var_t2__blk93_rv = 0.0;

        let assign6560_e4362: f64 = (var_lgleff - p.p71);
        var_t3__blk94 = assign6560_e4362;
        var_t3__blk94_rv = 0.0;

        let assign6570_e4366: f64 = (var_t3__blk94 * var_t3__blk94);
        let assign6570_e4367: f64 = (1.0 / assign6570_e4366);
        var_t4__blk95 = assign6570_e4367;
        var_t4__blk95_dn0 = 0.0;
        var_t4__blk95_dn2 = 0.0;
        var_t4__blk95_dn6 = 0.0;
        var_t4__blk95_dn7 = 0.0;
        var_t4__blk95_dn10 = 0.0;
        var_t4__blk95_dn11 = 0.0;
        var_t4__blk95_dn12 = 0.0;
        var_t4__blk95_dn17 = 0.0;
        var_t4__blk95_rv = 0.0;

        let assign6580_e4370: f64 = (var_t1__blk92 * var_t2__blk93);
        let assign6580_e4372: f64 = (assign6580_e4370 * var_t4__blk95);
        var_dvth0__blk97 = assign6580_e4372;
        var_dvth0__blk97_dn0 = ((((var_t1__blk92_dn0 * var_t2__blk93) + (var_t1__blk92 * var_t2__blk93_dn0)) * var_t4__blk95) + (assign6580_e4370 * var_t4__blk95_dn0));
        var_dvth0__blk97_dn2 = ((((var_t1__blk92_dn2 * var_t2__blk93) + (var_t1__blk92 * var_t2__blk93_dn2)) * var_t4__blk95) + (assign6580_e4370 * var_t4__blk95_dn2));
        var_dvth0__blk97_dn6 = ((((var_t1__blk92_dn6 * var_t2__blk93) + (var_t1__blk92 * var_t2__blk93_dn6)) * var_t4__blk95) + (assign6580_e4370 * var_t4__blk95_dn6));
        var_dvth0__blk97_dn7 = ((((var_t1__blk92_dn7 * var_t2__blk93) + (var_t1__blk92 * var_t2__blk93_dn7)) * var_t4__blk95) + (assign6580_e4370 * var_t4__blk95_dn7));
        var_dvth0__blk97_dn10 = ((((var_t1__blk92_dn10 * var_t2__blk93) + (var_t1__blk92 * var_t2__blk93_dn10)) * var_t4__blk95) + (assign6580_e4370 * var_t4__blk95_dn10));
        var_dvth0__blk97_dn11 = ((((var_t1__blk92_dn11 * var_t2__blk93) + (var_t1__blk92 * var_t2__blk93_dn11)) * var_t4__blk95) + (assign6580_e4370 * var_t4__blk95_dn11));
        var_dvth0__blk97_dn12 = ((((var_t1__blk92_dn12 * var_t2__blk93) + (var_t1__blk92 * var_t2__blk93_dn12)) * var_t4__blk95) + (assign6580_e4370 * var_t4__blk95_dn12));
        var_dvth0__blk97_dn17 = ((((var_t1__blk92_dn17 * var_t2__blk93) + (var_t1__blk92 * var_t2__blk93_dn17)) * var_t4__blk95) + (assign6580_e4370 * var_t4__blk95_dn17));
        var_dvth0__blk97_rv = 0.0;

        let assign6590_e4375: f64 = (var_uc_sc3 / var_lgleff);
        var_t1__blk92 = assign6590_e4375;
        var_t1__blk92_dn0 = 0.0;
        var_t1__blk92_dn2 = 0.0;
        var_t1__blk92_dn6 = 0.0;
        var_t1__blk92_dn7 = 0.0;
        var_t1__blk92_dn10 = 0.0;
        var_t1__blk92_dn11 = 0.0;
        var_t1__blk92_dn12 = 0.0;
        var_t1__blk92_dn17 = 0.0;
        var_t1__blk92_rv = 0.0;

        let assign6600_e4379: f64 = (var_t1__blk92 * var_pbsum);
        let assign6600_e4380: f64 = (p.p83 + assign6600_e4379);
        var_t4__blk95 = assign6600_e4380;
        var_t4__blk95_dn0 = ((var_t1__blk92_dn0 * var_pbsum) + (var_t1__blk92 * var_pbsum_dn0));
        var_t4__blk95_dn2 = ((var_t1__blk92_dn2 * var_pbsum) + (var_t1__blk92 * var_pbsum_dn2));
        var_t4__blk95_dn6 = ((var_t1__blk92_dn6 * var_pbsum) + (var_t1__blk92 * var_pbsum_dn6));
        var_t4__blk95_dn7 = ((var_t1__blk92_dn7 * var_pbsum) + (var_t1__blk92 * var_pbsum_dn7));
        var_t4__blk95_dn10 = ((var_t1__blk92_dn10 * var_pbsum) + (var_t1__blk92 * var_pbsum_dn10));
        var_t4__blk95_dn11 = ((var_t1__blk92_dn11 * var_pbsum) + (var_t1__blk92 * var_pbsum_dn11));
        var_t4__blk95_dn12 = ((var_t1__blk92_dn12 * var_pbsum) + (var_t1__blk92 * var_pbsum_dn12));
        var_t4__blk95_dn17 = ((var_t1__blk92_dn17 * var_pbsum) + (var_t1__blk92 * var_pbsum_dn17));
        var_t4__blk95_rv = 0.0;

        let assign6610_e4384: f64 = (var_uc_sc2 * var_vdsz);
        let assign6610_e4385: f64 = (var_t4__blk95 + assign6610_e4384);
        var_t5__blk96 = assign6610_e4385;
        var_t5__blk96_dn0 = (var_t4__blk95_dn0 + (var_uc_sc2 * var_vdsz_dn0));
        var_t5__blk96_dn2 = (var_t4__blk95_dn2 + (var_uc_sc2 * var_vdsz_dn2));
        var_t5__blk96_dn6 = (var_t4__blk95_dn6 + (var_uc_sc2 * var_vdsz_dn6));
        var_t5__blk96_dn7 = (var_t4__blk95_dn7 + (var_uc_sc2 * var_vdsz_dn7));
        var_t5__blk96_dn10 = (var_t4__blk95_dn10 + (var_uc_sc2 * var_vdsz_dn10));
        var_t5__blk96_dn11 = (var_t4__blk95_dn11 + (var_uc_sc2 * var_vdsz_dn11));
        var_t5__blk96_dn12 = (var_t4__blk95_dn12 + (var_uc_sc2 * var_vdsz_dn12));
        var_t5__blk96_dn17 = (var_t4__blk95_dn17 + (var_uc_sc2 * var_vdsz_dn17));
        var_t5__blk96_rv = 0.0;

        let assign6620_e4388: f64 = (var_dvth0__blk97 * var_t5__blk96);
        var_dvthsc = assign6620_e4388;
        var_dvthsc_dn0 = ((var_dvth0__blk97_dn0 * var_t5__blk96) + (var_dvth0__blk97 * var_t5__blk96_dn0));
        var_dvthsc_dn2 = ((var_dvth0__blk97_dn2 * var_t5__blk96) + (var_dvth0__blk97 * var_t5__blk96_dn2));
        var_dvthsc_dn6 = ((var_dvth0__blk97_dn6 * var_t5__blk96) + (var_dvth0__blk97 * var_t5__blk96_dn6));
        var_dvthsc_dn7 = ((var_dvth0__blk97_dn7 * var_t5__blk96) + (var_dvth0__blk97 * var_t5__blk96_dn7));
        var_dvthsc_dn10 = ((var_dvth0__blk97_dn10 * var_t5__blk96) + (var_dvth0__blk97 * var_t5__blk96_dn10));
        var_dvthsc_dn11 = ((var_dvth0__blk97_dn11 * var_t5__blk96) + (var_dvth0__blk97 * var_t5__blk96_dn11));
        var_dvthsc_dn12 = ((var_dvth0__blk97_dn12 * var_t5__blk96) + (var_dvth0__blk97 * var_t5__blk96_dn12));
        var_dvthsc_dn17 = ((var_dvth0__blk97_dn17 * var_t5__blk96) + (var_dvth0__blk97 * var_t5__blk96_dn17));
        var_dvthsc_rv = 0.0;

        let assign6630_e4391: f64 = if p.p86 > 0.0 { 1.0 } else { 0.0 };
        var_guard101 = assign6630_e4391;
        var_guard101_rv = 0.0;

        let (assign6640_e4405, assign6640_e4405_d_n0, assign6640_e4405_d_n2, assign6640_e4405_d_n6, assign6640_e4405_d_n7, assign6640_e4405_d_n10, assign6640_e4405_d_n11, assign6640_e4405_d_n12, assign6640_e4405_d_n17,) = {
    if (var_guard101 != 0.0) {
        let assign6640_e4395: f64 = (var_eg + var_pb2);
        let assign6640_e4398: f64 = (2.0 * p.p88);
        let assign6640_e4399: f64 = (assign6640_e4395 - assign6640_e4398);
        let assign6640_e4402: f64 = (p.p87 * var_vdsz);
        let assign6640_e4403: f64 = (assign6640_e4399 + assign6640_e4402);
        (assign6640_e4403, ((var_eg_dn0 + var_pb2_dn0) + (p.p87 * var_vdsz_dn0)), ((var_eg_dn2 + var_pb2_dn2) + (p.p87 * var_vdsz_dn2)), ((var_eg_dn6 + var_pb2_dn6) + (p.p87 * var_vdsz_dn6)), ((var_eg_dn7 + var_pb2_dn7) + (p.p87 * var_vdsz_dn7)), ((var_eg_dn10 + var_pb2_dn10) + (p.p87 * var_vdsz_dn10)), ((var_eg_dn11 + var_pb2_dn11) + (p.p87 * var_vdsz_dn11)), ((var_eg_dn12 + var_pb2_dn12) + (p.p87 * var_vdsz_dn12)), ((var_eg_dn17 + var_pb2_dn17) + (p.p87 * var_vdsz_dn17)),)
    } else {
        (var_t1__blk98, var_t1__blk98_dn0, var_t1__blk98_dn2, var_t1__blk98_dn6, var_t1__blk98_dn7, var_t1__blk98_dn10, var_t1__blk98_dn11, var_t1__blk98_dn12, var_t1__blk98_dn17,)
    }
};
        var_t1__blk98 = assign6640_e4405;
        var_t1__blk98_dn0 = assign6640_e4405_d_n0;
        var_t1__blk98_dn2 = assign6640_e4405_d_n2;
        var_t1__blk98_dn6 = assign6640_e4405_d_n6;
        var_t1__blk98_dn7 = assign6640_e4405_d_n7;
        var_t1__blk98_dn10 = assign6640_e4405_d_n10;
        var_t1__blk98_dn11 = assign6640_e4405_d_n11;
        var_t1__blk98_dn12 = assign6640_e4405_d_n12;
        var_t1__blk98_dn17 = assign6640_e4405_d_n17;
        var_t1__blk98_rv = 0.0;

        let (assign6650_e4413,) = {
    if (var_guard101 != 0.0) {
        let assign6650_e4409: f64 = (var_lgleff * 0.5);
        let assign6650_e4411: f64 = (assign6650_e4409 + var_mks_parl1);
        (assign6650_e4411,)
    } else {
        (var_t2__blk99,)
    }
};
        var_t2__blk99 = assign6650_e4413;
        var_t2__blk99_rv = 0.0;

        let (assign6660_e4421,) = {
    if (var_guard101 != 0.0) {
        let assign6660_e4417: f64 = (p.p86 * p.p237);
        let assign6660_e4419: f64 = (assign6660_e4417 / var_t2__blk99);
        (assign6660_e4419,)
    } else {
        (var_t3__blk100,)
    }
};
        var_t3__blk100 = assign6660_e4421;
        var_t3__blk100_rv = 0.0;

        let (assign6670_e4427, assign6670_e4427_d_n0, assign6670_e4427_d_n2, assign6670_e4427_d_n6, assign6670_e4427_d_n7, assign6670_e4427_d_n10, assign6670_e4427_d_n11, assign6670_e4427_d_n12, assign6670_e4427_d_n17,) = {
    if (var_guard101 != 0.0) {
        let assign6670_e4425: f64 = (var_t1__blk98 * var_t3__blk100);
        (assign6670_e4425, (var_t1__blk98_dn0 * var_t3__blk100), (var_t1__blk98_dn2 * var_t3__blk100), (var_t1__blk98_dn6 * var_t3__blk100), (var_t1__blk98_dn7 * var_t3__blk100), (var_t1__blk98_dn10 * var_t3__blk100), (var_t1__blk98_dn11 * var_t3__blk100), (var_t1__blk98_dn12 * var_t3__blk100), (var_t1__blk98_dn17 * var_t3__blk100),)
    } else {
        (var_dvthscr, var_dvthscr_dn0, var_dvthscr_dn2, var_dvthscr_dn6, var_dvthscr_dn7, var_dvthscr_dn10, var_dvthscr_dn11, var_dvthscr_dn12, var_dvthscr_dn17,)
    }
};
        var_dvthscr = assign6670_e4427;
        var_dvthscr_dn0 = assign6670_e4427_d_n0;
        var_dvthscr_dn2 = assign6670_e4427_d_n2;
        var_dvthscr_dn6 = assign6670_e4427_d_n6;
        var_dvthscr_dn7 = assign6670_e4427_d_n7;
        var_dvthscr_dn10 = assign6670_e4427_d_n10;
        var_dvthscr_dn11 = assign6670_e4427_d_n11;
        var_dvthscr_dn12 = assign6670_e4427_d_n12;
        var_dvthscr_dn17 = assign6670_e4427_d_n17;
        var_dvthscr_rv = 0.0;

        let (assign6680_e4432, assign6680_e4432_d_n0, assign6680_e4432_d_n2, assign6680_e4432_d_n6, assign6680_e4432_d_n7, assign6680_e4432_d_n10, assign6680_e4432_d_n11, assign6680_e4432_d_n12, assign6680_e4432_d_n17,) = {
    if (var_guard101 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dvthscr, var_dvthscr_dn0, var_dvthscr_dn2, var_dvthscr_dn6, var_dvthscr_dn7, var_dvthscr_dn10, var_dvthscr_dn11, var_dvthscr_dn12, var_dvthscr_dn17,)
    }
};
        var_dvthscr = assign6680_e4432;
        var_dvthscr_dn0 = assign6680_e4432_d_n0;
        var_dvthscr_dn2 = assign6680_e4432_d_n2;
        var_dvthscr_dn6 = assign6680_e4432_d_n6;
        var_dvthscr_dn7 = assign6680_e4432_d_n7;
        var_dvthscr_dn10 = assign6680_e4432_d_n10;
        var_dvthscr_dn11 = assign6680_e4432_d_n11;
        var_dvthscr_dn12 = assign6680_e4432_d_n12;
        var_dvthscr_dn17 = assign6680_e4432_d_n17;
        var_dvthscr_rv = 0.0;

        var_t1__blk102 = var_c_fox_inv;
        var_t1__blk102_dn0 = var_c_fox_inv_dn0;
        var_t1__blk102_dn2 = var_c_fox_inv_dn2;
        var_t1__blk102_dn6 = var_c_fox_inv_dn6;
        var_t1__blk102_dn7 = var_c_fox_inv_dn7;
        var_t1__blk102_dn10 = var_c_fox_inv_dn10;
        var_t1__blk102_dn11 = var_c_fox_inv_dn11;
        var_t1__blk102_dn12 = var_c_fox_inv_dn12;
        var_t1__blk102_dn17 = var_c_fox_inv_dn17;
        var_t1__blk102_rv = 0.0;

        let assign6700_e4438: f64 = (var_mks_wfc / var_weff);
        let assign6700_e4439: f64 = (var_c_fox + assign6700_e4438);
        let assign6700_e4440: f64 = (1.0 / assign6700_e4439);
        var_t3__blk103 = assign6700_e4440;
        var_t3__blk103_dn0 = (-(var_c_fox_dn0 / (assign6700_e4439 * assign6700_e4439)));
        var_t3__blk103_dn2 = (-(var_c_fox_dn2 / (assign6700_e4439 * assign6700_e4439)));
        var_t3__blk103_dn6 = (-(var_c_fox_dn6 / (assign6700_e4439 * assign6700_e4439)));
        var_t3__blk103_dn7 = (-(var_c_fox_dn7 / (assign6700_e4439 * assign6700_e4439)));
        var_t3__blk103_dn10 = (-(var_c_fox_dn10 / (assign6700_e4439 * assign6700_e4439)));
        var_t3__blk103_dn11 = (-(var_c_fox_dn11 / (assign6700_e4439 * assign6700_e4439)));
        var_t3__blk103_dn12 = (-(var_c_fox_dn12 / (assign6700_e4439 * assign6700_e4439)));
        var_t3__blk103_dn17 = (-(var_c_fox_dn17 / (assign6700_e4439 * assign6700_e4439)));
        var_t3__blk103_rv = 0.0;

        let assign6710_e4443: f64 = (var_t1__blk102 - var_t3__blk103);
        var_t5__blk104 = assign6710_e4443;
        var_t5__blk104_dn0 = (var_t1__blk102_dn0 - var_t3__blk103_dn0);
        var_t5__blk104_dn2 = (var_t1__blk102_dn2 - var_t3__blk103_dn2);
        var_t5__blk104_dn6 = (var_t1__blk102_dn6 - var_t3__blk103_dn6);
        var_t5__blk104_dn7 = (var_t1__blk102_dn7 - var_t3__blk103_dn7);
        var_t5__blk104_dn10 = (var_t1__blk102_dn10 - var_t3__blk103_dn10);
        var_t5__blk104_dn11 = (var_t1__blk102_dn11 - var_t3__blk103_dn11);
        var_t5__blk104_dn12 = (var_t1__blk102_dn12 - var_t3__blk103_dn12);
        var_t5__blk104_dn17 = (var_t1__blk102_dn17 - var_t3__blk103_dn17);
        var_t5__blk104_rv = 0.0;

        let assign6720_e4446: f64 = (var_qb0 * var_t5__blk104);
        let assign6720_e4449: f64 = (p.p105 / var_wg);
        let assign6720_e4450: f64 = (assign6720_e4446 + assign6720_e4449);
        var_dvthw = assign6720_e4450;
        var_dvthw_dn0 = ((var_qb0_dn0 * var_t5__blk104) + (var_qb0 * var_t5__blk104_dn0));
        var_dvthw_dn2 = ((var_qb0_dn2 * var_t5__blk104) + (var_qb0 * var_t5__blk104_dn2));
        var_dvthw_dn6 = ((var_qb0_dn6 * var_t5__blk104) + (var_qb0 * var_t5__blk104_dn6));
        var_dvthw_dn7 = ((var_qb0_dn7 * var_t5__blk104) + (var_qb0 * var_t5__blk104_dn7));
        var_dvthw_dn10 = ((var_qb0_dn10 * var_t5__blk104) + (var_qb0 * var_t5__blk104_dn10));
        var_dvthw_dn11 = ((var_qb0_dn11 * var_t5__blk104) + (var_qb0 * var_t5__blk104_dn11));
        var_dvthw_dn12 = ((var_qb0_dn12 * var_t5__blk104) + (var_qb0 * var_t5__blk104_dn12));
        var_dvthw_dn17 = ((var_qb0_dn17 * var_t5__blk104) + (var_qb0 * var_t5__blk104_dn17));
        var_dvthw_rv = 0.0;

        let assign6730_e4453: f64 = (var_dvthsc + var_dvthlp);
        let assign6730_e4455: f64 = (assign6730_e4453 + var_dvthw);
        let assign6730_e4457: f64 = (assign6730_e4455 + var_dvthscr);
        let assign6730_e4459: f64 = (assign6730_e4457 + var_dvthsm);
        var_dvth = assign6730_e4459;
        var_dvth_dn0 = (((var_dvthsc_dn0 + var_dvthlp_dn0) + var_dvthw_dn0) + var_dvthscr_dn0);
        var_dvth_dn2 = (((var_dvthsc_dn2 + var_dvthlp_dn2) + var_dvthw_dn2) + var_dvthscr_dn2);
        var_dvth_dn6 = (((var_dvthsc_dn6 + var_dvthlp_dn6) + var_dvthw_dn6) + var_dvthscr_dn6);
        var_dvth_dn7 = (((var_dvthsc_dn7 + var_dvthlp_dn7) + var_dvthw_dn7) + var_dvthscr_dn7);
        var_dvth_dn10 = (((var_dvthsc_dn10 + var_dvthlp_dn10) + var_dvthw_dn10) + var_dvthscr_dn10);
        var_dvth_dn11 = (((var_dvthsc_dn11 + var_dvthlp_dn11) + var_dvthw_dn11) + var_dvthscr_dn11);
        var_dvth_dn12 = (((var_dvthsc_dn12 + var_dvthlp_dn12) + var_dvthw_dn12) + var_dvthscr_dn12);
        var_dvth_dn17 = (((var_dvthsc_dn17 + var_dvthlp_dn17) + var_dvthw_dn17) + var_dvthscr_dn17);
        var_dvth_rv = 0.0;

        let assign6740_e4462: f64 = (var_vthp - var_dvth);
        var_vth = assign6740_e4462;
        var_vth_dn0 = (var_vthp_dn0 - var_dvth_dn0);
        var_vth_dn2 = (var_vthp_dn2 - var_dvth_dn2);
        var_vth_dn6 = (var_vthp_dn6 - var_dvth_dn6);
        var_vth_dn7 = (var_vthp_dn7 - var_dvth_dn7);
        var_vth_dn10 = (var_vthp_dn10 - var_dvth_dn10);
        var_vth_dn11 = (var_vthp_dn11 - var_dvth_dn11);
        var_vth_dn12 = (var_vthp_dn12 - var_dvth_dn12);
        var_vth_dn17 = (var_vthp_dn17 - var_dvth_dn17);
        var_vth_rv = 0.0;

        let assign6750_e4465: f64 = if p.p89 == 0.0 { 1.0 } else { 0.0 };
        var_guard108 = assign6750_e4465;
        var_guard108_rv = 0.0;

        let (assign6760_e4469,) = {
    if (var_guard108 != 0.0) {
        (0.0,)
    } else {
        (var_flg_dppg,)
    }
};
        var_flg_dppg = assign6760_e4469;
        var_flg_dppg_rv = 0.0;

        let (assign6770_e4474,) = {
    if (var_guard108 == 0.0) {
        (1.0,)
    } else {
        (var_flg_dppg,)
    }
};
        var_flg_dppg = assign6770_e4474;
        var_flg_dppg_rv = 0.0;

        let assign6780_e4477: f64 = if var_flg_dppg == 0.0 { 1.0 } else { 0.0 };
        var_guard109 = assign6780_e4477;
        var_guard109_rv = 0.0;

        let (assign6790_e4481, assign6790_e4481_d_n0, assign6790_e4481_d_n2, assign6790_e4481_d_n6, assign6790_e4481_d_n7, assign6790_e4481_d_n10, assign6790_e4481_d_n11, assign6790_e4481_d_n12, assign6790_e4481_d_n17,) = {
    if (var_guard109 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6790_e4481;
        var_dppg_dn0 = assign6790_e4481_d_n0;
        var_dppg_dn2 = assign6790_e4481_d_n2;
        var_dppg_dn6 = assign6790_e4481_d_n6;
        var_dppg_dn7 = assign6790_e4481_d_n7;
        var_dppg_dn10 = assign6790_e4481_d_n10;
        var_dppg_dn11 = assign6790_e4481_d_n11;
        var_dppg_dn12 = assign6790_e4481_d_n12;
        var_dppg_dn17 = assign6790_e4481_d_n17;
        var_dppg_rv = 0.0;

        let (assign6800_e4486, assign6800_e4486_d_n0, assign6800_e4486_d_n2, assign6800_e4486_d_n6, assign6800_e4486_d_n7, assign6800_e4486_d_n10, assign6800_e4486_d_n11, assign6800_e4486_d_n12, assign6800_e4486_d_n17,) = {
    if (var_guard109 == 0.0) {
        (var_vgsz, var_vgsz_dn0, var_vgsz_dn2, var_vgsz_dn6, var_vgsz_dn7, var_vgsz_dn10, var_vgsz_dn11, var_vgsz_dn12, var_vgsz_dn17,)
    } else {
        (var_t7__blk105, var_t7__blk105_dn0, var_t7__blk105_dn2, var_t7__blk105_dn6, var_t7__blk105_dn7, var_t7__blk105_dn10, var_t7__blk105_dn11, var_t7__blk105_dn12, var_t7__blk105_dn17,)
    }
};
        var_t7__blk105 = assign6800_e4486;
        var_t7__blk105_dn0 = assign6800_e4486_d_n0;
        var_t7__blk105_dn2 = assign6800_e4486_d_n2;
        var_t7__blk105_dn6 = assign6800_e4486_d_n6;
        var_t7__blk105_dn7 = assign6800_e4486_d_n7;
        var_t7__blk105_dn10 = assign6800_e4486_d_n10;
        var_t7__blk105_dn11 = assign6800_e4486_d_n11;
        var_t7__blk105_dn12 = assign6800_e4486_d_n12;
        var_t7__blk105_dn17 = assign6800_e4486_d_n17;
        var_t7__blk105_rv = 0.0;

        let (assign6810_e4491,) = {
    if (var_guard109 == 0.0) {
        (var_cnstpgd,)
    } else {
        (var_t0__blk106,)
    }
};
        var_t0__blk106 = assign6810_e4491;
        var_t0__blk106_rv = 0.0;

        let (assign6820_e4498, assign6820_e4498_d_n0, assign6820_e4498_d_n2, assign6820_e4498_d_n6, assign6820_e4498_d_n7, assign6820_e4498_d_n10, assign6820_e4498_d_n11, assign6820_e4498_d_n12, assign6820_e4498_d_n17,) = {
    if (var_guard109 == 0.0) {
        let assign6820_e4496: f64 = (var_t7__blk105 - p.p90);
        (assign6820_e4496, var_t7__blk105_dn0, var_t7__blk105_dn2, var_t7__blk105_dn6, var_t7__blk105_dn7, var_t7__blk105_dn10, var_t7__blk105_dn11, var_t7__blk105_dn12, var_t7__blk105_dn17,)
    } else {
        (var_t3__blk107, var_t3__blk107_dn0, var_t3__blk107_dn2, var_t3__blk107_dn6, var_t3__blk107_dn7, var_t3__blk107_dn10, var_t3__blk107_dn11, var_t3__blk107_dn12, var_t3__blk107_dn17,)
    }
};
        var_t3__blk107 = assign6820_e4498;
        var_t3__blk107_dn0 = assign6820_e4498_d_n0;
        var_t3__blk107_dn2 = assign6820_e4498_d_n2;
        var_t3__blk107_dn6 = assign6820_e4498_d_n6;
        var_t3__blk107_dn7 = assign6820_e4498_d_n7;
        var_t3__blk107_dn10 = assign6820_e4498_d_n10;
        var_t3__blk107_dn11 = assign6820_e4498_d_n11;
        var_t3__blk107_dn12 = assign6820_e4498_d_n12;
        var_t3__blk107_dn17 = assign6820_e4498_d_n17;
        var_t3__blk107_rv = 0.0;

        let assign6830_e4501: f64 = (-3.0);
        let assign6830_e4502: f64 = if var_t3__blk107 < assign6830_e4501 { 1.0 } else { 0.0 };
        var_guard110 = assign6830_e4502;
        var_guard110_rv = 0.0;

        let (assign6840_e4509, assign6840_e4509_d_n0, assign6840_e4509_d_n2, assign6840_e4509_d_n6, assign6840_e4509_d_n7, assign6840_e4509_d_n10, assign6840_e4509_d_n11, assign6840_e4509_d_n12, assign6840_e4509_d_n17,) = {
    if ((var_guard109 == 0.0) && (var_guard110 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6840_e4509;
        var_dppg_dn0 = assign6840_e4509_d_n0;
        var_dppg_dn2 = assign6840_e4509_d_n2;
        var_dppg_dn6 = assign6840_e4509_d_n6;
        var_dppg_dn7 = assign6840_e4509_d_n7;
        var_dppg_dn10 = assign6840_e4509_d_n10;
        var_dppg_dn11 = assign6840_e4509_d_n11;
        var_dppg_dn12 = assign6840_e4509_d_n12;
        var_dppg_dn17 = assign6840_e4509_d_n17;
        var_dppg_rv = 0.0;

        let assign6850_e4512: f64 = if var_t3__blk107 < 0.0 { 1.0 } else { 0.0 };
        var_guard111 = assign6850_e4512;
        var_guard111_rv = 0.0;

        let (assign6860_e4538, assign6860_e4538_d_n0, assign6860_e4538_d_n2, assign6860_e4538_d_n6, assign6860_e4538_d_n7, assign6860_e4538_d_n10, assign6860_e4538_d_n11, assign6860_e4538_d_n12, assign6860_e4538_d_n17,) = {
    if (((var_guard109 == 0.0) && (var_guard110 == 0.0)) && (var_guard111 != 0.0)) {
        let assign6860_e4526: f64 = (1.0 / 3.0);
        let assign6860_e4530: f64 = (1.0 / 27.0);
        let assign6860_e4531: f64 = (var_t3__blk107 * assign6860_e4530);
        let assign6860_e4532: f64 = (assign6860_e4526 + assign6860_e4531);
        let assign6860_e4533: f64 = (var_t3__blk107 * assign6860_e4532);
        let assign6860_e4534: f64 = (1.0 + assign6860_e4533);
        let assign6860_e4535: f64 = (var_t3__blk107 * assign6860_e4534);
        let assign6860_e4536: f64 = (1.0 + assign6860_e4535);
        (assign6860_e4536, ((var_t3__blk107_dn0 * assign6860_e4534) + (var_t3__blk107 * ((var_t3__blk107_dn0 * assign6860_e4532) + (var_t3__blk107 * (var_t3__blk107_dn0 * assign6860_e4530))))), ((var_t3__blk107_dn2 * assign6860_e4534) + (var_t3__blk107 * ((var_t3__blk107_dn2 * assign6860_e4532) + (var_t3__blk107 * (var_t3__blk107_dn2 * assign6860_e4530))))), ((var_t3__blk107_dn6 * assign6860_e4534) + (var_t3__blk107 * ((var_t3__blk107_dn6 * assign6860_e4532) + (var_t3__blk107 * (var_t3__blk107_dn6 * assign6860_e4530))))), ((var_t3__blk107_dn7 * assign6860_e4534) + (var_t3__blk107 * ((var_t3__blk107_dn7 * assign6860_e4532) + (var_t3__blk107 * (var_t3__blk107_dn7 * assign6860_e4530))))), ((var_t3__blk107_dn10 * assign6860_e4534) + (var_t3__blk107 * ((var_t3__blk107_dn10 * assign6860_e4532) + (var_t3__blk107 * (var_t3__blk107_dn10 * assign6860_e4530))))), ((var_t3__blk107_dn11 * assign6860_e4534) + (var_t3__blk107 * ((var_t3__blk107_dn11 * assign6860_e4532) + (var_t3__blk107 * (var_t3__blk107_dn11 * assign6860_e4530))))), ((var_t3__blk107_dn12 * assign6860_e4534) + (var_t3__blk107 * ((var_t3__blk107_dn12 * assign6860_e4532) + (var_t3__blk107 * (var_t3__blk107_dn12 * assign6860_e4530))))), ((var_t3__blk107_dn17 * assign6860_e4534) + (var_t3__blk107 * ((var_t3__blk107_dn17 * assign6860_e4532) + (var_t3__blk107 * (var_t3__blk107_dn17 * assign6860_e4530))))),)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6860_e4538;
        var_dppg_dn0 = assign6860_e4538_d_n0;
        var_dppg_dn2 = assign6860_e4538_d_n2;
        var_dppg_dn6 = assign6860_e4538_d_n6;
        var_dppg_dn7 = assign6860_e4538_d_n7;
        var_dppg_dn10 = assign6860_e4538_d_n10;
        var_dppg_dn11 = assign6860_e4538_d_n11;
        var_dppg_dn12 = assign6860_e4538_d_n12;
        var_dppg_dn17 = assign6860_e4538_d_n17;
        var_dppg_rv = 0.0;

        *var_dppg_slot = var_dppg;
        *var_dppg_dn0_slot = var_dppg_dn0;
        *var_dppg_dn10_slot = var_dppg_dn10;
        *var_dppg_dn11_slot = var_dppg_dn11;
        *var_dppg_dn12_slot = var_dppg_dn12;
        *var_dppg_dn17_slot = var_dppg_dn17;
        *var_dppg_dn2_slot = var_dppg_dn2;
        *var_dppg_dn6_slot = var_dppg_dn6;
        *var_dppg_dn7_slot = var_dppg_dn7;
        *var_dppg_rv_slot = var_dppg_rv;
        *var_dvth_slot = var_dvth;
        *var_dvth0__blk97_slot = var_dvth0__blk97;
        *var_dvth0__blk97_dn0_slot = var_dvth0__blk97_dn0;
        *var_dvth0__blk97_dn10_slot = var_dvth0__blk97_dn10;
        *var_dvth0__blk97_dn11_slot = var_dvth0__blk97_dn11;
        *var_dvth0__blk97_dn12_slot = var_dvth0__blk97_dn12;
        *var_dvth0__blk97_dn17_slot = var_dvth0__blk97_dn17;
        *var_dvth0__blk97_dn2_slot = var_dvth0__blk97_dn2;
        *var_dvth0__blk97_dn6_slot = var_dvth0__blk97_dn6;
        *var_dvth0__blk97_dn7_slot = var_dvth0__blk97_dn7;
        *var_dvth0__blk97_rv_slot = var_dvth0__blk97_rv;
        *var_dvth_dn0_slot = var_dvth_dn0;
        *var_dvth_dn10_slot = var_dvth_dn10;
        *var_dvth_dn11_slot = var_dvth_dn11;
        *var_dvth_dn12_slot = var_dvth_dn12;
        *var_dvth_dn17_slot = var_dvth_dn17;
        *var_dvth_dn2_slot = var_dvth_dn2;
        *var_dvth_dn6_slot = var_dvth_dn6;
        *var_dvth_dn7_slot = var_dvth_dn7;
        *var_dvth_rv_slot = var_dvth_rv;
        *var_dvthlp_slot = var_dvthlp;
        *var_dvthlp_dn0_slot = var_dvthlp_dn0;
        *var_dvthlp_dn10_slot = var_dvthlp_dn10;
        *var_dvthlp_dn11_slot = var_dvthlp_dn11;
        *var_dvthlp_dn12_slot = var_dvthlp_dn12;
        *var_dvthlp_dn17_slot = var_dvthlp_dn17;
        *var_dvthlp_dn2_slot = var_dvthlp_dn2;
        *var_dvthlp_dn6_slot = var_dvthlp_dn6;
        *var_dvthlp_dn7_slot = var_dvthlp_dn7;
        *var_dvthlp_rv_slot = var_dvthlp_rv;
        *var_dvthsc_slot = var_dvthsc;
        *var_dvthsc_dn0_slot = var_dvthsc_dn0;
        *var_dvthsc_dn10_slot = var_dvthsc_dn10;
        *var_dvthsc_dn11_slot = var_dvthsc_dn11;
        *var_dvthsc_dn12_slot = var_dvthsc_dn12;
        *var_dvthsc_dn17_slot = var_dvthsc_dn17;
        *var_dvthsc_dn2_slot = var_dvthsc_dn2;
        *var_dvthsc_dn6_slot = var_dvthsc_dn6;
        *var_dvthsc_dn7_slot = var_dvthsc_dn7;
        *var_dvthsc_rv_slot = var_dvthsc_rv;
        *var_dvthscr_slot = var_dvthscr;
        *var_dvthscr_dn0_slot = var_dvthscr_dn0;
        *var_dvthscr_dn10_slot = var_dvthscr_dn10;
        *var_dvthscr_dn11_slot = var_dvthscr_dn11;
        *var_dvthscr_dn12_slot = var_dvthscr_dn12;
        *var_dvthscr_dn17_slot = var_dvthscr_dn17;
        *var_dvthscr_dn2_slot = var_dvthscr_dn2;
        *var_dvthscr_dn6_slot = var_dvthscr_dn6;
        *var_dvthscr_dn7_slot = var_dvthscr_dn7;
        *var_dvthscr_rv_slot = var_dvthscr_rv;
        *var_dvthw_slot = var_dvthw;
        *var_dvthw_dn0_slot = var_dvthw_dn0;
        *var_dvthw_dn10_slot = var_dvthw_dn10;
        *var_dvthw_dn11_slot = var_dvthw_dn11;
        *var_dvthw_dn12_slot = var_dvthw_dn12;
        *var_dvthw_dn17_slot = var_dvthw_dn17;
        *var_dvthw_dn2_slot = var_dvthw_dn2;
        *var_dvthw_dn6_slot = var_dvthw_dn6;
        *var_dvthw_dn7_slot = var_dvthw_dn7;
        *var_dvthw_rv_slot = var_dvthw_rv;
        *var_flg_dppg_slot = var_flg_dppg;
        *var_flg_dppg_rv_slot = var_flg_dppg_rv;
        *var_guard101_slot = var_guard101;
        *var_guard101_rv_slot = var_guard101_rv;
        *var_guard108_slot = var_guard108;
        *var_guard108_rv_slot = var_guard108_rv;
        *var_guard109_slot = var_guard109;
        *var_guard109_rv_slot = var_guard109_rv;
        *var_guard110_slot = var_guard110;
        *var_guard110_rv_slot = var_guard110_rv;
        *var_guard111_slot = var_guard111;
        *var_guard111_rv_slot = var_guard111_rv;
        *var_t0__blk106_slot = var_t0__blk106;
        *var_t0__blk106_rv_slot = var_t0__blk106_rv;
        *var_t0__blk91_slot = var_t0__blk91;
        *var_t0__blk91_dn0_slot = var_t0__blk91_dn0;
        *var_t0__blk91_dn10_slot = var_t0__blk91_dn10;
        *var_t0__blk91_dn11_slot = var_t0__blk91_dn11;
        *var_t0__blk91_dn12_slot = var_t0__blk91_dn12;
        *var_t0__blk91_dn17_slot = var_t0__blk91_dn17;
        *var_t0__blk91_dn2_slot = var_t0__blk91_dn2;
        *var_t0__blk91_dn6_slot = var_t0__blk91_dn6;
        *var_t0__blk91_dn7_slot = var_t0__blk91_dn7;
        *var_t0__blk91_rv_slot = var_t0__blk91_rv;
        *var_t1__blk102_slot = var_t1__blk102;
        *var_t1__blk102_dn0_slot = var_t1__blk102_dn0;
        *var_t1__blk102_dn10_slot = var_t1__blk102_dn10;
        *var_t1__blk102_dn11_slot = var_t1__blk102_dn11;
        *var_t1__blk102_dn12_slot = var_t1__blk102_dn12;
        *var_t1__blk102_dn17_slot = var_t1__blk102_dn17;
        *var_t1__blk102_dn2_slot = var_t1__blk102_dn2;
        *var_t1__blk102_dn6_slot = var_t1__blk102_dn6;
        *var_t1__blk102_dn7_slot = var_t1__blk102_dn7;
        *var_t1__blk102_rv_slot = var_t1__blk102_rv;
        *var_t1__blk92_slot = var_t1__blk92;
        *var_t1__blk92_dn0_slot = var_t1__blk92_dn0;
        *var_t1__blk92_dn10_slot = var_t1__blk92_dn10;
        *var_t1__blk92_dn11_slot = var_t1__blk92_dn11;
        *var_t1__blk92_dn12_slot = var_t1__blk92_dn12;
        *var_t1__blk92_dn17_slot = var_t1__blk92_dn17;
        *var_t1__blk92_dn2_slot = var_t1__blk92_dn2;
        *var_t1__blk92_dn6_slot = var_t1__blk92_dn6;
        *var_t1__blk92_dn7_slot = var_t1__blk92_dn7;
        *var_t1__blk92_rv_slot = var_t1__blk92_rv;
        *var_t1__blk98_slot = var_t1__blk98;
        *var_t1__blk98_dn0_slot = var_t1__blk98_dn0;
        *var_t1__blk98_dn10_slot = var_t1__blk98_dn10;
        *var_t1__blk98_dn11_slot = var_t1__blk98_dn11;
        *var_t1__blk98_dn12_slot = var_t1__blk98_dn12;
        *var_t1__blk98_dn17_slot = var_t1__blk98_dn17;
        *var_t1__blk98_dn2_slot = var_t1__blk98_dn2;
        *var_t1__blk98_dn6_slot = var_t1__blk98_dn6;
        *var_t1__blk98_dn7_slot = var_t1__blk98_dn7;
        *var_t1__blk98_rv_slot = var_t1__blk98_rv;
        *var_t2__blk93_slot = var_t2__blk93;
        *var_t2__blk93_dn0_slot = var_t2__blk93_dn0;
        *var_t2__blk93_dn10_slot = var_t2__blk93_dn10;
        *var_t2__blk93_dn11_slot = var_t2__blk93_dn11;
        *var_t2__blk93_dn12_slot = var_t2__blk93_dn12;
        *var_t2__blk93_dn17_slot = var_t2__blk93_dn17;
        *var_t2__blk93_dn2_slot = var_t2__blk93_dn2;
        *var_t2__blk93_dn6_slot = var_t2__blk93_dn6;
        *var_t2__blk93_dn7_slot = var_t2__blk93_dn7;
        *var_t2__blk93_rv_slot = var_t2__blk93_rv;
        *var_t2__blk99_slot = var_t2__blk99;
        *var_t2__blk99_rv_slot = var_t2__blk99_rv;
        *var_t3__blk100_slot = var_t3__blk100;
        *var_t3__blk100_rv_slot = var_t3__blk100_rv;
        *var_t3__blk103_slot = var_t3__blk103;
        *var_t3__blk103_dn0_slot = var_t3__blk103_dn0;
        *var_t3__blk103_dn10_slot = var_t3__blk103_dn10;
        *var_t3__blk103_dn11_slot = var_t3__blk103_dn11;
        *var_t3__blk103_dn12_slot = var_t3__blk103_dn12;
        *var_t3__blk103_dn17_slot = var_t3__blk103_dn17;
        *var_t3__blk103_dn2_slot = var_t3__blk103_dn2;
        *var_t3__blk103_dn6_slot = var_t3__blk103_dn6;
        *var_t3__blk103_dn7_slot = var_t3__blk103_dn7;
        *var_t3__blk103_rv_slot = var_t3__blk103_rv;
        *var_t3__blk107_slot = var_t3__blk107;
        *var_t3__blk107_dn0_slot = var_t3__blk107_dn0;
        *var_t3__blk107_dn10_slot = var_t3__blk107_dn10;
        *var_t3__blk107_dn11_slot = var_t3__blk107_dn11;
        *var_t3__blk107_dn12_slot = var_t3__blk107_dn12;
        *var_t3__blk107_dn17_slot = var_t3__blk107_dn17;
        *var_t3__blk107_dn2_slot = var_t3__blk107_dn2;
        *var_t3__blk107_dn6_slot = var_t3__blk107_dn6;
        *var_t3__blk107_dn7_slot = var_t3__blk107_dn7;
        *var_t3__blk107_rv_slot = var_t3__blk107_rv;
        *var_t3__blk86_slot = var_t3__blk86;
        *var_t3__blk86_dn0_slot = var_t3__blk86_dn0;
        *var_t3__blk86_dn10_slot = var_t3__blk86_dn10;
        *var_t3__blk86_dn11_slot = var_t3__blk86_dn11;
        *var_t3__blk86_dn12_slot = var_t3__blk86_dn12;
        *var_t3__blk86_dn17_slot = var_t3__blk86_dn17;
        *var_t3__blk86_dn2_slot = var_t3__blk86_dn2;
        *var_t3__blk86_dn6_slot = var_t3__blk86_dn6;
        *var_t3__blk86_dn7_slot = var_t3__blk86_dn7;
        *var_t3__blk86_rv_slot = var_t3__blk86_rv;
        *var_t3__blk94_slot = var_t3__blk94;
        *var_t3__blk94_rv_slot = var_t3__blk94_rv;
        *var_t4__blk95_slot = var_t4__blk95;
        *var_t4__blk95_dn0_slot = var_t4__blk95_dn0;
        *var_t4__blk95_dn10_slot = var_t4__blk95_dn10;
        *var_t4__blk95_dn11_slot = var_t4__blk95_dn11;
        *var_t4__blk95_dn12_slot = var_t4__blk95_dn12;
        *var_t4__blk95_dn17_slot = var_t4__blk95_dn17;
        *var_t4__blk95_dn2_slot = var_t4__blk95_dn2;
        *var_t4__blk95_dn6_slot = var_t4__blk95_dn6;
        *var_t4__blk95_dn7_slot = var_t4__blk95_dn7;
        *var_t4__blk95_rv_slot = var_t4__blk95_rv;
        *var_t5__blk104_slot = var_t5__blk104;
        *var_t5__blk104_dn0_slot = var_t5__blk104_dn0;
        *var_t5__blk104_dn10_slot = var_t5__blk104_dn10;
        *var_t5__blk104_dn11_slot = var_t5__blk104_dn11;
        *var_t5__blk104_dn12_slot = var_t5__blk104_dn12;
        *var_t5__blk104_dn17_slot = var_t5__blk104_dn17;
        *var_t5__blk104_dn2_slot = var_t5__blk104_dn2;
        *var_t5__blk104_dn6_slot = var_t5__blk104_dn6;
        *var_t5__blk104_dn7_slot = var_t5__blk104_dn7;
        *var_t5__blk104_rv_slot = var_t5__blk104_rv;
        *var_t5__blk88_slot = var_t5__blk88;
        *var_t5__blk88_dn0_slot = var_t5__blk88_dn0;
        *var_t5__blk88_dn10_slot = var_t5__blk88_dn10;
        *var_t5__blk88_dn11_slot = var_t5__blk88_dn11;
        *var_t5__blk88_dn12_slot = var_t5__blk88_dn12;
        *var_t5__blk88_dn17_slot = var_t5__blk88_dn17;
        *var_t5__blk88_dn2_slot = var_t5__blk88_dn2;
        *var_t5__blk88_dn6_slot = var_t5__blk88_dn6;
        *var_t5__blk88_dn7_slot = var_t5__blk88_dn7;
        *var_t5__blk88_rv_slot = var_t5__blk88_rv;
        *var_t5__blk96_slot = var_t5__blk96;
        *var_t5__blk96_dn0_slot = var_t5__blk96_dn0;
        *var_t5__blk96_dn10_slot = var_t5__blk96_dn10;
        *var_t5__blk96_dn11_slot = var_t5__blk96_dn11;
        *var_t5__blk96_dn12_slot = var_t5__blk96_dn12;
        *var_t5__blk96_dn17_slot = var_t5__blk96_dn17;
        *var_t5__blk96_dn2_slot = var_t5__blk96_dn2;
        *var_t5__blk96_dn6_slot = var_t5__blk96_dn6;
        *var_t5__blk96_dn7_slot = var_t5__blk96_dn7;
        *var_t5__blk96_rv_slot = var_t5__blk96_rv;
        *var_t7__blk105_slot = var_t7__blk105;
        *var_t7__blk105_dn0_slot = var_t7__blk105_dn0;
        *var_t7__blk105_dn10_slot = var_t7__blk105_dn10;
        *var_t7__blk105_dn11_slot = var_t7__blk105_dn11;
        *var_t7__blk105_dn12_slot = var_t7__blk105_dn12;
        *var_t7__blk105_dn17_slot = var_t7__blk105_dn17;
        *var_t7__blk105_dn2_slot = var_t7__blk105_dn2;
        *var_t7__blk105_dn6_slot = var_t7__blk105_dn6;
        *var_t7__blk105_dn7_slot = var_t7__blk105_dn7;
        *var_t7__blk105_rv_slot = var_t7__blk105_rv;
        *var_vth_slot = var_vth;
        *var_vth_dn0_slot = var_vth_dn0;
        *var_vth_dn10_slot = var_vth_dn10;
        *var_vth_dn11_slot = var_vth_dn11;
        *var_vth_dn12_slot = var_vth_dn12;
        *var_vth_dn17_slot = var_vth_dn17;
        *var_vth_dn2_slot = var_vth_dn2;
        *var_vth_dn6_slot = var_vth_dn6;
        *var_vth_dn7_slot = var_vth_dn7;
        *var_vth_rv_slot = var_vth_rv;
    }
}
