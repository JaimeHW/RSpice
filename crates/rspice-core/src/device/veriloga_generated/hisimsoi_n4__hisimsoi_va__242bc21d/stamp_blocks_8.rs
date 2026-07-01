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
        var_guard1214_slot: &mut f64,
        var_guard1220_slot: &mut f64,
        var_guard1222_slot: &mut f64,
        var_guard1223_slot: &mut f64,
        var_guard1224_slot: &mut f64,
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
        let mut var_guard1214: f64 = *var_guard1214_slot;
        let mut var_guard1220: f64 = *var_guard1220_slot;
        let mut var_guard1222: f64 = *var_guard1222_slot;
        let mut var_guard1223: f64 = *var_guard1223_slot;
        let mut var_guard1224: f64 = *var_guard1224_slot;
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

        let assign37160_e51524: f64 = (p.p50 * var_cgdbd);
        var_cgdbd = assign37160_e51524;
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

        let assign37170_e51527: f64 = var_qg_dn7;
        var_cgsbd = assign37170_e51527;
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

        let assign37180_e51530: f64 = (p.p50 * var_cgsbd);
        var_cgsbd = assign37180_e51530;
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

        let assign37450_e51611: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1214 = assign37450_e51611;

        let (assign37460_e51617, assign37460_e51617_d_n0, assign37460_e51617_d_n2, assign37460_e51617_d_n6, assign37460_e51617_d_n7, assign37460_e51617_d_n10, assign37460_e51617_d_n11, assign37460_e51617_d_n12, assign37460_e51617_d_n17,) = {
    if (var_guard1214 != 0.0) {
        let assign37460_e51615: f64 = (p.p50 * var_ibd);
        (assign37460_e51615, (p.p50 * var_ibd_dn0), (p.p50 * var_ibd_dn2), (p.p50 * var_ibd_dn6), (p.p50 * var_ibd_dn7), (p.p50 * var_ibd_dn10), (p.p50 * var_ibd_dn11), (p.p50 * var_ibd_dn12), (p.p50 * var_ibd_dn17),)
    } else {
        (var_ibdb, var_ibdb_dn0, var_ibdb_dn2, var_ibdb_dn6, var_ibdb_dn7, var_ibdb_dn10, var_ibdb_dn11, var_ibdb_dn12, var_ibdb_dn17,)
    }
};
        var_ibdb = assign37460_e51617;
        var_ibdb_dn0 = assign37460_e51617_d_n0;
        var_ibdb_dn2 = assign37460_e51617_d_n2;
        var_ibdb_dn6 = assign37460_e51617_d_n6;
        var_ibdb_dn7 = assign37460_e51617_d_n7;
        var_ibdb_dn10 = assign37460_e51617_d_n10;
        var_ibdb_dn11 = assign37460_e51617_d_n11;
        var_ibdb_dn12 = assign37460_e51617_d_n12;
        var_ibdb_dn17 = assign37460_e51617_d_n17;

        let (assign37470_e51623, assign37470_e51623_d_n0, assign37470_e51623_d_n2, assign37470_e51623_d_n6, assign37470_e51623_d_n7, assign37470_e51623_d_n10, assign37470_e51623_d_n11, assign37470_e51623_d_n12, assign37470_e51623_d_n17,) = {
    if (var_guard1214 != 0.0) {
        let assign37470_e51621: f64 = (p.p50 * var_ibs);
        (assign37470_e51621, (p.p50 * var_ibs_dn0), (p.p50 * var_ibs_dn2), (p.p50 * var_ibs_dn6), (p.p50 * var_ibs_dn7), (p.p50 * var_ibs_dn10), (p.p50 * var_ibs_dn11), (p.p50 * var_ibs_dn12), (p.p50 * var_ibs_dn17),)
    } else {
        (var_ibsb, var_ibsb_dn0, var_ibsb_dn2, var_ibsb_dn6, var_ibsb_dn7, var_ibsb_dn10, var_ibsb_dn11, var_ibsb_dn12, var_ibsb_dn17,)
    }
};
        var_ibsb = assign37470_e51623;
        var_ibsb_dn0 = assign37470_e51623_d_n0;
        var_ibsb_dn2 = assign37470_e51623_d_n2;
        var_ibsb_dn6 = assign37470_e51623_d_n6;
        var_ibsb_dn7 = assign37470_e51623_d_n7;
        var_ibsb_dn10 = assign37470_e51623_d_n10;
        var_ibsb_dn11 = assign37470_e51623_d_n11;
        var_ibsb_dn12 = assign37470_e51623_d_n12;
        var_ibsb_dn17 = assign37470_e51623_d_n17;

        let assign37590_e51675: f64 = (4.0 * 1.3806226e-23);
        let assign37590_e51677: f64 = (assign37590_e51675 * var_ttemp);
        let assign37590_e51679: f64 = assign37590_e51677;
        var_whi_noise = assign37590_e51679;
        var_whi_noise_dn10 = (assign37590_e51675 * var_ttemp_dn10);

        let assign37600_e51682: f64 = if p.p27 == 1.0 { 1.0 } else { 0.0 };
        var_guard1220 = assign37600_e51682;

        var_qdrat = var_qdrat_noi;
        var_qdrat_dn0 = var_qdrat_noi_dn0;
        var_qdrat_dn2 = var_qdrat_noi_dn2;
        var_qdrat_dn6 = var_qdrat_noi_dn6;
        var_qdrat_dn7 = var_qdrat_noi_dn7;
        var_qdrat_dn10 = var_qdrat_noi_dn10;
        var_qdrat_dn11 = var_qdrat_noi_dn11;
        var_qdrat_dn12 = var_qdrat_noi_dn12;
        var_qdrat_dn17 = var_qdrat_noi_dn17;

        let assign37620_e51686: f64 = (var_whi_noise * var_noithrml);
        var_sid = assign37620_e51686;
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

        let (assign37640_e51700, assign37640_e51700_d_n0, assign37640_e51700_d_n2, assign37640_e51700_d_n6, assign37640_e51700_d_n7, assign37640_e51700_d_n10, assign37640_e51700_d_n11, assign37640_e51700_d_n12, assign37640_e51700_d_n13, assign37640_e51700_d_n15, assign37640_e51700_d_n16, assign37640_e51700_d_n17, assign37640_e51700_d_n18,) = {
    if ((var_sid > 0.0) && (var_noiigate > 0.0)) {
        let assign37640_e51697: f64 = (var_noiigate / var_sid);
        let assign37640_e51698: f64 = (assign37640_e51697).sqrt();
        (assign37640_e51698, ((((var_noiigate_dn0 * var_sid) - (var_noiigate * var_sid_dn0)) / (var_sid * var_sid)) / (2.0 * assign37640_e51698)), ((((var_noiigate_dn2 * var_sid) - (var_noiigate * var_sid_dn2)) / (var_sid * var_sid)) / (2.0 * assign37640_e51698)), ((((var_noiigate_dn6 * var_sid) - (var_noiigate * var_sid_dn6)) / (var_sid * var_sid)) / (2.0 * assign37640_e51698)), ((((var_noiigate_dn7 * var_sid) - (var_noiigate * var_sid_dn7)) / (var_sid * var_sid)) / (2.0 * assign37640_e51698)), ((((var_noiigate_dn10 * var_sid) - (var_noiigate * var_sid_dn10)) / (var_sid * var_sid)) / (2.0 * assign37640_e51698)), ((((var_noiigate_dn11 * var_sid) - (var_noiigate * var_sid_dn11)) / (var_sid * var_sid)) / (2.0 * assign37640_e51698)), ((((var_noiigate_dn12 * var_sid) - (var_noiigate * var_sid_dn12)) / (var_sid * var_sid)) / (2.0 * assign37640_e51698)), ((var_noiigate_dn13 / var_sid) / (2.0 * assign37640_e51698)), ((var_noiigate_dn15 / var_sid) / (2.0 * assign37640_e51698)), ((var_noiigate_dn16 / var_sid) / (2.0 * assign37640_e51698)), ((((var_noiigate_dn17 * var_sid) - (var_noiigate * var_sid_dn17)) / (var_sid * var_sid)) / (2.0 * assign37640_e51698)), ((var_noiigate_dn18 / var_sid) / (2.0 * assign37640_e51698)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_sigrat = assign37640_e51700;
        var_sigrat_dn0 = assign37640_e51700_d_n0;
        var_sigrat_dn2 = assign37640_e51700_d_n2;
        var_sigrat_dn6 = assign37640_e51700_d_n6;
        var_sigrat_dn7 = assign37640_e51700_d_n7;
        var_sigrat_dn10 = assign37640_e51700_d_n10;
        var_sigrat_dn11 = assign37640_e51700_d_n11;
        var_sigrat_dn12 = assign37640_e51700_d_n12;
        var_sigrat_dn13 = assign37640_e51700_d_n13;
        var_sigrat_dn15 = assign37640_e51700_d_n15;
        var_sigrat_dn16 = assign37640_e51700_d_n16;
        var_sigrat_dn17 = assign37640_e51700_d_n17;
        var_sigrat_dn18 = assign37640_e51700_d_n18;

        let (assign37650_e51712, assign37650_e51712_d_n0, assign37650_e51712_d_n2, assign37650_e51712_d_n6, assign37650_e51712_d_n7, assign37650_e51712_d_n10, assign37650_e51712_d_n11, assign37650_e51712_d_n12, assign37650_e51712_d_n13, assign37650_e51712_d_n15, assign37650_e51712_d_n16, assign37650_e51712_d_n17, assign37650_e51712_d_n18,) = {
    if (var_mode > 0.0) {
        let assign37650_e51707: f64 = (1.0 - var_qdrat);
        let assign37650_e51708: f64 = (var_sigrat * assign37650_e51707);
        (assign37650_e51708, ((var_sigrat_dn0 * assign37650_e51707) + (var_sigrat * (-var_qdrat_dn0))), ((var_sigrat_dn2 * assign37650_e51707) + (var_sigrat * (-var_qdrat_dn2))), ((var_sigrat_dn6 * assign37650_e51707) + (var_sigrat * (-var_qdrat_dn6))), ((var_sigrat_dn7 * assign37650_e51707) + (var_sigrat * (-var_qdrat_dn7))), ((var_sigrat_dn10 * assign37650_e51707) + (var_sigrat * (-var_qdrat_dn10))), ((var_sigrat_dn11 * assign37650_e51707) + (var_sigrat * (-var_qdrat_dn11))), ((var_sigrat_dn12 * assign37650_e51707) + (var_sigrat * (-var_qdrat_dn12))), (var_sigrat_dn13 * assign37650_e51707), (var_sigrat_dn15 * assign37650_e51707), (var_sigrat_dn16 * assign37650_e51707), ((var_sigrat_dn17 * assign37650_e51707) + (var_sigrat * (-var_qdrat_dn17))), (var_sigrat_dn18 * assign37650_e51707),)
    } else {
        let assign37650_e51711: f64 = (var_sigrat * var_qdrat);
        (assign37650_e51711, ((var_sigrat_dn0 * var_qdrat) + (var_sigrat * var_qdrat_dn0)), ((var_sigrat_dn2 * var_qdrat) + (var_sigrat * var_qdrat_dn2)), ((var_sigrat_dn6 * var_qdrat) + (var_sigrat * var_qdrat_dn6)), ((var_sigrat_dn7 * var_qdrat) + (var_sigrat * var_qdrat_dn7)), ((var_sigrat_dn10 * var_qdrat) + (var_sigrat * var_qdrat_dn10)), ((var_sigrat_dn11 * var_qdrat) + (var_sigrat * var_qdrat_dn11)), ((var_sigrat_dn12 * var_qdrat) + (var_sigrat * var_qdrat_dn12)), (var_sigrat_dn13 * var_qdrat), (var_sigrat_dn15 * var_qdrat), (var_sigrat_dn16 * var_qdrat), ((var_sigrat_dn17 * var_qdrat) + (var_sigrat * var_qdrat_dn17)), (var_sigrat_dn18 * var_qdrat),)
    }
};
        var_sigrat_s = assign37650_e51712;
        var_sigrat_s_dn0 = assign37650_e51712_d_n0;
        var_sigrat_s_dn2 = assign37650_e51712_d_n2;
        var_sigrat_s_dn6 = assign37650_e51712_d_n6;
        var_sigrat_s_dn7 = assign37650_e51712_d_n7;
        var_sigrat_s_dn10 = assign37650_e51712_d_n10;
        var_sigrat_s_dn11 = assign37650_e51712_d_n11;
        var_sigrat_s_dn12 = assign37650_e51712_d_n12;
        var_sigrat_s_dn13 = assign37650_e51712_d_n13;
        var_sigrat_s_dn15 = assign37650_e51712_d_n15;
        var_sigrat_s_dn16 = assign37650_e51712_d_n16;
        var_sigrat_s_dn17 = assign37650_e51712_d_n17;
        var_sigrat_s_dn18 = assign37650_e51712_d_n18;

        let (assign37660_e51724, assign37660_e51724_d_n0, assign37660_e51724_d_n2, assign37660_e51724_d_n6, assign37660_e51724_d_n7, assign37660_e51724_d_n10, assign37660_e51724_d_n11, assign37660_e51724_d_n12, assign37660_e51724_d_n13, assign37660_e51724_d_n15, assign37660_e51724_d_n16, assign37660_e51724_d_n17, assign37660_e51724_d_n18,) = {
    if (var_mode > 0.0) {
        let assign37660_e51718: f64 = (var_sigrat * var_qdrat);
        (assign37660_e51718, ((var_sigrat_dn0 * var_qdrat) + (var_sigrat * var_qdrat_dn0)), ((var_sigrat_dn2 * var_qdrat) + (var_sigrat * var_qdrat_dn2)), ((var_sigrat_dn6 * var_qdrat) + (var_sigrat * var_qdrat_dn6)), ((var_sigrat_dn7 * var_qdrat) + (var_sigrat * var_qdrat_dn7)), ((var_sigrat_dn10 * var_qdrat) + (var_sigrat * var_qdrat_dn10)), ((var_sigrat_dn11 * var_qdrat) + (var_sigrat * var_qdrat_dn11)), ((var_sigrat_dn12 * var_qdrat) + (var_sigrat * var_qdrat_dn12)), (var_sigrat_dn13 * var_qdrat), (var_sigrat_dn15 * var_qdrat), (var_sigrat_dn16 * var_qdrat), ((var_sigrat_dn17 * var_qdrat) + (var_sigrat * var_qdrat_dn17)), (var_sigrat_dn18 * var_qdrat),)
    } else {
        let assign37660_e51722: f64 = (1.0 - var_qdrat);
        let assign37660_e51723: f64 = (var_sigrat * assign37660_e51722);
        (assign37660_e51723, ((var_sigrat_dn0 * assign37660_e51722) + (var_sigrat * (-var_qdrat_dn0))), ((var_sigrat_dn2 * assign37660_e51722) + (var_sigrat * (-var_qdrat_dn2))), ((var_sigrat_dn6 * assign37660_e51722) + (var_sigrat * (-var_qdrat_dn6))), ((var_sigrat_dn7 * assign37660_e51722) + (var_sigrat * (-var_qdrat_dn7))), ((var_sigrat_dn10 * assign37660_e51722) + (var_sigrat * (-var_qdrat_dn10))), ((var_sigrat_dn11 * assign37660_e51722) + (var_sigrat * (-var_qdrat_dn11))), ((var_sigrat_dn12 * assign37660_e51722) + (var_sigrat * (-var_qdrat_dn12))), (var_sigrat_dn13 * assign37660_e51722), (var_sigrat_dn15 * assign37660_e51722), (var_sigrat_dn16 * assign37660_e51722), ((var_sigrat_dn17 * assign37660_e51722) + (var_sigrat * (-var_qdrat_dn17))), (var_sigrat_dn18 * assign37660_e51722),)
    }
};
        var_sigrat_d = assign37660_e51724;
        var_sigrat_d_dn0 = assign37660_e51724_d_n0;
        var_sigrat_d_dn2 = assign37660_e51724_d_n2;
        var_sigrat_d_dn6 = assign37660_e51724_d_n6;
        var_sigrat_d_dn7 = assign37660_e51724_d_n7;
        var_sigrat_d_dn10 = assign37660_e51724_d_n10;
        var_sigrat_d_dn11 = assign37660_e51724_d_n11;
        var_sigrat_d_dn12 = assign37660_e51724_d_n12;
        var_sigrat_d_dn13 = assign37660_e51724_d_n13;
        var_sigrat_d_dn15 = assign37660_e51724_d_n15;
        var_sigrat_d_dn16 = assign37660_e51724_d_n16;
        var_sigrat_d_dn17 = assign37660_e51724_d_n17;
        var_sigrat_d_dn18 = assign37660_e51724_d_n18;

        let assign37680_e51734: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1222 = assign37680_e51734;

        let (assign37690_e51738, assign37690_e51738_d_n0, assign37690_e51738_d_n2, assign37690_e51738_d_n6, assign37690_e51738_d_n7, assign37690_e51738_d_n10, assign37690_e51738_d_n11, assign37690_e51738_d_n12, assign37690_e51738_d_n17,) = {
    if (var_guard1222 != 0.0) {
        (var_rpower, var_rpower_dn0, var_rpower_dn2, var_rpower_dn6, var_rpower_dn7, var_rpower_dn10, var_rpower_dn11, var_rpower_dn12, var_rpower_dn17,)
    } else {
        (var_itemp, var_itemp_dn0, var_itemp_dn2, var_itemp_dn6, var_itemp_dn7, var_itemp_dn10, var_itemp_dn11, var_itemp_dn12, var_itemp_dn17,)
    }
};
        var_itemp = assign37690_e51738;
        var_itemp_dn0 = assign37690_e51738_d_n0;
        var_itemp_dn2 = assign37690_e51738_d_n2;
        var_itemp_dn6 = assign37690_e51738_d_n6;
        var_itemp_dn7 = assign37690_e51738_d_n7;
        var_itemp_dn10 = assign37690_e51738_d_n10;
        var_itemp_dn11 = assign37690_e51738_d_n11;
        var_itemp_dn12 = assign37690_e51738_d_n12;
        var_itemp_dn17 = assign37690_e51738_d_n17;

        let assign37700_e51741: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1223 = assign37700_e51741;

        let assign37710_e51750: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        var_guard1224 = assign37710_e51750;

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
        *var_guard1214_slot = var_guard1214;
        *var_guard1220_slot = var_guard1220;
        *var_guard1222_slot = var_guard1222;
        *var_guard1223_slot = var_guard1223;
        *var_guard1224_slot = var_guard1224;
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
        var_guard2_slot: &mut f64,
        var_guard2_rv_slot: &mut f64,
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
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_guard2_rv: f64 = *var_guard2_rv_slot;
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

        let assign1200_e968: f64 = (p.p51 * 10.0);
        let assign1200_e970: f64 = (assign1200_e968 % 10.0);
        var_subversion = assign1200_e970;
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

        let assign1310_e983: f64 = (p.p52 * 0.01);
        var_mks_vmax = assign1310_e983;
        var_mks_vmax_rv = 0.0;

        let assign1320_e986: f64 = (p.p73 / 1e-6);
        var_mks_nsubp = assign1320_e986;
        var_mks_nsubp_rv = 0.0;

        let assign1330_e989: f64 = (p.p104 * 0.01);
        var_mks_vtmp = assign1330_e989;
        var_mks_vtmp_rv = 0.0;

        let assign1340_e992: f64 = (p.p201 / 1e-6);
        var_mks_nsubcmax = assign1340_e992;
        var_mks_nsubcmax_rv = 0.0;

        let assign1380_e1004: f64 = (p.p240 / 1e-6);
        var_mks_nsubs = assign1380_e1004;
        var_mks_nsubs_rv = 0.0;

        let assign1390_e1007: f64 = (p.p241 / 1e-6);
        var_mks_nsubb = assign1390_e1007;
        var_mks_nsubb_rv = 0.0;

        let assign1400_e1010: f64 = (p.p242 * 0.01);
        var_mks_rth0 = assign1400_e1010;
        var_mks_rth0_rv = 0.0;

        let assign1410_e1013: f64 = (p.p243 / 0.01);
        var_mks_cth0 = assign1410_e1013;
        var_mks_cth0_rv = 0.0;

        let assign1420_e1016: f64 = (p.p59 / 1e-6);
        var_mks_nover = assign1420_e1016;
        var_mks_nover_rv = 0.0;

        let assign1430_e1019: f64 = (p.p284 / 1e-6);
        var_mks_njunc = assign1430_e1019;
        var_mks_njunc_rv = 0.0;

        let assign1440_e1022: f64 = (p.p148 / 1e-6);
        var_mks_nsti = assign1440_e1022;
        var_mks_nsti_rv = 0.0;

        let assign1450_e1025: f64 = (p.p198 / 0.0001);
        var_mks_wfc = assign1450_e1025;
        var_mks_wfc_rv = 0.0;

        let assign1460_e1028: f64 = (p.p70 * 0.01);
        var_mks_parl1 = assign1460_e1028;
        var_mks_parl1_rv = 0.0;

        let (assign1470_e1034,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p84,)
    }
};
        var_uc_sc2 = assign1470_e1034;
        var_uc_sc2_rv = 0.0;

        let (assign1480_e1040,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p85,)
    }
};
        var_uc_sc3 = assign1480_e1040;
        var_uc_sc3_rv = 0.0;

        let (assign1490_e1046,) = {
    if (p.p80 == 0.0) {
        (0.0,)
    } else {
        (p.p81,)
    }
};
        var_uc_scp2 = assign1490_e1046;
        var_uc_scp2_rv = 0.0;

        let (assign1500_e1052,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p82,)
    }
};
        var_uc_scp3 = assign1500_e1052;
        var_uc_scp3_rv = 0.0;

        let assign1510_e1055: f64 = (p.p250 * 1000000.0);
        var_uc_gdld = assign1510_e1055;
        var_uc_gdld_rv = 0.0;

        let assign1520_e1058: f64 = (p.p232 + 273.15);
        var_uc_tnom = assign1520_e1058;
        var_uc_tnom_rv = 0.0;

        var_uc_vfbover = p.p58;
        var_uc_vfbover_rv = 0.0;

        var_flg_info = p.p46;
        var_flg_info_rv = 0.0;

        var_flg_nqs = p.p34;
        var_flg_nqs_rv = 0.0;

        let (assign1570_e1073,) = {
    if param_given[190] {
        (p.p190,)
    } else {
        let assign1570_e1071: f64 = (p.p237 * p.p240);
        let assign1570_e1072: f64 = (5000000000.0 / assign1570_e1071);
        (assign1570_e1072,)
    }
};
        var_uc_clm2 = assign1570_e1073;
        var_uc_clm2_dn0 = 0.0;
        var_uc_clm2_dn2 = 0.0;
        var_uc_clm2_dn6 = 0.0;
        var_uc_clm2_dn7 = 0.0;
        var_uc_clm2_dn10 = 0.0;
        var_uc_clm2_dn11 = 0.0;
        var_uc_clm2_dn12 = 0.0;
        var_uc_clm2_dn17 = 0.0;
        var_uc_clm2_rv = 0.0;

        let assign1580_e1077: f64 = (2.0 + 0.1);
        let assign1580_e1082: f64 = if ((var_uc_clm2 < assign1580_e1077) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard2 = assign1580_e1082;
        var_guard2_rv = 0.0;

        let (assign1590_e1090, assign1590_e1090_d_n0, assign1590_e1090_d_n2, assign1590_e1090_d_n6, assign1590_e1090_d_n7, assign1590_e1090_d_n10, assign1590_e1090_d_n11, assign1590_e1090_d_n12, assign1590_e1090_d_n17,) = {
    if (var_guard2 != 0.0) {
        let assign1590_e1086: f64 = (2.0 + 0.1);
        let assign1590_e1088: f64 = (assign1590_e1086 - var_uc_clm2);
        (assign1590_e1088, (-var_uc_clm2_dn0), (-var_uc_clm2_dn2), (-var_uc_clm2_dn6), (-var_uc_clm2_dn7), (-var_uc_clm2_dn10), (-var_uc_clm2_dn11), (-var_uc_clm2_dn12), (-var_uc_clm2_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign1590_e1090;
        var_tmf1_dn0 = assign1590_e1090_d_n0;
        var_tmf1_dn2 = assign1590_e1090_d_n2;
        var_tmf1_dn6 = assign1590_e1090_d_n6;
        var_tmf1_dn7 = assign1590_e1090_d_n7;
        var_tmf1_dn10 = assign1590_e1090_d_n10;
        var_tmf1_dn11 = assign1590_e1090_d_n11;
        var_tmf1_dn12 = assign1590_e1090_d_n12;
        var_tmf1_dn17 = assign1590_e1090_d_n17;
        var_tmf1_rv = 0.0;

        let (assign1600_e1096, assign1600_e1096_d_n0, assign1600_e1096_d_n2, assign1600_e1096_d_n6, assign1600_e1096_d_n7, assign1600_e1096_d_n10, assign1600_e1096_d_n11, assign1600_e1096_d_n12, assign1600_e1096_d_n17,) = {
    if (var_guard2 != 0.0) {
        let assign1600_e1094: f64 = (var_tmf1 * var_tmf1);
        (assign1600_e1094, ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)), ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)), ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)), ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)), ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)), ((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)), ((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)), ((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn6, var_x2_dn7, var_x2_dn10, var_x2_dn11, var_x2_dn12, var_x2_dn17,)
    }
};
        var_x2 = assign1600_e1096;
        var_x2_dn0 = assign1600_e1096_d_n0;
        var_x2_dn2 = assign1600_e1096_d_n2;
        var_x2_dn6 = assign1600_e1096_d_n6;
        var_x2_dn7 = assign1600_e1096_d_n7;
        var_x2_dn10 = assign1600_e1096_d_n10;
        var_x2_dn11 = assign1600_e1096_d_n11;
        var_x2_dn12 = assign1600_e1096_d_n12;
        var_x2_dn17 = assign1600_e1096_d_n17;
        var_x2_rv = 0.0;

        let (assign1610_e1102, assign1610_e1102_d_n0, assign1610_e1102_d_n2, assign1610_e1102_d_n6, assign1610_e1102_d_n7, assign1610_e1102_d_n10, assign1610_e1102_d_n11, assign1610_e1102_d_n12, assign1610_e1102_d_n17,) = {
    if (var_guard2 != 0.0) {
        let assign1610_e1100: f64 = (0.1 * 0.1);
        (assign1610_e1100, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn10, var_xmax2_dn11, var_xmax2_dn12, var_xmax2_dn17,)
    }
};
        var_xmax2 = assign1610_e1102;
        var_xmax2_dn0 = assign1610_e1102_d_n0;
        var_xmax2_dn2 = assign1610_e1102_d_n2;
        var_xmax2_dn6 = assign1610_e1102_d_n6;
        var_xmax2_dn7 = assign1610_e1102_d_n7;
        var_xmax2_dn10 = assign1610_e1102_d_n10;
        var_xmax2_dn11 = assign1610_e1102_d_n11;
        var_xmax2_dn12 = assign1610_e1102_d_n12;
        var_xmax2_dn17 = assign1610_e1102_d_n17;
        var_xmax2_rv = 0.0;

        let (assign1620_e1106, assign1620_e1106_d_n0, assign1620_e1106_d_n2, assign1620_e1106_d_n6, assign1620_e1106_d_n7, assign1620_e1106_d_n10, assign1620_e1106_d_n11, assign1620_e1106_d_n12, assign1620_e1106_d_n17,) = {
    if (var_guard2 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign1620_e1106;
        var_xp_dn0 = assign1620_e1106_d_n0;
        var_xp_dn2 = assign1620_e1106_d_n2;
        var_xp_dn6 = assign1620_e1106_d_n6;
        var_xp_dn7 = assign1620_e1106_d_n7;
        var_xp_dn10 = assign1620_e1106_d_n10;
        var_xp_dn11 = assign1620_e1106_d_n11;
        var_xp_dn12 = assign1620_e1106_d_n12;
        var_xp_dn17 = assign1620_e1106_d_n17;
        var_xp_rv = 0.0;

        let (assign1630_e1110, assign1630_e1110_d_n0, assign1630_e1110_d_n2, assign1630_e1110_d_n6, assign1630_e1110_d_n7, assign1630_e1110_d_n10, assign1630_e1110_d_n11, assign1630_e1110_d_n12, assign1630_e1110_d_n17,) = {
    if (var_guard2 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign1630_e1110;
        var_xmp_dn0 = assign1630_e1110_d_n0;
        var_xmp_dn2 = assign1630_e1110_d_n2;
        var_xmp_dn6 = assign1630_e1110_d_n6;
        var_xmp_dn7 = assign1630_e1110_d_n7;
        var_xmp_dn10 = assign1630_e1110_d_n10;
        var_xmp_dn11 = assign1630_e1110_d_n11;
        var_xmp_dn12 = assign1630_e1110_d_n12;
        var_xmp_dn17 = assign1630_e1110_d_n17;
        var_xmp_rv = 0.0;

        let (assign1640_e1114,) = {
    if (var_guard2 != 0.0) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign1640_e1114;
        var_m0_rv = 0.0;

        let (assign1650_e1118,) = {
    if (var_guard2 != 0.0) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1650_e1118;
        var_mm_rv = 0.0;

        let (assign1660_e1122, assign1660_e1122_d_n0, assign1660_e1122_d_n2, assign1660_e1122_d_n6, assign1660_e1122_d_n7, assign1660_e1122_d_n10, assign1660_e1122_d_n11, assign1660_e1122_d_n12, assign1660_e1122_d_n17,) = {
    if (var_guard2 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    }
};
        var_arg = assign1660_e1122;
        var_arg_dn0 = assign1660_e1122_d_n0;
        var_arg_dn2 = assign1660_e1122_d_n2;
        var_arg_dn6 = assign1660_e1122_d_n6;
        var_arg_dn7 = assign1660_e1122_d_n7;
        var_arg_dn10 = assign1660_e1122_d_n10;
        var_arg_dn11 = assign1660_e1122_d_n11;
        var_arg_dn12 = assign1660_e1122_d_n12;
        var_arg_dn17 = assign1660_e1122_d_n17;
        var_arg_rv = 0.0;

        let (assign1670_e1126, assign1670_e1126_d_n0, assign1670_e1126_d_n2, assign1670_e1126_d_n6, assign1670_e1126_d_n7, assign1670_e1126_d_n10, assign1670_e1126_d_n11, assign1670_e1126_d_n12, assign1670_e1126_d_n17,) = {
    if (var_guard2 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign1670_e1126;
        var_dnm_dn0 = assign1670_e1126_d_n0;
        var_dnm_dn2 = assign1670_e1126_d_n2;
        var_dnm_dn6 = assign1670_e1126_d_n6;
        var_dnm_dn7 = assign1670_e1126_d_n7;
        var_dnm_dn10 = assign1670_e1126_d_n10;
        var_dnm_dn11 = assign1670_e1126_d_n11;
        var_dnm_dn12 = assign1670_e1126_d_n12;
        var_dnm_dn17 = assign1670_e1126_d_n17;
        var_dnm_rv = 0.0;

        let (assign1680_e1132, assign1680_e1132_d_n0, assign1680_e1132_d_n2, assign1680_e1132_d_n6, assign1680_e1132_d_n7, assign1680_e1132_d_n10, assign1680_e1132_d_n11, assign1680_e1132_d_n12, assign1680_e1132_d_n17,) = {
    if (var_guard2 != 0.0) {
        let assign1680_e1130: f64 = (var_xp * var_x2);
        (assign1680_e1130, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign1680_e1132;
        var_xp_dn0 = assign1680_e1132_d_n0;
        var_xp_dn2 = assign1680_e1132_d_n2;
        var_xp_dn6 = assign1680_e1132_d_n6;
        var_xp_dn7 = assign1680_e1132_d_n7;
        var_xp_dn10 = assign1680_e1132_d_n10;
        var_xp_dn11 = assign1680_e1132_d_n11;
        var_xp_dn12 = assign1680_e1132_d_n12;
        var_xp_dn17 = assign1680_e1132_d_n17;
        var_xp_rv = 0.0;

        let (assign1690_e1138, assign1690_e1138_d_n0, assign1690_e1138_d_n2, assign1690_e1138_d_n6, assign1690_e1138_d_n7, assign1690_e1138_d_n10, assign1690_e1138_d_n11, assign1690_e1138_d_n12, assign1690_e1138_d_n17,) = {
    if (var_guard2 != 0.0) {
        let assign1690_e1136: f64 = (var_xmp * var_xmax2);
        (assign1690_e1136, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign1690_e1138;
        var_xmp_dn0 = assign1690_e1138_d_n0;
        var_xmp_dn2 = assign1690_e1138_d_n2;
        var_xmp_dn6 = assign1690_e1138_d_n6;
        var_xmp_dn7 = assign1690_e1138_d_n7;
        var_xmp_dn10 = assign1690_e1138_d_n10;
        var_xmp_dn11 = assign1690_e1138_d_n11;
        var_xmp_dn12 = assign1690_e1138_d_n12;
        var_xmp_dn17 = assign1690_e1138_d_n17;
        var_xmp_rv = 0.0;

        let (assign1700_e1144, assign1700_e1144_d_n0, assign1700_e1144_d_n2, assign1700_e1144_d_n6, assign1700_e1144_d_n7, assign1700_e1144_d_n10, assign1700_e1144_d_n11, assign1700_e1144_d_n12, assign1700_e1144_d_n17,) = {
    if (var_guard2 != 0.0) {
        let assign1700_e1142: f64 = (var_xp * var_x2);
        (assign1700_e1142, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign1700_e1144;
        var_xp_dn0 = assign1700_e1144_d_n0;
        var_xp_dn2 = assign1700_e1144_d_n2;
        var_xp_dn6 = assign1700_e1144_d_n6;
        var_xp_dn7 = assign1700_e1144_d_n7;
        var_xp_dn10 = assign1700_e1144_d_n10;
        var_xp_dn11 = assign1700_e1144_d_n11;
        var_xp_dn12 = assign1700_e1144_d_n12;
        var_xp_dn17 = assign1700_e1144_d_n17;
        var_xp_rv = 0.0;

        let (assign1710_e1150, assign1710_e1150_d_n0, assign1710_e1150_d_n2, assign1710_e1150_d_n6, assign1710_e1150_d_n7, assign1710_e1150_d_n10, assign1710_e1150_d_n11, assign1710_e1150_d_n12, assign1710_e1150_d_n17,) = {
    if (var_guard2 != 0.0) {
        let assign1710_e1148: f64 = (var_xmp * var_xmax2);
        (assign1710_e1148, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign1710_e1150;
        var_xmp_dn0 = assign1710_e1150_d_n0;
        var_xmp_dn2 = assign1710_e1150_d_n2;
        var_xmp_dn6 = assign1710_e1150_d_n6;
        var_xmp_dn7 = assign1710_e1150_d_n7;
        var_xmp_dn10 = assign1710_e1150_d_n10;
        var_xmp_dn11 = assign1710_e1150_d_n11;
        var_xmp_dn12 = assign1710_e1150_d_n12;
        var_xmp_dn17 = assign1710_e1150_d_n17;
        var_xmp_rv = 0.0;

        let (assign1720_e1156, assign1720_e1156_d_n0, assign1720_e1156_d_n2, assign1720_e1156_d_n6, assign1720_e1156_d_n7, assign1720_e1156_d_n10, assign1720_e1156_d_n11, assign1720_e1156_d_n12, assign1720_e1156_d_n17,) = {
    if (var_guard2 != 0.0) {
        let assign1720_e1154: f64 = (var_xp + var_xmp);
        (assign1720_e1154, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn11 + var_xmp_dn11), (var_xp_dn12 + var_xmp_dn12), (var_xp_dn17 + var_xmp_dn17),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    }
};
        var_arg = assign1720_e1156;
        var_arg_dn0 = assign1720_e1156_d_n0;
        var_arg_dn2 = assign1720_e1156_d_n2;
        var_arg_dn6 = assign1720_e1156_d_n6;
        var_arg_dn7 = assign1720_e1156_d_n7;
        var_arg_dn10 = assign1720_e1156_d_n10;
        var_arg_dn11 = assign1720_e1156_d_n11;
        var_arg_dn12 = assign1720_e1156_d_n12;
        var_arg_dn17 = assign1720_e1156_d_n17;
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
        *var_guard2_slot = var_guard2;
        *var_guard2_rv_slot = var_guard2_rv;
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
        var_guard2: f64,
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
        var_guard3_slot: &mut f64,
        var_guard3_rv_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard4_rv_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard5_rv_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard6_rv_slot: &mut f64,
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
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard3_rv: f64 = *var_guard3_rv_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard4_rv: f64 = *var_guard4_rv_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard5_rv: f64 = *var_guard5_rv_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard6_rv: f64 = *var_guard6_rv_slot;
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

        let (assign1730_e1160, assign1730_e1160_d_n0, assign1730_e1160_d_n2, assign1730_e1160_d_n6, assign1730_e1160_d_n7, assign1730_e1160_d_n10, assign1730_e1160_d_n11, assign1730_e1160_d_n12, assign1730_e1160_d_n17,) = {
    if (var_guard2 != 0.0) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign1730_e1160;
        var_dnm_dn0 = assign1730_e1160_d_n0;
        var_dnm_dn2 = assign1730_e1160_d_n2;
        var_dnm_dn6 = assign1730_e1160_d_n6;
        var_dnm_dn7 = assign1730_e1160_d_n7;
        var_dnm_dn10 = assign1730_e1160_d_n10;
        var_dnm_dn11 = assign1730_e1160_d_n11;
        var_dnm_dn12 = assign1730_e1160_d_n12;
        var_dnm_dn17 = assign1730_e1160_d_n17;
        var_dnm_rv = 0.0;

        let assign1740_e1175: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard3 = assign1740_e1175;
        var_guard3_rv = 0.0;

        let assign1750_e1178: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard4 = assign1750_e1178;
        var_guard4_rv = 0.0;

        let (assign1760_e1186,) = {
    if (((var_guard2 != 0.0) && (var_guard3 != 0.0)) && (var_guard4 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1760_e1186;
        var_mm_rv = 0.0;

        let assign1770_e1189: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard5 = assign1770_e1189;
        var_guard5_rv = 0.0;

        let (assign1780_e1200,) = {
    if ((((var_guard2 != 0.0) && (var_guard3 != 0.0)) && (var_guard4 == 0.0)) && (var_guard5 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1780_e1200;
        var_mm_rv = 0.0;

        let assign1790_e1203: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard6 = assign1790_e1203;
        var_guard6_rv = 0.0;

        let (assign1800_e1217,) = {
    if (((((var_guard2 != 0.0) && (var_guard3 != 0.0)) && (var_guard4 == 0.0)) && (var_guard5 == 0.0)) && (var_guard6 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1800_e1217;
        var_mm_rv = 0.0;

        let assign1810_e1220: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard7 = assign1810_e1220;
        var_guard7_rv = 0.0;

        let (assign1820_e1237,) = {
    if ((((((var_guard2 != 0.0) && (var_guard3 != 0.0)) && (var_guard4 == 0.0)) && (var_guard5 == 0.0)) && (var_guard6 == 0.0)) && (var_guard7 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1820_e1237;
        var_mm_rv = 0.0;

        let (assign1830_e1243,) = {
    if ((var_guard2 != 0.0) && (var_guard3 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign1830_e1243;
        var_m0_rv = 0.0;

        let mut assign1840_loop_guard: usize = 0;
        while {
            let assign1840_cond_e1250: f64 = if (((var_guard2 != 0.0) && (var_guard3 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign1840_cond_e1250 != 0.0
        } {
            assign1840_loop_guard += 1;
            assert!(assign1840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign1840_body0_e1257, assign1840_body0_e1257_d_n0, assign1840_body0_e1257_d_n2, assign1840_body0_e1257_d_n6, assign1840_body0_e1257_d_n7, assign1840_body0_e1257_d_n10, assign1840_body0_e1257_d_n11, assign1840_body0_e1257_d_n12, assign1840_body0_e1257_d_n17,) = {
    if ((var_guard2 != 0.0) && (var_guard3 != 0.0)) {
        let assign1840_body0_e1255: f64 = (var_dnm).sqrt();
        (assign1840_body0_e1255, (var_dnm_dn0 / (2.0 * assign1840_body0_e1255)), (var_dnm_dn2 / (2.0 * assign1840_body0_e1255)), (var_dnm_dn6 / (2.0 * assign1840_body0_e1255)), (var_dnm_dn7 / (2.0 * assign1840_body0_e1255)), (var_dnm_dn10 / (2.0 * assign1840_body0_e1255)), (var_dnm_dn11 / (2.0 * assign1840_body0_e1255)), (var_dnm_dn12 / (2.0 * assign1840_body0_e1255)), (var_dnm_dn17 / (2.0 * assign1840_body0_e1255)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
            var_dnm = assign1840_body0_e1257;
            var_dnm_dn0 = assign1840_body0_e1257_d_n0;
            var_dnm_dn2 = assign1840_body0_e1257_d_n2;
            var_dnm_dn6 = assign1840_body0_e1257_d_n6;
            var_dnm_dn7 = assign1840_body0_e1257_d_n7;
            var_dnm_dn10 = assign1840_body0_e1257_d_n10;
            var_dnm_dn11 = assign1840_body0_e1257_d_n11;
            var_dnm_dn12 = assign1840_body0_e1257_d_n12;
            var_dnm_dn17 = assign1840_body0_e1257_d_n17;
            var_dnm_rv = 0.0;
            let (assign1840_body1_e1265,) = {
    if ((var_guard2 != 0.0) && (var_guard3 != 0.0)) {
        let assign1840_body1_e1263: f64 = (var_m0 + 1.0);
        (assign1840_body1_e1263,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign1840_body1_e1265;
            var_m0_rv = 0.0;
        }

        let (assign1850_e1278, assign1850_e1278_d_n0, assign1850_e1278_d_n2, assign1850_e1278_d_n6, assign1850_e1278_d_n7, assign1850_e1278_d_n10, assign1850_e1278_d_n11, assign1850_e1278_d_n12, assign1850_e1278_d_n17,) = {
    if ((var_guard2 != 0.0) && (var_guard3 == 0.0)) {
        let assign1850_e1274: f64 = (2.0 * 2.0);
        let assign1850_e1275: f64 = (1.0 / assign1850_e1274);
        let assign1850_e1276: f64 = (var_dnm).powf(assign1850_e1275);
        (assign1850_e1276, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((var_dnm).powf(assign1850_e1275 - 1.0) * var_dnm_dn0)) } } else { (assign1850_e1276 * (assign1850_e1275 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((var_dnm).powf(assign1850_e1275 - 1.0) * var_dnm_dn2)) } } else { (assign1850_e1276 * (assign1850_e1275 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((var_dnm).powf(assign1850_e1275 - 1.0) * var_dnm_dn6)) } } else { (assign1850_e1276 * (assign1850_e1275 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((var_dnm).powf(assign1850_e1275 - 1.0) * var_dnm_dn7)) } } else { (assign1850_e1276 * (assign1850_e1275 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((var_dnm).powf(assign1850_e1275 - 1.0) * var_dnm_dn10)) } } else { (assign1850_e1276 * (assign1850_e1275 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((var_dnm).powf(assign1850_e1275 - 1.0) * var_dnm_dn11)) } } else { (assign1850_e1276 * (assign1850_e1275 * (var_dnm_dn11 / var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((var_dnm).powf(assign1850_e1275 - 1.0) * var_dnm_dn12)) } } else { (assign1850_e1276 * (assign1850_e1275 * (var_dnm_dn12 / var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((var_dnm).powf(assign1850_e1275 - 1.0) * var_dnm_dn17)) } } else { (assign1850_e1276 * (assign1850_e1275 * (var_dnm_dn17 / var_dnm))) },)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign1850_e1278;
        var_dnm_dn0 = assign1850_e1278_d_n0;
        var_dnm_dn2 = assign1850_e1278_d_n2;
        var_dnm_dn6 = assign1850_e1278_d_n6;
        var_dnm_dn7 = assign1850_e1278_d_n7;
        var_dnm_dn10 = assign1850_e1278_d_n10;
        var_dnm_dn11 = assign1850_e1278_d_n11;
        var_dnm_dn12 = assign1850_e1278_d_n12;
        var_dnm_dn17 = assign1850_e1278_d_n17;
        var_dnm_rv = 0.0;

        let (assign1860_e1284, assign1860_e1284_d_n0, assign1860_e1284_d_n2, assign1860_e1284_d_n6, assign1860_e1284_d_n7, assign1860_e1284_d_n10, assign1860_e1284_d_n11, assign1860_e1284_d_n12, assign1860_e1284_d_n17,) = {
    if (var_guard2 != 0.0) {
        let assign1860_e1282: f64 = (1.0 / var_dnm);
        (assign1860_e1282, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn11 / (var_dnm * var_dnm))), (-(var_dnm_dn12 / (var_dnm * var_dnm))), (-(var_dnm_dn17 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign1860_e1284;
        var_dnm_dn0 = assign1860_e1284_d_n0;
        var_dnm_dn2 = assign1860_e1284_d_n2;
        var_dnm_dn6 = assign1860_e1284_d_n6;
        var_dnm_dn7 = assign1860_e1284_d_n7;
        var_dnm_dn10 = assign1860_e1284_d_n10;
        var_dnm_dn11 = assign1860_e1284_d_n11;
        var_dnm_dn12 = assign1860_e1284_d_n12;
        var_dnm_dn17 = assign1860_e1284_d_n17;
        var_dnm_rv = 0.0;

        let (assign1870_e1292, assign1870_e1292_d_n0, assign1870_e1292_d_n2, assign1870_e1292_d_n6, assign1870_e1292_d_n7, assign1870_e1292_d_n10, assign1870_e1292_d_n11, assign1870_e1292_d_n12, assign1870_e1292_d_n17,) = {
    if (var_guard2 != 0.0) {
        let assign1870_e1288: f64 = (var_tmf1 * 0.1);
        let assign1870_e1290: f64 = (assign1870_e1288 * var_dnm);
        (assign1870_e1290, (((var_tmf1_dn0 * 0.1) * var_dnm) + (assign1870_e1288 * var_dnm_dn0)), (((var_tmf1_dn2 * 0.1) * var_dnm) + (assign1870_e1288 * var_dnm_dn2)), (((var_tmf1_dn6 * 0.1) * var_dnm) + (assign1870_e1288 * var_dnm_dn6)), (((var_tmf1_dn7 * 0.1) * var_dnm) + (assign1870_e1288 * var_dnm_dn7)), (((var_tmf1_dn10 * 0.1) * var_dnm) + (assign1870_e1288 * var_dnm_dn10)), (((var_tmf1_dn11 * 0.1) * var_dnm) + (assign1870_e1288 * var_dnm_dn11)), (((var_tmf1_dn12 * 0.1) * var_dnm) + (assign1870_e1288 * var_dnm_dn12)), (((var_tmf1_dn17 * 0.1) * var_dnm) + (assign1870_e1288 * var_dnm_dn17)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn10, var_tmf0_dn11, var_tmf0_dn12, var_tmf0_dn17,)
    }
};
        var_tmf0 = assign1870_e1292;
        var_tmf0_dn0 = assign1870_e1292_d_n0;
        var_tmf0_dn2 = assign1870_e1292_d_n2;
        var_tmf0_dn6 = assign1870_e1292_d_n6;
        var_tmf0_dn7 = assign1870_e1292_d_n7;
        var_tmf0_dn10 = assign1870_e1292_d_n10;
        var_tmf0_dn11 = assign1870_e1292_d_n11;
        var_tmf0_dn12 = assign1870_e1292_d_n12;
        var_tmf0_dn17 = assign1870_e1292_d_n17;
        var_tmf0_rv = 0.0;

        let (assign1880_e1300, assign1880_e1300_d_n0, assign1880_e1300_d_n2, assign1880_e1300_d_n6, assign1880_e1300_d_n7, assign1880_e1300_d_n10, assign1880_e1300_d_n11, assign1880_e1300_d_n12, assign1880_e1300_d_n17,) = {
    if (var_guard2 != 0.0) {
        let assign1880_e1296: f64 = (2.0 + 0.1);
        let assign1880_e1298: f64 = (assign1880_e1296 - var_tmf0);
        (assign1880_e1298, (-var_tmf0_dn0), (-var_tmf0_dn2), (-var_tmf0_dn6), (-var_tmf0_dn7), (-var_tmf0_dn10), (-var_tmf0_dn11), (-var_tmf0_dn12), (-var_tmf0_dn17),)
    } else {
        (var_uc_clm2, var_uc_clm2_dn0, var_uc_clm2_dn2, var_uc_clm2_dn6, var_uc_clm2_dn7, var_uc_clm2_dn10, var_uc_clm2_dn11, var_uc_clm2_dn12, var_uc_clm2_dn17,)
    }
};
        var_uc_clm2 = assign1880_e1300;
        var_uc_clm2_dn0 = assign1880_e1300_d_n0;
        var_uc_clm2_dn2 = assign1880_e1300_d_n2;
        var_uc_clm2_dn6 = assign1880_e1300_d_n6;
        var_uc_clm2_dn7 = assign1880_e1300_d_n7;
        var_uc_clm2_dn10 = assign1880_e1300_d_n10;
        var_uc_clm2_dn11 = assign1880_e1300_d_n11;
        var_uc_clm2_dn12 = assign1880_e1300_d_n12;
        var_uc_clm2_dn17 = assign1880_e1300_d_n17;
        var_uc_clm2_rv = 0.0;

        let (assign1890_e1305, assign1890_e1305_d_n0, assign1890_e1305_d_n2, assign1890_e1305_d_n6, assign1890_e1305_d_n7, assign1890_e1305_d_n10, assign1890_e1305_d_n11, assign1890_e1305_d_n12, assign1890_e1305_d_n17,) = {
    if (var_guard2 == 0.0) {
        (var_uc_clm2, var_uc_clm2_dn0, var_uc_clm2_dn2, var_uc_clm2_dn6, var_uc_clm2_dn7, var_uc_clm2_dn10, var_uc_clm2_dn11, var_uc_clm2_dn12, var_uc_clm2_dn17,)
    } else {
        (var_uc_clm2, var_uc_clm2_dn0, var_uc_clm2_dn2, var_uc_clm2_dn6, var_uc_clm2_dn7, var_uc_clm2_dn10, var_uc_clm2_dn11, var_uc_clm2_dn12, var_uc_clm2_dn17,)
    }
};
        var_uc_clm2 = assign1890_e1305;
        var_uc_clm2_dn0 = assign1890_e1305_d_n0;
        var_uc_clm2_dn2 = assign1890_e1305_d_n2;
        var_uc_clm2_dn6 = assign1890_e1305_d_n6;
        var_uc_clm2_dn7 = assign1890_e1305_d_n7;
        var_uc_clm2_dn10 = assign1890_e1305_d_n10;
        var_uc_clm2_dn11 = assign1890_e1305_d_n11;
        var_uc_clm2_dn12 = assign1890_e1305_d_n12;
        var_uc_clm2_dn17 = assign1890_e1305_d_n17;
        var_uc_clm2_rv = 0.0;

        let assign1900_e1311: f64 = (var_uc_tnom * 1e-7);
        let assign1900_e1312: f64 = (9.025e-5 + assign1900_e1311);
        let assign1900_e1313: f64 = (var_uc_tnom * assign1900_e1312);
        let assign1900_e1314: f64 = (p.p55 - assign1900_e1313);
        var_egtnom = assign1900_e1314;
        var_egtnom_rv = 0.0;

        var_tfox0 = p.p236;
        var_tfox0_rv = 0.0;

        let assign1920_e1318: f64 = (1.034943e-10 / p.p237);
        var_c_soi = assign1920_e1318;
        var_c_soi_rv = 0.0;

        let assign1930_e1321: f64 = (1.0 / var_c_soi);
        var_c_soi_inv = assign1930_e1321;
        var_c_soi_inv_rv = 0.0;

        let assign1940_e1324: f64 = (3.453133e-11 / var_tfox0);
        var_c_fox0 = assign1940_e1324;
        var_c_fox0_rv = 0.0;

        let assign1950_e1327: f64 = (var_tfox0 / 3.453133e-11);
        var_c_fox0_inv = assign1950_e1327;
        var_c_fox0_inv_rv = 0.0;

        let assign1960_e1330: f64 = (3.453133e-11 / p.p239);
        var_c_box = assign1960_e1330;
        var_c_box_rv = 0.0;

        let assign1970_e1333: f64 = (p.p239 / 3.453133e-11);
        var_c_box_inv = assign1970_e1333;
        var_c_box_inv_rv = 0.0;

        let assign1980_e1336: f64 = (var_c_box_inv + var_c_soi_inv);
        var_c_box_fd_inv = assign1980_e1336;
        var_c_box_fd_inv_rv = 0.0;

        var_lgate = p.p0;
        var_lgate_rv = 0.0;

        let assign2000_e1341: f64 = (2.0 * p.p56);
        let assign2000_e1342: f64 = (var_lgate - assign2000_e1341);
        var_leff = assign2000_e1342;
        var_leff_rv = 0.0;

        let assign2010_e1346: f64 = (2.0 * p.p57);
        let assign2010_e1347: f64 = (var_lgate - assign2010_e1346);
        var_leff_cv = assign2010_e1347;
        var_leff_cv_rv = 0.0;

        let (assign2020_e1353,) = {
    if (p.p40 == 0.0) {
        (var_lgate,)
    } else {
        (var_leff,)
    }
};
        var_lgleff = assign2020_e1353;
        var_lgleff_rv = 0.0;

        let assign2030_e1356: f64 = (var_lgleff * 1000000.0);
        var_lgle = assign2030_e1356;
        var_lgle_rv = 0.0;

        let assign2040_e1359: f64 = (p.p1 / p.p9);
        var_wgate = assign2040_e1359;
        var_wgate_rv = 0.0;

        var_dw = p.p60;
        var_dw_rv = 0.0;

        let (assign2060_e1366,) = {
    if (var_subversion < 1.0) {
        (0.0,)
    } else {
        (p.p295,)
    }
};
        var_dwbt = assign2060_e1366;
        var_dwbt_rv = 0.0;

        let (assign2070_e1372,) = {
    if (var_subversion < 1.0) {
        (p.p60,)
    } else {
        (p.p61,)
    }
};
        var_dwcv = assign2070_e1372;
        var_dwcv_rv = 0.0;

        let assign2080_e1375: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        var_guard8 = assign2080_e1375;
        var_guard8_rv = 0.0;

        let (assign2090_e1383,) = {
    if (var_guard8 != 0.0) {
        let assign2090_e1380: f64 = (2.0 * var_dw);
        let assign2090_e1381: f64 = (var_wgate - assign2090_e1380);
        (assign2090_e1381,)
    } else {
        (var_weff,)
    }
};
        var_weff = assign2090_e1383;
        var_weff_rv = 0.0;

        let (assign2100_e1391,) = {
    if (var_guard8 != 0.0) {
        let assign2100_e1388: f64 = (2.0 * var_dwcv);
        let assign2100_e1389: f64 = (var_wgate - assign2100_e1388);
        (assign2100_e1389,)
    } else {
        (var_weff_cv,)
    }
};
        var_weff_cv = assign2100_e1391;
        var_weff_cv_rv = 0.0;

        let (assign2110_e1406,) = {
    if (var_guard8 == 0.0) {
        let assign2110_e1397: f64 = (p.p18 * var_dwbt);
        let assign2110_e1398: f64 = (var_wgate - assign2110_e1397);
        let assign2110_e1401: f64 = (2.0 - p.p18);
        let assign2110_e1403: f64 = (assign2110_e1401 * var_dw);
        let assign2110_e1404: f64 = (assign2110_e1398 - assign2110_e1403);
        (assign2110_e1404,)
    } else {
        (var_weff,)
    }
};
        var_weff = assign2110_e1406;
        var_weff_rv = 0.0;

        let (assign2120_e1421,) = {
    if (var_guard8 == 0.0) {
        let assign2120_e1412: f64 = (p.p18 * var_dwbt);
        let assign2120_e1413: f64 = (var_wgate - assign2120_e1412);
        let assign2120_e1416: f64 = (2.0 - p.p18);
        let assign2120_e1418: f64 = (assign2120_e1416 * var_dwcv);
        let assign2120_e1419: f64 = (assign2120_e1413 - assign2120_e1418);
        (assign2120_e1419,)
    } else {
        (var_weff_cv,)
    }
};
        var_weff_cv = assign2120_e1421;
        var_weff_cv_rv = 0.0;

        let assign2130_e1424: f64 = (var_weff * p.p9);
        var_weff_nf = assign2130_e1424;
        var_weff_nf_rv = 0.0;

        let assign2140_e1427: f64 = (var_weff_cv * p.p9);
        var_weffcv_nf = assign2140_e1427;
        var_weffcv_nf_rv = 0.0;

        let assign2150_e1430: f64 = (var_wgate * 1000000.0);
        var_wg = assign2150_e1430;
        var_wg_rv = 0.0;

        let assign2160_e1433: f64 = (var_wg * var_lgle);
        var_wl = assign2160_e1433;
        var_wl_rv = 0.0;

        let assign2170_e1439: f64 = (var_lgle).powf(p.p111);
        let assign2170_e1440: f64 = (p.p108 / assign2170_e1439);
        let assign2170_e1441: f64 = (1.0 + assign2170_e1440);
        let assign2170_e1442: f64 = (p.p107 * assign2170_e1441);
        let assign2170_e1447: f64 = (var_wg).powf(p.p110);
        let assign2170_e1448: f64 = (p.p109 / assign2170_e1447);
        let assign2170_e1449: f64 = (1.0 + assign2170_e1448);
        let assign2170_e1450: f64 = (assign2170_e1442 * assign2170_e1449);
        var_muesr = assign2170_e1450;
        var_muesr_rv = 0.0;

        let assign2180_e1461: f64 = if (((var_subversion > 3.0) && (var_mks_nsubp < var_mks_nsubs)) && (p.p72 > 0.0)) { 1.0 } else { 0.0 };
        var_guard9 = assign2180_e1461;
        var_guard9_rv = 0.0;

        let (assign2190_e1465,) = {
    if (var_guard9 != 0.0) {
        (var_mks_nsubs,)
    } else {
        (var_mks_nsubp,)
    }
};
        var_mks_nsubp = assign2190_e1465;
        var_mks_nsubp_rv = 0.0;

        let assign2200_e1471: f64 = (var_wg).powf(p.p75);
        let assign2200_e1472: f64 = (p.p74 / assign2200_e1471);
        let assign2200_e1473: f64 = (1.0 + assign2200_e1472);
        let assign2200_e1474: f64 = (var_mks_nsubp * assign2200_e1473);
        var_nsubpp = assign2200_e1474;
        var_nsubpp_rv = 0.0;

        let assign2210_e1480: f64 = (0.5 * var_lgate);
        let assign2210_e1481: f64 = (p.p62 + assign2210_e1480);
        let assign2210_e1482: f64 = (1.0 / assign2210_e1481);
        let assign2210_e1487: f64 = (0.5 * var_lgate);
        let assign2210_e1488: f64 = (p.p63 + assign2210_e1487);
        let assign2210_e1489: f64 = (1.0 / assign2210_e1488);
        let assign2210_e1490: f64 = (assign2210_e1482 + assign2210_e1489);
        let assign2210_e1491: f64 = (2.0 / assign2210_e1490);
        var_lod_half_ref = assign2210_e1491;
        var_lod_half_ref_rv = 0.0;

        let assign2220_e1495: f64 = (1.3806226e-23 * var_uc_tnom);
        let assign2220_e1496: f64 = (1.6021918e-19 / assign2220_e1495);
        var_betatnom = assign2220_e1496;
        var_betatnom_rv = 0.0;

        let assign2230_e1499: f64 = (1.6021918e-19 * var_mks_nsubb);
        let assign2230_e1501: f64 = (assign2230_e1499 * 1.034943e-10);
        var_qnbulk_esi = assign2230_e1501;
        var_qnbulk_esi_rv = 0.0;

        let assign2240_e1505: f64 = (-p.p247);
        let assign2240_e1506: f64 = (var_lgle).powf(assign2240_e1505);
        let assign2240_e1507: f64 = (p.p244 * assign2240_e1506);
        var_ptl0 = assign2240_e1507;
        var_ptl0_rv = 0.0;

        let assign2250_e1511: f64 = (-p.p252);
        let assign2250_e1512: f64 = (var_lgle).powf(assign2250_e1511);
        let assign2250_e1513: f64 = (p.p251 * assign2250_e1512);
        var_pt40 = assign2250_e1513;
        var_pt40_rv = 0.0;

        let assign2260_e1517: f64 = (var_lgle + var_uc_gdld);
        let assign2260_e1519: f64 = (-p.p249);
        let assign2260_e1520: f64 = (assign2260_e1517).powf(assign2260_e1519);
        let assign2260_e1521: f64 = (p.p248 * assign2260_e1520);
        var_gdl0 = assign2260_e1521;
        var_gdl0_rv = 0.0;

        let assign2270_e1524: f64 = (2.0 * 1.6021918e-19);
        let assign2270_e1526: f64 = (assign2270_e1524 * var_mks_nsti);
        let assign2270_e1528: f64 = (assign2270_e1526 * 1.034943e-10);
        let assign2270_e1529: f64 = (assign2270_e1528).sqrt();
        var_costi00 = assign2270_e1529;
        var_costi00_rv = 0.0;

        let assign2280_e1533: f64 = (var_mks_nsti * var_mks_nsti);
        let assign2280_e1534: f64 = (1.0 / assign2280_e1533);
        var_nsti_p2 = assign2280_e1534;
        var_nsti_p2_rv = 0.0;

        let assign2290_e1538: f64 = (1.0 / var_lgle);
        let assign2290_e1539: f64 = (1.0 + assign2290_e1538);
        let assign2290_e1541: f64 = (assign2290_e1539).powf(p.p91);
        let assign2290_e1543: f64 = (assign2290_e1541 * p.p89);
        var_cnstpgd = assign2290_e1543;
        var_cnstpgd_rv = 0.0;

        var_c0bulk = var_qnbulk_esi;
        var_c0bulk_rv = 0.0;

        var_vfb = p.p68;
        var_vfb_rv = 0.0;

        let assign2320_e1550: f64 = (var_wl).powf(p.p77);
        let assign2320_e1551: f64 = (p.p76 / assign2320_e1550);
        let assign2320_e1552: f64 = (var_lgleff + assign2320_e1551);
        var_lgatesm = assign2320_e1552;
        var_lgatesm_rv = 0.0;

        let assign2330_e1556: f64 = (var_wl).powf(p.p79);
        let assign2330_e1557: f64 = (p.p78 / assign2330_e1556);
        var_dvthsm = assign2330_e1557;
        var_dvthsm_rv = 0.0;

        let assign2340_e1563: f64 = (var_lgatesm * 1000000.0);
        let assign2340_e1565: f64 = (assign2340_e1563).powf(p.p151);
        let assign2340_e1566: f64 = (p.p150 / assign2340_e1565);
        let assign2340_e1567: f64 = (1.0 + assign2340_e1566);
        let assign2340_e1568: f64 = (p.p149 * assign2340_e1567);
        let assign2340_e1570: f64 = assign2340_e1568;
        let assign2340_e1574: f64 = (var_wg).powf(p.p153);
        let assign2340_e1575: f64 = (p.p152 / assign2340_e1574);
        let assign2340_e1576: f64 = (assign2340_e1570 + assign2340_e1575);
        var_uc_wsti = assign2340_e1576;
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
        *var_guard3_slot = var_guard3;
        *var_guard3_rv_slot = var_guard3_rv;
        *var_guard4_slot = var_guard4;
        *var_guard4_rv_slot = var_guard4_rv;
        *var_guard5_slot = var_guard5;
        *var_guard5_rv_slot = var_guard5_rv;
        *var_guard6_slot = var_guard6;
        *var_guard6_rv_slot = var_guard6_rv;
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
        var_guard10_slot: &mut f64,
        var_guard10_rv_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard11_rv_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_guard12_rv_slot: &mut f64,
        var_guard13_slot: &mut f64,
        var_guard13_rv_slot: &mut f64,
        var_guard14_slot: &mut f64,
        var_guard14_rv_slot: &mut f64,
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
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard10_rv: f64 = *var_guard10_rv_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard11_rv: f64 = *var_guard11_rv_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_guard12_rv: f64 = *var_guard12_rv_slot;
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard13_rv: f64 = *var_guard13_rv_slot;
        let mut var_guard14: f64 = *var_guard14_slot;
        let mut var_guard14_rv: f64 = *var_guard14_rv_slot;
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

        let assign2350_e1580: f64 = (var_lgle).powf(p.p192);
        let assign2350_e1582: f64 = (assign2350_e1580 * p.p193);
        let assign2350_e1583: f64 = (1.0 + assign2350_e1582);
        var_clmmod = assign2350_e1583;
        var_clmmod_rv = 0.0;

        let assign2370_e1603: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        var_guard10 = assign2370_e1603;
        var_guard10_rv = 0.0;

        let (assign2380_e1613,) = {
    if (var_guard10 != 0.0) {
        let assign2380_e1609: f64 = (var_wg).powf(p.p131);
        let assign2380_e1610: f64 = (p.p130 / assign2380_e1609);
        let assign2380_e1611: f64 = (1.0 + assign2380_e1610);
        (assign2380_e1611,)
    } else {
        (var_zvgs,)
    }
};
        var_zvgs = assign2380_e1613;
        var_zvgs_rv = 0.0;

        let (assign2390_e1625,) = {
    if (var_guard10 != 0.0) {
        let assign2390_e1620: f64 = (var_lgle).powf(p.p126);
        let assign2390_e1621: f64 = (p.p125 / assign2390_e1620);
        let assign2390_e1622: f64 = (1.0 + assign2390_e1621);
        let assign2390_e1623: f64 = (p.p124 * assign2390_e1622);
        (assign2390_e1623,)
    } else {
        (var_xvbs,)
    }
};
        var_xvbs = assign2390_e1625;
        var_xvbs_rv = 0.0;

        let (assign2400_e1633,) = {
    if (var_guard10 != 0.0) {
        let assign2400_e1630: f64 = (var_lgle + p.p123);
        let assign2400_e1631: f64 = (var_lgle / assign2400_e1630);
        (assign2400_e1631,)
    } else {
        (var_xgate,)
    }
};
        var_xgate = assign2400_e1633;
        var_xgate_rv = 0.0;

        let (assign2410_e1645,) = {
    if (var_guard10 != 0.0) {
        let assign2410_e1640: f64 = (var_lgle).powf(p.p120);
        let assign2410_e1641: f64 = (p.p119 / assign2410_e1640);
        let assign2410_e1642: f64 = (1.0 + assign2410_e1641);
        let assign2410_e1643: f64 = (p.p117 * assign2410_e1642);
        (assign2410_e1643,)
    } else {
        (var_xsub1,)
    }
};
        var_xsub1 = assign2410_e1645;
        var_xsub1_rv = 0.0;

        let (assign2420_e1655,) = {
    if (var_guard10 != 0.0) {
        let assign2420_e1651: f64 = (p.p121 / var_lgle);
        let assign2420_e1652: f64 = (1.0 + assign2420_e1651);
        let assign2420_e1653: f64 = (p.p118 * assign2420_e1652);
        (assign2420_e1653,)
    } else {
        (var_xsub2,)
    }
};
        var_xsub2 = assign2420_e1655;
        var_xsub2_rv = 0.0;

        let (assign2430_e1662, assign2430_e1662_d_n0, assign2430_e1662_d_n2, assign2430_e1662_d_n6, assign2430_e1662_d_n7, assign2430_e1662_d_n10, assign2430_e1662_d_n11, assign2430_e1662_d_n12, assign2430_e1662_d_n17,) = {
    if (var_guard10 == 0.0) {
        let assign2430_e1660: f64 = (var_wg).powf(p.p131);
        (assign2430_e1660, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign2430_e1662;
        var_t2_dn0 = assign2430_e1662_d_n0;
        var_t2_dn2 = assign2430_e1662_d_n2;
        var_t2_dn6 = assign2430_e1662_d_n6;
        var_t2_dn7 = assign2430_e1662_d_n7;
        var_t2_dn10 = assign2430_e1662_d_n10;
        var_t2_dn11 = assign2430_e1662_d_n11;
        var_t2_dn12 = assign2430_e1662_d_n12;
        var_t2_dn17 = assign2430_e1662_d_n17;
        var_t2_rv = 0.0;

        let (assign2440_e1681, assign2440_e1681_d_n0, assign2440_e1681_d_n2, assign2440_e1681_d_n6, assign2440_e1681_d_n7, assign2440_e1681_d_n10, assign2440_e1681_d_n11, assign2440_e1681_d_n12, assign2440_e1681_d_n17,) = {
    if (var_guard10 == 0.0) {
        let assign2440_e1670: f64 = (var_lgle).powf(p.p129);
        let assign2440_e1671: f64 = (p.p128 / assign2440_e1670);
        let assign2440_e1672: f64 = (1.0 + assign2440_e1671);
        let assign2440_e1673: f64 = (p.p127 * assign2440_e1672);
        let assign2440_e1677: f64 = (var_t2 + p.p130);
        let assign2440_e1678: f64 = (var_t2 / assign2440_e1677);
        let assign2440_e1679: f64 = (assign2440_e1673 * assign2440_e1678);
        (assign2440_e1679, (assign2440_e1673 * (((var_t2_dn0 * assign2440_e1677) - (var_t2 * var_t2_dn0)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((var_t2_dn2 * assign2440_e1677) - (var_t2 * var_t2_dn2)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((var_t2_dn6 * assign2440_e1677) - (var_t2 * var_t2_dn6)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((var_t2_dn7 * assign2440_e1677) - (var_t2 * var_t2_dn7)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((var_t2_dn10 * assign2440_e1677) - (var_t2 * var_t2_dn10)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((var_t2_dn11 * assign2440_e1677) - (var_t2 * var_t2_dn11)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((var_t2_dn12 * assign2440_e1677) - (var_t2 * var_t2_dn12)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((var_t2_dn17 * assign2440_e1677) - (var_t2 * var_t2_dn17)) / (assign2440_e1677 * assign2440_e1677))),)
    } else {
        (var_vg2const, var_vg2const_dn0, var_vg2const_dn2, var_vg2const_dn6, var_vg2const_dn7, var_vg2const_dn10, var_vg2const_dn11, var_vg2const_dn12, var_vg2const_dn17,)
    }
};
        var_vg2const = assign2440_e1681;
        var_vg2const_dn0 = assign2440_e1681_d_n0;
        var_vg2const_dn2 = assign2440_e1681_d_n2;
        var_vg2const_dn6 = assign2440_e1681_d_n6;
        var_vg2const_dn7 = assign2440_e1681_d_n7;
        var_vg2const_dn10 = assign2440_e1681_d_n10;
        var_vg2const_dn11 = assign2440_e1681_d_n11;
        var_vg2const_dn12 = assign2440_e1681_d_n12;
        var_vg2const_dn17 = assign2440_e1681_d_n17;
        var_vg2const_rv = 0.0;

        let (assign2450_e1694,) = {
    if (var_guard10 == 0.0) {
        let assign2450_e1689: f64 = (var_lgle).powf(p.p126);
        let assign2450_e1690: f64 = (p.p125 / assign2450_e1689);
        let assign2450_e1691: f64 = (1.0 + assign2450_e1690);
        let assign2450_e1692: f64 = (p.p124 * assign2450_e1691);
        (assign2450_e1692,)
    } else {
        (var_xvbs,)
    }
};
        var_xvbs = assign2450_e1694;
        var_xvbs_rv = 0.0;

        let (assign2460_e1707,) = {
    if (var_guard10 == 0.0) {
        let assign2460_e1702: f64 = (var_lgle).powf(p.p133);
        let assign2460_e1703: f64 = (p.p132 / assign2460_e1702);
        let assign2460_e1704: f64 = (1.0 + assign2460_e1703);
        let assign2460_e1705: f64 = (p.p123 * assign2460_e1704);
        (assign2460_e1705,)
    } else {
        (var_xgate,)
    }
};
        var_xgate = assign2460_e1707;
        var_xgate_rv = 0.0;

        let (assign2470_e1720,) = {
    if (var_guard10 == 0.0) {
        let assign2470_e1715: f64 = (var_lgle).powf(p.p120);
        let assign2470_e1716: f64 = (p.p119 / assign2470_e1715);
        let assign2470_e1717: f64 = (1.0 + assign2470_e1716);
        let assign2470_e1718: f64 = (p.p117 * assign2470_e1717);
        (assign2470_e1718,)
    } else {
        (var_xsub1,)
    }
};
        var_xsub1 = assign2470_e1720;
        var_xsub1_rv = 0.0;

        let (assign2480_e1731,) = {
    if (var_guard10 == 0.0) {
        let assign2480_e1727: f64 = (p.p121 / var_lgle);
        let assign2480_e1728: f64 = (1.0 + assign2480_e1727);
        let assign2480_e1729: f64 = (p.p118 * assign2480_e1728);
        (assign2480_e1729,)
    } else {
        (var_xsub2,)
    }
};
        var_xsub2 = assign2480_e1731;
        var_xsub2_rv = 0.0;

        let assign2490_e1734: f64 = (1000000.0 * var_weffcv_nf);
        let assign2490_e1736: f64 = (assign2490_e1734 * p.p65);
        let assign2490_e1739: f64 = (var_lgle).powf(p.p66);
        let assign2490_e1740: f64 = (assign2490_e1736 / assign2490_e1739);
        var_cqyb0 = assign2490_e1740;
        var_cqyb0_rv = 0.0;

        let assign2500_e1746: f64 = (var_lgle).powf(p.p136);
        let assign2500_e1747: f64 = (p.p135 / assign2500_e1746);
        let assign2500_e1748: f64 = (1.0 + assign2500_e1747);
        let assign2500_e1749: f64 = (p.p134 * assign2500_e1748);
        var_vfbsub0 = assign2500_e1749;
        var_vfbsub0_rv = 0.0;

        let assign2510_e1752: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        var_guard11 = assign2510_e1752;
        var_guard11_rv = 0.0;

        let (assign2520_e1764,) = {
    if (var_guard11 != 0.0) {
        let assign2520_e1759: f64 = (var_lgle).powf(p.p129);
        let assign2520_e1760: f64 = (p.p128 / assign2520_e1759);
        let assign2520_e1761: f64 = (1.0 + assign2520_e1760);
        let assign2520_e1762: f64 = (p.p127 * assign2520_e1761);
        (assign2520_e1762,)
    } else {
        (var_uc_svgs,)
    }
};
        var_uc_svgs = assign2520_e1764;
        var_uc_svgs_rv = 0.0;

        let assign2530_e1767: f64 = (p.p115 * var_lgle);
        let assign2530_e1769: f64 = (assign2530_e1767 * p.p114);
        let assign2530_e1772: f64 = (p.p115 * var_lgle);
        let assign2530_e1774: f64 = (assign2530_e1772 + p.p114);
        let assign2530_e1775: f64 = (assign2530_e1769 / assign2530_e1774);
        let assign2530_e1777: f64 = (assign2530_e1775 + p.p116);
        let assign2530_e1779: f64 = (assign2530_e1777 + 1e-50);
        var_ddlte = assign2530_e1779;
        var_ddlte_rv = 0.0;

        let assign2540_e1782: f64 = if var_ddlte < 3.0 { 1.0 } else { 0.0 };
        var_guard12 = assign2540_e1782;
        var_guard12_rv = 0.0;

        let (assign2550_e1786,) = {
    if (var_guard12 != 0.0) {
        (3.0,)
    } else {
        (var_ddlte,)
    }
};
        var_ddlte = assign2550_e1786;
        var_ddlte_rv = 0.0;

        let assign2560_e1789: f64 = (p.p50 * p.p253);
        var_vgs_min = assign2560_e1789;
        var_vgs_min_rv = 0.0;

        let assign2570_e1791: f64 = if param_given[168] { 1.0 } else { 0.0 };
        var_cgbo_given = assign2570_e1791;
        var_cgbo_given_rv = 0.0;

        let assign2580_e1793: f64 = if param_given[169] { 1.0 } else { 0.0 };
        var_cgdo_given = assign2580_e1793;
        var_cgdo_given_rv = 0.0;

        let assign2590_e1795: f64 = if param_given[170] { 1.0 } else { 0.0 };
        var_cgso_given = assign2590_e1795;
        var_cgso_given_rv = 0.0;

        let assign2600_e1797: f64 = if param_given[294] { 1.0 } else { 0.0 };
        var_cbtbp_given = assign2600_e1797;
        var_cbtbp_given_rv = 0.0;

        let assign2610_e1799: f64 = if param_given[293] { 1.0 } else { 0.0 };
        var_cbtbn_given = assign2610_e1799;
        var_cbtbn_given_rv = 0.0;

        let assign2620_e1801: f64 = if param_given[13] { 1.0 } else { 0.0 };
        var_pdbcp_given = assign2620_e1801;
        var_pdbcp_given_rv = 0.0;

        let assign2630_e1803: f64 = if param_given[14] { 1.0 } else { 0.0 };
        var_psbcp_given = assign2630_e1803;
        var_psbcp_given_rv = 0.0;

        let assign2640_e1805: f64 = if param_given[23] { 1.0 } else { 0.0 };
        var_abtp_given = assign2640_e1805;
        var_abtp_given_rv = 0.0;

        let assign2650_e1807: f64 = if param_given[22] { 1.0 } else { 0.0 };
        var_abtn_given = assign2650_e1807;
        var_abtn_given_rv = 0.0;

        let assign2660_e1809: f64 = if param_given[16] { 1.0 } else { 0.0 };
        var_temp_given = assign2660_e1809;
        var_temp_given_rv = 0.0;

        let (assign2670_e1815,) = {
    if (p.p17 == 0.0) {
        (0.0,)
    } else {
        (1.0,)
    }
};
        var_dtemp_given = assign2670_e1815;
        var_dtemp_given_rv = 0.0;

        var_mfactor = 1.0;
        var_mfactor_rv = 0.0;

        let assign2690_e1819: f64 = 0.0;
        var_gjmin = assign2690_e1819;
        var_gjmin_rv = 0.0;

        var_uc_pdbcp = p.p13;
        var_uc_pdbcp_rv = 0.0;

        var_uc_psbcp = p.p14;
        var_uc_psbcp_rv = 0.0;

        let assign2720_e1824: f64 = (p.p16 + 273.15);
        var_uc_temp = assign2720_e1824;
        var_uc_temp_rv = 0.0;

        let assign2740_e1833: f64 = (var_mfactor * var_weffcv_nf);
        let assign2740_e1834: f64 = (var_mks_cth0 * assign2740_e1833);
        var_cth = assign2740_e1834;
        var_cth_rv = 0.0;

        let assign2750_e1853: f64 = if (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p9 == 1.0) || ((p.p9 > 1.0) && (p.p12 > 0.0)))) { 1.0 } else { 0.0 };
        var_guard13 = assign2750_e1853;
        var_guard13_rv = 0.0;

        let (assign2760_e1857, assign2760_e1857_d_n0, assign2760_e1857_d_n2, assign2760_e1857_d_n6, assign2760_e1857_d_n7, assign2760_e1857_d_n10, assign2760_e1857_d_n11, assign2760_e1857_d_n12, assign2760_e1857_d_n17,) = {
    if (var_guard13 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign2760_e1857;
        var_t1_dn0 = assign2760_e1857_d_n0;
        var_t1_dn2 = assign2760_e1857_d_n2;
        var_t1_dn6 = assign2760_e1857_d_n6;
        var_t1_dn7 = assign2760_e1857_d_n7;
        var_t1_dn10 = assign2760_e1857_d_n10;
        var_t1_dn11 = assign2760_e1857_d_n11;
        var_t1_dn12 = assign2760_e1857_d_n12;
        var_t1_dn17 = assign2760_e1857_d_n17;
        var_t1_rv = 0.0;

        let (assign2770_e1861,) = {
    if (var_guard13 != 0.0) {
        (0.0,)
    } else {
        (var_i,)
    }
};
        var_i = assign2770_e1861;
        var_i_rv = 0.0;

        let mut assign2780_loop_guard: usize = 0;
        while {
            let assign2780_cond_e1866: f64 = if ((var_guard13 != 0.0) && (var_i < p.p9)) { 1.0 } else { 0.0 };
            assign2780_cond_e1866 != 0.0
        } {
            assign2780_loop_guard += 1;
            assert!(assign2780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign2780_body0_e1898, assign2780_body0_e1898_d_n0, assign2780_body0_e1898_d_n2, assign2780_body0_e1898_d_n6, assign2780_body0_e1898_d_n7, assign2780_body0_e1898_d_n10, assign2780_body0_e1898_d_n11, assign2780_body0_e1898_d_n12, assign2780_body0_e1898_d_n17,) = {
    if (var_guard13 != 0.0) {
        let assign2780_body0_e1873: f64 = (0.5 * var_lgate);
        let assign2780_body0_e1874: f64 = (p.p10 + assign2780_body0_e1873);
        let assign2780_body0_e1878: f64 = (p.p12 + var_lgate);
        let assign2780_body0_e1879: f64 = (var_i * assign2780_body0_e1878);
        let assign2780_body0_e1880: f64 = (assign2780_body0_e1874 + assign2780_body0_e1879);
        let assign2780_body0_e1881: f64 = (1.0 / assign2780_body0_e1880);
        let assign2780_body0_e1882: f64 = (var_t1 + assign2780_body0_e1881);
        let assign2780_body0_e1887: f64 = (0.5 * var_lgate);
        let assign2780_body0_e1888: f64 = (p.p11 + assign2780_body0_e1887);
        let assign2780_body0_e1892: f64 = (p.p12 + var_lgate);
        let assign2780_body0_e1893: f64 = (var_i * assign2780_body0_e1892);
        let assign2780_body0_e1894: f64 = (assign2780_body0_e1888 + assign2780_body0_e1893);
        let assign2780_body0_e1895: f64 = (1.0 / assign2780_body0_e1894);
        let assign2780_body0_e1896: f64 = (assign2780_body0_e1882 + assign2780_body0_e1895);
        (assign2780_body0_e1896, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
            var_t1 = assign2780_body0_e1898;
            var_t1_dn0 = assign2780_body0_e1898_d_n0;
            var_t1_dn2 = assign2780_body0_e1898_d_n2;
            var_t1_dn6 = assign2780_body0_e1898_d_n6;
            var_t1_dn7 = assign2780_body0_e1898_d_n7;
            var_t1_dn10 = assign2780_body0_e1898_d_n10;
            var_t1_dn11 = assign2780_body0_e1898_d_n11;
            var_t1_dn12 = assign2780_body0_e1898_d_n12;
            var_t1_dn17 = assign2780_body0_e1898_d_n17;
            var_t1_rv = 0.0;
            let (assign2780_body1_e1904,) = {
    if (var_guard13 != 0.0) {
        let assign2780_body1_e1902: f64 = (var_i + 1.0);
        (assign2780_body1_e1902,)
    } else {
        (var_i,)
    }
};
            var_i = assign2780_body1_e1904;
            var_i_rv = 0.0;
        }

        let (assign2790_e1912, assign2790_e1912_d_n0, assign2790_e1912_d_n2, assign2790_e1912_d_n6, assign2790_e1912_d_n7, assign2790_e1912_d_n10, assign2790_e1912_d_n11, assign2790_e1912_d_n12, assign2790_e1912_d_n17,) = {
    if (var_guard13 != 0.0) {
        let assign2790_e1908: f64 = (2.0 * p.p9);
        let assign2790_e1910: f64 = (assign2790_e1908 / var_t1);
        (assign2790_e1910, (-((assign2790_e1908 * var_t1_dn0) / (var_t1 * var_t1))), (-((assign2790_e1908 * var_t1_dn2) / (var_t1 * var_t1))), (-((assign2790_e1908 * var_t1_dn6) / (var_t1 * var_t1))), (-((assign2790_e1908 * var_t1_dn7) / (var_t1 * var_t1))), (-((assign2790_e1908 * var_t1_dn10) / (var_t1 * var_t1))), (-((assign2790_e1908 * var_t1_dn11) / (var_t1 * var_t1))), (-((assign2790_e1908 * var_t1_dn12) / (var_t1 * var_t1))), (-((assign2790_e1908 * var_t1_dn17) / (var_t1 * var_t1))),)
    } else {
        (var_lod_half, var_lod_half_dn0, var_lod_half_dn2, var_lod_half_dn6, var_lod_half_dn7, var_lod_half_dn10, var_lod_half_dn11, var_lod_half_dn12, var_lod_half_dn17,)
    }
};
        var_lod_half = assign2790_e1912;
        var_lod_half_dn0 = assign2790_e1912_d_n0;
        var_lod_half_dn2 = assign2790_e1912_d_n2;
        var_lod_half_dn6 = assign2790_e1912_d_n6;
        var_lod_half_dn7 = assign2790_e1912_d_n7;
        var_lod_half_dn10 = assign2790_e1912_d_n10;
        var_lod_half_dn11 = assign2790_e1912_d_n11;
        var_lod_half_dn12 = assign2790_e1912_d_n12;
        var_lod_half_dn17 = assign2790_e1912_d_n17;
        var_lod_half_rv = 0.0;

        let (assign2800_e1917, assign2800_e1917_d_n0, assign2800_e1917_d_n2, assign2800_e1917_d_n6, assign2800_e1917_d_n7, assign2800_e1917_d_n10, assign2800_e1917_d_n11, assign2800_e1917_d_n12, assign2800_e1917_d_n17,) = {
    if (var_guard13 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_lod_half, var_lod_half_dn0, var_lod_half_dn2, var_lod_half_dn6, var_lod_half_dn7, var_lod_half_dn10, var_lod_half_dn11, var_lod_half_dn12, var_lod_half_dn17,)
    }
};
        var_lod_half = assign2800_e1917;
        var_lod_half_dn0 = assign2800_e1917_d_n0;
        var_lod_half_dn2 = assign2800_e1917_d_n2;
        var_lod_half_dn6 = assign2800_e1917_d_n6;
        var_lod_half_dn7 = assign2800_e1917_d_n7;
        var_lod_half_dn10 = assign2800_e1917_d_n10;
        var_lod_half_dn11 = assign2800_e1917_d_n11;
        var_lod_half_dn12 = assign2800_e1917_d_n12;
        var_lod_half_dn17 = assign2800_e1917_d_n17;
        var_lod_half_rv = 0.0;

        let assign2810_e1920: f64 = if var_lod_half > 0.0 { 1.0 } else { 0.0 };
        var_guard14 = assign2810_e1920;
        var_guard14_rv = 0.0;

        let (assign2820_e1928, assign2820_e1928_d_n0, assign2820_e1928_d_n2, assign2820_e1928_d_n6, assign2820_e1928_d_n7, assign2820_e1928_d_n10, assign2820_e1928_d_n11, assign2820_e1928_d_n12, assign2820_e1928_d_n17,) = {
    if (var_guard14 != 0.0) {
        let assign2820_e1925: f64 = (1.0 + p.p162);
        let assign2820_e1926: f64 = (1.0 / assign2820_e1925);
        (assign2820_e1926, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign2820_e1928;
        var_t1_dn0 = assign2820_e1928_d_n0;
        var_t1_dn2 = assign2820_e1928_d_n2;
        var_t1_dn6 = assign2820_e1928_d_n6;
        var_t1_dn7 = assign2820_e1928_d_n7;
        var_t1_dn10 = assign2820_e1928_d_n10;
        var_t1_dn11 = assign2820_e1928_d_n11;
        var_t1_dn12 = assign2820_e1928_d_n12;
        var_t1_dn17 = assign2820_e1928_d_n17;
        var_t1_rv = 0.0;

        let (assign2830_e1936, assign2830_e1936_d_n0, assign2830_e1936_d_n2, assign2830_e1936_d_n6, assign2830_e1936_d_n7, assign2830_e1936_d_n10, assign2830_e1936_d_n11, assign2830_e1936_d_n12, assign2830_e1936_d_n17,) = {
    if (var_guard14 != 0.0) {
        let assign2830_e1932: f64 = (p.p161 / var_lod_half);
        let assign2830_e1934: f64 = (assign2830_e1932).powf(p.p163);
        (assign2830_e1934, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn0) / (var_lod_half * var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * var_lod_half_dn0) / (var_lod_half * var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn2) / (var_lod_half * var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * var_lod_half_dn2) / (var_lod_half * var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn6) / (var_lod_half * var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * var_lod_half_dn6) / (var_lod_half * var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn7) / (var_lod_half * var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * var_lod_half_dn7) / (var_lod_half * var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn10) / (var_lod_half * var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * var_lod_half_dn10) / (var_lod_half * var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn11) / (var_lod_half * var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * var_lod_half_dn11) / (var_lod_half * var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn12) / (var_lod_half * var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * var_lod_half_dn12) / (var_lod_half * var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn17) / (var_lod_half * var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * var_lod_half_dn17) / (var_lod_half * var_lod_half))) / assign2830_e1932))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign2830_e1936;
        var_t2_dn0 = assign2830_e1936_d_n0;
        var_t2_dn2 = assign2830_e1936_d_n2;
        var_t2_dn6 = assign2830_e1936_d_n6;
        var_t2_dn7 = assign2830_e1936_d_n7;
        var_t2_dn10 = assign2830_e1936_d_n10;
        var_t2_dn11 = assign2830_e1936_d_n11;
        var_t2_dn12 = assign2830_e1936_d_n12;
        var_t2_dn17 = assign2830_e1936_d_n17;
        var_t2_rv = 0.0;

        let (assign2840_e1944, assign2840_e1944_d_n0, assign2840_e1944_d_n2, assign2840_e1944_d_n6, assign2840_e1944_d_n7, assign2840_e1944_d_n10, assign2840_e1944_d_n11, assign2840_e1944_d_n12, assign2840_e1944_d_n17,) = {
    if (var_guard14 != 0.0) {
        let assign2840_e1940: f64 = (p.p161 / var_lod_half_ref);
        let assign2840_e1942: f64 = (assign2840_e1940).powf(p.p163);
        (assign2840_e1942, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign2840_e1944;
        var_t3_dn0 = assign2840_e1944_d_n0;
        var_t3_dn2 = assign2840_e1944_d_n2;
        var_t3_dn6 = assign2840_e1944_d_n6;
        var_t3_dn7 = assign2840_e1944_d_n7;
        var_t3_dn10 = assign2840_e1944_d_n10;
        var_t3_dn11 = assign2840_e1944_d_n11;
        var_t3_dn12 = assign2840_e1944_d_n12;
        var_t3_dn17 = assign2840_e1944_d_n17;
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
        *var_guard10_slot = var_guard10;
        *var_guard10_rv_slot = var_guard10_rv;
        *var_guard11_slot = var_guard11;
        *var_guard11_rv_slot = var_guard11_rv;
        *var_guard12_slot = var_guard12;
        *var_guard12_rv_slot = var_guard12_rv;
        *var_guard13_slot = var_guard13;
        *var_guard13_rv_slot = var_guard13_rv;
        *var_guard14_slot = var_guard14;
        *var_guard14_rv_slot = var_guard14_rv;
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
        var_guard14: f64,
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
        var_guard15_slot: &mut f64,
        var_guard15_rv_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_guard16_rv_slot: &mut f64,
        var_guard17_slot: &mut f64,
        var_guard17_rv_slot: &mut f64,
        var_guard18_slot: &mut f64,
        var_guard18_rv_slot: &mut f64,
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
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_guard15_rv: f64 = *var_guard15_rv_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard16_rv: f64 = *var_guard16_rv_slot;
        let mut var_guard17: f64 = *var_guard17_slot;
        let mut var_guard17_rv: f64 = *var_guard17_rv_slot;
        let mut var_guard18: f64 = *var_guard18_slot;
        let mut var_guard18_rv: f64 = *var_guard18_rv_slot;
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

        let (assign2850_e1960, assign2850_e1960_d_n0, assign2850_e1960_d_n2, assign2850_e1960_d_n6, assign2850_e1960_d_n7, assign2850_e1960_d_n10, assign2850_e1960_d_n11, assign2850_e1960_d_n12, assign2850_e1960_d_n17,) = {
    if (var_guard14 != 0.0) {
        let assign2850_e1950: f64 = (var_t1 * var_t2);
        let assign2850_e1951: f64 = (1.0 + assign2850_e1950);
        let assign2850_e1952: f64 = (var_nsubpp * assign2850_e1951);
        let assign2850_e1956: f64 = (var_t1 * var_t3);
        let assign2850_e1957: f64 = (1.0 + assign2850_e1956);
        let assign2850_e1958: f64 = (assign2850_e1952 / assign2850_e1957);
        (assign2850_e1958, ((((var_nsubpp * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0))) * assign2850_e1957) - (assign2850_e1952 * ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)))) / (assign2850_e1957 * assign2850_e1957)), ((((var_nsubpp * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2))) * assign2850_e1957) - (assign2850_e1952 * ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)))) / (assign2850_e1957 * assign2850_e1957)), ((((var_nsubpp * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6))) * assign2850_e1957) - (assign2850_e1952 * ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)))) / (assign2850_e1957 * assign2850_e1957)), ((((var_nsubpp * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7))) * assign2850_e1957) - (assign2850_e1952 * ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)))) / (assign2850_e1957 * assign2850_e1957)), ((((var_nsubpp * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10))) * assign2850_e1957) - (assign2850_e1952 * ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)))) / (assign2850_e1957 * assign2850_e1957)), ((((var_nsubpp * ((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11))) * assign2850_e1957) - (assign2850_e1952 * ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)))) / (assign2850_e1957 * assign2850_e1957)), ((((var_nsubpp * ((var_t1_dn12 * var_t2) + (var_t1 * var_t2_dn12))) * assign2850_e1957) - (assign2850_e1952 * ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)))) / (assign2850_e1957 * assign2850_e1957)), ((((var_nsubpp * ((var_t1_dn17 * var_t2) + (var_t1 * var_t2_dn17))) * assign2850_e1957) - (assign2850_e1952 * ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)))) / (assign2850_e1957 * assign2850_e1957)),)
    } else {
        (var_nsubps, var_nsubps_dn0, var_nsubps_dn2, var_nsubps_dn6, var_nsubps_dn7, var_nsubps_dn10, var_nsubps_dn11, var_nsubps_dn12, var_nsubps_dn17,)
    }
};
        var_nsubps = assign2850_e1960;
        var_nsubps_dn0 = assign2850_e1960_d_n0;
        var_nsubps_dn2 = assign2850_e1960_d_n2;
        var_nsubps_dn6 = assign2850_e1960_d_n6;
        var_nsubps_dn7 = assign2850_e1960_d_n7;
        var_nsubps_dn10 = assign2850_e1960_d_n10;
        var_nsubps_dn11 = assign2850_e1960_d_n11;
        var_nsubps_dn12 = assign2850_e1960_d_n12;
        var_nsubps_dn17 = assign2850_e1960_d_n17;
        var_nsubps_rv = 0.0;

        let (assign2860_e1965, assign2860_e1965_d_n0, assign2860_e1965_d_n2, assign2860_e1965_d_n6, assign2860_e1965_d_n7, assign2860_e1965_d_n10, assign2860_e1965_d_n11, assign2860_e1965_d_n12, assign2860_e1965_d_n17,) = {
    if (var_guard14 == 0.0) {
        (var_nsubpp, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nsubps, var_nsubps_dn0, var_nsubps_dn2, var_nsubps_dn6, var_nsubps_dn7, var_nsubps_dn10, var_nsubps_dn11, var_nsubps_dn12, var_nsubps_dn17,)
    }
};
        var_nsubps = assign2860_e1965;
        var_nsubps_dn0 = assign2860_e1965_d_n0;
        var_nsubps_dn2 = assign2860_e1965_d_n2;
        var_nsubps_dn6 = assign2860_e1965_d_n6;
        var_nsubps_dn7 = assign2860_e1965_d_n7;
        var_nsubps_dn10 = assign2860_e1965_d_n10;
        var_nsubps_dn11 = assign2860_e1965_d_n11;
        var_nsubps_dn12 = assign2860_e1965_d_n12;
        var_nsubps_dn17 = assign2860_e1965_d_n17;
        var_nsubps_rv = 0.0;

        let assign2870_e1970: f64 = (var_wg).powf(p.p200);
        let assign2870_e1971: f64 = (p.p199 / assign2870_e1970);
        let assign2870_e1972: f64 = (1.0 + assign2870_e1971);
        let assign2870_e1977: f64 = (var_lgle).powf(p.p203);
        let assign2870_e1978: f64 = (p.p202 / assign2870_e1977);
        let assign2870_e1979: f64 = (1.0 + assign2870_e1978);
        let assign2870_e1980: f64 = (assign2870_e1972 * assign2870_e1979);
        var_t2 = assign2870_e1980;
        var_t2_dn0 = 0.0;
        var_t2_dn2 = 0.0;
        var_t2_dn6 = 0.0;
        var_t2_dn7 = 0.0;
        var_t2_dn10 = 0.0;
        var_t2_dn11 = 0.0;
        var_t2_dn12 = 0.0;
        var_t2_dn17 = 0.0;
        var_t2_rv = 0.0;

        let assign2880_e1983: f64 = (var_mks_nsubcmax / var_mks_nsubs);
        var_t3 = assign2880_e1983;
        var_t3_dn0 = 0.0;
        var_t3_dn2 = 0.0;
        var_t3_dn6 = 0.0;
        var_t3_dn7 = 0.0;
        var_t3_dn10 = 0.0;
        var_t3_dn11 = 0.0;
        var_t3_dn12 = 0.0;
        var_t3_dn17 = 0.0;
        var_t3_rv = 0.0;

        let assign2890_e1986: f64 = (var_t3 - var_t2);
        let assign2890_e1988: f64 = (assign2890_e1986 - 0.01);
        var_tmf1 = assign2890_e1988;
        var_tmf1_dn0 = (var_t3_dn0 - var_t2_dn0);
        var_tmf1_dn2 = (var_t3_dn2 - var_t2_dn2);
        var_tmf1_dn6 = (var_t3_dn6 - var_t2_dn6);
        var_tmf1_dn7 = (var_t3_dn7 - var_t2_dn7);
        var_tmf1_dn10 = (var_t3_dn10 - var_t2_dn10);
        var_tmf1_dn11 = (var_t3_dn11 - var_t2_dn11);
        var_tmf1_dn12 = (var_t3_dn12 - var_t2_dn12);
        var_tmf1_dn17 = (var_t3_dn17 - var_t2_dn17);
        var_tmf1_rv = 0.0;

        let assign2900_e1991: f64 = (4.0 * var_t3);
        let assign2900_e1993: f64 = (assign2900_e1991 * 0.01);
        var_tmf2 = assign2900_e1993;
        var_tmf2_dn0 = ((4.0 * var_t3_dn0) * 0.01);
        var_tmf2_dn2 = ((4.0 * var_t3_dn2) * 0.01);
        var_tmf2_dn6 = ((4.0 * var_t3_dn6) * 0.01);
        var_tmf2_dn7 = ((4.0 * var_t3_dn7) * 0.01);
        var_tmf2_dn10 = ((4.0 * var_t3_dn10) * 0.01);
        var_tmf2_dn11 = ((4.0 * var_t3_dn11) * 0.01);
        var_tmf2_dn12 = ((4.0 * var_t3_dn12) * 0.01);
        var_tmf2_dn17 = ((4.0 * var_t3_dn17) * 0.01);
        var_tmf2_rv = 0.0;

        let (assign2910_e2000, assign2910_e2000_d_n0, assign2910_e2000_d_n2, assign2910_e2000_d_n6, assign2910_e2000_d_n7, assign2910_e2000_d_n10, assign2910_e2000_d_n11, assign2910_e2000_d_n12, assign2910_e2000_d_n17,) = {
    if (var_tmf2 > 0.0) {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    } else {
        let assign2910_e1999: f64 = (-var_tmf2);
        (assign2910_e1999, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
    }
};
        var_tmf2 = assign2910_e2000;
        var_tmf2_dn0 = assign2910_e2000_d_n0;
        var_tmf2_dn2 = assign2910_e2000_d_n2;
        var_tmf2_dn6 = assign2910_e2000_d_n6;
        var_tmf2_dn7 = assign2910_e2000_d_n7;
        var_tmf2_dn10 = assign2910_e2000_d_n10;
        var_tmf2_dn11 = assign2910_e2000_d_n11;
        var_tmf2_dn12 = assign2910_e2000_d_n12;
        var_tmf2_dn17 = assign2910_e2000_d_n17;
        var_tmf2_rv = 0.0;

        let assign2920_e2003: f64 = (var_tmf1 * var_tmf1);
        let assign2920_e2005: f64 = (assign2920_e2003 + var_tmf2);
        let assign2920_e2006: f64 = (assign2920_e2005).sqrt();
        var_tmf2 = assign2920_e2006;
        var_tmf2_dn0 = ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign2920_e2006));
        var_tmf2_dn2 = ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign2920_e2006));
        var_tmf2_dn6 = ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign2920_e2006));
        var_tmf2_dn7 = ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign2920_e2006));
        var_tmf2_dn10 = ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign2920_e2006));
        var_tmf2_dn11 = ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign2920_e2006));
        var_tmf2_dn12 = ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign2920_e2006));
        var_tmf2_dn17 = ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign2920_e2006));
        var_tmf2_rv = 0.0;

        let assign2930_e2011: f64 = (var_tmf1 + var_tmf2);
        let assign2930_e2012: f64 = (0.5 * assign2930_e2011);
        let assign2930_e2013: f64 = (var_t3 - assign2930_e2012);
        var_t1 = assign2930_e2013;
        var_t1_dn0 = (var_t3_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)));
        var_t1_dn2 = (var_t3_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)));
        var_t1_dn6 = (var_t3_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)));
        var_t1_dn7 = (var_t3_dn7 - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)));
        var_t1_dn10 = (var_t3_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)));
        var_t1_dn11 = (var_t3_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)));
        var_t1_dn12 = (var_t3_dn12 - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12)));
        var_t1_dn17 = (var_t3_dn17 - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17)));
        var_t1_rv = 0.0;

        let assign2940_e2016: f64 = (var_mks_nsubs * var_t1);
        var_uc_nsubs = assign2940_e2016;
        var_uc_nsubs_dn0 = (var_mks_nsubs * var_t1_dn0);
        var_uc_nsubs_dn2 = (var_mks_nsubs * var_t1_dn2);
        var_uc_nsubs_dn6 = (var_mks_nsubs * var_t1_dn6);
        var_uc_nsubs_dn7 = (var_mks_nsubs * var_t1_dn7);
        var_uc_nsubs_dn10 = (var_mks_nsubs * var_t1_dn10);
        var_uc_nsubs_dn11 = (var_mks_nsubs * var_t1_dn11);
        var_uc_nsubs_dn12 = (var_mks_nsubs * var_t1_dn12);
        var_uc_nsubs_dn17 = (var_mks_nsubs * var_t1_dn17);
        var_uc_nsubs_rv = 0.0;

        let assign2950_e2019: f64 = if var_lod_half > 0.0 { 1.0 } else { 0.0 };
        var_guard15 = assign2950_e2019;
        var_guard15_rv = 0.0;

        let (assign2960_e2027, assign2960_e2027_d_n0, assign2960_e2027_d_n2, assign2960_e2027_d_n6, assign2960_e2027_d_n7, assign2960_e2027_d_n10, assign2960_e2027_d_n11, assign2960_e2027_d_n12, assign2960_e2027_d_n17,) = {
    if (var_guard15 != 0.0) {
        let assign2960_e2024: f64 = (1.0 + p.p165);
        let assign2960_e2025: f64 = (1.0 / assign2960_e2024);
        (assign2960_e2025, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign2960_e2027;
        var_t1_dn0 = assign2960_e2027_d_n0;
        var_t1_dn2 = assign2960_e2027_d_n2;
        var_t1_dn6 = assign2960_e2027_d_n6;
        var_t1_dn7 = assign2960_e2027_d_n7;
        var_t1_dn10 = assign2960_e2027_d_n10;
        var_t1_dn11 = assign2960_e2027_d_n11;
        var_t1_dn12 = assign2960_e2027_d_n12;
        var_t1_dn17 = assign2960_e2027_d_n17;
        var_t1_rv = 0.0;

        let (assign2970_e2035, assign2970_e2035_d_n0, assign2970_e2035_d_n2, assign2970_e2035_d_n6, assign2970_e2035_d_n7, assign2970_e2035_d_n10, assign2970_e2035_d_n11, assign2970_e2035_d_n12, assign2970_e2035_d_n17,) = {
    if (var_guard15 != 0.0) {
        let assign2970_e2031: f64 = (p.p164 / var_lod_half);
        let assign2970_e2033: f64 = (assign2970_e2031).powf(p.p166);
        (assign2970_e2033, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn0) / (var_lod_half * var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * var_lod_half_dn0) / (var_lod_half * var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn2) / (var_lod_half * var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * var_lod_half_dn2) / (var_lod_half * var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn6) / (var_lod_half * var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * var_lod_half_dn6) / (var_lod_half * var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn7) / (var_lod_half * var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * var_lod_half_dn7) / (var_lod_half * var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn10) / (var_lod_half * var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * var_lod_half_dn10) / (var_lod_half * var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn11) / (var_lod_half * var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * var_lod_half_dn11) / (var_lod_half * var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn12) / (var_lod_half * var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * var_lod_half_dn12) / (var_lod_half * var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn17) / (var_lod_half * var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * var_lod_half_dn17) / (var_lod_half * var_lod_half))) / assign2970_e2031))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign2970_e2035;
        var_t2_dn0 = assign2970_e2035_d_n0;
        var_t2_dn2 = assign2970_e2035_d_n2;
        var_t2_dn6 = assign2970_e2035_d_n6;
        var_t2_dn7 = assign2970_e2035_d_n7;
        var_t2_dn10 = assign2970_e2035_d_n10;
        var_t2_dn11 = assign2970_e2035_d_n11;
        var_t2_dn12 = assign2970_e2035_d_n12;
        var_t2_dn17 = assign2970_e2035_d_n17;
        var_t2_rv = 0.0;

        let (assign2980_e2043, assign2980_e2043_d_n0, assign2980_e2043_d_n2, assign2980_e2043_d_n6, assign2980_e2043_d_n7, assign2980_e2043_d_n10, assign2980_e2043_d_n11, assign2980_e2043_d_n12, assign2980_e2043_d_n17,) = {
    if (var_guard15 != 0.0) {
        let assign2980_e2039: f64 = (p.p164 / var_lod_half_ref);
        let assign2980_e2041: f64 = (assign2980_e2039).powf(p.p166);
        (assign2980_e2041, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign2980_e2043;
        var_t3_dn0 = assign2980_e2043_d_n0;
        var_t3_dn2 = assign2980_e2043_d_n2;
        var_t3_dn6 = assign2980_e2043_d_n6;
        var_t3_dn7 = assign2980_e2043_d_n7;
        var_t3_dn10 = assign2980_e2043_d_n10;
        var_t3_dn11 = assign2980_e2043_d_n11;
        var_t3_dn12 = assign2980_e2043_d_n12;
        var_t3_dn17 = assign2980_e2043_d_n17;
        var_t3_rv = 0.0;

        let (assign2990_e2059, assign2990_e2059_d_n0, assign2990_e2059_d_n2, assign2990_e2059_d_n6, assign2990_e2059_d_n7, assign2990_e2059_d_n10, assign2990_e2059_d_n11, assign2990_e2059_d_n12, assign2990_e2059_d_n17,) = {
    if (var_guard15 != 0.0) {
        let assign2990_e2049: f64 = (var_t1 * var_t2);
        let assign2990_e2050: f64 = (1.0 + assign2990_e2049);
        let assign2990_e2051: f64 = (var_uc_nsubs * assign2990_e2050);
        let assign2990_e2055: f64 = (var_t1 * var_t3);
        let assign2990_e2056: f64 = (1.0 + assign2990_e2055);
        let assign2990_e2057: f64 = (assign2990_e2051 / assign2990_e2056);
        (assign2990_e2057, (((((var_uc_nsubs_dn0 * assign2990_e2050) + (var_uc_nsubs * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0)))) * assign2990_e2056) - (assign2990_e2051 * ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)))) / (assign2990_e2056 * assign2990_e2056)), (((((var_uc_nsubs_dn2 * assign2990_e2050) + (var_uc_nsubs * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2)))) * assign2990_e2056) - (assign2990_e2051 * ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)))) / (assign2990_e2056 * assign2990_e2056)), (((((var_uc_nsubs_dn6 * assign2990_e2050) + (var_uc_nsubs * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * assign2990_e2056) - (assign2990_e2051 * ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)))) / (assign2990_e2056 * assign2990_e2056)), (((((var_uc_nsubs_dn7 * assign2990_e2050) + (var_uc_nsubs * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * assign2990_e2056) - (assign2990_e2051 * ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)))) / (assign2990_e2056 * assign2990_e2056)), (((((var_uc_nsubs_dn10 * assign2990_e2050) + (var_uc_nsubs * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10)))) * assign2990_e2056) - (assign2990_e2051 * ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)))) / (assign2990_e2056 * assign2990_e2056)), (((((var_uc_nsubs_dn11 * assign2990_e2050) + (var_uc_nsubs * ((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11)))) * assign2990_e2056) - (assign2990_e2051 * ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)))) / (assign2990_e2056 * assign2990_e2056)), (((((var_uc_nsubs_dn12 * assign2990_e2050) + (var_uc_nsubs * ((var_t1_dn12 * var_t2) + (var_t1 * var_t2_dn12)))) * assign2990_e2056) - (assign2990_e2051 * ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)))) / (assign2990_e2056 * assign2990_e2056)), (((((var_uc_nsubs_dn17 * assign2990_e2050) + (var_uc_nsubs * ((var_t1_dn17 * var_t2) + (var_t1 * var_t2_dn17)))) * assign2990_e2056) - (assign2990_e2051 * ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)))) / (assign2990_e2056 * assign2990_e2056)),)
    } else {
        (var_uc_nsubs, var_uc_nsubs_dn0, var_uc_nsubs_dn2, var_uc_nsubs_dn6, var_uc_nsubs_dn7, var_uc_nsubs_dn10, var_uc_nsubs_dn11, var_uc_nsubs_dn12, var_uc_nsubs_dn17,)
    }
};
        var_uc_nsubs = assign2990_e2059;
        var_uc_nsubs_dn0 = assign2990_e2059_d_n0;
        var_uc_nsubs_dn2 = assign2990_e2059_d_n2;
        var_uc_nsubs_dn6 = assign2990_e2059_d_n6;
        var_uc_nsubs_dn7 = assign2990_e2059_d_n7;
        var_uc_nsubs_dn10 = assign2990_e2059_d_n10;
        var_uc_nsubs_dn11 = assign2990_e2059_d_n11;
        var_uc_nsubs_dn12 = assign2990_e2059_d_n12;
        var_uc_nsubs_dn17 = assign2990_e2059_d_n17;
        var_uc_nsubs_rv = 0.0;

        let assign3000_e2066: f64 = if ((var_lgleff > p.p72) || (p.p72 <= 0.0)) { 1.0 } else { 0.0 };
        var_guard16 = assign3000_e2066;
        var_guard16_rv = 0.0;

        let (assign3010_e2080, assign3010_e2080_d_n0, assign3010_e2080_d_n2, assign3010_e2080_d_n6, assign3010_e2080_d_n7, assign3010_e2080_d_n10, assign3010_e2080_d_n11, assign3010_e2080_d_n12, assign3010_e2080_d_n17,) = {
    if (var_guard16 != 0.0) {
        let assign3010_e2071: f64 = (var_lgleff - p.p72);
        let assign3010_e2072: f64 = (var_uc_nsubs * assign3010_e2071);
        let assign3010_e2075: f64 = (var_nsubps * p.p72);
        let assign3010_e2076: f64 = (assign3010_e2072 + assign3010_e2075);
        let assign3010_e2078: f64 = (assign3010_e2076 / var_lgleff);
        (assign3010_e2078, (((var_uc_nsubs_dn0 * assign3010_e2071) + (var_nsubps_dn0 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn2 * assign3010_e2071) + (var_nsubps_dn2 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn6 * assign3010_e2071) + (var_nsubps_dn6 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn7 * assign3010_e2071) + (var_nsubps_dn7 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn10 * assign3010_e2071) + (var_nsubps_dn10 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn11 * assign3010_e2071) + (var_nsubps_dn11 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn12 * assign3010_e2071) + (var_nsubps_dn12 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn17 * assign3010_e2071) + (var_nsubps_dn17 * p.p72)) / var_lgleff),)
    } else {
        (var_nsub, var_nsub_dn0, var_nsub_dn2, var_nsub_dn6, var_nsub_dn7, var_nsub_dn10, var_nsub_dn11, var_nsub_dn12, var_nsub_dn17,)
    }
};
        var_nsub = assign3010_e2080;
        var_nsub_dn0 = assign3010_e2080_d_n0;
        var_nsub_dn2 = assign3010_e2080_d_n2;
        var_nsub_dn6 = assign3010_e2080_d_n6;
        var_nsub_dn7 = assign3010_e2080_d_n7;
        var_nsub_dn10 = assign3010_e2080_d_n10;
        var_nsub_dn11 = assign3010_e2080_d_n11;
        var_nsub_dn12 = assign3010_e2080_d_n12;
        var_nsub_dn17 = assign3010_e2080_d_n17;
        var_nsub_rv = 0.0;

        let (assign3020_e2095, assign3020_e2095_d_n0, assign3020_e2095_d_n2, assign3020_e2095_d_n6, assign3020_e2095_d_n7, assign3020_e2095_d_n10, assign3020_e2095_d_n11, assign3020_e2095_d_n12, assign3020_e2095_d_n17,) = {
    if (var_guard16 == 0.0) {
        let assign3020_e2086: f64 = (var_nsubps - var_uc_nsubs);
        let assign3020_e2089: f64 = (p.p72 - var_lgleff);
        let assign3020_e2090: f64 = (assign3020_e2086 * assign3020_e2089);
        let assign3020_e2092: f64 = (assign3020_e2090 / p.p72);
        let assign3020_e2093: f64 = (var_nsubps + assign3020_e2092);
        (assign3020_e2093, (var_nsubps_dn0 + (((var_nsubps_dn0 - var_uc_nsubs_dn0) * assign3020_e2089) / p.p72)), (var_nsubps_dn2 + (((var_nsubps_dn2 - var_uc_nsubs_dn2) * assign3020_e2089) / p.p72)), (var_nsubps_dn6 + (((var_nsubps_dn6 - var_uc_nsubs_dn6) * assign3020_e2089) / p.p72)), (var_nsubps_dn7 + (((var_nsubps_dn7 - var_uc_nsubs_dn7) * assign3020_e2089) / p.p72)), (var_nsubps_dn10 + (((var_nsubps_dn10 - var_uc_nsubs_dn10) * assign3020_e2089) / p.p72)), (var_nsubps_dn11 + (((var_nsubps_dn11 - var_uc_nsubs_dn11) * assign3020_e2089) / p.p72)), (var_nsubps_dn12 + (((var_nsubps_dn12 - var_uc_nsubs_dn12) * assign3020_e2089) / p.p72)), (var_nsubps_dn17 + (((var_nsubps_dn17 - var_uc_nsubs_dn17) * assign3020_e2089) / p.p72)),)
    } else {
        (var_nsub, var_nsub_dn0, var_nsub_dn2, var_nsub_dn6, var_nsub_dn7, var_nsub_dn10, var_nsub_dn11, var_nsub_dn12, var_nsub_dn17,)
    }
};
        var_nsub = assign3020_e2095;
        var_nsub_dn0 = assign3020_e2095_d_n0;
        var_nsub_dn2 = assign3020_e2095_d_n2;
        var_nsub_dn6 = assign3020_e2095_d_n6;
        var_nsub_dn7 = assign3020_e2095_d_n7;
        var_nsub_dn10 = assign3020_e2095_d_n10;
        var_nsub_dn11 = assign3020_e2095_d_n11;
        var_nsub_dn12 = assign3020_e2095_d_n12;
        var_nsub_dn17 = assign3020_e2095_d_n17;
        var_nsub_rv = 0.0;

        let assign3030_e2098: f64 = (1.6021918e-19 * var_nsub);
        var_q_nsub = assign3030_e2098;
        var_q_nsub_dn0 = (1.6021918e-19 * var_nsub_dn0);
        var_q_nsub_dn2 = (1.6021918e-19 * var_nsub_dn2);
        var_q_nsub_dn6 = (1.6021918e-19 * var_nsub_dn6);
        var_q_nsub_dn7 = (1.6021918e-19 * var_nsub_dn7);
        var_q_nsub_dn10 = (1.6021918e-19 * var_nsub_dn10);
        var_q_nsub_dn11 = (1.6021918e-19 * var_nsub_dn11);
        var_q_nsub_dn12 = (1.6021918e-19 * var_nsub_dn12);
        var_q_nsub_dn17 = (1.6021918e-19 * var_nsub_dn17);
        var_q_nsub_rv = 0.0;

        let assign3040_e2101: f64 = (var_q_nsub * 1.034943e-10);
        var_qnsub_esi = assign3040_e2101;
        var_qnsub_esi_dn0 = (var_q_nsub_dn0 * 1.034943e-10);
        var_qnsub_esi_dn2 = (var_q_nsub_dn2 * 1.034943e-10);
        var_qnsub_esi_dn6 = (var_q_nsub_dn6 * 1.034943e-10);
        var_qnsub_esi_dn7 = (var_q_nsub_dn7 * 1.034943e-10);
        var_qnsub_esi_dn10 = (var_q_nsub_dn10 * 1.034943e-10);
        var_qnsub_esi_dn11 = (var_q_nsub_dn11 * 1.034943e-10);
        var_qnsub_esi_dn12 = (var_q_nsub_dn12 * 1.034943e-10);
        var_qnsub_esi_dn17 = (var_q_nsub_dn17 * 1.034943e-10);
        var_qnsub_esi_rv = 0.0;

        let assign3050_e2104: f64 = (2.0 * var_qnsub_esi);
        var_qnsub_esi2 = assign3050_e2104;
        var_qnsub_esi2_dn0 = (2.0 * var_qnsub_esi_dn0);
        var_qnsub_esi2_dn2 = (2.0 * var_qnsub_esi_dn2);
        var_qnsub_esi2_dn6 = (2.0 * var_qnsub_esi_dn6);
        var_qnsub_esi2_dn7 = (2.0 * var_qnsub_esi_dn7);
        var_qnsub_esi2_dn10 = (2.0 * var_qnsub_esi_dn10);
        var_qnsub_esi2_dn11 = (2.0 * var_qnsub_esi_dn11);
        var_qnsub_esi2_dn12 = (2.0 * var_qnsub_esi_dn12);
        var_qnsub_esi2_dn17 = (2.0 * var_qnsub_esi_dn17);
        var_qnsub_esi2_rv = 0.0;

        let assign3060_e2108: f64 = (2.0 * p.p72);
        let assign3060_e2113: f64 = if ((var_lgleff <= assign3060_e2108) && (p.p72 > 0.0)) { 1.0 } else { 0.0 };
        var_guard17 = assign3060_e2113;
        var_guard17_rv = 0.0;

        let (assign3070_e2129, assign3070_e2129_d_n0, assign3070_e2129_d_n2, assign3070_e2129_d_n6, assign3070_e2129_d_n7, assign3070_e2129_d_n10, assign3070_e2129_d_n11, assign3070_e2129_d_n12, assign3070_e2129_d_n17,) = {
    if (var_guard17 != 0.0) {
        let assign3070_e2117: f64 = (2.0 * var_nsubps);
        let assign3070_e2120: f64 = (var_nsubps - var_uc_nsubs);
        let assign3070_e2122: f64 = (assign3070_e2120 * var_lgleff);
        let assign3070_e2124: f64 = (assign3070_e2122 / p.p72);
        let assign3070_e2125: f64 = (assign3070_e2117 - assign3070_e2124);
        let assign3070_e2127: f64 = (assign3070_e2125 - var_uc_nsubs);
        (assign3070_e2127, (((2.0 * var_nsubps_dn0) - (((var_nsubps_dn0 - var_uc_nsubs_dn0) * var_lgleff) / p.p72)) - var_uc_nsubs_dn0), (((2.0 * var_nsubps_dn2) - (((var_nsubps_dn2 - var_uc_nsubs_dn2) * var_lgleff) / p.p72)) - var_uc_nsubs_dn2), (((2.0 * var_nsubps_dn6) - (((var_nsubps_dn6 - var_uc_nsubs_dn6) * var_lgleff) / p.p72)) - var_uc_nsubs_dn6), (((2.0 * var_nsubps_dn7) - (((var_nsubps_dn7 - var_uc_nsubs_dn7) * var_lgleff) / p.p72)) - var_uc_nsubs_dn7), (((2.0 * var_nsubps_dn10) - (((var_nsubps_dn10 - var_uc_nsubs_dn10) * var_lgleff) / p.p72)) - var_uc_nsubs_dn10), (((2.0 * var_nsubps_dn11) - (((var_nsubps_dn11 - var_uc_nsubs_dn11) * var_lgleff) / p.p72)) - var_uc_nsubs_dn11), (((2.0 * var_nsubps_dn12) - (((var_nsubps_dn12 - var_uc_nsubs_dn12) * var_lgleff) / p.p72)) - var_uc_nsubs_dn12), (((2.0 * var_nsubps_dn17) - (((var_nsubps_dn17 - var_uc_nsubs_dn17) * var_lgleff) / p.p72)) - var_uc_nsubs_dn17),)
    } else {
        (var_nsubb0, var_nsubb0_dn0, var_nsubb0_dn2, var_nsubb0_dn6, var_nsubb0_dn7, var_nsubb0_dn10, var_nsubb0_dn11, var_nsubb0_dn12, var_nsubb0_dn17,)
    }
};
        var_nsubb0 = assign3070_e2129;
        var_nsubb0_dn0 = assign3070_e2129_d_n0;
        var_nsubb0_dn2 = assign3070_e2129_d_n2;
        var_nsubb0_dn6 = assign3070_e2129_d_n6;
        var_nsubb0_dn7 = assign3070_e2129_d_n7;
        var_nsubb0_dn10 = assign3070_e2129_d_n10;
        var_nsubb0_dn11 = assign3070_e2129_d_n11;
        var_nsubb0_dn12 = assign3070_e2129_d_n12;
        var_nsubb0_dn17 = assign3070_e2129_d_n17;
        var_nsubb0_rv = 0.0;

        let (assign3080_e2136, assign3080_e2136_d_n0, assign3080_e2136_d_n2, assign3080_e2136_d_n6, assign3080_e2136_d_n7, assign3080_e2136_d_n10, assign3080_e2136_d_n11, assign3080_e2136_d_n12, assign3080_e2136_d_n17,) = {
    if (var_guard17 != 0.0) {
        let assign3080_e2133: f64 = (var_nsubb0 / var_uc_nsubs);
        let assign3080_e2134: f64 = (assign3080_e2133).ln();
        (assign3080_e2134, ((((var_nsubb0_dn0 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)) / assign3080_e2133), ((((var_nsubb0_dn2 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)) / assign3080_e2133), ((((var_nsubb0_dn6 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)) / assign3080_e2133), ((((var_nsubb0_dn7 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn7)) / (var_uc_nsubs * var_uc_nsubs)) / assign3080_e2133), ((((var_nsubb0_dn10 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)) / assign3080_e2133), ((((var_nsubb0_dn11 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)) / assign3080_e2133), ((((var_nsubb0_dn12 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)) / assign3080_e2133), ((((var_nsubb0_dn17 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn17)) / (var_uc_nsubs * var_uc_nsubs)) / assign3080_e2133),)
    } else {
        (var_ptovr0, var_ptovr0_dn0, var_ptovr0_dn2, var_ptovr0_dn6, var_ptovr0_dn7, var_ptovr0_dn10, var_ptovr0_dn11, var_ptovr0_dn12, var_ptovr0_dn17,)
    }
};
        var_ptovr0 = assign3080_e2136;
        var_ptovr0_dn0 = assign3080_e2136_d_n0;
        var_ptovr0_dn2 = assign3080_e2136_d_n2;
        var_ptovr0_dn6 = assign3080_e2136_d_n6;
        var_ptovr0_dn7 = assign3080_e2136_d_n7;
        var_ptovr0_dn10 = assign3080_e2136_d_n10;
        var_ptovr0_dn11 = assign3080_e2136_d_n11;
        var_ptovr0_dn12 = assign3080_e2136_d_n12;
        var_ptovr0_dn17 = assign3080_e2136_d_n17;
        var_ptovr0_rv = 0.0;

        let (assign3090_e2141, assign3090_e2141_d_n0, assign3090_e2141_d_n2, assign3090_e2141_d_n6, assign3090_e2141_d_n7, assign3090_e2141_d_n10, assign3090_e2141_d_n11, assign3090_e2141_d_n12, assign3090_e2141_d_n17,) = {
    if (var_guard17 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ptovr0, var_ptovr0_dn0, var_ptovr0_dn2, var_ptovr0_dn6, var_ptovr0_dn7, var_ptovr0_dn10, var_ptovr0_dn11, var_ptovr0_dn12, var_ptovr0_dn17,)
    }
};
        var_ptovr0 = assign3090_e2141;
        var_ptovr0_dn0 = assign3090_e2141_d_n0;
        var_ptovr0_dn2 = assign3090_e2141_d_n2;
        var_ptovr0_dn6 = assign3090_e2141_d_n6;
        var_ptovr0_dn7 = assign3090_e2141_d_n7;
        var_ptovr0_dn10 = assign3090_e2141_d_n10;
        var_ptovr0_dn11 = assign3090_e2141_d_n11;
        var_ptovr0_dn12 = assign3090_e2141_d_n12;
        var_ptovr0_dn17 = assign3090_e2141_d_n17;
        var_ptovr0_rv = 0.0;

        let assign3100_e2144: f64 = (2.0 / 38.68283);
        let assign3100_e2148: f64 = (10400000000.0 / 1e-6);
        let assign3100_e2149: f64 = (var_nsub / assign3100_e2148);
        let assign3100_e2150: f64 = (assign3100_e2149).ln();
        let assign3100_e2151: f64 = (assign3100_e2144 * assign3100_e2150);
        var_pb20 = assign3100_e2151;
        var_pb20_dn0 = (assign3100_e2144 * ((var_nsub_dn0 / assign3100_e2148) / assign3100_e2149));
        var_pb20_dn2 = (assign3100_e2144 * ((var_nsub_dn2 / assign3100_e2148) / assign3100_e2149));
        var_pb20_dn6 = (assign3100_e2144 * ((var_nsub_dn6 / assign3100_e2148) / assign3100_e2149));
        var_pb20_dn7 = (assign3100_e2144 * ((var_nsub_dn7 / assign3100_e2148) / assign3100_e2149));
        var_pb20_dn10 = (assign3100_e2144 * ((var_nsub_dn10 / assign3100_e2148) / assign3100_e2149));
        var_pb20_dn11 = (assign3100_e2144 * ((var_nsub_dn11 / assign3100_e2148) / assign3100_e2149));
        var_pb20_dn12 = (assign3100_e2144 * ((var_nsub_dn12 / assign3100_e2148) / assign3100_e2149));
        var_pb20_dn17 = (assign3100_e2144 * ((var_nsub_dn17 / assign3100_e2148) / assign3100_e2149));
        var_pb20_rv = 0.0;

        let assign3110_e2154: f64 = (2.0 / 38.68283);
        let assign3110_e2158: f64 = (10400000000.0 / 1e-6);
        let assign3110_e2159: f64 = (var_uc_nsubs / assign3110_e2158);
        let assign3110_e2160: f64 = (assign3110_e2159).ln();
        let assign3110_e2161: f64 = (assign3110_e2154 * assign3110_e2160);
        var_pb2c = assign3110_e2161;
        var_pb2c_dn0 = (assign3110_e2154 * ((var_uc_nsubs_dn0 / assign3110_e2158) / assign3110_e2159));
        var_pb2c_dn2 = (assign3110_e2154 * ((var_uc_nsubs_dn2 / assign3110_e2158) / assign3110_e2159));
        var_pb2c_dn6 = (assign3110_e2154 * ((var_uc_nsubs_dn6 / assign3110_e2158) / assign3110_e2159));
        var_pb2c_dn7 = (assign3110_e2154 * ((var_uc_nsubs_dn7 / assign3110_e2158) / assign3110_e2159));
        var_pb2c_dn10 = (assign3110_e2154 * ((var_uc_nsubs_dn10 / assign3110_e2158) / assign3110_e2159));
        var_pb2c_dn11 = (assign3110_e2154 * ((var_uc_nsubs_dn11 / assign3110_e2158) / assign3110_e2159));
        var_pb2c_dn12 = (assign3110_e2154 * ((var_uc_nsubs_dn12 / assign3110_e2158) / assign3110_e2159));
        var_pb2c_dn17 = (assign3110_e2154 * ((var_uc_nsubs_dn17 / assign3110_e2158) / assign3110_e2159));
        var_pb2c_rv = 0.0;

        let assign3120_e2164: f64 = (2.0 * 1.034943e-10);
        let assign3120_e2166: f64 = (assign3120_e2164 / 1.6021918e-19);
        let assign3120_e2168: f64 = (assign3120_e2166 / var_nsub);
        let assign3120_e2169: f64 = (assign3120_e2168).sqrt();
        var_wdpl = assign3120_e2169;
        var_wdpl_dn0 = ((-((assign3120_e2166 * var_nsub_dn0) / (var_nsub * var_nsub))) / (2.0 * assign3120_e2169));
        var_wdpl_dn2 = ((-((assign3120_e2166 * var_nsub_dn2) / (var_nsub * var_nsub))) / (2.0 * assign3120_e2169));
        var_wdpl_dn6 = ((-((assign3120_e2166 * var_nsub_dn6) / (var_nsub * var_nsub))) / (2.0 * assign3120_e2169));
        var_wdpl_dn7 = ((-((assign3120_e2166 * var_nsub_dn7) / (var_nsub * var_nsub))) / (2.0 * assign3120_e2169));
        var_wdpl_dn10 = ((-((assign3120_e2166 * var_nsub_dn10) / (var_nsub * var_nsub))) / (2.0 * assign3120_e2169));
        var_wdpl_dn11 = ((-((assign3120_e2166 * var_nsub_dn11) / (var_nsub * var_nsub))) / (2.0 * assign3120_e2169));
        var_wdpl_dn12 = ((-((assign3120_e2166 * var_nsub_dn12) / (var_nsub * var_nsub))) / (2.0 * assign3120_e2169));
        var_wdpl_dn17 = ((-((assign3120_e2166 * var_nsub_dn17) / (var_nsub * var_nsub))) / (2.0 * assign3120_e2169));
        var_wdpl_rv = 0.0;

        let assign3130_e2174: f64 = (var_lgle).powf(p.p195);
        let assign3130_e2175: f64 = (p.p194 / assign3130_e2174);
        let assign3130_e2176: f64 = (1.0 + assign3130_e2175);
        let assign3130_e2181: f64 = (var_wl).powf(p.p197);
        let assign3130_e2182: f64 = (p.p196 / assign3130_e2181);
        let assign3130_e2183: f64 = (1.0 + assign3130_e2182);
        let assign3130_e2184: f64 = (assign3130_e2176 * assign3130_e2183);
        var_t1 = assign3130_e2184;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn17 = 0.0;
        var_t1_rv = 0.0;

        let assign3140_e2187: f64 = (var_t1 * var_t1);
        let assign3140_e2190: f64 = (4.0 * 0.001);
        let assign3140_e2192: f64 = (assign3140_e2190 * 0.001);
        let assign3140_e2193: f64 = (assign3140_e2187 + assign3140_e2192);
        let assign3140_e2194: f64 = (assign3140_e2193).sqrt();
        var_tmf1 = assign3140_e2194;
        var_tmf1_dn0 = (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) / (2.0 * assign3140_e2194));
        var_tmf1_dn2 = (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) / (2.0 * assign3140_e2194));
        var_tmf1_dn6 = (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign3140_e2194));
        var_tmf1_dn7 = (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign3140_e2194));
        var_tmf1_dn10 = (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) / (2.0 * assign3140_e2194));
        var_tmf1_dn11 = (((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) / (2.0 * assign3140_e2194));
        var_tmf1_dn12 = (((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) / (2.0 * assign3140_e2194));
        var_tmf1_dn17 = (((var_t1_dn17 * var_t1) + (var_t1 * var_t1_dn17)) / (2.0 * assign3140_e2194));
        var_tmf1_rv = 0.0;

        let assign3150_e2198: f64 = (var_t1 + var_tmf1);
        let assign3150_e2199: f64 = (0.5 * assign3150_e2198);
        let assign3150_e2202: f64 = (1e-10 * 0.001);
        let assign3150_e2203: f64 = (assign3150_e2199 + assign3150_e2202);
        var_vmax0 = assign3150_e2203;
        var_vmax0_dn0 = (0.5 * (var_t1_dn0 + var_tmf1_dn0));
        var_vmax0_dn2 = (0.5 * (var_t1_dn2 + var_tmf1_dn2));
        var_vmax0_dn6 = (0.5 * (var_t1_dn6 + var_tmf1_dn6));
        var_vmax0_dn7 = (0.5 * (var_t1_dn7 + var_tmf1_dn7));
        var_vmax0_dn10 = (0.5 * (var_t1_dn10 + var_tmf1_dn10));
        var_vmax0_dn11 = (0.5 * (var_t1_dn11 + var_tmf1_dn11));
        var_vmax0_dn12 = (0.5 * (var_t1_dn12 + var_tmf1_dn12));
        var_vmax0_dn17 = (0.5 * (var_t1_dn17 + var_tmf1_dn17));
        var_vmax0_rv = 0.0;

        let assign3160_e2206: f64 = if var_vmax0 < 0.0 { 1.0 } else { 0.0 };
        var_guard18 = assign3160_e2206;
        var_guard18_rv = 0.0;

        *var_guard15_slot = var_guard15;
        *var_guard15_rv_slot = var_guard15_rv;
        *var_guard16_slot = var_guard16;
        *var_guard16_rv_slot = var_guard16_rv;
        *var_guard17_slot = var_guard17;
        *var_guard17_rv_slot = var_guard17_rv;
        *var_guard18_slot = var_guard18;
        *var_guard18_rv_slot = var_guard18_rv;
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
        var_guard18: f64,
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
        var_guard21_slot: &mut f64,
        var_guard21_rv_slot: &mut f64,
        var_guard26_slot: &mut f64,
        var_guard26_rv_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_guard27_rv_slot: &mut f64,
        var_guard28_slot: &mut f64,
        var_guard28_rv_slot: &mut f64,
        var_guard29_slot: &mut f64,
        var_guard29_rv_slot: &mut f64,
        var_guard30_slot: &mut f64,
        var_guard30_rv_slot: &mut f64,
        var_guard31_slot: &mut f64,
        var_guard31_rv_slot: &mut f64,
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
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard21_rv: f64 = *var_guard21_rv_slot;
        let mut var_guard26: f64 = *var_guard26_slot;
        let mut var_guard26_rv: f64 = *var_guard26_rv_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_guard27_rv: f64 = *var_guard27_rv_slot;
        let mut var_guard28: f64 = *var_guard28_slot;
        let mut var_guard28_rv: f64 = *var_guard28_rv_slot;
        let mut var_guard29: f64 = *var_guard29_slot;
        let mut var_guard29_rv: f64 = *var_guard29_rv_slot;
        let mut var_guard30: f64 = *var_guard30_slot;
        let mut var_guard30_rv: f64 = *var_guard30_rv_slot;
        let mut var_guard31: f64 = *var_guard31_slot;
        let mut var_guard31_rv: f64 = *var_guard31_rv_slot;
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

        let (assign3170_e2210, assign3170_e2210_d_n0, assign3170_e2210_d_n2, assign3170_e2210_d_n6, assign3170_e2210_d_n7, assign3170_e2210_d_n10, assign3170_e2210_d_n11, assign3170_e2210_d_n12, assign3170_e2210_d_n17,) = {
    if (var_guard18 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vmax0, var_vmax0_dn0, var_vmax0_dn2, var_vmax0_dn6, var_vmax0_dn7, var_vmax0_dn10, var_vmax0_dn11, var_vmax0_dn12, var_vmax0_dn17,)
    }
};
        var_vmax0 = assign3170_e2210;
        var_vmax0_dn0 = assign3170_e2210_d_n0;
        var_vmax0_dn2 = assign3170_e2210_d_n2;
        var_vmax0_dn6 = assign3170_e2210_d_n6;
        var_vmax0_dn7 = assign3170_e2210_d_n7;
        var_vmax0_dn10 = assign3170_e2210_d_n10;
        var_vmax0_dn11 = assign3170_e2210_d_n11;
        var_vmax0_dn12 = assign3170_e2210_d_n12;
        var_vmax0_dn17 = assign3170_e2210_d_n17;
        var_vmax0_rv = 0.0;

        let assign3230_e2243: f64 = if p.p261 == 1.0 { 1.0 } else { 0.0 };
        var_guard21 = assign3230_e2243;
        var_guard21_rv = 0.0;

        let (assign3240_e2251, assign3240_e2251_d_n0, assign3240_e2251_d_n2, assign3240_e2251_d_n6, assign3240_e2251_d_n7, assign3240_e2251_d_n10, assign3240_e2251_d_n11, assign3240_e2251_d_n12, assign3240_e2251_d_n17,) = {
    if (var_guard21 != 0.0) {
        let assign3240_e2247: f64 = (p.p289 * var_weff_nf);
        let assign3240_e2249: f64 = (assign3240_e2247 + p.p288);
        (assign3240_e2249, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn6, var_t0_dn7, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn17,)
    }
};
        var_t0 = assign3240_e2251;
        var_t0_dn0 = assign3240_e2251_d_n0;
        var_t0_dn2 = assign3240_e2251_d_n2;
        var_t0_dn6 = assign3240_e2251_d_n6;
        var_t0_dn7 = assign3240_e2251_d_n7;
        var_t0_dn10 = assign3240_e2251_d_n10;
        var_t0_dn11 = assign3240_e2251_d_n11;
        var_t0_dn12 = assign3240_e2251_d_n12;
        var_t0_dn17 = assign3240_e2251_d_n17;
        var_t0_rv = 0.0;

        let assign3380_e2327: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard26 = assign3380_e2327;
        var_guard26_rv = 0.0;

        let (assign3390_e2340,) = {
    if ((var_guard26 != 0.0) && (p.p24 != 0.0)) {
        let (assign3390_e2338,) = {
            if (var_abtp_given != 0.0) {
                (p.p23,)
            } else {
                let assign3390_e2335: f64 = (p.p20 * p.p9);
                let assign3390_e2337: f64 = (assign3390_e2335 * p.p19);
                (assign3390_e2337,)
            }
        };
        (assign3390_e2338,)
    } else {
        (var_area_bt_p,)
    }
};
        var_area_bt_p = assign3390_e2340;
        var_area_bt_p_rv = 0.0;

        let (assign3400_e2353,) = {
    if ((var_guard26 != 0.0) && (p.p24 != 0.0)) {
        let (assign3400_e2351,) = {
            if (var_abtn_given != 0.0) {
                (p.p22,)
            } else {
                let assign3400_e2348: f64 = (p.p21 * p.p9);
                let assign3400_e2350: f64 = (assign3400_e2348 * p.p19);
                (assign3400_e2350,)
            }
        };
        (assign3400_e2351,)
    } else {
        (var_area_bt_n,)
    }
};
        var_area_bt_n = assign3400_e2353;
        var_area_bt_n_rv = 0.0;

        let (assign3410_e2359,) = {
    if ((var_guard26 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3410_e2359;
        var_cbtp_rv = 0.0;

        let (assign3420_e2365,) = {
    if ((var_guard26 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (var_cbtn,)
    }
};
        var_cbtn = assign3420_e2365;
        var_cbtn_rv = 0.0;

        let assign3430_e2370: f64 = if ((var_area_bt_p > 0.0) && (var_cbtbp_given != 0.0)) { 1.0 } else { 0.0 };
        var_guard27 = assign3430_e2370;
        var_guard27_rv = 0.0;

        let (assign3440_e2381,) = {
    if (((var_guard26 != 0.0) && (p.p24 != 0.0)) && (var_guard27 != 0.0)) {
        let assign3440_e2377: f64 = (-var_area_bt_p);
        let assign3440_e2379: f64 = (assign3440_e2377 * p.p294);
        (assign3440_e2379,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3440_e2381;
        var_cbtp_rv = 0.0;

        let (assign3450_e2390,) = {
    if (((var_guard26 != 0.0) && (p.p24 != 0.0)) && (var_guard27 == 0.0)) {
        (0.0,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3450_e2390;
        var_cbtp_rv = 0.0;

        let assign3460_e2395: f64 = if ((var_area_bt_n > 0.0) && (var_cbtbn_given != 0.0)) { 1.0 } else { 0.0 };
        var_guard28 = assign3460_e2395;
        var_guard28_rv = 0.0;

        let (assign3470_e2406,) = {
    if (((var_guard26 != 0.0) && (p.p24 != 0.0)) && (var_guard28 != 0.0)) {
        let assign3470_e2402: f64 = (-var_area_bt_n);
        let assign3470_e2404: f64 = (assign3470_e2402 * p.p293);
        (assign3470_e2404,)
    } else {
        (var_cbtn,)
    }
};
        var_cbtn = assign3470_e2406;
        var_cbtn_rv = 0.0;

        let (assign3480_e2414,) = {
    if (((var_guard26 != 0.0) && (p.p24 != 0.0)) && (var_guard28 != 0.0)) {
        (0.0,)
    } else {
        (var_area_bt_n,)
    }
};
        var_area_bt_n = assign3480_e2414;
        var_area_bt_n_rv = 0.0;

        let (assign3490_e2421,) = {
    if ((var_guard26 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (var_area_bt_n,)
    }
};
        var_area_bt_n = assign3490_e2421;
        var_area_bt_n_rv = 0.0;

        let (assign3500_e2428,) = {
    if ((var_guard26 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (var_cbtn,)
    }
};
        var_cbtn = assign3500_e2428;
        var_cbtn_rv = 0.0;

        let (assign3510_e2435,) = {
    if ((var_guard26 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (var_area_bt_p,)
    }
};
        var_area_bt_p = assign3510_e2435;
        var_area_bt_p_rv = 0.0;

        let (assign3520_e2442,) = {
    if ((var_guard26 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3520_e2442;
        var_cbtp_rv = 0.0;

        let (assign3530_e2455,) = {
    if (var_guard26 != 0.0) {
        let (assign3530_e2453,) = {
            if (p.p19 > var_lgate) {
                let assign3530_e2450: f64 = (p.p19 - var_lgate);
                let assign3530_e2451: f64 = (0.5 * assign3530_e2450);
                (assign3530_e2451,)
            } else {
                (0.0,)
            }
        };
        (assign3530_e2453,)
    } else {
        (var_peri_hhi,)
    }
};
        var_peri_hhi = assign3530_e2455;
        var_peri_hhi_rv = 0.0;

        let assign3540_e2458: f64 = if var_pdbcp_given == 0.0 { 1.0 } else { 0.0 };
        var_guard29 = assign3540_e2458;
        var_guard29_rv = 0.0;

        let (assign3550_e2464,) = {
    if ((var_guard26 != 0.0) && (var_guard29 != 0.0)) {
        (var_peri_hhi,)
    } else {
        (var_uc_pdbcp,)
    }
};
        var_uc_pdbcp = assign3550_e2464;
        var_uc_pdbcp_rv = 0.0;

        let assign3560_e2467: f64 = if var_psbcp_given == 0.0 { 1.0 } else { 0.0 };
        var_guard30 = assign3560_e2467;
        var_guard30_rv = 0.0;

        let (assign3570_e2473,) = {
    if ((var_guard26 != 0.0) && (var_guard30 != 0.0)) {
        (var_peri_hhi,)
    } else {
        (var_uc_psbcp,)
    }
};
        var_uc_psbcp = assign3570_e2473;
        var_uc_psbcp_rv = 0.0;

        let (assign3580_e2481,) = {
    if (var_guard26 != 0.0) {
        let assign3580_e2478: f64 = (p.p9 * var_uc_pdbcp);
        let assign3580_e2479: f64 = (var_weff_nf + assign3580_e2478);
        (assign3580_e2479,)
    } else {
        (var_w_diod,)
    }
};
        var_w_diod = assign3580_e2481;
        var_w_diod_rv = 0.0;

        let (assign3590_e2489,) = {
    if (var_guard26 != 0.0) {
        let assign3590_e2486: f64 = (p.p9 * var_uc_psbcp);
        let assign3590_e2487: f64 = (var_weff_nf + assign3590_e2486);
        (assign3590_e2487,)
    } else {
        (var_w_dios,)
    }
};
        var_w_dios = assign3590_e2489;
        var_w_dios_rv = 0.0;

        let (assign3600_e2497,) = {
    if (var_guard26 != 0.0) {
        let assign3600_e2494: f64 = (p.p9 * var_uc_pdbcp);
        let assign3600_e2495: f64 = (var_weffcv_nf + assign3600_e2494);
        (assign3600_e2495,)
    } else {
        (var_w_diodcv,)
    }
};
        var_w_diodcv = assign3600_e2497;
        var_w_diodcv_rv = 0.0;

        let (assign3610_e2505,) = {
    if (var_guard26 != 0.0) {
        let assign3610_e2502: f64 = (p.p9 * var_uc_psbcp);
        let assign3610_e2503: f64 = (var_weffcv_nf + assign3610_e2502);
        (assign3610_e2503,)
    } else {
        (var_w_dioscv,)
    }
};
        var_w_dioscv = assign3610_e2505;
        var_w_dioscv_rv = 0.0;

        let (assign3620_e2510,) = {
    if (var_guard26 == 0.0) {
        (0.0,)
    } else {
        (var_area_bt_n,)
    }
};
        var_area_bt_n = assign3620_e2510;
        var_area_bt_n_rv = 0.0;

        let (assign3630_e2515,) = {
    if (var_guard26 == 0.0) {
        (0.0,)
    } else {
        (var_cbtn,)
    }
};
        var_cbtn = assign3630_e2515;
        var_cbtn_rv = 0.0;

        let (assign3640_e2520,) = {
    if (var_guard26 == 0.0) {
        (0.0,)
    } else {
        (var_area_bt_p,)
    }
};
        var_area_bt_p = assign3640_e2520;
        var_area_bt_p_rv = 0.0;

        let (assign3650_e2525,) = {
    if (var_guard26 == 0.0) {
        (0.0,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3650_e2525;
        var_cbtp_rv = 0.0;

        let (assign3660_e2530,) = {
    if (var_guard26 == 0.0) {
        (0.0,)
    } else {
        (var_w_diod,)
    }
};
        var_w_diod = assign3660_e2530;
        var_w_diod_rv = 0.0;

        let (assign3670_e2535,) = {
    if (var_guard26 == 0.0) {
        (0.0,)
    } else {
        (var_w_dios,)
    }
};
        var_w_dios = assign3670_e2535;
        var_w_dios_rv = 0.0;

        let (assign3680_e2540,) = {
    if (var_guard26 == 0.0) {
        (0.0,)
    } else {
        (var_w_diodcv,)
    }
};
        var_w_diodcv = assign3680_e2540;
        var_w_diodcv_rv = 0.0;

        let (assign3690_e2545,) = {
    if (var_guard26 == 0.0) {
        (0.0,)
    } else {
        (var_w_dioscv,)
    }
};
        var_w_dioscv = assign3690_e2545;
        var_w_dioscv_rv = 0.0;

        let assign3700_e2548: f64 = (p.p50 * (nv6 - nv7));
        var_vdsi = assign3700_e2548;
        var_vdsi_dn6 = p.p50;
        var_vdsi_dn7 = (-p.p50);
        var_vdsi_rv = 0.0;

        let assign3710_e2551: f64 = (p.p50 * (nv11 - nv7));
        var_vgsi = assign3710_e2551;
        var_vgsi_dn7 = (-p.p50);
        var_vgsi_dn11 = p.p50;
        var_vgsi_rv = 0.0;

        let assign3720_e2554: f64 = (p.p50 * (nv12 - nv7));
        var_vbsi = assign3720_e2554;
        var_vbsi_dn7 = (-p.p50);
        var_vbsi_dn12 = p.p50;
        var_vbsi_rv = 0.0;

        let assign3760_e2566: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard31 = assign3760_e2566;
        var_guard31_rv = 0.0;

        let (assign3770_e2572, assign3770_e2572_d_n6, assign3770_e2572_d_n12,) = {
    if (var_guard31 != 0.0) {
        let assign3770_e2570: f64 = (p.p50 * (nv12 - nv6));
        (assign3770_e2570, (-p.p50), p.p50,)
    } else {
        (var_vbcd, var_vbcd_dn6, var_vbcd_dn12,)
    }
};
        var_vbcd = assign3770_e2572;
        var_vbcd_dn6 = assign3770_e2572_d_n6;
        var_vbcd_dn12 = assign3770_e2572_d_n12;
        var_vbcd_rv = 0.0;

        let (assign3780_e2578, assign3780_e2578_d_n7, assign3780_e2578_d_n12,) = {
    if (var_guard31 != 0.0) {
        let assign3780_e2576: f64 = (p.p50 * (nv12 - nv7));
        (assign3780_e2576, (-p.p50), p.p50,)
    } else {
        (var_vbcs, var_vbcs_dn7, var_vbcs_dn12,)
    }
};
        var_vbcs = assign3780_e2578;
        var_vbcs_dn7 = assign3780_e2578_d_n7;
        var_vbcs_dn12 = assign3780_e2578_d_n12;
        var_vbcs_rv = 0.0;

        let (assign3790_e2588, assign3790_e2588_d_n18,) = {
    if ((var_guard31 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign3790_e2584: f64 = (1e-9 / 0.0001);
        let assign3790_e2586: f64 = (assign3790_e2584 * (nv18 - 0.0));
        (assign3790_e2586, assign3790_e2584,)
    } else {
        (var_qi_nqs, var_qi_nqs_dn18,)
    }
};
        var_qi_nqs = assign3790_e2588;
        var_qi_nqs_dn18 = assign3790_e2588_d_n18;
        var_qi_nqs_rv = 0.0;

        let (assign3800_e2598, assign3800_e2598_d_n13,) = {
    if ((var_guard31 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign3800_e2594: f64 = (1e-9 / 0.0001);
        let assign3800_e2596: f64 = (assign3800_e2594 * (nv13 - 0.0));
        (assign3800_e2596, assign3800_e2594,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign3800_e2598;
        var_qb_nqs_dn13 = assign3800_e2598_d_n13;
        var_qb_nqs_rv = 0.0;

        let (assign3810_e2605, assign3810_e2605_d_n18,) = {
    if ((var_guard31 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qi_nqs, var_qi_nqs_dn18,)
    }
};
        var_qi_nqs = assign3810_e2605;
        var_qi_nqs_dn18 = assign3810_e2605_d_n18;
        var_qi_nqs_rv = 0.0;

        let (assign3820_e2612, assign3820_e2612_d_n13,) = {
    if ((var_guard31 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign3820_e2612;
        var_qb_nqs_dn13 = assign3820_e2612_d_n13;
        var_qb_nqs_rv = 0.0;

        let (assign3830_e2617, assign3830_e2617_d_n6, assign3830_e2617_d_n12,) = {
    if (var_guard31 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_vbcd, var_vbcd_dn6, var_vbcd_dn12,)
    }
};
        var_vbcd = assign3830_e2617;
        var_vbcd_dn6 = assign3830_e2617_d_n6;
        var_vbcd_dn12 = assign3830_e2617_d_n12;
        var_vbcd_rv = 0.0;

        let (assign3840_e2622, assign3840_e2622_d_n7, assign3840_e2622_d_n12,) = {
    if (var_guard31 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_vbcs, var_vbcs_dn7, var_vbcs_dn12,)
    }
};
        var_vbcs = assign3840_e2622;
        var_vbcs_dn7 = assign3840_e2622_d_n7;
        var_vbcs_dn12 = assign3840_e2622_d_n12;
        var_vbcs_rv = 0.0;

        *var_area_bt_n_slot = var_area_bt_n;
        *var_area_bt_n_rv_slot = var_area_bt_n_rv;
        *var_area_bt_p_slot = var_area_bt_p;
        *var_area_bt_p_rv_slot = var_area_bt_p_rv;
        *var_cbtn_slot = var_cbtn;
        *var_cbtn_rv_slot = var_cbtn_rv;
        *var_cbtp_slot = var_cbtp;
        *var_cbtp_rv_slot = var_cbtp_rv;
        *var_guard21_slot = var_guard21;
        *var_guard21_rv_slot = var_guard21_rv;
        *var_guard26_slot = var_guard26;
        *var_guard26_rv_slot = var_guard26_rv;
        *var_guard27_slot = var_guard27;
        *var_guard27_rv_slot = var_guard27_rv;
        *var_guard28_slot = var_guard28;
        *var_guard28_rv_slot = var_guard28_rv;
        *var_guard29_slot = var_guard29;
        *var_guard29_rv_slot = var_guard29_rv;
        *var_guard30_slot = var_guard30;
        *var_guard30_rv_slot = var_guard30_rv;
        *var_guard31_slot = var_guard31;
        *var_guard31_rv_slot = var_guard31_rv;
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
        var_guard31: f64,
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
        var_guard32_slot: &mut f64,
        var_guard32_rv_slot: &mut f64,
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
        var_t1__blk37_slot: &mut f64,
        var_t1__blk37_dn10_slot: &mut f64,
        var_t1__blk37_rv_slot: &mut f64,
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
        var_t2__blk38_slot: &mut f64,
        var_t2__blk38_rv_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3__blk39_slot: &mut f64,
        var_t3__blk39_rv_slot: &mut f64,
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
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_guard32_rv: f64 = *var_guard32_rv_slot;
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
        let mut var_t1__blk37: f64 = *var_t1__blk37_slot;
        let mut var_t1__blk37_dn10: f64 = *var_t1__blk37_dn10_slot;
        let mut var_t1__blk37_rv: f64 = *var_t1__blk37_rv_slot;
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
        let mut var_t2__blk38: f64 = *var_t2__blk38_slot;
        let mut var_t2__blk38_rv: f64 = *var_t2__blk38_rv_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3__blk39: f64 = *var_t3__blk39_slot;
        let mut var_t3__blk39_rv: f64 = *var_t3__blk39_rv_slot;
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

        let (assign3850_e2633, assign3850_e2633_d_n0, assign3850_e2633_d_n2, assign3850_e2633_d_n6, assign3850_e2633_d_n7, assign3850_e2633_d_n10, assign3850_e2633_d_n11, assign3850_e2633_d_n12, assign3850_e2633_d_n15, assign3850_e2633_d_n17, assign3850_e2633_d_n18,) = {
    if ((var_guard31 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign3850_e2629: f64 = (1e-9 / 0.0001);
        let assign3850_e2631: f64 = (assign3850_e2629 * (nv15 - 0.0));
        (assign3850_e2631, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign3850_e2629, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign3850_e2633;
        var_qd_nqs_dn0 = assign3850_e2633_d_n0;
        var_qd_nqs_dn2 = assign3850_e2633_d_n2;
        var_qd_nqs_dn6 = assign3850_e2633_d_n6;
        var_qd_nqs_dn7 = assign3850_e2633_d_n7;
        var_qd_nqs_dn10 = assign3850_e2633_d_n10;
        var_qd_nqs_dn11 = assign3850_e2633_d_n11;
        var_qd_nqs_dn12 = assign3850_e2633_d_n12;
        var_qd_nqs_dn15 = assign3850_e2633_d_n15;
        var_qd_nqs_dn17 = assign3850_e2633_d_n17;
        var_qd_nqs_dn18 = assign3850_e2633_d_n18;
        var_qd_nqs_rv = 0.0;

        let (assign3860_e2644, assign3860_e2644_d_n0, assign3860_e2644_d_n2, assign3860_e2644_d_n6, assign3860_e2644_d_n7, assign3860_e2644_d_n10, assign3860_e2644_d_n11, assign3860_e2644_d_n12, assign3860_e2644_d_n16, assign3860_e2644_d_n17, assign3860_e2644_d_n18,) = {
    if ((var_guard31 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign3860_e2640: f64 = (1e-9 / 0.0001);
        let assign3860_e2642: f64 = (assign3860_e2640 * (nv16 - 0.0));
        (assign3860_e2642, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign3860_e2640, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign3860_e2644;
        var_qs_nqs_dn0 = assign3860_e2644_d_n0;
        var_qs_nqs_dn2 = assign3860_e2644_d_n2;
        var_qs_nqs_dn6 = assign3860_e2644_d_n6;
        var_qs_nqs_dn7 = assign3860_e2644_d_n7;
        var_qs_nqs_dn10 = assign3860_e2644_d_n10;
        var_qs_nqs_dn11 = assign3860_e2644_d_n11;
        var_qs_nqs_dn12 = assign3860_e2644_d_n12;
        var_qs_nqs_dn16 = assign3860_e2644_d_n16;
        var_qs_nqs_dn17 = assign3860_e2644_d_n17;
        var_qs_nqs_dn18 = assign3860_e2644_d_n18;
        var_qs_nqs_rv = 0.0;

        let (assign3870_e2655, assign3870_e2655_d_n13,) = {
    if ((var_guard31 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign3870_e2651: f64 = (1e-9 / 0.0001);
        let assign3870_e2653: f64 = (assign3870_e2651 * (nv13 - 0.0));
        (assign3870_e2653, assign3870_e2651,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign3870_e2655;
        var_qb_nqs_dn13 = assign3870_e2655_d_n13;
        var_qb_nqs_rv = 0.0;

        let (assign3880_e2663, assign3880_e2663_d_n0, assign3880_e2663_d_n2, assign3880_e2663_d_n6, assign3880_e2663_d_n7, assign3880_e2663_d_n10, assign3880_e2663_d_n11, assign3880_e2663_d_n12, assign3880_e2663_d_n15, assign3880_e2663_d_n17, assign3880_e2663_d_n18,) = {
    if ((var_guard31 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign3880_e2663;
        var_qd_nqs_dn0 = assign3880_e2663_d_n0;
        var_qd_nqs_dn2 = assign3880_e2663_d_n2;
        var_qd_nqs_dn6 = assign3880_e2663_d_n6;
        var_qd_nqs_dn7 = assign3880_e2663_d_n7;
        var_qd_nqs_dn10 = assign3880_e2663_d_n10;
        var_qd_nqs_dn11 = assign3880_e2663_d_n11;
        var_qd_nqs_dn12 = assign3880_e2663_d_n12;
        var_qd_nqs_dn15 = assign3880_e2663_d_n15;
        var_qd_nqs_dn17 = assign3880_e2663_d_n17;
        var_qd_nqs_dn18 = assign3880_e2663_d_n18;
        var_qd_nqs_rv = 0.0;

        let (assign3890_e2671, assign3890_e2671_d_n0, assign3890_e2671_d_n2, assign3890_e2671_d_n6, assign3890_e2671_d_n7, assign3890_e2671_d_n10, assign3890_e2671_d_n11, assign3890_e2671_d_n12, assign3890_e2671_d_n16, assign3890_e2671_d_n17, assign3890_e2671_d_n18,) = {
    if ((var_guard31 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign3890_e2671;
        var_qs_nqs_dn0 = assign3890_e2671_d_n0;
        var_qs_nqs_dn2 = assign3890_e2671_d_n2;
        var_qs_nqs_dn6 = assign3890_e2671_d_n6;
        var_qs_nqs_dn7 = assign3890_e2671_d_n7;
        var_qs_nqs_dn10 = assign3890_e2671_d_n10;
        var_qs_nqs_dn11 = assign3890_e2671_d_n11;
        var_qs_nqs_dn12 = assign3890_e2671_d_n12;
        var_qs_nqs_dn16 = assign3890_e2671_d_n16;
        var_qs_nqs_dn17 = assign3890_e2671_d_n17;
        var_qs_nqs_dn18 = assign3890_e2671_d_n18;
        var_qs_nqs_rv = 0.0;

        let (assign3900_e2679, assign3900_e2679_d_n13,) = {
    if ((var_guard31 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign3900_e2679;
        var_qb_nqs_dn13 = assign3900_e2679_d_n13;
        var_qb_nqs_rv = 0.0;

        let (assign3910_e2694, assign3910_e2694_d_n10,) = {
    if ((p.p38 > 0.0) && (var_mks_rth0 > 0.0)) {
        let (assign3910_e2692, assign3910_e2692_d_n10,) = {
            if ((nv10 - 0.0) > 0.0) {
                ((nv10 - 0.0), 1.0,)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign3910_e2692, assign3910_e2692_d_n10,)
    } else {
        (0.0, 0.0,)
    }
};
        var_deltemp = assign3910_e2694;
        var_deltemp_dn10 = assign3910_e2694_d_n10;
        var_deltemp_rv = 0.0;

        let assign3920_e2697: f64 = if var_vdsi >= 0.0 { 1.0 } else { 0.0 };
        var_guard32 = assign3920_e2697;
        var_guard32_rv = 0.0;

        let (assign3930_e2701,) = {
    if (var_guard32 != 0.0) {
        (1.0,)
    } else {
        (var_mode,)
    }
};
        var_mode = assign3930_e2701;
        var_mode_rv = 0.0;

        let (assign3940_e2705,) = {
    if (var_guard32 != 0.0) {
        (1.0,)
    } else {
        (var_modenml,)
    }
};
        var_modenml = assign3940_e2705;
        var_modenml_rv = 0.0;

        let (assign3950_e2709,) = {
    if (var_guard32 != 0.0) {
        (0.0,)
    } else {
        (var_modervs,)
    }
};
        var_modervs = assign3950_e2709;
        var_modervs_rv = 0.0;

        let (assign3960_e2713, assign3960_e2713_d_n0, assign3960_e2713_d_n2, assign3960_e2713_d_n6, assign3960_e2713_d_n7, assign3960_e2713_d_n10, assign3960_e2713_d_n11, assign3960_e2713_d_n12, assign3960_e2713_d_n17,) = {
    if (var_guard32 != 0.0) {
        (var_vdsi, 0.0, 0.0, var_vdsi_dn6, var_vdsi_dn7, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vds, var_vds_dn0, var_vds_dn2, var_vds_dn6, var_vds_dn7, var_vds_dn10, var_vds_dn11, var_vds_dn12, var_vds_dn17,)
    }
};
        var_vds = assign3960_e2713;
        var_vds_dn0 = assign3960_e2713_d_n0;
        var_vds_dn2 = assign3960_e2713_d_n2;
        var_vds_dn6 = assign3960_e2713_d_n6;
        var_vds_dn7 = assign3960_e2713_d_n7;
        var_vds_dn10 = assign3960_e2713_d_n10;
        var_vds_dn11 = assign3960_e2713_d_n11;
        var_vds_dn12 = assign3960_e2713_d_n12;
        var_vds_dn17 = assign3960_e2713_d_n17;
        var_vds_rv = 0.0;

        let (assign3970_e2717, assign3970_e2717_d_n6, assign3970_e2717_d_n7, assign3970_e2717_d_n11,) = {
    if (var_guard32 != 0.0) {
        (var_vgsi, 0.0, var_vgsi_dn7, var_vgsi_dn11,)
    } else {
        (var_vgs, var_vgs_dn6, var_vgs_dn7, var_vgs_dn11,)
    }
};
        var_vgs = assign3970_e2717;
        var_vgs_dn6 = assign3970_e2717_d_n6;
        var_vgs_dn7 = assign3970_e2717_d_n7;
        var_vgs_dn11 = assign3970_e2717_d_n11;
        var_vgs_rv = 0.0;

        let (assign3980_e2721, assign3980_e2721_d_n0, assign3980_e2721_d_n2, assign3980_e2721_d_n6, assign3980_e2721_d_n7, assign3980_e2721_d_n10, assign3980_e2721_d_n11, assign3980_e2721_d_n12, assign3980_e2721_d_n17,) = {
    if (var_guard32 != 0.0) {
        (var_vbsi, 0.0, 0.0, 0.0, var_vbsi_dn7, 0.0, 0.0, var_vbsi_dn12, 0.0,)
    } else {
        (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    }
};
        var_vbs = assign3980_e2721;
        var_vbs_dn0 = assign3980_e2721_d_n0;
        var_vbs_dn2 = assign3980_e2721_d_n2;
        var_vbs_dn6 = assign3980_e2721_d_n6;
        var_vbs_dn7 = assign3980_e2721_d_n7;
        var_vbs_dn10 = assign3980_e2721_d_n10;
        var_vbs_dn11 = assign3980_e2721_d_n11;
        var_vbs_dn12 = assign3980_e2721_d_n12;
        var_vbs_dn17 = assign3980_e2721_d_n17;
        var_vbs_rv = 0.0;

        let (assign4020_e2739,) = {
    if (var_guard32 == 0.0) {
        let assign4020_e2737: f64 = (-1.0);
        (assign4020_e2737,)
    } else {
        (var_mode,)
    }
};
        var_mode = assign4020_e2739;
        var_mode_rv = 0.0;

        let (assign4030_e2744,) = {
    if (var_guard32 == 0.0) {
        (0.0,)
    } else {
        (var_modenml,)
    }
};
        var_modenml = assign4030_e2744;
        var_modenml_rv = 0.0;

        let (assign4040_e2749,) = {
    if (var_guard32 == 0.0) {
        (1.0,)
    } else {
        (var_modervs,)
    }
};
        var_modervs = assign4040_e2749;
        var_modervs_rv = 0.0;

        let (assign4050_e2755, assign4050_e2755_d_n0, assign4050_e2755_d_n2, assign4050_e2755_d_n6, assign4050_e2755_d_n7, assign4050_e2755_d_n10, assign4050_e2755_d_n11, assign4050_e2755_d_n12, assign4050_e2755_d_n17,) = {
    if (var_guard32 == 0.0) {
        let assign4050_e2753: f64 = (-var_vdsi);
        (assign4050_e2753, 0.0, 0.0, (-var_vdsi_dn6), (-var_vdsi_dn7), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vds, var_vds_dn0, var_vds_dn2, var_vds_dn6, var_vds_dn7, var_vds_dn10, var_vds_dn11, var_vds_dn12, var_vds_dn17,)
    }
};
        var_vds = assign4050_e2755;
        var_vds_dn0 = assign4050_e2755_d_n0;
        var_vds_dn2 = assign4050_e2755_d_n2;
        var_vds_dn6 = assign4050_e2755_d_n6;
        var_vds_dn7 = assign4050_e2755_d_n7;
        var_vds_dn10 = assign4050_e2755_d_n10;
        var_vds_dn11 = assign4050_e2755_d_n11;
        var_vds_dn12 = assign4050_e2755_d_n12;
        var_vds_dn17 = assign4050_e2755_d_n17;
        var_vds_rv = 0.0;

        let (assign4060_e2762, assign4060_e2762_d_n6, assign4060_e2762_d_n7, assign4060_e2762_d_n11,) = {
    if (var_guard32 == 0.0) {
        let assign4060_e2760: f64 = (var_vgsi - var_vdsi);
        (assign4060_e2760, (-var_vdsi_dn6), (var_vgsi_dn7 - var_vdsi_dn7), var_vgsi_dn11,)
    } else {
        (var_vgs, var_vgs_dn6, var_vgs_dn7, var_vgs_dn11,)
    }
};
        var_vgs = assign4060_e2762;
        var_vgs_dn6 = assign4060_e2762_d_n6;
        var_vgs_dn7 = assign4060_e2762_d_n7;
        var_vgs_dn11 = assign4060_e2762_d_n11;
        var_vgs_rv = 0.0;

        let (assign4070_e2769, assign4070_e2769_d_n0, assign4070_e2769_d_n2, assign4070_e2769_d_n6, assign4070_e2769_d_n7, assign4070_e2769_d_n10, assign4070_e2769_d_n11, assign4070_e2769_d_n12, assign4070_e2769_d_n17,) = {
    if (var_guard32 == 0.0) {
        let assign4070_e2767: f64 = (var_vbsi - var_vdsi);
        (assign4070_e2767, 0.0, 0.0, (-var_vdsi_dn6), (var_vbsi_dn7 - var_vdsi_dn7), 0.0, 0.0, var_vbsi_dn12, 0.0,)
    } else {
        (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    }
};
        var_vbs = assign4070_e2769;
        var_vbs_dn0 = assign4070_e2769_d_n0;
        var_vbs_dn2 = assign4070_e2769_d_n2;
        var_vbs_dn6 = assign4070_e2769_d_n6;
        var_vbs_dn7 = assign4070_e2769_d_n7;
        var_vbs_dn10 = assign4070_e2769_d_n10;
        var_vbs_dn11 = assign4070_e2769_d_n11;
        var_vbs_dn12 = assign4070_e2769_d_n12;
        var_vbs_dn17 = assign4070_e2769_d_n17;
        var_vbs_rv = 0.0;

        let assign4130_e2796: f64 = ctx_temp;
        var_ttemp = assign4130_e2796;
        var_ttemp_dn10 = 0.0;
        var_ttemp_rv = 0.0;

        let (assign4140_e2800, assign4140_e2800_d_n10,) = {
    if (var_temp_given != 0.0) {
        (var_uc_temp, 0.0,)
    } else {
        (var_ttemp, var_ttemp_dn10,)
    }
};
        var_ttemp = assign4140_e2800;
        var_ttemp_dn10 = assign4140_e2800_d_n10;
        var_ttemp_rv = 0.0;

        let (assign4150_e2806, assign4150_e2806_d_n10,) = {
    if (var_dtemp_given != 0.0) {
        let assign4150_e2804: f64 = (var_ttemp + p.p17);
        (assign4150_e2804, var_ttemp_dn10,)
    } else {
        (var_ttemp, var_ttemp_dn10,)
    }
};
        var_ttemp = assign4150_e2806;
        var_ttemp_dn10 = assign4150_e2806_d_n10;
        var_ttemp_rv = 0.0;

        let assign4160_e2809: f64 = (var_ttemp + var_deltemp);
        var_ttemp = assign4160_e2809;
        var_ttemp_dn10 = (var_ttemp_dn10 + var_deltemp_dn10);
        var_ttemp_rv = 0.0;

        let assign4170_e2812: f64 = (var_ttemp - var_uc_tnom);
        var_t1 = assign4170_e2812;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn10 = var_ttemp_dn10;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn17 = 0.0;
        var_t1_rv = 0.0;

        let assign4180_e2816: f64 = (var_ttemp + var_uc_tnom);
        let assign4180_e2817: f64 = (var_t1 * assign4180_e2816);
        var_t2 = assign4180_e2817;
        var_t2_dn0 = (var_t1_dn0 * assign4180_e2816);
        var_t2_dn2 = (var_t1_dn2 * assign4180_e2816);
        var_t2_dn6 = (var_t1_dn6 * assign4180_e2816);
        var_t2_dn7 = (var_t1_dn7 * assign4180_e2816);
        var_t2_dn10 = ((var_t1_dn10 * assign4180_e2816) + (var_t1 * var_ttemp_dn10));
        var_t2_dn11 = (var_t1_dn11 * assign4180_e2816);
        var_t2_dn12 = (var_t1_dn12 * assign4180_e2816);
        var_t2_dn17 = (var_t1_dn17 * assign4180_e2816);
        var_t2_rv = 0.0;

        let assign4190_e2821: f64 = (p.p53 * var_t1);
        let assign4190_e2822: f64 = (var_egtnom - assign4190_e2821);
        let assign4190_e2825: f64 = (p.p54 * var_t2);
        let assign4190_e2826: f64 = (assign4190_e2822 - assign4190_e2825);
        var_eg = assign4190_e2826;
        var_eg_dn0 = ((-(p.p53 * var_t1_dn0)) - (p.p54 * var_t2_dn0));
        var_eg_dn2 = ((-(p.p53 * var_t1_dn2)) - (p.p54 * var_t2_dn2));
        var_eg_dn6 = ((-(p.p53 * var_t1_dn6)) - (p.p54 * var_t2_dn6));
        var_eg_dn7 = ((-(p.p53 * var_t1_dn7)) - (p.p54 * var_t2_dn7));
        var_eg_dn10 = ((-(p.p53 * var_t1_dn10)) - (p.p54 * var_t2_dn10));
        var_eg_dn11 = ((-(p.p53 * var_t1_dn11)) - (p.p54 * var_t2_dn11));
        var_eg_dn12 = ((-(p.p53 * var_t1_dn12)) - (p.p54 * var_t2_dn12));
        var_eg_dn17 = ((-(p.p53 * var_t1_dn17)) - (p.p54 * var_t2_dn17));
        var_eg_rv = 0.0;

        let assign4200_e2830: f64 = (1.3806226e-23 * var_ttemp);
        let assign4200_e2831: f64 = (1.6021918e-19 / assign4200_e2830);
        var_beta = assign4200_e2831;
        var_beta_dn10 = (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn10)) / (assign4200_e2830 * assign4200_e2830)));
        var_beta_rv = 0.0;

        let assign4210_e2834: f64 = (var_beta * var_beta);
        var_beta2 = assign4210_e2834;
        var_beta2_dn10 = ((var_beta_dn10 * var_beta) + (var_beta * var_beta_dn10));
        var_beta2_rv = 0.0;

        let assign4220_e2837: f64 = (1.0 / var_beta);
        var_beta_inv = assign4220_e2837;
        var_beta_inv_dn10 = (-(var_beta_dn10 / (var_beta * var_beta)));
        var_beta_inv_rv = 0.0;

        let assign4230_e2843: f64 = (var_wg).powf(p.p99);
        let assign4230_e2844: f64 = (p.p98 / assign4230_e2843);
        let assign4230_e2845: f64 = (1.0 + assign4230_e2844);
        let assign4230_e2846: f64 = (p.p254 * assign4230_e2845);
        let assign4230_e2851: f64 = (var_lgle).powf(p.p101);
        let assign4230_e2852: f64 = (p.p100 / assign4230_e2851);
        let assign4230_e2853: f64 = (1.0 + assign4230_e2852);
        let assign4230_e2854: f64 = (assign4230_e2846 * assign4230_e2853);
        let assign4230_e2859: f64 = (var_wl).powf(p.p103);
        let assign4230_e2860: f64 = (p.p102 / assign4230_e2859);
        let assign4230_e2861: f64 = (1.0 + assign4230_e2860);
        let assign4230_e2862: f64 = (assign4230_e2854 * assign4230_e2861);
        var_cgs_mueph = assign4230_e2862;
        var_cgs_mueph_rv = 0.0;

        let assign4240_e2866: f64 = (1.0 + p.p159);
        let assign4240_e2867: f64 = (1.0 / assign4240_e2866);
        var_t2__blk38 = assign4240_e2867;
        var_t2__blk38_rv = 0.0;

        var_t3__blk39 = 0.0;
        var_t3__blk39_rv = 0.0;

        let assign4260_e2873: f64 = (var_t2__blk38 * var_t3__blk39);
        let assign4260_e2874: f64 = (1.0 + assign4260_e2873);
        let assign4260_e2875: f64 = (var_cgs_mueph * assign4260_e2874);
        var_cgs_wmueph = assign4260_e2875;
        var_cgs_wmueph_rv = 0.0;

        let assign4270_e2878: f64 = (var_ttemp / var_uc_tnom);
        let assign4270_e2880: f64 = (assign4270_e2878).powf(p.p112);
        var_t1__blk37 = assign4270_e2880;
        var_t1__blk37_dn10 = if 0.0 == 0.0 && ((p.p112) as f64).is_finite() && ((p.p112) as f64).fract() == 0.0 { if p.p112 == 0.0 { 0.0 } else { (p.p112 * ((assign4270_e2878).powf(p.p112 - 1.0) * (var_ttemp_dn10 / var_uc_tnom))) } } else { (assign4270_e2880 * (p.p112 * ((var_ttemp_dn10 / var_uc_tnom) / assign4270_e2878))) };
        var_t1__blk37_rv = 0.0;

        let assign4280_e2883: f64 = (var_t1__blk37 / var_cgs_wmueph);
        var_cgs_mphn0 = assign4280_e2883;
        var_cgs_mphn0_dn10 = (var_t1__blk37_dn10 / var_cgs_wmueph);
        var_cgs_mphn0_rv = 0.0;

        let assign4290_e2886: f64 = (var_ptovr0 * var_beta_inv);
        var_ptovr = assign4290_e2886;
        var_ptovr_dn0 = (var_ptovr0_dn0 * var_beta_inv);
        var_ptovr_dn2 = (var_ptovr0_dn2 * var_beta_inv);
        var_ptovr_dn6 = (var_ptovr0_dn6 * var_beta_inv);
        var_ptovr_dn7 = (var_ptovr0_dn7 * var_beta_inv);
        var_ptovr_dn10 = ((var_ptovr0_dn10 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn10));
        var_ptovr_dn11 = (var_ptovr0_dn11 * var_beta_inv);
        var_ptovr_dn12 = (var_ptovr0_dn12 * var_beta_inv);
        var_ptovr_dn17 = (var_ptovr0_dn17 * var_beta_inv);
        var_ptovr_rv = 0.0;

        let assign4300_e2889: f64 = (var_ttemp / var_uc_tnom);
        var_t1 = assign4300_e2889;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn10 = (var_ttemp_dn10 / var_uc_tnom);
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn17 = 0.0;
        var_t1_rv = 0.0;

        let assign4310_e2892: f64 = (var_vmax0 * var_mks_vmax);
        let assign4310_e2896: f64 = (0.4 * var_t1);
        let assign4310_e2897: f64 = (1.8 + assign4310_e2896);
        let assign4310_e2900: f64 = (0.1 * var_t1);
        let assign4310_e2902: f64 = (assign4310_e2900 * var_t1);
        let assign4310_e2903: f64 = (assign4310_e2897 + assign4310_e2902);
        let assign4310_e2907: f64 = (1.0 - var_t1);
        let assign4310_e2908: f64 = (var_mks_vtmp * assign4310_e2907);
        let assign4310_e2909: f64 = (assign4310_e2903 - assign4310_e2908);
        let assign4310_e2910: f64 = (assign4310_e2892 / assign4310_e2909);
        var_vmaxe = assign4310_e2910;
        var_vmaxe_dn0 = ((((var_vmax0_dn0 * var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * var_t1_dn0) + (((0.1 * var_t1_dn0) * var_t1) + (assign4310_e2900 * var_t1_dn0))) - (var_mks_vtmp * (-var_t1_dn0))))) / (assign4310_e2909 * assign4310_e2909));
        var_vmaxe_dn2 = ((((var_vmax0_dn2 * var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * var_t1_dn2) + (((0.1 * var_t1_dn2) * var_t1) + (assign4310_e2900 * var_t1_dn2))) - (var_mks_vtmp * (-var_t1_dn2))))) / (assign4310_e2909 * assign4310_e2909));
        var_vmaxe_dn6 = ((((var_vmax0_dn6 * var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * var_t1_dn6) + (((0.1 * var_t1_dn6) * var_t1) + (assign4310_e2900 * var_t1_dn6))) - (var_mks_vtmp * (-var_t1_dn6))))) / (assign4310_e2909 * assign4310_e2909));
        var_vmaxe_dn7 = ((((var_vmax0_dn7 * var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * var_t1_dn7) + (((0.1 * var_t1_dn7) * var_t1) + (assign4310_e2900 * var_t1_dn7))) - (var_mks_vtmp * (-var_t1_dn7))))) / (assign4310_e2909 * assign4310_e2909));
        var_vmaxe_dn10 = ((((var_vmax0_dn10 * var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * var_t1_dn10) + (((0.1 * var_t1_dn10) * var_t1) + (assign4310_e2900 * var_t1_dn10))) - (var_mks_vtmp * (-var_t1_dn10))))) / (assign4310_e2909 * assign4310_e2909));
        var_vmaxe_dn11 = ((((var_vmax0_dn11 * var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * var_t1_dn11) + (((0.1 * var_t1_dn11) * var_t1) + (assign4310_e2900 * var_t1_dn11))) - (var_mks_vtmp * (-var_t1_dn11))))) / (assign4310_e2909 * assign4310_e2909));
        var_vmaxe_dn12 = ((((var_vmax0_dn12 * var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * var_t1_dn12) + (((0.1 * var_t1_dn12) * var_t1) + (assign4310_e2900 * var_t1_dn12))) - (var_mks_vtmp * (-var_t1_dn12))))) / (assign4310_e2909 * assign4310_e2909));
        var_vmaxe_dn17 = ((((var_vmax0_dn17 * var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * var_t1_dn17) + (((0.1 * var_t1_dn17) * var_t1) + (assign4310_e2900 * var_t1_dn17))) - (var_mks_vtmp * (-var_t1_dn17))))) / (assign4310_e2909 * assign4310_e2909));
        var_vmaxe_rv = 0.0;

        let assign4320_e2912: f64 = (var_eg).sqrt();
        var_egp12 = assign4320_e2912;
        var_egp12_dn0 = (var_eg_dn0 / (2.0 * assign4320_e2912));
        var_egp12_dn2 = (var_eg_dn2 / (2.0 * assign4320_e2912));
        var_egp12_dn6 = (var_eg_dn6 / (2.0 * assign4320_e2912));
        var_egp12_dn7 = (var_eg_dn7 / (2.0 * assign4320_e2912));
        var_egp12_dn10 = (var_eg_dn10 / (2.0 * assign4320_e2912));
        var_egp12_dn11 = (var_eg_dn11 / (2.0 * assign4320_e2912));
        var_egp12_dn12 = (var_eg_dn12 / (2.0 * assign4320_e2912));
        var_egp12_dn17 = (var_eg_dn17 / (2.0 * assign4320_e2912));
        var_egp12_rv = 0.0;

        let assign4330_e2915: f64 = (var_eg * var_egp12);
        var_egp32 = assign4330_e2915;
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
        *var_guard32_slot = var_guard32;
        *var_guard32_rv_slot = var_guard32_rv;
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
        *var_t1__blk37_slot = var_t1__blk37;
        *var_t1__blk37_dn10_slot = var_t1__blk37_dn10;
        *var_t1__blk37_rv_slot = var_t1__blk37_rv;
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
        *var_t2__blk38_slot = var_t2__blk38;
        *var_t2__blk38_rv_slot = var_t2__blk38_rv;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3__blk39_slot = var_t3__blk39;
        *var_t3__blk39_rv_slot = var_t3__blk39_rv;
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
        var_guard40_slot: &mut f64,
        var_guard40_rv_slot: &mut f64,
        var_guard41_slot: &mut f64,
        var_guard41_rv_slot: &mut f64,
        var_guard46_slot: &mut f64,
        var_guard46_rv_slot: &mut f64,
        var_guard47_slot: &mut f64,
        var_guard47_rv_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_guard48_rv_slot: &mut f64,
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
        let mut var_guard40: f64 = *var_guard40_slot;
        let mut var_guard40_rv: f64 = *var_guard40_rv_slot;
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_guard41_rv: f64 = *var_guard41_rv_slot;
        let mut var_guard46: f64 = *var_guard46_slot;
        let mut var_guard46_rv: f64 = *var_guard46_rv_slot;
        let mut var_guard47: f64 = *var_guard47_slot;
        let mut var_guard47_rv: f64 = *var_guard47_rv_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_guard48_rv: f64 = *var_guard48_rv_slot;
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

        let assign4340_e2918: f64 = (10400000000.0 / 1e-6);
        let assign4340_e2921: f64 = (var_ttemp / var_uc_tnom);
        let assign4340_e2923: f64 = (assign4340_e2921).powf(1.5);
        let assign4340_e2924: f64 = (assign4340_e2918 * assign4340_e2923);
        let assign4340_e2926: f64 = (-var_eg);
        let assign4340_e2928: f64 = (assign4340_e2926 / 2.0);
        let assign4340_e2930: f64 = (assign4340_e2928 * var_beta);
        let assign4340_e2933: f64 = (var_egtnom / 2.0);
        let assign4340_e2935: f64 = (assign4340_e2933 * var_betatnom);
        let assign4340_e2936: f64 = (assign4340_e2930 + assign4340_e2935);
        let assign4340_e2937: f64 = (assign4340_e2936).exp();
        let assign4340_e2938: f64 = (assign4340_e2924 * assign4340_e2937);
        var_nin = assign4340_e2938;
        var_nin_dn0 = (assign4340_e2924 * (assign4340_e2937 * (((-var_eg_dn0) / 2.0) * var_beta)));
        var_nin_dn2 = (assign4340_e2924 * (assign4340_e2937 * (((-var_eg_dn2) / 2.0) * var_beta)));
        var_nin_dn6 = (assign4340_e2924 * (assign4340_e2937 * (((-var_eg_dn6) / 2.0) * var_beta)));
        var_nin_dn7 = (assign4340_e2924 * (assign4340_e2937 * (((-var_eg_dn7) / 2.0) * var_beta)));
        var_nin_dn10 = (((assign4340_e2918 * if 0.0 == 0.0 && ((1.5) as f64).is_finite() && ((1.5) as f64).fract() == 0.0 { if 1.5 == 0.0 { 0.0 } else { (1.5 * ((assign4340_e2921).powf(1.5 - 1.0) * (var_ttemp_dn10 / var_uc_tnom))) } } else { (assign4340_e2923 * (1.5 * ((var_ttemp_dn10 / var_uc_tnom) / assign4340_e2921))) }) * assign4340_e2937) + (assign4340_e2924 * (assign4340_e2937 * ((((-var_eg_dn10) / 2.0) * var_beta) + (assign4340_e2928 * var_beta_dn10)))));
        var_nin_dn11 = (assign4340_e2924 * (assign4340_e2937 * (((-var_eg_dn11) / 2.0) * var_beta)));
        var_nin_dn12 = (assign4340_e2924 * (assign4340_e2937 * (((-var_eg_dn12) / 2.0) * var_beta)));
        var_nin_dn17 = (assign4340_e2924 * (assign4340_e2937 * (((-var_eg_dn17) / 2.0) * var_beta)));
        var_nin_rv = 0.0;

        let assign4350_e2941: f64 = (var_beta_inv).sqrt();
        let assign4350_e2942: f64 = (var_costi00 * assign4350_e2941);
        var_costi0 = assign4350_e2942;
        var_costi0_dn0 = 0.0;
        var_costi0_dn2 = 0.0;
        var_costi0_dn6 = 0.0;
        var_costi0_dn7 = 0.0;
        var_costi0_dn10 = (var_costi00 * (var_beta_inv_dn10 / (2.0 * assign4350_e2941)));
        var_costi0_dn11 = 0.0;
        var_costi0_dn12 = 0.0;
        var_costi0_dn17 = 0.0;
        var_costi0_rv = 0.0;

        let assign4360_e2945: f64 = (var_costi0 * var_costi0);
        var_costi0_p2 = assign4360_e2945;
        var_costi0_p2_dn0 = ((var_costi0_dn0 * var_costi0) + (var_costi0 * var_costi0_dn0));
        var_costi0_p2_dn2 = ((var_costi0_dn2 * var_costi0) + (var_costi0 * var_costi0_dn2));
        var_costi0_p2_dn6 = ((var_costi0_dn6 * var_costi0) + (var_costi0 * var_costi0_dn6));
        var_costi0_p2_dn7 = ((var_costi0_dn7 * var_costi0) + (var_costi0 * var_costi0_dn7));
        var_costi0_p2_dn10 = ((var_costi0_dn10 * var_costi0) + (var_costi0 * var_costi0_dn10));
        var_costi0_p2_dn11 = ((var_costi0_dn11 * var_costi0) + (var_costi0 * var_costi0_dn11));
        var_costi0_p2_dn12 = ((var_costi0_dn12 * var_costi0) + (var_costi0 * var_costi0_dn12));
        var_costi0_p2_dn17 = ((var_costi0_dn17 * var_costi0) + (var_costi0 * var_costi0_dn17));
        var_costi0_p2_rv = 0.0;

        let assign4370_e2948: f64 = (var_nin * var_nin);
        let assign4370_e2950: f64 = (assign4370_e2948 * var_nsti_p2);
        var_costi1 = assign4370_e2950;
        var_costi1_dn0 = (((var_nin_dn0 * var_nin) + (var_nin * var_nin_dn0)) * var_nsti_p2);
        var_costi1_dn2 = (((var_nin_dn2 * var_nin) + (var_nin * var_nin_dn2)) * var_nsti_p2);
        var_costi1_dn6 = (((var_nin_dn6 * var_nin) + (var_nin * var_nin_dn6)) * var_nsti_p2);
        var_costi1_dn7 = (((var_nin_dn7 * var_nin) + (var_nin * var_nin_dn7)) * var_nsti_p2);
        var_costi1_dn10 = (((var_nin_dn10 * var_nin) + (var_nin * var_nin_dn10)) * var_nsti_p2);
        var_costi1_dn11 = (((var_nin_dn11 * var_nin) + (var_nin * var_nin_dn11)) * var_nsti_p2);
        var_costi1_dn12 = (((var_nin_dn12 * var_nin) + (var_nin * var_nin_dn12)) * var_nsti_p2);
        var_costi1_dn17 = (((var_nin_dn17 * var_nin) + (var_nin * var_nin_dn17)) * var_nsti_p2);
        var_costi1_rv = 0.0;

        let assign4380_e2954: f64 = (2.0 * p.p56);
        let assign4380_e2955: f64 = (var_lgate - assign4380_e2954);
        var_lch = assign4380_e2955;
        var_lch_dn0 = 0.0;
        var_lch_dn2 = 0.0;
        var_lch_dn6 = 0.0;
        var_lch_dn7 = 0.0;
        var_lch_dn10 = 0.0;
        var_lch_dn11 = 0.0;
        var_lch_dn12 = 0.0;
        var_lch_dn17 = 0.0;
        var_lch_rv = 0.0;

        let assign4390_e2958: f64 = if var_subversion > 3.0 { 1.0 } else { 0.0 };
        var_guard40 = assign4390_e2958;
        var_guard40_rv = 0.0;

        let (assign4400_e2969, assign4400_e2969_d_n0, assign4400_e2969_d_n2, assign4400_e2969_d_n6, assign4400_e2969_d_n7, assign4400_e2969_d_n10, assign4400_e2969_d_n11, assign4400_e2969_d_n12, assign4400_e2969_d_n17,) = {
    if (var_guard40 != 0.0) {
        let assign4400_e2962: f64 = (2.0 * var_beta_inv);
        let assign4400_e2965: f64 = (var_nsub / var_nin);
        let assign4400_e2966: f64 = (assign4400_e2965).ln();
        let assign4400_e2967: f64 = (assign4400_e2962 * assign4400_e2966);
        (assign4400_e2967, (assign4400_e2962 * ((((var_nsub_dn0 * var_nin) - (var_nsub * var_nin_dn0)) / (var_nin * var_nin)) / assign4400_e2965)), (assign4400_e2962 * ((((var_nsub_dn2 * var_nin) - (var_nsub * var_nin_dn2)) / (var_nin * var_nin)) / assign4400_e2965)), (assign4400_e2962 * ((((var_nsub_dn6 * var_nin) - (var_nsub * var_nin_dn6)) / (var_nin * var_nin)) / assign4400_e2965)), (assign4400_e2962 * ((((var_nsub_dn7 * var_nin) - (var_nsub * var_nin_dn7)) / (var_nin * var_nin)) / assign4400_e2965)), (((2.0 * var_beta_inv_dn10) * assign4400_e2966) + (assign4400_e2962 * ((((var_nsub_dn10 * var_nin) - (var_nsub * var_nin_dn10)) / (var_nin * var_nin)) / assign4400_e2965))), (assign4400_e2962 * ((((var_nsub_dn11 * var_nin) - (var_nsub * var_nin_dn11)) / (var_nin * var_nin)) / assign4400_e2965)), (assign4400_e2962 * ((((var_nsub_dn12 * var_nin) - (var_nsub * var_nin_dn12)) / (var_nin * var_nin)) / assign4400_e2965)), (assign4400_e2962 * ((((var_nsub_dn17 * var_nin) - (var_nsub * var_nin_dn17)) / (var_nin * var_nin)) / assign4400_e2965)),)
    } else {
        (var_pb2, var_pb2_dn0, var_pb2_dn2, var_pb2_dn6, var_pb2_dn7, var_pb2_dn10, var_pb2_dn11, var_pb2_dn12, var_pb2_dn17,)
    }
};
        var_pb2 = assign4400_e2969;
        var_pb2_dn0 = assign4400_e2969_d_n0;
        var_pb2_dn2 = assign4400_e2969_d_n2;
        var_pb2_dn6 = assign4400_e2969_d_n6;
        var_pb2_dn7 = assign4400_e2969_d_n7;
        var_pb2_dn10 = assign4400_e2969_d_n10;
        var_pb2_dn11 = assign4400_e2969_d_n11;
        var_pb2_dn12 = assign4400_e2969_d_n12;
        var_pb2_dn17 = assign4400_e2969_d_n17;
        var_pb2_rv = 0.0;

        let (assign4410_e2981, assign4410_e2981_d_n0, assign4410_e2981_d_n2, assign4410_e2981_d_n6, assign4410_e2981_d_n7, assign4410_e2981_d_n10, assign4410_e2981_d_n11, assign4410_e2981_d_n12, assign4410_e2981_d_n17,) = {
    if (var_guard40 == 0.0) {
        let assign4410_e2974: f64 = (2.0 * var_beta_inv);
        let assign4410_e2977: f64 = (var_uc_nsubs / var_nin);
        let assign4410_e2978: f64 = (assign4410_e2977).ln();
        let assign4410_e2979: f64 = (assign4410_e2974 * assign4410_e2978);
        (assign4410_e2979, (assign4410_e2974 * ((((var_uc_nsubs_dn0 * var_nin) - (var_uc_nsubs * var_nin_dn0)) / (var_nin * var_nin)) / assign4410_e2977)), (assign4410_e2974 * ((((var_uc_nsubs_dn2 * var_nin) - (var_uc_nsubs * var_nin_dn2)) / (var_nin * var_nin)) / assign4410_e2977)), (assign4410_e2974 * ((((var_uc_nsubs_dn6 * var_nin) - (var_uc_nsubs * var_nin_dn6)) / (var_nin * var_nin)) / assign4410_e2977)), (assign4410_e2974 * ((((var_uc_nsubs_dn7 * var_nin) - (var_uc_nsubs * var_nin_dn7)) / (var_nin * var_nin)) / assign4410_e2977)), (((2.0 * var_beta_inv_dn10) * assign4410_e2978) + (assign4410_e2974 * ((((var_uc_nsubs_dn10 * var_nin) - (var_uc_nsubs * var_nin_dn10)) / (var_nin * var_nin)) / assign4410_e2977))), (assign4410_e2974 * ((((var_uc_nsubs_dn11 * var_nin) - (var_uc_nsubs * var_nin_dn11)) / (var_nin * var_nin)) / assign4410_e2977)), (assign4410_e2974 * ((((var_uc_nsubs_dn12 * var_nin) - (var_uc_nsubs * var_nin_dn12)) / (var_nin * var_nin)) / assign4410_e2977)), (assign4410_e2974 * ((((var_uc_nsubs_dn17 * var_nin) - (var_uc_nsubs * var_nin_dn17)) / (var_nin * var_nin)) / assign4410_e2977)),)
    } else {
        (var_pb2, var_pb2_dn0, var_pb2_dn2, var_pb2_dn6, var_pb2_dn7, var_pb2_dn10, var_pb2_dn11, var_pb2_dn12, var_pb2_dn17,)
    }
};
        var_pb2 = assign4410_e2981;
        var_pb2_dn0 = assign4410_e2981_d_n0;
        var_pb2_dn2 = assign4410_e2981_d_n2;
        var_pb2_dn6 = assign4410_e2981_d_n6;
        var_pb2_dn7 = assign4410_e2981_d_n7;
        var_pb2_dn10 = assign4410_e2981_d_n10;
        var_pb2_dn11 = assign4410_e2981_d_n11;
        var_pb2_dn12 = assign4410_e2981_d_n12;
        var_pb2_dn17 = assign4410_e2981_d_n17;
        var_pb2_rv = 0.0;

        let assign4420_e2984: f64 = (1.034943e-10 / var_q_nsub);
        let assign4420_e2986: f64 = (assign4420_e2984 * var_beta_inv);
        let assign4420_e2987: f64 = (assign4420_e2986).sqrt();
        var_ldby = assign4420_e2987;
        var_ldby_dn0 = (((-((1.034943e-10 * var_q_nsub_dn0) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4420_e2987));
        var_ldby_dn2 = (((-((1.034943e-10 * var_q_nsub_dn2) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4420_e2987));
        var_ldby_dn6 = (((-((1.034943e-10 * var_q_nsub_dn6) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4420_e2987));
        var_ldby_dn7 = (((-((1.034943e-10 * var_q_nsub_dn7) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4420_e2987));
        var_ldby_dn10 = ((((-((1.034943e-10 * var_q_nsub_dn10) / (var_q_nsub * var_q_nsub))) * var_beta_inv) + (assign4420_e2984 * var_beta_inv_dn10)) / (2.0 * assign4420_e2987));
        var_ldby_dn11 = (((-((1.034943e-10 * var_q_nsub_dn11) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4420_e2987));
        var_ldby_dn12 = (((-((1.034943e-10 * var_q_nsub_dn12) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4420_e2987));
        var_ldby_dn17 = (((-((1.034943e-10 * var_q_nsub_dn17) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4420_e2987));
        var_ldby_rv = 0.0;

        let assign4430_e2990: f64 = (var_q_nsub * 1.414213562373095);
        let assign4430_e2992: f64 = (assign4430_e2990 * var_ldby);
        var_cnst0soi = assign4430_e2992;
        var_cnst0soi_dn0 = (((var_q_nsub_dn0 * 1.414213562373095) * var_ldby) + (assign4430_e2990 * var_ldby_dn0));
        var_cnst0soi_dn2 = (((var_q_nsub_dn2 * 1.414213562373095) * var_ldby) + (assign4430_e2990 * var_ldby_dn2));
        var_cnst0soi_dn6 = (((var_q_nsub_dn6 * 1.414213562373095) * var_ldby) + (assign4430_e2990 * var_ldby_dn6));
        var_cnst0soi_dn7 = (((var_q_nsub_dn7 * 1.414213562373095) * var_ldby) + (assign4430_e2990 * var_ldby_dn7));
        var_cnst0soi_dn10 = (((var_q_nsub_dn10 * 1.414213562373095) * var_ldby) + (assign4430_e2990 * var_ldby_dn10));
        var_cnst0soi_dn11 = (((var_q_nsub_dn11 * 1.414213562373095) * var_ldby) + (assign4430_e2990 * var_ldby_dn11));
        var_cnst0soi_dn12 = (((var_q_nsub_dn12 * 1.414213562373095) * var_ldby) + (assign4430_e2990 * var_ldby_dn12));
        var_cnst0soi_dn17 = (((var_q_nsub_dn17 * 1.414213562373095) * var_ldby) + (assign4430_e2990 * var_ldby_dn17));
        var_cnst0soi_rv = 0.0;

        let assign4440_e2995: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard41 = assign4440_e2995;
        var_guard41_rv = 0.0;

        let (assign4450_e2999, assign4450_e2999_d_n10,) = {
    if (var_guard41 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_cnst0bulk, var_cnst0bulk_dn10,)
    }
};
        var_cnst0bulk = assign4450_e2999;
        var_cnst0bulk_dn10 = assign4450_e2999_d_n10;
        var_cnst0bulk_rv = 0.0;

        let (assign4460_e3003, assign4460_e3003_d_n0, assign4460_e3003_d_n2, assign4460_e3003_d_n6, assign4460_e3003_d_n7, assign4460_e3003_d_n10, assign4460_e3003_d_n11, assign4460_e3003_d_n12, assign4460_e3003_d_n17,) = {
    if (var_guard41 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cnst1bulk, var_cnst1bulk_dn0, var_cnst1bulk_dn2, var_cnst1bulk_dn6, var_cnst1bulk_dn7, var_cnst1bulk_dn10, var_cnst1bulk_dn11, var_cnst1bulk_dn12, var_cnst1bulk_dn17,)
    }
};
        var_cnst1bulk = assign4460_e3003;
        var_cnst1bulk_dn0 = assign4460_e3003_d_n0;
        var_cnst1bulk_dn2 = assign4460_e3003_d_n2;
        var_cnst1bulk_dn6 = assign4460_e3003_d_n6;
        var_cnst1bulk_dn7 = assign4460_e3003_d_n7;
        var_cnst1bulk_dn10 = assign4460_e3003_d_n10;
        var_cnst1bulk_dn11 = assign4460_e3003_d_n11;
        var_cnst1bulk_dn12 = assign4460_e3003_d_n12;
        var_cnst1bulk_dn17 = assign4460_e3003_d_n17;
        var_cnst1bulk_rv = 0.0;

        let (assign4470_e3009, assign4470_e3009_d_n0, assign4470_e3009_d_n2, assign4470_e3009_d_n6, assign4470_e3009_d_n7, assign4470_e3009_d_n10, assign4470_e3009_d_n11, assign4470_e3009_d_n12, assign4470_e3009_d_n17,) = {
    if (var_guard41 != 0.0) {
        let assign4470_e3007: f64 = (var_nin / var_nsub);
        (assign4470_e3007, (((var_nin_dn0 * var_nsub) - (var_nin * var_nsub_dn0)) / (var_nsub * var_nsub)), (((var_nin_dn2 * var_nsub) - (var_nin * var_nsub_dn2)) / (var_nsub * var_nsub)), (((var_nin_dn6 * var_nsub) - (var_nin * var_nsub_dn6)) / (var_nsub * var_nsub)), (((var_nin_dn7 * var_nsub) - (var_nin * var_nsub_dn7)) / (var_nsub * var_nsub)), (((var_nin_dn10 * var_nsub) - (var_nin * var_nsub_dn10)) / (var_nsub * var_nsub)), (((var_nin_dn11 * var_nsub) - (var_nin * var_nsub_dn11)) / (var_nsub * var_nsub)), (((var_nin_dn12 * var_nsub) - (var_nin * var_nsub_dn12)) / (var_nsub * var_nsub)), (((var_nin_dn17 * var_nsub) - (var_nin * var_nsub_dn17)) / (var_nsub * var_nsub)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign4470_e3009;
        var_t1_dn0 = assign4470_e3009_d_n0;
        var_t1_dn2 = assign4470_e3009_d_n2;
        var_t1_dn6 = assign4470_e3009_d_n6;
        var_t1_dn7 = assign4470_e3009_d_n7;
        var_t1_dn10 = assign4470_e3009_d_n10;
        var_t1_dn11 = assign4470_e3009_d_n11;
        var_t1_dn12 = assign4470_e3009_d_n12;
        var_t1_dn17 = assign4470_e3009_d_n17;
        var_t1_rv = 0.0;

        let (assign4480_e3019, assign4480_e3019_d_n10,) = {
    if (var_guard41 == 0.0) {
        let assign4480_e3014: f64 = (2.0 * var_c0bulk);
        let assign4480_e3016: f64 = (assign4480_e3014 * var_beta_inv);
        let assign4480_e3017: f64 = (assign4480_e3016).sqrt();
        (assign4480_e3017, ((assign4480_e3014 * var_beta_inv_dn10) / (2.0 * assign4480_e3017)),)
    } else {
        (var_cnst0bulk, var_cnst0bulk_dn10,)
    }
};
        var_cnst0bulk = assign4480_e3019;
        var_cnst0bulk_dn10 = assign4480_e3019_d_n10;
        var_cnst0bulk_rv = 0.0;

        let (assign4490_e3026, assign4490_e3026_d_n0, assign4490_e3026_d_n2, assign4490_e3026_d_n6, assign4490_e3026_d_n7, assign4490_e3026_d_n10, assign4490_e3026_d_n11, assign4490_e3026_d_n12, assign4490_e3026_d_n17,) = {
    if (var_guard41 == 0.0) {
        let assign4490_e3024: f64 = (var_nin / var_mks_nsubb);
        (assign4490_e3024, (var_nin_dn0 / var_mks_nsubb), (var_nin_dn2 / var_mks_nsubb), (var_nin_dn6 / var_mks_nsubb), (var_nin_dn7 / var_mks_nsubb), (var_nin_dn10 / var_mks_nsubb), (var_nin_dn11 / var_mks_nsubb), (var_nin_dn12 / var_mks_nsubb), (var_nin_dn17 / var_mks_nsubb),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign4490_e3026;
        var_t1_dn0 = assign4490_e3026_d_n0;
        var_t1_dn2 = assign4490_e3026_d_n2;
        var_t1_dn6 = assign4490_e3026_d_n6;
        var_t1_dn7 = assign4490_e3026_d_n7;
        var_t1_dn10 = assign4490_e3026_d_n10;
        var_t1_dn11 = assign4490_e3026_d_n11;
        var_t1_dn12 = assign4490_e3026_d_n12;
        var_t1_dn17 = assign4490_e3026_d_n17;
        var_t1_rv = 0.0;

        let (assign4500_e3033, assign4500_e3033_d_n0, assign4500_e3033_d_n2, assign4500_e3033_d_n6, assign4500_e3033_d_n7, assign4500_e3033_d_n10, assign4500_e3033_d_n11, assign4500_e3033_d_n12, assign4500_e3033_d_n17,) = {
    if (var_guard41 == 0.0) {
        let assign4500_e3031: f64 = (var_t1 * var_t1);
        (assign4500_e3031, ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)), ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)), ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)), ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)), ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)), ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)), ((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)), ((var_t1_dn17 * var_t1) + (var_t1 * var_t1_dn17)),)
    } else {
        (var_cnst1bulk, var_cnst1bulk_dn0, var_cnst1bulk_dn2, var_cnst1bulk_dn6, var_cnst1bulk_dn7, var_cnst1bulk_dn10, var_cnst1bulk_dn11, var_cnst1bulk_dn12, var_cnst1bulk_dn17,)
    }
};
        var_cnst1bulk = assign4500_e3033;
        var_cnst1bulk_dn0 = assign4500_e3033_d_n0;
        var_cnst1bulk_dn2 = assign4500_e3033_d_n2;
        var_cnst1bulk_dn6 = assign4500_e3033_d_n6;
        var_cnst1bulk_dn7 = assign4500_e3033_d_n7;
        var_cnst1bulk_dn10 = assign4500_e3033_d_n10;
        var_cnst1bulk_dn11 = assign4500_e3033_d_n11;
        var_cnst1bulk_dn12 = assign4500_e3033_d_n12;
        var_cnst1bulk_dn17 = assign4500_e3033_d_n17;
        var_cnst1bulk_rv = 0.0;

        let (assign4510_e3040, assign4510_e3040_d_n0, assign4510_e3040_d_n2, assign4510_e3040_d_n6, assign4510_e3040_d_n7, assign4510_e3040_d_n10, assign4510_e3040_d_n11, assign4510_e3040_d_n12, assign4510_e3040_d_n17,) = {
    if (var_guard41 == 0.0) {
        let assign4510_e3038: f64 = (var_nin / var_uc_nsubs);
        (assign4510_e3038, (((var_nin_dn0 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn2 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn6 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn7 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn7)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn10 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn11 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn12 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn17 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn17)) / (var_uc_nsubs * var_uc_nsubs)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign4510_e3040;
        var_t1_dn0 = assign4510_e3040_d_n0;
        var_t1_dn2 = assign4510_e3040_d_n2;
        var_t1_dn6 = assign4510_e3040_d_n6;
        var_t1_dn7 = assign4510_e3040_d_n7;
        var_t1_dn10 = assign4510_e3040_d_n10;
        var_t1_dn11 = assign4510_e3040_d_n11;
        var_t1_dn12 = assign4510_e3040_d_n12;
        var_t1_dn17 = assign4510_e3040_d_n17;
        var_t1_rv = 0.0;

        let assign4520_e3043: f64 = (var_t1 * var_t1);
        var_cnst1soi = assign4520_e3043;
        var_cnst1soi_dn0 = ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0));
        var_cnst1soi_dn2 = ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2));
        var_cnst1soi_dn6 = ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6));
        var_cnst1soi_dn7 = ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7));
        var_cnst1soi_dn10 = ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10));
        var_cnst1soi_dn11 = ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11));
        var_cnst1soi_dn12 = ((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12));
        var_cnst1soi_dn17 = ((var_t1_dn17 * var_t1) + (var_t1 * var_t1_dn17));
        var_cnst1soi_rv = 0.0;

        let assign4530_e3047: f64 = (1.034943e-10 / var_q_nsub);
        let assign4530_e3049: f64 = (assign4530_e3047 / var_beta);
        let assign4530_e3050: f64 = (2.0 * assign4530_e3049);
        let assign4530_e3051: f64 = (assign4530_e3050).sqrt();
        var_c_w_soi = assign4530_e3051;
        var_c_w_soi_dn0 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn0) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4530_e3051));
        var_c_w_soi_dn2 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn2) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4530_e3051));
        var_c_w_soi_dn6 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn6) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4530_e3051));
        var_c_w_soi_dn7 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn7) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4530_e3051));
        var_c_w_soi_dn10 = ((2.0 * ((((-((1.034943e-10 * var_q_nsub_dn10) / (var_q_nsub * var_q_nsub))) * var_beta) - (assign4530_e3047 * var_beta_dn10)) / (var_beta * var_beta))) / (2.0 * assign4530_e3051));
        var_c_w_soi_dn11 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn11) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4530_e3051));
        var_c_w_soi_dn12 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn12) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4530_e3051));
        var_c_w_soi_dn17 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn17) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4530_e3051));
        var_c_w_soi_rv = 0.0;

        let assign4540_e3054: f64 = (2.0 * 1.034943e-10);
        let assign4540_e3056: f64 = (assign4540_e3054 / 1.6021918e-19);
        let assign4540_e3058: f64 = (assign4540_e3056 / var_uc_nsubs);
        var_cnst_2esi_q_nsubs = assign4540_e3058;
        var_cnst_2esi_q_nsubs_dn0 = (-((assign4540_e3056 * var_uc_nsubs_dn0) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn2 = (-((assign4540_e3056 * var_uc_nsubs_dn2) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn6 = (-((assign4540_e3056 * var_uc_nsubs_dn6) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn7 = (-((assign4540_e3056 * var_uc_nsubs_dn7) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn10 = (-((assign4540_e3056 * var_uc_nsubs_dn10) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn11 = (-((assign4540_e3056 * var_uc_nsubs_dn11) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn12 = (-((assign4540_e3056 * var_uc_nsubs_dn12) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn17 = (-((assign4540_e3056 * var_uc_nsubs_dn17) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_rv = 0.0;

        let assign4550_e3061: f64 = (2.0 * 1.034943e-10);
        let assign4550_e3063: f64 = (assign4550_e3061 / 1.6021918e-19);
        let assign4550_e3065: f64 = (assign4550_e3063 * var_pb2);
        let assign4550_e3067: f64 = (assign4550_e3065 / var_uc_nsubs);
        let assign4550_e3068: f64 = (assign4550_e3067).sqrt();
        var_wdsoi_ini = assign4550_e3068;
        var_wdsoi_ini_dn0 = (((((assign4550_e3063 * var_pb2_dn0) * var_uc_nsubs) - (assign4550_e3065 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4550_e3068));
        var_wdsoi_ini_dn2 = (((((assign4550_e3063 * var_pb2_dn2) * var_uc_nsubs) - (assign4550_e3065 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4550_e3068));
        var_wdsoi_ini_dn6 = (((((assign4550_e3063 * var_pb2_dn6) * var_uc_nsubs) - (assign4550_e3065 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4550_e3068));
        var_wdsoi_ini_dn7 = (((((assign4550_e3063 * var_pb2_dn7) * var_uc_nsubs) - (assign4550_e3065 * var_uc_nsubs_dn7)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4550_e3068));
        var_wdsoi_ini_dn10 = (((((assign4550_e3063 * var_pb2_dn10) * var_uc_nsubs) - (assign4550_e3065 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4550_e3068));
        var_wdsoi_ini_dn11 = (((((assign4550_e3063 * var_pb2_dn11) * var_uc_nsubs) - (assign4550_e3065 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4550_e3068));
        var_wdsoi_ini_dn12 = (((((assign4550_e3063 * var_pb2_dn12) * var_uc_nsubs) - (assign4550_e3065 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4550_e3068));
        var_wdsoi_ini_dn17 = (((((assign4550_e3063 * var_pb2_dn17) * var_uc_nsubs) - (assign4550_e3065 * var_uc_nsubs_dn17)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign4550_e3068));
        var_wdsoi_ini_rv = 0.0;

        let assign4630_e3093: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard46 = assign4630_e3093;
        var_guard46_rv = 0.0;

        let (assign4640_e3097,) = {
    if (var_guard46 != 0.0) {
        (0.4,)
    } else {
        (var_vbs_bnd,)
    }
};
        var_vbs_bnd = assign4640_e3097;
        var_vbs_bnd_rv = 0.0;

        let (assign4650_e3101,) = {
    if (var_guard46 != 0.0) {
        (0.8,)
    } else {
        (var_vbs_max,)
    }
};
        var_vbs_max = assign4650_e3101;
        var_vbs_max_rv = 0.0;

        let (assign4660_e3106,) = {
    if (var_guard46 == 0.0) {
        (0.8,)
    } else {
        (var_vbs_bnd,)
    }
};
        var_vbs_bnd = assign4660_e3106;
        var_vbs_bnd_rv = 0.0;

        let (assign4670_e3111,) = {
    if (var_guard46 == 0.0) {
        (1.2,)
    } else {
        (var_vbs_max,)
    }
};
        var_vbs_max = assign4670_e3111;
        var_vbs_max_rv = 0.0;

        let assign4680_e3115: f64 = (var_vbs_max * 0.5);
        let assign4680_e3116: f64 = if var_vbs_bnd > assign4680_e3115 { 1.0 } else { 0.0 };
        var_guard47 = assign4680_e3116;
        var_guard47_rv = 0.0;

        let (assign4690_e3122,) = {
    if (var_guard47 != 0.0) {
        let assign4690_e3120: f64 = (0.5 * var_vbs_max);
        (assign4690_e3120,)
    } else {
        (var_vbs_bnd,)
    }
};
        var_vbs_bnd = assign4690_e3122;
        var_vbs_bnd_rv = 0.0;

        let assign4700_e3125: f64 = if var_vbs > var_vbs_bnd { 1.0 } else { 0.0 };
        var_guard48 = assign4700_e3125;
        var_guard48_rv = 0.0;

        let (assign4710_e3131, assign4710_e3131_d_n0, assign4710_e3131_d_n2, assign4710_e3131_d_n6, assign4710_e3131_d_n7, assign4710_e3131_d_n10, assign4710_e3131_d_n11, assign4710_e3131_d_n12, assign4710_e3131_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4710_e3129: f64 = (var_vbs - var_vbs_bnd);
        (assign4710_e3129, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign4710_e3131;
        var_t2_dn0 = assign4710_e3131_d_n0;
        var_t2_dn2 = assign4710_e3131_d_n2;
        var_t2_dn6 = assign4710_e3131_d_n6;
        var_t2_dn7 = assign4710_e3131_d_n7;
        var_t2_dn10 = assign4710_e3131_d_n10;
        var_t2_dn11 = assign4710_e3131_d_n11;
        var_t2_dn12 = assign4710_e3131_d_n12;
        var_t2_dn17 = assign4710_e3131_d_n17;
        var_t2_rv = 0.0;

        let (assign4720_e3137, assign4720_e3137_d_n0, assign4720_e3137_d_n2, assign4720_e3137_d_n6, assign4720_e3137_d_n7, assign4720_e3137_d_n10, assign4720_e3137_d_n11, assign4720_e3137_d_n12, assign4720_e3137_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4720_e3135: f64 = (var_vbs_max - var_vbs_bnd);
        (assign4720_e3135, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign4720_e3137;
        var_t3_dn0 = assign4720_e3137_d_n0;
        var_t3_dn2 = assign4720_e3137_d_n2;
        var_t3_dn6 = assign4720_e3137_d_n6;
        var_t3_dn7 = assign4720_e3137_d_n7;
        var_t3_dn10 = assign4720_e3137_d_n10;
        var_t3_dn11 = assign4720_e3137_d_n11;
        var_t3_dn12 = assign4720_e3137_d_n12;
        var_t3_dn17 = assign4720_e3137_d_n17;
        var_t3_rv = 0.0;

        let (assign4730_e3143, assign4730_e3143_d_n0, assign4730_e3143_d_n2, assign4730_e3143_d_n6, assign4730_e3143_d_n7, assign4730_e3143_d_n10, assign4730_e3143_d_n11, assign4730_e3143_d_n12, assign4730_e3143_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4730_e3141: f64 = (var_t2 * var_t2);
        (assign4730_e3141, ((var_t2_dn0 * var_t2) + (var_t2 * var_t2_dn0)), ((var_t2_dn2 * var_t2) + (var_t2 * var_t2_dn2)), ((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)), ((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7)), ((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10)), ((var_t2_dn11 * var_t2) + (var_t2 * var_t2_dn11)), ((var_t2_dn12 * var_t2) + (var_t2 * var_t2_dn12)), ((var_t2_dn17 * var_t2) + (var_t2 * var_t2_dn17)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn6, var_x2_dn7, var_x2_dn10, var_x2_dn11, var_x2_dn12, var_x2_dn17,)
    }
};
        var_x2 = assign4730_e3143;
        var_x2_dn0 = assign4730_e3143_d_n0;
        var_x2_dn2 = assign4730_e3143_d_n2;
        var_x2_dn6 = assign4730_e3143_d_n6;
        var_x2_dn7 = assign4730_e3143_d_n7;
        var_x2_dn10 = assign4730_e3143_d_n10;
        var_x2_dn11 = assign4730_e3143_d_n11;
        var_x2_dn12 = assign4730_e3143_d_n12;
        var_x2_dn17 = assign4730_e3143_d_n17;
        var_x2_rv = 0.0;

        let (assign4740_e3149, assign4740_e3149_d_n0, assign4740_e3149_d_n2, assign4740_e3149_d_n6, assign4740_e3149_d_n7, assign4740_e3149_d_n10, assign4740_e3149_d_n11, assign4740_e3149_d_n12, assign4740_e3149_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4740_e3147: f64 = (var_t3 * var_t3);
        (assign4740_e3147, ((var_t3_dn0 * var_t3) + (var_t3 * var_t3_dn0)), ((var_t3_dn2 * var_t3) + (var_t3 * var_t3_dn2)), ((var_t3_dn6 * var_t3) + (var_t3 * var_t3_dn6)), ((var_t3_dn7 * var_t3) + (var_t3 * var_t3_dn7)), ((var_t3_dn10 * var_t3) + (var_t3 * var_t3_dn10)), ((var_t3_dn11 * var_t3) + (var_t3 * var_t3_dn11)), ((var_t3_dn12 * var_t3) + (var_t3 * var_t3_dn12)), ((var_t3_dn17 * var_t3) + (var_t3 * var_t3_dn17)),)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn10, var_xmax2_dn11, var_xmax2_dn12, var_xmax2_dn17,)
    }
};
        var_xmax2 = assign4740_e3149;
        var_xmax2_dn0 = assign4740_e3149_d_n0;
        var_xmax2_dn2 = assign4740_e3149_d_n2;
        var_xmax2_dn6 = assign4740_e3149_d_n6;
        var_xmax2_dn7 = assign4740_e3149_d_n7;
        var_xmax2_dn10 = assign4740_e3149_d_n10;
        var_xmax2_dn11 = assign4740_e3149_d_n11;
        var_xmax2_dn12 = assign4740_e3149_d_n12;
        var_xmax2_dn17 = assign4740_e3149_d_n17;
        var_xmax2_rv = 0.0;

        let (assign4750_e3153, assign4750_e3153_d_n0, assign4750_e3153_d_n2, assign4750_e3153_d_n6, assign4750_e3153_d_n7, assign4750_e3153_d_n10, assign4750_e3153_d_n11, assign4750_e3153_d_n12, assign4750_e3153_d_n17,) = {
    if (var_guard48 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4750_e3153;
        var_xp_dn0 = assign4750_e3153_d_n0;
        var_xp_dn2 = assign4750_e3153_d_n2;
        var_xp_dn6 = assign4750_e3153_d_n6;
        var_xp_dn7 = assign4750_e3153_d_n7;
        var_xp_dn10 = assign4750_e3153_d_n10;
        var_xp_dn11 = assign4750_e3153_d_n11;
        var_xp_dn12 = assign4750_e3153_d_n12;
        var_xp_dn17 = assign4750_e3153_d_n17;
        var_xp_rv = 0.0;

        let (assign4760_e3157, assign4760_e3157_d_n0, assign4760_e3157_d_n2, assign4760_e3157_d_n6, assign4760_e3157_d_n7, assign4760_e3157_d_n10, assign4760_e3157_d_n11, assign4760_e3157_d_n12, assign4760_e3157_d_n17,) = {
    if (var_guard48 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4760_e3157;
        var_xmp_dn0 = assign4760_e3157_d_n0;
        var_xmp_dn2 = assign4760_e3157_d_n2;
        var_xmp_dn6 = assign4760_e3157_d_n6;
        var_xmp_dn7 = assign4760_e3157_d_n7;
        var_xmp_dn10 = assign4760_e3157_d_n10;
        var_xmp_dn11 = assign4760_e3157_d_n11;
        var_xmp_dn12 = assign4760_e3157_d_n12;
        var_xmp_dn17 = assign4760_e3157_d_n17;
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
        *var_guard40_slot = var_guard40;
        *var_guard40_rv_slot = var_guard40_rv;
        *var_guard41_slot = var_guard41;
        *var_guard41_rv_slot = var_guard41_rv;
        *var_guard46_slot = var_guard46;
        *var_guard46_rv_slot = var_guard46_rv;
        *var_guard47_slot = var_guard47;
        *var_guard47_rv_slot = var_guard47_rv;
        *var_guard48_slot = var_guard48;
        *var_guard48_rv_slot = var_guard48_rv;
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
        var_guard48: f64,
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
        var_guard49_slot: &mut f64,
        var_guard49_rv_slot: &mut f64,
        var_guard50_slot: &mut f64,
        var_guard50_rv_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard51_rv_slot: &mut f64,
        var_guard52_slot: &mut f64,
        var_guard52_rv_slot: &mut f64,
        var_guard53_slot: &mut f64,
        var_guard53_rv_slot: &mut f64,
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
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_guard49_rv: f64 = *var_guard49_rv_slot;
        let mut var_guard50: f64 = *var_guard50_slot;
        let mut var_guard50_rv: f64 = *var_guard50_rv_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard51_rv: f64 = *var_guard51_rv_slot;
        let mut var_guard52: f64 = *var_guard52_slot;
        let mut var_guard52_rv: f64 = *var_guard52_rv_slot;
        let mut var_guard53: f64 = *var_guard53_slot;
        let mut var_guard53_rv: f64 = *var_guard53_rv_slot;
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

        let (assign4770_e3161,) = {
    if (var_guard48 != 0.0) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign4770_e3161;
        var_m0_rv = 0.0;

        let (assign4780_e3165,) = {
    if (var_guard48 != 0.0) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign4780_e3165;
        var_mm_rv = 0.0;

        let (assign4790_e3169, assign4790_e3169_d_n0, assign4790_e3169_d_n2, assign4790_e3169_d_n6, assign4790_e3169_d_n7, assign4790_e3169_d_n10, assign4790_e3169_d_n11, assign4790_e3169_d_n12, assign4790_e3169_d_n17,) = {
    if (var_guard48 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    }
};
        var_arg = assign4790_e3169;
        var_arg_dn0 = assign4790_e3169_d_n0;
        var_arg_dn2 = assign4790_e3169_d_n2;
        var_arg_dn6 = assign4790_e3169_d_n6;
        var_arg_dn7 = assign4790_e3169_d_n7;
        var_arg_dn10 = assign4790_e3169_d_n10;
        var_arg_dn11 = assign4790_e3169_d_n11;
        var_arg_dn12 = assign4790_e3169_d_n12;
        var_arg_dn17 = assign4790_e3169_d_n17;
        var_arg_rv = 0.0;

        let (assign4800_e3173, assign4800_e3173_d_n0, assign4800_e3173_d_n2, assign4800_e3173_d_n6, assign4800_e3173_d_n7, assign4800_e3173_d_n10, assign4800_e3173_d_n11, assign4800_e3173_d_n12, assign4800_e3173_d_n17,) = {
    if (var_guard48 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign4800_e3173;
        var_dnm_dn0 = assign4800_e3173_d_n0;
        var_dnm_dn2 = assign4800_e3173_d_n2;
        var_dnm_dn6 = assign4800_e3173_d_n6;
        var_dnm_dn7 = assign4800_e3173_d_n7;
        var_dnm_dn10 = assign4800_e3173_d_n10;
        var_dnm_dn11 = assign4800_e3173_d_n11;
        var_dnm_dn12 = assign4800_e3173_d_n12;
        var_dnm_dn17 = assign4800_e3173_d_n17;
        var_dnm_rv = 0.0;

        let (assign4810_e3179, assign4810_e3179_d_n0, assign4810_e3179_d_n2, assign4810_e3179_d_n6, assign4810_e3179_d_n7, assign4810_e3179_d_n10, assign4810_e3179_d_n11, assign4810_e3179_d_n12, assign4810_e3179_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4810_e3177: f64 = (var_xp * var_x2);
        (assign4810_e3177, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4810_e3179;
        var_xp_dn0 = assign4810_e3179_d_n0;
        var_xp_dn2 = assign4810_e3179_d_n2;
        var_xp_dn6 = assign4810_e3179_d_n6;
        var_xp_dn7 = assign4810_e3179_d_n7;
        var_xp_dn10 = assign4810_e3179_d_n10;
        var_xp_dn11 = assign4810_e3179_d_n11;
        var_xp_dn12 = assign4810_e3179_d_n12;
        var_xp_dn17 = assign4810_e3179_d_n17;
        var_xp_rv = 0.0;

        let (assign4820_e3185, assign4820_e3185_d_n0, assign4820_e3185_d_n2, assign4820_e3185_d_n6, assign4820_e3185_d_n7, assign4820_e3185_d_n10, assign4820_e3185_d_n11, assign4820_e3185_d_n12, assign4820_e3185_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4820_e3183: f64 = (var_xmp * var_xmax2);
        (assign4820_e3183, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4820_e3185;
        var_xmp_dn0 = assign4820_e3185_d_n0;
        var_xmp_dn2 = assign4820_e3185_d_n2;
        var_xmp_dn6 = assign4820_e3185_d_n6;
        var_xmp_dn7 = assign4820_e3185_d_n7;
        var_xmp_dn10 = assign4820_e3185_d_n10;
        var_xmp_dn11 = assign4820_e3185_d_n11;
        var_xmp_dn12 = assign4820_e3185_d_n12;
        var_xmp_dn17 = assign4820_e3185_d_n17;
        var_xmp_rv = 0.0;

        let (assign4830_e3191, assign4830_e3191_d_n0, assign4830_e3191_d_n2, assign4830_e3191_d_n6, assign4830_e3191_d_n7, assign4830_e3191_d_n10, assign4830_e3191_d_n11, assign4830_e3191_d_n12, assign4830_e3191_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4830_e3189: f64 = (var_xp * var_x2);
        (assign4830_e3189, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4830_e3191;
        var_xp_dn0 = assign4830_e3191_d_n0;
        var_xp_dn2 = assign4830_e3191_d_n2;
        var_xp_dn6 = assign4830_e3191_d_n6;
        var_xp_dn7 = assign4830_e3191_d_n7;
        var_xp_dn10 = assign4830_e3191_d_n10;
        var_xp_dn11 = assign4830_e3191_d_n11;
        var_xp_dn12 = assign4830_e3191_d_n12;
        var_xp_dn17 = assign4830_e3191_d_n17;
        var_xp_rv = 0.0;

        let (assign4840_e3197, assign4840_e3197_d_n0, assign4840_e3197_d_n2, assign4840_e3197_d_n6, assign4840_e3197_d_n7, assign4840_e3197_d_n10, assign4840_e3197_d_n11, assign4840_e3197_d_n12, assign4840_e3197_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4840_e3195: f64 = (var_xmp * var_xmax2);
        (assign4840_e3195, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4840_e3197;
        var_xmp_dn0 = assign4840_e3197_d_n0;
        var_xmp_dn2 = assign4840_e3197_d_n2;
        var_xmp_dn6 = assign4840_e3197_d_n6;
        var_xmp_dn7 = assign4840_e3197_d_n7;
        var_xmp_dn10 = assign4840_e3197_d_n10;
        var_xmp_dn11 = assign4840_e3197_d_n11;
        var_xmp_dn12 = assign4840_e3197_d_n12;
        var_xmp_dn17 = assign4840_e3197_d_n17;
        var_xmp_rv = 0.0;

        let (assign4850_e3203, assign4850_e3203_d_n0, assign4850_e3203_d_n2, assign4850_e3203_d_n6, assign4850_e3203_d_n7, assign4850_e3203_d_n10, assign4850_e3203_d_n11, assign4850_e3203_d_n12, assign4850_e3203_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4850_e3201: f64 = (var_xp * var_x2);
        (assign4850_e3201, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4850_e3203;
        var_xp_dn0 = assign4850_e3203_d_n0;
        var_xp_dn2 = assign4850_e3203_d_n2;
        var_xp_dn6 = assign4850_e3203_d_n6;
        var_xp_dn7 = assign4850_e3203_d_n7;
        var_xp_dn10 = assign4850_e3203_d_n10;
        var_xp_dn11 = assign4850_e3203_d_n11;
        var_xp_dn12 = assign4850_e3203_d_n12;
        var_xp_dn17 = assign4850_e3203_d_n17;
        var_xp_rv = 0.0;

        let (assign4860_e3209, assign4860_e3209_d_n0, assign4860_e3209_d_n2, assign4860_e3209_d_n6, assign4860_e3209_d_n7, assign4860_e3209_d_n10, assign4860_e3209_d_n11, assign4860_e3209_d_n12, assign4860_e3209_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4860_e3207: f64 = (var_xmp * var_xmax2);
        (assign4860_e3207, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4860_e3209;
        var_xmp_dn0 = assign4860_e3209_d_n0;
        var_xmp_dn2 = assign4860_e3209_d_n2;
        var_xmp_dn6 = assign4860_e3209_d_n6;
        var_xmp_dn7 = assign4860_e3209_d_n7;
        var_xmp_dn10 = assign4860_e3209_d_n10;
        var_xmp_dn11 = assign4860_e3209_d_n11;
        var_xmp_dn12 = assign4860_e3209_d_n12;
        var_xmp_dn17 = assign4860_e3209_d_n17;
        var_xmp_rv = 0.0;

        let (assign4870_e3215, assign4870_e3215_d_n0, assign4870_e3215_d_n2, assign4870_e3215_d_n6, assign4870_e3215_d_n7, assign4870_e3215_d_n10, assign4870_e3215_d_n11, assign4870_e3215_d_n12, assign4870_e3215_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4870_e3213: f64 = (var_xp * var_x2);
        (assign4870_e3213, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4870_e3215;
        var_xp_dn0 = assign4870_e3215_d_n0;
        var_xp_dn2 = assign4870_e3215_d_n2;
        var_xp_dn6 = assign4870_e3215_d_n6;
        var_xp_dn7 = assign4870_e3215_d_n7;
        var_xp_dn10 = assign4870_e3215_d_n10;
        var_xp_dn11 = assign4870_e3215_d_n11;
        var_xp_dn12 = assign4870_e3215_d_n12;
        var_xp_dn17 = assign4870_e3215_d_n17;
        var_xp_rv = 0.0;

        let (assign4880_e3221, assign4880_e3221_d_n0, assign4880_e3221_d_n2, assign4880_e3221_d_n6, assign4880_e3221_d_n7, assign4880_e3221_d_n10, assign4880_e3221_d_n11, assign4880_e3221_d_n12, assign4880_e3221_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4880_e3219: f64 = (var_xmp * var_xmax2);
        (assign4880_e3219, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4880_e3221;
        var_xmp_dn0 = assign4880_e3221_d_n0;
        var_xmp_dn2 = assign4880_e3221_d_n2;
        var_xmp_dn6 = assign4880_e3221_d_n6;
        var_xmp_dn7 = assign4880_e3221_d_n7;
        var_xmp_dn10 = assign4880_e3221_d_n10;
        var_xmp_dn11 = assign4880_e3221_d_n11;
        var_xmp_dn12 = assign4880_e3221_d_n12;
        var_xmp_dn17 = assign4880_e3221_d_n17;
        var_xmp_rv = 0.0;

        let (assign4890_e3227, assign4890_e3227_d_n0, assign4890_e3227_d_n2, assign4890_e3227_d_n6, assign4890_e3227_d_n7, assign4890_e3227_d_n10, assign4890_e3227_d_n11, assign4890_e3227_d_n12, assign4890_e3227_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign4890_e3225: f64 = (var_xp + var_xmp);
        (assign4890_e3225, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn11 + var_xmp_dn11), (var_xp_dn12 + var_xmp_dn12), (var_xp_dn17 + var_xmp_dn17),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    }
};
        var_arg = assign4890_e3227;
        var_arg_dn0 = assign4890_e3227_d_n0;
        var_arg_dn2 = assign4890_e3227_d_n2;
        var_arg_dn6 = assign4890_e3227_d_n6;
        var_arg_dn7 = assign4890_e3227_d_n7;
        var_arg_dn10 = assign4890_e3227_d_n10;
        var_arg_dn11 = assign4890_e3227_d_n11;
        var_arg_dn12 = assign4890_e3227_d_n12;
        var_arg_dn17 = assign4890_e3227_d_n17;
        var_arg_rv = 0.0;

        let (assign4900_e3231, assign4900_e3231_d_n0, assign4900_e3231_d_n2, assign4900_e3231_d_n6, assign4900_e3231_d_n7, assign4900_e3231_d_n10, assign4900_e3231_d_n11, assign4900_e3231_d_n12, assign4900_e3231_d_n17,) = {
    if (var_guard48 != 0.0) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign4900_e3231;
        var_dnm_dn0 = assign4900_e3231_d_n0;
        var_dnm_dn2 = assign4900_e3231_d_n2;
        var_dnm_dn6 = assign4900_e3231_d_n6;
        var_dnm_dn7 = assign4900_e3231_d_n7;
        var_dnm_dn10 = assign4900_e3231_d_n10;
        var_dnm_dn11 = assign4900_e3231_d_n11;
        var_dnm_dn12 = assign4900_e3231_d_n12;
        var_dnm_dn17 = assign4900_e3231_d_n17;
        var_dnm_rv = 0.0;

        let assign4910_e3246: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard49 = assign4910_e3246;
        var_guard49_rv = 0.0;

        let assign4920_e3249: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard50 = assign4920_e3249;
        var_guard50_rv = 0.0;

        let (assign4930_e3257,) = {
    if (((var_guard48 != 0.0) && (var_guard49 != 0.0)) && (var_guard50 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign4930_e3257;
        var_mm_rv = 0.0;

        let assign4940_e3260: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard51 = assign4940_e3260;
        var_guard51_rv = 0.0;

        let (assign4950_e3271,) = {
    if ((((var_guard48 != 0.0) && (var_guard49 != 0.0)) && (var_guard50 == 0.0)) && (var_guard51 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign4950_e3271;
        var_mm_rv = 0.0;

        let assign4960_e3274: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard52 = assign4960_e3274;
        var_guard52_rv = 0.0;

        let (assign4970_e3288,) = {
    if (((((var_guard48 != 0.0) && (var_guard49 != 0.0)) && (var_guard50 == 0.0)) && (var_guard51 == 0.0)) && (var_guard52 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign4970_e3288;
        var_mm_rv = 0.0;

        let assign4980_e3291: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard53 = assign4980_e3291;
        var_guard53_rv = 0.0;

        let (assign4990_e3308,) = {
    if ((((((var_guard48 != 0.0) && (var_guard49 != 0.0)) && (var_guard50 == 0.0)) && (var_guard51 == 0.0)) && (var_guard52 == 0.0)) && (var_guard53 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign4990_e3308;
        var_mm_rv = 0.0;

        let (assign5000_e3314,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign5000_e3314;
        var_m0_rv = 0.0;

        let mut assign5010_loop_guard: usize = 0;
        while {
            let assign5010_cond_e3321: f64 = if (((var_guard48 != 0.0) && (var_guard49 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign5010_cond_e3321 != 0.0
        } {
            assign5010_loop_guard += 1;
            assert!(assign5010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign5010_body0_e3328, assign5010_body0_e3328_d_n0, assign5010_body0_e3328_d_n2, assign5010_body0_e3328_d_n6, assign5010_body0_e3328_d_n7, assign5010_body0_e3328_d_n10, assign5010_body0_e3328_d_n11, assign5010_body0_e3328_d_n12, assign5010_body0_e3328_d_n17,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        let assign5010_body0_e3326: f64 = (var_dnm).sqrt();
        (assign5010_body0_e3326, (var_dnm_dn0 / (2.0 * assign5010_body0_e3326)), (var_dnm_dn2 / (2.0 * assign5010_body0_e3326)), (var_dnm_dn6 / (2.0 * assign5010_body0_e3326)), (var_dnm_dn7 / (2.0 * assign5010_body0_e3326)), (var_dnm_dn10 / (2.0 * assign5010_body0_e3326)), (var_dnm_dn11 / (2.0 * assign5010_body0_e3326)), (var_dnm_dn12 / (2.0 * assign5010_body0_e3326)), (var_dnm_dn17 / (2.0 * assign5010_body0_e3326)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
            var_dnm = assign5010_body0_e3328;
            var_dnm_dn0 = assign5010_body0_e3328_d_n0;
            var_dnm_dn2 = assign5010_body0_e3328_d_n2;
            var_dnm_dn6 = assign5010_body0_e3328_d_n6;
            var_dnm_dn7 = assign5010_body0_e3328_d_n7;
            var_dnm_dn10 = assign5010_body0_e3328_d_n10;
            var_dnm_dn11 = assign5010_body0_e3328_d_n11;
            var_dnm_dn12 = assign5010_body0_e3328_d_n12;
            var_dnm_dn17 = assign5010_body0_e3328_d_n17;
            var_dnm_rv = 0.0;
            let (assign5010_body1_e3336,) = {
    if ((var_guard48 != 0.0) && (var_guard49 != 0.0)) {
        let assign5010_body1_e3334: f64 = (var_m0 + 1.0);
        (assign5010_body1_e3334,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign5010_body1_e3336;
            var_m0_rv = 0.0;
        }

        let (assign5020_e3349, assign5020_e3349_d_n0, assign5020_e3349_d_n2, assign5020_e3349_d_n6, assign5020_e3349_d_n7, assign5020_e3349_d_n10, assign5020_e3349_d_n11, assign5020_e3349_d_n12, assign5020_e3349_d_n17,) = {
    if ((var_guard48 != 0.0) && (var_guard49 == 0.0)) {
        let assign5020_e3345: f64 = (2.0 * 4.0);
        let assign5020_e3346: f64 = (1.0 / assign5020_e3345);
        let assign5020_e3347: f64 = (var_dnm).powf(assign5020_e3346);
        (assign5020_e3347, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((var_dnm).powf(assign5020_e3346 - 1.0) * var_dnm_dn0)) } } else { (assign5020_e3347 * (assign5020_e3346 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((var_dnm).powf(assign5020_e3346 - 1.0) * var_dnm_dn2)) } } else { (assign5020_e3347 * (assign5020_e3346 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((var_dnm).powf(assign5020_e3346 - 1.0) * var_dnm_dn6)) } } else { (assign5020_e3347 * (assign5020_e3346 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((var_dnm).powf(assign5020_e3346 - 1.0) * var_dnm_dn7)) } } else { (assign5020_e3347 * (assign5020_e3346 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((var_dnm).powf(assign5020_e3346 - 1.0) * var_dnm_dn10)) } } else { (assign5020_e3347 * (assign5020_e3346 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((var_dnm).powf(assign5020_e3346 - 1.0) * var_dnm_dn11)) } } else { (assign5020_e3347 * (assign5020_e3346 * (var_dnm_dn11 / var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((var_dnm).powf(assign5020_e3346 - 1.0) * var_dnm_dn12)) } } else { (assign5020_e3347 * (assign5020_e3346 * (var_dnm_dn12 / var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((var_dnm).powf(assign5020_e3346 - 1.0) * var_dnm_dn17)) } } else { (assign5020_e3347 * (assign5020_e3346 * (var_dnm_dn17 / var_dnm))) },)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign5020_e3349;
        var_dnm_dn0 = assign5020_e3349_d_n0;
        var_dnm_dn2 = assign5020_e3349_d_n2;
        var_dnm_dn6 = assign5020_e3349_d_n6;
        var_dnm_dn7 = assign5020_e3349_d_n7;
        var_dnm_dn10 = assign5020_e3349_d_n10;
        var_dnm_dn11 = assign5020_e3349_d_n11;
        var_dnm_dn12 = assign5020_e3349_d_n12;
        var_dnm_dn17 = assign5020_e3349_d_n17;
        var_dnm_rv = 0.0;

        let (assign5030_e3355, assign5030_e3355_d_n0, assign5030_e3355_d_n2, assign5030_e3355_d_n6, assign5030_e3355_d_n7, assign5030_e3355_d_n10, assign5030_e3355_d_n11, assign5030_e3355_d_n12, assign5030_e3355_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign5030_e3353: f64 = (1.0 / var_dnm);
        (assign5030_e3353, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn11 / (var_dnm * var_dnm))), (-(var_dnm_dn12 / (var_dnm * var_dnm))), (-(var_dnm_dn17 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign5030_e3355;
        var_dnm_dn0 = assign5030_e3355_d_n0;
        var_dnm_dn2 = assign5030_e3355_d_n2;
        var_dnm_dn6 = assign5030_e3355_d_n6;
        var_dnm_dn7 = assign5030_e3355_d_n7;
        var_dnm_dn10 = assign5030_e3355_d_n10;
        var_dnm_dn11 = assign5030_e3355_d_n11;
        var_dnm_dn12 = assign5030_e3355_d_n12;
        var_dnm_dn17 = assign5030_e3355_d_n17;
        var_dnm_rv = 0.0;

        let (assign5040_e3363, assign5040_e3363_d_n0, assign5040_e3363_d_n2, assign5040_e3363_d_n6, assign5040_e3363_d_n7, assign5040_e3363_d_n10, assign5040_e3363_d_n11, assign5040_e3363_d_n12, assign5040_e3363_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign5040_e3359: f64 = (var_t2 * var_t3);
        let assign5040_e3361: f64 = (assign5040_e3359 * var_dnm);
        (assign5040_e3361, ((((var_t2_dn0 * var_t3) + (var_t2 * var_t3_dn0)) * var_dnm) + (assign5040_e3359 * var_dnm_dn0)), ((((var_t2_dn2 * var_t3) + (var_t2 * var_t3_dn2)) * var_dnm) + (assign5040_e3359 * var_dnm_dn2)), ((((var_t2_dn6 * var_t3) + (var_t2 * var_t3_dn6)) * var_dnm) + (assign5040_e3359 * var_dnm_dn6)), ((((var_t2_dn7 * var_t3) + (var_t2 * var_t3_dn7)) * var_dnm) + (assign5040_e3359 * var_dnm_dn7)), ((((var_t2_dn10 * var_t3) + (var_t2 * var_t3_dn10)) * var_dnm) + (assign5040_e3359 * var_dnm_dn10)), ((((var_t2_dn11 * var_t3) + (var_t2 * var_t3_dn11)) * var_dnm) + (assign5040_e3359 * var_dnm_dn11)), ((((var_t2_dn12 * var_t3) + (var_t2 * var_t3_dn12)) * var_dnm) + (assign5040_e3359 * var_dnm_dn12)), ((((var_t2_dn17 * var_t3) + (var_t2 * var_t3_dn17)) * var_dnm) + (assign5040_e3359 * var_dnm_dn17)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign5040_e3363;
        var_t4_dn0 = assign5040_e3363_d_n0;
        var_t4_dn2 = assign5040_e3363_d_n2;
        var_t4_dn6 = assign5040_e3363_d_n6;
        var_t4_dn7 = assign5040_e3363_d_n7;
        var_t4_dn10 = assign5040_e3363_d_n10;
        var_t4_dn11 = assign5040_e3363_d_n11;
        var_t4_dn12 = assign5040_e3363_d_n12;
        var_t4_dn17 = assign5040_e3363_d_n17;
        var_t4_rv = 0.0;

        let (assign5050_e3373, assign5050_e3373_d_n0, assign5050_e3373_d_n2, assign5050_e3373_d_n6, assign5050_e3373_d_n7, assign5050_e3373_d_n10, assign5050_e3373_d_n11, assign5050_e3373_d_n12, assign5050_e3373_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign5050_e3367: f64 = (var_t3 * var_xmp);
        let assign5050_e3369: f64 = (assign5050_e3367 * var_dnm);
        let assign5050_e3371: f64 = (assign5050_e3369 / var_arg);
        (assign5050_e3371, (((((((var_t3_dn0 * var_xmp) + (var_t3 * var_xmp_dn0)) * var_dnm) + (assign5050_e3367 * var_dnm_dn0)) * var_arg) - (assign5050_e3369 * var_arg_dn0)) / (var_arg * var_arg)), (((((((var_t3_dn2 * var_xmp) + (var_t3 * var_xmp_dn2)) * var_dnm) + (assign5050_e3367 * var_dnm_dn2)) * var_arg) - (assign5050_e3369 * var_arg_dn2)) / (var_arg * var_arg)), (((((((var_t3_dn6 * var_xmp) + (var_t3 * var_xmp_dn6)) * var_dnm) + (assign5050_e3367 * var_dnm_dn6)) * var_arg) - (assign5050_e3369 * var_arg_dn6)) / (var_arg * var_arg)), (((((((var_t3_dn7 * var_xmp) + (var_t3 * var_xmp_dn7)) * var_dnm) + (assign5050_e3367 * var_dnm_dn7)) * var_arg) - (assign5050_e3369 * var_arg_dn7)) / (var_arg * var_arg)), (((((((var_t3_dn10 * var_xmp) + (var_t3 * var_xmp_dn10)) * var_dnm) + (assign5050_e3367 * var_dnm_dn10)) * var_arg) - (assign5050_e3369 * var_arg_dn10)) / (var_arg * var_arg)), (((((((var_t3_dn11 * var_xmp) + (var_t3 * var_xmp_dn11)) * var_dnm) + (assign5050_e3367 * var_dnm_dn11)) * var_arg) - (assign5050_e3369 * var_arg_dn11)) / (var_arg * var_arg)), (((((((var_t3_dn12 * var_xmp) + (var_t3 * var_xmp_dn12)) * var_dnm) + (assign5050_e3367 * var_dnm_dn12)) * var_arg) - (assign5050_e3369 * var_arg_dn12)) / (var_arg * var_arg)), (((((((var_t3_dn17 * var_xmp) + (var_t3 * var_xmp_dn17)) * var_dnm) + (assign5050_e3367 * var_dnm_dn17)) * var_arg) - (assign5050_e3369 * var_arg_dn17)) / (var_arg * var_arg)),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn6, var_t8_dn7, var_t8_dn10, var_t8_dn11, var_t8_dn12, var_t8_dn17,)
    }
};
        var_t8 = assign5050_e3373;
        var_t8_dn0 = assign5050_e3373_d_n0;
        var_t8_dn2 = assign5050_e3373_d_n2;
        var_t8_dn6 = assign5050_e3373_d_n6;
        var_t8_dn7 = assign5050_e3373_d_n7;
        var_t8_dn10 = assign5050_e3373_d_n10;
        var_t8_dn11 = assign5050_e3373_d_n11;
        var_t8_dn12 = assign5050_e3373_d_n12;
        var_t8_dn17 = assign5050_e3373_d_n17;
        var_t8_rv = 0.0;

        let (assign5060_e3379, assign5060_e3379_d_n0, assign5060_e3379_d_n2, assign5060_e3379_d_n6, assign5060_e3379_d_n7, assign5060_e3379_d_n10, assign5060_e3379_d_n11, assign5060_e3379_d_n12, assign5060_e3379_d_n17,) = {
    if (var_guard48 != 0.0) {
        let assign5060_e3377: f64 = (var_vbs_bnd + var_t4);
        (assign5060_e3377, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    } else {
        (var_vbsc, var_vbsc_dn0, var_vbsc_dn2, var_vbsc_dn6, var_vbsc_dn7, var_vbsc_dn10, var_vbsc_dn11, var_vbsc_dn12, var_vbsc_dn17,)
    }
};
        var_vbsc = assign5060_e3379;
        var_vbsc_dn0 = assign5060_e3379_d_n0;
        var_vbsc_dn2 = assign5060_e3379_d_n2;
        var_vbsc_dn6 = assign5060_e3379_d_n6;
        var_vbsc_dn7 = assign5060_e3379_d_n7;
        var_vbsc_dn10 = assign5060_e3379_d_n10;
        var_vbsc_dn11 = assign5060_e3379_d_n11;
        var_vbsc_dn12 = assign5060_e3379_d_n12;
        var_vbsc_dn17 = assign5060_e3379_d_n17;
        var_vbsc_rv = 0.0;

        let (assign5070_e3383, assign5070_e3383_d_n0, assign5070_e3383_d_n2, assign5070_e3383_d_n6, assign5070_e3383_d_n7, assign5070_e3383_d_n10, assign5070_e3383_d_n11, assign5070_e3383_d_n12, assign5070_e3383_d_n17,) = {
    if (var_guard48 != 0.0) {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn6, var_t8_dn7, var_t8_dn10, var_t8_dn11, var_t8_dn12, var_t8_dn17,)
    } else {
        (var_vbsc_dvbse, var_vbsc_dvbse_dn0, var_vbsc_dvbse_dn2, var_vbsc_dvbse_dn6, var_vbsc_dvbse_dn7, var_vbsc_dvbse_dn10, var_vbsc_dvbse_dn11, var_vbsc_dvbse_dn12, var_vbsc_dvbse_dn17,)
    }
};
        var_vbsc_dvbse = assign5070_e3383;
        var_vbsc_dvbse_dn0 = assign5070_e3383_d_n0;
        var_vbsc_dvbse_dn2 = assign5070_e3383_d_n2;
        var_vbsc_dvbse_dn6 = assign5070_e3383_d_n6;
        var_vbsc_dvbse_dn7 = assign5070_e3383_d_n7;
        var_vbsc_dvbse_dn10 = assign5070_e3383_d_n10;
        var_vbsc_dvbse_dn11 = assign5070_e3383_d_n11;
        var_vbsc_dvbse_dn12 = assign5070_e3383_d_n12;
        var_vbsc_dvbse_dn17 = assign5070_e3383_d_n17;
        var_vbsc_dvbse_rv = 0.0;

        let (assign5080_e3388, assign5080_e3388_d_n0, assign5080_e3388_d_n2, assign5080_e3388_d_n6, assign5080_e3388_d_n7, assign5080_e3388_d_n10, assign5080_e3388_d_n11, assign5080_e3388_d_n12, assign5080_e3388_d_n17,) = {
    if (var_guard48 == 0.0) {
        (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    } else {
        (var_vbsc, var_vbsc_dn0, var_vbsc_dn2, var_vbsc_dn6, var_vbsc_dn7, var_vbsc_dn10, var_vbsc_dn11, var_vbsc_dn12, var_vbsc_dn17,)
    }
};
        var_vbsc = assign5080_e3388;
        var_vbsc_dn0 = assign5080_e3388_d_n0;
        var_vbsc_dn2 = assign5080_e3388_d_n2;
        var_vbsc_dn6 = assign5080_e3388_d_n6;
        var_vbsc_dn7 = assign5080_e3388_d_n7;
        var_vbsc_dn10 = assign5080_e3388_d_n10;
        var_vbsc_dn11 = assign5080_e3388_d_n11;
        var_vbsc_dn12 = assign5080_e3388_d_n12;
        var_vbsc_dn17 = assign5080_e3388_d_n17;
        var_vbsc_rv = 0.0;

        let (assign5090_e3393, assign5090_e3393_d_n0, assign5090_e3393_d_n2, assign5090_e3393_d_n6, assign5090_e3393_d_n7, assign5090_e3393_d_n10, assign5090_e3393_d_n11, assign5090_e3393_d_n12, assign5090_e3393_d_n17,) = {
    if (var_guard48 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vbsc_dvbse, var_vbsc_dvbse_dn0, var_vbsc_dvbse_dn2, var_vbsc_dvbse_dn6, var_vbsc_dvbse_dn7, var_vbsc_dvbse_dn10, var_vbsc_dvbse_dn11, var_vbsc_dvbse_dn12, var_vbsc_dvbse_dn17,)
    }
};
        var_vbsc_dvbse = assign5090_e3393;
        var_vbsc_dvbse_dn0 = assign5090_e3393_d_n0;
        var_vbsc_dvbse_dn2 = assign5090_e3393_d_n2;
        var_vbsc_dvbse_dn6 = assign5090_e3393_d_n6;
        var_vbsc_dvbse_dn7 = assign5090_e3393_d_n7;
        var_vbsc_dvbse_dn10 = assign5090_e3393_d_n10;
        var_vbsc_dvbse_dn11 = assign5090_e3393_d_n11;
        var_vbsc_dvbse_dn12 = assign5090_e3393_d_n12;
        var_vbsc_dvbse_dn17 = assign5090_e3393_d_n17;
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
        *var_guard49_slot = var_guard49;
        *var_guard49_rv_slot = var_guard49_rv;
        *var_guard50_slot = var_guard50;
        *var_guard50_rv_slot = var_guard50_rv;
        *var_guard51_slot = var_guard51;
        *var_guard51_rv_slot = var_guard51_rv;
        *var_guard52_slot = var_guard52;
        *var_guard52_rv_slot = var_guard52_rv;
        *var_guard53_slot = var_guard53;
        *var_guard53_rv_slot = var_guard53_rv;
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
        var_guard55_slot: &mut f64,
        var_guard55_rv_slot: &mut f64,
        var_guard56_slot: &mut f64,
        var_guard56_rv_slot: &mut f64,
        var_guard61_slot: &mut f64,
        var_guard61_rv_slot: &mut f64,
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
        var_t1__blk54_slot: &mut f64,
        var_t1__blk54_dn0_slot: &mut f64,
        var_t1__blk54_dn10_slot: &mut f64,
        var_t1__blk54_dn11_slot: &mut f64,
        var_t1__blk54_dn12_slot: &mut f64,
        var_t1__blk54_dn17_slot: &mut f64,
        var_t1__blk54_dn2_slot: &mut f64,
        var_t1__blk54_dn6_slot: &mut f64,
        var_t1__blk54_dn7_slot: &mut f64,
        var_t1__blk54_rv_slot: &mut f64,
        var_t1__blk57_slot: &mut f64,
        var_t1__blk57_dn0_slot: &mut f64,
        var_t1__blk57_dn10_slot: &mut f64,
        var_t1__blk57_dn11_slot: &mut f64,
        var_t1__blk57_dn12_slot: &mut f64,
        var_t1__blk57_dn17_slot: &mut f64,
        var_t1__blk57_dn2_slot: &mut f64,
        var_t1__blk57_dn6_slot: &mut f64,
        var_t1__blk57_dn7_slot: &mut f64,
        var_t1__blk57_rv_slot: &mut f64,
        var_t2__blk58_slot: &mut f64,
        var_t2__blk58_dn11_slot: &mut f64,
        var_t2__blk58_dn6_slot: &mut f64,
        var_t2__blk58_dn7_slot: &mut f64,
        var_t2__blk58_rv_slot: &mut f64,
        var_t3__blk59_slot: &mut f64,
        var_t3__blk59_dn0_slot: &mut f64,
        var_t3__blk59_dn10_slot: &mut f64,
        var_t3__blk59_dn11_slot: &mut f64,
        var_t3__blk59_dn12_slot: &mut f64,
        var_t3__blk59_dn17_slot: &mut f64,
        var_t3__blk59_dn2_slot: &mut f64,
        var_t3__blk59_dn6_slot: &mut f64,
        var_t3__blk59_dn7_slot: &mut f64,
        var_t3__blk59_rv_slot: &mut f64,
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
        var_tx__blk60_slot: &mut f64,
        var_tx__blk60_dn0_slot: &mut f64,
        var_tx__blk60_dn10_slot: &mut f64,
        var_tx__blk60_dn11_slot: &mut f64,
        var_tx__blk60_dn12_slot: &mut f64,
        var_tx__blk60_dn17_slot: &mut f64,
        var_tx__blk60_dn2_slot: &mut f64,
        var_tx__blk60_dn6_slot: &mut f64,
        var_tx__blk60_dn7_slot: &mut f64,
        var_tx__blk60_rv_slot: &mut f64,
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
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_guard55_rv: f64 = *var_guard55_rv_slot;
        let mut var_guard56: f64 = *var_guard56_slot;
        let mut var_guard56_rv: f64 = *var_guard56_rv_slot;
        let mut var_guard61: f64 = *var_guard61_slot;
        let mut var_guard61_rv: f64 = *var_guard61_rv_slot;
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
        let mut var_t1__blk54: f64 = *var_t1__blk54_slot;
        let mut var_t1__blk54_dn0: f64 = *var_t1__blk54_dn0_slot;
        let mut var_t1__blk54_dn10: f64 = *var_t1__blk54_dn10_slot;
        let mut var_t1__blk54_dn11: f64 = *var_t1__blk54_dn11_slot;
        let mut var_t1__blk54_dn12: f64 = *var_t1__blk54_dn12_slot;
        let mut var_t1__blk54_dn17: f64 = *var_t1__blk54_dn17_slot;
        let mut var_t1__blk54_dn2: f64 = *var_t1__blk54_dn2_slot;
        let mut var_t1__blk54_dn6: f64 = *var_t1__blk54_dn6_slot;
        let mut var_t1__blk54_dn7: f64 = *var_t1__blk54_dn7_slot;
        let mut var_t1__blk54_rv: f64 = *var_t1__blk54_rv_slot;
        let mut var_t1__blk57: f64 = *var_t1__blk57_slot;
        let mut var_t1__blk57_dn0: f64 = *var_t1__blk57_dn0_slot;
        let mut var_t1__blk57_dn10: f64 = *var_t1__blk57_dn10_slot;
        let mut var_t1__blk57_dn11: f64 = *var_t1__blk57_dn11_slot;
        let mut var_t1__blk57_dn12: f64 = *var_t1__blk57_dn12_slot;
        let mut var_t1__blk57_dn17: f64 = *var_t1__blk57_dn17_slot;
        let mut var_t1__blk57_dn2: f64 = *var_t1__blk57_dn2_slot;
        let mut var_t1__blk57_dn6: f64 = *var_t1__blk57_dn6_slot;
        let mut var_t1__blk57_dn7: f64 = *var_t1__blk57_dn7_slot;
        let mut var_t1__blk57_rv: f64 = *var_t1__blk57_rv_slot;
        let mut var_t2__blk58: f64 = *var_t2__blk58_slot;
        let mut var_t2__blk58_dn11: f64 = *var_t2__blk58_dn11_slot;
        let mut var_t2__blk58_dn6: f64 = *var_t2__blk58_dn6_slot;
        let mut var_t2__blk58_dn7: f64 = *var_t2__blk58_dn7_slot;
        let mut var_t2__blk58_rv: f64 = *var_t2__blk58_rv_slot;
        let mut var_t3__blk59: f64 = *var_t3__blk59_slot;
        let mut var_t3__blk59_dn0: f64 = *var_t3__blk59_dn0_slot;
        let mut var_t3__blk59_dn10: f64 = *var_t3__blk59_dn10_slot;
        let mut var_t3__blk59_dn11: f64 = *var_t3__blk59_dn11_slot;
        let mut var_t3__blk59_dn12: f64 = *var_t3__blk59_dn12_slot;
        let mut var_t3__blk59_dn17: f64 = *var_t3__blk59_dn17_slot;
        let mut var_t3__blk59_dn2: f64 = *var_t3__blk59_dn2_slot;
        let mut var_t3__blk59_dn6: f64 = *var_t3__blk59_dn6_slot;
        let mut var_t3__blk59_dn7: f64 = *var_t3__blk59_dn7_slot;
        let mut var_t3__blk59_rv: f64 = *var_t3__blk59_rv_slot;
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
        let mut var_tx__blk60: f64 = *var_tx__blk60_slot;
        let mut var_tx__blk60_dn0: f64 = *var_tx__blk60_dn0_slot;
        let mut var_tx__blk60_dn10: f64 = *var_tx__blk60_dn10_slot;
        let mut var_tx__blk60_dn11: f64 = *var_tx__blk60_dn11_slot;
        let mut var_tx__blk60_dn12: f64 = *var_tx__blk60_dn12_slot;
        let mut var_tx__blk60_dn17: f64 = *var_tx__blk60_dn17_slot;
        let mut var_tx__blk60_dn2: f64 = *var_tx__blk60_dn2_slot;
        let mut var_tx__blk60_dn6: f64 = *var_tx__blk60_dn6_slot;
        let mut var_tx__blk60_dn7: f64 = *var_tx__blk60_dn7_slot;
        let mut var_tx__blk60_rv: f64 = *var_tx__blk60_rv_slot;
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

        let (assign5100_e3399, assign5100_e3399_d_n0, assign5100_e3399_d_n2, assign5100_e3399_d_n6, assign5100_e3399_d_n7, assign5100_e3399_d_n10, assign5100_e3399_d_n11, assign5100_e3399_d_n12, assign5100_e3399_d_n17,) = {
    if (var_vds > 20.0) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vds, var_vds_dn0, var_vds_dn2, var_vds_dn6, var_vds_dn7, var_vds_dn10, var_vds_dn11, var_vds_dn12, var_vds_dn17,)
    }
};
        var_vdsc = assign5100_e3399;
        var_vdsc_dn0 = assign5100_e3399_d_n0;
        var_vdsc_dn2 = assign5100_e3399_d_n2;
        var_vdsc_dn6 = assign5100_e3399_d_n6;
        var_vdsc_dn7 = assign5100_e3399_d_n7;
        var_vdsc_dn10 = assign5100_e3399_d_n10;
        var_vdsc_dn11 = assign5100_e3399_d_n11;
        var_vdsc_dn12 = assign5100_e3399_d_n12;
        var_vdsc_dn17 = assign5100_e3399_d_n17;
        var_vdsc_rv = 0.0;

        let (assign5110_e3405, assign5110_e3405_d_n6, assign5110_e3405_d_n7, assign5110_e3405_d_n11,) = {
    if (var_vgs > 20.0) {
        (20.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vgs, var_vgs_dn6, var_vgs_dn7, var_vgs_dn11,)
    }
};
        var_vgsc = assign5110_e3405;
        var_vgsc_dn6 = assign5110_e3405_d_n6;
        var_vgsc_dn7 = assign5110_e3405_d_n7;
        var_vgsc_dn11 = assign5110_e3405_d_n11;
        var_vgsc_rv = 0.0;

        let assign5120_e3408: f64 = (-20.0);
        let (assign5120_e3413, assign5120_e3413_d_n6, assign5120_e3413_d_n7, assign5120_e3413_d_n11,) = {
    if (var_vgs < assign5120_e3408) {
        let assign5120_e3411: f64 = (-20.0);
        (assign5120_e3411, 0.0, 0.0, 0.0,)
    } else {
        (var_vgsc, var_vgsc_dn6, var_vgsc_dn7, var_vgsc_dn11,)
    }
};
        var_vgsc = assign5120_e3413;
        var_vgsc_dn6 = assign5120_e3413_d_n6;
        var_vgsc_dn7 = assign5120_e3413_d_n7;
        var_vgsc_dn11 = assign5120_e3413_d_n11;
        var_vgsc_rv = 0.0;

        let assign5130_e3416: f64 = (-20.0);
        let (assign5130_e3421, assign5130_e3421_d_n0, assign5130_e3421_d_n2, assign5130_e3421_d_n6, assign5130_e3421_d_n7, assign5130_e3421_d_n10, assign5130_e3421_d_n11, assign5130_e3421_d_n12, assign5130_e3421_d_n17,) = {
    if (var_vbsc < assign5130_e3416) {
        let assign5130_e3419: f64 = (-20.0);
        (assign5130_e3419, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vbsc, var_vbsc_dn0, var_vbsc_dn2, var_vbsc_dn6, var_vbsc_dn7, var_vbsc_dn10, var_vbsc_dn11, var_vbsc_dn12, var_vbsc_dn17,)
    }
};
        var_vbsc = assign5130_e3421;
        var_vbsc_dn0 = assign5130_e3421_d_n0;
        var_vbsc_dn2 = assign5130_e3421_d_n2;
        var_vbsc_dn6 = assign5130_e3421_d_n6;
        var_vbsc_dn7 = assign5130_e3421_d_n7;
        var_vbsc_dn10 = assign5130_e3421_d_n10;
        var_vbsc_dn11 = assign5130_e3421_d_n11;
        var_vbsc_dn12 = assign5130_e3421_d_n12;
        var_vbsc_dn17 = assign5130_e3421_d_n17;
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

        let assign5300_e3440: f64 = (var_vbsc_dvbse * var_vds);
        let assign5300_e3442: f64 = (assign5300_e3440 / 2.0);
        var_t1__blk54 = assign5300_e3442;
        var_t1__blk54_dn0 = (((var_vbsc_dvbse_dn0 * var_vds) + (var_vbsc_dvbse * var_vds_dn0)) / 2.0);
        var_t1__blk54_dn2 = (((var_vbsc_dvbse_dn2 * var_vds) + (var_vbsc_dvbse * var_vds_dn2)) / 2.0);
        var_t1__blk54_dn6 = (((var_vbsc_dvbse_dn6 * var_vds) + (var_vbsc_dvbse * var_vds_dn6)) / 2.0);
        var_t1__blk54_dn7 = (((var_vbsc_dvbse_dn7 * var_vds) + (var_vbsc_dvbse * var_vds_dn7)) / 2.0);
        var_t1__blk54_dn10 = (((var_vbsc_dvbse_dn10 * var_vds) + (var_vbsc_dvbse * var_vds_dn10)) / 2.0);
        var_t1__blk54_dn11 = (((var_vbsc_dvbse_dn11 * var_vds) + (var_vbsc_dvbse * var_vds_dn11)) / 2.0);
        var_t1__blk54_dn12 = (((var_vbsc_dvbse_dn12 * var_vds) + (var_vbsc_dvbse * var_vds_dn12)) / 2.0);
        var_t1__blk54_dn17 = (((var_vbsc_dvbse_dn17 * var_vds) + (var_vbsc_dvbse * var_vds_dn17)) / 2.0);
        var_t1__blk54_rv = 0.0;

        let assign5310_e3445: f64 = (2.0 * var_t1__blk54);
        let assign5310_e3447: f64 = (assign5310_e3445 / p.p226);
        var_tmf1 = assign5310_e3447;
        var_tmf1_dn0 = ((2.0 * var_t1__blk54_dn0) / p.p226);
        var_tmf1_dn2 = ((2.0 * var_t1__blk54_dn2) / p.p226);
        var_tmf1_dn6 = ((2.0 * var_t1__blk54_dn6) / p.p226);
        var_tmf1_dn7 = ((2.0 * var_t1__blk54_dn7) / p.p226);
        var_tmf1_dn10 = ((2.0 * var_t1__blk54_dn10) / p.p226);
        var_tmf1_dn11 = ((2.0 * var_t1__blk54_dn11) / p.p226);
        var_tmf1_dn12 = ((2.0 * var_t1__blk54_dn12) / p.p226);
        var_tmf1_dn17 = ((2.0 * var_t1__blk54_dn17) / p.p226);
        var_tmf1_rv = 0.0;

        let assign5320_e3452: f64 = (1.0 / 2.0);
        let assign5320_e3456: f64 = (1.0 / 6.0);
        let assign5320_e3460: f64 = (1.0 / 24.0);
        let assign5320_e3464: f64 = (1.0 / 120.0);
        let assign5320_e3468: f64 = (1.0 / 720.0);
        let assign5320_e3472: f64 = (1.0 / 5040.0);
        let assign5320_e3473: f64 = (var_tmf1 * assign5320_e3472);
        let assign5320_e3474: f64 = (assign5320_e3468 + assign5320_e3473);
        let assign5320_e3475: f64 = (var_tmf1 * assign5320_e3474);
        let assign5320_e3476: f64 = (assign5320_e3464 + assign5320_e3475);
        let assign5320_e3477: f64 = (var_tmf1 * assign5320_e3476);
        let assign5320_e3478: f64 = (assign5320_e3460 + assign5320_e3477);
        let assign5320_e3479: f64 = (var_tmf1 * assign5320_e3478);
        let assign5320_e3480: f64 = (assign5320_e3456 + assign5320_e3479);
        let assign5320_e3481: f64 = (var_tmf1 * assign5320_e3480);
        let assign5320_e3482: f64 = (assign5320_e3452 + assign5320_e3481);
        let assign5320_e3483: f64 = (var_tmf1 * assign5320_e3482);
        let assign5320_e3484: f64 = (1.0 + assign5320_e3483);
        var_tmf2 = assign5320_e3484;
        var_tmf2_dn0 = ((var_tmf1_dn0 * assign5320_e3482) + (var_tmf1 * ((var_tmf1_dn0 * assign5320_e3480) + (var_tmf1 * ((var_tmf1_dn0 * assign5320_e3478) + (var_tmf1 * ((var_tmf1_dn0 * assign5320_e3476) + (var_tmf1 * ((var_tmf1_dn0 * assign5320_e3474) + (var_tmf1 * (var_tmf1_dn0 * assign5320_e3472)))))))))));
        var_tmf2_dn2 = ((var_tmf1_dn2 * assign5320_e3482) + (var_tmf1 * ((var_tmf1_dn2 * assign5320_e3480) + (var_tmf1 * ((var_tmf1_dn2 * assign5320_e3478) + (var_tmf1 * ((var_tmf1_dn2 * assign5320_e3476) + (var_tmf1 * ((var_tmf1_dn2 * assign5320_e3474) + (var_tmf1 * (var_tmf1_dn2 * assign5320_e3472)))))))))));
        var_tmf2_dn6 = ((var_tmf1_dn6 * assign5320_e3482) + (var_tmf1 * ((var_tmf1_dn6 * assign5320_e3480) + (var_tmf1 * ((var_tmf1_dn6 * assign5320_e3478) + (var_tmf1 * ((var_tmf1_dn6 * assign5320_e3476) + (var_tmf1 * ((var_tmf1_dn6 * assign5320_e3474) + (var_tmf1 * (var_tmf1_dn6 * assign5320_e3472)))))))))));
        var_tmf2_dn7 = ((var_tmf1_dn7 * assign5320_e3482) + (var_tmf1 * ((var_tmf1_dn7 * assign5320_e3480) + (var_tmf1 * ((var_tmf1_dn7 * assign5320_e3478) + (var_tmf1 * ((var_tmf1_dn7 * assign5320_e3476) + (var_tmf1 * ((var_tmf1_dn7 * assign5320_e3474) + (var_tmf1 * (var_tmf1_dn7 * assign5320_e3472)))))))))));
        var_tmf2_dn10 = ((var_tmf1_dn10 * assign5320_e3482) + (var_tmf1 * ((var_tmf1_dn10 * assign5320_e3480) + (var_tmf1 * ((var_tmf1_dn10 * assign5320_e3478) + (var_tmf1 * ((var_tmf1_dn10 * assign5320_e3476) + (var_tmf1 * ((var_tmf1_dn10 * assign5320_e3474) + (var_tmf1 * (var_tmf1_dn10 * assign5320_e3472)))))))))));
        var_tmf2_dn11 = ((var_tmf1_dn11 * assign5320_e3482) + (var_tmf1 * ((var_tmf1_dn11 * assign5320_e3480) + (var_tmf1 * ((var_tmf1_dn11 * assign5320_e3478) + (var_tmf1 * ((var_tmf1_dn11 * assign5320_e3476) + (var_tmf1 * ((var_tmf1_dn11 * assign5320_e3474) + (var_tmf1 * (var_tmf1_dn11 * assign5320_e3472)))))))))));
        var_tmf2_dn12 = ((var_tmf1_dn12 * assign5320_e3482) + (var_tmf1 * ((var_tmf1_dn12 * assign5320_e3480) + (var_tmf1 * ((var_tmf1_dn12 * assign5320_e3478) + (var_tmf1 * ((var_tmf1_dn12 * assign5320_e3476) + (var_tmf1 * ((var_tmf1_dn12 * assign5320_e3474) + (var_tmf1 * (var_tmf1_dn12 * assign5320_e3472)))))))))));
        var_tmf2_dn17 = ((var_tmf1_dn17 * assign5320_e3482) + (var_tmf1 * ((var_tmf1_dn17 * assign5320_e3480) + (var_tmf1 * ((var_tmf1_dn17 * assign5320_e3478) + (var_tmf1 * ((var_tmf1_dn17 * assign5320_e3476) + (var_tmf1 * ((var_tmf1_dn17 * assign5320_e3474) + (var_tmf1 * (var_tmf1_dn17 * assign5320_e3472)))))))))));
        var_tmf2_rv = 0.0;

        let assign5330_e3487: f64 = (p.p226 / var_tmf2);
        var_vzadd = assign5330_e3487;
        var_vzadd_dn0 = (-((p.p226 * var_tmf2_dn0) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn2 = (-((p.p226 * var_tmf2_dn2) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn6 = (-((p.p226 * var_tmf2_dn6) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn7 = (-((p.p226 * var_tmf2_dn7) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn10 = (-((p.p226 * var_tmf2_dn10) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn11 = (-((p.p226 * var_tmf2_dn11) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn12 = (-((p.p226 * var_tmf2_dn12) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn17 = (-((p.p226 * var_tmf2_dn17) / (var_tmf2 * var_tmf2)));
        var_vzadd_rv = 0.0;

        let assign5340_e3490: f64 = if var_vzadd < 5e-12 { 1.0 } else { 0.0 };
        var_guard55 = assign5340_e3490;
        var_guard55_rv = 0.0;

        let (assign5350_e3494, assign5350_e3494_d_n0, assign5350_e3494_d_n2, assign5350_e3494_d_n6, assign5350_e3494_d_n7, assign5350_e3494_d_n10, assign5350_e3494_d_n11, assign5350_e3494_d_n12, assign5350_e3494_d_n17,) = {
    if (var_guard55 != 0.0) {
        (5e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vzadd, var_vzadd_dn0, var_vzadd_dn2, var_vzadd_dn6, var_vzadd_dn7, var_vzadd_dn10, var_vzadd_dn11, var_vzadd_dn12, var_vzadd_dn17,)
    }
};
        var_vzadd = assign5350_e3494;
        var_vzadd_dn0 = assign5350_e3494_d_n0;
        var_vzadd_dn2 = assign5350_e3494_d_n2;
        var_vzadd_dn6 = assign5350_e3494_d_n6;
        var_vzadd_dn7 = assign5350_e3494_d_n7;
        var_vzadd_dn10 = assign5350_e3494_d_n10;
        var_vzadd_dn11 = assign5350_e3494_d_n11;
        var_vzadd_dn12 = assign5350_e3494_d_n12;
        var_vzadd_dn17 = assign5350_e3494_d_n17;
        var_vzadd_rv = 0.0;

        let assign5360_e3497: f64 = (var_vbs + var_vzadd);
        var_vbsz = assign5360_e3497;
        var_vbsz_dn0 = (var_vbs_dn0 + var_vzadd_dn0);
        var_vbsz_dn2 = (var_vbs_dn2 + var_vzadd_dn2);
        var_vbsz_dn6 = (var_vbs_dn6 + var_vzadd_dn6);
        var_vbsz_dn7 = (var_vbs_dn7 + var_vzadd_dn7);
        var_vbsz_dn10 = (var_vbs_dn10 + var_vzadd_dn10);
        var_vbsz_dn11 = (var_vbs_dn11 + var_vzadd_dn11);
        var_vbsz_dn12 = (var_vbs_dn12 + var_vzadd_dn12);
        var_vbsz_dn17 = (var_vbs_dn17 + var_vzadd_dn17);
        var_vbsz_rv = 0.0;

        let assign5370_e3501: f64 = (2.0 * var_vzadd);
        let assign5370_e3502: f64 = (var_vds + assign5370_e3501);
        var_vdsz = assign5370_e3502;
        var_vdsz_dn0 = (var_vds_dn0 + (2.0 * var_vzadd_dn0));
        var_vdsz_dn2 = (var_vds_dn2 + (2.0 * var_vzadd_dn2));
        var_vdsz_dn6 = (var_vds_dn6 + (2.0 * var_vzadd_dn6));
        var_vdsz_dn7 = (var_vds_dn7 + (2.0 * var_vzadd_dn7));
        var_vdsz_dn10 = (var_vds_dn10 + (2.0 * var_vzadd_dn10));
        var_vdsz_dn11 = (var_vds_dn11 + (2.0 * var_vzadd_dn11));
        var_vdsz_dn12 = (var_vds_dn12 + (2.0 * var_vzadd_dn12));
        var_vdsz_dn17 = (var_vds_dn17 + (2.0 * var_vzadd_dn17));
        var_vdsz_rv = 0.0;

        let assign5380_e3505: f64 = (var_vgs + var_vzadd);
        var_vgsz = assign5380_e3505;
        var_vgsz_dn0 = var_vzadd_dn0;
        var_vgsz_dn2 = var_vzadd_dn2;
        var_vgsz_dn6 = (var_vgs_dn6 + var_vzadd_dn6);
        var_vgsz_dn7 = (var_vgs_dn7 + var_vzadd_dn7);
        var_vgsz_dn10 = var_vzadd_dn10;
        var_vgsz_dn11 = (var_vgs_dn11 + var_vzadd_dn11);
        var_vgsz_dn12 = var_vzadd_dn12;
        var_vgsz_dn17 = var_vzadd_dn17;
        var_vgsz_rv = 0.0;

        let assign5390_e3508: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard56 = assign5390_e3508;
        var_guard56_rv = 0.0;

        let (assign5400_e3512, assign5400_e3512_d_n0, assign5400_e3512_d_n2, assign5400_e3512_d_n6, assign5400_e3512_d_n7, assign5400_e3512_d_n10, assign5400_e3512_d_n11, assign5400_e3512_d_n12, assign5400_e3512_d_n17,) = {
    if (var_guard56 != 0.0) {
        (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    } else {
        (var_vbsp, var_vbsp_dn0, var_vbsp_dn2, var_vbsp_dn6, var_vbsp_dn7, var_vbsp_dn10, var_vbsp_dn11, var_vbsp_dn12, var_vbsp_dn17,)
    }
};
        var_vbsp = assign5400_e3512;
        var_vbsp_dn0 = assign5400_e3512_d_n0;
        var_vbsp_dn2 = assign5400_e3512_d_n2;
        var_vbsp_dn6 = assign5400_e3512_d_n6;
        var_vbsp_dn7 = assign5400_e3512_d_n7;
        var_vbsp_dn10 = assign5400_e3512_d_n10;
        var_vbsp_dn11 = assign5400_e3512_d_n11;
        var_vbsp_dn12 = assign5400_e3512_d_n12;
        var_vbsp_dn17 = assign5400_e3512_d_n17;
        var_vbsp_rv = 0.0;

        let (assign5410_e3516, assign5410_e3516_d_n0, assign5410_e3516_d_n2, assign5410_e3516_d_n6, assign5410_e3516_d_n7, assign5410_e3516_d_n10, assign5410_e3516_d_n11, assign5410_e3516_d_n12, assign5410_e3516_d_n17,) = {
    if (var_guard56 != 0.0) {
        (var_vbsz, var_vbsz_dn0, var_vbsz_dn2, var_vbsz_dn6, var_vbsz_dn7, var_vbsz_dn10, var_vbsz_dn11, var_vbsz_dn12, var_vbsz_dn17,)
    } else {
        (var_vbspz, var_vbspz_dn0, var_vbspz_dn2, var_vbspz_dn6, var_vbspz_dn7, var_vbspz_dn10, var_vbspz_dn11, var_vbspz_dn12, var_vbspz_dn17,)
    }
};
        var_vbspz = assign5410_e3516;
        var_vbspz_dn0 = assign5410_e3516_d_n0;
        var_vbspz_dn2 = assign5410_e3516_d_n2;
        var_vbspz_dn6 = assign5410_e3516_d_n6;
        var_vbspz_dn7 = assign5410_e3516_d_n7;
        var_vbspz_dn10 = assign5410_e3516_d_n10;
        var_vbspz_dn11 = assign5410_e3516_d_n11;
        var_vbspz_dn12 = assign5410_e3516_d_n12;
        var_vbspz_dn17 = assign5410_e3516_d_n17;
        var_vbspz_rv = 0.0;

        let (assign5420_e3526, assign5420_e3526_d_n0, assign5420_e3526_d_n2, assign5420_e3526_d_n6, assign5420_e3526_d_n7, assign5420_e3526_d_n10, assign5420_e3526_d_n11, assign5420_e3526_d_n12, assign5420_e3526_d_n17,) = {
    if (var_guard56 == 0.0) {
        let (assign5420_e3524, assign5420_e3524_d_n0, assign5420_e3524_d_n2, assign5420_e3524_d_n6, assign5420_e3524_d_n7, assign5420_e3524_d_n10, assign5420_e3524_d_n11, assign5420_e3524_d_n12, assign5420_e3524_d_n17,) = {
            if (var_subversion < 3.0) {
                (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign5420_e3524, assign5420_e3524_d_n0, assign5420_e3524_d_n2, assign5420_e3524_d_n6, assign5420_e3524_d_n7, assign5420_e3524_d_n10, assign5420_e3524_d_n11, assign5420_e3524_d_n12, assign5420_e3524_d_n17,)
    } else {
        (var_vbsp, var_vbsp_dn0, var_vbsp_dn2, var_vbsp_dn6, var_vbsp_dn7, var_vbsp_dn10, var_vbsp_dn11, var_vbsp_dn12, var_vbsp_dn17,)
    }
};
        var_vbsp = assign5420_e3526;
        var_vbsp_dn0 = assign5420_e3526_d_n0;
        var_vbsp_dn2 = assign5420_e3526_d_n2;
        var_vbsp_dn6 = assign5420_e3526_d_n6;
        var_vbsp_dn7 = assign5420_e3526_d_n7;
        var_vbsp_dn10 = assign5420_e3526_d_n10;
        var_vbsp_dn11 = assign5420_e3526_d_n11;
        var_vbsp_dn12 = assign5420_e3526_d_n12;
        var_vbsp_dn17 = assign5420_e3526_d_n17;
        var_vbsp_rv = 0.0;

        let (assign5430_e3536, assign5430_e3536_d_n0, assign5430_e3536_d_n2, assign5430_e3536_d_n6, assign5430_e3536_d_n7, assign5430_e3536_d_n10, assign5430_e3536_d_n11, assign5430_e3536_d_n12, assign5430_e3536_d_n17,) = {
    if (var_guard56 == 0.0) {
        let (assign5430_e3534, assign5430_e3534_d_n0, assign5430_e3534_d_n2, assign5430_e3534_d_n6, assign5430_e3534_d_n7, assign5430_e3534_d_n10, assign5430_e3534_d_n11, assign5430_e3534_d_n12, assign5430_e3534_d_n17,) = {
            if (var_subversion < 3.0) {
                (var_vbsz, var_vbsz_dn0, var_vbsz_dn2, var_vbsz_dn6, var_vbsz_dn7, var_vbsz_dn10, var_vbsz_dn11, var_vbsz_dn12, var_vbsz_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign5430_e3534, assign5430_e3534_d_n0, assign5430_e3534_d_n2, assign5430_e3534_d_n6, assign5430_e3534_d_n7, assign5430_e3534_d_n10, assign5430_e3534_d_n11, assign5430_e3534_d_n12, assign5430_e3534_d_n17,)
    } else {
        (var_vbspz, var_vbspz_dn0, var_vbspz_dn2, var_vbspz_dn6, var_vbspz_dn7, var_vbspz_dn10, var_vbspz_dn11, var_vbspz_dn12, var_vbspz_dn17,)
    }
};
        var_vbspz = assign5430_e3536;
        var_vbspz_dn0 = assign5430_e3536_d_n0;
        var_vbspz_dn2 = assign5430_e3536_d_n2;
        var_vbspz_dn6 = assign5430_e3536_d_n6;
        var_vbspz_dn7 = assign5430_e3536_d_n7;
        var_vbspz_dn10 = assign5430_e3536_d_n10;
        var_vbspz_dn11 = assign5430_e3536_d_n11;
        var_vbspz_dn12 = assign5430_e3536_d_n12;
        var_vbspz_dn17 = assign5430_e3536_d_n17;
        var_vbspz_rv = 0.0;

        let assign5440_e3539: f64 = (2.0 * var_q_nsub);
        let assign5440_e3541: f64 = (assign5440_e3539 * 1.034943e-10);
        let assign5440_e3543: f64 = (assign5440_e3541 * var_c_fox0_inv);
        let assign5440_e3545: f64 = (assign5440_e3543 * var_c_fox0_inv);
        var_t1__blk57 = assign5440_e3545;
        var_t1__blk57_dn0 = ((((2.0 * var_q_nsub_dn0) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk57_dn2 = ((((2.0 * var_q_nsub_dn2) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk57_dn6 = ((((2.0 * var_q_nsub_dn6) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk57_dn7 = ((((2.0 * var_q_nsub_dn7) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk57_dn10 = ((((2.0 * var_q_nsub_dn10) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk57_dn11 = ((((2.0 * var_q_nsub_dn11) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk57_dn12 = ((((2.0 * var_q_nsub_dn12) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk57_dn17 = ((((2.0 * var_q_nsub_dn17) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk57_rv = 0.0;

        let assign5450_e3548: f64 = (var_vgs - var_vfb);
        var_t2__blk58 = assign5450_e3548;
        var_t2__blk58_dn6 = var_vgs_dn6;
        var_t2__blk58_dn7 = var_vgs_dn7;
        var_t2__blk58_dn11 = var_vgs_dn11;
        var_t2__blk58_rv = 0.0;

        let assign5460_e3552: f64 = (2.0 / var_t1__blk57);
        let assign5460_e3555: f64 = (var_t2__blk58 - var_beta_inv);
        let assign5460_e3557: f64 = (assign5460_e3555 - var_vbsp);
        let assign5460_e3558: f64 = (assign5460_e3552 * assign5460_e3557);
        let assign5460_e3559: f64 = (1.0 + assign5460_e3558);
        var_t3__blk59 = assign5460_e3559;
        var_t3__blk59_dn0 = (((-((2.0 * var_t1__blk57_dn0) / (var_t1__blk57 * var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (-var_vbsp_dn0)));
        var_t3__blk59_dn2 = (((-((2.0 * var_t1__blk57_dn2) / (var_t1__blk57 * var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (-var_vbsp_dn2)));
        var_t3__blk59_dn6 = (((-((2.0 * var_t1__blk57_dn6) / (var_t1__blk57 * var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (var_t2__blk58_dn6 - var_vbsp_dn6)));
        var_t3__blk59_dn7 = (((-((2.0 * var_t1__blk57_dn7) / (var_t1__blk57 * var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (var_t2__blk58_dn7 - var_vbsp_dn7)));
        var_t3__blk59_dn10 = (((-((2.0 * var_t1__blk57_dn10) / (var_t1__blk57 * var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * ((-var_beta_inv_dn10) - var_vbsp_dn10)));
        var_t3__blk59_dn11 = (((-((2.0 * var_t1__blk57_dn11) / (var_t1__blk57 * var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (var_t2__blk58_dn11 - var_vbsp_dn11)));
        var_t3__blk59_dn12 = (((-((2.0 * var_t1__blk57_dn12) / (var_t1__blk57 * var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (-var_vbsp_dn12)));
        var_t3__blk59_dn17 = (((-((2.0 * var_t1__blk57_dn17) / (var_t1__blk57 * var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (-var_vbsp_dn17)));
        var_t3__blk59_rv = 0.0;

        let assign5470_e3562: f64 = (var_t3__blk59 * var_t3__blk59);
        let assign5470_e3565: f64 = (4.0 * 0.001);
        let assign5470_e3567: f64 = (assign5470_e3565 * 0.001);
        let assign5470_e3568: f64 = (assign5470_e3562 + assign5470_e3567);
        let assign5470_e3569: f64 = (assign5470_e3568).sqrt();
        var_tmf1 = assign5470_e3569;
        var_tmf1_dn0 = (((var_t3__blk59_dn0 * var_t3__blk59) + (var_t3__blk59 * var_t3__blk59_dn0)) / (2.0 * assign5470_e3569));
        var_tmf1_dn2 = (((var_t3__blk59_dn2 * var_t3__blk59) + (var_t3__blk59 * var_t3__blk59_dn2)) / (2.0 * assign5470_e3569));
        var_tmf1_dn6 = (((var_t3__blk59_dn6 * var_t3__blk59) + (var_t3__blk59 * var_t3__blk59_dn6)) / (2.0 * assign5470_e3569));
        var_tmf1_dn7 = (((var_t3__blk59_dn7 * var_t3__blk59) + (var_t3__blk59 * var_t3__blk59_dn7)) / (2.0 * assign5470_e3569));
        var_tmf1_dn10 = (((var_t3__blk59_dn10 * var_t3__blk59) + (var_t3__blk59 * var_t3__blk59_dn10)) / (2.0 * assign5470_e3569));
        var_tmf1_dn11 = (((var_t3__blk59_dn11 * var_t3__blk59) + (var_t3__blk59 * var_t3__blk59_dn11)) / (2.0 * assign5470_e3569));
        var_tmf1_dn12 = (((var_t3__blk59_dn12 * var_t3__blk59) + (var_t3__blk59 * var_t3__blk59_dn12)) / (2.0 * assign5470_e3569));
        var_tmf1_dn17 = (((var_t3__blk59_dn17 * var_t3__blk59) + (var_t3__blk59 * var_t3__blk59_dn17)) / (2.0 * assign5470_e3569));
        var_tmf1_rv = 0.0;

        let assign5480_e3573: f64 = (var_t3__blk59 + var_tmf1);
        let assign5480_e3574: f64 = (0.5 * assign5480_e3573);
        let assign5480_e3577: f64 = (1e-10 * 0.001);
        let assign5480_e3578: f64 = (assign5480_e3574 + assign5480_e3577);
        var_t4 = assign5480_e3578;
        var_t4_dn0 = (0.5 * (var_t3__blk59_dn0 + var_tmf1_dn0));
        var_t4_dn2 = (0.5 * (var_t3__blk59_dn2 + var_tmf1_dn2));
        var_t4_dn6 = (0.5 * (var_t3__blk59_dn6 + var_tmf1_dn6));
        var_t4_dn7 = (0.5 * (var_t3__blk59_dn7 + var_tmf1_dn7));
        var_t4_dn10 = (0.5 * (var_t3__blk59_dn10 + var_tmf1_dn10));
        var_t4_dn11 = (0.5 * (var_t3__blk59_dn11 + var_tmf1_dn11));
        var_t4_dn12 = (0.5 * (var_t3__blk59_dn12 + var_tmf1_dn12));
        var_t4_dn17 = (0.5 * (var_t3__blk59_dn17 + var_tmf1_dn17));
        var_t4_rv = 0.0;

        let assign5490_e3581: f64 = if var_t4 < 0.0 { 1.0 } else { 0.0 };
        var_guard61 = assign5490_e3581;
        var_guard61_rv = 0.0;

        let (assign5500_e3585, assign5500_e3585_d_n0, assign5500_e3585_d_n2, assign5500_e3585_d_n6, assign5500_e3585_d_n7, assign5500_e3585_d_n10, assign5500_e3585_d_n11, assign5500_e3585_d_n12, assign5500_e3585_d_n17,) = {
    if (var_guard61 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign5500_e3585;
        var_t4_dn0 = assign5500_e3585_d_n0;
        var_t4_dn2 = assign5500_e3585_d_n2;
        var_t4_dn6 = assign5500_e3585_d_n6;
        var_t4_dn7 = assign5500_e3585_d_n7;
        var_t4_dn10 = assign5500_e3585_d_n10;
        var_t4_dn11 = assign5500_e3585_d_n11;
        var_t4_dn12 = assign5500_e3585_d_n12;
        var_t4_dn17 = assign5500_e3585_d_n17;
        var_t4_rv = 0.0;

        let assign5510_e3588: f64 = (var_t4 + 1e-50);
        let assign5510_e3589: f64 = (assign5510_e3588).sqrt();
        var_tx__blk60 = assign5510_e3589;
        var_tx__blk60_dn0 = (var_t4_dn0 / (2.0 * assign5510_e3589));
        var_tx__blk60_dn2 = (var_t4_dn2 / (2.0 * assign5510_e3589));
        var_tx__blk60_dn6 = (var_t4_dn6 / (2.0 * assign5510_e3589));
        var_tx__blk60_dn7 = (var_t4_dn7 / (2.0 * assign5510_e3589));
        var_tx__blk60_dn10 = (var_t4_dn10 / (2.0 * assign5510_e3589));
        var_tx__blk60_dn11 = (var_t4_dn11 / (2.0 * assign5510_e3589));
        var_tx__blk60_dn12 = (var_t4_dn12 / (2.0 * assign5510_e3589));
        var_tx__blk60_dn17 = (var_t4_dn17 / (2.0 * assign5510_e3589));
        var_tx__blk60_rv = 0.0;

        let assign5520_e3594: f64 = (1.0 - var_tx__blk60);
        let assign5520_e3595: f64 = (var_t1__blk57 * assign5520_e3594);
        let assign5520_e3596: f64 = (var_t2__blk58 + assign5520_e3595);
        var_pslsat = assign5520_e3596;
        var_pslsat_dn0 = ((var_t1__blk57_dn0 * assign5520_e3594) + (var_t1__blk57 * (-var_tx__blk60_dn0)));
        var_pslsat_dn2 = ((var_t1__blk57_dn2 * assign5520_e3594) + (var_t1__blk57 * (-var_tx__blk60_dn2)));
        var_pslsat_dn6 = (var_t2__blk58_dn6 + ((var_t1__blk57_dn6 * assign5520_e3594) + (var_t1__blk57 * (-var_tx__blk60_dn6))));
        var_pslsat_dn7 = (var_t2__blk58_dn7 + ((var_t1__blk57_dn7 * assign5520_e3594) + (var_t1__blk57 * (-var_tx__blk60_dn7))));
        var_pslsat_dn10 = ((var_t1__blk57_dn10 * assign5520_e3594) + (var_t1__blk57 * (-var_tx__blk60_dn10)));
        var_pslsat_dn11 = (var_t2__blk58_dn11 + ((var_t1__blk57_dn11 * assign5520_e3594) + (var_t1__blk57 * (-var_tx__blk60_dn11))));
        var_pslsat_dn12 = ((var_t1__blk57_dn12 * assign5520_e3594) + (var_t1__blk57 * (-var_tx__blk60_dn12)));
        var_pslsat_dn17 = ((var_t1__blk57_dn17 * assign5520_e3594) + (var_t1__blk57 * (-var_tx__blk60_dn17)));
        var_pslsat_rv = 0.0;

        let assign5530_e3599: f64 = (var_pslsat - var_pb2);
        var_vdsats = assign5530_e3599;
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
        *var_guard55_slot = var_guard55;
        *var_guard55_rv_slot = var_guard55_rv;
        *var_guard56_slot = var_guard56;
        *var_guard56_rv_slot = var_guard56_rv;
        *var_guard61_slot = var_guard61;
        *var_guard61_rv_slot = var_guard61_rv;
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
        *var_t1__blk54_slot = var_t1__blk54;
        *var_t1__blk54_dn0_slot = var_t1__blk54_dn0;
        *var_t1__blk54_dn10_slot = var_t1__blk54_dn10;
        *var_t1__blk54_dn11_slot = var_t1__blk54_dn11;
        *var_t1__blk54_dn12_slot = var_t1__blk54_dn12;
        *var_t1__blk54_dn17_slot = var_t1__blk54_dn17;
        *var_t1__blk54_dn2_slot = var_t1__blk54_dn2;
        *var_t1__blk54_dn6_slot = var_t1__blk54_dn6;
        *var_t1__blk54_dn7_slot = var_t1__blk54_dn7;
        *var_t1__blk54_rv_slot = var_t1__blk54_rv;
        *var_t1__blk57_slot = var_t1__blk57;
        *var_t1__blk57_dn0_slot = var_t1__blk57_dn0;
        *var_t1__blk57_dn10_slot = var_t1__blk57_dn10;
        *var_t1__blk57_dn11_slot = var_t1__blk57_dn11;
        *var_t1__blk57_dn12_slot = var_t1__blk57_dn12;
        *var_t1__blk57_dn17_slot = var_t1__blk57_dn17;
        *var_t1__blk57_dn2_slot = var_t1__blk57_dn2;
        *var_t1__blk57_dn6_slot = var_t1__blk57_dn6;
        *var_t1__blk57_dn7_slot = var_t1__blk57_dn7;
        *var_t1__blk57_rv_slot = var_t1__blk57_rv;
        *var_t2__blk58_slot = var_t2__blk58;
        *var_t2__blk58_dn11_slot = var_t2__blk58_dn11;
        *var_t2__blk58_dn6_slot = var_t2__blk58_dn6;
        *var_t2__blk58_dn7_slot = var_t2__blk58_dn7;
        *var_t2__blk58_rv_slot = var_t2__blk58_rv;
        *var_t3__blk59_slot = var_t3__blk59;
        *var_t3__blk59_dn0_slot = var_t3__blk59_dn0;
        *var_t3__blk59_dn10_slot = var_t3__blk59_dn10;
        *var_t3__blk59_dn11_slot = var_t3__blk59_dn11;
        *var_t3__blk59_dn12_slot = var_t3__blk59_dn12;
        *var_t3__blk59_dn17_slot = var_t3__blk59_dn17;
        *var_t3__blk59_dn2_slot = var_t3__blk59_dn2;
        *var_t3__blk59_dn6_slot = var_t3__blk59_dn6;
        *var_t3__blk59_dn7_slot = var_t3__blk59_dn7;
        *var_t3__blk59_rv_slot = var_t3__blk59_rv;
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
        *var_tx__blk60_slot = var_tx__blk60;
        *var_tx__blk60_dn0_slot = var_tx__blk60_dn0;
        *var_tx__blk60_dn10_slot = var_tx__blk60_dn10;
        *var_tx__blk60_dn11_slot = var_tx__blk60_dn11;
        *var_tx__blk60_dn12_slot = var_tx__blk60_dn12;
        *var_tx__blk60_dn17_slot = var_tx__blk60_dn17;
        *var_tx__blk60_dn2_slot = var_tx__blk60_dn2;
        *var_tx__blk60_dn6_slot = var_tx__blk60_dn6;
        *var_tx__blk60_dn7_slot = var_tx__blk60_dn7;
        *var_tx__blk60_rv_slot = var_tx__blk60_rv;
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
        var_guard68_slot: &mut f64,
        var_guard68_rv_slot: &mut f64,
        var_guard69_slot: &mut f64,
        var_guard69_rv_slot: &mut f64,
        var_guard70_slot: &mut f64,
        var_guard70_rv_slot: &mut f64,
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
        var_t1__blk57_slot: &mut f64,
        var_t1__blk57_dn0_slot: &mut f64,
        var_t1__blk57_dn10_slot: &mut f64,
        var_t1__blk57_dn11_slot: &mut f64,
        var_t1__blk57_dn12_slot: &mut f64,
        var_t1__blk57_dn17_slot: &mut f64,
        var_t1__blk57_dn2_slot: &mut f64,
        var_t1__blk57_dn6_slot: &mut f64,
        var_t1__blk57_dn7_slot: &mut f64,
        var_t1__blk57_rv_slot: &mut f64,
        var_t2__blk62_slot: &mut f64,
        var_t2__blk62_dn0_slot: &mut f64,
        var_t2__blk62_dn10_slot: &mut f64,
        var_t2__blk62_dn11_slot: &mut f64,
        var_t2__blk62_dn12_slot: &mut f64,
        var_t2__blk62_dn17_slot: &mut f64,
        var_t2__blk62_dn2_slot: &mut f64,
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
        var_t4__blk64_slot: &mut f64,
        var_t4__blk64_dn0_slot: &mut f64,
        var_t4__blk64_dn10_slot: &mut f64,
        var_t4__blk64_dn11_slot: &mut f64,
        var_t4__blk64_dn12_slot: &mut f64,
        var_t4__blk64_dn17_slot: &mut f64,
        var_t4__blk64_dn2_slot: &mut f64,
        var_t4__blk64_dn6_slot: &mut f64,
        var_t4__blk64_dn7_slot: &mut f64,
        var_t4__blk64_rv_slot: &mut f64,
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
        var_t5__blk66_slot: &mut f64,
        var_t5__blk66_dn0_slot: &mut f64,
        var_t5__blk66_dn10_slot: &mut f64,
        var_t5__blk66_dn11_slot: &mut f64,
        var_t5__blk66_dn12_slot: &mut f64,
        var_t5__blk66_dn17_slot: &mut f64,
        var_t5__blk66_dn2_slot: &mut f64,
        var_t5__blk66_dn6_slot: &mut f64,
        var_t5__blk66_dn7_slot: &mut f64,
        var_t5__blk66_rv_slot: &mut f64,
        var_t6__blk67_slot: &mut f64,
        var_t6__blk67_dn0_slot: &mut f64,
        var_t6__blk67_dn10_slot: &mut f64,
        var_t6__blk67_dn11_slot: &mut f64,
        var_t6__blk67_dn12_slot: &mut f64,
        var_t6__blk67_dn17_slot: &mut f64,
        var_t6__blk67_dn2_slot: &mut f64,
        var_t6__blk67_dn6_slot: &mut f64,
        var_t6__blk67_dn7_slot: &mut f64,
        var_t6__blk67_rv_slot: &mut f64,
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
        var_tx__blk60_slot: &mut f64,
        var_tx__blk60_dn0_slot: &mut f64,
        var_tx__blk60_dn10_slot: &mut f64,
        var_tx__blk60_dn11_slot: &mut f64,
        var_tx__blk60_dn12_slot: &mut f64,
        var_tx__blk60_dn17_slot: &mut f64,
        var_tx__blk60_dn2_slot: &mut f64,
        var_tx__blk60_dn6_slot: &mut f64,
        var_tx__blk60_dn7_slot: &mut f64,
        var_tx__blk60_rv_slot: &mut f64,
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
        let mut var_guard68: f64 = *var_guard68_slot;
        let mut var_guard68_rv: f64 = *var_guard68_rv_slot;
        let mut var_guard69: f64 = *var_guard69_slot;
        let mut var_guard69_rv: f64 = *var_guard69_rv_slot;
        let mut var_guard70: f64 = *var_guard70_slot;
        let mut var_guard70_rv: f64 = *var_guard70_rv_slot;
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
        let mut var_t1__blk57: f64 = *var_t1__blk57_slot;
        let mut var_t1__blk57_dn0: f64 = *var_t1__blk57_dn0_slot;
        let mut var_t1__blk57_dn10: f64 = *var_t1__blk57_dn10_slot;
        let mut var_t1__blk57_dn11: f64 = *var_t1__blk57_dn11_slot;
        let mut var_t1__blk57_dn12: f64 = *var_t1__blk57_dn12_slot;
        let mut var_t1__blk57_dn17: f64 = *var_t1__blk57_dn17_slot;
        let mut var_t1__blk57_dn2: f64 = *var_t1__blk57_dn2_slot;
        let mut var_t1__blk57_dn6: f64 = *var_t1__blk57_dn6_slot;
        let mut var_t1__blk57_dn7: f64 = *var_t1__blk57_dn7_slot;
        let mut var_t1__blk57_rv: f64 = *var_t1__blk57_rv_slot;
        let mut var_t2__blk62: f64 = *var_t2__blk62_slot;
        let mut var_t2__blk62_dn0: f64 = *var_t2__blk62_dn0_slot;
        let mut var_t2__blk62_dn10: f64 = *var_t2__blk62_dn10_slot;
        let mut var_t2__blk62_dn11: f64 = *var_t2__blk62_dn11_slot;
        let mut var_t2__blk62_dn12: f64 = *var_t2__blk62_dn12_slot;
        let mut var_t2__blk62_dn17: f64 = *var_t2__blk62_dn17_slot;
        let mut var_t2__blk62_dn2: f64 = *var_t2__blk62_dn2_slot;
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
        let mut var_t4__blk64: f64 = *var_t4__blk64_slot;
        let mut var_t4__blk64_dn0: f64 = *var_t4__blk64_dn0_slot;
        let mut var_t4__blk64_dn10: f64 = *var_t4__blk64_dn10_slot;
        let mut var_t4__blk64_dn11: f64 = *var_t4__blk64_dn11_slot;
        let mut var_t4__blk64_dn12: f64 = *var_t4__blk64_dn12_slot;
        let mut var_t4__blk64_dn17: f64 = *var_t4__blk64_dn17_slot;
        let mut var_t4__blk64_dn2: f64 = *var_t4__blk64_dn2_slot;
        let mut var_t4__blk64_dn6: f64 = *var_t4__blk64_dn6_slot;
        let mut var_t4__blk64_dn7: f64 = *var_t4__blk64_dn7_slot;
        let mut var_t4__blk64_rv: f64 = *var_t4__blk64_rv_slot;
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
        let mut var_t5__blk66: f64 = *var_t5__blk66_slot;
        let mut var_t5__blk66_dn0: f64 = *var_t5__blk66_dn0_slot;
        let mut var_t5__blk66_dn10: f64 = *var_t5__blk66_dn10_slot;
        let mut var_t5__blk66_dn11: f64 = *var_t5__blk66_dn11_slot;
        let mut var_t5__blk66_dn12: f64 = *var_t5__blk66_dn12_slot;
        let mut var_t5__blk66_dn17: f64 = *var_t5__blk66_dn17_slot;
        let mut var_t5__blk66_dn2: f64 = *var_t5__blk66_dn2_slot;
        let mut var_t5__blk66_dn6: f64 = *var_t5__blk66_dn6_slot;
        let mut var_t5__blk66_dn7: f64 = *var_t5__blk66_dn7_slot;
        let mut var_t5__blk66_rv: f64 = *var_t5__blk66_rv_slot;
        let mut var_t6__blk67: f64 = *var_t6__blk67_slot;
        let mut var_t6__blk67_dn0: f64 = *var_t6__blk67_dn0_slot;
        let mut var_t6__blk67_dn10: f64 = *var_t6__blk67_dn10_slot;
        let mut var_t6__blk67_dn11: f64 = *var_t6__blk67_dn11_slot;
        let mut var_t6__blk67_dn12: f64 = *var_t6__blk67_dn12_slot;
        let mut var_t6__blk67_dn17: f64 = *var_t6__blk67_dn17_slot;
        let mut var_t6__blk67_dn2: f64 = *var_t6__blk67_dn2_slot;
        let mut var_t6__blk67_dn6: f64 = *var_t6__blk67_dn6_slot;
        let mut var_t6__blk67_dn7: f64 = *var_t6__blk67_dn7_slot;
        let mut var_t6__blk67_rv: f64 = *var_t6__blk67_rv_slot;
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
        let mut var_tx__blk60: f64 = *var_tx__blk60_slot;
        let mut var_tx__blk60_dn0: f64 = *var_tx__blk60_dn0_slot;
        let mut var_tx__blk60_dn10: f64 = *var_tx__blk60_dn10_slot;
        let mut var_tx__blk60_dn11: f64 = *var_tx__blk60_dn11_slot;
        let mut var_tx__blk60_dn12: f64 = *var_tx__blk60_dn12_slot;
        let mut var_tx__blk60_dn17: f64 = *var_tx__blk60_dn17_slot;
        let mut var_tx__blk60_dn2: f64 = *var_tx__blk60_dn2_slot;
        let mut var_tx__blk60_dn6: f64 = *var_tx__blk60_dn6_slot;
        let mut var_tx__blk60_dn7: f64 = *var_tx__blk60_dn7_slot;
        let mut var_tx__blk60_rv: f64 = *var_tx__blk60_rv_slot;
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

        let assign5540_e3602: f64 = (var_vdsats - 0.1);
        let assign5540_e3604: f64 = (assign5540_e3602 - 0.05);
        var_tmf1 = assign5540_e3604;
        var_tmf1_dn0 = var_vdsats_dn0;
        var_tmf1_dn2 = var_vdsats_dn2;
        var_tmf1_dn6 = var_vdsats_dn6;
        var_tmf1_dn7 = var_vdsats_dn7;
        var_tmf1_dn10 = var_vdsats_dn10;
        var_tmf1_dn11 = var_vdsats_dn11;
        var_tmf1_dn12 = var_vdsats_dn12;
        var_tmf1_dn17 = var_vdsats_dn17;
        var_tmf1_rv = 0.0;

        let assign5550_e3607: f64 = (4.0 * 0.1);
        let assign5550_e3609: f64 = (assign5550_e3607 * 0.05);
        var_tmf2 = assign5550_e3609;
        var_tmf2_dn0 = 0.0;
        var_tmf2_dn2 = 0.0;
        var_tmf2_dn6 = 0.0;
        var_tmf2_dn7 = 0.0;
        var_tmf2_dn10 = 0.0;
        var_tmf2_dn11 = 0.0;
        var_tmf2_dn12 = 0.0;
        var_tmf2_dn17 = 0.0;
        var_tmf2_rv = 0.0;

        let (assign5560_e3616, assign5560_e3616_d_n0, assign5560_e3616_d_n2, assign5560_e3616_d_n6, assign5560_e3616_d_n7, assign5560_e3616_d_n10, assign5560_e3616_d_n11, assign5560_e3616_d_n12, assign5560_e3616_d_n17,) = {
    if (var_tmf2 > 0.0) {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    } else {
        let assign5560_e3615: f64 = (-var_tmf2);
        (assign5560_e3615, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
    }
};
        var_tmf2 = assign5560_e3616;
        var_tmf2_dn0 = assign5560_e3616_d_n0;
        var_tmf2_dn2 = assign5560_e3616_d_n2;
        var_tmf2_dn6 = assign5560_e3616_d_n6;
        var_tmf2_dn7 = assign5560_e3616_d_n7;
        var_tmf2_dn10 = assign5560_e3616_d_n10;
        var_tmf2_dn11 = assign5560_e3616_d_n11;
        var_tmf2_dn12 = assign5560_e3616_d_n12;
        var_tmf2_dn17 = assign5560_e3616_d_n17;
        var_tmf2_rv = 0.0;

        let assign5570_e3619: f64 = (var_tmf1 * var_tmf1);
        let assign5570_e3621: f64 = (assign5570_e3619 + var_tmf2);
        let assign5570_e3622: f64 = (assign5570_e3621).sqrt();
        var_tmf2 = assign5570_e3622;
        var_tmf2_dn0 = ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5570_e3622));
        var_tmf2_dn2 = ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5570_e3622));
        var_tmf2_dn6 = ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign5570_e3622));
        var_tmf2_dn7 = ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign5570_e3622));
        var_tmf2_dn10 = ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign5570_e3622));
        var_tmf2_dn11 = ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign5570_e3622));
        var_tmf2_dn12 = ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign5570_e3622));
        var_tmf2_dn17 = ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign5570_e3622));
        var_tmf2_rv = 0.0;

        let assign5580_e3627: f64 = (var_tmf1 + var_tmf2);
        let assign5580_e3628: f64 = (0.5 * assign5580_e3627);
        let assign5580_e3629: f64 = (0.1 + assign5580_e3628);
        var_vdsats = assign5580_e3629;
        var_vdsats_dn0 = (0.5 * (var_tmf1_dn0 + var_tmf2_dn0));
        var_vdsats_dn2 = (0.5 * (var_tmf1_dn2 + var_tmf2_dn2));
        var_vdsats_dn6 = (0.5 * (var_tmf1_dn6 + var_tmf2_dn6));
        var_vdsats_dn7 = (0.5 * (var_tmf1_dn7 + var_tmf2_dn7));
        var_vdsats_dn10 = (0.5 * (var_tmf1_dn10 + var_tmf2_dn10));
        var_vdsats_dn11 = (0.5 * (var_tmf1_dn11 + var_tmf2_dn11));
        var_vdsats_dn12 = (0.5 * (var_tmf1_dn12 + var_tmf2_dn12));
        var_vdsats_dn17 = (0.5 * (var_tmf1_dn17 + var_tmf2_dn17));
        var_vdsats_rv = 0.0;

        let assign5590_e3632: f64 = (var_vds / var_vdsats);
        var_t1__blk57 = assign5590_e3632;
        var_t1__blk57_dn0 = (((var_vds_dn0 * var_vdsats) - (var_vds * var_vdsats_dn0)) / (var_vdsats * var_vdsats));
        var_t1__blk57_dn2 = (((var_vds_dn2 * var_vdsats) - (var_vds * var_vdsats_dn2)) / (var_vdsats * var_vdsats));
        var_t1__blk57_dn6 = (((var_vds_dn6 * var_vdsats) - (var_vds * var_vdsats_dn6)) / (var_vdsats * var_vdsats));
        var_t1__blk57_dn7 = (((var_vds_dn7 * var_vdsats) - (var_vds * var_vdsats_dn7)) / (var_vdsats * var_vdsats));
        var_t1__blk57_dn10 = (((var_vds_dn10 * var_vdsats) - (var_vds * var_vdsats_dn10)) / (var_vdsats * var_vdsats));
        var_t1__blk57_dn11 = (((var_vds_dn11 * var_vdsats) - (var_vds * var_vdsats_dn11)) / (var_vdsats * var_vdsats));
        var_t1__blk57_dn12 = (((var_vds_dn12 * var_vdsats) - (var_vds * var_vdsats_dn12)) / (var_vdsats * var_vdsats));
        var_t1__blk57_dn17 = (((var_vds_dn17 * var_vdsats) - (var_vds * var_vdsats_dn17)) / (var_vdsats * var_vdsats));
        var_t1__blk57_rv = 0.0;

        let assign5600_e3635: f64 = var_t1__blk57;
        var_tmf1 = assign5600_e3635;
        var_tmf1_dn0 = var_t1__blk57_dn0;
        var_tmf1_dn2 = var_t1__blk57_dn2;
        var_tmf1_dn6 = var_t1__blk57_dn6;
        var_tmf1_dn7 = var_t1__blk57_dn7;
        var_tmf1_dn10 = var_t1__blk57_dn10;
        var_tmf1_dn11 = var_t1__blk57_dn11;
        var_tmf1_dn12 = var_t1__blk57_dn12;
        var_tmf1_dn17 = var_t1__blk57_dn17;
        var_tmf1_rv = 0.0;

        let assign5610_e3638: f64 = (var_tmf1 * var_tmf1);
        var_tmf2 = assign5610_e3638;
        var_tmf2_dn0 = ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0));
        var_tmf2_dn2 = ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2));
        var_tmf2_dn6 = ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6));
        var_tmf2_dn7 = ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7));
        var_tmf2_dn10 = ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10));
        var_tmf2_dn11 = ((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11));
        var_tmf2_dn12 = ((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12));
        var_tmf2_dn17 = ((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17));
        var_tmf2_rv = 0.0;

        let assign5620_e3641: f64 = (var_tmf2 * var_tmf1);
        var_tmf3 = assign5620_e3641;
        var_tmf3_dn0 = ((var_tmf2_dn0 * var_tmf1) + (var_tmf2 * var_tmf1_dn0));
        var_tmf3_dn2 = ((var_tmf2_dn2 * var_tmf1) + (var_tmf2 * var_tmf1_dn2));
        var_tmf3_dn6 = ((var_tmf2_dn6 * var_tmf1) + (var_tmf2 * var_tmf1_dn6));
        var_tmf3_dn7 = ((var_tmf2_dn7 * var_tmf1) + (var_tmf2 * var_tmf1_dn7));
        var_tmf3_dn10 = ((var_tmf2_dn10 * var_tmf1) + (var_tmf2 * var_tmf1_dn10));
        var_tmf3_dn11 = ((var_tmf2_dn11 * var_tmf1) + (var_tmf2 * var_tmf1_dn11));
        var_tmf3_dn12 = ((var_tmf2_dn12 * var_tmf1) + (var_tmf2 * var_tmf1_dn12));
        var_tmf3_dn17 = ((var_tmf2_dn17 * var_tmf1) + (var_tmf2 * var_tmf1_dn17));
        var_tmf3_rv = 0.0;

        let assign5630_e3644: f64 = (var_tmf2 * var_tmf2);
        var_tmf4 = assign5630_e3644;
        var_tmf4_dn0 = ((var_tmf2_dn0 * var_tmf2) + (var_tmf2 * var_tmf2_dn0));
        var_tmf4_dn2 = ((var_tmf2_dn2 * var_tmf2) + (var_tmf2 * var_tmf2_dn2));
        var_tmf4_dn6 = ((var_tmf2_dn6 * var_tmf2) + (var_tmf2 * var_tmf2_dn6));
        var_tmf4_dn7 = ((var_tmf2_dn7 * var_tmf2) + (var_tmf2 * var_tmf2_dn7));
        var_tmf4_dn10 = ((var_tmf2_dn10 * var_tmf2) + (var_tmf2 * var_tmf2_dn10));
        var_tmf4_dn11 = ((var_tmf2_dn11 * var_tmf2) + (var_tmf2 * var_tmf2_dn11));
        var_tmf4_dn12 = ((var_tmf2_dn12 * var_tmf2) + (var_tmf2 * var_tmf2_dn12));
        var_tmf4_dn17 = ((var_tmf2_dn17 * var_tmf2) + (var_tmf2 * var_tmf2_dn17));
        var_tmf4_rv = 0.0;

        let assign5640_e3648: f64 = (1.0 + var_tmf1);
        let assign5640_e3650: f64 = (assign5640_e3648 + var_tmf2);
        let assign5640_e3652: f64 = (assign5640_e3650 + var_tmf3);
        let assign5640_e3654: f64 = (assign5640_e3652 + var_tmf4);
        let assign5640_e3655: f64 = (1.0 / assign5640_e3654);
        var_tx__blk60 = assign5640_e3655;
        var_tx__blk60_dn0 = (-((((var_tmf1_dn0 + var_tmf2_dn0) + var_tmf3_dn0) + var_tmf4_dn0) / (assign5640_e3654 * assign5640_e3654)));
        var_tx__blk60_dn2 = (-((((var_tmf1_dn2 + var_tmf2_dn2) + var_tmf3_dn2) + var_tmf4_dn2) / (assign5640_e3654 * assign5640_e3654)));
        var_tx__blk60_dn6 = (-((((var_tmf1_dn6 + var_tmf2_dn6) + var_tmf3_dn6) + var_tmf4_dn6) / (assign5640_e3654 * assign5640_e3654)));
        var_tx__blk60_dn7 = (-((((var_tmf1_dn7 + var_tmf2_dn7) + var_tmf3_dn7) + var_tmf4_dn7) / (assign5640_e3654 * assign5640_e3654)));
        var_tx__blk60_dn10 = (-((((var_tmf1_dn10 + var_tmf2_dn10) + var_tmf3_dn10) + var_tmf4_dn10) / (assign5640_e3654 * assign5640_e3654)));
        var_tx__blk60_dn11 = (-((((var_tmf1_dn11 + var_tmf2_dn11) + var_tmf3_dn11) + var_tmf4_dn11) / (assign5640_e3654 * assign5640_e3654)));
        var_tx__blk60_dn12 = (-((((var_tmf1_dn12 + var_tmf2_dn12) + var_tmf3_dn12) + var_tmf4_dn12) / (assign5640_e3654 * assign5640_e3654)));
        var_tx__blk60_dn17 = (-((((var_tmf1_dn17 + var_tmf2_dn17) + var_tmf3_dn17) + var_tmf4_dn17) / (assign5640_e3654 * assign5640_e3654)));
        var_tx__blk60_rv = 0.0;

        let assign5650_e3659: f64 = (2.0 * var_tmf1);
        let assign5650_e3660: f64 = (1.0 + assign5650_e3659);
        let assign5650_e3663: f64 = (3.0 * var_tmf2);
        let assign5650_e3664: f64 = (assign5650_e3660 + assign5650_e3663);
        let assign5650_e3667: f64 = (4.0 * var_tmf3);
        let assign5650_e3668: f64 = (assign5650_e3664 + assign5650_e3667);
        let assign5650_e3669: f64 = (-assign5650_e3668);
        let assign5650_e3671: f64 = (assign5650_e3669 * var_tx__blk60);
        let assign5650_e3673: f64 = (assign5650_e3671 * var_tx__blk60);
        var_t0 = assign5650_e3673;
        var_t0_dn0 = (((((-(((2.0 * var_tmf1_dn0) + (3.0 * var_tmf2_dn0)) + (4.0 * var_tmf3_dn0))) * var_tx__blk60) + (assign5650_e3669 * var_tx__blk60_dn0)) * var_tx__blk60) + (assign5650_e3671 * var_tx__blk60_dn0));
        var_t0_dn2 = (((((-(((2.0 * var_tmf1_dn2) + (3.0 * var_tmf2_dn2)) + (4.0 * var_tmf3_dn2))) * var_tx__blk60) + (assign5650_e3669 * var_tx__blk60_dn2)) * var_tx__blk60) + (assign5650_e3671 * var_tx__blk60_dn2));
        var_t0_dn6 = (((((-(((2.0 * var_tmf1_dn6) + (3.0 * var_tmf2_dn6)) + (4.0 * var_tmf3_dn6))) * var_tx__blk60) + (assign5650_e3669 * var_tx__blk60_dn6)) * var_tx__blk60) + (assign5650_e3671 * var_tx__blk60_dn6));
        var_t0_dn7 = (((((-(((2.0 * var_tmf1_dn7) + (3.0 * var_tmf2_dn7)) + (4.0 * var_tmf3_dn7))) * var_tx__blk60) + (assign5650_e3669 * var_tx__blk60_dn7)) * var_tx__blk60) + (assign5650_e3671 * var_tx__blk60_dn7));
        var_t0_dn10 = (((((-(((2.0 * var_tmf1_dn10) + (3.0 * var_tmf2_dn10)) + (4.0 * var_tmf3_dn10))) * var_tx__blk60) + (assign5650_e3669 * var_tx__blk60_dn10)) * var_tx__blk60) + (assign5650_e3671 * var_tx__blk60_dn10));
        var_t0_dn11 = (((((-(((2.0 * var_tmf1_dn11) + (3.0 * var_tmf2_dn11)) + (4.0 * var_tmf3_dn11))) * var_tx__blk60) + (assign5650_e3669 * var_tx__blk60_dn11)) * var_tx__blk60) + (assign5650_e3671 * var_tx__blk60_dn11));
        var_t0_dn12 = (((((-(((2.0 * var_tmf1_dn12) + (3.0 * var_tmf2_dn12)) + (4.0 * var_tmf3_dn12))) * var_tx__blk60) + (assign5650_e3669 * var_tx__blk60_dn12)) * var_tx__blk60) + (assign5650_e3671 * var_tx__blk60_dn12));
        var_t0_dn17 = (((((-(((2.0 * var_tmf1_dn17) + (3.0 * var_tmf2_dn17)) + (4.0 * var_tmf3_dn17))) * var_tx__blk60) + (assign5650_e3669 * var_tx__blk60_dn17)) * var_tx__blk60) + (assign5650_e3671 * var_tx__blk60_dn17));
        var_t0_rv = 0.0;

        let assign5660_e3677: f64 = (1.0 - var_tx__blk60);
        let assign5660_e3678: f64 = assign5660_e3677;
        var_tx__blk60 = assign5660_e3678;
        var_tx__blk60_dn0 = (-var_tx__blk60_dn0);
        var_tx__blk60_dn2 = (-var_tx__blk60_dn2);
        var_tx__blk60_dn6 = (-var_tx__blk60_dn6);
        var_tx__blk60_dn7 = (-var_tx__blk60_dn7);
        var_tx__blk60_dn10 = (-var_tx__blk60_dn10);
        var_tx__blk60_dn11 = (-var_tx__blk60_dn11);
        var_tx__blk60_dn12 = (-var_tx__blk60_dn12);
        var_tx__blk60_dn17 = (-var_tx__blk60_dn17);
        var_tx__blk60_rv = 0.0;

        let assign5670_e3680: f64 = (-var_t0);
        var_t0 = assign5670_e3680;
        var_t0_dn0 = (-var_t0_dn0);
        var_t0_dn2 = (-var_t0_dn2);
        var_t0_dn6 = (-var_t0_dn6);
        var_t0_dn7 = (-var_t0_dn7);
        var_t0_dn10 = (-var_t0_dn10);
        var_t0_dn11 = (-var_t0_dn11);
        var_t0_dn12 = (-var_t0_dn12);
        var_t0_dn17 = (-var_t0_dn17);
        var_t0_rv = 0.0;

        let assign5680_e3683: f64 = (var_tx__blk60 * var_tx__blk60);
        var_fmdvds = assign5680_e3683;
        var_fmdvds_dn0 = ((var_tx__blk60_dn0 * var_tx__blk60) + (var_tx__blk60 * var_tx__blk60_dn0));
        var_fmdvds_dn2 = ((var_tx__blk60_dn2 * var_tx__blk60) + (var_tx__blk60 * var_tx__blk60_dn2));
        var_fmdvds_dn6 = ((var_tx__blk60_dn6 * var_tx__blk60) + (var_tx__blk60 * var_tx__blk60_dn6));
        var_fmdvds_dn7 = ((var_tx__blk60_dn7 * var_tx__blk60) + (var_tx__blk60 * var_tx__blk60_dn7));
        var_fmdvds_dn10 = ((var_tx__blk60_dn10 * var_tx__blk60) + (var_tx__blk60 * var_tx__blk60_dn10));
        var_fmdvds_dn11 = ((var_tx__blk60_dn11 * var_tx__blk60) + (var_tx__blk60 * var_tx__blk60_dn11));
        var_fmdvds_dn12 = ((var_tx__blk60_dn12 * var_tx__blk60) + (var_tx__blk60 * var_tx__blk60_dn12));
        var_fmdvds_dn17 = ((var_tx__blk60_dn17 * var_tx__blk60) + (var_tx__blk60 * var_tx__blk60_dn17));
        var_fmdvds_rv = 0.0;

        let assign5690_e3694: f64 = if (((p.p204 == 0.0) && (p.p206 == 0.0)) || (p.p205 == 0.0)) { 1.0 } else { 0.0 };
        var_guard68 = assign5690_e3694;
        var_guard68_rv = 0.0;

        let (assign5700_e3698,) = {
    if (var_guard68 != 0.0) {
        (0.0,)
    } else {
        (var_flg_qme,)
    }
};
        var_flg_qme = assign5700_e3698;
        var_flg_qme_rv = 0.0;

        let (assign5710_e3703,) = {
    if (var_guard68 == 0.0) {
        (1.0,)
    } else {
        (var_flg_qme,)
    }
};
        var_flg_qme = assign5710_e3703;
        var_flg_qme_rv = 0.0;

        let assign5720_e3706: f64 = (2.0 * var_q_nsub);
        let assign5720_e3708: f64 = (assign5720_e3706 * 1.034943e-10);
        let assign5720_e3710: f64 = (assign5720_e3708 * var_pb20);
        let assign5720_e3711: f64 = (assign5720_e3710).sqrt();
        var_t2__blk62 = assign5720_e3711;
        var_t2__blk62_dn0 = (((((2.0 * var_q_nsub_dn0) * 1.034943e-10) * var_pb20) + (assign5720_e3708 * var_pb20_dn0)) / (2.0 * assign5720_e3711));
        var_t2__blk62_dn2 = (((((2.0 * var_q_nsub_dn2) * 1.034943e-10) * var_pb20) + (assign5720_e3708 * var_pb20_dn2)) / (2.0 * assign5720_e3711));
        var_t2__blk62_dn6 = (((((2.0 * var_q_nsub_dn6) * 1.034943e-10) * var_pb20) + (assign5720_e3708 * var_pb20_dn6)) / (2.0 * assign5720_e3711));
        var_t2__blk62_dn7 = (((((2.0 * var_q_nsub_dn7) * 1.034943e-10) * var_pb20) + (assign5720_e3708 * var_pb20_dn7)) / (2.0 * assign5720_e3711));
        var_t2__blk62_dn10 = (((((2.0 * var_q_nsub_dn10) * 1.034943e-10) * var_pb20) + (assign5720_e3708 * var_pb20_dn10)) / (2.0 * assign5720_e3711));
        var_t2__blk62_dn11 = (((((2.0 * var_q_nsub_dn11) * 1.034943e-10) * var_pb20) + (assign5720_e3708 * var_pb20_dn11)) / (2.0 * assign5720_e3711));
        var_t2__blk62_dn12 = (((((2.0 * var_q_nsub_dn12) * 1.034943e-10) * var_pb20) + (assign5720_e3708 * var_pb20_dn12)) / (2.0 * assign5720_e3711));
        var_t2__blk62_dn17 = (((((2.0 * var_q_nsub_dn17) * 1.034943e-10) * var_pb20) + (assign5720_e3708 * var_pb20_dn17)) / (2.0 * assign5720_e3711));
        var_t2__blk62_rv = 0.0;

        let assign5730_e3714: f64 = (var_pb20 + var_vfb);
        let assign5730_e3717: f64 = (var_t2__blk62 / var_c_fox0);
        let assign5730_e3718: f64 = (assign5730_e3714 + assign5730_e3717);
        var_vthq = assign5730_e3718;
        var_vthq_dn0 = (var_pb20_dn0 + (var_t2__blk62_dn0 / var_c_fox0));
        var_vthq_dn2 = (var_pb20_dn2 + (var_t2__blk62_dn2 / var_c_fox0));
        var_vthq_dn6 = (var_pb20_dn6 + (var_t2__blk62_dn6 / var_c_fox0));
        var_vthq_dn7 = (var_pb20_dn7 + (var_t2__blk62_dn7 / var_c_fox0));
        var_vthq_dn10 = (var_pb20_dn10 + (var_t2__blk62_dn10 / var_c_fox0));
        var_vthq_dn11 = (var_pb20_dn11 + (var_t2__blk62_dn11 / var_c_fox0));
        var_vthq_dn12 = (var_pb20_dn12 + (var_t2__blk62_dn12 / var_c_fox0));
        var_vthq_dn17 = (var_pb20_dn17 + (var_t2__blk62_dn17 / var_c_fox0));
        var_vthq_rv = 0.0;

        let assign5740_e3721: f64 = if var_flg_qme == 0.0 { 1.0 } else { 0.0 };
        var_guard69 = assign5740_e3721;
        var_guard69_rv = 0.0;

        let (assign5750_e3725, assign5750_e3725_d_n0, assign5750_e3725_d_n2, assign5750_e3725_d_n6, assign5750_e3725_d_n7, assign5750_e3725_d_n10, assign5750_e3725_d_n11, assign5750_e3725_d_n12, assign5750_e3725_d_n17,) = {
    if (var_guard69 != 0.0) {
        (var_tfox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tfoxe, var_tfoxe_dn0, var_tfoxe_dn2, var_tfoxe_dn6, var_tfoxe_dn7, var_tfoxe_dn10, var_tfoxe_dn11, var_tfoxe_dn12, var_tfoxe_dn17,)
    }
};
        var_tfoxe = assign5750_e3725;
        var_tfoxe_dn0 = assign5750_e3725_d_n0;
        var_tfoxe_dn2 = assign5750_e3725_d_n2;
        var_tfoxe_dn6 = assign5750_e3725_d_n6;
        var_tfoxe_dn7 = assign5750_e3725_d_n7;
        var_tfoxe_dn10 = assign5750_e3725_d_n10;
        var_tfoxe_dn11 = assign5750_e3725_d_n11;
        var_tfoxe_dn12 = assign5750_e3725_d_n12;
        var_tfoxe_dn17 = assign5750_e3725_d_n17;
        var_tfoxe_rv = 0.0;

        let (assign5760_e3729, assign5760_e3729_d_n0, assign5760_e3729_d_n2, assign5760_e3729_d_n6, assign5760_e3729_d_n7, assign5760_e3729_d_n10, assign5760_e3729_d_n11, assign5760_e3729_d_n12, assign5760_e3729_d_n17,) = {
    if (var_guard69 != 0.0) {
        (var_c_fox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_c_fox, var_c_fox_dn0, var_c_fox_dn2, var_c_fox_dn6, var_c_fox_dn7, var_c_fox_dn10, var_c_fox_dn11, var_c_fox_dn12, var_c_fox_dn17,)
    }
};
        var_c_fox = assign5760_e3729;
        var_c_fox_dn0 = assign5760_e3729_d_n0;
        var_c_fox_dn2 = assign5760_e3729_d_n2;
        var_c_fox_dn6 = assign5760_e3729_d_n6;
        var_c_fox_dn7 = assign5760_e3729_d_n7;
        var_c_fox_dn10 = assign5760_e3729_d_n10;
        var_c_fox_dn11 = assign5760_e3729_d_n11;
        var_c_fox_dn12 = assign5760_e3729_d_n12;
        var_c_fox_dn17 = assign5760_e3729_d_n17;
        var_c_fox_rv = 0.0;

        let (assign5770_e3733, assign5770_e3733_d_n0, assign5770_e3733_d_n2, assign5770_e3733_d_n6, assign5770_e3733_d_n7, assign5770_e3733_d_n10, assign5770_e3733_d_n11, assign5770_e3733_d_n12, assign5770_e3733_d_n17,) = {
    if (var_guard69 != 0.0) {
        (var_c_fox0_inv, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_c_fox_inv, var_c_fox_inv_dn0, var_c_fox_inv_dn2, var_c_fox_inv_dn6, var_c_fox_inv_dn7, var_c_fox_inv_dn10, var_c_fox_inv_dn11, var_c_fox_inv_dn12, var_c_fox_inv_dn17,)
    }
};
        var_c_fox_inv = assign5770_e3733;
        var_c_fox_inv_dn0 = assign5770_e3733_d_n0;
        var_c_fox_inv_dn2 = assign5770_e3733_d_n2;
        var_c_fox_inv_dn6 = assign5770_e3733_d_n6;
        var_c_fox_inv_dn7 = assign5770_e3733_d_n7;
        var_c_fox_inv_dn10 = assign5770_e3733_d_n10;
        var_c_fox_inv_dn11 = assign5770_e3733_d_n11;
        var_c_fox_inv_dn12 = assign5770_e3733_d_n12;
        var_c_fox_inv_dn17 = assign5770_e3733_d_n17;
        var_c_fox_inv_rv = 0.0;

        let (assign5780_e3743, assign5780_e3743_d_n0, assign5780_e3743_d_n2, assign5780_e3743_d_n6, assign5780_e3743_d_n7, assign5780_e3743_d_n10, assign5780_e3743_d_n11, assign5780_e3743_d_n12, assign5780_e3743_d_n17,) = {
    if (var_guard69 != 0.0) {
        let assign5780_e3737: f64 = (var_cnst0soi * var_c_fox0_inv);
        let assign5780_e3739: f64 = (assign5780_e3737 * var_c_fox0_inv);
        let assign5780_e3741: f64 = (assign5780_e3739 * var_cnst0soi);
        (assign5780_e3741, ((((var_cnst0soi_dn0 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5780_e3739 * var_cnst0soi_dn0)), ((((var_cnst0soi_dn2 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5780_e3739 * var_cnst0soi_dn2)), ((((var_cnst0soi_dn6 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5780_e3739 * var_cnst0soi_dn6)), ((((var_cnst0soi_dn7 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5780_e3739 * var_cnst0soi_dn7)), ((((var_cnst0soi_dn10 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5780_e3739 * var_cnst0soi_dn10)), ((((var_cnst0soi_dn11 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5780_e3739 * var_cnst0soi_dn11)), ((((var_cnst0soi_dn12 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5780_e3739 * var_cnst0soi_dn12)), ((((var_cnst0soi_dn17 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5780_e3739 * var_cnst0soi_dn17)),)
    } else {
        (var_cnstc_foxi, var_cnstc_foxi_dn0, var_cnstc_foxi_dn2, var_cnstc_foxi_dn6, var_cnstc_foxi_dn7, var_cnstc_foxi_dn10, var_cnstc_foxi_dn11, var_cnstc_foxi_dn12, var_cnstc_foxi_dn17,)
    }
};
        var_cnstc_foxi = assign5780_e3743;
        var_cnstc_foxi_dn0 = assign5780_e3743_d_n0;
        var_cnstc_foxi_dn2 = assign5780_e3743_d_n2;
        var_cnstc_foxi_dn6 = assign5780_e3743_d_n6;
        var_cnstc_foxi_dn7 = assign5780_e3743_d_n7;
        var_cnstc_foxi_dn10 = assign5780_e3743_d_n10;
        var_cnstc_foxi_dn11 = assign5780_e3743_d_n11;
        var_cnstc_foxi_dn12 = assign5780_e3743_d_n12;
        var_cnstc_foxi_dn17 = assign5780_e3743_d_n17;
        var_cnstc_foxi_rv = 0.0;

        let (assign5790_e3754, assign5790_e3754_d_n0, assign5790_e3754_d_n2, assign5790_e3754_d_n6, assign5790_e3754_d_n7, assign5790_e3754_d_n10, assign5790_e3754_d_n11, assign5790_e3754_d_n12, assign5790_e3754_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5790_e3748: f64 = (var_vgs - var_vbsp);
        let assign5790_e3750: f64 = (assign5790_e3748 - var_vthq);
        let assign5790_e3752: f64 = (assign5790_e3750 + p.p205);
        (assign5790_e3752, ((-var_vbsp_dn0) - var_vthq_dn0), ((-var_vbsp_dn2) - var_vthq_dn2), ((var_vgs_dn6 - var_vbsp_dn6) - var_vthq_dn6), ((var_vgs_dn7 - var_vbsp_dn7) - var_vthq_dn7), ((-var_vbsp_dn10) - var_vthq_dn10), ((var_vgs_dn11 - var_vbsp_dn11) - var_vthq_dn11), ((-var_vbsp_dn12) - var_vthq_dn12), ((-var_vbsp_dn17) - var_vthq_dn17),)
    } else {
        (var_t5__blk66, var_t5__blk66_dn0, var_t5__blk66_dn2, var_t5__blk66_dn6, var_t5__blk66_dn7, var_t5__blk66_dn10, var_t5__blk66_dn11, var_t5__blk66_dn12, var_t5__blk66_dn17,)
    }
};
        var_t5__blk66 = assign5790_e3754;
        var_t5__blk66_dn0 = assign5790_e3754_d_n0;
        var_t5__blk66_dn2 = assign5790_e3754_d_n2;
        var_t5__blk66_dn6 = assign5790_e3754_d_n6;
        var_t5__blk66_dn7 = assign5790_e3754_d_n7;
        var_t5__blk66_dn10 = assign5790_e3754_d_n10;
        var_t5__blk66_dn11 = assign5790_e3754_d_n11;
        var_t5__blk66_dn12 = assign5790_e3754_d_n12;
        var_t5__blk66_dn17 = assign5790_e3754_d_n17;
        var_t5__blk66_rv = 0.0;

        let (assign5800_e3768, assign5800_e3768_d_n0, assign5800_e3768_d_n2, assign5800_e3768_d_n6, assign5800_e3768_d_n7, assign5800_e3768_d_n10, assign5800_e3768_d_n11, assign5800_e3768_d_n12, assign5800_e3768_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5800_e3759: f64 = (var_t5__blk66 * var_t5__blk66);
        let assign5800_e3762: f64 = (4.0 * 0.0001);
        let assign5800_e3764: f64 = (assign5800_e3762 * 0.0001);
        let assign5800_e3765: f64 = (assign5800_e3759 + assign5800_e3764);
        let assign5800_e3766: f64 = (assign5800_e3765).sqrt();
        (assign5800_e3766, (((var_t5__blk66_dn0 * var_t5__blk66) + (var_t5__blk66 * var_t5__blk66_dn0)) / (2.0 * assign5800_e3766)), (((var_t5__blk66_dn2 * var_t5__blk66) + (var_t5__blk66 * var_t5__blk66_dn2)) / (2.0 * assign5800_e3766)), (((var_t5__blk66_dn6 * var_t5__blk66) + (var_t5__blk66 * var_t5__blk66_dn6)) / (2.0 * assign5800_e3766)), (((var_t5__blk66_dn7 * var_t5__blk66) + (var_t5__blk66 * var_t5__blk66_dn7)) / (2.0 * assign5800_e3766)), (((var_t5__blk66_dn10 * var_t5__blk66) + (var_t5__blk66 * var_t5__blk66_dn10)) / (2.0 * assign5800_e3766)), (((var_t5__blk66_dn11 * var_t5__blk66) + (var_t5__blk66 * var_t5__blk66_dn11)) / (2.0 * assign5800_e3766)), (((var_t5__blk66_dn12 * var_t5__blk66) + (var_t5__blk66 * var_t5__blk66_dn12)) / (2.0 * assign5800_e3766)), (((var_t5__blk66_dn17 * var_t5__blk66) + (var_t5__blk66 * var_t5__blk66_dn17)) / (2.0 * assign5800_e3766)),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign5800_e3768;
        var_tmf1_dn0 = assign5800_e3768_d_n0;
        var_tmf1_dn2 = assign5800_e3768_d_n2;
        var_tmf1_dn6 = assign5800_e3768_d_n6;
        var_tmf1_dn7 = assign5800_e3768_d_n7;
        var_tmf1_dn10 = assign5800_e3768_d_n10;
        var_tmf1_dn11 = assign5800_e3768_d_n11;
        var_tmf1_dn12 = assign5800_e3768_d_n12;
        var_tmf1_dn17 = assign5800_e3768_d_n17;
        var_tmf1_rv = 0.0;

        let (assign5810_e3781, assign5810_e3781_d_n0, assign5810_e3781_d_n2, assign5810_e3781_d_n6, assign5810_e3781_d_n7, assign5810_e3781_d_n10, assign5810_e3781_d_n11, assign5810_e3781_d_n12, assign5810_e3781_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5810_e3774: f64 = (var_t5__blk66 + var_tmf1);
        let assign5810_e3775: f64 = (0.5 * assign5810_e3774);
        let assign5810_e3778: f64 = (1e-10 * 0.0001);
        let assign5810_e3779: f64 = (assign5810_e3775 + assign5810_e3778);
        (assign5810_e3779, (0.5 * (var_t5__blk66_dn0 + var_tmf1_dn0)), (0.5 * (var_t5__blk66_dn2 + var_tmf1_dn2)), (0.5 * (var_t5__blk66_dn6 + var_tmf1_dn6)), (0.5 * (var_t5__blk66_dn7 + var_tmf1_dn7)), (0.5 * (var_t5__blk66_dn10 + var_tmf1_dn10)), (0.5 * (var_t5__blk66_dn11 + var_tmf1_dn11)), (0.5 * (var_t5__blk66_dn12 + var_tmf1_dn12)), (0.5 * (var_t5__blk66_dn17 + var_tmf1_dn17)),)
    } else {
        (var_t2__blk62, var_t2__blk62_dn0, var_t2__blk62_dn2, var_t2__blk62_dn6, var_t2__blk62_dn7, var_t2__blk62_dn10, var_t2__blk62_dn11, var_t2__blk62_dn12, var_t2__blk62_dn17,)
    }
};
        var_t2__blk62 = assign5810_e3781;
        var_t2__blk62_dn0 = assign5810_e3781_d_n0;
        var_t2__blk62_dn2 = assign5810_e3781_d_n2;
        var_t2__blk62_dn6 = assign5810_e3781_d_n6;
        var_t2__blk62_dn7 = assign5810_e3781_d_n7;
        var_t2__blk62_dn10 = assign5810_e3781_d_n10;
        var_t2__blk62_dn11 = assign5810_e3781_d_n11;
        var_t2__blk62_dn12 = assign5810_e3781_d_n12;
        var_t2__blk62_dn17 = assign5810_e3781_d_n17;
        var_t2__blk62_rv = 0.0;

        let assign5820_e3784: f64 = if var_t2__blk62 < 0.0 { 1.0 } else { 0.0 };
        var_guard70 = assign5820_e3784;
        var_guard70_rv = 0.0;

        let (assign5830_e3791, assign5830_e3791_d_n0, assign5830_e3791_d_n2, assign5830_e3791_d_n6, assign5830_e3791_d_n7, assign5830_e3791_d_n10, assign5830_e3791_d_n11, assign5830_e3791_d_n12, assign5830_e3791_d_n17,) = {
    if ((var_guard69 == 0.0) && (var_guard70 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2__blk62, var_t2__blk62_dn0, var_t2__blk62_dn2, var_t2__blk62_dn6, var_t2__blk62_dn7, var_t2__blk62_dn10, var_t2__blk62_dn11, var_t2__blk62_dn12, var_t2__blk62_dn17,)
    }
};
        var_t2__blk62 = assign5830_e3791;
        var_t2__blk62_dn0 = assign5830_e3791_d_n0;
        var_t2__blk62_dn2 = assign5830_e3791_d_n2;
        var_t2__blk62_dn6 = assign5830_e3791_d_n6;
        var_t2__blk62_dn7 = assign5830_e3791_d_n7;
        var_t2__blk62_dn10 = assign5830_e3791_d_n10;
        var_t2__blk62_dn11 = assign5830_e3791_d_n11;
        var_t2__blk62_dn12 = assign5830_e3791_d_n12;
        var_t2__blk62_dn17 = assign5830_e3791_d_n17;
        var_t2__blk62_rv = 0.0;

        let (assign5840_e3798, assign5840_e3798_d_n0, assign5840_e3798_d_n2, assign5840_e3798_d_n6, assign5840_e3798_d_n7, assign5840_e3798_d_n10, assign5840_e3798_d_n11, assign5840_e3798_d_n12, assign5840_e3798_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5840_e3796: f64 = (1.0 / var_t2__blk62);
        (assign5840_e3796, (-(var_t2__blk62_dn0 / (var_t2__blk62 * var_t2__blk62))), (-(var_t2__blk62_dn2 / (var_t2__blk62 * var_t2__blk62))), (-(var_t2__blk62_dn6 / (var_t2__blk62 * var_t2__blk62))), (-(var_t2__blk62_dn7 / (var_t2__blk62 * var_t2__blk62))), (-(var_t2__blk62_dn10 / (var_t2__blk62 * var_t2__blk62))), (-(var_t2__blk62_dn11 / (var_t2__blk62 * var_t2__blk62))), (-(var_t2__blk62_dn12 / (var_t2__blk62 * var_t2__blk62))), (-(var_t2__blk62_dn17 / (var_t2__blk62 * var_t2__blk62))),)
    } else {
        (var_t3__blk63, var_t3__blk63_dn0, var_t3__blk63_dn2, var_t3__blk63_dn6, var_t3__blk63_dn7, var_t3__blk63_dn10, var_t3__blk63_dn11, var_t3__blk63_dn12, var_t3__blk63_dn17,)
    }
};
        var_t3__blk63 = assign5840_e3798;
        var_t3__blk63_dn0 = assign5840_e3798_d_n0;
        var_t3__blk63_dn2 = assign5840_e3798_d_n2;
        var_t3__blk63_dn6 = assign5840_e3798_d_n6;
        var_t3__blk63_dn7 = assign5840_e3798_d_n7;
        var_t3__blk63_dn10 = assign5840_e3798_d_n10;
        var_t3__blk63_dn11 = assign5840_e3798_d_n11;
        var_t3__blk63_dn12 = assign5840_e3798_d_n12;
        var_t3__blk63_dn17 = assign5840_e3798_d_n17;
        var_t3__blk63_rv = 0.0;

        let (assign5850_e3806, assign5850_e3806_d_n0, assign5850_e3806_d_n2, assign5850_e3806_d_n6, assign5850_e3806_d_n7, assign5850_e3806_d_n10, assign5850_e3806_d_n11, assign5850_e3806_d_n12, assign5850_e3806_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5850_e3803: f64 = (var_vthq).abs();
        let assign5850_e3804: f64 = (2.0 * assign5850_e3803);
        (assign5850_e3804, (2.0 * if var_vthq >= 0.0 { var_vthq_dn0 } else { (-var_vthq_dn0) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn2 } else { (-var_vthq_dn2) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn6 } else { (-var_vthq_dn6) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn7 } else { (-var_vthq_dn7) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn10 } else { (-var_vthq_dn10) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn11 } else { (-var_vthq_dn11) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn12 } else { (-var_vthq_dn12) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn17 } else { (-var_vthq_dn17) }),)
    } else {
        (var_t4w, var_t4w_dn0, var_t4w_dn2, var_t4w_dn6, var_t4w_dn7, var_t4w_dn10, var_t4w_dn11, var_t4w_dn12, var_t4w_dn17,)
    }
};
        var_t4w = assign5850_e3806;
        var_t4w_dn0 = assign5850_e3806_d_n0;
        var_t4w_dn2 = assign5850_e3806_d_n2;
        var_t4w_dn6 = assign5850_e3806_d_n6;
        var_t4w_dn7 = assign5850_e3806_d_n7;
        var_t4w_dn10 = assign5850_e3806_d_n10;
        var_t4w_dn11 = assign5850_e3806_d_n11;
        var_t4w_dn12 = assign5850_e3806_d_n12;
        var_t4w_dn17 = assign5850_e3806_d_n17;
        var_t4w_rv = 0.0;

        let (assign5860_e3815, assign5860_e3815_d_n0, assign5860_e3815_d_n2, assign5860_e3815_d_n6, assign5860_e3815_d_n7, assign5860_e3815_d_n10, assign5860_e3815_d_n11, assign5860_e3815_d_n12, assign5860_e3815_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5860_e3811: f64 = (var_vfb - var_vthq);
        let assign5860_e3813: f64 = (assign5860_e3811 + p.p205);
        (assign5860_e3813, (-var_vthq_dn0), (-var_vthq_dn2), (-var_vthq_dn6), (-var_vthq_dn7), (-var_vthq_dn10), (-var_vthq_dn11), (-var_vthq_dn12), (-var_vthq_dn17),)
    } else {
        (var_t6__blk67, var_t6__blk67_dn0, var_t6__blk67_dn2, var_t6__blk67_dn6, var_t6__blk67_dn7, var_t6__blk67_dn10, var_t6__blk67_dn11, var_t6__blk67_dn12, var_t6__blk67_dn17,)
    }
};
        var_t6__blk67 = assign5860_e3815;
        var_t6__blk67_dn0 = assign5860_e3815_d_n0;
        var_t6__blk67_dn2 = assign5860_e3815_d_n2;
        var_t6__blk67_dn6 = assign5860_e3815_d_n6;
        var_t6__blk67_dn7 = assign5860_e3815_d_n7;
        var_t6__blk67_dn10 = assign5860_e3815_d_n10;
        var_t6__blk67_dn11 = assign5860_e3815_d_n11;
        var_t6__blk67_dn12 = assign5860_e3815_d_n12;
        var_t6__blk67_dn17 = assign5860_e3815_d_n17;
        var_t6__blk67_rv = 0.0;

        let (assign5870_e3825, assign5870_e3825_d_n0, assign5870_e3825_d_n2, assign5870_e3825_d_n6, assign5870_e3825_d_n7, assign5870_e3825_d_n10, assign5870_e3825_d_n11, assign5870_e3825_d_n12, assign5870_e3825_d_n17,) = {
    if (var_guard69 == 0.0) {
        let (assign5870_e3823, assign5870_e3823_d_n0, assign5870_e3823_d_n2, assign5870_e3823_d_n6, assign5870_e3823_d_n7, assign5870_e3823_d_n10, assign5870_e3823_d_n11, assign5870_e3823_d_n12, assign5870_e3823_d_n17,) = {
            if (var_t6__blk67 > var_t4w) {
                (var_t6__blk67, var_t6__blk67_dn0, var_t6__blk67_dn2, var_t6__blk67_dn6, var_t6__blk67_dn7, var_t6__blk67_dn10, var_t6__blk67_dn11, var_t6__blk67_dn12, var_t6__blk67_dn17,)
            } else {
                (var_t4w, var_t4w_dn0, var_t4w_dn2, var_t4w_dn6, var_t4w_dn7, var_t4w_dn10, var_t4w_dn11, var_t4w_dn12, var_t4w_dn17,)
            }
        };
        (assign5870_e3823, assign5870_e3823_d_n0, assign5870_e3823_d_n2, assign5870_e3823_d_n6, assign5870_e3823_d_n7, assign5870_e3823_d_n10, assign5870_e3823_d_n11, assign5870_e3823_d_n12, assign5870_e3823_d_n17,)
    } else {
        (var_t4__blk64, var_t4__blk64_dn0, var_t4__blk64_dn2, var_t4__blk64_dn6, var_t4__blk64_dn7, var_t4__blk64_dn10, var_t4__blk64_dn11, var_t4__blk64_dn12, var_t4__blk64_dn17,)
    }
};
        var_t4__blk64 = assign5870_e3825;
        var_t4__blk64_dn0 = assign5870_e3825_d_n0;
        var_t4__blk64_dn2 = assign5870_e3825_d_n2;
        var_t4__blk64_dn6 = assign5870_e3825_d_n6;
        var_t4__blk64_dn7 = assign5870_e3825_d_n7;
        var_t4__blk64_dn10 = assign5870_e3825_d_n10;
        var_t4__blk64_dn11 = assign5870_e3825_d_n11;
        var_t4__blk64_dn12 = assign5870_e3825_d_n12;
        var_t4__blk64_dn17 = assign5870_e3825_d_n17;
        var_t4__blk64_rv = 0.0;

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
        *var_guard68_slot = var_guard68;
        *var_guard68_rv_slot = var_guard68_rv;
        *var_guard69_slot = var_guard69;
        *var_guard69_rv_slot = var_guard69_rv;
        *var_guard70_slot = var_guard70;
        *var_guard70_rv_slot = var_guard70_rv;
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
        *var_t1__blk57_slot = var_t1__blk57;
        *var_t1__blk57_dn0_slot = var_t1__blk57_dn0;
        *var_t1__blk57_dn10_slot = var_t1__blk57_dn10;
        *var_t1__blk57_dn11_slot = var_t1__blk57_dn11;
        *var_t1__blk57_dn12_slot = var_t1__blk57_dn12;
        *var_t1__blk57_dn17_slot = var_t1__blk57_dn17;
        *var_t1__blk57_dn2_slot = var_t1__blk57_dn2;
        *var_t1__blk57_dn6_slot = var_t1__blk57_dn6;
        *var_t1__blk57_dn7_slot = var_t1__blk57_dn7;
        *var_t1__blk57_rv_slot = var_t1__blk57_rv;
        *var_t2__blk62_slot = var_t2__blk62;
        *var_t2__blk62_dn0_slot = var_t2__blk62_dn0;
        *var_t2__blk62_dn10_slot = var_t2__blk62_dn10;
        *var_t2__blk62_dn11_slot = var_t2__blk62_dn11;
        *var_t2__blk62_dn12_slot = var_t2__blk62_dn12;
        *var_t2__blk62_dn17_slot = var_t2__blk62_dn17;
        *var_t2__blk62_dn2_slot = var_t2__blk62_dn2;
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
        *var_t4__blk64_slot = var_t4__blk64;
        *var_t4__blk64_dn0_slot = var_t4__blk64_dn0;
        *var_t4__blk64_dn10_slot = var_t4__blk64_dn10;
        *var_t4__blk64_dn11_slot = var_t4__blk64_dn11;
        *var_t4__blk64_dn12_slot = var_t4__blk64_dn12;
        *var_t4__blk64_dn17_slot = var_t4__blk64_dn17;
        *var_t4__blk64_dn2_slot = var_t4__blk64_dn2;
        *var_t4__blk64_dn6_slot = var_t4__blk64_dn6;
        *var_t4__blk64_dn7_slot = var_t4__blk64_dn7;
        *var_t4__blk64_rv_slot = var_t4__blk64_rv;
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
        *var_t5__blk66_slot = var_t5__blk66;
        *var_t5__blk66_dn0_slot = var_t5__blk66_dn0;
        *var_t5__blk66_dn10_slot = var_t5__blk66_dn10;
        *var_t5__blk66_dn11_slot = var_t5__blk66_dn11;
        *var_t5__blk66_dn12_slot = var_t5__blk66_dn12;
        *var_t5__blk66_dn17_slot = var_t5__blk66_dn17;
        *var_t5__blk66_dn2_slot = var_t5__blk66_dn2;
        *var_t5__blk66_dn6_slot = var_t5__blk66_dn6;
        *var_t5__blk66_dn7_slot = var_t5__blk66_dn7;
        *var_t5__blk66_rv_slot = var_t5__blk66_rv;
        *var_t6__blk67_slot = var_t6__blk67;
        *var_t6__blk67_dn0_slot = var_t6__blk67_dn0;
        *var_t6__blk67_dn10_slot = var_t6__blk67_dn10;
        *var_t6__blk67_dn11_slot = var_t6__blk67_dn11;
        *var_t6__blk67_dn12_slot = var_t6__blk67_dn12;
        *var_t6__blk67_dn17_slot = var_t6__blk67_dn17;
        *var_t6__blk67_dn2_slot = var_t6__blk67_dn2;
        *var_t6__blk67_dn6_slot = var_t6__blk67_dn6;
        *var_t6__blk67_dn7_slot = var_t6__blk67_dn7;
        *var_t6__blk67_rv_slot = var_t6__blk67_rv;
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
        *var_tx__blk60_slot = var_tx__blk60;
        *var_tx__blk60_dn0_slot = var_tx__blk60_dn0;
        *var_tx__blk60_dn10_slot = var_tx__blk60_dn10;
        *var_tx__blk60_dn11_slot = var_tx__blk60_dn11;
        *var_tx__blk60_dn12_slot = var_tx__blk60_dn12;
        *var_tx__blk60_dn17_slot = var_tx__blk60_dn17;
        *var_tx__blk60_dn2_slot = var_tx__blk60_dn2;
        *var_tx__blk60_dn6_slot = var_tx__blk60_dn6;
        *var_tx__blk60_dn7_slot = var_tx__blk60_dn7;
        *var_tx__blk60_rv_slot = var_tx__blk60_rv;
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
        var_guard69: f64,
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
        var_t3__blk63: f64,
        var_t3__blk63_dn0: f64,
        var_t3__blk63_dn10: f64,
        var_t3__blk63_dn11: f64,
        var_t3__blk63_dn12: f64,
        var_t3__blk63_dn17: f64,
        var_t3__blk63_dn2: f64,
        var_t3__blk63_dn6: f64,
        var_t3__blk63_dn7: f64,
        var_t4__blk64: f64,
        var_t4__blk64_dn0: f64,
        var_t4__blk64_dn10: f64,
        var_t4__blk64_dn11: f64,
        var_t4__blk64_dn12: f64,
        var_t4__blk64_dn17: f64,
        var_t4__blk64_dn2: f64,
        var_t4__blk64_dn6: f64,
        var_t4__blk64_dn7: f64,
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
        var_guard71_slot: &mut f64,
        var_guard71_rv_slot: &mut f64,
        var_guard72_slot: &mut f64,
        var_guard72_rv_slot: &mut f64,
        var_guard73_slot: &mut f64,
        var_guard73_rv_slot: &mut f64,
        var_t2__blk62_slot: &mut f64,
        var_t2__blk62_dn0_slot: &mut f64,
        var_t2__blk62_dn10_slot: &mut f64,
        var_t2__blk62_dn11_slot: &mut f64,
        var_t2__blk62_dn12_slot: &mut f64,
        var_t2__blk62_dn17_slot: &mut f64,
        var_t2__blk62_dn2_slot: &mut f64,
        var_t2__blk62_dn6_slot: &mut f64,
        var_t2__blk62_dn7_slot: &mut f64,
        var_t2__blk62_rv_slot: &mut f64,
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
        let mut var_guard71: f64 = *var_guard71_slot;
        let mut var_guard71_rv: f64 = *var_guard71_rv_slot;
        let mut var_guard72: f64 = *var_guard72_slot;
        let mut var_guard72_rv: f64 = *var_guard72_rv_slot;
        let mut var_guard73: f64 = *var_guard73_slot;
        let mut var_guard73_rv: f64 = *var_guard73_rv_slot;
        let mut var_t2__blk62: f64 = *var_t2__blk62_slot;
        let mut var_t2__blk62_dn0: f64 = *var_t2__blk62_dn0_slot;
        let mut var_t2__blk62_dn10: f64 = *var_t2__blk62_dn10_slot;
        let mut var_t2__blk62_dn11: f64 = *var_t2__blk62_dn11_slot;
        let mut var_t2__blk62_dn12: f64 = *var_t2__blk62_dn12_slot;
        let mut var_t2__blk62_dn17: f64 = *var_t2__blk62_dn17_slot;
        let mut var_t2__blk62_dn2: f64 = *var_t2__blk62_dn2_slot;
        let mut var_t2__blk62_dn6: f64 = *var_t2__blk62_dn6_slot;
        let mut var_t2__blk62_dn7: f64 = *var_t2__blk62_dn7_slot;
        let mut var_t2__blk62_rv: f64 = *var_t2__blk62_rv_slot;
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

        let (assign5880_e3836, assign5880_e3836_d_n0, assign5880_e3836_d_n2, assign5880_e3836_d_n6, assign5880_e3836_d_n7, assign5880_e3836_d_n10, assign5880_e3836_d_n11, assign5880_e3836_d_n12, assign5880_e3836_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5880_e3830: f64 = (1.0 / var_t4__blk64);
        let assign5880_e3832: f64 = (assign5880_e3830 - var_t3__blk63);
        let assign5880_e3834: f64 = (assign5880_e3832 - 0.0001);
        (assign5880_e3834, ((-(var_t4__blk64_dn0 / (var_t4__blk64 * var_t4__blk64))) - var_t3__blk63_dn0), ((-(var_t4__blk64_dn2 / (var_t4__blk64 * var_t4__blk64))) - var_t3__blk63_dn2), ((-(var_t4__blk64_dn6 / (var_t4__blk64 * var_t4__blk64))) - var_t3__blk63_dn6), ((-(var_t4__blk64_dn7 / (var_t4__blk64 * var_t4__blk64))) - var_t3__blk63_dn7), ((-(var_t4__blk64_dn10 / (var_t4__blk64 * var_t4__blk64))) - var_t3__blk63_dn10), ((-(var_t4__blk64_dn11 / (var_t4__blk64 * var_t4__blk64))) - var_t3__blk63_dn11), ((-(var_t4__blk64_dn12 / (var_t4__blk64 * var_t4__blk64))) - var_t3__blk63_dn12), ((-(var_t4__blk64_dn17 / (var_t4__blk64 * var_t4__blk64))) - var_t3__blk63_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign5880_e3836;
        var_tmf1_dn0 = assign5880_e3836_d_n0;
        var_tmf1_dn2 = assign5880_e3836_d_n2;
        var_tmf1_dn6 = assign5880_e3836_d_n6;
        var_tmf1_dn7 = assign5880_e3836_d_n7;
        var_tmf1_dn10 = assign5880_e3836_d_n10;
        var_tmf1_dn11 = assign5880_e3836_d_n11;
        var_tmf1_dn12 = assign5880_e3836_d_n12;
        var_tmf1_dn17 = assign5880_e3836_d_n17;
        var_tmf1_rv = 0.0;

        let (assign5890_e3847, assign5890_e3847_d_n0, assign5890_e3847_d_n2, assign5890_e3847_d_n6, assign5890_e3847_d_n7, assign5890_e3847_d_n10, assign5890_e3847_d_n11, assign5890_e3847_d_n12, assign5890_e3847_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5890_e3842: f64 = (1.0 / var_t4__blk64);
        let assign5890_e3843: f64 = (4.0 * assign5890_e3842);
        let assign5890_e3845: f64 = (assign5890_e3843 * 0.0001);
        (assign5890_e3845, ((4.0 * (-(var_t4__blk64_dn0 / (var_t4__blk64 * var_t4__blk64)))) * 0.0001), ((4.0 * (-(var_t4__blk64_dn2 / (var_t4__blk64 * var_t4__blk64)))) * 0.0001), ((4.0 * (-(var_t4__blk64_dn6 / (var_t4__blk64 * var_t4__blk64)))) * 0.0001), ((4.0 * (-(var_t4__blk64_dn7 / (var_t4__blk64 * var_t4__blk64)))) * 0.0001), ((4.0 * (-(var_t4__blk64_dn10 / (var_t4__blk64 * var_t4__blk64)))) * 0.0001), ((4.0 * (-(var_t4__blk64_dn11 / (var_t4__blk64 * var_t4__blk64)))) * 0.0001), ((4.0 * (-(var_t4__blk64_dn12 / (var_t4__blk64 * var_t4__blk64)))) * 0.0001), ((4.0 * (-(var_t4__blk64_dn17 / (var_t4__blk64 * var_t4__blk64)))) * 0.0001),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign5890_e3847;
        var_tmf2_dn0 = assign5890_e3847_d_n0;
        var_tmf2_dn2 = assign5890_e3847_d_n2;
        var_tmf2_dn6 = assign5890_e3847_d_n6;
        var_tmf2_dn7 = assign5890_e3847_d_n7;
        var_tmf2_dn10 = assign5890_e3847_d_n10;
        var_tmf2_dn11 = assign5890_e3847_d_n11;
        var_tmf2_dn12 = assign5890_e3847_d_n12;
        var_tmf2_dn17 = assign5890_e3847_d_n17;
        var_tmf2_rv = 0.0;

        let (assign5900_e3858, assign5900_e3858_d_n0, assign5900_e3858_d_n2, assign5900_e3858_d_n6, assign5900_e3858_d_n7, assign5900_e3858_d_n10, assign5900_e3858_d_n11, assign5900_e3858_d_n12, assign5900_e3858_d_n17,) = {
    if (var_guard69 == 0.0) {
        let (assign5900_e3856, assign5900_e3856_d_n0, assign5900_e3856_d_n2, assign5900_e3856_d_n6, assign5900_e3856_d_n7, assign5900_e3856_d_n10, assign5900_e3856_d_n11, assign5900_e3856_d_n12, assign5900_e3856_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign5900_e3855: f64 = (-var_tmf2);
                (assign5900_e3855, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign5900_e3856, assign5900_e3856_d_n0, assign5900_e3856_d_n2, assign5900_e3856_d_n6, assign5900_e3856_d_n7, assign5900_e3856_d_n10, assign5900_e3856_d_n11, assign5900_e3856_d_n12, assign5900_e3856_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign5900_e3858;
        var_tmf2_dn0 = assign5900_e3858_d_n0;
        var_tmf2_dn2 = assign5900_e3858_d_n2;
        var_tmf2_dn6 = assign5900_e3858_d_n6;
        var_tmf2_dn7 = assign5900_e3858_d_n7;
        var_tmf2_dn10 = assign5900_e3858_d_n10;
        var_tmf2_dn11 = assign5900_e3858_d_n11;
        var_tmf2_dn12 = assign5900_e3858_d_n12;
        var_tmf2_dn17 = assign5900_e3858_d_n17;
        var_tmf2_rv = 0.0;

        let (assign5910_e3868, assign5910_e3868_d_n0, assign5910_e3868_d_n2, assign5910_e3868_d_n6, assign5910_e3868_d_n7, assign5910_e3868_d_n10, assign5910_e3868_d_n11, assign5910_e3868_d_n12, assign5910_e3868_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5910_e3863: f64 = (var_tmf1 * var_tmf1);
        let assign5910_e3865: f64 = (assign5910_e3863 + var_tmf2);
        let assign5910_e3866: f64 = (assign5910_e3865).sqrt();
        (assign5910_e3866, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5910_e3866)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5910_e3866)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign5910_e3866)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign5910_e3866)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign5910_e3866)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign5910_e3866)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign5910_e3866)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign5910_e3866)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign5910_e3868;
        var_tmf2_dn0 = assign5910_e3868_d_n0;
        var_tmf2_dn2 = assign5910_e3868_d_n2;
        var_tmf2_dn6 = assign5910_e3868_d_n6;
        var_tmf2_dn7 = assign5910_e3868_d_n7;
        var_tmf2_dn10 = assign5910_e3868_d_n10;
        var_tmf2_dn11 = assign5910_e3868_d_n11;
        var_tmf2_dn12 = assign5910_e3868_d_n12;
        var_tmf2_dn17 = assign5910_e3868_d_n17;
        var_tmf2_rv = 0.0;

        let (assign5920_e3881, assign5920_e3881_d_n0, assign5920_e3881_d_n2, assign5920_e3881_d_n6, assign5920_e3881_d_n7, assign5920_e3881_d_n10, assign5920_e3881_d_n11, assign5920_e3881_d_n12, assign5920_e3881_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5920_e3873: f64 = (1.0 / var_t4__blk64);
        let assign5920_e3877: f64 = (var_tmf1 + var_tmf2);
        let assign5920_e3878: f64 = (0.5 * assign5920_e3877);
        let assign5920_e3879: f64 = (assign5920_e3873 - assign5920_e3878);
        (assign5920_e3879, ((-(var_t4__blk64_dn0 / (var_t4__blk64 * var_t4__blk64))) - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), ((-(var_t4__blk64_dn2 / (var_t4__blk64 * var_t4__blk64))) - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), ((-(var_t4__blk64_dn6 / (var_t4__blk64 * var_t4__blk64))) - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), ((-(var_t4__blk64_dn7 / (var_t4__blk64 * var_t4__blk64))) - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), ((-(var_t4__blk64_dn10 / (var_t4__blk64 * var_t4__blk64))) - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), ((-(var_t4__blk64_dn11 / (var_t4__blk64 * var_t4__blk64))) - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), ((-(var_t4__blk64_dn12 / (var_t4__blk64 * var_t4__blk64))) - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), ((-(var_t4__blk64_dn17 / (var_t4__blk64 * var_t4__blk64))) - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_t2__blk62, var_t2__blk62_dn0, var_t2__blk62_dn2, var_t2__blk62_dn6, var_t2__blk62_dn7, var_t2__blk62_dn10, var_t2__blk62_dn11, var_t2__blk62_dn12, var_t2__blk62_dn17,)
    }
};
        var_t2__blk62 = assign5920_e3881;
        var_t2__blk62_dn0 = assign5920_e3881_d_n0;
        var_t2__blk62_dn2 = assign5920_e3881_d_n2;
        var_t2__blk62_dn6 = assign5920_e3881_d_n6;
        var_t2__blk62_dn7 = assign5920_e3881_d_n7;
        var_t2__blk62_dn10 = assign5920_e3881_d_n10;
        var_t2__blk62_dn11 = assign5920_e3881_d_n11;
        var_t2__blk62_dn12 = assign5920_e3881_d_n12;
        var_t2__blk62_dn17 = assign5920_e3881_d_n17;
        var_t2__blk62_rv = 0.0;

        let (assign5930_e3890, assign5930_e3890_d_n0, assign5930_e3890_d_n2, assign5930_e3890_d_n6, assign5930_e3890_d_n7, assign5930_e3890_d_n10, assign5930_e3890_d_n11, assign5930_e3890_d_n12, assign5930_e3890_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5930_e3886: f64 = (p.p204 * var_t2__blk62);
        let assign5930_e3888: f64 = (assign5930_e3886 + p.p206);
        (assign5930_e3888, (p.p204 * var_t2__blk62_dn0), (p.p204 * var_t2__blk62_dn2), (p.p204 * var_t2__blk62_dn6), (p.p204 * var_t2__blk62_dn7), (p.p204 * var_t2__blk62_dn10), (p.p204 * var_t2__blk62_dn11), (p.p204 * var_t2__blk62_dn12), (p.p204 * var_t2__blk62_dn17),)
    } else {
        (var_dtfox, var_dtfox_dn0, var_dtfox_dn2, var_dtfox_dn6, var_dtfox_dn7, var_dtfox_dn10, var_dtfox_dn11, var_dtfox_dn12, var_dtfox_dn17,)
    }
};
        var_dtfox = assign5930_e3890;
        var_dtfox_dn0 = assign5930_e3890_d_n0;
        var_dtfox_dn2 = assign5930_e3890_d_n2;
        var_dtfox_dn6 = assign5930_e3890_d_n6;
        var_dtfox_dn7 = assign5930_e3890_d_n7;
        var_dtfox_dn10 = assign5930_e3890_d_n10;
        var_dtfox_dn11 = assign5930_e3890_d_n11;
        var_dtfox_dn12 = assign5930_e3890_d_n12;
        var_dtfox_dn17 = assign5930_e3890_d_n17;
        var_dtfox_rv = 0.0;

        let assign5940_e3893: f64 = (var_dtfox * 1000000000000.0);
        let assign5940_e3895: f64 = if assign5940_e3893 < var_tfox0 { 1.0 } else { 0.0 };
        var_guard71 = assign5940_e3895;
        var_guard71_rv = 0.0;

        let (assign5950_e3902, assign5950_e3902_d_n0, assign5950_e3902_d_n2, assign5950_e3902_d_n6, assign5950_e3902_d_n7, assign5950_e3902_d_n10, assign5950_e3902_d_n11, assign5950_e3902_d_n12, assign5950_e3902_d_n17,) = {
    if ((var_guard69 == 0.0) && (var_guard71 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dtfox, var_dtfox_dn0, var_dtfox_dn2, var_dtfox_dn6, var_dtfox_dn7, var_dtfox_dn10, var_dtfox_dn11, var_dtfox_dn12, var_dtfox_dn17,)
    }
};
        var_dtfox = assign5950_e3902;
        var_dtfox_dn0 = assign5950_e3902_d_n0;
        var_dtfox_dn2 = assign5950_e3902_d_n2;
        var_dtfox_dn6 = assign5950_e3902_d_n6;
        var_dtfox_dn7 = assign5950_e3902_d_n7;
        var_dtfox_dn10 = assign5950_e3902_d_n10;
        var_dtfox_dn11 = assign5950_e3902_d_n11;
        var_dtfox_dn12 = assign5950_e3902_d_n12;
        var_dtfox_dn17 = assign5950_e3902_d_n17;
        var_dtfox_rv = 0.0;

        let (assign5960_e3909,) = {
    if ((var_guard69 == 0.0) && (var_guard71 != 0.0)) {
        (0.0,)
    } else {
        (var_flg_qme,)
    }
};
        var_flg_qme = assign5960_e3909;
        var_flg_qme_rv = 0.0;

        let (assign5970_e3916, assign5970_e3916_d_n0, assign5970_e3916_d_n2, assign5970_e3916_d_n6, assign5970_e3916_d_n7, assign5970_e3916_d_n10, assign5970_e3916_d_n11, assign5970_e3916_d_n12, assign5970_e3916_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5970_e3914: f64 = (var_tfox0 + var_dtfox);
        (assign5970_e3914, var_dtfox_dn0, var_dtfox_dn2, var_dtfox_dn6, var_dtfox_dn7, var_dtfox_dn10, var_dtfox_dn11, var_dtfox_dn12, var_dtfox_dn17,)
    } else {
        (var_tfoxe, var_tfoxe_dn0, var_tfoxe_dn2, var_tfoxe_dn6, var_tfoxe_dn7, var_tfoxe_dn10, var_tfoxe_dn11, var_tfoxe_dn12, var_tfoxe_dn17,)
    }
};
        var_tfoxe = assign5970_e3916;
        var_tfoxe_dn0 = assign5970_e3916_d_n0;
        var_tfoxe_dn2 = assign5970_e3916_d_n2;
        var_tfoxe_dn6 = assign5970_e3916_d_n6;
        var_tfoxe_dn7 = assign5970_e3916_d_n7;
        var_tfoxe_dn10 = assign5970_e3916_d_n10;
        var_tfoxe_dn11 = assign5970_e3916_d_n11;
        var_tfoxe_dn12 = assign5970_e3916_d_n12;
        var_tfoxe_dn17 = assign5970_e3916_d_n17;
        var_tfoxe_rv = 0.0;

        let (assign5980_e3923, assign5980_e3923_d_n0, assign5980_e3923_d_n2, assign5980_e3923_d_n6, assign5980_e3923_d_n7, assign5980_e3923_d_n10, assign5980_e3923_d_n11, assign5980_e3923_d_n12, assign5980_e3923_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5980_e3921: f64 = (3.453133e-11 / var_tfoxe);
        (assign5980_e3921, (-((3.453133e-11 * var_tfoxe_dn0) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn2) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn6) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn7) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn10) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn11) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn12) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn17) / (var_tfoxe * var_tfoxe))),)
    } else {
        (var_c_fox, var_c_fox_dn0, var_c_fox_dn2, var_c_fox_dn6, var_c_fox_dn7, var_c_fox_dn10, var_c_fox_dn11, var_c_fox_dn12, var_c_fox_dn17,)
    }
};
        var_c_fox = assign5980_e3923;
        var_c_fox_dn0 = assign5980_e3923_d_n0;
        var_c_fox_dn2 = assign5980_e3923_d_n2;
        var_c_fox_dn6 = assign5980_e3923_d_n6;
        var_c_fox_dn7 = assign5980_e3923_d_n7;
        var_c_fox_dn10 = assign5980_e3923_d_n10;
        var_c_fox_dn11 = assign5980_e3923_d_n11;
        var_c_fox_dn12 = assign5980_e3923_d_n12;
        var_c_fox_dn17 = assign5980_e3923_d_n17;
        var_c_fox_rv = 0.0;

        let (assign5990_e3930, assign5990_e3930_d_n0, assign5990_e3930_d_n2, assign5990_e3930_d_n6, assign5990_e3930_d_n7, assign5990_e3930_d_n10, assign5990_e3930_d_n11, assign5990_e3930_d_n12, assign5990_e3930_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign5990_e3928: f64 = (var_tfoxe / 3.453133e-11);
        (assign5990_e3928, (var_tfoxe_dn0 / 3.453133e-11), (var_tfoxe_dn2 / 3.453133e-11), (var_tfoxe_dn6 / 3.453133e-11), (var_tfoxe_dn7 / 3.453133e-11), (var_tfoxe_dn10 / 3.453133e-11), (var_tfoxe_dn11 / 3.453133e-11), (var_tfoxe_dn12 / 3.453133e-11), (var_tfoxe_dn17 / 3.453133e-11),)
    } else {
        (var_c_fox_inv, var_c_fox_inv_dn0, var_c_fox_inv_dn2, var_c_fox_inv_dn6, var_c_fox_inv_dn7, var_c_fox_inv_dn10, var_c_fox_inv_dn11, var_c_fox_inv_dn12, var_c_fox_inv_dn17,)
    }
};
        var_c_fox_inv = assign5990_e3930;
        var_c_fox_inv_dn0 = assign5990_e3930_d_n0;
        var_c_fox_inv_dn2 = assign5990_e3930_d_n2;
        var_c_fox_inv_dn6 = assign5990_e3930_d_n6;
        var_c_fox_inv_dn7 = assign5990_e3930_d_n7;
        var_c_fox_inv_dn10 = assign5990_e3930_d_n10;
        var_c_fox_inv_dn11 = assign5990_e3930_d_n11;
        var_c_fox_inv_dn12 = assign5990_e3930_d_n12;
        var_c_fox_inv_dn17 = assign5990_e3930_d_n17;
        var_c_fox_inv_rv = 0.0;

        let (assign6000_e3941, assign6000_e3941_d_n0, assign6000_e3941_d_n2, assign6000_e3941_d_n6, assign6000_e3941_d_n7, assign6000_e3941_d_n10, assign6000_e3941_d_n11, assign6000_e3941_d_n12, assign6000_e3941_d_n17,) = {
    if (var_guard69 == 0.0) {
        let assign6000_e3935: f64 = (var_cnst0soi * var_cnst0soi);
        let assign6000_e3937: f64 = (assign6000_e3935 * var_c_fox_inv);
        let assign6000_e3939: f64 = (assign6000_e3937 * var_c_fox_inv);
        (assign6000_e3939, ((((((var_cnst0soi_dn0 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn0)) * var_c_fox_inv) + (assign6000_e3935 * var_c_fox_inv_dn0)) * var_c_fox_inv) + (assign6000_e3937 * var_c_fox_inv_dn0)), ((((((var_cnst0soi_dn2 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn2)) * var_c_fox_inv) + (assign6000_e3935 * var_c_fox_inv_dn2)) * var_c_fox_inv) + (assign6000_e3937 * var_c_fox_inv_dn2)), ((((((var_cnst0soi_dn6 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn6)) * var_c_fox_inv) + (assign6000_e3935 * var_c_fox_inv_dn6)) * var_c_fox_inv) + (assign6000_e3937 * var_c_fox_inv_dn6)), ((((((var_cnst0soi_dn7 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn7)) * var_c_fox_inv) + (assign6000_e3935 * var_c_fox_inv_dn7)) * var_c_fox_inv) + (assign6000_e3937 * var_c_fox_inv_dn7)), ((((((var_cnst0soi_dn10 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn10)) * var_c_fox_inv) + (assign6000_e3935 * var_c_fox_inv_dn10)) * var_c_fox_inv) + (assign6000_e3937 * var_c_fox_inv_dn10)), ((((((var_cnst0soi_dn11 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn11)) * var_c_fox_inv) + (assign6000_e3935 * var_c_fox_inv_dn11)) * var_c_fox_inv) + (assign6000_e3937 * var_c_fox_inv_dn11)), ((((((var_cnst0soi_dn12 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn12)) * var_c_fox_inv) + (assign6000_e3935 * var_c_fox_inv_dn12)) * var_c_fox_inv) + (assign6000_e3937 * var_c_fox_inv_dn12)), ((((((var_cnst0soi_dn17 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn17)) * var_c_fox_inv) + (assign6000_e3935 * var_c_fox_inv_dn17)) * var_c_fox_inv) + (assign6000_e3937 * var_c_fox_inv_dn17)),)
    } else {
        (var_cnstc_foxi, var_cnstc_foxi_dn0, var_cnstc_foxi_dn2, var_cnstc_foxi_dn6, var_cnstc_foxi_dn7, var_cnstc_foxi_dn10, var_cnstc_foxi_dn11, var_cnstc_foxi_dn12, var_cnstc_foxi_dn17,)
    }
};
        var_cnstc_foxi = assign6000_e3941;
        var_cnstc_foxi_dn0 = assign6000_e3941_d_n0;
        var_cnstc_foxi_dn2 = assign6000_e3941_d_n2;
        var_cnstc_foxi_dn6 = assign6000_e3941_d_n6;
        var_cnstc_foxi_dn7 = assign6000_e3941_d_n7;
        var_cnstc_foxi_dn10 = assign6000_e3941_d_n10;
        var_cnstc_foxi_dn11 = assign6000_e3941_d_n11;
        var_cnstc_foxi_dn12 = assign6000_e3941_d_n12;
        var_cnstc_foxi_dn17 = assign6000_e3941_d_n17;
        var_cnstc_foxi_rv = 0.0;

        let assign6010_e3948: f64 = if ((p.p43 == 1.0) || (var_subversion < 3.0)) { 1.0 } else { 0.0 };
        var_guard72 = assign6010_e3948;
        var_guard72_rv = 0.0;

        let (assign6020_e3956, assign6020_e3956_d_n0, assign6020_e3956_d_n2, assign6020_e3956_d_n6, assign6020_e3956_d_n7, assign6020_e3956_d_n10, assign6020_e3956_d_n11, assign6020_e3956_d_n12, assign6020_e3956_d_n17,) = {
    if (var_guard72 != 0.0) {
        let assign6020_e3952: f64 = (0.5 - var_vbspz);
        let assign6020_e3954: f64 = (assign6020_e3952 - 0.001);
        (assign6020_e3954, (-var_vbspz_dn0), (-var_vbspz_dn2), (-var_vbspz_dn6), (-var_vbspz_dn7), (-var_vbspz_dn10), (-var_vbspz_dn11), (-var_vbspz_dn12), (-var_vbspz_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign6020_e3956;
        var_tmf1_dn0 = assign6020_e3956_d_n0;
        var_tmf1_dn2 = assign6020_e3956_d_n2;
        var_tmf1_dn6 = assign6020_e3956_d_n6;
        var_tmf1_dn7 = assign6020_e3956_d_n7;
        var_tmf1_dn10 = assign6020_e3956_d_n10;
        var_tmf1_dn11 = assign6020_e3956_d_n11;
        var_tmf1_dn12 = assign6020_e3956_d_n12;
        var_tmf1_dn17 = assign6020_e3956_d_n17;
        var_tmf1_rv = 0.0;

        let (assign6030_e3964, assign6030_e3964_d_n0, assign6030_e3964_d_n2, assign6030_e3964_d_n6, assign6030_e3964_d_n7, assign6030_e3964_d_n10, assign6030_e3964_d_n11, assign6030_e3964_d_n12, assign6030_e3964_d_n17,) = {
    if (var_guard72 != 0.0) {
        let assign6030_e3960: f64 = (4.0 * 0.5);
        let assign6030_e3962: f64 = (assign6030_e3960 * 0.001);
        (assign6030_e3962, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6030_e3964;
        var_tmf2_dn0 = assign6030_e3964_d_n0;
        var_tmf2_dn2 = assign6030_e3964_d_n2;
        var_tmf2_dn6 = assign6030_e3964_d_n6;
        var_tmf2_dn7 = assign6030_e3964_d_n7;
        var_tmf2_dn10 = assign6030_e3964_d_n10;
        var_tmf2_dn11 = assign6030_e3964_d_n11;
        var_tmf2_dn12 = assign6030_e3964_d_n12;
        var_tmf2_dn17 = assign6030_e3964_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6040_e3974, assign6040_e3974_d_n0, assign6040_e3974_d_n2, assign6040_e3974_d_n6, assign6040_e3974_d_n7, assign6040_e3974_d_n10, assign6040_e3974_d_n11, assign6040_e3974_d_n12, assign6040_e3974_d_n17,) = {
    if (var_guard72 != 0.0) {
        let (assign6040_e3972, assign6040_e3972_d_n0, assign6040_e3972_d_n2, assign6040_e3972_d_n6, assign6040_e3972_d_n7, assign6040_e3972_d_n10, assign6040_e3972_d_n11, assign6040_e3972_d_n12, assign6040_e3972_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign6040_e3971: f64 = (-var_tmf2);
                (assign6040_e3971, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign6040_e3972, assign6040_e3972_d_n0, assign6040_e3972_d_n2, assign6040_e3972_d_n6, assign6040_e3972_d_n7, assign6040_e3972_d_n10, assign6040_e3972_d_n11, assign6040_e3972_d_n12, assign6040_e3972_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6040_e3974;
        var_tmf2_dn0 = assign6040_e3974_d_n0;
        var_tmf2_dn2 = assign6040_e3974_d_n2;
        var_tmf2_dn6 = assign6040_e3974_d_n6;
        var_tmf2_dn7 = assign6040_e3974_d_n7;
        var_tmf2_dn10 = assign6040_e3974_d_n10;
        var_tmf2_dn11 = assign6040_e3974_d_n11;
        var_tmf2_dn12 = assign6040_e3974_d_n12;
        var_tmf2_dn17 = assign6040_e3974_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6050_e3983, assign6050_e3983_d_n0, assign6050_e3983_d_n2, assign6050_e3983_d_n6, assign6050_e3983_d_n7, assign6050_e3983_d_n10, assign6050_e3983_d_n11, assign6050_e3983_d_n12, assign6050_e3983_d_n17,) = {
    if (var_guard72 != 0.0) {
        let assign6050_e3978: f64 = (var_tmf1 * var_tmf1);
        let assign6050_e3980: f64 = (assign6050_e3978 + var_tmf2);
        let assign6050_e3981: f64 = (assign6050_e3980).sqrt();
        (assign6050_e3981, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign6050_e3981)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign6050_e3981)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign6050_e3981)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign6050_e3981)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign6050_e3981)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign6050_e3981)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign6050_e3981)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign6050_e3981)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6050_e3983;
        var_tmf2_dn0 = assign6050_e3983_d_n0;
        var_tmf2_dn2 = assign6050_e3983_d_n2;
        var_tmf2_dn6 = assign6050_e3983_d_n6;
        var_tmf2_dn7 = assign6050_e3983_d_n7;
        var_tmf2_dn10 = assign6050_e3983_d_n10;
        var_tmf2_dn11 = assign6050_e3983_d_n11;
        var_tmf2_dn12 = assign6050_e3983_d_n12;
        var_tmf2_dn17 = assign6050_e3983_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6060_e3993, assign6060_e3993_d_n0, assign6060_e3993_d_n2, assign6060_e3993_d_n6, assign6060_e3993_d_n7, assign6060_e3993_d_n10, assign6060_e3993_d_n11, assign6060_e3993_d_n12, assign6060_e3993_d_n17,) = {
    if (var_guard72 != 0.0) {
        let assign6060_e3989: f64 = (var_tmf1 + var_tmf2);
        let assign6060_e3990: f64 = (0.5 * assign6060_e3989);
        let assign6060_e3991: f64 = (0.5 - assign6060_e3990);
        (assign6060_e3991, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (-(0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (-(0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (-(0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (-(0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), (-(0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_vbsz2, var_vbsz2_dn0, var_vbsz2_dn2, var_vbsz2_dn6, var_vbsz2_dn7, var_vbsz2_dn10, var_vbsz2_dn11, var_vbsz2_dn12, var_vbsz2_dn17,)
    }
};
        var_vbsz2 = assign6060_e3993;
        var_vbsz2_dn0 = assign6060_e3993_d_n0;
        var_vbsz2_dn2 = assign6060_e3993_d_n2;
        var_vbsz2_dn6 = assign6060_e3993_d_n6;
        var_vbsz2_dn7 = assign6060_e3993_d_n7;
        var_vbsz2_dn10 = assign6060_e3993_d_n10;
        var_vbsz2_dn11 = assign6060_e3993_d_n11;
        var_vbsz2_dn12 = assign6060_e3993_d_n12;
        var_vbsz2_dn17 = assign6060_e3993_d_n17;
        var_vbsz2_rv = 0.0;

        let (assign6070_e4010, assign6070_e4010_d_n0, assign6070_e4010_d_n2, assign6070_e4010_d_n6, assign6070_e4010_d_n7, assign6070_e4010_d_n10, assign6070_e4010_d_n11, assign6070_e4010_d_n12, assign6070_e4010_d_n17,) = {
    if (var_guard72 != 0.0) {
        let assign6070_e3996: f64 = (-p.p237);
        let assign6070_e3998: f64 = (assign6070_e3996 * p.p237);
        let assign6070_e4000: f64 = (assign6070_e3998 * var_q_nsub);
        let assign6070_e4003: f64 = (2.0 * 1.034943e-10);
        let assign6070_e4004: f64 = (assign6070_e4000 / assign6070_e4003);
        let assign6070_e4006: f64 = (assign6070_e4004 + var_pb2);
        let assign6070_e4008: f64 = (assign6070_e4006 - var_beta_inv);
        (assign6070_e4008, (((assign6070_e3998 * var_q_nsub_dn0) / assign6070_e4003) + var_pb2_dn0), (((assign6070_e3998 * var_q_nsub_dn2) / assign6070_e4003) + var_pb2_dn2), (((assign6070_e3998 * var_q_nsub_dn6) / assign6070_e4003) + var_pb2_dn6), (((assign6070_e3998 * var_q_nsub_dn7) / assign6070_e4003) + var_pb2_dn7), ((((assign6070_e3998 * var_q_nsub_dn10) / assign6070_e4003) + var_pb2_dn10) - var_beta_inv_dn10), (((assign6070_e3998 * var_q_nsub_dn11) / assign6070_e4003) + var_pb2_dn11), (((assign6070_e3998 * var_q_nsub_dn12) / assign6070_e4003) + var_pb2_dn12), (((assign6070_e3998 * var_q_nsub_dn17) / assign6070_e4003) + var_pb2_dn17),)
    } else {
        (var_vbslim, var_vbslim_dn0, var_vbslim_dn2, var_vbslim_dn6, var_vbslim_dn7, var_vbslim_dn10, var_vbslim_dn11, var_vbslim_dn12, var_vbslim_dn17,)
    }
};
        var_vbslim = assign6070_e4010;
        var_vbslim_dn0 = assign6070_e4010_d_n0;
        var_vbslim_dn2 = assign6070_e4010_d_n2;
        var_vbslim_dn6 = assign6070_e4010_d_n6;
        var_vbslim_dn7 = assign6070_e4010_d_n7;
        var_vbslim_dn10 = assign6070_e4010_d_n10;
        var_vbslim_dn11 = assign6070_e4010_d_n11;
        var_vbslim_dn12 = assign6070_e4010_d_n12;
        var_vbslim_dn17 = assign6070_e4010_d_n17;
        var_vbslim_rv = 0.0;

        let (assign6080_e4018, assign6080_e4018_d_n0, assign6080_e4018_d_n2, assign6080_e4018_d_n6, assign6080_e4018_d_n7, assign6080_e4018_d_n10, assign6080_e4018_d_n11, assign6080_e4018_d_n12, assign6080_e4018_d_n17,) = {
    if (var_guard72 != 0.0) {
        let assign6080_e4014: f64 = (var_vbsz2 - var_vbslim);
        let assign6080_e4016: f64 = (assign6080_e4014 - 0.001);
        (assign6080_e4016, (var_vbsz2_dn0 - var_vbslim_dn0), (var_vbsz2_dn2 - var_vbslim_dn2), (var_vbsz2_dn6 - var_vbslim_dn6), (var_vbsz2_dn7 - var_vbslim_dn7), (var_vbsz2_dn10 - var_vbslim_dn10), (var_vbsz2_dn11 - var_vbslim_dn11), (var_vbsz2_dn12 - var_vbslim_dn12), (var_vbsz2_dn17 - var_vbslim_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign6080_e4018;
        var_tmf1_dn0 = assign6080_e4018_d_n0;
        var_tmf1_dn2 = assign6080_e4018_d_n2;
        var_tmf1_dn6 = assign6080_e4018_d_n6;
        var_tmf1_dn7 = assign6080_e4018_d_n7;
        var_tmf1_dn10 = assign6080_e4018_d_n10;
        var_tmf1_dn11 = assign6080_e4018_d_n11;
        var_tmf1_dn12 = assign6080_e4018_d_n12;
        var_tmf1_dn17 = assign6080_e4018_d_n17;
        var_tmf1_rv = 0.0;

        let (assign6090_e4026, assign6090_e4026_d_n0, assign6090_e4026_d_n2, assign6090_e4026_d_n6, assign6090_e4026_d_n7, assign6090_e4026_d_n10, assign6090_e4026_d_n11, assign6090_e4026_d_n12, assign6090_e4026_d_n17,) = {
    if (var_guard72 != 0.0) {
        let assign6090_e4022: f64 = (4.0 * var_vbslim);
        let assign6090_e4024: f64 = (assign6090_e4022 * 0.001);
        (assign6090_e4024, ((4.0 * var_vbslim_dn0) * 0.001), ((4.0 * var_vbslim_dn2) * 0.001), ((4.0 * var_vbslim_dn6) * 0.001), ((4.0 * var_vbslim_dn7) * 0.001), ((4.0 * var_vbslim_dn10) * 0.001), ((4.0 * var_vbslim_dn11) * 0.001), ((4.0 * var_vbslim_dn12) * 0.001), ((4.0 * var_vbslim_dn17) * 0.001),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6090_e4026;
        var_tmf2_dn0 = assign6090_e4026_d_n0;
        var_tmf2_dn2 = assign6090_e4026_d_n2;
        var_tmf2_dn6 = assign6090_e4026_d_n6;
        var_tmf2_dn7 = assign6090_e4026_d_n7;
        var_tmf2_dn10 = assign6090_e4026_d_n10;
        var_tmf2_dn11 = assign6090_e4026_d_n11;
        var_tmf2_dn12 = assign6090_e4026_d_n12;
        var_tmf2_dn17 = assign6090_e4026_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6100_e4036, assign6100_e4036_d_n0, assign6100_e4036_d_n2, assign6100_e4036_d_n6, assign6100_e4036_d_n7, assign6100_e4036_d_n10, assign6100_e4036_d_n11, assign6100_e4036_d_n12, assign6100_e4036_d_n17,) = {
    if (var_guard72 != 0.0) {
        let (assign6100_e4034, assign6100_e4034_d_n0, assign6100_e4034_d_n2, assign6100_e4034_d_n6, assign6100_e4034_d_n7, assign6100_e4034_d_n10, assign6100_e4034_d_n11, assign6100_e4034_d_n12, assign6100_e4034_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign6100_e4033: f64 = (-var_tmf2);
                (assign6100_e4033, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign6100_e4034, assign6100_e4034_d_n0, assign6100_e4034_d_n2, assign6100_e4034_d_n6, assign6100_e4034_d_n7, assign6100_e4034_d_n10, assign6100_e4034_d_n11, assign6100_e4034_d_n12, assign6100_e4034_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6100_e4036;
        var_tmf2_dn0 = assign6100_e4036_d_n0;
        var_tmf2_dn2 = assign6100_e4036_d_n2;
        var_tmf2_dn6 = assign6100_e4036_d_n6;
        var_tmf2_dn7 = assign6100_e4036_d_n7;
        var_tmf2_dn10 = assign6100_e4036_d_n10;
        var_tmf2_dn11 = assign6100_e4036_d_n11;
        var_tmf2_dn12 = assign6100_e4036_d_n12;
        var_tmf2_dn17 = assign6100_e4036_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6110_e4045, assign6110_e4045_d_n0, assign6110_e4045_d_n2, assign6110_e4045_d_n6, assign6110_e4045_d_n7, assign6110_e4045_d_n10, assign6110_e4045_d_n11, assign6110_e4045_d_n12, assign6110_e4045_d_n17,) = {
    if (var_guard72 != 0.0) {
        let assign6110_e4040: f64 = (var_tmf1 * var_tmf1);
        let assign6110_e4042: f64 = (assign6110_e4040 + var_tmf2);
        let assign6110_e4043: f64 = (assign6110_e4042).sqrt();
        (assign6110_e4043, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign6110_e4043)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign6110_e4043)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign6110_e4043)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign6110_e4043)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign6110_e4043)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign6110_e4043)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign6110_e4043)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign6110_e4043)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6110_e4045;
        var_tmf2_dn0 = assign6110_e4045_d_n0;
        var_tmf2_dn2 = assign6110_e4045_d_n2;
        var_tmf2_dn6 = assign6110_e4045_d_n6;
        var_tmf2_dn7 = assign6110_e4045_d_n7;
        var_tmf2_dn10 = assign6110_e4045_d_n10;
        var_tmf2_dn11 = assign6110_e4045_d_n11;
        var_tmf2_dn12 = assign6110_e4045_d_n12;
        var_tmf2_dn17 = assign6110_e4045_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6120_e4055, assign6120_e4055_d_n0, assign6120_e4055_d_n2, assign6120_e4055_d_n6, assign6120_e4055_d_n7, assign6120_e4055_d_n10, assign6120_e4055_d_n11, assign6120_e4055_d_n12, assign6120_e4055_d_n17,) = {
    if (var_guard72 != 0.0) {
        let assign6120_e4051: f64 = (var_tmf1 + var_tmf2);
        let assign6120_e4052: f64 = (0.5 * assign6120_e4051);
        let assign6120_e4053: f64 = (var_vbslim + assign6120_e4052);
        (assign6120_e4053, (var_vbslim_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_vbslim_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_vbslim_dn6 + (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_vbslim_dn7 + (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_vbslim_dn10 + (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_vbslim_dn11 + (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_vbslim_dn12 + (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), (var_vbslim_dn17 + (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_vbsz2, var_vbsz2_dn0, var_vbsz2_dn2, var_vbsz2_dn6, var_vbsz2_dn7, var_vbsz2_dn10, var_vbsz2_dn11, var_vbsz2_dn12, var_vbsz2_dn17,)
    }
};
        var_vbsz2 = assign6120_e4055;
        var_vbsz2_dn0 = assign6120_e4055_d_n0;
        var_vbsz2_dn2 = assign6120_e4055_d_n2;
        var_vbsz2_dn6 = assign6120_e4055_d_n6;
        var_vbsz2_dn7 = assign6120_e4055_d_n7;
        var_vbsz2_dn10 = assign6120_e4055_d_n10;
        var_vbsz2_dn11 = assign6120_e4055_d_n11;
        var_vbsz2_dn12 = assign6120_e4055_d_n12;
        var_vbsz2_dn17 = assign6120_e4055_d_n17;
        var_vbsz2_rv = 0.0;

        let assign6130_e4058: f64 = if var_subversion > 2.0 { 1.0 } else { 0.0 };
        var_guard73 = assign6130_e4058;
        var_guard73_rv = 0.0;

        let (assign6140_e4068, assign6140_e4068_d_n0, assign6140_e4068_d_n2, assign6140_e4068_d_n6, assign6140_e4068_d_n7, assign6140_e4068_d_n10, assign6140_e4068_d_n11, assign6140_e4068_d_n12, assign6140_e4068_d_n17,) = {
    if ((var_guard72 != 0.0) && (var_guard73 != 0.0)) {
        let assign6140_e4064: f64 = (var_pb20 - var_vbsz2);
        let assign6140_e4066: f64 = (assign6140_e4064 - 0.001);
        (assign6140_e4066, (var_pb20_dn0 - var_vbsz2_dn0), (var_pb20_dn2 - var_vbsz2_dn2), (var_pb20_dn6 - var_vbsz2_dn6), (var_pb20_dn7 - var_vbsz2_dn7), (var_pb20_dn10 - var_vbsz2_dn10), (var_pb20_dn11 - var_vbsz2_dn11), (var_pb20_dn12 - var_vbsz2_dn12), (var_pb20_dn17 - var_vbsz2_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign6140_e4068;
        var_tmf1_dn0 = assign6140_e4068_d_n0;
        var_tmf1_dn2 = assign6140_e4068_d_n2;
        var_tmf1_dn6 = assign6140_e4068_d_n6;
        var_tmf1_dn7 = assign6140_e4068_d_n7;
        var_tmf1_dn10 = assign6140_e4068_d_n10;
        var_tmf1_dn11 = assign6140_e4068_d_n11;
        var_tmf1_dn12 = assign6140_e4068_d_n12;
        var_tmf1_dn17 = assign6140_e4068_d_n17;
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
        *var_guard71_slot = var_guard71;
        *var_guard71_rv_slot = var_guard71_rv;
        *var_guard72_slot = var_guard72;
        *var_guard72_rv_slot = var_guard72_rv;
        *var_guard73_slot = var_guard73;
        *var_guard73_rv_slot = var_guard73_rv;
        *var_t2__blk62_slot = var_t2__blk62;
        *var_t2__blk62_dn0_slot = var_t2__blk62_dn0;
        *var_t2__blk62_dn10_slot = var_t2__blk62_dn10;
        *var_t2__blk62_dn11_slot = var_t2__blk62_dn11;
        *var_t2__blk62_dn12_slot = var_t2__blk62_dn12;
        *var_t2__blk62_dn17_slot = var_t2__blk62_dn17;
        *var_t2__blk62_dn2_slot = var_t2__blk62_dn2;
        *var_t2__blk62_dn6_slot = var_t2__blk62_dn6;
        *var_t2__blk62_dn7_slot = var_t2__blk62_dn7;
        *var_t2__blk62_rv_slot = var_t2__blk62_rv;
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
        var_guard72: f64,
        var_guard73: f64,
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
        var_dvth0__blk85_slot: &mut f64,
        var_dvth0__blk85_dn0_slot: &mut f64,
        var_dvth0__blk85_dn10_slot: &mut f64,
        var_dvth0__blk85_dn11_slot: &mut f64,
        var_dvth0__blk85_dn12_slot: &mut f64,
        var_dvth0__blk85_dn17_slot: &mut f64,
        var_dvth0__blk85_dn2_slot: &mut f64,
        var_dvth0__blk85_dn6_slot: &mut f64,
        var_dvth0__blk85_dn7_slot: &mut f64,
        var_dvth0__blk85_rv_slot: &mut f64,
        var_guard74_slot: &mut f64,
        var_guard74_rv_slot: &mut f64,
        var_guard86_slot: &mut f64,
        var_guard86_rv_slot: &mut f64,
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
        var_t0__blk76_slot: &mut f64,
        var_t0__blk76_rv_slot: &mut f64,
        var_t0__blk79_slot: &mut f64,
        var_t0__blk79_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1__blk75_slot: &mut f64,
        var_t1__blk75_dn0_slot: &mut f64,
        var_t1__blk75_dn10_slot: &mut f64,
        var_t1__blk75_dn11_slot: &mut f64,
        var_t1__blk75_dn12_slot: &mut f64,
        var_t1__blk75_dn17_slot: &mut f64,
        var_t1__blk75_dn2_slot: &mut f64,
        var_t1__blk75_dn6_slot: &mut f64,
        var_t1__blk75_dn7_slot: &mut f64,
        var_t1__blk75_rv_slot: &mut f64,
        var_t1__blk80_slot: &mut f64,
        var_t1__blk80_dn0_slot: &mut f64,
        var_t1__blk80_dn10_slot: &mut f64,
        var_t1__blk80_dn11_slot: &mut f64,
        var_t1__blk80_dn12_slot: &mut f64,
        var_t1__blk80_dn17_slot: &mut f64,
        var_t1__blk80_dn2_slot: &mut f64,
        var_t1__blk80_dn6_slot: &mut f64,
        var_t1__blk80_dn7_slot: &mut f64,
        var_t1__blk80_rv_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2__blk77_slot: &mut f64,
        var_t2__blk77_dn0_slot: &mut f64,
        var_t2__blk77_dn10_slot: &mut f64,
        var_t2__blk77_dn11_slot: &mut f64,
        var_t2__blk77_dn12_slot: &mut f64,
        var_t2__blk77_dn17_slot: &mut f64,
        var_t2__blk77_dn2_slot: &mut f64,
        var_t2__blk77_dn6_slot: &mut f64,
        var_t2__blk77_dn7_slot: &mut f64,
        var_t2__blk77_rv_slot: &mut f64,
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
        var_t3__blk78_slot: &mut f64,
        var_t3__blk78_dn0_slot: &mut f64,
        var_t3__blk78_dn10_slot: &mut f64,
        var_t3__blk78_dn11_slot: &mut f64,
        var_t3__blk78_dn12_slot: &mut f64,
        var_t3__blk78_dn17_slot: &mut f64,
        var_t3__blk78_dn2_slot: &mut f64,
        var_t3__blk78_dn6_slot: &mut f64,
        var_t3__blk78_dn7_slot: &mut f64,
        var_t3__blk78_rv_slot: &mut f64,
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
        var_t4__blk83_slot: &mut f64,
        var_t4__blk83_rv_slot: &mut f64,
        var_t5__blk84_slot: &mut f64,
        var_t5__blk84_dn0_slot: &mut f64,
        var_t5__blk84_dn10_slot: &mut f64,
        var_t5__blk84_dn11_slot: &mut f64,
        var_t5__blk84_dn12_slot: &mut f64,
        var_t5__blk84_dn17_slot: &mut f64,
        var_t5__blk84_dn2_slot: &mut f64,
        var_t5__blk84_dn6_slot: &mut f64,
        var_t5__blk84_dn7_slot: &mut f64,
        var_t5__blk84_rv_slot: &mut f64,
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
        let mut var_dvth0__blk85: f64 = *var_dvth0__blk85_slot;
        let mut var_dvth0__blk85_dn0: f64 = *var_dvth0__blk85_dn0_slot;
        let mut var_dvth0__blk85_dn10: f64 = *var_dvth0__blk85_dn10_slot;
        let mut var_dvth0__blk85_dn11: f64 = *var_dvth0__blk85_dn11_slot;
        let mut var_dvth0__blk85_dn12: f64 = *var_dvth0__blk85_dn12_slot;
        let mut var_dvth0__blk85_dn17: f64 = *var_dvth0__blk85_dn17_slot;
        let mut var_dvth0__blk85_dn2: f64 = *var_dvth0__blk85_dn2_slot;
        let mut var_dvth0__blk85_dn6: f64 = *var_dvth0__blk85_dn6_slot;
        let mut var_dvth0__blk85_dn7: f64 = *var_dvth0__blk85_dn7_slot;
        let mut var_dvth0__blk85_rv: f64 = *var_dvth0__blk85_rv_slot;
        let mut var_guard74: f64 = *var_guard74_slot;
        let mut var_guard74_rv: f64 = *var_guard74_rv_slot;
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard86_rv: f64 = *var_guard86_rv_slot;
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
        let mut var_t0__blk76: f64 = *var_t0__blk76_slot;
        let mut var_t0__blk76_rv: f64 = *var_t0__blk76_rv_slot;
        let mut var_t0__blk79: f64 = *var_t0__blk79_slot;
        let mut var_t0__blk79_rv: f64 = *var_t0__blk79_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1__blk75: f64 = *var_t1__blk75_slot;
        let mut var_t1__blk75_dn0: f64 = *var_t1__blk75_dn0_slot;
        let mut var_t1__blk75_dn10: f64 = *var_t1__blk75_dn10_slot;
        let mut var_t1__blk75_dn11: f64 = *var_t1__blk75_dn11_slot;
        let mut var_t1__blk75_dn12: f64 = *var_t1__blk75_dn12_slot;
        let mut var_t1__blk75_dn17: f64 = *var_t1__blk75_dn17_slot;
        let mut var_t1__blk75_dn2: f64 = *var_t1__blk75_dn2_slot;
        let mut var_t1__blk75_dn6: f64 = *var_t1__blk75_dn6_slot;
        let mut var_t1__blk75_dn7: f64 = *var_t1__blk75_dn7_slot;
        let mut var_t1__blk75_rv: f64 = *var_t1__blk75_rv_slot;
        let mut var_t1__blk80: f64 = *var_t1__blk80_slot;
        let mut var_t1__blk80_dn0: f64 = *var_t1__blk80_dn0_slot;
        let mut var_t1__blk80_dn10: f64 = *var_t1__blk80_dn10_slot;
        let mut var_t1__blk80_dn11: f64 = *var_t1__blk80_dn11_slot;
        let mut var_t1__blk80_dn12: f64 = *var_t1__blk80_dn12_slot;
        let mut var_t1__blk80_dn17: f64 = *var_t1__blk80_dn17_slot;
        let mut var_t1__blk80_dn2: f64 = *var_t1__blk80_dn2_slot;
        let mut var_t1__blk80_dn6: f64 = *var_t1__blk80_dn6_slot;
        let mut var_t1__blk80_dn7: f64 = *var_t1__blk80_dn7_slot;
        let mut var_t1__blk80_rv: f64 = *var_t1__blk80_rv_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2__blk77: f64 = *var_t2__blk77_slot;
        let mut var_t2__blk77_dn0: f64 = *var_t2__blk77_dn0_slot;
        let mut var_t2__blk77_dn10: f64 = *var_t2__blk77_dn10_slot;
        let mut var_t2__blk77_dn11: f64 = *var_t2__blk77_dn11_slot;
        let mut var_t2__blk77_dn12: f64 = *var_t2__blk77_dn12_slot;
        let mut var_t2__blk77_dn17: f64 = *var_t2__blk77_dn17_slot;
        let mut var_t2__blk77_dn2: f64 = *var_t2__blk77_dn2_slot;
        let mut var_t2__blk77_dn6: f64 = *var_t2__blk77_dn6_slot;
        let mut var_t2__blk77_dn7: f64 = *var_t2__blk77_dn7_slot;
        let mut var_t2__blk77_rv: f64 = *var_t2__blk77_rv_slot;
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
        let mut var_t3__blk78: f64 = *var_t3__blk78_slot;
        let mut var_t3__blk78_dn0: f64 = *var_t3__blk78_dn0_slot;
        let mut var_t3__blk78_dn10: f64 = *var_t3__blk78_dn10_slot;
        let mut var_t3__blk78_dn11: f64 = *var_t3__blk78_dn11_slot;
        let mut var_t3__blk78_dn12: f64 = *var_t3__blk78_dn12_slot;
        let mut var_t3__blk78_dn17: f64 = *var_t3__blk78_dn17_slot;
        let mut var_t3__blk78_dn2: f64 = *var_t3__blk78_dn2_slot;
        let mut var_t3__blk78_dn6: f64 = *var_t3__blk78_dn6_slot;
        let mut var_t3__blk78_dn7: f64 = *var_t3__blk78_dn7_slot;
        let mut var_t3__blk78_rv: f64 = *var_t3__blk78_rv_slot;
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
        let mut var_t4__blk83: f64 = *var_t4__blk83_slot;
        let mut var_t4__blk83_rv: f64 = *var_t4__blk83_rv_slot;
        let mut var_t5__blk84: f64 = *var_t5__blk84_slot;
        let mut var_t5__blk84_dn0: f64 = *var_t5__blk84_dn0_slot;
        let mut var_t5__blk84_dn10: f64 = *var_t5__blk84_dn10_slot;
        let mut var_t5__blk84_dn11: f64 = *var_t5__blk84_dn11_slot;
        let mut var_t5__blk84_dn12: f64 = *var_t5__blk84_dn12_slot;
        let mut var_t5__blk84_dn17: f64 = *var_t5__blk84_dn17_slot;
        let mut var_t5__blk84_dn2: f64 = *var_t5__blk84_dn2_slot;
        let mut var_t5__blk84_dn6: f64 = *var_t5__blk84_dn6_slot;
        let mut var_t5__blk84_dn7: f64 = *var_t5__blk84_dn7_slot;
        let mut var_t5__blk84_rv: f64 = *var_t5__blk84_rv_slot;
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

        let (assign6150_e4078, assign6150_e4078_d_n0, assign6150_e4078_d_n2, assign6150_e4078_d_n6, assign6150_e4078_d_n7, assign6150_e4078_d_n10, assign6150_e4078_d_n11, assign6150_e4078_d_n12, assign6150_e4078_d_n17,) = {
    if ((var_guard72 != 0.0) && (var_guard73 != 0.0)) {
        let assign6150_e4074: f64 = (4.0 * var_pb20);
        let assign6150_e4076: f64 = (assign6150_e4074 * 0.001);
        (assign6150_e4076, ((4.0 * var_pb20_dn0) * 0.001), ((4.0 * var_pb20_dn2) * 0.001), ((4.0 * var_pb20_dn6) * 0.001), ((4.0 * var_pb20_dn7) * 0.001), ((4.0 * var_pb20_dn10) * 0.001), ((4.0 * var_pb20_dn11) * 0.001), ((4.0 * var_pb20_dn12) * 0.001), ((4.0 * var_pb20_dn17) * 0.001),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6150_e4078;
        var_tmf2_dn0 = assign6150_e4078_d_n0;
        var_tmf2_dn2 = assign6150_e4078_d_n2;
        var_tmf2_dn6 = assign6150_e4078_d_n6;
        var_tmf2_dn7 = assign6150_e4078_d_n7;
        var_tmf2_dn10 = assign6150_e4078_d_n10;
        var_tmf2_dn11 = assign6150_e4078_d_n11;
        var_tmf2_dn12 = assign6150_e4078_d_n12;
        var_tmf2_dn17 = assign6150_e4078_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6160_e4090, assign6160_e4090_d_n0, assign6160_e4090_d_n2, assign6160_e4090_d_n6, assign6160_e4090_d_n7, assign6160_e4090_d_n10, assign6160_e4090_d_n11, assign6160_e4090_d_n12, assign6160_e4090_d_n17,) = {
    if ((var_guard72 != 0.0) && (var_guard73 != 0.0)) {
        let (assign6160_e4088, assign6160_e4088_d_n0, assign6160_e4088_d_n2, assign6160_e4088_d_n6, assign6160_e4088_d_n7, assign6160_e4088_d_n10, assign6160_e4088_d_n11, assign6160_e4088_d_n12, assign6160_e4088_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign6160_e4087: f64 = (-var_tmf2);
                (assign6160_e4087, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign6160_e4088, assign6160_e4088_d_n0, assign6160_e4088_d_n2, assign6160_e4088_d_n6, assign6160_e4088_d_n7, assign6160_e4088_d_n10, assign6160_e4088_d_n11, assign6160_e4088_d_n12, assign6160_e4088_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6160_e4090;
        var_tmf2_dn0 = assign6160_e4090_d_n0;
        var_tmf2_dn2 = assign6160_e4090_d_n2;
        var_tmf2_dn6 = assign6160_e4090_d_n6;
        var_tmf2_dn7 = assign6160_e4090_d_n7;
        var_tmf2_dn10 = assign6160_e4090_d_n10;
        var_tmf2_dn11 = assign6160_e4090_d_n11;
        var_tmf2_dn12 = assign6160_e4090_d_n12;
        var_tmf2_dn17 = assign6160_e4090_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6170_e4101, assign6170_e4101_d_n0, assign6170_e4101_d_n2, assign6170_e4101_d_n6, assign6170_e4101_d_n7, assign6170_e4101_d_n10, assign6170_e4101_d_n11, assign6170_e4101_d_n12, assign6170_e4101_d_n17,) = {
    if ((var_guard72 != 0.0) && (var_guard73 != 0.0)) {
        let assign6170_e4096: f64 = (var_tmf1 * var_tmf1);
        let assign6170_e4098: f64 = (assign6170_e4096 + var_tmf2);
        let assign6170_e4099: f64 = (assign6170_e4098).sqrt();
        (assign6170_e4099, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign6170_e4099)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign6170_e4099)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign6170_e4099)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign6170_e4099)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign6170_e4099)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign6170_e4099)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign6170_e4099)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign6170_e4099)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6170_e4101;
        var_tmf2_dn0 = assign6170_e4101_d_n0;
        var_tmf2_dn2 = assign6170_e4101_d_n2;
        var_tmf2_dn6 = assign6170_e4101_d_n6;
        var_tmf2_dn7 = assign6170_e4101_d_n7;
        var_tmf2_dn10 = assign6170_e4101_d_n10;
        var_tmf2_dn11 = assign6170_e4101_d_n11;
        var_tmf2_dn12 = assign6170_e4101_d_n12;
        var_tmf2_dn17 = assign6170_e4101_d_n17;
        var_tmf2_rv = 0.0;

        let (assign6180_e4113, assign6180_e4113_d_n0, assign6180_e4113_d_n2, assign6180_e4113_d_n6, assign6180_e4113_d_n7, assign6180_e4113_d_n10, assign6180_e4113_d_n11, assign6180_e4113_d_n12, assign6180_e4113_d_n17,) = {
    if ((var_guard72 != 0.0) && (var_guard73 != 0.0)) {
        let assign6180_e4109: f64 = (var_tmf1 + var_tmf2);
        let assign6180_e4110: f64 = (0.5 * assign6180_e4109);
        let assign6180_e4111: f64 = (var_pb20 - assign6180_e4110);
        (assign6180_e4111, (var_pb20_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_pb20_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_pb20_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_pb20_dn7 - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_pb20_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_pb20_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_pb20_dn12 - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), (var_pb20_dn17 - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_vbsz2, var_vbsz2_dn0, var_vbsz2_dn2, var_vbsz2_dn6, var_vbsz2_dn7, var_vbsz2_dn10, var_vbsz2_dn11, var_vbsz2_dn12, var_vbsz2_dn17,)
    }
};
        var_vbsz2 = assign6180_e4113;
        var_vbsz2_dn0 = assign6180_e4113_d_n0;
        var_vbsz2_dn2 = assign6180_e4113_d_n2;
        var_vbsz2_dn6 = assign6180_e4113_d_n6;
        var_vbsz2_dn7 = assign6180_e4113_d_n7;
        var_vbsz2_dn10 = assign6180_e4113_d_n10;
        var_vbsz2_dn11 = assign6180_e4113_d_n11;
        var_vbsz2_dn12 = assign6180_e4113_d_n12;
        var_vbsz2_dn17 = assign6180_e4113_d_n17;
        var_vbsz2_rv = 0.0;

        let (assign6190_e4118, assign6190_e4118_d_n0, assign6190_e4118_d_n2, assign6190_e4118_d_n6, assign6190_e4118_d_n7, assign6190_e4118_d_n10, assign6190_e4118_d_n11, assign6190_e4118_d_n12, assign6190_e4118_d_n17,) = {
    if (var_guard72 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vbsz2, var_vbsz2_dn0, var_vbsz2_dn2, var_vbsz2_dn6, var_vbsz2_dn7, var_vbsz2_dn10, var_vbsz2_dn11, var_vbsz2_dn12, var_vbsz2_dn17,)
    }
};
        var_vbsz2 = assign6190_e4118;
        var_vbsz2_dn0 = assign6190_e4118_d_n0;
        var_vbsz2_dn2 = assign6190_e4118_d_n2;
        var_vbsz2_dn6 = assign6190_e4118_d_n6;
        var_vbsz2_dn7 = assign6190_e4118_d_n7;
        var_vbsz2_dn10 = assign6190_e4118_d_n10;
        var_vbsz2_dn11 = assign6190_e4118_d_n11;
        var_vbsz2_dn12 = assign6190_e4118_d_n12;
        var_vbsz2_dn17 = assign6190_e4118_d_n17;
        var_vbsz2_rv = 0.0;

        let assign6200_e4121: f64 = if var_subversion < 3.0 { 1.0 } else { 0.0 };
        var_guard74 = assign6200_e4121;
        var_guard74_rv = 0.0;

        let (assign6210_e4125, assign6210_e4125_d_n0, assign6210_e4125_d_n2, assign6210_e4125_d_n6, assign6210_e4125_d_n7, assign6210_e4125_d_n10, assign6210_e4125_d_n11, assign6210_e4125_d_n12, assign6210_e4125_d_n17,) = {
    if (var_guard74 != 0.0) {
        (p.p237, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_wd0, var_wd0_dn0, var_wd0_dn2, var_wd0_dn6, var_wd0_dn7, var_wd0_dn10, var_wd0_dn11, var_wd0_dn12, var_wd0_dn17,)
    }
};
        var_wd0 = assign6210_e4125;
        var_wd0_dn0 = assign6210_e4125_d_n0;
        var_wd0_dn2 = assign6210_e4125_d_n2;
        var_wd0_dn6 = assign6210_e4125_d_n6;
        var_wd0_dn7 = assign6210_e4125_d_n7;
        var_wd0_dn10 = assign6210_e4125_d_n10;
        var_wd0_dn11 = assign6210_e4125_d_n11;
        var_wd0_dn12 = assign6210_e4125_d_n12;
        var_wd0_dn17 = assign6210_e4125_d_n17;
        var_wd0_rv = 0.0;

        let (assign6220_e4134, assign6220_e4134_d_n0, assign6220_e4134_d_n2, assign6220_e4134_d_n6, assign6220_e4134_d_n7, assign6220_e4134_d_n10, assign6220_e4134_d_n11, assign6220_e4134_d_n12, assign6220_e4134_d_n17,) = {
    if (var_guard74 == 0.0) {
        let assign6220_e4130: f64 = (2.0 * 1.034943e-10);
        let assign6220_e4132: f64 = (assign6220_e4130 / var_q_nsub);
        (assign6220_e4132, (-((assign6220_e4130 * var_q_nsub_dn0) / (var_q_nsub * var_q_nsub))), (-((assign6220_e4130 * var_q_nsub_dn2) / (var_q_nsub * var_q_nsub))), (-((assign6220_e4130 * var_q_nsub_dn6) / (var_q_nsub * var_q_nsub))), (-((assign6220_e4130 * var_q_nsub_dn7) / (var_q_nsub * var_q_nsub))), (-((assign6220_e4130 * var_q_nsub_dn10) / (var_q_nsub * var_q_nsub))), (-((assign6220_e4130 * var_q_nsub_dn11) / (var_q_nsub * var_q_nsub))), (-((assign6220_e4130 * var_q_nsub_dn12) / (var_q_nsub * var_q_nsub))), (-((assign6220_e4130 * var_q_nsub_dn17) / (var_q_nsub * var_q_nsub))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign6220_e4134;
        var_t1_dn0 = assign6220_e4134_d_n0;
        var_t1_dn2 = assign6220_e4134_d_n2;
        var_t1_dn6 = assign6220_e4134_d_n6;
        var_t1_dn7 = assign6220_e4134_d_n7;
        var_t1_dn10 = assign6220_e4134_d_n10;
        var_t1_dn11 = assign6220_e4134_d_n11;
        var_t1_dn12 = assign6220_e4134_d_n12;
        var_t1_dn17 = assign6220_e4134_d_n17;
        var_t1_rv = 0.0;

        let (assign6230_e4144, assign6230_e4144_d_n0, assign6230_e4144_d_n2, assign6230_e4144_d_n6, assign6230_e4144_d_n7, assign6230_e4144_d_n10, assign6230_e4144_d_n11, assign6230_e4144_d_n12, assign6230_e4144_d_n17,) = {
    if (var_guard74 == 0.0) {
        let assign6230_e4140: f64 = (var_pb20 - var_vbsz2);
        let assign6230_e4141: f64 = (var_t1 * assign6230_e4140);
        let assign6230_e4142: f64 = (assign6230_e4141).sqrt();
        (assign6230_e4142, (((var_t1_dn0 * assign6230_e4140) + (var_t1 * (var_pb20_dn0 - var_vbsz2_dn0))) / (2.0 * assign6230_e4142)), (((var_t1_dn2 * assign6230_e4140) + (var_t1 * (var_pb20_dn2 - var_vbsz2_dn2))) / (2.0 * assign6230_e4142)), (((var_t1_dn6 * assign6230_e4140) + (var_t1 * (var_pb20_dn6 - var_vbsz2_dn6))) / (2.0 * assign6230_e4142)), (((var_t1_dn7 * assign6230_e4140) + (var_t1 * (var_pb20_dn7 - var_vbsz2_dn7))) / (2.0 * assign6230_e4142)), (((var_t1_dn10 * assign6230_e4140) + (var_t1 * (var_pb20_dn10 - var_vbsz2_dn10))) / (2.0 * assign6230_e4142)), (((var_t1_dn11 * assign6230_e4140) + (var_t1 * (var_pb20_dn11 - var_vbsz2_dn11))) / (2.0 * assign6230_e4142)), (((var_t1_dn12 * assign6230_e4140) + (var_t1 * (var_pb20_dn12 - var_vbsz2_dn12))) / (2.0 * assign6230_e4142)), (((var_t1_dn17 * assign6230_e4140) + (var_t1 * (var_pb20_dn17 - var_vbsz2_dn17))) / (2.0 * assign6230_e4142)),)
    } else {
        (var_wd0, var_wd0_dn0, var_wd0_dn2, var_wd0_dn6, var_wd0_dn7, var_wd0_dn10, var_wd0_dn11, var_wd0_dn12, var_wd0_dn17,)
    }
};
        var_wd0 = assign6230_e4144;
        var_wd0_dn0 = assign6230_e4144_d_n0;
        var_wd0_dn2 = assign6230_e4144_d_n2;
        var_wd0_dn6 = assign6230_e4144_d_n6;
        var_wd0_dn7 = assign6230_e4144_d_n7;
        var_wd0_dn10 = assign6230_e4144_d_n10;
        var_wd0_dn11 = assign6230_e4144_d_n11;
        var_wd0_dn12 = assign6230_e4144_d_n12;
        var_wd0_dn17 = assign6230_e4144_d_n17;
        var_wd0_rv = 0.0;

        let (assign6240_e4158, assign6240_e4158_d_n0, assign6240_e4158_d_n2, assign6240_e4158_d_n6, assign6240_e4158_d_n7, assign6240_e4158_d_n10, assign6240_e4158_d_n11, assign6240_e4158_d_n12, assign6240_e4158_d_n17,) = {
    if (var_subversion < 3.0) {
        let assign6240_e4150: f64 = (var_qnsub_esi2 * var_pb20);
        let assign6240_e4151: f64 = (assign6240_e4150).sqrt();
        (assign6240_e4151, (((var_qnsub_esi2_dn0 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn0)) / (2.0 * assign6240_e4151)), (((var_qnsub_esi2_dn2 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn2)) / (2.0 * assign6240_e4151)), (((var_qnsub_esi2_dn6 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn6)) / (2.0 * assign6240_e4151)), (((var_qnsub_esi2_dn7 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn7)) / (2.0 * assign6240_e4151)), (((var_qnsub_esi2_dn10 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn10)) / (2.0 * assign6240_e4151)), (((var_qnsub_esi2_dn11 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn11)) / (2.0 * assign6240_e4151)), (((var_qnsub_esi2_dn12 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn12)) / (2.0 * assign6240_e4151)), (((var_qnsub_esi2_dn17 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn17)) / (2.0 * assign6240_e4151)),)
    } else {
        let assign6240_e4155: f64 = (var_pb20 - var_vbsz2);
        let assign6240_e4156: f64 = (var_qnsub_esi2 * assign6240_e4155);
        let assign6240_e4157: f64 = (assign6240_e4156).sqrt();
        (assign6240_e4157, (((var_qnsub_esi2_dn0 * assign6240_e4155) + (var_qnsub_esi2 * (var_pb20_dn0 - var_vbsz2_dn0))) / (2.0 * assign6240_e4157)), (((var_qnsub_esi2_dn2 * assign6240_e4155) + (var_qnsub_esi2 * (var_pb20_dn2 - var_vbsz2_dn2))) / (2.0 * assign6240_e4157)), (((var_qnsub_esi2_dn6 * assign6240_e4155) + (var_qnsub_esi2 * (var_pb20_dn6 - var_vbsz2_dn6))) / (2.0 * assign6240_e4157)), (((var_qnsub_esi2_dn7 * assign6240_e4155) + (var_qnsub_esi2 * (var_pb20_dn7 - var_vbsz2_dn7))) / (2.0 * assign6240_e4157)), (((var_qnsub_esi2_dn10 * assign6240_e4155) + (var_qnsub_esi2 * (var_pb20_dn10 - var_vbsz2_dn10))) / (2.0 * assign6240_e4157)), (((var_qnsub_esi2_dn11 * assign6240_e4155) + (var_qnsub_esi2 * (var_pb20_dn11 - var_vbsz2_dn11))) / (2.0 * assign6240_e4157)), (((var_qnsub_esi2_dn12 * assign6240_e4155) + (var_qnsub_esi2 * (var_pb20_dn12 - var_vbsz2_dn12))) / (2.0 * assign6240_e4157)), (((var_qnsub_esi2_dn17 * assign6240_e4155) + (var_qnsub_esi2 * (var_pb20_dn17 - var_vbsz2_dn17))) / (2.0 * assign6240_e4157)),)
    }
};
        var_qb0 = assign6240_e4158;
        var_qb0_dn0 = assign6240_e4158_d_n0;
        var_qb0_dn2 = assign6240_e4158_d_n2;
        var_qb0_dn6 = assign6240_e4158_d_n6;
        var_qb0_dn7 = assign6240_e4158_d_n7;
        var_qb0_dn10 = assign6240_e4158_d_n10;
        var_qb0_dn11 = assign6240_e4158_d_n11;
        var_qb0_dn12 = assign6240_e4158_d_n12;
        var_qb0_dn17 = assign6240_e4158_d_n17;
        var_qb0_rv = 0.0;

        let assign6250_e4161: f64 = (var_pb20 + var_vfb);
        let assign6250_e4164: f64 = (var_qb0 * var_c_fox_inv);
        let assign6250_e4165: f64 = (assign6250_e4161 + assign6250_e4164);
        let assign6250_e4167: f64 = (assign6250_e4165 + var_ptovr);
        var_vthp = assign6250_e4167;
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

        var_t0__blk76 = 0.95;
        var_t0__blk76_rv = 0.0;

        let assign6280_e4172: f64 = (var_t0__blk76 * var_pb20b);
        let assign6280_e4174: f64 = (assign6280_e4172 - var_vbsz2);
        let assign6280_e4176: f64 = (assign6280_e4174 - 0.001);
        var_t1__blk75 = assign6280_e4176;
        var_t1__blk75_dn0 = ((var_t0__blk76 * var_pb20b_dn0) - var_vbsz2_dn0);
        var_t1__blk75_dn2 = ((var_t0__blk76 * var_pb20b_dn2) - var_vbsz2_dn2);
        var_t1__blk75_dn6 = ((var_t0__blk76 * var_pb20b_dn6) - var_vbsz2_dn6);
        var_t1__blk75_dn7 = ((var_t0__blk76 * var_pb20b_dn7) - var_vbsz2_dn7);
        var_t1__blk75_dn10 = ((var_t0__blk76 * var_pb20b_dn10) - var_vbsz2_dn10);
        var_t1__blk75_dn11 = ((var_t0__blk76 * var_pb20b_dn11) - var_vbsz2_dn11);
        var_t1__blk75_dn12 = ((var_t0__blk76 * var_pb20b_dn12) - var_vbsz2_dn12);
        var_t1__blk75_dn17 = ((var_t0__blk76 * var_pb20b_dn17) - var_vbsz2_dn17);
        var_t1__blk75_rv = 0.0;

        let assign6290_e4179: f64 = (var_t1__blk75 * var_t1__blk75);
        let assign6290_e4182: f64 = (4.0 * var_t0__blk76);
        let assign6290_e4184: f64 = (assign6290_e4182 * var_pb20b);
        let assign6290_e4186: f64 = (assign6290_e4184 * 0.001);
        let assign6290_e4187: f64 = (assign6290_e4179 + assign6290_e4186);
        let assign6290_e4188: f64 = (assign6290_e4187).sqrt();
        var_t2__blk77 = assign6290_e4188;
        var_t2__blk77_dn0 = ((((var_t1__blk75_dn0 * var_t1__blk75) + (var_t1__blk75 * var_t1__blk75_dn0)) + ((assign6290_e4182 * var_pb20b_dn0) * 0.001)) / (2.0 * assign6290_e4188));
        var_t2__blk77_dn2 = ((((var_t1__blk75_dn2 * var_t1__blk75) + (var_t1__blk75 * var_t1__blk75_dn2)) + ((assign6290_e4182 * var_pb20b_dn2) * 0.001)) / (2.0 * assign6290_e4188));
        var_t2__blk77_dn6 = ((((var_t1__blk75_dn6 * var_t1__blk75) + (var_t1__blk75 * var_t1__blk75_dn6)) + ((assign6290_e4182 * var_pb20b_dn6) * 0.001)) / (2.0 * assign6290_e4188));
        var_t2__blk77_dn7 = ((((var_t1__blk75_dn7 * var_t1__blk75) + (var_t1__blk75 * var_t1__blk75_dn7)) + ((assign6290_e4182 * var_pb20b_dn7) * 0.001)) / (2.0 * assign6290_e4188));
        var_t2__blk77_dn10 = ((((var_t1__blk75_dn10 * var_t1__blk75) + (var_t1__blk75 * var_t1__blk75_dn10)) + ((assign6290_e4182 * var_pb20b_dn10) * 0.001)) / (2.0 * assign6290_e4188));
        var_t2__blk77_dn11 = ((((var_t1__blk75_dn11 * var_t1__blk75) + (var_t1__blk75 * var_t1__blk75_dn11)) + ((assign6290_e4182 * var_pb20b_dn11) * 0.001)) / (2.0 * assign6290_e4188));
        var_t2__blk77_dn12 = ((((var_t1__blk75_dn12 * var_t1__blk75) + (var_t1__blk75 * var_t1__blk75_dn12)) + ((assign6290_e4182 * var_pb20b_dn12) * 0.001)) / (2.0 * assign6290_e4188));
        var_t2__blk77_dn17 = ((((var_t1__blk75_dn17 * var_t1__blk75) + (var_t1__blk75 * var_t1__blk75_dn17)) + ((assign6290_e4182 * var_pb20b_dn17) * 0.001)) / (2.0 * assign6290_e4188));
        var_t2__blk77_rv = 0.0;

        let assign6300_e4191: f64 = (var_t0__blk76 * var_pb20b);
        let assign6300_e4195: f64 = (var_t1__blk75 + var_t2__blk77);
        let assign6300_e4196: f64 = (0.5 * assign6300_e4195);
        let assign6300_e4197: f64 = (assign6300_e4191 - assign6300_e4196);
        var_t3__blk78 = assign6300_e4197;
        var_t3__blk78_dn0 = ((var_t0__blk76 * var_pb20b_dn0) - (0.5 * (var_t1__blk75_dn0 + var_t2__blk77_dn0)));
        var_t3__blk78_dn2 = ((var_t0__blk76 * var_pb20b_dn2) - (0.5 * (var_t1__blk75_dn2 + var_t2__blk77_dn2)));
        var_t3__blk78_dn6 = ((var_t0__blk76 * var_pb20b_dn6) - (0.5 * (var_t1__blk75_dn6 + var_t2__blk77_dn6)));
        var_t3__blk78_dn7 = ((var_t0__blk76 * var_pb20b_dn7) - (0.5 * (var_t1__blk75_dn7 + var_t2__blk77_dn7)));
        var_t3__blk78_dn10 = ((var_t0__blk76 * var_pb20b_dn10) - (0.5 * (var_t1__blk75_dn10 + var_t2__blk77_dn10)));
        var_t3__blk78_dn11 = ((var_t0__blk76 * var_pb20b_dn11) - (0.5 * (var_t1__blk75_dn11 + var_t2__blk77_dn11)));
        var_t3__blk78_dn12 = ((var_t0__blk76 * var_pb20b_dn12) - (0.5 * (var_t1__blk75_dn12 + var_t2__blk77_dn12)));
        var_t3__blk78_dn17 = ((var_t0__blk76 * var_pb20b_dn17) - (0.5 * (var_t1__blk75_dn17 + var_t2__blk77_dn17)));
        var_t3__blk78_rv = 0.0;

        let assign6310_e4200: f64 = (var_pb20b - var_t3__blk78);
        var_pbsum = assign6310_e4200;
        var_pbsum_dn0 = (var_pb20b_dn0 - var_t3__blk78_dn0);
        var_pbsum_dn2 = (var_pb20b_dn2 - var_t3__blk78_dn2);
        var_pbsum_dn6 = (var_pb20b_dn6 - var_t3__blk78_dn6);
        var_pbsum_dn7 = (var_pb20b_dn7 - var_t3__blk78_dn7);
        var_pbsum_dn10 = (var_pb20b_dn10 - var_t3__blk78_dn10);
        var_pbsum_dn11 = (var_pb20b_dn11 - var_t3__blk78_dn11);
        var_pbsum_dn12 = (var_pb20b_dn12 - var_t3__blk78_dn12);
        var_pbsum_dn17 = (var_pb20b_dn17 - var_t3__blk78_dn17);
        var_pbsum_rv = 0.0;

        let assign6320_e4202: f64 = (var_pbsum).sqrt();
        var_sqrt_pbsum = assign6320_e4202;
        var_sqrt_pbsum_dn0 = (var_pbsum_dn0 / (2.0 * assign6320_e4202));
        var_sqrt_pbsum_dn2 = (var_pbsum_dn2 / (2.0 * assign6320_e4202));
        var_sqrt_pbsum_dn6 = (var_pbsum_dn6 / (2.0 * assign6320_e4202));
        var_sqrt_pbsum_dn7 = (var_pbsum_dn7 / (2.0 * assign6320_e4202));
        var_sqrt_pbsum_dn10 = (var_pbsum_dn10 / (2.0 * assign6320_e4202));
        var_sqrt_pbsum_dn11 = (var_pbsum_dn11 / (2.0 * assign6320_e4202));
        var_sqrt_pbsum_dn12 = (var_pbsum_dn12 / (2.0 * assign6320_e4202));
        var_sqrt_pbsum_dn17 = (var_pbsum_dn17 / (2.0 * assign6320_e4202));
        var_sqrt_pbsum_rv = 0.0;

        let assign6330_e4205: f64 = if p.p72 != 0.0 { 1.0 } else { 0.0 };
        var_guard86 = assign6330_e4205;
        var_guard86_rv = 0.0;

        let (assign6340_e4215, assign6340_e4215_d_n0, assign6340_e4215_d_n2, assign6340_e4215_d_n6, assign6340_e4215_d_n7, assign6340_e4215_d_n10, assign6340_e4215_d_n11, assign6340_e4215_d_n12, assign6340_e4215_d_n17,) = {
    if (var_guard86 != 0.0) {
        let assign6340_e4209: f64 = (2.0 * 1.6021918e-19);
        let assign6340_e4211: f64 = (assign6340_e4209 * var_uc_nsubs);
        let assign6340_e4213: f64 = (assign6340_e4211 * 1.034943e-10);
        (assign6340_e4213, ((assign6340_e4209 * var_uc_nsubs_dn0) * 1.034943e-10), ((assign6340_e4209 * var_uc_nsubs_dn2) * 1.034943e-10), ((assign6340_e4209 * var_uc_nsubs_dn6) * 1.034943e-10), ((assign6340_e4209 * var_uc_nsubs_dn7) * 1.034943e-10), ((assign6340_e4209 * var_uc_nsubs_dn10) * 1.034943e-10), ((assign6340_e4209 * var_uc_nsubs_dn11) * 1.034943e-10), ((assign6340_e4209 * var_uc_nsubs_dn12) * 1.034943e-10), ((assign6340_e4209 * var_uc_nsubs_dn17) * 1.034943e-10),)
    } else {
        (var_t1__blk80, var_t1__blk80_dn0, var_t1__blk80_dn2, var_t1__blk80_dn6, var_t1__blk80_dn7, var_t1__blk80_dn10, var_t1__blk80_dn11, var_t1__blk80_dn12, var_t1__blk80_dn17,)
    }
};
        var_t1__blk80 = assign6340_e4215;
        var_t1__blk80_dn0 = assign6340_e4215_d_n0;
        var_t1__blk80_dn2 = assign6340_e4215_d_n2;
        var_t1__blk80_dn6 = assign6340_e4215_d_n6;
        var_t1__blk80_dn7 = assign6340_e4215_d_n7;
        var_t1__blk80_dn10 = assign6340_e4215_d_n10;
        var_t1__blk80_dn11 = assign6340_e4215_d_n11;
        var_t1__blk80_dn12 = assign6340_e4215_d_n12;
        var_t1__blk80_dn17 = assign6340_e4215_d_n17;
        var_t1__blk80_rv = 0.0;

        let (assign6350_e4232, assign6350_e4232_d_n0, assign6350_e4232_d_n2, assign6350_e4232_d_n6, assign6350_e4232_d_n7, assign6350_e4232_d_n10, assign6350_e4232_d_n11, assign6350_e4232_d_n12, assign6350_e4232_d_n17,) = {
    if (var_guard86 != 0.0) {
        let (assign6350_e4230, assign6350_e4230_d_n0, assign6350_e4230_d_n2, assign6350_e4230_d_n6, assign6350_e4230_d_n7, assign6350_e4230_d_n10, assign6350_e4230_d_n11, assign6350_e4230_d_n12, assign6350_e4230_d_n17,) = {
            if (var_subversion < 3.0) {
                let assign6350_e4222: f64 = (var_t1__blk80 * var_pb2c);
                let assign6350_e4223: f64 = (assign6350_e4222).sqrt();
                (assign6350_e4223, (((var_t1__blk80_dn0 * var_pb2c) + (var_t1__blk80 * var_pb2c_dn0)) / (2.0 * assign6350_e4223)), (((var_t1__blk80_dn2 * var_pb2c) + (var_t1__blk80 * var_pb2c_dn2)) / (2.0 * assign6350_e4223)), (((var_t1__blk80_dn6 * var_pb2c) + (var_t1__blk80 * var_pb2c_dn6)) / (2.0 * assign6350_e4223)), (((var_t1__blk80_dn7 * var_pb2c) + (var_t1__blk80 * var_pb2c_dn7)) / (2.0 * assign6350_e4223)), (((var_t1__blk80_dn10 * var_pb2c) + (var_t1__blk80 * var_pb2c_dn10)) / (2.0 * assign6350_e4223)), (((var_t1__blk80_dn11 * var_pb2c) + (var_t1__blk80 * var_pb2c_dn11)) / (2.0 * assign6350_e4223)), (((var_t1__blk80_dn12 * var_pb2c) + (var_t1__blk80 * var_pb2c_dn12)) / (2.0 * assign6350_e4223)), (((var_t1__blk80_dn17 * var_pb2c) + (var_t1__blk80 * var_pb2c_dn17)) / (2.0 * assign6350_e4223)),)
            } else {
                let assign6350_e4227: f64 = (var_pb2c - var_vbsz2);
                let assign6350_e4228: f64 = (var_t1__blk80 * assign6350_e4227);
                let assign6350_e4229: f64 = (assign6350_e4228).sqrt();
                (assign6350_e4229, (((var_t1__blk80_dn0 * assign6350_e4227) + (var_t1__blk80 * (var_pb2c_dn0 - var_vbsz2_dn0))) / (2.0 * assign6350_e4229)), (((var_t1__blk80_dn2 * assign6350_e4227) + (var_t1__blk80 * (var_pb2c_dn2 - var_vbsz2_dn2))) / (2.0 * assign6350_e4229)), (((var_t1__blk80_dn6 * assign6350_e4227) + (var_t1__blk80 * (var_pb2c_dn6 - var_vbsz2_dn6))) / (2.0 * assign6350_e4229)), (((var_t1__blk80_dn7 * assign6350_e4227) + (var_t1__blk80 * (var_pb2c_dn7 - var_vbsz2_dn7))) / (2.0 * assign6350_e4229)), (((var_t1__blk80_dn10 * assign6350_e4227) + (var_t1__blk80 * (var_pb2c_dn10 - var_vbsz2_dn10))) / (2.0 * assign6350_e4229)), (((var_t1__blk80_dn11 * assign6350_e4227) + (var_t1__blk80 * (var_pb2c_dn11 - var_vbsz2_dn11))) / (2.0 * assign6350_e4229)), (((var_t1__blk80_dn12 * assign6350_e4227) + (var_t1__blk80 * (var_pb2c_dn12 - var_vbsz2_dn12))) / (2.0 * assign6350_e4229)), (((var_t1__blk80_dn17 * assign6350_e4227) + (var_t1__blk80 * (var_pb2c_dn17 - var_vbsz2_dn17))) / (2.0 * assign6350_e4229)),)
            }
        };
        (assign6350_e4230, assign6350_e4230_d_n0, assign6350_e4230_d_n2, assign6350_e4230_d_n6, assign6350_e4230_d_n7, assign6350_e4230_d_n10, assign6350_e4230_d_n11, assign6350_e4230_d_n12, assign6350_e4230_d_n17,)
    } else {
        (var_t2__blk81, var_t2__blk81_dn0, var_t2__blk81_dn2, var_t2__blk81_dn6, var_t2__blk81_dn7, var_t2__blk81_dn10, var_t2__blk81_dn11, var_t2__blk81_dn12, var_t2__blk81_dn17,)
    }
};
        var_t2__blk81 = assign6350_e4232;
        var_t2__blk81_dn0 = assign6350_e4232_d_n0;
        var_t2__blk81_dn2 = assign6350_e4232_d_n2;
        var_t2__blk81_dn6 = assign6350_e4232_d_n6;
        var_t2__blk81_dn7 = assign6350_e4232_d_n7;
        var_t2__blk81_dn10 = assign6350_e4232_d_n10;
        var_t2__blk81_dn11 = assign6350_e4232_d_n11;
        var_t2__blk81_dn12 = assign6350_e4232_d_n12;
        var_t2__blk81_dn17 = assign6350_e4232_d_n17;
        var_t2__blk81_rv = 0.0;

        let (assign6360_e4242, assign6360_e4242_d_n0, assign6360_e4242_d_n2, assign6360_e4242_d_n6, assign6360_e4242_d_n7, assign6360_e4242_d_n10, assign6360_e4242_d_n11, assign6360_e4242_d_n12, assign6360_e4242_d_n17,) = {
    if (var_guard86 != 0.0) {
        let assign6360_e4236: f64 = (var_pb2c + var_vfb);
        let assign6360_e4239: f64 = (var_t2__blk81 * var_c_fox_inv);
        let assign6360_e4240: f64 = (assign6360_e4236 + assign6360_e4239);
        (assign6360_e4240, (var_pb2c_dn0 + ((var_t2__blk81_dn0 * var_c_fox_inv) + (var_t2__blk81 * var_c_fox_inv_dn0))), (var_pb2c_dn2 + ((var_t2__blk81_dn2 * var_c_fox_inv) + (var_t2__blk81 * var_c_fox_inv_dn2))), (var_pb2c_dn6 + ((var_t2__blk81_dn6 * var_c_fox_inv) + (var_t2__blk81 * var_c_fox_inv_dn6))), (var_pb2c_dn7 + ((var_t2__blk81_dn7 * var_c_fox_inv) + (var_t2__blk81 * var_c_fox_inv_dn7))), (var_pb2c_dn10 + ((var_t2__blk81_dn10 * var_c_fox_inv) + (var_t2__blk81 * var_c_fox_inv_dn10))), (var_pb2c_dn11 + ((var_t2__blk81_dn11 * var_c_fox_inv) + (var_t2__blk81 * var_c_fox_inv_dn11))), (var_pb2c_dn12 + ((var_t2__blk81_dn12 * var_c_fox_inv) + (var_t2__blk81 * var_c_fox_inv_dn12))), (var_pb2c_dn17 + ((var_t2__blk81_dn17 * var_c_fox_inv) + (var_t2__blk81 * var_c_fox_inv_dn17))),)
    } else {
        (var_vth0, var_vth0_dn0, var_vth0_dn2, var_vth0_dn6, var_vth0_dn7, var_vth0_dn10, var_vth0_dn11, var_vth0_dn12, var_vth0_dn17,)
    }
};
        var_vth0 = assign6360_e4242;
        var_vth0_dn0 = assign6360_e4242_d_n0;
        var_vth0_dn2 = assign6360_e4242_d_n2;
        var_vth0_dn6 = assign6360_e4242_d_n6;
        var_vth0_dn7 = assign6360_e4242_d_n7;
        var_vth0_dn10 = assign6360_e4242_d_n10;
        var_vth0_dn11 = assign6360_e4242_d_n11;
        var_vth0_dn12 = assign6360_e4242_d_n12;
        var_vth0_dn17 = assign6360_e4242_d_n17;
        var_vth0_rv = 0.0;

        let (assign6370_e4248, assign6370_e4248_d_n0, assign6370_e4248_d_n2, assign6370_e4248_d_n6, assign6370_e4248_d_n7, assign6370_e4248_d_n10, assign6370_e4248_d_n11, assign6370_e4248_d_n12, assign6370_e4248_d_n17,) = {
    if (var_guard86 != 0.0) {
        let assign6370_e4246: f64 = (1.034943e-10 * var_c_fox_inv);
        (assign6370_e4246, (1.034943e-10 * var_c_fox_inv_dn0), (1.034943e-10 * var_c_fox_inv_dn2), (1.034943e-10 * var_c_fox_inv_dn6), (1.034943e-10 * var_c_fox_inv_dn7), (1.034943e-10 * var_c_fox_inv_dn10), (1.034943e-10 * var_c_fox_inv_dn11), (1.034943e-10 * var_c_fox_inv_dn12), (1.034943e-10 * var_c_fox_inv_dn17),)
    } else {
        (var_t1__blk80, var_t1__blk80_dn0, var_t1__blk80_dn2, var_t1__blk80_dn6, var_t1__blk80_dn7, var_t1__blk80_dn10, var_t1__blk80_dn11, var_t1__blk80_dn12, var_t1__blk80_dn17,)
    }
};
        var_t1__blk80 = assign6370_e4248;
        var_t1__blk80_dn0 = assign6370_e4248_d_n0;
        var_t1__blk80_dn2 = assign6370_e4248_d_n2;
        var_t1__blk80_dn6 = assign6370_e4248_d_n6;
        var_t1__blk80_dn7 = assign6370_e4248_d_n7;
        var_t1__blk80_dn10 = assign6370_e4248_d_n10;
        var_t1__blk80_dn11 = assign6370_e4248_d_n11;
        var_t1__blk80_dn12 = assign6370_e4248_d_n12;
        var_t1__blk80_dn17 = assign6370_e4248_d_n17;
        var_t1__blk80_rv = 0.0;

        let (assign6380_e4256,) = {
    if (var_guard86 != 0.0) {
        let assign6380_e4253: f64 = (p.p72 * p.p72);
        let assign6380_e4254: f64 = (1.0 / assign6380_e4253);
        (assign6380_e4254,)
    } else {
        (var_t4__blk83,)
    }
};
        var_t4__blk83 = assign6380_e4256;
        var_t4__blk83_rv = 0.0;

        let (assign6390_e4264, assign6390_e4264_d_n0, assign6390_e4264_d_n2, assign6390_e4264_d_n6, assign6390_e4264_d_n7, assign6390_e4264_d_n10, assign6390_e4264_d_n11, assign6390_e4264_d_n12, assign6390_e4264_d_n17,) = {
    if (var_guard86 != 0.0) {
        let assign6390_e4260: f64 = (2.0 * var_wd0);
        let assign6390_e4262: f64 = (assign6390_e4260 * var_t4__blk83);
        (assign6390_e4262, ((2.0 * var_wd0_dn0) * var_t4__blk83), ((2.0 * var_wd0_dn2) * var_t4__blk83), ((2.0 * var_wd0_dn6) * var_t4__blk83), ((2.0 * var_wd0_dn7) * var_t4__blk83), ((2.0 * var_wd0_dn10) * var_t4__blk83), ((2.0 * var_wd0_dn11) * var_t4__blk83), ((2.0 * var_wd0_dn12) * var_t4__blk83), ((2.0 * var_wd0_dn17) * var_t4__blk83),)
    } else {
        (var_t3__blk82, var_t3__blk82_dn0, var_t3__blk82_dn2, var_t3__blk82_dn6, var_t3__blk82_dn7, var_t3__blk82_dn10, var_t3__blk82_dn11, var_t3__blk82_dn12, var_t3__blk82_dn17,)
    }
};
        var_t3__blk82 = assign6390_e4264;
        var_t3__blk82_dn0 = assign6390_e4264_d_n0;
        var_t3__blk82_dn2 = assign6390_e4264_d_n2;
        var_t3__blk82_dn6 = assign6390_e4264_d_n6;
        var_t3__blk82_dn7 = assign6390_e4264_d_n7;
        var_t3__blk82_dn10 = assign6390_e4264_d_n10;
        var_t3__blk82_dn11 = assign6390_e4264_d_n11;
        var_t3__blk82_dn12 = assign6390_e4264_d_n12;
        var_t3__blk82_dn17 = assign6390_e4264_d_n17;
        var_t3__blk82_rv = 0.0;

        let (assign6400_e4274, assign6400_e4274_d_n0, assign6400_e4274_d_n2, assign6400_e4274_d_n6, assign6400_e4274_d_n7, assign6400_e4274_d_n10, assign6400_e4274_d_n11, assign6400_e4274_d_n12, assign6400_e4274_d_n17,) = {
    if (var_guard86 != 0.0) {
        let assign6400_e4268: f64 = (var_t1__blk80 * var_t3__blk82);
        let assign6400_e4271: f64 = (p.p69 - var_pb20b);
        let assign6400_e4272: f64 = (assign6400_e4268 * assign6400_e4271);
        (assign6400_e4272, ((((var_t1__blk80_dn0 * var_t3__blk82) + (var_t1__blk80 * var_t3__blk82_dn0)) * assign6400_e4271) + (assign6400_e4268 * (-var_pb20b_dn0))), ((((var_t1__blk80_dn2 * var_t3__blk82) + (var_t1__blk80 * var_t3__blk82_dn2)) * assign6400_e4271) + (assign6400_e4268 * (-var_pb20b_dn2))), ((((var_t1__blk80_dn6 * var_t3__blk82) + (var_t1__blk80 * var_t3__blk82_dn6)) * assign6400_e4271) + (assign6400_e4268 * (-var_pb20b_dn6))), ((((var_t1__blk80_dn7 * var_t3__blk82) + (var_t1__blk80 * var_t3__blk82_dn7)) * assign6400_e4271) + (assign6400_e4268 * (-var_pb20b_dn7))), ((((var_t1__blk80_dn10 * var_t3__blk82) + (var_t1__blk80 * var_t3__blk82_dn10)) * assign6400_e4271) + (assign6400_e4268 * (-var_pb20b_dn10))), ((((var_t1__blk80_dn11 * var_t3__blk82) + (var_t1__blk80 * var_t3__blk82_dn11)) * assign6400_e4271) + (assign6400_e4268 * (-var_pb20b_dn11))), ((((var_t1__blk80_dn12 * var_t3__blk82) + (var_t1__blk80 * var_t3__blk82_dn12)) * assign6400_e4271) + (assign6400_e4268 * (-var_pb20b_dn12))), ((((var_t1__blk80_dn17 * var_t3__blk82) + (var_t1__blk80 * var_t3__blk82_dn17)) * assign6400_e4271) + (assign6400_e4268 * (-var_pb20b_dn17))),)
    } else {
        (var_t5__blk84, var_t5__blk84_dn0, var_t5__blk84_dn2, var_t5__blk84_dn6, var_t5__blk84_dn7, var_t5__blk84_dn10, var_t5__blk84_dn11, var_t5__blk84_dn12, var_t5__blk84_dn17,)
    }
};
        var_t5__blk84 = assign6400_e4274;
        var_t5__blk84_dn0 = assign6400_e4274_d_n0;
        var_t5__blk84_dn2 = assign6400_e4274_d_n2;
        var_t5__blk84_dn6 = assign6400_e4274_d_n6;
        var_t5__blk84_dn7 = assign6400_e4274_d_n7;
        var_t5__blk84_dn10 = assign6400_e4274_d_n10;
        var_t5__blk84_dn11 = assign6400_e4274_d_n11;
        var_t5__blk84_dn12 = assign6400_e4274_d_n12;
        var_t5__blk84_dn17 = assign6400_e4274_d_n17;
        var_t5__blk84_rv = 0.0;

        let (assign6410_e4278, assign6410_e4278_d_n0, assign6410_e4278_d_n2, assign6410_e4278_d_n6, assign6410_e4278_d_n7, assign6410_e4278_d_n10, assign6410_e4278_d_n11, assign6410_e4278_d_n12, assign6410_e4278_d_n17,) = {
    if (var_guard86 != 0.0) {
        (var_t5__blk84, var_t5__blk84_dn0, var_t5__blk84_dn2, var_t5__blk84_dn6, var_t5__blk84_dn7, var_t5__blk84_dn10, var_t5__blk84_dn11, var_t5__blk84_dn12, var_t5__blk84_dn17,)
    } else {
        (var_dvth0__blk85, var_dvth0__blk85_dn0, var_dvth0__blk85_dn2, var_dvth0__blk85_dn6, var_dvth0__blk85_dn7, var_dvth0__blk85_dn10, var_dvth0__blk85_dn11, var_dvth0__blk85_dn12, var_dvth0__blk85_dn17,)
    }
};
        var_dvth0__blk85 = assign6410_e4278;
        var_dvth0__blk85_dn0 = assign6410_e4278_d_n0;
        var_dvth0__blk85_dn2 = assign6410_e4278_d_n2;
        var_dvth0__blk85_dn6 = assign6410_e4278_d_n6;
        var_dvth0__blk85_dn7 = assign6410_e4278_d_n7;
        var_dvth0__blk85_dn10 = assign6410_e4278_d_n10;
        var_dvth0__blk85_dn11 = assign6410_e4278_d_n11;
        var_dvth0__blk85_dn12 = assign6410_e4278_d_n12;
        var_dvth0__blk85_dn17 = assign6410_e4278_d_n17;
        var_dvth0__blk85_rv = 0.0;

        let (assign6420_e4284, assign6420_e4284_d_n0, assign6420_e4284_d_n2, assign6420_e4284_d_n6, assign6420_e4284_d_n7, assign6420_e4284_d_n10, assign6420_e4284_d_n11, assign6420_e4284_d_n12, assign6420_e4284_d_n17,) = {
    if (var_guard86 != 0.0) {
        let assign6420_e4282: f64 = (var_vthp - var_vth0);
        (assign6420_e4282, (var_vthp_dn0 - var_vth0_dn0), (var_vthp_dn2 - var_vth0_dn2), (var_vthp_dn6 - var_vth0_dn6), (var_vthp_dn7 - var_vth0_dn7), (var_vthp_dn10 - var_vth0_dn10), (var_vthp_dn11 - var_vth0_dn11), (var_vthp_dn12 - var_vth0_dn12), (var_vthp_dn17 - var_vth0_dn17),)
    } else {
        (var_t1__blk80, var_t1__blk80_dn0, var_t1__blk80_dn2, var_t1__blk80_dn6, var_t1__blk80_dn7, var_t1__blk80_dn10, var_t1__blk80_dn11, var_t1__blk80_dn12, var_t1__blk80_dn17,)
    }
};
        var_t1__blk80 = assign6420_e4284;
        var_t1__blk80_dn0 = assign6420_e4284_d_n0;
        var_t1__blk80_dn2 = assign6420_e4284_d_n2;
        var_t1__blk80_dn6 = assign6420_e4284_d_n6;
        var_t1__blk80_dn7 = assign6420_e4284_d_n7;
        var_t1__blk80_dn10 = assign6420_e4284_d_n10;
        var_t1__blk80_dn11 = assign6420_e4284_d_n11;
        var_t1__blk80_dn12 = assign6420_e4284_d_n12;
        var_t1__blk80_dn17 = assign6420_e4284_d_n17;
        var_t1__blk80_rv = 0.0;

        let (assign6430_e4290,) = {
    if (var_guard86 != 0.0) {
        let assign6430_e4288: f64 = (var_uc_scp3 / p.p72);
        (assign6430_e4288,)
    } else {
        (var_t0__blk79,)
    }
};
        var_t0__blk79 = assign6430_e4290;
        var_t0__blk79_rv = 0.0;

        let (assign6440_e4298, assign6440_e4298_d_n0, assign6440_e4298_d_n2, assign6440_e4298_d_n6, assign6440_e4298_d_n7, assign6440_e4298_d_n10, assign6440_e4298_d_n11, assign6440_e4298_d_n12, assign6440_e4298_d_n17,) = {
    if (var_guard86 != 0.0) {
        let assign6440_e4295: f64 = (var_t0__blk79 * var_pbsum);
        let assign6440_e4296: f64 = (p.p80 + assign6440_e4295);
        (assign6440_e4296, (var_t0__blk79 * var_pbsum_dn0), (var_t0__blk79 * var_pbsum_dn2), (var_t0__blk79 * var_pbsum_dn6), (var_t0__blk79 * var_pbsum_dn7), (var_t0__blk79 * var_pbsum_dn10), (var_t0__blk79 * var_pbsum_dn11), (var_t0__blk79 * var_pbsum_dn12), (var_t0__blk79 * var_pbsum_dn17),)
    } else {
        (var_t2__blk81, var_t2__blk81_dn0, var_t2__blk81_dn2, var_t2__blk81_dn6, var_t2__blk81_dn7, var_t2__blk81_dn10, var_t2__blk81_dn11, var_t2__blk81_dn12, var_t2__blk81_dn17,)
    }
};
        var_t2__blk81 = assign6440_e4298;
        var_t2__blk81_dn0 = assign6440_e4298_d_n0;
        var_t2__blk81_dn2 = assign6440_e4298_d_n2;
        var_t2__blk81_dn6 = assign6440_e4298_d_n6;
        var_t2__blk81_dn7 = assign6440_e4298_d_n7;
        var_t2__blk81_dn10 = assign6440_e4298_d_n10;
        var_t2__blk81_dn11 = assign6440_e4298_d_n11;
        var_t2__blk81_dn12 = assign6440_e4298_d_n12;
        var_t2__blk81_dn17 = assign6440_e4298_d_n17;
        var_t2__blk81_rv = 0.0;

        *var_dvth0__blk85_slot = var_dvth0__blk85;
        *var_dvth0__blk85_dn0_slot = var_dvth0__blk85_dn0;
        *var_dvth0__blk85_dn10_slot = var_dvth0__blk85_dn10;
        *var_dvth0__blk85_dn11_slot = var_dvth0__blk85_dn11;
        *var_dvth0__blk85_dn12_slot = var_dvth0__blk85_dn12;
        *var_dvth0__blk85_dn17_slot = var_dvth0__blk85_dn17;
        *var_dvth0__blk85_dn2_slot = var_dvth0__blk85_dn2;
        *var_dvth0__blk85_dn6_slot = var_dvth0__blk85_dn6;
        *var_dvth0__blk85_dn7_slot = var_dvth0__blk85_dn7;
        *var_dvth0__blk85_rv_slot = var_dvth0__blk85_rv;
        *var_guard74_slot = var_guard74;
        *var_guard74_rv_slot = var_guard74_rv;
        *var_guard86_slot = var_guard86;
        *var_guard86_rv_slot = var_guard86_rv;
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
        *var_t0__blk76_slot = var_t0__blk76;
        *var_t0__blk76_rv_slot = var_t0__blk76_rv;
        *var_t0__blk79_slot = var_t0__blk79;
        *var_t0__blk79_rv_slot = var_t0__blk79_rv;
        *var_t1_slot = var_t1;
        *var_t1__blk75_slot = var_t1__blk75;
        *var_t1__blk75_dn0_slot = var_t1__blk75_dn0;
        *var_t1__blk75_dn10_slot = var_t1__blk75_dn10;
        *var_t1__blk75_dn11_slot = var_t1__blk75_dn11;
        *var_t1__blk75_dn12_slot = var_t1__blk75_dn12;
        *var_t1__blk75_dn17_slot = var_t1__blk75_dn17;
        *var_t1__blk75_dn2_slot = var_t1__blk75_dn2;
        *var_t1__blk75_dn6_slot = var_t1__blk75_dn6;
        *var_t1__blk75_dn7_slot = var_t1__blk75_dn7;
        *var_t1__blk75_rv_slot = var_t1__blk75_rv;
        *var_t1__blk80_slot = var_t1__blk80;
        *var_t1__blk80_dn0_slot = var_t1__blk80_dn0;
        *var_t1__blk80_dn10_slot = var_t1__blk80_dn10;
        *var_t1__blk80_dn11_slot = var_t1__blk80_dn11;
        *var_t1__blk80_dn12_slot = var_t1__blk80_dn12;
        *var_t1__blk80_dn17_slot = var_t1__blk80_dn17;
        *var_t1__blk80_dn2_slot = var_t1__blk80_dn2;
        *var_t1__blk80_dn6_slot = var_t1__blk80_dn6;
        *var_t1__blk80_dn7_slot = var_t1__blk80_dn7;
        *var_t1__blk80_rv_slot = var_t1__blk80_rv;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2__blk77_slot = var_t2__blk77;
        *var_t2__blk77_dn0_slot = var_t2__blk77_dn0;
        *var_t2__blk77_dn10_slot = var_t2__blk77_dn10;
        *var_t2__blk77_dn11_slot = var_t2__blk77_dn11;
        *var_t2__blk77_dn12_slot = var_t2__blk77_dn12;
        *var_t2__blk77_dn17_slot = var_t2__blk77_dn17;
        *var_t2__blk77_dn2_slot = var_t2__blk77_dn2;
        *var_t2__blk77_dn6_slot = var_t2__blk77_dn6;
        *var_t2__blk77_dn7_slot = var_t2__blk77_dn7;
        *var_t2__blk77_rv_slot = var_t2__blk77_rv;
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
        *var_t3__blk78_slot = var_t3__blk78;
        *var_t3__blk78_dn0_slot = var_t3__blk78_dn0;
        *var_t3__blk78_dn10_slot = var_t3__blk78_dn10;
        *var_t3__blk78_dn11_slot = var_t3__blk78_dn11;
        *var_t3__blk78_dn12_slot = var_t3__blk78_dn12;
        *var_t3__blk78_dn17_slot = var_t3__blk78_dn17;
        *var_t3__blk78_dn2_slot = var_t3__blk78_dn2;
        *var_t3__blk78_dn6_slot = var_t3__blk78_dn6;
        *var_t3__blk78_dn7_slot = var_t3__blk78_dn7;
        *var_t3__blk78_rv_slot = var_t3__blk78_rv;
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
        *var_t4__blk83_slot = var_t4__blk83;
        *var_t4__blk83_rv_slot = var_t4__blk83_rv;
        *var_t5__blk84_slot = var_t5__blk84;
        *var_t5__blk84_dn0_slot = var_t5__blk84_dn0;
        *var_t5__blk84_dn10_slot = var_t5__blk84_dn10;
        *var_t5__blk84_dn11_slot = var_t5__blk84_dn11;
        *var_t5__blk84_dn12_slot = var_t5__blk84_dn12;
        *var_t5__blk84_dn17_slot = var_t5__blk84_dn17;
        *var_t5__blk84_dn2_slot = var_t5__blk84_dn2;
        *var_t5__blk84_dn6_slot = var_t5__blk84_dn6;
        *var_t5__blk84_dn7_slot = var_t5__blk84_dn7;
        *var_t5__blk84_rv_slot = var_t5__blk84_rv;
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
        var_dvth0__blk85: f64,
        var_dvth0__blk85_dn0: f64,
        var_dvth0__blk85_dn10: f64,
        var_dvth0__blk85_dn11: f64,
        var_dvth0__blk85_dn12: f64,
        var_dvth0__blk85_dn17: f64,
        var_dvth0__blk85_dn2: f64,
        var_dvth0__blk85_dn6: f64,
        var_dvth0__blk85_dn7: f64,
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
        var_guard86: f64,
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
        var_t1__blk80: f64,
        var_t1__blk80_dn0: f64,
        var_t1__blk80_dn10: f64,
        var_t1__blk80_dn11: f64,
        var_t1__blk80_dn12: f64,
        var_t1__blk80_dn17: f64,
        var_t1__blk80_dn2: f64,
        var_t1__blk80_dn6: f64,
        var_t1__blk80_dn7: f64,
        var_t2__blk81: f64,
        var_t2__blk81_dn0: f64,
        var_t2__blk81_dn10: f64,
        var_t2__blk81_dn11: f64,
        var_t2__blk81_dn12: f64,
        var_t2__blk81_dn17: f64,
        var_t2__blk81_dn2: f64,
        var_t2__blk81_dn6: f64,
        var_t2__blk81_dn7: f64,
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
        var_dvth0__blk93_slot: &mut f64,
        var_dvth0__blk93_dn0_slot: &mut f64,
        var_dvth0__blk93_dn10_slot: &mut f64,
        var_dvth0__blk93_dn11_slot: &mut f64,
        var_dvth0__blk93_dn12_slot: &mut f64,
        var_dvth0__blk93_dn17_slot: &mut f64,
        var_dvth0__blk93_dn2_slot: &mut f64,
        var_dvth0__blk93_dn6_slot: &mut f64,
        var_dvth0__blk93_dn7_slot: &mut f64,
        var_dvth0__blk93_rv_slot: &mut f64,
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
        var_guard104_slot: &mut f64,
        var_guard104_rv_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard105_rv_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard106_rv_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard107_rv_slot: &mut f64,
        var_guard97_slot: &mut f64,
        var_guard97_rv_slot: &mut f64,
        var_t0__blk102_slot: &mut f64,
        var_t0__blk102_rv_slot: &mut f64,
        var_t0__blk87_slot: &mut f64,
        var_t0__blk87_dn0_slot: &mut f64,
        var_t0__blk87_dn10_slot: &mut f64,
        var_t0__blk87_dn11_slot: &mut f64,
        var_t0__blk87_dn12_slot: &mut f64,
        var_t0__blk87_dn17_slot: &mut f64,
        var_t0__blk87_dn2_slot: &mut f64,
        var_t0__blk87_dn6_slot: &mut f64,
        var_t0__blk87_dn7_slot: &mut f64,
        var_t0__blk87_rv_slot: &mut f64,
        var_t1__blk88_slot: &mut f64,
        var_t1__blk88_dn0_slot: &mut f64,
        var_t1__blk88_dn10_slot: &mut f64,
        var_t1__blk88_dn11_slot: &mut f64,
        var_t1__blk88_dn12_slot: &mut f64,
        var_t1__blk88_dn17_slot: &mut f64,
        var_t1__blk88_dn2_slot: &mut f64,
        var_t1__blk88_dn6_slot: &mut f64,
        var_t1__blk88_dn7_slot: &mut f64,
        var_t1__blk88_rv_slot: &mut f64,
        var_t1__blk94_slot: &mut f64,
        var_t1__blk94_dn0_slot: &mut f64,
        var_t1__blk94_dn10_slot: &mut f64,
        var_t1__blk94_dn11_slot: &mut f64,
        var_t1__blk94_dn12_slot: &mut f64,
        var_t1__blk94_dn17_slot: &mut f64,
        var_t1__blk94_dn2_slot: &mut f64,
        var_t1__blk94_dn6_slot: &mut f64,
        var_t1__blk94_dn7_slot: &mut f64,
        var_t1__blk94_rv_slot: &mut f64,
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
        var_t2__blk89_slot: &mut f64,
        var_t2__blk89_dn0_slot: &mut f64,
        var_t2__blk89_dn10_slot: &mut f64,
        var_t2__blk89_dn11_slot: &mut f64,
        var_t2__blk89_dn12_slot: &mut f64,
        var_t2__blk89_dn17_slot: &mut f64,
        var_t2__blk89_dn2_slot: &mut f64,
        var_t2__blk89_dn6_slot: &mut f64,
        var_t2__blk89_dn7_slot: &mut f64,
        var_t2__blk89_rv_slot: &mut f64,
        var_t2__blk95_slot: &mut f64,
        var_t2__blk95_rv_slot: &mut f64,
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
        var_t3__blk90_slot: &mut f64,
        var_t3__blk90_rv_slot: &mut f64,
        var_t3__blk96_slot: &mut f64,
        var_t3__blk96_rv_slot: &mut f64,
        var_t3__blk99_slot: &mut f64,
        var_t3__blk99_dn0_slot: &mut f64,
        var_t3__blk99_dn10_slot: &mut f64,
        var_t3__blk99_dn11_slot: &mut f64,
        var_t3__blk99_dn12_slot: &mut f64,
        var_t3__blk99_dn17_slot: &mut f64,
        var_t3__blk99_dn2_slot: &mut f64,
        var_t3__blk99_dn6_slot: &mut f64,
        var_t3__blk99_dn7_slot: &mut f64,
        var_t3__blk99_rv_slot: &mut f64,
        var_t4__blk91_slot: &mut f64,
        var_t4__blk91_dn0_slot: &mut f64,
        var_t4__blk91_dn10_slot: &mut f64,
        var_t4__blk91_dn11_slot: &mut f64,
        var_t4__blk91_dn12_slot: &mut f64,
        var_t4__blk91_dn17_slot: &mut f64,
        var_t4__blk91_dn2_slot: &mut f64,
        var_t4__blk91_dn6_slot: &mut f64,
        var_t4__blk91_dn7_slot: &mut f64,
        var_t4__blk91_rv_slot: &mut f64,
        var_t5__blk100_slot: &mut f64,
        var_t5__blk100_dn0_slot: &mut f64,
        var_t5__blk100_dn10_slot: &mut f64,
        var_t5__blk100_dn11_slot: &mut f64,
        var_t5__blk100_dn12_slot: &mut f64,
        var_t5__blk100_dn17_slot: &mut f64,
        var_t5__blk100_dn2_slot: &mut f64,
        var_t5__blk100_dn6_slot: &mut f64,
        var_t5__blk100_dn7_slot: &mut f64,
        var_t5__blk100_rv_slot: &mut f64,
        var_t5__blk84_slot: &mut f64,
        var_t5__blk84_dn0_slot: &mut f64,
        var_t5__blk84_dn10_slot: &mut f64,
        var_t5__blk84_dn11_slot: &mut f64,
        var_t5__blk84_dn12_slot: &mut f64,
        var_t5__blk84_dn17_slot: &mut f64,
        var_t5__blk84_dn2_slot: &mut f64,
        var_t5__blk84_dn6_slot: &mut f64,
        var_t5__blk84_dn7_slot: &mut f64,
        var_t5__blk84_rv_slot: &mut f64,
        var_t5__blk92_slot: &mut f64,
        var_t5__blk92_dn0_slot: &mut f64,
        var_t5__blk92_dn10_slot: &mut f64,
        var_t5__blk92_dn11_slot: &mut f64,
        var_t5__blk92_dn12_slot: &mut f64,
        var_t5__blk92_dn17_slot: &mut f64,
        var_t5__blk92_dn2_slot: &mut f64,
        var_t5__blk92_dn6_slot: &mut f64,
        var_t5__blk92_dn7_slot: &mut f64,
        var_t5__blk92_rv_slot: &mut f64,
        var_t7__blk101_slot: &mut f64,
        var_t7__blk101_dn0_slot: &mut f64,
        var_t7__blk101_dn10_slot: &mut f64,
        var_t7__blk101_dn11_slot: &mut f64,
        var_t7__blk101_dn12_slot: &mut f64,
        var_t7__blk101_dn17_slot: &mut f64,
        var_t7__blk101_dn2_slot: &mut f64,
        var_t7__blk101_dn6_slot: &mut f64,
        var_t7__blk101_dn7_slot: &mut f64,
        var_t7__blk101_rv_slot: &mut f64,
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
        let mut var_dvth0__blk93: f64 = *var_dvth0__blk93_slot;
        let mut var_dvth0__blk93_dn0: f64 = *var_dvth0__blk93_dn0_slot;
        let mut var_dvth0__blk93_dn10: f64 = *var_dvth0__blk93_dn10_slot;
        let mut var_dvth0__blk93_dn11: f64 = *var_dvth0__blk93_dn11_slot;
        let mut var_dvth0__blk93_dn12: f64 = *var_dvth0__blk93_dn12_slot;
        let mut var_dvth0__blk93_dn17: f64 = *var_dvth0__blk93_dn17_slot;
        let mut var_dvth0__blk93_dn2: f64 = *var_dvth0__blk93_dn2_slot;
        let mut var_dvth0__blk93_dn6: f64 = *var_dvth0__blk93_dn6_slot;
        let mut var_dvth0__blk93_dn7: f64 = *var_dvth0__blk93_dn7_slot;
        let mut var_dvth0__blk93_rv: f64 = *var_dvth0__blk93_rv_slot;
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
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard104_rv: f64 = *var_guard104_rv_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard105_rv: f64 = *var_guard105_rv_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard106_rv: f64 = *var_guard106_rv_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard107_rv: f64 = *var_guard107_rv_slot;
        let mut var_guard97: f64 = *var_guard97_slot;
        let mut var_guard97_rv: f64 = *var_guard97_rv_slot;
        let mut var_t0__blk102: f64 = *var_t0__blk102_slot;
        let mut var_t0__blk102_rv: f64 = *var_t0__blk102_rv_slot;
        let mut var_t0__blk87: f64 = *var_t0__blk87_slot;
        let mut var_t0__blk87_dn0: f64 = *var_t0__blk87_dn0_slot;
        let mut var_t0__blk87_dn10: f64 = *var_t0__blk87_dn10_slot;
        let mut var_t0__blk87_dn11: f64 = *var_t0__blk87_dn11_slot;
        let mut var_t0__blk87_dn12: f64 = *var_t0__blk87_dn12_slot;
        let mut var_t0__blk87_dn17: f64 = *var_t0__blk87_dn17_slot;
        let mut var_t0__blk87_dn2: f64 = *var_t0__blk87_dn2_slot;
        let mut var_t0__blk87_dn6: f64 = *var_t0__blk87_dn6_slot;
        let mut var_t0__blk87_dn7: f64 = *var_t0__blk87_dn7_slot;
        let mut var_t0__blk87_rv: f64 = *var_t0__blk87_rv_slot;
        let mut var_t1__blk88: f64 = *var_t1__blk88_slot;
        let mut var_t1__blk88_dn0: f64 = *var_t1__blk88_dn0_slot;
        let mut var_t1__blk88_dn10: f64 = *var_t1__blk88_dn10_slot;
        let mut var_t1__blk88_dn11: f64 = *var_t1__blk88_dn11_slot;
        let mut var_t1__blk88_dn12: f64 = *var_t1__blk88_dn12_slot;
        let mut var_t1__blk88_dn17: f64 = *var_t1__blk88_dn17_slot;
        let mut var_t1__blk88_dn2: f64 = *var_t1__blk88_dn2_slot;
        let mut var_t1__blk88_dn6: f64 = *var_t1__blk88_dn6_slot;
        let mut var_t1__blk88_dn7: f64 = *var_t1__blk88_dn7_slot;
        let mut var_t1__blk88_rv: f64 = *var_t1__blk88_rv_slot;
        let mut var_t1__blk94: f64 = *var_t1__blk94_slot;
        let mut var_t1__blk94_dn0: f64 = *var_t1__blk94_dn0_slot;
        let mut var_t1__blk94_dn10: f64 = *var_t1__blk94_dn10_slot;
        let mut var_t1__blk94_dn11: f64 = *var_t1__blk94_dn11_slot;
        let mut var_t1__blk94_dn12: f64 = *var_t1__blk94_dn12_slot;
        let mut var_t1__blk94_dn17: f64 = *var_t1__blk94_dn17_slot;
        let mut var_t1__blk94_dn2: f64 = *var_t1__blk94_dn2_slot;
        let mut var_t1__blk94_dn6: f64 = *var_t1__blk94_dn6_slot;
        let mut var_t1__blk94_dn7: f64 = *var_t1__blk94_dn7_slot;
        let mut var_t1__blk94_rv: f64 = *var_t1__blk94_rv_slot;
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
        let mut var_t2__blk89: f64 = *var_t2__blk89_slot;
        let mut var_t2__blk89_dn0: f64 = *var_t2__blk89_dn0_slot;
        let mut var_t2__blk89_dn10: f64 = *var_t2__blk89_dn10_slot;
        let mut var_t2__blk89_dn11: f64 = *var_t2__blk89_dn11_slot;
        let mut var_t2__blk89_dn12: f64 = *var_t2__blk89_dn12_slot;
        let mut var_t2__blk89_dn17: f64 = *var_t2__blk89_dn17_slot;
        let mut var_t2__blk89_dn2: f64 = *var_t2__blk89_dn2_slot;
        let mut var_t2__blk89_dn6: f64 = *var_t2__blk89_dn6_slot;
        let mut var_t2__blk89_dn7: f64 = *var_t2__blk89_dn7_slot;
        let mut var_t2__blk89_rv: f64 = *var_t2__blk89_rv_slot;
        let mut var_t2__blk95: f64 = *var_t2__blk95_slot;
        let mut var_t2__blk95_rv: f64 = *var_t2__blk95_rv_slot;
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
        let mut var_t3__blk90: f64 = *var_t3__blk90_slot;
        let mut var_t3__blk90_rv: f64 = *var_t3__blk90_rv_slot;
        let mut var_t3__blk96: f64 = *var_t3__blk96_slot;
        let mut var_t3__blk96_rv: f64 = *var_t3__blk96_rv_slot;
        let mut var_t3__blk99: f64 = *var_t3__blk99_slot;
        let mut var_t3__blk99_dn0: f64 = *var_t3__blk99_dn0_slot;
        let mut var_t3__blk99_dn10: f64 = *var_t3__blk99_dn10_slot;
        let mut var_t3__blk99_dn11: f64 = *var_t3__blk99_dn11_slot;
        let mut var_t3__blk99_dn12: f64 = *var_t3__blk99_dn12_slot;
        let mut var_t3__blk99_dn17: f64 = *var_t3__blk99_dn17_slot;
        let mut var_t3__blk99_dn2: f64 = *var_t3__blk99_dn2_slot;
        let mut var_t3__blk99_dn6: f64 = *var_t3__blk99_dn6_slot;
        let mut var_t3__blk99_dn7: f64 = *var_t3__blk99_dn7_slot;
        let mut var_t3__blk99_rv: f64 = *var_t3__blk99_rv_slot;
        let mut var_t4__blk91: f64 = *var_t4__blk91_slot;
        let mut var_t4__blk91_dn0: f64 = *var_t4__blk91_dn0_slot;
        let mut var_t4__blk91_dn10: f64 = *var_t4__blk91_dn10_slot;
        let mut var_t4__blk91_dn11: f64 = *var_t4__blk91_dn11_slot;
        let mut var_t4__blk91_dn12: f64 = *var_t4__blk91_dn12_slot;
        let mut var_t4__blk91_dn17: f64 = *var_t4__blk91_dn17_slot;
        let mut var_t4__blk91_dn2: f64 = *var_t4__blk91_dn2_slot;
        let mut var_t4__blk91_dn6: f64 = *var_t4__blk91_dn6_slot;
        let mut var_t4__blk91_dn7: f64 = *var_t4__blk91_dn7_slot;
        let mut var_t4__blk91_rv: f64 = *var_t4__blk91_rv_slot;
        let mut var_t5__blk100: f64 = *var_t5__blk100_slot;
        let mut var_t5__blk100_dn0: f64 = *var_t5__blk100_dn0_slot;
        let mut var_t5__blk100_dn10: f64 = *var_t5__blk100_dn10_slot;
        let mut var_t5__blk100_dn11: f64 = *var_t5__blk100_dn11_slot;
        let mut var_t5__blk100_dn12: f64 = *var_t5__blk100_dn12_slot;
        let mut var_t5__blk100_dn17: f64 = *var_t5__blk100_dn17_slot;
        let mut var_t5__blk100_dn2: f64 = *var_t5__blk100_dn2_slot;
        let mut var_t5__blk100_dn6: f64 = *var_t5__blk100_dn6_slot;
        let mut var_t5__blk100_dn7: f64 = *var_t5__blk100_dn7_slot;
        let mut var_t5__blk100_rv: f64 = *var_t5__blk100_rv_slot;
        let mut var_t5__blk84: f64 = *var_t5__blk84_slot;
        let mut var_t5__blk84_dn0: f64 = *var_t5__blk84_dn0_slot;
        let mut var_t5__blk84_dn10: f64 = *var_t5__blk84_dn10_slot;
        let mut var_t5__blk84_dn11: f64 = *var_t5__blk84_dn11_slot;
        let mut var_t5__blk84_dn12: f64 = *var_t5__blk84_dn12_slot;
        let mut var_t5__blk84_dn17: f64 = *var_t5__blk84_dn17_slot;
        let mut var_t5__blk84_dn2: f64 = *var_t5__blk84_dn2_slot;
        let mut var_t5__blk84_dn6: f64 = *var_t5__blk84_dn6_slot;
        let mut var_t5__blk84_dn7: f64 = *var_t5__blk84_dn7_slot;
        let mut var_t5__blk84_rv: f64 = *var_t5__blk84_rv_slot;
        let mut var_t5__blk92: f64 = *var_t5__blk92_slot;
        let mut var_t5__blk92_dn0: f64 = *var_t5__blk92_dn0_slot;
        let mut var_t5__blk92_dn10: f64 = *var_t5__blk92_dn10_slot;
        let mut var_t5__blk92_dn11: f64 = *var_t5__blk92_dn11_slot;
        let mut var_t5__blk92_dn12: f64 = *var_t5__blk92_dn12_slot;
        let mut var_t5__blk92_dn17: f64 = *var_t5__blk92_dn17_slot;
        let mut var_t5__blk92_dn2: f64 = *var_t5__blk92_dn2_slot;
        let mut var_t5__blk92_dn6: f64 = *var_t5__blk92_dn6_slot;
        let mut var_t5__blk92_dn7: f64 = *var_t5__blk92_dn7_slot;
        let mut var_t5__blk92_rv: f64 = *var_t5__blk92_rv_slot;
        let mut var_t7__blk101: f64 = *var_t7__blk101_slot;
        let mut var_t7__blk101_dn0: f64 = *var_t7__blk101_dn0_slot;
        let mut var_t7__blk101_dn10: f64 = *var_t7__blk101_dn10_slot;
        let mut var_t7__blk101_dn11: f64 = *var_t7__blk101_dn11_slot;
        let mut var_t7__blk101_dn12: f64 = *var_t7__blk101_dn12_slot;
        let mut var_t7__blk101_dn17: f64 = *var_t7__blk101_dn17_slot;
        let mut var_t7__blk101_dn2: f64 = *var_t7__blk101_dn2_slot;
        let mut var_t7__blk101_dn6: f64 = *var_t7__blk101_dn6_slot;
        let mut var_t7__blk101_dn7: f64 = *var_t7__blk101_dn7_slot;
        let mut var_t7__blk101_rv: f64 = *var_t7__blk101_rv_slot;
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

        let (assign6450_e4302, assign6450_e4302_d_n0, assign6450_e4302_d_n2, assign6450_e4302_d_n6, assign6450_e4302_d_n7, assign6450_e4302_d_n10, assign6450_e4302_d_n11, assign6450_e4302_d_n12, assign6450_e4302_d_n17,) = {
    if (var_guard86 != 0.0) {
        (var_uc_scp2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t5__blk84, var_t5__blk84_dn0, var_t5__blk84_dn2, var_t5__blk84_dn6, var_t5__blk84_dn7, var_t5__blk84_dn10, var_t5__blk84_dn11, var_t5__blk84_dn12, var_t5__blk84_dn17,)
    }
};
        var_t5__blk84 = assign6450_e4302;
        var_t5__blk84_dn0 = assign6450_e4302_d_n0;
        var_t5__blk84_dn2 = assign6450_e4302_d_n2;
        var_t5__blk84_dn6 = assign6450_e4302_d_n6;
        var_t5__blk84_dn7 = assign6450_e4302_d_n7;
        var_t5__blk84_dn10 = assign6450_e4302_d_n10;
        var_t5__blk84_dn11 = assign6450_e4302_d_n11;
        var_t5__blk84_dn12 = assign6450_e4302_d_n12;
        var_t5__blk84_dn17 = assign6450_e4302_d_n17;
        var_t5__blk84_rv = 0.0;

        let (assign6460_e4310, assign6460_e4310_d_n0, assign6460_e4310_d_n2, assign6460_e4310_d_n6, assign6460_e4310_d_n7, assign6460_e4310_d_n10, assign6460_e4310_d_n11, assign6460_e4310_d_n12, assign6460_e4310_d_n17,) = {
    if (var_guard86 != 0.0) {
        let assign6460_e4307: f64 = (var_t5__blk84 * var_vdsz);
        let assign6460_e4308: f64 = (var_t2__blk81 + assign6460_e4307);
        (assign6460_e4308, (var_t2__blk81_dn0 + ((var_t5__blk84_dn0 * var_vdsz) + (var_t5__blk84 * var_vdsz_dn0))), (var_t2__blk81_dn2 + ((var_t5__blk84_dn2 * var_vdsz) + (var_t5__blk84 * var_vdsz_dn2))), (var_t2__blk81_dn6 + ((var_t5__blk84_dn6 * var_vdsz) + (var_t5__blk84 * var_vdsz_dn6))), (var_t2__blk81_dn7 + ((var_t5__blk84_dn7 * var_vdsz) + (var_t5__blk84 * var_vdsz_dn7))), (var_t2__blk81_dn10 + ((var_t5__blk84_dn10 * var_vdsz) + (var_t5__blk84 * var_vdsz_dn10))), (var_t2__blk81_dn11 + ((var_t5__blk84_dn11 * var_vdsz) + (var_t5__blk84 * var_vdsz_dn11))), (var_t2__blk81_dn12 + ((var_t5__blk84_dn12 * var_vdsz) + (var_t5__blk84 * var_vdsz_dn12))), (var_t2__blk81_dn17 + ((var_t5__blk84_dn17 * var_vdsz) + (var_t5__blk84 * var_vdsz_dn17))),)
    } else {
        (var_t3__blk82, var_t3__blk82_dn0, var_t3__blk82_dn2, var_t3__blk82_dn6, var_t3__blk82_dn7, var_t3__blk82_dn10, var_t3__blk82_dn11, var_t3__blk82_dn12, var_t3__blk82_dn17,)
    }
};
        var_t3__blk82 = assign6460_e4310;
        var_t3__blk82_dn0 = assign6460_e4310_d_n0;
        var_t3__blk82_dn2 = assign6460_e4310_d_n2;
        var_t3__blk82_dn6 = assign6460_e4310_d_n6;
        var_t3__blk82_dn7 = assign6460_e4310_d_n7;
        var_t3__blk82_dn10 = assign6460_e4310_d_n10;
        var_t3__blk82_dn11 = assign6460_e4310_d_n11;
        var_t3__blk82_dn12 = assign6460_e4310_d_n12;
        var_t3__blk82_dn17 = assign6460_e4310_d_n17;
        var_t3__blk82_rv = 0.0;

        let (assign6470_e4318, assign6470_e4318_d_n0, assign6470_e4318_d_n2, assign6470_e4318_d_n6, assign6470_e4318_d_n7, assign6470_e4318_d_n10, assign6470_e4318_d_n11, assign6470_e4318_d_n12, assign6470_e4318_d_n17,) = {
    if (var_guard86 != 0.0) {
        let assign6470_e4314: f64 = (var_t1__blk80 * var_dvth0__blk85);
        let assign6470_e4316: f64 = (assign6470_e4314 * var_t3__blk82);
        (assign6470_e4316, ((((var_t1__blk80_dn0 * var_dvth0__blk85) + (var_t1__blk80 * var_dvth0__blk85_dn0)) * var_t3__blk82) + (assign6470_e4314 * var_t3__blk82_dn0)), ((((var_t1__blk80_dn2 * var_dvth0__blk85) + (var_t1__blk80 * var_dvth0__blk85_dn2)) * var_t3__blk82) + (assign6470_e4314 * var_t3__blk82_dn2)), ((((var_t1__blk80_dn6 * var_dvth0__blk85) + (var_t1__blk80 * var_dvth0__blk85_dn6)) * var_t3__blk82) + (assign6470_e4314 * var_t3__blk82_dn6)), ((((var_t1__blk80_dn7 * var_dvth0__blk85) + (var_t1__blk80 * var_dvth0__blk85_dn7)) * var_t3__blk82) + (assign6470_e4314 * var_t3__blk82_dn7)), ((((var_t1__blk80_dn10 * var_dvth0__blk85) + (var_t1__blk80 * var_dvth0__blk85_dn10)) * var_t3__blk82) + (assign6470_e4314 * var_t3__blk82_dn10)), ((((var_t1__blk80_dn11 * var_dvth0__blk85) + (var_t1__blk80 * var_dvth0__blk85_dn11)) * var_t3__blk82) + (assign6470_e4314 * var_t3__blk82_dn11)), ((((var_t1__blk80_dn12 * var_dvth0__blk85) + (var_t1__blk80 * var_dvth0__blk85_dn12)) * var_t3__blk82) + (assign6470_e4314 * var_t3__blk82_dn12)), ((((var_t1__blk80_dn17 * var_dvth0__blk85) + (var_t1__blk80 * var_dvth0__blk85_dn17)) * var_t3__blk82) + (assign6470_e4314 * var_t3__blk82_dn17)),)
    } else {
        (var_dvthlp, var_dvthlp_dn0, var_dvthlp_dn2, var_dvthlp_dn6, var_dvthlp_dn7, var_dvthlp_dn10, var_dvthlp_dn11, var_dvthlp_dn12, var_dvthlp_dn17,)
    }
};
        var_dvthlp = assign6470_e4318;
        var_dvthlp_dn0 = assign6470_e4318_d_n0;
        var_dvthlp_dn2 = assign6470_e4318_d_n2;
        var_dvthlp_dn6 = assign6470_e4318_d_n6;
        var_dvthlp_dn7 = assign6470_e4318_d_n7;
        var_dvthlp_dn10 = assign6470_e4318_d_n10;
        var_dvthlp_dn11 = assign6470_e4318_d_n11;
        var_dvthlp_dn12 = assign6470_e4318_d_n12;
        var_dvthlp_dn17 = assign6470_e4318_d_n17;
        var_dvthlp_rv = 0.0;

        let (assign6480_e4323, assign6480_e4323_d_n0, assign6480_e4323_d_n2, assign6480_e4323_d_n6, assign6480_e4323_d_n7, assign6480_e4323_d_n10, assign6480_e4323_d_n11, assign6480_e4323_d_n12, assign6480_e4323_d_n17,) = {
    if (var_guard86 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dvthlp, var_dvthlp_dn0, var_dvthlp_dn2, var_dvthlp_dn6, var_dvthlp_dn7, var_dvthlp_dn10, var_dvthlp_dn11, var_dvthlp_dn12, var_dvthlp_dn17,)
    }
};
        var_dvthlp = assign6480_e4323;
        var_dvthlp_dn0 = assign6480_e4323_d_n0;
        var_dvthlp_dn2 = assign6480_e4323_d_n2;
        var_dvthlp_dn6 = assign6480_e4323_d_n6;
        var_dvthlp_dn7 = assign6480_e4323_d_n7;
        var_dvthlp_dn10 = assign6480_e4323_d_n10;
        var_dvthlp_dn11 = assign6480_e4323_d_n11;
        var_dvthlp_dn12 = assign6480_e4323_d_n12;
        var_dvthlp_dn17 = assign6480_e4323_d_n17;
        var_dvthlp_rv = 0.0;

        let assign6490_e4326: f64 = (1.034943e-10 * var_wd0);
        let assign6490_e4328: f64 = (assign6490_e4326 * 2.0);
        var_t0__blk87 = assign6490_e4328;
        var_t0__blk87_dn0 = ((1.034943e-10 * var_wd0_dn0) * 2.0);
        var_t0__blk87_dn2 = ((1.034943e-10 * var_wd0_dn2) * 2.0);
        var_t0__blk87_dn6 = ((1.034943e-10 * var_wd0_dn6) * 2.0);
        var_t0__blk87_dn7 = ((1.034943e-10 * var_wd0_dn7) * 2.0);
        var_t0__blk87_dn10 = ((1.034943e-10 * var_wd0_dn10) * 2.0);
        var_t0__blk87_dn11 = ((1.034943e-10 * var_wd0_dn11) * 2.0);
        var_t0__blk87_dn12 = ((1.034943e-10 * var_wd0_dn12) * 2.0);
        var_t0__blk87_dn17 = ((1.034943e-10 * var_wd0_dn17) * 2.0);
        var_t0__blk87_rv = 0.0;

        let assign6500_e4331: f64 = (var_c_fox_inv * var_t0__blk87);
        var_t1__blk88 = assign6500_e4331;
        var_t1__blk88_dn0 = ((var_c_fox_inv_dn0 * var_t0__blk87) + (var_c_fox_inv * var_t0__blk87_dn0));
        var_t1__blk88_dn2 = ((var_c_fox_inv_dn2 * var_t0__blk87) + (var_c_fox_inv * var_t0__blk87_dn2));
        var_t1__blk88_dn6 = ((var_c_fox_inv_dn6 * var_t0__blk87) + (var_c_fox_inv * var_t0__blk87_dn6));
        var_t1__blk88_dn7 = ((var_c_fox_inv_dn7 * var_t0__blk87) + (var_c_fox_inv * var_t0__blk87_dn7));
        var_t1__blk88_dn10 = ((var_c_fox_inv_dn10 * var_t0__blk87) + (var_c_fox_inv * var_t0__blk87_dn10));
        var_t1__blk88_dn11 = ((var_c_fox_inv_dn11 * var_t0__blk87) + (var_c_fox_inv * var_t0__blk87_dn11));
        var_t1__blk88_dn12 = ((var_c_fox_inv_dn12 * var_t0__blk87) + (var_c_fox_inv * var_t0__blk87_dn12));
        var_t1__blk88_dn17 = ((var_c_fox_inv_dn17 * var_t0__blk87) + (var_c_fox_inv * var_t0__blk87_dn17));
        var_t1__blk88_rv = 0.0;

        let assign6510_e4334: f64 = (p.p69 - var_pb20b);
        var_t2__blk89 = assign6510_e4334;
        var_t2__blk89_dn0 = (-var_pb20b_dn0);
        var_t2__blk89_dn2 = (-var_pb20b_dn2);
        var_t2__blk89_dn6 = (-var_pb20b_dn6);
        var_t2__blk89_dn7 = (-var_pb20b_dn7);
        var_t2__blk89_dn10 = (-var_pb20b_dn10);
        var_t2__blk89_dn11 = (-var_pb20b_dn11);
        var_t2__blk89_dn12 = (-var_pb20b_dn12);
        var_t2__blk89_dn17 = (-var_pb20b_dn17);
        var_t2__blk89_rv = 0.0;

        let assign6520_e4337: f64 = (var_lgleff - p.p71);
        var_t3__blk90 = assign6520_e4337;
        var_t3__blk90_rv = 0.0;

        let assign6530_e4341: f64 = (var_t3__blk90 * var_t3__blk90);
        let assign6530_e4342: f64 = (1.0 / assign6530_e4341);
        var_t4__blk91 = assign6530_e4342;
        var_t4__blk91_dn0 = 0.0;
        var_t4__blk91_dn2 = 0.0;
        var_t4__blk91_dn6 = 0.0;
        var_t4__blk91_dn7 = 0.0;
        var_t4__blk91_dn10 = 0.0;
        var_t4__blk91_dn11 = 0.0;
        var_t4__blk91_dn12 = 0.0;
        var_t4__blk91_dn17 = 0.0;
        var_t4__blk91_rv = 0.0;

        let assign6540_e4345: f64 = (var_t1__blk88 * var_t2__blk89);
        let assign6540_e4347: f64 = (assign6540_e4345 * var_t4__blk91);
        var_dvth0__blk93 = assign6540_e4347;
        var_dvth0__blk93_dn0 = ((((var_t1__blk88_dn0 * var_t2__blk89) + (var_t1__blk88 * var_t2__blk89_dn0)) * var_t4__blk91) + (assign6540_e4345 * var_t4__blk91_dn0));
        var_dvth0__blk93_dn2 = ((((var_t1__blk88_dn2 * var_t2__blk89) + (var_t1__blk88 * var_t2__blk89_dn2)) * var_t4__blk91) + (assign6540_e4345 * var_t4__blk91_dn2));
        var_dvth0__blk93_dn6 = ((((var_t1__blk88_dn6 * var_t2__blk89) + (var_t1__blk88 * var_t2__blk89_dn6)) * var_t4__blk91) + (assign6540_e4345 * var_t4__blk91_dn6));
        var_dvth0__blk93_dn7 = ((((var_t1__blk88_dn7 * var_t2__blk89) + (var_t1__blk88 * var_t2__blk89_dn7)) * var_t4__blk91) + (assign6540_e4345 * var_t4__blk91_dn7));
        var_dvth0__blk93_dn10 = ((((var_t1__blk88_dn10 * var_t2__blk89) + (var_t1__blk88 * var_t2__blk89_dn10)) * var_t4__blk91) + (assign6540_e4345 * var_t4__blk91_dn10));
        var_dvth0__blk93_dn11 = ((((var_t1__blk88_dn11 * var_t2__blk89) + (var_t1__blk88 * var_t2__blk89_dn11)) * var_t4__blk91) + (assign6540_e4345 * var_t4__blk91_dn11));
        var_dvth0__blk93_dn12 = ((((var_t1__blk88_dn12 * var_t2__blk89) + (var_t1__blk88 * var_t2__blk89_dn12)) * var_t4__blk91) + (assign6540_e4345 * var_t4__blk91_dn12));
        var_dvth0__blk93_dn17 = ((((var_t1__blk88_dn17 * var_t2__blk89) + (var_t1__blk88 * var_t2__blk89_dn17)) * var_t4__blk91) + (assign6540_e4345 * var_t4__blk91_dn17));
        var_dvth0__blk93_rv = 0.0;

        let assign6550_e4350: f64 = (var_uc_sc3 / var_lgleff);
        var_t1__blk88 = assign6550_e4350;
        var_t1__blk88_dn0 = 0.0;
        var_t1__blk88_dn2 = 0.0;
        var_t1__blk88_dn6 = 0.0;
        var_t1__blk88_dn7 = 0.0;
        var_t1__blk88_dn10 = 0.0;
        var_t1__blk88_dn11 = 0.0;
        var_t1__blk88_dn12 = 0.0;
        var_t1__blk88_dn17 = 0.0;
        var_t1__blk88_rv = 0.0;

        let assign6560_e4354: f64 = (var_t1__blk88 * var_pbsum);
        let assign6560_e4355: f64 = (p.p83 + assign6560_e4354);
        var_t4__blk91 = assign6560_e4355;
        var_t4__blk91_dn0 = ((var_t1__blk88_dn0 * var_pbsum) + (var_t1__blk88 * var_pbsum_dn0));
        var_t4__blk91_dn2 = ((var_t1__blk88_dn2 * var_pbsum) + (var_t1__blk88 * var_pbsum_dn2));
        var_t4__blk91_dn6 = ((var_t1__blk88_dn6 * var_pbsum) + (var_t1__blk88 * var_pbsum_dn6));
        var_t4__blk91_dn7 = ((var_t1__blk88_dn7 * var_pbsum) + (var_t1__blk88 * var_pbsum_dn7));
        var_t4__blk91_dn10 = ((var_t1__blk88_dn10 * var_pbsum) + (var_t1__blk88 * var_pbsum_dn10));
        var_t4__blk91_dn11 = ((var_t1__blk88_dn11 * var_pbsum) + (var_t1__blk88 * var_pbsum_dn11));
        var_t4__blk91_dn12 = ((var_t1__blk88_dn12 * var_pbsum) + (var_t1__blk88 * var_pbsum_dn12));
        var_t4__blk91_dn17 = ((var_t1__blk88_dn17 * var_pbsum) + (var_t1__blk88 * var_pbsum_dn17));
        var_t4__blk91_rv = 0.0;

        let assign6570_e4359: f64 = (var_uc_sc2 * var_vdsz);
        let assign6570_e4360: f64 = (var_t4__blk91 + assign6570_e4359);
        var_t5__blk92 = assign6570_e4360;
        var_t5__blk92_dn0 = (var_t4__blk91_dn0 + (var_uc_sc2 * var_vdsz_dn0));
        var_t5__blk92_dn2 = (var_t4__blk91_dn2 + (var_uc_sc2 * var_vdsz_dn2));
        var_t5__blk92_dn6 = (var_t4__blk91_dn6 + (var_uc_sc2 * var_vdsz_dn6));
        var_t5__blk92_dn7 = (var_t4__blk91_dn7 + (var_uc_sc2 * var_vdsz_dn7));
        var_t5__blk92_dn10 = (var_t4__blk91_dn10 + (var_uc_sc2 * var_vdsz_dn10));
        var_t5__blk92_dn11 = (var_t4__blk91_dn11 + (var_uc_sc2 * var_vdsz_dn11));
        var_t5__blk92_dn12 = (var_t4__blk91_dn12 + (var_uc_sc2 * var_vdsz_dn12));
        var_t5__blk92_dn17 = (var_t4__blk91_dn17 + (var_uc_sc2 * var_vdsz_dn17));
        var_t5__blk92_rv = 0.0;

        let assign6580_e4363: f64 = (var_dvth0__blk93 * var_t5__blk92);
        var_dvthsc = assign6580_e4363;
        var_dvthsc_dn0 = ((var_dvth0__blk93_dn0 * var_t5__blk92) + (var_dvth0__blk93 * var_t5__blk92_dn0));
        var_dvthsc_dn2 = ((var_dvth0__blk93_dn2 * var_t5__blk92) + (var_dvth0__blk93 * var_t5__blk92_dn2));
        var_dvthsc_dn6 = ((var_dvth0__blk93_dn6 * var_t5__blk92) + (var_dvth0__blk93 * var_t5__blk92_dn6));
        var_dvthsc_dn7 = ((var_dvth0__blk93_dn7 * var_t5__blk92) + (var_dvth0__blk93 * var_t5__blk92_dn7));
        var_dvthsc_dn10 = ((var_dvth0__blk93_dn10 * var_t5__blk92) + (var_dvth0__blk93 * var_t5__blk92_dn10));
        var_dvthsc_dn11 = ((var_dvth0__blk93_dn11 * var_t5__blk92) + (var_dvth0__blk93 * var_t5__blk92_dn11));
        var_dvthsc_dn12 = ((var_dvth0__blk93_dn12 * var_t5__blk92) + (var_dvth0__blk93 * var_t5__blk92_dn12));
        var_dvthsc_dn17 = ((var_dvth0__blk93_dn17 * var_t5__blk92) + (var_dvth0__blk93 * var_t5__blk92_dn17));
        var_dvthsc_rv = 0.0;

        let assign6590_e4366: f64 = if p.p86 > 0.0 { 1.0 } else { 0.0 };
        var_guard97 = assign6590_e4366;
        var_guard97_rv = 0.0;

        let (assign6600_e4380, assign6600_e4380_d_n0, assign6600_e4380_d_n2, assign6600_e4380_d_n6, assign6600_e4380_d_n7, assign6600_e4380_d_n10, assign6600_e4380_d_n11, assign6600_e4380_d_n12, assign6600_e4380_d_n17,) = {
    if (var_guard97 != 0.0) {
        let assign6600_e4370: f64 = (var_eg + var_pb2);
        let assign6600_e4373: f64 = (2.0 * p.p88);
        let assign6600_e4374: f64 = (assign6600_e4370 - assign6600_e4373);
        let assign6600_e4377: f64 = (p.p87 * var_vdsz);
        let assign6600_e4378: f64 = (assign6600_e4374 + assign6600_e4377);
        (assign6600_e4378, ((var_eg_dn0 + var_pb2_dn0) + (p.p87 * var_vdsz_dn0)), ((var_eg_dn2 + var_pb2_dn2) + (p.p87 * var_vdsz_dn2)), ((var_eg_dn6 + var_pb2_dn6) + (p.p87 * var_vdsz_dn6)), ((var_eg_dn7 + var_pb2_dn7) + (p.p87 * var_vdsz_dn7)), ((var_eg_dn10 + var_pb2_dn10) + (p.p87 * var_vdsz_dn10)), ((var_eg_dn11 + var_pb2_dn11) + (p.p87 * var_vdsz_dn11)), ((var_eg_dn12 + var_pb2_dn12) + (p.p87 * var_vdsz_dn12)), ((var_eg_dn17 + var_pb2_dn17) + (p.p87 * var_vdsz_dn17)),)
    } else {
        (var_t1__blk94, var_t1__blk94_dn0, var_t1__blk94_dn2, var_t1__blk94_dn6, var_t1__blk94_dn7, var_t1__blk94_dn10, var_t1__blk94_dn11, var_t1__blk94_dn12, var_t1__blk94_dn17,)
    }
};
        var_t1__blk94 = assign6600_e4380;
        var_t1__blk94_dn0 = assign6600_e4380_d_n0;
        var_t1__blk94_dn2 = assign6600_e4380_d_n2;
        var_t1__blk94_dn6 = assign6600_e4380_d_n6;
        var_t1__blk94_dn7 = assign6600_e4380_d_n7;
        var_t1__blk94_dn10 = assign6600_e4380_d_n10;
        var_t1__blk94_dn11 = assign6600_e4380_d_n11;
        var_t1__blk94_dn12 = assign6600_e4380_d_n12;
        var_t1__blk94_dn17 = assign6600_e4380_d_n17;
        var_t1__blk94_rv = 0.0;

        let (assign6610_e4388,) = {
    if (var_guard97 != 0.0) {
        let assign6610_e4384: f64 = (var_lgleff * 0.5);
        let assign6610_e4386: f64 = (assign6610_e4384 + var_mks_parl1);
        (assign6610_e4386,)
    } else {
        (var_t2__blk95,)
    }
};
        var_t2__blk95 = assign6610_e4388;
        var_t2__blk95_rv = 0.0;

        let (assign6620_e4396,) = {
    if (var_guard97 != 0.0) {
        let assign6620_e4392: f64 = (p.p86 * p.p237);
        let assign6620_e4394: f64 = (assign6620_e4392 / var_t2__blk95);
        (assign6620_e4394,)
    } else {
        (var_t3__blk96,)
    }
};
        var_t3__blk96 = assign6620_e4396;
        var_t3__blk96_rv = 0.0;

        let (assign6630_e4402, assign6630_e4402_d_n0, assign6630_e4402_d_n2, assign6630_e4402_d_n6, assign6630_e4402_d_n7, assign6630_e4402_d_n10, assign6630_e4402_d_n11, assign6630_e4402_d_n12, assign6630_e4402_d_n17,) = {
    if (var_guard97 != 0.0) {
        let assign6630_e4400: f64 = (var_t1__blk94 * var_t3__blk96);
        (assign6630_e4400, (var_t1__blk94_dn0 * var_t3__blk96), (var_t1__blk94_dn2 * var_t3__blk96), (var_t1__blk94_dn6 * var_t3__blk96), (var_t1__blk94_dn7 * var_t3__blk96), (var_t1__blk94_dn10 * var_t3__blk96), (var_t1__blk94_dn11 * var_t3__blk96), (var_t1__blk94_dn12 * var_t3__blk96), (var_t1__blk94_dn17 * var_t3__blk96),)
    } else {
        (var_dvthscr, var_dvthscr_dn0, var_dvthscr_dn2, var_dvthscr_dn6, var_dvthscr_dn7, var_dvthscr_dn10, var_dvthscr_dn11, var_dvthscr_dn12, var_dvthscr_dn17,)
    }
};
        var_dvthscr = assign6630_e4402;
        var_dvthscr_dn0 = assign6630_e4402_d_n0;
        var_dvthscr_dn2 = assign6630_e4402_d_n2;
        var_dvthscr_dn6 = assign6630_e4402_d_n6;
        var_dvthscr_dn7 = assign6630_e4402_d_n7;
        var_dvthscr_dn10 = assign6630_e4402_d_n10;
        var_dvthscr_dn11 = assign6630_e4402_d_n11;
        var_dvthscr_dn12 = assign6630_e4402_d_n12;
        var_dvthscr_dn17 = assign6630_e4402_d_n17;
        var_dvthscr_rv = 0.0;

        let (assign6640_e4407, assign6640_e4407_d_n0, assign6640_e4407_d_n2, assign6640_e4407_d_n6, assign6640_e4407_d_n7, assign6640_e4407_d_n10, assign6640_e4407_d_n11, assign6640_e4407_d_n12, assign6640_e4407_d_n17,) = {
    if (var_guard97 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dvthscr, var_dvthscr_dn0, var_dvthscr_dn2, var_dvthscr_dn6, var_dvthscr_dn7, var_dvthscr_dn10, var_dvthscr_dn11, var_dvthscr_dn12, var_dvthscr_dn17,)
    }
};
        var_dvthscr = assign6640_e4407;
        var_dvthscr_dn0 = assign6640_e4407_d_n0;
        var_dvthscr_dn2 = assign6640_e4407_d_n2;
        var_dvthscr_dn6 = assign6640_e4407_d_n6;
        var_dvthscr_dn7 = assign6640_e4407_d_n7;
        var_dvthscr_dn10 = assign6640_e4407_d_n10;
        var_dvthscr_dn11 = assign6640_e4407_d_n11;
        var_dvthscr_dn12 = assign6640_e4407_d_n12;
        var_dvthscr_dn17 = assign6640_e4407_d_n17;
        var_dvthscr_rv = 0.0;

        var_t1__blk98 = var_c_fox_inv;
        var_t1__blk98_dn0 = var_c_fox_inv_dn0;
        var_t1__blk98_dn2 = var_c_fox_inv_dn2;
        var_t1__blk98_dn6 = var_c_fox_inv_dn6;
        var_t1__blk98_dn7 = var_c_fox_inv_dn7;
        var_t1__blk98_dn10 = var_c_fox_inv_dn10;
        var_t1__blk98_dn11 = var_c_fox_inv_dn11;
        var_t1__blk98_dn12 = var_c_fox_inv_dn12;
        var_t1__blk98_dn17 = var_c_fox_inv_dn17;
        var_t1__blk98_rv = 0.0;

        let assign6660_e4413: f64 = (var_mks_wfc / var_weff);
        let assign6660_e4414: f64 = (var_c_fox + assign6660_e4413);
        let assign6660_e4415: f64 = (1.0 / assign6660_e4414);
        var_t3__blk99 = assign6660_e4415;
        var_t3__blk99_dn0 = (-(var_c_fox_dn0 / (assign6660_e4414 * assign6660_e4414)));
        var_t3__blk99_dn2 = (-(var_c_fox_dn2 / (assign6660_e4414 * assign6660_e4414)));
        var_t3__blk99_dn6 = (-(var_c_fox_dn6 / (assign6660_e4414 * assign6660_e4414)));
        var_t3__blk99_dn7 = (-(var_c_fox_dn7 / (assign6660_e4414 * assign6660_e4414)));
        var_t3__blk99_dn10 = (-(var_c_fox_dn10 / (assign6660_e4414 * assign6660_e4414)));
        var_t3__blk99_dn11 = (-(var_c_fox_dn11 / (assign6660_e4414 * assign6660_e4414)));
        var_t3__blk99_dn12 = (-(var_c_fox_dn12 / (assign6660_e4414 * assign6660_e4414)));
        var_t3__blk99_dn17 = (-(var_c_fox_dn17 / (assign6660_e4414 * assign6660_e4414)));
        var_t3__blk99_rv = 0.0;

        let assign6670_e4418: f64 = (var_t1__blk98 - var_t3__blk99);
        var_t5__blk100 = assign6670_e4418;
        var_t5__blk100_dn0 = (var_t1__blk98_dn0 - var_t3__blk99_dn0);
        var_t5__blk100_dn2 = (var_t1__blk98_dn2 - var_t3__blk99_dn2);
        var_t5__blk100_dn6 = (var_t1__blk98_dn6 - var_t3__blk99_dn6);
        var_t5__blk100_dn7 = (var_t1__blk98_dn7 - var_t3__blk99_dn7);
        var_t5__blk100_dn10 = (var_t1__blk98_dn10 - var_t3__blk99_dn10);
        var_t5__blk100_dn11 = (var_t1__blk98_dn11 - var_t3__blk99_dn11);
        var_t5__blk100_dn12 = (var_t1__blk98_dn12 - var_t3__blk99_dn12);
        var_t5__blk100_dn17 = (var_t1__blk98_dn17 - var_t3__blk99_dn17);
        var_t5__blk100_rv = 0.0;

        let assign6680_e4421: f64 = (var_qb0 * var_t5__blk100);
        let assign6680_e4424: f64 = (p.p105 / var_wg);
        let assign6680_e4425: f64 = (assign6680_e4421 + assign6680_e4424);
        var_dvthw = assign6680_e4425;
        var_dvthw_dn0 = ((var_qb0_dn0 * var_t5__blk100) + (var_qb0 * var_t5__blk100_dn0));
        var_dvthw_dn2 = ((var_qb0_dn2 * var_t5__blk100) + (var_qb0 * var_t5__blk100_dn2));
        var_dvthw_dn6 = ((var_qb0_dn6 * var_t5__blk100) + (var_qb0 * var_t5__blk100_dn6));
        var_dvthw_dn7 = ((var_qb0_dn7 * var_t5__blk100) + (var_qb0 * var_t5__blk100_dn7));
        var_dvthw_dn10 = ((var_qb0_dn10 * var_t5__blk100) + (var_qb0 * var_t5__blk100_dn10));
        var_dvthw_dn11 = ((var_qb0_dn11 * var_t5__blk100) + (var_qb0 * var_t5__blk100_dn11));
        var_dvthw_dn12 = ((var_qb0_dn12 * var_t5__blk100) + (var_qb0 * var_t5__blk100_dn12));
        var_dvthw_dn17 = ((var_qb0_dn17 * var_t5__blk100) + (var_qb0 * var_t5__blk100_dn17));
        var_dvthw_rv = 0.0;

        let assign6690_e4428: f64 = (var_dvthsc + var_dvthlp);
        let assign6690_e4430: f64 = (assign6690_e4428 + var_dvthw);
        let assign6690_e4432: f64 = (assign6690_e4430 + var_dvthscr);
        let assign6690_e4434: f64 = (assign6690_e4432 + var_dvthsm);
        var_dvth = assign6690_e4434;
        var_dvth_dn0 = (((var_dvthsc_dn0 + var_dvthlp_dn0) + var_dvthw_dn0) + var_dvthscr_dn0);
        var_dvth_dn2 = (((var_dvthsc_dn2 + var_dvthlp_dn2) + var_dvthw_dn2) + var_dvthscr_dn2);
        var_dvth_dn6 = (((var_dvthsc_dn6 + var_dvthlp_dn6) + var_dvthw_dn6) + var_dvthscr_dn6);
        var_dvth_dn7 = (((var_dvthsc_dn7 + var_dvthlp_dn7) + var_dvthw_dn7) + var_dvthscr_dn7);
        var_dvth_dn10 = (((var_dvthsc_dn10 + var_dvthlp_dn10) + var_dvthw_dn10) + var_dvthscr_dn10);
        var_dvth_dn11 = (((var_dvthsc_dn11 + var_dvthlp_dn11) + var_dvthw_dn11) + var_dvthscr_dn11);
        var_dvth_dn12 = (((var_dvthsc_dn12 + var_dvthlp_dn12) + var_dvthw_dn12) + var_dvthscr_dn12);
        var_dvth_dn17 = (((var_dvthsc_dn17 + var_dvthlp_dn17) + var_dvthw_dn17) + var_dvthscr_dn17);
        var_dvth_rv = 0.0;

        let assign6700_e4437: f64 = (var_vthp - var_dvth);
        var_vth = assign6700_e4437;
        var_vth_dn0 = (var_vthp_dn0 - var_dvth_dn0);
        var_vth_dn2 = (var_vthp_dn2 - var_dvth_dn2);
        var_vth_dn6 = (var_vthp_dn6 - var_dvth_dn6);
        var_vth_dn7 = (var_vthp_dn7 - var_dvth_dn7);
        var_vth_dn10 = (var_vthp_dn10 - var_dvth_dn10);
        var_vth_dn11 = (var_vthp_dn11 - var_dvth_dn11);
        var_vth_dn12 = (var_vthp_dn12 - var_dvth_dn12);
        var_vth_dn17 = (var_vthp_dn17 - var_dvth_dn17);
        var_vth_rv = 0.0;

        let assign6710_e4440: f64 = if p.p89 == 0.0 { 1.0 } else { 0.0 };
        var_guard104 = assign6710_e4440;
        var_guard104_rv = 0.0;

        let (assign6720_e4444,) = {
    if (var_guard104 != 0.0) {
        (0.0,)
    } else {
        (var_flg_dppg,)
    }
};
        var_flg_dppg = assign6720_e4444;
        var_flg_dppg_rv = 0.0;

        let (assign6730_e4449,) = {
    if (var_guard104 == 0.0) {
        (1.0,)
    } else {
        (var_flg_dppg,)
    }
};
        var_flg_dppg = assign6730_e4449;
        var_flg_dppg_rv = 0.0;

        let assign6740_e4452: f64 = if var_flg_dppg == 0.0 { 1.0 } else { 0.0 };
        var_guard105 = assign6740_e4452;
        var_guard105_rv = 0.0;

        let (assign6750_e4456, assign6750_e4456_d_n0, assign6750_e4456_d_n2, assign6750_e4456_d_n6, assign6750_e4456_d_n7, assign6750_e4456_d_n10, assign6750_e4456_d_n11, assign6750_e4456_d_n12, assign6750_e4456_d_n17,) = {
    if (var_guard105 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6750_e4456;
        var_dppg_dn0 = assign6750_e4456_d_n0;
        var_dppg_dn2 = assign6750_e4456_d_n2;
        var_dppg_dn6 = assign6750_e4456_d_n6;
        var_dppg_dn7 = assign6750_e4456_d_n7;
        var_dppg_dn10 = assign6750_e4456_d_n10;
        var_dppg_dn11 = assign6750_e4456_d_n11;
        var_dppg_dn12 = assign6750_e4456_d_n12;
        var_dppg_dn17 = assign6750_e4456_d_n17;
        var_dppg_rv = 0.0;

        let (assign6760_e4461, assign6760_e4461_d_n0, assign6760_e4461_d_n2, assign6760_e4461_d_n6, assign6760_e4461_d_n7, assign6760_e4461_d_n10, assign6760_e4461_d_n11, assign6760_e4461_d_n12, assign6760_e4461_d_n17,) = {
    if (var_guard105 == 0.0) {
        (var_vgsz, var_vgsz_dn0, var_vgsz_dn2, var_vgsz_dn6, var_vgsz_dn7, var_vgsz_dn10, var_vgsz_dn11, var_vgsz_dn12, var_vgsz_dn17,)
    } else {
        (var_t7__blk101, var_t7__blk101_dn0, var_t7__blk101_dn2, var_t7__blk101_dn6, var_t7__blk101_dn7, var_t7__blk101_dn10, var_t7__blk101_dn11, var_t7__blk101_dn12, var_t7__blk101_dn17,)
    }
};
        var_t7__blk101 = assign6760_e4461;
        var_t7__blk101_dn0 = assign6760_e4461_d_n0;
        var_t7__blk101_dn2 = assign6760_e4461_d_n2;
        var_t7__blk101_dn6 = assign6760_e4461_d_n6;
        var_t7__blk101_dn7 = assign6760_e4461_d_n7;
        var_t7__blk101_dn10 = assign6760_e4461_d_n10;
        var_t7__blk101_dn11 = assign6760_e4461_d_n11;
        var_t7__blk101_dn12 = assign6760_e4461_d_n12;
        var_t7__blk101_dn17 = assign6760_e4461_d_n17;
        var_t7__blk101_rv = 0.0;

        let (assign6770_e4466,) = {
    if (var_guard105 == 0.0) {
        (var_cnstpgd,)
    } else {
        (var_t0__blk102,)
    }
};
        var_t0__blk102 = assign6770_e4466;
        var_t0__blk102_rv = 0.0;

        let (assign6780_e4473, assign6780_e4473_d_n0, assign6780_e4473_d_n2, assign6780_e4473_d_n6, assign6780_e4473_d_n7, assign6780_e4473_d_n10, assign6780_e4473_d_n11, assign6780_e4473_d_n12, assign6780_e4473_d_n17,) = {
    if (var_guard105 == 0.0) {
        let assign6780_e4471: f64 = (var_t7__blk101 - p.p90);
        (assign6780_e4471, var_t7__blk101_dn0, var_t7__blk101_dn2, var_t7__blk101_dn6, var_t7__blk101_dn7, var_t7__blk101_dn10, var_t7__blk101_dn11, var_t7__blk101_dn12, var_t7__blk101_dn17,)
    } else {
        (var_t3__blk103, var_t3__blk103_dn0, var_t3__blk103_dn2, var_t3__blk103_dn6, var_t3__blk103_dn7, var_t3__blk103_dn10, var_t3__blk103_dn11, var_t3__blk103_dn12, var_t3__blk103_dn17,)
    }
};
        var_t3__blk103 = assign6780_e4473;
        var_t3__blk103_dn0 = assign6780_e4473_d_n0;
        var_t3__blk103_dn2 = assign6780_e4473_d_n2;
        var_t3__blk103_dn6 = assign6780_e4473_d_n6;
        var_t3__blk103_dn7 = assign6780_e4473_d_n7;
        var_t3__blk103_dn10 = assign6780_e4473_d_n10;
        var_t3__blk103_dn11 = assign6780_e4473_d_n11;
        var_t3__blk103_dn12 = assign6780_e4473_d_n12;
        var_t3__blk103_dn17 = assign6780_e4473_d_n17;
        var_t3__blk103_rv = 0.0;

        let assign6790_e4476: f64 = (-3.0);
        let assign6790_e4477: f64 = if var_t3__blk103 < assign6790_e4476 { 1.0 } else { 0.0 };
        var_guard106 = assign6790_e4477;
        var_guard106_rv = 0.0;

        let (assign6800_e4484, assign6800_e4484_d_n0, assign6800_e4484_d_n2, assign6800_e4484_d_n6, assign6800_e4484_d_n7, assign6800_e4484_d_n10, assign6800_e4484_d_n11, assign6800_e4484_d_n12, assign6800_e4484_d_n17,) = {
    if ((var_guard105 == 0.0) && (var_guard106 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6800_e4484;
        var_dppg_dn0 = assign6800_e4484_d_n0;
        var_dppg_dn2 = assign6800_e4484_d_n2;
        var_dppg_dn6 = assign6800_e4484_d_n6;
        var_dppg_dn7 = assign6800_e4484_d_n7;
        var_dppg_dn10 = assign6800_e4484_d_n10;
        var_dppg_dn11 = assign6800_e4484_d_n11;
        var_dppg_dn12 = assign6800_e4484_d_n12;
        var_dppg_dn17 = assign6800_e4484_d_n17;
        var_dppg_rv = 0.0;

        let assign6810_e4487: f64 = if var_t3__blk103 < 0.0 { 1.0 } else { 0.0 };
        var_guard107 = assign6810_e4487;
        var_guard107_rv = 0.0;

        let (assign6820_e4513, assign6820_e4513_d_n0, assign6820_e4513_d_n2, assign6820_e4513_d_n6, assign6820_e4513_d_n7, assign6820_e4513_d_n10, assign6820_e4513_d_n11, assign6820_e4513_d_n12, assign6820_e4513_d_n17,) = {
    if (((var_guard105 == 0.0) && (var_guard106 == 0.0)) && (var_guard107 != 0.0)) {
        let assign6820_e4501: f64 = (1.0 / 3.0);
        let assign6820_e4505: f64 = (1.0 / 27.0);
        let assign6820_e4506: f64 = (var_t3__blk103 * assign6820_e4505);
        let assign6820_e4507: f64 = (assign6820_e4501 + assign6820_e4506);
        let assign6820_e4508: f64 = (var_t3__blk103 * assign6820_e4507);
        let assign6820_e4509: f64 = (1.0 + assign6820_e4508);
        let assign6820_e4510: f64 = (var_t3__blk103 * assign6820_e4509);
        let assign6820_e4511: f64 = (1.0 + assign6820_e4510);
        (assign6820_e4511, ((var_t3__blk103_dn0 * assign6820_e4509) + (var_t3__blk103 * ((var_t3__blk103_dn0 * assign6820_e4507) + (var_t3__blk103 * (var_t3__blk103_dn0 * assign6820_e4505))))), ((var_t3__blk103_dn2 * assign6820_e4509) + (var_t3__blk103 * ((var_t3__blk103_dn2 * assign6820_e4507) + (var_t3__blk103 * (var_t3__blk103_dn2 * assign6820_e4505))))), ((var_t3__blk103_dn6 * assign6820_e4509) + (var_t3__blk103 * ((var_t3__blk103_dn6 * assign6820_e4507) + (var_t3__blk103 * (var_t3__blk103_dn6 * assign6820_e4505))))), ((var_t3__blk103_dn7 * assign6820_e4509) + (var_t3__blk103 * ((var_t3__blk103_dn7 * assign6820_e4507) + (var_t3__blk103 * (var_t3__blk103_dn7 * assign6820_e4505))))), ((var_t3__blk103_dn10 * assign6820_e4509) + (var_t3__blk103 * ((var_t3__blk103_dn10 * assign6820_e4507) + (var_t3__blk103 * (var_t3__blk103_dn10 * assign6820_e4505))))), ((var_t3__blk103_dn11 * assign6820_e4509) + (var_t3__blk103 * ((var_t3__blk103_dn11 * assign6820_e4507) + (var_t3__blk103 * (var_t3__blk103_dn11 * assign6820_e4505))))), ((var_t3__blk103_dn12 * assign6820_e4509) + (var_t3__blk103 * ((var_t3__blk103_dn12 * assign6820_e4507) + (var_t3__blk103 * (var_t3__blk103_dn12 * assign6820_e4505))))), ((var_t3__blk103_dn17 * assign6820_e4509) + (var_t3__blk103 * ((var_t3__blk103_dn17 * assign6820_e4507) + (var_t3__blk103 * (var_t3__blk103_dn17 * assign6820_e4505))))),)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6820_e4513;
        var_dppg_dn0 = assign6820_e4513_d_n0;
        var_dppg_dn2 = assign6820_e4513_d_n2;
        var_dppg_dn6 = assign6820_e4513_d_n6;
        var_dppg_dn7 = assign6820_e4513_d_n7;
        var_dppg_dn10 = assign6820_e4513_d_n10;
        var_dppg_dn11 = assign6820_e4513_d_n11;
        var_dppg_dn12 = assign6820_e4513_d_n12;
        var_dppg_dn17 = assign6820_e4513_d_n17;
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
        *var_dvth0__blk93_slot = var_dvth0__blk93;
        *var_dvth0__blk93_dn0_slot = var_dvth0__blk93_dn0;
        *var_dvth0__blk93_dn10_slot = var_dvth0__blk93_dn10;
        *var_dvth0__blk93_dn11_slot = var_dvth0__blk93_dn11;
        *var_dvth0__blk93_dn12_slot = var_dvth0__blk93_dn12;
        *var_dvth0__blk93_dn17_slot = var_dvth0__blk93_dn17;
        *var_dvth0__blk93_dn2_slot = var_dvth0__blk93_dn2;
        *var_dvth0__blk93_dn6_slot = var_dvth0__blk93_dn6;
        *var_dvth0__blk93_dn7_slot = var_dvth0__blk93_dn7;
        *var_dvth0__blk93_rv_slot = var_dvth0__blk93_rv;
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
        *var_guard104_slot = var_guard104;
        *var_guard104_rv_slot = var_guard104_rv;
        *var_guard105_slot = var_guard105;
        *var_guard105_rv_slot = var_guard105_rv;
        *var_guard106_slot = var_guard106;
        *var_guard106_rv_slot = var_guard106_rv;
        *var_guard107_slot = var_guard107;
        *var_guard107_rv_slot = var_guard107_rv;
        *var_guard97_slot = var_guard97;
        *var_guard97_rv_slot = var_guard97_rv;
        *var_t0__blk102_slot = var_t0__blk102;
        *var_t0__blk102_rv_slot = var_t0__blk102_rv;
        *var_t0__blk87_slot = var_t0__blk87;
        *var_t0__blk87_dn0_slot = var_t0__blk87_dn0;
        *var_t0__blk87_dn10_slot = var_t0__blk87_dn10;
        *var_t0__blk87_dn11_slot = var_t0__blk87_dn11;
        *var_t0__blk87_dn12_slot = var_t0__blk87_dn12;
        *var_t0__blk87_dn17_slot = var_t0__blk87_dn17;
        *var_t0__blk87_dn2_slot = var_t0__blk87_dn2;
        *var_t0__blk87_dn6_slot = var_t0__blk87_dn6;
        *var_t0__blk87_dn7_slot = var_t0__blk87_dn7;
        *var_t0__blk87_rv_slot = var_t0__blk87_rv;
        *var_t1__blk88_slot = var_t1__blk88;
        *var_t1__blk88_dn0_slot = var_t1__blk88_dn0;
        *var_t1__blk88_dn10_slot = var_t1__blk88_dn10;
        *var_t1__blk88_dn11_slot = var_t1__blk88_dn11;
        *var_t1__blk88_dn12_slot = var_t1__blk88_dn12;
        *var_t1__blk88_dn17_slot = var_t1__blk88_dn17;
        *var_t1__blk88_dn2_slot = var_t1__blk88_dn2;
        *var_t1__blk88_dn6_slot = var_t1__blk88_dn6;
        *var_t1__blk88_dn7_slot = var_t1__blk88_dn7;
        *var_t1__blk88_rv_slot = var_t1__blk88_rv;
        *var_t1__blk94_slot = var_t1__blk94;
        *var_t1__blk94_dn0_slot = var_t1__blk94_dn0;
        *var_t1__blk94_dn10_slot = var_t1__blk94_dn10;
        *var_t1__blk94_dn11_slot = var_t1__blk94_dn11;
        *var_t1__blk94_dn12_slot = var_t1__blk94_dn12;
        *var_t1__blk94_dn17_slot = var_t1__blk94_dn17;
        *var_t1__blk94_dn2_slot = var_t1__blk94_dn2;
        *var_t1__blk94_dn6_slot = var_t1__blk94_dn6;
        *var_t1__blk94_dn7_slot = var_t1__blk94_dn7;
        *var_t1__blk94_rv_slot = var_t1__blk94_rv;
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
        *var_t2__blk89_slot = var_t2__blk89;
        *var_t2__blk89_dn0_slot = var_t2__blk89_dn0;
        *var_t2__blk89_dn10_slot = var_t2__blk89_dn10;
        *var_t2__blk89_dn11_slot = var_t2__blk89_dn11;
        *var_t2__blk89_dn12_slot = var_t2__blk89_dn12;
        *var_t2__blk89_dn17_slot = var_t2__blk89_dn17;
        *var_t2__blk89_dn2_slot = var_t2__blk89_dn2;
        *var_t2__blk89_dn6_slot = var_t2__blk89_dn6;
        *var_t2__blk89_dn7_slot = var_t2__blk89_dn7;
        *var_t2__blk89_rv_slot = var_t2__blk89_rv;
        *var_t2__blk95_slot = var_t2__blk95;
        *var_t2__blk95_rv_slot = var_t2__blk95_rv;
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
        *var_t3__blk90_slot = var_t3__blk90;
        *var_t3__blk90_rv_slot = var_t3__blk90_rv;
        *var_t3__blk96_slot = var_t3__blk96;
        *var_t3__blk96_rv_slot = var_t3__blk96_rv;
        *var_t3__blk99_slot = var_t3__blk99;
        *var_t3__blk99_dn0_slot = var_t3__blk99_dn0;
        *var_t3__blk99_dn10_slot = var_t3__blk99_dn10;
        *var_t3__blk99_dn11_slot = var_t3__blk99_dn11;
        *var_t3__blk99_dn12_slot = var_t3__blk99_dn12;
        *var_t3__blk99_dn17_slot = var_t3__blk99_dn17;
        *var_t3__blk99_dn2_slot = var_t3__blk99_dn2;
        *var_t3__blk99_dn6_slot = var_t3__blk99_dn6;
        *var_t3__blk99_dn7_slot = var_t3__blk99_dn7;
        *var_t3__blk99_rv_slot = var_t3__blk99_rv;
        *var_t4__blk91_slot = var_t4__blk91;
        *var_t4__blk91_dn0_slot = var_t4__blk91_dn0;
        *var_t4__blk91_dn10_slot = var_t4__blk91_dn10;
        *var_t4__blk91_dn11_slot = var_t4__blk91_dn11;
        *var_t4__blk91_dn12_slot = var_t4__blk91_dn12;
        *var_t4__blk91_dn17_slot = var_t4__blk91_dn17;
        *var_t4__blk91_dn2_slot = var_t4__blk91_dn2;
        *var_t4__blk91_dn6_slot = var_t4__blk91_dn6;
        *var_t4__blk91_dn7_slot = var_t4__blk91_dn7;
        *var_t4__blk91_rv_slot = var_t4__blk91_rv;
        *var_t5__blk100_slot = var_t5__blk100;
        *var_t5__blk100_dn0_slot = var_t5__blk100_dn0;
        *var_t5__blk100_dn10_slot = var_t5__blk100_dn10;
        *var_t5__blk100_dn11_slot = var_t5__blk100_dn11;
        *var_t5__blk100_dn12_slot = var_t5__blk100_dn12;
        *var_t5__blk100_dn17_slot = var_t5__blk100_dn17;
        *var_t5__blk100_dn2_slot = var_t5__blk100_dn2;
        *var_t5__blk100_dn6_slot = var_t5__blk100_dn6;
        *var_t5__blk100_dn7_slot = var_t5__blk100_dn7;
        *var_t5__blk100_rv_slot = var_t5__blk100_rv;
        *var_t5__blk84_slot = var_t5__blk84;
        *var_t5__blk84_dn0_slot = var_t5__blk84_dn0;
        *var_t5__blk84_dn10_slot = var_t5__blk84_dn10;
        *var_t5__blk84_dn11_slot = var_t5__blk84_dn11;
        *var_t5__blk84_dn12_slot = var_t5__blk84_dn12;
        *var_t5__blk84_dn17_slot = var_t5__blk84_dn17;
        *var_t5__blk84_dn2_slot = var_t5__blk84_dn2;
        *var_t5__blk84_dn6_slot = var_t5__blk84_dn6;
        *var_t5__blk84_dn7_slot = var_t5__blk84_dn7;
        *var_t5__blk84_rv_slot = var_t5__blk84_rv;
        *var_t5__blk92_slot = var_t5__blk92;
        *var_t5__blk92_dn0_slot = var_t5__blk92_dn0;
        *var_t5__blk92_dn10_slot = var_t5__blk92_dn10;
        *var_t5__blk92_dn11_slot = var_t5__blk92_dn11;
        *var_t5__blk92_dn12_slot = var_t5__blk92_dn12;
        *var_t5__blk92_dn17_slot = var_t5__blk92_dn17;
        *var_t5__blk92_dn2_slot = var_t5__blk92_dn2;
        *var_t5__blk92_dn6_slot = var_t5__blk92_dn6;
        *var_t5__blk92_dn7_slot = var_t5__blk92_dn7;
        *var_t5__blk92_rv_slot = var_t5__blk92_rv;
        *var_t7__blk101_slot = var_t7__blk101;
        *var_t7__blk101_dn0_slot = var_t7__blk101_dn0;
        *var_t7__blk101_dn10_slot = var_t7__blk101_dn10;
        *var_t7__blk101_dn11_slot = var_t7__blk101_dn11;
        *var_t7__blk101_dn12_slot = var_t7__blk101_dn12;
        *var_t7__blk101_dn17_slot = var_t7__blk101_dn17;
        *var_t7__blk101_dn2_slot = var_t7__blk101_dn2;
        *var_t7__blk101_dn6_slot = var_t7__blk101_dn6;
        *var_t7__blk101_dn7_slot = var_t7__blk101_dn7;
        *var_t7__blk101_rv_slot = var_t7__blk101_rv;
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
