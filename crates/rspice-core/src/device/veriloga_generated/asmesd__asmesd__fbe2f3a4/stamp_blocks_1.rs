#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_11(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_ibwd: f64,
        var_ibwd_db0: f64,
        var_ibwd_db1: f64,
        var_ibwd_db2: f64,
        var_ibwd_db3: f64,
        var_ibwd_db4: f64,
        var_ibwd_db5: f64,
        var_ibwd_db6: f64,
        var_ibwd_db7: f64,
        var_ibwd_dn0: f64,
        var_ibwd_dn1: f64,
        var_ibwd_dn2: f64,
        var_ibwd_dn3: f64,
        var_ibwd_dn4: f64,
        var_ibwd_dn5: f64,
        var_ibwd_dn6: f64,
        var_ibwd_dn7: f64,
        var_ibwd_dn8: f64,
        var_ibwd_dn9: f64,
        var_ifwd: f64,
        var_ifwd_db0: f64,
        var_ifwd_db1: f64,
        var_ifwd_db2: f64,
        var_ifwd_db3: f64,
        var_ifwd_db4: f64,
        var_ifwd_db5: f64,
        var_ifwd_db6: f64,
        var_ifwd_db7: f64,
        var_ifwd_dn0: f64,
        var_ifwd_dn1: f64,
        var_ifwd_dn2: f64,
        var_ifwd_dn3: f64,
        var_ifwd_dn4: f64,
        var_ifwd_dn5: f64,
        var_ifwd_dn6: f64,
        var_ifwd_dn7: f64,
        var_ifwd_dn8: f64,
        var_ifwd_dn9: f64,
        var_oikr: f64,
        var_oikr_db0: f64,
        var_oikr_db1: f64,
        var_oikr_db2: f64,
        var_oikr_db3: f64,
        var_oikr_db4: f64,
        var_oikr_db5: f64,
        var_oikr_db6: f64,
        var_oikr_db7: f64,
        var_oikr_dn0: f64,
        var_oikr_dn1: f64,
        var_oikr_dn2: f64,
        var_oikr_dn3: f64,
        var_oikr_dn4: f64,
        var_oikr_dn5: f64,
        var_oikr_dn6: f64,
        var_oikr_dn7: f64,
        var_oikr_dn8: f64,
        var_oikr_dn9: f64,
        var_ovaf: f64,
        var_ovaf_db0: f64,
        var_ovaf_db1: f64,
        var_ovaf_db2: f64,
        var_ovaf_db3: f64,
        var_ovaf_db4: f64,
        var_ovaf_db5: f64,
        var_ovaf_db6: f64,
        var_ovaf_db7: f64,
        var_ovaf_dn0: f64,
        var_ovaf_dn1: f64,
        var_ovaf_dn2: f64,
        var_ovaf_dn3: f64,
        var_ovaf_dn4: f64,
        var_ovaf_dn5: f64,
        var_ovaf_dn6: f64,
        var_ovaf_dn7: f64,
        var_ovaf_dn8: f64,
        var_ovaf_dn9: f64,
        var_ovar: f64,
        var_ovar_db0: f64,
        var_ovar_db1: f64,
        var_ovar_db2: f64,
        var_ovar_db3: f64,
        var_ovar_db4: f64,
        var_ovar_db5: f64,
        var_ovar_db6: f64,
        var_ovar_db7: f64,
        var_ovar_dn0: f64,
        var_ovar_dn1: f64,
        var_ovar_dn2: f64,
        var_ovar_dn3: f64,
        var_ovar_dn4: f64,
        var_ovar_dn5: f64,
        var_ovar_dn6: f64,
        var_ovar_dn7: f64,
        var_ovar_dn8: f64,
        var_ovar_dn9: f64,
        var_vbici: f64,
        var_vbici_db0: f64,
        var_vbici_db1: f64,
        var_vbici_db2: f64,
        var_vbici_db3: f64,
        var_vbici_db4: f64,
        var_vbici_db5: f64,
        var_vbici_db6: f64,
        var_vbici_db7: f64,
        var_vbici_dn0: f64,
        var_vbici_dn1: f64,
        var_vbici_dn2: f64,
        var_vbici_dn3: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbici_dn6: f64,
        var_vbici_dn7: f64,
        var_vbici_dn8: f64,
        var_vbici_dn9: f64,
        var_vbiei: f64,
        var_vbiei_db0: f64,
        var_vbiei_db1: f64,
        var_vbiei_db2: f64,
        var_vbiei_db3: f64,
        var_vbiei_db4: f64,
        var_vbiei_db5: f64,
        var_vbiei_db6: f64,
        var_vbiei_db7: f64,
        var_vbiei_dn0: f64,
        var_vbiei_dn1: f64,
        var_vbiei_dn2: f64,
        var_vbiei_dn3: f64,
        var_vbiei_dn4: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vbiei_dn7: f64,
        var_vbiei_dn8: f64,
        var_vbiei_dn9: f64,
        var_dkqb_slot: &mut f64,
        var_dkqb_db0_slot: &mut f64,
        var_dkqb_db1_slot: &mut f64,
        var_dkqb_db2_slot: &mut f64,
        var_dkqb_db3_slot: &mut f64,
        var_dkqb_db4_slot: &mut f64,
        var_dkqb_db5_slot: &mut f64,
        var_dkqb_db6_slot: &mut f64,
        var_dkqb_db7_slot: &mut f64,
        var_dkqb_dn0_slot: &mut f64,
        var_dkqb_dn1_slot: &mut f64,
        var_dkqb_dn2_slot: &mut f64,
        var_dkqb_dn3_slot: &mut f64,
        var_dkqb_dn4_slot: &mut f64,
        var_dkqb_dn5_slot: &mut f64,
        var_dkqb_dn6_slot: &mut f64,
        var_dkqb_dn7_slot: &mut f64,
        var_dkqb_dn8_slot: &mut f64,
        var_dkqb_dn9_slot: &mut f64,
        var_dkqb_rdb0_slot: &mut f64,
        var_dkqb_rdb1_slot: &mut f64,
        var_dkqb_rdb2_slot: &mut f64,
        var_dkqb_rdb3_slot: &mut f64,
        var_dkqb_rdb4_slot: &mut f64,
        var_dkqb_rdb5_slot: &mut f64,
        var_dkqb_rdb6_slot: &mut f64,
        var_dkqb_rdb7_slot: &mut f64,
        var_dkqb_rdn0_slot: &mut f64,
        var_dkqb_rdn1_slot: &mut f64,
        var_dkqb_rdn2_slot: &mut f64,
        var_dkqb_rdn3_slot: &mut f64,
        var_dkqb_rdn4_slot: &mut f64,
        var_dkqb_rdn5_slot: &mut f64,
        var_dkqb_rdn6_slot: &mut f64,
        var_dkqb_rdn7_slot: &mut f64,
        var_dkqb_rdn8_slot: &mut f64,
        var_dkqb_rdn9_slot: &mut f64,
        var_dkqb_rv_slot: &mut f64,
        var_ikq1_slot: &mut f64,
        var_ikq1_db0_slot: &mut f64,
        var_ikq1_db1_slot: &mut f64,
        var_ikq1_db2_slot: &mut f64,
        var_ikq1_db3_slot: &mut f64,
        var_ikq1_db4_slot: &mut f64,
        var_ikq1_db5_slot: &mut f64,
        var_ikq1_db6_slot: &mut f64,
        var_ikq1_db7_slot: &mut f64,
        var_ikq1_dn0_slot: &mut f64,
        var_ikq1_dn1_slot: &mut f64,
        var_ikq1_dn2_slot: &mut f64,
        var_ikq1_dn3_slot: &mut f64,
        var_ikq1_dn4_slot: &mut f64,
        var_ikq1_dn5_slot: &mut f64,
        var_ikq1_dn6_slot: &mut f64,
        var_ikq1_dn7_slot: &mut f64,
        var_ikq1_dn8_slot: &mut f64,
        var_ikq1_dn9_slot: &mut f64,
        var_ikq1_rdb0_slot: &mut f64,
        var_ikq1_rdb1_slot: &mut f64,
        var_ikq1_rdb2_slot: &mut f64,
        var_ikq1_rdb3_slot: &mut f64,
        var_ikq1_rdb4_slot: &mut f64,
        var_ikq1_rdb5_slot: &mut f64,
        var_ikq1_rdb6_slot: &mut f64,
        var_ikq1_rdb7_slot: &mut f64,
        var_ikq1_rdn0_slot: &mut f64,
        var_ikq1_rdn1_slot: &mut f64,
        var_ikq1_rdn2_slot: &mut f64,
        var_ikq1_rdn3_slot: &mut f64,
        var_ikq1_rdn4_slot: &mut f64,
        var_ikq1_rdn5_slot: &mut f64,
        var_ikq1_rdn6_slot: &mut f64,
        var_ikq1_rdn7_slot: &mut f64,
        var_ikq1_rdn8_slot: &mut f64,
        var_ikq1_rdn9_slot: &mut f64,
        var_ikq1_rv_slot: &mut f64,
        var_ikqb_slot: &mut f64,
        var_ikqb_db0_slot: &mut f64,
        var_ikqb_db1_slot: &mut f64,
        var_ikqb_db2_slot: &mut f64,
        var_ikqb_db3_slot: &mut f64,
        var_ikqb_db4_slot: &mut f64,
        var_ikqb_db5_slot: &mut f64,
        var_ikqb_db6_slot: &mut f64,
        var_ikqb_db7_slot: &mut f64,
        var_ikqb_dn0_slot: &mut f64,
        var_ikqb_dn1_slot: &mut f64,
        var_ikqb_dn2_slot: &mut f64,
        var_ikqb_dn3_slot: &mut f64,
        var_ikqb_dn4_slot: &mut f64,
        var_ikqb_dn5_slot: &mut f64,
        var_ikqb_dn6_slot: &mut f64,
        var_ikqb_dn7_slot: &mut f64,
        var_ikqb_dn8_slot: &mut f64,
        var_ikqb_dn9_slot: &mut f64,
        var_ikqb_rdb0_slot: &mut f64,
        var_ikqb_rdb1_slot: &mut f64,
        var_ikqb_rdb2_slot: &mut f64,
        var_ikqb_rdb3_slot: &mut f64,
        var_ikqb_rdb4_slot: &mut f64,
        var_ikqb_rdb5_slot: &mut f64,
        var_ikqb_rdb6_slot: &mut f64,
        var_ikqb_rdb7_slot: &mut f64,
        var_ikqb_rdn0_slot: &mut f64,
        var_ikqb_rdn1_slot: &mut f64,
        var_ikqb_rdn2_slot: &mut f64,
        var_ikqb_rdn3_slot: &mut f64,
        var_ikqb_rdn4_slot: &mut f64,
        var_ikqb_rdn5_slot: &mut f64,
        var_ikqb_rdn6_slot: &mut f64,
        var_ikqb_rdn7_slot: &mut f64,
        var_ikqb_rdn8_slot: &mut f64,
        var_ikqb_rdn9_slot: &mut f64,
        var_ikqb_rv_slot: &mut f64,
        var_itr_slot: &mut f64,
        var_itr_db0_slot: &mut f64,
        var_itr_db1_slot: &mut f64,
        var_itr_db2_slot: &mut f64,
        var_itr_db3_slot: &mut f64,
        var_itr_db4_slot: &mut f64,
        var_itr_db5_slot: &mut f64,
        var_itr_db6_slot: &mut f64,
        var_itr_db7_slot: &mut f64,
        var_itr_dn0_slot: &mut f64,
        var_itr_dn1_slot: &mut f64,
        var_itr_dn2_slot: &mut f64,
        var_itr_dn3_slot: &mut f64,
        var_itr_dn4_slot: &mut f64,
        var_itr_dn5_slot: &mut f64,
        var_itr_dn6_slot: &mut f64,
        var_itr_dn7_slot: &mut f64,
        var_itr_dn8_slot: &mut f64,
        var_itr_dn9_slot: &mut f64,
        var_itr_rdb0_slot: &mut f64,
        var_itr_rdb1_slot: &mut f64,
        var_itr_rdb2_slot: &mut f64,
        var_itr_rdb3_slot: &mut f64,
        var_itr_rdb4_slot: &mut f64,
        var_itr_rdb5_slot: &mut f64,
        var_itr_rdb6_slot: &mut f64,
        var_itr_rdb7_slot: &mut f64,
        var_itr_rdn0_slot: &mut f64,
        var_itr_rdn1_slot: &mut f64,
        var_itr_rdn2_slot: &mut f64,
        var_itr_rdn3_slot: &mut f64,
        var_itr_rdn4_slot: &mut f64,
        var_itr_rdn5_slot: &mut f64,
        var_itr_rdn6_slot: &mut f64,
        var_itr_rdn7_slot: &mut f64,
        var_itr_rdn8_slot: &mut f64,
        var_itr_rdn9_slot: &mut f64,
        var_itr_rv_slot: &mut f64,
        var_itzf_slot: &mut f64,
        var_itzf_db0_slot: &mut f64,
        var_itzf_db1_slot: &mut f64,
        var_itzf_db2_slot: &mut f64,
        var_itzf_db3_slot: &mut f64,
        var_itzf_db4_slot: &mut f64,
        var_itzf_db5_slot: &mut f64,
        var_itzf_db6_slot: &mut f64,
        var_itzf_db7_slot: &mut f64,
        var_itzf_dn0_slot: &mut f64,
        var_itzf_dn1_slot: &mut f64,
        var_itzf_dn2_slot: &mut f64,
        var_itzf_dn3_slot: &mut f64,
        var_itzf_dn4_slot: &mut f64,
        var_itzf_dn5_slot: &mut f64,
        var_itzf_dn6_slot: &mut f64,
        var_itzf_dn7_slot: &mut f64,
        var_itzf_dn8_slot: &mut f64,
        var_itzf_dn9_slot: &mut f64,
        var_itzf_rdb0_slot: &mut f64,
        var_itzf_rdb1_slot: &mut f64,
        var_itzf_rdb2_slot: &mut f64,
        var_itzf_rdb3_slot: &mut f64,
        var_itzf_rdb4_slot: &mut f64,
        var_itzf_rdb5_slot: &mut f64,
        var_itzf_rdb6_slot: &mut f64,
        var_itzf_rdb7_slot: &mut f64,
        var_itzf_rdn0_slot: &mut f64,
        var_itzf_rdn1_slot: &mut f64,
        var_itzf_rdn2_slot: &mut f64,
        var_itzf_rdn3_slot: &mut f64,
        var_itzf_rdn4_slot: &mut f64,
        var_itzf_rdn5_slot: &mut f64,
        var_itzf_rdn6_slot: &mut f64,
        var_itzf_rdn7_slot: &mut f64,
        var_itzf_rdn8_slot: &mut f64,
        var_itzf_rdn9_slot: &mut f64,
        var_itzf_rv_slot: &mut f64,
        var_kq2_slot: &mut f64,
        var_kq2_db0_slot: &mut f64,
        var_kq2_db1_slot: &mut f64,
        var_kq2_db2_slot: &mut f64,
        var_kq2_db3_slot: &mut f64,
        var_kq2_db4_slot: &mut f64,
        var_kq2_db5_slot: &mut f64,
        var_kq2_db6_slot: &mut f64,
        var_kq2_db7_slot: &mut f64,
        var_kq2_dn0_slot: &mut f64,
        var_kq2_dn1_slot: &mut f64,
        var_kq2_dn2_slot: &mut f64,
        var_kq2_dn3_slot: &mut f64,
        var_kq2_dn4_slot: &mut f64,
        var_kq2_dn5_slot: &mut f64,
        var_kq2_dn6_slot: &mut f64,
        var_kq2_dn7_slot: &mut f64,
        var_kq2_dn8_slot: &mut f64,
        var_kq2_dn9_slot: &mut f64,
        var_kq2_rdb0_slot: &mut f64,
        var_kq2_rdb1_slot: &mut f64,
        var_kq2_rdb2_slot: &mut f64,
        var_kq2_rdb3_slot: &mut f64,
        var_kq2_rdb4_slot: &mut f64,
        var_kq2_rdb5_slot: &mut f64,
        var_kq2_rdb6_slot: &mut f64,
        var_kq2_rdb7_slot: &mut f64,
        var_kq2_rdn0_slot: &mut f64,
        var_kq2_rdn1_slot: &mut f64,
        var_kq2_rdn2_slot: &mut f64,
        var_kq2_rdn3_slot: &mut f64,
        var_kq2_rdn4_slot: &mut f64,
        var_kq2_rdn5_slot: &mut f64,
        var_kq2_rdn6_slot: &mut f64,
        var_kq2_rdn7_slot: &mut f64,
        var_kq2_rdn8_slot: &mut f64,
        var_kq2_rdn9_slot: &mut f64,
        var_kq2_rv_slot: &mut f64,
        var_oikf_slot: &mut f64,
        var_oikf_db0_slot: &mut f64,
        var_oikf_db1_slot: &mut f64,
        var_oikf_db2_slot: &mut f64,
        var_oikf_db3_slot: &mut f64,
        var_oikf_db4_slot: &mut f64,
        var_oikf_db5_slot: &mut f64,
        var_oikf_db6_slot: &mut f64,
        var_oikf_db7_slot: &mut f64,
        var_oikf_dn0_slot: &mut f64,
        var_oikf_dn1_slot: &mut f64,
        var_oikf_dn2_slot: &mut f64,
        var_oikf_dn3_slot: &mut f64,
        var_oikf_dn4_slot: &mut f64,
        var_oikf_dn5_slot: &mut f64,
        var_oikf_dn6_slot: &mut f64,
        var_oikf_dn7_slot: &mut f64,
        var_oikf_dn8_slot: &mut f64,
        var_oikf_dn9_slot: &mut f64,
        var_oikf_rdb0_slot: &mut f64,
        var_oikf_rdb1_slot: &mut f64,
        var_oikf_rdb2_slot: &mut f64,
        var_oikf_rdb3_slot: &mut f64,
        var_oikf_rdb4_slot: &mut f64,
        var_oikf_rdb5_slot: &mut f64,
        var_oikf_rdb6_slot: &mut f64,
        var_oikf_rdb7_slot: &mut f64,
        var_oikf_rdn0_slot: &mut f64,
        var_oikf_rdn1_slot: &mut f64,
        var_oikf_rdn2_slot: &mut f64,
        var_oikf_rdn3_slot: &mut f64,
        var_oikf_rdn4_slot: &mut f64,
        var_oikf_rdn5_slot: &mut f64,
        var_oikf_rdn6_slot: &mut f64,
        var_oikf_rdn7_slot: &mut f64,
        var_oikf_rdn8_slot: &mut f64,
        var_oikf_rdn9_slot: &mut f64,
        var_oikf_rv_slot: &mut f64,
        var_qdc_slot: &mut f64,
        var_qdc_db0_slot: &mut f64,
        var_qdc_db1_slot: &mut f64,
        var_qdc_db2_slot: &mut f64,
        var_qdc_db3_slot: &mut f64,
        var_qdc_db4_slot: &mut f64,
        var_qdc_db5_slot: &mut f64,
        var_qdc_db6_slot: &mut f64,
        var_qdc_db7_slot: &mut f64,
        var_qdc_dn0_slot: &mut f64,
        var_qdc_dn1_slot: &mut f64,
        var_qdc_dn2_slot: &mut f64,
        var_qdc_dn3_slot: &mut f64,
        var_qdc_dn4_slot: &mut f64,
        var_qdc_dn5_slot: &mut f64,
        var_qdc_dn6_slot: &mut f64,
        var_qdc_dn7_slot: &mut f64,
        var_qdc_dn8_slot: &mut f64,
        var_qdc_dn9_slot: &mut f64,
        var_qdc_rdb0_slot: &mut f64,
        var_qdc_rdb1_slot: &mut f64,
        var_qdc_rdb2_slot: &mut f64,
        var_qdc_rdb3_slot: &mut f64,
        var_qdc_rdb4_slot: &mut f64,
        var_qdc_rdb5_slot: &mut f64,
        var_qdc_rdb6_slot: &mut f64,
        var_qdc_rdb7_slot: &mut f64,
        var_qdc_rdn0_slot: &mut f64,
        var_qdc_rdn1_slot: &mut f64,
        var_qdc_rdn2_slot: &mut f64,
        var_qdc_rdn3_slot: &mut f64,
        var_qdc_rdn4_slot: &mut f64,
        var_qdc_rdn5_slot: &mut f64,
        var_qdc_rdn6_slot: &mut f64,
        var_qdc_rdn7_slot: &mut f64,
        var_qdc_rdn8_slot: &mut f64,
        var_qdc_rdn9_slot: &mut f64,
        var_qdc_rv_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_db0_slot: &mut f64,
        var_qde_db1_slot: &mut f64,
        var_qde_db2_slot: &mut f64,
        var_qde_db3_slot: &mut f64,
        var_qde_db4_slot: &mut f64,
        var_qde_db5_slot: &mut f64,
        var_qde_db6_slot: &mut f64,
        var_qde_db7_slot: &mut f64,
        var_qde_dn0_slot: &mut f64,
        var_qde_dn1_slot: &mut f64,
        var_qde_dn2_slot: &mut f64,
        var_qde_dn3_slot: &mut f64,
        var_qde_dn4_slot: &mut f64,
        var_qde_dn5_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qde_dn7_slot: &mut f64,
        var_qde_dn8_slot: &mut f64,
        var_qde_dn9_slot: &mut f64,
        var_qde_rdb0_slot: &mut f64,
        var_qde_rdb1_slot: &mut f64,
        var_qde_rdb2_slot: &mut f64,
        var_qde_rdb3_slot: &mut f64,
        var_qde_rdb4_slot: &mut f64,
        var_qde_rdb5_slot: &mut f64,
        var_qde_rdb6_slot: &mut f64,
        var_qde_rdb7_slot: &mut f64,
        var_qde_rdn0_slot: &mut f64,
        var_qde_rdn1_slot: &mut f64,
        var_qde_rdn2_slot: &mut f64,
        var_qde_rdn3_slot: &mut f64,
        var_qde_rdn4_slot: &mut f64,
        var_qde_rdn5_slot: &mut f64,
        var_qde_rdn6_slot: &mut f64,
        var_qde_rdn7_slot: &mut f64,
        var_qde_rdn8_slot: &mut f64,
        var_qde_rdn9_slot: &mut f64,
        var_qde_rv_slot: &mut f64,
        var_tff_slot: &mut f64,
        var_tff_db0_slot: &mut f64,
        var_tff_db1_slot: &mut f64,
        var_tff_db2_slot: &mut f64,
        var_tff_db3_slot: &mut f64,
        var_tff_db4_slot: &mut f64,
        var_tff_db5_slot: &mut f64,
        var_tff_db6_slot: &mut f64,
        var_tff_db7_slot: &mut f64,
        var_tff_dn0_slot: &mut f64,
        var_tff_dn1_slot: &mut f64,
        var_tff_dn2_slot: &mut f64,
        var_tff_dn3_slot: &mut f64,
        var_tff_dn4_slot: &mut f64,
        var_tff_dn5_slot: &mut f64,
        var_tff_dn6_slot: &mut f64,
        var_tff_dn7_slot: &mut f64,
        var_tff_dn8_slot: &mut f64,
        var_tff_dn9_slot: &mut f64,
        var_tff_rdb0_slot: &mut f64,
        var_tff_rdb1_slot: &mut f64,
        var_tff_rdb2_slot: &mut f64,
        var_tff_rdb3_slot: &mut f64,
        var_tff_rdb4_slot: &mut f64,
        var_tff_rdb5_slot: &mut f64,
        var_tff_rdb6_slot: &mut f64,
        var_tff_rdb7_slot: &mut f64,
        var_tff_rdn0_slot: &mut f64,
        var_tff_rdn1_slot: &mut f64,
        var_tff_rdn2_slot: &mut f64,
        var_tff_rdn3_slot: &mut f64,
        var_tff_rdn4_slot: &mut f64,
        var_tff_rdn5_slot: &mut f64,
        var_tff_rdn6_slot: &mut f64,
        var_tff_rdn7_slot: &mut f64,
        var_tff_rdn8_slot: &mut f64,
        var_tff_rdn9_slot: &mut f64,
        var_tff_rv_slot: &mut f64,
        var_vtff_slot: &mut f64,
        var_vtff1_slot: &mut f64,
        var_vtff1_db0_slot: &mut f64,
        var_vtff1_db1_slot: &mut f64,
        var_vtff1_db2_slot: &mut f64,
        var_vtff1_db3_slot: &mut f64,
        var_vtff1_db4_slot: &mut f64,
        var_vtff1_db5_slot: &mut f64,
        var_vtff1_db6_slot: &mut f64,
        var_vtff1_db7_slot: &mut f64,
        var_vtff1_dn0_slot: &mut f64,
        var_vtff1_dn1_slot: &mut f64,
        var_vtff1_dn2_slot: &mut f64,
        var_vtff1_dn3_slot: &mut f64,
        var_vtff1_dn4_slot: &mut f64,
        var_vtff1_dn5_slot: &mut f64,
        var_vtff1_dn6_slot: &mut f64,
        var_vtff1_dn7_slot: &mut f64,
        var_vtff1_dn8_slot: &mut f64,
        var_vtff1_dn9_slot: &mut f64,
        var_vtff1_rdb0_slot: &mut f64,
        var_vtff1_rdb1_slot: &mut f64,
        var_vtff1_rdb2_slot: &mut f64,
        var_vtff1_rdb3_slot: &mut f64,
        var_vtff1_rdb4_slot: &mut f64,
        var_vtff1_rdb5_slot: &mut f64,
        var_vtff1_rdb6_slot: &mut f64,
        var_vtff1_rdb7_slot: &mut f64,
        var_vtff1_rdn0_slot: &mut f64,
        var_vtff1_rdn1_slot: &mut f64,
        var_vtff1_rdn2_slot: &mut f64,
        var_vtff1_rdn3_slot: &mut f64,
        var_vtff1_rdn4_slot: &mut f64,
        var_vtff1_rdn5_slot: &mut f64,
        var_vtff1_rdn6_slot: &mut f64,
        var_vtff1_rdn7_slot: &mut f64,
        var_vtff1_rdn8_slot: &mut f64,
        var_vtff1_rdn9_slot: &mut f64,
        var_vtff1_rv_slot: &mut f64,
        var_vtff_db0_slot: &mut f64,
        var_vtff_db1_slot: &mut f64,
        var_vtff_db2_slot: &mut f64,
        var_vtff_db3_slot: &mut f64,
        var_vtff_db4_slot: &mut f64,
        var_vtff_db5_slot: &mut f64,
        var_vtff_db6_slot: &mut f64,
        var_vtff_db7_slot: &mut f64,
        var_vtff_dn0_slot: &mut f64,
        var_vtff_dn1_slot: &mut f64,
        var_vtff_dn2_slot: &mut f64,
        var_vtff_dn3_slot: &mut f64,
        var_vtff_dn4_slot: &mut f64,
        var_vtff_dn5_slot: &mut f64,
        var_vtff_dn6_slot: &mut f64,
        var_vtff_dn7_slot: &mut f64,
        var_vtff_dn8_slot: &mut f64,
        var_vtff_dn9_slot: &mut f64,
        var_vtff_rdb0_slot: &mut f64,
        var_vtff_rdb1_slot: &mut f64,
        var_vtff_rdb2_slot: &mut f64,
        var_vtff_rdb3_slot: &mut f64,
        var_vtff_rdb4_slot: &mut f64,
        var_vtff_rdb5_slot: &mut f64,
        var_vtff_rdb6_slot: &mut f64,
        var_vtff_rdb7_slot: &mut f64,
        var_vtff_rdn0_slot: &mut f64,
        var_vtff_rdn1_slot: &mut f64,
        var_vtff_rdn2_slot: &mut f64,
        var_vtff_rdn3_slot: &mut f64,
        var_vtff_rdn4_slot: &mut f64,
        var_vtff_rdn5_slot: &mut f64,
        var_vtff_rdn6_slot: &mut f64,
        var_vtff_rdn7_slot: &mut f64,
        var_vtff_rdn8_slot: &mut f64,
        var_vtff_rdn9_slot: &mut f64,
        var_vtff_rv_slot: &mut f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let mut var_dkqb: f64 = *var_dkqb_slot;
        let mut var_dkqb_db0: f64 = *var_dkqb_db0_slot;
        let mut var_dkqb_db1: f64 = *var_dkqb_db1_slot;
        let mut var_dkqb_db2: f64 = *var_dkqb_db2_slot;
        let mut var_dkqb_db3: f64 = *var_dkqb_db3_slot;
        let mut var_dkqb_db4: f64 = *var_dkqb_db4_slot;
        let mut var_dkqb_db5: f64 = *var_dkqb_db5_slot;
        let mut var_dkqb_db6: f64 = *var_dkqb_db6_slot;
        let mut var_dkqb_db7: f64 = *var_dkqb_db7_slot;
        let mut var_dkqb_dn0: f64 = *var_dkqb_dn0_slot;
        let mut var_dkqb_dn1: f64 = *var_dkqb_dn1_slot;
        let mut var_dkqb_dn2: f64 = *var_dkqb_dn2_slot;
        let mut var_dkqb_dn3: f64 = *var_dkqb_dn3_slot;
        let mut var_dkqb_dn4: f64 = *var_dkqb_dn4_slot;
        let mut var_dkqb_dn5: f64 = *var_dkqb_dn5_slot;
        let mut var_dkqb_dn6: f64 = *var_dkqb_dn6_slot;
        let mut var_dkqb_dn7: f64 = *var_dkqb_dn7_slot;
        let mut var_dkqb_dn8: f64 = *var_dkqb_dn8_slot;
        let mut var_dkqb_dn9: f64 = *var_dkqb_dn9_slot;
        let mut var_dkqb_rdb0: f64 = *var_dkqb_rdb0_slot;
        let mut var_dkqb_rdb1: f64 = *var_dkqb_rdb1_slot;
        let mut var_dkqb_rdb2: f64 = *var_dkqb_rdb2_slot;
        let mut var_dkqb_rdb3: f64 = *var_dkqb_rdb3_slot;
        let mut var_dkqb_rdb4: f64 = *var_dkqb_rdb4_slot;
        let mut var_dkqb_rdb5: f64 = *var_dkqb_rdb5_slot;
        let mut var_dkqb_rdb6: f64 = *var_dkqb_rdb6_slot;
        let mut var_dkqb_rdb7: f64 = *var_dkqb_rdb7_slot;
        let mut var_dkqb_rdn0: f64 = *var_dkqb_rdn0_slot;
        let mut var_dkqb_rdn1: f64 = *var_dkqb_rdn1_slot;
        let mut var_dkqb_rdn2: f64 = *var_dkqb_rdn2_slot;
        let mut var_dkqb_rdn3: f64 = *var_dkqb_rdn3_slot;
        let mut var_dkqb_rdn4: f64 = *var_dkqb_rdn4_slot;
        let mut var_dkqb_rdn5: f64 = *var_dkqb_rdn5_slot;
        let mut var_dkqb_rdn6: f64 = *var_dkqb_rdn6_slot;
        let mut var_dkqb_rdn7: f64 = *var_dkqb_rdn7_slot;
        let mut var_dkqb_rdn8: f64 = *var_dkqb_rdn8_slot;
        let mut var_dkqb_rdn9: f64 = *var_dkqb_rdn9_slot;
        let mut var_dkqb_rv: f64 = *var_dkqb_rv_slot;
        let mut var_ikq1: f64 = *var_ikq1_slot;
        let mut var_ikq1_db0: f64 = *var_ikq1_db0_slot;
        let mut var_ikq1_db1: f64 = *var_ikq1_db1_slot;
        let mut var_ikq1_db2: f64 = *var_ikq1_db2_slot;
        let mut var_ikq1_db3: f64 = *var_ikq1_db3_slot;
        let mut var_ikq1_db4: f64 = *var_ikq1_db4_slot;
        let mut var_ikq1_db5: f64 = *var_ikq1_db5_slot;
        let mut var_ikq1_db6: f64 = *var_ikq1_db6_slot;
        let mut var_ikq1_db7: f64 = *var_ikq1_db7_slot;
        let mut var_ikq1_dn0: f64 = *var_ikq1_dn0_slot;
        let mut var_ikq1_dn1: f64 = *var_ikq1_dn1_slot;
        let mut var_ikq1_dn2: f64 = *var_ikq1_dn2_slot;
        let mut var_ikq1_dn3: f64 = *var_ikq1_dn3_slot;
        let mut var_ikq1_dn4: f64 = *var_ikq1_dn4_slot;
        let mut var_ikq1_dn5: f64 = *var_ikq1_dn5_slot;
        let mut var_ikq1_dn6: f64 = *var_ikq1_dn6_slot;
        let mut var_ikq1_dn7: f64 = *var_ikq1_dn7_slot;
        let mut var_ikq1_dn8: f64 = *var_ikq1_dn8_slot;
        let mut var_ikq1_dn9: f64 = *var_ikq1_dn9_slot;
        let mut var_ikq1_rdb0: f64 = *var_ikq1_rdb0_slot;
        let mut var_ikq1_rdb1: f64 = *var_ikq1_rdb1_slot;
        let mut var_ikq1_rdb2: f64 = *var_ikq1_rdb2_slot;
        let mut var_ikq1_rdb3: f64 = *var_ikq1_rdb3_slot;
        let mut var_ikq1_rdb4: f64 = *var_ikq1_rdb4_slot;
        let mut var_ikq1_rdb5: f64 = *var_ikq1_rdb5_slot;
        let mut var_ikq1_rdb6: f64 = *var_ikq1_rdb6_slot;
        let mut var_ikq1_rdb7: f64 = *var_ikq1_rdb7_slot;
        let mut var_ikq1_rdn0: f64 = *var_ikq1_rdn0_slot;
        let mut var_ikq1_rdn1: f64 = *var_ikq1_rdn1_slot;
        let mut var_ikq1_rdn2: f64 = *var_ikq1_rdn2_slot;
        let mut var_ikq1_rdn3: f64 = *var_ikq1_rdn3_slot;
        let mut var_ikq1_rdn4: f64 = *var_ikq1_rdn4_slot;
        let mut var_ikq1_rdn5: f64 = *var_ikq1_rdn5_slot;
        let mut var_ikq1_rdn6: f64 = *var_ikq1_rdn6_slot;
        let mut var_ikq1_rdn7: f64 = *var_ikq1_rdn7_slot;
        let mut var_ikq1_rdn8: f64 = *var_ikq1_rdn8_slot;
        let mut var_ikq1_rdn9: f64 = *var_ikq1_rdn9_slot;
        let mut var_ikq1_rv: f64 = *var_ikq1_rv_slot;
        let mut var_ikqb: f64 = *var_ikqb_slot;
        let mut var_ikqb_db0: f64 = *var_ikqb_db0_slot;
        let mut var_ikqb_db1: f64 = *var_ikqb_db1_slot;
        let mut var_ikqb_db2: f64 = *var_ikqb_db2_slot;
        let mut var_ikqb_db3: f64 = *var_ikqb_db3_slot;
        let mut var_ikqb_db4: f64 = *var_ikqb_db4_slot;
        let mut var_ikqb_db5: f64 = *var_ikqb_db5_slot;
        let mut var_ikqb_db6: f64 = *var_ikqb_db6_slot;
        let mut var_ikqb_db7: f64 = *var_ikqb_db7_slot;
        let mut var_ikqb_dn0: f64 = *var_ikqb_dn0_slot;
        let mut var_ikqb_dn1: f64 = *var_ikqb_dn1_slot;
        let mut var_ikqb_dn2: f64 = *var_ikqb_dn2_slot;
        let mut var_ikqb_dn3: f64 = *var_ikqb_dn3_slot;
        let mut var_ikqb_dn4: f64 = *var_ikqb_dn4_slot;
        let mut var_ikqb_dn5: f64 = *var_ikqb_dn5_slot;
        let mut var_ikqb_dn6: f64 = *var_ikqb_dn6_slot;
        let mut var_ikqb_dn7: f64 = *var_ikqb_dn7_slot;
        let mut var_ikqb_dn8: f64 = *var_ikqb_dn8_slot;
        let mut var_ikqb_dn9: f64 = *var_ikqb_dn9_slot;
        let mut var_ikqb_rdb0: f64 = *var_ikqb_rdb0_slot;
        let mut var_ikqb_rdb1: f64 = *var_ikqb_rdb1_slot;
        let mut var_ikqb_rdb2: f64 = *var_ikqb_rdb2_slot;
        let mut var_ikqb_rdb3: f64 = *var_ikqb_rdb3_slot;
        let mut var_ikqb_rdb4: f64 = *var_ikqb_rdb4_slot;
        let mut var_ikqb_rdb5: f64 = *var_ikqb_rdb5_slot;
        let mut var_ikqb_rdb6: f64 = *var_ikqb_rdb6_slot;
        let mut var_ikqb_rdb7: f64 = *var_ikqb_rdb7_slot;
        let mut var_ikqb_rdn0: f64 = *var_ikqb_rdn0_slot;
        let mut var_ikqb_rdn1: f64 = *var_ikqb_rdn1_slot;
        let mut var_ikqb_rdn2: f64 = *var_ikqb_rdn2_slot;
        let mut var_ikqb_rdn3: f64 = *var_ikqb_rdn3_slot;
        let mut var_ikqb_rdn4: f64 = *var_ikqb_rdn4_slot;
        let mut var_ikqb_rdn5: f64 = *var_ikqb_rdn5_slot;
        let mut var_ikqb_rdn6: f64 = *var_ikqb_rdn6_slot;
        let mut var_ikqb_rdn7: f64 = *var_ikqb_rdn7_slot;
        let mut var_ikqb_rdn8: f64 = *var_ikqb_rdn8_slot;
        let mut var_ikqb_rdn9: f64 = *var_ikqb_rdn9_slot;
        let mut var_ikqb_rv: f64 = *var_ikqb_rv_slot;
        let mut var_itr: f64 = *var_itr_slot;
        let mut var_itr_db0: f64 = *var_itr_db0_slot;
        let mut var_itr_db1: f64 = *var_itr_db1_slot;
        let mut var_itr_db2: f64 = *var_itr_db2_slot;
        let mut var_itr_db3: f64 = *var_itr_db3_slot;
        let mut var_itr_db4: f64 = *var_itr_db4_slot;
        let mut var_itr_db5: f64 = *var_itr_db5_slot;
        let mut var_itr_db6: f64 = *var_itr_db6_slot;
        let mut var_itr_db7: f64 = *var_itr_db7_slot;
        let mut var_itr_dn0: f64 = *var_itr_dn0_slot;
        let mut var_itr_dn1: f64 = *var_itr_dn1_slot;
        let mut var_itr_dn2: f64 = *var_itr_dn2_slot;
        let mut var_itr_dn3: f64 = *var_itr_dn3_slot;
        let mut var_itr_dn4: f64 = *var_itr_dn4_slot;
        let mut var_itr_dn5: f64 = *var_itr_dn5_slot;
        let mut var_itr_dn6: f64 = *var_itr_dn6_slot;
        let mut var_itr_dn7: f64 = *var_itr_dn7_slot;
        let mut var_itr_dn8: f64 = *var_itr_dn8_slot;
        let mut var_itr_dn9: f64 = *var_itr_dn9_slot;
        let mut var_itr_rdb0: f64 = *var_itr_rdb0_slot;
        let mut var_itr_rdb1: f64 = *var_itr_rdb1_slot;
        let mut var_itr_rdb2: f64 = *var_itr_rdb2_slot;
        let mut var_itr_rdb3: f64 = *var_itr_rdb3_slot;
        let mut var_itr_rdb4: f64 = *var_itr_rdb4_slot;
        let mut var_itr_rdb5: f64 = *var_itr_rdb5_slot;
        let mut var_itr_rdb6: f64 = *var_itr_rdb6_slot;
        let mut var_itr_rdb7: f64 = *var_itr_rdb7_slot;
        let mut var_itr_rdn0: f64 = *var_itr_rdn0_slot;
        let mut var_itr_rdn1: f64 = *var_itr_rdn1_slot;
        let mut var_itr_rdn2: f64 = *var_itr_rdn2_slot;
        let mut var_itr_rdn3: f64 = *var_itr_rdn3_slot;
        let mut var_itr_rdn4: f64 = *var_itr_rdn4_slot;
        let mut var_itr_rdn5: f64 = *var_itr_rdn5_slot;
        let mut var_itr_rdn6: f64 = *var_itr_rdn6_slot;
        let mut var_itr_rdn7: f64 = *var_itr_rdn7_slot;
        let mut var_itr_rdn8: f64 = *var_itr_rdn8_slot;
        let mut var_itr_rdn9: f64 = *var_itr_rdn9_slot;
        let mut var_itr_rv: f64 = *var_itr_rv_slot;
        let mut var_itzf: f64 = *var_itzf_slot;
        let mut var_itzf_db0: f64 = *var_itzf_db0_slot;
        let mut var_itzf_db1: f64 = *var_itzf_db1_slot;
        let mut var_itzf_db2: f64 = *var_itzf_db2_slot;
        let mut var_itzf_db3: f64 = *var_itzf_db3_slot;
        let mut var_itzf_db4: f64 = *var_itzf_db4_slot;
        let mut var_itzf_db5: f64 = *var_itzf_db5_slot;
        let mut var_itzf_db6: f64 = *var_itzf_db6_slot;
        let mut var_itzf_db7: f64 = *var_itzf_db7_slot;
        let mut var_itzf_dn0: f64 = *var_itzf_dn0_slot;
        let mut var_itzf_dn1: f64 = *var_itzf_dn1_slot;
        let mut var_itzf_dn2: f64 = *var_itzf_dn2_slot;
        let mut var_itzf_dn3: f64 = *var_itzf_dn3_slot;
        let mut var_itzf_dn4: f64 = *var_itzf_dn4_slot;
        let mut var_itzf_dn5: f64 = *var_itzf_dn5_slot;
        let mut var_itzf_dn6: f64 = *var_itzf_dn6_slot;
        let mut var_itzf_dn7: f64 = *var_itzf_dn7_slot;
        let mut var_itzf_dn8: f64 = *var_itzf_dn8_slot;
        let mut var_itzf_dn9: f64 = *var_itzf_dn9_slot;
        let mut var_itzf_rdb0: f64 = *var_itzf_rdb0_slot;
        let mut var_itzf_rdb1: f64 = *var_itzf_rdb1_slot;
        let mut var_itzf_rdb2: f64 = *var_itzf_rdb2_slot;
        let mut var_itzf_rdb3: f64 = *var_itzf_rdb3_slot;
        let mut var_itzf_rdb4: f64 = *var_itzf_rdb4_slot;
        let mut var_itzf_rdb5: f64 = *var_itzf_rdb5_slot;
        let mut var_itzf_rdb6: f64 = *var_itzf_rdb6_slot;
        let mut var_itzf_rdb7: f64 = *var_itzf_rdb7_slot;
        let mut var_itzf_rdn0: f64 = *var_itzf_rdn0_slot;
        let mut var_itzf_rdn1: f64 = *var_itzf_rdn1_slot;
        let mut var_itzf_rdn2: f64 = *var_itzf_rdn2_slot;
        let mut var_itzf_rdn3: f64 = *var_itzf_rdn3_slot;
        let mut var_itzf_rdn4: f64 = *var_itzf_rdn4_slot;
        let mut var_itzf_rdn5: f64 = *var_itzf_rdn5_slot;
        let mut var_itzf_rdn6: f64 = *var_itzf_rdn6_slot;
        let mut var_itzf_rdn7: f64 = *var_itzf_rdn7_slot;
        let mut var_itzf_rdn8: f64 = *var_itzf_rdn8_slot;
        let mut var_itzf_rdn9: f64 = *var_itzf_rdn9_slot;
        let mut var_itzf_rv: f64 = *var_itzf_rv_slot;
        let mut var_kq2: f64 = *var_kq2_slot;
        let mut var_kq2_db0: f64 = *var_kq2_db0_slot;
        let mut var_kq2_db1: f64 = *var_kq2_db1_slot;
        let mut var_kq2_db2: f64 = *var_kq2_db2_slot;
        let mut var_kq2_db3: f64 = *var_kq2_db3_slot;
        let mut var_kq2_db4: f64 = *var_kq2_db4_slot;
        let mut var_kq2_db5: f64 = *var_kq2_db5_slot;
        let mut var_kq2_db6: f64 = *var_kq2_db6_slot;
        let mut var_kq2_db7: f64 = *var_kq2_db7_slot;
        let mut var_kq2_dn0: f64 = *var_kq2_dn0_slot;
        let mut var_kq2_dn1: f64 = *var_kq2_dn1_slot;
        let mut var_kq2_dn2: f64 = *var_kq2_dn2_slot;
        let mut var_kq2_dn3: f64 = *var_kq2_dn3_slot;
        let mut var_kq2_dn4: f64 = *var_kq2_dn4_slot;
        let mut var_kq2_dn5: f64 = *var_kq2_dn5_slot;
        let mut var_kq2_dn6: f64 = *var_kq2_dn6_slot;
        let mut var_kq2_dn7: f64 = *var_kq2_dn7_slot;
        let mut var_kq2_dn8: f64 = *var_kq2_dn8_slot;
        let mut var_kq2_dn9: f64 = *var_kq2_dn9_slot;
        let mut var_kq2_rdb0: f64 = *var_kq2_rdb0_slot;
        let mut var_kq2_rdb1: f64 = *var_kq2_rdb1_slot;
        let mut var_kq2_rdb2: f64 = *var_kq2_rdb2_slot;
        let mut var_kq2_rdb3: f64 = *var_kq2_rdb3_slot;
        let mut var_kq2_rdb4: f64 = *var_kq2_rdb4_slot;
        let mut var_kq2_rdb5: f64 = *var_kq2_rdb5_slot;
        let mut var_kq2_rdb6: f64 = *var_kq2_rdb6_slot;
        let mut var_kq2_rdb7: f64 = *var_kq2_rdb7_slot;
        let mut var_kq2_rdn0: f64 = *var_kq2_rdn0_slot;
        let mut var_kq2_rdn1: f64 = *var_kq2_rdn1_slot;
        let mut var_kq2_rdn2: f64 = *var_kq2_rdn2_slot;
        let mut var_kq2_rdn3: f64 = *var_kq2_rdn3_slot;
        let mut var_kq2_rdn4: f64 = *var_kq2_rdn4_slot;
        let mut var_kq2_rdn5: f64 = *var_kq2_rdn5_slot;
        let mut var_kq2_rdn6: f64 = *var_kq2_rdn6_slot;
        let mut var_kq2_rdn7: f64 = *var_kq2_rdn7_slot;
        let mut var_kq2_rdn8: f64 = *var_kq2_rdn8_slot;
        let mut var_kq2_rdn9: f64 = *var_kq2_rdn9_slot;
        let mut var_kq2_rv: f64 = *var_kq2_rv_slot;
        let mut var_oikf: f64 = *var_oikf_slot;
        let mut var_oikf_db0: f64 = *var_oikf_db0_slot;
        let mut var_oikf_db1: f64 = *var_oikf_db1_slot;
        let mut var_oikf_db2: f64 = *var_oikf_db2_slot;
        let mut var_oikf_db3: f64 = *var_oikf_db3_slot;
        let mut var_oikf_db4: f64 = *var_oikf_db4_slot;
        let mut var_oikf_db5: f64 = *var_oikf_db5_slot;
        let mut var_oikf_db6: f64 = *var_oikf_db6_slot;
        let mut var_oikf_db7: f64 = *var_oikf_db7_slot;
        let mut var_oikf_dn0: f64 = *var_oikf_dn0_slot;
        let mut var_oikf_dn1: f64 = *var_oikf_dn1_slot;
        let mut var_oikf_dn2: f64 = *var_oikf_dn2_slot;
        let mut var_oikf_dn3: f64 = *var_oikf_dn3_slot;
        let mut var_oikf_dn4: f64 = *var_oikf_dn4_slot;
        let mut var_oikf_dn5: f64 = *var_oikf_dn5_slot;
        let mut var_oikf_dn6: f64 = *var_oikf_dn6_slot;
        let mut var_oikf_dn7: f64 = *var_oikf_dn7_slot;
        let mut var_oikf_dn8: f64 = *var_oikf_dn8_slot;
        let mut var_oikf_dn9: f64 = *var_oikf_dn9_slot;
        let mut var_oikf_rdb0: f64 = *var_oikf_rdb0_slot;
        let mut var_oikf_rdb1: f64 = *var_oikf_rdb1_slot;
        let mut var_oikf_rdb2: f64 = *var_oikf_rdb2_slot;
        let mut var_oikf_rdb3: f64 = *var_oikf_rdb3_slot;
        let mut var_oikf_rdb4: f64 = *var_oikf_rdb4_slot;
        let mut var_oikf_rdb5: f64 = *var_oikf_rdb5_slot;
        let mut var_oikf_rdb6: f64 = *var_oikf_rdb6_slot;
        let mut var_oikf_rdb7: f64 = *var_oikf_rdb7_slot;
        let mut var_oikf_rdn0: f64 = *var_oikf_rdn0_slot;
        let mut var_oikf_rdn1: f64 = *var_oikf_rdn1_slot;
        let mut var_oikf_rdn2: f64 = *var_oikf_rdn2_slot;
        let mut var_oikf_rdn3: f64 = *var_oikf_rdn3_slot;
        let mut var_oikf_rdn4: f64 = *var_oikf_rdn4_slot;
        let mut var_oikf_rdn5: f64 = *var_oikf_rdn5_slot;
        let mut var_oikf_rdn6: f64 = *var_oikf_rdn6_slot;
        let mut var_oikf_rdn7: f64 = *var_oikf_rdn7_slot;
        let mut var_oikf_rdn8: f64 = *var_oikf_rdn8_slot;
        let mut var_oikf_rdn9: f64 = *var_oikf_rdn9_slot;
        let mut var_oikf_rv: f64 = *var_oikf_rv_slot;
        let mut var_qdc: f64 = *var_qdc_slot;
        let mut var_qdc_db0: f64 = *var_qdc_db0_slot;
        let mut var_qdc_db1: f64 = *var_qdc_db1_slot;
        let mut var_qdc_db2: f64 = *var_qdc_db2_slot;
        let mut var_qdc_db3: f64 = *var_qdc_db3_slot;
        let mut var_qdc_db4: f64 = *var_qdc_db4_slot;
        let mut var_qdc_db5: f64 = *var_qdc_db5_slot;
        let mut var_qdc_db6: f64 = *var_qdc_db6_slot;
        let mut var_qdc_db7: f64 = *var_qdc_db7_slot;
        let mut var_qdc_dn0: f64 = *var_qdc_dn0_slot;
        let mut var_qdc_dn1: f64 = *var_qdc_dn1_slot;
        let mut var_qdc_dn2: f64 = *var_qdc_dn2_slot;
        let mut var_qdc_dn3: f64 = *var_qdc_dn3_slot;
        let mut var_qdc_dn4: f64 = *var_qdc_dn4_slot;
        let mut var_qdc_dn5: f64 = *var_qdc_dn5_slot;
        let mut var_qdc_dn6: f64 = *var_qdc_dn6_slot;
        let mut var_qdc_dn7: f64 = *var_qdc_dn7_slot;
        let mut var_qdc_dn8: f64 = *var_qdc_dn8_slot;
        let mut var_qdc_dn9: f64 = *var_qdc_dn9_slot;
        let mut var_qdc_rdb0: f64 = *var_qdc_rdb0_slot;
        let mut var_qdc_rdb1: f64 = *var_qdc_rdb1_slot;
        let mut var_qdc_rdb2: f64 = *var_qdc_rdb2_slot;
        let mut var_qdc_rdb3: f64 = *var_qdc_rdb3_slot;
        let mut var_qdc_rdb4: f64 = *var_qdc_rdb4_slot;
        let mut var_qdc_rdb5: f64 = *var_qdc_rdb5_slot;
        let mut var_qdc_rdb6: f64 = *var_qdc_rdb6_slot;
        let mut var_qdc_rdb7: f64 = *var_qdc_rdb7_slot;
        let mut var_qdc_rdn0: f64 = *var_qdc_rdn0_slot;
        let mut var_qdc_rdn1: f64 = *var_qdc_rdn1_slot;
        let mut var_qdc_rdn2: f64 = *var_qdc_rdn2_slot;
        let mut var_qdc_rdn3: f64 = *var_qdc_rdn3_slot;
        let mut var_qdc_rdn4: f64 = *var_qdc_rdn4_slot;
        let mut var_qdc_rdn5: f64 = *var_qdc_rdn5_slot;
        let mut var_qdc_rdn6: f64 = *var_qdc_rdn6_slot;
        let mut var_qdc_rdn7: f64 = *var_qdc_rdn7_slot;
        let mut var_qdc_rdn8: f64 = *var_qdc_rdn8_slot;
        let mut var_qdc_rdn9: f64 = *var_qdc_rdn9_slot;
        let mut var_qdc_rv: f64 = *var_qdc_rv_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_db0: f64 = *var_qde_db0_slot;
        let mut var_qde_db1: f64 = *var_qde_db1_slot;
        let mut var_qde_db2: f64 = *var_qde_db2_slot;
        let mut var_qde_db3: f64 = *var_qde_db3_slot;
        let mut var_qde_db4: f64 = *var_qde_db4_slot;
        let mut var_qde_db5: f64 = *var_qde_db5_slot;
        let mut var_qde_db6: f64 = *var_qde_db6_slot;
        let mut var_qde_db7: f64 = *var_qde_db7_slot;
        let mut var_qde_dn0: f64 = *var_qde_dn0_slot;
        let mut var_qde_dn1: f64 = *var_qde_dn1_slot;
        let mut var_qde_dn2: f64 = *var_qde_dn2_slot;
        let mut var_qde_dn3: f64 = *var_qde_dn3_slot;
        let mut var_qde_dn4: f64 = *var_qde_dn4_slot;
        let mut var_qde_dn5: f64 = *var_qde_dn5_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qde_dn7: f64 = *var_qde_dn7_slot;
        let mut var_qde_dn8: f64 = *var_qde_dn8_slot;
        let mut var_qde_dn9: f64 = *var_qde_dn9_slot;
        let mut var_qde_rdb0: f64 = *var_qde_rdb0_slot;
        let mut var_qde_rdb1: f64 = *var_qde_rdb1_slot;
        let mut var_qde_rdb2: f64 = *var_qde_rdb2_slot;
        let mut var_qde_rdb3: f64 = *var_qde_rdb3_slot;
        let mut var_qde_rdb4: f64 = *var_qde_rdb4_slot;
        let mut var_qde_rdb5: f64 = *var_qde_rdb5_slot;
        let mut var_qde_rdb6: f64 = *var_qde_rdb6_slot;
        let mut var_qde_rdb7: f64 = *var_qde_rdb7_slot;
        let mut var_qde_rdn0: f64 = *var_qde_rdn0_slot;
        let mut var_qde_rdn1: f64 = *var_qde_rdn1_slot;
        let mut var_qde_rdn2: f64 = *var_qde_rdn2_slot;
        let mut var_qde_rdn3: f64 = *var_qde_rdn3_slot;
        let mut var_qde_rdn4: f64 = *var_qde_rdn4_slot;
        let mut var_qde_rdn5: f64 = *var_qde_rdn5_slot;
        let mut var_qde_rdn6: f64 = *var_qde_rdn6_slot;
        let mut var_qde_rdn7: f64 = *var_qde_rdn7_slot;
        let mut var_qde_rdn8: f64 = *var_qde_rdn8_slot;
        let mut var_qde_rdn9: f64 = *var_qde_rdn9_slot;
        let mut var_qde_rv: f64 = *var_qde_rv_slot;
        let mut var_tff: f64 = *var_tff_slot;
        let mut var_tff_db0: f64 = *var_tff_db0_slot;
        let mut var_tff_db1: f64 = *var_tff_db1_slot;
        let mut var_tff_db2: f64 = *var_tff_db2_slot;
        let mut var_tff_db3: f64 = *var_tff_db3_slot;
        let mut var_tff_db4: f64 = *var_tff_db4_slot;
        let mut var_tff_db5: f64 = *var_tff_db5_slot;
        let mut var_tff_db6: f64 = *var_tff_db6_slot;
        let mut var_tff_db7: f64 = *var_tff_db7_slot;
        let mut var_tff_dn0: f64 = *var_tff_dn0_slot;
        let mut var_tff_dn1: f64 = *var_tff_dn1_slot;
        let mut var_tff_dn2: f64 = *var_tff_dn2_slot;
        let mut var_tff_dn3: f64 = *var_tff_dn3_slot;
        let mut var_tff_dn4: f64 = *var_tff_dn4_slot;
        let mut var_tff_dn5: f64 = *var_tff_dn5_slot;
        let mut var_tff_dn6: f64 = *var_tff_dn6_slot;
        let mut var_tff_dn7: f64 = *var_tff_dn7_slot;
        let mut var_tff_dn8: f64 = *var_tff_dn8_slot;
        let mut var_tff_dn9: f64 = *var_tff_dn9_slot;
        let mut var_tff_rdb0: f64 = *var_tff_rdb0_slot;
        let mut var_tff_rdb1: f64 = *var_tff_rdb1_slot;
        let mut var_tff_rdb2: f64 = *var_tff_rdb2_slot;
        let mut var_tff_rdb3: f64 = *var_tff_rdb3_slot;
        let mut var_tff_rdb4: f64 = *var_tff_rdb4_slot;
        let mut var_tff_rdb5: f64 = *var_tff_rdb5_slot;
        let mut var_tff_rdb6: f64 = *var_tff_rdb6_slot;
        let mut var_tff_rdb7: f64 = *var_tff_rdb7_slot;
        let mut var_tff_rdn0: f64 = *var_tff_rdn0_slot;
        let mut var_tff_rdn1: f64 = *var_tff_rdn1_slot;
        let mut var_tff_rdn2: f64 = *var_tff_rdn2_slot;
        let mut var_tff_rdn3: f64 = *var_tff_rdn3_slot;
        let mut var_tff_rdn4: f64 = *var_tff_rdn4_slot;
        let mut var_tff_rdn5: f64 = *var_tff_rdn5_slot;
        let mut var_tff_rdn6: f64 = *var_tff_rdn6_slot;
        let mut var_tff_rdn7: f64 = *var_tff_rdn7_slot;
        let mut var_tff_rdn8: f64 = *var_tff_rdn8_slot;
        let mut var_tff_rdn9: f64 = *var_tff_rdn9_slot;
        let mut var_tff_rv: f64 = *var_tff_rv_slot;
        let mut var_vtff: f64 = *var_vtff_slot;
        let mut var_vtff1: f64 = *var_vtff1_slot;
        let mut var_vtff1_db0: f64 = *var_vtff1_db0_slot;
        let mut var_vtff1_db1: f64 = *var_vtff1_db1_slot;
        let mut var_vtff1_db2: f64 = *var_vtff1_db2_slot;
        let mut var_vtff1_db3: f64 = *var_vtff1_db3_slot;
        let mut var_vtff1_db4: f64 = *var_vtff1_db4_slot;
        let mut var_vtff1_db5: f64 = *var_vtff1_db5_slot;
        let mut var_vtff1_db6: f64 = *var_vtff1_db6_slot;
        let mut var_vtff1_db7: f64 = *var_vtff1_db7_slot;
        let mut var_vtff1_dn0: f64 = *var_vtff1_dn0_slot;
        let mut var_vtff1_dn1: f64 = *var_vtff1_dn1_slot;
        let mut var_vtff1_dn2: f64 = *var_vtff1_dn2_slot;
        let mut var_vtff1_dn3: f64 = *var_vtff1_dn3_slot;
        let mut var_vtff1_dn4: f64 = *var_vtff1_dn4_slot;
        let mut var_vtff1_dn5: f64 = *var_vtff1_dn5_slot;
        let mut var_vtff1_dn6: f64 = *var_vtff1_dn6_slot;
        let mut var_vtff1_dn7: f64 = *var_vtff1_dn7_slot;
        let mut var_vtff1_dn8: f64 = *var_vtff1_dn8_slot;
        let mut var_vtff1_dn9: f64 = *var_vtff1_dn9_slot;
        let mut var_vtff1_rdb0: f64 = *var_vtff1_rdb0_slot;
        let mut var_vtff1_rdb1: f64 = *var_vtff1_rdb1_slot;
        let mut var_vtff1_rdb2: f64 = *var_vtff1_rdb2_slot;
        let mut var_vtff1_rdb3: f64 = *var_vtff1_rdb3_slot;
        let mut var_vtff1_rdb4: f64 = *var_vtff1_rdb4_slot;
        let mut var_vtff1_rdb5: f64 = *var_vtff1_rdb5_slot;
        let mut var_vtff1_rdb6: f64 = *var_vtff1_rdb6_slot;
        let mut var_vtff1_rdb7: f64 = *var_vtff1_rdb7_slot;
        let mut var_vtff1_rdn0: f64 = *var_vtff1_rdn0_slot;
        let mut var_vtff1_rdn1: f64 = *var_vtff1_rdn1_slot;
        let mut var_vtff1_rdn2: f64 = *var_vtff1_rdn2_slot;
        let mut var_vtff1_rdn3: f64 = *var_vtff1_rdn3_slot;
        let mut var_vtff1_rdn4: f64 = *var_vtff1_rdn4_slot;
        let mut var_vtff1_rdn5: f64 = *var_vtff1_rdn5_slot;
        let mut var_vtff1_rdn6: f64 = *var_vtff1_rdn6_slot;
        let mut var_vtff1_rdn7: f64 = *var_vtff1_rdn7_slot;
        let mut var_vtff1_rdn8: f64 = *var_vtff1_rdn8_slot;
        let mut var_vtff1_rdn9: f64 = *var_vtff1_rdn9_slot;
        let mut var_vtff1_rv: f64 = *var_vtff1_rv_slot;
        let mut var_vtff_db0: f64 = *var_vtff_db0_slot;
        let mut var_vtff_db1: f64 = *var_vtff_db1_slot;
        let mut var_vtff_db2: f64 = *var_vtff_db2_slot;
        let mut var_vtff_db3: f64 = *var_vtff_db3_slot;
        let mut var_vtff_db4: f64 = *var_vtff_db4_slot;
        let mut var_vtff_db5: f64 = *var_vtff_db5_slot;
        let mut var_vtff_db6: f64 = *var_vtff_db6_slot;
        let mut var_vtff_db7: f64 = *var_vtff_db7_slot;
        let mut var_vtff_dn0: f64 = *var_vtff_dn0_slot;
        let mut var_vtff_dn1: f64 = *var_vtff_dn1_slot;
        let mut var_vtff_dn2: f64 = *var_vtff_dn2_slot;
        let mut var_vtff_dn3: f64 = *var_vtff_dn3_slot;
        let mut var_vtff_dn4: f64 = *var_vtff_dn4_slot;
        let mut var_vtff_dn5: f64 = *var_vtff_dn5_slot;
        let mut var_vtff_dn6: f64 = *var_vtff_dn6_slot;
        let mut var_vtff_dn7: f64 = *var_vtff_dn7_slot;
        let mut var_vtff_dn8: f64 = *var_vtff_dn8_slot;
        let mut var_vtff_dn9: f64 = *var_vtff_dn9_slot;
        let mut var_vtff_rdb0: f64 = *var_vtff_rdb0_slot;
        let mut var_vtff_rdb1: f64 = *var_vtff_rdb1_slot;
        let mut var_vtff_rdb2: f64 = *var_vtff_rdb2_slot;
        let mut var_vtff_rdb3: f64 = *var_vtff_rdb3_slot;
        let mut var_vtff_rdb4: f64 = *var_vtff_rdb4_slot;
        let mut var_vtff_rdb5: f64 = *var_vtff_rdb5_slot;
        let mut var_vtff_rdb6: f64 = *var_vtff_rdb6_slot;
        let mut var_vtff_rdb7: f64 = *var_vtff_rdb7_slot;
        let mut var_vtff_rdn0: f64 = *var_vtff_rdn0_slot;
        let mut var_vtff_rdn1: f64 = *var_vtff_rdn1_slot;
        let mut var_vtff_rdn2: f64 = *var_vtff_rdn2_slot;
        let mut var_vtff_rdn3: f64 = *var_vtff_rdn3_slot;
        let mut var_vtff_rdn4: f64 = *var_vtff_rdn4_slot;
        let mut var_vtff_rdn5: f64 = *var_vtff_rdn5_slot;
        let mut var_vtff_rdn6: f64 = *var_vtff_rdn6_slot;
        let mut var_vtff_rdn7: f64 = *var_vtff_rdn7_slot;
        let mut var_vtff_rdn8: f64 = *var_vtff_rdn8_slot;
        let mut var_vtff_rdn9: f64 = *var_vtff_rdn9_slot;
        let mut var_vtff_rv: f64 = *var_vtff_rv_slot;

        let assign1320_e1679: f64 = (var_vbici * p.p81);
        let assign1320_e1680: f64 = (1.0 + assign1320_e1679);
        let assign1320_e1681: f64 = (var_oikf * assign1320_e1680);
        var_oikf = assign1320_e1681;
        var_oikf_dn0 = ((var_oikf_dn0 * assign1320_e1680) + (var_oikf * (var_vbici_dn0 * p.p81)));
        var_oikf_dn1 = ((var_oikf_dn1 * assign1320_e1680) + (var_oikf * (var_vbici_dn1 * p.p81)));
        var_oikf_dn2 = ((var_oikf_dn2 * assign1320_e1680) + (var_oikf * (var_vbici_dn2 * p.p81)));
        var_oikf_dn3 = ((var_oikf_dn3 * assign1320_e1680) + (var_oikf * (var_vbici_dn3 * p.p81)));
        var_oikf_dn4 = ((var_oikf_dn4 * assign1320_e1680) + (var_oikf * (var_vbici_dn4 * p.p81)));
        var_oikf_dn5 = ((var_oikf_dn5 * assign1320_e1680) + (var_oikf * (var_vbici_dn5 * p.p81)));
        var_oikf_dn6 = ((var_oikf_dn6 * assign1320_e1680) + (var_oikf * (var_vbici_dn6 * p.p81)));
        var_oikf_dn7 = ((var_oikf_dn7 * assign1320_e1680) + (var_oikf * (var_vbici_dn7 * p.p81)));
        var_oikf_dn8 = ((var_oikf_dn8 * assign1320_e1680) + (var_oikf * (var_vbici_dn8 * p.p81)));
        var_oikf_dn9 = ((var_oikf_dn9 * assign1320_e1680) + (var_oikf * (var_vbici_dn9 * p.p81)));
        var_oikf_db0 = ((var_oikf_db0 * assign1320_e1680) + (var_oikf * (var_vbici_db0 * p.p81)));
        var_oikf_db1 = ((var_oikf_db1 * assign1320_e1680) + (var_oikf * (var_vbici_db1 * p.p81)));
        var_oikf_db2 = ((var_oikf_db2 * assign1320_e1680) + (var_oikf * (var_vbici_db2 * p.p81)));
        var_oikf_db3 = ((var_oikf_db3 * assign1320_e1680) + (var_oikf * (var_vbici_db3 * p.p81)));
        var_oikf_db4 = ((var_oikf_db4 * assign1320_e1680) + (var_oikf * (var_vbici_db4 * p.p81)));
        var_oikf_db5 = ((var_oikf_db5 * assign1320_e1680) + (var_oikf * (var_vbici_db5 * p.p81)));
        var_oikf_db6 = ((var_oikf_db6 * assign1320_e1680) + (var_oikf * (var_vbici_db6 * p.p81)));
        var_oikf_db7 = ((var_oikf_db7 * assign1320_e1680) + (var_oikf * (var_vbici_db7 * p.p81)));
        var_oikf_rv = 0.0;
        var_oikf_rdn0 = 0.0;
        var_oikf_rdn1 = 0.0;
        var_oikf_rdn2 = 0.0;
        var_oikf_rdn3 = 0.0;
        var_oikf_rdn4 = 0.0;
        var_oikf_rdn5 = 0.0;
        var_oikf_rdn6 = 0.0;
        var_oikf_rdn7 = 0.0;
        var_oikf_rdn8 = 0.0;
        var_oikf_rdn9 = 0.0;
        var_oikf_rdb0 = 0.0;
        var_oikf_rdb1 = 0.0;
        var_oikf_rdb2 = 0.0;
        var_oikf_rdb3 = 0.0;
        var_oikf_rdb4 = 0.0;
        var_oikf_rdb5 = 0.0;
        var_oikf_rdb6 = 0.0;
        var_oikf_rdb7 = 0.0;

        let assign1330_e1684: f64 = (var_ifwd * var_oikf);
        let assign1330_e1687: f64 = (var_ibwd * var_oikr);
        let assign1330_e1688: f64 = (assign1330_e1684 + assign1330_e1687);
        var_kq2 = assign1330_e1688;
        var_kq2_dn0 = (((var_ifwd_dn0 * var_oikf) + (var_ifwd * var_oikf_dn0)) + ((var_ibwd_dn0 * var_oikr) + (var_ibwd * var_oikr_dn0)));
        var_kq2_dn1 = (((var_ifwd_dn1 * var_oikf) + (var_ifwd * var_oikf_dn1)) + ((var_ibwd_dn1 * var_oikr) + (var_ibwd * var_oikr_dn1)));
        var_kq2_dn2 = (((var_ifwd_dn2 * var_oikf) + (var_ifwd * var_oikf_dn2)) + ((var_ibwd_dn2 * var_oikr) + (var_ibwd * var_oikr_dn2)));
        var_kq2_dn3 = (((var_ifwd_dn3 * var_oikf) + (var_ifwd * var_oikf_dn3)) + ((var_ibwd_dn3 * var_oikr) + (var_ibwd * var_oikr_dn3)));
        var_kq2_dn4 = (((var_ifwd_dn4 * var_oikf) + (var_ifwd * var_oikf_dn4)) + ((var_ibwd_dn4 * var_oikr) + (var_ibwd * var_oikr_dn4)));
        var_kq2_dn5 = (((var_ifwd_dn5 * var_oikf) + (var_ifwd * var_oikf_dn5)) + ((var_ibwd_dn5 * var_oikr) + (var_ibwd * var_oikr_dn5)));
        var_kq2_dn6 = (((var_ifwd_dn6 * var_oikf) + (var_ifwd * var_oikf_dn6)) + ((var_ibwd_dn6 * var_oikr) + (var_ibwd * var_oikr_dn6)));
        var_kq2_dn7 = (((var_ifwd_dn7 * var_oikf) + (var_ifwd * var_oikf_dn7)) + ((var_ibwd_dn7 * var_oikr) + (var_ibwd * var_oikr_dn7)));
        var_kq2_dn8 = (((var_ifwd_dn8 * var_oikf) + (var_ifwd * var_oikf_dn8)) + ((var_ibwd_dn8 * var_oikr) + (var_ibwd * var_oikr_dn8)));
        var_kq2_dn9 = (((var_ifwd_dn9 * var_oikf) + (var_ifwd * var_oikf_dn9)) + ((var_ibwd_dn9 * var_oikr) + (var_ibwd * var_oikr_dn9)));
        var_kq2_db0 = (((var_ifwd_db0 * var_oikf) + (var_ifwd * var_oikf_db0)) + ((var_ibwd_db0 * var_oikr) + (var_ibwd * var_oikr_db0)));
        var_kq2_db1 = (((var_ifwd_db1 * var_oikf) + (var_ifwd * var_oikf_db1)) + ((var_ibwd_db1 * var_oikr) + (var_ibwd * var_oikr_db1)));
        var_kq2_db2 = (((var_ifwd_db2 * var_oikf) + (var_ifwd * var_oikf_db2)) + ((var_ibwd_db2 * var_oikr) + (var_ibwd * var_oikr_db2)));
        var_kq2_db3 = (((var_ifwd_db3 * var_oikf) + (var_ifwd * var_oikf_db3)) + ((var_ibwd_db3 * var_oikr) + (var_ibwd * var_oikr_db3)));
        var_kq2_db4 = (((var_ifwd_db4 * var_oikf) + (var_ifwd * var_oikf_db4)) + ((var_ibwd_db4 * var_oikr) + (var_ibwd * var_oikr_db4)));
        var_kq2_db5 = (((var_ifwd_db5 * var_oikf) + (var_ifwd * var_oikf_db5)) + ((var_ibwd_db5 * var_oikr) + (var_ibwd * var_oikr_db5)));
        var_kq2_db6 = (((var_ifwd_db6 * var_oikf) + (var_ifwd * var_oikf_db6)) + ((var_ibwd_db6 * var_oikr) + (var_ibwd * var_oikr_db6)));
        var_kq2_db7 = (((var_ifwd_db7 * var_oikf) + (var_ifwd * var_oikf_db7)) + ((var_ibwd_db7 * var_oikr) + (var_ibwd * var_oikr_db7)));
        var_kq2_rv = 0.0;
        var_kq2_rdn0 = 0.0;
        var_kq2_rdn1 = 0.0;
        var_kq2_rdn2 = 0.0;
        var_kq2_rdn3 = 0.0;
        var_kq2_rdn4 = 0.0;
        var_kq2_rdn5 = 0.0;
        var_kq2_rdn6 = 0.0;
        var_kq2_rdn7 = 0.0;
        var_kq2_rdn8 = 0.0;
        var_kq2_rdn9 = 0.0;
        var_kq2_rdb0 = 0.0;
        var_kq2_rdb1 = 0.0;
        var_kq2_rdb2 = 0.0;
        var_kq2_rdb3 = 0.0;
        var_kq2_rdb4 = 0.0;
        var_kq2_rdb5 = 0.0;
        var_kq2_rdb6 = 0.0;
        var_kq2_rdb7 = 0.0;

        let assign1340_e1692: f64 = (var_vbiei * var_ovar);
        let assign1340_e1693: f64 = (1.0 - assign1340_e1692);
        let assign1340_e1696: f64 = (var_vbici * var_ovaf);
        let assign1340_e1697: f64 = (assign1340_e1693 - assign1340_e1696);
        var_ikq1 = assign1340_e1697;
        var_ikq1_dn0 = ((-((var_vbiei_dn0 * var_ovar) + (var_vbiei * var_ovar_dn0))) - ((var_vbici_dn0 * var_ovaf) + (var_vbici * var_ovaf_dn0)));
        var_ikq1_dn1 = ((-((var_vbiei_dn1 * var_ovar) + (var_vbiei * var_ovar_dn1))) - ((var_vbici_dn1 * var_ovaf) + (var_vbici * var_ovaf_dn1)));
        var_ikq1_dn2 = ((-((var_vbiei_dn2 * var_ovar) + (var_vbiei * var_ovar_dn2))) - ((var_vbici_dn2 * var_ovaf) + (var_vbici * var_ovaf_dn2)));
        var_ikq1_dn3 = ((-((var_vbiei_dn3 * var_ovar) + (var_vbiei * var_ovar_dn3))) - ((var_vbici_dn3 * var_ovaf) + (var_vbici * var_ovaf_dn3)));
        var_ikq1_dn4 = ((-((var_vbiei_dn4 * var_ovar) + (var_vbiei * var_ovar_dn4))) - ((var_vbici_dn4 * var_ovaf) + (var_vbici * var_ovaf_dn4)));
        var_ikq1_dn5 = ((-((var_vbiei_dn5 * var_ovar) + (var_vbiei * var_ovar_dn5))) - ((var_vbici_dn5 * var_ovaf) + (var_vbici * var_ovaf_dn5)));
        var_ikq1_dn6 = ((-((var_vbiei_dn6 * var_ovar) + (var_vbiei * var_ovar_dn6))) - ((var_vbici_dn6 * var_ovaf) + (var_vbici * var_ovaf_dn6)));
        var_ikq1_dn7 = ((-((var_vbiei_dn7 * var_ovar) + (var_vbiei * var_ovar_dn7))) - ((var_vbici_dn7 * var_ovaf) + (var_vbici * var_ovaf_dn7)));
        var_ikq1_dn8 = ((-((var_vbiei_dn8 * var_ovar) + (var_vbiei * var_ovar_dn8))) - ((var_vbici_dn8 * var_ovaf) + (var_vbici * var_ovaf_dn8)));
        var_ikq1_dn9 = ((-((var_vbiei_dn9 * var_ovar) + (var_vbiei * var_ovar_dn9))) - ((var_vbici_dn9 * var_ovaf) + (var_vbici * var_ovaf_dn9)));
        var_ikq1_db0 = ((-((var_vbiei_db0 * var_ovar) + (var_vbiei * var_ovar_db0))) - ((var_vbici_db0 * var_ovaf) + (var_vbici * var_ovaf_db0)));
        var_ikq1_db1 = ((-((var_vbiei_db1 * var_ovar) + (var_vbiei * var_ovar_db1))) - ((var_vbici_db1 * var_ovaf) + (var_vbici * var_ovaf_db1)));
        var_ikq1_db2 = ((-((var_vbiei_db2 * var_ovar) + (var_vbiei * var_ovar_db2))) - ((var_vbici_db2 * var_ovaf) + (var_vbici * var_ovaf_db2)));
        var_ikq1_db3 = ((-((var_vbiei_db3 * var_ovar) + (var_vbiei * var_ovar_db3))) - ((var_vbici_db3 * var_ovaf) + (var_vbici * var_ovaf_db3)));
        var_ikq1_db4 = ((-((var_vbiei_db4 * var_ovar) + (var_vbiei * var_ovar_db4))) - ((var_vbici_db4 * var_ovaf) + (var_vbici * var_ovaf_db4)));
        var_ikq1_db5 = ((-((var_vbiei_db5 * var_ovar) + (var_vbiei * var_ovar_db5))) - ((var_vbici_db5 * var_ovaf) + (var_vbici * var_ovaf_db5)));
        var_ikq1_db6 = ((-((var_vbiei_db6 * var_ovar) + (var_vbiei * var_ovar_db6))) - ((var_vbici_db6 * var_ovaf) + (var_vbici * var_ovaf_db6)));
        var_ikq1_db7 = ((-((var_vbiei_db7 * var_ovar) + (var_vbiei * var_ovar_db7))) - ((var_vbici_db7 * var_ovaf) + (var_vbici * var_ovaf_db7)));
        var_ikq1_rv = 0.0;
        var_ikq1_rdn0 = 0.0;
        var_ikq1_rdn1 = 0.0;
        var_ikq1_rdn2 = 0.0;
        var_ikq1_rdn3 = 0.0;
        var_ikq1_rdn4 = 0.0;
        var_ikq1_rdn5 = 0.0;
        var_ikq1_rdn6 = 0.0;
        var_ikq1_rdn7 = 0.0;
        var_ikq1_rdn8 = 0.0;
        var_ikq1_rdn9 = 0.0;
        var_ikq1_rdb0 = 0.0;
        var_ikq1_rdb1 = 0.0;
        var_ikq1_rdb2 = 0.0;
        var_ikq1_rdb3 = 0.0;
        var_ikq1_rdb4 = 0.0;
        var_ikq1_rdb5 = 0.0;
        var_ikq1_rdb6 = 0.0;
        var_ikq1_rdb7 = 0.0;

        let assign1350_e1702: f64 = (4.0 * var_kq2);
        let assign1350_e1703: f64 = (1.0 + assign1350_e1702);
        let assign1350_e1704: f64 = (assign1350_e1703).abs();
        let assign1350_e1706: f64 = (assign1350_e1704).powf(p.p82);
        let assign1350_e1707: f64 = (1.0 + assign1350_e1706);
        var_dkqb = assign1350_e1707;
        var_dkqb_dn0 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn0) } else { (-(4.0 * var_kq2_dn0)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn0) } else { (-(4.0 * var_kq2_dn0)) } / assign1350_e1704))) };
        var_dkqb_dn1 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn1) } else { (-(4.0 * var_kq2_dn1)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn1) } else { (-(4.0 * var_kq2_dn1)) } / assign1350_e1704))) };
        var_dkqb_dn2 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn2) } else { (-(4.0 * var_kq2_dn2)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn2) } else { (-(4.0 * var_kq2_dn2)) } / assign1350_e1704))) };
        var_dkqb_dn3 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn3) } else { (-(4.0 * var_kq2_dn3)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn3) } else { (-(4.0 * var_kq2_dn3)) } / assign1350_e1704))) };
        var_dkqb_dn4 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn4) } else { (-(4.0 * var_kq2_dn4)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn4) } else { (-(4.0 * var_kq2_dn4)) } / assign1350_e1704))) };
        var_dkqb_dn5 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn5) } else { (-(4.0 * var_kq2_dn5)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn5) } else { (-(4.0 * var_kq2_dn5)) } / assign1350_e1704))) };
        var_dkqb_dn6 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn6) } else { (-(4.0 * var_kq2_dn6)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn6) } else { (-(4.0 * var_kq2_dn6)) } / assign1350_e1704))) };
        var_dkqb_dn7 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn7) } else { (-(4.0 * var_kq2_dn7)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn7) } else { (-(4.0 * var_kq2_dn7)) } / assign1350_e1704))) };
        var_dkqb_dn8 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn8) } else { (-(4.0 * var_kq2_dn8)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn8) } else { (-(4.0 * var_kq2_dn8)) } / assign1350_e1704))) };
        var_dkqb_dn9 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn9) } else { (-(4.0 * var_kq2_dn9)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_dn9) } else { (-(4.0 * var_kq2_dn9)) } / assign1350_e1704))) };
        var_dkqb_db0 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db0) } else { (-(4.0 * var_kq2_db0)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db0) } else { (-(4.0 * var_kq2_db0)) } / assign1350_e1704))) };
        var_dkqb_db1 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db1) } else { (-(4.0 * var_kq2_db1)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db1) } else { (-(4.0 * var_kq2_db1)) } / assign1350_e1704))) };
        var_dkqb_db2 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db2) } else { (-(4.0 * var_kq2_db2)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db2) } else { (-(4.0 * var_kq2_db2)) } / assign1350_e1704))) };
        var_dkqb_db3 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db3) } else { (-(4.0 * var_kq2_db3)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db3) } else { (-(4.0 * var_kq2_db3)) } / assign1350_e1704))) };
        var_dkqb_db4 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db4) } else { (-(4.0 * var_kq2_db4)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db4) } else { (-(4.0 * var_kq2_db4)) } / assign1350_e1704))) };
        var_dkqb_db5 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db5) } else { (-(4.0 * var_kq2_db5)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db5) } else { (-(4.0 * var_kq2_db5)) } / assign1350_e1704))) };
        var_dkqb_db6 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db6) } else { (-(4.0 * var_kq2_db6)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db6) } else { (-(4.0 * var_kq2_db6)) } / assign1350_e1704))) };
        var_dkqb_db7 = if 0.0 == 0.0 && ((p.p82) as f64).is_finite() && ((p.p82) as f64).fract() == 0.0 { if p.p82 == 0.0 { 0.0 } else { (p.p82 * ((assign1350_e1704).powf(p.p82 - 1.0) * if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db7) } else { (-(4.0 * var_kq2_db7)) })) } } else { (assign1350_e1706 * (p.p82 * (if assign1350_e1703 >= 0.0 { (4.0 * var_kq2_db7) } else { (-(4.0 * var_kq2_db7)) } / assign1350_e1704))) };
        var_dkqb_rv = 0.0;
        var_dkqb_rdn0 = 0.0;
        var_dkqb_rdn1 = 0.0;
        var_dkqb_rdn2 = 0.0;
        var_dkqb_rdn3 = 0.0;
        var_dkqb_rdn4 = 0.0;
        var_dkqb_rdn5 = 0.0;
        var_dkqb_rdn6 = 0.0;
        var_dkqb_rdn7 = 0.0;
        var_dkqb_rdn8 = 0.0;
        var_dkqb_rdn9 = 0.0;
        var_dkqb_rdb0 = 0.0;
        var_dkqb_rdb1 = 0.0;
        var_dkqb_rdb2 = 0.0;
        var_dkqb_rdb3 = 0.0;
        var_dkqb_rdb4 = 0.0;
        var_dkqb_rdb5 = 0.0;
        var_dkqb_rdb6 = 0.0;
        var_dkqb_rdb7 = 0.0;

        let assign1360_e1710: f64 = (2.0 * var_ikq1);
        let assign1360_e1712: f64 = (assign1360_e1710 / var_dkqb);
        var_ikqb = assign1360_e1712;
        var_ikqb_dn0 = ((((2.0 * var_ikq1_dn0) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn0)) / (var_dkqb * var_dkqb));
        var_ikqb_dn1 = ((((2.0 * var_ikq1_dn1) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn1)) / (var_dkqb * var_dkqb));
        var_ikqb_dn2 = ((((2.0 * var_ikq1_dn2) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn2)) / (var_dkqb * var_dkqb));
        var_ikqb_dn3 = ((((2.0 * var_ikq1_dn3) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn3)) / (var_dkqb * var_dkqb));
        var_ikqb_dn4 = ((((2.0 * var_ikq1_dn4) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn4)) / (var_dkqb * var_dkqb));
        var_ikqb_dn5 = ((((2.0 * var_ikq1_dn5) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn5)) / (var_dkqb * var_dkqb));
        var_ikqb_dn6 = ((((2.0 * var_ikq1_dn6) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn6)) / (var_dkqb * var_dkqb));
        var_ikqb_dn7 = ((((2.0 * var_ikq1_dn7) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn7)) / (var_dkqb * var_dkqb));
        var_ikqb_dn8 = ((((2.0 * var_ikq1_dn8) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn8)) / (var_dkqb * var_dkqb));
        var_ikqb_dn9 = ((((2.0 * var_ikq1_dn9) * var_dkqb) - (assign1360_e1710 * var_dkqb_dn9)) / (var_dkqb * var_dkqb));
        var_ikqb_db0 = ((((2.0 * var_ikq1_db0) * var_dkqb) - (assign1360_e1710 * var_dkqb_db0)) / (var_dkqb * var_dkqb));
        var_ikqb_db1 = ((((2.0 * var_ikq1_db1) * var_dkqb) - (assign1360_e1710 * var_dkqb_db1)) / (var_dkqb * var_dkqb));
        var_ikqb_db2 = ((((2.0 * var_ikq1_db2) * var_dkqb) - (assign1360_e1710 * var_dkqb_db2)) / (var_dkqb * var_dkqb));
        var_ikqb_db3 = ((((2.0 * var_ikq1_db3) * var_dkqb) - (assign1360_e1710 * var_dkqb_db3)) / (var_dkqb * var_dkqb));
        var_ikqb_db4 = ((((2.0 * var_ikq1_db4) * var_dkqb) - (assign1360_e1710 * var_dkqb_db4)) / (var_dkqb * var_dkqb));
        var_ikqb_db5 = ((((2.0 * var_ikq1_db5) * var_dkqb) - (assign1360_e1710 * var_dkqb_db5)) / (var_dkqb * var_dkqb));
        var_ikqb_db6 = ((((2.0 * var_ikq1_db6) * var_dkqb) - (assign1360_e1710 * var_dkqb_db6)) / (var_dkqb * var_dkqb));
        var_ikqb_db7 = ((((2.0 * var_ikq1_db7) * var_dkqb) - (assign1360_e1710 * var_dkqb_db7)) / (var_dkqb * var_dkqb));
        var_ikqb_rv = 0.0;
        var_ikqb_rdn0 = 0.0;
        var_ikqb_rdn1 = 0.0;
        var_ikqb_rdn2 = 0.0;
        var_ikqb_rdn3 = 0.0;
        var_ikqb_rdn4 = 0.0;
        var_ikqb_rdn5 = 0.0;
        var_ikqb_rdn6 = 0.0;
        var_ikqb_rdn7 = 0.0;
        var_ikqb_rdn8 = 0.0;
        var_ikqb_rdn9 = 0.0;
        var_ikqb_rdb0 = 0.0;
        var_ikqb_rdb1 = 0.0;
        var_ikqb_rdb2 = 0.0;
        var_ikqb_rdb3 = 0.0;
        var_ikqb_rdb4 = 0.0;
        var_ikqb_rdb5 = 0.0;
        var_ikqb_rdb6 = 0.0;
        var_ikqb_rdb7 = 0.0;

        let assign1370_e1715: f64 = (var_ibwd * var_ikqb);
        var_itr = assign1370_e1715;
        var_itr_dn0 = ((var_ibwd_dn0 * var_ikqb) + (var_ibwd * var_ikqb_dn0));
        var_itr_dn1 = ((var_ibwd_dn1 * var_ikqb) + (var_ibwd * var_ikqb_dn1));
        var_itr_dn2 = ((var_ibwd_dn2 * var_ikqb) + (var_ibwd * var_ikqb_dn2));
        var_itr_dn3 = ((var_ibwd_dn3 * var_ikqb) + (var_ibwd * var_ikqb_dn3));
        var_itr_dn4 = ((var_ibwd_dn4 * var_ikqb) + (var_ibwd * var_ikqb_dn4));
        var_itr_dn5 = ((var_ibwd_dn5 * var_ikqb) + (var_ibwd * var_ikqb_dn5));
        var_itr_dn6 = ((var_ibwd_dn6 * var_ikqb) + (var_ibwd * var_ikqb_dn6));
        var_itr_dn7 = ((var_ibwd_dn7 * var_ikqb) + (var_ibwd * var_ikqb_dn7));
        var_itr_dn8 = ((var_ibwd_dn8 * var_ikqb) + (var_ibwd * var_ikqb_dn8));
        var_itr_dn9 = ((var_ibwd_dn9 * var_ikqb) + (var_ibwd * var_ikqb_dn9));
        var_itr_db0 = ((var_ibwd_db0 * var_ikqb) + (var_ibwd * var_ikqb_db0));
        var_itr_db1 = ((var_ibwd_db1 * var_ikqb) + (var_ibwd * var_ikqb_db1));
        var_itr_db2 = ((var_ibwd_db2 * var_ikqb) + (var_ibwd * var_ikqb_db2));
        var_itr_db3 = ((var_ibwd_db3 * var_ikqb) + (var_ibwd * var_ikqb_db3));
        var_itr_db4 = ((var_ibwd_db4 * var_ikqb) + (var_ibwd * var_ikqb_db4));
        var_itr_db5 = ((var_ibwd_db5 * var_ikqb) + (var_ibwd * var_ikqb_db5));
        var_itr_db6 = ((var_ibwd_db6 * var_ikqb) + (var_ibwd * var_ikqb_db6));
        var_itr_db7 = ((var_ibwd_db7 * var_ikqb) + (var_ibwd * var_ikqb_db7));
        var_itr_rv = 0.0;
        var_itr_rdn0 = 0.0;
        var_itr_rdn1 = 0.0;
        var_itr_rdn2 = 0.0;
        var_itr_rdn3 = 0.0;
        var_itr_rdn4 = 0.0;
        var_itr_rdn5 = 0.0;
        var_itr_rdn6 = 0.0;
        var_itr_rdn7 = 0.0;
        var_itr_rdn8 = 0.0;
        var_itr_rdn9 = 0.0;
        var_itr_rdb0 = 0.0;
        var_itr_rdb1 = 0.0;
        var_itr_rdb2 = 0.0;
        var_itr_rdb3 = 0.0;
        var_itr_rdb4 = 0.0;
        var_itr_rdb5 = 0.0;
        var_itr_rdb6 = 0.0;
        var_itr_rdb7 = 0.0;

        let assign1380_e1718: f64 = (var_ifwd * var_ikqb);
        var_itzf = assign1380_e1718;
        var_itzf_dn0 = ((var_ifwd_dn0 * var_ikqb) + (var_ifwd * var_ikqb_dn0));
        var_itzf_dn1 = ((var_ifwd_dn1 * var_ikqb) + (var_ifwd * var_ikqb_dn1));
        var_itzf_dn2 = ((var_ifwd_dn2 * var_ikqb) + (var_ifwd * var_ikqb_dn2));
        var_itzf_dn3 = ((var_ifwd_dn3 * var_ikqb) + (var_ifwd * var_ikqb_dn3));
        var_itzf_dn4 = ((var_ifwd_dn4 * var_ikqb) + (var_ifwd * var_ikqb_dn4));
        var_itzf_dn5 = ((var_ifwd_dn5 * var_ikqb) + (var_ifwd * var_ikqb_dn5));
        var_itzf_dn6 = ((var_ifwd_dn6 * var_ikqb) + (var_ifwd * var_ikqb_dn6));
        var_itzf_dn7 = ((var_ifwd_dn7 * var_ikqb) + (var_ifwd * var_ikqb_dn7));
        var_itzf_dn8 = ((var_ifwd_dn8 * var_ikqb) + (var_ifwd * var_ikqb_dn8));
        var_itzf_dn9 = ((var_ifwd_dn9 * var_ikqb) + (var_ifwd * var_ikqb_dn9));
        var_itzf_db0 = ((var_ifwd_db0 * var_ikqb) + (var_ifwd * var_ikqb_db0));
        var_itzf_db1 = ((var_ifwd_db1 * var_ikqb) + (var_ifwd * var_ikqb_db1));
        var_itzf_db2 = ((var_ifwd_db2 * var_ikqb) + (var_ifwd * var_ikqb_db2));
        var_itzf_db3 = ((var_ifwd_db3 * var_ikqb) + (var_ifwd * var_ikqb_db3));
        var_itzf_db4 = ((var_ifwd_db4 * var_ikqb) + (var_ifwd * var_ikqb_db4));
        var_itzf_db5 = ((var_ifwd_db5 * var_ikqb) + (var_ifwd * var_ikqb_db5));
        var_itzf_db6 = ((var_ifwd_db6 * var_ikqb) + (var_ifwd * var_ikqb_db6));
        var_itzf_db7 = ((var_ifwd_db7 * var_ikqb) + (var_ifwd * var_ikqb_db7));
        var_itzf_rv = 0.0;
        var_itzf_rdn0 = 0.0;
        var_itzf_rdn1 = 0.0;
        var_itzf_rdn2 = 0.0;
        var_itzf_rdn3 = 0.0;
        var_itzf_rdn4 = 0.0;
        var_itzf_rdn5 = 0.0;
        var_itzf_rdn6 = 0.0;
        var_itzf_rdn7 = 0.0;
        var_itzf_rdn8 = 0.0;
        var_itzf_rdn9 = 0.0;
        var_itzf_rdb0 = 0.0;
        var_itzf_rdb1 = 0.0;
        var_itzf_rdb2 = 0.0;
        var_itzf_rdb3 = 0.0;
        var_itzf_rdb4 = 0.0;
        var_itzf_rdb5 = 0.0;
        var_itzf_rdb6 = 0.0;
        var_itzf_rdb7 = 0.0;

        let assign1450_e1782: f64 = ((nv1 - nv2) / p.p40);
        let assign1450_e1783: f64 = (assign1450_e1782).abs();
        let assign1450_e1785: f64 = (assign1450_e1783).powf(p.p39);
        var_vtff = assign1450_e1785;
        var_vtff_dn0 = 0.0;
        var_vtff_dn1 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign1450_e1783).powf(p.p39 - 1.0) * if assign1450_e1782 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) })) } } else { (assign1450_e1785 * (p.p39 * (if assign1450_e1782 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) } / assign1450_e1783))) };
        var_vtff_dn2 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign1450_e1783).powf(p.p39 - 1.0) * if assign1450_e1782 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) })) } } else { (assign1450_e1785 * (p.p39 * (if assign1450_e1782 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) } / assign1450_e1783))) };
        var_vtff_dn3 = 0.0;
        var_vtff_dn4 = 0.0;
        var_vtff_dn5 = 0.0;
        var_vtff_dn6 = 0.0;
        var_vtff_dn7 = 0.0;
        var_vtff_dn8 = 0.0;
        var_vtff_dn9 = 0.0;
        var_vtff_db0 = 0.0;
        var_vtff_db1 = 0.0;
        var_vtff_db2 = 0.0;
        var_vtff_db3 = 0.0;
        var_vtff_db4 = 0.0;
        var_vtff_db5 = 0.0;
        var_vtff_db6 = 0.0;
        var_vtff_db7 = 0.0;
        var_vtff_rv = 0.0;
        var_vtff_rdn0 = 0.0;
        var_vtff_rdn1 = 0.0;
        var_vtff_rdn2 = 0.0;
        var_vtff_rdn3 = 0.0;
        var_vtff_rdn4 = 0.0;
        var_vtff_rdn5 = 0.0;
        var_vtff_rdn6 = 0.0;
        var_vtff_rdn7 = 0.0;
        var_vtff_rdn8 = 0.0;
        var_vtff_rdn9 = 0.0;
        var_vtff_rdb0 = 0.0;
        var_vtff_rdb1 = 0.0;
        var_vtff_rdb2 = 0.0;
        var_vtff_rdb3 = 0.0;
        var_vtff_rdb4 = 0.0;
        var_vtff_rdb5 = 0.0;
        var_vtff_rdb6 = 0.0;
        var_vtff_rdb7 = 0.0;

        let assign1460_e1788: f64 = (1.0 + var_vtff);
        let assign1460_e1791: f64 = (1.0 / p.p39);
        let assign1460_e1792: f64 = (assign1460_e1788).powf(assign1460_e1791);
        let assign1460_e1794: f64 = (assign1460_e1792 - 1.0);
        var_vtff1 = assign1460_e1794;
        var_vtff1_dn0 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn0)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn0 / assign1460_e1788))) };
        var_vtff1_dn1 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn1)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn1 / assign1460_e1788))) };
        var_vtff1_dn2 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn2)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn2 / assign1460_e1788))) };
        var_vtff1_dn3 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn3)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn3 / assign1460_e1788))) };
        var_vtff1_dn4 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn4)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn4 / assign1460_e1788))) };
        var_vtff1_dn5 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn5)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn5 / assign1460_e1788))) };
        var_vtff1_dn6 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn6)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn6 / assign1460_e1788))) };
        var_vtff1_dn7 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn7)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn7 / assign1460_e1788))) };
        var_vtff1_dn8 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn8)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn8 / assign1460_e1788))) };
        var_vtff1_dn9 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_dn9)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_dn9 / assign1460_e1788))) };
        var_vtff1_db0 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db0)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db0 / assign1460_e1788))) };
        var_vtff1_db1 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db1)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db1 / assign1460_e1788))) };
        var_vtff1_db2 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db2)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db2 / assign1460_e1788))) };
        var_vtff1_db3 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db3)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db3 / assign1460_e1788))) };
        var_vtff1_db4 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db4)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db4 / assign1460_e1788))) };
        var_vtff1_db5 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db5)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db5 / assign1460_e1788))) };
        var_vtff1_db6 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db6)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db6 / assign1460_e1788))) };
        var_vtff1_db7 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * var_vtff_db7)) } } else { (assign1460_e1792 * (assign1460_e1791 * (var_vtff_db7 / assign1460_e1788))) };
        var_vtff1_rv = 0.0;
        var_vtff1_rdn0 = 0.0;
        var_vtff1_rdn1 = 0.0;
        var_vtff1_rdn2 = 0.0;
        var_vtff1_rdn3 = 0.0;
        var_vtff1_rdn4 = 0.0;
        var_vtff1_rdn5 = 0.0;
        var_vtff1_rdn6 = 0.0;
        var_vtff1_rdn7 = 0.0;
        var_vtff1_rdn8 = 0.0;
        var_vtff1_rdn9 = 0.0;
        var_vtff1_rdb0 = 0.0;
        var_vtff1_rdb1 = 0.0;
        var_vtff1_rdb2 = 0.0;
        var_vtff1_rdb3 = 0.0;
        var_vtff1_rdb4 = 0.0;
        var_vtff1_rdb5 = 0.0;
        var_vtff1_rdb6 = 0.0;
        var_vtff1_rdb7 = 0.0;

        let assign1470_e1799: f64 = (p.p41 * var_vtff1);
        let assign1470_e1800: f64 = (1.0 + assign1470_e1799);
        let assign1470_e1801: f64 = (p.p19 * assign1470_e1800);
        var_tff = assign1470_e1801;
        var_tff_dn0 = (p.p19 * (p.p41 * var_vtff1_dn0));
        var_tff_dn1 = (p.p19 * (p.p41 * var_vtff1_dn1));
        var_tff_dn2 = (p.p19 * (p.p41 * var_vtff1_dn2));
        var_tff_dn3 = (p.p19 * (p.p41 * var_vtff1_dn3));
        var_tff_dn4 = (p.p19 * (p.p41 * var_vtff1_dn4));
        var_tff_dn5 = (p.p19 * (p.p41 * var_vtff1_dn5));
        var_tff_dn6 = (p.p19 * (p.p41 * var_vtff1_dn6));
        var_tff_dn7 = (p.p19 * (p.p41 * var_vtff1_dn7));
        var_tff_dn8 = (p.p19 * (p.p41 * var_vtff1_dn8));
        var_tff_dn9 = (p.p19 * (p.p41 * var_vtff1_dn9));
        var_tff_db0 = (p.p19 * (p.p41 * var_vtff1_db0));
        var_tff_db1 = (p.p19 * (p.p41 * var_vtff1_db1));
        var_tff_db2 = (p.p19 * (p.p41 * var_vtff1_db2));
        var_tff_db3 = (p.p19 * (p.p41 * var_vtff1_db3));
        var_tff_db4 = (p.p19 * (p.p41 * var_vtff1_db4));
        var_tff_db5 = (p.p19 * (p.p41 * var_vtff1_db5));
        var_tff_db6 = (p.p19 * (p.p41 * var_vtff1_db6));
        var_tff_db7 = (p.p19 * (p.p41 * var_vtff1_db7));
        var_tff_rv = 0.0;
        var_tff_rdn0 = 0.0;
        var_tff_rdn1 = 0.0;
        var_tff_rdn2 = 0.0;
        var_tff_rdn3 = 0.0;
        var_tff_rdn4 = 0.0;
        var_tff_rdn5 = 0.0;
        var_tff_rdn6 = 0.0;
        var_tff_rdn7 = 0.0;
        var_tff_rdn8 = 0.0;
        var_tff_rdn9 = 0.0;
        var_tff_rdb0 = 0.0;
        var_tff_rdb1 = 0.0;
        var_tff_rdb2 = 0.0;
        var_tff_rdb3 = 0.0;
        var_tff_rdb4 = 0.0;
        var_tff_rdb5 = 0.0;
        var_tff_rdb6 = 0.0;
        var_tff_rdb7 = 0.0;

        let assign1480_e1804: f64 = (var_tff * var_ifwd);
        var_qde = assign1480_e1804;
        var_qde_dn0 = ((var_tff_dn0 * var_ifwd) + (var_tff * var_ifwd_dn0));
        var_qde_dn1 = ((var_tff_dn1 * var_ifwd) + (var_tff * var_ifwd_dn1));
        var_qde_dn2 = ((var_tff_dn2 * var_ifwd) + (var_tff * var_ifwd_dn2));
        var_qde_dn3 = ((var_tff_dn3 * var_ifwd) + (var_tff * var_ifwd_dn3));
        var_qde_dn4 = ((var_tff_dn4 * var_ifwd) + (var_tff * var_ifwd_dn4));
        var_qde_dn5 = ((var_tff_dn5 * var_ifwd) + (var_tff * var_ifwd_dn5));
        var_qde_dn6 = ((var_tff_dn6 * var_ifwd) + (var_tff * var_ifwd_dn6));
        var_qde_dn7 = ((var_tff_dn7 * var_ifwd) + (var_tff * var_ifwd_dn7));
        var_qde_dn8 = ((var_tff_dn8 * var_ifwd) + (var_tff * var_ifwd_dn8));
        var_qde_dn9 = ((var_tff_dn9 * var_ifwd) + (var_tff * var_ifwd_dn9));
        var_qde_db0 = ((var_tff_db0 * var_ifwd) + (var_tff * var_ifwd_db0));
        var_qde_db1 = ((var_tff_db1 * var_ifwd) + (var_tff * var_ifwd_db1));
        var_qde_db2 = ((var_tff_db2 * var_ifwd) + (var_tff * var_ifwd_db2));
        var_qde_db3 = ((var_tff_db3 * var_ifwd) + (var_tff * var_ifwd_db3));
        var_qde_db4 = ((var_tff_db4 * var_ifwd) + (var_tff * var_ifwd_db4));
        var_qde_db5 = ((var_tff_db5 * var_ifwd) + (var_tff * var_ifwd_db5));
        var_qde_db6 = ((var_tff_db6 * var_ifwd) + (var_tff * var_ifwd_db6));
        var_qde_db7 = ((var_tff_db7 * var_ifwd) + (var_tff * var_ifwd_db7));
        var_qde_rv = 0.0;
        var_qde_rdn0 = 0.0;
        var_qde_rdn1 = 0.0;
        var_qde_rdn2 = 0.0;
        var_qde_rdn3 = 0.0;
        var_qde_rdn4 = 0.0;
        var_qde_rdn5 = 0.0;
        var_qde_rdn6 = 0.0;
        var_qde_rdn7 = 0.0;
        var_qde_rdn8 = 0.0;
        var_qde_rdn9 = 0.0;
        var_qde_rdb0 = 0.0;
        var_qde_rdb1 = 0.0;
        var_qde_rdb2 = 0.0;
        var_qde_rdb3 = 0.0;
        var_qde_rdb4 = 0.0;
        var_qde_rdb5 = 0.0;
        var_qde_rdb6 = 0.0;
        var_qde_rdb7 = 0.0;

        let assign1490_e1807: f64 = (p.p73 * var_itr);
        var_qdc = assign1490_e1807;
        var_qdc_dn0 = (p.p73 * var_itr_dn0);
        var_qdc_dn1 = (p.p73 * var_itr_dn1);
        var_qdc_dn2 = (p.p73 * var_itr_dn2);
        var_qdc_dn3 = (p.p73 * var_itr_dn3);
        var_qdc_dn4 = (p.p73 * var_itr_dn4);
        var_qdc_dn5 = (p.p73 * var_itr_dn5);
        var_qdc_dn6 = (p.p73 * var_itr_dn6);
        var_qdc_dn7 = (p.p73 * var_itr_dn7);
        var_qdc_dn8 = (p.p73 * var_itr_dn8);
        var_qdc_dn9 = (p.p73 * var_itr_dn9);
        var_qdc_db0 = (p.p73 * var_itr_db0);
        var_qdc_db1 = (p.p73 * var_itr_db1);
        var_qdc_db2 = (p.p73 * var_itr_db2);
        var_qdc_db3 = (p.p73 * var_itr_db3);
        var_qdc_db4 = (p.p73 * var_itr_db4);
        var_qdc_db5 = (p.p73 * var_itr_db5);
        var_qdc_db6 = (p.p73 * var_itr_db6);
        var_qdc_db7 = (p.p73 * var_itr_db7);
        var_qdc_rv = 0.0;
        var_qdc_rdn0 = 0.0;
        var_qdc_rdn1 = 0.0;
        var_qdc_rdn2 = 0.0;
        var_qdc_rdn3 = 0.0;
        var_qdc_rdn4 = 0.0;
        var_qdc_rdn5 = 0.0;
        var_qdc_rdn6 = 0.0;
        var_qdc_rdn7 = 0.0;
        var_qdc_rdn8 = 0.0;
        var_qdc_rdn9 = 0.0;
        var_qdc_rdb0 = 0.0;
        var_qdc_rdb1 = 0.0;
        var_qdc_rdb2 = 0.0;
        var_qdc_rdb3 = 0.0;
        var_qdc_rdb4 = 0.0;
        var_qdc_rdb5 = 0.0;
        var_qdc_rdb6 = 0.0;
        var_qdc_rdb7 = 0.0;

        *var_dkqb_slot = var_dkqb;
        *var_dkqb_db0_slot = var_dkqb_db0;
        *var_dkqb_db1_slot = var_dkqb_db1;
        *var_dkqb_db2_slot = var_dkqb_db2;
        *var_dkqb_db3_slot = var_dkqb_db3;
        *var_dkqb_db4_slot = var_dkqb_db4;
        *var_dkqb_db5_slot = var_dkqb_db5;
        *var_dkqb_db6_slot = var_dkqb_db6;
        *var_dkqb_db7_slot = var_dkqb_db7;
        *var_dkqb_dn0_slot = var_dkqb_dn0;
        *var_dkqb_dn1_slot = var_dkqb_dn1;
        *var_dkqb_dn2_slot = var_dkqb_dn2;
        *var_dkqb_dn3_slot = var_dkqb_dn3;
        *var_dkqb_dn4_slot = var_dkqb_dn4;
        *var_dkqb_dn5_slot = var_dkqb_dn5;
        *var_dkqb_dn6_slot = var_dkqb_dn6;
        *var_dkqb_dn7_slot = var_dkqb_dn7;
        *var_dkqb_dn8_slot = var_dkqb_dn8;
        *var_dkqb_dn9_slot = var_dkqb_dn9;
        *var_dkqb_rdb0_slot = var_dkqb_rdb0;
        *var_dkqb_rdb1_slot = var_dkqb_rdb1;
        *var_dkqb_rdb2_slot = var_dkqb_rdb2;
        *var_dkqb_rdb3_slot = var_dkqb_rdb3;
        *var_dkqb_rdb4_slot = var_dkqb_rdb4;
        *var_dkqb_rdb5_slot = var_dkqb_rdb5;
        *var_dkqb_rdb6_slot = var_dkqb_rdb6;
        *var_dkqb_rdb7_slot = var_dkqb_rdb7;
        *var_dkqb_rdn0_slot = var_dkqb_rdn0;
        *var_dkqb_rdn1_slot = var_dkqb_rdn1;
        *var_dkqb_rdn2_slot = var_dkqb_rdn2;
        *var_dkqb_rdn3_slot = var_dkqb_rdn3;
        *var_dkqb_rdn4_slot = var_dkqb_rdn4;
        *var_dkqb_rdn5_slot = var_dkqb_rdn5;
        *var_dkqb_rdn6_slot = var_dkqb_rdn6;
        *var_dkqb_rdn7_slot = var_dkqb_rdn7;
        *var_dkqb_rdn8_slot = var_dkqb_rdn8;
        *var_dkqb_rdn9_slot = var_dkqb_rdn9;
        *var_dkqb_rv_slot = var_dkqb_rv;
        *var_ikq1_slot = var_ikq1;
        *var_ikq1_db0_slot = var_ikq1_db0;
        *var_ikq1_db1_slot = var_ikq1_db1;
        *var_ikq1_db2_slot = var_ikq1_db2;
        *var_ikq1_db3_slot = var_ikq1_db3;
        *var_ikq1_db4_slot = var_ikq1_db4;
        *var_ikq1_db5_slot = var_ikq1_db5;
        *var_ikq1_db6_slot = var_ikq1_db6;
        *var_ikq1_db7_slot = var_ikq1_db7;
        *var_ikq1_dn0_slot = var_ikq1_dn0;
        *var_ikq1_dn1_slot = var_ikq1_dn1;
        *var_ikq1_dn2_slot = var_ikq1_dn2;
        *var_ikq1_dn3_slot = var_ikq1_dn3;
        *var_ikq1_dn4_slot = var_ikq1_dn4;
        *var_ikq1_dn5_slot = var_ikq1_dn5;
        *var_ikq1_dn6_slot = var_ikq1_dn6;
        *var_ikq1_dn7_slot = var_ikq1_dn7;
        *var_ikq1_dn8_slot = var_ikq1_dn8;
        *var_ikq1_dn9_slot = var_ikq1_dn9;
        *var_ikq1_rdb0_slot = var_ikq1_rdb0;
        *var_ikq1_rdb1_slot = var_ikq1_rdb1;
        *var_ikq1_rdb2_slot = var_ikq1_rdb2;
        *var_ikq1_rdb3_slot = var_ikq1_rdb3;
        *var_ikq1_rdb4_slot = var_ikq1_rdb4;
        *var_ikq1_rdb5_slot = var_ikq1_rdb5;
        *var_ikq1_rdb6_slot = var_ikq1_rdb6;
        *var_ikq1_rdb7_slot = var_ikq1_rdb7;
        *var_ikq1_rdn0_slot = var_ikq1_rdn0;
        *var_ikq1_rdn1_slot = var_ikq1_rdn1;
        *var_ikq1_rdn2_slot = var_ikq1_rdn2;
        *var_ikq1_rdn3_slot = var_ikq1_rdn3;
        *var_ikq1_rdn4_slot = var_ikq1_rdn4;
        *var_ikq1_rdn5_slot = var_ikq1_rdn5;
        *var_ikq1_rdn6_slot = var_ikq1_rdn6;
        *var_ikq1_rdn7_slot = var_ikq1_rdn7;
        *var_ikq1_rdn8_slot = var_ikq1_rdn8;
        *var_ikq1_rdn9_slot = var_ikq1_rdn9;
        *var_ikq1_rv_slot = var_ikq1_rv;
        *var_ikqb_slot = var_ikqb;
        *var_ikqb_db0_slot = var_ikqb_db0;
        *var_ikqb_db1_slot = var_ikqb_db1;
        *var_ikqb_db2_slot = var_ikqb_db2;
        *var_ikqb_db3_slot = var_ikqb_db3;
        *var_ikqb_db4_slot = var_ikqb_db4;
        *var_ikqb_db5_slot = var_ikqb_db5;
        *var_ikqb_db6_slot = var_ikqb_db6;
        *var_ikqb_db7_slot = var_ikqb_db7;
        *var_ikqb_dn0_slot = var_ikqb_dn0;
        *var_ikqb_dn1_slot = var_ikqb_dn1;
        *var_ikqb_dn2_slot = var_ikqb_dn2;
        *var_ikqb_dn3_slot = var_ikqb_dn3;
        *var_ikqb_dn4_slot = var_ikqb_dn4;
        *var_ikqb_dn5_slot = var_ikqb_dn5;
        *var_ikqb_dn6_slot = var_ikqb_dn6;
        *var_ikqb_dn7_slot = var_ikqb_dn7;
        *var_ikqb_dn8_slot = var_ikqb_dn8;
        *var_ikqb_dn9_slot = var_ikqb_dn9;
        *var_ikqb_rdb0_slot = var_ikqb_rdb0;
        *var_ikqb_rdb1_slot = var_ikqb_rdb1;
        *var_ikqb_rdb2_slot = var_ikqb_rdb2;
        *var_ikqb_rdb3_slot = var_ikqb_rdb3;
        *var_ikqb_rdb4_slot = var_ikqb_rdb4;
        *var_ikqb_rdb5_slot = var_ikqb_rdb5;
        *var_ikqb_rdb6_slot = var_ikqb_rdb6;
        *var_ikqb_rdb7_slot = var_ikqb_rdb7;
        *var_ikqb_rdn0_slot = var_ikqb_rdn0;
        *var_ikqb_rdn1_slot = var_ikqb_rdn1;
        *var_ikqb_rdn2_slot = var_ikqb_rdn2;
        *var_ikqb_rdn3_slot = var_ikqb_rdn3;
        *var_ikqb_rdn4_slot = var_ikqb_rdn4;
        *var_ikqb_rdn5_slot = var_ikqb_rdn5;
        *var_ikqb_rdn6_slot = var_ikqb_rdn6;
        *var_ikqb_rdn7_slot = var_ikqb_rdn7;
        *var_ikqb_rdn8_slot = var_ikqb_rdn8;
        *var_ikqb_rdn9_slot = var_ikqb_rdn9;
        *var_ikqb_rv_slot = var_ikqb_rv;
        *var_itr_slot = var_itr;
        *var_itr_db0_slot = var_itr_db0;
        *var_itr_db1_slot = var_itr_db1;
        *var_itr_db2_slot = var_itr_db2;
        *var_itr_db3_slot = var_itr_db3;
        *var_itr_db4_slot = var_itr_db4;
        *var_itr_db5_slot = var_itr_db5;
        *var_itr_db6_slot = var_itr_db6;
        *var_itr_db7_slot = var_itr_db7;
        *var_itr_dn0_slot = var_itr_dn0;
        *var_itr_dn1_slot = var_itr_dn1;
        *var_itr_dn2_slot = var_itr_dn2;
        *var_itr_dn3_slot = var_itr_dn3;
        *var_itr_dn4_slot = var_itr_dn4;
        *var_itr_dn5_slot = var_itr_dn5;
        *var_itr_dn6_slot = var_itr_dn6;
        *var_itr_dn7_slot = var_itr_dn7;
        *var_itr_dn8_slot = var_itr_dn8;
        *var_itr_dn9_slot = var_itr_dn9;
        *var_itr_rdb0_slot = var_itr_rdb0;
        *var_itr_rdb1_slot = var_itr_rdb1;
        *var_itr_rdb2_slot = var_itr_rdb2;
        *var_itr_rdb3_slot = var_itr_rdb3;
        *var_itr_rdb4_slot = var_itr_rdb4;
        *var_itr_rdb5_slot = var_itr_rdb5;
        *var_itr_rdb6_slot = var_itr_rdb6;
        *var_itr_rdb7_slot = var_itr_rdb7;
        *var_itr_rdn0_slot = var_itr_rdn0;
        *var_itr_rdn1_slot = var_itr_rdn1;
        *var_itr_rdn2_slot = var_itr_rdn2;
        *var_itr_rdn3_slot = var_itr_rdn3;
        *var_itr_rdn4_slot = var_itr_rdn4;
        *var_itr_rdn5_slot = var_itr_rdn5;
        *var_itr_rdn6_slot = var_itr_rdn6;
        *var_itr_rdn7_slot = var_itr_rdn7;
        *var_itr_rdn8_slot = var_itr_rdn8;
        *var_itr_rdn9_slot = var_itr_rdn9;
        *var_itr_rv_slot = var_itr_rv;
        *var_itzf_slot = var_itzf;
        *var_itzf_db0_slot = var_itzf_db0;
        *var_itzf_db1_slot = var_itzf_db1;
        *var_itzf_db2_slot = var_itzf_db2;
        *var_itzf_db3_slot = var_itzf_db3;
        *var_itzf_db4_slot = var_itzf_db4;
        *var_itzf_db5_slot = var_itzf_db5;
        *var_itzf_db6_slot = var_itzf_db6;
        *var_itzf_db7_slot = var_itzf_db7;
        *var_itzf_dn0_slot = var_itzf_dn0;
        *var_itzf_dn1_slot = var_itzf_dn1;
        *var_itzf_dn2_slot = var_itzf_dn2;
        *var_itzf_dn3_slot = var_itzf_dn3;
        *var_itzf_dn4_slot = var_itzf_dn4;
        *var_itzf_dn5_slot = var_itzf_dn5;
        *var_itzf_dn6_slot = var_itzf_dn6;
        *var_itzf_dn7_slot = var_itzf_dn7;
        *var_itzf_dn8_slot = var_itzf_dn8;
        *var_itzf_dn9_slot = var_itzf_dn9;
        *var_itzf_rdb0_slot = var_itzf_rdb0;
        *var_itzf_rdb1_slot = var_itzf_rdb1;
        *var_itzf_rdb2_slot = var_itzf_rdb2;
        *var_itzf_rdb3_slot = var_itzf_rdb3;
        *var_itzf_rdb4_slot = var_itzf_rdb4;
        *var_itzf_rdb5_slot = var_itzf_rdb5;
        *var_itzf_rdb6_slot = var_itzf_rdb6;
        *var_itzf_rdb7_slot = var_itzf_rdb7;
        *var_itzf_rdn0_slot = var_itzf_rdn0;
        *var_itzf_rdn1_slot = var_itzf_rdn1;
        *var_itzf_rdn2_slot = var_itzf_rdn2;
        *var_itzf_rdn3_slot = var_itzf_rdn3;
        *var_itzf_rdn4_slot = var_itzf_rdn4;
        *var_itzf_rdn5_slot = var_itzf_rdn5;
        *var_itzf_rdn6_slot = var_itzf_rdn6;
        *var_itzf_rdn7_slot = var_itzf_rdn7;
        *var_itzf_rdn8_slot = var_itzf_rdn8;
        *var_itzf_rdn9_slot = var_itzf_rdn9;
        *var_itzf_rv_slot = var_itzf_rv;
        *var_kq2_slot = var_kq2;
        *var_kq2_db0_slot = var_kq2_db0;
        *var_kq2_db1_slot = var_kq2_db1;
        *var_kq2_db2_slot = var_kq2_db2;
        *var_kq2_db3_slot = var_kq2_db3;
        *var_kq2_db4_slot = var_kq2_db4;
        *var_kq2_db5_slot = var_kq2_db5;
        *var_kq2_db6_slot = var_kq2_db6;
        *var_kq2_db7_slot = var_kq2_db7;
        *var_kq2_dn0_slot = var_kq2_dn0;
        *var_kq2_dn1_slot = var_kq2_dn1;
        *var_kq2_dn2_slot = var_kq2_dn2;
        *var_kq2_dn3_slot = var_kq2_dn3;
        *var_kq2_dn4_slot = var_kq2_dn4;
        *var_kq2_dn5_slot = var_kq2_dn5;
        *var_kq2_dn6_slot = var_kq2_dn6;
        *var_kq2_dn7_slot = var_kq2_dn7;
        *var_kq2_dn8_slot = var_kq2_dn8;
        *var_kq2_dn9_slot = var_kq2_dn9;
        *var_kq2_rdb0_slot = var_kq2_rdb0;
        *var_kq2_rdb1_slot = var_kq2_rdb1;
        *var_kq2_rdb2_slot = var_kq2_rdb2;
        *var_kq2_rdb3_slot = var_kq2_rdb3;
        *var_kq2_rdb4_slot = var_kq2_rdb4;
        *var_kq2_rdb5_slot = var_kq2_rdb5;
        *var_kq2_rdb6_slot = var_kq2_rdb6;
        *var_kq2_rdb7_slot = var_kq2_rdb7;
        *var_kq2_rdn0_slot = var_kq2_rdn0;
        *var_kq2_rdn1_slot = var_kq2_rdn1;
        *var_kq2_rdn2_slot = var_kq2_rdn2;
        *var_kq2_rdn3_slot = var_kq2_rdn3;
        *var_kq2_rdn4_slot = var_kq2_rdn4;
        *var_kq2_rdn5_slot = var_kq2_rdn5;
        *var_kq2_rdn6_slot = var_kq2_rdn6;
        *var_kq2_rdn7_slot = var_kq2_rdn7;
        *var_kq2_rdn8_slot = var_kq2_rdn8;
        *var_kq2_rdn9_slot = var_kq2_rdn9;
        *var_kq2_rv_slot = var_kq2_rv;
        *var_oikf_slot = var_oikf;
        *var_oikf_db0_slot = var_oikf_db0;
        *var_oikf_db1_slot = var_oikf_db1;
        *var_oikf_db2_slot = var_oikf_db2;
        *var_oikf_db3_slot = var_oikf_db3;
        *var_oikf_db4_slot = var_oikf_db4;
        *var_oikf_db5_slot = var_oikf_db5;
        *var_oikf_db6_slot = var_oikf_db6;
        *var_oikf_db7_slot = var_oikf_db7;
        *var_oikf_dn0_slot = var_oikf_dn0;
        *var_oikf_dn1_slot = var_oikf_dn1;
        *var_oikf_dn2_slot = var_oikf_dn2;
        *var_oikf_dn3_slot = var_oikf_dn3;
        *var_oikf_dn4_slot = var_oikf_dn4;
        *var_oikf_dn5_slot = var_oikf_dn5;
        *var_oikf_dn6_slot = var_oikf_dn6;
        *var_oikf_dn7_slot = var_oikf_dn7;
        *var_oikf_dn8_slot = var_oikf_dn8;
        *var_oikf_dn9_slot = var_oikf_dn9;
        *var_oikf_rdb0_slot = var_oikf_rdb0;
        *var_oikf_rdb1_slot = var_oikf_rdb1;
        *var_oikf_rdb2_slot = var_oikf_rdb2;
        *var_oikf_rdb3_slot = var_oikf_rdb3;
        *var_oikf_rdb4_slot = var_oikf_rdb4;
        *var_oikf_rdb5_slot = var_oikf_rdb5;
        *var_oikf_rdb6_slot = var_oikf_rdb6;
        *var_oikf_rdb7_slot = var_oikf_rdb7;
        *var_oikf_rdn0_slot = var_oikf_rdn0;
        *var_oikf_rdn1_slot = var_oikf_rdn1;
        *var_oikf_rdn2_slot = var_oikf_rdn2;
        *var_oikf_rdn3_slot = var_oikf_rdn3;
        *var_oikf_rdn4_slot = var_oikf_rdn4;
        *var_oikf_rdn5_slot = var_oikf_rdn5;
        *var_oikf_rdn6_slot = var_oikf_rdn6;
        *var_oikf_rdn7_slot = var_oikf_rdn7;
        *var_oikf_rdn8_slot = var_oikf_rdn8;
        *var_oikf_rdn9_slot = var_oikf_rdn9;
        *var_oikf_rv_slot = var_oikf_rv;
        *var_qdc_slot = var_qdc;
        *var_qdc_db0_slot = var_qdc_db0;
        *var_qdc_db1_slot = var_qdc_db1;
        *var_qdc_db2_slot = var_qdc_db2;
        *var_qdc_db3_slot = var_qdc_db3;
        *var_qdc_db4_slot = var_qdc_db4;
        *var_qdc_db5_slot = var_qdc_db5;
        *var_qdc_db6_slot = var_qdc_db6;
        *var_qdc_db7_slot = var_qdc_db7;
        *var_qdc_dn0_slot = var_qdc_dn0;
        *var_qdc_dn1_slot = var_qdc_dn1;
        *var_qdc_dn2_slot = var_qdc_dn2;
        *var_qdc_dn3_slot = var_qdc_dn3;
        *var_qdc_dn4_slot = var_qdc_dn4;
        *var_qdc_dn5_slot = var_qdc_dn5;
        *var_qdc_dn6_slot = var_qdc_dn6;
        *var_qdc_dn7_slot = var_qdc_dn7;
        *var_qdc_dn8_slot = var_qdc_dn8;
        *var_qdc_dn9_slot = var_qdc_dn9;
        *var_qdc_rdb0_slot = var_qdc_rdb0;
        *var_qdc_rdb1_slot = var_qdc_rdb1;
        *var_qdc_rdb2_slot = var_qdc_rdb2;
        *var_qdc_rdb3_slot = var_qdc_rdb3;
        *var_qdc_rdb4_slot = var_qdc_rdb4;
        *var_qdc_rdb5_slot = var_qdc_rdb5;
        *var_qdc_rdb6_slot = var_qdc_rdb6;
        *var_qdc_rdb7_slot = var_qdc_rdb7;
        *var_qdc_rdn0_slot = var_qdc_rdn0;
        *var_qdc_rdn1_slot = var_qdc_rdn1;
        *var_qdc_rdn2_slot = var_qdc_rdn2;
        *var_qdc_rdn3_slot = var_qdc_rdn3;
        *var_qdc_rdn4_slot = var_qdc_rdn4;
        *var_qdc_rdn5_slot = var_qdc_rdn5;
        *var_qdc_rdn6_slot = var_qdc_rdn6;
        *var_qdc_rdn7_slot = var_qdc_rdn7;
        *var_qdc_rdn8_slot = var_qdc_rdn8;
        *var_qdc_rdn9_slot = var_qdc_rdn9;
        *var_qdc_rv_slot = var_qdc_rv;
        *var_qde_slot = var_qde;
        *var_qde_db0_slot = var_qde_db0;
        *var_qde_db1_slot = var_qde_db1;
        *var_qde_db2_slot = var_qde_db2;
        *var_qde_db3_slot = var_qde_db3;
        *var_qde_db4_slot = var_qde_db4;
        *var_qde_db5_slot = var_qde_db5;
        *var_qde_db6_slot = var_qde_db6;
        *var_qde_db7_slot = var_qde_db7;
        *var_qde_dn0_slot = var_qde_dn0;
        *var_qde_dn1_slot = var_qde_dn1;
        *var_qde_dn2_slot = var_qde_dn2;
        *var_qde_dn3_slot = var_qde_dn3;
        *var_qde_dn4_slot = var_qde_dn4;
        *var_qde_dn5_slot = var_qde_dn5;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qde_dn7_slot = var_qde_dn7;
        *var_qde_dn8_slot = var_qde_dn8;
        *var_qde_dn9_slot = var_qde_dn9;
        *var_qde_rdb0_slot = var_qde_rdb0;
        *var_qde_rdb1_slot = var_qde_rdb1;
        *var_qde_rdb2_slot = var_qde_rdb2;
        *var_qde_rdb3_slot = var_qde_rdb3;
        *var_qde_rdb4_slot = var_qde_rdb4;
        *var_qde_rdb5_slot = var_qde_rdb5;
        *var_qde_rdb6_slot = var_qde_rdb6;
        *var_qde_rdb7_slot = var_qde_rdb7;
        *var_qde_rdn0_slot = var_qde_rdn0;
        *var_qde_rdn1_slot = var_qde_rdn1;
        *var_qde_rdn2_slot = var_qde_rdn2;
        *var_qde_rdn3_slot = var_qde_rdn3;
        *var_qde_rdn4_slot = var_qde_rdn4;
        *var_qde_rdn5_slot = var_qde_rdn5;
        *var_qde_rdn6_slot = var_qde_rdn6;
        *var_qde_rdn7_slot = var_qde_rdn7;
        *var_qde_rdn8_slot = var_qde_rdn8;
        *var_qde_rdn9_slot = var_qde_rdn9;
        *var_qde_rv_slot = var_qde_rv;
        *var_tff_slot = var_tff;
        *var_tff_db0_slot = var_tff_db0;
        *var_tff_db1_slot = var_tff_db1;
        *var_tff_db2_slot = var_tff_db2;
        *var_tff_db3_slot = var_tff_db3;
        *var_tff_db4_slot = var_tff_db4;
        *var_tff_db5_slot = var_tff_db5;
        *var_tff_db6_slot = var_tff_db6;
        *var_tff_db7_slot = var_tff_db7;
        *var_tff_dn0_slot = var_tff_dn0;
        *var_tff_dn1_slot = var_tff_dn1;
        *var_tff_dn2_slot = var_tff_dn2;
        *var_tff_dn3_slot = var_tff_dn3;
        *var_tff_dn4_slot = var_tff_dn4;
        *var_tff_dn5_slot = var_tff_dn5;
        *var_tff_dn6_slot = var_tff_dn6;
        *var_tff_dn7_slot = var_tff_dn7;
        *var_tff_dn8_slot = var_tff_dn8;
        *var_tff_dn9_slot = var_tff_dn9;
        *var_tff_rdb0_slot = var_tff_rdb0;
        *var_tff_rdb1_slot = var_tff_rdb1;
        *var_tff_rdb2_slot = var_tff_rdb2;
        *var_tff_rdb3_slot = var_tff_rdb3;
        *var_tff_rdb4_slot = var_tff_rdb4;
        *var_tff_rdb5_slot = var_tff_rdb5;
        *var_tff_rdb6_slot = var_tff_rdb6;
        *var_tff_rdb7_slot = var_tff_rdb7;
        *var_tff_rdn0_slot = var_tff_rdn0;
        *var_tff_rdn1_slot = var_tff_rdn1;
        *var_tff_rdn2_slot = var_tff_rdn2;
        *var_tff_rdn3_slot = var_tff_rdn3;
        *var_tff_rdn4_slot = var_tff_rdn4;
        *var_tff_rdn5_slot = var_tff_rdn5;
        *var_tff_rdn6_slot = var_tff_rdn6;
        *var_tff_rdn7_slot = var_tff_rdn7;
        *var_tff_rdn8_slot = var_tff_rdn8;
        *var_tff_rdn9_slot = var_tff_rdn9;
        *var_tff_rv_slot = var_tff_rv;
        *var_vtff_slot = var_vtff;
        *var_vtff1_slot = var_vtff1;
        *var_vtff1_db0_slot = var_vtff1_db0;
        *var_vtff1_db1_slot = var_vtff1_db1;
        *var_vtff1_db2_slot = var_vtff1_db2;
        *var_vtff1_db3_slot = var_vtff1_db3;
        *var_vtff1_db4_slot = var_vtff1_db4;
        *var_vtff1_db5_slot = var_vtff1_db5;
        *var_vtff1_db6_slot = var_vtff1_db6;
        *var_vtff1_db7_slot = var_vtff1_db7;
        *var_vtff1_dn0_slot = var_vtff1_dn0;
        *var_vtff1_dn1_slot = var_vtff1_dn1;
        *var_vtff1_dn2_slot = var_vtff1_dn2;
        *var_vtff1_dn3_slot = var_vtff1_dn3;
        *var_vtff1_dn4_slot = var_vtff1_dn4;
        *var_vtff1_dn5_slot = var_vtff1_dn5;
        *var_vtff1_dn6_slot = var_vtff1_dn6;
        *var_vtff1_dn7_slot = var_vtff1_dn7;
        *var_vtff1_dn8_slot = var_vtff1_dn8;
        *var_vtff1_dn9_slot = var_vtff1_dn9;
        *var_vtff1_rdb0_slot = var_vtff1_rdb0;
        *var_vtff1_rdb1_slot = var_vtff1_rdb1;
        *var_vtff1_rdb2_slot = var_vtff1_rdb2;
        *var_vtff1_rdb3_slot = var_vtff1_rdb3;
        *var_vtff1_rdb4_slot = var_vtff1_rdb4;
        *var_vtff1_rdb5_slot = var_vtff1_rdb5;
        *var_vtff1_rdb6_slot = var_vtff1_rdb6;
        *var_vtff1_rdb7_slot = var_vtff1_rdb7;
        *var_vtff1_rdn0_slot = var_vtff1_rdn0;
        *var_vtff1_rdn1_slot = var_vtff1_rdn1;
        *var_vtff1_rdn2_slot = var_vtff1_rdn2;
        *var_vtff1_rdn3_slot = var_vtff1_rdn3;
        *var_vtff1_rdn4_slot = var_vtff1_rdn4;
        *var_vtff1_rdn5_slot = var_vtff1_rdn5;
        *var_vtff1_rdn6_slot = var_vtff1_rdn6;
        *var_vtff1_rdn7_slot = var_vtff1_rdn7;
        *var_vtff1_rdn8_slot = var_vtff1_rdn8;
        *var_vtff1_rdn9_slot = var_vtff1_rdn9;
        *var_vtff1_rv_slot = var_vtff1_rv;
        *var_vtff_db0_slot = var_vtff_db0;
        *var_vtff_db1_slot = var_vtff_db1;
        *var_vtff_db2_slot = var_vtff_db2;
        *var_vtff_db3_slot = var_vtff_db3;
        *var_vtff_db4_slot = var_vtff_db4;
        *var_vtff_db5_slot = var_vtff_db5;
        *var_vtff_db6_slot = var_vtff_db6;
        *var_vtff_db7_slot = var_vtff_db7;
        *var_vtff_dn0_slot = var_vtff_dn0;
        *var_vtff_dn1_slot = var_vtff_dn1;
        *var_vtff_dn2_slot = var_vtff_dn2;
        *var_vtff_dn3_slot = var_vtff_dn3;
        *var_vtff_dn4_slot = var_vtff_dn4;
        *var_vtff_dn5_slot = var_vtff_dn5;
        *var_vtff_dn6_slot = var_vtff_dn6;
        *var_vtff_dn7_slot = var_vtff_dn7;
        *var_vtff_dn8_slot = var_vtff_dn8;
        *var_vtff_dn9_slot = var_vtff_dn9;
        *var_vtff_rdb0_slot = var_vtff_rdb0;
        *var_vtff_rdb1_slot = var_vtff_rdb1;
        *var_vtff_rdb2_slot = var_vtff_rdb2;
        *var_vtff_rdb3_slot = var_vtff_rdb3;
        *var_vtff_rdb4_slot = var_vtff_rdb4;
        *var_vtff_rdb5_slot = var_vtff_rdb5;
        *var_vtff_rdb6_slot = var_vtff_rdb6;
        *var_vtff_rdb7_slot = var_vtff_rdb7;
        *var_vtff_rdn0_slot = var_vtff_rdn0;
        *var_vtff_rdn1_slot = var_vtff_rdn1;
        *var_vtff_rdn2_slot = var_vtff_rdn2;
        *var_vtff_rdn3_slot = var_vtff_rdn3;
        *var_vtff_rdn4_slot = var_vtff_rdn4;
        *var_vtff_rdn5_slot = var_vtff_rdn5;
        *var_vtff_rdn6_slot = var_vtff_rdn6;
        *var_vtff_rdn7_slot = var_vtff_rdn7;
        *var_vtff_rdn8_slot = var_vtff_rdn8;
        *var_vtff_rdn9_slot = var_vtff_rdn9;
        *var_vtff_rv_slot = var_vtff_rv;
    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        var_cjs_t: f64,
        var_cjs_t_db0: f64,
        var_cjs_t_db1: f64,
        var_cjs_t_db2: f64,
        var_cjs_t_db3: f64,
        var_cjs_t_db4: f64,
        var_cjs_t_db5: f64,
        var_cjs_t_db6: f64,
        var_cjs_t_db7: f64,
        var_cjs_t_dn0: f64,
        var_cjs_t_dn1: f64,
        var_cjs_t_dn2: f64,
        var_cjs_t_dn3: f64,
        var_cjs_t_dn4: f64,
        var_cjs_t_dn5: f64,
        var_cjs_t_dn6: f64,
        var_cjs_t_dn7: f64,
        var_cjs_t_dn8: f64,
        var_cjs_t_dn9: f64,
        var_vbiei: f64,
        var_vbiei_db0: f64,
        var_vbiei_db1: f64,
        var_vbiei_db2: f64,
        var_vbiei_db3: f64,
        var_vbiei_db4: f64,
        var_vbiei_db5: f64,
        var_vbiei_db6: f64,
        var_vbiei_db7: f64,
        var_vbiei_dn0: f64,
        var_vbiei_dn1: f64,
        var_vbiei_dn2: f64,
        var_vbiei_dn3: f64,
        var_vbiei_dn4: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vbiei_dn7: f64,
        var_vbiei_dn8: f64,
        var_vbiei_dn9: f64,
        var_veci: f64,
        var_veci_db0: f64,
        var_veci_db1: f64,
        var_veci_db2: f64,
        var_veci_db3: f64,
        var_veci_db4: f64,
        var_veci_db5: f64,
        var_veci_db6: f64,
        var_veci_db7: f64,
        var_veci_dn0: f64,
        var_veci_dn1: f64,
        var_veci_dn2: f64,
        var_veci_dn3: f64,
        var_veci_dn4: f64,
        var_veci_dn5: f64,
        var_veci_dn6: f64,
        var_veci_dn7: f64,
        var_veci_dn8: f64,
        var_veci_dn9: f64,
        var_vje_t: f64,
        var_vje_t_db0: f64,
        var_vje_t_db1: f64,
        var_vje_t_db2: f64,
        var_vje_t_db3: f64,
        var_vje_t_db4: f64,
        var_vje_t_db5: f64,
        var_vje_t_db6: f64,
        var_vje_t_db7: f64,
        var_vje_t_dn0: f64,
        var_vje_t_dn1: f64,
        var_vje_t_dn2: f64,
        var_vje_t_dn3: f64,
        var_vje_t_dn4: f64,
        var_vje_t_dn5: f64,
        var_vje_t_dn6: f64,
        var_vje_t_dn7: f64,
        var_vje_t_dn8: f64,
        var_vje_t_dn9: f64,
        var_vjs_t: f64,
        var_vjs_t_db0: f64,
        var_vjs_t_db1: f64,
        var_vjs_t_db2: f64,
        var_vjs_t_db3: f64,
        var_vjs_t_db4: f64,
        var_vjs_t_db5: f64,
        var_vjs_t_db6: f64,
        var_vjs_t_db7: f64,
        var_vjs_t_dn0: f64,
        var_vjs_t_dn1: f64,
        var_vjs_t_dn2: f64,
        var_vjs_t_dn3: f64,
        var_vjs_t_dn4: f64,
        var_vjs_t_dn5: f64,
        var_vjs_t_dn6: f64,
        var_vjs_t_dn7: f64,
        var_vjs_t_dn8: f64,
        var_vjs_t_dn9: f64,
        var_dv0_slot: &mut f64,
        var_dv0_db0_slot: &mut f64,
        var_dv0_db1_slot: &mut f64,
        var_dv0_db2_slot: &mut f64,
        var_dv0_db3_slot: &mut f64,
        var_dv0_db4_slot: &mut f64,
        var_dv0_db5_slot: &mut f64,
        var_dv0_db6_slot: &mut f64,
        var_dv0_db7_slot: &mut f64,
        var_dv0_dn0_slot: &mut f64,
        var_dv0_dn1_slot: &mut f64,
        var_dv0_dn2_slot: &mut f64,
        var_dv0_dn3_slot: &mut f64,
        var_dv0_dn4_slot: &mut f64,
        var_dv0_dn5_slot: &mut f64,
        var_dv0_dn6_slot: &mut f64,
        var_dv0_dn7_slot: &mut f64,
        var_dv0_dn8_slot: &mut f64,
        var_dv0_dn9_slot: &mut f64,
        var_dv0_rdb0_slot: &mut f64,
        var_dv0_rdb1_slot: &mut f64,
        var_dv0_rdb2_slot: &mut f64,
        var_dv0_rdb3_slot: &mut f64,
        var_dv0_rdb4_slot: &mut f64,
        var_dv0_rdb5_slot: &mut f64,
        var_dv0_rdb6_slot: &mut f64,
        var_dv0_rdb7_slot: &mut f64,
        var_dv0_rdn0_slot: &mut f64,
        var_dv0_rdn1_slot: &mut f64,
        var_dv0_rdn2_slot: &mut f64,
        var_dv0_rdn3_slot: &mut f64,
        var_dv0_rdn4_slot: &mut f64,
        var_dv0_rdn5_slot: &mut f64,
        var_dv0_rdn6_slot: &mut f64,
        var_dv0_rdn7_slot: &mut f64,
        var_dv0_rdn8_slot: &mut f64,
        var_dv0_rdn9_slot: &mut f64,
        var_dv0_rv_slot: &mut f64,
        var_dvh_slot: &mut f64,
        var_dvh_db0_slot: &mut f64,
        var_dvh_db1_slot: &mut f64,
        var_dvh_db2_slot: &mut f64,
        var_dvh_db3_slot: &mut f64,
        var_dvh_db4_slot: &mut f64,
        var_dvh_db5_slot: &mut f64,
        var_dvh_db6_slot: &mut f64,
        var_dvh_db7_slot: &mut f64,
        var_dvh_dn0_slot: &mut f64,
        var_dvh_dn1_slot: &mut f64,
        var_dvh_dn2_slot: &mut f64,
        var_dvh_dn3_slot: &mut f64,
        var_dvh_dn4_slot: &mut f64,
        var_dvh_dn5_slot: &mut f64,
        var_dvh_dn6_slot: &mut f64,
        var_dvh_dn7_slot: &mut f64,
        var_dvh_dn8_slot: &mut f64,
        var_dvh_dn9_slot: &mut f64,
        var_dvh_rdb0_slot: &mut f64,
        var_dvh_rdb1_slot: &mut f64,
        var_dvh_rdb2_slot: &mut f64,
        var_dvh_rdb3_slot: &mut f64,
        var_dvh_rdb4_slot: &mut f64,
        var_dvh_rdb5_slot: &mut f64,
        var_dvh_rdb6_slot: &mut f64,
        var_dvh_rdb7_slot: &mut f64,
        var_dvh_rdn0_slot: &mut f64,
        var_dvh_rdn1_slot: &mut f64,
        var_dvh_rdn2_slot: &mut f64,
        var_dvh_rdn3_slot: &mut f64,
        var_dvh_rdn4_slot: &mut f64,
        var_dvh_rdn5_slot: &mut f64,
        var_dvh_rdn6_slot: &mut f64,
        var_dvh_rdn7_slot: &mut f64,
        var_dvh_rdn8_slot: &mut f64,
        var_dvh_rdn9_slot: &mut f64,
        var_dvh_rv_slot: &mut f64,
        var_guard13_slot: &mut f64,
        var_guard13_db0_slot: &mut f64,
        var_guard13_db1_slot: &mut f64,
        var_guard13_db2_slot: &mut f64,
        var_guard13_db3_slot: &mut f64,
        var_guard13_db4_slot: &mut f64,
        var_guard13_db5_slot: &mut f64,
        var_guard13_db6_slot: &mut f64,
        var_guard13_db7_slot: &mut f64,
        var_guard13_dn0_slot: &mut f64,
        var_guard13_dn1_slot: &mut f64,
        var_guard13_dn2_slot: &mut f64,
        var_guard13_dn3_slot: &mut f64,
        var_guard13_dn4_slot: &mut f64,
        var_guard13_dn5_slot: &mut f64,
        var_guard13_dn6_slot: &mut f64,
        var_guard13_dn7_slot: &mut f64,
        var_guard13_dn8_slot: &mut f64,
        var_guard13_dn9_slot: &mut f64,
        var_guard13_rdb0_slot: &mut f64,
        var_guard13_rdb1_slot: &mut f64,
        var_guard13_rdb2_slot: &mut f64,
        var_guard13_rdb3_slot: &mut f64,
        var_guard13_rdb4_slot: &mut f64,
        var_guard13_rdb5_slot: &mut f64,
        var_guard13_rdb6_slot: &mut f64,
        var_guard13_rdb7_slot: &mut f64,
        var_guard13_rdn0_slot: &mut f64,
        var_guard13_rdn1_slot: &mut f64,
        var_guard13_rdn2_slot: &mut f64,
        var_guard13_rdn3_slot: &mut f64,
        var_guard13_rdn4_slot: &mut f64,
        var_guard13_rdn5_slot: &mut f64,
        var_guard13_rdn6_slot: &mut f64,
        var_guard13_rdn7_slot: &mut f64,
        var_guard13_rdn8_slot: &mut f64,
        var_guard13_rdn9_slot: &mut f64,
        var_guard13_rv_slot: &mut f64,
        var_guard15_slot: &mut f64,
        var_guard15_db0_slot: &mut f64,
        var_guard15_db1_slot: &mut f64,
        var_guard15_db2_slot: &mut f64,
        var_guard15_db3_slot: &mut f64,
        var_guard15_db4_slot: &mut f64,
        var_guard15_db5_slot: &mut f64,
        var_guard15_db6_slot: &mut f64,
        var_guard15_db7_slot: &mut f64,
        var_guard15_dn0_slot: &mut f64,
        var_guard15_dn1_slot: &mut f64,
        var_guard15_dn2_slot: &mut f64,
        var_guard15_dn3_slot: &mut f64,
        var_guard15_dn4_slot: &mut f64,
        var_guard15_dn5_slot: &mut f64,
        var_guard15_dn6_slot: &mut f64,
        var_guard15_dn7_slot: &mut f64,
        var_guard15_dn8_slot: &mut f64,
        var_guard15_dn9_slot: &mut f64,
        var_guard15_rdb0_slot: &mut f64,
        var_guard15_rdb1_slot: &mut f64,
        var_guard15_rdb2_slot: &mut f64,
        var_guard15_rdb3_slot: &mut f64,
        var_guard15_rdb4_slot: &mut f64,
        var_guard15_rdb5_slot: &mut f64,
        var_guard15_rdb6_slot: &mut f64,
        var_guard15_rdb7_slot: &mut f64,
        var_guard15_rdn0_slot: &mut f64,
        var_guard15_rdn1_slot: &mut f64,
        var_guard15_rdn2_slot: &mut f64,
        var_guard15_rdn3_slot: &mut f64,
        var_guard15_rdn4_slot: &mut f64,
        var_guard15_rdn5_slot: &mut f64,
        var_guard15_rdn6_slot: &mut f64,
        var_guard15_rdn7_slot: &mut f64,
        var_guard15_rdn8_slot: &mut f64,
        var_guard15_rdn9_slot: &mut f64,
        var_guard15_rv_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_guard16_db0_slot: &mut f64,
        var_guard16_db1_slot: &mut f64,
        var_guard16_db2_slot: &mut f64,
        var_guard16_db3_slot: &mut f64,
        var_guard16_db4_slot: &mut f64,
        var_guard16_db5_slot: &mut f64,
        var_guard16_db6_slot: &mut f64,
        var_guard16_db7_slot: &mut f64,
        var_guard16_dn0_slot: &mut f64,
        var_guard16_dn1_slot: &mut f64,
        var_guard16_dn2_slot: &mut f64,
        var_guard16_dn3_slot: &mut f64,
        var_guard16_dn4_slot: &mut f64,
        var_guard16_dn5_slot: &mut f64,
        var_guard16_dn6_slot: &mut f64,
        var_guard16_dn7_slot: &mut f64,
        var_guard16_dn8_slot: &mut f64,
        var_guard16_dn9_slot: &mut f64,
        var_guard16_rdb0_slot: &mut f64,
        var_guard16_rdb1_slot: &mut f64,
        var_guard16_rdb2_slot: &mut f64,
        var_guard16_rdb3_slot: &mut f64,
        var_guard16_rdb4_slot: &mut f64,
        var_guard16_rdb5_slot: &mut f64,
        var_guard16_rdb6_slot: &mut f64,
        var_guard16_rdb7_slot: &mut f64,
        var_guard16_rdn0_slot: &mut f64,
        var_guard16_rdn1_slot: &mut f64,
        var_guard16_rdn2_slot: &mut f64,
        var_guard16_rdn3_slot: &mut f64,
        var_guard16_rdn4_slot: &mut f64,
        var_guard16_rdn5_slot: &mut f64,
        var_guard16_rdn6_slot: &mut f64,
        var_guard16_rdn7_slot: &mut f64,
        var_guard16_rdn8_slot: &mut f64,
        var_guard16_rdn9_slot: &mut f64,
        var_guard16_rv_slot: &mut f64,
        var_pwq_slot: &mut f64,
        var_pwq_db0_slot: &mut f64,
        var_pwq_db1_slot: &mut f64,
        var_pwq_db2_slot: &mut f64,
        var_pwq_db3_slot: &mut f64,
        var_pwq_db4_slot: &mut f64,
        var_pwq_db5_slot: &mut f64,
        var_pwq_db6_slot: &mut f64,
        var_pwq_db7_slot: &mut f64,
        var_pwq_dn0_slot: &mut f64,
        var_pwq_dn1_slot: &mut f64,
        var_pwq_dn2_slot: &mut f64,
        var_pwq_dn3_slot: &mut f64,
        var_pwq_dn4_slot: &mut f64,
        var_pwq_dn5_slot: &mut f64,
        var_pwq_dn6_slot: &mut f64,
        var_pwq_dn7_slot: &mut f64,
        var_pwq_dn8_slot: &mut f64,
        var_pwq_dn9_slot: &mut f64,
        var_pwq_rdb0_slot: &mut f64,
        var_pwq_rdb1_slot: &mut f64,
        var_pwq_rdb2_slot: &mut f64,
        var_pwq_rdb3_slot: &mut f64,
        var_pwq_rdb4_slot: &mut f64,
        var_pwq_rdb5_slot: &mut f64,
        var_pwq_rdb6_slot: &mut f64,
        var_pwq_rdb7_slot: &mut f64,
        var_pwq_rdn0_slot: &mut f64,
        var_pwq_rdn1_slot: &mut f64,
        var_pwq_rdn2_slot: &mut f64,
        var_pwq_rdn3_slot: &mut f64,
        var_pwq_rdn4_slot: &mut f64,
        var_pwq_rdn5_slot: &mut f64,
        var_pwq_rdn6_slot: &mut f64,
        var_pwq_rdn7_slot: &mut f64,
        var_pwq_rdn8_slot: &mut f64,
        var_pwq_rdn9_slot: &mut f64,
        var_pwq_rv_slot: &mut f64,
        var_qhi_slot: &mut f64,
        var_qhi_db0_slot: &mut f64,
        var_qhi_db1_slot: &mut f64,
        var_qhi_db2_slot: &mut f64,
        var_qhi_db3_slot: &mut f64,
        var_qhi_db4_slot: &mut f64,
        var_qhi_db5_slot: &mut f64,
        var_qhi_db6_slot: &mut f64,
        var_qhi_db7_slot: &mut f64,
        var_qhi_dn0_slot: &mut f64,
        var_qhi_dn1_slot: &mut f64,
        var_qhi_dn2_slot: &mut f64,
        var_qhi_dn3_slot: &mut f64,
        var_qhi_dn4_slot: &mut f64,
        var_qhi_dn5_slot: &mut f64,
        var_qhi_dn6_slot: &mut f64,
        var_qhi_dn7_slot: &mut f64,
        var_qhi_dn8_slot: &mut f64,
        var_qhi_dn9_slot: &mut f64,
        var_qhi_rdb0_slot: &mut f64,
        var_qhi_rdb1_slot: &mut f64,
        var_qhi_rdb2_slot: &mut f64,
        var_qhi_rdb3_slot: &mut f64,
        var_qhi_rdb4_slot: &mut f64,
        var_qhi_rdb5_slot: &mut f64,
        var_qhi_rdb6_slot: &mut f64,
        var_qhi_rdb7_slot: &mut f64,
        var_qhi_rdn0_slot: &mut f64,
        var_qhi_rdn1_slot: &mut f64,
        var_qhi_rdn2_slot: &mut f64,
        var_qhi_rdn3_slot: &mut f64,
        var_qhi_rdn4_slot: &mut f64,
        var_qhi_rdn5_slot: &mut f64,
        var_qhi_rdn6_slot: &mut f64,
        var_qhi_rdn7_slot: &mut f64,
        var_qhi_rdn8_slot: &mut f64,
        var_qhi_rdn9_slot: &mut f64,
        var_qhi_rv_slot: &mut f64,
        var_qjs_slot: &mut f64,
        var_qjs_db0_slot: &mut f64,
        var_qjs_db1_slot: &mut f64,
        var_qjs_db2_slot: &mut f64,
        var_qjs_db3_slot: &mut f64,
        var_qjs_db4_slot: &mut f64,
        var_qjs_db5_slot: &mut f64,
        var_qjs_db6_slot: &mut f64,
        var_qjs_db7_slot: &mut f64,
        var_qjs_dn0_slot: &mut f64,
        var_qjs_dn1_slot: &mut f64,
        var_qjs_dn2_slot: &mut f64,
        var_qjs_dn3_slot: &mut f64,
        var_qjs_dn4_slot: &mut f64,
        var_qjs_dn5_slot: &mut f64,
        var_qjs_dn6_slot: &mut f64,
        var_qjs_dn7_slot: &mut f64,
        var_qjs_dn8_slot: &mut f64,
        var_qjs_dn9_slot: &mut f64,
        var_qjs_rdb0_slot: &mut f64,
        var_qjs_rdb1_slot: &mut f64,
        var_qjs_rdb2_slot: &mut f64,
        var_qjs_rdb3_slot: &mut f64,
        var_qjs_rdb4_slot: &mut f64,
        var_qjs_rdb5_slot: &mut f64,
        var_qjs_rdb6_slot: &mut f64,
        var_qjs_rdb7_slot: &mut f64,
        var_qjs_rdn0_slot: &mut f64,
        var_qjs_rdn1_slot: &mut f64,
        var_qjs_rdn2_slot: &mut f64,
        var_qjs_rdn3_slot: &mut f64,
        var_qjs_rdn4_slot: &mut f64,
        var_qjs_rdn5_slot: &mut f64,
        var_qjs_rdn6_slot: &mut f64,
        var_qjs_rdn7_slot: &mut f64,
        var_qjs_rdn8_slot: &mut f64,
        var_qjs_rdn9_slot: &mut f64,
        var_qjs_rv_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo_db0_slot: &mut f64,
        var_qlo_db1_slot: &mut f64,
        var_qlo_db2_slot: &mut f64,
        var_qlo_db3_slot: &mut f64,
        var_qlo_db4_slot: &mut f64,
        var_qlo_db5_slot: &mut f64,
        var_qlo_db6_slot: &mut f64,
        var_qlo_db7_slot: &mut f64,
        var_qlo_dn0_slot: &mut f64,
        var_qlo_dn1_slot: &mut f64,
        var_qlo_dn2_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_qlo_dn5_slot: &mut f64,
        var_qlo_dn6_slot: &mut f64,
        var_qlo_dn7_slot: &mut f64,
        var_qlo_dn8_slot: &mut f64,
        var_qlo_dn9_slot: &mut f64,
        var_qlo_rdb0_slot: &mut f64,
        var_qlo_rdb1_slot: &mut f64,
        var_qlo_rdb2_slot: &mut f64,
        var_qlo_rdb3_slot: &mut f64,
        var_qlo_rdb4_slot: &mut f64,
        var_qlo_rdb5_slot: &mut f64,
        var_qlo_rdb6_slot: &mut f64,
        var_qlo_rdb7_slot: &mut f64,
        var_qlo_rdn0_slot: &mut f64,
        var_qlo_rdn1_slot: &mut f64,
        var_qlo_rdn2_slot: &mut f64,
        var_qlo_rdn3_slot: &mut f64,
        var_qlo_rdn4_slot: &mut f64,
        var_qlo_rdn5_slot: &mut f64,
        var_qlo_rdn6_slot: &mut f64,
        var_qlo_rdn7_slot: &mut f64,
        var_qlo_rdn8_slot: &mut f64,
        var_qlo_rdn9_slot: &mut f64,
        var_qlo_rv_slot: &mut f64,
    ) {
        let mut var_dv0: f64 = *var_dv0_slot;
        let mut var_dv0_db0: f64 = *var_dv0_db0_slot;
        let mut var_dv0_db1: f64 = *var_dv0_db1_slot;
        let mut var_dv0_db2: f64 = *var_dv0_db2_slot;
        let mut var_dv0_db3: f64 = *var_dv0_db3_slot;
        let mut var_dv0_db4: f64 = *var_dv0_db4_slot;
        let mut var_dv0_db5: f64 = *var_dv0_db5_slot;
        let mut var_dv0_db6: f64 = *var_dv0_db6_slot;
        let mut var_dv0_db7: f64 = *var_dv0_db7_slot;
        let mut var_dv0_dn0: f64 = *var_dv0_dn0_slot;
        let mut var_dv0_dn1: f64 = *var_dv0_dn1_slot;
        let mut var_dv0_dn2: f64 = *var_dv0_dn2_slot;
        let mut var_dv0_dn3: f64 = *var_dv0_dn3_slot;
        let mut var_dv0_dn4: f64 = *var_dv0_dn4_slot;
        let mut var_dv0_dn5: f64 = *var_dv0_dn5_slot;
        let mut var_dv0_dn6: f64 = *var_dv0_dn6_slot;
        let mut var_dv0_dn7: f64 = *var_dv0_dn7_slot;
        let mut var_dv0_dn8: f64 = *var_dv0_dn8_slot;
        let mut var_dv0_dn9: f64 = *var_dv0_dn9_slot;
        let mut var_dv0_rdb0: f64 = *var_dv0_rdb0_slot;
        let mut var_dv0_rdb1: f64 = *var_dv0_rdb1_slot;
        let mut var_dv0_rdb2: f64 = *var_dv0_rdb2_slot;
        let mut var_dv0_rdb3: f64 = *var_dv0_rdb3_slot;
        let mut var_dv0_rdb4: f64 = *var_dv0_rdb4_slot;
        let mut var_dv0_rdb5: f64 = *var_dv0_rdb5_slot;
        let mut var_dv0_rdb6: f64 = *var_dv0_rdb6_slot;
        let mut var_dv0_rdb7: f64 = *var_dv0_rdb7_slot;
        let mut var_dv0_rdn0: f64 = *var_dv0_rdn0_slot;
        let mut var_dv0_rdn1: f64 = *var_dv0_rdn1_slot;
        let mut var_dv0_rdn2: f64 = *var_dv0_rdn2_slot;
        let mut var_dv0_rdn3: f64 = *var_dv0_rdn3_slot;
        let mut var_dv0_rdn4: f64 = *var_dv0_rdn4_slot;
        let mut var_dv0_rdn5: f64 = *var_dv0_rdn5_slot;
        let mut var_dv0_rdn6: f64 = *var_dv0_rdn6_slot;
        let mut var_dv0_rdn7: f64 = *var_dv0_rdn7_slot;
        let mut var_dv0_rdn8: f64 = *var_dv0_rdn8_slot;
        let mut var_dv0_rdn9: f64 = *var_dv0_rdn9_slot;
        let mut var_dv0_rv: f64 = *var_dv0_rv_slot;
        let mut var_dvh: f64 = *var_dvh_slot;
        let mut var_dvh_db0: f64 = *var_dvh_db0_slot;
        let mut var_dvh_db1: f64 = *var_dvh_db1_slot;
        let mut var_dvh_db2: f64 = *var_dvh_db2_slot;
        let mut var_dvh_db3: f64 = *var_dvh_db3_slot;
        let mut var_dvh_db4: f64 = *var_dvh_db4_slot;
        let mut var_dvh_db5: f64 = *var_dvh_db5_slot;
        let mut var_dvh_db6: f64 = *var_dvh_db6_slot;
        let mut var_dvh_db7: f64 = *var_dvh_db7_slot;
        let mut var_dvh_dn0: f64 = *var_dvh_dn0_slot;
        let mut var_dvh_dn1: f64 = *var_dvh_dn1_slot;
        let mut var_dvh_dn2: f64 = *var_dvh_dn2_slot;
        let mut var_dvh_dn3: f64 = *var_dvh_dn3_slot;
        let mut var_dvh_dn4: f64 = *var_dvh_dn4_slot;
        let mut var_dvh_dn5: f64 = *var_dvh_dn5_slot;
        let mut var_dvh_dn6: f64 = *var_dvh_dn6_slot;
        let mut var_dvh_dn7: f64 = *var_dvh_dn7_slot;
        let mut var_dvh_dn8: f64 = *var_dvh_dn8_slot;
        let mut var_dvh_dn9: f64 = *var_dvh_dn9_slot;
        let mut var_dvh_rdb0: f64 = *var_dvh_rdb0_slot;
        let mut var_dvh_rdb1: f64 = *var_dvh_rdb1_slot;
        let mut var_dvh_rdb2: f64 = *var_dvh_rdb2_slot;
        let mut var_dvh_rdb3: f64 = *var_dvh_rdb3_slot;
        let mut var_dvh_rdb4: f64 = *var_dvh_rdb4_slot;
        let mut var_dvh_rdb5: f64 = *var_dvh_rdb5_slot;
        let mut var_dvh_rdb6: f64 = *var_dvh_rdb6_slot;
        let mut var_dvh_rdb7: f64 = *var_dvh_rdb7_slot;
        let mut var_dvh_rdn0: f64 = *var_dvh_rdn0_slot;
        let mut var_dvh_rdn1: f64 = *var_dvh_rdn1_slot;
        let mut var_dvh_rdn2: f64 = *var_dvh_rdn2_slot;
        let mut var_dvh_rdn3: f64 = *var_dvh_rdn3_slot;
        let mut var_dvh_rdn4: f64 = *var_dvh_rdn4_slot;
        let mut var_dvh_rdn5: f64 = *var_dvh_rdn5_slot;
        let mut var_dvh_rdn6: f64 = *var_dvh_rdn6_slot;
        let mut var_dvh_rdn7: f64 = *var_dvh_rdn7_slot;
        let mut var_dvh_rdn8: f64 = *var_dvh_rdn8_slot;
        let mut var_dvh_rdn9: f64 = *var_dvh_rdn9_slot;
        let mut var_dvh_rv: f64 = *var_dvh_rv_slot;
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard13_db0: f64 = *var_guard13_db0_slot;
        let mut var_guard13_db1: f64 = *var_guard13_db1_slot;
        let mut var_guard13_db2: f64 = *var_guard13_db2_slot;
        let mut var_guard13_db3: f64 = *var_guard13_db3_slot;
        let mut var_guard13_db4: f64 = *var_guard13_db4_slot;
        let mut var_guard13_db5: f64 = *var_guard13_db5_slot;
        let mut var_guard13_db6: f64 = *var_guard13_db6_slot;
        let mut var_guard13_db7: f64 = *var_guard13_db7_slot;
        let mut var_guard13_dn0: f64 = *var_guard13_dn0_slot;
        let mut var_guard13_dn1: f64 = *var_guard13_dn1_slot;
        let mut var_guard13_dn2: f64 = *var_guard13_dn2_slot;
        let mut var_guard13_dn3: f64 = *var_guard13_dn3_slot;
        let mut var_guard13_dn4: f64 = *var_guard13_dn4_slot;
        let mut var_guard13_dn5: f64 = *var_guard13_dn5_slot;
        let mut var_guard13_dn6: f64 = *var_guard13_dn6_slot;
        let mut var_guard13_dn7: f64 = *var_guard13_dn7_slot;
        let mut var_guard13_dn8: f64 = *var_guard13_dn8_slot;
        let mut var_guard13_dn9: f64 = *var_guard13_dn9_slot;
        let mut var_guard13_rdb0: f64 = *var_guard13_rdb0_slot;
        let mut var_guard13_rdb1: f64 = *var_guard13_rdb1_slot;
        let mut var_guard13_rdb2: f64 = *var_guard13_rdb2_slot;
        let mut var_guard13_rdb3: f64 = *var_guard13_rdb3_slot;
        let mut var_guard13_rdb4: f64 = *var_guard13_rdb4_slot;
        let mut var_guard13_rdb5: f64 = *var_guard13_rdb5_slot;
        let mut var_guard13_rdb6: f64 = *var_guard13_rdb6_slot;
        let mut var_guard13_rdb7: f64 = *var_guard13_rdb7_slot;
        let mut var_guard13_rdn0: f64 = *var_guard13_rdn0_slot;
        let mut var_guard13_rdn1: f64 = *var_guard13_rdn1_slot;
        let mut var_guard13_rdn2: f64 = *var_guard13_rdn2_slot;
        let mut var_guard13_rdn3: f64 = *var_guard13_rdn3_slot;
        let mut var_guard13_rdn4: f64 = *var_guard13_rdn4_slot;
        let mut var_guard13_rdn5: f64 = *var_guard13_rdn5_slot;
        let mut var_guard13_rdn6: f64 = *var_guard13_rdn6_slot;
        let mut var_guard13_rdn7: f64 = *var_guard13_rdn7_slot;
        let mut var_guard13_rdn8: f64 = *var_guard13_rdn8_slot;
        let mut var_guard13_rdn9: f64 = *var_guard13_rdn9_slot;
        let mut var_guard13_rv: f64 = *var_guard13_rv_slot;
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_guard15_db0: f64 = *var_guard15_db0_slot;
        let mut var_guard15_db1: f64 = *var_guard15_db1_slot;
        let mut var_guard15_db2: f64 = *var_guard15_db2_slot;
        let mut var_guard15_db3: f64 = *var_guard15_db3_slot;
        let mut var_guard15_db4: f64 = *var_guard15_db4_slot;
        let mut var_guard15_db5: f64 = *var_guard15_db5_slot;
        let mut var_guard15_db6: f64 = *var_guard15_db6_slot;
        let mut var_guard15_db7: f64 = *var_guard15_db7_slot;
        let mut var_guard15_dn0: f64 = *var_guard15_dn0_slot;
        let mut var_guard15_dn1: f64 = *var_guard15_dn1_slot;
        let mut var_guard15_dn2: f64 = *var_guard15_dn2_slot;
        let mut var_guard15_dn3: f64 = *var_guard15_dn3_slot;
        let mut var_guard15_dn4: f64 = *var_guard15_dn4_slot;
        let mut var_guard15_dn5: f64 = *var_guard15_dn5_slot;
        let mut var_guard15_dn6: f64 = *var_guard15_dn6_slot;
        let mut var_guard15_dn7: f64 = *var_guard15_dn7_slot;
        let mut var_guard15_dn8: f64 = *var_guard15_dn8_slot;
        let mut var_guard15_dn9: f64 = *var_guard15_dn9_slot;
        let mut var_guard15_rdb0: f64 = *var_guard15_rdb0_slot;
        let mut var_guard15_rdb1: f64 = *var_guard15_rdb1_slot;
        let mut var_guard15_rdb2: f64 = *var_guard15_rdb2_slot;
        let mut var_guard15_rdb3: f64 = *var_guard15_rdb3_slot;
        let mut var_guard15_rdb4: f64 = *var_guard15_rdb4_slot;
        let mut var_guard15_rdb5: f64 = *var_guard15_rdb5_slot;
        let mut var_guard15_rdb6: f64 = *var_guard15_rdb6_slot;
        let mut var_guard15_rdb7: f64 = *var_guard15_rdb7_slot;
        let mut var_guard15_rdn0: f64 = *var_guard15_rdn0_slot;
        let mut var_guard15_rdn1: f64 = *var_guard15_rdn1_slot;
        let mut var_guard15_rdn2: f64 = *var_guard15_rdn2_slot;
        let mut var_guard15_rdn3: f64 = *var_guard15_rdn3_slot;
        let mut var_guard15_rdn4: f64 = *var_guard15_rdn4_slot;
        let mut var_guard15_rdn5: f64 = *var_guard15_rdn5_slot;
        let mut var_guard15_rdn6: f64 = *var_guard15_rdn6_slot;
        let mut var_guard15_rdn7: f64 = *var_guard15_rdn7_slot;
        let mut var_guard15_rdn8: f64 = *var_guard15_rdn8_slot;
        let mut var_guard15_rdn9: f64 = *var_guard15_rdn9_slot;
        let mut var_guard15_rv: f64 = *var_guard15_rv_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard16_db0: f64 = *var_guard16_db0_slot;
        let mut var_guard16_db1: f64 = *var_guard16_db1_slot;
        let mut var_guard16_db2: f64 = *var_guard16_db2_slot;
        let mut var_guard16_db3: f64 = *var_guard16_db3_slot;
        let mut var_guard16_db4: f64 = *var_guard16_db4_slot;
        let mut var_guard16_db5: f64 = *var_guard16_db5_slot;
        let mut var_guard16_db6: f64 = *var_guard16_db6_slot;
        let mut var_guard16_db7: f64 = *var_guard16_db7_slot;
        let mut var_guard16_dn0: f64 = *var_guard16_dn0_slot;
        let mut var_guard16_dn1: f64 = *var_guard16_dn1_slot;
        let mut var_guard16_dn2: f64 = *var_guard16_dn2_slot;
        let mut var_guard16_dn3: f64 = *var_guard16_dn3_slot;
        let mut var_guard16_dn4: f64 = *var_guard16_dn4_slot;
        let mut var_guard16_dn5: f64 = *var_guard16_dn5_slot;
        let mut var_guard16_dn6: f64 = *var_guard16_dn6_slot;
        let mut var_guard16_dn7: f64 = *var_guard16_dn7_slot;
        let mut var_guard16_dn8: f64 = *var_guard16_dn8_slot;
        let mut var_guard16_dn9: f64 = *var_guard16_dn9_slot;
        let mut var_guard16_rdb0: f64 = *var_guard16_rdb0_slot;
        let mut var_guard16_rdb1: f64 = *var_guard16_rdb1_slot;
        let mut var_guard16_rdb2: f64 = *var_guard16_rdb2_slot;
        let mut var_guard16_rdb3: f64 = *var_guard16_rdb3_slot;
        let mut var_guard16_rdb4: f64 = *var_guard16_rdb4_slot;
        let mut var_guard16_rdb5: f64 = *var_guard16_rdb5_slot;
        let mut var_guard16_rdb6: f64 = *var_guard16_rdb6_slot;
        let mut var_guard16_rdb7: f64 = *var_guard16_rdb7_slot;
        let mut var_guard16_rdn0: f64 = *var_guard16_rdn0_slot;
        let mut var_guard16_rdn1: f64 = *var_guard16_rdn1_slot;
        let mut var_guard16_rdn2: f64 = *var_guard16_rdn2_slot;
        let mut var_guard16_rdn3: f64 = *var_guard16_rdn3_slot;
        let mut var_guard16_rdn4: f64 = *var_guard16_rdn4_slot;
        let mut var_guard16_rdn5: f64 = *var_guard16_rdn5_slot;
        let mut var_guard16_rdn6: f64 = *var_guard16_rdn6_slot;
        let mut var_guard16_rdn7: f64 = *var_guard16_rdn7_slot;
        let mut var_guard16_rdn8: f64 = *var_guard16_rdn8_slot;
        let mut var_guard16_rdn9: f64 = *var_guard16_rdn9_slot;
        let mut var_guard16_rv: f64 = *var_guard16_rv_slot;
        let mut var_pwq: f64 = *var_pwq_slot;
        let mut var_pwq_db0: f64 = *var_pwq_db0_slot;
        let mut var_pwq_db1: f64 = *var_pwq_db1_slot;
        let mut var_pwq_db2: f64 = *var_pwq_db2_slot;
        let mut var_pwq_db3: f64 = *var_pwq_db3_slot;
        let mut var_pwq_db4: f64 = *var_pwq_db4_slot;
        let mut var_pwq_db5: f64 = *var_pwq_db5_slot;
        let mut var_pwq_db6: f64 = *var_pwq_db6_slot;
        let mut var_pwq_db7: f64 = *var_pwq_db7_slot;
        let mut var_pwq_dn0: f64 = *var_pwq_dn0_slot;
        let mut var_pwq_dn1: f64 = *var_pwq_dn1_slot;
        let mut var_pwq_dn2: f64 = *var_pwq_dn2_slot;
        let mut var_pwq_dn3: f64 = *var_pwq_dn3_slot;
        let mut var_pwq_dn4: f64 = *var_pwq_dn4_slot;
        let mut var_pwq_dn5: f64 = *var_pwq_dn5_slot;
        let mut var_pwq_dn6: f64 = *var_pwq_dn6_slot;
        let mut var_pwq_dn7: f64 = *var_pwq_dn7_slot;
        let mut var_pwq_dn8: f64 = *var_pwq_dn8_slot;
        let mut var_pwq_dn9: f64 = *var_pwq_dn9_slot;
        let mut var_pwq_rdb0: f64 = *var_pwq_rdb0_slot;
        let mut var_pwq_rdb1: f64 = *var_pwq_rdb1_slot;
        let mut var_pwq_rdb2: f64 = *var_pwq_rdb2_slot;
        let mut var_pwq_rdb3: f64 = *var_pwq_rdb3_slot;
        let mut var_pwq_rdb4: f64 = *var_pwq_rdb4_slot;
        let mut var_pwq_rdb5: f64 = *var_pwq_rdb5_slot;
        let mut var_pwq_rdb6: f64 = *var_pwq_rdb6_slot;
        let mut var_pwq_rdb7: f64 = *var_pwq_rdb7_slot;
        let mut var_pwq_rdn0: f64 = *var_pwq_rdn0_slot;
        let mut var_pwq_rdn1: f64 = *var_pwq_rdn1_slot;
        let mut var_pwq_rdn2: f64 = *var_pwq_rdn2_slot;
        let mut var_pwq_rdn3: f64 = *var_pwq_rdn3_slot;
        let mut var_pwq_rdn4: f64 = *var_pwq_rdn4_slot;
        let mut var_pwq_rdn5: f64 = *var_pwq_rdn5_slot;
        let mut var_pwq_rdn6: f64 = *var_pwq_rdn6_slot;
        let mut var_pwq_rdn7: f64 = *var_pwq_rdn7_slot;
        let mut var_pwq_rdn8: f64 = *var_pwq_rdn8_slot;
        let mut var_pwq_rdn9: f64 = *var_pwq_rdn9_slot;
        let mut var_pwq_rv: f64 = *var_pwq_rv_slot;
        let mut var_qhi: f64 = *var_qhi_slot;
        let mut var_qhi_db0: f64 = *var_qhi_db0_slot;
        let mut var_qhi_db1: f64 = *var_qhi_db1_slot;
        let mut var_qhi_db2: f64 = *var_qhi_db2_slot;
        let mut var_qhi_db3: f64 = *var_qhi_db3_slot;
        let mut var_qhi_db4: f64 = *var_qhi_db4_slot;
        let mut var_qhi_db5: f64 = *var_qhi_db5_slot;
        let mut var_qhi_db6: f64 = *var_qhi_db6_slot;
        let mut var_qhi_db7: f64 = *var_qhi_db7_slot;
        let mut var_qhi_dn0: f64 = *var_qhi_dn0_slot;
        let mut var_qhi_dn1: f64 = *var_qhi_dn1_slot;
        let mut var_qhi_dn2: f64 = *var_qhi_dn2_slot;
        let mut var_qhi_dn3: f64 = *var_qhi_dn3_slot;
        let mut var_qhi_dn4: f64 = *var_qhi_dn4_slot;
        let mut var_qhi_dn5: f64 = *var_qhi_dn5_slot;
        let mut var_qhi_dn6: f64 = *var_qhi_dn6_slot;
        let mut var_qhi_dn7: f64 = *var_qhi_dn7_slot;
        let mut var_qhi_dn8: f64 = *var_qhi_dn8_slot;
        let mut var_qhi_dn9: f64 = *var_qhi_dn9_slot;
        let mut var_qhi_rdb0: f64 = *var_qhi_rdb0_slot;
        let mut var_qhi_rdb1: f64 = *var_qhi_rdb1_slot;
        let mut var_qhi_rdb2: f64 = *var_qhi_rdb2_slot;
        let mut var_qhi_rdb3: f64 = *var_qhi_rdb3_slot;
        let mut var_qhi_rdb4: f64 = *var_qhi_rdb4_slot;
        let mut var_qhi_rdb5: f64 = *var_qhi_rdb5_slot;
        let mut var_qhi_rdb6: f64 = *var_qhi_rdb6_slot;
        let mut var_qhi_rdb7: f64 = *var_qhi_rdb7_slot;
        let mut var_qhi_rdn0: f64 = *var_qhi_rdn0_slot;
        let mut var_qhi_rdn1: f64 = *var_qhi_rdn1_slot;
        let mut var_qhi_rdn2: f64 = *var_qhi_rdn2_slot;
        let mut var_qhi_rdn3: f64 = *var_qhi_rdn3_slot;
        let mut var_qhi_rdn4: f64 = *var_qhi_rdn4_slot;
        let mut var_qhi_rdn5: f64 = *var_qhi_rdn5_slot;
        let mut var_qhi_rdn6: f64 = *var_qhi_rdn6_slot;
        let mut var_qhi_rdn7: f64 = *var_qhi_rdn7_slot;
        let mut var_qhi_rdn8: f64 = *var_qhi_rdn8_slot;
        let mut var_qhi_rdn9: f64 = *var_qhi_rdn9_slot;
        let mut var_qhi_rv: f64 = *var_qhi_rv_slot;
        let mut var_qjs: f64 = *var_qjs_slot;
        let mut var_qjs_db0: f64 = *var_qjs_db0_slot;
        let mut var_qjs_db1: f64 = *var_qjs_db1_slot;
        let mut var_qjs_db2: f64 = *var_qjs_db2_slot;
        let mut var_qjs_db3: f64 = *var_qjs_db3_slot;
        let mut var_qjs_db4: f64 = *var_qjs_db4_slot;
        let mut var_qjs_db5: f64 = *var_qjs_db5_slot;
        let mut var_qjs_db6: f64 = *var_qjs_db6_slot;
        let mut var_qjs_db7: f64 = *var_qjs_db7_slot;
        let mut var_qjs_dn0: f64 = *var_qjs_dn0_slot;
        let mut var_qjs_dn1: f64 = *var_qjs_dn1_slot;
        let mut var_qjs_dn2: f64 = *var_qjs_dn2_slot;
        let mut var_qjs_dn3: f64 = *var_qjs_dn3_slot;
        let mut var_qjs_dn4: f64 = *var_qjs_dn4_slot;
        let mut var_qjs_dn5: f64 = *var_qjs_dn5_slot;
        let mut var_qjs_dn6: f64 = *var_qjs_dn6_slot;
        let mut var_qjs_dn7: f64 = *var_qjs_dn7_slot;
        let mut var_qjs_dn8: f64 = *var_qjs_dn8_slot;
        let mut var_qjs_dn9: f64 = *var_qjs_dn9_slot;
        let mut var_qjs_rdb0: f64 = *var_qjs_rdb0_slot;
        let mut var_qjs_rdb1: f64 = *var_qjs_rdb1_slot;
        let mut var_qjs_rdb2: f64 = *var_qjs_rdb2_slot;
        let mut var_qjs_rdb3: f64 = *var_qjs_rdb3_slot;
        let mut var_qjs_rdb4: f64 = *var_qjs_rdb4_slot;
        let mut var_qjs_rdb5: f64 = *var_qjs_rdb5_slot;
        let mut var_qjs_rdb6: f64 = *var_qjs_rdb6_slot;
        let mut var_qjs_rdb7: f64 = *var_qjs_rdb7_slot;
        let mut var_qjs_rdn0: f64 = *var_qjs_rdn0_slot;
        let mut var_qjs_rdn1: f64 = *var_qjs_rdn1_slot;
        let mut var_qjs_rdn2: f64 = *var_qjs_rdn2_slot;
        let mut var_qjs_rdn3: f64 = *var_qjs_rdn3_slot;
        let mut var_qjs_rdn4: f64 = *var_qjs_rdn4_slot;
        let mut var_qjs_rdn5: f64 = *var_qjs_rdn5_slot;
        let mut var_qjs_rdn6: f64 = *var_qjs_rdn6_slot;
        let mut var_qjs_rdn7: f64 = *var_qjs_rdn7_slot;
        let mut var_qjs_rdn8: f64 = *var_qjs_rdn8_slot;
        let mut var_qjs_rdn9: f64 = *var_qjs_rdn9_slot;
        let mut var_qjs_rv: f64 = *var_qjs_rv_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo_db0: f64 = *var_qlo_db0_slot;
        let mut var_qlo_db1: f64 = *var_qlo_db1_slot;
        let mut var_qlo_db2: f64 = *var_qlo_db2_slot;
        let mut var_qlo_db3: f64 = *var_qlo_db3_slot;
        let mut var_qlo_db4: f64 = *var_qlo_db4_slot;
        let mut var_qlo_db5: f64 = *var_qlo_db5_slot;
        let mut var_qlo_db6: f64 = *var_qlo_db6_slot;
        let mut var_qlo_db7: f64 = *var_qlo_db7_slot;
        let mut var_qlo_dn0: f64 = *var_qlo_dn0_slot;
        let mut var_qlo_dn1: f64 = *var_qlo_dn1_slot;
        let mut var_qlo_dn2: f64 = *var_qlo_dn2_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_qlo_dn5: f64 = *var_qlo_dn5_slot;
        let mut var_qlo_dn6: f64 = *var_qlo_dn6_slot;
        let mut var_qlo_dn7: f64 = *var_qlo_dn7_slot;
        let mut var_qlo_dn8: f64 = *var_qlo_dn8_slot;
        let mut var_qlo_dn9: f64 = *var_qlo_dn9_slot;
        let mut var_qlo_rdb0: f64 = *var_qlo_rdb0_slot;
        let mut var_qlo_rdb1: f64 = *var_qlo_rdb1_slot;
        let mut var_qlo_rdb2: f64 = *var_qlo_rdb2_slot;
        let mut var_qlo_rdb3: f64 = *var_qlo_rdb3_slot;
        let mut var_qlo_rdb4: f64 = *var_qlo_rdb4_slot;
        let mut var_qlo_rdb5: f64 = *var_qlo_rdb5_slot;
        let mut var_qlo_rdb6: f64 = *var_qlo_rdb6_slot;
        let mut var_qlo_rdb7: f64 = *var_qlo_rdb7_slot;
        let mut var_qlo_rdn0: f64 = *var_qlo_rdn0_slot;
        let mut var_qlo_rdn1: f64 = *var_qlo_rdn1_slot;
        let mut var_qlo_rdn2: f64 = *var_qlo_rdn2_slot;
        let mut var_qlo_rdn3: f64 = *var_qlo_rdn3_slot;
        let mut var_qlo_rdn4: f64 = *var_qlo_rdn4_slot;
        let mut var_qlo_rdn5: f64 = *var_qlo_rdn5_slot;
        let mut var_qlo_rdn6: f64 = *var_qlo_rdn6_slot;
        let mut var_qlo_rdn7: f64 = *var_qlo_rdn7_slot;
        let mut var_qlo_rdn8: f64 = *var_qlo_rdn8_slot;
        let mut var_qlo_rdn9: f64 = *var_qlo_rdn9_slot;
        let mut var_qlo_rv: f64 = *var_qlo_rv_slot;

        let assign1500_e1810: f64 = if p.p32 == 1.0 { 1.0 } else { 0.0 };
        var_guard13 = assign1500_e1810;
        var_guard13_dn0 = 0.0;
        var_guard13_dn1 = 0.0;
        var_guard13_dn2 = 0.0;
        var_guard13_dn3 = 0.0;
        var_guard13_dn4 = 0.0;
        var_guard13_dn5 = 0.0;
        var_guard13_dn6 = 0.0;
        var_guard13_dn7 = 0.0;
        var_guard13_dn8 = 0.0;
        var_guard13_dn9 = 0.0;
        var_guard13_db0 = 0.0;
        var_guard13_db1 = 0.0;
        var_guard13_db2 = 0.0;
        var_guard13_db3 = 0.0;
        var_guard13_db4 = 0.0;
        var_guard13_db5 = 0.0;
        var_guard13_db6 = 0.0;
        var_guard13_db7 = 0.0;
        var_guard13_rv = 0.0;
        var_guard13_rdn0 = 0.0;
        var_guard13_rdn1 = 0.0;
        var_guard13_rdn2 = 0.0;
        var_guard13_rdn3 = 0.0;
        var_guard13_rdn4 = 0.0;
        var_guard13_rdn5 = 0.0;
        var_guard13_rdn6 = 0.0;
        var_guard13_rdn7 = 0.0;
        var_guard13_rdn8 = 0.0;
        var_guard13_rdn9 = 0.0;
        var_guard13_rdb0 = 0.0;
        var_guard13_rdb1 = 0.0;
        var_guard13_rdb2 = 0.0;
        var_guard13_rdb3 = 0.0;
        var_guard13_rdb4 = 0.0;
        var_guard13_rdb5 = 0.0;
        var_guard13_rdb6 = 0.0;
        var_guard13_rdb7 = 0.0;

        let assign1570_e1852: f64 = if var_veci <= 0.0 { 1.0 } else { 0.0 };
        var_guard15 = assign1570_e1852;
        var_guard15_dn0 = 0.0;
        var_guard15_dn1 = 0.0;
        var_guard15_dn2 = 0.0;
        var_guard15_dn3 = 0.0;
        var_guard15_dn4 = 0.0;
        var_guard15_dn5 = 0.0;
        var_guard15_dn6 = 0.0;
        var_guard15_dn7 = 0.0;
        var_guard15_dn8 = 0.0;
        var_guard15_dn9 = 0.0;
        var_guard15_db0 = 0.0;
        var_guard15_db1 = 0.0;
        var_guard15_db2 = 0.0;
        var_guard15_db3 = 0.0;
        var_guard15_db4 = 0.0;
        var_guard15_db5 = 0.0;
        var_guard15_db6 = 0.0;
        var_guard15_db7 = 0.0;
        var_guard15_rv = 0.0;
        var_guard15_rdn0 = 0.0;
        var_guard15_rdn1 = 0.0;
        var_guard15_rdn2 = 0.0;
        var_guard15_rdn3 = 0.0;
        var_guard15_rdn4 = 0.0;
        var_guard15_rdn5 = 0.0;
        var_guard15_rdn6 = 0.0;
        var_guard15_rdn7 = 0.0;
        var_guard15_rdn8 = 0.0;
        var_guard15_rdn9 = 0.0;
        var_guard15_rdb0 = 0.0;
        var_guard15_rdb1 = 0.0;
        var_guard15_rdb2 = 0.0;
        var_guard15_rdb3 = 0.0;
        var_guard15_rdb4 = 0.0;
        var_guard15_rdb5 = 0.0;
        var_guard15_rdb6 = 0.0;
        var_guard15_rdb7 = 0.0;

        let (assign1580_e1876, assign1580_e1876_d_n0, assign1580_e1876_d_n1, assign1580_e1876_d_n2, assign1580_e1876_d_n3, assign1580_e1876_d_n4, assign1580_e1876_d_n5, assign1580_e1876_d_n6, assign1580_e1876_d_n7, assign1580_e1876_d_n8, assign1580_e1876_d_n9, assign1580_e1876_d_b0, assign1580_e1876_d_b1, assign1580_e1876_d_b2, assign1580_e1876_d_b3, assign1580_e1876_d_b4, assign1580_e1876_d_b5, assign1580_e1876_d_b6, assign1580_e1876_d_b7,) = {
    if (var_guard15 != 0.0) {
        let assign1580_e1856: f64 = (var_cjs_t * var_vjs_t);
        let assign1580_e1860: f64 = (1.0 - p.p76);
        let assign1580_e1864: f64 = (var_veci / var_vjs_t);
        let assign1580_e1865: f64 = (1.0 - assign1580_e1864);
        let assign1580_e1866: f64 = (assign1580_e1865).ln();
        let assign1580_e1867: f64 = (assign1580_e1860 * assign1580_e1866);
        let assign1580_e1868: f64 = (assign1580_e1867).exp();
        let assign1580_e1869: f64 = (1.0 - assign1580_e1868);
        let assign1580_e1870: f64 = (assign1580_e1856 * assign1580_e1869);
        let assign1580_e1873: f64 = (1.0 - p.p76);
        let assign1580_e1874: f64 = (assign1580_e1870 / assign1580_e1873);
        (assign1580_e1874, (((((var_cjs_t_dn0 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn0)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn0 * var_vjs_t) - (var_veci * var_vjs_t_dn0)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn1 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn1)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn1 * var_vjs_t) - (var_veci * var_vjs_t_dn1)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn2 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn2)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn2 * var_vjs_t) - (var_veci * var_vjs_t_dn2)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn3 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn3)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn3 * var_vjs_t) - (var_veci * var_vjs_t_dn3)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn4 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn4)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn4 * var_vjs_t) - (var_veci * var_vjs_t_dn4)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn5 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn5)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn5 * var_vjs_t) - (var_veci * var_vjs_t_dn5)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn6 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn6)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn6 * var_vjs_t) - (var_veci * var_vjs_t_dn6)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn7 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn7)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn7 * var_vjs_t) - (var_veci * var_vjs_t_dn7)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn8 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn8)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn8 * var_vjs_t) - (var_veci * var_vjs_t_dn8)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_dn9 * var_vjs_t) + (var_cjs_t * var_vjs_t_dn9)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_dn9 * var_vjs_t) - (var_veci * var_vjs_t_dn9)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db0 * var_vjs_t) + (var_cjs_t * var_vjs_t_db0)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db0 * var_vjs_t) - (var_veci * var_vjs_t_db0)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db1 * var_vjs_t) + (var_cjs_t * var_vjs_t_db1)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db1 * var_vjs_t) - (var_veci * var_vjs_t_db1)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db2 * var_vjs_t) + (var_cjs_t * var_vjs_t_db2)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db2 * var_vjs_t) - (var_veci * var_vjs_t_db2)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db3 * var_vjs_t) + (var_cjs_t * var_vjs_t_db3)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db3 * var_vjs_t) - (var_veci * var_vjs_t_db3)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db4 * var_vjs_t) + (var_cjs_t * var_vjs_t_db4)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db4 * var_vjs_t) - (var_veci * var_vjs_t_db4)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db5 * var_vjs_t) + (var_cjs_t * var_vjs_t_db5)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db5 * var_vjs_t) - (var_veci * var_vjs_t_db5)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db6 * var_vjs_t) + (var_cjs_t * var_vjs_t_db6)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db6 * var_vjs_t) - (var_veci * var_vjs_t_db6)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873), (((((var_cjs_t_db7 * var_vjs_t) + (var_cjs_t * var_vjs_t_db7)) * assign1580_e1869) + (assign1580_e1856 * (-(assign1580_e1868 * (assign1580_e1860 * ((-(((var_veci_db7 * var_vjs_t) - (var_veci * var_vjs_t_db7)) / (var_vjs_t * var_vjs_t))) / assign1580_e1865)))))) / assign1580_e1873),)
    } else {
        (var_qjs, var_qjs_dn0, var_qjs_dn1, var_qjs_dn2, var_qjs_dn3, var_qjs_dn4, var_qjs_dn5, var_qjs_dn6, var_qjs_dn7, var_qjs_dn8, var_qjs_dn9, var_qjs_db0, var_qjs_db1, var_qjs_db2, var_qjs_db3, var_qjs_db4, var_qjs_db5, var_qjs_db6, var_qjs_db7,)
    }
};
        var_qjs = assign1580_e1876;
        var_qjs_dn0 = assign1580_e1876_d_n0;
        var_qjs_dn1 = assign1580_e1876_d_n1;
        var_qjs_dn2 = assign1580_e1876_d_n2;
        var_qjs_dn3 = assign1580_e1876_d_n3;
        var_qjs_dn4 = assign1580_e1876_d_n4;
        var_qjs_dn5 = assign1580_e1876_d_n5;
        var_qjs_dn6 = assign1580_e1876_d_n6;
        var_qjs_dn7 = assign1580_e1876_d_n7;
        var_qjs_dn8 = assign1580_e1876_d_n8;
        var_qjs_dn9 = assign1580_e1876_d_n9;
        var_qjs_db0 = assign1580_e1876_d_b0;
        var_qjs_db1 = assign1580_e1876_d_b1;
        var_qjs_db2 = assign1580_e1876_d_b2;
        var_qjs_db3 = assign1580_e1876_d_b3;
        var_qjs_db4 = assign1580_e1876_d_b4;
        var_qjs_db5 = assign1580_e1876_d_b5;
        var_qjs_db6 = assign1580_e1876_d_b6;
        var_qjs_db7 = assign1580_e1876_d_b7;
        var_qjs_rv = 0.0;
        var_qjs_rdn0 = 0.0;
        var_qjs_rdn1 = 0.0;
        var_qjs_rdn2 = 0.0;
        var_qjs_rdn3 = 0.0;
        var_qjs_rdn4 = 0.0;
        var_qjs_rdn5 = 0.0;
        var_qjs_rdn6 = 0.0;
        var_qjs_rdn7 = 0.0;
        var_qjs_rdn8 = 0.0;
        var_qjs_rdn9 = 0.0;
        var_qjs_rdb0 = 0.0;
        var_qjs_rdb1 = 0.0;
        var_qjs_rdb2 = 0.0;
        var_qjs_rdb3 = 0.0;
        var_qjs_rdb4 = 0.0;
        var_qjs_rdb5 = 0.0;
        var_qjs_rdb6 = 0.0;
        var_qjs_rdb7 = 0.0;

        let (assign1590_e1893, assign1590_e1893_d_n0, assign1590_e1893_d_n1, assign1590_e1893_d_n2, assign1590_e1893_d_n3, assign1590_e1893_d_n4, assign1590_e1893_d_n5, assign1590_e1893_d_n6, assign1590_e1893_d_n7, assign1590_e1893_d_n8, assign1590_e1893_d_n9, assign1590_e1893_d_b0, assign1590_e1893_d_b1, assign1590_e1893_d_b2, assign1590_e1893_d_b3, assign1590_e1893_d_b4, assign1590_e1893_d_b5, assign1590_e1893_d_b6, assign1590_e1893_d_b7,) = {
    if (var_guard15 == 0.0) {
        let assign1590_e1881: f64 = (var_cjs_t * var_veci);
        let assign1590_e1885: f64 = (0.5 * p.p76);
        let assign1590_e1887: f64 = (assign1590_e1885 * var_veci);
        let assign1590_e1889: f64 = (assign1590_e1887 / var_vjs_t);
        let assign1590_e1890: f64 = (1.0 + assign1590_e1889);
        let assign1590_e1891: f64 = (assign1590_e1881 * assign1590_e1890);
        (assign1590_e1891, ((((var_cjs_t_dn0 * var_veci) + (var_cjs_t * var_veci_dn0)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn0) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn0)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn1 * var_veci) + (var_cjs_t * var_veci_dn1)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn1) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn1)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn2 * var_veci) + (var_cjs_t * var_veci_dn2)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn2) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn2)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn3 * var_veci) + (var_cjs_t * var_veci_dn3)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn3) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn3)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn4 * var_veci) + (var_cjs_t * var_veci_dn4)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn4) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn4)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn5 * var_veci) + (var_cjs_t * var_veci_dn5)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn5) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn5)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn6 * var_veci) + (var_cjs_t * var_veci_dn6)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn6) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn6)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn7 * var_veci) + (var_cjs_t * var_veci_dn7)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn7) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn7)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn8 * var_veci) + (var_cjs_t * var_veci_dn8)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn8) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn8)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_dn9 * var_veci) + (var_cjs_t * var_veci_dn9)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_dn9) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_dn9)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db0 * var_veci) + (var_cjs_t * var_veci_db0)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db0) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db0)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db1 * var_veci) + (var_cjs_t * var_veci_db1)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db1) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db1)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db2 * var_veci) + (var_cjs_t * var_veci_db2)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db2) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db2)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db3 * var_veci) + (var_cjs_t * var_veci_db3)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db3) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db3)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db4 * var_veci) + (var_cjs_t * var_veci_db4)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db4) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db4)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db5 * var_veci) + (var_cjs_t * var_veci_db5)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db5) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db5)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db6 * var_veci) + (var_cjs_t * var_veci_db6)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db6) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db6)) / (var_vjs_t * var_vjs_t)))), ((((var_cjs_t_db7 * var_veci) + (var_cjs_t * var_veci_db7)) * assign1590_e1890) + (assign1590_e1881 * ((((assign1590_e1885 * var_veci_db7) * var_vjs_t) - (assign1590_e1887 * var_vjs_t_db7)) / (var_vjs_t * var_vjs_t)))),)
    } else {
        (var_qjs, var_qjs_dn0, var_qjs_dn1, var_qjs_dn2, var_qjs_dn3, var_qjs_dn4, var_qjs_dn5, var_qjs_dn6, var_qjs_dn7, var_qjs_dn8, var_qjs_dn9, var_qjs_db0, var_qjs_db1, var_qjs_db2, var_qjs_db3, var_qjs_db4, var_qjs_db5, var_qjs_db6, var_qjs_db7,)
    }
};
        var_qjs = assign1590_e1893;
        var_qjs_dn0 = assign1590_e1893_d_n0;
        var_qjs_dn1 = assign1590_e1893_d_n1;
        var_qjs_dn2 = assign1590_e1893_d_n2;
        var_qjs_dn3 = assign1590_e1893_d_n3;
        var_qjs_dn4 = assign1590_e1893_d_n4;
        var_qjs_dn5 = assign1590_e1893_d_n5;
        var_qjs_dn6 = assign1590_e1893_d_n6;
        var_qjs_dn7 = assign1590_e1893_d_n7;
        var_qjs_dn8 = assign1590_e1893_d_n8;
        var_qjs_dn9 = assign1590_e1893_d_n9;
        var_qjs_db0 = assign1590_e1893_d_b0;
        var_qjs_db1 = assign1590_e1893_d_b1;
        var_qjs_db2 = assign1590_e1893_d_b2;
        var_qjs_db3 = assign1590_e1893_d_b3;
        var_qjs_db4 = assign1590_e1893_d_b4;
        var_qjs_db5 = assign1590_e1893_d_b5;
        var_qjs_db6 = assign1590_e1893_d_b6;
        var_qjs_db7 = assign1590_e1893_d_b7;
        var_qjs_rv = 0.0;
        var_qjs_rdn0 = 0.0;
        var_qjs_rdn1 = 0.0;
        var_qjs_rdn2 = 0.0;
        var_qjs_rdn3 = 0.0;
        var_qjs_rdn4 = 0.0;
        var_qjs_rdn5 = 0.0;
        var_qjs_rdn6 = 0.0;
        var_qjs_rdn7 = 0.0;
        var_qjs_rdn8 = 0.0;
        var_qjs_rdn9 = 0.0;
        var_qjs_rdb0 = 0.0;
        var_qjs_rdb1 = 0.0;
        var_qjs_rdb2 = 0.0;
        var_qjs_rdb3 = 0.0;
        var_qjs_rdb4 = 0.0;
        var_qjs_rdb5 = 0.0;
        var_qjs_rdb6 = 0.0;
        var_qjs_rdb7 = 0.0;

        let assign1600_e1895: f64 = (-var_vje_t);
        let assign1600_e1897: f64 = (assign1600_e1895 * p.p24);
        var_dv0 = assign1600_e1897;
        var_dv0_dn0 = ((-var_vje_t_dn0) * p.p24);
        var_dv0_dn1 = ((-var_vje_t_dn1) * p.p24);
        var_dv0_dn2 = ((-var_vje_t_dn2) * p.p24);
        var_dv0_dn3 = ((-var_vje_t_dn3) * p.p24);
        var_dv0_dn4 = ((-var_vje_t_dn4) * p.p24);
        var_dv0_dn5 = ((-var_vje_t_dn5) * p.p24);
        var_dv0_dn6 = ((-var_vje_t_dn6) * p.p24);
        var_dv0_dn7 = ((-var_vje_t_dn7) * p.p24);
        var_dv0_dn8 = ((-var_vje_t_dn8) * p.p24);
        var_dv0_dn9 = ((-var_vje_t_dn9) * p.p24);
        var_dv0_db0 = ((-var_vje_t_db0) * p.p24);
        var_dv0_db1 = ((-var_vje_t_db1) * p.p24);
        var_dv0_db2 = ((-var_vje_t_db2) * p.p24);
        var_dv0_db3 = ((-var_vje_t_db3) * p.p24);
        var_dv0_db4 = ((-var_vje_t_db4) * p.p24);
        var_dv0_db5 = ((-var_vje_t_db5) * p.p24);
        var_dv0_db6 = ((-var_vje_t_db6) * p.p24);
        var_dv0_db7 = ((-var_vje_t_db7) * p.p24);
        var_dv0_rv = 0.0;
        var_dv0_rdn0 = 0.0;
        var_dv0_rdn1 = 0.0;
        var_dv0_rdn2 = 0.0;
        var_dv0_rdn3 = 0.0;
        var_dv0_rdn4 = 0.0;
        var_dv0_rdn5 = 0.0;
        var_dv0_rdn6 = 0.0;
        var_dv0_rdn7 = 0.0;
        var_dv0_rdn8 = 0.0;
        var_dv0_rdn9 = 0.0;
        var_dv0_rdb0 = 0.0;
        var_dv0_rdb1 = 0.0;
        var_dv0_rdb2 = 0.0;
        var_dv0_rdb3 = 0.0;
        var_dv0_rdb4 = 0.0;
        var_dv0_rdb5 = 0.0;
        var_dv0_rdb6 = 0.0;
        var_dv0_rdb7 = 0.0;

        let assign1610_e1900: f64 = (var_vbiei + var_dv0);
        var_dvh = assign1610_e1900;
        var_dvh_dn0 = (var_vbiei_dn0 + var_dv0_dn0);
        var_dvh_dn1 = (var_vbiei_dn1 + var_dv0_dn1);
        var_dvh_dn2 = (var_vbiei_dn2 + var_dv0_dn2);
        var_dvh_dn3 = (var_vbiei_dn3 + var_dv0_dn3);
        var_dvh_dn4 = (var_vbiei_dn4 + var_dv0_dn4);
        var_dvh_dn5 = (var_vbiei_dn5 + var_dv0_dn5);
        var_dvh_dn6 = (var_vbiei_dn6 + var_dv0_dn6);
        var_dvh_dn7 = (var_vbiei_dn7 + var_dv0_dn7);
        var_dvh_dn8 = (var_vbiei_dn8 + var_dv0_dn8);
        var_dvh_dn9 = (var_vbiei_dn9 + var_dv0_dn9);
        var_dvh_db0 = (var_vbiei_db0 + var_dv0_db0);
        var_dvh_db1 = (var_vbiei_db1 + var_dv0_db1);
        var_dvh_db2 = (var_vbiei_db2 + var_dv0_db2);
        var_dvh_db3 = (var_vbiei_db3 + var_dv0_db3);
        var_dvh_db4 = (var_vbiei_db4 + var_dv0_db4);
        var_dvh_db5 = (var_vbiei_db5 + var_dv0_db5);
        var_dvh_db6 = (var_vbiei_db6 + var_dv0_db6);
        var_dvh_db7 = (var_vbiei_db7 + var_dv0_db7);
        var_dvh_rv = 0.0;
        var_dvh_rdn0 = 0.0;
        var_dvh_rdn1 = 0.0;
        var_dvh_rdn2 = 0.0;
        var_dvh_rdn3 = 0.0;
        var_dvh_rdn4 = 0.0;
        var_dvh_rdn5 = 0.0;
        var_dvh_rdn6 = 0.0;
        var_dvh_rdn7 = 0.0;
        var_dvh_rdn8 = 0.0;
        var_dvh_rdn9 = 0.0;
        var_dvh_rdb0 = 0.0;
        var_dvh_rdb1 = 0.0;
        var_dvh_rdb2 = 0.0;
        var_dvh_rdb3 = 0.0;
        var_dvh_rdb4 = 0.0;
        var_dvh_rdb5 = 0.0;
        var_dvh_rdb6 = 0.0;
        var_dvh_rdb7 = 0.0;

        let assign1620_e1903: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard16 = assign1620_e1903;
        var_guard16_dn0 = 0.0;
        var_guard16_dn1 = 0.0;
        var_guard16_dn2 = 0.0;
        var_guard16_dn3 = 0.0;
        var_guard16_dn4 = 0.0;
        var_guard16_dn5 = 0.0;
        var_guard16_dn6 = 0.0;
        var_guard16_dn7 = 0.0;
        var_guard16_dn8 = 0.0;
        var_guard16_dn9 = 0.0;
        var_guard16_db0 = 0.0;
        var_guard16_db1 = 0.0;
        var_guard16_db2 = 0.0;
        var_guard16_db3 = 0.0;
        var_guard16_db4 = 0.0;
        var_guard16_db5 = 0.0;
        var_guard16_db6 = 0.0;
        var_guard16_db7 = 0.0;
        var_guard16_rv = 0.0;
        var_guard16_rdn0 = 0.0;
        var_guard16_rdn1 = 0.0;
        var_guard16_rdn2 = 0.0;
        var_guard16_rdn3 = 0.0;
        var_guard16_rdn4 = 0.0;
        var_guard16_rdn5 = 0.0;
        var_guard16_rdn6 = 0.0;
        var_guard16_rdn7 = 0.0;
        var_guard16_rdn8 = 0.0;
        var_guard16_rdn9 = 0.0;
        var_guard16_rdb0 = 0.0;
        var_guard16_rdb1 = 0.0;
        var_guard16_rdb2 = 0.0;
        var_guard16_rdb3 = 0.0;
        var_guard16_rdb4 = 0.0;
        var_guard16_rdb5 = 0.0;
        var_guard16_rdb6 = 0.0;
        var_guard16_rdb7 = 0.0;

        let (assign1630_e1916, assign1630_e1916_d_n0, assign1630_e1916_d_n1, assign1630_e1916_d_n2, assign1630_e1916_d_n3, assign1630_e1916_d_n4, assign1630_e1916_d_n5, assign1630_e1916_d_n6, assign1630_e1916_d_n7, assign1630_e1916_d_n8, assign1630_e1916_d_n9, assign1630_e1916_d_b0, assign1630_e1916_d_b1, assign1630_e1916_d_b2, assign1630_e1916_d_b3, assign1630_e1916_d_b4, assign1630_e1916_d_b5, assign1630_e1916_d_b6, assign1630_e1916_d_b7,) = {
    if (var_guard16 != 0.0) {
        let assign1630_e1906: f64 = (-1.0);
        let assign1630_e1908: f64 = (assign1630_e1906 - p.p18);
        let assign1630_e1911: f64 = (1.0 - p.p24);
        let assign1630_e1912: f64 = (assign1630_e1911).ln();
        let assign1630_e1913: f64 = (assign1630_e1908 * assign1630_e1912);
        let assign1630_e1914: f64 = (assign1630_e1913).exp();
        (assign1630_e1914, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pwq, var_pwq_dn0, var_pwq_dn1, var_pwq_dn2, var_pwq_dn3, var_pwq_dn4, var_pwq_dn5, var_pwq_dn6, var_pwq_dn7, var_pwq_dn8, var_pwq_dn9, var_pwq_db0, var_pwq_db1, var_pwq_db2, var_pwq_db3, var_pwq_db4, var_pwq_db5, var_pwq_db6, var_pwq_db7,)
    }
};
        var_pwq = assign1630_e1916;
        var_pwq_dn0 = assign1630_e1916_d_n0;
        var_pwq_dn1 = assign1630_e1916_d_n1;
        var_pwq_dn2 = assign1630_e1916_d_n2;
        var_pwq_dn3 = assign1630_e1916_d_n3;
        var_pwq_dn4 = assign1630_e1916_d_n4;
        var_pwq_dn5 = assign1630_e1916_d_n5;
        var_pwq_dn6 = assign1630_e1916_d_n6;
        var_pwq_dn7 = assign1630_e1916_d_n7;
        var_pwq_dn8 = assign1630_e1916_d_n8;
        var_pwq_dn9 = assign1630_e1916_d_n9;
        var_pwq_db0 = assign1630_e1916_d_b0;
        var_pwq_db1 = assign1630_e1916_d_b1;
        var_pwq_db2 = assign1630_e1916_d_b2;
        var_pwq_db3 = assign1630_e1916_d_b3;
        var_pwq_db4 = assign1630_e1916_d_b4;
        var_pwq_db5 = assign1630_e1916_d_b5;
        var_pwq_db6 = assign1630_e1916_d_b6;
        var_pwq_db7 = assign1630_e1916_d_b7;
        var_pwq_rv = 0.0;
        var_pwq_rdn0 = 0.0;
        var_pwq_rdn1 = 0.0;
        var_pwq_rdn2 = 0.0;
        var_pwq_rdn3 = 0.0;
        var_pwq_rdn4 = 0.0;
        var_pwq_rdn5 = 0.0;
        var_pwq_rdn6 = 0.0;
        var_pwq_rdn7 = 0.0;
        var_pwq_rdn8 = 0.0;
        var_pwq_rdn9 = 0.0;
        var_pwq_rdb0 = 0.0;
        var_pwq_rdb1 = 0.0;
        var_pwq_rdb2 = 0.0;
        var_pwq_rdb3 = 0.0;
        var_pwq_rdb4 = 0.0;
        var_pwq_rdb5 = 0.0;
        var_pwq_rdb6 = 0.0;
        var_pwq_rdb7 = 0.0;

        let (assign1640_e1936, assign1640_e1936_d_n0, assign1640_e1936_d_n1, assign1640_e1936_d_n2, assign1640_e1936_d_n3, assign1640_e1936_d_n4, assign1640_e1936_d_n5, assign1640_e1936_d_n6, assign1640_e1936_d_n7, assign1640_e1936_d_n8, assign1640_e1936_d_n9, assign1640_e1936_d_b0, assign1640_e1936_d_b1, assign1640_e1936_d_b2, assign1640_e1936_d_b3, assign1640_e1936_d_b4, assign1640_e1936_d_b5, assign1640_e1936_d_b6, assign1640_e1936_d_b7,) = {
    if (var_guard16 != 0.0) {
        let assign1640_e1923: f64 = (1.0 - p.p24);
        let assign1640_e1924: f64 = (var_pwq * assign1640_e1923);
        let assign1640_e1927: f64 = (1.0 - p.p24);
        let assign1640_e1928: f64 = (assign1640_e1924 * assign1640_e1927);
        let assign1640_e1929: f64 = (1.0 - assign1640_e1928);
        let assign1640_e1930: f64 = (var_vje_t * assign1640_e1929);
        let assign1640_e1933: f64 = (1.0 - p.p18);
        let assign1640_e1934: f64 = (assign1640_e1930 / assign1640_e1933);
        (assign1640_e1934, (((var_vje_t_dn0 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn0 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn1 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn1 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn2 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn2 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn3 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn3 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn4 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn4 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn5 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn5 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn6 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn6 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn7 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn7 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn8 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn8 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_dn9 * assign1640_e1929) + (var_vje_t * (-((var_pwq_dn9 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db0 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db0 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db1 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db1 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db2 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db2 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db3 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db3 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db4 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db4 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db5 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db5 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db6 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db6 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933), (((var_vje_t_db7 * assign1640_e1929) + (var_vje_t * (-((var_pwq_db7 * assign1640_e1923) * assign1640_e1927)))) / assign1640_e1933),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7,)
    }
};
        var_qlo = assign1640_e1936;
        var_qlo_dn0 = assign1640_e1936_d_n0;
        var_qlo_dn1 = assign1640_e1936_d_n1;
        var_qlo_dn2 = assign1640_e1936_d_n2;
        var_qlo_dn3 = assign1640_e1936_d_n3;
        var_qlo_dn4 = assign1640_e1936_d_n4;
        var_qlo_dn5 = assign1640_e1936_d_n5;
        var_qlo_dn6 = assign1640_e1936_d_n6;
        var_qlo_dn7 = assign1640_e1936_d_n7;
        var_qlo_dn8 = assign1640_e1936_d_n8;
        var_qlo_dn9 = assign1640_e1936_d_n9;
        var_qlo_db0 = assign1640_e1936_d_b0;
        var_qlo_db1 = assign1640_e1936_d_b1;
        var_qlo_db2 = assign1640_e1936_d_b2;
        var_qlo_db3 = assign1640_e1936_d_b3;
        var_qlo_db4 = assign1640_e1936_d_b4;
        var_qlo_db5 = assign1640_e1936_d_b5;
        var_qlo_db6 = assign1640_e1936_d_b6;
        var_qlo_db7 = assign1640_e1936_d_b7;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdn7 = 0.0;
        var_qlo_rdn8 = 0.0;
        var_qlo_rdn9 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;
        var_qlo_rdb7 = 0.0;

        let (assign1650_e1954, assign1650_e1954_d_n0, assign1650_e1954_d_n1, assign1650_e1954_d_n2, assign1650_e1954_d_n3, assign1650_e1954_d_n4, assign1650_e1954_d_n5, assign1650_e1954_d_n6, assign1650_e1954_d_n7, assign1650_e1954_d_n8, assign1650_e1954_d_n9, assign1650_e1954_d_b0, assign1650_e1954_d_b1, assign1650_e1954_d_b2, assign1650_e1954_d_b3, assign1650_e1954_d_b4, assign1650_e1954_d_b5, assign1650_e1954_d_b6, assign1650_e1954_d_b7,) = {
    if (var_guard16 != 0.0) {
        let assign1650_e1941: f64 = (1.0 - p.p24);
        let assign1650_e1944: f64 = (0.5 * p.p18);
        let assign1650_e1946: f64 = (assign1650_e1944 * var_dvh);
        let assign1650_e1948: f64 = (assign1650_e1946 / var_vje_t);
        let assign1650_e1949: f64 = (assign1650_e1941 + assign1650_e1948);
        let assign1650_e1950: f64 = (var_dvh * assign1650_e1949);
        let assign1650_e1952: f64 = (assign1650_e1950 * var_pwq);
        (assign1650_e1952, ((((var_dvh_dn0 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn0) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn0)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn0)), ((((var_dvh_dn1 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn1) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn1)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn1)), ((((var_dvh_dn2 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn2) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn2)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn2)), ((((var_dvh_dn3 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn3) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn3)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn3)), ((((var_dvh_dn4 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn4) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn4)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn4)), ((((var_dvh_dn5 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn5) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn5)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn5)), ((((var_dvh_dn6 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn6) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn6)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn6)), ((((var_dvh_dn7 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn7) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn7)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn7)), ((((var_dvh_dn8 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn8) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn8)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn8)), ((((var_dvh_dn9 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_dn9) * var_vje_t) - (assign1650_e1946 * var_vje_t_dn9)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_dn9)), ((((var_dvh_db0 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db0) * var_vje_t) - (assign1650_e1946 * var_vje_t_db0)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db0)), ((((var_dvh_db1 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db1) * var_vje_t) - (assign1650_e1946 * var_vje_t_db1)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db1)), ((((var_dvh_db2 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db2) * var_vje_t) - (assign1650_e1946 * var_vje_t_db2)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db2)), ((((var_dvh_db3 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db3) * var_vje_t) - (assign1650_e1946 * var_vje_t_db3)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db3)), ((((var_dvh_db4 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db4) * var_vje_t) - (assign1650_e1946 * var_vje_t_db4)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db4)), ((((var_dvh_db5 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db5) * var_vje_t) - (assign1650_e1946 * var_vje_t_db5)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db5)), ((((var_dvh_db6 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db6) * var_vje_t) - (assign1650_e1946 * var_vje_t_db6)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db6)), ((((var_dvh_db7 * assign1650_e1949) + (var_dvh * ((((assign1650_e1944 * var_dvh_db7) * var_vje_t) - (assign1650_e1946 * var_vje_t_db7)) / (var_vje_t * var_vje_t)))) * var_pwq) + (assign1650_e1950 * var_pwq_db7)),)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_dn7, var_qhi_dn8, var_qhi_dn9, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6, var_qhi_db7,)
    }
};
        var_qhi = assign1650_e1954;
        var_qhi_dn0 = assign1650_e1954_d_n0;
        var_qhi_dn1 = assign1650_e1954_d_n1;
        var_qhi_dn2 = assign1650_e1954_d_n2;
        var_qhi_dn3 = assign1650_e1954_d_n3;
        var_qhi_dn4 = assign1650_e1954_d_n4;
        var_qhi_dn5 = assign1650_e1954_d_n5;
        var_qhi_dn6 = assign1650_e1954_d_n6;
        var_qhi_dn7 = assign1650_e1954_d_n7;
        var_qhi_dn8 = assign1650_e1954_d_n8;
        var_qhi_dn9 = assign1650_e1954_d_n9;
        var_qhi_db0 = assign1650_e1954_d_b0;
        var_qhi_db1 = assign1650_e1954_d_b1;
        var_qhi_db2 = assign1650_e1954_d_b2;
        var_qhi_db3 = assign1650_e1954_d_b3;
        var_qhi_db4 = assign1650_e1954_d_b4;
        var_qhi_db5 = assign1650_e1954_d_b5;
        var_qhi_db6 = assign1650_e1954_d_b6;
        var_qhi_db7 = assign1650_e1954_d_b7;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdn7 = 0.0;
        var_qhi_rdn8 = 0.0;
        var_qhi_rdn9 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;
        var_qhi_rdb7 = 0.0;

        *var_dv0_slot = var_dv0;
        *var_dv0_db0_slot = var_dv0_db0;
        *var_dv0_db1_slot = var_dv0_db1;
        *var_dv0_db2_slot = var_dv0_db2;
        *var_dv0_db3_slot = var_dv0_db3;
        *var_dv0_db4_slot = var_dv0_db4;
        *var_dv0_db5_slot = var_dv0_db5;
        *var_dv0_db6_slot = var_dv0_db6;
        *var_dv0_db7_slot = var_dv0_db7;
        *var_dv0_dn0_slot = var_dv0_dn0;
        *var_dv0_dn1_slot = var_dv0_dn1;
        *var_dv0_dn2_slot = var_dv0_dn2;
        *var_dv0_dn3_slot = var_dv0_dn3;
        *var_dv0_dn4_slot = var_dv0_dn4;
        *var_dv0_dn5_slot = var_dv0_dn5;
        *var_dv0_dn6_slot = var_dv0_dn6;
        *var_dv0_dn7_slot = var_dv0_dn7;
        *var_dv0_dn8_slot = var_dv0_dn8;
        *var_dv0_dn9_slot = var_dv0_dn9;
        *var_dv0_rdb0_slot = var_dv0_rdb0;
        *var_dv0_rdb1_slot = var_dv0_rdb1;
        *var_dv0_rdb2_slot = var_dv0_rdb2;
        *var_dv0_rdb3_slot = var_dv0_rdb3;
        *var_dv0_rdb4_slot = var_dv0_rdb4;
        *var_dv0_rdb5_slot = var_dv0_rdb5;
        *var_dv0_rdb6_slot = var_dv0_rdb6;
        *var_dv0_rdb7_slot = var_dv0_rdb7;
        *var_dv0_rdn0_slot = var_dv0_rdn0;
        *var_dv0_rdn1_slot = var_dv0_rdn1;
        *var_dv0_rdn2_slot = var_dv0_rdn2;
        *var_dv0_rdn3_slot = var_dv0_rdn3;
        *var_dv0_rdn4_slot = var_dv0_rdn4;
        *var_dv0_rdn5_slot = var_dv0_rdn5;
        *var_dv0_rdn6_slot = var_dv0_rdn6;
        *var_dv0_rdn7_slot = var_dv0_rdn7;
        *var_dv0_rdn8_slot = var_dv0_rdn8;
        *var_dv0_rdn9_slot = var_dv0_rdn9;
        *var_dv0_rv_slot = var_dv0_rv;
        *var_dvh_slot = var_dvh;
        *var_dvh_db0_slot = var_dvh_db0;
        *var_dvh_db1_slot = var_dvh_db1;
        *var_dvh_db2_slot = var_dvh_db2;
        *var_dvh_db3_slot = var_dvh_db3;
        *var_dvh_db4_slot = var_dvh_db4;
        *var_dvh_db5_slot = var_dvh_db5;
        *var_dvh_db6_slot = var_dvh_db6;
        *var_dvh_db7_slot = var_dvh_db7;
        *var_dvh_dn0_slot = var_dvh_dn0;
        *var_dvh_dn1_slot = var_dvh_dn1;
        *var_dvh_dn2_slot = var_dvh_dn2;
        *var_dvh_dn3_slot = var_dvh_dn3;
        *var_dvh_dn4_slot = var_dvh_dn4;
        *var_dvh_dn5_slot = var_dvh_dn5;
        *var_dvh_dn6_slot = var_dvh_dn6;
        *var_dvh_dn7_slot = var_dvh_dn7;
        *var_dvh_dn8_slot = var_dvh_dn8;
        *var_dvh_dn9_slot = var_dvh_dn9;
        *var_dvh_rdb0_slot = var_dvh_rdb0;
        *var_dvh_rdb1_slot = var_dvh_rdb1;
        *var_dvh_rdb2_slot = var_dvh_rdb2;
        *var_dvh_rdb3_slot = var_dvh_rdb3;
        *var_dvh_rdb4_slot = var_dvh_rdb4;
        *var_dvh_rdb5_slot = var_dvh_rdb5;
        *var_dvh_rdb6_slot = var_dvh_rdb6;
        *var_dvh_rdb7_slot = var_dvh_rdb7;
        *var_dvh_rdn0_slot = var_dvh_rdn0;
        *var_dvh_rdn1_slot = var_dvh_rdn1;
        *var_dvh_rdn2_slot = var_dvh_rdn2;
        *var_dvh_rdn3_slot = var_dvh_rdn3;
        *var_dvh_rdn4_slot = var_dvh_rdn4;
        *var_dvh_rdn5_slot = var_dvh_rdn5;
        *var_dvh_rdn6_slot = var_dvh_rdn6;
        *var_dvh_rdn7_slot = var_dvh_rdn7;
        *var_dvh_rdn8_slot = var_dvh_rdn8;
        *var_dvh_rdn9_slot = var_dvh_rdn9;
        *var_dvh_rv_slot = var_dvh_rv;
        *var_guard13_slot = var_guard13;
        *var_guard13_db0_slot = var_guard13_db0;
        *var_guard13_db1_slot = var_guard13_db1;
        *var_guard13_db2_slot = var_guard13_db2;
        *var_guard13_db3_slot = var_guard13_db3;
        *var_guard13_db4_slot = var_guard13_db4;
        *var_guard13_db5_slot = var_guard13_db5;
        *var_guard13_db6_slot = var_guard13_db6;
        *var_guard13_db7_slot = var_guard13_db7;
        *var_guard13_dn0_slot = var_guard13_dn0;
        *var_guard13_dn1_slot = var_guard13_dn1;
        *var_guard13_dn2_slot = var_guard13_dn2;
        *var_guard13_dn3_slot = var_guard13_dn3;
        *var_guard13_dn4_slot = var_guard13_dn4;
        *var_guard13_dn5_slot = var_guard13_dn5;
        *var_guard13_dn6_slot = var_guard13_dn6;
        *var_guard13_dn7_slot = var_guard13_dn7;
        *var_guard13_dn8_slot = var_guard13_dn8;
        *var_guard13_dn9_slot = var_guard13_dn9;
        *var_guard13_rdb0_slot = var_guard13_rdb0;
        *var_guard13_rdb1_slot = var_guard13_rdb1;
        *var_guard13_rdb2_slot = var_guard13_rdb2;
        *var_guard13_rdb3_slot = var_guard13_rdb3;
        *var_guard13_rdb4_slot = var_guard13_rdb4;
        *var_guard13_rdb5_slot = var_guard13_rdb5;
        *var_guard13_rdb6_slot = var_guard13_rdb6;
        *var_guard13_rdb7_slot = var_guard13_rdb7;
        *var_guard13_rdn0_slot = var_guard13_rdn0;
        *var_guard13_rdn1_slot = var_guard13_rdn1;
        *var_guard13_rdn2_slot = var_guard13_rdn2;
        *var_guard13_rdn3_slot = var_guard13_rdn3;
        *var_guard13_rdn4_slot = var_guard13_rdn4;
        *var_guard13_rdn5_slot = var_guard13_rdn5;
        *var_guard13_rdn6_slot = var_guard13_rdn6;
        *var_guard13_rdn7_slot = var_guard13_rdn7;
        *var_guard13_rdn8_slot = var_guard13_rdn8;
        *var_guard13_rdn9_slot = var_guard13_rdn9;
        *var_guard13_rv_slot = var_guard13_rv;
        *var_guard15_slot = var_guard15;
        *var_guard15_db0_slot = var_guard15_db0;
        *var_guard15_db1_slot = var_guard15_db1;
        *var_guard15_db2_slot = var_guard15_db2;
        *var_guard15_db3_slot = var_guard15_db3;
        *var_guard15_db4_slot = var_guard15_db4;
        *var_guard15_db5_slot = var_guard15_db5;
        *var_guard15_db6_slot = var_guard15_db6;
        *var_guard15_db7_slot = var_guard15_db7;
        *var_guard15_dn0_slot = var_guard15_dn0;
        *var_guard15_dn1_slot = var_guard15_dn1;
        *var_guard15_dn2_slot = var_guard15_dn2;
        *var_guard15_dn3_slot = var_guard15_dn3;
        *var_guard15_dn4_slot = var_guard15_dn4;
        *var_guard15_dn5_slot = var_guard15_dn5;
        *var_guard15_dn6_slot = var_guard15_dn6;
        *var_guard15_dn7_slot = var_guard15_dn7;
        *var_guard15_dn8_slot = var_guard15_dn8;
        *var_guard15_dn9_slot = var_guard15_dn9;
        *var_guard15_rdb0_slot = var_guard15_rdb0;
        *var_guard15_rdb1_slot = var_guard15_rdb1;
        *var_guard15_rdb2_slot = var_guard15_rdb2;
        *var_guard15_rdb3_slot = var_guard15_rdb3;
        *var_guard15_rdb4_slot = var_guard15_rdb4;
        *var_guard15_rdb5_slot = var_guard15_rdb5;
        *var_guard15_rdb6_slot = var_guard15_rdb6;
        *var_guard15_rdb7_slot = var_guard15_rdb7;
        *var_guard15_rdn0_slot = var_guard15_rdn0;
        *var_guard15_rdn1_slot = var_guard15_rdn1;
        *var_guard15_rdn2_slot = var_guard15_rdn2;
        *var_guard15_rdn3_slot = var_guard15_rdn3;
        *var_guard15_rdn4_slot = var_guard15_rdn4;
        *var_guard15_rdn5_slot = var_guard15_rdn5;
        *var_guard15_rdn6_slot = var_guard15_rdn6;
        *var_guard15_rdn7_slot = var_guard15_rdn7;
        *var_guard15_rdn8_slot = var_guard15_rdn8;
        *var_guard15_rdn9_slot = var_guard15_rdn9;
        *var_guard15_rv_slot = var_guard15_rv;
        *var_guard16_slot = var_guard16;
        *var_guard16_db0_slot = var_guard16_db0;
        *var_guard16_db1_slot = var_guard16_db1;
        *var_guard16_db2_slot = var_guard16_db2;
        *var_guard16_db3_slot = var_guard16_db3;
        *var_guard16_db4_slot = var_guard16_db4;
        *var_guard16_db5_slot = var_guard16_db5;
        *var_guard16_db6_slot = var_guard16_db6;
        *var_guard16_db7_slot = var_guard16_db7;
        *var_guard16_dn0_slot = var_guard16_dn0;
        *var_guard16_dn1_slot = var_guard16_dn1;
        *var_guard16_dn2_slot = var_guard16_dn2;
        *var_guard16_dn3_slot = var_guard16_dn3;
        *var_guard16_dn4_slot = var_guard16_dn4;
        *var_guard16_dn5_slot = var_guard16_dn5;
        *var_guard16_dn6_slot = var_guard16_dn6;
        *var_guard16_dn7_slot = var_guard16_dn7;
        *var_guard16_dn8_slot = var_guard16_dn8;
        *var_guard16_dn9_slot = var_guard16_dn9;
        *var_guard16_rdb0_slot = var_guard16_rdb0;
        *var_guard16_rdb1_slot = var_guard16_rdb1;
        *var_guard16_rdb2_slot = var_guard16_rdb2;
        *var_guard16_rdb3_slot = var_guard16_rdb3;
        *var_guard16_rdb4_slot = var_guard16_rdb4;
        *var_guard16_rdb5_slot = var_guard16_rdb5;
        *var_guard16_rdb6_slot = var_guard16_rdb6;
        *var_guard16_rdb7_slot = var_guard16_rdb7;
        *var_guard16_rdn0_slot = var_guard16_rdn0;
        *var_guard16_rdn1_slot = var_guard16_rdn1;
        *var_guard16_rdn2_slot = var_guard16_rdn2;
        *var_guard16_rdn3_slot = var_guard16_rdn3;
        *var_guard16_rdn4_slot = var_guard16_rdn4;
        *var_guard16_rdn5_slot = var_guard16_rdn5;
        *var_guard16_rdn6_slot = var_guard16_rdn6;
        *var_guard16_rdn7_slot = var_guard16_rdn7;
        *var_guard16_rdn8_slot = var_guard16_rdn8;
        *var_guard16_rdn9_slot = var_guard16_rdn9;
        *var_guard16_rv_slot = var_guard16_rv;
        *var_pwq_slot = var_pwq;
        *var_pwq_db0_slot = var_pwq_db0;
        *var_pwq_db1_slot = var_pwq_db1;
        *var_pwq_db2_slot = var_pwq_db2;
        *var_pwq_db3_slot = var_pwq_db3;
        *var_pwq_db4_slot = var_pwq_db4;
        *var_pwq_db5_slot = var_pwq_db5;
        *var_pwq_db6_slot = var_pwq_db6;
        *var_pwq_db7_slot = var_pwq_db7;
        *var_pwq_dn0_slot = var_pwq_dn0;
        *var_pwq_dn1_slot = var_pwq_dn1;
        *var_pwq_dn2_slot = var_pwq_dn2;
        *var_pwq_dn3_slot = var_pwq_dn3;
        *var_pwq_dn4_slot = var_pwq_dn4;
        *var_pwq_dn5_slot = var_pwq_dn5;
        *var_pwq_dn6_slot = var_pwq_dn6;
        *var_pwq_dn7_slot = var_pwq_dn7;
        *var_pwq_dn8_slot = var_pwq_dn8;
        *var_pwq_dn9_slot = var_pwq_dn9;
        *var_pwq_rdb0_slot = var_pwq_rdb0;
        *var_pwq_rdb1_slot = var_pwq_rdb1;
        *var_pwq_rdb2_slot = var_pwq_rdb2;
        *var_pwq_rdb3_slot = var_pwq_rdb3;
        *var_pwq_rdb4_slot = var_pwq_rdb4;
        *var_pwq_rdb5_slot = var_pwq_rdb5;
        *var_pwq_rdb6_slot = var_pwq_rdb6;
        *var_pwq_rdb7_slot = var_pwq_rdb7;
        *var_pwq_rdn0_slot = var_pwq_rdn0;
        *var_pwq_rdn1_slot = var_pwq_rdn1;
        *var_pwq_rdn2_slot = var_pwq_rdn2;
        *var_pwq_rdn3_slot = var_pwq_rdn3;
        *var_pwq_rdn4_slot = var_pwq_rdn4;
        *var_pwq_rdn5_slot = var_pwq_rdn5;
        *var_pwq_rdn6_slot = var_pwq_rdn6;
        *var_pwq_rdn7_slot = var_pwq_rdn7;
        *var_pwq_rdn8_slot = var_pwq_rdn8;
        *var_pwq_rdn9_slot = var_pwq_rdn9;
        *var_pwq_rv_slot = var_pwq_rv;
        *var_qhi_slot = var_qhi;
        *var_qhi_db0_slot = var_qhi_db0;
        *var_qhi_db1_slot = var_qhi_db1;
        *var_qhi_db2_slot = var_qhi_db2;
        *var_qhi_db3_slot = var_qhi_db3;
        *var_qhi_db4_slot = var_qhi_db4;
        *var_qhi_db5_slot = var_qhi_db5;
        *var_qhi_db6_slot = var_qhi_db6;
        *var_qhi_db7_slot = var_qhi_db7;
        *var_qhi_dn0_slot = var_qhi_dn0;
        *var_qhi_dn1_slot = var_qhi_dn1;
        *var_qhi_dn2_slot = var_qhi_dn2;
        *var_qhi_dn3_slot = var_qhi_dn3;
        *var_qhi_dn4_slot = var_qhi_dn4;
        *var_qhi_dn5_slot = var_qhi_dn5;
        *var_qhi_dn6_slot = var_qhi_dn6;
        *var_qhi_dn7_slot = var_qhi_dn7;
        *var_qhi_dn8_slot = var_qhi_dn8;
        *var_qhi_dn9_slot = var_qhi_dn9;
        *var_qhi_rdb0_slot = var_qhi_rdb0;
        *var_qhi_rdb1_slot = var_qhi_rdb1;
        *var_qhi_rdb2_slot = var_qhi_rdb2;
        *var_qhi_rdb3_slot = var_qhi_rdb3;
        *var_qhi_rdb4_slot = var_qhi_rdb4;
        *var_qhi_rdb5_slot = var_qhi_rdb5;
        *var_qhi_rdb6_slot = var_qhi_rdb6;
        *var_qhi_rdb7_slot = var_qhi_rdb7;
        *var_qhi_rdn0_slot = var_qhi_rdn0;
        *var_qhi_rdn1_slot = var_qhi_rdn1;
        *var_qhi_rdn2_slot = var_qhi_rdn2;
        *var_qhi_rdn3_slot = var_qhi_rdn3;
        *var_qhi_rdn4_slot = var_qhi_rdn4;
        *var_qhi_rdn5_slot = var_qhi_rdn5;
        *var_qhi_rdn6_slot = var_qhi_rdn6;
        *var_qhi_rdn7_slot = var_qhi_rdn7;
        *var_qhi_rdn8_slot = var_qhi_rdn8;
        *var_qhi_rdn9_slot = var_qhi_rdn9;
        *var_qhi_rv_slot = var_qhi_rv;
        *var_qjs_slot = var_qjs;
        *var_qjs_db0_slot = var_qjs_db0;
        *var_qjs_db1_slot = var_qjs_db1;
        *var_qjs_db2_slot = var_qjs_db2;
        *var_qjs_db3_slot = var_qjs_db3;
        *var_qjs_db4_slot = var_qjs_db4;
        *var_qjs_db5_slot = var_qjs_db5;
        *var_qjs_db6_slot = var_qjs_db6;
        *var_qjs_db7_slot = var_qjs_db7;
        *var_qjs_dn0_slot = var_qjs_dn0;
        *var_qjs_dn1_slot = var_qjs_dn1;
        *var_qjs_dn2_slot = var_qjs_dn2;
        *var_qjs_dn3_slot = var_qjs_dn3;
        *var_qjs_dn4_slot = var_qjs_dn4;
        *var_qjs_dn5_slot = var_qjs_dn5;
        *var_qjs_dn6_slot = var_qjs_dn6;
        *var_qjs_dn7_slot = var_qjs_dn7;
        *var_qjs_dn8_slot = var_qjs_dn8;
        *var_qjs_dn9_slot = var_qjs_dn9;
        *var_qjs_rdb0_slot = var_qjs_rdb0;
        *var_qjs_rdb1_slot = var_qjs_rdb1;
        *var_qjs_rdb2_slot = var_qjs_rdb2;
        *var_qjs_rdb3_slot = var_qjs_rdb3;
        *var_qjs_rdb4_slot = var_qjs_rdb4;
        *var_qjs_rdb5_slot = var_qjs_rdb5;
        *var_qjs_rdb6_slot = var_qjs_rdb6;
        *var_qjs_rdb7_slot = var_qjs_rdb7;
        *var_qjs_rdn0_slot = var_qjs_rdn0;
        *var_qjs_rdn1_slot = var_qjs_rdn1;
        *var_qjs_rdn2_slot = var_qjs_rdn2;
        *var_qjs_rdn3_slot = var_qjs_rdn3;
        *var_qjs_rdn4_slot = var_qjs_rdn4;
        *var_qjs_rdn5_slot = var_qjs_rdn5;
        *var_qjs_rdn6_slot = var_qjs_rdn6;
        *var_qjs_rdn7_slot = var_qjs_rdn7;
        *var_qjs_rdn8_slot = var_qjs_rdn8;
        *var_qjs_rdn9_slot = var_qjs_rdn9;
        *var_qjs_rv_slot = var_qjs_rv;
        *var_qlo_slot = var_qlo;
        *var_qlo_db0_slot = var_qlo_db0;
        *var_qlo_db1_slot = var_qlo_db1;
        *var_qlo_db2_slot = var_qlo_db2;
        *var_qlo_db3_slot = var_qlo_db3;
        *var_qlo_db4_slot = var_qlo_db4;
        *var_qlo_db5_slot = var_qlo_db5;
        *var_qlo_db6_slot = var_qlo_db6;
        *var_qlo_db7_slot = var_qlo_db7;
        *var_qlo_dn0_slot = var_qlo_dn0;
        *var_qlo_dn1_slot = var_qlo_dn1;
        *var_qlo_dn2_slot = var_qlo_dn2;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_qlo_dn5_slot = var_qlo_dn5;
        *var_qlo_dn6_slot = var_qlo_dn6;
        *var_qlo_dn7_slot = var_qlo_dn7;
        *var_qlo_dn8_slot = var_qlo_dn8;
        *var_qlo_dn9_slot = var_qlo_dn9;
        *var_qlo_rdb0_slot = var_qlo_rdb0;
        *var_qlo_rdb1_slot = var_qlo_rdb1;
        *var_qlo_rdb2_slot = var_qlo_rdb2;
        *var_qlo_rdb3_slot = var_qlo_rdb3;
        *var_qlo_rdb4_slot = var_qlo_rdb4;
        *var_qlo_rdb5_slot = var_qlo_rdb5;
        *var_qlo_rdb6_slot = var_qlo_rdb6;
        *var_qlo_rdb7_slot = var_qlo_rdb7;
        *var_qlo_rdn0_slot = var_qlo_rdn0;
        *var_qlo_rdn1_slot = var_qlo_rdn1;
        *var_qlo_rdn2_slot = var_qlo_rdn2;
        *var_qlo_rdn3_slot = var_qlo_rdn3;
        *var_qlo_rdn4_slot = var_qlo_rdn4;
        *var_qlo_rdn5_slot = var_qlo_rdn5;
        *var_qlo_rdn6_slot = var_qlo_rdn6;
        *var_qlo_rdn7_slot = var_qlo_rdn7;
        *var_qlo_rdn8_slot = var_qlo_rdn8;
        *var_qlo_rdn9_slot = var_qlo_rdn9;
        *var_qlo_rv_slot = var_qlo_rv;
    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        var_cje_t: f64,
        var_cje_t_db0: f64,
        var_cje_t_db1: f64,
        var_cje_t_db2: f64,
        var_cje_t_db3: f64,
        var_cje_t_db4: f64,
        var_cje_t_db5: f64,
        var_cje_t_db6: f64,
        var_cje_t_db7: f64,
        var_cje_t_dn0: f64,
        var_cje_t_dn1: f64,
        var_cje_t_dn2: f64,
        var_cje_t_dn3: f64,
        var_cje_t_dn4: f64,
        var_cje_t_dn5: f64,
        var_cje_t_dn6: f64,
        var_cje_t_dn7: f64,
        var_cje_t_dn8: f64,
        var_cje_t_dn9: f64,
        var_guard16: f64,
        var_vbci: f64,
        var_vbci_db0: f64,
        var_vbci_db1: f64,
        var_vbci_db2: f64,
        var_vbci_db3: f64,
        var_vbci_db4: f64,
        var_vbci_db5: f64,
        var_vbci_db6: f64,
        var_vbci_db7: f64,
        var_vbci_dn0: f64,
        var_vbci_dn1: f64,
        var_vbci_dn2: f64,
        var_vbci_dn3: f64,
        var_vbci_dn4: f64,
        var_vbci_dn5: f64,
        var_vbci_dn6: f64,
        var_vbci_dn7: f64,
        var_vbci_dn8: f64,
        var_vbci_dn9: f64,
        var_vbiei: f64,
        var_vbiei_db0: f64,
        var_vbiei_db1: f64,
        var_vbiei_db2: f64,
        var_vbiei_db3: f64,
        var_vbiei_db4: f64,
        var_vbiei_db5: f64,
        var_vbiei_db6: f64,
        var_vbiei_db7: f64,
        var_vbiei_dn0: f64,
        var_vbiei_dn1: f64,
        var_vbiei_dn2: f64,
        var_vbiei_dn3: f64,
        var_vbiei_dn4: f64,
        var_vbiei_dn5: f64,
        var_vbiei_dn6: f64,
        var_vbiei_dn7: f64,
        var_vbiei_dn8: f64,
        var_vbiei_dn9: f64,
        var_vjc_t: f64,
        var_vjc_t_db0: f64,
        var_vjc_t_db1: f64,
        var_vjc_t_db2: f64,
        var_vjc_t_db3: f64,
        var_vjc_t_db4: f64,
        var_vjc_t_db5: f64,
        var_vjc_t_db6: f64,
        var_vjc_t_db7: f64,
        var_vjc_t_dn0: f64,
        var_vjc_t_dn1: f64,
        var_vjc_t_dn2: f64,
        var_vjc_t_dn3: f64,
        var_vjc_t_dn4: f64,
        var_vjc_t_dn5: f64,
        var_vjc_t_dn6: f64,
        var_vjc_t_dn7: f64,
        var_vjc_t_dn8: f64,
        var_vjc_t_dn9: f64,
        var_vje_t: f64,
        var_vje_t_db0: f64,
        var_vje_t_db1: f64,
        var_vje_t_db2: f64,
        var_vje_t_db3: f64,
        var_vje_t_db4: f64,
        var_vje_t_db5: f64,
        var_vje_t_db6: f64,
        var_vje_t_db7: f64,
        var_vje_t_dn0: f64,
        var_vje_t_dn1: f64,
        var_vje_t_dn2: f64,
        var_vje_t_dn3: f64,
        var_vje_t_dn4: f64,
        var_vje_t_dn5: f64,
        var_vje_t_dn6: f64,
        var_vje_t_dn7: f64,
        var_vje_t_dn8: f64,
        var_vje_t_dn9: f64,
        var_dv0_slot: &mut f64,
        var_dv0_db0_slot: &mut f64,
        var_dv0_db1_slot: &mut f64,
        var_dv0_db2_slot: &mut f64,
        var_dv0_db3_slot: &mut f64,
        var_dv0_db4_slot: &mut f64,
        var_dv0_db5_slot: &mut f64,
        var_dv0_db6_slot: &mut f64,
        var_dv0_db7_slot: &mut f64,
        var_dv0_dn0_slot: &mut f64,
        var_dv0_dn1_slot: &mut f64,
        var_dv0_dn2_slot: &mut f64,
        var_dv0_dn3_slot: &mut f64,
        var_dv0_dn4_slot: &mut f64,
        var_dv0_dn5_slot: &mut f64,
        var_dv0_dn6_slot: &mut f64,
        var_dv0_dn7_slot: &mut f64,
        var_dv0_dn8_slot: &mut f64,
        var_dv0_dn9_slot: &mut f64,
        var_dv0_rdb0_slot: &mut f64,
        var_dv0_rdb1_slot: &mut f64,
        var_dv0_rdb2_slot: &mut f64,
        var_dv0_rdb3_slot: &mut f64,
        var_dv0_rdb4_slot: &mut f64,
        var_dv0_rdb5_slot: &mut f64,
        var_dv0_rdb6_slot: &mut f64,
        var_dv0_rdb7_slot: &mut f64,
        var_dv0_rdn0_slot: &mut f64,
        var_dv0_rdn1_slot: &mut f64,
        var_dv0_rdn2_slot: &mut f64,
        var_dv0_rdn3_slot: &mut f64,
        var_dv0_rdn4_slot: &mut f64,
        var_dv0_rdn5_slot: &mut f64,
        var_dv0_rdn6_slot: &mut f64,
        var_dv0_rdn7_slot: &mut f64,
        var_dv0_rdn8_slot: &mut f64,
        var_dv0_rdn9_slot: &mut f64,
        var_dv0_rv_slot: &mut f64,
        var_dvh_slot: &mut f64,
        var_dvh_db0_slot: &mut f64,
        var_dvh_db1_slot: &mut f64,
        var_dvh_db2_slot: &mut f64,
        var_dvh_db3_slot: &mut f64,
        var_dvh_db4_slot: &mut f64,
        var_dvh_db5_slot: &mut f64,
        var_dvh_db6_slot: &mut f64,
        var_dvh_db7_slot: &mut f64,
        var_dvh_dn0_slot: &mut f64,
        var_dvh_dn1_slot: &mut f64,
        var_dvh_dn2_slot: &mut f64,
        var_dvh_dn3_slot: &mut f64,
        var_dvh_dn4_slot: &mut f64,
        var_dvh_dn5_slot: &mut f64,
        var_dvh_dn6_slot: &mut f64,
        var_dvh_dn7_slot: &mut f64,
        var_dvh_dn8_slot: &mut f64,
        var_dvh_dn9_slot: &mut f64,
        var_dvh_rdb0_slot: &mut f64,
        var_dvh_rdb1_slot: &mut f64,
        var_dvh_rdb2_slot: &mut f64,
        var_dvh_rdb3_slot: &mut f64,
        var_dvh_rdb4_slot: &mut f64,
        var_dvh_rdb5_slot: &mut f64,
        var_dvh_rdb6_slot: &mut f64,
        var_dvh_rdb7_slot: &mut f64,
        var_dvh_rdn0_slot: &mut f64,
        var_dvh_rdn1_slot: &mut f64,
        var_dvh_rdn2_slot: &mut f64,
        var_dvh_rdn3_slot: &mut f64,
        var_dvh_rdn4_slot: &mut f64,
        var_dvh_rdn5_slot: &mut f64,
        var_dvh_rdn6_slot: &mut f64,
        var_dvh_rdn7_slot: &mut f64,
        var_dvh_rdn8_slot: &mut f64,
        var_dvh_rdn9_slot: &mut f64,
        var_dvh_rv_slot: &mut f64,
        var_guard17_slot: &mut f64,
        var_guard17_db0_slot: &mut f64,
        var_guard17_db1_slot: &mut f64,
        var_guard17_db2_slot: &mut f64,
        var_guard17_db3_slot: &mut f64,
        var_guard17_db4_slot: &mut f64,
        var_guard17_db5_slot: &mut f64,
        var_guard17_db6_slot: &mut f64,
        var_guard17_db7_slot: &mut f64,
        var_guard17_dn0_slot: &mut f64,
        var_guard17_dn1_slot: &mut f64,
        var_guard17_dn2_slot: &mut f64,
        var_guard17_dn3_slot: &mut f64,
        var_guard17_dn4_slot: &mut f64,
        var_guard17_dn5_slot: &mut f64,
        var_guard17_dn6_slot: &mut f64,
        var_guard17_dn7_slot: &mut f64,
        var_guard17_dn8_slot: &mut f64,
        var_guard17_dn9_slot: &mut f64,
        var_guard17_rdb0_slot: &mut f64,
        var_guard17_rdb1_slot: &mut f64,
        var_guard17_rdb2_slot: &mut f64,
        var_guard17_rdb3_slot: &mut f64,
        var_guard17_rdb4_slot: &mut f64,
        var_guard17_rdb5_slot: &mut f64,
        var_guard17_rdb6_slot: &mut f64,
        var_guard17_rdb7_slot: &mut f64,
        var_guard17_rdn0_slot: &mut f64,
        var_guard17_rdn1_slot: &mut f64,
        var_guard17_rdn2_slot: &mut f64,
        var_guard17_rdn3_slot: &mut f64,
        var_guard17_rdn4_slot: &mut f64,
        var_guard17_rdn5_slot: &mut f64,
        var_guard17_rdn6_slot: &mut f64,
        var_guard17_rdn7_slot: &mut f64,
        var_guard17_rdn8_slot: &mut f64,
        var_guard17_rdn9_slot: &mut f64,
        var_guard17_rv_slot: &mut f64,
        var_pwq_slot: &mut f64,
        var_pwq_db0_slot: &mut f64,
        var_pwq_db1_slot: &mut f64,
        var_pwq_db2_slot: &mut f64,
        var_pwq_db3_slot: &mut f64,
        var_pwq_db4_slot: &mut f64,
        var_pwq_db5_slot: &mut f64,
        var_pwq_db6_slot: &mut f64,
        var_pwq_db7_slot: &mut f64,
        var_pwq_dn0_slot: &mut f64,
        var_pwq_dn1_slot: &mut f64,
        var_pwq_dn2_slot: &mut f64,
        var_pwq_dn3_slot: &mut f64,
        var_pwq_dn4_slot: &mut f64,
        var_pwq_dn5_slot: &mut f64,
        var_pwq_dn6_slot: &mut f64,
        var_pwq_dn7_slot: &mut f64,
        var_pwq_dn8_slot: &mut f64,
        var_pwq_dn9_slot: &mut f64,
        var_pwq_rdb0_slot: &mut f64,
        var_pwq_rdb1_slot: &mut f64,
        var_pwq_rdb2_slot: &mut f64,
        var_pwq_rdb3_slot: &mut f64,
        var_pwq_rdb4_slot: &mut f64,
        var_pwq_rdb5_slot: &mut f64,
        var_pwq_rdb6_slot: &mut f64,
        var_pwq_rdb7_slot: &mut f64,
        var_pwq_rdn0_slot: &mut f64,
        var_pwq_rdn1_slot: &mut f64,
        var_pwq_rdn2_slot: &mut f64,
        var_pwq_rdn3_slot: &mut f64,
        var_pwq_rdn4_slot: &mut f64,
        var_pwq_rdn5_slot: &mut f64,
        var_pwq_rdn6_slot: &mut f64,
        var_pwq_rdn7_slot: &mut f64,
        var_pwq_rdn8_slot: &mut f64,
        var_pwq_rdn9_slot: &mut f64,
        var_pwq_rv_slot: &mut f64,
        var_qhi_slot: &mut f64,
        var_qhi_db0_slot: &mut f64,
        var_qhi_db1_slot: &mut f64,
        var_qhi_db2_slot: &mut f64,
        var_qhi_db3_slot: &mut f64,
        var_qhi_db4_slot: &mut f64,
        var_qhi_db5_slot: &mut f64,
        var_qhi_db6_slot: &mut f64,
        var_qhi_db7_slot: &mut f64,
        var_qhi_dn0_slot: &mut f64,
        var_qhi_dn1_slot: &mut f64,
        var_qhi_dn2_slot: &mut f64,
        var_qhi_dn3_slot: &mut f64,
        var_qhi_dn4_slot: &mut f64,
        var_qhi_dn5_slot: &mut f64,
        var_qhi_dn6_slot: &mut f64,
        var_qhi_dn7_slot: &mut f64,
        var_qhi_dn8_slot: &mut f64,
        var_qhi_dn9_slot: &mut f64,
        var_qhi_rdb0_slot: &mut f64,
        var_qhi_rdb1_slot: &mut f64,
        var_qhi_rdb2_slot: &mut f64,
        var_qhi_rdb3_slot: &mut f64,
        var_qhi_rdb4_slot: &mut f64,
        var_qhi_rdb5_slot: &mut f64,
        var_qhi_rdb6_slot: &mut f64,
        var_qhi_rdb7_slot: &mut f64,
        var_qhi_rdn0_slot: &mut f64,
        var_qhi_rdn1_slot: &mut f64,
        var_qhi_rdn2_slot: &mut f64,
        var_qhi_rdn3_slot: &mut f64,
        var_qhi_rdn4_slot: &mut f64,
        var_qhi_rdn5_slot: &mut f64,
        var_qhi_rdn6_slot: &mut f64,
        var_qhi_rdn7_slot: &mut f64,
        var_qhi_rdn8_slot: &mut f64,
        var_qhi_rdn9_slot: &mut f64,
        var_qhi_rv_slot: &mut f64,
        var_qje_slot: &mut f64,
        var_qje_db0_slot: &mut f64,
        var_qje_db1_slot: &mut f64,
        var_qje_db2_slot: &mut f64,
        var_qje_db3_slot: &mut f64,
        var_qje_db4_slot: &mut f64,
        var_qje_db5_slot: &mut f64,
        var_qje_db6_slot: &mut f64,
        var_qje_db7_slot: &mut f64,
        var_qje_dn0_slot: &mut f64,
        var_qje_dn1_slot: &mut f64,
        var_qje_dn2_slot: &mut f64,
        var_qje_dn3_slot: &mut f64,
        var_qje_dn4_slot: &mut f64,
        var_qje_dn5_slot: &mut f64,
        var_qje_dn6_slot: &mut f64,
        var_qje_dn7_slot: &mut f64,
        var_qje_dn8_slot: &mut f64,
        var_qje_dn9_slot: &mut f64,
        var_qje_rdb0_slot: &mut f64,
        var_qje_rdb1_slot: &mut f64,
        var_qje_rdb2_slot: &mut f64,
        var_qje_rdb3_slot: &mut f64,
        var_qje_rdb4_slot: &mut f64,
        var_qje_rdb5_slot: &mut f64,
        var_qje_rdb6_slot: &mut f64,
        var_qje_rdb7_slot: &mut f64,
        var_qje_rdn0_slot: &mut f64,
        var_qje_rdn1_slot: &mut f64,
        var_qje_rdn2_slot: &mut f64,
        var_qje_rdn3_slot: &mut f64,
        var_qje_rdn4_slot: &mut f64,
        var_qje_rdn5_slot: &mut f64,
        var_qje_rdn6_slot: &mut f64,
        var_qje_rdn7_slot: &mut f64,
        var_qje_rdn8_slot: &mut f64,
        var_qje_rdn9_slot: &mut f64,
        var_qje_rv_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo_db0_slot: &mut f64,
        var_qlo_db1_slot: &mut f64,
        var_qlo_db2_slot: &mut f64,
        var_qlo_db3_slot: &mut f64,
        var_qlo_db4_slot: &mut f64,
        var_qlo_db5_slot: &mut f64,
        var_qlo_db6_slot: &mut f64,
        var_qlo_db7_slot: &mut f64,
        var_qlo_dn0_slot: &mut f64,
        var_qlo_dn1_slot: &mut f64,
        var_qlo_dn2_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_qlo_dn5_slot: &mut f64,
        var_qlo_dn6_slot: &mut f64,
        var_qlo_dn7_slot: &mut f64,
        var_qlo_dn8_slot: &mut f64,
        var_qlo_dn9_slot: &mut f64,
        var_qlo_rdb0_slot: &mut f64,
        var_qlo_rdb1_slot: &mut f64,
        var_qlo_rdb2_slot: &mut f64,
        var_qlo_rdb3_slot: &mut f64,
        var_qlo_rdb4_slot: &mut f64,
        var_qlo_rdb5_slot: &mut f64,
        var_qlo_rdb6_slot: &mut f64,
        var_qlo_rdb7_slot: &mut f64,
        var_qlo_rdn0_slot: &mut f64,
        var_qlo_rdn1_slot: &mut f64,
        var_qlo_rdn2_slot: &mut f64,
        var_qlo_rdn3_slot: &mut f64,
        var_qlo_rdn4_slot: &mut f64,
        var_qlo_rdn5_slot: &mut f64,
        var_qlo_rdn6_slot: &mut f64,
        var_qlo_rdn7_slot: &mut f64,
        var_qlo_rdn8_slot: &mut f64,
        var_qlo_rdn9_slot: &mut f64,
        var_qlo_rv_slot: &mut f64,
    ) {
        let mut var_dv0: f64 = *var_dv0_slot;
        let mut var_dv0_db0: f64 = *var_dv0_db0_slot;
        let mut var_dv0_db1: f64 = *var_dv0_db1_slot;
        let mut var_dv0_db2: f64 = *var_dv0_db2_slot;
        let mut var_dv0_db3: f64 = *var_dv0_db3_slot;
        let mut var_dv0_db4: f64 = *var_dv0_db4_slot;
        let mut var_dv0_db5: f64 = *var_dv0_db5_slot;
        let mut var_dv0_db6: f64 = *var_dv0_db6_slot;
        let mut var_dv0_db7: f64 = *var_dv0_db7_slot;
        let mut var_dv0_dn0: f64 = *var_dv0_dn0_slot;
        let mut var_dv0_dn1: f64 = *var_dv0_dn1_slot;
        let mut var_dv0_dn2: f64 = *var_dv0_dn2_slot;
        let mut var_dv0_dn3: f64 = *var_dv0_dn3_slot;
        let mut var_dv0_dn4: f64 = *var_dv0_dn4_slot;
        let mut var_dv0_dn5: f64 = *var_dv0_dn5_slot;
        let mut var_dv0_dn6: f64 = *var_dv0_dn6_slot;
        let mut var_dv0_dn7: f64 = *var_dv0_dn7_slot;
        let mut var_dv0_dn8: f64 = *var_dv0_dn8_slot;
        let mut var_dv0_dn9: f64 = *var_dv0_dn9_slot;
        let mut var_dv0_rdb0: f64 = *var_dv0_rdb0_slot;
        let mut var_dv0_rdb1: f64 = *var_dv0_rdb1_slot;
        let mut var_dv0_rdb2: f64 = *var_dv0_rdb2_slot;
        let mut var_dv0_rdb3: f64 = *var_dv0_rdb3_slot;
        let mut var_dv0_rdb4: f64 = *var_dv0_rdb4_slot;
        let mut var_dv0_rdb5: f64 = *var_dv0_rdb5_slot;
        let mut var_dv0_rdb6: f64 = *var_dv0_rdb6_slot;
        let mut var_dv0_rdb7: f64 = *var_dv0_rdb7_slot;
        let mut var_dv0_rdn0: f64 = *var_dv0_rdn0_slot;
        let mut var_dv0_rdn1: f64 = *var_dv0_rdn1_slot;
        let mut var_dv0_rdn2: f64 = *var_dv0_rdn2_slot;
        let mut var_dv0_rdn3: f64 = *var_dv0_rdn3_slot;
        let mut var_dv0_rdn4: f64 = *var_dv0_rdn4_slot;
        let mut var_dv0_rdn5: f64 = *var_dv0_rdn5_slot;
        let mut var_dv0_rdn6: f64 = *var_dv0_rdn6_slot;
        let mut var_dv0_rdn7: f64 = *var_dv0_rdn7_slot;
        let mut var_dv0_rdn8: f64 = *var_dv0_rdn8_slot;
        let mut var_dv0_rdn9: f64 = *var_dv0_rdn9_slot;
        let mut var_dv0_rv: f64 = *var_dv0_rv_slot;
        let mut var_dvh: f64 = *var_dvh_slot;
        let mut var_dvh_db0: f64 = *var_dvh_db0_slot;
        let mut var_dvh_db1: f64 = *var_dvh_db1_slot;
        let mut var_dvh_db2: f64 = *var_dvh_db2_slot;
        let mut var_dvh_db3: f64 = *var_dvh_db3_slot;
        let mut var_dvh_db4: f64 = *var_dvh_db4_slot;
        let mut var_dvh_db5: f64 = *var_dvh_db5_slot;
        let mut var_dvh_db6: f64 = *var_dvh_db6_slot;
        let mut var_dvh_db7: f64 = *var_dvh_db7_slot;
        let mut var_dvh_dn0: f64 = *var_dvh_dn0_slot;
        let mut var_dvh_dn1: f64 = *var_dvh_dn1_slot;
        let mut var_dvh_dn2: f64 = *var_dvh_dn2_slot;
        let mut var_dvh_dn3: f64 = *var_dvh_dn3_slot;
        let mut var_dvh_dn4: f64 = *var_dvh_dn4_slot;
        let mut var_dvh_dn5: f64 = *var_dvh_dn5_slot;
        let mut var_dvh_dn6: f64 = *var_dvh_dn6_slot;
        let mut var_dvh_dn7: f64 = *var_dvh_dn7_slot;
        let mut var_dvh_dn8: f64 = *var_dvh_dn8_slot;
        let mut var_dvh_dn9: f64 = *var_dvh_dn9_slot;
        let mut var_dvh_rdb0: f64 = *var_dvh_rdb0_slot;
        let mut var_dvh_rdb1: f64 = *var_dvh_rdb1_slot;
        let mut var_dvh_rdb2: f64 = *var_dvh_rdb2_slot;
        let mut var_dvh_rdb3: f64 = *var_dvh_rdb3_slot;
        let mut var_dvh_rdb4: f64 = *var_dvh_rdb4_slot;
        let mut var_dvh_rdb5: f64 = *var_dvh_rdb5_slot;
        let mut var_dvh_rdb6: f64 = *var_dvh_rdb6_slot;
        let mut var_dvh_rdb7: f64 = *var_dvh_rdb7_slot;
        let mut var_dvh_rdn0: f64 = *var_dvh_rdn0_slot;
        let mut var_dvh_rdn1: f64 = *var_dvh_rdn1_slot;
        let mut var_dvh_rdn2: f64 = *var_dvh_rdn2_slot;
        let mut var_dvh_rdn3: f64 = *var_dvh_rdn3_slot;
        let mut var_dvh_rdn4: f64 = *var_dvh_rdn4_slot;
        let mut var_dvh_rdn5: f64 = *var_dvh_rdn5_slot;
        let mut var_dvh_rdn6: f64 = *var_dvh_rdn6_slot;
        let mut var_dvh_rdn7: f64 = *var_dvh_rdn7_slot;
        let mut var_dvh_rdn8: f64 = *var_dvh_rdn8_slot;
        let mut var_dvh_rdn9: f64 = *var_dvh_rdn9_slot;
        let mut var_dvh_rv: f64 = *var_dvh_rv_slot;
        let mut var_guard17: f64 = *var_guard17_slot;
        let mut var_guard17_db0: f64 = *var_guard17_db0_slot;
        let mut var_guard17_db1: f64 = *var_guard17_db1_slot;
        let mut var_guard17_db2: f64 = *var_guard17_db2_slot;
        let mut var_guard17_db3: f64 = *var_guard17_db3_slot;
        let mut var_guard17_db4: f64 = *var_guard17_db4_slot;
        let mut var_guard17_db5: f64 = *var_guard17_db5_slot;
        let mut var_guard17_db6: f64 = *var_guard17_db6_slot;
        let mut var_guard17_db7: f64 = *var_guard17_db7_slot;
        let mut var_guard17_dn0: f64 = *var_guard17_dn0_slot;
        let mut var_guard17_dn1: f64 = *var_guard17_dn1_slot;
        let mut var_guard17_dn2: f64 = *var_guard17_dn2_slot;
        let mut var_guard17_dn3: f64 = *var_guard17_dn3_slot;
        let mut var_guard17_dn4: f64 = *var_guard17_dn4_slot;
        let mut var_guard17_dn5: f64 = *var_guard17_dn5_slot;
        let mut var_guard17_dn6: f64 = *var_guard17_dn6_slot;
        let mut var_guard17_dn7: f64 = *var_guard17_dn7_slot;
        let mut var_guard17_dn8: f64 = *var_guard17_dn8_slot;
        let mut var_guard17_dn9: f64 = *var_guard17_dn9_slot;
        let mut var_guard17_rdb0: f64 = *var_guard17_rdb0_slot;
        let mut var_guard17_rdb1: f64 = *var_guard17_rdb1_slot;
        let mut var_guard17_rdb2: f64 = *var_guard17_rdb2_slot;
        let mut var_guard17_rdb3: f64 = *var_guard17_rdb3_slot;
        let mut var_guard17_rdb4: f64 = *var_guard17_rdb4_slot;
        let mut var_guard17_rdb5: f64 = *var_guard17_rdb5_slot;
        let mut var_guard17_rdb6: f64 = *var_guard17_rdb6_slot;
        let mut var_guard17_rdb7: f64 = *var_guard17_rdb7_slot;
        let mut var_guard17_rdn0: f64 = *var_guard17_rdn0_slot;
        let mut var_guard17_rdn1: f64 = *var_guard17_rdn1_slot;
        let mut var_guard17_rdn2: f64 = *var_guard17_rdn2_slot;
        let mut var_guard17_rdn3: f64 = *var_guard17_rdn3_slot;
        let mut var_guard17_rdn4: f64 = *var_guard17_rdn4_slot;
        let mut var_guard17_rdn5: f64 = *var_guard17_rdn5_slot;
        let mut var_guard17_rdn6: f64 = *var_guard17_rdn6_slot;
        let mut var_guard17_rdn7: f64 = *var_guard17_rdn7_slot;
        let mut var_guard17_rdn8: f64 = *var_guard17_rdn8_slot;
        let mut var_guard17_rdn9: f64 = *var_guard17_rdn9_slot;
        let mut var_guard17_rv: f64 = *var_guard17_rv_slot;
        let mut var_pwq: f64 = *var_pwq_slot;
        let mut var_pwq_db0: f64 = *var_pwq_db0_slot;
        let mut var_pwq_db1: f64 = *var_pwq_db1_slot;
        let mut var_pwq_db2: f64 = *var_pwq_db2_slot;
        let mut var_pwq_db3: f64 = *var_pwq_db3_slot;
        let mut var_pwq_db4: f64 = *var_pwq_db4_slot;
        let mut var_pwq_db5: f64 = *var_pwq_db5_slot;
        let mut var_pwq_db6: f64 = *var_pwq_db6_slot;
        let mut var_pwq_db7: f64 = *var_pwq_db7_slot;
        let mut var_pwq_dn0: f64 = *var_pwq_dn0_slot;
        let mut var_pwq_dn1: f64 = *var_pwq_dn1_slot;
        let mut var_pwq_dn2: f64 = *var_pwq_dn2_slot;
        let mut var_pwq_dn3: f64 = *var_pwq_dn3_slot;
        let mut var_pwq_dn4: f64 = *var_pwq_dn4_slot;
        let mut var_pwq_dn5: f64 = *var_pwq_dn5_slot;
        let mut var_pwq_dn6: f64 = *var_pwq_dn6_slot;
        let mut var_pwq_dn7: f64 = *var_pwq_dn7_slot;
        let mut var_pwq_dn8: f64 = *var_pwq_dn8_slot;
        let mut var_pwq_dn9: f64 = *var_pwq_dn9_slot;
        let mut var_pwq_rdb0: f64 = *var_pwq_rdb0_slot;
        let mut var_pwq_rdb1: f64 = *var_pwq_rdb1_slot;
        let mut var_pwq_rdb2: f64 = *var_pwq_rdb2_slot;
        let mut var_pwq_rdb3: f64 = *var_pwq_rdb3_slot;
        let mut var_pwq_rdb4: f64 = *var_pwq_rdb4_slot;
        let mut var_pwq_rdb5: f64 = *var_pwq_rdb5_slot;
        let mut var_pwq_rdb6: f64 = *var_pwq_rdb6_slot;
        let mut var_pwq_rdb7: f64 = *var_pwq_rdb7_slot;
        let mut var_pwq_rdn0: f64 = *var_pwq_rdn0_slot;
        let mut var_pwq_rdn1: f64 = *var_pwq_rdn1_slot;
        let mut var_pwq_rdn2: f64 = *var_pwq_rdn2_slot;
        let mut var_pwq_rdn3: f64 = *var_pwq_rdn3_slot;
        let mut var_pwq_rdn4: f64 = *var_pwq_rdn4_slot;
        let mut var_pwq_rdn5: f64 = *var_pwq_rdn5_slot;
        let mut var_pwq_rdn6: f64 = *var_pwq_rdn6_slot;
        let mut var_pwq_rdn7: f64 = *var_pwq_rdn7_slot;
        let mut var_pwq_rdn8: f64 = *var_pwq_rdn8_slot;
        let mut var_pwq_rdn9: f64 = *var_pwq_rdn9_slot;
        let mut var_pwq_rv: f64 = *var_pwq_rv_slot;
        let mut var_qhi: f64 = *var_qhi_slot;
        let mut var_qhi_db0: f64 = *var_qhi_db0_slot;
        let mut var_qhi_db1: f64 = *var_qhi_db1_slot;
        let mut var_qhi_db2: f64 = *var_qhi_db2_slot;
        let mut var_qhi_db3: f64 = *var_qhi_db3_slot;
        let mut var_qhi_db4: f64 = *var_qhi_db4_slot;
        let mut var_qhi_db5: f64 = *var_qhi_db5_slot;
        let mut var_qhi_db6: f64 = *var_qhi_db6_slot;
        let mut var_qhi_db7: f64 = *var_qhi_db7_slot;
        let mut var_qhi_dn0: f64 = *var_qhi_dn0_slot;
        let mut var_qhi_dn1: f64 = *var_qhi_dn1_slot;
        let mut var_qhi_dn2: f64 = *var_qhi_dn2_slot;
        let mut var_qhi_dn3: f64 = *var_qhi_dn3_slot;
        let mut var_qhi_dn4: f64 = *var_qhi_dn4_slot;
        let mut var_qhi_dn5: f64 = *var_qhi_dn5_slot;
        let mut var_qhi_dn6: f64 = *var_qhi_dn6_slot;
        let mut var_qhi_dn7: f64 = *var_qhi_dn7_slot;
        let mut var_qhi_dn8: f64 = *var_qhi_dn8_slot;
        let mut var_qhi_dn9: f64 = *var_qhi_dn9_slot;
        let mut var_qhi_rdb0: f64 = *var_qhi_rdb0_slot;
        let mut var_qhi_rdb1: f64 = *var_qhi_rdb1_slot;
        let mut var_qhi_rdb2: f64 = *var_qhi_rdb2_slot;
        let mut var_qhi_rdb3: f64 = *var_qhi_rdb3_slot;
        let mut var_qhi_rdb4: f64 = *var_qhi_rdb4_slot;
        let mut var_qhi_rdb5: f64 = *var_qhi_rdb5_slot;
        let mut var_qhi_rdb6: f64 = *var_qhi_rdb6_slot;
        let mut var_qhi_rdb7: f64 = *var_qhi_rdb7_slot;
        let mut var_qhi_rdn0: f64 = *var_qhi_rdn0_slot;
        let mut var_qhi_rdn1: f64 = *var_qhi_rdn1_slot;
        let mut var_qhi_rdn2: f64 = *var_qhi_rdn2_slot;
        let mut var_qhi_rdn3: f64 = *var_qhi_rdn3_slot;
        let mut var_qhi_rdn4: f64 = *var_qhi_rdn4_slot;
        let mut var_qhi_rdn5: f64 = *var_qhi_rdn5_slot;
        let mut var_qhi_rdn6: f64 = *var_qhi_rdn6_slot;
        let mut var_qhi_rdn7: f64 = *var_qhi_rdn7_slot;
        let mut var_qhi_rdn8: f64 = *var_qhi_rdn8_slot;
        let mut var_qhi_rdn9: f64 = *var_qhi_rdn9_slot;
        let mut var_qhi_rv: f64 = *var_qhi_rv_slot;
        let mut var_qje: f64 = *var_qje_slot;
        let mut var_qje_db0: f64 = *var_qje_db0_slot;
        let mut var_qje_db1: f64 = *var_qje_db1_slot;
        let mut var_qje_db2: f64 = *var_qje_db2_slot;
        let mut var_qje_db3: f64 = *var_qje_db3_slot;
        let mut var_qje_db4: f64 = *var_qje_db4_slot;
        let mut var_qje_db5: f64 = *var_qje_db5_slot;
        let mut var_qje_db6: f64 = *var_qje_db6_slot;
        let mut var_qje_db7: f64 = *var_qje_db7_slot;
        let mut var_qje_dn0: f64 = *var_qje_dn0_slot;
        let mut var_qje_dn1: f64 = *var_qje_dn1_slot;
        let mut var_qje_dn2: f64 = *var_qje_dn2_slot;
        let mut var_qje_dn3: f64 = *var_qje_dn3_slot;
        let mut var_qje_dn4: f64 = *var_qje_dn4_slot;
        let mut var_qje_dn5: f64 = *var_qje_dn5_slot;
        let mut var_qje_dn6: f64 = *var_qje_dn6_slot;
        let mut var_qje_dn7: f64 = *var_qje_dn7_slot;
        let mut var_qje_dn8: f64 = *var_qje_dn8_slot;
        let mut var_qje_dn9: f64 = *var_qje_dn9_slot;
        let mut var_qje_rdb0: f64 = *var_qje_rdb0_slot;
        let mut var_qje_rdb1: f64 = *var_qje_rdb1_slot;
        let mut var_qje_rdb2: f64 = *var_qje_rdb2_slot;
        let mut var_qje_rdb3: f64 = *var_qje_rdb3_slot;
        let mut var_qje_rdb4: f64 = *var_qje_rdb4_slot;
        let mut var_qje_rdb5: f64 = *var_qje_rdb5_slot;
        let mut var_qje_rdb6: f64 = *var_qje_rdb6_slot;
        let mut var_qje_rdb7: f64 = *var_qje_rdb7_slot;
        let mut var_qje_rdn0: f64 = *var_qje_rdn0_slot;
        let mut var_qje_rdn1: f64 = *var_qje_rdn1_slot;
        let mut var_qje_rdn2: f64 = *var_qje_rdn2_slot;
        let mut var_qje_rdn3: f64 = *var_qje_rdn3_slot;
        let mut var_qje_rdn4: f64 = *var_qje_rdn4_slot;
        let mut var_qje_rdn5: f64 = *var_qje_rdn5_slot;
        let mut var_qje_rdn6: f64 = *var_qje_rdn6_slot;
        let mut var_qje_rdn7: f64 = *var_qje_rdn7_slot;
        let mut var_qje_rdn8: f64 = *var_qje_rdn8_slot;
        let mut var_qje_rdn9: f64 = *var_qje_rdn9_slot;
        let mut var_qje_rv: f64 = *var_qje_rv_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo_db0: f64 = *var_qlo_db0_slot;
        let mut var_qlo_db1: f64 = *var_qlo_db1_slot;
        let mut var_qlo_db2: f64 = *var_qlo_db2_slot;
        let mut var_qlo_db3: f64 = *var_qlo_db3_slot;
        let mut var_qlo_db4: f64 = *var_qlo_db4_slot;
        let mut var_qlo_db5: f64 = *var_qlo_db5_slot;
        let mut var_qlo_db6: f64 = *var_qlo_db6_slot;
        let mut var_qlo_db7: f64 = *var_qlo_db7_slot;
        let mut var_qlo_dn0: f64 = *var_qlo_dn0_slot;
        let mut var_qlo_dn1: f64 = *var_qlo_dn1_slot;
        let mut var_qlo_dn2: f64 = *var_qlo_dn2_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_qlo_dn5: f64 = *var_qlo_dn5_slot;
        let mut var_qlo_dn6: f64 = *var_qlo_dn6_slot;
        let mut var_qlo_dn7: f64 = *var_qlo_dn7_slot;
        let mut var_qlo_dn8: f64 = *var_qlo_dn8_slot;
        let mut var_qlo_dn9: f64 = *var_qlo_dn9_slot;
        let mut var_qlo_rdb0: f64 = *var_qlo_rdb0_slot;
        let mut var_qlo_rdb1: f64 = *var_qlo_rdb1_slot;
        let mut var_qlo_rdb2: f64 = *var_qlo_rdb2_slot;
        let mut var_qlo_rdb3: f64 = *var_qlo_rdb3_slot;
        let mut var_qlo_rdb4: f64 = *var_qlo_rdb4_slot;
        let mut var_qlo_rdb5: f64 = *var_qlo_rdb5_slot;
        let mut var_qlo_rdb6: f64 = *var_qlo_rdb6_slot;
        let mut var_qlo_rdb7: f64 = *var_qlo_rdb7_slot;
        let mut var_qlo_rdn0: f64 = *var_qlo_rdn0_slot;
        let mut var_qlo_rdn1: f64 = *var_qlo_rdn1_slot;
        let mut var_qlo_rdn2: f64 = *var_qlo_rdn2_slot;
        let mut var_qlo_rdn3: f64 = *var_qlo_rdn3_slot;
        let mut var_qlo_rdn4: f64 = *var_qlo_rdn4_slot;
        let mut var_qlo_rdn5: f64 = *var_qlo_rdn5_slot;
        let mut var_qlo_rdn6: f64 = *var_qlo_rdn6_slot;
        let mut var_qlo_rdn7: f64 = *var_qlo_rdn7_slot;
        let mut var_qlo_rdn8: f64 = *var_qlo_rdn8_slot;
        let mut var_qlo_rdn9: f64 = *var_qlo_rdn9_slot;
        let mut var_qlo_rv: f64 = *var_qlo_rv_slot;

        let (assign1660_e1977, assign1660_e1977_d_n0, assign1660_e1977_d_n1, assign1660_e1977_d_n2, assign1660_e1977_d_n3, assign1660_e1977_d_n4, assign1660_e1977_d_n5, assign1660_e1977_d_n6, assign1660_e1977_d_n7, assign1660_e1977_d_n8, assign1660_e1977_d_n9, assign1660_e1977_d_b0, assign1660_e1977_d_b1, assign1660_e1977_d_b2, assign1660_e1977_d_b3, assign1660_e1977_d_b4, assign1660_e1977_d_b5, assign1660_e1977_d_b6, assign1660_e1977_d_b7,) = {
    if (var_guard16 == 0.0) {
        let assign1660_e1961: f64 = (1.0 - p.p18);
        let assign1660_e1965: f64 = (var_vbiei / var_vje_t);
        let assign1660_e1966: f64 = (1.0 - assign1660_e1965);
        let assign1660_e1967: f64 = (assign1660_e1966).ln();
        let assign1660_e1968: f64 = (assign1660_e1961 * assign1660_e1967);
        let assign1660_e1969: f64 = (assign1660_e1968).exp();
        let assign1660_e1970: f64 = (1.0 - assign1660_e1969);
        let assign1660_e1971: f64 = (var_vje_t * assign1660_e1970);
        let assign1660_e1974: f64 = (1.0 - p.p18);
        let assign1660_e1975: f64 = (assign1660_e1971 / assign1660_e1974);
        (assign1660_e1975, (((var_vje_t_dn0 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn0 * var_vje_t) - (var_vbiei * var_vje_t_dn0)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn1 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn1 * var_vje_t) - (var_vbiei * var_vje_t_dn1)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn2 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn2 * var_vje_t) - (var_vbiei * var_vje_t_dn2)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn3 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn3 * var_vje_t) - (var_vbiei * var_vje_t_dn3)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn4 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn4 * var_vje_t) - (var_vbiei * var_vje_t_dn4)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn5 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn5 * var_vje_t) - (var_vbiei * var_vje_t_dn5)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn6 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn6 * var_vje_t) - (var_vbiei * var_vje_t_dn6)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn7 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn7 * var_vje_t) - (var_vbiei * var_vje_t_dn7)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn8 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn8 * var_vje_t) - (var_vbiei * var_vje_t_dn8)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_dn9 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_dn9 * var_vje_t) - (var_vbiei * var_vje_t_dn9)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db0 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db0 * var_vje_t) - (var_vbiei * var_vje_t_db0)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db1 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db1 * var_vje_t) - (var_vbiei * var_vje_t_db1)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db2 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db2 * var_vje_t) - (var_vbiei * var_vje_t_db2)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db3 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db3 * var_vje_t) - (var_vbiei * var_vje_t_db3)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db4 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db4 * var_vje_t) - (var_vbiei * var_vje_t_db4)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db5 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db5 * var_vje_t) - (var_vbiei * var_vje_t_db5)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db6 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db6 * var_vje_t) - (var_vbiei * var_vje_t_db6)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974), (((var_vje_t_db7 * assign1660_e1970) + (var_vje_t * (-(assign1660_e1969 * (assign1660_e1961 * ((-(((var_vbiei_db7 * var_vje_t) - (var_vbiei * var_vje_t_db7)) / (var_vje_t * var_vje_t))) / assign1660_e1966)))))) / assign1660_e1974),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7,)
    }
};
        var_qlo = assign1660_e1977;
        var_qlo_dn0 = assign1660_e1977_d_n0;
        var_qlo_dn1 = assign1660_e1977_d_n1;
        var_qlo_dn2 = assign1660_e1977_d_n2;
        var_qlo_dn3 = assign1660_e1977_d_n3;
        var_qlo_dn4 = assign1660_e1977_d_n4;
        var_qlo_dn5 = assign1660_e1977_d_n5;
        var_qlo_dn6 = assign1660_e1977_d_n6;
        var_qlo_dn7 = assign1660_e1977_d_n7;
        var_qlo_dn8 = assign1660_e1977_d_n8;
        var_qlo_dn9 = assign1660_e1977_d_n9;
        var_qlo_db0 = assign1660_e1977_d_b0;
        var_qlo_db1 = assign1660_e1977_d_b1;
        var_qlo_db2 = assign1660_e1977_d_b2;
        var_qlo_db3 = assign1660_e1977_d_b3;
        var_qlo_db4 = assign1660_e1977_d_b4;
        var_qlo_db5 = assign1660_e1977_d_b5;
        var_qlo_db6 = assign1660_e1977_d_b6;
        var_qlo_db7 = assign1660_e1977_d_b7;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdn7 = 0.0;
        var_qlo_rdn8 = 0.0;
        var_qlo_rdn9 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;
        var_qlo_rdb7 = 0.0;

        let (assign1670_e1982, assign1670_e1982_d_n0, assign1670_e1982_d_n1, assign1670_e1982_d_n2, assign1670_e1982_d_n3, assign1670_e1982_d_n4, assign1670_e1982_d_n5, assign1670_e1982_d_n6, assign1670_e1982_d_n7, assign1670_e1982_d_n8, assign1670_e1982_d_n9, assign1670_e1982_d_b0, assign1670_e1982_d_b1, assign1670_e1982_d_b2, assign1670_e1982_d_b3, assign1670_e1982_d_b4, assign1670_e1982_d_b5, assign1670_e1982_d_b6, assign1670_e1982_d_b7,) = {
    if (var_guard16 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_dn7, var_qhi_dn8, var_qhi_dn9, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6, var_qhi_db7,)
    }
};
        var_qhi = assign1670_e1982;
        var_qhi_dn0 = assign1670_e1982_d_n0;
        var_qhi_dn1 = assign1670_e1982_d_n1;
        var_qhi_dn2 = assign1670_e1982_d_n2;
        var_qhi_dn3 = assign1670_e1982_d_n3;
        var_qhi_dn4 = assign1670_e1982_d_n4;
        var_qhi_dn5 = assign1670_e1982_d_n5;
        var_qhi_dn6 = assign1670_e1982_d_n6;
        var_qhi_dn7 = assign1670_e1982_d_n7;
        var_qhi_dn8 = assign1670_e1982_d_n8;
        var_qhi_dn9 = assign1670_e1982_d_n9;
        var_qhi_db0 = assign1670_e1982_d_b0;
        var_qhi_db1 = assign1670_e1982_d_b1;
        var_qhi_db2 = assign1670_e1982_d_b2;
        var_qhi_db3 = assign1670_e1982_d_b3;
        var_qhi_db4 = assign1670_e1982_d_b4;
        var_qhi_db5 = assign1670_e1982_d_b5;
        var_qhi_db6 = assign1670_e1982_d_b6;
        var_qhi_db7 = assign1670_e1982_d_b7;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdn7 = 0.0;
        var_qhi_rdn8 = 0.0;
        var_qhi_rdn9 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;
        var_qhi_rdb7 = 0.0;

        let assign1680_e1986: f64 = (var_qlo + var_qhi);
        let assign1680_e1987: f64 = (var_cje_t * assign1680_e1986);
        var_qje = assign1680_e1987;
        var_qje_dn0 = ((var_cje_t_dn0 * assign1680_e1986) + (var_cje_t * (var_qlo_dn0 + var_qhi_dn0)));
        var_qje_dn1 = ((var_cje_t_dn1 * assign1680_e1986) + (var_cje_t * (var_qlo_dn1 + var_qhi_dn1)));
        var_qje_dn2 = ((var_cje_t_dn2 * assign1680_e1986) + (var_cje_t * (var_qlo_dn2 + var_qhi_dn2)));
        var_qje_dn3 = ((var_cje_t_dn3 * assign1680_e1986) + (var_cje_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qje_dn4 = ((var_cje_t_dn4 * assign1680_e1986) + (var_cje_t * (var_qlo_dn4 + var_qhi_dn4)));
        var_qje_dn5 = ((var_cje_t_dn5 * assign1680_e1986) + (var_cje_t * (var_qlo_dn5 + var_qhi_dn5)));
        var_qje_dn6 = ((var_cje_t_dn6 * assign1680_e1986) + (var_cje_t * (var_qlo_dn6 + var_qhi_dn6)));
        var_qje_dn7 = ((var_cje_t_dn7 * assign1680_e1986) + (var_cje_t * (var_qlo_dn7 + var_qhi_dn7)));
        var_qje_dn8 = ((var_cje_t_dn8 * assign1680_e1986) + (var_cje_t * (var_qlo_dn8 + var_qhi_dn8)));
        var_qje_dn9 = ((var_cje_t_dn9 * assign1680_e1986) + (var_cje_t * (var_qlo_dn9 + var_qhi_dn9)));
        var_qje_db0 = ((var_cje_t_db0 * assign1680_e1986) + (var_cje_t * (var_qlo_db0 + var_qhi_db0)));
        var_qje_db1 = ((var_cje_t_db1 * assign1680_e1986) + (var_cje_t * (var_qlo_db1 + var_qhi_db1)));
        var_qje_db2 = ((var_cje_t_db2 * assign1680_e1986) + (var_cje_t * (var_qlo_db2 + var_qhi_db2)));
        var_qje_db3 = ((var_cje_t_db3 * assign1680_e1986) + (var_cje_t * (var_qlo_db3 + var_qhi_db3)));
        var_qje_db4 = ((var_cje_t_db4 * assign1680_e1986) + (var_cje_t * (var_qlo_db4 + var_qhi_db4)));
        var_qje_db5 = ((var_cje_t_db5 * assign1680_e1986) + (var_cje_t * (var_qlo_db5 + var_qhi_db5)));
        var_qje_db6 = ((var_cje_t_db6 * assign1680_e1986) + (var_cje_t * (var_qlo_db6 + var_qhi_db6)));
        var_qje_db7 = ((var_cje_t_db7 * assign1680_e1986) + (var_cje_t * (var_qlo_db7 + var_qhi_db7)));
        var_qje_rv = 0.0;
        var_qje_rdn0 = 0.0;
        var_qje_rdn1 = 0.0;
        var_qje_rdn2 = 0.0;
        var_qje_rdn3 = 0.0;
        var_qje_rdn4 = 0.0;
        var_qje_rdn5 = 0.0;
        var_qje_rdn6 = 0.0;
        var_qje_rdn7 = 0.0;
        var_qje_rdn8 = 0.0;
        var_qje_rdn9 = 0.0;
        var_qje_rdb0 = 0.0;
        var_qje_rdb1 = 0.0;
        var_qje_rdb2 = 0.0;
        var_qje_rdb3 = 0.0;
        var_qje_rdb4 = 0.0;
        var_qje_rdb5 = 0.0;
        var_qje_rdb6 = 0.0;
        var_qje_rdb7 = 0.0;

        let assign1690_e1989: f64 = (-var_vjc_t);
        let assign1690_e1991: f64 = (assign1690_e1989 * p.p24);
        var_dv0 = assign1690_e1991;
        var_dv0_dn0 = ((-var_vjc_t_dn0) * p.p24);
        var_dv0_dn1 = ((-var_vjc_t_dn1) * p.p24);
        var_dv0_dn2 = ((-var_vjc_t_dn2) * p.p24);
        var_dv0_dn3 = ((-var_vjc_t_dn3) * p.p24);
        var_dv0_dn4 = ((-var_vjc_t_dn4) * p.p24);
        var_dv0_dn5 = ((-var_vjc_t_dn5) * p.p24);
        var_dv0_dn6 = ((-var_vjc_t_dn6) * p.p24);
        var_dv0_dn7 = ((-var_vjc_t_dn7) * p.p24);
        var_dv0_dn8 = ((-var_vjc_t_dn8) * p.p24);
        var_dv0_dn9 = ((-var_vjc_t_dn9) * p.p24);
        var_dv0_db0 = ((-var_vjc_t_db0) * p.p24);
        var_dv0_db1 = ((-var_vjc_t_db1) * p.p24);
        var_dv0_db2 = ((-var_vjc_t_db2) * p.p24);
        var_dv0_db3 = ((-var_vjc_t_db3) * p.p24);
        var_dv0_db4 = ((-var_vjc_t_db4) * p.p24);
        var_dv0_db5 = ((-var_vjc_t_db5) * p.p24);
        var_dv0_db6 = ((-var_vjc_t_db6) * p.p24);
        var_dv0_db7 = ((-var_vjc_t_db7) * p.p24);
        var_dv0_rv = 0.0;
        var_dv0_rdn0 = 0.0;
        var_dv0_rdn1 = 0.0;
        var_dv0_rdn2 = 0.0;
        var_dv0_rdn3 = 0.0;
        var_dv0_rdn4 = 0.0;
        var_dv0_rdn5 = 0.0;
        var_dv0_rdn6 = 0.0;
        var_dv0_rdn7 = 0.0;
        var_dv0_rdn8 = 0.0;
        var_dv0_rdn9 = 0.0;
        var_dv0_rdb0 = 0.0;
        var_dv0_rdb1 = 0.0;
        var_dv0_rdb2 = 0.0;
        var_dv0_rdb3 = 0.0;
        var_dv0_rdb4 = 0.0;
        var_dv0_rdb5 = 0.0;
        var_dv0_rdb6 = 0.0;
        var_dv0_rdb7 = 0.0;

        let assign1700_e1994: f64 = (var_vbci + var_dv0);
        var_dvh = assign1700_e1994;
        var_dvh_dn0 = (var_vbci_dn0 + var_dv0_dn0);
        var_dvh_dn1 = (var_vbci_dn1 + var_dv0_dn1);
        var_dvh_dn2 = (var_vbci_dn2 + var_dv0_dn2);
        var_dvh_dn3 = (var_vbci_dn3 + var_dv0_dn3);
        var_dvh_dn4 = (var_vbci_dn4 + var_dv0_dn4);
        var_dvh_dn5 = (var_vbci_dn5 + var_dv0_dn5);
        var_dvh_dn6 = (var_vbci_dn6 + var_dv0_dn6);
        var_dvh_dn7 = (var_vbci_dn7 + var_dv0_dn7);
        var_dvh_dn8 = (var_vbci_dn8 + var_dv0_dn8);
        var_dvh_dn9 = (var_vbci_dn9 + var_dv0_dn9);
        var_dvh_db0 = (var_vbci_db0 + var_dv0_db0);
        var_dvh_db1 = (var_vbci_db1 + var_dv0_db1);
        var_dvh_db2 = (var_vbci_db2 + var_dv0_db2);
        var_dvh_db3 = (var_vbci_db3 + var_dv0_db3);
        var_dvh_db4 = (var_vbci_db4 + var_dv0_db4);
        var_dvh_db5 = (var_vbci_db5 + var_dv0_db5);
        var_dvh_db6 = (var_vbci_db6 + var_dv0_db6);
        var_dvh_db7 = (var_vbci_db7 + var_dv0_db7);
        var_dvh_rv = 0.0;
        var_dvh_rdn0 = 0.0;
        var_dvh_rdn1 = 0.0;
        var_dvh_rdn2 = 0.0;
        var_dvh_rdn3 = 0.0;
        var_dvh_rdn4 = 0.0;
        var_dvh_rdn5 = 0.0;
        var_dvh_rdn6 = 0.0;
        var_dvh_rdn7 = 0.0;
        var_dvh_rdn8 = 0.0;
        var_dvh_rdn9 = 0.0;
        var_dvh_rdb0 = 0.0;
        var_dvh_rdb1 = 0.0;
        var_dvh_rdb2 = 0.0;
        var_dvh_rdb3 = 0.0;
        var_dvh_rdb4 = 0.0;
        var_dvh_rdb5 = 0.0;
        var_dvh_rdb6 = 0.0;
        var_dvh_rdb7 = 0.0;

        let assign1710_e1997: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard17 = assign1710_e1997;
        var_guard17_dn0 = 0.0;
        var_guard17_dn1 = 0.0;
        var_guard17_dn2 = 0.0;
        var_guard17_dn3 = 0.0;
        var_guard17_dn4 = 0.0;
        var_guard17_dn5 = 0.0;
        var_guard17_dn6 = 0.0;
        var_guard17_dn7 = 0.0;
        var_guard17_dn8 = 0.0;
        var_guard17_dn9 = 0.0;
        var_guard17_db0 = 0.0;
        var_guard17_db1 = 0.0;
        var_guard17_db2 = 0.0;
        var_guard17_db3 = 0.0;
        var_guard17_db4 = 0.0;
        var_guard17_db5 = 0.0;
        var_guard17_db6 = 0.0;
        var_guard17_db7 = 0.0;
        var_guard17_rv = 0.0;
        var_guard17_rdn0 = 0.0;
        var_guard17_rdn1 = 0.0;
        var_guard17_rdn2 = 0.0;
        var_guard17_rdn3 = 0.0;
        var_guard17_rdn4 = 0.0;
        var_guard17_rdn5 = 0.0;
        var_guard17_rdn6 = 0.0;
        var_guard17_rdn7 = 0.0;
        var_guard17_rdn8 = 0.0;
        var_guard17_rdn9 = 0.0;
        var_guard17_rdb0 = 0.0;
        var_guard17_rdb1 = 0.0;
        var_guard17_rdb2 = 0.0;
        var_guard17_rdb3 = 0.0;
        var_guard17_rdb4 = 0.0;
        var_guard17_rdb5 = 0.0;
        var_guard17_rdb6 = 0.0;
        var_guard17_rdb7 = 0.0;

        let (assign1720_e2010, assign1720_e2010_d_n0, assign1720_e2010_d_n1, assign1720_e2010_d_n2, assign1720_e2010_d_n3, assign1720_e2010_d_n4, assign1720_e2010_d_n5, assign1720_e2010_d_n6, assign1720_e2010_d_n7, assign1720_e2010_d_n8, assign1720_e2010_d_n9, assign1720_e2010_d_b0, assign1720_e2010_d_b1, assign1720_e2010_d_b2, assign1720_e2010_d_b3, assign1720_e2010_d_b4, assign1720_e2010_d_b5, assign1720_e2010_d_b6, assign1720_e2010_d_b7,) = {
    if (var_guard17 != 0.0) {
        let assign1720_e2000: f64 = (-1.0);
        let assign1720_e2002: f64 = (assign1720_e2000 - p.p71);
        let assign1720_e2005: f64 = (1.0 - p.p24);
        let assign1720_e2006: f64 = (assign1720_e2005).ln();
        let assign1720_e2007: f64 = (assign1720_e2002 * assign1720_e2006);
        let assign1720_e2008: f64 = (assign1720_e2007).exp();
        (assign1720_e2008, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pwq, var_pwq_dn0, var_pwq_dn1, var_pwq_dn2, var_pwq_dn3, var_pwq_dn4, var_pwq_dn5, var_pwq_dn6, var_pwq_dn7, var_pwq_dn8, var_pwq_dn9, var_pwq_db0, var_pwq_db1, var_pwq_db2, var_pwq_db3, var_pwq_db4, var_pwq_db5, var_pwq_db6, var_pwq_db7,)
    }
};
        var_pwq = assign1720_e2010;
        var_pwq_dn0 = assign1720_e2010_d_n0;
        var_pwq_dn1 = assign1720_e2010_d_n1;
        var_pwq_dn2 = assign1720_e2010_d_n2;
        var_pwq_dn3 = assign1720_e2010_d_n3;
        var_pwq_dn4 = assign1720_e2010_d_n4;
        var_pwq_dn5 = assign1720_e2010_d_n5;
        var_pwq_dn6 = assign1720_e2010_d_n6;
        var_pwq_dn7 = assign1720_e2010_d_n7;
        var_pwq_dn8 = assign1720_e2010_d_n8;
        var_pwq_dn9 = assign1720_e2010_d_n9;
        var_pwq_db0 = assign1720_e2010_d_b0;
        var_pwq_db1 = assign1720_e2010_d_b1;
        var_pwq_db2 = assign1720_e2010_d_b2;
        var_pwq_db3 = assign1720_e2010_d_b3;
        var_pwq_db4 = assign1720_e2010_d_b4;
        var_pwq_db5 = assign1720_e2010_d_b5;
        var_pwq_db6 = assign1720_e2010_d_b6;
        var_pwq_db7 = assign1720_e2010_d_b7;
        var_pwq_rv = 0.0;
        var_pwq_rdn0 = 0.0;
        var_pwq_rdn1 = 0.0;
        var_pwq_rdn2 = 0.0;
        var_pwq_rdn3 = 0.0;
        var_pwq_rdn4 = 0.0;
        var_pwq_rdn5 = 0.0;
        var_pwq_rdn6 = 0.0;
        var_pwq_rdn7 = 0.0;
        var_pwq_rdn8 = 0.0;
        var_pwq_rdn9 = 0.0;
        var_pwq_rdb0 = 0.0;
        var_pwq_rdb1 = 0.0;
        var_pwq_rdb2 = 0.0;
        var_pwq_rdb3 = 0.0;
        var_pwq_rdb4 = 0.0;
        var_pwq_rdb5 = 0.0;
        var_pwq_rdb6 = 0.0;
        var_pwq_rdb7 = 0.0;

        let (assign1730_e2030, assign1730_e2030_d_n0, assign1730_e2030_d_n1, assign1730_e2030_d_n2, assign1730_e2030_d_n3, assign1730_e2030_d_n4, assign1730_e2030_d_n5, assign1730_e2030_d_n6, assign1730_e2030_d_n7, assign1730_e2030_d_n8, assign1730_e2030_d_n9, assign1730_e2030_d_b0, assign1730_e2030_d_b1, assign1730_e2030_d_b2, assign1730_e2030_d_b3, assign1730_e2030_d_b4, assign1730_e2030_d_b5, assign1730_e2030_d_b6, assign1730_e2030_d_b7,) = {
    if (var_guard17 != 0.0) {
        let assign1730_e2017: f64 = (1.0 - p.p24);
        let assign1730_e2018: f64 = (var_pwq * assign1730_e2017);
        let assign1730_e2021: f64 = (1.0 - p.p24);
        let assign1730_e2022: f64 = (assign1730_e2018 * assign1730_e2021);
        let assign1730_e2023: f64 = (1.0 - assign1730_e2022);
        let assign1730_e2024: f64 = (var_vjc_t * assign1730_e2023);
        let assign1730_e2027: f64 = (1.0 - p.p71);
        let assign1730_e2028: f64 = (assign1730_e2024 / assign1730_e2027);
        (assign1730_e2028, (((var_vjc_t_dn0 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn0 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn1 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn1 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn2 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn2 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn3 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn3 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn4 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn4 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn5 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn5 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn6 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn6 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn7 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn7 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn8 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn8 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_dn9 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_dn9 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db0 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db0 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db1 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db1 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db2 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db2 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db3 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db3 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db4 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db4 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db5 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db5 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db6 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db6 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027), (((var_vjc_t_db7 * assign1730_e2023) + (var_vjc_t * (-((var_pwq_db7 * assign1730_e2017) * assign1730_e2021)))) / assign1730_e2027),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7,)
    }
};
        var_qlo = assign1730_e2030;
        var_qlo_dn0 = assign1730_e2030_d_n0;
        var_qlo_dn1 = assign1730_e2030_d_n1;
        var_qlo_dn2 = assign1730_e2030_d_n2;
        var_qlo_dn3 = assign1730_e2030_d_n3;
        var_qlo_dn4 = assign1730_e2030_d_n4;
        var_qlo_dn5 = assign1730_e2030_d_n5;
        var_qlo_dn6 = assign1730_e2030_d_n6;
        var_qlo_dn7 = assign1730_e2030_d_n7;
        var_qlo_dn8 = assign1730_e2030_d_n8;
        var_qlo_dn9 = assign1730_e2030_d_n9;
        var_qlo_db0 = assign1730_e2030_d_b0;
        var_qlo_db1 = assign1730_e2030_d_b1;
        var_qlo_db2 = assign1730_e2030_d_b2;
        var_qlo_db3 = assign1730_e2030_d_b3;
        var_qlo_db4 = assign1730_e2030_d_b4;
        var_qlo_db5 = assign1730_e2030_d_b5;
        var_qlo_db6 = assign1730_e2030_d_b6;
        var_qlo_db7 = assign1730_e2030_d_b7;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdn7 = 0.0;
        var_qlo_rdn8 = 0.0;
        var_qlo_rdn9 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;
        var_qlo_rdb7 = 0.0;

        let (assign1740_e2048, assign1740_e2048_d_n0, assign1740_e2048_d_n1, assign1740_e2048_d_n2, assign1740_e2048_d_n3, assign1740_e2048_d_n4, assign1740_e2048_d_n5, assign1740_e2048_d_n6, assign1740_e2048_d_n7, assign1740_e2048_d_n8, assign1740_e2048_d_n9, assign1740_e2048_d_b0, assign1740_e2048_d_b1, assign1740_e2048_d_b2, assign1740_e2048_d_b3, assign1740_e2048_d_b4, assign1740_e2048_d_b5, assign1740_e2048_d_b6, assign1740_e2048_d_b7,) = {
    if (var_guard17 != 0.0) {
        let assign1740_e2035: f64 = (1.0 - p.p24);
        let assign1740_e2038: f64 = (0.5 * p.p71);
        let assign1740_e2040: f64 = (assign1740_e2038 * var_dvh);
        let assign1740_e2042: f64 = (assign1740_e2040 / var_vjc_t);
        let assign1740_e2043: f64 = (assign1740_e2035 + assign1740_e2042);
        let assign1740_e2044: f64 = (var_dvh * assign1740_e2043);
        let assign1740_e2046: f64 = (assign1740_e2044 * var_pwq);
        (assign1740_e2046, ((((var_dvh_dn0 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn0) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn0)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn0)), ((((var_dvh_dn1 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn1) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn1)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn1)), ((((var_dvh_dn2 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn2) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn2)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn2)), ((((var_dvh_dn3 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn3) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn3)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn3)), ((((var_dvh_dn4 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn4) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn4)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn4)), ((((var_dvh_dn5 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn5) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn5)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn5)), ((((var_dvh_dn6 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn6) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn6)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn6)), ((((var_dvh_dn7 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn7) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn7)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn7)), ((((var_dvh_dn8 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn8) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn8)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn8)), ((((var_dvh_dn9 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_dn9) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_dn9)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_dn9)), ((((var_dvh_db0 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db0) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db0)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db0)), ((((var_dvh_db1 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db1) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db1)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db1)), ((((var_dvh_db2 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db2) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db2)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db2)), ((((var_dvh_db3 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db3) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db3)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db3)), ((((var_dvh_db4 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db4) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db4)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db4)), ((((var_dvh_db5 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db5) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db5)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db5)), ((((var_dvh_db6 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db6) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db6)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db6)), ((((var_dvh_db7 * assign1740_e2043) + (var_dvh * ((((assign1740_e2038 * var_dvh_db7) * var_vjc_t) - (assign1740_e2040 * var_vjc_t_db7)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1740_e2044 * var_pwq_db7)),)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_dn7, var_qhi_dn8, var_qhi_dn9, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6, var_qhi_db7,)
    }
};
        var_qhi = assign1740_e2048;
        var_qhi_dn0 = assign1740_e2048_d_n0;
        var_qhi_dn1 = assign1740_e2048_d_n1;
        var_qhi_dn2 = assign1740_e2048_d_n2;
        var_qhi_dn3 = assign1740_e2048_d_n3;
        var_qhi_dn4 = assign1740_e2048_d_n4;
        var_qhi_dn5 = assign1740_e2048_d_n5;
        var_qhi_dn6 = assign1740_e2048_d_n6;
        var_qhi_dn7 = assign1740_e2048_d_n7;
        var_qhi_dn8 = assign1740_e2048_d_n8;
        var_qhi_dn9 = assign1740_e2048_d_n9;
        var_qhi_db0 = assign1740_e2048_d_b0;
        var_qhi_db1 = assign1740_e2048_d_b1;
        var_qhi_db2 = assign1740_e2048_d_b2;
        var_qhi_db3 = assign1740_e2048_d_b3;
        var_qhi_db4 = assign1740_e2048_d_b4;
        var_qhi_db5 = assign1740_e2048_d_b5;
        var_qhi_db6 = assign1740_e2048_d_b6;
        var_qhi_db7 = assign1740_e2048_d_b7;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdn7 = 0.0;
        var_qhi_rdn8 = 0.0;
        var_qhi_rdn9 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;
        var_qhi_rdb7 = 0.0;

        let (assign1750_e2071, assign1750_e2071_d_n0, assign1750_e2071_d_n1, assign1750_e2071_d_n2, assign1750_e2071_d_n3, assign1750_e2071_d_n4, assign1750_e2071_d_n5, assign1750_e2071_d_n6, assign1750_e2071_d_n7, assign1750_e2071_d_n8, assign1750_e2071_d_n9, assign1750_e2071_d_b0, assign1750_e2071_d_b1, assign1750_e2071_d_b2, assign1750_e2071_d_b3, assign1750_e2071_d_b4, assign1750_e2071_d_b5, assign1750_e2071_d_b6, assign1750_e2071_d_b7,) = {
    if (var_guard17 == 0.0) {
        let assign1750_e2055: f64 = (1.0 - p.p71);
        let assign1750_e2059: f64 = (var_vbci / var_vjc_t);
        let assign1750_e2060: f64 = (1.0 - assign1750_e2059);
        let assign1750_e2061: f64 = (assign1750_e2060).ln();
        let assign1750_e2062: f64 = (assign1750_e2055 * assign1750_e2061);
        let assign1750_e2063: f64 = (assign1750_e2062).exp();
        let assign1750_e2064: f64 = (1.0 - assign1750_e2063);
        let assign1750_e2065: f64 = (var_vjc_t * assign1750_e2064);
        let assign1750_e2068: f64 = (1.0 - p.p71);
        let assign1750_e2069: f64 = (assign1750_e2065 / assign1750_e2068);
        (assign1750_e2069, (((var_vjc_t_dn0 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn0 * var_vjc_t) - (var_vbci * var_vjc_t_dn0)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn1 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn1 * var_vjc_t) - (var_vbci * var_vjc_t_dn1)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn2 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn2 * var_vjc_t) - (var_vbci * var_vjc_t_dn2)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn3 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn3 * var_vjc_t) - (var_vbci * var_vjc_t_dn3)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn4 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn4 * var_vjc_t) - (var_vbci * var_vjc_t_dn4)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn5 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn5 * var_vjc_t) - (var_vbci * var_vjc_t_dn5)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn6 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn6 * var_vjc_t) - (var_vbci * var_vjc_t_dn6)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn7 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn7 * var_vjc_t) - (var_vbci * var_vjc_t_dn7)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn8 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn8 * var_vjc_t) - (var_vbci * var_vjc_t_dn8)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_dn9 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_dn9 * var_vjc_t) - (var_vbci * var_vjc_t_dn9)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db0 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db0 * var_vjc_t) - (var_vbci * var_vjc_t_db0)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db1 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db1 * var_vjc_t) - (var_vbci * var_vjc_t_db1)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db2 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db2 * var_vjc_t) - (var_vbci * var_vjc_t_db2)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db3 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db3 * var_vjc_t) - (var_vbci * var_vjc_t_db3)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db4 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db4 * var_vjc_t) - (var_vbci * var_vjc_t_db4)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db5 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db5 * var_vjc_t) - (var_vbci * var_vjc_t_db5)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db6 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db6 * var_vjc_t) - (var_vbci * var_vjc_t_db6)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068), (((var_vjc_t_db7 * assign1750_e2064) + (var_vjc_t * (-(assign1750_e2063 * (assign1750_e2055 * ((-(((var_vbci_db7 * var_vjc_t) - (var_vbci * var_vjc_t_db7)) / (var_vjc_t * var_vjc_t))) / assign1750_e2060)))))) / assign1750_e2068),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7,)
    }
};
        var_qlo = assign1750_e2071;
        var_qlo_dn0 = assign1750_e2071_d_n0;
        var_qlo_dn1 = assign1750_e2071_d_n1;
        var_qlo_dn2 = assign1750_e2071_d_n2;
        var_qlo_dn3 = assign1750_e2071_d_n3;
        var_qlo_dn4 = assign1750_e2071_d_n4;
        var_qlo_dn5 = assign1750_e2071_d_n5;
        var_qlo_dn6 = assign1750_e2071_d_n6;
        var_qlo_dn7 = assign1750_e2071_d_n7;
        var_qlo_dn8 = assign1750_e2071_d_n8;
        var_qlo_dn9 = assign1750_e2071_d_n9;
        var_qlo_db0 = assign1750_e2071_d_b0;
        var_qlo_db1 = assign1750_e2071_d_b1;
        var_qlo_db2 = assign1750_e2071_d_b2;
        var_qlo_db3 = assign1750_e2071_d_b3;
        var_qlo_db4 = assign1750_e2071_d_b4;
        var_qlo_db5 = assign1750_e2071_d_b5;
        var_qlo_db6 = assign1750_e2071_d_b6;
        var_qlo_db7 = assign1750_e2071_d_b7;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdn7 = 0.0;
        var_qlo_rdn8 = 0.0;
        var_qlo_rdn9 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;
        var_qlo_rdb7 = 0.0;

        *var_dv0_slot = var_dv0;
        *var_dv0_db0_slot = var_dv0_db0;
        *var_dv0_db1_slot = var_dv0_db1;
        *var_dv0_db2_slot = var_dv0_db2;
        *var_dv0_db3_slot = var_dv0_db3;
        *var_dv0_db4_slot = var_dv0_db4;
        *var_dv0_db5_slot = var_dv0_db5;
        *var_dv0_db6_slot = var_dv0_db6;
        *var_dv0_db7_slot = var_dv0_db7;
        *var_dv0_dn0_slot = var_dv0_dn0;
        *var_dv0_dn1_slot = var_dv0_dn1;
        *var_dv0_dn2_slot = var_dv0_dn2;
        *var_dv0_dn3_slot = var_dv0_dn3;
        *var_dv0_dn4_slot = var_dv0_dn4;
        *var_dv0_dn5_slot = var_dv0_dn5;
        *var_dv0_dn6_slot = var_dv0_dn6;
        *var_dv0_dn7_slot = var_dv0_dn7;
        *var_dv0_dn8_slot = var_dv0_dn8;
        *var_dv0_dn9_slot = var_dv0_dn9;
        *var_dv0_rdb0_slot = var_dv0_rdb0;
        *var_dv0_rdb1_slot = var_dv0_rdb1;
        *var_dv0_rdb2_slot = var_dv0_rdb2;
        *var_dv0_rdb3_slot = var_dv0_rdb3;
        *var_dv0_rdb4_slot = var_dv0_rdb4;
        *var_dv0_rdb5_slot = var_dv0_rdb5;
        *var_dv0_rdb6_slot = var_dv0_rdb6;
        *var_dv0_rdb7_slot = var_dv0_rdb7;
        *var_dv0_rdn0_slot = var_dv0_rdn0;
        *var_dv0_rdn1_slot = var_dv0_rdn1;
        *var_dv0_rdn2_slot = var_dv0_rdn2;
        *var_dv0_rdn3_slot = var_dv0_rdn3;
        *var_dv0_rdn4_slot = var_dv0_rdn4;
        *var_dv0_rdn5_slot = var_dv0_rdn5;
        *var_dv0_rdn6_slot = var_dv0_rdn6;
        *var_dv0_rdn7_slot = var_dv0_rdn7;
        *var_dv0_rdn8_slot = var_dv0_rdn8;
        *var_dv0_rdn9_slot = var_dv0_rdn9;
        *var_dv0_rv_slot = var_dv0_rv;
        *var_dvh_slot = var_dvh;
        *var_dvh_db0_slot = var_dvh_db0;
        *var_dvh_db1_slot = var_dvh_db1;
        *var_dvh_db2_slot = var_dvh_db2;
        *var_dvh_db3_slot = var_dvh_db3;
        *var_dvh_db4_slot = var_dvh_db4;
        *var_dvh_db5_slot = var_dvh_db5;
        *var_dvh_db6_slot = var_dvh_db6;
        *var_dvh_db7_slot = var_dvh_db7;
        *var_dvh_dn0_slot = var_dvh_dn0;
        *var_dvh_dn1_slot = var_dvh_dn1;
        *var_dvh_dn2_slot = var_dvh_dn2;
        *var_dvh_dn3_slot = var_dvh_dn3;
        *var_dvh_dn4_slot = var_dvh_dn4;
        *var_dvh_dn5_slot = var_dvh_dn5;
        *var_dvh_dn6_slot = var_dvh_dn6;
        *var_dvh_dn7_slot = var_dvh_dn7;
        *var_dvh_dn8_slot = var_dvh_dn8;
        *var_dvh_dn9_slot = var_dvh_dn9;
        *var_dvh_rdb0_slot = var_dvh_rdb0;
        *var_dvh_rdb1_slot = var_dvh_rdb1;
        *var_dvh_rdb2_slot = var_dvh_rdb2;
        *var_dvh_rdb3_slot = var_dvh_rdb3;
        *var_dvh_rdb4_slot = var_dvh_rdb4;
        *var_dvh_rdb5_slot = var_dvh_rdb5;
        *var_dvh_rdb6_slot = var_dvh_rdb6;
        *var_dvh_rdb7_slot = var_dvh_rdb7;
        *var_dvh_rdn0_slot = var_dvh_rdn0;
        *var_dvh_rdn1_slot = var_dvh_rdn1;
        *var_dvh_rdn2_slot = var_dvh_rdn2;
        *var_dvh_rdn3_slot = var_dvh_rdn3;
        *var_dvh_rdn4_slot = var_dvh_rdn4;
        *var_dvh_rdn5_slot = var_dvh_rdn5;
        *var_dvh_rdn6_slot = var_dvh_rdn6;
        *var_dvh_rdn7_slot = var_dvh_rdn7;
        *var_dvh_rdn8_slot = var_dvh_rdn8;
        *var_dvh_rdn9_slot = var_dvh_rdn9;
        *var_dvh_rv_slot = var_dvh_rv;
        *var_guard17_slot = var_guard17;
        *var_guard17_db0_slot = var_guard17_db0;
        *var_guard17_db1_slot = var_guard17_db1;
        *var_guard17_db2_slot = var_guard17_db2;
        *var_guard17_db3_slot = var_guard17_db3;
        *var_guard17_db4_slot = var_guard17_db4;
        *var_guard17_db5_slot = var_guard17_db5;
        *var_guard17_db6_slot = var_guard17_db6;
        *var_guard17_db7_slot = var_guard17_db7;
        *var_guard17_dn0_slot = var_guard17_dn0;
        *var_guard17_dn1_slot = var_guard17_dn1;
        *var_guard17_dn2_slot = var_guard17_dn2;
        *var_guard17_dn3_slot = var_guard17_dn3;
        *var_guard17_dn4_slot = var_guard17_dn4;
        *var_guard17_dn5_slot = var_guard17_dn5;
        *var_guard17_dn6_slot = var_guard17_dn6;
        *var_guard17_dn7_slot = var_guard17_dn7;
        *var_guard17_dn8_slot = var_guard17_dn8;
        *var_guard17_dn9_slot = var_guard17_dn9;
        *var_guard17_rdb0_slot = var_guard17_rdb0;
        *var_guard17_rdb1_slot = var_guard17_rdb1;
        *var_guard17_rdb2_slot = var_guard17_rdb2;
        *var_guard17_rdb3_slot = var_guard17_rdb3;
        *var_guard17_rdb4_slot = var_guard17_rdb4;
        *var_guard17_rdb5_slot = var_guard17_rdb5;
        *var_guard17_rdb6_slot = var_guard17_rdb6;
        *var_guard17_rdb7_slot = var_guard17_rdb7;
        *var_guard17_rdn0_slot = var_guard17_rdn0;
        *var_guard17_rdn1_slot = var_guard17_rdn1;
        *var_guard17_rdn2_slot = var_guard17_rdn2;
        *var_guard17_rdn3_slot = var_guard17_rdn3;
        *var_guard17_rdn4_slot = var_guard17_rdn4;
        *var_guard17_rdn5_slot = var_guard17_rdn5;
        *var_guard17_rdn6_slot = var_guard17_rdn6;
        *var_guard17_rdn7_slot = var_guard17_rdn7;
        *var_guard17_rdn8_slot = var_guard17_rdn8;
        *var_guard17_rdn9_slot = var_guard17_rdn9;
        *var_guard17_rv_slot = var_guard17_rv;
        *var_pwq_slot = var_pwq;
        *var_pwq_db0_slot = var_pwq_db0;
        *var_pwq_db1_slot = var_pwq_db1;
        *var_pwq_db2_slot = var_pwq_db2;
        *var_pwq_db3_slot = var_pwq_db3;
        *var_pwq_db4_slot = var_pwq_db4;
        *var_pwq_db5_slot = var_pwq_db5;
        *var_pwq_db6_slot = var_pwq_db6;
        *var_pwq_db7_slot = var_pwq_db7;
        *var_pwq_dn0_slot = var_pwq_dn0;
        *var_pwq_dn1_slot = var_pwq_dn1;
        *var_pwq_dn2_slot = var_pwq_dn2;
        *var_pwq_dn3_slot = var_pwq_dn3;
        *var_pwq_dn4_slot = var_pwq_dn4;
        *var_pwq_dn5_slot = var_pwq_dn5;
        *var_pwq_dn6_slot = var_pwq_dn6;
        *var_pwq_dn7_slot = var_pwq_dn7;
        *var_pwq_dn8_slot = var_pwq_dn8;
        *var_pwq_dn9_slot = var_pwq_dn9;
        *var_pwq_rdb0_slot = var_pwq_rdb0;
        *var_pwq_rdb1_slot = var_pwq_rdb1;
        *var_pwq_rdb2_slot = var_pwq_rdb2;
        *var_pwq_rdb3_slot = var_pwq_rdb3;
        *var_pwq_rdb4_slot = var_pwq_rdb4;
        *var_pwq_rdb5_slot = var_pwq_rdb5;
        *var_pwq_rdb6_slot = var_pwq_rdb6;
        *var_pwq_rdb7_slot = var_pwq_rdb7;
        *var_pwq_rdn0_slot = var_pwq_rdn0;
        *var_pwq_rdn1_slot = var_pwq_rdn1;
        *var_pwq_rdn2_slot = var_pwq_rdn2;
        *var_pwq_rdn3_slot = var_pwq_rdn3;
        *var_pwq_rdn4_slot = var_pwq_rdn4;
        *var_pwq_rdn5_slot = var_pwq_rdn5;
        *var_pwq_rdn6_slot = var_pwq_rdn6;
        *var_pwq_rdn7_slot = var_pwq_rdn7;
        *var_pwq_rdn8_slot = var_pwq_rdn8;
        *var_pwq_rdn9_slot = var_pwq_rdn9;
        *var_pwq_rv_slot = var_pwq_rv;
        *var_qhi_slot = var_qhi;
        *var_qhi_db0_slot = var_qhi_db0;
        *var_qhi_db1_slot = var_qhi_db1;
        *var_qhi_db2_slot = var_qhi_db2;
        *var_qhi_db3_slot = var_qhi_db3;
        *var_qhi_db4_slot = var_qhi_db4;
        *var_qhi_db5_slot = var_qhi_db5;
        *var_qhi_db6_slot = var_qhi_db6;
        *var_qhi_db7_slot = var_qhi_db7;
        *var_qhi_dn0_slot = var_qhi_dn0;
        *var_qhi_dn1_slot = var_qhi_dn1;
        *var_qhi_dn2_slot = var_qhi_dn2;
        *var_qhi_dn3_slot = var_qhi_dn3;
        *var_qhi_dn4_slot = var_qhi_dn4;
        *var_qhi_dn5_slot = var_qhi_dn5;
        *var_qhi_dn6_slot = var_qhi_dn6;
        *var_qhi_dn7_slot = var_qhi_dn7;
        *var_qhi_dn8_slot = var_qhi_dn8;
        *var_qhi_dn9_slot = var_qhi_dn9;
        *var_qhi_rdb0_slot = var_qhi_rdb0;
        *var_qhi_rdb1_slot = var_qhi_rdb1;
        *var_qhi_rdb2_slot = var_qhi_rdb2;
        *var_qhi_rdb3_slot = var_qhi_rdb3;
        *var_qhi_rdb4_slot = var_qhi_rdb4;
        *var_qhi_rdb5_slot = var_qhi_rdb5;
        *var_qhi_rdb6_slot = var_qhi_rdb6;
        *var_qhi_rdb7_slot = var_qhi_rdb7;
        *var_qhi_rdn0_slot = var_qhi_rdn0;
        *var_qhi_rdn1_slot = var_qhi_rdn1;
        *var_qhi_rdn2_slot = var_qhi_rdn2;
        *var_qhi_rdn3_slot = var_qhi_rdn3;
        *var_qhi_rdn4_slot = var_qhi_rdn4;
        *var_qhi_rdn5_slot = var_qhi_rdn5;
        *var_qhi_rdn6_slot = var_qhi_rdn6;
        *var_qhi_rdn7_slot = var_qhi_rdn7;
        *var_qhi_rdn8_slot = var_qhi_rdn8;
        *var_qhi_rdn9_slot = var_qhi_rdn9;
        *var_qhi_rv_slot = var_qhi_rv;
        *var_qje_slot = var_qje;
        *var_qje_db0_slot = var_qje_db0;
        *var_qje_db1_slot = var_qje_db1;
        *var_qje_db2_slot = var_qje_db2;
        *var_qje_db3_slot = var_qje_db3;
        *var_qje_db4_slot = var_qje_db4;
        *var_qje_db5_slot = var_qje_db5;
        *var_qje_db6_slot = var_qje_db6;
        *var_qje_db7_slot = var_qje_db7;
        *var_qje_dn0_slot = var_qje_dn0;
        *var_qje_dn1_slot = var_qje_dn1;
        *var_qje_dn2_slot = var_qje_dn2;
        *var_qje_dn3_slot = var_qje_dn3;
        *var_qje_dn4_slot = var_qje_dn4;
        *var_qje_dn5_slot = var_qje_dn5;
        *var_qje_dn6_slot = var_qje_dn6;
        *var_qje_dn7_slot = var_qje_dn7;
        *var_qje_dn8_slot = var_qje_dn8;
        *var_qje_dn9_slot = var_qje_dn9;
        *var_qje_rdb0_slot = var_qje_rdb0;
        *var_qje_rdb1_slot = var_qje_rdb1;
        *var_qje_rdb2_slot = var_qje_rdb2;
        *var_qje_rdb3_slot = var_qje_rdb3;
        *var_qje_rdb4_slot = var_qje_rdb4;
        *var_qje_rdb5_slot = var_qje_rdb5;
        *var_qje_rdb6_slot = var_qje_rdb6;
        *var_qje_rdb7_slot = var_qje_rdb7;
        *var_qje_rdn0_slot = var_qje_rdn0;
        *var_qje_rdn1_slot = var_qje_rdn1;
        *var_qje_rdn2_slot = var_qje_rdn2;
        *var_qje_rdn3_slot = var_qje_rdn3;
        *var_qje_rdn4_slot = var_qje_rdn4;
        *var_qje_rdn5_slot = var_qje_rdn5;
        *var_qje_rdn6_slot = var_qje_rdn6;
        *var_qje_rdn7_slot = var_qje_rdn7;
        *var_qje_rdn8_slot = var_qje_rdn8;
        *var_qje_rdn9_slot = var_qje_rdn9;
        *var_qje_rv_slot = var_qje_rv;
        *var_qlo_slot = var_qlo;
        *var_qlo_db0_slot = var_qlo_db0;
        *var_qlo_db1_slot = var_qlo_db1;
        *var_qlo_db2_slot = var_qlo_db2;
        *var_qlo_db3_slot = var_qlo_db3;
        *var_qlo_db4_slot = var_qlo_db4;
        *var_qlo_db5_slot = var_qlo_db5;
        *var_qlo_db6_slot = var_qlo_db6;
        *var_qlo_db7_slot = var_qlo_db7;
        *var_qlo_dn0_slot = var_qlo_dn0;
        *var_qlo_dn1_slot = var_qlo_dn1;
        *var_qlo_dn2_slot = var_qlo_dn2;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_qlo_dn5_slot = var_qlo_dn5;
        *var_qlo_dn6_slot = var_qlo_dn6;
        *var_qlo_dn7_slot = var_qlo_dn7;
        *var_qlo_dn8_slot = var_qlo_dn8;
        *var_qlo_dn9_slot = var_qlo_dn9;
        *var_qlo_rdb0_slot = var_qlo_rdb0;
        *var_qlo_rdb1_slot = var_qlo_rdb1;
        *var_qlo_rdb2_slot = var_qlo_rdb2;
        *var_qlo_rdb3_slot = var_qlo_rdb3;
        *var_qlo_rdb4_slot = var_qlo_rdb4;
        *var_qlo_rdb5_slot = var_qlo_rdb5;
        *var_qlo_rdb6_slot = var_qlo_rdb6;
        *var_qlo_rdb7_slot = var_qlo_rdb7;
        *var_qlo_rdn0_slot = var_qlo_rdn0;
        *var_qlo_rdn1_slot = var_qlo_rdn1;
        *var_qlo_rdn2_slot = var_qlo_rdn2;
        *var_qlo_rdn3_slot = var_qlo_rdn3;
        *var_qlo_rdn4_slot = var_qlo_rdn4;
        *var_qlo_rdn5_slot = var_qlo_rdn5;
        *var_qlo_rdn6_slot = var_qlo_rdn6;
        *var_qlo_rdn7_slot = var_qlo_rdn7;
        *var_qlo_rdn8_slot = var_qlo_rdn8;
        *var_qlo_rdn9_slot = var_qlo_rdn9;
        *var_qlo_rv_slot = var_qlo_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        var_cjc_t: f64,
        var_cjc_t_db0: f64,
        var_cjc_t_db1: f64,
        var_cjc_t_db2: f64,
        var_cjc_t_db3: f64,
        var_cjc_t_db4: f64,
        var_cjc_t_db5: f64,
        var_cjc_t_db6: f64,
        var_cjc_t_db7: f64,
        var_cjc_t_dn0: f64,
        var_cjc_t_dn1: f64,
        var_cjc_t_dn2: f64,
        var_cjc_t_dn3: f64,
        var_cjc_t_dn4: f64,
        var_cjc_t_dn5: f64,
        var_cjc_t_dn6: f64,
        var_cjc_t_dn7: f64,
        var_cjc_t_dn8: f64,
        var_cjc_t_dn9: f64,
        var_guard17: f64,
        var_vbici: f64,
        var_vbici_db0: f64,
        var_vbici_db1: f64,
        var_vbici_db2: f64,
        var_vbici_db3: f64,
        var_vbici_db4: f64,
        var_vbici_db5: f64,
        var_vbici_db6: f64,
        var_vbici_db7: f64,
        var_vbici_dn0: f64,
        var_vbici_dn1: f64,
        var_vbici_dn2: f64,
        var_vbici_dn3: f64,
        var_vbici_dn4: f64,
        var_vbici_dn5: f64,
        var_vbici_dn6: f64,
        var_vbici_dn7: f64,
        var_vbici_dn8: f64,
        var_vbici_dn9: f64,
        var_vjc_t: f64,
        var_vjc_t_db0: f64,
        var_vjc_t_db1: f64,
        var_vjc_t_db2: f64,
        var_vjc_t_db3: f64,
        var_vjc_t_db4: f64,
        var_vjc_t_db5: f64,
        var_vjc_t_db6: f64,
        var_vjc_t_db7: f64,
        var_vjc_t_dn0: f64,
        var_vjc_t_dn1: f64,
        var_vjc_t_dn2: f64,
        var_vjc_t_dn3: f64,
        var_vjc_t_dn4: f64,
        var_vjc_t_dn5: f64,
        var_vjc_t_dn6: f64,
        var_vjc_t_dn7: f64,
        var_vjc_t_dn8: f64,
        var_vjc_t_dn9: f64,
        var_dv0_slot: &mut f64,
        var_dv0_db0_slot: &mut f64,
        var_dv0_db1_slot: &mut f64,
        var_dv0_db2_slot: &mut f64,
        var_dv0_db3_slot: &mut f64,
        var_dv0_db4_slot: &mut f64,
        var_dv0_db5_slot: &mut f64,
        var_dv0_db6_slot: &mut f64,
        var_dv0_db7_slot: &mut f64,
        var_dv0_dn0_slot: &mut f64,
        var_dv0_dn1_slot: &mut f64,
        var_dv0_dn2_slot: &mut f64,
        var_dv0_dn3_slot: &mut f64,
        var_dv0_dn4_slot: &mut f64,
        var_dv0_dn5_slot: &mut f64,
        var_dv0_dn6_slot: &mut f64,
        var_dv0_dn7_slot: &mut f64,
        var_dv0_dn8_slot: &mut f64,
        var_dv0_dn9_slot: &mut f64,
        var_dv0_rdb0_slot: &mut f64,
        var_dv0_rdb1_slot: &mut f64,
        var_dv0_rdb2_slot: &mut f64,
        var_dv0_rdb3_slot: &mut f64,
        var_dv0_rdb4_slot: &mut f64,
        var_dv0_rdb5_slot: &mut f64,
        var_dv0_rdb6_slot: &mut f64,
        var_dv0_rdb7_slot: &mut f64,
        var_dv0_rdn0_slot: &mut f64,
        var_dv0_rdn1_slot: &mut f64,
        var_dv0_rdn2_slot: &mut f64,
        var_dv0_rdn3_slot: &mut f64,
        var_dv0_rdn4_slot: &mut f64,
        var_dv0_rdn5_slot: &mut f64,
        var_dv0_rdn6_slot: &mut f64,
        var_dv0_rdn7_slot: &mut f64,
        var_dv0_rdn8_slot: &mut f64,
        var_dv0_rdn9_slot: &mut f64,
        var_dv0_rv_slot: &mut f64,
        var_dvh_slot: &mut f64,
        var_dvh_db0_slot: &mut f64,
        var_dvh_db1_slot: &mut f64,
        var_dvh_db2_slot: &mut f64,
        var_dvh_db3_slot: &mut f64,
        var_dvh_db4_slot: &mut f64,
        var_dvh_db5_slot: &mut f64,
        var_dvh_db6_slot: &mut f64,
        var_dvh_db7_slot: &mut f64,
        var_dvh_dn0_slot: &mut f64,
        var_dvh_dn1_slot: &mut f64,
        var_dvh_dn2_slot: &mut f64,
        var_dvh_dn3_slot: &mut f64,
        var_dvh_dn4_slot: &mut f64,
        var_dvh_dn5_slot: &mut f64,
        var_dvh_dn6_slot: &mut f64,
        var_dvh_dn7_slot: &mut f64,
        var_dvh_dn8_slot: &mut f64,
        var_dvh_dn9_slot: &mut f64,
        var_dvh_rdb0_slot: &mut f64,
        var_dvh_rdb1_slot: &mut f64,
        var_dvh_rdb2_slot: &mut f64,
        var_dvh_rdb3_slot: &mut f64,
        var_dvh_rdb4_slot: &mut f64,
        var_dvh_rdb5_slot: &mut f64,
        var_dvh_rdb6_slot: &mut f64,
        var_dvh_rdb7_slot: &mut f64,
        var_dvh_rdn0_slot: &mut f64,
        var_dvh_rdn1_slot: &mut f64,
        var_dvh_rdn2_slot: &mut f64,
        var_dvh_rdn3_slot: &mut f64,
        var_dvh_rdn4_slot: &mut f64,
        var_dvh_rdn5_slot: &mut f64,
        var_dvh_rdn6_slot: &mut f64,
        var_dvh_rdn7_slot: &mut f64,
        var_dvh_rdn8_slot: &mut f64,
        var_dvh_rdn9_slot: &mut f64,
        var_dvh_rv_slot: &mut f64,
        var_guard18_slot: &mut f64,
        var_guard18_db0_slot: &mut f64,
        var_guard18_db1_slot: &mut f64,
        var_guard18_db2_slot: &mut f64,
        var_guard18_db3_slot: &mut f64,
        var_guard18_db4_slot: &mut f64,
        var_guard18_db5_slot: &mut f64,
        var_guard18_db6_slot: &mut f64,
        var_guard18_db7_slot: &mut f64,
        var_guard18_dn0_slot: &mut f64,
        var_guard18_dn1_slot: &mut f64,
        var_guard18_dn2_slot: &mut f64,
        var_guard18_dn3_slot: &mut f64,
        var_guard18_dn4_slot: &mut f64,
        var_guard18_dn5_slot: &mut f64,
        var_guard18_dn6_slot: &mut f64,
        var_guard18_dn7_slot: &mut f64,
        var_guard18_dn8_slot: &mut f64,
        var_guard18_dn9_slot: &mut f64,
        var_guard18_rdb0_slot: &mut f64,
        var_guard18_rdb1_slot: &mut f64,
        var_guard18_rdb2_slot: &mut f64,
        var_guard18_rdb3_slot: &mut f64,
        var_guard18_rdb4_slot: &mut f64,
        var_guard18_rdb5_slot: &mut f64,
        var_guard18_rdb6_slot: &mut f64,
        var_guard18_rdb7_slot: &mut f64,
        var_guard18_rdn0_slot: &mut f64,
        var_guard18_rdn1_slot: &mut f64,
        var_guard18_rdn2_slot: &mut f64,
        var_guard18_rdn3_slot: &mut f64,
        var_guard18_rdn4_slot: &mut f64,
        var_guard18_rdn5_slot: &mut f64,
        var_guard18_rdn6_slot: &mut f64,
        var_guard18_rdn7_slot: &mut f64,
        var_guard18_rdn8_slot: &mut f64,
        var_guard18_rdn9_slot: &mut f64,
        var_guard18_rv_slot: &mut f64,
        var_pwq_slot: &mut f64,
        var_pwq_db0_slot: &mut f64,
        var_pwq_db1_slot: &mut f64,
        var_pwq_db2_slot: &mut f64,
        var_pwq_db3_slot: &mut f64,
        var_pwq_db4_slot: &mut f64,
        var_pwq_db5_slot: &mut f64,
        var_pwq_db6_slot: &mut f64,
        var_pwq_db7_slot: &mut f64,
        var_pwq_dn0_slot: &mut f64,
        var_pwq_dn1_slot: &mut f64,
        var_pwq_dn2_slot: &mut f64,
        var_pwq_dn3_slot: &mut f64,
        var_pwq_dn4_slot: &mut f64,
        var_pwq_dn5_slot: &mut f64,
        var_pwq_dn6_slot: &mut f64,
        var_pwq_dn7_slot: &mut f64,
        var_pwq_dn8_slot: &mut f64,
        var_pwq_dn9_slot: &mut f64,
        var_pwq_rdb0_slot: &mut f64,
        var_pwq_rdb1_slot: &mut f64,
        var_pwq_rdb2_slot: &mut f64,
        var_pwq_rdb3_slot: &mut f64,
        var_pwq_rdb4_slot: &mut f64,
        var_pwq_rdb5_slot: &mut f64,
        var_pwq_rdb6_slot: &mut f64,
        var_pwq_rdb7_slot: &mut f64,
        var_pwq_rdn0_slot: &mut f64,
        var_pwq_rdn1_slot: &mut f64,
        var_pwq_rdn2_slot: &mut f64,
        var_pwq_rdn3_slot: &mut f64,
        var_pwq_rdn4_slot: &mut f64,
        var_pwq_rdn5_slot: &mut f64,
        var_pwq_rdn6_slot: &mut f64,
        var_pwq_rdn7_slot: &mut f64,
        var_pwq_rdn8_slot: &mut f64,
        var_pwq_rdn9_slot: &mut f64,
        var_pwq_rv_slot: &mut f64,
        var_qhi_slot: &mut f64,
        var_qhi_db0_slot: &mut f64,
        var_qhi_db1_slot: &mut f64,
        var_qhi_db2_slot: &mut f64,
        var_qhi_db3_slot: &mut f64,
        var_qhi_db4_slot: &mut f64,
        var_qhi_db5_slot: &mut f64,
        var_qhi_db6_slot: &mut f64,
        var_qhi_db7_slot: &mut f64,
        var_qhi_dn0_slot: &mut f64,
        var_qhi_dn1_slot: &mut f64,
        var_qhi_dn2_slot: &mut f64,
        var_qhi_dn3_slot: &mut f64,
        var_qhi_dn4_slot: &mut f64,
        var_qhi_dn5_slot: &mut f64,
        var_qhi_dn6_slot: &mut f64,
        var_qhi_dn7_slot: &mut f64,
        var_qhi_dn8_slot: &mut f64,
        var_qhi_dn9_slot: &mut f64,
        var_qhi_rdb0_slot: &mut f64,
        var_qhi_rdb1_slot: &mut f64,
        var_qhi_rdb2_slot: &mut f64,
        var_qhi_rdb3_slot: &mut f64,
        var_qhi_rdb4_slot: &mut f64,
        var_qhi_rdb5_slot: &mut f64,
        var_qhi_rdb6_slot: &mut f64,
        var_qhi_rdb7_slot: &mut f64,
        var_qhi_rdn0_slot: &mut f64,
        var_qhi_rdn1_slot: &mut f64,
        var_qhi_rdn2_slot: &mut f64,
        var_qhi_rdn3_slot: &mut f64,
        var_qhi_rdn4_slot: &mut f64,
        var_qhi_rdn5_slot: &mut f64,
        var_qhi_rdn6_slot: &mut f64,
        var_qhi_rdn7_slot: &mut f64,
        var_qhi_rdn8_slot: &mut f64,
        var_qhi_rdn9_slot: &mut f64,
        var_qhi_rv_slot: &mut f64,
        var_qjcx_slot: &mut f64,
        var_qjcx_1_slot: &mut f64,
        var_qjcx_1_db0_slot: &mut f64,
        var_qjcx_1_db1_slot: &mut f64,
        var_qjcx_1_db2_slot: &mut f64,
        var_qjcx_1_db3_slot: &mut f64,
        var_qjcx_1_db4_slot: &mut f64,
        var_qjcx_1_db5_slot: &mut f64,
        var_qjcx_1_db6_slot: &mut f64,
        var_qjcx_1_db7_slot: &mut f64,
        var_qjcx_1_dn0_slot: &mut f64,
        var_qjcx_1_dn1_slot: &mut f64,
        var_qjcx_1_dn2_slot: &mut f64,
        var_qjcx_1_dn3_slot: &mut f64,
        var_qjcx_1_dn4_slot: &mut f64,
        var_qjcx_1_dn5_slot: &mut f64,
        var_qjcx_1_dn6_slot: &mut f64,
        var_qjcx_1_dn7_slot: &mut f64,
        var_qjcx_1_dn8_slot: &mut f64,
        var_qjcx_1_dn9_slot: &mut f64,
        var_qjcx_1_rdb0_slot: &mut f64,
        var_qjcx_1_rdb1_slot: &mut f64,
        var_qjcx_1_rdb2_slot: &mut f64,
        var_qjcx_1_rdb3_slot: &mut f64,
        var_qjcx_1_rdb4_slot: &mut f64,
        var_qjcx_1_rdb5_slot: &mut f64,
        var_qjcx_1_rdb6_slot: &mut f64,
        var_qjcx_1_rdb7_slot: &mut f64,
        var_qjcx_1_rdn0_slot: &mut f64,
        var_qjcx_1_rdn1_slot: &mut f64,
        var_qjcx_1_rdn2_slot: &mut f64,
        var_qjcx_1_rdn3_slot: &mut f64,
        var_qjcx_1_rdn4_slot: &mut f64,
        var_qjcx_1_rdn5_slot: &mut f64,
        var_qjcx_1_rdn6_slot: &mut f64,
        var_qjcx_1_rdn7_slot: &mut f64,
        var_qjcx_1_rdn8_slot: &mut f64,
        var_qjcx_1_rdn9_slot: &mut f64,
        var_qjcx_1_rv_slot: &mut f64,
        var_qjcx_db0_slot: &mut f64,
        var_qjcx_db1_slot: &mut f64,
        var_qjcx_db2_slot: &mut f64,
        var_qjcx_db3_slot: &mut f64,
        var_qjcx_db4_slot: &mut f64,
        var_qjcx_db5_slot: &mut f64,
        var_qjcx_db6_slot: &mut f64,
        var_qjcx_db7_slot: &mut f64,
        var_qjcx_dn0_slot: &mut f64,
        var_qjcx_dn1_slot: &mut f64,
        var_qjcx_dn2_slot: &mut f64,
        var_qjcx_dn3_slot: &mut f64,
        var_qjcx_dn4_slot: &mut f64,
        var_qjcx_dn5_slot: &mut f64,
        var_qjcx_dn6_slot: &mut f64,
        var_qjcx_dn7_slot: &mut f64,
        var_qjcx_dn8_slot: &mut f64,
        var_qjcx_dn9_slot: &mut f64,
        var_qjcx_rdb0_slot: &mut f64,
        var_qjcx_rdb1_slot: &mut f64,
        var_qjcx_rdb2_slot: &mut f64,
        var_qjcx_rdb3_slot: &mut f64,
        var_qjcx_rdb4_slot: &mut f64,
        var_qjcx_rdb5_slot: &mut f64,
        var_qjcx_rdb6_slot: &mut f64,
        var_qjcx_rdb7_slot: &mut f64,
        var_qjcx_rdn0_slot: &mut f64,
        var_qjcx_rdn1_slot: &mut f64,
        var_qjcx_rdn2_slot: &mut f64,
        var_qjcx_rdn3_slot: &mut f64,
        var_qjcx_rdn4_slot: &mut f64,
        var_qjcx_rdn5_slot: &mut f64,
        var_qjcx_rdn6_slot: &mut f64,
        var_qjcx_rdn7_slot: &mut f64,
        var_qjcx_rdn8_slot: &mut f64,
        var_qjcx_rdn9_slot: &mut f64,
        var_qjcx_rv_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo_db0_slot: &mut f64,
        var_qlo_db1_slot: &mut f64,
        var_qlo_db2_slot: &mut f64,
        var_qlo_db3_slot: &mut f64,
        var_qlo_db4_slot: &mut f64,
        var_qlo_db5_slot: &mut f64,
        var_qlo_db6_slot: &mut f64,
        var_qlo_db7_slot: &mut f64,
        var_qlo_dn0_slot: &mut f64,
        var_qlo_dn1_slot: &mut f64,
        var_qlo_dn2_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_qlo_dn5_slot: &mut f64,
        var_qlo_dn6_slot: &mut f64,
        var_qlo_dn7_slot: &mut f64,
        var_qlo_dn8_slot: &mut f64,
        var_qlo_dn9_slot: &mut f64,
        var_qlo_rdb0_slot: &mut f64,
        var_qlo_rdb1_slot: &mut f64,
        var_qlo_rdb2_slot: &mut f64,
        var_qlo_rdb3_slot: &mut f64,
        var_qlo_rdb4_slot: &mut f64,
        var_qlo_rdb5_slot: &mut f64,
        var_qlo_rdb6_slot: &mut f64,
        var_qlo_rdb7_slot: &mut f64,
        var_qlo_rdn0_slot: &mut f64,
        var_qlo_rdn1_slot: &mut f64,
        var_qlo_rdn2_slot: &mut f64,
        var_qlo_rdn3_slot: &mut f64,
        var_qlo_rdn4_slot: &mut f64,
        var_qlo_rdn5_slot: &mut f64,
        var_qlo_rdn6_slot: &mut f64,
        var_qlo_rdn7_slot: &mut f64,
        var_qlo_rdn8_slot: &mut f64,
        var_qlo_rdn9_slot: &mut f64,
        var_qlo_rv_slot: &mut f64,
    ) {
        let mut var_dv0: f64 = *var_dv0_slot;
        let mut var_dv0_db0: f64 = *var_dv0_db0_slot;
        let mut var_dv0_db1: f64 = *var_dv0_db1_slot;
        let mut var_dv0_db2: f64 = *var_dv0_db2_slot;
        let mut var_dv0_db3: f64 = *var_dv0_db3_slot;
        let mut var_dv0_db4: f64 = *var_dv0_db4_slot;
        let mut var_dv0_db5: f64 = *var_dv0_db5_slot;
        let mut var_dv0_db6: f64 = *var_dv0_db6_slot;
        let mut var_dv0_db7: f64 = *var_dv0_db7_slot;
        let mut var_dv0_dn0: f64 = *var_dv0_dn0_slot;
        let mut var_dv0_dn1: f64 = *var_dv0_dn1_slot;
        let mut var_dv0_dn2: f64 = *var_dv0_dn2_slot;
        let mut var_dv0_dn3: f64 = *var_dv0_dn3_slot;
        let mut var_dv0_dn4: f64 = *var_dv0_dn4_slot;
        let mut var_dv0_dn5: f64 = *var_dv0_dn5_slot;
        let mut var_dv0_dn6: f64 = *var_dv0_dn6_slot;
        let mut var_dv0_dn7: f64 = *var_dv0_dn7_slot;
        let mut var_dv0_dn8: f64 = *var_dv0_dn8_slot;
        let mut var_dv0_dn9: f64 = *var_dv0_dn9_slot;
        let mut var_dv0_rdb0: f64 = *var_dv0_rdb0_slot;
        let mut var_dv0_rdb1: f64 = *var_dv0_rdb1_slot;
        let mut var_dv0_rdb2: f64 = *var_dv0_rdb2_slot;
        let mut var_dv0_rdb3: f64 = *var_dv0_rdb3_slot;
        let mut var_dv0_rdb4: f64 = *var_dv0_rdb4_slot;
        let mut var_dv0_rdb5: f64 = *var_dv0_rdb5_slot;
        let mut var_dv0_rdb6: f64 = *var_dv0_rdb6_slot;
        let mut var_dv0_rdb7: f64 = *var_dv0_rdb7_slot;
        let mut var_dv0_rdn0: f64 = *var_dv0_rdn0_slot;
        let mut var_dv0_rdn1: f64 = *var_dv0_rdn1_slot;
        let mut var_dv0_rdn2: f64 = *var_dv0_rdn2_slot;
        let mut var_dv0_rdn3: f64 = *var_dv0_rdn3_slot;
        let mut var_dv0_rdn4: f64 = *var_dv0_rdn4_slot;
        let mut var_dv0_rdn5: f64 = *var_dv0_rdn5_slot;
        let mut var_dv0_rdn6: f64 = *var_dv0_rdn6_slot;
        let mut var_dv0_rdn7: f64 = *var_dv0_rdn7_slot;
        let mut var_dv0_rdn8: f64 = *var_dv0_rdn8_slot;
        let mut var_dv0_rdn9: f64 = *var_dv0_rdn9_slot;
        let mut var_dv0_rv: f64 = *var_dv0_rv_slot;
        let mut var_dvh: f64 = *var_dvh_slot;
        let mut var_dvh_db0: f64 = *var_dvh_db0_slot;
        let mut var_dvh_db1: f64 = *var_dvh_db1_slot;
        let mut var_dvh_db2: f64 = *var_dvh_db2_slot;
        let mut var_dvh_db3: f64 = *var_dvh_db3_slot;
        let mut var_dvh_db4: f64 = *var_dvh_db4_slot;
        let mut var_dvh_db5: f64 = *var_dvh_db5_slot;
        let mut var_dvh_db6: f64 = *var_dvh_db6_slot;
        let mut var_dvh_db7: f64 = *var_dvh_db7_slot;
        let mut var_dvh_dn0: f64 = *var_dvh_dn0_slot;
        let mut var_dvh_dn1: f64 = *var_dvh_dn1_slot;
        let mut var_dvh_dn2: f64 = *var_dvh_dn2_slot;
        let mut var_dvh_dn3: f64 = *var_dvh_dn3_slot;
        let mut var_dvh_dn4: f64 = *var_dvh_dn4_slot;
        let mut var_dvh_dn5: f64 = *var_dvh_dn5_slot;
        let mut var_dvh_dn6: f64 = *var_dvh_dn6_slot;
        let mut var_dvh_dn7: f64 = *var_dvh_dn7_slot;
        let mut var_dvh_dn8: f64 = *var_dvh_dn8_slot;
        let mut var_dvh_dn9: f64 = *var_dvh_dn9_slot;
        let mut var_dvh_rdb0: f64 = *var_dvh_rdb0_slot;
        let mut var_dvh_rdb1: f64 = *var_dvh_rdb1_slot;
        let mut var_dvh_rdb2: f64 = *var_dvh_rdb2_slot;
        let mut var_dvh_rdb3: f64 = *var_dvh_rdb3_slot;
        let mut var_dvh_rdb4: f64 = *var_dvh_rdb4_slot;
        let mut var_dvh_rdb5: f64 = *var_dvh_rdb5_slot;
        let mut var_dvh_rdb6: f64 = *var_dvh_rdb6_slot;
        let mut var_dvh_rdb7: f64 = *var_dvh_rdb7_slot;
        let mut var_dvh_rdn0: f64 = *var_dvh_rdn0_slot;
        let mut var_dvh_rdn1: f64 = *var_dvh_rdn1_slot;
        let mut var_dvh_rdn2: f64 = *var_dvh_rdn2_slot;
        let mut var_dvh_rdn3: f64 = *var_dvh_rdn3_slot;
        let mut var_dvh_rdn4: f64 = *var_dvh_rdn4_slot;
        let mut var_dvh_rdn5: f64 = *var_dvh_rdn5_slot;
        let mut var_dvh_rdn6: f64 = *var_dvh_rdn6_slot;
        let mut var_dvh_rdn7: f64 = *var_dvh_rdn7_slot;
        let mut var_dvh_rdn8: f64 = *var_dvh_rdn8_slot;
        let mut var_dvh_rdn9: f64 = *var_dvh_rdn9_slot;
        let mut var_dvh_rv: f64 = *var_dvh_rv_slot;
        let mut var_guard18: f64 = *var_guard18_slot;
        let mut var_guard18_db0: f64 = *var_guard18_db0_slot;
        let mut var_guard18_db1: f64 = *var_guard18_db1_slot;
        let mut var_guard18_db2: f64 = *var_guard18_db2_slot;
        let mut var_guard18_db3: f64 = *var_guard18_db3_slot;
        let mut var_guard18_db4: f64 = *var_guard18_db4_slot;
        let mut var_guard18_db5: f64 = *var_guard18_db5_slot;
        let mut var_guard18_db6: f64 = *var_guard18_db6_slot;
        let mut var_guard18_db7: f64 = *var_guard18_db7_slot;
        let mut var_guard18_dn0: f64 = *var_guard18_dn0_slot;
        let mut var_guard18_dn1: f64 = *var_guard18_dn1_slot;
        let mut var_guard18_dn2: f64 = *var_guard18_dn2_slot;
        let mut var_guard18_dn3: f64 = *var_guard18_dn3_slot;
        let mut var_guard18_dn4: f64 = *var_guard18_dn4_slot;
        let mut var_guard18_dn5: f64 = *var_guard18_dn5_slot;
        let mut var_guard18_dn6: f64 = *var_guard18_dn6_slot;
        let mut var_guard18_dn7: f64 = *var_guard18_dn7_slot;
        let mut var_guard18_dn8: f64 = *var_guard18_dn8_slot;
        let mut var_guard18_dn9: f64 = *var_guard18_dn9_slot;
        let mut var_guard18_rdb0: f64 = *var_guard18_rdb0_slot;
        let mut var_guard18_rdb1: f64 = *var_guard18_rdb1_slot;
        let mut var_guard18_rdb2: f64 = *var_guard18_rdb2_slot;
        let mut var_guard18_rdb3: f64 = *var_guard18_rdb3_slot;
        let mut var_guard18_rdb4: f64 = *var_guard18_rdb4_slot;
        let mut var_guard18_rdb5: f64 = *var_guard18_rdb5_slot;
        let mut var_guard18_rdb6: f64 = *var_guard18_rdb6_slot;
        let mut var_guard18_rdb7: f64 = *var_guard18_rdb7_slot;
        let mut var_guard18_rdn0: f64 = *var_guard18_rdn0_slot;
        let mut var_guard18_rdn1: f64 = *var_guard18_rdn1_slot;
        let mut var_guard18_rdn2: f64 = *var_guard18_rdn2_slot;
        let mut var_guard18_rdn3: f64 = *var_guard18_rdn3_slot;
        let mut var_guard18_rdn4: f64 = *var_guard18_rdn4_slot;
        let mut var_guard18_rdn5: f64 = *var_guard18_rdn5_slot;
        let mut var_guard18_rdn6: f64 = *var_guard18_rdn6_slot;
        let mut var_guard18_rdn7: f64 = *var_guard18_rdn7_slot;
        let mut var_guard18_rdn8: f64 = *var_guard18_rdn8_slot;
        let mut var_guard18_rdn9: f64 = *var_guard18_rdn9_slot;
        let mut var_guard18_rv: f64 = *var_guard18_rv_slot;
        let mut var_pwq: f64 = *var_pwq_slot;
        let mut var_pwq_db0: f64 = *var_pwq_db0_slot;
        let mut var_pwq_db1: f64 = *var_pwq_db1_slot;
        let mut var_pwq_db2: f64 = *var_pwq_db2_slot;
        let mut var_pwq_db3: f64 = *var_pwq_db3_slot;
        let mut var_pwq_db4: f64 = *var_pwq_db4_slot;
        let mut var_pwq_db5: f64 = *var_pwq_db5_slot;
        let mut var_pwq_db6: f64 = *var_pwq_db6_slot;
        let mut var_pwq_db7: f64 = *var_pwq_db7_slot;
        let mut var_pwq_dn0: f64 = *var_pwq_dn0_slot;
        let mut var_pwq_dn1: f64 = *var_pwq_dn1_slot;
        let mut var_pwq_dn2: f64 = *var_pwq_dn2_slot;
        let mut var_pwq_dn3: f64 = *var_pwq_dn3_slot;
        let mut var_pwq_dn4: f64 = *var_pwq_dn4_slot;
        let mut var_pwq_dn5: f64 = *var_pwq_dn5_slot;
        let mut var_pwq_dn6: f64 = *var_pwq_dn6_slot;
        let mut var_pwq_dn7: f64 = *var_pwq_dn7_slot;
        let mut var_pwq_dn8: f64 = *var_pwq_dn8_slot;
        let mut var_pwq_dn9: f64 = *var_pwq_dn9_slot;
        let mut var_pwq_rdb0: f64 = *var_pwq_rdb0_slot;
        let mut var_pwq_rdb1: f64 = *var_pwq_rdb1_slot;
        let mut var_pwq_rdb2: f64 = *var_pwq_rdb2_slot;
        let mut var_pwq_rdb3: f64 = *var_pwq_rdb3_slot;
        let mut var_pwq_rdb4: f64 = *var_pwq_rdb4_slot;
        let mut var_pwq_rdb5: f64 = *var_pwq_rdb5_slot;
        let mut var_pwq_rdb6: f64 = *var_pwq_rdb6_slot;
        let mut var_pwq_rdb7: f64 = *var_pwq_rdb7_slot;
        let mut var_pwq_rdn0: f64 = *var_pwq_rdn0_slot;
        let mut var_pwq_rdn1: f64 = *var_pwq_rdn1_slot;
        let mut var_pwq_rdn2: f64 = *var_pwq_rdn2_slot;
        let mut var_pwq_rdn3: f64 = *var_pwq_rdn3_slot;
        let mut var_pwq_rdn4: f64 = *var_pwq_rdn4_slot;
        let mut var_pwq_rdn5: f64 = *var_pwq_rdn5_slot;
        let mut var_pwq_rdn6: f64 = *var_pwq_rdn6_slot;
        let mut var_pwq_rdn7: f64 = *var_pwq_rdn7_slot;
        let mut var_pwq_rdn8: f64 = *var_pwq_rdn8_slot;
        let mut var_pwq_rdn9: f64 = *var_pwq_rdn9_slot;
        let mut var_pwq_rv: f64 = *var_pwq_rv_slot;
        let mut var_qhi: f64 = *var_qhi_slot;
        let mut var_qhi_db0: f64 = *var_qhi_db0_slot;
        let mut var_qhi_db1: f64 = *var_qhi_db1_slot;
        let mut var_qhi_db2: f64 = *var_qhi_db2_slot;
        let mut var_qhi_db3: f64 = *var_qhi_db3_slot;
        let mut var_qhi_db4: f64 = *var_qhi_db4_slot;
        let mut var_qhi_db5: f64 = *var_qhi_db5_slot;
        let mut var_qhi_db6: f64 = *var_qhi_db6_slot;
        let mut var_qhi_db7: f64 = *var_qhi_db7_slot;
        let mut var_qhi_dn0: f64 = *var_qhi_dn0_slot;
        let mut var_qhi_dn1: f64 = *var_qhi_dn1_slot;
        let mut var_qhi_dn2: f64 = *var_qhi_dn2_slot;
        let mut var_qhi_dn3: f64 = *var_qhi_dn3_slot;
        let mut var_qhi_dn4: f64 = *var_qhi_dn4_slot;
        let mut var_qhi_dn5: f64 = *var_qhi_dn5_slot;
        let mut var_qhi_dn6: f64 = *var_qhi_dn6_slot;
        let mut var_qhi_dn7: f64 = *var_qhi_dn7_slot;
        let mut var_qhi_dn8: f64 = *var_qhi_dn8_slot;
        let mut var_qhi_dn9: f64 = *var_qhi_dn9_slot;
        let mut var_qhi_rdb0: f64 = *var_qhi_rdb0_slot;
        let mut var_qhi_rdb1: f64 = *var_qhi_rdb1_slot;
        let mut var_qhi_rdb2: f64 = *var_qhi_rdb2_slot;
        let mut var_qhi_rdb3: f64 = *var_qhi_rdb3_slot;
        let mut var_qhi_rdb4: f64 = *var_qhi_rdb4_slot;
        let mut var_qhi_rdb5: f64 = *var_qhi_rdb5_slot;
        let mut var_qhi_rdb6: f64 = *var_qhi_rdb6_slot;
        let mut var_qhi_rdb7: f64 = *var_qhi_rdb7_slot;
        let mut var_qhi_rdn0: f64 = *var_qhi_rdn0_slot;
        let mut var_qhi_rdn1: f64 = *var_qhi_rdn1_slot;
        let mut var_qhi_rdn2: f64 = *var_qhi_rdn2_slot;
        let mut var_qhi_rdn3: f64 = *var_qhi_rdn3_slot;
        let mut var_qhi_rdn4: f64 = *var_qhi_rdn4_slot;
        let mut var_qhi_rdn5: f64 = *var_qhi_rdn5_slot;
        let mut var_qhi_rdn6: f64 = *var_qhi_rdn6_slot;
        let mut var_qhi_rdn7: f64 = *var_qhi_rdn7_slot;
        let mut var_qhi_rdn8: f64 = *var_qhi_rdn8_slot;
        let mut var_qhi_rdn9: f64 = *var_qhi_rdn9_slot;
        let mut var_qhi_rv: f64 = *var_qhi_rv_slot;
        let mut var_qjcx: f64 = *var_qjcx_slot;
        let mut var_qjcx_1: f64 = *var_qjcx_1_slot;
        let mut var_qjcx_1_db0: f64 = *var_qjcx_1_db0_slot;
        let mut var_qjcx_1_db1: f64 = *var_qjcx_1_db1_slot;
        let mut var_qjcx_1_db2: f64 = *var_qjcx_1_db2_slot;
        let mut var_qjcx_1_db3: f64 = *var_qjcx_1_db3_slot;
        let mut var_qjcx_1_db4: f64 = *var_qjcx_1_db4_slot;
        let mut var_qjcx_1_db5: f64 = *var_qjcx_1_db5_slot;
        let mut var_qjcx_1_db6: f64 = *var_qjcx_1_db6_slot;
        let mut var_qjcx_1_db7: f64 = *var_qjcx_1_db7_slot;
        let mut var_qjcx_1_dn0: f64 = *var_qjcx_1_dn0_slot;
        let mut var_qjcx_1_dn1: f64 = *var_qjcx_1_dn1_slot;
        let mut var_qjcx_1_dn2: f64 = *var_qjcx_1_dn2_slot;
        let mut var_qjcx_1_dn3: f64 = *var_qjcx_1_dn3_slot;
        let mut var_qjcx_1_dn4: f64 = *var_qjcx_1_dn4_slot;
        let mut var_qjcx_1_dn5: f64 = *var_qjcx_1_dn5_slot;
        let mut var_qjcx_1_dn6: f64 = *var_qjcx_1_dn6_slot;
        let mut var_qjcx_1_dn7: f64 = *var_qjcx_1_dn7_slot;
        let mut var_qjcx_1_dn8: f64 = *var_qjcx_1_dn8_slot;
        let mut var_qjcx_1_dn9: f64 = *var_qjcx_1_dn9_slot;
        let mut var_qjcx_1_rdb0: f64 = *var_qjcx_1_rdb0_slot;
        let mut var_qjcx_1_rdb1: f64 = *var_qjcx_1_rdb1_slot;
        let mut var_qjcx_1_rdb2: f64 = *var_qjcx_1_rdb2_slot;
        let mut var_qjcx_1_rdb3: f64 = *var_qjcx_1_rdb3_slot;
        let mut var_qjcx_1_rdb4: f64 = *var_qjcx_1_rdb4_slot;
        let mut var_qjcx_1_rdb5: f64 = *var_qjcx_1_rdb5_slot;
        let mut var_qjcx_1_rdb6: f64 = *var_qjcx_1_rdb6_slot;
        let mut var_qjcx_1_rdb7: f64 = *var_qjcx_1_rdb7_slot;
        let mut var_qjcx_1_rdn0: f64 = *var_qjcx_1_rdn0_slot;
        let mut var_qjcx_1_rdn1: f64 = *var_qjcx_1_rdn1_slot;
        let mut var_qjcx_1_rdn2: f64 = *var_qjcx_1_rdn2_slot;
        let mut var_qjcx_1_rdn3: f64 = *var_qjcx_1_rdn3_slot;
        let mut var_qjcx_1_rdn4: f64 = *var_qjcx_1_rdn4_slot;
        let mut var_qjcx_1_rdn5: f64 = *var_qjcx_1_rdn5_slot;
        let mut var_qjcx_1_rdn6: f64 = *var_qjcx_1_rdn6_slot;
        let mut var_qjcx_1_rdn7: f64 = *var_qjcx_1_rdn7_slot;
        let mut var_qjcx_1_rdn8: f64 = *var_qjcx_1_rdn8_slot;
        let mut var_qjcx_1_rdn9: f64 = *var_qjcx_1_rdn9_slot;
        let mut var_qjcx_1_rv: f64 = *var_qjcx_1_rv_slot;
        let mut var_qjcx_db0: f64 = *var_qjcx_db0_slot;
        let mut var_qjcx_db1: f64 = *var_qjcx_db1_slot;
        let mut var_qjcx_db2: f64 = *var_qjcx_db2_slot;
        let mut var_qjcx_db3: f64 = *var_qjcx_db3_slot;
        let mut var_qjcx_db4: f64 = *var_qjcx_db4_slot;
        let mut var_qjcx_db5: f64 = *var_qjcx_db5_slot;
        let mut var_qjcx_db6: f64 = *var_qjcx_db6_slot;
        let mut var_qjcx_db7: f64 = *var_qjcx_db7_slot;
        let mut var_qjcx_dn0: f64 = *var_qjcx_dn0_slot;
        let mut var_qjcx_dn1: f64 = *var_qjcx_dn1_slot;
        let mut var_qjcx_dn2: f64 = *var_qjcx_dn2_slot;
        let mut var_qjcx_dn3: f64 = *var_qjcx_dn3_slot;
        let mut var_qjcx_dn4: f64 = *var_qjcx_dn4_slot;
        let mut var_qjcx_dn5: f64 = *var_qjcx_dn5_slot;
        let mut var_qjcx_dn6: f64 = *var_qjcx_dn6_slot;
        let mut var_qjcx_dn7: f64 = *var_qjcx_dn7_slot;
        let mut var_qjcx_dn8: f64 = *var_qjcx_dn8_slot;
        let mut var_qjcx_dn9: f64 = *var_qjcx_dn9_slot;
        let mut var_qjcx_rdb0: f64 = *var_qjcx_rdb0_slot;
        let mut var_qjcx_rdb1: f64 = *var_qjcx_rdb1_slot;
        let mut var_qjcx_rdb2: f64 = *var_qjcx_rdb2_slot;
        let mut var_qjcx_rdb3: f64 = *var_qjcx_rdb3_slot;
        let mut var_qjcx_rdb4: f64 = *var_qjcx_rdb4_slot;
        let mut var_qjcx_rdb5: f64 = *var_qjcx_rdb5_slot;
        let mut var_qjcx_rdb6: f64 = *var_qjcx_rdb6_slot;
        let mut var_qjcx_rdb7: f64 = *var_qjcx_rdb7_slot;
        let mut var_qjcx_rdn0: f64 = *var_qjcx_rdn0_slot;
        let mut var_qjcx_rdn1: f64 = *var_qjcx_rdn1_slot;
        let mut var_qjcx_rdn2: f64 = *var_qjcx_rdn2_slot;
        let mut var_qjcx_rdn3: f64 = *var_qjcx_rdn3_slot;
        let mut var_qjcx_rdn4: f64 = *var_qjcx_rdn4_slot;
        let mut var_qjcx_rdn5: f64 = *var_qjcx_rdn5_slot;
        let mut var_qjcx_rdn6: f64 = *var_qjcx_rdn6_slot;
        let mut var_qjcx_rdn7: f64 = *var_qjcx_rdn7_slot;
        let mut var_qjcx_rdn8: f64 = *var_qjcx_rdn8_slot;
        let mut var_qjcx_rdn9: f64 = *var_qjcx_rdn9_slot;
        let mut var_qjcx_rv: f64 = *var_qjcx_rv_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo_db0: f64 = *var_qlo_db0_slot;
        let mut var_qlo_db1: f64 = *var_qlo_db1_slot;
        let mut var_qlo_db2: f64 = *var_qlo_db2_slot;
        let mut var_qlo_db3: f64 = *var_qlo_db3_slot;
        let mut var_qlo_db4: f64 = *var_qlo_db4_slot;
        let mut var_qlo_db5: f64 = *var_qlo_db5_slot;
        let mut var_qlo_db6: f64 = *var_qlo_db6_slot;
        let mut var_qlo_db7: f64 = *var_qlo_db7_slot;
        let mut var_qlo_dn0: f64 = *var_qlo_dn0_slot;
        let mut var_qlo_dn1: f64 = *var_qlo_dn1_slot;
        let mut var_qlo_dn2: f64 = *var_qlo_dn2_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_qlo_dn5: f64 = *var_qlo_dn5_slot;
        let mut var_qlo_dn6: f64 = *var_qlo_dn6_slot;
        let mut var_qlo_dn7: f64 = *var_qlo_dn7_slot;
        let mut var_qlo_dn8: f64 = *var_qlo_dn8_slot;
        let mut var_qlo_dn9: f64 = *var_qlo_dn9_slot;
        let mut var_qlo_rdb0: f64 = *var_qlo_rdb0_slot;
        let mut var_qlo_rdb1: f64 = *var_qlo_rdb1_slot;
        let mut var_qlo_rdb2: f64 = *var_qlo_rdb2_slot;
        let mut var_qlo_rdb3: f64 = *var_qlo_rdb3_slot;
        let mut var_qlo_rdb4: f64 = *var_qlo_rdb4_slot;
        let mut var_qlo_rdb5: f64 = *var_qlo_rdb5_slot;
        let mut var_qlo_rdb6: f64 = *var_qlo_rdb6_slot;
        let mut var_qlo_rdb7: f64 = *var_qlo_rdb7_slot;
        let mut var_qlo_rdn0: f64 = *var_qlo_rdn0_slot;
        let mut var_qlo_rdn1: f64 = *var_qlo_rdn1_slot;
        let mut var_qlo_rdn2: f64 = *var_qlo_rdn2_slot;
        let mut var_qlo_rdn3: f64 = *var_qlo_rdn3_slot;
        let mut var_qlo_rdn4: f64 = *var_qlo_rdn4_slot;
        let mut var_qlo_rdn5: f64 = *var_qlo_rdn5_slot;
        let mut var_qlo_rdn6: f64 = *var_qlo_rdn6_slot;
        let mut var_qlo_rdn7: f64 = *var_qlo_rdn7_slot;
        let mut var_qlo_rdn8: f64 = *var_qlo_rdn8_slot;
        let mut var_qlo_rdn9: f64 = *var_qlo_rdn9_slot;
        let mut var_qlo_rv: f64 = *var_qlo_rv_slot;

        let (assign1760_e2076, assign1760_e2076_d_n0, assign1760_e2076_d_n1, assign1760_e2076_d_n2, assign1760_e2076_d_n3, assign1760_e2076_d_n4, assign1760_e2076_d_n5, assign1760_e2076_d_n6, assign1760_e2076_d_n7, assign1760_e2076_d_n8, assign1760_e2076_d_n9, assign1760_e2076_d_b0, assign1760_e2076_d_b1, assign1760_e2076_d_b2, assign1760_e2076_d_b3, assign1760_e2076_d_b4, assign1760_e2076_d_b5, assign1760_e2076_d_b6, assign1760_e2076_d_b7,) = {
    if (var_guard17 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_dn7, var_qhi_dn8, var_qhi_dn9, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6, var_qhi_db7,)
    }
};
        var_qhi = assign1760_e2076;
        var_qhi_dn0 = assign1760_e2076_d_n0;
        var_qhi_dn1 = assign1760_e2076_d_n1;
        var_qhi_dn2 = assign1760_e2076_d_n2;
        var_qhi_dn3 = assign1760_e2076_d_n3;
        var_qhi_dn4 = assign1760_e2076_d_n4;
        var_qhi_dn5 = assign1760_e2076_d_n5;
        var_qhi_dn6 = assign1760_e2076_d_n6;
        var_qhi_dn7 = assign1760_e2076_d_n7;
        var_qhi_dn8 = assign1760_e2076_d_n8;
        var_qhi_dn9 = assign1760_e2076_d_n9;
        var_qhi_db0 = assign1760_e2076_d_b0;
        var_qhi_db1 = assign1760_e2076_d_b1;
        var_qhi_db2 = assign1760_e2076_d_b2;
        var_qhi_db3 = assign1760_e2076_d_b3;
        var_qhi_db4 = assign1760_e2076_d_b4;
        var_qhi_db5 = assign1760_e2076_d_b5;
        var_qhi_db6 = assign1760_e2076_d_b6;
        var_qhi_db7 = assign1760_e2076_d_b7;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdn7 = 0.0;
        var_qhi_rdn8 = 0.0;
        var_qhi_rdn9 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;
        var_qhi_rdb7 = 0.0;

        let assign1770_e2080: f64 = (var_qlo + var_qhi);
        let assign1770_e2081: f64 = (var_cjc_t * assign1770_e2080);
        var_qjcx = assign1770_e2081;
        var_qjcx_dn0 = ((var_cjc_t_dn0 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn0 + var_qhi_dn0)));
        var_qjcx_dn1 = ((var_cjc_t_dn1 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn1 + var_qhi_dn1)));
        var_qjcx_dn2 = ((var_cjc_t_dn2 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn2 + var_qhi_dn2)));
        var_qjcx_dn3 = ((var_cjc_t_dn3 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qjcx_dn4 = ((var_cjc_t_dn4 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn4 + var_qhi_dn4)));
        var_qjcx_dn5 = ((var_cjc_t_dn5 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn5 + var_qhi_dn5)));
        var_qjcx_dn6 = ((var_cjc_t_dn6 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn6 + var_qhi_dn6)));
        var_qjcx_dn7 = ((var_cjc_t_dn7 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn7 + var_qhi_dn7)));
        var_qjcx_dn8 = ((var_cjc_t_dn8 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn8 + var_qhi_dn8)));
        var_qjcx_dn9 = ((var_cjc_t_dn9 * assign1770_e2080) + (var_cjc_t * (var_qlo_dn9 + var_qhi_dn9)));
        var_qjcx_db0 = ((var_cjc_t_db0 * assign1770_e2080) + (var_cjc_t * (var_qlo_db0 + var_qhi_db0)));
        var_qjcx_db1 = ((var_cjc_t_db1 * assign1770_e2080) + (var_cjc_t * (var_qlo_db1 + var_qhi_db1)));
        var_qjcx_db2 = ((var_cjc_t_db2 * assign1770_e2080) + (var_cjc_t * (var_qlo_db2 + var_qhi_db2)));
        var_qjcx_db3 = ((var_cjc_t_db3 * assign1770_e2080) + (var_cjc_t * (var_qlo_db3 + var_qhi_db3)));
        var_qjcx_db4 = ((var_cjc_t_db4 * assign1770_e2080) + (var_cjc_t * (var_qlo_db4 + var_qhi_db4)));
        var_qjcx_db5 = ((var_cjc_t_db5 * assign1770_e2080) + (var_cjc_t * (var_qlo_db5 + var_qhi_db5)));
        var_qjcx_db6 = ((var_cjc_t_db6 * assign1770_e2080) + (var_cjc_t * (var_qlo_db6 + var_qhi_db6)));
        var_qjcx_db7 = ((var_cjc_t_db7 * assign1770_e2080) + (var_cjc_t * (var_qlo_db7 + var_qhi_db7)));
        var_qjcx_rv = 0.0;
        var_qjcx_rdn0 = 0.0;
        var_qjcx_rdn1 = 0.0;
        var_qjcx_rdn2 = 0.0;
        var_qjcx_rdn3 = 0.0;
        var_qjcx_rdn4 = 0.0;
        var_qjcx_rdn5 = 0.0;
        var_qjcx_rdn6 = 0.0;
        var_qjcx_rdn7 = 0.0;
        var_qjcx_rdn8 = 0.0;
        var_qjcx_rdn9 = 0.0;
        var_qjcx_rdb0 = 0.0;
        var_qjcx_rdb1 = 0.0;
        var_qjcx_rdb2 = 0.0;
        var_qjcx_rdb3 = 0.0;
        var_qjcx_rdb4 = 0.0;
        var_qjcx_rdb5 = 0.0;
        var_qjcx_rdb6 = 0.0;
        var_qjcx_rdb7 = 0.0;

        let assign1780_e2084: f64 = (1.0 - p.p72);
        let assign1780_e2086: f64 = (assign1780_e2084 * var_qjcx);
        var_qjcx_1 = assign1780_e2086;
        var_qjcx_1_dn0 = (assign1780_e2084 * var_qjcx_dn0);
        var_qjcx_1_dn1 = (assign1780_e2084 * var_qjcx_dn1);
        var_qjcx_1_dn2 = (assign1780_e2084 * var_qjcx_dn2);
        var_qjcx_1_dn3 = (assign1780_e2084 * var_qjcx_dn3);
        var_qjcx_1_dn4 = (assign1780_e2084 * var_qjcx_dn4);
        var_qjcx_1_dn5 = (assign1780_e2084 * var_qjcx_dn5);
        var_qjcx_1_dn6 = (assign1780_e2084 * var_qjcx_dn6);
        var_qjcx_1_dn7 = (assign1780_e2084 * var_qjcx_dn7);
        var_qjcx_1_dn8 = (assign1780_e2084 * var_qjcx_dn8);
        var_qjcx_1_dn9 = (assign1780_e2084 * var_qjcx_dn9);
        var_qjcx_1_db0 = (assign1780_e2084 * var_qjcx_db0);
        var_qjcx_1_db1 = (assign1780_e2084 * var_qjcx_db1);
        var_qjcx_1_db2 = (assign1780_e2084 * var_qjcx_db2);
        var_qjcx_1_db3 = (assign1780_e2084 * var_qjcx_db3);
        var_qjcx_1_db4 = (assign1780_e2084 * var_qjcx_db4);
        var_qjcx_1_db5 = (assign1780_e2084 * var_qjcx_db5);
        var_qjcx_1_db6 = (assign1780_e2084 * var_qjcx_db6);
        var_qjcx_1_db7 = (assign1780_e2084 * var_qjcx_db7);
        var_qjcx_1_rv = 0.0;
        var_qjcx_1_rdn0 = 0.0;
        var_qjcx_1_rdn1 = 0.0;
        var_qjcx_1_rdn2 = 0.0;
        var_qjcx_1_rdn3 = 0.0;
        var_qjcx_1_rdn4 = 0.0;
        var_qjcx_1_rdn5 = 0.0;
        var_qjcx_1_rdn6 = 0.0;
        var_qjcx_1_rdn7 = 0.0;
        var_qjcx_1_rdn8 = 0.0;
        var_qjcx_1_rdn9 = 0.0;
        var_qjcx_1_rdb0 = 0.0;
        var_qjcx_1_rdb1 = 0.0;
        var_qjcx_1_rdb2 = 0.0;
        var_qjcx_1_rdb3 = 0.0;
        var_qjcx_1_rdb4 = 0.0;
        var_qjcx_1_rdb5 = 0.0;
        var_qjcx_1_rdb6 = 0.0;
        var_qjcx_1_rdb7 = 0.0;

        let assign1790_e2088: f64 = (-var_vjc_t);
        let assign1790_e2090: f64 = (assign1790_e2088 * p.p24);
        var_dv0 = assign1790_e2090;
        var_dv0_dn0 = ((-var_vjc_t_dn0) * p.p24);
        var_dv0_dn1 = ((-var_vjc_t_dn1) * p.p24);
        var_dv0_dn2 = ((-var_vjc_t_dn2) * p.p24);
        var_dv0_dn3 = ((-var_vjc_t_dn3) * p.p24);
        var_dv0_dn4 = ((-var_vjc_t_dn4) * p.p24);
        var_dv0_dn5 = ((-var_vjc_t_dn5) * p.p24);
        var_dv0_dn6 = ((-var_vjc_t_dn6) * p.p24);
        var_dv0_dn7 = ((-var_vjc_t_dn7) * p.p24);
        var_dv0_dn8 = ((-var_vjc_t_dn8) * p.p24);
        var_dv0_dn9 = ((-var_vjc_t_dn9) * p.p24);
        var_dv0_db0 = ((-var_vjc_t_db0) * p.p24);
        var_dv0_db1 = ((-var_vjc_t_db1) * p.p24);
        var_dv0_db2 = ((-var_vjc_t_db2) * p.p24);
        var_dv0_db3 = ((-var_vjc_t_db3) * p.p24);
        var_dv0_db4 = ((-var_vjc_t_db4) * p.p24);
        var_dv0_db5 = ((-var_vjc_t_db5) * p.p24);
        var_dv0_db6 = ((-var_vjc_t_db6) * p.p24);
        var_dv0_db7 = ((-var_vjc_t_db7) * p.p24);
        var_dv0_rv = 0.0;
        var_dv0_rdn0 = 0.0;
        var_dv0_rdn1 = 0.0;
        var_dv0_rdn2 = 0.0;
        var_dv0_rdn3 = 0.0;
        var_dv0_rdn4 = 0.0;
        var_dv0_rdn5 = 0.0;
        var_dv0_rdn6 = 0.0;
        var_dv0_rdn7 = 0.0;
        var_dv0_rdn8 = 0.0;
        var_dv0_rdn9 = 0.0;
        var_dv0_rdb0 = 0.0;
        var_dv0_rdb1 = 0.0;
        var_dv0_rdb2 = 0.0;
        var_dv0_rdb3 = 0.0;
        var_dv0_rdb4 = 0.0;
        var_dv0_rdb5 = 0.0;
        var_dv0_rdb6 = 0.0;
        var_dv0_rdb7 = 0.0;

        let assign1800_e2093: f64 = (var_vbici + var_dv0);
        var_dvh = assign1800_e2093;
        var_dvh_dn0 = (var_vbici_dn0 + var_dv0_dn0);
        var_dvh_dn1 = (var_vbici_dn1 + var_dv0_dn1);
        var_dvh_dn2 = (var_vbici_dn2 + var_dv0_dn2);
        var_dvh_dn3 = (var_vbici_dn3 + var_dv0_dn3);
        var_dvh_dn4 = (var_vbici_dn4 + var_dv0_dn4);
        var_dvh_dn5 = (var_vbici_dn5 + var_dv0_dn5);
        var_dvh_dn6 = (var_vbici_dn6 + var_dv0_dn6);
        var_dvh_dn7 = (var_vbici_dn7 + var_dv0_dn7);
        var_dvh_dn8 = (var_vbici_dn8 + var_dv0_dn8);
        var_dvh_dn9 = (var_vbici_dn9 + var_dv0_dn9);
        var_dvh_db0 = (var_vbici_db0 + var_dv0_db0);
        var_dvh_db1 = (var_vbici_db1 + var_dv0_db1);
        var_dvh_db2 = (var_vbici_db2 + var_dv0_db2);
        var_dvh_db3 = (var_vbici_db3 + var_dv0_db3);
        var_dvh_db4 = (var_vbici_db4 + var_dv0_db4);
        var_dvh_db5 = (var_vbici_db5 + var_dv0_db5);
        var_dvh_db6 = (var_vbici_db6 + var_dv0_db6);
        var_dvh_db7 = (var_vbici_db7 + var_dv0_db7);
        var_dvh_rv = 0.0;
        var_dvh_rdn0 = 0.0;
        var_dvh_rdn1 = 0.0;
        var_dvh_rdn2 = 0.0;
        var_dvh_rdn3 = 0.0;
        var_dvh_rdn4 = 0.0;
        var_dvh_rdn5 = 0.0;
        var_dvh_rdn6 = 0.0;
        var_dvh_rdn7 = 0.0;
        var_dvh_rdn8 = 0.0;
        var_dvh_rdn9 = 0.0;
        var_dvh_rdb0 = 0.0;
        var_dvh_rdb1 = 0.0;
        var_dvh_rdb2 = 0.0;
        var_dvh_rdb3 = 0.0;
        var_dvh_rdb4 = 0.0;
        var_dvh_rdb5 = 0.0;
        var_dvh_rdb6 = 0.0;
        var_dvh_rdb7 = 0.0;

        let assign1810_e2096: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard18 = assign1810_e2096;
        var_guard18_dn0 = 0.0;
        var_guard18_dn1 = 0.0;
        var_guard18_dn2 = 0.0;
        var_guard18_dn3 = 0.0;
        var_guard18_dn4 = 0.0;
        var_guard18_dn5 = 0.0;
        var_guard18_dn6 = 0.0;
        var_guard18_dn7 = 0.0;
        var_guard18_dn8 = 0.0;
        var_guard18_dn9 = 0.0;
        var_guard18_db0 = 0.0;
        var_guard18_db1 = 0.0;
        var_guard18_db2 = 0.0;
        var_guard18_db3 = 0.0;
        var_guard18_db4 = 0.0;
        var_guard18_db5 = 0.0;
        var_guard18_db6 = 0.0;
        var_guard18_db7 = 0.0;
        var_guard18_rv = 0.0;
        var_guard18_rdn0 = 0.0;
        var_guard18_rdn1 = 0.0;
        var_guard18_rdn2 = 0.0;
        var_guard18_rdn3 = 0.0;
        var_guard18_rdn4 = 0.0;
        var_guard18_rdn5 = 0.0;
        var_guard18_rdn6 = 0.0;
        var_guard18_rdn7 = 0.0;
        var_guard18_rdn8 = 0.0;
        var_guard18_rdn9 = 0.0;
        var_guard18_rdb0 = 0.0;
        var_guard18_rdb1 = 0.0;
        var_guard18_rdb2 = 0.0;
        var_guard18_rdb3 = 0.0;
        var_guard18_rdb4 = 0.0;
        var_guard18_rdb5 = 0.0;
        var_guard18_rdb6 = 0.0;
        var_guard18_rdb7 = 0.0;

        let (assign1820_e2109, assign1820_e2109_d_n0, assign1820_e2109_d_n1, assign1820_e2109_d_n2, assign1820_e2109_d_n3, assign1820_e2109_d_n4, assign1820_e2109_d_n5, assign1820_e2109_d_n6, assign1820_e2109_d_n7, assign1820_e2109_d_n8, assign1820_e2109_d_n9, assign1820_e2109_d_b0, assign1820_e2109_d_b1, assign1820_e2109_d_b2, assign1820_e2109_d_b3, assign1820_e2109_d_b4, assign1820_e2109_d_b5, assign1820_e2109_d_b6, assign1820_e2109_d_b7,) = {
    if (var_guard18 != 0.0) {
        let assign1820_e2099: f64 = (-1.0);
        let assign1820_e2101: f64 = (assign1820_e2099 - p.p71);
        let assign1820_e2104: f64 = (1.0 - p.p24);
        let assign1820_e2105: f64 = (assign1820_e2104).ln();
        let assign1820_e2106: f64 = (assign1820_e2101 * assign1820_e2105);
        let assign1820_e2107: f64 = (assign1820_e2106).exp();
        (assign1820_e2107, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pwq, var_pwq_dn0, var_pwq_dn1, var_pwq_dn2, var_pwq_dn3, var_pwq_dn4, var_pwq_dn5, var_pwq_dn6, var_pwq_dn7, var_pwq_dn8, var_pwq_dn9, var_pwq_db0, var_pwq_db1, var_pwq_db2, var_pwq_db3, var_pwq_db4, var_pwq_db5, var_pwq_db6, var_pwq_db7,)
    }
};
        var_pwq = assign1820_e2109;
        var_pwq_dn0 = assign1820_e2109_d_n0;
        var_pwq_dn1 = assign1820_e2109_d_n1;
        var_pwq_dn2 = assign1820_e2109_d_n2;
        var_pwq_dn3 = assign1820_e2109_d_n3;
        var_pwq_dn4 = assign1820_e2109_d_n4;
        var_pwq_dn5 = assign1820_e2109_d_n5;
        var_pwq_dn6 = assign1820_e2109_d_n6;
        var_pwq_dn7 = assign1820_e2109_d_n7;
        var_pwq_dn8 = assign1820_e2109_d_n8;
        var_pwq_dn9 = assign1820_e2109_d_n9;
        var_pwq_db0 = assign1820_e2109_d_b0;
        var_pwq_db1 = assign1820_e2109_d_b1;
        var_pwq_db2 = assign1820_e2109_d_b2;
        var_pwq_db3 = assign1820_e2109_d_b3;
        var_pwq_db4 = assign1820_e2109_d_b4;
        var_pwq_db5 = assign1820_e2109_d_b5;
        var_pwq_db6 = assign1820_e2109_d_b6;
        var_pwq_db7 = assign1820_e2109_d_b7;
        var_pwq_rv = 0.0;
        var_pwq_rdn0 = 0.0;
        var_pwq_rdn1 = 0.0;
        var_pwq_rdn2 = 0.0;
        var_pwq_rdn3 = 0.0;
        var_pwq_rdn4 = 0.0;
        var_pwq_rdn5 = 0.0;
        var_pwq_rdn6 = 0.0;
        var_pwq_rdn7 = 0.0;
        var_pwq_rdn8 = 0.0;
        var_pwq_rdn9 = 0.0;
        var_pwq_rdb0 = 0.0;
        var_pwq_rdb1 = 0.0;
        var_pwq_rdb2 = 0.0;
        var_pwq_rdb3 = 0.0;
        var_pwq_rdb4 = 0.0;
        var_pwq_rdb5 = 0.0;
        var_pwq_rdb6 = 0.0;
        var_pwq_rdb7 = 0.0;

        let (assign1830_e2129, assign1830_e2129_d_n0, assign1830_e2129_d_n1, assign1830_e2129_d_n2, assign1830_e2129_d_n3, assign1830_e2129_d_n4, assign1830_e2129_d_n5, assign1830_e2129_d_n6, assign1830_e2129_d_n7, assign1830_e2129_d_n8, assign1830_e2129_d_n9, assign1830_e2129_d_b0, assign1830_e2129_d_b1, assign1830_e2129_d_b2, assign1830_e2129_d_b3, assign1830_e2129_d_b4, assign1830_e2129_d_b5, assign1830_e2129_d_b6, assign1830_e2129_d_b7,) = {
    if (var_guard18 != 0.0) {
        let assign1830_e2116: f64 = (1.0 - p.p24);
        let assign1830_e2117: f64 = (var_pwq * assign1830_e2116);
        let assign1830_e2120: f64 = (1.0 - p.p24);
        let assign1830_e2121: f64 = (assign1830_e2117 * assign1830_e2120);
        let assign1830_e2122: f64 = (1.0 - assign1830_e2121);
        let assign1830_e2123: f64 = (var_vjc_t * assign1830_e2122);
        let assign1830_e2126: f64 = (1.0 - p.p71);
        let assign1830_e2127: f64 = (assign1830_e2123 / assign1830_e2126);
        (assign1830_e2127, (((var_vjc_t_dn0 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn0 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn1 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn1 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn2 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn2 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn3 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn3 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn4 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn4 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn5 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn5 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn6 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn6 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn7 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn7 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn8 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn8 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_dn9 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_dn9 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db0 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db0 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db1 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db1 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db2 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db2 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db3 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db3 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db4 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db4 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db5 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db5 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db6 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db6 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126), (((var_vjc_t_db7 * assign1830_e2122) + (var_vjc_t * (-((var_pwq_db7 * assign1830_e2116) * assign1830_e2120)))) / assign1830_e2126),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7,)
    }
};
        var_qlo = assign1830_e2129;
        var_qlo_dn0 = assign1830_e2129_d_n0;
        var_qlo_dn1 = assign1830_e2129_d_n1;
        var_qlo_dn2 = assign1830_e2129_d_n2;
        var_qlo_dn3 = assign1830_e2129_d_n3;
        var_qlo_dn4 = assign1830_e2129_d_n4;
        var_qlo_dn5 = assign1830_e2129_d_n5;
        var_qlo_dn6 = assign1830_e2129_d_n6;
        var_qlo_dn7 = assign1830_e2129_d_n7;
        var_qlo_dn8 = assign1830_e2129_d_n8;
        var_qlo_dn9 = assign1830_e2129_d_n9;
        var_qlo_db0 = assign1830_e2129_d_b0;
        var_qlo_db1 = assign1830_e2129_d_b1;
        var_qlo_db2 = assign1830_e2129_d_b2;
        var_qlo_db3 = assign1830_e2129_d_b3;
        var_qlo_db4 = assign1830_e2129_d_b4;
        var_qlo_db5 = assign1830_e2129_d_b5;
        var_qlo_db6 = assign1830_e2129_d_b6;
        var_qlo_db7 = assign1830_e2129_d_b7;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdn7 = 0.0;
        var_qlo_rdn8 = 0.0;
        var_qlo_rdn9 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;
        var_qlo_rdb7 = 0.0;

        let (assign1840_e2147, assign1840_e2147_d_n0, assign1840_e2147_d_n1, assign1840_e2147_d_n2, assign1840_e2147_d_n3, assign1840_e2147_d_n4, assign1840_e2147_d_n5, assign1840_e2147_d_n6, assign1840_e2147_d_n7, assign1840_e2147_d_n8, assign1840_e2147_d_n9, assign1840_e2147_d_b0, assign1840_e2147_d_b1, assign1840_e2147_d_b2, assign1840_e2147_d_b3, assign1840_e2147_d_b4, assign1840_e2147_d_b5, assign1840_e2147_d_b6, assign1840_e2147_d_b7,) = {
    if (var_guard18 != 0.0) {
        let assign1840_e2134: f64 = (1.0 - p.p24);
        let assign1840_e2137: f64 = (0.5 * p.p71);
        let assign1840_e2139: f64 = (assign1840_e2137 * var_dvh);
        let assign1840_e2141: f64 = (assign1840_e2139 / var_vjc_t);
        let assign1840_e2142: f64 = (assign1840_e2134 + assign1840_e2141);
        let assign1840_e2143: f64 = (var_dvh * assign1840_e2142);
        let assign1840_e2145: f64 = (assign1840_e2143 * var_pwq);
        (assign1840_e2145, ((((var_dvh_dn0 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn0) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn0)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn0)), ((((var_dvh_dn1 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn1) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn1)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn1)), ((((var_dvh_dn2 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn2) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn2)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn2)), ((((var_dvh_dn3 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn3) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn3)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn3)), ((((var_dvh_dn4 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn4) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn4)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn4)), ((((var_dvh_dn5 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn5) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn5)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn5)), ((((var_dvh_dn6 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn6) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn6)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn6)), ((((var_dvh_dn7 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn7) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn7)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn7)), ((((var_dvh_dn8 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn8) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn8)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn8)), ((((var_dvh_dn9 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_dn9) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_dn9)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_dn9)), ((((var_dvh_db0 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db0) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db0)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db0)), ((((var_dvh_db1 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db1) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db1)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db1)), ((((var_dvh_db2 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db2) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db2)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db2)), ((((var_dvh_db3 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db3) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db3)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db3)), ((((var_dvh_db4 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db4) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db4)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db4)), ((((var_dvh_db5 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db5) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db5)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db5)), ((((var_dvh_db6 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db6) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db6)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db6)), ((((var_dvh_db7 * assign1840_e2142) + (var_dvh * ((((assign1840_e2137 * var_dvh_db7) * var_vjc_t) - (assign1840_e2139 * var_vjc_t_db7)) / (var_vjc_t * var_vjc_t)))) * var_pwq) + (assign1840_e2143 * var_pwq_db7)),)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_dn7, var_qhi_dn8, var_qhi_dn9, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6, var_qhi_db7,)
    }
};
        var_qhi = assign1840_e2147;
        var_qhi_dn0 = assign1840_e2147_d_n0;
        var_qhi_dn1 = assign1840_e2147_d_n1;
        var_qhi_dn2 = assign1840_e2147_d_n2;
        var_qhi_dn3 = assign1840_e2147_d_n3;
        var_qhi_dn4 = assign1840_e2147_d_n4;
        var_qhi_dn5 = assign1840_e2147_d_n5;
        var_qhi_dn6 = assign1840_e2147_d_n6;
        var_qhi_dn7 = assign1840_e2147_d_n7;
        var_qhi_dn8 = assign1840_e2147_d_n8;
        var_qhi_dn9 = assign1840_e2147_d_n9;
        var_qhi_db0 = assign1840_e2147_d_b0;
        var_qhi_db1 = assign1840_e2147_d_b1;
        var_qhi_db2 = assign1840_e2147_d_b2;
        var_qhi_db3 = assign1840_e2147_d_b3;
        var_qhi_db4 = assign1840_e2147_d_b4;
        var_qhi_db5 = assign1840_e2147_d_b5;
        var_qhi_db6 = assign1840_e2147_d_b6;
        var_qhi_db7 = assign1840_e2147_d_b7;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdn7 = 0.0;
        var_qhi_rdn8 = 0.0;
        var_qhi_rdn9 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;
        var_qhi_rdb7 = 0.0;

        let (assign1850_e2170, assign1850_e2170_d_n0, assign1850_e2170_d_n1, assign1850_e2170_d_n2, assign1850_e2170_d_n3, assign1850_e2170_d_n4, assign1850_e2170_d_n5, assign1850_e2170_d_n6, assign1850_e2170_d_n7, assign1850_e2170_d_n8, assign1850_e2170_d_n9, assign1850_e2170_d_b0, assign1850_e2170_d_b1, assign1850_e2170_d_b2, assign1850_e2170_d_b3, assign1850_e2170_d_b4, assign1850_e2170_d_b5, assign1850_e2170_d_b6, assign1850_e2170_d_b7,) = {
    if (var_guard18 == 0.0) {
        let assign1850_e2154: f64 = (1.0 - p.p71);
        let assign1850_e2158: f64 = (var_vbici / var_vjc_t);
        let assign1850_e2159: f64 = (1.0 - assign1850_e2158);
        let assign1850_e2160: f64 = (assign1850_e2159).ln();
        let assign1850_e2161: f64 = (assign1850_e2154 * assign1850_e2160);
        let assign1850_e2162: f64 = (assign1850_e2161).exp();
        let assign1850_e2163: f64 = (1.0 - assign1850_e2162);
        let assign1850_e2164: f64 = (var_vjc_t * assign1850_e2163);
        let assign1850_e2167: f64 = (1.0 - p.p71);
        let assign1850_e2168: f64 = (assign1850_e2164 / assign1850_e2167);
        (assign1850_e2168, (((var_vjc_t_dn0 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn0 * var_vjc_t) - (var_vbici * var_vjc_t_dn0)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn1 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn1 * var_vjc_t) - (var_vbici * var_vjc_t_dn1)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn2 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn2 * var_vjc_t) - (var_vbici * var_vjc_t_dn2)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn3 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn3 * var_vjc_t) - (var_vbici * var_vjc_t_dn3)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn4 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn4 * var_vjc_t) - (var_vbici * var_vjc_t_dn4)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn5 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn5 * var_vjc_t) - (var_vbici * var_vjc_t_dn5)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn6 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn6 * var_vjc_t) - (var_vbici * var_vjc_t_dn6)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn7 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn7 * var_vjc_t) - (var_vbici * var_vjc_t_dn7)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn8 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn8 * var_vjc_t) - (var_vbici * var_vjc_t_dn8)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_dn9 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_dn9 * var_vjc_t) - (var_vbici * var_vjc_t_dn9)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db0 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db0 * var_vjc_t) - (var_vbici * var_vjc_t_db0)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db1 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db1 * var_vjc_t) - (var_vbici * var_vjc_t_db1)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db2 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db2 * var_vjc_t) - (var_vbici * var_vjc_t_db2)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db3 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db3 * var_vjc_t) - (var_vbici * var_vjc_t_db3)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db4 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db4 * var_vjc_t) - (var_vbici * var_vjc_t_db4)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db5 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db5 * var_vjc_t) - (var_vbici * var_vjc_t_db5)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db6 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db6 * var_vjc_t) - (var_vbici * var_vjc_t_db6)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167), (((var_vjc_t_db7 * assign1850_e2163) + (var_vjc_t * (-(assign1850_e2162 * (assign1850_e2154 * ((-(((var_vbici_db7 * var_vjc_t) - (var_vbici * var_vjc_t_db7)) / (var_vjc_t * var_vjc_t))) / assign1850_e2159)))))) / assign1850_e2167),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_dn6, var_qlo_dn7, var_qlo_dn8, var_qlo_dn9, var_qlo_db0, var_qlo_db1, var_qlo_db2, var_qlo_db3, var_qlo_db4, var_qlo_db5, var_qlo_db6, var_qlo_db7,)
    }
};
        var_qlo = assign1850_e2170;
        var_qlo_dn0 = assign1850_e2170_d_n0;
        var_qlo_dn1 = assign1850_e2170_d_n1;
        var_qlo_dn2 = assign1850_e2170_d_n2;
        var_qlo_dn3 = assign1850_e2170_d_n3;
        var_qlo_dn4 = assign1850_e2170_d_n4;
        var_qlo_dn5 = assign1850_e2170_d_n5;
        var_qlo_dn6 = assign1850_e2170_d_n6;
        var_qlo_dn7 = assign1850_e2170_d_n7;
        var_qlo_dn8 = assign1850_e2170_d_n8;
        var_qlo_dn9 = assign1850_e2170_d_n9;
        var_qlo_db0 = assign1850_e2170_d_b0;
        var_qlo_db1 = assign1850_e2170_d_b1;
        var_qlo_db2 = assign1850_e2170_d_b2;
        var_qlo_db3 = assign1850_e2170_d_b3;
        var_qlo_db4 = assign1850_e2170_d_b4;
        var_qlo_db5 = assign1850_e2170_d_b5;
        var_qlo_db6 = assign1850_e2170_d_b6;
        var_qlo_db7 = assign1850_e2170_d_b7;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdn6 = 0.0;
        var_qlo_rdn7 = 0.0;
        var_qlo_rdn8 = 0.0;
        var_qlo_rdn9 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;
        var_qlo_rdb2 = 0.0;
        var_qlo_rdb3 = 0.0;
        var_qlo_rdb4 = 0.0;
        var_qlo_rdb5 = 0.0;
        var_qlo_rdb6 = 0.0;
        var_qlo_rdb7 = 0.0;

        let (assign1860_e2175, assign1860_e2175_d_n0, assign1860_e2175_d_n1, assign1860_e2175_d_n2, assign1860_e2175_d_n3, assign1860_e2175_d_n4, assign1860_e2175_d_n5, assign1860_e2175_d_n6, assign1860_e2175_d_n7, assign1860_e2175_d_n8, assign1860_e2175_d_n9, assign1860_e2175_d_b0, assign1860_e2175_d_b1, assign1860_e2175_d_b2, assign1860_e2175_d_b3, assign1860_e2175_d_b4, assign1860_e2175_d_b5, assign1860_e2175_d_b6, assign1860_e2175_d_b7,) = {
    if (var_guard18 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_dn6, var_qhi_dn7, var_qhi_dn8, var_qhi_dn9, var_qhi_db0, var_qhi_db1, var_qhi_db2, var_qhi_db3, var_qhi_db4, var_qhi_db5, var_qhi_db6, var_qhi_db7,)
    }
};
        var_qhi = assign1860_e2175;
        var_qhi_dn0 = assign1860_e2175_d_n0;
        var_qhi_dn1 = assign1860_e2175_d_n1;
        var_qhi_dn2 = assign1860_e2175_d_n2;
        var_qhi_dn3 = assign1860_e2175_d_n3;
        var_qhi_dn4 = assign1860_e2175_d_n4;
        var_qhi_dn5 = assign1860_e2175_d_n5;
        var_qhi_dn6 = assign1860_e2175_d_n6;
        var_qhi_dn7 = assign1860_e2175_d_n7;
        var_qhi_dn8 = assign1860_e2175_d_n8;
        var_qhi_dn9 = assign1860_e2175_d_n9;
        var_qhi_db0 = assign1860_e2175_d_b0;
        var_qhi_db1 = assign1860_e2175_d_b1;
        var_qhi_db2 = assign1860_e2175_d_b2;
        var_qhi_db3 = assign1860_e2175_d_b3;
        var_qhi_db4 = assign1860_e2175_d_b4;
        var_qhi_db5 = assign1860_e2175_d_b5;
        var_qhi_db6 = assign1860_e2175_d_b6;
        var_qhi_db7 = assign1860_e2175_d_b7;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdn6 = 0.0;
        var_qhi_rdn7 = 0.0;
        var_qhi_rdn8 = 0.0;
        var_qhi_rdn9 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;
        var_qhi_rdb2 = 0.0;
        var_qhi_rdb3 = 0.0;
        var_qhi_rdb4 = 0.0;
        var_qhi_rdb5 = 0.0;
        var_qhi_rdb6 = 0.0;
        var_qhi_rdb7 = 0.0;

        *var_dv0_slot = var_dv0;
        *var_dv0_db0_slot = var_dv0_db0;
        *var_dv0_db1_slot = var_dv0_db1;
        *var_dv0_db2_slot = var_dv0_db2;
        *var_dv0_db3_slot = var_dv0_db3;
        *var_dv0_db4_slot = var_dv0_db4;
        *var_dv0_db5_slot = var_dv0_db5;
        *var_dv0_db6_slot = var_dv0_db6;
        *var_dv0_db7_slot = var_dv0_db7;
        *var_dv0_dn0_slot = var_dv0_dn0;
        *var_dv0_dn1_slot = var_dv0_dn1;
        *var_dv0_dn2_slot = var_dv0_dn2;
        *var_dv0_dn3_slot = var_dv0_dn3;
        *var_dv0_dn4_slot = var_dv0_dn4;
        *var_dv0_dn5_slot = var_dv0_dn5;
        *var_dv0_dn6_slot = var_dv0_dn6;
        *var_dv0_dn7_slot = var_dv0_dn7;
        *var_dv0_dn8_slot = var_dv0_dn8;
        *var_dv0_dn9_slot = var_dv0_dn9;
        *var_dv0_rdb0_slot = var_dv0_rdb0;
        *var_dv0_rdb1_slot = var_dv0_rdb1;
        *var_dv0_rdb2_slot = var_dv0_rdb2;
        *var_dv0_rdb3_slot = var_dv0_rdb3;
        *var_dv0_rdb4_slot = var_dv0_rdb4;
        *var_dv0_rdb5_slot = var_dv0_rdb5;
        *var_dv0_rdb6_slot = var_dv0_rdb6;
        *var_dv0_rdb7_slot = var_dv0_rdb7;
        *var_dv0_rdn0_slot = var_dv0_rdn0;
        *var_dv0_rdn1_slot = var_dv0_rdn1;
        *var_dv0_rdn2_slot = var_dv0_rdn2;
        *var_dv0_rdn3_slot = var_dv0_rdn3;
        *var_dv0_rdn4_slot = var_dv0_rdn4;
        *var_dv0_rdn5_slot = var_dv0_rdn5;
        *var_dv0_rdn6_slot = var_dv0_rdn6;
        *var_dv0_rdn7_slot = var_dv0_rdn7;
        *var_dv0_rdn8_slot = var_dv0_rdn8;
        *var_dv0_rdn9_slot = var_dv0_rdn9;
        *var_dv0_rv_slot = var_dv0_rv;
        *var_dvh_slot = var_dvh;
        *var_dvh_db0_slot = var_dvh_db0;
        *var_dvh_db1_slot = var_dvh_db1;
        *var_dvh_db2_slot = var_dvh_db2;
        *var_dvh_db3_slot = var_dvh_db3;
        *var_dvh_db4_slot = var_dvh_db4;
        *var_dvh_db5_slot = var_dvh_db5;
        *var_dvh_db6_slot = var_dvh_db6;
        *var_dvh_db7_slot = var_dvh_db7;
        *var_dvh_dn0_slot = var_dvh_dn0;
        *var_dvh_dn1_slot = var_dvh_dn1;
        *var_dvh_dn2_slot = var_dvh_dn2;
        *var_dvh_dn3_slot = var_dvh_dn3;
        *var_dvh_dn4_slot = var_dvh_dn4;
        *var_dvh_dn5_slot = var_dvh_dn5;
        *var_dvh_dn6_slot = var_dvh_dn6;
        *var_dvh_dn7_slot = var_dvh_dn7;
        *var_dvh_dn8_slot = var_dvh_dn8;
        *var_dvh_dn9_slot = var_dvh_dn9;
        *var_dvh_rdb0_slot = var_dvh_rdb0;
        *var_dvh_rdb1_slot = var_dvh_rdb1;
        *var_dvh_rdb2_slot = var_dvh_rdb2;
        *var_dvh_rdb3_slot = var_dvh_rdb3;
        *var_dvh_rdb4_slot = var_dvh_rdb4;
        *var_dvh_rdb5_slot = var_dvh_rdb5;
        *var_dvh_rdb6_slot = var_dvh_rdb6;
        *var_dvh_rdb7_slot = var_dvh_rdb7;
        *var_dvh_rdn0_slot = var_dvh_rdn0;
        *var_dvh_rdn1_slot = var_dvh_rdn1;
        *var_dvh_rdn2_slot = var_dvh_rdn2;
        *var_dvh_rdn3_slot = var_dvh_rdn3;
        *var_dvh_rdn4_slot = var_dvh_rdn4;
        *var_dvh_rdn5_slot = var_dvh_rdn5;
        *var_dvh_rdn6_slot = var_dvh_rdn6;
        *var_dvh_rdn7_slot = var_dvh_rdn7;
        *var_dvh_rdn8_slot = var_dvh_rdn8;
        *var_dvh_rdn9_slot = var_dvh_rdn9;
        *var_dvh_rv_slot = var_dvh_rv;
        *var_guard18_slot = var_guard18;
        *var_guard18_db0_slot = var_guard18_db0;
        *var_guard18_db1_slot = var_guard18_db1;
        *var_guard18_db2_slot = var_guard18_db2;
        *var_guard18_db3_slot = var_guard18_db3;
        *var_guard18_db4_slot = var_guard18_db4;
        *var_guard18_db5_slot = var_guard18_db5;
        *var_guard18_db6_slot = var_guard18_db6;
        *var_guard18_db7_slot = var_guard18_db7;
        *var_guard18_dn0_slot = var_guard18_dn0;
        *var_guard18_dn1_slot = var_guard18_dn1;
        *var_guard18_dn2_slot = var_guard18_dn2;
        *var_guard18_dn3_slot = var_guard18_dn3;
        *var_guard18_dn4_slot = var_guard18_dn4;
        *var_guard18_dn5_slot = var_guard18_dn5;
        *var_guard18_dn6_slot = var_guard18_dn6;
        *var_guard18_dn7_slot = var_guard18_dn7;
        *var_guard18_dn8_slot = var_guard18_dn8;
        *var_guard18_dn9_slot = var_guard18_dn9;
        *var_guard18_rdb0_slot = var_guard18_rdb0;
        *var_guard18_rdb1_slot = var_guard18_rdb1;
        *var_guard18_rdb2_slot = var_guard18_rdb2;
        *var_guard18_rdb3_slot = var_guard18_rdb3;
        *var_guard18_rdb4_slot = var_guard18_rdb4;
        *var_guard18_rdb5_slot = var_guard18_rdb5;
        *var_guard18_rdb6_slot = var_guard18_rdb6;
        *var_guard18_rdb7_slot = var_guard18_rdb7;
        *var_guard18_rdn0_slot = var_guard18_rdn0;
        *var_guard18_rdn1_slot = var_guard18_rdn1;
        *var_guard18_rdn2_slot = var_guard18_rdn2;
        *var_guard18_rdn3_slot = var_guard18_rdn3;
        *var_guard18_rdn4_slot = var_guard18_rdn4;
        *var_guard18_rdn5_slot = var_guard18_rdn5;
        *var_guard18_rdn6_slot = var_guard18_rdn6;
        *var_guard18_rdn7_slot = var_guard18_rdn7;
        *var_guard18_rdn8_slot = var_guard18_rdn8;
        *var_guard18_rdn9_slot = var_guard18_rdn9;
        *var_guard18_rv_slot = var_guard18_rv;
        *var_pwq_slot = var_pwq;
        *var_pwq_db0_slot = var_pwq_db0;
        *var_pwq_db1_slot = var_pwq_db1;
        *var_pwq_db2_slot = var_pwq_db2;
        *var_pwq_db3_slot = var_pwq_db3;
        *var_pwq_db4_slot = var_pwq_db4;
        *var_pwq_db5_slot = var_pwq_db5;
        *var_pwq_db6_slot = var_pwq_db6;
        *var_pwq_db7_slot = var_pwq_db7;
        *var_pwq_dn0_slot = var_pwq_dn0;
        *var_pwq_dn1_slot = var_pwq_dn1;
        *var_pwq_dn2_slot = var_pwq_dn2;
        *var_pwq_dn3_slot = var_pwq_dn3;
        *var_pwq_dn4_slot = var_pwq_dn4;
        *var_pwq_dn5_slot = var_pwq_dn5;
        *var_pwq_dn6_slot = var_pwq_dn6;
        *var_pwq_dn7_slot = var_pwq_dn7;
        *var_pwq_dn8_slot = var_pwq_dn8;
        *var_pwq_dn9_slot = var_pwq_dn9;
        *var_pwq_rdb0_slot = var_pwq_rdb0;
        *var_pwq_rdb1_slot = var_pwq_rdb1;
        *var_pwq_rdb2_slot = var_pwq_rdb2;
        *var_pwq_rdb3_slot = var_pwq_rdb3;
        *var_pwq_rdb4_slot = var_pwq_rdb4;
        *var_pwq_rdb5_slot = var_pwq_rdb5;
        *var_pwq_rdb6_slot = var_pwq_rdb6;
        *var_pwq_rdb7_slot = var_pwq_rdb7;
        *var_pwq_rdn0_slot = var_pwq_rdn0;
        *var_pwq_rdn1_slot = var_pwq_rdn1;
        *var_pwq_rdn2_slot = var_pwq_rdn2;
        *var_pwq_rdn3_slot = var_pwq_rdn3;
        *var_pwq_rdn4_slot = var_pwq_rdn4;
        *var_pwq_rdn5_slot = var_pwq_rdn5;
        *var_pwq_rdn6_slot = var_pwq_rdn6;
        *var_pwq_rdn7_slot = var_pwq_rdn7;
        *var_pwq_rdn8_slot = var_pwq_rdn8;
        *var_pwq_rdn9_slot = var_pwq_rdn9;
        *var_pwq_rv_slot = var_pwq_rv;
        *var_qhi_slot = var_qhi;
        *var_qhi_db0_slot = var_qhi_db0;
        *var_qhi_db1_slot = var_qhi_db1;
        *var_qhi_db2_slot = var_qhi_db2;
        *var_qhi_db3_slot = var_qhi_db3;
        *var_qhi_db4_slot = var_qhi_db4;
        *var_qhi_db5_slot = var_qhi_db5;
        *var_qhi_db6_slot = var_qhi_db6;
        *var_qhi_db7_slot = var_qhi_db7;
        *var_qhi_dn0_slot = var_qhi_dn0;
        *var_qhi_dn1_slot = var_qhi_dn1;
        *var_qhi_dn2_slot = var_qhi_dn2;
        *var_qhi_dn3_slot = var_qhi_dn3;
        *var_qhi_dn4_slot = var_qhi_dn4;
        *var_qhi_dn5_slot = var_qhi_dn5;
        *var_qhi_dn6_slot = var_qhi_dn6;
        *var_qhi_dn7_slot = var_qhi_dn7;
        *var_qhi_dn8_slot = var_qhi_dn8;
        *var_qhi_dn9_slot = var_qhi_dn9;
        *var_qhi_rdb0_slot = var_qhi_rdb0;
        *var_qhi_rdb1_slot = var_qhi_rdb1;
        *var_qhi_rdb2_slot = var_qhi_rdb2;
        *var_qhi_rdb3_slot = var_qhi_rdb3;
        *var_qhi_rdb4_slot = var_qhi_rdb4;
        *var_qhi_rdb5_slot = var_qhi_rdb5;
        *var_qhi_rdb6_slot = var_qhi_rdb6;
        *var_qhi_rdb7_slot = var_qhi_rdb7;
        *var_qhi_rdn0_slot = var_qhi_rdn0;
        *var_qhi_rdn1_slot = var_qhi_rdn1;
        *var_qhi_rdn2_slot = var_qhi_rdn2;
        *var_qhi_rdn3_slot = var_qhi_rdn3;
        *var_qhi_rdn4_slot = var_qhi_rdn4;
        *var_qhi_rdn5_slot = var_qhi_rdn5;
        *var_qhi_rdn6_slot = var_qhi_rdn6;
        *var_qhi_rdn7_slot = var_qhi_rdn7;
        *var_qhi_rdn8_slot = var_qhi_rdn8;
        *var_qhi_rdn9_slot = var_qhi_rdn9;
        *var_qhi_rv_slot = var_qhi_rv;
        *var_qjcx_slot = var_qjcx;
        *var_qjcx_1_slot = var_qjcx_1;
        *var_qjcx_1_db0_slot = var_qjcx_1_db0;
        *var_qjcx_1_db1_slot = var_qjcx_1_db1;
        *var_qjcx_1_db2_slot = var_qjcx_1_db2;
        *var_qjcx_1_db3_slot = var_qjcx_1_db3;
        *var_qjcx_1_db4_slot = var_qjcx_1_db4;
        *var_qjcx_1_db5_slot = var_qjcx_1_db5;
        *var_qjcx_1_db6_slot = var_qjcx_1_db6;
        *var_qjcx_1_db7_slot = var_qjcx_1_db7;
        *var_qjcx_1_dn0_slot = var_qjcx_1_dn0;
        *var_qjcx_1_dn1_slot = var_qjcx_1_dn1;
        *var_qjcx_1_dn2_slot = var_qjcx_1_dn2;
        *var_qjcx_1_dn3_slot = var_qjcx_1_dn3;
        *var_qjcx_1_dn4_slot = var_qjcx_1_dn4;
        *var_qjcx_1_dn5_slot = var_qjcx_1_dn5;
        *var_qjcx_1_dn6_slot = var_qjcx_1_dn6;
        *var_qjcx_1_dn7_slot = var_qjcx_1_dn7;
        *var_qjcx_1_dn8_slot = var_qjcx_1_dn8;
        *var_qjcx_1_dn9_slot = var_qjcx_1_dn9;
        *var_qjcx_1_rdb0_slot = var_qjcx_1_rdb0;
        *var_qjcx_1_rdb1_slot = var_qjcx_1_rdb1;
        *var_qjcx_1_rdb2_slot = var_qjcx_1_rdb2;
        *var_qjcx_1_rdb3_slot = var_qjcx_1_rdb3;
        *var_qjcx_1_rdb4_slot = var_qjcx_1_rdb4;
        *var_qjcx_1_rdb5_slot = var_qjcx_1_rdb5;
        *var_qjcx_1_rdb6_slot = var_qjcx_1_rdb6;
        *var_qjcx_1_rdb7_slot = var_qjcx_1_rdb7;
        *var_qjcx_1_rdn0_slot = var_qjcx_1_rdn0;
        *var_qjcx_1_rdn1_slot = var_qjcx_1_rdn1;
        *var_qjcx_1_rdn2_slot = var_qjcx_1_rdn2;
        *var_qjcx_1_rdn3_slot = var_qjcx_1_rdn3;
        *var_qjcx_1_rdn4_slot = var_qjcx_1_rdn4;
        *var_qjcx_1_rdn5_slot = var_qjcx_1_rdn5;
        *var_qjcx_1_rdn6_slot = var_qjcx_1_rdn6;
        *var_qjcx_1_rdn7_slot = var_qjcx_1_rdn7;
        *var_qjcx_1_rdn8_slot = var_qjcx_1_rdn8;
        *var_qjcx_1_rdn9_slot = var_qjcx_1_rdn9;
        *var_qjcx_1_rv_slot = var_qjcx_1_rv;
        *var_qjcx_db0_slot = var_qjcx_db0;
        *var_qjcx_db1_slot = var_qjcx_db1;
        *var_qjcx_db2_slot = var_qjcx_db2;
        *var_qjcx_db3_slot = var_qjcx_db3;
        *var_qjcx_db4_slot = var_qjcx_db4;
        *var_qjcx_db5_slot = var_qjcx_db5;
        *var_qjcx_db6_slot = var_qjcx_db6;
        *var_qjcx_db7_slot = var_qjcx_db7;
        *var_qjcx_dn0_slot = var_qjcx_dn0;
        *var_qjcx_dn1_slot = var_qjcx_dn1;
        *var_qjcx_dn2_slot = var_qjcx_dn2;
        *var_qjcx_dn3_slot = var_qjcx_dn3;
        *var_qjcx_dn4_slot = var_qjcx_dn4;
        *var_qjcx_dn5_slot = var_qjcx_dn5;
        *var_qjcx_dn6_slot = var_qjcx_dn6;
        *var_qjcx_dn7_slot = var_qjcx_dn7;
        *var_qjcx_dn8_slot = var_qjcx_dn8;
        *var_qjcx_dn9_slot = var_qjcx_dn9;
        *var_qjcx_rdb0_slot = var_qjcx_rdb0;
        *var_qjcx_rdb1_slot = var_qjcx_rdb1;
        *var_qjcx_rdb2_slot = var_qjcx_rdb2;
        *var_qjcx_rdb3_slot = var_qjcx_rdb3;
        *var_qjcx_rdb4_slot = var_qjcx_rdb4;
        *var_qjcx_rdb5_slot = var_qjcx_rdb5;
        *var_qjcx_rdb6_slot = var_qjcx_rdb6;
        *var_qjcx_rdb7_slot = var_qjcx_rdb7;
        *var_qjcx_rdn0_slot = var_qjcx_rdn0;
        *var_qjcx_rdn1_slot = var_qjcx_rdn1;
        *var_qjcx_rdn2_slot = var_qjcx_rdn2;
        *var_qjcx_rdn3_slot = var_qjcx_rdn3;
        *var_qjcx_rdn4_slot = var_qjcx_rdn4;
        *var_qjcx_rdn5_slot = var_qjcx_rdn5;
        *var_qjcx_rdn6_slot = var_qjcx_rdn6;
        *var_qjcx_rdn7_slot = var_qjcx_rdn7;
        *var_qjcx_rdn8_slot = var_qjcx_rdn8;
        *var_qjcx_rdn9_slot = var_qjcx_rdn9;
        *var_qjcx_rv_slot = var_qjcx_rv;
        *var_qlo_slot = var_qlo;
        *var_qlo_db0_slot = var_qlo_db0;
        *var_qlo_db1_slot = var_qlo_db1;
        *var_qlo_db2_slot = var_qlo_db2;
        *var_qlo_db3_slot = var_qlo_db3;
        *var_qlo_db4_slot = var_qlo_db4;
        *var_qlo_db5_slot = var_qlo_db5;
        *var_qlo_db6_slot = var_qlo_db6;
        *var_qlo_db7_slot = var_qlo_db7;
        *var_qlo_dn0_slot = var_qlo_dn0;
        *var_qlo_dn1_slot = var_qlo_dn1;
        *var_qlo_dn2_slot = var_qlo_dn2;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_qlo_dn5_slot = var_qlo_dn5;
        *var_qlo_dn6_slot = var_qlo_dn6;
        *var_qlo_dn7_slot = var_qlo_dn7;
        *var_qlo_dn8_slot = var_qlo_dn8;
        *var_qlo_dn9_slot = var_qlo_dn9;
        *var_qlo_rdb0_slot = var_qlo_rdb0;
        *var_qlo_rdb1_slot = var_qlo_rdb1;
        *var_qlo_rdb2_slot = var_qlo_rdb2;
        *var_qlo_rdb3_slot = var_qlo_rdb3;
        *var_qlo_rdb4_slot = var_qlo_rdb4;
        *var_qlo_rdb5_slot = var_qlo_rdb5;
        *var_qlo_rdb6_slot = var_qlo_rdb6;
        *var_qlo_rdb7_slot = var_qlo_rdb7;
        *var_qlo_rdn0_slot = var_qlo_rdn0;
        *var_qlo_rdn1_slot = var_qlo_rdn1;
        *var_qlo_rdn2_slot = var_qlo_rdn2;
        *var_qlo_rdn3_slot = var_qlo_rdn3;
        *var_qlo_rdn4_slot = var_qlo_rdn4;
        *var_qlo_rdn5_slot = var_qlo_rdn5;
        *var_qlo_rdn6_slot = var_qlo_rdn6;
        *var_qlo_rdn7_slot = var_qlo_rdn7;
        *var_qlo_rdn8_slot = var_qlo_rdn8;
        *var_qlo_rdn9_slot = var_qlo_rdn9;
        *var_qlo_rv_slot = var_qlo_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        var_cjc_t: f64,
        var_cjc_t_db0: f64,
        var_cjc_t_db1: f64,
        var_cjc_t_db2: f64,
        var_cjc_t_db3: f64,
        var_cjc_t_db4: f64,
        var_cjc_t_db5: f64,
        var_cjc_t_db6: f64,
        var_cjc_t_db7: f64,
        var_cjc_t_dn0: f64,
        var_cjc_t_dn1: f64,
        var_cjc_t_dn2: f64,
        var_cjc_t_dn3: f64,
        var_cjc_t_dn4: f64,
        var_cjc_t_dn5: f64,
        var_cjc_t_dn6: f64,
        var_cjc_t_dn7: f64,
        var_cjc_t_dn8: f64,
        var_cjc_t_dn9: f64,
        var_itzf: f64,
        var_itzf_db0: f64,
        var_itzf_db1: f64,
        var_itzf_db2: f64,
        var_itzf_db3: f64,
        var_itzf_db4: f64,
        var_itzf_db5: f64,
        var_itzf_db6: f64,
        var_itzf_db7: f64,
        var_itzf_dn0: f64,
        var_itzf_dn1: f64,
        var_itzf_dn2: f64,
        var_itzf_dn3: f64,
        var_itzf_dn4: f64,
        var_itzf_dn5: f64,
        var_itzf_dn6: f64,
        var_itzf_dn7: f64,
        var_itzf_dn8: f64,
        var_itzf_dn9: f64,
        var_qhi: f64,
        var_qhi_db0: f64,
        var_qhi_db1: f64,
        var_qhi_db2: f64,
        var_qhi_db3: f64,
        var_qhi_db4: f64,
        var_qhi_db5: f64,
        var_qhi_db6: f64,
        var_qhi_db7: f64,
        var_qhi_dn0: f64,
        var_qhi_dn1: f64,
        var_qhi_dn2: f64,
        var_qhi_dn3: f64,
        var_qhi_dn4: f64,
        var_qhi_dn5: f64,
        var_qhi_dn6: f64,
        var_qhi_dn7: f64,
        var_qhi_dn8: f64,
        var_qhi_dn9: f64,
        var_qlo: f64,
        var_qlo_db0: f64,
        var_qlo_db1: f64,
        var_qlo_db2: f64,
        var_qlo_db3: f64,
        var_qlo_db4: f64,
        var_qlo_db5: f64,
        var_qlo_db6: f64,
        var_qlo_db7: f64,
        var_qlo_dn0: f64,
        var_qlo_dn1: f64,
        var_qlo_dn2: f64,
        var_qlo_dn3: f64,
        var_qlo_dn4: f64,
        var_qlo_dn5: f64,
        var_qlo_dn6: f64,
        var_qlo_dn7: f64,
        var_qlo_dn8: f64,
        var_qlo_dn9: f64,
        var_ttype: f64,
        var_ttype_db0: f64,
        var_ttype_db1: f64,
        var_ttype_db2: f64,
        var_ttype_db3: f64,
        var_ttype_db4: f64,
        var_ttype_db5: f64,
        var_ttype_db6: f64,
        var_ttype_db7: f64,
        var_ttype_dn0: f64,
        var_ttype_dn1: f64,
        var_ttype_dn2: f64,
        var_ttype_dn3: f64,
        var_ttype_dn4: f64,
        var_ttype_dn5: f64,
        var_ttype_dn6: f64,
        var_ttype_dn7: f64,
        var_ttype_dn8: f64,
        var_ttype_dn9: f64,
        var_guard19_slot: &mut f64,
        var_guard19_db0_slot: &mut f64,
        var_guard19_db1_slot: &mut f64,
        var_guard19_db2_slot: &mut f64,
        var_guard19_db3_slot: &mut f64,
        var_guard19_db4_slot: &mut f64,
        var_guard19_db5_slot: &mut f64,
        var_guard19_db6_slot: &mut f64,
        var_guard19_db7_slot: &mut f64,
        var_guard19_dn0_slot: &mut f64,
        var_guard19_dn1_slot: &mut f64,
        var_guard19_dn2_slot: &mut f64,
        var_guard19_dn3_slot: &mut f64,
        var_guard19_dn4_slot: &mut f64,
        var_guard19_dn5_slot: &mut f64,
        var_guard19_dn6_slot: &mut f64,
        var_guard19_dn7_slot: &mut f64,
        var_guard19_dn8_slot: &mut f64,
        var_guard19_dn9_slot: &mut f64,
        var_guard19_rdb0_slot: &mut f64,
        var_guard19_rdb1_slot: &mut f64,
        var_guard19_rdb2_slot: &mut f64,
        var_guard19_rdb3_slot: &mut f64,
        var_guard19_rdb4_slot: &mut f64,
        var_guard19_rdb5_slot: &mut f64,
        var_guard19_rdb6_slot: &mut f64,
        var_guard19_rdb7_slot: &mut f64,
        var_guard19_rdn0_slot: &mut f64,
        var_guard19_rdn1_slot: &mut f64,
        var_guard19_rdn2_slot: &mut f64,
        var_guard19_rdn3_slot: &mut f64,
        var_guard19_rdn4_slot: &mut f64,
        var_guard19_rdn5_slot: &mut f64,
        var_guard19_rdn6_slot: &mut f64,
        var_guard19_rdn7_slot: &mut f64,
        var_guard19_rdn8_slot: &mut f64,
        var_guard19_rdn9_slot: &mut f64,
        var_guard19_rv_slot: &mut f64,
        var_guard20_slot: &mut f64,
        var_guard20_db0_slot: &mut f64,
        var_guard20_db1_slot: &mut f64,
        var_guard20_db2_slot: &mut f64,
        var_guard20_db3_slot: &mut f64,
        var_guard20_db4_slot: &mut f64,
        var_guard20_db5_slot: &mut f64,
        var_guard20_db6_slot: &mut f64,
        var_guard20_db7_slot: &mut f64,
        var_guard20_dn0_slot: &mut f64,
        var_guard20_dn1_slot: &mut f64,
        var_guard20_dn2_slot: &mut f64,
        var_guard20_dn3_slot: &mut f64,
        var_guard20_dn4_slot: &mut f64,
        var_guard20_dn5_slot: &mut f64,
        var_guard20_dn6_slot: &mut f64,
        var_guard20_dn7_slot: &mut f64,
        var_guard20_dn8_slot: &mut f64,
        var_guard20_dn9_slot: &mut f64,
        var_guard20_rdb0_slot: &mut f64,
        var_guard20_rdb1_slot: &mut f64,
        var_guard20_rdb2_slot: &mut f64,
        var_guard20_rdb3_slot: &mut f64,
        var_guard20_rdb4_slot: &mut f64,
        var_guard20_rdb5_slot: &mut f64,
        var_guard20_rdb6_slot: &mut f64,
        var_guard20_rdb7_slot: &mut f64,
        var_guard20_rdn0_slot: &mut f64,
        var_guard20_rdn1_slot: &mut f64,
        var_guard20_rdn2_slot: &mut f64,
        var_guard20_rdn3_slot: &mut f64,
        var_guard20_rdn4_slot: &mut f64,
        var_guard20_rdn5_slot: &mut f64,
        var_guard20_rdn6_slot: &mut f64,
        var_guard20_rdn7_slot: &mut f64,
        var_guard20_rdn8_slot: &mut f64,
        var_guard20_rdn9_slot: &mut f64,
        var_guard20_rv_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard21_db0_slot: &mut f64,
        var_guard21_db1_slot: &mut f64,
        var_guard21_db2_slot: &mut f64,
        var_guard21_db3_slot: &mut f64,
        var_guard21_db4_slot: &mut f64,
        var_guard21_db5_slot: &mut f64,
        var_guard21_db6_slot: &mut f64,
        var_guard21_db7_slot: &mut f64,
        var_guard21_dn0_slot: &mut f64,
        var_guard21_dn1_slot: &mut f64,
        var_guard21_dn2_slot: &mut f64,
        var_guard21_dn3_slot: &mut f64,
        var_guard21_dn4_slot: &mut f64,
        var_guard21_dn5_slot: &mut f64,
        var_guard21_dn6_slot: &mut f64,
        var_guard21_dn7_slot: &mut f64,
        var_guard21_dn8_slot: &mut f64,
        var_guard21_dn9_slot: &mut f64,
        var_guard21_rdb0_slot: &mut f64,
        var_guard21_rdb1_slot: &mut f64,
        var_guard21_rdb2_slot: &mut f64,
        var_guard21_rdb3_slot: &mut f64,
        var_guard21_rdb4_slot: &mut f64,
        var_guard21_rdb5_slot: &mut f64,
        var_guard21_rdb6_slot: &mut f64,
        var_guard21_rdb7_slot: &mut f64,
        var_guard21_rdn0_slot: &mut f64,
        var_guard21_rdn1_slot: &mut f64,
        var_guard21_rdn2_slot: &mut f64,
        var_guard21_rdn3_slot: &mut f64,
        var_guard21_rdn4_slot: &mut f64,
        var_guard21_rdn5_slot: &mut f64,
        var_guard21_rdn6_slot: &mut f64,
        var_guard21_rdn7_slot: &mut f64,
        var_guard21_rdn8_slot: &mut f64,
        var_guard21_rdn9_slot: &mut f64,
        var_guard21_rv_slot: &mut f64,
        var_qjci_slot: &mut f64,
        var_qjci_1_slot: &mut f64,
        var_qjci_1_db0_slot: &mut f64,
        var_qjci_1_db1_slot: &mut f64,
        var_qjci_1_db2_slot: &mut f64,
        var_qjci_1_db3_slot: &mut f64,
        var_qjci_1_db4_slot: &mut f64,
        var_qjci_1_db5_slot: &mut f64,
        var_qjci_1_db6_slot: &mut f64,
        var_qjci_1_db7_slot: &mut f64,
        var_qjci_1_dn0_slot: &mut f64,
        var_qjci_1_dn1_slot: &mut f64,
        var_qjci_1_dn2_slot: &mut f64,
        var_qjci_1_dn3_slot: &mut f64,
        var_qjci_1_dn4_slot: &mut f64,
        var_qjci_1_dn5_slot: &mut f64,
        var_qjci_1_dn6_slot: &mut f64,
        var_qjci_1_dn7_slot: &mut f64,
        var_qjci_1_dn8_slot: &mut f64,
        var_qjci_1_dn9_slot: &mut f64,
        var_qjci_1_rdb0_slot: &mut f64,
        var_qjci_1_rdb1_slot: &mut f64,
        var_qjci_1_rdb2_slot: &mut f64,
        var_qjci_1_rdb3_slot: &mut f64,
        var_qjci_1_rdb4_slot: &mut f64,
        var_qjci_1_rdb5_slot: &mut f64,
        var_qjci_1_rdb6_slot: &mut f64,
        var_qjci_1_rdb7_slot: &mut f64,
        var_qjci_1_rdn0_slot: &mut f64,
        var_qjci_1_rdn1_slot: &mut f64,
        var_qjci_1_rdn2_slot: &mut f64,
        var_qjci_1_rdn3_slot: &mut f64,
        var_qjci_1_rdn4_slot: &mut f64,
        var_qjci_1_rdn5_slot: &mut f64,
        var_qjci_1_rdn6_slot: &mut f64,
        var_qjci_1_rdn7_slot: &mut f64,
        var_qjci_1_rdn8_slot: &mut f64,
        var_qjci_1_rdn9_slot: &mut f64,
        var_qjci_1_rv_slot: &mut f64,
        var_qjci_db0_slot: &mut f64,
        var_qjci_db1_slot: &mut f64,
        var_qjci_db2_slot: &mut f64,
        var_qjci_db3_slot: &mut f64,
        var_qjci_db4_slot: &mut f64,
        var_qjci_db5_slot: &mut f64,
        var_qjci_db6_slot: &mut f64,
        var_qjci_db7_slot: &mut f64,
        var_qjci_dn0_slot: &mut f64,
        var_qjci_dn1_slot: &mut f64,
        var_qjci_dn2_slot: &mut f64,
        var_qjci_dn3_slot: &mut f64,
        var_qjci_dn4_slot: &mut f64,
        var_qjci_dn5_slot: &mut f64,
        var_qjci_dn6_slot: &mut f64,
        var_qjci_dn7_slot: &mut f64,
        var_qjci_dn8_slot: &mut f64,
        var_qjci_dn9_slot: &mut f64,
        var_qjci_rdb0_slot: &mut f64,
        var_qjci_rdb1_slot: &mut f64,
        var_qjci_rdb2_slot: &mut f64,
        var_qjci_rdb3_slot: &mut f64,
        var_qjci_rdb4_slot: &mut f64,
        var_qjci_rdb5_slot: &mut f64,
        var_qjci_rdb6_slot: &mut f64,
        var_qjci_rdb7_slot: &mut f64,
        var_qjci_rdn0_slot: &mut f64,
        var_qjci_rdn1_slot: &mut f64,
        var_qjci_rdn2_slot: &mut f64,
        var_qjci_rdn3_slot: &mut f64,
        var_qjci_rdn4_slot: &mut f64,
        var_qjci_rdn5_slot: &mut f64,
        var_qjci_rdn6_slot: &mut f64,
        var_qjci_rdn7_slot: &mut f64,
        var_qjci_rdn8_slot: &mut f64,
        var_qjci_rdn9_slot: &mut f64,
        var_qjci_rv_slot: &mut f64,
        var_qxf1_slot: &mut f64,
        var_qxf1_db0_slot: &mut f64,
        var_qxf1_db1_slot: &mut f64,
        var_qxf1_db2_slot: &mut f64,
        var_qxf1_db3_slot: &mut f64,
        var_qxf1_db4_slot: &mut f64,
        var_qxf1_db5_slot: &mut f64,
        var_qxf1_db6_slot: &mut f64,
        var_qxf1_db7_slot: &mut f64,
        var_qxf1_dn0_slot: &mut f64,
        var_qxf1_dn1_slot: &mut f64,
        var_qxf1_dn2_slot: &mut f64,
        var_qxf1_dn3_slot: &mut f64,
        var_qxf1_dn4_slot: &mut f64,
        var_qxf1_dn5_slot: &mut f64,
        var_qxf1_dn6_slot: &mut f64,
        var_qxf1_dn7_slot: &mut f64,
        var_qxf1_dn8_slot: &mut f64,
        var_qxf1_dn9_slot: &mut f64,
        var_qxf1_rdb0_slot: &mut f64,
        var_qxf1_rdb1_slot: &mut f64,
        var_qxf1_rdb2_slot: &mut f64,
        var_qxf1_rdb3_slot: &mut f64,
        var_qxf1_rdb4_slot: &mut f64,
        var_qxf1_rdb5_slot: &mut f64,
        var_qxf1_rdb6_slot: &mut f64,
        var_qxf1_rdb7_slot: &mut f64,
        var_qxf1_rdn0_slot: &mut f64,
        var_qxf1_rdn1_slot: &mut f64,
        var_qxf1_rdn2_slot: &mut f64,
        var_qxf1_rdn3_slot: &mut f64,
        var_qxf1_rdn4_slot: &mut f64,
        var_qxf1_rdn5_slot: &mut f64,
        var_qxf1_rdn6_slot: &mut f64,
        var_qxf1_rdn7_slot: &mut f64,
        var_qxf1_rdn8_slot: &mut f64,
        var_qxf1_rdn9_slot: &mut f64,
        var_qxf1_rv_slot: &mut f64,
    ) {
        let mut var_guard19: f64 = *var_guard19_slot;
        let mut var_guard19_db0: f64 = *var_guard19_db0_slot;
        let mut var_guard19_db1: f64 = *var_guard19_db1_slot;
        let mut var_guard19_db2: f64 = *var_guard19_db2_slot;
        let mut var_guard19_db3: f64 = *var_guard19_db3_slot;
        let mut var_guard19_db4: f64 = *var_guard19_db4_slot;
        let mut var_guard19_db5: f64 = *var_guard19_db5_slot;
        let mut var_guard19_db6: f64 = *var_guard19_db6_slot;
        let mut var_guard19_db7: f64 = *var_guard19_db7_slot;
        let mut var_guard19_dn0: f64 = *var_guard19_dn0_slot;
        let mut var_guard19_dn1: f64 = *var_guard19_dn1_slot;
        let mut var_guard19_dn2: f64 = *var_guard19_dn2_slot;
        let mut var_guard19_dn3: f64 = *var_guard19_dn3_slot;
        let mut var_guard19_dn4: f64 = *var_guard19_dn4_slot;
        let mut var_guard19_dn5: f64 = *var_guard19_dn5_slot;
        let mut var_guard19_dn6: f64 = *var_guard19_dn6_slot;
        let mut var_guard19_dn7: f64 = *var_guard19_dn7_slot;
        let mut var_guard19_dn8: f64 = *var_guard19_dn8_slot;
        let mut var_guard19_dn9: f64 = *var_guard19_dn9_slot;
        let mut var_guard19_rdb0: f64 = *var_guard19_rdb0_slot;
        let mut var_guard19_rdb1: f64 = *var_guard19_rdb1_slot;
        let mut var_guard19_rdb2: f64 = *var_guard19_rdb2_slot;
        let mut var_guard19_rdb3: f64 = *var_guard19_rdb3_slot;
        let mut var_guard19_rdb4: f64 = *var_guard19_rdb4_slot;
        let mut var_guard19_rdb5: f64 = *var_guard19_rdb5_slot;
        let mut var_guard19_rdb6: f64 = *var_guard19_rdb6_slot;
        let mut var_guard19_rdb7: f64 = *var_guard19_rdb7_slot;
        let mut var_guard19_rdn0: f64 = *var_guard19_rdn0_slot;
        let mut var_guard19_rdn1: f64 = *var_guard19_rdn1_slot;
        let mut var_guard19_rdn2: f64 = *var_guard19_rdn2_slot;
        let mut var_guard19_rdn3: f64 = *var_guard19_rdn3_slot;
        let mut var_guard19_rdn4: f64 = *var_guard19_rdn4_slot;
        let mut var_guard19_rdn5: f64 = *var_guard19_rdn5_slot;
        let mut var_guard19_rdn6: f64 = *var_guard19_rdn6_slot;
        let mut var_guard19_rdn7: f64 = *var_guard19_rdn7_slot;
        let mut var_guard19_rdn8: f64 = *var_guard19_rdn8_slot;
        let mut var_guard19_rdn9: f64 = *var_guard19_rdn9_slot;
        let mut var_guard19_rv: f64 = *var_guard19_rv_slot;
        let mut var_guard20: f64 = *var_guard20_slot;
        let mut var_guard20_db0: f64 = *var_guard20_db0_slot;
        let mut var_guard20_db1: f64 = *var_guard20_db1_slot;
        let mut var_guard20_db2: f64 = *var_guard20_db2_slot;
        let mut var_guard20_db3: f64 = *var_guard20_db3_slot;
        let mut var_guard20_db4: f64 = *var_guard20_db4_slot;
        let mut var_guard20_db5: f64 = *var_guard20_db5_slot;
        let mut var_guard20_db6: f64 = *var_guard20_db6_slot;
        let mut var_guard20_db7: f64 = *var_guard20_db7_slot;
        let mut var_guard20_dn0: f64 = *var_guard20_dn0_slot;
        let mut var_guard20_dn1: f64 = *var_guard20_dn1_slot;
        let mut var_guard20_dn2: f64 = *var_guard20_dn2_slot;
        let mut var_guard20_dn3: f64 = *var_guard20_dn3_slot;
        let mut var_guard20_dn4: f64 = *var_guard20_dn4_slot;
        let mut var_guard20_dn5: f64 = *var_guard20_dn5_slot;
        let mut var_guard20_dn6: f64 = *var_guard20_dn6_slot;
        let mut var_guard20_dn7: f64 = *var_guard20_dn7_slot;
        let mut var_guard20_dn8: f64 = *var_guard20_dn8_slot;
        let mut var_guard20_dn9: f64 = *var_guard20_dn9_slot;
        let mut var_guard20_rdb0: f64 = *var_guard20_rdb0_slot;
        let mut var_guard20_rdb1: f64 = *var_guard20_rdb1_slot;
        let mut var_guard20_rdb2: f64 = *var_guard20_rdb2_slot;
        let mut var_guard20_rdb3: f64 = *var_guard20_rdb3_slot;
        let mut var_guard20_rdb4: f64 = *var_guard20_rdb4_slot;
        let mut var_guard20_rdb5: f64 = *var_guard20_rdb5_slot;
        let mut var_guard20_rdb6: f64 = *var_guard20_rdb6_slot;
        let mut var_guard20_rdb7: f64 = *var_guard20_rdb7_slot;
        let mut var_guard20_rdn0: f64 = *var_guard20_rdn0_slot;
        let mut var_guard20_rdn1: f64 = *var_guard20_rdn1_slot;
        let mut var_guard20_rdn2: f64 = *var_guard20_rdn2_slot;
        let mut var_guard20_rdn3: f64 = *var_guard20_rdn3_slot;
        let mut var_guard20_rdn4: f64 = *var_guard20_rdn4_slot;
        let mut var_guard20_rdn5: f64 = *var_guard20_rdn5_slot;
        let mut var_guard20_rdn6: f64 = *var_guard20_rdn6_slot;
        let mut var_guard20_rdn7: f64 = *var_guard20_rdn7_slot;
        let mut var_guard20_rdn8: f64 = *var_guard20_rdn8_slot;
        let mut var_guard20_rdn9: f64 = *var_guard20_rdn9_slot;
        let mut var_guard20_rv: f64 = *var_guard20_rv_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard21_db0: f64 = *var_guard21_db0_slot;
        let mut var_guard21_db1: f64 = *var_guard21_db1_slot;
        let mut var_guard21_db2: f64 = *var_guard21_db2_slot;
        let mut var_guard21_db3: f64 = *var_guard21_db3_slot;
        let mut var_guard21_db4: f64 = *var_guard21_db4_slot;
        let mut var_guard21_db5: f64 = *var_guard21_db5_slot;
        let mut var_guard21_db6: f64 = *var_guard21_db6_slot;
        let mut var_guard21_db7: f64 = *var_guard21_db7_slot;
        let mut var_guard21_dn0: f64 = *var_guard21_dn0_slot;
        let mut var_guard21_dn1: f64 = *var_guard21_dn1_slot;
        let mut var_guard21_dn2: f64 = *var_guard21_dn2_slot;
        let mut var_guard21_dn3: f64 = *var_guard21_dn3_slot;
        let mut var_guard21_dn4: f64 = *var_guard21_dn4_slot;
        let mut var_guard21_dn5: f64 = *var_guard21_dn5_slot;
        let mut var_guard21_dn6: f64 = *var_guard21_dn6_slot;
        let mut var_guard21_dn7: f64 = *var_guard21_dn7_slot;
        let mut var_guard21_dn8: f64 = *var_guard21_dn8_slot;
        let mut var_guard21_dn9: f64 = *var_guard21_dn9_slot;
        let mut var_guard21_rdb0: f64 = *var_guard21_rdb0_slot;
        let mut var_guard21_rdb1: f64 = *var_guard21_rdb1_slot;
        let mut var_guard21_rdb2: f64 = *var_guard21_rdb2_slot;
        let mut var_guard21_rdb3: f64 = *var_guard21_rdb3_slot;
        let mut var_guard21_rdb4: f64 = *var_guard21_rdb4_slot;
        let mut var_guard21_rdb5: f64 = *var_guard21_rdb5_slot;
        let mut var_guard21_rdb6: f64 = *var_guard21_rdb6_slot;
        let mut var_guard21_rdb7: f64 = *var_guard21_rdb7_slot;
        let mut var_guard21_rdn0: f64 = *var_guard21_rdn0_slot;
        let mut var_guard21_rdn1: f64 = *var_guard21_rdn1_slot;
        let mut var_guard21_rdn2: f64 = *var_guard21_rdn2_slot;
        let mut var_guard21_rdn3: f64 = *var_guard21_rdn3_slot;
        let mut var_guard21_rdn4: f64 = *var_guard21_rdn4_slot;
        let mut var_guard21_rdn5: f64 = *var_guard21_rdn5_slot;
        let mut var_guard21_rdn6: f64 = *var_guard21_rdn6_slot;
        let mut var_guard21_rdn7: f64 = *var_guard21_rdn7_slot;
        let mut var_guard21_rdn8: f64 = *var_guard21_rdn8_slot;
        let mut var_guard21_rdn9: f64 = *var_guard21_rdn9_slot;
        let mut var_guard21_rv: f64 = *var_guard21_rv_slot;
        let mut var_qjci: f64 = *var_qjci_slot;
        let mut var_qjci_1: f64 = *var_qjci_1_slot;
        let mut var_qjci_1_db0: f64 = *var_qjci_1_db0_slot;
        let mut var_qjci_1_db1: f64 = *var_qjci_1_db1_slot;
        let mut var_qjci_1_db2: f64 = *var_qjci_1_db2_slot;
        let mut var_qjci_1_db3: f64 = *var_qjci_1_db3_slot;
        let mut var_qjci_1_db4: f64 = *var_qjci_1_db4_slot;
        let mut var_qjci_1_db5: f64 = *var_qjci_1_db5_slot;
        let mut var_qjci_1_db6: f64 = *var_qjci_1_db6_slot;
        let mut var_qjci_1_db7: f64 = *var_qjci_1_db7_slot;
        let mut var_qjci_1_dn0: f64 = *var_qjci_1_dn0_slot;
        let mut var_qjci_1_dn1: f64 = *var_qjci_1_dn1_slot;
        let mut var_qjci_1_dn2: f64 = *var_qjci_1_dn2_slot;
        let mut var_qjci_1_dn3: f64 = *var_qjci_1_dn3_slot;
        let mut var_qjci_1_dn4: f64 = *var_qjci_1_dn4_slot;
        let mut var_qjci_1_dn5: f64 = *var_qjci_1_dn5_slot;
        let mut var_qjci_1_dn6: f64 = *var_qjci_1_dn6_slot;
        let mut var_qjci_1_dn7: f64 = *var_qjci_1_dn7_slot;
        let mut var_qjci_1_dn8: f64 = *var_qjci_1_dn8_slot;
        let mut var_qjci_1_dn9: f64 = *var_qjci_1_dn9_slot;
        let mut var_qjci_1_rdb0: f64 = *var_qjci_1_rdb0_slot;
        let mut var_qjci_1_rdb1: f64 = *var_qjci_1_rdb1_slot;
        let mut var_qjci_1_rdb2: f64 = *var_qjci_1_rdb2_slot;
        let mut var_qjci_1_rdb3: f64 = *var_qjci_1_rdb3_slot;
        let mut var_qjci_1_rdb4: f64 = *var_qjci_1_rdb4_slot;
        let mut var_qjci_1_rdb5: f64 = *var_qjci_1_rdb5_slot;
        let mut var_qjci_1_rdb6: f64 = *var_qjci_1_rdb6_slot;
        let mut var_qjci_1_rdb7: f64 = *var_qjci_1_rdb7_slot;
        let mut var_qjci_1_rdn0: f64 = *var_qjci_1_rdn0_slot;
        let mut var_qjci_1_rdn1: f64 = *var_qjci_1_rdn1_slot;
        let mut var_qjci_1_rdn2: f64 = *var_qjci_1_rdn2_slot;
        let mut var_qjci_1_rdn3: f64 = *var_qjci_1_rdn3_slot;
        let mut var_qjci_1_rdn4: f64 = *var_qjci_1_rdn4_slot;
        let mut var_qjci_1_rdn5: f64 = *var_qjci_1_rdn5_slot;
        let mut var_qjci_1_rdn6: f64 = *var_qjci_1_rdn6_slot;
        let mut var_qjci_1_rdn7: f64 = *var_qjci_1_rdn7_slot;
        let mut var_qjci_1_rdn8: f64 = *var_qjci_1_rdn8_slot;
        let mut var_qjci_1_rdn9: f64 = *var_qjci_1_rdn9_slot;
        let mut var_qjci_1_rv: f64 = *var_qjci_1_rv_slot;
        let mut var_qjci_db0: f64 = *var_qjci_db0_slot;
        let mut var_qjci_db1: f64 = *var_qjci_db1_slot;
        let mut var_qjci_db2: f64 = *var_qjci_db2_slot;
        let mut var_qjci_db3: f64 = *var_qjci_db3_slot;
        let mut var_qjci_db4: f64 = *var_qjci_db4_slot;
        let mut var_qjci_db5: f64 = *var_qjci_db5_slot;
        let mut var_qjci_db6: f64 = *var_qjci_db6_slot;
        let mut var_qjci_db7: f64 = *var_qjci_db7_slot;
        let mut var_qjci_dn0: f64 = *var_qjci_dn0_slot;
        let mut var_qjci_dn1: f64 = *var_qjci_dn1_slot;
        let mut var_qjci_dn2: f64 = *var_qjci_dn2_slot;
        let mut var_qjci_dn3: f64 = *var_qjci_dn3_slot;
        let mut var_qjci_dn4: f64 = *var_qjci_dn4_slot;
        let mut var_qjci_dn5: f64 = *var_qjci_dn5_slot;
        let mut var_qjci_dn6: f64 = *var_qjci_dn6_slot;
        let mut var_qjci_dn7: f64 = *var_qjci_dn7_slot;
        let mut var_qjci_dn8: f64 = *var_qjci_dn8_slot;
        let mut var_qjci_dn9: f64 = *var_qjci_dn9_slot;
        let mut var_qjci_rdb0: f64 = *var_qjci_rdb0_slot;
        let mut var_qjci_rdb1: f64 = *var_qjci_rdb1_slot;
        let mut var_qjci_rdb2: f64 = *var_qjci_rdb2_slot;
        let mut var_qjci_rdb3: f64 = *var_qjci_rdb3_slot;
        let mut var_qjci_rdb4: f64 = *var_qjci_rdb4_slot;
        let mut var_qjci_rdb5: f64 = *var_qjci_rdb5_slot;
        let mut var_qjci_rdb6: f64 = *var_qjci_rdb6_slot;
        let mut var_qjci_rdb7: f64 = *var_qjci_rdb7_slot;
        let mut var_qjci_rdn0: f64 = *var_qjci_rdn0_slot;
        let mut var_qjci_rdn1: f64 = *var_qjci_rdn1_slot;
        let mut var_qjci_rdn2: f64 = *var_qjci_rdn2_slot;
        let mut var_qjci_rdn3: f64 = *var_qjci_rdn3_slot;
        let mut var_qjci_rdn4: f64 = *var_qjci_rdn4_slot;
        let mut var_qjci_rdn5: f64 = *var_qjci_rdn5_slot;
        let mut var_qjci_rdn6: f64 = *var_qjci_rdn6_slot;
        let mut var_qjci_rdn7: f64 = *var_qjci_rdn7_slot;
        let mut var_qjci_rdn8: f64 = *var_qjci_rdn8_slot;
        let mut var_qjci_rdn9: f64 = *var_qjci_rdn9_slot;
        let mut var_qjci_rv: f64 = *var_qjci_rv_slot;
        let mut var_qxf1: f64 = *var_qxf1_slot;
        let mut var_qxf1_db0: f64 = *var_qxf1_db0_slot;
        let mut var_qxf1_db1: f64 = *var_qxf1_db1_slot;
        let mut var_qxf1_db2: f64 = *var_qxf1_db2_slot;
        let mut var_qxf1_db3: f64 = *var_qxf1_db3_slot;
        let mut var_qxf1_db4: f64 = *var_qxf1_db4_slot;
        let mut var_qxf1_db5: f64 = *var_qxf1_db5_slot;
        let mut var_qxf1_db6: f64 = *var_qxf1_db6_slot;
        let mut var_qxf1_db7: f64 = *var_qxf1_db7_slot;
        let mut var_qxf1_dn0: f64 = *var_qxf1_dn0_slot;
        let mut var_qxf1_dn1: f64 = *var_qxf1_dn1_slot;
        let mut var_qxf1_dn2: f64 = *var_qxf1_dn2_slot;
        let mut var_qxf1_dn3: f64 = *var_qxf1_dn3_slot;
        let mut var_qxf1_dn4: f64 = *var_qxf1_dn4_slot;
        let mut var_qxf1_dn5: f64 = *var_qxf1_dn5_slot;
        let mut var_qxf1_dn6: f64 = *var_qxf1_dn6_slot;
        let mut var_qxf1_dn7: f64 = *var_qxf1_dn7_slot;
        let mut var_qxf1_dn8: f64 = *var_qxf1_dn8_slot;
        let mut var_qxf1_dn9: f64 = *var_qxf1_dn9_slot;
        let mut var_qxf1_rdb0: f64 = *var_qxf1_rdb0_slot;
        let mut var_qxf1_rdb1: f64 = *var_qxf1_rdb1_slot;
        let mut var_qxf1_rdb2: f64 = *var_qxf1_rdb2_slot;
        let mut var_qxf1_rdb3: f64 = *var_qxf1_rdb3_slot;
        let mut var_qxf1_rdb4: f64 = *var_qxf1_rdb4_slot;
        let mut var_qxf1_rdb5: f64 = *var_qxf1_rdb5_slot;
        let mut var_qxf1_rdb6: f64 = *var_qxf1_rdb6_slot;
        let mut var_qxf1_rdb7: f64 = *var_qxf1_rdb7_slot;
        let mut var_qxf1_rdn0: f64 = *var_qxf1_rdn0_slot;
        let mut var_qxf1_rdn1: f64 = *var_qxf1_rdn1_slot;
        let mut var_qxf1_rdn2: f64 = *var_qxf1_rdn2_slot;
        let mut var_qxf1_rdn3: f64 = *var_qxf1_rdn3_slot;
        let mut var_qxf1_rdn4: f64 = *var_qxf1_rdn4_slot;
        let mut var_qxf1_rdn5: f64 = *var_qxf1_rdn5_slot;
        let mut var_qxf1_rdn6: f64 = *var_qxf1_rdn6_slot;
        let mut var_qxf1_rdn7: f64 = *var_qxf1_rdn7_slot;
        let mut var_qxf1_rdn8: f64 = *var_qxf1_rdn8_slot;
        let mut var_qxf1_rdn9: f64 = *var_qxf1_rdn9_slot;
        let mut var_qxf1_rv: f64 = *var_qxf1_rv_slot;

        let assign1870_e2179: f64 = (var_qlo + var_qhi);
        let assign1870_e2180: f64 = (var_cjc_t * assign1870_e2179);
        var_qjci = assign1870_e2180;
        var_qjci_dn0 = ((var_cjc_t_dn0 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn0 + var_qhi_dn0)));
        var_qjci_dn1 = ((var_cjc_t_dn1 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn1 + var_qhi_dn1)));
        var_qjci_dn2 = ((var_cjc_t_dn2 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn2 + var_qhi_dn2)));
        var_qjci_dn3 = ((var_cjc_t_dn3 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn3 + var_qhi_dn3)));
        var_qjci_dn4 = ((var_cjc_t_dn4 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn4 + var_qhi_dn4)));
        var_qjci_dn5 = ((var_cjc_t_dn5 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn5 + var_qhi_dn5)));
        var_qjci_dn6 = ((var_cjc_t_dn6 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn6 + var_qhi_dn6)));
        var_qjci_dn7 = ((var_cjc_t_dn7 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn7 + var_qhi_dn7)));
        var_qjci_dn8 = ((var_cjc_t_dn8 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn8 + var_qhi_dn8)));
        var_qjci_dn9 = ((var_cjc_t_dn9 * assign1870_e2179) + (var_cjc_t * (var_qlo_dn9 + var_qhi_dn9)));
        var_qjci_db0 = ((var_cjc_t_db0 * assign1870_e2179) + (var_cjc_t * (var_qlo_db0 + var_qhi_db0)));
        var_qjci_db1 = ((var_cjc_t_db1 * assign1870_e2179) + (var_cjc_t * (var_qlo_db1 + var_qhi_db1)));
        var_qjci_db2 = ((var_cjc_t_db2 * assign1870_e2179) + (var_cjc_t * (var_qlo_db2 + var_qhi_db2)));
        var_qjci_db3 = ((var_cjc_t_db3 * assign1870_e2179) + (var_cjc_t * (var_qlo_db3 + var_qhi_db3)));
        var_qjci_db4 = ((var_cjc_t_db4 * assign1870_e2179) + (var_cjc_t * (var_qlo_db4 + var_qhi_db4)));
        var_qjci_db5 = ((var_cjc_t_db5 * assign1870_e2179) + (var_cjc_t * (var_qlo_db5 + var_qhi_db5)));
        var_qjci_db6 = ((var_cjc_t_db6 * assign1870_e2179) + (var_cjc_t * (var_qlo_db6 + var_qhi_db6)));
        var_qjci_db7 = ((var_cjc_t_db7 * assign1870_e2179) + (var_cjc_t * (var_qlo_db7 + var_qhi_db7)));
        var_qjci_rv = 0.0;
        var_qjci_rdn0 = 0.0;
        var_qjci_rdn1 = 0.0;
        var_qjci_rdn2 = 0.0;
        var_qjci_rdn3 = 0.0;
        var_qjci_rdn4 = 0.0;
        var_qjci_rdn5 = 0.0;
        var_qjci_rdn6 = 0.0;
        var_qjci_rdn7 = 0.0;
        var_qjci_rdn8 = 0.0;
        var_qjci_rdn9 = 0.0;
        var_qjci_rdb0 = 0.0;
        var_qjci_rdb1 = 0.0;
        var_qjci_rdb2 = 0.0;
        var_qjci_rdb3 = 0.0;
        var_qjci_rdb4 = 0.0;
        var_qjci_rdb5 = 0.0;
        var_qjci_rdb6 = 0.0;
        var_qjci_rdb7 = 0.0;

        let assign1880_e2183: f64 = (p.p72 * var_qjci);
        var_qjci_1 = assign1880_e2183;
        var_qjci_1_dn0 = (p.p72 * var_qjci_dn0);
        var_qjci_1_dn1 = (p.p72 * var_qjci_dn1);
        var_qjci_1_dn2 = (p.p72 * var_qjci_dn2);
        var_qjci_1_dn3 = (p.p72 * var_qjci_dn3);
        var_qjci_1_dn4 = (p.p72 * var_qjci_dn4);
        var_qjci_1_dn5 = (p.p72 * var_qjci_dn5);
        var_qjci_1_dn6 = (p.p72 * var_qjci_dn6);
        var_qjci_1_dn7 = (p.p72 * var_qjci_dn7);
        var_qjci_1_dn8 = (p.p72 * var_qjci_dn8);
        var_qjci_1_dn9 = (p.p72 * var_qjci_dn9);
        var_qjci_1_db0 = (p.p72 * var_qjci_db0);
        var_qjci_1_db1 = (p.p72 * var_qjci_db1);
        var_qjci_1_db2 = (p.p72 * var_qjci_db2);
        var_qjci_1_db3 = (p.p72 * var_qjci_db3);
        var_qjci_1_db4 = (p.p72 * var_qjci_db4);
        var_qjci_1_db5 = (p.p72 * var_qjci_db5);
        var_qjci_1_db6 = (p.p72 * var_qjci_db6);
        var_qjci_1_db7 = (p.p72 * var_qjci_db7);
        var_qjci_1_rv = 0.0;
        var_qjci_1_rdn0 = 0.0;
        var_qjci_1_rdn1 = 0.0;
        var_qjci_1_rdn2 = 0.0;
        var_qjci_1_rdn3 = 0.0;
        var_qjci_1_rdn4 = 0.0;
        var_qjci_1_rdn5 = 0.0;
        var_qjci_1_rdn6 = 0.0;
        var_qjci_1_rdn7 = 0.0;
        var_qjci_1_rdn8 = 0.0;
        var_qjci_1_rdn9 = 0.0;
        var_qjci_1_rdb0 = 0.0;
        var_qjci_1_rdb1 = 0.0;
        var_qjci_1_rdb2 = 0.0;
        var_qjci_1_rdb3 = 0.0;
        var_qjci_1_rdb4 = 0.0;
        var_qjci_1_rdb5 = 0.0;
        var_qjci_1_rdb6 = 0.0;
        var_qjci_1_rdb7 = 0.0;

        let assign1890_e2190: f64 = if ((p.p68 != 0.0) && (p.p19 != 0.0)) { 1.0 } else { 0.0 };
        var_guard19 = assign1890_e2190;
        var_guard19_dn0 = 0.0;
        var_guard19_dn1 = 0.0;
        var_guard19_dn2 = 0.0;
        var_guard19_dn3 = 0.0;
        var_guard19_dn4 = 0.0;
        var_guard19_dn5 = 0.0;
        var_guard19_dn6 = 0.0;
        var_guard19_dn7 = 0.0;
        var_guard19_dn8 = 0.0;
        var_guard19_dn9 = 0.0;
        var_guard19_db0 = 0.0;
        var_guard19_db1 = 0.0;
        var_guard19_db2 = 0.0;
        var_guard19_db3 = 0.0;
        var_guard19_db4 = 0.0;
        var_guard19_db5 = 0.0;
        var_guard19_db6 = 0.0;
        var_guard19_db7 = 0.0;
        var_guard19_rv = 0.0;
        var_guard19_rdn0 = 0.0;
        var_guard19_rdn1 = 0.0;
        var_guard19_rdn2 = 0.0;
        var_guard19_rdn3 = 0.0;
        var_guard19_rdn4 = 0.0;
        var_guard19_rdn5 = 0.0;
        var_guard19_rdn6 = 0.0;
        var_guard19_rdn7 = 0.0;
        var_guard19_rdn8 = 0.0;
        var_guard19_rdn9 = 0.0;
        var_guard19_rdb0 = 0.0;
        var_guard19_rdb1 = 0.0;
        var_guard19_rdb2 = 0.0;
        var_guard19_rdb3 = 0.0;
        var_guard19_rdb4 = 0.0;
        var_guard19_rdb5 = 0.0;
        var_guard19_rdb6 = 0.0;
        var_guard19_rdb7 = 0.0;

        let (assign1900_e2204, assign1900_e2204_d_n0, assign1900_e2204_d_n1, assign1900_e2204_d_n2, assign1900_e2204_d_n3, assign1900_e2204_d_n4, assign1900_e2204_d_n5, assign1900_e2204_d_n6, assign1900_e2204_d_n7, assign1900_e2204_d_n8, assign1900_e2204_d_n9, assign1900_e2204_d_b0, assign1900_e2204_d_b1, assign1900_e2204_d_b2, assign1900_e2204_d_b3, assign1900_e2204_d_b4, assign1900_e2204_d_b5, assign1900_e2204_d_b6, assign1900_e2204_d_b7,) = {
    if (var_guard19 != 0.0) {
        let assign1900_e2194: f64 = (var_ttype * p.p68);
        let assign1900_e2196: f64 = (assign1900_e2194 * 3.141592653589793);
        let assign1900_e2198: f64 = (assign1900_e2196 / 180.0);
        let assign1900_e2200: f64 = (assign1900_e2198 * p.p19);
        let assign1900_e2202: f64 = (assign1900_e2200 * var_itzf);
        (assign1900_e2202, ((((((var_ttype_dn0 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn0)), ((((((var_ttype_dn1 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn1)), ((((((var_ttype_dn2 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn2)), ((((((var_ttype_dn3 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn3)), ((((((var_ttype_dn4 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn4)), ((((((var_ttype_dn5 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn5)), ((((((var_ttype_dn6 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn6)), ((((((var_ttype_dn7 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn7)), ((((((var_ttype_dn8 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn8)), ((((((var_ttype_dn9 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_dn9)), ((((((var_ttype_db0 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db0)), ((((((var_ttype_db1 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db1)), ((((((var_ttype_db2 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db2)), ((((((var_ttype_db3 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db3)), ((((((var_ttype_db4 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db4)), ((((((var_ttype_db5 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db5)), ((((((var_ttype_db6 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db6)), ((((((var_ttype_db7 * p.p68) * 3.141592653589793) / 180.0) * p.p19) * var_itzf) + (assign1900_e2200 * var_itzf_db7)),)
    } else {
        (var_qxf1, var_qxf1_dn0, var_qxf1_dn1, var_qxf1_dn2, var_qxf1_dn3, var_qxf1_dn4, var_qxf1_dn5, var_qxf1_dn6, var_qxf1_dn7, var_qxf1_dn8, var_qxf1_dn9, var_qxf1_db0, var_qxf1_db1, var_qxf1_db2, var_qxf1_db3, var_qxf1_db4, var_qxf1_db5, var_qxf1_db6, var_qxf1_db7,)
    }
};
        var_qxf1 = assign1900_e2204;
        var_qxf1_dn0 = assign1900_e2204_d_n0;
        var_qxf1_dn1 = assign1900_e2204_d_n1;
        var_qxf1_dn2 = assign1900_e2204_d_n2;
        var_qxf1_dn3 = assign1900_e2204_d_n3;
        var_qxf1_dn4 = assign1900_e2204_d_n4;
        var_qxf1_dn5 = assign1900_e2204_d_n5;
        var_qxf1_dn6 = assign1900_e2204_d_n6;
        var_qxf1_dn7 = assign1900_e2204_d_n7;
        var_qxf1_dn8 = assign1900_e2204_d_n8;
        var_qxf1_dn9 = assign1900_e2204_d_n9;
        var_qxf1_db0 = assign1900_e2204_d_b0;
        var_qxf1_db1 = assign1900_e2204_d_b1;
        var_qxf1_db2 = assign1900_e2204_d_b2;
        var_qxf1_db3 = assign1900_e2204_d_b3;
        var_qxf1_db4 = assign1900_e2204_d_b4;
        var_qxf1_db5 = assign1900_e2204_d_b5;
        var_qxf1_db6 = assign1900_e2204_d_b6;
        var_qxf1_db7 = assign1900_e2204_d_b7;
        var_qxf1_rv = 0.0;
        var_qxf1_rdn0 = 0.0;
        var_qxf1_rdn1 = 0.0;
        var_qxf1_rdn2 = 0.0;
        var_qxf1_rdn3 = 0.0;
        var_qxf1_rdn4 = 0.0;
        var_qxf1_rdn5 = 0.0;
        var_qxf1_rdn6 = 0.0;
        var_qxf1_rdn7 = 0.0;
        var_qxf1_rdn8 = 0.0;
        var_qxf1_rdn9 = 0.0;
        var_qxf1_rdb0 = 0.0;
        var_qxf1_rdb1 = 0.0;
        var_qxf1_rdb2 = 0.0;
        var_qxf1_rdb3 = 0.0;
        var_qxf1_rdb4 = 0.0;
        var_qxf1_rdb5 = 0.0;
        var_qxf1_rdb6 = 0.0;
        var_qxf1_rdb7 = 0.0;

        let (assign1910_e2209, assign1910_e2209_d_n0, assign1910_e2209_d_n1, assign1910_e2209_d_n2, assign1910_e2209_d_n3, assign1910_e2209_d_n4, assign1910_e2209_d_n5, assign1910_e2209_d_n6, assign1910_e2209_d_n7, assign1910_e2209_d_n8, assign1910_e2209_d_n9, assign1910_e2209_d_b0, assign1910_e2209_d_b1, assign1910_e2209_d_b2, assign1910_e2209_d_b3, assign1910_e2209_d_b4, assign1910_e2209_d_b5, assign1910_e2209_d_b6, assign1910_e2209_d_b7,) = {
    if (var_guard19 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qxf1, var_qxf1_dn0, var_qxf1_dn1, var_qxf1_dn2, var_qxf1_dn3, var_qxf1_dn4, var_qxf1_dn5, var_qxf1_dn6, var_qxf1_dn7, var_qxf1_dn8, var_qxf1_dn9, var_qxf1_db0, var_qxf1_db1, var_qxf1_db2, var_qxf1_db3, var_qxf1_db4, var_qxf1_db5, var_qxf1_db6, var_qxf1_db7,)
    }
};
        var_qxf1 = assign1910_e2209;
        var_qxf1_dn0 = assign1910_e2209_d_n0;
        var_qxf1_dn1 = assign1910_e2209_d_n1;
        var_qxf1_dn2 = assign1910_e2209_d_n2;
        var_qxf1_dn3 = assign1910_e2209_d_n3;
        var_qxf1_dn4 = assign1910_e2209_d_n4;
        var_qxf1_dn5 = assign1910_e2209_d_n5;
        var_qxf1_dn6 = assign1910_e2209_d_n6;
        var_qxf1_dn7 = assign1910_e2209_d_n7;
        var_qxf1_dn8 = assign1910_e2209_d_n8;
        var_qxf1_dn9 = assign1910_e2209_d_n9;
        var_qxf1_db0 = assign1910_e2209_d_b0;
        var_qxf1_db1 = assign1910_e2209_d_b1;
        var_qxf1_db2 = assign1910_e2209_d_b2;
        var_qxf1_db3 = assign1910_e2209_d_b3;
        var_qxf1_db4 = assign1910_e2209_d_b4;
        var_qxf1_db5 = assign1910_e2209_d_b5;
        var_qxf1_db6 = assign1910_e2209_d_b6;
        var_qxf1_db7 = assign1910_e2209_d_b7;
        var_qxf1_rv = 0.0;
        var_qxf1_rdn0 = 0.0;
        var_qxf1_rdn1 = 0.0;
        var_qxf1_rdn2 = 0.0;
        var_qxf1_rdn3 = 0.0;
        var_qxf1_rdn4 = 0.0;
        var_qxf1_rdn5 = 0.0;
        var_qxf1_rdn6 = 0.0;
        var_qxf1_rdn7 = 0.0;
        var_qxf1_rdn8 = 0.0;
        var_qxf1_rdn9 = 0.0;
        var_qxf1_rdb0 = 0.0;
        var_qxf1_rdb1 = 0.0;
        var_qxf1_rdb2 = 0.0;
        var_qxf1_rdb3 = 0.0;
        var_qxf1_rdb4 = 0.0;
        var_qxf1_rdb5 = 0.0;
        var_qxf1_rdb6 = 0.0;
        var_qxf1_rdb7 = 0.0;

        let assign1920_e2216: f64 = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        var_guard20 = assign1920_e2216;
        var_guard20_dn0 = 0.0;
        var_guard20_dn1 = 0.0;
        var_guard20_dn2 = 0.0;
        var_guard20_dn3 = 0.0;
        var_guard20_dn4 = 0.0;
        var_guard20_dn5 = 0.0;
        var_guard20_dn6 = 0.0;
        var_guard20_dn7 = 0.0;
        var_guard20_dn8 = 0.0;
        var_guard20_dn9 = 0.0;
        var_guard20_db0 = 0.0;
        var_guard20_db1 = 0.0;
        var_guard20_db2 = 0.0;
        var_guard20_db3 = 0.0;
        var_guard20_db4 = 0.0;
        var_guard20_db5 = 0.0;
        var_guard20_db6 = 0.0;
        var_guard20_db7 = 0.0;
        var_guard20_rv = 0.0;
        var_guard20_rdn0 = 0.0;
        var_guard20_rdn1 = 0.0;
        var_guard20_rdn2 = 0.0;
        var_guard20_rdn3 = 0.0;
        var_guard20_rdn4 = 0.0;
        var_guard20_rdn5 = 0.0;
        var_guard20_rdn6 = 0.0;
        var_guard20_rdn7 = 0.0;
        var_guard20_rdn8 = 0.0;
        var_guard20_rdn9 = 0.0;
        var_guard20_rdb0 = 0.0;
        var_guard20_rdb1 = 0.0;
        var_guard20_rdb2 = 0.0;
        var_guard20_rdb3 = 0.0;
        var_guard20_rdb4 = 0.0;
        var_guard20_rdb5 = 0.0;
        var_guard20_rdb6 = 0.0;
        var_guard20_rdb7 = 0.0;

        let assign1930_e2227: f64 = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };
        var_guard21 = assign1930_e2227;
        var_guard21_dn0 = 0.0;
        var_guard21_dn1 = 0.0;
        var_guard21_dn2 = 0.0;
        var_guard21_dn3 = 0.0;
        var_guard21_dn4 = 0.0;
        var_guard21_dn5 = 0.0;
        var_guard21_dn6 = 0.0;
        var_guard21_dn7 = 0.0;
        var_guard21_dn8 = 0.0;
        var_guard21_dn9 = 0.0;
        var_guard21_db0 = 0.0;
        var_guard21_db1 = 0.0;
        var_guard21_db2 = 0.0;
        var_guard21_db3 = 0.0;
        var_guard21_db4 = 0.0;
        var_guard21_db5 = 0.0;
        var_guard21_db6 = 0.0;
        var_guard21_db7 = 0.0;
        var_guard21_rv = 0.0;
        var_guard21_rdn0 = 0.0;
        var_guard21_rdn1 = 0.0;
        var_guard21_rdn2 = 0.0;
        var_guard21_rdn3 = 0.0;
        var_guard21_rdn4 = 0.0;
        var_guard21_rdn5 = 0.0;
        var_guard21_rdn6 = 0.0;
        var_guard21_rdn7 = 0.0;
        var_guard21_rdn8 = 0.0;
        var_guard21_rdn9 = 0.0;
        var_guard21_rdb0 = 0.0;
        var_guard21_rdb1 = 0.0;
        var_guard21_rdb2 = 0.0;
        var_guard21_rdb3 = 0.0;
        var_guard21_rdb4 = 0.0;
        var_guard21_rdb5 = 0.0;
        var_guard21_rdb6 = 0.0;
        var_guard21_rdb7 = 0.0;

        *var_guard19_slot = var_guard19;
        *var_guard19_db0_slot = var_guard19_db0;
        *var_guard19_db1_slot = var_guard19_db1;
        *var_guard19_db2_slot = var_guard19_db2;
        *var_guard19_db3_slot = var_guard19_db3;
        *var_guard19_db4_slot = var_guard19_db4;
        *var_guard19_db5_slot = var_guard19_db5;
        *var_guard19_db6_slot = var_guard19_db6;
        *var_guard19_db7_slot = var_guard19_db7;
        *var_guard19_dn0_slot = var_guard19_dn0;
        *var_guard19_dn1_slot = var_guard19_dn1;
        *var_guard19_dn2_slot = var_guard19_dn2;
        *var_guard19_dn3_slot = var_guard19_dn3;
        *var_guard19_dn4_slot = var_guard19_dn4;
        *var_guard19_dn5_slot = var_guard19_dn5;
        *var_guard19_dn6_slot = var_guard19_dn6;
        *var_guard19_dn7_slot = var_guard19_dn7;
        *var_guard19_dn8_slot = var_guard19_dn8;
        *var_guard19_dn9_slot = var_guard19_dn9;
        *var_guard19_rdb0_slot = var_guard19_rdb0;
        *var_guard19_rdb1_slot = var_guard19_rdb1;
        *var_guard19_rdb2_slot = var_guard19_rdb2;
        *var_guard19_rdb3_slot = var_guard19_rdb3;
        *var_guard19_rdb4_slot = var_guard19_rdb4;
        *var_guard19_rdb5_slot = var_guard19_rdb5;
        *var_guard19_rdb6_slot = var_guard19_rdb6;
        *var_guard19_rdb7_slot = var_guard19_rdb7;
        *var_guard19_rdn0_slot = var_guard19_rdn0;
        *var_guard19_rdn1_slot = var_guard19_rdn1;
        *var_guard19_rdn2_slot = var_guard19_rdn2;
        *var_guard19_rdn3_slot = var_guard19_rdn3;
        *var_guard19_rdn4_slot = var_guard19_rdn4;
        *var_guard19_rdn5_slot = var_guard19_rdn5;
        *var_guard19_rdn6_slot = var_guard19_rdn6;
        *var_guard19_rdn7_slot = var_guard19_rdn7;
        *var_guard19_rdn8_slot = var_guard19_rdn8;
        *var_guard19_rdn9_slot = var_guard19_rdn9;
        *var_guard19_rv_slot = var_guard19_rv;
        *var_guard20_slot = var_guard20;
        *var_guard20_db0_slot = var_guard20_db0;
        *var_guard20_db1_slot = var_guard20_db1;
        *var_guard20_db2_slot = var_guard20_db2;
        *var_guard20_db3_slot = var_guard20_db3;
        *var_guard20_db4_slot = var_guard20_db4;
        *var_guard20_db5_slot = var_guard20_db5;
        *var_guard20_db6_slot = var_guard20_db6;
        *var_guard20_db7_slot = var_guard20_db7;
        *var_guard20_dn0_slot = var_guard20_dn0;
        *var_guard20_dn1_slot = var_guard20_dn1;
        *var_guard20_dn2_slot = var_guard20_dn2;
        *var_guard20_dn3_slot = var_guard20_dn3;
        *var_guard20_dn4_slot = var_guard20_dn4;
        *var_guard20_dn5_slot = var_guard20_dn5;
        *var_guard20_dn6_slot = var_guard20_dn6;
        *var_guard20_dn7_slot = var_guard20_dn7;
        *var_guard20_dn8_slot = var_guard20_dn8;
        *var_guard20_dn9_slot = var_guard20_dn9;
        *var_guard20_rdb0_slot = var_guard20_rdb0;
        *var_guard20_rdb1_slot = var_guard20_rdb1;
        *var_guard20_rdb2_slot = var_guard20_rdb2;
        *var_guard20_rdb3_slot = var_guard20_rdb3;
        *var_guard20_rdb4_slot = var_guard20_rdb4;
        *var_guard20_rdb5_slot = var_guard20_rdb5;
        *var_guard20_rdb6_slot = var_guard20_rdb6;
        *var_guard20_rdb7_slot = var_guard20_rdb7;
        *var_guard20_rdn0_slot = var_guard20_rdn0;
        *var_guard20_rdn1_slot = var_guard20_rdn1;
        *var_guard20_rdn2_slot = var_guard20_rdn2;
        *var_guard20_rdn3_slot = var_guard20_rdn3;
        *var_guard20_rdn4_slot = var_guard20_rdn4;
        *var_guard20_rdn5_slot = var_guard20_rdn5;
        *var_guard20_rdn6_slot = var_guard20_rdn6;
        *var_guard20_rdn7_slot = var_guard20_rdn7;
        *var_guard20_rdn8_slot = var_guard20_rdn8;
        *var_guard20_rdn9_slot = var_guard20_rdn9;
        *var_guard20_rv_slot = var_guard20_rv;
        *var_guard21_slot = var_guard21;
        *var_guard21_db0_slot = var_guard21_db0;
        *var_guard21_db1_slot = var_guard21_db1;
        *var_guard21_db2_slot = var_guard21_db2;
        *var_guard21_db3_slot = var_guard21_db3;
        *var_guard21_db4_slot = var_guard21_db4;
        *var_guard21_db5_slot = var_guard21_db5;
        *var_guard21_db6_slot = var_guard21_db6;
        *var_guard21_db7_slot = var_guard21_db7;
        *var_guard21_dn0_slot = var_guard21_dn0;
        *var_guard21_dn1_slot = var_guard21_dn1;
        *var_guard21_dn2_slot = var_guard21_dn2;
        *var_guard21_dn3_slot = var_guard21_dn3;
        *var_guard21_dn4_slot = var_guard21_dn4;
        *var_guard21_dn5_slot = var_guard21_dn5;
        *var_guard21_dn6_slot = var_guard21_dn6;
        *var_guard21_dn7_slot = var_guard21_dn7;
        *var_guard21_dn8_slot = var_guard21_dn8;
        *var_guard21_dn9_slot = var_guard21_dn9;
        *var_guard21_rdb0_slot = var_guard21_rdb0;
        *var_guard21_rdb1_slot = var_guard21_rdb1;
        *var_guard21_rdb2_slot = var_guard21_rdb2;
        *var_guard21_rdb3_slot = var_guard21_rdb3;
        *var_guard21_rdb4_slot = var_guard21_rdb4;
        *var_guard21_rdb5_slot = var_guard21_rdb5;
        *var_guard21_rdb6_slot = var_guard21_rdb6;
        *var_guard21_rdb7_slot = var_guard21_rdb7;
        *var_guard21_rdn0_slot = var_guard21_rdn0;
        *var_guard21_rdn1_slot = var_guard21_rdn1;
        *var_guard21_rdn2_slot = var_guard21_rdn2;
        *var_guard21_rdn3_slot = var_guard21_rdn3;
        *var_guard21_rdn4_slot = var_guard21_rdn4;
        *var_guard21_rdn5_slot = var_guard21_rdn5;
        *var_guard21_rdn6_slot = var_guard21_rdn6;
        *var_guard21_rdn7_slot = var_guard21_rdn7;
        *var_guard21_rdn8_slot = var_guard21_rdn8;
        *var_guard21_rdn9_slot = var_guard21_rdn9;
        *var_guard21_rv_slot = var_guard21_rv;
        *var_qjci_slot = var_qjci;
        *var_qjci_1_slot = var_qjci_1;
        *var_qjci_1_db0_slot = var_qjci_1_db0;
        *var_qjci_1_db1_slot = var_qjci_1_db1;
        *var_qjci_1_db2_slot = var_qjci_1_db2;
        *var_qjci_1_db3_slot = var_qjci_1_db3;
        *var_qjci_1_db4_slot = var_qjci_1_db4;
        *var_qjci_1_db5_slot = var_qjci_1_db5;
        *var_qjci_1_db6_slot = var_qjci_1_db6;
        *var_qjci_1_db7_slot = var_qjci_1_db7;
        *var_qjci_1_dn0_slot = var_qjci_1_dn0;
        *var_qjci_1_dn1_slot = var_qjci_1_dn1;
        *var_qjci_1_dn2_slot = var_qjci_1_dn2;
        *var_qjci_1_dn3_slot = var_qjci_1_dn3;
        *var_qjci_1_dn4_slot = var_qjci_1_dn4;
        *var_qjci_1_dn5_slot = var_qjci_1_dn5;
        *var_qjci_1_dn6_slot = var_qjci_1_dn6;
        *var_qjci_1_dn7_slot = var_qjci_1_dn7;
        *var_qjci_1_dn8_slot = var_qjci_1_dn8;
        *var_qjci_1_dn9_slot = var_qjci_1_dn9;
        *var_qjci_1_rdb0_slot = var_qjci_1_rdb0;
        *var_qjci_1_rdb1_slot = var_qjci_1_rdb1;
        *var_qjci_1_rdb2_slot = var_qjci_1_rdb2;
        *var_qjci_1_rdb3_slot = var_qjci_1_rdb3;
        *var_qjci_1_rdb4_slot = var_qjci_1_rdb4;
        *var_qjci_1_rdb5_slot = var_qjci_1_rdb5;
        *var_qjci_1_rdb6_slot = var_qjci_1_rdb6;
        *var_qjci_1_rdb7_slot = var_qjci_1_rdb7;
        *var_qjci_1_rdn0_slot = var_qjci_1_rdn0;
        *var_qjci_1_rdn1_slot = var_qjci_1_rdn1;
        *var_qjci_1_rdn2_slot = var_qjci_1_rdn2;
        *var_qjci_1_rdn3_slot = var_qjci_1_rdn3;
        *var_qjci_1_rdn4_slot = var_qjci_1_rdn4;
        *var_qjci_1_rdn5_slot = var_qjci_1_rdn5;
        *var_qjci_1_rdn6_slot = var_qjci_1_rdn6;
        *var_qjci_1_rdn7_slot = var_qjci_1_rdn7;
        *var_qjci_1_rdn8_slot = var_qjci_1_rdn8;
        *var_qjci_1_rdn9_slot = var_qjci_1_rdn9;
        *var_qjci_1_rv_slot = var_qjci_1_rv;
        *var_qjci_db0_slot = var_qjci_db0;
        *var_qjci_db1_slot = var_qjci_db1;
        *var_qjci_db2_slot = var_qjci_db2;
        *var_qjci_db3_slot = var_qjci_db3;
        *var_qjci_db4_slot = var_qjci_db4;
        *var_qjci_db5_slot = var_qjci_db5;
        *var_qjci_db6_slot = var_qjci_db6;
        *var_qjci_db7_slot = var_qjci_db7;
        *var_qjci_dn0_slot = var_qjci_dn0;
        *var_qjci_dn1_slot = var_qjci_dn1;
        *var_qjci_dn2_slot = var_qjci_dn2;
        *var_qjci_dn3_slot = var_qjci_dn3;
        *var_qjci_dn4_slot = var_qjci_dn4;
        *var_qjci_dn5_slot = var_qjci_dn5;
        *var_qjci_dn6_slot = var_qjci_dn6;
        *var_qjci_dn7_slot = var_qjci_dn7;
        *var_qjci_dn8_slot = var_qjci_dn8;
        *var_qjci_dn9_slot = var_qjci_dn9;
        *var_qjci_rdb0_slot = var_qjci_rdb0;
        *var_qjci_rdb1_slot = var_qjci_rdb1;
        *var_qjci_rdb2_slot = var_qjci_rdb2;
        *var_qjci_rdb3_slot = var_qjci_rdb3;
        *var_qjci_rdb4_slot = var_qjci_rdb4;
        *var_qjci_rdb5_slot = var_qjci_rdb5;
        *var_qjci_rdb6_slot = var_qjci_rdb6;
        *var_qjci_rdb7_slot = var_qjci_rdb7;
        *var_qjci_rdn0_slot = var_qjci_rdn0;
        *var_qjci_rdn1_slot = var_qjci_rdn1;
        *var_qjci_rdn2_slot = var_qjci_rdn2;
        *var_qjci_rdn3_slot = var_qjci_rdn3;
        *var_qjci_rdn4_slot = var_qjci_rdn4;
        *var_qjci_rdn5_slot = var_qjci_rdn5;
        *var_qjci_rdn6_slot = var_qjci_rdn6;
        *var_qjci_rdn7_slot = var_qjci_rdn7;
        *var_qjci_rdn8_slot = var_qjci_rdn8;
        *var_qjci_rdn9_slot = var_qjci_rdn9;
        *var_qjci_rv_slot = var_qjci_rv;
        *var_qxf1_slot = var_qxf1;
        *var_qxf1_db0_slot = var_qxf1_db0;
        *var_qxf1_db1_slot = var_qxf1_db1;
        *var_qxf1_db2_slot = var_qxf1_db2;
        *var_qxf1_db3_slot = var_qxf1_db3;
        *var_qxf1_db4_slot = var_qxf1_db4;
        *var_qxf1_db5_slot = var_qxf1_db5;
        *var_qxf1_db6_slot = var_qxf1_db6;
        *var_qxf1_db7_slot = var_qxf1_db7;
        *var_qxf1_dn0_slot = var_qxf1_dn0;
        *var_qxf1_dn1_slot = var_qxf1_dn1;
        *var_qxf1_dn2_slot = var_qxf1_dn2;
        *var_qxf1_dn3_slot = var_qxf1_dn3;
        *var_qxf1_dn4_slot = var_qxf1_dn4;
        *var_qxf1_dn5_slot = var_qxf1_dn5;
        *var_qxf1_dn6_slot = var_qxf1_dn6;
        *var_qxf1_dn7_slot = var_qxf1_dn7;
        *var_qxf1_dn8_slot = var_qxf1_dn8;
        *var_qxf1_dn9_slot = var_qxf1_dn9;
        *var_qxf1_rdb0_slot = var_qxf1_rdb0;
        *var_qxf1_rdb1_slot = var_qxf1_rdb1;
        *var_qxf1_rdb2_slot = var_qxf1_rdb2;
        *var_qxf1_rdb3_slot = var_qxf1_rdb3;
        *var_qxf1_rdb4_slot = var_qxf1_rdb4;
        *var_qxf1_rdb5_slot = var_qxf1_rdb5;
        *var_qxf1_rdb6_slot = var_qxf1_rdb6;
        *var_qxf1_rdb7_slot = var_qxf1_rdb7;
        *var_qxf1_rdn0_slot = var_qxf1_rdn0;
        *var_qxf1_rdn1_slot = var_qxf1_rdn1;
        *var_qxf1_rdn2_slot = var_qxf1_rdn2;
        *var_qxf1_rdn3_slot = var_qxf1_rdn3;
        *var_qxf1_rdn4_slot = var_qxf1_rdn4;
        *var_qxf1_rdn5_slot = var_qxf1_rdn5;
        *var_qxf1_rdn6_slot = var_qxf1_rdn6;
        *var_qxf1_rdn7_slot = var_qxf1_rdn7;
        *var_qxf1_rdn8_slot = var_qxf1_rdn8;
        *var_qxf1_rdn9_slot = var_qxf1_rdn9;
        *var_qxf1_rv_slot = var_qxf1_rv;
    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
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
        var_bf_t: f64,
        var_bf_t_dn3: f64,
        var_bf_t_dn4: f64,
        var_bf_t_dn5: f64,
        var_guard13: f64,
        var_guard20: f64,
        var_guard21: f64,
        var_guard22: f64,
        var_guard23: f64,
        var_guard24: f64,
        var_guard25: f64,
        var_ibc: f64,
        var_ibc_dn3: f64,
        var_ibc_dn4: f64,
        var_ibc_dn5: f64,
        var_ibc_dn6: f64,
        var_ibe: f64,
        var_ibe_dn3: f64,
        var_ibe_dn4: f64,
        var_ibe_dn5: f64,
        var_ibe_dn6: f64,
        var_ifwd: f64,
        var_ifwd_dn3: f64,
        var_ifwd_dn4: f64,
        var_ifwd_dn5: f64,
        var_ifwd_dn6: f64,
        var_rb: f64,
        var_rb_dn1: f64,
        var_rb_dn3: f64,
        var_rb_dn5: f64,
        var_rb_dn8: f64,
        var_rc: f64,
        var_rc_dn3: f64,
        var_re: f64,
        var_re_dn2: f64,
        var_re_dn3: f64,
        var_re_dn6: f64,
        var_tff: f64,
        var_tff_dn1: f64,
        var_tff_dn2: f64,
        var_ttype: f64,
        var_weff: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq2_e98: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (nv9 - 0.0));
        let eq2_e99: f64 = (p.p83 * eq2_e98);
        let eq2_value: f64 = eq2_e99;
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * (eq2_value),
            9,
            multiplicity * ((p.p83 * ddt_scale)),
        );
        let (eq3_e108, eq3_e108_d_n1, eq3_e108_d_n2, eq3_e108_d_n3, eq3_e108_d_n4, eq3_e108_d_n5, eq3_e108_d_n6,) = {
    if (var_guard13 != 0.0) {
        let eq3_e103: f64 = (var_ifwd / var_bf_t);
        let __rspice_inv_cse_0: f64 = 1.0 / (var_bf_t * var_bf_t);
        let eq3_e103_d_n3: f64 = (((var_ifwd_dn3 * var_bf_t) - (var_ifwd * var_bf_t_dn3)) * __rspice_inv_cse_0);
        let eq3_e103_d_n4: f64 = (((var_ifwd_dn4 * var_bf_t) - (var_ifwd * var_bf_t_dn4)) * __rspice_inv_cse_0);
        let eq3_e103_d_n5: f64 = (((var_ifwd_dn5 * var_bf_t) - (var_ifwd * var_bf_t_dn5)) * __rspice_inv_cse_0);
        let eq3_e103_d_n6: f64 = (var_ifwd_dn6 / var_bf_t);
        let eq3_e104: f64 = (-eq3_e103);
        let eq3_e106: f64 = (eq3_e104 * var_tff);
        let eq3_e106_d_n1: f64 = (eq3_e104 * var_tff_dn1);
        let eq3_e106_d_n2: f64 = (eq3_e104 * var_tff_dn2);
        let eq3_e106_d_n3: f64 = ((-eq3_e103_d_n3) * var_tff);
        let eq3_e106_d_n4: f64 = ((-eq3_e103_d_n4) * var_tff);
        let eq3_e106_d_n5: f64 = ((-eq3_e103_d_n5) * var_tff);
        let eq3_e106_d_n6: f64 = ((-eq3_e103_d_n6) * var_tff);
        (eq3_e106, eq3_e106_d_n1, eq3_e106_d_n2, eq3_e106_d_n3, eq3_e106_d_n4, eq3_e106_d_n5, eq3_e106_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e108;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            None,
            multiplicity * (eq3_value),
            [1, 2, 3, 4, 5, 6],
            [multiplicity * (eq3_e108_d_n1), multiplicity * (eq3_e108_d_n2), multiplicity * (eq3_e108_d_n3), multiplicity * (eq3_e108_d_n4), multiplicity * (eq3_e108_d_n5), multiplicity * (eq3_e108_d_n6)],
            [],
            [],
            1.0,
        );
        let (eq5_e121, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n8,) = {
    if (var_guard13 != 0.0) {
        let eq5_e118: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (nv8 - 0.0));
        let eq5_e119: f64 = (var_tff * eq5_e118);
        let eq5_e119_d_n1: f64 = (var_tff_dn1 * eq5_e118);
        let eq5_e119_d_n2: f64 = (var_tff_dn2 * eq5_e118);
        (eq5_e119, eq5_e119_d_n1, eq5_e119_d_n2, (var_tff * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e121;
        stamper.stamp_current_node3_local(
            Some(8),
            None,
            multiplicity * (eq5_value),
            1,
            multiplicity * (eq5_e121_d_n1),
            2,
            multiplicity * (eq5_e121_d_n2),
            8,
            multiplicity * (eq5_e121_d_n8),
        );
        let (eq7_e141, eq7_e141_d_n0, eq7_e141_d_n1, eq7_e141_d_n2, eq7_e141_d_n3, eq7_e141_d_n4, eq7_e141_d_n5, eq7_e141_d_n6,) = {
    if (var_guard20 != 0.0) {
        let eq7_e129: f64 = (-1.0);
        let eq7_e132: f64 = (var_ibe * (nv1 - nv2));
        let eq7_e132_d_n3: f64 = (var_ibe_dn3 * (nv1 - nv2));
        let eq7_e132_d_n4: f64 = (var_ibe_dn4 * (nv1 - nv2));
        let eq7_e132_d_n5: f64 = (var_ibe_dn5 * (nv1 - nv2));
        let eq7_e132_d_n6: f64 = (var_ibe_dn6 * (nv1 - nv2));
        let eq7_e133: f64 = (eq7_e132).abs();
        let eq7_e133_d_n1: f64 = if eq7_e132 >= 0.0 { var_ibe } else { (-var_ibe) };
        let eq7_e133_d_n2: f64 = if eq7_e132 >= 0.0 { (-var_ibe) } else { (-(-var_ibe)) };
        let eq7_e133_d_n3: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n3 } else { (-eq7_e132_d_n3) };
        let eq7_e133_d_n4: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n4 } else { (-eq7_e132_d_n4) };
        let eq7_e133_d_n5: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n5 } else { (-eq7_e132_d_n5) };
        let eq7_e133_d_n6: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n6 } else { (-eq7_e132_d_n6) };
        let eq7_e134: f64 = (eq7_e129 * eq7_e133);
        let eq7_e134_d_n1: f64 = (eq7_e129 * eq7_e133_d_n1);
        let eq7_e134_d_n2: f64 = (eq7_e129 * eq7_e133_d_n2);
        let eq7_e134_d_n3: f64 = (eq7_e129 * eq7_e133_d_n3);
        let eq7_e134_d_n4: f64 = (eq7_e129 * eq7_e133_d_n4);
        let eq7_e134_d_n5: f64 = (eq7_e129 * eq7_e133_d_n5);
        let eq7_e134_d_n6: f64 = (eq7_e129 * eq7_e133_d_n6);
        let eq7_e137: f64 = (var_ibc * (nv1 - nv0));
        let eq7_e137_d_n3: f64 = (var_ibc_dn3 * (nv1 - nv0));
        let eq7_e137_d_n4: f64 = (var_ibc_dn4 * (nv1 - nv0));
        let eq7_e137_d_n5: f64 = (var_ibc_dn5 * (nv1 - nv0));
        let eq7_e137_d_n6: f64 = (var_ibc_dn6 * (nv1 - nv0));
        let eq7_e138: f64 = (eq7_e137).abs();
        let eq7_e138_d_n0: f64 = if eq7_e137 >= 0.0 { (-var_ibc) } else { (-(-var_ibc)) };
        let eq7_e138_d_n1: f64 = if eq7_e137 >= 0.0 { var_ibc } else { (-var_ibc) };
        let eq7_e138_d_n3: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n3 } else { (-eq7_e137_d_n3) };
        let eq7_e138_d_n4: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n4 } else { (-eq7_e137_d_n4) };
        let eq7_e138_d_n5: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n5 } else { (-eq7_e137_d_n5) };
        let eq7_e138_d_n6: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n6 } else { (-eq7_e137_d_n6) };
        let eq7_e139: f64 = (eq7_e134 - eq7_e138);
        let eq7_e139_d_n1: f64 = (eq7_e134_d_n1 - eq7_e138_d_n1);
        let eq7_e139_d_n3: f64 = (eq7_e134_d_n3 - eq7_e138_d_n3);
        let eq7_e139_d_n4: f64 = (eq7_e134_d_n4 - eq7_e138_d_n4);
        let eq7_e139_d_n5: f64 = (eq7_e134_d_n5 - eq7_e138_d_n5);
        let eq7_e139_d_n6: f64 = (eq7_e134_d_n6 - eq7_e138_d_n6);
        (eq7_e139, (-eq7_e138_d_n0), eq7_e139_d_n1, eq7_e134_d_n2, eq7_e139_d_n3, eq7_e139_d_n4, eq7_e139_d_n5, eq7_e139_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e141;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (eq7_value),
            [0, 1, 2, 3, 4, 5, 6],
            [multiplicity * (eq7_e141_d_n0), multiplicity * (eq7_e141_d_n1), multiplicity * (eq7_e141_d_n2), multiplicity * (eq7_e141_d_n3), multiplicity * (eq7_e141_d_n4), multiplicity * (eq7_e141_d_n5), multiplicity * (eq7_e141_d_n6)],
            [],
            [],
            1.0,
        );
        let (eq8_e147, eq8_e147_d_n3,) = {
    if (var_guard20 != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / p.p33;
        let eq8_e145: f64 = ((nv3 - 0.0) * __rspice_inv_cse_1);
        let eq8_e145_d_n3: f64 = (1.0 * __rspice_inv_cse_1);
        (eq8_e145, eq8_e145_d_n3,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e147;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq8_value),
            3,
            multiplicity * (eq8_e147_d_n3),
        );
        let (eq9_e154, eq9_e154_d_n3,) = {
    if (var_guard20 != 0.0) {
        let eq9_e151: f64 = ((nv3 - 0.0) * p.p34);
        let eq9_e152: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq9_e151);
        (eq9_e152, (p.p34 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e154;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq9_value),
            3,
            multiplicity * (eq9_e154_d_n3),
        );
        let (eq11_e176, eq11_e176_d_n0, eq11_e176_d_n1, eq11_e176_d_n2, eq11_e176_d_n3, eq11_e176_d_n4, eq11_e176_d_n5, eq11_e176_d_n6,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq11_e164: f64 = (-1.0);
        let eq11_e167: f64 = (var_ibe * (nv1 - nv2));
        let eq11_e167_d_n3: f64 = (var_ibe_dn3 * (nv1 - nv2));
        let eq11_e167_d_n4: f64 = (var_ibe_dn4 * (nv1 - nv2));
        let eq11_e167_d_n5: f64 = (var_ibe_dn5 * (nv1 - nv2));
        let eq11_e167_d_n6: f64 = (var_ibe_dn6 * (nv1 - nv2));
        let eq11_e168: f64 = (eq11_e167).abs();
        let eq11_e168_d_n1: f64 = if eq11_e167 >= 0.0 { var_ibe } else { (-var_ibe) };
        let eq11_e168_d_n2: f64 = if eq11_e167 >= 0.0 { (-var_ibe) } else { (-(-var_ibe)) };
        let eq11_e168_d_n3: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n3 } else { (-eq11_e167_d_n3) };
        let eq11_e168_d_n4: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n4 } else { (-eq11_e167_d_n4) };
        let eq11_e168_d_n5: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n5 } else { (-eq11_e167_d_n5) };
        let eq11_e168_d_n6: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n6 } else { (-eq11_e167_d_n6) };
        let eq11_e169: f64 = (eq11_e164 * eq11_e168);
        let eq11_e169_d_n1: f64 = (eq11_e164 * eq11_e168_d_n1);
        let eq11_e169_d_n2: f64 = (eq11_e164 * eq11_e168_d_n2);
        let eq11_e169_d_n3: f64 = (eq11_e164 * eq11_e168_d_n3);
        let eq11_e169_d_n4: f64 = (eq11_e164 * eq11_e168_d_n4);
        let eq11_e169_d_n5: f64 = (eq11_e164 * eq11_e168_d_n5);
        let eq11_e169_d_n6: f64 = (eq11_e164 * eq11_e168_d_n6);
        let eq11_e172: f64 = (var_ibc * (nv1 - nv0));
        let eq11_e172_d_n3: f64 = (var_ibc_dn3 * (nv1 - nv0));
        let eq11_e172_d_n4: f64 = (var_ibc_dn4 * (nv1 - nv0));
        let eq11_e172_d_n5: f64 = (var_ibc_dn5 * (nv1 - nv0));
        let eq11_e172_d_n6: f64 = (var_ibc_dn6 * (nv1 - nv0));
        let eq11_e173: f64 = (eq11_e172).abs();
        let eq11_e173_d_n0: f64 = if eq11_e172 >= 0.0 { (-var_ibc) } else { (-(-var_ibc)) };
        let eq11_e173_d_n1: f64 = if eq11_e172 >= 0.0 { var_ibc } else { (-var_ibc) };
        let eq11_e173_d_n3: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n3 } else { (-eq11_e172_d_n3) };
        let eq11_e173_d_n4: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n4 } else { (-eq11_e172_d_n4) };
        let eq11_e173_d_n5: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n5 } else { (-eq11_e172_d_n5) };
        let eq11_e173_d_n6: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n6 } else { (-eq11_e172_d_n6) };
        let eq11_e174: f64 = (eq11_e169 - eq11_e173);
        let eq11_e174_d_n1: f64 = (eq11_e169_d_n1 - eq11_e173_d_n1);
        let eq11_e174_d_n3: f64 = (eq11_e169_d_n3 - eq11_e173_d_n3);
        let eq11_e174_d_n4: f64 = (eq11_e169_d_n4 - eq11_e173_d_n4);
        let eq11_e174_d_n5: f64 = (eq11_e169_d_n5 - eq11_e173_d_n5);
        let eq11_e174_d_n6: f64 = (eq11_e169_d_n6 - eq11_e173_d_n6);
        (eq11_e174, (-eq11_e173_d_n0), eq11_e174_d_n1, eq11_e169_d_n2, eq11_e174_d_n3, eq11_e174_d_n4, eq11_e174_d_n5, eq11_e174_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e176;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (eq11_value),
            [0, 1, 2, 3, 4, 5, 6],
            [multiplicity * (eq11_e176_d_n0), multiplicity * (eq11_e176_d_n1), multiplicity * (eq11_e176_d_n2), multiplicity * (eq11_e176_d_n3), multiplicity * (eq11_e176_d_n4), multiplicity * (eq11_e176_d_n5), multiplicity * (eq11_e176_d_n6)],
            [],
            [],
            1.0,
        );
        let (eq12_e185, eq12_e185_d_n3, eq12_e185_d_n7,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let __rspice_inv_cse_2: f64 = 1.0 / p.p33;
        let eq12_e183: f64 = ((nv3 - nv7) * __rspice_inv_cse_2);
        let eq12_e183_d_n3: f64 = (1.0 * __rspice_inv_cse_2);
        let eq12_e183_d_n7: f64 = ((-1.0) * __rspice_inv_cse_2);
        (eq12_e183, eq12_e183_d_n3, eq12_e183_d_n7,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e185;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * (eq12_value),
            3,
            multiplicity * (eq12_e185_d_n3),
            7,
            multiplicity * (eq12_e185_d_n7),
        );
        let (eq13_e195, eq13_e195_d_n3,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq13_e192: f64 = (p.p34 * (nv3 - 0.0));
        let eq13_e193: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq13_e192);
        (eq13_e193, (p.p34 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e195;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq13_value),
            3,
            multiplicity * (eq13_e195_d_n3),
        );
        let (eq14_e204, eq14_e204_d_n7,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let __rspice_inv_cse_3: f64 = 1.0 / p.p35;
        let eq14_e202: f64 = ((nv7 - 0.0) * __rspice_inv_cse_3);
        let eq14_e202_d_n7: f64 = (1.0 * __rspice_inv_cse_3);
        (eq14_e202, eq14_e202_d_n7,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e204;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq14_value),
            7,
            multiplicity * (eq14_e204_d_n7),
        );
        let (eq15_e214, eq15_e214_d_n7,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq15_e211: f64 = (p.p36 * (nv7 - 0.0));
        let eq15_e212: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq15_e211);
        (eq15_e212, (p.p36 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e214;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq15_value),
            7,
            multiplicity * (eq15_e214_d_n7),
        );
        let (eq16_e235, eq16_e235_d_n0, eq16_e235_d_n1, eq16_e235_d_n2, eq16_e235_d_n3, eq16_e235_d_n4, eq16_e235_d_n5, eq16_e235_d_n6,) = {
    if (((var_guard20 == 0.0) && (var_guard21 == 0.0)) && (var_guard22 != 0.0)) {
        let eq16_e223: f64 = (-1.0);
        let eq16_e226: f64 = (var_ibe * (nv1 - nv2));
        let eq16_e226_d_n3: f64 = (var_ibe_dn3 * (nv1 - nv2));
        let eq16_e226_d_n4: f64 = (var_ibe_dn4 * (nv1 - nv2));
        let eq16_e226_d_n5: f64 = (var_ibe_dn5 * (nv1 - nv2));
        let eq16_e226_d_n6: f64 = (var_ibe_dn6 * (nv1 - nv2));
        let eq16_e227: f64 = (eq16_e226).abs();
        let eq16_e227_d_n1: f64 = if eq16_e226 >= 0.0 { var_ibe } else { (-var_ibe) };
        let eq16_e227_d_n2: f64 = if eq16_e226 >= 0.0 { (-var_ibe) } else { (-(-var_ibe)) };
        let eq16_e227_d_n3: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n3 } else { (-eq16_e226_d_n3) };
        let eq16_e227_d_n4: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n4 } else { (-eq16_e226_d_n4) };
        let eq16_e227_d_n5: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n5 } else { (-eq16_e226_d_n5) };
        let eq16_e227_d_n6: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n6 } else { (-eq16_e226_d_n6) };
        let eq16_e228: f64 = (eq16_e223 * eq16_e227);
        let eq16_e228_d_n1: f64 = (eq16_e223 * eq16_e227_d_n1);
        let eq16_e228_d_n2: f64 = (eq16_e223 * eq16_e227_d_n2);
        let eq16_e228_d_n3: f64 = (eq16_e223 * eq16_e227_d_n3);
        let eq16_e228_d_n4: f64 = (eq16_e223 * eq16_e227_d_n4);
        let eq16_e228_d_n5: f64 = (eq16_e223 * eq16_e227_d_n5);
        let eq16_e228_d_n6: f64 = (eq16_e223 * eq16_e227_d_n6);
        let eq16_e231: f64 = (var_ibc * (nv1 - nv0));
        let eq16_e231_d_n3: f64 = (var_ibc_dn3 * (nv1 - nv0));
        let eq16_e231_d_n4: f64 = (var_ibc_dn4 * (nv1 - nv0));
        let eq16_e231_d_n5: f64 = (var_ibc_dn5 * (nv1 - nv0));
        let eq16_e231_d_n6: f64 = (var_ibc_dn6 * (nv1 - nv0));
        let eq16_e232: f64 = (eq16_e231).abs();
        let eq16_e232_d_n0: f64 = if eq16_e231 >= 0.0 { (-var_ibc) } else { (-(-var_ibc)) };
        let eq16_e232_d_n1: f64 = if eq16_e231 >= 0.0 { var_ibc } else { (-var_ibc) };
        let eq16_e232_d_n3: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n3 } else { (-eq16_e231_d_n3) };
        let eq16_e232_d_n4: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n4 } else { (-eq16_e231_d_n4) };
        let eq16_e232_d_n5: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n5 } else { (-eq16_e231_d_n5) };
        let eq16_e232_d_n6: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n6 } else { (-eq16_e231_d_n6) };
        let eq16_e233: f64 = (eq16_e228 - eq16_e232);
        let eq16_e233_d_n1: f64 = (eq16_e228_d_n1 - eq16_e232_d_n1);
        let eq16_e233_d_n3: f64 = (eq16_e228_d_n3 - eq16_e232_d_n3);
        let eq16_e233_d_n4: f64 = (eq16_e228_d_n4 - eq16_e232_d_n4);
        let eq16_e233_d_n5: f64 = (eq16_e228_d_n5 - eq16_e232_d_n5);
        let eq16_e233_d_n6: f64 = (eq16_e228_d_n6 - eq16_e232_d_n6);
        (eq16_e233, (-eq16_e232_d_n0), eq16_e233_d_n1, eq16_e228_d_n2, eq16_e233_d_n3, eq16_e233_d_n4, eq16_e233_d_n5, eq16_e233_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e235;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (eq16_value),
            [0, 1, 2, 3, 4, 5, 6],
            [multiplicity * (eq16_e235_d_n0), multiplicity * (eq16_e235_d_n1), multiplicity * (eq16_e235_d_n2), multiplicity * (eq16_e235_d_n3), multiplicity * (eq16_e235_d_n4), multiplicity * (eq16_e235_d_n5), multiplicity * (eq16_e235_d_n6)],
            [],
            [],
            1.0,
        );
        let (eq23_e297, eq23_e297_d_n1, eq23_e297_d_n3, eq23_e297_d_n5, eq23_e297_d_n8,) = {
    if (var_guard23 != 0.0) {
        let __rspice_inv_cse_4: f64 = 1.0 / var_weff;
        let eq23_e287: f64 = (var_rb * __rspice_inv_cse_4);
        let eq23_e287_d_n1: f64 = (var_rb_dn1 * __rspice_inv_cse_4);
        let eq23_e287_d_n3: f64 = (var_rb_dn3 * __rspice_inv_cse_4);
        let eq23_e287_d_n5: f64 = (var_rb_dn5 * __rspice_inv_cse_4);
        let eq23_e287_d_n8: f64 = (var_rb_dn8 * __rspice_inv_cse_4);
        let (eq23_e294, eq23_e294_d_n1, eq23_e294_d_n3, eq23_e294_d_n5, eq23_e294_d_n8,) = {
            if (eq23_e287 > p.p46) {
                let __rspice_inv_cse_5: f64 = 1.0 / var_weff;
                let eq23_e292: f64 = (var_rb * __rspice_inv_cse_5);
                let eq23_e292_d_n1: f64 = (var_rb_dn1 * __rspice_inv_cse_5);
                let eq23_e292_d_n3: f64 = (var_rb_dn3 * __rspice_inv_cse_5);
                let eq23_e292_d_n5: f64 = (var_rb_dn5 * __rspice_inv_cse_5);
                let eq23_e292_d_n8: f64 = (var_rb_dn8 * __rspice_inv_cse_5);
                (eq23_e292, eq23_e292_d_n1, eq23_e292_d_n3, eq23_e292_d_n5, eq23_e292_d_n8,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq23_e295: f64 = ((nv1 - nv5) / eq23_e294);
        let eq23_e295_d_n1: f64 = ((eq23_e294 - ((nv1 - nv5) * eq23_e294_d_n1)) / (eq23_e294 * eq23_e294));
        let eq23_e295_d_n3: f64 = (-(((nv1 - nv5) * eq23_e294_d_n3) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n5: f64 = (((-eq23_e294) - ((nv1 - nv5) * eq23_e294_d_n5)) / (eq23_e294 * eq23_e294));
        let eq23_e295_d_n8: f64 = (-(((nv1 - nv5) * eq23_e294_d_n8) / (eq23_e294 * eq23_e294)));
        (eq23_e295, eq23_e295_d_n1, eq23_e295_d_n3, eq23_e295_d_n5, eq23_e295_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e297;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (eq23_value),
            [1, 3, 5, 8],
            [multiplicity * (eq23_e297_d_n1), multiplicity * (eq23_e297_d_n3), multiplicity * (eq23_e297_d_n5), multiplicity * (eq23_e297_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq26_e323, eq26_e323_d_n2, eq26_e323_d_n3, eq26_e323_d_n6,) = {
    if (var_guard24 != 0.0) {
        let __rspice_inv_cse_6: f64 = 1.0 / var_weff;
        let eq26_e313: f64 = (var_re * __rspice_inv_cse_6);
        let eq26_e313_d_n2: f64 = (var_re_dn2 * __rspice_inv_cse_6);
        let eq26_e313_d_n3: f64 = (var_re_dn3 * __rspice_inv_cse_6);
        let eq26_e313_d_n6: f64 = (var_re_dn6 * __rspice_inv_cse_6);
        let (eq26_e320, eq26_e320_d_n2, eq26_e320_d_n3, eq26_e320_d_n6,) = {
            if (eq26_e313 > p.p46) {
                let __rspice_inv_cse_7: f64 = 1.0 / var_weff;
                let eq26_e318: f64 = (var_re * __rspice_inv_cse_7);
                let eq26_e318_d_n2: f64 = (var_re_dn2 * __rspice_inv_cse_7);
                let eq26_e318_d_n3: f64 = (var_re_dn3 * __rspice_inv_cse_7);
                let eq26_e318_d_n6: f64 = (var_re_dn6 * __rspice_inv_cse_7);
                (eq26_e318, eq26_e318_d_n2, eq26_e318_d_n3, eq26_e318_d_n6,)
            } else {
                (p.p46, 0.0, 0.0, 0.0,)
            }
        };
        let eq26_e321: f64 = ((nv2 - nv6) / eq26_e320);
        let eq26_e321_d_n2: f64 = ((eq26_e320 - ((nv2 - nv6) * eq26_e320_d_n2)) / (eq26_e320 * eq26_e320));
        let eq26_e321_d_n3: f64 = (-(((nv2 - nv6) * eq26_e320_d_n3) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n6: f64 = (((-eq26_e320) - ((nv2 - nv6) * eq26_e320_d_n6)) / (eq26_e320 * eq26_e320));
        (eq26_e321, eq26_e321_d_n2, eq26_e321_d_n3, eq26_e321_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e323;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(6),
            multiplicity * (eq26_value),
            2,
            multiplicity * (eq26_e323_d_n2),
            3,
            multiplicity * (eq26_e323_d_n3),
            6,
            multiplicity * (eq26_e323_d_n6),
        );
        let (eq29_e349, eq29_e349_d_n0, eq29_e349_d_n3, eq29_e349_d_n4,) = {
    if (var_guard25 != 0.0) {
        let __rspice_inv_cse_8: f64 = 1.0 / var_weff;
        let eq29_e339: f64 = (var_rc * __rspice_inv_cse_8);
        let eq29_e339_d_n3: f64 = (var_rc_dn3 * __rspice_inv_cse_8);
        let (eq29_e346, eq29_e346_d_n3,) = {
            if (eq29_e339 > p.p46) {
                let __rspice_inv_cse_9: f64 = 1.0 / var_weff;
                let eq29_e344: f64 = (var_rc * __rspice_inv_cse_9);
                let eq29_e344_d_n3: f64 = (var_rc_dn3 * __rspice_inv_cse_9);
                (eq29_e344, eq29_e344_d_n3,)
            } else {
                (p.p46, 0.0,)
            }
        };
        let __rspice_inv_cse_10: f64 = 1.0 / eq29_e346;
        let eq29_e347: f64 = ((nv0 - nv4) * __rspice_inv_cse_10);
        let eq29_e347_d_n0: f64 = (1.0 * __rspice_inv_cse_10);
        let eq29_e347_d_n3: f64 = (-(((nv0 - nv4) * eq29_e346_d_n3) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n4: f64 = (-1.0 / eq29_e346);
        (eq29_e347, eq29_e347_d_n0, eq29_e347_d_n3, eq29_e347_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e349;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * (eq29_value),
            0,
            multiplicity * (eq29_e349_d_n0),
            3,
            multiplicity * (eq29_e349_d_n3),
            4,
            multiplicity * (eq29_e349_d_n4),
        );
        let eq32_e363: f64 = (var_ttype * var_ibe);
        let eq32_e363_d_n3: f64 = (var_ttype * var_ibe_dn3);
        let eq32_e363_d_n4: f64 = (var_ttype * var_ibe_dn4);
        let eq32_e363_d_n5: f64 = (var_ttype * var_ibe_dn5);
        let eq32_e363_d_n6: f64 = (var_ttype * var_ibe_dn6);
        let eq32_e365: f64 = (eq32_e363 * var_weff);
        let eq32_e365_d_n3: f64 = (eq32_e363_d_n3 * var_weff);
        let eq32_e365_d_n4: f64 = (eq32_e363_d_n4 * var_weff);
        let eq32_e365_d_n5: f64 = (eq32_e363_d_n5 * var_weff);
        let eq32_e365_d_n6: f64 = (eq32_e363_d_n6 * var_weff);
        let eq32_value: f64 = eq32_e365;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq32_value),
            [3, 4, 5, 6],
            [multiplicity * (eq32_e365_d_n3), multiplicity * (eq32_e365_d_n4), multiplicity * (eq32_e365_d_n5), multiplicity * (eq32_e365_d_n6)],
            [],
            [],
            1.0,
        );
        let eq33_e368: f64 = (var_ttype * var_ibc);
        let eq33_e368_d_n3: f64 = (var_ttype * var_ibc_dn3);
        let eq33_e368_d_n4: f64 = (var_ttype * var_ibc_dn4);
        let eq33_e368_d_n5: f64 = (var_ttype * var_ibc_dn5);
        let eq33_e368_d_n6: f64 = (var_ttype * var_ibc_dn6);
        let eq33_e370: f64 = (eq33_e368 * var_weff);
        let eq33_e370_d_n3: f64 = (eq33_e368_d_n3 * var_weff);
        let eq33_e370_d_n4: f64 = (eq33_e368_d_n4 * var_weff);
        let eq33_e370_d_n5: f64 = (eq33_e368_d_n5 * var_weff);
        let eq33_e370_d_n6: f64 = (eq33_e368_d_n6 * var_weff);
        let eq33_value: f64 = eq33_e370;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq33_value),
            [3, 4, 5, 6],
            [multiplicity * (eq33_e370_d_n3), multiplicity * (eq33_e370_d_n4), multiplicity * (eq33_e370_d_n5), multiplicity * (eq33_e370_d_n6)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
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
        var_itr: f64,
        var_itr_dn3: f64,
        var_itr_dn4: f64,
        var_itr_dn5: f64,
        var_itr_dn6: f64,
        var_itzf_f: f64,
        var_itzf_f_dn3: f64,
        var_itzf_f_dn4: f64,
        var_itzf_f_dn5: f64,
        var_itzf_f_dn6: f64,
        var_itzf_f_dn9: f64,
        var_qdc: f64,
        var_qdc_dn3: f64,
        var_qdc_dn4: f64,
        var_qdc_dn5: f64,
        var_qdc_dn6: f64,
        var_qde: f64,
        var_qde_dn1: f64,
        var_qde_dn2: f64,
        var_qde_dn3: f64,
        var_qde_dn4: f64,
        var_qde_dn5: f64,
        var_qde_dn6: f64,
        var_qjci_1: f64,
        var_qjci_1_dn1: f64,
        var_qjci_1_dn3: f64,
        var_qjci_1_dn4: f64,
        var_qjci_1_dn5: f64,
        var_qjci_1_dn6: f64,
        var_qjcx_1: f64,
        var_qjcx_1_dn1: f64,
        var_qjcx_1_dn3: f64,
        var_qjcx_1_dn4: f64,
        var_qjcx_1_dn5: f64,
        var_qjcx_1_dn6: f64,
        var_qje: f64,
        var_qje_dn1: f64,
        var_qje_dn3: f64,
        var_qje_dn4: f64,
        var_qje_dn5: f64,
        var_qje_dn6: f64,
        var_qjs: f64,
        var_qjs_dn2: f64,
        var_qjs_dn3: f64,
        var_qjs_dn4: f64,
        var_qxf1: f64,
        var_qxf1_dn3: f64,
        var_qxf1_dn4: f64,
        var_qxf1_dn5: f64,
        var_qxf1_dn6: f64,
        var_ttype: f64,
        var_weff: f64,
    ) {
        let eq34_e373: f64 = (-var_itr);
        let eq34_e375: f64 = (eq34_e373 * var_weff);
        let eq34_e375_d_n3: f64 = ((-var_itr_dn3) * var_weff);
        let eq34_e375_d_n4: f64 = ((-var_itr_dn4) * var_weff);
        let eq34_e375_d_n5: f64 = ((-var_itr_dn5) * var_weff);
        let eq34_e375_d_n6: f64 = ((-var_itr_dn6) * var_weff);
        let eq34_e376: f64 = (var_ttype * eq34_e375);
        let eq34_e376_d_n3: f64 = (var_ttype * eq34_e375_d_n3);
        let eq34_e376_d_n4: f64 = (var_ttype * eq34_e375_d_n4);
        let eq34_e376_d_n5: f64 = (var_ttype * eq34_e375_d_n5);
        let eq34_e376_d_n6: f64 = (var_ttype * eq34_e375_d_n6);
        let eq34_value: f64 = eq34_e376;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * (eq34_value),
            [3, 4, 5, 6],
            [multiplicity * (eq34_e376_d_n3), multiplicity * (eq34_e376_d_n4), multiplicity * (eq34_e376_d_n5), multiplicity * (eq34_e376_d_n6)],
            [],
            [],
            1.0,
        );
        let eq35_e379: f64 = (var_ttype * var_itzf_f);
        let eq35_e379_d_n3: f64 = (var_ttype * var_itzf_f_dn3);
        let eq35_e379_d_n4: f64 = (var_ttype * var_itzf_f_dn4);
        let eq35_e379_d_n5: f64 = (var_ttype * var_itzf_f_dn5);
        let eq35_e379_d_n6: f64 = (var_ttype * var_itzf_f_dn6);
        let eq35_e379_d_n9: f64 = (var_ttype * var_itzf_f_dn9);
        let eq35_e381: f64 = (eq35_e379 * var_weff);
        let eq35_e381_d_n3: f64 = (eq35_e379_d_n3 * var_weff);
        let eq35_e381_d_n4: f64 = (eq35_e379_d_n4 * var_weff);
        let eq35_e381_d_n5: f64 = (eq35_e379_d_n5 * var_weff);
        let eq35_e381_d_n6: f64 = (eq35_e379_d_n6 * var_weff);
        let eq35_e381_d_n9: f64 = (eq35_e379_d_n9 * var_weff);
        let eq35_value: f64 = eq35_e381;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(6),
            multiplicity * (eq35_value),
            [3, 4, 5, 6, 9],
            [multiplicity * (eq35_e381_d_n3), multiplicity * (eq35_e381_d_n4), multiplicity * (eq35_e381_d_n5), multiplicity * (eq35_e381_d_n6), multiplicity * (eq35_e381_d_n9)],
            [],
            [],
            1.0,
        );
        let eq36_e384: f64 = (var_ttype * var_qje);
        let eq36_e384_d_n1: f64 = (var_ttype * var_qje_dn1);
        let eq36_e384_d_n3: f64 = (var_ttype * var_qje_dn3);
        let eq36_e384_d_n4: f64 = (var_ttype * var_qje_dn4);
        let eq36_e384_d_n5: f64 = (var_ttype * var_qje_dn5);
        let eq36_e384_d_n6: f64 = (var_ttype * var_qje_dn6);
        let eq36_e386: f64 = (eq36_e384 * var_weff);
        let eq36_e386_d_n1: f64 = (eq36_e384_d_n1 * var_weff);
        let eq36_e386_d_n3: f64 = (eq36_e384_d_n3 * var_weff);
        let eq36_e386_d_n4: f64 = (eq36_e384_d_n4 * var_weff);
        let eq36_e386_d_n5: f64 = (eq36_e384_d_n5 * var_weff);
        let eq36_e386_d_n6: f64 = (eq36_e384_d_n6 * var_weff);
        let eq36_e387: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq36_e386);
        let eq36_value: f64 = eq36_e387;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq36_value),
            [1, 3, 4, 5, 6],
            [multiplicity * ((eq36_e386_d_n1 * ddt_scale)), multiplicity * ((eq36_e386_d_n3 * ddt_scale)), multiplicity * ((eq36_e386_d_n4 * ddt_scale)), multiplicity * ((eq36_e386_d_n5 * ddt_scale)), multiplicity * ((eq36_e386_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq37_e390: f64 = (var_ttype * var_qde);
        let eq37_e390_d_n1: f64 = (var_ttype * var_qde_dn1);
        let eq37_e390_d_n2: f64 = (var_ttype * var_qde_dn2);
        let eq37_e390_d_n3: f64 = (var_ttype * var_qde_dn3);
        let eq37_e390_d_n4: f64 = (var_ttype * var_qde_dn4);
        let eq37_e390_d_n5: f64 = (var_ttype * var_qde_dn5);
        let eq37_e390_d_n6: f64 = (var_ttype * var_qde_dn6);
        let eq37_e392: f64 = (eq37_e390 * var_weff);
        let eq37_e392_d_n1: f64 = (eq37_e390_d_n1 * var_weff);
        let eq37_e392_d_n2: f64 = (eq37_e390_d_n2 * var_weff);
        let eq37_e392_d_n3: f64 = (eq37_e390_d_n3 * var_weff);
        let eq37_e392_d_n4: f64 = (eq37_e390_d_n4 * var_weff);
        let eq37_e392_d_n5: f64 = (eq37_e390_d_n5 * var_weff);
        let eq37_e392_d_n6: f64 = (eq37_e390_d_n6 * var_weff);
        let eq37_e393: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq37_e392);
        let eq37_value: f64 = eq37_e393;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq37_value),
            [1, 2, 3, 4, 5, 6],
            [multiplicity * ((eq37_e392_d_n1 * ddt_scale)), multiplicity * ((eq37_e392_d_n2 * ddt_scale)), multiplicity * ((eq37_e392_d_n3 * ddt_scale)), multiplicity * ((eq37_e392_d_n4 * ddt_scale)), multiplicity * ((eq37_e392_d_n5 * ddt_scale)), multiplicity * ((eq37_e392_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq38_e396: f64 = (var_ttype * var_qjcx_1);
        let eq38_e396_d_n1: f64 = (var_ttype * var_qjcx_1_dn1);
        let eq38_e396_d_n3: f64 = (var_ttype * var_qjcx_1_dn3);
        let eq38_e396_d_n4: f64 = (var_ttype * var_qjcx_1_dn4);
        let eq38_e396_d_n5: f64 = (var_ttype * var_qjcx_1_dn5);
        let eq38_e396_d_n6: f64 = (var_ttype * var_qjcx_1_dn6);
        let eq38_e398: f64 = (eq38_e396 * var_weff);
        let eq38_e398_d_n1: f64 = (eq38_e396_d_n1 * var_weff);
        let eq38_e398_d_n3: f64 = (eq38_e396_d_n3 * var_weff);
        let eq38_e398_d_n4: f64 = (eq38_e396_d_n4 * var_weff);
        let eq38_e398_d_n5: f64 = (eq38_e396_d_n5 * var_weff);
        let eq38_e398_d_n6: f64 = (eq38_e396_d_n6 * var_weff);
        let eq38_e399: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq38_e398);
        let eq38_value: f64 = eq38_e399;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(4),
            multiplicity * (eq38_value),
            [1, 3, 4, 5, 6],
            [multiplicity * ((eq38_e398_d_n1 * ddt_scale)), multiplicity * ((eq38_e398_d_n3 * ddt_scale)), multiplicity * ((eq38_e398_d_n4 * ddt_scale)), multiplicity * ((eq38_e398_d_n5 * ddt_scale)), multiplicity * ((eq38_e398_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq39_e402: f64 = (var_ttype * var_qjci_1);
        let eq39_e402_d_n1: f64 = (var_ttype * var_qjci_1_dn1);
        let eq39_e402_d_n3: f64 = (var_ttype * var_qjci_1_dn3);
        let eq39_e402_d_n4: f64 = (var_ttype * var_qjci_1_dn4);
        let eq39_e402_d_n5: f64 = (var_ttype * var_qjci_1_dn5);
        let eq39_e402_d_n6: f64 = (var_ttype * var_qjci_1_dn6);
        let eq39_e404: f64 = (eq39_e402 * var_weff);
        let eq39_e404_d_n1: f64 = (eq39_e402_d_n1 * var_weff);
        let eq39_e404_d_n3: f64 = (eq39_e402_d_n3 * var_weff);
        let eq39_e404_d_n4: f64 = (eq39_e402_d_n4 * var_weff);
        let eq39_e404_d_n5: f64 = (eq39_e402_d_n5 * var_weff);
        let eq39_e404_d_n6: f64 = (eq39_e402_d_n6 * var_weff);
        let eq39_e405: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq39_e404);
        let eq39_value: f64 = eq39_e405;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq39_value),
            [1, 3, 4, 5, 6],
            [multiplicity * ((eq39_e404_d_n1 * ddt_scale)), multiplicity * ((eq39_e404_d_n3 * ddt_scale)), multiplicity * ((eq39_e404_d_n4 * ddt_scale)), multiplicity * ((eq39_e404_d_n5 * ddt_scale)), multiplicity * ((eq39_e404_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq40_e408: f64 = (var_ttype * var_qdc);
        let eq40_e408_d_n3: f64 = (var_ttype * var_qdc_dn3);
        let eq40_e408_d_n4: f64 = (var_ttype * var_qdc_dn4);
        let eq40_e408_d_n5: f64 = (var_ttype * var_qdc_dn5);
        let eq40_e408_d_n6: f64 = (var_ttype * var_qdc_dn6);
        let eq40_e410: f64 = (eq40_e408 * var_weff);
        let eq40_e410_d_n3: f64 = (eq40_e408_d_n3 * var_weff);
        let eq40_e410_d_n4: f64 = (eq40_e408_d_n4 * var_weff);
        let eq40_e410_d_n5: f64 = (eq40_e408_d_n5 * var_weff);
        let eq40_e410_d_n6: f64 = (eq40_e408_d_n6 * var_weff);
        let eq40_e411: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq40_e410);
        let eq40_value: f64 = eq40_e411;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq40_value),
            [3, 4, 5, 6],
            [multiplicity * ((eq40_e410_d_n3 * ddt_scale)), multiplicity * ((eq40_e410_d_n4 * ddt_scale)), multiplicity * ((eq40_e410_d_n5 * ddt_scale)), multiplicity * ((eq40_e410_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq41_e414: f64 = (var_ttype * var_qjs);
        let eq41_e414_d_n2: f64 = (var_ttype * var_qjs_dn2);
        let eq41_e414_d_n3: f64 = (var_ttype * var_qjs_dn3);
        let eq41_e414_d_n4: f64 = (var_ttype * var_qjs_dn4);
        let eq41_e416: f64 = (eq41_e414 * var_weff);
        let eq41_e416_d_n2: f64 = (eq41_e414_d_n2 * var_weff);
        let eq41_e416_d_n3: f64 = (eq41_e414_d_n3 * var_weff);
        let eq41_e416_d_n4: f64 = (eq41_e414_d_n4 * var_weff);
        let eq41_e417: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq41_e416);
        let eq41_value: f64 = eq41_e417;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * (eq41_value),
            2,
            multiplicity * ((eq41_e416_d_n2 * ddt_scale)),
            3,
            multiplicity * ((eq41_e416_d_n3 * ddt_scale)),
            4,
            multiplicity * ((eq41_e416_d_n4 * ddt_scale)),
        );
        let eq42_e419: f64 = (-var_qxf1);
        let eq42_e421: f64 = (eq42_e419 * var_weff);
        let eq42_e421_d_n3: f64 = ((-var_qxf1_dn3) * var_weff);
        let eq42_e421_d_n4: f64 = ((-var_qxf1_dn4) * var_weff);
        let eq42_e421_d_n5: f64 = ((-var_qxf1_dn5) * var_weff);
        let eq42_e421_d_n6: f64 = ((-var_qxf1_dn6) * var_weff);
        let eq42_e422: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq42_e421);
        let eq42_value: f64 = eq42_e422;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq42_value),
            [3, 4, 5, 6],
            [multiplicity * ((eq42_e421_d_n3 * ddt_scale)), multiplicity * ((eq42_e421_d_n4 * ddt_scale)), multiplicity * ((eq42_e421_d_n5 * ddt_scale)), multiplicity * ((eq42_e421_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq43_e425: f64 = (var_qxf1 * var_weff);
        let eq43_e425_d_n3: f64 = (var_qxf1_dn3 * var_weff);
        let eq43_e425_d_n4: f64 = (var_qxf1_dn4 * var_weff);
        let eq43_e425_d_n5: f64 = (var_qxf1_dn5 * var_weff);
        let eq43_e425_d_n6: f64 = (var_qxf1_dn6 * var_weff);
        let eq43_e426: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq43_e425);
        let eq43_value: f64 = eq43_e426;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq43_value),
            [3, 4, 5, 6],
            [multiplicity * ((eq43_e425_d_n3 * ddt_scale)), multiplicity * ((eq43_e425_d_n4 * ddt_scale)), multiplicity * ((eq43_e425_d_n5 * ddt_scale)), multiplicity * ((eq43_e425_d_n6 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard13: f64,
        var_guard20: f64,
        var_guard21: f64,
        var_qdc: f64,
        var_qdc_db0: f64,
        var_qdc_db1: f64,
        var_qdc_db2: f64,
        var_qdc_db3: f64,
        var_qdc_db4: f64,
        var_qdc_db5: f64,
        var_qdc_db6: f64,
        var_qdc_db7: f64,
        var_qdc_dn0: f64,
        var_qdc_dn1: f64,
        var_qdc_dn2: f64,
        var_qdc_dn3: f64,
        var_qdc_dn4: f64,
        var_qdc_dn5: f64,
        var_qdc_dn6: f64,
        var_qdc_dn7: f64,
        var_qdc_dn8: f64,
        var_qdc_dn9: f64,
        var_qde: f64,
        var_qde_db0: f64,
        var_qde_db1: f64,
        var_qde_db2: f64,
        var_qde_db3: f64,
        var_qde_db4: f64,
        var_qde_db5: f64,
        var_qde_db6: f64,
        var_qde_db7: f64,
        var_qde_dn0: f64,
        var_qde_dn1: f64,
        var_qde_dn2: f64,
        var_qde_dn3: f64,
        var_qde_dn4: f64,
        var_qde_dn5: f64,
        var_qde_dn6: f64,
        var_qde_dn7: f64,
        var_qde_dn8: f64,
        var_qde_dn9: f64,
        var_qjci_1: f64,
        var_qjci_1_db0: f64,
        var_qjci_1_db1: f64,
        var_qjci_1_db2: f64,
        var_qjci_1_db3: f64,
        var_qjci_1_db4: f64,
        var_qjci_1_db5: f64,
        var_qjci_1_db6: f64,
        var_qjci_1_db7: f64,
        var_qjci_1_dn0: f64,
        var_qjci_1_dn1: f64,
        var_qjci_1_dn2: f64,
        var_qjci_1_dn3: f64,
        var_qjci_1_dn4: f64,
        var_qjci_1_dn5: f64,
        var_qjci_1_dn6: f64,
        var_qjci_1_dn7: f64,
        var_qjci_1_dn8: f64,
        var_qjci_1_dn9: f64,
        var_qjcx_1: f64,
        var_qjcx_1_db0: f64,
        var_qjcx_1_db1: f64,
        var_qjcx_1_db2: f64,
        var_qjcx_1_db3: f64,
        var_qjcx_1_db4: f64,
        var_qjcx_1_db5: f64,
        var_qjcx_1_db6: f64,
        var_qjcx_1_db7: f64,
        var_qjcx_1_dn0: f64,
        var_qjcx_1_dn1: f64,
        var_qjcx_1_dn2: f64,
        var_qjcx_1_dn3: f64,
        var_qjcx_1_dn4: f64,
        var_qjcx_1_dn5: f64,
        var_qjcx_1_dn6: f64,
        var_qjcx_1_dn7: f64,
        var_qjcx_1_dn8: f64,
        var_qjcx_1_dn9: f64,
        var_qje: f64,
        var_qje_db0: f64,
        var_qje_db1: f64,
        var_qje_db2: f64,
        var_qje_db3: f64,
        var_qje_db4: f64,
        var_qje_db5: f64,
        var_qje_db6: f64,
        var_qje_db7: f64,
        var_qje_dn0: f64,
        var_qje_dn1: f64,
        var_qje_dn2: f64,
        var_qje_dn3: f64,
        var_qje_dn4: f64,
        var_qje_dn5: f64,
        var_qje_dn6: f64,
        var_qje_dn7: f64,
        var_qje_dn8: f64,
        var_qje_dn9: f64,
        var_qjs: f64,
        var_qjs_db0: f64,
        var_qjs_db1: f64,
        var_qjs_db2: f64,
        var_qjs_db3: f64,
        var_qjs_db4: f64,
        var_qjs_db5: f64,
        var_qjs_db6: f64,
        var_qjs_db7: f64,
        var_qjs_dn0: f64,
        var_qjs_dn1: f64,
        var_qjs_dn2: f64,
        var_qjs_dn3: f64,
        var_qjs_dn4: f64,
        var_qjs_dn5: f64,
        var_qjs_dn6: f64,
        var_qjs_dn7: f64,
        var_qjs_dn8: f64,
        var_qjs_dn9: f64,
        var_qxf1: f64,
        var_qxf1_db0: f64,
        var_qxf1_db1: f64,
        var_qxf1_db2: f64,
        var_qxf1_db3: f64,
        var_qxf1_db4: f64,
        var_qxf1_db5: f64,
        var_qxf1_db6: f64,
        var_qxf1_db7: f64,
        var_qxf1_dn0: f64,
        var_qxf1_dn1: f64,
        var_qxf1_dn2: f64,
        var_qxf1_dn3: f64,
        var_qxf1_dn4: f64,
        var_qxf1_dn5: f64,
        var_qxf1_dn6: f64,
        var_qxf1_dn7: f64,
        var_qxf1_dn8: f64,
        var_qxf1_dn9: f64,
        var_tff: f64,
        var_tff_db0: f64,
        var_tff_db1: f64,
        var_tff_db2: f64,
        var_tff_db3: f64,
        var_tff_db4: f64,
        var_tff_db5: f64,
        var_tff_db6: f64,
        var_tff_db7: f64,
        var_tff_dn0: f64,
        var_tff_dn1: f64,
        var_tff_dn2: f64,
        var_tff_dn3: f64,
        var_tff_dn4: f64,
        var_tff_dn5: f64,
        var_tff_dn6: f64,
        var_tff_dn7: f64,
        var_tff_dn8: f64,
        var_tff_dn9: f64,
        var_ttype: f64,
        var_ttype_db0: f64,
        var_ttype_db1: f64,
        var_ttype_db2: f64,
        var_ttype_db3: f64,
        var_ttype_db4: f64,
        var_ttype_db5: f64,
        var_ttype_db6: f64,
        var_ttype_db7: f64,
        var_ttype_dn0: f64,
        var_ttype_dn1: f64,
        var_ttype_dn2: f64,
        var_ttype_dn3: f64,
        var_ttype_dn4: f64,
        var_ttype_dn5: f64,
        var_ttype_dn6: f64,
        var_ttype_dn7: f64,
        var_ttype_dn8: f64,
        var_ttype_dn9: f64,
        var_weff: f64,
        var_weff_db0: f64,
        var_weff_db1: f64,
        var_weff_db2: f64,
        var_weff_db3: f64,
        var_weff_db4: f64,
        var_weff_db5: f64,
        var_weff_db6: f64,
        var_weff_db7: f64,
        var_weff_dn0: f64,
        var_weff_dn1: f64,
        var_weff_dn2: f64,
        var_weff_dn3: f64,
        var_weff_dn4: f64,
        var_weff_dn5: f64,
        var_weff_dn6: f64,
        var_weff_dn7: f64,
        var_weff_dn8: f64,
        var_weff_dn9: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq2_e98_q: f64 = (nv9 - 0.0);
        let eq2_e99: f64 = (p.p83 * (nv9 - 0.0));
        let eq2_e99_q: f64 = (p.p83 * eq2_e98_q);
        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * (p.p83),
        );
        let (eq5_e121, eq5_e121_d_n0, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n3, eq5_e121_d_n4, eq5_e121_d_n5, eq5_e121_d_n6, eq5_e121_d_n7, eq5_e121_d_n8, eq5_e121_d_n9, eq5_e121_d_b0, eq5_e121_d_b1, eq5_e121_d_b2, eq5_e121_d_b3, eq5_e121_d_b4, eq5_e121_d_b5, eq5_e121_d_b6, eq5_e121_d_b7, eq5_e121_q, eq5_e121_q_d_n0, eq5_e121_q_d_n1, eq5_e121_q_d_n2, eq5_e121_q_d_n3, eq5_e121_q_d_n4, eq5_e121_q_d_n5, eq5_e121_q_d_n6, eq5_e121_q_d_n7, eq5_e121_q_d_n8, eq5_e121_q_d_n9, eq5_e121_q_d_b0, eq5_e121_q_d_b1, eq5_e121_q_d_b2, eq5_e121_q_d_b3, eq5_e121_q_d_b4, eq5_e121_q_d_b5, eq5_e121_q_d_b6, eq5_e121_q_d_b7,) = {
    if (var_guard13 != 0.0) {
        let eq5_e118_q: f64 = (nv8 - 0.0);
        let eq5_e119: f64 = (var_tff * (nv8 - 0.0));
        let eq5_e119_d_n0: f64 = (var_tff_dn0 * (nv8 - 0.0));
        let eq5_e119_d_n1: f64 = (var_tff_dn1 * (nv8 - 0.0));
        let eq5_e119_d_n2: f64 = (var_tff_dn2 * (nv8 - 0.0));
        let eq5_e119_d_n3: f64 = (var_tff_dn3 * (nv8 - 0.0));
        let eq5_e119_d_n4: f64 = (var_tff_dn4 * (nv8 - 0.0));
        let eq5_e119_d_n5: f64 = (var_tff_dn5 * (nv8 - 0.0));
        let eq5_e119_d_n6: f64 = (var_tff_dn6 * (nv8 - 0.0));
        let eq5_e119_d_n7: f64 = (var_tff_dn7 * (nv8 - 0.0));
        let eq5_e119_d_n8: f64 = ((var_tff_dn8 * (nv8 - 0.0)) + var_tff);
        let eq5_e119_d_n9: f64 = (var_tff_dn9 * (nv8 - 0.0));
        let eq5_e119_d_b0: f64 = (var_tff_db0 * (nv8 - 0.0));
        let eq5_e119_d_b1: f64 = (var_tff_db1 * (nv8 - 0.0));
        let eq5_e119_d_b2: f64 = (var_tff_db2 * (nv8 - 0.0));
        let eq5_e119_d_b3: f64 = (var_tff_db3 * (nv8 - 0.0));
        let eq5_e119_d_b4: f64 = (var_tff_db4 * (nv8 - 0.0));
        let eq5_e119_d_b5: f64 = (var_tff_db5 * (nv8 - 0.0));
        let eq5_e119_d_b6: f64 = (var_tff_db6 * (nv8 - 0.0));
        let eq5_e119_d_b7: f64 = (var_tff_db7 * (nv8 - 0.0));
        let eq5_e119_q: f64 = (var_tff * eq5_e118_q);
        let eq5_e119_q_d_n0: f64 = (var_tff_dn0 * eq5_e118_q);
        let eq5_e119_q_d_n1: f64 = (var_tff_dn1 * eq5_e118_q);
        let eq5_e119_q_d_n2: f64 = (var_tff_dn2 * eq5_e118_q);
        let eq5_e119_q_d_n3: f64 = (var_tff_dn3 * eq5_e118_q);
        let eq5_e119_q_d_n4: f64 = (var_tff_dn4 * eq5_e118_q);
        let eq5_e119_q_d_n5: f64 = (var_tff_dn5 * eq5_e118_q);
        let eq5_e119_q_d_n6: f64 = (var_tff_dn6 * eq5_e118_q);
        let eq5_e119_q_d_n7: f64 = (var_tff_dn7 * eq5_e118_q);
        let eq5_e119_q_d_n8: f64 = ((var_tff_dn8 * eq5_e118_q) + var_tff);
        let eq5_e119_q_d_n9: f64 = (var_tff_dn9 * eq5_e118_q);
        let eq5_e119_q_d_b0: f64 = (var_tff_db0 * eq5_e118_q);
        let eq5_e119_q_d_b1: f64 = (var_tff_db1 * eq5_e118_q);
        let eq5_e119_q_d_b2: f64 = (var_tff_db2 * eq5_e118_q);
        let eq5_e119_q_d_b3: f64 = (var_tff_db3 * eq5_e118_q);
        let eq5_e119_q_d_b4: f64 = (var_tff_db4 * eq5_e118_q);
        let eq5_e119_q_d_b5: f64 = (var_tff_db5 * eq5_e118_q);
        let eq5_e119_q_d_b6: f64 = (var_tff_db6 * eq5_e118_q);
        let eq5_e119_q_d_b7: f64 = (var_tff_db7 * eq5_e118_q);
        (eq5_e119, eq5_e119_d_n0, eq5_e119_d_n1, eq5_e119_d_n2, eq5_e119_d_n3, eq5_e119_d_n4, eq5_e119_d_n5, eq5_e119_d_n6, eq5_e119_d_n7, eq5_e119_d_n8, eq5_e119_d_n9, eq5_e119_d_b0, eq5_e119_d_b1, eq5_e119_d_b2, eq5_e119_d_b3, eq5_e119_d_b4, eq5_e119_d_b5, eq5_e119_d_b6, eq5_e119_d_b7, eq5_e119_q, eq5_e119_q_d_n0, eq5_e119_q_d_n1, eq5_e119_q_d_n2, eq5_e119_q_d_n3, eq5_e119_q_d_n4, eq5_e119_q_d_n5, eq5_e119_q_d_n6, eq5_e119_q_d_n7, eq5_e119_q_d_n8, eq5_e119_q_d_n9, eq5_e119_q_d_b0, eq5_e119_q_d_b1, eq5_e119_q_d_b2, eq5_e119_q_d_b3, eq5_e119_q_d_b4, eq5_e119_q_d_b5, eq5_e119_q_d_b6, eq5_e119_q_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_reactive_node_derivatives: [f64; 10] = [eq5_e121_q_d_n0, eq5_e121_q_d_n1, eq5_e121_q_d_n2, eq5_e121_q_d_n3, eq5_e121_q_d_n4, eq5_e121_q_d_n5, eq5_e121_q_d_n6, eq5_e121_q_d_n7, eq5_e121_q_d_n8, eq5_e121_q_d_n9];
        let eq5_reactive_branch_derivatives: [f64; 8] = [eq5_e121_q_d_b0, eq5_e121_q_d_b1, eq5_e121_q_d_b2, eq5_e121_q_d_b3, eq5_e121_q_d_b4, eq5_e121_q_d_b5, eq5_e121_q_d_b6, eq5_e121_q_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            None,
            nodes,
            &eq5_reactive_node_derivatives,
            branches,
            &eq5_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq9_e154, eq9_e154_d_n3, eq9_e154_q,) = {
    if (var_guard20 != 0.0) {
        let eq9_e151: f64 = ((nv3 - 0.0) * p.p34);
        let eq9_e152_q: f64 = eq9_e151;
        (eq9_e151, p.p34, eq9_e152_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq9_e154_d_n3),
        );
        let (eq13_e195, eq13_e195_d_n3, eq13_e195_q,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq13_e192: f64 = (p.p34 * (nv3 - 0.0));
        let eq13_e193_q: f64 = eq13_e192;
        (eq13_e192, p.p34, eq13_e193_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq13_e195_d_n3),
        );
        let (eq15_e214, eq15_e214_d_n7, eq15_e214_q,) = {
    if ((var_guard20 == 0.0) && (var_guard21 != 0.0)) {
        let eq15_e211: f64 = (p.p36 * (nv7 - 0.0));
        let eq15_e212_q: f64 = eq15_e211;
        (eq15_e211, p.p36, eq15_e212_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            None,
            nodes[7],
            multiplicity * (eq15_e214_d_n7),
        );
        let eq36_e384: f64 = (var_ttype * var_qje);
        let eq36_e384_d_n0: f64 = ((var_ttype_dn0 * var_qje) + (var_ttype * var_qje_dn0));
        let eq36_e384_d_n1: f64 = ((var_ttype_dn1 * var_qje) + (var_ttype * var_qje_dn1));
        let eq36_e384_d_n2: f64 = ((var_ttype_dn2 * var_qje) + (var_ttype * var_qje_dn2));
        let eq36_e384_d_n3: f64 = ((var_ttype_dn3 * var_qje) + (var_ttype * var_qje_dn3));
        let eq36_e384_d_n4: f64 = ((var_ttype_dn4 * var_qje) + (var_ttype * var_qje_dn4));
        let eq36_e384_d_n5: f64 = ((var_ttype_dn5 * var_qje) + (var_ttype * var_qje_dn5));
        let eq36_e384_d_n6: f64 = ((var_ttype_dn6 * var_qje) + (var_ttype * var_qje_dn6));
        let eq36_e384_d_n7: f64 = ((var_ttype_dn7 * var_qje) + (var_ttype * var_qje_dn7));
        let eq36_e384_d_n8: f64 = ((var_ttype_dn8 * var_qje) + (var_ttype * var_qje_dn8));
        let eq36_e384_d_n9: f64 = ((var_ttype_dn9 * var_qje) + (var_ttype * var_qje_dn9));
        let eq36_e384_d_b0: f64 = ((var_ttype_db0 * var_qje) + (var_ttype * var_qje_db0));
        let eq36_e384_d_b1: f64 = ((var_ttype_db1 * var_qje) + (var_ttype * var_qje_db1));
        let eq36_e384_d_b2: f64 = ((var_ttype_db2 * var_qje) + (var_ttype * var_qje_db2));
        let eq36_e384_d_b3: f64 = ((var_ttype_db3 * var_qje) + (var_ttype * var_qje_db3));
        let eq36_e384_d_b4: f64 = ((var_ttype_db4 * var_qje) + (var_ttype * var_qje_db4));
        let eq36_e384_d_b5: f64 = ((var_ttype_db5 * var_qje) + (var_ttype * var_qje_db5));
        let eq36_e384_d_b6: f64 = ((var_ttype_db6 * var_qje) + (var_ttype * var_qje_db6));
        let eq36_e384_d_b7: f64 = ((var_ttype_db7 * var_qje) + (var_ttype * var_qje_db7));
        let eq36_e386: f64 = (eq36_e384 * var_weff);
        let eq36_e386_d_n0: f64 = ((eq36_e384_d_n0 * var_weff) + (eq36_e384 * var_weff_dn0));
        let eq36_e386_d_n1: f64 = ((eq36_e384_d_n1 * var_weff) + (eq36_e384 * var_weff_dn1));
        let eq36_e386_d_n2: f64 = ((eq36_e384_d_n2 * var_weff) + (eq36_e384 * var_weff_dn2));
        let eq36_e386_d_n3: f64 = ((eq36_e384_d_n3 * var_weff) + (eq36_e384 * var_weff_dn3));
        let eq36_e386_d_n4: f64 = ((eq36_e384_d_n4 * var_weff) + (eq36_e384 * var_weff_dn4));
        let eq36_e386_d_n5: f64 = ((eq36_e384_d_n5 * var_weff) + (eq36_e384 * var_weff_dn5));
        let eq36_e386_d_n6: f64 = ((eq36_e384_d_n6 * var_weff) + (eq36_e384 * var_weff_dn6));
        let eq36_e386_d_n7: f64 = ((eq36_e384_d_n7 * var_weff) + (eq36_e384 * var_weff_dn7));
        let eq36_e386_d_n8: f64 = ((eq36_e384_d_n8 * var_weff) + (eq36_e384 * var_weff_dn8));
        let eq36_e386_d_n9: f64 = ((eq36_e384_d_n9 * var_weff) + (eq36_e384 * var_weff_dn9));
        let eq36_e386_d_b0: f64 = ((eq36_e384_d_b0 * var_weff) + (eq36_e384 * var_weff_db0));
        let eq36_e386_d_b1: f64 = ((eq36_e384_d_b1 * var_weff) + (eq36_e384 * var_weff_db1));
        let eq36_e386_d_b2: f64 = ((eq36_e384_d_b2 * var_weff) + (eq36_e384 * var_weff_db2));
        let eq36_e386_d_b3: f64 = ((eq36_e384_d_b3 * var_weff) + (eq36_e384 * var_weff_db3));
        let eq36_e386_d_b4: f64 = ((eq36_e384_d_b4 * var_weff) + (eq36_e384 * var_weff_db4));
        let eq36_e386_d_b5: f64 = ((eq36_e384_d_b5 * var_weff) + (eq36_e384 * var_weff_db5));
        let eq36_e386_d_b6: f64 = ((eq36_e384_d_b6 * var_weff) + (eq36_e384 * var_weff_db6));
        let eq36_e386_d_b7: f64 = ((eq36_e384_d_b7 * var_weff) + (eq36_e384 * var_weff_db7));
        let eq36_e387_q: f64 = eq36_e386;
        let eq36_reactive_node_derivatives: [f64; 10] = [eq36_e386_d_n0, eq36_e386_d_n1, eq36_e386_d_n2, eq36_e386_d_n3, eq36_e386_d_n4, eq36_e386_d_n5, eq36_e386_d_n6, eq36_e386_d_n7, eq36_e386_d_n8, eq36_e386_d_n9];
        let eq36_reactive_branch_derivatives: [f64; 8] = [eq36_e386_d_b0, eq36_e386_d_b1, eq36_e386_d_b2, eq36_e386_d_b3, eq36_e386_d_b4, eq36_e386_d_b5, eq36_e386_d_b6, eq36_e386_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e390: f64 = (var_ttype * var_qde);
        let eq37_e390_d_n0: f64 = ((var_ttype_dn0 * var_qde) + (var_ttype * var_qde_dn0));
        let eq37_e390_d_n1: f64 = ((var_ttype_dn1 * var_qde) + (var_ttype * var_qde_dn1));
        let eq37_e390_d_n2: f64 = ((var_ttype_dn2 * var_qde) + (var_ttype * var_qde_dn2));
        let eq37_e390_d_n3: f64 = ((var_ttype_dn3 * var_qde) + (var_ttype * var_qde_dn3));
        let eq37_e390_d_n4: f64 = ((var_ttype_dn4 * var_qde) + (var_ttype * var_qde_dn4));
        let eq37_e390_d_n5: f64 = ((var_ttype_dn5 * var_qde) + (var_ttype * var_qde_dn5));
        let eq37_e390_d_n6: f64 = ((var_ttype_dn6 * var_qde) + (var_ttype * var_qde_dn6));
        let eq37_e390_d_n7: f64 = ((var_ttype_dn7 * var_qde) + (var_ttype * var_qde_dn7));
        let eq37_e390_d_n8: f64 = ((var_ttype_dn8 * var_qde) + (var_ttype * var_qde_dn8));
        let eq37_e390_d_n9: f64 = ((var_ttype_dn9 * var_qde) + (var_ttype * var_qde_dn9));
        let eq37_e390_d_b0: f64 = ((var_ttype_db0 * var_qde) + (var_ttype * var_qde_db0));
        let eq37_e390_d_b1: f64 = ((var_ttype_db1 * var_qde) + (var_ttype * var_qde_db1));
        let eq37_e390_d_b2: f64 = ((var_ttype_db2 * var_qde) + (var_ttype * var_qde_db2));
        let eq37_e390_d_b3: f64 = ((var_ttype_db3 * var_qde) + (var_ttype * var_qde_db3));
        let eq37_e390_d_b4: f64 = ((var_ttype_db4 * var_qde) + (var_ttype * var_qde_db4));
        let eq37_e390_d_b5: f64 = ((var_ttype_db5 * var_qde) + (var_ttype * var_qde_db5));
        let eq37_e390_d_b6: f64 = ((var_ttype_db6 * var_qde) + (var_ttype * var_qde_db6));
        let eq37_e390_d_b7: f64 = ((var_ttype_db7 * var_qde) + (var_ttype * var_qde_db7));
        let eq37_e392: f64 = (eq37_e390 * var_weff);
        let eq37_e392_d_n0: f64 = ((eq37_e390_d_n0 * var_weff) + (eq37_e390 * var_weff_dn0));
        let eq37_e392_d_n1: f64 = ((eq37_e390_d_n1 * var_weff) + (eq37_e390 * var_weff_dn1));
        let eq37_e392_d_n2: f64 = ((eq37_e390_d_n2 * var_weff) + (eq37_e390 * var_weff_dn2));
        let eq37_e392_d_n3: f64 = ((eq37_e390_d_n3 * var_weff) + (eq37_e390 * var_weff_dn3));
        let eq37_e392_d_n4: f64 = ((eq37_e390_d_n4 * var_weff) + (eq37_e390 * var_weff_dn4));
        let eq37_e392_d_n5: f64 = ((eq37_e390_d_n5 * var_weff) + (eq37_e390 * var_weff_dn5));
        let eq37_e392_d_n6: f64 = ((eq37_e390_d_n6 * var_weff) + (eq37_e390 * var_weff_dn6));
        let eq37_e392_d_n7: f64 = ((eq37_e390_d_n7 * var_weff) + (eq37_e390 * var_weff_dn7));
        let eq37_e392_d_n8: f64 = ((eq37_e390_d_n8 * var_weff) + (eq37_e390 * var_weff_dn8));
        let eq37_e392_d_n9: f64 = ((eq37_e390_d_n9 * var_weff) + (eq37_e390 * var_weff_dn9));
        let eq37_e392_d_b0: f64 = ((eq37_e390_d_b0 * var_weff) + (eq37_e390 * var_weff_db0));
        let eq37_e392_d_b1: f64 = ((eq37_e390_d_b1 * var_weff) + (eq37_e390 * var_weff_db1));
        let eq37_e392_d_b2: f64 = ((eq37_e390_d_b2 * var_weff) + (eq37_e390 * var_weff_db2));
        let eq37_e392_d_b3: f64 = ((eq37_e390_d_b3 * var_weff) + (eq37_e390 * var_weff_db3));
        let eq37_e392_d_b4: f64 = ((eq37_e390_d_b4 * var_weff) + (eq37_e390 * var_weff_db4));
        let eq37_e392_d_b5: f64 = ((eq37_e390_d_b5 * var_weff) + (eq37_e390 * var_weff_db5));
        let eq37_e392_d_b6: f64 = ((eq37_e390_d_b6 * var_weff) + (eq37_e390 * var_weff_db6));
        let eq37_e392_d_b7: f64 = ((eq37_e390_d_b7 * var_weff) + (eq37_e390 * var_weff_db7));
        let eq37_e393_q: f64 = eq37_e392;
        let eq37_reactive_node_derivatives: [f64; 10] = [eq37_e392_d_n0, eq37_e392_d_n1, eq37_e392_d_n2, eq37_e392_d_n3, eq37_e392_d_n4, eq37_e392_d_n5, eq37_e392_d_n6, eq37_e392_d_n7, eq37_e392_d_n8, eq37_e392_d_n9];
        let eq37_reactive_branch_derivatives: [f64; 8] = [eq37_e392_d_b0, eq37_e392_d_b1, eq37_e392_d_b2, eq37_e392_d_b3, eq37_e392_d_b4, eq37_e392_d_b5, eq37_e392_d_b6, eq37_e392_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let eq38_e396: f64 = (var_ttype * var_qjcx_1);
        let eq38_e396_d_n0: f64 = ((var_ttype_dn0 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn0));
        let eq38_e396_d_n1: f64 = ((var_ttype_dn1 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn1));
        let eq38_e396_d_n2: f64 = ((var_ttype_dn2 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn2));
        let eq38_e396_d_n3: f64 = ((var_ttype_dn3 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn3));
        let eq38_e396_d_n4: f64 = ((var_ttype_dn4 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn4));
        let eq38_e396_d_n5: f64 = ((var_ttype_dn5 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn5));
        let eq38_e396_d_n6: f64 = ((var_ttype_dn6 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn6));
        let eq38_e396_d_n7: f64 = ((var_ttype_dn7 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn7));
        let eq38_e396_d_n8: f64 = ((var_ttype_dn8 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn8));
        let eq38_e396_d_n9: f64 = ((var_ttype_dn9 * var_qjcx_1) + (var_ttype * var_qjcx_1_dn9));
        let eq38_e396_d_b0: f64 = ((var_ttype_db0 * var_qjcx_1) + (var_ttype * var_qjcx_1_db0));
        let eq38_e396_d_b1: f64 = ((var_ttype_db1 * var_qjcx_1) + (var_ttype * var_qjcx_1_db1));
        let eq38_e396_d_b2: f64 = ((var_ttype_db2 * var_qjcx_1) + (var_ttype * var_qjcx_1_db2));
        let eq38_e396_d_b3: f64 = ((var_ttype_db3 * var_qjcx_1) + (var_ttype * var_qjcx_1_db3));
        let eq38_e396_d_b4: f64 = ((var_ttype_db4 * var_qjcx_1) + (var_ttype * var_qjcx_1_db4));
        let eq38_e396_d_b5: f64 = ((var_ttype_db5 * var_qjcx_1) + (var_ttype * var_qjcx_1_db5));
        let eq38_e396_d_b6: f64 = ((var_ttype_db6 * var_qjcx_1) + (var_ttype * var_qjcx_1_db6));
        let eq38_e396_d_b7: f64 = ((var_ttype_db7 * var_qjcx_1) + (var_ttype * var_qjcx_1_db7));
        let eq38_e398: f64 = (eq38_e396 * var_weff);
        let eq38_e398_d_n0: f64 = ((eq38_e396_d_n0 * var_weff) + (eq38_e396 * var_weff_dn0));
        let eq38_e398_d_n1: f64 = ((eq38_e396_d_n1 * var_weff) + (eq38_e396 * var_weff_dn1));
        let eq38_e398_d_n2: f64 = ((eq38_e396_d_n2 * var_weff) + (eq38_e396 * var_weff_dn2));
        let eq38_e398_d_n3: f64 = ((eq38_e396_d_n3 * var_weff) + (eq38_e396 * var_weff_dn3));
        let eq38_e398_d_n4: f64 = ((eq38_e396_d_n4 * var_weff) + (eq38_e396 * var_weff_dn4));
        let eq38_e398_d_n5: f64 = ((eq38_e396_d_n5 * var_weff) + (eq38_e396 * var_weff_dn5));
        let eq38_e398_d_n6: f64 = ((eq38_e396_d_n6 * var_weff) + (eq38_e396 * var_weff_dn6));
        let eq38_e398_d_n7: f64 = ((eq38_e396_d_n7 * var_weff) + (eq38_e396 * var_weff_dn7));
        let eq38_e398_d_n8: f64 = ((eq38_e396_d_n8 * var_weff) + (eq38_e396 * var_weff_dn8));
        let eq38_e398_d_n9: f64 = ((eq38_e396_d_n9 * var_weff) + (eq38_e396 * var_weff_dn9));
        let eq38_e398_d_b0: f64 = ((eq38_e396_d_b0 * var_weff) + (eq38_e396 * var_weff_db0));
        let eq38_e398_d_b1: f64 = ((eq38_e396_d_b1 * var_weff) + (eq38_e396 * var_weff_db1));
        let eq38_e398_d_b2: f64 = ((eq38_e396_d_b2 * var_weff) + (eq38_e396 * var_weff_db2));
        let eq38_e398_d_b3: f64 = ((eq38_e396_d_b3 * var_weff) + (eq38_e396 * var_weff_db3));
        let eq38_e398_d_b4: f64 = ((eq38_e396_d_b4 * var_weff) + (eq38_e396 * var_weff_db4));
        let eq38_e398_d_b5: f64 = ((eq38_e396_d_b5 * var_weff) + (eq38_e396 * var_weff_db5));
        let eq38_e398_d_b6: f64 = ((eq38_e396_d_b6 * var_weff) + (eq38_e396 * var_weff_db6));
        let eq38_e398_d_b7: f64 = ((eq38_e396_d_b7 * var_weff) + (eq38_e396 * var_weff_db7));
        let eq38_e399_q: f64 = eq38_e398;
        let eq38_reactive_node_derivatives: [f64; 10] = [eq38_e398_d_n0, eq38_e398_d_n1, eq38_e398_d_n2, eq38_e398_d_n3, eq38_e398_d_n4, eq38_e398_d_n5, eq38_e398_d_n6, eq38_e398_d_n7, eq38_e398_d_n8, eq38_e398_d_n9];
        let eq38_reactive_branch_derivatives: [f64; 8] = [eq38_e398_d_b0, eq38_e398_d_b1, eq38_e398_d_b2, eq38_e398_d_b3, eq38_e398_d_b4, eq38_e398_d_b5, eq38_e398_d_b6, eq38_e398_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let eq39_e402: f64 = (var_ttype * var_qjci_1);
        let eq39_e402_d_n0: f64 = ((var_ttype_dn0 * var_qjci_1) + (var_ttype * var_qjci_1_dn0));
        let eq39_e402_d_n1: f64 = ((var_ttype_dn1 * var_qjci_1) + (var_ttype * var_qjci_1_dn1));
        let eq39_e402_d_n2: f64 = ((var_ttype_dn2 * var_qjci_1) + (var_ttype * var_qjci_1_dn2));
        let eq39_e402_d_n3: f64 = ((var_ttype_dn3 * var_qjci_1) + (var_ttype * var_qjci_1_dn3));
        let eq39_e402_d_n4: f64 = ((var_ttype_dn4 * var_qjci_1) + (var_ttype * var_qjci_1_dn4));
        let eq39_e402_d_n5: f64 = ((var_ttype_dn5 * var_qjci_1) + (var_ttype * var_qjci_1_dn5));
        let eq39_e402_d_n6: f64 = ((var_ttype_dn6 * var_qjci_1) + (var_ttype * var_qjci_1_dn6));
        let eq39_e402_d_n7: f64 = ((var_ttype_dn7 * var_qjci_1) + (var_ttype * var_qjci_1_dn7));
        let eq39_e402_d_n8: f64 = ((var_ttype_dn8 * var_qjci_1) + (var_ttype * var_qjci_1_dn8));
        let eq39_e402_d_n9: f64 = ((var_ttype_dn9 * var_qjci_1) + (var_ttype * var_qjci_1_dn9));
        let eq39_e402_d_b0: f64 = ((var_ttype_db0 * var_qjci_1) + (var_ttype * var_qjci_1_db0));
        let eq39_e402_d_b1: f64 = ((var_ttype_db1 * var_qjci_1) + (var_ttype * var_qjci_1_db1));
        let eq39_e402_d_b2: f64 = ((var_ttype_db2 * var_qjci_1) + (var_ttype * var_qjci_1_db2));
        let eq39_e402_d_b3: f64 = ((var_ttype_db3 * var_qjci_1) + (var_ttype * var_qjci_1_db3));
        let eq39_e402_d_b4: f64 = ((var_ttype_db4 * var_qjci_1) + (var_ttype * var_qjci_1_db4));
        let eq39_e402_d_b5: f64 = ((var_ttype_db5 * var_qjci_1) + (var_ttype * var_qjci_1_db5));
        let eq39_e402_d_b6: f64 = ((var_ttype_db6 * var_qjci_1) + (var_ttype * var_qjci_1_db6));
        let eq39_e402_d_b7: f64 = ((var_ttype_db7 * var_qjci_1) + (var_ttype * var_qjci_1_db7));
        let eq39_e404: f64 = (eq39_e402 * var_weff);
        let eq39_e404_d_n0: f64 = ((eq39_e402_d_n0 * var_weff) + (eq39_e402 * var_weff_dn0));
        let eq39_e404_d_n1: f64 = ((eq39_e402_d_n1 * var_weff) + (eq39_e402 * var_weff_dn1));
        let eq39_e404_d_n2: f64 = ((eq39_e402_d_n2 * var_weff) + (eq39_e402 * var_weff_dn2));
        let eq39_e404_d_n3: f64 = ((eq39_e402_d_n3 * var_weff) + (eq39_e402 * var_weff_dn3));
        let eq39_e404_d_n4: f64 = ((eq39_e402_d_n4 * var_weff) + (eq39_e402 * var_weff_dn4));
        let eq39_e404_d_n5: f64 = ((eq39_e402_d_n5 * var_weff) + (eq39_e402 * var_weff_dn5));
        let eq39_e404_d_n6: f64 = ((eq39_e402_d_n6 * var_weff) + (eq39_e402 * var_weff_dn6));
        let eq39_e404_d_n7: f64 = ((eq39_e402_d_n7 * var_weff) + (eq39_e402 * var_weff_dn7));
        let eq39_e404_d_n8: f64 = ((eq39_e402_d_n8 * var_weff) + (eq39_e402 * var_weff_dn8));
        let eq39_e404_d_n9: f64 = ((eq39_e402_d_n9 * var_weff) + (eq39_e402 * var_weff_dn9));
        let eq39_e404_d_b0: f64 = ((eq39_e402_d_b0 * var_weff) + (eq39_e402 * var_weff_db0));
        let eq39_e404_d_b1: f64 = ((eq39_e402_d_b1 * var_weff) + (eq39_e402 * var_weff_db1));
        let eq39_e404_d_b2: f64 = ((eq39_e402_d_b2 * var_weff) + (eq39_e402 * var_weff_db2));
        let eq39_e404_d_b3: f64 = ((eq39_e402_d_b3 * var_weff) + (eq39_e402 * var_weff_db3));
        let eq39_e404_d_b4: f64 = ((eq39_e402_d_b4 * var_weff) + (eq39_e402 * var_weff_db4));
        let eq39_e404_d_b5: f64 = ((eq39_e402_d_b5 * var_weff) + (eq39_e402 * var_weff_db5));
        let eq39_e404_d_b6: f64 = ((eq39_e402_d_b6 * var_weff) + (eq39_e402 * var_weff_db6));
        let eq39_e404_d_b7: f64 = ((eq39_e402_d_b7 * var_weff) + (eq39_e402 * var_weff_db7));
        let eq39_e405_q: f64 = eq39_e404;
        let eq39_reactive_node_derivatives: [f64; 10] = [eq39_e404_d_n0, eq39_e404_d_n1, eq39_e404_d_n2, eq39_e404_d_n3, eq39_e404_d_n4, eq39_e404_d_n5, eq39_e404_d_n6, eq39_e404_d_n7, eq39_e404_d_n8, eq39_e404_d_n9];
        let eq39_reactive_branch_derivatives: [f64; 8] = [eq39_e404_d_b0, eq39_e404_d_b1, eq39_e404_d_b2, eq39_e404_d_b3, eq39_e404_d_b4, eq39_e404_d_b5, eq39_e404_d_b6, eq39_e404_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let eq40_e408: f64 = (var_ttype * var_qdc);
        let eq40_e408_d_n0: f64 = ((var_ttype_dn0 * var_qdc) + (var_ttype * var_qdc_dn0));
        let eq40_e408_d_n1: f64 = ((var_ttype_dn1 * var_qdc) + (var_ttype * var_qdc_dn1));
        let eq40_e408_d_n2: f64 = ((var_ttype_dn2 * var_qdc) + (var_ttype * var_qdc_dn2));
        let eq40_e408_d_n3: f64 = ((var_ttype_dn3 * var_qdc) + (var_ttype * var_qdc_dn3));
        let eq40_e408_d_n4: f64 = ((var_ttype_dn4 * var_qdc) + (var_ttype * var_qdc_dn4));
        let eq40_e408_d_n5: f64 = ((var_ttype_dn5 * var_qdc) + (var_ttype * var_qdc_dn5));
        let eq40_e408_d_n6: f64 = ((var_ttype_dn6 * var_qdc) + (var_ttype * var_qdc_dn6));
        let eq40_e408_d_n7: f64 = ((var_ttype_dn7 * var_qdc) + (var_ttype * var_qdc_dn7));
        let eq40_e408_d_n8: f64 = ((var_ttype_dn8 * var_qdc) + (var_ttype * var_qdc_dn8));
        let eq40_e408_d_n9: f64 = ((var_ttype_dn9 * var_qdc) + (var_ttype * var_qdc_dn9));
        let eq40_e408_d_b0: f64 = ((var_ttype_db0 * var_qdc) + (var_ttype * var_qdc_db0));
        let eq40_e408_d_b1: f64 = ((var_ttype_db1 * var_qdc) + (var_ttype * var_qdc_db1));
        let eq40_e408_d_b2: f64 = ((var_ttype_db2 * var_qdc) + (var_ttype * var_qdc_db2));
        let eq40_e408_d_b3: f64 = ((var_ttype_db3 * var_qdc) + (var_ttype * var_qdc_db3));
        let eq40_e408_d_b4: f64 = ((var_ttype_db4 * var_qdc) + (var_ttype * var_qdc_db4));
        let eq40_e408_d_b5: f64 = ((var_ttype_db5 * var_qdc) + (var_ttype * var_qdc_db5));
        let eq40_e408_d_b6: f64 = ((var_ttype_db6 * var_qdc) + (var_ttype * var_qdc_db6));
        let eq40_e408_d_b7: f64 = ((var_ttype_db7 * var_qdc) + (var_ttype * var_qdc_db7));
        let eq40_e410: f64 = (eq40_e408 * var_weff);
        let eq40_e410_d_n0: f64 = ((eq40_e408_d_n0 * var_weff) + (eq40_e408 * var_weff_dn0));
        let eq40_e410_d_n1: f64 = ((eq40_e408_d_n1 * var_weff) + (eq40_e408 * var_weff_dn1));
        let eq40_e410_d_n2: f64 = ((eq40_e408_d_n2 * var_weff) + (eq40_e408 * var_weff_dn2));
        let eq40_e410_d_n3: f64 = ((eq40_e408_d_n3 * var_weff) + (eq40_e408 * var_weff_dn3));
        let eq40_e410_d_n4: f64 = ((eq40_e408_d_n4 * var_weff) + (eq40_e408 * var_weff_dn4));
        let eq40_e410_d_n5: f64 = ((eq40_e408_d_n5 * var_weff) + (eq40_e408 * var_weff_dn5));
        let eq40_e410_d_n6: f64 = ((eq40_e408_d_n6 * var_weff) + (eq40_e408 * var_weff_dn6));
        let eq40_e410_d_n7: f64 = ((eq40_e408_d_n7 * var_weff) + (eq40_e408 * var_weff_dn7));
        let eq40_e410_d_n8: f64 = ((eq40_e408_d_n8 * var_weff) + (eq40_e408 * var_weff_dn8));
        let eq40_e410_d_n9: f64 = ((eq40_e408_d_n9 * var_weff) + (eq40_e408 * var_weff_dn9));
        let eq40_e410_d_b0: f64 = ((eq40_e408_d_b0 * var_weff) + (eq40_e408 * var_weff_db0));
        let eq40_e410_d_b1: f64 = ((eq40_e408_d_b1 * var_weff) + (eq40_e408 * var_weff_db1));
        let eq40_e410_d_b2: f64 = ((eq40_e408_d_b2 * var_weff) + (eq40_e408 * var_weff_db2));
        let eq40_e410_d_b3: f64 = ((eq40_e408_d_b3 * var_weff) + (eq40_e408 * var_weff_db3));
        let eq40_e410_d_b4: f64 = ((eq40_e408_d_b4 * var_weff) + (eq40_e408 * var_weff_db4));
        let eq40_e410_d_b5: f64 = ((eq40_e408_d_b5 * var_weff) + (eq40_e408 * var_weff_db5));
        let eq40_e410_d_b6: f64 = ((eq40_e408_d_b6 * var_weff) + (eq40_e408 * var_weff_db6));
        let eq40_e410_d_b7: f64 = ((eq40_e408_d_b7 * var_weff) + (eq40_e408 * var_weff_db7));
        let eq40_e411_q: f64 = eq40_e410;
        let eq40_reactive_node_derivatives: [f64; 10] = [eq40_e410_d_n0, eq40_e410_d_n1, eq40_e410_d_n2, eq40_e410_d_n3, eq40_e410_d_n4, eq40_e410_d_n5, eq40_e410_d_n6, eq40_e410_d_n7, eq40_e410_d_n8, eq40_e410_d_n9];
        let eq40_reactive_branch_derivatives: [f64; 8] = [eq40_e410_d_b0, eq40_e410_d_b1, eq40_e410_d_b2, eq40_e410_d_b3, eq40_e410_d_b4, eq40_e410_d_b5, eq40_e410_d_b6, eq40_e410_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e414: f64 = (var_ttype * var_qjs);
        let eq41_e414_d_n0: f64 = ((var_ttype_dn0 * var_qjs) + (var_ttype * var_qjs_dn0));
        let eq41_e414_d_n1: f64 = ((var_ttype_dn1 * var_qjs) + (var_ttype * var_qjs_dn1));
        let eq41_e414_d_n2: f64 = ((var_ttype_dn2 * var_qjs) + (var_ttype * var_qjs_dn2));
        let eq41_e414_d_n3: f64 = ((var_ttype_dn3 * var_qjs) + (var_ttype * var_qjs_dn3));
        let eq41_e414_d_n4: f64 = ((var_ttype_dn4 * var_qjs) + (var_ttype * var_qjs_dn4));
        let eq41_e414_d_n5: f64 = ((var_ttype_dn5 * var_qjs) + (var_ttype * var_qjs_dn5));
        let eq41_e414_d_n6: f64 = ((var_ttype_dn6 * var_qjs) + (var_ttype * var_qjs_dn6));
        let eq41_e414_d_n7: f64 = ((var_ttype_dn7 * var_qjs) + (var_ttype * var_qjs_dn7));
        let eq41_e414_d_n8: f64 = ((var_ttype_dn8 * var_qjs) + (var_ttype * var_qjs_dn8));
        let eq41_e414_d_n9: f64 = ((var_ttype_dn9 * var_qjs) + (var_ttype * var_qjs_dn9));
        let eq41_e414_d_b0: f64 = ((var_ttype_db0 * var_qjs) + (var_ttype * var_qjs_db0));
        let eq41_e414_d_b1: f64 = ((var_ttype_db1 * var_qjs) + (var_ttype * var_qjs_db1));
        let eq41_e414_d_b2: f64 = ((var_ttype_db2 * var_qjs) + (var_ttype * var_qjs_db2));
        let eq41_e414_d_b3: f64 = ((var_ttype_db3 * var_qjs) + (var_ttype * var_qjs_db3));
        let eq41_e414_d_b4: f64 = ((var_ttype_db4 * var_qjs) + (var_ttype * var_qjs_db4));
        let eq41_e414_d_b5: f64 = ((var_ttype_db5 * var_qjs) + (var_ttype * var_qjs_db5));
        let eq41_e414_d_b6: f64 = ((var_ttype_db6 * var_qjs) + (var_ttype * var_qjs_db6));
        let eq41_e414_d_b7: f64 = ((var_ttype_db7 * var_qjs) + (var_ttype * var_qjs_db7));
        let eq41_e416: f64 = (eq41_e414 * var_weff);
        let eq41_e416_d_n0: f64 = ((eq41_e414_d_n0 * var_weff) + (eq41_e414 * var_weff_dn0));
        let eq41_e416_d_n1: f64 = ((eq41_e414_d_n1 * var_weff) + (eq41_e414 * var_weff_dn1));
        let eq41_e416_d_n2: f64 = ((eq41_e414_d_n2 * var_weff) + (eq41_e414 * var_weff_dn2));
        let eq41_e416_d_n3: f64 = ((eq41_e414_d_n3 * var_weff) + (eq41_e414 * var_weff_dn3));
        let eq41_e416_d_n4: f64 = ((eq41_e414_d_n4 * var_weff) + (eq41_e414 * var_weff_dn4));
        let eq41_e416_d_n5: f64 = ((eq41_e414_d_n5 * var_weff) + (eq41_e414 * var_weff_dn5));
        let eq41_e416_d_n6: f64 = ((eq41_e414_d_n6 * var_weff) + (eq41_e414 * var_weff_dn6));
        let eq41_e416_d_n7: f64 = ((eq41_e414_d_n7 * var_weff) + (eq41_e414 * var_weff_dn7));
        let eq41_e416_d_n8: f64 = ((eq41_e414_d_n8 * var_weff) + (eq41_e414 * var_weff_dn8));
        let eq41_e416_d_n9: f64 = ((eq41_e414_d_n9 * var_weff) + (eq41_e414 * var_weff_dn9));
        let eq41_e416_d_b0: f64 = ((eq41_e414_d_b0 * var_weff) + (eq41_e414 * var_weff_db0));
        let eq41_e416_d_b1: f64 = ((eq41_e414_d_b1 * var_weff) + (eq41_e414 * var_weff_db1));
        let eq41_e416_d_b2: f64 = ((eq41_e414_d_b2 * var_weff) + (eq41_e414 * var_weff_db2));
        let eq41_e416_d_b3: f64 = ((eq41_e414_d_b3 * var_weff) + (eq41_e414 * var_weff_db3));
        let eq41_e416_d_b4: f64 = ((eq41_e414_d_b4 * var_weff) + (eq41_e414 * var_weff_db4));
        let eq41_e416_d_b5: f64 = ((eq41_e414_d_b5 * var_weff) + (eq41_e414 * var_weff_db5));
        let eq41_e416_d_b6: f64 = ((eq41_e414_d_b6 * var_weff) + (eq41_e414 * var_weff_db6));
        let eq41_e416_d_b7: f64 = ((eq41_e414_d_b7 * var_weff) + (eq41_e414 * var_weff_db7));
        let eq41_e417_q: f64 = eq41_e416;
        let eq41_reactive_node_derivatives: [f64; 10] = [eq41_e416_d_n0, eq41_e416_d_n1, eq41_e416_d_n2, eq41_e416_d_n3, eq41_e416_d_n4, eq41_e416_d_n5, eq41_e416_d_n6, eq41_e416_d_n7, eq41_e416_d_n8, eq41_e416_d_n9];
        let eq41_reactive_branch_derivatives: [f64; 8] = [eq41_e416_d_b0, eq41_e416_d_b1, eq41_e416_d_b2, eq41_e416_d_b3, eq41_e416_d_b4, eq41_e416_d_b5, eq41_e416_d_b6, eq41_e416_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[4]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e419: f64 = (-var_qxf1);
        let eq42_e421: f64 = (eq42_e419 * var_weff);
        let eq42_e421_d_n0: f64 = (((-var_qxf1_dn0) * var_weff) + (eq42_e419 * var_weff_dn0));
        let eq42_e421_d_n1: f64 = (((-var_qxf1_dn1) * var_weff) + (eq42_e419 * var_weff_dn1));
        let eq42_e421_d_n2: f64 = (((-var_qxf1_dn2) * var_weff) + (eq42_e419 * var_weff_dn2));
        let eq42_e421_d_n3: f64 = (((-var_qxf1_dn3) * var_weff) + (eq42_e419 * var_weff_dn3));
        let eq42_e421_d_n4: f64 = (((-var_qxf1_dn4) * var_weff) + (eq42_e419 * var_weff_dn4));
        let eq42_e421_d_n5: f64 = (((-var_qxf1_dn5) * var_weff) + (eq42_e419 * var_weff_dn5));
        let eq42_e421_d_n6: f64 = (((-var_qxf1_dn6) * var_weff) + (eq42_e419 * var_weff_dn6));
        let eq42_e421_d_n7: f64 = (((-var_qxf1_dn7) * var_weff) + (eq42_e419 * var_weff_dn7));
        let eq42_e421_d_n8: f64 = (((-var_qxf1_dn8) * var_weff) + (eq42_e419 * var_weff_dn8));
        let eq42_e421_d_n9: f64 = (((-var_qxf1_dn9) * var_weff) + (eq42_e419 * var_weff_dn9));
        let eq42_e421_d_b0: f64 = (((-var_qxf1_db0) * var_weff) + (eq42_e419 * var_weff_db0));
        let eq42_e421_d_b1: f64 = (((-var_qxf1_db1) * var_weff) + (eq42_e419 * var_weff_db1));
        let eq42_e421_d_b2: f64 = (((-var_qxf1_db2) * var_weff) + (eq42_e419 * var_weff_db2));
        let eq42_e421_d_b3: f64 = (((-var_qxf1_db3) * var_weff) + (eq42_e419 * var_weff_db3));
        let eq42_e421_d_b4: f64 = (((-var_qxf1_db4) * var_weff) + (eq42_e419 * var_weff_db4));
        let eq42_e421_d_b5: f64 = (((-var_qxf1_db5) * var_weff) + (eq42_e419 * var_weff_db5));
        let eq42_e421_d_b6: f64 = (((-var_qxf1_db6) * var_weff) + (eq42_e419 * var_weff_db6));
        let eq42_e421_d_b7: f64 = (((-var_qxf1_db7) * var_weff) + (eq42_e419 * var_weff_db7));
        let eq42_e422_q: f64 = eq42_e421;
        let eq42_reactive_node_derivatives: [f64; 10] = [eq42_e421_d_n0, eq42_e421_d_n1, eq42_e421_d_n2, eq42_e421_d_n3, eq42_e421_d_n4, eq42_e421_d_n5, eq42_e421_d_n6, eq42_e421_d_n7, eq42_e421_d_n8, eq42_e421_d_n9];
        let eq42_reactive_branch_derivatives: [f64; 8] = [eq42_e421_d_b0, eq42_e421_d_b1, eq42_e421_d_b2, eq42_e421_d_b3, eq42_e421_d_b4, eq42_e421_d_b5, eq42_e421_d_b6, eq42_e421_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e425: f64 = (var_qxf1 * var_weff);
        let eq43_e425_d_n0: f64 = ((var_qxf1_dn0 * var_weff) + (var_qxf1 * var_weff_dn0));
        let eq43_e425_d_n1: f64 = ((var_qxf1_dn1 * var_weff) + (var_qxf1 * var_weff_dn1));
        let eq43_e425_d_n2: f64 = ((var_qxf1_dn2 * var_weff) + (var_qxf1 * var_weff_dn2));
        let eq43_e425_d_n3: f64 = ((var_qxf1_dn3 * var_weff) + (var_qxf1 * var_weff_dn3));
        let eq43_e425_d_n4: f64 = ((var_qxf1_dn4 * var_weff) + (var_qxf1 * var_weff_dn4));
        let eq43_e425_d_n5: f64 = ((var_qxf1_dn5 * var_weff) + (var_qxf1 * var_weff_dn5));
        let eq43_e425_d_n6: f64 = ((var_qxf1_dn6 * var_weff) + (var_qxf1 * var_weff_dn6));
        let eq43_e425_d_n7: f64 = ((var_qxf1_dn7 * var_weff) + (var_qxf1 * var_weff_dn7));
        let eq43_e425_d_n8: f64 = ((var_qxf1_dn8 * var_weff) + (var_qxf1 * var_weff_dn8));
        let eq43_e425_d_n9: f64 = ((var_qxf1_dn9 * var_weff) + (var_qxf1 * var_weff_dn9));
        let eq43_e425_d_b0: f64 = ((var_qxf1_db0 * var_weff) + (var_qxf1 * var_weff_db0));
        let eq43_e425_d_b1: f64 = ((var_qxf1_db1 * var_weff) + (var_qxf1 * var_weff_db1));
        let eq43_e425_d_b2: f64 = ((var_qxf1_db2 * var_weff) + (var_qxf1 * var_weff_db2));
        let eq43_e425_d_b3: f64 = ((var_qxf1_db3 * var_weff) + (var_qxf1 * var_weff_db3));
        let eq43_e425_d_b4: f64 = ((var_qxf1_db4 * var_weff) + (var_qxf1 * var_weff_db4));
        let eq43_e425_d_b5: f64 = ((var_qxf1_db5 * var_weff) + (var_qxf1 * var_weff_db5));
        let eq43_e425_d_b6: f64 = ((var_qxf1_db6 * var_weff) + (var_qxf1 * var_weff_db6));
        let eq43_e425_d_b7: f64 = ((var_qxf1_db7 * var_weff) + (var_qxf1 * var_weff_db7));
        let eq43_e426_q: f64 = eq43_e425;
        let eq43_reactive_node_derivatives: [f64; 10] = [eq43_e425_d_n0, eq43_e425_d_n1, eq43_e425_d_n2, eq43_e425_d_n3, eq43_e425_d_n4, eq43_e425_d_n5, eq43_e425_d_n6, eq43_e425_d_n7, eq43_e425_d_n8, eq43_e425_d_n9];
        let eq43_reactive_branch_derivatives: [f64; 8] = [eq43_e425_d_b0, eq43_e425_d_b1, eq43_e425_d_b2, eq43_e425_d_b3, eq43_e425_d_b4, eq43_e425_d_b5, eq43_e425_d_b6, eq43_e425_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
